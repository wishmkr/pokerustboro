//! The "Clear all save data?" screen reached with Up + Select + B on boot.

use crate::bg::{InitBgsFromTemplates, ResetBgsAndClearDma3BusyFlags, ShowBg};
use crate::ffi::{
    AddTextPrinterParameterized, BgTemplate, DUMMY_WIN_TEMPLATE, DestroyTask,
    DrawStdFrameWithCustomTileAndPalette, FONT_NORMAL, MAIN_STATE_OFFSET, MENU_B_PRESSED,
    MainCallback, PALETTES_BG, PLTT_SIZE, PLTT_SIZE_4BPP, PlaySE, RGB_WHITE, RGB_WHITEALPHA,
    SE_SELECT, WindowTemplate, dma3_fill_large, palette_fade_active, rgb, set_task_func,
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

/// `SetMainCallback2` with this module's view of its types.
#[inline]
unsafe fn SetMainCallback2(a0: MainCallback) {
    unsafe {
        crate::agb_main::SetMainCallback2(core::mem::transmute(a0));
    }
}
/// `SetVBlankCallback` with this module's view of its types.
#[inline]
unsafe fn SetVBlankCallback(a0: Option<unsafe fn()>) {
    unsafe {
        crate::agb_main::SetVBlankCallback(a0);
    }
}
/// `BeginNormalPaletteFade` with this module's view of its types.
#[inline]
unsafe fn BeginNormalPaletteFade(a0: u32, a1: i8, a2: u8, a3: u8, a4: u16) -> u8 {
    unsafe { crate::palette::BeginNormalPaletteFade(a0, a1, a2, a3, a4) }
}
/// `UpdatePaletteFade` with this module's view of its types.
#[inline]
unsafe fn UpdatePaletteFade() -> u8 {
    unsafe { crate::palette::UpdatePaletteFade() }
}
/// `TransferPlttBuffer` with this module's view of its types.
#[inline]
unsafe fn TransferPlttBuffer() {
    unsafe {
        crate::palette::TransferPlttBuffer();
    }
}
/// `ResetPaletteFade` with this module's view of its types.
#[inline]
unsafe fn ResetPaletteFade() {
    unsafe {
        crate::palette::ResetPaletteFade();
    }
}
/// `LoadPalette` with this module's view of its types.
#[inline]
unsafe fn LoadPalette(a0: *const core::ffi::c_void, a1: u16, a2: u16) {
    unsafe {
        crate::palette::LoadPalette(a0 as _, a1, a2);
    }
}
/// `DoSoftReset` with this module's view of its types.
#[inline]
unsafe fn DoSoftReset() {
    unsafe {
        crate::agb_main::DoSoftReset();
    }
}
/// `CreateYesNoMenu` with this module's view of its types.
#[inline]
unsafe fn CreateYesNoMenu(a0: *const WindowTemplate, a1: u16, a2: u8, a3: u8) {
    unsafe {
        crate::menu::CreateYesNoMenu(a0 as _, a1, a2, a3);
    }
}
/// `Menu_ProcessInputNoWrapClearOnChoose` with this module's view of its types.
#[inline]
unsafe fn Menu_ProcessInputNoWrapClearOnChoose() -> i8 {
    unsafe { crate::menu::Menu_ProcessInputNoWrapClearOnChoose() }
}
/// `DeactivateAllTextPrinters` with this module's view of its types.
#[inline]
unsafe fn DeactivateAllTextPrinters() {
    unsafe {
        crate::text::DeactivateAllTextPrinters();
    }
}
/// `ClearSaveData` with this module's view of its types.
#[inline]
unsafe fn ClearSaveData() {
    unsafe {
        crate::save::ClearSaveData();
    }
}

#[inline]
unsafe fn main_state() -> *mut u8 {
    unsafe {
        (&raw mut (*(&raw const crate::agb_main::gMain).cast::<u8>().cast_mut()))
            .cast::<u8>()
            .add(MAIN_STATE_OFFSET)
    }
}

#[unsafe(no_mangle)]
pub unsafe fn CB2_InitClearSaveDataScreen() {
    if unsafe { setup_screen() } {
        CreateTask(task_do_yes_no, 0);
    }
}

unsafe fn task_do_yes_no(task_id: u8) {
    unsafe { DrawStdFrameWithCustomTileAndPalette(0, 0, 2, 14) };
    unsafe {
        AddTextPrinterParameterized(
            0,
            FONT_NORMAL,
            &raw const (*(&raw const crate::data::strings::gText_ClearAllSaveData).cast::<u8>()),
            0,
            1,
            0,
            None,
        )
    };
    unsafe { CreateYesNoMenu(&raw const YES_NO, 2, 14, 1) };
    unsafe { set_task_func(task_id, task_yes_no_choice) };
}

unsafe fn task_yes_no_choice(task_id: u8) {
    match unsafe { Menu_ProcessInputNoWrapClearOnChoose() } {
        0 => {
            unsafe { FillWindowPixelBuffer(0, 0x11) };
            unsafe {
                AddTextPrinterParameterized(
                    0,
                    FONT_NORMAL,
                    &raw const (*(&raw const crate::data::strings::gText_ClearingData)
                        .cast::<u8>()),
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

unsafe fn task_clear_save_data(task_id: u8) {
    unsafe { ClearSaveData() };
    unsafe { DestroyTask(task_id) };
    unsafe { SetMainCallback2(cb2_fade_and_do_reset) };
}

unsafe fn main_cb() {
    RunTasks();
    unsafe { UpdatePaletteFade() };
}

unsafe fn vblank_cb() {
    unsafe { TransferPlttBuffer() };
}

unsafe fn set_palette_color(index: usize, color: u16) {
    unsafe {
        (&raw mut (*(&raw const crate::palette::gPlttBufferUnfaded)
            .cast::<[u16; crate::ffi::PLTT_BUFFER_SIZE]>()
            .cast_mut()))
            .cast::<u16>()
            .add(index)
            .write(color)
    };
    unsafe {
        (&raw mut (*(&raw const crate::palette::gPlttBufferFaded)
            .cast::<[u16; crate::ffi::PLTT_BUFFER_SIZE]>()
            .cast_mut()))
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
    ResetTasks();
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

unsafe fn cb2_fade_and_do_reset() {
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
            (&raw const (*(&raw const crate::data::menu::gStandardMenuPalette).cast::<u16>()))
                .cast(),
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
