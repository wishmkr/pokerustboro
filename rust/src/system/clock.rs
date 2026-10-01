//! Events driven by the real-time clock (was src/clock.c): once per new day
//! the daily flags reset and the day counters move on; berry trees grow
//! with the minutes passed.

use crate::agb_main::main;
use crate::event_data::{clear_daily_flags, flag_get, flag_set, var_get, var_set};
use crate::lottery_corner::set_random_lottery_number;
use crate::rtc::{CalcTimeDifference, RtcCalcLocalTime, gLocalTime};
use crate::save_blocks::save_block2;
use crate::time_events::{update_birch_state, update_mirage_random};
use crate::types::Time;

const FLAG_SYS_CLOCK_SET: u16 = 0x895;
const VAR_DAYS: u16 = 0x4040;

/// `InPokemonCenter` with this module's view of its types.
#[inline]
unsafe fn InPokemonCenter() -> u8 {
    unsafe { crate::field_specials::InPokemonCenter() }
}
/// `UpdateDewfordTrendPerDay` with this module's view of its types.
#[inline]
unsafe fn UpdateDewfordTrendPerDay(a0: u16) {
    unsafe {
        crate::dewford_trend::UpdateDewfordTrendPerDay(a0);
    }
}
/// `UpdateTVShowsPerDay` with this module's view of its types.
#[inline]
unsafe fn UpdateTVShowsPerDay(a0: u16) {
    unsafe {
        crate::tv::UpdateTVShowsPerDay(a0);
    }
}
/// `UpdateWeatherPerDay` with this module's view of its types.
#[inline]
unsafe fn UpdateWeatherPerDay(a0: u16) {
    unsafe {
        crate::field_weather_effect::UpdateWeatherPerDay(a0);
    }
}
/// `UpdatePartyPokerusTime` with this module's view of its types.
#[inline]
unsafe fn UpdatePartyPokerusTime(a0: u16) {
    unsafe {
        crate::pokemon::UpdatePartyPokerusTime(a0);
    }
}
/// `UpdateFrontierManiac` with this module's view of its types.
#[inline]
unsafe fn UpdateFrontierManiac(a0: u16) {
    unsafe {
        crate::field_specials::UpdateFrontierManiac(a0);
    }
}
/// `UpdateFrontierGambler` with this module's view of its types.
#[inline]
unsafe fn UpdateFrontierGambler(a0: u16) {
    unsafe {
        crate::field_specials::UpdateFrontierGambler(a0);
    }
}
/// `SetShoalItemFlag` with this module's view of its types.
#[inline]
unsafe fn SetShoalItemFlag(a0: u16) {
    unsafe {
        crate::field_specials::SetShoalItemFlag(a0);
    }
}
/// `BerryTreeTimeUpdate` with this module's view of its types.
#[inline]
unsafe fn BerryTreeTimeUpdate(a0: i32) {
    unsafe {
        crate::berry::BerryTreeTimeUpdate(a0);
    }
}
/// `SetMainCallback2` with this module's view of its types.
#[inline]
unsafe fn SetMainCallback2(a0: unsafe fn()) {
    unsafe {
        crate::agb_main::SetMainCallback2(core::mem::transmute(a0));
    }
}
/// `CB2_ReturnToFieldContinueScriptPlayMapMusic` with this module's view of its types.
#[inline]
unsafe fn CB2_ReturnToFieldContinueScriptPlayMapMusic() {
    unsafe {
        crate::overworld::CB2_ReturnToFieldContinueScriptPlayMapMusic();
    }
}
/// `CB2_StartWallClock` with this module's view of its types.
#[inline]
unsafe fn CB2_StartWallClock() {
    unsafe {
        crate::wallclock::CB2_StartWallClock();
    }
}

/// The local time, freshly read from the clock.
fn local_time() -> Time {
    // SAFETY: the RTC code is the only writer of gLocalTime.
    unsafe {
        RtcCalcLocalTime();
        *(&raw const gLocalTime)
    }
}

const fn total_minutes(time: &Time) -> i32 {
    24 * 60 * time.days as i32 + 60 * time.hours as i32 + time.minutes as i32
}

/// The clock has just been set: start counting from now.
fn init_time_based_events() {
    flag_set(FLAG_SYS_CLOCK_SET);
    let now = local_time();
    // SAFETY: the save blocks are set up at boot; the borrow ends here.
    unsafe { save_block2() }.lastBerryTreeUpdate = now;
    var_set(VAR_DAYS, now.days as u16);
}

pub fn do_time_based_events() {
    // SAFETY: a plain query of the current map.
    if flag_get(FLAG_SYS_CLOCK_SET) && unsafe { InPokemonCenter() } == 0 {
        let now = local_time();
        update_per_day(&now);
        update_per_minute(&now);
    }
}

fn update_per_day(now: &Time) {
    let last_day = var_get(VAR_DAYS);
    // C compares the u16 var with the s16 day as ints
    if i32::from(last_day) >= i32::from(now.days) {
        return;
    }
    let days = (now.days as u16).wrapping_sub(last_day);
    clear_daily_flags();
    // SAFETY: the per-day updates of other modules, in C's order.
    unsafe {
        UpdateDewfordTrendPerDay(days);
        UpdateTVShowsPerDay(days);
        UpdateWeatherPerDay(days);
        UpdatePartyPokerusTime(days);
    }
    update_mirage_random(days);
    update_birch_state(days);
    // SAFETY: as above.
    unsafe {
        UpdateFrontierManiac(days);
        UpdateFrontierGambler(days);
        SetShoalItemFlag(days);
    }
    set_random_lottery_number(days);
    var_set(VAR_DAYS, now.days as u16);
}

fn update_per_minute(now: &Time) {
    // SAFETY: the save blocks are set up at boot; the borrow ends here.
    let last_update = unsafe { save_block2() }.lastBerryTreeUpdate;
    let mut difference = Time {
        days: 0,
        hours: 0,
        minutes: 0,
        seconds: 0,
    };
    // SAFETY: three valid times.
    unsafe { CalcTimeDifference(&mut difference, &last_update, now) };
    let minutes = total_minutes(&difference);
    if minutes > 0 {
        // SAFETY: the berry tree code; then as above.
        unsafe {
            BerryTreeTimeUpdate(minutes);
            save_block2().lastBerryTreeUpdate = *now;
        }
    }
}

fn return_from_start_wall_clock() {
    init_time_based_events();
    // SAFETY: a main callback of the field.
    unsafe { SetMainCallback2(CB2_ReturnToFieldContinueScriptPlayMapMusic) };
}

/// Opens the wall clock to set the time, coming back to the field after.
pub fn start_wall_clock() {
    // SAFETY: as above; gMain isn't borrowed elsewhere.
    unsafe {
        SetMainCallback2(CB2_StartWallClock);
        main().savedCallback = Some(return_from_start_wall_clock);
    }
}

// ------------------------------------------------------------------ C names

#[unsafe(no_mangle)]
pub fn DoTimeBasedEvents() {
    do_time_based_events();
}

#[unsafe(no_mangle)]
pub fn StartWallClock() {
    start_wall_clock();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn elapsed_minutes_match_the_c_expression() {
        let difference = Time {
            days: 2,
            hours: 3,
            minutes: 4,
            seconds: 59,
        };
        assert_eq!(total_minutes(&difference), 3064);
    }
}
