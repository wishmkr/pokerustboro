//! Scripted field scenes: the moving truck at the start of the game (boxes
//! jostling, the camera bobbing, the door opening) and looking through the
//! S.S. Tidal porthole. Tables are in `data/field_special_scene.rs`.

use crate::data::field_special_scene::{
    sSSTidalSailEastMovementScript, sSSTidalSailWestMovementScript, sTruckCamera_HorizontalTable,
};
use crate::event_data::{FlagClear, FlagSet, GetVarPointer, VarGet};
use crate::ffi::{
    A_BUTTON, CreateTask, DestroyTask, OBJECT_EVENT_SIZE, PLTT_SIZE, PlaySE, TASK_IS_ACTIVE_OFFSET,
    gObjectEvents, gPlayerAvatar, gPlttBufferFaded, joy_new, set_task_data, set_task_func, sprite,
    task, task_data,
};
use crate::load_save::gSaveBlock1Ptr;
use crate::script_movement::ScriptMovement_StartObjectMovementScript;
use crate::sprite::{SpriteCallbackDummy, StartSpriteAnim};
use crate::task::FuncIsActiveTask;

const LOCALID_PLAYER: u8 = 0xff;
const LOCALID_TRUCK_BOX_TOP: u8 = 1;
const LOCALID_TRUCK_BOX_BOTTOM_L: u8 = 2;
const LOCALID_TRUCK_BOX_BOTTOM_R: u8 = 3;
const BOX1_OFFSET: (i16, i16) = (3, 3);
const BOX2_OFFSET: (i16, i16) = (0, -3);
const BOX3_OFFSET: (i16, i16) = (-3, 0);

const SE_TRUCK_MOVE: u16 = 0x31;
const SE_TRUCK_STOP: u16 = 0x32;
const SE_TRUCK_UNLOAD: u16 = 0x33;
const SE_TRUCK_DOOR: u16 = 0x34;
const MAP_OFFSET: i32 = 7;
const METATILE_EXIT_LIGHT: [u16; 3] = [0x208, 0x210, 0x218];
const METATILE_DOOR_CLOSED: [u16; 3] = [0x20d, 0x215, 0x21d];

const SS_TIDAL_LOCATION_CURRENTS: u8 = 0;
const VAR_SS_TIDAL_STATE: u16 = 0x40b4;
const SS_TIDAL_DEPART_SLATEPORT: u16 = 2;
const SS_TIDAL_EXIT_CURRENTS_RIGHT: u16 = 9;
const SS_TIDAL_EXIT_CURRENTS_LEFT: u16 = 10;
const FLAG_HIDE_MAP_NAME_POPUP: u16 = 0x4000;
const FLAG_DONT_TRANSITION_MUSIC: u16 = 0x4001;
const FLAG_SYS_CRUISE_MODE: u16 = 0x88d;
const OBJ_EVENT_GFX_SS_TIDAL: u16 = 0x8c;
const DIR_WEST: u8 = 3;
const DIR_EAST: u8 = 4;
const WARP_ID_NONE: i8 = -1;
const PLAYER_AVATAR_OBJECT_EVENT_ID: usize = 5;
const OBJECT_EVENT_INVISIBLE_BYTE: usize = 1;
const OBJECT_EVENT_INVISIBLE_BIT: u8 = 0x20;
const SPRITE_FLAGS0: usize = 0x3e;
const SPRITE_COORD_OFFSET_ENABLED: u8 = 0x02;
const SB1_LOCATION_MAP_GROUP: usize = 4;
const SB1_LOCATION_MAP_NUM: usize = 5;

// Porthole states.
const INIT_PORTHOLE: i16 = 0;
const IDLE_CHECK: i16 = 1;
const EXECUTE_MOVEMENT: i16 = 2;
const EXIT_PORTHOLE: i16 = 3;

