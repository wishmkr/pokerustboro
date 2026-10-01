//! Translated from `src/battle_pyramid.c` by tools/rustport/c2rs.py.
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
    clippy::eq_op,
    clippy::missing_transmute_annotations,
    clippy::type_complexity,
    dead_code,
    unused_assignments,
    unused_variables
)]

use crate::battle_main::{gBattleOutcome, gBattleTypeFlags};
use crate::battle_pyramid_bag::InitBattlePyramidBagCursorPosition;
use crate::battle_setup::{gTrainerBattleOpponent_A, gTrainerBattleOpponent_B};
use crate::battle_tower::{
    FrontierSpeechToString, GetBattleFacilityTrainerGfxId, GetRandomScaledFrontierTrainerId,
    SetFacilityPtrsGetLevel, gFacilityTrainers,
};
#[allow(unused_imports)]
use crate::c::*;
#[allow(unused_imports)]
use crate::consts::*;
use crate::event_data::VarSet;
use crate::ffi::{
    gSpecialVar_0x8000, gSpecialVar_0x8001, gSpecialVar_0x8005, gSpecialVar_0x8006,
    gSpecialVar_LastTalked, gSpecialVar_Result,
};
use crate::field_control_avatar::gSelectedObjectEvent;
use crate::field_message_box::ShowFieldMessage;
use crate::field_player_avatar::gObjectEvents;
use crate::field_screen_effect::WriteBattlePyramidViewScanlineEffectBuffer;
use crate::fieldmap::{SaveMapView, gBackupMapLayout, gMapHeader};
use crate::item::{AddBagItem, AddPyramidBagItem, CopyItemName};
use crate::load_save::LoadPlayerParty;
use crate::load_save::{gSaveBlock1Ptr, gSaveBlock2Ptr};
use crate::overworld::Overworld_GetMapHeaderByGroupAndId;
use crate::palette::gPaletteFade;
use crate::palette::gPlttBufferUnfaded;
use crate::party_menu::gSelectedOrderFromParty;
use crate::pokemon::{
    CalculateMonStats, GetMonData3, GetSpeciesName, SetMonData, SetMonMoveSlot, gEnemyParty,
    gPlayerParty,
};
use crate::random::{Random, Random2, SeedRng2};
use crate::save::TrySavingData;
use crate::script::RunOnLoadMapScript;
use crate::script::ScriptContext_SetupScript;
use crate::sound::PlaySE;
use crate::start_menu::ShowBattlePyramidStartMenu;
use crate::string_util::gStringVar1;
use crate::task::DestroyTask;
use crate::trainer_see::GetChosenApproachingTrainerObjectEventId;
#[allow(unused_imports)]
use crate::types::*;
use crate::util::gBitTable;
#[allow(unused_imports)]
use core::ffi::c_void;
#[allow(unused_imports)]
use core::mem::zeroed;
#[allow(unused_imports)]
use core::ptr::null_mut;
/// `CreateTask` with this module's view of its types.
#[inline]
unsafe fn CreateTask(a0: Option<unsafe fn(u8)>, a1: u8) -> u8 {
    unsafe { crate::task::CreateTask(core::mem::transmute(a0), a1) }
}
/// `Free` with this module's view of its types.
#[inline]
unsafe fn Free(a0: *mut c_void) {
    unsafe {
        crate::malloc::Free(a0 as _);
    }
}
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

static sBattlePyramidFunctions: Table<CArray<Option<unsafe fn()>, 18>> =
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

/// `AllocZeroed` with this module's view of its types.
#[inline]
unsafe fn AllocZeroed(a0: u32) -> *mut c_void {
    unsafe { crate::malloc::AllocZeroed(a0) as *mut c_void }
}
/// `CpuSet` with this module's view of its types.
#[inline]
unsafe fn CpuSet(a0: *mut c_void, a1: *mut c_void, a2: u32) {
    unsafe {
        crate::syscall::CpuSet(a0 as _, a1 as _, a2);
    }
}
/// `DoSoftReset` with this module's view of its types.
#[inline]
unsafe fn DoSoftReset() {
    unsafe {
        crate::agb_main::DoSoftReset();
    }
}

