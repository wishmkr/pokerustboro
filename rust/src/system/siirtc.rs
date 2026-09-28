//! Translated from `src/siirtc.c` by tools/rustport/c2rs.py.
#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
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
    static_mut_refs,
    unsafe_op_in_unsafe_fn,
    clippy::all,
    clashing_extern_declarations,
    unpredictable_function_pointer_comparisons,
    dangerous_implicit_autorefs
)]

#[allow(unused_imports)]
use crate::c::*;
#[allow(unused_imports)]
use crate::consts::*;
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use core::ffi::c_void;
#[allow(unused_imports)]
use core::mem::zeroed;
#[allow(unused_imports)]
use core::ptr::null_mut;
// Data tables (translate with cdata.py): AgbLibRtcVersion

const ALARM_PM: u8 = 128;
const CS_HI: u16 = 4;
const DIR_0_OUT: i32 = 1;
const DIR_1_IN: i32 = 0;
const DIR_2_OUT: i32 = 4;
const DIR_ALL_OUT: u16 = 7;
const RD: i32 = 1;
const SCK_HI: u16 = 1;
const SIO_HI: i32 = 2;
const STATUS_24HOUR: u8 = 64;
const STATUS_INTAE: i32 = 32;
const STATUS_INTFE: i32 = 2;
const STATUS_INTME: i32 = 8;
const STATUS_POWER: i32 = 128;
const TEST_MODE: i32 = 128;
const WR: i32 = 0;

pub(crate) static mut sDummy: u16 = 0;
pub(crate) static mut sLocked: u8 = 0;

