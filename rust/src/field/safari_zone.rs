//! The Safari Zone: a ball count, a step timer, and up to ten Pokeblock
//! feeders left lying on the ground to attract Pokemon.

use crate::ffi::{
    ConvertIntToDecimalStringN, FlagClear, FlagGet, FlagSet, StringCopy, gSpecialVar_Result,
    gStringVar1, gStringVar2,
};
use core::ffi::c_int;

const NUM_POKEBLOCK_FEEDERS: usize = 10;
const SAFARI_BALLS: u8 = 30;
const SAFARI_STEPS: u16 = 500;
/// A feeder lasts this many steps before it is cleared.
const FEEDER_STEPS: u8 = 100;
/// A feeder attracts Pokemon within this Manhattan distance.
const FEEDER_RANGE: i16 = 5;

const FLAG_SYS_SAFARI_MODE: u16 = 0x88c;
const GAME_STAT_ENTERED_SAFARI_ZONE: u8 = 17;
const B_OUTCOME_CAUGHT: u8 = 7;
const B_OUTCOME_NO_SAFARI_BALLS: u8 = 8;
const STR_CONV_MODE_LEADING_ZEROS: c_int = 2;

/// `struct PokeblockFeeder`: two coordinates, a map, a counter, then the
/// Pokeblock itself on the next word boundary.
const FEEDER_STRIDE: usize = 16;
const FEEDER_X: usize = 0;
const FEEDER_Y: usize = 2;
const FEEDER_MAP_NUM: usize = 4;
const FEEDER_STEP_COUNTER: usize = 5;
const FEEDER_POKEBLOCK: usize = 8;

/// `sizeof(struct Pokeblock)`; `color` is its first byte.
const POKEBLOCK_SIZE: usize = 8;

/// `offsetof(struct SaveBlock1, pokeblocks)` and `location.mapNum`.
const SAVE1_POKEBLOCKS: usize = 0x848;
const SAVE1_LOCATION_MAP_NUM: usize = 5;

/// `offsetof(struct BattleResults, pokeblockThrows)`
const BATTLE_RESULTS_POKEBLOCK_THROWS: usize = 31;

type MainCallback = unsafe extern "C" fn();

#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gNumSafariBalls: u8 = 0;

#[unsafe(link_section = "ewram_data")]
static mut STEP_COUNTER: u16 = 0;

#[unsafe(link_section = "ewram_data")]
static mut CAUGHT_MONS: u8 = 0;

#[unsafe(link_section = "ewram_data")]
static mut PKBLK_USES: u8 = 0;

#[unsafe(link_section = "ewram_data")]
static mut POKEBLOCK_FEEDERS: crate::ffi::Align4<[[u8; FEEDER_STRIDE]; NUM_POKEBLOCK_FEEDERS]> =
    crate::ffi::Align4([[0; FEEDER_STRIDE]; NUM_POKEBLOCK_FEEDERS]);

unsafe extern "C" {
    static mut gSaveBlock1Ptr: *mut u8;
    static mut gBattleResults: u8;
    static mut gBattleOutcome: u8;
    static mut gFieldCallback: Option<MainCallback>;
    static gPokeblockNames: *const u8;

    static SafariZone_EventScript_TimesUp: u8;
    static SafariZone_EventScript_RetirePrompt: u8;
    static SafariZone_EventScript_OutOfBallsMidBattle: u8;
    static SafariZone_EventScript_OutOfBalls: u8;

    fn IncrementGameStat(index: u8);
    fn TryPutSafariFanClubOnAir(mons_caught: u8, pokeblocks_used: u8);
    fn ScriptContext_SetupScript(script: *const u8);
    fn ScriptContext_Stop();
    fn RunScriptImmediately(script: *const u8);
    fn WarpIntoMap();
    fn SetMainCallback2(callback: MainCallback);
    fn CB2_ReturnToField();
    fn CB2_LoadMap();
    fn CB2_ReturnToFieldContinueScriptPlayMapMusic();
    fn FieldCB_ReturnToFieldNoScriptCheckMusic();
    fn GetXYCoordsOneStepInFrontOfPlayer(x: *mut i16, y: *mut i16);
    fn PlayerGetDestCoords(x: *mut i16, y: *mut i16);
}

