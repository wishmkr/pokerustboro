//! Bag item icons as sprites (was src/item_icon.c): the 24x24 picture is
//! decompressed, spread into a 32x32 sheet, loaded with its palette, and a
//! sprite is made from it. Tables in `data/item_icon.rs`.

use crate::data::item_icon::{gItemIconSpriteTemplate, gItemIconTable};
use crate::decompress::{LZDecompressWram, LoadCompressedSpritePalette};
use crate::ffi::{CompressedSpritePalette, CpuSet};
use crate::malloc::{Alloc, AllocZeroed, Free};
use crate::sprite::{CreateSprite, LoadSpriteSheet, MAX_SPRITES};
use crate::types::{SpriteSheet, SpriteTemplate};

const ITEM_LIST_END: u16 = 0xffff;
const ITEMS_COUNT: u16 = 0x179;

/// The compressed picture: 3x3 tiles.
const ICON_PIC_SIZE: u32 = 0x120;
/// The sprite sheet: 4x4 tiles.
const ICON_SHEET_SIZE: u16 = 0x200;
const TILE_ROW_3: usize = 3 * 32;
const TILE_ROW_4: usize = 4 * 32;

/// Which half of an item's `gItemIconTable` entry.
#[derive(Clone, Copy)]
#[repr(u8)]
pub enum IconPart {
    Pic = 0,
    Palette = 1,
}

#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gItemIconDecompressionBuffer: *mut u8 = core::ptr::null_mut();

#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gItemIcon4x4Buffer: *mut u8 = core::ptr::null_mut();

/// C's CpuCopy16.
unsafe fn cpu_copy16(src: *const u8, dest: *mut u8, size: u32) {
    unsafe { CpuSet(src.cast(), dest.cast(), (size / 2) & 0x1f_ffff) };
}

/// Allocates both buffers; false (and none kept) if the heap is full.
pub fn alloc_item_icon_temporary_buffers() -> bool {
    // SAFETY: plain heap calls; the buffers are only ours between this and
    // free_item_icon_temporary_buffers.
    unsafe {
        let pic = Alloc(ICON_PIC_SIZE);
        *(&raw mut gItemIconDecompressionBuffer) = pic;
        if pic.is_null() {
            return false;
        }
        let sheet = AllocZeroed(ICON_SHEET_SIZE.into());
        *(&raw mut gItemIcon4x4Buffer) = sheet;
        if sheet.is_null() {
            Free(pic);
            return false;
        }
    }
    true
}

pub fn free_item_icon_temporary_buffers() {
    // SAFETY: as above.
    unsafe {
        Free(*(&raw const gItemIconDecompressionBuffer));
        Free(*(&raw const gItemIcon4x4Buffer));
    }
}

/// Spreads three rows of three tiles into the top of a four-tile-wide sheet.
///
/// # Safety
/// `src` must hold a 3x3-tile picture and `dest` a 4x4-tile sheet.
pub unsafe fn copy_item_icon_pic_to_4x4_buffer(src: *const u8, dest: *mut u8) {
    for row in 0..3 {
        unsafe {
            cpu_copy16(
                src.add(row * TILE_ROW_3),
                dest.add(row * TILE_ROW_4),
                TILE_ROW_3 as u32,
            )
        };
    }
}

/// An item's compressed icon picture or palette. `ITEM_LIST_END` is the
/// "return to field" arrow, stored after the last item; other unknown items
/// get item 0's.
pub fn item_icon_pic_or_palette(item_id: u16, part: IconPart) -> *const u8 {
    let item = match item_id {
        ITEM_LIST_END => ITEMS_COUNT,
        id if id >= ITEMS_COUNT => 0,
        id => id,
    };
    gItemIconTable
        .get(usize::from(item) * 2 + part as usize)
        .map_or(core::ptr::null(), |p| p.0)
}

