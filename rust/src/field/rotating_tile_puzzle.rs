//! The arrow-tile puzzles in the Mossdeep Gym and the Trick House: objects
//! standing on arrows of the active colour shift one tile, then turn to
//! follow the rotation. Movement scripts are in `data/rotating_tile_puzzle.rs`.

use crate::data::rotating_tile_puzzle::{
    sMovement_FaceDown, sMovement_FaceLeft, sMovement_FaceRight, sMovement_FaceUp,
    sMovement_ShiftDown, sMovement_ShiftLeft, sMovement_ShiftRight, sMovement_ShiftUp,
};
use crate::ffi::OBJECT_EVENT_SIZE;
use crate::load_save::gSaveBlock1Ptr;
use crate::malloc::{AllocZeroed, Free};
use crate::script_movement::{
    ScriptMovement_StartObjectMovementScript, ScriptMovement_UnfreezeObjectEvents,
};

const ROTATE_COUNTERCLOCKWISE: i32 = 0;
const ROTATE_CLOCKWISE: i32 = 1;
const ROTATE_NONE: i32 = 2;

const OBJECT_EVENTS_COUNT: u8 = 16;
const OBJECT_EVENT_TEMPLATES_COUNT: usize = 64;
const MAP_OFFSET: i32 = 7;
const METATILE_ROW_WIDTH: i32 = 8;
const METATILE_MOSSDEEP_GYM_YELLOW_ARROW_RIGHT: i32 = 0x250;
const METATILE_TRICK_HOUSE_ARROW_YELLOW_ON_WHITE_RIGHT: i32 = 0x298;
const LOCALID_PLAYER: u8 = 0xff;
const LOCALID_NONE: u16 = 0;

const MOVEMENT_TYPE_FACE_UP: u8 = 7;
const MOVEMENT_TYPE_FACE_DOWN: u8 = 8;
const MOVEMENT_TYPE_FACE_LEFT: u8 = 9;
const MOVEMENT_TYPE_FACE_RIGHT: u8 = 10;
const DIR_SOUTH: u8 = 1;
const DIR_NORTH: u8 = 2;
const DIR_WEST: u8 = 3;
const DIR_EAST: u8 = 4;

/// `struct ObjectEventTemplate` fields.
const TEMPLATE_SIZE: usize = 0x18;
const TEMPLATE_LOCAL_ID: usize = 0x00;
const TEMPLATE_X: usize = 0x04;
const TEMPLATE_Y: usize = 0x06;
const TEMPLATE_MOVEMENT_TYPE: usize = 0x09;
const SB1_OBJECT_EVENT_TEMPLATES: usize = 0xc70;
const SB1_LOCATION_MAP_GROUP: usize = 4;
const SB1_LOCATION_MAP_NUM: usize = 5;
/// `gObjectEvents[i].facingDirection`: low nibble of byte 0x18.
const OBJECT_EVENT_FACING_DIRECTION: usize = 0x18;

/// `struct RotatingTilePuzzle`: 16 `{prevPuzzleTileNum, eventTemplateId}`
/// pairs, then `numObjects` and `isTrickHouse`.
const PUZZLE_SIZE: u32 = 36;
const PUZZLE_NUM_OBJECTS: usize = 32;
const PUZZLE_IS_TRICK_HOUSE: usize = 33;

#[unsafe(link_section = "ewram_data")]
static mut PUZZLE: *mut u8 = core::ptr::null_mut();

/// `MapGridGetMetatileIdAt` with this module's view of its types.
#[inline]
unsafe fn MapGridGetMetatileIdAt(a0: i32, a1: i32) -> i32 {
    unsafe { crate::fieldmap::MapGridGetMetatileIdAt(a0, a1) }
}
/// `GetObjectEventIdByLocalIdAndMap` with this module's view of its types.
#[inline]
unsafe fn GetObjectEventIdByLocalIdAndMap(a0: u8, a1: u8, a2: u8) -> u8 {
    unsafe { crate::event_object_movement::GetObjectEventIdByLocalIdAndMap(a0, a1, a2) }
}
/// `ObjectEventClearHeldMovementIfFinished` with this module's view of its types.
#[inline]
unsafe fn ObjectEventClearHeldMovementIfFinished(a0: *mut u8) -> u8 {
    unsafe { crate::event_object_movement::ObjectEventClearHeldMovementIfFinished(a0 as _) }
}

