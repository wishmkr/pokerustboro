//! Text window frames: the 20 selectable border styles, the message box, and
//! the helpers that draw a border around a window on its background.

use core::ffi::c_void;

use crate::bg::{FillBgTilemapBufferRect, LoadBgTiles};
use crate::ffi::{RomBytes, RomPtr};
use crate::load_save::gSaveBlock2Ptr;
use crate::window::GetWindowAttribute;

const WINDOW_FRAMES_COUNT: usize = 20;
const PLTT_SIZE_4BPP: u16 = 0x20;

const WINDOW_BG: u8 = 0;
const WINDOW_TILEMAP_LEFT: u8 = 1;
const WINDOW_TILEMAP_TOP: u8 = 2;
const WINDOW_WIDTH: u8 = 3;
const WINDOW_HEIGHT: u8 = 4;

/// `gSaveBlock2Ptr->optionsWindowFrameType`: bits 3..8 of the u16 at 0x14.
const SB2_OPTIONS: usize = 0x14;

/// `LoadPalette` with this module's view of its types.
#[inline]
unsafe fn LoadPalette(a0: *const c_void, a1: u16, a2: u16) {
    unsafe {
        crate::palette::LoadPalette(a0 as _, a1, a2);
    }
}

/// `struct TilesPal { const u8 *tiles; const u16 *pal; }`
#[repr(C)]
pub struct TilesPal {
    tiles: RomPtr<u8>,
    pal: RomPtr<u8>,
}

macro_rules! frames {
    ($(($gfx:ident, $pal:ident, $n:literal)),* $(,)?) => {
        $(
            crate::incbin!(
                $gfx,
                concat!("../../../build/assets/graphics/text_window/", $n, ".png.4bpp")
            );
            crate::incbin!(
                $pal,
                concat!("../../../build/assets/graphics/text_window/", $n, ".png.gbapal")
            );
        )*

        static WINDOW_FRAMES: [TilesPal; WINDOW_FRAMES_COUNT] = [
            $(TilesPal { tiles: RomPtr($gfx.as_ptr()), pal: RomPtr($pal.as_ptr()) }),*
        ];
    };
}

frames!(
    (gTextWindowFrame1_Gfx, gTextWindowFrame1_Pal, "1"),
    (sTextWindowFrame2_Gfx, sTextWindowFrame2_Pal, "2"),
    (sTextWindowFrame3_Gfx, sTextWindowFrame3_Pal, "3"),
    (sTextWindowFrame4_Gfx, sTextWindowFrame4_Pal, "4"),
    (sTextWindowFrame5_Gfx, sTextWindowFrame5_Pal, "5"),
    (sTextWindowFrame6_Gfx, sTextWindowFrame6_Pal, "6"),
    (sTextWindowFrame7_Gfx, sTextWindowFrame7_Pal, "7"),
    (sTextWindowFrame8_Gfx, sTextWindowFrame8_Pal, "8"),
    (sTextWindowFrame9_Gfx, sTextWindowFrame9_Pal, "9"),
    (sTextWindowFrame10_Gfx, sTextWindowFrame10_Pal, "10"),
    (sTextWindowFrame11_Gfx, sTextWindowFrame11_Pal, "11"),
    (sTextWindowFrame12_Gfx, sTextWindowFrame12_Pal, "12"),
    (sTextWindowFrame13_Gfx, sTextWindowFrame13_Pal, "13"),
    (sTextWindowFrame14_Gfx, sTextWindowFrame14_Pal, "14"),
    (sTextWindowFrame15_Gfx, sTextWindowFrame15_Pal, "15"),
    (sTextWindowFrame16_Gfx, sTextWindowFrame16_Pal, "16"),
    (sTextWindowFrame17_Gfx, sTextWindowFrame17_Pal, "17"),
    (sTextWindowFrame18_Gfx, sTextWindowFrame18_Pal, "18"),
    (sTextWindowFrame19_Gfx, sTextWindowFrame19_Pal, "19"),
    (sTextWindowFrame20_Gfx, sTextWindowFrame20_Pal, "20"),
);

