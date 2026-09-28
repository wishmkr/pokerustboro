//! Translated from `src/tv.c` by tools/rustport/c2rs.py.
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
// Data tables (translate with cdata.py): sPokeOutbreakSpeciesList sGoldSymbolFlags sSilverSymbolFlags sNumberOneVarsAndThresholds sPokeNewsTextGroup_Upcoming sPokeNewsTextGroup_Ongoing sPokeNewsTextGroup_Ending gTVStringVarPtrs sTVFanClubTextGroup sTVRecentHappeninssTextGroup sTVFanClubOpinionsTextGroup sTVMassOutbreakTextGroup sTVPokemonTodaySuccessfulTextGroup sTVTodaysSmartShopperTextGroup sTVBravoTrainerTextGroup sTV3CheersForPokeblocksTextGroup sTVBravoTrainerBattleTowerTextGroup sTVContestLiveUpdatesTextGroup sTVPokemonBattleUpdateTextGroup sTVTrainerFanClubSpecialTextGroup sTVNameRaterTextGroup sTVLilycoveContestLadyTextGroup sTVPokemonTodayFailedTextGroup sTVPokemonAnglerTextGroup sTVWorldOfMastersTextGroup sTVTodaysRivalTrainerTextGroup sTVDewfordTrendWatcherNetworkTextGroup sTVHoennTreasureInvestisatorsTextGroup sTVFindThatGamerTextGroup sTVBreakingNewsTextGroup sTVSecretBaseVisitTextGroup sTVPokemonLotteryWinnerFlashReportTextGroup sTVThePokemonBattleSeminarTextGroup sTVTrainerFanClubTextGroup sTVCutiesTextGroup sTVPokemonNewsBattleFrontierTextGroup sTVWhatsNo1InHoennTodayTextGroup sTVSecretBaseSecretsTextGroup sTVSafariFanClubTextGroup sTVInSearchOfTrainersTextGroup sTVSecretBaseSecretsActions

/// `__typeof__(sTV_SecretBaseVisitMonsTemp[0])`
#[repr(C, align(4))]
#[derive(Clone, Copy)]
pub struct sTV_SecretBaseVisitMonsTemp_0_t {
    pub level: u8,
    pub species: u16,
    pub r#move: u16,
}

unsafe impl Sync for sTV_SecretBaseVisitMonsTemp_0_t {}

/// `__typeof__(sPokeOutbreakSpeciesList[0])`
#[repr(C, align(4))]
#[derive(Clone, Copy)]
pub struct sPokeOutbreakSpeciesList_0_t {
    pub species: u16,
    pub moves: CArray<u16, 4>,
    pub level: u8,
    pub location: u8,
}

unsafe impl Sync for sPokeOutbreakSpeciesList_0_t {}

#[cfg(target_arch = "arm")]
const _: () = {
    #[allow(unused_imports)]
    use core::mem::{offset_of, size_of};
    assert!(size_of::<sTV_SecretBaseVisitMonsTemp_0_t>() == 8);
    assert!(offset_of!(sTV_SecretBaseVisitMonsTemp_0_t, level) == 0);
    assert!(offset_of!(sTV_SecretBaseVisitMonsTemp_0_t, species) == 2);
    assert!(offset_of!(sTV_SecretBaseVisitMonsTemp_0_t, r#move) == 4);
    assert!(size_of::<sPokeOutbreakSpeciesList_0_t>() == 12);
    assert!(offset_of!(sPokeOutbreakSpeciesList_0_t, species) == 0);
    assert!(offset_of!(sPokeOutbreakSpeciesList_0_t, moves) == 2);
    assert!(offset_of!(sPokeOutbreakSpeciesList_0_t, level) == 10);
    assert!(offset_of!(sPokeOutbreakSpeciesList_0_t, location) == 11);
};

const LAST_TVSHOW_IDX: u8 = 24;
const ROULETTE: u8 = 1;
const SLOT_MACHINE: u8 = 0;
const TVGROUP_NONE: u8 = 0;
const TVGROUP_NORMAL: u8 = 2;
const TVGROUP_OUTBREAK: u8 = 4;
const TVGROUP_RECORD_MIX: u8 = 3;

static gTVStringVarPtrs: Table<CArray<*mut u8, 3>> =
    Table((&raw const crate::data::tv::gTVStringVarPtrs).cast());
static sGoldSymbolFlags: Table<CArray<u16, 7>> =
    Table((&raw const crate::data::tv::sGoldSymbolFlags).cast());
static sNumberOneVarsAndThresholds: Table<CArray<CArray<u16, 2>, 7>> =
    Table((&raw const crate::data::tv::sNumberOneVarsAndThresholds).cast());
static sPokeNewsTextGroup_Ending: Table<CArray<*mut u8, 5>> =
    Table((&raw const crate::data::tv::sPokeNewsTextGroup_Ending).cast());
static sPokeNewsTextGroup_Ongoing: Table<CArray<*mut u8, 5>> =
    Table((&raw const crate::data::tv::sPokeNewsTextGroup_Ongoing).cast());
static sPokeNewsTextGroup_Upcoming: Table<CArray<*mut u8, 5>> =
    Table((&raw const crate::data::tv::sPokeNewsTextGroup_Upcoming).cast());
static sPokeOutbreakSpeciesList: Table<CArray<sPokeOutbreakSpeciesList_0_t, 5>> =
    Table((&raw const crate::data::tv::sPokeOutbreakSpeciesList).cast());
static sSilverSymbolFlags: Table<CArray<u16, 7>> =
    Table((&raw const crate::data::tv::sSilverSymbolFlags).cast());
static sTV3CheersForPokeblocksTextGroup: Table<CArray<*mut u8, 6>> =
    Table((&raw const crate::data::tv::sTV3CheersForPokeblocksTextGroup).cast());
static sTVBravoTrainerBattleTowerTextGroup: Table<CArray<*mut u8, 15>> =
    Table((&raw const crate::data::tv::sTVBravoTrainerBattleTowerTextGroup).cast());
static sTVBravoTrainerTextGroup: Table<CArray<*mut u8, 9>> =
    Table((&raw const crate::data::tv::sTVBravoTrainerTextGroup).cast());
static sTVBreakingNewsTextGroup: Table<CArray<*mut u8, 13>> =
    Table((&raw const crate::data::tv::sTVBreakingNewsTextGroup).cast());
static sTVContestLiveUpdatesTextGroup: Table<CArray<*mut u8, 33>> =
    Table((&raw const crate::data::tv::sTVContestLiveUpdatesTextGroup).cast());
static sTVCutiesTextGroup: Table<CArray<*mut u8, 16>> =
    Table((&raw const crate::data::tv::sTVCutiesTextGroup).cast());
static sTVDewfordTrendWatcherNetworkTextGroup: Table<CArray<*mut u8, 7>> =
    Table((&raw const crate::data::tv::sTVDewfordTrendWatcherNetworkTextGroup).cast());
static sTVFanClubOpinionsTextGroup: Table<CArray<*mut u8, 5>> =
    Table((&raw const crate::data::tv::sTVFanClubOpinionsTextGroup).cast());
static sTVFanClubTextGroup: Table<CArray<*mut u8, 8>> =
    Table((&raw const crate::data::tv::sTVFanClubTextGroup).cast());
static sTVFindThatGamerTextGroup: Table<CArray<*mut u8, 4>> =
    Table((&raw const crate::data::tv::sTVFindThatGamerTextGroup).cast());
static sTVHoennTreasureInvestisatorsTextGroup: Table<CArray<*mut u8, 3>> =
    Table((&raw const crate::data::tv::sTVHoennTreasureInvestisatorsTextGroup).cast());
static sTVInSearchOfTrainersTextGroup: Table<CArray<*mut u8, 9>> =
    Table((&raw const crate::data::tv::sTVInSearchOfTrainersTextGroup).cast());
static sTVLilycoveContestLadyTextGroup: Table<CArray<*mut u8, 4>> =
    Table((&raw const crate::data::tv::sTVLilycoveContestLadyTextGroup).cast());
static sTVMassOutbreakTextGroup: Table<CArray<*mut u8, 1>> =
    Table((&raw const crate::data::tv::sTVMassOutbreakTextGroup).cast());
static sTVNameRaterTextGroup: Table<CArray<*mut u8, 19>> =
    Table((&raw const crate::data::tv::sTVNameRaterTextGroup).cast());
static sTVPokemonAnglerTextGroup: Table<CArray<*mut u8, 2>> =
    Table((&raw const crate::data::tv::sTVPokemonAnglerTextGroup).cast());
static sTVPokemonBattleUpdateTextGroup: Table<CArray<*mut u8, 8>> =
    Table((&raw const crate::data::tv::sTVPokemonBattleUpdateTextGroup).cast());
static sTVPokemonLotteryWinnerFlashReportTextGroup: Table<CArray<*mut u8, 1>> =
    Table((&raw const crate::data::tv::sTVPokemonLotteryWinnerFlashReportTextGroup).cast());
static sTVPokemonNewsBattleFrontierTextGroup: Table<CArray<*mut u8, 19>> =
    Table((&raw const crate::data::tv::sTVPokemonNewsBattleFrontierTextGroup).cast());
static sTVPokemonTodayFailedTextGroup: Table<CArray<*mut u8, 7>> =
    Table((&raw const crate::data::tv::sTVPokemonTodayFailedTextGroup).cast());
static sTVPokemonTodaySuccessfulTextGroup: Table<CArray<*mut u8, 12>> =
    Table((&raw const crate::data::tv::sTVPokemonTodaySuccessfulTextGroup).cast());
static sTVRecentHappeninssTextGroup: Table<CArray<*mut u8, 6>> =
    Table((&raw const crate::data::tv::sTVRecentHappeninssTextGroup).cast());
static sTVSafariFanClubTextGroup: Table<CArray<*mut u8, 11>> =
    Table((&raw const crate::data::tv::sTVSafariFanClubTextGroup).cast());
static sTVSecretBaseSecretsActions: Table<CArray<u8, 32>> =
    Table((&raw const crate::data::tv::sTVSecretBaseSecretsActions).cast());
static sTVSecretBaseSecretsTextGroup: Table<CArray<*mut u8, 43>> =
    Table((&raw const crate::data::tv::sTVSecretBaseSecretsTextGroup).cast());
static sTVSecretBaseVisitTextGroup: Table<CArray<*mut u8, 14>> =
    Table((&raw const crate::data::tv::sTVSecretBaseVisitTextGroup).cast());
static sTVThePokemonBattleSeminarTextGroup: Table<CArray<*mut u8, 7>> =
    Table((&raw const crate::data::tv::sTVThePokemonBattleSeminarTextGroup).cast());
static sTVTodaysRivalTrainerTextGroup: Table<CArray<*mut u8, 11>> =
    Table((&raw const crate::data::tv::sTVTodaysRivalTrainerTextGroup).cast());
static sTVTodaysSmartShopperTextGroup: Table<CArray<*mut u8, 13>> =
    Table((&raw const crate::data::tv::sTVTodaysSmartShopperTextGroup).cast());
static sTVTrainerFanClubSpecialTextGroup: Table<CArray<*mut u8, 6>> =
    Table((&raw const crate::data::tv::sTVTrainerFanClubSpecialTextGroup).cast());
static sTVTrainerFanClubTextGroup: Table<CArray<*mut u8, 12>> =
    Table((&raw const crate::data::tv::sTVTrainerFanClubTextGroup).cast());
static sTVWhatsNo1InHoennTodayTextGroup: Table<CArray<*mut u8, 9>> =
    Table((&raw const crate::data::tv::sTVWhatsNo1InHoennTodayTextGroup).cast());
static sTVWorldOfMastersTextGroup: Table<CArray<*mut u8, 3>> =
    Table((&raw const crate::data::tv::sTVWorldOfMastersTextGroup).cast());

#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut sCurTVShowSlot: i8 = 0;
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut sTV_SecretBaseVisitMovesTemp: Aligned<CArray<u16, 8>> = Aligned(unsafe { zeroed() });
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut sTV_DecorationsBuffer: Aligned<CArray<u8, 16>> = Aligned(unsafe { zeroed() });
#[unsafe(no_mangle)]
pub static mut sTV_SecretBaseVisitMonsTemp: CArray<sTV_SecretBaseVisitMonsTemp_0_t, 10> =
    unsafe { zeroed() };
pub(crate) static mut sTVShowMixingNumPlayers: u8 = 0;
pub(crate) static mut sTVShowNewsMixingNumPlayers: u8 = 0;
pub(crate) static mut sTVShowMixingCurSlot: i8 = 0;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sPokemonAnglerSpecies: u16 = 0;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sPokemonAnglerAttemptCounters: u16 = 0;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sFindThatGamerCoinsSpent: u16 = 0;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sFindThatGamerWhichGame: u8 = 0;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sRecordMixingPartnersWithoutShowsToShare: u8 = 0;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sTVShowState: u8 = 0;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sTVSecretBaseSecretsRandomValues: Aligned<CArray<u8, 3>> =
    Aligned(unsafe { zeroed() });

unsafe extern "C" {
    static mut gBackupMapLayout: BackupMapLayout;
    static mut gBattleOutcome: u8;
    static mut gBattleResults: BattleResults;
    static mut gBattleTypeFlags: u32;
    static mut gContestMonPartyIndex: u8;
    static mut gContestMons: CArray<ContestPokemon, 4>;
    static gDecorations: CArray<Decoration, 0>;
    static gGameLanguage: u8;
    static mut gLastUsedItem: u16;
    static mut gLinkPlayers: CArray<LinkPlayer, 5>;
    static mut gLocalTime: Time;
    static mut gMapHeader: MapHeader;
    static mut gMartPurchaseHistory: CArray<ItemSlot, 3>;
    static gMoveNames: CArray<CArray<u8, 13>, 355>;
    static mut gNumLinkContestPlayers: u8;
    static mut gPlayerParty: CArray<Pokemon, 6>;
    static mut gSaveBlock1Ptr: *mut SaveBlock1;
    static mut gSaveBlock2Ptr: *mut SaveBlock2;
    static mut gSpecialVar_0x8004: u16;
    static mut gSpecialVar_0x8005: u16;
    static mut gSpecialVar_0x8006: u16;
    static mut gSpecialVar_0x8007: u16;
    static mut gSpecialVar_ContestCategory: u16;
    static mut gSpecialVar_ContestRank: u16;
    static mut gSpecialVar_LastTalked: u16;
    static mut gSpecialVar_MonBoxId: u16;
    static mut gSpecialVar_MonBoxPos: u16;
    static mut gSpecialVar_Result: u16;
    static gSpeciesNames: CArray<CArray<u8, 11>, 0>;
    static gStdStrings: CArray<*mut u8, 0>;
    static mut gStringVar1: CArray<u8, 256>;
    static mut gStringVar2: CArray<u8, 256>;
    static mut gStringVar3: CArray<u8, 256>;
    static mut gStringVar4: CArray<u8, 1000>;
    static gText_Bad: CArray<u8, 0>;
    static gText_Beauty: CArray<u8, 0>;
    static gText_Bitter2: CArray<u8, 0>;
    static gText_Cool: CArray<u8, 0>;
    static gText_Cute: CArray<u8, 0>;
    static gText_Dad: CArray<u8, 0>;
    static gText_Double: CArray<u8, 0>;
    static gText_Dry2: CArray<u8, 0>;
    static gText_Excellent: CArray<u8, 0>;
    static gText_First: CArray<u8, 0>;
    static gText_Good: CArray<u8, 0>;
    static gText_Jackpot: CArray<u8, 0>;
    static gText_Lv50: CArray<u8, 0>;
    static gText_Mom: CArray<u8, 0>;
    static gText_OpenLevel: CArray<u8, 0>;
    static gText_Roulette: CArray<u8, 0>;
    static gText_Second: CArray<u8, 0>;
    static gText_Single: CArray<u8, 0>;
    static gText_Slots: CArray<u8, 0>;
    static gText_Smart: CArray<u8, 0>;
    static gText_SoSo: CArray<u8, 0>;
    static gText_Sour2: CArray<u8, 0>;
    static gText_Spicy2: CArray<u8, 0>;
    static gText_Sweet2: CArray<u8, 0>;
    static gText_TheWorst: CArray<u8, 0>;
    static gText_Third: CArray<u8, 0>;
    static gText_Tough: CArray<u8, 0>;
    static gText_VeryGood: CArray<u8, 0>;
    fn Alloc(a0: u32) -> *mut c_void;
    fn AllocZeroed(a0: u32) -> *mut c_void;
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
    fn Free(a0: *mut c_void);
    fn GetBoxMonData3(a0: *mut BoxPokemon, a1: i32, a2: *mut u8) -> u32;
    fn GetBoxMonGender(a0: *mut BoxPokemon) -> u8;
    fn GetBoxedMonPtr(a0: u8, a1: u8) -> *mut BoxPokemon;
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
    fn GetMonData2(a0: *mut Pokemon, a1: i32) -> u32;
    fn GetMonData3(a0: *mut Pokemon, a1: i32, a2: *mut u8) -> u32;
    fn GetMonGender(a0: *mut Pokemon) -> u8;
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
    fn SetMonData(a0: *mut Pokemon, a1: i32, a2: *mut c_void);
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
    let mut i: u8 = 0;
    let mut j: u8 = 0;
    i = 0;
    while i < 25 {
        (*gSaveBlock1Ptr).tvShows[i].commonInit.kind = 0;
        (*gSaveBlock1Ptr).tvShows[i].commonInit.active = 0;
        j = 0;
        while j < 34 {
            (*gSaveBlock1Ptr).tvShows[i].commonInit.data[j] = 0;
            j += 1;
        }
        i += 1;
    }
    ClearPokeNews();
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetRandomActiveShowIdx() -> u8 {
    let mut i: u8 = 0;
    let mut j: u8 = 0;
    let mut selIdx: u8 = 0;
    let mut show: *mut TVShow = null_mut();
    i = NUM_NORMAL_TVSHOW_SLOTS;
    while i < LAST_TVSHOW_IDX {
        if (*gSaveBlock1Ptr).tvShows[i].common.kind == TVSHOW_OFF_AIR {
            break;
        }
        i += 1;
    }
    j = rem_i32(Random() as i32, i as i32) as u8;
    selIdx = j;
    loop {
        if GetTVGroupByShowId((*gSaveBlock1Ptr).tvShows[j].common.kind) != TVGROUP_OUTBREAK {
            if (*gSaveBlock1Ptr).tvShows[j].common.active == TRUE {
                return j;
            }
        } else {
            show = &raw mut (*gSaveBlock1Ptr).tvShows[j];
            if (*show).massOutbreak.daysBeforeOutbreak == 0 && (*show).massOutbreak.active == TRUE {
                return j;
            }
        }
        if j == 0 {
            j = 23;
        } else {
            j -= 1;
        }
        if j == selIdx {
            break;
        }
    }
    return 0xFF;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FindAnyTVShowOnTheAir() -> u8 {
    let mut slot: u8 = GetRandomActiveShowIdx();
    if slot == 0xFF {
        return 0xFF;
    }
    if (*gSaveBlock1Ptr).outbreakPokemonSpecies != SPECIES_NONE
        && (*gSaveBlock1Ptr).tvShows[slot].common.kind == TVSHOW_MASS_OUTBREAK
    {
        return FindFirstActiveTVShowThatIsNotAMassOutbreak();
    }
    return slot;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn UpdateTVScreensOnMap(width: i32, height: i32) {
    FlagSet(FLAG_SYS_TV_WATCH);
    match CheckForPlayersHouseNews() {
        PLAYERS_HOUSE_TV_LATI => {
            SetTVMetatilesOnMap(width, height, METATILE_Building_TV_On);
        }
        PLAYERS_HOUSE_TV_MOVIE => {}
        _ => {
            if (*gSaveBlock1Ptr).location.mapGroup == 13 && (*gSaveBlock1Ptr).location.mapNum == 0 {
                SetTVMetatilesOnMap(width, height, METATILE_Building_TV_On);
            } else if FlagGet(FLAG_SYS_TV_START) != 0
                && (FindAnyTVShowOnTheAir() != 0xFF
                    || FindAnyPokeNewsOnTheAir() != 0xFF
                    || IsGabbyAndTyShowOnTheAir() != 0)
            {
                FlagClear(FLAG_SYS_TV_WATCH);
                SetTVMetatilesOnMap(width, height, METATILE_Building_TV_On);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SetTVMetatilesOnMap(width: i32, height: i32, metatileId: u16) {
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    y = 0;
    while y < height {
        x = 0;
        while x < width {
            if MapGridGetMetatileBehaviorAt(x, y) == MB_TELEVISION as i32 {
                MapGridSetMetatileIdAt(x, y, metatileId | MAPGRID_COLLISION_MASK);
            }
            x += 1;
        }
        y += 1;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn TurnOffTVScreen() {
    SetTVMetatilesOnMap(
        gBackupMapLayout.width,
        gBackupMapLayout.height,
        METATILE_Building_TV_Off,
    );
    DrawWholeMapView();
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn TurnOnTVScreen() {
    SetTVMetatilesOnMap(
        gBackupMapLayout.width,
        gBackupMapLayout.height,
        METATILE_Building_TV_On,
    );
    DrawWholeMapView();
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetSelectedTVShow() -> u8 {
    return (*gSaveBlock1Ptr).tvShows[gSpecialVar_0x8004].common.kind;
}
pub(crate) unsafe extern "C" fn FindFirstActiveTVShowThatIsNotAMassOutbreak() -> u8 {
    let mut i: u8 = 0;
    i = 0;
    while i < 24 {
        if (*gSaveBlock1Ptr).tvShows[i].common.kind != TVSHOW_OFF_AIR
            && (*gSaveBlock1Ptr).tvShows[i].common.kind != TVSHOW_MASS_OUTBREAK
            && (*gSaveBlock1Ptr).tvShows[i].common.active == TRUE
        {
            return i;
        }
        i += 1;
    }
    return 0xFF;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetNextActiveShowIfMassOutbreak() -> u8 {
    let mut tvShow: *mut TVShow = null_mut();
    tvShow = &raw mut (*gSaveBlock1Ptr).tvShows[gSpecialVar_0x8004];
    if (*tvShow).common.kind == TVSHOW_MASS_OUTBREAK
        && (*gSaveBlock1Ptr).outbreakPokemonSpecies != SPECIES_NONE
    {
        return FindFirstActiveTVShowThatIsNotAMassOutbreak();
    }
    return gSpecialVar_0x8004 as u8;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ResetGabbyAndTy() {
    (*gSaveBlock1Ptr).gabbyAndTyData.mon1 = SPECIES_NONE;
    (*gSaveBlock1Ptr).gabbyAndTyData.mon2 = SPECIES_NONE;
    (*gSaveBlock1Ptr).gabbyAndTyData.lastMove = MOVE_NONE;
    (*gSaveBlock1Ptr).gabbyAndTyData.quote[0] = 65535;
    (*gSaveBlock1Ptr)
        .gabbyAndTyData
        .set_battleTookMoreThanOneTurn(FALSE);
    (*gSaveBlock1Ptr).gabbyAndTyData.set_playerLostAMon(FALSE);
    (*gSaveBlock1Ptr)
        .gabbyAndTyData
        .set_playerUsedHealingItem(FALSE);
    (*gSaveBlock1Ptr).gabbyAndTyData.set_playerThrewABall(FALSE);
    (*gSaveBlock1Ptr).gabbyAndTyData.set_onAir(FALSE);
    (*gSaveBlock1Ptr).gabbyAndTyData.set_valA_5(0);
    (*gSaveBlock1Ptr)
        .gabbyAndTyData
        .set_battleTookMoreThanOneTurn2(FALSE);
    (*gSaveBlock1Ptr).gabbyAndTyData.set_playerLostAMon2(FALSE);
    (*gSaveBlock1Ptr)
        .gabbyAndTyData
        .set_playerUsedHealingItem2(FALSE);
    (*gSaveBlock1Ptr)
        .gabbyAndTyData
        .set_playerThrewABall2(FALSE);
    (*gSaveBlock1Ptr).gabbyAndTyData.set_valB_4(0);
    (*gSaveBlock1Ptr).gabbyAndTyData.mapnum = 0;
    (*gSaveBlock1Ptr).gabbyAndTyData.battleNum = 0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GabbyAndTyBeforeInterview() {
    let mut i: u8 = 0;
    (*gSaveBlock1Ptr).gabbyAndTyData.mon1 = gBattleResults.playerMon1Species;
    (*gSaveBlock1Ptr).gabbyAndTyData.mon2 = gBattleResults.playerMon2Species;
    (*gSaveBlock1Ptr).gabbyAndTyData.lastMove = gBattleResults.lastUsedMovePlayer;
    if (*gSaveBlock1Ptr).gabbyAndTyData.battleNum != 0xFF {
        (*gSaveBlock1Ptr).gabbyAndTyData.battleNum += 1;
    }
    (*gSaveBlock1Ptr)
        .gabbyAndTyData
        .set_battleTookMoreThanOneTurn(gBattleResults.playerMonWasDamaged());
    if gBattleResults.playerFaintCounter != 0 {
        (*gSaveBlock1Ptr).gabbyAndTyData.set_playerLostAMon(TRUE);
    } else {
        (*gSaveBlock1Ptr).gabbyAndTyData.set_playerLostAMon(FALSE);
    }
    if gBattleResults.numHealingItemsUsed != 0 {
        (*gSaveBlock1Ptr)
            .gabbyAndTyData
            .set_playerUsedHealingItem(TRUE);
    } else {
        (*gSaveBlock1Ptr)
            .gabbyAndTyData
            .set_playerUsedHealingItem(FALSE);
    }
    if gBattleResults.usedMasterBall() == 0 {
        i = 0;
        while i < 11 {
            if gBattleResults.catchAttempts[i] != 0 {
                (*gSaveBlock1Ptr).gabbyAndTyData.set_playerThrewABall(TRUE);
                break;
            }
            i += 1;
        }
    } else {
        (*gSaveBlock1Ptr).gabbyAndTyData.set_playerThrewABall(TRUE);
    }
    TakeGabbyAndTyOffTheAir();
    if (*gSaveBlock1Ptr).gabbyAndTyData.lastMove == MOVE_NONE {
        FlagSet(FLAG_TEMP_SKIP_GABBY_INTERVIEW);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GabbyAndTyAfterInterview() {
    (*gSaveBlock1Ptr)
        .gabbyAndTyData
        .set_battleTookMoreThanOneTurn2(
            (*gSaveBlock1Ptr).gabbyAndTyData.battleTookMoreThanOneTurn(),
        );
    (*gSaveBlock1Ptr)
        .gabbyAndTyData
        .set_playerLostAMon2((*gSaveBlock1Ptr).gabbyAndTyData.playerLostAMon());
    (*gSaveBlock1Ptr)
        .gabbyAndTyData
        .set_playerUsedHealingItem2((*gSaveBlock1Ptr).gabbyAndTyData.playerUsedHealingItem());
    (*gSaveBlock1Ptr)
        .gabbyAndTyData
        .set_playerThrewABall2((*gSaveBlock1Ptr).gabbyAndTyData.playerThrewABall());
    (*gSaveBlock1Ptr).gabbyAndTyData.set_onAir(TRUE);
    (*gSaveBlock1Ptr).gabbyAndTyData.mapnum = gMapHeader.regionMapSectionId;
    IncrementGameStat(GAME_STAT_GOT_INTERVIEWED);
}
pub(crate) unsafe extern "C" fn TakeGabbyAndTyOffTheAir() {
    (*gSaveBlock1Ptr).gabbyAndTyData.set_onAir(FALSE);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GabbyAndTyGetBattleNum() -> u8 {
    if (*gSaveBlock1Ptr).gabbyAndTyData.battleNum > 5 {
        return ((*gSaveBlock1Ptr).gabbyAndTyData.battleNum as i32 % 3) as u8 + 6;
    }
    return (*gSaveBlock1Ptr).gabbyAndTyData.battleNum;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsGabbyAndTyShowOnTheAir() -> u8 {
    return (*gSaveBlock1Ptr).gabbyAndTyData.onAir();
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GabbyAndTyGetLastQuote() -> u8 {
    if (*gSaveBlock1Ptr).gabbyAndTyData.quote[0] == EC_EMPTY_WORD {
        return FALSE;
    }
    CopyEasyChatWord(
        gStringVar1.as_mut_ptr(),
        (*gSaveBlock1Ptr).gabbyAndTyData.quote[0],
    );
    (*gSaveBlock1Ptr).gabbyAndTyData.quote[0] = 65535;
    return TRUE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GabbyAndTyGetLastBattleTrivia() -> u8 {
    if (*gSaveBlock1Ptr)
        .gabbyAndTyData
        .battleTookMoreThanOneTurn2()
        == 0
    {
        return 1;
    }
    if (*gSaveBlock1Ptr).gabbyAndTyData.playerThrewABall2() != 0 {
        return 2;
    }
    if (*gSaveBlock1Ptr).gabbyAndTyData.playerUsedHealingItem2() != 0 {
        return 3;
    }
    if (*gSaveBlock1Ptr).gabbyAndTyData.playerLostAMon2() != 0 {
        return 4;
    }
    return 0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetGabbyAndTyLocalIds() {
    match GabbyAndTyGetBattleNum() {
        1 => {
            gSpecialVar_0x8004 = LOCALID_ROUTE111_GABBY_1;
            gSpecialVar_0x8005 = LOCALID_ROUTE111_TY_1;
        }
        2 => {
            gSpecialVar_0x8004 = LOCALID_ROUTE118_GABBY_1;
            gSpecialVar_0x8005 = LOCALID_ROUTE118_TY_1;
        }
        3 => {
            gSpecialVar_0x8004 = LOCALID_ROUTE120_GABBY_1;
            gSpecialVar_0x8005 = LOCALID_ROUTE120_TY_1;
        }
        4 => {
            gSpecialVar_0x8004 = LOCALID_ROUTE111_GABBY_2;
            gSpecialVar_0x8005 = LOCALID_ROUTE111_TY_2;
        }
        5 => {
            gSpecialVar_0x8004 = LOCALID_ROUTE118_GABBY_2;
            gSpecialVar_0x8005 = LOCALID_ROUTE118_TY_2;
        }
        6 => {
            gSpecialVar_0x8004 = LOCALID_ROUTE120_GABBY_2;
            gSpecialVar_0x8005 = LOCALID_ROUTE120_TY_2;
        }
        7 => {
            gSpecialVar_0x8004 = LOCALID_ROUTE111_GABBY_3;
            gSpecialVar_0x8005 = LOCALID_ROUTE111_TY_3;
        }
        8 => {
            gSpecialVar_0x8004 = LOCALID_ROUTE118_GABBY_3;
            gSpecialVar_0x8005 = LOCALID_ROUTE118_TY_3;
        }
        _ => {}
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn InterviewAfter() {
    match gSpecialVar_0x8005 {
        1 => {
            InterviewAfter_FanClubLetter();
        }
        2 => {
            InterviewAfter_RecentHappenings();
        }
        3 => {
            InterviewAfter_PkmnFanClubOpinions();
        }
        4 => {
            InterviewAfter_Dummy();
        }
        6 => {
            InterviewAfter_BravoTrainerPokemonProfile();
        }
        7 => {
            InterviewAfter_BravoTrainerBattleTowerProfile();
        }
        8 => {
            InterviewAfter_ContestLiveUpdates();
        }
        _ => {}
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn TryPutPokemonTodayOnAir() {
    let mut i: u8 = 0;
    let mut ballsUsed: u16 = 0;
    let mut show: *mut TVShow = null_mut();
    let mut language2: u32 = 0;
    let mut itemLastUsed: u16 = 0;
    ballsUsed = 0;
    TryPutRandomPokeNewsOnAir();
    TryStartRandomMassOutbreak();
    if gBattleResults.caughtMonSpecies == SPECIES_NONE {
        TryPutPokemonTodayFailedOnTheAir();
    } else {
        InitWorldOfMastersShowAttempt();
        if BernoulliTrial(65535) == 0
            && StringCompare(
                gSpeciesNames[gBattleResults.caughtMonSpecies]
                    .as_ptr()
                    .cast_mut(),
                gBattleResults.caughtMonNick.as_mut_ptr(),
            ) != 0
        {
            sCurTVShowSlot =
                FindFirstEmptyRecordMixTVShowSlot((*gSaveBlock1Ptr).tvShows.as_mut_ptr());
            if sCurTVShowSlot != -1
                && IsRecordMixShowAlreadySpawned(TVSHOW_POKEMON_TODAY_CAUGHT, FALSE) != 1
            {
                i = 0;
                while i < 11 {
                    ballsUsed += gBattleResults.catchAttempts[i] as u16;
                    i += 1;
                }
                if ballsUsed != 0 || gBattleResults.usedMasterBall() != 0 {
                    ballsUsed = 0;
                    show = &raw mut (*gSaveBlock1Ptr).tvShows[sCurTVShowSlot];
                    (*show).pokemonToday.kind = TVSHOW_POKEMON_TODAY_CAUGHT;
                    (*show).pokemonToday.active = FALSE;
                    if gBattleResults.usedMasterBall() != 0 {
                        ballsUsed = 1;
                        itemLastUsed = ITEM_MASTER_BALL;
                    } else {
                        i = 0;
                        while i < 11 {
                            ballsUsed += gBattleResults.catchAttempts[i] as u16;
                            i += 1;
                        }
                        if ballsUsed > 255 {
                            ballsUsed = 255;
                        }
                        itemLastUsed = gLastUsedItem;
                    }
                    (*show).pokemonToday.nBallsUsed = ballsUsed as u8;
                    (*show).pokemonToday.ball = itemLastUsed as u8;
                    StringCopy(
                        (*show).pokemonToday.playerName.as_mut_ptr(),
                        (*gSaveBlock2Ptr).playerName.as_mut_ptr(),
                    );
                    StringCopy(
                        (*show).pokemonToday.nickname.as_mut_ptr(),
                        gBattleResults.caughtMonNick.as_mut_ptr(),
                    );
                    language2 =
                        GetNicknameLanguage((*show).pokemonToday.nickname.as_mut_ptr()) as u32;
                    StripExtCtrlCodes((*show).pokemonToday.nickname.as_mut_ptr());
                    (*show).pokemonToday.species = gBattleResults.caughtMonSpecies;
                    StorePlayerIdInRecordMixShow(show);
                    (*show).pokemonToday.language = gGameLanguage;
                    (*show).pokemonToday.language2 = language2 as u8;
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn InitWorldOfMastersShowAttempt() {
    let mut show: *mut TVShow = &raw mut (*gSaveBlock1Ptr).tvShows[24];
    if (*show).common.kind != TVSHOW_WORLD_OF_MASTERS {
        DeleteTVShowInArrayByIdx((*gSaveBlock1Ptr).tvShows.as_mut_ptr(), LAST_TVSHOW_IDX);
        (*show).worldOfMasters.steps = GetGameStat(GAME_STAT_STEPS) as u16;
        (*show).worldOfMasters.kind = TVSHOW_WORLD_OF_MASTERS;
    }
    (*show).worldOfMasters.numPokeCaught += 1;
    (*show).worldOfMasters.caughtPoke = gBattleResults.caughtMonSpecies;
    (*show).worldOfMasters.species = gBattleResults.playerMon1Species;
    (*show).worldOfMasters.location = gMapHeader.regionMapSectionId;
}
pub(crate) unsafe extern "C" fn TryPutPokemonTodayFailedOnTheAir() {
    let mut ballsUsed: u16 = 0;
    let mut i: u8 = 0;
    let mut show: *mut TVShow = null_mut();
    if BernoulliTrial(65535) == 0 {
        i = 0;
        ballsUsed = 0;
        while i < 11 {
            ballsUsed += gBattleResults.catchAttempts[i] as u16;
            i += 1;
        }
        if ballsUsed > 255 {
            ballsUsed = 255;
        }
        if ballsUsed > 2
            && (gBattleOutcome == B_OUTCOME_MON_FLED || gBattleOutcome == B_OUTCOME_WON)
        {
            sCurTVShowSlot =
                FindFirstEmptyRecordMixTVShowSlot((*gSaveBlock1Ptr).tvShows.as_mut_ptr());
            if sCurTVShowSlot != -1
                && IsRecordMixShowAlreadySpawned(TVSHOW_POKEMON_TODAY_FAILED, FALSE) != 1
            {
                show = &raw mut (*gSaveBlock1Ptr).tvShows[sCurTVShowSlot];
                (*show).pokemonTodayFailed.kind = TVSHOW_POKEMON_TODAY_FAILED;
                (*show).pokemonTodayFailed.active = FALSE;
                (*show).pokemonTodayFailed.species = gBattleResults.playerMon1Species;
                (*show).pokemonTodayFailed.species2 = gBattleResults.lastOpponentSpecies;
                (*show).pokemonTodayFailed.nBallsUsed = ballsUsed as u8;
                (*show).pokemonTodayFailed.outcome = gBattleOutcome;
                (*show).pokemonTodayFailed.location = gMapHeader.regionMapSectionId;
                StringCopy(
                    (*show).pokemonTodayFailed.playerName.as_mut_ptr(),
                    (*gSaveBlock2Ptr).playerName.as_mut_ptr(),
                );
                StorePlayerIdInRecordMixShow(show);
                (*show).pokemonTodayFailed.language = gGameLanguage;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn StorePlayerIdInRecordMixShow(show: *mut TVShow) {
    let mut id: u32 = GetPlayerIDAsU32();
    (*show).common.srcTrainerId2Lo = id as u8;
    (*show).common.srcTrainerId2Hi = (id >> 8) as u8;
    (*show).common.srcTrainerIdLo = id as u8;
    (*show).common.srcTrainerIdHi = (id >> 8) as u8;
    (*show).common.trainerIdLo = id as u8;
    (*show).common.trainerIdHi = (id >> 8) as u8;
}
pub(crate) unsafe extern "C" fn StorePlayerIdInNormalShow(show: *mut TVShow) {
    let mut id: u32 = GetPlayerIDAsU32();
    (*show).common.srcTrainerIdLo = id as u8;
    (*show).common.srcTrainerIdHi = (id >> 8) as u8;
    (*show).common.trainerIdLo = id as u8;
    (*show).common.trainerIdHi = (id >> 8) as u8;
}
pub(crate) unsafe extern "C" fn InterviewAfter_ContestLiveUpdates() {
    let mut show: *mut TVShow = null_mut();
    let mut show2: *mut TVShow = null_mut();
    show = &raw mut (*gSaveBlock1Ptr).tvShows[24];
    if (*show).contestLiveUpdates.kind == TVSHOW_CONTEST_LIVE_UPDATES {
        show2 = &raw mut (*gSaveBlock1Ptr).tvShows[sCurTVShowSlot];
        (*show2).contestLiveUpdates.kind = TVSHOW_CONTEST_LIVE_UPDATES;
        (*show2).contestLiveUpdates.active = TRUE;
        StringCopy(
            (*show2).contestLiveUpdates.winningTrainerName.as_mut_ptr(),
            (*gSaveBlock2Ptr).playerName.as_mut_ptr(),
        );
        (*show2).contestLiveUpdates.category = gSpecialVar_ContestCategory as u8;
        (*show2).contestLiveUpdates.winningSpecies = GetMonData3(
            &raw mut gPlayerParty[gContestMonPartyIndex],
            MON_DATA_SPECIES,
            null_mut(),
        ) as u16;
        (*show2).contestLiveUpdates.losingSpecies = (*show).contestLiveUpdates.losingSpecies;
        (*show2).contestLiveUpdates.loserAppealFlag = (*show).contestLiveUpdates.loserAppealFlag;
        (*show2).contestLiveUpdates.round1Placing = (*show).contestLiveUpdates.round1Placing;
        (*show2).contestLiveUpdates.round2Placing = (*show).contestLiveUpdates.round2Placing;
        (*show2).contestLiveUpdates.r#move = (*show).contestLiveUpdates.r#move;
        (*show2).contestLiveUpdates.winnerAppealFlag = (*show).contestLiveUpdates.winnerAppealFlag;
        StringCopy(
            (*show2).contestLiveUpdates.losingTrainerName.as_mut_ptr(),
            (*show).contestLiveUpdates.losingTrainerName.as_mut_ptr(),
        );
        StorePlayerIdInNormalShow(show2);
        (*show2).contestLiveUpdates.winningTrainerLanguage = gGameLanguage;
        (*show2).contestLiveUpdates.losingTrainerLanguage =
            (*show).contestLiveUpdates.losingTrainerLanguage;
        DeleteTVShowInArrayByIdx((*gSaveBlock1Ptr).tvShows.as_mut_ptr(), LAST_TVSHOW_IDX);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PutBattleUpdateOnTheAir(
    opponentLinkPlayerId: u8,
    r#move: u16,
    speciesPlayer: u16,
    speciesOpponent: u16,
) {
    let mut show: *mut TVShow = null_mut();
    let mut name: CArray<u8, 32> = zeroed();
    sCurTVShowSlot = FindFirstEmptyNormalTVShowSlot((*gSaveBlock1Ptr).tvShows.as_mut_ptr());
    if sCurTVShowSlot != -1 {
        TryReplaceOldTVShowOfKind(TVSHOW_BATTLE_UPDATE);
        if gSpecialVar_Result != TRUE as u16 {
            show = &raw mut (*gSaveBlock1Ptr).tvShows[sCurTVShowSlot];
            (*show).battleUpdate.kind = TVSHOW_BATTLE_UPDATE;
            (*show).battleUpdate.active = TRUE;
            StringCopy(
                (*show).battleUpdate.playerName.as_mut_ptr(),
                (*gSaveBlock2Ptr).playerName.as_mut_ptr(),
            );
            if gBattleTypeFlags & BATTLE_TYPE_MULTI != 0 {
                (*show).battleUpdate.battleType = 2;
            } else if gBattleTypeFlags & BATTLE_TYPE_DOUBLE != 0 {
                (*show).battleUpdate.battleType = 1;
            } else {
                (*show).battleUpdate.battleType = 0;
            }
            (*show).battleUpdate.r#move = r#move;
            (*show).battleUpdate.speciesPlayer = speciesPlayer;
            (*show).battleUpdate.speciesOpponent = speciesOpponent;
            StringCopy(
                name.as_mut_ptr(),
                gLinkPlayers[opponentLinkPlayerId].name.as_mut_ptr(),
            );
            StripExtCtrlCodes(name.as_mut_ptr());
            StringCopy(
                (*show).battleUpdate.linkOpponentName.as_mut_ptr(),
                name.as_mut_ptr(),
            );
            StorePlayerIdInNormalShow(show);
            (*show).battleUpdate.language = gGameLanguage;
            if (*show).battleUpdate.language == LANGUAGE_JAPANESE
                || gLinkPlayers[opponentLinkPlayerId].language == LANGUAGE_JAPANESE as u16
            {
                (*show).battleUpdate.linkOpponentLanguage = LANGUAGE_JAPANESE;
            } else {
                (*show).battleUpdate.linkOpponentLanguage =
                    gLinkPlayers[opponentLinkPlayerId].language as u8;
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
    let mut show: *mut TVShow = null_mut();
    let mut name: CArray<u8, 32> = zeroed();
    sCurTVShowSlot = FindFirstEmptyNormalTVShowSlot((*gSaveBlock1Ptr).tvShows.as_mut_ptr());
    if sCurTVShowSlot == -1 {
        return FALSE;
    }
    TryReplaceOldTVShowOfKind(TVSHOW_3_CHEERS_FOR_POKEBLOCKS);
    if gSpecialVar_Result == TRUE as u16 {
        return FALSE;
    }
    show = &raw mut (*gSaveBlock1Ptr).tvShows[sCurTVShowSlot];
    (*show).threeCheers.kind = TVSHOW_3_CHEERS_FOR_POKEBLOCKS;
    (*show).threeCheers.active = TRUE;
    StringCopy(
        (*show).threeCheers.playerName.as_mut_ptr(),
        (*gSaveBlock2Ptr).playerName.as_mut_ptr(),
    );
    StringCopy(name.as_mut_ptr(), partnersName);
    StripExtCtrlCodes(name.as_mut_ptr());
    StringCopy(
        (*show).threeCheers.worstBlenderName.as_mut_ptr(),
        name.as_mut_ptr(),
    );
    (*show).threeCheers.set_flavor(flavor);
    (*show).threeCheers.set_color(color);
    (*show).threeCheers.sheen = sheen;
    StorePlayerIdInNormalShow(show);
    (*show).threeCheers.language = gGameLanguage;
    if (*show).threeCheers.language == LANGUAGE_JAPANESE || language == LANGUAGE_JAPANESE {
        (*show).threeCheers.worstBlenderLanguage = LANGUAGE_JAPANESE;
    } else {
        (*show).threeCheers.worstBlenderLanguage = language;
    }
    return TRUE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PutFanClubSpecialOnTheAir() {
    let mut show: *mut TVShow = null_mut();
    let mut name: CArray<u8, 32> = zeroed();
    let mut id: u32 = 0;
    show = &raw mut (*gSaveBlock1Ptr).tvShows[gSpecialVar_0x8006];
    (*show).fanClubSpecial.score = gSpecialVar_0x8005 as u8 * 10;
    StringCopy(
        (*show).fanClubSpecial.playerName.as_mut_ptr(),
        (*gSaveBlock2Ptr).playerName.as_mut_ptr(),
    );
    (*show).fanClubSpecial.kind = TVSHOW_FAN_CLUB_SPECIAL;
    (*show).fanClubSpecial.active = TRUE;
    id = GetPlayerIDAsU32();
    (*show).fanClubSpecial.idLo = id as u8;
    (*show).fanClubSpecial.idHi = (id >> 8) as u8;
    StringCopy(name.as_mut_ptr(), gStringVar1.as_mut_ptr());
    StripExtCtrlCodes(name.as_mut_ptr());
    StringCopy(
        (*show).fanClubSpecial.idolName.as_mut_ptr(),
        name.as_mut_ptr(),
    );
    StorePlayerIdInNormalShow(show);
    (*show).fanClubSpecial.language = gGameLanguage;
    if (*show).fanClubSpecial.language == LANGUAGE_JAPANESE
        || (*gSaveBlock1Ptr).linkBattleRecords.languages[0] == LANGUAGE_JAPANESE
    {
        (*show).fanClubSpecial.idolNameLanguage = LANGUAGE_JAPANESE;
    } else {
        (*show).fanClubSpecial.idolNameLanguage = (*gSaveBlock1Ptr).linkBattleRecords.languages[0];
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ContestLiveUpdates_Init(round1Placing: u8) {
    let mut show: *mut TVShow = null_mut();
    DeleteTVShowInArrayByIdx((*gSaveBlock1Ptr).tvShows.as_mut_ptr(), LAST_TVSHOW_IDX);
    sCurTVShowSlot = FindFirstEmptyNormalTVShowSlot((*gSaveBlock1Ptr).tvShows.as_mut_ptr());
    if sCurTVShowSlot != -1 {
        show = &raw mut (*gSaveBlock1Ptr).tvShows[24];
        (*show).contestLiveUpdates.round1Placing = round1Placing;
        (*show).contestLiveUpdates.kind = TVSHOW_CONTEST_LIVE_UPDATES;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ContestLiveUpdates_SetRound2Placing(round2Placing: u8) {
    let mut show: *mut TVShow = &raw mut (*gSaveBlock1Ptr).tvShows[24];
    sCurTVShowSlot = FindFirstEmptyNormalTVShowSlot((*gSaveBlock1Ptr).tvShows.as_mut_ptr());
    if sCurTVShowSlot != -1 {
        (*show).contestLiveUpdates.round2Placing = round2Placing;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ContestLiveUpdates_SetWinnerAppealFlag(flag: u8) {
    let mut show: *mut TVShow = &raw mut (*gSaveBlock1Ptr).tvShows[24];
    sCurTVShowSlot = FindFirstEmptyNormalTVShowSlot((*gSaveBlock1Ptr).tvShows.as_mut_ptr());
    if sCurTVShowSlot != -1 {
        (*show).contestLiveUpdates.winnerAppealFlag = flag;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ContestLiveUpdates_SetWinnerMoveUsed(r#move: u16) {
    let mut show: *mut TVShow = &raw mut (*gSaveBlock1Ptr).tvShows[24];
    sCurTVShowSlot = FindFirstEmptyNormalTVShowSlot((*gSaveBlock1Ptr).tvShows.as_mut_ptr());
    if sCurTVShowSlot != -1 {
        (*show).contestLiveUpdates.r#move = r#move;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ContestLiveUpdates_SetLoserData(flag: u8, loser: u8) {
    let mut show: *mut TVShow = &raw mut (*gSaveBlock1Ptr).tvShows[24];
    sCurTVShowSlot = FindFirstEmptyNormalTVShowSlot((*gSaveBlock1Ptr).tvShows.as_mut_ptr());
    if sCurTVShowSlot != -1 {
        (*show).contestLiveUpdates.losingSpecies = gContestMons[loser].species;
        StringCopy(
            (*show).contestLiveUpdates.losingTrainerName.as_mut_ptr(),
            gContestMons[loser].trainerName.as_mut_ptr(),
        );
        StripExtCtrlCodes((*show).contestLiveUpdates.losingTrainerName.as_mut_ptr());
        (*show).contestLiveUpdates.loserAppealFlag = flag;
        if loser as i32 + 1 > gNumLinkContestPlayers as i32 {
            (*show).contestLiveUpdates.losingTrainerLanguage = gLinkPlayers[0].language as u8;
        } else if gGameLanguage == LANGUAGE_JAPANESE
            || gLinkPlayers[loser].language == LANGUAGE_JAPANESE as u16
        {
            (*show).contestLiveUpdates.losingTrainerLanguage = LANGUAGE_JAPANESE;
        } else {
            (*show).contestLiveUpdates.losingTrainerLanguage = gLinkPlayers[loser].language as u8;
        }
    }
}
pub(crate) unsafe extern "C" fn InterviewAfter_BravoTrainerPokemonProfile() {
    let mut show: *mut TVShow = null_mut();
    let mut show2: *mut TVShow = null_mut();
    show = &raw mut (*gSaveBlock1Ptr).tvShows[24];
    if (*show).bravoTrainer.kind == TVSHOW_BRAVO_TRAINER_POKEMON_PROFILE {
        show2 = &raw mut (*gSaveBlock1Ptr).tvShows[sCurTVShowSlot];
        (*show2).bravoTrainer.kind = TVSHOW_BRAVO_TRAINER_POKEMON_PROFILE;
        (*show2).bravoTrainer.active = TRUE;
        (*show2).bravoTrainer.species = (*show).bravoTrainer.species;
        StringCopy(
            (*show2).bravoTrainer.playerName.as_mut_ptr(),
            (*gSaveBlock2Ptr).playerName.as_mut_ptr(),
        );
        StringCopy(
            (*show2).bravoTrainer.pokemonNickname.as_mut_ptr(),
            (*show).bravoTrainer.pokemonNickname.as_mut_ptr(),
        );
        (*show2)
            .bravoTrainer
            .set_contestCategory((*show).bravoTrainer.contestCategory());
        (*show2)
            .bravoTrainer
            .set_contestRank((*show).bravoTrainer.contestRank());
        (*show2).bravoTrainer.r#move = (*show).bravoTrainer.r#move;
        (*show2)
            .bravoTrainer
            .set_contestResult((*show).bravoTrainer.contestResult());
        (*show2)
            .bravoTrainer
            .set_contestCategory((*show).bravoTrainer.contestCategory());
        StorePlayerIdInNormalShow(show2);
        (*show2).bravoTrainer.language = gGameLanguage;
        if (*show2).bravoTrainer.language == LANGUAGE_JAPANESE
            || (*show).bravoTrainer.pokemonNameLanguage == LANGUAGE_JAPANESE
        {
            (*show2).bravoTrainer.pokemonNameLanguage = LANGUAGE_JAPANESE;
        } else {
            (*show2).bravoTrainer.pokemonNameLanguage = (*show).bravoTrainer.pokemonNameLanguage;
        }
        DeleteTVShowInArrayByIdx((*gSaveBlock1Ptr).tvShows.as_mut_ptr(), LAST_TVSHOW_IDX);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BravoTrainerPokemonProfile_BeforeInterview1(r#move: u16) {
    let mut show: *mut TVShow = &raw mut (*gSaveBlock1Ptr).tvShows[24];
    InterviewBefore_BravoTrainerPkmnProfile();
    sCurTVShowSlot = FindFirstEmptyNormalTVShowSlot((*gSaveBlock1Ptr).tvShows.as_mut_ptr());
    if sCurTVShowSlot != -1 {
        DeleteTVShowInArrayByIdx((*gSaveBlock1Ptr).tvShows.as_mut_ptr(), LAST_TVSHOW_IDX);
        (*show).bravoTrainer.r#move = r#move;
        (*show).bravoTrainer.kind = TVSHOW_BRAVO_TRAINER_POKEMON_PROFILE;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BravoTrainerPokemonProfile_BeforeInterview2(contestStandingPlace: u8) {
    let mut show: *mut TVShow = &raw mut (*gSaveBlock1Ptr).tvShows[24];
    sCurTVShowSlot = FindFirstEmptyNormalTVShowSlot((*gSaveBlock1Ptr).tvShows.as_mut_ptr());
    if sCurTVShowSlot != -1 {
        (*show).bravoTrainer.set_contestResult(contestStandingPlace);
        (*show)
            .bravoTrainer
            .set_contestCategory(gSpecialVar_ContestCategory as u8);
        (*show)
            .bravoTrainer
            .set_contestRank(gSpecialVar_ContestRank as u8);
        (*show).bravoTrainer.species = GetMonData3(
            &raw mut gPlayerParty[gContestMonPartyIndex],
            MON_DATA_SPECIES,
            null_mut(),
        ) as u16;
        GetMonData3(
            &raw mut gPlayerParty[gContestMonPartyIndex],
            MON_DATA_NICKNAME,
            (*show).bravoTrainer.pokemonNickname.as_mut_ptr(),
        );
        StripExtCtrlCodes((*show).bravoTrainer.pokemonNickname.as_mut_ptr());
        (*show).bravoTrainer.pokemonNameLanguage = GetMonData2(
            &raw mut gPlayerParty[gContestMonPartyIndex],
            MON_DATA_LANGUAGE,
        ) as u8;
    }
}
pub(crate) unsafe extern "C" fn InterviewAfter_BravoTrainerBattleTowerProfile() {
    let mut show: *mut TVShow = &raw mut (*gSaveBlock1Ptr).tvShows[sCurTVShowSlot];
    (*show).bravoTrainerTower.kind = TVSHOW_BRAVO_TRAINER_BATTLE_TOWER_PROFILE;
    (*show).bravoTrainerTower.active = TRUE;
    StringCopy(
        (*show).bravoTrainerTower.playerName.as_mut_ptr(),
        (*gSaveBlock2Ptr).playerName.as_mut_ptr(),
    );
    StringCopy(
        (*show).bravoTrainerTower.opponentName.as_mut_ptr(),
        (*gSaveBlock2Ptr)
            .frontier
            .towerInterview
            .opponentName
            .as_mut_ptr(),
    );
    (*show).bravoTrainerTower.species = (*gSaveBlock2Ptr).frontier.towerInterview.playerSpecies;
    (*show).bravoTrainerTower.defeatedSpecies =
        (*gSaveBlock2Ptr).frontier.towerInterview.opponentSpecies;
    (*show).bravoTrainerTower.numFights =
        GetCurrentBattleTowerWinStreak((*gSaveBlock2Ptr).frontier.towerLvlMode, 0);
    (*show).bravoTrainerTower.wonTheChallenge = (*gSaveBlock2Ptr).frontier.towerBattleOutcome;
    if (*gSaveBlock2Ptr).frontier.towerLvlMode == FRONTIER_LVL_50 {
        (*show).bravoTrainerTower.btLevel = FRONTIER_MAX_LEVEL_50;
    } else {
        (*show).bravoTrainerTower.btLevel = FRONTIER_MAX_LEVEL_OPEN;
    }
    (*show).bravoTrainerTower.interviewResponse = gSpecialVar_0x8004 as u8;
    StorePlayerIdInNormalShow(show);
    (*show).bravoTrainerTower.playerLanguage = gGameLanguage;
    if (*show).bravoTrainerTower.playerLanguage == LANGUAGE_JAPANESE
        || (*gSaveBlock2Ptr).frontier.towerInterview.opponentLanguage == LANGUAGE_JAPANESE
    {
        (*show).bravoTrainerTower.opponentLanguage = LANGUAGE_JAPANESE;
    } else {
        (*show).bravoTrainerTower.opponentLanguage =
            (*gSaveBlock2Ptr).frontier.towerInterview.opponentLanguage;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn TryPutSmartShopperOnAir() {
    let mut show: *mut TVShow = null_mut();
    let mut i: u8 = 0;
    if !((*gSaveBlock1Ptr).location.mapGroup == 26 && (*gSaveBlock1Ptr).location.mapNum == 60)
        && !((*gSaveBlock1Ptr).location.mapGroup == 26 && (*gSaveBlock1Ptr).location.mapNum == 55)
        && BernoulliTrial(21845) == 0
    {
        sCurTVShowSlot = FindFirstEmptyRecordMixTVShowSlot((*gSaveBlock1Ptr).tvShows.as_mut_ptr());
        if sCurTVShowSlot != -1 && IsRecordMixShowAlreadySpawned(TVSHOW_SMART_SHOPPER, FALSE) != 1 {
            SortPurchasesByQuantity();
            if gMartPurchaseHistory[0].quantity >= 20 {
                show = &raw mut (*gSaveBlock1Ptr).tvShows[sCurTVShowSlot];
                (*show).smartshopperShow.kind = TVSHOW_SMART_SHOPPER;
                (*show).smartshopperShow.active = FALSE;
                (*show).smartshopperShow.shopLocation = gMapHeader.regionMapSectionId;
                i = 0;
                while i < SMARTSHOPPER_NUM_ITEMS {
                    (*show).smartshopperShow.itemIds[i] = gMartPurchaseHistory[i].itemId;
                    (*show).smartshopperShow.itemAmounts[i] = gMartPurchaseHistory[i].quantity;
                    i += 1;
                }
                (*show).smartshopperShow.priceReduced = IsPokeNewsActive(POKENEWS_SLATEPORT);
                StringCopy(
                    (*show).smartshopperShow.playerName.as_mut_ptr(),
                    (*gSaveBlock2Ptr).playerName.as_mut_ptr(),
                );
                StorePlayerIdInRecordMixShow(show);
                (*show).smartshopperShow.language = gGameLanguage;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PutNameRaterShowOnTheAir() {
    let mut show: *mut TVShow = null_mut();
    InterviewBefore_NameRater();
    if gSpecialVar_Result != 1 {
        GetMonData3(
            &raw mut gPlayerParty[gSpecialVar_0x8004],
            MON_DATA_NICKNAME,
            gStringVar1.as_mut_ptr(),
        );
        if StringLength((*gSaveBlock2Ptr).playerName.as_mut_ptr()) > 1
            && StringLength(gStringVar1.as_mut_ptr()) > 1
        {
            show = &raw mut (*gSaveBlock1Ptr).tvShows[sCurTVShowSlot];
            (*show).nameRaterShow.kind = TVSHOW_NAME_RATER_SHOW;
            (*show).nameRaterShow.active = TRUE;
            (*show).nameRaterShow.species = GetMonData3(
                &raw mut gPlayerParty[gSpecialVar_0x8004],
                MON_DATA_SPECIES,
                null_mut(),
            ) as u16;
            (*show).nameRaterShow.random = (Random() as i32 % 3) as u8;
            (*show).nameRaterShow.random2 = (Random() as i32 % 2) as u8;
            (*show).nameRaterShow.randomSpecies =
                GetRandomDifferentSpeciesSeenByPlayer((*show).nameRaterShow.species);
            StringCopy(
                (*show).nameRaterShow.trainerName.as_mut_ptr(),
                (*gSaveBlock2Ptr).playerName.as_mut_ptr(),
            );
            GetMonData3(
                &raw mut gPlayerParty[gSpecialVar_0x8004],
                MON_DATA_NICKNAME,
                (*show).nameRaterShow.pokemonName.as_mut_ptr(),
            );
            StripExtCtrlCodes((*show).nameRaterShow.pokemonName.as_mut_ptr());
            StorePlayerIdInNormalShow(show);
            (*show).nameRaterShow.language = gGameLanguage;
            (*show).nameRaterShow.pokemonNameLanguage =
                GetMonData2(&raw mut gPlayerParty[gSpecialVar_0x8004], MON_DATA_LANGUAGE) as u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn StartMassOutbreak() {
    let mut show: *mut TVShow = &raw mut (*gSaveBlock1Ptr).tvShows[gSpecialVar_0x8004];
    (*gSaveBlock1Ptr).outbreakPokemonSpecies = (*show).massOutbreak.species;
    (*gSaveBlock1Ptr).outbreakLocationMapNum = (*show).massOutbreak.locationMapNum;
    (*gSaveBlock1Ptr).outbreakLocationMapGroup = (*show).massOutbreak.locationMapGroup;
    (*gSaveBlock1Ptr).outbreakPokemonLevel = (*show).massOutbreak.level;
    (*gSaveBlock1Ptr).outbreakUnused1 = (*show).massOutbreak.unused1;
    (*gSaveBlock1Ptr).outbreakUnused2 = (*show).massOutbreak.unused2;
    (*gSaveBlock1Ptr).outbreakPokemonMoves[0] = (*show).massOutbreak.moves[0];
    (*gSaveBlock1Ptr).outbreakPokemonMoves[1] = (*show).massOutbreak.moves[1];
    (*gSaveBlock1Ptr).outbreakPokemonMoves[2] = (*show).massOutbreak.moves[2];
    (*gSaveBlock1Ptr).outbreakPokemonMoves[3] = (*show).massOutbreak.moves[3];
    (*gSaveBlock1Ptr).outbreakUnused3 = (*show).massOutbreak.unused3;
    (*gSaveBlock1Ptr).outbreakPokemonProbability = (*show).massOutbreak.probability;
    (*gSaveBlock1Ptr).outbreakDaysLeft = 2;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PutLilycoveContestLadyShowOnTheAir() {
    let mut show: *mut TVShow = null_mut();
    Script_FindFirstEmptyNormalTVShowSlot();
    if gSpecialVar_Result != TRUE as u16 {
        show = &raw mut (*gSaveBlock1Ptr).tvShows[sCurTVShowSlot];
        BufferContestLadyLanguage(&raw mut (*show).contestLady.language);
        (*show).contestLady.pokemonNameLanguage = GAME_LANGUAGE;
        (*show).contestLady.kind = TVSHOW_LILYCOVE_CONTEST_LADY;
        (*show).contestLady.active = TRUE;
        BufferContestLadyPlayerName((*show).contestLady.playerName.as_mut_ptr());
        BufferContestLadyMonName(
            &raw mut (*show).contestLady.contestCategory,
            (*show).contestLady.nickname.as_mut_ptr(),
        );
        (*show).contestLady.pokeblockState = GetContestLadyPokeblockState();
        StorePlayerIdInNormalShow(show);
    }
}
pub(crate) unsafe extern "C" fn InterviewAfter_FanClubLetter() {
    let mut show: *mut TVShow = &raw mut (*gSaveBlock1Ptr).tvShows[sCurTVShowSlot];
    (*show).fanclubLetter.kind = TVSHOW_FAN_CLUB_LETTER;
    (*show).fanclubLetter.active = TRUE;
    StringCopy(
        (*show).fanclubLetter.playerName.as_mut_ptr(),
        (*gSaveBlock2Ptr).playerName.as_mut_ptr(),
    );
    (*show).fanclubLetter.species = GetMonData3(
        &raw mut gPlayerParty[GetLeadMonIndex()],
        MON_DATA_SPECIES,
        null_mut(),
    ) as u16;
    StorePlayerIdInNormalShow(show);
    (*show).fanclubLetter.language = gGameLanguage;
}
pub(crate) unsafe extern "C" fn InterviewAfter_RecentHappenings() {
    let mut show: *mut TVShow = &raw mut (*gSaveBlock1Ptr).tvShows[sCurTVShowSlot];
    (*show).recentHappenings.kind = TVSHOW_RECENT_HAPPENINGS;
    (*show).recentHappenings.active = TRUE;
    StringCopy(
        (*show).recentHappenings.playerName.as_mut_ptr(),
        (*gSaveBlock2Ptr).playerName.as_mut_ptr(),
    );
    (*show).recentHappenings.species = SPECIES_NONE;
    StorePlayerIdInNormalShow(show);
    (*show).recentHappenings.language = gGameLanguage;
}
pub(crate) unsafe extern "C" fn InterviewAfter_PkmnFanClubOpinions() {
    let mut show: *mut TVShow = &raw mut (*gSaveBlock1Ptr).tvShows[sCurTVShowSlot];
    (*show).fanclubOpinions.kind = TVSHOW_PKMN_FAN_CLUB_OPINIONS;
    (*show).fanclubOpinions.active = TRUE;
    (*show).fanclubOpinions.set_friendshipHighNybble(
        (GetMonData3(
            &raw mut gPlayerParty[GetLeadMonIndex()],
            MON_DATA_FRIENDSHIP,
            null_mut(),
        ) >> 4) as u8,
    );
    (*show)
        .fanclubOpinions
        .set_questionAsked(gSpecialVar_0x8007 as u8);
    StringCopy(
        (*show).fanclubOpinions.playerName.as_mut_ptr(),
        (*gSaveBlock2Ptr).playerName.as_mut_ptr(),
    );
    GetMonData3(
        &raw mut gPlayerParty[GetLeadMonIndex()],
        MON_DATA_NICKNAME,
        (*show).fanclubOpinions.nickname.as_mut_ptr(),
    );
    StripExtCtrlCodes((*show).fanclubOpinions.nickname.as_mut_ptr());
    (*show).fanclubOpinions.species = GetMonData3(
        &raw mut gPlayerParty[GetLeadMonIndex()],
        MON_DATA_SPECIES,
        null_mut(),
    ) as u16;
    StorePlayerIdInNormalShow(show);
    (*show).fanclubOpinions.language = gGameLanguage;
    if gGameLanguage == LANGUAGE_JAPANESE
        || GetMonData2(&raw mut gPlayerParty[GetLeadMonIndex()], MON_DATA_LANGUAGE)
            == LANGUAGE_JAPANESE as u32
    {
        (*show).fanclubOpinions.pokemonNameLanguage = LANGUAGE_JAPANESE;
    } else {
        (*show).fanclubOpinions.pokemonNameLanguage =
            GetMonData2(&raw mut gPlayerParty[GetLeadMonIndex()], MON_DATA_LANGUAGE) as u8;
    }
}
pub(crate) unsafe extern "C" fn InterviewAfter_Dummy() {
    let mut show: *mut TVShow = &raw mut (*gSaveBlock1Ptr).tvShows[sCurTVShowSlot];
}
pub(crate) unsafe extern "C" fn TryStartRandomMassOutbreak() {
    let mut i: u8 = 0;
    let mut outbreakIdx: u16 = 0;
    let mut show: *mut TVShow = null_mut();
    if FlagGet(FLAG_SYS_GAME_CLEAR) != 0 {
        i = 0;
        while i < LAST_TVSHOW_IDX {
            if (*gSaveBlock1Ptr).tvShows[i].common.kind == TVSHOW_MASS_OUTBREAK {
                return;
            }
            i += 1;
        }
        if BernoulliTrial(327) == 0 {
            sCurTVShowSlot = FindFirstEmptyNormalTVShowSlot((*gSaveBlock1Ptr).tvShows.as_mut_ptr());
            if sCurTVShowSlot != -1 {
                outbreakIdx = Random() % 5;
                show = &raw mut (*gSaveBlock1Ptr).tvShows[sCurTVShowSlot];
                (*show).massOutbreak.kind = TVSHOW_MASS_OUTBREAK;
                (*show).massOutbreak.active = TRUE;
                (*show).massOutbreak.level = sPokeOutbreakSpeciesList[outbreakIdx].level;
                (*show).massOutbreak.unused1 = 0;
                (*show).massOutbreak.unused3 = 0;
                (*show).massOutbreak.species = sPokeOutbreakSpeciesList[outbreakIdx].species;
                (*show).massOutbreak.unused2 = 0;
                (*show).massOutbreak.moves[0] = sPokeOutbreakSpeciesList[outbreakIdx].moves[0];
                (*show).massOutbreak.moves[1] = sPokeOutbreakSpeciesList[outbreakIdx].moves[1];
                (*show).massOutbreak.moves[2] = sPokeOutbreakSpeciesList[outbreakIdx].moves[2];
                (*show).massOutbreak.moves[3] = sPokeOutbreakSpeciesList[outbreakIdx].moves[3];
                (*show).massOutbreak.locationMapNum =
                    sPokeOutbreakSpeciesList[outbreakIdx].location;
                (*show).massOutbreak.locationMapGroup = 0;
                (*show).massOutbreak.unused4 = 0;
                (*show).massOutbreak.probability = 50;
                (*show).massOutbreak.unused5 = 0;
                (*show).massOutbreak.daysBeforeOutbreak = 1;
                StorePlayerIdInNormalShow(show);
                (*show).massOutbreak.language = gGameLanguage;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn EndMassOutbreak() {
    (*gSaveBlock1Ptr).outbreakPokemonSpecies = SPECIES_NONE;
    (*gSaveBlock1Ptr).outbreakLocationMapNum = 0;
    (*gSaveBlock1Ptr).outbreakLocationMapGroup = 0;
    (*gSaveBlock1Ptr).outbreakPokemonLevel = 0;
    (*gSaveBlock1Ptr).outbreakUnused1 = 0;
    (*gSaveBlock1Ptr).outbreakUnused2 = 0;
    (*gSaveBlock1Ptr).outbreakPokemonMoves[0] = 0;
    (*gSaveBlock1Ptr).outbreakPokemonMoves[1] = MOVE_NONE;
    (*gSaveBlock1Ptr).outbreakPokemonMoves[2] = MOVE_NONE;
    (*gSaveBlock1Ptr).outbreakPokemonMoves[3] = MOVE_NONE;
    (*gSaveBlock1Ptr).outbreakUnused3 = 0;
    (*gSaveBlock1Ptr).outbreakPokemonProbability = 0;
    (*gSaveBlock1Ptr).outbreakDaysLeft = 0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn UpdateTVShowsPerDay(days: u16) {
    UpdateTimeBeforeMassOutbreak(days);
    TryEndMassOutbreak(days);
    UpdatePokeNewsCountdown(days);
    ResolveWorldOfMastersShow(days);
    ResolveNumberOneShow(days);
}
pub(crate) unsafe extern "C" fn UpdateTimeBeforeMassOutbreak(days: u16) {
    let mut i: u8 = 0;
    let mut show: *mut TVShow = null_mut();
    if (*gSaveBlock1Ptr).outbreakPokemonSpecies == SPECIES_NONE {
        i = 0;
        while i < LAST_TVSHOW_IDX {
            if (*gSaveBlock1Ptr).tvShows[i].massOutbreak.kind == TVSHOW_MASS_OUTBREAK
                && (*gSaveBlock1Ptr).tvShows[i].massOutbreak.active == TRUE
            {
                show = &raw mut (*gSaveBlock1Ptr).tvShows[i];
                if (*show).massOutbreak.daysBeforeOutbreak < days {
                    (*show).massOutbreak.daysBeforeOutbreak = 0;
                } else {
                    (*show).massOutbreak.daysBeforeOutbreak -= days;
                }
                break;
            }
            i += 1;
        }
    }
}
pub(crate) unsafe extern "C" fn TryEndMassOutbreak(days: u16) {
    if (*gSaveBlock1Ptr).outbreakDaysLeft <= days {
        EndMassOutbreak();
    } else {
        (*gSaveBlock1Ptr).outbreakDaysLeft -= days;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn RecordFishingAttemptForTV(caughtFish: u8) {
    if caughtFish != 0 {
        if sPokemonAnglerAttemptCounters >> 8 > 4 {
            TryPutFishingAdviceOnAir();
        }
        sPokemonAnglerAttemptCounters &= 0xFF;
        if sPokemonAnglerAttemptCounters != 0xFF {
            sPokemonAnglerAttemptCounters += 0x01;
        }
    } else {
        if sPokemonAnglerAttemptCounters as u8 > 4 {
            TryPutFishingAdviceOnAir();
        }
        sPokemonAnglerAttemptCounters &= 0xFF00;
        if sPokemonAnglerAttemptCounters >> 8 != 0xFF {
            sPokemonAnglerAttemptCounters += 0x0100;
        }
    }
}
pub(crate) unsafe extern "C" fn TryPutFishingAdviceOnAir() {
    let mut show: *mut TVShow = null_mut();
    sCurTVShowSlot = FindFirstEmptyRecordMixTVShowSlot((*gSaveBlock1Ptr).tvShows.as_mut_ptr());
    if sCurTVShowSlot != -1 && IsRecordMixShowAlreadySpawned(TVSHOW_FISHING_ADVICE, FALSE) != 1 {
        show = &raw mut (*gSaveBlock1Ptr).tvShows[sCurTVShowSlot];
        (*show).pokemonAngler.kind = TVSHOW_FISHING_ADVICE;
        (*show).pokemonAngler.active = FALSE;
        (*show).pokemonAngler.nBites = sPokemonAnglerAttemptCounters as u8;
        (*show).pokemonAngler.nFails = (sPokemonAnglerAttemptCounters >> 8) as u8;
        (*show).pokemonAngler.species = sPokemonAnglerSpecies;
        StringCopy(
            (*show).pokemonAngler.playerName.as_mut_ptr(),
            (*gSaveBlock2Ptr).playerName.as_mut_ptr(),
        );
        StorePlayerIdInRecordMixShow(show);
        (*show).pokemonAngler.language = gGameLanguage;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetPokemonAnglerSpecies(species: u16) {
    sPokemonAnglerSpecies = species;
}
pub(crate) unsafe extern "C" fn ResolveWorldOfMastersShow(days: u16) {
    let mut show: *mut TVShow = &raw mut (*gSaveBlock1Ptr).tvShows[24];
    if (*show).worldOfMasters.kind == TVSHOW_WORLD_OF_MASTERS {
        if (*show).worldOfMasters.numPokeCaught >= 20 {
            TryPutWorldOfMastersOnAir();
        }
        DeleteTVShowInArrayByIdx((*gSaveBlock1Ptr).tvShows.as_mut_ptr(), LAST_TVSHOW_IDX);
    }
}
pub(crate) unsafe extern "C" fn TryPutWorldOfMastersOnAir() {
    let mut show: *mut TVShow = null_mut();
    let mut show2: *mut TVShow = null_mut();
    show = &raw mut (*gSaveBlock1Ptr).tvShows[24];
    if BernoulliTrial(65535) == 0 {
        sCurTVShowSlot = FindFirstEmptyRecordMixTVShowSlot((*gSaveBlock1Ptr).tvShows.as_mut_ptr());
        if sCurTVShowSlot != -1
            && IsRecordMixShowAlreadySpawned(TVSHOW_WORLD_OF_MASTERS, FALSE) != 1
        {
            show2 = &raw mut (*gSaveBlock1Ptr).tvShows[sCurTVShowSlot];
            (*show2).worldOfMasters.kind = TVSHOW_WORLD_OF_MASTERS;
            (*show2).worldOfMasters.active = FALSE;
            (*show2).worldOfMasters.numPokeCaught = (*show).worldOfMasters.numPokeCaught;
            (*show2).worldOfMasters.steps =
                GetGameStat(GAME_STAT_STEPS) as u16 - (*show).worldOfMasters.steps;
            (*show2).worldOfMasters.caughtPoke = (*show).worldOfMasters.caughtPoke;
            (*show2).worldOfMasters.species = (*show).worldOfMasters.species;
            (*show2).worldOfMasters.location = (*show).worldOfMasters.location;
            StringCopy(
                (*show2).worldOfMasters.playerName.as_mut_ptr(),
                (*gSaveBlock2Ptr).playerName.as_mut_ptr(),
            );
            StorePlayerIdInRecordMixShow(show2);
            (*show2).worldOfMasters.language = gGameLanguage;
            DeleteTVShowInArrayByIdx((*gSaveBlock1Ptr).tvShows.as_mut_ptr(), LAST_TVSHOW_IDX);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn TryPutTodaysRivalTrainerOnAir() {
    let mut show: *mut TVShow = null_mut();
    let mut i: u32 = 0;
    let mut nBadges: u8 = 0;
    IsRecordMixShowAlreadySpawned(TVSHOW_TODAYS_RIVAL_TRAINER, TRUE);
    sCurTVShowSlot = FindFirstEmptyRecordMixTVShowSlot((*gSaveBlock1Ptr).tvShows.as_mut_ptr());
    if sCurTVShowSlot != -1 {
        show = &raw mut (*gSaveBlock1Ptr).tvShows[sCurTVShowSlot];
        (*show).rivalTrainer.kind = TVSHOW_TODAYS_RIVAL_TRAINER;
        (*show).rivalTrainer.active = FALSE;
        i = FLAG_BADGE01_GET;
        nBadges = 0;
        while i < 2159 {
            if FlagGet(i as u16) != 0 {
                nBadges += 1;
            }
            i += 1;
        }
        (*show).rivalTrainer.badgeCount = nBadges;
        if IsNationalPokedexEnabled() != 0 {
            (*show).rivalTrainer.dexCount = GetNationalPokedexCount(FLAG_GET_CAUGHT);
        } else {
            (*show).rivalTrainer.dexCount = GetHoennPokedexCount(FLAG_GET_CAUGHT);
        }
        (*show).rivalTrainer.location = gMapHeader.regionMapSectionId;
        (*show).rivalTrainer.mapLayoutId = gMapHeader.mapLayoutId;
        (*show).rivalTrainer.nSilverSymbols = 0;
        (*show).rivalTrainer.nGoldSymbols = 0;
        i = 0;
        while i < NUM_FRONTIER_FACILITIES as u32 {
            if FlagGet(sSilverSymbolFlags[i]) == TRUE {
                (*show).rivalTrainer.nSilverSymbols += 1;
            }
            if FlagGet(sGoldSymbolFlags[i]) == TRUE {
                (*show).rivalTrainer.nGoldSymbols += 1;
            }
            i += 1;
        }
        (*show).rivalTrainer.battlePoints = (*gSaveBlock2Ptr).frontier.battlePoints;
        StringCopy(
            (*show).rivalTrainer.playerName.as_mut_ptr(),
            (*gSaveBlock2Ptr).playerName.as_mut_ptr(),
        );
        StorePlayerIdInRecordMixShow(show);
        (*show).rivalTrainer.language = gGameLanguage;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn TryPutTrendWatcherOnAir(words: *mut u16) {
    let mut show: *mut TVShow = null_mut();
    sCurTVShowSlot = FindFirstEmptyRecordMixTVShowSlot((*gSaveBlock1Ptr).tvShows.as_mut_ptr());
    if sCurTVShowSlot != -1 && IsRecordMixShowAlreadySpawned(TVSHOW_TREND_WATCHER, FALSE) != 1 {
        show = &raw mut (*gSaveBlock1Ptr).tvShows[sCurTVShowSlot];
        (*show).trendWatcher.kind = TVSHOW_TREND_WATCHER;
        (*show).trendWatcher.active = FALSE;
        (*show).trendWatcher.gender = (*gSaveBlock2Ptr).playerGender;
        (*show).trendWatcher.words[0] = *words;
        (*show).trendWatcher.words[1] = *words.at(1);
        StringCopy(
            (*show).trendWatcher.playerName.as_mut_ptr(),
            (*gSaveBlock2Ptr).playerName.as_mut_ptr(),
        );
        StorePlayerIdInRecordMixShow(show);
        (*show).trendWatcher.language = gGameLanguage;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn TryPutTreasureInvestigatorsOnAir() {
    let mut show: *mut TVShow = null_mut();
    sCurTVShowSlot = FindFirstEmptyRecordMixTVShowSlot((*gSaveBlock1Ptr).tvShows.as_mut_ptr());
    if sCurTVShowSlot != -1
        && IsRecordMixShowAlreadySpawned(TVSHOW_TREASURE_INVESTIGATORS, FALSE) != 1
    {
        show = &raw mut (*gSaveBlock1Ptr).tvShows[sCurTVShowSlot];
        (*show).treasureInvestigators.kind = TVSHOW_TREASURE_INVESTIGATORS;
        (*show).treasureInvestigators.active = FALSE;
        (*show).treasureInvestigators.item = gSpecialVar_0x8005;
        (*show).treasureInvestigators.location = gMapHeader.regionMapSectionId;
        (*show).treasureInvestigators.mapLayoutId = gMapHeader.mapLayoutId;
        StringCopy(
            (*show).treasureInvestigators.playerName.as_mut_ptr(),
            (*gSaveBlock2Ptr).playerName.as_mut_ptr(),
        );
        StorePlayerIdInRecordMixShow(show);
        (*show).treasureInvestigators.language = gGameLanguage;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn TryPutFindThatGamerOnAir(nCoinsPaidOut: u16) {
    let mut show: *mut TVShow = null_mut();
    let mut flag: u8 = 0;
    let mut nCoinsWon: u16 = 0;
    sCurTVShowSlot = FindFirstEmptyRecordMixTVShowSlot((*gSaveBlock1Ptr).tvShows.as_mut_ptr());
    if sCurTVShowSlot != -1 && IsRecordMixShowAlreadySpawned(TVSHOW_FIND_THAT_GAMER, FALSE) != 1 {
        flag = FALSE;
        'l1: {
            match sFindThatGamerWhichGame {
                SLOT_MACHINE => {
                    if nCoinsPaidOut as i32 >= sFindThatGamerCoinsSpent as i32 + 200 {
                        flag = TRUE;
                        nCoinsWon = nCoinsPaidOut - sFindThatGamerCoinsSpent;
                        break 'l1;
                    }
                    if sFindThatGamerCoinsSpent >= 100
                        && nCoinsPaidOut as i32 <= sFindThatGamerCoinsSpent as i32 - 100
                    {
                        nCoinsWon = sFindThatGamerCoinsSpent - nCoinsPaidOut;
                        break 'l1;
                    }
                    return;
                }
                ROULETTE => {
                    if nCoinsPaidOut as i32 >= sFindThatGamerCoinsSpent as i32 + 50 {
                        flag = TRUE;
                        nCoinsWon = nCoinsPaidOut - sFindThatGamerCoinsSpent;
                        break 'l1;
                    }
                    if sFindThatGamerCoinsSpent >= 50
                        && nCoinsPaidOut as i32 <= sFindThatGamerCoinsSpent as i32 - 50
                    {
                        nCoinsWon = sFindThatGamerCoinsSpent - nCoinsPaidOut;
                        break 'l1;
                    }
                    return;
                }
                _ => {
                    return;
                }
            }
        }
        show = &raw mut (*gSaveBlock1Ptr).tvShows[sCurTVShowSlot];
        (*show).findThatGamer.kind = TVSHOW_FIND_THAT_GAMER;
        (*show).findThatGamer.active = FALSE;
        (*show).findThatGamer.nCoins = nCoinsWon;
        (*show).findThatGamer.whichGame = sFindThatGamerWhichGame;
        (*show).findThatGamer.won = flag;
        StringCopy(
            (*show).findThatGamer.playerName.as_mut_ptr(),
            (*gSaveBlock2Ptr).playerName.as_mut_ptr(),
        );
        StorePlayerIdInRecordMixShow(show);
        (*show).findThatGamer.language = gGameLanguage;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AlertTVThatPlayerPlayedSlotMachine(nCoinsSpent: u16) {
    sFindThatGamerWhichGame = SLOT_MACHINE;
    sFindThatGamerCoinsSpent = nCoinsSpent;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AlertTVThatPlayerPlayedRoulette(nCoinsSpent: u16) {
    sFindThatGamerWhichGame = ROULETTE;
    sFindThatGamerCoinsSpent = nCoinsSpent;
}
pub(crate) unsafe extern "C" fn SecretBaseVisit_CalculateDecorationData(show: *mut TVShow) {
    let mut i: u8 = 0;
    let mut j: u8 = 0;
    let mut k: u16 = 0;
    let mut n: u8 = 0;
    let mut decoration: u8 = 0;
    i = 0;
    while i < DECOR_MAX_SECRET_BASE {
        sTV_DecorationsBuffer[i] = DECOR_NONE;
        i += 1;
    }
    i = 0;
    n = 0;
    while i < DECOR_MAX_SECRET_BASE {
        decoration = (*gSaveBlock1Ptr).secretBases[0].decorations[i];
        if decoration != DECOR_NONE {
            j = 0;
            while j < DECOR_MAX_SECRET_BASE {
                if sTV_DecorationsBuffer[j] == DECOR_NONE {
                    sTV_DecorationsBuffer[j] = decoration;
                    n += 1;
                    break;
                }
                if sTV_DecorationsBuffer[j] == decoration {
                    break;
                }
                j += 1;
            }
        }
        i += 1;
    }
    if n > 4 {
        (*show).secretBaseVisit.numDecorations = 4;
    } else {
        (*show).secretBaseVisit.numDecorations = n;
    }
    match (*show).secretBaseVisit.numDecorations {
        0 => {}
        1 => {
            (*show).secretBaseVisit.decorations[0] = sTV_DecorationsBuffer[0];
        }
        _ => {
            k = 0;
            while (k as i32) < n as i32 * n as i32 {
                decoration = rem_i32(Random() as i32, n as i32) as u8;
                j = rem_i32(Random() as i32, n as i32) as u8;
                i = sTV_DecorationsBuffer[decoration];
                sTV_DecorationsBuffer[decoration] = sTV_DecorationsBuffer[j];
                sTV_DecorationsBuffer[j] = i;
                k += 1;
            }
            i = 0;
            while i < (*show).secretBaseVisit.numDecorations {
                (*show).secretBaseVisit.decorations[i] = sTV_DecorationsBuffer[i];
                i += 1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SecretBaseVisit_CalculatePartyData(show: *mut TVShow) {
    let mut i: u8 = 0;
    let mut r#move: u16 = 0;
    let mut j: u16 = 0;
    let mut numMoves: u8 = 0;
    let mut numPokemon: u8 = 0;
    let mut sum: u16 = 0;
    i = 0;
    numPokemon = 0;
    while i < PARTY_SIZE as u8 {
        if GetMonData2(&raw mut gPlayerParty[i], MON_DATA_SPECIES) != SPECIES_NONE as u32
            && GetMonData2(&raw mut gPlayerParty[i], MON_DATA_IS_EGG) == 0
        {
            sTV_SecretBaseVisitMonsTemp[numPokemon].level =
                GetMonData2(&raw mut gPlayerParty[i], MON_DATA_LEVEL) as u8;
            sTV_SecretBaseVisitMonsTemp[numPokemon].species =
                GetMonData2(&raw mut gPlayerParty[i], MON_DATA_SPECIES) as u16;
            numMoves = 0;
            r#move = GetMonData2(&raw mut gPlayerParty[i], MON_DATA_MOVE1) as u16;
            if r#move != MOVE_NONE {
                sTV_SecretBaseVisitMovesTemp[numMoves] = r#move;
                numMoves += 1;
            }
            r#move = GetMonData2(&raw mut gPlayerParty[i], MON_DATA_MOVE2) as u16;
            if r#move != MOVE_NONE {
                sTV_SecretBaseVisitMovesTemp[numMoves] = r#move;
                numMoves += 1;
            }
            r#move = GetMonData2(&raw mut gPlayerParty[i], MON_DATA_MOVE3) as u16;
            if r#move != MOVE_NONE {
                sTV_SecretBaseVisitMovesTemp[numMoves] = r#move;
                numMoves += 1;
            }
            r#move = GetMonData2(&raw mut gPlayerParty[i], MON_DATA_MOVE4) as u16;
            if r#move != MOVE_NONE {
                sTV_SecretBaseVisitMovesTemp[numMoves] = r#move;
                numMoves += 1;
            }
            sTV_SecretBaseVisitMonsTemp[numPokemon].r#move =
                sTV_SecretBaseVisitMovesTemp[rem_i32(Random() as i32, numMoves as i32)];
            numPokemon += 1;
        }
        i += 1;
    }
    i = 0;
    sum = 0;
    while i < numPokemon {
        sum += sTV_SecretBaseVisitMonsTemp[i].level as u16;
        i += 1;
    }
    (*show).secretBaseVisit.avgLevel = div_i32(sum as i32, numPokemon as i32) as u8;
    j = rem_i32(Random() as i32, numPokemon as i32) as u16;
    (*show).secretBaseVisit.species = sTV_SecretBaseVisitMonsTemp[j].species;
    (*show).secretBaseVisit.r#move = sTV_SecretBaseVisitMonsTemp[j].r#move;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn TryPutSecretBaseVisitOnAir() {
    let mut show: *mut TVShow = null_mut();
    IsRecordMixShowAlreadySpawned(TVSHOW_SECRET_BASE_VISIT, TRUE);
    sCurTVShowSlot = FindFirstEmptyRecordMixTVShowSlot((*gSaveBlock1Ptr).tvShows.as_mut_ptr());
    if sCurTVShowSlot != -1 {
        show = &raw mut (*gSaveBlock1Ptr).tvShows[sCurTVShowSlot];
        (*show).secretBaseVisit.kind = TVSHOW_SECRET_BASE_VISIT;
        (*show).secretBaseVisit.active = FALSE;
        StringCopy(
            (*show).secretBaseVisit.playerName.as_mut_ptr(),
            (*gSaveBlock2Ptr).playerName.as_mut_ptr(),
        );
        SecretBaseVisit_CalculateDecorationData(show);
        SecretBaseVisit_CalculatePartyData(show);
        StorePlayerIdInRecordMixShow(show);
        (*show).secretBaseVisit.language = gGameLanguage;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn TryPutBreakingNewsOnAir() {
    let mut show: *mut TVShow = null_mut();
    let mut i: u8 = 0;
    let mut balls: u16 = 0;
    sCurTVShowSlot = FindFirstEmptyRecordMixTVShowSlot((*gSaveBlock1Ptr).tvShows.as_mut_ptr());
    if sCurTVShowSlot != -1 && IsRecordMixShowAlreadySpawned(TVSHOW_BREAKING_NEWS, FALSE) != 1 {
        show = &raw mut (*gSaveBlock1Ptr).tvShows[sCurTVShowSlot];
        (*show).breakingNews.kind = TVSHOW_BREAKING_NEWS;
        (*show).breakingNews.active = FALSE;
        balls = 0;
        i = 0;
        while i < 11 {
            balls += gBattleResults.catchAttempts[i] as u16;
            i += 1;
        }
        if gBattleResults.usedMasterBall() != 0 {
            balls += 1;
        }
        (*show).breakingNews.location = gMapHeader.regionMapSectionId;
        StringCopy(
            (*show).breakingNews.playerName.as_mut_ptr(),
            (*gSaveBlock2Ptr).playerName.as_mut_ptr(),
        );
        (*show).breakingNews.poke1Species = gBattleResults.playerMon1Species;
        match gBattleOutcome {
            B_OUTCOME_LOST | B_OUTCOME_DREW => {
                (*show).breakingNews.kind = TVSHOW_OFF_AIR;
                return;
            }
            B_OUTCOME_CAUGHT => {
                (*show).breakingNews.outcome = 0;
            }
            B_OUTCOME_WON => {
                (*show).breakingNews.outcome = 1;
            }
            B_OUTCOME_RAN | B_OUTCOME_PLAYER_TELEPORTED | B_OUTCOME_NO_SAFARI_BALLS => {
                (*show).breakingNews.outcome = 2;
            }
            B_OUTCOME_MON_FLED | B_OUTCOME_MON_TELEPORTED => {
                (*show).breakingNews.outcome = 3;
            }
            _ => {}
        }
        (*show).breakingNews.lastOpponentSpecies = gBattleResults.lastOpponentSpecies;
        match (*show).breakingNews.outcome {
            0 => {
                if gBattleResults.usedMasterBall() != 0 {
                    (*show).breakingNews.caughtMonBall = ITEM_MASTER_BALL;
                } else {
                    (*show).breakingNews.caughtMonBall = gBattleResults.caughtMonBall() as u16;
                }
                (*show).breakingNews.balls = balls;
            }
            1 => {
                (*show).breakingNews.lastUsedMove = gBattleResults.lastUsedMovePlayer;
            }
            2 | 3 => {}
            _ => {}
        }
        StorePlayerIdInRecordMixShow(show);
        (*show).breakingNews.language = gGameLanguage;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn TryPutLotteryWinnerReportOnAir() {
    let mut show: *mut TVShow = null_mut();
    sCurTVShowSlot = FindFirstEmptyRecordMixTVShowSlot((*gSaveBlock1Ptr).tvShows.as_mut_ptr());
    if sCurTVShowSlot != -1 && IsRecordMixShowAlreadySpawned(TVSHOW_LOTTO_WINNER, FALSE) != 1 {
        show = &raw mut (*gSaveBlock1Ptr).tvShows[sCurTVShowSlot];
        (*show).lottoWinner.kind = TVSHOW_LOTTO_WINNER;
        (*show).lottoWinner.active = FALSE;
        StringCopy(
            (*show).lottoWinner.playerName.as_mut_ptr(),
            (*gSaveBlock2Ptr).playerName.as_mut_ptr(),
        );
        (*show).lottoWinner.whichPrize = 4 - gSpecialVar_0x8004 as u8;
        (*show).lottoWinner.item = gSpecialVar_0x8005;
        StorePlayerIdInRecordMixShow(show);
        (*show).lottoWinner.language = gGameLanguage;
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
    let mut show: *mut TVShow = null_mut();
    let mut i: u8 = 0;
    let mut j: u8 = 0;
    sCurTVShowSlot = FindFirstEmptyRecordMixTVShowSlot((*gSaveBlock1Ptr).tvShows.as_mut_ptr());
    if sCurTVShowSlot != -1 && IsRecordMixShowAlreadySpawned(TVSHOW_BATTLE_SEMINAR, FALSE) != 1 {
        show = &raw mut (*gSaveBlock1Ptr).tvShows[sCurTVShowSlot];
        (*show).battleSeminar.kind = TVSHOW_BATTLE_SEMINAR;
        (*show).battleSeminar.active = FALSE;
        StringCopy(
            (*show).battleSeminar.playerName.as_mut_ptr(),
            (*gSaveBlock2Ptr).playerName.as_mut_ptr(),
        );
        (*show).battleSeminar.foeSpecies = foeSpecies;
        (*show).battleSeminar.species = species;
        (*show).battleSeminar.r#move = *movePtr.at(moveIndex);
        i = 0;
        j = 0;
        while i < MAX_MON_MOVES as u8 {
            if i != moveIndex && *movePtr.at(i) != 0 {
                (*show).battleSeminar.otherMoves[j] = *movePtr.at(i);
                j += 1;
            }
            i += 1;
        }
        (*show).battleSeminar.nOtherMoves = j;
        (*show).battleSeminar.betterMove = betterMove;
        StorePlayerIdInRecordMixShow(show);
        (*show).battleSeminar.language = gGameLanguage;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn TryPutSafariFanClubOnAir(monsCaught: u8, pokeblocksUsed: u8) {
    let mut show: *mut TVShow = null_mut();
    sCurTVShowSlot = FindFirstEmptyRecordMixTVShowSlot((*gSaveBlock1Ptr).tvShows.as_mut_ptr());
    if sCurTVShowSlot != -1 && IsRecordMixShowAlreadySpawned(TVSHOW_SAFARI_FAN_CLUB, FALSE) != 1 {
        show = &raw mut (*gSaveBlock1Ptr).tvShows[sCurTVShowSlot];
        (*show).safariFanClub.kind = TVSHOW_SAFARI_FAN_CLUB;
        (*show).safariFanClub.active = FALSE;
        StringCopy(
            (*show).safariFanClub.playerName.as_mut_ptr(),
            (*gSaveBlock2Ptr).playerName.as_mut_ptr(),
        );
        (*show).safariFanClub.monsCaught = monsCaught;
        (*show).safariFanClub.pokeblocksUsed = pokeblocksUsed;
        StorePlayerIdInRecordMixShow(show);
        (*show).safariFanClub.language = gGameLanguage;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn TryPutSpotTheCutiesOnAir(pokemon: *mut Pokemon, ribbonMonDataIdx: u8) {
    let mut show: *mut TVShow = null_mut();
    sCurTVShowSlot = FindFirstEmptyRecordMixTVShowSlot((*gSaveBlock1Ptr).tvShows.as_mut_ptr());
    if sCurTVShowSlot != -1 && IsRecordMixShowAlreadySpawned(TVSHOW_CUTIES, FALSE) != 1 {
        show = &raw mut (*gSaveBlock1Ptr).tvShows[sCurTVShowSlot];
        (*show).cuties.kind = TVSHOW_CUTIES;
        (*show).cuties.active = FALSE;
        StringCopy(
            (*show).cuties.playerName.as_mut_ptr(),
            (*gSaveBlock2Ptr).playerName.as_mut_ptr(),
        );
        GetMonData3(
            pokemon,
            MON_DATA_NICKNAME,
            (*show).cuties.nickname.as_mut_ptr(),
        );
        StripExtCtrlCodes((*show).cuties.nickname.as_mut_ptr());
        (*show).cuties.nRibbons = GetRibbonCount(pokemon);
        (*show).cuties.selectedRibbon = MonDataIdxToRibbon(ribbonMonDataIdx);
        StorePlayerIdInRecordMixShow(show);
        (*show).cuties.language = gGameLanguage;
        if (*show).cuties.language == LANGUAGE_JAPANESE
            || GetMonData2(pokemon, MON_DATA_LANGUAGE) == LANGUAGE_JAPANESE as u32
        {
            (*show).cuties.pokemonNameLanguage = LANGUAGE_JAPANESE;
        } else {
            (*show).cuties.pokemonNameLanguage = GetMonData2(pokemon, MON_DATA_LANGUAGE) as u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetRibbonCount(pokemon: *mut Pokemon) -> u8 {
    let mut nRibbons: u8 = 0;
    nRibbons = 0;
    nRibbons += GetMonData2(pokemon, MON_DATA_COOL_RIBBON) as u8;
    nRibbons += GetMonData2(pokemon, MON_DATA_BEAUTY_RIBBON) as u8;
    nRibbons += GetMonData2(pokemon, MON_DATA_CUTE_RIBBON) as u8;
    nRibbons += GetMonData2(pokemon, MON_DATA_SMART_RIBBON) as u8;
    nRibbons += GetMonData2(pokemon, MON_DATA_TOUGH_RIBBON) as u8;
    nRibbons += GetMonData2(pokemon, MON_DATA_CHAMPION_RIBBON) as u8;
    nRibbons += GetMonData2(pokemon, MON_DATA_WINNING_RIBBON) as u8;
    nRibbons += GetMonData2(pokemon, MON_DATA_VICTORY_RIBBON) as u8;
    nRibbons += GetMonData2(pokemon, MON_DATA_ARTIST_RIBBON) as u8;
    nRibbons += GetMonData2(pokemon, MON_DATA_EFFORT_RIBBON) as u8;
    nRibbons += GetMonData2(pokemon, MON_DATA_MARINE_RIBBON) as u8;
    nRibbons += GetMonData2(pokemon, MON_DATA_LAND_RIBBON) as u8;
    nRibbons += GetMonData2(pokemon, MON_DATA_SKY_RIBBON) as u8;
    nRibbons += GetMonData2(pokemon, MON_DATA_COUNTRY_RIBBON) as u8;
    nRibbons += GetMonData2(pokemon, MON_DATA_NATIONAL_RIBBON) as u8;
    nRibbons += GetMonData2(pokemon, MON_DATA_EARTH_RIBBON) as u8;
    nRibbons += GetMonData2(pokemon, MON_DATA_WORLD_RIBBON) as u8;
    return nRibbons;
}
pub(crate) unsafe extern "C" fn MonDataIdxToRibbon(monDataIdx: u8) -> u8 {
    if monDataIdx == MON_DATA_CHAMPION_RIBBON as u8 {
        return CHAMPION_RIBBON;
    }
    if monDataIdx == MON_DATA_COOL_RIBBON as u8 {
        return COOL_RIBBON_NORMAL;
    }
    if monDataIdx == MON_DATA_BEAUTY_RIBBON as u8 {
        return BEAUTY_RIBBON_NORMAL;
    }
    if monDataIdx == MON_DATA_CUTE_RIBBON as u8 {
        return CUTE_RIBBON_NORMAL;
    }
    if monDataIdx == MON_DATA_SMART_RIBBON as u8 {
        return SMART_RIBBON_NORMAL;
    }
    if monDataIdx == MON_DATA_TOUGH_RIBBON as u8 {
        return TOUGH_RIBBON_NORMAL;
    }
    if monDataIdx == MON_DATA_WINNING_RIBBON as u8 {
        return WINNING_RIBBON;
    }
    if monDataIdx == MON_DATA_VICTORY_RIBBON as u8 {
        return VICTORY_RIBBON;
    }
    if monDataIdx == MON_DATA_ARTIST_RIBBON as u8 {
        return ARTIST_RIBBON;
    }
    if monDataIdx == MON_DATA_EFFORT_RIBBON as u8 {
        return EFFORT_RIBBON;
    }
    if monDataIdx == MON_DATA_MARINE_RIBBON as u8 {
        return MARINE_RIBBON;
    }
    if monDataIdx == MON_DATA_LAND_RIBBON as u8 {
        return LAND_RIBBON;
    }
    if monDataIdx == MON_DATA_SKY_RIBBON as u8 {
        return SKY_RIBBON;
    }
    if monDataIdx == MON_DATA_COUNTRY_RIBBON as u8 {
        return COUNTRY_RIBBON;
    }
    if monDataIdx == MON_DATA_NATIONAL_RIBBON as u8 {
        return NATIONAL_RIBBON;
    }
    if monDataIdx == MON_DATA_EARTH_RIBBON as u8 {
        return EARTH_RIBBON;
    }
    if monDataIdx == MON_DATA_WORLD_RIBBON as u8 {
        return WORLD_RIBBON;
    }
    return CHAMPION_RIBBON;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn TryPutTrainerFanClubOnAir() {
    let mut show: *mut TVShow = null_mut();
    sCurTVShowSlot = FindFirstEmptyRecordMixTVShowSlot((*gSaveBlock1Ptr).tvShows.as_mut_ptr());
    if sCurTVShowSlot != -1 && IsRecordMixShowAlreadySpawned(TVSHOW_TRAINER_FAN_CLUB, FALSE) != 1 {
        show = &raw mut (*gSaveBlock1Ptr).tvShows[sCurTVShowSlot];
        (*show).trainerFanClub.kind = TVSHOW_TRAINER_FAN_CLUB;
        (*show).trainerFanClub.active = FALSE;
        StringCopy(
            (*show).trainerFanClub.playerName.as_mut_ptr(),
            (*gSaveBlock2Ptr).playerName.as_mut_ptr(),
        );
        (*show).trainerFanClub.words[0] = (*gSaveBlock1Ptr).easyChatProfile[0];
        (*show).trainerFanClub.words[1] = (*gSaveBlock1Ptr).easyChatProfile[1];
        StorePlayerIdInRecordMixShow(show);
        (*show).trainerFanClub.language = gGameLanguage;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ShouldHideFanClubInterviewer() -> u8 {
    sCurTVShowSlot = FindFirstEmptyNormalTVShowSlot((*gSaveBlock1Ptr).tvShows.as_mut_ptr());
    if sCurTVShowSlot == -1 {
        return TRUE;
    }
    TryReplaceOldTVShowOfKind(TVSHOW_FAN_CLUB_SPECIAL);
    if gSpecialVar_Result == TRUE as u16 {
        return TRUE;
    }
    if (*gSaveBlock1Ptr).linkBattleRecords.entries[0].name[0] == EOS {
        return TRUE;
    }
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ShouldAirFrontierTVShow() -> u8 {
    let mut playerId: u32 = 0;
    let mut showIdx: u8 = 0;
    let mut shows: *mut TVShow = null_mut();
    if IsRecordMixShowAlreadySpawned(TVSHOW_FRONTIER, FALSE) == TRUE {
        shows = (*gSaveBlock1Ptr).tvShows.as_mut_ptr();
        playerId = GetPlayerIDAsU32();
        showIdx = NUM_NORMAL_TVSHOW_SLOTS;
        while showIdx < LAST_TVSHOW_IDX {
            if (*shows.at(showIdx)).common.kind == TVSHOW_FRONTIER
                && playerId & 0xFF == (*shows.at(showIdx)).common.trainerIdLo as u32
                && playerId >> 8 & 0xFF == (*shows.at(showIdx)).common.trainerIdHi as u32
            {
                DeleteTVShowInArrayByIdx((*gSaveBlock1Ptr).tvShows.as_mut_ptr(), showIdx);
                CompactTVShowArray((*gSaveBlock1Ptr).tvShows.as_mut_ptr());
                return TRUE;
            }
            showIdx += 1;
        }
    }
    sCurTVShowSlot = FindFirstEmptyRecordMixTVShowSlot((*gSaveBlock1Ptr).tvShows.as_mut_ptr());
    if sCurTVShowSlot == -1 {
        return FALSE;
    }
    return TRUE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn TryPutFrontierTVShowOnAir(winStreak: u16, facilityAndMode: u8) {
    let mut show: *mut TVShow = null_mut();
    sCurTVShowSlot = FindFirstEmptyRecordMixTVShowSlot((*gSaveBlock1Ptr).tvShows.as_mut_ptr());
    if sCurTVShowSlot != -1 {
        show = &raw mut (*gSaveBlock1Ptr).tvShows[sCurTVShowSlot];
        (*show).frontier.kind = TVSHOW_FRONTIER;
        (*show).frontier.active = FALSE;
        StringCopy(
            (*show).frontier.playerName.as_mut_ptr(),
            (*gSaveBlock2Ptr).playerName.as_mut_ptr(),
        );
        (*show).frontier.winStreak = winStreak;
        (*show).frontier.facilityAndMode = facilityAndMode;
        match facilityAndMode {
            FRONTIER_SHOW_TOWER_SINGLES
            | FRONTIER_SHOW_DOME_SINGLES
            | FRONTIER_SHOW_DOME_DOUBLES
            | FRONTIER_SHOW_FACTORY_SINGLES
            | FRONTIER_SHOW_FACTORY_DOUBLES
            | FRONTIER_SHOW_PIKE
            | FRONTIER_SHOW_ARENA
            | FRONTIER_SHOW_PALACE_SINGLES
            | FRONTIER_SHOW_PALACE_DOUBLES
            | FRONTIER_SHOW_PYRAMID => {
                (*show).frontier.species1 =
                    GetMonData3(&raw mut gPlayerParty[0], MON_DATA_SPECIES, null_mut()) as u16;
                (*show).frontier.species2 =
                    GetMonData3(&raw mut gPlayerParty[1], MON_DATA_SPECIES, null_mut()) as u16;
                (*show).frontier.species3 =
                    GetMonData3(&raw mut gPlayerParty[2], MON_DATA_SPECIES, null_mut()) as u16;
            }
            FRONTIER_SHOW_TOWER_DOUBLES => {
                (*show).frontier.species1 =
                    GetMonData3(&raw mut gPlayerParty[0], MON_DATA_SPECIES, null_mut()) as u16;
                (*show).frontier.species2 =
                    GetMonData3(&raw mut gPlayerParty[1], MON_DATA_SPECIES, null_mut()) as u16;
                (*show).frontier.species3 =
                    GetMonData3(&raw mut gPlayerParty[2], MON_DATA_SPECIES, null_mut()) as u16;
                (*show).frontier.species4 =
                    GetMonData3(&raw mut gPlayerParty[3], MON_DATA_SPECIES, null_mut()) as u16;
            }
            FRONTIER_SHOW_TOWER_MULTIS => {
                (*show).frontier.species1 =
                    GetMonData3(&raw mut gPlayerParty[0], MON_DATA_SPECIES, null_mut()) as u16;
                (*show).frontier.species2 =
                    GetMonData3(&raw mut gPlayerParty[1], MON_DATA_SPECIES, null_mut()) as u16;
            }
            FRONTIER_SHOW_TOWER_LINK_MULTIS => {
                (*show).frontier.species1 = GetMonData3(
                    &raw mut (*gSaveBlock1Ptr).playerParty
                        [(*gSaveBlock2Ptr).frontier.selectedPartyMons[0] as i32 - 1],
                    MON_DATA_SPECIES,
                    null_mut(),
                ) as u16;
                (*show).frontier.species2 = GetMonData3(
                    &raw mut (*gSaveBlock1Ptr).playerParty
                        [(*gSaveBlock2Ptr).frontier.selectedPartyMons[1] as i32 - 1],
                    MON_DATA_SPECIES,
                    null_mut(),
                ) as u16;
            }
            _ => {}
        }
        StorePlayerIdInRecordMixShow(show);
        (*show).frontier.language = gGameLanguage;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn TryPutSecretBaseSecretsOnAir() {
    let mut show: *mut TVShow = null_mut();
    let mut strbuf: CArray<u8, 32> = zeroed();
    if IsRecordMixShowAlreadySpawned(TVSHOW_SECRET_BASE_SECRETS, FALSE) != TRUE {
        sCurTVShowSlot = FindFirstEmptyRecordMixTVShowSlot((*gSaveBlock1Ptr).tvShows.as_mut_ptr());
        if sCurTVShowSlot != -1 {
            show = &raw mut (*gSaveBlock1Ptr).tvShows[sCurTVShowSlot];
            (*show).secretBaseSecrets.kind = TVSHOW_SECRET_BASE_SECRETS;
            (*show).secretBaseSecrets.active = FALSE;
            StringCopy(
                (*show).secretBaseSecrets.playerName.as_mut_ptr(),
                (*gSaveBlock2Ptr).playerName.as_mut_ptr(),
            );
            (*show).secretBaseSecrets.stepsInBase = VarGet(VAR_SECRET_BASE_STEP_COUNTER);
            CopyCurSecretBaseOwnerName_StrVar1();
            StringCopy(strbuf.as_mut_ptr(), gStringVar1.as_mut_ptr());
            StripExtCtrlCodes(strbuf.as_mut_ptr());
            StringCopy(
                (*show).secretBaseSecrets.baseOwnersName.as_mut_ptr(),
                strbuf.as_mut_ptr(),
            );
            (*show).secretBaseSecrets.item = VarGet(VAR_SECRET_BASE_LAST_ITEM_USED);
            (*show).secretBaseSecrets.flags = VarGet(VAR_SECRET_BASE_LOW_TV_FLAGS) as u32
                + ((VarGet(VAR_SECRET_BASE_HIGH_TV_FLAGS) as u32) << 16);
            StorePlayerIdInRecordMixShow(show);
            (*show).secretBaseSecrets.language = gGameLanguage;
            if (*show).secretBaseSecrets.language == LANGUAGE_JAPANESE
                || (*gSaveBlock1Ptr).secretBases[VarGet(VAR_CURRENT_SECRET_BASE)].language
                    == LANGUAGE_JAPANESE
            {
                (*show).secretBaseSecrets.baseOwnersNameLanguage = LANGUAGE_JAPANESE;
            } else {
                (*show).secretBaseSecrets.baseOwnersNameLanguage =
                    (*gSaveBlock1Ptr).secretBases[VarGet(VAR_CURRENT_SECRET_BASE)].language;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn ResolveNumberOneShow(days: u16) {
    let mut i: u8 = 0;
    i = 0;
    while i < 7 {
        if VarGet(sNumberOneVarsAndThresholds[i][0]) >= sNumberOneVarsAndThresholds[i][1] {
            TryPutNumberOneOnAir(i);
            break;
        }
        i += 1;
    }
    i = 0;
    while i < 7 {
        VarSet(sNumberOneVarsAndThresholds[i][0], 0);
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn TryPutNumberOneOnAir(actionIdx: u8) {
    let mut show: *mut TVShow = null_mut();
    IsRecordMixShowAlreadySpawned(TVSHOW_NUMBER_ONE, TRUE);
    sCurTVShowSlot = FindFirstEmptyRecordMixTVShowSlot((*gSaveBlock1Ptr).tvShows.as_mut_ptr());
    if sCurTVShowSlot != -1 {
        show = &raw mut (*gSaveBlock1Ptr).tvShows[sCurTVShowSlot];
        (*show).numberOne.kind = TVSHOW_NUMBER_ONE;
        (*show).numberOne.active = FALSE;
        StringCopy(
            (*show).numberOne.playerName.as_mut_ptr(),
            (*gSaveBlock2Ptr).playerName.as_mut_ptr(),
        );
        (*show).numberOne.actionIdx = actionIdx;
        (*show).numberOne.count = VarGet(sNumberOneVarsAndThresholds[actionIdx][0]);
        StorePlayerIdInRecordMixShow(show);
        (*show).numberOne.language = gGameLanguage;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IncrementDailySlotsUses() {
    VarSet(VAR_DAILY_SLOTS, VarGet(VAR_DAILY_SLOTS) + 1);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IncrementDailyRouletteUses() {
    VarSet(VAR_DAILY_ROULETTE, VarGet(VAR_DAILY_ROULETTE) + 1);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IncrementDailyWildBattles() {
    VarSet(VAR_DAILY_WILDS, VarGet(VAR_DAILY_WILDS) + 1);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IncrementDailyBerryBlender() {
    VarSet(VAR_DAILY_BLENDER, VarGet(VAR_DAILY_BLENDER) + 1);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IncrementDailyPlantedBerries() {
    VarSet(
        VAR_DAILY_PLANTED_BERRIES,
        VarGet(VAR_DAILY_PLANTED_BERRIES) + 1,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IncrementDailyPickedBerries() {
    VarSet(
        VAR_DAILY_PICKED_BERRIES,
        VarGet(VAR_DAILY_PICKED_BERRIES) + gSpecialVar_0x8006,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IncrementDailyBattlePoints(delta: u16) {
    VarSet(VAR_DAILY_BP, VarGet(VAR_DAILY_BP) + delta);
}
pub(crate) unsafe extern "C" fn TryPutRandomPokeNewsOnAir() {
    if FlagGet(FLAG_SYS_GAME_CLEAR) != 0 {
        sCurTVShowSlot = GetFirstEmptyPokeNewsSlot((*gSaveBlock1Ptr).pokeNews.as_mut_ptr());
        if sCurTVShowSlot != -1 && BernoulliTrial(655) != 1 {
            let mut newsKind: u8 = (Random() as i32 % 4) as u8 + 1;
            if IsAddingPokeNewsDisallowed(newsKind) != TRUE {
                (*gSaveBlock1Ptr).pokeNews[sCurTVShowSlot].kind = newsKind;
                (*gSaveBlock1Ptr).pokeNews[sCurTVShowSlot].dayCountdown = POKENEWS_COUNTDOWN;
                (*gSaveBlock1Ptr).pokeNews[sCurTVShowSlot].state = POKENEWS_STATE_UPCOMING;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn GetFirstEmptyPokeNewsSlot(pokeNews: *mut PokeNews) -> i8 {
    let mut i: i8 = 0;
    i = 0;
    while i < POKE_NEWS_COUNT as i8 {
        if (*pokeNews.at(i)).kind == POKENEWS_NONE {
            return i;
        }
        i += 1;
    }
    return -1;
}
pub(crate) unsafe extern "C" fn ClearPokeNews() {
    let mut i: u8 = 0;
    i = 0;
    while i < POKE_NEWS_COUNT {
        ClearPokeNewsBySlot(i);
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn ClearPokeNewsBySlot(i: u8) {
    (*gSaveBlock1Ptr).pokeNews[i].kind = POKENEWS_NONE;
    (*gSaveBlock1Ptr).pokeNews[i].state = POKENEWS_STATE_INACTIVE;
    (*gSaveBlock1Ptr).pokeNews[i].dayCountdown = 0;
}
pub(crate) unsafe extern "C" fn CompactPokeNews() {
    let mut i: u8 = 0;
    let mut j: u8 = 0;
    i = 0;
    while i < 15 {
        if (*gSaveBlock1Ptr).pokeNews[i].kind == POKENEWS_NONE {
            j = i + 1;
            while j < POKE_NEWS_COUNT {
                if (*gSaveBlock1Ptr).pokeNews[j].kind != POKENEWS_NONE {
                    (*gSaveBlock1Ptr).pokeNews[i] = (*gSaveBlock1Ptr).pokeNews[j];
                    ClearPokeNewsBySlot(j);
                    break;
                }
                j += 1;
            }
        }
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn FindAnyPokeNewsOnTheAir() -> u8 {
    let mut i: u8 = 0;
    i = 0;
    while i < POKE_NEWS_COUNT {
        if (*gSaveBlock1Ptr).pokeNews[i].kind != POKENEWS_NONE
            && (*gSaveBlock1Ptr).pokeNews[i].state == POKENEWS_STATE_UPCOMING
            && (*gSaveBlock1Ptr).pokeNews[i].dayCountdown < 3
        {
            return i;
        }
        i += 1;
    }
    return 0xFF;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DoPokeNews() {
    let mut i: u8 = FindAnyPokeNewsOnTheAir();
    if i == 0xFF {
        gSpecialVar_Result = FALSE as u16;
    } else {
        if (*gSaveBlock1Ptr).pokeNews[i].dayCountdown == 0 {
            (*gSaveBlock1Ptr).pokeNews[i].state = POKENEWS_STATE_ACTIVE;
            if gLocalTime.hours < 20 {
                ShowFieldMessage(sPokeNewsTextGroup_Ongoing[(*gSaveBlock1Ptr).pokeNews[i].kind]);
            } else {
                ShowFieldMessage(sPokeNewsTextGroup_Ending[(*gSaveBlock1Ptr).pokeNews[i].kind]);
            }
        } else {
            let mut dayCountdown: u16 = (*gSaveBlock1Ptr).pokeNews[i].dayCountdown;
            ConvertIntToDecimalStringN(
                gStringVar1.as_mut_ptr(),
                dayCountdown as i32,
                STR_CONV_MODE_LEFT_ALIGN,
                1,
            );
            (*gSaveBlock1Ptr).pokeNews[i].state = POKENEWS_STATE_INACTIVE;
            ShowFieldMessage(sPokeNewsTextGroup_Upcoming[(*gSaveBlock1Ptr).pokeNews[i].kind]);
        }
        gSpecialVar_Result = TRUE as u16;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsPokeNewsActive(newsKind: u8) -> u8 {
    let mut i: u8 = 0;
    if newsKind == POKENEWS_NONE {
        return FALSE;
    }
    i = 0;
    while i < POKE_NEWS_COUNT {
        if (*gSaveBlock1Ptr).pokeNews[i].kind == newsKind {
            if (*gSaveBlock1Ptr).pokeNews[i].state == POKENEWS_STATE_ACTIVE
                && ShouldApplyPokeNewsEffect(newsKind) != 0
            {
                return TRUE;
            }
            return FALSE;
        }
        i += 1;
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn ShouldApplyPokeNewsEffect(newsKind: u8) -> u8 {
    match newsKind {
        POKENEWS_SLATEPORT => {
            if (*gSaveBlock1Ptr).location.mapGroup == 0
                && (*gSaveBlock1Ptr).location.mapNum == 1
                && gSpecialVar_LastTalked == LOCALID_SLATEPORT_ENERGY_GURU
            {
                return TRUE;
            }
            return FALSE;
        }
        POKENEWS_LILYCOVE => {
            if (*gSaveBlock1Ptr).location.mapGroup == 13 && (*gSaveBlock1Ptr).location.mapNum == 21
            {
                return TRUE;
            }
            return FALSE;
        }
        _ => {}
    }
    return TRUE;
}
pub(crate) unsafe extern "C" fn IsAddingPokeNewsDisallowed(newsKind: u8) -> u8 {
    let mut i: u8 = 0;
    if newsKind == POKENEWS_NONE {
        return TRUE;
    }
    i = 0;
    while i < POKE_NEWS_COUNT {
        if (*gSaveBlock1Ptr).pokeNews[i].kind == newsKind {
            return TRUE;
        }
        i += 1;
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn UpdatePokeNewsCountdown(days: u16) {
    let mut i: u8 = 0;
    i = 0;
    while i < POKE_NEWS_COUNT {
        if (*gSaveBlock1Ptr).pokeNews[i].kind != POKENEWS_NONE {
            if (*gSaveBlock1Ptr).pokeNews[i].dayCountdown < days {
                ClearPokeNewsBySlot(i);
            } else {
                if (*gSaveBlock1Ptr).pokeNews[i].state == POKENEWS_STATE_INACTIVE
                    && FlagGet(FLAG_SYS_GAME_CLEAR) == TRUE
                {
                    (*gSaveBlock1Ptr).pokeNews[i].state = POKENEWS_STATE_UPCOMING;
                }
                (*gSaveBlock1Ptr).pokeNews[i].dayCountdown -= days;
            }
        }
        i += 1;
    }
    CompactPokeNews();
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CopyContestRankToStringVar(varIdx: u8, rank: u8) {
    match rank {
        CONTEST_RANK_NORMAL => {
            StringCopy(gTVStringVarPtrs[varIdx], gStdStrings[5]);
        }
        CONTEST_RANK_SUPER => {
            StringCopy(gTVStringVarPtrs[varIdx], gStdStrings[6]);
        }
        CONTEST_RANK_HYPER => {
            StringCopy(gTVStringVarPtrs[varIdx], gStdStrings[7]);
        }
        CONTEST_RANK_MASTER => {
            StringCopy(gTVStringVarPtrs[varIdx], gStdStrings[8]);
        }
        _ => {}
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CopyContestCategoryToStringVar(varIdx: u8, category: u8) {
    match category {
        CONTEST_CATEGORY_COOL => {
            StringCopy(gTVStringVarPtrs[varIdx], gStdStrings[0]);
        }
        CONTEST_CATEGORY_BEAUTY => {
            StringCopy(gTVStringVarPtrs[varIdx], gStdStrings[1]);
        }
        CONTEST_CATEGORY_CUTE => {
            StringCopy(gTVStringVarPtrs[varIdx], gStdStrings[2]);
        }
        CONTEST_CATEGORY_SMART => {
            StringCopy(gTVStringVarPtrs[varIdx], gStdStrings[3]);
        }
        CONTEST_CATEGORY_TOUGH => {
            StringCopy(gTVStringVarPtrs[varIdx], gStdStrings[4]);
        }
        _ => {}
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetContestCategoryStringVarForInterview() {
    let mut show: *mut TVShow = &raw mut (*gSaveBlock1Ptr).tvShows[gSpecialVar_0x8004];
    CopyContestCategoryToStringVar(1, (*show).bravoTrainer.contestCategory());
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ConvertIntToDecimalString(varIdx: u8, value: i32) {
    let mut nDigits: i32 = CountDigits(value) as i32;
    ConvertIntToDecimalStringN(
        gTVStringVarPtrs[varIdx],
        value,
        STR_CONV_MODE_LEFT_ALIGN,
        nDigits as u8,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CountDigits(value: i32) -> u32 {
    if value / 10 == 0 {
        return 1;
    }
    if value / 100 == 0 {
        return 2;
    }
    if value / 1000 == 0 {
        return 3;
    }
    if value / 10000 == 0 {
        return 4;
    }
    if value / 100000 == 0 {
        return 5;
    }
    if value / 1000000 == 0 {
        return 6;
    }
    if value / 10000000 == 0 {
        return 7;
    }
    if value / 0x5f5e100 == 0 {
        return 8;
    }
    return 1;
}
pub(crate) unsafe extern "C" fn SmartShopper_BufferPurchaseTotal(varIdx: u8, show: *mut TVShow) {
    let mut i: u8 = 0;
    let mut price: i32 = 0;
    i = 0;
    while i < SMARTSHOPPER_NUM_ITEMS {
        if (*show).smartshopperShow.itemIds[i] != ITEM_NONE {
            price += GetItemPrice((*show).smartshopperShow.itemIds[i]) as i32
                * (*show).smartshopperShow.itemAmounts[i] as i32;
        }
        i += 1;
    }
    if (*show).smartshopperShow.priceReduced == TRUE {
        ConvertIntToDecimalString(varIdx, price >> 1);
    } else {
        ConvertIntToDecimalString(varIdx, price);
    }
}
pub(crate) unsafe extern "C" fn IsRecordMixShowAlreadySpawned(kind: u8, delete: u8) -> u8 {
    let mut playerId: u32 = 0;
    let mut shows: *mut TVShow = null_mut();
    let mut i: u8 = 0;
    shows = (*gSaveBlock1Ptr).tvShows.as_mut_ptr();
    playerId = GetPlayerIDAsU32();
    i = NUM_NORMAL_TVSHOW_SLOTS;
    while i < LAST_TVSHOW_IDX {
        if (*shows.at(i)).common.kind == kind
            && playerId & 0xFF == (*shows.at(i)).common.trainerIdLo as u32
            && playerId >> 8 & 0xFF == (*shows.at(i)).common.trainerIdHi as u32
        {
            if delete == TRUE {
                DeleteTVShowInArrayByIdx((*gSaveBlock1Ptr).tvShows.as_mut_ptr(), i);
                CompactTVShowArray((*gSaveBlock1Ptr).tvShows.as_mut_ptr());
            }
            return TRUE;
        }
        i += 1;
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn SortPurchasesByQuantity() {
    let mut i: u8 = 0;
    let mut j: u8 = 0;
    i = 0;
    while i < 2 {
        j = i + 1;
        while j < SMARTSHOPPER_NUM_ITEMS {
            if gMartPurchaseHistory[i].quantity < gMartPurchaseHistory[j].quantity {
                let mut tempItemId: u16 = gMartPurchaseHistory[i].itemId;
                let mut tempQuantity: u16 = gMartPurchaseHistory[i].quantity;
                gMartPurchaseHistory[i].itemId = gMartPurchaseHistory[j].itemId;
                gMartPurchaseHistory[i].quantity = gMartPurchaseHistory[j].quantity;
                gMartPurchaseHistory[j].itemId = tempItemId;
                gMartPurchaseHistory[j].quantity = tempQuantity;
            }
            j += 1;
        }
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn TryReplaceOldTVShowOfKind(kind: u8) {
    let mut i: u8 = 0;
    i = 0;
    while i < NUM_NORMAL_TVSHOW_SLOTS {
        if (*gSaveBlock1Ptr).tvShows[i].common.kind == kind {
            if (*gSaveBlock1Ptr).tvShows[i].common.active == TRUE {
                gSpecialVar_Result = TRUE as u16;
            } else {
                DeleteTVShowInArrayByIdx((*gSaveBlock1Ptr).tvShows.as_mut_ptr(), i);
                CompactTVShowArray((*gSaveBlock1Ptr).tvShows.as_mut_ptr());
                Script_FindFirstEmptyNormalTVShowSlot();
            }
            return;
        }
        i += 1;
    }
    Script_FindFirstEmptyNormalTVShowSlot();
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn InterviewBefore() {
    gSpecialVar_Result = FALSE as u16;
    match gSpecialVar_0x8005 {
        1 => {
            InterviewBefore_FanClubLetter();
        }
        2 => {
            InterviewBefore_RecentHappenings();
        }
        3 => {
            InterviewBefore_PkmnFanClubOpinions();
        }
        4 => {
            InterviewBefore_Dummy();
        }
        5 => {
            InterviewBefore_NameRater();
        }
        6 => {
            InterviewBefore_BravoTrainerPkmnProfile();
        }
        7 => {
            InterviewBefore_BravoTrainerBTProfile();
        }
        8 => {
            InterviewBefore_ContestLiveUpdates();
        }
        9 => {
            InterviewBefore_3CheersForPokeblocks();
        }
        11 => {
            InterviewBefore_FanClubSpecial();
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn InterviewBefore_FanClubLetter() {
    TryReplaceOldTVShowOfKind(TVSHOW_FAN_CLUB_LETTER);
    if gSpecialVar_Result == 0 {
        StringCopy(
            gStringVar1.as_mut_ptr(),
            gSpeciesNames[GetMonData3(
                &raw mut gPlayerParty[GetLeadMonIndex()],
                MON_DATA_SPECIES,
                null_mut(),
            )]
            .as_ptr()
            .cast_mut(),
        );
        InitializeEasyChatWordArray(
            (*gSaveBlock1Ptr).tvShows[sCurTVShowSlot]
                .fanclubLetter
                .words
                .as_mut_ptr(),
            6,
        );
    }
}
pub(crate) unsafe extern "C" fn InterviewBefore_RecentHappenings() {
    TryReplaceOldTVShowOfKind(TVSHOW_RECENT_HAPPENINGS);
    if gSpecialVar_Result == 0 {
        InitializeEasyChatWordArray(
            (*gSaveBlock1Ptr).tvShows[sCurTVShowSlot]
                .recentHappenings
                .words
                .as_mut_ptr(),
            6,
        );
    }
}
pub(crate) unsafe extern "C" fn InterviewBefore_PkmnFanClubOpinions() {
    TryReplaceOldTVShowOfKind(TVSHOW_PKMN_FAN_CLUB_OPINIONS);
    if gSpecialVar_Result == 0 {
        StringCopy(
            gStringVar1.as_mut_ptr(),
            gSpeciesNames[GetMonData3(
                &raw mut gPlayerParty[GetLeadMonIndex()],
                MON_DATA_SPECIES,
                null_mut(),
            )]
            .as_ptr()
            .cast_mut(),
        );
        GetMonData3(
            &raw mut gPlayerParty[GetLeadMonIndex()],
            MON_DATA_NICKNAME,
            gStringVar2.as_mut_ptr(),
        );
        StringGet_Nickname(gStringVar2.as_mut_ptr());
        InitializeEasyChatWordArray(
            (*gSaveBlock1Ptr).tvShows[sCurTVShowSlot]
                .fanclubOpinions
                .words
                .as_mut_ptr(),
            2,
        );
    }
}
pub(crate) unsafe extern "C" fn InterviewBefore_Dummy() {
    gSpecialVar_Result = TRUE as u16;
}
pub(crate) unsafe extern "C" fn InterviewBefore_NameRater() {
    TryReplaceOldTVShowOfKind(TVSHOW_NAME_RATER_SHOW);
}
pub(crate) unsafe extern "C" fn InterviewBefore_BravoTrainerPkmnProfile() {
    TryReplaceOldTVShowOfKind(TVSHOW_BRAVO_TRAINER_POKEMON_PROFILE);
    if gSpecialVar_Result == 0 {
        InitializeEasyChatWordArray(
            (*gSaveBlock1Ptr).tvShows[sCurTVShowSlot]
                .bravoTrainer
                .words
                .as_mut_ptr(),
            2,
        );
    }
}
pub(crate) unsafe extern "C" fn InterviewBefore_ContestLiveUpdates() {
    TryReplaceOldTVShowOfKind(TVSHOW_CONTEST_LIVE_UPDATES);
}
pub(crate) unsafe extern "C" fn InterviewBefore_3CheersForPokeblocks() {
    TryReplaceOldTVShowOfKind(TVSHOW_3_CHEERS_FOR_POKEBLOCKS);
}
pub(crate) unsafe extern "C" fn InterviewBefore_BravoTrainerBTProfile() {
    TryReplaceOldTVShowOfKind(TVSHOW_BRAVO_TRAINER_BATTLE_TOWER_PROFILE);
    if gSpecialVar_Result == 0 {
        InitializeEasyChatWordArray(
            (*gSaveBlock1Ptr).tvShows[sCurTVShowSlot]
                .bravoTrainerTower
                .words
                .as_mut_ptr(),
            1,
        );
    }
}
pub(crate) unsafe extern "C" fn InterviewBefore_FanClubSpecial() {
    TryReplaceOldTVShowOfKind(TVSHOW_FAN_CLUB_SPECIAL);
    if gSpecialVar_Result == 0 {
        InitializeEasyChatWordArray(
            (*gSaveBlock1Ptr).tvShows[sCurTVShowSlot]
                .fanClubSpecial
                .words
                .as_mut_ptr(),
            1,
        );
    }
}
pub(crate) unsafe extern "C" fn IsPartyMonNicknamedOrNotEnglish(monIdx: u8) -> u8 {
    let mut pokemon: *mut Pokemon = null_mut();
    let mut language: u8 = 0;
    pokemon = &raw mut gPlayerParty[monIdx];
    GetMonData3(pokemon, MON_DATA_NICKNAME, gStringVar1.as_mut_ptr());
    language = GetMonData3(pokemon, MON_DATA_LANGUAGE, &raw mut language) as u8;
    if language == GAME_LANGUAGE
        && StringCompare(
            gSpeciesNames[GetMonData3(pokemon, MON_DATA_SPECIES, null_mut())]
                .as_ptr()
                .cast_mut(),
            gStringVar1.as_mut_ptr(),
        ) == 0
    {
        return FALSE;
    }
    return TRUE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsLeadMonNicknamedOrNotEnglish() -> u8 {
    return IsPartyMonNicknamedOrNotEnglish(GetLeadMonIndex());
}
pub(crate) unsafe extern "C" fn DeleteTVShowInArrayByIdx(mut shows: *mut TVShow, idx: u8) {
    let mut i: u8 = 0;
    (*shows.at(idx)).commonInit.kind = TVSHOW_OFF_AIR;
    (*shows.at(idx)).commonInit.active = FALSE;
    i = 0;
    while i < 34 {
        (*shows.at(idx)).commonInit.data[i] = 0;
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn CompactTVShowArray(mut shows: *mut TVShow) {
    let mut i: u8 = 0;
    let mut j: u8 = 0;
    i = 0;
    while i < 4 {
        if (*shows.at(i)).common.kind == TVSHOW_OFF_AIR {
            j = i + 1;
            while j < NUM_NORMAL_TVSHOW_SLOTS {
                if (*shows.at(j)).common.kind != TVSHOW_OFF_AIR {
                    *shows.at(i) = *shows.at(j);
                    DeleteTVShowInArrayByIdx(shows, j);
                    break;
                }
                j += 1;
            }
        }
        i += 1;
    }
    i = NUM_NORMAL_TVSHOW_SLOTS;
    while i < LAST_TVSHOW_IDX {
        if (*shows.at(i)).common.kind == TVSHOW_OFF_AIR {
            j = i + 1;
            while j < LAST_TVSHOW_IDX {
                if (*shows.at(j)).common.kind != TVSHOW_OFF_AIR {
                    *shows.at(i) = *shows.at(j);
                    DeleteTVShowInArrayByIdx(shows, j);
                    break;
                }
                j += 1;
            }
        }
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn GetRandomDifferentSpeciesAndNameSeenByPlayer(
    varIdx: u8,
    excludedSpecies: u16,
) -> u16 {
    let mut species: u16 = GetRandomDifferentSpeciesSeenByPlayer(excludedSpecies);
    StringCopy(
        gTVStringVarPtrs[varIdx],
        gSpeciesNames[species].as_ptr().cast_mut(),
    );
    return species;
}
pub(crate) unsafe extern "C" fn GetRandomDifferentSpeciesSeenByPlayer(excludedSpecies: u16) -> u16 {
    let mut species: u16 = (Random() as i32 % 411) as u16 + 1;
    let mut initSpecies: u16 = species;
    while GetSetPokedexFlag(SpeciesToNationalPokedexNum(species), FLAG_GET_SEEN) != TRUE as i8
        || species == excludedSpecies
    {
        if species == 1 {
            species = 411;
        } else {
            species -= 1;
        }
        if species == initSpecies {
            species = excludedSpecies;
            return species;
        }
    }
    return species;
}
pub(crate) unsafe extern "C" fn Script_FindFirstEmptyNormalTVShowSlot() {
    sCurTVShowSlot = FindFirstEmptyNormalTVShowSlot((*gSaveBlock1Ptr).tvShows.as_mut_ptr());
    gSpecialVar_0x8006 = sCurTVShowSlot as u16;
    if sCurTVShowSlot == -1 {
        gSpecialVar_Result = TRUE as u16;
    } else {
        gSpecialVar_Result = FALSE as u16;
    }
}
pub(crate) unsafe extern "C" fn FindFirstEmptyNormalTVShowSlot(shows: *mut TVShow) -> i8 {
    let mut i: u8 = 0;
    i = 0;
    while i < NUM_NORMAL_TVSHOW_SLOTS {
        if (*shows.at(i)).common.kind == TVSHOW_OFF_AIR {
            return i as i8;
        }
        i += 1;
    }
    return -1;
}
pub(crate) unsafe extern "C" fn FindFirstEmptyRecordMixTVShowSlot(shows: *mut TVShow) -> i8 {
    let mut i: i8 = 0;
    i = NUM_NORMAL_TVSHOW_SLOTS as i8;
    while i < LAST_TVSHOW_IDX as i8 {
        if (*shows.at(i)).common.kind == TVSHOW_OFF_AIR {
            return i;
        }
        i += 1;
    }
    return -1;
}
pub(crate) unsafe extern "C" fn BernoulliTrial(ratio: u16) -> u8 {
    if Random() <= ratio {
        return FALSE;
    }
    return TRUE;
}
pub(crate) unsafe extern "C" fn GetRandomWordFromShow(show: *mut TVShow) {
    let mut i: u8 = 0;
    i = (Random() % 6) as u8;
    loop {
        if i == 6 {
            i = 0;
        }
        if (*show).fanclubLetter.words[i] != EC_EMPTY_WORD {
            break;
        }
        i += 1;
    }
    CopyEasyChatWord(gStringVar3.as_mut_ptr(), (*show).fanclubLetter.words[i]);
}
pub(crate) unsafe extern "C" fn GetRandomNameRaterStateFromName(show: *mut TVShow) -> u8 {
    let mut i: u8 = 0;
    let mut nameSum: u16 = 0;
    nameSum = 0;
    i = 0;
    while i < 11 {
        if (*show).nameRaterShow.pokemonName[i] == EOS {
            break;
        }
        nameSum += (*show).nameRaterShow.pokemonName[i] as u16;
        i += 1;
    }
    return nameSum as u8 & 7;
}
pub(crate) unsafe extern "C" fn GetNicknameSubstring(
    varIdx: u8,
    whichPosition: u8,
    charParam: u8,
    whichString: u16,
    species: u16,
    show: *mut TVShow,
) {
    let mut buff: CArray<u8, 16> = zeroed();
    let mut i: u8 = 0;
    let mut strlen: u16 = 0;
    i = 0;
    while i < 3 {
        buff[i] = EOS;
        i += 1;
    }
    if whichString == 0 {
        strlen = StringLength((*show).nameRaterShow.trainerName.as_mut_ptr());
        if charParam == 0 {
            buff[0] = (*show).nameRaterShow.trainerName[whichPosition];
        } else if charParam == 1 {
            buff[0] = (*show).nameRaterShow.trainerName[strlen as i32 - whichPosition as i32];
        } else if charParam == 2 {
            buff[0] = (*show).nameRaterShow.trainerName[whichPosition];
            buff[1] = (*show).nameRaterShow.trainerName[whichPosition as i32 + 1];
        } else {
            buff[0] = (*show).nameRaterShow.trainerName[strlen as i32 - (whichPosition as i32 + 2)];
            buff[1] = (*show).nameRaterShow.trainerName[strlen as i32 - (whichPosition as i32 + 1)];
        }
        ConvertInternationalString(buff.as_mut_ptr(), (*show).nameRaterShow.language);
    } else if whichString == 1 {
        strlen = StringLength((*show).nameRaterShow.pokemonName.as_mut_ptr());
        if charParam == 0 {
            buff[0] = (*show).nameRaterShow.pokemonName[whichPosition];
        } else if charParam == 1 {
            buff[0] = (*show).nameRaterShow.pokemonName[strlen as i32 - whichPosition as i32];
        } else if charParam == 2 {
            buff[0] = (*show).nameRaterShow.pokemonName[whichPosition];
            buff[1] = (*show).nameRaterShow.pokemonName[whichPosition as i32 + 1];
        } else {
            buff[0] = (*show).nameRaterShow.pokemonName[strlen as i32 - (whichPosition as i32 + 2)];
            buff[1] = (*show).nameRaterShow.pokemonName[strlen as i32 - (whichPosition as i32 + 1)];
        }
        ConvertInternationalString(buff.as_mut_ptr(), (*show).nameRaterShow.pokemonNameLanguage);
    } else {
        strlen = StringLength(gSpeciesNames[species].as_ptr().cast_mut());
        if charParam == 0 {
            buff[0] = gSpeciesNames[species][whichPosition];
        } else if charParam == 1 {
            buff[0] = gSpeciesNames[species][strlen as i32 - whichPosition as i32];
        } else if charParam == 2 {
            buff[0] = gSpeciesNames[species][whichPosition];
            buff[1] = gSpeciesNames[species][whichPosition as i32 + 1];
        } else {
            buff[0] = gSpeciesNames[species][strlen as i32 - (whichPosition as i32 + 2)];
            buff[1] = gSpeciesNames[species][strlen as i32 - (whichPosition as i32 + 1)];
        }
    }
    StringCopy(gTVStringVarPtrs[varIdx], buff.as_mut_ptr());
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsTVShowAlreadyInQueue() -> u8 {
    let mut i: u8 = 0;
    i = 0;
    while i < NUM_NORMAL_TVSHOW_SLOTS {
        if (*gSaveBlock1Ptr).tvShows[i].common.kind as u16 == gSpecialVar_0x8004 {
            return TRUE;
        }
        i += 1;
    }
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn TryPutNameRaterShowOnTheAir() -> u8 {
    GetMonData3(
        &raw mut gPlayerParty[gSpecialVar_0x8004],
        MON_DATA_NICKNAME,
        gStringVar1.as_mut_ptr(),
    );
    if StringCompare(gStringVar3.as_mut_ptr(), gStringVar1.as_mut_ptr()) == 0 {
        return FALSE;
    }
    PutNameRaterShowOnTheAir();
    return TRUE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ChangePokemonNickname() {
    GetMonData3(
        &raw mut gPlayerParty[gSpecialVar_0x8004],
        MON_DATA_NICKNAME,
        gStringVar3.as_mut_ptr(),
    );
    GetMonData3(
        &raw mut gPlayerParty[gSpecialVar_0x8004],
        MON_DATA_NICKNAME,
        gStringVar2.as_mut_ptr(),
    );
    DoNamingScreen(
        NAMING_SCREEN_NICKNAME,
        gStringVar2.as_mut_ptr(),
        GetMonData3(
            &raw mut gPlayerParty[gSpecialVar_0x8004],
            MON_DATA_SPECIES,
            null_mut(),
        ) as u16,
        GetMonGender(&raw mut gPlayerParty[gSpecialVar_0x8004]) as u16,
        GetMonData3(
            &raw mut gPlayerParty[gSpecialVar_0x8004],
            MON_DATA_PERSONALITY,
            null_mut(),
        ),
        Some(ChangePokemonNickname_CB),
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ChangePokemonNickname_CB() {
    SetMonData(
        &raw mut gPlayerParty[gSpecialVar_0x8004],
        MON_DATA_NICKNAME,
        gStringVar2.as_mut_ptr() as *mut c_void,
    );
    CB2_ReturnToFieldContinueScriptPlayMapMusic();
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ChangeBoxPokemonNickname() {
    let mut boxMon: *mut BoxPokemon = null_mut();
    boxMon = GetBoxedMonPtr(gSpecialVar_MonBoxId as u8, gSpecialVar_MonBoxPos as u8);
    GetBoxMonData3(boxMon, MON_DATA_NICKNAME, gStringVar3.as_mut_ptr());
    GetBoxMonData3(boxMon, MON_DATA_NICKNAME, gStringVar2.as_mut_ptr());
    DoNamingScreen(
        NAMING_SCREEN_NICKNAME,
        gStringVar2.as_mut_ptr(),
        GetBoxMonData3(boxMon, MON_DATA_SPECIES, null_mut()) as u16,
        GetBoxMonGender(boxMon) as u16,
        GetBoxMonData3(boxMon, MON_DATA_PERSONALITY, null_mut()),
        Some(ChangeBoxPokemonNickname_CB),
    );
}
pub(crate) unsafe extern "C" fn ChangeBoxPokemonNickname_CB() {
    SetBoxMonNickAt(
        gSpecialVar_MonBoxId as u8,
        gSpecialVar_MonBoxPos as u8,
        gStringVar2.as_mut_ptr(),
    );
    CB2_ReturnToFieldContinueScriptPlayMapMusic();
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BufferMonNickname() {
    GetMonData3(
        &raw mut gPlayerParty[gSpecialVar_0x8004],
        MON_DATA_NICKNAME,
        gStringVar1.as_mut_ptr(),
    );
    StringGet_Nickname(gStringVar1.as_mut_ptr());
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsMonOTIDNotPlayers() {
    if GetPlayerIDAsU32()
        == GetMonData3(
            &raw mut gPlayerParty[gSpecialVar_0x8004],
            MON_DATA_OT_ID,
            null_mut(),
        )
    {
        gSpecialVar_Result = FALSE as u16;
    } else {
        gSpecialVar_Result = TRUE as u16;
    }
}
pub(crate) unsafe extern "C" fn GetTVGroupByShowId(kind: u8) -> u8 {
    if kind == TVSHOW_OFF_AIR {
        return TVGROUP_NONE;
    }
    if kind >= TVGROUP_NORMAL_START && kind <= TVGROUP_NORMAL_END {
        return TVGROUP_NORMAL;
    }
    if kind >= TVGROUP_RECORD_MIX_START && kind <= TVGROUP_RECORD_MIX_END {
        return TVGROUP_RECORD_MIX;
    }
    if kind >= TVGROUP_OUTBREAK_START && kind <= TVGROUP_OUTBREAK_END {
        return TVGROUP_OUTBREAK;
    }
    return TVGROUP_NONE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetPlayerIDAsU32() -> u32 {
    return ((*gSaveBlock2Ptr).playerTrainerId[3] as u32) << 24
        | ((*gSaveBlock2Ptr).playerTrainerId[2] as u32) << 16
        | ((*gSaveBlock2Ptr).playerTrainerId[1] as u32) << 8
        | (*gSaveBlock2Ptr).playerTrainerId[0] as u32;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CheckForPlayersHouseNews() -> u8 {
    if (*gSaveBlock1Ptr).location.mapGroup != 1 {
        return PLAYERS_HOUSE_TV_NONE;
    }
    if (*gSaveBlock2Ptr).playerGender == MALE {
        if (*gSaveBlock1Ptr).location.mapNum != 0 {
            return PLAYERS_HOUSE_TV_NONE;
        }
    } else {
        if (*gSaveBlock1Ptr).location.mapNum != 2 {
            return PLAYERS_HOUSE_TV_NONE;
        }
    }
    if FlagGet(FLAG_SYS_TV_LATIAS_LATIOS) == TRUE {
        return PLAYERS_HOUSE_TV_LATI;
    }
    if FlagGet(FLAG_SYS_TV_HOME) == TRUE {
        return PLAYERS_HOUSE_TV_MOVIE;
    }
    return PLAYERS_HOUSE_TV_LATI;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetMomOrDadStringForTVMessage() {
    if (*gSaveBlock1Ptr).location.mapGroup == 1 {
        if (*gSaveBlock2Ptr).playerGender == MALE {
            if (*gSaveBlock1Ptr).location.mapNum == 0 {
                StringCopy(gStringVar1.as_mut_ptr(), gText_Mom.as_ptr().cast_mut());
                VarSet(VAR_TEMP_3, 1);
            }
        } else {
            if (*gSaveBlock1Ptr).location.mapNum == 2 {
                StringCopy(gStringVar1.as_mut_ptr(), gText_Mom.as_ptr().cast_mut());
                VarSet(VAR_TEMP_3, 1);
            }
        }
    }
    if VarGet(VAR_TEMP_3) == 1 {
        StringCopy(gStringVar1.as_mut_ptr(), gText_Mom.as_ptr().cast_mut());
    } else if VarGet(VAR_TEMP_3) == 2 {
        StringCopy(gStringVar1.as_mut_ptr(), gText_Dad.as_ptr().cast_mut());
    } else if VarGet(VAR_TEMP_3) > 2 {
        if VarGet(VAR_TEMP_3) as i32 % 2 == 0 {
            StringCopy(gStringVar1.as_mut_ptr(), gText_Mom.as_ptr().cast_mut());
        } else {
            StringCopy(gStringVar1.as_mut_ptr(), gText_Dad.as_ptr().cast_mut());
        }
    } else {
        if Random() as i32 % 2 != 0 {
            StringCopy(gStringVar1.as_mut_ptr(), gText_Mom.as_ptr().cast_mut());
            VarSet(VAR_TEMP_3, 1);
        } else {
            StringCopy(gStringVar1.as_mut_ptr(), gText_Dad.as_ptr().cast_mut());
            VarSet(VAR_TEMP_3, 2);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn HideBattleTowerReporter() {
    VarSet(VAR_BRAVO_TRAINER_BATTLE_TOWER_ON, 0);
    RemoveObjectEventByLocalIdAndMap(
        LOCALID_TOWER_LOBBY_REPORTER,
        (*gSaveBlock1Ptr).location.mapNum as u8,
        (*gSaveBlock1Ptr).location.mapGroup as u8,
    );
    FlagSet(FLAG_HIDE_BATTLE_TOWER_REPORTER);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ReceiveTvShowsData(src: *mut c_void, size: u32, playersLinkId: u8) {
    let mut i: u8 = 0;
    let mut version: u16 = 0;
    let mut rmBuffer2: *mut CArray<CArray<TVShow, 25>, 4> = null_mut();
    let mut rmBuffer: *mut CArray<CArray<TVShow, 25>, 4> = null_mut();
    rmBuffer2 = Alloc(3600) as *mut CArray<CArray<TVShow, 25>, 4>;
    if !rmBuffer2.is_null() {
        i = 0;
        while i < MAX_LINK_PLAYERS as u8 {
            memcpy(
                (*rmBuffer2)[i].as_mut_ptr() as *mut u8,
                (src as *mut u8).at(i as u32 * size) as *mut c_void as *mut u8,
                900,
            );
            i += 1;
        }
        rmBuffer = rmBuffer2;
        i = 0;
        while i < GetLinkPlayerCount() {
            version = gLinkPlayers[i].version as u8 as u16;
            if version == VERSION_RUBY as u16 || version == VERSION_SAPPHIRE as u16 {
                TranslateRubyShows((*rmBuffer)[i].as_mut_ptr());
            } else if version == VERSION_EMERALD as u16
                && gLinkPlayers[i].language == LANGUAGE_JAPANESE as u16
            {
                TranslateJapaneseEmeraldShows((*rmBuffer)[i].as_mut_ptr());
            }
            i += 1;
        }
        match playersLinkId {
            0 => {
                SetMixedTVShows(
                    (*gSaveBlock1Ptr).tvShows.as_mut_ptr(),
                    (*rmBuffer)[1].as_mut_ptr(),
                    (*rmBuffer)[2].as_mut_ptr(),
                    (*rmBuffer)[3].as_mut_ptr(),
                );
            }
            1 => {
                SetMixedTVShows(
                    (*rmBuffer)[0].as_mut_ptr(),
                    (*gSaveBlock1Ptr).tvShows.as_mut_ptr(),
                    (*rmBuffer)[2].as_mut_ptr(),
                    (*rmBuffer)[3].as_mut_ptr(),
                );
            }
            2 => {
                SetMixedTVShows(
                    (*rmBuffer)[0].as_mut_ptr(),
                    (*rmBuffer)[1].as_mut_ptr(),
                    (*gSaveBlock1Ptr).tvShows.as_mut_ptr(),
                    (*rmBuffer)[3].as_mut_ptr(),
                );
            }
            3 => {
                SetMixedTVShows(
                    (*rmBuffer)[0].as_mut_ptr(),
                    (*rmBuffer)[1].as_mut_ptr(),
                    (*rmBuffer)[2].as_mut_ptr(),
                    (*gSaveBlock1Ptr).tvShows.as_mut_ptr(),
                );
            }
            _ => {}
        }
        CompactTVShowArray((*gSaveBlock1Ptr).tvShows.as_mut_ptr());
        DeleteExcessMixedShows();
        CompactTVShowArray((*gSaveBlock1Ptr).tvShows.as_mut_ptr());
        DeactivateShowsWithUnseenSpecies();
        DeactivateGameCompleteShowsIfNotUnlocked();
        Free(rmBuffer2 as *mut c_void);
    }
}
pub(crate) unsafe extern "C" fn SetMixedTVShows(
    mut player1: *mut TVShow,
    mut player2: *mut TVShow,
    mut player3: *mut TVShow,
    mut player4: *mut TVShow,
) {
    let mut i: u8 = 0;
    let mut j: u8 = 0;
    let mut tvShows: CArray<*mut *mut TVShow, 4> = zeroed();
    tvShows[0] = &raw mut player1;
    tvShows[1] = &raw mut player2;
    tvShows[2] = &raw mut player3;
    tvShows[3] = &raw mut player4;
    sTVShowMixingNumPlayers = GetLinkPlayerCount();
    loop {
        i = 0;
        while i < sTVShowMixingNumPlayers {
            if i == 0 {
                sRecordMixingPartnersWithoutShowsToShare = 0;
            }
            sTVShowMixingCurSlot = FindInactiveShowInArray(*tvShows[i]);
            if sTVShowMixingCurSlot == -1 {
                sRecordMixingPartnersWithoutShowsToShare += 1;
                if sRecordMixingPartnersWithoutShowsToShare == sTVShowMixingNumPlayers {
                    return;
                }
            } else {
                j = 0;
                while (j as i32) < sTVShowMixingNumPlayers as i32 - 1 {
                    sCurTVShowSlot = FindFirstEmptyRecordMixTVShowSlot(
                        *tvShows[rem_i32(i as i32 + j as i32 + 1, sTVShowMixingNumPlayers as i32)],
                    );
                    if sCurTVShowSlot != -1
                        && TryMixTVShow(
                            tvShows
                                [rem_i32(i as i32 + j as i32 + 1, sTVShowMixingNumPlayers as i32)],
                            tvShows[i],
                            rem_i32(i as i32 + j as i32 + 1, sTVShowMixingNumPlayers as i32) as u8,
                        ) == 1
                    {
                        break;
                    }
                    j += 1;
                }
                if j as i32 == sTVShowMixingNumPlayers as i32 - 1 {
                    DeleteTVShowInArrayByIdx(*tvShows[i], sTVShowMixingCurSlot as u8);
                }
            }
            i += 1;
        }
    }
}
pub(crate) unsafe extern "C" fn TryMixTVShow(
    dest: *mut *mut TVShow,
    src: *mut *mut TVShow,
    idx: u8,
) -> u8 {
    let mut success: u8 = 0;
    let mut r#type: u8 = 0;
    let mut tv1: *mut TVShow = *dest;
    let mut tv2: *mut TVShow = *src;
    success = FALSE;
    r#type = GetTVGroupByShowId((*tv2.at(sTVShowMixingCurSlot)).common.kind);
    match r#type {
        TVGROUP_NORMAL => {
            success = TryMixNormalTVShow(tv1.at(sCurTVShowSlot), tv2.at(sTVShowMixingCurSlot), idx);
        }
        TVGROUP_RECORD_MIX => {
            success =
                TryMixRecordMixTVShow(tv1.at(sCurTVShowSlot), tv2.at(sTVShowMixingCurSlot), idx);
        }
        TVGROUP_OUTBREAK => {
            success =
                TryMixOutbreakTVShow(tv1.at(sCurTVShowSlot), tv2.at(sTVShowMixingCurSlot), idx);
        }
        _ => {}
    }
    if success == TRUE {
        DeleteTVShowInArrayByIdx(tv2, sTVShowMixingCurSlot as u8);
        return TRUE;
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn TryMixNormalTVShow(
    dest: *mut TVShow,
    src: *mut TVShow,
    idx: u8,
) -> u8 {
    let mut linkTrainerId: u32 = GetLinkPlayerTrainerId(idx);
    if linkTrainerId & 0xFF == (*src).common.trainerIdLo as u32
        && linkTrainerId >> 8 & 0xFF == (*src).common.trainerIdHi as u32
    {
        return FALSE;
    }
    (*src).common.trainerIdLo = (*src).common.srcTrainerIdLo;
    (*src).common.trainerIdHi = (*src).common.srcTrainerIdHi;
    (*src).common.srcTrainerIdLo = linkTrainerId as u8 & 0xFF;
    (*src).common.srcTrainerIdHi = (linkTrainerId >> 8) as u8;
    *dest = *src;
    (*dest).common.active = TRUE;
    return TRUE;
}
pub(crate) unsafe extern "C" fn TryMixRecordMixTVShow(
    dest: *mut TVShow,
    src: *mut TVShow,
    idx: u8,
) -> u8 {
    let mut linkTrainerId: u32 = GetLinkPlayerTrainerId(idx);
    if linkTrainerId & 0xFF == (*src).common.srcTrainerIdLo as u32
        && linkTrainerId >> 8 & 0xFF == (*src).common.srcTrainerIdHi as u32
    {
        return FALSE;
    }
    if linkTrainerId & 0xFF == (*src).common.trainerIdLo as u32
        && linkTrainerId >> 8 & 0xFF == (*src).common.trainerIdHi as u32
    {
        return FALSE;
    }
    (*src).common.srcTrainerIdLo = (*src).common.srcTrainerId2Lo;
    (*src).common.srcTrainerIdHi = (*src).common.srcTrainerId2Hi;
    (*src).common.srcTrainerId2Lo = linkTrainerId as u8 & 0xFF;
    (*src).common.srcTrainerId2Hi = (linkTrainerId >> 8) as u8;
    *dest = *src;
    (*dest).common.active = TRUE;
    return TRUE;
}
pub(crate) unsafe extern "C" fn TryMixOutbreakTVShow(
    dest: *mut TVShow,
    src: *mut TVShow,
    idx: u8,
) -> u8 {
    let mut linkTrainerId: u32 = GetLinkPlayerTrainerId(idx);
    if linkTrainerId & 0xFF == (*src).common.trainerIdLo as u32
        && linkTrainerId >> 8 & 0xFF == (*src).common.trainerIdHi as u32
    {
        return FALSE;
    }
    (*src).common.trainerIdLo = (*src).common.srcTrainerIdLo;
    (*src).common.trainerIdHi = (*src).common.srcTrainerIdHi;
    (*src).common.srcTrainerIdLo = linkTrainerId as u8 & 0xFF;
    (*src).common.srcTrainerIdHi = (linkTrainerId >> 8) as u8;
    *dest = *src;
    (*dest).common.active = TRUE;
    (*dest).massOutbreak.daysBeforeOutbreak = 1;
    return TRUE;
}
pub(crate) unsafe extern "C" fn FindInactiveShowInArray(tvShows: *mut TVShow) -> i8 {
    let mut i: u8 = 0;
    i = 0;
    while i < LAST_TVSHOW_IDX {
        if (*tvShows.at(i)).common.active == FALSE
            && (*tvShows.at(i)).common.kind as i32 - 1 < TVGROUP_OUTBREAK_END as i32
        {
            return i as i8;
        }
        i += 1;
    }
    return -1;
}
pub(crate) unsafe extern "C" fn DeactivateShowsWithUnseenSpecies() {
    let mut i: u16 = 0;
    let mut species: u16 = 0;
    i = 0;
    while i < LAST_TVSHOW_IDX as u16 {
        match (*gSaveBlock1Ptr).tvShows[i].common.kind {
            TVSHOW_CONTEST_LIVE_UPDATES => {
                species = (*(&raw mut (*gSaveBlock1Ptr).tvShows[i]))
                    .contestLiveUpdates
                    .winningSpecies;
                DeactivateShowIfNotSeenSpecies(species, i as u8);
                species = (*(&raw mut (*gSaveBlock1Ptr).tvShows[i]))
                    .contestLiveUpdates
                    .losingSpecies;
                DeactivateShowIfNotSeenSpecies(species, i as u8);
            }
            TVSHOW_BATTLE_UPDATE => {
                species = (*(&raw mut (*gSaveBlock1Ptr).tvShows[i]))
                    .battleUpdate
                    .speciesPlayer;
                DeactivateShowIfNotSeenSpecies(species, i as u8);
                species = (*(&raw mut (*gSaveBlock1Ptr).tvShows[i]))
                    .battleUpdate
                    .speciesOpponent;
                DeactivateShowIfNotSeenSpecies(species, i as u8);
            }
            TVSHOW_FAN_CLUB_LETTER => {
                species = (*(&raw mut (*gSaveBlock1Ptr).tvShows[i]))
                    .fanclubLetter
                    .species;
                DeactivateShowIfNotSeenSpecies(species, i as u8);
            }
            TVSHOW_PKMN_FAN_CLUB_OPINIONS => {
                species = (*(&raw mut (*gSaveBlock1Ptr).tvShows[i]))
                    .fanclubOpinions
                    .species;
                DeactivateShowIfNotSeenSpecies(species, i as u8);
            }
            TVSHOW_DUMMY => {
                species = (*(&raw mut (*gSaveBlock1Ptr).tvShows[i])).dummy.species;
                DeactivateShowIfNotSeenSpecies(species, i as u8);
            }
            TVSHOW_NAME_RATER_SHOW => {
                species = (*(&raw mut (*gSaveBlock1Ptr).tvShows[i]))
                    .nameRaterShow
                    .species;
                DeactivateShowIfNotSeenSpecies(species, i as u8);
                species = (*(&raw mut (*gSaveBlock1Ptr).tvShows[i]))
                    .nameRaterShow
                    .randomSpecies;
                DeactivateShowIfNotSeenSpecies(species, i as u8);
            }
            TVSHOW_BRAVO_TRAINER_POKEMON_PROFILE => {
                species = (*(&raw mut (*gSaveBlock1Ptr).tvShows[i]))
                    .bravoTrainer
                    .species;
                DeactivateShowIfNotSeenSpecies(species, i as u8);
            }
            TVSHOW_BRAVO_TRAINER_BATTLE_TOWER_PROFILE => {
                species = (*(&raw mut (*gSaveBlock1Ptr).tvShows[i]))
                    .bravoTrainerTower
                    .species;
                DeactivateShowIfNotSeenSpecies(species, i as u8);
                species = (*(&raw mut (*gSaveBlock1Ptr).tvShows[i]))
                    .bravoTrainerTower
                    .defeatedSpecies;
                DeactivateShowIfNotSeenSpecies(species, i as u8);
            }
            TVSHOW_POKEMON_TODAY_CAUGHT => {
                species = (*(&raw mut (*gSaveBlock1Ptr).tvShows[i]))
                    .pokemonToday
                    .species;
                DeactivateShowIfNotSeenSpecies(species, i as u8);
            }
            TVSHOW_POKEMON_TODAY_FAILED => {
                species = (*(&raw mut (*gSaveBlock1Ptr).tvShows[i]))
                    .pokemonTodayFailed
                    .species;
                DeactivateShowIfNotSeenSpecies(species, i as u8);
                species = (*(&raw mut (*gSaveBlock1Ptr).tvShows[i]))
                    .pokemonTodayFailed
                    .species2;
                DeactivateShowIfNotSeenSpecies(species, i as u8);
            }
            TVSHOW_FISHING_ADVICE => {
                species = (*(&raw mut (*gSaveBlock1Ptr).tvShows[i]))
                    .pokemonAngler
                    .species;
                DeactivateShowIfNotSeenSpecies(species, i as u8);
            }
            TVSHOW_WORLD_OF_MASTERS => {
                species = (*(&raw mut (*gSaveBlock1Ptr).tvShows[i]))
                    .worldOfMasters
                    .species;
                DeactivateShowIfNotSeenSpecies(species, i as u8);
                species = (*(&raw mut (*gSaveBlock1Ptr).tvShows[i]))
                    .worldOfMasters
                    .caughtPoke;
                DeactivateShowIfNotSeenSpecies(species, i as u8);
            }
            TVSHOW_BREAKING_NEWS => {
                species = (*(&raw mut (*gSaveBlock1Ptr).tvShows[i]))
                    .breakingNews
                    .lastOpponentSpecies;
                DeactivateShowIfNotSeenSpecies(species, i as u8);
                species = (*(&raw mut (*gSaveBlock1Ptr).tvShows[i]))
                    .breakingNews
                    .poke1Species;
                DeactivateShowIfNotSeenSpecies(species, i as u8);
            }
            TVSHOW_SECRET_BASE_VISIT => {
                species = (*(&raw mut (*gSaveBlock1Ptr).tvShows[i]))
                    .secretBaseVisit
                    .species;
                DeactivateShowIfNotSeenSpecies(species, i as u8);
            }
            TVSHOW_BATTLE_SEMINAR => {
                species = (*(&raw mut (*gSaveBlock1Ptr).tvShows[i]))
                    .battleSeminar
                    .species;
                DeactivateShowIfNotSeenSpecies(species, i as u8);
                species = (*(&raw mut (*gSaveBlock1Ptr).tvShows[i]))
                    .battleSeminar
                    .foeSpecies;
                DeactivateShowIfNotSeenSpecies(species, i as u8);
            }
            TVSHOW_FRONTIER => {
                species = (*(&raw mut (*gSaveBlock1Ptr).tvShows[i])).frontier.species1;
                DeactivateShowIfNotSeenSpecies(species, i as u8);
                species = (*(&raw mut (*gSaveBlock1Ptr).tvShows[i])).frontier.species2;
                DeactivateShowIfNotSeenSpecies(species, i as u8);
                species = (*(&raw mut (*gSaveBlock1Ptr).tvShows[i]))
                    .frontier
                    .facilityAndMode as u16;
                match species {
                    3 | 4 => {}
                    1 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 => {
                        species = (*(&raw mut (*gSaveBlock1Ptr).tvShows[i])).frontier.species3;
                        DeactivateShowIfNotSeenSpecies(species, i as u8);
                    }
                    2 => {
                        species = (*(&raw mut (*gSaveBlock1Ptr).tvShows[i])).frontier.species3;
                        DeactivateShowIfNotSeenSpecies(species, i as u8);
                        species = (*(&raw mut (*gSaveBlock1Ptr).tvShows[i])).frontier.species4;
                        DeactivateShowIfNotSeenSpecies(species, i as u8);
                    }
                    _ => {}
                }
            }
            TVSHOW_OFF_AIR
            | TVSHOW_RECENT_HAPPENINGS
            | TVSHOW_3_CHEERS_FOR_POKEBLOCKS
            | TVSHOW_TODAYS_RIVAL_TRAINER
            | TVSHOW_TREND_WATCHER
            | TVSHOW_TREASURE_INVESTIGATORS
            | TVSHOW_FIND_THAT_GAMER
            | TVSHOW_TRAINER_FAN_CLUB
            | TVSHOW_CUTIES
            | TVSHOW_SMART_SHOPPER
            | TVSHOW_FAN_CLUB_SPECIAL
            | TVSHOW_LILYCOVE_CONTEST_LADY
            | TVSHOW_LOTTO_WINNER
            | TVSHOW_NUMBER_ONE
            | TVSHOW_SECRET_BASE_SECRETS
            | TVSHOW_SAFARI_FAN_CLUB
            | TVSHOW_MASS_OUTBREAK => {}
            _ => {
                DeactivateShow(i as u8);
            }
        }
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn DeactivateShow(showIdx: u8) {
    (*gSaveBlock1Ptr).tvShows[showIdx].common.active = FALSE;
}
pub(crate) unsafe extern "C" fn DeactivateShowIfNotSeenSpecies(species: u16, showIdx: u8) {
    if GetSetPokedexFlag(SpeciesToNationalPokedexNum(species), FLAG_GET_SEEN) == 0 {
        (*gSaveBlock1Ptr).tvShows[showIdx].common.active = FALSE;
    }
}
pub(crate) unsafe extern "C" fn DeactivateGameCompleteShowsIfNotUnlocked() {
    let mut i: u16 = 0;
    if FlagGet(FLAG_SYS_GAME_CLEAR) != TRUE {
        i = 0;
        while i < LAST_TVSHOW_IDX as u16 {
            if (*gSaveBlock1Ptr).tvShows[i].common.kind == TVSHOW_BRAVO_TRAINER_BATTLE_TOWER_PROFILE
            {
                (*gSaveBlock1Ptr).tvShows[i].common.active = FALSE;
            } else if (*gSaveBlock1Ptr).tvShows[i].common.kind == TVSHOW_MASS_OUTBREAK {
                (*gSaveBlock1Ptr).tvShows[i].common.active = FALSE;
            }
            i += 1;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DeactivateAllNormalTVShows() {
    let mut i: u8 = 0;
    i = 0;
    while i < NUM_NORMAL_TVSHOW_SLOTS {
        if GetTVGroupByShowId((*gSaveBlock1Ptr).tvShows[i].common.kind) == TVGROUP_NORMAL {
            (*gSaveBlock1Ptr).tvShows[i].common.active = FALSE;
        }
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn DeleteExcessMixedShows() {
    let mut i: i8 = 0;
    let mut numEmptyMixSlots: i8 = 0;
    i = NUM_NORMAL_TVSHOW_SLOTS as i8;
    while i < LAST_TVSHOW_IDX as i8 {
        if (*gSaveBlock1Ptr).tvShows[i].common.kind == TVSHOW_OFF_AIR {
            numEmptyMixSlots += 1;
        }
        i += 1;
    }
    i = 0;
    while (i as i32) < NUM_NORMAL_TVSHOW_SLOTS as i32 - numEmptyMixSlots as i32 {
        DeleteTVShowInArrayByIdx(
            (*gSaveBlock1Ptr).tvShows.as_mut_ptr(),
            i as u8 + NUM_NORMAL_TVSHOW_SLOTS,
        );
        i += 1;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ReceivePokeNewsData(src: *mut c_void, size: u32, playersLinkId: u8) {
    let mut i: u8 = 0;
    let mut rmBuffer2: *mut CArray<CArray<PokeNews, 16>, 4> = null_mut();
    let mut rmBuffer: *mut CArray<CArray<PokeNews, 16>, 4> = null_mut();
    rmBuffer2 = Alloc(256) as *mut CArray<CArray<PokeNews, 16>, 4>;
    if !rmBuffer2.is_null() {
        i = 0;
        while i < MAX_LINK_PLAYERS as u8 {
            memcpy(
                (*rmBuffer2)[i].as_mut_ptr() as *mut u8,
                (src as *mut u8).at(i as u32 * size) as *mut c_void as *mut u8,
                64,
            );
            i += 1;
        }
        rmBuffer = rmBuffer2;
        match playersLinkId {
            0 => {
                SetMixedPokeNews(
                    (*gSaveBlock1Ptr).pokeNews.as_mut_ptr(),
                    (*rmBuffer)[1].as_mut_ptr(),
                    (*rmBuffer)[2].as_mut_ptr(),
                    (*rmBuffer)[3].as_mut_ptr(),
                );
            }
            1 => {
                SetMixedPokeNews(
                    (*rmBuffer)[0].as_mut_ptr(),
                    (*gSaveBlock1Ptr).pokeNews.as_mut_ptr(),
                    (*rmBuffer)[2].as_mut_ptr(),
                    (*rmBuffer)[3].as_mut_ptr(),
                );
            }
            2 => {
                SetMixedPokeNews(
                    (*rmBuffer)[0].as_mut_ptr(),
                    (*rmBuffer)[1].as_mut_ptr(),
                    (*gSaveBlock1Ptr).pokeNews.as_mut_ptr(),
                    (*rmBuffer)[3].as_mut_ptr(),
                );
            }
            3 => {
                SetMixedPokeNews(
                    (*rmBuffer)[0].as_mut_ptr(),
                    (*rmBuffer)[1].as_mut_ptr(),
                    (*rmBuffer)[2].as_mut_ptr(),
                    (*gSaveBlock1Ptr).pokeNews.as_mut_ptr(),
                );
            }
            _ => {}
        }
        ClearInvalidPokeNews();
        ClearPokeNewsIfGameNotComplete();
        Free(rmBuffer2 as *mut c_void);
    }
}
pub(crate) unsafe extern "C" fn SetMixedPokeNews(
    mut player1: *mut PokeNews,
    mut player2: *mut PokeNews,
    mut player3: *mut PokeNews,
    mut player4: *mut PokeNews,
) {
    let mut i: u8 = 0;
    let mut j: u8 = 0;
    let mut k: u8 = 0;
    let mut pokeNews: CArray<*mut *mut PokeNews, 4> = zeroed();
    pokeNews[0] = &raw mut player1;
    pokeNews[1] = &raw mut player2;
    pokeNews[2] = &raw mut player3;
    pokeNews[3] = &raw mut player4;
    sTVShowNewsMixingNumPlayers = GetLinkPlayerCount();
    i = 0;
    while i < POKE_NEWS_COUNT {
        j = 0;
        while j < sTVShowNewsMixingNumPlayers {
            sTVShowMixingCurSlot = GetPokeNewsSlotIfActive(*pokeNews[j], i);
            if sTVShowMixingCurSlot != -1 {
                k = 0;
                while (k as i32) < sTVShowNewsMixingNumPlayers as i32 - 1 {
                    sCurTVShowSlot = GetFirstEmptyPokeNewsSlot(
                        *pokeNews
                            [rem_i32(j as i32 + k as i32 + 1, sTVShowNewsMixingNumPlayers as i32)],
                    );
                    if sCurTVShowSlot != -1 {
                        InitTryMixPokeNewsShow(
                            pokeNews[rem_i32(
                                j as i32 + k as i32 + 1,
                                sTVShowNewsMixingNumPlayers as i32,
                            )],
                            pokeNews[j],
                        );
                    }
                    k += 1;
                }
            }
            j += 1;
        }
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn InitTryMixPokeNewsShow(
    dest: *mut *mut PokeNews,
    src: *mut *mut PokeNews,
) {
    let mut ptr1: *mut PokeNews = *dest;
    let mut ptr2: *mut PokeNews = *src;
    ptr2 = ptr2.at(sTVShowMixingCurSlot);
    TryMixPokeNewsShow(ptr1, ptr2, sCurTVShowSlot);
}
pub(crate) unsafe extern "C" fn TryMixPokeNewsShow(
    mut dest: *mut PokeNews,
    src: *mut PokeNews,
    slot: i8,
) -> u8 {
    let mut i: u8 = 0;
    if (*src).kind == POKENEWS_NONE {
        return FALSE;
    }
    i = 0;
    while i < POKE_NEWS_COUNT {
        if (*dest.at(i)).kind == (*src).kind {
            return FALSE;
        }
        i += 1;
    }
    (*dest.at(slot)).kind = (*src).kind;
    (*dest.at(slot)).state = POKENEWS_STATE_UPCOMING;
    (*dest.at(slot)).dayCountdown = (*src).dayCountdown;
    return TRUE;
}
pub(crate) unsafe extern "C" fn GetPokeNewsSlotIfActive(pokeNews: *mut PokeNews, idx: u8) -> i8 {
    if (*pokeNews.at(idx)).kind == POKENEWS_NONE {
        return -1;
    }
    return idx as i8;
}
pub(crate) unsafe extern "C" fn ClearInvalidPokeNews() {
    let mut i: u8 = 0;
    i = 0;
    while i < POKE_NEWS_COUNT {
        if (*gSaveBlock1Ptr).pokeNews[i].kind > POKENEWS_BLENDMASTER {
            ClearPokeNewsBySlot(i);
        }
        i += 1;
    }
    CompactPokeNews();
}
pub(crate) unsafe extern "C" fn ClearPokeNewsIfGameNotComplete() {
    let mut i: u8 = 0;
    if FlagGet(FLAG_SYS_GAME_CLEAR) != TRUE {
        i = 0;
        while i < POKE_NEWS_COUNT {
            (*gSaveBlock1Ptr).pokeNews[i].state = POKENEWS_STATE_INACTIVE;
            i += 1;
        }
    }
}
pub(crate) unsafe extern "C" fn TranslateShowNames(mut show: *mut TVShow, language: u32) {
    let mut i: i32 = 0;
    let mut shows: *mut *mut TVShow = null_mut();
    shows = AllocZeroed(44) as *mut *mut TVShow;
    i = 0;
    while i < LAST_TVSHOW_IDX as i32 {
        match (*show.at(i)).common.kind {
            TVSHOW_FAN_CLUB_LETTER | TVSHOW_RECENT_HAPPENINGS => {
                *shows = show.at(i);
                if IsStringJapanese((*(*shows)).fanclubLetter.playerName.as_mut_ptr()) != 0 {
                    (*(*shows)).fanclubLetter.language = 1;
                } else {
                    (*(*shows)).fanclubLetter.language = language as u8;
                }
            }
            TVSHOW_PKMN_FAN_CLUB_OPINIONS => {
                *shows.at(1) = show.at(i);
                if IsStringJapanese((*(*shows.at(1))).fanclubOpinions.playerName.as_mut_ptr()) != 0
                {
                    (*(*shows.at(1))).fanclubOpinions.language = 1;
                } else {
                    (*(*shows.at(1))).fanclubOpinions.language = language as u8;
                }
                if IsStringJapanese((*(*shows.at(1))).fanclubOpinions.nickname.as_mut_ptr()) != 0 {
                    (*(*shows.at(1))).fanclubOpinions.pokemonNameLanguage = 1;
                } else {
                    (*(*shows.at(1))).fanclubOpinions.pokemonNameLanguage = language as u8;
                }
            }
            TVSHOW_POKEMON_TODAY_CAUGHT => {
                *shows.at(6) = show.at(i);
                if IsStringJapanese((*(*shows.at(6))).pokemonToday.playerName.as_mut_ptr()) != 0 {
                    (*(*shows.at(6))).pokemonToday.language = 1;
                } else {
                    (*(*shows.at(6))).pokemonToday.language = language as u8;
                }
                if IsStringJapanese((*(*shows.at(6))).pokemonToday.nickname.as_mut_ptr()) != 0 {
                    (*(*shows.at(6))).pokemonToday.language2 = 1;
                } else {
                    (*(*shows.at(6))).pokemonToday.language2 = language as u8;
                }
            }
            TVSHOW_SMART_SHOPPER => {
                *shows.at(7) = show.at(i);
                if IsStringJapanese((*(*shows.at(7))).smartshopperShow.playerName.as_mut_ptr()) != 0
                {
                    (*(*shows.at(7))).smartshopperShow.language = 1;
                } else {
                    (*(*shows.at(7))).smartshopperShow.language = language as u8;
                }
            }
            TVSHOW_BRAVO_TRAINER_BATTLE_TOWER_PROFILE => {
                *shows.at(5) = show.at(i);
                if IsStringJapanese((*(*shows.at(5))).bravoTrainerTower.playerName.as_mut_ptr())
                    != 0
                {
                    (*(*shows.at(5))).bravoTrainerTower.playerLanguage = 1;
                } else {
                    (*(*shows.at(5))).bravoTrainerTower.playerLanguage = language as u8;
                }
                if IsStringJapanese(
                    (*(*shows.at(5)))
                        .bravoTrainerTower
                        .opponentName
                        .as_mut_ptr(),
                ) != 0
                {
                    (*(*shows.at(5))).bravoTrainerTower.opponentLanguage = 1;
                } else {
                    (*(*shows.at(5))).bravoTrainerTower.opponentLanguage = language as u8;
                }
            }
            TVSHOW_BRAVO_TRAINER_POKEMON_PROFILE => {
                *shows.at(4) = show.at(i);
                if IsStringJapanese((*(*shows.at(4))).bravoTrainer.playerName.as_mut_ptr()) != 0 {
                    (*(*shows.at(4))).bravoTrainer.language = 1;
                } else {
                    (*(*shows.at(4))).bravoTrainer.language = language as u8;
                }
                if IsStringJapanese((*(*shows.at(4))).bravoTrainer.pokemonNickname.as_mut_ptr())
                    != 0
                {
                    (*(*shows.at(4))).bravoTrainer.pokemonNameLanguage = 1;
                } else {
                    (*(*shows.at(4))).bravoTrainer.pokemonNameLanguage = language as u8;
                }
            }
            TVSHOW_NAME_RATER_SHOW => {
                *shows.at(3) = show.at(i);
                if IsStringJapanese((*(*shows.at(3))).nameRaterShow.trainerName.as_mut_ptr()) != 0 {
                    (*(*shows.at(3))).nameRaterShow.language = 1;
                } else {
                    (*(*shows.at(3))).nameRaterShow.language = language as u8;
                }
                if IsStringJapanese((*(*shows.at(3))).nameRaterShow.pokemonName.as_mut_ptr()) != 0 {
                    (*(*shows.at(3))).nameRaterShow.pokemonNameLanguage = 1;
                } else {
                    (*(*shows.at(3))).nameRaterShow.pokemonNameLanguage = language as u8;
                }
            }
            TVSHOW_POKEMON_TODAY_FAILED => {
                *shows.at(2) = show.at(i);
                if IsStringJapanese((*(*shows.at(2))).pokemonTodayFailed.playerName.as_mut_ptr())
                    != 0
                {
                    (*(*shows.at(2))).pokemonTodayFailed.language = 1;
                } else {
                    (*(*shows.at(2))).pokemonTodayFailed.language = language as u8;
                }
            }
            TVSHOW_FISHING_ADVICE => {
                *shows.at(8) = show.at(i);
                if IsStringJapanese((*(*shows.at(8))).pokemonAngler.playerName.as_mut_ptr()) != 0 {
                    (*(*shows.at(8))).pokemonAngler.language = 1;
                } else {
                    (*(*shows.at(8))).pokemonAngler.language = language as u8;
                }
            }
            TVSHOW_WORLD_OF_MASTERS => {
                *shows.at(9) = show.at(i);
                if IsStringJapanese((*(*shows.at(9))).worldOfMasters.playerName.as_mut_ptr()) != 0 {
                    (*(*shows.at(9))).worldOfMasters.language = 1;
                } else {
                    (*(*shows.at(9))).worldOfMasters.language = language as u8;
                }
            }
            TVSHOW_MASS_OUTBREAK => {
                *shows.at(10) = show.at(i);
                (*(*shows.at(10))).massOutbreak.language = language as u8;
            }
            _ => {}
        }
        i += 1;
    }
    Free(shows as *mut c_void);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SanitizeTVShowsForRuby(shows: *mut TVShow) {
    let mut curShow: *mut TVShow = null_mut();
    SanitizeTVShowLocationsForRuby(shows);
    curShow = shows;
    while curShow < shows.at(24) {
        if (*curShow).bravoTrainerTower.kind == TVSHOW_BRAVO_TRAINER_BATTLE_TOWER_PROFILE {
            if (*curShow).bravoTrainerTower.playerLanguage == LANGUAGE_JAPANESE
                && (*curShow).bravoTrainerTower.opponentLanguage != LANGUAGE_JAPANESE
                || (*curShow).bravoTrainerTower.playerLanguage != LANGUAGE_JAPANESE
                    && (*curShow).bravoTrainerTower.opponentLanguage == LANGUAGE_JAPANESE
            {
                memset(curShow as *mut u8, 0, 36);
            }
        }
        curShow = curShow.at(1);
    }
}
pub(crate) unsafe extern "C" fn TranslateRubyShows(shows: *mut TVShow) {
    let mut curShow: *mut TVShow = null_mut();
    curShow = shows;
    while curShow < shows.at(24) {
        if (*curShow).bravoTrainerTower.kind == TVSHOW_BRAVO_TRAINER_BATTLE_TOWER_PROFILE {
            if IsStringJapanese((*curShow).bravoTrainerTower.opponentName.as_mut_ptr()) != 0 {
                (*curShow).bravoTrainerTower.opponentLanguage = LANGUAGE_JAPANESE;
            } else {
                (*curShow).bravoTrainerTower.opponentLanguage = GAME_LANGUAGE;
            }
        }
        curShow = curShow.at(1);
    }
}
pub(crate) unsafe extern "C" fn GetStringLanguage(str: *mut u8) -> u8 {
    return (if IsStringJapanese(str) != 0 {
        LANGUAGE_JAPANESE as i32
    } else {
        GAME_LANGUAGE as i32
    }) as u8;
}
pub(crate) unsafe extern "C" fn TranslateJapaneseEmeraldShows(shows: *mut TVShow) {
    let mut curShow: *mut TVShow = null_mut();
    curShow = shows;
    while curShow < shows.at(24) {
        match (*curShow).common.kind {
            TVSHOW_FAN_CLUB_LETTER => {
                (*curShow).fanclubLetter.language =
                    GetStringLanguage((*curShow).fanclubLetter.playerName.as_mut_ptr());
            }
            TVSHOW_RECENT_HAPPENINGS => {
                (*curShow).recentHappenings.language =
                    GetStringLanguage((*curShow).recentHappenings.playerName.as_mut_ptr());
            }
            TVSHOW_PKMN_FAN_CLUB_OPINIONS => {
                (*curShow).fanclubOpinions.language =
                    GetStringLanguage((*curShow).fanclubOpinions.playerName.as_mut_ptr());
                (*curShow).fanclubOpinions.pokemonNameLanguage =
                    GetStringLanguage((*curShow).fanclubOpinions.nickname.as_mut_ptr());
            }
            TVSHOW_DUMMY => {
                (*curShow).dummy.language = GetStringLanguage((*curShow).dummy.name.as_mut_ptr());
            }
            TVSHOW_NAME_RATER_SHOW => {
                (*curShow).nameRaterShow.language =
                    GetStringLanguage((*curShow).nameRaterShow.trainerName.as_mut_ptr());
                (*curShow).nameRaterShow.pokemonNameLanguage =
                    GetStringLanguage((*curShow).nameRaterShow.pokemonName.as_mut_ptr());
            }
            TVSHOW_BRAVO_TRAINER_POKEMON_PROFILE => {
                (*curShow).bravoTrainer.language =
                    GetStringLanguage((*curShow).bravoTrainer.playerName.as_mut_ptr());
                (*curShow).bravoTrainer.pokemonNameLanguage =
                    GetStringLanguage((*curShow).bravoTrainer.pokemonNickname.as_mut_ptr());
            }
            TVSHOW_BRAVO_TRAINER_BATTLE_TOWER_PROFILE => {
                (*curShow).bravoTrainerTower.playerLanguage =
                    GetStringLanguage((*curShow).bravoTrainerTower.playerName.as_mut_ptr());
                (*curShow).bravoTrainerTower.opponentLanguage =
                    GetStringLanguage((*curShow).bravoTrainerTower.opponentName.as_mut_ptr());
            }
            TVSHOW_CONTEST_LIVE_UPDATES => {
                (*curShow).contestLiveUpdates.winningTrainerLanguage = GetStringLanguage(
                    (*curShow)
                        .contestLiveUpdates
                        .winningTrainerName
                        .as_mut_ptr(),
                );
                (*curShow).contestLiveUpdates.losingTrainerLanguage =
                    GetStringLanguage((*curShow).contestLiveUpdates.losingTrainerName.as_mut_ptr());
            }
            TVSHOW_3_CHEERS_FOR_POKEBLOCKS => {
                (*curShow).threeCheers.language =
                    GetStringLanguage((*curShow).threeCheers.playerName.as_mut_ptr());
                (*curShow).threeCheers.worstBlenderLanguage =
                    GetStringLanguage((*curShow).threeCheers.worstBlenderName.as_mut_ptr());
            }
            TVSHOW_BATTLE_UPDATE => {
                (*curShow).battleUpdate.language =
                    GetStringLanguage((*curShow).battleUpdate.playerName.as_mut_ptr());
                (*curShow).battleUpdate.linkOpponentLanguage =
                    GetStringLanguage((*curShow).battleUpdate.linkOpponentName.as_mut_ptr());
            }
            TVSHOW_FAN_CLUB_SPECIAL => {
                (*curShow).fanClubSpecial.language =
                    GetStringLanguage((*curShow).fanClubSpecial.playerName.as_mut_ptr());
                (*curShow).fanClubSpecial.idolNameLanguage =
                    GetStringLanguage((*curShow).fanClubSpecial.idolName.as_mut_ptr());
            }
            TVSHOW_LILYCOVE_CONTEST_LADY => {
                (*curShow).contestLady.language =
                    GetStringLanguage((*curShow).contestLady.playerName.as_mut_ptr());
                (*curShow).contestLady.pokemonNameLanguage =
                    GetStringLanguage((*curShow).contestLady.nickname.as_mut_ptr());
            }
            TVSHOW_POKEMON_TODAY_CAUGHT => {
                (*curShow).pokemonToday.language =
                    GetStringLanguage((*curShow).pokemonToday.playerName.as_mut_ptr());
                (*curShow).pokemonToday.language2 =
                    GetStringLanguage((*curShow).pokemonToday.nickname.as_mut_ptr());
            }
            TVSHOW_SMART_SHOPPER => {
                (*curShow).smartshopperShow.language =
                    GetStringLanguage((*curShow).smartshopperShow.playerName.as_mut_ptr());
            }
            TVSHOW_POKEMON_TODAY_FAILED => {
                (*curShow).pokemonTodayFailed.language =
                    GetStringLanguage((*curShow).pokemonTodayFailed.playerName.as_mut_ptr());
            }
            TVSHOW_FISHING_ADVICE => {
                (*curShow).pokemonAngler.language =
                    GetStringLanguage((*curShow).pokemonAngler.playerName.as_mut_ptr());
            }
            TVSHOW_WORLD_OF_MASTERS => {
                (*curShow).worldOfMasters.language =
                    GetStringLanguage((*curShow).worldOfMasters.playerName.as_mut_ptr());
            }
            TVSHOW_TREND_WATCHER => {
                (*curShow).trendWatcher.language =
                    GetStringLanguage((*curShow).trendWatcher.playerName.as_mut_ptr());
            }
            TVSHOW_BREAKING_NEWS => {
                (*curShow).breakingNews.language =
                    GetStringLanguage((*curShow).breakingNews.playerName.as_mut_ptr());
            }
            TVSHOW_BATTLE_SEMINAR => {
                (*curShow).battleSeminar.language =
                    GetStringLanguage((*curShow).battleSeminar.playerName.as_mut_ptr());
            }
            TVSHOW_FIND_THAT_GAMER | TVSHOW_TRAINER_FAN_CLUB => {
                (*curShow).trainerFanClub.language =
                    GetStringLanguage((*curShow).trainerFanClub.playerName.as_mut_ptr());
            }
            TVSHOW_CUTIES => {
                (*curShow).cuties.language =
                    GetStringLanguage((*curShow).cuties.playerName.as_mut_ptr());
                (*curShow).cuties.pokemonNameLanguage =
                    GetStringLanguage((*curShow).cuties.nickname.as_mut_ptr());
            }
            TVSHOW_TODAYS_RIVAL_TRAINER | TVSHOW_SECRET_BASE_VISIT | TVSHOW_FRONTIER => {
                (*curShow).rivalTrainer.language =
                    GetStringLanguage((*curShow).rivalTrainer.playerName.as_mut_ptr());
            }
            TVSHOW_TREASURE_INVESTIGATORS | TVSHOW_LOTTO_WINNER | TVSHOW_NUMBER_ONE => {
                (*curShow).treasureInvestigators.language =
                    GetStringLanguage((*curShow).treasureInvestigators.playerName.as_mut_ptr());
            }
            TVSHOW_SECRET_BASE_SECRETS => {
                (*curShow).secretBaseSecrets.language =
                    GetStringLanguage((*curShow).secretBaseSecrets.playerName.as_mut_ptr());
                (*curShow).secretBaseSecrets.baseOwnersNameLanguage =
                    GetStringLanguage((*curShow).secretBaseSecrets.baseOwnersName.as_mut_ptr());
            }
            TVSHOW_SAFARI_FAN_CLUB => {
                (*curShow).safariFanClub.language =
                    GetStringLanguage((*curShow).safariFanClub.playerName.as_mut_ptr());
            }
            TVSHOW_MASS_OUTBREAK => {}
            _ => {}
        }
        curShow = curShow.at(1);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SanitizeTVShowLocationsForRuby(mut shows: *mut TVShow) {
    let mut i: i32 = 0;
    i = 0;
    while i < LAST_TVSHOW_IDX as i32 {
        match (*shows.at(i)).common.kind {
            TVSHOW_WORLD_OF_MASTERS => {
                if (*shows.at(i)).worldOfMasters.location > MAPSEC_PALLET_TOWN {
                    memset(shows.at(i) as *mut u8, 0, 36);
                }
            }
            TVSHOW_POKEMON_TODAY_FAILED => {
                if (*shows.at(i)).pokemonTodayFailed.location > MAPSEC_PALLET_TOWN {
                    memset(shows.at(i) as *mut u8, 0, 36);
                }
            }
            _ => {}
        }
        i += 1;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DoTVShow() {
    if (*gSaveBlock1Ptr).tvShows[gSpecialVar_0x8004].common.active != 0 {
        match (*gSaveBlock1Ptr).tvShows[gSpecialVar_0x8004].common.kind {
            TVSHOW_FAN_CLUB_LETTER => {
                DoTVShowPokemonFanClubLetter();
            }
            TVSHOW_RECENT_HAPPENINGS => {
                DoTVShowRecentHappenings();
            }
            TVSHOW_PKMN_FAN_CLUB_OPINIONS => {
                DoTVShowPokemonFanClubOpinions();
            }
            TVSHOW_DUMMY => {
                DoTVShowDummiedOut();
            }
            TVSHOW_MASS_OUTBREAK => {
                DoTVShowPokemonNewsMassOutbreak();
            }
            TVSHOW_BRAVO_TRAINER_POKEMON_PROFILE => {
                DoTVShowBravoTrainerPokemonProfile();
            }
            TVSHOW_BRAVO_TRAINER_BATTLE_TOWER_PROFILE => {
                DoTVShowBravoTrainerBattleTower();
            }
            TVSHOW_POKEMON_TODAY_CAUGHT => {
                DoTVShowPokemonTodaySuccessfulCapture();
            }
            TVSHOW_SMART_SHOPPER => {
                DoTVShowTodaysSmartShopper();
            }
            TVSHOW_NAME_RATER_SHOW => {
                DoTVShowTheNameRaterShow();
            }
            TVSHOW_CONTEST_LIVE_UPDATES => {
                DoTVShowPokemonContestLiveUpdates();
            }
            TVSHOW_BATTLE_UPDATE => {
                DoTVShowPokemonBattleUpdate();
            }
            TVSHOW_3_CHEERS_FOR_POKEBLOCKS => {
                DoTVShow3CheersForPokeblocks();
            }
            TVSHOW_POKEMON_TODAY_FAILED => {
                DoTVShowPokemonTodayFailedCapture();
            }
            TVSHOW_FISHING_ADVICE => {
                DoTVShowPokemonAngler();
            }
            TVSHOW_WORLD_OF_MASTERS => {
                DoTVShowTheWorldOfMasters();
            }
            TVSHOW_TODAYS_RIVAL_TRAINER => {
                DoTVShowTodaysRivalTrainer();
            }
            TVSHOW_TREND_WATCHER => {
                DoTVShowDewfordTrendWatcherNetwork();
            }
            TVSHOW_TREASURE_INVESTIGATORS => {
                DoTVShowHoennTreasureInvestigators();
            }
            TVSHOW_FIND_THAT_GAMER => {
                DoTVShowFindThatGamer();
            }
            TVSHOW_BREAKING_NEWS => {
                DoTVShowBreakingNewsTV();
            }
            TVSHOW_SECRET_BASE_VISIT => {
                DoTVShowSecretBaseVisit();
            }
            TVSHOW_LOTTO_WINNER => {
                DoTVShowPokemonLotteryWinnerFlashReport();
            }
            TVSHOW_BATTLE_SEMINAR => {
                DoTVShowThePokemonBattleSeminar();
            }
            TVSHOW_FAN_CLUB_SPECIAL => {
                DoTVShowTrainerFanClubSpecial();
            }
            TVSHOW_TRAINER_FAN_CLUB => {
                DoTVShowTrainerFanClub();
            }
            TVSHOW_CUTIES => {
                DoTVShowSpotTheCuties();
            }
            TVSHOW_FRONTIER => {
                DoTVShowPokemonNewsBattleFrontier();
            }
            TVSHOW_NUMBER_ONE => {
                DoTVShowWhatsNo1InHoennToday();
            }
            TVSHOW_SECRET_BASE_SECRETS => {
                DoTVShowSecretBaseSecrets();
            }
            TVSHOW_SAFARI_FAN_CLUB => {
                DoTVShowSafariFanClub();
            }
            TVSHOW_LILYCOVE_CONTEST_LADY => {
                DoTVShowLilycoveContestLady();
            }
            _ => {}
        }
    }
}
pub(crate) unsafe extern "C" fn DoTVShowBravoTrainerPokemonProfile() {
    let mut show: *mut TVShow = null_mut();
    let mut state: u8 = 0;
    show = &raw mut (*gSaveBlock1Ptr).tvShows[gSpecialVar_0x8004];
    gSpecialVar_Result = FALSE as u16;
    state = sTVShowState;
    match state {
        0 => {
            TVShowConvertInternationalString(
                gStringVar1.as_mut_ptr(),
                (*show).bravoTrainer.playerName.as_mut_ptr(),
                (*show).bravoTrainer.language as i32,
            );
            CopyContestCategoryToStringVar(1, (*show).bravoTrainer.contestCategory());
            CopyContestRankToStringVar(2, (*show).bravoTrainer.contestRank());
            if StringCompare(
                gSpeciesNames[(*show).bravoTrainer.species]
                    .as_ptr()
                    .cast_mut(),
                (*show).bravoTrainer.pokemonNickname.as_mut_ptr(),
            ) == 0
            {
                sTVShowState = 8;
            } else {
                sTVShowState = 1;
            }
        }
        1 => {
            StringCopy(
                gStringVar1.as_mut_ptr(),
                gSpeciesNames[(*show).bravoTrainer.species]
                    .as_ptr()
                    .cast_mut(),
            );
            TVShowConvertInternationalString(
                gStringVar2.as_mut_ptr(),
                (*show).bravoTrainer.pokemonNickname.as_mut_ptr(),
                (*show).bravoTrainer.pokemonNameLanguage as i32,
            );
            CopyContestCategoryToStringVar(2, (*show).bravoTrainer.contestCategory());
            sTVShowState = 2;
        }
        2 => {
            TVShowConvertInternationalString(
                gStringVar1.as_mut_ptr(),
                (*show).bravoTrainer.playerName.as_mut_ptr(),
                (*show).bravoTrainer.language as i32,
            );
            if (*show).bravoTrainer.contestResult() == 0 {
                sTVShowState = 3;
            } else {
                sTVShowState = 4;
            }
        }
        3 => {
            TVShowConvertInternationalString(
                gStringVar1.as_mut_ptr(),
                (*show).bravoTrainer.playerName.as_mut_ptr(),
                (*show).bravoTrainer.language as i32,
            );
            CopyEasyChatWord(gStringVar2.as_mut_ptr(), (*show).bravoTrainer.words[0]);
            ConvertIntToDecimalString(2, (*show).bravoTrainer.contestResult() as i32 + 1);
            sTVShowState = 5;
        }
        4 => {
            TVShowConvertInternationalString(
                gStringVar1.as_mut_ptr(),
                (*show).bravoTrainer.playerName.as_mut_ptr(),
                (*show).bravoTrainer.language as i32,
            );
            CopyEasyChatWord(gStringVar2.as_mut_ptr(), (*show).bravoTrainer.words[0]);
            ConvertIntToDecimalString(2, (*show).bravoTrainer.contestResult() as i32 + 1);
            sTVShowState = 5;
        }
        5 => {
            TVShowConvertInternationalString(
                gStringVar1.as_mut_ptr(),
                (*show).bravoTrainer.playerName.as_mut_ptr(),
                (*show).bravoTrainer.language as i32,
            );
            CopyContestCategoryToStringVar(1, (*show).bravoTrainer.contestCategory());
            CopyEasyChatWord(gStringVar3.as_mut_ptr(), (*show).bravoTrainer.words[1]);
            if (*show).bravoTrainer.r#move != 0 {
                sTVShowState = 6;
            } else {
                sTVShowState = 7;
            }
        }
        6 => {
            StringCopy(
                gStringVar1.as_mut_ptr(),
                gSpeciesNames[(*show).bravoTrainer.species]
                    .as_ptr()
                    .cast_mut(),
            );
            StringCopy(
                gStringVar2.as_mut_ptr(),
                gMoveNames[(*show).bravoTrainer.r#move].as_ptr().cast_mut(),
            );
            CopyEasyChatWord(gStringVar3.as_mut_ptr(), (*show).bravoTrainer.words[1]);
            sTVShowState = 7;
        }
        7 => {
            TVShowConvertInternationalString(
                gStringVar1.as_mut_ptr(),
                (*show).bravoTrainer.playerName.as_mut_ptr(),
                (*show).bravoTrainer.language as i32,
            );
            StringCopy(
                gStringVar2.as_mut_ptr(),
                gSpeciesNames[(*show).bravoTrainer.species]
                    .as_ptr()
                    .cast_mut(),
            );
            TVShowDone();
        }
        8 => {
            StringCopy(
                gStringVar1.as_mut_ptr(),
                gSpeciesNames[(*show).bravoTrainer.species]
                    .as_ptr()
                    .cast_mut(),
            );
            sTVShowState = 2;
        }
        _ => {}
    }
    ShowFieldMessage(sTVBravoTrainerTextGroup[state]);
}
pub(crate) unsafe extern "C" fn DoTVShowBravoTrainerBattleTower() {
    let mut show: *mut TVShow = null_mut();
    let mut state: u8 = 0;
    show = &raw mut (*gSaveBlock1Ptr).tvShows[gSpecialVar_0x8004];
    gSpecialVar_Result = FALSE as u16;
    state = sTVShowState;
    match state {
        BRAVOTOWER_STATE_INTRO => {
            TVShowConvertInternationalString(
                gStringVar1.as_mut_ptr(),
                (*show).bravoTrainerTower.playerName.as_mut_ptr(),
                (*show).bravoTrainerTower.playerLanguage as i32,
            );
            StringCopy(
                gStringVar2.as_mut_ptr(),
                gSpeciesNames[(*show).bravoTrainerTower.species]
                    .as_ptr()
                    .cast_mut(),
            );
            if (*show).bravoTrainerTower.numFights >= FRONTIER_STAGES_PER_CHALLENGE {
                sTVShowState = BRAVOTOWER_STATE_NEW_RECORD;
            } else {
                sTVShowState = BRAVOTOWER_STATE_LOST;
            }
        }
        BRAVOTOWER_STATE_NEW_RECORD => {
            if (*show).bravoTrainerTower.btLevel == FRONTIER_MAX_LEVEL_50 {
                StringCopy(gStringVar1.as_mut_ptr(), gText_Lv50.as_ptr().cast_mut());
            } else {
                StringCopy(
                    gStringVar1.as_mut_ptr(),
                    gText_OpenLevel.as_ptr().cast_mut(),
                );
            }
            ConvertIntToDecimalString(1, (*show).bravoTrainerTower.numFights as i32);
            if (*show).bravoTrainerTower.wonTheChallenge == TRUE {
                sTVShowState = BRAVOTOWER_STATE_WON;
            } else {
                sTVShowState = BRAVOTOWER_STATE_LOST_FINAL;
            }
        }
        BRAVOTOWER_STATE_LOST => {
            TVShowConvertInternationalString(
                gStringVar1.as_mut_ptr(),
                (*show).bravoTrainerTower.opponentName.as_mut_ptr(),
                (*show).bravoTrainerTower.opponentLanguage as i32,
            );
            ConvertIntToDecimalString(1, (*show).bravoTrainerTower.numFights as i32 + 1);
            if (*show).bravoTrainerTower.interviewResponse == 0 {
                sTVShowState = BRAVOTOWER_STATE_SATISFIED;
            } else {
                sTVShowState = BRAVOTOWER_STATE_UNSATISFIED;
            }
        }
        BRAVOTOWER_STATE_WON => {
            TVShowConvertInternationalString(
                gStringVar1.as_mut_ptr(),
                (*show).bravoTrainerTower.opponentName.as_mut_ptr(),
                (*show).bravoTrainerTower.opponentLanguage as i32,
            );
            StringCopy(
                gStringVar2.as_mut_ptr(),
                gSpeciesNames[(*show).bravoTrainerTower.defeatedSpecies]
                    .as_ptr()
                    .cast_mut(),
            );
            if (*show).bravoTrainerTower.interviewResponse == 0 {
                sTVShowState = BRAVOTOWER_STATE_SATISFIED;
            } else {
                sTVShowState = BRAVOTOWER_STATE_UNSATISFIED;
            }
        }
        BRAVOTOWER_STATE_LOST_FINAL => {
            TVShowConvertInternationalString(
                gStringVar1.as_mut_ptr(),
                (*show).bravoTrainerTower.opponentName.as_mut_ptr(),
                (*show).bravoTrainerTower.opponentLanguage as i32,
            );
            StringCopy(
                gStringVar2.as_mut_ptr(),
                gSpeciesNames[(*show).bravoTrainerTower.defeatedSpecies]
                    .as_ptr()
                    .cast_mut(),
            );
            if (*show).bravoTrainerTower.interviewResponse == 0 {
                sTVShowState = BRAVOTOWER_STATE_SATISFIED;
            } else {
                sTVShowState = BRAVOTOWER_STATE_UNSATISFIED;
            }
        }
        BRAVOTOWER_STATE_SATISFIED => {
            TVShowConvertInternationalString(
                gStringVar1.as_mut_ptr(),
                (*show).bravoTrainerTower.opponentName.as_mut_ptr(),
                (*show).bravoTrainerTower.opponentLanguage as i32,
            );
            sTVShowState = BRAVOTOWER_STATE_RESPONSE;
        }
        BRAVOTOWER_STATE_UNSATISFIED => {
            TVShowConvertInternationalString(
                gStringVar1.as_mut_ptr(),
                (*show).bravoTrainerTower.opponentName.as_mut_ptr(),
                (*show).bravoTrainerTower.opponentLanguage as i32,
            );
            sTVShowState = BRAVOTOWER_STATE_RESPONSE;
        }
        BRAVOTOWER_STATE_UNUSED_1 => {
            sTVShowState = BRAVOTOWER_STATE_RESPONSE;
        }
        BRAVOTOWER_STATE_UNUSED_2 | BRAVOTOWER_STATE_UNUSED_3 | BRAVOTOWER_STATE_UNUSED_4 => {
            TVShowConvertInternationalString(
                gStringVar1.as_mut_ptr(),
                (*show).bravoTrainerTower.playerName.as_mut_ptr(),
                (*show).bravoTrainerTower.playerLanguage as i32,
            );
            sTVShowState = BRAVOTOWER_STATE_RESPONSE;
        }
        BRAVOTOWER_STATE_RESPONSE => {
            CopyEasyChatWord(gStringVar1.as_mut_ptr(), (*show).bravoTrainerTower.words[0]);
            if (*show).bravoTrainerTower.interviewResponse == 0 {
                sTVShowState = BRAVOTOWER_STATE_RESPONSE_SATISFIED;
            } else {
                sTVShowState = BRAVOTOWER_STATE_RESPONSE_UNSATISFIED;
            }
        }
        BRAVOTOWER_STATE_RESPONSE_SATISFIED | BRAVOTOWER_STATE_RESPONSE_UNSATISFIED => {
            CopyEasyChatWord(gStringVar1.as_mut_ptr(), (*show).bravoTrainerTower.words[0]);
            TVShowConvertInternationalString(
                gStringVar2.as_mut_ptr(),
                (*show).bravoTrainerTower.playerName.as_mut_ptr(),
                (*show).bravoTrainerTower.playerLanguage as i32,
            );
            TVShowConvertInternationalString(
                gStringVar3.as_mut_ptr(),
                (*show).bravoTrainerTower.opponentName.as_mut_ptr(),
                (*show).bravoTrainerTower.opponentLanguage as i32,
            );
            sTVShowState = BRAVOTOWER_STATE_OUTRO;
        }
        BRAVOTOWER_STATE_OUTRO => {
            TVShowConvertInternationalString(
                gStringVar1.as_mut_ptr(),
                (*show).bravoTrainerTower.playerName.as_mut_ptr(),
                (*show).bravoTrainerTower.playerLanguage as i32,
            );
            StringCopy(
                gStringVar2.as_mut_ptr(),
                gSpeciesNames[(*show).bravoTrainerTower.species]
                    .as_ptr()
                    .cast_mut(),
            );
            TVShowDone();
        }
        _ => {}
    }
    ShowFieldMessage(sTVBravoTrainerBattleTowerTextGroup[state]);
}
pub(crate) unsafe extern "C" fn DoTVShowTodaysSmartShopper() {
    let mut show: *mut TVShow = null_mut();
    let mut state: u8 = 0;
    show = &raw mut (*gSaveBlock1Ptr).tvShows[gSpecialVar_0x8004];
    gSpecialVar_Result = FALSE as u16;
    state = sTVShowState;
    match state {
        SMARTSHOPPER_STATE_INTRO => {
            TVShowConvertInternationalString(
                gStringVar1.as_mut_ptr(),
                (*show).smartshopperShow.playerName.as_mut_ptr(),
                (*show).smartshopperShow.language as i32,
            );
            GetMapName(
                gStringVar2.as_mut_ptr(),
                (*show).smartshopperShow.shopLocation as u16,
                0,
            );
            if (*show).smartshopperShow.itemAmounts[0] >= 255 {
                sTVShowState = SMARTSHOPPER_STATE_CLERK_MAX;
            } else {
                sTVShowState = SMARTSHOPPER_STATE_CLERK_NORMAL;
            }
        }
        SMARTSHOPPER_STATE_CLERK_NORMAL => {
            TVShowConvertInternationalString(
                gStringVar1.as_mut_ptr(),
                (*show).smartshopperShow.playerName.as_mut_ptr(),
                (*show).smartshopperShow.language as i32,
            );
            StringCopy(
                gStringVar2.as_mut_ptr(),
                GetItemName((*show).smartshopperShow.itemIds[0]),
            );
            ConvertIntToDecimalString(2, (*show).smartshopperShow.itemAmounts[0] as i32);
            sTVShowState += 1 + (Random() as i32 % 4) as u8;
        }
        SMARTSHOPPER_STATE_RAND_COMMENT_1
        | SMARTSHOPPER_STATE_RAND_COMMENT_3
        | SMARTSHOPPER_STATE_RAND_COMMENT_4 => {
            if (*show).smartshopperShow.itemIds[1] != ITEM_NONE {
                sTVShowState = SMARTSHOPPER_STATE_SECOND_ITEM;
            } else {
                sTVShowState = SMARTSHOPPER_STATE_IS_VIP;
            }
        }
        SMARTSHOPPER_STATE_RAND_COMMENT_2 => {
            ConvertIntToDecimalString(2, (*show).smartshopperShow.itemAmounts[0] as i32 + 1);
            if (*show).smartshopperShow.itemIds[1] != ITEM_NONE {
                sTVShowState = SMARTSHOPPER_STATE_SECOND_ITEM;
            } else {
                sTVShowState = SMARTSHOPPER_STATE_IS_VIP;
            }
        }
        SMARTSHOPPER_STATE_SECOND_ITEM => {
            StringCopy(
                gStringVar2.as_mut_ptr(),
                GetItemName((*show).smartshopperShow.itemIds[1]),
            );
            ConvertIntToDecimalString(2, (*show).smartshopperShow.itemAmounts[1] as i32);
            if (*show).smartshopperShow.itemIds[2] != ITEM_NONE {
                sTVShowState = SMARTSHOPPER_STATE_THIRD_ITEM;
            } else if (*show).smartshopperShow.priceReduced == TRUE {
                sTVShowState = SMARTSHOPPER_STATE_DURING_SALE;
            } else {
                sTVShowState = SMARTSHOPPER_STATE_OUTRO_NORMAL;
            }
        }
        SMARTSHOPPER_STATE_THIRD_ITEM => {
            StringCopy(
                gStringVar2.as_mut_ptr(),
                GetItemName((*show).smartshopperShow.itemIds[2]),
            );
            ConvertIntToDecimalString(2, (*show).smartshopperShow.itemAmounts[2] as i32);
            if (*show).smartshopperShow.priceReduced == TRUE {
                sTVShowState = SMARTSHOPPER_STATE_DURING_SALE;
            } else {
                sTVShowState = SMARTSHOPPER_STATE_OUTRO_NORMAL;
            }
        }
        SMARTSHOPPER_STATE_DURING_SALE => {
            if (*show).smartshopperShow.itemAmounts[0] >= 255 {
                sTVShowState = SMARTSHOPPER_STATE_OUTRO_MAX;
            } else {
                sTVShowState = SMARTSHOPPER_STATE_OUTRO_NORMAL;
            }
        }
        SMARTSHOPPER_STATE_OUTRO_NORMAL => {
            SmartShopper_BufferPurchaseTotal(1, show);
            TVShowDone();
        }
        SMARTSHOPPER_STATE_IS_VIP => {
            if (*show).smartshopperShow.priceReduced == TRUE {
                sTVShowState = SMARTSHOPPER_STATE_DURING_SALE;
            } else {
                sTVShowState = SMARTSHOPPER_STATE_OUTRO_NORMAL;
            }
        }
        SMARTSHOPPER_STATE_CLERK_MAX => {
            TVShowConvertInternationalString(
                gStringVar1.as_mut_ptr(),
                (*show).smartshopperShow.playerName.as_mut_ptr(),
                (*show).smartshopperShow.language as i32,
            );
            StringCopy(
                gStringVar2.as_mut_ptr(),
                GetItemName((*show).smartshopperShow.itemIds[0]),
            );
            if (*show).smartshopperShow.priceReduced == TRUE {
                sTVShowState = SMARTSHOPPER_STATE_DURING_SALE;
            } else {
                sTVShowState = SMARTSHOPPER_STATE_OUTRO_MAX;
            }
        }
        SMARTSHOPPER_STATE_OUTRO_MAX => {
            TVShowConvertInternationalString(
                gStringVar1.as_mut_ptr(),
                (*show).smartshopperShow.playerName.as_mut_ptr(),
                (*show).smartshopperShow.language as i32,
            );
            TVShowDone();
        }
        _ => {}
    }
    ShowFieldMessage(sTVTodaysSmartShopperTextGroup[state]);
}
pub(crate) unsafe extern "C" fn DoTVShowTheNameRaterShow() {
    let mut show: *mut TVShow = null_mut();
    let mut state: u8 = 0;
    show = &raw mut (*gSaveBlock1Ptr).tvShows[gSpecialVar_0x8004];
    gSpecialVar_Result = FALSE as u16;
    state = sTVShowState;
    'l1: {
        let sw1: u8 = state;
        let mut fall = false;
        if sw1 == 0 {
            fall = true;
            TVShowConvertInternationalString(
                gStringVar1.as_mut_ptr(),
                (*show).nameRaterShow.trainerName.as_mut_ptr(),
                (*show).nameRaterShow.language as i32,
            );
            StringCopy(
                gStringVar2.as_mut_ptr(),
                gSpeciesNames[(*show).nameRaterShow.species]
                    .as_ptr()
                    .cast_mut(),
            );
            TVShowConvertInternationalString(
                gStringVar3.as_mut_ptr(),
                (*show).nameRaterShow.pokemonName.as_mut_ptr(),
                (*show).nameRaterShow.pokemonNameLanguage as i32,
            );
            sTVShowState = GetRandomNameRaterStateFromName(show) + 1;
            break 'l1;
        }
        if sw1 == 1 || sw1 == 3 || sw1 == 4 || sw1 == 5 || sw1 == 6 || sw1 == 7 || sw1 == 8 {
            fall = true;
            if (*show).nameRaterShow.random == 0 {
                sTVShowState = 9;
            } else if (*show).nameRaterShow.random == 1 {
                sTVShowState = 10;
            } else if (*show).nameRaterShow.random == 2 {
                sTVShowState = 11;
            }
            break 'l1;
        }
        if sw1 == 2 {
            fall = true;
            TVShowConvertInternationalString(
                gStringVar1.as_mut_ptr(),
                (*show).nameRaterShow.trainerName.as_mut_ptr(),
                (*show).nameRaterShow.language as i32,
            );
            if (*show).nameRaterShow.random == 0 {
                sTVShowState = 9;
            } else if (*show).nameRaterShow.random == 1 {
                sTVShowState = 10;
            } else if (*show).nameRaterShow.random == 2 {
                sTVShowState = 11;
            }
            break 'l1;
        }
        if sw1 == 9 || sw1 == 10 || sw1 == 11 {
            fall = true;
            TVShowConvertInternationalString(
                gStringVar1.as_mut_ptr(),
                (*show).nameRaterShow.pokemonName.as_mut_ptr(),
                (*show).nameRaterShow.pokemonNameLanguage as i32,
            );
            GetNicknameSubstring(1, 0, 0, 1, 0, show);
            GetNicknameSubstring(2, 1, 0, 1, 0, show);
            sTVShowState = 12;
            break 'l1;
        }
        if sw1 == 13 {
            fall = true;
            TVShowConvertInternationalString(
                gStringVar1.as_mut_ptr(),
                (*show).nameRaterShow.trainerName.as_mut_ptr(),
                (*show).nameRaterShow.language as i32,
            );
            GetNicknameSubstring(1, 0, 2, 0, 0, show);
            GetNicknameSubstring(2, 0, 3, 1, 0, show);
            sTVShowState = 14;
            break 'l1;
        }
        if sw1 == 14 {
            fall = true;
            GetNicknameSubstring(1, 0, 2, 1, 0, show);
            GetNicknameSubstring(2, 0, 3, 0, 0, show);
            sTVShowState = 18;
            break 'l1;
        }
        if sw1 == 15 {
            fall = true;
            GetNicknameSubstring(0, 0, 2, 1, 0, show);
            StringCopy(
                gStringVar2.as_mut_ptr(),
                gSpeciesNames[(*show).nameRaterShow.species]
                    .as_ptr()
                    .cast_mut(),
            );
            GetNicknameSubstring(2, 0, 3, 2, (*show).nameRaterShow.species, show);
            sTVShowState = 16;
            break 'l1;
        }
        if sw1 == 16 {
            fall = true;
            GetNicknameSubstring(0, 0, 2, 2, (*show).nameRaterShow.species, show);
            GetNicknameSubstring(2, 0, 3, 1, 0, show);
            sTVShowState = 17;
            break 'l1;
        }
        if sw1 == 17 {
            fall = true;
            GetNicknameSubstring(0, 0, 2, 1, 0, show);
            StringCopy(
                gStringVar2.as_mut_ptr(),
                gSpeciesNames[(*show).nameRaterShow.randomSpecies]
                    .as_ptr()
                    .cast_mut(),
            );
            GetNicknameSubstring(2, 0, 3, 2, (*show).nameRaterShow.randomSpecies, show);
            sTVShowState = 18;
            break 'l1;
        }
        if sw1 == 12 {
            fall = true;
            state = 18;
            sTVShowState = 18;
        }
        if fall || sw1 == 18 {
            fall = true;
            TVShowConvertInternationalString(
                gStringVar1.as_mut_ptr(),
                (*show).nameRaterShow.pokemonName.as_mut_ptr(),
                (*show).nameRaterShow.pokemonNameLanguage as i32,
            );
            TVShowDone();
            break 'l1;
        }
    }
    ShowFieldMessage(sTVNameRaterTextGroup[state]);
}
pub(crate) unsafe extern "C" fn DoTVShowPokemonTodaySuccessfulCapture() {
    let mut show: *mut TVShow = null_mut();
    let mut state: u8 = 0;
    show = &raw mut (*gSaveBlock1Ptr).tvShows[gSpecialVar_0x8004];
    gSpecialVar_Result = FALSE as u16;
    state = sTVShowState;
    match state {
        0 => {
            TVShowConvertInternationalString(
                gStringVar1.as_mut_ptr(),
                (*show).pokemonToday.playerName.as_mut_ptr(),
                (*show).pokemonToday.language as i32,
            );
            StringCopy(
                gStringVar2.as_mut_ptr(),
                gSpeciesNames[(*show).pokemonToday.species]
                    .as_ptr()
                    .cast_mut(),
            );
            TVShowConvertInternationalString(
                gStringVar3.as_mut_ptr(),
                (*show).pokemonToday.nickname.as_mut_ptr(),
                (*show).pokemonToday.language2 as i32,
            );
            if (*show).pokemonToday.ball == ITEM_MASTER_BALL as u8 {
                sTVShowState = 5;
            } else {
                sTVShowState = 1;
            }
        }
        1 => {
            sTVShowState = 2;
        }
        2 => {
            StringCopy(
                gStringVar2.as_mut_ptr(),
                GetItemName((*show).pokemonToday.ball as u16),
            );
            ConvertIntToDecimalString(2, (*show).pokemonToday.nBallsUsed as i32);
            if (*show).pokemonToday.nBallsUsed < 4 {
                sTVShowState = 3;
            } else {
                sTVShowState = 4;
            }
        }
        3 => {
            TVShowConvertInternationalString(
                gStringVar1.as_mut_ptr(),
                (*show).pokemonToday.playerName.as_mut_ptr(),
                (*show).pokemonToday.language as i32,
            );
            StringCopy(
                gStringVar2.as_mut_ptr(),
                gSpeciesNames[(*show).pokemonToday.species]
                    .as_ptr()
                    .cast_mut(),
            );
            TVShowConvertInternationalString(
                gStringVar3.as_mut_ptr(),
                (*show).pokemonToday.nickname.as_mut_ptr(),
                (*show).pokemonToday.language2 as i32,
            );
            sTVShowState = 6;
        }
        4 => {
            sTVShowState = 6;
        }
        5 => {
            TVShowConvertInternationalString(
                gStringVar1.as_mut_ptr(),
                (*show).pokemonToday.playerName.as_mut_ptr(),
                (*show).pokemonToday.language as i32,
            );
            StringCopy(
                gStringVar2.as_mut_ptr(),
                gSpeciesNames[(*show).pokemonToday.species]
                    .as_ptr()
                    .cast_mut(),
            );
            sTVShowState = 6;
        }
        6 => {
            TVShowConvertInternationalString(
                gStringVar1.as_mut_ptr(),
                (*show).pokemonToday.playerName.as_mut_ptr(),
                (*show).pokemonToday.language as i32,
            );
            StringCopy(
                gStringVar2.as_mut_ptr(),
                gSpeciesNames[(*show).pokemonToday.species]
                    .as_ptr()
                    .cast_mut(),
            );
            TVShowConvertInternationalString(
                gStringVar3.as_mut_ptr(),
                (*show).pokemonToday.nickname.as_mut_ptr(),
                (*show).pokemonToday.language2 as i32,
            );
            sTVShowState += 1 + (Random() as i32 % 4) as u8;
        }
        7 | 8 => {
            StringCopy(
                gStringVar1.as_mut_ptr(),
                gSpeciesNames[(*show).pokemonToday.species]
                    .as_ptr()
                    .cast_mut(),
            );
            TVShowConvertInternationalString(
                gStringVar2.as_mut_ptr(),
                (*show).pokemonToday.nickname.as_mut_ptr(),
                (*show).pokemonToday.language2 as i32,
            );
            GetRandomDifferentSpeciesAndNameSeenByPlayer(2, (*show).pokemonToday.species);
            sTVShowState = 11;
        }
        9 | 10 => {
            StringCopy(
                gStringVar1.as_mut_ptr(),
                gSpeciesNames[(*show).pokemonToday.species]
                    .as_ptr()
                    .cast_mut(),
            );
            TVShowConvertInternationalString(
                gStringVar2.as_mut_ptr(),
                (*show).pokemonToday.nickname.as_mut_ptr(),
                (*show).pokemonToday.language2 as i32,
            );
            sTVShowState = 11;
        }
        11 => {
            TVShowDone();
        }
        _ => {}
    }
    ShowFieldMessage(sTVPokemonTodaySuccessfulTextGroup[state]);
}
pub(crate) unsafe extern "C" fn DoTVShowPokemonTodayFailedCapture() {
    let mut show: *mut TVShow = null_mut();
    let mut state: u8 = 0;
    show = &raw mut (*gSaveBlock1Ptr).tvShows[gSpecialVar_0x8004];
    gSpecialVar_Result = FALSE as u16;
    state = sTVShowState;
    match state {
        0 => {
            TVShowConvertInternationalString(
                gStringVar1.as_mut_ptr(),
                (*show).pokemonTodayFailed.playerName.as_mut_ptr(),
                (*show).pokemonTodayFailed.language as i32,
            );
            StringCopy(
                gStringVar2.as_mut_ptr(),
                gSpeciesNames[(*show).pokemonTodayFailed.species]
                    .as_ptr()
                    .cast_mut(),
            );
            sTVShowState = 1;
        }
        1 => {
            TVShowConvertInternationalString(
                gStringVar1.as_mut_ptr(),
                (*show).pokemonTodayFailed.playerName.as_mut_ptr(),
                (*show).pokemonTodayFailed.language as i32,
            );
            GetMapName(
                gStringVar2.as_mut_ptr(),
                (*show).pokemonTodayFailed.location as u16,
                0,
            );
            StringCopy(
                gStringVar3.as_mut_ptr(),
                gSpeciesNames[(*show).pokemonTodayFailed.species2]
                    .as_ptr()
                    .cast_mut(),
            );
            if (*show).pokemonTodayFailed.outcome == 1 {
                sTVShowState = 3;
            } else {
                sTVShowState = 2;
            }
        }
        2 | 3 => {
            TVShowConvertInternationalString(
                gStringVar1.as_mut_ptr(),
                (*show).pokemonTodayFailed.playerName.as_mut_ptr(),
                (*show).pokemonTodayFailed.language as i32,
            );
            ConvertIntToDecimalString(1, (*show).pokemonTodayFailed.nBallsUsed as i32);
            if Random() as i32 % 3 == 0 {
                sTVShowState = 5;
            } else {
                sTVShowState = 4;
            }
        }
        4 | 5 => {
            TVShowConvertInternationalString(
                gStringVar1.as_mut_ptr(),
                (*show).pokemonTodayFailed.playerName.as_mut_ptr(),
                (*show).pokemonTodayFailed.language as i32,
            );
            sTVShowState = 6;
        }
        6 => {
            TVShowDone();
        }
        _ => {}
    }
    ShowFieldMessage(sTVPokemonTodayFailedTextGroup[state]);
}
pub(crate) unsafe extern "C" fn DoTVShowPokemonFanClubLetter() {
    let mut show: *mut TVShow = null_mut();
    let mut state: u8 = 0;
    let mut rval: u16 = 0;
    show = &raw mut (*gSaveBlock1Ptr).tvShows[gSpecialVar_0x8004];
    gSpecialVar_Result = FALSE as u16;
    state = sTVShowState;
    match state {
        0 => {
            TVShowConvertInternationalString(
                gStringVar1.as_mut_ptr(),
                (*show).fanclubLetter.playerName.as_mut_ptr(),
                (*show).fanclubLetter.language as i32,
            );
            StringCopy(
                gStringVar2.as_mut_ptr(),
                gSpeciesNames[(*show).fanclubLetter.species]
                    .as_ptr()
                    .cast_mut(),
            );
            sTVShowState = 50;
        }
        1 => {
            rval = (Random() as i32 % 4) as u16 + 1;
            if rval == 1 {
                sTVShowState = 2;
            } else {
                sTVShowState = rval as u8 + 2;
            }
        }
        2 => {
            sTVShowState = 51;
        }
        3 => {
            sTVShowState += (Random() as i32 % 3) as u8 + 1;
        }
        4 | 5 | 6 => {
            GetRandomWordFromShow(show);
            sTVShowState = 7;
        }
        7 => {
            rval = (Random() as i32 % 31) as u16 + 0x46;
            ConvertIntToDecimalString(2, rval as i32);
            TVShowDone();
        }
        50 => {
            ConvertEasyChatWordsToString(
                gStringVar4.as_mut_ptr(),
                (*show).fanclubLetter.words.as_mut_ptr(),
                2,
                2,
            );
            ShowFieldMessage(gStringVar4.as_mut_ptr());
            sTVShowState = 1;
            return;
        }
        51 => {
            ConvertEasyChatWordsToString(
                gStringVar4.as_mut_ptr(),
                (*show).fanclubLetter.words.as_mut_ptr(),
                2,
                2,
            );
            ShowFieldMessage(gStringVar4.as_mut_ptr());
            sTVShowState = 3;
            return;
        }
        _ => {}
    }
    ShowFieldMessage(sTVFanClubTextGroup[state]);
}
pub(crate) unsafe extern "C" fn DoTVShowRecentHappenings() {
    let mut show: *mut TVShow = null_mut();
    let mut state: u8 = 0;
    show = &raw mut (*gSaveBlock1Ptr).tvShows[gSpecialVar_0x8004];
    gSpecialVar_Result = FALSE as u16;
    state = sTVShowState;
    match state {
        0 => {
            TVShowConvertInternationalString(
                gStringVar1.as_mut_ptr(),
                (*show).recentHappenings.playerName.as_mut_ptr(),
                (*show).recentHappenings.language as i32,
            );
            GetRandomWordFromShow(show);
            sTVShowState = 50;
        }
        1 => {
            sTVShowState += 1 + (Random() as i32 % 3) as u8;
        }
        2 | 3 | 4 => {
            sTVShowState = 5;
        }
        5 => {
            TVShowDone();
        }
        50 => {
            ConvertEasyChatWordsToString(
                gStringVar4.as_mut_ptr(),
                (*show).recentHappenings.words.as_mut_ptr(),
                2,
                2,
            );
            ShowFieldMessage(gStringVar4.as_mut_ptr());
            sTVShowState = 1;
            return;
        }
        _ => {}
    }
    ShowFieldMessage(sTVRecentHappeninssTextGroup[state]);
}
pub(crate) unsafe extern "C" fn DoTVShowPokemonFanClubOpinions() {
    let mut show: *mut TVShow = null_mut();
    let mut state: u8 = 0;
    show = &raw mut (*gSaveBlock1Ptr).tvShows[gSpecialVar_0x8004];
    gSpecialVar_Result = FALSE as u16;
    state = sTVShowState;
    match state {
        0 => {
            TVShowConvertInternationalString(
                gStringVar1.as_mut_ptr(),
                (*show).fanclubOpinions.playerName.as_mut_ptr(),
                (*show).fanclubOpinions.language as i32,
            );
            StringCopy(
                gStringVar2.as_mut_ptr(),
                gSpeciesNames[(*show).fanclubOpinions.species]
                    .as_ptr()
                    .cast_mut(),
            );
            TVShowConvertInternationalString(
                gStringVar3.as_mut_ptr(),
                (*show).fanclubOpinions.nickname.as_mut_ptr(),
                (*show).fanclubOpinions.pokemonNameLanguage as i32,
            );
            sTVShowState = (*show).fanclubOpinions.questionAsked() + 1;
        }
        1 | 2 | 3 => {
            TVShowConvertInternationalString(
                gStringVar1.as_mut_ptr(),
                (*show).fanclubOpinions.playerName.as_mut_ptr(),
                (*show).fanclubOpinions.language as i32,
            );
            StringCopy(
                gStringVar2.as_mut_ptr(),
                gSpeciesNames[(*show).fanclubOpinions.species]
                    .as_ptr()
                    .cast_mut(),
            );
            CopyEasyChatWord(gStringVar3.as_mut_ptr(), (*show).fanclubOpinions.words[0]);
            sTVShowState = 4;
        }
        4 => {
            TVShowConvertInternationalString(
                gStringVar1.as_mut_ptr(),
                (*show).fanclubOpinions.playerName.as_mut_ptr(),
                (*show).fanclubOpinions.language as i32,
            );
            CopyEasyChatWord(gStringVar3.as_mut_ptr(), (*show).fanclubOpinions.words[1]);
            TVShowDone();
        }
        _ => {}
    }
    ShowFieldMessage(sTVFanClubOpinionsTextGroup[state]);
}
pub(crate) unsafe extern "C" fn DoTVShowDummiedOut() {}
pub(crate) unsafe extern "C" fn DoTVShowPokemonNewsMassOutbreak() {
    let mut show: *mut TVShow = null_mut();
    show = &raw mut (*gSaveBlock1Ptr).tvShows[gSpecialVar_0x8004];
    GetMapName(
        gStringVar1.as_mut_ptr(),
        (*show).massOutbreak.locationMapNum as u16,
        0,
    );
    StringCopy(
        gStringVar2.as_mut_ptr(),
        gSpeciesNames[(*show).massOutbreak.species]
            .as_ptr()
            .cast_mut(),
    );
    TVShowDone();
    StartMassOutbreak();
    ShowFieldMessage(sTVMassOutbreakTextGroup[sTVShowState]);
}
pub(crate) unsafe extern "C" fn DoTVShowPokemonContestLiveUpdates() {
    let mut show: *mut TVShow = null_mut();
    let mut state: u8 = 0;
    show = &raw mut (*gSaveBlock1Ptr).tvShows[gSpecialVar_0x8004];
    gSpecialVar_Result = FALSE as u16;
    state = sTVShowState;
    match state {
        CONTESTLIVE_STATE_INTRO => {
            BufferContestName(
                gStringVar1.as_mut_ptr(),
                (*show).contestLiveUpdates.category,
            );
            StringCopy(
                gStringVar2.as_mut_ptr(),
                gSpeciesNames[(*show).contestLiveUpdates.winningSpecies]
                    .as_ptr()
                    .cast_mut(),
            );
            TVShowConvertInternationalString(
                gStringVar3.as_mut_ptr(),
                (*show).contestLiveUpdates.winningTrainerName.as_mut_ptr(),
                (*show).contestLiveUpdates.winningTrainerLanguage as i32,
            );
            if (*show).contestLiveUpdates.round1Placing == (*show).contestLiveUpdates.round2Placing
            {
                if (*show).contestLiveUpdates.round1Placing == 0 {
                    sTVShowState = CONTESTLIVE_STATE_WON_BOTH_ROUNDS;
                } else {
                    sTVShowState = CONTESTLIVE_STATE_EQUAL_ROUNDS;
                }
            } else if (*show).contestLiveUpdates.round1Placing
                > (*show).contestLiveUpdates.round2Placing
            {
                sTVShowState = CONTESTLIVE_STATE_BETTER_ROUND2;
            } else {
                sTVShowState = CONTESTLIVE_STATE_BETTER_ROUND1;
            }
        }
        CONTESTLIVE_STATE_WON_BOTH_ROUNDS => {
            StringCopy(
                gStringVar2.as_mut_ptr(),
                gSpeciesNames[(*show).contestLiveUpdates.winningSpecies]
                    .as_ptr()
                    .cast_mut(),
            );
            match (*show).contestLiveUpdates.winnerAppealFlag {
                CONTESTLIVE_FLAG_EXCITING_APPEAL => {
                    sTVShowState = CONTESTLIVE_STATE_EXCITING_APPEAL;
                }
                CONTESTLIVE_FLAG_GOT_NERVOUS => {
                    sTVShowState = CONTESTLIVE_STATE_GOT_NERVOUS;
                }
                CONTESTLIVE_FLAG_MAXED_EXCITEMENT => {
                    sTVShowState = CONTESTLIVE_STATE_VERY_EXCITING_APPEAL;
                }
                CONTESTLIVE_FLAG_USED_COMBO => {
                    sTVShowState = CONTESTLIVE_STATE_USED_COMBO;
                }
                CONTESTLIVE_FLAG_STARTLED_OTHER => {
                    sTVShowState = CONTESTLIVE_STATE_STARTLED_OTHER;
                }
                CONTESTLIVE_FLAG_SKIPPED_TURN => {
                    sTVShowState = CONTESTLIVE_STATE_TOOK_BREAK;
                }
                CONTESTLIVE_FLAG_GOT_STARTLED => {
                    sTVShowState = CONTESTLIVE_STATE_GOT_STARTLED;
                }
                CONTESTLIVE_FLAG_MADE_APPEAL => {
                    sTVShowState = CONTESTLIVE_STATE_USED_MOVE;
                }
                _ => {}
            }
        }
        CONTESTLIVE_STATE_BETTER_ROUND2 => {
            StringCopy(
                gStringVar2.as_mut_ptr(),
                gSpeciesNames[(*show).contestLiveUpdates.winningSpecies]
                    .as_ptr()
                    .cast_mut(),
            );
            match (*show).contestLiveUpdates.winnerAppealFlag {
                CONTESTLIVE_FLAG_EXCITING_APPEAL => {
                    sTVShowState = CONTESTLIVE_STATE_EXCITING_APPEAL;
                }
                CONTESTLIVE_FLAG_GOT_NERVOUS => {
                    sTVShowState = CONTESTLIVE_STATE_GOT_NERVOUS;
                }
                CONTESTLIVE_FLAG_MAXED_EXCITEMENT => {
                    sTVShowState = CONTESTLIVE_STATE_VERY_EXCITING_APPEAL;
                }
                CONTESTLIVE_FLAG_USED_COMBO => {
                    sTVShowState = CONTESTLIVE_STATE_USED_COMBO;
                }
                CONTESTLIVE_FLAG_STARTLED_OTHER => {
                    sTVShowState = CONTESTLIVE_STATE_STARTLED_OTHER;
                }
                CONTESTLIVE_FLAG_SKIPPED_TURN => {
                    sTVShowState = CONTESTLIVE_STATE_TOOK_BREAK;
                }
                CONTESTLIVE_FLAG_GOT_STARTLED => {
                    sTVShowState = CONTESTLIVE_STATE_GOT_STARTLED;
                }
                CONTESTLIVE_FLAG_MADE_APPEAL => {
                    sTVShowState = CONTESTLIVE_STATE_USED_MOVE;
                }
                _ => {}
            }
        }
        CONTESTLIVE_STATE_EQUAL_ROUNDS => {
            StringCopy(
                gStringVar2.as_mut_ptr(),
                gSpeciesNames[(*show).contestLiveUpdates.winningSpecies]
                    .as_ptr()
                    .cast_mut(),
            );
            TVShowConvertInternationalString(
                gStringVar3.as_mut_ptr(),
                (*show).contestLiveUpdates.winningTrainerName.as_mut_ptr(),
                (*show).contestLiveUpdates.winningTrainerLanguage as i32,
            );
            match (*show).contestLiveUpdates.winnerAppealFlag {
                CONTESTLIVE_FLAG_EXCITING_APPEAL => {
                    sTVShowState = CONTESTLIVE_STATE_EXCITING_APPEAL;
                }
                CONTESTLIVE_FLAG_GOT_NERVOUS => {
                    sTVShowState = CONTESTLIVE_STATE_GOT_NERVOUS;
                }
                CONTESTLIVE_FLAG_MAXED_EXCITEMENT => {
                    sTVShowState = CONTESTLIVE_STATE_VERY_EXCITING_APPEAL;
                }
                CONTESTLIVE_FLAG_USED_COMBO => {
                    sTVShowState = CONTESTLIVE_STATE_USED_COMBO;
                }
                CONTESTLIVE_FLAG_STARTLED_OTHER => {
                    sTVShowState = CONTESTLIVE_STATE_STARTLED_OTHER;
                }
                CONTESTLIVE_FLAG_SKIPPED_TURN => {
                    sTVShowState = CONTESTLIVE_STATE_TOOK_BREAK;
                }
                CONTESTLIVE_FLAG_GOT_STARTLED => {
                    sTVShowState = CONTESTLIVE_STATE_GOT_STARTLED;
                }
                CONTESTLIVE_FLAG_MADE_APPEAL => {
                    sTVShowState = CONTESTLIVE_STATE_USED_MOVE;
                }
                _ => {}
            }
        }
        CONTESTLIVE_STATE_BETTER_ROUND1 => {
            match (*show).contestLiveUpdates.category {
                CONTEST_CATEGORY_COOL => {
                    StringCopy(gStringVar1.as_mut_ptr(), gText_Cool.as_ptr().cast_mut());
                }
                CONTEST_CATEGORY_BEAUTY => {
                    StringCopy(gStringVar1.as_mut_ptr(), gText_Beauty.as_ptr().cast_mut());
                }
                CONTEST_CATEGORY_CUTE => {
                    StringCopy(gStringVar1.as_mut_ptr(), gText_Cute.as_ptr().cast_mut());
                }
                CONTEST_CATEGORY_SMART => {
                    StringCopy(gStringVar1.as_mut_ptr(), gText_Smart.as_ptr().cast_mut());
                }
                CONTEST_CATEGORY_TOUGH => {
                    StringCopy(gStringVar1.as_mut_ptr(), gText_Tough.as_ptr().cast_mut());
                }
                _ => {}
            }
            StringCopy(
                gStringVar2.as_mut_ptr(),
                gSpeciesNames[(*show).contestLiveUpdates.winningSpecies]
                    .as_ptr()
                    .cast_mut(),
            );
            match (*show).contestLiveUpdates.winnerAppealFlag {
                CONTESTLIVE_FLAG_EXCITING_APPEAL => {
                    sTVShowState = CONTESTLIVE_STATE_EXCITING_APPEAL;
                }
                CONTESTLIVE_FLAG_GOT_NERVOUS => {
                    sTVShowState = CONTESTLIVE_STATE_GOT_NERVOUS;
                }
                CONTESTLIVE_FLAG_MAXED_EXCITEMENT => {
                    sTVShowState = CONTESTLIVE_STATE_VERY_EXCITING_APPEAL;
                }
                CONTESTLIVE_FLAG_USED_COMBO => {
                    sTVShowState = CONTESTLIVE_STATE_USED_COMBO;
                }
                CONTESTLIVE_FLAG_STARTLED_OTHER => {
                    sTVShowState = CONTESTLIVE_STATE_STARTLED_OTHER;
                }
                CONTESTLIVE_FLAG_SKIPPED_TURN => {
                    sTVShowState = CONTESTLIVE_STATE_TOOK_BREAK;
                }
                CONTESTLIVE_FLAG_GOT_STARTLED => {
                    sTVShowState = CONTESTLIVE_STATE_GOT_STARTLED;
                }
                CONTESTLIVE_FLAG_MADE_APPEAL => {
                    sTVShowState = CONTESTLIVE_STATE_USED_MOVE;
                }
                _ => {}
            }
        }
        CONTESTLIVE_STATE_GOT_NERVOUS => {
            StringCopy(
                gStringVar2.as_mut_ptr(),
                gSpeciesNames[(*show).contestLiveUpdates.winningSpecies]
                    .as_ptr()
                    .cast_mut(),
            );
            sTVShowState = CONTESTLIVE_STATE_TALK_ABOUT_LOSER;
        }
        CONTESTLIVE_STATE_STARTLED_OTHER => {
            StringCopy(
                gStringVar2.as_mut_ptr(),
                gSpeciesNames[(*show).contestLiveUpdates.winningSpecies]
                    .as_ptr()
                    .cast_mut(),
            );
            sTVShowState = CONTESTLIVE_STATE_TALK_ABOUT_LOSER;
        }
        CONTESTLIVE_STATE_USED_COMBO => {
            StringCopy(
                gStringVar2.as_mut_ptr(),
                gSpeciesNames[(*show).contestLiveUpdates.winningSpecies]
                    .as_ptr()
                    .cast_mut(),
            );
            sTVShowState = CONTESTLIVE_STATE_TALK_ABOUT_LOSER;
        }
        CONTESTLIVE_STATE_EXCITING_APPEAL => {
            StringCopy(
                gStringVar2.as_mut_ptr(),
                gSpeciesNames[(*show).contestLiveUpdates.winningSpecies]
                    .as_ptr()
                    .cast_mut(),
            );
            match (*show).contestLiveUpdates.category {
                CONTEST_CATEGORY_COOL => {
                    sTVShowState = CONTESTLIVE_STATE_COOL;
                }
                CONTEST_CATEGORY_BEAUTY => {
                    sTVShowState = CONTESTLIVE_STATE_BEAUTIFUL;
                }
                CONTEST_CATEGORY_CUTE => {
                    sTVShowState = CONTESTLIVE_STATE_CUTE;
                }
                CONTEST_CATEGORY_SMART => {
                    sTVShowState = CONTESTLIVE_STATE_SMART;
                }
                CONTEST_CATEGORY_TOUGH => {
                    sTVShowState = CONTESTLIVE_STATE_TOUGH;
                }
                _ => {}
            }
        }
        CONTESTLIVE_STATE_COOL => {
            StringCopy(
                gStringVar2.as_mut_ptr(),
                gSpeciesNames[(*show).contestLiveUpdates.winningSpecies]
                    .as_ptr()
                    .cast_mut(),
            );
            sTVShowState = CONTESTLIVE_STATE_TALK_ABOUT_LOSER;
        }
        CONTESTLIVE_STATE_BEAUTIFUL => {
            StringCopy(
                gStringVar2.as_mut_ptr(),
                gSpeciesNames[(*show).contestLiveUpdates.winningSpecies]
                    .as_ptr()
                    .cast_mut(),
            );
            sTVShowState = CONTESTLIVE_STATE_TALK_ABOUT_LOSER;
        }
        CONTESTLIVE_STATE_CUTE => {
            StringCopy(
                gStringVar2.as_mut_ptr(),
                gSpeciesNames[(*show).contestLiveUpdates.winningSpecies]
                    .as_ptr()
                    .cast_mut(),
            );
            sTVShowState = CONTESTLIVE_STATE_TALK_ABOUT_LOSER;
        }
        CONTESTLIVE_STATE_SMART => {
            StringCopy(
                gStringVar2.as_mut_ptr(),
                gSpeciesNames[(*show).contestLiveUpdates.winningSpecies]
                    .as_ptr()
                    .cast_mut(),
            );
            sTVShowState = CONTESTLIVE_STATE_TALK_ABOUT_LOSER;
        }
        CONTESTLIVE_STATE_TOUGH => {
            StringCopy(
                gStringVar2.as_mut_ptr(),
                gSpeciesNames[(*show).contestLiveUpdates.winningSpecies]
                    .as_ptr()
                    .cast_mut(),
            );
            sTVShowState = CONTESTLIVE_STATE_TALK_ABOUT_LOSER;
        }
        CONTESTLIVE_STATE_VERY_EXCITING_APPEAL => {
            StringCopy(
                gStringVar2.as_mut_ptr(),
                gSpeciesNames[(*show).contestLiveUpdates.winningSpecies]
                    .as_ptr()
                    .cast_mut(),
            );
            match (*show).contestLiveUpdates.category {
                CONTEST_CATEGORY_COOL => {
                    sTVShowState = CONTESTLIVE_STATE_VERY_COOL;
                }
                CONTEST_CATEGORY_BEAUTY => {
                    sTVShowState = CONTESTLIVE_STATE_VERY_BEAUTIFUL;
                }
                CONTEST_CATEGORY_CUTE => {
                    sTVShowState = CONTESTLIVE_STATE_VERY_CUTE;
                }
                CONTEST_CATEGORY_SMART => {
                    sTVShowState = CONTESTLIVE_STATE_VERY_SMART;
                }
                CONTEST_CATEGORY_TOUGH => {
                    sTVShowState = CONTESTLIVE_STATE_VERY_TOUGH;
                }
                _ => {}
            }
        }
        CONTESTLIVE_STATE_VERY_COOL => {
            StringCopy(
                gStringVar2.as_mut_ptr(),
                gSpeciesNames[(*show).contestLiveUpdates.winningSpecies]
                    .as_ptr()
                    .cast_mut(),
            );
            sTVShowState = CONTESTLIVE_STATE_TALK_ABOUT_LOSER;
        }
        CONTESTLIVE_STATE_VERY_BEAUTIFUL => {
            StringCopy(
                gStringVar2.as_mut_ptr(),
                gSpeciesNames[(*show).contestLiveUpdates.winningSpecies]
                    .as_ptr()
                    .cast_mut(),
            );
            sTVShowState = CONTESTLIVE_STATE_TALK_ABOUT_LOSER;
        }
        CONTESTLIVE_STATE_VERY_CUTE => {
            StringCopy(
                gStringVar2.as_mut_ptr(),
                gSpeciesNames[(*show).contestLiveUpdates.winningSpecies]
                    .as_ptr()
                    .cast_mut(),
            );
            sTVShowState = CONTESTLIVE_STATE_TALK_ABOUT_LOSER;
        }
        CONTESTLIVE_STATE_VERY_SMART => {
            StringCopy(
                gStringVar2.as_mut_ptr(),
                gSpeciesNames[(*show).contestLiveUpdates.winningSpecies]
                    .as_ptr()
                    .cast_mut(),
            );
            sTVShowState = CONTESTLIVE_STATE_TALK_ABOUT_LOSER;
        }
        CONTESTLIVE_STATE_VERY_TOUGH => {
            StringCopy(
                gStringVar2.as_mut_ptr(),
                gSpeciesNames[(*show).contestLiveUpdates.winningSpecies]
                    .as_ptr()
                    .cast_mut(),
            );
            sTVShowState = CONTESTLIVE_STATE_TALK_ABOUT_LOSER;
        }
        CONTESTLIVE_STATE_TOOK_BREAK => {
            StringCopy(
                gStringVar2.as_mut_ptr(),
                gSpeciesNames[(*show).contestLiveUpdates.winningSpecies]
                    .as_ptr()
                    .cast_mut(),
            );
            sTVShowState = CONTESTLIVE_STATE_TALK_ABOUT_LOSER;
        }
        CONTESTLIVE_STATE_GOT_STARTLED => {
            StringCopy(
                gStringVar2.as_mut_ptr(),
                gSpeciesNames[(*show).contestLiveUpdates.winningSpecies]
                    .as_ptr()
                    .cast_mut(),
            );
            sTVShowState = CONTESTLIVE_STATE_TALK_ABOUT_LOSER;
        }
        CONTESTLIVE_STATE_USED_MOVE => {
            StringCopy(
                gStringVar2.as_mut_ptr(),
                gSpeciesNames[(*show).contestLiveUpdates.winningSpecies]
                    .as_ptr()
                    .cast_mut(),
            );
            StringCopy(
                gStringVar3.as_mut_ptr(),
                gMoveNames[(*show).contestLiveUpdates.r#move]
                    .as_ptr()
                    .cast_mut(),
            );
            sTVShowState = CONTESTLIVE_STATE_TALK_ABOUT_LOSER;
        }
        CONTESTLIVE_STATE_TALK_ABOUT_LOSER => {
            StringCopy(
                gStringVar1.as_mut_ptr(),
                gSpeciesNames[(*show).contestLiveUpdates.winningSpecies]
                    .as_ptr()
                    .cast_mut(),
            );
            TVShowConvertInternationalString(
                gStringVar2.as_mut_ptr(),
                (*show).contestLiveUpdates.losingTrainerName.as_mut_ptr(),
                (*show).contestLiveUpdates.losingTrainerLanguage as i32,
            );
            StringCopy(
                gStringVar3.as_mut_ptr(),
                gSpeciesNames[(*show).contestLiveUpdates.losingSpecies]
                    .as_ptr()
                    .cast_mut(),
            );
            match (*show).contestLiveUpdates.loserAppealFlag {
                CONTESTLIVE_FLAG_LOST => {
                    sTVShowState = CONTESTLIVE_STATE_LOST;
                }
                CONTESTLIVE_FLAG_REPEATED_MOVE => {
                    sTVShowState = CONTESTLIVE_STATE_REPEATED_APPEALS;
                }
                CONTESTLIVE_FLAG_LOST_SMALL_MARGIN => {
                    sTVShowState = CONTESTLIVE_STATE_LOST_SMALL_MARGIN;
                }
                CONTESTLIVE_FLAG_NO_EXCITEMENT => {
                    sTVShowState = CONTESTLIVE_STATE_NO_EXCITING_APPEALS;
                }
                CONTESTLIVE_FLAG_BLEW_LEAD => {
                    sTVShowState = CONTESTLIVE_STATE_LOST_AFTER_ROUND1_WIN;
                }
                CONTESTLIVE_FLAG_MISSED_EXCITEMENT => {
                    sTVShowState = CONTESTLIVE_STATE_NOT_EXCITING_ENOUGH;
                }
                CONTESTLIVE_FLAG_LAST_BOTH_ROUNDS => {
                    sTVShowState = CONTESTLIVE_STATE_LAST_BOTH;
                }
                CONTESTLIVE_FLAG_NO_APPEALS => {
                    sTVShowState = CONTESTLIVE_STATE_NO_APPEALS;
                }
                _ => {}
            }
        }
        CONTESTLIVE_STATE_NO_APPEALS => {
            StringCopy(
                gStringVar1.as_mut_ptr(),
                gSpeciesNames[(*show).contestLiveUpdates.losingSpecies]
                    .as_ptr()
                    .cast_mut(),
            );
            sTVShowState = CONTESTLIVE_STATE_OUTRO;
        }
        CONTESTLIVE_STATE_LAST_BOTH => {
            TVShowConvertInternationalString(
                gStringVar1.as_mut_ptr(),
                (*show).contestLiveUpdates.losingTrainerName.as_mut_ptr(),
                (*show).contestLiveUpdates.losingTrainerLanguage as i32,
            );
            StringCopy(
                gStringVar2.as_mut_ptr(),
                gSpeciesNames[(*show).contestLiveUpdates.losingSpecies]
                    .as_ptr()
                    .cast_mut(),
            );
            sTVShowState = CONTESTLIVE_STATE_OUTRO;
        }
        CONTESTLIVE_STATE_NO_EXCITING_APPEALS => {
            sTVShowState = CONTESTLIVE_STATE_OUTRO;
        }
        CONTESTLIVE_STATE_LOST_SMALL_MARGIN => {
            TVShowConvertInternationalString(
                gStringVar1.as_mut_ptr(),
                (*show).contestLiveUpdates.winningTrainerName.as_mut_ptr(),
                (*show).contestLiveUpdates.winningTrainerLanguage as i32,
            );
            StringCopy(
                gStringVar2.as_mut_ptr(),
                gSpeciesNames[(*show).contestLiveUpdates.winningSpecies]
                    .as_ptr()
                    .cast_mut(),
            );
            TVShowConvertInternationalString(
                gStringVar3.as_mut_ptr(),
                (*show).contestLiveUpdates.losingTrainerName.as_mut_ptr(),
                (*show).contestLiveUpdates.losingTrainerLanguage as i32,
            );
            sTVShowState = CONTESTLIVE_STATE_OUTRO;
        }
        CONTESTLIVE_STATE_NOT_EXCITING_ENOUGH
        | CONTESTLIVE_STATE_LOST_AFTER_ROUND1_WIN
        | CONTESTLIVE_STATE_REPEATED_APPEALS
        | CONTESTLIVE_STATE_LOST => {
            TVShowConvertInternationalString(
                gStringVar1.as_mut_ptr(),
                (*show).contestLiveUpdates.losingTrainerName.as_mut_ptr(),
                (*show).contestLiveUpdates.losingTrainerLanguage as i32,
            );
            sTVShowState = CONTESTLIVE_STATE_OUTRO;
        }
        CONTESTLIVE_STATE_OUTRO => {
            TVShowConvertInternationalString(
                gStringVar1.as_mut_ptr(),
                (*show).contestLiveUpdates.winningTrainerName.as_mut_ptr(),
                (*show).contestLiveUpdates.winningTrainerLanguage as i32,
            );
            StringCopy(
                gStringVar2.as_mut_ptr(),
                gSpeciesNames[(*show).contestLiveUpdates.winningSpecies]
                    .as_ptr()
                    .cast_mut(),
            );
            TVShowDone();
        }
        _ => {}
    }
    ShowFieldMessage(sTVContestLiveUpdatesTextGroup[state]);
}
pub(crate) unsafe extern "C" fn DoTVShowPokemonBattleUpdate() {
    let mut show: *mut TVShow = null_mut();
    let mut state: u8 = 0;
    show = &raw mut (*gSaveBlock1Ptr).tvShows[gSpecialVar_0x8004];
    gSpecialVar_Result = FALSE as u16;
    state = sTVShowState;
    match state {
        0 => match (*show).battleUpdate.battleType {
            0 | 1 => {
                sTVShowState = 1;
            }
            2 => {
                sTVShowState = 5;
            }
            _ => {}
        },
        1 => {
            TVShowConvertInternationalString(
                gStringVar1.as_mut_ptr(),
                (*show).battleUpdate.playerName.as_mut_ptr(),
                (*show).battleUpdate.language as i32,
            );
            TVShowConvertInternationalString(
                gStringVar2.as_mut_ptr(),
                (*show).battleUpdate.linkOpponentName.as_mut_ptr(),
                (*show).battleUpdate.linkOpponentLanguage as i32,
            );
            if (*show).battleUpdate.battleType == 0 {
                StringCopy(gStringVar3.as_mut_ptr(), gText_Single.as_ptr().cast_mut());
            } else {
                StringCopy(gStringVar3.as_mut_ptr(), gText_Double.as_ptr().cast_mut());
            }
            sTVShowState = 2;
        }
        2 => {
            TVShowConvertInternationalString(
                gStringVar1.as_mut_ptr(),
                (*show).battleUpdate.playerName.as_mut_ptr(),
                (*show).battleUpdate.language as i32,
            );
            StringCopy(
                gStringVar2.as_mut_ptr(),
                gSpeciesNames[(*show).battleUpdate.speciesPlayer]
                    .as_ptr()
                    .cast_mut(),
            );
            StringCopy(
                gStringVar3.as_mut_ptr(),
                gMoveNames[(*show).battleUpdate.r#move].as_ptr().cast_mut(),
            );
            sTVShowState = 3;
        }
        3 => {
            TVShowConvertInternationalString(
                gStringVar1.as_mut_ptr(),
                (*show).battleUpdate.linkOpponentName.as_mut_ptr(),
                (*show).battleUpdate.linkOpponentLanguage as i32,
            );
            StringCopy(
                gStringVar2.as_mut_ptr(),
                gSpeciesNames[(*show).battleUpdate.speciesOpponent]
                    .as_ptr()
                    .cast_mut(),
            );
            sTVShowState = 4;
        }
        4 => {
            TVShowConvertInternationalString(
                gStringVar1.as_mut_ptr(),
                (*show).battleUpdate.playerName.as_mut_ptr(),
                (*show).battleUpdate.language as i32,
            );
            TVShowConvertInternationalString(
                gStringVar2.as_mut_ptr(),
                (*show).battleUpdate.linkOpponentName.as_mut_ptr(),
                (*show).battleUpdate.linkOpponentLanguage as i32,
            );
            TVShowDone();
        }
        5 => {
            TVShowConvertInternationalString(
                gStringVar1.as_mut_ptr(),
                (*show).battleUpdate.playerName.as_mut_ptr(),
                (*show).battleUpdate.language as i32,
            );
            TVShowConvertInternationalString(
                gStringVar2.as_mut_ptr(),
                (*show).battleUpdate.linkOpponentName.as_mut_ptr(),
                (*show).battleUpdate.linkOpponentLanguage as i32,
            );
            sTVShowState = 6;
        }
        6 => {
            TVShowConvertInternationalString(
                gStringVar1.as_mut_ptr(),
                (*show).battleUpdate.playerName.as_mut_ptr(),
                (*show).battleUpdate.language as i32,
            );
            StringCopy(
                gStringVar2.as_mut_ptr(),
                gSpeciesNames[(*show).battleUpdate.speciesPlayer]
                    .as_ptr()
                    .cast_mut(),
            );
            StringCopy(
                gStringVar3.as_mut_ptr(),
                gMoveNames[(*show).battleUpdate.r#move].as_ptr().cast_mut(),
            );
            sTVShowState = 7;
        }
        7 => {
            TVShowConvertInternationalString(
                gStringVar1.as_mut_ptr(),
                (*show).battleUpdate.playerName.as_mut_ptr(),
                (*show).battleUpdate.language as i32,
            );
            TVShowConvertInternationalString(
                gStringVar2.as_mut_ptr(),
                (*show).battleUpdate.linkOpponentName.as_mut_ptr(),
                (*show).battleUpdate.linkOpponentLanguage as i32,
            );
            StringCopy(
                gStringVar3.as_mut_ptr(),
                gSpeciesNames[(*show).battleUpdate.speciesOpponent]
                    .as_ptr()
                    .cast_mut(),
            );
            TVShowDone();
        }
        _ => {}
    }
    ShowFieldMessage(sTVPokemonBattleUpdateTextGroup[state]);
}
pub(crate) unsafe extern "C" fn DoTVShow3CheersForPokeblocks() {
    let mut show: *mut TVShow = null_mut();
    let mut state: u8 = 0;
    show = &raw mut (*gSaveBlock1Ptr).tvShows[gSpecialVar_0x8004];
    gSpecialVar_Result = FALSE as u16;
    state = sTVShowState;
    match state {
        0 => {
            TVShowConvertInternationalString(
                gStringVar1.as_mut_ptr(),
                (*show).threeCheers.playerName.as_mut_ptr(),
                (*show).threeCheers.language as i32,
            );
            if (*show).threeCheers.sheen > 20 {
                sTVShowState = 1;
            } else {
                sTVShowState = 3;
            }
        }
        1 => {
            match (*show).threeCheers.flavor() {
                0 => {
                    StringCopy(gStringVar1.as_mut_ptr(), gText_Spicy2.as_ptr().cast_mut());
                }
                1 => {
                    StringCopy(gStringVar1.as_mut_ptr(), gText_Dry2.as_ptr().cast_mut());
                }
                2 => {
                    StringCopy(gStringVar1.as_mut_ptr(), gText_Sweet2.as_ptr().cast_mut());
                }
                3 => {
                    StringCopy(gStringVar1.as_mut_ptr(), gText_Bitter2.as_ptr().cast_mut());
                }
                4 => {
                    StringCopy(gStringVar1.as_mut_ptr(), gText_Sour2.as_ptr().cast_mut());
                }
                _ => {}
            }
            if (*show).threeCheers.sheen > 24 {
                StringCopy(
                    gStringVar2.as_mut_ptr(),
                    gText_Excellent.as_ptr().cast_mut(),
                );
            } else if (*show).threeCheers.sheen > 22 {
                StringCopy(gStringVar2.as_mut_ptr(), gText_VeryGood.as_ptr().cast_mut());
            } else {
                StringCopy(gStringVar2.as_mut_ptr(), gText_Good.as_ptr().cast_mut());
            }
            TVShowConvertInternationalString(
                gStringVar3.as_mut_ptr(),
                (*show).threeCheers.playerName.as_mut_ptr(),
                (*show).threeCheers.language as i32,
            );
            sTVShowState = 2;
        }
        2 => {
            TVShowConvertInternationalString(
                gStringVar1.as_mut_ptr(),
                (*show).threeCheers.worstBlenderName.as_mut_ptr(),
                (*show).threeCheers.worstBlenderLanguage as i32,
            );
            sTVShowState = 5;
        }
        3 => {
            match (*show).threeCheers.flavor() {
                0 => {
                    StringCopy(gStringVar1.as_mut_ptr(), gText_Spicy2.as_ptr().cast_mut());
                }
                1 => {
                    StringCopy(gStringVar1.as_mut_ptr(), gText_Dry2.as_ptr().cast_mut());
                }
                2 => {
                    StringCopy(gStringVar1.as_mut_ptr(), gText_Sweet2.as_ptr().cast_mut());
                }
                3 => {
                    StringCopy(gStringVar1.as_mut_ptr(), gText_Bitter2.as_ptr().cast_mut());
                }
                4 => {
                    StringCopy(gStringVar1.as_mut_ptr(), gText_Sour2.as_ptr().cast_mut());
                }
                _ => {}
            }
            if (*show).threeCheers.sheen > 16 {
                StringCopy(gStringVar2.as_mut_ptr(), gText_SoSo.as_ptr().cast_mut());
            } else if (*show).threeCheers.sheen > 13 {
                StringCopy(gStringVar2.as_mut_ptr(), gText_Bad.as_ptr().cast_mut());
            } else {
                StringCopy(gStringVar2.as_mut_ptr(), gText_TheWorst.as_ptr().cast_mut());
            }
            TVShowConvertInternationalString(
                gStringVar3.as_mut_ptr(),
                (*show).threeCheers.playerName.as_mut_ptr(),
                (*show).threeCheers.language as i32,
            );
            sTVShowState = 4;
        }
        4 => {
            TVShowConvertInternationalString(
                gStringVar1.as_mut_ptr(),
                (*show).threeCheers.worstBlenderName.as_mut_ptr(),
                (*show).threeCheers.worstBlenderLanguage as i32,
            );
            TVShowConvertInternationalString(
                gStringVar2.as_mut_ptr(),
                (*show).threeCheers.playerName.as_mut_ptr(),
                (*show).threeCheers.language as i32,
            );
            sTVShowState = 5;
        }
        5 => {
            TVShowDone();
        }
        _ => {}
    }
    ShowFieldMessage(sTV3CheersForPokeblocksTextGroup[state]);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DoTVShowInSearchOfTrainers() {
    let mut state: u8 = 0;
    gSpecialVar_Result = FALSE as u16;
    state = sTVShowState;
    match state {
        0 => {
            GetMapName(
                gStringVar1.as_mut_ptr(),
                (*gSaveBlock1Ptr).gabbyAndTyData.mapnum as u16,
                0,
            );
            if (*gSaveBlock1Ptr).gabbyAndTyData.battleNum > 1 {
                sTVShowState = 1;
            } else {
                sTVShowState = 2;
            }
        }
        1 => {
            sTVShowState = 2;
        }
        2 => {
            if (*gSaveBlock1Ptr).gabbyAndTyData.battleTookMoreThanOneTurn() == 0 {
                sTVShowState = 4;
            } else if (*gSaveBlock1Ptr).gabbyAndTyData.playerThrewABall() != 0 {
                sTVShowState = 5;
            } else if (*gSaveBlock1Ptr).gabbyAndTyData.playerUsedHealingItem() != 0 {
                sTVShowState = 6;
            } else if (*gSaveBlock1Ptr).gabbyAndTyData.playerLostAMon() != 0 {
                sTVShowState = 7;
            } else {
                sTVShowState = 3;
            }
        }
        3 => {
            StringCopy(
                gStringVar1.as_mut_ptr(),
                gSpeciesNames[(*gSaveBlock1Ptr).gabbyAndTyData.mon1]
                    .as_ptr()
                    .cast_mut(),
            );
            StringCopy(
                gStringVar2.as_mut_ptr(),
                gMoveNames[(*gSaveBlock1Ptr).gabbyAndTyData.lastMove]
                    .as_ptr()
                    .cast_mut(),
            );
            StringCopy(
                gStringVar3.as_mut_ptr(),
                gSpeciesNames[(*gSaveBlock1Ptr).gabbyAndTyData.mon2]
                    .as_ptr()
                    .cast_mut(),
            );
            sTVShowState = 8;
        }
        4 | 5 | 6 | 7 => {
            sTVShowState = 8;
        }
        8 => {
            CopyEasyChatWord(
                gStringVar1.as_mut_ptr(),
                (*gSaveBlock1Ptr).gabbyAndTyData.quote[0],
            );
            StringCopy(
                gStringVar2.as_mut_ptr(),
                gSpeciesNames[(*gSaveBlock1Ptr).gabbyAndTyData.mon1]
                    .as_ptr()
                    .cast_mut(),
            );
            StringCopy(
                gStringVar3.as_mut_ptr(),
                gSpeciesNames[(*gSaveBlock1Ptr).gabbyAndTyData.mon2]
                    .as_ptr()
                    .cast_mut(),
            );
            gSpecialVar_Result = TRUE as u16;
            sTVShowState = 0;
            TakeGabbyAndTyOffTheAir();
        }
        _ => {}
    }
    ShowFieldMessage(sTVInSearchOfTrainersTextGroup[state]);
}
pub(crate) unsafe extern "C" fn DoTVShowPokemonAngler() {
    let mut show: *mut TVShow = null_mut();
    let mut state: u8 = 0;
    show = &raw mut (*gSaveBlock1Ptr).tvShows[gSpecialVar_0x8004];
    gSpecialVar_Result = FALSE as u16;
    if (*show).pokemonAngler.nBites < (*show).pokemonAngler.nFails {
        sTVShowState = 0;
    } else {
        sTVShowState = 1;
    }
    state = sTVShowState;
    match state {
        0 => {
            TVShowConvertInternationalString(
                gStringVar1.as_mut_ptr(),
                (*show).pokemonAngler.playerName.as_mut_ptr(),
                (*show).pokemonAngler.language as i32,
            );
            StringCopy(
                gStringVar2.as_mut_ptr(),
                gSpeciesNames[(*show).pokemonAngler.species]
                    .as_ptr()
                    .cast_mut(),
            );
            ConvertIntToDecimalString(2, (*show).pokemonAngler.nFails as i32);
            TVShowDone();
        }
        1 => {
            TVShowConvertInternationalString(
                gStringVar1.as_mut_ptr(),
                (*show).pokemonAngler.playerName.as_mut_ptr(),
                (*show).pokemonAngler.language as i32,
            );
            StringCopy(
                gStringVar2.as_mut_ptr(),
                gSpeciesNames[(*show).pokemonAngler.species]
                    .as_ptr()
                    .cast_mut(),
            );
            ConvertIntToDecimalString(2, (*show).pokemonAngler.nBites as i32);
            TVShowDone();
        }
        _ => {}
    }
    ShowFieldMessage(sTVPokemonAnglerTextGroup[state]);
}
pub(crate) unsafe extern "C" fn DoTVShowTheWorldOfMasters() {
    let mut show: *mut TVShow = null_mut();
    let mut state: u8 = 0;
    show = &raw mut (*gSaveBlock1Ptr).tvShows[gSpecialVar_0x8004];
    gSpecialVar_Result = FALSE as u16;
    state = sTVShowState;
    match state {
        0 => {
            TVShowConvertInternationalString(
                gStringVar1.as_mut_ptr(),
                (*show).worldOfMasters.playerName.as_mut_ptr(),
                (*show).worldOfMasters.language as i32,
            );
            ConvertIntToDecimalString(1, (*show).worldOfMasters.steps as i32);
            ConvertIntToDecimalString(2, (*show).worldOfMasters.numPokeCaught as i32);
            sTVShowState = 1;
        }
        1 => {
            StringCopy(
                gStringVar1.as_mut_ptr(),
                gSpeciesNames[(*show).worldOfMasters.species]
                    .as_ptr()
                    .cast_mut(),
            );
            sTVShowState = 2;
        }
        2 => {
            TVShowConvertInternationalString(
                gStringVar1.as_mut_ptr(),
                (*show).worldOfMasters.playerName.as_mut_ptr(),
                (*show).worldOfMasters.language as i32,
            );
            GetMapName(
                gStringVar2.as_mut_ptr(),
                (*show).worldOfMasters.location as u16,
                0,
            );
            StringCopy(
                gStringVar3.as_mut_ptr(),
                gSpeciesNames[(*show).worldOfMasters.caughtPoke]
                    .as_ptr()
                    .cast_mut(),
            );
            TVShowDone();
        }
        _ => {}
    }
    ShowFieldMessage(sTVWorldOfMastersTextGroup[state]);
}
pub(crate) unsafe extern "C" fn DoTVShowTodaysRivalTrainer() {
    let mut show: *mut TVShow = null_mut();
    let mut state: u8 = 0;
    show = &raw mut (*gSaveBlock1Ptr).tvShows[gSpecialVar_0x8004];
    gSpecialVar_Result = FALSE as u16;
    state = sTVShowState;
    match state {
        0 => match (*show).rivalTrainer.location {
            MAPSEC_SECRET_BASE => {
                sTVShowState = 8;
            }
            87 => match (*show).rivalTrainer.mapLayoutId {
                LAYOUT_SS_TIDAL_CORRIDOR | LAYOUT_SS_TIDAL_LOWER_DECK | LAYOUT_SS_TIDAL_ROOMS => {
                    sTVShowState = 10;
                }
                _ => {
                    sTVShowState = 9;
                }
            },
            _ => {
                sTVShowState = 7;
            }
        },
        7 => {
            TVShowConvertInternationalString(
                gStringVar1.as_mut_ptr(),
                (*show).rivalTrainer.playerName.as_mut_ptr(),
                (*show).rivalTrainer.language as i32,
            );
            ConvertIntToDecimalString(1, (*show).rivalTrainer.dexCount as i32);
            GetMapName(
                gStringVar3.as_mut_ptr(),
                (*show).rivalTrainer.location as u16,
                0,
            );
            if (*show).rivalTrainer.badgeCount != 0 {
                sTVShowState = 1;
            } else {
                sTVShowState = 2;
            }
        }
        8 => {
            TVShowConvertInternationalString(
                gStringVar1.as_mut_ptr(),
                (*show).rivalTrainer.playerName.as_mut_ptr(),
                (*show).rivalTrainer.language as i32,
            );
            ConvertIntToDecimalString(1, (*show).rivalTrainer.dexCount as i32);
            if (*show).rivalTrainer.badgeCount != 0 {
                sTVShowState = 1;
            } else {
                sTVShowState = 2;
            }
        }
        9 => {
            TVShowConvertInternationalString(
                gStringVar1.as_mut_ptr(),
                (*show).rivalTrainer.playerName.as_mut_ptr(),
                (*show).rivalTrainer.language as i32,
            );
            ConvertIntToDecimalString(1, (*show).rivalTrainer.dexCount as i32);
            if (*show).rivalTrainer.badgeCount != 0 {
                sTVShowState = 1;
            } else {
                sTVShowState = 2;
            }
        }
        10 => {
            TVShowConvertInternationalString(
                gStringVar1.as_mut_ptr(),
                (*show).rivalTrainer.playerName.as_mut_ptr(),
                (*show).rivalTrainer.language as i32,
            );
            ConvertIntToDecimalString(1, (*show).rivalTrainer.dexCount as i32);
            if (*show).rivalTrainer.badgeCount != 0 {
                sTVShowState = 1;
            } else {
                sTVShowState = 2;
            }
        }
        1 => {
            ConvertIntToDecimalString(0, (*show).rivalTrainer.badgeCount as i32);
            if FlagGet(FLAG_LANDMARK_BATTLE_FRONTIER) != 0 {
                if (*show).rivalTrainer.nSilverSymbols != 0
                    || (*show).rivalTrainer.nGoldSymbols != 0
                {
                    sTVShowState = 4;
                } else {
                    sTVShowState = 3;
                }
            } else {
                sTVShowState = 6;
            }
        }
        2 => {
            if FlagGet(FLAG_LANDMARK_BATTLE_FRONTIER) != 0 {
                if (*show).rivalTrainer.nSilverSymbols != 0
                    || (*show).rivalTrainer.nGoldSymbols != 0
                {
                    sTVShowState = 4;
                } else {
                    sTVShowState = 3;
                }
            } else {
                sTVShowState = 6;
            }
        }
        3 => {
            if (*show).rivalTrainer.battlePoints == 0 {
                sTVShowState = 6;
            } else {
                sTVShowState = 5;
            }
        }
        4 => {
            ConvertIntToDecimalString(0, (*show).rivalTrainer.nGoldSymbols as i32);
            ConvertIntToDecimalString(1, (*show).rivalTrainer.nSilverSymbols as i32);
            if (*show).rivalTrainer.battlePoints == 0 {
                sTVShowState = 6;
            } else {
                sTVShowState = 5;
            }
        }
        5 => {
            ConvertIntToDecimalString(0, (*show).rivalTrainer.battlePoints as i32);
            sTVShowState = 6;
        }
        6 => {
            TVShowConvertInternationalString(
                gStringVar1.as_mut_ptr(),
                (*show).rivalTrainer.playerName.as_mut_ptr(),
                (*show).rivalTrainer.language as i32,
            );
            TVShowDone();
        }
        _ => {}
    }
    ShowFieldMessage(sTVTodaysRivalTrainerTextGroup[state]);
}
pub(crate) unsafe extern "C" fn DoTVShowDewfordTrendWatcherNetwork() {
    let mut show: *mut TVShow = null_mut();
    let mut state: u8 = 0;
    show = &raw mut (*gSaveBlock1Ptr).tvShows[gSpecialVar_0x8004];
    gSpecialVar_Result = FALSE as u16;
    state = sTVShowState;
    match state {
        TRENDWATCHER_STATE_INTRO => {
            CopyEasyChatWord(gStringVar1.as_mut_ptr(), (*show).trendWatcher.words[0]);
            CopyEasyChatWord(gStringVar2.as_mut_ptr(), (*show).trendWatcher.words[1]);
            if (*show).trendWatcher.gender == MALE {
                sTVShowState = TRENDWATCHER_STATE_TAUGHT_MALE;
            } else {
                sTVShowState = TRENDWATCHER_STATE_TAUGHT_FEMALE;
            }
        }
        TRENDWATCHER_STATE_TAUGHT_MALE | TRENDWATCHER_STATE_TAUGHT_FEMALE => {
            CopyEasyChatWord(gStringVar1.as_mut_ptr(), (*show).trendWatcher.words[0]);
            CopyEasyChatWord(gStringVar2.as_mut_ptr(), (*show).trendWatcher.words[1]);
            TVShowConvertInternationalString(
                gStringVar3.as_mut_ptr(),
                (*show).trendWatcher.playerName.as_mut_ptr(),
                (*show).trendWatcher.language as i32,
            );
            sTVShowState = TRENDWATCHER_STATE_PHRASE_HOPELESS;
        }
        TRENDWATCHER_STATE_PHRASE_HOPELESS => {
            CopyEasyChatWord(gStringVar1.as_mut_ptr(), (*show).trendWatcher.words[0]);
            CopyEasyChatWord(gStringVar2.as_mut_ptr(), (*show).trendWatcher.words[1]);
            if (*show).trendWatcher.gender == MALE {
                sTVShowState = TRENDWATCHER_STATE_BIGGER_MALE;
            } else {
                sTVShowState = TRENDWATCHER_STATE_BIGGER_FEMALE;
            }
        }
        TRENDWATCHER_STATE_BIGGER_MALE | TRENDWATCHER_STATE_BIGGER_FEMALE => {
            CopyEasyChatWord(gStringVar1.as_mut_ptr(), (*show).trendWatcher.words[0]);
            CopyEasyChatWord(gStringVar2.as_mut_ptr(), (*show).trendWatcher.words[1]);
            TVShowConvertInternationalString(
                gStringVar3.as_mut_ptr(),
                (*show).trendWatcher.playerName.as_mut_ptr(),
                (*show).trendWatcher.language as i32,
            );
            sTVShowState = TRENDWATCHER_STATE_OUTRO;
        }
        TRENDWATCHER_STATE_OUTRO => {
            CopyEasyChatWord(gStringVar1.as_mut_ptr(), (*show).trendWatcher.words[0]);
            CopyEasyChatWord(gStringVar2.as_mut_ptr(), (*show).trendWatcher.words[1]);
            TVShowDone();
        }
        _ => {}
    }
    ShowFieldMessage(sTVDewfordTrendWatcherNetworkTextGroup[state]);
}
pub(crate) unsafe extern "C" fn DoTVShowHoennTreasureInvestigators() {
    let mut show: *mut TVShow = null_mut();
    let mut state: u8 = 0;
    show = &raw mut (*gSaveBlock1Ptr).tvShows[gSpecialVar_0x8004];
    gSpecialVar_Result = FALSE as u16;
    state = sTVShowState;
    match state {
        0 => {
            StringCopy(
                gStringVar1.as_mut_ptr(),
                GetItemName((*show).treasureInvestigators.item),
            );
            if (*show).treasureInvestigators.location == MAPSEC_DYNAMIC as u8 {
                match (*show).treasureInvestigators.mapLayoutId {
                    LAYOUT_SS_TIDAL_CORRIDOR
                    | LAYOUT_SS_TIDAL_LOWER_DECK
                    | LAYOUT_SS_TIDAL_ROOMS => {
                        sTVShowState = 2;
                    }
                    _ => {
                        sTVShowState = 1;
                    }
                }
            } else {
                sTVShowState = 1;
            }
        }
        1 => {
            StringCopy(
                gStringVar1.as_mut_ptr(),
                GetItemName((*show).treasureInvestigators.item),
            );
            TVShowConvertInternationalString(
                gStringVar2.as_mut_ptr(),
                (*show).treasureInvestigators.playerName.as_mut_ptr(),
                (*show).treasureInvestigators.language as i32,
            );
            GetMapName(
                gStringVar3.as_mut_ptr(),
                (*show).treasureInvestigators.location as u16,
                0,
            );
            TVShowDone();
        }
        2 => {
            StringCopy(
                gStringVar1.as_mut_ptr(),
                GetItemName((*show).treasureInvestigators.item),
            );
            TVShowConvertInternationalString(
                gStringVar2.as_mut_ptr(),
                (*show).treasureInvestigators.playerName.as_mut_ptr(),
                (*show).treasureInvestigators.language as i32,
            );
            TVShowDone();
        }
        _ => {}
    }
    ShowFieldMessage(sTVHoennTreasureInvestisatorsTextGroup[state]);
}
pub(crate) unsafe extern "C" fn DoTVShowFindThatGamer() {
    let mut show: *mut TVShow = null_mut();
    let mut state: u8 = 0;
    show = &raw mut (*gSaveBlock1Ptr).tvShows[gSpecialVar_0x8004];
    gSpecialVar_Result = FALSE as u16;
    state = sTVShowState;
    match state {
        0 => {
            TVShowConvertInternationalString(
                gStringVar1.as_mut_ptr(),
                (*show).findThatGamer.playerName.as_mut_ptr(),
                (*show).findThatGamer.language as i32,
            );
            match (*show).findThatGamer.whichGame {
                0 => {
                    StringCopy(gStringVar2.as_mut_ptr(), gText_Slots.as_ptr().cast_mut());
                }
                1 => {
                    StringCopy(gStringVar2.as_mut_ptr(), gText_Roulette.as_ptr().cast_mut());
                }
                _ => {}
            }
            if (*show).findThatGamer.won == TRUE {
                sTVShowState = 1;
            } else {
                sTVShowState = 2;
            }
        }
        1 => {
            TVShowConvertInternationalString(
                gStringVar1.as_mut_ptr(),
                (*show).findThatGamer.playerName.as_mut_ptr(),
                (*show).findThatGamer.language as i32,
            );
            match (*show).findThatGamer.whichGame {
                0 => {
                    StringCopy(gStringVar2.as_mut_ptr(), gText_Slots.as_ptr().cast_mut());
                }
                1 => {
                    StringCopy(gStringVar2.as_mut_ptr(), gText_Roulette.as_ptr().cast_mut());
                }
                _ => {}
            }
            ConvertIntToDecimalString(2, (*show).findThatGamer.nCoins as i32);
            TVShowDone();
        }
        2 => {
            TVShowConvertInternationalString(
                gStringVar1.as_mut_ptr(),
                (*show).findThatGamer.playerName.as_mut_ptr(),
                (*show).findThatGamer.language as i32,
            );
            match (*show).findThatGamer.whichGame {
                0 => {
                    StringCopy(gStringVar2.as_mut_ptr(), gText_Slots.as_ptr().cast_mut());
                }
                1 => {
                    StringCopy(gStringVar2.as_mut_ptr(), gText_Roulette.as_ptr().cast_mut());
                }
                _ => {}
            }
            ConvertIntToDecimalString(2, (*show).findThatGamer.nCoins as i32);
            sTVShowState = 3;
        }
        3 => {
            TVShowConvertInternationalString(
                gStringVar1.as_mut_ptr(),
                (*show).findThatGamer.playerName.as_mut_ptr(),
                (*show).findThatGamer.language as i32,
            );
            match (*show).findThatGamer.whichGame {
                0 => {
                    StringCopy(gStringVar2.as_mut_ptr(), gText_Roulette.as_ptr().cast_mut());
                }
                1 => {
                    StringCopy(gStringVar2.as_mut_ptr(), gText_Slots.as_ptr().cast_mut());
                }
                _ => {}
            }
            TVShowDone();
        }
        _ => {}
    }
    ShowFieldMessage(sTVFindThatGamerTextGroup[state]);
}
pub(crate) unsafe extern "C" fn DoTVShowBreakingNewsTV() {
    let mut show: *mut TVShow = null_mut();
    let mut state: u8 = 0;
    show = &raw mut (*gSaveBlock1Ptr).tvShows[gSpecialVar_0x8004];
    gSpecialVar_Result = FALSE as u16;
    state = sTVShowState;
    match state {
        0 => {
            if (*show).breakingNews.outcome == 0 {
                sTVShowState = 1;
            } else {
                sTVShowState = 5;
            }
        }
        1 => {
            TVShowConvertInternationalString(
                gStringVar1.as_mut_ptr(),
                (*show).breakingNews.playerName.as_mut_ptr(),
                (*show).breakingNews.language as i32,
            );
            StringCopy(
                gStringVar2.as_mut_ptr(),
                gSpeciesNames[(*show).breakingNews.lastOpponentSpecies]
                    .as_ptr()
                    .cast_mut(),
            );
            GetMapName(
                gStringVar3.as_mut_ptr(),
                (*show).breakingNews.location as u16,
                0,
            );
            sTVShowState = 2;
        }
        2 => {
            TVShowConvertInternationalString(
                gStringVar1.as_mut_ptr(),
                (*show).breakingNews.playerName.as_mut_ptr(),
                (*show).breakingNews.language as i32,
            );
            StringCopy(
                gStringVar2.as_mut_ptr(),
                gSpeciesNames[(*show).breakingNews.lastOpponentSpecies]
                    .as_ptr()
                    .cast_mut(),
            );
            StringCopy(
                gStringVar3.as_mut_ptr(),
                gSpeciesNames[(*show).breakingNews.poke1Species]
                    .as_ptr()
                    .cast_mut(),
            );
            sTVShowState = 3;
        }
        3 => {
            ConvertIntToDecimalString(0, (*show).breakingNews.balls as i32);
            StringCopy(
                gStringVar2.as_mut_ptr(),
                GetItemName((*show).breakingNews.caughtMonBall),
            );
            sTVShowState = 4;
        }
        4 => {
            TVShowConvertInternationalString(
                gStringVar1.as_mut_ptr(),
                (*show).breakingNews.playerName.as_mut_ptr(),
                (*show).breakingNews.language as i32,
            );
            GetMapName(
                gStringVar2.as_mut_ptr(),
                (*show).breakingNews.location as u16,
                0,
            );
            TVShowDone();
        }
        5 => {
            TVShowConvertInternationalString(
                gStringVar1.as_mut_ptr(),
                (*show).breakingNews.playerName.as_mut_ptr(),
                (*show).breakingNews.language as i32,
            );
            StringCopy(
                gStringVar2.as_mut_ptr(),
                gSpeciesNames[(*show).breakingNews.lastOpponentSpecies]
                    .as_ptr()
                    .cast_mut(),
            );
            GetMapName(
                gStringVar3.as_mut_ptr(),
                (*show).breakingNews.location as u16,
                0,
            );
            sTVShowState = 6;
        }
        6 => {
            TVShowConvertInternationalString(
                gStringVar1.as_mut_ptr(),
                (*show).breakingNews.playerName.as_mut_ptr(),
                (*show).breakingNews.language as i32,
            );
            StringCopy(
                gStringVar2.as_mut_ptr(),
                gSpeciesNames[(*show).breakingNews.lastOpponentSpecies]
                    .as_ptr()
                    .cast_mut(),
            );
            StringCopy(
                gStringVar3.as_mut_ptr(),
                gSpeciesNames[(*show).breakingNews.poke1Species]
                    .as_ptr()
                    .cast_mut(),
            );
            match (*show).breakingNews.outcome {
                1 => {
                    if (*show).breakingNews.lastUsedMove == MOVE_NONE {
                        sTVShowState = 12;
                    } else {
                        sTVShowState = 7;
                    }
                }
                2 => {
                    sTVShowState = 9;
                }
                3 => {
                    sTVShowState = 10;
                }
                _ => {}
            }
        }
        7 => {
            StringCopy(
                gStringVar1.as_mut_ptr(),
                gMoveNames[(*show).breakingNews.lastUsedMove]
                    .as_ptr()
                    .cast_mut(),
            );
            StringCopy(
                gStringVar2.as_mut_ptr(),
                gSpeciesNames[(*show).breakingNews.poke1Species]
                    .as_ptr()
                    .cast_mut(),
            );
            sTVShowState = 8;
        }
        12 => {
            TVShowConvertInternationalString(
                gStringVar1.as_mut_ptr(),
                (*show).breakingNews.playerName.as_mut_ptr(),
                (*show).breakingNews.language as i32,
            );
            StringCopy(
                gStringVar2.as_mut_ptr(),
                gSpeciesNames[(*show).breakingNews.lastOpponentSpecies]
                    .as_ptr()
                    .cast_mut(),
            );
            StringCopy(
                gStringVar3.as_mut_ptr(),
                gSpeciesNames[(*show).breakingNews.poke1Species]
                    .as_ptr()
                    .cast_mut(),
            );
            sTVShowState = 8;
        }
        8 => {
            TVShowConvertInternationalString(
                gStringVar1.as_mut_ptr(),
                (*show).breakingNews.playerName.as_mut_ptr(),
                (*show).breakingNews.language as i32,
            );
            GetMapName(
                gStringVar2.as_mut_ptr(),
                (*show).breakingNews.location as u16,
                0,
            );
            sTVShowState = 11;
        }
        9 | 10 => {
            TVShowConvertInternationalString(
                gStringVar1.as_mut_ptr(),
                (*show).breakingNews.playerName.as_mut_ptr(),
                (*show).breakingNews.language as i32,
            );
            StringCopy(
                gStringVar2.as_mut_ptr(),
                gSpeciesNames[(*show).breakingNews.lastOpponentSpecies]
                    .as_ptr()
                    .cast_mut(),
            );
            GetMapName(
                gStringVar3.as_mut_ptr(),
                (*show).breakingNews.location as u16,
                0,
            );
            sTVShowState = 11;
        }
        11 => {
            TVShowConvertInternationalString(
                gStringVar1.as_mut_ptr(),
                (*show).breakingNews.playerName.as_mut_ptr(),
                (*show).breakingNews.language as i32,
            );
            TVShowDone();
        }
        _ => {}
    }
    ShowFieldMessage(sTVBreakingNewsTextGroup[state]);
}
pub(crate) unsafe extern "C" fn DoTVShowSecretBaseVisit() {
    let mut show: *mut TVShow = null_mut();
    let mut state: u8 = 0;
    show = &raw mut (*gSaveBlock1Ptr).tvShows[gSpecialVar_0x8004];
    gSpecialVar_Result = FALSE as u16;
    state = sTVShowState;
    match state {
        0 => {
            TVShowConvertInternationalString(
                gStringVar1.as_mut_ptr(),
                (*show).secretBaseVisit.playerName.as_mut_ptr(),
                (*show).secretBaseVisit.language as i32,
            );
            if (*show).secretBaseVisit.numDecorations == 0 {
                sTVShowState = 2;
            } else {
                sTVShowState = 1;
            }
        }
        1 => {
            StringCopy(
                gStringVar2.as_mut_ptr(),
                gDecorations[(*show).secretBaseVisit.decorations[0]]
                    .name
                    .as_ptr()
                    .cast_mut(),
            );
            if (*show).secretBaseVisit.numDecorations == 1 {
                sTVShowState = 4;
            } else {
                sTVShowState = 3;
            }
        }
        3 => {
            StringCopy(
                gStringVar2.as_mut_ptr(),
                gDecorations[(*show).secretBaseVisit.decorations[1]]
                    .name
                    .as_ptr()
                    .cast_mut(),
            );
            match (*show).secretBaseVisit.numDecorations {
                2 => {
                    sTVShowState = 7;
                }
                3 => {
                    sTVShowState = 6;
                }
                4 => {
                    sTVShowState = 5;
                }
                _ => {}
            }
        }
        5 => {
            StringCopy(
                gStringVar2.as_mut_ptr(),
                gDecorations[(*show).secretBaseVisit.decorations[2]]
                    .name
                    .as_ptr()
                    .cast_mut(),
            );
            StringCopy(
                gStringVar3.as_mut_ptr(),
                gDecorations[(*show).secretBaseVisit.decorations[3]]
                    .name
                    .as_ptr()
                    .cast_mut(),
            );
            sTVShowState = 8;
        }
        6 => {
            StringCopy(
                gStringVar2.as_mut_ptr(),
                gDecorations[(*show).secretBaseVisit.decorations[2]]
                    .name
                    .as_ptr()
                    .cast_mut(),
            );
            sTVShowState = 8;
        }
        2 | 4 | 7 => {
            sTVShowState = 8;
        }
        8 => {
            TVShowConvertInternationalString(
                gStringVar1.as_mut_ptr(),
                (*show).secretBaseVisit.playerName.as_mut_ptr(),
                (*show).secretBaseVisit.language as i32,
            );
            if (*show).secretBaseVisit.avgLevel < 25 {
                sTVShowState = 12;
            } else if (*show).secretBaseVisit.avgLevel < 50 {
                sTVShowState = 11;
            } else if (*show).secretBaseVisit.avgLevel < 70 {
                sTVShowState = 10;
            } else {
                sTVShowState = 9;
            }
        }
        9 | 10 | 11 | 12 => {
            TVShowConvertInternationalString(
                gStringVar1.as_mut_ptr(),
                (*show).secretBaseVisit.playerName.as_mut_ptr(),
                (*show).secretBaseVisit.language as i32,
            );
            StringCopy(
                gStringVar2.as_mut_ptr(),
                gSpeciesNames[(*show).secretBaseVisit.species]
                    .as_ptr()
                    .cast_mut(),
            );
            StringCopy(
                gStringVar3.as_mut_ptr(),
                gMoveNames[(*show).secretBaseVisit.r#move]
                    .as_ptr()
                    .cast_mut(),
            );
            sTVShowState = 13;
        }
        13 => {
            TVShowDone();
        }
        _ => {}
    }
    ShowFieldMessage(sTVSecretBaseVisitTextGroup[state]);
}
pub(crate) unsafe extern "C" fn DoTVShowPokemonLotteryWinnerFlashReport() {
    let mut show: *mut TVShow = null_mut();
    let mut state: u8 = 0;
    show = &raw mut (*gSaveBlock1Ptr).tvShows[gSpecialVar_0x8004];
    gSpecialVar_Result = FALSE as u16;
    state = sTVShowState;
    TVShowConvertInternationalString(
        gStringVar1.as_mut_ptr(),
        (*show).lottoWinner.playerName.as_mut_ptr(),
        (*show).lottoWinner.language as i32,
    );
    if (*show).lottoWinner.whichPrize == 0 {
        StringCopy(gStringVar2.as_mut_ptr(), gText_Jackpot.as_ptr().cast_mut());
    } else if (*show).lottoWinner.whichPrize == 1 {
        StringCopy(gStringVar2.as_mut_ptr(), gText_First.as_ptr().cast_mut());
    } else if (*show).lottoWinner.whichPrize == 2 {
        StringCopy(gStringVar2.as_mut_ptr(), gText_Second.as_ptr().cast_mut());
    } else {
        StringCopy(gStringVar2.as_mut_ptr(), gText_Third.as_ptr().cast_mut());
    }
    StringCopy(
        gStringVar3.as_mut_ptr(),
        GetItemName((*show).lottoWinner.item),
    );
    TVShowDone();
    ShowFieldMessage(sTVPokemonLotteryWinnerFlashReportTextGroup[state]);
}
pub(crate) unsafe extern "C" fn DoTVShowThePokemonBattleSeminar() {
    let mut show: *mut TVShow = null_mut();
    let mut state: u8 = 0;
    show = &raw mut (*gSaveBlock1Ptr).tvShows[gSpecialVar_0x8004];
    gSpecialVar_Result = FALSE as u16;
    state = sTVShowState;
    match state {
        0 => {
            TVShowConvertInternationalString(
                gStringVar1.as_mut_ptr(),
                (*show).battleSeminar.playerName.as_mut_ptr(),
                (*show).battleSeminar.language as i32,
            );
            StringCopy(
                gStringVar2.as_mut_ptr(),
                gSpeciesNames[(*show).battleSeminar.species]
                    .as_ptr()
                    .cast_mut(),
            );
            StringCopy(
                gStringVar3.as_mut_ptr(),
                gSpeciesNames[(*show).battleSeminar.foeSpecies]
                    .as_ptr()
                    .cast_mut(),
            );
            sTVShowState = 1;
        }
        1 => {
            TVShowConvertInternationalString(
                gStringVar1.as_mut_ptr(),
                (*show).battleSeminar.playerName.as_mut_ptr(),
                (*show).battleSeminar.language as i32,
            );
            StringCopy(
                gStringVar2.as_mut_ptr(),
                gSpeciesNames[(*show).battleSeminar.foeSpecies]
                    .as_ptr()
                    .cast_mut(),
            );
            StringCopy(
                gStringVar3.as_mut_ptr(),
                gMoveNames[(*show).battleSeminar.r#move].as_ptr().cast_mut(),
            );
            sTVShowState = 2;
        }
        2 => {
            StringCopy(
                gStringVar1.as_mut_ptr(),
                gSpeciesNames[(*show).battleSeminar.species]
                    .as_ptr()
                    .cast_mut(),
            );
            match (*show).battleSeminar.nOtherMoves {
                1 => {
                    sTVShowState = 5;
                }
                2 => {
                    sTVShowState = 4;
                }
                3 => {
                    sTVShowState = 3;
                }
                _ => {
                    sTVShowState = 6;
                }
            }
        }
        3 => {
            StringCopy(
                gStringVar1.as_mut_ptr(),
                gMoveNames[(*show).battleSeminar.otherMoves[0]]
                    .as_ptr()
                    .cast_mut(),
            );
            StringCopy(
                gStringVar2.as_mut_ptr(),
                gMoveNames[(*show).battleSeminar.otherMoves[1]]
                    .as_ptr()
                    .cast_mut(),
            );
            StringCopy(
                gStringVar3.as_mut_ptr(),
                gMoveNames[(*show).battleSeminar.otherMoves[2]]
                    .as_ptr()
                    .cast_mut(),
            );
            sTVShowState = 6;
        }
        4 => {
            StringCopy(
                gStringVar1.as_mut_ptr(),
                gMoveNames[(*show).battleSeminar.otherMoves[0]]
                    .as_ptr()
                    .cast_mut(),
            );
            StringCopy(
                gStringVar2.as_mut_ptr(),
                gMoveNames[(*show).battleSeminar.otherMoves[1]]
                    .as_ptr()
                    .cast_mut(),
            );
            sTVShowState = 6;
        }
        5 => {
            StringCopy(
                gStringVar2.as_mut_ptr(),
                gMoveNames[(*show).battleSeminar.otherMoves[0]]
                    .as_ptr()
                    .cast_mut(),
            );
            sTVShowState = 6;
        }
        6 => {
            StringCopy(
                gStringVar1.as_mut_ptr(),
                gMoveNames[(*show).battleSeminar.betterMove]
                    .as_ptr()
                    .cast_mut(),
            );
            StringCopy(
                gStringVar2.as_mut_ptr(),
                gMoveNames[(*show).battleSeminar.r#move].as_ptr().cast_mut(),
            );
            TVShowDone();
        }
        _ => {}
    }
    ShowFieldMessage(sTVThePokemonBattleSeminarTextGroup[state]);
}
pub(crate) unsafe extern "C" fn DoTVShowTrainerFanClubSpecial() {
    let mut show: *mut TVShow = null_mut();
    let mut state: u8 = 0;
    show = &raw mut (*gSaveBlock1Ptr).tvShows[gSpecialVar_0x8004];
    gSpecialVar_Result = FALSE as u16;
    state = sTVShowState;
    match state {
        0 => {
            TVShowConvertInternationalString(
                gStringVar1.as_mut_ptr(),
                (*show).fanClubSpecial.idolName.as_mut_ptr(),
                (*show).fanClubSpecial.idolNameLanguage as i32,
            );
            TVShowConvertInternationalString(
                gStringVar2.as_mut_ptr(),
                (*show).fanClubSpecial.playerName.as_mut_ptr(),
                (*show).fanClubSpecial.language as i32,
            );
            CopyEasyChatWord(gStringVar3.as_mut_ptr(), (*show).fanClubSpecial.words[0]);
            if (*show).fanClubSpecial.score >= 90 {
                sTVShowState = 1;
            } else if (*show).fanClubSpecial.score >= 70 {
                sTVShowState = 2;
            } else if (*show).fanClubSpecial.score >= 30 {
                sTVShowState = 3;
            } else {
                sTVShowState = 4;
            }
        }
        1 => {
            TVShowConvertInternationalString(
                gStringVar1.as_mut_ptr(),
                (*show).fanClubSpecial.idolName.as_mut_ptr(),
                (*show).fanClubSpecial.idolNameLanguage as i32,
            );
            TVShowConvertInternationalString(
                gStringVar2.as_mut_ptr(),
                (*show).fanClubSpecial.playerName.as_mut_ptr(),
                (*show).fanClubSpecial.language as i32,
            );
            ConvertIntToDecimalString(2, (*show).fanClubSpecial.score as i32);
            sTVShowState = 5;
        }
        2 => {
            TVShowConvertInternationalString(
                gStringVar1.as_mut_ptr(),
                (*show).fanClubSpecial.idolName.as_mut_ptr(),
                (*show).fanClubSpecial.idolNameLanguage as i32,
            );
            TVShowConvertInternationalString(
                gStringVar2.as_mut_ptr(),
                (*show).fanClubSpecial.playerName.as_mut_ptr(),
                (*show).fanClubSpecial.language as i32,
            );
            ConvertIntToDecimalString(2, (*show).fanClubSpecial.score as i32);
            sTVShowState = 5;
        }
        3 => {
            TVShowConvertInternationalString(
                gStringVar1.as_mut_ptr(),
                (*show).fanClubSpecial.idolName.as_mut_ptr(),
                (*show).fanClubSpecial.idolNameLanguage as i32,
            );
            TVShowConvertInternationalString(
                gStringVar2.as_mut_ptr(),
                (*show).fanClubSpecial.playerName.as_mut_ptr(),
                (*show).fanClubSpecial.language as i32,
            );
            ConvertIntToDecimalString(2, (*show).fanClubSpecial.score as i32);
            sTVShowState = 5;
        }
        4 => {
            TVShowConvertInternationalString(
                gStringVar1.as_mut_ptr(),
                (*show).fanClubSpecial.idolName.as_mut_ptr(),
                (*show).fanClubSpecial.idolNameLanguage as i32,
            );
            TVShowConvertInternationalString(
                gStringVar2.as_mut_ptr(),
                (*show).fanClubSpecial.playerName.as_mut_ptr(),
                (*show).fanClubSpecial.language as i32,
            );
            ConvertIntToDecimalString(2, (*show).fanClubSpecial.score as i32);
            sTVShowState = 5;
        }
        5 => {
            TVShowConvertInternationalString(
                gStringVar1.as_mut_ptr(),
                (*show).fanClubSpecial.idolName.as_mut_ptr(),
                (*show).fanClubSpecial.idolNameLanguage as i32,
            );
            TVShowConvertInternationalString(
                gStringVar2.as_mut_ptr(),
                (*show).fanClubSpecial.playerName.as_mut_ptr(),
                (*show).fanClubSpecial.language as i32,
            );
            CopyEasyChatWord(gStringVar3.as_mut_ptr(), (*show).fanClubSpecial.words[0]);
            TVShowDone();
        }
        _ => {}
    }
    ShowFieldMessage(sTVTrainerFanClubSpecialTextGroup[state]);
}
pub(crate) unsafe extern "C" fn DoTVShowTrainerFanClub() {
    let mut show: *mut TVShow = null_mut();
    let mut state: u8 = 0;
    let mut playerId: u32 = 0;
    show = &raw mut (*gSaveBlock1Ptr).tvShows[gSpecialVar_0x8004];
    gSpecialVar_Result = FALSE as u16;
    state = sTVShowState;
    match state {
        0 => {
            TVShowConvertInternationalString(
                gStringVar1.as_mut_ptr(),
                (*show).trainerFanClub.playerName.as_mut_ptr(),
                (*show).trainerFanClub.language as i32,
            );
            playerId =
                (((*show).common.trainerIdHi as u32) << 8) + (*show).common.trainerIdLo as u32;
            match playerId % 10 {
                0 => {
                    sTVShowState = 1;
                }
                1 => {
                    sTVShowState = 2;
                }
                2 => {
                    sTVShowState = 3;
                }
                3 => {
                    sTVShowState = 4;
                }
                4 => {
                    sTVShowState = 5;
                }
                5 => {
                    sTVShowState = 6;
                }
                6 => {
                    sTVShowState = 7;
                }
                7 => {
                    sTVShowState = 8;
                }
                8 => {
                    sTVShowState = 9;
                }
                9 => {
                    sTVShowState = 10;
                }
                _ => {}
            }
        }
        1 => {
            sTVShowState = 11;
        }
        2 => {
            sTVShowState = 11;
        }
        3 => {
            sTVShowState = 11;
        }
        4 => {
            sTVShowState = 11;
        }
        5 => {
            sTVShowState = 11;
        }
        6 => {
            sTVShowState = 11;
        }
        7 => {
            sTVShowState = 11;
        }
        8 => {
            sTVShowState = 11;
        }
        9 => {
            sTVShowState = 11;
        }
        10 => {
            sTVShowState = 11;
        }
        11 => {
            TVShowConvertInternationalString(
                gStringVar1.as_mut_ptr(),
                (*show).trainerFanClub.playerName.as_mut_ptr(),
                (*show).trainerFanClub.language as i32,
            );
            CopyEasyChatWord(gStringVar2.as_mut_ptr(), (*show).trainerFanClub.words[0]);
            CopyEasyChatWord(gStringVar3.as_mut_ptr(), (*show).trainerFanClub.words[1]);
            TVShowDone();
        }
        _ => {}
    }
    ShowFieldMessage(sTVTrainerFanClubTextGroup[state]);
}
pub(crate) unsafe extern "C" fn DoTVShowSpotTheCuties() {
    let mut show: *mut TVShow = null_mut();
    let mut state: u8 = 0;
    show = &raw mut (*gSaveBlock1Ptr).tvShows[gSpecialVar_0x8004];
    gSpecialVar_Result = FALSE as u16;
    state = sTVShowState;
    match state {
        SPOTCUTIES_STATE_INTRO => {
            TVShowConvertInternationalString(
                gStringVar1.as_mut_ptr(),
                (*show).cuties.playerName.as_mut_ptr(),
                (*show).cuties.language as i32,
            );
            TVShowConvertInternationalString(
                gStringVar2.as_mut_ptr(),
                (*show).cuties.nickname.as_mut_ptr(),
                (*show).cuties.pokemonNameLanguage as i32,
            );
            if (*show).cuties.nRibbons < 10 {
                sTVShowState = SPOTCUTIES_STATE_RIBBONS_LOW;
            } else if (*show).cuties.nRibbons < 20 {
                sTVShowState = SPOTCUTIES_STATE_RIBBONS_MID;
            } else {
                sTVShowState = SPOTCUTIES_STATE_RIBBONS_HIGH;
            }
        }
        SPOTCUTIES_STATE_RIBBONS_LOW
        | SPOTCUTIES_STATE_RIBBONS_MID
        | SPOTCUTIES_STATE_RIBBONS_HIGH => {
            TVShowConvertInternationalString(
                gStringVar1.as_mut_ptr(),
                (*show).cuties.playerName.as_mut_ptr(),
                (*show).cuties.language as i32,
            );
            TVShowConvertInternationalString(
                gStringVar2.as_mut_ptr(),
                (*show).cuties.nickname.as_mut_ptr(),
                (*show).cuties.pokemonNameLanguage as i32,
            );
            ConvertIntToDecimalString(2, (*show).cuties.nRibbons as i32);
            sTVShowState = SPOTCUTIES_STATE_RIBBON_INTRO;
        }
        SPOTCUTIES_STATE_RIBBON_INTRO => {
            TVShowConvertInternationalString(
                gStringVar2.as_mut_ptr(),
                (*show).cuties.nickname.as_mut_ptr(),
                (*show).cuties.pokemonNameLanguage as i32,
            );
            match (*show).cuties.selectedRibbon {
                CHAMPION_RIBBON => {
                    sTVShowState = SPOTCUTIES_STATE_RIBBON_CHAMPION;
                }
                COOL_RIBBON_NORMAL | COOL_RIBBON_SUPER | COOL_RIBBON_HYPER | COOL_RIBBON_MASTER => {
                    sTVShowState = SPOTCUTIES_STATE_RIBBON_COOL;
                }
                BEAUTY_RIBBON_NORMAL | BEAUTY_RIBBON_SUPER | BEAUTY_RIBBON_HYPER
                | BEAUTY_RIBBON_MASTER => {
                    sTVShowState = SPOTCUTIES_STATE_RIBBON_BEAUTY;
                }
                CUTE_RIBBON_NORMAL | CUTE_RIBBON_SUPER | CUTE_RIBBON_HYPER | CUTE_RIBBON_MASTER => {
                    sTVShowState = SPOTCUTIES_STATE_RIBBON_CUTE;
                }
                SMART_RIBBON_NORMAL | SMART_RIBBON_SUPER | SMART_RIBBON_HYPER
                | SMART_RIBBON_MASTER => {
                    sTVShowState = SPOTCUTIES_STATE_RIBBON_SMART;
                }
                TOUGH_RIBBON_NORMAL | TOUGH_RIBBON_SUPER | TOUGH_RIBBON_HYPER
                | TOUGH_RIBBON_MASTER => {
                    sTVShowState = SPOTCUTIES_STATE_RIBBON_TOUGH;
                }
                WINNING_RIBBON => {
                    sTVShowState = SPOTCUTIES_STATE_RIBBON_WINNING;
                }
                VICTORY_RIBBON => {
                    sTVShowState = SPOTCUTIES_STATE_RIBBON_VICTORY;
                }
                ARTIST_RIBBON => {
                    sTVShowState = SPOTCUTIES_STATE_RIBBON_ARTIST;
                }
                EFFORT_RIBBON => {
                    sTVShowState = SPOTCUTIES_STATE_RIBBON_EFFORT;
                }
                _ => {}
            }
        }
        SPOTCUTIES_STATE_RIBBON_CHAMPION
        | SPOTCUTIES_STATE_RIBBON_COOL
        | SPOTCUTIES_STATE_RIBBON_BEAUTY
        | SPOTCUTIES_STATE_RIBBON_CUTE
        | SPOTCUTIES_STATE_RIBBON_SMART
        | SPOTCUTIES_STATE_RIBBON_TOUGH
        | SPOTCUTIES_STATE_RIBBON_WINNING
        | SPOTCUTIES_STATE_RIBBON_VICTORY
        | SPOTCUTIES_STATE_RIBBON_ARTIST
        | SPOTCUTIES_STATE_RIBBON_EFFORT => {
            TVShowConvertInternationalString(
                gStringVar2.as_mut_ptr(),
                (*show).cuties.nickname.as_mut_ptr(),
                (*show).cuties.pokemonNameLanguage as i32,
            );
            sTVShowState = SPOTCUTIES_STATE_OUTRO;
        }
        SPOTCUTIES_STATE_OUTRO => {
            TVShowDone();
        }
        _ => {}
    }
    ShowFieldMessage(sTVCutiesTextGroup[state]);
}
pub(crate) unsafe extern "C" fn DoTVShowPokemonNewsBattleFrontier() {
    let mut show: *mut TVShow = null_mut();
    let mut state: u8 = 0;
    show = &raw mut (*gSaveBlock1Ptr).tvShows[gSpecialVar_0x8004];
    gSpecialVar_Result = FALSE as u16;
    state = sTVShowState;
    match state {
        0 => match (*show).frontier.facilityAndMode {
            1 => {
                sTVShowState = 1;
            }
            2 => {
                sTVShowState = 2;
            }
            3 => {
                sTVShowState = 3;
            }
            4 => {
                sTVShowState = 4;
            }
            5 => {
                sTVShowState = 5;
            }
            6 => {
                sTVShowState = 6;
            }
            7 => {
                sTVShowState = 7;
            }
            8 => {
                sTVShowState = 8;
            }
            9 => {
                sTVShowState = 9;
            }
            10 => {
                sTVShowState = 10;
            }
            11 => {
                sTVShowState = 11;
            }
            12 => {
                sTVShowState = 12;
            }
            13 => {
                sTVShowState = 13;
            }
            _ => {}
        },
        1 => {
            TVShowConvertInternationalString(
                gStringVar1.as_mut_ptr(),
                (*show).frontier.playerName.as_mut_ptr(),
                (*show).frontier.language as i32,
            );
            ConvertIntToDecimalString(1, (*show).frontier.winStreak as i32);
            sTVShowState = 14;
        }
        2 => {
            TVShowConvertInternationalString(
                gStringVar1.as_mut_ptr(),
                (*show).frontier.playerName.as_mut_ptr(),
                (*show).frontier.language as i32,
            );
            ConvertIntToDecimalString(1, (*show).frontier.winStreak as i32);
            sTVShowState = 16;
        }
        3 => {
            TVShowConvertInternationalString(
                gStringVar1.as_mut_ptr(),
                (*show).frontier.playerName.as_mut_ptr(),
                (*show).frontier.language as i32,
            );
            ConvertIntToDecimalString(1, (*show).frontier.winStreak as i32);
            sTVShowState = 15;
        }
        4 => {
            TVShowConvertInternationalString(
                gStringVar1.as_mut_ptr(),
                (*show).frontier.playerName.as_mut_ptr(),
                (*show).frontier.language as i32,
            );
            ConvertIntToDecimalString(1, (*show).frontier.winStreak as i32);
            sTVShowState = 15;
        }
        5 => {
            TVShowConvertInternationalString(
                gStringVar1.as_mut_ptr(),
                (*show).frontier.playerName.as_mut_ptr(),
                (*show).frontier.language as i32,
            );
            ConvertIntToDecimalString(1, (*show).frontier.winStreak as i32);
            sTVShowState = 14;
        }
        6 => {
            TVShowConvertInternationalString(
                gStringVar1.as_mut_ptr(),
                (*show).frontier.playerName.as_mut_ptr(),
                (*show).frontier.language as i32,
            );
            ConvertIntToDecimalString(1, (*show).frontier.winStreak as i32);
            sTVShowState = 14;
        }
        7 => {
            TVShowConvertInternationalString(
                gStringVar1.as_mut_ptr(),
                (*show).frontier.playerName.as_mut_ptr(),
                (*show).frontier.language as i32,
            );
            ConvertIntToDecimalString(1, (*show).frontier.winStreak as i32);
            sTVShowState = 14;
        }
        8 => {
            TVShowConvertInternationalString(
                gStringVar1.as_mut_ptr(),
                (*show).frontier.playerName.as_mut_ptr(),
                (*show).frontier.language as i32,
            );
            ConvertIntToDecimalString(1, (*show).frontier.winStreak as i32);
            sTVShowState = 14;
        }
        9 => {
            TVShowConvertInternationalString(
                gStringVar1.as_mut_ptr(),
                (*show).frontier.playerName.as_mut_ptr(),
                (*show).frontier.language as i32,
            );
            ConvertIntToDecimalString(1, (*show).frontier.winStreak as i32);
            sTVShowState = 14;
        }
        10 => {
            TVShowConvertInternationalString(
                gStringVar1.as_mut_ptr(),
                (*show).frontier.playerName.as_mut_ptr(),
                (*show).frontier.language as i32,
            );
            ConvertIntToDecimalString(1, (*show).frontier.winStreak as i32);
            sTVShowState = 14;
        }
        11 => {
            TVShowConvertInternationalString(
                gStringVar1.as_mut_ptr(),
                (*show).frontier.playerName.as_mut_ptr(),
                (*show).frontier.language as i32,
            );
            ConvertIntToDecimalString(1, (*show).frontier.winStreak as i32);
            sTVShowState = 14;
        }
        12 => {
            TVShowConvertInternationalString(
                gStringVar1.as_mut_ptr(),
                (*show).frontier.playerName.as_mut_ptr(),
                (*show).frontier.language as i32,
            );
            ConvertIntToDecimalString(1, (*show).frontier.winStreak as i32);
            sTVShowState = 14;
        }
        13 => {
            TVShowConvertInternationalString(
                gStringVar1.as_mut_ptr(),
                (*show).frontier.playerName.as_mut_ptr(),
                (*show).frontier.language as i32,
            );
            ConvertIntToDecimalString(1, (*show).frontier.winStreak as i32);
            sTVShowState = 14;
        }
        14 => {
            StringCopy(
                gStringVar1.as_mut_ptr(),
                gSpeciesNames[(*show).frontier.species1].as_ptr().cast_mut(),
            );
            StringCopy(
                gStringVar2.as_mut_ptr(),
                gSpeciesNames[(*show).frontier.species2].as_ptr().cast_mut(),
            );
            StringCopy(
                gStringVar3.as_mut_ptr(),
                gSpeciesNames[(*show).frontier.species3].as_ptr().cast_mut(),
            );
            sTVShowState = 18;
        }
        15 => {
            StringCopy(
                gStringVar1.as_mut_ptr(),
                gSpeciesNames[(*show).frontier.species1].as_ptr().cast_mut(),
            );
            StringCopy(
                gStringVar2.as_mut_ptr(),
                gSpeciesNames[(*show).frontier.species2].as_ptr().cast_mut(),
            );
            sTVShowState = 18;
        }
        16 => {
            StringCopy(
                gStringVar1.as_mut_ptr(),
                gSpeciesNames[(*show).frontier.species1].as_ptr().cast_mut(),
            );
            StringCopy(
                gStringVar2.as_mut_ptr(),
                gSpeciesNames[(*show).frontier.species2].as_ptr().cast_mut(),
            );
            StringCopy(
                gStringVar3.as_mut_ptr(),
                gSpeciesNames[(*show).frontier.species3].as_ptr().cast_mut(),
            );
            sTVShowState = 17;
        }
        17 => {
            StringCopy(
                gStringVar1.as_mut_ptr(),
                gSpeciesNames[(*show).frontier.species4].as_ptr().cast_mut(),
            );
            sTVShowState = 18;
        }
        18 => {
            TVShowConvertInternationalString(
                gStringVar1.as_mut_ptr(),
                (*show).frontier.playerName.as_mut_ptr(),
                (*show).frontier.language as i32,
            );
            TVShowDone();
        }
        _ => {}
    }
    ShowFieldMessage(sTVPokemonNewsBattleFrontierTextGroup[state]);
}
pub(crate) unsafe extern "C" fn DoTVShowWhatsNo1InHoennToday() {
    let mut show: *mut TVShow = null_mut();
    let mut state: u8 = 0;
    show = &raw mut (*gSaveBlock1Ptr).tvShows[gSpecialVar_0x8004];
    gSpecialVar_Result = FALSE as u16;
    state = sTVShowState;
    match state {
        0 => {
            TVShowConvertInternationalString(
                gStringVar1.as_mut_ptr(),
                (*show).numberOne.playerName.as_mut_ptr(),
                (*show).numberOne.language as i32,
            );
            match (*show).numberOne.actionIdx {
                0 => {
                    sTVShowState = 1;
                }
                1 => {
                    sTVShowState = 2;
                }
                2 => {
                    sTVShowState = 3;
                }
                3 => {
                    sTVShowState = 4;
                }
                4 => {
                    sTVShowState = 5;
                }
                5 => {
                    sTVShowState = 6;
                }
                6 => {
                    sTVShowState = 7;
                }
                _ => {}
            }
        }
        1 => {
            TVShowConvertInternationalString(
                gStringVar1.as_mut_ptr(),
                (*show).numberOne.playerName.as_mut_ptr(),
                (*show).numberOne.language as i32,
            );
            ConvertIntToDecimalString(1, (*show).numberOne.count as i32);
            sTVShowState = 8;
        }
        2 => {
            TVShowConvertInternationalString(
                gStringVar1.as_mut_ptr(),
                (*show).numberOne.playerName.as_mut_ptr(),
                (*show).numberOne.language as i32,
            );
            ConvertIntToDecimalString(1, (*show).numberOne.count as i32);
            sTVShowState = 8;
        }
        3 => {
            TVShowConvertInternationalString(
                gStringVar1.as_mut_ptr(),
                (*show).numberOne.playerName.as_mut_ptr(),
                (*show).numberOne.language as i32,
            );
            ConvertIntToDecimalString(1, (*show).numberOne.count as i32);
            sTVShowState = 8;
        }
        4 => {
            TVShowConvertInternationalString(
                gStringVar1.as_mut_ptr(),
                (*show).numberOne.playerName.as_mut_ptr(),
                (*show).numberOne.language as i32,
            );
            ConvertIntToDecimalString(1, (*show).numberOne.count as i32);
            sTVShowState = 8;
        }
        5 => {
            TVShowConvertInternationalString(
                gStringVar1.as_mut_ptr(),
                (*show).numberOne.playerName.as_mut_ptr(),
                (*show).numberOne.language as i32,
            );
            ConvertIntToDecimalString(1, (*show).numberOne.count as i32);
            sTVShowState = 8;
        }
        6 => {
            TVShowConvertInternationalString(
                gStringVar1.as_mut_ptr(),
                (*show).numberOne.playerName.as_mut_ptr(),
                (*show).numberOne.language as i32,
            );
            ConvertIntToDecimalString(1, (*show).numberOne.count as i32);
            sTVShowState = 8;
        }
        7 => {
            TVShowConvertInternationalString(
                gStringVar1.as_mut_ptr(),
                (*show).numberOne.playerName.as_mut_ptr(),
                (*show).numberOne.language as i32,
            );
            ConvertIntToDecimalString(1, (*show).numberOne.count as i32);
            sTVShowState = 8;
        }
        8 => {
            TVShowConvertInternationalString(
                gStringVar1.as_mut_ptr(),
                (*show).numberOne.playerName.as_mut_ptr(),
                (*show).numberOne.language as i32,
            );
            TVShowDone();
        }
        _ => {}
    }
    ShowFieldMessage(sTVWhatsNo1InHoennTodayTextGroup[state]);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SecretBaseSecrets_GetNumActionsTaken(show: *mut TVShow) -> u8 {
    let mut i: u8 = 0;
    let mut flagsSet: u8 = 0;
    i = 0;
    flagsSet = 0;
    while i < NUM_SECRET_BASE_FLAGS {
        if shr_u32((*show).secretBaseSecrets.flags, i as u32) & 1 != 0 {
            flagsSet += 1;
        }
        i += 1;
    }
    return flagsSet;
}
pub(crate) unsafe extern "C" fn SecretBaseSecrets_GetStateByFlagNumber(
    show: *mut TVShow,
    flagId: u8,
) -> u8 {
    let mut i: u8 = 0;
    let mut flagsSet: u8 = 0;
    i = 0;
    flagsSet = 0;
    while i < NUM_SECRET_BASE_FLAGS {
        if shr_u32((*show).secretBaseSecrets.flags, i as u32) & 1 != 0 {
            if flagsSet == flagId {
                return sTVSecretBaseSecretsActions[i];
            }
            flagsSet += 1;
        }
        i += 1;
    }
    return 0;
}
pub(crate) unsafe extern "C" fn DoTVShowSecretBaseSecrets() {
    let mut show: *mut TVShow = null_mut();
    let mut state: u8 = 0;
    let mut numActions: u8 = 0;
    let mut i: u16 = 0;
    show = &raw mut (*gSaveBlock1Ptr).tvShows[gSpecialVar_0x8004];
    gSpecialVar_Result = FALSE as u16;
    state = sTVShowState;
    match state {
        SBSECRETS_STATE_INTRO => {
            TVShowConvertInternationalString(
                gStringVar1.as_mut_ptr(),
                (*show).secretBaseSecrets.baseOwnersName.as_mut_ptr(),
                (*show).secretBaseSecrets.baseOwnersNameLanguage as i32,
            );
            TVShowConvertInternationalString(
                gStringVar2.as_mut_ptr(),
                (*show).secretBaseSecrets.playerName.as_mut_ptr(),
                (*show).secretBaseSecrets.language as i32,
            );
            numActions = SecretBaseSecrets_GetNumActionsTaken(show);
            if numActions == 0 {
                sTVShowState = SBSECRETS_STATE_NOTHING_USED1;
            } else {
                (*show).secretBaseSecrets.savedState = SBSECRETS_STATE_DO_NEXT1;
                sTVSecretBaseSecretsRandomValues[0] =
                    rem_i32(Random() as i32, numActions as i32) as u8;
                sTVShowState = SecretBaseSecrets_GetStateByFlagNumber(
                    show,
                    sTVSecretBaseSecretsRandomValues[0],
                );
            }
        }
        SBSECRETS_STATE_DO_NEXT1 => {
            TVShowConvertInternationalString(
                gStringVar2.as_mut_ptr(),
                (*show).secretBaseSecrets.playerName.as_mut_ptr(),
                (*show).secretBaseSecrets.language as i32,
            );
            numActions = SecretBaseSecrets_GetNumActionsTaken(show);
            match numActions {
                1 => {
                    sTVShowState = SBSECRETS_STATE_NOTHING_USED2;
                }
                2 => {
                    (*show).secretBaseSecrets.savedState = SBSECRETS_STATE_DO_NEXT2;
                    if sTVSecretBaseSecretsRandomValues[0] == 0 {
                        sTVShowState = SecretBaseSecrets_GetStateByFlagNumber(show, 1);
                    } else {
                        sTVShowState = SecretBaseSecrets_GetStateByFlagNumber(show, 0);
                    }
                }
                _ => {
                    i = 0;
                    while i < 0xFFFF {
                        sTVSecretBaseSecretsRandomValues[1] =
                            rem_i32(Random() as i32, numActions as i32) as u8;
                        if sTVSecretBaseSecretsRandomValues[1]
                            != sTVSecretBaseSecretsRandomValues[0]
                        {
                            break;
                        }
                        i += 1;
                    }
                    (*show).secretBaseSecrets.savedState = SBSECRETS_STATE_DO_NEXT2;
                    sTVShowState = SecretBaseSecrets_GetStateByFlagNumber(
                        show,
                        sTVSecretBaseSecretsRandomValues[1],
                    );
                }
            }
        }
        SBSECRETS_STATE_DO_NEXT2 => {
            TVShowConvertInternationalString(
                gStringVar2.as_mut_ptr(),
                (*show).secretBaseSecrets.playerName.as_mut_ptr(),
                (*show).secretBaseSecrets.language as i32,
            );
            numActions = SecretBaseSecrets_GetNumActionsTaken(show);
            if numActions == 2 {
                sTVShowState = SBSECRETS_STATE_NOTHING_USED2;
            } else {
                i = 0;
                while i < 0xFFFF {
                    sTVSecretBaseSecretsRandomValues[2] =
                        rem_i32(Random() as i32, numActions as i32) as u8;
                    if sTVSecretBaseSecretsRandomValues[2] != sTVSecretBaseSecretsRandomValues[0]
                        && sTVSecretBaseSecretsRandomValues[2]
                            != sTVSecretBaseSecretsRandomValues[1]
                    {
                        break;
                    }
                    i += 1;
                }
                (*show).secretBaseSecrets.savedState = SBSECRETS_STATE_TOOK_X_STEPS;
                sTVShowState = SecretBaseSecrets_GetStateByFlagNumber(
                    show,
                    sTVSecretBaseSecretsRandomValues[2],
                );
            }
        }
        SBSECRETS_STATE_TOOK_X_STEPS => {
            TVShowConvertInternationalString(
                gStringVar1.as_mut_ptr(),
                (*show).secretBaseSecrets.baseOwnersName.as_mut_ptr(),
                (*show).secretBaseSecrets.baseOwnersNameLanguage as i32,
            );
            TVShowConvertInternationalString(
                gStringVar2.as_mut_ptr(),
                (*show).secretBaseSecrets.playerName.as_mut_ptr(),
                (*show).secretBaseSecrets.language as i32,
            );
            ConvertIntToDecimalString(2, (*show).secretBaseSecrets.stepsInBase as i32);
            if (*show).secretBaseSecrets.stepsInBase <= 30 {
                sTVShowState = SBSECRETS_STATE_BASE_INTEREST_LOW;
            } else if (*show).secretBaseSecrets.stepsInBase <= 100 {
                sTVShowState = SBSECRETS_STATE_BASE_INTEREST_MED;
            } else {
                sTVShowState = SBSECRETS_STATE_BASE_INTEREST_HIGH;
            }
        }
        4..=6 => {
            TVShowConvertInternationalString(
                gStringVar1.as_mut_ptr(),
                (*show).secretBaseSecrets.baseOwnersName.as_mut_ptr(),
                (*show).secretBaseSecrets.baseOwnersNameLanguage as i32,
            );
            TVShowConvertInternationalString(
                gStringVar2.as_mut_ptr(),
                (*show).secretBaseSecrets.playerName.as_mut_ptr(),
                (*show).secretBaseSecrets.language as i32,
            );
            sTVShowState = SBSECRETS_STATE_OUTRO;
        }
        SBSECRETS_STATE_OUTRO => {
            TVShowConvertInternationalString(
                gStringVar1.as_mut_ptr(),
                (*show).secretBaseSecrets.baseOwnersName.as_mut_ptr(),
                (*show).secretBaseSecrets.baseOwnersNameLanguage as i32,
            );
            TVShowConvertInternationalString(
                gStringVar2.as_mut_ptr(),
                (*show).secretBaseSecrets.playerName.as_mut_ptr(),
                (*show).secretBaseSecrets.language as i32,
            );
            TVShowDone();
        }
        SBSECRETS_STATE_NOTHING_USED1 => {
            sTVShowState = SBSECRETS_STATE_TOOK_X_STEPS;
        }
        SBSECRETS_STATE_NOTHING_USED2 => {
            sTVShowState = SBSECRETS_STATE_TOOK_X_STEPS;
        }
        10..=18 => {
            sTVShowState = (*show).secretBaseSecrets.savedState;
        }
        SBSECRETS_STATE_USED_BAG => {
            StringCopy(
                gStringVar2.as_mut_ptr(),
                GetItemName((*show).secretBaseSecrets.item),
            );
            sTVShowState = (*show).secretBaseSecrets.savedState;
        }
        SBSECRETS_STATE_USED_CUSHION => {
            if (*show).common.trainerIdLo as i32 & 1 != 0 {
                sTVShowState = SBSECRETS_STATE_HUGGED_CUSHION;
            } else {
                sTVShowState = SBSECRETS_STATE_HIT_CUSHION;
            }
        }
        21..=43 => {
            sTVShowState = (*show).secretBaseSecrets.savedState;
        }
        _ => {}
    }
    ShowFieldMessage(sTVSecretBaseSecretsTextGroup[state]);
}
pub(crate) unsafe extern "C" fn DoTVShowSafariFanClub() {
    let mut show: *mut TVShow = null_mut();
    let mut state: u8 = 0;
    show = &raw mut (*gSaveBlock1Ptr).tvShows[gSpecialVar_0x8004];
    gSpecialVar_Result = FALSE as u16;
    state = sTVShowState;
    match state {
        0 => {
            if (*show).safariFanClub.monsCaught == 0 {
                sTVShowState = 6;
            } else if (*show).safariFanClub.monsCaught < 4 {
                sTVShowState = 5;
            } else {
                sTVShowState = 1;
            }
        }
        1 => {
            TVShowConvertInternationalString(
                gStringVar1.as_mut_ptr(),
                (*show).safariFanClub.playerName.as_mut_ptr(),
                (*show).safariFanClub.language as i32,
            );
            ConvertIntToDecimalString(1, (*show).safariFanClub.monsCaught as i32);
            if (*show).safariFanClub.pokeblocksUsed == 0 {
                sTVShowState = 3;
            } else {
                sTVShowState = 2;
            }
        }
        2 => {
            ConvertIntToDecimalString(1, (*show).safariFanClub.pokeblocksUsed as i32);
            sTVShowState = 4;
        }
        3 => {
            sTVShowState = 4;
        }
        4 => {
            TVShowConvertInternationalString(
                gStringVar1.as_mut_ptr(),
                (*show).safariFanClub.playerName.as_mut_ptr(),
                (*show).safariFanClub.language as i32,
            );
            sTVShowState = 10;
        }
        5 => {
            TVShowConvertInternationalString(
                gStringVar1.as_mut_ptr(),
                (*show).safariFanClub.playerName.as_mut_ptr(),
                (*show).safariFanClub.language as i32,
            );
            ConvertIntToDecimalString(1, (*show).safariFanClub.monsCaught as i32);
            if (*show).safariFanClub.pokeblocksUsed == 0 {
                sTVShowState = 8;
            } else {
                sTVShowState = 7;
            }
        }
        6 => {
            TVShowConvertInternationalString(
                gStringVar1.as_mut_ptr(),
                (*show).safariFanClub.playerName.as_mut_ptr(),
                (*show).safariFanClub.language as i32,
            );
            if (*show).safariFanClub.pokeblocksUsed == 0 {
                sTVShowState = 8;
            } else {
                sTVShowState = 7;
            }
        }
        7 => {
            ConvertIntToDecimalString(1, (*show).safariFanClub.pokeblocksUsed as i32);
            sTVShowState = 9;
        }
        8 => {
            sTVShowState = 9;
        }
        9 => {
            TVShowConvertInternationalString(
                gStringVar1.as_mut_ptr(),
                (*show).safariFanClub.playerName.as_mut_ptr(),
                (*show).safariFanClub.language as i32,
            );
            sTVShowState = 10;
        }
        10 => {
            TVShowDone();
        }
        _ => {}
    }
    ShowFieldMessage(sTVSafariFanClubTextGroup[state]);
}
pub(crate) unsafe extern "C" fn DoTVShowLilycoveContestLady() {
    let mut show: *mut TVShow = null_mut();
    let mut state: u8 = 0;
    show = &raw mut (*gSaveBlock1Ptr).tvShows[gSpecialVar_0x8004];
    gSpecialVar_Result = FALSE as u16;
    state = sTVShowState;
    'l1: {
        let sw1: u8 = state;
        let mut fall = false;
        if sw1 == CONTESTLADYLIVE_STATE_INTRO {
            fall = true;
            BufferContestName(
                gStringVar1.as_mut_ptr(),
                (*show).contestLady.contestCategory,
            );
            if (*show).contestLady.pokeblockState == CONTEST_LADY_GOOD {
                sTVShowState = CONTESTLADYLIVE_STATE_WON;
            } else if (*show).contestLady.pokeblockState == CONTEST_LADY_NORMAL {
                sTVShowState = CONTESTLADYLIVE_STATE_LOST;
            } else {
                sTVShowState = CONTESTLADYLIVE_STATE_LOST_BADLY;
            }
            break 'l1;
        }
        if sw1 == CONTESTLADYLIVE_STATE_WON || sw1 == CONTESTLADYLIVE_STATE_LOST {
            fall = true;
            TVShowConvertInternationalString(
                gStringVar3.as_mut_ptr(),
                (*show).contestLady.playerName.as_mut_ptr(),
                (*show).contestLady.language as i32,
            );
        }
        if fall || sw1 == CONTESTLADYLIVE_STATE_LOST_BADLY {
            fall = true;
            TVShowConvertInternationalString(
                gStringVar2.as_mut_ptr(),
                (*show).contestLady.nickname.as_mut_ptr(),
                (*show).contestLady.pokemonNameLanguage as i32,
            );
            TVShowDone();
            break 'l1;
        }
    }
    ShowFieldMessage(sTVLilycoveContestLadyTextGroup[state]);
}
pub(crate) unsafe extern "C" fn TVShowDone() {
    gSpecialVar_Result = TRUE as u16;
    sTVShowState = 0;
    (*gSaveBlock1Ptr).tvShows[gSpecialVar_0x8004].common.active = FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ResetTVShowState() {
    sTVShowState = 0;
}
