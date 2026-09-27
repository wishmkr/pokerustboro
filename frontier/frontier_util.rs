//! Translated from `src/frontier_util.c` by tools/rustport/c2rs.py, then reviewed.
#![allow(
    non_snake_case,
    non_upper_case_globals,
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
    clippy::all,
    clashing_extern_declarations,
    unpredictable_function_pointer_comparisons
)]

// Data tables (translate with cdata.py): sFrontierBrainStreakAppearances sFrontierBrainsMons sBattlePointAwards sBattledBrainBitFlags sFrontierUtilFuncs sFrontierResultsWindowTemplate sLinkContestResultsWindowTemplate sRankingHallRecordsWindowTemplate sFrontierBrainObjEventGfx gFrontierBannedSpecies sRecordsWindowChallengeTexts sLevelModeText sHallFacilityToRecordsText sFrontierBrainTrainerIds sFrontierBrainPlayerLostSilverTexts sFrontierBrainPlayerWonSilverTexts sFrontierBrainPlayerLostGoldTexts sFrontierBrainPlayerWonGoldTexts sFrontierBrainPlayerLostTexts sFrontierBrainPlayerWonTexts
#[allow(unused_imports)]
use crate::data::frontier_util::*;

unsafe extern "C" {
    static mut gApprentices: u8;
    static mut gBattleFrontierTrainers: u8;
    static mut gBattleOutcome: u8;
    static mut gBattleScripting: u8;
    static mut gBattleTypeFlags: u8;
    static mut gEnemyParty: u8;
    static mut gFacilityTrainers: u8;
    static mut gLinkPlayers: u8;
    static mut gPlayerParty: u8;
    static mut gPlayerPartyCount: u8;
    static mut gRecordsWindowId: u8;
    static mut gSaveBlock1Ptr: u8;
    static mut gSaveBlock2Ptr: u8;
    static mut gSelectedOrderFromParty: u8;
    static mut gSpecialVar_0x8004: u8;
    static mut gSpecialVar_0x8005: u8;
    static mut gSpecialVar_0x8006: u8;
    static mut gSpecialVar_Result: u8;
    static mut gSpeciesNames: u8;
    static mut gStringVar1: u8;
    static mut gStringVar2: u8;
    static mut gStringVar4: u8;
    static mut gText_123Dot: u8;
    static mut gText_1st: u8;
    static mut gText_2nd: u8;
    static mut gText_3rd: u8;
    static mut gText_4th: u8;
    static mut gText_Are: u8;
    static mut gText_Are2: u8;
    static mut gText_BattleChoiceResults: u8;
    static mut gText_BattleQuestResults: u8;
    static mut gText_BattleSwapDoubleResults: u8;
    static mut gText_BattleSwapSingleResults: u8;
    static mut gText_Beauty: u8;
    static mut gText_Championships: u8;
    static mut gText_ClearStreak: u8;
    static mut gText_CommaSpace: u8;
    static mut gText_Cool: u8;
    static mut gText_Current: u8;
    static mut gText_Cute: u8;
    static mut gText_DoubleBattleHallResults: u8;
    static mut gText_DoubleBattleRoomResults: u8;
    static mut gText_DoubleBattleTourneyResults: u8;
    static mut gText_FloorsCleared: u8;
    static mut gText_KOsInARow: u8;
    static mut gText_LineBreak: u8;
    static mut gText_LinkContestResults: u8;
    static mut gText_LinkMultiBattleRoomResults: u8;
    static mut gText_Lv502: u8;
    static mut gText_MultiBattleRoomResults: u8;
    static mut gText_NewLine: u8;
    static mut gText_OpenLv: u8;
    static mut gText_Prev: u8;
    static mut gText_Record: u8;
    static mut gText_RentalSwap: u8;
    static mut gText_RoomsCleared: u8;
    static mut gText_SetKOTourneyResults: u8;
    static mut gText_SingleBattleHallResults: u8;
    static mut gText_SingleBattleRoomResults: u8;
    static mut gText_SingleBattleTourneyResults: u8;
    static mut gText_Smart: u8;
    static mut gText_Space2: u8;
    static mut gText_SpaceAndSpace: u8;
    static mut gText_TimesCleared: u8;
    static mut gText_TimesVar1: u8;
    static mut gText_Total: u8;
    static mut gText_Tough: u8;
    static mut gText_WinStreak: u8;
    static mut gTrainerBattleOpponent_A: u8;
    static mut gTrainers: u8;
    fn AddTextPrinterParameterized(
        a0: u8,
        a1: u8,
        a2: *mut u8,
        a3: u8,
        a4: u8,
        a5: u8,
        a6: Option<unsafe extern "C" fn(*mut u8, u16)>,
    ) -> u16;
    fn AddWindow(a0: *mut u8) -> u16;
    fn AllocZeroed(a0: u32) -> *mut u8;
    fn BufferApprenticeChallengeText(a0: u8);
    fn CalculateMonStats(a0: *mut u8);
    fn ClearContinueGameWarpStatus2();
    fn ClearSelectedPartyOrder();
    fn ConvertIntToDecimalStringN(a0: *mut u8, a1: i32, a2: i32, a3: u8) -> *mut u8;
    fn CopyTrainerId(a0: *mut u8, a1: *mut u8);
    fn CopyWindowToVram(a0: u8, a1: u8);
    fn CreateMon(a0: *mut u8, a1: u16, a2: u8, a3: u8, a4: u8, a5: u32, a6: u8, a7: u32);
    fn DoSoftReset();
    fn DrawStdWindowFrame(a0: u8, a1: u8);
    fn FillWindowPixelBuffer(a0: u8, a1: u8);
    fn FlagGet(a0: u16) -> u8;
    fn FlagSet(a0: u16) -> u8;
    fn Free(a0: *mut u8);
    fn FrontierGamblerSetWonOrLost(a0: u8);
    fn FrontierSpeechToString(a0: *mut u16);
    fn GetDomeTrainerSelectedMons(a0: u16) -> i32;
    fn GetFrontierTrainerName(a0: *mut u8, a1: u16);
    fn GetMonData2(a0: *mut u8, a1: i32) -> u32;
    fn GetMonData3(a0: *mut u8, a1: i32, a2: *mut u8) -> u32;
    fn GetNatureFromPersonality(a0: u32) -> u8;
    fn GetPlayerHallRecords(a0: *mut u8);
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
    fn SetMonData(a0: *mut u8, a1: i32, a2: *mut u8);
    fn SetMonMoveSlot(a0: *mut u8, a1: u16, a2: u8);
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
    unsafe {
        (((((&raw const sFrontierUtilFuncs)
            .cast::<u8>()
            .cast_mut()
            .cast::<Option<unsafe extern "C" fn()>>())
        .cast::<Option<unsafe extern "C" fn()>>())
        .wrapping_offset(
            ((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32) as isize,
        ))
        .read())
        .unwrap_unchecked()();
    }
}
pub(crate) unsafe extern "C" fn GetChallengeStatus() {
    unsafe {
        VarSet(16384u16, 255u16);
        'l1: {
            let __sw1 = (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                .wrapping_add(1612))
            .wrapping_add(1628))
            .read()) as i32);
            if __sw1 == 0i32 {
                break 'l1;
            }
            if __sw1 == 1i32 {
                FrontierGamblerSetWonOrLost(0u8);
                VarSet(
                    16384u16,
                    (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                        .wrapping_add(1628))
                    .read()) as u16),
                );
                break 'l1;
            }
            if __sw1 == 4i32 {
                FrontierGamblerSetWonOrLost(0u8);
                VarSet(
                    16384u16,
                    (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                        .wrapping_add(1628))
                    .read()) as u16),
                );
                break 'l1;
            }
            if __sw1 == 3i32 {
                FrontierGamblerSetWonOrLost(1u8);
                VarSet(
                    16384u16,
                    (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                        .wrapping_add(1628))
                    .read()) as u16),
                );
                break 'l1;
            }
            if __sw1 == 2i32 {
                VarSet(
                    16384u16,
                    (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                        .wrapping_add(1628))
                    .read()) as u16),
                );
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn GetFrontierData() {
    unsafe {
        let mut facility: u8 = ((VarGet(16591u16)) as u8);
        let mut hasSymbol: u8 = GetPlayerSymbolCountForFacility(facility);
        if ((hasSymbol) as i32) == 2i32 {
            hasSymbol = 1u8;
        }
        'l1: {
            let __sw1 = ((((&raw mut gSpecialVar_0x8005).cast::<u16>()).read()) as i32);
            if __sw1 == 0i32 {
                ((&raw mut gSpecialVar_Result).cast::<u16>()).write(
                    (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                        .wrapping_add(1628))
                    .read()) as u16),
                );
                break 'l1;
            }
            if __sw1 == 1i32 {
                ((&raw mut gSpecialVar_Result).cast::<u16>()).write(
                    ((crate::c::bf_read(
                        ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                            .wrapping_add(1629),
                        0,
                        2,
                        false,
                    ) as u8) as u16),
                );
                break 'l1;
            }
            if __sw1 == 2i32 {
                ((&raw mut gSpecialVar_Result).cast::<u16>()).write(
                    (((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                        .wrapping_add(1638)
                        .cast::<u16>())
                    .read(),
                );
                break 'l1;
            }
            if __sw1 == 3i32 {
                ((&raw mut gSpecialVar_Result).cast::<u16>()).write(
                    ((crate::c::bf_read(
                        ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                            .wrapping_add(1629),
                        2,
                        1,
                        false,
                    ) as u8) as u16),
                );
                break 'l1;
            }
            if __sw1 == 5i32 {
                ((&raw mut gSpecialVar_Result).cast::<u16>())
                    .write(((((&raw mut gBattleOutcome).cast::<u8>()).read()) as u16));
                ((&raw mut gBattleOutcome).cast::<u8>()).write(0u8);
                break 'l1;
            }
            if __sw1 == 6i32 {
                ((&raw mut gSpecialVar_Result).cast::<u16>()).write(
                    ((crate::c::bf_read(
                        ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                            .wrapping_add(1629),
                        3,
                        1,
                        false,
                    ) as u8) as u16),
                );
                break 'l1;
            }
            if __sw1 == 7i32 {
                ((&raw mut gSpecialVar_Result).cast::<u16>()).write(
                    (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(1612))
                    .wrapping_add(1716)
                    .cast::<u16>())
                    .read()) as i32)
                        & ((((((((&raw const sBattledBrainBitFlags).cast::<u8>().cast_mut())
                            .cast::<u8>())
                        .wrapping_offset(((facility) as i32) as isize * 4))
                        .cast::<u16>())
                        .wrapping_offset(((hasSymbol) as i32) as isize))
                        .read()) as i32)) as u16),
                );
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SetFrontierData() {
    unsafe {
        let mut i: i32 = 0i32;
        let mut facility: u8 = ((VarGet(16591u16)) as u8);
        let mut hasSymbol: u8 = GetPlayerSymbolCountForFacility(facility);
        if ((hasSymbol) as i32) == 2i32 {
            hasSymbol = 1u8;
        }
        'l1: {
            let __sw1 = ((((&raw mut gSpecialVar_0x8005).cast::<u16>()).read()) as i32);
            if __sw1 == 0i32 {
                (((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                    .wrapping_add(1628))
                .write(((((&raw mut gSpecialVar_0x8006).cast::<u16>()).read()) as u8));
                break 'l1;
            }
            if __sw1 == 1i32 {
                crate::c::bf_write(
                    ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                        .wrapping_add(1629),
                    0,
                    2,
                    ((((&raw mut gSpecialVar_0x8006).cast::<u16>()).read()) as u8) as i32,
                );
                break 'l1;
            }
            if __sw1 == 2i32 {
                (((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                    .wrapping_add(1638)
                    .cast::<u16>())
                .write(((&raw mut gSpecialVar_0x8006).cast::<u16>()).read());
                break 'l1;
            }
            if __sw1 == 3i32 {
                crate::c::bf_write(
                    ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                        .wrapping_add(1629),
                    2,
                    1,
                    ((((&raw mut gSpecialVar_0x8006).cast::<u16>()).read()) as u8) as i32,
                );
                break 'l1;
            }
            if __sw1 == 4i32 {
                {
                    i = 0i32;
                    'l2: loop {
                        if !(i
                            < (if 3i32 >= (if 4i32 >= 2i32 { 4i32 } else { 2i32 }) {
                                3i32
                            } else {
                                (if 4i32 >= 2i32 { 4i32 } else { 2i32 })
                            }))
                        {
                            break 'l2;
                        }
                        'l3: {
                            (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(1612))
                            .wrapping_add(1630))
                            .cast::<u16>())
                            .wrapping_offset((i) as isize))
                            .write(
                                (((((&raw mut gSelectedOrderFromParty).cast::<u8>())
                                    .wrapping_offset((i) as isize))
                                .read()) as u16),
                            );
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                break 'l1;
            }
            if __sw1 == 6i32 {
                crate::c::bf_write(
                    ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                        .wrapping_add(1629),
                    3,
                    1,
                    ((((&raw mut gSpecialVar_0x8006).cast::<u16>()).read()) as u8) as i32,
                );
                break 'l1;
            }
            if __sw1 == 7i32 {
                let __p2 = ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                    .wrapping_add(1612))
                .wrapping_add(1716)
                .cast::<u16>();
                (__p2).write(
                    (((((__p2).read()) as i32)
                        | ((((((((&raw const sBattledBrainBitFlags).cast::<u8>().cast_mut())
                            .cast::<u8>())
                        .wrapping_offset(((facility) as i32) as isize * 4))
                        .cast::<u16>())
                        .wrapping_offset(((hasSymbol) as i32) as isize))
                        .read()) as i32)) as u16),
                );
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SetSelectedPartyOrder() {
    unsafe {
        let mut i: i32 = 0i32;
        ClearSelectedPartyOrder();
        {
            i = 0i32;
            'l1: loop {
                if !(i < ((((&raw mut gSpecialVar_0x8005).cast::<u16>()).read()) as i32)) {
                    break 'l1;
                }
                'l2: {
                    (((&raw mut gSelectedOrderFromParty).cast::<u8>())
                        .wrapping_offset((i) as isize))
                    .write(
                        (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(1612))
                        .wrapping_add(1630))
                        .cast::<u16>())
                        .wrapping_offset((i) as isize))
                        .read()) as u8),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        ReducePlayerPartyToSelectedMons();
    }
}
pub(crate) unsafe extern "C" fn DoSoftReset_() {
    unsafe {
        DoSoftReset();
    }
}
pub(crate) unsafe extern "C" fn SetFrontierTrainers() {
    unsafe {
        ((&raw mut gFacilityTrainers).cast::<*mut u8>())
            .write((&raw mut gBattleFrontierTrainers).cast::<u8>());
    }
}
pub(crate) unsafe extern "C" fn SaveSelectedParty() {
    unsafe {
        let mut i: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32)
                    < (if 3i32 >= (if 4i32 >= 2i32 { 4i32 } else { 2i32 }) {
                        3i32
                    } else {
                        (if 4i32 >= 2i32 { 4i32 } else { 2i32 })
                    }))
                {
                    break 'l1;
                }
                'l2: {
                    let mut monId: u16 =
                        (((((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(1612))
                        .wrapping_add(1630))
                        .cast::<u16>())
                        .wrapping_offset(((i) as i32) as isize))
                        .read()) as i32)
                            .wrapping_sub(1i32)) as u16);
                    if ((monId) as i32) < 6i32 {
                        (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(568))
                        .cast::<u8>())
                        .wrapping_offset(
                            ((((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(1612))
                            .wrapping_add(1630))
                            .cast::<u16>())
                            .wrapping_offset(((i) as i32) as isize))
                            .read()) as i32)
                                .wrapping_sub(1i32)) as isize
                                * 100,
                        )
                        .cast::<crate::c::Rec4<100>>()
                        .write_unaligned(
                            ((&raw mut gPlayerParty).cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize * 100)
                                .cast::<crate::c::Rec4<100>>()
                                .read_unaligned(),
                        );
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn ShowFacilityResultsWindow() {
    unsafe {
        if ((((&raw mut gSpecialVar_0x8006).cast::<u16>()).read()) as i32) >= 4i32 {
            ((&raw mut gSpecialVar_0x8006).cast::<u16>()).write(0u16);
        }
        'l1: {
            let __sw1 = ((((&raw mut gSpecialVar_0x8005).cast::<u16>()).read()) as i32);
            if __sw1 == 0i32 {
                ShowTowerResultsWindow(
                    ((((&raw mut gSpecialVar_0x8006).cast::<u16>()).read()) as u8),
                );
                break 'l1;
            }
            if __sw1 == 1i32 {
                ShowDomeResultsWindow(
                    ((((&raw mut gSpecialVar_0x8006).cast::<u16>()).read()) as u8),
                );
                break 'l1;
            }
            if __sw1 == 2i32 {
                ShowPalaceResultsWindow(
                    ((((&raw mut gSpecialVar_0x8006).cast::<u16>()).read()) as u8),
                );
                break 'l1;
            }
            if __sw1 == 5i32 {
                ShowPikeResultsWindow();
                break 'l1;
            }
            if __sw1 == 4i32 {
                ShowFactoryResultsWindow(
                    ((((&raw mut gSpecialVar_0x8006).cast::<u16>()).read()) as u8),
                );
                break 'l1;
            }
            if __sw1 == 3i32 {
                ShowArenaResultsWindow();
                break 'l1;
            }
            if __sw1 == 6i32 {
                ShowPyramidResultsWindow();
                break 'l1;
            }
            if __sw1 == 7i32 {
                ShowLinkContestResultsWindow();
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn IsWinStreakActive(challenge: u32) -> u8 {
    unsafe {
        let mut challenge = challenge;
        if ((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
            .wrapping_add(1680)
            .cast::<u32>())
        .read()
            & challenge)
            != 0
        {
            return 1u8;
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
pub(crate) unsafe extern "C" fn PrintAligned(str: *mut u8, y: i32) {
    unsafe {
        let mut str = str;
        let mut y = y;
        let mut x: i32 = GetStringCenterAlignXOffset(1i32, str, 224i32);
        y = ((y).wrapping_mul(8i32)).wrapping_add(1i32);
        AddTextPrinterParameterized(
            ((&raw mut gRecordsWindowId).cast::<u8>()).read(),
            1u8,
            str,
            ((x) as u8),
            ((y) as u8),
            255u8,
            None,
        );
    }
}
pub(crate) unsafe extern "C" fn PrintHyphens(y: i32) {
    unsafe {
        let mut y = y;
        let mut i: i32 = 0i32;
        let mut text = crate::ffi::Align4([0u8; 37]);
        {
            i = 0i32;
            'l1: loop {
                if !(i < ((crate::c::div_u32(37u32, 1u32)) as i32).wrapping_sub(1i32)) {
                    break 'l1;
                }
                'l2: {
                    (((&raw mut text).cast::<u8>()).wrapping_offset((i) as isize)).write(174u8);
                }
                i = (i).wrapping_add(1);
            }
        }
        (((&raw mut text).cast::<u8>()).wrapping_offset((i) as isize)).write(255u8);
        y = ((y).wrapping_mul(8i32)).wrapping_add(1i32);
        AddTextPrinterParameterized(
            ((&raw mut gRecordsWindowId).cast::<u8>()).read(),
            1u8,
            (&raw mut text).cast::<u8>(),
            4u8,
            ((y) as u8),
            255u8,
            None,
        );
    }
}
pub(crate) unsafe extern "C" fn TowerPrintStreak(str: *mut u8, num: u16, x1: u8, x2: u8, y: u8) {
    unsafe {
        let mut str = str;
        let mut num = num;
        let mut x1 = x1;
        let mut x2 = x2;
        let mut y = y;
        AddTextPrinterParameterized(
            ((&raw mut gRecordsWindowId).cast::<u8>()).read(),
            1u8,
            str,
            x1,
            y,
            255u8,
            None,
        );
        if ((num) as i32) > 9999i32 {
            num = 9999u16;
        }
        ConvertIntToDecimalStringN(
            (&raw mut gStringVar1).cast::<u8>(),
            ((num) as i32),
            1i32,
            4u8,
        );
        StringExpandPlaceholders(
            (&raw mut gStringVar4).cast::<u8>(),
            (&raw mut gText_WinStreak).cast::<u8>(),
        );
        AddTextPrinterParameterized(
            ((&raw mut gRecordsWindowId).cast::<u8>()).read(),
            1u8,
            (&raw mut gStringVar4).cast::<u8>(),
            x2,
            y,
            255u8,
            None,
        );
    }
}
pub(crate) unsafe extern "C" fn TowerPrintRecordStreak(
    battleMode: u8,
    lvlMode: u8,
    x1: u8,
    x2: u8,
    y: u8,
) {
    unsafe {
        let mut battleMode = battleMode;
        let mut lvlMode = lvlMode;
        let mut x1 = x1;
        let mut x2 = x2;
        let mut y = y;
        let mut num: u16 = (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
            .wrapping_add(1612))
        .wrapping_add(1700))
        .cast::<u8>())
        .wrapping_offset(((battleMode) as i32) as isize * 4))
        .cast::<u16>())
        .wrapping_offset(((lvlMode) as i32) as isize))
        .read();
        TowerPrintStreak((&raw mut gText_Record).cast::<u8>(), num, x1, x2, y);
    }
}
pub(crate) unsafe extern "C" fn TowerGetWinStreak(battleMode: u8, lvlMode: u8) -> u16 {
    unsafe {
        let mut battleMode = battleMode;
        let mut lvlMode = lvlMode;
        let mut winStreak: u16 = (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
            .wrapping_add(1612))
        .wrapping_add(1684))
        .cast::<u8>())
        .wrapping_offset(((battleMode) as i32) as isize * 4))
        .cast::<u16>())
        .wrapping_offset(((lvlMode) as i32) as isize))
        .read();
        if ((winStreak) as i32) > 9999i32 {
            return 9999u16;
        } else {
            return winStreak;
        }
        #[allow(unreachable_code)]
        {
            return 0u16;
        }
    }
}
pub(crate) unsafe extern "C" fn TowerPrintPrevOrCurrentStreak(
    battleMode: u8,
    lvlMode: u8,
    x1: u8,
    x2: u8,
    y: u8,
) {
    unsafe {
        let mut battleMode = battleMode;
        let mut lvlMode = lvlMode;
        let mut x1 = x1;
        let mut x2 = x2;
        let mut y = y;
        let mut isCurrent: u8 = 0u8;
        let mut winStreak: u16 = TowerGetWinStreak(battleMode, lvlMode);
        'l1: {
            let __sw1 = ((battleMode) as i32);
            let __matched = __sw1 == 0i32 || __sw1 == 1i32 || __sw1 == 2i32 || __sw1 == 3i32;
            if __sw1 == 0i32 || !__matched {
                if ((lvlMode) as i32) != 0i32 {
                    isCurrent = IsWinStreakActive(2u32);
                } else {
                    isCurrent = IsWinStreakActive(1u32);
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                if ((lvlMode) as i32) != 0i32 {
                    isCurrent = IsWinStreakActive(32768u32);
                } else {
                    isCurrent = IsWinStreakActive(16384u32);
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                if ((lvlMode) as i32) != 0i32 {
                    isCurrent = IsWinStreakActive(131072u32);
                } else {
                    isCurrent = IsWinStreakActive(65536u32);
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                if ((lvlMode) as i32) != 0i32 {
                    isCurrent = IsWinStreakActive(524288u32);
                } else {
                    isCurrent = IsWinStreakActive(262144u32);
                }
                break 'l1;
            }
        }
        if ((isCurrent) as i32) == 1i32 {
            TowerPrintStreak((&raw mut gText_Current).cast::<u8>(), winStreak, x1, x2, y);
        } else {
            TowerPrintStreak((&raw mut gText_Prev).cast::<u8>(), winStreak, x1, x2, y);
        }
    }
}
pub(crate) unsafe extern "C" fn ShowTowerResultsWindow(battleMode: u8) {
    unsafe {
        let mut battleMode = battleMode;
        ((&raw mut gRecordsWindowId).cast::<u8>()).write(
            ((AddWindow(
                (&raw const sFrontierResultsWindowTemplate)
                    .cast::<u8>()
                    .cast_mut(),
            )) as u8),
        );
        DrawStdWindowFrame(((&raw mut gRecordsWindowId).cast::<u8>()).read(), 0u8);
        FillWindowPixelBuffer(((&raw mut gRecordsWindowId).cast::<u8>()).read(), 17u8);
        if ((battleMode) as i32) == 0i32 {
            StringExpandPlaceholders(
                (&raw mut gStringVar4).cast::<u8>(),
                (&raw mut gText_SingleBattleRoomResults).cast::<u8>(),
            );
        } else {
            if ((battleMode) as i32) == 1i32 {
                StringExpandPlaceholders(
                    (&raw mut gStringVar4).cast::<u8>(),
                    (&raw mut gText_DoubleBattleRoomResults).cast::<u8>(),
                );
            } else {
                if ((battleMode) as i32) == 2i32 {
                    StringExpandPlaceholders(
                        (&raw mut gStringVar4).cast::<u8>(),
                        (&raw mut gText_MultiBattleRoomResults).cast::<u8>(),
                    );
                } else {
                    StringExpandPlaceholders(
                        (&raw mut gStringVar4).cast::<u8>(),
                        (&raw mut gText_LinkMultiBattleRoomResults).cast::<u8>(),
                    );
                }
            }
        }
        PrintAligned((&raw mut gStringVar4).cast::<u8>(), 2i32);
        AddTextPrinterParameterized(
            ((&raw mut gRecordsWindowId).cast::<u8>()).read(),
            1u8,
            (&raw mut gText_Lv502).cast::<u8>(),
            16u8,
            49u8,
            255u8,
            None,
        );
        AddTextPrinterParameterized(
            ((&raw mut gRecordsWindowId).cast::<u8>()).read(),
            1u8,
            (&raw mut gText_OpenLv).cast::<u8>(),
            16u8,
            97u8,
            255u8,
            None,
        );
        PrintHyphens(10i32);
        TowerPrintPrevOrCurrentStreak(battleMode, 0u8, 72u8, 132u8, 49u8);
        TowerPrintRecordStreak(battleMode, 0u8, 72u8, 132u8, 65u8);
        TowerPrintPrevOrCurrentStreak(battleMode, 1u8, 72u8, 132u8, 97u8);
        TowerPrintRecordStreak(battleMode, 1u8, 72u8, 132u8, 113u8);
        PutWindowTilemap(((&raw mut gRecordsWindowId).cast::<u8>()).read());
        CopyWindowToVram(((&raw mut gRecordsWindowId).cast::<u8>()).read(), 3u8);
    }
}
pub(crate) unsafe extern "C" fn DomeGetWinStreak(battleMode: u8, lvlMode: u8) -> u16 {
    unsafe {
        let mut battleMode = battleMode;
        let mut lvlMode = lvlMode;
        let mut winStreak: u16 = (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
            .wrapping_add(1612))
        .wrapping_add(1728))
        .cast::<u8>())
        .wrapping_offset(((battleMode) as i32) as isize * 4))
        .cast::<u16>())
        .wrapping_offset(((lvlMode) as i32) as isize))
        .read();
        if ((winStreak) as i32) > 9999i32 {
            return 9999u16;
        } else {
            return winStreak;
        }
        #[allow(unreachable_code)]
        {
            return 0u16;
        }
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
    unsafe {
        let mut str1 = str1;
        let mut str2 = str2;
        let mut num = num;
        let mut x1 = x1;
        let mut x2 = x2;
        let mut y = y;
        AddTextPrinterParameterized(
            ((&raw mut gRecordsWindowId).cast::<u8>()).read(),
            1u8,
            str1,
            x1,
            y,
            255u8,
            None,
        );
        ConvertIntToDecimalStringN(
            (&raw mut gStringVar1).cast::<u8>(),
            ((num) as i32),
            1i32,
            4u8,
        );
        StringExpandPlaceholders((&raw mut gStringVar4).cast::<u8>(), str2);
        AddTextPrinterParameterized(
            ((&raw mut gRecordsWindowId).cast::<u8>()).read(),
            1u8,
            (&raw mut gStringVar4).cast::<u8>(),
            x2,
            y,
            255u8,
            None,
        );
    }
}
pub(crate) unsafe extern "C" fn DomePrintPrevOrCurrentStreak(
    battleMode: u8,
    lvlMode: u8,
    x1: u8,
    x2: u8,
    y: u8,
) {
    unsafe {
        let mut battleMode = battleMode;
        let mut lvlMode = lvlMode;
        let mut x1 = x1;
        let mut x2 = x2;
        let mut y = y;
        let mut isCurrent: u8 = 0u8;
        let mut winStreak: u16 = DomeGetWinStreak(battleMode, lvlMode);
        'l1: {
            let __sw1 = ((battleMode) as i32);
            let __matched = __sw1 == 0i32 || __sw1 == 1i32;
            if __sw1 == 0i32 || !__matched {
                if ((lvlMode) as i32) != 0i32 {
                    isCurrent = IsWinStreakActive(8u32);
                } else {
                    isCurrent = IsWinStreakActive(4u32);
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                if ((lvlMode) as i32) != 0i32 {
                    isCurrent = IsWinStreakActive(2097152u32);
                } else {
                    isCurrent = IsWinStreakActive(1048576u32);
                }
                break 'l1;
            }
        }
        if ((isCurrent) as i32) == 1i32 {
            PrintTwoStrings(
                (&raw mut gText_Current).cast::<u8>(),
                (&raw mut gText_ClearStreak).cast::<u8>(),
                winStreak,
                x1,
                x2,
                y,
            );
        } else {
            PrintTwoStrings(
                (&raw mut gText_Prev).cast::<u8>(),
                (&raw mut gText_ClearStreak).cast::<u8>(),
                winStreak,
                x1,
                x2,
                y,
            );
        }
    }
}
pub(crate) unsafe extern "C" fn ShowDomeResultsWindow(battleMode: u8) {
    unsafe {
        let mut battleMode = battleMode;
        ((&raw mut gRecordsWindowId).cast::<u8>()).write(
            ((AddWindow(
                (&raw const sFrontierResultsWindowTemplate)
                    .cast::<u8>()
                    .cast_mut(),
            )) as u8),
        );
        DrawStdWindowFrame(((&raw mut gRecordsWindowId).cast::<u8>()).read(), 0u8);
        FillWindowPixelBuffer(((&raw mut gRecordsWindowId).cast::<u8>()).read(), 17u8);
        if ((battleMode) as i32) == 0i32 {
            StringExpandPlaceholders(
                (&raw mut gStringVar4).cast::<u8>(),
                (&raw mut gText_SingleBattleTourneyResults).cast::<u8>(),
            );
        } else {
            StringExpandPlaceholders(
                (&raw mut gStringVar4).cast::<u8>(),
                (&raw mut gText_DoubleBattleTourneyResults).cast::<u8>(),
            );
        }
        PrintAligned((&raw mut gStringVar4).cast::<u8>(), 0i32);
        AddTextPrinterParameterized(
            ((&raw mut gRecordsWindowId).cast::<u8>()).read(),
            1u8,
            (&raw mut gText_Lv502).cast::<u8>(),
            8u8,
            33u8,
            255u8,
            None,
        );
        AddTextPrinterParameterized(
            ((&raw mut gRecordsWindowId).cast::<u8>()).read(),
            1u8,
            (&raw mut gText_OpenLv).cast::<u8>(),
            8u8,
            97u8,
            255u8,
            None,
        );
        PrintHyphens(10i32);
        DomePrintPrevOrCurrentStreak(battleMode, 0u8, 64u8, 121u8, 33u8);
        PrintTwoStrings(
            (&raw mut gText_Record).cast::<u8>(),
            (&raw mut gText_ClearStreak).cast::<u8>(),
            ((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(1736))
            .cast::<u8>())
            .wrapping_offset(((battleMode) as i32) as isize * 4))
            .cast::<u16>())
            .read(),
            64u8,
            121u8,
            49u8,
        );
        PrintTwoStrings(
            (&raw mut gText_Total).cast::<u8>(),
            (&raw mut gText_Championships).cast::<u8>(),
            ((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(1744))
            .cast::<u8>())
            .wrapping_offset(((battleMode) as i32) as isize * 4))
            .cast::<u16>())
            .read(),
            64u8,
            112u8,
            65u8,
        );
        DomePrintPrevOrCurrentStreak(battleMode, 1u8, 64u8, 121u8, 97u8);
        PrintTwoStrings(
            (&raw mut gText_Record).cast::<u8>(),
            (&raw mut gText_ClearStreak).cast::<u8>(),
            (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(1736))
            .cast::<u8>())
            .wrapping_offset(((battleMode) as i32) as isize * 4))
            .cast::<u16>())
            .wrapping_offset(1))
            .read(),
            64u8,
            121u8,
            113u8,
        );
        PrintTwoStrings(
            (&raw mut gText_Total).cast::<u8>(),
            (&raw mut gText_Championships).cast::<u8>(),
            (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(1744))
            .cast::<u8>())
            .wrapping_offset(((battleMode) as i32) as isize * 4))
            .cast::<u16>())
            .wrapping_offset(1))
            .read(),
            64u8,
            112u8,
            129u8,
        );
        PutWindowTilemap(((&raw mut gRecordsWindowId).cast::<u8>()).read());
        CopyWindowToVram(((&raw mut gRecordsWindowId).cast::<u8>()).read(), 3u8);
    }
}
pub(crate) unsafe extern "C" fn PalacePrintStreak(str: *mut u8, num: u16, x1: u8, x2: u8, y: u8) {
    unsafe {
        let mut str = str;
        let mut num = num;
        let mut x1 = x1;
        let mut x2 = x2;
        let mut y = y;
        AddTextPrinterParameterized(
            ((&raw mut gRecordsWindowId).cast::<u8>()).read(),
            1u8,
            str,
            x1,
            y,
            255u8,
            None,
        );
        if ((num) as i32) > 9999i32 {
            num = 9999u16;
        }
        ConvertIntToDecimalStringN(
            (&raw mut gStringVar1).cast::<u8>(),
            ((num) as i32),
            1i32,
            4u8,
        );
        StringExpandPlaceholders(
            (&raw mut gStringVar4).cast::<u8>(),
            (&raw mut gText_WinStreak).cast::<u8>(),
        );
        AddTextPrinterParameterized(
            ((&raw mut gRecordsWindowId).cast::<u8>()).read(),
            1u8,
            (&raw mut gStringVar4).cast::<u8>(),
            x2,
            y,
            255u8,
            None,
        );
    }
}
pub(crate) unsafe extern "C" fn PalacePrintRecordStreak(
    battleMode: u8,
    lvlMode: u8,
    x1: u8,
    x2: u8,
    y: u8,
) {
    unsafe {
        let mut battleMode = battleMode;
        let mut lvlMode = lvlMode;
        let mut x1 = x1;
        let mut x2 = x2;
        let mut y = y;
        let mut num: u16 = (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
            .wrapping_add(1612))
        .wrapping_add(1924))
        .cast::<u8>())
        .wrapping_offset(((battleMode) as i32) as isize * 4))
        .cast::<u16>())
        .wrapping_offset(((lvlMode) as i32) as isize))
        .read();
        PalacePrintStreak((&raw mut gText_Record).cast::<u8>(), num, x1, x2, y);
    }
}
pub(crate) unsafe extern "C" fn PalaceGetWinStreak(battleMode: u8, lvlMode: u8) -> u16 {
    unsafe {
        let mut battleMode = battleMode;
        let mut lvlMode = lvlMode;
        let mut winStreak: u16 = (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
            .wrapping_add(1612))
        .wrapping_add(1916))
        .cast::<u8>())
        .wrapping_offset(((battleMode) as i32) as isize * 4))
        .cast::<u16>())
        .wrapping_offset(((lvlMode) as i32) as isize))
        .read();
        if ((winStreak) as i32) > 9999i32 {
            return 9999u16;
        } else {
            return winStreak;
        }
        #[allow(unreachable_code)]
        {
            return 0u16;
        }
    }
}
pub(crate) unsafe extern "C" fn PalacePrintPrevOrCurrentStreak(
    battleMode: u8,
    lvlMode: u8,
    x1: u8,
    x2: u8,
    y: u8,
) {
    unsafe {
        let mut battleMode = battleMode;
        let mut lvlMode = lvlMode;
        let mut x1 = x1;
        let mut x2 = x2;
        let mut y = y;
        let mut isCurrent: u8 = 0u8;
        let mut winStreak: u16 = PalaceGetWinStreak(battleMode, lvlMode);
        'l1: {
            let __sw1 = ((battleMode) as i32);
            let __matched = __sw1 == 0i32 || __sw1 == 1i32;
            if __sw1 == 0i32 || !__matched {
                if ((lvlMode) as i32) != 0i32 {
                    isCurrent = IsWinStreakActive(32u32);
                } else {
                    isCurrent = IsWinStreakActive(16u32);
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                if ((lvlMode) as i32) != 0i32 {
                    isCurrent = IsWinStreakActive(8388608u32);
                } else {
                    isCurrent = IsWinStreakActive(4194304u32);
                }
            }
        }
        if ((isCurrent) as i32) == 1i32 {
            PalacePrintStreak((&raw mut gText_Current).cast::<u8>(), winStreak, x1, x2, y);
        } else {
            PalacePrintStreak((&raw mut gText_Prev).cast::<u8>(), winStreak, x1, x2, y);
        }
    }
}
pub(crate) unsafe extern "C" fn ShowPalaceResultsWindow(battleMode: u8) {
    unsafe {
        let mut battleMode = battleMode;
        ((&raw mut gRecordsWindowId).cast::<u8>()).write(
            ((AddWindow(
                (&raw const sFrontierResultsWindowTemplate)
                    .cast::<u8>()
                    .cast_mut(),
            )) as u8),
        );
        DrawStdWindowFrame(((&raw mut gRecordsWindowId).cast::<u8>()).read(), 0u8);
        FillWindowPixelBuffer(((&raw mut gRecordsWindowId).cast::<u8>()).read(), 17u8);
        if ((battleMode) as i32) == 0i32 {
            StringExpandPlaceholders(
                (&raw mut gStringVar4).cast::<u8>(),
                (&raw mut gText_SingleBattleHallResults).cast::<u8>(),
            );
        } else {
            StringExpandPlaceholders(
                (&raw mut gStringVar4).cast::<u8>(),
                (&raw mut gText_DoubleBattleHallResults).cast::<u8>(),
            );
        }
        PrintAligned((&raw mut gStringVar4).cast::<u8>(), 2i32);
        AddTextPrinterParameterized(
            ((&raw mut gRecordsWindowId).cast::<u8>()).read(),
            1u8,
            (&raw mut gText_Lv502).cast::<u8>(),
            16u8,
            49u8,
            255u8,
            None,
        );
        AddTextPrinterParameterized(
            ((&raw mut gRecordsWindowId).cast::<u8>()).read(),
            1u8,
            (&raw mut gText_OpenLv).cast::<u8>(),
            16u8,
            97u8,
            255u8,
            None,
        );
        PrintHyphens(10i32);
        PalacePrintPrevOrCurrentStreak(battleMode, 0u8, 72u8, 131u8, 49u8);
        PalacePrintRecordStreak(battleMode, 0u8, 72u8, 131u8, 65u8);
        PalacePrintPrevOrCurrentStreak(battleMode, 1u8, 72u8, 131u8, 97u8);
        PalacePrintRecordStreak(battleMode, 1u8, 72u8, 131u8, 113u8);
        PutWindowTilemap(((&raw mut gRecordsWindowId).cast::<u8>()).read());
        CopyWindowToVram(((&raw mut gRecordsWindowId).cast::<u8>()).read(), 3u8);
    }
}
pub(crate) unsafe extern "C" fn PikeGetWinStreak(lvlMode: u8) -> u16 {
    unsafe {
        let mut lvlMode = lvlMode;
        let mut winStreak: u16 = (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
            .wrapping_add(1612))
        .wrapping_add(1976))
        .cast::<u16>())
        .wrapping_offset(((lvlMode) as i32) as isize))
        .read();
        if ((winStreak) as i32) > 9999i32 {
            return 9999u16;
        } else {
            return winStreak;
        }
        #[allow(unreachable_code)]
        {
            return 0u16;
        }
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
    unsafe {
        let mut str1 = str1;
        let mut str2 = str2;
        let mut num = num;
        let mut x1 = x1;
        let mut x2 = x2;
        let mut y = y;
        AddTextPrinterParameterized(
            ((&raw mut gRecordsWindowId).cast::<u8>()).read(),
            1u8,
            str1,
            x1,
            y,
            255u8,
            None,
        );
        ConvertIntToDecimalStringN(
            (&raw mut gStringVar1).cast::<u8>(),
            ((num) as i32),
            1i32,
            4u8,
        );
        StringExpandPlaceholders((&raw mut gStringVar4).cast::<u8>(), str2);
        AddTextPrinterParameterized(
            ((&raw mut gRecordsWindowId).cast::<u8>()).read(),
            1u8,
            (&raw mut gStringVar4).cast::<u8>(),
            x2,
            y,
            255u8,
            None,
        );
    }
}
pub(crate) unsafe extern "C" fn PikePrintPrevOrCurrentStreak(lvlMode: u8, x1: u8, x2: u8, y: u8) {
    unsafe {
        let mut lvlMode = lvlMode;
        let mut x1 = x1;
        let mut x2 = x2;
        let mut y = y;
        let mut isCurrent: u8 = 0u8;
        let mut winStreak: u16 = PikeGetWinStreak(lvlMode);
        if ((lvlMode) as i32) != 0i32 {
            isCurrent = IsWinStreakActive(2048u32);
        } else {
            isCurrent = IsWinStreakActive(1024u32);
        }
        if ((isCurrent) as i32) == 1i32 {
            PrintTwoStrings(
                (&raw mut gText_Current).cast::<u8>(),
                (&raw mut gText_RoomsCleared).cast::<u8>(),
                winStreak,
                x1,
                x2,
                y,
            );
        } else {
            PrintTwoStrings(
                (&raw mut gText_Prev).cast::<u8>(),
                (&raw mut gText_RoomsCleared).cast::<u8>(),
                winStreak,
                x1,
                x2,
                y,
            );
        }
    }
}
pub(crate) unsafe extern "C" fn ShowPikeResultsWindow() {
    unsafe {
        ((&raw mut gRecordsWindowId).cast::<u8>()).write(
            ((AddWindow(
                (&raw const sFrontierResultsWindowTemplate)
                    .cast::<u8>()
                    .cast_mut(),
            )) as u8),
        );
        DrawStdWindowFrame(((&raw mut gRecordsWindowId).cast::<u8>()).read(), 0u8);
        FillWindowPixelBuffer(((&raw mut gRecordsWindowId).cast::<u8>()).read(), 17u8);
        StringExpandPlaceholders(
            (&raw mut gStringVar4).cast::<u8>(),
            (&raw mut gText_BattleChoiceResults).cast::<u8>(),
        );
        PrintAligned((&raw mut gStringVar4).cast::<u8>(), 0i32);
        AddTextPrinterParameterized(
            ((&raw mut gRecordsWindowId).cast::<u8>()).read(),
            1u8,
            (&raw mut gText_Lv502).cast::<u8>(),
            8u8,
            33u8,
            255u8,
            None,
        );
        AddTextPrinterParameterized(
            ((&raw mut gRecordsWindowId).cast::<u8>()).read(),
            1u8,
            (&raw mut gText_OpenLv).cast::<u8>(),
            8u8,
            97u8,
            255u8,
            None,
        );
        PrintHyphens(10i32);
        PikePrintPrevOrCurrentStreak(0u8, 64u8, 114u8, 33u8);
        PikePrintCleared(
            (&raw mut gText_Record).cast::<u8>(),
            (&raw mut gText_RoomsCleared).cast::<u8>(),
            ((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(1980))
            .cast::<u16>())
            .read(),
            64u8,
            114u8,
            49u8,
        );
        PikePrintCleared(
            (&raw mut gText_Total).cast::<u8>(),
            (&raw mut gText_TimesCleared).cast::<u8>(),
            ((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(1984))
            .cast::<u16>())
            .read(),
            64u8,
            114u8,
            65u8,
        );
        PikePrintPrevOrCurrentStreak(1u8, 64u8, 114u8, 97u8);
        PikePrintCleared(
            (&raw mut gText_Record).cast::<u8>(),
            (&raw mut gText_RoomsCleared).cast::<u8>(),
            (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(1980))
            .cast::<u16>())
            .wrapping_offset(1))
            .read(),
            64u8,
            114u8,
            113u8,
        );
        PikePrintCleared(
            (&raw mut gText_Total).cast::<u8>(),
            (&raw mut gText_TimesCleared).cast::<u8>(),
            (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(1984))
            .cast::<u16>())
            .wrapping_offset(1))
            .read(),
            64u8,
            114u8,
            129u8,
        );
        PutWindowTilemap(((&raw mut gRecordsWindowId).cast::<u8>()).read());
        CopyWindowToVram(((&raw mut gRecordsWindowId).cast::<u8>()).read(), 3u8);
    }
}
pub(crate) unsafe extern "C" fn ArenaPrintStreak(str: *mut u8, num: u16, x1: u8, x2: u8, y: u8) {
    unsafe {
        let mut str = str;
        let mut num = num;
        let mut x1 = x1;
        let mut x2 = x2;
        let mut y = y;
        AddTextPrinterParameterized(
            ((&raw mut gRecordsWindowId).cast::<u8>()).read(),
            1u8,
            str,
            x1,
            y,
            255u8,
            None,
        );
        if ((num) as i32) > 9999i32 {
            num = 9999u16;
        }
        ConvertIntToDecimalStringN(
            (&raw mut gStringVar1).cast::<u8>(),
            ((num) as i32),
            1i32,
            4u8,
        );
        StringExpandPlaceholders(
            (&raw mut gStringVar4).cast::<u8>(),
            (&raw mut gText_KOsInARow).cast::<u8>(),
        );
        AddTextPrinterParameterized(
            ((&raw mut gRecordsWindowId).cast::<u8>()).read(),
            1u8,
            (&raw mut gStringVar4).cast::<u8>(),
            x2,
            y,
            255u8,
            None,
        );
    }
}
pub(crate) unsafe extern "C" fn ArenaPrintRecordStreak(lvlMode: u8, x1: u8, x2: u8, y: u8) {
    unsafe {
        let mut lvlMode = lvlMode;
        let mut x1 = x1;
        let mut x2 = x2;
        let mut y = y;
        let mut num: u16 = (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
            .wrapping_add(1612))
        .wrapping_add(1938))
        .cast::<u16>())
        .wrapping_offset(((lvlMode) as i32) as isize))
        .read();
        ArenaPrintStreak((&raw mut gText_Record).cast::<u8>(), num, x1, x2, y);
    }
}
pub(crate) unsafe extern "C" fn ArenaGetWinStreak(lvlMode: u8) -> u16 {
    unsafe {
        let mut lvlMode = lvlMode;
        let mut winStreak: u16 = (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
            .wrapping_add(1612))
        .wrapping_add(1934))
        .cast::<u16>())
        .wrapping_offset(((lvlMode) as i32) as isize))
        .read();
        if ((winStreak) as i32) > 9999i32 {
            return 9999u16;
        } else {
            return winStreak;
        }
        #[allow(unreachable_code)]
        {
            return 0u16;
        }
    }
}
pub(crate) unsafe extern "C" fn ArenaPrintPrevOrCurrentStreak(lvlMode: u8, x1: u8, x2: u8, y: u8) {
    unsafe {
        let mut lvlMode = lvlMode;
        let mut x1 = x1;
        let mut x2 = x2;
        let mut y = y;
        let mut isCurrent: u8 = 0u8;
        let mut winStreak: u16 = ArenaGetWinStreak(lvlMode);
        if ((lvlMode) as i32) != 0i32 {
            isCurrent = IsWinStreakActive(128u32);
        } else {
            isCurrent = IsWinStreakActive(64u32);
        }
        if ((isCurrent) as i32) == 1i32 {
            ArenaPrintStreak((&raw mut gText_Current).cast::<u8>(), winStreak, x1, x2, y);
        } else {
            ArenaPrintStreak((&raw mut gText_Prev).cast::<u8>(), winStreak, x1, x2, y);
        }
    }
}
pub(crate) unsafe extern "C" fn ShowArenaResultsWindow() {
    unsafe {
        ((&raw mut gRecordsWindowId).cast::<u8>()).write(
            ((AddWindow(
                (&raw const sFrontierResultsWindowTemplate)
                    .cast::<u8>()
                    .cast_mut(),
            )) as u8),
        );
        DrawStdWindowFrame(((&raw mut gRecordsWindowId).cast::<u8>()).read(), 0u8);
        FillWindowPixelBuffer(((&raw mut gRecordsWindowId).cast::<u8>()).read(), 17u8);
        PrintHyphens(10i32);
        StringExpandPlaceholders(
            (&raw mut gStringVar4).cast::<u8>(),
            (&raw mut gText_SetKOTourneyResults).cast::<u8>(),
        );
        PrintAligned((&raw mut gStringVar4).cast::<u8>(), 2i32);
        AddTextPrinterParameterized(
            ((&raw mut gRecordsWindowId).cast::<u8>()).read(),
            1u8,
            (&raw mut gText_Lv502).cast::<u8>(),
            16u8,
            49u8,
            255u8,
            None,
        );
        AddTextPrinterParameterized(
            ((&raw mut gRecordsWindowId).cast::<u8>()).read(),
            1u8,
            (&raw mut gText_OpenLv).cast::<u8>(),
            16u8,
            97u8,
            255u8,
            None,
        );
        ArenaPrintPrevOrCurrentStreak(0u8, 72u8, 126u8, 49u8);
        ArenaPrintRecordStreak(0u8, 72u8, 126u8, 65u8);
        ArenaPrintPrevOrCurrentStreak(1u8, 72u8, 126u8, 97u8);
        ArenaPrintRecordStreak(1u8, 72u8, 126u8, 113u8);
        PutWindowTilemap(((&raw mut gRecordsWindowId).cast::<u8>()).read());
        CopyWindowToVram(((&raw mut gRecordsWindowId).cast::<u8>()).read(), 3u8);
    }
}
pub(crate) unsafe extern "C" fn FactoryPrintStreak(
    str: *mut u8,
    num1: u16,
    num2: u16,
    x1: u8,
    x2: u8,
    x3: u8,
    y: u8,
) {
    unsafe {
        let mut str = str;
        let mut num1 = num1;
        let mut num2 = num2;
        let mut x1 = x1;
        let mut x2 = x2;
        let mut x3 = x3;
        let mut y = y;
        AddTextPrinterParameterized(
            ((&raw mut gRecordsWindowId).cast::<u8>()).read(),
            1u8,
            str,
            x1,
            y,
            255u8,
            None,
        );
        if ((num1) as i32) > 9999i32 {
            num1 = 9999u16;
        }
        ConvertIntToDecimalStringN(
            (&raw mut gStringVar1).cast::<u8>(),
            ((num1) as i32),
            1i32,
            4u8,
        );
        StringExpandPlaceholders(
            (&raw mut gStringVar4).cast::<u8>(),
            (&raw mut gText_WinStreak).cast::<u8>(),
        );
        AddTextPrinterParameterized(
            ((&raw mut gRecordsWindowId).cast::<u8>()).read(),
            1u8,
            (&raw mut gStringVar4).cast::<u8>(),
            x2,
            y,
            255u8,
            None,
        );
        ConvertIntToDecimalStringN(
            (&raw mut gStringVar1).cast::<u8>(),
            ((num2) as i32),
            1i32,
            4u8,
        );
        StringExpandPlaceholders(
            (&raw mut gStringVar4).cast::<u8>(),
            (&raw mut gText_TimesVar1).cast::<u8>(),
        );
        AddTextPrinterParameterized(
            ((&raw mut gRecordsWindowId).cast::<u8>()).read(),
            1u8,
            (&raw mut gStringVar4).cast::<u8>(),
            x3,
            y,
            255u8,
            None,
        );
    }
}
pub(crate) unsafe extern "C" fn FactoryPrintRecordStreak(
    battleMode: u8,
    lvlMode: u8,
    x1: u8,
    x2: u8,
    x3: u8,
    y: u8,
) {
    unsafe {
        let mut battleMode = battleMode;
        let mut lvlMode = lvlMode;
        let mut x1 = x1;
        let mut x2 = x2;
        let mut x3 = x3;
        let mut y = y;
        let mut num1: u16 = (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
            .wrapping_add(1612))
        .wrapping_add(1950))
        .cast::<u8>())
        .wrapping_offset(((battleMode) as i32) as isize * 4))
        .cast::<u16>())
        .wrapping_offset(((lvlMode) as i32) as isize))
        .read();
        let mut num2: u16 = (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
            .wrapping_add(1612))
        .wrapping_add(1966))
        .cast::<u8>())
        .wrapping_offset(((battleMode) as i32) as isize * 4))
        .cast::<u16>())
        .wrapping_offset(((lvlMode) as i32) as isize))
        .read();
        FactoryPrintStreak(
            (&raw mut gText_Record).cast::<u8>(),
            num1,
            num2,
            x1,
            x2,
            x3,
            y,
        );
    }
}
pub(crate) unsafe extern "C" fn FactoryGetWinStreak(battleMode: u8, lvlMode: u8) -> u16 {
    unsafe {
        let mut battleMode = battleMode;
        let mut lvlMode = lvlMode;
        let mut winStreak: u16 = (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
            .wrapping_add(1612))
        .wrapping_add(1942))
        .cast::<u8>())
        .wrapping_offset(((battleMode) as i32) as isize * 4))
        .cast::<u16>())
        .wrapping_offset(((lvlMode) as i32) as isize))
        .read();
        if ((winStreak) as i32) > 9999i32 {
            return 9999u16;
        } else {
            return winStreak;
        }
        #[allow(unreachable_code)]
        {
            return 0u16;
        }
    }
}
pub(crate) unsafe extern "C" fn FactoryGetRentsCount(battleMode: u8, lvlMode: u8) -> u16 {
    unsafe {
        let mut battleMode = battleMode;
        let mut lvlMode = lvlMode;
        let mut rents: u16 = (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
            .wrapping_add(1612))
        .wrapping_add(1958))
        .cast::<u8>())
        .wrapping_offset(((battleMode) as i32) as isize * 4))
        .cast::<u16>())
        .wrapping_offset(((lvlMode) as i32) as isize))
        .read();
        if ((rents) as i32) > 9999i32 {
            return 9999u16;
        } else {
            return rents;
        }
        #[allow(unreachable_code)]
        {
            return 0u16;
        }
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
    unsafe {
        let mut battleMode = battleMode;
        let mut lvlMode = lvlMode;
        let mut x1 = x1;
        let mut x2 = x2;
        let mut x3 = x3;
        let mut y = y;
        let mut isCurrent: u8 = 0u8;
        let mut winStreak: u16 = FactoryGetWinStreak(battleMode, lvlMode);
        let mut rents: u16 = FactoryGetRentsCount(battleMode, lvlMode);
        'l1: {
            let __sw1 = ((battleMode) as i32);
            let __matched = __sw1 == 0i32 || __sw1 == 1i32;
            if __sw1 == 0i32 || !__matched {
                if ((lvlMode) as i32) != 0i32 {
                    isCurrent = IsWinStreakActive(512u32);
                } else {
                    isCurrent = IsWinStreakActive(256u32);
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                if ((lvlMode) as i32) != 0i32 {
                    isCurrent = IsWinStreakActive(33554432u32);
                } else {
                    isCurrent = IsWinStreakActive(16777216u32);
                }
                break 'l1;
            }
        }
        if ((isCurrent) as i32) == 1i32 {
            FactoryPrintStreak(
                (&raw mut gText_Current).cast::<u8>(),
                winStreak,
                rents,
                x1,
                x2,
                x3,
                y,
            );
        } else {
            FactoryPrintStreak(
                (&raw mut gText_Prev).cast::<u8>(),
                winStreak,
                rents,
                x1,
                x2,
                x3,
                y,
            );
        }
    }
}
pub(crate) unsafe extern "C" fn ShowFactoryResultsWindow(battleMode: u8) {
    unsafe {
        let mut battleMode = battleMode;
        ((&raw mut gRecordsWindowId).cast::<u8>()).write(
            ((AddWindow(
                (&raw const sFrontierResultsWindowTemplate)
                    .cast::<u8>()
                    .cast_mut(),
            )) as u8),
        );
        DrawStdWindowFrame(((&raw mut gRecordsWindowId).cast::<u8>()).read(), 0u8);
        FillWindowPixelBuffer(((&raw mut gRecordsWindowId).cast::<u8>()).read(), 17u8);
        if ((battleMode) as i32) == 0i32 {
            StringExpandPlaceholders(
                (&raw mut gStringVar4).cast::<u8>(),
                (&raw mut gText_BattleSwapSingleResults).cast::<u8>(),
            );
        } else {
            StringExpandPlaceholders(
                (&raw mut gStringVar4).cast::<u8>(),
                (&raw mut gText_BattleSwapDoubleResults).cast::<u8>(),
            );
        }
        PrintAligned((&raw mut gStringVar4).cast::<u8>(), 0i32);
        AddTextPrinterParameterized(
            ((&raw mut gRecordsWindowId).cast::<u8>()).read(),
            1u8,
            (&raw mut gText_Lv502).cast::<u8>(),
            8u8,
            33u8,
            255u8,
            None,
        );
        AddTextPrinterParameterized(
            ((&raw mut gRecordsWindowId).cast::<u8>()).read(),
            1u8,
            (&raw mut gText_RentalSwap).cast::<u8>(),
            152u8,
            33u8,
            255u8,
            None,
        );
        AddTextPrinterParameterized(
            ((&raw mut gRecordsWindowId).cast::<u8>()).read(),
            1u8,
            (&raw mut gText_OpenLv).cast::<u8>(),
            8u8,
            97u8,
            255u8,
            None,
        );
        PrintHyphens(10i32);
        FactoryPrintPrevOrCurrentStreak(battleMode, 0u8, 8u8, 64u8, 158u8, 49u8);
        FactoryPrintRecordStreak(battleMode, 0u8, 8u8, 64u8, 158u8, 65u8);
        FactoryPrintPrevOrCurrentStreak(battleMode, 1u8, 8u8, 64u8, 158u8, 113u8);
        FactoryPrintRecordStreak(battleMode, 1u8, 8u8, 64u8, 158u8, 129u8);
        PutWindowTilemap(((&raw mut gRecordsWindowId).cast::<u8>()).read());
        CopyWindowToVram(((&raw mut gRecordsWindowId).cast::<u8>()).read(), 3u8);
    }
}
pub(crate) unsafe extern "C" fn PyramidPrintStreak(str: *mut u8, num: u16, x1: u8, x2: u8, y: u8) {
    unsafe {
        let mut str = str;
        let mut num = num;
        let mut x1 = x1;
        let mut x2 = x2;
        let mut y = y;
        AddTextPrinterParameterized(
            ((&raw mut gRecordsWindowId).cast::<u8>()).read(),
            1u8,
            str,
            x1,
            y,
            255u8,
            None,
        );
        if ((num) as i32) > 9999i32 {
            num = 9999u16;
        }
        ConvertIntToDecimalStringN(
            (&raw mut gStringVar1).cast::<u8>(),
            ((num) as i32),
            1i32,
            4u8,
        );
        StringExpandPlaceholders(
            (&raw mut gStringVar4).cast::<u8>(),
            (&raw mut gText_FloorsCleared).cast::<u8>(),
        );
        AddTextPrinterParameterized(
            ((&raw mut gRecordsWindowId).cast::<u8>()).read(),
            1u8,
            (&raw mut gStringVar4).cast::<u8>(),
            x2,
            y,
            255u8,
            None,
        );
    }
}
pub(crate) unsafe extern "C" fn PyramidPrintRecordStreak(lvlMode: u8, x1: u8, x2: u8, y: u8) {
    unsafe {
        let mut lvlMode = lvlMode;
        let mut x1 = x1;
        let mut x2 = x2;
        let mut y = y;
        let mut num: u16 = (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
            .wrapping_add(1612))
        .wrapping_add(2002))
        .cast::<u16>())
        .wrapping_offset(((lvlMode) as i32) as isize))
        .read();
        PyramidPrintStreak((&raw mut gText_Record).cast::<u8>(), num, x1, x2, y);
    }
}
pub(crate) unsafe extern "C" fn PyramidGetWinStreak(lvlMode: u8) -> u16 {
    unsafe {
        let mut lvlMode = lvlMode;
        let mut winStreak: u16 = (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
            .wrapping_add(1612))
        .wrapping_add(1998))
        .cast::<u16>())
        .wrapping_offset(((lvlMode) as i32) as isize))
        .read();
        if ((winStreak) as i32) > 9999i32 {
            return 9999u16;
        } else {
            return winStreak;
        }
        #[allow(unreachable_code)]
        {
            return 0u16;
        }
    }
}
pub(crate) unsafe extern "C" fn PyramidPrintPrevOrCurrentStreak(
    lvlMode: u8,
    x1: u8,
    x2: u8,
    y: u8,
) {
    unsafe {
        let mut lvlMode = lvlMode;
        let mut x1 = x1;
        let mut x2 = x2;
        let mut y = y;
        let mut isCurrent: u8 = 0u8;
        let mut winStreak: u16 = PyramidGetWinStreak(lvlMode);
        if ((lvlMode) as i32) != 0i32 {
            isCurrent = IsWinStreakActive(8192u32);
        } else {
            isCurrent = IsWinStreakActive(4096u32);
        }
        if ((isCurrent) as i32) == 1i32 {
            PyramidPrintStreak((&raw mut gText_Current).cast::<u8>(), winStreak, x1, x2, y);
        } else {
            PyramidPrintStreak((&raw mut gText_Prev).cast::<u8>(), winStreak, x1, x2, y);
        }
    }
}
pub(crate) unsafe extern "C" fn ShowPyramidResultsWindow() {
    unsafe {
        ((&raw mut gRecordsWindowId).cast::<u8>()).write(
            ((AddWindow(
                (&raw const sFrontierResultsWindowTemplate)
                    .cast::<u8>()
                    .cast_mut(),
            )) as u8),
        );
        DrawStdWindowFrame(((&raw mut gRecordsWindowId).cast::<u8>()).read(), 0u8);
        FillWindowPixelBuffer(((&raw mut gRecordsWindowId).cast::<u8>()).read(), 17u8);
        StringExpandPlaceholders(
            (&raw mut gStringVar4).cast::<u8>(),
            (&raw mut gText_BattleQuestResults).cast::<u8>(),
        );
        PrintAligned((&raw mut gStringVar4).cast::<u8>(), 2i32);
        AddTextPrinterParameterized(
            ((&raw mut gRecordsWindowId).cast::<u8>()).read(),
            1u8,
            (&raw mut gText_Lv502).cast::<u8>(),
            8u8,
            49u8,
            255u8,
            None,
        );
        AddTextPrinterParameterized(
            ((&raw mut gRecordsWindowId).cast::<u8>()).read(),
            1u8,
            (&raw mut gText_OpenLv).cast::<u8>(),
            8u8,
            97u8,
            255u8,
            None,
        );
        PrintHyphens(10i32);
        PyramidPrintPrevOrCurrentStreak(0u8, 64u8, 111u8, 49u8);
        PyramidPrintRecordStreak(0u8, 64u8, 111u8, 65u8);
        PyramidPrintPrevOrCurrentStreak(1u8, 64u8, 111u8, 97u8);
        PyramidPrintRecordStreak(1u8, 64u8, 111u8, 113u8);
        PutWindowTilemap(((&raw mut gRecordsWindowId).cast::<u8>()).read());
        CopyWindowToVram(((&raw mut gRecordsWindowId).cast::<u8>()).read(), 3u8);
    }
}
pub(crate) unsafe extern "C" fn ShowLinkContestResultsWindow() {
    unsafe {
        let mut str: *mut u8 = core::ptr::null_mut();
        let mut i: i32 = 0i32;
        let mut j: i32 = 0i32;
        let mut x: i32 = 0i32;
        ((&raw mut gRecordsWindowId).cast::<u8>()).write(
            ((AddWindow(
                (&raw const sLinkContestResultsWindowTemplate)
                    .cast::<u8>()
                    .cast_mut(),
            )) as u8),
        );
        DrawStdWindowFrame(((&raw mut gRecordsWindowId).cast::<u8>()).read(), 0u8);
        FillWindowPixelBuffer(((&raw mut gRecordsWindowId).cast::<u8>()).read(), 17u8);
        StringExpandPlaceholders(
            (&raw mut gStringVar4).cast::<u8>(),
            (&raw mut gText_LinkContestResults).cast::<u8>(),
        );
        x = GetStringCenterAlignXOffset(1i32, (&raw mut gStringVar4).cast::<u8>(), 208i32);
        AddTextPrinterParameterized(
            ((&raw mut gRecordsWindowId).cast::<u8>()).read(),
            1u8,
            (&raw mut gStringVar4).cast::<u8>(),
            ((x) as u8),
            1u8,
            255u8,
            None,
        );
        str = (&raw mut gText_1st).cast::<u8>();
        x = (GetStringRightAlignXOffset(1i32, str, 38i32)).wrapping_add(50i32);
        AddTextPrinterParameterized(
            ((&raw mut gRecordsWindowId).cast::<u8>()).read(),
            1u8,
            str,
            ((x) as u8),
            25u8,
            255u8,
            None,
        );
        str = (&raw mut gText_2nd).cast::<u8>();
        x = (GetStringRightAlignXOffset(1i32, str, 38i32)).wrapping_add(88i32);
        AddTextPrinterParameterized(
            ((&raw mut gRecordsWindowId).cast::<u8>()).read(),
            1u8,
            str,
            ((x) as u8),
            25u8,
            255u8,
            None,
        );
        str = (&raw mut gText_3rd).cast::<u8>();
        x = (GetStringRightAlignXOffset(1i32, str, 38i32)).wrapping_add(126i32);
        AddTextPrinterParameterized(
            ((&raw mut gRecordsWindowId).cast::<u8>()).read(),
            1u8,
            str,
            ((x) as u8),
            25u8,
            255u8,
            None,
        );
        str = (&raw mut gText_4th).cast::<u8>();
        x = (GetStringRightAlignXOffset(1i32, str, 38i32)).wrapping_add(164i32);
        AddTextPrinterParameterized(
            ((&raw mut gRecordsWindowId).cast::<u8>()).read(),
            1u8,
            str,
            ((x) as u8),
            25u8,
            255u8,
            None,
        );
        x = 6i32;
        AddTextPrinterParameterized(
            ((&raw mut gRecordsWindowId).cast::<u8>()).read(),
            1u8,
            (&raw mut gText_Cool).cast::<u8>(),
            ((x) as u8),
            41u8,
            255u8,
            None,
        );
        AddTextPrinterParameterized(
            ((&raw mut gRecordsWindowId).cast::<u8>()).read(),
            1u8,
            (&raw mut gText_Beauty).cast::<u8>(),
            ((x) as u8),
            57u8,
            255u8,
            None,
        );
        AddTextPrinterParameterized(
            ((&raw mut gRecordsWindowId).cast::<u8>()).read(),
            1u8,
            (&raw mut gText_Cute).cast::<u8>(),
            ((x) as u8),
            73u8,
            255u8,
            None,
        );
        AddTextPrinterParameterized(
            ((&raw mut gRecordsWindowId).cast::<u8>()).read(),
            1u8,
            (&raw mut gText_Smart).cast::<u8>(),
            ((x) as u8),
            89u8,
            255u8,
            None,
        );
        AddTextPrinterParameterized(
            ((&raw mut gRecordsWindowId).cast::<u8>()).read(),
            1u8,
            (&raw mut gText_Tough).cast::<u8>(),
            ((x) as u8),
            105u8,
            255u8,
            None,
        );
        {
            i = 0i32;
            'l1: loop {
                if !(i < 5i32) {
                    break 'l1;
                }
                'l2: {
                    {
                        j = 0i32;
                        'l3: loop {
                            if !(j < 4i32) {
                                break 'l3;
                            }
                            'l4: {
                                ConvertIntToDecimalStringN(
                                    (&raw mut gStringVar4).cast::<u8>(),
                                    ((((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(1572))
                                    .cast::<u8>())
                                    .wrapping_offset((i) as isize * 8))
                                    .cast::<u16>())
                                    .wrapping_offset((j) as isize))
                                    .read()) as i32),
                                    1i32,
                                    4u8,
                                );
                                AddTextPrinterParameterized(
                                    ((&raw mut gRecordsWindowId).cast::<u8>()).read(),
                                    1u8,
                                    (&raw mut gStringVar4).cast::<u8>(),
                                    ((((j).wrapping_mul(38i32)).wrapping_add(64i32)) as u8),
                                    ((((i).wrapping_mul(16i32)).wrapping_add(41i32)) as u8),
                                    255u8,
                                    None,
                                );
                            }
                            j = (j).wrapping_add(1);
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        PutWindowTilemap(((&raw mut gRecordsWindowId).cast::<u8>()).read());
        CopyWindowToVram(((&raw mut gRecordsWindowId).cast::<u8>()).read(), 3u8);
    }
}
pub(crate) unsafe extern "C" fn CheckPutFrontierTVShowOnAir() {
    unsafe {
        let mut name = crate::ffi::Align4([0u8; 32]);
        let mut lvlMode: i32 = ((crate::c::bf_read(
            ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(1629),
            0,
            2,
            false,
        ) as u8) as i32);
        let mut facility: i32 = ((VarGet(16591u16)) as i32);
        let mut battleMode: i32 = ((VarGet(16590u16)) as i32);
        'l1: {
            let __sw1 = facility;
            if __sw1 == 0i32 {
                if (((((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                    .wrapping_add(1612))
                .wrapping_add(1684))
                .cast::<u8>())
                .wrapping_offset((battleMode) as isize * 4))
                .cast::<u16>())
                .wrapping_offset((lvlMode) as isize))
                .read()) as i32)
                    > (((((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(1612))
                    .wrapping_add(1700))
                    .cast::<u8>())
                    .wrapping_offset((battleMode) as isize * 4))
                    .cast::<u16>())
                    .wrapping_offset((lvlMode) as isize))
                    .read()) as i32)
                {
                    (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(1612))
                    .wrapping_add(1700))
                    .cast::<u8>())
                    .wrapping_offset((battleMode) as isize * 4))
                    .cast::<u16>())
                    .wrapping_offset((lvlMode) as isize))
                    .write(
                        (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(1612))
                        .wrapping_add(1684))
                        .cast::<u8>())
                        .wrapping_offset((battleMode) as isize * 4))
                        .cast::<u16>())
                        .wrapping_offset((lvlMode) as isize))
                        .read(),
                    );
                    if battleMode == 3i32 {
                        StringCopy(
                            (&raw mut name).cast::<u8>(),
                            ((((&raw mut gLinkPlayers).cast::<u8>()).wrapping_offset(
                                ((((((&raw mut gBattleScripting).cast::<u8>()).wrapping_add(37))
                                    .read()) as i32)
                                    ^ 1i32) as isize
                                    * 28,
                            ))
                            .wrapping_add(8))
                            .cast::<u8>(),
                        );
                        StripExtCtrlCodes((&raw mut name).cast::<u8>());
                        StringCopy(
                            (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(1612))
                            .wrapping_add(2197))
                            .cast::<u8>())
                            .wrapping_offset((lvlMode) as isize * 8))
                            .cast::<u8>(),
                            (&raw mut name).cast::<u8>(),
                        );
                        SetTrainerId(
                            ((((&raw mut gLinkPlayers).cast::<u8>()).wrapping_offset(
                                ((((((&raw mut gBattleScripting).cast::<u8>()).wrapping_add(37))
                                    .read()) as i32)
                                    ^ 1i32) as isize
                                    * 28,
                            ))
                            .wrapping_add(4)
                            .cast::<u32>())
                            .read(),
                            (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(1612))
                            .wrapping_add(2213))
                            .cast::<u8>())
                            .wrapping_offset((lvlMode) as isize * 4))
                            .cast::<u8>(),
                        );
                    }
                    if ((((((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(1612))
                    .wrapping_add(1684))
                    .cast::<u8>())
                    .wrapping_offset((battleMode) as isize * 4))
                    .cast::<u16>())
                    .wrapping_offset((lvlMode) as isize))
                    .read()) as i32)
                        > 1i32)
                        && ((ShouldAirFrontierTVShow()) != 0)
                    {
                        'l2: {
                            let __sw2 = battleMode;
                            if __sw2 == 0i32 {
                                TryPutFrontierTVShowOnAir(
                                    (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(1612))
                                    .wrapping_add(1684))
                                    .cast::<u8>())
                                    .wrapping_offset((battleMode) as isize * 4))
                                    .cast::<u16>())
                                    .wrapping_offset((lvlMode) as isize))
                                    .read(),
                                    1u8,
                                );
                                break 'l2;
                            }
                            if __sw2 == 1i32 {
                                TryPutFrontierTVShowOnAir(
                                    (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(1612))
                                    .wrapping_add(1684))
                                    .cast::<u8>())
                                    .wrapping_offset((battleMode) as isize * 4))
                                    .cast::<u16>())
                                    .wrapping_offset((lvlMode) as isize))
                                    .read(),
                                    2u8,
                                );
                                break 'l2;
                            }
                            if __sw2 == 2i32 {
                                TryPutFrontierTVShowOnAir(
                                    (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(1612))
                                    .wrapping_add(1684))
                                    .cast::<u8>())
                                    .wrapping_offset((battleMode) as isize * 4))
                                    .cast::<u16>())
                                    .wrapping_offset((lvlMode) as isize))
                                    .read(),
                                    3u8,
                                );
                                break 'l2;
                            }
                            if __sw2 == 3i32 {
                                TryPutFrontierTVShowOnAir(
                                    (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(1612))
                                    .wrapping_add(1684))
                                    .cast::<u8>())
                                    .wrapping_offset((battleMode) as isize * 4))
                                    .cast::<u16>())
                                    .wrapping_offset((lvlMode) as isize))
                                    .read(),
                                    4u8,
                                );
                                break 'l2;
                            }
                        }
                    }
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                if (((((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                    .wrapping_add(1612))
                .wrapping_add(1728))
                .cast::<u8>())
                .wrapping_offset((battleMode) as isize * 4))
                .cast::<u16>())
                .wrapping_offset((lvlMode) as isize))
                .read()) as i32)
                    > (((((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(1612))
                    .wrapping_add(1736))
                    .cast::<u8>())
                    .wrapping_offset((battleMode) as isize * 4))
                    .cast::<u16>())
                    .wrapping_offset((lvlMode) as isize))
                    .read()) as i32)
                {
                    (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(1612))
                    .wrapping_add(1736))
                    .cast::<u8>())
                    .wrapping_offset((battleMode) as isize * 4))
                    .cast::<u16>())
                    .wrapping_offset((lvlMode) as isize))
                    .write(
                        (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(1612))
                        .wrapping_add(1728))
                        .cast::<u8>())
                        .wrapping_offset((battleMode) as isize * 4))
                        .cast::<u16>())
                        .wrapping_offset((lvlMode) as isize))
                        .read(),
                    );
                    if ((((((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(1612))
                    .wrapping_add(1728))
                    .cast::<u8>())
                    .wrapping_offset((battleMode) as isize * 4))
                    .cast::<u16>())
                    .wrapping_offset((lvlMode) as isize))
                    .read()) as i32)
                        > 1i32)
                        && ((ShouldAirFrontierTVShow()) != 0)
                    {
                        if battleMode == 0i32 {
                            TryPutFrontierTVShowOnAir(
                                (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                    .wrapping_add(1612))
                                .wrapping_add(1728))
                                .cast::<u8>())
                                .wrapping_offset((battleMode) as isize * 4))
                                .cast::<u16>())
                                .wrapping_offset((lvlMode) as isize))
                                .read(),
                                5u8,
                            );
                        } else {
                            TryPutFrontierTVShowOnAir(
                                (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                    .wrapping_add(1612))
                                .wrapping_add(1728))
                                .cast::<u8>())
                                .wrapping_offset((battleMode) as isize * 4))
                                .cast::<u16>())
                                .wrapping_offset((lvlMode) as isize))
                                .read(),
                                6u8,
                            );
                        }
                    }
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                if (((((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                    .wrapping_add(1612))
                .wrapping_add(1916))
                .cast::<u8>())
                .wrapping_offset((battleMode) as isize * 4))
                .cast::<u16>())
                .wrapping_offset((lvlMode) as isize))
                .read()) as i32)
                    > (((((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(1612))
                    .wrapping_add(1924))
                    .cast::<u8>())
                    .wrapping_offset((battleMode) as isize * 4))
                    .cast::<u16>())
                    .wrapping_offset((lvlMode) as isize))
                    .read()) as i32)
                {
                    (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(1612))
                    .wrapping_add(1924))
                    .cast::<u8>())
                    .wrapping_offset((battleMode) as isize * 4))
                    .cast::<u16>())
                    .wrapping_offset((lvlMode) as isize))
                    .write(
                        (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(1612))
                        .wrapping_add(1916))
                        .cast::<u8>())
                        .wrapping_offset((battleMode) as isize * 4))
                        .cast::<u16>())
                        .wrapping_offset((lvlMode) as isize))
                        .read(),
                    );
                    if ((((((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(1612))
                    .wrapping_add(1916))
                    .cast::<u8>())
                    .wrapping_offset((battleMode) as isize * 4))
                    .cast::<u16>())
                    .wrapping_offset((lvlMode) as isize))
                    .read()) as i32)
                        > 1i32)
                        && ((ShouldAirFrontierTVShow()) != 0)
                    {
                        if battleMode == 0i32 {
                            TryPutFrontierTVShowOnAir(
                                (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                    .wrapping_add(1612))
                                .wrapping_add(1916))
                                .cast::<u8>())
                                .wrapping_offset((battleMode) as isize * 4))
                                .cast::<u16>())
                                .wrapping_offset((lvlMode) as isize))
                                .read(),
                                11u8,
                            );
                        } else {
                            TryPutFrontierTVShowOnAir(
                                (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                    .wrapping_add(1612))
                                .wrapping_add(1916))
                                .cast::<u8>())
                                .wrapping_offset((battleMode) as isize * 4))
                                .cast::<u16>())
                                .wrapping_offset((lvlMode) as isize))
                                .read(),
                                12u8,
                            );
                        }
                    }
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                if (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                    .wrapping_add(1612))
                .wrapping_add(1934))
                .cast::<u16>())
                .wrapping_offset((lvlMode) as isize))
                .read()) as i32)
                    > (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(1612))
                    .wrapping_add(1938))
                    .cast::<u16>())
                    .wrapping_offset((lvlMode) as isize))
                    .read()) as i32)
                {
                    (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(1612))
                    .wrapping_add(1938))
                    .cast::<u16>())
                    .wrapping_offset((lvlMode) as isize))
                    .write(
                        (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(1612))
                        .wrapping_add(1934))
                        .cast::<u16>())
                        .wrapping_offset((lvlMode) as isize))
                        .read(),
                    );
                    if ((((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(1612))
                    .wrapping_add(1934))
                    .cast::<u16>())
                    .wrapping_offset((lvlMode) as isize))
                    .read()) as i32)
                        > 1i32)
                        && ((ShouldAirFrontierTVShow()) != 0)
                    {
                        TryPutFrontierTVShowOnAir(
                            (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(1612))
                            .wrapping_add(1934))
                            .cast::<u16>())
                            .wrapping_offset((lvlMode) as isize))
                            .read(),
                            10u8,
                        );
                    }
                }
                break 'l1;
            }
            if __sw1 == 4i32 {
                if (((((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                    .wrapping_add(1612))
                .wrapping_add(1942))
                .cast::<u8>())
                .wrapping_offset((battleMode) as isize * 4))
                .cast::<u16>())
                .wrapping_offset((lvlMode) as isize))
                .read()) as i32)
                    > (((((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(1612))
                    .wrapping_add(1950))
                    .cast::<u8>())
                    .wrapping_offset((battleMode) as isize * 4))
                    .cast::<u16>())
                    .wrapping_offset((lvlMode) as isize))
                    .read()) as i32)
                {
                    (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(1612))
                    .wrapping_add(1950))
                    .cast::<u8>())
                    .wrapping_offset((battleMode) as isize * 4))
                    .cast::<u16>())
                    .wrapping_offset((lvlMode) as isize))
                    .write(
                        (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(1612))
                        .wrapping_add(1942))
                        .cast::<u8>())
                        .wrapping_offset((battleMode) as isize * 4))
                        .cast::<u16>())
                        .wrapping_offset((lvlMode) as isize))
                        .read(),
                    );
                    (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(1612))
                    .wrapping_add(1966))
                    .cast::<u8>())
                    .wrapping_offset((battleMode) as isize * 4))
                    .cast::<u16>())
                    .wrapping_offset((lvlMode) as isize))
                    .write(
                        (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(1612))
                        .wrapping_add(1958))
                        .cast::<u8>())
                        .wrapping_offset((battleMode) as isize * 4))
                        .cast::<u16>())
                        .wrapping_offset((lvlMode) as isize))
                        .read(),
                    );
                    if ((((((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(1612))
                    .wrapping_add(1942))
                    .cast::<u8>())
                    .wrapping_offset((battleMode) as isize * 4))
                    .cast::<u16>())
                    .wrapping_offset((lvlMode) as isize))
                    .read()) as i32)
                        > 1i32)
                        && ((ShouldAirFrontierTVShow()) != 0)
                    {
                        if battleMode == 0i32 {
                            TryPutFrontierTVShowOnAir(
                                (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                    .wrapping_add(1612))
                                .wrapping_add(1942))
                                .cast::<u8>())
                                .wrapping_offset((battleMode) as isize * 4))
                                .cast::<u16>())
                                .wrapping_offset((lvlMode) as isize))
                                .read(),
                                7u8,
                            );
                        } else {
                            TryPutFrontierTVShowOnAir(
                                (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                    .wrapping_add(1612))
                                .wrapping_add(1942))
                                .cast::<u8>())
                                .wrapping_offset((battleMode) as isize * 4))
                                .cast::<u16>())
                                .wrapping_offset((lvlMode) as isize))
                                .read(),
                                8u8,
                            );
                        }
                    }
                }
                break 'l1;
            }
            if __sw1 == 5i32 {
                if (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                    .wrapping_add(1612))
                .wrapping_add(1976))
                .cast::<u16>())
                .wrapping_offset((lvlMode) as isize))
                .read()) as i32)
                    > (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(1612))
                    .wrapping_add(1980))
                    .cast::<u16>())
                    .wrapping_offset((lvlMode) as isize))
                    .read()) as i32)
                {
                    (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(1612))
                    .wrapping_add(1980))
                    .cast::<u16>())
                    .wrapping_offset((lvlMode) as isize))
                    .write(
                        (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(1612))
                        .wrapping_add(1976))
                        .cast::<u16>())
                        .wrapping_offset((lvlMode) as isize))
                        .read(),
                    );
                    if ((((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(1612))
                    .wrapping_add(1976))
                    .cast::<u16>())
                    .wrapping_offset((lvlMode) as isize))
                    .read()) as i32)
                        > 1i32)
                        && ((ShouldAirFrontierTVShow()) != 0)
                    {
                        TryPutFrontierTVShowOnAir(
                            (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(1612))
                            .wrapping_add(1976))
                            .cast::<u16>())
                            .wrapping_offset((lvlMode) as isize))
                            .read(),
                            9u8,
                        );
                    }
                }
                break 'l1;
            }
            if __sw1 == 6i32 {
                if (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                    .wrapping_add(1612))
                .wrapping_add(1998))
                .cast::<u16>())
                .wrapping_offset((lvlMode) as isize))
                .read()) as i32)
                    > (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(1612))
                    .wrapping_add(2002))
                    .cast::<u16>())
                    .wrapping_offset((lvlMode) as isize))
                    .read()) as i32)
                {
                    (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(1612))
                    .wrapping_add(2002))
                    .cast::<u16>())
                    .wrapping_offset((lvlMode) as isize))
                    .write(
                        (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(1612))
                        .wrapping_add(1998))
                        .cast::<u16>())
                        .wrapping_offset((lvlMode) as isize))
                        .read(),
                    );
                    if ((((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(1612))
                    .wrapping_add(1998))
                    .cast::<u16>())
                    .wrapping_offset((lvlMode) as isize))
                    .read()) as i32)
                        > 1i32)
                        && ((ShouldAirFrontierTVShow()) != 0)
                    {
                        TryPutFrontierTVShowOnAir(
                            (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(1612))
                            .wrapping_add(1998))
                            .cast::<u16>())
                            .wrapping_offset((lvlMode) as isize))
                            .read(),
                            13u8,
                        );
                    }
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Script_GetFrontierBrainStatus() {
    unsafe {
        VarGet(16591u16);
        ((&raw mut gSpecialVar_Result).cast::<u16>()).write(((GetFrontierBrainStatus()) as u16));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetFrontierBrainStatus() -> u8 {
    unsafe {
        let mut status: i32 = 0i32;
        let mut facility: i32 = ((VarGet(16591u16)) as i32);
        let mut battleMode: i32 = ((VarGet(16590u16)) as i32);
        let mut winStreakNoModifier: u16 = ((GetCurrentFacilityWinStreak()) as u16);
        let mut winStreak: i32 = ((winStreakNoModifier) as i32).wrapping_add(
            ((((((((&raw const sFrontierBrainStreakAppearances)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>())
            .wrapping_offset((facility) as isize * 4))
            .cast::<u8>())
            .wrapping_offset(3))
            .read()) as i32),
        );
        let mut symbolsCount: i32 = 0i32;
        if battleMode != 0i32 {
            return 0u8;
        }
        symbolsCount = ((GetPlayerSymbolCountForFacility(((facility) as u8))) as i32);
        'l1: {
            let __sw1 = symbolsCount;
            let __matched = __sw1 == 0i32 || __sw1 == 1i32 || __sw1 == 2i32;
            if __sw1 == 0i32 || __sw1 == 1i32 {
                if winStreak
                    == ((((((((&raw const sFrontierBrainStreakAppearances)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>())
                    .wrapping_offset((facility) as isize * 4))
                    .cast::<u8>())
                    .wrapping_offset((symbolsCount) as isize))
                    .read()) as i32)
                {
                    status = (symbolsCount).wrapping_add(1i32);
                }
                break 'l1;
            }
            if __sw1 == 2i32 || !__matched {
                if winStreak
                    == (((((((&raw const sFrontierBrainStreakAppearances)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>())
                    .wrapping_offset((facility) as isize * 4))
                    .cast::<u8>())
                    .read()) as i32)
                {
                    status = 3i32;
                } else {
                    if winStreak
                        == ((((((((&raw const sFrontierBrainStreakAppearances)
                            .cast::<u8>()
                            .cast_mut())
                        .cast::<u8>())
                        .wrapping_offset((facility) as isize * 4))
                        .cast::<u8>())
                        .wrapping_offset(1))
                        .read()) as i32)
                    {
                        status = 4i32;
                    } else {
                        if (winStreak
                            > ((((((((&raw const sFrontierBrainStreakAppearances)
                                .cast::<u8>()
                                .cast_mut())
                            .cast::<u8>())
                            .wrapping_offset((facility) as isize * 4))
                            .cast::<u8>())
                            .wrapping_offset(1))
                            .read()) as i32))
                            && (crate::c::rem_i32(
                                (winStreak).wrapping_sub(
                                    ((((((((&raw const sFrontierBrainStreakAppearances)
                                        .cast::<u8>()
                                        .cast_mut())
                                    .cast::<u8>())
                                    .wrapping_offset((facility) as isize * 4))
                                    .cast::<u8>())
                                    .wrapping_offset(1))
                                    .read()) as i32),
                                ),
                                ((((((((&raw const sFrontierBrainStreakAppearances)
                                    .cast::<u8>()
                                    .cast_mut())
                                .cast::<u8>())
                                .wrapping_offset((facility) as isize * 4))
                                .cast::<u8>())
                                .wrapping_offset(2))
                                .read()) as i32),
                            ) == 0i32)
                        {
                            status = 4i32;
                        }
                    }
                }
                break 'l1;
            }
        }
        return ((status) as u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CopyFrontierTrainerText(whichText: u8, trainerId: u16) {
    unsafe {
        let mut whichText = whichText;
        let mut trainerId = trainerId;
        'l1: {
            let __sw1 = ((whichText) as i32);
            if __sw1 == 0i32 {
                if ((trainerId) as i32) == 500i32 {
                    FrontierSpeechToString(
                        ((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(1612))
                        .wrapping_add(1440))
                        .wrapping_add(16))
                        .cast::<u16>(),
                    );
                } else {
                    if ((trainerId) as i32) == 1022i32 {
                        CopyFrontierBrainText(0u8);
                    } else {
                        if ((trainerId) as i32) < 300i32 {
                            FrontierSpeechToString(
                                (((((&raw mut gFacilityTrainers).cast::<*mut u8>()).read())
                                    .wrapping_offset(((trainerId) as i32) as isize * 52))
                                .wrapping_add(12))
                                .cast::<u16>(),
                            );
                        } else {
                            if ((trainerId) as i32) < 400i32 {
                                FrontierSpeechToString(
                                    ((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(1612))
                                    .wrapping_add(236))
                                    .cast::<u8>())
                                    .wrapping_offset(
                                        (((trainerId) as i32).wrapping_sub(300i32)) as isize * 236,
                                    ))
                                    .wrapping_add(16))
                                    .cast::<u16>(),
                                );
                            } else {
                                BufferApprenticeChallengeText(
                                    ((((trainerId) as i32).wrapping_sub(400i32)) as u8),
                                );
                            }
                        }
                    }
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                if ((trainerId) as i32) == 500i32 {
                    FrontierSpeechToString(
                        ((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(1612))
                        .wrapping_add(1440))
                        .wrapping_add(28))
                        .cast::<u16>(),
                    );
                } else {
                    if ((trainerId) as i32) == 1022i32 {
                        CopyFrontierBrainText(0u8);
                    } else {
                        if ((trainerId) as i32) < 300i32 {
                            FrontierSpeechToString(
                                (((((&raw mut gFacilityTrainers).cast::<*mut u8>()).read())
                                    .wrapping_offset(((trainerId) as i32) as isize * 52))
                                .wrapping_add(24))
                                .cast::<u16>(),
                            );
                        } else {
                            if ((trainerId) as i32) < 400i32 {
                                if (((&raw mut gBattleTypeFlags).cast::<u32>()).read()
                                    & 16777216u32)
                                    != 0
                                {
                                    FrontierSpeechToString(GetRecordedBattleEasyChatSpeech());
                                } else {
                                    FrontierSpeechToString(
                                        ((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>())
                                            .read())
                                        .wrapping_add(1612))
                                        .wrapping_add(236))
                                        .cast::<u8>())
                                        .wrapping_offset(
                                            (((trainerId) as i32).wrapping_sub(300i32)) as isize
                                                * 236,
                                        ))
                                        .wrapping_add(28))
                                        .cast::<u16>(),
                                    );
                                }
                            } else {
                                if (((&raw mut gBattleTypeFlags).cast::<u32>()).read()
                                    & 16777216u32)
                                    != 0
                                {
                                    FrontierSpeechToString(GetRecordedBattleEasyChatSpeech());
                                } else {
                                    FrontierSpeechToString(
                                        (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>())
                                            .read())
                                        .wrapping_add(220))
                                        .cast::<u8>())
                                        .wrapping_offset(
                                            (((trainerId) as i32).wrapping_sub(400i32)) as isize
                                                * 68,
                                        ))
                                        .wrapping_add(40))
                                        .cast::<u16>(),
                                    );
                                }
                            }
                        }
                    }
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                if ((trainerId) as i32) == 500i32 {
                    FrontierSpeechToString(
                        ((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(1612))
                        .wrapping_add(1440))
                        .wrapping_add(40))
                        .cast::<u16>(),
                    );
                } else {
                    if ((trainerId) as i32) == 1022i32 {
                        CopyFrontierBrainText(1u8);
                    } else {
                        if ((trainerId) as i32) < 300i32 {
                            FrontierSpeechToString(
                                (((((&raw mut gFacilityTrainers).cast::<*mut u8>()).read())
                                    .wrapping_offset(((trainerId) as i32) as isize * 52))
                                .wrapping_add(36))
                                .cast::<u16>(),
                            );
                        } else {
                            if ((trainerId) as i32) < 400i32 {
                                if (((&raw mut gBattleTypeFlags).cast::<u32>()).read()
                                    & 16777216u32)
                                    != 0
                                {
                                    FrontierSpeechToString(GetRecordedBattleEasyChatSpeech());
                                } else {
                                    FrontierSpeechToString(
                                        ((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>())
                                            .read())
                                        .wrapping_add(1612))
                                        .wrapping_add(236))
                                        .cast::<u8>())
                                        .wrapping_offset(
                                            (((trainerId) as i32).wrapping_sub(300i32)) as isize
                                                * 236,
                                        ))
                                        .wrapping_add(40))
                                        .cast::<u16>(),
                                    );
                                }
                            } else {
                                if (((&raw mut gBattleTypeFlags).cast::<u32>()).read()
                                    & 16777216u32)
                                    != 0
                                {
                                    trainerId = ((GetRecordedBattleApprenticeId()) as u16);
                                    FrontierSpeechToString(
                                        ((((&raw mut gApprentices).cast::<u8>())
                                            .wrapping_offset(((trainerId) as i32) as isize * 88))
                                        .wrapping_add(74))
                                        .cast::<u16>(),
                                    );
                                } else {
                                    trainerId = ((crate::c::bf_read(
                                        ((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>())
                                            .read())
                                        .wrapping_add(220))
                                        .cast::<u8>())
                                        .wrapping_offset(
                                            (((trainerId) as i32).wrapping_sub(400i32)) as isize
                                                * 68,
                                        ))
                                        .wrapping_add(0),
                                        0,
                                        5,
                                        false,
                                    ) as u8)
                                        as u16);
                                    FrontierSpeechToString(
                                        ((((&raw mut gApprentices).cast::<u8>())
                                            .wrapping_offset(((trainerId) as i32) as isize * 88))
                                        .wrapping_add(74))
                                        .cast::<u16>(),
                                    );
                                }
                            }
                        }
                    }
                }
                break 'l1;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ResetWinStreaks() {
    unsafe {
        let mut battleMode: i32 = 0i32;
        let mut lvlMode: i32 = 0i32;
        (((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
            .wrapping_add(1680)
            .cast::<u32>())
        .write(0u32);
        {
            battleMode = 0i32;
            'l1: loop {
                if !(battleMode < 4i32) {
                    break 'l1;
                }
                'l2: {
                    {
                        lvlMode = 0i32;
                        'l3: loop {
                            if !(lvlMode < 2i32) {
                                break 'l3;
                            }
                            'l4: {
                                (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                    .wrapping_add(1612))
                                .wrapping_add(1684))
                                .cast::<u8>())
                                .wrapping_offset((battleMode) as isize * 4))
                                .cast::<u16>())
                                .wrapping_offset((lvlMode) as isize))
                                .write(0u16);
                                if battleMode < 2i32 {
                                    (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(1612))
                                    .wrapping_add(1728))
                                    .cast::<u8>())
                                    .wrapping_offset((battleMode) as isize * 4))
                                    .cast::<u16>())
                                    .wrapping_offset((lvlMode) as isize))
                                    .write(0u16);
                                    (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(1612))
                                    .wrapping_add(1916))
                                    .cast::<u8>())
                                    .wrapping_offset((battleMode) as isize * 4))
                                    .cast::<u16>())
                                    .wrapping_offset((lvlMode) as isize))
                                    .write(0u16);
                                    (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(1612))
                                    .wrapping_add(1942))
                                    .cast::<u8>())
                                    .wrapping_offset((battleMode) as isize * 4))
                                    .cast::<u16>())
                                    .wrapping_offset((lvlMode) as isize))
                                    .write(0u16);
                                }
                                if battleMode == 0i32 {
                                    (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                        .wrapping_add(1612))
                                    .wrapping_add(1934))
                                    .cast::<u16>())
                                    .wrapping_offset((lvlMode) as isize))
                                    .write(0u16);
                                    (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                        .wrapping_add(1612))
                                    .wrapping_add(1976))
                                    .cast::<u16>())
                                    .wrapping_offset((lvlMode) as isize))
                                    .write(0u16);
                                    (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                        .wrapping_add(1612))
                                    .wrapping_add(1998))
                                    .cast::<u16>())
                                    .wrapping_offset((lvlMode) as isize))
                                    .write(0u16);
                                }
                            }
                            lvlMode = (lvlMode).wrapping_add(1);
                        }
                    }
                }
                battleMode = (battleMode).wrapping_add(1);
            }
        }
        if (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
            .wrapping_add(1628))
        .read()) as i32)
            != 0i32
        {
            (((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(1628))
            .write(1u8);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetCurrentFacilityWinStreak() -> u32 {
    unsafe {
        let mut lvlMode: i32 = ((crate::c::bf_read(
            ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(1629),
            0,
            2,
            false,
        ) as u8) as i32);
        let mut battleMode: i32 = ((VarGet(16590u16)) as i32);
        let mut facility: i32 = ((VarGet(16591u16)) as i32);
        'l1: {
            let __sw1 = facility;
            let __matched = __sw1 == 0i32
                || __sw1 == 1i32
                || __sw1 == 2i32
                || __sw1 == 3i32
                || __sw1 == 4i32
                || __sw1 == 5i32
                || __sw1 == 6i32;
            if __sw1 == 0i32 {
                return (((((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                    .wrapping_add(1612))
                .wrapping_add(1684))
                .cast::<u8>())
                .wrapping_offset((battleMode) as isize * 4))
                .cast::<u16>())
                .wrapping_offset((lvlMode) as isize))
                .read()) as u32);
            }
            if __sw1 == 1i32 {
                return (((((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                    .wrapping_add(1612))
                .wrapping_add(1728))
                .cast::<u8>())
                .wrapping_offset((battleMode) as isize * 4))
                .cast::<u16>())
                .wrapping_offset((lvlMode) as isize))
                .read()) as u32);
            }
            if __sw1 == 2i32 {
                return (((((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                    .wrapping_add(1612))
                .wrapping_add(1916))
                .cast::<u8>())
                .wrapping_offset((battleMode) as isize * 4))
                .cast::<u16>())
                .wrapping_offset((lvlMode) as isize))
                .read()) as u32);
            }
            if __sw1 == 3i32 {
                return (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                    .wrapping_add(1612))
                .wrapping_add(1934))
                .cast::<u16>())
                .wrapping_offset((lvlMode) as isize))
                .read()) as u32);
            }
            if __sw1 == 4i32 {
                return (((((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                    .wrapping_add(1612))
                .wrapping_add(1942))
                .cast::<u8>())
                .wrapping_offset((battleMode) as isize * 4))
                .cast::<u16>())
                .wrapping_offset((lvlMode) as isize))
                .read()) as u32);
            }
            if __sw1 == 5i32 {
                return (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                    .wrapping_add(1612))
                .wrapping_add(1976))
                .cast::<u16>())
                .wrapping_offset((lvlMode) as isize))
                .read()) as u32);
            }
            if __sw1 == 6i32 {
                return (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                    .wrapping_add(1612))
                .wrapping_add(1998))
                .cast::<u16>())
                .wrapping_offset((lvlMode) as isize))
                .read()) as u32);
            }
            if !__matched {
                return 0u32;
            }
        }
        #[allow(unreachable_code)]
        {
            return 0u32;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ResetFrontierTrainerIds() {
    unsafe {
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < ((crate::c::div_u32(40u32, 2u32)) as i32)) {
                    break 'l1;
                }
                'l2: {
                    (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(1612))
                    .wrapping_add(1640))
                    .cast::<u16>())
                    .wrapping_offset((i) as isize))
                    .write(65535u16);
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn IsTrainerFrontierBrain() {
    unsafe {
        if ((((&raw mut gTrainerBattleOpponent_A).cast::<u16>()).read()) as i32) == 1022i32 {
            ((&raw mut gSpecialVar_Result).cast::<u16>()).write(1u16);
        } else {
            ((&raw mut gSpecialVar_Result).cast::<u16>()).write(0u16);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetPlayerSymbolCountForFacility(facility: u8) -> u8 {
    unsafe {
        let mut facility = facility;
        return ((((FlagGet(
            (((2244i32).wrapping_add(((facility) as i32).wrapping_mul(2i32))) as u16),
        )) as i32)
            .wrapping_add(
                ((FlagGet(
                    (((2245i32).wrapping_add(((facility) as i32).wrapping_mul(2i32))) as u16),
                )) as i32),
            )) as u8);
    }
}
pub(crate) unsafe extern "C" fn GiveBattlePoints() {
    unsafe {
        let mut challengeNum: i32 = 0i32;
        let mut lvlMode: i32 = ((crate::c::bf_read(
            ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(1629),
            0,
            2,
            false,
        ) as u8) as i32);
        let mut facility: i32 = ((VarGet(16591u16)) as i32);
        let mut battleMode: i32 = ((VarGet(16590u16)) as i32);
        let mut points: i32 = 0i32;
        'l1: {
            let __sw1 = facility;
            if __sw1 == 0i32 {
                challengeNum = crate::c::div_i32(
                    (((((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(1612))
                    .wrapping_add(1684))
                    .cast::<u8>())
                    .wrapping_offset((battleMode) as isize * 4))
                    .cast::<u16>())
                    .wrapping_offset((lvlMode) as isize))
                    .read()) as i32),
                    7i32,
                );
                break 'l1;
            }
            if __sw1 == 1i32 {
                challengeNum = (((((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                    .wrapping_add(1612))
                .wrapping_add(1728))
                .cast::<u8>())
                .wrapping_offset((battleMode) as isize * 4))
                .cast::<u16>())
                .wrapping_offset((lvlMode) as isize))
                .read()) as i32);
                break 'l1;
            }
            if __sw1 == 2i32 {
                challengeNum = crate::c::div_i32(
                    (((((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(1612))
                    .wrapping_add(1916))
                    .cast::<u8>())
                    .wrapping_offset((battleMode) as isize * 4))
                    .cast::<u16>())
                    .wrapping_offset((lvlMode) as isize))
                    .read()) as i32),
                    7i32,
                );
                break 'l1;
            }
            if __sw1 == 3i32 {
                challengeNum = crate::c::div_i32(
                    (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(1612))
                    .wrapping_add(1934))
                    .cast::<u16>())
                    .wrapping_offset((lvlMode) as isize))
                    .read()) as i32),
                    7i32,
                );
                break 'l1;
            }
            if __sw1 == 4i32 {
                challengeNum = crate::c::div_i32(
                    (((((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(1612))
                    .wrapping_add(1942))
                    .cast::<u8>())
                    .wrapping_offset((battleMode) as isize * 4))
                    .cast::<u16>())
                    .wrapping_offset((lvlMode) as isize))
                    .read()) as i32),
                    7i32,
                );
                break 'l1;
            }
            if __sw1 == 5i32 {
                challengeNum = crate::c::div_i32(
                    (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(1612))
                    .wrapping_add(1976))
                    .cast::<u16>())
                    .wrapping_offset((lvlMode) as isize))
                    .read()) as i32),
                    14i32,
                );
                break 'l1;
            }
            if __sw1 == 6i32 {
                challengeNum = crate::c::div_i32(
                    (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(1612))
                    .wrapping_add(1998))
                    .cast::<u16>())
                    .wrapping_offset((lvlMode) as isize))
                    .read()) as i32),
                    7i32,
                );
                break 'l1;
            }
        }
        if challengeNum != 0i32 {
            challengeNum = (challengeNum).wrapping_sub(1);
        }
        if ((challengeNum) as u32) >= crate::c::div_u32(840u32, 28u32) {
            challengeNum = (((crate::c::div_u32(840u32, 28u32)).wrapping_sub(1u32)) as i32);
        }
        points = ((((((((((&raw const sBattlePointAwards).cast::<u8>().cast_mut())
            .cast::<u8>())
        .wrapping_offset((challengeNum) as isize * 28))
        .cast::<u8>())
        .wrapping_offset((facility) as isize * 4))
        .cast::<u8>())
        .wrapping_offset((battleMode) as isize))
        .read()) as i32);
        if ((((&raw mut gTrainerBattleOpponent_A).cast::<u16>()).read()) as i32) == 1022i32 {
            points = (points).wrapping_add(10i32);
        }
        let __p2 = ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
            .wrapping_add(2156)
            .cast::<u16>();
        (__p2).write((((((__p2).read()) as i32).wrapping_add(points)) as u16));
        ConvertIntToDecimalStringN((&raw mut gStringVar1).cast::<u8>(), points, 0i32, 2u8);
        if (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
            .wrapping_add(2156)
            .cast::<u16>())
        .read()) as i32)
            > 9999i32
        {
            (((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(2156)
                .cast::<u16>())
            .write(9999u16);
        }
        points = (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
            .wrapping_add(2158)
            .cast::<u16>())
        .read()) as i32);
        points = (points).wrapping_add(
            ((((((((((&raw const sBattlePointAwards).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset((challengeNum) as isize * 28))
            .cast::<u8>())
            .wrapping_offset((facility) as isize * 4))
            .cast::<u8>())
            .wrapping_offset((battleMode) as isize))
            .read()) as i32),
        );
        IncrementDailyBattlePoints(
            ((((((((((&raw const sBattlePointAwards).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset((challengeNum) as isize * 28))
            .cast::<u8>())
            .wrapping_offset((facility) as isize * 4))
            .cast::<u8>())
            .wrapping_offset((battleMode) as isize))
            .read()) as u16),
        );
        if ((((&raw mut gTrainerBattleOpponent_A).cast::<u16>()).read()) as i32) == 1022i32 {
            points = (points).wrapping_add(10i32);
            IncrementDailyBattlePoints(10u16);
        }
        if points > 65535i32 {
            points = 65535i32;
        }
        (((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
            .wrapping_add(2158)
            .cast::<u16>())
        .write(((points) as u16));
    }
}
pub(crate) unsafe extern "C" fn GetFacilitySymbolCount() {
    unsafe {
        let mut facility: i32 = ((VarGet(16591u16)) as i32);
        ((&raw mut gSpecialVar_Result).cast::<u16>())
            .write(((GetPlayerSymbolCountForFacility(((facility) as u8))) as u16));
    }
}
pub(crate) unsafe extern "C" fn GiveFacilitySymbol() {
    unsafe {
        let mut facility: i32 = ((VarGet(16591u16)) as i32);
        if ((GetPlayerSymbolCountForFacility(((facility) as u8))) as i32) == 0i32 {
            FlagSet((((2244i32).wrapping_add((facility).wrapping_mul(2i32))) as u16));
        } else {
            FlagSet((((2245i32).wrapping_add((facility).wrapping_mul(2i32))) as u16));
        }
    }
}
pub(crate) unsafe extern "C" fn CheckBattleTypeFlag() {
    unsafe {
        if (((&raw mut gBattleTypeFlags).cast::<u32>()).read()
            & ((((&raw mut gSpecialVar_0x8005).cast::<u16>()).read()) as u32))
            != 0
        {
            ((&raw mut gSpecialVar_Result).cast::<u16>()).write(1u16);
        } else {
            ((&raw mut gSpecialVar_Result).cast::<u16>()).write(0u16);
        }
    }
}
pub(crate) unsafe extern "C" fn AppendCaughtBannedMonSpeciesName(
    species: u16,
    count: u8,
    numBannedMonsCaught: i32,
) -> u8 {
    unsafe {
        let mut species = species;
        let mut count = count;
        let mut numBannedMonsCaught = numBannedMonsCaught;
        if (GetSetPokedexFlag(SpeciesToNationalPokedexNum(species), 1u8)) != 0 {
            count = (count).wrapping_add(1);
            'l1: {
                let __sw1 = ((count) as i32);
                let __matched = __sw1 == 1i32
                    || __sw1 == 3i32
                    || __sw1 == 5i32
                    || __sw1 == 7i32
                    || __sw1 == 9i32
                    || __sw1 == 11i32
                    || __sw1 == 2i32;
                if __sw1 == 1i32
                    || __sw1 == 3i32
                    || __sw1 == 5i32
                    || __sw1 == 7i32
                    || __sw1 == 9i32
                    || __sw1 == 11i32
                {
                    if numBannedMonsCaught == ((count) as i32) {
                        StringAppend(
                            (&raw mut gStringVar1).cast::<u8>(),
                            (&raw mut gText_SpaceAndSpace).cast::<u8>(),
                        );
                    } else {
                        if numBannedMonsCaught > ((count) as i32) {
                            StringAppend(
                                (&raw mut gStringVar1).cast::<u8>(),
                                (&raw mut gText_CommaSpace).cast::<u8>(),
                            );
                        }
                    }
                    break 'l1;
                }
                if __sw1 == 2i32 {
                    if ((count) as i32) == numBannedMonsCaught {
                        StringAppend(
                            (&raw mut gStringVar1).cast::<u8>(),
                            (&raw mut gText_SpaceAndSpace).cast::<u8>(),
                        );
                    } else {
                        StringAppend(
                            (&raw mut gStringVar1).cast::<u8>(),
                            (&raw mut gText_CommaSpace).cast::<u8>(),
                        );
                    }
                    StringAppend(
                        (&raw mut gStringVar1).cast::<u8>(),
                        (&raw mut gText_NewLine).cast::<u8>(),
                    );
                    break 'l1;
                }
                if !__matched {
                    if ((count) as i32) == numBannedMonsCaught {
                        StringAppend(
                            (&raw mut gStringVar1).cast::<u8>(),
                            (&raw mut gText_SpaceAndSpace).cast::<u8>(),
                        );
                    } else {
                        StringAppend(
                            (&raw mut gStringVar1).cast::<u8>(),
                            (&raw mut gText_CommaSpace).cast::<u8>(),
                        );
                    }
                    StringAppend(
                        (&raw mut gStringVar1).cast::<u8>(),
                        (&raw mut gText_LineBreak).cast::<u8>(),
                    );
                    break 'l1;
                }
            }
            StringAppend(
                (&raw mut gStringVar1).cast::<u8>(),
                (((&raw mut gSpeciesNames).cast::<u8>())
                    .wrapping_offset(((species) as i32) as isize * 11))
                .cast::<u8>(),
            );
        }
        return count;
    }
}
pub(crate) unsafe extern "C" fn AppendIfValid(
    species: u16,
    heldItem: u16,
    hp: u16,
    lvlMode: u8,
    monLevel: u8,
    speciesArray: *mut u16,
    itemsArray: *mut u16,
    count: *mut u8,
) {
    unsafe {
        let mut species = species;
        let mut heldItem = heldItem;
        let mut hp = hp;
        let mut lvlMode = lvlMode;
        let mut monLevel = monLevel;
        let mut speciesArray = speciesArray;
        let mut itemsArray = itemsArray;
        let mut count = count;
        let mut i: i32 = 0i32;
        if (((species) as i32) == 412i32) || (((species) as i32) == 0i32) {
            return;
        }
        {
            i = 0i32;
            'l1: loop {
                if !((((((((&raw const gFrontierBannedSpecies)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<u16>())
                .cast::<u16>())
                .wrapping_offset((i) as isize))
                .read()) as i32)
                    != 65535i32)
                    && (((((((&raw const gFrontierBannedSpecies)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<u16>())
                    .cast::<u16>())
                    .wrapping_offset((i) as isize))
                    .read()) as i32)
                        != ((species) as i32)))
                {
                    break 'l1;
                }
                'l2: {}
                i = (i).wrapping_add(1);
            }
        }
        if ((((((&raw const gFrontierBannedSpecies)
            .cast::<u8>()
            .cast_mut()
            .cast::<u16>())
        .cast::<u16>())
        .wrapping_offset((i) as isize))
        .read()) as i32)
            != 65535i32
        {
            return;
        }
        if (((lvlMode) as i32) == 0i32) && (((monLevel) as i32) > 50i32) {
            return;
        }
        {
            i = 0i32;
            'l3: loop {
                if !((i < (((count).read()) as i32))
                    && (((((speciesArray).wrapping_offset((i) as isize)).read()) as i32)
                        != ((species) as i32)))
                {
                    break 'l3;
                }
                'l4: {}
                i = (i).wrapping_add(1);
            }
        }
        if i != (((count).read()) as i32) {
            return;
        }
        if ((heldItem) as i32) != 0i32 {
            {
                i = 0i32;
                'l5: loop {
                    if !((i < (((count).read()) as i32))
                        && (((((itemsArray).wrapping_offset((i) as isize)).read()) as i32)
                            != ((heldItem) as i32)))
                    {
                        break 'l5;
                    }
                    'l6: {}
                    i = (i).wrapping_add(1);
                }
            }
            if i != (((count).read()) as i32) {
                return;
            }
        }
        ((speciesArray).wrapping_offset((((count).read()) as i32) as isize)).write(species);
        ((itemsArray).wrapping_offset((((count).read()) as i32) as isize)).write(heldItem);
        (count).write(((count).read()).wrapping_add(1));
    }
}
pub(crate) unsafe extern "C" fn CheckPartyIneligibility() {
    unsafe {
        let mut speciesArray = crate::ffi::Align4([0u8; 12]);
        let mut itemArray = crate::ffi::Align4([0u8; 12]);
        let mut monId: i32 = 0i32;
        let mut toChoose: i32 = 0i32;
        let mut count: u8 = 0u8;
        let mut battleMode: i32 = ((VarGet(16590u16)) as i32);
        let mut monIdLooper: i32 = 0i32;
        'l1: {
            let __sw1 = battleMode;
            if __sw1 == 0i32 {
                toChoose = 3i32;
                break 'l1;
            }
            if __sw1 == 2i32 || __sw1 == 3i32 {
                toChoose = 2i32;
                break 'l1;
            }
            if __sw1 == 1i32 {
                if ((VarGet(16591u16)) as i32) == 0i32 {
                    toChoose = 4i32;
                } else {
                    toChoose = 3i32;
                }
                break 'l1;
            }
        }
        monIdLooper = 0i32;
        'l2: loop {
            'l3: {
                monId = monIdLooper;
                count = 0u8;
                'l4: loop {
                    'l5: {
                        let mut species: u16 = ((GetMonData2(
                            ((&raw mut gPlayerParty).cast::<u8>())
                                .wrapping_offset((monId) as isize * 100),
                            65i32,
                        )) as u16);
                        let mut heldItem: u16 = ((GetMonData2(
                            ((&raw mut gPlayerParty).cast::<u8>())
                                .wrapping_offset((monId) as isize * 100),
                            12i32,
                        )) as u16);
                        let mut level: u8 = ((GetMonData2(
                            ((&raw mut gPlayerParty).cast::<u8>())
                                .wrapping_offset((monId) as isize * 100),
                            56i32,
                        )) as u8);
                        let mut hp: u16 = ((GetMonData2(
                            ((&raw mut gPlayerParty).cast::<u8>())
                                .wrapping_offset((monId) as isize * 100),
                            57i32,
                        )) as u16);
                        if ((VarGet(16591u16)) as i32) == 6i32 {
                            if ((heldItem) as i32) == 0i32 {
                                AppendIfValid(
                                    species,
                                    heldItem,
                                    hp,
                                    ((((&raw mut gSpecialVar_Result).cast::<u16>()).read()) as u8),
                                    level,
                                    (&raw mut speciesArray).cast::<u16>(),
                                    (&raw mut itemArray).cast::<u16>(),
                                    &raw mut count,
                                );
                            }
                        } else {
                            AppendIfValid(
                                species,
                                heldItem,
                                hp,
                                ((((&raw mut gSpecialVar_Result).cast::<u16>()).read()) as u8),
                                level,
                                (&raw mut speciesArray).cast::<u16>(),
                                (&raw mut itemArray).cast::<u16>(),
                                &raw mut count,
                            );
                        }
                        monId = (monId).wrapping_add(1);
                        if monId >= 6i32 {
                            monId = 0i32;
                        }
                    }
                    if !(monId != monIdLooper) {
                        break 'l4;
                    }
                }
                monIdLooper = (monIdLooper).wrapping_add(1);
            }
            if !((monIdLooper < 6i32) && (((count) as i32) < toChoose)) {
                break 'l2;
            }
        }
        if ((count) as i32) < toChoose {
            let mut i: i32 = 0i32;
            let mut caughtBannedMons: i32 = 0i32;
            let mut species: i32 = (((((&raw const gFrontierBannedSpecies)
                .cast::<u8>()
                .cast_mut()
                .cast::<u16>())
            .cast::<u16>())
            .read()) as i32);
            {
                i = 0i32;
                'l6: loop {
                    if !(species != 65535i32) {
                        break 'l6;
                    }
                    'l7: {
                        if (GetSetPokedexFlag(SpeciesToNationalPokedexNum(((species) as u16)), 1u8))
                            != 0
                        {
                            caughtBannedMons = (caughtBannedMons).wrapping_add(1);
                        }
                    }
                    i = (i).wrapping_add(1);
                    species = ((((((&raw const gFrontierBannedSpecies)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<u16>())
                    .cast::<u16>())
                    .wrapping_offset((i) as isize))
                    .read()) as i32);
                }
            }
            ((&raw mut gStringVar1).cast::<u8>()).write(255u8);
            ((&raw mut gSpecialVar_0x8004).cast::<u16>()).write(1u16);
            count = 0u8;
            {
                i = 0i32;
                'l8: loop {
                    if !(((((((&raw const gFrontierBannedSpecies)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<u16>())
                    .cast::<u16>())
                    .wrapping_offset((i) as isize))
                    .read()) as i32)
                        != 65535i32)
                    {
                        break 'l8;
                    }
                    'l9: {
                        count = AppendCaughtBannedMonSpeciesName(
                            ((((&raw const gFrontierBannedSpecies)
                                .cast::<u8>()
                                .cast_mut()
                                .cast::<u16>())
                            .cast::<u16>())
                            .wrapping_offset((i) as isize))
                            .read(),
                            count,
                            caughtBannedMons,
                        );
                    }
                    i = (i).wrapping_add(1);
                }
            }
            if ((count) as i32) == 0i32 {
                StringAppend(
                    (&raw mut gStringVar1).cast::<u8>(),
                    (&raw mut gText_Space2).cast::<u8>(),
                );
                StringAppend(
                    (&raw mut gStringVar1).cast::<u8>(),
                    (&raw mut gText_Are).cast::<u8>(),
                );
            } else {
                if (((count) as i32) & 1i32) != 0 {
                    StringAppend(
                        (&raw mut gStringVar1).cast::<u8>(),
                        (&raw mut gText_LineBreak).cast::<u8>(),
                    );
                } else {
                    StringAppend(
                        (&raw mut gStringVar1).cast::<u8>(),
                        (&raw mut gText_Space2).cast::<u8>(),
                    );
                }
                StringAppend(
                    (&raw mut gStringVar1).cast::<u8>(),
                    (&raw mut gText_Are2).cast::<u8>(),
                );
            }
        } else {
            ((&raw mut gSpecialVar_0x8004).cast::<u16>()).write(0u16);
            crate::c::bf_write(
                ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                    .wrapping_add(1629),
                0,
                2,
                ((((&raw mut gSpecialVar_Result).cast::<u16>()).read()) as u8) as i32,
            );
        }
    }
}
pub(crate) unsafe extern "C" fn ValidateVisitingTrainer() {
    unsafe {
        ValidateEReaderTrainer();
    }
}
pub(crate) unsafe extern "C" fn IncrementWinStreak() {
    unsafe {
        let mut lvlMode: i32 = ((crate::c::bf_read(
            ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(1629),
            0,
            2,
            false,
        ) as u8) as i32);
        let mut battleMode: i32 = ((VarGet(16590u16)) as i32);
        let mut facility: i32 = ((VarGet(16591u16)) as i32);
        'l1: {
            let __sw1 = facility;
            if __sw1 == 0i32 {
                if (((((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                    .wrapping_add(1612))
                .wrapping_add(1684))
                .cast::<u8>())
                .wrapping_offset((battleMode) as isize * 4))
                .cast::<u16>())
                .wrapping_offset((lvlMode) as isize))
                .read()) as i32)
                    < 9999i32
                {
                    let __p2 = ((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(1612))
                    .wrapping_add(1684))
                    .cast::<u8>())
                    .wrapping_offset((battleMode) as isize * 4))
                    .cast::<u16>())
                    .wrapping_offset((lvlMode) as isize);
                    (__p2).write(((__p2).read()).wrapping_add(1));
                    if battleMode == 0i32 {
                        SetGameStat(
                            32u8,
                            (((((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(1612))
                            .wrapping_add(1684))
                            .cast::<u8>())
                            .wrapping_offset((battleMode) as isize * 4))
                            .cast::<u16>())
                            .wrapping_offset((lvlMode) as isize))
                            .read()) as u32),
                        );
                        (((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(1612))
                        .wrapping_add(1718)
                        .cast::<u16>())
                        .write(
                            (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(1612))
                            .wrapping_add(1684))
                            .cast::<u8>())
                            .wrapping_offset((battleMode) as isize * 4))
                            .cast::<u16>())
                            .wrapping_offset((lvlMode) as isize))
                            .read(),
                        );
                    }
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                if (((((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                    .wrapping_add(1612))
                .wrapping_add(1728))
                .cast::<u8>())
                .wrapping_offset((battleMode) as isize * 4))
                .cast::<u16>())
                .wrapping_offset((lvlMode) as isize))
                .read()) as i32)
                    < 9999i32
                {
                    let __p3 = ((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(1612))
                    .wrapping_add(1728))
                    .cast::<u8>())
                    .wrapping_offset((battleMode) as isize * 4))
                    .cast::<u16>())
                    .wrapping_offset((lvlMode) as isize);
                    (__p3).write(((__p3).read()).wrapping_add(1));
                }
                if (((((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                    .wrapping_add(1612))
                .wrapping_add(1744))
                .cast::<u8>())
                .wrapping_offset((battleMode) as isize * 4))
                .cast::<u16>())
                .wrapping_offset((lvlMode) as isize))
                .read()) as i32)
                    < 9999i32
                {
                    let __p4 = ((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(1612))
                    .wrapping_add(1744))
                    .cast::<u8>())
                    .wrapping_offset((battleMode) as isize * 4))
                    .cast::<u16>())
                    .wrapping_offset((lvlMode) as isize);
                    (__p4).write(((__p4).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                if (((((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                    .wrapping_add(1612))
                .wrapping_add(1916))
                .cast::<u8>())
                .wrapping_offset((battleMode) as isize * 4))
                .cast::<u16>())
                .wrapping_offset((lvlMode) as isize))
                .read()) as i32)
                    < 9999i32
                {
                    let __p5 = ((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(1612))
                    .wrapping_add(1916))
                    .cast::<u8>())
                    .wrapping_offset((battleMode) as isize * 4))
                    .cast::<u16>())
                    .wrapping_offset((lvlMode) as isize);
                    (__p5).write(((__p5).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                if (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                    .wrapping_add(1612))
                .wrapping_add(1934))
                .cast::<u16>())
                .wrapping_offset((lvlMode) as isize))
                .read()) as i32)
                    < 9999i32
                {
                    let __p6 = ((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(1612))
                    .wrapping_add(1934))
                    .cast::<u16>())
                    .wrapping_offset((lvlMode) as isize);
                    (__p6).write(((__p6).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 4i32 {
                if (((((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                    .wrapping_add(1612))
                .wrapping_add(1942))
                .cast::<u8>())
                .wrapping_offset((battleMode) as isize * 4))
                .cast::<u16>())
                .wrapping_offset((lvlMode) as isize))
                .read()) as i32)
                    < 9999i32
                {
                    let __p7 = ((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(1612))
                    .wrapping_add(1942))
                    .cast::<u8>())
                    .wrapping_offset((battleMode) as isize * 4))
                    .cast::<u16>())
                    .wrapping_offset((lvlMode) as isize);
                    (__p7).write(((__p7).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 5i32 {
                if (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                    .wrapping_add(1612))
                .wrapping_add(1976))
                .cast::<u16>())
                .wrapping_offset((lvlMode) as isize))
                .read()) as i32)
                    < 9999i32
                {
                    let __p8 = ((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(1612))
                    .wrapping_add(1976))
                    .cast::<u16>())
                    .wrapping_offset((lvlMode) as isize);
                    (__p8).write(((__p8).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 6i32 {
                if (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                    .wrapping_add(1612))
                .wrapping_add(1998))
                .cast::<u16>())
                .wrapping_offset((lvlMode) as isize))
                .read()) as i32)
                    < 9999i32
                {
                    let __p9 = ((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(1612))
                    .wrapping_add(1998))
                    .cast::<u16>())
                    .wrapping_offset((lvlMode) as isize);
                    (__p9).write(((__p9).read()).wrapping_add(1));
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn RestoreHeldItems() {
    unsafe {
        let mut i: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32)
                    < (if 3i32 >= (if 4i32 >= 2i32 { 4i32 } else { 2i32 }) {
                        3i32
                    } else {
                        (if 4i32 >= 2i32 { 4i32 } else { 2i32 })
                    }))
                {
                    break 'l1;
                }
                'l2: {
                    if (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(1612))
                    .wrapping_add(1630))
                    .cast::<u16>())
                    .wrapping_offset(((i) as i32) as isize))
                    .read()) as i32)
                        != 0i32
                    {
                        let mut item: u16 = ((GetMonData3(
                            (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(568))
                            .cast::<u8>())
                            .wrapping_offset(
                                ((((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                    .wrapping_add(1612))
                                .wrapping_add(1630))
                                .cast::<u16>())
                                .wrapping_offset(((i) as i32) as isize))
                                .read()) as i32)
                                    .wrapping_sub(1i32)) as isize
                                    * 100,
                            ),
                            12i32,
                            core::ptr::null_mut(),
                        )) as u16);
                        SetMonData(
                            ((&raw mut gPlayerParty).cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize * 100),
                            12i32,
                            (&raw mut item).cast::<u8>(),
                        );
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SaveRecordBattle() {
    unsafe {
        ((&raw mut gSpecialVar_Result).cast::<u16>())
            .write(((MoveRecordedBattleToSaveData()) as u16));
        crate::c::bf_write(
            ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(1629),
            3,
            1,
            (1u8) as i32,
        );
    }
}
pub(crate) unsafe extern "C" fn BufferFrontierTrainerName() {
    unsafe {
        'l1: {
            let __sw1 = ((((&raw mut gSpecialVar_0x8005).cast::<u16>()).read()) as i32);
            if __sw1 == 0i32 {
                GetFrontierTrainerName(
                    (&raw mut gStringVar1).cast::<u8>(),
                    ((&raw mut gTrainerBattleOpponent_A).cast::<u16>()).read(),
                );
                break 'l1;
            }
            if __sw1 == 1i32 {
                GetFrontierTrainerName(
                    (&raw mut gStringVar2).cast::<u8>(),
                    ((&raw mut gTrainerBattleOpponent_A).cast::<u16>()).read(),
                );
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn ResetSketchedMoves() {
    unsafe {
        let mut i: u8 = 0u8;
        let mut j: u8 = 0u8;
        let mut k: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32)
                    < (if 3i32 >= (if 4i32 >= 2i32 { 4i32 } else { 2i32 }) {
                        3i32
                    } else {
                        (if 4i32 >= 2i32 { 4i32 } else { 2i32 })
                    }))
                {
                    break 'l1;
                }
                'l2: {
                    let mut monId: u16 =
                        (((((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(1612))
                        .wrapping_add(1630))
                        .cast::<u16>())
                        .wrapping_offset(((i) as i32) as isize))
                        .read()) as i32)
                            .wrapping_sub(1i32)) as u16);
                    if ((monId) as i32) < 6i32 {
                        {
                            j = 0u8;
                            'l3: loop {
                                if !(((j) as i32) < 4i32) {
                                    break 'l3;
                                }
                                'l4: {
                                    {
                                        k = 0u8;
                                        'l5: loop {
                                            if !(((k) as i32) < 4i32) {
                                                break 'l5;
                                            }
                                            'l6: {
                                                if GetMonData3(
                                                    (((((&raw mut gSaveBlock1Ptr)
                                                        .cast::<*mut u8>())
                                                    .read())
                                                    .wrapping_add(568))
                                                    .cast::<u8>())
                                                    .wrapping_offset(
                                                        ((((((((((&raw mut gSaveBlock2Ptr)
                                                            .cast::<*mut u8>())
                                                        .read())
                                                        .wrapping_add(1612))
                                                        .wrapping_add(1630))
                                                        .cast::<u16>())
                                                        .wrapping_offset(((i) as i32) as isize))
                                                        .read())
                                                            as i32)
                                                            .wrapping_sub(1i32))
                                                            as isize
                                                            * 100,
                                                    ),
                                                    (13i32).wrapping_add(((k) as i32)),
                                                    core::ptr::null_mut(),
                                                ) == GetMonData3(
                                                    ((&raw mut gPlayerParty).cast::<u8>())
                                                        .wrapping_offset(
                                                            ((i) as i32) as isize * 100,
                                                        ),
                                                    (13i32).wrapping_add(((j) as i32)),
                                                    core::ptr::null_mut(),
                                                ) {
                                                    break 'l5;
                                                }
                                            }
                                            k = (k).wrapping_add(1);
                                        }
                                    }
                                    if ((k) as i32) == 4i32 {
                                        SetMonMoveSlot(
                                            ((&raw mut gPlayerParty).cast::<u8>())
                                                .wrapping_offset(((i) as i32) as isize * 100),
                                            166u16,
                                            j,
                                        );
                                    }
                                }
                                j = (j).wrapping_add(1);
                            }
                        }
                        (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(568))
                        .cast::<u8>())
                        .wrapping_offset(
                            ((((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(1612))
                            .wrapping_add(1630))
                            .cast::<u16>())
                            .wrapping_offset(((i) as i32) as isize))
                            .read()) as i32)
                                .wrapping_sub(1i32)) as isize
                                * 100,
                        )
                        .cast::<crate::c::Rec4<100>>()
                        .write_unaligned(
                            ((&raw mut gPlayerParty).cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize * 100)
                                .cast::<crate::c::Rec4<100>>()
                                .read_unaligned(),
                        );
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SetFacilityBrainObjectEvent() {
    unsafe {
        SetFrontierBrainObjEventGfx(((VarGet(16591u16)) as u8));
    }
}
pub(crate) unsafe extern "C" fn Print1PRecord(
    position: i32,
    x: i32,
    y: i32,
    hallRecord: *mut u8,
    hallFacilityId: i32,
) {
    unsafe {
        let mut position = position;
        let mut x = x;
        let mut y = y;
        let mut hallRecord = hallRecord;
        let mut hallFacilityId = hallFacilityId;
        let mut text = crate::ffi::Align4([0u8; 32]);
        let mut winStreak: u16 = 0u16;
        AddTextPrinterParameterized(
            ((&raw mut gRecordsWindowId).cast::<u8>()).read(),
            1u8,
            (((&raw mut gText_123Dot).cast::<u8>()).wrapping_offset((position) as isize * 3))
                .cast::<u8>(),
            (((x).wrapping_mul(8i32)) as u8),
            ((((8i32).wrapping_mul((y).wrapping_add((5i32).wrapping_mul(position))))
                .wrapping_add(1i32)) as u8),
            255u8,
            None,
        );
        ((((hallRecord).wrapping_add(6)).cast::<u8>()).wrapping_offset(7)).write(255u8);
        if (((hallRecord).wrapping_add(4).cast::<u16>()).read()) != 0 {
            TVShowConvertInternationalString(
                (&raw mut text).cast::<u8>(),
                ((hallRecord).wrapping_add(6)).cast::<u8>(),
                ((((hallRecord).wrapping_add(14)).read()) as i32),
            );
            AddTextPrinterParameterized(
                ((&raw mut gRecordsWindowId).cast::<u8>()).read(),
                1u8,
                (&raw mut text).cast::<u8>(),
                ((((x).wrapping_add(2i32)).wrapping_mul(8i32)) as u8),
                ((((8i32).wrapping_mul((y).wrapping_add((5i32).wrapping_mul(position))))
                    .wrapping_add(1i32)) as u8),
                255u8,
                None,
            );
            winStreak = ((hallRecord).wrapping_add(4).cast::<u16>()).read();
            if ((winStreak) as i32) > 9999i32 {
                winStreak = 9999u16;
            }
            ConvertIntToDecimalStringN(
                (&raw mut gStringVar2).cast::<u8>(),
                ((winStreak) as i32),
                1i32,
                4u8,
            );
            StringExpandPlaceholders(
                (&raw mut gStringVar4).cast::<u8>(),
                ((((&raw const sHallFacilityToRecordsText)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<*mut u8>())
                .cast::<*mut u8>())
                .wrapping_offset((hallFacilityId) as isize))
                .read(),
            );
            AddTextPrinterParameterized(
                ((&raw mut gRecordsWindowId).cast::<u8>()).read(),
                1u8,
                (&raw mut gStringVar4).cast::<u8>(),
                ((GetStringRightAlignXOffset(
                    1i32,
                    ((((&raw const sHallFacilityToRecordsText)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<*mut u8>())
                    .cast::<*mut u8>())
                    .wrapping_offset((hallFacilityId) as isize))
                    .read(),
                    200i32,
                )) as u8),
                ((((8i32).wrapping_mul((y).wrapping_add((5i32).wrapping_mul(position))))
                    .wrapping_add(1i32)) as u8),
                255u8,
                None,
            );
        }
    }
}
pub(crate) unsafe extern "C" fn Print2PRecord(position: i32, x: i32, y: i32, hallRecord: *mut u8) {
    unsafe {
        let mut position = position;
        let mut x = x;
        let mut y = y;
        let mut hallRecord = hallRecord;
        let mut text = crate::ffi::Align4([0u8; 32]);
        let mut winStreak: u16 = 0u16;
        AddTextPrinterParameterized(
            ((&raw mut gRecordsWindowId).cast::<u8>()).read(),
            1u8,
            (((&raw mut gText_123Dot).cast::<u8>()).wrapping_offset((position) as isize * 3))
                .cast::<u8>(),
            (((x).wrapping_mul(8i32)) as u8),
            ((((8i32).wrapping_mul((y).wrapping_add((5i32).wrapping_mul(position))))
                .wrapping_add(1i32)) as u8),
            255u8,
            None,
        );
        if (((hallRecord).wrapping_add(8).cast::<u16>()).read()) != 0 {
            ((((hallRecord).wrapping_add(10)).cast::<u8>()).wrapping_offset(7)).write(255u8);
            ((((hallRecord).wrapping_add(18)).cast::<u8>()).wrapping_offset(7)).write(255u8);
            TVShowConvertInternationalString(
                (&raw mut text).cast::<u8>(),
                ((hallRecord).wrapping_add(10)).cast::<u8>(),
                ((((hallRecord).wrapping_add(26)).read()) as i32),
            );
            AddTextPrinterParameterized(
                ((&raw mut gRecordsWindowId).cast::<u8>()).read(),
                1u8,
                (&raw mut text).cast::<u8>(),
                ((((x).wrapping_add(2i32)).wrapping_mul(8i32)) as u8),
                ((((8i32).wrapping_mul(
                    ((y).wrapping_add((5i32).wrapping_mul(position))).wrapping_sub(1i32),
                ))
                .wrapping_add(1i32)) as u8),
                255u8,
                None,
            );
            if (IsStringJapanese(((hallRecord).wrapping_add(18)).cast::<u8>())) != 0 {
                TVShowConvertInternationalString(
                    (&raw mut text).cast::<u8>(),
                    ((hallRecord).wrapping_add(18)).cast::<u8>(),
                    1i32,
                );
            } else {
                StringCopy(
                    (&raw mut text).cast::<u8>(),
                    ((hallRecord).wrapping_add(18)).cast::<u8>(),
                );
            }
            AddTextPrinterParameterized(
                ((&raw mut gRecordsWindowId).cast::<u8>()).read(),
                1u8,
                (&raw mut text).cast::<u8>(),
                ((((x).wrapping_add(4i32)).wrapping_mul(8i32)) as u8),
                ((((8i32).wrapping_mul(
                    ((y).wrapping_add((5i32).wrapping_mul(position))).wrapping_add(1i32),
                ))
                .wrapping_add(1i32)) as u8),
                255u8,
                None,
            );
            winStreak = ((hallRecord).wrapping_add(8).cast::<u16>()).read();
            if ((winStreak) as i32) > 9999i32 {
                winStreak = 9999u16;
            }
            ConvertIntToDecimalStringN(
                (&raw mut gStringVar2).cast::<u8>(),
                ((winStreak) as i32),
                1i32,
                4u8,
            );
            StringExpandPlaceholders(
                (&raw mut gStringVar4).cast::<u8>(),
                ((((&raw const sHallFacilityToRecordsText)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<*mut u8>())
                .cast::<*mut u8>())
                .wrapping_offset(9))
                .read(),
            );
            AddTextPrinterParameterized(
                ((&raw mut gRecordsWindowId).cast::<u8>()).read(),
                1u8,
                (&raw mut gStringVar4).cast::<u8>(),
                ((GetStringRightAlignXOffset(
                    1i32,
                    ((((&raw const sHallFacilityToRecordsText)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<*mut u8>())
                    .cast::<*mut u8>())
                    .wrapping_offset(9))
                    .read(),
                    200i32,
                )) as u8),
                ((((8i32).wrapping_mul((y).wrapping_add((5i32).wrapping_mul(position))))
                    .wrapping_add(1i32)) as u8),
                255u8,
                None,
            );
        }
    }
}
pub(crate) unsafe extern "C" fn Fill1PRecords(dst: *mut u8, hallFacilityId: i32, lvlMode: i32) {
    unsafe {
        let mut dst = dst;
        let mut hallFacilityId = hallFacilityId;
        let mut lvlMode = lvlMode;
        let mut i: i32 = 0i32;
        let mut j: i32 = 0i32;
        let mut record1P = crate::ffi::Align4([0u8; 64]);
        let mut playerHallRecords: *mut u8 = AllocZeroed(344u32);
        GetPlayerHallRecords(playerHallRecords);
        {
            i = 0i32;
            'l1: loop {
                if !(i < 3i32) {
                    break 'l1;
                }
                'l2: {
                    ((&raw mut record1P).cast::<u8>())
                        .wrapping_offset((i) as isize * 16)
                        .cast::<crate::c::Rec4<16>>()
                        .write_unaligned(
                            (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(540))
                            .cast::<u8>())
                            .wrapping_offset((hallFacilityId) as isize * 96))
                            .cast::<u8>())
                            .wrapping_offset((lvlMode) as isize * 48))
                            .cast::<u8>())
                            .wrapping_offset((i) as isize * 16)
                            .cast::<crate::c::Rec4<16>>()
                            .read_unaligned(),
                        );
                }
                i = (i).wrapping_add(1);
            }
        }
        ((&raw mut record1P).cast::<u8>())
            .wrapping_offset(48)
            .cast::<crate::c::Rec4<16>>()
            .write_unaligned(
                ((((playerHallRecords).cast::<u8>())
                    .wrapping_offset((hallFacilityId) as isize * 32))
                .cast::<u8>())
                .wrapping_offset((lvlMode) as isize * 16)
                .cast::<crate::c::Rec4<16>>()
                .read_unaligned(),
            );
        {
            i = 0i32;
            'l3: loop {
                if !(i < 3i32) {
                    break 'l3;
                }
                'l4: {
                    let mut highestWinStreak: i32 = 0i32;
                    let mut highestId: i32 = 0i32;
                    {
                        j = 0i32;
                        'l5: loop {
                            if !(j < 4i32) {
                                break 'l5;
                            }
                            'l6: {
                                if ((((((&raw mut record1P).cast::<u8>())
                                    .wrapping_offset((j) as isize * 16))
                                .wrapping_add(4)
                                .cast::<u16>())
                                .read()) as i32)
                                    > highestWinStreak
                                {
                                    highestId = j;
                                    highestWinStreak = ((((((&raw mut record1P).cast::<u8>())
                                        .wrapping_offset((j) as isize * 16))
                                    .wrapping_add(4)
                                    .cast::<u16>())
                                    .read())
                                        as i32);
                                }
                            }
                            j = (j).wrapping_add(1);
                        }
                    }
                    if ((((((&raw mut record1P).cast::<u8>()).wrapping_offset(48))
                        .wrapping_add(4)
                        .cast::<u16>())
                    .read()) as i32)
                        >= highestWinStreak
                    {
                        highestId = 3i32;
                    }
                    (dst)
                        .wrapping_offset((i) as isize * 16)
                        .cast::<crate::c::Rec4<16>>()
                        .write_unaligned(
                            ((&raw mut record1P).cast::<u8>())
                                .wrapping_offset((highestId) as isize * 16)
                                .cast::<crate::c::Rec4<16>>()
                                .read_unaligned(),
                        );
                    ((((&raw mut record1P).cast::<u8>())
                        .wrapping_offset((highestId) as isize * 16))
                    .wrapping_add(4)
                    .cast::<u16>())
                    .write(0u16);
                }
                i = (i).wrapping_add(1);
            }
        }
        Free(playerHallRecords);
    }
}
pub(crate) unsafe extern "C" fn Fill2PRecords(dst: *mut u8, lvlMode: i32) {
    unsafe {
        let mut dst = dst;
        let mut lvlMode = lvlMode;
        let mut i: i32 = 0i32;
        let mut j: i32 = 0i32;
        let mut record2P = crate::ffi::Align4([0u8; 112]);
        let mut playerHallRecords: *mut u8 = AllocZeroed(344u32);
        GetPlayerHallRecords(playerHallRecords);
        {
            i = 0i32;
            'l1: loop {
                if !(i < 3i32) {
                    break 'l1;
                }
                'l2: {
                    ((&raw mut record2P).cast::<u8>())
                        .wrapping_offset((i) as isize * 28)
                        .cast::<crate::c::Rec4<28>>()
                        .write_unaligned(
                            (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(1404))
                            .cast::<u8>())
                            .wrapping_offset((lvlMode) as isize * 84))
                            .cast::<u8>())
                            .wrapping_offset((i) as isize * 28)
                            .cast::<crate::c::Rec4<28>>()
                            .read_unaligned(),
                        );
                }
                i = (i).wrapping_add(1);
            }
        }
        ((&raw mut record2P).cast::<u8>())
            .wrapping_offset(84)
            .cast::<crate::c::Rec4<28>>()
            .write_unaligned(
                (((playerHallRecords).wrapping_add(288)).cast::<u8>())
                    .wrapping_offset((lvlMode) as isize * 28)
                    .cast::<crate::c::Rec4<28>>()
                    .read_unaligned(),
            );
        {
            i = 0i32;
            'l3: loop {
                if !(i < 3i32) {
                    break 'l3;
                }
                'l4: {
                    let mut highestWinStreak: i32 = 0i32;
                    let mut highestId: i32 = 0i32;
                    {
                        j = 0i32;
                        'l5: loop {
                            if !(j < 3i32) {
                                break 'l5;
                            }
                            'l6: {
                                if ((((((&raw mut record2P).cast::<u8>())
                                    .wrapping_offset((j) as isize * 28))
                                .wrapping_add(8)
                                .cast::<u16>())
                                .read()) as i32)
                                    > highestWinStreak
                                {
                                    highestId = j;
                                    highestWinStreak = ((((((&raw mut record2P).cast::<u8>())
                                        .wrapping_offset((j) as isize * 28))
                                    .wrapping_add(8)
                                    .cast::<u16>())
                                    .read())
                                        as i32);
                                }
                            }
                            j = (j).wrapping_add(1);
                        }
                    }
                    if ((((((&raw mut record2P).cast::<u8>()).wrapping_offset(84))
                        .wrapping_add(8)
                        .cast::<u16>())
                    .read()) as i32)
                        >= highestWinStreak
                    {
                        highestId = 3i32;
                    }
                    (dst)
                        .wrapping_offset((i) as isize * 28)
                        .cast::<crate::c::Rec4<28>>()
                        .write_unaligned(
                            ((&raw mut record2P).cast::<u8>())
                                .wrapping_offset((highestId) as isize * 28)
                                .cast::<crate::c::Rec4<28>>()
                                .read_unaligned(),
                        );
                    ((((&raw mut record2P).cast::<u8>())
                        .wrapping_offset((highestId) as isize * 28))
                    .wrapping_add(8)
                    .cast::<u16>())
                    .write(0u16);
                }
                i = (i).wrapping_add(1);
            }
        }
        Free(playerHallRecords);
    }
}
pub(crate) unsafe extern "C" fn PrintHallRecords(hallFacilityId: i32, lvlMode: i32) {
    unsafe {
        let mut hallFacilityId = hallFacilityId;
        let mut lvlMode = lvlMode;
        let mut i: i32 = 0i32;
        let mut x: i32 = 0i32;
        let mut records1P = crate::ffi::Align4([0u8; 48]);
        let mut records2P = crate::ffi::Align4([0u8; 84]);
        StringCopy(
            (&raw mut gStringVar1).cast::<u8>(),
            (((((&raw const sRecordsWindowChallengeTexts)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>())
            .wrapping_offset((hallFacilityId) as isize * 8))
            .cast::<*mut u8>())
            .read(),
        );
        StringExpandPlaceholders(
            (&raw mut gStringVar4).cast::<u8>(),
            ((((((&raw const sRecordsWindowChallengeTexts)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>())
            .wrapping_offset((hallFacilityId) as isize * 8))
            .cast::<*mut u8>())
            .wrapping_offset(1))
            .read(),
        );
        AddTextPrinterParameterized(
            ((&raw mut gRecordsWindowId).cast::<u8>()).read(),
            1u8,
            (&raw mut gStringVar4).cast::<u8>(),
            0u8,
            1u8,
            255u8,
            None,
        );
        x = GetStringRightAlignXOffset(
            1i32,
            ((((&raw const sLevelModeText)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u8>())
            .cast::<*mut u8>())
            .wrapping_offset((lvlMode) as isize))
            .read(),
            208i32,
        );
        AddTextPrinterParameterized(
            ((&raw mut gRecordsWindowId).cast::<u8>()).read(),
            1u8,
            ((((&raw const sLevelModeText)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u8>())
            .cast::<*mut u8>())
            .wrapping_offset((lvlMode) as isize))
            .read(),
            ((x) as u8),
            1u8,
            255u8,
            None,
        );
        if hallFacilityId == 9i32 {
            ((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(2197))
            .cast::<u8>())
            .cast::<u8>())
            .wrapping_offset(7))
            .write(255u8);
            (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(2197))
            .cast::<u8>())
            .wrapping_offset(8))
            .cast::<u8>())
            .wrapping_offset(7))
            .write(255u8);
            Fill2PRecords((&raw mut records2P).cast::<u8>(), lvlMode);
            {
                i = 0i32;
                'l1: loop {
                    if !(i < 3i32) {
                        break 'l1;
                    }
                    'l2: {
                        Print2PRecord(
                            i,
                            1i32,
                            4i32,
                            ((&raw mut records2P).cast::<u8>()).wrapping_offset((i) as isize * 28),
                        );
                    }
                    i = (i).wrapping_add(1);
                }
            }
        } else {
            Fill1PRecords((&raw mut records1P).cast::<u8>(), hallFacilityId, lvlMode);
            {
                i = 0i32;
                'l3: loop {
                    if !(i < 3i32) {
                        break 'l3;
                    }
                    'l4: {
                        Print1PRecord(
                            i,
                            1i32,
                            4i32,
                            ((&raw mut records1P).cast::<u8>()).wrapping_offset((i) as isize * 16),
                            hallFacilityId,
                        );
                    }
                    i = (i).wrapping_add(1);
                }
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ShowRankingHallRecordsWindow() {
    unsafe {
        ((&raw mut gRecordsWindowId).cast::<u8>()).write(
            ((AddWindow(
                (&raw const sRankingHallRecordsWindowTemplate)
                    .cast::<u8>()
                    .cast_mut(),
            )) as u8),
        );
        DrawStdWindowFrame(((&raw mut gRecordsWindowId).cast::<u8>()).read(), 0u8);
        FillWindowPixelBuffer(((&raw mut gRecordsWindowId).cast::<u8>()).read(), 17u8);
        PrintHallRecords(
            ((((&raw mut gSpecialVar_0x8005).cast::<u16>()).read()) as i32),
            0i32,
        );
        PutWindowTilemap(((&raw mut gRecordsWindowId).cast::<u8>()).read());
        CopyWindowToVram(((&raw mut gRecordsWindowId).cast::<u8>()).read(), 3u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrollRankingHallRecordsWindow() {
    unsafe {
        FillWindowPixelBuffer(((&raw mut gRecordsWindowId).cast::<u8>()).read(), 17u8);
        PrintHallRecords(
            ((((&raw mut gSpecialVar_0x8005).cast::<u16>()).read()) as i32),
            1i32,
        );
        CopyWindowToVram(((&raw mut gRecordsWindowId).cast::<u8>()).read(), 2u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ClearRankingHallRecords() {
    unsafe {
        let mut i: i32 = 0i32;
        let mut j: i32 = 0i32;
        let mut k: i32 = 0i32;
        let mut emptyId = crate::ffi::Align4([0u8; 4]);
        (&raw mut emptyId).cast::<u8>().wrapping_add(0).write(0u8);
        {
            i = 0i32;
            'l1: loop {
                if !(i < 9i32) {
                    break 'l1;
                }
                'l2: {
                    {
                        j = 0i32;
                        'l3: loop {
                            if !(j < 2i32) {
                                break 'l3;
                            }
                            'l4: {
                                {
                                    k = 0i32;
                                    'l5: loop {
                                        if !(k < 3i32) {
                                            break 'l5;
                                        }
                                        'l6: {
                                            CopyTrainerId(
                                                ((((((((((&raw mut gSaveBlock2Ptr)
                                                    .cast::<*mut u8>())
                                                .read())
                                                .wrapping_add(540))
                                                .cast::<u8>())
                                                .wrapping_offset((i) as isize * 96))
                                                .cast::<u8>())
                                                .wrapping_offset((j) as isize * 48))
                                                .cast::<u8>())
                                                .wrapping_offset((k) as isize * 16))
                                                .cast::<u8>(),
                                                (&raw mut emptyId).cast::<u8>(),
                                            );
                                            ((((((((((((&raw mut gSaveBlock2Ptr)
                                                .cast::<*mut u8>())
                                            .read())
                                            .wrapping_add(540))
                                            .cast::<u8>())
                                            .wrapping_offset((i) as isize * 96))
                                            .cast::<u8>())
                                            .wrapping_offset((j) as isize * 48))
                                            .cast::<u8>())
                                            .wrapping_offset((k) as isize * 16))
                                            .wrapping_add(6))
                                            .cast::<u8>())
                                            .write(255u8);
                                            (((((((((((&raw mut gSaveBlock2Ptr)
                                                .cast::<*mut u8>())
                                            .read())
                                            .wrapping_add(540))
                                            .cast::<u8>())
                                            .wrapping_offset((i) as isize * 96))
                                            .cast::<u8>())
                                            .wrapping_offset((j) as isize * 48))
                                            .cast::<u8>())
                                            .wrapping_offset((k) as isize * 16))
                                            .wrapping_add(4)
                                            .cast::<u16>())
                                            .write(0u16);
                                        }
                                        k = (k).wrapping_add(1);
                                    }
                                }
                            }
                            j = (j).wrapping_add(1);
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        {
            j = 0i32;
            'l7: loop {
                if !(j < 2i32) {
                    break 'l7;
                }
                'l8: {
                    {
                        k = 0i32;
                        'l9: loop {
                            if !(k < 3i32) {
                                break 'l9;
                            }
                            'l10: {
                                CopyTrainerId(
                                    ((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(1404))
                                    .cast::<u8>())
                                    .wrapping_offset((j) as isize * 84))
                                    .cast::<u8>())
                                    .wrapping_offset((k) as isize * 28))
                                    .cast::<u8>(),
                                    (&raw mut emptyId).cast::<u8>(),
                                );
                                CopyTrainerId(
                                    (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(1404))
                                    .cast::<u8>())
                                    .wrapping_offset((j) as isize * 84))
                                    .cast::<u8>())
                                    .wrapping_offset((k) as isize * 28))
                                    .wrapping_add(4))
                                    .cast::<u8>(),
                                    (&raw mut emptyId).cast::<u8>(),
                                );
                                ((((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>())
                                    .read())
                                .wrapping_add(1404))
                                .cast::<u8>())
                                .wrapping_offset((j) as isize * 84))
                                .cast::<u8>())
                                .wrapping_offset((k) as isize * 28))
                                .wrapping_add(10))
                                .cast::<u8>())
                                .write(255u8);
                                ((((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>())
                                    .read())
                                .wrapping_add(1404))
                                .cast::<u8>())
                                .wrapping_offset((j) as isize * 84))
                                .cast::<u8>())
                                .wrapping_offset((k) as isize * 28))
                                .wrapping_add(18))
                                .cast::<u8>())
                                .write(255u8);
                                (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                    .wrapping_add(1404))
                                .cast::<u8>())
                                .wrapping_offset((j) as isize * 84))
                                .cast::<u8>())
                                .wrapping_offset((k) as isize * 28))
                                .wrapping_add(8)
                                .cast::<u16>())
                                .write(0u16);
                            }
                            k = (k).wrapping_add(1);
                        }
                    }
                }
                j = (j).wrapping_add(1);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SaveGameFrontier() {
    unsafe {
        let mut i: i32 = 0i32;
        let mut monsParty: *mut u8 = AllocZeroed(600u32);
        {
            i = 0i32;
            'l1: loop {
                if !(i < 6i32) {
                    break 'l1;
                }
                'l2: {
                    (monsParty)
                        .wrapping_offset((i) as isize * 100)
                        .cast::<crate::c::Rec4<100>>()
                        .write_unaligned(
                            ((&raw mut gPlayerParty).cast::<u8>())
                                .wrapping_offset((i) as isize * 100)
                                .cast::<crate::c::Rec4<100>>()
                                .read_unaligned(),
                        );
                }
                i = (i).wrapping_add(1);
            }
        }
        i = ((((&raw mut gPlayerPartyCount).cast::<u8>()).read()) as i32);
        LoadPlayerParty();
        SetContinueGameWarpStatusToDynamicWarp();
        TrySavingData(1u8);
        ClearContinueGameWarpStatus2();
        ((&raw mut gPlayerPartyCount).cast::<u8>()).write(((i) as u8));
        {
            i = 0i32;
            'l3: loop {
                if !(i < 6i32) {
                    break 'l3;
                }
                'l4: {
                    ((&raw mut gPlayerParty).cast::<u8>())
                        .wrapping_offset((i) as isize * 100)
                        .cast::<crate::c::Rec4<100>>()
                        .write_unaligned(
                            (monsParty)
                                .wrapping_offset((i) as isize * 100)
                                .cast::<crate::c::Rec4<100>>()
                                .read_unaligned(),
                        );
                }
                i = (i).wrapping_add(1);
            }
        }
        Free(monsParty);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetFrontierBrainTrainerPicIndex() -> u8 {
    unsafe {
        let mut facility: i32 = 0i32;
        if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 16777216u32) != 0 {
            facility = ((GetRecordedBattleFrontierFacility()) as i32);
        } else {
            facility = ((VarGet(16591u16)) as i32);
        }
        return ((((&raw mut gTrainers).cast::<u8>()).wrapping_offset(
            ((((((&raw const sFrontierBrainTrainerIds)
                .cast::<u8>()
                .cast_mut()
                .cast::<u16>())
            .cast::<u16>())
            .wrapping_offset((facility) as isize))
            .read()) as i32) as isize
                * 40,
        ))
        .wrapping_add(3))
        .read();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetFrontierBrainTrainerClass() -> u8 {
    unsafe {
        let mut facility: i32 = 0i32;
        if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 16777216u32) != 0 {
            facility = ((GetRecordedBattleFrontierFacility()) as i32);
        } else {
            facility = ((VarGet(16591u16)) as i32);
        }
        return ((((&raw mut gTrainers).cast::<u8>()).wrapping_offset(
            ((((((&raw const sFrontierBrainTrainerIds)
                .cast::<u8>()
                .cast_mut()
                .cast::<u16>())
            .cast::<u16>())
            .wrapping_offset((facility) as isize))
            .read()) as i32) as isize
                * 40,
        ))
        .wrapping_add(1))
        .read();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CopyFrontierBrainTrainerName(dst: *mut u8) {
    unsafe {
        let mut dst = dst;
        let mut i: i32 = 0i32;
        let mut facility: i32 = 0i32;
        if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 16777216u32) != 0 {
            facility = ((GetRecordedBattleFrontierFacility()) as i32);
        } else {
            facility = ((VarGet(16591u16)) as i32);
        }
        {
            i = 0i32;
            'l1: loop {
                if !(i < 7i32) {
                    break 'l1;
                }
                'l2: {
                    ((dst).wrapping_offset((i) as isize)).write(
                        ((((((&raw mut gTrainers).cast::<u8>()).wrapping_offset(
                            ((((((&raw const sFrontierBrainTrainerIds)
                                .cast::<u8>()
                                .cast_mut()
                                .cast::<u16>())
                            .cast::<u16>())
                            .wrapping_offset((facility) as isize))
                            .read()) as i32) as isize
                                * 40,
                        ))
                        .wrapping_add(4))
                        .cast::<u8>())
                        .wrapping_offset((i) as isize))
                        .read(),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        ((dst).wrapping_offset((i) as isize)).write(255u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsFrontierBrainFemale() -> u8 {
    unsafe {
        let mut facility: i32 = ((VarGet(16591u16)) as i32);
        return ((((((&raw const sFrontierBrainObjEventGfx)
            .cast::<u8>()
            .cast_mut())
        .cast::<u8>())
        .wrapping_offset((facility) as isize * 2))
        .cast::<u8>())
        .wrapping_offset(1))
        .read();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetFrontierBrainObjEventGfx_2() {
    unsafe {
        let mut facility: i32 = ((VarGet(16591u16)) as i32);
        VarSet(
            16400u16,
            (((((((&raw const sFrontierBrainObjEventGfx)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>())
            .wrapping_offset((facility) as isize * 2))
            .cast::<u8>())
            .read()) as u16),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CreateFrontierBrainPokemon() {
    unsafe {
        let mut i: i32 = 0i32;
        let mut j: i32 = 0i32;
        let mut selectedMonBits: i32 = 0i32;
        let mut monPartyId: i32 = 0i32;
        let mut monLevel: i32 = 0i32;
        let mut friendship: u8 = 0u8;
        let mut facility: i32 = ((VarGet(16591u16)) as i32);
        let mut symbol: i32 = GetFronterBrainSymbol();
        if facility == 1i32 {
            selectedMonBits =
                GetDomeTrainerSelectedMons(((TrainerIdToDomeTournamentId(1022u16)) as u16));
        } else {
            selectedMonBits = 7i32;
        }
        ZeroEnemyPartyMons();
        monPartyId = 0i32;
        monLevel = ((SetFacilityPtrsGetLevel()) as i32);
        {
            i = 0i32;
            'l1: loop {
                if !(i < 3i32) {
                    break 'l1;
                }
                'l2: {
                    if !((selectedMonBits & 1i32) != 0) {
                        break 'l2;
                    }
                    'l3: loop {
                        'l4: {
                            'l5: loop {
                                'l6: {
                                    j = (((Random()) as i32) | (((Random()) as i32) << 16));
                                }
                                if !((IsShinyOtIdPersonality(61226u32, ((j) as u32))) != 0) {
                                    break 'l5;
                                }
                            }
                        }
                        if !((((((((((((&raw const sFrontierBrainsMons)
                            .cast::<u8>()
                            .cast_mut())
                        .cast::<u8>())
                        .wrapping_offset((facility) as isize * 120))
                        .cast::<u8>())
                        .wrapping_offset((symbol) as isize * 60))
                        .cast::<u8>())
                        .wrapping_offset((i) as isize * 20))
                        .wrapping_add(5))
                        .read()) as i32)
                            != ((GetNatureFromPersonality(((j) as u32))) as i32))
                        {
                            break 'l3;
                        }
                    }
                    CreateMon(
                        ((&raw mut gEnemyParty).cast::<u8>())
                            .wrapping_offset((monPartyId) as isize * 100),
                        (((((((((&raw const sFrontierBrainsMons).cast::<u8>().cast_mut())
                            .cast::<u8>())
                        .wrapping_offset((facility) as isize * 120))
                        .cast::<u8>())
                        .wrapping_offset((symbol) as isize * 60))
                        .cast::<u8>())
                        .wrapping_offset((i) as isize * 20))
                        .cast::<u16>())
                        .read(),
                        ((monLevel) as u8),
                        (((((((((&raw const sFrontierBrainsMons).cast::<u8>().cast_mut())
                            .cast::<u8>())
                        .wrapping_offset((facility) as isize * 120))
                        .cast::<u8>())
                        .wrapping_offset((symbol) as isize * 60))
                        .cast::<u8>())
                        .wrapping_offset((i) as isize * 20))
                        .wrapping_add(4))
                        .read(),
                        1u8,
                        ((j) as u32),
                        1u8,
                        61226u32,
                    );
                    SetMonData(
                        ((&raw mut gEnemyParty).cast::<u8>())
                            .wrapping_offset((monPartyId) as isize * 100),
                        12i32,
                        (((((((((&raw const sFrontierBrainsMons).cast::<u8>().cast_mut())
                            .cast::<u8>())
                        .wrapping_offset((facility) as isize * 120))
                        .cast::<u8>())
                        .wrapping_offset((symbol) as isize * 60))
                        .cast::<u8>())
                        .wrapping_offset((i) as isize * 20))
                        .wrapping_add(2)
                        .cast::<u16>())
                        .cast::<u8>(),
                    );
                    {
                        j = 0i32;
                        'l7: loop {
                            if !(j < 6i32) {
                                break 'l7;
                            }
                            'l8: {
                                SetMonData(
                                    ((&raw mut gEnemyParty).cast::<u8>())
                                        .wrapping_offset((monPartyId) as isize * 100),
                                    (26i32).wrapping_add(j),
                                    ((((((((((&raw const sFrontierBrainsMons)
                                        .cast::<u8>()
                                        .cast_mut())
                                    .cast::<u8>())
                                    .wrapping_offset((facility) as isize * 120))
                                    .cast::<u8>())
                                    .wrapping_offset((symbol) as isize * 60))
                                    .cast::<u8>())
                                    .wrapping_offset((i) as isize * 20))
                                    .wrapping_add(6))
                                    .cast::<u8>())
                                    .wrapping_offset((j) as isize),
                                );
                            }
                            j = (j).wrapping_add(1);
                        }
                    }
                    friendship = 255u8;
                    {
                        j = 0i32;
                        'l9: loop {
                            if !(j < 4i32) {
                                break 'l9;
                            }
                            'l10: {
                                SetMonMoveSlot(
                                    ((&raw mut gEnemyParty).cast::<u8>())
                                        .wrapping_offset((monPartyId) as isize * 100),
                                    (((((((((((&raw const sFrontierBrainsMons)
                                        .cast::<u8>()
                                        .cast_mut())
                                    .cast::<u8>())
                                    .wrapping_offset((facility) as isize * 120))
                                    .cast::<u8>())
                                    .wrapping_offset((symbol) as isize * 60))
                                    .cast::<u8>())
                                    .wrapping_offset((i) as isize * 20))
                                    .wrapping_add(12))
                                    .cast::<u16>())
                                    .wrapping_offset((j) as isize))
                                    .read(),
                                    ((j) as u8),
                                );
                                if (((((((((((((&raw const sFrontierBrainsMons)
                                    .cast::<u8>()
                                    .cast_mut())
                                .cast::<u8>())
                                .wrapping_offset((facility) as isize * 120))
                                .cast::<u8>())
                                .wrapping_offset((symbol) as isize * 60))
                                .cast::<u8>())
                                .wrapping_offset((i) as isize * 20))
                                .wrapping_add(12))
                                .cast::<u16>())
                                .wrapping_offset((j) as isize))
                                .read()) as i32)
                                    == 218i32
                                {
                                    friendship = 0u8;
                                }
                            }
                            j = (j).wrapping_add(1);
                        }
                    }
                    SetMonData(
                        ((&raw mut gEnemyParty).cast::<u8>())
                            .wrapping_offset((monPartyId) as isize * 100),
                        32i32,
                        &raw mut friendship,
                    );
                    CalculateMonStats(
                        ((&raw mut gEnemyParty).cast::<u8>())
                            .wrapping_offset((monPartyId) as isize * 100),
                    );
                    monPartyId = (monPartyId).wrapping_add(1);
                }
                selectedMonBits = (selectedMonBits >> 1);
                i = (i).wrapping_add(1);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetFrontierBrainMonSpecies(monId: u8) -> u16 {
    unsafe {
        let mut monId = monId;
        let mut facility: i32 = ((VarGet(16591u16)) as i32);
        let mut symbol: i32 = GetFronterBrainSymbol();
        return (((((((((&raw const sFrontierBrainsMons).cast::<u8>().cast_mut())
            .cast::<u8>())
        .wrapping_offset((facility) as isize * 120))
        .cast::<u8>())
        .wrapping_offset((symbol) as isize * 60))
        .cast::<u8>())
        .wrapping_offset(((monId) as i32) as isize * 20))
        .cast::<u16>())
        .read();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetFrontierBrainObjEventGfx(facility: u8) {
    unsafe {
        let mut facility = facility;
        ((&raw mut gTrainerBattleOpponent_A).cast::<u16>()).write(1022u16);
        VarSet(
            16400u16,
            (((((((&raw const sFrontierBrainObjEventGfx)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>())
            .wrapping_offset(((facility) as i32) as isize * 2))
            .cast::<u8>())
            .read()) as u16),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetFrontierBrainMonMove(monId: u8, moveSlotId: u8) -> u16 {
    unsafe {
        let mut monId = monId;
        let mut moveSlotId = moveSlotId;
        let mut facility: i32 = ((VarGet(16591u16)) as i32);
        let mut symbol: i32 = GetFronterBrainSymbol();
        return (((((((((((&raw const sFrontierBrainsMons).cast::<u8>().cast_mut())
            .cast::<u8>())
        .wrapping_offset((facility) as isize * 120))
        .cast::<u8>())
        .wrapping_offset((symbol) as isize * 60))
        .cast::<u8>())
        .wrapping_offset(((monId) as i32) as isize * 20))
        .wrapping_add(12))
        .cast::<u16>())
        .wrapping_offset(((moveSlotId) as i32) as isize))
        .read();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetFrontierBrainMonNature(monId: u8) -> u8 {
    unsafe {
        let mut monId = monId;
        let mut facility: i32 = ((VarGet(16591u16)) as i32);
        let mut symbol: i32 = GetFronterBrainSymbol();
        return (((((((((&raw const sFrontierBrainsMons).cast::<u8>().cast_mut())
            .cast::<u8>())
        .wrapping_offset((facility) as isize * 120))
        .cast::<u8>())
        .wrapping_offset((symbol) as isize * 60))
        .cast::<u8>())
        .wrapping_offset(((monId) as i32) as isize * 20))
        .wrapping_add(5))
        .read();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetFrontierBrainMonEvs(monId: u8, evStatId: u8) -> u8 {
    unsafe {
        let mut monId = monId;
        let mut evStatId = evStatId;
        let mut facility: i32 = ((VarGet(16591u16)) as i32);
        let mut symbol: i32 = GetFronterBrainSymbol();
        return (((((((((((&raw const sFrontierBrainsMons).cast::<u8>().cast_mut())
            .cast::<u8>())
        .wrapping_offset((facility) as isize * 120))
        .cast::<u8>())
        .wrapping_offset((symbol) as isize * 60))
        .cast::<u8>())
        .wrapping_offset(((monId) as i32) as isize * 20))
        .wrapping_add(6))
        .cast::<u8>())
        .wrapping_offset(((evStatId) as i32) as isize))
        .read();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetFronterBrainSymbol() -> i32 {
    unsafe {
        let mut facility: i32 = ((VarGet(16591u16)) as i32);
        let mut symbol: i32 = ((GetPlayerSymbolCountForFacility(((facility) as u8))) as i32);
        if symbol == 2i32 {
            let mut winStreak: u16 = ((GetCurrentFacilityWinStreak()) as u16);
            if ((winStreak) as i32).wrapping_add(
                ((((((((&raw const sFrontierBrainStreakAppearances)
                    .cast::<u8>()
                    .cast_mut())
                .cast::<u8>())
                .wrapping_offset((facility) as isize * 4))
                .cast::<u8>())
                .wrapping_offset(3))
                .read()) as i32),
            ) == (((((((&raw const sFrontierBrainStreakAppearances)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>())
            .wrapping_offset((facility) as isize * 4))
            .cast::<u8>())
            .read()) as i32)
            {
                symbol = 0i32;
            } else {
                if ((winStreak) as i32).wrapping_add(
                    ((((((((&raw const sFrontierBrainStreakAppearances)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>())
                    .wrapping_offset((facility) as isize * 4))
                    .cast::<u8>())
                    .wrapping_offset(3))
                    .read()) as i32),
                ) == ((((((((&raw const sFrontierBrainStreakAppearances)
                    .cast::<u8>()
                    .cast_mut())
                .cast::<u8>())
                .wrapping_offset((facility) as isize * 4))
                .cast::<u8>())
                .wrapping_offset(1))
                .read()) as i32)
                {
                    symbol = 1i32;
                } else {
                    if (((winStreak) as i32).wrapping_add(
                        ((((((((&raw const sFrontierBrainStreakAppearances)
                            .cast::<u8>()
                            .cast_mut())
                        .cast::<u8>())
                        .wrapping_offset((facility) as isize * 4))
                        .cast::<u8>())
                        .wrapping_offset(3))
                        .read()) as i32),
                    ) > ((((((((&raw const sFrontierBrainStreakAppearances)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>())
                    .wrapping_offset((facility) as isize * 4))
                    .cast::<u8>())
                    .wrapping_offset(1))
                    .read()) as i32))
                        && (crate::c::rem_i32(
                            (((winStreak) as i32).wrapping_add(
                                ((((((((&raw const sFrontierBrainStreakAppearances)
                                    .cast::<u8>()
                                    .cast_mut())
                                .cast::<u8>())
                                .wrapping_offset((facility) as isize * 4))
                                .cast::<u8>())
                                .wrapping_offset(3))
                                .read()) as i32),
                            ))
                            .wrapping_sub(
                                ((((((((&raw const sFrontierBrainStreakAppearances)
                                    .cast::<u8>()
                                    .cast_mut())
                                .cast::<u8>())
                                .wrapping_offset((facility) as isize * 4))
                                .cast::<u8>())
                                .wrapping_offset(1))
                                .read()) as i32),
                            ),
                            ((((((((&raw const sFrontierBrainStreakAppearances)
                                .cast::<u8>()
                                .cast_mut())
                            .cast::<u8>())
                            .wrapping_offset((facility) as isize * 4))
                            .cast::<u8>())
                            .wrapping_offset(2))
                            .read()) as i32),
                        ) == 0i32)
                    {
                        symbol = 1i32;
                    }
                }
            }
        }
        return symbol;
    }
}
pub(crate) unsafe extern "C" fn CopyFrontierBrainText(playerWonText: u8) {
    unsafe {
        let mut playerWonText = playerWonText;
        let mut facility: i32 = 0i32;
        let mut symbol: i32 = 0i32;
        if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 16777216u32) != 0 {
            facility = ((GetRecordedBattleFrontierFacility()) as i32);
            symbol = ((GetRecordedBattleFronterBrainSymbol()) as i32);
        } else {
            facility = ((VarGet(16591u16)) as i32);
            symbol = GetFronterBrainSymbol();
        }
        'l1: {
            let __sw1 = ((playerWonText) as i32);
            if __sw1 == 0i32 {
                StringCopy(
                    (&raw mut gStringVar4).cast::<u8>(),
                    ((((((&raw const sFrontierBrainPlayerLostTexts)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<*mut *mut u8>())
                    .cast::<*mut *mut u8>())
                    .wrapping_offset((symbol) as isize))
                    .read())
                    .wrapping_offset((facility) as isize))
                    .read(),
                );
                break 'l1;
            }
            if __sw1 == 1i32 {
                StringCopy(
                    (&raw mut gStringVar4).cast::<u8>(),
                    ((((((&raw const sFrontierBrainPlayerWonTexts)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<*mut *mut u8>())
                    .cast::<*mut *mut u8>())
                    .wrapping_offset((symbol) as isize))
                    .read())
                    .wrapping_offset((facility) as isize))
                    .read(),
                );
                break 'l1;
            }
        }
    }
}