/// `sTextWindowPalettes[5][16]`, one 16-colour palette after another.
static TEXT_WINDOW_PALETTES: RomBytes<{ 5 * 32 }> = {
    const PALETTES: [&[u8; 32]; 5] = [
        include_bytes!("../../../build/assets/graphics/text_window/message_box.png.gbapal"),
        include_bytes!("../../../build/assets/graphics/text_window/text_pal1.pal.gbapal"),
        include_bytes!("../../../build/assets/graphics/text_window/text_pal2.pal.gbapal"),
        include_bytes!("../../../build/assets/graphics/text_window/text_pal3.pal.gbapal"),
        include_bytes!("../../../build/assets/graphics/text_window/text_pal4.pal.gbapal"),
    ];
    let mut bytes = [0u8; 5 * 32];
    let mut i = 0;
    while i < bytes.len() {
        bytes[i] = PALETTES[i / 32][i % 32];
        i += 1;
    }
    RomBytes(bytes)
};

#[inline]
fn frame(id: usize) -> &'static TilesPal {
    &WINDOW_FRAMES[id % WINDOW_FRAMES_COUNT]
}

unsafe fn user_frame_type() -> usize {
    let sb2 = unsafe { (&raw const gSaveBlock2Ptr).read().cast::<u8>() };
    let options = unsafe { sb2.add(SB2_OPTIONS).cast::<u16>().read() };
    usize::from((options >> 3) & 0x1f)
}

#[unsafe(no_mangle)]
pub unsafe fn GetWindowFrameTilesPal(id: u8) -> *const TilesPal {
    let id = usize::from(id);
    if id >= WINDOW_FRAMES_COUNT {
        frame(0)
    } else {
        frame(id)
    }
}

#[unsafe(no_mangle)]
pub unsafe fn LoadMessageBoxGfx(window_id: u8, dest_offset: u16, pal_offset: u8) {
    let bg = unsafe { GetWindowAttribute(window_id, WINDOW_BG) } as u8;
    unsafe {
        LoadBgTiles(
            bg,
            (&raw const (*(&raw const crate::data::graphics::gMessageBox_Gfx).cast::<u8>())).cast(),
            0x1c0,
            dest_offset,
        )
    };
    unsafe {
        LoadPalette(
            GetOverworldTextboxPalettePtr().cast(),
            u16::from(pal_offset),
            PLTT_SIZE_4BPP,
        )
    };
}

#[unsafe(no_mangle)]
pub unsafe fn LoadUserWindowBorderGfx_(window_id: u8, dest_offset: u16, pal_offset: u8) {
    unsafe { LoadUserWindowBorderGfx(window_id, dest_offset, pal_offset) };
}

#[unsafe(no_mangle)]
pub unsafe fn LoadWindowGfx(window_id: u8, frame_id: u8, dest_offset: u16, pal_offset: u8) {
    // The original indexes without a bounds check; ids come from the options
    // menu, which never exceeds the table.
    let frame = frame(usize::from(frame_id));
    let bg = unsafe { GetWindowAttribute(window_id, WINDOW_BG) } as u8;
    unsafe { LoadBgTiles(bg, frame.tiles.0.cast(), 0x120, dest_offset) };
    unsafe { LoadPalette(frame.pal.0.cast(), u16::from(pal_offset), PLTT_SIZE_4BPP) };
}

#[unsafe(no_mangle)]
pub unsafe fn LoadUserWindowBorderGfx(window_id: u8, dest_offset: u16, pal_offset: u8) {
    let frame_type = unsafe { user_frame_type() } as u8;
    unsafe { LoadWindowGfx(window_id, frame_type, dest_offset, pal_offset) };
}

struct Rect {
    bg: u8,
    left: u16,
    top: u16,
    width: u16,
    height: u16,
}

unsafe fn window_rect(window_id: u8) -> Rect {
    unsafe {
        Rect {
            bg: GetWindowAttribute(window_id, WINDOW_BG) as u8,
            left: GetWindowAttribute(window_id, WINDOW_TILEMAP_LEFT) as u16,
            top: GetWindowAttribute(window_id, WINDOW_TILEMAP_TOP) as u16,
            width: GetWindowAttribute(window_id, WINDOW_WIDTH) as u16,
            height: GetWindowAttribute(window_id, WINDOW_HEIGHT) as u16,
        }
    }
}

