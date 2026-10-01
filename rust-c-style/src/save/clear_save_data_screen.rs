//! The "Clear all save data?" screen reached with Up + Select + B on boot.

use crate::bg::{InitBgsFromTemplates, ResetBgsAndClearDma3BusyFlags, ShowBg};
use crate::ffi::{
    AddTextPrinterParameterized, BgTemplate, DUMMY_WIN_TEMPLATE, DestroyTask,
    DrawStdFrameWithCustomTileAndPalette, FONT_NORMAL, MAIN_STATE_OFFSET, MENU_B_PRESSED,
    MainCallback, PALETTES_BG, PLTT_SIZE, PLTT_SIZE_4BPP, PlaySE, RGB_WHITE, RGB_WHITEALPHA,
    SE_SELECT, WindowTemplate, dma3_fill_large, gPlttBufferFaded, gPlttBufferUnfaded,
    palette_fade_active, rgb, set_task_func,
};
use crate::gpu_regs::{EnableInterrupts, SetGpuReg};
use crate::sprite::ResetSpriteData;
use crate::task::{CreateTask, ResetTasks, RunTasks};
use crate::text_window::LoadWindowGfx;
use crate::window::{FillWindowPixelBuffer, FreeAllWindowBuffers, InitWindows};

const VRAM: usize = 0x0600_0000;
const VRAM_SIZE: u32 = 0x1_8000;
const OAM: usize = 0x0700_0000;
const OAM_SIZE: u32 = 0x400;
const PLTT: usize = 0x0500_0000;
/// `BG_SCREEN_ADDR(30)`
const BG_SCREEN_30: usize = VRAM + 30 * 0x800;

const REG_OFFSET_DISPCNT: u8 = 0x00;
const REG_OFFSET_BG0HOFS: u8 = 0x10;
const REG_OFFSET_BG0VOFS: u8 = 0x12;
const REG_OFFSET_BG3HOFS: u8 = 0x1c;
const REG_OFFSET_BG3VOFS: u8 = 0x1e;
const REG_OFFSET_WIN0H: u8 = 0x40;
const REG_OFFSET_WIN0V: u8 = 0x44;
const REG_OFFSET_WININ: u8 = 0x48;
const REG_OFFSET_WINOUT: u8 = 0x4a;
const REG_OFFSET_BLDCNT: u8 = 0x50;
const REG_OFFSET_BLDALPHA: u8 = 0x52;
const REG_OFFSET_BLDY: u8 = 0x54;
const DISPCNT_MODE_0: u16 = 0;
const DISPCNT_OBJ_1D_MAP: u16 = 0x0040;
const DISPCNT_OBJ_ON: u16 = 0x1000;
const INTR_FLAG_VBLANK: u16 = 1;

static BG_TEMPLATES: [BgTemplate; 2] = [
    BgTemplate::new(0, 0, 31, 0, 0, 0, 0),
    BgTemplate::new(3, 0, 30, 0, 0, 1, 0),
];

static TEXT_WINDOW: [WindowTemplate; 2] = [
    WindowTemplate {
        bg: 0,
        tilemap_left: 3,
        tilemap_top: 15,
        width: 26,
        height: 4,
        palette_num: 15,
        base_block: 11,
    },
    DUMMY_WIN_TEMPLATE,
];

static YES_NO: WindowTemplate = WindowTemplate {
    bg: 0,
    tilemap_left: 3,
    tilemap_top: 2,
    width: 5,
    height: 4,
    palette_num: 15,
    base_block: 115,
};

unsafe extern "C" {
    static gText_ClearAllSaveData: u8;
    static gText_ClearingData: u8;
    static gStandardMenuPalette: u16;
    static mut gMain: u8;

    fn SetMainCallback2(callback: MainCallback);
    fn SetVBlankCallback(callback: Option<unsafe extern "C" fn()>);
    fn BeginNormalPaletteFade(
        selected_palettes: u32,
        delay: i8,
        start_y: u8,
        target_y: u8,
        blend_color: u16,
    ) -> u8;
    fn UpdatePaletteFade() -> u8;
    fn TransferPlttBuffer();
    fn ResetPaletteFade();
    fn LoadPalette(src: *const core::ffi::c_void, offset: u16, size: u16);
    fn DoSoftReset();
    fn CreateYesNoMenu(
        window: *const WindowTemplate,
        base_tile_num: u16,
        palette_num: u8,
        initial_cursor_pos: u8,
    );
    fn Menu_ProcessInputNoWrapClearOnChoose() -> i8;
    fn DeactivateAllTextPrinters();
    fn ClearSaveData();
}

