//! The cartridge real-time clock: reading it through the Seiko driver
//! (siirtc.c), validating the BCD date and time, and the local time offset
//! the game keeps in the save.

use crate::load_save::gSaveBlock2Ptr;

const RTC_INIT_ERROR: u16 = 0x001;
const RTC_INIT_WARNING: u16 = 0x002;
const RTC_ERR_FLAG_MASK: u16 = 0x0ff0;
const RTC_ERR_POWER_FAILURE: u16 = 0x0020;
const RTC_ERR_12HOUR_CLOCK: u16 = 0x0010;
const RTC_ERR_INVALID_YEAR: u16 = 0x0040;
const RTC_ERR_INVALID_MONTH: u16 = 0x0080;
const RTC_ERR_INVALID_DAY: u16 = 0x0100;
const RTC_ERR_INVALID_HOUR: u16 = 0x0200;
const RTC_ERR_INVALID_MINUTE: u16 = 0x0400;
const RTC_ERR_INVALID_SECOND: u16 = 0x0800;
const SIIRTCINFO_POWER: u8 = 0x80;
const SIIRTCINFO_24HOUR: u8 = 0x40;

const MONTH_FEB: i32 = 2;
const MONTH_COUNT: i32 = 12;
const HOURS_PER_DAY: i32 = 24;
const MINUTES_PER_HOUR: i32 = 60;
const SECONDS_PER_MINUTE: i32 = 60;

const REG_IME: *mut u16 = 0x0400_0208 as *mut u16;
const SB2_LOCAL_TIME_OFFSET: usize = 0x98;

/// `struct SiiRtcInfo`: BCD year, month, day, weekday, hour, minute, second,
/// then status and the alarm.
#[repr(C, align(4))]
#[derive(Clone, Copy)]
pub struct SiiRtcInfo {
    pub year: u8,
    pub month: u8,
    pub day: u8,
    pub day_of_week: u8,
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
    pub status: u8,
    pub alarm_hour: u8,
    pub alarm_minute: u8,
}

pub use crate::types::Time;

const RTC_DUMMY: SiiRtcInfo = SiiRtcInfo {
    year: 0,
    month: 1,
    day: 1,
    day_of_week: 0,
    hour: 0,
    minute: 0,
    second: 0,
    status: 0,
    alarm_hour: 0,
    alarm_minute: 0,
};

static NUM_DAYS_IN_MONTHS: [i32; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

static ERROR_STATUS: crate::global::Global<u16> = crate::global::Global::new(0);
static mut RTC: SiiRtcInfo = SiiRtcInfo {
    year: 0,
    month: 0,
    day: 0,
    day_of_week: 0,
    hour: 0,
    minute: 0,
    second: 0,
    status: 0,
    alarm_hour: 0,
    alarm_minute: 0,
};
static PROBE_RESULT: crate::global::Global<u8> = crate::global::Global::new(0);
static SAVED_IME: crate::global::Global<u16> = crate::global::Global::new(0);

#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gLocalTime: Time = Time {
    days: 0,
    hours: 0,
    minutes: 0,
    seconds: 0,
};

/// `SiiRtcUnprotect` with this module's view of its types.
#[inline]
unsafe fn SiiRtcUnprotect() {
    unsafe {
        crate::siirtc::SiiRtcUnprotect();
    }
}
/// `SiiRtcProbe` with this module's view of its types.
#[inline]
unsafe fn SiiRtcProbe() -> u8 {
    unsafe { crate::siirtc::SiiRtcProbe() }
}
/// `SiiRtcReset` with this module's view of its types.
#[inline]
unsafe fn SiiRtcReset() -> u8 {
    unsafe { crate::siirtc::SiiRtcReset() }
}
/// `SiiRtcGetStatus` with this module's view of its types.
#[inline]
unsafe fn SiiRtcGetStatus(a0: *mut SiiRtcInfo) -> u8 {
    unsafe { crate::siirtc::SiiRtcGetStatus(a0 as _) }
}
/// `SiiRtcGetDateTime` with this module's view of its types.
#[inline]
unsafe fn SiiRtcGetDateTime(a0: *mut SiiRtcInfo) -> u8 {
    unsafe { crate::siirtc::SiiRtcGetDateTime(a0 as _) }
}

