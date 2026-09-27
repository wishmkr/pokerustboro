//! Translated from `src/siirtc.c` by tools/rustport/c2rs.py, then reviewed.
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

// Data tables (translate with cdata.py): AgbLibRtcVersion
#[allow(unused_imports)]
use crate::data::siirtc::*;

pub(crate) static mut sDummy: u16 = 0u16;
pub(crate) static mut sLocked: u8 = 0u8;

unsafe extern "C" {
    static mut GPIOPortDirection: u8;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn SiiRtcUnprotect() {
    unsafe {
        EnableGpioPortRead();
        ((&raw mut sLocked).cast::<u8>().cast::<u8>()).write(0u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SiiRtcProtect() {
    unsafe {
        DisableGpioPortRead();
        ((&raw mut sLocked).cast::<u8>().cast::<u8>()).write(1u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SiiRtcProbe() -> u8 {
    unsafe {
        let mut errorCode: u8 = 0u8;
        let mut rtc = crate::ffi::Align4([0u8; 12]);
        if !((SiiRtcGetStatus((&raw mut rtc).cast::<u8>())) != 0) {
            return 0u8;
        }
        errorCode = 0u8;
        if (((((((&raw mut rtc).cast::<u8>()).wrapping_add(7)).read()) as i32) & 192i32) == 128i32)
            || (((((((&raw mut rtc).cast::<u8>()).wrapping_add(7)).read()) as i32) & 192i32)
                == 0i32)
        {
            if !((SiiRtcReset()) != 0) {
                return 0u8;
            }
            errorCode = (errorCode).wrapping_add(1);
        }
        SiiRtcGetTime((&raw mut rtc).cast::<u8>());
        if ((((((&raw mut rtc).cast::<u8>()).wrapping_add(6)).read()) as i32) & 128i32) != 0 {
            if !((SiiRtcReset()) != 0) {
                return (((((errorCode) as i32) << 4) & 240i32) as u8);
            }
            errorCode = (errorCode).wrapping_add(1);
        }
        return (((((errorCode) as i32) << 4) | 1i32) as u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SiiRtcReset() -> u8 {
    unsafe {
        let mut result: u8 = 0u8;
        let mut rtc = crate::ffi::Align4([0u8; 12]);
        if ((((&raw mut sLocked).cast::<u8>().cast::<u8>()).read()) as i32) == 1i32 {
            return 0u8;
        }
        ((&raw mut sLocked).cast::<u8>().cast::<u8>()).write(1u8);
        crate::c::volatile_write(((134217924i32) as usize as *mut u16), 1u16);
        crate::c::volatile_write(((134217924i32) as usize as *mut u16), 5u16);
        crate::c::volatile_write(((134217926i32) as usize as *mut u16), 7u16);
        WriteCommand(96u8);
        crate::c::volatile_write(((134217924i32) as usize as *mut u16), 1u16);
        crate::c::volatile_write(((134217924i32) as usize as *mut u16), 1u16);
        ((&raw mut sLocked).cast::<u8>().cast::<u8>()).write(0u8);
        (((&raw mut rtc).cast::<u8>()).wrapping_add(7)).write(64u8);
        result = SiiRtcSetStatus((&raw mut rtc).cast::<u8>());
        return result;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SiiRtcGetStatus(rtc: *mut u8) -> u8 {
    unsafe {
        let mut rtc = rtc;
        let mut statusData: u8 = 0u8;
        if ((((&raw mut sLocked).cast::<u8>().cast::<u8>()).read()) as i32) == 1i32 {
            return 0u8;
        }
        ((&raw mut sLocked).cast::<u8>().cast::<u8>()).write(1u8);
        crate::c::volatile_write(((134217924i32) as usize as *mut u16), 1u16);
        crate::c::volatile_write(((134217924i32) as usize as *mut u16), 5u16);
        crate::c::volatile_write(((134217926i32) as usize as *mut u16), 7u16);
        WriteCommand(99u8);
        crate::c::volatile_write(((134217926i32) as usize as *mut u16), 5u16);
        statusData = ReadData();
        ((rtc).wrapping_add(7)).write(
            (((((((statusData) as i32) & 192i32) | ((((statusData) as i32) & 32i32) >> 3))
                | ((((statusData) as i32) & 8i32) >> 2))
                | ((((statusData) as i32) & 2i32) >> 1)) as u8),
        );
        crate::c::volatile_write(((134217924i32) as usize as *mut u16), 1u16);
        crate::c::volatile_write(((134217924i32) as usize as *mut u16), 1u16);
        ((&raw mut sLocked).cast::<u8>().cast::<u8>()).write(0u8);
        return 1u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SiiRtcSetStatus(rtc: *mut u8) -> u8 {
    unsafe {
        let mut rtc = rtc;
        let mut statusData: u8 = 0u8;
        if ((((&raw mut sLocked).cast::<u8>().cast::<u8>()).read()) as i32) == 1i32 {
            return 0u8;
        }
        ((&raw mut sLocked).cast::<u8>().cast::<u8>()).write(1u8);
        crate::c::volatile_write(((134217924i32) as usize as *mut u16), 1u16);
        crate::c::volatile_write(((134217924i32) as usize as *mut u16), 5u16);
        statusData = ((((64i32 | ((((((rtc).wrapping_add(7)).read()) as i32) & 4i32) << 3))
            | ((((((rtc).wrapping_add(7)).read()) as i32) & 2i32) << 2))
            | ((((((rtc).wrapping_add(7)).read()) as i32) & 1i32) << 1))
            as u8);
        crate::c::volatile_write(((134217926i32) as usize as *mut u16), 7u16);
        WriteCommand(98u8);
        WriteData(statusData);
        crate::c::volatile_write(((134217924i32) as usize as *mut u16), 1u16);
        crate::c::volatile_write(((134217924i32) as usize as *mut u16), 1u16);
        ((&raw mut sLocked).cast::<u8>().cast::<u8>()).write(0u8);
        return 1u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SiiRtcGetDateTime(rtc: *mut u8) -> u8 {
    unsafe {
        let mut rtc = rtc;
        let mut i: u8 = 0u8;
        if ((((&raw mut sLocked).cast::<u8>().cast::<u8>()).read()) as i32) == 1i32 {
            return 0u8;
        }
        ((&raw mut sLocked).cast::<u8>().cast::<u8>()).write(1u8);
        crate::c::volatile_write(((134217924i32) as usize as *mut u16), 1u16);
        crate::c::volatile_write(((134217924i32) as usize as *mut u16), 5u16);
        crate::c::volatile_write(((134217926i32) as usize as *mut u16), 7u16);
        WriteCommand(101u8);
        crate::c::volatile_write(((134217926i32) as usize as *mut u16), 5u16);
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as u32) < 7u32) {
                    break 'l1;
                }
                'l2: {
                    ((rtc).wrapping_offset((((0u32).wrapping_add(((i) as u32))) as i32) as isize))
                        .write(ReadData());
                }
                i = (i).wrapping_add(1);
            }
        }
        let __p1 = (rtc).wrapping_offset(4);
        (__p1).write((((((__p1).read()) as i32) & 127i32) as u8));
        crate::c::volatile_write(((134217924i32) as usize as *mut u16), 1u16);
        crate::c::volatile_write(((134217924i32) as usize as *mut u16), 1u16);
        ((&raw mut sLocked).cast::<u8>().cast::<u8>()).write(0u8);
        return 1u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SiiRtcSetDateTime(rtc: *mut u8) -> u8 {
    unsafe {
        let mut rtc = rtc;
        let mut i: u8 = 0u8;
        if ((((&raw mut sLocked).cast::<u8>().cast::<u8>()).read()) as i32) == 1i32 {
            return 0u8;
        }
        ((&raw mut sLocked).cast::<u8>().cast::<u8>()).write(1u8);
        crate::c::volatile_write(((134217924i32) as usize as *mut u16), 1u16);
        crate::c::volatile_write(((134217924i32) as usize as *mut u16), 5u16);
        crate::c::volatile_write(((134217926i32) as usize as *mut u16), 7u16);
        WriteCommand(100u8);
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as u32) < 7u32) {
                    break 'l1;
                }
                'l2: {
                    WriteData(
                        ((rtc).wrapping_offset(
                            (((0u32).wrapping_add(((i) as u32))) as i32) as isize,
                        ))
                        .read(),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        crate::c::volatile_write(((134217924i32) as usize as *mut u16), 1u16);
        crate::c::volatile_write(((134217924i32) as usize as *mut u16), 1u16);
        ((&raw mut sLocked).cast::<u8>().cast::<u8>()).write(0u8);
        return 1u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SiiRtcGetTime(rtc: *mut u8) -> u8 {
    unsafe {
        let mut rtc = rtc;
        let mut i: u8 = 0u8;
        if ((((&raw mut sLocked).cast::<u8>().cast::<u8>()).read()) as i32) == 1i32 {
            return 0u8;
        }
        ((&raw mut sLocked).cast::<u8>().cast::<u8>()).write(1u8);
        crate::c::volatile_write(((134217924i32) as usize as *mut u16), 1u16);
        crate::c::volatile_write(((134217924i32) as usize as *mut u16), 5u16);
        crate::c::volatile_write(((134217926i32) as usize as *mut u16), 7u16);
        WriteCommand(103u8);
        crate::c::volatile_write(((134217926i32) as usize as *mut u16), 5u16);
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as u32) < 3u32) {
                    break 'l1;
                }
                'l2: {
                    ((rtc).wrapping_offset((((4u32).wrapping_add(((i) as u32))) as i32) as isize))
                        .write(ReadData());
                }
                i = (i).wrapping_add(1);
            }
        }
        let __p1 = (rtc).wrapping_offset(4);
        (__p1).write((((((__p1).read()) as i32) & 127i32) as u8));
        crate::c::volatile_write(((134217924i32) as usize as *mut u16), 1u16);
        crate::c::volatile_write(((134217924i32) as usize as *mut u16), 1u16);
        ((&raw mut sLocked).cast::<u8>().cast::<u8>()).write(0u8);
        return 1u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SiiRtcSetTime(rtc: *mut u8) -> u8 {
    unsafe {
        let mut rtc = rtc;
        let mut i: u8 = 0u8;
        if ((((&raw mut sLocked).cast::<u8>().cast::<u8>()).read()) as i32) == 1i32 {
            return 0u8;
        }
        ((&raw mut sLocked).cast::<u8>().cast::<u8>()).write(1u8);
        crate::c::volatile_write(((134217924i32) as usize as *mut u16), 1u16);
        crate::c::volatile_write(((134217924i32) as usize as *mut u16), 5u16);
        crate::c::volatile_write(((134217926i32) as usize as *mut u16), 7u16);
        WriteCommand(102u8);
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as u32) < 3u32) {
                    break 'l1;
                }
                'l2: {
                    WriteData(
                        ((rtc).wrapping_offset(
                            (((4u32).wrapping_add(((i) as u32))) as i32) as isize,
                        ))
                        .read(),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        crate::c::volatile_write(((134217924i32) as usize as *mut u16), 1u16);
        crate::c::volatile_write(((134217924i32) as usize as *mut u16), 1u16);
        ((&raw mut sLocked).cast::<u8>().cast::<u8>()).write(0u8);
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn SiiRtcSetAlarm(rtc: *mut u8) -> u8 {
    unsafe {
        let mut rtc = rtc;
        let mut i: u8 = 0u8;
        let mut alarmData = crate::ffi::Align4([0u8; 2]);
        if ((((&raw mut sLocked).cast::<u8>().cast::<u8>()).read()) as i32) == 1i32 {
            return 0u8;
        }
        ((&raw mut sLocked).cast::<u8>().cast::<u8>()).write(1u8);
        ((&raw mut alarmData).cast::<u8>()).write(
            (((((((rtc).wrapping_add(8)).read()) as i32) & 15i32).wrapping_add(
                (10i32).wrapping_mul(((((((rtc).wrapping_add(8)).read()) as i32) >> 4) & 15i32)),
            )) as u8),
        );
        if ((((&raw mut alarmData).cast::<u8>()).read()) as i32) < 12i32 {
            ((&raw mut alarmData).cast::<u8>())
                .write(((((((rtc).wrapping_add(8)).read()) as i32) | 0i32) as u8));
        } else {
            ((&raw mut alarmData).cast::<u8>())
                .write(((((((rtc).wrapping_add(8)).read()) as i32) | 128i32) as u8));
        }
        (((&raw mut alarmData).cast::<u8>()).wrapping_offset(1))
            .write(((rtc).wrapping_add(9)).read());
        crate::c::volatile_write(((134217924i32) as usize as *mut u16), 1u16);
        crate::c::volatile_write(((134217924i32) as usize as *mut u16), 5u16);
        crate::c::volatile_write((&raw mut GPIOPortDirection).cast::<u16>(), 7u16);
        WriteCommand(104u8);
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 2i32) {
                    break 'l1;
                }
                'l2: {
                    WriteData(
                        (((&raw mut alarmData).cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize))
                        .read(),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        crate::c::volatile_write(((134217924i32) as usize as *mut u16), 1u16);
        crate::c::volatile_write(((134217924i32) as usize as *mut u16), 1u16);
        ((&raw mut sLocked).cast::<u8>().cast::<u8>()).write(0u8);
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn WriteCommand(value: u8) -> i32 {
    unsafe {
        let mut value = value;
        let mut i: u8 = 0u8;
        let mut temp: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 8i32) {
                    break 'l1;
                }
                'l2: {
                    temp = ((crate::c::shr_i32(
                        ((value) as i32),
                        (((7i32).wrapping_sub(((i) as i32))) as u32),
                    ) & 1i32) as u8);
                    crate::c::volatile_write(
                        ((134217924i32) as usize as *mut u16),
                        (((((temp) as i32) << 1) | 4i32) as u16),
                    );
                    crate::c::volatile_write(
                        ((134217924i32) as usize as *mut u16),
                        (((((temp) as i32) << 1) | 4i32) as u16),
                    );
                    crate::c::volatile_write(
                        ((134217924i32) as usize as *mut u16),
                        (((((temp) as i32) << 1) | 4i32) as u16),
                    );
                    crate::c::volatile_write(
                        ((134217924i32) as usize as *mut u16),
                        ((((((temp) as i32) << 1) | 1i32) | 4i32) as u16),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        return 0i32;
    }
}
pub(crate) unsafe extern "C" fn WriteData(value: u8) -> i32 {
    unsafe {
        let mut value = value;
        let mut i: u8 = 0u8;
        let mut temp: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 8i32) {
                    break 'l1;
                }
                'l2: {
                    temp = ((crate::c::shr_i32(((value) as i32), ((i) as u32)) & 1i32) as u8);
                    crate::c::volatile_write(
                        ((134217924i32) as usize as *mut u16),
                        (((((temp) as i32) << 1) | 4i32) as u16),
                    );
                    crate::c::volatile_write(
                        ((134217924i32) as usize as *mut u16),
                        (((((temp) as i32) << 1) | 4i32) as u16),
                    );
                    crate::c::volatile_write(
                        ((134217924i32) as usize as *mut u16),
                        (((((temp) as i32) << 1) | 4i32) as u16),
                    );
                    crate::c::volatile_write(
                        ((134217924i32) as usize as *mut u16),
                        ((((((temp) as i32) << 1) | 1i32) | 4i32) as u16),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        return 0i32;
    }
}
pub(crate) unsafe extern "C" fn ReadData() -> u8 {
    unsafe {
        let mut i: u8 = 0u8;
        let mut temp: u8 = 0u8;
        let mut value: u8 = 0u8;
        value = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 8i32) {
                    break 'l1;
                }
                'l2: {
                    crate::c::volatile_write(((134217924i32) as usize as *mut u16), 4u16);
                    crate::c::volatile_write(((134217924i32) as usize as *mut u16), 4u16);
                    crate::c::volatile_write(((134217924i32) as usize as *mut u16), 4u16);
                    crate::c::volatile_write(((134217924i32) as usize as *mut u16), 4u16);
                    crate::c::volatile_write(((134217924i32) as usize as *mut u16), 4u16);
                    crate::c::volatile_write(((134217924i32) as usize as *mut u16), 5u16);
                    temp = (((((((134217924i32) as usize as *mut u16).read_volatile()) as i32)
                        & 2i32)
                        >> 1) as u8);
                    value = (((((value) as i32) >> 1) | (((temp) as i32) << 7)) as u8);
                }
                i = (i).wrapping_add(1);
            }
        }
        return value;
    }
}
pub(crate) unsafe extern "C" fn EnableGpioPortRead() {
    unsafe {
        crate::c::volatile_write(((134217928i32) as usize as *mut u16), 1u16);
    }
}
pub(crate) unsafe extern "C" fn DisableGpioPortRead() {
    unsafe {
        crate::c::volatile_write(((134217928i32) as usize as *mut u16), 0u16);
    }
}
