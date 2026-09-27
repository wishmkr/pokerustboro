//! Windows: rectangular tile regions with their own off-screen pixel buffer.
//!
//! A window owns a `WindowTemplate` (where it sits on a background and which
//! tiles it occupies) plus a `tileData` buffer that text and graphics are
//! drawn into before being pushed to VRAM.

use crate::blit::{
    BlitBitmapRect4Bit, BlitBitmapRect4BitTo8Bit, FillBitmapRect4Bit, FillBitmapRect8Bit,
};
use crate::ffi::{Alloc, AllocZeroed, CpuSet, Free, WindowTemplate};
use core::ffi::c_void;

pub const WINDOWS_MAX: usize = 32;
pub const NUM_BACKGROUNDS: usize = 4;
pub const WINDOW_NONE: u8 = 0xff;

const BG_ATTR_METRIC: u8 = 8;
const BG_ATTR_BASETILE: u8 = 10;

const COPYWIN_MAP: u8 = 1;
const COPYWIN_GFX: u8 = 2;
const COPYWIN_FULL: u8 = 3;

const WINDOW_BG: u8 = 0;
const WINDOW_TILEMAP_LEFT: u8 = 1;
const WINDOW_TILEMAP_TOP: u8 = 2;
const WINDOW_WIDTH: u8 = 3;
const WINDOW_HEIGHT: u8 = 4;
const WINDOW_PALETTE_NUM: u8 = 5;
const WINDOW_BASE_BLOCK: u8 = 6;
const WINDOW_TILE_DATA: u8 = 7;

/// `sizeof(struct Window)`: an eight-byte template then a four-byte pointer.
const WINDOW_SIZE: usize = 12;
const WINDOW_TILE_DATA_OFFSET: usize = 8;

/// Offsets inside the embedded `WindowTemplate`.
const TMPL_BG: usize = 0;
const TMPL_TILEMAP_LEFT: usize = 1;
const TMPL_TILEMAP_TOP: usize = 2;
const TMPL_WIDTH: usize = 3;
const TMPL_HEIGHT: usize = 4;
const TMPL_PALETTE_NUM: usize = 5;
const TMPL_BASE_BLOCK: usize = 6;
const TEMPLATE_SIZE: usize = 8;

/// A 4bpp tile is 32 bytes, an 8bpp tile 64.
const TILE_SIZE_4BPP: usize = 32;
const TILE_SIZE_8BPP: usize = 64;

/// One `struct Window` as raw bytes; a Rust struct holding a real pointer
/// would be 16 bytes on a 64-bit host instead of 12.
#[repr(C, align(4))]
pub struct WindowArm(pub [u8; WINDOW_SIZE]);

#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gWindows: [WindowArm; WINDOWS_MAX] =
    [const { WindowArm([0; WINDOW_SIZE]) }; WINDOWS_MAX];

/// Set to 0 and never changed.
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gTransparentTileNumber: u8 = 0;

#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gWindowBgTilemapBuffers: [*mut u8; NUM_BACKGROUNDS] =
    [core::ptr::null_mut(); NUM_BACKGROUNDS];

#[unsafe(link_section = "ewram_data")]
static mut WINDOW_PTR: *mut u8 = core::ptr::null_mut();

#[unsafe(link_section = "ewram_data")]
static mut WINDOW_SIZE_8BIT: u16 = 0;

use crate::bg::{
    BgTileAllocOp, CopyBgTilemapBufferToVram, FillBgTilemapBufferRect, GetBgAttribute,
    GetBgTilemapBuffer, LoadBgTiles, SetBgTilemapBuffer, WriteSequenceToBgTilemapBuffer,
    gWindowTileAutoAllocEnabled,
};

unsafe extern "C" {
    fn LZ77UnCompWram(src: *const u32, dest: *mut c_void);
}

/// The buffer slot for a background is either NULL, a real allocation, or the
/// address of this function used purely as a "do not free me" marker. That is
/// what the original does, oddly enough.
unsafe extern "C" fn dummy_window_bg_tilemap() {}

unsafe extern "C" fn dummy_window_bg_tilemap_8bit() {}

#[inline]
fn dummy_marker() -> *mut u8 {
    dummy_window_bg_tilemap as *const () as *mut u8
}

#[inline]
fn dummy_marker_8bit() -> *mut u8 {
    dummy_window_bg_tilemap_8bit as *const () as *mut u8
}

#[inline]
unsafe fn window(window_id: u8) -> *mut u8 {
    unsafe {
        (&raw mut gWindows)
            .cast::<u8>()
            .add(window_id as usize * WINDOW_SIZE)
    }
}

