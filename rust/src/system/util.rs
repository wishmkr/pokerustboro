use crate::ffi::{
    CpuSet, CreateSprite, DISPLAY_HEIGHT, DISPLAY_WIDTH, SPRITE_CALLBACK_OFFSET, SPRITE_FLAGS_BYTE,
    SPRITE_INVISIBLE_BIT, SpriteCallback, SpriteCallbackDummy, SpriteTemplate, gDummyOamData,
    gDummySpriteAffineAnimTable, gDummySpriteAnimTable, rgb, rgb_blue, rgb_green, rgb_red, sprite,
};
use crate::incbin;
use core::ffi::c_int;

const CPU_SET_32BIT: u32 = 0x0400_0000;

/// `CpuCopy32(src, dest, size)`
#[inline]
unsafe fn cpu_copy32(src: *const u8, dest: *mut u8, size: u32) {
    unsafe {
        CpuSet(
            src.cast(),
            dest.cast(),
            CPU_SET_32BIT | (size / 4 & 0x1f_ffff),
        )
    };
}

#[unsafe(no_mangle)]
pub static gBitTable: crate::c::CArray<u32, 32> = {
    let mut table = [0u32; 32];
    let mut i = 0;
    while i < 32 {
        table[i] = 1u32 << i;
        i += 1;
    }
    crate::c::CArray(table)
};

incbin!(
    gMiscBlank_Gfx,
    "../../../build/assets/graphics/interface/blank.png.4bpp"
);

/// `BgAffineSet` with this module's view of its types.
#[inline]
unsafe fn BgAffineSet(a0: *const BgAffineSrcData, a1: *mut u8, a2: i32) {
    unsafe {
        crate::syscall::BgAffineSet(a0 as _, a1 as _, a2);
    }
}

static INVISIBLE_SPRITE_TEMPLATE: SpriteTemplate = SpriteTemplate {
    tile_tag: 0,
    palette_tag: 0,
    oam: &raw const gDummyOamData,
    anims: (&raw const gDummySpriteAnimTable).cast(),
    images: core::ptr::null(),
    affine_anims: (&raw const gDummySpriteAffineAnimTable).cast(),
    callback: SpriteCallbackDummy,
};

/// `sSpriteDimensions[shape][size]` in tiles: square, horizontal, vertical.
static SPRITE_DIMENSIONS: [[[u8; 2]; 4]; 3] = [
    [[1, 1], [2, 2], [4, 4], [8, 8]],
    [[2, 1], [4, 1], [4, 2], [8, 4]],
    [[1, 2], [1, 4], [2, 4], [4, 8]],
];

/// `struct BgAffineSrcData`, 20 bytes.
#[repr(C, align(4))]
pub struct BgAffineSrcData {
    pub tex_x: u32,
    pub tex_y: u32,
    pub scr_x: i16,
    pub scr_y: i16,
    pub sx: i16,
    pub sy: i16,
    pub alpha: u16,
}

#[unsafe(no_mangle)]
pub unsafe fn CreateInvisibleSpriteWithCallback(callback: SpriteCallback) -> u8 {
    let sprite_id = unsafe {
        CreateSprite(
            &raw const INVISIBLE_SPRITE_TEMPLATE,
            (DISPLAY_WIDTH + 8) as i16,
            (DISPLAY_HEIGHT + 8) as i16,
            14,
        )
    };

    let slot = unsafe { sprite(sprite_id as usize) };
    let flags = unsafe { slot.add(SPRITE_FLAGS_BYTE) };
    unsafe { flags.write_volatile(flags.read_volatile() | SPRITE_INVISIBLE_BIT) };
    unsafe {
        slot.add(SPRITE_CALLBACK_OFFSET)
            .cast::<SpriteCallback>()
            .write(callback)
    };

    sprite_id
}

#[unsafe(no_mangle)]
pub unsafe fn StoreWordInTwoHalfwords(halfwords: *mut u16, word: u32) {
    unsafe { halfwords.write(word as u16) };
    unsafe { halfwords.add(1).write((word >> 16) as u16) };
}

#[unsafe(no_mangle)]
pub unsafe fn LoadWordFromTwoHalfwords(halfwords: *mut u16, word: *mut u32) {
    // The high half is read back signed, matching the original cast.
    let low = u32::from(unsafe { halfwords.read() });
    let high = (i32::from(unsafe { halfwords.add(1).read() } as i16) << 16) as u32;
    unsafe { word.write(low | high) };
}

#[unsafe(no_mangle)]
pub unsafe fn SetBgAffineStruct(
    src: *mut BgAffineSrcData,
    tex_x: u32,
    tex_y: u32,
    scr_x: i16,
    scr_y: i16,
    sx: i16,
    sy: i16,
    alpha: u16,
) {
    unsafe {
        (&raw mut (*src).tex_x).write(tex_x);
        (&raw mut (*src).tex_y).write(tex_y);
        (&raw mut (*src).scr_x).write(scr_x);
        (&raw mut (*src).scr_y).write(scr_y);
        (&raw mut (*src).sx).write(sx);
        (&raw mut (*src).sy).write(sy);
        (&raw mut (*src).alpha).write(alpha);
    }
}

