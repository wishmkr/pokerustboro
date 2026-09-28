//! Translated from `src/frontier_util.c` by tools/rustport/c2rs.py.
#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    unused_mut,
    unused_variables,
    unused_assignments,
    unused_parens,
    unused_braces,
    unused_labels,
    unused_comparisons,
    overflowing_literals,
    unused_unsafe,
    dead_code,
    unreachable_code,
    static_mut_refs,
    unsafe_op_in_unsafe_fn,
    clippy::all,
    clashing_extern_declarations,
    unpredictable_function_pointer_comparisons,
    dangerous_implicit_autorefs
)]

#[allow(unused_imports)]
use crate::c::*;
#[allow(unused_imports)]
use crate::consts::*;
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use core::ffi::c_void;
#[allow(unused_imports)]
use core::mem::zeroed;
#[allow(unused_imports)]
use core::ptr::null_mut;
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
static sFrontierUtilFuncs: Table<CArray<Option<unsafe extern "C" fn()>, 23>> =
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

unsafe extern "C" {
    static gApprentices: CArray<ApprenticeTrainer, 0>;
    static gBattleFrontierTrainers: CArray<BattleFrontierTrainer, 0>;
    static mut gBattleOutcome: u8;
    static mut gBattleScripting: BattleScripting;
    static mut gBattleTypeFlags: u32;
    static mut gEnemyParty: CArray<Pokemon, 6>;
    static mut gFacilityTrainers: *mut BattleFrontierTrainer;
    static mut gLinkPlayers: CArray<LinkPlayer, 5>;
    static mut gPlayerParty: CArray<Pokemon, 6>;
    static mut gPlayerPartyCount: u8;
    static mut gRecordsWindowId: u8;
    static mut gSaveBlock1Ptr: *mut SaveBlock1;
    static mut gSaveBlock2Ptr: *mut SaveBlock2;
    static mut gSelectedOrderFromParty: CArray<u8, 4>;
    static mut gSpecialVar_0x8004: u16;
    static mut gSpecialVar_0x8005: u16;
    static mut gSpecialVar_0x8006: u16;
    static mut gSpecialVar_Result: u16;
    static gSpeciesNames: CArray<CArray<u8, 11>, 0>;
    static mut gStringVar1: CArray<u8, 256>;
    static mut gStringVar2: CArray<u8, 256>;
    static mut gStringVar4: CArray<u8, 1000>;
    static gText_123Dot: CArray<CArray<u8, 3>, 0>;
    static gText_1st: CArray<u8, 0>;
    static gText_2nd: CArray<u8, 0>;
    static gText_3rd: CArray<u8, 0>;
    static gText_4th: CArray<u8, 0>;
    static gText_Are: CArray<u8, 0>;
    static gText_Are2: CArray<u8, 0>;
    static gText_BattleChoiceResults: CArray<u8, 0>;
    static gText_BattleQuestResults: CArray<u8, 0>;
    static gText_BattleSwapDoubleResults: CArray<u8, 0>;
    static gText_BattleSwapSingleResults: CArray<u8, 0>;
    static gText_Beauty: CArray<u8, 0>;
    static gText_Championships: CArray<u8, 0>;
    static gText_ClearStreak: CArray<u8, 0>;
    static gText_CommaSpace: CArray<u8, 0>;
    static gText_Cool: CArray<u8, 0>;
    static gText_Current: CArray<u8, 0>;
    static gText_Cute: CArray<u8, 0>;
    static gText_DoubleBattleHallResults: CArray<u8, 0>;
    static gText_DoubleBattleRoomResults: CArray<u8, 0>;
    static gText_DoubleBattleTourneyResults: CArray<u8, 0>;
    static gText_FloorsCleared: CArray<u8, 0>;
    static gText_KOsInARow: CArray<u8, 0>;
    static gText_LineBreak: CArray<u8, 0>;
    static gText_LinkContestResults: CArray<u8, 0>;
    static gText_LinkMultiBattleRoomResults: CArray<u8, 0>;
    static gText_Lv502: CArray<u8, 0>;
    static gText_MultiBattleRoomResults: CArray<u8, 0>;
    static gText_NewLine: CArray<u8, 0>;
    static gText_OpenLv: CArray<u8, 0>;
    static gText_Prev: CArray<u8, 0>;
    static gText_Record: CArray<u8, 0>;
    static gText_RentalSwap: CArray<u8, 0>;
    static gText_RoomsCleared: CArray<u8, 0>;
    static gText_SetKOTourneyResults: CArray<u8, 0>;
    static gText_SingleBattleHallResults: CArray<u8, 0>;
    static gText_SingleBattleRoomResults: CArray<u8, 0>;
    static gText_SingleBattleTourneyResults: CArray<u8, 0>;
    static gText_Smart: CArray<u8, 0>;
    static gText_Space2: CArray<u8, 0>;
    static gText_SpaceAndSpace: CArray<u8, 0>;
    static gText_TimesCleared: CArray<u8, 0>;
    static gText_TimesVar1: CArray<u8, 0>;
    static gText_Total: CArray<u8, 0>;
    static gText_Tough: CArray<u8, 0>;
    static gText_WinStreak: CArray<u8, 0>;
    static mut gTrainerBattleOpponent_A: u16;
    static gTrainers: CArray<Trainer, 0>;
    fn AddTextPrinterParameterized(
        a0: u8,
        a1: u8,
        a2: *mut u8,
        a3: u8,
        a4: u8,
        a5: u8,
        a6: Option<unsafe extern "C" fn(*mut TextPrinterTemplate, u16)>,
    ) -> u16;
    fn AddWindow(a0: *mut WindowTemplate) -> u16;
    fn AllocZeroed(a0: u32) -> *mut c_void;
    fn BufferApprenticeChallengeText(a0: u8);
    fn CalculateMonStats(a0: *mut Pokemon);
    fn ClearContinueGameWarpStatus2();
    fn ClearSelectedPartyOrder();
    fn ConvertIntToDecimalStringN(a0: *mut u8, a1: i32, a2: i32, a3: u8) -> *mut u8;
    fn CopyTrainerId(a0: *mut u8, a1: *mut u8);
    fn CopyWindowToVram(a0: u8, a1: u8);
    fn CreateMon(a0: *mut Pokemon, a1: u16, a2: u8, a3: u8, a4: u8, a5: u32, a6: u8, a7: u32);
    fn DoSoftReset();
    fn DrawStdWindowFrame(a0: u8, a1: u8);
    fn FillWindowPixelBuffer(a0: u8, a1: u8);
    fn FlagGet(a0: u16) -> u8;
    fn FlagSet(a0: u16) -> u8;
    fn Free(a0: *mut c_void);
    fn FrontierGamblerSetWonOrLost(a0: u8);
    fn FrontierSpeechToString(a0: *mut u16);
    fn GetDomeTrainerSelectedMons(a0: u16) -> i32;
    fn GetFrontierTrainerName(a0: *mut u8, a1: u16);
    fn GetMonData2(a0: *mut Pokemon, a1: i32) -> u32;
    fn GetMonData3(a0: *mut Pokemon, a1: i32, a2: *mut u8) -> u32;
    fn GetNatureFromPersonality(a0: u32) -> u8;
    fn GetPlayerHallRecords(a0: *mut PlayerHallRecords);
    fn GetRecordedBattleApprenticeId() -> u8;
    fn GetRecordedBattleEasyChatSpeech() -> *mut u16;
    fn GetRecordedBattleFronterBrainSymbol() -> u8;
    fn GetRecordedBattleFrontierFacility() -> u8;
    fn GetSetPokedexFlag(a0: u16, a1: u8) -> i8;
    fn GetStringCenterAlignXOffset(a0: i32, a1: *mut u8, a2: i32) -> i32;
    fn GetStringRightAlignXOffset(a0: i32, a1: *mut u8, a2: i32) -> i32;
    fn IncrementDailyBattlePoints(a0: u16);
    fn IsShinyOtIdPersonality(a0: u32, a1: u32) -> u8;
    fn IsStringJapanese(a0: *mut u8) -> u32;
    fn LoadPlayerParty();
    fn MoveRecordedBattleToSaveData() -> u32;
    fn PutWindowTilemap(a0: u8);
    fn Random() -> u16;
    fn ReducePlayerPartyToSelectedMons();
    fn SetContinueGameWarpStatusToDynamicWarp();
    fn SetFacilityPtrsGetLevel() -> u8;
    fn SetGameStat(a0: u8, a1: u32);
    fn SetMonData(a0: *mut Pokemon, a1: i32, a2: *mut c_void);
    fn SetMonMoveSlot(a0: *mut Pokemon, a1: u16, a2: u8);
    fn SetTrainerId(a0: u32, a1: *mut u8);
    fn ShouldAirFrontierTVShow() -> u8;
    fn SpeciesToNationalPokedexNum(a0: u16) -> u16;
    fn StringAppend(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn StringCopy(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn StringExpandPlaceholders(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn StripExtCtrlCodes(a0: *mut u8);
    fn TVShowConvertInternationalString(a0: *mut u8, a1: *mut u8, a2: i32);
    fn TrainerIdToDomeTournamentId(a0: u16) -> i32;
    fn TryPutFrontierTVShowOnAir(a0: u16, a1: u8);
    fn TrySavingData(a0: u8) -> u8;
    fn ValidateEReaderTrainer();
    fn VarGet(a0: u16) -> u16;
    fn VarSet(a0: u16, a1: u16) -> u8;
    fn ZeroEnemyPartyMons();
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn CallFrontierUtilFunc() {
    sFrontierUtilFuncs[gSpecialVar_0x8004].unwrap_unchecked()();
}
pub(crate) unsafe extern "C" fn GetChallengeStatus() {
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
pub(crate) unsafe extern "C" fn GetFrontierData() {
    let mut facility: u8 = VarGet(VAR_FRONTIER_FACILITY) as u8;
    let mut hasSymbol: u8 = GetPlayerSymbolCountForFacility(facility);
    if hasSymbol == 2 {
        hasSymbol = 1;
    }
    match gSpecialVar_0x8005 {
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
pub(crate) unsafe extern "C" fn SetFrontierData() {
    let mut i: i32 = 0;
    let mut facility: u8 = VarGet(VAR_FRONTIER_FACILITY) as u8;
    let mut hasSymbol: u8 = GetPlayerSymbolCountForFacility(facility);
    if hasSymbol == 2 {
        hasSymbol = 1;
    }
    match gSpecialVar_0x8005 {
        FRONTIER_DATA_CHALLENGE_STATUS => {
            (*gSaveBlock2Ptr).frontier.challengeStatus = gSpecialVar_0x8006 as u8;
        }
        FRONTIER_DATA_LVL_MODE => {
            (*gSaveBlock2Ptr)
                .frontier
                .set_lvlMode(gSpecialVar_0x8006 as u8);
        }
        FRONTIER_DATA_BATTLE_NUM => {
            (*gSaveBlock2Ptr).frontier.curChallengeBattleNum = gSpecialVar_0x8006;
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
pub(crate) unsafe extern "C" fn SetSelectedPartyOrder() {
    let mut i: i32 = 0;
    ClearSelectedPartyOrder();
    i = 0;
    while i < gSpecialVar_0x8005 as i32 {
        gSelectedOrderFromParty[i] = (*gSaveBlock2Ptr).frontier.selectedPartyMons[i] as u8;
        i += 1;
    }
    ReducePlayerPartyToSelectedMons();
}
pub(crate) unsafe extern "C" fn DoSoftReset_() {
    DoSoftReset();
}
pub(crate) unsafe extern "C" fn SetFrontierTrainers() {
    gFacilityTrainers = gBattleFrontierTrainers.as_ptr().cast_mut();
}
pub(crate) unsafe extern "C" fn SaveSelectedParty() {
    let mut i: u8 = 0;
    i = 0;
    while (i as i32)
        < (if 3 >= (if 4 >= 2 { 4 } else { 2 }) {
            3
        } else {
            if 4 >= 2 { 4 } else { 2 }
        })
    {
        let mut monId: u16 = (*gSaveBlock2Ptr).frontier.selectedPartyMons[i] - 1;
        if monId < PARTY_SIZE as u16 {
            (*gSaveBlock1Ptr).playerParty
                [(*gSaveBlock2Ptr).frontier.selectedPartyMons[i] as i32 - 1] = gPlayerParty[i];
        }
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn ShowFacilityResultsWindow() {
    if gSpecialVar_0x8006 >= FRONTIER_MODE_COUNT {
        gSpecialVar_0x8006 = 0;
    }
    match gSpecialVar_0x8005 {
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
pub(crate) unsafe extern "C" fn IsWinStreakActive(challenge: u32) -> u8 {
    if (*gSaveBlock2Ptr).frontier.winStreakActiveFlags & challenge != 0 {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn PrintAligned(str: *mut u8, mut y: i32) {
    let mut x: i32 = GetStringCenterAlignXOffset(FONT_NORMAL as i32, str, 224);
    y = y * 8 + 1;
    AddTextPrinterParameterized(
        gRecordsWindowId,
        FONT_NORMAL,
        str,
        x as u8,
        y as u8,
        TEXT_SKIP_DRAW,
        None,
    );
}
pub(crate) unsafe extern "C" fn PrintHyphens(mut y: i32) {
    let mut i: i32 = 0;
    let mut text: CArray<u8, 37> = zeroed();
    i = 0;
    while i < 36 {
        text[i] = CHAR_HYPHEN;
        i += 1;
    }
    text[i] = EOS;
    y = y * 8 + 1;
    AddTextPrinterParameterized(
        gRecordsWindowId,
        FONT_NORMAL,
        text.as_mut_ptr(),
        4,
        y as u8,
        TEXT_SKIP_DRAW,
        None,
    );
}
pub(crate) unsafe extern "C" fn TowerPrintStreak(
    str: *mut u8,
    mut num: u16,
    x1: u8,
    x2: u8,
    y: u8,
) {
    AddTextPrinterParameterized(
        gRecordsWindowId,
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
        gText_WinStreak.as_ptr().cast_mut(),
    );
    AddTextPrinterParameterized(
        gRecordsWindowId,
        FONT_NORMAL,
        gStringVar4.as_mut_ptr(),
        x2,
        y,
        TEXT_SKIP_DRAW,
        None,
    );
}
pub(crate) unsafe extern "C" fn TowerPrintRecordStreak(
    battleMode: u8,
    lvlMode: u8,
    x1: u8,
    x2: u8,
    y: u8,
) {
    let mut num: u16 = (*gSaveBlock2Ptr).frontier.towerRecordWinStreaks[battleMode][lvlMode];
    TowerPrintStreak(gText_Record.as_ptr().cast_mut(), num, x1, x2, y);
}
pub(crate) unsafe extern "C" fn TowerGetWinStreak(battleMode: u8, lvlMode: u8) -> u16 {
    let mut winStreak: u16 = (*gSaveBlock2Ptr).frontier.towerWinStreaks[battleMode][lvlMode];
    if winStreak > MAX_STREAK {
        return MAX_STREAK;
    } else {
        return winStreak;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn TowerPrintPrevOrCurrentStreak(
    battleMode: u8,
    lvlMode: u8,
    x1: u8,
    x2: u8,
    y: u8,
) {
    let mut isCurrent: u8 = 0;
    let mut winStreak: u16 = TowerGetWinStreak(battleMode, lvlMode);
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
        TowerPrintStreak(gText_Current.as_ptr().cast_mut(), winStreak, x1, x2, y);
    } else {
        TowerPrintStreak(gText_Prev.as_ptr().cast_mut(), winStreak, x1, x2, y);
    }
}
pub(crate) unsafe extern "C" fn ShowTowerResultsWindow(battleMode: u8) {
    gRecordsWindowId = AddWindow((&raw const *sFrontierResultsWindowTemplate).cast_mut()) as u8;
    DrawStdWindowFrame(gRecordsWindowId, FALSE);
    FillWindowPixelBuffer(gRecordsWindowId, 17);
    if battleMode == FRONTIER_MODE_SINGLES as u8 {
        StringExpandPlaceholders(
            gStringVar4.as_mut_ptr(),
            gText_SingleBattleRoomResults.as_ptr().cast_mut(),
        );
    } else if battleMode == FRONTIER_MODE_DOUBLES as u8 {
        StringExpandPlaceholders(
            gStringVar4.as_mut_ptr(),
            gText_DoubleBattleRoomResults.as_ptr().cast_mut(),
        );
    } else if battleMode == FRONTIER_MODE_MULTIS as u8 {
        StringExpandPlaceholders(
            gStringVar4.as_mut_ptr(),
            gText_MultiBattleRoomResults.as_ptr().cast_mut(),
        );
    } else {
        StringExpandPlaceholders(
            gStringVar4.as_mut_ptr(),
            gText_LinkMultiBattleRoomResults.as_ptr().cast_mut(),
        );
    }
    PrintAligned(gStringVar4.as_mut_ptr(), 2);
    AddTextPrinterParameterized(
        gRecordsWindowId,
        FONT_NORMAL,
        gText_Lv502.as_ptr().cast_mut(),
        16,
        49,
        TEXT_SKIP_DRAW,
        None,
    );
    AddTextPrinterParameterized(
        gRecordsWindowId,
        FONT_NORMAL,
        gText_OpenLv.as_ptr().cast_mut(),
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
    PutWindowTilemap(gRecordsWindowId);
    CopyWindowToVram(gRecordsWindowId, COPYWIN_FULL);
}
pub(crate) unsafe extern "C" fn DomeGetWinStreak(battleMode: u8, lvlMode: u8) -> u16 {
    let mut winStreak: u16 = (*gSaveBlock2Ptr).frontier.domeWinStreaks[battleMode][lvlMode];
    if winStreak > MAX_STREAK {
        return MAX_STREAK;
    } else {
        return winStreak;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn PrintTwoStrings(
    str1: *mut u8,
    str2: *mut u8,
    num: u16,
    x1: u8,
    x2: u8,
    y: u8,
) {
    AddTextPrinterParameterized(
        gRecordsWindowId,
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
        gRecordsWindowId,
        FONT_NORMAL,
        gStringVar4.as_mut_ptr(),
        x2,
        y,
        TEXT_SKIP_DRAW,
        None,
    );
}
pub(crate) unsafe extern "C" fn DomePrintPrevOrCurrentStreak(
    battleMode: u8,
    lvlMode: u8,
    x1: u8,
    x2: u8,
    y: u8,
) {
    let mut isCurrent: u8 = 0;
    let mut winStreak: u16 = DomeGetWinStreak(battleMode, lvlMode);
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
            gText_Current.as_ptr().cast_mut(),
            gText_ClearStreak.as_ptr().cast_mut(),
            winStreak,
            x1,
            x2,
            y,
        );
    } else {
        PrintTwoStrings(
            gText_Prev.as_ptr().cast_mut(),
            gText_ClearStreak.as_ptr().cast_mut(),
            winStreak,
            x1,
            x2,
            y,
        );
    }
}
pub(crate) unsafe extern "C" fn ShowDomeResultsWindow(battleMode: u8) {
    gRecordsWindowId = AddWindow((&raw const *sFrontierResultsWindowTemplate).cast_mut()) as u8;
    DrawStdWindowFrame(gRecordsWindowId, FALSE);
    FillWindowPixelBuffer(gRecordsWindowId, 17);
    if battleMode == FRONTIER_MODE_SINGLES as u8 {
        StringExpandPlaceholders(
            gStringVar4.as_mut_ptr(),
            gText_SingleBattleTourneyResults.as_ptr().cast_mut(),
        );
    } else {
        StringExpandPlaceholders(
            gStringVar4.as_mut_ptr(),
            gText_DoubleBattleTourneyResults.as_ptr().cast_mut(),
        );
    }
    PrintAligned(gStringVar4.as_mut_ptr(), 0);
    AddTextPrinterParameterized(
        gRecordsWindowId,
        FONT_NORMAL,
        gText_Lv502.as_ptr().cast_mut(),
        8,
        33,
        TEXT_SKIP_DRAW,
        None,
    );
    AddTextPrinterParameterized(
        gRecordsWindowId,
        FONT_NORMAL,
        gText_OpenLv.as_ptr().cast_mut(),
        8,
        97,
        TEXT_SKIP_DRAW,
        None,
    );
    PrintHyphens(10);
    DomePrintPrevOrCurrentStreak(battleMode, FRONTIER_LVL_50, 64, 121, 33);
    PrintTwoStrings(
        gText_Record.as_ptr().cast_mut(),
        gText_ClearStreak.as_ptr().cast_mut(),
        (*gSaveBlock2Ptr).frontier.domeRecordWinStreaks[battleMode][0],
        64,
        121,
        49,
    );
    PrintTwoStrings(
        gText_Total.as_ptr().cast_mut(),
        gText_Championships.as_ptr().cast_mut(),
        (*gSaveBlock2Ptr).frontier.domeTotalChampionships[battleMode][0],
        64,
        112,
        65,
    );
    DomePrintPrevOrCurrentStreak(battleMode, FRONTIER_LVL_OPEN, 64, 121, 97);
    PrintTwoStrings(
        gText_Record.as_ptr().cast_mut(),
        gText_ClearStreak.as_ptr().cast_mut(),
        (*gSaveBlock2Ptr).frontier.domeRecordWinStreaks[battleMode][1],
        64,
        121,
        113,
    );
    PrintTwoStrings(
        gText_Total.as_ptr().cast_mut(),
        gText_Championships.as_ptr().cast_mut(),
        (*gSaveBlock2Ptr).frontier.domeTotalChampionships[battleMode][1],
        64,
        112,
        129,
    );
    PutWindowTilemap(gRecordsWindowId);
    CopyWindowToVram(gRecordsWindowId, COPYWIN_FULL);
}
pub(crate) unsafe extern "C" fn PalacePrintStreak(
    str: *mut u8,
    mut num: u16,
    x1: u8,
    x2: u8,
    y: u8,
) {
    AddTextPrinterParameterized(
        gRecordsWindowId,
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
        gText_WinStreak.as_ptr().cast_mut(),
    );
    AddTextPrinterParameterized(
        gRecordsWindowId,
        FONT_NORMAL,
        gStringVar4.as_mut_ptr(),
        x2,
        y,
        TEXT_SKIP_DRAW,
        None,
    );
}
pub(crate) unsafe extern "C" fn PalacePrintRecordStreak(
    battleMode: u8,
    lvlMode: u8,
    x1: u8,
    x2: u8,
    y: u8,
) {
    let mut num: u16 = (*gSaveBlock2Ptr).frontier.palaceRecordWinStreaks[battleMode][lvlMode];
    PalacePrintStreak(gText_Record.as_ptr().cast_mut(), num, x1, x2, y);
}
pub(crate) unsafe extern "C" fn PalaceGetWinStreak(battleMode: u8, lvlMode: u8) -> u16 {
    let mut winStreak: u16 = (*gSaveBlock2Ptr).frontier.palaceWinStreaks[battleMode][lvlMode];
    if winStreak > MAX_STREAK {
        return MAX_STREAK;
    } else {
        return winStreak;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn PalacePrintPrevOrCurrentStreak(
    battleMode: u8,
    lvlMode: u8,
    x1: u8,
    x2: u8,
    y: u8,
) {
    let mut isCurrent: u8 = 0;
    let mut winStreak: u16 = PalaceGetWinStreak(battleMode, lvlMode);
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
        PalacePrintStreak(gText_Current.as_ptr().cast_mut(), winStreak, x1, x2, y);
    } else {
        PalacePrintStreak(gText_Prev.as_ptr().cast_mut(), winStreak, x1, x2, y);
    }
}
pub(crate) unsafe extern "C" fn ShowPalaceResultsWindow(battleMode: u8) {
    gRecordsWindowId = AddWindow((&raw const *sFrontierResultsWindowTemplate).cast_mut()) as u8;
    DrawStdWindowFrame(gRecordsWindowId, FALSE);
    FillWindowPixelBuffer(gRecordsWindowId, 17);
    if battleMode == FRONTIER_MODE_SINGLES as u8 {
        StringExpandPlaceholders(
            gStringVar4.as_mut_ptr(),
            gText_SingleBattleHallResults.as_ptr().cast_mut(),
        );
    } else {
        StringExpandPlaceholders(
            gStringVar4.as_mut_ptr(),
            gText_DoubleBattleHallResults.as_ptr().cast_mut(),
        );
    }
    PrintAligned(gStringVar4.as_mut_ptr(), 2);
    AddTextPrinterParameterized(
        gRecordsWindowId,
        FONT_NORMAL,
        gText_Lv502.as_ptr().cast_mut(),
        16,
        49,
        TEXT_SKIP_DRAW,
        None,
    );
    AddTextPrinterParameterized(
        gRecordsWindowId,
        FONT_NORMAL,
        gText_OpenLv.as_ptr().cast_mut(),
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
    PutWindowTilemap(gRecordsWindowId);
    CopyWindowToVram(gRecordsWindowId, COPYWIN_FULL);
}
pub(crate) unsafe extern "C" fn PikeGetWinStreak(lvlMode: u8) -> u16 {
    let mut winStreak: u16 = (*gSaveBlock2Ptr).frontier.pikeWinStreaks[lvlMode];
    if winStreak > MAX_STREAK {
        return MAX_STREAK;
    } else {
        return winStreak;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn PikePrintCleared(
    str1: *mut u8,
    str2: *mut u8,
    num: u16,
    x1: u8,
    x2: u8,
    y: u8,
) {
    AddTextPrinterParameterized(
        gRecordsWindowId,
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
        gRecordsWindowId,
        FONT_NORMAL,
        gStringVar4.as_mut_ptr(),
        x2,
        y,
        TEXT_SKIP_DRAW,
        None,
    );
}
pub(crate) unsafe extern "C" fn PikePrintPrevOrCurrentStreak(lvlMode: u8, x1: u8, x2: u8, y: u8) {
    let mut isCurrent: u8 = 0;
    let mut winStreak: u16 = PikeGetWinStreak(lvlMode);
    if lvlMode != FRONTIER_LVL_50 {
        isCurrent = IsWinStreakActive(STREAK_PIKE_OPEN);
    } else {
        isCurrent = IsWinStreakActive(STREAK_PIKE_50);
    }
    if isCurrent == TRUE {
        PrintTwoStrings(
            gText_Current.as_ptr().cast_mut(),
            gText_RoomsCleared.as_ptr().cast_mut(),
            winStreak,
            x1,
            x2,
            y,
        );
    } else {
        PrintTwoStrings(
            gText_Prev.as_ptr().cast_mut(),
            gText_RoomsCleared.as_ptr().cast_mut(),
            winStreak,
            x1,
            x2,
            y,
        );
    }
}
pub(crate) unsafe extern "C" fn ShowPikeResultsWindow() {
    gRecordsWindowId = AddWindow((&raw const *sFrontierResultsWindowTemplate).cast_mut()) as u8;
    DrawStdWindowFrame(gRecordsWindowId, FALSE);
    FillWindowPixelBuffer(gRecordsWindowId, 17);
    StringExpandPlaceholders(
        gStringVar4.as_mut_ptr(),
        gText_BattleChoiceResults.as_ptr().cast_mut(),
    );
    PrintAligned(gStringVar4.as_mut_ptr(), 0);
    AddTextPrinterParameterized(
        gRecordsWindowId,
        FONT_NORMAL,
        gText_Lv502.as_ptr().cast_mut(),
        8,
        33,
        TEXT_SKIP_DRAW,
        None,
    );
    AddTextPrinterParameterized(
        gRecordsWindowId,
        FONT_NORMAL,
        gText_OpenLv.as_ptr().cast_mut(),
        8,
        97,
        TEXT_SKIP_DRAW,
        None,
    );
    PrintHyphens(10);
    PikePrintPrevOrCurrentStreak(FRONTIER_LVL_50, 64, 114, 33);
    PikePrintCleared(
        gText_Record.as_ptr().cast_mut(),
        gText_RoomsCleared.as_ptr().cast_mut(),
        (*gSaveBlock2Ptr).frontier.pikeRecordStreaks[0],
        64,
        114,
        49,
    );
    PikePrintCleared(
        gText_Total.as_ptr().cast_mut(),
        gText_TimesCleared.as_ptr().cast_mut(),
        (*gSaveBlock2Ptr).frontier.pikeTotalStreaks[0],
        64,
        114,
        65,
    );
    PikePrintPrevOrCurrentStreak(FRONTIER_LVL_OPEN, 64, 114, 97);
    PikePrintCleared(
        gText_Record.as_ptr().cast_mut(),
        gText_RoomsCleared.as_ptr().cast_mut(),
        (*gSaveBlock2Ptr).frontier.pikeRecordStreaks[1],
        64,
        114,
        113,
    );
    PikePrintCleared(
        gText_Total.as_ptr().cast_mut(),
        gText_TimesCleared.as_ptr().cast_mut(),
        (*gSaveBlock2Ptr).frontier.pikeTotalStreaks[1],
        64,
        114,
        129,
    );
    PutWindowTilemap(gRecordsWindowId);
    CopyWindowToVram(gRecordsWindowId, COPYWIN_FULL);
}
pub(crate) unsafe extern "C" fn ArenaPrintStreak(
    str: *mut u8,
    mut num: u16,
    x1: u8,
    x2: u8,
    y: u8,
) {
    AddTextPrinterParameterized(
        gRecordsWindowId,
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
        gText_KOsInARow.as_ptr().cast_mut(),
    );
    AddTextPrinterParameterized(
        gRecordsWindowId,
        FONT_NORMAL,
        gStringVar4.as_mut_ptr(),
        x2,
        y,
        TEXT_SKIP_DRAW,
        None,
    );
}
pub(crate) unsafe extern "C" fn ArenaPrintRecordStreak(lvlMode: u8, x1: u8, x2: u8, y: u8) {
    let mut num: u16 = (*gSaveBlock2Ptr).frontier.arenaRecordStreaks[lvlMode];
    ArenaPrintStreak(gText_Record.as_ptr().cast_mut(), num, x1, x2, y);
}
pub(crate) unsafe extern "C" fn ArenaGetWinStreak(lvlMode: u8) -> u16 {
    let mut winStreak: u16 = (*gSaveBlock2Ptr).frontier.arenaWinStreaks[lvlMode];
    if winStreak > MAX_STREAK {
        return MAX_STREAK;
    } else {
        return winStreak;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn ArenaPrintPrevOrCurrentStreak(lvlMode: u8, x1: u8, x2: u8, y: u8) {
    let mut isCurrent: u8 = 0;
    let mut winStreak: u16 = ArenaGetWinStreak(lvlMode);
    if lvlMode != FRONTIER_LVL_50 {
        isCurrent = IsWinStreakActive(STREAK_ARENA_OPEN);
    } else {
        isCurrent = IsWinStreakActive(STREAK_ARENA_50);
    }
    if isCurrent == TRUE {
        ArenaPrintStreak(gText_Current.as_ptr().cast_mut(), winStreak, x1, x2, y);
    } else {
        ArenaPrintStreak(gText_Prev.as_ptr().cast_mut(), winStreak, x1, x2, y);
    }
}
pub(crate) unsafe extern "C" fn ShowArenaResultsWindow() {
    gRecordsWindowId = AddWindow((&raw const *sFrontierResultsWindowTemplate).cast_mut()) as u8;
    DrawStdWindowFrame(gRecordsWindowId, FALSE);
    FillWindowPixelBuffer(gRecordsWindowId, 17);
    PrintHyphens(10);
    StringExpandPlaceholders(
        gStringVar4.as_mut_ptr(),
        gText_SetKOTourneyResults.as_ptr().cast_mut(),
    );
    PrintAligned(gStringVar4.as_mut_ptr(), 2);
    AddTextPrinterParameterized(
        gRecordsWindowId,
        FONT_NORMAL,
        gText_Lv502.as_ptr().cast_mut(),
        16,
        49,
        TEXT_SKIP_DRAW,
        None,
    );
    AddTextPrinterParameterized(
        gRecordsWindowId,
        FONT_NORMAL,
        gText_OpenLv.as_ptr().cast_mut(),
        16,
        97,
        TEXT_SKIP_DRAW,
        None,
    );
    ArenaPrintPrevOrCurrentStreak(FRONTIER_LVL_50, 72, 126, 49);
    ArenaPrintRecordStreak(FRONTIER_LVL_50, 72, 126, 65);
    ArenaPrintPrevOrCurrentStreak(FRONTIER_LVL_OPEN, 72, 126, 97);
    ArenaPrintRecordStreak(FRONTIER_LVL_OPEN, 72, 126, 113);
    PutWindowTilemap(gRecordsWindowId);
    CopyWindowToVram(gRecordsWindowId, COPYWIN_FULL);
}
pub(crate) unsafe extern "C" fn FactoryPrintStreak(
    str: *mut u8,
    mut num1: u16,
    num2: u16,
    x1: u8,
    x2: u8,
    x3: u8,
    y: u8,
) {
    AddTextPrinterParameterized(
        gRecordsWindowId,
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
        gText_WinStreak.as_ptr().cast_mut(),
    );
    AddTextPrinterParameterized(
        gRecordsWindowId,
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
        gText_TimesVar1.as_ptr().cast_mut(),
    );
    AddTextPrinterParameterized(
        gRecordsWindowId,
        FONT_NORMAL,
        gStringVar4.as_mut_ptr(),
        x3,
        y,
        TEXT_SKIP_DRAW,
        None,
    );
}
pub(crate) unsafe extern "C" fn FactoryPrintRecordStreak(
    battleMode: u8,
    lvlMode: u8,
    x1: u8,
    x2: u8,
    x3: u8,
    y: u8,
) {
    let mut num1: u16 = (*gSaveBlock2Ptr).frontier.factoryRecordWinStreaks[battleMode][lvlMode];
    let mut num2: u16 = (*gSaveBlock2Ptr).frontier.factoryRecordRentsCount[battleMode][lvlMode];
    FactoryPrintStreak(gText_Record.as_ptr().cast_mut(), num1, num2, x1, x2, x3, y);
}
pub(crate) unsafe extern "C" fn FactoryGetWinStreak(battleMode: u8, lvlMode: u8) -> u16 {
    let mut winStreak: u16 = (*gSaveBlock2Ptr).frontier.factoryWinStreaks[battleMode][lvlMode];
    if winStreak > MAX_STREAK {
        return MAX_STREAK;
    } else {
        return winStreak;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn FactoryGetRentsCount(battleMode: u8, lvlMode: u8) -> u16 {
    let mut rents: u16 = (*gSaveBlock2Ptr).frontier.factoryRentsCount[battleMode][lvlMode];
    if rents > MAX_STREAK {
        return MAX_STREAK;
    } else {
        return rents;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn FactoryPrintPrevOrCurrentStreak(
    battleMode: u8,
    lvlMode: u8,
    x1: u8,
    x2: u8,
    x3: u8,
    y: u8,
) {
    let mut isCurrent: u8 = 0;
    let mut winStreak: u16 = FactoryGetWinStreak(battleMode, lvlMode);
    let mut rents: u16 = FactoryGetRentsCount(battleMode, lvlMode);
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
            gText_Current.as_ptr().cast_mut(),
            winStreak,
            rents,
            x1,
            x2,
            x3,
            y,
        );
    } else {
        FactoryPrintStreak(
            gText_Prev.as_ptr().cast_mut(),
            winStreak,
            rents,
            x1,
            x2,
            x3,
            y,
        );
    }
}
pub(crate) unsafe extern "C" fn ShowFactoryResultsWindow(battleMode: u8) {
    gRecordsWindowId = AddWindow((&raw const *sFrontierResultsWindowTemplate).cast_mut()) as u8;
    DrawStdWindowFrame(gRecordsWindowId, FALSE);
    FillWindowPixelBuffer(gRecordsWindowId, 17);
    if battleMode == FRONTIER_MODE_SINGLES as u8 {
        StringExpandPlaceholders(
            gStringVar4.as_mut_ptr(),
            gText_BattleSwapSingleResults.as_ptr().cast_mut(),
        );
    } else {
        StringExpandPlaceholders(
            gStringVar4.as_mut_ptr(),
            gText_BattleSwapDoubleResults.as_ptr().cast_mut(),
        );
    }
    PrintAligned(gStringVar4.as_mut_ptr(), 0);
    AddTextPrinterParameterized(
        gRecordsWindowId,
        FONT_NORMAL,
        gText_Lv502.as_ptr().cast_mut(),
        8,
        33,
        TEXT_SKIP_DRAW,
        None,
    );
    AddTextPrinterParameterized(
        gRecordsWindowId,
        FONT_NORMAL,
        gText_RentalSwap.as_ptr().cast_mut(),
        152,
        33,
        TEXT_SKIP_DRAW,
        None,
    );
    AddTextPrinterParameterized(
        gRecordsWindowId,
        FONT_NORMAL,
        gText_OpenLv.as_ptr().cast_mut(),
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
    PutWindowTilemap(gRecordsWindowId);
    CopyWindowToVram(gRecordsWindowId, COPYWIN_FULL);
}
pub(crate) unsafe extern "C" fn PyramidPrintStreak(
    str: *mut u8,
    mut num: u16,
    x1: u8,
    x2: u8,
    y: u8,
) {
    AddTextPrinterParameterized(
        gRecordsWindowId,
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
        gText_FloorsCleared.as_ptr().cast_mut(),
    );
    AddTextPrinterParameterized(
        gRecordsWindowId,
        FONT_NORMAL,
        gStringVar4.as_mut_ptr(),
        x2,
        y,
        TEXT_SKIP_DRAW,
        None,
    );
}
pub(crate) unsafe extern "C" fn PyramidPrintRecordStreak(lvlMode: u8, x1: u8, x2: u8, y: u8) {
    let mut num: u16 = (*gSaveBlock2Ptr).frontier.pyramidRecordStreaks[lvlMode];
    PyramidPrintStreak(gText_Record.as_ptr().cast_mut(), num, x1, x2, y);
}
pub(crate) unsafe extern "C" fn PyramidGetWinStreak(lvlMode: u8) -> u16 {
    let mut winStreak: u16 = (*gSaveBlock2Ptr).frontier.pyramidWinStreaks[lvlMode];
    if winStreak > MAX_STREAK {
        return MAX_STREAK;
    } else {
        return winStreak;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn PyramidPrintPrevOrCurrentStreak(
    lvlMode: u8,
    x1: u8,
    x2: u8,
    y: u8,
) {
    let mut isCurrent: u8 = 0;
    let mut winStreak: u16 = PyramidGetWinStreak(lvlMode);
    if lvlMode != FRONTIER_LVL_50 {
        isCurrent = IsWinStreakActive(STREAK_PYRAMID_OPEN);
    } else {
        isCurrent = IsWinStreakActive(STREAK_PYRAMID_50);
    }
    if isCurrent == TRUE {
        PyramidPrintStreak(gText_Current.as_ptr().cast_mut(), winStreak, x1, x2, y);
    } else {
        PyramidPrintStreak(gText_Prev.as_ptr().cast_mut(), winStreak, x1, x2, y);
    }
}
pub(crate) unsafe extern "C" fn ShowPyramidResultsWindow() {
    gRecordsWindowId = AddWindow((&raw const *sFrontierResultsWindowTemplate).cast_mut()) as u8;
    DrawStdWindowFrame(gRecordsWindowId, FALSE);
    FillWindowPixelBuffer(gRecordsWindowId, 17);
    StringExpandPlaceholders(
        gStringVar4.as_mut_ptr(),
        gText_BattleQuestResults.as_ptr().cast_mut(),
    );
    PrintAligned(gStringVar4.as_mut_ptr(), 2);
    AddTextPrinterParameterized(
        gRecordsWindowId,
        FONT_NORMAL,
        gText_Lv502.as_ptr().cast_mut(),
        8,
        49,
        TEXT_SKIP_DRAW,
        None,
    );
    AddTextPrinterParameterized(
        gRecordsWindowId,
        FONT_NORMAL,
        gText_OpenLv.as_ptr().cast_mut(),
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
    PutWindowTilemap(gRecordsWindowId);
    CopyWindowToVram(gRecordsWindowId, COPYWIN_FULL);
}
pub(crate) unsafe extern "C" fn ShowLinkContestResultsWindow() {
    let mut str: *mut u8 = null_mut();
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut x: i32 = 0;
    gRecordsWindowId = AddWindow((&raw const *sLinkContestResultsWindowTemplate).cast_mut()) as u8;
    DrawStdWindowFrame(gRecordsWindowId, FALSE);
    FillWindowPixelBuffer(gRecordsWindowId, 17);
    StringExpandPlaceholders(
        gStringVar4.as_mut_ptr(),
        gText_LinkContestResults.as_ptr().cast_mut(),
    );
    x = GetStringCenterAlignXOffset(FONT_NORMAL as i32, gStringVar4.as_mut_ptr(), 208);
    AddTextPrinterParameterized(
        gRecordsWindowId,
        FONT_NORMAL,
        gStringVar4.as_mut_ptr(),
        x as u8,
        1,
        TEXT_SKIP_DRAW,
        None,
    );
    str = gText_1st.as_ptr().cast_mut();
    x = GetStringRightAlignXOffset(FONT_NORMAL as i32, str, 38) + 50;
    AddTextPrinterParameterized(
        gRecordsWindowId,
        FONT_NORMAL,
        str,
        x as u8,
        25,
        TEXT_SKIP_DRAW,
        None,
    );
    str = gText_2nd.as_ptr().cast_mut();
    x = GetStringRightAlignXOffset(FONT_NORMAL as i32, str, 38) + 88;
    AddTextPrinterParameterized(
        gRecordsWindowId,
        FONT_NORMAL,
        str,
        x as u8,
        25,
        TEXT_SKIP_DRAW,
        None,
    );
    str = gText_3rd.as_ptr().cast_mut();
    x = GetStringRightAlignXOffset(FONT_NORMAL as i32, str, 38) + 126;
    AddTextPrinterParameterized(
        gRecordsWindowId,
        FONT_NORMAL,
        str,
        x as u8,
        25,
        TEXT_SKIP_DRAW,
        None,
    );
    str = gText_4th.as_ptr().cast_mut();
    x = GetStringRightAlignXOffset(FONT_NORMAL as i32, str, 38) + 164;
    AddTextPrinterParameterized(
        gRecordsWindowId,
        FONT_NORMAL,
        str,
        x as u8,
        25,
        TEXT_SKIP_DRAW,
        None,
    );
    x = 6;
    AddTextPrinterParameterized(
        gRecordsWindowId,
        FONT_NORMAL,
        gText_Cool.as_ptr().cast_mut(),
        x as u8,
        41,
        TEXT_SKIP_DRAW,
        None,
    );
    AddTextPrinterParameterized(
        gRecordsWindowId,
        FONT_NORMAL,
        gText_Beauty.as_ptr().cast_mut(),
        x as u8,
        57,
        TEXT_SKIP_DRAW,
        None,
    );
    AddTextPrinterParameterized(
        gRecordsWindowId,
        FONT_NORMAL,
        gText_Cute.as_ptr().cast_mut(),
        x as u8,
        73,
        TEXT_SKIP_DRAW,
        None,
    );
    AddTextPrinterParameterized(
        gRecordsWindowId,
        FONT_NORMAL,
        gText_Smart.as_ptr().cast_mut(),
        x as u8,
        89,
        TEXT_SKIP_DRAW,
        None,
    );
    AddTextPrinterParameterized(
        gRecordsWindowId,
        FONT_NORMAL,
        gText_Tough.as_ptr().cast_mut(),
        x as u8,
        105,
        TEXT_SKIP_DRAW,
        None,
    );
    i = 0;
    while i < CONTEST_CATEGORIES_COUNT {
        j = 0;
        while j < CONTESTANT_COUNT {
            ConvertIntToDecimalStringN(
                gStringVar4.as_mut_ptr(),
                (*gSaveBlock2Ptr).contestLinkResults[i][j] as i32,
                STR_CONV_MODE_RIGHT_ALIGN,
                4,
            );
            AddTextPrinterParameterized(
                gRecordsWindowId,
                FONT_NORMAL,
                gStringVar4.as_mut_ptr(),
                j as u8 * 38 + 64,
                i as u8 * 16 + 41,
                TEXT_SKIP_DRAW,
                None,
            );
            j += 1;
        }
        i += 1;
    }
    PutWindowTilemap(gRecordsWindowId);
    CopyWindowToVram(gRecordsWindowId, COPYWIN_FULL);
}
pub(crate) unsafe extern "C" fn CheckPutFrontierTVShowOnAir() {
    let mut name: CArray<u8, 32> = zeroed();
    let mut lvlMode: i32 = (*gSaveBlock2Ptr).frontier.lvlMode() as i32;
    let mut facility: i32 = VarGet(VAR_FRONTIER_FACILITY) as i32;
    let mut battleMode: i32 = VarGet(VAR_FRONTIER_BATTLE_MODE) as i32;
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
        FRONTIER_FACILITY_PYRAMID => {
            if (*gSaveBlock2Ptr).frontier.pyramidWinStreaks[lvlMode]
                > (*gSaveBlock2Ptr).frontier.pyramidRecordStreaks[lvlMode]
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
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn Script_GetFrontierBrainStatus() {
    VarGet(VAR_FRONTIER_FACILITY);
    gSpecialVar_Result = GetFrontierBrainStatus() as u16;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetFrontierBrainStatus() -> u8 {
    let mut status: i32 = FRONTIER_BRAIN_NOT_READY as i32;
    let mut facility: i32 = VarGet(VAR_FRONTIER_FACILITY) as i32;
    let mut battleMode: i32 = VarGet(VAR_FRONTIER_BATTLE_MODE) as i32;
    let mut winStreakNoModifier: u16 = GetCurrentFacilityWinStreak() as u16;
    let mut winStreak: i32 =
        winStreakNoModifier as i32 + sFrontierBrainStreakAppearances[facility][3] as i32;
    let mut symbolsCount: i32 = 0;
    if battleMode != FRONTIER_MODE_SINGLES {
        return FRONTIER_BRAIN_NOT_READY;
    }
    symbolsCount = GetPlayerSymbolCountForFacility(facility as u8) as i32;
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
    return status as u8;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CopyFrontierTrainerText(whichText: u8, mut trainerId: u16) {
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
                    FrontierSpeechToString(gApprentices[trainerId].speechLost.as_ptr().cast_mut());
                } else {
                    trainerId = (*gSaveBlock2Ptr).apprentices
                        [trainerId as i32 - TRAINER_RECORD_MIXING_APPRENTICE]
                        .id() as u16;
                    FrontierSpeechToString(gApprentices[trainerId].speechLost.as_ptr().cast_mut());
                }
            }
        }
        _ => {}
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ResetWinStreaks() {
    let mut battleMode: i32 = 0;
    let mut lvlMode: i32 = 0;
    (*gSaveBlock2Ptr).frontier.winStreakActiveFlags = 0;
    battleMode = 0;
    while battleMode < FRONTIER_MODE_COUNT as i32 {
        lvlMode = 0;
        while lvlMode < FRONTIER_LVL_TENT as i32 {
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
            lvlMode += 1;
        }
        battleMode += 1;
    }
    if (*gSaveBlock2Ptr).frontier.challengeStatus != 0 {
        (*gSaveBlock2Ptr).frontier.challengeStatus = CHALLENGE_STATUS_SAVING;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetCurrentFacilityWinStreak() -> u32 {
    let mut lvlMode: i32 = (*gSaveBlock2Ptr).frontier.lvlMode() as i32;
    let mut battleMode: i32 = VarGet(VAR_FRONTIER_BATTLE_MODE) as i32;
    let mut facility: i32 = VarGet(VAR_FRONTIER_FACILITY) as i32;
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
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ResetFrontierTrainerIds() {
    let mut i: i32 = 0;
    i = 0;
    while i < 20 {
        (*gSaveBlock2Ptr).frontier.trainerIds[i] = 0xFFFF;
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn IsTrainerFrontierBrain() {
    if gTrainerBattleOpponent_A == TRAINER_FRONTIER_BRAIN {
        gSpecialVar_Result = TRUE as u16;
    } else {
        gSpecialVar_Result = FALSE as u16;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetPlayerSymbolCountForFacility(facility: u8) -> u8 {
    return FlagGet(FLAG_SYS_TOWER_SILVER + facility as u16 * 2)
        + FlagGet(FLAG_SYS_TOWER_GOLD + facility as u16 * 2);
}
pub(crate) unsafe extern "C" fn GiveBattlePoints() {
    let mut challengeNum: i32 = 0;
    let mut lvlMode: i32 = (*gSaveBlock2Ptr).frontier.lvlMode() as i32;
    let mut facility: i32 = VarGet(VAR_FRONTIER_FACILITY) as i32;
    let mut battleMode: i32 = VarGet(VAR_FRONTIER_BATTLE_MODE) as i32;
    let mut points: i32 = 0;
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
    points = sBattlePointAwards[challengeNum][facility][battleMode] as i32;
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
pub(crate) unsafe extern "C" fn GetFacilitySymbolCount() {
    let mut facility: i32 = VarGet(VAR_FRONTIER_FACILITY) as i32;
    gSpecialVar_Result = GetPlayerSymbolCountForFacility(facility as u8) as u16;
}
pub(crate) unsafe extern "C" fn GiveFacilitySymbol() {
    let mut facility: i32 = VarGet(VAR_FRONTIER_FACILITY) as i32;
    if GetPlayerSymbolCountForFacility(facility as u8) == 0 {
        FlagSet(FLAG_SYS_TOWER_SILVER + facility as u16 * 2);
    } else {
        FlagSet(FLAG_SYS_TOWER_GOLD + facility as u16 * 2);
    }
}
pub(crate) unsafe extern "C" fn CheckBattleTypeFlag() {
    if gBattleTypeFlags & gSpecialVar_0x8005 as u32 != 0 {
        gSpecialVar_Result = TRUE as u16;
    } else {
        gSpecialVar_Result = FALSE as u16;
    }
}
pub(crate) unsafe extern "C" fn AppendCaughtBannedMonSpeciesName(
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
                        gText_SpaceAndSpace.as_ptr().cast_mut(),
                    );
                } else if numBannedMonsCaught > count as i32 {
                    StringAppend(
                        gStringVar1.as_mut_ptr(),
                        gText_CommaSpace.as_ptr().cast_mut(),
                    );
                }
            }
            2 => {
                if count as i32 == numBannedMonsCaught {
                    StringAppend(
                        gStringVar1.as_mut_ptr(),
                        gText_SpaceAndSpace.as_ptr().cast_mut(),
                    );
                } else {
                    StringAppend(
                        gStringVar1.as_mut_ptr(),
                        gText_CommaSpace.as_ptr().cast_mut(),
                    );
                }
                StringAppend(gStringVar1.as_mut_ptr(), gText_NewLine.as_ptr().cast_mut());
            }
            _ => {
                if count as i32 == numBannedMonsCaught {
                    StringAppend(
                        gStringVar1.as_mut_ptr(),
                        gText_SpaceAndSpace.as_ptr().cast_mut(),
                    );
                } else {
                    StringAppend(
                        gStringVar1.as_mut_ptr(),
                        gText_CommaSpace.as_ptr().cast_mut(),
                    );
                }
                StringAppend(
                    gStringVar1.as_mut_ptr(),
                    gText_LineBreak.as_ptr().cast_mut(),
                );
            }
        }
        StringAppend(
            gStringVar1.as_mut_ptr(),
            gSpeciesNames[species].as_ptr().cast_mut(),
        );
    }
    return count;
}
pub(crate) unsafe extern "C" fn AppendIfValid(
    species: u16,
    heldItem: u16,
    hp: u16,
    lvlMode: u8,
    monLevel: u8,
    mut speciesArray: *mut u16,
    mut itemsArray: *mut u16,
    count: *mut u8,
) {
    let mut i: i32 = 0;
    if species == SPECIES_EGG as u16 || species == SPECIES_NONE {
        return;
    }
    i = 0;
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
pub(crate) unsafe extern "C" fn CheckPartyIneligibility() {
    let mut speciesArray: CArray<u16, 6> = zeroed();
    let mut itemArray: CArray<u16, 6> = zeroed();
    let mut monId: i32 = 0;
    let mut toChoose: i32 = 0;
    let mut count: u8 = 0;
    let mut battleMode: i32 = VarGet(VAR_FRONTIER_BATTLE_MODE) as i32;
    let mut monIdLooper: i32 = 0;
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
    monIdLooper = 0;
    loop {
        monId = monIdLooper;
        count = 0;
        loop {
            let mut species: u16 =
                GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_SPECIES_OR_EGG) as u16;
            let mut heldItem: u16 =
                GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_HELD_ITEM) as u16;
            let mut level: u8 = GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_LEVEL) as u8;
            let mut hp: u16 = GetMonData2(&raw mut gPlayerParty[monId], MON_DATA_HP) as u16;
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
        let mut i: i32 = 0;
        let mut caughtBannedMons: i32 = 0;
        let mut species: i32 = gFrontierBannedSpecies[0] as i32;
        i = 0;
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
            StringAppend(gStringVar1.as_mut_ptr(), gText_Space2.as_ptr().cast_mut());
            StringAppend(gStringVar1.as_mut_ptr(), gText_Are.as_ptr().cast_mut());
        } else {
            if count as i32 & 1 != 0 {
                StringAppend(
                    gStringVar1.as_mut_ptr(),
                    gText_LineBreak.as_ptr().cast_mut(),
                );
            } else {
                StringAppend(gStringVar1.as_mut_ptr(), gText_Space2.as_ptr().cast_mut());
            }
            StringAppend(gStringVar1.as_mut_ptr(), gText_Are2.as_ptr().cast_mut());
        }
    } else {
        gSpecialVar_0x8004 = FALSE as u16;
        (*gSaveBlock2Ptr)
            .frontier
            .set_lvlMode(gSpecialVar_Result as u8);
    }
}
pub(crate) unsafe extern "C" fn ValidateVisitingTrainer() {
    ValidateEReaderTrainer();
}
pub(crate) unsafe extern "C" fn IncrementWinStreak() {
    let mut lvlMode: i32 = (*gSaveBlock2Ptr).frontier.lvlMode() as i32;
    let mut battleMode: i32 = VarGet(VAR_FRONTIER_BATTLE_MODE) as i32;
    let mut facility: i32 = VarGet(VAR_FRONTIER_FACILITY) as i32;
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
        FRONTIER_FACILITY_PYRAMID => {
            if (*gSaveBlock2Ptr).frontier.pyramidWinStreaks[lvlMode] < MAX_STREAK {
                (*gSaveBlock2Ptr).frontier.pyramidWinStreaks[lvlMode] += 1;
            }
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn RestoreHeldItems() {
    let mut i: u8 = 0;
    i = 0;
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
pub(crate) unsafe extern "C" fn SaveRecordBattle() {
    gSpecialVar_Result = MoveRecordedBattleToSaveData() as u16;
    (*gSaveBlock2Ptr).frontier.set_disableRecordBattle(TRUE);
}
pub(crate) unsafe extern "C" fn BufferFrontierTrainerName() {
    match gSpecialVar_0x8005 {
        0 => {
            GetFrontierTrainerName(gStringVar1.as_mut_ptr(), gTrainerBattleOpponent_A);
        }
        1 => {
            GetFrontierTrainerName(gStringVar2.as_mut_ptr(), gTrainerBattleOpponent_A);
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn ResetSketchedMoves() {
    let mut i: u8 = 0;
    let mut j: u8 = 0;
    let mut k: u8 = 0;
    i = 0;
    while (i as i32)
        < (if 3 >= (if 4 >= 2 { 4 } else { 2 }) {
            3
        } else {
            if 4 >= 2 { 4 } else { 2 }
        })
    {
        let mut monId: u16 = (*gSaveBlock2Ptr).frontier.selectedPartyMons[i] - 1;
        if monId < PARTY_SIZE as u16 {
            j = 0;
            while j < MAX_MON_MOVES as u8 {
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
                j += 1;
            }
            (*gSaveBlock1Ptr).playerParty
                [(*gSaveBlock2Ptr).frontier.selectedPartyMons[i] as i32 - 1] = gPlayerParty[i];
        }
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn SetFacilityBrainObjectEvent() {
    SetFrontierBrainObjEventGfx(VarGet(VAR_FRONTIER_FACILITY) as u8);
}
pub(crate) unsafe extern "C" fn Print1PRecord(
    position: i32,
    x: i32,
    y: i32,
    hallRecord: *mut RankingHall1P,
    hallFacilityId: i32,
) {
    let mut text: CArray<u8, 32> = zeroed();
    let mut winStreak: u16 = 0;
    AddTextPrinterParameterized(
        gRecordsWindowId,
        FONT_NORMAL,
        gText_123Dot[position].as_ptr().cast_mut(),
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
            gRecordsWindowId,
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
            gRecordsWindowId,
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
pub(crate) unsafe extern "C" fn Print2PRecord(
    position: i32,
    x: i32,
    y: i32,
    hallRecord: *mut RankingHall2P,
) {
    let mut text: CArray<u8, 32> = zeroed();
    let mut winStreak: u16 = 0;
    AddTextPrinterParameterized(
        gRecordsWindowId,
        FONT_NORMAL,
        gText_123Dot[position].as_ptr().cast_mut(),
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
            gRecordsWindowId,
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
            gRecordsWindowId,
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
            gRecordsWindowId,
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
pub(crate) unsafe extern "C" fn Fill1PRecords(
    mut dst: *mut RankingHall1P,
    hallFacilityId: i32,
    lvlMode: i32,
) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut record1P: CArray<RankingHall1P, 4> = zeroed();
    let mut playerHallRecords: *mut PlayerHallRecords = AllocZeroed(344) as *mut PlayerHallRecords;
    GetPlayerHallRecords(playerHallRecords);
    i = 0;
    while i < HALL_RECORDS_COUNT {
        record1P[i] = (*gSaveBlock2Ptr).hallRecords1P[hallFacilityId][lvlMode][i];
        i += 1;
    }
    record1P[3] = (*playerHallRecords).onePlayer[hallFacilityId][lvlMode];
    i = 0;
    while i < HALL_RECORDS_COUNT {
        let mut highestWinStreak: i32 = 0;
        let mut highestId: i32 = 0;
        j = 0;
        while j < 4 {
            if record1P[j].winStreak as i32 > highestWinStreak {
                highestId = j;
                highestWinStreak = record1P[j].winStreak as i32;
            }
            j += 1;
        }
        if record1P[3].winStreak as i32 >= highestWinStreak {
            highestId = HALL_RECORDS_COUNT;
        }
        *dst.at(i) = record1P[highestId];
        record1P[highestId].winStreak = 0;
        i += 1;
    }
    Free(playerHallRecords as *mut c_void);
}
pub(crate) unsafe extern "C" fn Fill2PRecords(mut dst: *mut RankingHall2P, lvlMode: i32) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut record2P: CArray<RankingHall2P, 4> = zeroed();
    let mut playerHallRecords: *mut PlayerHallRecords = AllocZeroed(344) as *mut PlayerHallRecords;
    GetPlayerHallRecords(playerHallRecords);
    i = 0;
    while i < HALL_RECORDS_COUNT {
        record2P[i] = (*gSaveBlock2Ptr).hallRecords2P[lvlMode][i];
        i += 1;
    }
    record2P[3] = (*playerHallRecords).twoPlayers[lvlMode];
    i = 0;
    while i < HALL_RECORDS_COUNT {
        let mut highestWinStreak: i32 = 0;
        let mut highestId: i32 = 0;
        j = 0;
        while j < HALL_RECORDS_COUNT {
            if record2P[j].winStreak as i32 > highestWinStreak {
                highestId = j;
                highestWinStreak = record2P[j].winStreak as i32;
            }
            j += 1;
        }
        if record2P[3].winStreak as i32 >= highestWinStreak {
            highestId = HALL_RECORDS_COUNT;
        }
        *dst.at(i) = record2P[highestId];
        record2P[highestId].winStreak = 0;
        i += 1;
    }
    Free(playerHallRecords as *mut c_void);
}
pub(crate) unsafe extern "C" fn PrintHallRecords(hallFacilityId: i32, lvlMode: i32) {
    let mut i: i32 = 0;
    let mut x: i32 = 0;
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
        gRecordsWindowId,
        FONT_NORMAL,
        gStringVar4.as_mut_ptr(),
        0,
        1,
        TEXT_SKIP_DRAW,
        None,
    );
    x = GetStringRightAlignXOffset(FONT_NORMAL as i32, sLevelModeText[lvlMode], 208);
    AddTextPrinterParameterized(
        gRecordsWindowId,
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
        i = 0;
        while i < HALL_RECORDS_COUNT {
            Print2PRecord(i, 1, 4, &raw mut records2P[i]);
            i += 1;
        }
    } else {
        Fill1PRecords(records1P.as_mut_ptr(), hallFacilityId, lvlMode);
        i = 0;
        while i < HALL_RECORDS_COUNT {
            Print1PRecord(i, 1, 4, &raw mut records1P[i], hallFacilityId);
            i += 1;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ShowRankingHallRecordsWindow() {
    gRecordsWindowId = AddWindow((&raw const *sRankingHallRecordsWindowTemplate).cast_mut()) as u8;
    DrawStdWindowFrame(gRecordsWindowId, FALSE);
    FillWindowPixelBuffer(gRecordsWindowId, 17);
    PrintHallRecords(gSpecialVar_0x8005 as i32, FRONTIER_LVL_50 as i32);
    PutWindowTilemap(gRecordsWindowId);
    CopyWindowToVram(gRecordsWindowId, COPYWIN_FULL);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrollRankingHallRecordsWindow() {
    FillWindowPixelBuffer(gRecordsWindowId, 17);
    PrintHallRecords(gSpecialVar_0x8005 as i32, FRONTIER_LVL_OPEN as i32);
    CopyWindowToVram(gRecordsWindowId, COPYWIN_GFX);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ClearRankingHallRecords() {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut emptyId: CArray<u8, 4> = CArray([0, 0, 0, 0]);
    i = 0;
    while i < HALL_FACILITIES_COUNT {
        j = 0;
        while j < FRONTIER_LVL_MODE_COUNT {
            k = 0;
            while k < HALL_RECORDS_COUNT {
                CopyTrainerId(
                    (*gSaveBlock2Ptr).hallRecords1P[i][j][k].id.as_mut_ptr(),
                    emptyId.as_mut_ptr(),
                );
                (*gSaveBlock2Ptr).hallRecords1P[i][j][k].name[0] = EOS;
                (*gSaveBlock2Ptr).hallRecords1P[i][j][k].winStreak = 0;
                k += 1;
            }
            j += 1;
        }
        i += 1;
    }
    j = 0;
    while j < FRONTIER_LVL_MODE_COUNT {
        k = 0;
        while k < HALL_RECORDS_COUNT {
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
            k += 1;
        }
        j += 1;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SaveGameFrontier() {
    let mut i: i32 = 0;
    let mut monsParty: *mut Pokemon = AllocZeroed(600) as *mut Pokemon;
    i = 0;
    while i < PARTY_SIZE {
        *monsParty.at(i) = gPlayerParty[i];
        i += 1;
    }
    i = gPlayerPartyCount as i32;
    LoadPlayerParty();
    SetContinueGameWarpStatusToDynamicWarp();
    TrySavingData(SAVE_LINK);
    ClearContinueGameWarpStatus2();
    gPlayerPartyCount = i as u8;
    i = 0;
    while i < PARTY_SIZE {
        gPlayerParty[i] = *monsParty.at(i);
        i += 1;
    }
    Free(monsParty as *mut c_void);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetFrontierBrainTrainerPicIndex() -> u8 {
    let mut facility: i32 = 0;
    if gBattleTypeFlags & BATTLE_TYPE_RECORDED != 0 {
        facility = GetRecordedBattleFrontierFacility() as i32;
    } else {
        facility = VarGet(VAR_FRONTIER_FACILITY) as i32;
    }
    return gTrainers[sFrontierBrainTrainerIds[facility]].trainerPic;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetFrontierBrainTrainerClass() -> u8 {
    let mut facility: i32 = 0;
    if gBattleTypeFlags & BATTLE_TYPE_RECORDED != 0 {
        facility = GetRecordedBattleFrontierFacility() as i32;
    } else {
        facility = VarGet(VAR_FRONTIER_FACILITY) as i32;
    }
    return gTrainers[sFrontierBrainTrainerIds[facility]].trainerClass;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CopyFrontierBrainTrainerName(mut dst: *mut u8) {
    let mut i: i32 = 0;
    let mut facility: i32 = 0;
    if gBattleTypeFlags & BATTLE_TYPE_RECORDED != 0 {
        facility = GetRecordedBattleFrontierFacility() as i32;
    } else {
        facility = VarGet(VAR_FRONTIER_FACILITY) as i32;
    }
    i = 0;
    while i < PLAYER_NAME_LENGTH {
        *dst.at(i) = gTrainers[sFrontierBrainTrainerIds[facility]].trainerName[i];
        i += 1;
    }
    *dst.at(i) = EOS;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsFrontierBrainFemale() -> u8 {
    let mut facility: i32 = VarGet(VAR_FRONTIER_FACILITY) as i32;
    return sFrontierBrainObjEventGfx[facility][1];
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetFrontierBrainObjEventGfx_2() {
    let mut facility: i32 = VarGet(VAR_FRONTIER_FACILITY) as i32;
    VarSet(
        VAR_OBJ_GFX_ID_0,
        sFrontierBrainObjEventGfx[facility][0] as u16,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CreateFrontierBrainPokemon() {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut selectedMonBits: i32 = 0;
    let mut monPartyId: i32 = 0;
    let mut monLevel: i32 = 0;
    let mut friendship: u8 = 0;
    let mut facility: i32 = VarGet(VAR_FRONTIER_FACILITY) as i32;
    let mut symbol: i32 = GetFronterBrainSymbol();
    if facility == FRONTIER_FACILITY_DOME {
        selectedMonBits =
            GetDomeTrainerSelectedMons(TrainerIdToDomeTournamentId(TRAINER_FRONTIER_BRAIN) as u16);
    } else {
        selectedMonBits = 7;
    }
    ZeroEnemyPartyMons();
    monPartyId = 0;
    monLevel = SetFacilityPtrsGetLevel() as i32;
    i = 0;
    while i < FRONTIER_PARTY_SIZE {
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
            j = 0;
            while j < MAX_MON_MOVES {
                SetMonMoveSlot(
                    &raw mut gEnemyParty[monPartyId],
                    sFrontierBrainsMons[facility][symbol][i].moves[j],
                    j as u8,
                );
                if sFrontierBrainsMons[facility][symbol][i].moves[j] == MOVE_FRUSTRATION {
                    friendship = 0;
                }
                j += 1;
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
        i += 1;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetFrontierBrainMonSpecies(monId: u8) -> u16 {
    let mut facility: i32 = VarGet(VAR_FRONTIER_FACILITY) as i32;
    let mut symbol: i32 = GetFronterBrainSymbol();
    return sFrontierBrainsMons[facility][symbol][monId].species;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetFrontierBrainObjEventGfx(facility: u8) {
    gTrainerBattleOpponent_A = TRAINER_FRONTIER_BRAIN;
    VarSet(
        VAR_OBJ_GFX_ID_0,
        sFrontierBrainObjEventGfx[facility][0] as u16,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetFrontierBrainMonMove(monId: u8, moveSlotId: u8) -> u16 {
    let mut facility: i32 = VarGet(VAR_FRONTIER_FACILITY) as i32;
    let mut symbol: i32 = GetFronterBrainSymbol();
    return sFrontierBrainsMons[facility][symbol][monId].moves[moveSlotId];
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetFrontierBrainMonNature(monId: u8) -> u8 {
    let mut facility: i32 = VarGet(VAR_FRONTIER_FACILITY) as i32;
    let mut symbol: i32 = GetFronterBrainSymbol();
    return sFrontierBrainsMons[facility][symbol][monId].nature;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetFrontierBrainMonEvs(monId: u8, evStatId: u8) -> u8 {
    let mut facility: i32 = VarGet(VAR_FRONTIER_FACILITY) as i32;
    let mut symbol: i32 = GetFronterBrainSymbol();
    return sFrontierBrainsMons[facility][symbol][monId].evs[evStatId];
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetFronterBrainSymbol() -> i32 {
    let mut facility: i32 = VarGet(VAR_FRONTIER_FACILITY) as i32;
    let mut symbol: i32 = GetPlayerSymbolCountForFacility(facility as u8) as i32;
    if symbol == 2 {
        let mut winStreak: u16 = GetCurrentFacilityWinStreak() as u16;
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
    return symbol;
}
pub(crate) unsafe extern "C" fn CopyFrontierBrainText(playerWonText: u8) {
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
