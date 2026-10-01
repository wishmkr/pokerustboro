//! Translated from `src/trainer_hill.c` by tools/rustport/c2rs.py.
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
    clippy::if_same_then_else,
    clippy::type_complexity,
    dead_code,
    unused_assignments
)]

use crate::agb_main::{ClearTrainerHillVBlankCounter, SetTrainerHillVBlankCounter};
use crate::battle_main::{gBattleOutcome, gBattleTypeFlags};
use crate::battle_setup::{gTrainerBattleOpponent_A, gTrainerBattleOpponent_B};
use crate::battle_tower::{
    FacilityClassToGraphicsId, FrontierSpeechToString, GetHighestLevelInPlayerParty,
};
#[allow(unused_imports)]
use crate::c::*;
#[allow(unused_imports)]
use crate::consts::*;
use crate::ereader_helpers::ReadTrainerHillAndValidate;
use crate::event_data::{FlagGet, VarGet};
use crate::ffi::gSpecialVar_Result;
use crate::field_message_box::ShowFieldMessageFromBuffer;
use crate::field_player_avatar::gObjectEvents;
use crate::fieldmap::{InitMapFromSavedGame, gBackupMapLayout, gMapHeader};
use crate::international_string_util::{GetStringCenterAlignXOffset, GetStringRightAlignXOffset};
use crate::item::{AddBagItem, CopyItemName};
use crate::load_save::{gSaveBlock1Ptr, gSaveBlock2Ptr};
use crate::menu::AddTextPrinterParameterized3;
use crate::overworld::Overworld_GetMapHeaderByGroupAndId;
use crate::palette::LoadPalette;
use crate::pokemon::{
    CalculateMonStats, CreateBattleTowerMon, GetMonData3, SetMonData, ZeroEnemyPartyMons,
    gEnemyParty,
};
use crate::script::RunOnLoadMapScript;
use crate::string_util::{ConvertIntToDecimalStringN, StringCopy, StringExpandPlaceholders};
use crate::string_util::{gStringVar1, gStringVar2, gStringVar3, gStringVar4};
#[allow(unused_imports)]
use crate::types::*;
use crate::util::gBitTable;
use crate::window::{CopyWindowToVram, FillWindowPixelBuffer, PutWindowTilemap};
#[allow(unused_imports)]
use core::ffi::c_void;
#[allow(unused_imports)]
use core::mem::zeroed;
#[allow(unused_imports)]
use core::ptr::null_mut;
/// `Free` with this module's view of its types.
#[inline]
unsafe fn Free(a0: *mut c_void) {
    unsafe {
        crate::malloc::Free(a0 as _);
    }
}
// Data tables (translate with cdata.py): sChallenge_JPDefault sFloors_JPDefault sChallenge_Normal sFloors_Normal sChallenge_Variety sFloors_Variety sChallenge_Unique sFloors_Unique sChallenge_Expert sFloors_Expert sTrainerClassesAndMusic sPrizeListRareCandy1 sPrizeListLuxuryBall1 sPrizeListMaxRevive1 sPrizeListMaxEther1 sPrizeListElixir1 sPrizeListRoar sPrizeListSludgeBomb sPrizeListToxic sPrizeListSunnyDay sPrizeListEarthQuake sPrizeListRareCandy2 sPrizeListLuxuryBall2 sPrizeListMaxRevive2 sPrizeListMaxEther2 sPrizeListElixir2 sPrizeListBrickBreak sPrizeListTorment sPrizeListSkillSwap sPrizeListGigaDrain sPrizeListAttract sPrizeLists1 sPrizeLists2 sPrizeListSets sEReader_Pal sRecordWinColors sChallengeData sFloorStrings sHillFunctions sModeStrings sTrainerObjectEventTemplate sNextFloorMapNum sTrainerPartySlots

/// `__typeof__(*((__typeof__(sHillData))0))`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct typeof___sHillData_0_t {
    pub floorId: u8,
    pub challenge: TrainerHillChallenge,
    pub floors: CArray<TrainerHillFloor, 4>,
}

unsafe impl Sync for typeof___sHillData_0_t {}

/// `struct FloorTrainers`
#[repr(C, align(4))]
#[derive(Clone, Copy)]
pub struct FloorTrainers {
    pub name: CArray<CArray<u8, 11>, 2>,
    pub facilityClass: CArray<u8, 2>,
}

unsafe impl Sync for FloorTrainers {}

/// `__typeof__(sTrainerClassesAndMusic[0])`
#[repr(C, align(4))]
#[derive(Clone, Copy)]
pub struct sTrainerClassesAndMusic_0_t {
    pub trainerClass: u8,
    pub musicId: u8,
}

unsafe impl Sync for sTrainerClassesAndMusic_0_t {}