#[inline]
fn puzzle() -> *mut u8 {
    unsafe { (&raw const PUZZLE).read() }
}

#[inline]
unsafe fn template(index: usize) -> *mut u8 {
    let sb1 = unsafe { (&raw const gSaveBlock1Ptr).read().cast::<u8>() };
    unsafe { sb1.add(SB1_OBJECT_EVENT_TEMPLATES + index * TEMPLATE_SIZE) }
}

#[inline]
unsafe fn location() -> (u8, u8) {
    let sb1 = unsafe { (&raw const gSaveBlock1Ptr).read().cast::<u8>() };
    unsafe {
        (
            sb1.add(SB1_LOCATION_MAP_NUM).read(),
            sb1.add(SB1_LOCATION_MAP_GROUP).read(),
        )
    }
}

unsafe fn puzzle_tile_start() -> i32 {
    if unsafe { puzzle().add(PUZZLE_IS_TRICK_HOUSE).read() } == 0 {
        METATILE_MOSSDEEP_GYM_YELLOW_ARROW_RIGHT
    } else {
        METATILE_TRICK_HOUSE_ARROW_YELLOW_ON_WHITE_RIGHT
    }
}

/// The metatile under an object template, as the original's `u16`.
unsafe fn metatile_under(t: *mut u8) -> i32 {
    let x = i32::from(unsafe { t.add(TEMPLATE_X).cast::<i16>().read() }) + MAP_OFFSET;
    let y = i32::from(unsafe { t.add(TEMPLATE_Y).cast::<i16>().read() }) + MAP_OFFSET;
    i32::from(unsafe { MapGridGetMetatileIdAt(x, y) } as u16)
}

#[unsafe(no_mangle)]
pub unsafe fn InitRotatingTilePuzzle(is_trick_house: u8) {
    if puzzle().is_null() {
        unsafe { (&raw mut PUZZLE).write(AllocZeroed(PUZZLE_SIZE)) };
    }
    unsafe { puzzle().add(PUZZLE_IS_TRICK_HOUSE).write(is_trick_house) };
}

#[unsafe(no_mangle)]
pub unsafe fn FreeRotatingTilePuzzle() {
    if !puzzle().is_null() {
        unsafe { Free(puzzle()) };
        unsafe { (&raw mut PUZZLE).write(core::ptr::null_mut()) };
    }
    let id = unsafe { GetObjectEventIdByLocalIdAndMap(LOCALID_PLAYER, 0, 0) };
    let object = unsafe {
        (&raw mut (*(&raw const crate::field_player_avatar::gObjectEvents)
            .cast::<u8>()
            .cast_mut()))
            .cast::<u8>()
            .add(usize::from(id) * OBJECT_EVENT_SIZE)
    };
    unsafe { ObjectEventClearHeldMovementIfFinished(object) };
    unsafe { ScriptMovement_UnfreezeObjectEvents() };
}

