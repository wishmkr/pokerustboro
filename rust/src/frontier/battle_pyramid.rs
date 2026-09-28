//! Translated from `src/battle_pyramid.c` by tools/rustport/c2rs.py.
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
// Data tables (translate with cdata.py): sLevel50WildMons_Round1 sLevel50WildMons_Round2 sLevel50WildMons_Round3 sLevel50WildMons_Round4 sLevel50WildMons_Round5 sLevel50WildMons_Round6 sLevel50WildMons_Round7 sLevel50WildMons_Round8 sLevel50WildMons_Round9 sLevel50WildMons_Round10 sLevel50WildMons_Round11 sLevel50WildMons_Round12 sLevel50WildMons_Round13 sLevel50WildMons_Round14 sLevel50WildMons_Round15 sLevel50WildMons_Round16 sLevel50WildMons_Round17 sLevel50WildMons_Round18 sLevel50WildMons_Round19 sLevel50WildMons_Round20 sLevel50WildMonPointers sOpenLevelWildMons_Round1 sOpenLevelWildMons_Round2 sOpenLevelWildMons_Round3 sOpenLevelWildMons_Round4 sOpenLevelWildMons_Round5 sOpenLevelWildMons_Round6 sOpenLevelWildMons_Round7 sOpenLevelWildMons_Round8 sOpenLevelWildMons_Round9 sOpenLevelWildMons_Round10 sOpenLevelWildMons_Round11 sOpenLevelWildMons_Round12 sOpenLevelWildMons_Round13 sOpenLevelWildMons_Round14 sOpenLevelWildMons_Round15 sOpenLevelWildMons_Round16 sOpenLevelWildMons_Round17 sOpenLevelWildMons_Round18 sOpenLevelWildMons_Round19 sOpenLevelWildMons_Round20 sOpenLevelWildMonPointers sPyramidFloorTemplates sPyramidFloorTemplateOptions sFloorTemplateOffsets sPickupItemsLvl50 sPickupItemsLvlOpen sPickupItemSlots sPickupItemOffsets sTrainerClassEncounterMusic sTrainerTextGroups sExitDirectionHintTexts1 sRemainingItemsHintTexts1 sRemainingTrainersHintTexts1 sExitDirectionHintTexts2 sRemainingItemsHintTexts2 sRemainingTrainersHintTexts2 sExitDirectionHintTexts3 sRemainingItemsHintTexts3 sRemainingTrainersHintTexts3 sExitDirectionHintTexts4 sRemainingItemsHintTexts4 sRemainingTrainersHintTexts4 sExitDirectionHintTexts5 sRemainingItemsHintTexts5 sRemainingTrainersHintTexts5 sExitDirectionHintTexts6 sRemainingItemsHintTexts6 sRemainingTrainersHintTexts6 sPostBattleHintTexts1 sPostBattleHintTexts2 sPostBattleHintTexts3 sPostBattleHintTexts4 sPostBattleHintTexts5 sPostBattleHintTexts6 sPostBattleTexts sHintTextTypes sBattlePyramidFunctions sShortStreakRewardItems sLongStreakRewardItems sBorderedSquareIds sPickupPercentages

/// `struct PyramidWildMon`
#[repr(C, align(4))]
#[derive(Clone, Copy)]
pub struct PyramidWildMon {
    pub species: u16,
    pub lvl: u8,
    pub abilityNum: u8,
    pub moves: CArray<u16, 4>,
}

unsafe impl Sync for PyramidWildMon {}

/// `struct PyramidFloorTemplate`
#[repr(C, align(4))]
#[derive(Clone, Copy)]
pub struct PyramidFloorTemplate {
    pub numItems: u8,
    pub numTrainers: u8,
    pub itemPositions: u8,
    pub trainerPositions: u8,
    pub runMultiplier: u8,
    pub layoutOffsets: CArray<u8, 8>,
}

unsafe impl Sync for PyramidFloorTemplate {}

/// `struct PyramidTrainerEncounterMusic`
#[repr(C, align(4))]
#[derive(Clone, Copy)]
pub struct PyramidTrainerEncounterMusic {
    pub trainerClass: u8,
    pub trainerEncounterMusic: u8,
}

unsafe impl Sync for PyramidTrainerEncounterMusic {}

#[cfg(target_arch = "arm")]
const _: () = {
    #[allow(unused_imports)]
    use core::mem::{offset_of, size_of};
    assert!(size_of::<PyramidWildMon>() == 12);
    assert!(offset_of!(PyramidWildMon, species) == 0);
    assert!(offset_of!(PyramidWildMon, lvl) == 2);
    assert!(offset_of!(PyramidWildMon, abilityNum) == 3);
    assert!(offset_of!(PyramidWildMon, moves) == 4);
    assert!(size_of::<PyramidFloorTemplate>() == 16);
    assert!(offset_of!(PyramidFloorTemplate, numItems) == 0);
    assert!(offset_of!(PyramidFloorTemplate, numTrainers) == 1);
    assert!(offset_of!(PyramidFloorTemplate, itemPositions) == 2);
    assert!(offset_of!(PyramidFloorTemplate, trainerPositions) == 3);
    assert!(offset_of!(PyramidFloorTemplate, runMultiplier) == 4);
    assert!(offset_of!(PyramidFloorTemplate, layoutOffsets) == 5);
    assert!(size_of::<PyramidTrainerEncounterMusic>() == 4);
    assert!(offset_of!(PyramidTrainerEncounterMusic, trainerClass) == 0);
    assert!(offset_of!(PyramidTrainerEncounterMusic, trainerEncounterMusic) == 1);
};

const NUM_LAYOUT_OFFSETS: i32 = 8;

static sBattlePyramidFunctions: Table<CArray<Option<unsafe extern "C" fn()>, 18>> =
    Table((&raw const crate::data::battle_pyramid::sBattlePyramidFunctions).cast());
static sBorderedSquareIds: Table<CArray<CArray<u8, 4>, 16>> =
    Table((&raw const crate::data::battle_pyramid::sBorderedSquareIds).cast());
static sFloorTemplateOffsets: Table<CArray<u8, 7>> =
    Table((&raw const crate::data::battle_pyramid::sFloorTemplateOffsets).cast());
static sHintTextTypes: Table<CArray<u8, 8>> =
    Table((&raw const crate::data::battle_pyramid::sHintTextTypes).cast());
static sLevel50WildMonPointers: Table<CArray<*mut PyramidWildMon, 20>> =
    Table((&raw const crate::data::battle_pyramid::sLevel50WildMonPointers).cast());
static sLongStreakRewardItems: Table<CArray<u16, 9>> =
    Table((&raw const crate::data::battle_pyramid::sLongStreakRewardItems).cast());
static sOpenLevelWildMonPointers: Table<CArray<*mut PyramidWildMon, 20>> =
    Table((&raw const crate::data::battle_pyramid::sOpenLevelWildMonPointers).cast());
static sPickupItemOffsets: Table<CArray<u8, 7>> =
    Table((&raw const crate::data::battle_pyramid::sPickupItemOffsets).cast());
static sPickupItemSlots: Table<CArray<CArray<u8, 2>, 63>> =
    Table((&raw const crate::data::battle_pyramid::sPickupItemSlots).cast());
static sPickupItemsLvl50: Table<CArray<CArray<u16, 10>, 20>> =
    Table((&raw const crate::data::battle_pyramid::sPickupItemsLvl50).cast());
static sPickupItemsLvlOpen: Table<CArray<CArray<u16, 10>, 20>> =
    Table((&raw const crate::data::battle_pyramid::sPickupItemsLvlOpen).cast());
static sPickupPercentages: Table<CArray<u8, 10>> =
    Table((&raw const crate::data::battle_pyramid::sPickupPercentages).cast());
static sPostBattleTexts: Table<CArray<*mut *mut *mut u8, 6>> =
    Table((&raw const crate::data::battle_pyramid::sPostBattleTexts).cast());
static sPyramidFloorTemplateOptions: Table<CArray<CArray<u8, 2>, 34>> =
    Table((&raw const crate::data::battle_pyramid::sPyramidFloorTemplateOptions).cast());
