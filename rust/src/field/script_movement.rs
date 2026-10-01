//! Scripted object movement. One task drives up to sixteen concurrent
//! movement scripts, one per object event.
//!
//! The task packs its state tightly: `data[0]` is a bitfield of which scripts
//! have finished, and the object event id for slot *i* is the *i*th **byte**
//! starting at `data[1]`, so sixteen ids share eight halfwords.

use crate::ffi::{
    CreateTask, DestroyTask, FindTaskIdByFunc, FuncIsActiveTask, LOCALID_PLAYER, NUM_TASK_DATA,
    TASK_NONE, TaskFunc, object_event, set_task_data, task, task_data,
};

const OBJECT_EVENTS_COUNT: usize = 16;
const MOVEMENT_ACTION_STEP_END: u8 = 0xfe;
/// The marker for a slot with no object event in it.
const NO_OBJECT: u8 = 0xff;

const MOVE_OBJECTS_PRIORITY: u8 = 50;

/// `data[0]` holds the finished bits; the ids start in `data[1]`.
const T_FINISHED_BITS: usize = 0;
const T_OBJECT_IDS: usize = 1;

#[unsafe(link_section = "ewram_data")]
static mut MOVEMENT_SCRIPTS: [*const u8; OBJECT_EVENTS_COUNT] =
    [core::ptr::null(); OBJECT_EVENTS_COUNT];

/// `TryGetObjectEventIdByLocalIdAndMap` with this module's view of its types.
#[inline]
unsafe fn TryGetObjectEventIdByLocalIdAndMap(a0: u8, a1: u8, a2: u8, a3: *mut u8) -> u8 {
    unsafe { crate::event_object_movement::TryGetObjectEventIdByLocalIdAndMap(a0, a1, a2, a3 as _) }
}
/// `ObjectEventIsHeldMovementActive` with this module's view of its types.
#[inline]
unsafe fn ObjectEventIsHeldMovementActive(a0: *mut u8) -> u8 {
    unsafe { crate::event_object_movement::ObjectEventIsHeldMovementActive(a0 as _) }
}
/// `ObjectEventClearHeldMovementIfFinished` with this module's view of its types.
#[inline]
unsafe fn ObjectEventClearHeldMovementIfFinished(a0: *mut u8) -> u8 {
    unsafe { crate::event_object_movement::ObjectEventClearHeldMovementIfFinished(a0 as _) }
}
/// `ObjectEventSetHeldMovement` with this module's view of its types.
#[inline]
unsafe fn ObjectEventSetHeldMovement(a0: *mut u8, a1: u8) -> u8 {
    unsafe { crate::event_object_movement::ObjectEventSetHeldMovement(a0 as _, a1) }
}
/// `FreezeObjectEvent` with this module's view of its types.
#[inline]
unsafe fn FreezeObjectEvent(a0: *mut u8) -> u8 {
    unsafe { crate::event_object_movement::FreezeObjectEvent(a0 as _) }
}
/// `UnfreezeObjectEvent` with this module's view of its types.
#[inline]
unsafe fn UnfreezeObjectEvent(a0: *mut u8) {
    unsafe {
        crate::event_object_movement::UnfreezeObjectEvent(a0 as _);
    }
}

/// `(u8 *)&gTasks[taskId].data[1]` - the packed object event id array.
#[inline]
unsafe fn object_id_slot(task_id: u8, index: usize) -> *mut u8 {
    unsafe {
        task(task_id)
            .add(crate::ffi::TASK_DATA_OFFSET + T_OBJECT_IDS * 2)
            .add(index)
    }
}

#[inline]
unsafe fn object_id(task_id: u8, index: usize) -> u8 {
    unsafe { object_id_slot(task_id, index).read_volatile() }
}

#[inline]
unsafe fn set_object_id(task_id: u8, index: usize, value: u8) {
    unsafe { object_id_slot(task_id, index).write_volatile(value) };
}

#[inline]
unsafe fn movement_script(index: usize) -> *const u8 {
    unsafe {
        (&raw const MOVEMENT_SCRIPTS)
            .cast::<*const u8>()
            .add(index)
            .read()
    }
}

#[inline]
unsafe fn set_movement_script(index: usize, script: *const u8) {
    unsafe {
        (&raw mut MOVEMENT_SCRIPTS)
            .cast::<*const u8>()
            .add(index)
            .write(script)
    };
}

#[inline]
unsafe fn finished_bits(task_id: u8) -> u16 {
    let bits = unsafe { task_data(task_id, T_FINISHED_BITS) };
    bits as u16
}

#[inline]
unsafe fn set_finished(task_id: u8, move_scr_id: usize, finished: bool) {
    let bit = 1u16 << move_scr_id;
    let bits = unsafe { finished_bits(task_id) };
    let bits = if finished { bits | bit } else { bits & !bit };
    unsafe { set_task_data(task_id, T_FINISHED_BITS, bits as i16) };
}

#[inline]
unsafe fn is_finished(task_id: u8, move_scr_id: usize) -> bool {
    let bits = unsafe { finished_bits(task_id) };
    bits & (1u16 << move_scr_id) != 0
}

/// Slot holding this object event, or `OBJECT_EVENTS_COUNT` if it has none.
unsafe fn slot_of_object(task_id: u8, object_event_id: u8) -> usize {
    for i in 0..OBJECT_EVENTS_COUNT {
        if unsafe { object_id(task_id, i) } == object_event_id {
            return i;
        }
    }
    OBJECT_EVENTS_COUNT
}

unsafe fn start_move_objects(priority: u8) {
    let task_id = unsafe { CreateTask(move_objects, priority) };
    // Every id slot starts as "no object"; data[0] keeps its zeroed bits.
    for i in 1..NUM_TASK_DATA {
        unsafe { set_task_data(task_id, i, -1) };
    }
}

