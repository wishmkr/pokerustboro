use crate::ffi::{
    CreateTask, DestroyTask, FuncIsActiveTask, LOCALID_PLAYER, T_TILE_TRANSITION,
    gSelectedObjectEvent, gSpecialVar_Facing, object_event, object_event_active,
    object_event_single_movement_active, player_avatar_tile_transition_state, set_task_data,
    task_data,
};
use core::ptr::addr_of;

/// Task priority shared by every lock task in the original file.
const LOCK_TASK_PRIORITY: u8 = 80;

// Task data slots, matching the tPlayerFrozen/tObjectFrozen/tObjectId macros.
const T_PLAYER_FROZEN: usize = 0;
const T_OBJECT_FROZEN: usize = 1;
const T_OBJECT_ID: usize = 2;

unsafe extern "C" {
    static mut gNoOfApproachingTrainers: u8;

    fn PlayerFreeze();
    fn StopPlayerAvatar();
    fn FreezeObjectEvents();
    fn FreezeObjectEvent(object_event: *mut u8) -> u8;
    fn FreezeObjectEventsExceptOne(object_event_id: u8);
    fn FreezeObjectEventsExceptTwo(object_event_id1: u8, object_event_id2: u8);
    fn UnfreezeObjectEvents();
    fn ObjectEventClearHeldMovementIfFinished(object_event: *mut u8) -> u8;
    fn ObjectEventClearHeldMovementIfActive(object_event: *mut u8);
    fn ObjectEventFaceOppositeDirection(object_event: *mut u8, direction: u8) -> u8;
    fn ScriptMovement_UnfreezeObjectEvents();
    fn GetObjectEventIdByLocalIdAndMap(local_id: u8, map_num: u8, map_group_id: u8) -> u8;
    fn GetChosenApproachingTrainerObjectEventId(array_id: u8) -> u8;
}

#[inline]
unsafe fn selected_object_event() -> usize {
    unsafe { addr_of!(gSelectedObjectEvent).read_volatile() as usize }
}

