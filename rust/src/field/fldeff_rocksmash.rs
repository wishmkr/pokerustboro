use crate::ffi::{
    CreateTask, DestroyTask, FieldCallback, MenuFieldCallback, PlaySE, gSpecialVar_LastTalked,
    gSpecialVar_Result, object_event, set_field_move_callback, set_task_func, sprite, task_data,
};
use core::ptr::{addr_of, addr_of_mut};

const MAP_TYPE_UNDERWATER: u8 = 5;
const FLDEFF_FIELD_MOVE_SHOW_MON: u8 = 6;
const FLDEFF_FIELD_MOVE_SHOW_MON_INIT: u8 = 59;
const FLDEFF_USE_ROCK_SMASH: u8 = 37;
const MOVEMENT_ACTION_START_ANIM_IN_DIRECTION: u8 = 57;
const OBJ_EVENT_GFX_BREAKABLE_ROCK: u8 = 86;
const SE_M_ROCK_THROW: u16 = 131;
const GAME_STAT_USED_ROCK_SMASH: u8 = 19;

const DIR_SOUTH: i32 = 1;
const DIR_NORTH: i32 = 2;
const DIR_WEST: i32 = 3;
const DIR_EAST: i32 = 4;

/// `offsetof(struct PlayerAvatar, spriteId)`
const PLAYER_AVATAR_SPRITE_ID: usize = 4;
/// `offsetof(struct PlayerAvatar, objectEventId)`
const PLAYER_AVATAR_OBJECT_EVENT_ID: usize = 5;
/// `offsetof(struct PlayerAvatar, preventStep)`
const PLAYER_AVATAR_PREVENT_STEP: usize = 6;

/// `offsetof(struct ObjectEvent, graphicsId)`
const OBJECT_EVENT_GRAPHICS_ID: usize = 5;
/// `offsetof(struct ObjectEvent, localId)`
const OBJECT_EVENT_LOCAL_ID: usize = 8;

/// `offsetof(struct MapHeader, mapType)`
const MAP_HEADER_MAP_TYPE: usize = 0x17;

/// `struct MapPosition { s16 x; s16 y; s8 elevation; }`
///
/// Five used bytes, but APCS rounds the structure up to eight.
#[repr(C, align(4))]
struct MapPosition {
    x: i16,
    y: i16,
    elevation: i8,
}

