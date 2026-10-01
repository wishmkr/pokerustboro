//! The wall-mounted region map in Pokémon Centers: no zoom, A or B closes it.
//! The fly map and the shared map code live in region_map.c.

use crate::bg::{InitBgsFromTemplates, ResetBgsAndClearDma3BusyFlags, ShowBg};
use crate::ffi::{
    AddTextPrinterParameterized, BgTemplate, COPYWIN_FULL_MODE, DUMMY_WIN_TEMPLATE,
    DrawStdFrameWithCustomTileAndPalette, FONT_NORMAL, MainCallback, WindowTemplate,
    palette_fade_active,
};
use crate::gpu_regs::{SetGpuReg, SetGpuRegBits};
use crate::international_string_util::GetStringCenterAlignXOffset;
use crate::malloc::{Alloc, Free};
use crate::sprite::{
    AnimateSprites, BuildOamBuffer, FreeAllSpritePalettes, LoadOam, ProcessSpriteCopyRequests,
    ResetSpriteData,
};
use crate::text_window::LoadUserWindowBorderGfx;
use crate::window::{CopyWindowToVram, FillWindowPixelBuffer, FreeAllWindowBuffers, InitWindows};

const WIN_MAPSEC_NAME: u8 = 0;
const WIN_TITLE: u8 = 1;
const TAG_PLAYER_ICON: u16 = 0;
const TAG_CURSOR: u16 = 1;

const MAPSECTYPE_NONE: u8 = 0;
const MAP_INPUT_MOVE_END: u8 = 3;
const MAP_INPUT_A_BUTTON: u8 = 4;
const MAP_INPUT_B_BUTTON: u8 = 5;
const PALETTES_ALL: u32 = 0xffff_ffff;
const RGB_BLACK: u16 = 0;

const REG_OFFSET_DISPCNT: u8 = 0x00;
const DISPCNT_OBJ_1D_MAP: u16 = 0x0040;
const DISPCNT_OBJ_ON: u16 = 0x1000;

/// The EWRAM handler: callback, unused word, `struct RegionMap`, state.
const H_SIZE: u32 = 0x890;
const H_CALLBACK: usize = 0x000;
const H_REGION_MAP: usize = 0x008;
const H_STATE: usize = 0x88c;
const RM_MAPSEC_TYPE: usize = 0x02;
const RM_MAPSEC_NAME: usize = 0x04;

#[unsafe(link_section = "ewram_data")]
static mut HANDLER: *mut u8 = core::ptr::null_mut();

static BG_TEMPLATES: [BgTemplate; 2] = [
    BgTemplate::new(0, 0, 31, 0, 0, 0, 0),
    BgTemplate::new(2, 2, 28, 2, 1, 2, 0),
];

