//! Setting up a fresh save file when the player picks NEW GAME, plus the
//! trainer id helpers shared with the rest of the game.

use crate::Random;
use crate::berry_powder::SetBerryPowder;
use crate::coins::SetCoins;
use crate::decoration_inventory::ClearDecorationInventories;
use crate::event_data::InitEventData;
use crate::load_save::{ClearSav1, ClearSav2, gSaveBlock1Ptr, gSaveBlock2Ptr};
use crate::lottery_corner::ResetLotteryCorner;
use crate::mail_data::ClearAllMail;
use crate::money::SetMoney;
use crate::play_time::PlayTimeCounter_Reset;
use crate::pokemon_size_record::{InitLotadSizeRecord, InitSeedotSizeRecord};
use crate::roamer::{ClearRoamerData, ClearRoamerLocationData};

const TRAINER_ID_LENGTH: usize = 4;

const SAVE_STATUS_EMPTY: u8 = 0;
const SAVE_STATUS_CORRUPT: u8 = 2;

const OPTIONS_TEXT_SPEED_MID: u16 = 1;
const ITEM_NONE: u16 = 0;
const EOS: u8 = 0xff;
const WARP_ID_NONE: i8 = -1;
const MAP_INSIDE_OF_TRUCK_GROUP: i8 = 0x19;
const MAP_INSIDE_OF_TRUCK_NUM: i8 = 0x28;

const MUSEUM_CONTEST_WINNERS_START: usize = 8;
const NUM_CONTEST_WINNERS: usize = 13;
const CONTEST_WINNER_SIZE: usize = 0x20;
const CONTEST_WINNER_MON_NAME: usize = 0x0b;
const CONTEST_WINNER_TRAINER_NAME: usize = 0x16;

// SaveBlock2
const SB2_PLAYER_TRAINER_ID: usize = 0x0a;
const SB2_OPTIONS: usize = 0x14;
const SB2_POKEDEX_OWNED: usize = 0x28;
const SB2_POKEDEX_SEEN: usize = 0x5c;
const POKEDEX_FLAGS_SIZE: usize = 0x34;
const SB2_SPECIAL_SAVE_WARP_FLAGS: usize = 0x09;
const SB2_GCN_LINK_FLAGS: usize = 0xa8;
const SB2_ENCRYPTION_KEY: usize = 0xac;
const SB2_BERRY_CRUSH: usize = 0x1ec;
const SB2_BERRY_POWDER: usize = 0x1f4;
const SB2_BERRY_PICK: usize = 0x20c;
const BERRY_RECORD_SIZE: usize = 0x10;
const SB2_FRONTIER: usize = 0x64c;
const FRONTIER_SIZE: usize = 0x8e0;
const SB2_FRONTIER_OPPONENT_NAMES: usize = 0xee1;
const OPPONENT_NAME_SIZE: usize = 8;

// SaveBlock1
const SB1_MONEY: usize = 0x490;
const SB1_REGISTERED_ITEM: usize = 0x496;
const SB1_CONTEST_WINNERS: usize = 0x2e90;

#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gDifferentSaveFile: u8 = 0;

#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gEnableContestDebugging: u8 = 0;

unsafe extern "C" {
    static EventScript_ResetAllMapFlags: u8;
    static mut gSaveFileStatus: u8;
    static mut gPlayerPartyCount: u8;
    static mut gUnusedPokedexU8: u8;

    fn GetGeneratedTrainerIdLower() -> u16;
    fn ClearContestWinnerPicsInContestHall();
    fn SetWarpDestination(map_group: i8, map_num: i8, warp_id: i8, x: i8, y: i8);
    fn WarpIntoMap();
    fn ResetPokedexScrollPositions();
    fn ZeroPlayerPartyMons();
    fn ZeroEnemyPartyMons();
    fn ResetBagScrollPositions();
    fn ResetPokeblockScrollPositions();
    fn RtcReset();
    fn ResetPokedex();
    fn ClearTVShowData();
    fn ResetGabbyAndTy();
    fn ClearSecretBases();
    fn ClearBerryTrees();
    fn ResetLinkContestBoolean();
    fn ResetGameStats();
    fn ClearPlayerLinkBattleRecords();
    fn ResetPokemonStorageSystem();
    fn ClearBag();
    fn NewGameInitPCItems();
    fn ClearPokeblocks();
    fn InitEasyChatPhrases();
    fn SetMauvilleOldMan();
    fn InitDewfordTrend();
    fn ResetFanClub();
    fn RunScriptImmediately(script: *const u8);
    fn InitUnionRoomChatRegisteredTexts();
    fn InitLilycoveLady();
    fn ResetAllApprenticeData();
    fn ClearRankingHallRecords();
    fn InitMatchCallCounters();
    fn ClearMysteryGift();
    fn WipeTrainerNameRecords();
    fn ResetTrainerHillResults();
    fn ResetContestLinkResults();
    fn ResetPokemonJumpRecords();
}