static sPyramidFloorTemplates: Table<CArray<PyramidFloorTemplate, 16>> =
    Table((&raw const crate::data::battle_pyramid::sPyramidFloorTemplates).cast());
static sShortStreakRewardItems: Table<CArray<u16, 6>> =
    Table((&raw const crate::data::battle_pyramid::sShortStreakRewardItems).cast());
static sTrainerClassEncounterMusic: Table<CArray<PyramidTrainerEncounterMusic, 54>> =
    Table((&raw const crate::data::battle_pyramid::sTrainerClassEncounterMusic).cast());
static sTrainerTextGroups: Table<CArray<CArray<u8, 2>, 50>> =
    Table((&raw const crate::data::battle_pyramid::sTrainerTextGroups).cast());

unsafe extern "C" {
    static BattlePyramid_FindItemBall: CArray<u8, 0>;
    static BattlePyramid_Retire: CArray<u8, 0>;
    static BattlePyramid_TrainerBattle: CArray<u8, 0>;
    static mut gBackupMapLayout: BackupMapLayout;
    static gBattleFrontierTrainers: CArray<BattleFrontierTrainer, 0>;
    static mut gBattleOutcome: u8;
    static gBattlePyramidFloor_Pal: CArray<CArray<u16, 16>, 0>;
    static mut gBattleTypeFlags: u32;
    static gBitTable: CArray<u32, 0>;
    static mut gEnemyParty: CArray<Pokemon, 6>;
    static gExperienceTables: CArray<CArray<u32, 101>, 0>;
    static gFacilityClassToTrainerClass: CArray<u8, 0>;
    static mut gFacilityTrainers: *mut BattleFrontierTrainer;
    static mut gMapHeader: MapHeader;
    static gMapLayouts: CArray<*mut MapLayout, 0>;
    static mut gObjectEvents: CArray<ObjectEvent, 16>;
    static mut gPaletteFade: PaletteFadeControl;
    static mut gPlayerParty: CArray<Pokemon, 6>;
    static mut gPlttBufferUnfaded: CArray<u16, 512>;
    static mut gSaveBlock1Ptr: *mut SaveBlock1;
    static mut gSaveBlock2Ptr: *mut SaveBlock2;
    static mut gSelectedObjectEvent: u8;
    static mut gSelectedOrderFromParty: CArray<u8, 4>;
    static mut gSpecialVar_0x8000: u16;
    static mut gSpecialVar_0x8001: u16;
    static mut gSpecialVar_0x8004: u16;
    static mut gSpecialVar_0x8005: u16;
    static mut gSpecialVar_0x8006: u16;
    static mut gSpecialVar_0x8007: u16;
    static mut gSpecialVar_LastTalked: u16;
    static mut gSpecialVar_Result: u16;
    static gSpeciesInfo: CArray<SpeciesInfo, 0>;
    static mut gStringVar1: CArray<u8, 256>;
    static mut gTrainerBattleOpponent_A: u16;
    static mut gTrainerBattleOpponent_B: u16;
    fn AddBagItem(a0: u16, a1: u16) -> u8;
    fn AddPyramidBagItem(a0: u16, a1: u16) -> u8;
    fn AllocZeroed(a0: u32) -> *mut c_void;
    fn CalculateMonStats(a0: *mut Pokemon);
    fn CopyItemName(a0: u16, a1: *mut u8);
    fn CpuSet(a0: *mut c_void, a1: *mut c_void, a2: u32);
    fn CreateTask(a0: Option<unsafe extern "C" fn(u8)>, a1: u8) -> u8;
    fn DestroyTask(a0: u8);
    fn DoSoftReset();
    fn Free(a0: *mut c_void);
    fn FrontierSpeechToString(a0: *mut u16);
    fn GetBattleFacilityTrainerGfxId(a0: u16) -> u8;
    fn GetChosenApproachingTrainerObjectEventId(a0: u8) -> u8;
    fn GetMonData3(a0: *mut Pokemon, a1: i32, a2: *mut u8) -> u32;
    fn GetRandomScaledFrontierTrainerId(a0: u8, a1: u8) -> u16;
    fn GetSpeciesName(a0: *mut u8, a1: u16);
    fn InitBattlePyramidBagCursorPosition();
    fn LoadPlayerParty();
    fn Overworld_GetMapHeaderByGroupAndId(a0: u16, a1: u16) -> *mut MapHeader;
    fn PlaySE(a0: u16);
    fn Random() -> u16;
    fn Random2() -> u16;
    fn RunOnLoadMapScript();
    fn SaveMapView();
    fn ScriptContext_SetupScript(a0: *mut u8);
    fn SeedRng2(a0: u16);
    fn SetFacilityPtrsGetLevel() -> u8;
    fn SetMonData(a0: *mut Pokemon, a1: i32, a2: *mut c_void);
    fn SetMonMoveSlot(a0: *mut Pokemon, a1: u16, a2: u8);
    fn ShowBattlePyramidStartMenu();
    fn ShowFieldMessage(a0: *mut u8) -> u8;
    fn TrySavingData(a0: u8) -> u8;
    fn VarSet(a0: u16, a1: u16) -> u8;
    fn WriteBattlePyramidViewScanlineEffectBuffer();
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn CallBattlePyramidFunction() {
    sBattlePyramidFunctions[gSpecialVar_0x8004].unwrap_unchecked()();
}
pub(crate) unsafe extern "C" fn InitPyramidChallenge() {
    let mut isCurrent: u32 = 0;
    let mut lvlMode: u32 = (*gSaveBlock2Ptr).frontier.lvlMode() as u32;
    (*gSaveBlock2Ptr).frontier.challengeStatus = 0;
    (*gSaveBlock2Ptr).frontier.curChallengeBattleNum = 0;
    (*gSaveBlock2Ptr).frontier.set_challengePaused(FALSE);
    if lvlMode != FRONTIER_LVL_50 as u32 {
        isCurrent = (*gSaveBlock2Ptr).frontier.winStreakActiveFlags & STREAK_PYRAMID_OPEN;
    } else {
        isCurrent = (*gSaveBlock2Ptr).frontier.winStreakActiveFlags & STREAK_PYRAMID_50;
    }
    if isCurrent == 0 {
        (*gSaveBlock2Ptr).frontier.pyramidWinStreaks[lvlMode] = 0;
        InitPyramidBagItems(lvlMode as u8);
    }
    InitBattlePyramidBagCursorPosition();
    gTrainerBattleOpponent_A = 0;
    gBattleOutcome = 0;
}
pub(crate) unsafe extern "C" fn GetBattlePyramidData() {
    let mut lvlMode: u32 = (*gSaveBlock2Ptr).frontier.lvlMode() as u32;
    match gSpecialVar_0x8005 {
        PYRAMID_DATA_PRIZE => {
            gSpecialVar_Result = (*gSaveBlock2Ptr).frontier.pyramidPrize;
        }
        PYRAMID_DATA_WIN_STREAK => {
            gSpecialVar_Result = (*gSaveBlock2Ptr).frontier.pyramidWinStreaks[lvlMode];
        }
        PYRAMID_DATA_WIN_STREAK_ACTIVE => {
            if lvlMode != FRONTIER_LVL_50 as u32 {
                gSpecialVar_Result = (*gSaveBlock2Ptr).frontier.winStreakActiveFlags as u16
                    & STREAK_PYRAMID_OPEN as u16;
            } else {
                gSpecialVar_Result = (*gSaveBlock2Ptr).frontier.winStreakActiveFlags as u16
                    & STREAK_PYRAMID_50 as u16;
            }
        }
        PYRAMID_DATA_WIN_STREAK_50 => {
            gSpecialVar_Result = (*gSaveBlock2Ptr).frontier.pyramidWinStreaks[0];
        }
        PYRAMID_DATA_WIN_STREAK_OPEN => {
            gSpecialVar_Result = (*gSaveBlock2Ptr).frontier.pyramidWinStreaks[1];
        }
        PYRAMID_DATA_WIN_STREAK_ACTIVE_50 => {
            gSpecialVar_Result =
                (*gSaveBlock2Ptr).frontier.winStreakActiveFlags as u16 & STREAK_PYRAMID_50 as u16;
        }
        PYRAMID_DATA_WIN_STREAK_ACTIVE_OPEN => {
            gSpecialVar_Result =
                (*gSaveBlock2Ptr).frontier.winStreakActiveFlags as u16 & STREAK_PYRAMID_OPEN as u16;
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn SetBattlePyramidData() {
    let mut lvlMode: u32 = (*gSaveBlock2Ptr).frontier.lvlMode() as u32;
    match gSpecialVar_0x8005 {
        PYRAMID_DATA_PRIZE => {
            (*gSaveBlock2Ptr).frontier.pyramidPrize = gSpecialVar_0x8006;
        }
        PYRAMID_DATA_WIN_STREAK => {
            (*gSaveBlock2Ptr).frontier.pyramidWinStreaks[lvlMode] = gSpecialVar_0x8006;
        }
        PYRAMID_DATA_WIN_STREAK_ACTIVE => {
            if lvlMode != FRONTIER_LVL_50 as u32 {
                if gSpecialVar_0x8006 != 0 {
                    (*gSaveBlock2Ptr).frontier.winStreakActiveFlags |= STREAK_PYRAMID_OPEN;
                } else {
                    (*gSaveBlock2Ptr).frontier.winStreakActiveFlags &= 0xffffdfff;
                }
            } else {
                if gSpecialVar_0x8006 != 0 {
                    (*gSaveBlock2Ptr).frontier.winStreakActiveFlags |= STREAK_PYRAMID_50;
                } else {
                    (*gSaveBlock2Ptr).frontier.winStreakActiveFlags &= 0xffffefff;
                }
            }
        }
        PYRAMID_DATA_TRAINER_FLAGS => {
            (*gSaveBlock2Ptr).frontier.pyramidTrainerFlags = gSpecialVar_0x8006 as u8;
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn SavePyramidChallenge() {
    (*gSaveBlock2Ptr).frontier.challengeStatus = gSpecialVar_0x8005 as u8;
    VarSet(VAR_TEMP_CHALLENGE_STATUS, 0);
    (*gSaveBlock2Ptr).frontier.set_challengePaused(TRUE);
    SaveMapView();
    TrySavingData(SAVE_LINK);
}
pub(crate) unsafe extern "C" fn SetBattlePyramidPrize() {
    if (*gSaveBlock2Ptr).frontier.pyramidWinStreaks[(*gSaveBlock2Ptr).frontier.lvlMode()] > 41 {
        (*gSaveBlock2Ptr).frontier.pyramidPrize = sLongStreakRewardItems[Random() % 9];
    } else {
        (*gSaveBlock2Ptr).frontier.pyramidPrize = sShortStreakRewardItems[Random() % 6];
    }
}
pub(crate) unsafe extern "C" fn GiveBattlePyramidPrize() {
    if AddBagItem((*gSaveBlock2Ptr).frontier.pyramidPrize, 1) == 1 {
        CopyItemName(
            (*gSaveBlock2Ptr).frontier.pyramidPrize,
            gStringVar1.as_mut_ptr(),
        );
        (*gSaveBlock2Ptr).frontier.pyramidPrize = 0;
        gSpecialVar_Result = TRUE as u16;
    } else {
        gSpecialVar_Result = FALSE as u16;
    }
}
pub(crate) unsafe extern "C" fn SeedPyramidFloor() {
    let mut i: i32 = 0;
    i = 0;
    while i < 4 {
        (*gSaveBlock2Ptr).frontier.pyramidRandoms[i] = Random();
        i += 1;
    }
    (*gSaveBlock2Ptr).frontier.pyramidTrainerFlags = 0;
}
pub(crate) unsafe extern "C" fn SetPickupItem() {
    let mut i: i32 = 0;
    let mut itemIndex: i32 = 0;
    let mut rand: i32 = 0;
    let mut id: u8 = 0;
    let mut lvlMode: u32 = (*gSaveBlock2Ptr).frontier.lvlMode() as u32;
    let mut floor: u32 = (*gSaveBlock2Ptr).frontier.curChallengeBattleNum as u32;
    let mut round: u32 =
        ((*gSaveBlock2Ptr).frontier.pyramidWinStreaks[lvlMode] as i32 / 7 % 20) as u32;
    if round >= TOTAL_PYRAMID_ROUNDS as u32 {
        round = 19;
    }
    id = GetPyramidFloorTemplateId();
    itemIndex = gSpecialVar_LastTalked as i32 - sPyramidFloorTemplates[id].numTrainers as i32 - 1;
    rand = (*gSaveBlock2Ptr).frontier.pyramidRandoms[itemIndex / 2] as i32;
    SeedRng2(rand as u16);
    i = 0;
    while i < itemIndex + 1 {
        rand = Random2() as i32 % 100;
        i += 1;
    }
    i = sPickupItemOffsets[floor] as i32;
    while i < 63 {
        if rand < sPickupItemSlots[i][0] as i32 {
            break;
        }
        i += 1;
    }
    if lvlMode != FRONTIER_LVL_50 as u32 {
        gSpecialVar_0x8000 = sPickupItemsLvlOpen[round][sPickupItemSlots[i][1]];
    } else {
        gSpecialVar_0x8000 = sPickupItemsLvl50[round][sPickupItemSlots[i][1]];
    }
    gSpecialVar_0x8001 = 1;
}
pub(crate) unsafe extern "C" fn HidePyramidItem() {
    let mut events: *mut ObjectEventTemplate = (*gSaveBlock1Ptr).objectEventTemplates.as_mut_ptr();
    let mut i: i32 = 0;
    loop {
        if (*events.at(i)).localId as u16 == gSpecialVar_LastTalked {
            (*events.at(i)).x = 32767;
            (*events.at(i)).y = 32767;
            break;
        }
        i += 1;
        if (*events.at(i)).localId == LOCALID_NONE {
            break;
        }
    }
}
pub(crate) unsafe extern "C" fn SetPyramidFacilityTrainers() {
    gFacilityTrainers = gBattleFrontierTrainers.as_ptr().cast_mut();
}
pub(crate) unsafe extern "C" fn ShowPostBattleHintText() {
    let mut i: i32 = 0;
    let mut hintType: i32 = 0;
    let mut id: u8 = 0;
    let mut textGroup: i32 = 0;
    let mut textIndex: i32 = 0;
    let mut events: *mut ObjectEventTemplate = (*gSaveBlock1Ptr).objectEventTemplates.as_mut_ptr();
    let mut trainerId: u16 = LocalIdToPyramidTrainerId(gObjectEvents[gSelectedObjectEvent].localId);
    i = 0;
    while i < 50 {
        if sTrainerTextGroups[i][0] == (*gFacilityTrainers.at(trainerId)).facilityClass {
            textGroup = sTrainerTextGroups[i][1] as i32;
            break;
        }
        i += 1;
    }
    hintType = sHintTextTypes[gObjectEvents[gSelectedObjectEvent].localId as i32 - 1] as i32;
    i = 0;
    while i == 0 {
        match hintType {
            0 => {
                textIndex =
                    GetPostBattleDirectionHintTextIndex(&raw mut hintType, 8, HINT_EXIT_DIRECTION)
                        as i32;
                i = 1;
            }
            1 => {
                i = 0;
                while i < GetNumBattlePyramidObjectEvents() as i32 {
                    if (*events.at(i)).graphicsId == OBJ_EVENT_GFX_ITEM_BALL
                        && (*events.at(i)).x != 32767
                        && (*events.at(i)).y != 32767
                    {
                        textIndex += 1;
                    }
                    i += 1;
                }
                i = 1;
            }
            2 => {
                id = GetPyramidFloorTemplateId();
                textIndex = sPyramidFloorTemplates[id].numTrainers as i32;
                i = 0;
                while i < MAX_PYRAMID_TRAINERS {
                    if gBitTable[i] & (*gSaveBlock2Ptr).frontier.pyramidTrainerFlags as u32 != 0 {
                        textIndex -= 1;
                    }
                    i += 1;
                }
                i = 1;
            }
            HINT_EXIT_SHORT_REMAINING_TRAINERS => {
                GetPostBattleDirectionHintTextIndex(&raw mut hintType, 8, HINT_REMAINING_TRAINERS);
            }
            HINT_EXIT_SHORT_REMAINING_ITEMS => {
                GetPostBattleDirectionHintTextIndex(&raw mut hintType, 8, HINT_REMAINING_ITEMS);
            }
            HINT_EXIT_MEDIUM_REMAINING_TRAINERS => {
                GetPostBattleDirectionHintTextIndex(&raw mut hintType, 16, HINT_REMAINING_TRAINERS);
            }
            HINT_EXIT_MEDIUM_REMAINING_ITEMS => {
                GetPostBattleDirectionHintTextIndex(&raw mut hintType, 16, HINT_REMAINING_ITEMS);
            }
            HINT_EXIT_FAR_REMAINING_TRAINERS => {
                GetPostBattleDirectionHintTextIndex(&raw mut hintType, 24, HINT_REMAINING_TRAINERS);
            }
            HINT_EXIT_FAR_REMAINING_ITEMS => {
                GetPostBattleDirectionHintTextIndex(&raw mut hintType, 24, HINT_REMAINING_ITEMS);
            }
            _ => {}
        }
    }
    ShowFieldMessage(*(*sPostBattleTexts[textGroup].at(hintType)).at(textIndex));
}
pub(crate) unsafe extern "C" fn UpdatePyramidWinStreak() {
    let mut lvlMode: u32 = (*gSaveBlock2Ptr).frontier.lvlMode() as u32;
    if (*gSaveBlock2Ptr).frontier.pyramidWinStreaks[lvlMode] < 999 {
        (*gSaveBlock2Ptr).frontier.pyramidWinStreaks[lvlMode] += 1;
    }
    if (*gSaveBlock2Ptr).frontier.pyramidWinStreaks[lvlMode]
        > (*gSaveBlock2Ptr).frontier.pyramidRecordStreaks[lvlMode]
    {
        (*gSaveBlock2Ptr).frontier.pyramidRecordStreaks[lvlMode] =
            (*gSaveBlock2Ptr).frontier.pyramidWinStreaks[lvlMode];
    }
}
pub(crate) unsafe extern "C" fn GetCurrentBattlePyramidLocation() {
    gSpecialVar_Result = CurrentBattlePyramidLocation() as u16;
}
pub(crate) unsafe extern "C" fn UpdatePyramidLightRadius() {
    match gSpecialVar_0x8006 {
        PYRAMID_LIGHT_SET_RADIUS => {
            (*gSaveBlock2Ptr).frontier.pyramidLightRadius = gSpecialVar_0x8005 as u8;
        }
        PYRAMID_LIGHT_INCR_RADIUS => match gSpecialVar_Result {
            0 => {
                if gPaletteFade.active() == 0 {
                    if (*gSaveBlock2Ptr).frontier.pyramidLightRadius >= 120 {
                        (*gSaveBlock2Ptr).frontier.pyramidLightRadius = 120;
                    } else {
                        PlaySE(gSpecialVar_0x8007);
                    }
                    gSpecialVar_Result += 1;
                }
            }
            1 => {
                if gSpecialVar_0x8005 != 0 {
                    gSpecialVar_0x8005 -= 1;
                    (*gSaveBlock2Ptr).frontier.pyramidLightRadius += 1;
                    if (*gSaveBlock2Ptr).frontier.pyramidLightRadius > 120 {
                        (*gSaveBlock2Ptr).frontier.pyramidLightRadius = 120;
                        gSpecialVar_Result += 1;
                    }
                    WriteBattlePyramidViewScanlineEffectBuffer();
                } else {
                    gSpecialVar_Result = 2;
                }
            }
            _ => {}
        },
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn ClearPyramidPartyHeldItems() {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut item: u16 = 0;
    i = 0;
    while i < PARTY_SIZE {
        j = 0;
        while j
            < (if 3 >= (if 4 >= 2 { 4 } else { 2 }) {
                3
            } else {
                if 4 >= 2 { 4 } else { 2 }
            })
        {
            if (*gSaveBlock2Ptr).frontier.selectedPartyMons[j] != 0
                && (*gSaveBlock2Ptr).frontier.selectedPartyMons[j] as i32 - 1 == i
            {
                SetMonData(
                    &raw mut gPlayerParty[i],
                    MON_DATA_HELD_ITEM,
                    &raw mut item as *mut c_void,
                );
            }
            j += 1;
        }
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn SetPyramidFloorPalette() {
    CreateTask(Some(Task_SetPyramidFloorPalette), 0);
}
pub(crate) unsafe extern "C" fn Task_SetPyramidFloorPalette(taskId: u8) {
    if gPaletteFade.active() != 0 {
        CpuSet(
            gBattlePyramidFloor_Pal[(*gSaveBlock2Ptr).frontier.curChallengeBattleNum]
                .as_ptr()
                .cast_mut() as *mut c_void,
            &raw mut gPlttBufferUnfaded[96] as *mut c_void,
            16,
        );
        DestroyTask(taskId);
    }
}
pub(crate) unsafe extern "C" fn BattlePyramidStartMenu() {
    ShowBattlePyramidStartMenu();
}
pub(crate) unsafe extern "C" fn RestorePyramidPlayerParty() {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut l: i32 = 0;
    i = 0;
    while i < FRONTIER_PARTY_SIZE {
        let mut partyIndex: i32 = (*gSaveBlock2Ptr).frontier.selectedPartyMons[i] as i32 - 1;
        j = 0;
        while j < FRONTIER_PARTY_SIZE {
            if GetMonData3(
                &raw mut (*gSaveBlock1Ptr).playerParty[partyIndex],
                MON_DATA_SPECIES,
                null_mut(),
            ) == GetMonData3(&raw mut gPlayerParty[j], MON_DATA_SPECIES, null_mut())
            {
                k = 0;
                while k < MAX_MON_MOVES {
                    l = 0;
                    while l < MAX_MON_MOVES {
                        if GetMonData3(
                            &raw mut (*gSaveBlock1Ptr).playerParty[partyIndex],
                            MON_DATA_MOVE1 + l,
                            null_mut(),
                        ) == GetMonData3(
                            &raw mut gPlayerParty[j],
                            MON_DATA_MOVE1 + k,
                            null_mut(),
                        ) {
                            break;
                        }
                        l += 1;
                    }
                    if l == MAX_MON_MOVES {
                        SetMonMoveSlot(&raw mut gPlayerParty[j], MOVE_SKETCH, k as u8);
                    }
                    k += 1;
                }
                (*gSaveBlock1Ptr).playerParty[partyIndex] = gPlayerParty[j];
                gSelectedOrderFromParty[j] = partyIndex as u8 + 1;
                break;
            }
            j += 1;
        }
        i += 1;
    }
    i = 0;
    while i < FRONTIER_PARTY_SIZE {
        (*gSaveBlock2Ptr).frontier.selectedPartyMons[i] = gSelectedOrderFromParty[i] as u16;
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn GetPostBattleDirectionHintTextIndex(
    hintType: *mut i32,
    minDistanceForExitHint: u8,
    defaultHintType: u8,
) -> u8 {
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut textIndex: u8 = 0;
    let mut map: *mut u16 = gBackupMapLayout.map;
    map = map.at(gBackupMapLayout.width * MAP_OFFSET + MAP_OFFSET);
    y = 0;
    while y < 32 {
        x = 0;
        while x < 32 {
            if *map.at(x) as i32 & MAPGRID_METATILE_ID_MASK == METATILE_BattlePyramid_Exit {
                x -= gObjectEvents[gSelectedObjectEvent].initialCoords.x as i32 - MAP_OFFSET;
                y -= gObjectEvents[gSelectedObjectEvent].initialCoords.y as i32 - MAP_OFFSET;
                if x >= minDistanceForExitHint as i32
                    || x <= -(minDistanceForExitHint as i32)
                    || y >= minDistanceForExitHint as i32
                    || y <= -(minDistanceForExitHint as i32)
                    || defaultHintType == HINT_EXIT_DIRECTION
                {
                    if x > 0 && y > 0 {
                        if x >= y {
                            textIndex = 2;
                        } else {
                            textIndex = 3;
                        }
                    } else if x < 0 && y < 0 {
                        if x > y {
                            textIndex = 0;
                        } else {
                            textIndex = 1;
                        }
                    } else if x == 0 {
                        if y > 0 {
                            textIndex = 3;
                        } else {
                            textIndex = 0;
                        }
                    } else if y == 0 {
                        if x > 0 {
                            textIndex = 2;
                        } else {
                            textIndex = 1;
                        }
                    } else if x < 0 {
                        if x + y > 0 {
                            textIndex = 3;
                        } else {
                            textIndex = 1;
                        }
                    } else {
                        if x + y >= 0 {
                            textIndex = 2;
                        } else {
                            textIndex = 0;
                        }
                    }
                    *hintType = HINT_EXIT_DIRECTION as i32;
                } else {
                    *hintType = defaultHintType as i32;
                }
                return textIndex;
            }
            x += 1;
        }
        map = map.at(47);
        y += 1;
    }
    return textIndex;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn LocalIdToPyramidTrainerId(localId: u8) -> u16 {
    return (*gSaveBlock2Ptr).frontier.trainerIds[localId as i32 - 1];
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetBattlePyramidTrainerFlag(eventId: u8) -> u8 {
    return (*gSaveBlock2Ptr).frontier.pyramidTrainerFlags
        & gBitTable[gObjectEvents[eventId].localId as i32 - 1] as u8;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MarkApproachingPyramidTrainersAsBattled() {
    MarkPyramidTrainerAsBattled(gTrainerBattleOpponent_A);
    if gBattleTypeFlags & BATTLE_TYPE_TWO_OPPONENTS != 0 {
        gSelectedObjectEvent = GetChosenApproachingTrainerObjectEventId(1);
        MarkPyramidTrainerAsBattled(gTrainerBattleOpponent_B);
    }
}
pub(crate) unsafe extern "C" fn MarkPyramidTrainerAsBattled(trainerId: u16) {
    let mut i: i32 = 0;
    i = 0;
    while i < MAX_PYRAMID_TRAINERS {
        if (*gSaveBlock2Ptr).frontier.trainerIds[i] == trainerId {
            (*gSaveBlock2Ptr).frontier.pyramidTrainerFlags |= gBitTable[i] as u8;
        }
        i += 1;
    }
    gObjectEvents[gSelectedObjectEvent].movementType = MOVEMENT_TYPE_WANDER_AROUND;
    (*gSaveBlock1Ptr).objectEventTemplates[gSpecialVar_LastTalked as i32 - 1].movementType =
        MOVEMENT_TYPE_WANDER_AROUND;
    gObjectEvents[gSelectedObjectEvent].initialCoords.x =
        gObjectEvents[gSelectedObjectEvent].currentCoords.x;
    gObjectEvents[gSelectedObjectEvent].initialCoords.y =
        gObjectEvents[gSelectedObjectEvent].currentCoords.y;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GenerateBattlePyramidWildMon() {
    let mut name: CArray<u8, 11> = zeroed();
    let mut i: i32 = 0;
    let mut wildMons: *mut PyramidWildMon = null_mut();
    let mut id: u32 = 0;
    let mut lvl: u32 = (*gSaveBlock2Ptr).frontier.lvlMode() as u32;
    let mut round: u16 = ((*gSaveBlock2Ptr).frontier.pyramidWinStreaks[lvl] as i32 / 7 % 20) as u16;
    if round >= TOTAL_PYRAMID_ROUNDS {
        round = 19;
    }
    if lvl != FRONTIER_LVL_50 as u32 {
        wildMons = sOpenLevelWildMonPointers[round];
    } else {
        wildMons = sLevel50WildMonPointers[round];
    }
    id = GetMonData3(&raw mut gEnemyParty[0], MON_DATA_SPECIES, null_mut()) - 1;
    SetMonData(
        &raw mut gEnemyParty[0],
        MON_DATA_SPECIES,
        &raw mut (*wildMons.at(id)).species as *mut c_void,
    );
    GetSpeciesName(name.as_mut_ptr(), (*wildMons.at(id)).species);
    SetMonData(
        &raw mut gEnemyParty[0],
        MON_DATA_NICKNAME,
        &raw mut name as *mut c_void,
    );
    if lvl != FRONTIER_LVL_50 as u32 {
        lvl = SetFacilityPtrsGetLevel() as u32;
        lvl -= (*wildMons.at(id)).lvl as u32;
        lvl = lvl - 5 + (Random() as i32 % 11) as u32;
    } else {
        lvl = (*wildMons.at(id)).lvl as u32 - 5 + (Random() as i32 % 11) as u32;
    }
    SetMonData(
        &raw mut gEnemyParty[0],
        MON_DATA_EXP,
        (&raw const gExperienceTables[gSpeciesInfo[(*wildMons.at(id)).species].growthRate][lvl])
            .cast_mut() as *mut c_void,
    );
    match (*wildMons.at(id)).abilityNum {
        0 | 1 => {
            SetMonData(
                &raw mut gEnemyParty[0],
                MON_DATA_ABILITY_NUM,
                &raw mut (*wildMons.at(id)).abilityNum as *mut c_void,
            );
        }
        _ => {
            if gSpeciesInfo[(*wildMons.at(id)).species].abilities[1] != 0 {
                i = (GetMonData3(&raw mut gEnemyParty[0], MON_DATA_PERSONALITY, null_mut()) % 2)
                    as i32;
                SetMonData(
                    &raw mut gEnemyParty[0],
                    MON_DATA_ABILITY_NUM,
                    &raw mut i as *mut c_void,
                );
            } else {
                i = 0;
                SetMonData(
                    &raw mut gEnemyParty[0],
                    MON_DATA_ABILITY_NUM,
                    &raw mut i as *mut c_void,
                );
            }
        }
    }
    i = 0;
    while i < MAX_MON_MOVES {
        SetMonMoveSlot(
            &raw mut gEnemyParty[0],
            (*wildMons.at(id)).moves[i],
            i as u8,
        );
        i += 1;
    }
    if (*gSaveBlock2Ptr).frontier.pyramidWinStreaks[(*gSaveBlock2Ptr).frontier.lvlMode()] >= 140 {
        id = (Random() as i32 % 17) as u32 + 15;
        i = 0;
        while i < NUM_STATS {
            SetMonData(
                &raw mut gEnemyParty[0],
                MON_DATA_HP_IV + i,
                &raw mut id as *mut c_void,
            );
            i += 1;
        }
    }
    CalculateMonStats(&raw mut gEnemyParty[0]);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetPyramidRunMultiplier() -> u8 {
    let mut id: u8 = GetPyramidFloorTemplateId();
    return sPyramidFloorTemplates[id].runMultiplier;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CurrentBattlePyramidLocation() -> u8 {
    if gMapHeader.mapLayoutId == LAYOUT_BATTLE_FRONTIER_BATTLE_PYRAMID_FLOOR {
        return PYRAMID_LOCATION_FLOOR;
    } else if gMapHeader.mapLayoutId == LAYOUT_BATTLE_FRONTIER_BATTLE_PYRAMID_TOP {
        return PYRAMID_LOCATION_TOP;
    } else {
        return PYRAMID_LOCATION_NONE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn InBattlePyramid_() -> u8 {
    return (gMapHeader.mapLayoutId == LAYOUT_BATTLE_FRONTIER_BATTLE_PYRAMID_FLOOR
        || gMapHeader.mapLayoutId == LAYOUT_BATTLE_FRONTIER_BATTLE_PYRAMID_TOP) as u8;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PausePyramidChallenge() {
    if CurrentBattlePyramidLocation() != PYRAMID_LOCATION_NONE {
        RestorePyramidPlayerParty();
        (*gSaveBlock2Ptr).frontier.challengeStatus = CHALLENGE_STATUS_PAUSED;
        VarSet(VAR_TEMP_PLAYING_PYRAMID_MUSIC, 0);
        LoadPlayerParty();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SoftResetInBattlePyramid() {
    if CurrentBattlePyramidLocation() != PYRAMID_LOCATION_NONE {
        DoSoftReset();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CopyPyramidTrainerSpeechBefore(trainerId: u16) {
    FrontierSpeechToString((*gFacilityTrainers.at(trainerId)).speechBefore.as_mut_ptr());
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CopyPyramidTrainerWinSpeech(trainerId: u16) {
    FrontierSpeechToString((*gFacilityTrainers.at(trainerId)).speechWin.as_mut_ptr());
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CopyPyramidTrainerLoseSpeech(trainerId: u16) {
    FrontierSpeechToString((*gFacilityTrainers.at(trainerId)).speechLose.as_mut_ptr());
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetTrainerEncounterMusicIdInBattlePyramid(trainerId: u16) -> u8 {
    let mut i: i32 = 0;
    i = 0;
    while i < 54 {
        if sTrainerClassEncounterMusic[i].trainerClass
            == gFacilityClassToTrainerClass[(*gFacilityTrainers.at(trainerId)).facilityClass]
        {
            return sTrainerClassEncounterMusic[i].trainerEncounterMusic;
        }
        i += 1;
    }
    return TRAINER_ENCOUNTER_MUSIC_MALE;
}
pub(crate) unsafe extern "C" fn BattlePyramidRetireChallenge() {
    ScriptContext_SetupScript(BattlePyramid_Retire.as_ptr().cast_mut());
}
pub(crate) unsafe extern "C" fn GetUniqueTrainerId(objectEventId: u8) -> u16 {
    let mut i: i32 = 0;
    let mut trainerId: u16 = 0;
    let mut lvlMode: u32 = (*gSaveBlock2Ptr).frontier.lvlMode() as u32;
    let mut challengeNum: u32 =
        ((*gSaveBlock2Ptr).frontier.pyramidWinStreaks[lvlMode] as i32 / 7) as u32;
    let mut floor: u32 = (*gSaveBlock2Ptr).frontier.curChallengeBattleNum as u32;
    if floor == FRONTIER_STAGES_PER_CHALLENGE as u32 {
        loop {
            trainerId = GetRandomScaledFrontierTrainerId(challengeNum as u8 + 1, floor as u8);
            i = 0;
            while i < objectEventId as i32 {
                if (*gSaveBlock2Ptr).frontier.trainerIds[i] == trainerId {
                    break;
                }
                i += 1;
            }
            if i == objectEventId as i32 {
                break;
            }
        }
    } else {
        loop {
            trainerId = GetRandomScaledFrontierTrainerId(challengeNum as u8, floor as u8);
            i = 0;
            while i < objectEventId as i32 {
                if (*gSaveBlock2Ptr).frontier.trainerIds[i] == trainerId {
                    break;
                }
                i += 1;
            }
            if i == objectEventId as i32 {
                break;
            }
        }
    }
    return trainerId;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GenerateBattlePyramidFloorLayout(
    backupMapData: *mut u16,
    setPlayerPosition: u8,
) {
    let mut y: i32 = 0;
    let mut x: i32 = 0;
    let mut i: i32 = 0;
    let mut entranceSquareId: u8 = 0;
    let mut exitSquareId: u8 = 0;
    let mut floorLayoutOffsets: *mut u8 = AllocZeroed(NUM_PYRAMID_FLOOR_SQUARES) as *mut u8;
    GetPyramidFloorLayoutOffsets(floorLayoutOffsets);
    GetPyramidEntranceAndExitSquareIds(&raw mut entranceSquareId, &raw mut exitSquareId);
    i = 0;
    while i < NUM_PYRAMID_FLOOR_SQUARES as i32 {
        let mut map: *mut u16 = null_mut();
        let mut mapLayout: *mut MapLayout = gMapLayouts
            [*floorLayoutOffsets.at(i) as i32 + LAYOUT_BATTLE_FRONTIER_BATTLE_PYRAMID_FLOOR as i32];
        let mut layoutMap: *mut u16 = (*mapLayout).map;
        gBackupMapLayout.map = backupMapData;
        gBackupMapLayout.width = (*mapLayout).width * PYRAMID_FLOOR_SQUARES_WIDE + MAP_OFFSET_W;
        gBackupMapLayout.height = (*mapLayout).height * PYRAMID_FLOOR_SQUARES_HIGH + MAP_OFFSET_H;
        map = gBackupMapLayout.map;
        map = map.at(
            gBackupMapLayout.width * (MAP_OFFSET + i / 4 * (*mapLayout).height)
                + MAP_OFFSET
                + i % 4 * (*mapLayout).width,
        );
        y = 0;
        while y < (*mapLayout).height {
            x = 0;
            while x < (*mapLayout).width {
                if *layoutMap.at(x) as i32 & MAPGRID_METATILE_ID_MASK != METATILE_BattlePyramid_Exit
                {
                    *map.at(x) = *layoutMap.at(x);
                } else if i != exitSquareId as i32 {
                    if i == entranceSquareId as i32 && setPlayerPosition == FALSE {
                        (*gSaveBlock1Ptr).pos.x =
                            (*mapLayout).width as i16 * (i % 4) as i16 + x as i16;
                        (*gSaveBlock1Ptr).pos.y =
                            (*mapLayout).height as i16 * (i / 4) as i16 + y as i16;
                    }
                    *map.at(x) = *layoutMap.at(x) & 64512 | METATILE_BattlePyramid_Floor;
                } else {
                    *map.at(x) = *layoutMap.at(x);
                }
                x += 1;
            }
            map = map.at(MAP_OFFSET_W + (*mapLayout).width * PYRAMID_FLOOR_SQUARES_WIDE);
            layoutMap = layoutMap.at((*mapLayout).width);
            y += 1;
        }
        i += 1;
    }
    RunOnLoadMapScript();
    Free(floorLayoutOffsets as *mut c_void);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn LoadBattlePyramidObjectEventTemplates() {
    let mut i: i32 = 0;
    let mut id: u8 = 0;
    let mut entranceSquareId: u8 = 0;
    let mut exitSquareId: u8 = 0;
    i = 0;
    while i < MAX_PYRAMID_TRAINERS {
        (*gSaveBlock2Ptr).frontier.trainerIds[i] = 0xFFFF;
        i += 1;
    }
    id = GetPyramidFloorTemplateId();
    GetPyramidEntranceAndExitSquareIds(&raw mut entranceSquareId, &raw mut exitSquareId);
    {
        {
            let mut tmp: u32 = 0;
            volatile_write(&raw mut tmp, 0);
            CpuSet(
                &raw mut tmp as *mut c_void,
                (*gSaveBlock1Ptr).objectEventTemplates.as_mut_ptr() as *mut c_void,
                0x5000180,
            );
        }
    }
    i = 0;
    while i < 2 {
        let mut objectPositionsType: u8 = 0;
        if i == OBJ_TRAINERS as i32 {
            objectPositionsType = sPyramidFloorTemplates[id].trainerPositions;
        } else {
            objectPositionsType = sPyramidFloorTemplates[id].itemPositions;
        }
        match objectPositionsType {
            OBJ_POSITIONS_UNIFORM => {
                SetPyramidObjectPositionsUniformly(i as u8);
            }
            OBJ_POSITIONS_IN_AND_NEAR_ENTRANCE => {
                if SetPyramidObjectPositionsInAndNearSquare(i as u8, entranceSquareId) != 0 {
                    SetPyramidObjectPositionsUniformly(i as u8);
                }
            }
            OBJ_POSITIONS_IN_AND_NEAR_EXIT => {
                if SetPyramidObjectPositionsInAndNearSquare(i as u8, exitSquareId) != 0 {
                    SetPyramidObjectPositionsUniformly(i as u8);
                }
            }
            OBJ_POSITIONS_NEAR_ENTRANCE => {
                if SetPyramidObjectPositionsNearSquare(i as u8, entranceSquareId) != 0 {
                    SetPyramidObjectPositionsUniformly(i as u8);
                }
            }
            OBJ_POSITIONS_NEAR_EXIT => {
                if SetPyramidObjectPositionsNearSquare(i as u8, exitSquareId) != 0 {
                    SetPyramidObjectPositionsUniformly(i as u8);
                }
            }
            _ => {}
        }
        i += 1;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn LoadBattlePyramidFloorObjectEventScripts() {
    let mut i: i32 = 0;
    let mut events: *mut ObjectEventTemplate = (*gSaveBlock1Ptr).objectEventTemplates.as_mut_ptr();
    i = 0;
    while i < OBJECT_EVENT_TEMPLATES_COUNT {
        if (*events.at(i)).graphicsId != OBJ_EVENT_GFX_ITEM_BALL {
            (*events.at(i)).script = BattlePyramid_TrainerBattle.as_ptr().cast_mut();
        } else {
            (*events.at(i)).script = BattlePyramid_FindItemBall.as_ptr().cast_mut();
        }
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn GetPyramidEntranceAndExitSquareIds(
    entranceSquareId: *mut u8,
    exitSquareId: *mut u8,
) {
    *entranceSquareId = ((*gSaveBlock2Ptr).frontier.pyramidRandoms[3] as i32 % 16) as u8;
    *exitSquareId = ((*gSaveBlock2Ptr).frontier.pyramidRandoms[0] as i32 % 16) as u8;
    if *entranceSquareId == *exitSquareId {
        *entranceSquareId = (((*gSaveBlock2Ptr).frontier.pyramidRandoms[3] as i32 + 1) % 16) as u8;
        *exitSquareId = (((*gSaveBlock2Ptr).frontier.pyramidRandoms[0] as i32
            + NUM_PYRAMID_FLOOR_SQUARES as i32
            - 1)
            % 16) as u8;
    }
}
pub(crate) unsafe extern "C" fn SetPyramidObjectPositionsUniformly(objType: u8) {
    let mut i: i32 = 0;
    let mut numObjects: i32 = 0;
    let mut objectStartIndex: i32 = 0;
    let mut squareId: i32 = 0;
    let mut bits: u32 = 0;
    let mut id: u8 = GetPyramidFloorTemplateId();
    let mut floorLayoutOffsets: *mut u8 = AllocZeroed(NUM_PYRAMID_FLOOR_SQUARES) as *mut u8;
    GetPyramidFloorLayoutOffsets(floorLayoutOffsets);
    squareId = (*gSaveBlock2Ptr).frontier.pyramidRandoms[2] as i32 % 16;
    if objType == OBJ_TRAINERS {
        numObjects = sPyramidFloorTemplates[id].numTrainers as i32;
        objectStartIndex = 0;
    } else {
        numObjects = sPyramidFloorTemplates[id].numItems as i32;
        objectStartIndex = sPyramidFloorTemplates[id].numTrainers as i32;
    }
    i = 0;
    while i < numObjects {
        loop {
            loop {
                if bits & 1 != 0 {
                    if gBitTable[squareId] & (*gSaveBlock2Ptr).frontier.pyramidRandoms[3] as u32
                        == 0
                    {
                        bits |= 2;
                    }
                } else {
                    if gBitTable[squareId] & (*gSaveBlock2Ptr).frontier.pyramidRandoms[3] as u32
                        != 0
                    {
                        bits |= 2;
                    }
                }
                if ({
                    squareId += 1;
                    squareId
                }) >= NUM_PYRAMID_FLOOR_SQUARES as i32
                {
                    squareId = 0;
                }
                if squareId == (*gSaveBlock2Ptr).frontier.pyramidRandoms[2] as i32 % 16 {
                    if bits & 1 != 0 {
                        bits |= 6;
                    } else {
                        bits |= 1;
                    }
                }
                if bits & 2 != 0 {
                    break;
                }
            }
            if !(bits & 4 == 0
                && TrySetPyramidObjectEventPositionInSquare(
                    objType,
                    floorLayoutOffsets,
                    squareId as u8,
                    objectStartIndex as u8 + i as u8,
                ) != 0)
            {
                break;
            }
        }
        bits &= 1;
        i += 1;
    }
    Free(floorLayoutOffsets as *mut c_void);
}
pub(crate) unsafe extern "C" fn SetPyramidObjectPositionsInAndNearSquare(
    objType: u8,
    squareId: u8,
) -> u8 {
    let mut i: i32 = 0;
    let mut objectStartIndex: i32 = 0;
    let mut borderedIndex: i32 = 0;
    let mut r7: i32 = 0;
    let mut numPlacedObjects: i32 = 0;
    let mut numObjects: i32 = 0;
    let mut id: u8 = GetPyramidFloorTemplateId();
    let mut floorLayoutOffsets: *mut u8 = AllocZeroed(NUM_PYRAMID_FLOOR_SQUARES) as *mut u8;
    GetPyramidFloorLayoutOffsets(floorLayoutOffsets);
    if objType == OBJ_TRAINERS {
        numObjects = sPyramidFloorTemplates[id].numTrainers as i32;
        objectStartIndex = 0;
    } else {
        numObjects = sPyramidFloorTemplates[id].numItems as i32;
        objectStartIndex = sPyramidFloorTemplates[id].numTrainers as i32;
    }
    i = 0;
    while i < numObjects {
        if r7 == 0 {
            if TrySetPyramidObjectEventPositionInSquare(
                objType,
                floorLayoutOffsets,
                squareId,
                objectStartIndex as u8 + i as u8,
            ) != 0
            {
                r7 = 1;
            } else {
                numPlacedObjects += 1;
            }
        }
        if r7 & 1 != 0 {
            if TrySetPyramidObjectEventPositionInSquare(
                objType,
                floorLayoutOffsets,
                sBorderedSquareIds[squareId][borderedIndex],
                objectStartIndex as u8 + i as u8,
            ) != 0
            {
                loop {
                    borderedIndex += 1;
                    if sBorderedSquareIds[squareId][borderedIndex] == 0xFF || borderedIndex >= 4 {
                        borderedIndex = 0;
                    }
                    r7 += 2;
                    if !(r7 >> 1 != 4
                        && TrySetPyramidObjectEventPositionInSquare(
                            objType,
                            floorLayoutOffsets,
                            sBorderedSquareIds[squareId][borderedIndex],
                            objectStartIndex as u8 + i as u8,
                        ) != 0)
                    {
                        break;
                    }
                }
                numPlacedObjects += 1;
            } else {
                borderedIndex += 1;
                if sBorderedSquareIds[squareId][borderedIndex] == 0xFF || borderedIndex >= 4 {
                    borderedIndex = 0;
                }
                numPlacedObjects += 1;
            }
        }
        if r7 >> 1 == 4 {
            break;
        }
        r7 &= 1;
        i += 1;
    }
    return (numObjects / 2 > numPlacedObjects) as u8;
}
pub(crate) unsafe extern "C" fn SetPyramidObjectPositionsNearSquare(
    objType: u8,
    squareId: u8,
) -> u8 {
    let mut i: i32 = 0;
    let mut objectStartIndex: i32 = 0;
    let mut borderOffset: i32 = 0;
    let mut numPlacedObjects: i32 = 0;
    let mut r8: i32 = 0;
    let mut numObjects: i32 = 0;
    let mut id: u8 = GetPyramidFloorTemplateId();
    let mut floorLayoutOffsets: *mut u8 = AllocZeroed(NUM_PYRAMID_FLOOR_SQUARES) as *mut u8;
    GetPyramidFloorLayoutOffsets(floorLayoutOffsets);
    if objType == OBJ_TRAINERS {
        numObjects = sPyramidFloorTemplates[id].numTrainers as i32;
        objectStartIndex = 0;
    } else {
        numObjects = sPyramidFloorTemplates[id].numItems as i32;
        objectStartIndex = sPyramidFloorTemplates[id].numTrainers as i32;
    }
    i = 0;
    while i < numObjects {
        if TrySetPyramidObjectEventPositionInSquare(
            objType,
            floorLayoutOffsets,
            sBorderedSquareIds[squareId][borderOffset],
            objectStartIndex as u8 + i as u8,
        ) != 0
        {
            loop {
                borderOffset += 1;
                if sBorderedSquareIds[squareId][borderOffset] == 0xFF || borderOffset >= 4 {
                    borderOffset = 0;
                }
                r8 += 1;
                if !(r8 != 4
                    && TrySetPyramidObjectEventPositionInSquare(
                        objType,
                        floorLayoutOffsets,
                        sBorderedSquareIds[squareId][borderOffset],
                        objectStartIndex as u8 + i as u8,
                    ) != 0)
                {
                    break;
                }
            }
            numPlacedObjects += 1;
        } else {
            borderOffset += 1;
            if sBorderedSquareIds[squareId][borderOffset] == 0xFF || borderOffset >= 4 {
                borderOffset = 0;
            }
            numPlacedObjects += 1;
        }
        if r8 == 4 {
            break;
        }
        i += 1;
    }
    return (numObjects / 2 > numPlacedObjects) as u8;
}
pub(crate) unsafe extern "C" fn TrySetPyramidObjectEventPositionInSquare(
    objType: u8,
    floorLayoutOffsets: *mut u8,
    squareId: u8,
    objectEventId: u8,
) -> u8 {
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    if (*gSaveBlock2Ptr).frontier.pyramidRandoms[0] as i32 & 1 != 0 {
        y = 7;
        while y > -1 {
            x = 7;
            while x > -1 {
                if TrySetPyramidObjectEventPositionAtCoords(
                    objType,
                    x as u8,
                    y as u8,
                    floorLayoutOffsets,
                    squareId,
                    objectEventId,
                ) == 0
                {
                    return FALSE;
                }
                x -= 1;
            }
            y -= 1;
        }
    } else {
        y = 0;
        while y < 8 {
            x = 0;
            while x < 8 {
                if TrySetPyramidObjectEventPositionAtCoords(
                    objType,
                    x as u8,
                    y as u8,
                    floorLayoutOffsets,
                    squareId,
                    objectEventId,
                ) == 0
                {
                    return FALSE;
                }
                x += 1;
            }
            y += 1;
        }
    }
    return TRUE;
}
pub(crate) unsafe extern "C" fn TrySetPyramidObjectEventPositionAtCoords(
    objType: u8,
    x: u8,
    y: u8,
    floorLayoutOffsets: *mut u8,
    squareId: u8,
    objectEventId: u8,
) -> u8 {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut mapHeader: *mut MapHeader = null_mut();
    let mut floorEvents: *mut ObjectEventTemplate =
        (*gSaveBlock1Ptr).objectEventTemplates.as_mut_ptr();
    mapHeader =
        Overworld_GetMapHeaderByGroupAndId(25, *floorLayoutOffsets.at(squareId) as u16 + 44);
    i = 0;
    while i < (*(*mapHeader).events).objectEventCount as i32 {
        'l1: {
            if (*(*(*mapHeader).events).objectEvents.at(i)).x != x as i16
                || (*(*(*mapHeader).events).objectEvents.at(i)).y != y as i16
            {
                break 'l1;
            }
            if objType != OBJ_TRAINERS
                || (*(*(*mapHeader).events).objectEvents.at(i)).graphicsId
                    == OBJ_EVENT_GFX_ITEM_BALL
            {
                if objType != OBJ_ITEMS
                    || (*(*(*mapHeader).events).objectEvents.at(i)).graphicsId
                        != OBJ_EVENT_GFX_ITEM_BALL
                {
                    break 'l1;
                }
            }
            j = 0;
            while j < objectEventId as i32 {
                if (*floorEvents.at(j)).x as i32 == x as i32 + squareId as i32 % 4 * 8
                    && (*floorEvents.at(j)).y as i32 == y as i32 + squareId as i32 / 4 * 8
                {
                    break;
                }
                j += 1;
            }
            if j == objectEventId as i32 {
                *floorEvents.at(objectEventId) = *(*(*mapHeader).events).objectEvents.at(i);
                (*floorEvents.at(objectEventId)).x += (squareId as i32 % 4) as i16 * 8;
                (*floorEvents.at(objectEventId)).y += (squareId as i32 / 4) as i16 * 8;
                (*floorEvents.at(objectEventId)).localId = objectEventId + 1;
                if (*floorEvents.at(objectEventId)).graphicsId != OBJ_EVENT_GFX_ITEM_BALL {
                    i = GetUniqueTrainerId(objectEventId) as i32;
                    (*floorEvents.at(objectEventId)).graphicsId =
                        GetBattleFacilityTrainerGfxId(i as u16);
                    (*gSaveBlock2Ptr).frontier.trainerIds[objectEventId] = i as u16;
                }
                return FALSE;
            }
        }
        i += 1;
    }
    return TRUE;
}
pub(crate) unsafe extern "C" fn GetPyramidFloorLayoutOffsets(mut layoutOffsets: *mut u8) {
    let mut i: i32 = 0;
    let mut rand: i32 = (*gSaveBlock2Ptr).frontier.pyramidRandoms[0] as i32
        | ((*gSaveBlock2Ptr).frontier.pyramidRandoms[1] as i32) << 16;
    let mut id: u8 = GetPyramidFloorTemplateId();
    i = 0;
    while i < NUM_PYRAMID_FLOOR_SQUARES as i32 {
        *layoutOffsets.at(i) =
            sPyramidFloorTemplates[id].layoutOffsets[if 0 != 0 { rand % 8 } else { rand & 7 }];
        rand >>= 3;
        if i == 7 {
            rand = (*gSaveBlock2Ptr).frontier.pyramidRandoms[2] as i32
                | ((*gSaveBlock2Ptr).frontier.pyramidRandoms[3] as i32) << 16;
            rand >>= 8;
        }
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn GetPyramidFloorTemplateId() -> u8 {
    let mut i: i32 = 0;
    let mut rand: i32 = (*gSaveBlock2Ptr).frontier.pyramidRandoms[3] as i32 % 100;
    let mut floor: i32 = (*gSaveBlock2Ptr).frontier.curChallengeBattleNum as i32;
    i = sFloorTemplateOffsets[floor] as i32;
    while i < 34 {
        if rand < sPyramidFloorTemplateOptions[i][0] as i32 {
            return sPyramidFloorTemplateOptions[i][1];
        }
        i += 1;
    }
    return 0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetNumBattlePyramidObjectEvents() -> u8 {
    let mut i: u8 = 0;
    let mut events: *mut ObjectEventTemplate = (*gSaveBlock1Ptr).objectEventTemplates.as_mut_ptr();
    i = 0;
    while i < OBJECT_EVENTS_COUNT {
        if (*events.at(i)).localId == LOCALID_NONE {
            break;
        }
        i += 1;
    }
    return i;
}
pub(crate) unsafe extern "C" fn InitPyramidBagItems(lvlMode: u8) {
    let mut i: i32 = 0;
    i = 0;
    while i < PYRAMID_BAG_ITEMS_COUNT as i32 {
        (*gSaveBlock2Ptr).frontier.pyramidBag.itemId[lvlMode][i] = ITEM_NONE;
        (*gSaveBlock2Ptr).frontier.pyramidBag.quantity[lvlMode][i] = ITEM_NONE as u8;
        i += 1;
    }
    AddPyramidBagItem(ITEM_HYPER_POTION, 1);
    AddPyramidBagItem(ITEM_ETHER, 1);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetBattlePyramidPickupItemId() -> u16 {
    let mut rand: i32 = 0;
    let mut i: u32 = 0;
    let mut lvlMode: u32 = (*gSaveBlock2Ptr).frontier.lvlMode() as u32;
    let mut round: i32 = (*gSaveBlock2Ptr).frontier.pyramidWinStreaks[lvlMode] as i32 / 7;
    if round >= TOTAL_PYRAMID_ROUNDS as i32 {
        round = 19;
    }
    rand = Random() as i32 % 100;
    i = 0;
    while i < 10 {
        if sPickupPercentages[i] as i32 > rand {
            break;
        }
        i += 1;
    }
    if i >= PICKUP_ITEMS_PER_ROUND {
        i = 9;
    }
    if lvlMode != FRONTIER_LVL_50 as u32 {
        return sPickupItemsLvlOpen[round][i];
    } else {
        return sPickupItemsLvl50[round][i];
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