static WINDOW_TEMPLATES: [WindowTemplate; 3] = [
    WindowTemplate {
        bg: 0,
        tilemap_left: 17,
        tilemap_top: 17,
        width: 12,
        height: 2,
        palette_num: 15,
        base_block: 1,
    },
    WindowTemplate {
        bg: 0,
        tilemap_left: 22,
        tilemap_top: 1,
        width: 7,
        height: 2,
        palette_num: 15,
        base_block: 25,
    },
    DUMMY_WIN_TEMPLATE,
];

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
/// `DeactivateAllTextPrinters` with this module's view of its types.
#[inline]
unsafe fn DeactivateAllTextPrinters() {
    unsafe {
        crate::text::DeactivateAllTextPrinters();
    }
}
/// `ClearScheduledBgCopiesToVram` with this module's view of its types.
#[inline]
unsafe fn ClearScheduledBgCopiesToVram() {
    unsafe {
        crate::menu::ClearScheduledBgCopiesToVram();
    }
}
/// `DoScheduledBgTilemapCopiesToVram` with this module's view of its types.
#[inline]
unsafe fn DoScheduledBgTilemapCopiesToVram() {
    unsafe {
        crate::menu::DoScheduledBgTilemapCopiesToVram();
    }
}
/// `ScheduleBgCopyTilemapToVram` with this module's view of its types.
#[inline]
unsafe fn ScheduleBgCopyTilemapToVram(a0: u8) {
    unsafe {
        crate::menu::ScheduleBgCopyTilemapToVram(a0);
    }
}
/// `InitRegionMap` with this module's view of its types.
#[inline]
unsafe fn InitRegionMap(a0: *mut u8, a1: u8) {
    unsafe {
        crate::region_map::InitRegionMap(a0 as _, a1);
    }
}
/// `CreateRegionMapPlayerIcon` with this module's view of its types.
#[inline]
unsafe fn CreateRegionMapPlayerIcon(a0: u16, a1: u16) {
    unsafe {
        crate::region_map::CreateRegionMapPlayerIcon(a0, a1);
    }
}
/// `CreateRegionMapCursor` with this module's view of its types.
#[inline]
unsafe fn CreateRegionMapCursor(a0: u16, a1: u16) {
    unsafe {
        crate::region_map::CreateRegionMapCursor(a0, a1);
    }
}
/// `DoRegionMapInputCallback` with this module's view of its types.
#[inline]
unsafe fn DoRegionMapInputCallback() -> u8 {
    unsafe { crate::region_map::DoRegionMapInputCallback() }
}
/// `FreeRegionMapIconResources` with this module's view of its types.
#[inline]
unsafe fn FreeRegionMapIconResources() {
    unsafe {
        crate::region_map::FreeRegionMapIconResources();
    }
}

#[inline]
unsafe fn handler() -> *mut u8 {
    unsafe { (&raw const HANDLER).read() }
}

#[inline]
unsafe fn state() -> *mut u16 {
    unsafe { handler().add(H_STATE).cast() }
}

unsafe fn advance() {
    let state = unsafe { state() };
    unsafe { state.write(state.read().wrapping_add(1)) };
}

#[unsafe(no_mangle)]
pub unsafe fn FieldInitRegionMap(callback: MainCallback) {
    unsafe { SetVBlankCallback(None) };
    let h = unsafe { Alloc(H_SIZE) };
    unsafe { (&raw mut HANDLER).write(h) };
    unsafe { state().write(0) };
    unsafe { h.add(H_CALLBACK).cast::<MainCallback>().write(callback) };
    unsafe { SetMainCallback2(init_region_map_registers) };
}

unsafe fn init_region_map_registers() {
    // DISPCNT, then BG0..BG3 HOFS/VOFS.
    unsafe { SetGpuReg(REG_OFFSET_DISPCNT, 0) };
    for reg in (0x10..0x20).step_by(2) {
        unsafe { SetGpuReg(reg, 0) };
    }
    unsafe { ResetSpriteData() };
    unsafe { FreeAllSpritePalettes() };
    unsafe { ResetBgsAndClearDma3BusyFlags(0) };
    unsafe { InitBgsFromTemplates(1, BG_TEMPLATES.as_ptr().cast(), BG_TEMPLATES.len() as u8) };
    unsafe { InitWindows(WINDOW_TEMPLATES.as_ptr()) };
    unsafe { DeactivateAllTextPrinters() };
    unsafe { LoadUserWindowBorderGfx(0, 0x27, 13 * 16) };
    unsafe { ClearScheduledBgCopiesToVram() };
    unsafe { SetMainCallback2(field_update_region_map_cb2) };
    unsafe { SetVBlankCallback(Some(vblank_cb)) };
}

unsafe fn vblank_cb() {
    unsafe { LoadOam() };
    unsafe { ProcessSpriteCopyRequests() };
    unsafe { TransferPlttBuffer() };
}

