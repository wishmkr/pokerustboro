//! Translated from `src/match_call.c` by tools/rustport/c2rs.py.
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
static sMatchCallTaskFuncs: Table<CArray<Option<unsafe extern "C" fn(u8) -> u32>, 8>> =
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
static sPopulateMatchCallStringVarFuncs: Table<
    CArray<Option<unsafe extern "C" fn(i32, *mut u8)>, 6>,
> = Table((&raw const crate::data::match_call::sPopulateMatchCallStringVarFuncs).cast());
static sText_PokenavCallEllipsis: Table<CArray<u8, 8>> =
    Table((&raw const crate::data::match_call::sText_PokenavCallEllipsis).cast());

#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sMatchCallState: MatchCallState = unsafe { zeroed() };
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sBattleFrontierStreakInfo: BattleFrontierStreakInfo = unsafe { zeroed() };

unsafe extern "C" {
    static gBirchDexRatingText_AreYouCurious: CArray<u8, 0>;
    static gBirchDexRatingText_OnANationwideBasis: CArray<u8, 0>;
    static gBirchDexRatingText_SoYouveSeenAndCaught: CArray<u8, 0>;
    static mut gLocalTime: Time;
    static mut gMain: Main;
    static mut gMapHeader: MapHeader;
    static mut gObjectEvents: CArray<ObjectEvent, 16>;
    static mut gPlayerParty: CArray<Pokemon, 6>;
    static gRematchTable: CArray<RematchTrainer, 78>;
    static mut gSaveBlock1Ptr: *mut SaveBlock1;
    static mut gSaveBlock2Ptr: *mut SaveBlock2;
    static gSpeciesNames: CArray<CArray<u8, 11>, 0>;
    static mut gStringVar1: CArray<u8, 256>;
    static mut gStringVar2: CArray<u8, 256>;
    static mut gStringVar4: CArray<u8, 1000>;
    static mut gTasks: CArray<Task, 0>;
    static mut gTextFlags: TextFlags;
    static gTrainers: CArray<Trainer, 0>;
    static gWildMonHeaders: CArray<WildPokemonHeader, 0>;
    fn AddTextPrinter(
        a0: *mut TextPrinterTemplate,
        a1: u8,
        a2: Option<unsafe extern "C" fn(*mut TextPrinterTemplate, u16)>,
    ) -> u16;
    fn AddWindow(a0: *mut WindowTemplate) -> u16;
    fn Alloc(a0: u32) -> *mut c_void;
    fn ChangeBgY(a0: u8, a1: i32, a2: u8) -> i32;
    fn ConvertIntToDecimalStringN(a0: *mut u8, a1: i32, a2: i32, a3: u8) -> *mut u8;
    fn CopyBgTilemapBufferToVram(a0: u8);
    fn CopyWindowToVram(a0: u8, a1: u8);
    fn CreateTask(a0: Option<unsafe extern "C" fn(u8)>, a1: u8) -> u8;
    fn DecompressAndCopyTileDataToVram(
        a0: u8,
        a1: *mut c_void,
        a2: u32,
        a3: u16,
        a4: u8,
    ) -> *mut c_void;
    fn DestroyTask(a0: u8);
    fn FillBgTilemapBufferRect_Palette0(a0: u8, a1: u16, a2: u8, a3: u8, a4: u8, a5: u8);
    fn FillWindowPixelBuffer(a0: u8, a1: u8);
    fn FlagGet(a0: u16) -> u8;
    fn Free(a0: *mut c_void);
    fn FreeTempTileDataBuffersIfPossible() -> u8;
    fn FreezeObjectEvents();
    fn FuncIsActiveTask(a0: Option<unsafe extern "C" fn(u8)>) -> u8;
    fn GetBgAttribute(a0: u8, a1: u8) -> u16;
    fn GetGameStat(a0: u8) -> u32;
    fn GetHoennPokedexCount(a0: u8) -> u16;
    fn GetLastBeatenRematchTrainerId(a0: u16) -> u16;
    fn GetMapName(a0: *mut u8, a1: u16, a2: u16) -> *mut u8;
    fn GetMonAbility(a0: *mut Pokemon) -> u8;
    fn GetMonData2(a0: *mut Pokemon, a1: i32) -> u32;
    fn GetNationalPokedexCount(a0: u8) -> u16;
    fn GetObjectEventIdByLocalIdAndMap(a0: u8, a1: u8, a2: u8) -> u8;
    fn GetPlayerTextSpeedDelay() -> u8;
    fn GetSetPokedexFlag(a0: u16, a1: u8) -> i8;
    fn GetTrainerId(a0: *mut u8) -> u32;
    fn GetWindowAttribute(a0: u8, a1: u8) -> u32;
    fn HasTrainerBeenFought(a0: u16) -> u8;
    fn IsDma3ManagerBusyWithBgCopy() -> u8;
    fn IsNationalPokedexEnabled() -> u32;
    fn IsSEPlaying() -> u8;
    fn IsTextPrinterActive(a0: u8) -> u16;
    fn LoadBgTiles(a0: u8, a1: *mut c_void, a2: u16, a3: u16) -> u16;
    fn LoadMessageBoxAndBorderGfx();
    fn LoadPalette(a0: *mut c_void, a1: u16, a2: u16);
    fn LockPlayerFieldControls();
    fn ObjectEventClearHeldMovementIfFinished(a0: *mut ObjectEvent) -> u8;
    fn Overworld_GetMapHeaderByGroupAndId(a0: u16, a1: u16) -> *mut MapHeader;
    fn Overworld_MapTypeAllowsTeleportAndFly(a0: u8) -> u8;
    fn PlaySE(a0: u16);
    fn PlayerFreeze();
    fn PutWindowTilemap(a0: u8);
    fn Random() -> u16;
    fn RemoveWindow(a0: u8);
    fn RtcCalcLocalTime();
    fn RtcGetLocalDayCount() -> u32;
    fn RunTextPrinters();
    fn ScriptMovement_UnfreezeObjectEvents();
    fn SpeciesToNationalPokedexNum(a0: u16) -> u16;
    fn SpriteCallbackDummy(a0: *mut Sprite);
    fn StopPlayerAvatar();
    fn StringCopy(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn StringExpandPlaceholders(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn UnfreezeObjectEvents();
    fn UnlockPlayerFieldControls();
    fn UpdateRematchIfDefeated(a0: i32);
    fn WriteSequenceToBgTilemapBuffer(
        a0: u8,
        a1: u16,
        a2: u8,
        a3: u8,
        a4: u8,
        a5: u8,
        a6: u8,
        a7: i16,
    );
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn InitMatchCallCounters() {
    RtcCalcLocalTime();
    sMatchCallState.minutes = GetCurrentTotalMinutes(&raw mut gLocalTime) + 10;
    sMatchCallState.stepCounter = 0;
}
pub(crate) unsafe extern "C" fn GetCurrentTotalMinutes(time: *mut Time) -> u32 {
    return (*time).days as u32 * 24 * 60 + (*time).hours as u32 * 60 + (*time).minutes as u32;
}
pub(crate) unsafe extern "C" fn UpdateMatchCallMinutesCounter() -> u32 {
    let mut curMinutes: i32 = 0;
    RtcCalcLocalTime();
    curMinutes = GetCurrentTotalMinutes(&raw mut gLocalTime) as i32;
    if sMatchCallState.minutes > curMinutes as u32
        || curMinutes as u32 - sMatchCallState.minutes > 9
    {
        sMatchCallState.minutes = curMinutes as u32;
        return TRUE as u32;
    }
    return FALSE as u32;
}
pub(crate) unsafe extern "C" fn CheckMatchCallChance() -> u32 {
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
        return 0;
    }
}
pub(crate) unsafe extern "C" fn MapAllowsMatchCall() -> u32 {
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
    return TRUE as u32;
}
pub(crate) unsafe extern "C" fn UpdateMatchCallStepCounter() -> u32 {
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
        return 0;
    }
}
pub(crate) unsafe extern "C" fn SelectMatchCallTrainer() -> u32 {
    let mut matchCallId: u32 = 0;
    let mut numRegistered: u32 = GetNumRegisteredTrainers();
    if numRegistered == 0 {
        return FALSE as u32;
    }
    sMatchCallState.trainerId =
        GetActiveMatchCallTrainerId(rem_u32(Random() as u32, numRegistered)) as u16;
    sMatchCallState.triggeredFromScript = FALSE;
    if sMatchCallState.trainerId == REMATCH_TABLE_ENTRIES as u16 {
        return FALSE as u32;
    }
    matchCallId = GetTrainerMatchCallId(sMatchCallState.trainerId as i32) as u32;
    if GetRematchTrainerLocation(matchCallId as i32) == gMapHeader.regionMapSectionId as u16
        && TrainerIsEligibleForRematch(matchCallId as i32) == 0
    {
        return FALSE as u32;
    }
    return TRUE as u32;
}
pub(crate) unsafe extern "C" fn GetNumRegisteredTrainers() -> u32 {
    let mut i: u32 = 0;
    let mut count: u32 = 0;
    i = 0;
    count = 0;
    while i < REMATCH_WALLY_VR {
        if FlagGet(TRAINER_REGISTERED_FLAGS_START + i as u16) != 0 {
            count += 1;
        }
        i += 1;
    }
    return count;
}
pub(crate) unsafe extern "C" fn GetActiveMatchCallTrainerId(mut activeMatchCallId: u32) -> u32 {
    let mut i: u32 = 0;
    i = 0;
    while i < REMATCH_WALLY_VR {
        if FlagGet(TRAINER_REGISTERED_FLAGS_START + i as u16) != 0 {
            if activeMatchCallId == 0 {
                return gRematchTable[i].trainerIds[0] as u32;
            }
            activeMatchCallId -= 1;
        }
        i += 1;
    }
    return REMATCH_TABLE_ENTRIES as u32;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn TryStartMatchCall() -> u32 {
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
    return FALSE as u32;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn StartMatchCallFromScript(message: *mut u8) {
    sMatchCallState.triggeredFromScript = TRUE;
    StartMatchCall();
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsMatchCallTaskActive() -> u32 {
    return FuncIsActiveTask(Some(ExecuteMatchCall)) as u32;
}
pub(crate) unsafe extern "C" fn StartMatchCall() {
    if sMatchCallState.triggeredFromScript == 0 {
        LockPlayerFieldControls();
        FreezeObjectEvents();
        PlayerFreeze();
        StopPlayerAvatar();
    }
    PlaySE(SE_POKENAV_CALL);
    CreateTask(Some(ExecuteMatchCall), 1);
}
pub(crate) unsafe extern "C" fn ExecuteMatchCall(taskId: u8) {
    let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
    if sMatchCallTaskFuncs[*data].unwrap_unchecked()(taskId) != 0 {
        *data += 1;
        *data.at(1) = 0;
        if *data as u16 > 7 {
            DestroyTask(taskId);
        }
    }
}
pub(crate) unsafe extern "C" fn MatchCall_LoadGfx(taskId: u8) -> u32 {
    let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
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
    return TRUE as u32;
}
pub(crate) unsafe extern "C" fn MatchCall_DrawWindow(taskId: u8) -> u32 {
    let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
    if FreeTempTileDataBuffersIfPossible() != 0 {
        return FALSE as u32;
    }
    PutWindowTilemap(*data.at(2) as u8);
    DrawMatchCallTextBoxBorder_Internal(*data.at(2) as u32, TILE_MC_WINDOW as u32, 14);
    WriteSequenceToBgTilemapBuffer(0, 62073, 1, 15, 4, 4, 17, 1);
    *data.at(5) = CreateTask(Some(Task_SpinPokenavIcon), 10) as i16;
    CopyWindowToVram(*data.at(2) as u8, COPYWIN_GFX);
    CopyBgTilemapBufferToVram(0);
    return TRUE as u32;
}
pub(crate) unsafe extern "C" fn MatchCall_ReadyIntro(taskId: u8) -> u32 {
    let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
    if IsDma3ManagerBusyWithBgCopy() == 0 {
        InitMatchCallTextPrinter(
            *data.at(2) as i32,
            sText_PokenavCallEllipsis.as_ptr().cast_mut(),
        );
        return TRUE as u32;
    }
    return FALSE as u32;
}
pub(crate) unsafe extern "C" fn MatchCall_SlideWindowIn(taskId: u8) -> u32 {
    if ChangeBgY(0, 0x600, BG_COORD_ADD) >= 0 {
        ChangeBgY(0, 0, BG_COORD_SET);
        return TRUE as u32;
    }
    return FALSE as u32;
}
pub(crate) unsafe extern "C" fn MatchCall_PrintIntro(taskId: u8) -> u32 {
    let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
    if RunMatchCallTextPrinter(*data.at(2) as i32) == 0 {
        FillWindowPixelBuffer(*data.at(2) as u8, 136);
        if sMatchCallState.triggeredFromScript == 0 {
            SelectMatchCallMessage(sMatchCallState.trainerId as i32, gStringVar4.as_mut_ptr());
        }
        InitMatchCallTextPrinter(*data.at(2) as i32, gStringVar4.as_mut_ptr());
        return TRUE as u32;
    }
    return FALSE as u32;
}
pub(crate) unsafe extern "C" fn MatchCall_PrintMessage(taskId: u8) -> u32 {
    let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
    if RunMatchCallTextPrinter(*data.at(2) as i32) == 0
        && IsSEPlaying() == 0
        && gMain.newKeys as i32 & 3 != 0
    {
        FillWindowPixelBuffer(*data.at(2) as u8, 136);
        CopyWindowToVram(*data.at(2) as u8, COPYWIN_GFX);
        PlaySE(SE_POKENAV_HANG_UP);
        return TRUE as u32;
    }
    return FALSE as u32;
}
pub(crate) unsafe extern "C" fn MatchCall_SlideWindowOut(taskId: u8) -> u32 {
    let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
    if ChangeBgY(0, 0x600, BG_COORD_SUB) <= -8192 {
        FillBgTilemapBufferRect_Palette0(0, 0, 0, 14, 30, 6);
        DestroyTask(*data.at(5) as u8);
        RemoveWindow(*data.at(2) as u8);
        CopyBgTilemapBufferToVram(0);
        return TRUE as u32;
    }
    return FALSE as u32;
}
pub(crate) unsafe extern "C" fn MatchCall_EndCall(taskId: u8) -> u32 {
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
    return FALSE as u32;
}
pub(crate) unsafe extern "C" fn DrawMatchCallTextBoxBorder_Internal(
    windowId: u32,
    tileOffset: u32,
    paletteId: u32,
) {
    let mut bg: i32 = 0;
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut width: i32 = 0;
    let mut height: i32 = 0;
    let mut tileNum: i32 = 0;
    bg = GetWindowAttribute(windowId as u8, WINDOW_BG) as i32;
    x = GetWindowAttribute(windowId as u8, WINDOW_TILEMAP_LEFT) as i32;
    y = GetWindowAttribute(windowId as u8, WINDOW_TILEMAP_TOP) as i32;
    width = GetWindowAttribute(windowId as u8, WINDOW_WIDTH) as i32;
    height = GetWindowAttribute(windowId as u8, WINDOW_HEIGHT) as i32;
    tileNum = tileOffset as i32 + GetBgAttribute(bg as u8, BG_ATTR_BASETILE) as i32;
    FillBgTilemapBufferRect_Palette0(
        bg as u8,
        (paletteId as u16) << 12 & 0xF000 | tileNum as u16 + 0,
        x as u8 - 1,
        y as u8 - 1,
        1,
        1,
    );
    FillBgTilemapBufferRect_Palette0(
        bg as u8,
        (paletteId as u16) << 12 & 0xF000 | tileNum as u16 + 1,
        x as u8,
        y as u8 - 1,
        width as u8,
        1,
    );
    FillBgTilemapBufferRect_Palette0(
        bg as u8,
        (paletteId as u16) << 12 & 0xF000 | tileNum as u16 + 2,
        x as u8 + width as u8,
        y as u8 - 1,
        1,
        1,
    );
    FillBgTilemapBufferRect_Palette0(
        bg as u8,
        (paletteId as u16) << 12 & 0xF000 | tileNum as u16 + 3,
        x as u8 - 1,
        y as u8,
        1,
        height as u8,
    );
    FillBgTilemapBufferRect_Palette0(
        bg as u8,
        (paletteId as u16) << 12 & 0xF000 | tileNum as u16 + 4,
        x as u8 + width as u8,
        y as u8,
        1,
        height as u8,
    );
    FillBgTilemapBufferRect_Palette0(
        bg as u8,
        (paletteId as u16) << 12 & 0xF000 | tileNum as u16 + 5,
        x as u8 - 1,
        y as u8 + height as u8,
        1,
        1,
    );
    FillBgTilemapBufferRect_Palette0(
        bg as u8,
        (paletteId as u16) << 12 & 0xF000 | tileNum as u16 + 6,
        x as u8,
        y as u8 + height as u8,
        width as u8,
        1,
    );
    FillBgTilemapBufferRect_Palette0(
        bg as u8,
        (paletteId as u16) << 12 & 0xF000 | tileNum as u16 + 7,
        x as u8 + width as u8,
        y as u8 + height as u8,
        1,
        1,
    );
}
pub(crate) unsafe extern "C" fn InitMatchCallTextPrinter(windowId: i32, str: *mut u8) {
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
    gTextFlags.set_useAlternateDownArrow(FALSE);
    AddTextPrinter(&raw mut printerTemplate, GetPlayerTextSpeedDelay(), None);
}
pub(crate) unsafe extern "C" fn RunMatchCallTextPrinter(windowId: i32) -> u32 {
    if gMain.heldKeys as i32 & A_BUTTON != 0 {
        gTextFlags.set_canABSpeedUpPrint(TRUE);
    } else {
        gTextFlags.set_canABSpeedUpPrint(FALSE);
    }
    RunTextPrinters();
    return IsTextPrinterActive(windowId as u8) as u32;
}
pub(crate) unsafe extern "C" fn Task_SpinPokenavIcon(taskId: u8) {
    let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
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
pub(crate) unsafe extern "C" fn TrainerIsEligibleForRematch(matchCallId: i32) -> u32 {
    return ((*gSaveBlock1Ptr).trainerRematches[matchCallId] > 0) as u32;
}
pub(crate) unsafe extern "C" fn GetRematchTrainerLocation(matchCallId: i32) -> u16 {
    let mut mapHeader: *mut MapHeader = Overworld_GetMapHeaderByGroupAndId(
        gRematchTable[matchCallId].mapGroup,
        gRematchTable[matchCallId].mapNum,
    );
    return (*mapHeader).regionMapSectionId as u16;
}
pub(crate) unsafe extern "C" fn GetNumRematchTrainersFought() -> u32 {
    let mut i: u32 = 0;
    let mut count: u32 = 0;
    i = 0;
    count = 0;
    while i < REMATCH_WALLY_VR {
        if HasTrainerBeenFought(gRematchTable[i].trainerIds[0]) != 0 {
            count += 1;
        }
        i += 1;
    }
    return count;
}
pub(crate) unsafe extern "C" fn GetNthRematchTrainerFought(n: i32) -> u32 {
    let mut i: u32 = 0;
    let mut count: u32 = 0;
    i = 0;
    count = 0;
    while i < REMATCH_TABLE_ENTRIES as u32 {
        if HasTrainerBeenFought(gRematchTable[i].trainerIds[0]) != 0 {
            if count == n as u32 {
                return i;
            }
            count += 1;
        }
        i += 1;
    }
    return REMATCH_TABLE_ENTRIES as u32;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SelectMatchCallMessage(trainerId: i32, str: *mut u8) -> u32 {
    let mut matchCallId: u32 = 0;
    let mut matchCallText: *mut MatchCallText = null_mut();
    let mut newRematchRequest: u32 = FALSE as u32;
    matchCallId = GetTrainerMatchCallId(trainerId) as u32;
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
    return newRematchRequest;
}
pub(crate) unsafe extern "C" fn GetTrainerMatchCallId(trainerId: i32) -> i32 {
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
        return 0;
    }
}
pub(crate) unsafe extern "C" fn GetSameRouteMatchCallText(
    matchCallId: i32,
    str: *mut u8,
) -> *mut MatchCallText {
    let mut textId: u16 = sMatchCallTrainers[matchCallId].sameRouteMatchCallTextId;
    let mut mask: i32 = 0xFF;
    let mut topic: u32 = (textId >> 8) as u32 - 1;
    let mut id: u32 = (textId as u32 & mask as u32) - 1;
    return sMatchCallBattleRequestTopics[topic].at(id);
}
pub(crate) unsafe extern "C" fn GetDifferentRouteMatchCallText(
    matchCallId: i32,
    str: *mut u8,
) -> *mut MatchCallText {
    let mut textId: u16 = sMatchCallTrainers[matchCallId].differentRouteMatchCallTextId;
    let mut mask: i32 = 0xFF;
    let mut topic: u32 = (textId >> 8) as u32 - 1;
    let mut id: u32 = (textId as u32 & mask as u32) - 1;
    return sMatchCallBattleRequestTopics[topic].at(id);
}
pub(crate) unsafe extern "C" fn GetBattleMatchCallText(
    matchCallId: i32,
    str: *mut u8,
) -> *mut MatchCallText {
    let mut mask: i32 = 0;
    let mut textId: u32 = 0;
    let mut topic: u32 = 0;
    let mut id: u32 = 0;
    topic = (Random() as i32 % 3) as u32;
    textId = sMatchCallTrainers[matchCallId].battleTopicTextIds[topic] as u32;
    if textId == 0 {
        SpriteCallbackDummy(null_mut());
    }
    mask = 0xFF;
    topic = (textId >> 8) - 1;
    id = (textId & mask as u32) - 1;
    return sMatchCallBattleTopics[topic].at(id);
}
pub(crate) unsafe extern "C" fn GetGeneralMatchCallText(
    matchCallId: i32,
    str: *mut u8,
) -> *mut MatchCallText {
    let mut i: i32 = 0;
    let mut count: i32 = 0;
    let mut topic: u32 = 0;
    let mut id: u32 = 0;
    let mut rand: u16 = 0;
    rand = Random();
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
    return sMatchCallGeneralTopics[topic].at(id);
}
pub(crate) unsafe extern "C" fn BuildMatchCallString(
    matchCallId: i32,
    matchCallText: *mut MatchCallText,
    str: *mut u8,
) {
    PopulateMatchCallStringVars(matchCallId, (*matchCallText).stringVarFuncIds.as_mut_ptr());
    StringExpandPlaceholders(str, (*matchCallText).text);
}
pub(crate) unsafe extern "C" fn PopulateMatchCallStringVars(
    matchCallId: i32,
    stringVarFuncIds: *mut i8,
) {
    let mut i: i32 = 0;
    i = 0;
    while i < NUM_STRVARS_IN_MSG {
        if *stringVarFuncIds.at(i) >= 0 {
            PopulateMatchCallStringVar(
                matchCallId,
                *stringVarFuncIds.at(i) as i32,
                sMatchCallTextStringVars[i],
            );
        }
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn PopulateMatchCallStringVar(
    matchCallId: i32,
    funcId: i32,
    destStr: *mut u8,
) {
    sPopulateMatchCallStringVarFuncs[funcId].unwrap_unchecked()(matchCallId, destStr);
}
pub(crate) unsafe extern "C" fn PopulateTrainerName(matchCallId: i32, destStr: *mut u8) {
    let mut i: u32 = 0;
    let mut trainerId: u16 = sMatchCallTrainers[matchCallId].trainerId;
    i = 0;
    while i < 6 {
        if sMultiTrainerMatchCallTexts[i].trainerId == trainerId {
            StringCopy(destStr, sMultiTrainerMatchCallTexts[i].text);
            return;
        }
        i += 1;
    }
    StringCopy(
        destStr,
        gTrainers[trainerId].trainerName.as_ptr().cast_mut(),
    );
}
pub(crate) unsafe extern "C" fn PopulateMapName(matchCallId: i32, destStr: *mut u8) {
    GetMapName(destStr, GetRematchTrainerLocation(matchCallId), 0);
}
pub(crate) unsafe extern "C" fn GetLandEncounterSlot() -> u8 {
    let mut rand: i32 = Random() as i32 % 100;
    if rand < ENCOUNTER_CHANCE_LAND_MONS_SLOT_0 {
        return 0;
    } else if rand >= ENCOUNTER_CHANCE_LAND_MONS_SLOT_0 && rand < ENCOUNTER_CHANCE_LAND_MONS_SLOT_1
    {
        return 1;
    } else if rand >= ENCOUNTER_CHANCE_LAND_MONS_SLOT_1 && rand < ENCOUNTER_CHANCE_LAND_MONS_SLOT_2
    {
        return 2;
    } else if rand >= ENCOUNTER_CHANCE_LAND_MONS_SLOT_2 && rand < ENCOUNTER_CHANCE_LAND_MONS_SLOT_3
    {
        return 3;
    } else if rand >= ENCOUNTER_CHANCE_LAND_MONS_SLOT_3 && rand < ENCOUNTER_CHANCE_LAND_MONS_SLOT_4
    {
        return 4;
    } else if rand >= ENCOUNTER_CHANCE_LAND_MONS_SLOT_4 && rand < ENCOUNTER_CHANCE_LAND_MONS_SLOT_5
    {
        return 5;
    } else if rand >= ENCOUNTER_CHANCE_LAND_MONS_SLOT_5 && rand < ENCOUNTER_CHANCE_LAND_MONS_SLOT_6
    {
        return 6;
    } else if rand >= ENCOUNTER_CHANCE_LAND_MONS_SLOT_6 && rand < ENCOUNTER_CHANCE_LAND_MONS_SLOT_7
    {
        return 7;
    } else if rand >= ENCOUNTER_CHANCE_LAND_MONS_SLOT_7 && rand < ENCOUNTER_CHANCE_LAND_MONS_SLOT_8
    {
        return 8;
    } else if rand >= ENCOUNTER_CHANCE_LAND_MONS_SLOT_8 && rand < ENCOUNTER_CHANCE_LAND_MONS_SLOT_9
    {
        return 9;
    } else if rand >= ENCOUNTER_CHANCE_LAND_MONS_SLOT_9 && rand < ENCOUNTER_CHANCE_LAND_MONS_SLOT_10
    {
        return 10;
    } else {
        return 11;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn GetWaterEncounterSlot() -> u8 {
    let mut rand: i32 = Random() as i32 % 100;
    if rand < ENCOUNTER_CHANCE_WATER_MONS_SLOT_0 {
        return 0;
    } else if rand >= ENCOUNTER_CHANCE_WATER_MONS_SLOT_0
        && rand < ENCOUNTER_CHANCE_WATER_MONS_SLOT_1
    {
        return 1;
    } else if rand >= ENCOUNTER_CHANCE_WATER_MONS_SLOT_1
        && rand < ENCOUNTER_CHANCE_WATER_MONS_SLOT_2
    {
        return 2;
    } else if rand >= ENCOUNTER_CHANCE_WATER_MONS_SLOT_2
        && rand < ENCOUNTER_CHANCE_WATER_MONS_SLOT_3
    {
        return 3;
    } else {
        return 4;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn PopulateSpeciesFromTrainerLocation(
    matchCallId: i32,
    mut destStr: *mut u8,
) {
    let mut species: CArray<u16, 2> = zeroed();
    let mut numSpecies: i32 = 0;
    let mut slot: u8 = 0;
    let mut i: i32 = 0;
    if gWildMonHeaders[i].mapGroup != 255 {
        while gWildMonHeaders[i].mapGroup != 255 {
            if gWildMonHeaders[i].mapGroup as u16 == gRematchTable[matchCallId].mapGroup
                && gWildMonHeaders[i].mapNum as u16 == gRematchTable[matchCallId].mapNum
            {
                break;
            }
            i += 1;
        }
        if gWildMonHeaders[i].mapGroup != 255 {
            numSpecies = 0;
            if !gWildMonHeaders[i].landMonsInfo.is_null() {
                slot = GetLandEncounterSlot();
                species[numSpecies] =
                    (*(*gWildMonHeaders[i].landMonsInfo).wildPokemon.at(slot)).species;
                numSpecies += 1;
            }
            if !gWildMonHeaders[i].waterMonsInfo.is_null() {
                slot = GetWaterEncounterSlot();
                species[numSpecies] =
                    (*(*gWildMonHeaders[i].waterMonsInfo).wildPokemon.at(slot)).species;
                numSpecies += 1;
            }
            if numSpecies != 0 {
                StringCopy(
                    destStr,
                    gSpeciesNames[species[rem_i32(Random() as i32, numSpecies)]]
                        .as_ptr()
                        .cast_mut(),
                );
                return;
            }
        }
    }
    *destStr = EOS;
}
pub(crate) unsafe extern "C" fn PopulateSpeciesFromTrainerParty(
    matchCallId: i32,
    destStr: *mut u8,
) {
    let mut trainerId: u16 = 0;
    let mut party: TrainerMonPtr = zeroed();
    let mut monId: u8 = 0;
    let mut speciesName: *mut u8 = null_mut();
    trainerId = GetLastBeatenRematchTrainerId(sMatchCallTrainers[matchCallId].trainerId);
    party = gTrainers[trainerId].party;
    monId = rem_i32(Random() as i32, gTrainers[trainerId].partySize as i32) as u8;
    match gTrainers[trainerId].partyFlags {
        F_TRAINER_PARTY_CUSTOM_MOVESET => {
            speciesName = gSpeciesNames[(*party.NoItemCustomMoves.at(monId)).species]
                .as_ptr()
                .cast_mut();
        }
        F_TRAINER_PARTY_HELD_ITEM => {
            speciesName = gSpeciesNames[(*party.ItemDefaultMoves.at(monId)).species]
                .as_ptr()
                .cast_mut();
        }
        3 => {
            speciesName = gSpeciesNames[(*party.ItemCustomMoves.at(monId)).species]
                .as_ptr()
                .cast_mut();
        }
        _ => {
            speciesName = gSpeciesNames[(*party.NoItemDefaultMoves.at(monId)).species]
                .as_ptr()
                .cast_mut();
        }
    }
    StringCopy(destStr, speciesName);
}
pub(crate) unsafe extern "C" fn PopulateBattleFrontierFacilityName(
    matchCallId: i32,
    destStr: *mut u8,
) {
    StringCopy(
        destStr,
        sBattleFrontierFacilityNames[sBattleFrontierStreakInfo.facilityId],
    );
}
pub(crate) unsafe extern "C" fn PopulateBattleFrontierStreak(matchCallId: i32, destStr: *mut u8) {
    let mut i: i32 = 0;
    let mut streak: i32 = sBattleFrontierStreakInfo.streak as i32;
    while streak != 0 {
        streak = streak / 10;
        i += 1;
    }
    ConvertIntToDecimalStringN(
        destStr,
        sBattleFrontierStreakInfo.streak as i32,
        STR_CONV_MODE_LEFT_ALIGN,
        i as u8,
    );
}
pub(crate) unsafe extern "C" fn GetNumOwnedBadges() -> i32 {
    let mut i: u32 = 0;
    i = 0;
    while i < NUM_BADGES {
        if FlagGet(sBadgeFlags[i]) == 0 {
            break;
        }
        i += 1;
    }
    return i as i32;
}
pub(crate) unsafe extern "C" fn ShouldTrainerRequestBattle(matchCallId: i32) -> u32 {
    let mut dayCount: i32 = 0;
    let mut otId: i32 = 0;
    let mut dewfordRand: u16 = 0;
    let mut numRematchTrainersFought: i32 = 0;
    let mut max: i32 = 0;
    let mut rand: i32 = 0;
    let mut n: i32 = 0;
    if GetNumOwnedBadges() < 5 {
        return FALSE as u32;
    }
    dayCount = RtcGetLocalDayCount() as i32;
    otId = GetTrainerId((*gSaveBlock2Ptr).playerTrainerId.as_mut_ptr()) as i32 & 0xFFFF;
    dewfordRand = (*gSaveBlock1Ptr).dewfordTrends[0].rand;
    numRematchTrainersFought = GetNumRematchTrainersFought() as i32;
    max = numRematchTrainersFought * 13 / 10;
    rand = (dayCount ^ dewfordRand as i32)
        + (dewfordRand as i32 ^ GetGameStat(GAME_STAT_TRAINER_BATTLES) as i32)
        ^ otId;
    n = rem_i32(rand, max);
    if n < numRematchTrainersFought {
        if GetNthRematchTrainerFought(n) == matchCallId as u32 {
            return TRUE as u32;
        }
    }
    return FALSE as u32;
}
pub(crate) unsafe extern "C" fn GetFrontierStreakInfo(
    facilityId: u16,
    topicTextId: *mut u32,
) -> u16 {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut streak: u16 = 0;
    match facilityId {
        1 => {
            i = 0;
            while i < 2 {
                j = 0;
                while j < FRONTIER_LVL_MODE_COUNT {
                    if streak < (*gSaveBlock2Ptr).frontier.domeRecordWinStreaks[i][j] {
                        streak = (*gSaveBlock2Ptr).frontier.domeRecordWinStreaks[i][j];
                    }
                    j += 1;
                }
                i += 1;
            }
            *topicTextId = 3;
        }
        MATCH_CALL_PIKE => {
            i = 0;
            while i < FRONTIER_LVL_MODE_COUNT {
                if streak < (*gSaveBlock2Ptr).frontier.pikeRecordStreaks[i] {
                    streak = (*gSaveBlock2Ptr).frontier.pikeRecordStreaks[i];
                }
                i += 1;
            }
            *topicTextId = 4;
        }
        FRONTIER_FACILITY_TOWER => {
            i = 0;
            while i < 4 {
                j = 0;
                while j < FRONTIER_LVL_MODE_COUNT {
                    if streak < (*gSaveBlock2Ptr).frontier.towerRecordWinStreaks[i][j] {
                        streak = (*gSaveBlock2Ptr).frontier.towerRecordWinStreaks[i][j];
                    }
                    j += 1;
                }
                i += 1;
            }
            *topicTextId = 2;
        }
        2 => {
            i = 0;
            while i < 2 {
                j = 0;
                while j < FRONTIER_LVL_MODE_COUNT {
                    if streak < (*gSaveBlock2Ptr).frontier.palaceRecordWinStreaks[i][j] {
                        streak = (*gSaveBlock2Ptr).frontier.palaceRecordWinStreaks[i][j];
                    }
                    j += 1;
                }
                i += 1;
            }
            *topicTextId = 2;
        }
        MATCH_CALL_FACTORY => {
            i = 0;
            while i < 2 {
                j = 0;
                while j < FRONTIER_LVL_MODE_COUNT {
                    if streak < (*gSaveBlock2Ptr).frontier.factoryRecordWinStreaks[i][j] {
                        streak = (*gSaveBlock2Ptr).frontier.factoryRecordWinStreaks[i][j];
                    }
                    j += 1;
                }
                i += 1;
            }
            *topicTextId = 2;
        }
        3 => {
            i = 0;
            while i < FRONTIER_LVL_MODE_COUNT {
                if streak < (*gSaveBlock2Ptr).frontier.arenaRecordStreaks[i] {
                    streak = (*gSaveBlock2Ptr).frontier.arenaRecordStreaks[i];
                }
                i += 1;
            }
            *topicTextId = 2;
        }
        6 => {
            i = 0;
            while i < FRONTIER_LVL_MODE_COUNT {
                if streak < (*gSaveBlock2Ptr).frontier.pyramidRecordStreaks[i] {
                    streak = (*gSaveBlock2Ptr).frontier.pyramidRecordStreaks[i];
                }
                i += 1;
            }
            *topicTextId = 5;
        }
        _ => {}
    }
    return streak;
}
pub(crate) unsafe extern "C" fn GetPokedexRatingLevel(mut numSeen: u16) -> u8 {
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
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BufferPokedexRatingForMatchCall(mut destStr: *mut u8) {
    let mut numSeen: i32 = 0;
    let mut numCaught: i32 = 0;
    let mut str: *mut u8 = null_mut();
    let mut dexRatingLevel: u8 = 0;
    let mut buffer: *mut u8 = Alloc(1000) as *mut u8;
    if buffer.is_null() {
        *destStr = EOS;
        return;
    }
    numSeen = GetHoennPokedexCount(FLAG_GET_SEEN) as i32;
    numCaught = GetHoennPokedexCount(FLAG_GET_CAUGHT) as i32;
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
    dexRatingLevel = GetPokedexRatingLevel(numCaught as u16);
    str = StringCopy(
        buffer,
        gBirchDexRatingText_AreYouCurious.as_ptr().cast_mut(),
    );
    *({
        let t1 = str;
        str = str.at(1);
        t1
    }) = CHAR_PROMPT_CLEAR;
    str = StringCopy(
        str,
        gBirchDexRatingText_SoYouveSeenAndCaught.as_ptr().cast_mut(),
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
            gBirchDexRatingText_OnANationwideBasis.as_ptr().cast_mut(),
        );
    }
    Free(buffer as *mut c_void);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn LoadMatchCallWindowGfx(windowId: u32, destOffset: u32, paletteId: u32) {
    let mut bg: u8 = GetWindowAttribute(windowId as u8, WINDOW_BG) as u8;
    LoadBgTiles(
        bg,
        sMatchCallWindow_Gfx.as_ptr().cast_mut() as *mut c_void,
        0x100,
        destOffset as u16,
    );
    LoadPalette(
        sMatchCallWindow_Pal.as_ptr().cast_mut() as *mut c_void,
        0x000 + paletteId as u16 * 16,
        32,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DrawMatchCallTextBoxBorder(
    windowId: u32,
    tileOffset: u32,
    paletteId: u32,
) {
    DrawMatchCallTextBoxBorder_Internal(windowId, tileOffset, paletteId);
}
