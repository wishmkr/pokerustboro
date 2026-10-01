use crate::ffi::{
    CreateTask, FieldCallback, MenuFieldCallback, gSpecialVar_Result, gStringVar1, party_mon,
    set_field_move_callback, set_task_data,
};
use core::ptr::{addr_of, addr_of_mut};

#[repr(C)]
struct MapHeaderView {
    prefix: [u8; 0x17],
    map_type: u8,
}

/// `FieldCallback_PrepareFadeInFromMenu` with this module's view of its types.
#[inline]
unsafe fn FieldCallback_PrepareFadeInFromMenu() -> u8 {
    unsafe { crate::party_menu::FieldCallback_PrepareFadeInFromMenu() }
}
/// `GetCursorSelectionMonId` with this module's view of its types.
#[inline]
unsafe fn GetCursorSelectionMonId() -> u8 {
    unsafe { crate::party_menu::GetCursorSelectionMonId() }
}
/// `CreateFieldMoveTask` with this module's view of its types.
#[inline]
unsafe fn CreateFieldMoveTask() -> u8 {
    unsafe { crate::fldeff_rocksmash::CreateFieldMoveTask() }
}
/// `FieldEffectStart` with this module's view of its types.
#[inline]
unsafe fn FieldEffectStart(a0: u8) -> u32 {
    unsafe { crate::field_effect::FieldEffectStart(a0) }
}
/// `FieldEffectActiveListRemove` with this module's view of its types.
#[inline]
unsafe fn FieldEffectActiveListRemove(a0: u8) {
    unsafe {
        crate::field_effect::FieldEffectActiveListRemove(a0);
    }
}
/// `SetPlayerAvatarTransitionFlags` with this module's view of its types.
#[inline]
unsafe fn SetPlayerAvatarTransitionFlags(a0: u16) {
    unsafe {
        crate::field_player_avatar::SetPlayerAvatarTransitionFlags(a0);
    }
}
/// `Overworld_MapTypeAllowsTeleportAndFly` with this module's view of its types.
#[inline]
unsafe fn Overworld_MapTypeAllowsTeleportAndFly(a0: u8) -> u8 {
    unsafe { crate::overworld::Overworld_MapTypeAllowsTeleportAndFly(a0) }
}
/// `Overworld_ResetStateAfterTeleport` with this module's view of its types.
#[inline]
unsafe fn Overworld_ResetStateAfterTeleport() {
    unsafe {
        crate::overworld::Overworld_ResetStateAfterTeleport();
    }
}
/// `FldEff_TeleportWarpOut` with this module's view of its types.
#[inline]
unsafe fn FldEff_TeleportWarpOut() {
    unsafe {
        crate::field_effect::FldEff_TeleportWarpOut();
    }
}
/// `CheckObjectGraphicsInFrontOfPlayer` with this module's view of its types.
#[inline]
unsafe fn CheckObjectGraphicsInFrontOfPlayer(a0: u8) -> u8 {
    unsafe { crate::fldeff_rocksmash::CheckObjectGraphicsInFrontOfPlayer(a0) }
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
/// `GetMonNickname` with this module's view of its types.
#[inline]
unsafe fn GetMonNickname(a0: *mut u8, a1: *mut u8) -> *mut u8 {
    unsafe { crate::party_menu::GetMonNickname(a0 as _, a1 as _) as *mut u8 }
}
/// `CanUseDigOrEscapeRopeOnCurMap` with this module's view of its types.
#[inline]
unsafe fn CanUseDigOrEscapeRopeOnCurMap() -> u8 {
    unsafe { crate::item_use::CanUseDigOrEscapeRopeOnCurMap() }
}
/// `Overworld_ResetStateAfterDigEscRope` with this module's view of its types.
#[inline]
unsafe fn Overworld_ResetStateAfterDigEscRope() {
    unsafe {
        crate::overworld::Overworld_ResetStateAfterDigEscRope();
    }
}
/// `ShouldDoBrailleDigEffect` with this module's view of its types.
#[inline]
unsafe fn ShouldDoBrailleDigEffect() -> u8 {
    unsafe { crate::braille_puzzles::ShouldDoBrailleDigEffect() }
}
/// `DoBrailleDigEffect` with this module's view of its types.
#[inline]
unsafe fn DoBrailleDigEffect() {
    unsafe {
        crate::braille_puzzles::DoBrailleDigEffect();
    }
}
/// `Task_UseDigEscapeRopeOnField` with this module's view of its types.
#[inline]
unsafe fn Task_UseDigEscapeRopeOnField(a0: u8) {
    unsafe {
        crate::item_use::Task_UseDigEscapeRopeOnField(a0);
    }
}

unsafe fn field_callback_teleport() {
    unsafe { Overworld_ResetStateAfterTeleport() };
    let _ = unsafe { FieldEffectStart(63) };
    unsafe {
        (&raw mut (*(&raw const crate::field_effect::gFieldEffectArguments)
            .cast::<[i32; 8]>()
            .cast_mut()))
            .cast::<i32>()
            .write(i32::from(GetCursorSelectionMonId()))
    };
}

unsafe fn start_teleport_field_effect() {
    unsafe { FieldEffectActiveListRemove(63) };
    unsafe { FldEff_TeleportWarpOut() };
}

#[unsafe(no_mangle)]
pub unsafe fn SetUpFieldMove_Teleport() -> u8 {
    let map_type = unsafe {
        addr_of!((*(&raw const crate::fieldmap::gMapHeader).cast::<MapHeaderView>()).map_type)
            .read()
    };
    if unsafe { Overworld_MapTypeAllowsTeleportAndFly(map_type) } != 0 {
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
            .write(Some(field_callback_teleport))
        };
        1
    } else {
        0
    }
}

