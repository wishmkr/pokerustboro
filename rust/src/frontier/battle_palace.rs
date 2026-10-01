//! Battle Palace challenge specials, dispatched through
//! `CallBattlePalaceFunction` with the function id in `gSpecialVar_0x8004`.

use crate::Random;
use crate::battle_setup::gTrainerBattleOpponent_A;
use crate::ffi::{
    VarGet, VarSet, gSpecialVar_0x8004, gSpecialVar_0x8005, gSpecialVar_0x8006, gSpecialVar_Result,
    gStringVar1,
};
use crate::load_save::{gSaveBlock1Ptr, gSaveBlock2Ptr};

const VAR_FRONTIER_BATTLE_MODE: u16 = 0x40ce;
const VAR_TEMP_CHALLENGE_STATUS: u16 = 0x4000;
const WARP_ID_NONE: i8 = -1;
const MAX_STREAK: u16 = 9999;
const FRONTIER_TRAINERS_COUNT: u16 = 300;
const FRONTIER_TRAINER_SIZE: usize = 0x34;
const FRONTIER_TRAINER_SPEECH_BEFORE: usize = 0x0c;

const PALACE_DATA_PRIZE: u16 = 0;
const PALACE_DATA_WIN_STREAK: u16 = 1;
const PALACE_DATA_WIN_STREAK_ACTIVE: u16 = 2;

// SaveBlock2 frontier fields.
const SB2_CHALLENGE_STATUS: usize = 0xca8;
/// `lvlMode:2`, `challengePaused:1`, `disableRecordBattle:1`.
const SB2_FRONTIER_FLAGS: usize = 0xca9;
const LVL_MODE_MASK: u8 = 0x03;
const CHALLENGE_PAUSED_BIT: u8 = 0x04;
const DISABLE_RECORD_BATTLE_BIT: u8 = 0x08;
const SB2_CUR_CHALLENGE_BATTLE_NUM: usize = 0xcb2;
const SB2_WIN_STREAK_ACTIVE_FLAGS: usize = 0xcdc;
const SB2_PALACE_PRIZE: usize = 0xdc6;
const SB2_PALACE_WIN_STREAKS: usize = 0xdc8;
const SB2_PALACE_RECORD_WIN_STREAKS: usize = 0xdd0;
const SB1_LOCATION_MAP_GROUP: usize = 4;
const SB1_LOCATION_MAP_NUM: usize = 5;

static EARLY_PRIZES: [u16; 6] = [0x3f, 0x40, 0x41, 0x43, 0x42, 0x46];
static LATE_PRIZES: [u16; 9] = [0xb3, 0xb4, 0xb7, 0xc8, 0xb9, 0xbb, 0xc4, 0xc6, 0xba];

/// `sWinStreakFlags[battleMode][lvlMode]`, flattened.
static WIN_STREAK_FLAGS: [u32; 4] = [0x10, 0x20, 0x40_0000, 0x80_0000];

/// `SetDynamicWarp` with this module's view of its types.
#[inline]
unsafe fn SetDynamicWarp(a0: i32, a1: i8, a2: i8, a3: i8) {
    unsafe {
        crate::overworld::SetDynamicWarp(a0, a1, a2, a3);
    }
}
/// `SetBattleFacilityTrainerGfxId` with this module's view of its types.
#[inline]
unsafe fn SetBattleFacilityTrainerGfxId(a0: u16, a1: u8) {
    unsafe {
        crate::battle_tower::SetBattleFacilityTrainerGfxId(a0, a1);
    }
}
/// `FrontierSpeechToString` with this module's view of its types.
#[inline]
unsafe fn FrontierSpeechToString(a0: *const u16) {
    unsafe {
        crate::battle_tower::FrontierSpeechToString(a0 as _);
    }
}
/// `SaveGameFrontier` with this module's view of its types.
#[inline]
unsafe fn SaveGameFrontier() {
    unsafe {
        crate::frontier_util::SaveGameFrontier();
    }
}
/// `AddBagItem` with this module's view of its types.
#[inline]
unsafe fn AddBagItem(a0: u16, a1: u16) -> u8 {
    crate::item::AddBagItem(a0, a1)
}
/// `CopyItemName` with this module's view of its types.
#[inline]
unsafe fn CopyItemName(a0: u16, a1: *mut u8) {
    unsafe {
        crate::item::CopyItemName(a0, a1 as _);
    }
}

#[inline]
unsafe fn sb2() -> *mut u8 {
    unsafe { (&raw const gSaveBlock2Ptr).read().cast::<u8>() }
}

#[inline]
unsafe fn sb2_u16(offset: usize) -> *mut u16 {
    unsafe { sb2().add(offset).cast() }
}

