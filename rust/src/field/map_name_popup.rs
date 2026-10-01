//! The map name sign that slides down when entering a new map section.
//! Frames, palettes and the theme table are in `data/map_name_popup.rs`.

use crate::bg::{FillBgTilemapBufferRect, LoadBgTiles};
use crate::data::map_name_popup::{
    sBattlePyramid_MapHeaderStrings, sMapPopUp_OutlineTable, sMapPopUp_Palette_Underwater,
    sMapPopUp_PaletteTable, sMapPopUp_Table, sMapSectionToThemeId,
};
use crate::event_data::FlagGet;
use crate::ffi::{COPYWIN_FULL_MODE, CreateTask, DestroyTask, set_task_data, task_data};
use crate::gpu_regs::{SetGpuReg, SetGpuReg_ForcedBlank};
use crate::international_string_util::GetStringCenterAlignXOffset;
use crate::load_save::gSaveBlock2Ptr;
use crate::string_util::StringCopy;
use crate::task::FuncIsActiveTask;
use crate::text::AddTextPrinterParameterized;
use crate::window::{
    BlitBitmapToWindow, CallWindowFunction, CopyWindowToVram, GetWindowAttribute, PutWindowTilemap,
};

const FLAG_HIDE_MAP_NAME_POPUP: u16 = 0x4000;
const REG_OFFSET_BG0VOFS: u8 = 0x12;
const POPUP_OFFSCREEN_Y: i16 = 40;
const POPUP_SLIDE_SPEED: i16 = 2;
const FONT_NARROW: u8 = 7;
const TEXT_SKIP_DRAW: u8 = 0xff;
const EXT_CTRL_CODE_BEGIN: u8 = 0xfc;
const EXT_CTRL_CODE_HIGHLIGHT: u8 = 0x02;
const TEXT_COLOR_TRANSPARENT: u8 = 0;
const WINDOW_BG: u8 = 0;

const PYRAMID_LOCATION_NONE: u8 = 0;
const LAYOUT_BATTLE_FRONTIER_BATTLE_PYRAMID_TOP: u16 = 0x17a;
const FRONTIER_STAGES_PER_CHALLENGE: usize = 7;
const SB2_CUR_CHALLENGE_BATTLE_NUM: usize = 0xcb2;
const KANTO_MAPSEC_START: u16 = 0x58;
const KANTO_MAPSEC_END: u16 = 0xc4;
const KANTO_MAPSEC_COUNT: u16 = 0x6d;
const WEATHER_UNDERWATER_BUBBLES: u8 = 0x0e;

/// `gMapHeader` fields.
const MH_MAP_LAYOUT_ID: usize = 0x12;
const MH_REGION_MAP_SECTION_ID: usize = 0x14;
const MH_WEATHER: usize = 0x16;

const THEME_TILES_SIZE: usize = 960;
const THEME_PALETTE_SIZE: usize = 32;

// Task states. The first state is numerically last, as in the original.
const STATE_SLIDE_IN: i16 = 0;
const STATE_WAIT: i16 = 1;
const STATE_SLIDE_OUT: i16 = 2;
const STATE_ERASE: i16 = 4;
const STATE_END: i16 = 5;
const STATE_PRINT: i16 = 6;

const T_STATE: usize = 0;
const T_ONSCREEN_TIMER: usize = 1;
const T_Y_OFFSET: usize = 2;
const T_INCOMING_POPUP: usize = 3;
const T_PRINT_TIMER: usize = 4;

#[unsafe(link_section = "ewram_data")]
static POPUP_TASK_ID: crate::global::Global<u8> = crate::global::Global::new(0);