#[unsafe(no_mangle)]
pub unsafe fn FldEff_UseTeleport() -> u8 {
    let task_id = unsafe { CreateFieldMoveTask() };
    unsafe { set_field_move_callback(task_id, start_teleport_field_effect) };
    unsafe { SetPlayerAvatarTransitionFlags(1) };
    0
}

unsafe fn field_callback_strength() {
    unsafe {
        (&raw mut (*(&raw const crate::field_effect::gFieldEffectArguments)
            .cast::<[i32; 8]>()
            .cast_mut()))
            .cast::<i32>()
            .write(i32::from(GetCursorSelectionMonId()))
    };
    unsafe {
        ScriptContext_SetupScript(addr_of!(
            (*crate::asmdata::EventScript_UseStrength.cast::<u8>())
        ))
    };
}

unsafe fn start_strength_field_effect() {
    unsafe { FieldEffectActiveListRemove(40) };
    unsafe { ScriptContext_Enable() };
}

#[unsafe(no_mangle)]
pub unsafe fn SetUpFieldMove_Strength() -> u8 {
    if unsafe { CheckObjectGraphicsInFrontOfPlayer(87) } != 0 {
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
            .write(Some(field_callback_strength))
        };
        1
    } else {
        0
    }
}

#[unsafe(no_mangle)]
pub unsafe fn FldEff_UseStrength() -> u8 {
    let task_id = unsafe { CreateFieldMoveTask() };
    unsafe { set_field_move_callback(task_id, start_strength_field_effect) };
    let party_index = unsafe {
        (&raw const (*(&raw const crate::field_effect::gFieldEffectArguments)
            .cast::<[i32; 8]>()
            .cast_mut()))
            .cast::<i32>()
            .read() as usize
    };
    let mon = unsafe { party_mon(party_index) };
    let _ = unsafe { GetMonNickname(mon, (&raw mut gStringVar1).cast::<u8>()) };
    0
}

unsafe fn field_callback_dig() {
    unsafe { Overworld_ResetStateAfterDigEscRope() };
    let _ = unsafe { FieldEffectStart(38) };
    unsafe {
        (&raw mut (*(&raw const crate::field_effect::gFieldEffectArguments)
            .cast::<[i32; 8]>()
            .cast_mut()))
            .cast::<i32>()
            .write(i32::from(GetCursorSelectionMonId()))
    };
}

unsafe fn start_dig_field_effect() {
    unsafe { FieldEffectActiveListRemove(38) };
    if unsafe { ShouldDoBrailleDigEffect() } != 0 {
        unsafe { DoBrailleDigEffect() };
    } else {
        let task_id = unsafe { CreateTask(Task_UseDigEscapeRopeOnField, 8) };
        unsafe { set_task_data(task_id, 0, 0) };
    }
}

#[unsafe(no_mangle)]
pub unsafe fn SetUpFieldMove_Dig() -> u8 {
    if unsafe { CanUseDigOrEscapeRopeOnCurMap() } != 0 {
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
            .write(Some(field_callback_dig))
        };
        1
    } else {
        0
    }
}

#[unsafe(no_mangle)]
pub unsafe fn FldEff_UseDig() -> u8 {
    let task_id = unsafe { CreateFieldMoveTask() };
    unsafe { set_field_move_callback(task_id, start_dig_field_effect) };
    if unsafe { ShouldDoBrailleDigEffect() } == 0 {
        unsafe { SetPlayerAvatarTransitionFlags(1) };
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn map_type_offset_matches_map_header() {
        assert_eq!(core::mem::offset_of!(MapHeaderView, map_type), 0x17);
    }
}
