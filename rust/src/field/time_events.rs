//! Things that change with the days (was src/time_events.c): Mirage Island,
//! the tide in Shoal Cave and Professor Birch's whereabouts.
//!
//! Mirage Island shows when the low half of a party Pokémon's personality
//! equals the high half of a 32-bit random number, which the save keeps in
//! two vars and advances once a day.

use crate::event_data::{flag_clear, flag_set, var_get, var_set};
use crate::party::player_party;
use crate::random::random;
use crate::rtc::{RtcCalcLocalTime, gLocalTime};
use crate::task::{create_task, destroy_task};

const VAR_MIRAGE_RND_H: u16 = 0x4024;
const VAR_MIRAGE_RND_L: u16 = 0x4025;
const VAR_BIRCH_STATE: u16 = 0x4049;
const FLAG_SYS_SHOAL_TIDE: u16 = 0x89a;
/// Birch's schedule repeats every week.
const BIRCH_STATES: u16 = 7;

/// `GetLastUsedWarpMapType` with this module's view of its types.
#[inline]
unsafe fn GetLastUsedWarpMapType() -> u8 {
    unsafe { crate::overworld::GetLastUsedWarpMapType() }
}
/// `IsMapTypeOutdoors` with this module's view of its types.
#[inline]
unsafe fn IsMapTypeOutdoors(a0: u8) -> u8 {
    unsafe { crate::overworld::IsMapTypeOutdoors(a0) }
}
/// `IsWeatherChangeComplete` with this module's view of its types.
#[inline]
unsafe fn IsWeatherChangeComplete() -> u8 {
    unsafe { crate::field_weather::IsWeatherChangeComplete() }
}
/// `ScriptContext_Enable` with this module's view of its types.
#[inline]
unsafe fn ScriptContext_Enable() {
    unsafe {
        crate::script::ScriptContext_Enable();
    }
}

fn mirage_random() -> u32 {
    u32::from(var_get(VAR_MIRAGE_RND_H)) << 16 | u32::from(var_get(VAR_MIRAGE_RND_L))
}

fn set_mirage_random(value: u32) {
    var_set(VAR_MIRAGE_RND_H, (value >> 16) as u16);
    var_set(VAR_MIRAGE_RND_L, value as u16);
}

/// One day's step of the Mirage Island number (C's `ISO_RANDOMIZE2`).
const fn advance_mirage_random(value: u32) -> u32 {
    value.wrapping_mul(1_103_515_245).wrapping_add(12_345)
}

/// Whether the tide in Shoal Cave is high at `hour`.
const fn is_high_tide_hour(hour: i8) -> bool {
    matches!(hour, 0..=2 | 9..=14 | 21..=23)
}

pub fn init_mirage_random() {
    let high = u32::from(random());
    set_mirage_random(high << 16 | u32::from(random()));
}

pub fn update_mirage_random(days: u16) {
    let value = (0..days).fold(mirage_random(), |value, _| advance_mirage_random(value));
    set_mirage_random(value);
}

pub fn is_mirage_island_present() -> bool {
    let target = (mirage_random() >> 16) as u16;
    // SAFETY: the party isn't borrowed elsewhere; the borrow ends here.
    let party = unsafe { player_party() };
    party
        .0
        .iter_mut()
        .any(|mon| mon.r#box.species() != 0 && mon.r#box.personality as u16 == target)
}

pub fn update_shoal_tide_flag() {
    // SAFETY: plain C functions without preconditions.
    if unsafe { IsMapTypeOutdoors(GetLastUsedWarpMapType()) } == 0 {
        return;
    }
    // SAFETY: as above; gLocalTime is only written by the RTC code.
    let hour = unsafe {
        RtcCalcLocalTime();
        (*(&raw const gLocalTime)).hours
    };
    if is_high_tide_hour(hour) {
        flag_set(FLAG_SYS_SHOAL_TIDE);
    } else {
        flag_clear(FLAG_SYS_SHOAL_TIDE);
    }
}

fn task_wait_weather(task_id: u8) {
    // SAFETY: plain C functions without preconditions.
    if unsafe { IsWeatherChangeComplete() } != 0 {
        unsafe { ScriptContext_Enable() };
        destroy_task(task_id);
    }
}

/// Pauses the running script until the weather has finished changing.
pub fn wait_weather() {
    create_task(task_wait_weather, 80);
}

pub fn init_birch_state() {
    var_set(VAR_BIRCH_STATE, 0);
}

pub fn update_birch_state(days: u16) {
    let state = var_get(VAR_BIRCH_STATE).wrapping_add(days) % BIRCH_STATES;
    var_set(VAR_BIRCH_STATE, state);
}

// ------------------------------------------------------------------ C names

#[unsafe(no_mangle)]
pub fn InitMirageRnd() {
    init_mirage_random();
}

#[unsafe(no_mangle)]
pub fn UpdateMirageRnd(days: u16) {
    update_mirage_random(days);
}

#[unsafe(no_mangle)]
pub fn IsMirageIslandPresent() -> u8 {
    is_mirage_island_present().into()
}

#[unsafe(no_mangle)]
pub fn UpdateShoalTideFlag() {
    update_shoal_tide_flag();
}

#[unsafe(no_mangle)]
pub fn WaitWeather() {
    wait_weather();
}

#[unsafe(no_mangle)]
pub fn InitBirchState() {
    init_birch_state();
}

#[unsafe(no_mangle)]
pub fn UpdateBirchState(days: u16) {
    update_birch_state(days);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mirage_rng_uses_the_second_iso_increment() {
        assert_eq!(advance_mirage_random(0), 12_345);
        assert_eq!(advance_mirage_random(12_345), 3_554_416_254);
    }

    #[test]
    fn shoal_tide_schedule_matches_all_24_hours() {
        let expected = [
            true, true, true, false, false, false, false, false, false, true, true, true, true,
            true, true, false, false, false, false, false, false, true, true, true,
        ];
        for (hour, expected_high) in expected.into_iter().enumerate() {
            assert_eq!(is_high_tide_hour(hour as i8), expected_high);
        }
    }
}