/// `ClearStdWindowAndFrame` with this module's view of its types.
#[inline]
unsafe fn ClearStdWindowAndFrame(a0: u8, a1: u8) {
    unsafe {
        crate::menu::ClearStdWindowAndFrame(a0, a1);
    }
}
/// `GetMapNamePopUpWindowId` with this module's view of its types.
#[inline]
unsafe fn GetMapNamePopUpWindowId() -> u8 {
    crate::menu::GetMapNamePopUpWindowId()
}
/// `AddMapNamePopUpWindow` with this module's view of its types.
#[inline]
unsafe fn AddMapNamePopUpWindow() -> u8 {
    unsafe { crate::menu::AddMapNamePopUpWindow() }
}
/// `RemoveMapNamePopUpWindow` with this module's view of its types.
#[inline]
unsafe fn RemoveMapNamePopUpWindow() {
    unsafe {
        crate::menu::RemoveMapNamePopUpWindow();
    }
}
/// `CurrentBattlePyramidLocation` with this module's view of its types.
#[inline]
unsafe fn CurrentBattlePyramidLocation() -> u8 {
    unsafe { crate::battle_pyramid::CurrentBattlePyramidLocation() }
}
/// `GetMapName` with this module's view of its types.
#[inline]
unsafe fn GetMapName(a0: *mut u8, a1: u16, a2: u16) -> *mut u8 {
    unsafe { crate::region_map::GetMapName(a0 as _, a1, a2) as *mut u8 }
}
/// `LoadPalette` with this module's view of its types.
#[inline]
unsafe fn LoadPalette(a0: *const core::ffi::c_void, a1: u16, a2: u16) {
    unsafe {
        crate::palette::LoadPalette(a0 as _, a1, a2);
    }
}

#[inline]
fn popup_task() -> u8 {
    unsafe { (POPUP_TASK_ID.as_ptr().cast_const()).read() }
}

#[inline]
unsafe fn map_header(offset: usize) -> *const u8 {
    unsafe { (&raw const (*(&raw const crate::fieldmap::gMapHeader).cast::<u8>())).add(offset) }
}

#[unsafe(no_mangle)]
pub unsafe fn ShowMapNamePopup() {
    if unsafe { FlagGet(FLAG_HIDE_MAP_NAME_POPUP) } == 1 {
        return;
    }
    if FuncIsActiveTask(task_map_name_popup_window) == 0 {
        let task_id = unsafe { CreateTask(task_map_name_popup_window, 90) };
        unsafe { (POPUP_TASK_ID.as_ptr()).write(task_id) };
        unsafe { SetGpuReg(REG_OFFSET_BG0VOFS, POPUP_OFFSCREEN_Y as u16) };
        unsafe { set_task_data(task_id, T_STATE, STATE_PRINT) };
        unsafe { set_task_data(task_id, T_Y_OFFSET, POPUP_OFFSCREEN_Y) };
    } else {
        // A pop up is already showing: hurry it off so the new one can come.
        let task_id = popup_task();
        if unsafe { task_data(task_id, T_STATE) } != STATE_SLIDE_OUT {
            unsafe { set_task_data(task_id, T_STATE, STATE_SLIDE_OUT) };
        }
        unsafe { set_task_data(task_id, T_INCOMING_POPUP, 1) };
    }
}

