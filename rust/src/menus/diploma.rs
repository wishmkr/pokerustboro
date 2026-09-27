//! The Pokédex diploma handed out by the Game Freak designer in Lilycove.

use core::ffi::c_void;

use crate::bg::{
    CopyBgTilemapBufferToVram, InitBgsFromTemplates, ResetBgsAndClearDma3BusyFlags,
    SetBgTilemapBuffer, ShowBg,
};
use crate::ffi::{
    A_BUTTON, B_BUTTON, BgTemplate, COPYWIN_FULL_MODE, DUMMY_WIN_TEMPLATE, DestroyTask,
    FONT_NORMAL, MainCallback, PLTT_SIZE, PLTT_SIZE_4BPP, RomBytes, WindowTemplate,
    dma3_fill_large, gStringVar1, gStringVar4, joy_new, palette_fade_active, set_task_func,
};
use crate::gpu_regs::{EnableInterrupts, SetGpuReg};
use crate::malloc::{Alloc, Free};
use crate::sprite::{
    AnimateSprites, BuildOamBuffer, FreeAllSpritePalettes, LoadOam, ProcessSpriteCopyRequests,
    ResetSpriteData,
};
use crate::string_util::{StringCopy, StringExpandPlaceholders};
use crate::task::{CreateTask, ResetTasks, RunTasks};
use crate::window::{
    CopyWindowToVram, FillWindowPixelBuffer, FreeAllWindowBuffers, InitWindows, PutWindowTilemap,
};

const VRAM: usize = 0x0600_0000;
const VRAM_SIZE: u32 = 0x1_8000;
const OAM: usize = 0x0700_0000;
const OAM_SIZE: u32 = 0x400;
const PLTT: usize = 0x0500_0000;

const REG_OFFSET_DISPCNT: u8 = 0x00;
const REG_OFFSET_BG0CNT: u8 = 0x08;
const REG_OFFSET_BG1HOFS: u8 = 0x14;
const REG_OFFSET_BLDCNT: u8 = 0x50;
const REG_OFFSET_BLDALPHA: u8 = 0x52;
const REG_OFFSET_BLDY: u8 = 0x54;
const DISPCNT_MODE_0: u16 = 0;
const DISPCNT_OBJ_1D_MAP: u16 = 0x0040;
const DISPCNT_OBJ_ON: u16 = 0x1000;
const DISPLAY_WIDTH: u16 = 240;
const PALETTES_ALL: u32 = 0xffff_ffff;
const RGB_BLACK: u16 = 0;
const TEXT_SKIP_DRAW: i8 = -1;

#[unsafe(link_section = "ewram_data")]
static mut TILEMAP: *mut u8 = core::ptr::null_mut();

/// `sDiplomaPalettes[2][16]`: national, then Hoenn.
static DIPLOMA_PALETTES: RomBytes<64> = {
    const NATIONAL: &[u8; 32] =
        include_bytes!("../../../build/assets/graphics/diploma/national.pal.gbapal");
    const HOENN: &[u8; 32] =
        include_bytes!("../../../build/assets/graphics/diploma/hoenn.pal.gbapal");
    let mut bytes = [0u8; 64];
    let mut i = 0;
    while i < 32 {
        bytes[i] = NATIONAL[i];
        bytes[32 + i] = HOENN[i];
        i += 1;
    }
    RomBytes(bytes)
};

crate::incbin!(
    sDiplomaTilemap,
    "../../../build/assets/graphics/diploma/tilemap.bin.lz"
);
crate::incbin!(
    sDiplomaTiles,
    "../../../build/assets/graphics/diploma/tiles.png.4bpp.lz"
);

static BG_TEMPLATES: [BgTemplate; 2] = [
    BgTemplate::new(0, 1, 31, 0, 0, 0, 0),
    BgTemplate::new(1, 0, 6, 1, 0, 1, 0),
];

static WINDOW_TEMPLATES: [WindowTemplate; 2] = [
    WindowTemplate {
        bg: 0,
        tilemap_left: 5,
        tilemap_top: 2,
        width: 20,
        height: 16,
        palette_num: 15,
        base_block: 1,
    },
    DUMMY_WIN_TEMPLATE,
];