/// `sNumDaysInMonths[month - 1]`. An invalid month reads outside the table
/// in the original; that read is kept (volatile, so it happens as written).
fn days_in_month(month: i32) -> i32 {
    let entry = NUM_DAYS_IN_MONTHS
        .as_ptr()
        .wrapping_offset((month - 1) as isize);
    unsafe { entry.read_volatile() }
}

#[unsafe(no_mangle)]
pub unsafe fn RtcDisableInterrupts() {
    unsafe { (SAVED_IME.as_ptr()).write(REG_IME.read_volatile()) };
    unsafe { REG_IME.write_volatile(0) };
}

#[unsafe(no_mangle)]
pub unsafe fn RtcRestoreInterrupts() {
    unsafe { REG_IME.write_volatile((SAVED_IME.as_ptr().cast_const()).read()) };
}

#[unsafe(no_mangle)]
pub fn ConvertBcdToBinary(bcd: u8) -> u32 {
    if bcd > 0x9f || (bcd & 0xf) > 9 {
        return 0xff;
    }
    10 * u32::from((bcd >> 4) & 0xf) + u32::from(bcd & 0xf)
}

#[unsafe(no_mangle)]
pub fn IsLeapYear(year: u32) -> u8 {
    u8::from((year % 4 == 0 && year % 100 != 0) || year % 400 == 0)
}

#[unsafe(no_mangle)]
pub fn ConvertDateToDayCount(year: u8, month: u8, day: u8) -> u16 {
    let mut day_count: u16 = 0;
    let mut i = i32::from(year) - 1;
    while i >= 0 {
        day_count = day_count.wrapping_add(365);
        if IsLeapYear(i as u32) == 1 {
            day_count = day_count.wrapping_add(1);
        }
        i -= 1;
    }
    for m in 0..i32::from(month) - 1 {
        day_count = day_count.wrapping_add(days_in_month(m + 1) as u16);
    }
    if i32::from(month) > MONTH_FEB && IsLeapYear(u32::from(year)) == 1 {
        day_count = day_count.wrapping_add(1);
    }
    day_count.wrapping_add(u16::from(day))
}

#[unsafe(no_mangle)]
pub unsafe fn RtcGetDayCount(rtc: *const SiiRtcInfo) -> u16 {
    let rtc = unsafe { rtc.read() };
    ConvertDateToDayCount(
        ConvertBcdToBinary(rtc.year) as u8,
        ConvertBcdToBinary(rtc.month) as u8,
        ConvertBcdToBinary(rtc.day) as u8,
    )
}

#[unsafe(no_mangle)]
pub unsafe fn RtcInit() {
    unsafe { (ERROR_STATUS.as_ptr()).write(0) };
    unsafe { RtcDisableInterrupts() };
    unsafe { SiiRtcUnprotect() };
    let probe = unsafe { SiiRtcProbe() };
    unsafe { (PROBE_RESULT.as_ptr()).write(probe) };
    unsafe { RtcRestoreInterrupts() };

    if probe & 0xf != 1 {
        unsafe { (ERROR_STATUS.as_ptr()).write(RTC_INIT_ERROR) };
        return;
    }
    let status = if probe & 0xf0 != 0 {
        RTC_INIT_WARNING
    } else {
        0
    };
    unsafe { (ERROR_STATUS.as_ptr()).write(status) };
    unsafe { RtcGetRawInfo(&raw mut RTC) };
    let checked = unsafe { RtcCheckInfo(&raw const RTC) };
    unsafe { (ERROR_STATUS.as_ptr()).write(checked) };
}

#[unsafe(no_mangle)]
pub unsafe fn RtcGetErrorStatus() -> u16 {
    unsafe { (ERROR_STATUS.as_ptr().cast_const()).read() }
}

#[unsafe(no_mangle)]
pub unsafe fn RtcGetInfo(rtc: *mut SiiRtcInfo) {
    if unsafe { (ERROR_STATUS.as_ptr().cast_const()).read() } & RTC_ERR_FLAG_MASK != 0 {
        unsafe { rtc.write(RTC_DUMMY) };
    } else {
        unsafe { RtcGetRawInfo(rtc) };
    }
}

