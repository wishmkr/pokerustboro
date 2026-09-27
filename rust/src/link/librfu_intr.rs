//! Translated from `src/librfu_intr.c` by tools/rustport/c2rs.py, then reviewed.
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
    static mut gSTWIStatus: u8;
}

#[cfg_attr(target_arch = "arm", instruction_set(arm::a32))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IntrSIO32() {
    unsafe {
        if ((((&raw mut gSTWIStatus).cast::<*mut u8>()).read()).cast::<i32>()).read_volatile()
            == 10i32
        {
            if core::mem::transmute::<_, usize>(
                ((((&raw mut gSTWIStatus).cast::<*mut u8>()).read())
                    .wrapping_add(32)
                    .cast::<Option<unsafe extern "C" fn()>>())
                .read(),
            ) != 0usize
            {
                Callback_Dummy_ID(
                    ((((&raw mut gSTWIStatus).cast::<*mut u8>()).read())
                        .wrapping_add(32)
                        .cast::<Option<unsafe extern "C" fn()>>())
                    .read(),
                );
            }
        } else {
            if ((((((&raw mut gSTWIStatus).cast::<*mut u8>()).read()).wrapping_add(20))
                .read_volatile()) as i32)
                == 1i32
            {
                sio32intr_clock_master();
            } else {
                sio32intr_clock_slave();
            }
        }
    }
}
#[cfg_attr(target_arch = "arm", instruction_set(arm::a32))]
pub(crate) unsafe extern "C" fn sio32intr_clock_master() {
    unsafe {
        let mut regSIODATA32: u32 = 0u32;
        let mut ackLen: u32 = 0u32;
        STWI_set_timer_in_RAM(80u8);
        regSIODATA32 = ((67109152i32) as usize as *mut u32).read_volatile();
        if ((((&raw mut gSTWIStatus).cast::<*mut u8>()).read()).cast::<i32>()).read_volatile()
            == 0i32
        {
            if regSIODATA32 == 2147483648u32 {
                if ((((((&raw mut gSTWIStatus).cast::<*mut u8>()).read()).wrapping_add(5)).read())
                    as i32)
                    <= ((((((&raw mut gSTWIStatus).cast::<*mut u8>()).read()).wrapping_add(4))
                        .read()) as i32)
                {
                    crate::c::volatile_write(
                        ((67109152i32) as usize as *mut u32),
                        ((((((((&raw mut gSTWIStatus).cast::<*mut u8>()).read())
                            .wrapping_add(36)
                            .cast::<*mut u8>())
                        .read())
                        .cast::<u8>())
                        .cast::<u32>())
                        .wrapping_offset(
                            ((((((&raw mut gSTWIStatus).cast::<*mut u8>()).read()).wrapping_add(5))
                                .read()) as i32) as isize,
                        ))
                        .read(),
                    );
                    let __p1 = (((&raw mut gSTWIStatus).cast::<*mut u8>()).read()).wrapping_add(5);
                    (__p1).write(((__p1).read()).wrapping_add(1));
                } else {
                    crate::c::volatile_write(
                        (((&raw mut gSTWIStatus).cast::<*mut u8>()).read()).cast::<i32>(),
                        1i32,
                    );
                    crate::c::volatile_write(((67109152i32) as usize as *mut u32), 2147483648u32);
                }
            } else {
                STWI_stop_timer_in_RAM();
                STWI_set_timer_in_RAM(130u8);
                return;
            }
        } else {
            if ((((&raw mut gSTWIStatus).cast::<*mut u8>()).read()).cast::<i32>()).read_volatile()
                == 1i32
            {
                if (regSIODATA32 & 4294901760u32) == 2573598720u32 {
                    ((((&raw mut gSTWIStatus).cast::<*mut u8>()).read()).wrapping_add(8))
                        .write(0u8);
                    (((((((&raw mut gSTWIStatus).cast::<*mut u8>()).read())
                        .wrapping_add(40)
                        .cast::<*mut u8>())
                    .read())
                    .cast::<u32>())
                    .wrapping_offset(
                        ((((((&raw mut gSTWIStatus).cast::<*mut u8>()).read()).wrapping_add(8))
                            .read()) as i32) as isize,
                    ))
                    .write(regSIODATA32);
                    let __p2 = (((&raw mut gSTWIStatus).cast::<*mut u8>()).read()).wrapping_add(8);
                    (__p2).write(((__p2).read()).wrapping_add(1));
                    ((((&raw mut gSTWIStatus).cast::<*mut u8>()).read()).wrapping_add(9))
                        .write(((regSIODATA32) as u8));
                    ((((&raw mut gSTWIStatus).cast::<*mut u8>()).read()).wrapping_add(7)).write(
                        (({
                            let __v3 = (regSIODATA32 >> 8);
                            ackLen = __v3;
                            __v3
                        }) as u8),
                    );
                    if {
                        let __v4 = ((((((&raw mut gSTWIStatus).cast::<*mut u8>()).read())
                            .wrapping_add(7))
                        .read()) as u32);
                        ackLen = __v4;
                        __v4
                    } >= ((((((&raw mut gSTWIStatus).cast::<*mut u8>()).read()).wrapping_add(8))
                        .read()) as u32)
                    {
                        crate::c::volatile_write(
                            (((&raw mut gSTWIStatus).cast::<*mut u8>()).read()).cast::<i32>(),
                            2i32,
                        );
                        crate::c::volatile_write(
                            ((67109152i32) as usize as *mut u32),
                            2147483648u32,
                        );
                    } else {
                        crate::c::volatile_write(
                            (((&raw mut gSTWIStatus).cast::<*mut u8>()).read()).cast::<i32>(),
                            3i32,
                        );
                    }
                } else {
                    STWI_stop_timer_in_RAM();
                    STWI_set_timer_in_RAM(130u8);
                    return;
                }
            } else {
                if ((((&raw mut gSTWIStatus).cast::<*mut u8>()).read()).cast::<i32>())
                    .read_volatile()
                    == 2i32
                {
                    (((((((&raw mut gSTWIStatus).cast::<*mut u8>()).read())
                        .wrapping_add(40)
                        .cast::<*mut u8>())
                    .read())
                    .cast::<u32>())
                    .wrapping_offset(
                        ((((((&raw mut gSTWIStatus).cast::<*mut u8>()).read()).wrapping_add(8))
                            .read()) as i32) as isize,
                    ))
                    .write(regSIODATA32);
                    let __p5 = (((&raw mut gSTWIStatus).cast::<*mut u8>()).read()).wrapping_add(8);
                    (__p5).write(((__p5).read()).wrapping_add(1));
                    if ((((((&raw mut gSTWIStatus).cast::<*mut u8>()).read()).wrapping_add(7))
                        .read()) as i32)
                        < ((((((&raw mut gSTWIStatus).cast::<*mut u8>()).read()).wrapping_add(8))
                            .read()) as i32)
                    {
                        crate::c::volatile_write(
                            (((&raw mut gSTWIStatus).cast::<*mut u8>()).read()).cast::<i32>(),
                            3i32,
                        );
                    } else {
                        crate::c::volatile_write(
                            ((67109152i32) as usize as *mut u32),
                            2147483648u32,
                        );
                    }
                }
            }
        }
        if ((handshake_wait(1u16)) as i32) == 1i32 {
            return;
        }
        crate::c::volatile_write(((67109160i32) as usize as *mut u16), 20491u16);
        if ((handshake_wait(0u16)) as i32) == 1i32 {
            return;
        }
        STWI_stop_timer_in_RAM();
        if ((((&raw mut gSTWIStatus).cast::<*mut u8>()).read()).cast::<i32>()).read_volatile()
            == 3i32
        {
            if (((((((((&raw mut gSTWIStatus).cast::<*mut u8>()).read()).wrapping_add(9)).read())
                as i32)
                == 167i32)
                || (((((((&raw mut gSTWIStatus).cast::<*mut u8>()).read()).wrapping_add(9)).read())
                    as i32)
                    == 165i32))
                || (((((((&raw mut gSTWIStatus).cast::<*mut u8>()).read()).wrapping_add(9)).read())
                    as i32)
                    == 181i32))
                || (((((((&raw mut gSTWIStatus).cast::<*mut u8>()).read()).wrapping_add(9)).read())
                    as i32)
                    == 183i32)
            {
                crate::c::volatile_write(
                    (((&raw mut gSTWIStatus).cast::<*mut u8>()).read()).wrapping_add(20),
                    0u8,
                );
                crate::c::volatile_write(((67109152i32) as usize as *mut u32), 2147483648u32);
                crate::c::volatile_write(((67109160i32) as usize as *mut u16), 20482u16);
                crate::c::volatile_write(((67109160i32) as usize as *mut u16), 20610u16);
                crate::c::volatile_write(
                    (((&raw mut gSTWIStatus).cast::<*mut u8>()).read()).cast::<i32>(),
                    5i32,
                );
            } else {
                if ((((((&raw mut gSTWIStatus).cast::<*mut u8>()).read()).wrapping_add(9)).read())
                    as i32)
                    == 238i32
                {
                    crate::c::volatile_write(((67109160i32) as usize as *mut u16), 20483u16);
                    crate::c::volatile_write(
                        (((&raw mut gSTWIStatus).cast::<*mut u8>()).read()).cast::<i32>(),
                        4i32,
                    );
                    crate::c::volatile_write(
                        (((&raw mut gSTWIStatus).cast::<*mut u8>()).read())
                            .wrapping_add(18)
                            .cast::<u16>(),
                        3u16,
                    );
                } else {
                    crate::c::volatile_write(((67109160i32) as usize as *mut u16), 20483u16);
                    crate::c::volatile_write(
                        (((&raw mut gSTWIStatus).cast::<*mut u8>()).read()).cast::<i32>(),
                        4i32,
                    );
                }
            }
            crate::c::volatile_write(
                (((&raw mut gSTWIStatus).cast::<*mut u8>()).read()).wrapping_add(44),
                0u8,
            );
            if core::mem::transmute::<_, usize>(
                ((((&raw mut gSTWIStatus).cast::<*mut u8>()).read())
                    .wrapping_add(24)
                    .cast::<Option<unsafe extern "C" fn()>>())
                .read(),
            ) != 0usize
            {
                Callback_Dummy_M(
                    ((((((&raw mut gSTWIStatus).cast::<*mut u8>()).read()).wrapping_add(6)).read())
                        as i32),
                    ((((((&raw mut gSTWIStatus).cast::<*mut u8>()).read())
                        .wrapping_add(18)
                        .cast::<u16>())
                    .read_volatile()) as i32),
                    ((((&raw mut gSTWIStatus).cast::<*mut u8>()).read())
                        .wrapping_add(24)
                        .cast::<Option<unsafe extern "C" fn()>>())
                    .read(),
                );
            }
        } else {
            crate::c::volatile_write(((67109160i32) as usize as *mut u16), 20483u16);
            crate::c::volatile_write(((67109160i32) as usize as *mut u16), 20611u16);
        }
    }
}
#[cfg_attr(target_arch = "arm", instruction_set(arm::a32))]
pub(crate) unsafe extern "C" fn sio32intr_clock_slave() {
    unsafe {
        let mut regSIODATA32: u32 = 0u32;
        let mut r0: u32 = 0u32;
        let mut reqLen: u32 = 0u32;
        crate::c::volatile_write(
            (((&raw mut gSTWIStatus).cast::<*mut u8>()).read()).wrapping_add(16),
            0u8,
        );
        STWI_set_timer_in_RAM(100u8);
        if ((handshake_wait(0u16)) as i32) == 1i32 {
            return;
        }
        crate::c::volatile_write(((67109160i32) as usize as *mut u16), 20490u16);
        regSIODATA32 = ((67109152i32) as usize as *mut u32).read_volatile();
        if ((((&raw mut gSTWIStatus).cast::<*mut u8>()).read()).cast::<i32>()).read_volatile()
            == 5i32
        {
            ((((((&raw mut gSTWIStatus).cast::<*mut u8>()).read())
                .wrapping_add(40)
                .cast::<*mut u8>())
            .read())
            .cast::<u32>())
            .write(regSIODATA32);
            ((((&raw mut gSTWIStatus).cast::<*mut u8>()).read()).wrapping_add(5)).write(1u8);
            r0 = 2573598720u32;
            reqLen = (regSIODATA32 >> 16);
            if reqLen == (r0 >> 16) {
                ((((&raw mut gSTWIStatus).cast::<*mut u8>()).read()).wrapping_add(4)).write(
                    (({
                        let __v1 = (regSIODATA32 >> 8);
                        reqLen = __v1;
                        __v1
                    }) as u8),
                );
                ((((&raw mut gSTWIStatus).cast::<*mut u8>()).read()).wrapping_add(6)).write(
                    (({
                        let __v2 = (regSIODATA32 >> 0);
                        reqLen = __v2;
                        __v2
                    }) as u8),
                );
                if ((((((&raw mut gSTWIStatus).cast::<*mut u8>()).read()).wrapping_add(4)).read())
                    as i32)
                    == 0i32
                {
                    if (((((((((&raw mut gSTWIStatus).cast::<*mut u8>()).read()).wrapping_add(6))
                        .read()) as i32)
                        == 39i32)
                        || (((((((&raw mut gSTWIStatus).cast::<*mut u8>()).read()).wrapping_add(6))
                            .read()) as i32)
                            == 40i32))
                        || (((((((&raw mut gSTWIStatus).cast::<*mut u8>()).read()).wrapping_add(6))
                            .read()) as i32)
                            == 41i32))
                        || (((((((&raw mut gSTWIStatus).cast::<*mut u8>()).read()).wrapping_add(6))
                            .read()) as i32)
                            == 54i32)
                    {
                        ((((&raw mut gSTWIStatus).cast::<*mut u8>()).read()).wrapping_add(9))
                            .write(
                                ((((((((&raw mut gSTWIStatus).cast::<*mut u8>()).read())
                                    .wrapping_add(6))
                                .read()) as i32)
                                    .wrapping_add(128i32)) as u8),
                            );
                        ((((((&raw mut gSTWIStatus).cast::<*mut u8>()).read())
                            .wrapping_add(36)
                            .cast::<*mut u8>())
                        .read())
                        .cast::<u32>())
                        .write(
                            (2573598720u32).wrapping_add(
                                ((((((&raw mut gSTWIStatus).cast::<*mut u8>()).read())
                                    .wrapping_add(9))
                                .read()) as u32),
                            ),
                        );
                        ((((&raw mut gSTWIStatus).cast::<*mut u8>()).read()).wrapping_add(7))
                            .write(0u8);
                    } else {
                        ((((((&raw mut gSTWIStatus).cast::<*mut u8>()).read())
                            .wrapping_add(36)
                            .cast::<*mut u8>())
                        .read())
                        .cast::<u32>())
                        .write(2573599214u32);
                        if (((((((&raw mut gSTWIStatus).cast::<*mut u8>()).read()).wrapping_add(6))
                            .read()) as i32)
                            >= 16i32)
                            && (((((((&raw mut gSTWIStatus).cast::<*mut u8>()).read())
                                .wrapping_add(6))
                            .read()) as i32)
                                <= 61i32)
                        {
                            (((((((&raw mut gSTWIStatus).cast::<*mut u8>()).read())
                                .wrapping_add(36)
                                .cast::<*mut u8>())
                            .read())
                            .cast::<u32>())
                            .wrapping_offset(1))
                            .write(1u32);
                        } else {
                            (((((((&raw mut gSTWIStatus).cast::<*mut u8>()).read())
                                .wrapping_add(36)
                                .cast::<*mut u8>())
                            .read())
                            .cast::<u32>())
                            .wrapping_offset(1))
                            .write(2u32);
                        }
                        ((((&raw mut gSTWIStatus).cast::<*mut u8>()).read()).wrapping_add(7))
                            .write(1u8);
                        crate::c::volatile_write(
                            (((&raw mut gSTWIStatus).cast::<*mut u8>()).read())
                                .wrapping_add(18)
                                .cast::<u16>(),
                            3u16,
                        );
                    }
                    crate::c::volatile_write(
                        ((67109152i32) as usize as *mut u32),
                        ((((((&raw mut gSTWIStatus).cast::<*mut u8>()).read())
                            .wrapping_add(36)
                            .cast::<*mut u8>())
                        .read())
                        .cast::<u32>())
                        .read(),
                    );
                    ((((&raw mut gSTWIStatus).cast::<*mut u8>()).read()).wrapping_add(8))
                        .write(1u8);
                    crate::c::volatile_write(
                        (((&raw mut gSTWIStatus).cast::<*mut u8>()).read()).cast::<i32>(),
                        7i32,
                    );
                } else {
                    crate::c::volatile_write(((67109152i32) as usize as *mut u32), 2147483648u32);
                    ((((&raw mut gSTWIStatus).cast::<*mut u8>()).read()).wrapping_add(5))
                        .write(1u8);
                    crate::c::volatile_write(
                        (((&raw mut gSTWIStatus).cast::<*mut u8>()).read()).cast::<i32>(),
                        6i32,
                    );
                }
            } else {
                STWI_stop_timer_in_RAM();
                STWI_set_timer_in_RAM(100u8);
                return;
            }
        } else {
            if ((((&raw mut gSTWIStatus).cast::<*mut u8>()).read()).cast::<i32>()).read_volatile()
                == 6i32
            {
                (((((((&raw mut gSTWIStatus).cast::<*mut u8>()).read())
                    .wrapping_add(40)
                    .cast::<*mut u8>())
                .read())
                .cast::<u32>())
                .wrapping_offset(
                    ((((((&raw mut gSTWIStatus).cast::<*mut u8>()).read()).wrapping_add(5)).read())
                        as i32) as isize,
                ))
                .write(regSIODATA32);
                let __p3 = (((&raw mut gSTWIStatus).cast::<*mut u8>()).read()).wrapping_add(5);
                (__p3).write(((__p3).read()).wrapping_add(1));
                if ((((((&raw mut gSTWIStatus).cast::<*mut u8>()).read()).wrapping_add(4)).read())
                    as i32)
                    < ((((((&raw mut gSTWIStatus).cast::<*mut u8>()).read()).wrapping_add(5))
                        .read()) as i32)
                {
                    if ((((((((&raw mut gSTWIStatus).cast::<*mut u8>()).read()).wrapping_add(6))
                        .read()) as i32)
                        == 40i32)
                        || (((((((&raw mut gSTWIStatus).cast::<*mut u8>()).read()).wrapping_add(6))
                            .read()) as i32)
                            == 41i32))
                        || (((((((&raw mut gSTWIStatus).cast::<*mut u8>()).read()).wrapping_add(6))
                            .read()) as i32)
                            == 54i32)
                    {
                        ((((&raw mut gSTWIStatus).cast::<*mut u8>()).read()).wrapping_add(9))
                            .write(
                                ((((((((&raw mut gSTWIStatus).cast::<*mut u8>()).read())
                                    .wrapping_add(6))
                                .read()) as i32)
                                    .wrapping_add(128i32)) as u8),
                            );
                        ((((((&raw mut gSTWIStatus).cast::<*mut u8>()).read())
                            .wrapping_add(36)
                            .cast::<*mut u8>())
                        .read())
                        .cast::<u32>())
                        .write(
                            (2573598720u32
                                | ((((((&raw mut gSTWIStatus).cast::<*mut u8>()).read())
                                    .wrapping_add(9))
                                .read()) as u32)),
                        );
                        ((((&raw mut gSTWIStatus).cast::<*mut u8>()).read()).wrapping_add(7))
                            .write(0u8);
                    } else {
                        ((((((&raw mut gSTWIStatus).cast::<*mut u8>()).read())
                            .wrapping_add(36)
                            .cast::<*mut u8>())
                        .read())
                        .cast::<u32>())
                        .write(2573599214u32);
                        if (((((((&raw mut gSTWIStatus).cast::<*mut u8>()).read()).wrapping_add(6))
                            .read()) as i32)
                            >= 16i32)
                            && (((((((&raw mut gSTWIStatus).cast::<*mut u8>()).read())
                                .wrapping_add(6))
                            .read()) as i32)
                                <= 61i32)
                        {
                            (((((((&raw mut gSTWIStatus).cast::<*mut u8>()).read())
                                .wrapping_add(36)
                                .cast::<*mut u8>())
                            .read())
                            .cast::<u32>())
                            .wrapping_offset(1))
                            .write(1u32);
                        } else {
                            (((((((&raw mut gSTWIStatus).cast::<*mut u8>()).read())
                                .wrapping_add(36)
                                .cast::<*mut u8>())
                            .read())
                            .cast::<u32>())
                            .wrapping_offset(1))
                            .write(2u32);
                        }
                        ((((&raw mut gSTWIStatus).cast::<*mut u8>()).read()).wrapping_add(7))
                            .write(1u8);
                        crate::c::volatile_write(
                            (((&raw mut gSTWIStatus).cast::<*mut u8>()).read())
                                .wrapping_add(18)
                                .cast::<u16>(),
                            3u16,
                        );
                    }
                    crate::c::volatile_write(
                        ((67109152i32) as usize as *mut u32),
                        ((((((&raw mut gSTWIStatus).cast::<*mut u8>()).read())
                            .wrapping_add(36)
                            .cast::<*mut u8>())
                        .read())
                        .cast::<u32>())
                        .read(),
                    );
                    ((((&raw mut gSTWIStatus).cast::<*mut u8>()).read()).wrapping_add(8))
                        .write(1u8);
                    crate::c::volatile_write(
                        (((&raw mut gSTWIStatus).cast::<*mut u8>()).read()).cast::<i32>(),
                        7i32,
                    );
                } else {
                    crate::c::volatile_write(((67109152i32) as usize as *mut u32), 2147483648u32);
                }
            } else {
                if ((((&raw mut gSTWIStatus).cast::<*mut u8>()).read()).cast::<i32>())
                    .read_volatile()
                    == 7i32
                {
                    if regSIODATA32 == 2147483648u32 {
                        if ((((((&raw mut gSTWIStatus).cast::<*mut u8>()).read()).wrapping_add(7))
                            .read()) as i32)
                            < ((((((&raw mut gSTWIStatus).cast::<*mut u8>()).read())
                                .wrapping_add(8))
                            .read()) as i32)
                        {
                            crate::c::volatile_write(
                                (((&raw mut gSTWIStatus).cast::<*mut u8>()).read()).cast::<i32>(),
                                8i32,
                            );
                        } else {
                            crate::c::volatile_write(
                                ((67109152i32) as usize as *mut u32),
                                (((((((&raw mut gSTWIStatus).cast::<*mut u8>()).read())
                                    .wrapping_add(36)
                                    .cast::<*mut u8>())
                                .read())
                                .cast::<u32>())
                                .wrapping_offset(
                                    ((((((&raw mut gSTWIStatus).cast::<*mut u8>()).read())
                                        .wrapping_add(8))
                                    .read()) as i32) as isize,
                                ))
                                .read(),
                            );
                            let __p4 =
                                (((&raw mut gSTWIStatus).cast::<*mut u8>()).read()).wrapping_add(8);
                            (__p4).write(((__p4).read()).wrapping_add(1));
                        }
                    } else {
                        STWI_stop_timer_in_RAM();
                        STWI_set_timer_in_RAM(100u8);
                        return;
                    }
                }
            }
        }
        if ((handshake_wait(1u16)) as i32) == 1i32 {
            return;
        }
        if ((((&raw mut gSTWIStatus).cast::<*mut u8>()).read()).cast::<i32>()).read_volatile()
            == 8i32
        {
            crate::c::volatile_write(((67109160i32) as usize as *mut u16), 20482u16);
            STWI_stop_timer_in_RAM();
            if ((((((&raw mut gSTWIStatus).cast::<*mut u8>()).read())
                .wrapping_add(18)
                .cast::<u16>())
            .read_volatile()) as i32)
                == 3i32
            {
                STWI_init_slave();
                if core::mem::transmute::<_, usize>(
                    ((((&raw mut gSTWIStatus).cast::<*mut u8>()).read())
                        .wrapping_add(28)
                        .cast::<Option<unsafe extern "C" fn(u16)>>())
                    .read(),
                ) != 0usize
                {
                    Callback_Dummy_S(
                        494u16,
                        ((((&raw mut gSTWIStatus).cast::<*mut u8>()).read())
                            .wrapping_add(28)
                            .cast::<Option<unsafe extern "C" fn(u16)>>())
                        .read(),
                    );
                }
            } else {
                crate::c::volatile_write(((67109152i32) as usize as *mut u32), 0u32);
                crate::c::volatile_write(((67109160i32) as usize as *mut u16), 0u16);
                crate::c::volatile_write(((67109160i32) as usize as *mut u16), 20483u16);
                crate::c::volatile_write(
                    (((&raw mut gSTWIStatus).cast::<*mut u8>()).read()).wrapping_add(20),
                    1u8,
                );
                crate::c::volatile_write(
                    (((&raw mut gSTWIStatus).cast::<*mut u8>()).read()).cast::<i32>(),
                    0i32,
                );
                if core::mem::transmute::<_, usize>(
                    ((((&raw mut gSTWIStatus).cast::<*mut u8>()).read())
                        .wrapping_add(28)
                        .cast::<Option<unsafe extern "C" fn(u16)>>())
                    .read(),
                ) != 0usize
                {
                    Callback_Dummy_S(
                        (((((((((&raw mut gSTWIStatus).cast::<*mut u8>()).read()).wrapping_add(4))
                            .read()) as i32)
                            << 8)
                            | ((((((&raw mut gSTWIStatus).cast::<*mut u8>()).read())
                                .wrapping_add(6))
                            .read()) as i32)) as u16),
                        ((((&raw mut gSTWIStatus).cast::<*mut u8>()).read())
                            .wrapping_add(28)
                            .cast::<Option<unsafe extern "C" fn(u16)>>())
                        .read(),
                    );
                }
            }
        } else {
            crate::c::volatile_write(((67109384i32) as usize as *mut u16), 0u16);
            if (((((67109122i32) as usize as *mut u16).read_volatile()) as i32) & 128i32) != 0 {
                if (((((67109122i32) as usize as *mut u16).read_volatile()) as i32) & 3i32) == 0i32
                {
                    'l1: loop {
                        if !(((((67109120i32) as usize as *mut u16).read_volatile()) as i32)
                            > 65435i32)
                        {
                            break 'l1;
                        }
                    }
                } else {
                    'l2: loop {
                        if !(((((67109120i32) as usize as *mut u16).read_volatile()) as i32)
                            > 65534i32)
                        {
                            break 'l2;
                        }
                    }
                }
            }
            crate::c::volatile_write(((67109160i32) as usize as *mut u16), 20482u16);
            crate::c::volatile_write(((67109160i32) as usize as *mut u16), 20610u16);
            crate::c::volatile_write(((67109384i32) as usize as *mut u16), 1u16);
        }
    }
}
#[cfg_attr(target_arch = "arm", instruction_set(arm::a32))]
pub(crate) unsafe extern "C" fn handshake_wait(slot: u16) -> u16 {
    unsafe {
        let mut slot = slot;
        'l1: loop {
            'l2: {
                if (((((((&raw mut gSTWIStatus).cast::<*mut u8>()).read()).wrapping_add(16))
                    .read_volatile()) as i32)
                    & 255i32)
                    == 1i32
                {
                    crate::c::volatile_write(
                        (((&raw mut gSTWIStatus).cast::<*mut u8>()).read()).wrapping_add(16),
                        0u8,
                    );
                    return 1u16;
                }
            }
            if !((((((67109160i32) as usize as *mut u16).read_volatile()) as i32) & 4i32)
                != (((slot) as i32) << 2))
            {
                break 'l1;
            }
        }
        return 0u16;
    }
}
#[cfg_attr(target_arch = "arm", instruction_set(arm::a32))]
pub(crate) unsafe extern "C" fn STWI_set_timer_in_RAM(count: u8) {
    unsafe {
        let mut count = count;
        let mut regTMCNTL: *mut u16 = (((67109120i32).wrapping_add(
            ((((((&raw mut gSTWIStatus).cast::<*mut u8>()).read()).wrapping_add(10)).read())
                as i32)
                .wrapping_mul(4i32),
        )) as usize as *mut u16);
        let mut regTMCNTH: *mut u16 = (((67109122i32).wrapping_add(
            ((((((&raw mut gSTWIStatus).cast::<*mut u8>()).read()).wrapping_add(10)).read())
                as i32)
                .wrapping_mul(4i32),
        )) as usize as *mut u16);
        crate::c::volatile_write(((67109384i32) as usize as *mut u16), 0u16);
        'l1: {
            let __sw1 = ((count) as i32);
            if __sw1 == 50i32 {
                crate::c::volatile_write(regTMCNTL, 64715u16);
                ((((&raw mut gSTWIStatus).cast::<*mut u8>()).read())
                    .wrapping_add(12)
                    .cast::<i32>())
                .write(1i32);
                break 'l1;
            }
            if __sw1 == 80i32 {
                crate::c::volatile_write(regTMCNTL, 64224u16);
                ((((&raw mut gSTWIStatus).cast::<*mut u8>()).read())
                    .wrapping_add(12)
                    .cast::<i32>())
                .write(2i32);
                break 'l1;
            }
            if __sw1 == 100i32 {
                crate::c::volatile_write(regTMCNTL, 63894u16);
                ((((&raw mut gSTWIStatus).cast::<*mut u8>()).read())
                    .wrapping_add(12)
                    .cast::<i32>())
                .write(3i32);
                break 'l1;
            }
            if __sw1 == 130i32 {
                crate::c::volatile_write(regTMCNTL, 63405u16);
                ((((&raw mut gSTWIStatus).cast::<*mut u8>()).read())
                    .wrapping_add(12)
                    .cast::<i32>())
                .write(4i32);
                break 'l1;
            }
        }
        crate::c::volatile_write(regTMCNTH, 195u16);
        crate::c::volatile_write(
            ((67109378i32) as usize as *mut u16),
            ((crate::c::shl_i32(
                8i32,
                ((((((&raw mut gSTWIStatus).cast::<*mut u8>()).read()).wrapping_add(10)).read())
                    as u32),
            )) as u16),
        );
        crate::c::volatile_write(((67109384i32) as usize as *mut u16), 1u16);
    }
}
#[cfg_attr(target_arch = "arm", instruction_set(arm::a32))]
pub(crate) unsafe extern "C" fn STWI_stop_timer_in_RAM() {
    unsafe {
        ((((&raw mut gSTWIStatus).cast::<*mut u8>()).read())
            .wrapping_add(12)
            .cast::<i32>())
        .write(0i32);
        crate::c::volatile_write(
            (((67109120i32).wrapping_add(
                ((((((&raw mut gSTWIStatus).cast::<*mut u8>()).read()).wrapping_add(10)).read())
                    as i32)
                    .wrapping_mul(4i32),
            )) as usize as *mut u16),
            0u16,
        );
        crate::c::volatile_write(
            (((67109122i32).wrapping_add(
                ((((((&raw mut gSTWIStatus).cast::<*mut u8>()).read()).wrapping_add(10)).read())
                    as i32)
                    .wrapping_mul(4i32),
            )) as usize as *mut u16),
            0u16,
        );
    }
}
#[cfg_attr(target_arch = "arm", instruction_set(arm::a32))]
pub(crate) unsafe extern "C" fn STWI_init_slave() {
    unsafe {
        crate::c::volatile_write(
            (((&raw mut gSTWIStatus).cast::<*mut u8>()).read()).cast::<i32>(),
            5i32,
        );
        crate::c::volatile_write(
            (((&raw mut gSTWIStatus).cast::<*mut u8>()).read()).wrapping_add(20),
            0u8,
        );
        ((((&raw mut gSTWIStatus).cast::<*mut u8>()).read()).wrapping_add(4)).write(0u8);
        ((((&raw mut gSTWIStatus).cast::<*mut u8>()).read()).wrapping_add(5)).write(0u8);
        ((((&raw mut gSTWIStatus).cast::<*mut u8>()).read()).wrapping_add(6)).write(0u8);
        ((((&raw mut gSTWIStatus).cast::<*mut u8>()).read()).wrapping_add(7)).write(0u8);
        ((((&raw mut gSTWIStatus).cast::<*mut u8>()).read()).wrapping_add(8)).write(0u8);
        ((((&raw mut gSTWIStatus).cast::<*mut u8>()).read()).wrapping_add(9)).write(0u8);
        ((((&raw mut gSTWIStatus).cast::<*mut u8>()).read())
            .wrapping_add(12)
            .cast::<i32>())
        .write(0i32);
        crate::c::volatile_write(
            (((&raw mut gSTWIStatus).cast::<*mut u8>()).read()).wrapping_add(16),
            0u8,
        );
        crate::c::volatile_write(
            (((&raw mut gSTWIStatus).cast::<*mut u8>()).read())
                .wrapping_add(18)
                .cast::<u16>(),
            0u16,
        );
        ((((&raw mut gSTWIStatus).cast::<*mut u8>()).read()).wrapping_add(21)).write(0u8);
        crate::c::volatile_write(((67109160i32) as usize as *mut u16), 20610u16);
    }
}
// hand-written: tools/rustport/overrides/librfu_intr/Callback_Dummy_M.rs
/// `Callback_Dummy_M`: a naked `bx r2` in C, i.e. a tail call of `callbackM`
/// with this function's own arguments still in r0-r2.
unsafe extern "C" fn Callback_Dummy_M(
    req_command_id: i32,
    error: i32,
    callback_m: Option<unsafe extern "C" fn()>,
) {
    unsafe {
        let f: unsafe extern "C" fn(i32, i32, Option<unsafe extern "C" fn()>) =
            core::mem::transmute(callback_m.unwrap_unchecked());
        f(req_command_id, error, callback_m)
    }
}

// hand-written: tools/rustport/overrides/librfu_intr/Callback_Dummy_S.rs
/// `Callback_Dummy_S`: a naked `bx r1`, i.e. `callbackS(reqCommandId)`.
unsafe extern "C" fn Callback_Dummy_S(
    req_command_id: u16,
    callback_s: Option<unsafe extern "C" fn(u16)>,
) {
    unsafe {
        let f: unsafe extern "C" fn(u16, Option<unsafe extern "C" fn(u16)>) =
            core::mem::transmute(callback_s.unwrap_unchecked());
        f(req_command_id, callback_s)
    }
}

// hand-written: tools/rustport/overrides/librfu_intr/Callback_Dummy_ID.rs
/// `Callback_Dummy_ID`: a naked `bx r0`, i.e. `callbackId()`.
unsafe extern "C" fn Callback_Dummy_ID(callback_id: Option<unsafe extern "C" fn()>) {
    unsafe {
        let f: unsafe extern "C" fn(Option<unsafe extern "C" fn()>) =
            core::mem::transmute(callback_id.unwrap_unchecked());
        f(callback_id)
    }
}