static TEXT_COLOR: [u8; 3] = [0, 2, 3];

unsafe extern "C" {
    static gText_DexNational: u8;
    static gText_DexHoenn: u8;
    static gText_PokedexDiploma: u8;
    static gStandardMenuPalette: u16;

    fn SetMainCallback2(callback: MainCallback);
    fn SetVBlankCallback(callback: Option<unsafe extern "C" fn()>);
    fn BeginNormalPaletteFade(
        selected_palettes: u32,
        delay: i8,
        start_y: u8,
        target_y: u8,
        blend_color: u16,
    ) -> u8;
    fn BlendPalettes(selected_palettes: u32, coeff: u8, color: u16);
    fn UpdatePaletteFade() -> u8;
    fn TransferPlttBuffer();
    fn ResetPaletteFade();
    fn LoadPalette(src: *const c_void, offset: u16, size: u16);
    fn ScanlineEffect_Stop();
    fn ResetTempTileDataBuffers();
    fn DecompressAndCopyTileDataToVram(
        bg: u8,
        src: *const c_void,
        size: u32,
        offset: u16,
        mode: u8,
    ) -> *mut c_void;
    fn FreeTempTileDataBuffersIfPossible() -> u8;
    fn HasAllMons() -> u16;
    fn DeactivateAllTextPrinters();
    fn AddTextPrinterParameterized4(
        window_id: u8,
        font_id: u8,
        left: u8,
        top: u8,
        letter_spacing: u8,
        line_spacing: u8,
        color: *const u8,
        speed: i8,
        string: *const u8,
    );
    fn CB2_ReturnToFieldFadeFromBlack();
}

unsafe extern "C" fn vblank_cb() {
    unsafe { LoadOam() };
    unsafe { ProcessSpriteCopyRequests() };
    unsafe { TransferPlttBuffer() };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn CB2_ShowDiploma() {
    unsafe { SetVBlankCallback(None) };
    unsafe { SetGpuReg(REG_OFFSET_DISPCNT, DISPCNT_MODE_0) };
    // BG3CNT..BG0CNT, then BG3..BG0 scroll, in the original's order.
    for reg in (REG_OFFSET_BG0CNT..REG_OFFSET_BG0CNT + 8).step_by(2).rev() {
        unsafe { SetGpuReg(reg, 0) };
    }
    for bg in (0..4u8).rev() {
        unsafe { SetGpuReg(0x10 + bg * 4, 0) };
        unsafe { SetGpuReg(0x12 + bg * 4, 0) };
    }
    unsafe { dma3_fill_large(0, VRAM as *mut u8, VRAM_SIZE, false) };
    unsafe { dma3_fill_large(0, OAM as *mut u8, OAM_SIZE, true) };
    unsafe { dma3_fill_large(0, PLTT as *mut u8, PLTT_SIZE as u32, false) };
    unsafe { ScanlineEffect_Stop() };
    unsafe { ResetTasks() };
    unsafe { ResetSpriteData() };
    unsafe { ResetPaletteFade() };
    unsafe { FreeAllSpritePalettes() };
    unsafe {
        LoadPalette(
            DIPLOMA_PALETTES.as_ptr().cast(),
            0,
            DIPLOMA_PALETTES.0.len() as u16,
        )
    };
    let tilemap = unsafe { Alloc(0x1000) };
    unsafe { (&raw mut TILEMAP).write(tilemap) };
    unsafe { init_bg(tilemap) };
    unsafe { init_window() };
    unsafe { ResetTempTileDataBuffers() };
    unsafe { DecompressAndCopyTileDataToVram(1, sDiplomaTiles.as_ptr().cast(), 0, 0, 0) };
    while unsafe { FreeTempTileDataBuffersIfPossible() } != 0 {}
    unsafe { crate::decompress::LZDecompressWram(sDiplomaTilemap.as_ptr().cast(), tilemap) };
    unsafe { CopyBgTilemapBufferToVram(1) };
    unsafe { display_text() };
    unsafe { BlendPalettes(PALETTES_ALL, 16, RGB_BLACK) };
    unsafe { BeginNormalPaletteFade(PALETTES_ALL, 0, 16, 0, RGB_BLACK) };
    unsafe { EnableInterrupts(1) };
    unsafe { SetVBlankCallback(Some(vblank_cb)) };
    unsafe { SetMainCallback2(main_cb2) };
    unsafe { CreateTask(task_fade_in, 0) };
}

unsafe extern "C" fn main_cb2() {
    unsafe { RunTasks() };
    unsafe { AnimateSprites() };
    unsafe { BuildOamBuffer() };
    unsafe { UpdatePaletteFade() };
}

unsafe extern "C" fn task_fade_in(task_id: u8) {
    if !unsafe { palette_fade_active() } {
        unsafe { set_task_func(task_id, task_wait_for_key_press) };
    }
}

unsafe extern "C" fn task_wait_for_key_press(task_id: u8) {
    if unsafe { joy_new(A_BUTTON | B_BUTTON) } {
        unsafe { BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 16, RGB_BLACK) };
        unsafe { set_task_func(task_id, task_fade_out) };
    }
}

