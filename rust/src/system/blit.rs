//! Software blitting between GBA tile-format bitmaps.
//!
//! Pixels are not stored in scanline order: an image is a grid of 8x8 tiles,
//! each tile holding its rows back to back. The offset helpers below encode
//! that addressing once instead of repeating the original shift chains.

/// `struct Bitmap { u8 *pixels; u32 width:16; u32 height:16; }`
const BITMAP_PIXELS: usize = 0;
const BITMAP_WIDTH: usize = 4;
const BITMAP_HEIGHT: usize = 6;

#[inline]
unsafe fn pixels(bitmap: *const u8) -> *mut u8 {
    unsafe { bitmap.add(BITMAP_PIXELS).cast::<*mut u8>().read() }
}

#[inline]
unsafe fn width(bitmap: *const u8) -> i32 {
    i32::from(unsafe { bitmap.add(BITMAP_WIDTH).cast::<u16>().read() })
}

#[inline]
unsafe fn height(bitmap: *const u8) -> i32 {
    i32::from(unsafe { bitmap.add(BITMAP_HEIGHT).cast::<u16>().read() })
}

/// Tiles per row, rounded so a width that is not a multiple of eight still
/// advances a whole tile.
#[inline]
fn tiles_per_row(bitmap_width: i32) -> i32 {
    (bitmap_width + (bitmap_width & 7)) >> 3
}

/// Byte offset of the pixel pair containing (x, y) in a 4bpp bitmap.
/// Each 8x8 tile is 32 bytes and each row of a tile is 4 bytes.
#[inline]
fn offset_4bpp(x: i32, y: i32, tiles_per_row: i32) -> isize {
    (((x >> 1) & 3) + ((x >> 3) << 5) + (((y >> 3) * tiles_per_row) << 5) + ((y & 7) << 2)) as isize
}

/// Byte offset of the pixel at (x, y) in an 8bpp bitmap. Each tile is 64
/// bytes and each row of a tile is 8 bytes.
#[inline]
fn offset_8bpp(x: i32, y: i32, tiles_per_row: i32) -> isize {
    ((x & 7) + ((x >> 3) << 6) + (((y >> 3) * tiles_per_row) << 6) + ((y & 7) << 3)) as isize
}

#[unsafe(no_mangle)]
pub unsafe fn BlitBitmapRect4BitWithoutColorKey(
    src: *const u8,
    dst: *mut u8,
    src_x: u16,
    src_y: u16,
    dst_x: u16,
    dst_y: u16,
    blit_width: u16,
    blit_height: u16,
) {
    unsafe {
        BlitBitmapRect4Bit(
            src,
            dst,
            src_x,
            src_y,
            dst_x,
            dst_y,
            blit_width,
            blit_height,
            0xff,
        )
    };
}

#[unsafe(no_mangle)]
pub unsafe fn BlitBitmapRect4Bit(
    src: *const u8,
    dst: *mut u8,
    src_x: u16,
    src_y: u16,
    dst_x: u16,
    dst_y: u16,
    blit_width: u16,
    blit_height: u16,
    color_key: u8,
) {
    let (src_x, src_y) = (i32::from(src_x), i32::from(src_y));
    let (dst_x, dst_y) = (i32::from(dst_x), i32::from(dst_y));
    let (blit_width, blit_height) = (i32::from(blit_width), i32::from(blit_height));

    let dst_width = unsafe { width(dst) };
    let dst_height = unsafe { height(dst) };

    // Clip against the destination, in source coordinates.
    let x_end = if dst_width - dst_x < blit_width {
        (dst_width - dst_x) + src_x
    } else {
        src_x + blit_width
    };
    let y_end = if dst_height - dst_y < blit_height {
        (dst_height - dst_y) + src_y
    } else {
        blit_height + src_y
    };

    let src_tiles = tiles_per_row(unsafe { width(src) });
    let dst_tiles = tiles_per_row(dst_width);
    let src_pixels = unsafe { pixels(src) };
    let dst_pixels = unsafe { pixels(dst) };
    let keyed = color_key != 0xff;

    let mut loop_src_y = src_y;
    let mut loop_dst_y = dst_y;
    while loop_src_y < y_end {
        let mut loop_src_x = src_x;
        let mut loop_dst_x = dst_x;
        while loop_src_x < x_end {
            let source =
                unsafe { src_pixels.offset(offset_4bpp(loop_src_x, loop_src_y, src_tiles)) };
            let destination =
                unsafe { dst_pixels.offset(offset_4bpp(loop_dst_x, loop_dst_y, dst_tiles)) };

            let nibble = (unsafe { source.read() } >> ((loop_src_x & 1) << 2)) & 0xf;
            if !keyed || nibble != color_key {
                let shift = (loop_dst_x & 1) << 2;
                let keep = 0xf0u8 >> shift;
                unsafe { destination.write((nibble << shift) | (destination.read() & keep)) };
            }

            loop_src_x += 1;
            loop_dst_x += 1;
        }
        loop_src_y += 1;
        loop_dst_y += 1;
    }
}

