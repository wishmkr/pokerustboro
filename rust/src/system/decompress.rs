//! LZ77 decompression helpers (the BIOS does the work) and the Pokémon
//! picture loaders that pick Unown forms, the "?" picture and Deoxys' forms.

use crate::ffi::{Align4, CompressedSpritePalette, CompressedSpriteSheet, CpuSet};
use crate::malloc::{AllocZeroed, Free};
use crate::sprite::{LoadSpritePalette, LoadSpriteSheet};

const NUM_SPECIES: i32 = 0x19c;
const SPECIES_UNOWN: i32 = 0xc9;
const SPECIES_UNOWN_B: u16 = 0x19d;
const SPECIES_DEOXYS: i32 = 0x19a;
const NUM_UNOWN_FORMS: u32 = 28;
const MON_PIC_SIZE: u32 = 0x800;
const SHEET_SIZE: usize = 8;

#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gDecompressionBuffer: Align4<[u8; 0x4000]> = Align4([0; 0x4000]);

/// `LZ77UnCompWram` with this module's view of its types.
#[inline]
unsafe fn LZ77UnCompWram(a0: *const u32, a1: *mut core::ffi::c_void) {
    unsafe {
        crate::syscall::LZ77UnCompWram(a0 as _, a1 as _);
    }
}
/// `LZ77UnCompVram` with this module's view of its types.
#[inline]
unsafe fn LZ77UnCompVram(a0: *const u32, a1: *mut core::ffi::c_void) {
    unsafe {
        crate::syscall::LZ77UnCompVram(a0 as _, a1 as _);
    }
}
/// `DrawSpindaSpots` with this module's view of its types.
#[inline]
unsafe fn DrawSpindaSpots(a0: u16, a1: u32, a2: *mut u8, a3: u8) {
    unsafe {
        crate::pokemon::DrawSpindaSpots(a0, a1, a2 as _, a3);
    }
}

/// `struct SpriteSheet { const void *data; u16 size; u16 tag; }`
#[repr(C, align(4))]
struct SpriteSheet {
    data: *const u8,
    size: u16,
    tag: u16,
}

/// `struct SpritePalette { const u16 *data; u16 tag; }`
#[repr(C, align(4))]
struct SpritePalette {
    data: *const u8,
    tag: u16,
}

#[inline]
fn buffer() -> *mut u8 {
    (&raw mut gDecompressionBuffer).cast()
}

#[inline]
unsafe fn table_entry(
    table: *const CompressedSpriteSheet,
    index: usize,
) -> *const CompressedSpriteSheet {
    table.cast::<u8>().wrapping_add(index * SHEET_SIZE).cast()
}

#[unsafe(no_mangle)]
pub unsafe fn LZDecompressWram(src: *const u32, dest: *mut u8) {
    unsafe { LZ77UnCompWram(src, dest.cast()) };
}

#[unsafe(no_mangle)]
pub unsafe fn LZDecompressVram(src: *const u32, dest: *mut u8) {
    unsafe { LZ77UnCompVram(src, dest.cast()) };
}

unsafe fn load_sheet(src: *const CompressedSpriteSheet, into: *mut u8) -> u16 {
    unsafe { LZ77UnCompWram((*src).data, into.cast()) };
    let sheet = SpriteSheet {
        data: into,
        size: unsafe { (*src).size },
        tag: unsafe { (*src).tag },
    };
    unsafe { LoadSpriteSheet((&raw const sheet).cast()) }
}

unsafe fn load_palette(src: *const CompressedSpritePalette, into: *mut u8) {
    unsafe { LZ77UnCompWram((*src).data, into.cast()) };
    let palette = SpritePalette {
        data: into,
        tag: unsafe { (*src).tag },
    };
    unsafe { LoadSpritePalette((&raw const palette).cast()) };
}

#[unsafe(no_mangle)]
pub unsafe fn LoadCompressedSpriteSheet(src: *const CompressedSpriteSheet) -> u16 {
    unsafe { load_sheet(src, buffer()) }
}