unsafe extern "C" {
    fn SetObjectEventSpritePosByLocalIdAndMap(
        local_id: u8,
        map_num: u8,
        map_group: u8,
        x: i16,
        y: i16,
    );
    fn SetCameraPanning(horizontal: i16, vertical: i16);
    fn SetCameraPanningCallback(callback: Option<unsafe extern "C" fn()>);
    fn FadeInFromBlack();
    fn InstallCameraPanAheadCallback();
    fn MapGridSetMetatileIdAt(x: i32, y: i32, metatile: u16);
    fn DrawWholeMapView();
    fn LockPlayerFieldControls();
    fn UnlockPlayerFieldControls();
    fn GetSSTidalLocation(map_group: *mut i8, map_num: *mut i8, x: *mut i16, y: *mut i16) -> u8;
    fn SetWarpDestination(map_group: i8, map_num: i8, warp_id: i8, x: i8, y: i8);
    fn ScriptMovement_IsObjectMovementFinished(local_id: u8, map_num: u8, map_group: u8) -> u8;
    fn CountSSTidalStep(delta: u16) -> u32;
    fn SetWarpDestinationToDynamicWarp(unused: u8);
    fn DoDiveWarp();
    fn CreateObjectGraphicsSprite(
        graphics_id: u16,
        callback: unsafe extern "C" fn(*mut u8),
        x: i16,
        y: i16,
        subpriority: u8,
    ) -> u8;
    fn GetFaceDirectionAnimNum(direction: u8) -> u8;
    fn SetDynamicWarp(unused: i32, map_group: i8, map_num: i8, warp_id: i8);
    fn DoPortholeWarp();
    fn CpuFastSet(src: *const core::ffi::c_void, dest: *mut core::ffi::c_void, control: u32);
}

#[inline]
unsafe fn location() -> (u8, u8) {
    let sb1 = unsafe { (&raw const gSaveBlock1Ptr).read() };
    unsafe {
        (
            sb1.add(SB1_LOCATION_MAP_NUM).read(),
            sb1.add(SB1_LOCATION_MAP_GROUP).read(),
        )
    }
}

fn truck_camera_bobbing_y(time: i32) -> i16 {
    if time % 120 == 0 {
        -1
    } else if time % 10 <= 4 {
        1
    } else {
        0
    }
}

/// -1 makes a box hop, 0 keeps it at rest.
fn truck_box_y_movement(time: i32) -> i16 {
    if (time + 120) % 180 == 0 { -1 } else { 0 }
}

unsafe fn place_box(local_id: u8, offset: (i16, i16), camera_x: i16, y: i16) {
    let (map_num, map_group) = unsafe { location() };
    unsafe {
        SetObjectEventSpritePosByLocalIdAndMap(
            local_id,
            map_num,
            map_group,
            offset.0 - camera_x,
            offset.1 + y,
        )
    };
}

unsafe fn place_boxes(camera_x: i16, timer: i32) {
    unsafe {
        place_box(
            LOCALID_TRUCK_BOX_TOP,
            BOX1_OFFSET,
            camera_x,
            truck_box_y_movement(timer + 30) * 4,
        )
    };
    unsafe {
        place_box(
            LOCALID_TRUCK_BOX_BOTTOM_L,
            BOX2_OFFSET,
            camera_x,
            truck_box_y_movement(timer) * 2,
        )
    };
    unsafe {
        place_box(
            LOCALID_TRUCK_BOX_BOTTOM_R,
            BOX3_OFFSET,
            camera_x,
            truck_box_y_movement(timer) * 4,
        )
    };
}

unsafe extern "C" fn task_truck1(task_id: u8) {
    let timer = i32::from(unsafe { task_data(task_id, 0) });
    unsafe { place_boxes(0, timer) };
    // An arbitrary limit that is never reached.
    let mut next = unsafe { task_data(task_id, 0) }.wrapping_add(1);
    if next == 30000 {
        next = 0;
    }
    unsafe { set_task_data(task_id, 0, next) };
    unsafe { SetCameraPanning(0, truck_camera_bobbing_y(i32::from(next))) };
}