unsafe fn field_update_region_map_cb2() {
    unsafe { field_update_region_map() };
    unsafe { AnimateSprites() };
    unsafe { BuildOamBuffer() };
    unsafe { UpdatePaletteFade() };
    unsafe { DoScheduledBgTilemapCopiesToVram() };
}

unsafe fn field_update_region_map() {
    let h = unsafe { handler() };
    match unsafe { state().read() } {
        0 => {
            unsafe { InitRegionMap(h.add(H_REGION_MAP), 0) };
            unsafe { CreateRegionMapPlayerIcon(TAG_PLAYER_ICON, TAG_PLAYER_ICON) };
            unsafe { CreateRegionMapCursor(TAG_CURSOR, TAG_CURSOR) };
            unsafe { advance() };
        }
        1 => {
            unsafe { DrawStdFrameWithCustomTileAndPalette(WIN_TITLE, 0, 0x27, 0xd) };
            let hoenn = &raw const (*(&raw const crate::data::strings::gText_Hoenn).cast::<u8>());
            let offset =
                unsafe { GetStringCenterAlignXOffset(i32::from(FONT_NORMAL), hoenn, 0x38) };
            unsafe {
                AddTextPrinterParameterized(WIN_TITLE, FONT_NORMAL, hoenn, offset as u8, 1, 0, None)
            };
            unsafe { ScheduleBgCopyTilemapToVram(0) };
            unsafe { DrawStdFrameWithCustomTileAndPalette(WIN_MAPSEC_NAME, 0, 0x27, 0xd) };
            unsafe { print_region_map_sec_name() };
            unsafe { BeginNormalPaletteFade(PALETTES_ALL, 0, 16, 0, RGB_BLACK) };
            unsafe { advance() };
        }
        2 => {
            unsafe { SetGpuRegBits(REG_OFFSET_DISPCNT, DISPCNT_OBJ_1D_MAP | DISPCNT_OBJ_ON) };
            unsafe { ShowBg(0) };
            unsafe { ShowBg(2) };
            unsafe { advance() };
        }
        3 => {
            if !unsafe { palette_fade_active() } {
                unsafe { advance() };
            }
        }
        4 => match unsafe { DoRegionMapInputCallback() } {
            MAP_INPUT_MOVE_END => unsafe { print_region_map_sec_name() },
            MAP_INPUT_A_BUTTON | MAP_INPUT_B_BUTTON => unsafe { advance() },
            _ => {}
        },
        5 => {
            unsafe { BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 16, RGB_BLACK) };
            unsafe { advance() };
        }
        6 => {
            if !unsafe { palette_fade_active() } {
                unsafe { FreeRegionMapIconResources() };
                let callback = unsafe { h.add(H_CALLBACK).cast::<MainCallback>().read() };
                unsafe { SetMainCallback2(callback) };
                unsafe { Free(h) };
                unsafe { (&raw mut HANDLER).write(core::ptr::null_mut()) };
                unsafe { FreeAllWindowBuffers() };
            }
        }
        _ => {}
    }
}

unsafe fn print_region_map_sec_name() {
    let region_map = unsafe { handler().add(H_REGION_MAP) };
    unsafe { FillWindowPixelBuffer(WIN_MAPSEC_NAME, 0x11) };
    if unsafe { region_map.add(RM_MAPSEC_TYPE).read() } != MAPSECTYPE_NONE {
        unsafe {
            AddTextPrinterParameterized(
                WIN_MAPSEC_NAME,
                FONT_NORMAL,
                region_map.add(RM_MAPSEC_NAME),
                0,
                1,
                0,
                None,
            )
        };
        unsafe { ScheduleBgCopyTilemapToVram(WIN_MAPSEC_NAME) };
    } else {
        unsafe { CopyWindowToVram(WIN_MAPSEC_NAME, COPYWIN_FULL_MODE) };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn state_follows_the_region_map() {
        assert!(H_STATE >= H_REGION_MAP);
        assert_eq!(H_STATE + 4, H_SIZE as usize);
    }
}