#[unsafe(no_mangle)]
pub unsafe fn RtcGetDateTime(rtc: *mut SiiRtcInfo) {
    unsafe { RtcDisableInterrupts() };
    unsafe { SiiRtcGetDateTime(rtc) };
    unsafe { RtcRestoreInterrupts() };
}

#[unsafe(no_mangle)]
pub unsafe fn RtcGetStatus(rtc: *mut SiiRtcInfo) {
    unsafe { RtcDisableInterrupts() };
    unsafe { SiiRtcGetStatus(rtc) };
    unsafe { RtcRestoreInterrupts() };
}

#[unsafe(no_mangle)]
pub unsafe fn RtcGetRawInfo(rtc: *mut SiiRtcInfo) {
    unsafe { RtcGetStatus(rtc) };
    unsafe { RtcGetDateTime(rtc) };
}

#[unsafe(no_mangle)]
pub unsafe fn RtcCheckInfo(rtc: *const SiiRtcInfo) -> u16 {
    let rtc = unsafe { rtc.read() };
    let mut errors = 0u16;
    if rtc.status & SIIRTCINFO_POWER != 0 {
        errors |= RTC_ERR_POWER_FAILURE;
    }
    if rtc.status & SIIRTCINFO_24HOUR == 0 {
        errors |= RTC_ERR_12HOUR_CLOCK;
    }
    let year = ConvertBcdToBinary(rtc.year) as i32;
    if year == 0xff {
        errors |= RTC_ERR_INVALID_YEAR;
    }
    let month = ConvertBcdToBinary(rtc.month) as i32;
    if month == 0xff || month == 0 || month > MONTH_COUNT {
        errors |= RTC_ERR_INVALID_MONTH;
    }
    let day = ConvertBcdToBinary(rtc.day) as i32;
    if day == 0xff {
        errors |= RTC_ERR_INVALID_DAY;
    }
    let limit = if month == MONTH_FEB {
        i32::from(IsLeapYear(year as u32)) + days_in_month(month)
    } else {
        days_in_month(month)
    };
    if day > limit {
        errors |= RTC_ERR_INVALID_DAY;
    }
    if ConvertBcdToBinary(rtc.hour) as i32 > HOURS_PER_DAY {
        errors |= RTC_ERR_INVALID_HOUR;
    }
    if ConvertBcdToBinary(rtc.minute) as i32 > MINUTES_PER_HOUR {
        errors |= RTC_ERR_INVALID_MINUTE;
    }
    if ConvertBcdToBinary(rtc.second) as i32 > SECONDS_PER_MINUTE {
        errors |= RTC_ERR_INVALID_SECOND;
    }
    errors
}

#[unsafe(no_mangle)]
pub unsafe fn RtcReset() {
    unsafe { RtcDisableInterrupts() };
    unsafe { SiiRtcReset() };
    unsafe { RtcRestoreInterrupts() };
}

/// Borrows seconds, minutes and hours downwards like a subtraction by hand.
fn normalize(result: &mut Time) {
    if result.seconds < 0 {
        result.seconds = result.seconds.wrapping_add(SECONDS_PER_MINUTE as i8);
        result.minutes = result.minutes.wrapping_sub(1);
    }
    if result.minutes < 0 {
        result.minutes = result.minutes.wrapping_add(MINUTES_PER_HOUR as i8);
        result.hours = result.hours.wrapping_sub(1);
    }
    if result.hours < 0 {
        result.hours = result.hours.wrapping_add(HOURS_PER_DAY as i8);
        result.days = result.days.wrapping_sub(1);
    }
}