#[unsafe(no_mangle)]
pub unsafe fn CallBattlePyramidFunction() {
    sBattlePyramidFunctions[*(&raw const crate::ffi::gSpecialVar_0x8004)
        .cast::<u16>()
        .cast_mut()]
    .unwrap_unchecked()();
}
pub(crate) unsafe fn InitPyramidChallenge() {
    let mut isCurrent: u32 = 0;
    let lvlMode: u32 = (*gSaveBlock2Ptr).frontier.lvlMode() as u32;
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
pub(crate) unsafe fn GetBattlePyramidData() {
    let lvlMode: u32 = (*gSaveBlock2Ptr).frontier.lvlMode() as u32;
    match *(&raw const crate::ffi::gSpecialVar_0x8005)
        .cast::<u16>()
        .cast_mut()
    {
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
pub(crate) unsafe fn SetBattlePyramidData() {
    let lvlMode: u32 = (*gSaveBlock2Ptr).frontier.lvlMode() as u32;
    match *(&raw const crate::ffi::gSpecialVar_0x8005)
        .cast::<u16>()
        .cast_mut()
    {
        PYRAMID_DATA_PRIZE => {
            (*gSaveBlock2Ptr).frontier.pyramidPrize = *(&raw const crate::ffi::gSpecialVar_0x8006)
                .cast::<u16>()
                .cast_mut();
        }
        PYRAMID_DATA_WIN_STREAK => {
            (*gSaveBlock2Ptr).frontier.pyramidWinStreaks[lvlMode] =
                *(&raw const crate::ffi::gSpecialVar_0x8006)
                    .cast::<u16>()
                    .cast_mut();
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
pub(crate) unsafe fn SavePyramidChallenge() {
    (*gSaveBlock2Ptr).frontier.challengeStatus = gSpecialVar_0x8005 as u8;
    VarSet(VAR_TEMP_CHALLENGE_STATUS, 0);
    (*gSaveBlock2Ptr).frontier.set_challengePaused(TRUE);
    SaveMapView();
    TrySavingData(SAVE_LINK);
}
pub(crate) unsafe fn SetBattlePyramidPrize() {
    if (*gSaveBlock2Ptr).frontier.pyramidWinStreaks[(*gSaveBlock2Ptr).frontier.lvlMode()] > 41 {
        (*gSaveBlock2Ptr).frontier.pyramidPrize = sLongStreakRewardItems[Random() % 9];
    } else {
        (*gSaveBlock2Ptr).frontier.pyramidPrize = sShortStreakRewardItems[Random() % 6];
    }
}
pub(crate) unsafe fn GiveBattlePyramidPrize() {
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
pub(crate) unsafe fn SeedPyramidFloor() {
    for i in 0..4i32 {
        (*gSaveBlock2Ptr).frontier.pyramidRandoms[i] = Random();
    }
    (*gSaveBlock2Ptr).frontier.pyramidTrainerFlags = 0;
}
pub(crate) unsafe fn SetPickupItem() {
    let lvlMode: u32 = (*gSaveBlock2Ptr).frontier.lvlMode() as u32;
    let floor: u32 = (*gSaveBlock2Ptr).frontier.curChallengeBattleNum as u32;
    let mut round: u32 =
        ((*gSaveBlock2Ptr).frontier.pyramidWinStreaks[lvlMode] as i32 / 7 % 20) as u32;
    if round >= TOTAL_PYRAMID_ROUNDS as u32 {
        round = 19;
    }
    let id: u8 = GetPyramidFloorTemplateId();
    let itemIndex: i32 =
        gSpecialVar_LastTalked as i32 - sPyramidFloorTemplates[id].numTrainers as i32 - 1;
    let mut rand: i32 = (*gSaveBlock2Ptr).frontier.pyramidRandoms[itemIndex / 2] as i32;
    SeedRng2(rand as u16);
    for i in 0..(itemIndex + 1) {
        rand = Random2() as i32 % 100;
    }
    let mut i: i32 = sPickupItemOffsets[floor] as i32;
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
pub(crate) unsafe fn HidePyramidItem() {
    let events: *mut ObjectEventTemplate = (*gSaveBlock1Ptr).objectEventTemplates.as_mut_ptr();
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
pub(crate) unsafe fn SetPyramidFacilityTrainers() {
    gFacilityTrainers = (*(&raw const crate::data::battle_tower::gBattleFrontierTrainers)
        .cast::<CArray<BattleFrontierTrainer, 0>>())
    .as_ptr()
    .cast_mut();
}
pub(crate) unsafe fn ShowPostBattleHintText() {
    let mut id: u8 = 0;
    let mut textGroup: i32 = 0;
    let mut textIndex: i32 = 0;
    let events: *mut ObjectEventTemplate = (*gSaveBlock1Ptr).objectEventTemplates.as_mut_ptr();
    let trainerId: u16 = LocalIdToPyramidTrainerId(gObjectEvents[gSelectedObjectEvent].localId);
    for i in 0..50i32 {
        if sTrainerTextGroups[i][0] == (*gFacilityTrainers.at(trainerId)).facilityClass {
            textGroup = sTrainerTextGroups[i][1] as i32;
            break;
        }
    }
    let mut hintType: i32 =
        sHintTextTypes[gObjectEvents[gSelectedObjectEvent].localId as i32 - 1] as i32;
    let mut i: i32 = 0;
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
                for i in 0..MAX_PYRAMID_TRAINERS {
                    if gBitTable[i] & (*gSaveBlock2Ptr).frontier.pyramidTrainerFlags as u32 != 0 {
                        textIndex -= 1;
                    }
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
pub(crate) unsafe fn UpdatePyramidWinStreak() {
    let lvlMode: u32 = (*gSaveBlock2Ptr).frontier.lvlMode() as u32;
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
pub(crate) unsafe fn GetCurrentBattlePyramidLocation() {
    gSpecialVar_Result = CurrentBattlePyramidLocation() as u16;
}
pub(crate) unsafe fn UpdatePyramidLightRadius() {
    match *(&raw const crate::ffi::gSpecialVar_0x8006)
        .cast::<u16>()
        .cast_mut()
    {
        PYRAMID_LIGHT_SET_RADIUS => {
            (*gSaveBlock2Ptr).frontier.pyramidLightRadius = gSpecialVar_0x8005 as u8;
        }
        PYRAMID_LIGHT_INCR_RADIUS => match *(&raw const crate::ffi::gSpecialVar_Result)
            .cast::<u16>()
            .cast_mut()
        {
            0 => {
                if gPaletteFade.active() == 0 {
                    if (*gSaveBlock2Ptr).frontier.pyramidLightRadius >= 120 {
                        (*gSaveBlock2Ptr).frontier.pyramidLightRadius = 120;
                    } else {
                        PlaySE(
                            *(&raw const crate::ffi::gSpecialVar_0x8007)
                                .cast::<u16>()
                                .cast_mut(),
                        );
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
pub(crate) unsafe fn ClearPyramidPartyHeldItems() {
    let mut j: i32 = 0;
    let mut item: u16 = 0;
    for i in 0..PARTY_SIZE {
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
    }
}
pub(crate) unsafe fn SetPyramidFloorPalette() {
    CreateTask(Some(Task_SetPyramidFloorPalette), 0);
}
pub(crate) unsafe fn Task_SetPyramidFloorPalette(taskId: u8) {
    if gPaletteFade.active() != 0 {
        CpuSet(
            (*(&raw const crate::data::graphics::gBattlePyramidFloor_Pal)
                .cast::<CArray<CArray<u16, 16>, 0>>())
                [(*gSaveBlock2Ptr).frontier.curChallengeBattleNum]
                .as_ptr()
                .cast_mut() as *mut c_void,
            &raw mut gPlttBufferUnfaded[96] as *mut c_void,
            16,
        );
        DestroyTask(taskId);
    }
}
pub(crate) unsafe fn BattlePyramidStartMenu() {
    ShowBattlePyramidStartMenu();
}
pub(crate) unsafe fn RestorePyramidPlayerParty() {
    let mut l: i32 = 0;
    let mut i: i32 = 0;
    while i < FRONTIER_PARTY_SIZE {
        let partyIndex: i32 = (*gSaveBlock2Ptr).frontier.selectedPartyMons[i] as i32 - 1;
        for j in 0..FRONTIER_PARTY_SIZE {
            if GetMonData3(
                &raw mut (*gSaveBlock1Ptr).playerParty[partyIndex],
                MON_DATA_SPECIES,
                null_mut(),
            ) == GetMonData3(&raw mut gPlayerParty[j], MON_DATA_SPECIES, null_mut())
            {
                for k in 0..MAX_MON_MOVES {
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
                }
                (*gSaveBlock1Ptr).playerParty[partyIndex] = gPlayerParty[j];
                gSelectedOrderFromParty[j] = partyIndex as u8 + 1;
                break;
            }
        }
        i += 1;
    }
    for i in 0..FRONTIER_PARTY_SIZE {
        (*gSaveBlock2Ptr).frontier.selectedPartyMons[i] = gSelectedOrderFromParty[i] as u16;
    }
}
unsafe fn GetPostBattleDirectionHintTextIndex(
    hintType: *mut i32,
    minDistanceForExitHint: u8,
    defaultHintType: u8,
) -> u8 {
    let mut x: i32 = 0;
    let mut textIndex: u8 = 0;
    let mut map: *mut u16 = gBackupMapLayout.map;
    map = map.at(gBackupMapLayout.width * MAP_OFFSET + MAP_OFFSET);
    let mut y: i32 = 0;
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
    textIndex
}
pub unsafe fn LocalIdToPyramidTrainerId(localId: u8) -> u16 {
    (*gSaveBlock2Ptr).frontier.trainerIds[localId as i32 - 1]
}
pub unsafe fn GetBattlePyramidTrainerFlag(eventId: u8) -> u8 {
    (*gSaveBlock2Ptr).frontier.pyramidTrainerFlags
        & (*(&raw const crate::util::gBitTable).cast::<CArray<u32, 0>>())
            [gObjectEvents[eventId].localId as i32 - 1] as u8
}
pub unsafe fn MarkApproachingPyramidTrainersAsBattled() {
    MarkPyramidTrainerAsBattled(gTrainerBattleOpponent_A);
    if gBattleTypeFlags & BATTLE_TYPE_TWO_OPPONENTS != 0 {
        gSelectedObjectEvent = GetChosenApproachingTrainerObjectEventId(1);
        MarkPyramidTrainerAsBattled(gTrainerBattleOpponent_B);
    }
}
unsafe fn MarkPyramidTrainerAsBattled(trainerId: u16) {
    for i in 0..MAX_PYRAMID_TRAINERS {
        if (*gSaveBlock2Ptr).frontier.trainerIds[i] == trainerId {
            (*gSaveBlock2Ptr).frontier.pyramidTrainerFlags |= gBitTable[i] as u8;
        }
    }
    gObjectEvents[gSelectedObjectEvent].movementType = MOVEMENT_TYPE_WANDER_AROUND;
    (*gSaveBlock1Ptr).objectEventTemplates[gSpecialVar_LastTalked as i32 - 1].movementType =
        MOVEMENT_TYPE_WANDER_AROUND;
    gObjectEvents[gSelectedObjectEvent].initialCoords.x =
        gObjectEvents[gSelectedObjectEvent].currentCoords.x;
    gObjectEvents[gSelectedObjectEvent].initialCoords.y =
        gObjectEvents[gSelectedObjectEvent].currentCoords.y;
}
pub unsafe fn GenerateBattlePyramidWildMon() {
    let mut name: CArray<u8, 11> = zeroed();
    let mut i: i32 = 0;
    let mut wildMons: *mut PyramidWildMon = null_mut();
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
    let mut id: u32 = GetMonData3(&raw mut gEnemyParty[0], MON_DATA_SPECIES, null_mut()) - 1;
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
        (&raw const (*(&raw const crate::data::pokemon::gExperienceTables)
            .cast::<CArray<CArray<u32, 101>, 0>>())
            [(*(&raw const crate::data::pokemon::gSpeciesInfo).cast::<CArray<SpeciesInfo, 0>>())
                [(*wildMons.at(id)).species]
                .growthRate][lvl])
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
            if (*(&raw const crate::data::pokemon::gSpeciesInfo).cast::<CArray<SpeciesInfo, 0>>())
                [(*wildMons.at(id)).species]
                .abilities[1]
                != 0
            {
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
pub unsafe fn GetPyramidRunMultiplier() -> u8 {
    let id: u8 = GetPyramidFloorTemplateId();
    sPyramidFloorTemplates[id].runMultiplier
}
#[unsafe(no_mangle)]
pub unsafe fn CurrentBattlePyramidLocation() -> u8 {
    if gMapHeader.mapLayoutId == LAYOUT_BATTLE_FRONTIER_BATTLE_PYRAMID_FLOOR {
        return PYRAMID_LOCATION_FLOOR;
    } else if gMapHeader.mapLayoutId == LAYOUT_BATTLE_FRONTIER_BATTLE_PYRAMID_TOP {
        return PYRAMID_LOCATION_TOP;
    } else {
        return PYRAMID_LOCATION_NONE;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
pub unsafe fn InBattlePyramid_() -> u8 {
    (gMapHeader.mapLayoutId == LAYOUT_BATTLE_FRONTIER_BATTLE_PYRAMID_FLOOR
        || gMapHeader.mapLayoutId == LAYOUT_BATTLE_FRONTIER_BATTLE_PYRAMID_TOP) as u8
}
pub unsafe fn PausePyramidChallenge() {
    if CurrentBattlePyramidLocation() != PYRAMID_LOCATION_NONE {
        RestorePyramidPlayerParty();
        (*gSaveBlock2Ptr).frontier.challengeStatus = CHALLENGE_STATUS_PAUSED;
        VarSet(VAR_TEMP_PLAYING_PYRAMID_MUSIC, 0);
        LoadPlayerParty();
    }
}
pub unsafe fn SoftResetInBattlePyramid() {
    if CurrentBattlePyramidLocation() != PYRAMID_LOCATION_NONE {
        DoSoftReset();
    }
}
pub unsafe fn CopyPyramidTrainerSpeechBefore(trainerId: u16) {
    FrontierSpeechToString((*gFacilityTrainers.at(trainerId)).speechBefore.as_mut_ptr());
}
pub unsafe fn CopyPyramidTrainerWinSpeech(trainerId: u16) {
    FrontierSpeechToString((*gFacilityTrainers.at(trainerId)).speechWin.as_mut_ptr());
}
pub unsafe fn CopyPyramidTrainerLoseSpeech(trainerId: u16) {
    FrontierSpeechToString((*gFacilityTrainers.at(trainerId)).speechLose.as_mut_ptr());
}
pub unsafe fn GetTrainerEncounterMusicIdInBattlePyramid(trainerId: u16) -> u8 {
    for i in 0..54i32 {
        if sTrainerClassEncounterMusic[i].trainerClass
            == (*(&raw const crate::data::pokemon::gFacilityClassToTrainerClass)
                .cast::<CArray<u8, 0>>())[(*gFacilityTrainers.at(trainerId)).facilityClass]
        {
            return sTrainerClassEncounterMusic[i].trainerEncounterMusic;
        }
    }
    TRAINER_ENCOUNTER_MUSIC_MALE
}
unsafe fn BattlePyramidRetireChallenge() {
    ScriptContext_SetupScript(
        (*crate::asmdata::BattlePyramid_Retire.cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
    );
}
unsafe fn GetUniqueTrainerId(objectEventId: u8) -> u16 {
    let mut i: i32 = 0;
    let mut trainerId: u16 = 0;
    let lvlMode: u32 = (*gSaveBlock2Ptr).frontier.lvlMode() as u32;
    let challengeNum: u32 =
        ((*gSaveBlock2Ptr).frontier.pyramidWinStreaks[lvlMode] as i32 / 7) as u32;
    let floor: u32 = (*gSaveBlock2Ptr).frontier.curChallengeBattleNum as u32;
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
    trainerId
}
pub unsafe fn GenerateBattlePyramidFloorLayout(backupMapData: *mut u16, setPlayerPosition: u8) {
    let mut y: i32 = 0;
    let mut x: i32 = 0;
    let mut entranceSquareId: u8 = 0;
    let mut exitSquareId: u8 = 0;
    let floorLayoutOffsets: *mut u8 = AllocZeroed(NUM_PYRAMID_FLOOR_SQUARES) as *mut u8;
    GetPyramidFloorLayoutOffsets(floorLayoutOffsets);
    GetPyramidEntranceAndExitSquareIds(&raw mut entranceSquareId, &raw mut exitSquareId);
    for i in 0..(NUM_PYRAMID_FLOOR_SQUARES as i32) {
        let mut map: *mut u16 = null_mut();
        let mapLayout: *mut MapLayout = (*crate::asmdata::gMapLayouts
            .cast::<CArray<*mut MapLayout, 0>>())
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
    }
    RunOnLoadMapScript();
    Free(floorLayoutOffsets as *mut c_void);
}
pub unsafe fn LoadBattlePyramidObjectEventTemplates() {
    let mut entranceSquareId: u8 = 0;
    let mut exitSquareId: u8 = 0;
    for i in 0..MAX_PYRAMID_TRAINERS {
        (*gSaveBlock2Ptr).frontier.trainerIds[i] = 0xFFFF;
    }
    let id: u8 = GetPyramidFloorTemplateId();
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
    for i in 0..2i32 {
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
            OBJ_POSITIONS_NEAR_EXIT
                if SetPyramidObjectPositionsNearSquare(i as u8, exitSquareId) != 0 =>
            {
                SetPyramidObjectPositionsUniformly(i as u8);
            }
            _ => {}
        }
    }
}
pub unsafe fn LoadBattlePyramidFloorObjectEventScripts() {
    let events: *mut ObjectEventTemplate = (*gSaveBlock1Ptr).objectEventTemplates.as_mut_ptr();
    for i in 0..OBJECT_EVENT_TEMPLATES_COUNT {
        if (*events.at(i)).graphicsId != OBJ_EVENT_GFX_ITEM_BALL {
            (*events.at(i)).script = (*crate::asmdata::BattlePyramid_TrainerBattle
                .cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut();
        } else {
            (*events.at(i)).script = (*crate::asmdata::BattlePyramid_FindItemBall
                .cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut();
        }
    }
}
unsafe fn GetPyramidEntranceAndExitSquareIds(entranceSquareId: *mut u8, exitSquareId: *mut u8) {
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
unsafe fn SetPyramidObjectPositionsUniformly(objType: u8) {
    let mut numObjects: i32 = 0;
    let mut objectStartIndex: i32 = 0;
    let mut bits: u32 = 0;
    let id: u8 = GetPyramidFloorTemplateId();
    let floorLayoutOffsets: *mut u8 = AllocZeroed(NUM_PYRAMID_FLOOR_SQUARES) as *mut u8;
    GetPyramidFloorLayoutOffsets(floorLayoutOffsets);
    let mut squareId: i32 = (*gSaveBlock2Ptr).frontier.pyramidRandoms[2] as i32 % 16;
    if objType == OBJ_TRAINERS {
        numObjects = sPyramidFloorTemplates[id].numTrainers as i32;
        objectStartIndex = 0;
    } else {
        numObjects = sPyramidFloorTemplates[id].numItems as i32;
        objectStartIndex = sPyramidFloorTemplates[id].numTrainers as i32;
    }
    for i in 0..numObjects {
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
    }
    Free(floorLayoutOffsets as *mut c_void);
}
unsafe fn SetPyramidObjectPositionsInAndNearSquare(objType: u8, squareId: u8) -> u8 {
    let mut objectStartIndex: i32 = 0;
    let mut borderedIndex: i32 = 0;
    let mut r7: i32 = 0;
    let mut numPlacedObjects: i32 = 0;
    let mut numObjects: i32 = 0;
    let id: u8 = GetPyramidFloorTemplateId();
    let floorLayoutOffsets: *mut u8 = AllocZeroed(NUM_PYRAMID_FLOOR_SQUARES) as *mut u8;
    GetPyramidFloorLayoutOffsets(floorLayoutOffsets);
    if objType == OBJ_TRAINERS {
        numObjects = sPyramidFloorTemplates[id].numTrainers as i32;
        objectStartIndex = 0;
    } else {
        numObjects = sPyramidFloorTemplates[id].numItems as i32;
        objectStartIndex = sPyramidFloorTemplates[id].numTrainers as i32;
    }
    for i in 0..numObjects {
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
    }
    (numObjects / 2 > numPlacedObjects) as u8
}
unsafe fn SetPyramidObjectPositionsNearSquare(objType: u8, squareId: u8) -> u8 {
    let mut objectStartIndex: i32 = 0;
    let mut borderOffset: i32 = 0;
    let mut numPlacedObjects: i32 = 0;
    let mut r8: i32 = 0;
    let mut numObjects: i32 = 0;
    let id: u8 = GetPyramidFloorTemplateId();
    let floorLayoutOffsets: *mut u8 = AllocZeroed(NUM_PYRAMID_FLOOR_SQUARES) as *mut u8;
    GetPyramidFloorLayoutOffsets(floorLayoutOffsets);
    if objType == OBJ_TRAINERS {
        numObjects = sPyramidFloorTemplates[id].numTrainers as i32;
        objectStartIndex = 0;
    } else {
        numObjects = sPyramidFloorTemplates[id].numItems as i32;
        objectStartIndex = sPyramidFloorTemplates[id].numTrainers as i32;
    }
    for i in 0..numObjects {
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
    }
    (numObjects / 2 > numPlacedObjects) as u8
}
unsafe fn TrySetPyramidObjectEventPositionInSquare(
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
        for y in 0..8i32 {
            for x in 0..8i32 {
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
            }
        }
    }
    TRUE
}
unsafe fn TrySetPyramidObjectEventPositionAtCoords(
    objType: u8,
    x: u8,
    y: u8,
    floorLayoutOffsets: *mut u8,
    squareId: u8,
    objectEventId: u8,
) -> u8 {
    let mut j: i32 = 0;
    let floorEvents: *mut ObjectEventTemplate = (*gSaveBlock1Ptr).objectEventTemplates.as_mut_ptr();
    let mapHeader: *mut MapHeader =
        Overworld_GetMapHeaderByGroupAndId(25, *floorLayoutOffsets.at(squareId) as u16 + 44);
    let mut i: i32 = 0;
    while i < (*(*mapHeader).events).objectEventCount as i32 {
        'l1: {
            if (*(*(*mapHeader).events).objectEvents.at(i)).x != x as i16
                || (*(*(*mapHeader).events).objectEvents.at(i)).y != y as i16
            {
                break 'l1;
            }
            if (objType != OBJ_TRAINERS
                || (*(*(*mapHeader).events).objectEvents.at(i)).graphicsId
                    == OBJ_EVENT_GFX_ITEM_BALL)
                && (objType != OBJ_ITEMS
                    || (*(*(*mapHeader).events).objectEvents.at(i)).graphicsId
                        != OBJ_EVENT_GFX_ITEM_BALL)
            {
                break 'l1;
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
    TRUE
}
unsafe fn GetPyramidFloorLayoutOffsets(layoutOffsets: *mut u8) {
    let mut rand: i32 = (*gSaveBlock2Ptr).frontier.pyramidRandoms[0] as i32
        | ((*gSaveBlock2Ptr).frontier.pyramidRandoms[1] as i32) << 16;
    let id: u8 = GetPyramidFloorTemplateId();
    for i in 0..(NUM_PYRAMID_FLOOR_SQUARES as i32) {
        *layoutOffsets.at(i) =
            sPyramidFloorTemplates[id].layoutOffsets[if 0 != 0 { rand % 8 } else { rand & 7 }];
        rand >>= 3;
        if i == 7 {
            rand = (*gSaveBlock2Ptr).frontier.pyramidRandoms[2] as i32
                | ((*gSaveBlock2Ptr).frontier.pyramidRandoms[3] as i32) << 16;
            rand >>= 8;
        }
    }
}
unsafe fn GetPyramidFloorTemplateId() -> u8 {
    let rand: i32 = (*gSaveBlock2Ptr).frontier.pyramidRandoms[3] as i32 % 100;
    let floor: i32 = (*gSaveBlock2Ptr).frontier.curChallengeBattleNum as i32;
    for i in (sFloorTemplateOffsets[floor] as i32)..34 {
        if rand < sPyramidFloorTemplateOptions[i][0] as i32 {
            return sPyramidFloorTemplateOptions[i][1];
        }
    }
    0
}
pub unsafe fn GetNumBattlePyramidObjectEvents() -> u8 {
    let events: *mut ObjectEventTemplate = (*gSaveBlock1Ptr).objectEventTemplates.as_mut_ptr();
    let mut i: u8 = 0;
    while i < OBJECT_EVENTS_COUNT {
        if (*events.at(i)).localId == LOCALID_NONE {
            break;
        }
        i += 1;
    }
    i
}
unsafe fn InitPyramidBagItems(lvlMode: u8) {
    for i in 0..(PYRAMID_BAG_ITEMS_COUNT as i32) {
        (*gSaveBlock2Ptr).frontier.pyramidBag.itemId[lvlMode][i] = ITEM_NONE;
        (*gSaveBlock2Ptr).frontier.pyramidBag.quantity[lvlMode][i] = ITEM_NONE as u8;
    }
    AddPyramidBagItem(ITEM_HYPER_POTION, 1);
    AddPyramidBagItem(ITEM_ETHER, 1);
}
pub unsafe fn GetBattlePyramidPickupItemId() -> u16 {
    let lvlMode: u32 = (*gSaveBlock2Ptr).frontier.lvlMode() as u32;
    let mut round: i32 = (*gSaveBlock2Ptr).frontier.pyramidWinStreaks[lvlMode] as i32 / 7;
    if round >= TOTAL_PYRAMID_ROUNDS as i32 {
        round = 19;
    }
    let rand: i32 = Random() as i32 % 100;
    let mut i: u32 = 0;
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
        0
    }
}