unsafe extern "C" fn task_fade_out(task_id: u8) {
    if !unsafe { palette_fade_active() } {
        unsafe { Free((&raw const TILEMAP).read()) };
        unsafe { FreeAllWindowBuffers() };
        unsafe { DestroyTask(task_id) };
        unsafe { SetMainCallback2(CB2_ReturnToFieldFadeFromBlack) };
    }
}

unsafe fn display_text() {
    let var1 = (&raw mut gStringVar1).cast::<u8>();
    let var4 = (&raw mut gStringVar4).cast::<u8>();
    if unsafe { HasAllMons() } != 0 {
        unsafe { SetGpuReg(REG_OFFSET_BG1HOFS, DISPLAY_WIDTH + 16) };
        unsafe { StringCopy(var1, &raw const gText_DexNational) };
    } else {
        unsafe { SetGpuReg(REG_OFFSET_BG1HOFS, 0) };
        unsafe { StringCopy(var1, &raw const gText_DexHoenn) };
    }
    unsafe { StringExpandPlaceholders(var4, &raw const gText_PokedexDiploma) };
    unsafe {
        AddTextPrinterParameterized4(
            0,
            FONT_NORMAL,
            0,
            1,
            0,
            0,
            TEXT_COLOR.as_ptr(),
            TEXT_SKIP_DRAW,
            var4,
        )
    };
    unsafe { PutWindowTilemap(0) };
    unsafe { CopyWindowToVram(0, COPYWIN_FULL_MODE) };
}

unsafe fn init_bg(tilemap: *mut u8) {
    unsafe { ResetBgsAndClearDma3BusyFlags(0) };
    unsafe { InitBgsFromTemplates(0, BG_TEMPLATES.as_ptr().cast(), BG_TEMPLATES.len() as u8) };
    unsafe { SetBgTilemapBuffer(1, tilemap.cast()) };
    unsafe { SetGpuReg(REG_OFFSET_DISPCNT, DISPCNT_OBJ_ON | DISPCNT_OBJ_1D_MAP) };
    unsafe { ShowBg(0) };
    unsafe { ShowBg(1) };
    unsafe { SetGpuReg(REG_OFFSET_BLDCNT, 0) };
    unsafe { SetGpuReg(REG_OFFSET_BLDALPHA, 0) };
    unsafe { SetGpuReg(REG_OFFSET_BLDY, 0) };
}

unsafe fn init_window() {
    unsafe { InitWindows(WINDOW_TEMPLATES.as_ptr()) };
    unsafe { DeactivateAllTextPrinters() };
    unsafe {
        LoadPalette(
            (&raw const gStandardMenuPalette).cast(),
            15 * 16,
            PLTT_SIZE_4BPP,
        )
    };
    unsafe { FillWindowPixelBuffer(0, 0) };
    unsafe { PutWindowTilemap(0) };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bg1_uses_the_wide_screen() {
        assert_eq!(BG_TEMPLATES[1].0, 1 | (6 << 4) | (1 << 9) | (1 << 12));
    }
}
