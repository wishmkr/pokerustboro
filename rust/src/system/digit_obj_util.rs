//! Translated from `src/digit_obj_util.c` by tools/rustport/c2rs.py.
#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    static_mut_refs,
    unsafe_op_in_unsafe_fn,
    clashing_extern_declarations,
    unpredictable_function_pointer_comparisons,
    dangerous_implicit_autorefs,
    overflowing_literals,
    unused_assignments,
    unused_variables
)]

use crate::agb_main::gMain;
#[allow(unused_imports)]
use crate::c::*;
#[allow(unused_imports)]
use crate::consts::*;
use crate::decompress::GetDecompressedDataSize;
use crate::sprite::{
    FreeSpritePaletteByTag, FreeSpriteTilesByTag, GetSpriteTileStartByTag, IndexOfSpritePaletteTag,
};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use core::ffi::c_void;
#[allow(unused_imports)]
use core::mem::zeroed;
#[allow(unused_imports)]
use core::ptr::null_mut;
/// `Free` with this module's view of its types.
#[inline]
unsafe fn Free(a0: *mut c_void) {
    unsafe {
        crate::malloc::Free(a0 as _);
    }
}
/// `LoadCompressedSpriteSheet` with this module's view of its types.
#[inline]
unsafe fn LoadCompressedSpriteSheet(a0: *mut CompressedSpriteSheet) -> u16 {
    unsafe { crate::decompress::LoadCompressedSpriteSheet(a0 as _) }
}
/// `LoadSpritePalette` with this module's view of its types.
#[inline]
unsafe fn LoadSpritePalette(a0: *mut SpritePalette) -> u8 {
    unsafe { crate::sprite::LoadSpritePalette(a0 as _) }
}
/// `LoadSpriteSheet` with this module's view of its types.
#[inline]
unsafe fn LoadSpriteSheet(a0: *mut SpriteSheet) -> u16 {
    unsafe { crate::sprite::LoadSpriteSheet(a0 as _) }
}
// Data tables (translate with cdata.py): sTilesPerImage

/// `struct DigitPrinterAlloc`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DigitPrinterAlloc {
    pub count: u32,
    pub array: *mut DigitPrinter,
}

unsafe impl Sync for DigitPrinterAlloc {}

/// `struct DigitPrinter`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DigitPrinter {
    pub isActive: u8,
    pub firstOamId: u8,
    pub strConvMode: u8,
    pub oamCount: u8,
    pub palTagIndex: u8,
    pub size: u8,
    pub shape: u8,
    pub priority: u8,
    pub xDelta: u8,
    pub tilesPerImage: u8,
    pub tileStart: u16,
    pub x: i16,
    pub y: i16,
    pub tileTag: u16,
    pub palTag: u16,
    pub pow10: u32,
    pub lastPrinted: i32,
}

unsafe impl Sync for DigitPrinter {}

#[cfg(target_arch = "arm")]
const _: () = {
    #[allow(unused_imports)]
    use core::mem::{offset_of, size_of};
    assert!(size_of::<DigitPrinterAlloc>() == 8);
    assert!(offset_of!(DigitPrinterAlloc, count) == 0);
    assert!(offset_of!(DigitPrinterAlloc, array) == 4);
    assert!(size_of::<DigitPrinter>() == 28);
    assert!(offset_of!(DigitPrinter, isActive) == 0);
    assert!(offset_of!(DigitPrinter, firstOamId) == 1);
    assert!(offset_of!(DigitPrinter, strConvMode) == 2);
    assert!(offset_of!(DigitPrinter, oamCount) == 3);
    assert!(offset_of!(DigitPrinter, palTagIndex) == 4);
    assert!(offset_of!(DigitPrinter, size) == 5);
    assert!(offset_of!(DigitPrinter, shape) == 6);
    assert!(offset_of!(DigitPrinter, priority) == 7);
    assert!(offset_of!(DigitPrinter, xDelta) == 8);
    assert!(offset_of!(DigitPrinter, tilesPerImage) == 9);
    assert!(offset_of!(DigitPrinter, tileStart) == 10);
    assert!(offset_of!(DigitPrinter, x) == 12);
    assert!(offset_of!(DigitPrinter, y) == 14);
    assert!(offset_of!(DigitPrinter, tileTag) == 16);
    assert!(offset_of!(DigitPrinter, palTag) == 18);
    assert!(offset_of!(DigitPrinter, pow10) == 20);
    assert!(offset_of!(DigitPrinter, lastPrinted) == 24);
};

