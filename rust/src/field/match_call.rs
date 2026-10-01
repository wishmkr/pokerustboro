//! Translated from `src/match_call.c` by tools/rustport/c2rs.py.
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
    clippy::missing_transmute_annotations,
    clippy::type_complexity,
    dead_code,
    unused_assignments,
    unused_variables
)]

use crate::agb_main::gMain;
use crate::battle_setup::{
    GetLastBeatenRematchTrainerId, HasTrainerBeenFought, UpdateRematchIfDefeated,
};
use crate::bg::LoadBgTiles;
use crate::bg::{
    ChangeBgY, CopyBgTilemapBufferToVram, FillBgTilemapBufferRect_Palette0, GetBgAttribute,
    IsDma3ManagerBusyWithBgCopy, WriteSequenceToBgTilemapBuffer,
};
#[allow(unused_imports)]
use crate::c::*;
#[allow(unused_imports)]
use crate::consts::*;
use crate::event_data::{FlagGet, IsNationalPokedexEnabled};
use crate::event_object_movement::{
    FreezeObjectEvents, GetObjectEventIdByLocalIdAndMap, ObjectEventClearHeldMovementIfFinished,
    UnfreezeObjectEvents,
};
use crate::field_player_avatar::{PlayerFreeze, StopPlayerAvatar, gObjectEvents};
use crate::fieldmap::gMapHeader;
use crate::load_save::{gSaveBlock1Ptr, gSaveBlock2Ptr};
use crate::menu::{
    DecompressAndCopyTileDataToVram, FreeTempTileDataBuffersIfPossible, GetPlayerTextSpeedDelay,
    LoadMessageBoxAndBorderGfx,
};
use crate::new_game::GetTrainerId;
use crate::overworld::{
    GetGameStat, Overworld_GetMapHeaderByGroupAndId, Overworld_MapTypeAllowsTeleportAndFly,
};
use crate::palette::LoadPalette;
use crate::pokedex::{GetHoennPokedexCount, GetNationalPokedexCount, GetSetPokedexFlag};
use crate::pokemon::{GetMonAbility, GetMonData2, SpeciesToNationalPokedexNum, gPlayerParty};
use crate::random::Random;
use crate::region_map::GetMapName;
use crate::rtc::{RtcCalcLocalTime, RtcGetLocalDayCount, gLocalTime};
use crate::script::{LockPlayerFieldControls, UnlockPlayerFieldControls};
use crate::script_movement::ScriptMovement_UnfreezeObjectEvents;
use crate::sound::{IsSEPlaying, PlaySE};
use crate::sprite::SpriteCallbackDummy;
use crate::string_util::{ConvertIntToDecimalStringN, StringCopy, StringExpandPlaceholders};
use crate::string_util::{gStringVar1, gStringVar2, gStringVar4};
use crate::task::DestroyTask;
use crate::task::gTasks;
use crate::text::{IsTextPrinterActive, RunTextPrinters};
#[allow(unused_imports)]
use crate::types::*;
use crate::window::{
    CopyWindowToVram, FillWindowPixelBuffer, GetWindowAttribute, PutWindowTilemap, RemoveWindow,
};
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
/// `FuncIsActiveTask` with this module's view of its types.
#[inline]
unsafe fn FuncIsActiveTask(a0: Option<unsafe fn(u8)>) -> u8 {
    unsafe { crate::task::FuncIsActiveTask(core::mem::transmute(a0)) }
}
// Data tables (translate with cdata.py): sMatchCallTrainers sMatchCallWildBattleTexts sMatchCallNegativeBattleTexts sMatchCallPositiveBattleTexts sMatchCallSameRouteBattleRequestTexts sMatchCallDifferentRouteBattleRequestTexts sMatchCallPersonalizedTexts sMatchCallBattleFrontierStreakTexts sMatchCallBattleFrontierRecordStreakTexts sMatchCallBattleDomeTexts sMatchCallBattlePikeTexts sMatchCallBattlePyramidTexts sMatchCallBattleTopics sMatchCallBattleRequestTopics sMatchCallGeneralTopics sMatchCallWindow_Pal sMatchCallWindow_Gfx sPokenavIcon_Pal sPokenavIcon_Gfx sText_PokenavCallEllipsis sMatchCallTaskFuncs sMatchCallTextWindow sMatchCallTextStringVars sPopulateMatchCallStringVarFuncs sMultiTrainerMatchCallTexts sBattleFrontierFacilityNames sBadgeFlags sBirchDexRatingTexts

/// `struct MatchCallState`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MatchCallState {
    pub minutes: u32,
    pub trainerId: u16,
    pub stepCounter: u8,
    pub triggeredFromScript: u8,
}

unsafe impl Sync for MatchCallState {}

/// `struct BattleFrontierStreakInfo`
#[repr(C, align(4))]
#[derive(Clone, Copy)]
pub struct BattleFrontierStreakInfo {
    pub facilityId: u16,
    pub streak: u16,
}

unsafe impl Sync for BattleFrontierStreakInfo {}

/// `struct MatchCallText`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MatchCallText {
    pub text: *mut u8,
    pub stringVarFuncIds: CArray<i8, 3>,
}

unsafe impl Sync for MatchCallText {}

/// `struct MatchCallTrainerTextInfo`
#[repr(C, align(4))]
#[derive(Clone, Copy)]
pub struct MatchCallTrainerTextInfo {
    pub trainerId: u16,
    pub unused: u16,
    pub battleTopicTextIds: CArray<u16, 3>,
    pub generalTextId: u16,
    pub battleFrontierRecordStreakTextIndex: u8,
    pub sameRouteMatchCallTextId: u16,
    pub differentRouteMatchCallTextId: u16,
}

unsafe impl Sync for MatchCallTrainerTextInfo {}

/// `struct MultiTrainerMatchCallText`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MultiTrainerMatchCallText {
    pub trainerId: u16,
    pub text: *mut u8,
}

unsafe impl Sync for MultiTrainerMatchCallText {}

#[cfg(target_arch = "arm")]
const _: () = {
    #[allow(unused_imports)]
    use core::mem::{offset_of, size_of};
    assert!(size_of::<MatchCallState>() == 8);
    assert!(offset_of!(MatchCallState, minutes) == 0);
    assert!(offset_of!(MatchCallState, trainerId) == 4);
    assert!(offset_of!(MatchCallState, stepCounter) == 6);
    assert!(offset_of!(MatchCallState, triggeredFromScript) == 7);
    assert!(size_of::<BattleFrontierStreakInfo>() == 4);
    assert!(offset_of!(BattleFrontierStreakInfo, facilityId) == 0);
    assert!(offset_of!(BattleFrontierStreakInfo, streak) == 2);
    assert!(size_of::<MatchCallText>() == 8);
    assert!(offset_of!(MatchCallText, text) == 0);
    assert!(offset_of!(MatchCallText, stringVarFuncIds) == 4);
    assert!(size_of::<MatchCallTrainerTextInfo>() == 20);
    assert!(offset_of!(MatchCallTrainerTextInfo, trainerId) == 0);
    assert!(offset_of!(MatchCallTrainerTextInfo, unused) == 2);
    assert!(offset_of!(MatchCallTrainerTextInfo, battleTopicTextIds) == 4);
    assert!(offset_of!(MatchCallTrainerTextInfo, generalTextId) == 10);
    assert!(
        offset_of!(
            MatchCallTrainerTextInfo,
            battleFrontierRecordStreakTextIndex
        ) == 12
    );
    assert!(offset_of!(MatchCallTrainerTextInfo, sameRouteMatchCallTextId) == 14);
    assert!(offset_of!(MatchCallTrainerTextInfo, differentRouteMatchCallTextId) == 16);
    assert!(size_of::<MultiTrainerMatchCallText>() == 8);
    assert!(offset_of!(MultiTrainerMatchCallText, trainerId) == 0);
    assert!(offset_of!(MultiTrainerMatchCallText, text) == 4);
};

