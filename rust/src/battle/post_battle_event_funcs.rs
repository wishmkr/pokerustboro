use crate::ffi::{FlagGet, FlagSet};
use core::ffi::{c_int, c_void};
use core::ptr::{addr_of, addr_of_mut};

const FLAG_SYS_GAME_CLEAR: u16 = 0x864;
const FLAG_SYS_RIBBON_GET: u16 = 0x89b;
const GAME_STAT_FIRST_HOF_PLAY_TIME: u8 = 1;
const GAME_STAT_RECEIVED_RIBBONS: u8 = 42;
const HEAL_LOCATION_BRENDANS_HOUSE_2F: u8 = 1;
const HEAL_LOCATION_MAYS_HOUSE_2F: u8 = 2;
const MON_DATA_SANITY_HAS_SPECIES: c_int = 5;
const MON_DATA_SANITY_IS_EGG: c_int = 6;
const MON_DATA_CHAMPION_RIBBON: c_int = 67;
const NUM_CUTIES_RIBBONS: u8 = 4;

type MainCallback = unsafe extern "C" fn();

#[repr(C, align(4))]
struct Pokemon {
    bytes: [u8; 100],
}

#[repr(C)]
struct SaveBlock2HallOfFameView {
    player_name: [u8; 8],
    player_gender: u8,
    special_save_warp_flags: u8,
    trainer_id: [u8; 4],
    play_time_hours: u16,
    play_time_minutes: u8,
    play_time_seconds: u8,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct RibbonCounter {
    party_index: u8,
    count: u8,
}

unsafe extern "C" {
    static mut gPlayerParty: [Pokemon; 6];
    static mut gSaveBlock2Ptr: *mut SaveBlock2HallOfFameView;
    static mut gHasHallOfFameRecords: u8;

    fn HealPlayerParty();
    fn GetGameStat(index: u8) -> u32;
    fn SetGameStat(index: u8, value: u32);
    fn IncrementGameStat(index: u8);
    fn SetContinueGameWarpStatus();
    fn SetContinueGameWarpToHealLocation(location: u8);
    fn GetMonData2(mon: *mut u8, field: c_int) -> u32;
    fn SetMonData(mon: *mut u8, field: c_int, value: *const c_void);
    fn GetRibbonCount(mon: *mut Pokemon) -> u8;
    fn TryPutSpotTheCutiesOnAir(mon: *mut Pokemon, ribbon_field: u8);
    fn SetMainCallback2(callback: MainCallback);
    fn CB2_DoHallOfFameScreen();
    fn CB2_WhiteOut();
}

unsafe fn first_hall_of_fame_time() -> u32 {
    let save = unsafe { gSaveBlock2Ptr };
    (u32::from(unsafe { addr_of!((*save).play_time_hours).read() }) << 16)
        | (u32::from(unsafe { addr_of!((*save).play_time_minutes).read() }) << 8)
        | u32::from(unsafe { addr_of!((*save).play_time_seconds).read() })
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn GameClear() -> c_int {
    unsafe { HealPlayerParty() };

    if unsafe { FlagGet(FLAG_SYS_GAME_CLEAR) } != 0 {
        unsafe { addr_of_mut!(gHasHallOfFameRecords).write(1) };
    } else {
        unsafe { addr_of_mut!(gHasHallOfFameRecords).write(0) };
        let _ = unsafe { FlagSet(FLAG_SYS_GAME_CLEAR) };
    }

    if unsafe { GetGameStat(GAME_STAT_FIRST_HOF_PLAY_TIME) } == 0 {
        unsafe { SetGameStat(GAME_STAT_FIRST_HOF_PLAY_TIME, first_hall_of_fame_time()) };
    }

    unsafe { SetContinueGameWarpStatus() };
    let gender = unsafe { addr_of!((*gSaveBlock2Ptr).player_gender).read() };
    unsafe {
        SetContinueGameWarpToHealLocation(if gender == 0 {
            HEAL_LOCATION_BRENDANS_HOUSE_2F
        } else {
            HEAL_LOCATION_MAYS_HOUSE_2F
        })
    };

    let mut ribbon_get = false;
    let mut ribbon_counts = [RibbonCounter {
        party_index: 0,
        count: 0,
    }; 6];
    let party = (&raw mut gPlayerParty).cast::<Pokemon>();

    let mut index = 0usize;
    while index < 6 {
        let mon = unsafe { party.add(index) };
        ribbon_counts[index].party_index = index as u8;

        if unsafe { GetMonData2(mon.cast(), MON_DATA_SANITY_HAS_SPECIES) } != 0
            && unsafe { GetMonData2(mon.cast(), MON_DATA_SANITY_IS_EGG) } == 0
            && unsafe { GetMonData2(mon.cast(), MON_DATA_CHAMPION_RIBBON) } == 0
        {
            let value = 1u8;
            unsafe {
                SetMonData(
                    mon.cast(),
                    MON_DATA_CHAMPION_RIBBON,
                    (&raw const value).cast(),
                )
            };
            ribbon_counts[index].count = unsafe { GetRibbonCount(mon) };
            ribbon_get = true;
        }
        index += 1;
    }

    if ribbon_get {
        unsafe { IncrementGameStat(GAME_STAT_RECEIVED_RIBBONS) };
        let _ = unsafe { FlagSet(FLAG_SYS_RIBBON_GET) };

        let mut index = 1usize;
        while index < 6 {
            if ribbon_counts[index].count > ribbon_counts[0].count {
                ribbon_counts.swap(0, index);
            }
            index += 1;
        }

        if ribbon_counts[0].count > NUM_CUTIES_RIBBONS {
            let mon = unsafe { party.add(ribbon_counts[0].party_index as usize) };
            unsafe { TryPutSpotTheCutiesOnAir(mon, MON_DATA_CHAMPION_RIBBON as u8) };
        }
    }

    unsafe { SetMainCallback2(CB2_DoHallOfFameScreen) };
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetCB2WhiteOut() -> u8 {
    unsafe { SetMainCallback2(CB2_WhiteOut) };
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn arm_layouts_match_the_c_structures() {
        assert_eq!(core::mem::size_of::<Pokemon>(), 100);
        assert_eq!(
            core::mem::offset_of!(SaveBlock2HallOfFameView, player_gender),
            0x08
        );
        assert_eq!(
            core::mem::offset_of!(SaveBlock2HallOfFameView, play_time_hours),
            0x0e
        );
        assert_eq!(
            core::mem::offset_of!(SaveBlock2HallOfFameView, play_time_minutes),
            0x10
        );
        assert_eq!(
            core::mem::offset_of!(SaveBlock2HallOfFameView, play_time_seconds),
            0x11
        );
        assert_eq!(core::mem::size_of::<RibbonCounter>(), 2);
    }

    #[test]
    fn ribbon_selection_keeps_the_highest_count_first() {
        let mut counts = [
            RibbonCounter {
                party_index: 0,
                count: 2,
            },
            RibbonCounter {
                party_index: 1,
                count: 7,
            },
            RibbonCounter {
                party_index: 2,
                count: 5,
            },
            RibbonCounter {
                party_index: 3,
                count: 9,
            },
            RibbonCounter {
                party_index: 4,
                count: 1,
            },
            RibbonCounter {
                party_index: 5,
                count: 8,
            },
        ];
        let mut index = 1;
        while index < counts.len() {
            if counts[index].count > counts[0].count {
                counts.swap(0, index);
            }
            index += 1;
        }
        assert_eq!(counts[0].party_index, 3);
        assert_eq!(counts[0].count, 9);
    }
}