unsafe fn task_map_name_popup_window(task_id: u8) {
    let get = |i| unsafe { task_data(task_id, i) };
    let put = |i, v| unsafe { set_task_data(task_id, i, v) };

    match get(T_STATE) {
        STATE_PRINT => {
            // Wait, then create and print the pop up window.
            let timer = get(T_PRINT_TIMER).wrapping_add(1);
            put(T_PRINT_TIMER, timer);
            if timer > 30 {
                put(T_STATE, STATE_SLIDE_IN);
                put(T_PRINT_TIMER, 0);
                unsafe { show_map_name_popup_window() };
            }
        }
        STATE_SLIDE_IN => {
            let y = get(T_Y_OFFSET).wrapping_sub(POPUP_SLIDE_SPEED);
            put(T_Y_OFFSET, y);
            if y <= 0 {
                put(T_Y_OFFSET, 0);
                put(T_STATE, STATE_WAIT);
                unsafe { set_task_data(popup_task(), T_ONSCREEN_TIMER, 0) };
            }
        }
        STATE_WAIT => {
            let timer = get(T_ONSCREEN_TIMER).wrapping_add(1);
            put(T_ONSCREEN_TIMER, timer);
            if timer > 120 {
                put(T_ONSCREEN_TIMER, 0);
                put(T_STATE, STATE_SLIDE_OUT);
            }
        }
        STATE_SLIDE_OUT => {
            let y = get(T_Y_OFFSET).wrapping_add(POPUP_SLIDE_SPEED);
            put(T_Y_OFFSET, y);
            if y >= POPUP_OFFSCREEN_Y {
                put(T_Y_OFFSET, POPUP_OFFSCREEN_Y);
                if get(T_INCOMING_POPUP) != 0 {
                    put(T_STATE, STATE_PRINT);
                    put(T_PRINT_TIMER, 0);
                    put(T_INCOMING_POPUP, 0);
                } else {
                    put(T_STATE, STATE_ERASE);
                    return;
                }
            }
        }
        STATE_ERASE => {
            unsafe { ClearStdWindowAndFrame(GetMapNamePopUpWindowId(), 1) };
            put(T_STATE, STATE_END);
        }
        STATE_END => {
            unsafe { HideMapNamePopUpWindow() };
            return;
        }
        _ => {}
    }
    unsafe { SetGpuReg(REG_OFFSET_BG0VOFS, get(T_Y_OFFSET) as u16) };
}

#[unsafe(no_mangle)]
pub unsafe fn HideMapNamePopUpWindow() {
    if FuncIsActiveTask(task_map_name_popup_window) != 0 {
        unsafe { ClearStdWindowAndFrame(GetMapNamePopUpWindowId(), 1) };
        unsafe { RemoveMapNamePopUpWindow() };
        unsafe { SetGpuReg_ForcedBlank(REG_OFFSET_BG0VOFS, 0) };
        unsafe { DestroyTask(popup_task()) };
    }
}

unsafe fn show_map_name_popup_window() {
    let mut header = [0u8; 24];
    let without_prefix = unsafe { header.as_mut_ptr().add(3) };

    if unsafe { CurrentBattlePyramidLocation() } != PYRAMID_LOCATION_NONE {
        let layout = unsafe { map_header(MH_MAP_LAYOUT_ID).cast::<u16>().read() };
        let index = if layout == LAYOUT_BATTLE_FRONTIER_BATTLE_PYRAMID_TOP {
            FRONTIER_STAGES_PER_CHALLENGE
        } else {
            let sb2 = unsafe { (&raw const gSaveBlock2Ptr).read().cast::<u8>() };
            usize::from(unsafe { sb2.add(SB2_CUR_CHALLENGE_BATTLE_NUM).cast::<u16>().read() })
        };
        let source = unsafe { sBattlePyramid_MapHeaderStrings.as_ptr().add(index).read() }.0;
        unsafe { StringCopy(without_prefix, source) };
    } else {
        let section = u16::from(unsafe { map_header(MH_REGION_MAP_SECTION_ID).read() });
        unsafe { GetMapName(without_prefix, section, 0) };
    }

    unsafe { AddMapNamePopUpWindow() };
    unsafe { load_map_name_popup_window_bg() };
    let x = unsafe { GetStringCenterAlignXOffset(i32::from(FONT_NARROW), without_prefix, 80) };
    header[0] = EXT_CTRL_CODE_BEGIN;
    header[1] = EXT_CTRL_CODE_HIGHLIGHT;
    header[2] = TEXT_COLOR_TRANSPARENT;
    let window_id = unsafe { GetMapNamePopUpWindowId() };
    unsafe {
        AddTextPrinterParameterized(
            window_id,
            FONT_NARROW,
            header.as_ptr(),
            x as u8,
            3,
            TEXT_SKIP_DRAW,
            None,
        )
    };
    unsafe { CopyWindowToVram(window_id, COPYWIN_FULL_MODE) };
}

