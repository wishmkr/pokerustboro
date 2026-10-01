//! Translated from `src/frontier_util.c` by tools/rustport/c2rs.py.
#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    static_mut_refs,
    unsafe_op_in_unsafe_fn,
    clashing_extern_declarations,
    unpredictable_function_pointer_comparisons,
    dangerous_implicit_autorefs,
    overflowing_literals,
    clippy::explicit_counter_loop,
    clippy::if_same_then_else,
    clippy::missing_transmute_annotations,
    clippy::too_many_arguments,
    clippy::type_complexity,
    unused_assignments,
    unused_variables
)]

use crate::apprentice::BufferApprenticeChallengeText;
use crate::battle_dome::{GetDomeTrainerSelectedMons, TrainerIdToDomeTournamentId};
use crate::battle_main::{gBattleOutcome, gBattleScripting, gBattleTypeFlags};
use crate::battle_records::gRecordsWindowId;
use crate::battle_setup::gTrainerBattleOpponent_A;
use crate::battle_tower::{
    FrontierSpeechToString, GetFrontierTrainerName, SetFacilityPtrsGetLevel,
    ValidateEReaderTrainer, gFacilityTrainers,
};
#[allow(unused_imports)]
use crate::c::*;
#[allow(unused_imports)]
use crate::consts::*;
use crate::event_data::{FlagGet, FlagSet, VarGet, VarSet};
use crate::ffi::{gSpecialVar_0x8004, gSpecialVar_0x8005, gSpecialVar_0x8006, gSpecialVar_Result};
use crate::field_specials::FrontierGamblerSetWonOrLost;
use crate::international_string_util::{
    GetStringCenterAlignXOffset, GetStringRightAlignXOffset, TVShowConvertInternationalString,
};
use crate::link::gLinkPlayers;
use crate::load_save::{
    ClearContinueGameWarpStatus2, LoadPlayerParty, SetContinueGameWarpStatusToDynamicWarp,
};
use crate::load_save::{gSaveBlock1Ptr, gSaveBlock2Ptr};
use crate::menu::DrawStdWindowFrame;
use crate::new_game::CopyTrainerId;
use crate::new_game::SetTrainerId;
use crate::overworld::SetGameStat;
use crate::party_menu::ClearSelectedPartyOrder;
use crate::party_menu::gSelectedOrderFromParty;
use crate::pokedex::GetSetPokedexFlag;
use crate::pokemon::{
    CalculateMonStats, CreateMon, GetMonData2, GetMonData3, GetNatureFromPersonality,
    IsShinyOtIdPersonality, SetMonData, SetMonMoveSlot, SpeciesToNationalPokedexNum,
    ZeroEnemyPartyMons, gEnemyParty, gPlayerParty, gPlayerPartyCount,
};
use crate::random::Random;
use crate::record_mixing::GetPlayerHallRecords;
use crate::recorded_battle::{
    GetRecordedBattleApprenticeId, GetRecordedBattleEasyChatSpeech,
    GetRecordedBattleFronterBrainSymbol, GetRecordedBattleFrontierFacility,
    MoveRecordedBattleToSaveData,
};
use crate::save::TrySavingData;
use crate::script_pokemon_util::ReducePlayerPartyToSelectedMons;
use crate::string_util::{
    ConvertIntToDecimalStringN, StringAppend, StringCopy, StringExpandPlaceholders,
};
use crate::string_util::{IsStringJapanese, StripExtCtrlCodes};
use crate::string_util::{gStringVar1, gStringVar2, gStringVar4};
use crate::tv::{IncrementDailyBattlePoints, ShouldAirFrontierTVShow, TryPutFrontierTVShowOnAir};
#[allow(unused_imports)]
use crate::types::*;
use crate::window::{CopyWindowToVram, FillWindowPixelBuffer, PutWindowTilemap};
#[allow(unused_imports)]
use core::ffi::c_void;
#[allow(unused_imports)]
use core::mem::zeroed;
#[allow(unused_imports)]
use core::ptr::null_mut;
/// `AddWindow` with this module's view of its types.
#[inline]
unsafe fn AddWindow(a0: *mut WindowTemplate) -> u16 {
    unsafe { crate::window::AddWindow(a0 as _) }
}
/// `Free` with this module's view of its types.
#[inline]
unsafe fn Free(a0: *mut c_void) {
    unsafe {
        crate::malloc::Free(a0 as _);
    }
}
// Data tables (translate with cdata.py): sFrontierBrainStreakAppearances sFrontierBrainsMons sBattlePointAwards sBattledBrainBitFlags sFrontierUtilFuncs sFrontierResultsWindowTemplate sLinkContestResultsWindowTemplate sRankingHallRecordsWindowTemplate sFrontierBrainObjEventGfx gFrontierBannedSpecies sRecordsWindowChallengeTexts sLevelModeText sHallFacilityToRecordsText sFrontierBrainTrainerIds sFrontierBrainPlayerLostSilverTexts sFrontierBrainPlayerWonSilverTexts sFrontierBrainPlayerLostGoldTexts sFrontierBrainPlayerWonGoldTexts sFrontierBrainPlayerLostTexts sFrontierBrainPlayerWonTexts

/// `struct FrontierBrainMon`
#[repr(C, align(4))]
#[derive(Clone, Copy)]
pub struct FrontierBrainMon {
    pub species: u16,
    pub heldItem: u16,
    pub fixedIV: u8,
    pub nature: u8,
    pub evs: CArray<u8, 6>,
    pub moves: CArray<u16, 4>,
}

unsafe impl Sync for FrontierBrainMon {}

#[cfg(target_arch = "arm")]
const _: () = {
    #[allow(unused_imports)]
    use core::mem::{offset_of, size_of};
    assert!(size_of::<FrontierBrainMon>() == 20);
    assert!(offset_of!(FrontierBrainMon, species) == 0);
    assert!(offset_of!(FrontierBrainMon, heldItem) == 2);
    assert!(offset_of!(FrontierBrainMon, fixedIV) == 4);
    assert!(offset_of!(FrontierBrainMon, nature) == 5);
    assert!(offset_of!(FrontierBrainMon, evs) == 6);
    assert!(offset_of!(FrontierBrainMon, moves) == 12);
};

const FRONTIER_BRAIN_OTID: u32 = 61226;

static gFrontierBannedSpecies: Table<CArray<u16, 11>> =
    Table((&raw const crate::data::frontier_util::gFrontierBannedSpecies).cast());
static sBattlePointAwards: Table<CArray<CArray<CArray<u8, 4>, 7>, 30>> =
    Table((&raw const crate::data::frontier_util::sBattlePointAwards).cast());
static sBattledBrainBitFlags: Table<CArray<CArray<u16, 2>, 7>> =
    Table((&raw const crate::data::frontier_util::sBattledBrainBitFlags).cast());
static sFrontierBrainObjEventGfx: Table<CArray<CArray<u8, 2>, 7>> =
    Table((&raw const crate::data::frontier_util::sFrontierBrainObjEventGfx).cast());
static sFrontierBrainPlayerLostTexts: Table<CArray<*mut *mut u8, 2>> =
    Table((&raw const crate::data::frontier_util::sFrontierBrainPlayerLostTexts).cast());
static sFrontierBrainPlayerWonTexts: Table<CArray<*mut *mut u8, 2>> =
    Table((&raw const crate::data::frontier_util::sFrontierBrainPlayerWonTexts).cast());
static sFrontierBrainStreakAppearances: Table<CArray<CArray<u8, 4>, 7>> =
    Table((&raw const crate::data::frontier_util::sFrontierBrainStreakAppearances).cast());
static sFrontierBrainTrainerIds: Table<CArray<u16, 7>> =
    Table((&raw const crate::data::frontier_util::sFrontierBrainTrainerIds).cast());
static sFrontierBrainsMons: Table<CArray<CArray<CArray<FrontierBrainMon, 3>, 2>, 7>> =
    Table((&raw const crate::data::frontier_util::sFrontierBrainsMons).cast());
static sFrontierResultsWindowTemplate: Table<WindowTemplate> =
    Table((&raw const crate::data::frontier_util::sFrontierResultsWindowTemplate).cast());
static sFrontierUtilFuncs: Table<CArray<Option<unsafe fn()>, 23>> =
    Table((&raw const crate::data::frontier_util::sFrontierUtilFuncs).cast());
static sHallFacilityToRecordsText: Table<CArray<*mut u8, 10>> =
    Table((&raw const crate::data::frontier_util::sHallFacilityToRecordsText).cast());
static sLevelModeText: Table<CArray<*mut u8, 2>> =
    Table((&raw const crate::data::frontier_util::sLevelModeText).cast());
static sLinkContestResultsWindowTemplate: Table<WindowTemplate> =
    Table((&raw const crate::data::frontier_util::sLinkContestResultsWindowTemplate).cast());
static sRankingHallRecordsWindowTemplate: Table<WindowTemplate> =
    Table((&raw const crate::data::frontier_util::sRankingHallRecordsWindowTemplate).cast());
static sRecordsWindowChallengeTexts: Table<CArray<CArray<*mut u8, 2>, 10>> =
    Table((&raw const crate::data::frontier_util::sRecordsWindowChallengeTexts).cast());

/// `AddTextPrinterParameterized` with this module's view of its types.
#[inline]
unsafe fn AddTextPrinterParameterized(
    a0: u8,
    a1: u8,
    a2: *mut u8,
    a3: u8,
    a4: u8,
    a5: u8,
    a6: Option<unsafe fn(*mut TextPrinterTemplate, u16)>,
) -> u16 {
    unsafe {
        crate::text::AddTextPrinterParameterized(
            a0,
            a1,
            a2 as _,
            a3,
            a4,
            a5,
            core::mem::transmute(a6),
        )
    }
}
/// `AllocZeroed` with this module's view of its types.
#[inline]
unsafe fn AllocZeroed(a0: u32) -> *mut c_void {
    unsafe { crate::malloc::AllocZeroed(a0) as *mut c_void }
}
/// `DoSoftReset` with this module's view of its types.
#[inline]
unsafe fn DoSoftReset() {
    unsafe {
        crate::agb_main::DoSoftReset();
    }
}

