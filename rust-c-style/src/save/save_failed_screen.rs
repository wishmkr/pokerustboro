//! Shown when writing the save fails: wipe the damaged flash sectors, try the
//! save again, and report whether the game can carry on. A little clock
//! spins while it works. Graphics are in `data/save_failed_screen.rs`.

use crate::bg::{
    InitBgsFromTemplates, LoadBgTiles, ResetBgsAndClearDma3BusyFlags, SetBgTilemapBuffer, ShowBg,
};
use crate::data::save_failed_screen::{sClockFrames, sSaveFailedClockGfx, sSaveFailedClockPal};
use crate::decompress::gDecompressionBuffer;
use crate::ffi::{
    A_BUTTON, BgTemplate, COPYWIN_GFX, COPYWIN_MAP, DUMMY_WIN_TEMPLATE,
    DrawStdFrameWithCustomTileAndPalette, FONT_NORMAL, MainCallback, OamData, PLTT_SIZE,
    ST_OAM_SQUARE, WindowTemplate, dma3_fill_large, gMain, joy_new, main_state,
};
use crate::gpu_regs::{EnableInterrupts, SetGpuReg};
use crate::sprite::{LoadOam, ProcessSpriteCopyRequests, ResetSpriteData};
use crate::task::ResetTasks;
use crate::text::DeactivateAllTextPrinters;
use crate::text_window::{gTextWindowFrame1_Gfx, gTextWindowFrame1_Pal};
use crate::window::{
    AddWindowWithoutTileMap, CopyWindowToVram, FillWindowPixelBuffer, InitWindows,
    SetWindowAttribute,
};

/// `sWindowIds[0]` is the message window, `[1]` the clock.
const TEXT_WIN_ID: usize = 0;

const CLOCK_RUNNING: usize = 0;
const DEBUG_TIMER: usize = 1;
const MSG_WIN_TOP: u16 = 12;
const CLOCK_WIN_TOP: u16 = MSG_WIN_TOP - 4;

const VRAM: usize = 0x0600_0000;
const VRAM_SIZE: u32 = 0x1_8000;
const OBJ_VRAM0: usize = 0x0601_0000;
const OAM: usize = 0x0700_0000;
const OAM_SIZE: u32 = 0x400;
const PLTT: usize = 0x0500_0000;
const PLTT_SIZE_4BPP: u16 = 0x20;
const SECTOR_SIZE: u32 = 0x1000;
const SECTORS_COUNT: u16 = 32;
const MAIN_VBLANK_COUNTER2: usize = 0x24;
const MAIN_OAM_BUFFER: usize = 0x38;
const PALETTES_ALL: u32 = 0xffff_ffff;
const RGB_BLACK: u16 = 0;
const DISPCNT_OBJ_ON_1D: u16 = 0x1040;
const WINDOW_TILE_DATA: u8 = 7;

#[unsafe(link_section = "ewram_data")]
static mut SAVE_FAILED_TYPE: u16 = 0;
#[unsafe(link_section = "ewram_data")]
static mut CLOCK_INFO: [u16; 2] = [0; 2];
#[unsafe(link_section = "ewram_data")]
static mut WINDOW_IDS: [u8; 2] = [0; 2];

static CLOCK_OAM: OamData = {
    let mut oam = OamData::simple(ST_OAM_SQUARE, 1, 0);
    oam.0[0] = 160; // y = DISPLAY_HEIGHT
    oam
};

static BG_TEMPLATES: [BgTemplate; 3] = [
    BgTemplate::new(0, 2, 31, 0, 0, 0, 0),
    BgTemplate::new(2, 0, 14, 0, 0, 2, 0),
    BgTemplate::new(3, 0, 15, 0, 0, 3, 0),
];

static DUMMY_WINDOW: [WindowTemplate; 1] = [DUMMY_WIN_TEMPLATE];
static TEXT_WINDOW: WindowTemplate = WindowTemplate {
    bg: 0,
    tilemap_left: 1,
    tilemap_top: 13,
    width: 28,
    height: 6,
    palette_num: 15,
    base_block: 1,
};
static CLOCK_WINDOW: WindowTemplate = WindowTemplate {
    bg: 0,
    tilemap_left: 14,
    tilemap_top: 9,
    width: 2,
    height: 2,
    palette_num: 15,
    base_block: 169,
};

/// Transparent background, dynamic colour 6, light grey shadow.
static TEXT_COLOR: [u8; 3] = [0, 15, 3];

