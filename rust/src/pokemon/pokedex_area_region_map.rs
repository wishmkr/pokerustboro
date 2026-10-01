//! The Hoenn map background shown behind the Pokédex area screen.

use core::ffi::c_void;

use crate::bg::{ChangeBgX, ChangeBgY, SetBgAttribute, ShowBg};
use crate::ffi::CpuSet;
use crate::malloc::{Alloc, Free};

const BG_ATTR_PALETTEMODE: u8 = 4;
const BG_ATTR_METRIC: u8 = 8;
const BG_ATTR_TYPE: u8 = 9;
const BG_TYPE_AFFINE: u8 = 1;
const BG_COORD_SET: u8 = 0;
const CPU_SET_32BIT: u32 = 0x0400_0000;
/// `BG_PLTT_ID(7)`
const MAP_PALETTE_INDEX: usize = 7 * 16;

#[unsafe(link_section = "ewram_data")]
static mut BG_NUM: *mut u8 = core::ptr::null_mut();

crate::incbin!(
    sPokedexAreaMap_Pal,
    "../../../build/assets/graphics/pokedex/region_map.pal.gbapal"
);
crate::incbin!(
    sPokedexAreaMap_Gfx,
    "../../../build/assets/graphics/pokedex/region_map.png_num_tiles_232__Wnum_tiles.8bpp.lz"
);
crate::incbin!(
    sPokedexAreaMap_Tilemap,
    "../../../build/assets/graphics/pokedex/region_map.bin.lz"
);
crate::incbin!(
    sPokedexAreaMapAffine_Gfx,
    "../../../build/assets/graphics/pokedex/region_map_affine.png_num_tiles_233__Wnum_tiles.8bpp.lz"
);
crate::incbin!(
    sPokedexAreaMapAffine_Tilemap,
    "../../../build/assets/graphics/pokedex/region_map_affine.bin.lz"
);

/// `DecompressAndCopyTileDataToVram` with this module's view of its types.
#[inline]
unsafe fn DecompressAndCopyTileDataToVram(
    a0: u8,
    a1: *const c_void,
    a2: u32,
    a3: u16,
    a4: u8,
) -> *mut c_void {
    unsafe { crate::menu::DecompressAndCopyTileDataToVram(a0, a1 as _, a2, a3, a4) as *mut c_void }
}
/// `FreeTempTileDataBuffersIfPossible` with this module's view of its types.
#[inline]
unsafe fn FreeTempTileDataBuffersIfPossible() -> u8 {
    unsafe { crate::menu::FreeTempTileDataBuffersIfPossible() }
}
/// `AddValToTilemapBuffer` with this module's view of its types.
#[inline]
unsafe fn AddValToTilemapBuffer(a0: *mut c_void, a1: i32, a2: i32, a3: i32, a4: u32) {
    unsafe {
        crate::menu::AddValToTilemapBuffer(a0 as _, a1, a2, a3, a4);
    }
}

/// `struct PokedexAreaMapTemplate { u32 bg:2; u32 offset:8; u32 mode:2; ... }`
#[inline]
fn template_fields(bits: u32) -> (u8, u8, u8) {
    (
        (bits & 3) as u8,
        ((bits >> 2) & 0xff) as u8,
        ((bits >> 10) & 3) as u8,
    )
}

#[unsafe(no_mangle)]
pub unsafe fn LoadPokedexAreaMapGfx(template: *const u32) {
    let (bg, offset, mode) = template_fields(unsafe { template.read() });
    // The C code allocates sizeof(u8 *) for what it uses as a u8.
    let bg_num = unsafe { Alloc(4) };
    unsafe { (&raw mut BG_NUM).write(bg_num) };

    let (metric, gfx, tilemap, size, affine) = if mode == 0 {
        (
            0,
            sPokedexAreaMap_Gfx.as_ptr(),
            sPokedexAreaMap_Tilemap.as_ptr(),
            32,
            0,
        )
    } else {
        // Never reached: every caller passes mode 0.
        (
            2,
            sPokedexAreaMapAffine_Gfx.as_ptr(),
            sPokedexAreaMapAffine_Tilemap.as_ptr(),
            64,
            1,
        )
    };
    unsafe { SetBgAttribute(bg, BG_ATTR_METRIC, metric) };
    if mode != 0 {
        // Has no effect: BG_ATTR_TYPE can't be set this way.
        unsafe { SetBgAttribute(bg, BG_ATTR_TYPE, BG_TYPE_AFFINE) };
    }
    unsafe { DecompressAndCopyTileDataToVram(bg, gfx.cast(), 0, u16::from(offset), 0) };
    let buffer = unsafe { DecompressAndCopyTileDataToVram(bg, tilemap.cast(), 0, 0, 1) };
    unsafe { AddValToTilemapBuffer(buffer, i32::from(offset), size, size, affine) };

    unsafe { ChangeBgX(bg, 0, BG_COORD_SET) };
    unsafe { ChangeBgY(bg, 0, BG_COORD_SET) };
    unsafe { SetBgAttribute(bg, BG_ATTR_PALETTEMODE, 1) };
    let words = (sPokedexAreaMap_Pal.0.len() / 4) as u32;
    let dest = unsafe {
        (&raw mut (*(&raw const crate::palette::gPlttBufferUnfaded)
            .cast::<[u16; crate::ffi::PLTT_BUFFER_SIZE]>()
            .cast_mut()))
            .cast::<u16>()
            .add(MAP_PALETTE_INDEX)
    };
    unsafe {
        CpuSet(
            sPokedexAreaMap_Pal.as_ptr().cast(),
            dest.cast(),
            CPU_SET_32BIT | words,
        )
    };
    unsafe { bg_num.write(bg) };
}

#[unsafe(no_mangle)]
pub unsafe fn TryShowPokedexAreaMap() -> u32 {
    if unsafe { FreeTempTileDataBuffersIfPossible() } == 0 {
        let bg = unsafe { (&raw const BG_NUM).read().read() };
        unsafe { ShowBg(bg) };
        0
    } else {
        1
    }
}

#[unsafe(no_mangle)]
pub unsafe fn FreePokedexAreaMapBgNum() {
    let bg_num = unsafe { (&raw const BG_NUM).read() };
    if !bg_num.is_null() {
        unsafe { Free(bg_num) };
        unsafe { (&raw mut BG_NUM).write(core::ptr::null_mut()) };
    }
}

#[unsafe(no_mangle)]
pub unsafe fn PokedexAreaMapChangeBgY(move_: u32) {
    let bg = unsafe { (&raw const BG_NUM).read().read() };
    unsafe { ChangeBgY(bg, move_.wrapping_mul(0x100) as i32, BG_COORD_SET) };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn template_bitfields_unpack_like_gcc() {
        // bg 2, offset 0x5a, mode 1, unk all ones
        let bits = 2 | (0x5a << 2) | (1 << 10) | (0xfffff << 12);
        assert_eq!(template_fields(bits), (2, 0x5a, 1));
    }
}