static sTilesPerImage: Table<CArray<CArray<u8, 4>, 4>> =
    Table((&raw const crate::data::digit_obj_util::sTilesPerImage).cast());

#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sOamWork: *mut DigitPrinterAlloc = null_mut();
static DrawNumObjsMinusInFront_oamId: crate::global::Global<i32> = crate::global::Global::new(0);
static DrawNumObjsMinusInFront_curDigit: crate::global::Global<i32> = crate::global::Global::new(0);
static DrawNumObjsMinusInFront_firstDigit: crate::global::Global<i32> =
    crate::global::Global::new(0);

/// `Alloc` with this module's view of its types.
#[inline]
unsafe fn Alloc(a0: u32) -> *mut c_void {
    unsafe { crate::malloc::Alloc(a0) as *mut c_void }
}
/// `CpuSet` with this module's view of its types.
#[inline]
unsafe fn CpuSet(a0: *mut c_void, a1: *mut c_void, a2: u32) {
    unsafe {
        crate::syscall::CpuSet(a0 as _, a1 as _, a2);
    }
}

pub unsafe fn DigitObjUtil_Init(count: u32) -> u32 {
    if !sOamWork.is_null() {
        DigitObjUtil_Free();
    }
    sOamWork = Alloc(8) as *mut DigitPrinterAlloc;
    if sOamWork.is_null() {
        return FALSE as u32;
    }
    (*sOamWork).array = Alloc(28 * count) as *mut DigitPrinter;
    if (*sOamWork).array.is_null() {
        Free(sOamWork as *mut c_void);
        return FALSE as u32;
    }
    (*sOamWork).count = count;
    for i in 0..count {
        (*(*sOamWork).array.at(i)).isActive = FALSE;
        (*(*sOamWork).array.at(i)).firstOamId = 0xFF;
    }
    TRUE as u32
}
pub unsafe fn DigitObjUtil_Free() {
    if !sOamWork.is_null() {
        if !(*sOamWork).array.is_null() {
            let mut i: u32 = 0;
            while i < (*sOamWork).count {
                DigitObjUtil_DeletePrinter(i);
                i += 1;
            }
            Free((*sOamWork).array as *mut c_void);
        }
        Free(sOamWork as *mut c_void);
        sOamWork = null_mut();
    }
}
pub unsafe fn DigitObjUtil_CreatePrinter(
    id: u32,
    num: i32,
    template: *mut DigitObjUtilTemplate,
) -> u32 {
    if sOamWork.is_null() {
        return FALSE as u32;
    }
    if (*(*sOamWork).array.at(id)).isActive != 0 {
        return FALSE as u32;
    }
    (*(*sOamWork).array.at(id)).firstOamId = GetFirstOamId((*template).oamCount);
    if (*(*sOamWork).array.at(id)).firstOamId == 0xFF {
        return FALSE as u32;
    }
    (*(*sOamWork).array.at(id)).tileStart = GetSpriteTileStartByTag((*(*template).spriteSheet).tag);
    if (*(*sOamWork).array.at(id)).tileStart == 0xFFFF {
        if (*(*template).spriteSheet).size != 0 {
            (*(*sOamWork).array.at(id)).tileStart = LoadSpriteSheet((*template).spriteSheet);
        } else {
            let mut compSpriteSheet: CompressedSpriteSheet =
                *((*template).spriteSheet as *mut CompressedSpriteSheet);
            compSpriteSheet.size =
                GetDecompressedDataSize((*(*template).spriteSheet).data as *mut u32) as u16;
            (*(*sOamWork).array.at(id)).tileStart =
                LoadCompressedSpriteSheet(&raw mut compSpriteSheet);
        }
        if (*(*sOamWork).array.at(id)).tileStart == 0xFFFF {
            return FALSE as u32;
        }
    }
    (*(*sOamWork).array.at(id)).palTagIndex = IndexOfSpritePaletteTag((*(*template).spritePal).tag);
    if (*(*sOamWork).array.at(id)).palTagIndex == 0xFF {
        (*(*sOamWork).array.at(id)).palTagIndex = LoadSpritePalette((*template).spritePal);
    }
    (*(*sOamWork).array.at(id)).strConvMode = (*template).strConvMode();
    (*(*sOamWork).array.at(id)).oamCount = (*template).oamCount;
    (*(*sOamWork).array.at(id)).x = (*template).x;
    (*(*sOamWork).array.at(id)).y = (*template).y;
    (*(*sOamWork).array.at(id)).shape = (*template).shape();
    (*(*sOamWork).array.at(id)).size = (*template).size();
    (*(*sOamWork).array.at(id)).priority = (*template).priority();
    (*(*sOamWork).array.at(id)).xDelta = (*template).xDelta;
    (*(*sOamWork).array.at(id)).tilesPerImage =
        GetTilesPerImage((*template).shape() as u32, (*template).size() as u32);
    (*(*sOamWork).array.at(id)).tileTag = (*(*template).spriteSheet).tag;
    (*(*sOamWork).array.at(id)).palTag = (*(*template).spritePal).tag;
    (*(*sOamWork).array.at(id)).isActive = TRUE;
    (*(*sOamWork).array.at(id)).pow10 = 1;
    for i in 1..((*template).oamCount as u32) {
        (*(*sOamWork).array.at(id)).pow10 *= 10;
    }
    CopyWorkToOam((*sOamWork).array.at(id));
    DigitObjUtil_PrintNumOn(id, num);
    TRUE as u32
}
unsafe fn CopyWorkToOam(objWork: *mut DigitPrinter) {
    let mut oamId: u32 = (*objWork).firstOamId as u32;
    let mut x: u32 = (*objWork).x as u32;
    let oamCount: u32 = (*objWork).oamCount as u32 + 1;
    {
        {
            let mut tmp: u16 = 0;
            volatile_write(&raw mut tmp, 0);
            CpuSet(
                &raw mut tmp as *mut c_void,
                &raw mut gMain.oamBuffer[oamId] as *mut c_void,
                0x1000000 | (8 * oamCount / 2) & 0x1FFFFF,
            );
        }
    }
    let mut i: u32 = 0;
    oamId = (*objWork).firstOamId as u32;
    while i < oamCount {
        gMain.oamBuffer[oamId].set_y((*objWork).y as u32);
        gMain.oamBuffer[oamId].set_x(x);
        gMain.oamBuffer[oamId].set_shape((*objWork).shape as u32);
        gMain.oamBuffer[oamId].set_size((*objWork).size as u32);
        gMain.oamBuffer[oamId].set_tileNum((*objWork).tileStart);
        gMain.oamBuffer[oamId].set_priority((*objWork).priority as u16);
        gMain.oamBuffer[oamId].set_paletteNum((*objWork).palTagIndex as u16);
        x += (*objWork).xDelta as u32;
        i += 1;
        oamId += 1;
    }
    oamId -= 1;
    gMain.oamBuffer[oamId].set_x((*objWork).x as u32 - (*objWork).xDelta as u32);
    gMain.oamBuffer[oamId].set_affineMode(ST_OAM_AFFINE_ERASE as u32);
    gMain.oamBuffer[oamId].set_tileNum((*objWork).tileStart + (*objWork).tilesPerImage as u16 * 10);
}
pub unsafe fn DigitObjUtil_PrintNumOn(id: u32, mut num: i32) {
    let mut sign: u32 = 0;
    if sOamWork.is_null() {
        return;
    }
    if (*(*sOamWork).array.at(id)).isActive == 0 {
        return;
    }
    (*(*sOamWork).array.at(id)).lastPrinted = num;
    if num < 0 {
        sign = TRUE as u32;
        num *= -1;
    } else {
        sign = FALSE as u32;
    }
    match (*(*sOamWork).array.at(id)).strConvMode {
        1 => {
            DrawNumObjsMinusInFront((*sOamWork).array.at(id), num, sign);
        }
        2 => {
            DrawNumObjsMinusInBack((*sOamWork).array.at(id), num, sign);
        }
        _ => {
            DrawNumObjsLeadingZeros((*sOamWork).array.at(id), num, sign);
        }
    }
}
unsafe fn DrawNumObjsLeadingZeros(objWork: *mut DigitPrinter, mut num: i32, sign: u32) {
    let mut pow10: u32 = (*objWork).pow10;
    let mut oamId: u32 = (*objWork).firstOamId as u32;
    while pow10 != 0 {
        let digit: u32 = div_u32(num as u32, pow10);
        num -= digit as i32 * pow10 as i32;
        pow10 /= 10;
        gMain.oamBuffer[oamId]
            .set_tileNum(digit as u16 * (*objWork).tilesPerImage as u16 + (*objWork).tileStart);
        oamId += 1;
    }
    if sign != 0 {
        gMain.oamBuffer[oamId].set_affineMode(ST_OAM_AFFINE_OFF);
    } else {
        gMain.oamBuffer[oamId].set_affineMode(ST_OAM_AFFINE_ERASE as u32);
    }
}
unsafe fn DrawNumObjsMinusInFront(objWork: *mut DigitPrinter, mut num: i32, sign: u32) {
    let mut pow10: u32 = (*objWork).pow10;
    DrawNumObjsMinusInFront_oamId.set((*objWork).firstOamId as i32);
    DrawNumObjsMinusInFront_curDigit.set(0);
    DrawNumObjsMinusInFront_firstDigit.set(-1);
    while pow10 != 0 {
        let digit: u32 = div_u32(num as u32, pow10);
        num -= digit as i32 * pow10 as i32;
        pow10 /= 10;
        if digit != 0 || DrawNumObjsMinusInFront_firstDigit.get() != -1 || pow10 == 0 {
            gMain.oamBuffer[DrawNumObjsMinusInFront_oamId.get()]
                .set_tileNum(digit as u16 * (*objWork).tilesPerImage as u16 + (*objWork).tileStart);
            gMain.oamBuffer[DrawNumObjsMinusInFront_oamId.get()].set_affineMode(ST_OAM_AFFINE_OFF);
            if DrawNumObjsMinusInFront_firstDigit.get() == -1 {
                DrawNumObjsMinusInFront_firstDigit.set(DrawNumObjsMinusInFront_curDigit.get());
            }
        } else {
            gMain.oamBuffer[DrawNumObjsMinusInFront_oamId.get()]
                .set_affineMode(ST_OAM_AFFINE_ERASE as u32);
        }
        DrawNumObjsMinusInFront_oamId.set(DrawNumObjsMinusInFront_oamId.get() + 1);
        DrawNumObjsMinusInFront_curDigit.set(DrawNumObjsMinusInFront_curDigit.get() + 1);
    }
    if sign != 0 {
        gMain.oamBuffer[DrawNumObjsMinusInFront_oamId.get()].set_affineMode(ST_OAM_AFFINE_OFF);
        gMain.oamBuffer[DrawNumObjsMinusInFront_oamId.get()].set_x(
            (*objWork).x as u32
                + (DrawNumObjsMinusInFront_firstDigit.get() as u32 - 1) * (*objWork).xDelta as u32,
        );
    } else {
        gMain.oamBuffer[DrawNumObjsMinusInFront_oamId.get()]
            .set_affineMode(ST_OAM_AFFINE_ERASE as u32);
    }
}
unsafe fn DrawNumObjsMinusInBack(objWork: *mut DigitPrinter, mut num: i32, sign: u32) {
    let mut pow10: u32 = (*objWork).pow10;
    let mut oamId: u32 = (*objWork).firstOamId as u32;
    let mut printingDigits: u32 = FALSE as u32;
    let mut nsprites: i32 = 0;
    while pow10 != 0 {
        let digit: u32 = div_u32(num as u32, pow10);
        num -= digit as i32 * pow10 as i32;
        pow10 /= 10;
        if digit != 0 || printingDigits != 0 || pow10 == 0 {
            printingDigits = TRUE as u32;
            gMain.oamBuffer[oamId]
                .set_tileNum(digit as u16 * (*objWork).tilesPerImage as u16 + (*objWork).tileStart);
            gMain.oamBuffer[oamId].set_affineMode(ST_OAM_AFFINE_OFF);
            oamId += 1;
            nsprites += 1;
        }
    }
    while nsprites < (*objWork).oamCount as i32 {
        gMain.oamBuffer[oamId].set_affineMode(ST_OAM_AFFINE_ERASE as u32);
        oamId += 1;
        nsprites += 1;
    }
    if sign != 0 {
        gMain.oamBuffer[oamId].set_affineMode(ST_OAM_AFFINE_OFF);
    } else {
        gMain.oamBuffer[oamId].set_affineMode(ST_OAM_AFFINE_ERASE as u32);
    }
}
pub unsafe fn DigitObjUtil_DeletePrinter(id: u32) {
    let mut oamCount: i32 = 0;
    if sOamWork.is_null() {
        return;
    }
    if (*(*sOamWork).array.at(id)).isActive == 0 {
        return;
    }
    oamCount = (*(*sOamWork).array.at(id)).oamCount as i32 + 1;
    let mut oamId: i32 = (*(*sOamWork).array.at(id)).firstOamId as i32;
    let mut i: i32 = 0;
    while i < oamCount {
        gMain.oamBuffer[oamId].set_affineMode(ST_OAM_AFFINE_ERASE as u32);
        i += 1;
        oamId += 1;
    }
    if SharesTileWithAnyActive(id) == 0 {
        FreeSpriteTilesByTag((*(*sOamWork).array.at(id)).tileTag);
    }
    if SharesPalWithAnyActive(id) == 0 {
        FreeSpritePaletteByTag((*(*sOamWork).array.at(id)).palTag);
    }
    (*(*sOamWork).array.at(id)).isActive = FALSE;
}
pub unsafe fn DigitObjUtil_HideOrShow(id: u32, hide: u32) {
    let mut oamCount: i32 = 0;
    let mut i: i32 = 0;
    if sOamWork.is_null() {
        return;
    }
    if (*(*sOamWork).array.at(id)).isActive == 0 {
        return;
    }
    oamCount = (*(*sOamWork).array.at(id)).oamCount as i32 + 1;
    let mut oamId: i32 = (*(*sOamWork).array.at(id)).firstOamId as i32;
    if hide != 0 {
        i = 0;
        while i < oamCount {
            gMain.oamBuffer[oamId].set_affineMode(ST_OAM_AFFINE_ERASE as u32);
            i += 1;
            oamId += 1;
        }
    } else {
        i = 0;
        while i < oamCount {
            gMain.oamBuffer[oamId].set_affineMode(ST_OAM_AFFINE_OFF);
            i += 1;
            oamId += 1;
        }
        DigitObjUtil_PrintNumOn(id, (*(*sOamWork).array.at(id)).lastPrinted);
    }
}
unsafe fn GetFirstOamId(oamCount: u8) -> u8 {
    let mut firstOamId: u16 = 64;
    for i in 0..(*sOamWork).count {
        if (*(*sOamWork).array.at(i)).isActive == 0 {
            if (*(*sOamWork).array.at(i)).firstOamId != 0xFF
                && (*(*sOamWork).array.at(i)).oamCount <= oamCount
            {
                return (*(*sOamWork).array.at(i)).firstOamId;
            }
        } else {
            firstOamId += 1 + (*(*sOamWork).array.at(i)).oamCount as u16;
        }
    }
    if firstOamId as i32 + oamCount as i32 + 1 > 128 {
        return 0xFF;
    } else {
        return firstOamId as u8;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
unsafe fn SharesTileWithAnyActive(id: u32) -> u32 {
    for i in 0..(*sOamWork).count {
        if (*(*sOamWork).array.at(i)).isActive != 0
            && i != id
            && (*(*sOamWork).array.at(i)).tileTag == (*(*sOamWork).array.at(id)).tileTag
        {
            return TRUE as u32;
        }
    }
    FALSE as u32
}
unsafe fn SharesPalWithAnyActive(id: u32) -> u32 {
    for i in 0..(*sOamWork).count {
        if (*(*sOamWork).array.at(i)).isActive != 0
            && i != id
            && (*(*sOamWork).array.at(i)).palTag == (*(*sOamWork).array.at(id)).palTag
        {
            return TRUE as u32;
        }
    }
    FALSE as u32
}
#[unsafe(no_mangle)]
pub unsafe fn GetTilesPerImage(shape: u32, size: u32) -> u8 {
    sTilesPerImage[shape][size]
}