unsafe extern "C" {
    static gText_SaveFailedCheckingBackup: u8;
    static gText_BackupMemoryDamaged: u8;
    static gText_CheckCompleted: u8;
    static gText_SaveCompleteGameCannotContinue: u8;
    static gText_SaveCompletePressA: u8;
    static gText_GamePlayCannotBeContinued: u8;
    static gBirchBagGrass_Gfx: u32;
    static gBirchBagTilemap: u32;
    static gBirchGrassTilemap: u32;
    static gBirchBagGrass_Pal: u16;
    static gStandardMenuPalette: u16;
    static gDamagedSaveSectors: u32;
    static mut gGameContinueCallback: Option<MainCallback>;
    static mut gSaveDataBuffer: u8;
    static ProgramFlashByte: Option<unsafe extern "C" fn(u16, u32, u8) -> u16>;

    fn SetMainCallback2(callback: MainCallback);
    fn SetVBlankCallback(callback: Option<unsafe extern "C" fn()>);
    fn TransferPlttBuffer();
    fn ResetPaletteFade();
    fn UpdatePaletteFade() -> u8;
    fn BeginNormalPaletteFade(
        selected: u32,
        delay: i8,
        start_y: u8,
        target_y: u8,
        color: u16,
    ) -> u8;
    fn LoadPalette(src: *const core::ffi::c_void, offset: u16, size: u16);
    fn LZ77UnCompVram(src: *const u32, dest: *mut core::ffi::c_void);
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
    fn HandleSavingData(save_type: u8) -> u8;
    fn ReadFlash(sector: u16, offset: u32, dest: *mut u8, size: u32);
    fn DoSoftReset();
    fn CpuFastSet(src: *const core::ffi::c_void, dest: *mut core::ffi::c_void, control: u32);
}

#[inline]
fn window_id(index: usize) -> u8 {
    unsafe { (&raw const WINDOW_IDS).cast::<u8>().add(index).read() }
}

#[inline]
unsafe fn set_clock_info(index: usize, value: u16) {
    unsafe { (&raw mut CLOCK_INFO).cast::<u16>().add(index).write(value) };
}

unsafe fn print_text(text: *const u8, x: u8, y: u8) {
    unsafe {
        AddTextPrinterParameterized4(
            window_id(TEXT_WIN_ID),
            FONT_NORMAL,
            x * 8,
            y * 8 + 1,
            0,
            0,
            TEXT_COLOR.as_ptr(),
            0,
            text,
        )
    };
}