#[inline]
unsafe fn lvl_mode() -> usize {
    let flags = unsafe { sb2().add(SB2_FRONTIER_FLAGS).read() };
    usize::from(flags & LVL_MODE_MASK)
}

#[inline]
unsafe fn battle_mode() -> usize {
    usize::from(unsafe { VarGet(VAR_FRONTIER_BATTLE_MODE) })
}

/// `palaceWinStreaks[battleMode][lvlMode]` (or the record array).
#[inline]
unsafe fn streak(base: usize, battle_mode: usize, lvl_mode: usize) -> *mut u16 {
    unsafe { sb2_u16(base + (battle_mode * 2 + lvl_mode) * 2) }
}

#[inline]
fn streak_flag(battle_mode: usize, lvl_mode: usize) -> u32 {
    WIN_STREAK_FLAGS[(battle_mode * 2 + lvl_mode) % 4]
}

#[inline]
unsafe fn win_streak_active_flags() -> *mut u32 {
    unsafe { sb2().add(SB2_WIN_STREAK_ACTIVE_FLAGS).cast() }
}

#[inline]
unsafe fn set_result(value: u16) {
    unsafe { (&raw mut gSpecialVar_Result).write(value) };
}

#[unsafe(no_mangle)]
pub unsafe fn CallBattlePalaceFunction() {
    match unsafe { (&raw const gSpecialVar_0x8004).read() } {
        0 => unsafe { init_palace_challenge() },
        1 => unsafe { get_palace_data() },
        2 => unsafe { set_palace_data() },
        3 => unsafe { get_palace_comment_id() },
        4 => unsafe { set_palace_opponent() },
        5 => unsafe { buffer_opponent_intro_speech() },
        6 => unsafe { increment_palace_streak() },
        7 => unsafe { save_palace_challenge() },
        8 => unsafe { set_random_palace_prize() },
        9 => unsafe { give_palace_prize() },
        _ => {}
    }
}

unsafe fn init_palace_challenge() {
    let lvl_mode = unsafe { lvl_mode() };
    let battle_mode = unsafe { battle_mode() };

    unsafe { sb2().add(SB2_CHALLENGE_STATUS).write(0) };
    unsafe { sb2_u16(SB2_CUR_CHALLENGE_BATTLE_NUM).write(0) };
    let flags = unsafe { sb2().add(SB2_FRONTIER_FLAGS) };
    unsafe { flags.write(flags.read() & !(CHALLENGE_PAUSED_BIT | DISABLE_RECORD_BATTLE_BIT)) };
    let active = unsafe { win_streak_active_flags().read() };
    if active & streak_flag(battle_mode, lvl_mode) == 0 {
        unsafe { streak(SB2_PALACE_WIN_STREAKS, battle_mode, lvl_mode).write(0) };
    }

    let sb1 = unsafe { (&raw const gSaveBlock1Ptr).read().cast::<u8>() };
    let map_group = unsafe { sb1.add(SB1_LOCATION_MAP_GROUP).read() } as i8;
    let map_num = unsafe { sb1.add(SB1_LOCATION_MAP_NUM).read() } as i8;
    unsafe { SetDynamicWarp(0, map_group, map_num, WARP_ID_NONE) };
    unsafe { (&raw mut gTrainerBattleOpponent_A).write(0) };
}

unsafe fn get_palace_data() {
    let lvl_mode = unsafe { lvl_mode() };
    let battle_mode = unsafe { battle_mode() };
    match unsafe { (&raw const gSpecialVar_0x8005).read() } {
        PALACE_DATA_PRIZE => unsafe { set_result(sb2_u16(SB2_PALACE_PRIZE).read()) },
        PALACE_DATA_WIN_STREAK => unsafe {
            set_result(streak(SB2_PALACE_WIN_STREAKS, battle_mode, lvl_mode).read())
        },
        PALACE_DATA_WIN_STREAK_ACTIVE => {
            let active = unsafe { win_streak_active_flags().read() };
            unsafe { set_result(u16::from(active & streak_flag(battle_mode, lvl_mode) != 0)) };
        }
        _ => {}
    }
}

unsafe fn set_palace_data() {
    let lvl_mode = unsafe { lvl_mode() };
    let battle_mode = unsafe { battle_mode() };
    let value = unsafe { (&raw const gSpecialVar_0x8006).read() };
    match unsafe { (&raw const gSpecialVar_0x8005).read() } {
        PALACE_DATA_PRIZE => unsafe { sb2_u16(SB2_PALACE_PRIZE).write(value) },
        PALACE_DATA_WIN_STREAK => unsafe {
            streak(SB2_PALACE_WIN_STREAKS, battle_mode, lvl_mode).write(value)
        },
        PALACE_DATA_WIN_STREAK_ACTIVE => {
            let active = unsafe { win_streak_active_flags() };
            let flag = streak_flag(battle_mode, lvl_mode);
            if value != 0 {
                unsafe { active.write(active.read() | flag) };
            } else {
                unsafe { active.write(active.read() & !flag) };
            }
        }
        _ => {}
    }
}

