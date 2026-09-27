//! Translated from `src/mon_markings.c` by tools/rustport/c2rs.py, then reviewed.
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

// Data tables (translate with cdata.py): sMonMarkings_Pal sMonMarkings_Gfx sOamData_MenuWindow sOamData_8x8 sAnim_Marking_CircleOff sAnim_Marking_CircleOn sAnim_Marking_SquareOff sAnim_Marking_SquareOn sAnim_Marking_TriangleOff sAnim_Marking_TriangleOn sAnim_Marking_HeartOff sAnim_Marking_HeartOn sAnim_Cursor sAnim_OKCancelText sAnims_MenuSprite sAnim_MenuWindow_UpperHalf sAnim_MenuWindow_LowerHalf sAnims_MenuWindow sOamData_MarkingCombo sAnim_MarkingCombo_AllOff sAnim_MarkingCombo_Circle sAnim_MarkingCombo_Square sAnim_MarkingCombo_CircleSquare sAnim_MarkingCombo_Triangle sAnim_MarkingCombo_CircleTriangle sAnim_MarkingCombo_SquareTriangle sAnim_MarkingCombo_CircleSquareTriangle sAnim_MarkingCombo_Heart sAnim_MarkingCombo_CircleHeart sAnim_MarkingCombo_SquareHeart sAnim_MarkingCombo_CircleSquareHeart sAnim_MarkingCombo_TriangleHeart sAnim_MarkingCombo_CircleTriangleHeart sAnim_MarkingCombo_SquareTriangleHeart sAnim_MarkingCombo_AllOn sAnims_MarkingCombo
#[allow(unused_imports)]
use crate::data::mon_markings::*;

#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sMenu: *mut u8 = core::ptr::null_mut();