/// Advances the horizontal pan every six frames. Returns the step, or None
/// once the table is exhausted (the task is then destroyed).
unsafe fn advance_pan(task_id: u8) -> Option<usize> {
    let horizontal = unsafe { task_data(task_id, 0) }.wrapping_add(1);
    unsafe { set_task_data(task_id, 0, horizontal) };
    if horizontal > 5 {
        unsafe { set_task_data(task_id, 0, 0) };
        unsafe { set_task_data(task_id, 1, task_data(task_id, 1).wrapping_add(1)) };
    }
    let step = unsafe { task_data(task_id, 1) } as u16 as usize;
    if step == sTruckCamera_HorizontalTable.0.len() {
        unsafe { DestroyTask(task_id) };
        None
    } else {
        Some(step)
    }
}

#[inline]
fn pan_at(step: usize) -> i16 {
    i16::from(unsafe { sTruckCamera_HorizontalTable.as_ptr().add(step).read() } as i8)
}

unsafe extern "C" fn task_truck2(task_id: u8) {
    unsafe { set_task_data(task_id, 2, task_data(task_id, 2).wrapping_add(1)) };
    let Some(step) = (unsafe { advance_pan(task_id) }) else {
        return;
    };
    if pan_at(step) == 2 {
        unsafe { set_task_func(task_id, task_truck3) };
    }
    let camera_x = pan_at(step);
    let vertical = i32::from(unsafe { task_data(task_id, 2) });
    unsafe { SetCameraPanning(camera_x, truck_camera_bobbing_y(vertical)) };
    unsafe { place_boxes(camera_x, vertical) };
}

unsafe extern "C" fn task_truck3(task_id: u8) {
    let Some(step) = (unsafe { advance_pan(task_id) }) else {
        return;
    };
    let camera_x = pan_at(step);
    unsafe { SetCameraPanning(camera_x, 0) };
    unsafe { place_box(LOCALID_TRUCK_BOX_TOP, BOX1_OFFSET, camera_x, 0) };
    unsafe { place_box(LOCALID_TRUCK_BOX_BOTTOM_L, BOX2_OFFSET, camera_x, 0) };
    unsafe { place_box(LOCALID_TRUCK_BOX_BOTTOM_R, BOX3_OFFSET, camera_x, 0) };
}

unsafe fn set_truck_door(metatiles: [u16; 3]) {
    for (i, metatile) in metatiles.into_iter().enumerate() {
        unsafe { MapGridSetMetatileIdAt(4 + MAP_OFFSET, 1 + i as i32 + MAP_OFFSET, metatile) };
    }
    unsafe { DrawWholeMapView() };
}