#[unsafe(no_mangle)]
pub unsafe fn LoadCompressedSpriteSheetOverrideBuffer(
    src: *const CompressedSpriteSheet,
    into: *mut u8,
) {
    unsafe { load_sheet(src, into) };
}

#[unsafe(no_mangle)]
pub unsafe fn LoadCompressedSpritePalette(src: *const CompressedSpritePalette) {
    unsafe { load_palette(src, buffer()) };
}

#[unsafe(no_mangle)]
pub unsafe fn LoadCompressedSpritePaletteOverrideBuffer(
    src: *const CompressedSpritePalette,
    into: *mut u8,
) {
    unsafe { load_palette(src, into) };
}

unsafe fn decompress_pic(src: *const CompressedSpriteSheet, into: *mut u8, species: i32) {
    let src = if species > NUM_SPECIES {
        &raw const (*(&raw const crate::data::data_tables::gMonFrontPicTable)
            .cast::<CompressedSpriteSheet>())
    } else {
        src
    };
    unsafe { LZ77UnCompWram((*src).data, into.cast()) };
}

#[unsafe(no_mangle)]
pub unsafe fn DecompressPicFromTable(
    src: *const CompressedSpriteSheet,
    into: *mut u8,
    species: i32,
) {
    unsafe { decompress_pic(src, into, species) };
    unsafe { duplicate_deoxys_tiles(into, species) };
}

#[unsafe(no_mangle)]
pub unsafe fn DecompressPicFromTable_2(
    src: *const CompressedSpriteSheet,
    into: *mut u8,
    species: i32,
) {
    unsafe { DecompressPicFromTable(src, into, species) };
}

#[unsafe(no_mangle)]
pub unsafe fn DecompressPicFromTable_DontHandleDeoxys(
    src: *const CompressedSpriteSheet,
    into: *mut u8,
    species: i32,
) {
    unsafe { decompress_pic(src, into, species) };
}

unsafe fn is_front_pic(src: *const CompressedSpriteSheet, species: i32) -> u8 {
    let front = unsafe {
        table_entry(
            &raw const (*(&raw const crate::data::data_tables::gMonFrontPicTable)
                .cast::<CompressedSpriteSheet>()),
            species as usize,
        )
    };
    u8::from(core::ptr::eq(src, front))
}

/// Decompresses a species' picture, handling Unown's letter forms and the
/// "?" picture for out-of-range species.
unsafe fn load_special_pic(
    src: *const CompressedSpriteSheet,
    dest: *mut u8,
    species: i32,
    personality: u32,
    is_front_pic: u8,
) {
    let src = if species == SPECIES_UNOWN {
        let letter = (((personality & 0x0300_0000) >> 18)
            | ((personality & 0x0003_0000) >> 12)
            | ((personality & 0x0000_0300) >> 6)
            | (personality & 0x0000_0003))
            % NUM_UNOWN_FORMS;
        // Unown A is the base species; the other letters are stored apart.
        let index = if letter == 0 {
            SPECIES_UNOWN as u16
        } else {
            (letter as u16).wrapping_add(SPECIES_UNOWN_B - 1)
        };
        let table = if is_front_pic == 0 {
            &raw const (*(&raw const crate::data::data_tables::gMonBackPicTable)
                .cast::<CompressedSpriteSheet>())
        } else {
            &raw const (*(&raw const crate::data::data_tables::gMonFrontPicTable)
                .cast::<CompressedSpriteSheet>())
        };
        unsafe { table_entry(table, usize::from(index)) }
    } else if species > NUM_SPECIES {
        &raw const (*(&raw const crate::data::data_tables::gMonFrontPicTable)
            .cast::<CompressedSpriteSheet>())
    } else {
        src
    };
    unsafe { LZ77UnCompWram((*src).data, dest.cast()) };
}