#[unsafe(no_mangle)]
pub unsafe fn FillBitmapRect4Bit(
    surface: *mut u8,
    x: u16,
    y: u16,
    fill_width: u16,
    fill_height: u16,
    fill_value: u8,
) {
    let (x, y) = (i32::from(x), i32::from(y));
    let surface_width = unsafe { width(surface) };
    let surface_height = unsafe { height(surface) };

    let x_end = (x + i32::from(fill_width)).min(surface_width);
    let y_end = (y + i32::from(fill_height)).min(surface_height);

    let tiles = tiles_per_row(surface_width);
    let surface_pixels = unsafe { pixels(surface) };
    let high_nibble = fill_value << 4;
    let low_nibble = fill_value & 0xf;

    let mut loop_y = y;
    while loop_y < y_end {
        let mut loop_x = x;
        while loop_x < x_end {
            let pixel = unsafe { surface_pixels.offset(offset_4bpp(loop_x, loop_y, tiles)) };
            if loop_x & 1 != 0 {
                unsafe { pixel.write(high_nibble | (pixel.read() & 0xf)) };
            } else {
                unsafe { pixel.write(low_nibble | (pixel.read() & 0xf0)) };
            }
            loop_x += 1;
        }
        loop_y += 1;
    }
}

#[unsafe(no_mangle)]
pub unsafe fn BlitBitmapRect4BitTo8Bit(
    src: *const u8,
    dst: *mut u8,
    src_x: u16,
    src_y: u16,
    dst_x: u16,
    dst_y: u16,
    blit_width: u16,
    blit_height: u16,
    color_key: u8,
    palette_offset: u8,
) {
    let (src_x, src_y) = (i32::from(src_x), i32::from(src_y));
    let (dst_x, dst_y) = (i32::from(dst_x), i32::from(dst_y));
    let (blit_width, blit_height) = (i32::from(blit_width), i32::from(blit_height));

    // Both are widened from a nibble to a full 8bpp palette index.
    let pal_offset_bits = (palette_offset & 0xf) << 4;
    let color_key_bits = (color_key & 0xf) << 4;

    let dst_width = unsafe { width(dst) };
    let dst_height = unsafe { height(dst) };

    let x_end = if dst_width - dst_x < blit_width {
        (dst_width - dst_x) + src_x
    } else {
        blit_width + src_x
    };
    let y_end = if dst_height - dst_y < blit_height {
        (src_y + dst_height) - dst_y
    } else {
        src_y + blit_height
    };

    let src_tiles = tiles_per_row(unsafe { width(src) });
    let dst_tiles = tiles_per_row(dst_width);
    let src_pixels = unsafe { pixels(src) };
    let dst_pixels = unsafe { pixels(dst) };
    let keyed = color_key != 0xff;

    let mut loop_src_y = src_y;
    let mut loop_dst_y = dst_y;
    while loop_src_y < y_end {
        // The source byte holds two pixels, so it is only stepped on even
        // source columns; odd columns reuse the byte fetched before them.
        let mut source = unsafe { src_pixels.offset(offset_4bpp(src_x, loop_src_y, src_tiles)) };

        let mut loop_src_x = src_x;
        let mut loop_dst_x = dst_x;
        while loop_src_x < x_end {
            if loop_src_x & 1 != 0 {
                let byte = unsafe { source.read() };
                if !keyed || (byte & 0xf0) != color_key_bits {
                    let destination = unsafe {
                        dst_pixels.offset(offset_8bpp(loop_dst_x, loop_dst_y, dst_tiles))
                    };
                    unsafe { destination.write(pal_offset_bits + (byte >> 4)) };
                }
            } else {
                source =
                    unsafe { src_pixels.offset(offset_4bpp(loop_src_x, loop_src_y, src_tiles)) };
                let byte = unsafe { source.read() };
                if !keyed || (byte & 0xf) != color_key {
                    let destination = unsafe {
                        dst_pixels.offset(offset_8bpp(loop_dst_x, loop_dst_y, dst_tiles))
                    };
                    unsafe { destination.write(pal_offset_bits + (byte & 0xf)) };
                }
            }

            loop_src_x += 1;
            loop_dst_x += 1;
        }

        loop_src_y += 1;
        loop_dst_y += 1;
    }
}