#[inline]
unsafe fn feeder(index: usize) -> *mut u8 {
    unsafe {
        (&raw mut POKEBLOCK_FEEDERS)
            .cast::<u8>()
            .add(index * FEEDER_STRIDE)
    }
}

#[inline]
unsafe fn feeder_i16(index: usize, offset: usize) -> i16 {
    unsafe { feeder(index).add(offset).cast::<i16>().read_volatile() }
}

#[inline]
unsafe fn feeder_u8(index: usize, offset: usize) -> u8 {
    unsafe { feeder(index).add(offset).read_volatile() }
}

#[inline]
unsafe fn current_map_num() -> u8 {
    unsafe { gSaveBlock1Ptr.add(SAVE1_LOCATION_MAP_NUM).read_volatile() }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetSafariZoneFlag() -> u32 {
    u32::from(unsafe { FlagGet(FLAG_SYS_SAFARI_MODE) })
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetSafariZoneFlag() {
    unsafe { FlagSet(FLAG_SYS_SAFARI_MODE) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ResetSafariZoneFlag() {
    unsafe { FlagClear(FLAG_SYS_SAFARI_MODE) };
}

unsafe fn clear_all_pokeblock_feeders() {
    unsafe {
        core::ptr::write_bytes(
            (&raw mut POKEBLOCK_FEEDERS).cast::<u8>(),
            0,
            FEEDER_STRIDE * NUM_POKEBLOCK_FEEDERS,
        )
    };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn EnterSafariMode() {
    unsafe { IncrementGameStat(GAME_STAT_ENTERED_SAFARI_ZONE) };
    unsafe { SetSafariZoneFlag() };
    unsafe { clear_all_pokeblock_feeders() };
    unsafe { (&raw mut gNumSafariBalls).write_volatile(SAFARI_BALLS) };
    unsafe { (&raw mut STEP_COUNTER).write_volatile(SAFARI_STEPS) };
    unsafe { (&raw mut CAUGHT_MONS).write_volatile(0) };
    unsafe { (&raw mut PKBLK_USES).write_volatile(0) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ExitSafariMode() {
    unsafe {
        TryPutSafariFanClubOnAir(
            (&raw const CAUGHT_MONS).read_volatile(),
            (&raw const PKBLK_USES).read_volatile(),
        )
    };
    unsafe { ResetSafariZoneFlag() };
    unsafe { clear_all_pokeblock_feeders() };
    unsafe { (&raw mut gNumSafariBalls).write_volatile(0) };
    unsafe { (&raw mut STEP_COUNTER).write_volatile(0) };
}

unsafe fn decrement_feeder_step_counters() {
    for i in 0..NUM_POKEBLOCK_FEEDERS {
        let counter = unsafe { feeder_u8(i, FEEDER_STEP_COUNTER) };
        if counter == 0 {
            continue;
        }
        let counter = counter - 1;
        unsafe { feeder(i).add(FEEDER_STEP_COUNTER).write_volatile(counter) };
        if counter == 0 {
            unsafe { core::ptr::write_bytes(feeder(i), 0, FEEDER_STRIDE) };
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn SafariZoneTakeStep() -> u8 {
    if unsafe { GetSafariZoneFlag() } == 0 {
        return 0;
    }

    unsafe { decrement_feeder_step_counters() };
    let steps = unsafe { (&raw const STEP_COUNTER).read_volatile() }.wrapping_sub(1);
    unsafe { (&raw mut STEP_COUNTER).write_volatile(steps) };

    if steps == 0 {
        unsafe { ScriptContext_SetupScript(&raw const SafariZone_EventScript_TimesUp) };
        return 1;
    }
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn SafariZoneRetirePrompt() {
    unsafe { ScriptContext_SetupScript(&raw const SafariZone_EventScript_RetirePrompt) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn CB2_EndSafariBattle() {
    let throws = unsafe {
        (&raw const gBattleResults)
            .add(BATTLE_RESULTS_POKEBLOCK_THROWS)
            .read_volatile()
    };
    let uses = unsafe { (&raw const PKBLK_USES).read_volatile() }.wrapping_add(throws);
    unsafe { (&raw mut PKBLK_USES).write_volatile(uses) };

    let outcome = unsafe { (&raw const gBattleOutcome).read_volatile() };
    if outcome == B_OUTCOME_CAUGHT {
        let caught = unsafe { (&raw const CAUGHT_MONS).read_volatile() }.wrapping_add(1);
        unsafe { (&raw mut CAUGHT_MONS).write_volatile(caught) };
    }

    if unsafe { (&raw const gNumSafariBalls).read_volatile() } != 0 {
        unsafe { SetMainCallback2(CB2_ReturnToField) };
    } else if outcome == B_OUTCOME_NO_SAFARI_BALLS {
        // Ran out mid-battle: warp out and tell the player on arrival.
        unsafe { RunScriptImmediately(&raw const SafariZone_EventScript_OutOfBallsMidBattle) };
        unsafe { WarpIntoMap() };
        unsafe { (&raw mut gFieldCallback).write(Some(FieldCB_ReturnToFieldNoScriptCheckMusic)) };
        unsafe { SetMainCallback2(CB2_LoadMap) };
    } else if outcome == B_OUTCOME_CAUGHT {
        unsafe { ScriptContext_SetupScript(&raw const SafariZone_EventScript_OutOfBalls) };
        unsafe { ScriptContext_Stop() };
        unsafe { SetMainCallback2(CB2_ReturnToFieldContinueScriptPlayMapMusic) };
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetPokeblockFeederInFront() {
    let mut x = 0i16;
    let mut y = 0i16;
    unsafe { GetXYCoordsOneStepInFrontOfPlayer(&raw mut x, &raw mut y) };

    for i in 0..NUM_POKEBLOCK_FEEDERS {
        if unsafe { current_map_num() } == unsafe { feeder_u8(i, FEEDER_MAP_NUM) }
            && unsafe { feeder_i16(i, FEEDER_X) } == x
            && unsafe { feeder_i16(i, FEEDER_Y) } == y
        {
            unsafe { (&raw mut gSpecialVar_Result).write_volatile(i as u16) };
            let color = unsafe { feeder_u8(i, FEEDER_POKEBLOCK) } as usize;
            let name = unsafe {
                (&raw const gPokeblockNames)
                    .cast::<*const u8>()
                    .add(color)
                    .read()
            };
            let _ = unsafe { StringCopy((&raw mut gStringVar1).cast::<u8>(), name) };
            return;
        }
    }

    unsafe { (&raw mut gSpecialVar_Result).write_volatile(0xffff) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetPokeblockFeederWithinRange() {
    let mut x = 0i16;
    let mut y = 0i16;
    unsafe { PlayerGetDestCoords(&raw mut x, &raw mut y) };

    for i in 0..NUM_POKEBLOCK_FEEDERS {
        if unsafe { current_map_num() } != unsafe { feeder_u8(i, FEEDER_MAP_NUM) } {
            continue;
        }

        // The original narrows x and y in place, so each miss leaves them
        // relative to the feeder it just tested. That is preserved here.
        x -= unsafe { feeder_i16(i, FEEDER_X) };
        y -= unsafe { feeder_i16(i, FEEDER_Y) };
        if x < 0 {
            x = -x;
        }
        if y < 0 {
            y = -y;
        }
        if x + y <= FEEDER_RANGE {
            unsafe { (&raw mut gSpecialVar_Result).write_volatile(i as u16) };
            return;
        }
    }

    unsafe { (&raw mut gSpecialVar_Result).write_volatile(0xffff) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn SafariZoneGetPokeblockInFront() -> *mut u8 {
    unsafe { GetPokeblockFeederInFront() };
    let result = unsafe { (&raw const gSpecialVar_Result).read_volatile() };
    if result == 0xffff {
        core::ptr::null_mut()
    } else {
        unsafe { feeder(result as usize).add(FEEDER_POKEBLOCK) }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn SafariZoneGetActivePokeblock() -> *mut u8 {
    unsafe { GetPokeblockFeederWithinRange() };
    let result = unsafe { (&raw const gSpecialVar_Result).read_volatile() };
    if result == 0xffff {
        core::ptr::null_mut()
    } else {
        unsafe { feeder(result as usize).add(FEEDER_POKEBLOCK) }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn SafariZoneActivatePokeblockFeeder(pkbl_id: u8) {
    for i in 0..NUM_POKEBLOCK_FEEDERS {
        // A free slot is one that is entirely zeroed.
        if unsafe { feeder_u8(i, FEEDER_MAP_NUM) } != 0
            || unsafe { feeder_i16(i, FEEDER_X) } != 0
            || unsafe { feeder_i16(i, FEEDER_Y) } != 0
        {
            continue;
        }

        let mut x = 0i16;
        let mut y = 0i16;
        unsafe { GetXYCoordsOneStepInFrontOfPlayer(&raw mut x, &raw mut y) };

        let slot = unsafe { feeder(i) };
        unsafe { slot.add(FEEDER_MAP_NUM).write_volatile(current_map_num()) };
        unsafe {
            core::ptr::copy_nonoverlapping(
                gSaveBlock1Ptr.add(SAVE1_POKEBLOCKS + pkbl_id as usize * POKEBLOCK_SIZE),
                slot.add(FEEDER_POKEBLOCK),
                POKEBLOCK_SIZE,
            )
        };
        unsafe { slot.add(FEEDER_STEP_COUNTER).write_volatile(FEEDER_STEPS) };
        unsafe { slot.add(FEEDER_X).cast::<i16>().write_volatile(x) };
        unsafe { slot.add(FEEDER_Y).cast::<i16>().write_volatile(y) };
        break;
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetInFrontFeederPokeblockAndSteps() -> u8 {
    unsafe { GetPokeblockFeederInFront() };
    let result = unsafe { (&raw const gSpecialVar_Result).read_volatile() };
    if result == 0xffff {
        return 0;
    }

    let steps = unsafe { feeder_u8(result as usize, FEEDER_STEP_COUNTER) };
    let _ = unsafe {
        ConvertIntToDecimalStringN(
            (&raw mut gStringVar2).cast::<u8>(),
            i32::from(steps),
            STR_CONV_MODE_LEADING_ZEROS,
            3,
        )
    };
    1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn feeder_field_offsets_match_the_arm_structure() {
        assert_eq!(FEEDER_X, 0);
        assert_eq!(FEEDER_Y, 2);
        assert_eq!(FEEDER_MAP_NUM, 4);
        assert_eq!(FEEDER_STEP_COUNTER, 5);
        // The Pokeblock is word aligned, so there are two bytes of padding.
        assert_eq!(FEEDER_POKEBLOCK, 8);
        assert_eq!(FEEDER_POKEBLOCK + POKEBLOCK_SIZE, FEEDER_STRIDE);
    }

    #[test]
    fn the_safari_run_is_thirty_balls_and_five_hundred_steps() {
        assert_eq!(SAFARI_BALLS, 30);
        assert_eq!(SAFARI_STEPS, 500);
        assert_eq!(FEEDER_STEPS, 100);
    }

    #[test]
    fn range_is_measured_as_a_manhattan_distance() {
        let in_range = |dx: i16, dy: i16| dx.abs() + dy.abs() <= FEEDER_RANGE;
        assert!(in_range(5, 0));
        assert!(in_range(3, 2));
        assert!(!in_range(3, 3));
        assert!(in_range(-2, -3));
    }
}