unsafe extern "C" fn task_handle_truck_sequence(task_id: u8) {
    let get = |i| unsafe { task_data(task_id, i) };
    let put = |i, v| unsafe { set_task_data(task_id, i, v) };
    const STATE: usize = 0;
    const TIMER: usize = 1;
    const TASK_ID1: usize = 2;
    const TASK_ID2: usize = 3;

    match get(STATE) {
        0 => {
            put(TIMER, get(TIMER) + 1);
            if get(TIMER) == 90 {
                unsafe { SetCameraPanningCallback(None) };
                put(TIMER, 0);
                put(TASK_ID1, i16::from(unsafe { CreateTask(task_truck1, 0xa) }));
                put(STATE, 1);
                unsafe { PlaySE(SE_TRUCK_MOVE) };
            }
        }
        1 => {
            put(TIMER, get(TIMER) + 1);
            if get(TIMER) == 150 {
                unsafe { FadeInFromBlack() };
                put(TIMER, 0);
                put(STATE, 2);
            }
        }
        2 => {
            put(TIMER, get(TIMER) + 1);
            if !unsafe { crate::ffi::palette_fade_active() } && get(TIMER) > 300 {
                put(TIMER, 0);
                unsafe { DestroyTask(get(TASK_ID1) as u8) };
                put(TASK_ID2, i16::from(unsafe { CreateTask(task_truck2, 0xa) }));
                put(STATE, 3);
                unsafe { PlaySE(SE_TRUCK_STOP) };
            }
        }
        3 => {
            let other = get(TASK_ID2) as u8;
            if unsafe { task(other).add(TASK_IS_ACTIVE_OFFSET).read() } == 0 {
                // task_truck2/task_truck3 has finished.
                unsafe { InstallCameraPanAheadCallback() };
                put(TIMER, 0);
                put(STATE, 4);
            }
        }
        4 => {
            put(TIMER, get(TIMER) + 1);
            if get(TIMER) == 90 {
                unsafe { PlaySE(SE_TRUCK_UNLOAD) };
                put(TIMER, 0);
                put(STATE, 5);
            }
        }
        5 => {
            put(TIMER, get(TIMER) + 1);
            if get(TIMER) == 120 {
                unsafe { set_truck_door(METATILE_EXIT_LIGHT) };
                unsafe { PlaySE(SE_TRUCK_DOOR) };
                unsafe { DestroyTask(task_id) };
                unsafe { UnlockPlayerFieldControls() };
            }
        }
        _ => {}
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ExecuteTruckSequence() {
    unsafe { set_truck_door(METATILE_DOOR_CLOSED) };
    unsafe { LockPlayerFieldControls() };
    // CpuFastFill(0, gPlttBufferFaded, PLTT_SIZE)
    let zero = 0u32;
    unsafe {
        CpuFastSet(
            (&raw const zero).cast(),
            (&raw mut gPlttBufferFaded).cast(),
            (1 << 24) | (PLTT_SIZE as u32 / 4),
        )
    };
    unsafe { CreateTask(task_handle_truck_sequence, 0xa) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn EndTruckSequence(_task_id: u8) {
    if unsafe { FuncIsActiveTask(task_handle_truck_sequence) } == 0 {
        unsafe { place_box(LOCALID_TRUCK_BOX_TOP, BOX1_OFFSET, 0, 0) };
        unsafe { place_box(LOCALID_TRUCK_BOX_BOTTOM_L, BOX2_OFFSET, 0, 0) };
        unsafe { place_box(LOCALID_TRUCK_BOX_BOTTOM_R, BOX3_OFFSET, 0, 0) };
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn TrySetPortholeWarpDestination() -> u8 {
    let (mut map_group, mut map_num, mut x, mut y) = (0i8, 0i8, 0i16, 0i16);
    if unsafe { GetSSTidalLocation(&raw mut map_group, &raw mut map_num, &raw mut x, &raw mut y) }
        != SS_TIDAL_LOCATION_CURRENTS
    {
        return 0;
    }
    unsafe { SetWarpDestination(map_group, map_num, WARP_ID_NONE, x as i8, y as i8) };
    1
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn Task_HandlePorthole(task_id: u8) {
    let cruise_state = unsafe { GetVarPointer(VAR_SS_TIDAL_STATE) };
    let (map_num, map_group) = unsafe { location() };
    let get = |i| unsafe { task_data(task_id, i) };
    let put = |i, v| unsafe { set_task_data(task_id, i, v) };

    let mut state = get(0);
    if state == INIT_PORTHOLE {
        // Finish fading, then move before checking whether to leave.
        if !unsafe { crate::ffi::palette_fade_active() } {
            put(1, 0);
            put(0, EXECUTE_MOVEMENT);
        }
        return;
    }
    if state == IDLE_CHECK {
        if unsafe { joy_new(A_BUTTON) } {
            put(1, 1);
        }
        if unsafe { ScriptMovement_IsObjectMovementFinished(LOCALID_PLAYER, map_num, map_group) }
            == 0
        {
            return;
        }
        if unsafe { CountSSTidalStep(1) } == 1 {
            let exit = if unsafe { cruise_state.read() } == SS_TIDAL_DEPART_SLATEPORT {
                SS_TIDAL_EXIT_CURRENTS_RIGHT
            } else {
                SS_TIDAL_EXIT_CURRENTS_LEFT
            };
            unsafe { cruise_state.write(exit) };
            put(0, EXIT_PORTHOLE);
            return;
        }
        put(0, EXECUTE_MOVEMENT);
        state = EXECUTE_MOVEMENT;
    }
    match state {
        EXECUTE_MOVEMENT => {
            if get(1) != 0 {
                put(0, EXIT_PORTHOLE);
                return;
            }
            let script = if unsafe { cruise_state.read() } == SS_TIDAL_DEPART_SLATEPORT {
                sSSTidalSailEastMovementScript.as_ptr()
            } else {
                sSSTidalSailWestMovementScript.as_ptr()
            };
            unsafe {
                ScriptMovement_StartObjectMovementScript(LOCALID_PLAYER, map_num, map_group, script)
            };
            put(0, IDLE_CHECK);
        }
        EXIT_PORTHOLE => {
            unsafe { FlagClear(FLAG_DONT_TRANSITION_MUSIC) };
            unsafe { FlagClear(FLAG_HIDE_MAP_NAME_POPUP) };
            unsafe { SetWarpDestinationToDynamicWarp(0) };
            unsafe { DoDiveWarp() };
            unsafe { DestroyTask(task_id) };
        }
        _ => {}
    }
}

unsafe fn show_ss_tidal_while_sailing() {
    let sprite_id = unsafe {
        CreateObjectGraphicsSprite(OBJ_EVENT_GFX_SS_TIDAL, SpriteCallbackDummy, 112, 80, 0)
    };
    let s = unsafe { sprite(usize::from(sprite_id)) };
    unsafe {
        s.add(SPRITE_FLAGS0)
            .write(s.add(SPRITE_FLAGS0).read() & !SPRITE_COORD_OFFSET_ENABLED)
    };
    let direction = if unsafe { VarGet(VAR_SS_TIDAL_STATE) } == SS_TIDAL_DEPART_SLATEPORT {
        DIR_EAST
    } else {
        DIR_WEST
    };
    unsafe { StartSpriteAnim(s, GetFaceDirectionAnimNum(direction)) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn FieldCB_ShowPortholeView() {
    unsafe { show_ss_tidal_while_sailing() };
    let id = usize::from(unsafe {
        (&raw const gPlayerAvatar)
            .add(PLAYER_AVATAR_OBJECT_EVENT_ID)
            .read()
    });
    let object = unsafe {
        (&raw mut gObjectEvents).add(id * OBJECT_EVENT_SIZE + OBJECT_EVENT_INVISIBLE_BYTE)
    };
    unsafe { object.write(object.read() | OBJECT_EVENT_INVISIBLE_BIT) };
    unsafe { FadeInFromBlack() };
    unsafe { CreateTask(Task_HandlePorthole, 80) };
    unsafe { LockPlayerFieldControls() };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn LookThroughPorthole() {
    unsafe { FlagSet(FLAG_SYS_CRUISE_MODE) };
    unsafe { FlagSet(FLAG_DONT_TRANSITION_MUSIC) };
    unsafe { FlagSet(FLAG_HIDE_MAP_NAME_POPUP) };
    let sb1 = unsafe { (&raw const gSaveBlock1Ptr).read() };
    let group = unsafe { sb1.add(SB1_LOCATION_MAP_GROUP).read() } as i8;
    let num = unsafe { sb1.add(SB1_LOCATION_MAP_NUM).read() } as i8;
    unsafe { SetDynamicWarp(0, group, num, WARP_ID_NONE) };
    unsafe { TrySetPortholeWarpDestination() };
    unsafe { DoPortholeWarp() };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bobbing_and_box_timing() {
        assert_eq!(truck_camera_bobbing_y(120), -1);
        assert_eq!(truck_camera_bobbing_y(3), 1);
        assert_eq!(truck_camera_bobbing_y(7), 0);
        assert_eq!(truck_box_y_movement(60), -1);
        assert_eq!(truck_box_y_movement(61), 0);
    }
}