/// Freezes `objectEventId` right away when it is not mid-step, and records
/// that on the task so the follow-up task has nothing left to do.
unsafe fn freeze_object_now_if_idle(task_id: u8, object_event_id: u8) {
    if !unsafe { object_event_single_movement_active(object_event_id as usize) } {
        let _ = unsafe { FreezeObjectEvent(object_event(object_event_id as usize)) };
        unsafe { set_task_data(task_id, T_OBJECT_FROZEN, 1) };
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsPlayerStandingStill() -> u8 {
    u8::from(unsafe { player_avatar_tile_transition_state() } != T_TILE_TRANSITION)
}

/// Freezes the player once their movement is finished.
unsafe extern "C" fn task_freeze_player(task_id: u8) {
    if unsafe { IsPlayerStandingStill() } != 0 {
        unsafe { PlayerFreeze() };
        unsafe { DestroyTask(task_id) };
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsFreezePlayerFinished() -> u8 {
    if unsafe { FuncIsActiveTask(task_freeze_player) } != 0 {
        0
    } else {
        unsafe { StopPlayerAvatar() };
        1
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn FreezeObjects_WaitForPlayer() {
    unsafe { FreezeObjectEvents() };
    let _ = unsafe { CreateTask(task_freeze_player, LOCK_TASK_PRIORITY) };
}

/// Freezes the selected object and the player once their movement is finished.
unsafe extern "C" fn task_freeze_selected_object_and_player(task_id: u8) {
    if unsafe { task_data(task_id, T_PLAYER_FROZEN) } == 0
        && unsafe { IsPlayerStandingStill() } == 1
    {
        unsafe { PlayerFreeze() };
        unsafe { set_task_data(task_id, T_PLAYER_FROZEN, 1) };
    }

    let selected = unsafe { selected_object_event() };
    if unsafe { task_data(task_id, T_OBJECT_FROZEN) } == 0
        && !unsafe { object_event_single_movement_active(selected) }
    {
        let _ = unsafe { FreezeObjectEvent(object_event(selected)) };
        unsafe { set_task_data(task_id, T_OBJECT_FROZEN, 1) };
    }

    if unsafe { task_data(task_id, T_PLAYER_FROZEN) } != 0
        && unsafe { task_data(task_id, T_OBJECT_FROZEN) } != 0
    {
        unsafe { DestroyTask(task_id) };
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsFreezeSelectedObjectAndPlayerFinished() -> u8 {
    if unsafe { FuncIsActiveTask(task_freeze_selected_object_and_player) } != 0 {
        0
    } else {
        unsafe { StopPlayerAvatar() };
        1
    }
}

/// Freezes every object except the selected one and the player immediately;
/// those two are frozen once their movement is finished.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FreezeObjects_WaitForPlayerAndSelected() {
    let selected = unsafe { addr_of!(gSelectedObjectEvent).read_volatile() };
    unsafe { FreezeObjectEventsExceptOne(selected) };
    let task_id = unsafe { CreateTask(task_freeze_selected_object_and_player, LOCK_TASK_PRIORITY) };
    unsafe { freeze_object_now_if_idle(task_id, selected) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScriptUnfreezeObjectEvents() {
    let player_object_id = unsafe { GetObjectEventIdByLocalIdAndMap(LOCALID_PLAYER, 0, 0) };
    let _ =
        unsafe { ObjectEventClearHeldMovementIfFinished(object_event(player_object_id as usize)) };
    unsafe { ScriptMovement_UnfreezeObjectEvents() };
    unsafe { UnfreezeObjectEvents() };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn UnionRoom_UnlockPlayerAndChatPartner() {
    let selected = unsafe { selected_object_event() };
    if unsafe { object_event_active(selected) } {
        let _ = unsafe { ObjectEventClearHeldMovementIfFinished(object_event(selected)) };
    }

    let player_object_id = unsafe { GetObjectEventIdByLocalIdAndMap(LOCALID_PLAYER, 0, 0) };
    let _ =
        unsafe { ObjectEventClearHeldMovementIfFinished(object_event(player_object_id as usize)) };
    unsafe { ScriptMovement_UnfreezeObjectEvents() };
    unsafe { UnfreezeObjectEvents() };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn Script_FacePlayer() {
    let selected = unsafe { selected_object_event() };
    let facing = unsafe { addr_of!(gSpecialVar_Facing).read_volatile() } as u8;
    let _ = unsafe { ObjectEventFaceOppositeDirection(object_event(selected), facing) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn Script_ClearHeldMovement() {
    let selected = unsafe { selected_object_event() };
    unsafe { ObjectEventClearHeldMovementIfActive(object_event(selected)) };
}

/// Freezes the object named in `tObjectId` and the player once their movement
/// is finished.
unsafe extern "C" fn task_freeze_object_and_player(task_id: u8) {
    let object_event_id = unsafe { task_data(task_id, T_OBJECT_ID) } as u8 as usize;

    if unsafe { task_data(task_id, T_PLAYER_FROZEN) } == 0
        && unsafe { IsPlayerStandingStill() } == 1
    {
        unsafe { PlayerFreeze() };
        unsafe { set_task_data(task_id, T_PLAYER_FROZEN, 1) };
    }

    if unsafe { task_data(task_id, T_OBJECT_FROZEN) } == 0
        && !unsafe { object_event_single_movement_active(object_event_id) }
    {
        let _ = unsafe { FreezeObjectEvent(object_event(object_event_id)) };
        unsafe { set_task_data(task_id, T_OBJECT_FROZEN, 1) };
    }

    if unsafe { task_data(task_id, T_PLAYER_FROZEN) } != 0
        && unsafe { task_data(task_id, T_OBJECT_FROZEN) } != 0
    {
        unsafe { DestroyTask(task_id) };
    }
}

/// Freezes every object except the player and the approaching trainers
/// immediately; those are frozen once their movement is finished.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FreezeForApproachingTrainers() {
    let trainer1 = unsafe { GetChosenApproachingTrainerObjectEventId(0) };

    if unsafe { addr_of!(gNoOfApproachingTrainers).read_volatile() } == 2 {
        let trainer2 = unsafe { GetChosenApproachingTrainerObjectEventId(1) };
        unsafe { FreezeObjectEventsExceptTwo(trainer1, trainer2) };

        let task_id = unsafe { CreateTask(task_freeze_object_and_player, LOCK_TASK_PRIORITY) };
        unsafe { set_task_data(task_id, T_OBJECT_ID, i16::from(trainer1)) };
        unsafe { freeze_object_now_if_idle(task_id, trainer1) };

        let task_id = unsafe { CreateTask(task_freeze_object_and_player, LOCK_TASK_PRIORITY + 1) };
        unsafe { set_task_data(task_id, T_OBJECT_ID, i16::from(trainer2)) };
        unsafe { freeze_object_now_if_idle(task_id, trainer2) };
    } else {
        unsafe { FreezeObjectEventsExceptOne(trainer1) };
        let task_id = unsafe { CreateTask(task_freeze_object_and_player, LOCK_TASK_PRIORITY) };
        unsafe { set_task_data(task_id, T_OBJECT_ID, i16::from(trainer1)) };
        unsafe { freeze_object_now_if_idle(task_id, trainer1) };
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsFreezeObjectAndPlayerFinished() -> u8 {
    if unsafe { FuncIsActiveTask(task_freeze_object_and_player) } != 0 {
        0
    } else {
        unsafe { StopPlayerAvatar() };
        1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn task_data_slots_match_the_original_macros() {
        assert_eq!(T_PLAYER_FROZEN, 0);
        assert_eq!(T_OBJECT_FROZEN, 1);
        assert_eq!(T_OBJECT_ID, 2);
    }

    #[test]
    fn the_second_trainer_task_runs_one_priority_later() {
        assert_eq!(LOCK_TASK_PRIORITY, 80);
        assert_eq!(LOCK_TASK_PRIORITY + 1, 81);
    }
}