#[unsafe(no_mangle)]
pub unsafe fn MoveRotatingTileObjects(puzzle_number: u8) -> u16 {
    let mut local_id = LOCALID_NONE;
    for i in 0..OBJECT_EVENT_TEMPLATES_COUNT {
        let t = unsafe { template(i) };
        let metatile = unsafe { metatile_under(t) };
        let start = unsafe { puzzle_tile_start() };

        // The lower bound is always the Mossdeep one; for the Trick House a
        // tile between the two starts gives a negative difference, cast to u8.
        if metatile < METATILE_MOSSDEEP_GYM_YELLOW_ARROW_RIGHT {
            continue;
        }
        let row = ((metatile - start) / METATILE_ROW_WIDTH) as u8;
        if row >= 5 || row != puzzle_number {
            continue;
        }
        let tile_num = ((metatile - start) % METATILE_ROW_WIDTH) as u8;
        // The first four puzzle tiles are the coloured arrows.
        let (script, dx, dy): (*const u8, i16, i16) = match tile_num {
            0 => (sMovement_ShiftRight.as_ptr(), 1, 0),
            1 => (sMovement_ShiftDown.as_ptr(), 0, 1),
            2 => (sMovement_ShiftLeft.as_ptr(), -1, 0),
            3 => (sMovement_ShiftUp.as_ptr(), 0, -1),
            _ => continue,
        };
        let x = unsafe { t.add(TEMPLATE_X).cast::<i16>() };
        let y = unsafe { t.add(TEMPLATE_Y).cast::<i16>() };
        unsafe { x.write(x.read().wrapping_add(dx)) };
        unsafe { y.write(y.read().wrapping_add(dy)) };

        let id = unsafe { t.add(TEMPLATE_LOCAL_ID).read() };
        let (map_num, map_group) = unsafe { location() };
        if unsafe { GetObjectEventIdByLocalIdAndMap(id, map_num, map_group) } != OBJECT_EVENTS_COUNT
        {
            unsafe { save_rotating_tile_object(i as u8, tile_num) };
            local_id = u16::from(id);
            unsafe { ScriptMovement_StartObjectMovementScript(id, map_num, map_group, script) };
        } else {
            // Never happens in normal play.
            unsafe { turn_unsaved_rotating_tile_object(i as u8, tile_num) };
        }
    }
    local_id
}

/// Every saved object has moved one step counter-clockwise, so the
/// difference is always -1 or 3 and the rotation always counter-clockwise;
/// the other cases are kept as in the original.
fn rotation_for(tile_difference: i8, strict: bool) -> i32 {
    if tile_difference < 0 || tile_difference == 3 {
        if strict && tile_difference == -3 {
            ROTATE_CLOCKWISE
        } else {
            ROTATE_COUNTERCLOCKWISE
        }
    } else if tile_difference > 0 || (!strict && tile_difference == -3) {
        ROTATE_CLOCKWISE
    } else {
        ROTATE_NONE
    }
}

#[unsafe(no_mangle)]
pub unsafe fn TurnRotatingTileObjects() {
    let p = puzzle();
    if p.is_null() {
        return;
    }
    let start = unsafe { puzzle_tile_start() };
    let count = unsafe { p.add(PUZZLE_NUM_OBJECTS).read() };
    for i in 0..usize::from(count) {
        let prev = unsafe { p.add(i * 2).read() };
        let template_id = usize::from(unsafe { p.add(i * 2 + 1).read() });
        let t = unsafe { template(template_id) };
        let metatile = unsafe { metatile_under(t) };

        let tile = ((metatile - start) % METATILE_ROW_WIDTH) as u8 as i8;
        let rotation = rotation_for(tile.wrapping_sub(prev as i8), true);

        let (map_num, map_group) = unsafe { location() };
        let local_id = unsafe { t.add(TEMPLATE_LOCAL_ID).read() };
        let object_id = unsafe { GetObjectEventIdByLocalIdAndMap(local_id, map_num, map_group) };
        if object_id == OBJECT_EVENTS_COUNT {
            continue;
        }
        let object = unsafe {
            (&raw const (*(&raw const crate::field_player_avatar::gObjectEvents)
                .cast::<u8>()
                .cast_mut()))
                .cast::<u8>()
                .add(usize::from(object_id) * OBJECT_EVENT_SIZE)
        };
        let direction = unsafe { object.add(OBJECT_EVENT_FACING_DIRECTION).read() } & 0xf;

        let turn = match rotation {
            ROTATE_COUNTERCLOCKWISE => match direction {
                DIR_EAST => (sMovement_FaceUp.as_ptr(), MOVEMENT_TYPE_FACE_UP),
                DIR_SOUTH => (sMovement_FaceRight.as_ptr(), MOVEMENT_TYPE_FACE_RIGHT),
                DIR_WEST => (sMovement_FaceDown.as_ptr(), MOVEMENT_TYPE_FACE_DOWN),
                DIR_NORTH => (sMovement_FaceLeft.as_ptr(), MOVEMENT_TYPE_FACE_LEFT),
                _ => continue,
            },
            ROTATE_CLOCKWISE => match direction {
                DIR_EAST => (sMovement_FaceDown.as_ptr(), MOVEMENT_TYPE_FACE_DOWN),
                DIR_SOUTH => (sMovement_FaceLeft.as_ptr(), MOVEMENT_TYPE_FACE_LEFT),
                DIR_WEST => (sMovement_FaceUp.as_ptr(), MOVEMENT_TYPE_FACE_UP),
                DIR_NORTH => (sMovement_FaceRight.as_ptr(), MOVEMENT_TYPE_FACE_RIGHT),
                _ => continue,
            },
            _ => continue,
        };
        unsafe { t.add(TEMPLATE_MOVEMENT_TYPE).write(turn.1) };
        unsafe { ScriptMovement_StartObjectMovementScript(local_id, map_num, map_group, turn.0) };
    }
}