#[inline]
unsafe fn main_state() -> *mut u8 {
    unsafe { (&raw mut gMain).cast::<u8>().add(MAIN_STATE_OFFSET) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn CB2_InitClearSaveDataScreen() {
    if unsafe { setup_screen() } {
        unsafe { CreateTask(task_do_yes_no, 0) };
    }
}

unsafe extern "C" fn task_do_yes_no(task_id: u8) {
    unsafe { DrawStdFrameWithCustomTileAndPalette(0, 0, 2, 14) };
    unsafe {
        AddTextPrinterParameterized(
            0,
            FONT_NORMAL,
            &raw const gText_ClearAllSaveData,
            0,
            1,
            0,
            None,
        )
    };
    unsafe { CreateYesNoMenu(&raw const YES_NO, 2, 14, 1) };
    unsafe { set_task_func(task_id, task_yes_no_choice) };
}

unsafe extern "C" fn task_yes_no_choice(task_id: u8) {
    match unsafe { Menu_ProcessInputNoWrapClearOnChoose() } {
        0 => {
            unsafe { FillWindowPixelBuffer(0, 0x11) };
            unsafe {
                AddTextPrinterParameterized(
                    0,
                    FONT_NORMAL,
                    &raw const gText_ClearingData,
                    0,
                    1,
                    0,
                    None,
                )
            };
            unsafe { set_task_func(task_id, task_clear_save_data) };
        }
        1 | MENU_B_PRESSED => {
            unsafe { PlaySE(SE_SELECT) };
            unsafe { DestroyTask(task_id) };
            unsafe { SetMainCallback2(cb2_fade_and_do_reset) };
        }
        _ => {}
    }
}

unsafe extern "C" fn task_clear_save_data(task_id: u8) {
    unsafe { ClearSaveData() };
    unsafe { DestroyTask(task_id) };
    unsafe { SetMainCallback2(cb2_fade_and_do_reset) };
}

unsafe extern "C" fn main_cb() {
    unsafe { RunTasks() };
    unsafe { UpdatePaletteFade() };
}

unsafe extern "C" fn vblank_cb() {
    unsafe { TransferPlttBuffer() };
}

unsafe fn set_palette_color(index: usize, color: u16) {
    unsafe {
        (&raw mut gPlttBufferUnfaded)
            .cast::<u16>()
            .add(index)
            .write(color)
    };
    unsafe {
        (&raw mut gPlttBufferFaded)
            .cast::<u16>()
            .add(index)
            .write(color)
    };
}

unsafe fn setup_screen() -> bool {
    let state = unsafe { main_state() };
    if unsafe { state.read() } == 1 {
        unsafe { UpdatePaletteFade() };
        if !unsafe { palette_fade_active() } {
            unsafe { SetMainCallback2(main_cb) };
            return true;
        }
        return false;
    }

    unsafe { SetVBlankCallback(None) };
    unsafe { SetGpuReg(REG_OFFSET_DISPCNT, DISPCNT_MODE_0) };
    for reg in [
        REG_OFFSET_BG0HOFS,
        REG_OFFSET_BG0VOFS,
        REG_OFFSET_BG3HOFS,
        REG_OFFSET_BG3VOFS,
        REG_OFFSET_WIN0H,
        REG_OFFSET_WIN0V,
        REG_OFFSET_WININ,
        REG_OFFSET_WINOUT,
        REG_OFFSET_BLDCNT,
        REG_OFFSET_BLDALPHA,
        REG_OFFSET_BLDY,
    ] {
        unsafe { SetGpuReg(reg, 0) };
    }
    unsafe { dma3_fill_large(0, VRAM as *mut u8, VRAM_SIZE, false) };
    unsafe { dma3_fill_large(0, OAM as *mut u8, OAM_SIZE, true) };
    unsafe { dma3_fill_large(0, (PLTT + 2) as *mut u8, PLTT_SIZE as u32 - 2, false) };
    unsafe { ResetPaletteFade() };
    unsafe { set_palette_color(0, RGB_WHITE) };
    unsafe { set_palette_color(1, rgb(5, 10, 14)) };
    // Tile 1 is solid colour 1; the whole of screen 30 uses it.
    for i in 0..0x10 {
        unsafe { ((VRAM + 0x20) as *mut u16).add(i).write_volatile(0x1111) };
    }
    for i in 0..0x400 {
        unsafe { (BG_SCREEN_30 as *mut u16).add(i).write_volatile(0x0001) };
    }
    unsafe { ResetTasks() };
    unsafe { ResetSpriteData() };
    unsafe { ResetBgsAndClearDma3BusyFlags(0) };
    unsafe { InitBgsFromTemplates(0, BG_TEMPLATES.as_ptr().cast(), BG_TEMPLATES.len() as u8) };
    unsafe { SetGpuReg(REG_OFFSET_DISPCNT, DISPCNT_OBJ_ON | DISPCNT_OBJ_1D_MAP) };
    unsafe { ShowBg(0) };
    unsafe { ShowBg(3) };
    unsafe { SetGpuReg(REG_OFFSET_BLDCNT, 0) };
    unsafe { init_windows() };
    unsafe { BeginNormalPaletteFade(PALETTES_BG, 0, 0x10, 0, RGB_WHITEALPHA) };
    unsafe { EnableInterrupts(INTR_FLAG_VBLANK) };
    unsafe { SetVBlankCallback(Some(vblank_cb)) };
    unsafe { state.write(1) };
    false
}

unsafe extern "C" fn cb2_fade_and_do_reset() {
    let state = unsafe { main_state() };
    if unsafe { state.read() } == 1 {
        unsafe { UpdatePaletteFade() };
        if !unsafe { palette_fade_active() } {
            unsafe { FreeAllWindowBuffers() };
            unsafe { DoSoftReset() };
        }
    } else {
        unsafe { BeginNormalPaletteFade(PALETTES_BG, 0, 0, 0x10, RGB_WHITEALPHA) };
        unsafe { state.write(1) };
    }
}

unsafe fn init_windows() {
    unsafe { InitWindows(TEXT_WINDOW.as_ptr()) };
    unsafe { DeactivateAllTextPrinters() };
    unsafe { FillWindowPixelBuffer(0, 0) };
    unsafe { LoadWindowGfx(0, 0, 2, 14 * 16) };
    unsafe {
        LoadPalette(
            (&raw const gStandardMenuPalette).cast(),
            15 * 16,
            PLTT_SIZE_4BPP,
        )
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bg_templates_pack_like_gcc() {
        // bg 3, map base 30, priority 1
        assert_eq!(BG_TEMPLATES[1].0, 3 | (30 << 4) | (1 << 12));
    }
}
