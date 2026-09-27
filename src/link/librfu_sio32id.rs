//! Translated from `src/librfu_sio32id.c` by tools/rustport/c2rs.py, then reviewed.
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

// Data tables (translate with cdata.py): Sio32ConnectionData Sio32IDLib_Var
#[allow(unused_imports)]
use crate::data::librfu_sio32id::*;

#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gRfuSIO32Id: crate::ffi::Align4<[u8; 12]> = crate::ffi::Align4([0; 12]);

unsafe extern "C" {
    static mut gSTWIStatus: u8;
    fn CpuSet(a0: *mut u8, a1: *mut u8, a2: u32);
    fn STWI_set_Callback_ID(a0: Option<unsafe extern "C" fn()>);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn AgbRFU_checkID(maxTries: u8) -> i32 {
    unsafe {
        let mut maxTries = maxTries;
        let mut ieBak: u16 = 0u16;
        let mut regTMCNTL: *mut u16 = core::ptr::null_mut();
        let mut id: i32 = 0i32;
        if ((((67109384i32) as usize as *mut u16).read_volatile()) as i32) == 0i32 {
            return (-1i32);
        }
        ieBak = ((67109376i32) as usize as *mut u16).read_volatile();
        crate::c::volatile_write(
            (((&raw mut gSTWIStatus).cast::<*mut u8>()).read()).cast::<i32>(),
            10i32,
        );
        STWI_set_Callback_ID(Some(Sio32IDIntr as unsafe extern "C" fn()));
        Sio32IDInit();
        regTMCNTL = (((67109120i32).wrapping_add(
            ((((((&raw mut gSTWIStatus).cast::<*mut u8>()).read()).wrapping_add(10)).read())
                as i32)
                .wrapping_mul(4i32),
        )) as usize as *mut u16);
        maxTries = ((((maxTries) as i32).wrapping_mul(8i32)) as u8);
        'l1: loop {
            if !((({
                let __t1 = (maxTries).wrapping_sub(1);
                maxTries = __t1;
                __t1
            }) as i32)
                != 255i32)
            {
                break 'l1;
            }
            id = Sio32IDMain();
            if id != 0i32 {
                break 'l1;
            }
            crate::c::volatile_write((regTMCNTL).wrapping_offset(1), 0u16);
            crate::c::volatile_write(regTMCNTL, 0u16);
            crate::c::volatile_write((regTMCNTL).wrapping_offset(1), 131u16);
            'l2: loop {
                if !((((regTMCNTL).read_volatile()) as i32) < 32i32) {
                    break 'l2;
                }
            }
            crate::c::volatile_write((regTMCNTL).wrapping_offset(1), 0u16);
            crate::c::volatile_write(regTMCNTL, 0u16);
        }
        crate::c::volatile_write(((67109384i32) as usize as *mut u16), 0u16);
        crate::c::volatile_write(((67109376i32) as usize as *mut u16), ieBak);
        crate::c::volatile_write(((67109384i32) as usize as *mut u16), 1u16);
        crate::c::volatile_write(
            (((&raw mut gSTWIStatus).cast::<*mut u8>()).read()).cast::<i32>(),
            0i32,
        );
        STWI_set_Callback_ID(None);
        return id;
    }
}
pub(crate) unsafe extern "C" fn Sio32IDInit() {
    unsafe {
        crate::c::volatile_write(((67109384i32) as usize as *mut u16), 0u16);
        let __p1 = ((67109376i32) as usize as *mut u16);
        crate::c::volatile_write(
            __p1,
            (((((__p1).read_volatile()) as i32)
                & !(crate::c::shl_i32(
                    8i32,
                    ((((((&raw mut gSTWIStatus).cast::<*mut u8>()).read()).wrapping_add(10)).read())
                        as u32),
                ) | 128i32)) as u16),
        );
        crate::c::volatile_write(((67109384i32) as usize as *mut u16), 1u16);
        crate::c::volatile_write(((67109172i32) as usize as *mut u16), 0u16);
        crate::c::volatile_write(((67109160i32) as usize as *mut u16), 4096u16);
        let __p2 = ((67109160i32) as usize as *mut u16);
        crate::c::volatile_write(
            __p2,
            (((((__p2).read_volatile()) as i32) | 16512i32) as u16),
        );
        'l1: loop {
            'l2: {
                {
                    let mut tmp: u32 = 0u32;
                    (&raw mut tmp).write_volatile(0u32);
                    'l3: loop {
                        'l4: {
                            CpuSet(
                                (&raw mut tmp).cast::<u8>(),
                                (&raw mut gRfuSIO32Id).cast::<u8>(),
                                (83886080u32
                                    | (crate::c::div_u32(
                                        12u32,
                                        ((crate::c::div_i32(32i32, 8i32)) as u32),
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
        crate::c::volatile_write(((67109378i32) as usize as *mut u16), 128u16);
    }
}
pub(crate) unsafe extern "C" fn Sio32IDMain() -> i32 {
    unsafe {
        'l1: {
            let __sw1 = (((((&raw mut gRfuSIO32Id).cast::<u8>()).wrapping_add(1)).read()) as i32);
            let __matched = __sw1 == 0i32 || __sw1 == 1i32;
            let mut __fall = false;
            if __sw1 == 0i32 {
                __fall = true;
                ((&raw mut gRfuSIO32Id).cast::<u8>()).write(1u8);
                let __p2 = ((67109160i32) as usize as *mut u16);
                crate::c::volatile_write(__p2, (((((__p2).read_volatile()) as i32) | 1i32) as u16));
                crate::c::volatile_write(((67109384i32) as usize as *mut u16), 0u16);
                let __p3 = ((67109376i32) as usize as *mut u16);
                crate::c::volatile_write(
                    __p3,
                    (((((__p3).read_volatile()) as i32) | 128i32) as u16),
                );
                crate::c::volatile_write(((67109384i32) as usize as *mut u16), 1u16);
                (((&raw mut gRfuSIO32Id).cast::<u8>()).wrapping_add(1)).write(1u8);
                let __p4 = ((67109160i32) as usize as *mut u16).cast::<u8>();
                crate::c::volatile_write(
                    __p4,
                    (((((__p4).read_volatile()) as i32) | 128i32) as u8),
                );
                break 'l1;
            }
            if __sw1 == 1i32 {
                __fall = true;
                if (((((&raw mut gRfuSIO32Id).cast::<u8>())
                    .wrapping_add(10)
                    .cast::<u16>())
                .read()) as i32)
                    == 0i32
                {
                    if ((((&raw mut gRfuSIO32Id).cast::<u8>()).read()) as i32) == 1i32 {
                        if (((((&raw mut gRfuSIO32Id).cast::<u8>())
                            .wrapping_add(2)
                            .cast::<u16>())
                        .read()) as i32)
                            == 0i32
                        {
                            crate::c::volatile_write(((67109384i32) as usize as *mut u16), 0u16);
                            let __p5 = ((67109160i32) as usize as *mut u16);
                            crate::c::volatile_write(
                                __p5,
                                (((((__p5).read_volatile()) as i32) | 128i32) as u16),
                            );
                            crate::c::volatile_write(((67109384i32) as usize as *mut u16), 1u16);
                        }
                    } else {
                        if ((((((&raw mut gRfuSIO32Id).cast::<u8>())
                            .wrapping_add(4)
                            .cast::<u16>())
                        .read()) as i32)
                            != 32769i32)
                            && (!(((((&raw mut gRfuSIO32Id).cast::<u8>())
                                .wrapping_add(2)
                                .cast::<u16>())
                            .read())
                                != 0))
                        {
                            crate::c::volatile_write(((67109384i32) as usize as *mut u16), 0u16);
                            let __p6 = ((67109376i32) as usize as *mut u16);
                            crate::c::volatile_write(
                                __p6,
                                (((((__p6).read_volatile()) as i32) & (-129i32)) as u16),
                            );
                            crate::c::volatile_write(((67109384i32) as usize as *mut u16), 1u16);
                            crate::c::volatile_write(((67109160i32) as usize as *mut u16), 0u16);
                            crate::c::volatile_write(((67109160i32) as usize as *mut u16), 4096u16);
                            crate::c::volatile_write(((67109378i32) as usize as *mut u16), 128u16);
                            let __p7 = ((67109160i32) as usize as *mut u16);
                            crate::c::volatile_write(
                                __p7,
                                (((((__p7).read_volatile()) as i32) | 16512i32) as u16),
                            );
                            crate::c::volatile_write(((67109384i32) as usize as *mut u16), 0u16);
                            let __p8 = ((67109376i32) as usize as *mut u16);
                            crate::c::volatile_write(
                                __p8,
                                (((((__p8).read_volatile()) as i32) | 128i32) as u16),
                            );
                            crate::c::volatile_write(((67109384i32) as usize as *mut u16), 1u16);
                        }
                    }
                    break 'l1;
                } else {
                    (((&raw mut gRfuSIO32Id).cast::<u8>()).wrapping_add(1)).write(2u8);
                }
            }
            if __fall || !__matched {
                __fall = true;
                return (((((&raw mut gRfuSIO32Id).cast::<u8>())
                    .wrapping_add(10)
                    .cast::<u16>())
                .read()) as i32);
            }
        }
        return 0i32;
    }
}
pub(crate) unsafe extern "C" fn Sio32IDIntr() {
    unsafe {
        let mut regSIODATA32: u32 = 0u32;
        let mut delay: u16 = 0u16;
        let mut rfuSIO32IdUnk0_times_16: u32 = 0u32;
        regSIODATA32 = ((67109152i32) as usize as *mut u32).read_volatile();
        if ((((&raw mut gRfuSIO32Id).cast::<u8>()).read()) as i32) != 1i32 {
            let __p1 = ((67109160i32) as usize as *mut u16);
            crate::c::volatile_write(__p1, (((((__p1).read_volatile()) as i32) | 128i32) as u16));
        }
        rfuSIO32IdUnk0_times_16 = (crate::c::shl_u32(
            regSIODATA32,
            (((16i32).wrapping_mul(((((&raw mut gRfuSIO32Id).cast::<u8>()).read()) as i32)))
                as u32),
        ) >> 16);
        regSIODATA32 = (crate::c::shl_u32(
            regSIODATA32,
            (((16i32).wrapping_mul(
                (1i32).wrapping_sub(((((&raw mut gRfuSIO32Id).cast::<u8>()).read()) as i32)),
            )) as u32),
        ) >> 16);
        if (((((&raw mut gRfuSIO32Id).cast::<u8>())
            .wrapping_add(10)
            .cast::<u16>())
        .read()) as i32)
            == 0i32
        {
            let mut backup: u16 = ((rfuSIO32IdUnk0_times_16) as u16);
            if ((backup) as i32)
                == (((((&raw mut gRfuSIO32Id).cast::<u8>())
                    .wrapping_add(6)
                    .cast::<u16>())
                .read()) as i32)
            {
                if (((((&raw mut gRfuSIO32Id).cast::<u8>())
                    .wrapping_add(2)
                    .cast::<u16>())
                .read()) as i32)
                    < 4i32
                {
                    backup = ((!(((((&raw mut gRfuSIO32Id).cast::<u8>())
                        .wrapping_add(4)
                        .cast::<u16>())
                    .read()) as i32)) as u16);
                    if (((((&raw mut gRfuSIO32Id).cast::<u8>())
                        .wrapping_add(6)
                        .cast::<u16>())
                    .read()) as i32)
                        == ((backup) as i32)
                    {
                        if regSIODATA32
                            == (((!(((((&raw mut gRfuSIO32Id).cast::<u8>())
                                .wrapping_add(6)
                                .cast::<u16>())
                            .read()) as i32)) as u16) as u32)
                        {
                            let __p2 = ((&raw mut gRfuSIO32Id).cast::<u8>())
                                .wrapping_add(2)
                                .cast::<u16>();
                            (__p2).write(((__p2).read()).wrapping_add(1));
                        }
                    }
                } else {
                    (((&raw mut gRfuSIO32Id).cast::<u8>())
                        .wrapping_add(10)
                        .cast::<u16>())
                    .write(((regSIODATA32) as u16));
                }
            } else {
                (((&raw mut gRfuSIO32Id).cast::<u8>())
                    .wrapping_add(2)
                    .cast::<u16>())
                .write(0u16);
            }
        }
        if (((((&raw mut gRfuSIO32Id).cast::<u8>())
            .wrapping_add(2)
            .cast::<u16>())
        .read()) as i32)
            < 4i32
        {
            (((&raw mut gRfuSIO32Id).cast::<u8>())
                .wrapping_add(4)
                .cast::<u16>())
            .write(
                ((((&raw const Sio32ConnectionData)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<u16>())
                .cast::<u16>())
                .wrapping_offset(
                    (((((&raw mut gRfuSIO32Id).cast::<u8>())
                        .wrapping_add(2)
                        .cast::<u16>())
                    .read()) as i32) as isize,
                ))
                .read(),
            );
        } else {
            (((&raw mut gRfuSIO32Id).cast::<u8>())
                .wrapping_add(4)
                .cast::<u16>())
            .write(32769u16);
        }
        (((&raw mut gRfuSIO32Id).cast::<u8>())
            .wrapping_add(6)
            .cast::<u16>())
        .write(((!(regSIODATA32)) as u16));
        crate::c::volatile_write(
            ((67109152i32) as usize as *mut u32),
            (((crate::c::shl_i32(
                (((((&raw mut gRfuSIO32Id).cast::<u8>())
                    .wrapping_add(4)
                    .cast::<u16>())
                .read()) as i32),
                (((16i32).wrapping_mul(
                    (1i32).wrapping_sub(((((&raw mut gRfuSIO32Id).cast::<u8>()).read()) as i32)),
                )) as u32),
            ))
            .wrapping_add(crate::c::shl_i32(
                (((((&raw mut gRfuSIO32Id).cast::<u8>())
                    .wrapping_add(6)
                    .cast::<u16>())
                .read()) as i32),
                (((16i32).wrapping_mul(((((&raw mut gRfuSIO32Id).cast::<u8>()).read()) as i32)))
                    as u32),
            ))) as u32),
        );
        if (((((&raw mut gRfuSIO32Id).cast::<u8>()).read()) as i32) == 1i32)
            && (((((((&raw mut gRfuSIO32Id).cast::<u8>())
                .wrapping_add(2)
                .cast::<u16>())
            .read()) as i32)
                != 0i32)
                || (regSIODATA32 == 18766u32))
        {
            {
                delay = 0u16;
                'l1: loop {
                    if !(((delay) as i32) < 600i32) {
                        break 'l1;
                    }
                    'l2: {}
                    delay = (delay).wrapping_add(1);
                }
            }
            if (((((&raw mut gRfuSIO32Id).cast::<u8>())
                .wrapping_add(10)
                .cast::<u16>())
            .read()) as i32)
                == 0i32
            {
                let __p3 = ((67109160i32) as usize as *mut u16);
                crate::c::volatile_write(
                    __p3,
                    (((((__p3).read_volatile()) as i32) | 128i32) as u16),
                );
            }
        }
    }
}
