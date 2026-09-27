//! Translated from `src/digit_obj_util.c` by tools/rustport/c2rs.py, then reviewed.
#![allow(
    non_snake_case,
    non_upper_case_globals,
    unused_mut,
    unused_variables,
    unused_assignments,
    unused_parens,
    unused_braces,
    unused_labels,
    unused_comparisons,
    overflowing_literals,
    unused_unsafe,
    dead_code,
    unreachable_code,
    clippy::all,
    clashing_extern_declarations,
    unpredictable_function_pointer_comparisons
)]

// Data tables (translate with cdata.py): sTilesPerImage
#[allow(unused_imports)]
use crate::data::digit_obj_util::*;

#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sOamWork: *mut u8 = core::ptr::null_mut();
static mut DRAWNUMOBJSMINUSINFRONT_OAMID: i32 = 0i32;
static mut DRAWNUMOBJSMINUSINFRONT_CURDIGIT: i32 = 0i32;
static mut DRAWNUMOBJSMINUSINFRONT_FIRSTDIGIT: i32 = 0i32;

unsafe extern "C" {
    static mut gMain: u8;
    fn Alloc(a0: u32) -> *mut u8;
    fn CpuSet(a0: *mut u8, a1: *mut u8, a2: u32);
    fn Free(a0: *mut u8);
    fn FreeSpritePaletteByTag(a0: u16);
    fn FreeSpriteTilesByTag(a0: u16);
    fn GetDecompressedDataSize(a0: *mut u32) -> u32;
    fn GetSpriteTileStartByTag(a0: u16) -> u16;
    fn IndexOfSpritePaletteTag(a0: u16) -> u8;
    fn LoadCompressedSpriteSheet(a0: *mut u8) -> u16;
    fn LoadSpritePalette(a0: *mut u8) -> u8;
    fn LoadSpriteSheet(a0: *mut u8) -> u16;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn DigitObjUtil_Init(count: u32) -> u32 {
    unsafe {
        let mut count = count;
        let mut i: u32 = 0u32;
        if ((((&raw mut sOamWork).cast::<u8>().cast::<*mut u8>()).read()) as usize) != 0usize {
            DigitObjUtil_Free();
        }
        ((&raw mut sOamWork).cast::<u8>().cast::<*mut u8>()).write(Alloc(8u32));
        if ((((&raw mut sOamWork).cast::<u8>().cast::<*mut u8>()).read()) as usize) == 0usize {
            return 0u32;
        }
        ((((&raw mut sOamWork).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(4)
            .cast::<*mut u8>())
        .write(Alloc((28u32).wrapping_mul(count)));
        if ((((((&raw mut sOamWork).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(4)
            .cast::<*mut u8>())
        .read()) as usize)
            == 0usize
        {
            Free(((&raw mut sOamWork).cast::<u8>().cast::<*mut u8>()).read());
            return 0u32;
        }
        ((((&raw mut sOamWork).cast::<u8>().cast::<*mut u8>()).read()).cast::<u32>()).write(count);
        {
            i = 0u32;
            'l1: loop {
                if !(i < count) {
                    break 'l1;
                }
                'l2: {
                    ((((((&raw mut sOamWork).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(4)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_offset(((i) as i32) as isize * 28))
                    .write(0u8);
                    (((((((&raw mut sOamWork).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(4)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_offset(((i) as i32) as isize * 28))
                    .wrapping_add(1))
                    .write(255u8);
                }
                i = (i).wrapping_add(1);
            }
        }
        return 1u32;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DigitObjUtil_Free() {
    unsafe {
        if ((((&raw mut sOamWork).cast::<u8>().cast::<*mut u8>()).read()) as usize) != 0usize {
            if ((((((&raw mut sOamWork).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(4)
                .cast::<*mut u8>())
            .read()) as usize)
                != 0usize
            {
                let mut i: u32 = 0u32;
                {
                    i = 0u32;
                    'l1: loop {
                        if !(i
                            < ((((&raw mut sOamWork).cast::<u8>().cast::<*mut u8>()).read())
                                .cast::<u32>())
                            .read())
                        {
                            break 'l1;
                        }
                        'l2: {
                            DigitObjUtil_DeletePrinter(i);
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                Free(
                    ((((&raw mut sOamWork).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(4)
                        .cast::<*mut u8>())
                    .read(),
                );
            }
            {
                Free(((&raw mut sOamWork).cast::<u8>().cast::<*mut u8>()).read());
                ((&raw mut sOamWork).cast::<u8>().cast::<*mut u8>()).write(core::ptr::null_mut());
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DigitObjUtil_CreatePrinter(id: u32, num: i32, template: *mut u8) -> u32 {
    unsafe {
        let mut id = id;
        let mut num = num;
        let mut template = template;
        let mut i: u32 = 0u32;
        if ((((&raw mut sOamWork).cast::<u8>().cast::<*mut u8>()).read()) as usize) == 0usize {
            return 0u32;
        }
        if (((((((&raw mut sOamWork).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(4)
            .cast::<*mut u8>())
        .read())
        .wrapping_offset(((id) as i32) as isize * 28))
        .read())
            != 0
        {
            return 0u32;
        }
        (((((((&raw mut sOamWork).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(4)
            .cast::<*mut u8>())
        .read())
        .wrapping_offset(((id) as i32) as isize * 28))
        .wrapping_add(1))
        .write(GetFirstOamId(((template).wrapping_add(1)).read()));
        if (((((((((&raw mut sOamWork).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(4)
            .cast::<*mut u8>())
        .read())
        .wrapping_offset(((id) as i32) as isize * 28))
        .wrapping_add(1))
        .read()) as i32)
            == 255i32
        {
            return 0u32;
        }
        (((((((&raw mut sOamWork).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(4)
            .cast::<*mut u8>())
        .read())
        .wrapping_offset(((id) as i32) as isize * 28))
        .wrapping_add(10)
        .cast::<u16>())
        .write(GetSpriteTileStartByTag(
            ((((template).wrapping_add(8).cast::<*mut u8>()).read())
                .wrapping_add(6)
                .cast::<u16>())
            .read(),
        ));
        if (((((((((&raw mut sOamWork).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(4)
            .cast::<*mut u8>())
        .read())
        .wrapping_offset(((id) as i32) as isize * 28))
        .wrapping_add(10)
        .cast::<u16>())
        .read()) as i32)
            == 65535i32
        {
            if ((((((template).wrapping_add(8).cast::<*mut u8>()).read())
                .wrapping_add(4)
                .cast::<u16>())
            .read()) as i32)
                != 0i32
            {
                (((((((&raw mut sOamWork).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(4)
                    .cast::<*mut u8>())
                .read())
                .wrapping_offset(((id) as i32) as isize * 28))
                .wrapping_add(10)
                .cast::<u16>())
                .write(LoadSpriteSheet(
                    ((template).wrapping_add(8).cast::<*mut u8>()).read(),
                ));
            } else {
                let mut compSpriteSheet = crate::ffi::Align4([0u8; 8]);
                (&raw mut compSpriteSheet)
                    .cast::<u8>()
                    .cast::<crate::c::Rec4<8>>()
                    .write_unaligned(
                        ((template).wrapping_add(8).cast::<*mut u8>())
                            .read()
                            .cast::<crate::c::Rec4<8>>()
                            .read_unaligned(),
                    );
                (((&raw mut compSpriteSheet).cast::<u8>())
                    .wrapping_add(4)
                    .cast::<u16>())
                .write(
                    ((GetDecompressedDataSize(
                        (((((template).wrapping_add(8).cast::<*mut u8>()).read())
                            .cast::<*mut u8>())
                        .read())
                        .cast::<u32>(),
                    )) as u16),
                );
                (((((((&raw mut sOamWork).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(4)
                    .cast::<*mut u8>())
                .read())
                .wrapping_offset(((id) as i32) as isize * 28))
                .wrapping_add(10)
                .cast::<u16>())
                .write(LoadCompressedSpriteSheet(
                    (&raw mut compSpriteSheet).cast::<u8>(),
                ));
            }
            if (((((((((&raw mut sOamWork).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(4)
                .cast::<*mut u8>())
            .read())
            .wrapping_offset(((id) as i32) as isize * 28))
            .wrapping_add(10)
            .cast::<u16>())
            .read()) as i32)
                == 65535i32
            {
                return 0u32;
            }
        }
        (((((((&raw mut sOamWork).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(4)
            .cast::<*mut u8>())
        .read())
        .wrapping_offset(((id) as i32) as isize * 28))
        .wrapping_add(4))
        .write(IndexOfSpritePaletteTag(
            ((((template).wrapping_add(12).cast::<*mut u8>()).read())
                .wrapping_add(4)
                .cast::<u16>())
            .read(),
        ));
        if (((((((((&raw mut sOamWork).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(4)
            .cast::<*mut u8>())
        .read())
        .wrapping_offset(((id) as i32) as isize * 28))
        .wrapping_add(4))
        .read()) as i32)
            == 255i32
        {
            (((((((&raw mut sOamWork).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(4)
                .cast::<*mut u8>())
            .read())
            .wrapping_offset(((id) as i32) as isize * 28))
            .wrapping_add(4))
            .write(LoadSpritePalette(
                ((template).wrapping_add(12).cast::<*mut u8>()).read(),
            ));
        }
        (((((((&raw mut sOamWork).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(4)
            .cast::<*mut u8>())
        .read())
        .wrapping_offset(((id) as i32) as isize * 28))
        .wrapping_add(2))
        .write((crate::c::bf_read((template).wrapping_add(0), 0, 2, false) as u8));
        (((((((&raw mut sOamWork).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(4)
            .cast::<*mut u8>())
        .read())
        .wrapping_offset(((id) as i32) as isize * 28))
        .wrapping_add(3))
        .write(((template).wrapping_add(1)).read());
        (((((((&raw mut sOamWork).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(4)
            .cast::<*mut u8>())
        .read())
        .wrapping_offset(((id) as i32) as isize * 28))
        .wrapping_add(12)
        .cast::<i16>())
        .write(((template).wrapping_add(4).cast::<i16>()).read());
        (((((((&raw mut sOamWork).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(4)
            .cast::<*mut u8>())
        .read())
        .wrapping_offset(((id) as i32) as isize * 28))
        .wrapping_add(14)
        .cast::<i16>())
        .write(((template).wrapping_add(6).cast::<i16>()).read());
        (((((((&raw mut sOamWork).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(4)
            .cast::<*mut u8>())
        .read())
        .wrapping_offset(((id) as i32) as isize * 28))
        .wrapping_add(6))
        .write((crate::c::bf_read((template).wrapping_add(0), 2, 2, false) as u8));
        (((((((&raw mut sOamWork).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(4)
            .cast::<*mut u8>())
        .read())
        .wrapping_offset(((id) as i32) as isize * 28))
        .wrapping_add(5))
        .write((crate::c::bf_read((template).wrapping_add(0), 4, 2, false) as u8));
        (((((((&raw mut sOamWork).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(4)
            .cast::<*mut u8>())
        .read())
        .wrapping_offset(((id) as i32) as isize * 28))
        .wrapping_add(7))
        .write((crate::c::bf_read((template).wrapping_add(0), 6, 2, false) as u8));
        (((((((&raw mut sOamWork).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(4)
            .cast::<*mut u8>())
        .read())
        .wrapping_offset(((id) as i32) as isize * 28))
        .wrapping_add(8))
        .write(((template).wrapping_add(2)).read());
        (((((((&raw mut sOamWork).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(4)
            .cast::<*mut u8>())
        .read())
        .wrapping_offset(((id) as i32) as isize * 28))
        .wrapping_add(9))
        .write(GetTilesPerImage(
            ((crate::c::bf_read((template).wrapping_add(0), 2, 2, false) as u8) as u32),
            ((crate::c::bf_read((template).wrapping_add(0), 4, 2, false) as u8) as u32),
        ));
        (((((((&raw mut sOamWork).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(4)
            .cast::<*mut u8>())
        .read())
        .wrapping_offset(((id) as i32) as isize * 28))
        .wrapping_add(16)
        .cast::<u16>())
        .write(
            ((((template).wrapping_add(8).cast::<*mut u8>()).read())
                .wrapping_add(6)
                .cast::<u16>())
            .read(),
        );
        (((((((&raw mut sOamWork).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(4)
            .cast::<*mut u8>())
        .read())
        .wrapping_offset(((id) as i32) as isize * 28))
        .wrapping_add(18)
        .cast::<u16>())
        .write(
            ((((template).wrapping_add(12).cast::<*mut u8>()).read())
                .wrapping_add(4)
                .cast::<u16>())
            .read(),
        );
        ((((((&raw mut sOamWork).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(4)
            .cast::<*mut u8>())
        .read())
        .wrapping_offset(((id) as i32) as isize * 28))
        .write(1u8);
        (((((((&raw mut sOamWork).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(4)
            .cast::<*mut u8>())
        .read())
        .wrapping_offset(((id) as i32) as isize * 28))
        .wrapping_add(20)
        .cast::<u32>())
        .write(1u32);
        {
            i = 1u32;
            'l1: loop {
                if !(i < ((((template).wrapping_add(1)).read()) as u32)) {
                    break 'l1;
                }
                'l2: {
                    let __p1 = ((((((&raw mut sOamWork).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(4)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_offset(((id) as i32) as isize * 28))
                    .wrapping_add(20)
                    .cast::<u32>();
                    (__p1).write(((__p1).read()).wrapping_mul(10u32));
                }
                i = (i).wrapping_add(1);
            }
        }
        CopyWorkToOam(
            (((((&raw mut sOamWork).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(4)
                .cast::<*mut u8>())
            .read())
            .wrapping_offset(((id) as i32) as isize * 28),
        );
        DigitObjUtil_PrintNumOn(id, num);
        return 1u32;
    }
}
pub(crate) unsafe extern "C" fn CopyWorkToOam(objWork: *mut u8) {
    unsafe {
        let mut objWork = objWork;
        let mut i: u32 = 0u32;
        let mut oamId: u32 = ((((objWork).wrapping_add(1)).read()) as u32);
        let mut x: u32 = ((((objWork).wrapping_add(12).cast::<i16>()).read()) as u32);
        let mut oamCount: u32 =
            ((((((objWork).wrapping_add(3)).read()) as i32).wrapping_add(1i32)) as u32);
        'l1: loop {
            'l2: {
                {
                    let mut tmp: u16 = 0u16;
                    (&raw mut tmp).write_volatile(0u16);
                    'l3: loop {
                        'l4: {
                            CpuSet(
                                (&raw mut tmp).cast::<u8>(),
                                ((((&raw mut gMain).cast::<u8>()).wrapping_add(56)).cast::<u8>())
                                    .wrapping_offset(((oamId) as i32) as isize * 8),
                                (16777216u32
                                    | (crate::c::div_u32(
                                        (8u32).wrapping_mul(oamCount),
                                        ((crate::c::div_i32(16i32, 8i32)) as u32),
                                    ) & 2097151u32)),
                            );
                        }
                        if !((0i32) != 0) {
                            break 'l3;
                        }
                    }
                }
            }
            if !((0i32) != 0) {
                break 'l1;
            }
        }
        {
            i = 0u32;
            oamId = ((((objWork).wrapping_add(1)).read()) as u32);
            'l5: loop {
                if !(i < oamCount) {
                    break 'l5;
                }
                'l6: {
                    crate::c::bf_write(
                        (((((&raw mut gMain).cast::<u8>()).wrapping_add(56)).cast::<u8>())
                            .wrapping_offset(((oamId) as i32) as isize * 8))
                        .wrapping_add(0),
                        0,
                        8,
                        ((((objWork).wrapping_add(14).cast::<i16>()).read()) as u32) as i32,
                    );
                    crate::c::bf_write(
                        (((((&raw mut gMain).cast::<u8>()).wrapping_add(56)).cast::<u8>())
                            .wrapping_offset(((oamId) as i32) as isize * 8))
                        .wrapping_add(2),
                        0,
                        9,
                        (x) as i32,
                    );
                    crate::c::bf_write(
                        (((((&raw mut gMain).cast::<u8>()).wrapping_add(56)).cast::<u8>())
                            .wrapping_offset(((oamId) as i32) as isize * 8))
                        .wrapping_add(1),
                        6,
                        2,
                        ((((objWork).wrapping_add(6)).read()) as u32) as i32,
                    );
                    crate::c::bf_write(
                        (((((&raw mut gMain).cast::<u8>()).wrapping_add(56)).cast::<u8>())
                            .wrapping_offset(((oamId) as i32) as isize * 8))
                        .wrapping_add(3),
                        6,
                        2,
                        ((((objWork).wrapping_add(5)).read()) as u32) as i32,
                    );
                    crate::c::bf_write(
                        (((((&raw mut gMain).cast::<u8>()).wrapping_add(56)).cast::<u8>())
                            .wrapping_offset(((oamId) as i32) as isize * 8))
                        .wrapping_add(4),
                        0,
                        10,
                        (((objWork).wrapping_add(10).cast::<u16>()).read()) as i32,
                    );
                    crate::c::bf_write(
                        (((((&raw mut gMain).cast::<u8>()).wrapping_add(56)).cast::<u8>())
                            .wrapping_offset(((oamId) as i32) as isize * 8))
                        .wrapping_add(5),
                        2,
                        2,
                        ((((objWork).wrapping_add(7)).read()) as u16) as i32,
                    );
                    crate::c::bf_write(
                        (((((&raw mut gMain).cast::<u8>()).wrapping_add(56)).cast::<u8>())
                            .wrapping_offset(((oamId) as i32) as isize * 8))
                        .wrapping_add(5),
                        4,
                        4,
                        ((((objWork).wrapping_add(4)).read()) as u16) as i32,
                    );
                    x = (x).wrapping_add(((((objWork).wrapping_add(8)).read()) as u32));
                }
                i = (i).wrapping_add(1);
                oamId = (oamId).wrapping_add(1);
            }
        }
        oamId = (oamId).wrapping_sub(1);
        crate::c::bf_write(
            (((((&raw mut gMain).cast::<u8>()).wrapping_add(56)).cast::<u8>())
                .wrapping_offset(((oamId) as i32) as isize * 8))
            .wrapping_add(2),
            0,
            9,
            ((((((objWork).wrapping_add(12).cast::<i16>()).read()) as i32)
                .wrapping_sub(((((objWork).wrapping_add(8)).read()) as i32))) as u32)
                as i32,
        );
        crate::c::bf_write(
            (((((&raw mut gMain).cast::<u8>()).wrapping_add(56)).cast::<u8>())
                .wrapping_offset(((oamId) as i32) as isize * 8))
            .wrapping_add(1),
            0,
            2,
            (2u32) as i32,
        );
        crate::c::bf_write(
            (((((&raw mut gMain).cast::<u8>()).wrapping_add(56)).cast::<u8>())
                .wrapping_offset(((oamId) as i32) as isize * 8))
            .wrapping_add(4),
            0,
            10,
            ((((((objWork).wrapping_add(10).cast::<u16>()).read()) as i32)
                .wrapping_add(((((objWork).wrapping_add(9)).read()) as i32).wrapping_mul(10i32)))
                as u16) as i32,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DigitObjUtil_PrintNumOn(id: u32, num: i32) {
    unsafe {
        let mut id = id;
        let mut num = num;
        let mut sign: u32 = 0u32;
        if ((((&raw mut sOamWork).cast::<u8>().cast::<*mut u8>()).read()) as usize) == 0usize {
            return;
        }
        if !((((((((&raw mut sOamWork).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(4)
            .cast::<*mut u8>())
        .read())
        .wrapping_offset(((id) as i32) as isize * 28))
        .read())
            != 0)
        {
            return;
        }
        (((((((&raw mut sOamWork).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(4)
            .cast::<*mut u8>())
        .read())
        .wrapping_offset(((id) as i32) as isize * 28))
        .wrapping_add(24)
        .cast::<i32>())
        .write(num);
        if num < 0i32 {
            sign = 1u32;
            num = (num).wrapping_mul((-1i32));
        } else {
            sign = 0u32;
        }
        'l1: {
            let __sw1 = (((((((((&raw mut sOamWork).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(4)
                .cast::<*mut u8>())
            .read())
            .wrapping_offset(((id) as i32) as isize * 28))
            .wrapping_add(2))
            .read()) as i32);
            let __matched = __sw1 == 0i32 || __sw1 == 1i32 || __sw1 == 2i32;
            if __sw1 == 0i32 || !__matched {
                DrawNumObjsLeadingZeros(
                    (((((&raw mut sOamWork).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(4)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_offset(((id) as i32) as isize * 28),
                    num,
                    sign,
                );
                break 'l1;
            }
            if __sw1 == 1i32 {
                DrawNumObjsMinusInFront(
                    (((((&raw mut sOamWork).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(4)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_offset(((id) as i32) as isize * 28),
                    num,
                    sign,
                );
                break 'l1;
            }
            if __sw1 == 2i32 {
                DrawNumObjsMinusInBack(
                    (((((&raw mut sOamWork).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(4)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_offset(((id) as i32) as isize * 28),
                    num,
                    sign,
                );
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn DrawNumObjsLeadingZeros(objWork: *mut u8, num: i32, sign: u32) {
    unsafe {
        let mut objWork = objWork;
        let mut num = num;
        let mut sign = sign;
        let mut pow10: u32 = ((objWork).wrapping_add(20).cast::<u32>()).read();
        let mut oamId: u32 = ((((objWork).wrapping_add(1)).read()) as u32);
        'l1: loop {
            if !(pow10 != 0u32) {
                break 'l1;
            }
            let mut digit: u32 = crate::c::div_u32(((num) as u32), pow10);
            num = ((((num) as u32).wrapping_sub((digit).wrapping_mul(pow10))) as i32);
            pow10 = crate::c::div_u32(pow10, 10u32);
            crate::c::bf_write(
                (((((&raw mut gMain).cast::<u8>()).wrapping_add(56)).cast::<u8>())
                    .wrapping_offset(((oamId) as i32) as isize * 8))
                .wrapping_add(4),
                0,
                10,
                ((((digit).wrapping_mul(((((objWork).wrapping_add(9)).read()) as u32)))
                    .wrapping_add(((((objWork).wrapping_add(10).cast::<u16>()).read()) as u32)))
                    as u16) as i32,
            );
            oamId = (oamId).wrapping_add(1);
        }
        if (sign) != 0 {
            crate::c::bf_write(
                (((((&raw mut gMain).cast::<u8>()).wrapping_add(56)).cast::<u8>())
                    .wrapping_offset(((oamId) as i32) as isize * 8))
                .wrapping_add(1),
                0,
                2,
                (0u32) as i32,
            );
        } else {
            crate::c::bf_write(
                (((((&raw mut gMain).cast::<u8>()).wrapping_add(56)).cast::<u8>())
                    .wrapping_offset(((oamId) as i32) as isize * 8))
                .wrapping_add(1),
                0,
                2,
                (2u32) as i32,
            );
        }
    }
}
pub(crate) unsafe extern "C" fn DrawNumObjsMinusInFront(objWork: *mut u8, num: i32, sign: u32) {
    unsafe {
        let mut objWork = objWork;
        let mut num = num;
        let mut sign = sign;
        let mut pow10: u32 = ((objWork).wrapping_add(20).cast::<u32>()).read();
        (&raw mut DRAWNUMOBJSMINUSINFRONT_OAMID)
            .write(((((objWork).wrapping_add(1)).read()) as i32));
        (&raw mut DRAWNUMOBJSMINUSINFRONT_CURDIGIT).write(0i32);
        (&raw mut DRAWNUMOBJSMINUSINFRONT_FIRSTDIGIT).write((-1i32));
        'l1: loop {
            if !(pow10 != 0u32) {
                break 'l1;
            }
            let mut digit: u32 = crate::c::div_u32(((num) as u32), pow10);
            num = ((((num) as u32).wrapping_sub((digit).wrapping_mul(pow10))) as i32);
            pow10 = crate::c::div_u32(pow10, 10u32);
            if ((digit != 0u32)
                || ((&raw mut DRAWNUMOBJSMINUSINFRONT_FIRSTDIGIT).read() != (-1i32)))
                || (pow10 == 0u32)
            {
                crate::c::bf_write(
                    (((((&raw mut gMain).cast::<u8>()).wrapping_add(56)).cast::<u8>())
                        .wrapping_offset(
                            ((&raw mut DRAWNUMOBJSMINUSINFRONT_OAMID).read()) as isize * 8,
                        ))
                    .wrapping_add(4),
                    0,
                    10,
                    ((((digit).wrapping_mul(((((objWork).wrapping_add(9)).read()) as u32)))
                        .wrapping_add(((((objWork).wrapping_add(10).cast::<u16>()).read()) as u32)))
                        as u16) as i32,
                );
                crate::c::bf_write(
                    (((((&raw mut gMain).cast::<u8>()).wrapping_add(56)).cast::<u8>())
                        .wrapping_offset(
                            ((&raw mut DRAWNUMOBJSMINUSINFRONT_OAMID).read()) as isize * 8,
                        ))
                    .wrapping_add(1),
                    0,
                    2,
                    (0u32) as i32,
                );
                if (&raw mut DRAWNUMOBJSMINUSINFRONT_FIRSTDIGIT).read() == (-1i32) {
                    (&raw mut DRAWNUMOBJSMINUSINFRONT_FIRSTDIGIT)
                        .write((&raw mut DRAWNUMOBJSMINUSINFRONT_CURDIGIT).read());
                }
            } else {
                crate::c::bf_write(
                    (((((&raw mut gMain).cast::<u8>()).wrapping_add(56)).cast::<u8>())
                        .wrapping_offset(
                            ((&raw mut DRAWNUMOBJSMINUSINFRONT_OAMID).read()) as isize * 8,
                        ))
                    .wrapping_add(1),
                    0,
                    2,
                    (2u32) as i32,
                );
            }
            (&raw mut DRAWNUMOBJSMINUSINFRONT_OAMID)
                .write(((&raw mut DRAWNUMOBJSMINUSINFRONT_OAMID).read()).wrapping_add(1));
            (&raw mut DRAWNUMOBJSMINUSINFRONT_CURDIGIT)
                .write(((&raw mut DRAWNUMOBJSMINUSINFRONT_CURDIGIT).read()).wrapping_add(1));
        }
        if (sign) != 0 {
            crate::c::bf_write(
                (((((&raw mut gMain).cast::<u8>()).wrapping_add(56)).cast::<u8>())
                    .wrapping_offset(
                        ((&raw mut DRAWNUMOBJSMINUSINFRONT_OAMID).read()) as isize * 8,
                    ))
                .wrapping_add(1),
                0,
                2,
                (0u32) as i32,
            );
            crate::c::bf_write(
                (((((&raw mut gMain).cast::<u8>()).wrapping_add(56)).cast::<u8>())
                    .wrapping_offset(
                        ((&raw mut DRAWNUMOBJSMINUSINFRONT_OAMID).read()) as isize * 8,
                    ))
                .wrapping_add(2),
                0,
                9,
                ((((((objWork).wrapping_add(12).cast::<i16>()).read()) as i32).wrapping_add(
                    (((&raw mut DRAWNUMOBJSMINUSINFRONT_FIRSTDIGIT).read()).wrapping_sub(1i32))
                        .wrapping_mul(((((objWork).wrapping_add(8)).read()) as i32)),
                )) as u32) as i32,
            );
        } else {
            crate::c::bf_write(
                (((((&raw mut gMain).cast::<u8>()).wrapping_add(56)).cast::<u8>())
                    .wrapping_offset(
                        ((&raw mut DRAWNUMOBJSMINUSINFRONT_OAMID).read()) as isize * 8,
                    ))
                .wrapping_add(1),
                0,
                2,
                (2u32) as i32,
            );
        }
    }
}
pub(crate) unsafe extern "C" fn DrawNumObjsMinusInBack(objWork: *mut u8, num: i32, sign: u32) {
    unsafe {
        let mut objWork = objWork;
        let mut num = num;
        let mut sign = sign;
        let mut pow10: u32 = ((objWork).wrapping_add(20).cast::<u32>()).read();
        let mut oamId: u32 = ((((objWork).wrapping_add(1)).read()) as u32);
        let mut printingDigits: u32 = 0u32;
        let mut nsprites: i32 = 0i32;
        'l1: loop {
            if !(pow10 != 0u32) {
                break 'l1;
            }
            let mut digit: u32 = crate::c::div_u32(((num) as u32), pow10);
            num = ((((num) as u32).wrapping_sub((digit).wrapping_mul(pow10))) as i32);
            pow10 = crate::c::div_u32(pow10, 10u32);
            if ((digit != 0u32) || ((printingDigits) != 0)) || (pow10 == 0u32) {
                printingDigits = 1u32;
                crate::c::bf_write(
                    (((((&raw mut gMain).cast::<u8>()).wrapping_add(56)).cast::<u8>())
                        .wrapping_offset(((oamId) as i32) as isize * 8))
                    .wrapping_add(4),
                    0,
                    10,
                    ((((digit).wrapping_mul(((((objWork).wrapping_add(9)).read()) as u32)))
                        .wrapping_add(((((objWork).wrapping_add(10).cast::<u16>()).read()) as u32)))
                        as u16) as i32,
                );
                crate::c::bf_write(
                    (((((&raw mut gMain).cast::<u8>()).wrapping_add(56)).cast::<u8>())
                        .wrapping_offset(((oamId) as i32) as isize * 8))
                    .wrapping_add(1),
                    0,
                    2,
                    (0u32) as i32,
                );
                oamId = (oamId).wrapping_add(1);
                nsprites = (nsprites).wrapping_add(1);
            }
        }
        'l2: loop {
            if !(nsprites < ((((objWork).wrapping_add(3)).read()) as i32)) {
                break 'l2;
            }
            crate::c::bf_write(
                (((((&raw mut gMain).cast::<u8>()).wrapping_add(56)).cast::<u8>())
                    .wrapping_offset(((oamId) as i32) as isize * 8))
                .wrapping_add(1),
                0,
                2,
                (2u32) as i32,
            );
            oamId = (oamId).wrapping_add(1);
            nsprites = (nsprites).wrapping_add(1);
        }
        if (sign) != 0 {
            crate::c::bf_write(
                (((((&raw mut gMain).cast::<u8>()).wrapping_add(56)).cast::<u8>())
                    .wrapping_offset(((oamId) as i32) as isize * 8))
                .wrapping_add(1),
                0,
                2,
                (0u32) as i32,
            );
        } else {
            crate::c::bf_write(
                (((((&raw mut gMain).cast::<u8>()).wrapping_add(56)).cast::<u8>())
                    .wrapping_offset(((oamId) as i32) as isize * 8))
                .wrapping_add(1),
                0,
                2,
                (2u32) as i32,
            );
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DigitObjUtil_DeletePrinter(id: u32) {
    unsafe {
        let mut id = id;
        let mut oamId: i32 = 0i32;
        let mut oamCount: i32 = 0i32;
        let mut i: i32 = 0i32;
        if ((((&raw mut sOamWork).cast::<u8>().cast::<*mut u8>()).read()) as usize) == 0usize {
            return;
        }
        if !((((((((&raw mut sOamWork).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(4)
            .cast::<*mut u8>())
        .read())
        .wrapping_offset(((id) as i32) as isize * 28))
        .read())
            != 0)
        {
            return;
        }
        oamCount = (((((((((&raw mut sOamWork).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(4)
            .cast::<*mut u8>())
        .read())
        .wrapping_offset(((id) as i32) as isize * 28))
        .wrapping_add(3))
        .read()) as i32)
            .wrapping_add(1i32);
        oamId = (((((((((&raw mut sOamWork).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(4)
            .cast::<*mut u8>())
        .read())
        .wrapping_offset(((id) as i32) as isize * 28))
        .wrapping_add(1))
        .read()) as i32);
        {
            i = 0i32;
            'l1: loop {
                if !(i < oamCount) {
                    break 'l1;
                }
                'l2: {
                    crate::c::bf_write(
                        (((((&raw mut gMain).cast::<u8>()).wrapping_add(56)).cast::<u8>())
                            .wrapping_offset((oamId) as isize * 8))
                        .wrapping_add(1),
                        0,
                        2,
                        (2u32) as i32,
                    );
                }
                i = (i).wrapping_add(1);
                oamId = (oamId).wrapping_add(1);
            }
        }
        if !((SharesTileWithAnyActive(id)) != 0) {
            FreeSpriteTilesByTag(
                (((((((&raw mut sOamWork).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(4)
                    .cast::<*mut u8>())
                .read())
                .wrapping_offset(((id) as i32) as isize * 28))
                .wrapping_add(16)
                .cast::<u16>())
                .read(),
            );
        }
        if !((SharesPalWithAnyActive(id)) != 0) {
            FreeSpritePaletteByTag(
                (((((((&raw mut sOamWork).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(4)
                    .cast::<*mut u8>())
                .read())
                .wrapping_offset(((id) as i32) as isize * 28))
                .wrapping_add(18)
                .cast::<u16>())
                .read(),
            );
        }
        ((((((&raw mut sOamWork).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(4)
            .cast::<*mut u8>())
        .read())
        .wrapping_offset(((id) as i32) as isize * 28))
        .write(0u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DigitObjUtil_HideOrShow(id: u32, hide: u32) {
    unsafe {
        let mut id = id;
        let mut hide = hide;
        let mut oamId: i32 = 0i32;
        let mut oamCount: i32 = 0i32;
        let mut i: i32 = 0i32;
        if ((((&raw mut sOamWork).cast::<u8>().cast::<*mut u8>()).read()) as usize) == 0usize {
            return;
        }
        if !((((((((&raw mut sOamWork).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(4)
            .cast::<*mut u8>())
        .read())
        .wrapping_offset(((id) as i32) as isize * 28))
        .read())
            != 0)
        {
            return;
        }
        oamCount = (((((((((&raw mut sOamWork).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(4)
            .cast::<*mut u8>())
        .read())
        .wrapping_offset(((id) as i32) as isize * 28))
        .wrapping_add(3))
        .read()) as i32)
            .wrapping_add(1i32);
        oamId = (((((((((&raw mut sOamWork).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(4)
            .cast::<*mut u8>())
        .read())
        .wrapping_offset(((id) as i32) as isize * 28))
        .wrapping_add(1))
        .read()) as i32);
        if (hide) != 0 {
            {
                i = 0i32;
                'l1: loop {
                    if !(i < oamCount) {
                        break 'l1;
                    }
                    'l2: {
                        crate::c::bf_write(
                            (((((&raw mut gMain).cast::<u8>()).wrapping_add(56)).cast::<u8>())
                                .wrapping_offset((oamId) as isize * 8))
                            .wrapping_add(1),
                            0,
                            2,
                            (2u32) as i32,
                        );
                    }
                    i = (i).wrapping_add(1);
                    oamId = (oamId).wrapping_add(1);
                }
            }
        } else {
            {
                i = 0i32;
                'l3: loop {
                    if !(i < oamCount) {
                        break 'l3;
                    }
                    'l4: {
                        crate::c::bf_write(
                            (((((&raw mut gMain).cast::<u8>()).wrapping_add(56)).cast::<u8>())
                                .wrapping_offset((oamId) as isize * 8))
                            .wrapping_add(1),
                            0,
                            2,
                            (0u32) as i32,
                        );
                    }
                    i = (i).wrapping_add(1);
                    oamId = (oamId).wrapping_add(1);
                }
            }
            DigitObjUtil_PrintNumOn(
                id,
                (((((((&raw mut sOamWork).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(4)
                    .cast::<*mut u8>())
                .read())
                .wrapping_offset(((id) as i32) as isize * 28))
                .wrapping_add(24)
                .cast::<i32>())
                .read(),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn GetFirstOamId(oamCount: u8) -> u8 {
    unsafe {
        let mut oamCount = oamCount;
        let mut i: u32 = 0u32;
        let mut firstOamId: u16 = 64u16;
        {
            i = 0u32;
            'l1: loop {
                if !(i
                    < ((((&raw mut sOamWork).cast::<u8>().cast::<*mut u8>()).read()).cast::<u32>())
                        .read())
                {
                    break 'l1;
                }
                'l2: {
                    if !((((((((&raw mut sOamWork).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(4)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_offset(((i) as i32) as isize * 28))
                    .read())
                        != 0)
                    {
                        if ((((((((((&raw mut sOamWork).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(4)
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_offset(((i) as i32) as isize * 28))
                        .wrapping_add(1))
                        .read()) as i32)
                            != 255i32)
                            && ((((((((((&raw mut sOamWork).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(4)
                            .cast::<*mut u8>())
                            .read())
                            .wrapping_offset(((i) as i32) as isize * 28))
                            .wrapping_add(3))
                            .read()) as i32)
                                <= ((oamCount) as i32))
                        {
                            return (((((((&raw mut sOamWork).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(4)
                            .cast::<*mut u8>())
                            .read())
                            .wrapping_offset(((i) as i32) as isize * 28))
                            .wrapping_add(1))
                            .read();
                        }
                    } else {
                        firstOamId = ((((firstOamId) as i32).wrapping_add(
                            (1i32).wrapping_add(
                                (((((((((&raw mut sOamWork).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(4)
                                .cast::<*mut u8>())
                                .read())
                                .wrapping_offset(((i) as i32) as isize * 28))
                                .wrapping_add(3))
                                .read()) as i32),
                            ),
                        )) as u16);
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        if (((firstOamId) as i32).wrapping_add(((oamCount) as i32))).wrapping_add(1i32) > 128i32 {
            return 255u8;
        } else {
            return ((firstOamId) as u8);
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
pub(crate) unsafe extern "C" fn SharesTileWithAnyActive(id: u32) -> u32 {
    unsafe {
        let mut id = id;
        let mut i: u32 = 0u32;
        {
            i = 0u32;
            'l1: loop {
                if !(i
                    < ((((&raw mut sOamWork).cast::<u8>().cast::<*mut u8>()).read()).cast::<u32>())
                        .read())
                {
                    break 'l1;
                }
                'l2: {
                    if (((((((((&raw mut sOamWork).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(4)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_offset(((i) as i32) as isize * 28))
                    .read())
                        != 0)
                        && (i != id))
                        && ((((((((((&raw mut sOamWork).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(4)
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_offset(((i) as i32) as isize * 28))
                        .wrapping_add(16)
                        .cast::<u16>())
                        .read()) as i32)
                            == (((((((((&raw mut sOamWork).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(4)
                            .cast::<*mut u8>())
                            .read())
                            .wrapping_offset(((id) as i32) as isize * 28))
                            .wrapping_add(16)
                            .cast::<u16>())
                            .read()) as i32))
                    {
                        return 1u32;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return 0u32;
    }
}
pub(crate) unsafe extern "C" fn SharesPalWithAnyActive(id: u32) -> u32 {
    unsafe {
        let mut id = id;
        let mut i: u32 = 0u32;
        {
            i = 0u32;
            'l1: loop {
                if !(i
                    < ((((&raw mut sOamWork).cast::<u8>().cast::<*mut u8>()).read()).cast::<u32>())
                        .read())
                {
                    break 'l1;
                }
                'l2: {
                    if (((((((((&raw mut sOamWork).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(4)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_offset(((i) as i32) as isize * 28))
                    .read())
                        != 0)
                        && (i != id))
                        && ((((((((((&raw mut sOamWork).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(4)
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_offset(((i) as i32) as isize * 28))
                        .wrapping_add(18)
                        .cast::<u16>())
                        .read()) as i32)
                            == (((((((((&raw mut sOamWork).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(4)
                            .cast::<*mut u8>())
                            .read())
                            .wrapping_offset(((id) as i32) as isize * 28))
                            .wrapping_add(18)
                            .cast::<u16>())
                            .read()) as i32))
                    {
                        return 1u32;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return 0u32;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetTilesPerImage(shape: u32, size: u32) -> u8 {
    unsafe {
        let mut shape = shape;
        let mut size = size;
        return ((((((&raw const sTilesPerImage).cast::<u8>().cast_mut()).cast::<u8>())
            .wrapping_offset(((shape) as i32) as isize * 4))
        .cast::<u8>())
        .wrapping_offset(((size) as i32) as isize))
        .read();
    }
}
