use crate::ffi::{ClearDailyFlags, FlagGet, FlagSet, GetVarPointer, VarSet};
use core::ptr::{addr_of, addr_of_mut};

const FLAG_SYS_CLOCK_SET: u16 = 0x895;
const VAR_DAYS: u16 = 0x4040;
const MAIN_SAVED_CALLBACK_OFFSET: usize = 0x008;

type MainCallback = unsafe extern "C" fn();

#[repr(C, align(4))]
#[derive(Clone, Copy)]
struct Time {
    days: i16,
    hours: i8,
    minutes: i8,
    seconds: i8,
}

#[repr(C)]
struct SaveBlock2ClockView {
    prefix: [u8; 0xa0],
    last_berry_tree_update: Time,
}

unsafe extern "C" {
    static mut gLocalTime: Time;
    static mut gSaveBlock2Ptr: *mut SaveBlock2ClockView;
    static mut gMain: u8;
    fn RtcCalcLocalTime();
    fn InPokemonCenter() -> u8;
    fn UpdateDewfordTrendPerDay(days: u16);
    fn UpdateTVShowsPerDay(days: u16);
    fn UpdateWeatherPerDay(days: u16);
    fn UpdatePartyPokerusTime(days: u16);
    fn UpdateMirageRnd(days: u16);
    fn UpdateBirchState(days: u16);
    fn UpdateFrontierManiac(days: u16);
    fn UpdateFrontierGambler(days: u16);
    fn SetShoalItemFlag(days: u16);
    fn SetRandomLotteryNumber(days: u16);
    fn CalcTimeDifference(result: *mut Time, from: *mut Time, to: *mut Time);
    fn BerryTreeTimeUpdate(minutes: i32);
    fn SetMainCallback2(callback: MainCallback);
    fn CB2_ReturnToFieldContinueScriptPlayMapMusic();
    fn CB2_StartWallClock();
}

unsafe fn init_time_based_events() {
    let _ = unsafe { FlagSet(FLAG_SYS_CLOCK_SET) };
    unsafe { RtcCalcLocalTime() };
    let local_time = unsafe { addr_of!(gLocalTime).read() };
    let save = unsafe { gSaveBlock2Ptr };
    unsafe { addr_of_mut!((*save).last_berry_tree_update).write(local_time) };
    let _ = unsafe { VarSet(VAR_DAYS, local_time.days as u16) };
}

unsafe fn update_per_day(local_time: *mut Time) {
    let days = unsafe { GetVarPointer(VAR_DAYS) };
    let previous = unsafe { days.read() };
    let current = unsafe { addr_of!((*local_time).days).read() as u16 };
    if previous == current || previous > current {
        return;
    }

    let elapsed = current - previous;
    unsafe { ClearDailyFlags() };
    unsafe { UpdateDewfordTrendPerDay(elapsed) };
    unsafe { UpdateTVShowsPerDay(elapsed) };
    unsafe { UpdateWeatherPerDay(elapsed) };
    unsafe { UpdatePartyPokerusTime(elapsed) };
    unsafe { UpdateMirageRnd(elapsed) };
    unsafe { UpdateBirchState(elapsed) };
    unsafe { UpdateFrontierManiac(elapsed) };
    unsafe { UpdateFrontierGambler(elapsed) };
    unsafe { SetShoalItemFlag(elapsed) };
    unsafe { SetRandomLotteryNumber(elapsed) };
    unsafe { days.write(current) };
}

const fn total_minutes(time: Time) -> i32 {
    24 * 60 * time.days as i32 + 60 * time.hours as i32 + time.minutes as i32
}

unsafe fn update_per_minute(local_time: *mut Time) {
    let save = unsafe { gSaveBlock2Ptr };
    let last_update = unsafe { addr_of_mut!((*save).last_berry_tree_update) };
    let mut difference = Time {
        days: 0,
        hours: 0,
        minutes: 0,
        seconds: 0,
    };
    unsafe { CalcTimeDifference(&raw mut difference, last_update, local_time) };
    let minutes = total_minutes(difference);
    if minutes > 0 {
        unsafe { BerryTreeTimeUpdate(minutes) };
        let current = unsafe { local_time.read() };
        unsafe { last_update.write(current) };
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn DoTimeBasedEvents() {
    if unsafe { FlagGet(FLAG_SYS_CLOCK_SET) } != 0 && unsafe { InPokemonCenter() } == 0 {
        unsafe { RtcCalcLocalTime() };
        let local_time = &raw mut gLocalTime;
        unsafe { update_per_day(local_time) };
        unsafe { update_per_minute(local_time) };
    }
}

unsafe extern "C" fn return_from_start_wall_clock() {
    unsafe { init_time_based_events() };
    unsafe { SetMainCallback2(CB2_ReturnToFieldContinueScriptPlayMapMusic) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn StartWallClock() {
    unsafe { SetMainCallback2(CB2_StartWallClock) };
    let saved_callback = unsafe {
        (&raw mut gMain)
            .add(MAIN_SAVED_CALLBACK_OFFSET)
            .cast::<MainCallback>()
    };
    unsafe { saved_callback.write(return_from_start_wall_clock) };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn layouts_match_the_arm_c_types() {
        assert_eq!(core::mem::size_of::<Time>(), 8);
        assert_eq!(core::mem::offset_of!(Time, hours), 2);
        assert_eq!(
            core::mem::offset_of!(SaveBlock2ClockView, last_berry_tree_update),
            0xa0
        );
    }

    #[test]
    fn elapsed_minutes_match_the_c_expression() {
        let difference = Time {
            days: 2,
            hours: 3,
            minutes: 4,
            seconds: 59,
        };
        assert_eq!(total_minutes(difference), 3064);
    }
}