const GEN_TOPIC_B_DOME: i32 = 4;
const GEN_TOPIC_B_PIKE: i32 = 5;
const GEN_TOPIC_B_PYRAMID: i32 = 6;
const GEN_TOPIC_STREAK_RECORD: i32 = 3;
const MATCH_CALL_FACTORY: u16 = 5;
const MATCH_CALL_PIKE: u16 = 4;
const NUM_STRVARS_IN_MSG: i32 = 3;
const TILE_MC_WINDOW: u16 = 624;
const TILE_POKENAV_ICON: u16 = 633;

static sBadgeFlags: Table<CArray<u16, 8>> =
    Table((&raw const crate::data::match_call::sBadgeFlags).cast());
static sBattleFrontierFacilityNames: Table<CArray<*mut u8, 7>> =
    Table((&raw const crate::data::match_call::sBattleFrontierFacilityNames).cast());
static sBirchDexRatingTexts: Table<CArray<*mut u8, 21>> =
    Table((&raw const crate::data::match_call::sBirchDexRatingTexts).cast());
static sMatchCallBattleRequestTopics: Table<CArray<*mut MatchCallText, 2>> =
    Table((&raw const crate::data::match_call::sMatchCallBattleRequestTopics).cast());
static sMatchCallBattleTopics: Table<CArray<*mut MatchCallText, 3>> =
    Table((&raw const crate::data::match_call::sMatchCallBattleTopics).cast());
static sMatchCallGeneralTopics: Table<CArray<*mut MatchCallText, 6>> =
    Table((&raw const crate::data::match_call::sMatchCallGeneralTopics).cast());
static sMatchCallTaskFuncs: Table<CArray<Option<unsafe fn(u8) -> u32>, 8>> =
    Table((&raw const crate::data::match_call::sMatchCallTaskFuncs).cast());
static sMatchCallTextStringVars: Table<CArray<*mut u8, 3>> =
    Table((&raw const crate::data::match_call::sMatchCallTextStringVars).cast());
static sMatchCallTextWindow: Table<WindowTemplate> =
    Table((&raw const crate::data::match_call::sMatchCallTextWindow).cast());
static sMatchCallTrainers: Table<CArray<MatchCallTrainerTextInfo, 64>> =
    Table((&raw const crate::data::match_call::sMatchCallTrainers).cast());
static sMatchCallWindow_Gfx: Table<CArray<u8, 256>> =
    Table((&raw const crate::data::match_call::sMatchCallWindow_Gfx).cast());
static sMatchCallWindow_Pal: Table<CArray<u16, 16>> =
    Table((&raw const crate::data::match_call::sMatchCallWindow_Pal).cast());
static sMultiTrainerMatchCallTexts: Table<CArray<MultiTrainerMatchCallText, 6>> =
    Table((&raw const crate::data::match_call::sMultiTrainerMatchCallTexts).cast());
static sPokenavIcon_Gfx: Table<CArray<u32, 249>> =
    Table((&raw const crate::data::match_call::sPokenavIcon_Gfx).cast());
static sPokenavIcon_Pal: Table<CArray<u16, 16>> =
    Table((&raw const crate::data::match_call::sPokenavIcon_Pal).cast());
static sPopulateMatchCallStringVarFuncs: Table<CArray<Option<unsafe fn(i32, *mut u8)>, 6>> =
    Table((&raw const crate::data::match_call::sPopulateMatchCallStringVarFuncs).cast());
static sText_PokenavCallEllipsis: Table<CArray<u8, 8>> =
    Table((&raw const crate::data::match_call::sText_PokenavCallEllipsis).cast());

#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sMatchCallState: MatchCallState = unsafe { zeroed() };
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sBattleFrontierStreakInfo: BattleFrontierStreakInfo = unsafe { zeroed() };

/// `AddTextPrinter` with this module's view of its types.
#[inline]
unsafe fn AddTextPrinter(
    a0: *mut TextPrinterTemplate,
    a1: u8,
    a2: Option<unsafe fn(*mut TextPrinterTemplate, u16)>,
) -> u16 {
    unsafe { crate::text::AddTextPrinter(a0 as _, a1, core::mem::transmute(a2)) }
}
/// `Alloc` with this module's view of its types.
#[inline]
unsafe fn Alloc(a0: u32) -> *mut c_void {
    unsafe { crate::malloc::Alloc(a0) as *mut c_void }
}

