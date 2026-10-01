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

unsafe extern "C" {
    static mut gFieldCallback2: Option<FieldCallback>;
    static mut gPostMenuFieldCallback: Option<MenuFieldCallback>;
    static mut gFieldEffectArguments: [i32; 8];
    static gMapHeader: MapHeaderView;
    static EventScript_UseStrength: u8;

    fn FieldCallback_PrepareFadeInFromMenu() -> u8;
    fn GetCursorSelectionMonId() -> u8;
    fn CreateFieldMoveTask() -> u8;
    fn FieldEffectStart(effect: u8) -> u32;
    fn FieldEffectActiveListRemove(effect: u8);
    fn SetPlayerAvatarTransitionFlags(flags: u16);

    fn Overworld_MapTypeAllowsTeleportAndFly(map_type: u8) -> u8;
    fn Overworld_ResetStateAfterTeleport();
    fn FldEff_TeleportWarpOut();

    fn CheckObjectGraphicsInFrontOfPlayer(graphics_id: u8) -> u8;
    fn ScriptContext_SetupScript(script: *const u8);
    fn ScriptContext_Enable();
    fn GetMonNickname(mon: *mut u8, destination: *mut u8) -> *mut u8;

    fn CanUseDigOrEscapeRopeOnCurMap() -> u8;
    fn Overworld_ResetStateAfterDigEscRope();
    fn ShouldDoBrailleDigEffect() -> u8;
    fn DoBrailleDigEffect();
    fn Task_UseDigEscapeRopeOnField(task_id: u8);
}

unsafe extern "C" fn field_callback_teleport() {
    unsafe { Overworld_ResetStateAfterTeleport() };
    let _ = unsafe { FieldEffectStart(63) };
    unsafe {
        (&raw mut gFieldEffectArguments)
            .cast::<i32>()
            .write(i32::from(GetCursorSelectionMonId()))
    };
}

unsafe extern "C" fn start_teleport_field_effect() {
    unsafe { FieldEffectActiveListRemove(63) };
    unsafe { FldEff_TeleportWarpOut() };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetUpFieldMove_Teleport() -> u8 {
    let map_type = unsafe { addr_of!(gMapHeader.map_type).read() };
    if unsafe { Overworld_MapTypeAllowsTeleportAndFly(map_type) } != 0 {
        unsafe { addr_of_mut!(gFieldCallback2).write(Some(FieldCallback_PrepareFadeInFromMenu)) };
        unsafe { addr_of_mut!(gPostMenuFieldCallback).write(Some(field_callback_teleport)) };
        1
    } else {
        0
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn FldEff_UseTeleport() -> u8 {
    let task_id = unsafe { CreateFieldMoveTask() };
    unsafe { set_field_move_callback(task_id, start_teleport_field_effect) };
    unsafe { SetPlayerAvatarTransitionFlags(1) };
    0
}

unsafe extern "C" fn field_callback_strength() {
    unsafe {
        (&raw mut gFieldEffectArguments)
            .cast::<i32>()
            .write(i32::from(GetCursorSelectionMonId()))
    };
    unsafe { ScriptContext_SetupScript(addr_of!(EventScript_UseStrength)) };
}

unsafe extern "C" fn start_strength_field_effect() {
    unsafe { FieldEffectActiveListRemove(40) };
    unsafe { ScriptContext_Enable() };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetUpFieldMove_Strength() -> u8 {
    if unsafe { CheckObjectGraphicsInFrontOfPlayer(87) } != 0 {
        unsafe { addr_of_mut!(gSpecialVar_Result).write(u16::from(GetCursorSelectionMonId())) };
        unsafe { addr_of_mut!(gFieldCallback2).write(Some(FieldCallback_PrepareFadeInFromMenu)) };
        unsafe { addr_of_mut!(gPostMenuFieldCallback).write(Some(field_callback_strength)) };
        1
    } else {
        0
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn FldEff_UseStrength() -> u8 {
    let task_id = unsafe { CreateFieldMoveTask() };
    unsafe { set_field_move_callback(task_id, start_strength_field_effect) };
    let party_index = unsafe { (&raw const gFieldEffectArguments).cast::<i32>().read() as usize };
    let mon = unsafe { party_mon(party_index) };
    let _ = unsafe { GetMonNickname(mon, (&raw mut gStringVar1).cast::<u8>()) };
    0
}

unsafe extern "C" fn field_callback_dig() {
    unsafe { Overworld_ResetStateAfterDigEscRope() };
    let _ = unsafe { FieldEffectStart(38) };
    unsafe {
        (&raw mut gFieldEffectArguments)
            .cast::<i32>()
            .write(i32::from(GetCursorSelectionMonId()))
    };
}

unsafe extern "C" fn start_dig_field_effect() {
    unsafe { FieldEffectActiveListRemove(38) };
    if unsafe { ShouldDoBrailleDigEffect() } != 0 {
        unsafe { DoBrailleDigEffect() };
    } else {
        let task_id = unsafe { CreateTask(Task_UseDigEscapeRopeOnField, 8) };
        unsafe { set_task_data(task_id, 0, 0) };
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetUpFieldMove_Dig() -> u8 {
    if unsafe { CanUseDigOrEscapeRopeOnCurMap() } != 0 {
        unsafe { addr_of_mut!(gFieldCallback2).write(Some(FieldCallback_PrepareFadeInFromMenu)) };
        unsafe { addr_of_mut!(gPostMenuFieldCallback).write(Some(field_callback_dig)) };
        1
    } else {
        0
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn FldEff_UseDig() -> u8 {
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