#[cfg(target_arch = "arm")]
const _: () = {
    #[allow(unused_imports)]
    use core::mem::{offset_of, size_of};
    assert!(size_of::<typeof___sHillData_0_t>() == 3820);
    assert!(offset_of!(typeof___sHillData_0_t, floorId) == 0);
    assert!(offset_of!(typeof___sHillData_0_t, challenge) == 4);
    assert!(offset_of!(typeof___sHillData_0_t, floors) == 12);
    assert!(size_of::<FloorTrainers>() == 24);
    assert!(offset_of!(FloorTrainers, name) == 0);
    assert!(offset_of!(FloorTrainers, facilityClass) == 22);
    assert!(size_of::<sTrainerClassesAndMusic_0_t>() == 4);
    assert!(offset_of!(sTrainerClassesAndMusic_0_t, trainerClass) == 0);
    assert!(offset_of!(sTrainerClassesAndMusic_0_t, musicId) == 1);
};

const HILL_MAX_TIME: u32 = 0x34bbf;

static sChallengeData: Table<CArray<*mut TrainerHillChallenge, 4>> =
    Table((&raw const crate::data::trainer_hill::sChallengeData).cast());
static sEReader_Pal: Table<CArray<u16, 16>> =
    Table((&raw const crate::data::trainer_hill::sEReader_Pal).cast());
static sHillFunctions: Table<CArray<Option<unsafe fn()>, 18>> =
    Table((&raw const crate::data::trainer_hill::sHillFunctions).cast());
static sModeStrings: Table<CArray<*mut u8, 4>> =
    Table((&raw const crate::data::trainer_hill::sModeStrings).cast());
static sNextFloorMapNum: Table<CArray<u32, 4>> =
    Table((&raw const crate::data::trainer_hill::sNextFloorMapNum).cast());
static sPrizeListSets: Table<CArray<*mut *mut u16, 2>> =
    Table((&raw const crate::data::trainer_hill::sPrizeListSets).cast());
static sRecordWinColors: Table<CArray<u8, 3>> =
    Table((&raw const crate::data::trainer_hill::sRecordWinColors).cast());
static sTrainerClassesAndMusic: Table<CArray<sTrainerClassesAndMusic_0_t, 54>> =
    Table((&raw const crate::data::trainer_hill::sTrainerClassesAndMusic).cast());
static sTrainerObjectEventTemplate: Table<ObjectEventTemplate> =
    Table((&raw const crate::data::trainer_hill::sTrainerObjectEventTemplate).cast());
static sTrainerPartySlots: Table<CArray<CArray<u8, 3>, 2>> =
    Table((&raw const crate::data::trainer_hill::sTrainerPartySlots).cast());

pub(crate) static mut sHillData: *mut typeof___sHillData_0_t = null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sFloorTrainers: *mut FloorTrainers = null_mut();
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gTrainerHillVBlankCounter: *mut u32 = null_mut();

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