#[unsafe(no_mangle)]
pub unsafe fn InitMatchCallCounters() {
    RtcCalcLocalTime();
    sMatchCallState.minutes = GetCurrentTotalMinutes(&raw mut gLocalTime) + 10;
    sMatchCallState.stepCounter = 0;
}
unsafe fn GetCurrentTotalMinutes(time: *mut Time) -> u32 {
    (*time).days as u32 * 24 * 60 + (*time).hours as u32 * 60 + (*time).minutes as u32
}
unsafe fn UpdateMatchCallMinutesCounter() -> u32 {
    RtcCalcLocalTime();
    let curMinutes: i32 = GetCurrentTotalMinutes(&raw mut gLocalTime) as i32;
    if sMatchCallState.minutes > curMinutes as u32
        || curMinutes as u32 - sMatchCallState.minutes > 9
    {
        sMatchCallState.minutes = curMinutes as u32;
        return TRUE as u32;
    }
    FALSE as u32
}
unsafe fn CheckMatchCallChance() -> u32 {
    let mut callChance: i32 = 1;
    if GetMonData2(&raw mut gPlayerParty[0], MON_DATA_SANITY_IS_EGG) == 0
        && GetMonAbility(&raw mut gPlayerParty[0]) == ABILITY_LIGHTNING_ROD
    {
        callChance = 2;
    }
    if Random() as i32 % 10 < callChance * 3 {
        return TRUE as u32;
    } else {
        return FALSE as u32;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
unsafe fn MapAllowsMatchCall() -> u32 {
    if Overworld_MapTypeAllowsTeleportAndFly(gMapHeader.mapType) == 0
        || gMapHeader.regionMapSectionId == MAPSEC_SAFARI_ZONE
    {
        return FALSE as u32;
    }
    if gMapHeader.regionMapSectionId == MAPSEC_SOOTOPOLIS_CITY
        && FlagGet(FLAG_HIDE_SOOTOPOLIS_CITY_RAYQUAZA) == TRUE
        && FlagGet(FLAG_NEVER_SET_0x0DC) == FALSE
    {
        return FALSE as u32;
    }
    if gMapHeader.regionMapSectionId == MAPSEC_MT_CHIMNEY
        && FlagGet(FLAG_MET_ARCHIE_METEOR_FALLS) == TRUE
        && FlagGet(FLAG_DEFEATED_EVIL_TEAM_MT_CHIMNEY) == FALSE
    {
        return FALSE as u32;
    }
    TRUE as u32
}
unsafe fn UpdateMatchCallStepCounter() -> u32 {
    if ({
        sMatchCallState.stepCounter += 1;
        sMatchCallState.stepCounter
    }) >= 10
    {
        sMatchCallState.stepCounter = 0;
        return TRUE as u32;
    } else {
        return FALSE as u32;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
unsafe fn SelectMatchCallTrainer() -> u32 {
    let numRegistered: u32 = GetNumRegisteredTrainers();
    if numRegistered == 0 {
        return FALSE as u32;
    }
    sMatchCallState.trainerId =
        GetActiveMatchCallTrainerId(rem_u32(Random() as u32, numRegistered)) as u16;
    sMatchCallState.triggeredFromScript = FALSE;
    if sMatchCallState.trainerId == REMATCH_TABLE_ENTRIES as u16 {
        return FALSE as u32;
    }
    let matchCallId: u32 = GetTrainerMatchCallId(sMatchCallState.trainerId as i32) as u32;
    if GetRematchTrainerLocation(matchCallId as i32) == gMapHeader.regionMapSectionId as u16
        && TrainerIsEligibleForRematch(matchCallId as i32) == 0
    {
        return FALSE as u32;
    }
    TRUE as u32
}
unsafe fn GetNumRegisteredTrainers() -> u32 {
    let mut count: u32 = 0;
    for i in 0..REMATCH_WALLY_VR {
        if FlagGet(TRAINER_REGISTERED_FLAGS_START + i as u16) != 0 {
            count += 1;
        }
    }
    count
}
unsafe fn GetActiveMatchCallTrainerId(mut activeMatchCallId: u32) -> u32 {
    for i in 0..REMATCH_WALLY_VR {
        if FlagGet(TRAINER_REGISTERED_FLAGS_START + i as u16) != 0 {
            if activeMatchCallId == 0 {
                return (*(&raw const crate::data::battle_setup::gRematchTable)
                    .cast::<CArray<RematchTrainer, 78>>())[i]
                    .trainerIds[0] as u32;
            }
            activeMatchCallId -= 1;
        }
    }
    REMATCH_TABLE_ENTRIES as u32
}
pub unsafe fn TryStartMatchCall() -> u32 {
    if FlagGet(FLAG_HAS_MATCH_CALL) != 0
        && UpdateMatchCallStepCounter() != 0
        && UpdateMatchCallMinutesCounter() != 0
        && CheckMatchCallChance() != 0
        && MapAllowsMatchCall() != 0
        && SelectMatchCallTrainer() != 0
    {
        StartMatchCall();
        return TRUE as u32;
    }
    FALSE as u32
}
#[unsafe(no_mangle)]
pub unsafe fn StartMatchCallFromScript(message: *mut u8) {
    sMatchCallState.triggeredFromScript = TRUE;
    StartMatchCall();
}
#[unsafe(no_mangle)]
pub unsafe fn IsMatchCallTaskActive() -> u32 {
    FuncIsActiveTask(Some(ExecuteMatchCall)) as u32
}
unsafe fn StartMatchCall() {
    if sMatchCallState.triggeredFromScript == 0 {
        LockPlayerFieldControls();
        FreezeObjectEvents();
        PlayerFreeze();
        StopPlayerAvatar();
    }
    PlaySE(SE_POKENAV_CALL);
    CreateTask(Some(ExecuteMatchCall), 1);
}
pub(crate) unsafe fn ExecuteMatchCall(taskId: u8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    if sMatchCallTaskFuncs[*data].unwrap_unchecked()(taskId) != 0 {
        *data += 1;
        *data.at(1) = 0;
        if *data as u16 > 7 {
            DestroyTask(taskId);
        }
    }
}
pub(crate) unsafe fn MatchCall_LoadGfx(taskId: u8) -> u32 {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    *data.at(2) = AddWindow((&raw const *sMatchCallTextWindow).cast_mut()) as i16;
    if *data.at(2) == WINDOW_NONE as i16 {
        DestroyTask(taskId);
        return FALSE as u32;
    }
    if LoadBgTiles(
        0,
        sMatchCallWindow_Gfx.as_ptr().cast_mut() as *mut c_void,
        256,
        TILE_MC_WINDOW,
    ) == 0xFFFF
    {
        RemoveWindow(*data.at(2) as u8);
        DestroyTask(taskId);
        return FALSE as u32;
    }
    if DecompressAndCopyTileDataToVram(
        0,
        sPokenavIcon_Gfx.as_ptr().cast_mut() as *mut c_void,
        0,
        TILE_POKENAV_ICON,
        0,
    )
    .is_null()
    {
        RemoveWindow(*data.at(2) as u8);
        DestroyTask(taskId);
        return FALSE as u32;
    }
    FillWindowPixelBuffer(*data.at(2) as u8, 136);
    LoadPalette(
        sMatchCallWindow_Pal.as_ptr().cast_mut() as *mut c_void,
        224,
        32,
    );
    LoadPalette(sPokenavIcon_Pal.as_ptr().cast_mut() as *mut c_void, 240, 32);
    ChangeBgY(0, -8192, BG_COORD_SET);
    TRUE as u32
}
pub(crate) unsafe fn MatchCall_DrawWindow(taskId: u8) -> u32 {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    if FreeTempTileDataBuffersIfPossible() != 0 {
        return FALSE as u32;
    }
    PutWindowTilemap(*data.at(2) as u8);
    DrawMatchCallTextBoxBorder_Internal(*data.at(2) as u32, TILE_MC_WINDOW as u32, 14);
    WriteSequenceToBgTilemapBuffer(0, 62073, 1, 15, 4, 4, 17, 1);
    *data.at(5) = CreateTask(Some(Task_SpinPokenavIcon), 10) as i16;
    CopyWindowToVram(*data.at(2) as u8, COPYWIN_GFX);
    CopyBgTilemapBufferToVram(0);
    TRUE as u32
}
pub(crate) unsafe fn MatchCall_ReadyIntro(taskId: u8) -> u32 {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    if IsDma3ManagerBusyWithBgCopy() == 0 {
        InitMatchCallTextPrinter(
            *data.at(2) as i32,
            sText_PokenavCallEllipsis.as_ptr().cast_mut(),
        );
        return TRUE as u32;
    }
    FALSE as u32
}
pub(crate) unsafe fn MatchCall_SlideWindowIn(taskId: u8) -> u32 {
    if ChangeBgY(0, 0x600, BG_COORD_ADD) >= 0 {
        ChangeBgY(0, 0, BG_COORD_SET);
        return TRUE as u32;
    }
    FALSE as u32
}
pub(crate) unsafe fn MatchCall_PrintIntro(taskId: u8) -> u32 {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    if RunMatchCallTextPrinter(*data.at(2) as i32) == 0 {
        FillWindowPixelBuffer(*data.at(2) as u8, 136);
        if sMatchCallState.triggeredFromScript == 0 {
            SelectMatchCallMessage(sMatchCallState.trainerId as i32, gStringVar4.as_mut_ptr());
        }
        InitMatchCallTextPrinter(*data.at(2) as i32, gStringVar4.as_mut_ptr());
        return TRUE as u32;
    }
    FALSE as u32
}
pub(crate) unsafe fn MatchCall_PrintMessage(taskId: u8) -> u32 {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    if RunMatchCallTextPrinter(*data.at(2) as i32) == 0
        && IsSEPlaying() == 0
        && gMain.newKeys as i32 & 3 != 0
    {
        FillWindowPixelBuffer(*data.at(2) as u8, 136);
        CopyWindowToVram(*data.at(2) as u8, COPYWIN_GFX);
        PlaySE(SE_POKENAV_HANG_UP);
        return TRUE as u32;
    }
    FALSE as u32
}
pub(crate) unsafe fn MatchCall_SlideWindowOut(taskId: u8) -> u32 {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    if ChangeBgY(0, 0x600, BG_COORD_SUB) <= -8192 {
        FillBgTilemapBufferRect_Palette0(0, 0, 0, 14, 30, 6);
        DestroyTask(*data.at(5) as u8);
        RemoveWindow(*data.at(2) as u8);
        CopyBgTilemapBufferToVram(0);
        return TRUE as u32;
    }
    FALSE as u32
}
pub(crate) unsafe fn MatchCall_EndCall(taskId: u8) -> u32 {
    let mut playerObjectId: u8 = 0;
    if IsDma3ManagerBusyWithBgCopy() == 0 && IsSEPlaying() == 0 {
        ChangeBgY(0, 0, BG_COORD_SET);
        if sMatchCallState.triggeredFromScript == 0 {
            LoadMessageBoxAndBorderGfx();
            playerObjectId = GetObjectEventIdByLocalIdAndMap(LOCALID_PLAYER, 0, 0);
            ObjectEventClearHeldMovementIfFinished(&raw mut gObjectEvents[playerObjectId]);
            ScriptMovement_UnfreezeObjectEvents();
            UnfreezeObjectEvents();
            UnlockPlayerFieldControls();
        }
        return TRUE as u32;
    }
    FALSE as u32
}
unsafe fn DrawMatchCallTextBoxBorder_Internal(windowId: u32, tileOffset: u32, paletteId: u32) {
    let bg: i32 = GetWindowAttribute(windowId as u8, WINDOW_BG) as i32;
    let x: i32 = GetWindowAttribute(windowId as u8, WINDOW_TILEMAP_LEFT) as i32;
    let y: i32 = GetWindowAttribute(windowId as u8, WINDOW_TILEMAP_TOP) as i32;
    let width: i32 = GetWindowAttribute(windowId as u8, WINDOW_WIDTH) as i32;
    let height: i32 = GetWindowAttribute(windowId as u8, WINDOW_HEIGHT) as i32;
    let tileNum: i32 = tileOffset as i32 + GetBgAttribute(bg as u8, BG_ATTR_BASETILE) as i32;
    FillBgTilemapBufferRect_Palette0(
        bg as u8,
        (paletteId as u16) << 12 & 0xF000 | (tileNum as u16),
        x as u8 - 1,
        y as u8 - 1,
        1,
        1,
    );
    FillBgTilemapBufferRect_Palette0(
        bg as u8,
        (paletteId as u16) << 12 & 0xF000 | (tileNum as u16 + 1),
        x as u8,
        y as u8 - 1,
        width as u8,
        1,
    );
    FillBgTilemapBufferRect_Palette0(
        bg as u8,
        (paletteId as u16) << 12 & 0xF000 | (tileNum as u16 + 2),
        x as u8 + width as u8,
        y as u8 - 1,
        1,
        1,
    );
    FillBgTilemapBufferRect_Palette0(
        bg as u8,
        (paletteId as u16) << 12 & 0xF000 | (tileNum as u16 + 3),
        x as u8 - 1,
        y as u8,
        1,
        height as u8,
    );
    FillBgTilemapBufferRect_Palette0(
        bg as u8,
        (paletteId as u16) << 12 & 0xF000 | (tileNum as u16 + 4),
        x as u8 + width as u8,
        y as u8,
        1,
        height as u8,
    );
    FillBgTilemapBufferRect_Palette0(
        bg as u8,
        (paletteId as u16) << 12 & 0xF000 | (tileNum as u16 + 5),
        x as u8 - 1,
        y as u8 + height as u8,
        1,
        1,
    );
    FillBgTilemapBufferRect_Palette0(
        bg as u8,
        (paletteId as u16) << 12 & 0xF000 | (tileNum as u16 + 6),
        x as u8,
        y as u8 + height as u8,
        width as u8,
        1,
    );
    FillBgTilemapBufferRect_Palette0(
        bg as u8,
        (paletteId as u16) << 12 & 0xF000 | (tileNum as u16 + 7),
        x as u8 + width as u8,
        y as u8 + height as u8,
        1,
        1,
    );
}
unsafe fn InitMatchCallTextPrinter(windowId: i32, str: *mut u8) {
    let mut printerTemplate: TextPrinterTemplate = zeroed();
    printerTemplate.currentChar = str;
    printerTemplate.windowId = windowId as u8;
    printerTemplate.fontId = FONT_NORMAL;
    printerTemplate.x = 32;
    printerTemplate.y = 1;
    printerTemplate.currentX = 32;
    printerTemplate.currentY = 1;
    printerTemplate.letterSpacing = 0;
    printerTemplate.lineSpacing = 0;
    printerTemplate.set_unk(0);
    printerTemplate.set_fgColor(TEXT_DYNAMIC_COLOR_1);
    printerTemplate.set_bgColor(TEXT_COLOR_BLUE);
    printerTemplate.set_shadowColor(TEXT_DYNAMIC_COLOR_5);
    (*(&raw const crate::text::gTextFlags)
        .cast::<TextFlags>()
        .cast_mut())
    .set_useAlternateDownArrow(FALSE);
    AddTextPrinter(&raw mut printerTemplate, GetPlayerTextSpeedDelay(), None);
}
unsafe fn RunMatchCallTextPrinter(windowId: i32) -> u32 {
    if gMain.heldKeys as i32 & A_BUTTON != 0 {
        (*(&raw const crate::text::gTextFlags)
            .cast::<TextFlags>()
            .cast_mut())
        .set_canABSpeedUpPrint(TRUE);
    } else {
        (*(&raw const crate::text::gTextFlags)
            .cast::<TextFlags>()
            .cast_mut())
        .set_canABSpeedUpPrint(FALSE);
    }
    RunTextPrinters();
    IsTextPrinterActive(windowId as u8) as u32
}
pub(crate) unsafe fn Task_SpinPokenavIcon(taskId: u8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    if ({
        *data += 1;
        *data
    }) > 8
    {
        *data = 0;
        if ({
            *data.at(1) += 1;
            *data.at(1)
        }) > 7
        {
            *data.at(1) = 0;
        }
        *data.at(2) = *data.at(1) * 16 + TILE_POKENAV_ICON as i16;
        WriteSequenceToBgTilemapBuffer(0, *data.at(2) as u16 | 61440, 1, 15, 4, 4, 17, 1);
        CopyBgTilemapBufferToVram(0);
    }
}
unsafe fn TrainerIsEligibleForRematch(matchCallId: i32) -> u32 {
    ((*gSaveBlock1Ptr).trainerRematches[matchCallId] > 0) as u32
}
unsafe fn GetRematchTrainerLocation(matchCallId: i32) -> u16 {
    let mapHeader: *mut MapHeader = Overworld_GetMapHeaderByGroupAndId(
        (*(&raw const crate::data::battle_setup::gRematchTable)
            .cast::<CArray<RematchTrainer, 78>>())[matchCallId]
            .mapGroup,
        (*(&raw const crate::data::battle_setup::gRematchTable)
            .cast::<CArray<RematchTrainer, 78>>())[matchCallId]
            .mapNum,
    );
    (*mapHeader).regionMapSectionId as u16
}
unsafe fn GetNumRematchTrainersFought() -> u32 {
    let mut count: u32 = 0;
    for i in 0..REMATCH_WALLY_VR {
        if HasTrainerBeenFought(
            (*(&raw const crate::data::battle_setup::gRematchTable)
                .cast::<CArray<RematchTrainer, 78>>())[i]
                .trainerIds[0],
        ) != 0
        {
            count += 1;
        }
    }
    count
}
unsafe fn GetNthRematchTrainerFought(n: i32) -> u32 {
    let mut count: u32 = 0;
    for i in 0..(REMATCH_TABLE_ENTRIES as u32) {
        if HasTrainerBeenFought(
            (*(&raw const crate::data::battle_setup::gRematchTable)
                .cast::<CArray<RematchTrainer, 78>>())[i]
                .trainerIds[0],
        ) != 0
        {
            if count == n as u32 {
                return i;
            }
            count += 1;
        }
    }
    REMATCH_TABLE_ENTRIES as u32
}
pub unsafe fn SelectMatchCallMessage(trainerId: i32, str: *mut u8) -> u32 {
    let mut matchCallText: *mut MatchCallText = null_mut();
    let mut newRematchRequest: u32 = FALSE as u32;
    let matchCallId: u32 = GetTrainerMatchCallId(trainerId) as u32;
    sBattleFrontierStreakInfo.facilityId = 0;
    if TrainerIsEligibleForRematch(matchCallId as i32) != 0
        && GetRematchTrainerLocation(matchCallId as i32) == gMapHeader.regionMapSectionId as u16
    {
        matchCallText = GetSameRouteMatchCallText(matchCallId as i32, str);
    } else if ShouldTrainerRequestBattle(matchCallId as i32) != 0 {
        matchCallText = GetDifferentRouteMatchCallText(matchCallId as i32, str);
        newRematchRequest = TRUE as u32;
        UpdateRematchIfDefeated(matchCallId as i32);
    } else if Random() as i32 % 3 != 0 {
        matchCallText = GetBattleMatchCallText(matchCallId as i32, str);
    } else {
        matchCallText = GetGeneralMatchCallText(matchCallId as i32, str);
    }
    BuildMatchCallString(matchCallId as i32, matchCallText, str);
    newRematchRequest
}
unsafe fn GetTrainerMatchCallId(trainerId: i32) -> i32 {
    let mut i: i32 = 0;
    loop {
        if sMatchCallTrainers[i].trainerId as i32 == trainerId {
            return i;
        } else {
            i += 1;
        }
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
fn GetSameRouteMatchCallText(matchCallId: i32, str: *mut u8) -> *mut MatchCallText {
    let textId: u16 = sMatchCallTrainers[matchCallId].sameRouteMatchCallTextId;
    let mask: i32 = 0xFF;
    let topic: u32 = (textId >> 8) as u32 - 1;
    let id: u32 = (textId as u32 & mask as u32) - 1;
    sMatchCallBattleRequestTopics[topic].at(id)
}
fn GetDifferentRouteMatchCallText(matchCallId: i32, str: *mut u8) -> *mut MatchCallText {
    let textId: u16 = sMatchCallTrainers[matchCallId].differentRouteMatchCallTextId;
    let mask: i32 = 0xFF;
    let topic: u32 = (textId >> 8) as u32 - 1;
    let id: u32 = (textId as u32 & mask as u32) - 1;
    sMatchCallBattleRequestTopics[topic].at(id)
}
unsafe fn GetBattleMatchCallText(matchCallId: i32, str: *mut u8) -> *mut MatchCallText {
    let mut topic: u32 = (Random() as i32 % 3) as u32;
    let textId: u32 = sMatchCallTrainers[matchCallId].battleTopicTextIds[topic] as u32;
    if textId == 0 {
        SpriteCallbackDummy(null_mut());
    }
    let mask: i32 = 0xFF;
    topic = (textId >> 8) - 1;
    let id: u32 = (textId & mask as u32) - 1;
    sMatchCallBattleTopics[topic].at(id)
}
unsafe fn GetGeneralMatchCallText(matchCallId: i32, str: *mut u8) -> *mut MatchCallText {
    let mut i: i32 = 0;
    let mut count: i32 = 0;
    let mut topic: u32 = 0;
    let mut id: u32 = 0;
    let rand: u16 = Random();
    if rand as i32 & 1 == 0 {
        count = 0;
        i = 0;
        while i < NUM_FRONTIER_FACILITIES as i32 {
            if GetFrontierStreakInfo(i as u16, &raw mut topic) > 1 {
                count += 1;
            }
            i += 1;
        }
        if count != 0 {
            count = rem_i32(Random() as i32, count);
            i = 0;
            'l3: while i < NUM_FRONTIER_FACILITIES as i32 {
                'l2: {
                    sBattleFrontierStreakInfo.streak =
                        GetFrontierStreakInfo(i as u16, &raw mut topic);
                    if sBattleFrontierStreakInfo.streak < 2 {
                        break 'l2;
                    }
                    if count == 0 {
                        break 'l3;
                    }
                    count -= 1;
                }
                i += 1;
            }
            sBattleFrontierStreakInfo.facilityId = i as u16;
            id = sMatchCallTrainers[matchCallId].battleFrontierRecordStreakTextIndex as u32 - 1;
            return sMatchCallGeneralTopics[topic].at(id);
        }
    }
    topic = (sMatchCallTrainers[matchCallId].generalTextId >> 8) as u32 - 1;
    id = (sMatchCallTrainers[matchCallId].generalTextId as u32 & 0xFF) - 1;
    sMatchCallGeneralTopics[topic].at(id)
}
unsafe fn BuildMatchCallString(matchCallId: i32, matchCallText: *mut MatchCallText, str: *mut u8) {
    PopulateMatchCallStringVars(matchCallId, (*matchCallText).stringVarFuncIds.as_mut_ptr());
    StringExpandPlaceholders(str, (*matchCallText).text);
}
unsafe fn PopulateMatchCallStringVars(matchCallId: i32, stringVarFuncIds: *mut i8) {
    for i in 0..NUM_STRVARS_IN_MSG {
        if *stringVarFuncIds.at(i) >= 0 {
            PopulateMatchCallStringVar(
                matchCallId,
                *stringVarFuncIds.at(i) as i32,
                sMatchCallTextStringVars[i],
            );
        }
    }
}
unsafe fn PopulateMatchCallStringVar(matchCallId: i32, funcId: i32, destStr: *mut u8) {
    sPopulateMatchCallStringVarFuncs[funcId].unwrap_unchecked()(matchCallId, destStr);
}
pub(crate) unsafe fn PopulateTrainerName(matchCallId: i32, destStr: *mut u8) {
    let trainerId: u16 = sMatchCallTrainers[matchCallId].trainerId;
    for i in 0..6u32 {
        if sMultiTrainerMatchCallTexts[i].trainerId == trainerId {
            StringCopy(destStr, sMultiTrainerMatchCallTexts[i].text);
            return;
        }
    }
    StringCopy(
        destStr,
        (*(&raw const crate::data::data_tables::gTrainers).cast::<CArray<Trainer, 0>>())[trainerId]
            .trainerName
            .as_ptr()
            .cast_mut(),
    );
}
pub(crate) unsafe fn PopulateMapName(matchCallId: i32, destStr: *mut u8) {
    GetMapName(destStr, GetRematchTrainerLocation(matchCallId), 0);
}
fn GetLandEncounterSlot() -> u8 {
    let rand: i32 = Random() as i32 % 100;
    if rand < ENCOUNTER_CHANCE_LAND_MONS_SLOT_0 {
        return 0;
    } else if (ENCOUNTER_CHANCE_LAND_MONS_SLOT_0..ENCOUNTER_CHANCE_LAND_MONS_SLOT_1).contains(&rand)
    {
        return 1;
    } else if (ENCOUNTER_CHANCE_LAND_MONS_SLOT_1..ENCOUNTER_CHANCE_LAND_MONS_SLOT_2).contains(&rand)
    {
        return 2;
    } else if (ENCOUNTER_CHANCE_LAND_MONS_SLOT_2..ENCOUNTER_CHANCE_LAND_MONS_SLOT_3).contains(&rand)
    {
        return 3;
    } else if (ENCOUNTER_CHANCE_LAND_MONS_SLOT_3..ENCOUNTER_CHANCE_LAND_MONS_SLOT_4).contains(&rand)
    {
        return 4;
    } else if (ENCOUNTER_CHANCE_LAND_MONS_SLOT_4..ENCOUNTER_CHANCE_LAND_MONS_SLOT_5).contains(&rand)
    {
        return 5;
    } else if (ENCOUNTER_CHANCE_LAND_MONS_SLOT_5..ENCOUNTER_CHANCE_LAND_MONS_SLOT_6).contains(&rand)
    {
        return 6;
    } else if (ENCOUNTER_CHANCE_LAND_MONS_SLOT_6..ENCOUNTER_CHANCE_LAND_MONS_SLOT_7).contains(&rand)
    {
        return 7;
    } else if (ENCOUNTER_CHANCE_LAND_MONS_SLOT_7..ENCOUNTER_CHANCE_LAND_MONS_SLOT_8).contains(&rand)
    {
        return 8;
    } else if (ENCOUNTER_CHANCE_LAND_MONS_SLOT_8..ENCOUNTER_CHANCE_LAND_MONS_SLOT_9).contains(&rand)
    {
        return 9;
    } else if (ENCOUNTER_CHANCE_LAND_MONS_SLOT_9..ENCOUNTER_CHANCE_LAND_MONS_SLOT_10)
        .contains(&rand)
    {
        return 10;
    } else {
        return 11;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
fn GetWaterEncounterSlot() -> u8 {
    let rand: i32 = Random() as i32 % 100;
    if rand < ENCOUNTER_CHANCE_WATER_MONS_SLOT_0 {
        return 0;
    } else if (ENCOUNTER_CHANCE_WATER_MONS_SLOT_0..ENCOUNTER_CHANCE_WATER_MONS_SLOT_1)
        .contains(&rand)
    {
        return 1;
    } else if (ENCOUNTER_CHANCE_WATER_MONS_SLOT_1..ENCOUNTER_CHANCE_WATER_MONS_SLOT_2)
        .contains(&rand)
    {
        return 2;
    } else if (ENCOUNTER_CHANCE_WATER_MONS_SLOT_2..ENCOUNTER_CHANCE_WATER_MONS_SLOT_3)
        .contains(&rand)
    {
        return 3;
    } else {
        return 4;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
pub(crate) unsafe fn PopulateSpeciesFromTrainerLocation(matchCallId: i32, destStr: *mut u8) {
    let mut species: CArray<u16, 2> = zeroed();
    let mut numSpecies: i32 = 0;
    let mut slot: u8 = 0;
    let mut i: i32 = 0;
    if (*(&raw const crate::data::wild_encounter::gWildMonHeaders)
        .cast::<CArray<WildPokemonHeader, 0>>())[i]
        .mapGroup
        != 255
    {
        while (*(&raw const crate::data::wild_encounter::gWildMonHeaders)
            .cast::<CArray<WildPokemonHeader, 0>>())[i]
            .mapGroup
            != 255
        {
            if (*(&raw const crate::data::wild_encounter::gWildMonHeaders)
                .cast::<CArray<WildPokemonHeader, 0>>())[i]
                .mapGroup as u16
                == (*(&raw const crate::data::battle_setup::gRematchTable)
                    .cast::<CArray<RematchTrainer, 78>>())[matchCallId]
                    .mapGroup
                && (*(&raw const crate::data::wild_encounter::gWildMonHeaders)
                    .cast::<CArray<WildPokemonHeader, 0>>())[i]
                    .mapNum as u16
                    == (*(&raw const crate::data::battle_setup::gRematchTable).cast::<CArray<
                        RematchTrainer,
                        78,
                    >>(
                    ))[matchCallId]
                        .mapNum
            {
                break;
            }
            i += 1;
        }
        if (*(&raw const crate::data::wild_encounter::gWildMonHeaders)
            .cast::<CArray<WildPokemonHeader, 0>>())[i]
            .mapGroup
            != 255
        {
            numSpecies = 0;
            if !(*(&raw const crate::data::wild_encounter::gWildMonHeaders)
                .cast::<CArray<WildPokemonHeader, 0>>())[i]
                .landMonsInfo
                .is_null()
            {
                slot = GetLandEncounterSlot();
                species[numSpecies] =
                    (*(*(*(&raw const crate::data::wild_encounter::gWildMonHeaders)
                        .cast::<CArray<WildPokemonHeader, 0>>())[i]
                        .landMonsInfo)
                        .wildPokemon
                        .at(slot))
                    .species;
                numSpecies += 1;
            }
            if !(*(&raw const crate::data::wild_encounter::gWildMonHeaders)
                .cast::<CArray<WildPokemonHeader, 0>>())[i]
                .waterMonsInfo
                .is_null()
            {
                slot = GetWaterEncounterSlot();
                species[numSpecies] =
                    (*(*(*(&raw const crate::data::wild_encounter::gWildMonHeaders)
                        .cast::<CArray<WildPokemonHeader, 0>>())[i]
                        .waterMonsInfo)
                        .wildPokemon
                        .at(slot))
                    .species;
                numSpecies += 1;
            }
            if numSpecies != 0 {
                StringCopy(
                    destStr,
                    (*(&raw const crate::data::data_tables::gSpeciesNames)
                        .cast::<CArray<CArray<u8, 11>, 0>>())
                        [species[rem_i32(Random() as i32, numSpecies)]]
                    .as_ptr()
                    .cast_mut(),
                );
                return;
            }
        }
    }
    *destStr = EOS;
}
pub(crate) unsafe fn PopulateSpeciesFromTrainerParty(matchCallId: i32, destStr: *mut u8) {
    let mut trainerId: u16 = 0;
    let mut party: TrainerMonPtr = zeroed();
    let mut speciesName: *mut u8 = null_mut();
    trainerId = GetLastBeatenRematchTrainerId(sMatchCallTrainers[matchCallId].trainerId);
    party = (*(&raw const crate::data::data_tables::gTrainers).cast::<CArray<Trainer, 0>>())
        [trainerId]
        .party;
    let monId: u8 = rem_i32(
        Random() as i32,
        (*(&raw const crate::data::data_tables::gTrainers).cast::<CArray<Trainer, 0>>())[trainerId]
            .partySize as i32,
    ) as u8;
    match (*(&raw const crate::data::data_tables::gTrainers).cast::<CArray<Trainer, 0>>())
        [trainerId]
        .partyFlags
    {
        F_TRAINER_PARTY_CUSTOM_MOVESET => {
            speciesName = (*(&raw const crate::data::data_tables::gSpeciesNames)
                .cast::<CArray<CArray<u8, 11>, 0>>())[(*party.NoItemCustomMoves.at(monId)).species]
                .as_ptr()
                .cast_mut();
        }
        F_TRAINER_PARTY_HELD_ITEM => {
            speciesName = (*(&raw const crate::data::data_tables::gSpeciesNames)
                .cast::<CArray<CArray<u8, 11>, 0>>())[(*party.ItemDefaultMoves.at(monId)).species]
                .as_ptr()
                .cast_mut();
        }
        3 => {
            speciesName = (*(&raw const crate::data::data_tables::gSpeciesNames)
                .cast::<CArray<CArray<u8, 11>, 0>>())[(*party.ItemCustomMoves.at(monId)).species]
                .as_ptr()
                .cast_mut();
        }
        _ => {
            speciesName = (*(&raw const crate::data::data_tables::gSpeciesNames)
                .cast::<CArray<CArray<u8, 11>, 0>>())
                [(*party.NoItemDefaultMoves.at(monId)).species]
                .as_ptr()
                .cast_mut();
        }
    }
    StringCopy(destStr, speciesName);
}
pub(crate) unsafe fn PopulateBattleFrontierFacilityName(matchCallId: i32, destStr: *mut u8) {
    StringCopy(
        destStr,
        sBattleFrontierFacilityNames[sBattleFrontierStreakInfo.facilityId],
    );
}
pub(crate) unsafe fn PopulateBattleFrontierStreak(matchCallId: i32, destStr: *mut u8) {
    let mut i: i32 = 0;
    let mut streak: i32 = sBattleFrontierStreakInfo.streak as i32;
    while streak != 0 {
        streak /= 10;
        i += 1;
    }
    ConvertIntToDecimalStringN(
        destStr,
        sBattleFrontierStreakInfo.streak as i32,
        STR_CONV_MODE_LEFT_ALIGN,
        i as u8,
    );
}
unsafe fn GetNumOwnedBadges() -> i32 {
    let mut i: u32 = 0;
    while i < NUM_BADGES {
        if FlagGet(sBadgeFlags[i]) == 0 {
            break;
        }
        i += 1;
    }
    i as i32
}
unsafe fn ShouldTrainerRequestBattle(matchCallId: i32) -> u32 {
    let mut rand: i32 = 0;
    if GetNumOwnedBadges() < 5 {
        return FALSE as u32;
    }
    let dayCount: i32 = RtcGetLocalDayCount() as i32;
    let otId: i32 = GetTrainerId((*gSaveBlock2Ptr).playerTrainerId.as_mut_ptr()) as i32 & 0xFFFF;
    let dewfordRand: u16 = (*gSaveBlock1Ptr).dewfordTrends[0].rand;
    let numRematchTrainersFought: i32 = GetNumRematchTrainersFought() as i32;
    let max: i32 = numRematchTrainersFought * 13 / 10;
    rand = ((dayCount ^ dewfordRand as i32)
        + (dewfordRand as i32 ^ GetGameStat(GAME_STAT_TRAINER_BATTLES) as i32))
        ^ otId;
    let n: i32 = rem_i32(rand, max);
    if n < numRematchTrainersFought && GetNthRematchTrainerFought(n) == matchCallId as u32 {
        return TRUE as u32;
    }
    FALSE as u32
}
unsafe fn GetFrontierStreakInfo(facilityId: u16, topicTextId: *mut u32) -> u16 {
    let mut streak: u16 = 0;
    match facilityId {
        1 => {
            for i in 0..2i32 {
                for j in 0..FRONTIER_LVL_MODE_COUNT {
                    if streak < (*gSaveBlock2Ptr).frontier.domeRecordWinStreaks[i][j] {
                        streak = (*gSaveBlock2Ptr).frontier.domeRecordWinStreaks[i][j];
                    }
                }
            }
            *topicTextId = 3;
        }
        MATCH_CALL_PIKE => {
            for i in 0..FRONTIER_LVL_MODE_COUNT {
                if streak < (*gSaveBlock2Ptr).frontier.pikeRecordStreaks[i] {
                    streak = (*gSaveBlock2Ptr).frontier.pikeRecordStreaks[i];
                }
            }
            *topicTextId = 4;
        }
        FRONTIER_FACILITY_TOWER => {
            for i in 0..4i32 {
                for j in 0..FRONTIER_LVL_MODE_COUNT {
                    if streak < (*gSaveBlock2Ptr).frontier.towerRecordWinStreaks[i][j] {
                        streak = (*gSaveBlock2Ptr).frontier.towerRecordWinStreaks[i][j];
                    }
                }
            }
            *topicTextId = 2;
        }
        2 => {
            for i in 0..2i32 {
                for j in 0..FRONTIER_LVL_MODE_COUNT {
                    if streak < (*gSaveBlock2Ptr).frontier.palaceRecordWinStreaks[i][j] {
                        streak = (*gSaveBlock2Ptr).frontier.palaceRecordWinStreaks[i][j];
                    }
                }
            }
            *topicTextId = 2;
        }
        MATCH_CALL_FACTORY => {
            for i in 0..2i32 {
                for j in 0..FRONTIER_LVL_MODE_COUNT {
                    if streak < (*gSaveBlock2Ptr).frontier.factoryRecordWinStreaks[i][j] {
                        streak = (*gSaveBlock2Ptr).frontier.factoryRecordWinStreaks[i][j];
                    }
                }
            }
            *topicTextId = 2;
        }
        3 => {
            for i in 0..FRONTIER_LVL_MODE_COUNT {
                if streak < (*gSaveBlock2Ptr).frontier.arenaRecordStreaks[i] {
                    streak = (*gSaveBlock2Ptr).frontier.arenaRecordStreaks[i];
                }
            }
            *topicTextId = 2;
        }
        6 => {
            for i in 0..FRONTIER_LVL_MODE_COUNT {
                if streak < (*gSaveBlock2Ptr).frontier.pyramidRecordStreaks[i] {
                    streak = (*gSaveBlock2Ptr).frontier.pyramidRecordStreaks[i];
                }
            }
            *topicTextId = 5;
        }
        _ => {}
    }
    streak
}
unsafe fn GetPokedexRatingLevel(mut numSeen: u16) -> u8 {
    if numSeen < 10 {
        return 0;
    }
    if numSeen < 20 {
        return 1;
    }
    if numSeen < 30 {
        return 2;
    }
    if numSeen < 40 {
        return 3;
    }
    if numSeen < 50 {
        return 4;
    }
    if numSeen < 60 {
        return 5;
    }
    if numSeen < 70 {
        return 6;
    }
    if numSeen < 80 {
        return 7;
    }
    if numSeen < 90 {
        return 8;
    }
    if numSeen < 100 {
        return 9;
    }
    if numSeen < 110 {
        return 10;
    }
    if numSeen < 120 {
        return 11;
    }
    if numSeen < 130 {
        return 12;
    }
    if numSeen < 140 {
        return 13;
    }
    if numSeen < 150 {
        return 14;
    }
    if numSeen < 160 {
        return 15;
    }
    if numSeen < 170 {
        return 16;
    }
    if numSeen < 180 {
        return 17;
    }
    if numSeen < 190 {
        return 18;
    }
    if numSeen < 200 {
        return 19;
    }
    if GetSetPokedexFlag(
        SpeciesToNationalPokedexNum(SPECIES_DEOXYS as u16),
        FLAG_GET_CAUGHT,
    ) != 0
    {
        numSeen -= 1;
    }
    if GetSetPokedexFlag(
        SpeciesToNationalPokedexNum(SPECIES_JIRACHI),
        FLAG_GET_CAUGHT,
    ) != 0
    {
        numSeen -= 1;
    }
    if numSeen < 200 {
        return 19;
    } else {
        return 20;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
pub unsafe fn BufferPokedexRatingForMatchCall(destStr: *mut u8) {
    let buffer: *mut u8 = Alloc(1000) as *mut u8;
    if buffer.is_null() {
        *destStr = EOS;
        return;
    }
    let mut numSeen: i32 = GetHoennPokedexCount(FLAG_GET_SEEN) as i32;
    let mut numCaught: i32 = GetHoennPokedexCount(FLAG_GET_CAUGHT) as i32;
    ConvertIntToDecimalStringN(
        gStringVar1.as_mut_ptr(),
        numSeen,
        STR_CONV_MODE_LEFT_ALIGN,
        3,
    );
    ConvertIntToDecimalStringN(
        gStringVar2.as_mut_ptr(),
        numCaught,
        STR_CONV_MODE_LEFT_ALIGN,
        3,
    );
    let dexRatingLevel: u8 = GetPokedexRatingLevel(numCaught as u16);
    let mut str: *mut u8 = StringCopy(
        buffer,
        (*crate::asmdata::gBirchDexRatingText_AreYouCurious.cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
    );
    *({
        let t1 = str;
        str = str.at(1);
        t1
    }) = CHAR_PROMPT_CLEAR;
    str = StringCopy(
        str,
        (*crate::asmdata::gBirchDexRatingText_SoYouveSeenAndCaught.cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
    );
    *({
        let t2 = str;
        str = str.at(1);
        t2
    }) = CHAR_PROMPT_CLEAR;
    StringCopy(str, sBirchDexRatingTexts[dexRatingLevel]);
    str = StringExpandPlaceholders(destStr, buffer);
    if IsNationalPokedexEnabled() != 0 {
        *({
            let t3 = str;
            str = str.at(1);
            t3
        }) = CHAR_PROMPT_CLEAR;
        numSeen = GetNationalPokedexCount(FLAG_GET_SEEN) as i32;
        numCaught = GetNationalPokedexCount(FLAG_GET_CAUGHT) as i32;
        ConvertIntToDecimalStringN(
            gStringVar1.as_mut_ptr(),
            numSeen,
            STR_CONV_MODE_LEFT_ALIGN,
            3,
        );
        ConvertIntToDecimalStringN(
            gStringVar2.as_mut_ptr(),
            numCaught,
            STR_CONV_MODE_LEFT_ALIGN,
            3,
        );
        StringExpandPlaceholders(
            str,
            (*crate::asmdata::gBirchDexRatingText_OnANationwideBasis.cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
        );
    }
    Free(buffer as *mut c_void);
}
pub unsafe fn LoadMatchCallWindowGfx(windowId: u32, destOffset: u32, paletteId: u32) {
    let bg: u8 = GetWindowAttribute(windowId as u8, WINDOW_BG) as u8;
    LoadBgTiles(
        bg,
        sMatchCallWindow_Gfx.as_ptr().cast_mut() as *mut c_void,
        0x100,
        destOffset as u16,
    );
    LoadPalette(
        sMatchCallWindow_Pal.as_ptr().cast_mut() as *mut c_void,
        paletteId as u16 * 16,
        32,
    );
}
pub unsafe fn DrawMatchCallTextBoxBorder(windowId: u32, tileOffset: u32, paletteId: u32) {
    DrawMatchCallTextBoxBorder_Internal(windowId, tileOffset, paletteId);
}