unsafe fn move_objects_task_id() -> u8 {
    unsafe { FindTaskIdByFunc(move_objects) }
}

unsafe fn add_new_movement(
    task_id: u8,
    move_scr_id: usize,
    object_event_id: u8,
    script: *const u8,
) {
    unsafe { set_finished(task_id, move_scr_id, false) };
    unsafe { set_movement_script(move_scr_id, script) };
    unsafe { set_object_id(task_id, move_scr_id, object_event_id) };
}

unsafe fn try_add_new_movement(task_id: u8, object_event_id: u8, script: *const u8) -> u8 {
    // Reuse this object's own slot if it already has one.
    let slot = unsafe { slot_of_object(task_id, object_event_id) };
    if slot != OBJECT_EVENTS_COUNT {
        if !unsafe { is_finished(task_id, slot) } {
            return 1;
        }
        unsafe { add_new_movement(task_id, slot, object_event_id, script) };
        return 0;
    }

    // Otherwise take a free slot, which is marked with LOCALID_PLAYER.
    let slot = unsafe { slot_of_object(task_id, LOCALID_PLAYER) };
    if slot == OBJECT_EVENTS_COUNT {
        return 1;
    }
    unsafe { add_new_movement(task_id, slot, object_event_id, script) };
    0
}

#[unsafe(no_mangle)]
pub unsafe fn ScriptMovement_StartObjectMovementScript(
    local_id: u8,
    map_num: u8,
    map_group: u8,
    movement_script: *const u8,
) -> u8 {
    let mut object_event_id = 0u8;
    if unsafe {
        TryGetObjectEventIdByLocalIdAndMap(local_id, map_num, map_group, &raw mut object_event_id)
    } != 0
    {
        return 1;
    }

    if unsafe { FuncIsActiveTask(move_objects) } == 0 {
        unsafe { start_move_objects(MOVE_OBJECTS_PRIORITY) };
    }

    unsafe { try_add_new_movement(move_objects_task_id(), object_event_id, movement_script) }
}

#[unsafe(no_mangle)]
pub unsafe fn ScriptMovement_IsObjectMovementFinished(
    local_id: u8,
    map_num: u8,
    map_group: u8,
) -> u8 {
    let mut object_event_id = 0u8;
    if unsafe {
        TryGetObjectEventIdByLocalIdAndMap(local_id, map_num, map_group, &raw mut object_event_id)
    } != 0
    {
        return 1;
    }

    let task_id = unsafe { move_objects_task_id() };
    let slot = unsafe { slot_of_object(task_id, object_event_id) };
    if slot == OBJECT_EVENTS_COUNT {
        return 1;
    }
    u8::from(unsafe { is_finished(task_id, slot) })
}

#[unsafe(no_mangle)]
pub unsafe fn ScriptMovement_UnfreezeObjectEvents() {
    let task_id = unsafe { move_objects_task_id() };
    if task_id == TASK_NONE {
        return;
    }

    for i in 0..OBJECT_EVENTS_COUNT {
        let id = unsafe { object_id(task_id, i) };
        if id != NO_OBJECT {
            unsafe { UnfreezeObjectEvent(object_event(id as usize)) };
        }
    }
    unsafe { DestroyTask(task_id) };
}

unsafe fn move_objects(task_id: u8) {
    for i in 0..OBJECT_EVENTS_COUNT {
        let id = unsafe { object_id(task_id, i) };
        if id != NO_OBJECT {
            unsafe { take_step(task_id, i, id, movement_script(i)) };
        }
    }
}

unsafe fn take_step(task_id: u8, move_scr_id: usize, object_event_id: u8, script: *const u8) {
    let object = unsafe { object_event(object_event_id as usize) };

    // Wait for the current step to land before starting the next one.
    if unsafe { ObjectEventIsHeldMovementActive(object) } != 0
        && unsafe { ObjectEventClearHeldMovementIfFinished(object) } == 0
    {
        return;
    }

    let next_action = unsafe { script.read() };
    if next_action == MOVEMENT_ACTION_STEP_END {
        unsafe { set_finished(task_id, move_scr_id, true) };
        unsafe { FreezeObjectEvent(object) };
        return;
    }

    // Only advance the script once the action was accepted.
    if unsafe { ObjectEventSetHeldMovement(object, next_action) } == 0 {
        unsafe { set_movement_script(move_scr_id, script.add(1)) };
    }
}

const _: TaskFunc = move_objects;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sixteen_object_ids_fit_in_the_task_data_after_the_bitfield() {
        // data[1..] is 15 halfwords, so 30 bytes, which comfortably holds
        // the 16 packed ids.
        assert_eq!(OBJECT_EVENTS_COUNT, 16);
        assert_eq!((NUM_TASK_DATA - T_OBJECT_IDS) * 2, 30);
        assert!(OBJECT_EVENTS_COUNT <= (NUM_TASK_DATA - T_OBJECT_IDS) * 2);
    }

    #[test]
    fn the_finished_bitfield_covers_every_slot() {
        // data[0] is 16 bits, one per movement script.
        assert!(OBJECT_EVENTS_COUNT <= 16);
        let all: u16 = (0..OBJECT_EVENTS_COUNT)
            .map(|i| 1u16 << i)
            .fold(0, |a, b| a | b);
        assert_eq!(all, 0xffff);
    }

    #[test]
    fn a_free_slot_is_marked_with_the_player_local_id() {
        // start_move_objects fills data[1..] with 0xFFFF, so every packed
        // byte reads back as LOCALID_PLAYER.
        assert_eq!(LOCALID_PLAYER, 0xff);
        assert_eq!(NO_OBJECT, 0xff);
        assert_eq!((-1i16) as u16, 0xffff);
    }
}