#[unsafe(no_mangle)]
pub unsafe fn RtcCalcTimeDifference(rtc: *const SiiRtcInfo, result: *mut Time, t: *const Time) {
    let days = unsafe { RtcGetDayCount(rtc) };
    let (rtc, t) = unsafe { (rtc.read(), t.read()) };
    let mut out = Time {
        seconds: (ConvertBcdToBinary(rtc.second) as i32 - i32::from(t.seconds)) as i8,
        minutes: (ConvertBcdToBinary(rtc.minute) as i32 - i32::from(t.minutes)) as i8,
        hours: (ConvertBcdToBinary(rtc.hour) as i32 - i32::from(t.hours)) as i8,
        days: (i32::from(days) - i32::from(t.days)) as i16,
    };
    normalize(&mut out);
    unsafe { result.write(out) };
}

unsafe fn local_time_offset() -> *mut Time {
    unsafe {
        (&raw const gSaveBlock2Ptr)
            .read()
            .cast::<u8>()
            .add(SB2_LOCAL_TIME_OFFSET)
            .cast()
    }
}

#[unsafe(no_mangle)]
pub unsafe fn RtcCalcLocalTime() {
    unsafe { RtcGetInfo(&raw mut RTC) };
    unsafe { RtcCalcTimeDifference(&raw const RTC, &raw mut gLocalTime, local_time_offset()) };
}

#[unsafe(no_mangle)]
pub unsafe fn RtcInitLocalTimeOffset(hour: i32, minute: i32) {
    unsafe { RtcCalcLocalTimeOffset(0, hour, minute, 0) };
}

#[unsafe(no_mangle)]
pub unsafe fn RtcCalcLocalTimeOffset(days: i32, hours: i32, minutes: i32, seconds: i32) {
    let local = Time {
        days: days as i16,
        hours: hours as i8,
        minutes: minutes as i8,
        seconds: seconds as i8,
    };
    unsafe { (&raw mut gLocalTime).write(local) };
    unsafe { RtcGetInfo(&raw mut RTC) };
    unsafe { RtcCalcTimeDifference(&raw const RTC, local_time_offset(), &raw const gLocalTime) };
}

#[unsafe(no_mangle)]
pub unsafe fn CalcTimeDifference(result: *mut Time, t1: *const Time, t2: *const Time) {
    let (t1, t2) = unsafe { (t1.read(), t2.read()) };
    let mut out = Time {
        seconds: t2.seconds.wrapping_sub(t1.seconds),
        minutes: t2.minutes.wrapping_sub(t1.minutes),
        hours: t2.hours.wrapping_sub(t1.hours),
        days: t2.days.wrapping_sub(t1.days),
    };
    normalize(&mut out);
    unsafe { result.write(out) };
}

#[unsafe(no_mangle)]
pub unsafe fn RtcGetMinuteCount() -> u32 {
    unsafe { RtcGetInfo(&raw mut RTC) };
    let rtc = unsafe { (&raw const RTC).read() };
    // Hour and minute are used as raw BCD here, as in the original.
    let days = u32::from(unsafe { RtcGetDayCount(&raw const RTC) });
    (HOURS_PER_DAY * MINUTES_PER_HOUR) as u32 * days
        + MINUTES_PER_HOUR as u32 * u32::from(rtc.hour)
        + u32::from(rtc.minute)
}

#[unsafe(no_mangle)]
pub unsafe fn RtcGetLocalDayCount() -> u32 {
    u32::from(unsafe { RtcGetDayCount(&raw const RTC) })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bcd_and_dates() {
        assert_eq!(ConvertBcdToBinary(0x59), 59);
        assert_eq!(ConvertBcdToBinary(0x5a), 0xff);
        assert_eq!(ConvertBcdToBinary(0xa0), 0xff);
        // 2000-01-01 is day 1; year 0 is a leap year.
        assert_eq!(ConvertDateToDayCount(0, 1, 1), 1);
        assert_eq!(ConvertDateToDayCount(0, 3, 1), 31 + 28 + 1 + 1);
        assert_eq!(ConvertDateToDayCount(1, 1, 1), 366 + 1);
    }

    #[test]
    fn time_difference_borrows() {
        let mut t = Time {
            days: 1,
            hours: 0,
            minutes: 0,
            seconds: -1,
        };
        normalize(&mut t);
        assert_eq!((t.days, t.hours, t.minutes, t.seconds), (0, 23, 59, 59));
    }
}