#[inline]
unsafe fn tmpl_u8(window_id: u8, offset: usize) -> u8 {
    unsafe { window(window_id).add(offset).read_volatile() }
}

#[inline]
unsafe fn set_tmpl_u8(window_id: u8, offset: usize, value: u8) {
    unsafe { window(window_id).add(offset).write_volatile(value) };
}

#[inline]
unsafe fn base_block(window_id: u8) -> u16 {
    unsafe {
        window(window_id)
            .add(TMPL_BASE_BLOCK)
            .cast::<u16>()
            .read_volatile()
    }
}

#[inline]
unsafe fn set_base_block(window_id: u8, value: u16) {
    unsafe {
        window(window_id)
            .add(TMPL_BASE_BLOCK)
            .cast::<u16>()
            .write_volatile(value)
    };
}

#[inline]
unsafe fn tile_data(window_id: u8) -> *mut u8 {
    unsafe {
        window(window_id)
            .add(WINDOW_TILE_DATA_OFFSET)
            .cast::<*mut u8>()
            .read()
    }
}

#[inline]
unsafe fn set_tile_data(window_id: u8, value: *mut u8) {
    unsafe {
        window(window_id)
            .add(WINDOW_TILE_DATA_OFFSET)
            .cast::<*mut u8>()
            .write(value)
    };
}

/// `gWindows[windowId].window = *template`
#[inline]
unsafe fn copy_template_in(window_id: u8, template: *const WindowTemplate) {
    unsafe {
        core::ptr::copy_nonoverlapping(template.cast::<u8>(), window(window_id), TEMPLATE_SIZE)
    };
}

/// `gWindows[windowId].window = sDummyWindowTemplate`, which is all zeroes
/// except for `bg = 0xFF`.
#[inline]
unsafe fn clear_template(window_id: u8) {
    unsafe { core::ptr::write_bytes(window(window_id), 0, TEMPLATE_SIZE) };
    unsafe { set_tmpl_u8(window_id, TMPL_BG, 0xff) };
}

#[inline]
unsafe fn buffer(bg: usize) -> *mut u8 {
    unsafe {
        (&raw const gWindowBgTilemapBuffers)
            .cast::<*mut u8>()
            .add(bg)
            .read()
    }
}

#[inline]
unsafe fn set_buffer(bg: usize, value: *mut u8) {
    unsafe {
        (&raw mut gWindowBgTilemapBuffers)
            .cast::<*mut u8>()
            .add(bg)
            .write(value)
    };
}

#[inline]
unsafe fn auto_alloc_enabled() -> bool {
    let enabled = unsafe { (&raw const gWindowTileAutoAllocEnabled).read_volatile() };
    enabled == 1
}

