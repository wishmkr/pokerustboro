//! Bag item icons as sprites: decompress the 24x24 picture into a 32x32
//! sheet, load its palette and create the sprite. Tables in
//! `data/item_icon.rs`.

use crate::data::item_icon::{gItemIconSpriteTemplate, gItemIconTable};
use crate::ffi::{CompressedSpritePalette, CpuSet, LoadCompressedSpritePalette, SpriteTemplate};
use crate::malloc::{Alloc, AllocZeroed, Free};
use crate::sprite::{CreateSprite, LoadSpriteSheet, MAX_SPRITES};

const ITEM_LIST_END: u16 = 0xffff;
const ITEMS_COUNT: u16 = 0x179;
const SPRITE_TEMPLATE_SIZE: u32 = 24;

#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gItemIconDecompressionBuffer: *mut u8 = core::ptr::null_mut();

#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gItemIcon4x4Buffer: *mut u8 = core::ptr::null_mut();

/// `struct SpriteSheet { const void *data; u16 size; u16 tag; }`
#[repr(C, align(4))]
struct SpriteSheet {
    data: *const u8,
    size: u16,
    tag: u16,
}

#[inline]
unsafe fn cpu_copy16(src: *const u8, dest: *mut u8, size: u32) {
    unsafe { CpuSet(src.cast(), dest.cast(), (size / 2) & 0x1f_ffff) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn AllocItemIconTemporaryBuffers() -> u8 {
    let decompression = unsafe { Alloc(0x120) };
    unsafe { (&raw mut gItemIconDecompressionBuffer).write(decompression) };
    if decompression.is_null() {
        return 0;
    }
    let buffer = unsafe { AllocZeroed(0x200) };
    unsafe { (&raw mut gItemIcon4x4Buffer).write(buffer) };
    if buffer.is_null() {
        unsafe { Free(decompression) };
        return 0;
    }
    1
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn FreeItemIconTemporaryBuffers() {
    unsafe { Free((&raw const gItemIconDecompressionBuffer).read()) };
    unsafe { Free((&raw const gItemIcon4x4Buffer).read()) };
}

/// Spreads three rows of three tiles into the top of a four-tile-wide sheet.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CopyItemIconPicTo4x4Buffer(src: *const u8, dest: *mut u8) {
    for i in 0..3 {
        unsafe { cpu_copy16(src.add(i * 96), dest.add(i * 128), 0x60) };
    }
}

unsafe fn add_icon_sprite(
    template: *const SpriteTemplate,
    tiles_tag: u16,
    palette_tag: u16,
    item_id: u16,
) -> u8 {
    if unsafe { AllocItemIconTemporaryBuffers() } == 0 {
        return MAX_SPRITES as u8;
    }
    let decompression = unsafe { (&raw const gItemIconDecompressionBuffer).read() };
    let buffer = unsafe { (&raw const gItemIcon4x4Buffer).read() };

    unsafe {
        crate::decompress::LZDecompressWram(
            GetItemIconPicOrPalette(item_id, 0).cast(),
            decompression,
        )
    };
    unsafe { CopyItemIconPicTo4x4Buffer(decompression, buffer) };
    let sheet = SpriteSheet {
        data: buffer,
        size: 0x200,
        tag: tiles_tag,
    };
    unsafe { LoadSpriteSheet((&raw const sheet).cast()) };

    let palette = CompressedSpritePalette {
        data: unsafe { GetItemIconPicOrPalette(item_id, 1) }.cast(),
        tag: palette_tag,
    };
    unsafe { LoadCompressedSpritePalette(&raw const palette) };

    let copy = unsafe { Alloc(SPRITE_TEMPLATE_SIZE) };
    unsafe { cpu_copy16(template.cast(), copy, SPRITE_TEMPLATE_SIZE) };
    unsafe { copy.cast::<u16>().write(tiles_tag) };
    unsafe { copy.add(2).cast::<u16>().write(palette_tag) };
    let sprite_id = unsafe { CreateSprite(copy.cast(), 0, 0, 0) };

    unsafe { FreeItemIconTemporaryBuffers() };
    unsafe { Free(copy) };
    sprite_id
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn AddItemIconSprite(tiles_tag: u16, palette_tag: u16, item_id: u16) -> u8 {
    unsafe {
        add_icon_sprite(
            gItemIconSpriteTemplate.as_ptr().cast(),
            tiles_tag,
            palette_tag,
            item_id,
        )
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn AddCustomItemIconSprite(
    custom_template: *const SpriteTemplate,
    tiles_tag: u16,
    palette_tag: u16,
    item_id: u16,
) -> u8 {
    unsafe { add_icon_sprite(custom_template, tiles_tag, palette_tag, item_id) }
}

/// `gItemIconTable[itemId][which]`: 0 is the picture, 1 the palette.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetItemIconPicOrPalette(mut item_id: u16, which: u8) -> *const u8 {
    if item_id == ITEM_LIST_END {
        // The "return to field" arrow, stored after the last item.
        item_id = ITEMS_COUNT;
    } else if item_id >= ITEMS_COUNT {
        item_id = 0;
    }
    let index = usize::from(item_id) * 2 + usize::from(which);
    unsafe { gItemIconTable.as_ptr().add(index).read().0 }
}