unsafe extern "C" {
    static mut GPIOPortDirection: u16;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn SiiRtcUnprotect() {
    EnableGpioPortRead();
    sLocked = FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SiiRtcProtect() {
    DisableGpioPortRead();
    sLocked = TRUE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SiiRtcProbe() -> u8 {
    let mut errorCode: u8 = 0;
    let mut rtc: SiiRtcInfo = zeroed();
    if SiiRtcGetStatus(&raw mut rtc) == 0 {
        return 0;
    }
    errorCode = 0;
    if rtc.status as i32 & 192 == SIIRTCINFO_POWER || rtc.status as i32 & 192 == 0 {
        if SiiRtcReset() == 0 {
            return 0;
        }
        errorCode += 1;
    }
    SiiRtcGetTime(&raw mut rtc);
    if rtc.second as i32 & TEST_MODE != 0 {
        if SiiRtcReset() == 0 {
            return errorCode << 4 & 0xF0;
        }
        errorCode += 1;
    }
    return errorCode << 4 | 1;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SiiRtcReset() -> u8 {
    let mut result: u8 = 0;
    let mut rtc: SiiRtcInfo = zeroed();
    if sLocked == TRUE {
        return FALSE;
    }
    sLocked = TRUE;
    volatile_write(0x80000C4 as usize as *mut u16, SCK_HI);
    volatile_write(0x80000C4 as usize as *mut u16, 5);
    volatile_write(0x80000C6 as usize as *mut u16, DIR_ALL_OUT);
    WriteCommand(96);
    volatile_write(0x80000C4 as usize as *mut u16, SCK_HI);
    volatile_write(0x80000C4 as usize as *mut u16, SCK_HI);
    sLocked = FALSE;
    rtc.status = SIIRTCINFO_24HOUR;
    result = SiiRtcSetStatus(&raw mut rtc);
    return result;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SiiRtcGetStatus(rtc: *mut SiiRtcInfo) -> u8 {
    let mut statusData: u8 = 0;
    if sLocked == TRUE {
        return FALSE;
    }
    sLocked = TRUE;
    volatile_write(0x80000C4 as usize as *mut u16, SCK_HI);
    volatile_write(0x80000C4 as usize as *mut u16, 5);
    volatile_write(0x80000C6 as usize as *mut u16, DIR_ALL_OUT);
    WriteCommand(99);
    volatile_write(0x80000C6 as usize as *mut u16, 5);
    statusData = ReadData();
    (*rtc).status = statusData & 192
        | ((statusData as i32 & STATUS_INTAE) >> 3) as u8
        | ((statusData as i32 & STATUS_INTME) >> 2) as u8
        | ((statusData as i32 & STATUS_INTFE) >> 1) as u8;
    volatile_write(0x80000C4 as usize as *mut u16, SCK_HI);
    volatile_write(0x80000C4 as usize as *mut u16, SCK_HI);
    sLocked = FALSE;
    return TRUE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SiiRtcSetStatus(rtc: *mut SiiRtcInfo) -> u8 {
    let mut statusData: u8 = 0;
    if sLocked == TRUE {
        return FALSE;
    }
    sLocked = TRUE;
    volatile_write(0x80000C4 as usize as *mut u16, SCK_HI);
    volatile_write(0x80000C4 as usize as *mut u16, 5);
    statusData = STATUS_24HOUR
        | ((*rtc).status & SIIRTCINFO_INTAE) << 3
        | ((*rtc).status & 0x02) << 2
        | ((*rtc).status & 0x01) << 1;
    volatile_write(0x80000C6 as usize as *mut u16, DIR_ALL_OUT);
    WriteCommand(98);
    WriteData(statusData);
    volatile_write(0x80000C4 as usize as *mut u16, SCK_HI);
    volatile_write(0x80000C4 as usize as *mut u16, SCK_HI);
    sLocked = FALSE;
    return TRUE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SiiRtcGetDateTime(rtc: *mut SiiRtcInfo) -> u8 {
    let mut i: u8 = 0;
    if sLocked == TRUE {
        return FALSE;
    }
    sLocked = TRUE;
    volatile_write(0x80000C4 as usize as *mut u16, SCK_HI);
    volatile_write(0x80000C4 as usize as *mut u16, 5);
    volatile_write(0x80000C6 as usize as *mut u16, DIR_ALL_OUT);
    WriteCommand(101);
    volatile_write(0x80000C6 as usize as *mut u16, 5);
    i = 0;
    while i < 7 {
        *(rtc as *mut u8).at(0 + i as u32) = ReadData();
        i += 1;
    }
    *(rtc as *mut u8).at(4) &= 0x7F;
    volatile_write(0x80000C4 as usize as *mut u16, SCK_HI);
    volatile_write(0x80000C4 as usize as *mut u16, SCK_HI);
    sLocked = FALSE;
    return TRUE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SiiRtcSetDateTime(rtc: *mut SiiRtcInfo) -> u8 {
    let mut i: u8 = 0;
    if sLocked == TRUE {
        return FALSE;
    }
    sLocked = TRUE;
    volatile_write(0x80000C4 as usize as *mut u16, SCK_HI);
    volatile_write(0x80000C4 as usize as *mut u16, 5);
    volatile_write(0x80000C6 as usize as *mut u16, DIR_ALL_OUT);
    WriteCommand(100);
    i = 0;
    while i < 7 {
        WriteData(*(rtc as *mut u8).at(0 + i as u32));
        i += 1;
    }
    volatile_write(0x80000C4 as usize as *mut u16, SCK_HI);
    volatile_write(0x80000C4 as usize as *mut u16, SCK_HI);
    sLocked = FALSE;
    return TRUE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SiiRtcGetTime(rtc: *mut SiiRtcInfo) -> u8 {
    let mut i: u8 = 0;
    if sLocked == TRUE {
        return FALSE;
    }
    sLocked = TRUE;
    volatile_write(0x80000C4 as usize as *mut u16, SCK_HI);
    volatile_write(0x80000C4 as usize as *mut u16, 5);
    volatile_write(0x80000C6 as usize as *mut u16, DIR_ALL_OUT);
    WriteCommand(103);
    volatile_write(0x80000C6 as usize as *mut u16, 5);
    i = 0;
    while i < 3 {
        *(rtc as *mut u8).at(4 + i as u32) = ReadData();
        i += 1;
    }
    *(rtc as *mut u8).at(4) &= 0x7F;
    volatile_write(0x80000C4 as usize as *mut u16, SCK_HI);
    volatile_write(0x80000C4 as usize as *mut u16, SCK_HI);
    sLocked = FALSE;
    return TRUE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SiiRtcSetTime(rtc: *mut SiiRtcInfo) -> u8 {
    let mut i: u8 = 0;
    if sLocked == TRUE {
        return FALSE;
    }
    sLocked = TRUE;
    volatile_write(0x80000C4 as usize as *mut u16, SCK_HI);
    volatile_write(0x80000C4 as usize as *mut u16, 5);
    volatile_write(0x80000C6 as usize as *mut u16, DIR_ALL_OUT);
    WriteCommand(102);
    i = 0;
    while i < 3 {
        WriteData(*(rtc as *mut u8).at(4 + i as u32));
        i += 1;
    }
    volatile_write(0x80000C4 as usize as *mut u16, SCK_HI);
    volatile_write(0x80000C4 as usize as *mut u16, SCK_HI);
    sLocked = FALSE;
    return TRUE;
}
pub(crate) unsafe extern "C" fn SiiRtcSetAlarm(rtc: *mut SiiRtcInfo) -> u8 {
    let mut i: u8 = 0;
    let mut alarmData: CArray<u8, 2> = zeroed();
    if sLocked == TRUE {
        return FALSE;
    }
    sLocked = TRUE;
    alarmData[0] = ((*rtc).alarmHour & 0xF) + 10 * ((*rtc).alarmHour >> 4 & 0xF);
    if alarmData[0] < 12 {
        alarmData[0] = (*rtc).alarmHour | 0x00;
    } else {
        alarmData[0] = (*rtc).alarmHour | ALARM_PM;
    }
    alarmData[1] = (*rtc).alarmMinute;
    volatile_write(0x80000C4 as usize as *mut u16, SCK_HI);
    volatile_write(0x80000C4 as usize as *mut u16, 5);
    volatile_write(&raw mut GPIOPortDirection, DIR_ALL_OUT);
    WriteCommand(104);
    i = 0;
    while i < 2 {
        WriteData(alarmData[i]);
        i += 1;
    }
    volatile_write(0x80000C4 as usize as *mut u16, SCK_HI);
    volatile_write(0x80000C4 as usize as *mut u16, SCK_HI);
    sLocked = FALSE;
    return TRUE;
}
pub(crate) unsafe extern "C" fn WriteCommand(value: u8) -> i32 {
    let mut i: u8 = 0;
    let mut temp: u8 = 0;
    i = 0;
    while i < 8 {
        temp = shr_i32(value as i32, 7 - i as u32) as u8 & 1;
        volatile_write(0x80000C4 as usize as *mut u16, (temp as u16) << 1 | CS_HI);
        volatile_write(0x80000C4 as usize as *mut u16, (temp as u16) << 1 | CS_HI);
        volatile_write(0x80000C4 as usize as *mut u16, (temp as u16) << 1 | CS_HI);
        volatile_write(
            0x80000C4 as usize as *mut u16,
            (temp as u16) << 1 | 1 | CS_HI,
        );
        i += 1;
    }
    return 0;
}
pub(crate) unsafe extern "C" fn WriteData(value: u8) -> i32 {
    let mut i: u8 = 0;
    let mut temp: u8 = 0;
    i = 0;
    while i < 8 {
        temp = shr_i32(value as i32, i as u32) as u8 & 1;
        volatile_write(0x80000C4 as usize as *mut u16, (temp as u16) << 1 | CS_HI);
        volatile_write(0x80000C4 as usize as *mut u16, (temp as u16) << 1 | CS_HI);
        volatile_write(0x80000C4 as usize as *mut u16, (temp as u16) << 1 | CS_HI);
        volatile_write(
            0x80000C4 as usize as *mut u16,
            (temp as u16) << 1 | 1 | CS_HI,
        );
        i += 1;
    }
    return 0;
}
pub(crate) unsafe extern "C" fn ReadData() -> u8 {
    let mut i: u8 = 0;
    let mut temp: u8 = 0;
    let mut value: u8 = 0;
    value = 0;
    i = 0;
    while i < 8 {
        volatile_write(0x80000C4 as usize as *mut u16, CS_HI);
        volatile_write(0x80000C4 as usize as *mut u16, CS_HI);
        volatile_write(0x80000C4 as usize as *mut u16, CS_HI);
        volatile_write(0x80000C4 as usize as *mut u16, CS_HI);
        volatile_write(0x80000C4 as usize as *mut u16, CS_HI);
        volatile_write(0x80000C4 as usize as *mut u16, 5);
        temp = (((0x80000C4 as usize as *mut u16).read_volatile() as i32 & SIO_HI) >> 1) as u8;
        value = value >> 1 | temp << 7;
        i += 1;
    }
    return value;
}
pub(crate) unsafe extern "C" fn EnableGpioPortRead() {
    volatile_write(0x80000C8 as usize as *mut u16, TRUE as u16);
}
pub(crate) unsafe extern "C" fn DisableGpioPortRead() {
    volatile_write(0x80000C8 as usize as *mut u16, FALSE as u16);
}