unsafe fn clear_and_print(text: *const u8) {
    unsafe { FillWindowPixelBuffer(window_id(TEXT_WIN_ID), 0x11) };
    unsafe { print_text(text, 1, 0) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn DoSaveFailedScreen(save_type: u8) {
    unsafe { SetMainCallback2(cb2_save_failed_screen) };
    unsafe { (&raw mut SAVE_FAILED_TYPE).write(u16::from(save_type)) };
    unsafe { set_clock_info(CLOCK_RUNNING, 0) };
    unsafe { set_clock_info(DEBUG_TIMER, 0) };
    unsafe { (&raw mut WINDOW_IDS).write([0, 0]) };
}

unsafe extern "C" fn vblank_cb() {
    unsafe { LoadOam() };
    unsafe { ProcessSpriteCopyRequests() };
    unsafe { TransferPlttBuffer() };
}

unsafe extern "C" fn cb2_save_failed_screen() {
    let state = unsafe { main_state() };
    if unsafe { state.read() } == 1 {
        if unsafe { UpdatePaletteFade() } == 0 {
            unsafe { SetMainCallback2(cb2_wipe_save) };
            unsafe { SetVBlankCallback(Some(vblank_cb_update_clock_graphics)) };
        }
        return;
    }

    unsafe { SetVBlankCallback(None) };
    unsafe { SetGpuReg(0, 0) };
    // BG3CNT..BG0CNT, then BG3..BG0 scroll.
    for reg in [
        0x0e, 0x0c, 0x0a, 0x08, 0x1c, 0x1e, 0x18, 0x1a, 0x14, 0x16, 0x10, 0x12,
    ] {
        unsafe { SetGpuReg(reg, 0) };
    }
    unsafe { dma3_fill_large(0, VRAM as *mut u8, VRAM_SIZE, false) };
    unsafe { dma3_fill_large(0, OAM as *mut u8, OAM_SIZE, true) };
    unsafe { dma3_fill_large(0, PLTT as *mut u8, PLTT_SIZE as u32, false) };
    unsafe { LZ77UnCompVram(&raw const gBirchBagGrass_Gfx, VRAM as *mut _) };
    unsafe { LZ77UnCompVram(&raw const gBirchBagTilemap, (VRAM + 14 * 0x800) as *mut _) };
    unsafe { LZ77UnCompVram(&raw const gBirchGrassTilemap, (VRAM + 15 * 0x800) as *mut _) };
    unsafe {
        LZ77UnCompVram(
            sSaveFailedClockGfx.as_ptr().cast(),
            (OBJ_VRAM0 + 0x20) as *mut _,
        )
    };
    unsafe { ResetBgsAndClearDma3BusyFlags(0) };
    unsafe { InitBgsFromTemplates(0, BG_TEMPLATES.as_ptr().cast(), BG_TEMPLATES.len() as u8) };
    let buffer = (&raw mut gDecompressionBuffer).cast::<u8>();
    unsafe { SetBgTilemapBuffer(0, buffer.add(0x2000)) };
    unsafe { buffer.add(0x2000).write_bytes(0, 0x800) };
    unsafe { LoadBgTiles(0, gTextWindowFrame1_Gfx.as_ptr().cast(), 0x120, 0x214) };
    unsafe { InitWindows(DUMMY_WINDOW.as_ptr()) };
    let text_window = unsafe { AddWindowWithoutTileMap(&raw const TEXT_WINDOW) } as u8;
    unsafe {
        SetWindowAttribute(
            text_window,
            WINDOW_TILE_DATA,
            buffer.add(0x2800) as usize as u32,
        )
    };
    let clock_window = unsafe { AddWindowWithoutTileMap(&raw const CLOCK_WINDOW) } as u8;
    unsafe {
        SetWindowAttribute(
            clock_window,
            WINDOW_TILE_DATA,
            buffer.add(0x3d00) as usize as u32,
        )
    };
    unsafe { (&raw mut WINDOW_IDS).write([text_window, clock_window]) };
    unsafe { DeactivateAllTextPrinters() };
    unsafe { ResetSpriteData() };
    unsafe { ResetTasks() };
    unsafe { ResetPaletteFade() };
    unsafe {
        LoadPalette(
            (&raw const gBirchBagGrass_Pal).cast(),
            0,
            2 * PLTT_SIZE_4BPP,
        )
    };
    unsafe { LoadPalette(sSaveFailedClockPal.as_ptr().cast(), 0x100, PLTT_SIZE_4BPP) };
    unsafe {
        LoadPalette(
            gTextWindowFrame1_Pal.as_ptr().cast(),
            14 * 16,
            PLTT_SIZE_4BPP,
        )
    };
    unsafe {
        LoadPalette(
            (&raw const gStandardMenuPalette).cast(),
            15 * 16,
            PLTT_SIZE_4BPP,
        )
    };
    unsafe { DrawStdFrameWithCustomTileAndPalette(text_window, 0, 0x214, 0xe) };
    unsafe { DrawStdFrameWithCustomTileAndPalette(clock_window, 0, 0x214, 0xe) };
    unsafe { FillWindowPixelBuffer(clock_window, 0x11) };
    unsafe { FillWindowPixelBuffer(text_window, 0x11) };
    unsafe { CopyWindowToVram(clock_window, COPYWIN_GFX) };
    unsafe { CopyWindowToVram(text_window, COPYWIN_MAP) };
    unsafe { print_text(&raw const gText_SaveFailedCheckingBackup, 1, 0) };
    unsafe { BeginNormalPaletteFade(PALETTES_ALL, 0, 16, 0, RGB_BLACK) };
    unsafe { EnableInterrupts(1) };
    unsafe { SetVBlankCallback(Some(vblank_cb)) };
    unsafe { SetGpuReg(0, DISPCNT_OBJ_ON_1D) };
    unsafe { ShowBg(0) };
    unsafe { ShowBg(2) };
    unsafe { ShowBg(3) };
    unsafe { state.write(state.read().wrapping_add(1)) };
}

unsafe extern "C" fn cb2_wipe_save() {
    let mut wipe_tries = 0u8;
    unsafe { set_clock_info(CLOCK_RUNNING, 1) };
    let damaged = || unsafe { (&raw const gDamagedSaveSectors).read_volatile() };
    while damaged() != 0 && wipe_tries < 3 {
        if unsafe { wipe_sectors(damaged()) } {
            unsafe { clear_and_print(&raw const gText_BackupMemoryDamaged) };
            unsafe { SetMainCallback2(cb2_gameplay_cannot_be_continued) };
            return;
        }
        unsafe { clear_and_print(&raw const gText_CheckCompleted) };
        unsafe { HandleSavingData((&raw const SAVE_FAILED_TYPE).read() as u8) };
        if damaged() != 0 {
            unsafe { clear_and_print(&raw const gText_SaveFailedCheckingBackup) };
        }
        wipe_tries += 1;
    }

    if wipe_tries == 3 {
        unsafe { clear_and_print(&raw const gText_BackupMemoryDamaged) };
    } else if unsafe { (&raw const gGameContinueCallback).read() }.is_none() {
        unsafe { clear_and_print(&raw const gText_SaveCompleteGameCannotContinue) };
    } else {
        unsafe { clear_and_print(&raw const gText_SaveCompletePressA) };
    }
    unsafe { SetMainCallback2(cb2_fade_and_return_to_title_screen) };
}

unsafe extern "C" fn cb2_gameplay_cannot_be_continued() {
    unsafe { set_clock_info(CLOCK_RUNNING, 0) };
    if unsafe { joy_new(A_BUTTON) } {
        unsafe { clear_and_print(&raw const gText_GamePlayCannotBeContinued) };
        unsafe { SetVBlankCallback(Some(vblank_cb)) };
        unsafe { SetMainCallback2(cb2_fade_and_return_to_title_screen) };
    }
}

unsafe extern "C" fn cb2_fade_and_return_to_title_screen() {
    unsafe { set_clock_info(CLOCK_RUNNING, 0) };
    if unsafe { joy_new(A_BUTTON) } {
        unsafe { BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 16, RGB_BLACK) };
        unsafe { SetVBlankCallback(Some(vblank_cb)) };
        unsafe { SetMainCallback2(cb2_return_to_title_screen) };
    }
}

unsafe extern "C" fn cb2_return_to_title_screen() {
    if unsafe { UpdatePaletteFade() } != 0 {
        return;
    }
    match unsafe { (&raw const gGameContinueCallback).read() } {
        None => unsafe { DoSoftReset() },
        Some(callback) => {
            unsafe { SetMainCallback2(callback) };
            unsafe { (&raw mut gGameContinueCallback).write(None) };
        }
    }
}

unsafe extern "C" fn vblank_cb_update_clock_graphics() {
    let main = &raw mut gMain;
    let counter = unsafe { main.add(MAIN_VBLANK_COUNTER2).cast::<u32>().read() };
    let n = ((counter >> 3) & 7) as usize;
    let oam = unsafe { main.add(MAIN_OAM_BUFFER) };
    unsafe { core::ptr::copy_nonoverlapping(CLOCK_OAM.0.as_ptr(), oam, 8) };
    unsafe { crate::ffi::set_oam_x(oam, 112) };
    unsafe { oam.write(((CLOCK_WIN_TOP + 1) * 8) as u8) };
    let running = unsafe { (&raw const CLOCK_INFO).cast::<u16>().read() } != 0;
    let frame = unsafe { sClockFrames.as_ptr().add(n * 3) };
    let tile = if running { unsafe { frame.read() } } else { 1 };
    unsafe { crate::ffi::set_oam_tile_num(oam, u16::from(tile)) };
    if running {
        let h_flip = unsafe { frame.add(1).read() };
        let v_flip = unsafe { frame.add(2).read() };
        let matrix = (v_flip << 4) | (h_flip << 3);
        let attr1 = unsafe { oam.add(2).cast::<u16>().read() };
        let attr1 = (attr1 & !(0x1f << 9)) | (u16::from(matrix & 0x1f) << 9);
        unsafe { oam.add(2).cast::<u16>().write(attr1) };
    }
    // CpuFastCopy(buffer, OAM, 4): a count of one word, which the BIOS rounds
    // up to a block of eight, so the first four OAM entries are copied.
    unsafe { CpuFastSet(oam.cast(), OAM as *mut _, 4 / 4) };
    let timer = unsafe {
        (&raw const CLOCK_INFO)
            .cast::<u16>()
            .add(DEBUG_TIMER)
            .read()
    };
    if timer != 0 {
        unsafe { set_clock_info(DEBUG_TIMER, timer - 1) };
    }
}

unsafe fn verify_sector_wipe(sector: u16) -> bool {
    let buffer = &raw mut gSaveDataBuffer;
    unsafe { ReadFlash(sector, 0, buffer, SECTOR_SIZE) };
    let words = buffer.cast::<u32>();
    (0..(SECTOR_SIZE / 4) as usize).any(|i| unsafe { words.add(i).read() } != 0)
}

unsafe fn wipe_sector(sector: u16) -> bool {
    let mut failed = true;
    let mut tries = 0u16;
    // An arbitrary limit of 130 attempts.
    while failed && tries < 130 {
        if let Some(program) = unsafe { (&raw const ProgramFlashByte).read() } {
            for offset in 0..SECTOR_SIZE {
                unsafe { program(sector, offset, 0) };
            }
        }
        failed = unsafe { verify_sector_wipe(sector) };
        tries += 1;
    }
    failed
}

unsafe fn wipe_sectors(mut sector_bits: u32) -> bool {
    for i in 0..SECTORS_COUNT {
        if sector_bits & (1 << i) != 0 && !unsafe { wipe_sector(i) } {
            sector_bits &= !(1 << i);
        }
    }
    sector_bits != 0
}