unsafe fn get_palace_comment_id() {
    let battle_mode = unsafe { battle_mode() };
    let lvl_mode = unsafe { lvl_mode() };
    let current = unsafe { streak(SB2_PALACE_WIN_STREAKS, battle_mode, lvl_mode).read() };
    let comment = if current < 50 {
        let roll = Random();
        roll % 3
    } else if current < 99 {
        3
    } else {
        4
    };
    unsafe { set_result(comment) };
}

unsafe fn set_palace_opponent() {
    let roll = u32::from(Random()) % 255;
    let opponent = (5 * roll / 64) as u16;
    unsafe { (&raw mut gTrainerBattleOpponent_A).write(opponent) };
    unsafe { SetBattleFacilityTrainerGfxId(opponent, 0) };
}

unsafe fn buffer_opponent_intro_speech() {
    let opponent = unsafe { (&raw const gTrainerBattleOpponent_A).read() };
    if opponent < FRONTIER_TRAINERS_COUNT {
        let trainers = unsafe {
            (&raw const (*(&raw const crate::battle_tower::gFacilityTrainers).cast::<*const u8>()))
                .read()
        };
        let trainer = unsafe { trainers.add(usize::from(opponent) * FRONTIER_TRAINER_SIZE) };
        unsafe { FrontierSpeechToString(trainer.add(FRONTIER_TRAINER_SPEECH_BEFORE).cast()) };
    }
}

unsafe fn increment_palace_streak() {
    let lvl_mode = unsafe { lvl_mode() } as u8;
    let battle_mode = unsafe { battle_mode() } as u8 as usize;
    let lvl = usize::from(lvl_mode);
    let current = unsafe { streak(SB2_PALACE_WIN_STREAKS, battle_mode, lvl) };
    if unsafe { current.read() } < MAX_STREAK {
        unsafe { current.write(current.read() + 1) };
        // The original compares the wrong things here; kept as is.
        let record = unsafe { streak(SB2_PALACE_RECORD_WIN_STREAKS, battle_mode, lvl) };
        let index = usize::from(u16::from(lvl_mode) > unsafe { record.read() });
        if unsafe { streak(SB2_PALACE_WIN_STREAKS, battle_mode, index).read() } != 0 {
            unsafe { record.write(current.read()) };
        }
    }
}

unsafe fn save_palace_challenge() {
    let status = unsafe { (&raw const gSpecialVar_0x8005).read() };
    unsafe { sb2().add(SB2_CHALLENGE_STATUS).write(status as u8) };
    unsafe { VarSet(VAR_TEMP_CHALLENGE_STATUS, 0) };
    let flags = unsafe { sb2().add(SB2_FRONTIER_FLAGS) };
    unsafe { flags.write(flags.read() | CHALLENGE_PAUSED_BIT) };
    unsafe { SaveGameFrontier() };
}

unsafe fn set_random_palace_prize() {
    let battle_mode = unsafe { battle_mode() };
    let lvl_mode = unsafe { lvl_mode() };
    let roll = usize::from(Random());
    let prize = if unsafe { streak(SB2_PALACE_WIN_STREAKS, battle_mode, lvl_mode).read() } > 41 {
        LATE_PRIZES[roll % LATE_PRIZES.len()]
    } else {
        EARLY_PRIZES[roll % EARLY_PRIZES.len()]
    };
    unsafe { sb2_u16(SB2_PALACE_PRIZE).write(prize) };
}

unsafe fn give_palace_prize() {
    let prize = unsafe { sb2_u16(SB2_PALACE_PRIZE) };
    if unsafe { AddBagItem(prize.read(), 1) } == 1 {
        unsafe { CopyItemName(prize.read(), (&raw mut gStringVar1).cast()) };
        unsafe { prize.write(0) };
        unsafe { set_result(1) };
    } else {
        unsafe { set_result(0) };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn streak_flags_follow_mode_order() {
        assert_eq!(streak_flag(0, 0), 0x10);
        assert_eq!(streak_flag(0, 1), 0x20);
        assert_eq!(streak_flag(1, 0), 0x40_0000);
        assert_eq!(streak_flag(1, 1), 0x80_0000);
    }
}