#[unsafe(no_mangle)]
pub unsafe fn CallTrainerHillFunction() {
    SetUpDataStruct();
    sHillFunctions[*(&raw const crate::ffi::gSpecialVar_0x8004)
        .cast::<u16>()
        .cast_mut()]
    .unwrap_unchecked()();
    FreeDataStruct();
}
#[unsafe(no_mangle)]
pub unsafe fn ResetTrainerHillResults() {
    (*gSaveBlock2Ptr).frontier.set_savedGame(0);
    (*gSaveBlock2Ptr).frontier.set_unk_EF9(0);
    (*gSaveBlock1Ptr).trainerHill.bestTime = 0;
    for i in 0..NUM_TRAINER_HILL_MODES {
        SetTimerValue(
            &raw mut (*gSaveBlock1Ptr).trainerHillTimes[i],
            HILL_MAX_TIME,
        );
    }
}
unsafe fn GetFloorId() -> u8 {
    gMapHeader.mapLayoutId as u8 - LAYOUT_TRAINER_HILL_1F as u8
}
pub unsafe fn GetTrainerHillOpponentClass(trainerId: u16) -> u8 {
    let id: u8 = trainerId as u8 - 1;
    (*(&raw const crate::data::pokemon::gFacilityClassToTrainerClass).cast::<CArray<u8, 0>>())
        [(*sFloorTrainers).facilityClass[id]]
}
pub unsafe fn GetTrainerHillTrainerName(dst: *mut u8, trainerId: u16) {
    let id: u8 = trainerId as u8 - 1;
    for i in 0..11i32 {
        *dst.at(i) = (*sFloorTrainers).name[id][i];
    }
}
pub unsafe fn GetTrainerHillTrainerFrontSpriteId(trainerId: u16) -> u8 {
    let mut facilityClass: u8 = 0;
    SetUpDataStruct();
    let id: u8 = trainerId as u8 - 1;
    facilityClass = (*sHillData).floors[(*sHillData).floorId].trainers[id].facilityClass;
    FreeDataStruct();
    (*(&raw const crate::data::pokemon::gFacilityClassToPicIndex).cast::<CArray<u8, 0>>())
        [facilityClass]
}
#[unsafe(no_mangle)]
pub unsafe fn InitTrainerHillBattleStruct() {
    SetUpDataStruct();
    sFloorTrainers = AllocZeroed(24) as *mut FloorTrainers;
    for i in 0..(HILL_TRAINERS_PER_FLOOR as i32) {
        for j in 0..11i32 {
            (*sFloorTrainers).name[i][j] =
                (*sHillData).floors[(*sHillData).floorId].trainers[i].name[j];
        }
        (*sFloorTrainers).facilityClass[i] =
            (*sHillData).floors[(*sHillData).floorId].trainers[i].facilityClass;
    }
    SetTrainerHillVBlankCounter(&raw mut (*gSaveBlock1Ptr).trainerHill.timer);
    FreeDataStruct();
}
#[unsafe(no_mangle)]
pub unsafe fn FreeTrainerHillBattleStruct() {
    if !sFloorTrainers.is_null() {
        Free(sFloorTrainers as *mut c_void);
        sFloorTrainers = null_mut();
    }
}
unsafe fn SetUpDataStruct() {
    if sHillData.is_null() {
        sHillData = AllocZeroed(3820) as *mut typeof___sHillData_0_t;
        (*sHillData).floorId = gMapHeader.mapLayoutId as u8 - LAYOUT_TRAINER_HILL_1F as u8;
        CpuSet(
            sChallengeData[(*gSaveBlock1Ptr).trainerHill.mode()] as *mut c_void,
            &raw mut (*sHillData).challenge as *mut c_void,
            0x40003ba,
        );
        TrainerHillDummy();
    }
}
unsafe fn FreeDataStruct() {
    if !sHillData.is_null() {
        Free(sHillData as *mut c_void);
        sHillData = null_mut();
    }
}
pub unsafe fn CopyTrainerHillTrainerText(which: u8, localId: u16) {
    SetUpDataStruct();
    let floorId: u8 = GetFloorId();
    let id: u8 = localId as u8 - 1;
    match which {
        TRAINER_HILL_TEXT_INTRO => {
            FrontierSpeechToString(
                (*sHillData).floors[floorId].trainers[id]
                    .speechBefore
                    .as_mut_ptr(),
            );
        }
        TRAINER_HILL_TEXT_PLAYER_LOST => {
            FrontierSpeechToString(
                (*sHillData).floors[floorId].trainers[id]
                    .speechWin
                    .as_mut_ptr(),
            );
        }
        TRAINER_HILL_TEXT_PLAYER_WON => {
            FrontierSpeechToString(
                (*sHillData).floors[floorId].trainers[id]
                    .speechLose
                    .as_mut_ptr(),
            );
        }
        TRAINER_HILL_TEXT_AFTER => {
            FrontierSpeechToString(
                (*sHillData).floors[floorId].trainers[id]
                    .speechAfter
                    .as_mut_ptr(),
            );
        }
        _ => {}
    }
    FreeDataStruct();
}
pub(crate) unsafe fn TrainerHillStartChallenge() {
    TrainerHillDummy();
    if ReadTrainerHillAndValidate() == 0 {
        (*gSaveBlock1Ptr).trainerHill.set_field_3D6E_0f(1);
    } else {
        (*gSaveBlock1Ptr).trainerHill.set_field_3D6E_0f(0);
    }
    (*gSaveBlock1Ptr).trainerHill.unk_3D6C = 0;
    SetTrainerHillVBlankCounter(&raw mut (*gSaveBlock1Ptr).trainerHill.timer);
    (*gSaveBlock1Ptr).trainerHill.timer = 0;
    (*gSaveBlock1Ptr).trainerHill.set_spokeToOwner(0);
    (*gSaveBlock1Ptr).trainerHill.set_checkedFinalTime(0);
    (*gSaveBlock1Ptr)
        .trainerHill
        .set_maybeECardScanDuringChallenge(0);
    (*gSaveBlock2Ptr).frontier.trainerFlags = 0;
    gBattleOutcome = 0;
    (*gSaveBlock1Ptr).trainerHill.set_receivedPrize(0);
}
pub(crate) unsafe fn GetOwnerState() {
    ClearTrainerHillVBlankCounter();
    gSpecialVar_Result = 0;
    if (*gSaveBlock1Ptr).trainerHill.spokeToOwner() != 0 {
        gSpecialVar_Result += 1;
    }
    if (*gSaveBlock1Ptr).trainerHill.receivedPrize() != 0
        && (*gSaveBlock1Ptr).trainerHill.checkedFinalTime() != 0
    {
        gSpecialVar_Result += 1;
    }
    (*gSaveBlock1Ptr).trainerHill.set_spokeToOwner(TRUE as u16);
}
pub(crate) unsafe fn GiveChallengePrize() {
    let itemId: u16 = GetPrizeItemId();
    if (*sHillData).challenge.numFloors != NUM_TRAINER_HILL_FLOORS
        || (*gSaveBlock1Ptr).trainerHill.receivedPrize() != 0
    {
        gSpecialVar_Result = 2;
    } else if AddBagItem(itemId, 1) == 1 {
        CopyItemName(itemId, gStringVar2.as_mut_ptr());
        (*gSaveBlock1Ptr).trainerHill.set_receivedPrize(TRUE as u16);
        (*gSaveBlock2Ptr).frontier.set_unk_EF9(0);
        gSpecialVar_Result = 0;
    } else {
        gSpecialVar_Result = 1;
    }
}
pub(crate) unsafe fn CheckFinalTime() {
    if (*gSaveBlock1Ptr).trainerHill.checkedFinalTime() != 0 {
        gSpecialVar_Result = 2;
    } else if GetTimerValue(&raw mut (*gSaveBlock1Ptr).trainerHill.bestTime)
        > (*gSaveBlock1Ptr).trainerHill.timer
    {
        SetTimerValue(
            &raw mut (*gSaveBlock1Ptr).trainerHill.bestTime,
            (*gSaveBlock1Ptr).trainerHill.timer,
        );
        (*gSaveBlock1Ptr).trainerHillTimes[(*gSaveBlock1Ptr).trainerHill.mode()] =
            (*gSaveBlock1Ptr).trainerHill.bestTime;
        gSpecialVar_Result = 0;
    } else {
        gSpecialVar_Result = 1;
    }
    (*gSaveBlock1Ptr)
        .trainerHill
        .set_checkedFinalTime(TRUE as u16);
}
pub(crate) unsafe fn TrainerHillResumeTimer() {
    if (*gSaveBlock1Ptr).trainerHill.spokeToOwner() == 0 {
        if (*gSaveBlock1Ptr).trainerHill.timer >= HILL_MAX_TIME {
            (*gSaveBlock1Ptr).trainerHill.timer = HILL_MAX_TIME;
        } else {
            SetTrainerHillVBlankCounter(&raw mut (*gSaveBlock1Ptr).trainerHill.timer);
        }
    }
}
pub(crate) unsafe fn TrainerHillSetPlayerLost() {
    (*gSaveBlock1Ptr).trainerHill.set_hasLost(TRUE as u16);
}
pub(crate) unsafe fn TrainerHillGetChallengeStatus() {
    if (*gSaveBlock1Ptr).trainerHill.hasLost() != 0 {
        (*gSaveBlock1Ptr).trainerHill.set_hasLost(FALSE as u16);
        gSpecialVar_Result = TRAINER_HILL_PLAYER_STATUS_LOST;
    } else if (*gSaveBlock1Ptr)
        .trainerHill
        .maybeECardScanDuringChallenge()
        != 0
    {
        (*gSaveBlock1Ptr)
            .trainerHill
            .set_maybeECardScanDuringChallenge(0);
        gSpecialVar_Result = TRAINER_HILL_PLAYER_STATUS_ECARD_SCANNED;
    } else {
        gSpecialVar_Result = TRAINER_HILL_PLAYER_STATUS_NORMAL;
    }
}
pub(crate) unsafe fn BufferChallengeTime() {
    let mut total: i32 = (*gSaveBlock1Ptr).trainerHill.timer as i32;
    if total >= HILL_MAX_TIME as i32 {
        total = HILL_MAX_TIME as i32;
    }
    let minutes: i32 = total / 3600;
    total %= 3600;
    let secondsWhole: i32 = total / 60;
    total %= 60;
    let secondsFraction: i32 = total * 168 / 100;
    ConvertIntToDecimalStringN(
        gStringVar1.as_mut_ptr(),
        minutes,
        STR_CONV_MODE_RIGHT_ALIGN,
        2,
    );
    ConvertIntToDecimalStringN(
        gStringVar2.as_mut_ptr(),
        secondsWhole,
        STR_CONV_MODE_RIGHT_ALIGN,
        2,
    );
    ConvertIntToDecimalStringN(
        gStringVar3.as_mut_ptr(),
        secondsFraction,
        STR_CONV_MODE_LEADING_ZEROS,
        2,
    );
}
pub(crate) unsafe fn GetAllFloorsUsed() {
    SetUpDataStruct();
    if (*sHillData).challenge.numFloors != NUM_TRAINER_HILL_FLOORS {
        ConvertIntToDecimalStringN(
            gStringVar1.as_mut_ptr(),
            (*sHillData).challenge.numFloors as i32,
            STR_CONV_MODE_LEFT_ALIGN,
            1,
        );
        gSpecialVar_Result = FALSE as u16;
    } else {
        gSpecialVar_Result = TRUE as u16;
    }
    FreeDataStruct();
}
pub(crate) unsafe fn GetInEReaderMode() {
    SetUpDataStruct();
    gSpecialVar_Result = FALSE as u16;
    FreeDataStruct();
}
#[unsafe(no_mangle)]
pub unsafe fn InTrainerHillChallenge() -> u8 {
    if VarGet(VAR_TRAINER_HILL_IS_ACTIVE) == 0 {
        return FALSE;
    } else if (*gSaveBlock1Ptr).trainerHill.spokeToOwner() != 0 {
        return FALSE;
    } else if GetCurrentTrainerHillMapId() != 0 {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
pub(crate) unsafe fn IsTrainerHillChallengeActive() {
    if InTrainerHillChallenge() == 0 {
        gSpecialVar_Result = FALSE as u16;
    } else {
        gSpecialVar_Result = TRUE as u16;
    }
}
fn TrainerHillDummy_Unused() {}
fn TrainerHillDummy() {}
pub unsafe fn PrintOnTrainerHillRecordsWindow() {
    let mut total: u32 = 0;
    let mut minutes: u32 = 0;
    let mut secondsWhole: u32 = 0;
    let mut secondsFraction: u32 = 0;
    SetUpDataStruct();
    FillWindowPixelBuffer(0, 0);
    let mut x: i32 = GetStringCenterAlignXOffset(
        FONT_NORMAL as i32,
        (*(&raw const crate::data::strings::gText_TimeBoard).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        0xD0,
    );
    AddTextPrinterParameterized3(
        0,
        FONT_NORMAL,
        x as u8,
        2,
        sRecordWinColors.as_ptr().cast_mut(),
        TEXT_SKIP_DRAW as i8,
        (*(&raw const crate::data::strings::gText_TimeBoard).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
    );
    let mut y: i32 = 18;
    for i in 0..NUM_TRAINER_HILL_MODES {
        AddTextPrinterParameterized3(
            0,
            FONT_NORMAL,
            0,
            y as u8,
            sRecordWinColors.as_ptr().cast_mut(),
            TEXT_SKIP_DRAW as i8,
            sModeStrings[i],
        );
        y += 15;
        total = GetTimerValue(&raw mut (*gSaveBlock1Ptr).trainerHillTimes[i]);
        minutes = total / 3600;
        total %= 3600;
        ConvertIntToDecimalStringN(
            gStringVar1.as_mut_ptr(),
            minutes as i32,
            STR_CONV_MODE_RIGHT_ALIGN,
            2,
        );
        secondsWhole = total / 60;
        total %= 60;
        ConvertIntToDecimalStringN(
            gStringVar2.as_mut_ptr(),
            secondsWhole as i32,
            STR_CONV_MODE_RIGHT_ALIGN,
            2,
        );
        secondsFraction = total * 168 / 100;
        ConvertIntToDecimalStringN(
            gStringVar3.as_mut_ptr(),
            secondsFraction as i32,
            STR_CONV_MODE_LEADING_ZEROS,
            2,
        );
        StringExpandPlaceholders(
            StringCopy(
                gStringVar4.as_mut_ptr(),
                (*(&raw const crate::data::strings::gText_TimeCleared).cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut(),
            ),
            (*(&raw const crate::data::strings::gText_XMinYDotZSec).cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
        );
        x = GetStringRightAlignXOffset(FONT_NORMAL as i32, gStringVar4.as_mut_ptr(), 0xD0);
        AddTextPrinterParameterized3(
            0,
            FONT_NORMAL,
            x as u8,
            y as u8,
            sRecordWinColors.as_ptr().cast_mut(),
            TEXT_SKIP_DRAW as i8,
            gStringVar4.as_mut_ptr(),
        );
        y += 17;
    }
    PutWindowTilemap(0);
    CopyWindowToVram(0, COPYWIN_FULL);
    FreeDataStruct();
}
unsafe fn GetTimerValue(src: *mut u32) -> u32 {
    *src
}
unsafe fn SetTimerValue(dst: *mut u32, val: u32) {
    *dst = val;
}
pub unsafe fn LoadTrainerHillObjectEventTemplates() {
    let eventTemplates: *mut ObjectEventTemplate =
        (*gSaveBlock1Ptr).objectEventTemplates.as_mut_ptr();
    if LoadTrainerHillFloorObjectEventScripts() == 0 {
        return;
    }
    SetUpDataStruct();
    let mut i: u8 = 0;
    while i < HILL_TRAINERS_PER_FLOOR {
        (*gSaveBlock2Ptr).frontier.trainerIds[i] = 0xFFFF;
        i += 1;
    }
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
    let floorId: u8 = GetFloorId();
    for i in 0..HILL_TRAINERS_PER_FLOOR {
        *eventTemplates.at(i) = *sTrainerObjectEventTemplate;
        (*eventTemplates.at(i)).localId = i + 1;
        (*eventTemplates.at(i)).graphicsId =
            FacilityClassToGraphicsId((*sHillData).floors[floorId].trainers[i].facilityClass);
        (*eventTemplates.at(i)).x = (*sHillData).floors[floorId].map.trainerCoords[i] as i16 & 0xF;
        (*eventTemplates.at(i)).y =
            (((*sHillData).floors[floorId].map.trainerCoords[i] >> 4) as i16 & 0xF)
                + HILL_FLOOR_HEIGHT_MARGIN;
        let bits: u8 = i << 2;
        (*eventTemplates.at(i)).movementType = (shr_i32(
            (*sHillData).floors[floorId].map.trainerDirections as i32,
            bits as u32,
        ) as u8
            & 0xF)
            + MOVEMENT_TYPE_FACE_UP;
        (*eventTemplates.at(i)).trainerRange_berryTreeId = shr_i32(
            (*sHillData).floors[floorId].map.trainerRanges as i32,
            bits as u32,
        ) as u16
            & 0xF;
        (*eventTemplates.at(i)).script = (*crate::asmdata::TrainerHill_EventScript_TrainerBattle
            .cast::<CArray<u8, 0>>())
        .as_ptr()
        .cast_mut();
        (*gSaveBlock2Ptr).frontier.trainerIds[i] = i as u16 + 1;
    }
    FreeDataStruct();
}
pub unsafe fn LoadTrainerHillFloorObjectEventScripts() -> u32 {
    SetUpDataStruct();
    FreeDataStruct();
    TRUE as u32
}
unsafe fn GetMapDataForFloor(floorId: u8, x: u32, y: u32, floorWidth: u32) -> u16 {
    let impassable: u8 = shr_i32(
        (*sHillData).floors[floorId].map.collisionData[y] as i32,
        15 - x,
    ) as u8
        & 1;
    let metatileId: u16 = (*sHillData).floors[floorId].map.metatileData[floorWidth * y + x] as u16
        + NUM_METATILES_IN_PRIMARY;
    let elevation: u16 = 12288;
    (impassable as u16) << 10 & 0x0C00 | elevation | metatileId & 0x03FF
}
pub unsafe fn GenerateTrainerHillFloorLayout(mapArg: *mut u16) {
    let mut mapId: u8 = GetCurrentTrainerHillMapId();
    if mapId == TRAINER_HILL_ENTRANCE {
        InitMapFromSavedGame();
        return;
    }
    SetUpDataStruct();
    if mapId == TRAINER_HILL_ROOF {
        InitMapFromSavedGame();
        FreeDataStruct();
        return;
    }
    mapId = GetFloorId();
    let mut src: *mut u16 = (*gMapHeader.mapLayout).map;
    gBackupMapLayout.map = mapArg;
    gBackupMapLayout.width = 31;
    gBackupMapLayout.height = 35;
    let mut dst: *mut u16 = mapArg.at(224);
    let mut y: i32 = 0;
    while y < HILL_FLOOR_HEIGHT_MARGIN as i32 {
        for x in 0..HILL_FLOOR_WIDTH {
            *dst.at(x) = *src.at(x);
        }
        dst = dst.at(31);
        src = src.at(16);
        y += 1;
    }
    for y in 0..HILL_FLOOR_HEIGHT_MAIN {
        for x in 0..HILL_FLOOR_WIDTH {
            *dst.at(x) = GetMapDataForFloor(mapId, x as u32, y as u32, HILL_FLOOR_WIDTH as u32);
        }
        dst = dst.at(31);
    }
    RunOnLoadMapScript();
    FreeDataStruct();
}
pub unsafe fn InTrainerHill() -> u32 {
    let mut ret: u32 = 0;
    if gMapHeader.mapLayoutId == LAYOUT_TRAINER_HILL_1F
        || gMapHeader.mapLayoutId == LAYOUT_TRAINER_HILL_2F
        || gMapHeader.mapLayoutId == LAYOUT_TRAINER_HILL_3F
        || gMapHeader.mapLayoutId == LAYOUT_TRAINER_HILL_4F
    {
        ret = TRUE as u32;
    } else {
        ret = FALSE as u32;
    }
    ret
}
pub unsafe fn GetCurrentTrainerHillMapId() -> u8 {
    let mut mapId: u8 = 0;
    if gMapHeader.mapLayoutId == LAYOUT_TRAINER_HILL_1F {
        mapId = TRAINER_HILL_1F;
    } else if gMapHeader.mapLayoutId == LAYOUT_TRAINER_HILL_2F {
        mapId = TRAINER_HILL_2F;
    } else if gMapHeader.mapLayoutId == LAYOUT_TRAINER_HILL_3F {
        mapId = TRAINER_HILL_3F;
    } else if gMapHeader.mapLayoutId == LAYOUT_TRAINER_HILL_4F {
        mapId = TRAINER_HILL_4F;
    } else if gMapHeader.mapLayoutId == LAYOUT_TRAINER_HILL_ROOF {
        mapId = TRAINER_HILL_ROOF;
    } else if gMapHeader.mapLayoutId == LAYOUT_TRAINER_HILL_ENTRANCE {
        mapId = TRAINER_HILL_ENTRANCE;
    } else {
        mapId = 0;
    }
    mapId
}
unsafe fn OnTrainerHillRoof() -> u32 {
    let mut onRoof: u32 = 0;
    if gMapHeader.mapLayoutId == LAYOUT_TRAINER_HILL_ROOF {
        onRoof = TRUE as u32;
    } else {
        onRoof = FALSE as u32;
    }
    onRoof
}
pub unsafe fn SetWarpDestinationTrainerHill4F() -> *mut WarpEvent {
    let header: *mut MapHeader = Overworld_GetMapHeaderByGroupAndId(26, 64);
    (*(*header).events).warps.at(1)
}
pub unsafe fn SetWarpDestinationTrainerHillFinalFloor(warpEventId: u8) -> *mut WarpEvent {
    if warpEventId == 1 {
        return (*gMapHeader.events).warps.at(1);
    }
    let mut numFloors: u8 = GetNumFloorsInTrainerHillChallenge();
    if numFloors == 0 || numFloors > NUM_TRAINER_HILL_FLOORS {
        numFloors = NUM_TRAINER_HILL_FLOORS;
    }
    let header: *mut MapHeader =
        Overworld_GetMapHeaderByGroupAndId(26, sNextFloorMapNum[numFloors as i32 - 1] as u16);
    (*(*header).events).warps
}
pub unsafe fn LocalIdToHillTrainerId(localId: u8) -> u16 {
    (*gSaveBlock2Ptr).frontier.trainerIds[localId as i32 - 1]
}
pub unsafe fn GetHillTrainerFlag(objectEventId: u8) -> u8 {
    let trainerIndexStart: u32 = GetFloorId() as u32 * HILL_TRAINERS_PER_FLOOR as u32;
    let bitId: u8 = gObjectEvents[objectEventId].localId - 1 + trainerIndexStart as u8;
    (*gSaveBlock2Ptr).frontier.trainerFlags
        & (*(&raw const crate::util::gBitTable).cast::<CArray<u32, 0>>())[bitId] as u8
}
pub unsafe fn SetHillTrainerFlag() {
    let trainerIndexStart: u8 = GetFloorId() * HILL_TRAINERS_PER_FLOOR;
    let mut i: u8 = 0;
    while i < HILL_TRAINERS_PER_FLOOR {
        if (*gSaveBlock2Ptr).frontier.trainerIds[i] == gTrainerBattleOpponent_A {
            (*gSaveBlock2Ptr).frontier.trainerFlags |=
                gBitTable[trainerIndexStart as i32 + i as i32] as u8;
            break;
        }
        i += 1;
    }
    if gBattleTypeFlags & BATTLE_TYPE_TWO_OPPONENTS != 0 {
        for i in 0..HILL_TRAINERS_PER_FLOOR {
            if (*gSaveBlock2Ptr).frontier.trainerIds[i] == gTrainerBattleOpponent_B {
                (*gSaveBlock2Ptr).frontier.trainerFlags |=
                    gBitTable[trainerIndexStart as i32 + i as i32] as u8;
                break;
            }
        }
    }
}
pub unsafe fn GetTrainerHillTrainerScript() -> *mut u8 {
    (*crate::asmdata::TrainerHill_EventScript_TrainerBattle.cast::<CArray<u8, 0>>())
        .as_ptr()
        .cast_mut()
}
pub(crate) unsafe fn ShowTrainerHillPostBattleText() {
    CopyTrainerHillTrainerText(
        TRAINER_HILL_TEXT_AFTER,
        *(&raw const crate::ffi::gSpecialVar_LastTalked)
            .cast::<u16>()
            .cast_mut(),
    );
    ShowFieldMessageFromBuffer();
}
unsafe fn CreateNPCTrainerHillParty(trainerId: u16, firstMonId: u8) {
    if trainerId == 0 || trainerId > HILL_TRAINERS_PER_FLOOR as u16 {
        return;
    }
    let trId: u8 = trainerId as u8 - 1;
    SetUpDataStruct();
    let level: u8 = GetHighestLevelInPlayerParty() as u8;
    let floorId: i32 = GetFloorId() as i32;
    let mut i: i32 = firstMonId as i32;
    let mut partySlot: i32 = 0;
    while i < firstMonId as i32 + 3 {
        let id: u8 = sTrainerPartySlots[trId][partySlot];
        let mon: *mut Pokemon = &raw mut gEnemyParty[i];
        CreateBattleTowerMon(
            mon,
            &raw mut (*sHillData).floors[floorId].trainers[trId].mons[id],
        );
        SetTrainerHillMonLevel(mon, level);
        i += 1;
        partySlot += 1;
    }
    FreeDataStruct();
}
pub unsafe fn FillHillTrainerParty() {
    ZeroEnemyPartyMons();
    CreateNPCTrainerHillParty(gTrainerBattleOpponent_A, 0);
}
pub unsafe fn FillHillTrainersParties() {
    ZeroEnemyPartyMons();
    CreateNPCTrainerHillParty(gTrainerBattleOpponent_A, 0);
    CreateNPCTrainerHillParty(gTrainerBattleOpponent_B, 3);
}
pub fn GetTrainerHillAIFlags() -> u32 {
    7
}
pub unsafe fn GetTrainerEncounterMusicIdInTrainerHill(trainerId: u16) -> u8 {
    let mut facilityClass: u8 = 0;
    SetUpDataStruct();
    let trId: u8 = trainerId as u8 - 1;
    facilityClass = (*sHillData).floors[(*sHillData).floorId].trainers[trId].facilityClass;
    FreeDataStruct();
    for i in 0..54i32 {
        if sTrainerClassesAndMusic[i].trainerClass
            == (*(&raw const crate::data::pokemon::gFacilityClassToTrainerClass)
                .cast::<CArray<u8, 0>>())[facilityClass]
        {
            return sTrainerClassesAndMusic[i].musicId;
        }
    }
    0
}
unsafe fn SetTrainerHillMonLevel(mon: *mut Pokemon, mut level: u8) {
    let species: u16 = GetMonData3(mon, MON_DATA_SPECIES, null_mut()) as u16;
    let mut exp: u32 = (*(&raw const crate::data::pokemon::gExperienceTables)
        .cast::<CArray<CArray<u32, 101>, 0>>())[(*(&raw const crate::data::pokemon::gSpeciesInfo)
        .cast::<CArray<SpeciesInfo, 0>>())[species]
        .growthRate][level];
    SetMonData(mon, MON_DATA_EXP, &raw mut exp as *mut c_void);
    SetMonData(mon, MON_DATA_LEVEL, &raw mut level as *mut c_void);
    CalculateMonStats(mon);
}
pub unsafe fn GetNumFloorsInTrainerHillChallenge() -> u8 {
    SetUpDataStruct();
    let floors: u8 = (*sHillData).challenge.numFloors;
    FreeDataStruct();
    floors
}
pub(crate) unsafe fn SetAllTrainerFlags() {
    (*gSaveBlock2Ptr).frontier.trainerFlags = 0xFF;
}
pub unsafe fn TryLoadTrainerHillEReaderPalette() {
    if OnTrainerHillEReaderChallengeFloor() == TRUE as u32 {
        LoadPalette(sEReader_Pal.as_ptr().cast_mut() as *mut c_void, 112, 32);
    }
}
pub(crate) unsafe fn GetGameSaved() {
    gSpecialVar_Result = (*gSaveBlock2Ptr).frontier.savedGame() as u16;
}
pub(crate) unsafe fn SetGameSaved() {
    (*gSaveBlock2Ptr).frontier.set_savedGame(TRUE);
}
pub(crate) unsafe fn ClearGameSaved() {
    (*gSaveBlock2Ptr).frontier.set_savedGame(FALSE);
}
pub unsafe fn OnTrainerHillEReaderChallengeFloor() -> u32 {
    if InTrainerHillChallenge() == 0 || GetCurrentTrainerHillMapId() == TRAINER_HILL_ENTRANCE {
        return FALSE as u32;
    }
    GetInEReaderMode();
    if gSpecialVar_Result == FALSE as u16 {
        return FALSE as u32;
    } else {
        return TRUE as u32;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
pub(crate) unsafe fn GetChallengeWon() {
    if (*gSaveBlock1Ptr).trainerHill.hasLost() != 0 {
        gSpecialVar_Result = FALSE as u16;
    } else {
        gSpecialVar_Result = TRUE as u16;
    }
}
pub(crate) unsafe fn TrainerHillSetMode() {
    (*gSaveBlock1Ptr).trainerHill.set_mode(
        *(&raw const crate::ffi::gSpecialVar_0x8005)
            .cast::<u16>()
            .cast_mut(),
    );
    (*gSaveBlock1Ptr).trainerHill.bestTime = (*gSaveBlock1Ptr).trainerHillTimes
        [*(&raw const crate::ffi::gSpecialVar_0x8005)
            .cast::<u16>()
            .cast_mut()];
}
unsafe fn GetPrizeListId(allowTMs: u8) -> u8 {
    let mut modBy: u8 = 0;
    let mut prizeListId: u8 = 0;
    for i in 0..NUM_TRAINER_HILL_FLOORS {
        prizeListId ^= (*sHillData).floors[i].trainerNum1 & 0x1F;
        prizeListId ^= (*sHillData).floors[i].trainerNum2 & 0x1F;
    }
    if allowTMs != 0 {
        modBy = NUM_TRAINER_HILL_PRIZE_LISTS;
    } else {
        modBy = 5;
    }
    prizeListId = rem_i32(prizeListId as i32, modBy as i32) as u8;
    prizeListId
}
pub(crate) unsafe fn GetPrizeItemId() -> u16 {
    let mut trainerNumSum: i32 = 0;
    let mut id: i32 = 0;
    let mut i: u8 = 0;
    while i < NUM_TRAINER_HILL_FLOORS {
        trainerNumSum += (*sHillData).floors[i].trainerNum1 as i32;
        trainerNumSum += (*sHillData).floors[i].trainerNum2 as i32;
        i += 1;
    }
    let mut prizeListSetId: i32 = trainerNumSum / 256;
    prizeListSetId %= 2;
    if FlagGet(FLAG_SYS_GAME_CLEAR) != 0
        && (*sHillData).challenge.numTrainers == NUM_TRAINER_HILL_TRAINERS as u8
    {
        i = GetPrizeListId(TRUE);
    } else {
        i = GetPrizeListId(FALSE);
    }
    if (*gSaveBlock1Ptr).trainerHill.mode() == HILL_MODE_EXPERT {
        i = ((i as i32 + 1) % 10) as u8;
    }
    let prizeList: *mut u16 = *sPrizeListSets[prizeListSetId].at(i);
    let minutes: i32 = (*gSaveBlock1Ptr).trainerHill.timer as i32 / 3600;
    if minutes < 12 {
        id = 0;
    } else if minutes < 13 {
        id = 1;
    } else if minutes < 14 {
        id = 2;
    } else if minutes < 16 {
        id = 3;
    } else if minutes < 18 {
        id = 4;
    } else {
        id = 5;
    }
    *prizeList.at(id)
}