#[unsafe(no_mangle)]
pub unsafe fn DoBgAffineSet(
    dest: *mut u8,
    tex_x: u32,
    tex_y: u32,
    scr_x: i16,
    scr_y: i16,
    sx: i16,
    sy: i16,
    alpha: u16,
) {
    let mut src = BgAffineSrcData {
        tex_x: 0,
        tex_y: 0,
        scr_x: 0,
        scr_y: 0,
        sx: 0,
        sy: 0,
        alpha: 0,
    };
    unsafe { SetBgAffineStruct(&raw mut src, tex_x, tex_y, scr_x, scr_y, sx, sy, alpha) };
    unsafe { BgAffineSet(&raw const src, dest, 1) };
}

#[unsafe(no_mangle)]
pub unsafe fn CopySpriteTiles(
    shape: u8,
    size: u8,
    tiles: *mut u8,
    tilemap: *mut u16,
    output: *mut u8,
) {
    let dimensions = SPRITE_DIMENSIONS[(shape & 3).min(2) as usize][(size & 3) as usize];
    let width = dimensions[0] as usize;
    let height = dimensions[1] as usize;

    // CpuCopy32 moves words, so the scratch buffer must be word-aligned.
    #[repr(C, align(4))]
    struct Aligned32([u8; 32]);
    let mut xflip = Aligned32([0u8; 32]);
    let mut tilemap = tilemap;
    let mut output = output;

    let mut y = 0usize;
    while y < height {
        let mut x = 0usize;
        while x < width {
            let entry = unsafe { tilemap.read() };
            let tile = (entry & 0x3ff) as usize * 32;

            match entry & 0xc00 {
                0 => unsafe { cpu_copy32(tiles.add(tile), output, 32) },
                0x800 => {
                    // Vertical flip only: copy the eight rows backwards.
                    let mut i = 0usize;
                    while i < 8 {
                        unsafe { cpu_copy32(tiles.add(tile + (7 - i) * 4), output.add(i * 4), 4) };
                        i += 1;
                    }
                }
                _ => {
                    // Horizontal flip: reverse the nibble pairs of every row.
                    let mut i = 0usize;
                    while i < 8 {
                        let mut j = 0usize;
                        while j < 4 {
                            let row = i * 4;
                            let byte = unsafe { tiles.add(tile + row + j).read() };
                            xflip.0[row + (3 - j)] = ((byte & 0xf) << 4) | (byte >> 4);
                            j += 1;
                        }
                        i += 1;
                    }

                    if entry & 0x800 != 0 {
                        let mut i = 0usize;
                        while i < 8 {
                            unsafe {
                                cpu_copy32(xflip.0.as_ptr().add((7 - i) * 4), output.add(i * 4), 4)
                            };
                            i += 1;
                        }
                    } else {
                        unsafe { cpu_copy32(xflip.0.as_ptr(), output, 32) };
                    }
                }
            }

            tilemap = unsafe { tilemap.add(1) };
            output = unsafe { output.add(32) };
            x += 1;
        }
        tilemap = unsafe { tilemap.add(32 - width) };
        y += 1;
    }
}

#[unsafe(no_mangle)]
pub unsafe fn CountTrailingZeroBits(value: u32) -> c_int {
    let mut value = value;
    let mut i = 0i32;
    while i < 32 {
        if value & 1 == 0 {
            value >>= 1;
        } else {
            return i;
        }
        i += 1;
    }
    0
}

#[unsafe(no_mangle)]
pub unsafe fn CalcCRC16(data: *const u8, length: i32) -> u16 {
    let mut crc = 0x1121u16;
    let mut i = 0i32;
    while i < length {
        crc ^= u16::from(unsafe { data.offset(i as isize).read() });
        let mut j = 0;
        while j < 8 {
            crc = if crc & 1 != 0 {
                (crc >> 1) ^ 0x8408
            } else {
                crc >> 1
            };
            j += 1;
        }
        i += 1;
    }
    !crc
}

#[unsafe(no_mangle)]
pub unsafe fn CalcCRC16WithTable(data: *const u8, length: u32) -> u16 {
    let mut crc = 0x1121u16;
    let mut i = 0u32;
    while i < length {
        let byte = crc >> 8;
        crc ^= u16::from(unsafe { data.add(i as usize).read() });
        crc = byte ^ CRC16_TABLE[(crc & 0xff) as usize];
        i += 1;
    }
    !crc
}

#[unsafe(no_mangle)]
pub unsafe fn CalcByteArraySum(data: *const u8, length: u32) -> u32 {
    let mut sum = 0u32;
    let mut i = 0u32;
    while i < length {
        sum = sum.wrapping_add(u32::from(unsafe { data.add(i as usize).read() }));
        i += 1;
    }
    sum
}