const TILE_TOP_EDGE_START: u16 = 0x21d;
const TILE_TOP_EDGE_END: u16 = 0x228;
const TILE_LEFT_EDGE_TOP: u16 = 0x229;
const TILE_RIGHT_EDGE_TOP: u16 = 0x22a;
const TILE_LEFT_EDGE_MID: u16 = 0x22b;
const TILE_RIGHT_EDGE_MID: u16 = 0x22c;
const TILE_LEFT_EDGE_BOT: u16 = 0x22d;
const TILE_RIGHT_EDGE_BOT: u16 = 0x22e;
const TILE_BOT_EDGE_START: u16 = 0x22f;
const TILE_BOT_EDGE_END: u16 = 0x23a;

unsafe fn draw_map_name_popup_frame(bg: u8, x: u8, y: u8, delta_x: u8, delta_y: u8, _unused: u8) {
    let tile =
        |tile: u16, tx: u8, ty: u8| unsafe { FillBgTilemapBufferRect(bg, tile, tx, ty, 1, 1, 14) };

    for i in 0..=(TILE_TOP_EDGE_END - TILE_TOP_EDGE_START) {
        tile(
            TILE_TOP_EDGE_START + i,
            (i as u8).wrapping_sub(1).wrapping_add(x),
            y.wrapping_sub(1),
        );
    }
    let left = x.wrapping_sub(1);
    let right = delta_x.wrapping_add(x);
    tile(TILE_LEFT_EDGE_TOP, left, y);
    tile(TILE_RIGHT_EDGE_TOP, right, y);
    tile(TILE_LEFT_EDGE_MID, left, y.wrapping_add(1));
    tile(TILE_RIGHT_EDGE_MID, right, y.wrapping_add(1));
    tile(TILE_LEFT_EDGE_BOT, left, y.wrapping_add(2));
    tile(TILE_RIGHT_EDGE_BOT, right, y.wrapping_add(2));
    for i in 0..=(TILE_BOT_EDGE_END - TILE_BOT_EDGE_START) {
        tile(
            TILE_BOT_EDGE_START + i,
            (i as u8).wrapping_sub(1).wrapping_add(x),
            y.wrapping_add(delta_y),
        );
    }
}

unsafe fn load_map_name_popup_window_bg() {
    let window_id = unsafe { GetMapNamePopUpWindowId() };
    let mut section = u16::from(unsafe { map_header(MH_REGION_MAP_SECTION_ID).read() });
    if section >= KANTO_MAPSEC_START {
        section = if section > KANTO_MAPSEC_END {
            section - KANTO_MAPSEC_COUNT
        } else {
            0
        };
    }
    let theme = usize::from(unsafe {
        sMapSectionToThemeId
            .as_ptr()
            .add(usize::from(section))
            .read()
    });

    let bg = unsafe { GetWindowAttribute(window_id, WINDOW_BG) } as u8;
    let outline = sMapPopUp_OutlineTable
        .as_ptr()
        .wrapping_add(theme * THEME_TILES_SIZE);
    unsafe { LoadBgTiles(bg, outline.cast(), 0x400, 0x21d) };
    unsafe { CallWindowFunction(window_id, draw_map_name_popup_frame) };
    unsafe { PutWindowTilemap(window_id) };
    let palette = if unsafe { map_header(MH_WEATHER).read() } == WEATHER_UNDERWATER_BUBBLES {
        sMapPopUp_Palette_Underwater.as_ptr()
    } else {
        sMapPopUp_PaletteTable
            .as_ptr()
            .wrapping_add(theme * THEME_PALETTE_SIZE)
    };
    unsafe { LoadPalette(palette.cast(), 14 * 16, THEME_PALETTE_SIZE as u16) };
    let tiles = sMapPopUp_Table
        .as_ptr()
        .wrapping_add(theme * THEME_TILES_SIZE);
    unsafe { BlitBitmapToWindow(window_id, tiles, 0, 0, 80, 24) };
}