#[unsafe(no_mangle)]
pub unsafe fn FillBitmapRect8Bit(
    surface: *mut u8,
    x: u16,
    y: u16,
    fill_width: u16,
    fill_height: u16,
    fill_value: u8,
) {
    let (x, y) = (i32::from(x), i32::from(y));
    let surface_width = unsafe { width(surface) };
    let surface_height = unsafe { height(surface) };

    let x_end = (x + i32::from(fill_width)).min(surface_width);
    let y_end = (y + i32::from(fill_height)).min(surface_height);

    let tiles = tiles_per_row(surface_width);
    let surface_pixels = unsafe { pixels(surface) };

    let mut loop_y = y;
    while loop_y < y_end {
        let mut loop_x = x;
        while loop_x < x_end {
            unsafe {
                surface_pixels
                    .offset(offset_8bpp(loop_x, loop_y, tiles))
                    .write(fill_value)
            };
            loop_x += 1;
        }
        loop_y += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bitmap_field_offsets_match_the_arm_structure() {
        assert_eq!(BITMAP_PIXELS, 0);
        assert_eq!(BITMAP_WIDTH, 4);
        assert_eq!(BITMAP_HEIGHT, 6);
    }

    #[test]
    fn tiles_per_row_rounds_a_partial_tile_up() {
        assert_eq!(tiles_per_row(8), 1);
        assert_eq!(tiles_per_row(16), 2);
        assert_eq!(tiles_per_row(64), 8);
        // A width of 12 is a tile and a half, which the original rounds by
        // adding the low three bits back in before shifting.
        assert_eq!(tiles_per_row(12), 2);
    }

    #[test]
    fn four_bit_offsets_walk_tiles_then_rows() {
        // Within the first tile: two pixels per byte, four bytes per row.
        assert_eq!(offset_4bpp(0, 0, 8), 0);
        assert_eq!(offset_4bpp(1, 0, 8), 0);
        assert_eq!(offset_4bpp(2, 0, 8), 1);
        assert_eq!(offset_4bpp(0, 1, 8), 4);
        assert_eq!(offset_4bpp(0, 7, 8), 28);
        // The next tile across is 32 bytes on.
        assert_eq!(offset_4bpp(8, 0, 8), 32);
        // The next tile row is a whole row of tiles on.
        assert_eq!(offset_4bpp(0, 8, 8), 8 * 32);
    }

    #[test]
    fn eight_bit_offsets_use_double_width_tiles() {
        assert_eq!(offset_8bpp(0, 0, 8), 0);
        assert_eq!(offset_8bpp(1, 0, 8), 1);
        assert_eq!(offset_8bpp(0, 1, 8), 8);
        assert_eq!(offset_8bpp(8, 0, 8), 64);
        assert_eq!(offset_8bpp(0, 8, 8), 8 * 64);
    }

    #[test]
    fn the_original_shift_chains_reduce_to_the_helpers() {
        // ((u32)(y << 0x1d) >> 0x1b) is (y & 7) * 4.
        for y in 0i32..64 {
            assert_eq!((((y as u32) << 0x1d) >> 0x1b) as i32, (y & 7) << 2);
            assert_eq!((((y as u32) << 0x1d) >> 0x1a) as i32, (y & 7) << 3);
        }
        // (u32)(v << 0x1c) >> 0x18 is (v & 0xf) << 4.
        for v in 0u32..256 {
            assert_eq!(((v << 0x1c) >> 0x18) as u8, ((v as u8) & 0xf) << 4);
        }
    }
}