/// Allocates and zeroes the shared tilemap buffer for a background if it does
/// not have one yet. Returns false only when the allocation failed.
unsafe fn ensure_bg_tilemap_buffer(bg: usize, zeroed: bool) -> bool {
    if !unsafe { buffer(bg) }.is_null() {
        return true;
    }

    let attribute = unsafe { GetBgAttribute(bg as u8, BG_ATTR_METRIC) };
    if attribute == 0xffff {
        return true;
    }

    let allocated = if zeroed {
        unsafe { AllocZeroed(u32::from(attribute)) }
    } else {
        unsafe { Alloc(u32::from(attribute)) }
    };
    if allocated.is_null() {
        return false;
    }

    // The original zeroes the block again by hand even after AllocZeroed.
    unsafe { core::ptr::write_bytes(allocated, 0, attribute as usize) };
    unsafe { set_buffer(bg, allocated) };
    unsafe { SetBgTilemapBuffer(bg as u8, allocated) };
    true
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn InitWindows(templates: *const WindowTemplate) -> u16 {
    for i in 0..NUM_BACKGROUNDS {
        let existing = unsafe { GetBgTilemapBuffer(i as u8) };
        // A background that already has a buffer gets the marker so it is
        // never freed; otherwise the NULL is stored as-is.
        unsafe {
            set_buffer(
                i,
                if existing.is_null() {
                    existing
                } else {
                    dummy_marker()
                },
            )
        };
    }

    for i in 0..WINDOWS_MAX {
        unsafe { clear_template(i as u8) };
        unsafe { set_tile_data(i as u8, core::ptr::null_mut()) };
    }

    let mut i = 0usize;
    while i < WINDOWS_MAX {
        let template = unsafe { templates.add(i) };
        let bg = unsafe { template.cast::<u8>().add(TMPL_BG).read() };
        if bg == 0xff {
            break;
        }

        let width = unsafe { template.cast::<u8>().add(TMPL_WIDTH).read() };
        let height = unsafe { template.cast::<u8>().add(TMPL_HEIGHT).read() };
        let tiles = i32::from(width) * i32::from(height);

        let mut allocated_base_block = 0i32;
        if unsafe { auto_alloc_enabled() } {
            allocated_base_block = unsafe { BgTileAllocOp(i32::from(bg), 0, tiles, 0) };
            if allocated_base_block == -1 {
                return 0;
            }
        }

        if !unsafe { ensure_bg_tilemap_buffer(bg as usize, true) } {
            unsafe { FreeAllWindowBuffers() };
            return 0;
        }

        let pixels = unsafe { AllocZeroed((TILE_SIZE_4BPP as u32 * tiles as u32) as u16 as u32) };
        if pixels.is_null() {
            unsafe { release_bg_buffer_if_unused(bg as usize, dummy_marker(), pixels) };
            return 0;
        }

        unsafe { set_tile_data(i as u8, pixels) };
        unsafe { copy_template_in(i as u8, template) };

        if unsafe { auto_alloc_enabled() } {
            unsafe { set_base_block(i as u8, allocated_base_block as u16) };
            unsafe { BgTileAllocOp(i32::from(bg), allocated_base_block, tiles, 1) };
        }

        i += 1;
    }

    unsafe { (&raw mut gTransparentTileNumber).write_volatile(0) };
    1
}

/// On an allocation failure the original hands the background's buffer back
/// only when nothing else is using it, and stores the failed pointer into the
/// slot rather than a plain NULL.
unsafe fn release_bg_buffer_if_unused(bg: usize, marker: *mut u8, replacement: *mut u8) {
    if unsafe { active_windows_on_bg(bg as u8) } == 0 && unsafe { buffer(bg) } != marker {
        unsafe { Free(buffer(bg)) };
        unsafe { set_buffer(bg, replacement) };
    }
}

/// First slot whose background is 0xFF, or WINDOWS_MAX when all are taken.
unsafe fn first_free_window() -> usize {
    let mut win = 0usize;
    while win < WINDOWS_MAX {
        if unsafe { tmpl_u8(win as u8, TMPL_BG) } == 0xff {
            break;
        }
        win += 1;
    }
    win
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn AddWindow(template: *const WindowTemplate) -> u16 {
    let win = unsafe { first_free_window() };
    if win == WINDOWS_MAX {
        return u16::from(WINDOW_NONE);
    }

    let bg = unsafe { template.cast::<u8>().add(TMPL_BG).read() };
    let width = unsafe { template.cast::<u8>().add(TMPL_WIDTH).read() };
    let height = unsafe { template.cast::<u8>().add(TMPL_HEIGHT).read() };
    let tiles = i32::from(width) * i32::from(height);

    let mut allocated_base_block = 0i32;
    if unsafe { auto_alloc_enabled() } {
        allocated_base_block = unsafe { BgTileAllocOp(i32::from(bg), 0, tiles, 0) };
        if allocated_base_block == -1 {
            return u16::from(WINDOW_NONE);
        }
    }

    if !unsafe { ensure_bg_tilemap_buffer(bg as usize, true) } {
        return u16::from(WINDOW_NONE);
    }

    let pixels = unsafe { AllocZeroed((TILE_SIZE_4BPP as u32 * tiles as u32) as u16 as u32) };
    if pixels.is_null() {
        unsafe { release_bg_buffer_if_unused(bg as usize, dummy_marker(), pixels) };
        return u16::from(WINDOW_NONE);
    }

    unsafe { set_tile_data(win as u8, pixels) };
    unsafe { copy_template_in(win as u8, template) };

    if unsafe { auto_alloc_enabled() } {
        unsafe { set_base_block(win as u8, allocated_base_block as u16) };
        let width = unsafe { tmpl_u8(win as u8, TMPL_WIDTH) };
        let height = unsafe { tmpl_u8(win as u8, TMPL_HEIGHT) };
        unsafe {
            BgTileAllocOp(
                i32::from(bg),
                allocated_base_block,
                i32::from(width) * i32::from(height),
                1,
            )
        };
    }

    win as u16
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn AddWindowWithoutTileMap(template: *const WindowTemplate) -> i32 {
    let win = unsafe { first_free_window() };
    if win == WINDOWS_MAX {
        return i32::from(WINDOW_NONE);
    }

    let bg = unsafe { template.cast::<u8>().add(TMPL_BG).read() };
    let width = unsafe { template.cast::<u8>().add(TMPL_WIDTH).read() };
    let height = unsafe { template.cast::<u8>().add(TMPL_HEIGHT).read() };
    let tiles = i32::from(width) * i32::from(height);

    let mut allocated_base_block = 0i32;
    if unsafe { auto_alloc_enabled() } {
        allocated_base_block = unsafe { BgTileAllocOp(i32::from(bg), 0, tiles, 0) };
        if allocated_base_block == -1 {
            return i32::from(WINDOW_NONE);
        }
    }

    unsafe { copy_template_in(win as u8, template) };

    if unsafe { auto_alloc_enabled() } {
        unsafe { set_base_block(win as u8, allocated_base_block as u16) };
        let width = unsafe { tmpl_u8(win as u8, TMPL_WIDTH) };
        let height = unsafe { tmpl_u8(win as u8, TMPL_HEIGHT) };
        unsafe {
            BgTileAllocOp(
                i32::from(bg),
                allocated_base_block,
                i32::from(width) * i32::from(height),
                1,
            )
        };
    }

    win as i32
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn RemoveWindow(window_id: u8) {
    let bg = unsafe { tmpl_u8(window_id, TMPL_BG) };

    if unsafe { auto_alloc_enabled() } {
        let width = unsafe { tmpl_u8(window_id, TMPL_WIDTH) };
        let height = unsafe { tmpl_u8(window_id, TMPL_HEIGHT) };
        unsafe {
            BgTileAllocOp(
                i32::from(bg),
                i32::from(base_block(window_id)),
                i32::from(width) * i32::from(height),
                2,
            )
        };
    }

    unsafe { clear_template(window_id) };

    if unsafe { active_windows_on_bg(bg) } == 0 && unsafe { buffer(bg as usize) } != dummy_marker()
    {
        unsafe { Free(buffer(bg as usize)) };
        unsafe { set_buffer(bg as usize, core::ptr::null_mut()) };
    }

    if !unsafe { tile_data(window_id) }.is_null() {
        unsafe { Free(tile_data(window_id)) };
        unsafe { set_tile_data(window_id, core::ptr::null_mut()) };
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn FreeAllWindowBuffers() {
    for i in 0..NUM_BACKGROUNDS {
        let pointer = unsafe { buffer(i) };
        if !pointer.is_null() && pointer != dummy_marker() {
            unsafe { Free(pointer) };
            unsafe { set_buffer(i, core::ptr::null_mut()) };
        }
    }

    for i in 0..WINDOWS_MAX {
        let pixels = unsafe { tile_data(i as u8) };
        if !pixels.is_null() {
            unsafe { Free(pixels) };
            unsafe { set_tile_data(i as u8, core::ptr::null_mut()) };
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn CopyWindowToVram(window_id: u8, mode: u8) {
    let bg = unsafe { tmpl_u8(window_id, TMPL_BG) };
    let width = unsafe { tmpl_u8(window_id, TMPL_WIDTH) };
    let height = unsafe { tmpl_u8(window_id, TMPL_HEIGHT) };
    let size = (TILE_SIZE_4BPP as u32 * u32::from(width) * u32::from(height)) as u16;

    if mode == COPYWIN_GFX || mode == COPYWIN_FULL {
        unsafe { LoadBgTiles(bg, tile_data(window_id).cast(), size, base_block(window_id)) };
    }
    if mode == COPYWIN_MAP || mode == COPYWIN_FULL {
        unsafe { CopyBgTilemapBufferToVram(bg) };
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn CopyWindowRectToVram(
    window_id: u32,
    mode: u32,
    x: u32,
    y: u32,
    w: u32,
    h: u32,
) {
    if w == 0 || h == 0 {
        return;
    }

    let window_id = window_id as u8;
    let bg = unsafe { tmpl_u8(window_id, TMPL_BG) };
    let width = u32::from(unsafe { tmpl_u8(window_id, TMPL_WIDTH) });

    // Tiles from (x, y) to the end of the rectangle's last row.
    let rect_size = (((h - 1) * width) + (width - x) - (width - (x + w))) * TILE_SIZE_4BPP as u32;
    let rect_pos = (y * width) + x;

    let mode = mode as u8;
    if mode == COPYWIN_GFX || mode == COPYWIN_FULL {
        unsafe {
            LoadBgTiles(
                bg,
                tile_data(window_id)
                    .add(rect_pos as usize * TILE_SIZE_4BPP)
                    .cast(),
                rect_size as u16,
                base_block(window_id).wrapping_add(rect_pos as u16),
            )
        };
    }
    if mode == COPYWIN_MAP || mode == COPYWIN_FULL {
        unsafe { CopyBgTilemapBufferToVram(bg) };
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn PutWindowTilemap(window_id: u8) {
    let bg = unsafe { tmpl_u8(window_id, TMPL_BG) };
    unsafe {
        WriteSequenceToBgTilemapBuffer(
            bg,
            GetBgAttribute(bg, BG_ATTR_BASETILE).wrapping_add(base_block(window_id)),
            tmpl_u8(window_id, TMPL_TILEMAP_LEFT),
            tmpl_u8(window_id, TMPL_TILEMAP_TOP),
            tmpl_u8(window_id, TMPL_WIDTH),
            tmpl_u8(window_id, TMPL_HEIGHT),
            tmpl_u8(window_id, TMPL_PALETTE_NUM),
            1,
        )
    };
}

/// Writes `height` single-row runs, stepping one window row of tiles each
/// time, so a sub-rectangle of the window lands in the right tilemap cells.
unsafe fn put_window_rect(window_id: u8, x: u8, y: u8, width: u8, height: u8, palette: u8) {
    let bg = unsafe { tmpl_u8(window_id, TMPL_BG) };
    let window_width = unsafe { tmpl_u8(window_id, TMPL_WIDTH) };
    let mut current_row = unsafe { base_block(window_id) }
        .wrapping_add(u16::from(y) * u16::from(window_width))
        .wrapping_add(u16::from(x))
        .wrapping_add(unsafe { GetBgAttribute(bg, BG_ATTR_BASETILE) });

    let mut i = 0u8;
    while i < height {
        unsafe {
            WriteSequenceToBgTilemapBuffer(
                bg,
                current_row,
                tmpl_u8(window_id, TMPL_TILEMAP_LEFT).wrapping_add(x),
                tmpl_u8(window_id, TMPL_TILEMAP_TOP)
                    .wrapping_add(y)
                    .wrapping_add(i),
                width,
                1,
                palette,
                1,
            )
        };
        current_row = current_row.wrapping_add(u16::from(window_width));
        i += 1;
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn PutWindowRectTilemapOverridePalette(
    window_id: u8,
    x: u8,
    y: u8,
    width: u8,
    height: u8,
    palette: u8,
) {
    unsafe { put_window_rect(window_id, x, y, width, height, palette) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn PutWindowRectTilemap(window_id: u8, x: u8, y: u8, width: u8, height: u8) {
    let palette = unsafe { tmpl_u8(window_id, TMPL_PALETTE_NUM) };
    unsafe { put_window_rect(window_id, x, y, width, height, palette) };
}

/// Fills the window's tilemap cells with the transparent tile.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ClearWindowTilemap(window_id: u8) {
    unsafe {
        FillBgTilemapBufferRect(
            tmpl_u8(window_id, TMPL_BG),
            u16::from((&raw const gTransparentTileNumber).read_volatile()),
            tmpl_u8(window_id, TMPL_TILEMAP_LEFT),
            tmpl_u8(window_id, TMPL_TILEMAP_TOP),
            tmpl_u8(window_id, TMPL_WIDTH),
            tmpl_u8(window_id, TMPL_HEIGHT),
            tmpl_u8(window_id, TMPL_PALETTE_NUM),
        )
    };
}

/// Builds the eight-byte ARM `struct Bitmap` the blit routines expect.
/// A Rust struct would put the size fields at the wrong offset on a host with
/// eight-byte pointers.
#[repr(C, align(4))]
struct BitmapArm([u8; 8]);

impl BitmapArm {
    fn new(pixels: *mut u8, width: u16, height: u16) -> Self {
        let mut bytes = [0u8; 8];
        bytes[0..4].copy_from_slice(&(pixels as usize as u32).to_le_bytes());
        bytes[4..6].copy_from_slice(&width.to_le_bytes());
        bytes[6..8].copy_from_slice(&height.to_le_bytes());
        Self(bytes)
    }

    #[inline]
    fn as_ptr(&self) -> *const u8 {
        self.0.as_ptr()
    }

    #[inline]
    fn as_mut_ptr(&mut self) -> *mut u8 {
        self.0.as_mut_ptr()
    }
}

/// The window's pixel buffer seen as a bitmap, eight pixels per tile.
unsafe fn window_bitmap(window_id: u8) -> BitmapArm {
    BitmapArm::new(
        unsafe { tile_data(window_id) },
        8 * u16::from(unsafe { tmpl_u8(window_id, TMPL_WIDTH) }),
        8 * u16::from(unsafe { tmpl_u8(window_id, TMPL_HEIGHT) }),
    )
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn BlitBitmapToWindow(
    window_id: u8,
    pixels: *const u8,
    x: u16,
    y: u16,
    width: u16,
    height: u16,
) {
    unsafe {
        BlitBitmapRectToWindow(
            window_id,
            pixels,
            0,
            0,
            width,
            height as i32,
            x,
            y,
            width,
            height,
        )
    };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn BlitBitmapRectToWindow(
    window_id: u8,
    pixels: *const u8,
    src_x: u16,
    src_y: u16,
    src_width: u16,
    src_height: i32,
    dest_x: u16,
    dest_y: u16,
    rect_width: u16,
    rect_height: u16,
) {
    let source = BitmapArm::new(pixels.cast_mut(), src_width, src_height as u16);
    let mut destination = unsafe { window_bitmap(window_id) };

    unsafe {
        BlitBitmapRect4Bit(
            source.as_ptr(),
            destination.as_mut_ptr(),
            src_x,
            src_y,
            dest_x,
            dest_y,
            rect_width,
            rect_height,
            0,
        )
    };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn FillWindowPixelRect(
    window_id: u8,
    fill_value: u8,
    x: u16,
    y: u16,
    width: u16,
    height: u16,
) {
    let mut rect = unsafe { window_bitmap(window_id) };
    unsafe { FillBitmapRect4Bit(rect.as_mut_ptr(), x, y, width, height, fill_value) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn CopyToWindowPixelBuffer(
    window_id: u8,
    src: *const c_void,
    size: u16,
    tile_offset: u16,
) {
    let dest = unsafe { tile_data(window_id).add(TILE_SIZE_4BPP * tile_offset as usize) };
    if size != 0 {
        // CpuCopy16: halfword units, no fixed source.
        unsafe { CpuSet(src, dest.cast(), u32::from(size) / 2 & 0x1f_ffff) };
    } else {
        unsafe { LZ77UnCompWram(src.cast(), dest.cast()) };
    }
}

/// Sets every pixel in the window to `fillValue`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FillWindowPixelBuffer(window_id: u8, fill_value: u8) {
    let width = unsafe { tmpl_u8(window_id, TMPL_WIDTH) };
    let height = unsafe { tmpl_u8(window_id, TMPL_HEIGHT) };
    let size = TILE_SIZE_4BPP * usize::from(width) * usize::from(height);

    // CpuFastFill8 from a fixed word of four copies of the fill value.
    let word = u32::from_ne_bytes([fill_value; 4]);
    unsafe {
        crate::ffi::CpuFastSet(
            (&raw const word).cast(),
            tile_data(window_id).cast(),
            0x0100_0000 | (size as u32 / 4 & 0x1f_ffff),
        )
    };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrollWindow(window_id: u8, direction: u8, distance: u8, fill_value: u8) {
    let width = u32::from(unsafe { tmpl_u8(window_id, TMPL_WIDTH) });
    let height = u32::from(unsafe { tmpl_u8(window_id, TMPL_HEIGHT) });
    let fill_word = u32::from_ne_bytes([fill_value; 4]);
    let size = (height * width * TILE_SIZE_4BPP as u32) as i32;
    let base = unsafe { tile_data(window_id) };

    // Rows are four bytes and tiles are eight rows, so a scroll by `distance`
    // rows means reading from a row `distance` further on, which crosses into
    // the next tile every eight rows. That is what the width multiply does.
    let source_offset = |i: i32, distance_loop: u32| -> i32 {
        i + (((width * (distance_loop & !7)) | (distance_loop & 7)) * 4) as i32
    };

    match direction {
        0 => {
            let mut i = 0i32;
            while i < size {
                let mut distance_loop = u32::from(distance);
                let mut row = 0i32;
                while row < TILE_SIZE_4BPP as i32 {
                    let dest = i + row;
                    let src = source_offset(i, distance_loop);
                    let value = if src < size {
                        unsafe { base.offset(src as isize).cast::<u32>().read() }
                    } else {
                        fill_word
                    };
                    unsafe { base.offset(dest as isize).cast::<u32>().write(value) };
                    distance_loop += 1;
                    row += 4;
                }
                i += TILE_SIZE_4BPP as i32;
            }
        }
        1 => {
            // Upward scrolling walks the buffer backwards from the last row.
            let base = unsafe { base.offset((size - 4) as isize) };
            let mut i = 0i32;
            while i < size {
                let mut distance_loop = u32::from(distance);
                let mut row = 0i32;
                while row < TILE_SIZE_4BPP as i32 {
                    let dest = i + row;
                    let src = source_offset(i, distance_loop);
                    let value = if src < size {
                        unsafe { base.offset(-(src as isize)).cast::<u32>().read() }
                    } else {
                        fill_word
                    };
                    unsafe { base.offset(-(dest as isize)).cast::<u32>().write(value) };
                    distance_loop += 1;
                    row += 4;
                }
                i += TILE_SIZE_4BPP as i32;
            }
        }
        _ => {}
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn CallWindowFunction(
    window_id: u8,
    func: unsafe extern "C" fn(u8, u8, u8, u8, u8, u8),
) {
    unsafe {
        func(
            tmpl_u8(window_id, TMPL_BG),
            tmpl_u8(window_id, TMPL_TILEMAP_LEFT),
            tmpl_u8(window_id, TMPL_TILEMAP_TOP),
            tmpl_u8(window_id, TMPL_WIDTH),
            tmpl_u8(window_id, TMPL_HEIGHT),
            tmpl_u8(window_id, TMPL_PALETTE_NUM),
        )
    };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetWindowAttribute(window_id: u8, attribute_id: u8, value: u32) -> u8 {
    match attribute_id {
        WINDOW_TILEMAP_LEFT => {
            unsafe { set_tmpl_u8(window_id, TMPL_TILEMAP_LEFT, value as u8) };
            0
        }
        WINDOW_TILEMAP_TOP => {
            unsafe { set_tmpl_u8(window_id, TMPL_TILEMAP_TOP, value as u8) };
            0
        }
        WINDOW_PALETTE_NUM => {
            unsafe { set_tmpl_u8(window_id, TMPL_PALETTE_NUM, value as u8) };
            0
        }
        WINDOW_BASE_BLOCK => {
            unsafe { set_base_block(window_id, value as u16) };
            0
        }
        WINDOW_TILE_DATA => {
            unsafe { set_tile_data(window_id, value as usize as *mut u8) };
            1
        }
        // WINDOW_BG, WINDOW_WIDTH and WINDOW_HEIGHT are deliberately
        // read-only: changing them would invalidate the allocation.
        _ => 1,
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetWindowAttribute(window_id: u8, attribute_id: u8) -> u32 {
    match attribute_id {
        WINDOW_BG => u32::from(unsafe { tmpl_u8(window_id, TMPL_BG) }),
        WINDOW_TILEMAP_LEFT => u32::from(unsafe { tmpl_u8(window_id, TMPL_TILEMAP_LEFT) }),
        WINDOW_TILEMAP_TOP => u32::from(unsafe { tmpl_u8(window_id, TMPL_TILEMAP_TOP) }),
        WINDOW_WIDTH => u32::from(unsafe { tmpl_u8(window_id, TMPL_WIDTH) }),
        WINDOW_HEIGHT => u32::from(unsafe { tmpl_u8(window_id, TMPL_HEIGHT) }),
        WINDOW_PALETTE_NUM => u32::from(unsafe { tmpl_u8(window_id, TMPL_PALETTE_NUM) }),
        WINDOW_BASE_BLOCK => u32::from(unsafe { base_block(window_id) }),
        WINDOW_TILE_DATA => {
            let pixels = unsafe { tile_data(window_id) };
            pixels as usize as u32
        }
        _ => 0,
    }
}

/// Counts windows assigned to a background. Both the 4bpp and 8bpp variants
/// in the original are identical; only their dummy markers differ.
unsafe fn active_windows_on_bg(bg_id: u8) -> u8 {
    let mut count = 0u8;
    for i in 0..WINDOWS_MAX {
        if unsafe { tmpl_u8(i as u8, TMPL_BG) } == bg_id {
            count += 1;
        }
    }
    count
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn AddWindow8Bit(template: *const WindowTemplate) -> u16 {
    let win = unsafe { first_free_window() };
    if win == WINDOWS_MAX {
        return u16::from(WINDOW_NONE);
    }

    let bg = unsafe { template.cast::<u8>().add(TMPL_BG).read() };
    if !unsafe { ensure_bg_tilemap_buffer(bg as usize, false) } {
        return u16::from(WINDOW_NONE);
    }

    let width = unsafe { template.cast::<u8>().add(TMPL_WIDTH).read() };
    let height = unsafe { template.cast::<u8>().add(TMPL_HEIGHT).read() };
    let pixels = unsafe {
        Alloc((TILE_SIZE_8BPP as u32 * u32::from(width) * u32::from(height)) as u16 as u32)
    };

    if pixels.is_null() {
        // Unlike the 4bpp path this one stores a plain NULL back.
        if unsafe { active_windows_on_bg(bg) } == 0
            && unsafe { buffer(bg as usize) } != dummy_marker_8bit()
        {
            unsafe { Free(buffer(bg as usize)) };
            unsafe { set_buffer(bg as usize, core::ptr::null_mut()) };
        }
        return u16::from(WINDOW_NONE);
    }

    unsafe { set_tile_data(win as u8, pixels) };
    unsafe { copy_template_in(win as u8, template) };
    win as u16
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn FillWindowPixelBuffer8Bit(window_id: u8, fill_value: u8) {
    let width = unsafe { tmpl_u8(window_id, TMPL_WIDTH) };
    let height = unsafe { tmpl_u8(window_id, TMPL_HEIGHT) };
    let size = (TILE_SIZE_8BPP as u32 * u32::from(width) * u32::from(height)) as u16;

    let pixels = unsafe { tile_data(window_id) };
    let mut i = 0u32;
    while i < u32::from(size) {
        unsafe { pixels.add(i as usize).write(fill_value) };
        i += 1;
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn FillWindowPixelRect8Bit(
    window_id: u8,
    fill_value: u8,
    x: u16,
    y: u16,
    width: u16,
    height: u16,
) {
    let mut rect = unsafe { window_bitmap(window_id) };
    unsafe { FillBitmapRect8Bit(rect.as_mut_ptr(), x, y, width, height, fill_value) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn BlitBitmapRectToWindow4BitTo8Bit(
    window_id: u8,
    pixels: *const u8,
    src_x: u16,
    src_y: u16,
    src_width: u16,
    src_height: i32,
    dest_x: u16,
    dest_y: u16,
    rect_width: u16,
    rect_height: u16,
    palette_num: u8,
) {
    let source = BitmapArm::new(pixels.cast_mut(), src_width, src_height as u16);
    let mut destination = unsafe { window_bitmap(window_id) };

    unsafe {
        BlitBitmapRect4BitTo8Bit(
            source.as_ptr(),
            destination.as_mut_ptr(),
            src_x,
            src_y,
            dest_x,
            dest_y,
            rect_width,
            rect_height,
            0,
            palette_num,
        )
    };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn CopyWindowToVram8Bit(window_id: u8, mode: u8) {
    unsafe { (&raw mut WINDOW_PTR).write(window(window_id)) };
    let width = unsafe { tmpl_u8(window_id, TMPL_WIDTH) };
    let height = unsafe { tmpl_u8(window_id, TMPL_HEIGHT) };
    let size = (TILE_SIZE_8BPP as u32 * u32::from(width) * u32::from(height)) as u16;
    unsafe { (&raw mut WINDOW_SIZE_8BIT).write_volatile(size) };

    let bg = unsafe { tmpl_u8(window_id, TMPL_BG) };
    if mode == COPYWIN_GFX || mode == COPYWIN_FULL {
        unsafe { LoadBgTiles(bg, tile_data(window_id).cast(), size, base_block(window_id)) };
    }
    if mode == COPYWIN_MAP || mode == COPYWIN_FULL {
        unsafe { CopyBgTilemapBufferToVram(bg) };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn window_layout_matches_the_arm_structure() {
        assert_eq!(WINDOW_SIZE, 12);
        assert_eq!(TEMPLATE_SIZE, 8);
        assert_eq!(WINDOW_TILE_DATA_OFFSET, TEMPLATE_SIZE);
        assert_eq!(core::mem::size_of::<WindowArm>(), 12);
        assert_eq!(core::mem::size_of::<BitmapArm>(), 8);
    }

    #[test]
    fn attribute_ids_match_the_enumeration() {
        assert_eq!(
            [
                WINDOW_BG,
                WINDOW_TILEMAP_LEFT,
                WINDOW_TILEMAP_TOP,
                WINDOW_WIDTH,
                WINDOW_HEIGHT,
                WINDOW_PALETTE_NUM,
                WINDOW_BASE_BLOCK,
                WINDOW_TILE_DATA
            ],
            [0, 1, 2, 3, 4, 5, 6, 7]
        );
    }

    #[test]
    fn a_bitmap_is_built_in_arm_layout() {
        let bitmap = BitmapArm::new(core::ptr::null_mut(), 0x1234, 0x5678);
        assert_eq!(&bitmap.0[4..6], &0x1234u16.to_le_bytes());
        assert_eq!(&bitmap.0[6..8], &0x5678u16.to_le_bytes());
    }

    #[test]
    fn scroll_source_offsets_cross_into_the_next_tile_every_eight_rows() {
        let width = 10u32;
        let offset = |d: u32| ((width * (d & !7)) | (d & 7)) * 4;
        // Within the first tile the step is one row of four bytes.
        assert_eq!(offset(0), 0);
        assert_eq!(offset(1), 4);
        assert_eq!(offset(7), 28);
        // Row eight is the first row of the tile one window-row down.
        assert_eq!(offset(8), width * 8 * 4);
        assert_eq!(offset(9), width * 8 * 4 + 4);
    }
}