#[unsafe(no_mangle)]
pub unsafe fn CallFrontierUtilFunc() {
    sFrontierUtilFuncs[*(&raw const crate::ffi::gSpecialVar_0x8004)
        .cast::<u16>()
        .cast_mut()]
    .unwrap_unchecked()();
}
pub(crate) unsafe fn GetChallengeStatus() {
    VarSet(VAR_TEMP_CHALLENGE_STATUS, 0xFF);
    match (*gSaveBlock2Ptr).frontier.challengeStatus {
        0 => {}
        CHALLENGE_STATUS_SAVING => {
            FrontierGamblerSetWonOrLost(FALSE);
            VarSet(
                VAR_TEMP_CHALLENGE_STATUS,
                (*gSaveBlock2Ptr).frontier.challengeStatus as u16,
            );
        }
        CHALLENGE_STATUS_LOST => {
            FrontierGamblerSetWonOrLost(FALSE);
            VarSet(
                VAR_TEMP_CHALLENGE_STATUS,
                (*gSaveBlock2Ptr).frontier.challengeStatus as u16,
            );
        }
        CHALLENGE_STATUS_WON => {
            FrontierGamblerSetWonOrLost(TRUE);
            VarSet(
                VAR_TEMP_CHALLENGE_STATUS,
                (*gSaveBlock2Ptr).frontier.challengeStatus as u16,
            );
        }
        CHALLENGE_STATUS_PAUSED => {
            VarSet(
                VAR_TEMP_CHALLENGE_STATUS,
                (*gSaveBlock2Ptr).frontier.challengeStatus as u16,
            );
        }
        _ => {}
    }
}
pub(crate) unsafe fn GetFrontierData() {
    let facility: u8 = VarGet(VAR_FRONTIER_FACILITY) as u8;
    let mut hasSymbol: u8 = GetPlayerSymbolCountForFacility(facility);
    if hasSymbol == 2 {
        hasSymbol = 1;
    }
    match *(&raw const crate::ffi::gSpecialVar_0x8005)
        .cast::<u16>()
        .cast_mut()
    {
        FRONTIER_DATA_CHALLENGE_STATUS => {
            gSpecialVar_Result = (*gSaveBlock2Ptr).frontier.challengeStatus as u16;
        }
        FRONTIER_DATA_LVL_MODE => {
            gSpecialVar_Result = (*gSaveBlock2Ptr).frontier.lvlMode() as u16;
        }
        FRONTIER_DATA_BATTLE_NUM => {
            gSpecialVar_Result = (*gSaveBlock2Ptr).frontier.curChallengeBattleNum;
        }
        FRONTIER_DATA_PAUSED => {
            gSpecialVar_Result = (*gSaveBlock2Ptr).frontier.challengePaused() as u16;
        }
        FRONTIER_DATA_BATTLE_OUTCOME => {
            gSpecialVar_Result = gBattleOutcome as u16;
            gBattleOutcome = 0;
        }
        FRONTIER_DATA_RECORD_DISABLED => {
            gSpecialVar_Result = (*gSaveBlock2Ptr).frontier.disableRecordBattle() as u16;
        }
        FRONTIER_DATA_HEARD_BRAIN_SPEECH => {
            gSpecialVar_Result = (*gSaveBlock2Ptr).frontier.battledBrainFlags
                & sBattledBrainBitFlags[facility][hasSymbol];
        }
        _ => {}
    }
}
pub(crate) unsafe fn SetFrontierData() {
    let mut i: i32 = 0;
    let facility: u8 = VarGet(VAR_FRONTIER_FACILITY) as u8;
    let mut hasSymbol: u8 = GetPlayerSymbolCountForFacility(facility);
    if hasSymbol == 2 {
        hasSymbol = 1;
    }
    match *(&raw const crate::ffi::gSpecialVar_0x8005)
        .cast::<u16>()
        .cast_mut()
    {
        FRONTIER_DATA_CHALLENGE_STATUS => {
            (*gSaveBlock2Ptr).frontier.challengeStatus = gSpecialVar_0x8006 as u8;
        }
        FRONTIER_DATA_LVL_MODE => {
            (*gSaveBlock2Ptr)
                .frontier
                .set_lvlMode(gSpecialVar_0x8006 as u8);
        }
        FRONTIER_DATA_BATTLE_NUM => {
            (*gSaveBlock2Ptr).frontier.curChallengeBattleNum =
                *(&raw const crate::ffi::gSpecialVar_0x8006)
                    .cast::<u16>()
                    .cast_mut();
        }
        FRONTIER_DATA_PAUSED => {
            (*gSaveBlock2Ptr)
                .frontier
                .set_challengePaused(gSpecialVar_0x8006 as u8);
        }
        FRONTIER_DATA_SELECTED_MON_ORDER => {
            i = 0;
            while i
                < (if 3 >= (if 4 >= 2 { 4 } else { 2 }) {
                    3
                } else {
                    if 4 >= 2 { 4 } else { 2 }
                })
            {
                (*gSaveBlock2Ptr).frontier.selectedPartyMons[i] = gSelectedOrderFromParty[i] as u16;
                i += 1;
            }
        }
        FRONTIER_DATA_RECORD_DISABLED => {
            (*gSaveBlock2Ptr)
                .frontier
                .set_disableRecordBattle(gSpecialVar_0x8006 as u8);
        }
        FRONTIER_DATA_HEARD_BRAIN_SPEECH => {
            (*gSaveBlock2Ptr).frontier.battledBrainFlags |=
                sBattledBrainBitFlags[facility][hasSymbol];
        }
        _ => {}
    }
}
pub(crate) unsafe fn SetSelectedPartyOrder() {
    ClearSelectedPartyOrder();
    for i in 0..(gSpecialVar_0x8005 as i32) {
        gSelectedOrderFromParty[i] = (*gSaveBlock2Ptr).frontier.selectedPartyMons[i] as u8;
    }
    ReducePlayerPartyToSelectedMons();
}
pub(crate) unsafe fn DoSoftReset_() {
    DoSoftReset();
}
pub(crate) unsafe fn SetFrontierTrainers() {
    gFacilityTrainers = (*(&raw const crate::data::battle_tower::gBattleFrontierTrainers)
        .cast::<CArray<BattleFrontierTrainer, 0>>())
    .as_ptr()
    .cast_mut();
}
pub(crate) unsafe fn SaveSelectedParty() {
    let mut i: u8 = 0;
    while (i as i32)
        < (if 3 >= (if 4 >= 2 { 4 } else { 2 }) {
            3
        } else {
            if 4 >= 2 { 4 } else { 2 }
        })
    {
        let monId: u16 = (*gSaveBlock2Ptr).frontier.selectedPartyMons[i] - 1;
        if monId < PARTY_SIZE as u16 {
            (*gSaveBlock1Ptr).playerParty
                [(*gSaveBlock2Ptr).frontier.selectedPartyMons[i] as i32 - 1] = gPlayerParty[i];
        }
        i += 1;
    }
}
pub(crate) unsafe fn ShowFacilityResultsWindow() {
    if gSpecialVar_0x8006 >= FRONTIER_MODE_COUNT {
        gSpecialVar_0x8006 = 0;
    }
    match *(&raw const crate::ffi::gSpecialVar_0x8005)
        .cast::<u16>()
        .cast_mut()
    {
        FRONTIER_FACILITY_TOWER => {
            ShowTowerResultsWindow(gSpecialVar_0x8006 as u8);
        }
        1 => {
            ShowDomeResultsWindow(gSpecialVar_0x8006 as u8);
        }
        2 => {
            ShowPalaceResultsWindow(gSpecialVar_0x8006 as u8);
        }
        5 => {
            ShowPikeResultsWindow();
        }
        4 => {
            ShowFactoryResultsWindow(gSpecialVar_0x8006 as u8);
        }
        3 => {
            ShowArenaResultsWindow();
        }
        6 => {
            ShowPyramidResultsWindow();
        }
        FACILITY_LINK_CONTEST => {
            ShowLinkContestResultsWindow();
        }
        _ => {}
    }
}
unsafe fn IsWinStreakActive(challenge: u32) -> u8 {
    if (*gSaveBlock2Ptr).frontier.winStreakActiveFlags & challenge != 0 {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
unsafe fn PrintAligned(str: *mut u8, mut y: i32) {
    let x: i32 = GetStringCenterAlignXOffset(FONT_NORMAL as i32, str, 224);
    y = y * 8 + 1;
    AddTextPrinterParameterized(
        gRecordsWindowId.get(),
        FONT_NORMAL,
        str,
        x as u8,
        y as u8,
        TEXT_SKIP_DRAW,
        None,
    );
}
unsafe fn PrintHyphens(mut y: i32) {
    let mut text: CArray<u8, 37> = zeroed();
    let mut i: i32 = 0;
    while i < 36 {
        text[i] = CHAR_HYPHEN;
        i += 1;
    }
    text[i] = EOS;
    y = y * 8 + 1;
    AddTextPrinterParameterized(
        gRecordsWindowId.get(),
        FONT_NORMAL,
        text.as_mut_ptr(),
        4,
        y as u8,
        TEXT_SKIP_DRAW,
        None,
    );
}
unsafe fn TowerPrintStreak(str: *mut u8, mut num: u16, x1: u8, x2: u8, y: u8) {
    AddTextPrinterParameterized(
        gRecordsWindowId.get(),
        FONT_NORMAL,
        str,
        x1,
        y,
        TEXT_SKIP_DRAW,
        None,
    );
    if num > MAX_STREAK {
        num = MAX_STREAK;
    }
    ConvertIntToDecimalStringN(
        gStringVar1.as_mut_ptr(),
        num as i32,
        STR_CONV_MODE_RIGHT_ALIGN,
        4,
    );
    StringExpandPlaceholders(
        gStringVar4.as_mut_ptr(),
        (*(&raw const crate::data::strings::gText_WinStreak).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
    );
    AddTextPrinterParameterized(
        gRecordsWindowId.get(),
        FONT_NORMAL,
        gStringVar4.as_mut_ptr(),
        x2,
        y,
        TEXT_SKIP_DRAW,
        None,
    );
}
unsafe fn TowerPrintRecordStreak(battleMode: u8, lvlMode: u8, x1: u8, x2: u8, y: u8) {
    let num: u16 = (*gSaveBlock2Ptr).frontier.towerRecordWinStreaks[battleMode][lvlMode];
    TowerPrintStreak(
        (*(&raw const crate::data::strings::gText_Record).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        num,
        x1,
        x2,
        y,
    );
}
unsafe fn TowerGetWinStreak(battleMode: u8, lvlMode: u8) -> u16 {
    let winStreak: u16 = (*gSaveBlock2Ptr).frontier.towerWinStreaks[battleMode][lvlMode];
    if winStreak > MAX_STREAK {
        return MAX_STREAK;
    } else {
        return winStreak;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
unsafe fn TowerPrintPrevOrCurrentStreak(battleMode: u8, lvlMode: u8, x1: u8, x2: u8, y: u8) {
    let mut isCurrent: u8 = 0;
    let winStreak: u16 = TowerGetWinStreak(battleMode, lvlMode);
    match battleMode {
        1 => {
            if lvlMode != FRONTIER_LVL_50 {
                isCurrent = IsWinStreakActive(STREAK_TOWER_DOUBLES_OPEN);
            } else {
                isCurrent = IsWinStreakActive(STREAK_TOWER_DOUBLES_50);
            }
        }
        2 => {
            if lvlMode != FRONTIER_LVL_50 {
                isCurrent = IsWinStreakActive(STREAK_TOWER_MULTIS_OPEN);
            } else {
                isCurrent = IsWinStreakActive(STREAK_TOWER_MULTIS_50);
            }
        }
        3 => {
            if lvlMode != FRONTIER_LVL_50 {
                isCurrent = IsWinStreakActive(STREAK_TOWER_LINK_MULTIS_OPEN);
            } else {
                isCurrent = IsWinStreakActive(STREAK_TOWER_LINK_MULTIS_50);
            }
        }
        _ => {
            if lvlMode != FRONTIER_LVL_50 {
                isCurrent = IsWinStreakActive(STREAK_TOWER_SINGLES_OPEN);
            } else {
                isCurrent = IsWinStreakActive(STREAK_TOWER_SINGLES_50);
            }
        }
    }
    if isCurrent == TRUE {
        TowerPrintStreak(
            (*(&raw const crate::data::strings::gText_Current).cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
            winStreak,
            x1,
            x2,
            y,
        );
    } else {
        TowerPrintStreak(
            (*(&raw const crate::data::strings::gText_Prev).cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
            winStreak,
            x1,
            x2,
            y,
        );
    }
}
unsafe fn ShowTowerResultsWindow(battleMode: u8) {
    gRecordsWindowId.set(AddWindow((&raw const *sFrontierResultsWindowTemplate).cast_mut()) as u8);
    DrawStdWindowFrame(gRecordsWindowId.get(), FALSE);
    FillWindowPixelBuffer(gRecordsWindowId.get(), 17);
    if battleMode == FRONTIER_MODE_SINGLES as u8 {
        StringExpandPlaceholders(
            gStringVar4.as_mut_ptr(),
            (*(&raw const crate::data::strings::gText_SingleBattleRoomResults)
                .cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        );
    } else if battleMode == FRONTIER_MODE_DOUBLES as u8 {
        StringExpandPlaceholders(
            gStringVar4.as_mut_ptr(),
            (*(&raw const crate::data::strings::gText_DoubleBattleRoomResults)
                .cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        );
    } else if battleMode == FRONTIER_MODE_MULTIS as u8 {
        StringExpandPlaceholders(
            gStringVar4.as_mut_ptr(),
            (*(&raw const crate::data::strings::gText_MultiBattleRoomResults)
                .cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        );
    } else {
        StringExpandPlaceholders(
            gStringVar4.as_mut_ptr(),
            (*(&raw const crate::data::strings::gText_LinkMultiBattleRoomResults)
                .cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        );
    }
    PrintAligned(gStringVar4.as_mut_ptr(), 2);
    AddTextPrinterParameterized(
        gRecordsWindowId.get(),
        FONT_NORMAL,
        (*(&raw const crate::data::strings::gText_Lv502).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        16,
        49,
        TEXT_SKIP_DRAW,
        None,
    );
    AddTextPrinterParameterized(
        gRecordsWindowId.get(),
        FONT_NORMAL,
        (*(&raw const crate::data::strings::gText_OpenLv).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        16,
        97,
        TEXT_SKIP_DRAW,
        None,
    );
    PrintHyphens(10);
    TowerPrintPrevOrCurrentStreak(battleMode, FRONTIER_LVL_50, 72, 132, 49);
    TowerPrintRecordStreak(battleMode, FRONTIER_LVL_50, 72, 132, 65);
    TowerPrintPrevOrCurrentStreak(battleMode, FRONTIER_LVL_OPEN, 72, 132, 97);
    TowerPrintRecordStreak(battleMode, FRONTIER_LVL_OPEN, 72, 132, 113);
    PutWindowTilemap(gRecordsWindowId.get());
    CopyWindowToVram(gRecordsWindowId.get(), COPYWIN_FULL);
}
unsafe fn DomeGetWinStreak(battleMode: u8, lvlMode: u8) -> u16 {
    let winStreak: u16 = (*gSaveBlock2Ptr).frontier.domeWinStreaks[battleMode][lvlMode];
    if winStreak > MAX_STREAK {
        return MAX_STREAK;
    } else {
        return winStreak;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
unsafe fn PrintTwoStrings(str1: *mut u8, str2: *mut u8, num: u16, x1: u8, x2: u8, y: u8) {
    AddTextPrinterParameterized(
        gRecordsWindowId.get(),
        FONT_NORMAL,
        str1,
        x1,
        y,
        TEXT_SKIP_DRAW,
        None,
    );
    ConvertIntToDecimalStringN(
        gStringVar1.as_mut_ptr(),
        num as i32,
        STR_CONV_MODE_RIGHT_ALIGN,
        4,
    );
    StringExpandPlaceholders(gStringVar4.as_mut_ptr(), str2);
    AddTextPrinterParameterized(
        gRecordsWindowId.get(),
        FONT_NORMAL,
        gStringVar4.as_mut_ptr(),
        x2,
        y,
        TEXT_SKIP_DRAW,
        None,
    );
}
unsafe fn DomePrintPrevOrCurrentStreak(battleMode: u8, lvlMode: u8, x1: u8, x2: u8, y: u8) {
    let mut isCurrent: u8 = 0;
    let winStreak: u16 = DomeGetWinStreak(battleMode, lvlMode);
    match battleMode {
        1 => {
            if lvlMode != FRONTIER_LVL_50 {
                isCurrent = IsWinStreakActive(STREAK_DOME_DOUBLES_OPEN);
            } else {
                isCurrent = IsWinStreakActive(STREAK_DOME_DOUBLES_50);
            }
        }
        _ => {
            if lvlMode != FRONTIER_LVL_50 {
                isCurrent = IsWinStreakActive(STREAK_DOME_SINGLES_OPEN);
            } else {
                isCurrent = IsWinStreakActive(STREAK_DOME_SINGLES_50);
            }
        }
    }
    if isCurrent == TRUE {
        PrintTwoStrings(
            (*(&raw const crate::data::strings::gText_Current).cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
            (*(&raw const crate::data::strings::gText_ClearStreak).cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
            winStreak,
            x1,
            x2,
            y,
        );
    } else {
        PrintTwoStrings(
            (*(&raw const crate::data::strings::gText_Prev).cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
            (*(&raw const crate::data::strings::gText_ClearStreak).cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
            winStreak,
            x1,
            x2,
            y,
        );
    }
}
unsafe fn ShowDomeResultsWindow(battleMode: u8) {
    gRecordsWindowId.set(AddWindow((&raw const *sFrontierResultsWindowTemplate).cast_mut()) as u8);
    DrawStdWindowFrame(gRecordsWindowId.get(), FALSE);
    FillWindowPixelBuffer(gRecordsWindowId.get(), 17);
    if battleMode == FRONTIER_MODE_SINGLES as u8 {
        StringExpandPlaceholders(
            gStringVar4.as_mut_ptr(),
            (*(&raw const crate::data::strings::gText_SingleBattleTourneyResults)
                .cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        );
    } else {
        StringExpandPlaceholders(
            gStringVar4.as_mut_ptr(),
            (*(&raw const crate::data::strings::gText_DoubleBattleTourneyResults)
                .cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        );
    }
    PrintAligned(gStringVar4.as_mut_ptr(), 0);
    AddTextPrinterParameterized(
        gRecordsWindowId.get(),
        FONT_NORMAL,
        (*(&raw const crate::data::strings::gText_Lv502).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        8,
        33,
        TEXT_SKIP_DRAW,
        None,
    );
    AddTextPrinterParameterized(
        gRecordsWindowId.get(),
        FONT_NORMAL,
        (*(&raw const crate::data::strings::gText_OpenLv).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        8,
        97,
        TEXT_SKIP_DRAW,
        None,
    );
    PrintHyphens(10);
    DomePrintPrevOrCurrentStreak(battleMode, FRONTIER_LVL_50, 64, 121, 33);
    PrintTwoStrings(
        (*(&raw const crate::data::strings::gText_Record).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        (*(&raw const crate::data::strings::gText_ClearStreak).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        (*gSaveBlock2Ptr).frontier.domeRecordWinStreaks[battleMode][0],
        64,
        121,
        49,
    );
    PrintTwoStrings(
        (*(&raw const crate::data::strings::gText_Total).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        (*(&raw const crate::data::strings::gText_Championships).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        (*gSaveBlock2Ptr).frontier.domeTotalChampionships[battleMode][0],
        64,
        112,
        65,
    );
    DomePrintPrevOrCurrentStreak(battleMode, FRONTIER_LVL_OPEN, 64, 121, 97);
    PrintTwoStrings(
        (*(&raw const crate::data::strings::gText_Record).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        (*(&raw const crate::data::strings::gText_ClearStreak).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        (*gSaveBlock2Ptr).frontier.domeRecordWinStreaks[battleMode][1],
        64,
        121,
        113,
    );
    PrintTwoStrings(
        (*(&raw const crate::data::strings::gText_Total).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        (*(&raw const crate::data::strings::gText_Championships).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        (*gSaveBlock2Ptr).frontier.domeTotalChampionships[battleMode][1],
        64,
        112,
        129,
    );
    PutWindowTilemap(gRecordsWindowId.get());
    CopyWindowToVram(gRecordsWindowId.get(), COPYWIN_FULL);
}
unsafe fn PalacePrintStreak(str: *mut u8, mut num: u16, x1: u8, x2: u8, y: u8) {
    AddTextPrinterParameterized(
        gRecordsWindowId.get(),
        FONT_NORMAL,
        str,
        x1,
        y,
        TEXT_SKIP_DRAW,
        None,
    );
    if num > MAX_STREAK {
        num = MAX_STREAK;
    }
    ConvertIntToDecimalStringN(
        gStringVar1.as_mut_ptr(),
        num as i32,
        STR_CONV_MODE_RIGHT_ALIGN,
        4,
    );
    StringExpandPlaceholders(
        gStringVar4.as_mut_ptr(),
        (*(&raw const crate::data::strings::gText_WinStreak).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
    );
    AddTextPrinterParameterized(
        gRecordsWindowId.get(),
        FONT_NORMAL,
        gStringVar4.as_mut_ptr(),
        x2,
        y,
        TEXT_SKIP_DRAW,
        None,
    );
}
unsafe fn PalacePrintRecordStreak(battleMode: u8, lvlMode: u8, x1: u8, x2: u8, y: u8) {
    let num: u16 = (*gSaveBlock2Ptr).frontier.palaceRecordWinStreaks[battleMode][lvlMode];
    PalacePrintStreak(
        (*(&raw const crate::data::strings::gText_Record).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        num,
        x1,
        x2,
        y,
    );
}
unsafe fn PalaceGetWinStreak(battleMode: u8, lvlMode: u8) -> u16 {
    let winStreak: u16 = (*gSaveBlock2Ptr).frontier.palaceWinStreaks[battleMode][lvlMode];
    if winStreak > MAX_STREAK {
        return MAX_STREAK;
    } else {
        return winStreak;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
unsafe fn PalacePrintPrevOrCurrentStreak(battleMode: u8, lvlMode: u8, x1: u8, x2: u8, y: u8) {
    let mut isCurrent: u8 = 0;
    let winStreak: u16 = PalaceGetWinStreak(battleMode, lvlMode);
    match battleMode {
        1 => {
            if lvlMode != FRONTIER_LVL_50 {
                isCurrent = IsWinStreakActive(STREAK_PALACE_DOUBLES_OPEN);
            } else {
                isCurrent = IsWinStreakActive(STREAK_PALACE_DOUBLES_50);
            }
        }
        _ => {
            if lvlMode != FRONTIER_LVL_50 {
                isCurrent = IsWinStreakActive(STREAK_PALACE_SINGLES_OPEN);
            } else {
                isCurrent = IsWinStreakActive(STREAK_PALACE_SINGLES_50);
            }
        }
    }
    if isCurrent == TRUE {
        PalacePrintStreak(
            (*(&raw const crate::data::strings::gText_Current).cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
            winStreak,
            x1,
            x2,
            y,
        );
    } else {
        PalacePrintStreak(
            (*(&raw const crate::data::strings::gText_Prev).cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
            winStreak,
            x1,
            x2,
            y,
        );
    }
}
unsafe fn ShowPalaceResultsWindow(battleMode: u8) {
    gRecordsWindowId.set(AddWindow((&raw const *sFrontierResultsWindowTemplate).cast_mut()) as u8);
    DrawStdWindowFrame(gRecordsWindowId.get(), FALSE);
    FillWindowPixelBuffer(gRecordsWindowId.get(), 17);
    if battleMode == FRONTIER_MODE_SINGLES as u8 {
        StringExpandPlaceholders(
            gStringVar4.as_mut_ptr(),
            (*(&raw const crate::data::strings::gText_SingleBattleHallResults)
                .cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        );
    } else {
        StringExpandPlaceholders(
            gStringVar4.as_mut_ptr(),
            (*(&raw const crate::data::strings::gText_DoubleBattleHallResults)
                .cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        );
    }
    PrintAligned(gStringVar4.as_mut_ptr(), 2);
    AddTextPrinterParameterized(
        gRecordsWindowId.get(),
        FONT_NORMAL,
        (*(&raw const crate::data::strings::gText_Lv502).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        16,
        49,
        TEXT_SKIP_DRAW,
        None,
    );
    AddTextPrinterParameterized(
        gRecordsWindowId.get(),
        FONT_NORMAL,
        (*(&raw const crate::data::strings::gText_OpenLv).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        16,
        97,
        TEXT_SKIP_DRAW,
        None,
    );
    PrintHyphens(10);
    PalacePrintPrevOrCurrentStreak(battleMode, FRONTIER_LVL_50, 72, 131, 49);
    PalacePrintRecordStreak(battleMode, FRONTIER_LVL_50, 72, 131, 65);
    PalacePrintPrevOrCurrentStreak(battleMode, FRONTIER_LVL_OPEN, 72, 131, 97);
    PalacePrintRecordStreak(battleMode, FRONTIER_LVL_OPEN, 72, 131, 113);
    PutWindowTilemap(gRecordsWindowId.get());
    CopyWindowToVram(gRecordsWindowId.get(), COPYWIN_FULL);
}
unsafe fn PikeGetWinStreak(lvlMode: u8) -> u16 {
    let winStreak: u16 = (*gSaveBlock2Ptr).frontier.pikeWinStreaks[lvlMode];
    if winStreak > MAX_STREAK {
        return MAX_STREAK;
    } else {
        return winStreak;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
unsafe fn PikePrintCleared(str1: *mut u8, str2: *mut u8, num: u16, x1: u8, x2: u8, y: u8) {
    AddTextPrinterParameterized(
        gRecordsWindowId.get(),
        FONT_NORMAL,
        str1,
        x1,
        y,
        TEXT_SKIP_DRAW,
        None,
    );
    ConvertIntToDecimalStringN(
        gStringVar1.as_mut_ptr(),
        num as i32,
        STR_CONV_MODE_RIGHT_ALIGN,
        4,
    );
    StringExpandPlaceholders(gStringVar4.as_mut_ptr(), str2);
    AddTextPrinterParameterized(
        gRecordsWindowId.get(),
        FONT_NORMAL,
        gStringVar4.as_mut_ptr(),
        x2,
        y,
        TEXT_SKIP_DRAW,
        None,
    );
}
unsafe fn PikePrintPrevOrCurrentStreak(lvlMode: u8, x1: u8, x2: u8, y: u8) {
    let mut isCurrent: u8 = 0;
    let winStreak: u16 = PikeGetWinStreak(lvlMode);
    if lvlMode != FRONTIER_LVL_50 {
        isCurrent = IsWinStreakActive(STREAK_PIKE_OPEN);
    } else {
        isCurrent = IsWinStreakActive(STREAK_PIKE_50);
    }
    if isCurrent == TRUE {
        PrintTwoStrings(
            (*(&raw const crate::data::strings::gText_Current).cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
            (*(&raw const crate::data::strings::gText_RoomsCleared).cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
            winStreak,
            x1,
            x2,
            y,
        );
    } else {
        PrintTwoStrings(
            (*(&raw const crate::data::strings::gText_Prev).cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
            (*(&raw const crate::data::strings::gText_RoomsCleared).cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
            winStreak,
            x1,
            x2,
            y,
        );
    }
}
unsafe fn ShowPikeResultsWindow() {
    gRecordsWindowId.set(AddWindow((&raw const *sFrontierResultsWindowTemplate).cast_mut()) as u8);
    DrawStdWindowFrame(gRecordsWindowId.get(), FALSE);
    FillWindowPixelBuffer(gRecordsWindowId.get(), 17);
    StringExpandPlaceholders(
        gStringVar4.as_mut_ptr(),
        (*(&raw const crate::data::strings::gText_BattleChoiceResults).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
    );
    PrintAligned(gStringVar4.as_mut_ptr(), 0);
    AddTextPrinterParameterized(
        gRecordsWindowId.get(),
        FONT_NORMAL,
        (*(&raw const crate::data::strings::gText_Lv502).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        8,
        33,
        TEXT_SKIP_DRAW,
        None,
    );
    AddTextPrinterParameterized(
        gRecordsWindowId.get(),
        FONT_NORMAL,
        (*(&raw const crate::data::strings::gText_OpenLv).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        8,
        97,
        TEXT_SKIP_DRAW,
        None,
    );
    PrintHyphens(10);
    PikePrintPrevOrCurrentStreak(FRONTIER_LVL_50, 64, 114, 33);
    PikePrintCleared(
        (*(&raw const crate::data::strings::gText_Record).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        (*(&raw const crate::data::strings::gText_RoomsCleared).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        (*gSaveBlock2Ptr).frontier.pikeRecordStreaks[0],
        64,
        114,
        49,
    );
    PikePrintCleared(
        (*(&raw const crate::data::strings::gText_Total).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        (*(&raw const crate::data::strings::gText_TimesCleared).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        (*gSaveBlock2Ptr).frontier.pikeTotalStreaks[0],
        64,
        114,
        65,
    );
    PikePrintPrevOrCurrentStreak(FRONTIER_LVL_OPEN, 64, 114, 97);
    PikePrintCleared(
        (*(&raw const crate::data::strings::gText_Record).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        (*(&raw const crate::data::strings::gText_RoomsCleared).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        (*gSaveBlock2Ptr).frontier.pikeRecordStreaks[1],
        64,
        114,
        113,
    );
    PikePrintCleared(
        (*(&raw const crate::data::strings::gText_Total).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        (*(&raw const crate::data::strings::gText_TimesCleared).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        (*gSaveBlock2Ptr).frontier.pikeTotalStreaks[1],
        64,
        114,
        129,
    );
    PutWindowTilemap(gRecordsWindowId.get());
    CopyWindowToVram(gRecordsWindowId.get(), COPYWIN_FULL);
}
unsafe fn ArenaPrintStreak(str: *mut u8, mut num: u16, x1: u8, x2: u8, y: u8) {
    AddTextPrinterParameterized(
        gRecordsWindowId.get(),
        FONT_NORMAL,
        str,
        x1,
        y,
        TEXT_SKIP_DRAW,
        None,
    );
    if num > MAX_STREAK {
        num = MAX_STREAK;
    }
    ConvertIntToDecimalStringN(
        gStringVar1.as_mut_ptr(),
        num as i32,
        STR_CONV_MODE_RIGHT_ALIGN,
        4,
    );
    StringExpandPlaceholders(
        gStringVar4.as_mut_ptr(),
        (*(&raw const crate::data::strings::gText_KOsInARow).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
    );
    AddTextPrinterParameterized(
        gRecordsWindowId.get(),
        FONT_NORMAL,
        gStringVar4.as_mut_ptr(),
        x2,
        y,
        TEXT_SKIP_DRAW,
        None,
    );
}
unsafe fn ArenaPrintRecordStreak(lvlMode: u8, x1: u8, x2: u8, y: u8) {
    let num: u16 = (*gSaveBlock2Ptr).frontier.arenaRecordStreaks[lvlMode];
    ArenaPrintStreak(
        (*(&raw const crate::data::strings::gText_Record).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        num,
        x1,
        x2,
        y,
    );
}
unsafe fn ArenaGetWinStreak(lvlMode: u8) -> u16 {
    let winStreak: u16 = (*gSaveBlock2Ptr).frontier.arenaWinStreaks[lvlMode];
    if winStreak > MAX_STREAK {
        return MAX_STREAK;
    } else {
        return winStreak;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
unsafe fn ArenaPrintPrevOrCurrentStreak(lvlMode: u8, x1: u8, x2: u8, y: u8) {
    let mut isCurrent: u8 = 0;
    let winStreak: u16 = ArenaGetWinStreak(lvlMode);
    if lvlMode != FRONTIER_LVL_50 {
        isCurrent = IsWinStreakActive(STREAK_ARENA_OPEN);
    } else {
        isCurrent = IsWinStreakActive(STREAK_ARENA_50);
    }
    if isCurrent == TRUE {
        ArenaPrintStreak(
            (*(&raw const crate::data::strings::gText_Current).cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
            winStreak,
            x1,
            x2,
            y,
        );
    } else {
        ArenaPrintStreak(
            (*(&raw const crate::data::strings::gText_Prev).cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
            winStreak,
            x1,
            x2,
            y,
        );
    }
}
unsafe fn ShowArenaResultsWindow() {
    gRecordsWindowId.set(AddWindow((&raw const *sFrontierResultsWindowTemplate).cast_mut()) as u8);
    DrawStdWindowFrame(gRecordsWindowId.get(), FALSE);
    FillWindowPixelBuffer(gRecordsWindowId.get(), 17);
    PrintHyphens(10);
    StringExpandPlaceholders(
        gStringVar4.as_mut_ptr(),
        (*(&raw const crate::data::strings::gText_SetKOTourneyResults).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
    );
    PrintAligned(gStringVar4.as_mut_ptr(), 2);
    AddTextPrinterParameterized(
        gRecordsWindowId.get(),
        FONT_NORMAL,
        (*(&raw const crate::data::strings::gText_Lv502).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        16,
        49,
        TEXT_SKIP_DRAW,
        None,
    );
    AddTextPrinterParameterized(
        gRecordsWindowId.get(),
        FONT_NORMAL,
        (*(&raw const crate::data::strings::gText_OpenLv).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        16,
        97,
        TEXT_SKIP_DRAW,
        None,
    );
    ArenaPrintPrevOrCurrentStreak(FRONTIER_LVL_50, 72, 126, 49);
    ArenaPrintRecordStreak(FRONTIER_LVL_50, 72, 126, 65);
    ArenaPrintPrevOrCurrentStreak(FRONTIER_LVL_OPEN, 72, 126, 97);
    ArenaPrintRecordStreak(FRONTIER_LVL_OPEN, 72, 126, 113);
    PutWindowTilemap(gRecordsWindowId.get());
    CopyWindowToVram(gRecordsWindowId.get(), COPYWIN_FULL);
}
unsafe fn FactoryPrintStreak(
    str: *mut u8,
    mut num1: u16,
    num2: u16,
    x1: u8,
    x2: u8,
    x3: u8,
    y: u8,
) {
    AddTextPrinterParameterized(
        gRecordsWindowId.get(),
        FONT_NORMAL,
        str,
        x1,
        y,
        TEXT_SKIP_DRAW,
        None,
    );
    if num1 > MAX_STREAK {
        num1 = MAX_STREAK;
    }
    ConvertIntToDecimalStringN(
        gStringVar1.as_mut_ptr(),
        num1 as i32,
        STR_CONV_MODE_RIGHT_ALIGN,
        4,
    );
    StringExpandPlaceholders(
        gStringVar4.as_mut_ptr(),
        (*(&raw const crate::data::strings::gText_WinStreak).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
    );
    AddTextPrinterParameterized(
        gRecordsWindowId.get(),
        FONT_NORMAL,
        gStringVar4.as_mut_ptr(),
        x2,
        y,
        TEXT_SKIP_DRAW,
        None,
    );
    ConvertIntToDecimalStringN(
        gStringVar1.as_mut_ptr(),
        num2 as i32,
        STR_CONV_MODE_RIGHT_ALIGN,
        4,
    );
    StringExpandPlaceholders(
        gStringVar4.as_mut_ptr(),
        (*(&raw const crate::data::strings::gText_TimesVar1).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
    );
    AddTextPrinterParameterized(
        gRecordsWindowId.get(),
        FONT_NORMAL,
        gStringVar4.as_mut_ptr(),
        x3,
        y,
        TEXT_SKIP_DRAW,
        None,
    );
}
unsafe fn FactoryPrintRecordStreak(battleMode: u8, lvlMode: u8, x1: u8, x2: u8, x3: u8, y: u8) {
    let num1: u16 = (*gSaveBlock2Ptr).frontier.factoryRecordWinStreaks[battleMode][lvlMode];
    let num2: u16 = (*gSaveBlock2Ptr).frontier.factoryRecordRentsCount[battleMode][lvlMode];
    FactoryPrintStreak(
        (*(&raw const crate::data::strings::gText_Record).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        num1,
        num2,
        x1,
        x2,
        x3,
        y,
    );
}
unsafe fn FactoryGetWinStreak(battleMode: u8, lvlMode: u8) -> u16 {
    let winStreak: u16 = (*gSaveBlock2Ptr).frontier.factoryWinStreaks[battleMode][lvlMode];
    if winStreak > MAX_STREAK {
        return MAX_STREAK;
    } else {
        return winStreak;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
unsafe fn FactoryGetRentsCount(battleMode: u8, lvlMode: u8) -> u16 {
    let rents: u16 = (*gSaveBlock2Ptr).frontier.factoryRentsCount[battleMode][lvlMode];
    if rents > MAX_STREAK {
        return MAX_STREAK;
    } else {
        return rents;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
unsafe fn FactoryPrintPrevOrCurrentStreak(
    battleMode: u8,
    lvlMode: u8,
    x1: u8,
    x2: u8,
    x3: u8,
    y: u8,
) {
    let mut isCurrent: u8 = 0;
    let winStreak: u16 = FactoryGetWinStreak(battleMode, lvlMode);
    let rents: u16 = FactoryGetRentsCount(battleMode, lvlMode);
    match battleMode {
        1 => {
            if lvlMode != FRONTIER_LVL_50 {
                isCurrent = IsWinStreakActive(STREAK_FACTORY_DOUBLES_OPEN);
            } else {
                isCurrent = IsWinStreakActive(STREAK_FACTORY_DOUBLES_50);
            }
        }
        _ => {
            if lvlMode != FRONTIER_LVL_50 {
                isCurrent = IsWinStreakActive(STREAK_FACTORY_SINGLES_OPEN);
            } else {
                isCurrent = IsWinStreakActive(STREAK_FACTORY_SINGLES_50);
            }
        }
    }
    if isCurrent == TRUE {
        FactoryPrintStreak(
            (*(&raw const crate::data::strings::gText_Current).cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
            winStreak,
            rents,
            x1,
            x2,
            x3,
            y,
        );
    } else {
        FactoryPrintStreak(
            (*(&raw const crate::data::strings::gText_Prev).cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
            winStreak,
            rents,
            x1,
            x2,
            x3,
            y,
        );
    }
}
unsafe fn ShowFactoryResultsWindow(battleMode: u8) {
    gRecordsWindowId.set(AddWindow((&raw const *sFrontierResultsWindowTemplate).cast_mut()) as u8);
    DrawStdWindowFrame(gRecordsWindowId.get(), FALSE);
    FillWindowPixelBuffer(gRecordsWindowId.get(), 17);
    if battleMode == FRONTIER_MODE_SINGLES as u8 {
        StringExpandPlaceholders(
            gStringVar4.as_mut_ptr(),
            (*(&raw const crate::data::strings::gText_BattleSwapSingleResults)
                .cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        );
    } else {
        StringExpandPlaceholders(
            gStringVar4.as_mut_ptr(),
            (*(&raw const crate::data::strings::gText_BattleSwapDoubleResults)
                .cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        );
    }
    PrintAligned(gStringVar4.as_mut_ptr(), 0);
    AddTextPrinterParameterized(
        gRecordsWindowId.get(),
        FONT_NORMAL,
        (*(&raw const crate::data::strings::gText_Lv502).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        8,
        33,
        TEXT_SKIP_DRAW,
        None,
    );
    AddTextPrinterParameterized(
        gRecordsWindowId.get(),
        FONT_NORMAL,
        (*(&raw const crate::data::strings::gText_RentalSwap).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        152,
        33,
        TEXT_SKIP_DRAW,
        None,
    );
    AddTextPrinterParameterized(
        gRecordsWindowId.get(),
        FONT_NORMAL,
        (*(&raw const crate::data::strings::gText_OpenLv).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        8,
        97,
        TEXT_SKIP_DRAW,
        None,
    );
    PrintHyphens(10);
    FactoryPrintPrevOrCurrentStreak(battleMode, FRONTIER_LVL_50, 8, 64, 158, 49);
    FactoryPrintRecordStreak(battleMode, FRONTIER_LVL_50, 8, 64, 158, 65);
    FactoryPrintPrevOrCurrentStreak(battleMode, FRONTIER_LVL_OPEN, 8, 64, 158, 113);
    FactoryPrintRecordStreak(battleMode, FRONTIER_LVL_OPEN, 8, 64, 158, 129);
    PutWindowTilemap(gRecordsWindowId.get());
    CopyWindowToVram(gRecordsWindowId.get(), COPYWIN_FULL);
}
unsafe fn PyramidPrintStreak(str: *mut u8, mut num: u16, x1: u8, x2: u8, y: u8) {
    AddTextPrinterParameterized(
        gRecordsWindowId.get(),
        FONT_NORMAL,
        str,
        x1,
        y,
        TEXT_SKIP_DRAW,
        None,
    );
    if num > MAX_STREAK {
        num = MAX_STREAK;
    }
    ConvertIntToDecimalStringN(
        gStringVar1.as_mut_ptr(),
        num as i32,
        STR_CONV_MODE_RIGHT_ALIGN,
        4,
    );
    StringExpandPlaceholders(
        gStringVar4.as_mut_ptr(),
        (*(&raw const crate::data::strings::gText_FloorsCleared).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
    );
    AddTextPrinterParameterized(
        gRecordsWindowId.get(),
        FONT_NORMAL,
        gStringVar4.as_mut_ptr(),
        x2,
        y,
        TEXT_SKIP_DRAW,
        None,
    );
}
unsafe fn PyramidPrintRecordStreak(lvlMode: u8, x1: u8, x2: u8, y: u8) {
    let num: u16 = (*gSaveBlock2Ptr).frontier.pyramidRecordStreaks[lvlMode];
    PyramidPrintStreak(
        (*(&raw const crate::data::strings::gText_Record).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        num,
        x1,
        x2,
        y,
    );
}
unsafe fn PyramidGetWinStreak(lvlMode: u8) -> u16 {
    let winStreak: u16 = (*gSaveBlock2Ptr).frontier.pyramidWinStreaks[lvlMode];
    if winStreak > MAX_STREAK {
        return MAX_STREAK;
    } else {
        return winStreak;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
unsafe fn PyramidPrintPrevOrCurrentStreak(lvlMode: u8, x1: u8, x2: u8, y: u8) {
    let mut isCurrent: u8 = 0;
    let winStreak: u16 = PyramidGetWinStreak(lvlMode);
    if lvlMode != FRONTIER_LVL_50 {
        isCurrent = IsWinStreakActive(STREAK_PYRAMID_OPEN);
    } else {
        isCurrent = IsWinStreakActive(STREAK_PYRAMID_50);
    }
    if isCurrent == TRUE {
        PyramidPrintStreak(
            (*(&raw const crate::data::strings::gText_Current).cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
            winStreak,
            x1,
            x2,
            y,
        );
    } else {
        PyramidPrintStreak(
            (*(&raw const crate::data::strings::gText_Prev).cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
            winStreak,
            x1,
            x2,
            y,
        );
    }
}
unsafe fn ShowPyramidResultsWindow() {
    gRecordsWindowId.set(AddWindow((&raw const *sFrontierResultsWindowTemplate).cast_mut()) as u8);
    DrawStdWindowFrame(gRecordsWindowId.get(), FALSE);
    FillWindowPixelBuffer(gRecordsWindowId.get(), 17);
    StringExpandPlaceholders(
        gStringVar4.as_mut_ptr(),
        (*(&raw const crate::data::strings::gText_BattleQuestResults).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
    );
    PrintAligned(gStringVar4.as_mut_ptr(), 2);
    AddTextPrinterParameterized(
        gRecordsWindowId.get(),
        FONT_NORMAL,
        (*(&raw const crate::data::strings::gText_Lv502).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        8,
        49,
        TEXT_SKIP_DRAW,
        None,
    );
    AddTextPrinterParameterized(
        gRecordsWindowId.get(),
        FONT_NORMAL,
        (*(&raw const crate::data::strings::gText_OpenLv).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        8,
        97,
        TEXT_SKIP_DRAW,
        None,
    );
    PrintHyphens(10);
    PyramidPrintPrevOrCurrentStreak(FRONTIER_LVL_50, 64, 111, 49);
    PyramidPrintRecordStreak(FRONTIER_LVL_50, 64, 111, 65);
    PyramidPrintPrevOrCurrentStreak(FRONTIER_LVL_OPEN, 64, 111, 97);
    PyramidPrintRecordStreak(FRONTIER_LVL_OPEN, 64, 111, 113);
    PutWindowTilemap(gRecordsWindowId.get());
    CopyWindowToVram(gRecordsWindowId.get(), COPYWIN_FULL);
}
unsafe fn ShowLinkContestResultsWindow() {
    gRecordsWindowId
        .set(AddWindow((&raw const *sLinkContestResultsWindowTemplate).cast_mut()) as u8);
    DrawStdWindowFrame(gRecordsWindowId.get(), FALSE);
    FillWindowPixelBuffer(gRecordsWindowId.get(), 17);
    StringExpandPlaceholders(
        gStringVar4.as_mut_ptr(),
        (*(&raw const crate::data::strings::gText_LinkContestResults).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
    );
    let mut x: i32 = GetStringCenterAlignXOffset(FONT_NORMAL as i32, gStringVar4.as_mut_ptr(), 208);
    AddTextPrinterParameterized(
        gRecordsWindowId.get(),
        FONT_NORMAL,
        gStringVar4.as_mut_ptr(),
        x as u8,
        1,
        TEXT_SKIP_DRAW,
        None,
    );
    let mut str: *mut u8 = (*(&raw const crate::data::strings::gText_1st).cast::<CArray<u8, 0>>())
        .as_ptr()
        .cast_mut();
    x = GetStringRightAlignXOffset(FONT_NORMAL as i32, str, 38) + 50;
    AddTextPrinterParameterized(
        gRecordsWindowId.get(),
        FONT_NORMAL,
        str,
        x as u8,
        25,
        TEXT_SKIP_DRAW,
        None,
    );
    str = (*(&raw const crate::data::strings::gText_2nd).cast::<CArray<u8, 0>>())
        .as_ptr()
        .cast_mut();
    x = GetStringRightAlignXOffset(FONT_NORMAL as i32, str, 38) + 88;
    AddTextPrinterParameterized(
        gRecordsWindowId.get(),
        FONT_NORMAL,
        str,
        x as u8,
        25,
        TEXT_SKIP_DRAW,
        None,
    );
    str = (*(&raw const crate::data::strings::gText_3rd).cast::<CArray<u8, 0>>())
        .as_ptr()
        .cast_mut();
    x = GetStringRightAlignXOffset(FONT_NORMAL as i32, str, 38) + 126;
    AddTextPrinterParameterized(
        gRecordsWindowId.get(),
        FONT_NORMAL,
        str,
        x as u8,
        25,
        TEXT_SKIP_DRAW,
        None,
    );
    str = (*(&raw const crate::data::strings::gText_4th).cast::<CArray<u8, 0>>())
        .as_ptr()
        .cast_mut();
    x = GetStringRightAlignXOffset(FONT_NORMAL as i32, str, 38) + 164;
    AddTextPrinterParameterized(
        gRecordsWindowId.get(),
        FONT_NORMAL,
        str,
        x as u8,
        25,
        TEXT_SKIP_DRAW,
        None,
    );
    x = 6;
    AddTextPrinterParameterized(
        gRecordsWindowId.get(),
        FONT_NORMAL,
        (*(&raw const crate::data::strings::gText_Cool).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        x as u8,
        41,
        TEXT_SKIP_DRAW,
        None,
    );
    AddTextPrinterParameterized(
        gRecordsWindowId.get(),
        FONT_NORMAL,
        (*(&raw const crate::data::strings::gText_Beauty).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        x as u8,
        57,
        TEXT_SKIP_DRAW,
        None,
    );
    AddTextPrinterParameterized(
        gRecordsWindowId.get(),
        FONT_NORMAL,
        (*(&raw const crate::data::strings::gText_Cute).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        x as u8,
        73,
        TEXT_SKIP_DRAW,
        None,
    );
    AddTextPrinterParameterized(
        gRecordsWindowId.get(),
        FONT_NORMAL,
        (*(&raw const crate::data::strings::gText_Smart).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        x as u8,
        89,
        TEXT_SKIP_DRAW,
        None,
    );
    AddTextPrinterParameterized(
        gRecordsWindowId.get(),
        FONT_NORMAL,
        (*(&raw const crate::data::strings::gText_Tough).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        x as u8,
        105,
        TEXT_SKIP_DRAW,
        None,
    );
    for i in 0..CONTEST_CATEGORIES_COUNT {
        for j in 0..CONTESTANT_COUNT {
            ConvertIntToDecimalStringN(
                gStringVar4.as_mut_ptr(),
                (*gSaveBlock2Ptr).contestLinkResults[i][j] as i32,
                STR_CONV_MODE_RIGHT_ALIGN,
                4,
            );
            AddTextPrinterParameterized(
                gRecordsWindowId.get(),
                FONT_NORMAL,
                gStringVar4.as_mut_ptr(),
                j as u8 * 38 + 64,
                i as u8 * 16 + 41,
                TEXT_SKIP_DRAW,
                None,
            );
        }
    }
    PutWindowTilemap(gRecordsWindowId.get());
    CopyWindowToVram(gRecordsWindowId.get(), COPYWIN_FULL);
}
pub(crate) unsafe fn CheckPutFrontierTVShowOnAir() {
    let mut name: CArray<u8, 32> = zeroed();
    let lvlMode: i32 = (*gSaveBlock2Ptr).frontier.lvlMode() as i32;
    let facility: i32 = VarGet(VAR_FRONTIER_FACILITY) as i32;
    let battleMode: i32 = VarGet(VAR_FRONTIER_BATTLE_MODE) as i32;
    match facility {
        0 => {
            if (*gSaveBlock2Ptr).frontier.towerWinStreaks[battleMode][lvlMode]
                > (*gSaveBlock2Ptr).frontier.towerRecordWinStreaks[battleMode][lvlMode]
            {
                (*gSaveBlock2Ptr).frontier.towerRecordWinStreaks[battleMode][lvlMode] =
                    (*gSaveBlock2Ptr).frontier.towerWinStreaks[battleMode][lvlMode];
                if battleMode == FRONTIER_MODE_LINK_MULTIS as i32 {
                    StringCopy(
                        name.as_mut_ptr(),
                        gLinkPlayers[gBattleScripting.multiplayerId as i32 ^ 1]
                            .name
                            .as_mut_ptr(),
                    );
                    StripExtCtrlCodes(name.as_mut_ptr());
                    StringCopy(
                        (*gSaveBlock2Ptr).frontier.opponentNames[lvlMode].as_mut_ptr(),
                        name.as_mut_ptr(),
                    );
                    SetTrainerId(
                        gLinkPlayers[gBattleScripting.multiplayerId as i32 ^ 1].trainerId,
                        (*gSaveBlock2Ptr).frontier.opponentTrainerIds[lvlMode].as_mut_ptr(),
                    );
                }
                if (*gSaveBlock2Ptr).frontier.towerWinStreaks[battleMode][lvlMode] > 1
                    && ShouldAirFrontierTVShow() != 0
                {
                    match battleMode {
                        FRONTIER_MODE_SINGLES => {
                            TryPutFrontierTVShowOnAir(
                                (*gSaveBlock2Ptr).frontier.towerWinStreaks[battleMode][lvlMode],
                                FRONTIER_SHOW_TOWER_SINGLES,
                            );
                        }
                        1 => {
                            TryPutFrontierTVShowOnAir(
                                (*gSaveBlock2Ptr).frontier.towerWinStreaks[battleMode][lvlMode],
                                FRONTIER_SHOW_TOWER_DOUBLES,
                            );
                        }
                        2 => {
                            TryPutFrontierTVShowOnAir(
                                (*gSaveBlock2Ptr).frontier.towerWinStreaks[battleMode][lvlMode],
                                FRONTIER_SHOW_TOWER_MULTIS,
                            );
                        }
                        3 => {
                            TryPutFrontierTVShowOnAir(
                                (*gSaveBlock2Ptr).frontier.towerWinStreaks[battleMode][lvlMode],
                                FRONTIER_SHOW_TOWER_LINK_MULTIS,
                            );
                        }
                        _ => {}
                    }
                }
            }
        }
        FRONTIER_FACILITY_DOME => {
            if (*gSaveBlock2Ptr).frontier.domeWinStreaks[battleMode][lvlMode]
                > (*gSaveBlock2Ptr).frontier.domeRecordWinStreaks[battleMode][lvlMode]
            {
                (*gSaveBlock2Ptr).frontier.domeRecordWinStreaks[battleMode][lvlMode] =
                    (*gSaveBlock2Ptr).frontier.domeWinStreaks[battleMode][lvlMode];
                if (*gSaveBlock2Ptr).frontier.domeWinStreaks[battleMode][lvlMode] > 1
                    && ShouldAirFrontierTVShow() != 0
                {
                    if battleMode == FRONTIER_MODE_SINGLES {
                        TryPutFrontierTVShowOnAir(
                            (*gSaveBlock2Ptr).frontier.domeWinStreaks[battleMode][lvlMode],
                            FRONTIER_SHOW_DOME_SINGLES,
                        );
                    } else {
                        TryPutFrontierTVShowOnAir(
                            (*gSaveBlock2Ptr).frontier.domeWinStreaks[battleMode][lvlMode],
                            FRONTIER_SHOW_DOME_DOUBLES,
                        );
                    }
                }
            }
        }
        FRONTIER_FACILITY_PALACE => {
            if (*gSaveBlock2Ptr).frontier.palaceWinStreaks[battleMode][lvlMode]
                > (*gSaveBlock2Ptr).frontier.palaceRecordWinStreaks[battleMode][lvlMode]
            {
                (*gSaveBlock2Ptr).frontier.palaceRecordWinStreaks[battleMode][lvlMode] =
                    (*gSaveBlock2Ptr).frontier.palaceWinStreaks[battleMode][lvlMode];
                if (*gSaveBlock2Ptr).frontier.palaceWinStreaks[battleMode][lvlMode] > 1
                    && ShouldAirFrontierTVShow() != 0
                {
                    if battleMode == FRONTIER_MODE_SINGLES {
                        TryPutFrontierTVShowOnAir(
                            (*gSaveBlock2Ptr).frontier.palaceWinStreaks[battleMode][lvlMode],
                            FRONTIER_SHOW_PALACE_SINGLES,
                        );
                    } else {
                        TryPutFrontierTVShowOnAir(
                            (*gSaveBlock2Ptr).frontier.palaceWinStreaks[battleMode][lvlMode],
                            FRONTIER_SHOW_PALACE_DOUBLES,
                        );
                    }
                }
            }
        }
        FRONTIER_FACILITY_ARENA => {
            if (*gSaveBlock2Ptr).frontier.arenaWinStreaks[lvlMode]
                > (*gSaveBlock2Ptr).frontier.arenaRecordStreaks[lvlMode]
            {
                (*gSaveBlock2Ptr).frontier.arenaRecordStreaks[lvlMode] =
                    (*gSaveBlock2Ptr).frontier.arenaWinStreaks[lvlMode];
                if (*gSaveBlock2Ptr).frontier.arenaWinStreaks[lvlMode] > 1
                    && ShouldAirFrontierTVShow() != 0
                {
                    TryPutFrontierTVShowOnAir(
                        (*gSaveBlock2Ptr).frontier.arenaWinStreaks[lvlMode],
                        FRONTIER_SHOW_ARENA,
                    );
                }
            }
        }
        FRONTIER_FACILITY_FACTORY => {
            if (*gSaveBlock2Ptr).frontier.factoryWinStreaks[battleMode][lvlMode]
                > (*gSaveBlock2Ptr).frontier.factoryRecordWinStreaks[battleMode][lvlMode]
            {
                (*gSaveBlock2Ptr).frontier.factoryRecordWinStreaks[battleMode][lvlMode] =
                    (*gSaveBlock2Ptr).frontier.factoryWinStreaks[battleMode][lvlMode];
                (*gSaveBlock2Ptr).frontier.factoryRecordRentsCount[battleMode][lvlMode] =
                    (*gSaveBlock2Ptr).frontier.factoryRentsCount[battleMode][lvlMode];
                if (*gSaveBlock2Ptr).frontier.factoryWinStreaks[battleMode][lvlMode] > 1
                    && ShouldAirFrontierTVShow() != 0
                {
                    if battleMode == FRONTIER_MODE_SINGLES {
                        TryPutFrontierTVShowOnAir(
                            (*gSaveBlock2Ptr).frontier.factoryWinStreaks[battleMode][lvlMode],
                            FRONTIER_SHOW_FACTORY_SINGLES,
                        );
                    } else {
                        TryPutFrontierTVShowOnAir(
                            (*gSaveBlock2Ptr).frontier.factoryWinStreaks[battleMode][lvlMode],
                            FRONTIER_SHOW_FACTORY_DOUBLES,
                        );
                    }
                }
            }
        }
        FRONTIER_FACILITY_PIKE => {
            if (*gSaveBlock2Ptr).frontier.pikeWinStreaks[lvlMode]
                > (*gSaveBlock2Ptr).frontier.pikeRecordStreaks[lvlMode]
            {
                (*gSaveBlock2Ptr).frontier.pikeRecordStreaks[lvlMode] =
                    (*gSaveBlock2Ptr).frontier.pikeWinStreaks[lvlMode];
                if (*gSaveBlock2Ptr).frontier.pikeWinStreaks[lvlMode] > 1
                    && ShouldAirFrontierTVShow() != 0
                {
                    TryPutFrontierTVShowOnAir(
                        (*gSaveBlock2Ptr).frontier.pikeWinStreaks[lvlMode],
                        FRONTIER_SHOW_PIKE,
                    );
                }
            }
        }
        FRONTIER_FACILITY_PYRAMID
            if (*gSaveBlock2Ptr).frontier.pyramidWinStreaks[lvlMode]
                > (*gSaveBlock2Ptr).frontier.pyramidRecordStreaks[lvlMode] =>
        {
            (*gSaveBlock2Ptr).frontier.pyramidRecordStreaks[lvlMode] =
                (*gSaveBlock2Ptr).frontier.pyramidWinStreaks[lvlMode];
            if (*gSaveBlock2Ptr).frontier.pyramidWinStreaks[lvlMode] > 1
                && ShouldAirFrontierTVShow() != 0
            {
                TryPutFrontierTVShowOnAir(
                    (*gSaveBlock2Ptr).frontier.pyramidWinStreaks[lvlMode],
                    FRONTIER_SHOW_PYRAMID,
                );
            }
        }
        _ => {}
    }
}
pub(crate) unsafe fn Script_GetFrontierBrainStatus() {
    VarGet(VAR_FRONTIER_FACILITY);
    gSpecialVar_Result = GetFrontierBrainStatus() as u16;
}
pub unsafe fn GetFrontierBrainStatus() -> u8 {
    let mut status: i32 = FRONTIER_BRAIN_NOT_READY as i32;
    let facility: i32 = VarGet(VAR_FRONTIER_FACILITY) as i32;
    let battleMode: i32 = VarGet(VAR_FRONTIER_BATTLE_MODE) as i32;
    let winStreakNoModifier: u16 = GetCurrentFacilityWinStreak() as u16;
    let winStreak: i32 =
        winStreakNoModifier as i32 + sFrontierBrainStreakAppearances[facility][3] as i32;
    if battleMode != FRONTIER_MODE_SINGLES {
        return FRONTIER_BRAIN_NOT_READY;
    }
    let symbolsCount: i32 = GetPlayerSymbolCountForFacility(facility as u8) as i32;
    match symbolsCount {
        0 | 1 => {
            if winStreak == sFrontierBrainStreakAppearances[facility][symbolsCount] as i32 {
                status = symbolsCount + 1;
            }
        }
        _ => {
            if winStreak == sFrontierBrainStreakAppearances[facility][0] as i32 {
                status = FRONTIER_BRAIN_STREAK as i32;
            } else if winStreak == sFrontierBrainStreakAppearances[facility][1] as i32 {
                status = FRONTIER_BRAIN_STREAK_LONG;
            } else if winStreak > sFrontierBrainStreakAppearances[facility][1] as i32
                && rem_i32(
                    winStreak - sFrontierBrainStreakAppearances[facility][1] as i32,
                    sFrontierBrainStreakAppearances[facility][2] as i32,
                ) == 0
            {
                status = FRONTIER_BRAIN_STREAK_LONG;
            }
        }
    }
    status as u8
}
pub unsafe fn CopyFrontierTrainerText(whichText: u8, mut trainerId: u16) {
    match whichText {
        FRONTIER_BEFORE_TEXT => {
            if trainerId == TRAINER_EREADER {
                FrontierSpeechToString(
                    (*gSaveBlock2Ptr)
                        .frontier
                        .ereaderTrainer
                        .greeting
                        .as_mut_ptr(),
                );
            } else if trainerId == TRAINER_FRONTIER_BRAIN {
                CopyFrontierBrainText(FALSE);
            } else if trainerId < FRONTIER_TRAINERS_COUNT {
                FrontierSpeechToString(
                    (*gFacilityTrainers.at(trainerId)).speechBefore.as_mut_ptr(),
                );
            } else if trainerId < TRAINER_RECORD_MIXING_APPRENTICE as u16 {
                FrontierSpeechToString(
                    (*gSaveBlock2Ptr).frontier.towerRecords
                        [trainerId as i32 - TRAINER_RECORD_MIXING_FRIEND]
                        .greeting
                        .as_mut_ptr(),
                );
            } else {
                BufferApprenticeChallengeText(
                    trainerId as u8 - TRAINER_RECORD_MIXING_APPRENTICE as u8,
                );
            }
        }
        FRONTIER_PLAYER_LOST_TEXT => {
            if trainerId == TRAINER_EREADER {
                FrontierSpeechToString(
                    (*gSaveBlock2Ptr)
                        .frontier
                        .ereaderTrainer
                        .farewellPlayerLost
                        .as_mut_ptr(),
                );
            } else if trainerId == TRAINER_FRONTIER_BRAIN {
                CopyFrontierBrainText(FALSE);
            } else if trainerId < FRONTIER_TRAINERS_COUNT {
                FrontierSpeechToString((*gFacilityTrainers.at(trainerId)).speechWin.as_mut_ptr());
            } else if trainerId < TRAINER_RECORD_MIXING_APPRENTICE as u16 {
                if gBattleTypeFlags & BATTLE_TYPE_RECORDED != 0 {
                    FrontierSpeechToString(GetRecordedBattleEasyChatSpeech());
                } else {
                    FrontierSpeechToString(
                        (*gSaveBlock2Ptr).frontier.towerRecords
                            [trainerId as i32 - TRAINER_RECORD_MIXING_FRIEND]
                            .speechWon
                            .as_mut_ptr(),
                    );
                }
            } else {
                if gBattleTypeFlags & BATTLE_TYPE_RECORDED != 0 {
                    FrontierSpeechToString(GetRecordedBattleEasyChatSpeech());
                } else {
                    FrontierSpeechToString(
                        (*gSaveBlock2Ptr).apprentices
                            [trainerId as i32 - TRAINER_RECORD_MIXING_APPRENTICE]
                            .speechWon
                            .as_mut_ptr(),
                    );
                }
            }
        }
        FRONTIER_PLAYER_WON_TEXT => {
            if trainerId == TRAINER_EREADER {
                FrontierSpeechToString(
                    (*gSaveBlock2Ptr)
                        .frontier
                        .ereaderTrainer
                        .farewellPlayerWon
                        .as_mut_ptr(),
                );
            } else if trainerId == TRAINER_FRONTIER_BRAIN {
                CopyFrontierBrainText(TRUE);
            } else if trainerId < FRONTIER_TRAINERS_COUNT {
                FrontierSpeechToString((*gFacilityTrainers.at(trainerId)).speechLose.as_mut_ptr());
            } else if trainerId < TRAINER_RECORD_MIXING_APPRENTICE as u16 {
                if gBattleTypeFlags & BATTLE_TYPE_RECORDED != 0 {
                    FrontierSpeechToString(GetRecordedBattleEasyChatSpeech());
                } else {
                    FrontierSpeechToString(
                        (*gSaveBlock2Ptr).frontier.towerRecords
                            [trainerId as i32 - TRAINER_RECORD_MIXING_FRIEND]
                            .speechLost
                            .as_mut_ptr(),
                    );
                }
            } else {
                if gBattleTypeFlags & BATTLE_TYPE_RECORDED != 0 {
                    trainerId = GetRecordedBattleApprenticeId() as u16;
                    FrontierSpeechToString(
                        (*(&raw const crate::data::apprentice::gApprentices).cast::<CArray<
                            ApprenticeTrainer,
                            0,
                        >>(
                        ))[trainerId]
                            .speechLost
                            .as_ptr()
                            .cast_mut(),
                    );
                } else {
                    trainerId = (*gSaveBlock2Ptr).apprentices
                        [trainerId as i32 - TRAINER_RECORD_MIXING_APPRENTICE]
                        .id() as u16;
                    FrontierSpeechToString(
                        (*(&raw const crate::data::apprentice::gApprentices).cast::<CArray<
                            ApprenticeTrainer,
                            0,
                        >>(
                        ))[trainerId]
                            .speechLost
                            .as_ptr()
                            .cast_mut(),
                    );
                }
            }
        }
        _ => {}
    }
}
pub unsafe fn ResetWinStreaks() {
    (*gSaveBlock2Ptr).frontier.winStreakActiveFlags = 0;
    for battleMode in 0..(FRONTIER_MODE_COUNT as i32) {
        for lvlMode in 0..(FRONTIER_LVL_TENT as i32) {
            (*gSaveBlock2Ptr).frontier.towerWinStreaks[battleMode][lvlMode] = 0;
            if battleMode < FRONTIER_MODE_MULTIS as i32 {
                (*gSaveBlock2Ptr).frontier.domeWinStreaks[battleMode][lvlMode] = 0;
                (*gSaveBlock2Ptr).frontier.palaceWinStreaks[battleMode][lvlMode] = 0;
                (*gSaveBlock2Ptr).frontier.factoryWinStreaks[battleMode][lvlMode] = 0;
            }
            if battleMode == FRONTIER_MODE_SINGLES {
                (*gSaveBlock2Ptr).frontier.arenaWinStreaks[lvlMode] = 0;
                (*gSaveBlock2Ptr).frontier.pikeWinStreaks[lvlMode] = 0;
                (*gSaveBlock2Ptr).frontier.pyramidWinStreaks[lvlMode] = 0;
            }
        }
    }
    if (*gSaveBlock2Ptr).frontier.challengeStatus != 0 {
        (*gSaveBlock2Ptr).frontier.challengeStatus = CHALLENGE_STATUS_SAVING;
    }
}
pub unsafe fn GetCurrentFacilityWinStreak() -> u32 {
    let lvlMode: i32 = (*gSaveBlock2Ptr).frontier.lvlMode() as i32;
    let battleMode: i32 = VarGet(VAR_FRONTIER_BATTLE_MODE) as i32;
    let facility: i32 = VarGet(VAR_FRONTIER_FACILITY) as i32;
    match facility {
        0 => {
            return (*gSaveBlock2Ptr).frontier.towerWinStreaks[battleMode][lvlMode] as u32;
        }
        FRONTIER_FACILITY_DOME => {
            return (*gSaveBlock2Ptr).frontier.domeWinStreaks[battleMode][lvlMode] as u32;
        }
        FRONTIER_FACILITY_PALACE => {
            return (*gSaveBlock2Ptr).frontier.palaceWinStreaks[battleMode][lvlMode] as u32;
        }
        FRONTIER_FACILITY_ARENA => {
            return (*gSaveBlock2Ptr).frontier.arenaWinStreaks[lvlMode] as u32;
        }
        FRONTIER_FACILITY_FACTORY => {
            return (*gSaveBlock2Ptr).frontier.factoryWinStreaks[battleMode][lvlMode] as u32;
        }
        FRONTIER_FACILITY_PIKE => {
            return (*gSaveBlock2Ptr).frontier.pikeWinStreaks[lvlMode] as u32;
        }
        FRONTIER_FACILITY_PYRAMID => {
            return (*gSaveBlock2Ptr).frontier.pyramidWinStreaks[lvlMode] as u32;
        }
        _ => {
            return 0;
        }
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
pub unsafe fn ResetFrontierTrainerIds() {
    for i in 0..20i32 {
        (*gSaveBlock2Ptr).frontier.trainerIds[i] = 0xFFFF;
    }
}
pub(crate) unsafe fn IsTrainerFrontierBrain() {
    if gTrainerBattleOpponent_A == TRAINER_FRONTIER_BRAIN {
        gSpecialVar_Result = TRUE as u16;
    } else {
        gSpecialVar_Result = FALSE as u16;
    }
}
pub unsafe fn GetPlayerSymbolCountForFacility(facility: u8) -> u8 {
    FlagGet(FLAG_SYS_TOWER_SILVER + facility as u16 * 2)
        + FlagGet(FLAG_SYS_TOWER_GOLD + facility as u16 * 2)
}
pub(crate) unsafe fn GiveBattlePoints() {
    let mut challengeNum: i32 = 0;
    let lvlMode: i32 = (*gSaveBlock2Ptr).frontier.lvlMode() as i32;
    let facility: i32 = VarGet(VAR_FRONTIER_FACILITY) as i32;
    let battleMode: i32 = VarGet(VAR_FRONTIER_BATTLE_MODE) as i32;
    match facility {
        0 => {
            challengeNum =
                (*gSaveBlock2Ptr).frontier.towerWinStreaks[battleMode][lvlMode] as i32 / 7;
        }
        FRONTIER_FACILITY_DOME => {
            challengeNum = (*gSaveBlock2Ptr).frontier.domeWinStreaks[battleMode][lvlMode] as i32;
        }
        FRONTIER_FACILITY_PALACE => {
            challengeNum =
                (*gSaveBlock2Ptr).frontier.palaceWinStreaks[battleMode][lvlMode] as i32 / 7;
        }
        FRONTIER_FACILITY_ARENA => {
            challengeNum = (*gSaveBlock2Ptr).frontier.arenaWinStreaks[lvlMode] as i32 / 7;
        }
        FRONTIER_FACILITY_FACTORY => {
            challengeNum =
                (*gSaveBlock2Ptr).frontier.factoryWinStreaks[battleMode][lvlMode] as i32 / 7;
        }
        FRONTIER_FACILITY_PIKE => {
            challengeNum = (*gSaveBlock2Ptr).frontier.pikeWinStreaks[lvlMode] as i32 / 14;
        }
        FRONTIER_FACILITY_PYRAMID => {
            challengeNum = (*gSaveBlock2Ptr).frontier.pyramidWinStreaks[lvlMode] as i32 / 7;
        }
        _ => {}
    }
    if challengeNum != 0 {
        challengeNum -= 1;
    }
    if challengeNum >= 30 {
        challengeNum = 29;
    }
    let mut points: i32 = sBattlePointAwards[challengeNum][facility][battleMode] as i32;
    if gTrainerBattleOpponent_A == TRAINER_FRONTIER_BRAIN {
        points += 10;
    }
    (*gSaveBlock2Ptr).frontier.battlePoints += points as u16;
    ConvertIntToDecimalStringN(
        gStringVar1.as_mut_ptr(),
        points,
        STR_CONV_MODE_LEFT_ALIGN,
        2,
    );
    if (*gSaveBlock2Ptr).frontier.battlePoints > MAX_BATTLE_FRONTIER_POINTS {
        (*gSaveBlock2Ptr).frontier.battlePoints = MAX_BATTLE_FRONTIER_POINTS;
    }
    points = (*gSaveBlock2Ptr).frontier.cardBattlePoints as i32;
    points += sBattlePointAwards[challengeNum][facility][battleMode] as i32;
    IncrementDailyBattlePoints(sBattlePointAwards[challengeNum][facility][battleMode] as u16);
    if gTrainerBattleOpponent_A == TRAINER_FRONTIER_BRAIN {
        points += 10;
        IncrementDailyBattlePoints(10);
    }
    if points > 0xFFFF {
        points = 0xFFFF;
    }
    (*gSaveBlock2Ptr).frontier.cardBattlePoints = points as u16;
}
pub(crate) unsafe fn GetFacilitySymbolCount() {
    let facility: i32 = VarGet(VAR_FRONTIER_FACILITY) as i32;
    gSpecialVar_Result = GetPlayerSymbolCountForFacility(facility as u8) as u16;
}
pub(crate) unsafe fn GiveFacilitySymbol() {
    let facility: i32 = VarGet(VAR_FRONTIER_FACILITY) as i32;
    if GetPlayerSymbolCountForFacility(facility as u8) == 0 {
        FlagSet(FLAG_SYS_TOWER_SILVER + facility as u16 * 2);
    } else {
        FlagSet(FLAG_SYS_TOWER_GOLD + facility as u16 * 2);
    }
}
pub(crate) unsafe fn CheckBattleTypeFlag() {
    if gBattleTypeFlags & gSpecialVar_0x8005 as u32 != 0 {
        gSpecialVar_Result = TRUE as u16;
    } else {
        gSpecialVar_Result = FALSE as u16;
    }
}
unsafe fn AppendCaughtBannedMonSpeciesName(
    species: u16,
    mut count: u8,
    numBannedMonsCaught: i32,
) -> u8 {
    if GetSetPokedexFlag(SpeciesToNationalPokedexNum(species), FLAG_GET_CAUGHT) != 0 {
        count += 1;
        match count {
            1 | 3 | 5 | 7 | 9 | 11 => {
                if numBannedMonsCaught == count as i32 {
                    StringAppend(
                        gStringVar1.as_mut_ptr(),
                        (*(&raw const crate::data::battle_message::gText_SpaceAndSpace)
                            .cast::<CArray<u8, 0>>())
                        .as_ptr()
                        .cast_mut(),
                    );
                } else if numBannedMonsCaught > count as i32 {
                    StringAppend(
                        gStringVar1.as_mut_ptr(),
                        (*(&raw const crate::data::battle_message::gText_CommaSpace)
                            .cast::<CArray<u8, 0>>())
                        .as_ptr()
                        .cast_mut(),
                    );
                }
            }
            2 => {
                if count as i32 == numBannedMonsCaught {
                    StringAppend(
                        gStringVar1.as_mut_ptr(),
                        (*(&raw const crate::data::battle_message::gText_SpaceAndSpace)
                            .cast::<CArray<u8, 0>>())
                        .as_ptr()
                        .cast_mut(),
                    );
                } else {
                    StringAppend(
                        gStringVar1.as_mut_ptr(),
                        (*(&raw const crate::data::battle_message::gText_CommaSpace)
                            .cast::<CArray<u8, 0>>())
                        .as_ptr()
                        .cast_mut(),
                    );
                }
                StringAppend(
                    gStringVar1.as_mut_ptr(),
                    (*(&raw const crate::data::battle_message::gText_NewLine)
                        .cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut(),
                );
            }
            _ => {
                if count as i32 == numBannedMonsCaught {
                    StringAppend(
                        gStringVar1.as_mut_ptr(),
                        (*(&raw const crate::data::battle_message::gText_SpaceAndSpace)
                            .cast::<CArray<u8, 0>>())
                        .as_ptr()
                        .cast_mut(),
                    );
                } else {
                    StringAppend(
                        gStringVar1.as_mut_ptr(),
                        (*(&raw const crate::data::battle_message::gText_CommaSpace)
                            .cast::<CArray<u8, 0>>())
                        .as_ptr()
                        .cast_mut(),
                    );
                }
                StringAppend(
                    gStringVar1.as_mut_ptr(),
                    (*(&raw const crate::data::battle_message::gText_LineBreak)
                        .cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut(),
                );
            }
        }
        StringAppend(
            gStringVar1.as_mut_ptr(),
            (*(&raw const crate::data::data_tables::gSpeciesNames)
                .cast::<CArray<CArray<u8, 11>, 0>>())[species]
                .as_ptr()
                .cast_mut(),
        );
    }
    count
}
unsafe fn AppendIfValid(
    species: u16,
    heldItem: u16,
    hp: u16,
    lvlMode: u8,
    monLevel: u8,
    speciesArray: *mut u16,
    itemsArray: *mut u16,
    count: *mut u8,
) {
    if species == SPECIES_EGG as u16 || species == SPECIES_NONE {
        return;
    }
    let mut i: i32 = 0;
    while gFrontierBannedSpecies[i] != 0xFFFF && gFrontierBannedSpecies[i] != species {
        i += 1;
    }
    if gFrontierBannedSpecies[i] != 0xFFFF {
        return;
    }
    if lvlMode == FRONTIER_LVL_50 && monLevel > FRONTIER_MAX_LEVEL_50 {
        return;
    }
    i = 0;
    while i < *count as i32 && *speciesArray.at(i) != species {
        i += 1;
    }
    if i != *count as i32 {
        return;
    }
    if heldItem != 0 {
        i = 0;
        while i < *count as i32 && *itemsArray.at(i) != heldItem {
            i += 1;
        }
        if i != *count as i32 {
            return;
        }
    }
    *speciesArray.at(*count) = species;
    *itemsArray.at(*count) = heldItem;
    *count += 1;
}
pub(crate) unsafe fn CheckPartyIneligibility() {
    let mut speciesArray: CArray<u16, 6> = zeroed();
    let mut itemArray: CArray<u16, 6> = zeroed();
    let mut monId: i32 = 0;
    let mut toChoose: i32 = 0;
    let mut count: u8 = 0;
    let battleMode: i32 = VarGet(VAR_FRONTIER_BATTLE_MODE) as i32;
    match battleMode {
        FRONTIER_MODE_SINGLES => {
            toChoose = FRONTIER_PARTY_SIZE;
        }
        2 | 3 => {
            toChoose = FRONTIER_MULTI_PARTY_SIZE;
        }
        1 => {
            if VarGet(VAR_FRONTIER_FACILITY) == FRONTIER_FACILITY_TOWER {
                toChoose = FRONTIER_DOUBLES_PARTY_SIZE as i32;
            } else {
                toChoose = FRONTIER_PARTY_SIZE;
            }
        }
        _ => {}
    }
    let mut monIdLooper: i32 = 0;
    loop {
        monId = monIdLooper;
        count = 0;
        loop {
            let species: u16 =
                GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_SPECIES_OR_EGG) as u16;
            let heldItem: u16 =
                GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_HELD_ITEM) as u16;
            let level: u8 = GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_LEVEL) as u8;
            let hp: u16 = GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_HP) as u16;
            if VarGet(VAR_FRONTIER_FACILITY) == FRONTIER_FACILITY_PYRAMID as u16 {
                if heldItem == ITEM_NONE {
                    AppendIfValid(
                        species,
                        heldItem,
                        hp,
                        gSpecialVar_Result as u8,
                        level,
                        speciesArray.as_mut_ptr(),
                        itemArray.as_mut_ptr(),
                        &raw mut count,
                    );
                }
            } else {
                AppendIfValid(
                    species,
                    heldItem,
                    hp,
                    gSpecialVar_Result as u8,
                    level,
                    speciesArray.as_mut_ptr(),
                    itemArray.as_mut_ptr(),
                    &raw mut count,
                );
            }
            monId += 1;
            if monId >= PARTY_SIZE {
                monId = 0;
            }
            if monId == monIdLooper {
                break;
            }
        }
        monIdLooper += 1;
        if !(monIdLooper < PARTY_SIZE && (count as i32) < toChoose) {
            break;
        }
    }
    if (count as i32) < toChoose {
        let mut caughtBannedMons: i32 = 0;
        let mut species: i32 = gFrontierBannedSpecies[0] as i32;
        let mut i: i32 = 0;
        while species != 0xFFFF {
            if GetSetPokedexFlag(SpeciesToNationalPokedexNum(species as u16), FLAG_GET_CAUGHT) != 0
            {
                caughtBannedMons += 1;
            }
            i += 1;
            species = gFrontierBannedSpecies[i] as i32;
        }
        gStringVar1[0] = EOS;
        gSpecialVar_0x8004 = TRUE as u16;
        count = 0;
        i = 0;
        while gFrontierBannedSpecies[i] != 0xFFFF {
            count = AppendCaughtBannedMonSpeciesName(
                gFrontierBannedSpecies[i],
                count,
                caughtBannedMons,
            );
            i += 1;
        }
        if count == 0 {
            StringAppend(
                gStringVar1.as_mut_ptr(),
                (*(&raw const crate::data::battle_message::gText_Space2).cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut(),
            );
            StringAppend(
                gStringVar1.as_mut_ptr(),
                (*(&raw const crate::data::battle_message::gText_Are).cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut(),
            );
        } else {
            if count as i32 & 1 != 0 {
                StringAppend(
                    gStringVar1.as_mut_ptr(),
                    (*(&raw const crate::data::battle_message::gText_LineBreak)
                        .cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut(),
                );
            } else {
                StringAppend(
                    gStringVar1.as_mut_ptr(),
                    (*(&raw const crate::data::battle_message::gText_Space2)
                        .cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut(),
                );
            }
            StringAppend(
                gStringVar1.as_mut_ptr(),
                (*(&raw const crate::data::battle_message::gText_Are2).cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut(),
            );
        }
    } else {
        gSpecialVar_0x8004 = FALSE as u16;
        (*gSaveBlock2Ptr)
            .frontier
            .set_lvlMode(gSpecialVar_Result as u8);
    }
}
pub(crate) unsafe fn ValidateVisitingTrainer() {
    ValidateEReaderTrainer();
}
pub(crate) unsafe fn IncrementWinStreak() {
    let lvlMode: i32 = (*gSaveBlock2Ptr).frontier.lvlMode() as i32;
    let battleMode: i32 = VarGet(VAR_FRONTIER_BATTLE_MODE) as i32;
    let facility: i32 = VarGet(VAR_FRONTIER_FACILITY) as i32;
    match facility {
        0 => {
            if (*gSaveBlock2Ptr).frontier.towerWinStreaks[battleMode][lvlMode] < MAX_STREAK {
                (*gSaveBlock2Ptr).frontier.towerWinStreaks[battleMode][lvlMode] += 1;
                if battleMode == FRONTIER_MODE_SINGLES {
                    SetGameStat(
                        GAME_STAT_BATTLE_TOWER_SINGLES_STREAK,
                        (*gSaveBlock2Ptr).frontier.towerWinStreaks[battleMode][lvlMode] as u32,
                    );
                    (*gSaveBlock2Ptr).frontier.towerSinglesStreak =
                        (*gSaveBlock2Ptr).frontier.towerWinStreaks[battleMode][lvlMode];
                }
            }
        }
        FRONTIER_FACILITY_DOME => {
            if (*gSaveBlock2Ptr).frontier.domeWinStreaks[battleMode][lvlMode] < MAX_STREAK {
                (*gSaveBlock2Ptr).frontier.domeWinStreaks[battleMode][lvlMode] += 1;
            }
            if (*gSaveBlock2Ptr).frontier.domeTotalChampionships[battleMode][lvlMode] < MAX_STREAK {
                (*gSaveBlock2Ptr).frontier.domeTotalChampionships[battleMode][lvlMode] += 1;
            }
        }
        FRONTIER_FACILITY_PALACE => {
            if (*gSaveBlock2Ptr).frontier.palaceWinStreaks[battleMode][lvlMode] < MAX_STREAK {
                (*gSaveBlock2Ptr).frontier.palaceWinStreaks[battleMode][lvlMode] += 1;
            }
        }
        FRONTIER_FACILITY_ARENA => {
            if (*gSaveBlock2Ptr).frontier.arenaWinStreaks[lvlMode] < MAX_STREAK {
                (*gSaveBlock2Ptr).frontier.arenaWinStreaks[lvlMode] += 1;
            }
        }
        FRONTIER_FACILITY_FACTORY => {
            if (*gSaveBlock2Ptr).frontier.factoryWinStreaks[battleMode][lvlMode] < MAX_STREAK {
                (*gSaveBlock2Ptr).frontier.factoryWinStreaks[battleMode][lvlMode] += 1;
            }
        }
        FRONTIER_FACILITY_PIKE => {
            if (*gSaveBlock2Ptr).frontier.pikeWinStreaks[lvlMode] < MAX_STREAK {
                (*gSaveBlock2Ptr).frontier.pikeWinStreaks[lvlMode] += 1;
            }
        }
        FRONTIER_FACILITY_PYRAMID
            if (*gSaveBlock2Ptr).frontier.pyramidWinStreaks[lvlMode] < MAX_STREAK =>
        {
            (*gSaveBlock2Ptr).frontier.pyramidWinStreaks[lvlMode] += 1;
        }
        _ => {}
    }
}
pub(crate) unsafe fn RestoreHeldItems() {
    let mut i: u8 = 0;
    while (i as i32)
        < (if 3 >= (if 4 >= 2 { 4 } else { 2 }) {
            3
        } else {
            if 4 >= 2 { 4 } else { 2 }
        })
    {
        if (*gSaveBlock2Ptr).frontier.selectedPartyMons[i] != 0 {
            let mut item: u16 = GetMonData3(
                &raw mut (*gSaveBlock1Ptr).playerParty
                    [(*gSaveBlock2Ptr).frontier.selectedPartyMons[i] as i32 - 1],
                MON_DATA_HELD_ITEM,
                null_mut(),
            ) as u16;
            SetMonData(
                &raw mut gPlayerParty[i],
                MON_DATA_HELD_ITEM,
                &raw mut item as *mut c_void,
            );
        }
        i += 1;
    }
}
pub(crate) unsafe fn SaveRecordBattle() {
    gSpecialVar_Result = MoveRecordedBattleToSaveData() as u16;
    (*gSaveBlock2Ptr).frontier.set_disableRecordBattle(TRUE);
}
pub(crate) unsafe fn BufferFrontierTrainerName() {
    match *(&raw const crate::ffi::gSpecialVar_0x8005)
        .cast::<u16>()
        .cast_mut()
    {
        0 => {
            GetFrontierTrainerName(gStringVar1.as_mut_ptr(), gTrainerBattleOpponent_A);
        }
        1 => {
            GetFrontierTrainerName(gStringVar2.as_mut_ptr(), gTrainerBattleOpponent_A);
        }
        _ => {}
    }
}
pub(crate) unsafe fn ResetSketchedMoves() {
    let mut k: u8 = 0;
    let mut i: u8 = 0;
    while (i as i32)
        < (if 3 >= (if 4 >= 2 { 4 } else { 2 }) {
            3
        } else {
            if 4 >= 2 { 4 } else { 2 }
        })
    {
        let monId: u16 = (*gSaveBlock2Ptr).frontier.selectedPartyMons[i] - 1;
        if monId < PARTY_SIZE as u16 {
            for j in 0..(MAX_MON_MOVES as u8) {
                k = 0;
                while k < MAX_MON_MOVES as u8 {
                    if GetMonData3(
                        &raw mut (*gSaveBlock1Ptr).playerParty
                            [(*gSaveBlock2Ptr).frontier.selectedPartyMons[i] as i32 - 1],
                        MON_DATA_MOVE1 + k as i32,
                        null_mut(),
                    ) == GetMonData3(
                        &raw mut gPlayerParty[i],
                        MON_DATA_MOVE1 + j as i32,
                        null_mut(),
                    ) {
                        break;
                    }
                    k += 1;
                }
                if k == MAX_MON_MOVES as u8 {
                    SetMonMoveSlot(&raw mut gPlayerParty[i], MOVE_SKETCH, j);
                }
            }
            (*gSaveBlock1Ptr).playerParty
                [(*gSaveBlock2Ptr).frontier.selectedPartyMons[i] as i32 - 1] = gPlayerParty[i];
        }
        i += 1;
    }
}
pub(crate) unsafe fn SetFacilityBrainObjectEvent() {
    SetFrontierBrainObjEventGfx(VarGet(VAR_FRONTIER_FACILITY) as u8);
}
unsafe fn Print1PRecord(
    position: i32,
    x: i32,
    y: i32,
    hallRecord: *mut RankingHall1P,
    hallFacilityId: i32,
) {
    let mut text: CArray<u8, 32> = zeroed();
    let mut winStreak: u16 = 0;
    AddTextPrinterParameterized(
        gRecordsWindowId.get(),
        FONT_NORMAL,
        (*(&raw const crate::data::strings::gText_123Dot).cast::<CArray<CArray<u8, 3>, 0>>())
            [position]
            .as_ptr()
            .cast_mut(),
        x as u8 * 8,
        8 * (y as u8 + 5 * position as u8) + 1,
        TEXT_SKIP_DRAW,
        None,
    );
    (*hallRecord).name[7] = EOS;
    if (*hallRecord).winStreak != 0 {
        TVShowConvertInternationalString(
            text.as_mut_ptr(),
            (*hallRecord).name.as_mut_ptr(),
            (*hallRecord).language as i32,
        );
        AddTextPrinterParameterized(
            gRecordsWindowId.get(),
            FONT_NORMAL,
            text.as_mut_ptr(),
            (x as u8 + 2) * 8,
            8 * (y as u8 + 5 * position as u8) + 1,
            TEXT_SKIP_DRAW,
            None,
        );
        winStreak = (*hallRecord).winStreak;
        if winStreak > MAX_STREAK {
            winStreak = MAX_STREAK;
        }
        ConvertIntToDecimalStringN(
            gStringVar2.as_mut_ptr(),
            winStreak as i32,
            STR_CONV_MODE_RIGHT_ALIGN,
            4,
        );
        StringExpandPlaceholders(
            gStringVar4.as_mut_ptr(),
            sHallFacilityToRecordsText[hallFacilityId],
        );
        AddTextPrinterParameterized(
            gRecordsWindowId.get(),
            FONT_NORMAL,
            gStringVar4.as_mut_ptr(),
            GetStringRightAlignXOffset(
                FONT_NORMAL as i32,
                sHallFacilityToRecordsText[hallFacilityId],
                0xC8,
            ) as u8,
            8 * (y as u8 + 5 * position as u8) + 1,
            TEXT_SKIP_DRAW,
            None,
        );
    }
}
unsafe fn Print2PRecord(position: i32, x: i32, y: i32, hallRecord: *mut RankingHall2P) {
    let mut text: CArray<u8, 32> = zeroed();
    let mut winStreak: u16 = 0;
    AddTextPrinterParameterized(
        gRecordsWindowId.get(),
        FONT_NORMAL,
        (*(&raw const crate::data::strings::gText_123Dot).cast::<CArray<CArray<u8, 3>, 0>>())
            [position]
            .as_ptr()
            .cast_mut(),
        x as u8 * 8,
        8 * (y as u8 + 5 * position as u8) + 1,
        TEXT_SKIP_DRAW,
        None,
    );
    if (*hallRecord).winStreak != 0 {
        (*hallRecord).name1[7] = EOS;
        (*hallRecord).name2[7] = EOS;
        TVShowConvertInternationalString(
            text.as_mut_ptr(),
            (*hallRecord).name1.as_mut_ptr(),
            (*hallRecord).language as i32,
        );
        AddTextPrinterParameterized(
            gRecordsWindowId.get(),
            FONT_NORMAL,
            text.as_mut_ptr(),
            (x as u8 + 2) * 8,
            8 * (y as u8 + 5 * position as u8 - 1) + 1,
            TEXT_SKIP_DRAW,
            None,
        );
        if IsStringJapanese((*hallRecord).name2.as_mut_ptr()) != 0 {
            TVShowConvertInternationalString(
                text.as_mut_ptr(),
                (*hallRecord).name2.as_mut_ptr(),
                LANGUAGE_JAPANESE as i32,
            );
        } else {
            StringCopy(text.as_mut_ptr(), (*hallRecord).name2.as_mut_ptr());
        }
        AddTextPrinterParameterized(
            gRecordsWindowId.get(),
            FONT_NORMAL,
            text.as_mut_ptr(),
            (x as u8 + 4) * 8,
            8 * (y as u8 + 5 * position as u8 + 1) + 1,
            TEXT_SKIP_DRAW,
            None,
        );
        winStreak = (*hallRecord).winStreak;
        if winStreak > MAX_STREAK {
            winStreak = MAX_STREAK;
        }
        ConvertIntToDecimalStringN(
            gStringVar2.as_mut_ptr(),
            winStreak as i32,
            STR_CONV_MODE_RIGHT_ALIGN,
            4,
        );
        StringExpandPlaceholders(gStringVar4.as_mut_ptr(), sHallFacilityToRecordsText[9]);
        AddTextPrinterParameterized(
            gRecordsWindowId.get(),
            FONT_NORMAL,
            gStringVar4.as_mut_ptr(),
            GetStringRightAlignXOffset(FONT_NORMAL as i32, sHallFacilityToRecordsText[9], 0xC8)
                as u8,
            8 * (y as u8 + 5 * position as u8) + 1,
            TEXT_SKIP_DRAW,
            None,
        );
    }
}
unsafe fn Fill1PRecords(dst: *mut RankingHall1P, hallFacilityId: i32, lvlMode: i32) {
    let mut record1P: CArray<RankingHall1P, 4> = zeroed();
    let playerHallRecords: *mut PlayerHallRecords = AllocZeroed(344) as *mut PlayerHallRecords;
    GetPlayerHallRecords(playerHallRecords);
    let mut i: i32 = 0;
    while i < HALL_RECORDS_COUNT {
        record1P[i] = (*gSaveBlock2Ptr).hallRecords1P[hallFacilityId][lvlMode][i];
        i += 1;
    }
    record1P[3] = (*playerHallRecords).onePlayer[hallFacilityId][lvlMode];
    for i in 0..HALL_RECORDS_COUNT {
        let mut highestWinStreak: i32 = 0;
        let mut highestId: i32 = 0;
        for j in 0..4i32 {
            if record1P[j].winStreak as i32 > highestWinStreak {
                highestId = j;
                highestWinStreak = record1P[j].winStreak as i32;
            }
        }
        if record1P[3].winStreak as i32 >= highestWinStreak {
            highestId = HALL_RECORDS_COUNT;
        }
        *dst.at(i) = record1P[highestId];
        record1P[highestId].winStreak = 0;
    }
    Free(playerHallRecords as *mut c_void);
}
unsafe fn Fill2PRecords(dst: *mut RankingHall2P, lvlMode: i32) {
    let mut record2P: CArray<RankingHall2P, 4> = zeroed();
    let playerHallRecords: *mut PlayerHallRecords = AllocZeroed(344) as *mut PlayerHallRecords;
    GetPlayerHallRecords(playerHallRecords);
    let mut i: i32 = 0;
    while i < HALL_RECORDS_COUNT {
        record2P[i] = (*gSaveBlock2Ptr).hallRecords2P[lvlMode][i];
        i += 1;
    }
    record2P[3] = (*playerHallRecords).twoPlayers[lvlMode];
    for i in 0..HALL_RECORDS_COUNT {
        let mut highestWinStreak: i32 = 0;
        let mut highestId: i32 = 0;
        for j in 0..HALL_RECORDS_COUNT {
            if record2P[j].winStreak as i32 > highestWinStreak {
                highestId = j;
                highestWinStreak = record2P[j].winStreak as i32;
            }
        }
        if record2P[3].winStreak as i32 >= highestWinStreak {
            highestId = HALL_RECORDS_COUNT;
        }
        *dst.at(i) = record2P[highestId];
        record2P[highestId].winStreak = 0;
    }
    Free(playerHallRecords as *mut c_void);
}
unsafe fn PrintHallRecords(hallFacilityId: i32, lvlMode: i32) {
    let mut records1P: CArray<RankingHall1P, 3> = zeroed();
    let mut records2P: CArray<RankingHall2P, 3> = zeroed();
    StringCopy(
        gStringVar1.as_mut_ptr(),
        sRecordsWindowChallengeTexts[hallFacilityId][0],
    );
    StringExpandPlaceholders(
        gStringVar4.as_mut_ptr(),
        sRecordsWindowChallengeTexts[hallFacilityId][1],
    );
    AddTextPrinterParameterized(
        gRecordsWindowId.get(),
        FONT_NORMAL,
        gStringVar4.as_mut_ptr(),
        0,
        1,
        TEXT_SKIP_DRAW,
        None,
    );
    let x: i32 = GetStringRightAlignXOffset(FONT_NORMAL as i32, sLevelModeText[lvlMode], 208);
    AddTextPrinterParameterized(
        gRecordsWindowId.get(),
        FONT_NORMAL,
        sLevelModeText[lvlMode],
        x as u8,
        1,
        TEXT_SKIP_DRAW,
        None,
    );
    if hallFacilityId == RANKING_HALL_TOWER_LINK {
        (*gSaveBlock2Ptr).frontier.opponentNames[0][7] = EOS;
        (*gSaveBlock2Ptr).frontier.opponentNames[1][7] = EOS;
        Fill2PRecords(records2P.as_mut_ptr(), lvlMode);
        for i in 0..HALL_RECORDS_COUNT {
            Print2PRecord(i, 1, 4, &raw mut records2P[i]);
        }
    } else {
        Fill1PRecords(records1P.as_mut_ptr(), hallFacilityId, lvlMode);
        for i in 0..HALL_RECORDS_COUNT {
            Print1PRecord(i, 1, 4, &raw mut records1P[i], hallFacilityId);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe fn ShowRankingHallRecordsWindow() {
    gRecordsWindowId
        .set(AddWindow((&raw const *sRankingHallRecordsWindowTemplate).cast_mut()) as u8);
    DrawStdWindowFrame(gRecordsWindowId.get(), FALSE);
    FillWindowPixelBuffer(gRecordsWindowId.get(), 17);
    PrintHallRecords(gSpecialVar_0x8005 as i32, FRONTIER_LVL_50 as i32);
    PutWindowTilemap(gRecordsWindowId.get());
    CopyWindowToVram(gRecordsWindowId.get(), COPYWIN_FULL);
}
#[unsafe(no_mangle)]
pub unsafe fn ScrollRankingHallRecordsWindow() {
    FillWindowPixelBuffer(gRecordsWindowId.get(), 17);
    PrintHallRecords(gSpecialVar_0x8005 as i32, FRONTIER_LVL_OPEN as i32);
    CopyWindowToVram(gRecordsWindowId.get(), COPYWIN_GFX);
}
#[unsafe(no_mangle)]
pub unsafe fn ClearRankingHallRecords() {
    let mut emptyId: CArray<u8, 4> = CArray([0, 0, 0, 0]);
    for i in 0..HALL_FACILITIES_COUNT {
        for j in 0..FRONTIER_LVL_MODE_COUNT {
            for k in 0..HALL_RECORDS_COUNT {
                CopyTrainerId(
                    (*gSaveBlock2Ptr).hallRecords1P[i][j][k].id.as_mut_ptr(),
                    emptyId.as_mut_ptr(),
                );
                (*gSaveBlock2Ptr).hallRecords1P[i][j][k].name[0] = EOS;
                (*gSaveBlock2Ptr).hallRecords1P[i][j][k].winStreak = 0;
            }
        }
    }
    for j in 0..FRONTIER_LVL_MODE_COUNT {
        for k in 0..HALL_RECORDS_COUNT {
            CopyTrainerId(
                (*gSaveBlock2Ptr).hallRecords2P[j][k].id1.as_mut_ptr(),
                emptyId.as_mut_ptr(),
            );
            CopyTrainerId(
                (*gSaveBlock2Ptr).hallRecords2P[j][k].id2.as_mut_ptr(),
                emptyId.as_mut_ptr(),
            );
            (*gSaveBlock2Ptr).hallRecords2P[j][k].name1[0] = EOS;
            (*gSaveBlock2Ptr).hallRecords2P[j][k].name2[0] = EOS;
            (*gSaveBlock2Ptr).hallRecords2P[j][k].winStreak = 0;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe fn SaveGameFrontier() {
    let monsParty: *mut Pokemon = AllocZeroed(600) as *mut Pokemon;
    for i in 0..PARTY_SIZE {
        *monsParty.at(i) = gPlayerParty[i];
    }
    let i: i32 = gPlayerPartyCount as i32;
    LoadPlayerParty();
    SetContinueGameWarpStatusToDynamicWarp();
    TrySavingData(SAVE_LINK);
    ClearContinueGameWarpStatus2();
    gPlayerPartyCount = i as u8;
    for i in 0..PARTY_SIZE {
        gPlayerParty[i] = *monsParty.at(i);
    }
    Free(monsParty as *mut c_void);
}
pub unsafe fn GetFrontierBrainTrainerPicIndex() -> u8 {
    let mut facility: i32 = 0;
    if gBattleTypeFlags & BATTLE_TYPE_RECORDED != 0 {
        facility = GetRecordedBattleFrontierFacility() as i32;
    } else {
        facility = VarGet(VAR_FRONTIER_FACILITY) as i32;
    }
    (*(&raw const crate::data::data_tables::gTrainers).cast::<CArray<Trainer, 0>>())
        [sFrontierBrainTrainerIds[facility]]
        .trainerPic
}
pub unsafe fn GetFrontierBrainTrainerClass() -> u8 {
    let mut facility: i32 = 0;
    if gBattleTypeFlags & BATTLE_TYPE_RECORDED != 0 {
        facility = GetRecordedBattleFrontierFacility() as i32;
    } else {
        facility = VarGet(VAR_FRONTIER_FACILITY) as i32;
    }
    (*(&raw const crate::data::data_tables::gTrainers).cast::<CArray<Trainer, 0>>())
        [sFrontierBrainTrainerIds[facility]]
        .trainerClass
}
pub unsafe fn CopyFrontierBrainTrainerName(dst: *mut u8) {
    let mut facility: i32 = 0;
    if gBattleTypeFlags & BATTLE_TYPE_RECORDED != 0 {
        facility = GetRecordedBattleFrontierFacility() as i32;
    } else {
        facility = VarGet(VAR_FRONTIER_FACILITY) as i32;
    }
    let mut i: i32 = 0;
    while i < PLAYER_NAME_LENGTH {
        *dst.at(i) = (*(&raw const crate::data::data_tables::gTrainers)
            .cast::<CArray<Trainer, 0>>())[sFrontierBrainTrainerIds[facility]]
            .trainerName[i];
        i += 1;
    }
    *dst.at(i) = EOS;
}
pub unsafe fn IsFrontierBrainFemale() -> u8 {
    let facility: i32 = VarGet(VAR_FRONTIER_FACILITY) as i32;
    sFrontierBrainObjEventGfx[facility][1]
}
pub unsafe fn SetFrontierBrainObjEventGfx_2() {
    let facility: i32 = VarGet(VAR_FRONTIER_FACILITY) as i32;
    VarSet(
        VAR_OBJ_GFX_ID_0,
        sFrontierBrainObjEventGfx[facility][0] as u16,
    );
}
pub unsafe fn CreateFrontierBrainPokemon() {
    let mut j: i32 = 0;
    let mut selectedMonBits: i32 = 0;
    let mut friendship: u8 = 0;
    let facility: i32 = VarGet(VAR_FRONTIER_FACILITY) as i32;
    let symbol: i32 = GetFronterBrainSymbol();
    if facility == FRONTIER_FACILITY_DOME {
        selectedMonBits =
            GetDomeTrainerSelectedMons(TrainerIdToDomeTournamentId(TRAINER_FRONTIER_BRAIN) as u16);
    } else {
        selectedMonBits = 7;
    }
    ZeroEnemyPartyMons();
    let mut monPartyId: i32 = 0;
    let monLevel: i32 = SetFacilityPtrsGetLevel() as i32;
    for i in 0..FRONTIER_PARTY_SIZE {
        'l1: {
            if selectedMonBits & 1 == 0 {
                break 'l1;
            }
            loop {
                loop {
                    j = Random() as i32 | (Random() as i32) << 16;
                    if IsShinyOtIdPersonality(FRONTIER_BRAIN_OTID, j as u32) == 0 {
                        break;
                    }
                }
                if sFrontierBrainsMons[facility][symbol][i].nature
                    == GetNatureFromPersonality(j as u32)
                {
                    break;
                }
            }
            CreateMon(
                &raw mut gEnemyParty[monPartyId],
                sFrontierBrainsMons[facility][symbol][i].species,
                monLevel as u8,
                sFrontierBrainsMons[facility][symbol][i].fixedIV,
                TRUE,
                j as u32,
                OT_ID_PRESET,
                FRONTIER_BRAIN_OTID,
            );
            SetMonData(
                &raw mut gEnemyParty[monPartyId],
                MON_DATA_HELD_ITEM,
                (&raw const sFrontierBrainsMons[facility][symbol][i].heldItem).cast_mut()
                    as *mut c_void,
            );
            j = 0;
            while j < NUM_STATS {
                SetMonData(
                    &raw mut gEnemyParty[monPartyId],
                    MON_DATA_HP_EV + j,
                    (&raw const sFrontierBrainsMons[facility][symbol][i].evs[j]).cast_mut()
                        as *mut c_void,
                );
                j += 1;
            }
            friendship = MAX_FRIENDSHIP;
            for j in 0..MAX_MON_MOVES {
                SetMonMoveSlot(
                    &raw mut gEnemyParty[monPartyId],
                    sFrontierBrainsMons[facility][symbol][i].moves[j],
                    j as u8,
                );
                if sFrontierBrainsMons[facility][symbol][i].moves[j] == MOVE_FRUSTRATION {
                    friendship = 0;
                }
            }
            SetMonData(
                &raw mut gEnemyParty[monPartyId],
                MON_DATA_FRIENDSHIP,
                &raw mut friendship as *mut c_void,
            );
            CalculateMonStats(&raw mut gEnemyParty[monPartyId]);
            monPartyId += 1;
        }
        selectedMonBits >>= 1;
    }
}
pub unsafe fn GetFrontierBrainMonSpecies(monId: u8) -> u16 {
    let facility: i32 = VarGet(VAR_FRONTIER_FACILITY) as i32;
    let symbol: i32 = GetFronterBrainSymbol();
    sFrontierBrainsMons[facility][symbol][monId].species
}
pub unsafe fn SetFrontierBrainObjEventGfx(facility: u8) {
    gTrainerBattleOpponent_A = TRAINER_FRONTIER_BRAIN;
    VarSet(
        VAR_OBJ_GFX_ID_0,
        sFrontierBrainObjEventGfx[facility][0] as u16,
    );
}
pub unsafe fn GetFrontierBrainMonMove(monId: u8, moveSlotId: u8) -> u16 {
    let facility: i32 = VarGet(VAR_FRONTIER_FACILITY) as i32;
    let symbol: i32 = GetFronterBrainSymbol();
    sFrontierBrainsMons[facility][symbol][monId].moves[moveSlotId]
}
pub unsafe fn GetFrontierBrainMonNature(monId: u8) -> u8 {
    let facility: i32 = VarGet(VAR_FRONTIER_FACILITY) as i32;
    let symbol: i32 = GetFronterBrainSymbol();
    sFrontierBrainsMons[facility][symbol][monId].nature
}
pub unsafe fn GetFrontierBrainMonEvs(monId: u8, evStatId: u8) -> u8 {
    let facility: i32 = VarGet(VAR_FRONTIER_FACILITY) as i32;
    let symbol: i32 = GetFronterBrainSymbol();
    sFrontierBrainsMons[facility][symbol][monId].evs[evStatId]
}
pub unsafe fn GetFronterBrainSymbol() -> i32 {
    let facility: i32 = VarGet(VAR_FRONTIER_FACILITY) as i32;
    let mut symbol: i32 = GetPlayerSymbolCountForFacility(facility as u8) as i32;
    if symbol == 2 {
        let winStreak: u16 = GetCurrentFacilityWinStreak() as u16;
        if winStreak as i32 + sFrontierBrainStreakAppearances[facility][3] as i32
            == sFrontierBrainStreakAppearances[facility][0] as i32
        {
            symbol = 0;
        } else if winStreak as i32 + sFrontierBrainStreakAppearances[facility][3] as i32
            == sFrontierBrainStreakAppearances[facility][1] as i32
        {
            symbol = 1;
        } else if winStreak as i32 + sFrontierBrainStreakAppearances[facility][3] as i32
            > sFrontierBrainStreakAppearances[facility][1] as i32
            && rem_i32(
                winStreak as i32 + sFrontierBrainStreakAppearances[facility][3] as i32
                    - sFrontierBrainStreakAppearances[facility][1] as i32,
                sFrontierBrainStreakAppearances[facility][2] as i32,
            ) == 0
        {
            symbol = 1;
        }
    }
    symbol
}
unsafe fn CopyFrontierBrainText(playerWonText: u8) {
    let mut facility: i32 = 0;
    let mut symbol: i32 = 0;
    if gBattleTypeFlags & BATTLE_TYPE_RECORDED != 0 {
        facility = GetRecordedBattleFrontierFacility() as i32;
        symbol = GetRecordedBattleFronterBrainSymbol() as i32;
    } else {
        facility = VarGet(VAR_FRONTIER_FACILITY) as i32;
        symbol = GetFronterBrainSymbol();
    }
    match playerWonText {
        FALSE => {
            StringCopy(
                gStringVar4.as_mut_ptr(),
                *sFrontierBrainPlayerLostTexts[symbol].at(facility),
            );
        }
        TRUE => {
            StringCopy(
                gStringVar4.as_mut_ptr(),
                *sFrontierBrainPlayerWonTexts[symbol].at(facility),
            );
        }
        _ => {}
    }
}