/// `GetXYCoordsOneStepInFrontOfPlayer` with this module's view of its types.
#[inline]
unsafe fn GetXYCoordsOneStepInFrontOfPlayer(a0: *mut i16, a1: *mut i16) {
    unsafe {
        crate::field_player_avatar::GetXYCoordsOneStepInFrontOfPlayer(a0 as _, a1 as _);
    }
}
/// `PlayerGetElevation` with this module's view of its types.
#[inline]
unsafe fn PlayerGetElevation() -> u8 {
    unsafe { crate::field_player_avatar::PlayerGetElevation() }
}
/// `GetObjectEventIdByPosition` with this module's view of its types.
#[inline]
unsafe fn GetObjectEventIdByPosition(a0: i16, a1: i16, a2: u8) -> u8 {
    unsafe { crate::event_object_movement::GetObjectEventIdByPosition(a0 as _, a1 as _, a2) }
}
/// `LockPlayerFieldControls` with this module's view of its types.
#[inline]
unsafe fn LockPlayerFieldControls() {
    unsafe {
        crate::script::LockPlayerFieldControls();
    }
}
/// `ObjectEventIsMovementOverridden` with this module's view of its types.
#[inline]
unsafe fn ObjectEventIsMovementOverridden(a0: *mut u8) -> u8 {
    unsafe { crate::event_object_movement::ObjectEventIsMovementOverridden(a0 as _) }
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
/// `ObjectEventCheckHeldMovementStatus` with this module's view of its types.
#[inline]
unsafe fn ObjectEventCheckHeldMovementStatus(a0: *mut u8) -> u8 {
    unsafe { crate::event_object_movement::ObjectEventCheckHeldMovementStatus(a0 as _) }
}
/// `ObjectEventSetGraphicsId` with this module's view of its types.
#[inline]
unsafe fn ObjectEventSetGraphicsId(a0: *mut u8, a1: u8) {
    unsafe {
        crate::event_object_movement::ObjectEventSetGraphicsId(a0 as _, a1);
    }
}
/// `SetPlayerAvatarFieldMove` with this module's view of its types.
#[inline]
unsafe fn SetPlayerAvatarFieldMove() {
    unsafe {
        crate::field_player_avatar::SetPlayerAvatarFieldMove();
    }
}
/// `GetPlayerFacingDirection` with this module's view of its types.
#[inline]
unsafe fn GetPlayerFacingDirection() -> u8 {
    unsafe { crate::field_player_avatar::GetPlayerFacingDirection() }
}
/// `GetPlayerAvatarGraphicsIdByCurrentState` with this module's view of its types.
#[inline]
unsafe fn GetPlayerAvatarGraphicsIdByCurrentState() -> u8 {
    unsafe { crate::field_player_avatar::GetPlayerAvatarGraphicsIdByCurrentState() }
}
/// `StartSpriteAnim` with this module's view of its types.
#[inline]
unsafe fn StartSpriteAnim(a0: *mut u8, a1: u8) {
    unsafe {
        crate::sprite::StartSpriteAnim(a0 as _, a1);
    }
}
/// `FieldEffectStart` with this module's view of its types.
#[inline]
unsafe fn FieldEffectStart(a0: u8) -> u32 {
    unsafe { crate::field_effect::FieldEffectStart(a0) }
}
/// `FieldEffectActiveListContains` with this module's view of its types.
#[inline]
unsafe fn FieldEffectActiveListContains(a0: u8) -> u8 {
    unsafe { crate::field_effect::FieldEffectActiveListContains(a0) }
}
/// `FieldEffectActiveListRemove` with this module's view of its types.
#[inline]
unsafe fn FieldEffectActiveListRemove(a0: u8) {
    unsafe {
        crate::field_effect::FieldEffectActiveListRemove(a0);
    }
}
/// `IncrementGameStat` with this module's view of its types.
#[inline]
unsafe fn IncrementGameStat(a0: u8) {
    unsafe {
        crate::overworld::IncrementGameStat(a0);
    }
}
/// `GetCursorSelectionMonId` with this module's view of its types.
#[inline]
unsafe fn GetCursorSelectionMonId() -> u8 {
    unsafe { crate::party_menu::GetCursorSelectionMonId() }
}
/// `FieldCallback_PrepareFadeInFromMenu` with this module's view of its types.
#[inline]
unsafe fn FieldCallback_PrepareFadeInFromMenu() -> u8 {
    unsafe { crate::party_menu::FieldCallback_PrepareFadeInFromMenu() }
}
/// `ShouldDoBrailleRegirockEffect` with this module's view of its types.
#[inline]
unsafe fn ShouldDoBrailleRegirockEffect() -> u8 {
    unsafe { crate::braille_puzzles::ShouldDoBrailleRegirockEffect() }
}
/// `SetUpPuzzleEffectRegirock` with this module's view of its types.
#[inline]
unsafe fn SetUpPuzzleEffectRegirock() {
    unsafe {
        crate::braille_puzzles::SetUpPuzzleEffectRegirock();
    }
}
/// `ScriptContext_SetupScript` with this module's view of its types.
#[inline]
unsafe fn ScriptContext_SetupScript(a0: *const u8) {
    unsafe {
        crate::script::ScriptContext_SetupScript(a0 as _);
    }
}
/// `ScriptContext_Enable` with this module's view of its types.
#[inline]
unsafe fn ScriptContext_Enable() {
    unsafe {
        crate::script::ScriptContext_Enable();
    }
}

#[inline]
unsafe fn player_object_event_id() -> usize {
    unsafe {
        (&raw const (*(&raw const crate::field_player_avatar::gPlayerAvatar)
            .cast::<u8>()
            .cast_mut()))
            .add(PLAYER_AVATAR_OBJECT_EVENT_ID)
            .read_volatile() as usize
    }
}

#[inline]
unsafe fn player_sprite_id() -> usize {
    unsafe {
        (&raw const (*(&raw const crate::field_player_avatar::gPlayerAvatar)
            .cast::<u8>()
            .cast_mut()))
            .add(PLAYER_AVATAR_SPRITE_ID)
            .read_volatile() as usize
    }
}

#[inline]
unsafe fn set_prevent_step(value: u8) {
    unsafe {
        (&raw mut (*(&raw const crate::field_player_avatar::gPlayerAvatar)
            .cast::<u8>()
            .cast_mut()))
            .add(PLAYER_AVATAR_PREVENT_STEP)
            .write_volatile(value)
    };
}

#[inline]
unsafe fn field_effect_argument(index: usize) -> i32 {
    unsafe {
        (&raw const (*(&raw const crate::field_effect::gFieldEffectArguments)
            .cast::<[i32; 8]>()
            .cast_mut()))
            .cast::<i32>()
            .add(index)
            .read()
    }
}

#[inline]
unsafe fn set_field_effect_argument(index: usize, value: i32) {
    unsafe {
        (&raw mut (*(&raw const crate::field_effect::gFieldEffectArguments)
            .cast::<[i32; 8]>()
            .cast_mut()))
            .cast::<i32>()
            .add(index)
            .write(value)
    };
}

#[unsafe(no_mangle)]
pub unsafe fn CheckObjectGraphicsInFrontOfPlayer(graphics_id: u8) -> u8 {
    unsafe {
        GetXYCoordsOneStepInFrontOfPlayer(
            &raw mut (*(&raw const crate::fldeff_misc::gPlayerFacingPosition)
                .cast::<MapPosition>()
                .cast_mut())
            .x,
            &raw mut (*(&raw const crate::fldeff_misc::gPlayerFacingPosition)
                .cast::<MapPosition>()
                .cast_mut())
            .y,
        )
    };
    unsafe {
        addr_of_mut!(
            (*(&raw const crate::fldeff_misc::gPlayerFacingPosition)
                .cast::<MapPosition>()
                .cast_mut())
            .elevation
        )
        .write(PlayerGetElevation() as i8)
    };

    let object_event_id = unsafe {
        GetObjectEventIdByPosition(
            addr_of!(
                (*(&raw const crate::fldeff_misc::gPlayerFacingPosition)
                    .cast::<MapPosition>()
                    .cast_mut())
                .x
            )
            .read(),
            addr_of!(
                (*(&raw const crate::fldeff_misc::gPlayerFacingPosition)
                    .cast::<MapPosition>()
                    .cast_mut())
                .y
            )
            .read(),
            addr_of!(
                (*(&raw const crate::fldeff_misc::gPlayerFacingPosition)
                    .cast::<MapPosition>()
                    .cast_mut())
                .elevation
            )
            .read() as u8,
        )
    } as usize;

    let found = unsafe {
        object_event(object_event_id)
            .add(OBJECT_EVENT_GRAPHICS_ID)
            .read_volatile()
    };
    if found != graphics_id {
        return 0;
    }

    let local_id = unsafe {
        object_event(object_event_id)
            .add(OBJECT_EVENT_LOCAL_ID)
            .read_volatile()
    };
    unsafe { addr_of_mut!(gSpecialVar_LastTalked).write(u16::from(local_id)) };
    1
}

#[unsafe(no_mangle)]
pub unsafe fn CreateFieldMoveTask() -> u8 {
    unsafe {
        GetXYCoordsOneStepInFrontOfPlayer(
            &raw mut (*(&raw const crate::fldeff_misc::gPlayerFacingPosition)
                .cast::<MapPosition>()
                .cast_mut())
            .x,
            &raw mut (*(&raw const crate::fldeff_misc::gPlayerFacingPosition)
                .cast::<MapPosition>()
                .cast_mut())
            .y,
        )
    };
    unsafe { CreateTask(task_do_field_move_init, 8) }
}

unsafe fn task_do_field_move_init(task_id: u8) {
    unsafe { LockPlayerFieldControls() };
    unsafe { set_prevent_step(1) };

    let object = unsafe { object_event(player_object_event_id()) };
    if unsafe { ObjectEventIsMovementOverridden(object) } != 0
        && unsafe { ObjectEventClearHeldMovementIfFinished(object) } == 0
    {
        return;
    }

    let map_type = unsafe {
        (&raw const (*(&raw const crate::fieldmap::gMapHeader).cast::<u8>()))
            .add(MAP_HEADER_MAP_TYPE)
            .read_volatile()
    };
    if map_type == MAP_TYPE_UNDERWATER {
        // The field move pose is skipped underwater.
        let _ = unsafe { FieldEffectStart(FLDEFF_FIELD_MOVE_SHOW_MON_INIT) };
        unsafe { set_task_func(task_id, task_do_field_move_wait_for_mon) };
    } else {
        unsafe { SetPlayerAvatarFieldMove() };
        let _ =
            unsafe { ObjectEventSetHeldMovement(object, MOVEMENT_ACTION_START_ANIM_IN_DIRECTION) };
        unsafe { set_task_func(task_id, task_do_field_move_show_mon_after_pose) };
    }
}

unsafe fn task_do_field_move_show_mon_after_pose(task_id: u8) {
    let object = unsafe { object_event(player_object_event_id()) };
    if unsafe { ObjectEventCheckHeldMovementStatus(object) } == 1 {
        let _ = unsafe { FieldEffectStart(FLDEFF_FIELD_MOVE_SHOW_MON_INIT) };
        unsafe { set_task_func(task_id, task_do_field_move_wait_for_mon) };
    }
}

unsafe fn task_do_field_move_wait_for_mon(task_id: u8) {
    if unsafe { FieldEffectActiveListContains(FLDEFF_FIELD_MOVE_SHOW_MON) } != 0 {
        return;
    }

    unsafe { set_field_effect_argument(1, i32::from(GetPlayerFacingDirection())) };
    let direction = unsafe { field_effect_argument(1) };
    // The original tests each direction separately and leaves argument 2
    // untouched for any other value.
    if direction == DIR_SOUTH {
        unsafe { set_field_effect_argument(2, 0) };
    }
    if direction == DIR_NORTH {
        unsafe { set_field_effect_argument(2, 1) };
    }
    if direction == DIR_WEST {
        unsafe { set_field_effect_argument(2, 2) };
    }
    if direction == DIR_EAST {
        unsafe { set_field_effect_argument(2, 3) };
    }

    let object = unsafe { object_event(player_object_event_id()) };
    unsafe { ObjectEventSetGraphicsId(object, GetPlayerAvatarGraphicsIdByCurrentState()) };
    unsafe { StartSpriteAnim(sprite(player_sprite_id()), field_effect_argument(2) as u8) };
    unsafe { FieldEffectActiveListRemove(FLDEFF_FIELD_MOVE_SHOW_MON) };
    unsafe { set_task_func(task_id, task_do_field_move_run_func) };
}

unsafe fn task_do_field_move_run_func(task_id: u8) {
    // The field-move function is stored in halves across data[8] and data[9].
    let high = unsafe { task_data(task_id, 8) } as u16;
    let low = unsafe { task_data(task_id, 9) } as u16;
    let address = (u32::from(high) << 16) | u32::from(low);
    let field_move_func: MenuFieldCallback = unsafe { core::mem::transmute(address as usize) };

    unsafe { field_move_func() };
    unsafe { set_prevent_step(0) };
    unsafe { DestroyTask(task_id) };
}

/// Called when Rock Smash is used from the party menu. Interacting with a
/// smashable rock in the field goes through `EventScript_RockSmash` instead.
#[unsafe(no_mangle)]
pub unsafe fn SetUpFieldMove_RockSmash() -> u8 {
    // Ruby and Sapphire open Regirock's tomb with Strength; Emerald uses
    // Rock Smash.
    if unsafe { ShouldDoBrailleRegirockEffect() } != 0 {
        unsafe { addr_of_mut!(gSpecialVar_Result).write(u16::from(GetCursorSelectionMonId())) };
        unsafe {
            addr_of_mut!(
                (*(&raw const crate::overworld::gFieldCallback2)
                    .cast::<Option<FieldCallback>>()
                    .cast_mut())
            )
            .write(Some(FieldCallback_PrepareFadeInFromMenu))
        };
        unsafe {
            addr_of_mut!(
                (*(&raw const crate::party_menu::gPostMenuFieldCallback)
                    .cast::<Option<MenuFieldCallback>>()
                    .cast_mut())
            )
            .write(Some(SetUpPuzzleEffectRegirock))
        };
        1
    } else if unsafe { CheckObjectGraphicsInFrontOfPlayer(OBJ_EVENT_GFX_BREAKABLE_ROCK) } == 1 {
        unsafe {
            addr_of_mut!(
                (*(&raw const crate::overworld::gFieldCallback2)
                    .cast::<Option<FieldCallback>>()
                    .cast_mut())
            )
            .write(Some(FieldCallback_PrepareFadeInFromMenu))
        };
        unsafe {
            addr_of_mut!(
                (*(&raw const crate::party_menu::gPostMenuFieldCallback)
                    .cast::<Option<MenuFieldCallback>>()
                    .cast_mut())
            )
            .write(Some(field_callback_rock_smash))
        };
        1
    } else {
        0
    }
}

unsafe fn field_callback_rock_smash() {
    unsafe { set_field_effect_argument(0, i32::from(GetCursorSelectionMonId())) };
    unsafe {
        ScriptContext_SetupScript(addr_of!(
            (*crate::asmdata::EventScript_UseRockSmash.cast::<u8>())
        ))
    };
}

#[unsafe(no_mangle)]
pub unsafe fn FldEff_UseRockSmash() -> u8 {
    let task_id = unsafe { CreateFieldMoveTask() };
    unsafe { set_field_move_callback(task_id, field_move_rock_smash) };
    unsafe { IncrementGameStat(GAME_STAT_USED_ROCK_SMASH) };
    0
}

/// `EventScript_SmashRock` does the actual smashing, so this only plays the
/// sound and hands control back to the script.
unsafe fn field_move_rock_smash() {
    unsafe { PlaySE(SE_M_ROCK_THROW) };
    unsafe { FieldEffectActiveListRemove(FLDEFF_USE_ROCK_SMASH) };
    unsafe { ScriptContext_Enable() };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn map_position_layout_matches_the_arm_structure() {
        assert_eq!(core::mem::size_of::<MapPosition>(), 8);
        assert_eq!(core::mem::offset_of!(MapPosition, x), 0);
        assert_eq!(core::mem::offset_of!(MapPosition, y), 2);
        assert_eq!(core::mem::offset_of!(MapPosition, elevation), 4);
    }

    #[test]
    fn the_callback_halves_rebuild_the_original_address() {
        let address = 0x0812_3456u32;
        let high = (address >> 16) as i16;
        let low = address as i16;
        let rebuilt = (u32::from(high as u16) << 16) | u32::from(low as u16);
        assert_eq!(rebuilt, address);
    }
}