#[unsafe(no_mangle)]
pub unsafe fn LoadSpecialPokePic(
    src: *const CompressedSpriteSheet,
    dest: *mut u8,
    species: i32,
    personality: u32,
    is_front_pic: u8,
) {
    unsafe { load_special_pic(src, dest, species, personality, is_front_pic) };
    unsafe { duplicate_deoxys_tiles(dest, species) };
    unsafe { DrawSpindaSpots(species as u16, personality, dest, is_front_pic) };
}

#[unsafe(no_mangle)]
pub unsafe fn LoadSpecialPokePic_2(
    src: *const CompressedSpriteSheet,
    dest: *mut u8,
    species: i32,
    personality: u32,
    is_front_pic: u8,
) {
    unsafe { LoadSpecialPokePic(src, dest, species, personality, is_front_pic) };
}

#[unsafe(no_mangle)]
pub unsafe fn LoadSpecialPokePic_DontHandleDeoxys(
    src: *const CompressedSpriteSheet,
    dest: *mut u8,
    species: i32,
    personality: u32,
    is_front_pic: u8,
) {
    unsafe { load_special_pic(src, dest, species, personality, is_front_pic) };
    unsafe { DrawSpindaSpots(species as u16, personality, dest, is_front_pic) };
}

#[unsafe(no_mangle)]
pub unsafe fn HandleLoadSpecialPokePic(
    src: *const CompressedSpriteSheet,
    dest: *mut u8,
    species: i32,
    personality: u32,
) {
    // The _2 variant is what the original calls here.
    unsafe { LoadSpecialPokePic_2(src, dest, species, personality, is_front_pic(src, species)) };
}

#[unsafe(no_mangle)]
pub unsafe fn HandleLoadSpecialPokePic_2(
    src: *const CompressedSpriteSheet,
    dest: *mut u8,
    species: i32,
    personality: u32,
) {
    unsafe { LoadSpecialPokePic_2(src, dest, species, personality, is_front_pic(src, species)) };
}

#[unsafe(no_mangle)]
pub unsafe fn HandleLoadSpecialPokePic_DontHandleDeoxys(
    src: *const CompressedSpriteSheet,
    dest: *mut u8,
    species: i32,
    personality: u32,
) {
    unsafe {
        LoadSpecialPokePic_DontHandleDeoxys(
            src,
            dest,
            species,
            personality,
            is_front_pic(src, species),
        )
    };
}

#[unsafe(no_mangle)]
pub unsafe fn Unused_LZDecompressWramIndirect(src: *const *const u32, dest: *mut u8) {
    unsafe { LZ77UnCompWram(src.read(), dest.cast()) };
}

#[unsafe(no_mangle)]
pub unsafe fn GetDecompressedDataSize(ptr: *const u32) -> u32 {
    let bytes = ptr.cast::<u8>();
    unsafe {
        (u32::from(bytes.add(3).read()) << 16)
            | (u32::from(bytes.add(2).read()) << 8)
            | u32::from(bytes.add(1).read())
    }
}

#[unsafe(no_mangle)]
pub unsafe fn LoadCompressedSpriteSheetUsingHeap(src: *const CompressedSpriteSheet) -> u8 {
    let size = unsafe { (*src).data.read() } >> 8;
    let heap = unsafe { AllocZeroed(size) };
    unsafe { load_sheet(src, heap) };
    unsafe { Free(heap) };
    0
}

#[unsafe(no_mangle)]
pub unsafe fn LoadCompressedSpritePaletteUsingHeap(src: *const CompressedSpritePalette) -> u8 {
    let size = unsafe { (*src).data.read() } >> 8;
    let heap = unsafe { AllocZeroed(size) };
    unsafe { load_palette(src, heap) };
    unsafe { Free(heap) };
    0
}

/// Deoxys' normal form is the second half of its picture.
unsafe fn duplicate_deoxys_tiles(pointer: *mut u8, species: i32) {
    if species == SPECIES_DEOXYS {
        const CPU_SET_32BIT: u32 = 0x0400_0000;
        unsafe {
            CpuSet(
                pointer.add(MON_PIC_SIZE as usize).cast(),
                pointer.cast(),
                CPU_SET_32BIT | (MON_PIC_SIZE / 4),
            )
        };
    }
}
