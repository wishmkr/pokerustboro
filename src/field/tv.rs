//! Translated from `src/tv.c` by tools/rustport/c2rs.py, then reviewed.
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

// Data tables (translate with cdata.py): sPokeOutbreakSpeciesList sGoldSymbolFlags sSilverSymbolFlags sNumberOneVarsAndThresholds sPokeNewsTextGroup_Upcoming sPokeNewsTextGroup_Ongoing sPokeNewsTextGroup_Ending gTVStringVarPtrs sTVFanClubTextGroup sTVRecentHappeninssTextGroup sTVFanClubOpinionsTextGroup sTVMassOutbreakTextGroup sTVPokemonTodaySuccessfulTextGroup sTVTodaysSmartShopperTextGroup sTVBravoTrainerTextGroup sTV3CheersForPokeblocksTextGroup sTVBravoTrainerBattleTowerTextGroup sTVContestLiveUpdatesTextGroup sTVPokemonBattleUpdateTextGroup sTVTrainerFanClubSpecialTextGroup sTVNameRaterTextGroup sTVLilycoveContestLadyTextGroup sTVPokemonTodayFailedTextGroup sTVPokemonAnglerTextGroup sTVWorldOfMastersTextGroup sTVTodaysRivalTrainerTextGroup sTVDewfordTrendWatcherNetworkTextGroup sTVHoennTreasureInvestisatorsTextGroup sTVFindThatGamerTextGroup sTVBreakingNewsTextGroup sTVSecretBaseVisitTextGroup sTVPokemonLotteryWinnerFlashReportTextGroup sTVThePokemonBattleSeminarTextGroup sTVTrainerFanClubTextGroup sTVCutiesTextGroup sTVPokemonNewsBattleFrontierTextGroup sTVWhatsNo1InHoennTodayTextGroup sTVSecretBaseSecretsTextGroup sTVSafariFanClubTextGroup sTVInSearchOfTrainersTextGroup sTVSecretBaseSecretsActions
#[allow(unused_imports)]
use crate::data::tv::*;

#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut sCurTVShowSlot: i8 = 0i8;
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut sTV_SecretBaseVisitMovesTemp: crate::ffi::Align4<[u8; 16]> =
    crate::ffi::Align4([0; 16]);
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut sTV_DecorationsBuffer: crate::ffi::Align4<[u8; 16]> = crate::ffi::Align4([0; 16]);
#[unsafe(no_mangle)]
pub static mut sTV_SecretBaseVisitMonsTemp: crate::ffi::Align4<[u8; 80]> =
    crate::ffi::Align4([0; 80]);
pub(crate) static mut sTVShowMixingNumPlayers: u8 = 0u8;
pub(crate) static mut sTVShowNewsMixingNumPlayers: u8 = 0u8;
pub(crate) static mut sTVShowMixingCurSlot: i8 = 0i8;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sPokemonAnglerSpecies: u16 = 0u16;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sPokemonAnglerAttemptCounters: u16 = 0u16;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sFindThatGamerCoinsSpent: u16 = 0u16;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sFindThatGamerWhichGame: u8 = 0u8;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sRecordMixingPartnersWithoutShowsToShare: u8 = 0u8;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sTVShowState: u8 = 0u8;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sTVSecretBaseSecretsRandomValues: crate::ffi::Align4<[u8; 3]> =
    crate::ffi::Align4([0; 3]);

unsafe extern "C" {
    static mut gBackupMapLayout: u8;
    static mut gBattleOutcome: u8;
    static mut gBattleResults: u8;
    static mut gBattleTypeFlags: u8;
    static mut gContestMonPartyIndex: u8;
    static mut gContestMons: u8;
    static mut gDecorations: u8;
    static mut gGameLanguage: u8;
    static mut gLastUsedItem: u8;
    static mut gLinkPlayers: u8;
    static mut gLocalTime: u8;
    static mut gMapHeader: u8;
    static mut gMartPurchaseHistory: u8;
    static mut gMoveNames: u8;
    static mut gNumLinkContestPlayers: u8;
    static mut gPlayerParty: u8;
    static mut gSaveBlock1Ptr: u8;
    static mut gSaveBlock2Ptr: u8;
    static mut gSpecialVar_0x8004: u8;
    static mut gSpecialVar_0x8005: u8;
    static mut gSpecialVar_0x8006: u8;
    static mut gSpecialVar_0x8007: u8;
    static mut gSpecialVar_ContestCategory: u8;
    static mut gSpecialVar_ContestRank: u8;
    static mut gSpecialVar_LastTalked: u8;
    static mut gSpecialVar_MonBoxId: u8;
    static mut gSpecialVar_MonBoxPos: u8;
    static mut gSpecialVar_Result: u8;
    static mut gSpeciesNames: u8;
    static mut gStdStrings: u8;
    static mut gStringVar1: u8;
    static mut gStringVar2: u8;
    static mut gStringVar3: u8;
    static mut gStringVar4: u8;
    static mut gText_Bad: u8;
    static mut gText_Beauty: u8;
    static mut gText_Bitter2: u8;
    static mut gText_Cool: u8;
    static mut gText_Cute: u8;
    static mut gText_Dad: u8;
    static mut gText_Double: u8;
    static mut gText_Dry2: u8;
    static mut gText_Excellent: u8;
    static mut gText_First: u8;
    static mut gText_Good: u8;
    static mut gText_Jackpot: u8;
    static mut gText_Lv50: u8;
    static mut gText_Mom: u8;
    static mut gText_OpenLevel: u8;
    static mut gText_Roulette: u8;
    static mut gText_Second: u8;
    static mut gText_Single: u8;
    static mut gText_Slots: u8;
    static mut gText_Smart: u8;
    static mut gText_SoSo: u8;
    static mut gText_Sour2: u8;
    static mut gText_Spicy2: u8;
    static mut gText_Sweet2: u8;
    static mut gText_TheWorst: u8;
    static mut gText_Third: u8;
    static mut gText_Tough: u8;
    static mut gText_VeryGood: u8;
    fn Alloc(a0: u32) -> *mut u8;
    fn AllocZeroed(a0: u32) -> *mut u8;
    fn BufferContestLadyLanguage(a0: *mut u8);
    fn BufferContestLadyMonName(a0: *mut u8, a1: *mut u8);
    fn BufferContestLadyPlayerName(a0: *mut u8);
    fn BufferContestName(a0: *mut u8, a1: u8);
    fn CB2_ReturnToFieldContinueScriptPlayMapMusic();
    fn ConvertEasyChatWordsToString(a0: *mut u8, a1: *mut u16, a2: u16, a3: u16) -> *mut u8;
    fn ConvertIntToDecimalStringN(a0: *mut u8, a1: i32, a2: i32, a3: u8) -> *mut u8;
    fn ConvertInternationalString(a0: *mut u8, a1: u8);
    fn CopyCurSecretBaseOwnerName_StrVar1();
    fn CopyEasyChatWord(a0: *mut u8, a1: u16) -> *mut u8;
    fn DoNamingScreen(
        a0: u8,
        a1: *mut u8,
        a2: u16,
        a3: u16,
        a4: u32,
        a5: Option<unsafe extern "C" fn()>,
    );
    fn DrawWholeMapView();
    fn FlagClear(a0: u16) -> u8;
    fn FlagGet(a0: u16) -> u8;
    fn FlagSet(a0: u16) -> u8;
    fn Free(a0: *mut u8);
    fn GetBoxMonData3(a0: *mut u8, a1: i32, a2: *mut u8) -> u32;
    fn GetBoxMonGender(a0: *mut u8) -> u8;
    fn GetBoxedMonPtr(a0: u8, a1: u8) -> *mut u8;
    fn GetContestLadyPokeblockState() -> u8;
    fn GetCurrentBattleTowerWinStreak(a0: u8, a1: u8) -> u16;
    fn GetGameStat(a0: u8) -> u32;
    fn GetHoennPokedexCount(a0: u8) -> u16;
    fn GetItemName(a0: u16) -> *mut u8;
    fn GetItemPrice(a0: u16) -> u16;
    fn GetLeadMonIndex() -> u8;
    fn GetLinkPlayerCount() -> u8;
    fn GetLinkPlayerTrainerId(a0: u8) -> u32;
    fn GetMapName(a0: *mut u8, a1: u16, a2: u16) -> *mut u8;
    fn GetMonData2(a0: *mut u8, a1: i32) -> u32;
    fn GetMonData3(a0: *mut u8, a1: i32, a2: *mut u8) -> u32;
    fn GetMonGender(a0: *mut u8) -> u8;
    fn GetNationalPokedexCount(a0: u8) -> u16;
    fn GetNicknameLanguage(a0: *mut u8) -> i32;
    fn GetSetPokedexFlag(a0: u16, a1: u8) -> i8;
    fn IncrementGameStat(a0: u8);
    fn InitializeEasyChatWordArray(a0: *mut u16, a1: u16);
    fn IsNationalPokedexEnabled() -> u32;
    fn IsStringJapanese(a0: *mut u8) -> u32;
    fn MapGridGetMetatileBehaviorAt(a0: i32, a1: i32) -> i32;
    fn MapGridSetMetatileIdAt(a0: i32, a1: i32, a2: u16);
    fn Random() -> u16;
    fn RemoveObjectEventByLocalIdAndMap(a0: u8, a1: u8, a2: u8);
    fn SetBoxMonNickAt(a0: u8, a1: u8, a2: *mut u8);
    fn SetMonData(a0: *mut u8, a1: i32, a2: *mut u8);
    fn ShowFieldMessage(a0: *mut u8) -> u8;
    fn SpeciesToNationalPokedexNum(a0: u16) -> u16;
    fn StringCompare(a0: *mut u8, a1: *mut u8) -> i32;
    fn StringCopy(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn StringGet_Nickname(a0: *mut u8) -> *mut u8;
    fn StringLength(a0: *mut u8) -> u16;
    fn StripExtCtrlCodes(a0: *mut u8);
    fn TVShowConvertInternationalString(a0: *mut u8, a1: *mut u8, a2: i32);
    fn VarGet(a0: u16) -> u16;
    fn VarSet(a0: u16, a1: u16) -> u8;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ClearTVShowData() {
    unsafe {
        let mut i: u8 = 0u8;
        let mut j: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as u32) < crate::c::div_u32(900u32, 36u32)) {
                    break 'l1;
                }
                'l2: {
                    ((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(10188))
                    .cast::<u8>())
                    .wrapping_offset(((i) as i32) as isize * 36))
                    .write(0u8);
                    (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(10188))
                    .cast::<u8>())
                    .wrapping_offset(((i) as i32) as isize * 36))
                    .wrapping_add(1))
                    .write(0u8);
                    {
                        j = 0u8;
                        'l3: loop {
                            if !(((j) as u32) < crate::c::div_u32(34u32, 1u32)) {
                                break 'l3;
                            }
                            'l4: {
                                (((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                                    .wrapping_add(10188))
                                .cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize * 36))
                                .wrapping_add(2))
                                .cast::<u8>())
                                .wrapping_offset(((j) as i32) as isize))
                                .write(0u8);
                            }
                            j = (j).wrapping_add(1);
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        ClearPokeNews();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetRandomActiveShowIdx() -> u8 {
    unsafe {
        let mut i: u8 = 0u8;
        let mut j: u8 = 0u8;
        let mut selIdx: u8 = 0u8;
        let mut show: *mut u8 = core::ptr::null_mut();
        {
            i = 5u8;
            'l1: loop {
                if !(((i) as i32) < 24i32) {
                    break 'l1;
                }
                'l2: {
                    if ((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(10188))
                    .cast::<u8>())
                    .wrapping_offset(((i) as i32) as isize * 36))
                    .read()) as i32)
                        == 0i32
                    {
                        break 'l1;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        j = ((crate::c::rem_i32(((Random()) as i32), ((i) as i32))) as u8);
        selIdx = j;
        'l3: loop {
            'l4: {
                if ((GetTVGroupByShowId(
                    ((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(10188))
                    .cast::<u8>())
                    .wrapping_offset(((j) as i32) as isize * 36))
                    .read(),
                )) as i32)
                    != 4i32
                {
                    if (((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(10188))
                    .cast::<u8>())
                    .wrapping_offset(((j) as i32) as isize * 36))
                    .wrapping_add(1))
                    .read()) as i32)
                        == 1i32
                    {
                        return j;
                    }
                } else {
                    show = (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(10188))
                    .cast::<u8>())
                    .wrapping_offset(((j) as i32) as isize * 36);
                    if (((((show).wrapping_add(22).cast::<u16>()).read()) as i32) == 0i32)
                        && (((((show).wrapping_add(1)).read()) as i32) == 1i32)
                    {
                        return j;
                    }
                }
                if ((j) as i32) == 0i32 {
                    j = (((crate::c::div_u32(900u32, 36u32)).wrapping_sub(2u32)) as u8);
                } else {
                    j = (j).wrapping_sub(1);
                }
            }
            if !(((j) as i32) != ((selIdx) as i32)) {
                break 'l3;
            }
        }
        return 255u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FindAnyTVShowOnTheAir() -> u8 {
    unsafe {
        let mut slot: u8 = GetRandomActiveShowIdx();
        if ((slot) as i32) == 255i32 {
            return 255u8;
        }
        if (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
            .wrapping_add(11152)
            .cast::<u16>())
        .read()) as i32)
            != 0i32)
            && (((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(10188))
                .cast::<u8>())
            .wrapping_offset(((slot) as i32) as isize * 36))
            .read()) as i32)
                == 41i32)
        {
            return FindFirstActiveTVShowThatIsNotAMassOutbreak();
        }
        return slot;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn UpdateTVScreensOnMap(width: i32, height: i32) {
    unsafe {
        let mut width = width;
        let mut height = height;
        FlagSet(2193u16);
        'l1: {
            let __sw1 = ((CheckForPlayersHouseNews()) as i32);
            let __matched = __sw1 == 1i32 || __sw1 == 2i32;
            if __sw1 == 1i32 {
                SetTVMetatilesOnMap(width, height, 3u16);
                break 'l1;
            }
            if __sw1 == 2i32 {
                break 'l1;
            }
            if !__matched {
                if ((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4))
                    .cast::<i8>())
                .read()) as i32)
                    == 13i32)
                    && ((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(4))
                    .wrapping_add(1)
                    .cast::<i8>())
                    .read()) as i32)
                        == 0i32)
                {
                    SetTVMetatilesOnMap(width, height, 3u16);
                } else {
                    if ((FlagGet(2194u16)) != 0)
                        && (((((FindAnyTVShowOnTheAir()) as i32) != 255i32)
                            || (((FindAnyPokeNewsOnTheAir()) as i32) != 255i32))
                            || ((IsGabbyAndTyShowOnTheAir()) != 0))
                    {
                        FlagClear(2193u16);
                        SetTVMetatilesOnMap(width, height, 3u16);
                    }
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SetTVMetatilesOnMap(width: i32, height: i32, metatileId: u16) {
    unsafe {
        let mut width = width;
        let mut height = height;
        let mut metatileId = metatileId;
        let mut x: i32 = 0i32;
        let mut y: i32 = 0i32;
        {
            y = 0i32;
            'l1: loop {
                if !(y < height) {
                    break 'l1;
                }
                'l2: {
                    {
                        x = 0i32;
                        'l3: loop {
                            if !(x < width) {
                                break 'l3;
                            }
                            'l4: {
                                if MapGridGetMetatileBehaviorAt(x, y) == 134i32 {
                                    MapGridSetMetatileIdAt(
                                        x,
                                        y,
                                        ((((metatileId) as i32) | 3072i32) as u16),
                                    );
                                }
                            }
                            x = (x).wrapping_add(1);
                        }
                    }
                }
                y = (y).wrapping_add(1);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn TurnOffTVScreen() {
    unsafe {
        SetTVMetatilesOnMap(
            (((&raw mut gBackupMapLayout).cast::<u8>()).cast::<i32>()).read(),
            (((&raw mut gBackupMapLayout).cast::<u8>())
                .wrapping_add(4)
                .cast::<i32>())
            .read(),
            2u16,
        );
        DrawWholeMapView();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn TurnOnTVScreen() {
    unsafe {
        SetTVMetatilesOnMap(
            (((&raw mut gBackupMapLayout).cast::<u8>()).cast::<i32>()).read(),
            (((&raw mut gBackupMapLayout).cast::<u8>())
                .wrapping_add(4)
                .cast::<i32>())
            .read(),
            3u16,
        );
        DrawWholeMapView();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetSelectedTVShow() -> u8 {
    unsafe {
        return ((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(10188))
            .cast::<u8>())
        .wrapping_offset(
            ((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32) as isize * 36,
        ))
        .read();
    }
}
pub(crate) unsafe extern "C" fn FindFirstActiveTVShowThatIsNotAMassOutbreak() -> u8 {
    unsafe {
        let mut i: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as u32) < (crate::c::div_u32(900u32, 36u32)).wrapping_sub(1u32)) {
                    break 'l1;
                }
                'l2: {
                    if ((((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(10188))
                    .cast::<u8>())
                    .wrapping_offset(((i) as i32) as isize * 36))
                    .read()) as i32)
                        != 0i32)
                        && (((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(10188))
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 36))
                        .read()) as i32)
                            != 41i32))
                        && ((((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(10188))
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 36))
                        .wrapping_add(1))
                        .read()) as i32)
                            == 1i32)
                    {
                        return i;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return 255u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetNextActiveShowIfMassOutbreak() -> u8 {
    unsafe {
        let mut tvShow: *mut u8 = core::ptr::null_mut();
        tvShow = (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(10188))
            .cast::<u8>())
        .wrapping_offset(
            ((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32) as isize * 36,
        );
        if ((((tvShow).read()) as i32) == 41i32)
            && (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                .wrapping_add(11152)
                .cast::<u16>())
            .read()) as i32)
                != 0i32)
        {
            return FindFirstActiveTVShowThatIsNotAMassOutbreak();
        }
        return ((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ResetGabbyAndTy() {
    unsafe {
        (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(11172))
            .cast::<u16>())
        .write(0u16);
        (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(11172))
            .wrapping_add(2)
            .cast::<u16>())
        .write(0u16);
        (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(11172))
            .wrapping_add(4)
            .cast::<u16>())
        .write(0u16);
        ((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(11172))
            .wrapping_add(6))
        .cast::<u16>())
        .write(65535u16);
        crate::c::bf_write(
            ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(11172))
                .wrapping_add(10),
            0,
            1,
            (0u8) as i32,
        );
        crate::c::bf_write(
            ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(11172))
                .wrapping_add(10),
            1,
            1,
            (0u8) as i32,
        );
        crate::c::bf_write(
            ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(11172))
                .wrapping_add(10),
            2,
            1,
            (0u8) as i32,
        );
        crate::c::bf_write(
            ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(11172))
                .wrapping_add(10),
            3,
            1,
            (0u8) as i32,
        );
        crate::c::bf_write(
            ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(11172))
                .wrapping_add(10),
            4,
            1,
            (0u8) as i32,
        );
        crate::c::bf_write(
            ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(11172))
                .wrapping_add(10),
            5,
            3,
            (0u8) as i32,
        );
        crate::c::bf_write(
            ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(11172))
                .wrapping_add(11),
            0,
            1,
            (0u8) as i32,
        );
        crate::c::bf_write(
            ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(11172))
                .wrapping_add(11),
            1,
            1,
            (0u8) as i32,
        );
        crate::c::bf_write(
            ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(11172))
                .wrapping_add(11),
            2,
            1,
            (0u8) as i32,
        );
        crate::c::bf_write(
            ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(11172))
                .wrapping_add(11),
            3,
            1,
            (0u8) as i32,
        );
        crate::c::bf_write(
            ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(11172))
                .wrapping_add(11),
            4,
            4,
            (0u8) as i32,
        );
        (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(11172))
            .wrapping_add(8))
        .write(0u8);
        (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(11172))
            .wrapping_add(9))
        .write(0u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GabbyAndTyBeforeInterview() {
    unsafe {
        let mut i: u8 = 0u8;
        (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(11172))
            .cast::<u16>())
        .write(
            (((&raw mut gBattleResults).cast::<u8>())
                .wrapping_add(6)
                .cast::<u16>())
            .read(),
        );
        (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(11172))
            .wrapping_add(2)
            .cast::<u16>())
        .write(
            (((&raw mut gBattleResults).cast::<u8>())
                .wrapping_add(38)
                .cast::<u16>())
            .read(),
        );
        (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(11172))
            .wrapping_add(4)
            .cast::<u16>())
        .write(
            (((&raw mut gBattleResults).cast::<u8>())
                .wrapping_add(34)
                .cast::<u16>())
            .read(),
        );
        if (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(11172))
            .wrapping_add(9))
        .read()) as i32)
            != 255i32
        {
            let __p1 = ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(11172))
                .wrapping_add(9);
            (__p1).write(((__p1).read()).wrapping_add(1));
        }
        crate::c::bf_write(
            ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(11172))
                .wrapping_add(10),
            0,
            1,
            (crate::c::bf_read(
                ((&raw mut gBattleResults).cast::<u8>()).wrapping_add(5),
                0,
                1,
                false,
            ) as u8) as i32,
        );
        if ((((&raw mut gBattleResults).cast::<u8>()).read()) as i32) != 0i32 {
            crate::c::bf_write(
                ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(11172))
                    .wrapping_add(10),
                1,
                1,
                (1u8) as i32,
            );
        } else {
            crate::c::bf_write(
                ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(11172))
                    .wrapping_add(10),
                1,
                1,
                (0u8) as i32,
            );
        }
        if (((((&raw mut gBattleResults).cast::<u8>()).wrapping_add(3)).read()) as i32) != 0i32 {
            crate::c::bf_write(
                ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(11172))
                    .wrapping_add(10),
                2,
                1,
                (1u8) as i32,
            );
        } else {
            crate::c::bf_write(
                ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(11172))
                    .wrapping_add(10),
                2,
                1,
                (0u8) as i32,
            );
        }
        if !((crate::c::bf_read(
            ((&raw mut gBattleResults).cast::<u8>()).wrapping_add(5),
            1,
            1,
            false,
        ) as u8)
            != 0)
        {
            {
                i = 0u8;
                'l1: loop {
                    if !(((i) as i32) < 11i32) {
                        break 'l1;
                    }
                    'l2: {
                        if ((((((&raw mut gBattleResults).cast::<u8>()).wrapping_add(54))
                            .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize))
                        .read())
                            != 0
                        {
                            crate::c::bf_write(
                                ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                                    .wrapping_add(11172))
                                .wrapping_add(10),
                                3,
                                1,
                                (1u8) as i32,
                            );
                            break 'l1;
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
        } else {
            crate::c::bf_write(
                ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(11172))
                    .wrapping_add(10),
                3,
                1,
                (1u8) as i32,
            );
        }
        TakeGabbyAndTyOffTheAir();
        if (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(11172))
            .wrapping_add(4)
            .cast::<u16>())
        .read()) as i32)
            == 0i32
        {
            FlagSet(1u16);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GabbyAndTyAfterInterview() {
    unsafe {
        crate::c::bf_write(
            ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(11172))
                .wrapping_add(11),
            0,
            1,
            (crate::c::bf_read(
                ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(11172))
                    .wrapping_add(10),
                0,
                1,
                false,
            ) as u8) as i32,
        );
        crate::c::bf_write(
            ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(11172))
                .wrapping_add(11),
            1,
            1,
            (crate::c::bf_read(
                ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(11172))
                    .wrapping_add(10),
                1,
                1,
                false,
            ) as u8) as i32,
        );
        crate::c::bf_write(
            ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(11172))
                .wrapping_add(11),
            2,
            1,
            (crate::c::bf_read(
                ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(11172))
                    .wrapping_add(10),
                2,
                1,
                false,
            ) as u8) as i32,
        );
        crate::c::bf_write(
            ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(11172))
                .wrapping_add(11),
            3,
            1,
            (crate::c::bf_read(
                ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(11172))
                    .wrapping_add(10),
                3,
                1,
                false,
            ) as u8) as i32,
        );
        crate::c::bf_write(
            ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(11172))
                .wrapping_add(10),
            4,
            1,
            (1u8) as i32,
        );
        (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(11172))
            .wrapping_add(8))
        .write((((&raw mut gMapHeader).cast::<u8>()).wrapping_add(20)).read());
        IncrementGameStat(6u8);
    }
}
pub(crate) unsafe extern "C" fn TakeGabbyAndTyOffTheAir() {
    unsafe {
        crate::c::bf_write(
            ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(11172))
                .wrapping_add(10),
            4,
            1,
            (0u8) as i32,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GabbyAndTyGetBattleNum() -> u8 {
    unsafe {
        if (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(11172))
            .wrapping_add(9))
        .read()) as i32)
            > 5i32
        {
            return (((crate::c::rem_i32(
                (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(11172))
                    .wrapping_add(9))
                .read()) as i32),
                3i32,
            ))
            .wrapping_add(6i32)) as u8);
        }
        return (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(11172))
            .wrapping_add(9))
        .read();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsGabbyAndTyShowOnTheAir() -> u8 {
    unsafe {
        return (crate::c::bf_read(
            ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(11172))
                .wrapping_add(10),
            4,
            1,
            false,
        ) as u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GabbyAndTyGetLastQuote() -> u8 {
    unsafe {
        if ((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(11172))
            .wrapping_add(6))
        .cast::<u16>())
        .read()) as i32)
            == 65535i32
        {
            return 0u8;
        }
        CopyEasyChatWord(
            (&raw mut gStringVar1).cast::<u8>(),
            ((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(11172))
                .wrapping_add(6))
            .cast::<u16>())
            .read(),
        );
        ((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(11172))
            .wrapping_add(6))
        .cast::<u16>())
        .write(65535u16);
        return 1u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GabbyAndTyGetLastBattleTrivia() -> u8 {
    unsafe {
        if !((crate::c::bf_read(
            ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(11172))
                .wrapping_add(11),
            0,
            1,
            false,
        ) as u8)
            != 0)
        {
            return 1u8;
        }
        if (crate::c::bf_read(
            ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(11172))
                .wrapping_add(11),
            3,
            1,
            false,
        ) as u8)
            != 0
        {
            return 2u8;
        }
        if (crate::c::bf_read(
            ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(11172))
                .wrapping_add(11),
            2,
            1,
            false,
        ) as u8)
            != 0
        {
            return 3u8;
        }
        if (crate::c::bf_read(
            ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(11172))
                .wrapping_add(11),
            1,
            1,
            false,
        ) as u8)
            != 0
        {
            return 4u8;
        }
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetGabbyAndTyLocalIds() {
    unsafe {
        'l1: {
            let __sw1 = ((GabbyAndTyGetBattleNum()) as i32);
            if __sw1 == 1i32 {
                ((&raw mut gSpecialVar_0x8004).cast::<u16>()).write(14u16);
                ((&raw mut gSpecialVar_0x8005).cast::<u16>()).write(13u16);
                break 'l1;
            }
            if __sw1 == 2i32 {
                ((&raw mut gSpecialVar_0x8004).cast::<u16>()).write(5u16);
                ((&raw mut gSpecialVar_0x8005).cast::<u16>()).write(6u16);
                break 'l1;
            }
            if __sw1 == 3i32 {
                ((&raw mut gSpecialVar_0x8004).cast::<u16>()).write(18u16);
                ((&raw mut gSpecialVar_0x8005).cast::<u16>()).write(17u16);
                break 'l1;
            }
            if __sw1 == 4i32 {
                ((&raw mut gSpecialVar_0x8004).cast::<u16>()).write(21u16);
                ((&raw mut gSpecialVar_0x8005).cast::<u16>()).write(22u16);
                break 'l1;
            }
            if __sw1 == 5i32 {
                ((&raw mut gSpecialVar_0x8004).cast::<u16>()).write(8u16);
                ((&raw mut gSpecialVar_0x8005).cast::<u16>()).write(9u16);
                break 'l1;
            }
            if __sw1 == 6i32 {
                ((&raw mut gSpecialVar_0x8004).cast::<u16>()).write(19u16);
                ((&raw mut gSpecialVar_0x8005).cast::<u16>()).write(20u16);
                break 'l1;
            }
            if __sw1 == 7i32 {
                ((&raw mut gSpecialVar_0x8004).cast::<u16>()).write(23u16);
                ((&raw mut gSpecialVar_0x8005).cast::<u16>()).write(24u16);
                break 'l1;
            }
            if __sw1 == 8i32 {
                ((&raw mut gSpecialVar_0x8004).cast::<u16>()).write(10u16);
                ((&raw mut gSpecialVar_0x8005).cast::<u16>()).write(11u16);
                break 'l1;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn InterviewAfter() {
    unsafe {
        'l1: {
            let __sw1 = ((((&raw mut gSpecialVar_0x8005).cast::<u16>()).read()) as i32);
            if __sw1 == 1i32 {
                InterviewAfter_FanClubLetter();
                break 'l1;
            }
            if __sw1 == 2i32 {
                InterviewAfter_RecentHappenings();
                break 'l1;
            }
            if __sw1 == 3i32 {
                InterviewAfter_PkmnFanClubOpinions();
                break 'l1;
            }
            if __sw1 == 4i32 {
                InterviewAfter_Dummy();
                break 'l1;
            }
            if __sw1 == 6i32 {
                InterviewAfter_BravoTrainerPokemonProfile();
                break 'l1;
            }
            if __sw1 == 7i32 {
                InterviewAfter_BravoTrainerBattleTowerProfile();
                break 'l1;
            }
            if __sw1 == 8i32 {
                InterviewAfter_ContestLiveUpdates();
                break 'l1;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn TryPutPokemonTodayOnAir() {
    unsafe {
        let mut i: u8 = 0u8;
        let mut ballsUsed: u16 = 0u16;
        let mut show: *mut u8 = core::ptr::null_mut();
        let mut language2: u32 = 0u32;
        let mut itemLastUsed: u16 = 0u16;
        ballsUsed = 0u16;
        TryPutRandomPokeNewsOnAir();
        TryStartRandomMassOutbreak();
        if (((((&raw mut gBattleResults).cast::<u8>())
            .wrapping_add(40)
            .cast::<u16>())
        .read()) as i32)
            == 0i32
        {
            TryPutPokemonTodayFailedOnTheAir();
        } else {
            InitWorldOfMastersShowAttempt();
            if (!((BernoulliTrial(((crate::c::div_i32(65535i32, 1i32)) as u16))) != 0))
                && ((StringCompare(
                    (((&raw mut gSpeciesNames).cast::<u8>()).wrapping_offset(
                        (((((&raw mut gBattleResults).cast::<u8>())
                            .wrapping_add(40)
                            .cast::<u16>())
                        .read()) as i32) as isize
                            * 11,
                    ))
                    .cast::<u8>(),
                    (((&raw mut gBattleResults).cast::<u8>()).wrapping_add(42)).cast::<u8>(),
                )) != 0)
            {
                ((&raw mut sCurTVShowSlot).cast::<u8>().cast::<i8>()).write(
                    FindFirstEmptyRecordMixTVShowSlot(
                        ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(10188))
                        .cast::<u8>(),
                    ),
                );
                if (((((&raw mut sCurTVShowSlot).cast::<u8>().cast::<i8>()).read()) as i32)
                    != (-1i32))
                    && (((IsRecordMixShowAlreadySpawned(21u8, 0u8)) as i32) != 1i32)
                {
                    {
                        i = 0u8;
                        'l1: loop {
                            if !(((i) as i32) < 11i32) {
                                break 'l1;
                            }
                            'l2: {
                                ballsUsed = ((((ballsUsed) as i32).wrapping_add(
                                    (((((((&raw mut gBattleResults).cast::<u8>())
                                        .wrapping_add(54))
                                    .cast::<u8>())
                                    .wrapping_offset(((i) as i32) as isize))
                                    .read()) as i32),
                                )) as u16);
                            }
                            i = (i).wrapping_add(1);
                        }
                    }
                    if (((ballsUsed) as i32) != 0i32)
                        || ((crate::c::bf_read(
                            ((&raw mut gBattleResults).cast::<u8>()).wrapping_add(5),
                            1,
                            1,
                            false,
                        ) as u8)
                            != 0)
                    {
                        ballsUsed = 0u16;
                        show = (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(10188))
                        .cast::<u8>())
                        .wrapping_offset(
                            ((((&raw mut sCurTVShowSlot).cast::<u8>().cast::<i8>()).read()) as i32)
                                as isize
                                * 36,
                        );
                        (show).write(21u8);
                        ((show).wrapping_add(1)).write(0u8);
                        if (crate::c::bf_read(
                            ((&raw mut gBattleResults).cast::<u8>()).wrapping_add(5),
                            1,
                            1,
                            false,
                        ) as u8)
                            != 0
                        {
                            ballsUsed = 1u16;
                            itemLastUsed = 1u16;
                        } else {
                            {
                                i = 0u8;
                                'l3: loop {
                                    if !(((i) as i32) < 11i32) {
                                        break 'l3;
                                    }
                                    'l4: {
                                        ballsUsed = ((((ballsUsed) as i32).wrapping_add(
                                            (((((((&raw mut gBattleResults).cast::<u8>())
                                                .wrapping_add(54))
                                            .cast::<u8>())
                                            .wrapping_offset(((i) as i32) as isize))
                                            .read())
                                                as i32),
                                        ))
                                            as u16);
                                    }
                                    i = (i).wrapping_add(1);
                                }
                            }
                            if ((ballsUsed) as i32) > 255i32 {
                                ballsUsed = 255u16;
                            }
                            itemLastUsed = ((&raw mut gLastUsedItem).cast::<u16>()).read();
                        }
                        ((show).wrapping_add(18)).write(((ballsUsed) as u8));
                        ((show).wrapping_add(15)).write(((itemLastUsed) as u8));
                        StringCopy(
                            ((show).wrapping_add(19)).cast::<u8>(),
                            (((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).cast::<u8>(),
                        );
                        StringCopy(
                            ((show).wrapping_add(4)).cast::<u8>(),
                            (((&raw mut gBattleResults).cast::<u8>()).wrapping_add(42))
                                .cast::<u8>(),
                        );
                        language2 =
                            ((GetNicknameLanguage(((show).wrapping_add(4)).cast::<u8>())) as u32);
                        StripExtCtrlCodes(((show).wrapping_add(4)).cast::<u8>());
                        ((show).wrapping_add(16).cast::<u16>()).write(
                            (((&raw mut gBattleResults).cast::<u8>())
                                .wrapping_add(40)
                                .cast::<u16>())
                            .read(),
                        );
                        StorePlayerIdInRecordMixShow(show);
                        ((show).wrapping_add(2))
                            .write(((&raw mut gGameLanguage).cast::<u8>()).read());
                        ((show).wrapping_add(3)).write(((language2) as u8));
                    }
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn InitWorldOfMastersShowAttempt() {
    unsafe {
        let mut show: *mut u8 = (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
            .wrapping_add(10188))
        .cast::<u8>())
        .wrapping_offset(864);
        if (((show).read()) as i32) != 25i32 {
            DeleteTVShowInArrayByIdx(
                ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(10188))
                    .cast::<u8>(),
                24u8,
            );
            ((show).wrapping_add(6).cast::<u16>()).write(((GetGameStat(5u8)) as u16));
            (show).write(25u8);
        }
        let __p1 = (show).wrapping_add(2).cast::<u16>();
        (__p1).write(((__p1).read()).wrapping_add(1));
        ((show).wrapping_add(4).cast::<u16>()).write(
            (((&raw mut gBattleResults).cast::<u8>())
                .wrapping_add(40)
                .cast::<u16>())
            .read(),
        );
        ((show).wrapping_add(8).cast::<u16>()).write(
            (((&raw mut gBattleResults).cast::<u8>())
                .wrapping_add(6)
                .cast::<u16>())
            .read(),
        );
        ((show).wrapping_add(10))
            .write((((&raw mut gMapHeader).cast::<u8>()).wrapping_add(20)).read());
    }
}
pub(crate) unsafe extern "C" fn TryPutPokemonTodayFailedOnTheAir() {
    unsafe {
        let mut ballsUsed: u16 = 0u16;
        let mut i: u8 = 0u8;
        let mut show: *mut u8 = core::ptr::null_mut();
        if !((BernoulliTrial(((crate::c::div_i32(65535i32, 1i32)) as u16))) != 0) {
            {
                i = 0u8;
                ballsUsed = 0u16;
                'l1: loop {
                    if !(((i) as i32) < 11i32) {
                        break 'l1;
                    }
                    'l2: {
                        ballsUsed = ((((ballsUsed) as i32).wrapping_add(
                            (((((((&raw mut gBattleResults).cast::<u8>()).wrapping_add(54))
                                .cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize))
                            .read()) as i32),
                        )) as u16);
                    }
                    i = (i).wrapping_add(1);
                }
            }
            if ((ballsUsed) as i32) > 255i32 {
                ballsUsed = 255u16;
            }
            if (((ballsUsed) as i32) > 2i32)
                && ((((((&raw mut gBattleOutcome).cast::<u8>()).read()) as i32) == 6i32)
                    || (((((&raw mut gBattleOutcome).cast::<u8>()).read()) as i32) == 1i32))
            {
                ((&raw mut sCurTVShowSlot).cast::<u8>().cast::<i8>()).write(
                    FindFirstEmptyRecordMixTVShowSlot(
                        ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(10188))
                        .cast::<u8>(),
                    ),
                );
                if (((((&raw mut sCurTVShowSlot).cast::<u8>().cast::<i8>()).read()) as i32)
                    != (-1i32))
                    && (((IsRecordMixShowAlreadySpawned(23u8, 0u8)) as i32) != 1i32)
                {
                    show = (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(10188))
                    .cast::<u8>())
                    .wrapping_offset(
                        ((((&raw mut sCurTVShowSlot).cast::<u8>().cast::<i8>()).read()) as i32)
                            as isize
                            * 36,
                    );
                    (show).write(23u8);
                    ((show).wrapping_add(1)).write(0u8);
                    ((show).wrapping_add(12).cast::<u16>()).write(
                        (((&raw mut gBattleResults).cast::<u8>())
                            .wrapping_add(6)
                            .cast::<u16>())
                        .read(),
                    );
                    ((show).wrapping_add(14).cast::<u16>()).write(
                        (((&raw mut gBattleResults).cast::<u8>())
                            .wrapping_add(32)
                            .cast::<u16>())
                        .read(),
                    );
                    ((show).wrapping_add(16)).write(((ballsUsed) as u8));
                    ((show).wrapping_add(17))
                        .write(((&raw mut gBattleOutcome).cast::<u8>()).read());
                    ((show).wrapping_add(18))
                        .write((((&raw mut gMapHeader).cast::<u8>()).wrapping_add(20)).read());
                    StringCopy(
                        ((show).wrapping_add(19)).cast::<u8>(),
                        (((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).cast::<u8>(),
                    );
                    StorePlayerIdInRecordMixShow(show);
                    ((show).wrapping_add(2)).write(((&raw mut gGameLanguage).cast::<u8>()).read());
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn StorePlayerIdInRecordMixShow(show: *mut u8) {
    unsafe {
        let mut show = show;
        let mut id: u32 = GetPlayerIDAsU32();
        ((show).wrapping_add(30)).write(((id) as u8));
        ((show).wrapping_add(31)).write(((id >> 8) as u8));
        ((show).wrapping_add(32)).write(((id) as u8));
        ((show).wrapping_add(33)).write(((id >> 8) as u8));
        ((show).wrapping_add(34)).write(((id) as u8));
        ((show).wrapping_add(35)).write(((id >> 8) as u8));
    }
}
pub(crate) unsafe extern "C" fn StorePlayerIdInNormalShow(show: *mut u8) {
    unsafe {
        let mut show = show;
        let mut id: u32 = GetPlayerIDAsU32();
        ((show).wrapping_add(32)).write(((id) as u8));
        ((show).wrapping_add(33)).write(((id >> 8) as u8));
        ((show).wrapping_add(34)).write(((id) as u8));
        ((show).wrapping_add(35)).write(((id >> 8) as u8));
    }
}
pub(crate) unsafe extern "C" fn InterviewAfter_ContestLiveUpdates() {
    unsafe {
        let mut show: *mut u8 = core::ptr::null_mut();
        let mut show2: *mut u8 = core::ptr::null_mut();
        show = (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(10188))
            .cast::<u8>())
        .wrapping_offset(864);
        if (((show).read()) as i32) == 8i32 {
            show2 = (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(10188))
                .cast::<u8>())
            .wrapping_offset(
                ((((&raw mut sCurTVShowSlot).cast::<u8>().cast::<i8>()).read()) as i32) as isize
                    * 36,
            );
            (show2).write(8u8);
            ((show2).wrapping_add(1)).write(1u8);
            StringCopy(
                ((show2).wrapping_add(20)).cast::<u8>(),
                (((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).cast::<u8>(),
            );
            ((show2).wrapping_add(28))
                .write(((((&raw mut gSpecialVar_ContestCategory).cast::<u16>()).read()) as u8));
            ((show2).wrapping_add(18).cast::<u16>()).write(
                ((GetMonData3(
                    ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                        ((((&raw mut gContestMonPartyIndex).cast::<u8>()).read()) as i32) as isize
                            * 100,
                    ),
                    11i32,
                    core::ptr::null_mut(),
                )) as u16),
            );
            ((show2).wrapping_add(2).cast::<u16>())
                .write(((show).wrapping_add(2).cast::<u16>()).read());
            ((show2).wrapping_add(12)).write(((show).wrapping_add(12)).read());
            ((show2).wrapping_add(13)).write(((show).wrapping_add(13)).read());
            ((show2).wrapping_add(14)).write(((show).wrapping_add(14)).read());
            ((show2).wrapping_add(16).cast::<u16>())
                .write(((show).wrapping_add(16).cast::<u16>()).read());
            ((show2).wrapping_add(15)).write(((show).wrapping_add(15)).read());
            StringCopy(
                ((show2).wrapping_add(4)).cast::<u8>(),
                ((show).wrapping_add(4)).cast::<u8>(),
            );
            StorePlayerIdInNormalShow(show2);
            ((show2).wrapping_add(29)).write(((&raw mut gGameLanguage).cast::<u8>()).read());
            ((show2).wrapping_add(30)).write(((show).wrapping_add(30)).read());
            DeleteTVShowInArrayByIdx(
                ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(10188))
                    .cast::<u8>(),
                24u8,
            );
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PutBattleUpdateOnTheAir(
    opponentLinkPlayerId: u8,
    r#move: u16,
    speciesPlayer: u16,
    speciesOpponent: u16,
) {
    unsafe {
        let mut opponentLinkPlayerId = opponentLinkPlayerId;
        let mut r#move = r#move;
        let mut speciesPlayer = speciesPlayer;
        let mut speciesOpponent = speciesOpponent;
        let mut show: *mut u8 = core::ptr::null_mut();
        let mut name = crate::ffi::Align4([0u8; 32]);
        ((&raw mut sCurTVShowSlot).cast::<u8>().cast::<i8>()).write(
            FindFirstEmptyNormalTVShowSlot(
                ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(10188))
                    .cast::<u8>(),
            ),
        );
        if ((((&raw mut sCurTVShowSlot).cast::<u8>().cast::<i8>()).read()) as i32) != (-1i32) {
            TryReplaceOldTVShowOfKind(10u8);
            if ((((&raw mut gSpecialVar_Result).cast::<u16>()).read()) as i32) != 1i32 {
                show = (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                    .wrapping_add(10188))
                .cast::<u8>())
                .wrapping_offset(
                    ((((&raw mut sCurTVShowSlot).cast::<u8>().cast::<i8>()).read()) as i32)
                        as isize
                        * 36,
                );
                (show).write(10u8);
                ((show).wrapping_add(1)).write(1u8);
                StringCopy(
                    ((show).wrapping_add(4)).cast::<u8>(),
                    (((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).cast::<u8>(),
                );
                if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 64u32) != 0 {
                    ((show).wrapping_add(24)).write(2u8);
                } else {
                    if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 1u32) != 0 {
                        ((show).wrapping_add(24)).write(1u8);
                    } else {
                        ((show).wrapping_add(24)).write(0u8);
                    }
                }
                ((show).wrapping_add(20).cast::<u16>()).write(r#move);
                ((show).wrapping_add(22).cast::<u16>()).write(speciesPlayer);
                ((show).wrapping_add(2).cast::<u16>()).write(speciesOpponent);
                StringCopy(
                    (&raw mut name).cast::<u8>(),
                    ((((&raw mut gLinkPlayers).cast::<u8>())
                        .wrapping_offset(((opponentLinkPlayerId) as i32) as isize * 28))
                    .wrapping_add(8))
                    .cast::<u8>(),
                );
                StripExtCtrlCodes((&raw mut name).cast::<u8>());
                StringCopy(
                    ((show).wrapping_add(12)).cast::<u8>(),
                    (&raw mut name).cast::<u8>(),
                );
                StorePlayerIdInNormalShow(show);
                ((show).wrapping_add(25)).write(((&raw mut gGameLanguage).cast::<u8>()).read());
                if (((((show).wrapping_add(25)).read()) as i32) == 1i32)
                    || (((((((&raw mut gLinkPlayers).cast::<u8>())
                        .wrapping_offset(((opponentLinkPlayerId) as i32) as isize * 28))
                    .wrapping_add(26)
                    .cast::<u16>())
                    .read()) as i32)
                        == 1i32)
                {
                    ((show).wrapping_add(26)).write(1u8);
                } else {
                    ((show).wrapping_add(26)).write(
                        ((((((&raw mut gLinkPlayers).cast::<u8>())
                            .wrapping_offset(((opponentLinkPlayerId) as i32) as isize * 28))
                        .wrapping_add(26)
                        .cast::<u16>())
                        .read()) as u8),
                    );
                }
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Put3CheersForPokeblocksOnTheAir(
    partnersName: *mut u8,
    flavor: u8,
    color: u8,
    sheen: u8,
    language: u8,
) -> u8 {
    unsafe {
        let mut partnersName = partnersName;
        let mut flavor = flavor;
        let mut color = color;
        let mut sheen = sheen;
        let mut language = language;
        let mut show: *mut u8 = core::ptr::null_mut();
        let mut name = crate::ffi::Align4([0u8; 32]);
        ((&raw mut sCurTVShowSlot).cast::<u8>().cast::<i8>()).write(
            FindFirstEmptyNormalTVShowSlot(
                ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(10188))
                    .cast::<u8>(),
            ),
        );
        if ((((&raw mut sCurTVShowSlot).cast::<u8>().cast::<i8>()).read()) as i32) == (-1i32) {
            return 0u8;
        }
        TryReplaceOldTVShowOfKind(9u8);
        if ((((&raw mut gSpecialVar_Result).cast::<u16>()).read()) as i32) == 1i32 {
            return 0u8;
        }
        show = (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(10188))
            .cast::<u8>())
        .wrapping_offset(
            ((((&raw mut sCurTVShowSlot).cast::<u8>().cast::<i8>()).read()) as i32) as isize * 36,
        );
        (show).write(9u8);
        ((show).wrapping_add(1)).write(1u8);
        StringCopy(
            ((show).wrapping_add(12)).cast::<u8>(),
            (((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).cast::<u8>(),
        );
        StringCopy((&raw mut name).cast::<u8>(), partnersName);
        StripExtCtrlCodes((&raw mut name).cast::<u8>());
        StringCopy(
            ((show).wrapping_add(4)).cast::<u8>(),
            (&raw mut name).cast::<u8>(),
        );
        crate::c::bf_write((show).wrapping_add(3), 0, 3, (flavor) as i32);
        crate::c::bf_write((show).wrapping_add(3), 3, 2, (color) as i32);
        ((show).wrapping_add(2)).write(sheen);
        StorePlayerIdInNormalShow(show);
        ((show).wrapping_add(20)).write(((&raw mut gGameLanguage).cast::<u8>()).read());
        if (((((show).wrapping_add(20)).read()) as i32) == 1i32) || (((language) as i32) == 1i32) {
            ((show).wrapping_add(21)).write(1u8);
        } else {
            ((show).wrapping_add(21)).write(language);
        }
        return 1u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PutFanClubSpecialOnTheAir() {
    unsafe {
        let mut show: *mut u8 = core::ptr::null_mut();
        let mut name = crate::ffi::Align4([0u8; 32]);
        let mut id: u32 = 0u32;
        show = (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(10188))
            .cast::<u8>())
        .wrapping_offset(
            ((((&raw mut gSpecialVar_0x8006).cast::<u16>()).read()) as i32) as isize * 36,
        );
        ((show).wrapping_add(22)).write(
            ((((((&raw mut gSpecialVar_0x8005).cast::<u16>()).read()) as i32).wrapping_mul(10i32))
                as u8),
        );
        StringCopy(
            ((show).wrapping_add(2)).cast::<u8>(),
            (((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).cast::<u8>(),
        );
        (show).write(11u8);
        ((show).wrapping_add(1)).write(1u8);
        id = GetPlayerIDAsU32();
        ((show).wrapping_add(10)).write(((id) as u8));
        ((show).wrapping_add(11)).write(((id >> 8) as u8));
        StringCopy(
            (&raw mut name).cast::<u8>(),
            (&raw mut gStringVar1).cast::<u8>(),
        );
        StripExtCtrlCodes((&raw mut name).cast::<u8>());
        StringCopy(
            ((show).wrapping_add(12)).cast::<u8>(),
            (&raw mut name).cast::<u8>(),
        );
        StorePlayerIdInNormalShow(show);
        ((show).wrapping_add(23)).write(((&raw mut gGameLanguage).cast::<u8>()).read());
        if (((((show).wrapping_add(23)).read()) as i32) == 1i32)
            || (((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(12624))
                .wrapping_add(80))
            .cast::<u8>())
            .read()) as i32)
                == 1i32)
        {
            ((show).wrapping_add(24)).write(1u8);
        } else {
            ((show).wrapping_add(24)).write(
                ((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(12624))
                    .wrapping_add(80))
                .cast::<u8>())
                .read(),
            );
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ContestLiveUpdates_Init(round1Placing: u8) {
    unsafe {
        let mut round1Placing = round1Placing;
        let mut show: *mut u8 = core::ptr::null_mut();
        DeleteTVShowInArrayByIdx(
            ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(10188))
                .cast::<u8>(),
            24u8,
        );
        ((&raw mut sCurTVShowSlot).cast::<u8>().cast::<i8>()).write(
            FindFirstEmptyNormalTVShowSlot(
                ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(10188))
                    .cast::<u8>(),
            ),
        );
        if ((((&raw mut sCurTVShowSlot).cast::<u8>().cast::<i8>()).read()) as i32) != (-1i32) {
            show = (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(10188))
                .cast::<u8>())
            .wrapping_offset(864);
            ((show).wrapping_add(13)).write(round1Placing);
            (show).write(8u8);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ContestLiveUpdates_SetRound2Placing(round2Placing: u8) {
    unsafe {
        let mut round2Placing = round2Placing;
        let mut show: *mut u8 = (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
            .wrapping_add(10188))
        .cast::<u8>())
        .wrapping_offset(864);
        ((&raw mut sCurTVShowSlot).cast::<u8>().cast::<i8>()).write(
            FindFirstEmptyNormalTVShowSlot(
                ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(10188))
                    .cast::<u8>(),
            ),
        );
        if ((((&raw mut sCurTVShowSlot).cast::<u8>().cast::<i8>()).read()) as i32) != (-1i32) {
            ((show).wrapping_add(14)).write(round2Placing);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ContestLiveUpdates_SetWinnerAppealFlag(flag: u8) {
    unsafe {
        let mut flag = flag;
        let mut show: *mut u8 = (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
            .wrapping_add(10188))
        .cast::<u8>())
        .wrapping_offset(864);
        ((&raw mut sCurTVShowSlot).cast::<u8>().cast::<i8>()).write(
            FindFirstEmptyNormalTVShowSlot(
                ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(10188))
                    .cast::<u8>(),
            ),
        );
        if ((((&raw mut sCurTVShowSlot).cast::<u8>().cast::<i8>()).read()) as i32) != (-1i32) {
            ((show).wrapping_add(15)).write(flag);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ContestLiveUpdates_SetWinnerMoveUsed(r#move: u16) {
    unsafe {
        let mut r#move = r#move;
        let mut show: *mut u8 = (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
            .wrapping_add(10188))
        .cast::<u8>())
        .wrapping_offset(864);
        ((&raw mut sCurTVShowSlot).cast::<u8>().cast::<i8>()).write(
            FindFirstEmptyNormalTVShowSlot(
                ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(10188))
                    .cast::<u8>(),
            ),
        );
        if ((((&raw mut sCurTVShowSlot).cast::<u8>().cast::<i8>()).read()) as i32) != (-1i32) {
            ((show).wrapping_add(16).cast::<u16>()).write(r#move);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ContestLiveUpdates_SetLoserData(flag: u8, loser: u8) {
    unsafe {
        let mut flag = flag;
        let mut loser = loser;
        let mut show: *mut u8 = (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
            .wrapping_add(10188))
        .cast::<u8>())
        .wrapping_offset(864);
        ((&raw mut sCurTVShowSlot).cast::<u8>().cast::<i8>()).write(
            FindFirstEmptyNormalTVShowSlot(
                ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(10188))
                    .cast::<u8>(),
            ),
        );
        if ((((&raw mut sCurTVShowSlot).cast::<u8>().cast::<i8>()).read()) as i32) != (-1i32) {
            ((show).wrapping_add(2).cast::<u16>()).write(
                ((((&raw mut gContestMons).cast::<u8>())
                    .wrapping_offset(((loser) as i32) as isize * 64))
                .cast::<u16>())
                .read(),
            );
            StringCopy(
                ((show).wrapping_add(4)).cast::<u8>(),
                ((((&raw mut gContestMons).cast::<u8>())
                    .wrapping_offset(((loser) as i32) as isize * 64))
                .wrapping_add(13))
                .cast::<u8>(),
            );
            StripExtCtrlCodes(((show).wrapping_add(4)).cast::<u8>());
            ((show).wrapping_add(12)).write(flag);
            if ((loser) as i32).wrapping_add(1i32)
                > ((((&raw mut gNumLinkContestPlayers).cast::<u8>()).read()) as i32)
            {
                ((show).wrapping_add(30)).write(
                    (((((&raw mut gLinkPlayers).cast::<u8>())
                        .wrapping_add(26)
                        .cast::<u16>())
                    .read()) as u8),
                );
            } else {
                if (((((&raw mut gGameLanguage).cast::<u8>()).read()) as i32) == 1i32)
                    || (((((((&raw mut gLinkPlayers).cast::<u8>())
                        .wrapping_offset(((loser) as i32) as isize * 28))
                    .wrapping_add(26)
                    .cast::<u16>())
                    .read()) as i32)
                        == 1i32)
                {
                    ((show).wrapping_add(30)).write(1u8);
                } else {
                    ((show).wrapping_add(30)).write(
                        ((((((&raw mut gLinkPlayers).cast::<u8>())
                            .wrapping_offset(((loser) as i32) as isize * 28))
                        .wrapping_add(26)
                        .cast::<u16>())
                        .read()) as u8),
                    );
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn InterviewAfter_BravoTrainerPokemonProfile() {
    unsafe {
        let mut show: *mut u8 = core::ptr::null_mut();
        let mut show2: *mut u8 = core::ptr::null_mut();
        show = (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(10188))
            .cast::<u8>())
        .wrapping_offset(864);
        if (((show).read()) as i32) == 6i32 {
            show2 = (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(10188))
                .cast::<u8>())
            .wrapping_offset(
                ((((&raw mut sCurTVShowSlot).cast::<u8>().cast::<i8>()).read()) as i32) as isize
                    * 36,
            );
            (show2).write(6u8);
            ((show2).wrapping_add(1)).write(1u8);
            ((show2).wrapping_add(2).cast::<u16>())
                .write(((show).wrapping_add(2).cast::<u16>()).read());
            StringCopy(
                ((show2).wrapping_add(22)).cast::<u8>(),
                (((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).cast::<u8>(),
            );
            StringCopy(
                ((show2).wrapping_add(8)).cast::<u8>(),
                ((show).wrapping_add(8)).cast::<u8>(),
            );
            crate::c::bf_write(
                (show2).wrapping_add(19),
                0,
                3,
                (crate::c::bf_read((show).wrapping_add(19), 0, 3, false) as u8) as i32,
            );
            crate::c::bf_write(
                (show2).wrapping_add(19),
                3,
                2,
                (crate::c::bf_read((show).wrapping_add(19), 3, 2, false) as u8) as i32,
            );
            ((show2).wrapping_add(20).cast::<u16>())
                .write(((show).wrapping_add(20).cast::<u16>()).read());
            crate::c::bf_write(
                (show2).wrapping_add(19),
                5,
                2,
                (crate::c::bf_read((show).wrapping_add(19), 5, 2, false) as u8) as i32,
            );
            crate::c::bf_write(
                (show2).wrapping_add(19),
                0,
                3,
                (crate::c::bf_read((show).wrapping_add(19), 0, 3, false) as u8) as i32,
            );
            StorePlayerIdInNormalShow(show2);
            ((show2).wrapping_add(30)).write(((&raw mut gGameLanguage).cast::<u8>()).read());
            if (((((show2).wrapping_add(30)).read()) as i32) == 1i32)
                || (((((show).wrapping_add(31)).read()) as i32) == 1i32)
            {
                ((show2).wrapping_add(31)).write(1u8);
            } else {
                ((show2).wrapping_add(31)).write(((show).wrapping_add(31)).read());
            }
            DeleteTVShowInArrayByIdx(
                ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(10188))
                    .cast::<u8>(),
                24u8,
            );
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BravoTrainerPokemonProfile_BeforeInterview1(r#move: u16) {
    unsafe {
        let mut r#move = r#move;
        let mut show: *mut u8 = (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
            .wrapping_add(10188))
        .cast::<u8>())
        .wrapping_offset(864);
        InterviewBefore_BravoTrainerPkmnProfile();
        ((&raw mut sCurTVShowSlot).cast::<u8>().cast::<i8>()).write(
            FindFirstEmptyNormalTVShowSlot(
                ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(10188))
                    .cast::<u8>(),
            ),
        );
        if ((((&raw mut sCurTVShowSlot).cast::<u8>().cast::<i8>()).read()) as i32) != (-1i32) {
            DeleteTVShowInArrayByIdx(
                ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(10188))
                    .cast::<u8>(),
                24u8,
            );
            ((show).wrapping_add(20).cast::<u16>()).write(r#move);
            (show).write(6u8);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BravoTrainerPokemonProfile_BeforeInterview2(contestStandingPlace: u8) {
    unsafe {
        let mut contestStandingPlace = contestStandingPlace;
        let mut show: *mut u8 = (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
            .wrapping_add(10188))
        .cast::<u8>())
        .wrapping_offset(864);
        ((&raw mut sCurTVShowSlot).cast::<u8>().cast::<i8>()).write(
            FindFirstEmptyNormalTVShowSlot(
                ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(10188))
                    .cast::<u8>(),
            ),
        );
        if ((((&raw mut sCurTVShowSlot).cast::<u8>().cast::<i8>()).read()) as i32) != (-1i32) {
            crate::c::bf_write((show).wrapping_add(19), 5, 2, (contestStandingPlace) as i32);
            crate::c::bf_write(
                (show).wrapping_add(19),
                0,
                3,
                ((((&raw mut gSpecialVar_ContestCategory).cast::<u16>()).read()) as u8) as i32,
            );
            crate::c::bf_write(
                (show).wrapping_add(19),
                3,
                2,
                ((((&raw mut gSpecialVar_ContestRank).cast::<u16>()).read()) as u8) as i32,
            );
            ((show).wrapping_add(2).cast::<u16>()).write(
                ((GetMonData3(
                    ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                        ((((&raw mut gContestMonPartyIndex).cast::<u8>()).read()) as i32) as isize
                            * 100,
                    ),
                    11i32,
                    core::ptr::null_mut(),
                )) as u16),
            );
            GetMonData3(
                ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                    ((((&raw mut gContestMonPartyIndex).cast::<u8>()).read()) as i32) as isize
                        * 100,
                ),
                2i32,
                ((show).wrapping_add(8)).cast::<u8>(),
            );
            StripExtCtrlCodes(((show).wrapping_add(8)).cast::<u8>());
            ((show).wrapping_add(31)).write(
                ((GetMonData2(
                    ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                        ((((&raw mut gContestMonPartyIndex).cast::<u8>()).read()) as i32) as isize
                            * 100,
                    ),
                    3i32,
                )) as u8),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn InterviewAfter_BravoTrainerBattleTowerProfile() {
    unsafe {
        let mut show: *mut u8 = (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
            .wrapping_add(10188))
        .cast::<u8>())
        .wrapping_offset(
            ((((&raw mut sCurTVShowSlot).cast::<u8>().cast::<i8>()).read()) as i32) as isize * 36,
        );
        (show).write(7u8);
        ((show).wrapping_add(1)).write(1u8);
        StringCopy(
            ((show).wrapping_add(2)).cast::<u8>(),
            (((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).cast::<u8>(),
        );
        StringCopy(
            ((show).wrapping_add(12)).cast::<u8>(),
            ((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(1416))
            .wrapping_add(4))
            .cast::<u8>(),
        );
        ((show).wrapping_add(10).cast::<u16>()).write(
            ((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(1416))
            .cast::<u16>())
            .read(),
        );
        ((show).wrapping_add(20).cast::<u16>()).write(
            ((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(1416))
            .wrapping_add(2)
            .cast::<u16>())
            .read(),
        );
        ((show).wrapping_add(22).cast::<u16>()).write(GetCurrentBattleTowerWinStreak(
            (((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(1723))
            .read(),
            0u8,
        ));
        ((show).wrapping_add(28)).write(
            (((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(1722))
            .read(),
        );
        if (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
            .wrapping_add(1723))
        .read()) as i32)
            == 0i32
        {
            ((show).wrapping_add(26)).write(50u8);
        } else {
            ((show).wrapping_add(26)).write(100u8);
        }
        ((show).wrapping_add(27))
            .write(((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as u8));
        StorePlayerIdInNormalShow(show);
        ((show).wrapping_add(29)).write(((&raw mut gGameLanguage).cast::<u8>()).read());
        if (((((show).wrapping_add(29)).read()) as i32) == 1i32)
            || (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(1416))
            .wrapping_add(23))
            .read()) as i32)
                == 1i32)
        {
            ((show).wrapping_add(30)).write(1u8);
        } else {
            ((show).wrapping_add(30)).write(
                ((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                    .wrapping_add(1416))
                .wrapping_add(23))
                .read(),
            );
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn TryPutSmartShopperOnAir() {
    unsafe {
        let mut show: *mut u8 = core::ptr::null_mut();
        let mut i: u8 = 0u8;
        if ((!(((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4))
            .cast::<i8>())
        .read()) as i32)
            == 26i32)
            && ((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4))
                .wrapping_add(1)
                .cast::<i8>())
            .read()) as i32)
                == 60i32)))
            && (!(((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4))
                .cast::<i8>())
            .read()) as i32)
                == 26i32)
                && ((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4))
                    .wrapping_add(1)
                    .cast::<i8>())
                .read()) as i32)
                    == 55i32))))
            && (!((BernoulliTrial(((crate::c::div_i32(65535i32, 3i32)) as u16))) != 0))
        {
            ((&raw mut sCurTVShowSlot).cast::<u8>().cast::<i8>()).write(
                FindFirstEmptyRecordMixTVShowSlot(
                    ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(10188))
                        .cast::<u8>(),
                ),
            );
            if (((((&raw mut sCurTVShowSlot).cast::<u8>().cast::<i8>()).read()) as i32) != (-1i32))
                && (((IsRecordMixShowAlreadySpawned(22u8, 0u8)) as i32) != 1i32)
            {
                SortPurchasesByQuantity();
                if (((((&raw mut gMartPurchaseHistory).cast::<u8>())
                    .wrapping_add(2)
                    .cast::<u16>())
                .read()) as i32)
                    >= 20i32
                {
                    show = (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(10188))
                    .cast::<u8>())
                    .wrapping_offset(
                        ((((&raw mut sCurTVShowSlot).cast::<u8>().cast::<i8>()).read()) as i32)
                            as isize
                            * 36,
                    );
                    (show).write(22u8);
                    ((show).wrapping_add(1)).write(0u8);
                    ((show).wrapping_add(18))
                        .write((((&raw mut gMapHeader).cast::<u8>()).wrapping_add(20)).read());
                    {
                        i = 0u8;
                        'l1: loop {
                            if !(((i) as i32) < 3i32) {
                                break 'l1;
                            }
                            'l2: {
                                ((((show).wrapping_add(6)).cast::<u16>())
                                    .wrapping_offset(((i) as i32) as isize))
                                .write(
                                    ((((&raw mut gMartPurchaseHistory).cast::<u8>())
                                        .wrapping_offset(((i) as i32) as isize * 4))
                                    .cast::<u16>())
                                    .read(),
                                );
                                ((((show).wrapping_add(12)).cast::<u16>())
                                    .wrapping_offset(((i) as i32) as isize))
                                .write(
                                    ((((&raw mut gMartPurchaseHistory).cast::<u8>())
                                        .wrapping_offset(((i) as i32) as isize * 4))
                                    .wrapping_add(2)
                                    .cast::<u16>())
                                    .read(),
                                );
                            }
                            i = (i).wrapping_add(1);
                        }
                    }
                    ((show).wrapping_add(2)).write(IsPokeNewsActive(1u8));
                    StringCopy(
                        ((show).wrapping_add(19)).cast::<u8>(),
                        (((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).cast::<u8>(),
                    );
                    StorePlayerIdInRecordMixShow(show);
                    ((show).wrapping_add(3)).write(((&raw mut gGameLanguage).cast::<u8>()).read());
                }
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PutNameRaterShowOnTheAir() {
    unsafe {
        let mut show: *mut u8 = core::ptr::null_mut();
        InterviewBefore_NameRater();
        if ((((&raw mut gSpecialVar_Result).cast::<u16>()).read()) as i32) != 1i32 {
            GetMonData3(
                ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                    ((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32) as isize * 100,
                ),
                2i32,
                (&raw mut gStringVar1).cast::<u8>(),
            );
            if (((StringLength((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).cast::<u8>()))
                as i32)
                > 1i32)
                && (((StringLength((&raw mut gStringVar1).cast::<u8>())) as i32) > 1i32)
            {
                show = (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                    .wrapping_add(10188))
                .cast::<u8>())
                .wrapping_offset(
                    ((((&raw mut sCurTVShowSlot).cast::<u8>().cast::<i8>()).read()) as i32)
                        as isize
                        * 36,
                );
                (show).write(5u8);
                ((show).wrapping_add(1)).write(1u8);
                ((show).wrapping_add(2).cast::<u16>()).write(
                    ((GetMonData3(
                        ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                            ((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32)
                                as isize
                                * 100,
                        ),
                        11i32,
                        core::ptr::null_mut(),
                    )) as u16),
                );
                ((show).wrapping_add(26))
                    .write(((crate::c::rem_i32(((Random()) as i32), 3i32)) as u8));
                ((show).wrapping_add(27))
                    .write(((crate::c::rem_i32(((Random()) as i32), 2i32)) as u8));
                ((show).wrapping_add(28).cast::<u16>()).write(
                    GetRandomDifferentSpeciesSeenByPlayer(
                        ((show).wrapping_add(2).cast::<u16>()).read(),
                    ),
                );
                StringCopy(
                    ((show).wrapping_add(15)).cast::<u8>(),
                    (((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).cast::<u8>(),
                );
                GetMonData3(
                    ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                        ((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32) as isize
                            * 100,
                    ),
                    2i32,
                    ((show).wrapping_add(4)).cast::<u8>(),
                );
                StripExtCtrlCodes(((show).wrapping_add(4)).cast::<u8>());
                StorePlayerIdInNormalShow(show);
                ((show).wrapping_add(30)).write(((&raw mut gGameLanguage).cast::<u8>()).read());
                ((show).wrapping_add(31)).write(
                    ((GetMonData2(
                        ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                            ((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32)
                                as isize
                                * 100,
                        ),
                        3i32,
                    )) as u8),
                );
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn StartMassOutbreak() {
    unsafe {
        let mut show: *mut u8 = (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
            .wrapping_add(10188))
        .cast::<u8>())
        .wrapping_offset(
            ((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32) as isize * 36,
        );
        ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
            .wrapping_add(11152)
            .cast::<u16>())
        .write(((show).wrapping_add(12).cast::<u16>()).read());
        ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(11154))
            .write(((show).wrapping_add(16)).read());
        ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(11155))
            .write(((show).wrapping_add(17)).read());
        ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(11156))
            .write(((show).wrapping_add(20)).read());
        ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(11157))
            .write(((show).wrapping_add(2)).read());
        ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
            .wrapping_add(11158)
            .cast::<u16>())
        .write(((show).wrapping_add(14).cast::<u16>()).read());
        (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(11160))
            .cast::<u16>())
        .write((((show).wrapping_add(4)).cast::<u16>()).read());
        ((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(11160))
            .cast::<u16>())
        .wrapping_offset(1))
        .write(((((show).wrapping_add(4)).cast::<u16>()).wrapping_offset(1)).read());
        ((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(11160))
            .cast::<u16>())
        .wrapping_offset(2))
        .write(((((show).wrapping_add(4)).cast::<u16>()).wrapping_offset(2)).read());
        ((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(11160))
            .cast::<u16>())
        .wrapping_offset(3))
        .write(((((show).wrapping_add(4)).cast::<u16>()).wrapping_offset(3)).read());
        ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(11168))
            .write(((show).wrapping_add(3)).read());
        ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(11169))
            .write(((show).wrapping_add(19)).read());
        ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
            .wrapping_add(11170)
            .cast::<u16>())
        .write(2u16);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PutLilycoveContestLadyShowOnTheAir() {
    unsafe {
        let mut show: *mut u8 = core::ptr::null_mut();
        Script_FindFirstEmptyNormalTVShowSlot();
        if ((((&raw mut gSpecialVar_Result).cast::<u16>()).read()) as i32) != 1i32 {
            show = (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(10188))
                .cast::<u8>())
            .wrapping_offset(
                ((((&raw mut sCurTVShowSlot).cast::<u8>().cast::<i8>()).read()) as i32) as isize
                    * 36,
            );
            BufferContestLadyLanguage((show).wrapping_add(23));
            ((show).wrapping_add(24)).write(2u8);
            (show).write(12u8);
            ((show).wrapping_add(1)).write(1u8);
            BufferContestLadyPlayerName(((show).wrapping_add(2)).cast::<u8>());
            BufferContestLadyMonName(
                (show).wrapping_add(10),
                ((show).wrapping_add(11)).cast::<u8>(),
            );
            ((show).wrapping_add(22)).write(GetContestLadyPokeblockState());
            StorePlayerIdInNormalShow(show);
        }
    }
}
pub(crate) unsafe extern "C" fn InterviewAfter_FanClubLetter() {
    unsafe {
        let mut show: *mut u8 = (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
            .wrapping_add(10188))
        .cast::<u8>())
        .wrapping_offset(
            ((((&raw mut sCurTVShowSlot).cast::<u8>().cast::<i8>()).read()) as i32) as isize * 36,
        );
        (show).write(1u8);
        ((show).wrapping_add(1)).write(1u8);
        StringCopy(
            ((show).wrapping_add(16)).cast::<u8>(),
            (((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).cast::<u8>(),
        );
        ((show).wrapping_add(2).cast::<u16>()).write(
            ((GetMonData3(
                ((&raw mut gPlayerParty).cast::<u8>())
                    .wrapping_offset(((GetLeadMonIndex()) as i32) as isize * 100),
                11i32,
                core::ptr::null_mut(),
            )) as u16),
        );
        StorePlayerIdInNormalShow(show);
        ((show).wrapping_add(24)).write(((&raw mut gGameLanguage).cast::<u8>()).read());
    }
}
pub(crate) unsafe extern "C" fn InterviewAfter_RecentHappenings() {
    unsafe {
        let mut show: *mut u8 = (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
            .wrapping_add(10188))
        .cast::<u8>())
        .wrapping_offset(
            ((((&raw mut sCurTVShowSlot).cast::<u8>().cast::<i8>()).read()) as i32) as isize * 36,
        );
        (show).write(2u8);
        ((show).wrapping_add(1)).write(1u8);
        StringCopy(
            ((show).wrapping_add(16)).cast::<u8>(),
            (((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).cast::<u8>(),
        );
        ((show).wrapping_add(2).cast::<u16>()).write(0u16);
        StorePlayerIdInNormalShow(show);
        ((show).wrapping_add(24)).write(((&raw mut gGameLanguage).cast::<u8>()).read());
    }
}
pub(crate) unsafe extern "C" fn InterviewAfter_PkmnFanClubOpinions() {
    unsafe {
        let mut show: *mut u8 = (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
            .wrapping_add(10188))
        .cast::<u8>())
        .wrapping_offset(
            ((((&raw mut sCurTVShowSlot).cast::<u8>().cast::<i8>()).read()) as i32) as isize * 36,
        );
        (show).write(3u8);
        ((show).wrapping_add(1)).write(1u8);
        crate::c::bf_write(
            (show).wrapping_add(4),
            0,
            4,
            ((GetMonData3(
                ((&raw mut gPlayerParty).cast::<u8>())
                    .wrapping_offset(((GetLeadMonIndex()) as i32) as isize * 100),
                32i32,
                core::ptr::null_mut(),
            ) >> 4) as u8) as i32,
        );
        crate::c::bf_write(
            (show).wrapping_add(4),
            4,
            4,
            ((((&raw mut gSpecialVar_0x8007).cast::<u16>()).read()) as u8) as i32,
        );
        StringCopy(
            ((show).wrapping_add(5)).cast::<u8>(),
            (((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).cast::<u8>(),
        );
        GetMonData3(
            ((&raw mut gPlayerParty).cast::<u8>())
                .wrapping_offset(((GetLeadMonIndex()) as i32) as isize * 100),
            2i32,
            ((show).wrapping_add(16)).cast::<u8>(),
        );
        StripExtCtrlCodes(((show).wrapping_add(16)).cast::<u8>());
        ((show).wrapping_add(2).cast::<u16>()).write(
            ((GetMonData3(
                ((&raw mut gPlayerParty).cast::<u8>())
                    .wrapping_offset(((GetLeadMonIndex()) as i32) as isize * 100),
                11i32,
                core::ptr::null_mut(),
            )) as u16),
        );
        StorePlayerIdInNormalShow(show);
        ((show).wrapping_add(13)).write(((&raw mut gGameLanguage).cast::<u8>()).read());
        if (((((&raw mut gGameLanguage).cast::<u8>()).read()) as i32) == 1i32)
            || (GetMonData2(
                ((&raw mut gPlayerParty).cast::<u8>())
                    .wrapping_offset(((GetLeadMonIndex()) as i32) as isize * 100),
                3i32,
            ) == 1u32)
        {
            ((show).wrapping_add(14)).write(1u8);
        } else {
            ((show).wrapping_add(14)).write(
                ((GetMonData2(
                    ((&raw mut gPlayerParty).cast::<u8>())
                        .wrapping_offset(((GetLeadMonIndex()) as i32) as isize * 100),
                    3i32,
                )) as u8),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn InterviewAfter_Dummy() {
    unsafe {
        let mut show: *mut u8 = (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
            .wrapping_add(10188))
        .cast::<u8>())
        .wrapping_offset(
            ((((&raw mut sCurTVShowSlot).cast::<u8>().cast::<i8>()).read()) as i32) as isize * 36,
        );
    }
}
pub(crate) unsafe extern "C" fn TryStartRandomMassOutbreak() {
    unsafe {
        let mut i: u8 = 0u8;
        let mut outbreakIdx: u16 = 0u16;
        let mut show: *mut u8 = core::ptr::null_mut();
        if (FlagGet(2148u16)) != 0 {
            {
                i = 0u8;
                'l1: loop {
                    if !(((i) as i32) < 24i32) {
                        break 'l1;
                    }
                    'l2: {
                        if ((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(10188))
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 36))
                        .read()) as i32)
                            == 41i32
                        {
                            return;
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
            if !((BernoulliTrial(((crate::c::div_i32(65535i32, 200i32)) as u16))) != 0) {
                ((&raw mut sCurTVShowSlot).cast::<u8>().cast::<i8>()).write(
                    FindFirstEmptyNormalTVShowSlot(
                        ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(10188))
                        .cast::<u8>(),
                    ),
                );
                if ((((&raw mut sCurTVShowSlot).cast::<u8>().cast::<i8>()).read()) as i32)
                    != (-1i32)
                {
                    outbreakIdx =
                        ((crate::c::rem_u32(((Random()) as u32), crate::c::div_u32(60u32, 12u32)))
                            as u16);
                    show = (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(10188))
                    .cast::<u8>())
                    .wrapping_offset(
                        ((((&raw mut sCurTVShowSlot).cast::<u8>().cast::<i8>()).read()) as i32)
                            as isize
                            * 36,
                    );
                    (show).write(41u8);
                    ((show).wrapping_add(1)).write(1u8);
                    ((show).wrapping_add(20)).write(
                        (((((&raw const sPokeOutbreakSpeciesList)
                            .cast::<u8>()
                            .cast_mut())
                        .cast::<u8>())
                        .wrapping_offset(((outbreakIdx) as i32) as isize * 12))
                        .wrapping_add(10))
                        .read(),
                    );
                    ((show).wrapping_add(2)).write(0u8);
                    ((show).wrapping_add(3)).write(0u8);
                    ((show).wrapping_add(12).cast::<u16>()).write(
                        (((((&raw const sPokeOutbreakSpeciesList)
                            .cast::<u8>()
                            .cast_mut())
                        .cast::<u8>())
                        .wrapping_offset(((outbreakIdx) as i32) as isize * 12))
                        .cast::<u16>())
                        .read(),
                    );
                    ((show).wrapping_add(14).cast::<u16>()).write(0u16);
                    (((show).wrapping_add(4)).cast::<u16>()).write(
                        ((((((&raw const sPokeOutbreakSpeciesList)
                            .cast::<u8>()
                            .cast_mut())
                        .cast::<u8>())
                        .wrapping_offset(((outbreakIdx) as i32) as isize * 12))
                        .wrapping_add(2))
                        .cast::<u16>())
                        .read(),
                    );
                    ((((show).wrapping_add(4)).cast::<u16>()).wrapping_offset(1)).write(
                        (((((((&raw const sPokeOutbreakSpeciesList)
                            .cast::<u8>()
                            .cast_mut())
                        .cast::<u8>())
                        .wrapping_offset(((outbreakIdx) as i32) as isize * 12))
                        .wrapping_add(2))
                        .cast::<u16>())
                        .wrapping_offset(1))
                        .read(),
                    );
                    ((((show).wrapping_add(4)).cast::<u16>()).wrapping_offset(2)).write(
                        (((((((&raw const sPokeOutbreakSpeciesList)
                            .cast::<u8>()
                            .cast_mut())
                        .cast::<u8>())
                        .wrapping_offset(((outbreakIdx) as i32) as isize * 12))
                        .wrapping_add(2))
                        .cast::<u16>())
                        .wrapping_offset(2))
                        .read(),
                    );
                    ((((show).wrapping_add(4)).cast::<u16>()).wrapping_offset(3)).write(
                        (((((((&raw const sPokeOutbreakSpeciesList)
                            .cast::<u8>()
                            .cast_mut())
                        .cast::<u8>())
                        .wrapping_offset(((outbreakIdx) as i32) as isize * 12))
                        .wrapping_add(2))
                        .cast::<u16>())
                        .wrapping_offset(3))
                        .read(),
                    );
                    ((show).wrapping_add(16)).write(
                        (((((&raw const sPokeOutbreakSpeciesList)
                            .cast::<u8>()
                            .cast_mut())
                        .cast::<u8>())
                        .wrapping_offset(((outbreakIdx) as i32) as isize * 12))
                        .wrapping_add(11))
                        .read(),
                    );
                    ((show).wrapping_add(17)).write(0u8);
                    ((show).wrapping_add(18)).write(0u8);
                    ((show).wrapping_add(19)).write(50u8);
                    ((show).wrapping_add(21)).write(0u8);
                    ((show).wrapping_add(22).cast::<u16>()).write(1u16);
                    StorePlayerIdInNormalShow(show);
                    ((show).wrapping_add(24)).write(((&raw mut gGameLanguage).cast::<u8>()).read());
                }
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn EndMassOutbreak() {
    unsafe {
        ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
            .wrapping_add(11152)
            .cast::<u16>())
        .write(0u16);
        ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(11154)).write(0u8);
        ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(11155)).write(0u8);
        ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(11156)).write(0u8);
        ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(11157)).write(0u8);
        ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
            .wrapping_add(11158)
            .cast::<u16>())
        .write(0u16);
        (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(11160))
            .cast::<u16>())
        .write(0u16);
        ((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(11160))
            .cast::<u16>())
        .wrapping_offset(1))
        .write(0u16);
        ((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(11160))
            .cast::<u16>())
        .wrapping_offset(2))
        .write(0u16);
        ((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(11160))
            .cast::<u16>())
        .wrapping_offset(3))
        .write(0u16);
        ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(11168)).write(0u8);
        ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(11169)).write(0u8);
        ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
            .wrapping_add(11170)
            .cast::<u16>())
        .write(0u16);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn UpdateTVShowsPerDay(days: u16) {
    unsafe {
        let mut days = days;
        UpdateTimeBeforeMassOutbreak(days);
        TryEndMassOutbreak(days);
        UpdatePokeNewsCountdown(days);
        ResolveWorldOfMastersShow(days);
        ResolveNumberOneShow(days);
    }
}
pub(crate) unsafe extern "C" fn UpdateTimeBeforeMassOutbreak(days: u16) {
    unsafe {
        let mut days = days;
        let mut i: u8 = 0u8;
        let mut show: *mut u8 = core::ptr::null_mut();
        if ((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
            .wrapping_add(11152)
            .cast::<u16>())
        .read()) as i32)
            == 0i32
        {
            {
                i = 0u8;
                'l1: loop {
                    if !(((i) as i32) < 24i32) {
                        break 'l1;
                    }
                    'l2: {
                        if (((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(10188))
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 36))
                        .read()) as i32)
                            == 41i32)
                            && ((((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(10188))
                            .cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 36))
                            .wrapping_add(1))
                            .read()) as i32)
                                == 1i32)
                        {
                            show = (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(10188))
                            .cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 36);
                            if ((((show).wrapping_add(22).cast::<u16>()).read()) as i32)
                                < ((days) as i32)
                            {
                                ((show).wrapping_add(22).cast::<u16>()).write(0u16);
                            } else {
                                let __p1 = (show).wrapping_add(22).cast::<u16>();
                                (__p1).write(
                                    (((((__p1).read()) as i32).wrapping_sub(((days) as i32)))
                                        as u16),
                                );
                            }
                            break 'l1;
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn TryEndMassOutbreak(days: u16) {
    unsafe {
        let mut days = days;
        if ((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
            .wrapping_add(11170)
            .cast::<u16>())
        .read()) as i32)
            <= ((days) as i32)
        {
            EndMassOutbreak();
        } else {
            let __p1 = (((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                .wrapping_add(11170)
                .cast::<u16>();
            (__p1).write((((((__p1).read()) as i32).wrapping_sub(((days) as i32))) as u16));
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn RecordFishingAttemptForTV(caughtFish: u8) {
    unsafe {
        let mut caughtFish = caughtFish;
        if (caughtFish) != 0 {
            if (((((&raw mut sPokemonAnglerAttemptCounters)
                .cast::<u8>()
                .cast::<u16>())
            .read()) as i32)
                >> 8)
                > 4i32
            {
                TryPutFishingAdviceOnAir();
            }
            let __p1 = (&raw mut sPokemonAnglerAttemptCounters)
                .cast::<u8>()
                .cast::<u16>();
            (__p1).write((((((__p1).read()) as i32) & 255i32) as u16));
            if ((((&raw mut sPokemonAnglerAttemptCounters)
                .cast::<u8>()
                .cast::<u16>())
            .read()) as i32)
                != 255i32
            {
                let __p2 = (&raw mut sPokemonAnglerAttemptCounters)
                    .cast::<u8>()
                    .cast::<u16>();
                (__p2).write((((((__p2).read()) as i32).wrapping_add(1i32)) as u16));
            }
        } else {
            if (((((&raw mut sPokemonAnglerAttemptCounters)
                .cast::<u8>()
                .cast::<u16>())
            .read()) as u8) as i32)
                > 4i32
            {
                TryPutFishingAdviceOnAir();
            }
            let __p3 = (&raw mut sPokemonAnglerAttemptCounters)
                .cast::<u8>()
                .cast::<u16>();
            (__p3).write((((((__p3).read()) as i32) & 65280i32) as u16));
            if (((((&raw mut sPokemonAnglerAttemptCounters)
                .cast::<u8>()
                .cast::<u16>())
            .read()) as i32)
                >> 8)
                != 255i32
            {
                let __p4 = (&raw mut sPokemonAnglerAttemptCounters)
                    .cast::<u8>()
                    .cast::<u16>();
                (__p4).write((((((__p4).read()) as i32).wrapping_add(256i32)) as u16));
            }
        }
    }
}
pub(crate) unsafe extern "C" fn TryPutFishingAdviceOnAir() {
    unsafe {
        let mut show: *mut u8 = core::ptr::null_mut();
        ((&raw mut sCurTVShowSlot).cast::<u8>().cast::<i8>()).write(
            FindFirstEmptyRecordMixTVShowSlot(
                ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(10188))
                    .cast::<u8>(),
            ),
        );
        if (((((&raw mut sCurTVShowSlot).cast::<u8>().cast::<i8>()).read()) as i32) != (-1i32))
            && (((IsRecordMixShowAlreadySpawned(24u8, 0u8)) as i32) != 1i32)
        {
            show = (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(10188))
                .cast::<u8>())
            .wrapping_offset(
                ((((&raw mut sCurTVShowSlot).cast::<u8>().cast::<i8>()).read()) as i32) as isize
                    * 36,
            );
            (show).write(24u8);
            ((show).wrapping_add(1)).write(0u8);
            ((show).wrapping_add(2)).write(
                ((((&raw mut sPokemonAnglerAttemptCounters)
                    .cast::<u8>()
                    .cast::<u16>())
                .read()) as u8),
            );
            ((show).wrapping_add(3)).write(
                ((((((&raw mut sPokemonAnglerAttemptCounters)
                    .cast::<u8>()
                    .cast::<u16>())
                .read()) as i32)
                    >> 8) as u8),
            );
            ((show).wrapping_add(4).cast::<u16>())
                .write(((&raw mut sPokemonAnglerSpecies).cast::<u8>().cast::<u16>()).read());
            StringCopy(
                ((show).wrapping_add(19)).cast::<u8>(),
                (((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).cast::<u8>(),
            );
            StorePlayerIdInRecordMixShow(show);
            ((show).wrapping_add(6)).write(((&raw mut gGameLanguage).cast::<u8>()).read());
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetPokemonAnglerSpecies(species: u16) {
    unsafe {
        let mut species = species;
        ((&raw mut sPokemonAnglerSpecies).cast::<u8>().cast::<u16>()).write(species);
    }
}
pub(crate) unsafe extern "C" fn ResolveWorldOfMastersShow(days: u16) {
    unsafe {
        let mut days = days;
        let mut show: *mut u8 = (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
            .wrapping_add(10188))
        .cast::<u8>())
        .wrapping_offset(864);
        if (((show).read()) as i32) == 25i32 {
            if ((((show).wrapping_add(2).cast::<u16>()).read()) as i32) >= 20i32 {
                TryPutWorldOfMastersOnAir();
            }
            DeleteTVShowInArrayByIdx(
                ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(10188))
                    .cast::<u8>(),
                24u8,
            );
        }
    }
}
pub(crate) unsafe extern "C" fn TryPutWorldOfMastersOnAir() {
    unsafe {
        let mut show: *mut u8 = core::ptr::null_mut();
        let mut show2: *mut u8 = core::ptr::null_mut();
        show = (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(10188))
            .cast::<u8>())
        .wrapping_offset(864);
        if !((BernoulliTrial(((crate::c::div_i32(65535i32, 1i32)) as u16))) != 0) {
            ((&raw mut sCurTVShowSlot).cast::<u8>().cast::<i8>()).write(
                FindFirstEmptyRecordMixTVShowSlot(
                    ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(10188))
                        .cast::<u8>(),
                ),
            );
            if (((((&raw mut sCurTVShowSlot).cast::<u8>().cast::<i8>()).read()) as i32) != (-1i32))
                && (((IsRecordMixShowAlreadySpawned(25u8, 0u8)) as i32) != 1i32)
            {
                show2 = (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                    .wrapping_add(10188))
                .cast::<u8>())
                .wrapping_offset(
                    ((((&raw mut sCurTVShowSlot).cast::<u8>().cast::<i8>()).read()) as i32)
                        as isize
                        * 36,
                );
                (show2).write(25u8);
                ((show2).wrapping_add(1)).write(0u8);
                ((show2).wrapping_add(2).cast::<u16>())
                    .write(((show).wrapping_add(2).cast::<u16>()).read());
                ((show2).wrapping_add(6).cast::<u16>()).write(
                    (((GetGameStat(5u8))
                        .wrapping_sub(((((show).wrapping_add(6).cast::<u16>()).read()) as u32)))
                        as u16),
                );
                ((show2).wrapping_add(4).cast::<u16>())
                    .write(((show).wrapping_add(4).cast::<u16>()).read());
                ((show2).wrapping_add(8).cast::<u16>())
                    .write(((show).wrapping_add(8).cast::<u16>()).read());
                ((show2).wrapping_add(10)).write(((show).wrapping_add(10)).read());
                StringCopy(
                    ((show2).wrapping_add(19)).cast::<u8>(),
                    (((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).cast::<u8>(),
                );
                StorePlayerIdInRecordMixShow(show2);
                ((show2).wrapping_add(11)).write(((&raw mut gGameLanguage).cast::<u8>()).read());
                DeleteTVShowInArrayByIdx(
                    ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(10188))
                        .cast::<u8>(),
                    24u8,
                );
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn TryPutTodaysRivalTrainerOnAir() {
    unsafe {
        let mut show: *mut u8 = core::ptr::null_mut();
        let mut i: u32 = 0u32;
        let mut nBadges: u8 = 0u8;
        IsRecordMixShowAlreadySpawned(26u8, 1u8);
        ((&raw mut sCurTVShowSlot).cast::<u8>().cast::<i8>()).write(
            FindFirstEmptyRecordMixTVShowSlot(
                ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(10188))
                    .cast::<u8>(),
            ),
        );
        if ((((&raw mut sCurTVShowSlot).cast::<u8>().cast::<i8>()).read()) as i32) != (-1i32) {
            show = (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(10188))
                .cast::<u8>())
            .wrapping_offset(
                ((((&raw mut sCurTVShowSlot).cast::<u8>().cast::<i8>()).read()) as i32) as isize
                    * 36,
            );
            (show).write(26u8);
            ((show).wrapping_add(1)).write(0u8);
            {
                i = 2151u32;
                nBadges = 0u8;
                'l1: loop {
                    if !(i < 2159u32) {
                        break 'l1;
                    }
                    'l2: {
                        if (FlagGet(((i) as u16))) != 0 {
                            nBadges = (nBadges).wrapping_add(1);
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
            ((show).wrapping_add(4)).write(nBadges);
            if (IsNationalPokedexEnabled()) != 0 {
                ((show).wrapping_add(2).cast::<u16>()).write(GetNationalPokedexCount(1u8));
            } else {
                ((show).wrapping_add(2).cast::<u16>()).write(GetHoennPokedexCount(1u8));
            }
            ((show).wrapping_add(7))
                .write((((&raw mut gMapHeader).cast::<u8>()).wrapping_add(20)).read());
            ((show).wrapping_add(10).cast::<u16>()).write(
                (((&raw mut gMapHeader).cast::<u8>())
                    .wrapping_add(18)
                    .cast::<u16>())
                .read(),
            );
            ((show).wrapping_add(5)).write(0u8);
            ((show).wrapping_add(6)).write(0u8);
            {
                i = 0u32;
                'l3: loop {
                    if !(i < 7u32) {
                        break 'l3;
                    }
                    'l4: {
                        if ((FlagGet(
                            ((((&raw const sSilverSymbolFlags)
                                .cast::<u8>()
                                .cast_mut()
                                .cast::<u16>())
                            .cast::<u16>())
                            .wrapping_offset(((i) as i32) as isize))
                            .read(),
                        )) as i32)
                            == 1i32
                        {
                            let __p1 = (show).wrapping_add(5);
                            (__p1).write(((__p1).read()).wrapping_add(1));
                        }
                        if ((FlagGet(
                            ((((&raw const sGoldSymbolFlags)
                                .cast::<u8>()
                                .cast_mut()
                                .cast::<u16>())
                            .cast::<u16>())
                            .wrapping_offset(((i) as i32) as isize))
                            .read(),
                        )) as i32)
                            == 1i32
                        {
                            let __p2 = (show).wrapping_add(6);
                            (__p2).write(((__p2).read()).wrapping_add(1));
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
            ((show).wrapping_add(8).cast::<u16>()).write(
                (((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                    .wrapping_add(2156)
                    .cast::<u16>())
                .read(),
            );
            StringCopy(
                ((show).wrapping_add(19)).cast::<u8>(),
                (((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).cast::<u8>(),
            );
            StorePlayerIdInRecordMixShow(show);
            ((show).wrapping_add(12)).write(((&raw mut gGameLanguage).cast::<u8>()).read());
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn TryPutTrendWatcherOnAir(words: *mut u16) {
    unsafe {
        let mut words = words;
        let mut show: *mut u8 = core::ptr::null_mut();
        ((&raw mut sCurTVShowSlot).cast::<u8>().cast::<i8>()).write(
            FindFirstEmptyRecordMixTVShowSlot(
                ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(10188))
                    .cast::<u8>(),
            ),
        );
        if (((((&raw mut sCurTVShowSlot).cast::<u8>().cast::<i8>()).read()) as i32) != (-1i32))
            && (((IsRecordMixShowAlreadySpawned(27u8, 0u8)) as i32) != 1i32)
        {
            show = (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(10188))
                .cast::<u8>())
            .wrapping_offset(
                ((((&raw mut sCurTVShowSlot).cast::<u8>().cast::<i8>()).read()) as i32) as isize
                    * 36,
            );
            (show).write(27u8);
            ((show).wrapping_add(1)).write(0u8);
            ((show).wrapping_add(8)).write(
                ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(8)).read(),
            );
            (((show).wrapping_add(4)).cast::<u16>()).write((words).read());
            ((((show).wrapping_add(4)).cast::<u16>()).wrapping_offset(1))
                .write(((words).wrapping_offset(1)).read());
            StringCopy(
                ((show).wrapping_add(19)).cast::<u8>(),
                (((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).cast::<u8>(),
            );
            StorePlayerIdInRecordMixShow(show);
            ((show).wrapping_add(9)).write(((&raw mut gGameLanguage).cast::<u8>()).read());
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn TryPutTreasureInvestigatorsOnAir() {
    unsafe {
        let mut show: *mut u8 = core::ptr::null_mut();
        ((&raw mut sCurTVShowSlot).cast::<u8>().cast::<i8>()).write(
            FindFirstEmptyRecordMixTVShowSlot(
                ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(10188))
                    .cast::<u8>(),
            ),
        );
        if (((((&raw mut sCurTVShowSlot).cast::<u8>().cast::<i8>()).read()) as i32) != (-1i32))
            && (((IsRecordMixShowAlreadySpawned(28u8, 0u8)) as i32) != 1i32)
        {
            show = (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(10188))
                .cast::<u8>())
            .wrapping_offset(
                ((((&raw mut sCurTVShowSlot).cast::<u8>().cast::<i8>()).read()) as i32) as isize
                    * 36,
            );
            (show).write(28u8);
            ((show).wrapping_add(1)).write(0u8);
            ((show).wrapping_add(2).cast::<u16>())
                .write(((&raw mut gSpecialVar_0x8005).cast::<u16>()).read());
            ((show).wrapping_add(4))
                .write((((&raw mut gMapHeader).cast::<u8>()).wrapping_add(20)).read());
            ((show).wrapping_add(6).cast::<u16>()).write(
                (((&raw mut gMapHeader).cast::<u8>())
                    .wrapping_add(18)
                    .cast::<u16>())
                .read(),
            );
            StringCopy(
                ((show).wrapping_add(19)).cast::<u8>(),
                (((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).cast::<u8>(),
            );
            StorePlayerIdInRecordMixShow(show);
            ((show).wrapping_add(5)).write(((&raw mut gGameLanguage).cast::<u8>()).read());
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn TryPutFindThatGamerOnAir(nCoinsPaidOut: u16) {
    unsafe {
        let mut nCoinsPaidOut = nCoinsPaidOut;
        let mut show: *mut u8 = core::ptr::null_mut();
        let mut flag: u8 = 0u8;
        let mut nCoinsWon: u16 = 0u16;
        ((&raw mut sCurTVShowSlot).cast::<u8>().cast::<i8>()).write(
            FindFirstEmptyRecordMixTVShowSlot(
                ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(10188))
                    .cast::<u8>(),
            ),
        );
        if (((((&raw mut sCurTVShowSlot).cast::<u8>().cast::<i8>()).read()) as i32) != (-1i32))
            && (((IsRecordMixShowAlreadySpawned(29u8, 0u8)) as i32) != 1i32)
        {
            flag = 0u8;
            'l1: {
                let __sw1 = ((((&raw mut sFindThatGamerWhichGame).cast::<u8>().cast::<u8>()).read())
                    as i32);
                let __matched = __sw1 == 0i32 || __sw1 == 1i32;
                if __sw1 == 0i32 {
                    if ((nCoinsPaidOut) as i32)
                        >= ((((&raw mut sFindThatGamerCoinsSpent)
                            .cast::<u8>()
                            .cast::<u16>())
                        .read()) as i32)
                            .wrapping_add(200i32)
                    {
                        flag = 1u8;
                        nCoinsWon = ((((nCoinsPaidOut) as i32).wrapping_sub(
                            ((((&raw mut sFindThatGamerCoinsSpent)
                                .cast::<u8>()
                                .cast::<u16>())
                            .read()) as i32),
                        )) as u16);
                        break 'l1;
                    }
                    if (((((&raw mut sFindThatGamerCoinsSpent)
                        .cast::<u8>()
                        .cast::<u16>())
                    .read()) as i32)
                        >= 100i32)
                        && (((nCoinsPaidOut) as i32)
                            <= ((((&raw mut sFindThatGamerCoinsSpent)
                                .cast::<u8>()
                                .cast::<u16>())
                            .read()) as i32)
                                .wrapping_sub(100i32))
                    {
                        nCoinsWon = ((((((&raw mut sFindThatGamerCoinsSpent)
                            .cast::<u8>()
                            .cast::<u16>())
                        .read()) as i32)
                            .wrapping_sub(((nCoinsPaidOut) as i32)))
                            as u16);
                        break 'l1;
                    }
                    return;
                }
                if __sw1 == 1i32 {
                    if ((nCoinsPaidOut) as i32)
                        >= ((((&raw mut sFindThatGamerCoinsSpent)
                            .cast::<u8>()
                            .cast::<u16>())
                        .read()) as i32)
                            .wrapping_add(50i32)
                    {
                        flag = 1u8;
                        nCoinsWon = ((((nCoinsPaidOut) as i32).wrapping_sub(
                            ((((&raw mut sFindThatGamerCoinsSpent)
                                .cast::<u8>()
                                .cast::<u16>())
                            .read()) as i32),
                        )) as u16);
                        break 'l1;
                    }
                    if (((((&raw mut sFindThatGamerCoinsSpent)
                        .cast::<u8>()
                        .cast::<u16>())
                    .read()) as i32)
                        >= 50i32)
                        && (((nCoinsPaidOut) as i32)
                            <= ((((&raw mut sFindThatGamerCoinsSpent)
                                .cast::<u8>()
                                .cast::<u16>())
                            .read()) as i32)
                                .wrapping_sub(50i32))
                    {
                        nCoinsWon = ((((((&raw mut sFindThatGamerCoinsSpent)
                            .cast::<u8>()
                            .cast::<u16>())
                        .read()) as i32)
                            .wrapping_sub(((nCoinsPaidOut) as i32)))
                            as u16);
                        break 'l1;
                    }
                    return;
                }
                if !__matched {
                    return;
                }
            }
            show = (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(10188))
                .cast::<u8>())
            .wrapping_offset(
                ((((&raw mut sCurTVShowSlot).cast::<u8>().cast::<i8>()).read()) as i32) as isize
                    * 36,
            );
            (show).write(29u8);
            ((show).wrapping_add(1)).write(0u8);
            ((show).wrapping_add(4).cast::<u16>()).write(nCoinsWon);
            ((show).wrapping_add(3))
                .write(((&raw mut sFindThatGamerWhichGame).cast::<u8>().cast::<u8>()).read());
            ((show).wrapping_add(2)).write(flag);
            StringCopy(
                ((show).wrapping_add(19)).cast::<u8>(),
                (((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).cast::<u8>(),
            );
            StorePlayerIdInRecordMixShow(show);
            ((show).wrapping_add(8)).write(((&raw mut gGameLanguage).cast::<u8>()).read());
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AlertTVThatPlayerPlayedSlotMachine(nCoinsSpent: u16) {
    unsafe {
        let mut nCoinsSpent = nCoinsSpent;
        ((&raw mut sFindThatGamerWhichGame).cast::<u8>().cast::<u8>()).write(0u8);
        ((&raw mut sFindThatGamerCoinsSpent)
            .cast::<u8>()
            .cast::<u16>())
        .write(nCoinsSpent);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AlertTVThatPlayerPlayedRoulette(nCoinsSpent: u16) {
    unsafe {
        let mut nCoinsSpent = nCoinsSpent;
        ((&raw mut sFindThatGamerWhichGame).cast::<u8>().cast::<u8>()).write(1u8);
        ((&raw mut sFindThatGamerCoinsSpent)
            .cast::<u8>()
            .cast::<u16>())
        .write(nCoinsSpent);
    }
}
pub(crate) unsafe extern "C" fn SecretBaseVisit_CalculateDecorationData(show: *mut u8) {
    unsafe {
        let mut show = show;
        let mut i: u8 = 0u8;
        let mut j: u8 = 0u8;
        let mut k: u16 = 0u16;
        let mut n: u8 = 0u8;
        let mut decoration: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 16i32) {
                    break 'l1;
                }
                'l2: {
                    ((((&raw mut sTV_DecorationsBuffer).cast::<u8>()).cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize))
                    .write(0u8);
                }
                i = (i).wrapping_add(1);
            }
        }
        {
            i = 0u8;
            n = 0u8;
            'l3: loop {
                if !(((i) as i32) < 16i32) {
                    break 'l3;
                }
                'l4: {
                    decoration = ((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(6812))
                    .cast::<u8>())
                    .wrapping_add(18))
                    .cast::<u8>())
                    .wrapping_offset(((i) as i32) as isize))
                    .read();
                    if ((decoration) as i32) != 0i32 {
                        {
                            j = 0u8;
                            'l5: loop {
                                if !(((j) as i32) < 16i32) {
                                    break 'l5;
                                }
                                'l6: {
                                    if ((((((&raw mut sTV_DecorationsBuffer).cast::<u8>())
                                        .cast::<u8>())
                                    .wrapping_offset(((j) as i32) as isize))
                                    .read()) as i32)
                                        == 0i32
                                    {
                                        ((((&raw mut sTV_DecorationsBuffer).cast::<u8>())
                                            .cast::<u8>())
                                        .wrapping_offset(((j) as i32) as isize))
                                        .write(decoration);
                                        n = (n).wrapping_add(1);
                                        break 'l5;
                                    }
                                    if ((((((&raw mut sTV_DecorationsBuffer).cast::<u8>())
                                        .cast::<u8>())
                                    .wrapping_offset(((j) as i32) as isize))
                                    .read()) as i32)
                                        == ((decoration) as i32)
                                    {
                                        break 'l5;
                                    }
                                }
                                j = (j).wrapping_add(1);
                            }
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        if ((n) as u32) > crate::c::div_u32(4u32, 1u32) {
            ((show).wrapping_add(3)).write(((crate::c::div_u32(4u32, 1u32)) as u8));
        } else {
            ((show).wrapping_add(3)).write(n);
        }
        'l7: {
            let __sw1 = ((((show).wrapping_add(3)).read()) as i32);
            let __matched = __sw1 == 0i32 || __sw1 == 1i32;
            if __sw1 == 0i32 {
                break 'l7;
            }
            if __sw1 == 1i32 {
                (((show).wrapping_add(4)).cast::<u8>())
                    .write((((&raw mut sTV_DecorationsBuffer).cast::<u8>()).cast::<u8>()).read());
                break 'l7;
            }
            if !__matched {
                {
                    k = 0u16;
                    'l8: loop {
                        if !(((k) as i32) < ((n) as i32).wrapping_mul(((n) as i32))) {
                            break 'l8;
                        }
                        'l9: {
                            decoration =
                                ((crate::c::rem_i32(((Random()) as i32), ((n) as i32))) as u8);
                            j = ((crate::c::rem_i32(((Random()) as i32), ((n) as i32))) as u8);
                            {
                                i = ((((&raw mut sTV_DecorationsBuffer).cast::<u8>())
                                    .cast::<u8>())
                                .wrapping_offset(((decoration) as i32) as isize))
                                .read();
                                ((((&raw mut sTV_DecorationsBuffer).cast::<u8>()).cast::<u8>())
                                    .wrapping_offset(((decoration) as i32) as isize))
                                .write(
                                    ((((&raw mut sTV_DecorationsBuffer).cast::<u8>())
                                        .cast::<u8>())
                                    .wrapping_offset(((j) as i32) as isize))
                                    .read(),
                                );
                                ((((&raw mut sTV_DecorationsBuffer).cast::<u8>()).cast::<u8>())
                                    .wrapping_offset(((j) as i32) as isize))
                                .write(i);
                            }
                        }
                        k = (k).wrapping_add(1);
                    }
                }
                {
                    i = 0u8;
                    'l10: loop {
                        if !(((i) as i32) < ((((show).wrapping_add(3)).read()) as i32)) {
                            break 'l10;
                        }
                        'l11: {
                            ((((show).wrapping_add(4)).cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize))
                            .write(
                                ((((&raw mut sTV_DecorationsBuffer).cast::<u8>()).cast::<u8>())
                                    .wrapping_offset(((i) as i32) as isize))
                                .read(),
                            );
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                break 'l7;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SecretBaseVisit_CalculatePartyData(show: *mut u8) {
    unsafe {
        let mut show = show;
        let mut i: u8 = 0u8;
        let mut r#move: u16 = 0u16;
        let mut j: u16 = 0u16;
        let mut numMoves: u8 = 0u8;
        let mut numPokemon: u8 = 0u8;
        let mut sum: u16 = 0u16;
        {
            i = 0u8;
            numPokemon = 0u8;
            'l1: loop {
                if !(((i) as i32) < 6i32) {
                    break 'l1;
                }
                'l2: {
                    if (GetMonData2(
                        ((&raw mut gPlayerParty).cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 100),
                        11i32,
                    ) != 0u32)
                        && (!((GetMonData2(
                            ((&raw mut gPlayerParty).cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize * 100),
                            45i32,
                        )) != 0))
                    {
                        ((((&raw mut sTV_SecretBaseVisitMonsTemp).cast::<u8>()).cast::<u8>())
                            .wrapping_offset(((numPokemon) as i32) as isize * 8))
                        .write(
                            ((GetMonData2(
                                ((&raw mut gPlayerParty).cast::<u8>())
                                    .wrapping_offset(((i) as i32) as isize * 100),
                                56i32,
                            )) as u8),
                        );
                        (((((&raw mut sTV_SecretBaseVisitMonsTemp).cast::<u8>()).cast::<u8>())
                            .wrapping_offset(((numPokemon) as i32) as isize * 8))
                        .wrapping_add(2)
                        .cast::<u16>())
                        .write(
                            ((GetMonData2(
                                ((&raw mut gPlayerParty).cast::<u8>())
                                    .wrapping_offset(((i) as i32) as isize * 100),
                                11i32,
                            )) as u16),
                        );
                        numMoves = 0u8;
                        r#move = ((GetMonData2(
                            ((&raw mut gPlayerParty).cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize * 100),
                            13i32,
                        )) as u16);
                        if ((r#move) as i32) != 0i32 {
                            ((((&raw mut sTV_SecretBaseVisitMovesTemp)
                                .cast::<u8>()
                                .cast::<u16>())
                            .cast::<u16>())
                            .wrapping_offset(((numMoves) as i32) as isize))
                            .write(r#move);
                            numMoves = (numMoves).wrapping_add(1);
                        }
                        r#move = ((GetMonData2(
                            ((&raw mut gPlayerParty).cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize * 100),
                            14i32,
                        )) as u16);
                        if ((r#move) as i32) != 0i32 {
                            ((((&raw mut sTV_SecretBaseVisitMovesTemp)
                                .cast::<u8>()
                                .cast::<u16>())
                            .cast::<u16>())
                            .wrapping_offset(((numMoves) as i32) as isize))
                            .write(r#move);
                            numMoves = (numMoves).wrapping_add(1);
                        }
                        r#move = ((GetMonData2(
                            ((&raw mut gPlayerParty).cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize * 100),
                            15i32,
                        )) as u16);
                        if ((r#move) as i32) != 0i32 {
                            ((((&raw mut sTV_SecretBaseVisitMovesTemp)
                                .cast::<u8>()
                                .cast::<u16>())
                            .cast::<u16>())
                            .wrapping_offset(((numMoves) as i32) as isize))
                            .write(r#move);
                            numMoves = (numMoves).wrapping_add(1);
                        }
                        r#move = ((GetMonData2(
                            ((&raw mut gPlayerParty).cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize * 100),
                            16i32,
                        )) as u16);
                        if ((r#move) as i32) != 0i32 {
                            ((((&raw mut sTV_SecretBaseVisitMovesTemp)
                                .cast::<u8>()
                                .cast::<u16>())
                            .cast::<u16>())
                            .wrapping_offset(((numMoves) as i32) as isize))
                            .write(r#move);
                            numMoves = (numMoves).wrapping_add(1);
                        }
                        (((((&raw mut sTV_SecretBaseVisitMonsTemp).cast::<u8>()).cast::<u8>())
                            .wrapping_offset(((numPokemon) as i32) as isize * 8))
                        .wrapping_add(4)
                        .cast::<u16>())
                        .write(
                            ((((&raw mut sTV_SecretBaseVisitMovesTemp)
                                .cast::<u8>()
                                .cast::<u16>())
                            .cast::<u16>())
                            .wrapping_offset(
                                (crate::c::rem_i32(((Random()) as i32), ((numMoves) as i32)))
                                    as isize,
                            ))
                            .read(),
                        );
                        numPokemon = (numPokemon).wrapping_add(1);
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        {
            i = 0u8;
            sum = 0u16;
            'l3: loop {
                if !(((i) as i32) < ((numPokemon) as i32)) {
                    break 'l3;
                }
                'l4: {
                    sum = ((((sum) as i32).wrapping_add(
                        ((((((&raw mut sTV_SecretBaseVisitMonsTemp).cast::<u8>()).cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 8))
                        .read()) as i32),
                    )) as u16);
                }
                i = (i).wrapping_add(1);
            }
        }
        ((show).wrapping_add(2))
            .write(((crate::c::div_i32(((sum) as i32), ((numPokemon) as i32))) as u8));
        j = ((crate::c::rem_i32(((Random()) as i32), ((numPokemon) as i32))) as u16);
        ((show).wrapping_add(8).cast::<u16>()).write(
            (((((&raw mut sTV_SecretBaseVisitMonsTemp).cast::<u8>()).cast::<u8>())
                .wrapping_offset(((j) as i32) as isize * 8))
            .wrapping_add(2)
            .cast::<u16>())
            .read(),
        );
        ((show).wrapping_add(10).cast::<u16>()).write(
            (((((&raw mut sTV_SecretBaseVisitMonsTemp).cast::<u8>()).cast::<u8>())
                .wrapping_offset(((j) as i32) as isize * 8))
            .wrapping_add(4)
            .cast::<u16>())
            .read(),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn TryPutSecretBaseVisitOnAir() {
    unsafe {
        let mut show: *mut u8 = core::ptr::null_mut();
        IsRecordMixShowAlreadySpawned(31u8, 1u8);
        ((&raw mut sCurTVShowSlot).cast::<u8>().cast::<i8>()).write(
            FindFirstEmptyRecordMixTVShowSlot(
                ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(10188))
                    .cast::<u8>(),
            ),
        );
        if ((((&raw mut sCurTVShowSlot).cast::<u8>().cast::<i8>()).read()) as i32) != (-1i32) {
            show = (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(10188))
                .cast::<u8>())
            .wrapping_offset(
                ((((&raw mut sCurTVShowSlot).cast::<u8>().cast::<i8>()).read()) as i32) as isize
                    * 36,
            );
            (show).write(31u8);
            ((show).wrapping_add(1)).write(0u8);
            StringCopy(
                ((show).wrapping_add(19)).cast::<u8>(),
                (((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).cast::<u8>(),
            );
            SecretBaseVisit_CalculateDecorationData(show);
            SecretBaseVisit_CalculatePartyData(show);
            StorePlayerIdInRecordMixShow(show);
            ((show).wrapping_add(12)).write(((&raw mut gGameLanguage).cast::<u8>()).read());
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn TryPutBreakingNewsOnAir() {
    unsafe {
        let mut show: *mut u8 = core::ptr::null_mut();
        let mut i: u8 = 0u8;
        let mut balls: u16 = 0u16;
        ((&raw mut sCurTVShowSlot).cast::<u8>().cast::<i8>()).write(
            FindFirstEmptyRecordMixTVShowSlot(
                ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(10188))
                    .cast::<u8>(),
            ),
        );
        if (((((&raw mut sCurTVShowSlot).cast::<u8>().cast::<i8>()).read()) as i32) != (-1i32))
            && (((IsRecordMixShowAlreadySpawned(30u8, 0u8)) as i32) != 1i32)
        {
            show = (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(10188))
                .cast::<u8>())
            .wrapping_offset(
                ((((&raw mut sCurTVShowSlot).cast::<u8>().cast::<i8>()).read()) as i32) as isize
                    * 36,
            );
            (show).write(30u8);
            ((show).wrapping_add(1)).write(0u8);
            balls = 0u16;
            {
                i = 0u8;
                'l1: loop {
                    if !(((i) as i32) < 11i32) {
                        break 'l1;
                    }
                    'l2: {
                        balls = ((((balls) as i32).wrapping_add(
                            (((((((&raw mut gBattleResults).cast::<u8>()).wrapping_add(54))
                                .cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize))
                            .read()) as i32),
                        )) as u16);
                    }
                    i = (i).wrapping_add(1);
                }
            }
            if (crate::c::bf_read(
                ((&raw mut gBattleResults).cast::<u8>()).wrapping_add(5),
                1,
                1,
                false,
            ) as u8)
                != 0
            {
                balls = (balls).wrapping_add(1);
            }
            ((show).wrapping_add(4))
                .write((((&raw mut gMapHeader).cast::<u8>()).wrapping_add(20)).read());
            StringCopy(
                ((show).wrapping_add(19)).cast::<u8>(),
                (((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).cast::<u8>(),
            );
            ((show).wrapping_add(10).cast::<u16>()).write(
                (((&raw mut gBattleResults).cast::<u8>())
                    .wrapping_add(6)
                    .cast::<u16>())
                .read(),
            );
            'l3: {
                let __sw1 = ((((&raw mut gBattleOutcome).cast::<u8>()).read()) as i32);
                if __sw1 == 2i32 || __sw1 == 3i32 {
                    (show).write(0u8);
                    return;
                }
                if __sw1 == 7i32 {
                    ((show).wrapping_add(5)).write(0u8);
                    break 'l3;
                }
                if __sw1 == 1i32 {
                    ((show).wrapping_add(5)).write(1u8);
                    break 'l3;
                }
                if __sw1 == 4i32 || __sw1 == 5i32 || __sw1 == 8i32 {
                    ((show).wrapping_add(5)).write(2u8);
                    break 'l3;
                }
                if __sw1 == 6i32 || __sw1 == 10i32 {
                    ((show).wrapping_add(5)).write(3u8);
                    break 'l3;
                }
            }
            ((show).wrapping_add(2).cast::<u16>()).write(
                (((&raw mut gBattleResults).cast::<u8>())
                    .wrapping_add(32)
                    .cast::<u16>())
                .read(),
            );
            'l4: {
                let __sw2 = ((((show).wrapping_add(5)).read()) as i32);
                if __sw2 == 0i32 {
                    if (crate::c::bf_read(
                        ((&raw mut gBattleResults).cast::<u8>()).wrapping_add(5),
                        1,
                        1,
                        false,
                    ) as u8)
                        != 0
                    {
                        ((show).wrapping_add(6).cast::<u16>()).write(1u16);
                    } else {
                        ((show).wrapping_add(6).cast::<u16>()).write(
                            ((crate::c::bf_read(
                                ((&raw mut gBattleResults).cast::<u8>()).wrapping_add(5),
                                2,
                                4,
                                false,
                            ) as u8) as u16),
                        );
                    }
                    ((show).wrapping_add(8).cast::<u16>()).write(balls);
                    break 'l4;
                }
                if __sw2 == 1i32 {
                    ((show).wrapping_add(12).cast::<u16>()).write(
                        (((&raw mut gBattleResults).cast::<u8>())
                            .wrapping_add(34)
                            .cast::<u16>())
                        .read(),
                    );
                    break 'l4;
                }
                if __sw2 == 2i32 || __sw2 == 3i32 {
                    break 'l4;
                }
            }
            StorePlayerIdInRecordMixShow(show);
            ((show).wrapping_add(14)).write(((&raw mut gGameLanguage).cast::<u8>()).read());
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn TryPutLotteryWinnerReportOnAir() {
    unsafe {
        let mut show: *mut u8 = core::ptr::null_mut();
        ((&raw mut sCurTVShowSlot).cast::<u8>().cast::<i8>()).write(
            FindFirstEmptyRecordMixTVShowSlot(
                ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(10188))
                    .cast::<u8>(),
            ),
        );
        if (((((&raw mut sCurTVShowSlot).cast::<u8>().cast::<i8>()).read()) as i32) != (-1i32))
            && (((IsRecordMixShowAlreadySpawned(32u8, 0u8)) as i32) != 1i32)
        {
            show = (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(10188))
                .cast::<u8>())
            .wrapping_offset(
                ((((&raw mut sCurTVShowSlot).cast::<u8>().cast::<i8>()).read()) as i32) as isize
                    * 36,
            );
            (show).write(32u8);
            ((show).wrapping_add(1)).write(0u8);
            StringCopy(
                ((show).wrapping_add(19)).cast::<u8>(),
                (((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).cast::<u8>(),
            );
            ((show).wrapping_add(4)).write(
                (((4i32)
                    .wrapping_sub(((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32)))
                    as u8),
            );
            ((show).wrapping_add(2).cast::<u16>())
                .write(((&raw mut gSpecialVar_0x8005).cast::<u16>()).read());
            StorePlayerIdInRecordMixShow(show);
            ((show).wrapping_add(5)).write(((&raw mut gGameLanguage).cast::<u8>()).read());
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn TryPutBattleSeminarOnAir(
    foeSpecies: u16,
    species: u16,
    moveIndex: u8,
    movePtr: *mut u16,
    betterMove: u16,
) {
    unsafe {
        let mut foeSpecies = foeSpecies;
        let mut species = species;
        let mut moveIndex = moveIndex;
        let mut movePtr = movePtr;
        let mut betterMove = betterMove;
        let mut show: *mut u8 = core::ptr::null_mut();
        let mut i: u8 = 0u8;
        let mut j: u8 = 0u8;
        ((&raw mut sCurTVShowSlot).cast::<u8>().cast::<i8>()).write(
            FindFirstEmptyRecordMixTVShowSlot(
                ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(10188))
                    .cast::<u8>(),
            ),
        );
        if (((((&raw mut sCurTVShowSlot).cast::<u8>().cast::<i8>()).read()) as i32) != (-1i32))
            && (((IsRecordMixShowAlreadySpawned(33u8, 0u8)) as i32) != 1i32)
        {
            show = (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(10188))
                .cast::<u8>())
            .wrapping_offset(
                ((((&raw mut sCurTVShowSlot).cast::<u8>().cast::<i8>()).read()) as i32) as isize
                    * 36,
            );
            (show).write(33u8);
            ((show).wrapping_add(1)).write(0u8);
            StringCopy(
                ((show).wrapping_add(19)).cast::<u8>(),
                (((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).cast::<u8>(),
            );
            ((show).wrapping_add(4).cast::<u16>()).write(foeSpecies);
            ((show).wrapping_add(6).cast::<u16>()).write(species);
            ((show).wrapping_add(2).cast::<u16>())
                .write(((movePtr).wrapping_offset(((moveIndex) as i32) as isize)).read());
            {
                i = 0u8;
                j = 0u8;
                'l1: loop {
                    if !(((i) as i32) < 4i32) {
                        break 'l1;
                    }
                    'l2: {
                        if (((i) as i32) != ((moveIndex) as i32))
                            && ((((movePtr).wrapping_offset(((i) as i32) as isize)).read()) != 0)
                        {
                            ((((show).wrapping_add(8)).cast::<u16>())
                                .wrapping_offset(((j) as i32) as isize))
                            .write(((movePtr).wrapping_offset(((i) as i32) as isize)).read());
                            j = (j).wrapping_add(1);
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
            ((show).wrapping_add(16)).write(j);
            ((show).wrapping_add(14).cast::<u16>()).write(betterMove);
            StorePlayerIdInRecordMixShow(show);
            ((show).wrapping_add(17)).write(((&raw mut gGameLanguage).cast::<u8>()).read());
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn TryPutSafariFanClubOnAir(monsCaught: u8, pokeblocksUsed: u8) {
    unsafe {
        let mut monsCaught = monsCaught;
        let mut pokeblocksUsed = pokeblocksUsed;
        let mut show: *mut u8 = core::ptr::null_mut();
        ((&raw mut sCurTVShowSlot).cast::<u8>().cast::<i8>()).write(
            FindFirstEmptyRecordMixTVShowSlot(
                ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(10188))
                    .cast::<u8>(),
            ),
        );
        if (((((&raw mut sCurTVShowSlot).cast::<u8>().cast::<i8>()).read()) as i32) != (-1i32))
            && (((IsRecordMixShowAlreadySpawned(39u8, 0u8)) as i32) != 1i32)
        {
            show = (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(10188))
                .cast::<u8>())
            .wrapping_offset(
                ((((&raw mut sCurTVShowSlot).cast::<u8>().cast::<i8>()).read()) as i32) as isize
                    * 36,
            );
            (show).write(39u8);
            ((show).wrapping_add(1)).write(0u8);
            StringCopy(
                ((show).wrapping_add(19)).cast::<u8>(),
                (((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).cast::<u8>(),
            );
            ((show).wrapping_add(2)).write(monsCaught);
            ((show).wrapping_add(3)).write(pokeblocksUsed);
            StorePlayerIdInRecordMixShow(show);
            ((show).wrapping_add(4)).write(((&raw mut gGameLanguage).cast::<u8>()).read());
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn TryPutSpotTheCutiesOnAir(pokemon: *mut u8, ribbonMonDataIdx: u8) {
    unsafe {
        let mut pokemon = pokemon;
        let mut ribbonMonDataIdx = ribbonMonDataIdx;
        let mut show: *mut u8 = core::ptr::null_mut();
        ((&raw mut sCurTVShowSlot).cast::<u8>().cast::<i8>()).write(
            FindFirstEmptyRecordMixTVShowSlot(
                ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(10188))
                    .cast::<u8>(),
            ),
        );
        if (((((&raw mut sCurTVShowSlot).cast::<u8>().cast::<i8>()).read()) as i32) != (-1i32))
            && (((IsRecordMixShowAlreadySpawned(35u8, 0u8)) as i32) != 1i32)
        {
            show = (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(10188))
                .cast::<u8>())
            .wrapping_offset(
                ((((&raw mut sCurTVShowSlot).cast::<u8>().cast::<i8>()).read()) as i32) as isize
                    * 36,
            );
            (show).write(35u8);
            ((show).wrapping_add(1)).write(0u8);
            StringCopy(
                ((show).wrapping_add(19)).cast::<u8>(),
                (((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).cast::<u8>(),
            );
            GetMonData3(pokemon, 2i32, ((show).wrapping_add(4)).cast::<u8>());
            StripExtCtrlCodes(((show).wrapping_add(4)).cast::<u8>());
            ((show).wrapping_add(2)).write(GetRibbonCount(pokemon));
            ((show).wrapping_add(3)).write(MonDataIdxToRibbon(ribbonMonDataIdx));
            StorePlayerIdInRecordMixShow(show);
            ((show).wrapping_add(15)).write(((&raw mut gGameLanguage).cast::<u8>()).read());
            if (((((show).wrapping_add(15)).read()) as i32) == 1i32)
                || (GetMonData2(pokemon, 3i32) == 1u32)
            {
                ((show).wrapping_add(16)).write(1u8);
            } else {
                ((show).wrapping_add(16)).write(((GetMonData2(pokemon, 3i32)) as u8));
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetRibbonCount(pokemon: *mut u8) -> u8 {
    unsafe {
        let mut pokemon = pokemon;
        let mut nRibbons: u8 = 0u8;
        nRibbons = 0u8;
        nRibbons = ((((nRibbons) as u32).wrapping_add(GetMonData2(pokemon, 50i32))) as u8);
        nRibbons = ((((nRibbons) as u32).wrapping_add(GetMonData2(pokemon, 51i32))) as u8);
        nRibbons = ((((nRibbons) as u32).wrapping_add(GetMonData2(pokemon, 52i32))) as u8);
        nRibbons = ((((nRibbons) as u32).wrapping_add(GetMonData2(pokemon, 53i32))) as u8);
        nRibbons = ((((nRibbons) as u32).wrapping_add(GetMonData2(pokemon, 54i32))) as u8);
        nRibbons = ((((nRibbons) as u32).wrapping_add(GetMonData2(pokemon, 67i32))) as u8);
        nRibbons = ((((nRibbons) as u32).wrapping_add(GetMonData2(pokemon, 68i32))) as u8);
        nRibbons = ((((nRibbons) as u32).wrapping_add(GetMonData2(pokemon, 69i32))) as u8);
        nRibbons = ((((nRibbons) as u32).wrapping_add(GetMonData2(pokemon, 70i32))) as u8);
        nRibbons = ((((nRibbons) as u32).wrapping_add(GetMonData2(pokemon, 71i32))) as u8);
        nRibbons = ((((nRibbons) as u32).wrapping_add(GetMonData2(pokemon, 72i32))) as u8);
        nRibbons = ((((nRibbons) as u32).wrapping_add(GetMonData2(pokemon, 73i32))) as u8);
        nRibbons = ((((nRibbons) as u32).wrapping_add(GetMonData2(pokemon, 74i32))) as u8);
        nRibbons = ((((nRibbons) as u32).wrapping_add(GetMonData2(pokemon, 75i32))) as u8);
        nRibbons = ((((nRibbons) as u32).wrapping_add(GetMonData2(pokemon, 76i32))) as u8);
        nRibbons = ((((nRibbons) as u32).wrapping_add(GetMonData2(pokemon, 77i32))) as u8);
        nRibbons = ((((nRibbons) as u32).wrapping_add(GetMonData2(pokemon, 78i32))) as u8);
        return nRibbons;
    }
}
pub(crate) unsafe extern "C" fn MonDataIdxToRibbon(monDataIdx: u8) -> u8 {
    unsafe {
        let mut monDataIdx = monDataIdx;
        if ((monDataIdx) as i32) == 67i32 {
            return 0u8;
        }
        if ((monDataIdx) as i32) == 50i32 {
            return 1u8;
        }
        if ((monDataIdx) as i32) == 51i32 {
            return 5u8;
        }
        if ((monDataIdx) as i32) == 52i32 {
            return 9u8;
        }
        if ((monDataIdx) as i32) == 53i32 {
            return 13u8;
        }
        if ((monDataIdx) as i32) == 54i32 {
            return 17u8;
        }
        if ((monDataIdx) as i32) == 68i32 {
            return 21u8;
        }
        if ((monDataIdx) as i32) == 69i32 {
            return 22u8;
        }
        if ((monDataIdx) as i32) == 70i32 {
            return 23u8;
        }
        if ((monDataIdx) as i32) == 71i32 {
            return 24u8;
        }
        if ((monDataIdx) as i32) == 72i32 {
            return 25u8;
        }
        if ((monDataIdx) as i32) == 73i32 {
            return 26u8;
        }
        if ((monDataIdx) as i32) == 74i32 {
            return 27u8;
        }
        if ((monDataIdx) as i32) == 75i32 {
            return 28u8;
        }
        if ((monDataIdx) as i32) == 76i32 {
            return 29u8;
        }
        if ((monDataIdx) as i32) == 77i32 {
            return 30u8;
        }
        if ((monDataIdx) as i32) == 78i32 {
            return 31u8;
        }
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn TryPutTrainerFanClubOnAir() {
    unsafe {
        let mut show: *mut u8 = core::ptr::null_mut();
        ((&raw mut sCurTVShowSlot).cast::<u8>().cast::<i8>()).write(
            FindFirstEmptyRecordMixTVShowSlot(
                ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(10188))
                    .cast::<u8>(),
            ),
        );
        if (((((&raw mut sCurTVShowSlot).cast::<u8>().cast::<i8>()).read()) as i32) != (-1i32))
            && (((IsRecordMixShowAlreadySpawned(34u8, 0u8)) as i32) != 1i32)
        {
            show = (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(10188))
                .cast::<u8>())
            .wrapping_offset(
                ((((&raw mut sCurTVShowSlot).cast::<u8>().cast::<i8>()).read()) as i32) as isize
                    * 36,
            );
            (show).write(34u8);
            ((show).wrapping_add(1)).write(0u8);
            StringCopy(
                ((show).wrapping_add(19)).cast::<u8>(),
                (((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).cast::<u8>(),
            );
            (((show).wrapping_add(4)).cast::<u16>()).write(
                (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(11184))
                    .cast::<u16>())
                .read(),
            );
            ((((show).wrapping_add(4)).cast::<u16>()).wrapping_offset(1)).write(
                ((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(11184))
                    .cast::<u16>())
                .wrapping_offset(1))
                .read(),
            );
            StorePlayerIdInRecordMixShow(show);
            ((show).wrapping_add(8)).write(((&raw mut gGameLanguage).cast::<u8>()).read());
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ShouldHideFanClubInterviewer() -> u8 {
    unsafe {
        ((&raw mut sCurTVShowSlot).cast::<u8>().cast::<i8>()).write(
            FindFirstEmptyNormalTVShowSlot(
                ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(10188))
                    .cast::<u8>(),
            ),
        );
        if ((((&raw mut sCurTVShowSlot).cast::<u8>().cast::<i8>()).read()) as i32) == (-1i32) {
            return 1u8;
        }
        TryReplaceOldTVShowOfKind(11u8);
        if ((((&raw mut gSpecialVar_Result).cast::<u16>()).read()) as i32) == 1i32 {
            return 1u8;
        }
        if ((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(12624))
            .cast::<u8>())
        .cast::<u8>())
        .read()) as i32)
            == 255i32
        {
            return 1u8;
        }
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ShouldAirFrontierTVShow() -> u8 {
    unsafe {
        let mut playerId: u32 = 0u32;
        let mut showIdx: u8 = 0u8;
        let mut shows: *mut u8 = core::ptr::null_mut();
        if ((IsRecordMixShowAlreadySpawned(36u8, 0u8)) as i32) == 1i32 {
            shows = ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(10188))
                .cast::<u8>();
            playerId = GetPlayerIDAsU32();
            {
                showIdx = 5u8;
                'l1: loop {
                    if !(((showIdx) as i32) < 24i32) {
                        break 'l1;
                    }
                    'l2: {
                        if ((((((shows).wrapping_offset(((showIdx) as i32) as isize * 36)).read())
                            as i32)
                            == 36i32)
                            && ((playerId & 255u32)
                                == (((((shows).wrapping_offset(((showIdx) as i32) as isize * 36))
                                    .wrapping_add(34))
                                .read()) as u32)))
                            && (((playerId >> 8) & 255u32)
                                == (((((shows).wrapping_offset(((showIdx) as i32) as isize * 36))
                                    .wrapping_add(35))
                                .read()) as u32))
                        {
                            DeleteTVShowInArrayByIdx(
                                ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                                    .wrapping_add(10188))
                                .cast::<u8>(),
                                showIdx,
                            );
                            CompactTVShowArray(
                                ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                                    .wrapping_add(10188))
                                .cast::<u8>(),
                            );
                            return 1u8;
                        }
                    }
                    showIdx = (showIdx).wrapping_add(1);
                }
            }
        }
        ((&raw mut sCurTVShowSlot).cast::<u8>().cast::<i8>()).write(
            FindFirstEmptyRecordMixTVShowSlot(
                ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(10188))
                    .cast::<u8>(),
            ),
        );
        if ((((&raw mut sCurTVShowSlot).cast::<u8>().cast::<i8>()).read()) as i32) == (-1i32) {
            return 0u8;
        }
        return 1u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn TryPutFrontierTVShowOnAir(winStreak: u16, facilityAndMode: u8) {
    unsafe {
        let mut winStreak = winStreak;
        let mut facilityAndMode = facilityAndMode;
        let mut show: *mut u8 = core::ptr::null_mut();
        ((&raw mut sCurTVShowSlot).cast::<u8>().cast::<i8>()).write(
            FindFirstEmptyRecordMixTVShowSlot(
                ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(10188))
                    .cast::<u8>(),
            ),
        );
        if ((((&raw mut sCurTVShowSlot).cast::<u8>().cast::<i8>()).read()) as i32) != (-1i32) {
            show = (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(10188))
                .cast::<u8>())
            .wrapping_offset(
                ((((&raw mut sCurTVShowSlot).cast::<u8>().cast::<i8>()).read()) as i32) as isize
                    * 36,
            );
            (show).write(36u8);
            ((show).wrapping_add(1)).write(0u8);
            StringCopy(
                ((show).wrapping_add(19)).cast::<u8>(),
                (((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).cast::<u8>(),
            );
            ((show).wrapping_add(2).cast::<u16>()).write(winStreak);
            ((show).wrapping_add(13)).write(facilityAndMode);
            'l1: {
                let __sw1 = ((facilityAndMode) as i32);
                if __sw1 == 1i32
                    || __sw1 == 5i32
                    || __sw1 == 6i32
                    || __sw1 == 7i32
                    || __sw1 == 8i32
                    || __sw1 == 9i32
                    || __sw1 == 10i32
                    || __sw1 == 11i32
                    || __sw1 == 12i32
                    || __sw1 == 13i32
                {
                    ((show).wrapping_add(4).cast::<u16>()).write(
                        ((GetMonData3(
                            (&raw mut gPlayerParty).cast::<u8>(),
                            11i32,
                            core::ptr::null_mut(),
                        )) as u16),
                    );
                    ((show).wrapping_add(6).cast::<u16>()).write(
                        ((GetMonData3(
                            ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(100),
                            11i32,
                            core::ptr::null_mut(),
                        )) as u16),
                    );
                    ((show).wrapping_add(8).cast::<u16>()).write(
                        ((GetMonData3(
                            ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(200),
                            11i32,
                            core::ptr::null_mut(),
                        )) as u16),
                    );
                    break 'l1;
                }
                if __sw1 == 2i32 {
                    ((show).wrapping_add(4).cast::<u16>()).write(
                        ((GetMonData3(
                            (&raw mut gPlayerParty).cast::<u8>(),
                            11i32,
                            core::ptr::null_mut(),
                        )) as u16),
                    );
                    ((show).wrapping_add(6).cast::<u16>()).write(
                        ((GetMonData3(
                            ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(100),
                            11i32,
                            core::ptr::null_mut(),
                        )) as u16),
                    );
                    ((show).wrapping_add(8).cast::<u16>()).write(
                        ((GetMonData3(
                            ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(200),
                            11i32,
                            core::ptr::null_mut(),
                        )) as u16),
                    );
                    ((show).wrapping_add(10).cast::<u16>()).write(
                        ((GetMonData3(
                            ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(300),
                            11i32,
                            core::ptr::null_mut(),
                        )) as u16),
                    );
                    break 'l1;
                }
                if __sw1 == 3i32 {
                    ((show).wrapping_add(4).cast::<u16>()).write(
                        ((GetMonData3(
                            (&raw mut gPlayerParty).cast::<u8>(),
                            11i32,
                            core::ptr::null_mut(),
                        )) as u16),
                    );
                    ((show).wrapping_add(6).cast::<u16>()).write(
                        ((GetMonData3(
                            ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(100),
                            11i32,
                            core::ptr::null_mut(),
                        )) as u16),
                    );
                    break 'l1;
                }
                if __sw1 == 4i32 {
                    ((show).wrapping_add(4).cast::<u16>()).write(
                        ((GetMonData3(
                            (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(568))
                            .cast::<u8>())
                            .wrapping_offset(
                                (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                    .wrapping_add(1612))
                                .wrapping_add(1630))
                                .cast::<u16>())
                                .read()) as i32)
                                    .wrapping_sub(1i32)) as isize
                                    * 100,
                            ),
                            11i32,
                            core::ptr::null_mut(),
                        )) as u16),
                    );
                    ((show).wrapping_add(6).cast::<u16>()).write(
                        ((GetMonData3(
                            (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(568))
                            .cast::<u8>())
                            .wrapping_offset(
                                ((((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                    .wrapping_add(1612))
                                .wrapping_add(1630))
                                .cast::<u16>())
                                .wrapping_offset(1))
                                .read()) as i32)
                                    .wrapping_sub(1i32)) as isize
                                    * 100,
                            ),
                            11i32,
                            core::ptr::null_mut(),
                        )) as u16),
                    );
                    break 'l1;
                }
            }
            StorePlayerIdInRecordMixShow(show);
            ((show).wrapping_add(12)).write(((&raw mut gGameLanguage).cast::<u8>()).read());
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn TryPutSecretBaseSecretsOnAir() {
    unsafe {
        let mut show: *mut u8 = core::ptr::null_mut();
        let mut strbuf = crate::ffi::Align4([0u8; 32]);
        if ((IsRecordMixShowAlreadySpawned(38u8, 0u8)) as i32) != 1i32 {
            ((&raw mut sCurTVShowSlot).cast::<u8>().cast::<i8>()).write(
                FindFirstEmptyRecordMixTVShowSlot(
                    ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(10188))
                        .cast::<u8>(),
                ),
            );
            if ((((&raw mut sCurTVShowSlot).cast::<u8>().cast::<i8>()).read()) as i32) != (-1i32) {
                show = (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                    .wrapping_add(10188))
                .cast::<u8>())
                .wrapping_offset(
                    ((((&raw mut sCurTVShowSlot).cast::<u8>().cast::<i8>()).read()) as i32)
                        as isize
                        * 36,
                );
                (show).write(38u8);
                ((show).wrapping_add(1)).write(0u8);
                StringCopy(
                    ((show).wrapping_add(19)).cast::<u8>(),
                    (((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).cast::<u8>(),
                );
                ((show).wrapping_add(2).cast::<u16>()).write(VarGet(16620u16));
                CopyCurSecretBaseOwnerName_StrVar1();
                StringCopy(
                    (&raw mut strbuf).cast::<u8>(),
                    (&raw mut gStringVar1).cast::<u8>(),
                );
                StripExtCtrlCodes((&raw mut strbuf).cast::<u8>());
                StringCopy(
                    ((show).wrapping_add(4)).cast::<u8>(),
                    (&raw mut strbuf).cast::<u8>(),
                );
                ((show).wrapping_add(16).cast::<u16>()).write(VarGet(16621u16));
                ((show).wrapping_add(12).cast::<u32>()).write(
                    ((((VarGet(16622u16)) as i32).wrapping_add((((VarGet(16623u16)) as i32) << 16)))
                        as u32),
                );
                StorePlayerIdInRecordMixShow(show);
                ((show).wrapping_add(27)).write(((&raw mut gGameLanguage).cast::<u8>()).read());
                if (((((show).wrapping_add(27)).read()) as i32) == 1i32)
                    || ((((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(6812))
                    .cast::<u8>())
                    .wrapping_offset(((VarGet(16468u16)) as i32) as isize * 160))
                    .wrapping_add(13))
                    .read()) as i32)
                        == 1i32)
                {
                    ((show).wrapping_add(28)).write(1u8);
                } else {
                    ((show).wrapping_add(28)).write(
                        (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(6812))
                        .cast::<u8>())
                        .wrapping_offset(((VarGet(16468u16)) as i32) as isize * 160))
                        .wrapping_add(13))
                        .read(),
                    );
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn ResolveNumberOneShow(days: u16) {
    unsafe {
        let mut days = days;
        let mut i: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as u32) < crate::c::div_u32(28u32, 4u32)) {
                    break 'l1;
                }
                'l2: {
                    if ((VarGet(
                        (((((&raw const sNumberOneVarsAndThresholds)
                            .cast::<u8>()
                            .cast_mut())
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 4))
                        .cast::<u16>())
                        .read(),
                    )) as i32)
                        >= ((((((((&raw const sNumberOneVarsAndThresholds)
                            .cast::<u8>()
                            .cast_mut())
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 4))
                        .cast::<u16>())
                        .wrapping_offset(1))
                        .read()) as i32)
                    {
                        TryPutNumberOneOnAir(i);
                        break 'l1;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        {
            i = 0u8;
            'l3: loop {
                if !(((i) as u32) < crate::c::div_u32(28u32, 4u32)) {
                    break 'l3;
                }
                'l4: {
                    VarSet(
                        (((((&raw const sNumberOneVarsAndThresholds)
                            .cast::<u8>()
                            .cast_mut())
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 4))
                        .cast::<u16>())
                        .read(),
                        0u16,
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn TryPutNumberOneOnAir(actionIdx: u8) {
    unsafe {
        let mut actionIdx = actionIdx;
        let mut show: *mut u8 = core::ptr::null_mut();
        IsRecordMixShowAlreadySpawned(37u8, 1u8);
        ((&raw mut sCurTVShowSlot).cast::<u8>().cast::<i8>()).write(
            FindFirstEmptyRecordMixTVShowSlot(
                ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(10188))
                    .cast::<u8>(),
            ),
        );
        if ((((&raw mut sCurTVShowSlot).cast::<u8>().cast::<i8>()).read()) as i32) != (-1i32) {
            show = (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(10188))
                .cast::<u8>())
            .wrapping_offset(
                ((((&raw mut sCurTVShowSlot).cast::<u8>().cast::<i8>()).read()) as i32) as isize
                    * 36,
            );
            (show).write(37u8);
            ((show).wrapping_add(1)).write(0u8);
            StringCopy(
                ((show).wrapping_add(19)).cast::<u8>(),
                (((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).cast::<u8>(),
            );
            ((show).wrapping_add(4)).write(actionIdx);
            ((show).wrapping_add(2).cast::<u16>()).write(VarGet(
                (((((&raw const sNumberOneVarsAndThresholds)
                    .cast::<u8>()
                    .cast_mut())
                .cast::<u8>())
                .wrapping_offset(((actionIdx) as i32) as isize * 4))
                .cast::<u16>())
                .read(),
            ));
            StorePlayerIdInRecordMixShow(show);
            ((show).wrapping_add(5)).write(((&raw mut gGameLanguage).cast::<u8>()).read());
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IncrementDailySlotsUses() {
    unsafe {
        VarSet(
            16614u16,
            ((((VarGet(16614u16)) as i32).wrapping_add(1i32)) as u16),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IncrementDailyRouletteUses() {
    unsafe {
        VarSet(
            16619u16,
            ((((VarGet(16619u16)) as i32).wrapping_add(1i32)) as u16),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IncrementDailyWildBattles() {
    unsafe {
        VarSet(
            16615u16,
            ((((VarGet(16615u16)) as i32).wrapping_add(1i32)) as u16),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IncrementDailyBerryBlender() {
    unsafe {
        VarSet(
            16616u16,
            ((((VarGet(16616u16)) as i32).wrapping_add(1i32)) as u16),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IncrementDailyPlantedBerries() {
    unsafe {
        VarSet(
            16617u16,
            ((((VarGet(16617u16)) as i32).wrapping_add(1i32)) as u16),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IncrementDailyPickedBerries() {
    unsafe {
        VarSet(
            16618u16,
            ((((VarGet(16618u16)) as i32)
                .wrapping_add(((((&raw mut gSpecialVar_0x8006).cast::<u16>()).read()) as i32)))
                as u16),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IncrementDailyBattlePoints(delta: u16) {
    unsafe {
        let mut delta = delta;
        VarSet(
            16625u16,
            ((((VarGet(16625u16)) as i32).wrapping_add(((delta) as i32))) as u16),
        );
    }
}
pub(crate) unsafe extern "C" fn TryPutRandomPokeNewsOnAir() {
    unsafe {
        if (FlagGet(2148u16)) != 0 {
            ((&raw mut sCurTVShowSlot).cast::<u8>().cast::<i8>()).write(GetFirstEmptyPokeNewsSlot(
                ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(11088))
                    .cast::<u8>(),
            ));
            if (((((&raw mut sCurTVShowSlot).cast::<u8>().cast::<i8>()).read()) as i32) != (-1i32))
                && (((BernoulliTrial(((crate::c::div_i32(65535i32, 100i32)) as u16))) as i32)
                    != 1i32)
            {
                let mut newsKind: u8 =
                    (((crate::c::rem_i32(((Random()) as i32), 4i32)).wrapping_add(1i32)) as u8);
                if ((IsAddingPokeNewsDisallowed(newsKind)) as i32) != 1i32 {
                    ((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(11088))
                    .cast::<u8>())
                    .wrapping_offset(
                        ((((&raw mut sCurTVShowSlot).cast::<u8>().cast::<i8>()).read()) as i32)
                            as isize
                            * 4,
                    ))
                    .write(newsKind);
                    (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(11088))
                    .cast::<u8>())
                    .wrapping_offset(
                        ((((&raw mut sCurTVShowSlot).cast::<u8>().cast::<i8>()).read()) as i32)
                            as isize
                            * 4,
                    ))
                    .wrapping_add(2)
                    .cast::<u16>())
                    .write(4u16);
                    (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(11088))
                    .cast::<u8>())
                    .wrapping_offset(
                        ((((&raw mut sCurTVShowSlot).cast::<u8>().cast::<i8>()).read()) as i32)
                            as isize
                            * 4,
                    ))
                    .wrapping_add(1))
                    .write(1u8);
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn GetFirstEmptyPokeNewsSlot(pokeNews: *mut u8) -> i8 {
    unsafe {
        let mut pokeNews = pokeNews;
        let mut i: i8 = 0i8;
        {
            i = 0i8;
            'l1: loop {
                if !(((i) as i32) < 16i32) {
                    break 'l1;
                }
                'l2: {
                    if ((((pokeNews).wrapping_offset(((i) as i32) as isize * 4)).read()) as i32)
                        == 0i32
                    {
                        return i;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return (-1i8);
    }
}
pub(crate) unsafe extern "C" fn ClearPokeNews() {
    unsafe {
        let mut i: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 16i32) {
                    break 'l1;
                }
                'l2: {
                    ClearPokeNewsBySlot(i);
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn ClearPokeNewsBySlot(i: u8) {
    unsafe {
        let mut i = i;
        ((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(11088))
            .cast::<u8>())
        .wrapping_offset(((i) as i32) as isize * 4))
        .write(0u8);
        (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(11088))
            .cast::<u8>())
        .wrapping_offset(((i) as i32) as isize * 4))
        .wrapping_add(1))
        .write(0u8);
        (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(11088))
            .cast::<u8>())
        .wrapping_offset(((i) as i32) as isize * 4))
        .wrapping_add(2)
        .cast::<u16>())
        .write(0u16);
    }
}
pub(crate) unsafe extern "C" fn CompactPokeNews() {
    unsafe {
        let mut i: u8 = 0u8;
        let mut j: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 15i32) {
                    break 'l1;
                }
                'l2: {
                    if ((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(11088))
                    .cast::<u8>())
                    .wrapping_offset(((i) as i32) as isize * 4))
                    .read()) as i32)
                        == 0i32
                    {
                        {
                            j = ((((i) as i32).wrapping_add(1i32)) as u8);
                            'l3: loop {
                                if !(((j) as i32) < 16i32) {
                                    break 'l3;
                                }
                                'l4: {
                                    if ((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(11088))
                                    .cast::<u8>())
                                    .wrapping_offset(((j) as i32) as isize * 4))
                                    .read()) as i32)
                                        != 0i32
                                    {
                                        (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                                            .wrapping_add(11088))
                                        .cast::<u8>())
                                        .wrapping_offset(((i) as i32) as isize * 4)
                                        .cast::<crate::c::Rec4<4>>()
                                        .write_unaligned(
                                            (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>())
                                                .read())
                                            .wrapping_add(11088))
                                            .cast::<u8>())
                                            .wrapping_offset(((j) as i32) as isize * 4)
                                            .cast::<crate::c::Rec4<4>>()
                                            .read_unaligned(),
                                        );
                                        ClearPokeNewsBySlot(j);
                                        break 'l3;
                                    }
                                }
                                j = (j).wrapping_add(1);
                            }
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn FindAnyPokeNewsOnTheAir() -> u8 {
    unsafe {
        let mut i: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 16i32) {
                    break 'l1;
                }
                'l2: {
                    if ((((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(11088))
                    .cast::<u8>())
                    .wrapping_offset(((i) as i32) as isize * 4))
                    .read()) as i32)
                        != 0i32)
                        && ((((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(11088))
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 4))
                        .wrapping_add(1))
                        .read()) as i32)
                            == 1i32))
                        && ((((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(11088))
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 4))
                        .wrapping_add(2)
                        .cast::<u16>())
                        .read()) as i32)
                            < 3i32)
                    {
                        return i;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return 255u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DoPokeNews() {
    unsafe {
        let mut i: u8 = FindAnyPokeNewsOnTheAir();
        if ((i) as i32) == 255i32 {
            ((&raw mut gSpecialVar_Result).cast::<u16>()).write(0u16);
        } else {
            if (((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(11088))
                .cast::<u8>())
            .wrapping_offset(((i) as i32) as isize * 4))
            .wrapping_add(2)
            .cast::<u16>())
            .read()) as i32)
                == 0i32
            {
                (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(11088))
                    .cast::<u8>())
                .wrapping_offset(((i) as i32) as isize * 4))
                .wrapping_add(1))
                .write(2u8);
                if (((((&raw mut gLocalTime).cast::<u8>())
                    .wrapping_add(2)
                    .cast::<i8>())
                .read()) as i32)
                    < 20i32
                {
                    ShowFieldMessage(
                        ((((&raw const sPokeNewsTextGroup_Ongoing)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<*mut u8>())
                        .cast::<*mut u8>())
                        .wrapping_offset(
                            ((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(11088))
                            .cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 4))
                            .read()) as i32) as isize,
                        ))
                        .read(),
                    );
                } else {
                    ShowFieldMessage(
                        ((((&raw const sPokeNewsTextGroup_Ending)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<*mut u8>())
                        .cast::<*mut u8>())
                        .wrapping_offset(
                            ((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(11088))
                            .cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 4))
                            .read()) as i32) as isize,
                        ))
                        .read(),
                    );
                }
            } else {
                let mut dayCountdown: u16 = (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>())
                    .read())
                .wrapping_add(11088))
                .cast::<u8>())
                .wrapping_offset(((i) as i32) as isize * 4))
                .wrapping_add(2)
                .cast::<u16>())
                .read();
                ConvertIntToDecimalStringN(
                    (&raw mut gStringVar1).cast::<u8>(),
                    ((dayCountdown) as i32),
                    0i32,
                    1u8,
                );
                (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(11088))
                    .cast::<u8>())
                .wrapping_offset(((i) as i32) as isize * 4))
                .wrapping_add(1))
                .write(0u8);
                ShowFieldMessage(
                    ((((&raw const sPokeNewsTextGroup_Upcoming)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<*mut u8>())
                    .cast::<*mut u8>())
                    .wrapping_offset(
                        ((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(11088))
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 4))
                        .read()) as i32) as isize,
                    ))
                    .read(),
                );
            }
            ((&raw mut gSpecialVar_Result).cast::<u16>()).write(1u16);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsPokeNewsActive(newsKind: u8) -> u8 {
    unsafe {
        let mut newsKind = newsKind;
        let mut i: u8 = 0u8;
        if ((newsKind) as i32) == 0i32 {
            return 0u8;
        }
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 16i32) {
                    break 'l1;
                }
                'l2: {
                    if ((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(11088))
                    .cast::<u8>())
                    .wrapping_offset(((i) as i32) as isize * 4))
                    .read()) as i32)
                        == ((newsKind) as i32)
                    {
                        if ((((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(11088))
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 4))
                        .wrapping_add(1))
                        .read()) as i32)
                            == 2i32)
                            && ((ShouldApplyPokeNewsEffect(newsKind)) != 0)
                        {
                            return 1u8;
                        }
                        return 0u8;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn ShouldApplyPokeNewsEffect(newsKind: u8) -> u8 {
    unsafe {
        let mut newsKind = newsKind;
        'l1: {
            let __sw1 = ((newsKind) as i32);
            if __sw1 == 1i32 {
                if (((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4))
                    .cast::<i8>())
                .read()) as i32)
                    == 0i32)
                    && ((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(4))
                    .wrapping_add(1)
                    .cast::<i8>())
                    .read()) as i32)
                        == 1i32))
                    && (((((&raw mut gSpecialVar_LastTalked).cast::<u16>()).read()) as i32)
                        == 25i32)
                {
                    return 1u8;
                }
                return 0u8;
            }
            if __sw1 == 3i32 {
                if ((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4))
                    .cast::<i8>())
                .read()) as i32)
                    == 13i32)
                    && ((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(4))
                    .wrapping_add(1)
                    .cast::<i8>())
                    .read()) as i32)
                        == 21i32)
                {
                    return 1u8;
                }
                return 0u8;
            }
        }
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn IsAddingPokeNewsDisallowed(newsKind: u8) -> u8 {
    unsafe {
        let mut newsKind = newsKind;
        let mut i: u8 = 0u8;
        if ((newsKind) as i32) == 0i32 {
            return 1u8;
        }
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 16i32) {
                    break 'l1;
                }
                'l2: {
                    if ((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(11088))
                    .cast::<u8>())
                    .wrapping_offset(((i) as i32) as isize * 4))
                    .read()) as i32)
                        == ((newsKind) as i32)
                    {
                        return 1u8;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn UpdatePokeNewsCountdown(days: u16) {
    unsafe {
        let mut days = days;
        let mut i: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 16i32) {
                    break 'l1;
                }
                'l2: {
                    if ((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(11088))
                    .cast::<u8>())
                    .wrapping_offset(((i) as i32) as isize * 4))
                    .read()) as i32)
                        != 0i32
                    {
                        if (((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(11088))
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 4))
                        .wrapping_add(2)
                        .cast::<u16>())
                        .read()) as i32)
                            < ((days) as i32)
                        {
                            ClearPokeNewsBySlot(i);
                        } else {
                            if ((((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(11088))
                            .cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 4))
                            .wrapping_add(1))
                            .read()) as i32)
                                == 0i32)
                                && (((FlagGet(2148u16)) as i32) == 1i32)
                            {
                                (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                                    .wrapping_add(11088))
                                .cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize * 4))
                                .wrapping_add(1))
                                .write(1u8);
                            }
                            let __p1 = ((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(11088))
                            .cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 4))
                            .wrapping_add(2)
                            .cast::<u16>();
                            (__p1).write(
                                (((((__p1).read()) as i32).wrapping_sub(((days) as i32))) as u16),
                            );
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        CompactPokeNews();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CopyContestRankToStringVar(varIdx: u8, rank: u8) {
    unsafe {
        let mut varIdx = varIdx;
        let mut rank = rank;
        'l1: {
            let __sw1 = ((rank) as i32);
            if __sw1 == 0i32 {
                StringCopy(
                    ((((&raw const gTVStringVarPtrs)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<*mut u8>())
                    .cast::<*mut u8>())
                    .wrapping_offset(((varIdx) as i32) as isize))
                    .read(),
                    ((((&raw mut gStdStrings).cast::<*mut u8>()).cast::<*mut u8>())
                        .wrapping_offset(5))
                    .read(),
                );
                break 'l1;
            }
            if __sw1 == 1i32 {
                StringCopy(
                    ((((&raw const gTVStringVarPtrs)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<*mut u8>())
                    .cast::<*mut u8>())
                    .wrapping_offset(((varIdx) as i32) as isize))
                    .read(),
                    ((((&raw mut gStdStrings).cast::<*mut u8>()).cast::<*mut u8>())
                        .wrapping_offset(6))
                    .read(),
                );
                break 'l1;
            }
            if __sw1 == 2i32 {
                StringCopy(
                    ((((&raw const gTVStringVarPtrs)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<*mut u8>())
                    .cast::<*mut u8>())
                    .wrapping_offset(((varIdx) as i32) as isize))
                    .read(),
                    ((((&raw mut gStdStrings).cast::<*mut u8>()).cast::<*mut u8>())
                        .wrapping_offset(7))
                    .read(),
                );
                break 'l1;
            }
            if __sw1 == 3i32 {
                StringCopy(
                    ((((&raw const gTVStringVarPtrs)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<*mut u8>())
                    .cast::<*mut u8>())
                    .wrapping_offset(((varIdx) as i32) as isize))
                    .read(),
                    ((((&raw mut gStdStrings).cast::<*mut u8>()).cast::<*mut u8>())
                        .wrapping_offset(8))
                    .read(),
                );
                break 'l1;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CopyContestCategoryToStringVar(varIdx: u8, category: u8) {
    unsafe {
        let mut varIdx = varIdx;
        let mut category = category;
        'l1: {
            let __sw1 = ((category) as i32);
            if __sw1 == 0i32 {
                StringCopy(
                    ((((&raw const gTVStringVarPtrs)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<*mut u8>())
                    .cast::<*mut u8>())
                    .wrapping_offset(((varIdx) as i32) as isize))
                    .read(),
                    (((&raw mut gStdStrings).cast::<*mut u8>()).cast::<*mut u8>()).read(),
                );
                break 'l1;
            }
            if __sw1 == 1i32 {
                StringCopy(
                    ((((&raw const gTVStringVarPtrs)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<*mut u8>())
                    .cast::<*mut u8>())
                    .wrapping_offset(((varIdx) as i32) as isize))
                    .read(),
                    ((((&raw mut gStdStrings).cast::<*mut u8>()).cast::<*mut u8>())
                        .wrapping_offset(1))
                    .read(),
                );
                break 'l1;
            }
            if __sw1 == 2i32 {
                StringCopy(
                    ((((&raw const gTVStringVarPtrs)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<*mut u8>())
                    .cast::<*mut u8>())
                    .wrapping_offset(((varIdx) as i32) as isize))
                    .read(),
                    ((((&raw mut gStdStrings).cast::<*mut u8>()).cast::<*mut u8>())
                        .wrapping_offset(2))
                    .read(),
                );
                break 'l1;
            }
            if __sw1 == 3i32 {
                StringCopy(
                    ((((&raw const gTVStringVarPtrs)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<*mut u8>())
                    .cast::<*mut u8>())
                    .wrapping_offset(((varIdx) as i32) as isize))
                    .read(),
                    ((((&raw mut gStdStrings).cast::<*mut u8>()).cast::<*mut u8>())
                        .wrapping_offset(3))
                    .read(),
                );
                break 'l1;
            }
            if __sw1 == 4i32 {
                StringCopy(
                    ((((&raw const gTVStringVarPtrs)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<*mut u8>())
                    .cast::<*mut u8>())
                    .wrapping_offset(((varIdx) as i32) as isize))
                    .read(),
                    ((((&raw mut gStdStrings).cast::<*mut u8>()).cast::<*mut u8>())
                        .wrapping_offset(4))
                    .read(),
                );
                break 'l1;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetContestCategoryStringVarForInterview() {
    unsafe {
        let mut show: *mut u8 = (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
            .wrapping_add(10188))
        .cast::<u8>())
        .wrapping_offset(
            ((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32) as isize * 36,
        );
        CopyContestCategoryToStringVar(
            1u8,
            (crate::c::bf_read((show).wrapping_add(19), 0, 3, false) as u8),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ConvertIntToDecimalString(varIdx: u8, value: i32) {
    unsafe {
        let mut varIdx = varIdx;
        let mut value = value;
        let mut nDigits: i32 = ((CountDigits(value)) as i32);
        ConvertIntToDecimalStringN(
            ((((&raw const gTVStringVarPtrs)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u8>())
            .cast::<*mut u8>())
            .wrapping_offset(((varIdx) as i32) as isize))
            .read(),
            value,
            0i32,
            ((nDigits) as u8),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CountDigits(value: i32) -> u32 {
    unsafe {
        let mut value = value;
        if crate::c::div_i32(value, 10i32) == 0i32 {
            return 1u32;
        }
        if crate::c::div_i32(value, 100i32) == 0i32 {
            return 2u32;
        }
        if crate::c::div_i32(value, 1000i32) == 0i32 {
            return 3u32;
        }
        if crate::c::div_i32(value, 10000i32) == 0i32 {
            return 4u32;
        }
        if crate::c::div_i32(value, 100000i32) == 0i32 {
            return 5u32;
        }
        if crate::c::div_i32(value, 1000000i32) == 0i32 {
            return 6u32;
        }
        if crate::c::div_i32(value, 10000000i32) == 0i32 {
            return 7u32;
        }
        if crate::c::div_i32(value, 100000000i32) == 0i32 {
            return 8u32;
        }
        return 1u32;
    }
}
pub(crate) unsafe extern "C" fn SmartShopper_BufferPurchaseTotal(varIdx: u8, show: *mut u8) {
    unsafe {
        let mut varIdx = varIdx;
        let mut show = show;
        let mut i: u8 = 0u8;
        let mut price: i32 = 0i32;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 3i32) {
                    break 'l1;
                }
                'l2: {
                    if ((((((show).wrapping_add(6)).cast::<u16>())
                        .wrapping_offset(((i) as i32) as isize))
                    .read()) as i32)
                        != 0i32
                    {
                        price = (price).wrapping_add(
                            ((GetItemPrice(
                                ((((show).wrapping_add(6)).cast::<u16>())
                                    .wrapping_offset(((i) as i32) as isize))
                                .read(),
                            )) as i32)
                                .wrapping_mul(
                                    ((((((show).wrapping_add(12)).cast::<u16>())
                                        .wrapping_offset(((i) as i32) as isize))
                                    .read()) as i32),
                                ),
                        );
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        if ((((show).wrapping_add(2)).read()) as i32) == 1i32 {
            ConvertIntToDecimalString(varIdx, (price >> 1));
        } else {
            ConvertIntToDecimalString(varIdx, price);
        }
    }
}
pub(crate) unsafe extern "C" fn IsRecordMixShowAlreadySpawned(kind: u8, delete: u8) -> u8 {
    unsafe {
        let mut kind = kind;
        let mut delete = delete;
        let mut playerId: u32 = 0u32;
        let mut shows: *mut u8 = core::ptr::null_mut();
        let mut i: u8 = 0u8;
        shows = ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(10188))
            .cast::<u8>();
        playerId = GetPlayerIDAsU32();
        {
            i = 5u8;
            'l1: loop {
                if !(((i) as i32) < 24i32) {
                    break 'l1;
                }
                'l2: {
                    if ((((((shows).wrapping_offset(((i) as i32) as isize * 36)).read()) as i32)
                        == ((kind) as i32))
                        && ((playerId & 255u32)
                            == (((((shows).wrapping_offset(((i) as i32) as isize * 36))
                                .wrapping_add(34))
                            .read()) as u32)))
                        && (((playerId >> 8) & 255u32)
                            == (((((shows).wrapping_offset(((i) as i32) as isize * 36))
                                .wrapping_add(35))
                            .read()) as u32))
                    {
                        if ((delete) as i32) == 1i32 {
                            DeleteTVShowInArrayByIdx(
                                ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                                    .wrapping_add(10188))
                                .cast::<u8>(),
                                i,
                            );
                            CompactTVShowArray(
                                ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                                    .wrapping_add(10188))
                                .cast::<u8>(),
                            );
                        }
                        return 1u8;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn SortPurchasesByQuantity() {
    unsafe {
        let mut i: u8 = 0u8;
        let mut j: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 2i32) {
                    break 'l1;
                }
                'l2: {
                    {
                        j = ((((i) as i32).wrapping_add(1i32)) as u8);
                        'l3: loop {
                            if !(((j) as i32) < 3i32) {
                                break 'l3;
                            }
                            'l4: {
                                if ((((((&raw mut gMartPurchaseHistory).cast::<u8>())
                                    .wrapping_offset(((i) as i32) as isize * 4))
                                .wrapping_add(2)
                                .cast::<u16>())
                                .read()) as i32)
                                    < ((((((&raw mut gMartPurchaseHistory).cast::<u8>())
                                        .wrapping_offset(((j) as i32) as isize * 4))
                                    .wrapping_add(2)
                                    .cast::<u16>())
                                    .read()) as i32)
                                {
                                    let mut tempItemId: u16 = ((((&raw mut gMartPurchaseHistory)
                                        .cast::<u8>())
                                    .wrapping_offset(((i) as i32) as isize * 4))
                                    .cast::<u16>())
                                    .read();
                                    let mut tempQuantity: u16 =
                                        ((((&raw mut gMartPurchaseHistory).cast::<u8>())
                                            .wrapping_offset(((i) as i32) as isize * 4))
                                        .wrapping_add(2)
                                        .cast::<u16>())
                                        .read();
                                    ((((&raw mut gMartPurchaseHistory).cast::<u8>())
                                        .wrapping_offset(((i) as i32) as isize * 4))
                                    .cast::<u16>())
                                    .write(
                                        ((((&raw mut gMartPurchaseHistory).cast::<u8>())
                                            .wrapping_offset(((j) as i32) as isize * 4))
                                        .cast::<u16>())
                                        .read(),
                                    );
                                    ((((&raw mut gMartPurchaseHistory).cast::<u8>())
                                        .wrapping_offset(((i) as i32) as isize * 4))
                                    .wrapping_add(2)
                                    .cast::<u16>())
                                    .write(
                                        ((((&raw mut gMartPurchaseHistory).cast::<u8>())
                                            .wrapping_offset(((j) as i32) as isize * 4))
                                        .wrapping_add(2)
                                        .cast::<u16>())
                                        .read(),
                                    );
                                    ((((&raw mut gMartPurchaseHistory).cast::<u8>())
                                        .wrapping_offset(((j) as i32) as isize * 4))
                                    .cast::<u16>())
                                    .write(tempItemId);
                                    ((((&raw mut gMartPurchaseHistory).cast::<u8>())
                                        .wrapping_offset(((j) as i32) as isize * 4))
                                    .wrapping_add(2)
                                    .cast::<u16>())
                                    .write(tempQuantity);
                                }
                            }
                            j = (j).wrapping_add(1);
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn TryReplaceOldTVShowOfKind(kind: u8) {
    unsafe {
        let mut kind = kind;
        let mut i: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 5i32) {
                    break 'l1;
                }
                'l2: {
                    if ((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(10188))
                    .cast::<u8>())
                    .wrapping_offset(((i) as i32) as isize * 36))
                    .read()) as i32)
                        == ((kind) as i32)
                    {
                        if (((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(10188))
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 36))
                        .wrapping_add(1))
                        .read()) as i32)
                            == 1i32
                        {
                            ((&raw mut gSpecialVar_Result).cast::<u16>()).write(1u16);
                        } else {
                            DeleteTVShowInArrayByIdx(
                                ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                                    .wrapping_add(10188))
                                .cast::<u8>(),
                                i,
                            );
                            CompactTVShowArray(
                                ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                                    .wrapping_add(10188))
                                .cast::<u8>(),
                            );
                            Script_FindFirstEmptyNormalTVShowSlot();
                        }
                        return;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        Script_FindFirstEmptyNormalTVShowSlot();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn InterviewBefore() {
    unsafe {
        ((&raw mut gSpecialVar_Result).cast::<u16>()).write(0u16);
        'l1: {
            let __sw1 = ((((&raw mut gSpecialVar_0x8005).cast::<u16>()).read()) as i32);
            if __sw1 == 1i32 {
                InterviewBefore_FanClubLetter();
                break 'l1;
            }
            if __sw1 == 2i32 {
                InterviewBefore_RecentHappenings();
                break 'l1;
            }
            if __sw1 == 3i32 {
                InterviewBefore_PkmnFanClubOpinions();
                break 'l1;
            }
            if __sw1 == 4i32 {
                InterviewBefore_Dummy();
                break 'l1;
            }
            if __sw1 == 5i32 {
                InterviewBefore_NameRater();
                break 'l1;
            }
            if __sw1 == 6i32 {
                InterviewBefore_BravoTrainerPkmnProfile();
                break 'l1;
            }
            if __sw1 == 7i32 {
                InterviewBefore_BravoTrainerBTProfile();
                break 'l1;
            }
            if __sw1 == 8i32 {
                InterviewBefore_ContestLiveUpdates();
                break 'l1;
            }
            if __sw1 == 9i32 {
                InterviewBefore_3CheersForPokeblocks();
                break 'l1;
            }
            if __sw1 == 11i32 {
                InterviewBefore_FanClubSpecial();
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn InterviewBefore_FanClubLetter() {
    unsafe {
        TryReplaceOldTVShowOfKind(1u8);
        if !((((&raw mut gSpecialVar_Result).cast::<u16>()).read()) != 0) {
            StringCopy(
                (&raw mut gStringVar1).cast::<u8>(),
                (((&raw mut gSpeciesNames).cast::<u8>()).wrapping_offset(
                    ((GetMonData3(
                        ((&raw mut gPlayerParty).cast::<u8>())
                            .wrapping_offset(((GetLeadMonIndex()) as i32) as isize * 100),
                        11i32,
                        core::ptr::null_mut(),
                    )) as i32) as isize
                        * 11,
                ))
                .cast::<u8>(),
            );
            InitializeEasyChatWordArray(
                (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(10188))
                    .cast::<u8>())
                .wrapping_offset(
                    ((((&raw mut sCurTVShowSlot).cast::<u8>().cast::<i8>()).read()) as i32)
                        as isize
                        * 36,
                ))
                .wrapping_add(4))
                .cast::<u16>(),
                ((crate::c::div_u32(12u32, 2u32)) as u16),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn InterviewBefore_RecentHappenings() {
    unsafe {
        TryReplaceOldTVShowOfKind(2u8);
        if !((((&raw mut gSpecialVar_Result).cast::<u16>()).read()) != 0) {
            InitializeEasyChatWordArray(
                (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(10188))
                    .cast::<u8>())
                .wrapping_offset(
                    ((((&raw mut sCurTVShowSlot).cast::<u8>().cast::<i8>()).read()) as i32)
                        as isize
                        * 36,
                ))
                .wrapping_add(4))
                .cast::<u16>(),
                ((crate::c::div_u32(12u32, 2u32)) as u16),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn InterviewBefore_PkmnFanClubOpinions() {
    unsafe {
        TryReplaceOldTVShowOfKind(3u8);
        if !((((&raw mut gSpecialVar_Result).cast::<u16>()).read()) != 0) {
            StringCopy(
                (&raw mut gStringVar1).cast::<u8>(),
                (((&raw mut gSpeciesNames).cast::<u8>()).wrapping_offset(
                    ((GetMonData3(
                        ((&raw mut gPlayerParty).cast::<u8>())
                            .wrapping_offset(((GetLeadMonIndex()) as i32) as isize * 100),
                        11i32,
                        core::ptr::null_mut(),
                    )) as i32) as isize
                        * 11,
                ))
                .cast::<u8>(),
            );
            GetMonData3(
                ((&raw mut gPlayerParty).cast::<u8>())
                    .wrapping_offset(((GetLeadMonIndex()) as i32) as isize * 100),
                2i32,
                (&raw mut gStringVar2).cast::<u8>(),
            );
            StringGet_Nickname((&raw mut gStringVar2).cast::<u8>());
            InitializeEasyChatWordArray(
                (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(10188))
                    .cast::<u8>())
                .wrapping_offset(
                    ((((&raw mut sCurTVShowSlot).cast::<u8>().cast::<i8>()).read()) as i32)
                        as isize
                        * 36,
                ))
                .wrapping_add(28))
                .cast::<u16>(),
                ((crate::c::div_u32(4u32, 2u32)) as u16),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn InterviewBefore_Dummy() {
    unsafe {
        ((&raw mut gSpecialVar_Result).cast::<u16>()).write(1u16);
    }
}
pub(crate) unsafe extern "C" fn InterviewBefore_NameRater() {
    unsafe {
        TryReplaceOldTVShowOfKind(5u8);
    }
}
pub(crate) unsafe extern "C" fn InterviewBefore_BravoTrainerPkmnProfile() {
    unsafe {
        TryReplaceOldTVShowOfKind(6u8);
        if !((((&raw mut gSpecialVar_Result).cast::<u16>()).read()) != 0) {
            InitializeEasyChatWordArray(
                (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(10188))
                    .cast::<u8>())
                .wrapping_offset(
                    ((((&raw mut sCurTVShowSlot).cast::<u8>().cast::<i8>()).read()) as i32)
                        as isize
                        * 36,
                ))
                .wrapping_add(4))
                .cast::<u16>(),
                ((crate::c::div_u32(4u32, 2u32)) as u16),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn InterviewBefore_ContestLiveUpdates() {
    unsafe {
        TryReplaceOldTVShowOfKind(8u8);
    }
}
pub(crate) unsafe extern "C" fn InterviewBefore_3CheersForPokeblocks() {
    unsafe {
        TryReplaceOldTVShowOfKind(9u8);
    }
}
pub(crate) unsafe extern "C" fn InterviewBefore_BravoTrainerBTProfile() {
    unsafe {
        TryReplaceOldTVShowOfKind(7u8);
        if !((((&raw mut gSpecialVar_Result).cast::<u16>()).read()) != 0) {
            InitializeEasyChatWordArray(
                (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(10188))
                    .cast::<u8>())
                .wrapping_offset(
                    ((((&raw mut sCurTVShowSlot).cast::<u8>().cast::<i8>()).read()) as i32)
                        as isize
                        * 36,
                ))
                .wrapping_add(24))
                .cast::<u16>(),
                ((crate::c::div_u32(2u32, 2u32)) as u16),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn InterviewBefore_FanClubSpecial() {
    unsafe {
        TryReplaceOldTVShowOfKind(11u8);
        if !((((&raw mut gSpecialVar_Result).cast::<u16>()).read()) != 0) {
            InitializeEasyChatWordArray(
                (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(10188))
                    .cast::<u8>())
                .wrapping_offset(
                    ((((&raw mut sCurTVShowSlot).cast::<u8>().cast::<i8>()).read()) as i32)
                        as isize
                        * 36,
                ))
                .wrapping_add(20))
                .cast::<u16>(),
                ((crate::c::div_u32(2u32, 2u32)) as u16),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn IsPartyMonNicknamedOrNotEnglish(monIdx: u8) -> u8 {
    unsafe {
        let mut monIdx = monIdx;
        let mut pokemon: *mut u8 = core::ptr::null_mut();
        let mut language: u8 = 0u8;
        pokemon = ((&raw mut gPlayerParty).cast::<u8>())
            .wrapping_offset(((monIdx) as i32) as isize * 100);
        GetMonData3(pokemon, 2i32, (&raw mut gStringVar1).cast::<u8>());
        language = ((GetMonData3(pokemon, 3i32, &raw mut language)) as u8);
        if (((language) as i32) == 2i32)
            && (!((StringCompare(
                (((&raw mut gSpeciesNames).cast::<u8>()).wrapping_offset(
                    ((GetMonData3(pokemon, 11i32, core::ptr::null_mut())) as i32) as isize * 11,
                ))
                .cast::<u8>(),
                (&raw mut gStringVar1).cast::<u8>(),
            )) != 0))
        {
            return 0u8;
        }
        return 1u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsLeadMonNicknamedOrNotEnglish() -> u8 {
    unsafe {
        return IsPartyMonNicknamedOrNotEnglish(GetLeadMonIndex());
    }
}
pub(crate) unsafe extern "C" fn DeleteTVShowInArrayByIdx(shows: *mut u8, idx: u8) {
    unsafe {
        let mut shows = shows;
        let mut idx = idx;
        let mut i: u8 = 0u8;
        ((shows).wrapping_offset(((idx) as i32) as isize * 36)).write(0u8);
        (((shows).wrapping_offset(((idx) as i32) as isize * 36)).wrapping_add(1)).write(0u8);
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as u32) < crate::c::div_u32(34u32, 1u32)) {
                    break 'l1;
                }
                'l2: {
                    (((((shows).wrapping_offset(((idx) as i32) as isize * 36)).wrapping_add(2))
                        .cast::<u8>())
                    .wrapping_offset(((i) as i32) as isize))
                    .write(0u8);
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn CompactTVShowArray(shows: *mut u8) {
    unsafe {
        let mut shows = shows;
        let mut i: u8 = 0u8;
        let mut j: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 4i32) {
                    break 'l1;
                }
                'l2: {
                    if ((((shows).wrapping_offset(((i) as i32) as isize * 36)).read()) as i32)
                        == 0i32
                    {
                        {
                            j = ((((i) as i32).wrapping_add(1i32)) as u8);
                            'l3: loop {
                                if !(((j) as i32) < 5i32) {
                                    break 'l3;
                                }
                                'l4: {
                                    if ((((shows).wrapping_offset(((j) as i32) as isize * 36))
                                        .read()) as i32)
                                        != 0i32
                                    {
                                        (shows)
                                            .wrapping_offset(((i) as i32) as isize * 36)
                                            .cast::<crate::c::Rec4<36>>()
                                            .write_unaligned(
                                                (shows)
                                                    .wrapping_offset(((j) as i32) as isize * 36)
                                                    .cast::<crate::c::Rec4<36>>()
                                                    .read_unaligned(),
                                            );
                                        DeleteTVShowInArrayByIdx(shows, j);
                                        break 'l3;
                                    }
                                }
                                j = (j).wrapping_add(1);
                            }
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        {
            i = 5u8;
            'l5: loop {
                if !(((i) as i32) < 24i32) {
                    break 'l5;
                }
                'l6: {
                    if ((((shows).wrapping_offset(((i) as i32) as isize * 36)).read()) as i32)
                        == 0i32
                    {
                        {
                            j = ((((i) as i32).wrapping_add(1i32)) as u8);
                            'l7: loop {
                                if !(((j) as i32) < 24i32) {
                                    break 'l7;
                                }
                                'l8: {
                                    if ((((shows).wrapping_offset(((j) as i32) as isize * 36))
                                        .read()) as i32)
                                        != 0i32
                                    {
                                        (shows)
                                            .wrapping_offset(((i) as i32) as isize * 36)
                                            .cast::<crate::c::Rec4<36>>()
                                            .write_unaligned(
                                                (shows)
                                                    .wrapping_offset(((j) as i32) as isize * 36)
                                                    .cast::<crate::c::Rec4<36>>()
                                                    .read_unaligned(),
                                            );
                                        DeleteTVShowInArrayByIdx(shows, j);
                                        break 'l7;
                                    }
                                }
                                j = (j).wrapping_add(1);
                            }
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn GetRandomDifferentSpeciesAndNameSeenByPlayer(
    varIdx: u8,
    excludedSpecies: u16,
) -> u16 {
    unsafe {
        let mut varIdx = varIdx;
        let mut excludedSpecies = excludedSpecies;
        let mut species: u16 = GetRandomDifferentSpeciesSeenByPlayer(excludedSpecies);
        StringCopy(
            ((((&raw const gTVStringVarPtrs)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u8>())
            .cast::<*mut u8>())
            .wrapping_offset(((varIdx) as i32) as isize))
            .read(),
            (((&raw mut gSpeciesNames).cast::<u8>())
                .wrapping_offset(((species) as i32) as isize * 11))
            .cast::<u8>(),
        );
        return species;
    }
}
pub(crate) unsafe extern "C" fn GetRandomDifferentSpeciesSeenByPlayer(excludedSpecies: u16) -> u16 {
    unsafe {
        let mut excludedSpecies = excludedSpecies;
        let mut species: u16 =
            (((crate::c::rem_i32(((Random()) as i32), 411i32)).wrapping_add(1i32)) as u16);
        let mut initSpecies: u16 = species;
        'l1: loop {
            if !((((GetSetPokedexFlag(SpeciesToNationalPokedexNum(species), 0u8)) as i32) != 1i32)
                || (((species) as i32) == ((excludedSpecies) as i32)))
            {
                break 'l1;
            }
            if ((species) as i32) == 1i32 {
                species = 411u16;
            } else {
                species = (species).wrapping_sub(1);
            }
            if ((species) as i32) == ((initSpecies) as i32) {
                species = excludedSpecies;
                return species;
            }
        }
        return species;
    }
}
pub(crate) unsafe extern "C" fn Script_FindFirstEmptyNormalTVShowSlot() {
    unsafe {
        ((&raw mut sCurTVShowSlot).cast::<u8>().cast::<i8>()).write(
            FindFirstEmptyNormalTVShowSlot(
                ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(10188))
                    .cast::<u8>(),
            ),
        );
        ((&raw mut gSpecialVar_0x8006).cast::<u16>())
            .write(((((&raw mut sCurTVShowSlot).cast::<u8>().cast::<i8>()).read()) as u16));
        if ((((&raw mut sCurTVShowSlot).cast::<u8>().cast::<i8>()).read()) as i32) == (-1i32) {
            ((&raw mut gSpecialVar_Result).cast::<u16>()).write(1u16);
        } else {
            ((&raw mut gSpecialVar_Result).cast::<u16>()).write(0u16);
        }
    }
}
pub(crate) unsafe extern "C" fn FindFirstEmptyNormalTVShowSlot(shows: *mut u8) -> i8 {
    unsafe {
        let mut shows = shows;
        let mut i: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 5i32) {
                    break 'l1;
                }
                'l2: {
                    if ((((shows).wrapping_offset(((i) as i32) as isize * 36)).read()) as i32)
                        == 0i32
                    {
                        return ((i) as i8);
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return (-1i8);
    }
}
pub(crate) unsafe extern "C" fn FindFirstEmptyRecordMixTVShowSlot(shows: *mut u8) -> i8 {
    unsafe {
        let mut shows = shows;
        let mut i: i8 = 0i8;
        {
            i = 5i8;
            'l1: loop {
                if !(((i) as i32) < 24i32) {
                    break 'l1;
                }
                'l2: {
                    if ((((shows).wrapping_offset(((i) as i32) as isize * 36)).read()) as i32)
                        == 0i32
                    {
                        return i;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return (-1i8);
    }
}
pub(crate) unsafe extern "C" fn BernoulliTrial(ratio: u16) -> u8 {
    unsafe {
        let mut ratio = ratio;
        if ((Random()) as i32) <= ((ratio) as i32) {
            return 0u8;
        }
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn GetRandomWordFromShow(show: *mut u8) {
    unsafe {
        let mut show = show;
        let mut i: u8 = 0u8;
        i = ((crate::c::rem_u32(((Random()) as u32), crate::c::div_u32(12u32, 2u32))) as u8);
        'l1: loop {
            if !((1i32) != 0) {
                break 'l1;
            }
            if ((i) as u32) == crate::c::div_u32(12u32, 2u32) {
                i = 0u8;
            }
            if ((((((show).wrapping_add(4)).cast::<u16>()).wrapping_offset(((i) as i32) as isize))
                .read()) as i32)
                != 65535i32
            {
                break 'l1;
            }
            i = (i).wrapping_add(1);
        }
        CopyEasyChatWord(
            (&raw mut gStringVar3).cast::<u8>(),
            ((((show).wrapping_add(4)).cast::<u16>()).wrapping_offset(((i) as i32) as isize))
                .read(),
        );
    }
}
pub(crate) unsafe extern "C" fn GetRandomNameRaterStateFromName(show: *mut u8) -> u8 {
    unsafe {
        let mut show = show;
        let mut i: u8 = 0u8;
        let mut nameSum: u16 = 0u16;
        nameSum = 0u16;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 11i32) {
                    break 'l1;
                }
                'l2: {
                    if ((((((show).wrapping_add(4)).cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize))
                    .read()) as i32)
                        == 255i32
                    {
                        break 'l1;
                    }
                    nameSum = ((((nameSum) as i32).wrapping_add(
                        ((((((show).wrapping_add(4)).cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize))
                        .read()) as i32),
                    )) as u16);
                }
                i = (i).wrapping_add(1);
            }
        }
        return ((((nameSum) as i32) & 7i32) as u8);
    }
}
pub(crate) unsafe extern "C" fn GetNicknameSubstring(
    varIdx: u8,
    whichPosition: u8,
    charParam: u8,
    whichString: u16,
    species: u16,
    show: *mut u8,
) {
    unsafe {
        let mut varIdx = varIdx;
        let mut whichPosition = whichPosition;
        let mut charParam = charParam;
        let mut whichString = whichString;
        let mut species = species;
        let mut show = show;
        let mut buff = crate::ffi::Align4([0u8; 16]);
        let mut i: u8 = 0u8;
        let mut strlen: u16 = 0u16;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 3i32) {
                    break 'l1;
                }
                'l2: {
                    (((&raw mut buff).cast::<u8>()).wrapping_offset(((i) as i32) as isize))
                        .write(255u8);
                }
                i = (i).wrapping_add(1);
            }
        }
        if ((whichString) as i32) == 0i32 {
            strlen = StringLength(((show).wrapping_add(15)).cast::<u8>());
            if ((charParam) as i32) == 0i32 {
                ((&raw mut buff).cast::<u8>()).write(
                    ((((show).wrapping_add(15)).cast::<u8>())
                        .wrapping_offset(((whichPosition) as i32) as isize))
                    .read(),
                );
            } else {
                if ((charParam) as i32) == 1i32 {
                    ((&raw mut buff).cast::<u8>()).write(
                        ((((show).wrapping_add(15)).cast::<u8>()).wrapping_offset(
                            (((strlen) as i32).wrapping_sub(((whichPosition) as i32))) as isize,
                        ))
                        .read(),
                    );
                } else {
                    if ((charParam) as i32) == 2i32 {
                        ((&raw mut buff).cast::<u8>()).write(
                            ((((show).wrapping_add(15)).cast::<u8>())
                                .wrapping_offset(((whichPosition) as i32) as isize))
                            .read(),
                        );
                        (((&raw mut buff).cast::<u8>()).wrapping_offset(1)).write(
                            ((((show).wrapping_add(15)).cast::<u8>()).wrapping_offset(
                                (((whichPosition) as i32).wrapping_add(1i32)) as isize,
                            ))
                            .read(),
                        );
                    } else {
                        ((&raw mut buff).cast::<u8>()).write(
                            ((((show).wrapping_add(15)).cast::<u8>()).wrapping_offset(
                                (((strlen) as i32)
                                    .wrapping_sub(((whichPosition) as i32).wrapping_add(2i32)))
                                    as isize,
                            ))
                            .read(),
                        );
                        (((&raw mut buff).cast::<u8>()).wrapping_offset(1)).write(
                            ((((show).wrapping_add(15)).cast::<u8>()).wrapping_offset(
                                (((strlen) as i32)
                                    .wrapping_sub(((whichPosition) as i32).wrapping_add(1i32)))
                                    as isize,
                            ))
                            .read(),
                        );
                    }
                }
            }
            ConvertInternationalString(
                (&raw mut buff).cast::<u8>(),
                ((show).wrapping_add(30)).read(),
            );
        } else {
            if ((whichString) as i32) == 1i32 {
                strlen = StringLength(((show).wrapping_add(4)).cast::<u8>());
                if ((charParam) as i32) == 0i32 {
                    ((&raw mut buff).cast::<u8>()).write(
                        ((((show).wrapping_add(4)).cast::<u8>())
                            .wrapping_offset(((whichPosition) as i32) as isize))
                        .read(),
                    );
                } else {
                    if ((charParam) as i32) == 1i32 {
                        ((&raw mut buff).cast::<u8>()).write(
                            ((((show).wrapping_add(4)).cast::<u8>()).wrapping_offset(
                                (((strlen) as i32).wrapping_sub(((whichPosition) as i32))) as isize,
                            ))
                            .read(),
                        );
                    } else {
                        if ((charParam) as i32) == 2i32 {
                            ((&raw mut buff).cast::<u8>()).write(
                                ((((show).wrapping_add(4)).cast::<u8>())
                                    .wrapping_offset(((whichPosition) as i32) as isize))
                                .read(),
                            );
                            (((&raw mut buff).cast::<u8>()).wrapping_offset(1)).write(
                                ((((show).wrapping_add(4)).cast::<u8>()).wrapping_offset(
                                    (((whichPosition) as i32).wrapping_add(1i32)) as isize,
                                ))
                                .read(),
                            );
                        } else {
                            ((&raw mut buff).cast::<u8>()).write(
                                ((((show).wrapping_add(4)).cast::<u8>()).wrapping_offset(
                                    (((strlen) as i32)
                                        .wrapping_sub(((whichPosition) as i32).wrapping_add(2i32)))
                                        as isize,
                                ))
                                .read(),
                            );
                            (((&raw mut buff).cast::<u8>()).wrapping_offset(1)).write(
                                ((((show).wrapping_add(4)).cast::<u8>()).wrapping_offset(
                                    (((strlen) as i32)
                                        .wrapping_sub(((whichPosition) as i32).wrapping_add(1i32)))
                                        as isize,
                                ))
                                .read(),
                            );
                        }
                    }
                }
                ConvertInternationalString(
                    (&raw mut buff).cast::<u8>(),
                    ((show).wrapping_add(31)).read(),
                );
            } else {
                strlen = StringLength(
                    (((&raw mut gSpeciesNames).cast::<u8>())
                        .wrapping_offset(((species) as i32) as isize * 11))
                    .cast::<u8>(),
                );
                if ((charParam) as i32) == 0i32 {
                    ((&raw mut buff).cast::<u8>()).write(
                        (((((&raw mut gSpeciesNames).cast::<u8>())
                            .wrapping_offset(((species) as i32) as isize * 11))
                        .cast::<u8>())
                        .wrapping_offset(((whichPosition) as i32) as isize))
                        .read(),
                    );
                } else {
                    if ((charParam) as i32) == 1i32 {
                        ((&raw mut buff).cast::<u8>()).write(
                            (((((&raw mut gSpeciesNames).cast::<u8>())
                                .wrapping_offset(((species) as i32) as isize * 11))
                            .cast::<u8>())
                            .wrapping_offset(
                                (((strlen) as i32).wrapping_sub(((whichPosition) as i32))) as isize,
                            ))
                            .read(),
                        );
                    } else {
                        if ((charParam) as i32) == 2i32 {
                            ((&raw mut buff).cast::<u8>()).write(
                                (((((&raw mut gSpeciesNames).cast::<u8>())
                                    .wrapping_offset(((species) as i32) as isize * 11))
                                .cast::<u8>())
                                .wrapping_offset(((whichPosition) as i32) as isize))
                                .read(),
                            );
                            (((&raw mut buff).cast::<u8>()).wrapping_offset(1)).write(
                                (((((&raw mut gSpeciesNames).cast::<u8>())
                                    .wrapping_offset(((species) as i32) as isize * 11))
                                .cast::<u8>())
                                .wrapping_offset(
                                    (((whichPosition) as i32).wrapping_add(1i32)) as isize,
                                ))
                                .read(),
                            );
                        } else {
                            ((&raw mut buff).cast::<u8>()).write(
                                (((((&raw mut gSpeciesNames).cast::<u8>())
                                    .wrapping_offset(((species) as i32) as isize * 11))
                                .cast::<u8>())
                                .wrapping_offset(
                                    (((strlen) as i32)
                                        .wrapping_sub(((whichPosition) as i32).wrapping_add(2i32)))
                                        as isize,
                                ))
                                .read(),
                            );
                            (((&raw mut buff).cast::<u8>()).wrapping_offset(1)).write(
                                (((((&raw mut gSpeciesNames).cast::<u8>())
                                    .wrapping_offset(((species) as i32) as isize * 11))
                                .cast::<u8>())
                                .wrapping_offset(
                                    (((strlen) as i32)
                                        .wrapping_sub(((whichPosition) as i32).wrapping_add(1i32)))
                                        as isize,
                                ))
                                .read(),
                            );
                        }
                    }
                }
            }
        }
        StringCopy(
            ((((&raw const gTVStringVarPtrs)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u8>())
            .cast::<*mut u8>())
            .wrapping_offset(((varIdx) as i32) as isize))
            .read(),
            (&raw mut buff).cast::<u8>(),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsTVShowAlreadyInQueue() -> u8 {
    unsafe {
        let mut i: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 5i32) {
                    break 'l1;
                }
                'l2: {
                    if ((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(10188))
                    .cast::<u8>())
                    .wrapping_offset(((i) as i32) as isize * 36))
                    .read()) as i32)
                        == ((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32)
                    {
                        return 1u8;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn TryPutNameRaterShowOnTheAir() -> u8 {
    unsafe {
        GetMonData3(
            ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                ((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32) as isize * 100,
            ),
            2i32,
            (&raw mut gStringVar1).cast::<u8>(),
        );
        if !((StringCompare(
            (&raw mut gStringVar3).cast::<u8>(),
            (&raw mut gStringVar1).cast::<u8>(),
        )) != 0)
        {
            return 0u8;
        }
        PutNameRaterShowOnTheAir();
        return 1u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ChangePokemonNickname() {
    unsafe {
        GetMonData3(
            ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                ((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32) as isize * 100,
            ),
            2i32,
            (&raw mut gStringVar3).cast::<u8>(),
        );
        GetMonData3(
            ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                ((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32) as isize * 100,
            ),
            2i32,
            (&raw mut gStringVar2).cast::<u8>(),
        );
        DoNamingScreen(
            3u8,
            (&raw mut gStringVar2).cast::<u8>(),
            ((GetMonData3(
                ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                    ((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32) as isize * 100,
                ),
                11i32,
                core::ptr::null_mut(),
            )) as u16),
            ((GetMonGender(((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                ((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32) as isize * 100,
            ))) as u16),
            GetMonData3(
                ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                    ((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32) as isize * 100,
                ),
                0i32,
                core::ptr::null_mut(),
            ),
            Some(ChangePokemonNickname_CB),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ChangePokemonNickname_CB() {
    unsafe {
        SetMonData(
            ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                ((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32) as isize * 100,
            ),
            2i32,
            (&raw mut gStringVar2).cast::<u8>(),
        );
        CB2_ReturnToFieldContinueScriptPlayMapMusic();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ChangeBoxPokemonNickname() {
    unsafe {
        let mut boxMon: *mut u8 = core::ptr::null_mut();
        boxMon = GetBoxedMonPtr(
            ((((&raw mut gSpecialVar_MonBoxId).cast::<u16>()).read()) as u8),
            ((((&raw mut gSpecialVar_MonBoxPos).cast::<u16>()).read()) as u8),
        );
        GetBoxMonData3(boxMon, 2i32, (&raw mut gStringVar3).cast::<u8>());
        GetBoxMonData3(boxMon, 2i32, (&raw mut gStringVar2).cast::<u8>());
        DoNamingScreen(
            3u8,
            (&raw mut gStringVar2).cast::<u8>(),
            ((GetBoxMonData3(boxMon, 11i32, core::ptr::null_mut())) as u16),
            ((GetBoxMonGender(boxMon)) as u16),
            GetBoxMonData3(boxMon, 0i32, core::ptr::null_mut()),
            Some(ChangeBoxPokemonNickname_CB),
        );
    }
}
pub(crate) unsafe extern "C" fn ChangeBoxPokemonNickname_CB() {
    unsafe {
        SetBoxMonNickAt(
            ((((&raw mut gSpecialVar_MonBoxId).cast::<u16>()).read()) as u8),
            ((((&raw mut gSpecialVar_MonBoxPos).cast::<u16>()).read()) as u8),
            (&raw mut gStringVar2).cast::<u8>(),
        );
        CB2_ReturnToFieldContinueScriptPlayMapMusic();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BufferMonNickname() {
    unsafe {
        GetMonData3(
            ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                ((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32) as isize * 100,
            ),
            2i32,
            (&raw mut gStringVar1).cast::<u8>(),
        );
        StringGet_Nickname((&raw mut gStringVar1).cast::<u8>());
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsMonOTIDNotPlayers() {
    unsafe {
        if GetPlayerIDAsU32()
            == GetMonData3(
                ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                    ((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32) as isize * 100,
                ),
                1i32,
                core::ptr::null_mut(),
            )
        {
            ((&raw mut gSpecialVar_Result).cast::<u16>()).write(0u16);
        } else {
            ((&raw mut gSpecialVar_Result).cast::<u16>()).write(1u16);
        }
    }
}
pub(crate) unsafe extern "C" fn GetTVGroupByShowId(kind: u8) -> u8 {
    unsafe {
        let mut kind = kind;
        if ((kind) as i32) == 0i32 {
            return 0u8;
        }
        if (((kind) as i32) >= 1i32) && (((kind) as i32) <= 20i32) {
            return 2u8;
        }
        if (((kind) as i32) >= 21i32) && (((kind) as i32) <= 40i32) {
            return 3u8;
        }
        if (((kind) as i32) >= 41i32) && (((kind) as i32) <= 60i32) {
            return 4u8;
        }
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetPlayerIDAsU32() -> u32 {
    unsafe {
        return (((((((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(10))
            .cast::<u8>())
        .wrapping_offset(3))
        .read()) as i32)
            << 24)
            | (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(10))
                .cast::<u8>())
            .wrapping_offset(2))
            .read()) as i32)
                << 16))
            | (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(10))
                .cast::<u8>())
            .wrapping_offset(1))
            .read()) as i32)
                << 8))
            | (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(10))
                .cast::<u8>())
            .read()) as i32)) as u32);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CheckForPlayersHouseNews() -> u8 {
    unsafe {
        if (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4))
            .cast::<i8>())
        .read()) as i32)
            != 1i32
        {
            return 0u8;
        }
        if ((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(8)).read())
            as i32)
            == 0i32
        {
            if (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4))
                .wrapping_add(1)
                .cast::<i8>())
            .read()) as i32)
                != 0i32
            {
                return 0u8;
            }
        } else {
            if (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4))
                .wrapping_add(1)
                .cast::<i8>())
            .read()) as i32)
                != 2i32
            {
                return 0u8;
            }
        }
        if ((FlagGet(2237u16)) as i32) == 1i32 {
            return 1u8;
        }
        if ((FlagGet(2192u16)) as i32) == 1i32 {
            return 2u8;
        }
        return 1u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetMomOrDadStringForTVMessage() {
    unsafe {
        if (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4))
            .cast::<i8>())
        .read()) as i32)
            == 1i32
        {
            if ((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(8)).read())
                as i32)
                == 0i32
            {
                if (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4))
                    .wrapping_add(1)
                    .cast::<i8>())
                .read()) as i32)
                    == 0i32
                {
                    StringCopy(
                        (&raw mut gStringVar1).cast::<u8>(),
                        (&raw mut gText_Mom).cast::<u8>(),
                    );
                    VarSet(16387u16, 1u16);
                }
            } else {
                if (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4))
                    .wrapping_add(1)
                    .cast::<i8>())
                .read()) as i32)
                    == 2i32
                {
                    StringCopy(
                        (&raw mut gStringVar1).cast::<u8>(),
                        (&raw mut gText_Mom).cast::<u8>(),
                    );
                    VarSet(16387u16, 1u16);
                }
            }
        }
        if ((VarGet(16387u16)) as i32) == 1i32 {
            StringCopy(
                (&raw mut gStringVar1).cast::<u8>(),
                (&raw mut gText_Mom).cast::<u8>(),
            );
        } else {
            if ((VarGet(16387u16)) as i32) == 2i32 {
                StringCopy(
                    (&raw mut gStringVar1).cast::<u8>(),
                    (&raw mut gText_Dad).cast::<u8>(),
                );
            } else {
                if ((VarGet(16387u16)) as i32) > 2i32 {
                    if crate::c::rem_i32(((VarGet(16387u16)) as i32), 2i32) == 0i32 {
                        StringCopy(
                            (&raw mut gStringVar1).cast::<u8>(),
                            (&raw mut gText_Mom).cast::<u8>(),
                        );
                    } else {
                        StringCopy(
                            (&raw mut gStringVar1).cast::<u8>(),
                            (&raw mut gText_Dad).cast::<u8>(),
                        );
                    }
                } else {
                    if crate::c::rem_i32(((Random()) as i32), 2i32) != 0i32 {
                        StringCopy(
                            (&raw mut gStringVar1).cast::<u8>(),
                            (&raw mut gText_Mom).cast::<u8>(),
                        );
                        VarSet(16387u16, 1u16);
                    } else {
                        StringCopy(
                            (&raw mut gStringVar1).cast::<u8>(),
                            (&raw mut gText_Dad).cast::<u8>(),
                        );
                        VarSet(16387u16, 2u16);
                    }
                }
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn HideBattleTowerReporter() {
    unsafe {
        VarSet(16572u16, 0u16);
        RemoveObjectEventByLocalIdAndMap(
            5u8,
            (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4))
                .wrapping_add(1)
                .cast::<i8>())
            .read()) as u8),
            (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4))
                .cast::<i8>())
            .read()) as u8),
        );
        FlagSet(918u16);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ReceiveTvShowsData(src: *mut u8, size: u32, playersLinkId: u8) {
    unsafe {
        let mut src = src;
        let mut size = size;
        let mut playersLinkId = playersLinkId;
        let mut i: u8 = 0u8;
        let mut version: u16 = 0u16;
        let mut rmBuffer2: *mut u8 = core::ptr::null_mut();
        let mut rmBuffer: *mut u8 = core::ptr::null_mut();
        rmBuffer2 = Alloc(3600u32);
        if ((rmBuffer2) as usize) != 0usize {
            {
                i = 0u8;
                'l1: loop {
                    if !(((i) as i32) < 4i32) {
                        break 'l1;
                    }
                    'l2: {
                        crate::c::memcpy(
                            (((rmBuffer2).cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize * 900))
                            .cast::<u8>(),
                            (src).wrapping_offset(
                                ((((i) as u32).wrapping_mul(size)) as i32) as isize * 1,
                            ),
                            900u32,
                        );
                    }
                    i = (i).wrapping_add(1);
                }
            }
            rmBuffer = rmBuffer2;
            {
                i = 0u8;
                'l3: loop {
                    if !(((i) as i32) < ((GetLinkPlayerCount()) as i32)) {
                        break 'l3;
                    }
                    'l4: {
                        version = (((((((&raw mut gLinkPlayers).cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 28))
                        .cast::<u16>())
                        .read()) as u8) as u16);
                        if (((version) as i32) == 2i32) || (((version) as i32) == 1i32) {
                            TranslateRubyShows(
                                (((rmBuffer).cast::<u8>())
                                    .wrapping_offset(((i) as i32) as isize * 900))
                                .cast::<u8>(),
                            );
                        } else {
                            if (((version) as i32) == 3i32)
                                && (((((((&raw mut gLinkPlayers).cast::<u8>())
                                    .wrapping_offset(((i) as i32) as isize * 28))
                                .wrapping_add(26)
                                .cast::<u16>())
                                .read()) as i32)
                                    == 1i32)
                            {
                                TranslateJapaneseEmeraldShows(
                                    (((rmBuffer).cast::<u8>())
                                        .wrapping_offset(((i) as i32) as isize * 900))
                                    .cast::<u8>(),
                                );
                            }
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
            'l5: {
                let __sw1 = ((playersLinkId) as i32);
                if __sw1 == 0i32 {
                    SetMixedTVShows(
                        ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(10188))
                        .cast::<u8>(),
                        (((rmBuffer).cast::<u8>()).wrapping_offset(900)).cast::<u8>(),
                        (((rmBuffer).cast::<u8>()).wrapping_offset(1800)).cast::<u8>(),
                        (((rmBuffer).cast::<u8>()).wrapping_offset(2700)).cast::<u8>(),
                    );
                    break 'l5;
                }
                if __sw1 == 1i32 {
                    SetMixedTVShows(
                        ((rmBuffer).cast::<u8>()).cast::<u8>(),
                        ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(10188))
                        .cast::<u8>(),
                        (((rmBuffer).cast::<u8>()).wrapping_offset(1800)).cast::<u8>(),
                        (((rmBuffer).cast::<u8>()).wrapping_offset(2700)).cast::<u8>(),
                    );
                    break 'l5;
                }
                if __sw1 == 2i32 {
                    SetMixedTVShows(
                        ((rmBuffer).cast::<u8>()).cast::<u8>(),
                        (((rmBuffer).cast::<u8>()).wrapping_offset(900)).cast::<u8>(),
                        ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(10188))
                        .cast::<u8>(),
                        (((rmBuffer).cast::<u8>()).wrapping_offset(2700)).cast::<u8>(),
                    );
                    break 'l5;
                }
                if __sw1 == 3i32 {
                    SetMixedTVShows(
                        ((rmBuffer).cast::<u8>()).cast::<u8>(),
                        (((rmBuffer).cast::<u8>()).wrapping_offset(900)).cast::<u8>(),
                        (((rmBuffer).cast::<u8>()).wrapping_offset(1800)).cast::<u8>(),
                        ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(10188))
                        .cast::<u8>(),
                    );
                    break 'l5;
                }
            }
            CompactTVShowArray(
                ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(10188))
                    .cast::<u8>(),
            );
            DeleteExcessMixedShows();
            CompactTVShowArray(
                ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(10188))
                    .cast::<u8>(),
            );
            DeactivateShowsWithUnseenSpecies();
            DeactivateGameCompleteShowsIfNotUnlocked();
            Free(rmBuffer2);
        }
    }
}
pub(crate) unsafe extern "C" fn SetMixedTVShows(
    player1: *mut u8,
    player2: *mut u8,
    player3: *mut u8,
    player4: *mut u8,
) {
    unsafe {
        let mut player1 = player1;
        let mut player2 = player2;
        let mut player3 = player3;
        let mut player4 = player4;
        let mut i: u8 = 0u8;
        let mut j: u8 = 0u8;
        let mut tvShows = crate::ffi::Align4([0u8; 16]);
        ((&raw mut tvShows).cast::<*mut *mut u8>()).write(&raw mut player1);
        (((&raw mut tvShows).cast::<*mut *mut u8>()).wrapping_offset(1)).write(&raw mut player2);
        (((&raw mut tvShows).cast::<*mut *mut u8>()).wrapping_offset(2)).write(&raw mut player3);
        (((&raw mut tvShows).cast::<*mut *mut u8>()).wrapping_offset(3)).write(&raw mut player4);
        ((&raw mut sTVShowMixingNumPlayers).cast::<u8>().cast::<u8>()).write(GetLinkPlayerCount());
        'l1: loop {
            if !((1i32) != 0) {
                break 'l1;
            }
            {
                i = 0u8;
                'l2: loop {
                    if !(((i) as i32)
                        < ((((&raw mut sTVShowMixingNumPlayers).cast::<u8>().cast::<u8>()).read())
                            as i32))
                    {
                        break 'l2;
                    }
                    'l3: {
                        if ((i) as i32) == 0i32 {
                            ((&raw mut sRecordMixingPartnersWithoutShowsToShare)
                                .cast::<u8>()
                                .cast::<u8>())
                            .write(0u8);
                        }
                        ((&raw mut sTVShowMixingCurSlot).cast::<u8>().cast::<i8>()).write(
                            FindInactiveShowInArray(
                                ((((&raw mut tvShows).cast::<*mut *mut u8>())
                                    .wrapping_offset(((i) as i32) as isize))
                                .read())
                                .read(),
                            ),
                        );
                        if ((((&raw mut sTVShowMixingCurSlot).cast::<u8>().cast::<i8>()).read())
                            as i32)
                            == (-1i32)
                        {
                            let __p1 = (&raw mut sRecordMixingPartnersWithoutShowsToShare)
                                .cast::<u8>()
                                .cast::<u8>();
                            (__p1).write(((__p1).read()).wrapping_add(1));
                            if ((((&raw mut sRecordMixingPartnersWithoutShowsToShare)
                                .cast::<u8>()
                                .cast::<u8>())
                            .read()) as i32)
                                == ((((&raw mut sTVShowMixingNumPlayers).cast::<u8>().cast::<u8>())
                                    .read()) as i32)
                            {
                                return;
                            }
                        } else {
                            {
                                j = 0u8;
                                'l4: loop {
                                    if !(((j) as i32)
                                        < ((((&raw mut sTVShowMixingNumPlayers)
                                            .cast::<u8>()
                                            .cast::<u8>())
                                        .read()) as i32)
                                            .wrapping_sub(1i32))
                                    {
                                        break 'l4;
                                    }
                                    'l5: {
                                        ((&raw mut sCurTVShowSlot).cast::<u8>().cast::<i8>())
                                            .write(FindFirstEmptyRecordMixTVShowSlot(
                                                ((((&raw mut tvShows).cast::<*mut *mut u8>())
                                                    .wrapping_offset(
                                                        (crate::c::rem_i32(
                                                            (((i) as i32)
                                                                .wrapping_add(((j) as i32)))
                                                            .wrapping_add(1i32),
                                                            ((((&raw mut sTVShowMixingNumPlayers)
                                                                .cast::<u8>()
                                                                .cast::<u8>())
                                                            .read())
                                                                as i32),
                                                        ))
                                                            as isize,
                                                    ))
                                                .read())
                                                .read(),
                                            ));
                                        if (((((&raw mut sCurTVShowSlot).cast::<u8>().cast::<i8>())
                                            .read())
                                            as i32)
                                            != (-1i32))
                                            && (((TryMixTVShow(
                                                (((&raw mut tvShows).cast::<*mut *mut u8>())
                                                    .wrapping_offset(
                                                        (crate::c::rem_i32(
                                                            (((i) as i32)
                                                                .wrapping_add(((j) as i32)))
                                                            .wrapping_add(1i32),
                                                            ((((&raw mut sTVShowMixingNumPlayers)
                                                                .cast::<u8>()
                                                                .cast::<u8>())
                                                            .read())
                                                                as i32),
                                                        ))
                                                            as isize,
                                                    ))
                                                .read(),
                                                (((&raw mut tvShows).cast::<*mut *mut u8>())
                                                    .wrapping_offset(((i) as i32) as isize))
                                                .read(),
                                                ((crate::c::rem_i32(
                                                    (((i) as i32).wrapping_add(((j) as i32)))
                                                        .wrapping_add(1i32),
                                                    ((((&raw mut sTVShowMixingNumPlayers)
                                                        .cast::<u8>()
                                                        .cast::<u8>())
                                                    .read())
                                                        as i32),
                                                ))
                                                    as u8),
                                            ))
                                                as i32)
                                                == 1i32)
                                        {
                                            break 'l4;
                                        }
                                    }
                                    j = (j).wrapping_add(1);
                                }
                            }
                            if ((j) as i32)
                                == ((((&raw mut sTVShowMixingNumPlayers).cast::<u8>().cast::<u8>())
                                    .read()) as i32)
                                    .wrapping_sub(1i32)
                            {
                                DeleteTVShowInArrayByIdx(
                                    ((((&raw mut tvShows).cast::<*mut *mut u8>())
                                        .wrapping_offset(((i) as i32) as isize))
                                    .read())
                                    .read(),
                                    ((((&raw mut sTVShowMixingCurSlot).cast::<u8>().cast::<i8>())
                                        .read()) as u8),
                                );
                            }
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn TryMixTVShow(dest: *mut *mut u8, src: *mut *mut u8, idx: u8) -> u8 {
    unsafe {
        let mut dest = dest;
        let mut src = src;
        let mut idx = idx;
        let mut success: u8 = 0u8;
        let mut r#type: u8 = 0u8;
        let mut tv1: *mut u8 = (dest).read();
        let mut tv2: *mut u8 = (src).read();
        success = 0u8;
        r#type = GetTVGroupByShowId(
            ((tv2).wrapping_offset(
                ((((&raw mut sTVShowMixingCurSlot).cast::<u8>().cast::<i8>()).read()) as i32)
                    as isize
                    * 36,
            ))
            .read(),
        );
        'l1: {
            let __sw1 = ((r#type) as i32);
            if __sw1 == 2i32 {
                success = TryMixNormalTVShow(
                    (tv1).wrapping_offset(
                        ((((&raw mut sCurTVShowSlot).cast::<u8>().cast::<i8>()).read()) as i32)
                            as isize
                            * 36,
                    ),
                    (tv2).wrapping_offset(
                        ((((&raw mut sTVShowMixingCurSlot).cast::<u8>().cast::<i8>()).read())
                            as i32) as isize
                            * 36,
                    ),
                    idx,
                );
                break 'l1;
            }
            if __sw1 == 3i32 {
                success = TryMixRecordMixTVShow(
                    (tv1).wrapping_offset(
                        ((((&raw mut sCurTVShowSlot).cast::<u8>().cast::<i8>()).read()) as i32)
                            as isize
                            * 36,
                    ),
                    (tv2).wrapping_offset(
                        ((((&raw mut sTVShowMixingCurSlot).cast::<u8>().cast::<i8>()).read())
                            as i32) as isize
                            * 36,
                    ),
                    idx,
                );
                break 'l1;
            }
            if __sw1 == 4i32 {
                success = TryMixOutbreakTVShow(
                    (tv1).wrapping_offset(
                        ((((&raw mut sCurTVShowSlot).cast::<u8>().cast::<i8>()).read()) as i32)
                            as isize
                            * 36,
                    ),
                    (tv2).wrapping_offset(
                        ((((&raw mut sTVShowMixingCurSlot).cast::<u8>().cast::<i8>()).read())
                            as i32) as isize
                            * 36,
                    ),
                    idx,
                );
                break 'l1;
            }
        }
        if ((success) as i32) == 1i32 {
            DeleteTVShowInArrayByIdx(
                tv2,
                ((((&raw mut sTVShowMixingCurSlot).cast::<u8>().cast::<i8>()).read()) as u8),
            );
            return 1u8;
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn TryMixNormalTVShow(dest: *mut u8, src: *mut u8, idx: u8) -> u8 {
    unsafe {
        let mut dest = dest;
        let mut src = src;
        let mut idx = idx;
        let mut linkTrainerId: u32 = GetLinkPlayerTrainerId(idx);
        if ((linkTrainerId & 255u32) == ((((src).wrapping_add(34)).read()) as u32))
            && (((linkTrainerId >> 8) & 255u32) == ((((src).wrapping_add(35)).read()) as u32))
        {
            return 0u8;
        }
        ((src).wrapping_add(34)).write(((src).wrapping_add(32)).read());
        ((src).wrapping_add(35)).write(((src).wrapping_add(33)).read());
        ((src).wrapping_add(32)).write(((linkTrainerId & 255u32) as u8));
        ((src).wrapping_add(33)).write(((linkTrainerId >> 8) as u8));
        dest.cast::<crate::c::Rec4<36>>()
            .write_unaligned(src.cast::<crate::c::Rec4<36>>().read_unaligned());
        ((dest).wrapping_add(1)).write(1u8);
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn TryMixRecordMixTVShow(dest: *mut u8, src: *mut u8, idx: u8) -> u8 {
    unsafe {
        let mut dest = dest;
        let mut src = src;
        let mut idx = idx;
        let mut linkTrainerId: u32 = GetLinkPlayerTrainerId(idx);
        if ((linkTrainerId & 255u32) == ((((src).wrapping_add(32)).read()) as u32))
            && (((linkTrainerId >> 8) & 255u32) == ((((src).wrapping_add(33)).read()) as u32))
        {
            return 0u8;
        }
        if ((linkTrainerId & 255u32) == ((((src).wrapping_add(34)).read()) as u32))
            && (((linkTrainerId >> 8) & 255u32) == ((((src).wrapping_add(35)).read()) as u32))
        {
            return 0u8;
        }
        ((src).wrapping_add(32)).write(((src).wrapping_add(30)).read());
        ((src).wrapping_add(33)).write(((src).wrapping_add(31)).read());
        ((src).wrapping_add(30)).write(((linkTrainerId & 255u32) as u8));
        ((src).wrapping_add(31)).write(((linkTrainerId >> 8) as u8));
        dest.cast::<crate::c::Rec4<36>>()
            .write_unaligned(src.cast::<crate::c::Rec4<36>>().read_unaligned());
        ((dest).wrapping_add(1)).write(1u8);
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn TryMixOutbreakTVShow(dest: *mut u8, src: *mut u8, idx: u8) -> u8 {
    unsafe {
        let mut dest = dest;
        let mut src = src;
        let mut idx = idx;
        let mut linkTrainerId: u32 = GetLinkPlayerTrainerId(idx);
        if ((linkTrainerId & 255u32) == ((((src).wrapping_add(34)).read()) as u32))
            && (((linkTrainerId >> 8) & 255u32) == ((((src).wrapping_add(35)).read()) as u32))
        {
            return 0u8;
        }
        ((src).wrapping_add(34)).write(((src).wrapping_add(32)).read());
        ((src).wrapping_add(35)).write(((src).wrapping_add(33)).read());
        ((src).wrapping_add(32)).write(((linkTrainerId & 255u32) as u8));
        ((src).wrapping_add(33)).write(((linkTrainerId >> 8) as u8));
        dest.cast::<crate::c::Rec4<36>>()
            .write_unaligned(src.cast::<crate::c::Rec4<36>>().read_unaligned());
        ((dest).wrapping_add(1)).write(1u8);
        ((dest).wrapping_add(22).cast::<u16>()).write(1u16);
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn FindInactiveShowInArray(tvShows: *mut u8) -> i8 {
    unsafe {
        let mut tvShows = tvShows;
        let mut i: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 24i32) {
                    break 'l1;
                }
                'l2: {
                    if ((((((tvShows).wrapping_offset(((i) as i32) as isize * 36)).wrapping_add(1))
                        .read()) as i32)
                        == 0i32)
                        && ((((((((tvShows).wrapping_offset(((i) as i32) as isize * 36)).read())
                            as i32)
                            .wrapping_sub(1i32)) as u8) as i32)
                            < 60i32)
                    {
                        return ((i) as i8);
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return (-1i8);
    }
}
pub(crate) unsafe extern "C" fn DeactivateShowsWithUnseenSpecies() {
    unsafe {
        let mut i: u16 = 0u16;
        let mut species: u16 = 0u16;
        {
            i = 0u16;
            'l1: loop {
                if !(((i) as i32) < 24i32) {
                    break 'l1;
                }
                'l2: {
                    'l3: {
                        let __sw1 = ((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(10188))
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 36))
                        .read()) as i32);
                        let __matched = __sw1 == 8i32
                            || __sw1 == 10i32
                            || __sw1 == 1i32
                            || __sw1 == 3i32
                            || __sw1 == 4i32
                            || __sw1 == 5i32
                            || __sw1 == 6i32
                            || __sw1 == 7i32
                            || __sw1 == 21i32
                            || __sw1 == 23i32
                            || __sw1 == 24i32
                            || __sw1 == 25i32
                            || __sw1 == 30i32
                            || __sw1 == 31i32
                            || __sw1 == 33i32
                            || __sw1 == 36i32
                            || __sw1 == 0i32
                            || __sw1 == 2i32
                            || __sw1 == 9i32
                            || __sw1 == 26i32
                            || __sw1 == 27i32
                            || __sw1 == 28i32
                            || __sw1 == 29i32
                            || __sw1 == 34i32
                            || __sw1 == 35i32
                            || __sw1 == 22i32
                            || __sw1 == 11i32
                            || __sw1 == 12i32
                            || __sw1 == 32i32
                            || __sw1 == 37i32
                            || __sw1 == 38i32
                            || __sw1 == 39i32
                            || __sw1 == 41i32;
                        if __sw1 == 8i32 {
                            species = (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>())
                                .read())
                            .wrapping_add(10188))
                            .cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 36))
                            .wrapping_add(18)
                            .cast::<u16>())
                            .read();
                            DeactivateShowIfNotSeenSpecies(species, ((i) as u8));
                            species = (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>())
                                .read())
                            .wrapping_add(10188))
                            .cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 36))
                            .wrapping_add(2)
                            .cast::<u16>())
                            .read();
                            DeactivateShowIfNotSeenSpecies(species, ((i) as u8));
                            break 'l3;
                        }
                        if __sw1 == 10i32 {
                            species = (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>())
                                .read())
                            .wrapping_add(10188))
                            .cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 36))
                            .wrapping_add(22)
                            .cast::<u16>())
                            .read();
                            DeactivateShowIfNotSeenSpecies(species, ((i) as u8));
                            species = (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>())
                                .read())
                            .wrapping_add(10188))
                            .cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 36))
                            .wrapping_add(2)
                            .cast::<u16>())
                            .read();
                            DeactivateShowIfNotSeenSpecies(species, ((i) as u8));
                            break 'l3;
                        }
                        if __sw1 == 1i32 {
                            species = (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>())
                                .read())
                            .wrapping_add(10188))
                            .cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 36))
                            .wrapping_add(2)
                            .cast::<u16>())
                            .read();
                            DeactivateShowIfNotSeenSpecies(species, ((i) as u8));
                            break 'l3;
                        }
                        if __sw1 == 3i32 {
                            species = (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>())
                                .read())
                            .wrapping_add(10188))
                            .cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 36))
                            .wrapping_add(2)
                            .cast::<u16>())
                            .read();
                            DeactivateShowIfNotSeenSpecies(species, ((i) as u8));
                            break 'l3;
                        }
                        if __sw1 == 4i32 {
                            species = (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>())
                                .read())
                            .wrapping_add(10188))
                            .cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 36))
                            .wrapping_add(6)
                            .cast::<u16>())
                            .read();
                            DeactivateShowIfNotSeenSpecies(species, ((i) as u8));
                            break 'l3;
                        }
                        if __sw1 == 5i32 {
                            species = (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>())
                                .read())
                            .wrapping_add(10188))
                            .cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 36))
                            .wrapping_add(2)
                            .cast::<u16>())
                            .read();
                            DeactivateShowIfNotSeenSpecies(species, ((i) as u8));
                            species = (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>())
                                .read())
                            .wrapping_add(10188))
                            .cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 36))
                            .wrapping_add(28)
                            .cast::<u16>())
                            .read();
                            DeactivateShowIfNotSeenSpecies(species, ((i) as u8));
                            break 'l3;
                        }
                        if __sw1 == 6i32 {
                            species = (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>())
                                .read())
                            .wrapping_add(10188))
                            .cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 36))
                            .wrapping_add(2)
                            .cast::<u16>())
                            .read();
                            DeactivateShowIfNotSeenSpecies(species, ((i) as u8));
                            break 'l3;
                        }
                        if __sw1 == 7i32 {
                            species = (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>())
                                .read())
                            .wrapping_add(10188))
                            .cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 36))
                            .wrapping_add(10)
                            .cast::<u16>())
                            .read();
                            DeactivateShowIfNotSeenSpecies(species, ((i) as u8));
                            species = (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>())
                                .read())
                            .wrapping_add(10188))
                            .cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 36))
                            .wrapping_add(20)
                            .cast::<u16>())
                            .read();
                            DeactivateShowIfNotSeenSpecies(species, ((i) as u8));
                            break 'l3;
                        }
                        if __sw1 == 21i32 {
                            species = (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>())
                                .read())
                            .wrapping_add(10188))
                            .cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 36))
                            .wrapping_add(16)
                            .cast::<u16>())
                            .read();
                            DeactivateShowIfNotSeenSpecies(species, ((i) as u8));
                            break 'l3;
                        }
                        if __sw1 == 23i32 {
                            species = (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>())
                                .read())
                            .wrapping_add(10188))
                            .cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 36))
                            .wrapping_add(12)
                            .cast::<u16>())
                            .read();
                            DeactivateShowIfNotSeenSpecies(species, ((i) as u8));
                            species = (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>())
                                .read())
                            .wrapping_add(10188))
                            .cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 36))
                            .wrapping_add(14)
                            .cast::<u16>())
                            .read();
                            DeactivateShowIfNotSeenSpecies(species, ((i) as u8));
                            break 'l3;
                        }
                        if __sw1 == 24i32 {
                            species = (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>())
                                .read())
                            .wrapping_add(10188))
                            .cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 36))
                            .wrapping_add(4)
                            .cast::<u16>())
                            .read();
                            DeactivateShowIfNotSeenSpecies(species, ((i) as u8));
                            break 'l3;
                        }
                        if __sw1 == 25i32 {
                            species = (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>())
                                .read())
                            .wrapping_add(10188))
                            .cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 36))
                            .wrapping_add(8)
                            .cast::<u16>())
                            .read();
                            DeactivateShowIfNotSeenSpecies(species, ((i) as u8));
                            species = (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>())
                                .read())
                            .wrapping_add(10188))
                            .cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 36))
                            .wrapping_add(4)
                            .cast::<u16>())
                            .read();
                            DeactivateShowIfNotSeenSpecies(species, ((i) as u8));
                            break 'l3;
                        }
                        if __sw1 == 30i32 {
                            species = (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>())
                                .read())
                            .wrapping_add(10188))
                            .cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 36))
                            .wrapping_add(2)
                            .cast::<u16>())
                            .read();
                            DeactivateShowIfNotSeenSpecies(species, ((i) as u8));
                            species = (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>())
                                .read())
                            .wrapping_add(10188))
                            .cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 36))
                            .wrapping_add(10)
                            .cast::<u16>())
                            .read();
                            DeactivateShowIfNotSeenSpecies(species, ((i) as u8));
                            break 'l3;
                        }
                        if __sw1 == 31i32 {
                            species = (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>())
                                .read())
                            .wrapping_add(10188))
                            .cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 36))
                            .wrapping_add(8)
                            .cast::<u16>())
                            .read();
                            DeactivateShowIfNotSeenSpecies(species, ((i) as u8));
                            break 'l3;
                        }
                        if __sw1 == 33i32 {
                            species = (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>())
                                .read())
                            .wrapping_add(10188))
                            .cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 36))
                            .wrapping_add(6)
                            .cast::<u16>())
                            .read();
                            DeactivateShowIfNotSeenSpecies(species, ((i) as u8));
                            species = (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>())
                                .read())
                            .wrapping_add(10188))
                            .cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 36))
                            .wrapping_add(4)
                            .cast::<u16>())
                            .read();
                            DeactivateShowIfNotSeenSpecies(species, ((i) as u8));
                            break 'l3;
                        }
                        if __sw1 == 36i32 {
                            species = (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>())
                                .read())
                            .wrapping_add(10188))
                            .cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 36))
                            .wrapping_add(4)
                            .cast::<u16>())
                            .read();
                            DeactivateShowIfNotSeenSpecies(species, ((i) as u8));
                            species = (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>())
                                .read())
                            .wrapping_add(10188))
                            .cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 36))
                            .wrapping_add(6)
                            .cast::<u16>())
                            .read();
                            DeactivateShowIfNotSeenSpecies(species, ((i) as u8));
                            species = (((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>())
                                .read())
                            .wrapping_add(10188))
                            .cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 36))
                            .wrapping_add(13))
                            .read()) as u16);
                            'l4: {
                                let __sw2 = ((species) as i32);
                                if __sw2 == 3i32 || __sw2 == 4i32 {
                                    break 'l4;
                                }
                                if __sw2 == 1i32
                                    || __sw2 == 5i32
                                    || __sw2 == 6i32
                                    || __sw2 == 7i32
                                    || __sw2 == 8i32
                                    || __sw2 == 9i32
                                    || __sw2 == 10i32
                                    || __sw2 == 11i32
                                    || __sw2 == 12i32
                                    || __sw2 == 13i32
                                {
                                    species = (((((((&raw mut gSaveBlock1Ptr)
                                        .cast::<*mut u8>())
                                    .read())
                                    .wrapping_add(10188))
                                    .cast::<u8>())
                                    .wrapping_offset(((i) as i32) as isize * 36))
                                    .wrapping_add(8)
                                    .cast::<u16>())
                                    .read();
                                    DeactivateShowIfNotSeenSpecies(species, ((i) as u8));
                                    break 'l4;
                                }
                                if __sw2 == 2i32 {
                                    species = (((((((&raw mut gSaveBlock1Ptr)
                                        .cast::<*mut u8>())
                                    .read())
                                    .wrapping_add(10188))
                                    .cast::<u8>())
                                    .wrapping_offset(((i) as i32) as isize * 36))
                                    .wrapping_add(8)
                                    .cast::<u16>())
                                    .read();
                                    DeactivateShowIfNotSeenSpecies(species, ((i) as u8));
                                    species = (((((((&raw mut gSaveBlock1Ptr)
                                        .cast::<*mut u8>())
                                    .read())
                                    .wrapping_add(10188))
                                    .cast::<u8>())
                                    .wrapping_offset(((i) as i32) as isize * 36))
                                    .wrapping_add(10)
                                    .cast::<u16>())
                                    .read();
                                    DeactivateShowIfNotSeenSpecies(species, ((i) as u8));
                                    break 'l4;
                                }
                            }
                            break 'l3;
                        }
                        if __sw1 == 0i32
                            || __sw1 == 2i32
                            || __sw1 == 9i32
                            || __sw1 == 26i32
                            || __sw1 == 27i32
                            || __sw1 == 28i32
                            || __sw1 == 29i32
                            || __sw1 == 34i32
                            || __sw1 == 35i32
                            || __sw1 == 22i32
                            || __sw1 == 11i32
                            || __sw1 == 12i32
                            || __sw1 == 32i32
                            || __sw1 == 37i32
                            || __sw1 == 38i32
                            || __sw1 == 39i32
                            || __sw1 == 41i32
                        {
                            break 'l3;
                        }
                        if !__matched {
                            DeactivateShow(((i) as u8));
                            break 'l3;
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn DeactivateShow(showIdx: u8) {
    unsafe {
        let mut showIdx = showIdx;
        (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(10188))
            .cast::<u8>())
        .wrapping_offset(((showIdx) as i32) as isize * 36))
        .wrapping_add(1))
        .write(0u8);
    }
}
pub(crate) unsafe extern "C" fn DeactivateShowIfNotSeenSpecies(species: u16, showIdx: u8) {
    unsafe {
        let mut species = species;
        let mut showIdx = showIdx;
        if !((GetSetPokedexFlag(SpeciesToNationalPokedexNum(species), 0u8)) != 0) {
            (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(10188))
                .cast::<u8>())
            .wrapping_offset(((showIdx) as i32) as isize * 36))
            .wrapping_add(1))
            .write(0u8);
        }
    }
}
pub(crate) unsafe extern "C" fn DeactivateGameCompleteShowsIfNotUnlocked() {
    unsafe {
        let mut i: u16 = 0u16;
        if ((FlagGet(2148u16)) as i32) != 1i32 {
            {
                i = 0u16;
                'l1: loop {
                    if !(((i) as i32) < 24i32) {
                        break 'l1;
                    }
                    'l2: {
                        if ((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(10188))
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 36))
                        .read()) as i32)
                            == 7i32
                        {
                            (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(10188))
                            .cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 36))
                            .wrapping_add(1))
                            .write(0u8);
                        } else {
                            if ((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(10188))
                            .cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 36))
                            .read()) as i32)
                                == 41i32
                            {
                                (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                                    .wrapping_add(10188))
                                .cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize * 36))
                                .wrapping_add(1))
                                .write(0u8);
                            }
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DeactivateAllNormalTVShows() {
    unsafe {
        let mut i: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 5i32) {
                    break 'l1;
                }
                'l2: {
                    if ((GetTVGroupByShowId(
                        ((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(10188))
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 36))
                        .read(),
                    )) as i32)
                        == 2i32
                    {
                        (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(10188))
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 36))
                        .wrapping_add(1))
                        .write(0u8);
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn DeleteExcessMixedShows() {
    unsafe {
        let mut i: i8 = 0i8;
        let mut numEmptyMixSlots: i8 = 0i8;
        {
            i = 5i8;
            'l1: loop {
                if !(((i) as i32) < 24i32) {
                    break 'l1;
                }
                'l2: {
                    if ((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(10188))
                    .cast::<u8>())
                    .wrapping_offset(((i) as i32) as isize * 36))
                    .read()) as i32)
                        == 0i32
                    {
                        numEmptyMixSlots = (numEmptyMixSlots).wrapping_add(1);
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        {
            i = 0i8;
            'l3: loop {
                if !(((i) as i32) < (5i32).wrapping_sub(((numEmptyMixSlots) as i32))) {
                    break 'l3;
                }
                'l4: {
                    DeleteTVShowInArrayByIdx(
                        ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(10188))
                        .cast::<u8>(),
                        ((((i) as i32).wrapping_add(5i32)) as u8),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ReceivePokeNewsData(src: *mut u8, size: u32, playersLinkId: u8) {
    unsafe {
        let mut src = src;
        let mut size = size;
        let mut playersLinkId = playersLinkId;
        let mut i: u8 = 0u8;
        let mut rmBuffer2: *mut u8 = core::ptr::null_mut();
        let mut rmBuffer: *mut u8 = core::ptr::null_mut();
        rmBuffer2 = Alloc(256u32);
        if ((rmBuffer2) as usize) != 0usize {
            {
                i = 0u8;
                'l1: loop {
                    if !(((i) as i32) < 4i32) {
                        break 'l1;
                    }
                    'l2: {
                        crate::c::memcpy(
                            (((rmBuffer2).cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize * 64))
                            .cast::<u8>(),
                            (src).wrapping_offset(
                                ((((i) as u32).wrapping_mul(size)) as i32) as isize * 1,
                            ),
                            64u32,
                        );
                    }
                    i = (i).wrapping_add(1);
                }
            }
            rmBuffer = rmBuffer2;
            'l3: {
                let __sw1 = ((playersLinkId) as i32);
                if __sw1 == 0i32 {
                    SetMixedPokeNews(
                        ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(11088))
                        .cast::<u8>(),
                        (((rmBuffer).cast::<u8>()).wrapping_offset(64)).cast::<u8>(),
                        (((rmBuffer).cast::<u8>()).wrapping_offset(128)).cast::<u8>(),
                        (((rmBuffer).cast::<u8>()).wrapping_offset(192)).cast::<u8>(),
                    );
                    break 'l3;
                }
                if __sw1 == 1i32 {
                    SetMixedPokeNews(
                        ((rmBuffer).cast::<u8>()).cast::<u8>(),
                        ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(11088))
                        .cast::<u8>(),
                        (((rmBuffer).cast::<u8>()).wrapping_offset(128)).cast::<u8>(),
                        (((rmBuffer).cast::<u8>()).wrapping_offset(192)).cast::<u8>(),
                    );
                    break 'l3;
                }
                if __sw1 == 2i32 {
                    SetMixedPokeNews(
                        ((rmBuffer).cast::<u8>()).cast::<u8>(),
                        (((rmBuffer).cast::<u8>()).wrapping_offset(64)).cast::<u8>(),
                        ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(11088))
                        .cast::<u8>(),
                        (((rmBuffer).cast::<u8>()).wrapping_offset(192)).cast::<u8>(),
                    );
                    break 'l3;
                }
                if __sw1 == 3i32 {
                    SetMixedPokeNews(
                        ((rmBuffer).cast::<u8>()).cast::<u8>(),
                        (((rmBuffer).cast::<u8>()).wrapping_offset(64)).cast::<u8>(),
                        (((rmBuffer).cast::<u8>()).wrapping_offset(128)).cast::<u8>(),
                        ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(11088))
                        .cast::<u8>(),
                    );
                    break 'l3;
                }
            }
            ClearInvalidPokeNews();
            ClearPokeNewsIfGameNotComplete();
            Free(rmBuffer2);
        }
    }
}
pub(crate) unsafe extern "C" fn SetMixedPokeNews(
    player1: *mut u8,
    player2: *mut u8,
    player3: *mut u8,
    player4: *mut u8,
) {
    unsafe {
        let mut player1 = player1;
        let mut player2 = player2;
        let mut player3 = player3;
        let mut player4 = player4;
        let mut i: u8 = 0u8;
        let mut j: u8 = 0u8;
        let mut k: u8 = 0u8;
        let mut pokeNews = crate::ffi::Align4([0u8; 16]);
        ((&raw mut pokeNews).cast::<*mut *mut u8>()).write(&raw mut player1);
        (((&raw mut pokeNews).cast::<*mut *mut u8>()).wrapping_offset(1)).write(&raw mut player2);
        (((&raw mut pokeNews).cast::<*mut *mut u8>()).wrapping_offset(2)).write(&raw mut player3);
        (((&raw mut pokeNews).cast::<*mut *mut u8>()).wrapping_offset(3)).write(&raw mut player4);
        ((&raw mut sTVShowNewsMixingNumPlayers)
            .cast::<u8>()
            .cast::<u8>())
        .write(GetLinkPlayerCount());
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 16i32) {
                    break 'l1;
                }
                'l2: {
                    {
                        j = 0u8;
                        'l3: loop {
                            if !(((j) as i32)
                                < ((((&raw mut sTVShowNewsMixingNumPlayers)
                                    .cast::<u8>()
                                    .cast::<u8>())
                                .read()) as i32))
                            {
                                break 'l3;
                            }
                            'l4: {
                                ((&raw mut sTVShowMixingCurSlot).cast::<u8>().cast::<i8>()).write(
                                    GetPokeNewsSlotIfActive(
                                        ((((&raw mut pokeNews).cast::<*mut *mut u8>())
                                            .wrapping_offset(((j) as i32) as isize))
                                        .read())
                                        .read(),
                                        i,
                                    ),
                                );
                                if ((((&raw mut sTVShowMixingCurSlot).cast::<u8>().cast::<i8>())
                                    .read()) as i32)
                                    != (-1i32)
                                {
                                    {
                                        k = 0u8;
                                        'l5: loop {
                                            if !(((k) as i32)
                                                < ((((&raw mut sTVShowNewsMixingNumPlayers)
                                                    .cast::<u8>()
                                                    .cast::<u8>())
                                                .read())
                                                    as i32)
                                                    .wrapping_sub(1i32))
                                            {
                                                break 'l5;
                                            }
                                            'l6: {
                                                ((&raw mut sCurTVShowSlot).cast::<u8>().cast::<i8>()).write(GetFirstEmptyPokeNewsSlot(((((&raw mut pokeNews).cast::<*mut *mut u8>()).wrapping_offset((crate::c::rem_i32((((((j) as i32))).wrapping_add((((k) as i32)))).wrapping_add(1i32), (((((&raw mut sTVShowNewsMixingNumPlayers).cast::<u8>().cast::<u8>()).read()) as i32)))) as isize)).read()).read()));
                                                if ((((&raw mut sCurTVShowSlot)
                                                    .cast::<u8>()
                                                    .cast::<i8>())
                                                .read())
                                                    as i32)
                                                    != (-1i32)
                                                {
                                                    InitTryMixPokeNewsShow((((&raw mut pokeNews).cast::<*mut *mut u8>()).wrapping_offset((crate::c::rem_i32((((((j) as i32))).wrapping_add((((k) as i32)))).wrapping_add(1i32), (((((&raw mut sTVShowNewsMixingNumPlayers).cast::<u8>().cast::<u8>()).read()) as i32)))) as isize)).read(), (((&raw mut pokeNews).cast::<*mut *mut u8>()).wrapping_offset((((j) as i32)) as isize)).read());
                                                }
                                            }
                                            k = (k).wrapping_add(1);
                                        }
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
    }
}
pub(crate) unsafe extern "C" fn InitTryMixPokeNewsShow(dest: *mut *mut u8, src: *mut *mut u8) {
    unsafe {
        let mut dest = dest;
        let mut src = src;
        let mut ptr1: *mut u8 = (dest).read();
        let mut ptr2: *mut u8 = (src).read();
        ptr2 = (ptr2).wrapping_offset(
            ((((&raw mut sTVShowMixingCurSlot).cast::<u8>().cast::<i8>()).read()) as i32) as isize
                * 4,
        );
        TryMixPokeNewsShow(
            ptr1,
            ptr2,
            ((&raw mut sCurTVShowSlot).cast::<u8>().cast::<i8>()).read(),
        );
    }
}
pub(crate) unsafe extern "C" fn TryMixPokeNewsShow(dest: *mut u8, src: *mut u8, slot: i8) -> u8 {
    unsafe {
        let mut dest = dest;
        let mut src = src;
        let mut slot = slot;
        let mut i: u8 = 0u8;
        if (((src).read()) as i32) == 0i32 {
            return 0u8;
        }
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 16i32) {
                    break 'l1;
                }
                'l2: {
                    if ((((dest).wrapping_offset(((i) as i32) as isize * 4)).read()) as i32)
                        == (((src).read()) as i32)
                    {
                        return 0u8;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        ((dest).wrapping_offset(((slot) as i32) as isize * 4)).write((src).read());
        (((dest).wrapping_offset(((slot) as i32) as isize * 4)).wrapping_add(1)).write(1u8);
        (((dest).wrapping_offset(((slot) as i32) as isize * 4))
            .wrapping_add(2)
            .cast::<u16>())
        .write(((src).wrapping_add(2).cast::<u16>()).read());
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn GetPokeNewsSlotIfActive(pokeNews: *mut u8, idx: u8) -> i8 {
    unsafe {
        let mut pokeNews = pokeNews;
        let mut idx = idx;
        if ((((pokeNews).wrapping_offset(((idx) as i32) as isize * 4)).read()) as i32) == 0i32 {
            return (-1i8);
        }
        return ((idx) as i8);
    }
}
pub(crate) unsafe extern "C" fn ClearInvalidPokeNews() {
    unsafe {
        let mut i: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 16i32) {
                    break 'l1;
                }
                'l2: {
                    if ((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(11088))
                    .cast::<u8>())
                    .wrapping_offset(((i) as i32) as isize * 4))
                    .read()) as i32)
                        > 4i32
                    {
                        ClearPokeNewsBySlot(i);
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        CompactPokeNews();
    }
}
pub(crate) unsafe extern "C" fn ClearPokeNewsIfGameNotComplete() {
    unsafe {
        let mut i: u8 = 0u8;
        if ((FlagGet(2148u16)) as i32) != 1i32 {
            {
                i = 0u8;
                'l1: loop {
                    if !(((i) as i32) < 16i32) {
                        break 'l1;
                    }
                    'l2: {
                        (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(11088))
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 4))
                        .wrapping_add(1))
                        .write(0u8);
                    }
                    i = (i).wrapping_add(1);
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn TranslateShowNames(show: *mut u8, language: u32) {
    unsafe {
        let mut show = show;
        let mut language = language;
        let mut i: i32 = 0i32;
        let mut shows: *mut *mut u8 = core::ptr::null_mut();
        shows = (AllocZeroed(44u32)).cast::<*mut u8>();
        {
            i = 0i32;
            'l1: loop {
                if !(i < 24i32) {
                    break 'l1;
                }
                'l2: {
                    'l3: {
                        let __sw1 = ((((show).wrapping_offset((i) as isize * 36)).read()) as i32);
                        if __sw1 == 1i32 || __sw1 == 2i32 {
                            (shows).write((show).wrapping_offset((i) as isize * 36));
                            if (IsStringJapanese((((shows).read()).wrapping_add(16)).cast::<u8>()))
                                != 0
                            {
                                (((shows).read()).wrapping_add(24)).write(1u8);
                            } else {
                                (((shows).read()).wrapping_add(24)).write(((language) as u8));
                            }
                            break 'l3;
                        }
                        if __sw1 == 3i32 {
                            ((shows).wrapping_offset(1))
                                .write((show).wrapping_offset((i) as isize * 36));
                            if (IsStringJapanese(
                                ((((shows).wrapping_offset(1)).read()).wrapping_add(5))
                                    .cast::<u8>(),
                            )) != 0
                            {
                                ((((shows).wrapping_offset(1)).read()).wrapping_add(13)).write(1u8);
                            } else {
                                ((((shows).wrapping_offset(1)).read()).wrapping_add(13))
                                    .write(((language) as u8));
                            }
                            if (IsStringJapanese(
                                ((((shows).wrapping_offset(1)).read()).wrapping_add(16))
                                    .cast::<u8>(),
                            )) != 0
                            {
                                ((((shows).wrapping_offset(1)).read()).wrapping_add(14)).write(1u8);
                            } else {
                                ((((shows).wrapping_offset(1)).read()).wrapping_add(14))
                                    .write(((language) as u8));
                            }
                            break 'l3;
                        }
                        if __sw1 == 21i32 {
                            ((shows).wrapping_offset(6))
                                .write((show).wrapping_offset((i) as isize * 36));
                            if (IsStringJapanese(
                                ((((shows).wrapping_offset(6)).read()).wrapping_add(19))
                                    .cast::<u8>(),
                            )) != 0
                            {
                                ((((shows).wrapping_offset(6)).read()).wrapping_add(2)).write(1u8);
                            } else {
                                ((((shows).wrapping_offset(6)).read()).wrapping_add(2))
                                    .write(((language) as u8));
                            }
                            if (IsStringJapanese(
                                ((((shows).wrapping_offset(6)).read()).wrapping_add(4))
                                    .cast::<u8>(),
                            )) != 0
                            {
                                ((((shows).wrapping_offset(6)).read()).wrapping_add(3)).write(1u8);
                            } else {
                                ((((shows).wrapping_offset(6)).read()).wrapping_add(3))
                                    .write(((language) as u8));
                            }
                            break 'l3;
                        }
                        if __sw1 == 22i32 {
                            ((shows).wrapping_offset(7))
                                .write((show).wrapping_offset((i) as isize * 36));
                            if (IsStringJapanese(
                                ((((shows).wrapping_offset(7)).read()).wrapping_add(19))
                                    .cast::<u8>(),
                            )) != 0
                            {
                                ((((shows).wrapping_offset(7)).read()).wrapping_add(3)).write(1u8);
                            } else {
                                ((((shows).wrapping_offset(7)).read()).wrapping_add(3))
                                    .write(((language) as u8));
                            }
                            break 'l3;
                        }
                        if __sw1 == 7i32 {
                            ((shows).wrapping_offset(5))
                                .write((show).wrapping_offset((i) as isize * 36));
                            if (IsStringJapanese(
                                ((((shows).wrapping_offset(5)).read()).wrapping_add(2))
                                    .cast::<u8>(),
                            )) != 0
                            {
                                ((((shows).wrapping_offset(5)).read()).wrapping_add(29)).write(1u8);
                            } else {
                                ((((shows).wrapping_offset(5)).read()).wrapping_add(29))
                                    .write(((language) as u8));
                            }
                            if (IsStringJapanese(
                                ((((shows).wrapping_offset(5)).read()).wrapping_add(12))
                                    .cast::<u8>(),
                            )) != 0
                            {
                                ((((shows).wrapping_offset(5)).read()).wrapping_add(30)).write(1u8);
                            } else {
                                ((((shows).wrapping_offset(5)).read()).wrapping_add(30))
                                    .write(((language) as u8));
                            }
                            break 'l3;
                        }
                        if __sw1 == 6i32 {
                            ((shows).wrapping_offset(4))
                                .write((show).wrapping_offset((i) as isize * 36));
                            if (IsStringJapanese(
                                ((((shows).wrapping_offset(4)).read()).wrapping_add(22))
                                    .cast::<u8>(),
                            )) != 0
                            {
                                ((((shows).wrapping_offset(4)).read()).wrapping_add(30)).write(1u8);
                            } else {
                                ((((shows).wrapping_offset(4)).read()).wrapping_add(30))
                                    .write(((language) as u8));
                            }
                            if (IsStringJapanese(
                                ((((shows).wrapping_offset(4)).read()).wrapping_add(8))
                                    .cast::<u8>(),
                            )) != 0
                            {
                                ((((shows).wrapping_offset(4)).read()).wrapping_add(31)).write(1u8);
                            } else {
                                ((((shows).wrapping_offset(4)).read()).wrapping_add(31))
                                    .write(((language) as u8));
                            }
                            break 'l3;
                        }
                        if __sw1 == 5i32 {
                            ((shows).wrapping_offset(3))
                                .write((show).wrapping_offset((i) as isize * 36));
                            if (IsStringJapanese(
                                ((((shows).wrapping_offset(3)).read()).wrapping_add(15))
                                    .cast::<u8>(),
                            )) != 0
                            {
                                ((((shows).wrapping_offset(3)).read()).wrapping_add(30)).write(1u8);
                            } else {
                                ((((shows).wrapping_offset(3)).read()).wrapping_add(30))
                                    .write(((language) as u8));
                            }
                            if (IsStringJapanese(
                                ((((shows).wrapping_offset(3)).read()).wrapping_add(4))
                                    .cast::<u8>(),
                            )) != 0
                            {
                                ((((shows).wrapping_offset(3)).read()).wrapping_add(31)).write(1u8);
                            } else {
                                ((((shows).wrapping_offset(3)).read()).wrapping_add(31))
                                    .write(((language) as u8));
                            }
                            break 'l3;
                        }
                        if __sw1 == 23i32 {
                            ((shows).wrapping_offset(2))
                                .write((show).wrapping_offset((i) as isize * 36));
                            if (IsStringJapanese(
                                ((((shows).wrapping_offset(2)).read()).wrapping_add(19))
                                    .cast::<u8>(),
                            )) != 0
                            {
                                ((((shows).wrapping_offset(2)).read()).wrapping_add(2)).write(1u8);
                            } else {
                                ((((shows).wrapping_offset(2)).read()).wrapping_add(2))
                                    .write(((language) as u8));
                            }
                            break 'l3;
                        }
                        if __sw1 == 24i32 {
                            ((shows).wrapping_offset(8))
                                .write((show).wrapping_offset((i) as isize * 36));
                            if (IsStringJapanese(
                                ((((shows).wrapping_offset(8)).read()).wrapping_add(19))
                                    .cast::<u8>(),
                            )) != 0
                            {
                                ((((shows).wrapping_offset(8)).read()).wrapping_add(6)).write(1u8);
                            } else {
                                ((((shows).wrapping_offset(8)).read()).wrapping_add(6))
                                    .write(((language) as u8));
                            }
                            break 'l3;
                        }
                        if __sw1 == 25i32 {
                            ((shows).wrapping_offset(9))
                                .write((show).wrapping_offset((i) as isize * 36));
                            if (IsStringJapanese(
                                ((((shows).wrapping_offset(9)).read()).wrapping_add(19))
                                    .cast::<u8>(),
                            )) != 0
                            {
                                ((((shows).wrapping_offset(9)).read()).wrapping_add(11)).write(1u8);
                            } else {
                                ((((shows).wrapping_offset(9)).read()).wrapping_add(11))
                                    .write(((language) as u8));
                            }
                            break 'l3;
                        }
                        if __sw1 == 41i32 {
                            ((shows).wrapping_offset(10))
                                .write((show).wrapping_offset((i) as isize * 36));
                            ((((shows).wrapping_offset(10)).read()).wrapping_add(24))
                                .write(((language) as u8));
                            break 'l3;
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        Free((shows).cast::<u8>());
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SanitizeTVShowsForRuby(shows: *mut u8) {
    unsafe {
        let mut shows = shows;
        let mut curShow: *mut u8 = core::ptr::null_mut();
        SanitizeTVShowLocationsForRuby(shows);
        {
            curShow = shows;
            'l1: loop {
                if !(((curShow) as usize) < (((shows).wrapping_offset(864)) as usize)) {
                    break 'l1;
                }
                'l2: {
                    if (((curShow).read()) as i32) == 7i32 {
                        if ((((((curShow).wrapping_add(29)).read()) as i32) == 1i32)
                            && (((((curShow).wrapping_add(30)).read()) as i32) != 1i32))
                            || ((((((curShow).wrapping_add(29)).read()) as i32) != 1i32)
                                && (((((curShow).wrapping_add(30)).read()) as i32) == 1i32))
                        {
                            crate::c::memset(curShow, 0i32, 36u32);
                        }
                    }
                }
                curShow = (curShow).wrapping_offset(36);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn TranslateRubyShows(shows: *mut u8) {
    unsafe {
        let mut shows = shows;
        let mut curShow: *mut u8 = core::ptr::null_mut();
        {
            curShow = shows;
            'l1: loop {
                if !(((curShow) as usize) < (((shows).wrapping_offset(864)) as usize)) {
                    break 'l1;
                }
                'l2: {
                    if (((curShow).read()) as i32) == 7i32 {
                        if (IsStringJapanese(((curShow).wrapping_add(12)).cast::<u8>())) != 0 {
                            ((curShow).wrapping_add(30)).write(1u8);
                        } else {
                            ((curShow).wrapping_add(30)).write(2u8);
                        }
                    }
                }
                curShow = (curShow).wrapping_offset(36);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn GetStringLanguage(str: *mut u8) -> u8 {
    unsafe {
        let mut str = str;
        return ((if (IsStringJapanese(str)) != 0 {
            1i32
        } else {
            2i32
        }) as u8);
    }
}
pub(crate) unsafe extern "C" fn TranslateJapaneseEmeraldShows(shows: *mut u8) {
    unsafe {
        let mut shows = shows;
        let mut curShow: *mut u8 = core::ptr::null_mut();
        {
            curShow = shows;
            'l1: loop {
                if !(((curShow) as usize) < (((shows).wrapping_offset(864)) as usize)) {
                    break 'l1;
                }
                'l2: {
                    'l3: {
                        let __sw1 = (((curShow).read()) as i32);
                        if __sw1 == 1i32 {
                            ((curShow).wrapping_add(24)).write(GetStringLanguage(
                                ((curShow).wrapping_add(16)).cast::<u8>(),
                            ));
                            break 'l3;
                        }
                        if __sw1 == 2i32 {
                            ((curShow).wrapping_add(24)).write(GetStringLanguage(
                                ((curShow).wrapping_add(16)).cast::<u8>(),
                            ));
                            break 'l3;
                        }
                        if __sw1 == 3i32 {
                            ((curShow).wrapping_add(13))
                                .write(GetStringLanguage(((curShow).wrapping_add(5)).cast::<u8>()));
                            ((curShow).wrapping_add(14)).write(GetStringLanguage(
                                ((curShow).wrapping_add(16)).cast::<u8>(),
                            ));
                            break 'l3;
                        }
                        if __sw1 == 4i32 {
                            ((curShow).wrapping_add(23)).write(GetStringLanguage(
                                ((curShow).wrapping_add(11)).cast::<u8>(),
                            ));
                            break 'l3;
                        }
                        if __sw1 == 5i32 {
                            ((curShow).wrapping_add(30)).write(GetStringLanguage(
                                ((curShow).wrapping_add(15)).cast::<u8>(),
                            ));
                            ((curShow).wrapping_add(31))
                                .write(GetStringLanguage(((curShow).wrapping_add(4)).cast::<u8>()));
                            break 'l3;
                        }
                        if __sw1 == 6i32 {
                            ((curShow).wrapping_add(30)).write(GetStringLanguage(
                                ((curShow).wrapping_add(22)).cast::<u8>(),
                            ));
                            ((curShow).wrapping_add(31))
                                .write(GetStringLanguage(((curShow).wrapping_add(8)).cast::<u8>()));
                            break 'l3;
                        }
                        if __sw1 == 7i32 {
                            ((curShow).wrapping_add(29))
                                .write(GetStringLanguage(((curShow).wrapping_add(2)).cast::<u8>()));
                            ((curShow).wrapping_add(30)).write(GetStringLanguage(
                                ((curShow).wrapping_add(12)).cast::<u8>(),
                            ));
                            break 'l3;
                        }
                        if __sw1 == 8i32 {
                            ((curShow).wrapping_add(29)).write(GetStringLanguage(
                                ((curShow).wrapping_add(20)).cast::<u8>(),
                            ));
                            ((curShow).wrapping_add(30))
                                .write(GetStringLanguage(((curShow).wrapping_add(4)).cast::<u8>()));
                            break 'l3;
                        }
                        if __sw1 == 9i32 {
                            ((curShow).wrapping_add(20)).write(GetStringLanguage(
                                ((curShow).wrapping_add(12)).cast::<u8>(),
                            ));
                            ((curShow).wrapping_add(21))
                                .write(GetStringLanguage(((curShow).wrapping_add(4)).cast::<u8>()));
                            break 'l3;
                        }
                        if __sw1 == 10i32 {
                            ((curShow).wrapping_add(25))
                                .write(GetStringLanguage(((curShow).wrapping_add(4)).cast::<u8>()));
                            ((curShow).wrapping_add(26)).write(GetStringLanguage(
                                ((curShow).wrapping_add(12)).cast::<u8>(),
                            ));
                            break 'l3;
                        }
                        if __sw1 == 11i32 {
                            ((curShow).wrapping_add(23))
                                .write(GetStringLanguage(((curShow).wrapping_add(2)).cast::<u8>()));
                            ((curShow).wrapping_add(24)).write(GetStringLanguage(
                                ((curShow).wrapping_add(12)).cast::<u8>(),
                            ));
                            break 'l3;
                        }
                        if __sw1 == 12i32 {
                            ((curShow).wrapping_add(23))
                                .write(GetStringLanguage(((curShow).wrapping_add(2)).cast::<u8>()));
                            ((curShow).wrapping_add(24)).write(GetStringLanguage(
                                ((curShow).wrapping_add(11)).cast::<u8>(),
                            ));
                            break 'l3;
                        }
                        if __sw1 == 21i32 {
                            ((curShow).wrapping_add(2)).write(GetStringLanguage(
                                ((curShow).wrapping_add(19)).cast::<u8>(),
                            ));
                            ((curShow).wrapping_add(3))
                                .write(GetStringLanguage(((curShow).wrapping_add(4)).cast::<u8>()));
                            break 'l3;
                        }
                        if __sw1 == 22i32 {
                            ((curShow).wrapping_add(3)).write(GetStringLanguage(
                                ((curShow).wrapping_add(19)).cast::<u8>(),
                            ));
                            break 'l3;
                        }
                        if __sw1 == 23i32 {
                            ((curShow).wrapping_add(2)).write(GetStringLanguage(
                                ((curShow).wrapping_add(19)).cast::<u8>(),
                            ));
                            break 'l3;
                        }
                        if __sw1 == 24i32 {
                            ((curShow).wrapping_add(6)).write(GetStringLanguage(
                                ((curShow).wrapping_add(19)).cast::<u8>(),
                            ));
                            break 'l3;
                        }
                        if __sw1 == 25i32 {
                            ((curShow).wrapping_add(11)).write(GetStringLanguage(
                                ((curShow).wrapping_add(19)).cast::<u8>(),
                            ));
                            break 'l3;
                        }
                        if __sw1 == 27i32 {
                            ((curShow).wrapping_add(9)).write(GetStringLanguage(
                                ((curShow).wrapping_add(19)).cast::<u8>(),
                            ));
                            break 'l3;
                        }
                        if __sw1 == 30i32 {
                            ((curShow).wrapping_add(14)).write(GetStringLanguage(
                                ((curShow).wrapping_add(19)).cast::<u8>(),
                            ));
                            break 'l3;
                        }
                        if __sw1 == 33i32 {
                            ((curShow).wrapping_add(17)).write(GetStringLanguage(
                                ((curShow).wrapping_add(19)).cast::<u8>(),
                            ));
                            break 'l3;
                        }
                        if __sw1 == 29i32 || __sw1 == 34i32 {
                            ((curShow).wrapping_add(8)).write(GetStringLanguage(
                                ((curShow).wrapping_add(19)).cast::<u8>(),
                            ));
                            break 'l3;
                        }
                        if __sw1 == 35i32 {
                            ((curShow).wrapping_add(15)).write(GetStringLanguage(
                                ((curShow).wrapping_add(19)).cast::<u8>(),
                            ));
                            ((curShow).wrapping_add(16))
                                .write(GetStringLanguage(((curShow).wrapping_add(4)).cast::<u8>()));
                            break 'l3;
                        }
                        if __sw1 == 26i32 || __sw1 == 31i32 || __sw1 == 36i32 {
                            ((curShow).wrapping_add(12)).write(GetStringLanguage(
                                ((curShow).wrapping_add(19)).cast::<u8>(),
                            ));
                            break 'l3;
                        }
                        if __sw1 == 28i32 || __sw1 == 32i32 || __sw1 == 37i32 {
                            ((curShow).wrapping_add(5)).write(GetStringLanguage(
                                ((curShow).wrapping_add(19)).cast::<u8>(),
                            ));
                            break 'l3;
                        }
                        if __sw1 == 38i32 {
                            ((curShow).wrapping_add(27)).write(GetStringLanguage(
                                ((curShow).wrapping_add(19)).cast::<u8>(),
                            ));
                            ((curShow).wrapping_add(28))
                                .write(GetStringLanguage(((curShow).wrapping_add(4)).cast::<u8>()));
                            break 'l3;
                        }
                        if __sw1 == 39i32 {
                            ((curShow).wrapping_add(4)).write(GetStringLanguage(
                                ((curShow).wrapping_add(19)).cast::<u8>(),
                            ));
                            break 'l3;
                        }
                        if __sw1 == 41i32 {
                            break 'l3;
                        }
                    }
                }
                curShow = (curShow).wrapping_offset(36);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SanitizeTVShowLocationsForRuby(shows: *mut u8) {
    unsafe {
        let mut shows = shows;
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 24i32) {
                    break 'l1;
                }
                'l2: {
                    'l3: {
                        let __sw1 = ((((shows).wrapping_offset((i) as isize * 36)).read()) as i32);
                        if __sw1 == 25i32 {
                            if (((((shows).wrapping_offset((i) as isize * 36)).wrapping_add(10))
                                .read()) as i32)
                                > 88i32
                            {
                                crate::c::memset(
                                    (shows).wrapping_offset((i) as isize * 36),
                                    0i32,
                                    36u32,
                                );
                            }
                            break 'l3;
                        }
                        if __sw1 == 23i32 {
                            if (((((shows).wrapping_offset((i) as isize * 36)).wrapping_add(18))
                                .read()) as i32)
                                > 88i32
                            {
                                crate::c::memset(
                                    (shows).wrapping_offset((i) as isize * 36),
                                    0i32,
                                    36u32,
                                );
                            }
                            break 'l3;
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DoTVShow() {
    unsafe {
        if ((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(10188))
            .cast::<u8>())
        .wrapping_offset(
            ((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32) as isize * 36,
        ))
        .wrapping_add(1))
        .read())
            != 0
        {
            'l1: {
                let __sw1 = ((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                    .wrapping_add(10188))
                .cast::<u8>())
                .wrapping_offset(
                    ((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32) as isize * 36,
                ))
                .read()) as i32);
                if __sw1 == 1i32 {
                    DoTVShowPokemonFanClubLetter();
                    break 'l1;
                }
                if __sw1 == 2i32 {
                    DoTVShowRecentHappenings();
                    break 'l1;
                }
                if __sw1 == 3i32 {
                    DoTVShowPokemonFanClubOpinions();
                    break 'l1;
                }
                if __sw1 == 4i32 {
                    DoTVShowDummiedOut();
                    break 'l1;
                }
                if __sw1 == 41i32 {
                    DoTVShowPokemonNewsMassOutbreak();
                    break 'l1;
                }
                if __sw1 == 6i32 {
                    DoTVShowBravoTrainerPokemonProfile();
                    break 'l1;
                }
                if __sw1 == 7i32 {
                    DoTVShowBravoTrainerBattleTower();
                    break 'l1;
                }
                if __sw1 == 21i32 {
                    DoTVShowPokemonTodaySuccessfulCapture();
                    break 'l1;
                }
                if __sw1 == 22i32 {
                    DoTVShowTodaysSmartShopper();
                    break 'l1;
                }
                if __sw1 == 5i32 {
                    DoTVShowTheNameRaterShow();
                    break 'l1;
                }
                if __sw1 == 8i32 {
                    DoTVShowPokemonContestLiveUpdates();
                    break 'l1;
                }
                if __sw1 == 10i32 {
                    DoTVShowPokemonBattleUpdate();
                    break 'l1;
                }
                if __sw1 == 9i32 {
                    DoTVShow3CheersForPokeblocks();
                    break 'l1;
                }
                if __sw1 == 23i32 {
                    DoTVShowPokemonTodayFailedCapture();
                    break 'l1;
                }
                if __sw1 == 24i32 {
                    DoTVShowPokemonAngler();
                    break 'l1;
                }
                if __sw1 == 25i32 {
                    DoTVShowTheWorldOfMasters();
                    break 'l1;
                }
                if __sw1 == 26i32 {
                    DoTVShowTodaysRivalTrainer();
                    break 'l1;
                }
                if __sw1 == 27i32 {
                    DoTVShowDewfordTrendWatcherNetwork();
                    break 'l1;
                }
                if __sw1 == 28i32 {
                    DoTVShowHoennTreasureInvestigators();
                    break 'l1;
                }
                if __sw1 == 29i32 {
                    DoTVShowFindThatGamer();
                    break 'l1;
                }
                if __sw1 == 30i32 {
                    DoTVShowBreakingNewsTV();
                    break 'l1;
                }
                if __sw1 == 31i32 {
                    DoTVShowSecretBaseVisit();
                    break 'l1;
                }
                if __sw1 == 32i32 {
                    DoTVShowPokemonLotteryWinnerFlashReport();
                    break 'l1;
                }
                if __sw1 == 33i32 {
                    DoTVShowThePokemonBattleSeminar();
                    break 'l1;
                }
                if __sw1 == 11i32 {
                    DoTVShowTrainerFanClubSpecial();
                    break 'l1;
                }
                if __sw1 == 34i32 {
                    DoTVShowTrainerFanClub();
                    break 'l1;
                }
                if __sw1 == 35i32 {
                    DoTVShowSpotTheCuties();
                    break 'l1;
                }
                if __sw1 == 36i32 {
                    DoTVShowPokemonNewsBattleFrontier();
                    break 'l1;
                }
                if __sw1 == 37i32 {
                    DoTVShowWhatsNo1InHoennToday();
                    break 'l1;
                }
                if __sw1 == 38i32 {
                    DoTVShowSecretBaseSecrets();
                    break 'l1;
                }
                if __sw1 == 39i32 {
                    DoTVShowSafariFanClub();
                    break 'l1;
                }
                if __sw1 == 12i32 {
                    DoTVShowLilycoveContestLady();
                    break 'l1;
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn DoTVShowBravoTrainerPokemonProfile() {
    unsafe {
        let mut show: *mut u8 = core::ptr::null_mut();
        let mut state: u8 = 0u8;
        show = (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(10188))
            .cast::<u8>())
        .wrapping_offset(
            ((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32) as isize * 36,
        );
        ((&raw mut gSpecialVar_Result).cast::<u16>()).write(0u16);
        state = ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).read();
        'l1: {
            let __sw1 = ((state) as i32);
            if __sw1 == 0i32 {
                TVShowConvertInternationalString(
                    (&raw mut gStringVar1).cast::<u8>(),
                    ((show).wrapping_add(22)).cast::<u8>(),
                    ((((show).wrapping_add(30)).read()) as i32),
                );
                CopyContestCategoryToStringVar(
                    1u8,
                    (crate::c::bf_read((show).wrapping_add(19), 0, 3, false) as u8),
                );
                CopyContestRankToStringVar(
                    2u8,
                    (crate::c::bf_read((show).wrapping_add(19), 3, 2, false) as u8),
                );
                if !((StringCompare(
                    (((&raw mut gSpeciesNames).cast::<u8>()).wrapping_offset(
                        ((((show).wrapping_add(2).cast::<u16>()).read()) as i32) as isize * 11,
                    ))
                    .cast::<u8>(),
                    ((show).wrapping_add(8)).cast::<u8>(),
                )) != 0)
                {
                    ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(8u8);
                } else {
                    ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(1u8);
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                StringCopy(
                    (&raw mut gStringVar1).cast::<u8>(),
                    (((&raw mut gSpeciesNames).cast::<u8>()).wrapping_offset(
                        ((((show).wrapping_add(2).cast::<u16>()).read()) as i32) as isize * 11,
                    ))
                    .cast::<u8>(),
                );
                TVShowConvertInternationalString(
                    (&raw mut gStringVar2).cast::<u8>(),
                    ((show).wrapping_add(8)).cast::<u8>(),
                    ((((show).wrapping_add(31)).read()) as i32),
                );
                CopyContestCategoryToStringVar(
                    2u8,
                    (crate::c::bf_read((show).wrapping_add(19), 0, 3, false) as u8),
                );
                ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(2u8);
                break 'l1;
            }
            if __sw1 == 2i32 {
                TVShowConvertInternationalString(
                    (&raw mut gStringVar1).cast::<u8>(),
                    ((show).wrapping_add(22)).cast::<u8>(),
                    ((((show).wrapping_add(30)).read()) as i32),
                );
                if ((crate::c::bf_read((show).wrapping_add(19), 5, 2, false) as u8) as i32) == 0i32
                {
                    ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(3u8);
                } else {
                    ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(4u8);
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                TVShowConvertInternationalString(
                    (&raw mut gStringVar1).cast::<u8>(),
                    ((show).wrapping_add(22)).cast::<u8>(),
                    ((((show).wrapping_add(30)).read()) as i32),
                );
                CopyEasyChatWord(
                    (&raw mut gStringVar2).cast::<u8>(),
                    (((show).wrapping_add(4)).cast::<u16>()).read(),
                );
                ConvertIntToDecimalString(
                    2u8,
                    ((crate::c::bf_read((show).wrapping_add(19), 5, 2, false) as u8) as i32)
                        .wrapping_add(1i32),
                );
                ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(5u8);
                break 'l1;
            }
            if __sw1 == 4i32 {
                TVShowConvertInternationalString(
                    (&raw mut gStringVar1).cast::<u8>(),
                    ((show).wrapping_add(22)).cast::<u8>(),
                    ((((show).wrapping_add(30)).read()) as i32),
                );
                CopyEasyChatWord(
                    (&raw mut gStringVar2).cast::<u8>(),
                    (((show).wrapping_add(4)).cast::<u16>()).read(),
                );
                ConvertIntToDecimalString(
                    2u8,
                    ((crate::c::bf_read((show).wrapping_add(19), 5, 2, false) as u8) as i32)
                        .wrapping_add(1i32),
                );
                ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(5u8);
                break 'l1;
            }
            if __sw1 == 5i32 {
                TVShowConvertInternationalString(
                    (&raw mut gStringVar1).cast::<u8>(),
                    ((show).wrapping_add(22)).cast::<u8>(),
                    ((((show).wrapping_add(30)).read()) as i32),
                );
                CopyContestCategoryToStringVar(
                    1u8,
                    (crate::c::bf_read((show).wrapping_add(19), 0, 3, false) as u8),
                );
                CopyEasyChatWord(
                    (&raw mut gStringVar3).cast::<u8>(),
                    ((((show).wrapping_add(4)).cast::<u16>()).wrapping_offset(1)).read(),
                );
                if (((show).wrapping_add(20).cast::<u16>()).read()) != 0 {
                    ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(6u8);
                } else {
                    ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(7u8);
                }
                break 'l1;
            }
            if __sw1 == 6i32 {
                StringCopy(
                    (&raw mut gStringVar1).cast::<u8>(),
                    (((&raw mut gSpeciesNames).cast::<u8>()).wrapping_offset(
                        ((((show).wrapping_add(2).cast::<u16>()).read()) as i32) as isize * 11,
                    ))
                    .cast::<u8>(),
                );
                StringCopy(
                    (&raw mut gStringVar2).cast::<u8>(),
                    (((&raw mut gMoveNames).cast::<u8>()).wrapping_offset(
                        ((((show).wrapping_add(20).cast::<u16>()).read()) as i32) as isize * 13,
                    ))
                    .cast::<u8>(),
                );
                CopyEasyChatWord(
                    (&raw mut gStringVar3).cast::<u8>(),
                    ((((show).wrapping_add(4)).cast::<u16>()).wrapping_offset(1)).read(),
                );
                ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(7u8);
                break 'l1;
            }
            if __sw1 == 7i32 {
                TVShowConvertInternationalString(
                    (&raw mut gStringVar1).cast::<u8>(),
                    ((show).wrapping_add(22)).cast::<u8>(),
                    ((((show).wrapping_add(30)).read()) as i32),
                );
                StringCopy(
                    (&raw mut gStringVar2).cast::<u8>(),
                    (((&raw mut gSpeciesNames).cast::<u8>()).wrapping_offset(
                        ((((show).wrapping_add(2).cast::<u16>()).read()) as i32) as isize * 11,
                    ))
                    .cast::<u8>(),
                );
                TVShowDone();
                break 'l1;
            }
            if __sw1 == 8i32 {
                StringCopy(
                    (&raw mut gStringVar1).cast::<u8>(),
                    (((&raw mut gSpeciesNames).cast::<u8>()).wrapping_offset(
                        ((((show).wrapping_add(2).cast::<u16>()).read()) as i32) as isize * 11,
                    ))
                    .cast::<u8>(),
                );
                ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(2u8);
                break 'l1;
            }
        }
        ShowFieldMessage(
            ((((&raw const sTVBravoTrainerTextGroup)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u8>())
            .cast::<*mut u8>())
            .wrapping_offset(((state) as i32) as isize))
            .read(),
        );
    }
}
pub(crate) unsafe extern "C" fn DoTVShowBravoTrainerBattleTower() {
    unsafe {
        let mut show: *mut u8 = core::ptr::null_mut();
        let mut state: u8 = 0u8;
        show = (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(10188))
            .cast::<u8>())
        .wrapping_offset(
            ((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32) as isize * 36,
        );
        ((&raw mut gSpecialVar_Result).cast::<u16>()).write(0u16);
        state = ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).read();
        'l1: {
            let __sw1 = ((state) as i32);
            if __sw1 == 0i32 {
                TVShowConvertInternationalString(
                    (&raw mut gStringVar1).cast::<u8>(),
                    ((show).wrapping_add(2)).cast::<u8>(),
                    ((((show).wrapping_add(29)).read()) as i32),
                );
                StringCopy(
                    (&raw mut gStringVar2).cast::<u8>(),
                    (((&raw mut gSpeciesNames).cast::<u8>()).wrapping_offset(
                        ((((show).wrapping_add(10).cast::<u16>()).read()) as i32) as isize * 11,
                    ))
                    .cast::<u8>(),
                );
                if ((((show).wrapping_add(22).cast::<u16>()).read()) as i32) >= 7i32 {
                    ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(1u8);
                } else {
                    ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(2u8);
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                if ((((show).wrapping_add(26)).read()) as i32) == 50i32 {
                    StringCopy(
                        (&raw mut gStringVar1).cast::<u8>(),
                        (&raw mut gText_Lv50).cast::<u8>(),
                    );
                } else {
                    StringCopy(
                        (&raw mut gStringVar1).cast::<u8>(),
                        (&raw mut gText_OpenLevel).cast::<u8>(),
                    );
                }
                ConvertIntToDecimalString(
                    1u8,
                    ((((show).wrapping_add(22).cast::<u16>()).read()) as i32),
                );
                if ((((show).wrapping_add(28)).read()) as i32) == 1i32 {
                    ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(3u8);
                } else {
                    ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(4u8);
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                TVShowConvertInternationalString(
                    (&raw mut gStringVar1).cast::<u8>(),
                    ((show).wrapping_add(12)).cast::<u8>(),
                    ((((show).wrapping_add(30)).read()) as i32),
                );
                ConvertIntToDecimalString(
                    1u8,
                    ((((show).wrapping_add(22).cast::<u16>()).read()) as i32).wrapping_add(1i32),
                );
                if ((((show).wrapping_add(27)).read()) as i32) == 0i32 {
                    ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(5u8);
                } else {
                    ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(6u8);
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                TVShowConvertInternationalString(
                    (&raw mut gStringVar1).cast::<u8>(),
                    ((show).wrapping_add(12)).cast::<u8>(),
                    ((((show).wrapping_add(30)).read()) as i32),
                );
                StringCopy(
                    (&raw mut gStringVar2).cast::<u8>(),
                    (((&raw mut gSpeciesNames).cast::<u8>()).wrapping_offset(
                        ((((show).wrapping_add(20).cast::<u16>()).read()) as i32) as isize * 11,
                    ))
                    .cast::<u8>(),
                );
                if ((((show).wrapping_add(27)).read()) as i32) == 0i32 {
                    ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(5u8);
                } else {
                    ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(6u8);
                }
                break 'l1;
            }
            if __sw1 == 4i32 {
                TVShowConvertInternationalString(
                    (&raw mut gStringVar1).cast::<u8>(),
                    ((show).wrapping_add(12)).cast::<u8>(),
                    ((((show).wrapping_add(30)).read()) as i32),
                );
                StringCopy(
                    (&raw mut gStringVar2).cast::<u8>(),
                    (((&raw mut gSpeciesNames).cast::<u8>()).wrapping_offset(
                        ((((show).wrapping_add(20).cast::<u16>()).read()) as i32) as isize * 11,
                    ))
                    .cast::<u8>(),
                );
                if ((((show).wrapping_add(27)).read()) as i32) == 0i32 {
                    ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(5u8);
                } else {
                    ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(6u8);
                }
                break 'l1;
            }
            if __sw1 == 5i32 {
                TVShowConvertInternationalString(
                    (&raw mut gStringVar1).cast::<u8>(),
                    ((show).wrapping_add(12)).cast::<u8>(),
                    ((((show).wrapping_add(30)).read()) as i32),
                );
                ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(11u8);
                break 'l1;
            }
            if __sw1 == 6i32 {
                TVShowConvertInternationalString(
                    (&raw mut gStringVar1).cast::<u8>(),
                    ((show).wrapping_add(12)).cast::<u8>(),
                    ((((show).wrapping_add(30)).read()) as i32),
                );
                ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(11u8);
                break 'l1;
            }
            if __sw1 == 7i32 {
                ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(11u8);
                break 'l1;
            }
            if __sw1 == 8i32 || __sw1 == 9i32 || __sw1 == 10i32 {
                TVShowConvertInternationalString(
                    (&raw mut gStringVar1).cast::<u8>(),
                    ((show).wrapping_add(2)).cast::<u8>(),
                    ((((show).wrapping_add(29)).read()) as i32),
                );
                ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(11u8);
                break 'l1;
            }
            if __sw1 == 11i32 {
                CopyEasyChatWord(
                    (&raw mut gStringVar1).cast::<u8>(),
                    (((show).wrapping_add(24)).cast::<u16>()).read(),
                );
                if ((((show).wrapping_add(27)).read()) as i32) == 0i32 {
                    ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(12u8);
                } else {
                    ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(13u8);
                }
                break 'l1;
            }
            if __sw1 == 12i32 || __sw1 == 13i32 {
                CopyEasyChatWord(
                    (&raw mut gStringVar1).cast::<u8>(),
                    (((show).wrapping_add(24)).cast::<u16>()).read(),
                );
                TVShowConvertInternationalString(
                    (&raw mut gStringVar2).cast::<u8>(),
                    ((show).wrapping_add(2)).cast::<u8>(),
                    ((((show).wrapping_add(29)).read()) as i32),
                );
                TVShowConvertInternationalString(
                    (&raw mut gStringVar3).cast::<u8>(),
                    ((show).wrapping_add(12)).cast::<u8>(),
                    ((((show).wrapping_add(30)).read()) as i32),
                );
                ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(14u8);
                break 'l1;
            }
            if __sw1 == 14i32 {
                TVShowConvertInternationalString(
                    (&raw mut gStringVar1).cast::<u8>(),
                    ((show).wrapping_add(2)).cast::<u8>(),
                    ((((show).wrapping_add(29)).read()) as i32),
                );
                StringCopy(
                    (&raw mut gStringVar2).cast::<u8>(),
                    (((&raw mut gSpeciesNames).cast::<u8>()).wrapping_offset(
                        ((((show).wrapping_add(10).cast::<u16>()).read()) as i32) as isize * 11,
                    ))
                    .cast::<u8>(),
                );
                TVShowDone();
                break 'l1;
            }
        }
        ShowFieldMessage(
            ((((&raw const sTVBravoTrainerBattleTowerTextGroup)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u8>())
            .cast::<*mut u8>())
            .wrapping_offset(((state) as i32) as isize))
            .read(),
        );
    }
}
pub(crate) unsafe extern "C" fn DoTVShowTodaysSmartShopper() {
    unsafe {
        let mut show: *mut u8 = core::ptr::null_mut();
        let mut state: u8 = 0u8;
        show = (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(10188))
            .cast::<u8>())
        .wrapping_offset(
            ((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32) as isize * 36,
        );
        ((&raw mut gSpecialVar_Result).cast::<u16>()).write(0u16);
        state = ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).read();
        'l1: {
            let __sw1 = ((state) as i32);
            if __sw1 == 0i32 {
                TVShowConvertInternationalString(
                    (&raw mut gStringVar1).cast::<u8>(),
                    ((show).wrapping_add(19)).cast::<u8>(),
                    ((((show).wrapping_add(3)).read()) as i32),
                );
                GetMapName(
                    (&raw mut gStringVar2).cast::<u8>(),
                    ((((show).wrapping_add(18)).read()) as u16),
                    0u16,
                );
                if (((((show).wrapping_add(12)).cast::<u16>()).read()) as i32) >= 255i32 {
                    ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(11u8);
                } else {
                    ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(1u8);
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                TVShowConvertInternationalString(
                    (&raw mut gStringVar1).cast::<u8>(),
                    ((show).wrapping_add(19)).cast::<u8>(),
                    ((((show).wrapping_add(3)).read()) as i32),
                );
                StringCopy(
                    (&raw mut gStringVar2).cast::<u8>(),
                    GetItemName((((show).wrapping_add(6)).cast::<u16>()).read()),
                );
                ConvertIntToDecimalString(
                    2u8,
                    (((((show).wrapping_add(12)).cast::<u16>()).read()) as i32),
                );
                let __p2 = (&raw mut sTVShowState).cast::<u8>().cast::<u8>();
                (__p2).write(
                    (((((__p2).read()) as i32).wrapping_add(
                        (1i32).wrapping_add(crate::c::rem_i32(((Random()) as i32), 4i32)),
                    )) as u8),
                );
                break 'l1;
            }
            if __sw1 == 2i32 || __sw1 == 4i32 || __sw1 == 5i32 {
                if ((((((show).wrapping_add(6)).cast::<u16>()).wrapping_offset(1)).read()) as i32)
                    != 0i32
                {
                    ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(6u8);
                } else {
                    ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(10u8);
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                ConvertIntToDecimalString(
                    2u8,
                    (((((show).wrapping_add(12)).cast::<u16>()).read()) as i32).wrapping_add(1i32),
                );
                if ((((((show).wrapping_add(6)).cast::<u16>()).wrapping_offset(1)).read()) as i32)
                    != 0i32
                {
                    ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(6u8);
                } else {
                    ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(10u8);
                }
                break 'l1;
            }
            if __sw1 == 6i32 {
                StringCopy(
                    (&raw mut gStringVar2).cast::<u8>(),
                    GetItemName(
                        ((((show).wrapping_add(6)).cast::<u16>()).wrapping_offset(1)).read(),
                    ),
                );
                ConvertIntToDecimalString(
                    2u8,
                    ((((((show).wrapping_add(12)).cast::<u16>()).wrapping_offset(1)).read())
                        as i32),
                );
                if ((((((show).wrapping_add(6)).cast::<u16>()).wrapping_offset(2)).read()) as i32)
                    != 0i32
                {
                    ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(7u8);
                } else {
                    if ((((show).wrapping_add(2)).read()) as i32) == 1i32 {
                        ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(8u8);
                    } else {
                        ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(9u8);
                    }
                }
                break 'l1;
            }
            if __sw1 == 7i32 {
                StringCopy(
                    (&raw mut gStringVar2).cast::<u8>(),
                    GetItemName(
                        ((((show).wrapping_add(6)).cast::<u16>()).wrapping_offset(2)).read(),
                    ),
                );
                ConvertIntToDecimalString(
                    2u8,
                    ((((((show).wrapping_add(12)).cast::<u16>()).wrapping_offset(2)).read())
                        as i32),
                );
                if ((((show).wrapping_add(2)).read()) as i32) == 1i32 {
                    ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(8u8);
                } else {
                    ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(9u8);
                }
                break 'l1;
            }
            if __sw1 == 8i32 {
                if (((((show).wrapping_add(12)).cast::<u16>()).read()) as i32) >= 255i32 {
                    ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(12u8);
                } else {
                    ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(9u8);
                }
                break 'l1;
            }
            if __sw1 == 9i32 {
                SmartShopper_BufferPurchaseTotal(1u8, show);
                TVShowDone();
                break 'l1;
            }
            if __sw1 == 10i32 {
                if ((((show).wrapping_add(2)).read()) as i32) == 1i32 {
                    ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(8u8);
                } else {
                    ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(9u8);
                }
                break 'l1;
            }
            if __sw1 == 11i32 {
                TVShowConvertInternationalString(
                    (&raw mut gStringVar1).cast::<u8>(),
                    ((show).wrapping_add(19)).cast::<u8>(),
                    ((((show).wrapping_add(3)).read()) as i32),
                );
                StringCopy(
                    (&raw mut gStringVar2).cast::<u8>(),
                    GetItemName((((show).wrapping_add(6)).cast::<u16>()).read()),
                );
                if ((((show).wrapping_add(2)).read()) as i32) == 1i32 {
                    ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(8u8);
                } else {
                    ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(12u8);
                }
                break 'l1;
            }
            if __sw1 == 12i32 {
                TVShowConvertInternationalString(
                    (&raw mut gStringVar1).cast::<u8>(),
                    ((show).wrapping_add(19)).cast::<u8>(),
                    ((((show).wrapping_add(3)).read()) as i32),
                );
                TVShowDone();
                break 'l1;
            }
        }
        ShowFieldMessage(
            ((((&raw const sTVTodaysSmartShopperTextGroup)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u8>())
            .cast::<*mut u8>())
            .wrapping_offset(((state) as i32) as isize))
            .read(),
        );
    }
}
pub(crate) unsafe extern "C" fn DoTVShowTheNameRaterShow() {
    unsafe {
        let mut show: *mut u8 = core::ptr::null_mut();
        let mut state: u8 = 0u8;
        show = (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(10188))
            .cast::<u8>())
        .wrapping_offset(
            ((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32) as isize * 36,
        );
        ((&raw mut gSpecialVar_Result).cast::<u16>()).write(0u16);
        state = ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).read();
        'l1: {
            let __sw1 = ((state) as i32);
            let mut __fall = false;
            if __sw1 == 0i32 {
                __fall = true;
                TVShowConvertInternationalString(
                    (&raw mut gStringVar1).cast::<u8>(),
                    ((show).wrapping_add(15)).cast::<u8>(),
                    ((((show).wrapping_add(30)).read()) as i32),
                );
                StringCopy(
                    (&raw mut gStringVar2).cast::<u8>(),
                    (((&raw mut gSpeciesNames).cast::<u8>()).wrapping_offset(
                        ((((show).wrapping_add(2).cast::<u16>()).read()) as i32) as isize * 11,
                    ))
                    .cast::<u8>(),
                );
                TVShowConvertInternationalString(
                    (&raw mut gStringVar3).cast::<u8>(),
                    ((show).wrapping_add(4)).cast::<u8>(),
                    ((((show).wrapping_add(31)).read()) as i32),
                );
                ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(
                    ((((GetRandomNameRaterStateFromName(show)) as i32).wrapping_add(1i32)) as u8),
                );
                break 'l1;
            }
            if __sw1 == 1i32
                || __sw1 == 3i32
                || __sw1 == 4i32
                || __sw1 == 5i32
                || __sw1 == 6i32
                || __sw1 == 7i32
                || __sw1 == 8i32
            {
                __fall = true;
                if ((((show).wrapping_add(26)).read()) as i32) == 0i32 {
                    ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(9u8);
                } else {
                    if ((((show).wrapping_add(26)).read()) as i32) == 1i32 {
                        ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(10u8);
                    } else {
                        if ((((show).wrapping_add(26)).read()) as i32) == 2i32 {
                            ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(11u8);
                        }
                    }
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                __fall = true;
                TVShowConvertInternationalString(
                    (&raw mut gStringVar1).cast::<u8>(),
                    ((show).wrapping_add(15)).cast::<u8>(),
                    ((((show).wrapping_add(30)).read()) as i32),
                );
                if ((((show).wrapping_add(26)).read()) as i32) == 0i32 {
                    ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(9u8);
                } else {
                    if ((((show).wrapping_add(26)).read()) as i32) == 1i32 {
                        ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(10u8);
                    } else {
                        if ((((show).wrapping_add(26)).read()) as i32) == 2i32 {
                            ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(11u8);
                        }
                    }
                }
                break 'l1;
            }
            if __sw1 == 9i32 || __sw1 == 10i32 || __sw1 == 11i32 {
                __fall = true;
                TVShowConvertInternationalString(
                    (&raw mut gStringVar1).cast::<u8>(),
                    ((show).wrapping_add(4)).cast::<u8>(),
                    ((((show).wrapping_add(31)).read()) as i32),
                );
                GetNicknameSubstring(1u8, 0u8, 0u8, 1u16, 0u16, show);
                GetNicknameSubstring(2u8, 1u8, 0u8, 1u16, 0u16, show);
                ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(12u8);
                break 'l1;
            }
            if __sw1 == 13i32 {
                __fall = true;
                TVShowConvertInternationalString(
                    (&raw mut gStringVar1).cast::<u8>(),
                    ((show).wrapping_add(15)).cast::<u8>(),
                    ((((show).wrapping_add(30)).read()) as i32),
                );
                GetNicknameSubstring(1u8, 0u8, 2u8, 0u16, 0u16, show);
                GetNicknameSubstring(2u8, 0u8, 3u8, 1u16, 0u16, show);
                ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(14u8);
                break 'l1;
            }
            if __sw1 == 14i32 {
                __fall = true;
                GetNicknameSubstring(1u8, 0u8, 2u8, 1u16, 0u16, show);
                GetNicknameSubstring(2u8, 0u8, 3u8, 0u16, 0u16, show);
                ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(18u8);
                break 'l1;
            }
            if __sw1 == 15i32 {
                __fall = true;
                GetNicknameSubstring(0u8, 0u8, 2u8, 1u16, 0u16, show);
                StringCopy(
                    (&raw mut gStringVar2).cast::<u8>(),
                    (((&raw mut gSpeciesNames).cast::<u8>()).wrapping_offset(
                        ((((show).wrapping_add(2).cast::<u16>()).read()) as i32) as isize * 11,
                    ))
                    .cast::<u8>(),
                );
                GetNicknameSubstring(
                    2u8,
                    0u8,
                    3u8,
                    2u16,
                    ((show).wrapping_add(2).cast::<u16>()).read(),
                    show,
                );
                ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(16u8);
                break 'l1;
            }
            if __sw1 == 16i32 {
                __fall = true;
                GetNicknameSubstring(
                    0u8,
                    0u8,
                    2u8,
                    2u16,
                    ((show).wrapping_add(2).cast::<u16>()).read(),
                    show,
                );
                GetNicknameSubstring(2u8, 0u8, 3u8, 1u16, 0u16, show);
                ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(17u8);
                break 'l1;
            }
            if __sw1 == 17i32 {
                __fall = true;
                GetNicknameSubstring(0u8, 0u8, 2u8, 1u16, 0u16, show);
                StringCopy(
                    (&raw mut gStringVar2).cast::<u8>(),
                    (((&raw mut gSpeciesNames).cast::<u8>()).wrapping_offset(
                        ((((show).wrapping_add(28).cast::<u16>()).read()) as i32) as isize * 11,
                    ))
                    .cast::<u8>(),
                );
                GetNicknameSubstring(
                    2u8,
                    0u8,
                    3u8,
                    2u16,
                    ((show).wrapping_add(28).cast::<u16>()).read(),
                    show,
                );
                ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(18u8);
                break 'l1;
            }
            if __sw1 == 12i32 {
                __fall = true;
                state = 18u8;
                ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(18u8);
            }
            if __fall || __sw1 == 18i32 {
                __fall = true;
                TVShowConvertInternationalString(
                    (&raw mut gStringVar1).cast::<u8>(),
                    ((show).wrapping_add(4)).cast::<u8>(),
                    ((((show).wrapping_add(31)).read()) as i32),
                );
                TVShowDone();
                break 'l1;
            }
        }
        ShowFieldMessage(
            ((((&raw const sTVNameRaterTextGroup)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u8>())
            .cast::<*mut u8>())
            .wrapping_offset(((state) as i32) as isize))
            .read(),
        );
    }
}
pub(crate) unsafe extern "C" fn DoTVShowPokemonTodaySuccessfulCapture() {
    unsafe {
        let mut show: *mut u8 = core::ptr::null_mut();
        let mut state: u8 = 0u8;
        show = (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(10188))
            .cast::<u8>())
        .wrapping_offset(
            ((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32) as isize * 36,
        );
        ((&raw mut gSpecialVar_Result).cast::<u16>()).write(0u16);
        state = ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).read();
        'l1: {
            let __sw1 = ((state) as i32);
            if __sw1 == 0i32 {
                TVShowConvertInternationalString(
                    (&raw mut gStringVar1).cast::<u8>(),
                    ((show).wrapping_add(19)).cast::<u8>(),
                    ((((show).wrapping_add(2)).read()) as i32),
                );
                StringCopy(
                    (&raw mut gStringVar2).cast::<u8>(),
                    (((&raw mut gSpeciesNames).cast::<u8>()).wrapping_offset(
                        ((((show).wrapping_add(16).cast::<u16>()).read()) as i32) as isize * 11,
                    ))
                    .cast::<u8>(),
                );
                TVShowConvertInternationalString(
                    (&raw mut gStringVar3).cast::<u8>(),
                    ((show).wrapping_add(4)).cast::<u8>(),
                    ((((show).wrapping_add(3)).read()) as i32),
                );
                if ((((show).wrapping_add(15)).read()) as i32) == 1i32 {
                    ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(5u8);
                } else {
                    ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(1u8);
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(2u8);
                break 'l1;
            }
            if __sw1 == 2i32 {
                StringCopy(
                    (&raw mut gStringVar2).cast::<u8>(),
                    GetItemName(((((show).wrapping_add(15)).read()) as u16)),
                );
                ConvertIntToDecimalString(2u8, ((((show).wrapping_add(18)).read()) as i32));
                if ((((show).wrapping_add(18)).read()) as i32) < 4i32 {
                    ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(3u8);
                } else {
                    ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(4u8);
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                TVShowConvertInternationalString(
                    (&raw mut gStringVar1).cast::<u8>(),
                    ((show).wrapping_add(19)).cast::<u8>(),
                    ((((show).wrapping_add(2)).read()) as i32),
                );
                StringCopy(
                    (&raw mut gStringVar2).cast::<u8>(),
                    (((&raw mut gSpeciesNames).cast::<u8>()).wrapping_offset(
                        ((((show).wrapping_add(16).cast::<u16>()).read()) as i32) as isize * 11,
                    ))
                    .cast::<u8>(),
                );
                TVShowConvertInternationalString(
                    (&raw mut gStringVar3).cast::<u8>(),
                    ((show).wrapping_add(4)).cast::<u8>(),
                    ((((show).wrapping_add(3)).read()) as i32),
                );
                ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(6u8);
                break 'l1;
            }
            if __sw1 == 4i32 {
                ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(6u8);
                break 'l1;
            }
            if __sw1 == 5i32 {
                TVShowConvertInternationalString(
                    (&raw mut gStringVar1).cast::<u8>(),
                    ((show).wrapping_add(19)).cast::<u8>(),
                    ((((show).wrapping_add(2)).read()) as i32),
                );
                StringCopy(
                    (&raw mut gStringVar2).cast::<u8>(),
                    (((&raw mut gSpeciesNames).cast::<u8>()).wrapping_offset(
                        ((((show).wrapping_add(16).cast::<u16>()).read()) as i32) as isize * 11,
                    ))
                    .cast::<u8>(),
                );
                ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(6u8);
                break 'l1;
            }
            if __sw1 == 6i32 {
                TVShowConvertInternationalString(
                    (&raw mut gStringVar1).cast::<u8>(),
                    ((show).wrapping_add(19)).cast::<u8>(),
                    ((((show).wrapping_add(2)).read()) as i32),
                );
                StringCopy(
                    (&raw mut gStringVar2).cast::<u8>(),
                    (((&raw mut gSpeciesNames).cast::<u8>()).wrapping_offset(
                        ((((show).wrapping_add(16).cast::<u16>()).read()) as i32) as isize * 11,
                    ))
                    .cast::<u8>(),
                );
                TVShowConvertInternationalString(
                    (&raw mut gStringVar3).cast::<u8>(),
                    ((show).wrapping_add(4)).cast::<u8>(),
                    ((((show).wrapping_add(3)).read()) as i32),
                );
                let __p2 = (&raw mut sTVShowState).cast::<u8>().cast::<u8>();
                (__p2).write(
                    (((((__p2).read()) as i32).wrapping_add(
                        (1i32).wrapping_add(crate::c::rem_i32(((Random()) as i32), 4i32)),
                    )) as u8),
                );
                break 'l1;
            }
            if __sw1 == 7i32 || __sw1 == 8i32 {
                StringCopy(
                    (&raw mut gStringVar1).cast::<u8>(),
                    (((&raw mut gSpeciesNames).cast::<u8>()).wrapping_offset(
                        ((((show).wrapping_add(16).cast::<u16>()).read()) as i32) as isize * 11,
                    ))
                    .cast::<u8>(),
                );
                TVShowConvertInternationalString(
                    (&raw mut gStringVar2).cast::<u8>(),
                    ((show).wrapping_add(4)).cast::<u8>(),
                    ((((show).wrapping_add(3)).read()) as i32),
                );
                GetRandomDifferentSpeciesAndNameSeenByPlayer(
                    2u8,
                    ((show).wrapping_add(16).cast::<u16>()).read(),
                );
                ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(11u8);
                break 'l1;
            }
            if __sw1 == 9i32 || __sw1 == 10i32 {
                StringCopy(
                    (&raw mut gStringVar1).cast::<u8>(),
                    (((&raw mut gSpeciesNames).cast::<u8>()).wrapping_offset(
                        ((((show).wrapping_add(16).cast::<u16>()).read()) as i32) as isize * 11,
                    ))
                    .cast::<u8>(),
                );
                TVShowConvertInternationalString(
                    (&raw mut gStringVar2).cast::<u8>(),
                    ((show).wrapping_add(4)).cast::<u8>(),
                    ((((show).wrapping_add(3)).read()) as i32),
                );
                ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(11u8);
                break 'l1;
            }
            if __sw1 == 11i32 {
                TVShowDone();
                break 'l1;
            }
        }
        ShowFieldMessage(
            ((((&raw const sTVPokemonTodaySuccessfulTextGroup)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u8>())
            .cast::<*mut u8>())
            .wrapping_offset(((state) as i32) as isize))
            .read(),
        );
    }
}
pub(crate) unsafe extern "C" fn DoTVShowPokemonTodayFailedCapture() {
    unsafe {
        let mut show: *mut u8 = core::ptr::null_mut();
        let mut state: u8 = 0u8;
        show = (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(10188))
            .cast::<u8>())
        .wrapping_offset(
            ((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32) as isize * 36,
        );
        ((&raw mut gSpecialVar_Result).cast::<u16>()).write(0u16);
        state = ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).read();
        'l1: {
            let __sw1 = ((state) as i32);
            if __sw1 == 0i32 {
                TVShowConvertInternationalString(
                    (&raw mut gStringVar1).cast::<u8>(),
                    ((show).wrapping_add(19)).cast::<u8>(),
                    ((((show).wrapping_add(2)).read()) as i32),
                );
                StringCopy(
                    (&raw mut gStringVar2).cast::<u8>(),
                    (((&raw mut gSpeciesNames).cast::<u8>()).wrapping_offset(
                        ((((show).wrapping_add(12).cast::<u16>()).read()) as i32) as isize * 11,
                    ))
                    .cast::<u8>(),
                );
                ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(1u8);
                break 'l1;
            }
            if __sw1 == 1i32 {
                TVShowConvertInternationalString(
                    (&raw mut gStringVar1).cast::<u8>(),
                    ((show).wrapping_add(19)).cast::<u8>(),
                    ((((show).wrapping_add(2)).read()) as i32),
                );
                GetMapName(
                    (&raw mut gStringVar2).cast::<u8>(),
                    ((((show).wrapping_add(18)).read()) as u16),
                    0u16,
                );
                StringCopy(
                    (&raw mut gStringVar3).cast::<u8>(),
                    (((&raw mut gSpeciesNames).cast::<u8>()).wrapping_offset(
                        ((((show).wrapping_add(14).cast::<u16>()).read()) as i32) as isize * 11,
                    ))
                    .cast::<u8>(),
                );
                if ((((show).wrapping_add(17)).read()) as i32) == 1i32 {
                    ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(3u8);
                } else {
                    ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(2u8);
                }
                break 'l1;
            }
            if __sw1 == 2i32 || __sw1 == 3i32 {
                TVShowConvertInternationalString(
                    (&raw mut gStringVar1).cast::<u8>(),
                    ((show).wrapping_add(19)).cast::<u8>(),
                    ((((show).wrapping_add(2)).read()) as i32),
                );
                ConvertIntToDecimalString(1u8, ((((show).wrapping_add(16)).read()) as i32));
                if crate::c::rem_i32(((Random()) as i32), 3i32) == 0i32 {
                    ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(5u8);
                } else {
                    ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(4u8);
                }
                break 'l1;
            }
            if __sw1 == 4i32 || __sw1 == 5i32 {
                TVShowConvertInternationalString(
                    (&raw mut gStringVar1).cast::<u8>(),
                    ((show).wrapping_add(19)).cast::<u8>(),
                    ((((show).wrapping_add(2)).read()) as i32),
                );
                ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(6u8);
                break 'l1;
            }
            if __sw1 == 6i32 {
                TVShowDone();
                break 'l1;
            }
        }
        ShowFieldMessage(
            ((((&raw const sTVPokemonTodayFailedTextGroup)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u8>())
            .cast::<*mut u8>())
            .wrapping_offset(((state) as i32) as isize))
            .read(),
        );
    }
}
pub(crate) unsafe extern "C" fn DoTVShowPokemonFanClubLetter() {
    unsafe {
        let mut show: *mut u8 = core::ptr::null_mut();
        let mut state: u8 = 0u8;
        let mut rval: u16 = 0u16;
        show = (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(10188))
            .cast::<u8>())
        .wrapping_offset(
            ((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32) as isize * 36,
        );
        ((&raw mut gSpecialVar_Result).cast::<u16>()).write(0u16);
        state = ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).read();
        'l1: {
            let __sw1 = ((state) as i32);
            if __sw1 == 0i32 {
                TVShowConvertInternationalString(
                    (&raw mut gStringVar1).cast::<u8>(),
                    ((show).wrapping_add(16)).cast::<u8>(),
                    ((((show).wrapping_add(24)).read()) as i32),
                );
                StringCopy(
                    (&raw mut gStringVar2).cast::<u8>(),
                    (((&raw mut gSpeciesNames).cast::<u8>()).wrapping_offset(
                        ((((show).wrapping_add(2).cast::<u16>()).read()) as i32) as isize * 11,
                    ))
                    .cast::<u8>(),
                );
                ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(50u8);
                break 'l1;
            }
            if __sw1 == 1i32 {
                rval = (((crate::c::rem_i32(((Random()) as i32), 4i32)).wrapping_add(1i32)) as u16);
                if ((rval) as i32) == 1i32 {
                    ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(2u8);
                } else {
                    ((&raw mut sTVShowState).cast::<u8>().cast::<u8>())
                        .write(((((rval) as i32).wrapping_add(2i32)) as u8));
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(51u8);
                break 'l1;
            }
            if __sw1 == 3i32 {
                let __p2 = (&raw mut sTVShowState).cast::<u8>().cast::<u8>();
                (__p2).write(
                    (((((__p2).read()) as i32).wrapping_add(
                        (crate::c::rem_i32(((Random()) as i32), 3i32)).wrapping_add(1i32),
                    )) as u8),
                );
                break 'l1;
            }
            if __sw1 == 4i32 || __sw1 == 5i32 || __sw1 == 6i32 {
                GetRandomWordFromShow(show);
                ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(7u8);
                break 'l1;
            }
            if __sw1 == 7i32 {
                rval =
                    (((crate::c::rem_i32(((Random()) as i32), 31i32)).wrapping_add(70i32)) as u16);
                ConvertIntToDecimalString(2u8, ((rval) as i32));
                TVShowDone();
                break 'l1;
            }
            if __sw1 == 50i32 {
                ConvertEasyChatWordsToString(
                    (&raw mut gStringVar4).cast::<u8>(),
                    ((show).wrapping_add(4)).cast::<u16>(),
                    2u16,
                    2u16,
                );
                ShowFieldMessage((&raw mut gStringVar4).cast::<u8>());
                ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(1u8);
                return;
            }
            if __sw1 == 51i32 {
                ConvertEasyChatWordsToString(
                    (&raw mut gStringVar4).cast::<u8>(),
                    ((show).wrapping_add(4)).cast::<u16>(),
                    2u16,
                    2u16,
                );
                ShowFieldMessage((&raw mut gStringVar4).cast::<u8>());
                ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(3u8);
                return;
            }
        }
        ShowFieldMessage(
            ((((&raw const sTVFanClubTextGroup)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u8>())
            .cast::<*mut u8>())
            .wrapping_offset(((state) as i32) as isize))
            .read(),
        );
    }
}
pub(crate) unsafe extern "C" fn DoTVShowRecentHappenings() {
    unsafe {
        let mut show: *mut u8 = core::ptr::null_mut();
        let mut state: u8 = 0u8;
        show = (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(10188))
            .cast::<u8>())
        .wrapping_offset(
            ((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32) as isize * 36,
        );
        ((&raw mut gSpecialVar_Result).cast::<u16>()).write(0u16);
        state = ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).read();
        'l1: {
            let __sw1 = ((state) as i32);
            if __sw1 == 0i32 {
                TVShowConvertInternationalString(
                    (&raw mut gStringVar1).cast::<u8>(),
                    ((show).wrapping_add(16)).cast::<u8>(),
                    ((((show).wrapping_add(24)).read()) as i32),
                );
                GetRandomWordFromShow(show);
                ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(50u8);
                break 'l1;
            }
            if __sw1 == 1i32 {
                let __p2 = (&raw mut sTVShowState).cast::<u8>().cast::<u8>();
                (__p2).write(
                    (((((__p2).read()) as i32).wrapping_add(
                        (1i32).wrapping_add(crate::c::rem_i32(((Random()) as i32), 3i32)),
                    )) as u8),
                );
                break 'l1;
            }
            if __sw1 == 2i32 || __sw1 == 3i32 || __sw1 == 4i32 {
                ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(5u8);
                break 'l1;
            }
            if __sw1 == 5i32 {
                TVShowDone();
                break 'l1;
            }
            if __sw1 == 50i32 {
                ConvertEasyChatWordsToString(
                    (&raw mut gStringVar4).cast::<u8>(),
                    ((show).wrapping_add(4)).cast::<u16>(),
                    2u16,
                    2u16,
                );
                ShowFieldMessage((&raw mut gStringVar4).cast::<u8>());
                ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(1u8);
                return;
            }
        }
        ShowFieldMessage(
            ((((&raw const sTVRecentHappeninssTextGroup)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u8>())
            .cast::<*mut u8>())
            .wrapping_offset(((state) as i32) as isize))
            .read(),
        );
    }
}
pub(crate) unsafe extern "C" fn DoTVShowPokemonFanClubOpinions() {
    unsafe {
        let mut show: *mut u8 = core::ptr::null_mut();
        let mut state: u8 = 0u8;
        show = (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(10188))
            .cast::<u8>())
        .wrapping_offset(
            ((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32) as isize * 36,
        );
        ((&raw mut gSpecialVar_Result).cast::<u16>()).write(0u16);
        state = ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).read();
        'l1: {
            let __sw1 = ((state) as i32);
            if __sw1 == 0i32 {
                TVShowConvertInternationalString(
                    (&raw mut gStringVar1).cast::<u8>(),
                    ((show).wrapping_add(5)).cast::<u8>(),
                    ((((show).wrapping_add(13)).read()) as i32),
                );
                StringCopy(
                    (&raw mut gStringVar2).cast::<u8>(),
                    (((&raw mut gSpeciesNames).cast::<u8>()).wrapping_offset(
                        ((((show).wrapping_add(2).cast::<u16>()).read()) as i32) as isize * 11,
                    ))
                    .cast::<u8>(),
                );
                TVShowConvertInternationalString(
                    (&raw mut gStringVar3).cast::<u8>(),
                    ((show).wrapping_add(16)).cast::<u8>(),
                    ((((show).wrapping_add(14)).read()) as i32),
                );
                ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(
                    ((((crate::c::bf_read((show).wrapping_add(4), 4, 4, false) as u8) as i32)
                        .wrapping_add(1i32)) as u8),
                );
                break 'l1;
            }
            if __sw1 == 1i32 || __sw1 == 2i32 || __sw1 == 3i32 {
                TVShowConvertInternationalString(
                    (&raw mut gStringVar1).cast::<u8>(),
                    ((show).wrapping_add(5)).cast::<u8>(),
                    ((((show).wrapping_add(13)).read()) as i32),
                );
                StringCopy(
                    (&raw mut gStringVar2).cast::<u8>(),
                    (((&raw mut gSpeciesNames).cast::<u8>()).wrapping_offset(
                        ((((show).wrapping_add(2).cast::<u16>()).read()) as i32) as isize * 11,
                    ))
                    .cast::<u8>(),
                );
                CopyEasyChatWord(
                    (&raw mut gStringVar3).cast::<u8>(),
                    (((show).wrapping_add(28)).cast::<u16>()).read(),
                );
                ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(4u8);
                break 'l1;
            }
            if __sw1 == 4i32 {
                TVShowConvertInternationalString(
                    (&raw mut gStringVar1).cast::<u8>(),
                    ((show).wrapping_add(5)).cast::<u8>(),
                    ((((show).wrapping_add(13)).read()) as i32),
                );
                CopyEasyChatWord(
                    (&raw mut gStringVar3).cast::<u8>(),
                    ((((show).wrapping_add(28)).cast::<u16>()).wrapping_offset(1)).read(),
                );
                TVShowDone();
                break 'l1;
            }
        }
        ShowFieldMessage(
            ((((&raw const sTVFanClubOpinionsTextGroup)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u8>())
            .cast::<*mut u8>())
            .wrapping_offset(((state) as i32) as isize))
            .read(),
        );
    }
}
pub(crate) unsafe extern "C" fn DoTVShowDummiedOut() {
    unsafe {}
}
pub(crate) unsafe extern "C" fn DoTVShowPokemonNewsMassOutbreak() {
    unsafe {
        let mut show: *mut u8 = core::ptr::null_mut();
        show = (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(10188))
            .cast::<u8>())
        .wrapping_offset(
            ((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32) as isize * 36,
        );
        GetMapName(
            (&raw mut gStringVar1).cast::<u8>(),
            ((((show).wrapping_add(16)).read()) as u16),
            0u16,
        );
        StringCopy(
            (&raw mut gStringVar2).cast::<u8>(),
            (((&raw mut gSpeciesNames).cast::<u8>()).wrapping_offset(
                ((((show).wrapping_add(12).cast::<u16>()).read()) as i32) as isize * 11,
            ))
            .cast::<u8>(),
        );
        TVShowDone();
        StartMassOutbreak();
        ShowFieldMessage(
            ((((&raw const sTVMassOutbreakTextGroup)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u8>())
            .cast::<*mut u8>())
            .wrapping_offset(
                ((((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).read()) as i32) as isize,
            ))
            .read(),
        );
    }
}
pub(crate) unsafe extern "C" fn DoTVShowPokemonContestLiveUpdates() {
    unsafe {
        let mut show: *mut u8 = core::ptr::null_mut();
        let mut state: u8 = 0u8;
        show = (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(10188))
            .cast::<u8>())
        .wrapping_offset(
            ((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32) as isize * 36,
        );
        ((&raw mut gSpecialVar_Result).cast::<u16>()).write(0u16);
        state = ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).read();
        'l1: {
            let __sw1 = ((state) as i32);
            if __sw1 == 0i32 {
                BufferContestName(
                    (&raw mut gStringVar1).cast::<u8>(),
                    ((show).wrapping_add(28)).read(),
                );
                StringCopy(
                    (&raw mut gStringVar2).cast::<u8>(),
                    (((&raw mut gSpeciesNames).cast::<u8>()).wrapping_offset(
                        ((((show).wrapping_add(18).cast::<u16>()).read()) as i32) as isize * 11,
                    ))
                    .cast::<u8>(),
                );
                TVShowConvertInternationalString(
                    (&raw mut gStringVar3).cast::<u8>(),
                    ((show).wrapping_add(20)).cast::<u8>(),
                    ((((show).wrapping_add(29)).read()) as i32),
                );
                if ((((show).wrapping_add(13)).read()) as i32)
                    == ((((show).wrapping_add(14)).read()) as i32)
                {
                    if ((((show).wrapping_add(13)).read()) as i32) == 0i32 {
                        ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(1u8);
                    } else {
                        ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(3u8);
                    }
                } else {
                    if ((((show).wrapping_add(13)).read()) as i32)
                        > ((((show).wrapping_add(14)).read()) as i32)
                    {
                        ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(2u8);
                    } else {
                        ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(4u8);
                    }
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                StringCopy(
                    (&raw mut gStringVar2).cast::<u8>(),
                    (((&raw mut gSpeciesNames).cast::<u8>()).wrapping_offset(
                        ((((show).wrapping_add(18).cast::<u16>()).read()) as i32) as isize * 11,
                    ))
                    .cast::<u8>(),
                );
                'l2: {
                    let __sw2 = ((((show).wrapping_add(15)).read()) as i32);
                    if __sw2 == 1i32 {
                        ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(8u8);
                        break 'l2;
                    }
                    if __sw2 == 2i32 {
                        ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(5u8);
                        break 'l2;
                    }
                    if __sw2 == 4i32 {
                        ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(14u8);
                        break 'l2;
                    }
                    if __sw2 == 8i32 {
                        ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(7u8);
                        break 'l2;
                    }
                    if __sw2 == 16i32 {
                        ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(6u8);
                        break 'l2;
                    }
                    if __sw2 == 32i32 {
                        ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(20u8);
                        break 'l2;
                    }
                    if __sw2 == 64i32 {
                        ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(21u8);
                        break 'l2;
                    }
                    if __sw2 == 128i32 {
                        ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(22u8);
                        break 'l2;
                    }
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                StringCopy(
                    (&raw mut gStringVar2).cast::<u8>(),
                    (((&raw mut gSpeciesNames).cast::<u8>()).wrapping_offset(
                        ((((show).wrapping_add(18).cast::<u16>()).read()) as i32) as isize * 11,
                    ))
                    .cast::<u8>(),
                );
                'l3: {
                    let __sw3 = ((((show).wrapping_add(15)).read()) as i32);
                    if __sw3 == 1i32 {
                        ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(8u8);
                        break 'l3;
                    }
                    if __sw3 == 2i32 {
                        ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(5u8);
                        break 'l3;
                    }
                    if __sw3 == 4i32 {
                        ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(14u8);
                        break 'l3;
                    }
                    if __sw3 == 8i32 {
                        ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(7u8);
                        break 'l3;
                    }
                    if __sw3 == 16i32 {
                        ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(6u8);
                        break 'l3;
                    }
                    if __sw3 == 32i32 {
                        ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(20u8);
                        break 'l3;
                    }
                    if __sw3 == 64i32 {
                        ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(21u8);
                        break 'l3;
                    }
                    if __sw3 == 128i32 {
                        ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(22u8);
                        break 'l3;
                    }
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                StringCopy(
                    (&raw mut gStringVar2).cast::<u8>(),
                    (((&raw mut gSpeciesNames).cast::<u8>()).wrapping_offset(
                        ((((show).wrapping_add(18).cast::<u16>()).read()) as i32) as isize * 11,
                    ))
                    .cast::<u8>(),
                );
                TVShowConvertInternationalString(
                    (&raw mut gStringVar3).cast::<u8>(),
                    ((show).wrapping_add(20)).cast::<u8>(),
                    ((((show).wrapping_add(29)).read()) as i32),
                );
                'l4: {
                    let __sw4 = ((((show).wrapping_add(15)).read()) as i32);
                    if __sw4 == 1i32 {
                        ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(8u8);
                        break 'l4;
                    }
                    if __sw4 == 2i32 {
                        ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(5u8);
                        break 'l4;
                    }
                    if __sw4 == 4i32 {
                        ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(14u8);
                        break 'l4;
                    }
                    if __sw4 == 8i32 {
                        ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(7u8);
                        break 'l4;
                    }
                    if __sw4 == 16i32 {
                        ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(6u8);
                        break 'l4;
                    }
                    if __sw4 == 32i32 {
                        ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(20u8);
                        break 'l4;
                    }
                    if __sw4 == 64i32 {
                        ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(21u8);
                        break 'l4;
                    }
                    if __sw4 == 128i32 {
                        ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(22u8);
                        break 'l4;
                    }
                }
                break 'l1;
            }
            if __sw1 == 4i32 {
                'l5: {
                    let __sw5 = ((((show).wrapping_add(28)).read()) as i32);
                    if __sw5 == 0i32 {
                        StringCopy(
                            (&raw mut gStringVar1).cast::<u8>(),
                            (&raw mut gText_Cool).cast::<u8>(),
                        );
                        break 'l5;
                    }
                    if __sw5 == 1i32 {
                        StringCopy(
                            (&raw mut gStringVar1).cast::<u8>(),
                            (&raw mut gText_Beauty).cast::<u8>(),
                        );
                        break 'l5;
                    }
                    if __sw5 == 2i32 {
                        StringCopy(
                            (&raw mut gStringVar1).cast::<u8>(),
                            (&raw mut gText_Cute).cast::<u8>(),
                        );
                        break 'l5;
                    }
                    if __sw5 == 3i32 {
                        StringCopy(
                            (&raw mut gStringVar1).cast::<u8>(),
                            (&raw mut gText_Smart).cast::<u8>(),
                        );
                        break 'l5;
                    }
                    if __sw5 == 4i32 {
                        StringCopy(
                            (&raw mut gStringVar1).cast::<u8>(),
                            (&raw mut gText_Tough).cast::<u8>(),
                        );
                        break 'l5;
                    }
                }
                StringCopy(
                    (&raw mut gStringVar2).cast::<u8>(),
                    (((&raw mut gSpeciesNames).cast::<u8>()).wrapping_offset(
                        ((((show).wrapping_add(18).cast::<u16>()).read()) as i32) as isize * 11,
                    ))
                    .cast::<u8>(),
                );
                'l6: {
                    let __sw6 = ((((show).wrapping_add(15)).read()) as i32);
                    if __sw6 == 1i32 {
                        ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(8u8);
                        break 'l6;
                    }
                    if __sw6 == 2i32 {
                        ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(5u8);
                        break 'l6;
                    }
                    if __sw6 == 4i32 {
                        ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(14u8);
                        break 'l6;
                    }
                    if __sw6 == 8i32 {
                        ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(7u8);
                        break 'l6;
                    }
                    if __sw6 == 16i32 {
                        ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(6u8);
                        break 'l6;
                    }
                    if __sw6 == 32i32 {
                        ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(20u8);
                        break 'l6;
                    }
                    if __sw6 == 64i32 {
                        ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(21u8);
                        break 'l6;
                    }
                    if __sw6 == 128i32 {
                        ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(22u8);
                        break 'l6;
                    }
                }
                break 'l1;
            }
            if __sw1 == 5i32 {
                StringCopy(
                    (&raw mut gStringVar2).cast::<u8>(),
                    (((&raw mut gSpeciesNames).cast::<u8>()).wrapping_offset(
                        ((((show).wrapping_add(18).cast::<u16>()).read()) as i32) as isize * 11,
                    ))
                    .cast::<u8>(),
                );
                ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(23u8);
                break 'l1;
            }
            if __sw1 == 6i32 {
                StringCopy(
                    (&raw mut gStringVar2).cast::<u8>(),
                    (((&raw mut gSpeciesNames).cast::<u8>()).wrapping_offset(
                        ((((show).wrapping_add(18).cast::<u16>()).read()) as i32) as isize * 11,
                    ))
                    .cast::<u8>(),
                );
                ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(23u8);
                break 'l1;
            }
            if __sw1 == 7i32 {
                StringCopy(
                    (&raw mut gStringVar2).cast::<u8>(),
                    (((&raw mut gSpeciesNames).cast::<u8>()).wrapping_offset(
                        ((((show).wrapping_add(18).cast::<u16>()).read()) as i32) as isize * 11,
                    ))
                    .cast::<u8>(),
                );
                ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(23u8);
                break 'l1;
            }
            if __sw1 == 8i32 {
                StringCopy(
                    (&raw mut gStringVar2).cast::<u8>(),
                    (((&raw mut gSpeciesNames).cast::<u8>()).wrapping_offset(
                        ((((show).wrapping_add(18).cast::<u16>()).read()) as i32) as isize * 11,
                    ))
                    .cast::<u8>(),
                );
                'l7: {
                    let __sw7 = ((((show).wrapping_add(28)).read()) as i32);
                    if __sw7 == 0i32 {
                        ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(9u8);
                        break 'l7;
                    }
                    if __sw7 == 1i32 {
                        ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(10u8);
                        break 'l7;
                    }
                    if __sw7 == 2i32 {
                        ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(11u8);
                        break 'l7;
                    }
                    if __sw7 == 3i32 {
                        ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(12u8);
                        break 'l7;
                    }
                    if __sw7 == 4i32 {
                        ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(13u8);
                        break 'l7;
                    }
                }
                break 'l1;
            }
            if __sw1 == 9i32 {
                StringCopy(
                    (&raw mut gStringVar2).cast::<u8>(),
                    (((&raw mut gSpeciesNames).cast::<u8>()).wrapping_offset(
                        ((((show).wrapping_add(18).cast::<u16>()).read()) as i32) as isize * 11,
                    ))
                    .cast::<u8>(),
                );
                ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(23u8);
                break 'l1;
            }
            if __sw1 == 10i32 {
                StringCopy(
                    (&raw mut gStringVar2).cast::<u8>(),
                    (((&raw mut gSpeciesNames).cast::<u8>()).wrapping_offset(
                        ((((show).wrapping_add(18).cast::<u16>()).read()) as i32) as isize * 11,
                    ))
                    .cast::<u8>(),
                );
                ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(23u8);
                break 'l1;
            }
            if __sw1 == 11i32 {
                StringCopy(
                    (&raw mut gStringVar2).cast::<u8>(),
                    (((&raw mut gSpeciesNames).cast::<u8>()).wrapping_offset(
                        ((((show).wrapping_add(18).cast::<u16>()).read()) as i32) as isize * 11,
                    ))
                    .cast::<u8>(),
                );
                ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(23u8);
                break 'l1;
            }
            if __sw1 == 12i32 {
                StringCopy(
                    (&raw mut gStringVar2).cast::<u8>(),
                    (((&raw mut gSpeciesNames).cast::<u8>()).wrapping_offset(
                        ((((show).wrapping_add(18).cast::<u16>()).read()) as i32) as isize * 11,
                    ))
                    .cast::<u8>(),
                );
                ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(23u8);
                break 'l1;
            }
            if __sw1 == 13i32 {
                StringCopy(
                    (&raw mut gStringVar2).cast::<u8>(),
                    (((&raw mut gSpeciesNames).cast::<u8>()).wrapping_offset(
                        ((((show).wrapping_add(18).cast::<u16>()).read()) as i32) as isize * 11,
                    ))
                    .cast::<u8>(),
                );
                ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(23u8);
                break 'l1;
            }
            if __sw1 == 14i32 {
                StringCopy(
                    (&raw mut gStringVar2).cast::<u8>(),
                    (((&raw mut gSpeciesNames).cast::<u8>()).wrapping_offset(
                        ((((show).wrapping_add(18).cast::<u16>()).read()) as i32) as isize * 11,
                    ))
                    .cast::<u8>(),
                );
                'l8: {
                    let __sw8 = ((((show).wrapping_add(28)).read()) as i32);
                    if __sw8 == 0i32 {
                        ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(15u8);
                        break 'l8;
                    }
                    if __sw8 == 1i32 {
                        ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(16u8);
                        break 'l8;
                    }
                    if __sw8 == 2i32 {
                        ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(17u8);
                        break 'l8;
                    }
                    if __sw8 == 3i32 {
                        ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(18u8);
                        break 'l8;
                    }
                    if __sw8 == 4i32 {
                        ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(19u8);
                        break 'l8;
                    }
                }
                break 'l1;
            }
            if __sw1 == 15i32 {
                StringCopy(
                    (&raw mut gStringVar2).cast::<u8>(),
                    (((&raw mut gSpeciesNames).cast::<u8>()).wrapping_offset(
                        ((((show).wrapping_add(18).cast::<u16>()).read()) as i32) as isize * 11,
                    ))
                    .cast::<u8>(),
                );
                ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(23u8);
                break 'l1;
            }
            if __sw1 == 16i32 {
                StringCopy(
                    (&raw mut gStringVar2).cast::<u8>(),
                    (((&raw mut gSpeciesNames).cast::<u8>()).wrapping_offset(
                        ((((show).wrapping_add(18).cast::<u16>()).read()) as i32) as isize * 11,
                    ))
                    .cast::<u8>(),
                );
                ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(23u8);
                break 'l1;
            }
            if __sw1 == 17i32 {
                StringCopy(
                    (&raw mut gStringVar2).cast::<u8>(),
                    (((&raw mut gSpeciesNames).cast::<u8>()).wrapping_offset(
                        ((((show).wrapping_add(18).cast::<u16>()).read()) as i32) as isize * 11,
                    ))
                    .cast::<u8>(),
                );
                ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(23u8);
                break 'l1;
            }
            if __sw1 == 18i32 {
                StringCopy(
                    (&raw mut gStringVar2).cast::<u8>(),
                    (((&raw mut gSpeciesNames).cast::<u8>()).wrapping_offset(
                        ((((show).wrapping_add(18).cast::<u16>()).read()) as i32) as isize * 11,
                    ))
                    .cast::<u8>(),
                );
                ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(23u8);
                break 'l1;
            }
            if __sw1 == 19i32 {
                StringCopy(
                    (&raw mut gStringVar2).cast::<u8>(),
                    (((&raw mut gSpeciesNames).cast::<u8>()).wrapping_offset(
                        ((((show).wrapping_add(18).cast::<u16>()).read()) as i32) as isize * 11,
                    ))
                    .cast::<u8>(),
                );
                ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(23u8);
                break 'l1;
            }
            if __sw1 == 20i32 {
                StringCopy(
                    (&raw mut gStringVar2).cast::<u8>(),
                    (((&raw mut gSpeciesNames).cast::<u8>()).wrapping_offset(
                        ((((show).wrapping_add(18).cast::<u16>()).read()) as i32) as isize * 11,
                    ))
                    .cast::<u8>(),
                );
                ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(23u8);
                break 'l1;
            }
            if __sw1 == 21i32 {
                StringCopy(
                    (&raw mut gStringVar2).cast::<u8>(),
                    (((&raw mut gSpeciesNames).cast::<u8>()).wrapping_offset(
                        ((((show).wrapping_add(18).cast::<u16>()).read()) as i32) as isize * 11,
                    ))
                    .cast::<u8>(),
                );
                ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(23u8);
                break 'l1;
            }
            if __sw1 == 22i32 {
                StringCopy(
                    (&raw mut gStringVar2).cast::<u8>(),
                    (((&raw mut gSpeciesNames).cast::<u8>()).wrapping_offset(
                        ((((show).wrapping_add(18).cast::<u16>()).read()) as i32) as isize * 11,
                    ))
                    .cast::<u8>(),
                );
                StringCopy(
                    (&raw mut gStringVar3).cast::<u8>(),
                    (((&raw mut gMoveNames).cast::<u8>()).wrapping_offset(
                        ((((show).wrapping_add(16).cast::<u16>()).read()) as i32) as isize * 13,
                    ))
                    .cast::<u8>(),
                );
                ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(23u8);
                break 'l1;
            }
            if __sw1 == 23i32 {
                StringCopy(
                    (&raw mut gStringVar1).cast::<u8>(),
                    (((&raw mut gSpeciesNames).cast::<u8>()).wrapping_offset(
                        ((((show).wrapping_add(18).cast::<u16>()).read()) as i32) as isize * 11,
                    ))
                    .cast::<u8>(),
                );
                TVShowConvertInternationalString(
                    (&raw mut gStringVar2).cast::<u8>(),
                    ((show).wrapping_add(4)).cast::<u8>(),
                    ((((show).wrapping_add(30)).read()) as i32),
                );
                StringCopy(
                    (&raw mut gStringVar3).cast::<u8>(),
                    (((&raw mut gSpeciesNames).cast::<u8>()).wrapping_offset(
                        ((((show).wrapping_add(2).cast::<u16>()).read()) as i32) as isize * 11,
                    ))
                    .cast::<u8>(),
                );
                'l9: {
                    let __sw9 = ((((show).wrapping_add(12)).read()) as i32);
                    if __sw9 == 1i32 {
                        ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(31u8);
                        break 'l9;
                    }
                    if __sw9 == 2i32 {
                        ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(30u8);
                        break 'l9;
                    }
                    if __sw9 == 4i32 {
                        ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(29u8);
                        break 'l9;
                    }
                    if __sw9 == 8i32 {
                        ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(28u8);
                        break 'l9;
                    }
                    if __sw9 == 16i32 {
                        ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(27u8);
                        break 'l9;
                    }
                    if __sw9 == 32i32 {
                        ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(26u8);
                        break 'l9;
                    }
                    if __sw9 == 64i32 {
                        ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(25u8);
                        break 'l9;
                    }
                    if __sw9 == 128i32 {
                        ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(24u8);
                        break 'l9;
                    }
                }
                break 'l1;
            }
            if __sw1 == 24i32 {
                StringCopy(
                    (&raw mut gStringVar1).cast::<u8>(),
                    (((&raw mut gSpeciesNames).cast::<u8>()).wrapping_offset(
                        ((((show).wrapping_add(2).cast::<u16>()).read()) as i32) as isize * 11,
                    ))
                    .cast::<u8>(),
                );
                ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(32u8);
                break 'l1;
            }
            if __sw1 == 25i32 {
                TVShowConvertInternationalString(
                    (&raw mut gStringVar1).cast::<u8>(),
                    ((show).wrapping_add(4)).cast::<u8>(),
                    ((((show).wrapping_add(30)).read()) as i32),
                );
                StringCopy(
                    (&raw mut gStringVar2).cast::<u8>(),
                    (((&raw mut gSpeciesNames).cast::<u8>()).wrapping_offset(
                        ((((show).wrapping_add(2).cast::<u16>()).read()) as i32) as isize * 11,
                    ))
                    .cast::<u8>(),
                );
                ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(32u8);
                break 'l1;
            }
            if __sw1 == 28i32 {
                ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(32u8);
                break 'l1;
            }
            if __sw1 == 29i32 {
                TVShowConvertInternationalString(
                    (&raw mut gStringVar1).cast::<u8>(),
                    ((show).wrapping_add(20)).cast::<u8>(),
                    ((((show).wrapping_add(29)).read()) as i32),
                );
                StringCopy(
                    (&raw mut gStringVar2).cast::<u8>(),
                    (((&raw mut gSpeciesNames).cast::<u8>()).wrapping_offset(
                        ((((show).wrapping_add(18).cast::<u16>()).read()) as i32) as isize * 11,
                    ))
                    .cast::<u8>(),
                );
                TVShowConvertInternationalString(
                    (&raw mut gStringVar3).cast::<u8>(),
                    ((show).wrapping_add(4)).cast::<u8>(),
                    ((((show).wrapping_add(30)).read()) as i32),
                );
                ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(32u8);
                break 'l1;
            }
            if __sw1 == 26i32 || __sw1 == 27i32 || __sw1 == 30i32 || __sw1 == 31i32 {
                TVShowConvertInternationalString(
                    (&raw mut gStringVar1).cast::<u8>(),
                    ((show).wrapping_add(4)).cast::<u8>(),
                    ((((show).wrapping_add(30)).read()) as i32),
                );
                ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(32u8);
                break 'l1;
            }
            if __sw1 == 32i32 {
                TVShowConvertInternationalString(
                    (&raw mut gStringVar1).cast::<u8>(),
                    ((show).wrapping_add(20)).cast::<u8>(),
                    ((((show).wrapping_add(29)).read()) as i32),
                );
                StringCopy(
                    (&raw mut gStringVar2).cast::<u8>(),
                    (((&raw mut gSpeciesNames).cast::<u8>()).wrapping_offset(
                        ((((show).wrapping_add(18).cast::<u16>()).read()) as i32) as isize * 11,
                    ))
                    .cast::<u8>(),
                );
                TVShowDone();
                break 'l1;
            }
        }
        ShowFieldMessage(
            ((((&raw const sTVContestLiveUpdatesTextGroup)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u8>())
            .cast::<*mut u8>())
            .wrapping_offset(((state) as i32) as isize))
            .read(),
        );
    }
}
pub(crate) unsafe extern "C" fn DoTVShowPokemonBattleUpdate() {
    unsafe {
        let mut show: *mut u8 = core::ptr::null_mut();
        let mut state: u8 = 0u8;
        show = (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(10188))
            .cast::<u8>())
        .wrapping_offset(
            ((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32) as isize * 36,
        );
        ((&raw mut gSpecialVar_Result).cast::<u16>()).write(0u16);
        state = ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).read();
        'l1: {
            let __sw1 = ((state) as i32);
            if __sw1 == 0i32 {
                'l2: {
                    let __sw2 = ((((show).wrapping_add(24)).read()) as i32);
                    if __sw2 == 0i32 || __sw2 == 1i32 {
                        ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(1u8);
                        break 'l2;
                    }
                    if __sw2 == 2i32 {
                        ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(5u8);
                        break 'l2;
                    }
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                TVShowConvertInternationalString(
                    (&raw mut gStringVar1).cast::<u8>(),
                    ((show).wrapping_add(4)).cast::<u8>(),
                    ((((show).wrapping_add(25)).read()) as i32),
                );
                TVShowConvertInternationalString(
                    (&raw mut gStringVar2).cast::<u8>(),
                    ((show).wrapping_add(12)).cast::<u8>(),
                    ((((show).wrapping_add(26)).read()) as i32),
                );
                if ((((show).wrapping_add(24)).read()) as i32) == 0i32 {
                    StringCopy(
                        (&raw mut gStringVar3).cast::<u8>(),
                        (&raw mut gText_Single).cast::<u8>(),
                    );
                } else {
                    StringCopy(
                        (&raw mut gStringVar3).cast::<u8>(),
                        (&raw mut gText_Double).cast::<u8>(),
                    );
                }
                ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(2u8);
                break 'l1;
            }
            if __sw1 == 2i32 {
                TVShowConvertInternationalString(
                    (&raw mut gStringVar1).cast::<u8>(),
                    ((show).wrapping_add(4)).cast::<u8>(),
                    ((((show).wrapping_add(25)).read()) as i32),
                );
                StringCopy(
                    (&raw mut gStringVar2).cast::<u8>(),
                    (((&raw mut gSpeciesNames).cast::<u8>()).wrapping_offset(
                        ((((show).wrapping_add(22).cast::<u16>()).read()) as i32) as isize * 11,
                    ))
                    .cast::<u8>(),
                );
                StringCopy(
                    (&raw mut gStringVar3).cast::<u8>(),
                    (((&raw mut gMoveNames).cast::<u8>()).wrapping_offset(
                        ((((show).wrapping_add(20).cast::<u16>()).read()) as i32) as isize * 13,
                    ))
                    .cast::<u8>(),
                );
                ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(3u8);
                break 'l1;
            }
            if __sw1 == 3i32 {
                TVShowConvertInternationalString(
                    (&raw mut gStringVar1).cast::<u8>(),
                    ((show).wrapping_add(12)).cast::<u8>(),
                    ((((show).wrapping_add(26)).read()) as i32),
                );
                StringCopy(
                    (&raw mut gStringVar2).cast::<u8>(),
                    (((&raw mut gSpeciesNames).cast::<u8>()).wrapping_offset(
                        ((((show).wrapping_add(2).cast::<u16>()).read()) as i32) as isize * 11,
                    ))
                    .cast::<u8>(),
                );
                ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(4u8);
                break 'l1;
            }
            if __sw1 == 4i32 {
                TVShowConvertInternationalString(
                    (&raw mut gStringVar1).cast::<u8>(),
                    ((show).wrapping_add(4)).cast::<u8>(),
                    ((((show).wrapping_add(25)).read()) as i32),
                );
                TVShowConvertInternationalString(
                    (&raw mut gStringVar2).cast::<u8>(),
                    ((show).wrapping_add(12)).cast::<u8>(),
                    ((((show).wrapping_add(26)).read()) as i32),
                );
                TVShowDone();
                break 'l1;
            }
            if __sw1 == 5i32 {
                TVShowConvertInternationalString(
                    (&raw mut gStringVar1).cast::<u8>(),
                    ((show).wrapping_add(4)).cast::<u8>(),
                    ((((show).wrapping_add(25)).read()) as i32),
                );
                TVShowConvertInternationalString(
                    (&raw mut gStringVar2).cast::<u8>(),
                    ((show).wrapping_add(12)).cast::<u8>(),
                    ((((show).wrapping_add(26)).read()) as i32),
                );
                ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(6u8);
                break 'l1;
            }
            if __sw1 == 6i32 {
                TVShowConvertInternationalString(
                    (&raw mut gStringVar1).cast::<u8>(),
                    ((show).wrapping_add(4)).cast::<u8>(),
                    ((((show).wrapping_add(25)).read()) as i32),
                );
                StringCopy(
                    (&raw mut gStringVar2).cast::<u8>(),
                    (((&raw mut gSpeciesNames).cast::<u8>()).wrapping_offset(
                        ((((show).wrapping_add(22).cast::<u16>()).read()) as i32) as isize * 11,
                    ))
                    .cast::<u8>(),
                );
                StringCopy(
                    (&raw mut gStringVar3).cast::<u8>(),
                    (((&raw mut gMoveNames).cast::<u8>()).wrapping_offset(
                        ((((show).wrapping_add(20).cast::<u16>()).read()) as i32) as isize * 13,
                    ))
                    .cast::<u8>(),
                );
                ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(7u8);
                break 'l1;
            }
            if __sw1 == 7i32 {
                TVShowConvertInternationalString(
                    (&raw mut gStringVar1).cast::<u8>(),
                    ((show).wrapping_add(4)).cast::<u8>(),
                    ((((show).wrapping_add(25)).read()) as i32),
                );
                TVShowConvertInternationalString(
                    (&raw mut gStringVar2).cast::<u8>(),
                    ((show).wrapping_add(12)).cast::<u8>(),
                    ((((show).wrapping_add(26)).read()) as i32),
                );
                StringCopy(
                    (&raw mut gStringVar3).cast::<u8>(),
                    (((&raw mut gSpeciesNames).cast::<u8>()).wrapping_offset(
                        ((((show).wrapping_add(2).cast::<u16>()).read()) as i32) as isize * 11,
                    ))
                    .cast::<u8>(),
                );
                TVShowDone();
                break 'l1;
            }
        }
        ShowFieldMessage(
            ((((&raw const sTVPokemonBattleUpdateTextGroup)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u8>())
            .cast::<*mut u8>())
            .wrapping_offset(((state) as i32) as isize))
            .read(),
        );
    }
}
pub(crate) unsafe extern "C" fn DoTVShow3CheersForPokeblocks() {
    unsafe {
        let mut show: *mut u8 = core::ptr::null_mut();
        let mut state: u8 = 0u8;
        show = (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(10188))
            .cast::<u8>())
        .wrapping_offset(
            ((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32) as isize * 36,
        );
        ((&raw mut gSpecialVar_Result).cast::<u16>()).write(0u16);
        state = ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).read();
        'l1: {
            let __sw1 = ((state) as i32);
            if __sw1 == 0i32 {
                TVShowConvertInternationalString(
                    (&raw mut gStringVar1).cast::<u8>(),
                    ((show).wrapping_add(12)).cast::<u8>(),
                    ((((show).wrapping_add(20)).read()) as i32),
                );
                if ((((show).wrapping_add(2)).read()) as i32) > 20i32 {
                    ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(1u8);
                } else {
                    ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(3u8);
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                'l2: {
                    let __sw2 =
                        ((crate::c::bf_read((show).wrapping_add(3), 0, 3, false) as u8) as i32);
                    if __sw2 == 0i32 {
                        StringCopy(
                            (&raw mut gStringVar1).cast::<u8>(),
                            (&raw mut gText_Spicy2).cast::<u8>(),
                        );
                        break 'l2;
                    }
                    if __sw2 == 1i32 {
                        StringCopy(
                            (&raw mut gStringVar1).cast::<u8>(),
                            (&raw mut gText_Dry2).cast::<u8>(),
                        );
                        break 'l2;
                    }
                    if __sw2 == 2i32 {
                        StringCopy(
                            (&raw mut gStringVar1).cast::<u8>(),
                            (&raw mut gText_Sweet2).cast::<u8>(),
                        );
                        break 'l2;
                    }
                    if __sw2 == 3i32 {
                        StringCopy(
                            (&raw mut gStringVar1).cast::<u8>(),
                            (&raw mut gText_Bitter2).cast::<u8>(),
                        );
                        break 'l2;
                    }
                    if __sw2 == 4i32 {
                        StringCopy(
                            (&raw mut gStringVar1).cast::<u8>(),
                            (&raw mut gText_Sour2).cast::<u8>(),
                        );
                        break 'l2;
                    }
                }
                if ((((show).wrapping_add(2)).read()) as i32) > 24i32 {
                    StringCopy(
                        (&raw mut gStringVar2).cast::<u8>(),
                        (&raw mut gText_Excellent).cast::<u8>(),
                    );
                } else {
                    if ((((show).wrapping_add(2)).read()) as i32) > 22i32 {
                        StringCopy(
                            (&raw mut gStringVar2).cast::<u8>(),
                            (&raw mut gText_VeryGood).cast::<u8>(),
                        );
                    } else {
                        StringCopy(
                            (&raw mut gStringVar2).cast::<u8>(),
                            (&raw mut gText_Good).cast::<u8>(),
                        );
                    }
                }
                TVShowConvertInternationalString(
                    (&raw mut gStringVar3).cast::<u8>(),
                    ((show).wrapping_add(12)).cast::<u8>(),
                    ((((show).wrapping_add(20)).read()) as i32),
                );
                ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(2u8);
                break 'l1;
            }
            if __sw1 == 2i32 {
                TVShowConvertInternationalString(
                    (&raw mut gStringVar1).cast::<u8>(),
                    ((show).wrapping_add(4)).cast::<u8>(),
                    ((((show).wrapping_add(21)).read()) as i32),
                );
                ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(5u8);
                break 'l1;
            }
            if __sw1 == 3i32 {
                'l3: {
                    let __sw3 =
                        ((crate::c::bf_read((show).wrapping_add(3), 0, 3, false) as u8) as i32);
                    if __sw3 == 0i32 {
                        StringCopy(
                            (&raw mut gStringVar1).cast::<u8>(),
                            (&raw mut gText_Spicy2).cast::<u8>(),
                        );
                        break 'l3;
                    }
                    if __sw3 == 1i32 {
                        StringCopy(
                            (&raw mut gStringVar1).cast::<u8>(),
                            (&raw mut gText_Dry2).cast::<u8>(),
                        );
                        break 'l3;
                    }
                    if __sw3 == 2i32 {
                        StringCopy(
                            (&raw mut gStringVar1).cast::<u8>(),
                            (&raw mut gText_Sweet2).cast::<u8>(),
                        );
                        break 'l3;
                    }
                    if __sw3 == 3i32 {
                        StringCopy(
                            (&raw mut gStringVar1).cast::<u8>(),
                            (&raw mut gText_Bitter2).cast::<u8>(),
                        );
                        break 'l3;
                    }
                    if __sw3 == 4i32 {
                        StringCopy(
                            (&raw mut gStringVar1).cast::<u8>(),
                            (&raw mut gText_Sour2).cast::<u8>(),
                        );
                        break 'l3;
                    }
                }
                if ((((show).wrapping_add(2)).read()) as i32) > 16i32 {
                    StringCopy(
                        (&raw mut gStringVar2).cast::<u8>(),
                        (&raw mut gText_SoSo).cast::<u8>(),
                    );
                } else {
                    if ((((show).wrapping_add(2)).read()) as i32) > 13i32 {
                        StringCopy(
                            (&raw mut gStringVar2).cast::<u8>(),
                            (&raw mut gText_Bad).cast::<u8>(),
                        );
                    } else {
                        StringCopy(
                            (&raw mut gStringVar2).cast::<u8>(),
                            (&raw mut gText_TheWorst).cast::<u8>(),
                        );
                    }
                }
                TVShowConvertInternationalString(
                    (&raw mut gStringVar3).cast::<u8>(),
                    ((show).wrapping_add(12)).cast::<u8>(),
                    ((((show).wrapping_add(20)).read()) as i32),
                );
                ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(4u8);
                break 'l1;
            }
            if __sw1 == 4i32 {
                TVShowConvertInternationalString(
                    (&raw mut gStringVar1).cast::<u8>(),
                    ((show).wrapping_add(4)).cast::<u8>(),
                    ((((show).wrapping_add(21)).read()) as i32),
                );
                TVShowConvertInternationalString(
                    (&raw mut gStringVar2).cast::<u8>(),
                    ((show).wrapping_add(12)).cast::<u8>(),
                    ((((show).wrapping_add(20)).read()) as i32),
                );
                ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(5u8);
                break 'l1;
            }
            if __sw1 == 5i32 {
                TVShowDone();
                break 'l1;
            }
        }
        ShowFieldMessage(
            ((((&raw const sTV3CheersForPokeblocksTextGroup)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u8>())
            .cast::<*mut u8>())
            .wrapping_offset(((state) as i32) as isize))
            .read(),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DoTVShowInSearchOfTrainers() {
    unsafe {
        let mut state: u8 = 0u8;
        ((&raw mut gSpecialVar_Result).cast::<u16>()).write(0u16);
        state = ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).read();
        'l1: {
            let __sw1 = ((state) as i32);
            if __sw1 == 0i32 {
                GetMapName(
                    (&raw mut gStringVar1).cast::<u8>(),
                    (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(11172))
                    .wrapping_add(8))
                    .read()) as u16),
                    0u16,
                );
                if (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(11172))
                    .wrapping_add(9))
                .read()) as i32)
                    > 1i32
                {
                    ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(1u8);
                } else {
                    ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(2u8);
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(2u8);
                break 'l1;
            }
            if __sw1 == 2i32 {
                if !((crate::c::bf_read(
                    ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(11172))
                        .wrapping_add(10),
                    0,
                    1,
                    false,
                ) as u8)
                    != 0)
                {
                    ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(4u8);
                } else {
                    if (crate::c::bf_read(
                        ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(11172))
                        .wrapping_add(10),
                        3,
                        1,
                        false,
                    ) as u8)
                        != 0
                    {
                        ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(5u8);
                    } else {
                        if (crate::c::bf_read(
                            ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(11172))
                            .wrapping_add(10),
                            2,
                            1,
                            false,
                        ) as u8)
                            != 0
                        {
                            ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(6u8);
                        } else {
                            if (crate::c::bf_read(
                                ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                                    .wrapping_add(11172))
                                .wrapping_add(10),
                                1,
                                1,
                                false,
                            ) as u8)
                                != 0
                            {
                                ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(7u8);
                            } else {
                                ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(3u8);
                            }
                        }
                    }
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                StringCopy(
                    (&raw mut gStringVar1).cast::<u8>(),
                    (((&raw mut gSpeciesNames).cast::<u8>()).wrapping_offset(
                        (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(11172))
                        .cast::<u16>())
                        .read()) as i32) as isize
                            * 11,
                    ))
                    .cast::<u8>(),
                );
                StringCopy(
                    (&raw mut gStringVar2).cast::<u8>(),
                    (((&raw mut gMoveNames).cast::<u8>()).wrapping_offset(
                        (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(11172))
                        .wrapping_add(4)
                        .cast::<u16>())
                        .read()) as i32) as isize
                            * 13,
                    ))
                    .cast::<u8>(),
                );
                StringCopy(
                    (&raw mut gStringVar3).cast::<u8>(),
                    (((&raw mut gSpeciesNames).cast::<u8>()).wrapping_offset(
                        (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(11172))
                        .wrapping_add(2)
                        .cast::<u16>())
                        .read()) as i32) as isize
                            * 11,
                    ))
                    .cast::<u8>(),
                );
                ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(8u8);
                break 'l1;
            }
            if __sw1 == 4i32 || __sw1 == 5i32 || __sw1 == 6i32 || __sw1 == 7i32 {
                ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(8u8);
                break 'l1;
            }
            if __sw1 == 8i32 {
                CopyEasyChatWord(
                    (&raw mut gStringVar1).cast::<u8>(),
                    ((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(11172))
                    .wrapping_add(6))
                    .cast::<u16>())
                    .read(),
                );
                StringCopy(
                    (&raw mut gStringVar2).cast::<u8>(),
                    (((&raw mut gSpeciesNames).cast::<u8>()).wrapping_offset(
                        (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(11172))
                        .cast::<u16>())
                        .read()) as i32) as isize
                            * 11,
                    ))
                    .cast::<u8>(),
                );
                StringCopy(
                    (&raw mut gStringVar3).cast::<u8>(),
                    (((&raw mut gSpeciesNames).cast::<u8>()).wrapping_offset(
                        (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(11172))
                        .wrapping_add(2)
                        .cast::<u16>())
                        .read()) as i32) as isize
                            * 11,
                    ))
                    .cast::<u8>(),
                );
                ((&raw mut gSpecialVar_Result).cast::<u16>()).write(1u16);
                ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(0u8);
                TakeGabbyAndTyOffTheAir();
                break 'l1;
            }
        }
        ShowFieldMessage(
            ((((&raw const sTVInSearchOfTrainersTextGroup)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u8>())
            .cast::<*mut u8>())
            .wrapping_offset(((state) as i32) as isize))
            .read(),
        );
    }
}
pub(crate) unsafe extern "C" fn DoTVShowPokemonAngler() {
    unsafe {
        let mut show: *mut u8 = core::ptr::null_mut();
        let mut state: u8 = 0u8;
        show = (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(10188))
            .cast::<u8>())
        .wrapping_offset(
            ((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32) as isize * 36,
        );
        ((&raw mut gSpecialVar_Result).cast::<u16>()).write(0u16);
        if ((((show).wrapping_add(2)).read()) as i32) < ((((show).wrapping_add(3)).read()) as i32) {
            ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(0u8);
        } else {
            ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(1u8);
        }
        state = ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).read();
        'l1: {
            let __sw1 = ((state) as i32);
            if __sw1 == 0i32 {
                TVShowConvertInternationalString(
                    (&raw mut gStringVar1).cast::<u8>(),
                    ((show).wrapping_add(19)).cast::<u8>(),
                    ((((show).wrapping_add(6)).read()) as i32),
                );
                StringCopy(
                    (&raw mut gStringVar2).cast::<u8>(),
                    (((&raw mut gSpeciesNames).cast::<u8>()).wrapping_offset(
                        ((((show).wrapping_add(4).cast::<u16>()).read()) as i32) as isize * 11,
                    ))
                    .cast::<u8>(),
                );
                ConvertIntToDecimalString(2u8, ((((show).wrapping_add(3)).read()) as i32));
                TVShowDone();
                break 'l1;
            }
            if __sw1 == 1i32 {
                TVShowConvertInternationalString(
                    (&raw mut gStringVar1).cast::<u8>(),
                    ((show).wrapping_add(19)).cast::<u8>(),
                    ((((show).wrapping_add(6)).read()) as i32),
                );
                StringCopy(
                    (&raw mut gStringVar2).cast::<u8>(),
                    (((&raw mut gSpeciesNames).cast::<u8>()).wrapping_offset(
                        ((((show).wrapping_add(4).cast::<u16>()).read()) as i32) as isize * 11,
                    ))
                    .cast::<u8>(),
                );
                ConvertIntToDecimalString(2u8, ((((show).wrapping_add(2)).read()) as i32));
                TVShowDone();
                break 'l1;
            }
        }
        ShowFieldMessage(
            ((((&raw const sTVPokemonAnglerTextGroup)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u8>())
            .cast::<*mut u8>())
            .wrapping_offset(((state) as i32) as isize))
            .read(),
        );
    }
}
pub(crate) unsafe extern "C" fn DoTVShowTheWorldOfMasters() {
    unsafe {
        let mut show: *mut u8 = core::ptr::null_mut();
        let mut state: u8 = 0u8;
        show = (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(10188))
            .cast::<u8>())
        .wrapping_offset(
            ((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32) as isize * 36,
        );
        ((&raw mut gSpecialVar_Result).cast::<u16>()).write(0u16);
        state = ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).read();
        'l1: {
            let __sw1 = ((state) as i32);
            if __sw1 == 0i32 {
                TVShowConvertInternationalString(
                    (&raw mut gStringVar1).cast::<u8>(),
                    ((show).wrapping_add(19)).cast::<u8>(),
                    ((((show).wrapping_add(11)).read()) as i32),
                );
                ConvertIntToDecimalString(
                    1u8,
                    ((((show).wrapping_add(6).cast::<u16>()).read()) as i32),
                );
                ConvertIntToDecimalString(
                    2u8,
                    ((((show).wrapping_add(2).cast::<u16>()).read()) as i32),
                );
                ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(1u8);
                break 'l1;
            }
            if __sw1 == 1i32 {
                StringCopy(
                    (&raw mut gStringVar1).cast::<u8>(),
                    (((&raw mut gSpeciesNames).cast::<u8>()).wrapping_offset(
                        ((((show).wrapping_add(8).cast::<u16>()).read()) as i32) as isize * 11,
                    ))
                    .cast::<u8>(),
                );
                ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(2u8);
                break 'l1;
            }
            if __sw1 == 2i32 {
                TVShowConvertInternationalString(
                    (&raw mut gStringVar1).cast::<u8>(),
                    ((show).wrapping_add(19)).cast::<u8>(),
                    ((((show).wrapping_add(11)).read()) as i32),
                );
                GetMapName(
                    (&raw mut gStringVar2).cast::<u8>(),
                    ((((show).wrapping_add(10)).read()) as u16),
                    0u16,
                );
                StringCopy(
                    (&raw mut gStringVar3).cast::<u8>(),
                    (((&raw mut gSpeciesNames).cast::<u8>()).wrapping_offset(
                        ((((show).wrapping_add(4).cast::<u16>()).read()) as i32) as isize * 11,
                    ))
                    .cast::<u8>(),
                );
                TVShowDone();
                break 'l1;
            }
        }
        ShowFieldMessage(
            ((((&raw const sTVWorldOfMastersTextGroup)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u8>())
            .cast::<*mut u8>())
            .wrapping_offset(((state) as i32) as isize))
            .read(),
        );
    }
}
pub(crate) unsafe extern "C" fn DoTVShowTodaysRivalTrainer() {
    unsafe {
        let mut show: *mut u8 = core::ptr::null_mut();
        let mut state: u8 = 0u8;
        show = (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(10188))
            .cast::<u8>())
        .wrapping_offset(
            ((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32) as isize * 36,
        );
        ((&raw mut gSpecialVar_Result).cast::<u16>()).write(0u16);
        state = ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).read();
        'l1: {
            let __sw1 = ((state) as i32);
            if __sw1 == 0i32 {
                'l2: {
                    let __sw2 = ((((show).wrapping_add(7)).read()) as i32);
                    let __matched = __sw2 == 86i32 || __sw2 == 87i32;
                    if !__matched {
                        ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(7u8);
                        break 'l2;
                    }
                    if __sw2 == 86i32 {
                        ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(8u8);
                        break 'l2;
                    }
                    if __sw2 == 87i32 {
                        'l3: {
                            let __sw3 = ((((show).wrapping_add(10).cast::<u16>()).read()) as i32);
                            let __matched = __sw3 == 277i32 || __sw3 == 278i32 || __sw3 == 279i32;
                            if __sw3 == 277i32 || __sw3 == 278i32 || __sw3 == 279i32 {
                                ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(10u8);
                                break 'l3;
                            }
                            if !__matched {
                                ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(9u8);
                                break 'l3;
                            }
                        }
                        break 'l2;
                    }
                }
                break 'l1;
            }
            if __sw1 == 7i32 {
                TVShowConvertInternationalString(
                    (&raw mut gStringVar1).cast::<u8>(),
                    ((show).wrapping_add(19)).cast::<u8>(),
                    ((((show).wrapping_add(12)).read()) as i32),
                );
                ConvertIntToDecimalString(
                    1u8,
                    ((((show).wrapping_add(2).cast::<u16>()).read()) as i32),
                );
                GetMapName(
                    (&raw mut gStringVar3).cast::<u8>(),
                    ((((show).wrapping_add(7)).read()) as u16),
                    0u16,
                );
                if ((((show).wrapping_add(4)).read()) as i32) != 0i32 {
                    ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(1u8);
                } else {
                    ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(2u8);
                }
                break 'l1;
            }
            if __sw1 == 8i32 {
                TVShowConvertInternationalString(
                    (&raw mut gStringVar1).cast::<u8>(),
                    ((show).wrapping_add(19)).cast::<u8>(),
                    ((((show).wrapping_add(12)).read()) as i32),
                );
                ConvertIntToDecimalString(
                    1u8,
                    ((((show).wrapping_add(2).cast::<u16>()).read()) as i32),
                );
                if ((((show).wrapping_add(4)).read()) as i32) != 0i32 {
                    ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(1u8);
                } else {
                    ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(2u8);
                }
                break 'l1;
            }
            if __sw1 == 9i32 {
                TVShowConvertInternationalString(
                    (&raw mut gStringVar1).cast::<u8>(),
                    ((show).wrapping_add(19)).cast::<u8>(),
                    ((((show).wrapping_add(12)).read()) as i32),
                );
                ConvertIntToDecimalString(
                    1u8,
                    ((((show).wrapping_add(2).cast::<u16>()).read()) as i32),
                );
                if ((((show).wrapping_add(4)).read()) as i32) != 0i32 {
                    ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(1u8);
                } else {
                    ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(2u8);
                }
                break 'l1;
            }
            if __sw1 == 10i32 {
                TVShowConvertInternationalString(
                    (&raw mut gStringVar1).cast::<u8>(),
                    ((show).wrapping_add(19)).cast::<u8>(),
                    ((((show).wrapping_add(12)).read()) as i32),
                );
                ConvertIntToDecimalString(
                    1u8,
                    ((((show).wrapping_add(2).cast::<u16>()).read()) as i32),
                );
                if ((((show).wrapping_add(4)).read()) as i32) != 0i32 {
                    ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(1u8);
                } else {
                    ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(2u8);
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                ConvertIntToDecimalString(0u8, ((((show).wrapping_add(4)).read()) as i32));
                if (FlagGet(2216u16)) != 0 {
                    if ((((show).wrapping_add(5)).read()) != 0)
                        || ((((show).wrapping_add(6)).read()) != 0)
                    {
                        ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(4u8);
                    } else {
                        ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(3u8);
                    }
                } else {
                    ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(6u8);
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                if (FlagGet(2216u16)) != 0 {
                    if ((((show).wrapping_add(5)).read()) != 0)
                        || ((((show).wrapping_add(6)).read()) != 0)
                    {
                        ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(4u8);
                    } else {
                        ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(3u8);
                    }
                } else {
                    ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(6u8);
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                if ((((show).wrapping_add(8).cast::<u16>()).read()) as i32) == 0i32 {
                    ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(6u8);
                } else {
                    ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(5u8);
                }
                break 'l1;
            }
            if __sw1 == 4i32 {
                ConvertIntToDecimalString(0u8, ((((show).wrapping_add(6)).read()) as i32));
                ConvertIntToDecimalString(1u8, ((((show).wrapping_add(5)).read()) as i32));
                if ((((show).wrapping_add(8).cast::<u16>()).read()) as i32) == 0i32 {
                    ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(6u8);
                } else {
                    ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(5u8);
                }
                break 'l1;
            }
            if __sw1 == 5i32 {
                ConvertIntToDecimalString(
                    0u8,
                    ((((show).wrapping_add(8).cast::<u16>()).read()) as i32),
                );
                ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(6u8);
                break 'l1;
            }
            if __sw1 == 6i32 {
                TVShowConvertInternationalString(
                    (&raw mut gStringVar1).cast::<u8>(),
                    ((show).wrapping_add(19)).cast::<u8>(),
                    ((((show).wrapping_add(12)).read()) as i32),
                );
                TVShowDone();
            }
        }
        ShowFieldMessage(
            ((((&raw const sTVTodaysRivalTrainerTextGroup)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u8>())
            .cast::<*mut u8>())
            .wrapping_offset(((state) as i32) as isize))
            .read(),
        );
    }
}
pub(crate) unsafe extern "C" fn DoTVShowDewfordTrendWatcherNetwork() {
    unsafe {
        let mut show: *mut u8 = core::ptr::null_mut();
        let mut state: u8 = 0u8;
        show = (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(10188))
            .cast::<u8>())
        .wrapping_offset(
            ((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32) as isize * 36,
        );
        ((&raw mut gSpecialVar_Result).cast::<u16>()).write(0u16);
        state = ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).read();
        'l1: {
            let __sw1 = ((state) as i32);
            if __sw1 == 0i32 {
                CopyEasyChatWord(
                    (&raw mut gStringVar1).cast::<u8>(),
                    (((show).wrapping_add(4)).cast::<u16>()).read(),
                );
                CopyEasyChatWord(
                    (&raw mut gStringVar2).cast::<u8>(),
                    ((((show).wrapping_add(4)).cast::<u16>()).wrapping_offset(1)).read(),
                );
                if ((((show).wrapping_add(8)).read()) as i32) == 0i32 {
                    ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(1u8);
                } else {
                    ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(2u8);
                }
                break 'l1;
            }
            if __sw1 == 1i32 || __sw1 == 2i32 {
                CopyEasyChatWord(
                    (&raw mut gStringVar1).cast::<u8>(),
                    (((show).wrapping_add(4)).cast::<u16>()).read(),
                );
                CopyEasyChatWord(
                    (&raw mut gStringVar2).cast::<u8>(),
                    ((((show).wrapping_add(4)).cast::<u16>()).wrapping_offset(1)).read(),
                );
                TVShowConvertInternationalString(
                    (&raw mut gStringVar3).cast::<u8>(),
                    ((show).wrapping_add(19)).cast::<u8>(),
                    ((((show).wrapping_add(9)).read()) as i32),
                );
                ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(3u8);
                break 'l1;
            }
            if __sw1 == 3i32 {
                CopyEasyChatWord(
                    (&raw mut gStringVar1).cast::<u8>(),
                    (((show).wrapping_add(4)).cast::<u16>()).read(),
                );
                CopyEasyChatWord(
                    (&raw mut gStringVar2).cast::<u8>(),
                    ((((show).wrapping_add(4)).cast::<u16>()).wrapping_offset(1)).read(),
                );
                if ((((show).wrapping_add(8)).read()) as i32) == 0i32 {
                    ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(4u8);
                } else {
                    ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(5u8);
                }
                break 'l1;
            }
            if __sw1 == 4i32 || __sw1 == 5i32 {
                CopyEasyChatWord(
                    (&raw mut gStringVar1).cast::<u8>(),
                    (((show).wrapping_add(4)).cast::<u16>()).read(),
                );
                CopyEasyChatWord(
                    (&raw mut gStringVar2).cast::<u8>(),
                    ((((show).wrapping_add(4)).cast::<u16>()).wrapping_offset(1)).read(),
                );
                TVShowConvertInternationalString(
                    (&raw mut gStringVar3).cast::<u8>(),
                    ((show).wrapping_add(19)).cast::<u8>(),
                    ((((show).wrapping_add(9)).read()) as i32),
                );
                ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(6u8);
                break 'l1;
            }
            if __sw1 == 6i32 {
                CopyEasyChatWord(
                    (&raw mut gStringVar1).cast::<u8>(),
                    (((show).wrapping_add(4)).cast::<u16>()).read(),
                );
                CopyEasyChatWord(
                    (&raw mut gStringVar2).cast::<u8>(),
                    ((((show).wrapping_add(4)).cast::<u16>()).wrapping_offset(1)).read(),
                );
                TVShowDone();
            }
        }
        ShowFieldMessage(
            ((((&raw const sTVDewfordTrendWatcherNetworkTextGroup)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u8>())
            .cast::<*mut u8>())
            .wrapping_offset(((state) as i32) as isize))
            .read(),
        );
    }
}
pub(crate) unsafe extern "C" fn DoTVShowHoennTreasureInvestigators() {
    unsafe {
        let mut show: *mut u8 = core::ptr::null_mut();
        let mut state: u8 = 0u8;
        show = (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(10188))
            .cast::<u8>())
        .wrapping_offset(
            ((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32) as isize * 36,
        );
        ((&raw mut gSpecialVar_Result).cast::<u16>()).write(0u16);
        state = ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).read();
        'l1: {
            let __sw1 = ((state) as i32);
            if __sw1 == 0i32 {
                StringCopy(
                    (&raw mut gStringVar1).cast::<u8>(),
                    GetItemName(((show).wrapping_add(2).cast::<u16>()).read()),
                );
                if ((((show).wrapping_add(4)).read()) as i32) == 87i32 {
                    'l2: {
                        let __sw2 = ((((show).wrapping_add(6).cast::<u16>()).read()) as i32);
                        let __matched = __sw2 == 277i32 || __sw2 == 278i32 || __sw2 == 279i32;
                        if __sw2 == 277i32 || __sw2 == 278i32 || __sw2 == 279i32 {
                            ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(2u8);
                            break 'l2;
                        }
                        if !__matched {
                            ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(1u8);
                            break 'l2;
                        }
                    }
                } else {
                    ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(1u8);
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                StringCopy(
                    (&raw mut gStringVar1).cast::<u8>(),
                    GetItemName(((show).wrapping_add(2).cast::<u16>()).read()),
                );
                TVShowConvertInternationalString(
                    (&raw mut gStringVar2).cast::<u8>(),
                    ((show).wrapping_add(19)).cast::<u8>(),
                    ((((show).wrapping_add(5)).read()) as i32),
                );
                GetMapName(
                    (&raw mut gStringVar3).cast::<u8>(),
                    ((((show).wrapping_add(4)).read()) as u16),
                    0u16,
                );
                TVShowDone();
                break 'l1;
            }
            if __sw1 == 2i32 {
                StringCopy(
                    (&raw mut gStringVar1).cast::<u8>(),
                    GetItemName(((show).wrapping_add(2).cast::<u16>()).read()),
                );
                TVShowConvertInternationalString(
                    (&raw mut gStringVar2).cast::<u8>(),
                    ((show).wrapping_add(19)).cast::<u8>(),
                    ((((show).wrapping_add(5)).read()) as i32),
                );
                TVShowDone();
                break 'l1;
            }
        }
        ShowFieldMessage(
            ((((&raw const sTVHoennTreasureInvestisatorsTextGroup)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u8>())
            .cast::<*mut u8>())
            .wrapping_offset(((state) as i32) as isize))
            .read(),
        );
    }
}
pub(crate) unsafe extern "C" fn DoTVShowFindThatGamer() {
    unsafe {
        let mut show: *mut u8 = core::ptr::null_mut();
        let mut state: u8 = 0u8;
        show = (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(10188))
            .cast::<u8>())
        .wrapping_offset(
            ((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32) as isize * 36,
        );
        ((&raw mut gSpecialVar_Result).cast::<u16>()).write(0u16);
        state = ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).read();
        'l1: {
            let __sw1 = ((state) as i32);
            if __sw1 == 0i32 {
                TVShowConvertInternationalString(
                    (&raw mut gStringVar1).cast::<u8>(),
                    ((show).wrapping_add(19)).cast::<u8>(),
                    ((((show).wrapping_add(8)).read()) as i32),
                );
                'l2: {
                    let __sw2 = ((((show).wrapping_add(3)).read()) as i32);
                    if __sw2 == 0i32 {
                        StringCopy(
                            (&raw mut gStringVar2).cast::<u8>(),
                            (&raw mut gText_Slots).cast::<u8>(),
                        );
                        break 'l2;
                    }
                    if __sw2 == 1i32 {
                        StringCopy(
                            (&raw mut gStringVar2).cast::<u8>(),
                            (&raw mut gText_Roulette).cast::<u8>(),
                        );
                        break 'l2;
                    }
                }
                if ((((show).wrapping_add(2)).read()) as i32) == 1i32 {
                    ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(1u8);
                } else {
                    ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(2u8);
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                TVShowConvertInternationalString(
                    (&raw mut gStringVar1).cast::<u8>(),
                    ((show).wrapping_add(19)).cast::<u8>(),
                    ((((show).wrapping_add(8)).read()) as i32),
                );
                'l3: {
                    let __sw3 = ((((show).wrapping_add(3)).read()) as i32);
                    if __sw3 == 0i32 {
                        StringCopy(
                            (&raw mut gStringVar2).cast::<u8>(),
                            (&raw mut gText_Slots).cast::<u8>(),
                        );
                        break 'l3;
                    }
                    if __sw3 == 1i32 {
                        StringCopy(
                            (&raw mut gStringVar2).cast::<u8>(),
                            (&raw mut gText_Roulette).cast::<u8>(),
                        );
                        break 'l3;
                    }
                }
                ConvertIntToDecimalString(
                    2u8,
                    ((((show).wrapping_add(4).cast::<u16>()).read()) as i32),
                );
                TVShowDone();
                break 'l1;
            }
            if __sw1 == 2i32 {
                TVShowConvertInternationalString(
                    (&raw mut gStringVar1).cast::<u8>(),
                    ((show).wrapping_add(19)).cast::<u8>(),
                    ((((show).wrapping_add(8)).read()) as i32),
                );
                'l4: {
                    let __sw4 = ((((show).wrapping_add(3)).read()) as i32);
                    if __sw4 == 0i32 {
                        StringCopy(
                            (&raw mut gStringVar2).cast::<u8>(),
                            (&raw mut gText_Slots).cast::<u8>(),
                        );
                        break 'l4;
                    }
                    if __sw4 == 1i32 {
                        StringCopy(
                            (&raw mut gStringVar2).cast::<u8>(),
                            (&raw mut gText_Roulette).cast::<u8>(),
                        );
                        break 'l4;
                    }
                }
                ConvertIntToDecimalString(
                    2u8,
                    ((((show).wrapping_add(4).cast::<u16>()).read()) as i32),
                );
                ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(3u8);
                break 'l1;
            }
            if __sw1 == 3i32 {
                TVShowConvertInternationalString(
                    (&raw mut gStringVar1).cast::<u8>(),
                    ((show).wrapping_add(19)).cast::<u8>(),
                    ((((show).wrapping_add(8)).read()) as i32),
                );
                'l5: {
                    let __sw5 = ((((show).wrapping_add(3)).read()) as i32);
                    if __sw5 == 0i32 {
                        StringCopy(
                            (&raw mut gStringVar2).cast::<u8>(),
                            (&raw mut gText_Roulette).cast::<u8>(),
                        );
                        break 'l5;
                    }
                    if __sw5 == 1i32 {
                        StringCopy(
                            (&raw mut gStringVar2).cast::<u8>(),
                            (&raw mut gText_Slots).cast::<u8>(),
                        );
                        break 'l5;
                    }
                }
                TVShowDone();
                break 'l1;
            }
        }
        ShowFieldMessage(
            ((((&raw const sTVFindThatGamerTextGroup)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u8>())
            .cast::<*mut u8>())
            .wrapping_offset(((state) as i32) as isize))
            .read(),
        );
    }
}
pub(crate) unsafe extern "C" fn DoTVShowBreakingNewsTV() {
    unsafe {
        let mut show: *mut u8 = core::ptr::null_mut();
        let mut state: u8 = 0u8;
        show = (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(10188))
            .cast::<u8>())
        .wrapping_offset(
            ((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32) as isize * 36,
        );
        ((&raw mut gSpecialVar_Result).cast::<u16>()).write(0u16);
        state = ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).read();
        'l1: {
            let __sw1 = ((state) as i32);
            if __sw1 == 0i32 {
                if ((((show).wrapping_add(5)).read()) as i32) == 0i32 {
                    ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(1u8);
                } else {
                    ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(5u8);
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                TVShowConvertInternationalString(
                    (&raw mut gStringVar1).cast::<u8>(),
                    ((show).wrapping_add(19)).cast::<u8>(),
                    ((((show).wrapping_add(14)).read()) as i32),
                );
                StringCopy(
                    (&raw mut gStringVar2).cast::<u8>(),
                    (((&raw mut gSpeciesNames).cast::<u8>()).wrapping_offset(
                        ((((show).wrapping_add(2).cast::<u16>()).read()) as i32) as isize * 11,
                    ))
                    .cast::<u8>(),
                );
                GetMapName(
                    (&raw mut gStringVar3).cast::<u8>(),
                    ((((show).wrapping_add(4)).read()) as u16),
                    0u16,
                );
                ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(2u8);
                break 'l1;
            }
            if __sw1 == 2i32 {
                TVShowConvertInternationalString(
                    (&raw mut gStringVar1).cast::<u8>(),
                    ((show).wrapping_add(19)).cast::<u8>(),
                    ((((show).wrapping_add(14)).read()) as i32),
                );
                StringCopy(
                    (&raw mut gStringVar2).cast::<u8>(),
                    (((&raw mut gSpeciesNames).cast::<u8>()).wrapping_offset(
                        ((((show).wrapping_add(2).cast::<u16>()).read()) as i32) as isize * 11,
                    ))
                    .cast::<u8>(),
                );
                StringCopy(
                    (&raw mut gStringVar3).cast::<u8>(),
                    (((&raw mut gSpeciesNames).cast::<u8>()).wrapping_offset(
                        ((((show).wrapping_add(10).cast::<u16>()).read()) as i32) as isize * 11,
                    ))
                    .cast::<u8>(),
                );
                ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(3u8);
                break 'l1;
            }
            if __sw1 == 3i32 {
                ConvertIntToDecimalString(
                    0u8,
                    ((((show).wrapping_add(8).cast::<u16>()).read()) as i32),
                );
                StringCopy(
                    (&raw mut gStringVar2).cast::<u8>(),
                    GetItemName(((show).wrapping_add(6).cast::<u16>()).read()),
                );
                ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(4u8);
                break 'l1;
            }
            if __sw1 == 4i32 {
                TVShowConvertInternationalString(
                    (&raw mut gStringVar1).cast::<u8>(),
                    ((show).wrapping_add(19)).cast::<u8>(),
                    ((((show).wrapping_add(14)).read()) as i32),
                );
                GetMapName(
                    (&raw mut gStringVar2).cast::<u8>(),
                    ((((show).wrapping_add(4)).read()) as u16),
                    0u16,
                );
                TVShowDone();
                break 'l1;
            }
            if __sw1 == 5i32 {
                TVShowConvertInternationalString(
                    (&raw mut gStringVar1).cast::<u8>(),
                    ((show).wrapping_add(19)).cast::<u8>(),
                    ((((show).wrapping_add(14)).read()) as i32),
                );
                StringCopy(
                    (&raw mut gStringVar2).cast::<u8>(),
                    (((&raw mut gSpeciesNames).cast::<u8>()).wrapping_offset(
                        ((((show).wrapping_add(2).cast::<u16>()).read()) as i32) as isize * 11,
                    ))
                    .cast::<u8>(),
                );
                GetMapName(
                    (&raw mut gStringVar3).cast::<u8>(),
                    ((((show).wrapping_add(4)).read()) as u16),
                    0u16,
                );
                ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(6u8);
                break 'l1;
            }
            if __sw1 == 6i32 {
                TVShowConvertInternationalString(
                    (&raw mut gStringVar1).cast::<u8>(),
                    ((show).wrapping_add(19)).cast::<u8>(),
                    ((((show).wrapping_add(14)).read()) as i32),
                );
                StringCopy(
                    (&raw mut gStringVar2).cast::<u8>(),
                    (((&raw mut gSpeciesNames).cast::<u8>()).wrapping_offset(
                        ((((show).wrapping_add(2).cast::<u16>()).read()) as i32) as isize * 11,
                    ))
                    .cast::<u8>(),
                );
                StringCopy(
                    (&raw mut gStringVar3).cast::<u8>(),
                    (((&raw mut gSpeciesNames).cast::<u8>()).wrapping_offset(
                        ((((show).wrapping_add(10).cast::<u16>()).read()) as i32) as isize * 11,
                    ))
                    .cast::<u8>(),
                );
                'l2: {
                    let __sw2 = ((((show).wrapping_add(5)).read()) as i32);
                    if __sw2 == 1i32 {
                        if ((((show).wrapping_add(12).cast::<u16>()).read()) as i32) == 0i32 {
                            ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(12u8);
                        } else {
                            ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(7u8);
                        }
                        break 'l2;
                    }
                    if __sw2 == 2i32 {
                        ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(9u8);
                        break 'l2;
                    }
                    if __sw2 == 3i32 {
                        ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(10u8);
                        break 'l2;
                    }
                }
                break 'l1;
            }
            if __sw1 == 7i32 {
                StringCopy(
                    (&raw mut gStringVar1).cast::<u8>(),
                    (((&raw mut gMoveNames).cast::<u8>()).wrapping_offset(
                        ((((show).wrapping_add(12).cast::<u16>()).read()) as i32) as isize * 13,
                    ))
                    .cast::<u8>(),
                );
                StringCopy(
                    (&raw mut gStringVar2).cast::<u8>(),
                    (((&raw mut gSpeciesNames).cast::<u8>()).wrapping_offset(
                        ((((show).wrapping_add(10).cast::<u16>()).read()) as i32) as isize * 11,
                    ))
                    .cast::<u8>(),
                );
                ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(8u8);
                break 'l1;
            }
            if __sw1 == 12i32 {
                TVShowConvertInternationalString(
                    (&raw mut gStringVar1).cast::<u8>(),
                    ((show).wrapping_add(19)).cast::<u8>(),
                    ((((show).wrapping_add(14)).read()) as i32),
                );
                StringCopy(
                    (&raw mut gStringVar2).cast::<u8>(),
                    (((&raw mut gSpeciesNames).cast::<u8>()).wrapping_offset(
                        ((((show).wrapping_add(2).cast::<u16>()).read()) as i32) as isize * 11,
                    ))
                    .cast::<u8>(),
                );
                StringCopy(
                    (&raw mut gStringVar3).cast::<u8>(),
                    (((&raw mut gSpeciesNames).cast::<u8>()).wrapping_offset(
                        ((((show).wrapping_add(10).cast::<u16>()).read()) as i32) as isize * 11,
                    ))
                    .cast::<u8>(),
                );
                ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(8u8);
                break 'l1;
            }
            if __sw1 == 8i32 {
                TVShowConvertInternationalString(
                    (&raw mut gStringVar1).cast::<u8>(),
                    ((show).wrapping_add(19)).cast::<u8>(),
                    ((((show).wrapping_add(14)).read()) as i32),
                );
                GetMapName(
                    (&raw mut gStringVar2).cast::<u8>(),
                    ((((show).wrapping_add(4)).read()) as u16),
                    0u16,
                );
                ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(11u8);
                break 'l1;
            }
            if __sw1 == 9i32 || __sw1 == 10i32 {
                TVShowConvertInternationalString(
                    (&raw mut gStringVar1).cast::<u8>(),
                    ((show).wrapping_add(19)).cast::<u8>(),
                    ((((show).wrapping_add(14)).read()) as i32),
                );
                StringCopy(
                    (&raw mut gStringVar2).cast::<u8>(),
                    (((&raw mut gSpeciesNames).cast::<u8>()).wrapping_offset(
                        ((((show).wrapping_add(2).cast::<u16>()).read()) as i32) as isize * 11,
                    ))
                    .cast::<u8>(),
                );
                GetMapName(
                    (&raw mut gStringVar3).cast::<u8>(),
                    ((((show).wrapping_add(4)).read()) as u16),
                    0u16,
                );
                ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(11u8);
                break 'l1;
            }
            if __sw1 == 11i32 {
                TVShowConvertInternationalString(
                    (&raw mut gStringVar1).cast::<u8>(),
                    ((show).wrapping_add(19)).cast::<u8>(),
                    ((((show).wrapping_add(14)).read()) as i32),
                );
                TVShowDone();
                break 'l1;
            }
        }
        ShowFieldMessage(
            ((((&raw const sTVBreakingNewsTextGroup)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u8>())
            .cast::<*mut u8>())
            .wrapping_offset(((state) as i32) as isize))
            .read(),
        );
    }
}
pub(crate) unsafe extern "C" fn DoTVShowSecretBaseVisit() {
    unsafe {
        let mut show: *mut u8 = core::ptr::null_mut();
        let mut state: u8 = 0u8;
        show = (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(10188))
            .cast::<u8>())
        .wrapping_offset(
            ((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32) as isize * 36,
        );
        ((&raw mut gSpecialVar_Result).cast::<u16>()).write(0u16);
        state = ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).read();
        'l1: {
            let __sw1 = ((state) as i32);
            if __sw1 == 0i32 {
                TVShowConvertInternationalString(
                    (&raw mut gStringVar1).cast::<u8>(),
                    ((show).wrapping_add(19)).cast::<u8>(),
                    ((((show).wrapping_add(12)).read()) as i32),
                );
                if ((((show).wrapping_add(3)).read()) as i32) == 0i32 {
                    ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(2u8);
                } else {
                    ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(1u8);
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                StringCopy(
                    (&raw mut gStringVar2).cast::<u8>(),
                    ((((&raw mut gDecorations).cast::<u8>()).wrapping_offset(
                        (((((show).wrapping_add(4)).cast::<u8>()).read()) as i32) as isize * 32,
                    ))
                    .wrapping_add(1))
                    .cast::<u8>(),
                );
                if ((((show).wrapping_add(3)).read()) as i32) == 1i32 {
                    ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(4u8);
                } else {
                    ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(3u8);
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                StringCopy(
                    (&raw mut gStringVar2).cast::<u8>(),
                    ((((&raw mut gDecorations).cast::<u8>()).wrapping_offset(
                        ((((((show).wrapping_add(4)).cast::<u8>()).wrapping_offset(1)).read())
                            as i32) as isize
                            * 32,
                    ))
                    .wrapping_add(1))
                    .cast::<u8>(),
                );
                'l2: {
                    let __sw2 = ((((show).wrapping_add(3)).read()) as i32);
                    if __sw2 == 2i32 {
                        ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(7u8);
                        break 'l2;
                    }
                    if __sw2 == 3i32 {
                        ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(6u8);
                        break 'l2;
                    }
                    if __sw2 == 4i32 {
                        ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(5u8);
                        break 'l2;
                    }
                }
                break 'l1;
            }
            if __sw1 == 5i32 {
                StringCopy(
                    (&raw mut gStringVar2).cast::<u8>(),
                    ((((&raw mut gDecorations).cast::<u8>()).wrapping_offset(
                        ((((((show).wrapping_add(4)).cast::<u8>()).wrapping_offset(2)).read())
                            as i32) as isize
                            * 32,
                    ))
                    .wrapping_add(1))
                    .cast::<u8>(),
                );
                StringCopy(
                    (&raw mut gStringVar3).cast::<u8>(),
                    ((((&raw mut gDecorations).cast::<u8>()).wrapping_offset(
                        ((((((show).wrapping_add(4)).cast::<u8>()).wrapping_offset(3)).read())
                            as i32) as isize
                            * 32,
                    ))
                    .wrapping_add(1))
                    .cast::<u8>(),
                );
                ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(8u8);
                break 'l1;
            }
            if __sw1 == 6i32 {
                StringCopy(
                    (&raw mut gStringVar2).cast::<u8>(),
                    ((((&raw mut gDecorations).cast::<u8>()).wrapping_offset(
                        ((((((show).wrapping_add(4)).cast::<u8>()).wrapping_offset(2)).read())
                            as i32) as isize
                            * 32,
                    ))
                    .wrapping_add(1))
                    .cast::<u8>(),
                );
                ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(8u8);
                break 'l1;
            }
            if __sw1 == 2i32 || __sw1 == 4i32 || __sw1 == 7i32 {
                ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(8u8);
                break 'l1;
            }
            if __sw1 == 8i32 {
                TVShowConvertInternationalString(
                    (&raw mut gStringVar1).cast::<u8>(),
                    ((show).wrapping_add(19)).cast::<u8>(),
                    ((((show).wrapping_add(12)).read()) as i32),
                );
                if ((((show).wrapping_add(2)).read()) as i32) < 25i32 {
                    ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(12u8);
                } else {
                    if ((((show).wrapping_add(2)).read()) as i32) < 50i32 {
                        ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(11u8);
                    } else {
                        if ((((show).wrapping_add(2)).read()) as i32) < 70i32 {
                            ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(10u8);
                        } else {
                            ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(9u8);
                        }
                    }
                }
                break 'l1;
            }
            if __sw1 == 9i32 || __sw1 == 10i32 || __sw1 == 11i32 || __sw1 == 12i32 {
                TVShowConvertInternationalString(
                    (&raw mut gStringVar1).cast::<u8>(),
                    ((show).wrapping_add(19)).cast::<u8>(),
                    ((((show).wrapping_add(12)).read()) as i32),
                );
                StringCopy(
                    (&raw mut gStringVar2).cast::<u8>(),
                    (((&raw mut gSpeciesNames).cast::<u8>()).wrapping_offset(
                        ((((show).wrapping_add(8).cast::<u16>()).read()) as i32) as isize * 11,
                    ))
                    .cast::<u8>(),
                );
                StringCopy(
                    (&raw mut gStringVar3).cast::<u8>(),
                    (((&raw mut gMoveNames).cast::<u8>()).wrapping_offset(
                        ((((show).wrapping_add(10).cast::<u16>()).read()) as i32) as isize * 13,
                    ))
                    .cast::<u8>(),
                );
                ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(13u8);
                break 'l1;
            }
            if __sw1 == 13i32 {
                TVShowDone();
                break 'l1;
            }
        }
        ShowFieldMessage(
            ((((&raw const sTVSecretBaseVisitTextGroup)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u8>())
            .cast::<*mut u8>())
            .wrapping_offset(((state) as i32) as isize))
            .read(),
        );
    }
}
pub(crate) unsafe extern "C" fn DoTVShowPokemonLotteryWinnerFlashReport() {
    unsafe {
        let mut show: *mut u8 = core::ptr::null_mut();
        let mut state: u8 = 0u8;
        show = (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(10188))
            .cast::<u8>())
        .wrapping_offset(
            ((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32) as isize * 36,
        );
        ((&raw mut gSpecialVar_Result).cast::<u16>()).write(0u16);
        state = ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).read();
        TVShowConvertInternationalString(
            (&raw mut gStringVar1).cast::<u8>(),
            ((show).wrapping_add(19)).cast::<u8>(),
            ((((show).wrapping_add(5)).read()) as i32),
        );
        if ((((show).wrapping_add(4)).read()) as i32) == 0i32 {
            StringCopy(
                (&raw mut gStringVar2).cast::<u8>(),
                (&raw mut gText_Jackpot).cast::<u8>(),
            );
        } else {
            if ((((show).wrapping_add(4)).read()) as i32) == 1i32 {
                StringCopy(
                    (&raw mut gStringVar2).cast::<u8>(),
                    (&raw mut gText_First).cast::<u8>(),
                );
            } else {
                if ((((show).wrapping_add(4)).read()) as i32) == 2i32 {
                    StringCopy(
                        (&raw mut gStringVar2).cast::<u8>(),
                        (&raw mut gText_Second).cast::<u8>(),
                    );
                } else {
                    StringCopy(
                        (&raw mut gStringVar2).cast::<u8>(),
                        (&raw mut gText_Third).cast::<u8>(),
                    );
                }
            }
        }
        StringCopy(
            (&raw mut gStringVar3).cast::<u8>(),
            GetItemName(((show).wrapping_add(2).cast::<u16>()).read()),
        );
        TVShowDone();
        ShowFieldMessage(
            ((((&raw const sTVPokemonLotteryWinnerFlashReportTextGroup)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u8>())
            .cast::<*mut u8>())
            .wrapping_offset(((state) as i32) as isize))
            .read(),
        );
    }
}
pub(crate) unsafe extern "C" fn DoTVShowThePokemonBattleSeminar() {
    unsafe {
        let mut show: *mut u8 = core::ptr::null_mut();
        let mut state: u8 = 0u8;
        show = (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(10188))
            .cast::<u8>())
        .wrapping_offset(
            ((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32) as isize * 36,
        );
        ((&raw mut gSpecialVar_Result).cast::<u16>()).write(0u16);
        state = ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).read();
        'l1: {
            let __sw1 = ((state) as i32);
            if __sw1 == 0i32 {
                TVShowConvertInternationalString(
                    (&raw mut gStringVar1).cast::<u8>(),
                    ((show).wrapping_add(19)).cast::<u8>(),
                    ((((show).wrapping_add(17)).read()) as i32),
                );
                StringCopy(
                    (&raw mut gStringVar2).cast::<u8>(),
                    (((&raw mut gSpeciesNames).cast::<u8>()).wrapping_offset(
                        ((((show).wrapping_add(6).cast::<u16>()).read()) as i32) as isize * 11,
                    ))
                    .cast::<u8>(),
                );
                StringCopy(
                    (&raw mut gStringVar3).cast::<u8>(),
                    (((&raw mut gSpeciesNames).cast::<u8>()).wrapping_offset(
                        ((((show).wrapping_add(4).cast::<u16>()).read()) as i32) as isize * 11,
                    ))
                    .cast::<u8>(),
                );
                ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(1u8);
                break 'l1;
            }
            if __sw1 == 1i32 {
                TVShowConvertInternationalString(
                    (&raw mut gStringVar1).cast::<u8>(),
                    ((show).wrapping_add(19)).cast::<u8>(),
                    ((((show).wrapping_add(17)).read()) as i32),
                );
                StringCopy(
                    (&raw mut gStringVar2).cast::<u8>(),
                    (((&raw mut gSpeciesNames).cast::<u8>()).wrapping_offset(
                        ((((show).wrapping_add(4).cast::<u16>()).read()) as i32) as isize * 11,
                    ))
                    .cast::<u8>(),
                );
                StringCopy(
                    (&raw mut gStringVar3).cast::<u8>(),
                    (((&raw mut gMoveNames).cast::<u8>()).wrapping_offset(
                        ((((show).wrapping_add(2).cast::<u16>()).read()) as i32) as isize * 13,
                    ))
                    .cast::<u8>(),
                );
                ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(2u8);
                break 'l1;
            }
            if __sw1 == 2i32 {
                StringCopy(
                    (&raw mut gStringVar1).cast::<u8>(),
                    (((&raw mut gSpeciesNames).cast::<u8>()).wrapping_offset(
                        ((((show).wrapping_add(6).cast::<u16>()).read()) as i32) as isize * 11,
                    ))
                    .cast::<u8>(),
                );
                'l2: {
                    let __sw2 = ((((show).wrapping_add(16)).read()) as i32);
                    let __matched = __sw2 == 1i32 || __sw2 == 2i32 || __sw2 == 3i32;
                    if __sw2 == 1i32 {
                        ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(5u8);
                        break 'l2;
                    }
                    if __sw2 == 2i32 {
                        ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(4u8);
                        break 'l2;
                    }
                    if __sw2 == 3i32 {
                        ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(3u8);
                        break 'l2;
                    }
                    if !__matched {
                        ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(6u8);
                        break 'l2;
                    }
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                StringCopy(
                    (&raw mut gStringVar1).cast::<u8>(),
                    (((&raw mut gMoveNames).cast::<u8>()).wrapping_offset(
                        (((((show).wrapping_add(8)).cast::<u16>()).read()) as i32) as isize * 13,
                    ))
                    .cast::<u8>(),
                );
                StringCopy(
                    (&raw mut gStringVar2).cast::<u8>(),
                    (((&raw mut gMoveNames).cast::<u8>()).wrapping_offset(
                        ((((((show).wrapping_add(8)).cast::<u16>()).wrapping_offset(1)).read())
                            as i32) as isize
                            * 13,
                    ))
                    .cast::<u8>(),
                );
                StringCopy(
                    (&raw mut gStringVar3).cast::<u8>(),
                    (((&raw mut gMoveNames).cast::<u8>()).wrapping_offset(
                        ((((((show).wrapping_add(8)).cast::<u16>()).wrapping_offset(2)).read())
                            as i32) as isize
                            * 13,
                    ))
                    .cast::<u8>(),
                );
                ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(6u8);
                break 'l1;
            }
            if __sw1 == 4i32 {
                StringCopy(
                    (&raw mut gStringVar1).cast::<u8>(),
                    (((&raw mut gMoveNames).cast::<u8>()).wrapping_offset(
                        (((((show).wrapping_add(8)).cast::<u16>()).read()) as i32) as isize * 13,
                    ))
                    .cast::<u8>(),
                );
                StringCopy(
                    (&raw mut gStringVar2).cast::<u8>(),
                    (((&raw mut gMoveNames).cast::<u8>()).wrapping_offset(
                        ((((((show).wrapping_add(8)).cast::<u16>()).wrapping_offset(1)).read())
                            as i32) as isize
                            * 13,
                    ))
                    .cast::<u8>(),
                );
                ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(6u8);
                break 'l1;
            }
            if __sw1 == 5i32 {
                StringCopy(
                    (&raw mut gStringVar2).cast::<u8>(),
                    (((&raw mut gMoveNames).cast::<u8>()).wrapping_offset(
                        (((((show).wrapping_add(8)).cast::<u16>()).read()) as i32) as isize * 13,
                    ))
                    .cast::<u8>(),
                );
                ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(6u8);
                break 'l1;
            }
            if __sw1 == 6i32 {
                StringCopy(
                    (&raw mut gStringVar1).cast::<u8>(),
                    (((&raw mut gMoveNames).cast::<u8>()).wrapping_offset(
                        ((((show).wrapping_add(14).cast::<u16>()).read()) as i32) as isize * 13,
                    ))
                    .cast::<u8>(),
                );
                StringCopy(
                    (&raw mut gStringVar2).cast::<u8>(),
                    (((&raw mut gMoveNames).cast::<u8>()).wrapping_offset(
                        ((((show).wrapping_add(2).cast::<u16>()).read()) as i32) as isize * 13,
                    ))
                    .cast::<u8>(),
                );
                TVShowDone();
                break 'l1;
            }
        }
        ShowFieldMessage(
            ((((&raw const sTVThePokemonBattleSeminarTextGroup)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u8>())
            .cast::<*mut u8>())
            .wrapping_offset(((state) as i32) as isize))
            .read(),
        );
    }
}
pub(crate) unsafe extern "C" fn DoTVShowTrainerFanClubSpecial() {
    unsafe {
        let mut show: *mut u8 = core::ptr::null_mut();
        let mut state: u8 = 0u8;
        show = (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(10188))
            .cast::<u8>())
        .wrapping_offset(
            ((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32) as isize * 36,
        );
        ((&raw mut gSpecialVar_Result).cast::<u16>()).write(0u16);
        state = ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).read();
        'l1: {
            let __sw1 = ((state) as i32);
            if __sw1 == 0i32 {
                TVShowConvertInternationalString(
                    (&raw mut gStringVar1).cast::<u8>(),
                    ((show).wrapping_add(12)).cast::<u8>(),
                    ((((show).wrapping_add(24)).read()) as i32),
                );
                TVShowConvertInternationalString(
                    (&raw mut gStringVar2).cast::<u8>(),
                    ((show).wrapping_add(2)).cast::<u8>(),
                    ((((show).wrapping_add(23)).read()) as i32),
                );
                CopyEasyChatWord(
                    (&raw mut gStringVar3).cast::<u8>(),
                    (((show).wrapping_add(20)).cast::<u16>()).read(),
                );
                if ((((show).wrapping_add(22)).read()) as i32) >= 90i32 {
                    ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(1u8);
                } else {
                    if ((((show).wrapping_add(22)).read()) as i32) >= 70i32 {
                        ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(2u8);
                    } else {
                        if ((((show).wrapping_add(22)).read()) as i32) >= 30i32 {
                            ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(3u8);
                        } else {
                            ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(4u8);
                        }
                    }
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                TVShowConvertInternationalString(
                    (&raw mut gStringVar1).cast::<u8>(),
                    ((show).wrapping_add(12)).cast::<u8>(),
                    ((((show).wrapping_add(24)).read()) as i32),
                );
                TVShowConvertInternationalString(
                    (&raw mut gStringVar2).cast::<u8>(),
                    ((show).wrapping_add(2)).cast::<u8>(),
                    ((((show).wrapping_add(23)).read()) as i32),
                );
                ConvertIntToDecimalString(2u8, ((((show).wrapping_add(22)).read()) as i32));
                ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(5u8);
                break 'l1;
            }
            if __sw1 == 2i32 {
                TVShowConvertInternationalString(
                    (&raw mut gStringVar1).cast::<u8>(),
                    ((show).wrapping_add(12)).cast::<u8>(),
                    ((((show).wrapping_add(24)).read()) as i32),
                );
                TVShowConvertInternationalString(
                    (&raw mut gStringVar2).cast::<u8>(),
                    ((show).wrapping_add(2)).cast::<u8>(),
                    ((((show).wrapping_add(23)).read()) as i32),
                );
                ConvertIntToDecimalString(2u8, ((((show).wrapping_add(22)).read()) as i32));
                ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(5u8);
                break 'l1;
            }
            if __sw1 == 3i32 {
                TVShowConvertInternationalString(
                    (&raw mut gStringVar1).cast::<u8>(),
                    ((show).wrapping_add(12)).cast::<u8>(),
                    ((((show).wrapping_add(24)).read()) as i32),
                );
                TVShowConvertInternationalString(
                    (&raw mut gStringVar2).cast::<u8>(),
                    ((show).wrapping_add(2)).cast::<u8>(),
                    ((((show).wrapping_add(23)).read()) as i32),
                );
                ConvertIntToDecimalString(2u8, ((((show).wrapping_add(22)).read()) as i32));
                ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(5u8);
                break 'l1;
            }
            if __sw1 == 4i32 {
                TVShowConvertInternationalString(
                    (&raw mut gStringVar1).cast::<u8>(),
                    ((show).wrapping_add(12)).cast::<u8>(),
                    ((((show).wrapping_add(24)).read()) as i32),
                );
                TVShowConvertInternationalString(
                    (&raw mut gStringVar2).cast::<u8>(),
                    ((show).wrapping_add(2)).cast::<u8>(),
                    ((((show).wrapping_add(23)).read()) as i32),
                );
                ConvertIntToDecimalString(2u8, ((((show).wrapping_add(22)).read()) as i32));
                ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(5u8);
                break 'l1;
            }
            if __sw1 == 5i32 {
                TVShowConvertInternationalString(
                    (&raw mut gStringVar1).cast::<u8>(),
                    ((show).wrapping_add(12)).cast::<u8>(),
                    ((((show).wrapping_add(24)).read()) as i32),
                );
                TVShowConvertInternationalString(
                    (&raw mut gStringVar2).cast::<u8>(),
                    ((show).wrapping_add(2)).cast::<u8>(),
                    ((((show).wrapping_add(23)).read()) as i32),
                );
                CopyEasyChatWord(
                    (&raw mut gStringVar3).cast::<u8>(),
                    (((show).wrapping_add(20)).cast::<u16>()).read(),
                );
                TVShowDone();
                break 'l1;
            }
        }
        ShowFieldMessage(
            ((((&raw const sTVTrainerFanClubSpecialTextGroup)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u8>())
            .cast::<*mut u8>())
            .wrapping_offset(((state) as i32) as isize))
            .read(),
        );
    }
}
pub(crate) unsafe extern "C" fn DoTVShowTrainerFanClub() {
    unsafe {
        let mut show: *mut u8 = core::ptr::null_mut();
        let mut state: u8 = 0u8;
        let mut playerId: u32 = 0u32;
        show = (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(10188))
            .cast::<u8>())
        .wrapping_offset(
            ((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32) as isize * 36,
        );
        ((&raw mut gSpecialVar_Result).cast::<u16>()).write(0u16);
        state = ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).read();
        'l1: {
            let __sw1 = ((state) as i32);
            if __sw1 == 0i32 {
                TVShowConvertInternationalString(
                    (&raw mut gStringVar1).cast::<u8>(),
                    ((show).wrapping_add(19)).cast::<u8>(),
                    ((((show).wrapping_add(8)).read()) as i32),
                );
                playerId = (((((((show).wrapping_add(35)).read()) as i32) << 8)
                    .wrapping_add(((((show).wrapping_add(34)).read()) as i32)))
                    as u32);
                'l2: {
                    let __sw2 = crate::c::rem_u32(playerId, 10u32);
                    if __sw2 == 0u32 {
                        ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(1u8);
                        break 'l2;
                    }
                    if __sw2 == 1u32 {
                        ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(2u8);
                        break 'l2;
                    }
                    if __sw2 == 2u32 {
                        ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(3u8);
                        break 'l2;
                    }
                    if __sw2 == 3u32 {
                        ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(4u8);
                        break 'l2;
                    }
                    if __sw2 == 4u32 {
                        ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(5u8);
                        break 'l2;
                    }
                    if __sw2 == 5u32 {
                        ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(6u8);
                        break 'l2;
                    }
                    if __sw2 == 6u32 {
                        ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(7u8);
                        break 'l2;
                    }
                    if __sw2 == 7u32 {
                        ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(8u8);
                        break 'l2;
                    }
                    if __sw2 == 8u32 {
                        ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(9u8);
                        break 'l2;
                    }
                    if __sw2 == 9u32 {
                        ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(10u8);
                        break 'l2;
                    }
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(11u8);
                break 'l1;
            }
            if __sw1 == 2i32 {
                ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(11u8);
                break 'l1;
            }
            if __sw1 == 3i32 {
                ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(11u8);
                break 'l1;
            }
            if __sw1 == 4i32 {
                ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(11u8);
                break 'l1;
            }
            if __sw1 == 5i32 {
                ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(11u8);
                break 'l1;
            }
            if __sw1 == 6i32 {
                ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(11u8);
                break 'l1;
            }
            if __sw1 == 7i32 {
                ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(11u8);
                break 'l1;
            }
            if __sw1 == 8i32 {
                ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(11u8);
                break 'l1;
            }
            if __sw1 == 9i32 {
                ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(11u8);
                break 'l1;
            }
            if __sw1 == 10i32 {
                ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(11u8);
                break 'l1;
            }
            if __sw1 == 11i32 {
                TVShowConvertInternationalString(
                    (&raw mut gStringVar1).cast::<u8>(),
                    ((show).wrapping_add(19)).cast::<u8>(),
                    ((((show).wrapping_add(8)).read()) as i32),
                );
                CopyEasyChatWord(
                    (&raw mut gStringVar2).cast::<u8>(),
                    (((show).wrapping_add(4)).cast::<u16>()).read(),
                );
                CopyEasyChatWord(
                    (&raw mut gStringVar3).cast::<u8>(),
                    ((((show).wrapping_add(4)).cast::<u16>()).wrapping_offset(1)).read(),
                );
                TVShowDone();
                break 'l1;
            }
        }
        ShowFieldMessage(
            ((((&raw const sTVTrainerFanClubTextGroup)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u8>())
            .cast::<*mut u8>())
            .wrapping_offset(((state) as i32) as isize))
            .read(),
        );
    }
}
pub(crate) unsafe extern "C" fn DoTVShowSpotTheCuties() {
    unsafe {
        let mut show: *mut u8 = core::ptr::null_mut();
        let mut state: u8 = 0u8;
        show = (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(10188))
            .cast::<u8>())
        .wrapping_offset(
            ((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32) as isize * 36,
        );
        ((&raw mut gSpecialVar_Result).cast::<u16>()).write(0u16);
        state = ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).read();
        'l1: {
            let __sw1 = ((state) as i32);
            if __sw1 == 0i32 {
                TVShowConvertInternationalString(
                    (&raw mut gStringVar1).cast::<u8>(),
                    ((show).wrapping_add(19)).cast::<u8>(),
                    ((((show).wrapping_add(15)).read()) as i32),
                );
                TVShowConvertInternationalString(
                    (&raw mut gStringVar2).cast::<u8>(),
                    ((show).wrapping_add(4)).cast::<u8>(),
                    ((((show).wrapping_add(16)).read()) as i32),
                );
                if ((((show).wrapping_add(2)).read()) as i32) < 10i32 {
                    ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(1u8);
                } else {
                    if ((((show).wrapping_add(2)).read()) as i32) < 20i32 {
                        ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(2u8);
                    } else {
                        ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(3u8);
                    }
                }
                break 'l1;
            }
            if __sw1 == 1i32 || __sw1 == 2i32 || __sw1 == 3i32 {
                TVShowConvertInternationalString(
                    (&raw mut gStringVar1).cast::<u8>(),
                    ((show).wrapping_add(19)).cast::<u8>(),
                    ((((show).wrapping_add(15)).read()) as i32),
                );
                TVShowConvertInternationalString(
                    (&raw mut gStringVar2).cast::<u8>(),
                    ((show).wrapping_add(4)).cast::<u8>(),
                    ((((show).wrapping_add(16)).read()) as i32),
                );
                ConvertIntToDecimalString(2u8, ((((show).wrapping_add(2)).read()) as i32));
                ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(4u8);
                break 'l1;
            }
            if __sw1 == 4i32 {
                TVShowConvertInternationalString(
                    (&raw mut gStringVar2).cast::<u8>(),
                    ((show).wrapping_add(4)).cast::<u8>(),
                    ((((show).wrapping_add(16)).read()) as i32),
                );
                'l2: {
                    let __sw2 = ((((show).wrapping_add(3)).read()) as i32);
                    if __sw2 == 0i32 {
                        ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(5u8);
                        break 'l2;
                    }
                    if __sw2 == 1i32 || __sw2 == 2i32 || __sw2 == 3i32 || __sw2 == 4i32 {
                        ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(6u8);
                        break 'l2;
                    }
                    if __sw2 == 5i32 || __sw2 == 6i32 || __sw2 == 7i32 || __sw2 == 8i32 {
                        ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(7u8);
                        break 'l2;
                    }
                    if __sw2 == 9i32 || __sw2 == 10i32 || __sw2 == 11i32 || __sw2 == 12i32 {
                        ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(8u8);
                        break 'l2;
                    }
                    if __sw2 == 13i32 || __sw2 == 14i32 || __sw2 == 15i32 || __sw2 == 16i32 {
                        ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(9u8);
                        break 'l2;
                    }
                    if __sw2 == 17i32 || __sw2 == 18i32 || __sw2 == 19i32 || __sw2 == 20i32 {
                        ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(10u8);
                        break 'l2;
                    }
                    if __sw2 == 21i32 {
                        ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(11u8);
                        break 'l2;
                    }
                    if __sw2 == 22i32 {
                        ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(12u8);
                        break 'l2;
                    }
                    if __sw2 == 23i32 {
                        ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(13u8);
                        break 'l2;
                    }
                    if __sw2 == 24i32 {
                        ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(14u8);
                        break 'l2;
                    }
                }
                break 'l1;
            }
            if __sw1 == 5i32
                || __sw1 == 6i32
                || __sw1 == 7i32
                || __sw1 == 8i32
                || __sw1 == 9i32
                || __sw1 == 10i32
                || __sw1 == 11i32
                || __sw1 == 12i32
                || __sw1 == 13i32
                || __sw1 == 14i32
            {
                TVShowConvertInternationalString(
                    (&raw mut gStringVar2).cast::<u8>(),
                    ((show).wrapping_add(4)).cast::<u8>(),
                    ((((show).wrapping_add(16)).read()) as i32),
                );
                ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(15u8);
                break 'l1;
            }
            if __sw1 == 15i32 {
                TVShowDone();
            }
        }
        ShowFieldMessage(
            ((((&raw const sTVCutiesTextGroup)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u8>())
            .cast::<*mut u8>())
            .wrapping_offset(((state) as i32) as isize))
            .read(),
        );
    }
}
pub(crate) unsafe extern "C" fn DoTVShowPokemonNewsBattleFrontier() {
    unsafe {
        let mut show: *mut u8 = core::ptr::null_mut();
        let mut state: u8 = 0u8;
        show = (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(10188))
            .cast::<u8>())
        .wrapping_offset(
            ((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32) as isize * 36,
        );
        ((&raw mut gSpecialVar_Result).cast::<u16>()).write(0u16);
        state = ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).read();
        'l1: {
            let __sw1 = ((state) as i32);
            if __sw1 == 0i32 {
                'l2: {
                    let __sw2 = ((((show).wrapping_add(13)).read()) as i32);
                    if __sw2 == 1i32 {
                        ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(1u8);
                        break 'l2;
                    }
                    if __sw2 == 2i32 {
                        ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(2u8);
                        break 'l2;
                    }
                    if __sw2 == 3i32 {
                        ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(3u8);
                        break 'l2;
                    }
                    if __sw2 == 4i32 {
                        ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(4u8);
                        break 'l2;
                    }
                    if __sw2 == 5i32 {
                        ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(5u8);
                        break 'l2;
                    }
                    if __sw2 == 6i32 {
                        ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(6u8);
                        break 'l2;
                    }
                    if __sw2 == 7i32 {
                        ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(7u8);
                        break 'l2;
                    }
                    if __sw2 == 8i32 {
                        ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(8u8);
                        break 'l2;
                    }
                    if __sw2 == 9i32 {
                        ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(9u8);
                        break 'l2;
                    }
                    if __sw2 == 10i32 {
                        ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(10u8);
                        break 'l2;
                    }
                    if __sw2 == 11i32 {
                        ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(11u8);
                        break 'l2;
                    }
                    if __sw2 == 12i32 {
                        ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(12u8);
                        break 'l2;
                    }
                    if __sw2 == 13i32 {
                        ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(13u8);
                        break 'l2;
                    }
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                TVShowConvertInternationalString(
                    (&raw mut gStringVar1).cast::<u8>(),
                    ((show).wrapping_add(19)).cast::<u8>(),
                    ((((show).wrapping_add(12)).read()) as i32),
                );
                ConvertIntToDecimalString(
                    1u8,
                    ((((show).wrapping_add(2).cast::<u16>()).read()) as i32),
                );
                ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(14u8);
                break 'l1;
            }
            if __sw1 == 2i32 {
                TVShowConvertInternationalString(
                    (&raw mut gStringVar1).cast::<u8>(),
                    ((show).wrapping_add(19)).cast::<u8>(),
                    ((((show).wrapping_add(12)).read()) as i32),
                );
                ConvertIntToDecimalString(
                    1u8,
                    ((((show).wrapping_add(2).cast::<u16>()).read()) as i32),
                );
                ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(16u8);
                break 'l1;
            }
            if __sw1 == 3i32 {
                TVShowConvertInternationalString(
                    (&raw mut gStringVar1).cast::<u8>(),
                    ((show).wrapping_add(19)).cast::<u8>(),
                    ((((show).wrapping_add(12)).read()) as i32),
                );
                ConvertIntToDecimalString(
                    1u8,
                    ((((show).wrapping_add(2).cast::<u16>()).read()) as i32),
                );
                ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(15u8);
                break 'l1;
            }
            if __sw1 == 4i32 {
                TVShowConvertInternationalString(
                    (&raw mut gStringVar1).cast::<u8>(),
                    ((show).wrapping_add(19)).cast::<u8>(),
                    ((((show).wrapping_add(12)).read()) as i32),
                );
                ConvertIntToDecimalString(
                    1u8,
                    ((((show).wrapping_add(2).cast::<u16>()).read()) as i32),
                );
                ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(15u8);
                break 'l1;
            }
            if __sw1 == 5i32 {
                TVShowConvertInternationalString(
                    (&raw mut gStringVar1).cast::<u8>(),
                    ((show).wrapping_add(19)).cast::<u8>(),
                    ((((show).wrapping_add(12)).read()) as i32),
                );
                ConvertIntToDecimalString(
                    1u8,
                    ((((show).wrapping_add(2).cast::<u16>()).read()) as i32),
                );
                ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(14u8);
                break 'l1;
            }
            if __sw1 == 6i32 {
                TVShowConvertInternationalString(
                    (&raw mut gStringVar1).cast::<u8>(),
                    ((show).wrapping_add(19)).cast::<u8>(),
                    ((((show).wrapping_add(12)).read()) as i32),
                );
                ConvertIntToDecimalString(
                    1u8,
                    ((((show).wrapping_add(2).cast::<u16>()).read()) as i32),
                );
                ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(14u8);
                break 'l1;
            }
            if __sw1 == 7i32 {
                TVShowConvertInternationalString(
                    (&raw mut gStringVar1).cast::<u8>(),
                    ((show).wrapping_add(19)).cast::<u8>(),
                    ((((show).wrapping_add(12)).read()) as i32),
                );
                ConvertIntToDecimalString(
                    1u8,
                    ((((show).wrapping_add(2).cast::<u16>()).read()) as i32),
                );
                ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(14u8);
                break 'l1;
            }
            if __sw1 == 8i32 {
                TVShowConvertInternationalString(
                    (&raw mut gStringVar1).cast::<u8>(),
                    ((show).wrapping_add(19)).cast::<u8>(),
                    ((((show).wrapping_add(12)).read()) as i32),
                );
                ConvertIntToDecimalString(
                    1u8,
                    ((((show).wrapping_add(2).cast::<u16>()).read()) as i32),
                );
                ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(14u8);
                break 'l1;
            }
            if __sw1 == 9i32 {
                TVShowConvertInternationalString(
                    (&raw mut gStringVar1).cast::<u8>(),
                    ((show).wrapping_add(19)).cast::<u8>(),
                    ((((show).wrapping_add(12)).read()) as i32),
                );
                ConvertIntToDecimalString(
                    1u8,
                    ((((show).wrapping_add(2).cast::<u16>()).read()) as i32),
                );
                ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(14u8);
                break 'l1;
            }
            if __sw1 == 10i32 {
                TVShowConvertInternationalString(
                    (&raw mut gStringVar1).cast::<u8>(),
                    ((show).wrapping_add(19)).cast::<u8>(),
                    ((((show).wrapping_add(12)).read()) as i32),
                );
                ConvertIntToDecimalString(
                    1u8,
                    ((((show).wrapping_add(2).cast::<u16>()).read()) as i32),
                );
                ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(14u8);
                break 'l1;
            }
            if __sw1 == 11i32 {
                TVShowConvertInternationalString(
                    (&raw mut gStringVar1).cast::<u8>(),
                    ((show).wrapping_add(19)).cast::<u8>(),
                    ((((show).wrapping_add(12)).read()) as i32),
                );
                ConvertIntToDecimalString(
                    1u8,
                    ((((show).wrapping_add(2).cast::<u16>()).read()) as i32),
                );
                ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(14u8);
                break 'l1;
            }
            if __sw1 == 12i32 {
                TVShowConvertInternationalString(
                    (&raw mut gStringVar1).cast::<u8>(),
                    ((show).wrapping_add(19)).cast::<u8>(),
                    ((((show).wrapping_add(12)).read()) as i32),
                );
                ConvertIntToDecimalString(
                    1u8,
                    ((((show).wrapping_add(2).cast::<u16>()).read()) as i32),
                );
                ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(14u8);
                break 'l1;
            }
            if __sw1 == 13i32 {
                TVShowConvertInternationalString(
                    (&raw mut gStringVar1).cast::<u8>(),
                    ((show).wrapping_add(19)).cast::<u8>(),
                    ((((show).wrapping_add(12)).read()) as i32),
                );
                ConvertIntToDecimalString(
                    1u8,
                    ((((show).wrapping_add(2).cast::<u16>()).read()) as i32),
                );
                ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(14u8);
                break 'l1;
            }
            if __sw1 == 14i32 {
                StringCopy(
                    (&raw mut gStringVar1).cast::<u8>(),
                    (((&raw mut gSpeciesNames).cast::<u8>()).wrapping_offset(
                        ((((show).wrapping_add(4).cast::<u16>()).read()) as i32) as isize * 11,
                    ))
                    .cast::<u8>(),
                );
                StringCopy(
                    (&raw mut gStringVar2).cast::<u8>(),
                    (((&raw mut gSpeciesNames).cast::<u8>()).wrapping_offset(
                        ((((show).wrapping_add(6).cast::<u16>()).read()) as i32) as isize * 11,
                    ))
                    .cast::<u8>(),
                );
                StringCopy(
                    (&raw mut gStringVar3).cast::<u8>(),
                    (((&raw mut gSpeciesNames).cast::<u8>()).wrapping_offset(
                        ((((show).wrapping_add(8).cast::<u16>()).read()) as i32) as isize * 11,
                    ))
                    .cast::<u8>(),
                );
                ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(18u8);
                break 'l1;
            }
            if __sw1 == 15i32 {
                StringCopy(
                    (&raw mut gStringVar1).cast::<u8>(),
                    (((&raw mut gSpeciesNames).cast::<u8>()).wrapping_offset(
                        ((((show).wrapping_add(4).cast::<u16>()).read()) as i32) as isize * 11,
                    ))
                    .cast::<u8>(),
                );
                StringCopy(
                    (&raw mut gStringVar2).cast::<u8>(),
                    (((&raw mut gSpeciesNames).cast::<u8>()).wrapping_offset(
                        ((((show).wrapping_add(6).cast::<u16>()).read()) as i32) as isize * 11,
                    ))
                    .cast::<u8>(),
                );
                ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(18u8);
                break 'l1;
            }
            if __sw1 == 16i32 {
                StringCopy(
                    (&raw mut gStringVar1).cast::<u8>(),
                    (((&raw mut gSpeciesNames).cast::<u8>()).wrapping_offset(
                        ((((show).wrapping_add(4).cast::<u16>()).read()) as i32) as isize * 11,
                    ))
                    .cast::<u8>(),
                );
                StringCopy(
                    (&raw mut gStringVar2).cast::<u8>(),
                    (((&raw mut gSpeciesNames).cast::<u8>()).wrapping_offset(
                        ((((show).wrapping_add(6).cast::<u16>()).read()) as i32) as isize * 11,
                    ))
                    .cast::<u8>(),
                );
                StringCopy(
                    (&raw mut gStringVar3).cast::<u8>(),
                    (((&raw mut gSpeciesNames).cast::<u8>()).wrapping_offset(
                        ((((show).wrapping_add(8).cast::<u16>()).read()) as i32) as isize * 11,
                    ))
                    .cast::<u8>(),
                );
                ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(17u8);
                break 'l1;
            }
            if __sw1 == 17i32 {
                StringCopy(
                    (&raw mut gStringVar1).cast::<u8>(),
                    (((&raw mut gSpeciesNames).cast::<u8>()).wrapping_offset(
                        ((((show).wrapping_add(10).cast::<u16>()).read()) as i32) as isize * 11,
                    ))
                    .cast::<u8>(),
                );
                ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(18u8);
                break 'l1;
            }
            if __sw1 == 18i32 {
                TVShowConvertInternationalString(
                    (&raw mut gStringVar1).cast::<u8>(),
                    ((show).wrapping_add(19)).cast::<u8>(),
                    ((((show).wrapping_add(12)).read()) as i32),
                );
                TVShowDone();
                break 'l1;
            }
        }
        ShowFieldMessage(
            ((((&raw const sTVPokemonNewsBattleFrontierTextGroup)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u8>())
            .cast::<*mut u8>())
            .wrapping_offset(((state) as i32) as isize))
            .read(),
        );
    }
}
pub(crate) unsafe extern "C" fn DoTVShowWhatsNo1InHoennToday() {
    unsafe {
        let mut show: *mut u8 = core::ptr::null_mut();
        let mut state: u8 = 0u8;
        show = (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(10188))
            .cast::<u8>())
        .wrapping_offset(
            ((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32) as isize * 36,
        );
        ((&raw mut gSpecialVar_Result).cast::<u16>()).write(0u16);
        state = ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).read();
        'l1: {
            let __sw1 = ((state) as i32);
            if __sw1 == 0i32 {
                TVShowConvertInternationalString(
                    (&raw mut gStringVar1).cast::<u8>(),
                    ((show).wrapping_add(19)).cast::<u8>(),
                    ((((show).wrapping_add(5)).read()) as i32),
                );
                'l2: {
                    let __sw2 = ((((show).wrapping_add(4)).read()) as i32);
                    if __sw2 == 0i32 {
                        ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(1u8);
                        break 'l2;
                    }
                    if __sw2 == 1i32 {
                        ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(2u8);
                        break 'l2;
                    }
                    if __sw2 == 2i32 {
                        ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(3u8);
                        break 'l2;
                    }
                    if __sw2 == 3i32 {
                        ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(4u8);
                        break 'l2;
                    }
                    if __sw2 == 4i32 {
                        ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(5u8);
                        break 'l2;
                    }
                    if __sw2 == 5i32 {
                        ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(6u8);
                        break 'l2;
                    }
                    if __sw2 == 6i32 {
                        ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(7u8);
                        break 'l2;
                    }
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                TVShowConvertInternationalString(
                    (&raw mut gStringVar1).cast::<u8>(),
                    ((show).wrapping_add(19)).cast::<u8>(),
                    ((((show).wrapping_add(5)).read()) as i32),
                );
                ConvertIntToDecimalString(
                    1u8,
                    ((((show).wrapping_add(2).cast::<u16>()).read()) as i32),
                );
                ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(8u8);
                break 'l1;
            }
            if __sw1 == 2i32 {
                TVShowConvertInternationalString(
                    (&raw mut gStringVar1).cast::<u8>(),
                    ((show).wrapping_add(19)).cast::<u8>(),
                    ((((show).wrapping_add(5)).read()) as i32),
                );
                ConvertIntToDecimalString(
                    1u8,
                    ((((show).wrapping_add(2).cast::<u16>()).read()) as i32),
                );
                ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(8u8);
                break 'l1;
            }
            if __sw1 == 3i32 {
                TVShowConvertInternationalString(
                    (&raw mut gStringVar1).cast::<u8>(),
                    ((show).wrapping_add(19)).cast::<u8>(),
                    ((((show).wrapping_add(5)).read()) as i32),
                );
                ConvertIntToDecimalString(
                    1u8,
                    ((((show).wrapping_add(2).cast::<u16>()).read()) as i32),
                );
                ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(8u8);
                break 'l1;
            }
            if __sw1 == 4i32 {
                TVShowConvertInternationalString(
                    (&raw mut gStringVar1).cast::<u8>(),
                    ((show).wrapping_add(19)).cast::<u8>(),
                    ((((show).wrapping_add(5)).read()) as i32),
                );
                ConvertIntToDecimalString(
                    1u8,
                    ((((show).wrapping_add(2).cast::<u16>()).read()) as i32),
                );
                ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(8u8);
                break 'l1;
            }
            if __sw1 == 5i32 {
                TVShowConvertInternationalString(
                    (&raw mut gStringVar1).cast::<u8>(),
                    ((show).wrapping_add(19)).cast::<u8>(),
                    ((((show).wrapping_add(5)).read()) as i32),
                );
                ConvertIntToDecimalString(
                    1u8,
                    ((((show).wrapping_add(2).cast::<u16>()).read()) as i32),
                );
                ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(8u8);
                break 'l1;
            }
            if __sw1 == 6i32 {
                TVShowConvertInternationalString(
                    (&raw mut gStringVar1).cast::<u8>(),
                    ((show).wrapping_add(19)).cast::<u8>(),
                    ((((show).wrapping_add(5)).read()) as i32),
                );
                ConvertIntToDecimalString(
                    1u8,
                    ((((show).wrapping_add(2).cast::<u16>()).read()) as i32),
                );
                ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(8u8);
                break 'l1;
            }
            if __sw1 == 7i32 {
                TVShowConvertInternationalString(
                    (&raw mut gStringVar1).cast::<u8>(),
                    ((show).wrapping_add(19)).cast::<u8>(),
                    ((((show).wrapping_add(5)).read()) as i32),
                );
                ConvertIntToDecimalString(
                    1u8,
                    ((((show).wrapping_add(2).cast::<u16>()).read()) as i32),
                );
                ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(8u8);
                break 'l1;
            }
            if __sw1 == 8i32 {
                TVShowConvertInternationalString(
                    (&raw mut gStringVar1).cast::<u8>(),
                    ((show).wrapping_add(19)).cast::<u8>(),
                    ((((show).wrapping_add(5)).read()) as i32),
                );
                TVShowDone();
                break 'l1;
            }
        }
        ShowFieldMessage(
            ((((&raw const sTVWhatsNo1InHoennTodayTextGroup)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u8>())
            .cast::<*mut u8>())
            .wrapping_offset(((state) as i32) as isize))
            .read(),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SecretBaseSecrets_GetNumActionsTaken(show: *mut u8) -> u8 {
    unsafe {
        let mut show = show;
        let mut i: u8 = 0u8;
        let mut flagsSet: u8 = 0u8;
        {
            i = 0u8;
            flagsSet = 0u8;
            'l1: loop {
                if !(((i) as i32) < 32i32) {
                    break 'l1;
                }
                'l2: {
                    if (crate::c::shr_u32(
                        ((show).wrapping_add(12).cast::<u32>()).read(),
                        ((i) as u32),
                    ) & 1u32)
                        != 0
                    {
                        flagsSet = (flagsSet).wrapping_add(1);
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return flagsSet;
    }
}
pub(crate) unsafe extern "C" fn SecretBaseSecrets_GetStateByFlagNumber(
    show: *mut u8,
    flagId: u8,
) -> u8 {
    unsafe {
        let mut show = show;
        let mut flagId = flagId;
        let mut i: u8 = 0u8;
        let mut flagsSet: u8 = 0u8;
        {
            i = 0u8;
            flagsSet = 0u8;
            'l1: loop {
                if !(((i) as i32) < 32i32) {
                    break 'l1;
                }
                'l2: {
                    if (crate::c::shr_u32(
                        ((show).wrapping_add(12).cast::<u32>()).read(),
                        ((i) as u32),
                    ) & 1u32)
                        != 0
                    {
                        if ((flagsSet) as i32) == ((flagId) as i32) {
                            return ((((&raw const sTVSecretBaseSecretsActions)
                                .cast::<u8>()
                                .cast_mut())
                            .cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize))
                            .read();
                        }
                        flagsSet = (flagsSet).wrapping_add(1);
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn DoTVShowSecretBaseSecrets() {
    unsafe {
        let mut show: *mut u8 = core::ptr::null_mut();
        let mut state: u8 = 0u8;
        let mut numActions: u8 = 0u8;
        let mut i: u16 = 0u16;
        show = (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(10188))
            .cast::<u8>())
        .wrapping_offset(
            ((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32) as isize * 36,
        );
        ((&raw mut gSpecialVar_Result).cast::<u16>()).write(0u16);
        state = ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).read();
        'l1: {
            let __sw1 = ((state) as i32);
            if __sw1 == 0i32 {
                TVShowConvertInternationalString(
                    (&raw mut gStringVar1).cast::<u8>(),
                    ((show).wrapping_add(4)).cast::<u8>(),
                    ((((show).wrapping_add(28)).read()) as i32),
                );
                TVShowConvertInternationalString(
                    (&raw mut gStringVar2).cast::<u8>(),
                    ((show).wrapping_add(19)).cast::<u8>(),
                    ((((show).wrapping_add(27)).read()) as i32),
                );
                numActions = SecretBaseSecrets_GetNumActionsTaken(show);
                if ((numActions) as i32) == 0i32 {
                    ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(8u8);
                } else {
                    ((show).wrapping_add(18)).write(1u8);
                    (((&raw mut sTVSecretBaseSecretsRandomValues).cast::<u8>()).cast::<u8>())
                        .write(
                            ((crate::c::rem_i32(((Random()) as i32), ((numActions) as i32))) as u8),
                        );
                    ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(
                        SecretBaseSecrets_GetStateByFlagNumber(
                            show,
                            (((&raw mut sTVSecretBaseSecretsRandomValues).cast::<u8>())
                                .cast::<u8>())
                            .read(),
                        ),
                    );
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                TVShowConvertInternationalString(
                    (&raw mut gStringVar2).cast::<u8>(),
                    ((show).wrapping_add(19)).cast::<u8>(),
                    ((((show).wrapping_add(27)).read()) as i32),
                );
                numActions = SecretBaseSecrets_GetNumActionsTaken(show);
                'l2: {
                    let __sw2 = ((numActions) as i32);
                    let __matched = __sw2 == 1i32 || __sw2 == 2i32;
                    if __sw2 == 1i32 {
                        ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(9u8);
                        break 'l2;
                    }
                    if __sw2 == 2i32 {
                        ((show).wrapping_add(18)).write(2u8);
                        if (((((&raw mut sTVSecretBaseSecretsRandomValues).cast::<u8>())
                            .cast::<u8>())
                        .read()) as i32)
                            == 0i32
                        {
                            ((&raw mut sTVShowState).cast::<u8>().cast::<u8>())
                                .write(SecretBaseSecrets_GetStateByFlagNumber(show, 1u8));
                        } else {
                            ((&raw mut sTVShowState).cast::<u8>().cast::<u8>())
                                .write(SecretBaseSecrets_GetStateByFlagNumber(show, 0u8));
                        }
                        break 'l2;
                    }
                    if !__matched {
                        {
                            i = 0u16;
                            'l3: loop {
                                if !(((i) as i32) < 65535i32) {
                                    break 'l3;
                                }
                                'l4: {
                                    ((((&raw mut sTVSecretBaseSecretsRandomValues).cast::<u8>())
                                        .cast::<u8>())
                                    .wrapping_offset(1))
                                    .write(
                                        ((crate::c::rem_i32(
                                            ((Random()) as i32),
                                            ((numActions) as i32),
                                        )) as u8),
                                    );
                                    if ((((((&raw mut sTVSecretBaseSecretsRandomValues)
                                        .cast::<u8>())
                                    .cast::<u8>())
                                    .wrapping_offset(1))
                                    .read()) as i32)
                                        != (((((&raw mut sTVSecretBaseSecretsRandomValues)
                                            .cast::<u8>())
                                        .cast::<u8>())
                                        .read()) as i32)
                                    {
                                        break 'l3;
                                    }
                                }
                                i = (i).wrapping_add(1);
                            }
                        }
                        ((show).wrapping_add(18)).write(2u8);
                        ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(
                            SecretBaseSecrets_GetStateByFlagNumber(
                                show,
                                ((((&raw mut sTVSecretBaseSecretsRandomValues).cast::<u8>())
                                    .cast::<u8>())
                                .wrapping_offset(1))
                                .read(),
                            ),
                        );
                        break 'l2;
                    }
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                TVShowConvertInternationalString(
                    (&raw mut gStringVar2).cast::<u8>(),
                    ((show).wrapping_add(19)).cast::<u8>(),
                    ((((show).wrapping_add(27)).read()) as i32),
                );
                numActions = SecretBaseSecrets_GetNumActionsTaken(show);
                if ((numActions) as i32) == 2i32 {
                    ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(9u8);
                } else {
                    {
                        i = 0u16;
                        'l5: loop {
                            if !(((i) as i32) < 65535i32) {
                                break 'l5;
                            }
                            'l6: {
                                ((((&raw mut sTVSecretBaseSecretsRandomValues).cast::<u8>())
                                    .cast::<u8>())
                                .wrapping_offset(2))
                                .write(
                                    ((crate::c::rem_i32(((Random()) as i32), ((numActions) as i32)))
                                        as u8),
                                );
                                if (((((((&raw mut sTVSecretBaseSecretsRandomValues)
                                    .cast::<u8>())
                                .cast::<u8>())
                                .wrapping_offset(2))
                                .read()) as i32)
                                    != (((((&raw mut sTVSecretBaseSecretsRandomValues)
                                        .cast::<u8>())
                                    .cast::<u8>())
                                    .read()) as i32))
                                    && (((((((&raw mut sTVSecretBaseSecretsRandomValues)
                                        .cast::<u8>())
                                    .cast::<u8>())
                                    .wrapping_offset(2))
                                    .read()) as i32)
                                        != ((((((&raw mut sTVSecretBaseSecretsRandomValues)
                                            .cast::<u8>())
                                        .cast::<u8>())
                                        .wrapping_offset(1))
                                        .read())
                                            as i32))
                                {
                                    break 'l5;
                                }
                            }
                            i = (i).wrapping_add(1);
                        }
                    }
                    ((show).wrapping_add(18)).write(3u8);
                    ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(
                        SecretBaseSecrets_GetStateByFlagNumber(
                            show,
                            ((((&raw mut sTVSecretBaseSecretsRandomValues).cast::<u8>())
                                .cast::<u8>())
                            .wrapping_offset(2))
                            .read(),
                        ),
                    );
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                TVShowConvertInternationalString(
                    (&raw mut gStringVar1).cast::<u8>(),
                    ((show).wrapping_add(4)).cast::<u8>(),
                    ((((show).wrapping_add(28)).read()) as i32),
                );
                TVShowConvertInternationalString(
                    (&raw mut gStringVar2).cast::<u8>(),
                    ((show).wrapping_add(19)).cast::<u8>(),
                    ((((show).wrapping_add(27)).read()) as i32),
                );
                ConvertIntToDecimalString(
                    2u8,
                    ((((show).wrapping_add(2).cast::<u16>()).read()) as i32),
                );
                if ((((show).wrapping_add(2).cast::<u16>()).read()) as i32) <= 30i32 {
                    ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(4u8);
                } else {
                    if ((((show).wrapping_add(2).cast::<u16>()).read()) as i32) <= 100i32 {
                        ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(5u8);
                    } else {
                        ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(6u8);
                    }
                }
                break 'l1;
            }
            if (4i32..=6i32).contains(&__sw1) {
                TVShowConvertInternationalString(
                    (&raw mut gStringVar1).cast::<u8>(),
                    ((show).wrapping_add(4)).cast::<u8>(),
                    ((((show).wrapping_add(28)).read()) as i32),
                );
                TVShowConvertInternationalString(
                    (&raw mut gStringVar2).cast::<u8>(),
                    ((show).wrapping_add(19)).cast::<u8>(),
                    ((((show).wrapping_add(27)).read()) as i32),
                );
                ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(7u8);
                break 'l1;
            }
            if __sw1 == 7i32 {
                TVShowConvertInternationalString(
                    (&raw mut gStringVar1).cast::<u8>(),
                    ((show).wrapping_add(4)).cast::<u8>(),
                    ((((show).wrapping_add(28)).read()) as i32),
                );
                TVShowConvertInternationalString(
                    (&raw mut gStringVar2).cast::<u8>(),
                    ((show).wrapping_add(19)).cast::<u8>(),
                    ((((show).wrapping_add(27)).read()) as i32),
                );
                TVShowDone();
                break 'l1;
            }
            if __sw1 == 8i32 {
                ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(3u8);
                break 'l1;
            }
            if __sw1 == 9i32 {
                ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(3u8);
                break 'l1;
            }
            if (10i32..=18i32).contains(&__sw1) {
                ((&raw mut sTVShowState).cast::<u8>().cast::<u8>())
                    .write(((show).wrapping_add(18)).read());
                break 'l1;
            }
            if __sw1 == 19i32 {
                StringCopy(
                    (&raw mut gStringVar2).cast::<u8>(),
                    GetItemName(((show).wrapping_add(16).cast::<u16>()).read()),
                );
                ((&raw mut sTVShowState).cast::<u8>().cast::<u8>())
                    .write(((show).wrapping_add(18)).read());
                break 'l1;
            }
            if __sw1 == 20i32 {
                if (((((show).wrapping_add(34)).read()) as i32) & 1i32) != 0 {
                    ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(22u8);
                } else {
                    ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(21u8);
                }
                break 'l1;
            }
            if (21i32..=43i32).contains(&__sw1) {
                ((&raw mut sTVShowState).cast::<u8>().cast::<u8>())
                    .write(((show).wrapping_add(18)).read());
                break 'l1;
            }
        }
        ShowFieldMessage(
            ((((&raw const sTVSecretBaseSecretsTextGroup)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u8>())
            .cast::<*mut u8>())
            .wrapping_offset(((state) as i32) as isize))
            .read(),
        );
    }
}
pub(crate) unsafe extern "C" fn DoTVShowSafariFanClub() {
    unsafe {
        let mut show: *mut u8 = core::ptr::null_mut();
        let mut state: u8 = 0u8;
        show = (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(10188))
            .cast::<u8>())
        .wrapping_offset(
            ((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32) as isize * 36,
        );
        ((&raw mut gSpecialVar_Result).cast::<u16>()).write(0u16);
        state = ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).read();
        'l1: {
            let __sw1 = ((state) as i32);
            if __sw1 == 0i32 {
                if ((((show).wrapping_add(2)).read()) as i32) == 0i32 {
                    ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(6u8);
                } else {
                    if ((((show).wrapping_add(2)).read()) as i32) < 4i32 {
                        ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(5u8);
                    } else {
                        ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(1u8);
                    }
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                TVShowConvertInternationalString(
                    (&raw mut gStringVar1).cast::<u8>(),
                    ((show).wrapping_add(19)).cast::<u8>(),
                    ((((show).wrapping_add(4)).read()) as i32),
                );
                ConvertIntToDecimalString(1u8, ((((show).wrapping_add(2)).read()) as i32));
                if ((((show).wrapping_add(3)).read()) as i32) == 0i32 {
                    ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(3u8);
                } else {
                    ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(2u8);
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                ConvertIntToDecimalString(1u8, ((((show).wrapping_add(3)).read()) as i32));
                ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(4u8);
                break 'l1;
            }
            if __sw1 == 3i32 {
                ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(4u8);
                break 'l1;
            }
            if __sw1 == 4i32 {
                TVShowConvertInternationalString(
                    (&raw mut gStringVar1).cast::<u8>(),
                    ((show).wrapping_add(19)).cast::<u8>(),
                    ((((show).wrapping_add(4)).read()) as i32),
                );
                ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(10u8);
                break 'l1;
            }
            if __sw1 == 5i32 {
                TVShowConvertInternationalString(
                    (&raw mut gStringVar1).cast::<u8>(),
                    ((show).wrapping_add(19)).cast::<u8>(),
                    ((((show).wrapping_add(4)).read()) as i32),
                );
                ConvertIntToDecimalString(1u8, ((((show).wrapping_add(2)).read()) as i32));
                if ((((show).wrapping_add(3)).read()) as i32) == 0i32 {
                    ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(8u8);
                } else {
                    ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(7u8);
                }
                break 'l1;
            }
            if __sw1 == 6i32 {
                TVShowConvertInternationalString(
                    (&raw mut gStringVar1).cast::<u8>(),
                    ((show).wrapping_add(19)).cast::<u8>(),
                    ((((show).wrapping_add(4)).read()) as i32),
                );
                if ((((show).wrapping_add(3)).read()) as i32) == 0i32 {
                    ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(8u8);
                } else {
                    ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(7u8);
                }
                break 'l1;
            }
            if __sw1 == 7i32 {
                ConvertIntToDecimalString(1u8, ((((show).wrapping_add(3)).read()) as i32));
                ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(9u8);
                break 'l1;
            }
            if __sw1 == 8i32 {
                ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(9u8);
                break 'l1;
            }
            if __sw1 == 9i32 {
                TVShowConvertInternationalString(
                    (&raw mut gStringVar1).cast::<u8>(),
                    ((show).wrapping_add(19)).cast::<u8>(),
                    ((((show).wrapping_add(4)).read()) as i32),
                );
                ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(10u8);
                break 'l1;
            }
            if __sw1 == 10i32 {
                TVShowDone();
                break 'l1;
            }
        }
        ShowFieldMessage(
            ((((&raw const sTVSafariFanClubTextGroup)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u8>())
            .cast::<*mut u8>())
            .wrapping_offset(((state) as i32) as isize))
            .read(),
        );
    }
}
pub(crate) unsafe extern "C" fn DoTVShowLilycoveContestLady() {
    unsafe {
        let mut show: *mut u8 = core::ptr::null_mut();
        let mut state: u8 = 0u8;
        show = (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(10188))
            .cast::<u8>())
        .wrapping_offset(
            ((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32) as isize * 36,
        );
        ((&raw mut gSpecialVar_Result).cast::<u16>()).write(0u16);
        state = ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).read();
        'l1: {
            let __sw1 = ((state) as i32);
            let mut __fall = false;
            if __sw1 == 0i32 {
                __fall = true;
                BufferContestName(
                    (&raw mut gStringVar1).cast::<u8>(),
                    ((show).wrapping_add(10)).read(),
                );
                if ((((show).wrapping_add(22)).read()) as i32) == 1i32 {
                    ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(1u8);
                } else {
                    if ((((show).wrapping_add(22)).read()) as i32) == 0i32 {
                        ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(2u8);
                    } else {
                        ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(3u8);
                    }
                }
                break 'l1;
            }
            if __sw1 == 1i32 || __sw1 == 2i32 {
                __fall = true;
                TVShowConvertInternationalString(
                    (&raw mut gStringVar3).cast::<u8>(),
                    ((show).wrapping_add(2)).cast::<u8>(),
                    ((((show).wrapping_add(23)).read()) as i32),
                );
            }
            if __fall || __sw1 == 3i32 {
                __fall = true;
                TVShowConvertInternationalString(
                    (&raw mut gStringVar2).cast::<u8>(),
                    ((show).wrapping_add(11)).cast::<u8>(),
                    ((((show).wrapping_add(24)).read()) as i32),
                );
                TVShowDone();
                break 'l1;
            }
        }
        ShowFieldMessage(
            ((((&raw const sTVLilycoveContestLadyTextGroup)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u8>())
            .cast::<*mut u8>())
            .wrapping_offset(((state) as i32) as isize))
            .read(),
        );
    }
}
pub(crate) unsafe extern "C" fn TVShowDone() {
    unsafe {
        ((&raw mut gSpecialVar_Result).cast::<u16>()).write(1u16);
        ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(0u8);
        (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(10188))
            .cast::<u8>())
        .wrapping_offset(
            ((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32) as isize * 36,
        ))
        .wrapping_add(1))
        .write(0u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ResetTVShowState() {
    unsafe {
        ((&raw mut sTVShowState).cast::<u8>().cast::<u8>()).write(0u8);
    }
}