unsafe extern "C" {
    static mut gDummySpriteAffineAnimTable: u8;
    static mut gMain: u8;
    static mut gMonMarkingsMenu_Gfx: u8;
    static mut gMonMarkingsMenu_Pal: u8;
    static mut gSaveBlock2Ptr: u8;
    static mut gSprites: u8;
    fn CalcCenterToCornerVec(a0: *mut u8, a1: u8, a2: u8, a3: u8);
    fn CpuFastSet(a0: *mut u8, a1: *mut u8, a2: u32);
    fn CpuSet(a0: *mut u8, a1: *mut u8, a2: u32);
    fn CreateSprite(a0: *mut u8, a1: i16, a2: i16, a3: u8) -> u8;
    fn DestroySprite(a0: *mut u8);
    fn FreeSpritePaletteByTag(a0: u16);
    fn FreeSpriteTilesByTag(a0: u16);
    fn GetWindowFrameTilesPal(a0: u8) -> *mut u8;
    fn LoadSpritePalette(a0: *mut u8) -> u8;
    fn LoadSpritePalettes(a0: *mut u8);
    fn LoadSpriteSheet(a0: *mut u8) -> u16;
    fn LoadSpriteSheets(a0: *mut u8);
    fn PlaySE(a0: u16);
    fn RequestDma3Copy(a0: *mut u8, a1: *mut u8, a2: u16, a3: u8) -> i16;
    fn SpriteCallbackDummy(a0: *mut u8);
    fn StartSpriteAnim(a0: *mut u8, a1: u8);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn InitMonMarkingsMenu(ptr: *mut u8) {
    unsafe {
        let mut ptr = ptr;
        ((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).write(ptr);
    }
}
pub(crate) unsafe extern "C" fn BufferMenuWindowTiles() {
    unsafe {
        let mut frame: *mut u8 = GetWindowFrameTilesPal(
            ((crate::c::bf_read(
                (((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(20),
                3,
                5,
                false,
            ) as u16) as u8),
        );
        ((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(44)
            .cast::<*mut u8>())
        .write(((frame).cast::<*mut u8>()).read());
        ((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(48)
            .cast::<*mut u16>())
        .write(((frame).wrapping_add(4).cast::<*mut u16>()).read());
        ((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(4276)).write(0u8);
        'l1: loop {
            'l2: {
                {
                    let mut tmp: u16 = 0u16;
                    (&raw mut tmp).write_volatile(0u16);
                    'l3: loop {
                        'l4: {
                            CpuSet(
                                (&raw mut tmp).cast::<u8>(),
                                ((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                                    .wrapping_add(52))
                                .cast::<u8>(),
                                (16777216u32
                                    | (crate::c::div_u32(
                                        4096u32,
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
    }
}
pub(crate) unsafe extern "C" fn BufferMenuFrameTiles() -> u8 {
    unsafe {
        let mut i: u16 = 0u16;
        let mut dest: *mut u8 = (((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(52))
        .cast::<u8>())
        .wrapping_offset(
            (((((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(4276))
                .read()) as i32)
                .wrapping_mul(256i32)) as isize,
        );
        'l1: {
            let __sw1 = ((((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(4276))
            .read()) as i32);
            let __matched = __sw1 == 0i32 || __sw1 == 13i32 || __sw1 == 14i32;
            if __sw1 == 0i32 {
                'l2: loop {
                    'l3: {
                        CpuFastSet(
                            ((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(44)
                                .cast::<*mut u8>())
                            .read(),
                            dest,
                            ((crate::c::div_i32(
                                crate::c::div_i32(256i32, 8i32),
                                crate::c::div_i32(32i32, 8i32),
                            ) & 2097151i32) as u32),
                        );
                    }
                    if !((0i32) != 0) {
                        break 'l2;
                    }
                }
                {
                    i = 0u16;
                    'l4: loop {
                        if !(((i) as i32) < 6i32) {
                            break 'l4;
                        }
                        'l5: {
                            'l6: loop {
                                'l7: {
                                    CpuFastSet(
                                        (((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>())
                                            .read())
                                        .wrapping_add(44)
                                        .cast::<*mut u8>())
                                        .read())
                                        .wrapping_offset(
                                            (crate::c::div_i32(256i32, 8i32)) as isize,
                                        ),
                                        (dest).wrapping_offset(
                                            ((crate::c::div_i32(256i32, 8i32))
                                                .wrapping_mul(((i) as i32).wrapping_add(1i32)))
                                                as isize,
                                        ),
                                        ((crate::c::div_i32(
                                            crate::c::div_i32(256i32, 8i32),
                                            crate::c::div_i32(32i32, 8i32),
                                        ) & 2097151i32)
                                            as u32),
                                    );
                                }
                                if !((0i32) != 0) {
                                    break 'l6;
                                }
                            }
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                'l8: loop {
                    'l9: {
                        CpuFastSet(
                            (((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(44)
                                .cast::<*mut u8>())
                            .read())
                            .wrapping_offset(
                                ((crate::c::div_i32(256i32, 8i32)).wrapping_mul(2i32)) as isize,
                            ),
                            (dest).wrapping_offset(
                                ((crate::c::div_i32(256i32, 8i32)).wrapping_mul(7i32)) as isize,
                            ),
                            ((crate::c::div_i32(
                                crate::c::div_i32(256i32, 8i32),
                                crate::c::div_i32(32i32, 8i32),
                            ) & 2097151i32) as u32),
                        );
                    }
                    if !((0i32) != 0) {
                        break 'l8;
                    }
                }
                let __p2 =
                    (((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(4276);
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if !__matched {
                'l10: loop {
                    'l11: {
                        CpuFastSet(
                            (((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(44)
                                .cast::<*mut u8>())
                            .read())
                            .wrapping_offset(
                                ((crate::c::div_i32(256i32, 8i32)).wrapping_mul(3i32)) as isize,
                            ),
                            dest,
                            ((crate::c::div_i32(
                                crate::c::div_i32(256i32, 8i32),
                                crate::c::div_i32(32i32, 8i32),
                            ) & 2097151i32) as u32),
                        );
                    }
                    if !((0i32) != 0) {
                        break 'l10;
                    }
                }
                {
                    i = 0u16;
                    'l12: loop {
                        if !(((i) as i32) < 6i32) {
                            break 'l12;
                        }
                        'l13: {
                            'l14: loop {
                                'l15: {
                                    CpuFastSet(
                                        (((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>())
                                            .read())
                                        .wrapping_add(44)
                                        .cast::<*mut u8>())
                                        .read())
                                        .wrapping_offset(
                                            ((crate::c::div_i32(256i32, 8i32)).wrapping_mul(4i32))
                                                as isize,
                                        ),
                                        (dest).wrapping_offset(
                                            ((crate::c::div_i32(256i32, 8i32))
                                                .wrapping_mul(((i) as i32).wrapping_add(1i32)))
                                                as isize,
                                        ),
                                        ((crate::c::div_i32(
                                            crate::c::div_i32(256i32, 8i32),
                                            crate::c::div_i32(32i32, 8i32),
                                        ) & 2097151i32)
                                            as u32),
                                    );
                                }
                                if !((0i32) != 0) {
                                    break 'l14;
                                }
                            }
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                'l16: loop {
                    'l17: {
                        CpuFastSet(
                            (((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(44)
                                .cast::<*mut u8>())
                            .read())
                            .wrapping_offset(
                                ((crate::c::div_i32(256i32, 8i32)).wrapping_mul(5i32)) as isize,
                            ),
                            (dest).wrapping_offset(
                                ((crate::c::div_i32(256i32, 8i32)).wrapping_mul(7i32)) as isize,
                            ),
                            ((crate::c::div_i32(
                                crate::c::div_i32(256i32, 8i32),
                                crate::c::div_i32(32i32, 8i32),
                            ) & 2097151i32) as u32),
                        );
                    }
                    if !((0i32) != 0) {
                        break 'l16;
                    }
                }
                let __p3 =
                    (((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(4276);
                (__p3).write(((__p3).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 13i32 {
                'l18: loop {
                    'l19: {
                        CpuFastSet(
                            (((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(44)
                                .cast::<*mut u8>())
                            .read())
                            .wrapping_offset(
                                ((crate::c::div_i32(256i32, 8i32)).wrapping_mul(6i32)) as isize,
                            ),
                            dest,
                            ((crate::c::div_i32(
                                crate::c::div_i32(256i32, 8i32),
                                crate::c::div_i32(32i32, 8i32),
                            ) & 2097151i32) as u32),
                        );
                    }
                    if !((0i32) != 0) {
                        break 'l18;
                    }
                }
                {
                    i = 0u16;
                    'l20: loop {
                        if !(((i) as i32) < 6i32) {
                            break 'l20;
                        }
                        'l21: {
                            'l22: loop {
                                'l23: {
                                    CpuFastSet(
                                        (((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>())
                                            .read())
                                        .wrapping_add(44)
                                        .cast::<*mut u8>())
                                        .read())
                                        .wrapping_offset(
                                            ((crate::c::div_i32(256i32, 8i32)).wrapping_mul(7i32))
                                                as isize,
                                        ),
                                        (dest).wrapping_offset(
                                            ((crate::c::div_i32(256i32, 8i32))
                                                .wrapping_mul(((i) as i32).wrapping_add(1i32)))
                                                as isize,
                                        ),
                                        ((crate::c::div_i32(
                                            crate::c::div_i32(256i32, 8i32),
                                            crate::c::div_i32(32i32, 8i32),
                                        ) & 2097151i32)
                                            as u32),
                                    );
                                }
                                if !((0i32) != 0) {
                                    break 'l22;
                                }
                            }
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                'l24: loop {
                    'l25: {
                        CpuFastSet(
                            (((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(44)
                                .cast::<*mut u8>())
                            .read())
                            .wrapping_offset(
                                ((crate::c::div_i32(256i32, 8i32)).wrapping_mul(8i32)) as isize,
                            ),
                            (dest).wrapping_offset(
                                ((crate::c::div_i32(256i32, 8i32)).wrapping_mul(7i32)) as isize,
                            ),
                            ((crate::c::div_i32(
                                crate::c::div_i32(256i32, 8i32),
                                crate::c::div_i32(32i32, 8i32),
                            ) & 2097151i32) as u32),
                        );
                    }
                    if !((0i32) != 0) {
                        break 'l24;
                    }
                }
                let __p4 =
                    (((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(4276);
                (__p4).write(((__p4).read()).wrapping_add(1));
                return 0u8;
            }
            if __sw1 == 14i32 {
                return 0u8;
            }
        }
        return 1u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BufferMonMarkingsMenuTiles() {
    unsafe {
        BufferMenuWindowTiles();
        'l1: loop {
            if !((BufferMenuFrameTiles()) != 0) {
                break 'l1;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn OpenMonMarkingsMenu(markings: u8, x: i16, y: i16) {
    unsafe {
        let mut markings = markings;
        let mut x = x;
        let mut y = y;
        let mut i: u16 = 0u16;
        ((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(5)
            .cast::<i8>())
        .write(0i8);
        ((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(4))
            .write(markings);
        {
            i = 0u16;
            'l1: loop {
                if !(((i) as i32) < 4i32) {
                    break 'l1;
                }
                'l2: {
                    ((((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(6))
                    .cast::<u8>())
                    .wrapping_offset(((i) as i32) as isize))
                    .write(
                        ((crate::c::shr_i32(
                            ((((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(4))
                            .read()) as i32),
                            ((i) as u32),
                        ) & 1i32) as u8),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        CreateMonMarkingsMenuSprites(
            x,
            y,
            ((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read()).cast::<u16>()).read(),
            ((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(2)
                .cast::<u16>())
            .read(),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FreeMonMarkingsMenu() {
    unsafe {
        let mut i: u16 = 0u16;
        {
            i = 0u16;
            'l1: loop {
                if !(((i) as i32) < 2i32) {
                    break 'l1;
                }
                'l2: {
                    FreeSpriteTilesByTag(
                        ((((((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                            .cast::<u16>())
                        .read()) as i32)
                            .wrapping_add(((i) as i32))) as u16),
                    );
                    FreeSpritePaletteByTag(
                        ((((((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(2)
                            .cast::<u16>())
                        .read()) as i32)
                            .wrapping_add(((i) as i32))) as u16),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        {
            i = 0u16;
            'l3: loop {
                if !(((i) as u32) < crate::c::div_u32(8u32, 4u32)) {
                    break 'l3;
                }
                'l4: {
                    if !(!(((((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(12))
                    .cast::<*mut u8>())
                    .wrapping_offset(((i) as i32) as isize))
                    .read())
                    .is_null())
                    {
                        return;
                    }
                    DestroySprite(
                        ((((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(12))
                        .cast::<*mut u8>())
                        .wrapping_offset(((i) as i32) as isize))
                        .read(),
                    );
                    ((((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(12))
                    .cast::<*mut u8>())
                    .wrapping_offset(((i) as i32) as isize))
                    .write(core::ptr::null_mut());
                }
                i = (i).wrapping_add(1);
            }
        }
        {
            i = 0u16;
            'l5: loop {
                if !(((i) as i32) < 4i32) {
                    break 'l5;
                }
                'l6: {
                    if !(!(((((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(20))
                    .cast::<*mut u8>())
                    .wrapping_offset(((i) as i32) as isize))
                    .read())
                    .is_null())
                    {
                        return;
                    }
                    DestroySprite(
                        ((((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(20))
                        .cast::<*mut u8>())
                        .wrapping_offset(((i) as i32) as isize))
                        .read(),
                    );
                    ((((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(20))
                    .cast::<*mut u8>())
                    .wrapping_offset(((i) as i32) as isize))
                    .write(core::ptr::null_mut());
                }
                i = (i).wrapping_add(1);
            }
        }
        if !(((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(36)
            .cast::<*mut u8>())
        .read())
        .is_null()
        {
            DestroySprite(
                ((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(36)
                    .cast::<*mut u8>())
                .read(),
            );
            ((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(36)
                .cast::<*mut u8>())
            .write(core::ptr::null_mut());
        }
        if !(((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(40)
            .cast::<*mut u8>())
        .read())
        .is_null()
        {
            DestroySprite(
                ((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(40)
                    .cast::<*mut u8>())
                .read(),
            );
            ((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(40)
                .cast::<*mut u8>())
            .write(core::ptr::null_mut());
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn HandleMonMarkingsMenuInput() -> u8 {
    unsafe {
        let mut i: u16 = 0u16;
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(46)
            .cast::<u16>())
        .read()) as i32)
            & 64i32)
            != 0
        {
            PlaySE(5u16);
            if (({
                let __p1 = (((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(5)
                    .cast::<i8>();
                let __t2 = ((__p1).read()).wrapping_sub(1);
                (__p1).write(__t2);
                __t2
            }) as i32)
                < 0i32
            {
                ((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(5)
                    .cast::<i8>())
                .write(5i8);
            }
            return 1u8;
        }
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(46)
            .cast::<u16>())
        .read()) as i32)
            & 128i32)
            != 0
        {
            PlaySE(5u16);
            if (({
                let __p3 = (((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(5)
                    .cast::<i8>();
                let __t4 = ((__p3).read()).wrapping_add(1);
                (__p3).write(__t4);
                __t4
            }) as i32)
                > 5i32
            {
                ((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(5)
                    .cast::<i8>())
                .write(0i8);
            }
            return 1u8;
        }
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(46)
            .cast::<u16>())
        .read()) as i32)
            & 1i32)
            != 0
        {
            PlaySE(5u16);
            'l1: {
                let __sw5 = ((((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(5)
                    .cast::<i8>())
                .read()) as i32);
                if __sw5 == 4i32 {
                    ((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(4))
                        .write(0u8);
                    {
                        i = 0u16;
                        'l2: loop {
                            if !(((i) as i32) < 4i32) {
                                break 'l2;
                            }
                            'l3: {
                                let __p6 = (((&raw mut sMenu).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(4);
                                (__p6).write(
                                    (((((__p6).read()) as i32)
                                        | crate::c::shl_i32(
                                            ((((((((&raw mut sMenu)
                                                .cast::<u8>()
                                                .cast::<*mut u8>())
                                            .read())
                                            .wrapping_add(6))
                                            .cast::<u8>())
                                            .wrapping_offset(((i) as i32) as isize))
                                            .read())
                                                as i32),
                                            ((i) as u32),
                                        )) as u8),
                                );
                            }
                            i = (i).wrapping_add(1);
                        }
                    }
                    return 0u8;
                }
                if __sw5 == 5i32 {
                    return 0u8;
                }
            }
            ((((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(6))
                .cast::<u8>())
            .wrapping_offset(
                ((((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(5)
                    .cast::<i8>())
                .read()) as i32) as isize,
            ))
            .write(
                ((!((((((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(6))
                .cast::<u8>())
                .wrapping_offset(
                    ((((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(5)
                        .cast::<i8>())
                    .read()) as i32) as isize,
                ))
                .read())
                    != 0)) as u8),
            );
            return 1u8;
        }
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(46)
            .cast::<u16>())
        .read()) as i32)
            & 2i32)
            != 0
        {
            PlaySE(5u16);
            return 0u8;
        }
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn CreateMonMarkingsMenuSprites(
    x: i16,
    y: i16,
    baseTileTag: u16,
    basePaletteTag: u16,
) {
    unsafe {
        let mut x = x;
        let mut y = y;
        let mut baseTileTag = baseTileTag;
        let mut basePaletteTag = basePaletteTag;
        let mut i: u16 = 0u16;
        let mut spriteId: u8 = 0u8;
        let mut sheets = crate::ffi::Align4([0u8; 24]);
        (&raw mut sheets)
            .cast::<u8>()
            .wrapping_add(0)
            .wrapping_add(0)
            .cast::<*mut u8>()
            .write(
                ((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(52))
                    .cast::<u8>(),
            );
        (&raw mut sheets)
            .cast::<u8>()
            .wrapping_add(0)
            .wrapping_add(4)
            .cast::<u16>()
            .write(4096u16);
        (&raw mut sheets)
            .cast::<u8>()
            .wrapping_add(0)
            .wrapping_add(6)
            .cast::<u16>()
            .write(baseTileTag);
        (&raw mut sheets)
            .cast::<u8>()
            .wrapping_add(8)
            .wrapping_add(0)
            .cast::<*mut u8>()
            .write((&raw mut gMonMarkingsMenu_Gfx).cast::<u8>());
        (&raw mut sheets)
            .cast::<u8>()
            .wrapping_add(8)
            .wrapping_add(4)
            .cast::<u16>()
            .write(800u16);
        (&raw mut sheets)
            .cast::<u8>()
            .wrapping_add(8)
            .wrapping_add(6)
            .cast::<u16>()
            .write(((((baseTileTag) as i32).wrapping_add(1i32)) as u16));
        let mut palettes = crate::ffi::Align4([0u8; 24]);
        (&raw mut palettes)
            .cast::<u8>()
            .wrapping_add(0)
            .wrapping_add(0)
            .cast::<*mut u16>()
            .write(
                ((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(48)
                    .cast::<*mut u16>())
                .read(),
            );
        (&raw mut palettes)
            .cast::<u8>()
            .wrapping_add(0)
            .wrapping_add(4)
            .cast::<u16>()
            .write(basePaletteTag);
        (&raw mut palettes)
            .cast::<u8>()
            .wrapping_add(8)
            .wrapping_add(0)
            .cast::<*mut u16>()
            .write(((&raw mut gMonMarkingsMenu_Pal).cast::<u16>()).cast::<u16>());
        (&raw mut palettes)
            .cast::<u8>()
            .wrapping_add(8)
            .wrapping_add(4)
            .cast::<u16>()
            .write(((((basePaletteTag) as i32).wrapping_add(1i32)) as u16));
        let mut template = crate::ffi::Align4([0u8; 24]);
        (&raw mut template)
            .cast::<u8>()
            .wrapping_add(0)
            .cast::<u16>()
            .write(baseTileTag);
        (&raw mut template)
            .cast::<u8>()
            .wrapping_add(2)
            .cast::<u16>()
            .write(basePaletteTag);
        (&raw mut template)
            .cast::<u8>()
            .wrapping_add(4)
            .cast::<*mut u8>()
            .write((&raw const sOamData_MenuWindow).cast::<u8>().cast_mut());
        (&raw mut template)
            .cast::<u8>()
            .wrapping_add(8)
            .cast::<*mut *mut u8>()
            .write(
                ((&raw const sAnims_MenuWindow)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<*mut u8>())
                .cast::<*mut u8>(),
            );
        (&raw mut template)
            .cast::<u8>()
            .wrapping_add(12)
            .cast::<*mut u8>()
            .write(core::ptr::null_mut());
        (&raw mut template)
            .cast::<u8>()
            .wrapping_add(16)
            .cast::<*mut *mut u8>()
            .write(((&raw mut gDummySpriteAffineAnimTable).cast::<*mut u8>()).cast::<*mut u8>());
        (&raw mut template)
            .cast::<u8>()
            .wrapping_add(20)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>()
            .write(Some(SpriteCB_Dummy));
        LoadSpriteSheets((&raw mut sheets).cast::<u8>());
        LoadSpritePalettes((&raw mut palettes).cast::<u8>());
        {
            i = 0u16;
            'l1: loop {
                if !(((i) as u32) < crate::c::div_u32(8u32, 4u32)) {
                    break 'l1;
                }
                'l2: {
                    spriteId = CreateSprite(
                        (&raw mut template).cast::<u8>(),
                        ((((x) as i32).wrapping_add(32i32)) as i16),
                        ((((y) as i32).wrapping_add(32i32)) as i16),
                        1u8,
                    );
                    if ((spriteId) as i32) != 64i32 {
                        ((((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(12))
                        .cast::<*mut u8>())
                        .wrapping_offset(((i) as i32) as isize))
                        .write(
                            ((&raw mut gSprites).cast::<u8>())
                                .wrapping_offset(((spriteId) as i32) as isize * 68),
                        );
                        StartSpriteAnim(
                            ((&raw mut gSprites).cast::<u8>())
                                .wrapping_offset(((spriteId) as i32) as isize * 68),
                            ((i) as u8),
                        );
                    } else {
                        ((((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(12))
                        .cast::<*mut u8>())
                        .wrapping_offset(((i) as i32) as isize))
                        .write(core::ptr::null_mut());
                        return;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        ((((((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(12))
            .cast::<*mut u8>())
        .wrapping_offset(1))
        .read())
        .wrapping_add(34)
        .cast::<i16>())
        .write(((((y) as i32).wrapping_add(96i32)) as i16));
        let __p1 = ((&raw mut template).cast::<u8>()).cast::<u16>();
        (__p1).write(((__p1).read()).wrapping_add(1));
        let __p2 = ((&raw mut template).cast::<u8>())
            .wrapping_add(2)
            .cast::<u16>();
        (__p2).write(((__p2).read()).wrapping_add(1));
        (((&raw mut template).cast::<u8>())
            .wrapping_add(8)
            .cast::<*mut *mut u8>())
        .write(
            ((&raw const sAnims_MenuSprite)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u8>())
            .cast::<*mut u8>(),
        );
        (((&raw mut template).cast::<u8>())
            .wrapping_add(20)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(SpriteCB_Marking));
        (((&raw mut template).cast::<u8>())
            .wrapping_add(4)
            .cast::<*mut u8>())
        .write((&raw const sOamData_8x8).cast::<u8>().cast_mut());
        {
            i = 0u16;
            'l3: loop {
                if !(((i) as i32) < 4i32) {
                    break 'l3;
                }
                'l4: {
                    spriteId = CreateSprite(
                        (&raw mut template).cast::<u8>(),
                        ((((x) as i32).wrapping_add(32i32)) as i16),
                        (((((y) as i32).wrapping_add(16i32))
                            .wrapping_add((16i32).wrapping_mul(((i) as i32))))
                            as i16),
                        0u8,
                    );
                    if ((spriteId) as i32) != 64i32 {
                        ((((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(20))
                        .cast::<*mut u8>())
                        .wrapping_offset(((i) as i32) as isize))
                        .write(
                            ((&raw mut gSprites).cast::<u8>())
                                .wrapping_offset(((spriteId) as i32) as isize * 68),
                        );
                        (((((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((spriteId) as i32) as isize * 68))
                        .wrapping_add(46))
                        .cast::<i16>())
                        .write(((i) as i16));
                    } else {
                        ((((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(20))
                        .cast::<*mut u8>())
                        .wrapping_offset(((i) as i32) as isize))
                        .write(core::ptr::null_mut());
                        return;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        (((&raw mut template).cast::<u8>())
            .wrapping_add(20)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(SpriteCallbackDummy));
        spriteId = CreateSprite((&raw mut template).cast::<u8>(), 0i16, 0i16, 0u8);
        if ((spriteId) as i32) != 64i32 {
            ((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(40)
                .cast::<*mut u8>())
            .write(
                ((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68),
            );
            crate::c::bf_write(
                (((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(40)
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(1),
                6,
                2,
                (0u32) as i32,
            );
            crate::c::bf_write(
                (((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(40)
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(3),
                6,
                2,
                (2u32) as i32,
            );
            StartSpriteAnim(
                ((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(40)
                    .cast::<*mut u8>())
                .read(),
                9u8,
            );
            ((((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(40)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(32)
            .cast::<i16>())
            .write(((((x) as i32).wrapping_add(32i32)) as i16));
            ((((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(40)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(34)
            .cast::<i16>())
            .write(((((y) as i32).wrapping_add(80i32)) as i16));
            CalcCenterToCornerVec(
                ((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(40)
                    .cast::<*mut u8>())
                .read(),
                1u8,
                2u8,
                0u8,
            );
        } else {
            ((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(40)
                .cast::<*mut u8>())
            .write(core::ptr::null_mut());
        }
        (((&raw mut template).cast::<u8>())
            .wrapping_add(20)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(SpriteCB_Cursor));
        spriteId = CreateSprite(
            (&raw mut template).cast::<u8>(),
            ((((x) as i32).wrapping_add(12i32)) as i16),
            0i16,
            0u8,
        );
        if ((spriteId) as i32) != 64i32 {
            ((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(36)
                .cast::<*mut u8>())
            .write(
                ((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68),
            );
            (((((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(36)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(46))
            .cast::<i16>())
            .write(((((y) as i32).wrapping_add(16i32)) as i16));
            StartSpriteAnim(
                ((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(36)
                    .cast::<*mut u8>())
                .read(),
                8u8,
            );
        } else {
            ((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(36)
                .cast::<*mut u8>())
            .write(core::ptr::null_mut());
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_Dummy(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_Marking(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (((((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(6))
            .cast::<u8>())
        .wrapping_offset((((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) as isize))
        .read())
            != 0
        {
            StartSpriteAnim(
                sprite,
                ((((2i32)
                    .wrapping_mul((((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32)))
                .wrapping_add(1i32)) as u8),
            );
        } else {
            StartSpriteAnim(
                sprite,
                (((2i32)
                    .wrapping_mul((((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32)))
                    as u8),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_Cursor(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        ((sprite).wrapping_add(34).cast::<i16>()).write(
            ((((16i32).wrapping_mul(
                ((((((&raw mut sMenu).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(5)
                    .cast::<i8>())
                .read()) as i32),
            ))
            .wrapping_add((((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32)))
                as i16),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CreateMonMarkingAllCombosSprite(
    tileTag: u16,
    paletteTag: u16,
    palette: *mut u16,
) -> *mut u8 {
    unsafe {
        let mut tileTag = tileTag;
        let mut paletteTag = paletteTag;
        let mut palette = palette;
        if !(!(palette).is_null()) {
            palette = ((&raw const sMonMarkings_Pal)
                .cast::<u8>()
                .cast_mut()
                .cast::<u16>())
            .cast::<u16>();
        }
        return CreateMarkingComboSprite(tileTag, paletteTag, palette, 16u16);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CreateMonMarkingComboSprite(
    tileTag: u16,
    paletteTag: u16,
    palette: *mut u16,
) -> *mut u8 {
    unsafe {
        let mut tileTag = tileTag;
        let mut paletteTag = paletteTag;
        let mut palette = palette;
        if !(!(palette).is_null()) {
            palette = ((&raw const sMonMarkings_Pal)
                .cast::<u8>()
                .cast_mut()
                .cast::<u16>())
            .cast::<u16>();
        }
        return CreateMarkingComboSprite(tileTag, paletteTag, palette, 1u16);
    }
}
pub(crate) unsafe extern "C" fn CreateMarkingComboSprite(
    tileTag: u16,
    paletteTag: u16,
    palette: *mut u16,
    size: u16,
) -> *mut u8 {
    unsafe {
        let mut tileTag = tileTag;
        let mut paletteTag = paletteTag;
        let mut palette = palette;
        let mut size = size;
        let mut spriteId: u8 = 0u8;
        let mut template = crate::ffi::Align4([0u8; 24]);
        let mut sheet = crate::ffi::Align4([0u8; 8]);
        (&raw mut sheet)
            .cast::<u8>()
            .wrapping_add(0)
            .cast::<*mut u8>()
            .write(((&raw const sMonMarkings_Gfx).cast::<u8>().cast_mut()).cast::<u8>());
        (&raw mut sheet)
            .cast::<u8>()
            .wrapping_add(4)
            .cast::<u16>()
            .write(128u16);
        (&raw mut sheet)
            .cast::<u8>()
            .wrapping_add(6)
            .cast::<u16>()
            .write(tileTag);
        let mut sprPalette = crate::ffi::Align4([0u8; 8]);
        (&raw mut sprPalette)
            .cast::<u8>()
            .wrapping_add(0)
            .cast::<*mut u16>()
            .write(palette);
        (&raw mut sprPalette)
            .cast::<u8>()
            .wrapping_add(4)
            .cast::<u16>()
            .write(paletteTag);
        (((&raw mut template).cast::<u8>()).cast::<u16>()).write(tileTag);
        (((&raw mut template).cast::<u8>())
            .wrapping_add(2)
            .cast::<u16>())
        .write(paletteTag);
        (((&raw mut template).cast::<u8>())
            .wrapping_add(4)
            .cast::<*mut u8>())
        .write((&raw const sOamData_MarkingCombo).cast::<u8>().cast_mut());
        (((&raw mut template).cast::<u8>())
            .wrapping_add(8)
            .cast::<*mut *mut u8>())
        .write(
            ((&raw const sAnims_MarkingCombo)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u8>())
            .cast::<*mut u8>(),
        );
        (((&raw mut template).cast::<u8>())
            .wrapping_add(12)
            .cast::<*mut u8>())
        .write(core::ptr::null_mut());
        (((&raw mut template).cast::<u8>())
            .wrapping_add(16)
            .cast::<*mut *mut u8>())
        .write(((&raw mut gDummySpriteAffineAnimTable).cast::<*mut u8>()).cast::<*mut u8>());
        (((&raw mut template).cast::<u8>())
            .wrapping_add(20)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(SpriteCB_Dummy));
        (((&raw mut sheet).cast::<u8>())
            .wrapping_add(4)
            .cast::<u16>())
        .write(((((size) as i32).wrapping_mul(128i32)) as u16));
        LoadSpriteSheet((&raw mut sheet).cast::<u8>());
        LoadSpritePalette((&raw mut sprPalette).cast::<u8>());
        spriteId = CreateSprite((&raw mut template).cast::<u8>(), 0i16, 0i16, 0u8);
        if ((spriteId) as i32) != 64i32 {
            return ((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId) as i32) as isize * 68);
        } else {
            return core::ptr::null_mut();
        }
        #[allow(unreachable_code)]
        {
            return core::ptr::null_mut();
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn UpdateMonMarkingTiles(markings: u8, dest: *mut u8) {
    unsafe {
        let mut markings = markings;
        let mut dest = dest;
        RequestDma3Copy(
            (((&raw const sMonMarkings_Gfx).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset((((markings) as i32).wrapping_mul(128i32)) as isize),
            dest,
            128u16,
            16u8,
        );
    }
}