/// Makes an item icon sprite from `template` with the given tags; the
/// sprite id, or MAX_SPRITES when out of memory.
///
/// # Safety
/// `template` must be a valid sprite template.
pub unsafe fn add_custom_item_icon_sprite(
    template: *const SpriteTemplate,
    tiles_tag: u16,
    palette_tag: u16,
    item_id: u16,
) -> u8 {
    if !alloc_item_icon_temporary_buffers() {
        return MAX_SPRITES as u8;
    }
    // SAFETY: the buffers were just allocated with the right sizes; the
    // rest are C's sprite calls in C's order.
    unsafe {
        let pic = *(&raw const gItemIconDecompressionBuffer);
        let sheet = *(&raw const gItemIcon4x4Buffer);
        LZDecompressWram(item_icon_pic_or_palette(item_id, IconPart::Pic).cast(), pic);
        copy_item_icon_pic_to_4x4_buffer(pic, sheet);
        let sprite_sheet = SpriteSheet {
            data: sheet.cast(),
            size: ICON_SHEET_SIZE,
            tag: tiles_tag,
        };
        LoadSpriteSheet((&raw const sprite_sheet).cast());

        let palette = CompressedSpritePalette {
            data: item_icon_pic_or_palette(item_id, IconPart::Palette).cast(),
            tag: palette_tag,
        };
        LoadCompressedSpritePalette(&palette);

        // The template goes on the heap as in C: the sprite keeps a pointer
        // to it (freed below, as in C).
        let size = size_of::<SpriteTemplate>() as u32;
        let copy = Alloc(size).cast::<SpriteTemplate>();
        cpu_copy16(template.cast(), copy.cast(), size);
        (*copy).tileTag = tiles_tag;
        (*copy).paletteTag = palette_tag;
        let sprite_id = CreateSprite(copy.cast(), 0, 0, 0);

        free_item_icon_temporary_buffers();
        Free(copy.cast());
        sprite_id
    }
}

pub fn add_item_icon_sprite(tiles_tag: u16, palette_tag: u16, item_id: u16) -> u8 {
    // SAFETY: the item icon template is a valid one.
    unsafe {
        add_custom_item_icon_sprite(
            gItemIconSpriteTemplate.as_ptr().cast(),
            tiles_tag,
            palette_tag,
            item_id,
        )
    }
}

// ------------------------------------------------------------------ C names

#[unsafe(no_mangle)]
pub fn AllocItemIconTemporaryBuffers() -> u8 {
    alloc_item_icon_temporary_buffers().into()
}

#[unsafe(no_mangle)]
pub fn FreeItemIconTemporaryBuffers() {
    free_item_icon_temporary_buffers();
}

/// # Safety
/// As for [`copy_item_icon_pic_to_4x4_buffer`].
#[unsafe(no_mangle)]
pub unsafe fn CopyItemIconPicTo4x4Buffer(src: *const u8, dest: *mut u8) {
    unsafe { copy_item_icon_pic_to_4x4_buffer(src, dest) };
}

#[unsafe(no_mangle)]
pub fn AddItemIconSprite(tiles_tag: u16, palette_tag: u16, item_id: u16) -> u8 {
    add_item_icon_sprite(tiles_tag, palette_tag, item_id)
}

/// # Safety
/// As for [`add_custom_item_icon_sprite`].
#[unsafe(no_mangle)]
pub unsafe fn AddCustomItemIconSprite(
    template: *const SpriteTemplate,
    tiles_tag: u16,
    palette_tag: u16,
    item_id: u16,
) -> u8 {
    unsafe { add_custom_item_icon_sprite(template, tiles_tag, palette_tag, item_id) }
}

/// `which`: 0 for the picture, 1 for the palette.
#[unsafe(no_mangle)]
pub fn GetItemIconPicOrPalette(item_id: u16, which: u8) -> *const u8 {
    let part = if which == 0 {
        IconPart::Pic
    } else {
        IconPart::Palette
    };
    item_icon_pic_or_palette(item_id, part)
}