unsafe fn save_rotating_tile_object(template_id: u8, puzzle_tile_num: u8) {
    let p = puzzle();
    let n = unsafe { p.add(PUZZLE_NUM_OBJECTS).read() };
    let slot = unsafe { p.add(usize::from(n) * 2) };
    unsafe { slot.add(1).write(template_id) };
    unsafe { slot.write(puzzle_tile_num) };
    unsafe { p.add(PUZZLE_NUM_OBJECTS).write(n.wrapping_add(1)) };
}

/// Functionally unused: turns an object whose event isn't loaded.
unsafe fn turn_unsaved_rotating_tile_object(template_id: u8, puzzle_tile_num: u8) {
    let t = unsafe { template(usize::from(template_id)) };
    let metatile = unsafe { metatile_under(t) };
    let start = unsafe { puzzle_tile_start() };
    let tile = ((metatile - start) % METATILE_ROW_WIDTH) as u8 as i8;
    let rotation = rotation_for(tile.wrapping_sub(puzzle_tile_num as i8), false);

    let movement_type = unsafe { t.add(TEMPLATE_MOVEMENT_TYPE) };
    let new_type = match (rotation, unsafe { movement_type.read() }) {
        (ROTATE_COUNTERCLOCKWISE, MOVEMENT_TYPE_FACE_RIGHT) => MOVEMENT_TYPE_FACE_UP,
        (ROTATE_COUNTERCLOCKWISE, MOVEMENT_TYPE_FACE_DOWN) => MOVEMENT_TYPE_FACE_RIGHT,
        (ROTATE_COUNTERCLOCKWISE, MOVEMENT_TYPE_FACE_LEFT) => MOVEMENT_TYPE_FACE_DOWN,
        (ROTATE_COUNTERCLOCKWISE, MOVEMENT_TYPE_FACE_UP) => MOVEMENT_TYPE_FACE_LEFT,
        (ROTATE_CLOCKWISE, MOVEMENT_TYPE_FACE_RIGHT) => MOVEMENT_TYPE_FACE_DOWN,
        (ROTATE_CLOCKWISE, MOVEMENT_TYPE_FACE_DOWN) => MOVEMENT_TYPE_FACE_LEFT,
        (ROTATE_CLOCKWISE, MOVEMENT_TYPE_FACE_LEFT) => MOVEMENT_TYPE_FACE_UP,
        (ROTATE_CLOCKWISE, MOVEMENT_TYPE_FACE_UP) => MOVEMENT_TYPE_FACE_RIGHT,
        (_, current) => current,
    };
    unsafe { movement_type.write(new_type) };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rotation_matches_the_original_branches() {
        assert_eq!(rotation_for(-1, true), ROTATE_COUNTERCLOCKWISE);
        assert_eq!(rotation_for(3, true), ROTATE_COUNTERCLOCKWISE);
        assert_eq!(rotation_for(-3, true), ROTATE_CLOCKWISE);
        assert_eq!(rotation_for(1, true), ROTATE_CLOCKWISE);
        assert_eq!(rotation_for(0, true), ROTATE_NONE);
        assert_eq!(rotation_for(-3, false), ROTATE_COUNTERCLOCKWISE);
    }
}
