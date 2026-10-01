use crate::ffi::{FlagClear, FlagSet, GetVarPointer, VarGet, VarSet};
use core::ffi::c_int;
use core::ptr::addr_of;

const VAR_MIRAGE_RND_H: u16 = 0x4024;
const VAR_MIRAGE_RND_L: u16 = 0x4025;
const VAR_BIRCH_STATE: u16 = 0x4049;
const FLAG_SYS_SHOAL_TIDE: u16 = 0x89a;
const MON_DATA_PERSONALITY: c_int = 0;
const MON_DATA_SPECIES: c_int = 11;

#[repr(C, align(4))]
struct Time {
    days: i16,
    hours: i8,
    minutes: i8,
    seconds: i8,
}

#[repr(C, align(4))]
struct Pokemon {
    bytes: [u8; 100],
}

type TaskFunc = unsafe extern "C" fn(u8);

unsafe extern "C" {
    static mut gLocalTime: Time;
    static mut gPlayerParty: [Pokemon; 6];
    fn Random() -> u16;
    fn GetMonData2(mon: *mut u8, field: c_int) -> u32;
    fn GetLastUsedWarpMapType() -> u8;
    fn IsMapTypeOutdoors(map_type: u8) -> u8;
    fn RtcCalcLocalTime();
    fn IsWeatherChangeComplete() -> u8;
    fn ScriptContext_Enable();
    fn CreateTask(function: TaskFunc, priority: u8) -> u8;
    fn DestroyTask(task_id: u8);
}

unsafe fn get_mirage_random() -> u32 {
    let high = u32::from(unsafe { VarGet(VAR_MIRAGE_RND_H) });
    let low = u32::from(unsafe { VarGet(VAR_MIRAGE_RND_L) });
    (high << 16) | low
}

unsafe fn set_mirage_random(value: u32) {
    let _ = unsafe { VarSet(VAR_MIRAGE_RND_H, (value >> 16) as u16) };
    let _ = unsafe { VarSet(VAR_MIRAGE_RND_L, value as u16) };
}

const fn advance_mirage_random(value: u32) -> u32 {
    value.wrapping_mul(1_103_515_245).wrapping_add(12_345)
}

const fn is_high_tide_hour(hour: i8) -> bool {
    matches!(hour, 0..=2 | 9..=14 | 21..=23)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn InitMirageRnd() {
    let value = (u32::from(unsafe { Random() }) << 16) | u32::from(unsafe { Random() });
    unsafe { set_mirage_random(value) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn UpdateMirageRnd(mut days: u16) {
    let mut value = unsafe { get_mirage_random() };
    while days != 0 {
        value = advance_mirage_random(value);
        days -= 1;
    }
    unsafe { set_mirage_random(value) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsMirageIslandPresent() -> u8 {
    let target = (unsafe { get_mirage_random() } >> 16) as u16;
    let party = (&raw mut gPlayerParty).cast::<Pokemon>();
    let mut index = 0;
    while index < 6 {
        let mon = unsafe { party.add(index) }.cast::<u8>();
        if unsafe { GetMonData2(mon, MON_DATA_SPECIES) } != 0
            && (unsafe { GetMonData2(mon, MON_DATA_PERSONALITY) } as u16) == target
        {
            return 1;
        }
        index += 1;
    }
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn UpdateShoalTideFlag() {
    if unsafe { IsMapTypeOutdoors(GetLastUsedWarpMapType()) } != 0 {
        unsafe { RtcCalcLocalTime() };
        let hour = unsafe { addr_of!(gLocalTime.hours).read() };
        if is_high_tide_hour(hour) {
            let _ = unsafe { FlagSet(FLAG_SYS_SHOAL_TIDE) };
        } else {
            let _ = unsafe { FlagClear(FLAG_SYS_SHOAL_TIDE) };
        }
    }
}

unsafe extern "C" fn task_wait_weather(task_id: u8) {
    if unsafe { IsWeatherChangeComplete() } != 0 {
        unsafe { ScriptContext_Enable() };
        unsafe { DestroyTask(task_id) };
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn WaitWeather() {
    let _ = unsafe { CreateTask(task_wait_weather, 80) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn InitBirchState() {
    unsafe { GetVarPointer(VAR_BIRCH_STATE).write(0) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn UpdateBirchState(days: u16) {
    let state = unsafe { GetVarPointer(VAR_BIRCH_STATE) };
    let updated = unsafe { state.read() }.wrapping_add(days) % 7;
    unsafe { state.write(updated) };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ffi_types_match_the_arm_c_layout() {
        assert_eq!(core::mem::size_of::<Time>(), 8);
        assert_eq!(core::mem::offset_of!(Time, hours), 2);
        assert_eq!(core::mem::size_of::<Pokemon>(), 100);
    }

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
