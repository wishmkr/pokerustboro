//! Translated from `src/palette_util.c` by tools/rustport/c2rs.py, then reviewed.
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

unsafe extern "C" {
    static mut gPaletteFade: u8;
    static mut gPlttBufferFaded: u8;
    static mut gPlttBufferUnfaded: u8;
    fn BlendPalette(a0: u16, a1: u16, a2: u8, a3: u16);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn RouletteFlash_Reset(flash: *mut u8) {
    unsafe {
        let mut flash = flash;
        (flash).write(0u8);
        ((flash).wrapping_add(2).cast::<u16>()).write(0u16);
        crate::c::memset(
            (((flash).wrapping_add(4)).cast::<u8>()).cast::<u8>(),
            0i32,
            192u32,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn RouletteFlash_Add(flash: *mut u8, id: u8, settings: *mut u8) -> u8 {
    unsafe {
        let mut flash = flash;
        let mut id = id;
        let mut settings = settings;
        if (((id) as u32) >= crate::c::div_u32(192u32, 12u32))
            || ((crate::c::bf_read(
                ((((flash).wrapping_add(4)).cast::<u8>())
                    .wrapping_offset(((id) as i32) as isize * 12))
                .wrapping_add(0),
                7,
                1,
                false,
            ) as u8)
                != 0)
        {
            return 255u8;
        }
        ((((((flash).wrapping_add(4)).cast::<u8>()).wrapping_offset(((id) as i32) as isize * 12))
            .wrapping_add(4))
        .cast::<u16>())
        .write(((settings).cast::<u16>()).read());
        ((((((flash).wrapping_add(4)).cast::<u8>()).wrapping_offset(((id) as i32) as isize * 12))
            .wrapping_add(4))
        .wrapping_add(2)
        .cast::<u16>())
        .write(((settings).wrapping_add(2).cast::<u16>()).read());
        ((((((flash).wrapping_add(4)).cast::<u8>()).wrapping_offset(((id) as i32) as isize * 12))
            .wrapping_add(4))
        .wrapping_add(4))
        .write(((settings).wrapping_add(4)).read());
        ((((((flash).wrapping_add(4)).cast::<u8>()).wrapping_offset(((id) as i32) as isize * 12))
            .wrapping_add(4))
        .wrapping_add(5))
        .write(((settings).wrapping_add(5)).read());
        ((((((flash).wrapping_add(4)).cast::<u8>()).wrapping_offset(((id) as i32) as isize * 12))
            .wrapping_add(4))
        .wrapping_add(6)
        .cast::<i8>())
        .write(((settings).wrapping_add(6).cast::<i8>()).read());
        crate::c::bf_write(
            (((((flash).wrapping_add(4)).cast::<u8>())
                .wrapping_offset(((id) as i32) as isize * 12))
            .wrapping_add(4))
            .wrapping_add(7),
            0,
            5,
            (crate::c::bf_read((settings).wrapping_add(7), 0, 5, true) as i8) as i32,
        );
        crate::c::bf_write(
            (((((flash).wrapping_add(4)).cast::<u8>())
                .wrapping_offset(((id) as i32) as isize * 12))
            .wrapping_add(4))
            .wrapping_add(7),
            5,
            2,
            (crate::c::bf_read((settings).wrapping_add(7), 5, 2, true) as i8) as i32,
        );
        crate::c::bf_write(
            (((((flash).wrapping_add(4)).cast::<u8>())
                .wrapping_offset(((id) as i32) as isize * 12))
            .wrapping_add(4))
            .wrapping_add(7),
            7,
            1,
            (crate::c::bf_read((settings).wrapping_add(7), 7, 1, true) as i8) as i32,
        );
        crate::c::bf_write(
            ((((flash).wrapping_add(4)).cast::<u8>()).wrapping_offset(((id) as i32) as isize * 12))
                .wrapping_add(0),
            0,
            7,
            (0u8) as i32,
        );
        crate::c::bf_write(
            ((((flash).wrapping_add(4)).cast::<u8>()).wrapping_offset(((id) as i32) as isize * 12))
                .wrapping_add(0),
            7,
            1,
            (1u8) as i32,
        );
        (((((flash).wrapping_add(4)).cast::<u8>()).wrapping_offset(((id) as i32) as isize * 12))
            .wrapping_add(2)
            .cast::<i8>())
        .write(0i8);
        (((((flash).wrapping_add(4)).cast::<u8>()).wrapping_offset(((id) as i32) as isize * 12))
            .wrapping_add(1))
        .write(0u8);
        if ((crate::c::bf_read(
            (((((flash).wrapping_add(4)).cast::<u8>())
                .wrapping_offset(((id) as i32) as isize * 12))
            .wrapping_add(4))
            .wrapping_add(7),
            7,
            1,
            true,
        ) as i8) as i32)
            < 0i32
        {
            (((((flash).wrapping_add(4)).cast::<u8>())
                .wrapping_offset(((id) as i32) as isize * 12))
            .wrapping_add(3)
            .cast::<i8>())
            .write((-1i8));
        } else {
            (((((flash).wrapping_add(4)).cast::<u8>())
                .wrapping_offset(((id) as i32) as isize * 12))
            .wrapping_add(3)
            .cast::<i8>())
            .write(1i8);
        }
        return id;
    }
}
pub(crate) unsafe extern "C" fn RouletteFlash_Remove(flash: *mut u8, id: u8) -> u8 {
    unsafe {
        let mut flash = flash;
        let mut id = id;
        if ((id) as u32) >= crate::c::div_u32(192u32, 12u32) {
            return 255u8;
        }
        if !((crate::c::bf_read(
            ((((flash).wrapping_add(4)).cast::<u8>()).wrapping_offset(((id) as i32) as isize * 12))
                .wrapping_add(0),
            7,
            1,
            false,
        ) as u8)
            != 0)
        {
            return 255u8;
        }
        crate::c::memset(
            (((flash).wrapping_add(4)).cast::<u8>()).wrapping_offset(((id) as i32) as isize * 12),
            0i32,
            12u32,
        );
        return id;
    }
}
pub(crate) unsafe extern "C" fn RouletteFlash_FadePalette(pal: *mut u8) -> u8 {
    unsafe {
        let mut pal = pal;
        let mut i: u8 = 0u8;
        let mut returnval: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < (((((pal).wrapping_add(4)).wrapping_add(4)).read()) as i32)) {
                    break 'l1;
                }
                'l2: {
                    let mut faded: *mut u8 = ((((&raw mut gPlttBufferFaded).cast::<u16>())
                        .cast::<u16>())
                    .wrapping_offset(
                        ((((((pal).wrapping_add(4)).wrapping_add(2).cast::<u16>()).read()) as i32)
                            .wrapping_add(((i) as i32))) as isize,
                    ))
                    .cast::<u8>();
                    let mut unfaded: *mut u8 = ((((&raw mut gPlttBufferUnfaded).cast::<u16>())
                        .cast::<u16>())
                    .wrapping_offset(
                        ((((((pal).wrapping_add(4)).wrapping_add(2).cast::<u16>()).read()) as i32)
                            .wrapping_add(((i) as i32))) as isize,
                    ))
                    .cast::<u8>();
                    'l3: {
                        let __sw1 =
                            ((crate::c::bf_read((pal).wrapping_add(0), 0, 7, false) as u8) as i32);
                        if __sw1 == 1i32 {
                            if (((crate::c::bf_read((faded).wrapping_add(0), 0, 5, false) as u16)
                                as i32)
                                .wrapping_add(
                                    ((((pal).wrapping_add(3).cast::<i8>()).read()) as i32),
                                )
                                >= 0i32)
                                && (((crate::c::bf_read((faded).wrapping_add(0), 0, 5, false)
                                    as u16) as i32)
                                    .wrapping_add(
                                        ((((pal).wrapping_add(3).cast::<i8>()).read()) as i32),
                                    )
                                    < 32i32)
                            {
                                crate::c::bf_write(
                                    (faded).wrapping_add(0),
                                    0,
                                    5,
                                    ((((crate::c::bf_read((faded).wrapping_add(0), 0, 5, false)
                                        as u16) as i32)
                                        .wrapping_add(
                                            ((((pal).wrapping_add(3).cast::<i8>()).read()) as i32),
                                        )) as u16) as i32,
                                );
                            }
                            if (((crate::c::bf_read((faded).wrapping_add(0), 5, 5, false) as u16)
                                as i32)
                                .wrapping_add(
                                    ((((pal).wrapping_add(3).cast::<i8>()).read()) as i32),
                                )
                                >= 0i32)
                                && (((crate::c::bf_read((faded).wrapping_add(0), 5, 5, false)
                                    as u16) as i32)
                                    .wrapping_add(
                                        ((((pal).wrapping_add(3).cast::<i8>()).read()) as i32),
                                    )
                                    < 32i32)
                            {
                                crate::c::bf_write(
                                    (faded).wrapping_add(0),
                                    5,
                                    5,
                                    ((((crate::c::bf_read((faded).wrapping_add(0), 5, 5, false)
                                        as u16) as i32)
                                        .wrapping_add(
                                            ((((pal).wrapping_add(3).cast::<i8>()).read()) as i32),
                                        )) as u16) as i32,
                                );
                            }
                            if (((crate::c::bf_read((faded).wrapping_add(1), 2, 5, false) as u16)
                                as i32)
                                .wrapping_add(
                                    ((((pal).wrapping_add(3).cast::<i8>()).read()) as i32),
                                )
                                >= 0i32)
                                && (((crate::c::bf_read((faded).wrapping_add(1), 2, 5, false)
                                    as u16) as i32)
                                    .wrapping_add(
                                        ((((pal).wrapping_add(3).cast::<i8>()).read()) as i32),
                                    )
                                    < 32i32)
                            {
                                crate::c::bf_write(
                                    (faded).wrapping_add(1),
                                    2,
                                    5,
                                    ((((crate::c::bf_read((faded).wrapping_add(1), 2, 5, false)
                                        as u16) as i32)
                                        .wrapping_add(
                                            ((((pal).wrapping_add(3).cast::<i8>()).read()) as i32),
                                        )) as u16) as i32,
                                );
                            }
                            break 'l3;
                        }
                        if __sw1 == 2i32 {
                            if ((((pal).wrapping_add(3).cast::<i8>()).read()) as i32) < 0i32 {
                                if ((crate::c::bf_read((faded).wrapping_add(0), 0, 5, false) as u16)
                                    as i32)
                                    .wrapping_add(
                                        ((((pal).wrapping_add(3).cast::<i8>()).read()) as i32),
                                    )
                                    >= ((crate::c::bf_read((unfaded).wrapping_add(0), 0, 5, false)
                                        as u16) as i32)
                                {
                                    crate::c::bf_write(
                                        (faded).wrapping_add(0),
                                        0,
                                        5,
                                        ((((crate::c::bf_read((faded).wrapping_add(0), 0, 5, false)
                                            as u16)
                                            as i32)
                                            .wrapping_add(
                                                ((((pal).wrapping_add(3).cast::<i8>()).read())
                                                    as i32),
                                            )) as u16)
                                            as i32,
                                    );
                                }
                                if ((crate::c::bf_read((faded).wrapping_add(0), 5, 5, false) as u16)
                                    as i32)
                                    .wrapping_add(
                                        ((((pal).wrapping_add(3).cast::<i8>()).read()) as i32),
                                    )
                                    >= ((crate::c::bf_read((unfaded).wrapping_add(0), 5, 5, false)
                                        as u16) as i32)
                                {
                                    crate::c::bf_write(
                                        (faded).wrapping_add(0),
                                        5,
                                        5,
                                        ((((crate::c::bf_read((faded).wrapping_add(0), 5, 5, false)
                                            as u16)
                                            as i32)
                                            .wrapping_add(
                                                ((((pal).wrapping_add(3).cast::<i8>()).read())
                                                    as i32),
                                            )) as u16)
                                            as i32,
                                    );
                                }
                                if ((crate::c::bf_read((faded).wrapping_add(1), 2, 5, false) as u16)
                                    as i32)
                                    .wrapping_add(
                                        ((((pal).wrapping_add(3).cast::<i8>()).read()) as i32),
                                    )
                                    >= ((crate::c::bf_read((unfaded).wrapping_add(1), 2, 5, false)
                                        as u16) as i32)
                                {
                                    crate::c::bf_write(
                                        (faded).wrapping_add(1),
                                        2,
                                        5,
                                        ((((crate::c::bf_read((faded).wrapping_add(1), 2, 5, false)
                                            as u16)
                                            as i32)
                                            .wrapping_add(
                                                ((((pal).wrapping_add(3).cast::<i8>()).read())
                                                    as i32),
                                            )) as u16)
                                            as i32,
                                    );
                                }
                            } else {
                                if ((crate::c::bf_read((faded).wrapping_add(0), 0, 5, false) as u16)
                                    as i32)
                                    .wrapping_add(
                                        ((((pal).wrapping_add(3).cast::<i8>()).read()) as i32),
                                    )
                                    <= ((crate::c::bf_read((unfaded).wrapping_add(0), 0, 5, false)
                                        as u16) as i32)
                                {
                                    crate::c::bf_write(
                                        (faded).wrapping_add(0),
                                        0,
                                        5,
                                        ((((crate::c::bf_read((faded).wrapping_add(0), 0, 5, false)
                                            as u16)
                                            as i32)
                                            .wrapping_add(
                                                ((((pal).wrapping_add(3).cast::<i8>()).read())
                                                    as i32),
                                            )) as u16)
                                            as i32,
                                    );
                                }
                                if ((crate::c::bf_read((faded).wrapping_add(0), 5, 5, false) as u16)
                                    as i32)
                                    .wrapping_add(
                                        ((((pal).wrapping_add(3).cast::<i8>()).read()) as i32),
                                    )
                                    <= ((crate::c::bf_read((unfaded).wrapping_add(0), 5, 5, false)
                                        as u16) as i32)
                                {
                                    crate::c::bf_write(
                                        (faded).wrapping_add(0),
                                        5,
                                        5,
                                        ((((crate::c::bf_read((faded).wrapping_add(0), 5, 5, false)
                                            as u16)
                                            as i32)
                                            .wrapping_add(
                                                ((((pal).wrapping_add(3).cast::<i8>()).read())
                                                    as i32),
                                            )) as u16)
                                            as i32,
                                    );
                                }
                                if ((crate::c::bf_read((faded).wrapping_add(1), 2, 5, false) as u16)
                                    as i32)
                                    .wrapping_add(
                                        ((((pal).wrapping_add(3).cast::<i8>()).read()) as i32),
                                    )
                                    <= ((crate::c::bf_read((unfaded).wrapping_add(1), 2, 5, false)
                                        as u16) as i32)
                                {
                                    crate::c::bf_write(
                                        (faded).wrapping_add(1),
                                        2,
                                        5,
                                        ((((crate::c::bf_read((faded).wrapping_add(1), 2, 5, false)
                                            as u16)
                                            as i32)
                                            .wrapping_add(
                                                ((((pal).wrapping_add(3).cast::<i8>()).read())
                                                    as i32),
                                            )) as u16)
                                            as i32,
                                    );
                                }
                            }
                            break 'l3;
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        if (({
            let __p2 = (pal).wrapping_add(2).cast::<i8>();
            let __t3 = (__p2).read();
            (__p2).write(((__p2).read()).wrapping_add(1));
            __t3
        }) as u32)
            != ((crate::c::bf_read(((pal).wrapping_add(4)).wrapping_add(7), 0, 5, true) as i8)
                as u32)
        {
            returnval = 0u8;
        } else {
            ((pal).wrapping_add(2).cast::<i8>()).write(0i8);
            let __p4 = (pal).wrapping_add(3).cast::<i8>();
            (__p4).write((((((__p4).read()) as i32).wrapping_mul((-1i32))) as i8));
            if ((crate::c::bf_read((pal).wrapping_add(0), 0, 7, false) as u8) as i32) == 1i32 {
                crate::c::bf_write(
                    (pal).wrapping_add(0),
                    0,
                    7,
                    ((crate::c::bf_read((pal).wrapping_add(0), 0, 7, false) as u8).wrapping_add(1))
                        as i32,
                );
            } else {
                crate::c::bf_write(
                    (pal).wrapping_add(0),
                    0,
                    7,
                    ((crate::c::bf_read((pal).wrapping_add(0), 0, 7, false) as u8).wrapping_sub(1))
                        as i32,
                );
            }
            returnval = 1u8;
        }
        return returnval;
    }
}
pub(crate) unsafe extern "C" fn RouletteFlash_FlashPalette(pal: *mut u8) -> u8 {
    unsafe {
        let mut pal = pal;
        let mut i: u8 = 0u8;
        'l1: {
            let __sw1 = ((crate::c::bf_read((pal).wrapping_add(0), 0, 7, false) as u8) as i32);
            if __sw1 == 1i32 {
                {
                    'l2: loop {
                        if !(((i) as i32)
                            < (((((pal).wrapping_add(4)).wrapping_add(4)).read()) as i32))
                        {
                            break 'l2;
                        }
                        'l3: {
                            ((((&raw mut gPlttBufferFaded).cast::<u16>()).cast::<u16>())
                                .wrapping_offset(
                                    ((((((pal).wrapping_add(4)).wrapping_add(2).cast::<u16>())
                                        .read()) as i32)
                                        .wrapping_add(((i) as i32)))
                                        as isize,
                                ))
                            .write((((pal).wrapping_add(4)).cast::<u16>()).read());
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                crate::c::bf_write(
                    (pal).wrapping_add(0),
                    0,
                    7,
                    ((crate::c::bf_read((pal).wrapping_add(0), 0, 7, false) as u8).wrapping_add(1))
                        as i32,
                );
                break 'l1;
            }
            if __sw1 == 2i32 {
                {
                    'l4: loop {
                        if !(((i) as i32)
                            < (((((pal).wrapping_add(4)).wrapping_add(4)).read()) as i32))
                        {
                            break 'l4;
                        }
                        'l5: {
                            ((((&raw mut gPlttBufferFaded).cast::<u16>()).cast::<u16>())
                                .wrapping_offset(
                                    ((((((pal).wrapping_add(4)).wrapping_add(2).cast::<u16>())
                                        .read()) as i32)
                                        .wrapping_add(((i) as i32)))
                                        as isize,
                                ))
                            .write(
                                ((((&raw mut gPlttBufferUnfaded).cast::<u16>()).cast::<u16>())
                                    .wrapping_offset(
                                        ((((((pal).wrapping_add(4)).wrapping_add(2).cast::<u16>())
                                            .read())
                                            as i32)
                                            .wrapping_add(((i) as i32)))
                                            as isize,
                                    ))
                                .read(),
                            );
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                crate::c::bf_write(
                    (pal).wrapping_add(0),
                    0,
                    7,
                    ((crate::c::bf_read((pal).wrapping_add(0), 0, 7, false) as u8).wrapping_sub(1))
                        as i32,
                );
                break 'l1;
            }
        }
        return 1u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn RouletteFlash_Run(flash: *mut u8) {
    unsafe {
        let mut flash = flash;
        let mut i: u8 = 0u8;
        if ((flash).read()) != 0 {
            {
                i = 0u8;
                'l1: loop {
                    if !(((i) as u32) < crate::c::div_u32(192u32, 12u32)) {
                        break 'l1;
                    }
                    'l2: {
                        if (crate::c::shr_i32(
                            ((((flash).wrapping_add(2).cast::<u16>()).read()) as i32),
                            ((i) as u32),
                        ) & 1i32)
                            != 0
                        {
                            if (({
                                let __p1 = ((((flash).wrapping_add(4)).cast::<u8>())
                                    .wrapping_offset(((i) as i32) as isize * 12))
                                .wrapping_add(1);
                                let __t2 = ((__p1).read()).wrapping_sub(1);
                                (__p1).write(__t2);
                                __t2
                            }) as i32)
                                == 255i32
                            {
                                if (((((((((flash).wrapping_add(4)).cast::<u8>())
                                    .wrapping_offset(((i) as i32) as isize * 12))
                                .wrapping_add(4))
                                .cast::<u16>())
                                .read()) as i32)
                                    & 32768i32)
                                    != 0
                                {
                                    RouletteFlash_FadePalette(
                                        (((flash).wrapping_add(4)).cast::<u8>())
                                            .wrapping_offset(((i) as i32) as isize * 12),
                                    );
                                } else {
                                    RouletteFlash_FlashPalette(
                                        (((flash).wrapping_add(4)).cast::<u8>())
                                            .wrapping_offset(((i) as i32) as isize * 12),
                                    );
                                }
                                (((((flash).wrapping_add(4)).cast::<u8>())
                                    .wrapping_offset(((i) as i32) as isize * 12))
                                .wrapping_add(1))
                                .write(
                                    ((((((flash).wrapping_add(4)).cast::<u8>())
                                        .wrapping_offset(((i) as i32) as isize * 12))
                                    .wrapping_add(4))
                                    .wrapping_add(5))
                                    .read(),
                                );
                            }
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn RouletteFlash_Enable(flash: *mut u8, flags: u16) {
    unsafe {
        let mut flash = flash;
        let mut flags = flags;
        let mut i: u8 = 0u8;
        let __p1 = (flash);
        (__p1).write(((__p1).read()).wrapping_add(1));
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as u32) < crate::c::div_u32(192u32, 12u32)) {
                    break 'l1;
                }
                'l2: {
                    if (crate::c::shr_i32(((flags) as i32), ((i) as u32)) & 1i32) != 0 {
                        if (crate::c::bf_read(
                            ((((flash).wrapping_add(4)).cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize * 12))
                            .wrapping_add(0),
                            7,
                            1,
                            false,
                        ) as u8)
                            != 0
                        {
                            let __p2 = (flash).wrapping_add(2).cast::<u16>();
                            (__p2).write(
                                (((((__p2).read()) as i32) | crate::c::shl_i32(1i32, ((i) as u32)))
                                    as u16),
                            );
                            crate::c::bf_write(
                                ((((flash).wrapping_add(4)).cast::<u8>())
                                    .wrapping_offset(((i) as i32) as isize * 12))
                                .wrapping_add(0),
                                0,
                                7,
                                (1u8) as i32,
                            );
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn RouletteFlash_Stop(flash: *mut u8, flags: u16) {
    unsafe {
        let mut flash = flash;
        let mut flags = flags;
        let mut i: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as u32) < crate::c::div_u32(192u32, 12u32)) {
                    break 'l1;
                }
                'l2: {
                    if (crate::c::shr_i32(
                        ((((flash).wrapping_add(2).cast::<u16>()).read()) as i32),
                        ((i) as u32),
                    ) & 1i32)
                        != 0
                    {
                        if (crate::c::bf_read(
                            ((((flash).wrapping_add(4)).cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize * 12))
                            .wrapping_add(0),
                            7,
                            1,
                            false,
                        ) as u8)
                            != 0
                        {
                            if (crate::c::shr_i32(((flags) as i32), ((i) as u32)) & 1i32) != 0 {
                                let mut offset: u32 =
                                    ((((((((flash).wrapping_add(4)).cast::<u8>())
                                        .wrapping_offset(((i) as i32) as isize * 12))
                                    .wrapping_add(4))
                                    .wrapping_add(2)
                                    .cast::<u16>())
                                    .read()) as u32);
                                let mut faded: *mut u16 =
                                    (((&raw mut gPlttBufferFaded).cast::<u16>()).cast::<u16>())
                                        .wrapping_offset(((offset) as i32) as isize);
                                let mut unfaded: *mut u16 =
                                    (((&raw mut gPlttBufferUnfaded).cast::<u16>()).cast::<u16>())
                                        .wrapping_offset(((offset) as i32) as isize);
                                crate::c::memcpy(
                                    (faded).cast::<u8>(),
                                    (unfaded).cast::<u8>(),
                                    ((((((((((flash).wrapping_add(4)).cast::<u8>())
                                        .wrapping_offset(((i) as i32) as isize * 12))
                                    .wrapping_add(4))
                                    .wrapping_add(4))
                                    .read()) as i32)
                                        .wrapping_mul(2i32))
                                        as u32),
                                );
                                crate::c::bf_write(
                                    ((((flash).wrapping_add(4)).cast::<u8>())
                                        .wrapping_offset(((i) as i32) as isize * 12))
                                    .wrapping_add(0),
                                    0,
                                    7,
                                    (0u8) as i32,
                                );
                                (((((flash).wrapping_add(4)).cast::<u8>())
                                    .wrapping_offset(((i) as i32) as isize * 12))
                                .wrapping_add(2)
                                .cast::<i8>())
                                .write(0i8);
                                (((((flash).wrapping_add(4)).cast::<u8>())
                                    .wrapping_offset(((i) as i32) as isize * 12))
                                .wrapping_add(1))
                                .write(0u8);
                                if ((crate::c::bf_read(
                                    (((((flash).wrapping_add(4)).cast::<u8>())
                                        .wrapping_offset(((i) as i32) as isize * 12))
                                    .wrapping_add(4))
                                    .wrapping_add(7),
                                    7,
                                    1,
                                    true,
                                ) as i8) as i32)
                                    < 0i32
                                {
                                    (((((flash).wrapping_add(4)).cast::<u8>())
                                        .wrapping_offset(((i) as i32) as isize * 12))
                                    .wrapping_add(3)
                                    .cast::<i8>())
                                    .write((-1i8));
                                } else {
                                    (((((flash).wrapping_add(4)).cast::<u8>())
                                        .wrapping_offset(((i) as i32) as isize * 12))
                                    .wrapping_add(3)
                                    .cast::<i8>())
                                    .write(1i8);
                                }
                            }
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        if ((flags) as i32) == 65535i32 {
            (flash).write(0u8);
            ((flash).wrapping_add(2).cast::<u16>()).write(0u16);
        } else {
            let __p1 = (flash).wrapping_add(2).cast::<u16>();
            (__p1).write((((((__p1).read()) as i32) & !((flags) as i32)) as u16));
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn InitPulseBlend(pulseBlend: *mut u8) {
    unsafe {
        let mut pulseBlend = pulseBlend;
        let mut i: u8 = 0u8;
        ((pulseBlend).cast::<u16>()).write(0u16);
        crate::c::memset(
            (((pulseBlend).wrapping_add(4)).cast::<u8>()).cast::<u8>(),
            0i32,
            192u32,
        );
        {
            'l1: loop {
                if !(((i) as i32) < 16i32) {
                    break 'l1;
                }
                'l2: {
                    ((((pulseBlend).wrapping_add(4)).cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 12))
                    .write(i);
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn InitPulseBlendPaletteSettings(
    pulseBlend: *mut u8,
    settings: *mut u8,
) -> i32 {
    unsafe {
        let mut pulseBlend = pulseBlend;
        let mut settings = settings;
        let mut i: u8 = 0u8;
        let mut pulseBlendPalette: *mut u8 = core::ptr::null_mut();
        if !((crate::c::bf_read(
            (((pulseBlend).wrapping_add(4)).cast::<u8>()).wrapping_add(1),
            7,
            1,
            false,
        ) as u32)
            != 0)
        {
            pulseBlendPalette = ((pulseBlend).wrapping_add(4)).cast::<u8>();
        } else {
            'l1: loop {
                if !((({
                    let __t1 = (i).wrapping_add(1);
                    i = __t1;
                    __t1
                }) as i32)
                    < 16i32)
                {
                    break 'l1;
                }
                if !((crate::c::bf_read(
                    ((((pulseBlend).wrapping_add(4)).cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 12))
                    .wrapping_add(1),
                    7,
                    1,
                    false,
                ) as u32)
                    != 0)
                {
                    pulseBlendPalette = (((pulseBlend).wrapping_add(4)).cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 12);
                    break 'l1;
                }
            }
        }
        if ((pulseBlendPalette) as usize) == 0usize {
            return 255i32;
        }
        crate::c::bf_write((pulseBlendPalette).wrapping_add(1), 0, 4, (0u8) as i32);
        crate::c::bf_write((pulseBlendPalette).wrapping_add(1), 4, 1, (0u8) as i32);
        crate::c::bf_write((pulseBlendPalette).wrapping_add(1), 6, 1, (1i8) as i32);
        crate::c::bf_write((pulseBlendPalette).wrapping_add(1), 7, 1, (1u32) as i32);
        ((pulseBlendPalette).wrapping_add(2)).write(0u8);
        ((pulseBlendPalette).wrapping_add(3)).write(0u8);
        crate::c::memcpy((pulseBlendPalette).wrapping_add(4), settings, 8u32);
        return ((i) as i32);
    }
}
pub(crate) unsafe extern "C" fn ClearPulseBlendPalettesSettings(pulseBlendPalette: *mut u8) {
    unsafe {
        let mut pulseBlendPalette = pulseBlendPalette;
        let mut i: u16 = 0u16;
        if (!((crate::c::bf_read((pulseBlendPalette).wrapping_add(1), 6, 1, true) as i8) != 0))
            && ((crate::c::bf_read(
                ((pulseBlendPalette).wrapping_add(4)).wrapping_add(7),
                6,
                1,
                true,
            ) as i8)
                != 0)
        {
            {
                i = (((pulseBlendPalette).wrapping_add(4))
                    .wrapping_add(2)
                    .cast::<u16>())
                .read();
                'l1: loop {
                    if !(((i) as i32)
                        < (((((pulseBlendPalette).wrapping_add(4))
                            .wrapping_add(2)
                            .cast::<u16>())
                        .read()) as i32)
                            .wrapping_add(
                                (((((pulseBlendPalette).wrapping_add(4)).wrapping_add(4)).read())
                                    as i32),
                            ))
                    {
                        break 'l1;
                    }
                    'l2: {
                        ((((&raw mut gPlttBufferFaded).cast::<u16>()).cast::<u16>())
                            .wrapping_offset(((i) as i32) as isize))
                        .write(
                            ((((&raw mut gPlttBufferUnfaded).cast::<u16>()).cast::<u16>())
                                .wrapping_offset(((i) as i32) as isize))
                            .read(),
                        );
                    }
                    i = (i).wrapping_add(1);
                }
            }
        }
        crate::c::memset((pulseBlendPalette).wrapping_add(4), 0i32, 8u32);
        crate::c::bf_write((pulseBlendPalette).wrapping_add(1), 0, 4, (0u8) as i32);
        crate::c::bf_write((pulseBlendPalette).wrapping_add(1), 4, 1, (0u8) as i32);
        crate::c::bf_write((pulseBlendPalette).wrapping_add(1), 5, 1, (0i8) as i32);
        crate::c::bf_write((pulseBlendPalette).wrapping_add(1), 6, 1, (1i8) as i32);
        crate::c::bf_write((pulseBlendPalette).wrapping_add(1), 7, 1, (0u32) as i32);
        ((pulseBlendPalette).wrapping_add(3)).write(0u8);
        ((pulseBlendPalette).wrapping_add(2)).write(0u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn UnloadUsedPulseBlendPalettes(
    pulseBlend: *mut u8,
    pulseBlendPaletteSelector: u16,
    multiSelection: u8,
) {
    unsafe {
        let mut pulseBlend = pulseBlend;
        let mut pulseBlendPaletteSelector = pulseBlendPaletteSelector;
        let mut multiSelection = multiSelection;
        let mut i: u16 = 0u16;
        if !((multiSelection) != 0) {
            ClearPulseBlendPalettesSettings(
                (((pulseBlend).wrapping_add(4)).cast::<u8>())
                    .wrapping_offset((((pulseBlendPaletteSelector) as i32) & 15i32) as isize * 12),
            );
        } else {
            {
                i = 0u16;
                'l1: loop {
                    if !(((i) as i32) < 16i32) {
                        break 'l1;
                    }
                    'l2: {
                        if ((((pulseBlendPaletteSelector) as i32) & 1i32) != 0)
                            && ((crate::c::bf_read(
                                ((((pulseBlend).wrapping_add(4)).cast::<u8>())
                                    .wrapping_offset(((i) as i32) as isize * 12))
                                .wrapping_add(1),
                                7,
                                1,
                                false,
                            ) as u32)
                                != 0)
                        {
                            ClearPulseBlendPalettesSettings(
                                (((pulseBlend).wrapping_add(4)).cast::<u8>())
                                    .wrapping_offset(((i) as i32) as isize * 12),
                            );
                        }
                        pulseBlendPaletteSelector =
                            ((((pulseBlendPaletteSelector) as i32) >> 1) as u16);
                    }
                    i = (i).wrapping_add(1);
                }
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MarkUsedPulseBlendPalettes(
    pulseBlend: *mut u8,
    pulseBlendPaletteSelector: u16,
    multiSelection: u8,
) {
    unsafe {
        let mut pulseBlend = pulseBlend;
        let mut pulseBlendPaletteSelector = pulseBlendPaletteSelector;
        let mut multiSelection = multiSelection;
        let mut i: u8 = 0u8;
        if !((multiSelection) != 0) {
            i = ((((pulseBlendPaletteSelector) as i32) & 15i32) as u8);
            crate::c::bf_write(
                ((((pulseBlend).wrapping_add(4)).cast::<u8>())
                    .wrapping_offset(((i) as i32) as isize * 12))
                .wrapping_add(1),
                6,
                1,
                (0i8) as i32,
            );
            let __p1 = (pulseBlend).cast::<u16>();
            (__p1)
                .write((((((__p1).read()) as i32) | crate::c::shl_i32(1i32, ((i) as u32))) as u16));
        } else {
            {
                i = 0u8;
                'l1: loop {
                    if !(((i) as i32) < 16i32) {
                        break 'l1;
                    }
                    'l2: {
                        if ((!((((pulseBlendPaletteSelector) as i32) & 1i32) != 0))
                            || (!((crate::c::bf_read(
                                ((((pulseBlend).wrapping_add(4)).cast::<u8>())
                                    .wrapping_offset(((i) as i32) as isize * 12))
                                .wrapping_add(1),
                                7,
                                1,
                                false,
                            ) as u32)
                                != 0)))
                            || (!((crate::c::bf_read(
                                ((((pulseBlend).wrapping_add(4)).cast::<u8>())
                                    .wrapping_offset(((i) as i32) as isize * 12))
                                .wrapping_add(1),
                                6,
                                1,
                                true,
                            ) as i8)
                                != 0))
                        {
                            pulseBlendPaletteSelector =
                                ((((pulseBlendPaletteSelector) as i32) << 1) as u16);
                        } else {
                            crate::c::bf_write(
                                ((((pulseBlend).wrapping_add(4)).cast::<u8>())
                                    .wrapping_offset(((i) as i32) as isize * 12))
                                .wrapping_add(1),
                                6,
                                1,
                                (0i8) as i32,
                            );
                            let __p2 = (pulseBlend).cast::<u16>();
                            (__p2).write(
                                (((((__p2).read()) as i32) | crate::c::shl_i32(1i32, ((i) as u32)))
                                    as u16),
                            );
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn UnmarkUsedPulseBlendPalettes(
    pulseBlend: *mut u8,
    pulseBlendPaletteSelector: u16,
    multiSelection: u8,
) {
    unsafe {
        let mut pulseBlend = pulseBlend;
        let mut pulseBlendPaletteSelector = pulseBlendPaletteSelector;
        let mut multiSelection = multiSelection;
        let mut i: u16 = 0u16;
        let mut pulseBlendPalette: *mut u8 = core::ptr::null_mut();
        let mut j: u8 = 0u8;
        if !((multiSelection) != 0) {
            pulseBlendPalette = (((pulseBlend).wrapping_add(4)).cast::<u8>())
                .wrapping_offset((((pulseBlendPaletteSelector) as i32) & 15i32) as isize * 12);
            if (!((crate::c::bf_read((pulseBlendPalette).wrapping_add(1), 6, 1, true) as i8) != 0))
                && ((crate::c::bf_read((pulseBlendPalette).wrapping_add(1), 7, 1, false) as u32)
                    != 0)
            {
                if (crate::c::bf_read(
                    ((pulseBlendPalette).wrapping_add(4)).wrapping_add(7),
                    6,
                    1,
                    true,
                ) as i8)
                    != 0
                {
                    {
                        i = (((pulseBlendPalette).wrapping_add(4))
                            .wrapping_add(2)
                            .cast::<u16>())
                        .read();
                        'l1: loop {
                            if !(((i) as i32)
                                < (((((pulseBlendPalette).wrapping_add(4))
                                    .wrapping_add(2)
                                    .cast::<u16>())
                                .read()) as i32)
                                    .wrapping_add(
                                        (((((pulseBlendPalette).wrapping_add(4)).wrapping_add(4))
                                            .read())
                                            as i32),
                                    ))
                            {
                                break 'l1;
                            }
                            'l2: {
                                ((((&raw mut gPlttBufferFaded).cast::<u16>()).cast::<u16>())
                                    .wrapping_offset(((i) as i32) as isize))
                                .write(
                                    ((((&raw mut gPlttBufferUnfaded).cast::<u16>()).cast::<u16>())
                                        .wrapping_offset(((i) as i32) as isize))
                                    .read(),
                                );
                            }
                            i = (i).wrapping_add(1);
                        }
                    }
                }
                crate::c::bf_write((pulseBlendPalette).wrapping_add(1), 6, 1, (1i8) as i32);
                let __p1 = (pulseBlend).cast::<u16>();
                (__p1).write(
                    (((((__p1).read()) as i32) & !(crate::c::shl_i32(1i32, ((j) as u32)))) as u16),
                );
            }
        } else {
            {
                j = 0u8;
                'l3: loop {
                    if !(((j) as i32) < 16i32) {
                        break 'l3;
                    }
                    'l4: {
                        pulseBlendPalette = (((pulseBlend).wrapping_add(4)).cast::<u8>())
                            .wrapping_offset(((j) as i32) as isize * 12);
                        if ((!((((pulseBlendPaletteSelector) as i32) & 1i32) != 0))
                            || ((crate::c::bf_read((pulseBlendPalette).wrapping_add(1), 6, 1, true)
                                as i8)
                                != 0))
                            || (!((crate::c::bf_read(
                                (pulseBlendPalette).wrapping_add(1),
                                7,
                                1,
                                false,
                            ) as u32)
                                != 0))
                        {
                            pulseBlendPaletteSelector =
                                ((((pulseBlendPaletteSelector) as i32) << 1) as u16);
                        } else {
                            if (crate::c::bf_read(
                                ((pulseBlendPalette).wrapping_add(4)).wrapping_add(7),
                                6,
                                1,
                                true,
                            ) as i8)
                                != 0
                            {
                                {
                                    i = (((pulseBlendPalette).wrapping_add(4))
                                        .wrapping_add(2)
                                        .cast::<u16>())
                                    .read();
                                    'l5: loop {
                                        if !(((i) as i32)
                                            < (((((pulseBlendPalette).wrapping_add(4))
                                                .wrapping_add(2)
                                                .cast::<u16>())
                                            .read())
                                                as i32)
                                                .wrapping_add(
                                                    (((((pulseBlendPalette).wrapping_add(4))
                                                        .wrapping_add(4))
                                                    .read())
                                                        as i32),
                                                ))
                                        {
                                            break 'l5;
                                        }
                                        'l6: {
                                            ((((&raw mut gPlttBufferFaded).cast::<u16>())
                                                .cast::<u16>())
                                            .wrapping_offset(((i) as i32) as isize))
                                            .write(
                                                ((((&raw mut gPlttBufferUnfaded).cast::<u16>())
                                                    .cast::<u16>())
                                                .wrapping_offset(((i) as i32) as isize))
                                                .read(),
                                            );
                                        }
                                        i = (i).wrapping_add(1);
                                    }
                                }
                            }
                            crate::c::bf_write(
                                (pulseBlendPalette).wrapping_add(1),
                                6,
                                1,
                                (1i8) as i32,
                            );
                            let __p2 = (pulseBlend).cast::<u16>();
                            (__p2).write(
                                (((((__p2).read()) as i32)
                                    & !(crate::c::shl_i32(1i32, ((j) as u32))))
                                    as u16),
                            );
                        }
                    }
                    j = (j).wrapping_add(1);
                }
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn UpdatePulseBlend(pulseBlend: *mut u8) {
    unsafe {
        let mut pulseBlend = pulseBlend;
        let mut pulseBlendPalette: *mut u8 = core::ptr::null_mut();
        let mut i: u8 = 0u8;
        if (((pulseBlend).cast::<u16>()).read()) != 0 {
            {
                i = 0u8;
                'l1: loop {
                    if !(((i) as i32) < 16i32) {
                        break 'l1;
                    }
                    'l2: {
                        pulseBlendPalette = (((pulseBlend).wrapping_add(4)).cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 12);
                        if ((!((crate::c::bf_read((pulseBlendPalette).wrapping_add(1), 6, 1, true)
                            as i8)
                            != 0))
                            && ((crate::c::bf_read((pulseBlendPalette).wrapping_add(1), 7, 1, false)
                                as u32)
                                != 0))
                            && ((!((crate::c::bf_read(
                                ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
                                7,
                                1,
                                false,
                            ) as u16)
                                != 0))
                                || (!((crate::c::bf_read(
                                    ((pulseBlendPalette).wrapping_add(4)).wrapping_add(7),
                                    7,
                                    1,
                                    true,
                                ) as i8)
                                    != 0)))
                        {
                            if (({
                                let __p1 = (pulseBlendPalette).wrapping_add(2);
                                let __t2 = ((__p1).read()).wrapping_sub(1);
                                (__p1).write(__t2);
                                __t2
                            }) as i32)
                                == 255i32
                            {
                                ((pulseBlendPalette).wrapping_add(2)).write(
                                    (((pulseBlendPalette).wrapping_add(4)).wrapping_add(5)).read(),
                                );
                                BlendPalette(
                                    (((pulseBlendPalette).wrapping_add(4))
                                        .wrapping_add(2)
                                        .cast::<u16>())
                                    .read(),
                                    (((((pulseBlendPalette).wrapping_add(4)).wrapping_add(4))
                                        .read()) as u16),
                                    (crate::c::bf_read(
                                        (pulseBlendPalette).wrapping_add(1),
                                        0,
                                        4,
                                        false,
                                    ) as u8),
                                    (((pulseBlendPalette).wrapping_add(4)).cast::<u16>()).read(),
                                );
                                'l3: {
                                    let __sw3 = ((crate::c::bf_read(
                                        ((pulseBlendPalette).wrapping_add(4)).wrapping_add(7),
                                        4,
                                        2,
                                        true,
                                    ) as i8)
                                        as i32);
                                    if __sw3 == 0i32 {
                                        if (({
                                            let __t4 = (crate::c::bf_read(
                                                (pulseBlendPalette).wrapping_add(1),
                                                0,
                                                4,
                                                false,
                                            )
                                                as u8);
                                            crate::c::bf_write(
                                                (pulseBlendPalette).wrapping_add(1),
                                                0,
                                                4,
                                                ((crate::c::bf_read(
                                                    (pulseBlendPalette).wrapping_add(1),
                                                    0,
                                                    4,
                                                    false,
                                                )
                                                    as u8)
                                                    .wrapping_add(1))
                                                    as i32,
                                            );
                                            __t4
                                        }) as i32)
                                            == ((crate::c::bf_read(
                                                ((pulseBlendPalette).wrapping_add(4))
                                                    .wrapping_add(7),
                                                0,
                                                4,
                                                true,
                                            )
                                                as i8)
                                                as i32)
                                        {
                                            let __p5 = (pulseBlendPalette).wrapping_add(3);
                                            (__p5).write(((__p5).read()).wrapping_add(1));
                                            crate::c::bf_write(
                                                (pulseBlendPalette).wrapping_add(1),
                                                0,
                                                4,
                                                (0u8) as i32,
                                            );
                                        }
                                        break 'l3;
                                    }
                                    if __sw3 == 1i32 {
                                        if (crate::c::bf_read(
                                            (pulseBlendPalette).wrapping_add(1),
                                            4,
                                            1,
                                            false,
                                        ) as u8)
                                            != 0
                                        {
                                            if (({
                                                let __t6 = (crate::c::bf_read(
                                                    (pulseBlendPalette).wrapping_add(1),
                                                    0,
                                                    4,
                                                    false,
                                                )
                                                    as u8)
                                                    .wrapping_sub(1);
                                                crate::c::bf_write(
                                                    (pulseBlendPalette).wrapping_add(1),
                                                    0,
                                                    4,
                                                    (__t6) as i32,
                                                );
                                                __t6
                                            })
                                                as i32)
                                                == 0i32
                                            {
                                                let __p7 = (pulseBlendPalette).wrapping_add(3);
                                                (__p7).write(((__p7).read()).wrapping_add(1));
                                                crate::c::bf_write(
                                                    (pulseBlendPalette).wrapping_add(1),
                                                    4,
                                                    1,
                                                    ((((crate::c::bf_read(
                                                        (pulseBlendPalette).wrapping_add(1),
                                                        4,
                                                        1,
                                                        false,
                                                    )
                                                        as u8)
                                                        as i32)
                                                        ^ 1i32)
                                                        as u8)
                                                        as i32,
                                                );
                                            }
                                        } else {
                                            let mut max: u8 = ((((crate::c::bf_read(
                                                ((pulseBlendPalette).wrapping_add(4))
                                                    .wrapping_add(7),
                                                0,
                                                4,
                                                true,
                                            )
                                                as i8)
                                                as i32)
                                                .wrapping_sub(1i32)
                                                & 15i32)
                                                as u8);
                                            if (({
                                                let __t8 = (crate::c::bf_read(
                                                    (pulseBlendPalette).wrapping_add(1),
                                                    0,
                                                    4,
                                                    false,
                                                )
                                                    as u8);
                                                crate::c::bf_write(
                                                    (pulseBlendPalette).wrapping_add(1),
                                                    0,
                                                    4,
                                                    ((crate::c::bf_read(
                                                        (pulseBlendPalette).wrapping_add(1),
                                                        0,
                                                        4,
                                                        false,
                                                    )
                                                        as u8)
                                                        .wrapping_add(1))
                                                        as i32,
                                                );
                                                __t8
                                            })
                                                as i32)
                                                == ((max) as i32)
                                            {
                                                let __p9 = (pulseBlendPalette).wrapping_add(3);
                                                (__p9).write(((__p9).read()).wrapping_add(1));
                                                crate::c::bf_write(
                                                    (pulseBlendPalette).wrapping_add(1),
                                                    4,
                                                    1,
                                                    ((((crate::c::bf_read(
                                                        (pulseBlendPalette).wrapping_add(1),
                                                        4,
                                                        1,
                                                        false,
                                                    )
                                                        as u8)
                                                        as i32)
                                                        ^ 1i32)
                                                        as u8)
                                                        as i32,
                                                );
                                            }
                                        }
                                        break 'l3;
                                    }
                                    if __sw3 == (-2i32) {
                                        if (crate::c::bf_read(
                                            (pulseBlendPalette).wrapping_add(1),
                                            4,
                                            1,
                                            false,
                                        ) as u8)
                                            != 0
                                        {
                                            crate::c::bf_write(
                                                (pulseBlendPalette).wrapping_add(1),
                                                0,
                                                4,
                                                (0u8) as i32,
                                            );
                                        } else {
                                            crate::c::bf_write(
                                                (pulseBlendPalette).wrapping_add(1),
                                                0,
                                                4,
                                                ((((crate::c::bf_read(
                                                    ((pulseBlendPalette).wrapping_add(4))
                                                        .wrapping_add(7),
                                                    0,
                                                    4,
                                                    true,
                                                )
                                                    as i8)
                                                    as i32)
                                                    & 15i32)
                                                    as u8)
                                                    as i32,
                                            );
                                        }
                                        crate::c::bf_write(
                                            (pulseBlendPalette).wrapping_add(1),
                                            4,
                                            1,
                                            ((((crate::c::bf_read(
                                                (pulseBlendPalette).wrapping_add(1),
                                                4,
                                                1,
                                                false,
                                            ) as u8)
                                                as i32)
                                                ^ 1i32)
                                                as u8)
                                                as i32,
                                        );
                                        let __p10 = (pulseBlendPalette).wrapping_add(3);
                                        (__p10).write(((__p10).read()).wrapping_add(1));
                                        break 'l3;
                                    }
                                }
                                if ((((((pulseBlendPalette).wrapping_add(4)).wrapping_add(6))
                                    .read()) as i32)
                                    != 255i32)
                                    && (((((pulseBlendPalette).wrapping_add(3)).read()) as i32)
                                        == (((((pulseBlendPalette).wrapping_add(4))
                                            .wrapping_add(6))
                                        .read())
                                            as i32))
                                {
                                    UnmarkUsedPulseBlendPalettes(
                                        pulseBlend,
                                        (((pulseBlendPalette).read()) as u16),
                                        0u8,
                                    );
                                }
                            }
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FillTilemapRect(
    dest: *mut u16,
    value: u16,
    left: u8,
    top: u8,
    width: u8,
    height: u8,
) {
    unsafe {
        let mut dest = dest;
        let mut value = value;
        let mut left = left;
        let mut top = top;
        let mut width = width;
        let mut height = height;
        let mut _dest: *mut u16 = core::ptr::null_mut();
        let mut i: u8 = 0u8;
        let mut j: u8 = 0u8;
        i = 0u8;
        dest = (dest).wrapping_offset(
            ((((top) as i32).wrapping_mul(32i32)).wrapping_add(((left) as i32))) as isize,
        );
        {
            'l1: loop {
                if !(((i) as i32) < ((height) as i32)) {
                    break 'l1;
                }
                'l2: {
                    _dest = (dest).wrapping_offset((((i) as i32).wrapping_mul(32i32)) as isize);
                    {
                        j = 0u8;
                        'l3: loop {
                            if !(((j) as i32) < ((width) as i32)) {
                                break 'l3;
                            }
                            'l4: {
                                ({
                                    let __t1 = _dest;
                                    _dest = (_dest).wrapping_offset(1);
                                    __t1
                                })
                                .write(value);
                            }
                            j = (j).wrapping_add(1);
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetTilemapRect(
    dest: *mut u16,
    src: *mut u16,
    left: u8,
    top: u8,
    width: u8,
    height: u8,
) {
    unsafe {
        let mut dest = dest;
        let mut src = src;
        let mut left = left;
        let mut top = top;
        let mut width = width;
        let mut height = height;
        let mut _dest: *mut u16 = core::ptr::null_mut();
        let mut _src: *mut u16 = src;
        let mut i: u8 = 0u8;
        let mut j: u8 = 0u8;
        i = 0u8;
        dest = (dest).wrapping_offset(
            ((((top) as i32).wrapping_mul(32i32)).wrapping_add(((left) as i32))) as isize,
        );
        {
            'l1: loop {
                if !(((i) as i32) < ((height) as i32)) {
                    break 'l1;
                }
                'l2: {
                    _dest = (dest).wrapping_offset((((i) as i32).wrapping_mul(32i32)) as isize);
                    {
                        j = 0u8;
                        'l3: loop {
                            if !(((j) as i32) < ((width) as i32)) {
                                break 'l3;
                            }
                            'l4: {
                                ({
                                    let __t1 = _dest;
                                    _dest = (_dest).wrapping_offset(1);
                                    __t1
                                })
                                .write(
                                    ({
                                        let __t3 = _src;
                                        _src = (_src).wrapping_offset(1);
                                        __t3
                                    })
                                    .read(),
                                );
                            }
                            j = (j).wrapping_add(1);
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn FillTilemapRect_Unused(
    dest: *mut u8,
    value: u16,
    left: u8,
    top: u8,
    width: u8,
    height: u8,
) {
    unsafe {
        let mut dest = dest;
        let mut value = value;
        let mut left = left;
        let mut top = top;
        let mut width = width;
        let mut height = height;
        let mut i: u8 = 0u8;
        let mut j: u8 = 0u8;
        let mut x: u8 = 0u8;
        let mut y: u8 = 0u8;
        {
            i = 0u8;
            y = top;
            'l1: loop {
                if !(((i) as i32) < ((height) as i32)) {
                    break 'l1;
                }
                'l2: {
                    {
                        x = left;
                        j = 0u8;
                        'l3: loop {
                            if !(((j) as i32) < ((width) as i32)) {
                                break 'l3;
                            }
                            'l4: {
                                (((dest).wrapping_offset(
                                    ((((y) as i32).wrapping_mul(64i32))
                                        .wrapping_add(((x) as i32).wrapping_mul(2i32)))
                                        as isize
                                        * 1,
                                ))
                                .cast::<u16>())
                                .write(value);
                                x = ((crate::c::rem_i32(((x) as i32).wrapping_add(1i32), 32i32))
                                    as u8);
                            }
                            j = (j).wrapping_add(1);
                        }
                    }
                    y = ((crate::c::rem_i32(((y) as i32).wrapping_add(1i32), 32i32)) as u8);
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SetTilemapRect_Unused(
    dest: *mut u8,
    src: *mut u16,
    left: u8,
    top: u8,
    width: u8,
    height: u8,
) {
    unsafe {
        let mut dest = dest;
        let mut src = src;
        let mut left = left;
        let mut top = top;
        let mut width = width;
        let mut height = height;
        let mut i: u8 = 0u8;
        let mut j: u8 = 0u8;
        let mut x: u8 = 0u8;
        let mut y: u8 = 0u8;
        let mut _src: *mut u16 = core::ptr::null_mut();
        {
            i = 0u8;
            _src = src;
            y = top;
            'l1: loop {
                if !(((i) as i32) < ((height) as i32)) {
                    break 'l1;
                }
                'l2: {
                    {
                        x = left;
                        j = 0u8;
                        'l3: loop {
                            if !(((j) as i32) < ((width) as i32)) {
                                break 'l3;
                            }
                            'l4: {
                                (((dest).wrapping_offset(
                                    ((((y) as i32).wrapping_mul(64i32))
                                        .wrapping_add(((x) as i32).wrapping_mul(2i32)))
                                        as isize
                                        * 1,
                                ))
                                .cast::<u16>())
                                .write(
                                    ({
                                        let __t2 = _src;
                                        _src = (_src).wrapping_offset(1);
                                        __t2
                                    })
                                    .read(),
                                );
                                x = ((crate::c::rem_i32(((x) as i32).wrapping_add(1i32), 32i32))
                                    as u8);
                            }
                            j = (j).wrapping_add(1);
                        }
                    }
                    y = ((crate::c::rem_i32(((y) as i32).wrapping_add(1i32), 32i32)) as u8);
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