#[inline]
unsafe fn sb1() -> *mut u8 {
    unsafe { (&raw const gSaveBlock1Ptr).read() }
}

#[inline]
unsafe fn sb2() -> *mut u8 {
    unsafe { (&raw const gSaveBlock2Ptr).read() }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetTrainerId(trainer_id: u32, dst: *mut u8) {
    let bytes = trainer_id.to_le_bytes();
    unsafe { core::ptr::copy_nonoverlapping(bytes.as_ptr(), dst, TRAINER_ID_LENGTH) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetTrainerId(trainer_id: *const u8) -> u32 {
    let mut bytes = [0u8; TRAINER_ID_LENGTH];
    unsafe { core::ptr::copy_nonoverlapping(trainer_id, bytes.as_mut_ptr(), TRAINER_ID_LENGTH) };
    u32::from_le_bytes(bytes)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn CopyTrainerId(dst: *mut u8, src: *const u8) {
    for i in 0..TRAINER_ID_LENGTH {
        unsafe { dst.add(i).write(src.add(i).read()) };
    }
}

unsafe fn init_player_trainer_id() {
    let high = u32::from(unsafe { Random() }) << 16;
    let trainer_id = high | u32::from(unsafe { GetGeneratedTrainerIdLower() });
    unsafe { SetTrainerId(trainer_id, sb2().add(SB2_PLAYER_TRAINER_ID)) };
}

/// Text speed mid, frame 0, mono, shift, animations on, map zoomed out.
/// The button mode (L=A) is left alone, as in the original.
unsafe fn set_default_options() {
    let options = unsafe { sb2().add(SB2_OPTIONS).cast::<u16>() };
    let value = unsafe { options.read() };
    unsafe { options.write((value & !0x0fff) | OPTIONS_TEXT_SPEED_MID) };
}

unsafe fn clear_pokedex_flags() {
    unsafe { (&raw mut gUnusedPokedexU8).write(0) };
    unsafe {
        sb2()
            .add(SB2_POKEDEX_OWNED)
            .write_bytes(0, POKEDEX_FLAGS_SIZE)
    };
    unsafe {
        sb2()
            .add(SB2_POKEDEX_SEEN)
            .write_bytes(0, POKEDEX_FLAGS_SIZE)
    };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ClearAllContestWinnerPics() {
    unsafe { ClearContestWinnerPicsInContestHall() };

    // Museum paintings: an empty winner with blank names.
    for i in MUSEUM_CONTEST_WINNERS_START..NUM_CONTEST_WINNERS {
        let winner = unsafe { sb1().add(SB1_CONTEST_WINNERS + i * CONTEST_WINNER_SIZE) };
        unsafe { winner.write_bytes(0, CONTEST_WINNER_SIZE) };
        unsafe { winner.add(CONTEST_WINNER_MON_NAME).write(EOS) };
        unsafe { winner.add(CONTEST_WINNER_TRAINER_NAME).write(EOS) };
    }
}

unsafe fn clear_frontier_record() {
    unsafe { sb2().add(SB2_FRONTIER).write_bytes(0, FRONTIER_SIZE) };
    unsafe { sb2().add(SB2_FRONTIER_OPPONENT_NAMES).write(EOS) };
    unsafe {
        sb2()
            .add(SB2_FRONTIER_OPPONENT_NAMES + OPPONENT_NAME_SIZE)
            .write(EOS)
    };
}

unsafe fn warp_to_truck() {
    unsafe {
        SetWarpDestination(
            MAP_INSIDE_OF_TRUCK_GROUP,
            MAP_INSIDE_OF_TRUCK_NUM,
            WARP_ID_NONE,
            -1,
            -1,
        )
    };
    unsafe { WarpIntoMap() };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn Sav2_ClearSetDefault() {
    unsafe { ClearSav2() };
    unsafe { set_default_options() };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ResetMenuAndMonGlobals() {
    unsafe { (&raw mut gDifferentSaveFile).write(0) };
    unsafe { ResetPokedexScrollPositions() };
    unsafe { ZeroPlayerPartyMons() };
    unsafe { ZeroEnemyPartyMons() };
    unsafe { ResetBagScrollPositions() };
    unsafe { ResetPokeblockScrollPositions() };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn NewGameInitData() {
    let status = unsafe { (&raw const gSaveFileStatus).read() };
    if status == SAVE_STATUS_EMPTY || status == SAVE_STATUS_CORRUPT {
        unsafe { RtcReset() };
    }

    unsafe {
        (&raw mut gDifferentSaveFile).write(1);
        sb2().add(SB2_ENCRYPTION_KEY).cast::<u32>().write(0);
        ZeroPlayerPartyMons();
        ZeroEnemyPartyMons();
        ResetPokedex();
        clear_frontier_record();
        ClearSav1();
        ClearAllMail();
        sb2().add(SB2_SPECIAL_SAVE_WARP_FLAGS).write(0);
        sb2().add(SB2_GCN_LINK_FLAGS).write(0);
        init_player_trainer_id();
        PlayTimeCounter_Reset();
        clear_pokedex_flags();
        InitEventData();
        ClearTVShowData();
        ResetGabbyAndTy();
        ClearSecretBases();
        ClearBerryTrees();
        SetMoney(sb1().add(SB1_MONEY).cast(), 3000);
        SetCoins(0);
        ResetLinkContestBoolean();
        ResetGameStats();
        ClearAllContestWinnerPics();
        ClearPlayerLinkBattleRecords();
        InitSeedotSizeRecord();
        InitLotadSizeRecord();
        (&raw mut gPlayerPartyCount).write(0);
        ZeroPlayerPartyMons();
        ResetPokemonStorageSystem();
        ClearRoamerData();
        ClearRoamerLocationData();
        sb1()
            .add(SB1_REGISTERED_ITEM)
            .cast::<u16>()
            .write(ITEM_NONE);
        ClearBag();
        NewGameInitPCItems();
        ClearPokeblocks();
        ClearDecorationInventories();
        InitEasyChatPhrases();
        SetMauvilleOldMan();
        InitDewfordTrend();
        ResetFanClub();
        ResetLotteryCorner();
        warp_to_truck();
        RunScriptImmediately(&raw const EventScript_ResetAllMapFlags);
        reset_mini_games_records();
        InitUnionRoomChatRegisteredTexts();
        InitLilycoveLady();
        ResetAllApprenticeData();
        ClearRankingHallRecords();
        InitMatchCallCounters();
        ClearMysteryGift();
        WipeTrainerNameRecords();
        ResetTrainerHillResults();
        ResetContestLinkResults();
    }
}

unsafe fn reset_mini_games_records() {
    unsafe { sb2().add(SB2_BERRY_CRUSH).write_bytes(0, BERRY_RECORD_SIZE) };
    unsafe { SetBerryPowder(sb2().add(SB2_BERRY_POWDER).cast(), 0) };
    unsafe { ResetPokemonJumpRecords() };
    unsafe { sb2().add(SB2_BERRY_PICK).write_bytes(0, BERRY_RECORD_SIZE) };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trainer_ids_round_trip_little_endian() {
        let mut bytes = [0u8; 4];
        unsafe { SetTrainerId(0x1234_5678, bytes.as_mut_ptr()) };
        assert_eq!(bytes, [0x78, 0x56, 0x34, 0x12]);
        assert_eq!(unsafe { GetTrainerId(bytes.as_ptr()) }, 0x1234_5678);

        let mut copy = [0u8; 4];
        unsafe { CopyTrainerId(copy.as_mut_ptr(), bytes.as_ptr()) };
        assert_eq!(copy, bytes);
    }
}