#[unsafe(no_mangle)]
pub unsafe fn BlendPalette(pal_offset: u16, num_entries: u16, coeff: u8, blend_color: u16) {
    let target_r = rgb_red(blend_color);
    let target_g = rgb_green(blend_color);
    let target_b = rgb_blue(blend_color);

    let mut i = 0u16;
    while i < num_entries {
        let index = (i + pal_offset) as usize;
        let color = unsafe {
            (&raw const (*(&raw const crate::palette::gPlttBufferUnfaded)
                .cast::<[u16; crate::ffi::PLTT_BUFFER_SIZE]>()
                .cast_mut()))
                .cast::<u16>()
                .add(index)
                .read_volatile()
        };
        let r = rgb_red(color);
        let g = rgb_green(color);
        let b = rgb_blue(color);
        let blended = rgb(
            r + (((target_r - r) * i32::from(coeff)) >> 4),
            g + (((target_g - g) * i32::from(coeff)) >> 4),
            b + (((target_b - b) * i32::from(coeff)) >> 4),
        );
        unsafe {
            (&raw mut (*(&raw const crate::palette::gPlttBufferFaded)
                .cast::<[u16; crate::ffi::PLTT_BUFFER_SIZE]>()
                .cast_mut()))
                .cast::<u16>()
                .add(index)
                .write_volatile(blended)
        };
        i += 1;
    }
}

/// The reflected CRC-16 table (polynomial 0x8408) used by the link code.
/// Generating it reproduces the literal table in the original source exactly.
static CRC16_TABLE: [u16; 256] = {
    let mut table = [0u16; 256];
    let mut i = 0usize;
    while i < 256 {
        let mut crc = i as u16;
        let mut bit = 0;
        while bit < 8 {
            crc = if crc & 1 != 0 {
                (crc >> 1) ^ 0x8408
            } else {
                crc >> 1
            };
            bit += 1;
        }
        table[i] = crc;
        i += 1;
    }
    table
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_bit_table_is_a_power_of_two_ladder() {
        assert_eq!(gBitTable[0], 1);
        assert_eq!(gBitTable[31], 0x8000_0000);
        for i in 0..32 {
            assert_eq!(gBitTable[i], 1u32 << i);
        }
    }

    #[test]
    fn sprite_dimensions_cover_every_oam_shape_and_size() {
        assert_eq!(SPRITE_DIMENSIONS[0][3], [8, 8]);
        assert_eq!(SPRITE_DIMENSIONS[1][0], [2, 1]);
        assert_eq!(SPRITE_DIMENSIONS[2][3], [4, 8]);
    }

    #[test]
    fn trailing_zero_count_matches_the_original_loop() {
        assert_eq!(unsafe { CountTrailingZeroBits(1) }, 0);
        assert_eq!(unsafe { CountTrailingZeroBits(2) }, 1);
        assert_eq!(unsafe { CountTrailingZeroBits(0x8000_0000) }, 31);
        // The original returns 0 for a value with no set bits.
        assert_eq!(unsafe { CountTrailingZeroBits(0) }, 0);
    }

    #[test]
    fn the_crc_table_matches_the_literal_table_in_the_source() {
        assert_eq!(CRC16_TABLE[0], 0x0000);
        assert_eq!(CRC16_TABLE[1], 0x1189);
        assert_eq!(CRC16_TABLE[2], 0x2312);
        assert_eq!(CRC16_TABLE[3], 0x329b);
        assert_eq!(CRC16_TABLE[8], 0x8c48);
        assert_eq!(CRC16_TABLE[16], 0x1081);
        assert_eq!(CRC16_TABLE[255], 0x0f78);
    }

    #[test]
    fn both_crc_implementations_agree() {
        let data: [u8; 16] = [
            0x00, 0x01, 0x02, 0x7f, 0x80, 0xff, 0x10, 0x20, 0x30, 0x40, 0x50, 0x60, 0x70, 0x80,
            0x90, 0xa0,
        ];
        for length in 0..=data.len() {
            let plain = unsafe { CalcCRC16(data.as_ptr(), length as i32) };
            let tabled = unsafe { CalcCRC16WithTable(data.as_ptr(), length as u32) };
            assert_eq!(plain, tabled, "length {length}");
        }
    }

    #[test]
    fn rgb_round_trips_through_the_channel_helpers() {
        let color = rgb(31, 15, 7);
        assert_eq!(rgb_red(color), 31);
        assert_eq!(rgb_green(color), 15);
        assert_eq!(rgb_blue(color), 7);
        assert_eq!(rgb(31, 0, 0), 0x001f);
        assert_eq!(rgb(0, 31, 0), 0x03e0);
        assert_eq!(rgb(0, 0, 31), 0x7c00);
    }
}
