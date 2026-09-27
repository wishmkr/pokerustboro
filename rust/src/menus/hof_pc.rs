use core::ptr::{addr_of, addr_of_mut};

type Callback = unsafe extern "C" fn();
type TaskFunc = unsafe extern "C" fn(u8);

#[repr(C)]
struct PaletteFadeActiveView {
    prefix: [u8; 7],
    active_byte: u8,
}

unsafe extern "C" {
    static mut gFieldCallback: Option<Callback>;
    static mut gPaletteFade: PaletteFadeActiveView;

    fn CB2_DoHallOfFamePC();
    fn CB2_ReturnToField();
    fn SetMainCallback2(callback: Callback);
    fn LockPlayerFieldControls();
    fn Overworld_PlaySpecialMapMusic();
    fn ScriptMenu_CreatePCMultichoice();
    fn ScriptMenu_DisplayPCStartupPrompt();
    fn BeginNormalPaletteFade(
        selected_palettes: u32,
        delay: i8,
        start_y: u8,
        target_y: u8,
        blend_color: u16,
    ) -> u8;
    fn CreateTask(function: TaskFunc, priority: u8) -> u8;
    fn DestroyTask(task_id: u8);
}

unsafe extern "C" fn reshow_pc_menu_after_hall_of_fame_pc() {
    unsafe { LockPlayerFieldControls() };
    unsafe { Overworld_PlaySpecialMapMusic() };
    unsafe { ScriptMenu_CreatePCMultichoice() };
    unsafe { ScriptMenu_DisplayPCStartupPrompt() };
    let _ = unsafe { BeginNormalPaletteFade(u32::MAX, 0, 0x10, 0, 0) };
    let _ = unsafe { CreateTask(task_wait_for_palette_fade, 10) };
}

const fn palette_fade_is_active(active_byte: u8) -> bool {
    active_byte & 0x80 != 0
}

unsafe extern "C" fn task_wait_for_palette_fade(task_id: u8) {
    let active_byte = unsafe { addr_of!(gPaletteFade.active_byte).read() };
    if !palette_fade_is_active(active_byte) {
        unsafe { DestroyTask(task_id) };
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn AccessHallOfFamePC() {
    unsafe { SetMainCallback2(CB2_DoHallOfFamePC) };
    unsafe { LockPlayerFieldControls() };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ReturnFromHallOfFamePC() {
    unsafe { SetMainCallback2(CB2_ReturnToField) };
    unsafe { addr_of_mut!(gFieldCallback).write(Some(reshow_pc_menu_after_hall_of_fame_pc)) };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn palette_active_bit_matches_the_c_bitfield_layout() {
        assert_eq!(core::mem::offset_of!(PaletteFadeActiveView, active_byte), 7);
        assert!(!palette_fade_is_active(0x7f));
        assert!(palette_fade_is_active(0x80));
        assert!(palette_fade_is_active(0xff));
    }
}
