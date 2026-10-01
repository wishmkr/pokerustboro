//! After the Champion is beaten (was src/post_battle_event_funcs.c): the
//! Hall of Fame bookkeeping, the Champion Ribbon, and where Continue starts.

use crate::box_mon::{get_box_mon_data, set_box_mon_data};
use crate::consts::*;
use crate::credits::gHasHallOfFameRecords;
use crate::event_data::{flag_get, flag_set};
use crate::party::player_party;
use crate::save_blocks::save_block2;
use crate::types::Pokemon;

const FLAG_SYS_GAME_CLEAR: u16 = 0x864;
const FLAG_SYS_RIBBON_GET: u16 = 0x89b;
const HEAL_LOCATION_BRENDANS_HOUSE_2F: u8 = 1;
const HEAL_LOCATION_MAYS_HOUSE_2F: u8 = 2;
const MON_DATA_CHAMPION_RIBBON: i32 = 67;
/// Spot the Cuties covers a Pokémon with more ribbons than this.
const NUM_CUTIES_RIBBONS: u8 = 4;

/// `HealPlayerParty` with this module's view of its types.
#[inline]
unsafe fn HealPlayerParty() {
    unsafe {
        crate::script_pokemon_util::HealPlayerParty();
    }
}
/// `GetGameStat` with this module's view of its types.
#[inline]
unsafe fn GetGameStat(a0: u8) -> u32 {
    unsafe { crate::overworld::GetGameStat(a0) }
}
/// `SetGameStat` with this module's view of its types.
#[inline]
unsafe fn SetGameStat(a0: u8, a1: u32) {
    unsafe {
        crate::overworld::SetGameStat(a0, a1);
    }
}
/// `IncrementGameStat` with this module's view of its types.
#[inline]
unsafe fn IncrementGameStat(a0: u8) {
    unsafe {
        crate::overworld::IncrementGameStat(a0);
    }
}
/// `SetContinueGameWarpStatus` with this module's view of its types.
#[inline]
unsafe fn SetContinueGameWarpStatus() {
    unsafe {
        crate::load_save::SetContinueGameWarpStatus();
    }
}
/// `SetContinueGameWarpToHealLocation` with this module's view of its types.
#[inline]
unsafe fn SetContinueGameWarpToHealLocation(a0: u8) {
    unsafe {
        crate::overworld::SetContinueGameWarpToHealLocation(a0);
    }
}
/// `GetRibbonCount` with this module's view of its types.
#[inline]
unsafe fn GetRibbonCount(a0: *mut Pokemon) -> u8 {
    unsafe { crate::tv::GetRibbonCount(a0 as _) }
}
/// `TryPutSpotTheCutiesOnAir` with this module's view of its types.
#[inline]
unsafe fn TryPutSpotTheCutiesOnAir(a0: *mut Pokemon, a1: u8) {
    unsafe {
        crate::tv::TryPutSpotTheCutiesOnAir(a0 as _, a1);
    }
}
/// `SetMainCallback2` with this module's view of its types.
#[inline]
unsafe fn SetMainCallback2(a0: unsafe fn()) {
    unsafe {
        crate::agb_main::SetMainCallback2(core::mem::transmute(a0));
    }
}
/// `CB2_DoHallOfFameScreen` with this module's view of its types.
#[inline]
unsafe fn CB2_DoHallOfFameScreen() {
    unsafe {
        crate::hall_of_fame::CB2_DoHallOfFameScreen();
    }
}
/// `CB2_WhiteOut` with this module's view of its types.
#[inline]
unsafe fn CB2_WhiteOut() {
    unsafe {
        crate::overworld::CB2_WhiteOut();
    }
}

/// The play time packed as GAME_STAT_FIRST_HOF_PLAY_TIME keeps it.
fn packed_play_time() -> u32 {
    // SAFETY: the save blocks are set up at boot; the borrow ends here.
    let save = unsafe { save_block2() };
    u32::from(save.playTimeHours) << 16
        | u32::from(save.playTimeMinutes) << 8
        | u32::from(save.playTimeSeconds)
}

/// Gives the Champion Ribbon to every party Pokémon that lacks it. Returns
/// the index and ribbon count of the first one with the most ribbons among
/// those, if any got it.
fn give_champion_ribbons() -> Option<(usize, u8)> {
    let mut best: Option<(usize, u8)> = None;
    // SAFETY: the party isn't borrowed elsewhere; GetRibbonCount only reads
    // the Pokémon it's given.
    for (i, mon) in unsafe { player_party() }.0.iter_mut().enumerate() {
        let b = &mut mon.r#box;
        // SAFETY: the ribbon field is one byte and needs no out pointer.
        let has_ribbon =
            unsafe { get_box_mon_data(b, MON_DATA_CHAMPION_RIBBON, core::ptr::null_mut()) } != 0;
        if b.hasSpecies() == 0 || b.isEgg() != 0 || has_ribbon {
            continue;
        }
        // SAFETY: as above.
        unsafe { set_box_mon_data(b, MON_DATA_CHAMPION_RIBBON, &1u8) };
        let count = unsafe { GetRibbonCount(mon) };
        if best.is_none_or(|(_, most)| count > most) {
            best = Some((i, count));
        }
    }
    best
}

/// Called when the player enters the Hall of Fame.
pub fn game_clear() {
    // SAFETY: C's own steps, in C's order, with nothing borrowed.
    unsafe {
        HealPlayerParty();
        let cleared_before = flag_get(FLAG_SYS_GAME_CLEAR);
        *(&raw mut gHasHallOfFameRecords) = cleared_before.into();
        if !cleared_before {
            flag_set(FLAG_SYS_GAME_CLEAR);
        }
        if GetGameStat(GAME_STAT_FIRST_HOF_PLAY_TIME) == 0 {
            SetGameStat(GAME_STAT_FIRST_HOF_PLAY_TIME, packed_play_time());
        }
        SetContinueGameWarpStatus();
        SetContinueGameWarpToHealLocation(if save_block2().playerGender == MALE {
            HEAL_LOCATION_BRENDANS_HOUSE_2F
        } else {
            HEAL_LOCATION_MAYS_HOUSE_2F
        });
    }

    if let Some((index, ribbons)) = give_champion_ribbons() {
        // SAFETY: as above.
        unsafe { IncrementGameStat(GAME_STAT_RECEIVED_RIBBONS) };
        flag_set(FLAG_SYS_RIBBON_GET);
        if ribbons > NUM_CUTIES_RIBBONS {
            // SAFETY: as above.
            if let Some(mon) = unsafe { player_party() }.0.get_mut(index) {
                unsafe { TryPutSpotTheCutiesOnAir(mon, MON_DATA_CHAMPION_RIBBON as u8) };
            }
        }
    }

    // SAFETY: a main callback.
    unsafe { SetMainCallback2(CB2_DoHallOfFameScreen) };
}

// ------------------------------------------------------------------ C names

#[unsafe(no_mangle)]
pub fn GameClear() -> i32 {
    game_clear();
    0
}

#[unsafe(no_mangle)]
pub fn SetCB2WhiteOut() -> u8 {
    // SAFETY: a main callback.
    unsafe { SetMainCallback2(CB2_WhiteOut) };
    0
}