/// Draws the nine border pieces (the centre, tile 4, is skipped). `inset`
/// selects whether the border sits outside the window or on its edge.
unsafe fn draw_border(window_id: u8, tile_num: u16, pal_num: u8, inset: bool) {
    let r = unsafe { window_rect(window_id) };
    let (x0, y0, inner_w, inner_h) = if inset {
        (
            r.left,
            r.top,
            r.width.wrapping_sub(2),
            r.height.wrapping_sub(2),
        )
    } else {
        (
            r.left.wrapping_sub(1),
            r.top.wrapping_sub(1),
            r.width,
            r.height,
        )
    };
    let x1 = x0.wrapping_add(1);
    let x2 = x1.wrapping_add(inner_w);
    let y1 = y0.wrapping_add(1);
    let y2 = y1.wrapping_add(inner_h);
    let pieces: [(u16, u16, u16, u16, u16); 8] = [
        (0, x0, y0, 1, 1),
        (1, x1, y0, inner_w, 1),
        (2, x2, y0, 1, 1),
        (3, x0, y1, 1, inner_h),
        (5, x2, y1, 1, inner_h),
        (6, x0, y2, 1, 1),
        (7, x1, y2, inner_w, 1),
        (8, x2, y2, 1, 1),
    ];
    for (tile, x, y, w, h) in pieces {
        unsafe {
            FillBgTilemapBufferRect(
                r.bg,
                tile_num.wrapping_add(tile),
                x as u8,
                y as u8,
                w as u8,
                h as u8,
                pal_num,
            )
        };
    }
}

#[unsafe(no_mangle)]
pub unsafe fn DrawTextBorderOuter(window_id: u8, tile_num: u16, pal_num: u8) {
    unsafe { draw_border(window_id, tile_num, pal_num, false) };
}

#[unsafe(no_mangle)]
pub unsafe fn DrawTextBorderInner(window_id: u8, tile_num: u16, pal_num: u8) {
    unsafe { draw_border(window_id, tile_num, pal_num, true) };
}

#[unsafe(no_mangle)]
pub unsafe fn rbox_fill_rectangle(window_id: u8) {
    let r = unsafe { window_rect(window_id) };
    unsafe {
        FillBgTilemapBufferRect(
            r.bg,
            0,
            r.left.wrapping_sub(1) as u8,
            r.top.wrapping_sub(1) as u8,
            r.width.wrapping_add(2) as u8,
            r.height.wrapping_add(2) as u8,
            0x11,
        )
    };
}

#[unsafe(no_mangle)]
pub unsafe fn GetTextWindowPalette(id: u8) -> *const u16 {
    let index = if id < 4 { usize::from(id) } else { 4 };
    unsafe { TEXT_WINDOW_PALETTES.as_ptr().add(index * 32).cast() }
}

#[unsafe(no_mangle)]
pub unsafe fn GetOverworldTextboxPalettePtr() -> *const u16 {
    &raw const (*(&raw const crate::data::graphics::gMessageBox_Pal).cast::<u16>())
}

/// `LoadUserWindowBorderGfx` for a background instead of a window on it.
#[unsafe(no_mangle)]
pub unsafe fn LoadUserWindowBorderGfxOnBg(bg: u8, dest_offset: u16, pal_offset: u8) {
    let frame_type = unsafe { user_frame_type() };
    unsafe { LoadBgTiles(bg, frame(frame_type).tiles.0.cast(), 0x120, dest_offset) };
    let tiles_pal = unsafe { GetWindowFrameTilesPal(frame_type as u8) };
    let pal = unsafe { (*tiles_pal).pal.0 };
    unsafe { LoadPalette(pal.cast(), u16::from(pal_offset), PLTT_SIZE_4BPP) };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tiles_pal_is_two_pointers() {
        assert_eq!(
            core::mem::size_of::<TilesPal>(),
            2 * core::mem::size_of::<usize>()
        );
    }

    #[test]
    fn out_of_range_frames_fall_back_to_the_first() {
        let first = unsafe { GetWindowFrameTilesPal(0) };
        assert_eq!(unsafe { GetWindowFrameTilesPal(20) }, first);
        assert_eq!(unsafe { GetWindowFrameTilesPal(255) }, first);
        assert_ne!(unsafe { GetWindowFrameTilesPal(19) }, first);
    }

    #[test]
    fn text_palettes_are_sixteen_colours_apart() {
        let base = TEXT_WINDOW_PALETTES.as_ptr() as usize;
        for (id, index) in [(0u8, 0usize), (1, 1), (3, 3), (4, 4), (9, 4)] {
            let pal = unsafe { GetTextWindowPalette(id) } as usize;
            assert_eq!(pal - base, index * 32);
        }
    }
}
