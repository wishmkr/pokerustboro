//! Translated from `src/siirtc.c` by tools/rustport/c2rs.py.
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
    dead_code,
    unused_assignments,
    unused_variables
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

pub(crate) static sDummy: crate::global::Global<u16> = crate::global::Global::new(0);
pub(crate) static sLocked: crate::global::Global<u8> = crate::global::Global::new(0);

unsafe extern "C" {
    static mut GPIOPortDirection: u16;
}

#[unsafe(no_mangle)]
pub unsafe fn SiiRtcUnprotect() {
    EnableGpioPortRead();
    sLocked.set(FALSE);
}
#[unsafe(no_mangle)]
pub unsafe fn SiiRtcProtect() {
    DisableGpioPortRead();
    sLocked.set(TRUE);
}
#[unsafe(no_mangle)]
pub unsafe fn SiiRtcProbe() -> u8 {
    let mut rtc: SiiRtcInfo = zeroed();
    if SiiRtcGetStatus(&raw mut rtc) == 0 {
        return 0;
    }
    let mut errorCode: u8 = 0;
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
    errorCode << 4 | 1
}
#[unsafe(no_mangle)]
pub unsafe fn SiiRtcReset() -> u8 {
    let mut rtc: SiiRtcInfo = zeroed();
    if sLocked.get() == TRUE {
        return FALSE;
    }
    sLocked.set(TRUE);
    volatile_write(0x80000C4_usize as *mut u16, SCK_HI);
    volatile_write(0x80000C4_usize as *mut u16, 5);
    volatile_write(0x80000C6_usize as *mut u16, DIR_ALL_OUT);
    WriteCommand(96);
    volatile_write(0x80000C4_usize as *mut u16, SCK_HI);
    volatile_write(0x80000C4_usize as *mut u16, SCK_HI);
    sLocked.set(FALSE);
    rtc.status = SIIRTCINFO_24HOUR;
    let result: u8 = SiiRtcSetStatus(&raw mut rtc);
    result
}
#[unsafe(no_mangle)]
pub unsafe fn SiiRtcGetStatus(rtc: *mut SiiRtcInfo) -> u8 {
    if sLocked.get() == TRUE {
        return FALSE;
    }
    sLocked.set(TRUE);
    volatile_write(0x80000C4_usize as *mut u16, SCK_HI);
    volatile_write(0x80000C4_usize as *mut u16, 5);
    volatile_write(0x80000C6_usize as *mut u16, DIR_ALL_OUT);
    WriteCommand(99);
    volatile_write(0x80000C6_usize as *mut u16, 5);
    let statusData: u8 = ReadData();
    (*rtc).status = statusData & 192
        | ((statusData as i32 & STATUS_INTAE) >> 3) as u8
        | ((statusData as i32 & STATUS_INTME) >> 2) as u8
        | ((statusData as i32 & STATUS_INTFE) >> 1) as u8;
    volatile_write(0x80000C4_usize as *mut u16, SCK_HI);
    volatile_write(0x80000C4_usize as *mut u16, SCK_HI);
    sLocked.set(FALSE);
    TRUE
}
pub unsafe fn SiiRtcSetStatus(rtc: *mut SiiRtcInfo) -> u8 {
    if sLocked.get() == TRUE {
        return FALSE;
    }
    sLocked.set(TRUE);
    volatile_write(0x80000C4_usize as *mut u16, SCK_HI);
    volatile_write(0x80000C4_usize as *mut u16, 5);
    let statusData: u8 = STATUS_24HOUR
        | ((*rtc).status & SIIRTCINFO_INTAE) << 3
        | ((*rtc).status & 0x02) << 2
        | ((*rtc).status & 0x01) << 1;
    volatile_write(0x80000C6_usize as *mut u16, DIR_ALL_OUT);
    WriteCommand(98);
    WriteData(statusData);
    volatile_write(0x80000C4_usize as *mut u16, SCK_HI);
    volatile_write(0x80000C4_usize as *mut u16, SCK_HI);
    sLocked.set(FALSE);
    TRUE
}
#[unsafe(no_mangle)]
pub unsafe fn SiiRtcGetDateTime(rtc: *mut SiiRtcInfo) -> u8 {
    if sLocked.get() == TRUE {
        return FALSE;
    }
    sLocked.set(TRUE);
    volatile_write(0x80000C4_usize as *mut u16, SCK_HI);
    volatile_write(0x80000C4_usize as *mut u16, 5);
    volatile_write(0x80000C6_usize as *mut u16, DIR_ALL_OUT);
    WriteCommand(101);
    volatile_write(0x80000C6_usize as *mut u16, 5);
    for i in 0..7u8 {
        *(rtc as *mut u8).at(i as u32) = ReadData();
    }
    *(rtc as *mut u8).at(4) &= 0x7F;
    volatile_write(0x80000C4_usize as *mut u16, SCK_HI);
    volatile_write(0x80000C4_usize as *mut u16, SCK_HI);
    sLocked.set(FALSE);
    TRUE
}
pub unsafe fn SiiRtcSetDateTime(rtc: *mut SiiRtcInfo) -> u8 {
    if sLocked.get() == TRUE {
        return FALSE;
    }
    sLocked.set(TRUE);
    volatile_write(0x80000C4_usize as *mut u16, SCK_HI);
    volatile_write(0x80000C4_usize as *mut u16, 5);
    volatile_write(0x80000C6_usize as *mut u16, DIR_ALL_OUT);
    WriteCommand(100);
    for i in 0..7u8 {
        WriteData(*(rtc as *mut u8).at(i as u32));
    }
    volatile_write(0x80000C4_usize as *mut u16, SCK_HI);
    volatile_write(0x80000C4_usize as *mut u16, SCK_HI);
    sLocked.set(FALSE);
    TRUE
}
pub unsafe fn SiiRtcGetTime(rtc: *mut SiiRtcInfo) -> u8 {
    if sLocked.get() == TRUE {
        return FALSE;
    }
    sLocked.set(TRUE);
    volatile_write(0x80000C4_usize as *mut u16, SCK_HI);
    volatile_write(0x80000C4_usize as *mut u16, 5);
    volatile_write(0x80000C6_usize as *mut u16, DIR_ALL_OUT);
    WriteCommand(103);
    volatile_write(0x80000C6_usize as *mut u16, 5);
    for i in 0..3u8 {
        *(rtc as *mut u8).at(4 + i as u32) = ReadData();
    }
    *(rtc as *mut u8).at(4) &= 0x7F;
    volatile_write(0x80000C4_usize as *mut u16, SCK_HI);
    volatile_write(0x80000C4_usize as *mut u16, SCK_HI);
    sLocked.set(FALSE);
    TRUE
}
pub unsafe fn SiiRtcSetTime(rtc: *mut SiiRtcInfo) -> u8 {
    if sLocked.get() == TRUE {
        return FALSE;
    }
    sLocked.set(TRUE);
    volatile_write(0x80000C4_usize as *mut u16, SCK_HI);
    volatile_write(0x80000C4_usize as *mut u16, 5);
    volatile_write(0x80000C6_usize as *mut u16, DIR_ALL_OUT);
    WriteCommand(102);
    for i in 0..3u8 {
        WriteData(*(rtc as *mut u8).at(4 + i as u32));
    }
    volatile_write(0x80000C4_usize as *mut u16, SCK_HI);
    volatile_write(0x80000C4_usize as *mut u16, SCK_HI);
    sLocked.set(FALSE);
    TRUE
}
unsafe fn SiiRtcSetAlarm(rtc: *mut SiiRtcInfo) -> u8 {
    let mut alarmData: CArray<u8, 2> = zeroed();
    if sLocked.get() == TRUE {
        return FALSE;
    }
    sLocked.set(TRUE);
    alarmData[0] = ((*rtc).alarmHour & 0xF) + 10 * ((*rtc).alarmHour >> 4 & 0xF);
    if alarmData[0] < 12 {
        alarmData[0] = (*rtc).alarmHour;
    } else {
        alarmData[0] = (*rtc).alarmHour | ALARM_PM;
    }
    alarmData[1] = (*rtc).alarmMinute;
    volatile_write(0x80000C4_usize as *mut u16, SCK_HI);
    volatile_write(0x80000C4_usize as *mut u16, 5);
    volatile_write(&raw mut GPIOPortDirection, DIR_ALL_OUT);
    WriteCommand(104);
    for i in 0..2u8 {
        WriteData(alarmData[i]);
    }
    volatile_write(0x80000C4_usize as *mut u16, SCK_HI);
    volatile_write(0x80000C4_usize as *mut u16, SCK_HI);
    sLocked.set(FALSE);
    TRUE
}
unsafe fn WriteCommand(value: u8) -> i32 {
    let mut temp: u8 = 0;
    for i in 0..8u8 {
        temp = shr_i32(value as i32, 7 - i as u32) as u8 & 1;
        volatile_write(0x80000C4_usize as *mut u16, (temp as u16) << 1 | CS_HI);
        volatile_write(0x80000C4_usize as *mut u16, (temp as u16) << 1 | CS_HI);
        volatile_write(0x80000C4_usize as *mut u16, (temp as u16) << 1 | CS_HI);
        volatile_write(0x80000C4_usize as *mut u16, (temp as u16) << 1 | 1 | CS_HI);
    }
    0
}
unsafe fn WriteData(value: u8) -> i32 {
    let mut temp: u8 = 0;
    for i in 0..8u8 {
        temp = shr_i32(value as i32, i as u32) as u8 & 1;
        volatile_write(0x80000C4_usize as *mut u16, (temp as u16) << 1 | CS_HI);
        volatile_write(0x80000C4_usize as *mut u16, (temp as u16) << 1 | CS_HI);
        volatile_write(0x80000C4_usize as *mut u16, (temp as u16) << 1 | CS_HI);
        volatile_write(0x80000C4_usize as *mut u16, (temp as u16) << 1 | 1 | CS_HI);
    }
    0
}
unsafe fn ReadData() -> u8 {
    let mut temp: u8 = 0;
    let mut value: u8 = 0;
    for i in 0..8u8 {
        volatile_write(0x80000C4_usize as *mut u16, CS_HI);
        volatile_write(0x80000C4_usize as *mut u16, CS_HI);
        volatile_write(0x80000C4_usize as *mut u16, CS_HI);
        volatile_write(0x80000C4_usize as *mut u16, CS_HI);
        volatile_write(0x80000C4_usize as *mut u16, CS_HI);
        volatile_write(0x80000C4_usize as *mut u16, 5);
        temp = (((0x80000C4_usize as *mut u16).read_volatile() as i32 & SIO_HI) >> 1) as u8;
        value = value >> 1 | temp << 7;
    }
    value
}
unsafe fn EnableGpioPortRead() {
    volatile_write(0x80000C8_usize as *mut u16, TRUE as u16);
}
unsafe fn DisableGpioPortRead() {
    volatile_write(0x80000C8_usize as *mut u16, FALSE as u16);
}
