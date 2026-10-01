//! Translated from `src/overworld.c` by tools/rustport/c2rs.py.
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
    clippy::missing_transmute_annotations,
    clippy::type_complexity,
    clippy::useless_transmute,
    dead_code,
    unused_assignments,
    unused_variables
)]

use crate::agb_main::gMain;
use crate::agb_main::{SetHBlankCallback, SetVBlankCallback};
use crate::battle_pyramid::{
    InBattlePyramid_, LoadBattlePyramidFloorObjectEventScripts,
    LoadBattlePyramidObjectEventTemplates,
};
use crate::battle_setup::TryUpdateRandomTrainerRematches;
use crate::bg::{ChangeBgX, ChangeBgY, SetBgAttribute, ShowBg};
#[allow(unused_imports)]
use crate::c::*;
use crate::cable_club::GetLinkTrainerCardColor;
use crate::clock::DoTimeBasedEvents;
#[allow(unused_imports)]
use crate::consts::*;
use crate::event_data::{ClearTempFieldEventData, FlagClear, FlagGet, VarGet, VarSet};
use crate::event_object_movement::{
    CreateObjectGraphicsSprite, ElevationToPriority, GetFaceDirectionAnimNum,
    GetFirstInactiveObjectEventId, GetMoveDirectionAnimNum, InitObjectEventPalettes, MoveCoords,
    ObjectEventMoveDestCoords, ObjectEventUpdateElevation, ResetObjectEvents,
    SetObjectSubpriorityByElevation, SetSpritePosToMapCoords, ShiftObjectEventCoords,
    ShiftStillObjectEventCoords, SpawnObjectEventsOnReturnToField, TrySpawnObjectEvents,
    UnfreezeObjectEvents, UpdateObjectEventSpriteInvisibility,
};
use crate::field_camera::{
    CameraUpdate, DrawWholeMapView, FieldUpdateBgTilemapScroll, InitCameraUpdateCallback,
    InstallCameraPanAheadCallback, ResetCameraUpdateInfo, ResetFieldCamera, UpdateCameraPanning,
    gTotalCameraPixelOffsetX, gTotalCameraPixelOffsetY,
};
use crate::field_control_avatar::{
    FieldClearPlayerInput, FieldGetPlayerInput, GetCoordEventScriptAtMapPosition,
    GetInteractedLinkPlayerScript, ProcessPlayerFieldInput, RestartWildEncounterImmunitySteps,
};
use crate::field_effect::FieldEffectActiveListClear;
use crate::field_message_box::InitFieldMessageBox;
use crate::field_player_avatar::{
    GetFRLGAvatarGraphicsIdByGender, GetPlayerFacingDirection, GetRSAvatarGraphicsIdByGender,
    GetRivalAvatarGraphicsIdByStateIdAndGender, InitPlayerAvatar, PlayerGetDestCoords, PlayerStep,
    SetPlayerAvatarTransitionFlags, TestPlayerAvatarFlags, UpdatePlayerAvatarTransitionState,
    gObjectEvents, gPlayerAvatar,
};
use crate::field_screen_effect::{
    FieldCB_ContinueScript, FieldCB_ContinueScriptHandleMusic, FieldCB_DefaultWarpExit,
    FieldCB_ReturnToFieldCableLink, FieldCB_ReturnToFieldOpenStartMenu,
    FieldCB_ReturnToFieldWirelessLink, FieldCB_WarpExitFadeFromBlack,
    WriteBattlePyramidViewScanlineEffectBuffer, WriteFlashScanlineEffectBuffer,
};
use crate::field_special_scene::ExecuteTruckSequence;
use crate::field_specials::{ResetCyclingRoadChallengeData, UsedPokemonCenterWarp};
use crate::field_tasks::{ResetFieldTasksArgs, SetUpFieldTasks};
use crate::field_weather::{ApplyWeatherColorMapToPal, StartWeather};
use crate::field_weather_effect::{
    DoCurrentWeather, GetSavedWeather, ResumePausedWeather, SetSavedWeatherFromCurrMapHeader,
};
use crate::fieldmap::{
    CopyMapTilesetsToVram, CopyPrimaryTilesetToVram, CopySecondaryTilesetToVram,
    CopySecondaryTilesetToVramUsingHeap, GetCameraFocusCoords, InitBattlePyramidMap, InitMap,
    InitMapFromSavedGame, InitTrainerHillMap, LoadMapTilesetPalettes, LoadSecondaryTilesetPalette,
    MapGridGetCollisionAt, MapGridGetMetatileBehaviorAt, SetCameraFocusCoords, gBackupMapLayout,
    gMapHeader,
};
use crate::fldeff_flash::CB2_DoChangeMap;
use crate::frontier_util::ResetWinStreaks;
use crate::gpu_regs::{DisableInterrupts, EnableInterrupts, SetGpuReg};
use crate::link::gLinkPartnersHeldKeys;
use crate::link::{
    CloseLink, GetLinkRecvQueueLength, IsSendingKeysToLink, gLink, gLinkPlayers, gWirelessCommType,
};
use crate::link_rfu_2::{IsRfuRecvQueueEmpty, LinkRfu_FatalError, gRfu};
use crate::link_rfu_3::{
    CreateWirelessStatusIndicatorSprite, LoadWirelessStatusIndicatorSpriteGfx,
};
use crate::load_save::{
    ApplyNewEncryptionKeyToWord, ClearContinueGameWarpStatus, MoveSaveBlocks_ResetHeap,
    UseContinueGameWarp,
};
use crate::load_save::{gSaveBlock1Ptr, gSaveBlock2Ptr};
use crate::map_name_popup::{HideMapNamePopUpWindow, ShowMapNamePopup};
use crate::match_call::InitMatchCallCounters;
use crate::menu::{
    ClearScheduledBgCopiesToVram, DoScheduledBgTilemapCopiesToVram, FreeAllOverworldWindowBuffers,
    FreeTempTileDataBuffersIfPossible, InitStandardTextBoxWindows, InitTextBoxGfxAndPrinters,
    ResetTempTileDataBuffers, ScheduleBgCopyTilemapToVram,
};
use crate::metatile_behavior::{
    MetatileBehavior_IsDeepSouthWarp, MetatileBehavior_IsDoor, MetatileBehavior_IsEastArrowWarp,
    MetatileBehavior_IsLadder, MetatileBehavior_IsNonAnimDoor, MetatileBehavior_IsNorthArrowWarp,
    MetatileBehavior_IsSouthArrowWarp, MetatileBehavior_IsSurfableWaterOrUnderwater,
    MetatileBehavior_IsWestArrowWarp,
};
use crate::mirage_tower::{
    ClearMirageTowerPulseBlend, ClearMirageTowerPulseBlendEffect,
    TryStartMirageTowerPulseBlendEffect,
};
use crate::money::{GetMoney, SetMoney};
use crate::new_game::NewGameInitData;
use crate::palette::{ResetPaletteFade, TransferPlttBuffer, UpdatePaletteFade, gPaletteFade};
use crate::play_time::PlayTimeCounter_Start;
use crate::pokemon::{CalculatePlayerPartyCount, GetMonAbility, GetMonData2, gPlayerParty};
use crate::random::Random;
use crate::roamer::{RoamerMove, RoamerMoveToOtherLocationSet, UpdateLocationHistoryForRoamer};
use crate::rotating_gate::RotatingGate_InitPuzzleAndGraphics;
use crate::safari_zone::ResetSafariZoneFlag;
use crate::save::gSaveFileStatus;
use crate::save_location::TrySetMapSaveWarpStatus;
use crate::scanline_effect::{
    ScanlineEffect_Clear, ScanlineEffect_InitHBlankDmaTransfer, ScanlineEffect_Stop,
};
use crate::script::{
    ArePlayerFieldControlsLocked, LockPlayerFieldControls, RunOnDiveWarpMapScript,
    RunOnResumeMapScript, RunOnReturnToFieldMapScript, RunOnTransitionMapScript,
    ScriptContext_Init, ScriptContext_RunScript, TryRunOnWarpIntoMapScript,
    UnlockPlayerFieldControls,
};
use crate::script_pokemon_util::HealPlayerParty;
use crate::secret_base::{
    CheckLeftFriendsSecretBase, InitSecretBaseAppearance, SecretBaseMapPopupEnabled,
};
use crate::sound::{
    FadeOutAndFadeInNewMapMusic, FadeOutAndPlayNewMapMusic, FadeOutMapMusic, GetCurrentMapMusic,
    IsNotWaitingForBGMStop, PlayCry_NormalNoDucking, PlayNewMapMusic, PlaySE, ResetMapMusic,
    StopMapMusic,
};
use crate::sprite::gSprites;
use crate::sprite::{
    AnimateSprites, BuildOamBuffer, LoadOam, ProcessSpriteCopyRequests, ResetOamRange,
    ResetSpriteData,
};
use crate::start_menu::ShowStartMenu;
use crate::task::{ResetTasks, RunTasks};
use crate::tileset_anims::{
    InitSecondaryTilesetAnimation, InitTilesetAnimations, TransferTilesetAnimsBuffer,
    UpdateTilesetAnimations,
};
use crate::time_events::IsMirageIslandPresent;
use crate::trainer_hill::{
    GetCurrentTrainerHillMapId, InTrainerHill, LoadTrainerHillFloorObjectEventScripts,
    LoadTrainerHillObjectEventTemplates, TryLoadTrainerHillEReaderPalette,
};
use crate::trainer_pokemon_sprites::ResetAllPicSprites;
use crate::tv::{TryPutTodaysRivalTrainerOnAir, UpdateTVScreensOnMap};
#[allow(unused_imports)]
use crate::types::*;
use crate::wild_encounter::{GetLocalWaterMon, GetLocalWildMon};
#[allow(unused_imports)]
use core::ffi::c_void;
#[allow(unused_imports)]
use core::mem::zeroed;
#[allow(unused_imports)]
use core::ptr::null_mut;
/// `DestroySprite` with this module's view of its types.
#[inline]
unsafe fn DestroySprite(a0: *mut Sprite) {
    unsafe {
        crate::sprite::DestroySprite(a0 as _);
    }
}
/// `Free` with this module's view of its types.
#[inline]
unsafe fn Free(a0: *mut c_void) {
    unsafe {
        crate::malloc::Free(a0 as _);
    }
}
/// `InitBgsFromTemplates` with this module's view of its types.
#[inline]
unsafe fn InitBgsFromTemplates(a0: u8, a1: *mut BgTemplate, a2: u8) {
    unsafe {
        crate::bg::InitBgsFromTemplates(a0, a1 as _, a2);
    }
}
/// `RunScriptImmediately` with this module's view of its types.
#[inline]
unsafe fn RunScriptImmediately(a0: *mut u8) {
    unsafe {
        crate::script::RunScriptImmediately(a0 as _);
    }
}
/// `ScanlineEffect_SetParams` with this module's view of its types.
#[inline]
unsafe fn ScanlineEffect_SetParams(a0: ScanlineEffectParams) {
    unsafe {
        crate::scanline_effect::ScanlineEffect_SetParams(core::mem::transmute(a0));
    }
}
/// `ScriptContext_SetupScript` with this module's view of its types.
#[inline]
unsafe fn ScriptContext_SetupScript(a0: *mut u8) {
    unsafe {
        crate::script::ScriptContext_SetupScript(a0 as _);
    }
}
/// `SetBgTilemapBuffer` with this module's view of its types.
#[inline]
unsafe fn SetBgTilemapBuffer(a0: u8, a1: *mut c_void) {
    unsafe {
        crate::bg::SetBgTilemapBuffer(a0, a1 as _);
    }
}
/// `StartSpriteAnim` with this module's view of its types.
#[inline]
unsafe fn StartSpriteAnim(a0: *mut Sprite, a1: u8) {
    unsafe {
        crate::sprite::StartSpriteAnim(a0 as _, a1);
    }
}
/// `StartSpriteAnimIfDifferent` with this module's view of its types.
#[inline]
unsafe fn StartSpriteAnimIfDifferent(a0: *mut Sprite, a1: u8) {
    unsafe {
        crate::sprite::StartSpriteAnimIfDifferent(a0 as _, a1);
    }
}
// Data tables (translate with cdata.py): sDummyWarpData sUnusedData gDirectionToVectors sOverworldBgTemplates sFlashEffectParams sLinkPlayerMovementModes sLinkPlayerFacingHandlers sMovementStatusHandler

/// `struct CableClubPlayer`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CableClubPlayer {
    pub playerId: u8,
    pub isLocalPlayer: u8,
    pub movementMode: u8,
    pub facing: u8,
    pub pos: MapPosition,
    pub metatileBehavior: u16,
}

unsafe impl Sync for CableClubPlayer {}

#[cfg(target_arch = "arm")]
const _: () = {
    #[allow(unused_imports)]
    use core::mem::{offset_of, size_of};
    assert!(size_of::<CableClubPlayer>() == 16);
    assert!(offset_of!(CableClubPlayer, playerId) == 0);
    assert!(offset_of!(CableClubPlayer, isLocalPlayer) == 1);
    assert!(offset_of!(CableClubPlayer, movementMode) == 2);
    assert!(offset_of!(CableClubPlayer, facing) == 3);
    assert!(offset_of!(CableClubPlayer, pos) == 4);
    assert!(offset_of!(CableClubPlayer, metatileBehavior) == 12);
};

const AMB_CRY_FIRST: i16 = 1;
const AMB_CRY_IDLE: i16 = 4;
const AMB_CRY_INIT: i16 = 0;
const AMB_CRY_RESET: i16 = 2;
const AMB_CRY_WAIT: i16 = 3;
const FACING_DOWN: u16 = 2;
const FACING_FORCED_DOWN: u8 = 8;
const FACING_FORCED_LEFT: u16 = 9;
const FACING_FORCED_RIGHT: u16 = 10;
const FACING_FORCED_UP: u8 = 7;
const FACING_LEFT: u16 = 3;
const FACING_NONE: u16 = 0;
const FACING_RIGHT: u16 = 4;
const FACING_UP: u8 = 1;
const PLAYER_LINK_STATE_BUSY: u8 = 129;
const PLAYER_LINK_STATE_EXITING_ROOM: u16 = 131;
const PLAYER_LINK_STATE_IDLE: u8 = 128;
const PLAYER_LINK_STATE_READY: u8 = 130;

static gDirectionToVectors: Table<CArray<UCoords32, 9>> =
    Table((&raw const crate::data::overworld::gDirectionToVectors).cast());
static sDummyWarpData: Table<WarpData> =
    Table((&raw const crate::data::overworld::sDummyWarpData).cast());
static sFlashEffectParams: Table<ScanlineEffectParams> =
    Table((&raw const crate::data::overworld::sFlashEffectParams).cast());
static sLinkPlayerFacingHandlers: Table<
    CArray<Option<unsafe fn(*mut LinkPlayerObjectEvent, *mut ObjectEvent, u8) -> u8>, 11>,
> = Table((&raw const crate::data::overworld::sLinkPlayerFacingHandlers).cast());
static sLinkPlayerMovementModes: Table<
    CArray<Option<unsafe fn(*mut LinkPlayerObjectEvent, *mut ObjectEvent, u8) -> u8>, 3>,
> = Table((&raw const crate::data::overworld::sLinkPlayerMovementModes).cast());
static sMovementStatusHandler: Table<
    CArray<Option<unsafe fn(*mut LinkPlayerObjectEvent, *mut ObjectEvent)>, 2>,
> = Table((&raw const crate::data::overworld::sMovementStatusHandler).cast());
static sOverworldBgTemplates: Table<CArray<BgTemplate, 4>> =
    Table((&raw const crate::data::overworld::sOverworldBgTemplates).cast());

pub(crate) static mut sUnusedOverworldCallback: *mut c_void = null_mut();
pub(crate) static mut sPlayerLinkStates: Aligned<CArray<u8, 4>> = Aligned(unsafe { zeroed() });
pub(crate) static mut sPlayerKeyInterceptCallback: Option<unsafe fn(u32) -> u16> = None;
pub(crate) static sReceivingFromLink: crate::global::Global<u8> = crate::global::Global::new(0);
pub(crate) static sRfuKeepAliveTimer: crate::global::Global<u8> = crate::global::Global::new(0);
#[unsafe(link_section = "common_data")]
pub static mut gOverworldTilemapBuffer_Bg2: *mut u16 = null_mut();
#[unsafe(link_section = "common_data")]
pub static mut gOverworldTilemapBuffer_Bg1: *mut u16 = null_mut();
#[unsafe(link_section = "common_data")]
pub static mut gOverworldTilemapBuffer_Bg3: *mut u16 = null_mut();
#[unsafe(link_section = "common_data")]
pub static mut gHeldKeyCodeToSend: u16 = 0;
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gFieldCallback: Option<unsafe fn()> = None;
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gFieldCallback2: Option<unsafe fn() -> u8> = None;
#[unsafe(link_section = "common_data")]
pub static mut gLocalLinkPlayerId: u8 = 0;
#[unsafe(link_section = "common_data")]
pub static mut gFieldLinkPlayerCount: u8 = 0;
#[unsafe(link_section = "ewram_data")]
pub(crate) static sObjectEventLoadFlag: crate::global::Global<u8> = crate::global::Global::new(0);
#[unsafe(link_section = "ewram_data")]
pub static mut gLastUsedWarp: WarpData = unsafe { zeroed() };
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sWarpDestination: WarpData = unsafe { zeroed() };
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sFixedDiveWarp: WarpData = unsafe { zeroed() };
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sFixedHoleWarp: WarpData = unsafe { zeroed() };
#[unsafe(link_section = "ewram_data")]
pub(crate) static sLastMapSectionId: crate::global::Global<u16> = crate::global::Global::new(0);
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sInitialPlayerAvatarState: InitialPlayerAvatarState = unsafe { zeroed() };
#[unsafe(link_section = "ewram_data")]
pub(crate) static sAmbientCrySpecies: crate::global::Global<u16> = crate::global::Global::new(0);
#[unsafe(link_section = "ewram_data")]
pub(crate) static sIsAmbientCryWaterMon: crate::global::Global<u8> = crate::global::Global::new(0);
#[unsafe(link_section = "ewram_data")]
pub static mut gLinkPlayerObjectEvents: CArray<LinkPlayerObjectEvent, 4> = unsafe { zeroed() };

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
/// `GetHealLocation` with this module's view of its types.
#[inline]
unsafe fn GetHealLocation(a0: u32) -> *mut HealLocation {
    crate::heal_location::GetHealLocation(a0) as *mut HealLocation
}
/// `SetMainCallback2` with this module's view of its types.
#[inline]
unsafe fn SetMainCallback2(a0: Option<unsafe fn()>) {
    unsafe {
        crate::agb_main::SetMainCallback2(core::mem::transmute(a0));
    }
}

pub unsafe fn DoWhiteOut() {
    RunScriptImmediately(
        (*crate::asmdata::EventScript_WhiteOut.cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
    );
    SetMoney(
        &raw mut (*gSaveBlock1Ptr).money,
        GetMoney(&raw mut (*gSaveBlock1Ptr).money) / 2,
    );
    HealPlayerParty();
    Overworld_ResetStateAfterWhiteOut();
    SetWarpDestinationToLastHealLocation();
    WarpIntoMap();
}
pub unsafe fn Overworld_ResetStateAfterFly() {
    ResetInitialPlayerAvatarState();
    FlagClear(FLAG_SYS_CYCLING_ROAD);
    FlagClear(FLAG_SYS_CRUISE_MODE);
    FlagClear(FLAG_SYS_SAFARI_MODE);
    FlagClear(FLAG_SYS_USE_STRENGTH);
    FlagClear(FLAG_SYS_USE_FLASH);
}
#[unsafe(no_mangle)]
pub unsafe fn Overworld_ResetStateAfterTeleport() {
    ResetInitialPlayerAvatarState();
    FlagClear(FLAG_SYS_CYCLING_ROAD);
    FlagClear(FLAG_SYS_CRUISE_MODE);
    FlagClear(FLAG_SYS_SAFARI_MODE);
    FlagClear(FLAG_SYS_USE_STRENGTH);
    FlagClear(FLAG_SYS_USE_FLASH);
    RunScriptImmediately(
        (*crate::asmdata::EventScript_ResetMrBriney.cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
    );
}
#[unsafe(no_mangle)]
pub unsafe fn Overworld_ResetStateAfterDigEscRope() {
    ResetInitialPlayerAvatarState();
    FlagClear(FLAG_SYS_CYCLING_ROAD);
    FlagClear(FLAG_SYS_CRUISE_MODE);
    FlagClear(FLAG_SYS_SAFARI_MODE);
    FlagClear(FLAG_SYS_USE_STRENGTH);
    FlagClear(FLAG_SYS_USE_FLASH);
}
unsafe fn Overworld_ResetStateAfterWhiteOut() {
    ResetInitialPlayerAvatarState();
    FlagClear(FLAG_SYS_CYCLING_ROAD);
    FlagClear(FLAG_SYS_CRUISE_MODE);
    FlagClear(FLAG_SYS_SAFARI_MODE);
    FlagClear(FLAG_SYS_USE_STRENGTH);
    FlagClear(FLAG_SYS_USE_FLASH);
    if VarGet(VAR_SHOULD_END_ABNORMAL_WEATHER) == 1 {
        VarSet(VAR_SHOULD_END_ABNORMAL_WEATHER, 0);
        VarSet(VAR_ABNORMAL_WEATHER_LOCATION, ABNORMAL_WEATHER_NONE);
    }
}
unsafe fn UpdateMiscOverworldStates() {
    FlagClear(FLAG_SYS_SAFARI_MODE);
    ChooseAmbientCrySpecies();
    ResetCyclingRoadChallengeData();
    UpdateLocationHistoryForRoamer();
    RoamerMoveToOtherLocationSet();
}
#[unsafe(no_mangle)]
pub unsafe fn ResetGameStats() {
    for i in 0..NUM_GAME_STATS {
        SetGameStat(i as u8, 0);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn IncrementGameStat(index: u8) {
    if index < NUM_USED_GAME_STATS {
        let mut statVal: u32 = GetGameStat(index);
        if statVal < 0xFFFFFF {
            statVal += 1;
        } else {
            statVal = 0xFFFFFF;
        }
        SetGameStat(index, statVal);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn GetGameStat(index: u8) -> u32 {
    if index >= NUM_USED_GAME_STATS {
        return 0;
    }
    (*gSaveBlock1Ptr).gameStats[index] ^ (*gSaveBlock2Ptr).encryptionKey
}
#[unsafe(no_mangle)]
pub unsafe fn SetGameStat(index: u8, value: u32) {
    if index < NUM_USED_GAME_STATS {
        (*gSaveBlock1Ptr).gameStats[index] = value ^ (*gSaveBlock2Ptr).encryptionKey;
    }
}
#[unsafe(no_mangle)]
pub unsafe fn ApplyNewEncryptionKeyToGameStats(newKey: u32) {
    for i in 0..(NUM_GAME_STATS as u8) {
        ApplyNewEncryptionKeyToWord(&raw mut (*gSaveBlock1Ptr).gameStats[i], newKey);
    }
}
pub unsafe fn LoadObjEventTemplatesFromHeader() {
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
    CpuSet(
        (*gMapHeader.events).objectEvents as *mut c_void,
        (*gSaveBlock1Ptr).objectEventTemplates.as_mut_ptr() as *mut c_void,
        0x04000000 | ((*gMapHeader.events).objectEventCount as u32 * 24 / 4) & 0x1FFFFF,
    );
}
pub unsafe fn LoadSaveblockObjEventScripts() {
    let mapHeaderObjTemplates: *mut ObjectEventTemplate = (*gMapHeader.events).objectEvents;
    let savObjTemplates: *mut ObjectEventTemplate =
        (*gSaveBlock1Ptr).objectEventTemplates.as_mut_ptr();
    for i in 0..OBJECT_EVENT_TEMPLATES_COUNT {
        (*savObjTemplates.at(i)).script = (*mapHeaderObjTemplates.at(i)).script;
    }
}
pub unsafe fn SetObjEventTemplateCoords(localId: u8, x: i16, y: i16) {
    let savObjTemplates: *mut ObjectEventTemplate =
        (*gSaveBlock1Ptr).objectEventTemplates.as_mut_ptr();
    for i in 0..OBJECT_EVENT_TEMPLATES_COUNT {
        let objectEventTemplate: *mut ObjectEventTemplate = savObjTemplates.at(i);
        if (*objectEventTemplate).localId == localId {
            (*objectEventTemplate).x = x;
            (*objectEventTemplate).y = y;
            return;
        }
    }
}
pub unsafe fn SetObjEventTemplateMovementType(localId: u8, movementType: u8) {
    let savObjTemplates: *mut ObjectEventTemplate =
        (*gSaveBlock1Ptr).objectEventTemplates.as_mut_ptr();
    for i in 0..OBJECT_EVENT_TEMPLATES_COUNT {
        let objectEventTemplate: *mut ObjectEventTemplate = savObjTemplates.at(i);
        if (*objectEventTemplate).localId == localId {
            (*objectEventTemplate).movementType = movementType;
            return;
        }
    }
}
unsafe fn InitMapView() {
    ResetFieldCamera();
    CopyMapTilesetsToVram(gMapHeader.mapLayout);
    LoadMapTilesetPalettes(gMapHeader.mapLayout);
    DrawWholeMapView();
    InitTilesetAnimations();
}
pub unsafe fn GetMapLayout() -> *mut MapLayout {
    let mapLayoutId: u16 = (*gSaveBlock1Ptr).mapLayoutId;
    if mapLayoutId != 0 {
        return (*crate::asmdata::gMapLayouts.cast::<CArray<*mut MapLayout, 0>>())
            [mapLayoutId as i32 - 1];
    }
    null_mut()
}
pub unsafe fn ApplyCurrentWarp() {
    gLastUsedWarp = (*gSaveBlock1Ptr).location;
    (*gSaveBlock1Ptr).location = sWarpDestination;
    sFixedDiveWarp = *sDummyWarpData;
    sFixedHoleWarp = *sDummyWarpData;
}
unsafe fn ClearDiveAndHoleWarps() {
    sFixedDiveWarp = *sDummyWarpData;
    sFixedHoleWarp = *sDummyWarpData;
}
unsafe fn SetWarpData(warp: *mut WarpData, mapGroup: i8, mapNum: i8, warpId: i8, x: i8, y: i8) {
    (*warp).mapGroup = mapGroup;
    (*warp).mapNum = mapNum;
    (*warp).warpId = warpId;
    (*warp).x = x as i16;
    (*warp).y = y as i16;
}
unsafe fn IsDummyWarp(warp: *mut WarpData) -> u32 {
    if (*warp).mapGroup != -1 {
        return FALSE as u32;
    } else if (*warp).mapNum != -1 {
        return FALSE as u32;
    } else if (*warp).warpId != WARP_ID_NONE {
        return FALSE as u32;
    } else if (*warp).x != -1 {
        return FALSE as u32;
    } else if (*warp).y != -1 {
        return FALSE as u32;
    } else {
        return TRUE as u32;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
pub unsafe fn Overworld_GetMapHeaderByGroupAndId(mapGroup: u16, mapNum: u16) -> *mut MapHeader {
    *(*crate::asmdata::gMapGroups.cast::<CArray<*mut *mut MapHeader, 0>>())[mapGroup].at(mapNum)
}
pub unsafe fn GetDestinationWarpMapHeader() -> *mut MapHeader {
    Overworld_GetMapHeaderByGroupAndId(
        sWarpDestination.mapGroup as u16,
        sWarpDestination.mapNum as u16,
    )
}
unsafe fn LoadCurrentMapData() {
    sLastMapSectionId.set(gMapHeader.regionMapSectionId as u16);
    gMapHeader = *Overworld_GetMapHeaderByGroupAndId(
        (*gSaveBlock1Ptr).location.mapGroup as u16,
        (*gSaveBlock1Ptr).location.mapNum as u16,
    );
    (*gSaveBlock1Ptr).mapLayoutId = gMapHeader.mapLayoutId;
    gMapHeader.mapLayout = GetMapLayout();
}
unsafe fn LoadSaveblockMapHeader() {
    gMapHeader = *Overworld_GetMapHeaderByGroupAndId(
        (*gSaveBlock1Ptr).location.mapGroup as u16,
        (*gSaveBlock1Ptr).location.mapNum as u16,
    );
    gMapHeader.mapLayout = GetMapLayout();
}
unsafe fn SetPlayerCoordsFromWarp() {
    if (*gSaveBlock1Ptr).location.warpId >= 0
        && ((*gSaveBlock1Ptr).location.warpId as i32) < (*gMapHeader.events).warpCount as i32
    {
        (*gSaveBlock1Ptr).pos.x = (*(*gMapHeader.events)
            .warps
            .at((*gSaveBlock1Ptr).location.warpId))
        .x;
        (*gSaveBlock1Ptr).pos.y = (*(*gMapHeader.events)
            .warps
            .at((*gSaveBlock1Ptr).location.warpId))
        .y;
    } else if (*gSaveBlock1Ptr).location.x >= 0 && (*gSaveBlock1Ptr).location.y >= 0 {
        (*gSaveBlock1Ptr).pos.x = (*gSaveBlock1Ptr).location.x;
        (*gSaveBlock1Ptr).pos.y = (*gSaveBlock1Ptr).location.y;
    } else {
        (*gSaveBlock1Ptr).pos.x = ((*gMapHeader.mapLayout).width / 2) as i16;
        (*gSaveBlock1Ptr).pos.y = ((*gMapHeader.mapLayout).height / 2) as i16;
    }
}
#[unsafe(no_mangle)]
pub unsafe fn WarpIntoMap() {
    ApplyCurrentWarp();
    LoadCurrentMapData();
    SetPlayerCoordsFromWarp();
}
#[unsafe(no_mangle)]
pub unsafe fn SetWarpDestination(mapGroup: i8, mapNum: i8, warpId: i8, x: i8, y: i8) {
    SetWarpData(&raw mut sWarpDestination, mapGroup, mapNum, warpId, x, y);
}
pub unsafe fn SetWarpDestinationToMapWarp(mapGroup: i8, mapNum: i8, warpId: i8) {
    SetWarpDestination(mapGroup, mapNum, warpId, -1, -1);
}
#[unsafe(no_mangle)]
pub unsafe fn SetDynamicWarp(unused: i32, mapGroup: i8, mapNum: i8, warpId: i8) {
    SetWarpData(
        &raw mut (*gSaveBlock1Ptr).dynamicWarp,
        mapGroup,
        mapNum,
        warpId,
        (*gSaveBlock1Ptr).pos.x as i8,
        (*gSaveBlock1Ptr).pos.y as i8,
    );
}
pub unsafe fn SetDynamicWarpWithCoords(
    unused: i32,
    mapGroup: i8,
    mapNum: i8,
    warpId: i8,
    x: i8,
    y: i8,
) {
    SetWarpData(
        &raw mut (*gSaveBlock1Ptr).dynamicWarp,
        mapGroup,
        mapNum,
        warpId,
        x,
        y,
    );
}
#[unsafe(no_mangle)]
pub unsafe fn SetWarpDestinationToDynamicWarp(unusedWarpId: u8) {
    sWarpDestination = (*gSaveBlock1Ptr).dynamicWarp;
}
pub unsafe fn SetWarpDestinationToHealLocation(healLocationId: u8) {
    let healLocation: *mut HealLocation = GetHealLocation(healLocationId as u32);
    if !healLocation.is_null() {
        SetWarpDestination(
            (*healLocation).mapGroup,
            (*healLocation).mapNum,
            WARP_ID_NONE,
            (*healLocation).x as i8,
            (*healLocation).y as i8,
        );
    }
}
pub unsafe fn SetWarpDestinationToLastHealLocation() {
    sWarpDestination = (*gSaveBlock1Ptr).lastHealLocation;
}
pub unsafe fn SetLastHealLocationWarp(healLocationId: u8) {
    let healLocation: *mut HealLocation = GetHealLocation(healLocationId as u32);
    if !healLocation.is_null() {
        SetWarpData(
            &raw mut (*gSaveBlock1Ptr).lastHealLocation,
            (*healLocation).mapGroup,
            (*healLocation).mapNum,
            WARP_ID_NONE,
            (*healLocation).x as i8,
            (*healLocation).y as i8,
        );
    }
}
pub unsafe fn UpdateEscapeWarp(x: i16, y: i16) {
    let currMapType: u8 = GetCurrentMapType();
    let destMapType: u8 =
        GetMapTypeByGroupAndId(sWarpDestination.mapGroup, sWarpDestination.mapNum);
    if IsMapTypeOutdoors(currMapType) != 0 && IsMapTypeOutdoors(destMapType) != TRUE {
        SetEscapeWarp(
            (*gSaveBlock1Ptr).location.mapGroup,
            (*gSaveBlock1Ptr).location.mapNum,
            WARP_ID_NONE,
            x as i8 - MAP_OFFSET as i8,
            y as i8 - MAP_OFFSET as i8 + 1,
        );
    }
}
pub unsafe fn SetEscapeWarp(mapGroup: i8, mapNum: i8, warpId: i8, x: i8, y: i8) {
    SetWarpData(
        &raw mut (*gSaveBlock1Ptr).escapeWarp,
        mapGroup,
        mapNum,
        warpId,
        x,
        y,
    );
}
pub unsafe fn SetWarpDestinationToEscapeWarp() {
    sWarpDestination = (*gSaveBlock1Ptr).escapeWarp;
}
pub unsafe fn SetFixedDiveWarp(mapGroup: i8, mapNum: i8, warpId: i8, x: i8, y: i8) {
    SetWarpData(&raw mut sFixedDiveWarp, mapGroup, mapNum, warpId, x, y);
}
unsafe fn SetWarpDestinationToDiveWarp() {
    sWarpDestination = sFixedDiveWarp;
}
pub unsafe fn SetFixedHoleWarp(mapGroup: i8, mapNum: i8, warpId: i8, x: i8, y: i8) {
    SetWarpData(&raw mut sFixedHoleWarp, mapGroup, mapNum, warpId, x, y);
}
pub unsafe fn SetWarpDestinationToFixedHoleWarp(x: i16, y: i16) {
    if IsDummyWarp(&raw mut sFixedHoleWarp) == TRUE as u32 {
        sWarpDestination = gLastUsedWarp;
    } else {
        SetWarpDestination(
            sFixedHoleWarp.mapGroup,
            sFixedHoleWarp.mapNum,
            WARP_ID_NONE,
            x as i8,
            y as i8,
        );
    }
}
unsafe fn SetWarpDestinationToContinueGameWarp() {
    sWarpDestination = (*gSaveBlock1Ptr).continueGameWarp;
}
pub unsafe fn SetContinueGameWarp(mapGroup: i8, mapNum: i8, warpId: i8, x: i8, y: i8) {
    SetWarpData(
        &raw mut (*gSaveBlock1Ptr).continueGameWarp,
        mapGroup,
        mapNum,
        warpId,
        x,
        y,
    );
}
#[unsafe(no_mangle)]
pub unsafe fn SetContinueGameWarpToHealLocation(healLocationId: u8) {
    let healLocation: *mut HealLocation = GetHealLocation(healLocationId as u32);
    if !healLocation.is_null() {
        SetWarpData(
            &raw mut (*gSaveBlock1Ptr).continueGameWarp,
            (*healLocation).mapGroup,
            (*healLocation).mapNum,
            WARP_ID_NONE,
            (*healLocation).x as i8,
            (*healLocation).y as i8,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe fn SetContinueGameWarpToDynamicWarp(unused: i32) {
    (*gSaveBlock1Ptr).continueGameWarp = (*gSaveBlock1Ptr).dynamicWarp;
}
pub unsafe fn GetMapConnection(dir: u8) -> *mut MapConnection {
    let count: i32 = (*gMapHeader.connections).count;
    let mut connection: *mut MapConnection = (*gMapHeader.connections).connections;
    if connection.is_null() {
        return null_mut();
    }
    let mut i: i32 = 0;
    while i < count {
        if (*connection).direction == dir {
            return connection;
        }
        i += 1;
        connection = connection.at(1);
    }
    null_mut()
}
unsafe fn SetDiveWarp(dir: u8, x: u16, y: u16) -> u8 {
    let connection: *mut MapConnection = GetMapConnection(dir);
    if !connection.is_null() {
        SetWarpDestination(
            (*connection).mapGroup as i8,
            (*connection).mapNum as i8,
            WARP_ID_NONE,
            x as i8,
            y as i8,
        );
    } else {
        RunOnDiveWarpMapScript();
        if IsDummyWarp(&raw mut sFixedDiveWarp) != 0 {
            return FALSE;
        }
        SetWarpDestinationToDiveWarp();
    }
    TRUE
}
pub unsafe fn SetDiveWarpEmerge(x: u16, y: u16) -> u8 {
    SetDiveWarp(CONNECTION_EMERGE, x, y)
}
pub unsafe fn SetDiveWarpDive(x: u16, y: u16) -> u8 {
    SetDiveWarp(CONNECTION_DIVE, x, y)
}
pub unsafe fn LoadMapFromCameraTransition(mapGroup: u8, mapNum: u8) {
    SetWarpDestination(
        mapGroup as i8,
        mapNum as i8,
        WARP_ID_NONE,
        WARP_ID_NONE,
        WARP_ID_NONE,
    );
    if gMapHeader.regionMapSectionId != MAPSEC_BATTLE_FRONTIER {
        TransitionMapMusic();
    }
    ApplyCurrentWarp();
    LoadCurrentMapData();
    LoadObjEventTemplatesFromHeader();
    TrySetMapSaveWarpStatus();
    ClearTempFieldEventData();
    ResetCyclingRoadChallengeData();
    RestartWildEncounterImmunitySteps();
    TryUpdateRandomTrainerRematches(mapGroup as u16, mapNum as u16);
    DoTimeBasedEvents();
    SetSavedWeatherFromCurrMapHeader();
    ChooseAmbientCrySpecies();
    SetDefaultFlashLevel();
    Overworld_ClearSavedMusic();
    RunOnTransitionMapScript();
    InitMap();
    CopySecondaryTilesetToVramUsingHeap(gMapHeader.mapLayout);
    LoadSecondaryTilesetPalette(gMapHeader.mapLayout);
    for paletteIndex in NUM_PALS_IN_PRIMARY..NUM_PALS_TOTAL {
        ApplyWeatherColorMapToPal(paletteIndex as u8);
    }
    InitSecondaryTilesetAnimation();
    UpdateLocationHistoryForRoamer();
    RoamerMove();
    DoCurrentWeather();
    ResetFieldTasksArgs();
    RunOnResumeMapScript();
    if gMapHeader.regionMapSectionId != MAPSEC_BATTLE_FRONTIER
        || gMapHeader.regionMapSectionId as u16 != sLastMapSectionId.get()
    {
        ShowMapNamePopup();
    }
}
unsafe fn LoadMapFromWarp(a1: u32) {
    LoadCurrentMapData();
    if sObjectEventLoadFlag.get() as i32 & SKIP_OBJECT_EVENT_LOAD as i32 == 0 {
        if gMapHeader.mapLayoutId == LAYOUT_BATTLE_FRONTIER_BATTLE_PYRAMID_FLOOR {
            LoadBattlePyramidObjectEventTemplates();
        } else if InTrainerHill() != 0 {
            LoadTrainerHillObjectEventTemplates();
        } else {
            LoadObjEventTemplatesFromHeader();
        }
    }
    let isOutdoors: u8 = IsMapTypeOutdoors(gMapHeader.mapType);
    let isIndoors: u8 = IsMapTypeIndoors(gMapHeader.mapType);
    CheckLeftFriendsSecretBase();
    TrySetMapSaveWarpStatus();
    ClearTempFieldEventData();
    ResetCyclingRoadChallengeData();
    RestartWildEncounterImmunitySteps();
    TryUpdateRandomTrainerRematches(
        (*gSaveBlock1Ptr).location.mapGroup as u16,
        (*gSaveBlock1Ptr).location.mapNum as u16,
    );
    if a1 != TRUE as u32 {
        DoTimeBasedEvents();
    }
    SetSavedWeatherFromCurrMapHeader();
    ChooseAmbientCrySpecies();
    if isOutdoors != 0 {
        FlagClear(FLAG_SYS_USE_FLASH);
    }
    SetDefaultFlashLevel();
    Overworld_ClearSavedMusic();
    RunOnTransitionMapScript();
    UpdateLocationHistoryForRoamer();
    RoamerMoveToOtherLocationSet();
    if gMapHeader.mapLayoutId == LAYOUT_BATTLE_FRONTIER_BATTLE_PYRAMID_FLOOR {
        InitBattlePyramidMap(FALSE);
    } else if InTrainerHill() != 0 {
        InitTrainerHillMap();
    } else {
        InitMap();
    }
    if a1 != TRUE as u32 && isIndoors != 0 {
        UpdateTVScreensOnMap(gBackupMapLayout.width, gBackupMapLayout.height);
        InitSecretBaseAppearance(TRUE);
    }
}
pub unsafe fn ResetInitialPlayerAvatarState() {
    sInitialPlayerAvatarState.direction = DIR_SOUTH;
    sInitialPlayerAvatarState.transitionFlags = PLAYER_AVATAR_FLAG_ON_FOOT;
}
pub unsafe fn StoreInitialPlayerAvatarState() {
    sInitialPlayerAvatarState.direction = GetPlayerFacingDirection();
    if TestPlayerAvatarFlags(PLAYER_AVATAR_FLAG_MACH_BIKE) != 0 {
        sInitialPlayerAvatarState.transitionFlags = PLAYER_AVATAR_FLAG_MACH_BIKE;
    } else if TestPlayerAvatarFlags(PLAYER_AVATAR_FLAG_ACRO_BIKE) != 0 {
        sInitialPlayerAvatarState.transitionFlags = PLAYER_AVATAR_FLAG_ACRO_BIKE;
    } else if TestPlayerAvatarFlags(PLAYER_AVATAR_FLAG_SURFING) != 0 {
        sInitialPlayerAvatarState.transitionFlags = PLAYER_AVATAR_FLAG_SURFING;
    } else if TestPlayerAvatarFlags(PLAYER_AVATAR_FLAG_UNDERWATER) != 0 {
        sInitialPlayerAvatarState.transitionFlags = PLAYER_AVATAR_FLAG_UNDERWATER;
    } else {
        sInitialPlayerAvatarState.transitionFlags = PLAYER_AVATAR_FLAG_ON_FOOT;
    }
}
unsafe fn GetInitialPlayerAvatarState() -> *mut InitialPlayerAvatarState {
    let mut playerStruct: InitialPlayerAvatarState = zeroed();
    let mapType: u8 = GetCurrentMapType();
    let metatileBehavior: u16 = GetCenterScreenMetatileBehavior();
    let transitionFlags: u8 = GetAdjustedInitialTransitionFlags(
        &raw mut sInitialPlayerAvatarState,
        metatileBehavior,
        mapType,
    );
    playerStruct.transitionFlags = transitionFlags;
    playerStruct.direction = GetAdjustedInitialDirection(
        &raw mut sInitialPlayerAvatarState,
        transitionFlags,
        metatileBehavior,
        mapType,
    );
    sInitialPlayerAvatarState = playerStruct;
    &raw mut sInitialPlayerAvatarState
}
unsafe fn GetAdjustedInitialTransitionFlags(
    playerStruct: *mut InitialPlayerAvatarState,
    metatileBehavior: u16,
    mapType: u8,
) -> u8 {
    if mapType != MAP_TYPE_INDOOR && FlagGet(FLAG_SYS_CRUISE_MODE) != 0 {
        return PLAYER_AVATAR_FLAG_ON_FOOT;
    } else if mapType == MAP_TYPE_UNDERWATER {
        return PLAYER_AVATAR_FLAG_UNDERWATER;
    } else if MetatileBehavior_IsSurfableWaterOrUnderwater(metatileBehavior as u8) == TRUE {
        return PLAYER_AVATAR_FLAG_SURFING;
    } else if Overworld_IsBikingAllowed() != TRUE as u32 {
        return PLAYER_AVATAR_FLAG_ON_FOOT;
    } else if (*playerStruct).transitionFlags == PLAYER_AVATAR_FLAG_MACH_BIKE {
        return PLAYER_AVATAR_FLAG_MACH_BIKE;
    } else if (*playerStruct).transitionFlags != PLAYER_AVATAR_FLAG_ACRO_BIKE {
        return PLAYER_AVATAR_FLAG_ON_FOOT;
    } else {
        return PLAYER_AVATAR_FLAG_ACRO_BIKE;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
unsafe fn GetAdjustedInitialDirection(
    playerStruct: *mut InitialPlayerAvatarState,
    transitionFlags: u8,
    metatileBehavior: u16,
    mapType: u8,
) -> u8 {
    if FlagGet(FLAG_SYS_CRUISE_MODE) != 0 && mapType == MAP_TYPE_OCEAN_ROUTE {
        return DIR_EAST;
    } else if MetatileBehavior_IsDeepSouthWarp(metatileBehavior as u8) == TRUE {
        return DIR_NORTH;
    } else if MetatileBehavior_IsNonAnimDoor(metatileBehavior as u8) == TRUE
        || MetatileBehavior_IsDoor(metatileBehavior as u8) == TRUE
    {
        return DIR_SOUTH;
    } else if MetatileBehavior_IsSouthArrowWarp(metatileBehavior as u8) == TRUE {
        return DIR_NORTH;
    } else if MetatileBehavior_IsNorthArrowWarp(metatileBehavior as u8) == TRUE {
        return DIR_SOUTH;
    } else if MetatileBehavior_IsWestArrowWarp(metatileBehavior as u8) == TRUE {
        return DIR_EAST;
    } else if MetatileBehavior_IsEastArrowWarp(metatileBehavior as u8) == TRUE {
        return DIR_WEST;
    } else if (*playerStruct).transitionFlags == PLAYER_AVATAR_FLAG_UNDERWATER
        && transitionFlags == PLAYER_AVATAR_FLAG_SURFING
        || (*playerStruct).transitionFlags == PLAYER_AVATAR_FLAG_SURFING
            && transitionFlags == PLAYER_AVATAR_FLAG_UNDERWATER
    {
        return (*playerStruct).direction;
    } else if MetatileBehavior_IsLadder(metatileBehavior as u8) == TRUE {
        return (*playerStruct).direction;
    } else {
        return DIR_SOUTH;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
unsafe fn GetCenterScreenMetatileBehavior() -> u16 {
    MapGridGetMetatileBehaviorAt(
        (*gSaveBlock1Ptr).pos.x as i32 + MAP_OFFSET,
        (*gSaveBlock1Ptr).pos.y as i32 + MAP_OFFSET,
    ) as u16
}
pub unsafe fn Overworld_IsBikingAllowed() -> u32 {
    if gMapHeader.allowCycling() == 0 {
        return FALSE as u32;
    } else {
        return TRUE as u32;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
pub unsafe fn SetDefaultFlashLevel() {
    if gMapHeader.cave == 0 {
        (*gSaveBlock1Ptr).flashLevel = 0;
    } else if FlagGet(FLAG_SYS_USE_FLASH) != 0 {
        (*gSaveBlock1Ptr).flashLevel = 1;
    } else {
        (*gSaveBlock1Ptr).flashLevel =
            (*(&raw const crate::data::field_screen_effect::gMaxFlashLevel).cast::<i32>()) as u8
                - 1;
    }
}
pub unsafe fn SetFlashLevel(mut flashLevel: i32) {
    if flashLevel < 0
        || flashLevel
            > (*(&raw const crate::data::field_screen_effect::gMaxFlashLevel).cast::<i32>())
    {
        flashLevel = 0;
    }
    (*gSaveBlock1Ptr).flashLevel = flashLevel as u8;
}
pub unsafe fn GetFlashLevel() -> u8 {
    (*gSaveBlock1Ptr).flashLevel
}
pub unsafe fn SetCurrentMapLayout(mapLayoutId: u16) {
    (*gSaveBlock1Ptr).mapLayoutId = mapLayoutId;
    gMapHeader.mapLayout = GetMapLayout();
}
pub fn SetObjectEventLoadFlag(flag: u8) {
    sObjectEventLoadFlag.set(flag);
}
fn GetObjectEventLoadFlag() -> u8 {
    sObjectEventLoadFlag.get()
}
unsafe fn ShouldLegendaryMusicPlayAtLocation(warp: *mut WarpData) -> u16 {
    if FlagGet(FLAG_SYS_WEATHER_CTRL) == 0 {
        return FALSE as u16;
    }
    if (*warp).mapGroup == 0 {
        match (*warp).mapNum {
            5 | 6 | 7 | 8 | 39 | 40 | 41 | 42 | 43 => {
                return TRUE as u16;
            }
            _ => {
                if VarGet(VAR_SOOTOPOLIS_CITY_STATE) < 4 {
                    return FALSE as u16;
                }
                if let 44..=46 = (*warp).mapNum {
                    return TRUE as u16;
                }
            }
        }
    }
    FALSE as u16
}
unsafe fn NoMusicInSootopolisWithLegendaries(warp: *mut WarpData) -> u16 {
    if VarGet(VAR_SKY_PILLAR_STATE) != 1 {
        return FALSE as u16;
    } else if (*warp).mapGroup != 0 {
        return FALSE as u16;
    } else if (*warp).mapNum == 7 {
        return TRUE as u16;
    } else {
        return FALSE as u16;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
unsafe fn IsInfiltratedWeatherInstitute(warp: *mut WarpData) -> u16 {
    if VarGet(VAR_WEATHER_INSTITUTE_STATE) != 0 {
        return FALSE as u16;
    } else if (*warp).mapGroup != 32 {
        return FALSE as u16;
    } else if (*warp).mapNum == 0 || (*warp).mapNum == 1 {
        return TRUE as u16;
    } else {
        return FALSE as u16;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
unsafe fn IsInfiltratedSpaceCenter(warp: *mut WarpData) -> u16 {
    if VarGet(VAR_MOSSDEEP_CITY_STATE) == 0 {
        return FALSE as u16;
    } else if VarGet(VAR_MOSSDEEP_CITY_STATE) > 2 {
        return FALSE as u16;
    } else if (*warp).mapGroup != 14 {
        return FALSE as u16;
    } else if (*warp).mapNum == 9 || (*warp).mapNum == 10 {
        return TRUE as u16;
    }
    FALSE as u16
}
pub unsafe fn GetLocationMusic(warp: *mut WarpData) -> u16 {
    if NoMusicInSootopolisWithLegendaries(warp) == TRUE as u16 {
        return MUS_NONE;
    } else if ShouldLegendaryMusicPlayAtLocation(warp) == TRUE as u16 {
        return MUS_ABNORMAL_WEATHER;
    } else if IsInfiltratedSpaceCenter(warp) == TRUE as u16 {
        return MUS_ENCOUNTER_MAGMA;
    } else if IsInfiltratedWeatherInstitute(warp) == TRUE as u16 {
        return MUS_MT_CHIMNEY;
    } else {
        return (*Overworld_GetMapHeaderByGroupAndId(
            (*warp).mapGroup as u16,
            (*warp).mapNum as u16,
        ))
        .music;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
pub unsafe fn GetCurrLocationDefaultMusic() -> u16 {
    if (*gSaveBlock1Ptr).location.mapGroup == 0
        && (*gSaveBlock1Ptr).location.mapNum == 26
        && GetSavedWeather() == WEATHER_SANDSTORM
    {
        return MUS_DESERT;
    }
    let music: u16 = GetLocationMusic(&raw mut (*gSaveBlock1Ptr).location);
    if music != MUS_ROUTE118 {
        return music;
    } else {
        if (*gSaveBlock1Ptr).pos.x < 24 {
            return MUS_ROUTE110;
        } else {
            return MUS_ROUTE119;
        }
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
pub unsafe fn GetWarpDestinationMusic() -> u16 {
    let music: u16 = GetLocationMusic(&raw mut sWarpDestination);
    if music != MUS_ROUTE118 {
        return music;
    } else {
        if (*gSaveBlock1Ptr).location.mapGroup == 0 && (*gSaveBlock1Ptr).location.mapNum == 2 {
            return MUS_ROUTE110;
        } else {
            return MUS_ROUTE119;
        }
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
pub fn Overworld_ResetMapMusic() {
    ResetMapMusic();
}
#[unsafe(no_mangle)]
pub unsafe fn Overworld_PlaySpecialMapMusic() {
    let mut music: u16 = GetCurrLocationDefaultMusic();
    if music != MUS_ABNORMAL_WEATHER && music != MUS_NONE {
        if (*gSaveBlock1Ptr).savedMusic != 0 {
            music = (*gSaveBlock1Ptr).savedMusic;
        } else if GetCurrentMapType() == MAP_TYPE_UNDERWATER {
            music = MUS_UNDERWATER;
        } else if TestPlayerAvatarFlags(PLAYER_AVATAR_FLAG_SURFING) != 0 {
            music = MUS_SURF;
        }
    }
    if music != GetCurrentMapMusic() {
        PlayNewMapMusic(music);
    }
}
pub unsafe fn Overworld_SetSavedMusic(songNum: u16) {
    (*gSaveBlock1Ptr).savedMusic = songNum;
}
pub unsafe fn Overworld_ClearSavedMusic() {
    (*gSaveBlock1Ptr).savedMusic = MUS_DUMMY;
}
unsafe fn TransitionMapMusic() {
    if FlagGet(FLAG_DONT_TRANSITION_MUSIC) != TRUE {
        let mut newMusic: u16 = GetWarpDestinationMusic();
        let currentMusic: u16 = GetCurrentMapMusic();
        if newMusic != MUS_ABNORMAL_WEATHER && newMusic != MUS_NONE {
            if currentMusic == MUS_UNDERWATER || currentMusic == MUS_SURF {
                return;
            }
            if TestPlayerAvatarFlags(PLAYER_AVATAR_FLAG_SURFING) != 0 {
                newMusic = MUS_SURF;
            }
        }
        if newMusic != currentMusic {
            if TestPlayerAvatarFlags(6) != 0 {
                FadeOutAndFadeInNewMapMusic(newMusic, 4, 4);
            } else {
                FadeOutAndPlayNewMapMusic(newMusic, 8);
            }
        }
    }
}
pub unsafe fn Overworld_ChangeMusicToDefault() {
    let currentMusic: u16 = GetCurrentMapMusic();
    if currentMusic != GetCurrLocationDefaultMusic() {
        FadeOutAndPlayNewMapMusic(GetCurrLocationDefaultMusic(), 8);
    }
}
pub unsafe fn Overworld_ChangeMusicTo(newMusic: u16) {
    let currentMusic: u16 = GetCurrentMapMusic();
    if currentMusic != newMusic && currentMusic != MUS_ABNORMAL_WEATHER {
        FadeOutAndPlayNewMapMusic(newMusic, 8);
    }
}
pub unsafe fn GetMapMusicFadeoutSpeed() -> u8 {
    let mapHeader: *mut MapHeader = GetDestinationWarpMapHeader();
    if IsMapTypeIndoors((*mapHeader).mapType) == TRUE {
        return 2;
    } else {
        return 4;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
pub unsafe fn TryFadeOutOldMapMusic() {
    let currentMusic: u16 = GetCurrentMapMusic();
    let warpMusic: u16 = GetWarpDestinationMusic();
    if FlagGet(FLAG_DONT_TRANSITION_MUSIC) != TRUE && warpMusic != GetCurrentMapMusic() {
        if currentMusic == MUS_SURF
            && VarGet(VAR_SKY_PILLAR_STATE) == 2
            && (*gSaveBlock1Ptr).location.mapGroup == 0
            && (*gSaveBlock1Ptr).location.mapNum == 7
            && sWarpDestination.mapGroup == 0
            && sWarpDestination.mapNum == 7
            && sWarpDestination.x == 29
            && sWarpDestination.y == 53
        {
            return;
        }
        FadeOutMapMusic(GetMapMusicFadeoutSpeed());
    }
}
pub unsafe fn BGMusicStopped() -> u8 {
    IsNotWaitingForBGMStop()
}
pub unsafe fn Overworld_FadeOutMapMusic() {
    FadeOutMapMusic(4);
}
unsafe fn PlayAmbientCry() {
    let mut x: i16 = 0;
    let mut y: i16 = 0;
    PlayerGetDestCoords(&raw mut x, &raw mut y);
    if sIsAmbientCryWaterMon.get() == TRUE
        && MetatileBehavior_IsSurfableWaterOrUnderwater(MapGridGetMetatileBehaviorAt(
            x as i32, y as i32,
        ) as u8)
            == 0
    {
        return;
    }
    let pan: i8 = (Random() as i32 % 88) as i8 + -44;
    let volume: i8 = (Random() as i32 % 30) as i8 + 50;
    PlayCry_NormalNoDucking(sAmbientCrySpecies.get(), pan, volume, CRY_PRIORITY_AMBIENT);
}
pub unsafe fn UpdateAmbientCry(state: *mut i16, delayCounter: *mut u16) {
    let mut monsCount: u8 = 0;
    let mut divBy: u8 = 0;
    match *state {
        AMB_CRY_INIT => {
            if sAmbientCrySpecies.get() == SPECIES_NONE {
                *state = AMB_CRY_IDLE;
            } else {
                *state = AMB_CRY_FIRST;
            }
        }
        AMB_CRY_FIRST => {
            *delayCounter = (Random() as i32 % 2400) as u16 + 1200;
            *state = AMB_CRY_WAIT;
        }
        AMB_CRY_RESET => {
            divBy = 1;
            monsCount = CalculatePlayerPartyCount();
            for i in 0..monsCount {
                if GetMonData2(&raw mut gPlayerParty[i], MON_DATA_SANITY_IS_EGG) == 0
                    && GetMonAbility(&raw mut gPlayerParty[0]) == ABILITY_SWARM
                {
                    divBy = 2;
                    break;
                }
            }
            *delayCounter = div_i32(Random() as i32 % 1200 + 1200, divBy as i32) as u16;
            *state = AMB_CRY_WAIT;
        }
        AMB_CRY_WAIT => {
            if ({
                *delayCounter -= 1;
                *delayCounter
            }) == 0
            {
                PlayAmbientCry();
                *state = AMB_CRY_RESET;
            }
        }
        AMB_CRY_IDLE => {}
        _ => {}
    }
}
unsafe fn ChooseAmbientCrySpecies() {
    if (*gSaveBlock1Ptr).location.mapGroup == 0
        && (*gSaveBlock1Ptr).location.mapNum == 45
        && IsMirageIslandPresent() == 0
    {
        sIsAmbientCryWaterMon.set(TRUE);
        sAmbientCrySpecies.set(GetLocalWaterMon());
    } else {
        sAmbientCrySpecies.set(GetLocalWildMon(sIsAmbientCryWaterMon.as_ptr()));
    }
}
pub unsafe fn GetMapTypeByGroupAndId(mapGroup: i8, mapNum: i8) -> u8 {
    (*Overworld_GetMapHeaderByGroupAndId(mapGroup as u16, mapNum as u16)).mapType
}
pub unsafe fn GetMapTypeByWarpData(warp: *mut WarpData) -> u8 {
    GetMapTypeByGroupAndId((*warp).mapGroup, (*warp).mapNum)
}
pub unsafe fn GetCurrentMapType() -> u8 {
    GetMapTypeByWarpData(&raw mut (*gSaveBlock1Ptr).location)
}
#[unsafe(no_mangle)]
pub unsafe fn GetLastUsedWarpMapType() -> u8 {
    GetMapTypeByWarpData(&raw mut gLastUsedWarp)
}
#[unsafe(no_mangle)]
pub unsafe fn IsMapTypeOutdoors(mapType: u8) -> u8 {
    if mapType == MAP_TYPE_ROUTE
        || mapType == MAP_TYPE_TOWN
        || mapType == MAP_TYPE_UNDERWATER
        || mapType == MAP_TYPE_CITY
        || mapType == MAP_TYPE_OCEAN_ROUTE
    {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
#[unsafe(no_mangle)]
pub unsafe fn Overworld_MapTypeAllowsTeleportAndFly(mapType: u8) -> u8 {
    if mapType == MAP_TYPE_ROUTE
        || mapType == MAP_TYPE_TOWN
        || mapType == MAP_TYPE_OCEAN_ROUTE
        || mapType == MAP_TYPE_CITY
    {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
pub unsafe fn IsMapTypeIndoors(mapType: u8) -> u8 {
    if mapType == MAP_TYPE_INDOOR || mapType == MAP_TYPE_SECRET_BASE {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
pub unsafe fn GetSavedWarpRegionMapSectionId() -> u8 {
    (*Overworld_GetMapHeaderByGroupAndId(
        (*gSaveBlock1Ptr).dynamicWarp.mapGroup as u16,
        (*gSaveBlock1Ptr).dynamicWarp.mapNum as u16,
    ))
    .regionMapSectionId
}
pub unsafe fn GetCurrentRegionMapSectionId() -> u8 {
    (*Overworld_GetMapHeaderByGroupAndId(
        (*gSaveBlock1Ptr).location.mapGroup as u16,
        (*gSaveBlock1Ptr).location.mapNum as u16,
    ))
    .regionMapSectionId
}
pub unsafe fn GetCurrentMapBattleScene() -> u8 {
    (*Overworld_GetMapHeaderByGroupAndId(
        (*gSaveBlock1Ptr).location.mapGroup as u16,
        (*gSaveBlock1Ptr).location.mapNum as u16,
    ))
    .battleType
}
unsafe fn InitOverworldBgs() {
    InitBgsFromTemplates(0, sOverworldBgTemplates.as_ptr().cast_mut(), 4);
    SetBgAttribute(1, BG_ATTR_MOSAIC, 1);
    SetBgAttribute(2, BG_ATTR_MOSAIC, 1);
    SetBgAttribute(3, BG_ATTR_MOSAIC, 1);
    gOverworldTilemapBuffer_Bg1 = AllocZeroed(BG_SCREEN_SIZE) as *mut u16;
    gOverworldTilemapBuffer_Bg2 = AllocZeroed(BG_SCREEN_SIZE) as *mut u16;
    gOverworldTilemapBuffer_Bg3 = AllocZeroed(BG_SCREEN_SIZE) as *mut u16;
    SetBgTilemapBuffer(1, gOverworldTilemapBuffer_Bg1 as *mut c_void);
    SetBgTilemapBuffer(2, gOverworldTilemapBuffer_Bg2 as *mut c_void);
    SetBgTilemapBuffer(3, gOverworldTilemapBuffer_Bg3 as *mut c_void);
    InitStandardTextBoxWindows();
}
pub unsafe fn CleanupOverworldWindowsAndTilemaps() {
    ClearMirageTowerPulseBlendEffect();
    FreeAllOverworldWindowBuffers();
    if !gOverworldTilemapBuffer_Bg3.is_null() {
        Free(gOverworldTilemapBuffer_Bg3 as *mut c_void);
        gOverworldTilemapBuffer_Bg3 = null_mut();
    }
    if !gOverworldTilemapBuffer_Bg2.is_null() {
        Free(gOverworldTilemapBuffer_Bg2 as *mut c_void);
        gOverworldTilemapBuffer_Bg2 = null_mut();
    }
    if !gOverworldTilemapBuffer_Bg1.is_null() {
        Free(gOverworldTilemapBuffer_Bg1 as *mut c_void);
        gOverworldTilemapBuffer_Bg1 = null_mut();
    }
}
unsafe fn ResetSafariZoneFlag_() {
    ResetSafariZoneFlag();
}
pub unsafe fn IsOverworldLinkActive() -> u32 {
    if gMain.callback1 == Some(CB1_OverworldLink as unsafe fn()) {
        return TRUE as u32;
    } else {
        return FALSE as u32;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
unsafe fn DoCB1_Overworld(newKeys: u16, heldKeys: u16) {
    let mut inputStruct: FieldInput = zeroed();
    UpdatePlayerAvatarTransitionState();
    FieldClearPlayerInput(&raw mut inputStruct);
    FieldGetPlayerInput(&raw mut inputStruct, newKeys, heldKeys);
    if ArePlayerFieldControlsLocked() == 0 {
        if ProcessPlayerFieldInput(&raw mut inputStruct) == 1 {
            LockPlayerFieldControls();
            HideMapNamePopUpWindow();
        } else {
            PlayerStep(inputStruct.dpadDirection, newKeys, heldKeys);
        }
    }
}
pub unsafe fn CB1_Overworld() {
    if gMain.callback2 == Some(CB2_Overworld as unsafe fn()) {
        DoCB1_Overworld(gMain.newKeys, gMain.heldKeys);
    }
}
unsafe fn OverworldBasic() {
    ScriptContext_RunScript();
    RunTasks();
    AnimateSprites();
    CameraUpdate();
    UpdateCameraPanning();
    BuildOamBuffer();
    UpdatePaletteFade();
    UpdateTilesetAnimations();
    DoScheduledBgTilemapCopiesToVram();
}
pub unsafe fn CB2_OverworldBasic() {
    OverworldBasic();
}
pub unsafe fn CB2_Overworld() {
    let fading: u32 = (gPaletteFade.active() != 0) as u32;
    if fading != 0 {
        SetVBlankCallback(None);
    }
    OverworldBasic();
    if fading != 0 {
        SetFieldVBlankCallback();
    }
}
pub unsafe fn SetMainCallback1(cb: Option<unsafe fn()>) {
    gMain.callback1 = cb;
}
pub unsafe fn SetUnusedCallback(func: *mut c_void) {
    sUnusedOverworldCallback = func;
}
unsafe fn RunFieldCallback() -> u8 {
    if gFieldCallback2.is_some() {
        if gFieldCallback2.unwrap_unchecked()() == 0 {
            return FALSE;
        } else {
            gFieldCallback2 = None;
            gFieldCallback = None;
        }
    } else {
        if gFieldCallback.is_some() {
            gFieldCallback.unwrap_unchecked()();
        } else {
            FieldCB_DefaultWarpExit();
        }
        gFieldCallback = None;
    }
    TRUE
}
pub unsafe fn CB2_NewGame() {
    FieldClearVBlankHBlankCallbacks();
    StopMapMusic();
    ResetSafariZoneFlag_();
    NewGameInitData();
    ResetInitialPlayerAvatarState();
    PlayTimeCounter_Start();
    ScriptContext_Init();
    UnlockPlayerFieldControls();
    gFieldCallback = Some(ExecuteTruckSequence);
    gFieldCallback2 = None;
    DoMapLoadLoop(&raw mut gMain.state);
    SetFieldVBlankCallback();
    SetMainCallback1(Some(CB1_Overworld));
    SetMainCallback2(Some(CB2_Overworld));
}
#[unsafe(no_mangle)]
pub unsafe fn CB2_WhiteOut() {
    let mut state: u8 = 0;
    if ({
        gMain.state += 1;
        gMain.state
    }) >= 120
    {
        FieldClearVBlankHBlankCallbacks();
        StopMapMusic();
        ResetSafariZoneFlag_();
        DoWhiteOut();
        ResetInitialPlayerAvatarState();
        ScriptContext_Init();
        UnlockPlayerFieldControls();
        gFieldCallback = Some(FieldCB_WarpExitFadeFromBlack);
        state = 0;
        DoMapLoadLoop(&raw mut state);
        SetFieldVBlankCallback();
        SetMainCallback1(Some(CB1_Overworld));
        SetMainCallback2(Some(CB2_Overworld));
    }
}
#[unsafe(no_mangle)]
pub unsafe fn CB2_LoadMap() {
    FieldClearVBlankHBlankCallbacks();
    ScriptContext_Init();
    UnlockPlayerFieldControls();
    SetMainCallback1(None);
    SetMainCallback2(Some(CB2_DoChangeMap));
    gMain.savedCallback = Some(CB2_LoadMap2);
}
pub(crate) unsafe fn CB2_LoadMap2() {
    DoMapLoadLoop(&raw mut gMain.state);
    SetFieldVBlankCallback();
    SetMainCallback1(Some(CB1_Overworld));
    SetMainCallback2(Some(CB2_Overworld));
}
pub unsafe fn CB2_ReturnToFieldContestHall() {
    if gMain.state == 0 {
        FieldClearVBlankHBlankCallbacks();
        ScriptContext_Init();
        UnlockPlayerFieldControls();
        SetMainCallback1(None);
    }
    if LoadMapInStepsLocal(&raw mut gMain.state, TRUE as u32) != 0 {
        SetFieldVBlankCallback();
        SetMainCallback1(Some(CB1_Overworld));
        SetMainCallback2(Some(CB2_Overworld));
    }
}
pub unsafe fn CB2_ReturnToFieldCableClub() {
    FieldClearVBlankHBlankCallbacks();
    gFieldCallback = Some(FieldCB_ReturnToFieldWirelessLink);
    SetMainCallback2(Some(CB2_LoadMapOnReturnToFieldCableClub));
}
pub(crate) unsafe fn CB2_LoadMapOnReturnToFieldCableClub() {
    if LoadMapInStepsLink(&raw mut gMain.state) != 0 {
        SetFieldVBlankCallback();
        SetMainCallback1(Some(CB1_OverworldLink));
        ResetAllMultiplayerState();
        SetMainCallback2(Some(CB2_Overworld));
    }
}
#[unsafe(no_mangle)]
pub unsafe fn CB2_ReturnToField() {
    if IsOverworldLinkActive() == TRUE as u32 {
        SetMainCallback2(Some(CB2_ReturnToFieldLink));
    } else {
        FieldClearVBlankHBlankCallbacks();
        SetMainCallback2(Some(CB2_ReturnToFieldLocal));
    }
}
pub(crate) unsafe fn CB2_ReturnToFieldLocal() {
    if ReturnToFieldLocal(&raw mut gMain.state) != 0 {
        SetFieldVBlankCallback();
        SetMainCallback2(Some(CB2_Overworld));
    }
}
pub(crate) unsafe fn CB2_ReturnToFieldLink() {
    if Overworld_IsRecvQueueAtMax() == 0 && ReturnToFieldLink(&raw mut gMain.state) != 0 {
        SetMainCallback2(Some(CB2_Overworld));
    }
}
pub unsafe fn CB2_ReturnToFieldFromMultiplayer() {
    FieldClearVBlankHBlankCallbacks();
    StopMapMusic();
    SetMainCallback1(Some(CB1_OverworldLink));
    ResetAllMultiplayerState();
    if gWirelessCommType != 0 {
        gFieldCallback = Some(FieldCB_ReturnToFieldWirelessLink);
    } else {
        gFieldCallback = Some(FieldCB_ReturnToFieldCableLink);
    }
    ScriptContext_Init();
    UnlockPlayerFieldControls();
    CB2_ReturnToField();
}
pub unsafe fn CB2_ReturnToFieldWithOpenMenu() {
    FieldClearVBlankHBlankCallbacks();
    gFieldCallback2 = Some(FieldCB_ReturnToFieldOpenStartMenu);
    CB2_ReturnToField();
}
pub unsafe fn CB2_ReturnToFieldContinueScript() {
    FieldClearVBlankHBlankCallbacks();
    gFieldCallback = Some(FieldCB_ContinueScript);
    CB2_ReturnToField();
}
#[unsafe(no_mangle)]
pub unsafe fn CB2_ReturnToFieldContinueScriptPlayMapMusic() {
    FieldClearVBlankHBlankCallbacks();
    gFieldCallback = Some(FieldCB_ContinueScriptHandleMusic);
    CB2_ReturnToField();
}
#[unsafe(no_mangle)]
pub unsafe fn CB2_ReturnToFieldFadeFromBlack() {
    FieldClearVBlankHBlankCallbacks();
    gFieldCallback = Some(FieldCB_WarpExitFadeFromBlack);
    CB2_ReturnToField();
}
pub(crate) unsafe fn FieldCB_FadeTryShowMapPopup() {
    if gMapHeader.showMapName() == TRUE && SecretBaseMapPopupEnabled() == TRUE {
        ShowMapNamePopup();
    }
    FieldCB_WarpExitFadeFromBlack();
}
#[unsafe(no_mangle)]
pub unsafe fn CB2_ContinueSavedGame() {
    FieldClearVBlankHBlankCallbacks();
    StopMapMusic();
    ResetSafariZoneFlag_();
    if gSaveFileStatus == SAVE_STATUS_ERROR as u16 {
        ResetWinStreaks();
    }
    LoadSaveblockMapHeader();
    ClearDiveAndHoleWarps();
    let trainerHillMapId: u8 = GetCurrentTrainerHillMapId();
    if gMapHeader.mapLayoutId == LAYOUT_BATTLE_FRONTIER_BATTLE_PYRAMID_FLOOR {
        LoadBattlePyramidFloorObjectEventScripts();
    } else if trainerHillMapId != 0 && trainerHillMapId != TRAINER_HILL_ENTRANCE {
        LoadTrainerHillFloorObjectEventScripts();
    } else {
        LoadSaveblockObjEventScripts();
    }
    UnfreezeObjectEvents();
    DoTimeBasedEvents();
    UpdateMiscOverworldStates();
    if gMapHeader.mapLayoutId == LAYOUT_BATTLE_FRONTIER_BATTLE_PYRAMID_FLOOR {
        InitBattlePyramidMap(TRUE);
    } else if trainerHillMapId != 0 {
        InitTrainerHillMap();
    } else {
        InitMapFromSavedGame();
    }
    PlayTimeCounter_Start();
    ScriptContext_Init();
    UnlockPlayerFieldControls();
    InitMatchCallCounters();
    if UseContinueGameWarp() == TRUE as u32 {
        ClearContinueGameWarpStatus();
        SetWarpDestinationToContinueGameWarp();
        WarpIntoMap();
        TryPutTodaysRivalTrainerOnAir();
        SetMainCallback2(Some(CB2_LoadMap));
    } else {
        TryPutTodaysRivalTrainerOnAir();
        gFieldCallback = Some(FieldCB_FadeTryShowMapPopup);
        SetMainCallback1(Some(CB1_Overworld));
        CB2_ReturnToField();
    }
}
unsafe fn FieldClearVBlankHBlankCallbacks() {
    if UsedPokemonCenterWarp() == TRUE {
        CloseLink();
    }
    if gWirelessCommType != 0 {
        EnableInterrupts(197);
        DisableInterrupts(INTR_FLAG_HBLANK);
    } else {
        let savedIme: u16 = (67109384_usize as *mut u16).read_volatile();
        volatile_write(67109384_usize as *mut u16, 0);
        volatile_write(
            0x4000200_usize as *mut u16,
            (0x4000200_usize as *mut u16).read_volatile() & 65533,
        );
        volatile_write(
            0x4000200_usize as *mut u16,
            (0x4000200_usize as *mut u16).read_volatile() | INTR_FLAG_VBLANK,
        );
        volatile_write(67109384_usize as *mut u16, savedIme);
    }
    SetVBlankCallback(None);
    SetHBlankCallback(None);
}
unsafe fn SetFieldVBlankCallback() {
    SetVBlankCallback(Some(VBlankCB_Field));
}
pub(crate) unsafe fn VBlankCB_Field() {
    LoadOam();
    ProcessSpriteCopyRequests();
    ScanlineEffect_InitHBlankDmaTransfer();
    FieldUpdateBgTilemapScroll();
    TransferPlttBuffer();
    TransferTilesetAnimsBuffer();
}
unsafe fn InitCurrentFlashLevelScanlineEffect() {
    let mut flashLevel: u8 = 0;
    if InBattlePyramid_() != 0 {
        WriteBattlePyramidViewScanlineEffectBuffer();
        ScanlineEffect_SetParams(*sFlashEffectParams);
    } else if ({
        flashLevel = GetFlashLevel();
        flashLevel
    }) != 0
    {
        WriteFlashScanlineEffectBuffer(flashLevel);
        ScanlineEffect_SetParams(*sFlashEffectParams);
    }
}
unsafe fn LoadMapInStepsLink(state: *mut u8) -> u32 {
    match *state {
        0 => {
            InitOverworldBgs();
            ScriptContext_Init();
            UnlockPlayerFieldControls();
            ResetMirageTowerAndSaveBlockPtrs();
            ResetScreenForMapLoad();
            *state += 1;
        }
        1 => {
            LoadMapFromWarp(TRUE as u32);
            *state += 1;
        }
        2 => {
            ResumeMap(TRUE as u32);
            *state += 1;
        }
        3 => {
            OffsetCameraFocusByLinkPlayerId();
            InitObjectEventsLink();
            SpawnLinkPlayers();
            SetCameraToTrackGuestPlayer();
            *state += 1;
        }
        4 => {
            InitCurrentFlashLevelScanlineEffect();
            InitOverworldGraphicsRegisters();
            InitTextBoxGfxAndPrinters();
            *state += 1;
        }
        5 => {
            ResetFieldCamera();
            *state += 1;
        }
        6 => {
            CopyPrimaryTilesetToVram(gMapHeader.mapLayout);
            *state += 1;
        }
        7 => {
            CopySecondaryTilesetToVram(gMapHeader.mapLayout);
            *state += 1;
        }
        8 => {
            if FreeTempTileDataBuffersIfPossible() != TRUE {
                LoadMapTilesetPalettes(gMapHeader.mapLayout);
                *state += 1;
            }
        }
        9 => {
            DrawWholeMapView();
            *state += 1;
        }
        10 => {
            InitTilesetAnimations();
            *state += 1;
        }
        11 => {
            if gWirelessCommType != 0 {
                LoadWirelessStatusIndicatorSpriteGfx();
                CreateWirelessStatusIndicatorSprite(0, 0);
            }
            *state += 1;
        }
        12 => {
            if RunFieldCallback() != 0 {
                *state += 1;
            }
        }
        13 => {
            return TRUE as u32;
        }
        _ => {}
    }
    FALSE as u32
}
unsafe fn LoadMapInStepsLocal(state: *mut u8, a2: u32) -> u32 {
    match *state {
        0 => {
            FieldClearVBlankHBlankCallbacks();
            LoadMapFromWarp(a2);
            *state += 1;
        }
        1 => {
            ResetMirageTowerAndSaveBlockPtrs();
            ResetScreenForMapLoad();
            *state += 1;
        }
        2 => {
            ResumeMap(a2);
            *state += 1;
        }
        3 => {
            InitObjectEventsLocal();
            SetCameraToTrackPlayer();
            *state += 1;
        }
        4 => {
            InitCurrentFlashLevelScanlineEffect();
            InitOverworldGraphicsRegisters();
            InitTextBoxGfxAndPrinters();
            *state += 1;
        }
        5 => {
            ResetFieldCamera();
            *state += 1;
        }
        6 => {
            CopyPrimaryTilesetToVram(gMapHeader.mapLayout);
            *state += 1;
        }
        7 => {
            CopySecondaryTilesetToVram(gMapHeader.mapLayout);
            *state += 1;
        }
        8 => {
            if FreeTempTileDataBuffersIfPossible() != TRUE {
                LoadMapTilesetPalettes(gMapHeader.mapLayout);
                *state += 1;
            }
        }
        9 => {
            DrawWholeMapView();
            *state += 1;
        }
        10 => {
            InitTilesetAnimations();
            *state += 1;
        }
        11 => {
            if gMapHeader.showMapName() == TRUE && SecretBaseMapPopupEnabled() == TRUE {
                ShowMapNamePopup();
            }
            *state += 1;
        }
        12 => {
            if RunFieldCallback() != 0 {
                *state += 1;
            }
        }
        13 => {
            return TRUE as u32;
        }
        _ => {}
    }
    FALSE as u32
}
unsafe fn ReturnToFieldLocal(state: *mut u8) -> u32 {
    match *state {
        0 => {
            ResetMirageTowerAndSaveBlockPtrs();
            ResetScreenForMapLoad();
            ResumeMap(FALSE as u32);
            InitObjectEventsReturnToField();
            SetCameraToTrackPlayer();
            *state += 1;
        }
        1 => {
            InitViewGraphics();
            TryLoadTrainerHillEReaderPalette();
            *state += 1;
        }
        2 => {
            if RunFieldCallback() != 0 {
                *state += 1;
            }
        }
        3 => {
            return TRUE as u32;
        }
        _ => {}
    }
    FALSE as u32
}
unsafe fn ReturnToFieldLink(state: *mut u8) -> u32 {
    match *state {
        0 => {
            FieldClearVBlankHBlankCallbacks();
            ResetMirageTowerAndSaveBlockPtrs();
            ResetScreenForMapLoad();
            *state += 1;
        }
        1 => {
            ResumeMap(TRUE as u32);
            *state += 1;
        }
        2 => {
            CreateLinkPlayerSprites();
            InitObjectEventsReturnToField();
            SetCameraToTrackGuestPlayer_2();
            *state += 1;
        }
        3 => {
            InitCurrentFlashLevelScanlineEffect();
            InitOverworldGraphicsRegisters();
            InitTextBoxGfxAndPrinters();
            *state += 1;
        }
        4 => {
            ResetFieldCamera();
            *state += 1;
        }
        5 => {
            CopyPrimaryTilesetToVram(gMapHeader.mapLayout);
            *state += 1;
        }
        6 => {
            CopySecondaryTilesetToVram(gMapHeader.mapLayout);
            *state += 1;
        }
        7 => {
            if FreeTempTileDataBuffersIfPossible() != TRUE {
                LoadMapTilesetPalettes(gMapHeader.mapLayout);
                *state += 1;
            }
        }
        8 => {
            DrawWholeMapView();
            *state += 1;
        }
        9 => {
            InitTilesetAnimations();
            *state += 1;
        }
        11 => {
            if gWirelessCommType != 0 {
                LoadWirelessStatusIndicatorSpriteGfx();
                CreateWirelessStatusIndicatorSprite(0, 0);
            }
            *state += 1;
        }
        12 => {
            if RunFieldCallback() != 0 {
                *state += 1;
            }
        }
        10 => {
            *state += 1;
        }
        13 => {
            SetFieldVBlankCallback();
            *state += 1;
            return TRUE as u32;
        }
        _ => {}
    }
    FALSE as u32
}
unsafe fn DoMapLoadLoop(state: *mut u8) {
    while LoadMapInStepsLocal(state, FALSE as u32) == 0 {}
}
unsafe fn ResetMirageTowerAndSaveBlockPtrs() {
    ClearMirageTowerPulseBlend();
    MoveSaveBlocks_ResetHeap();
}
unsafe fn ResetScreenForMapLoad() {
    SetGpuReg(0x0, 0);
    ScanlineEffect_Stop();
    {
        {
            let mut _dest: *mut u16 = 83886082_usize as *mut u16;
            let mut _size: u32 = 1022;
            {
                {
                    let mut tmp: u16 = 0;
                    volatile_write(&raw mut tmp, 0);
                    {
                        {
                            let dmaRegs: *mut u32 = 67109076_usize as *mut u32;
                            volatile_write(dmaRegs, &raw mut tmp as usize as u32);
                            volatile_write(dmaRegs.at(1), _dest as usize as u32);
                            volatile_write(dmaRegs.at(2), 0x81000000 | (_size / 2));
                            let _ = (dmaRegs.at(2)).read_volatile();
                        }
                    }
                }
            }
        }
    }
    {
        let mut _dest: *mut c_void = VRAM as usize as *mut c_void;
        let mut _size: u32 = VRAM_SIZE;
        loop {
            {
                {
                    let mut tmp: u16 = 0;
                    volatile_write(&raw mut tmp, 0);
                    {
                        {
                            let dmaRegs: *mut u32 = 67109076_usize as *mut u32;
                            volatile_write(dmaRegs, &raw mut tmp as usize as u32);
                            volatile_write(dmaRegs.at(1), _dest as usize as u32);
                            volatile_write(dmaRegs.at(2), 0x81000800);
                            let _ = (dmaRegs.at(2)).read_volatile();
                        }
                    }
                }
            }
            _dest = (_dest as *mut u8).at(4096) as *mut c_void;
            _size -= 0x1000;
            if _size <= 0x1000 {
                {
                    {
                        let mut tmp: u16 = 0;
                        volatile_write(&raw mut tmp, 0);
                        {
                            {
                                let dmaRegs: *mut u32 = 67109076_usize as *mut u32;
                                volatile_write(dmaRegs, &raw mut tmp as usize as u32);
                                volatile_write(dmaRegs.at(1), _dest as usize as u32);
                                volatile_write(dmaRegs.at(2), 0x81000000 | (_size / 2));
                                let _ = (dmaRegs.at(2)).read_volatile();
                            }
                        }
                    }
                }
                break;
            }
        }
    }
    ResetOamRange(0, 128);
    LoadOam();
}
unsafe fn InitViewGraphics() {
    InitCurrentFlashLevelScanlineEffect();
    InitOverworldGraphicsRegisters();
    InitTextBoxGfxAndPrinters();
    InitMapView();
}
unsafe fn InitOverworldGraphicsRegisters() {
    ClearScheduledBgCopiesToVram();
    ResetTempTileDataBuffers();
    SetGpuReg(REG_OFFSET_MOSAIC, 0);
    SetGpuReg(REG_OFFSET_WININ, 7967);
    SetGpuReg(REG_OFFSET_WINOUT, 257);
    SetGpuReg(REG_OFFSET_WIN0H, 0xFF);
    SetGpuReg(REG_OFFSET_WIN0V, 0xFF);
    SetGpuReg(REG_OFFSET_WIN1H, 0xFFFF);
    SetGpuReg(REG_OFFSET_WIN1V, 0xFFFF);
    SetGpuReg(
        REG_OFFSET_BLDCNT,
        (*(&raw const crate::io_reg::gOverworldBackgroundLayerFlags).cast::<CArray<u16, 0>>())[1]
            | (*(&raw const crate::io_reg::gOverworldBackgroundLayerFlags)
                .cast::<CArray<u16, 0>>())[2]
            | (*(&raw const crate::io_reg::gOverworldBackgroundLayerFlags)
                .cast::<CArray<u16, 0>>())[3]
            | BLDCNT_TGT2_OBJ
            | BLDCNT_EFFECT_BLEND,
    );
    SetGpuReg(REG_OFFSET_BLDALPHA, 1805);
    InitOverworldBgs();
    ScheduleBgCopyTilemapToVram(1);
    ScheduleBgCopyTilemapToVram(2);
    ScheduleBgCopyTilemapToVram(3);
    ChangeBgX(0, 0, BG_COORD_SET);
    ChangeBgY(0, 0, BG_COORD_SET);
    ChangeBgX(1, 0, BG_COORD_SET);
    ChangeBgY(1, 0, BG_COORD_SET);
    ChangeBgX(2, 0, BG_COORD_SET);
    ChangeBgY(2, 0, BG_COORD_SET);
    ChangeBgX(3, 0, BG_COORD_SET);
    ChangeBgY(3, 0, BG_COORD_SET);
    SetGpuReg(REG_OFFSET_DISPCNT, 28768);
    ShowBg(0);
    ShowBg(1);
    ShowBg(2);
    ShowBg(3);
    InitFieldMessageBox();
}
unsafe fn ResumeMap(a1: u32) {
    ResetTasks();
    ResetSpriteData();
    ResetPaletteFade();
    ScanlineEffect_Clear();
    ResetAllPicSprites();
    ResetCameraUpdateInfo();
    InstallCameraPanAheadCallback();
    if a1 == 0 {
        InitObjectEventPalettes(0);
    } else {
        InitObjectEventPalettes(1);
    }
    FieldEffectActiveListClear();
    StartWeather();
    ResumePausedWeather();
    if a1 == 0 {
        SetUpFieldTasks();
    }
    RunOnResumeMapScript();
    TryStartMirageTowerPulseBlendEffect();
}
unsafe fn InitObjectEventsLink() {
    gTotalCameraPixelOffsetX = 0;
    gTotalCameraPixelOffsetY = 0;
    ResetObjectEvents();
    TrySpawnObjectEvents(0, 0);
    TryRunOnWarpIntoMapScript();
}
unsafe fn InitObjectEventsLocal() {
    let mut x: u16 = 0;
    let mut y: u16 = 0;
    gTotalCameraPixelOffsetX = 0;
    gTotalCameraPixelOffsetY = 0;
    ResetObjectEvents();
    GetCameraFocusCoords(&raw mut x, &raw mut y);
    let player: *mut InitialPlayerAvatarState = GetInitialPlayerAvatarState();
    InitPlayerAvatar(
        x as i16,
        y as i16,
        (*player).direction,
        (*gSaveBlock2Ptr).playerGender,
    );
    SetPlayerAvatarTransitionFlags((*player).transitionFlags as u16);
    ResetInitialPlayerAvatarState();
    TrySpawnObjectEvents(0, 0);
    TryRunOnWarpIntoMapScript();
}
unsafe fn InitObjectEventsReturnToField() {
    SpawnObjectEventsOnReturnToField(0, 0);
    RotatingGate_InitPuzzleAndGraphics();
    RunOnReturnToFieldMapScript();
}
unsafe fn SetCameraToTrackPlayer() {
    gObjectEvents[gPlayerAvatar.objectEventId].set_trackedByCamera(TRUE as u32);
    InitCameraUpdateCallback(gPlayerAvatar.spriteId);
}
unsafe fn SetCameraToTrackGuestPlayer() {
    InitCameraUpdateCallback(GetSpriteForLinkedPlayer(gLocalLinkPlayerId));
}
unsafe fn SetCameraToTrackGuestPlayer_2() {
    InitCameraUpdateCallback(GetSpriteForLinkedPlayer(gLocalLinkPlayerId));
}
unsafe fn OffsetCameraFocusByLinkPlayerId() {
    let mut x: u16 = 0;
    let mut y: u16 = 0;
    GetCameraFocusCoords(&raw mut x, &raw mut y);
    SetCameraFocusCoords(x + gLocalLinkPlayerId as u16, y);
}
unsafe fn SpawnLinkPlayers() {
    let mut x: u16 = 0;
    let mut y: u16 = 0;
    GetCameraFocusCoords(&raw mut x, &raw mut y);
    x -= gLocalLinkPlayerId as u16;
    let mut i: u16 = 0;
    while i < gFieldLinkPlayerCount as u16 {
        SpawnLinkPlayerObjectEvent(
            i as u8,
            i as i16 + x as i16,
            y as i16,
            gLinkPlayers[i].gender,
        );
        CreateLinkPlayerSprite(i as u8, gLinkPlayers[i].version as u8);
        i += 1;
    }
    ClearAllPlayerKeys();
}
unsafe fn CreateLinkPlayerSprites() {
    let mut i: u16 = 0;
    while i < gFieldLinkPlayerCount as u16 {
        CreateLinkPlayerSprite(i as u8, gLinkPlayers[i].version as u8);
        i += 1;
    }
}
pub(crate) unsafe fn CB1_OverworldLink() {
    if gWirelessCommType == 0 || IsRfuRecvQueueEmpty() == 0 || IsSendingKeysToLink() == 0 {
        let selfId: u8 = gLocalLinkPlayerId;
        UpdateAllLinkPlayers(gLinkPartnersHeldKeys.as_mut_ptr(), selfId as i32);
        UpdateHeldKeyCode(sPlayerKeyInterceptCallback.unwrap_unchecked()(
            selfId as u32,
        ));
        ClearAllPlayerKeys();
    }
}
pub unsafe fn ResetAllMultiplayerState() {
    ResetAllPlayerLinkStates();
    SetKeyInterceptCallback(Some(KeyInterCB_SelfIdle));
}
unsafe fn ClearAllPlayerKeys() {
    ResetPlayerHeldKeys(gLinkPartnersHeldKeys.as_mut_ptr());
}
unsafe fn SetKeyInterceptCallback(func: Option<unsafe fn(u32) -> u16>) {
    sRfuKeepAliveTimer.set(0);
    sPlayerKeyInterceptCallback = func;
}
unsafe fn CheckRfuKeepAliveTimer() {
    if gWirelessCommType != 0
        && ({
            sRfuKeepAliveTimer.set(sRfuKeepAliveTimer.get() + 1);
            sRfuKeepAliveTimer.get()
        }) > 60
    {
        LinkRfu_FatalError();
    }
}
unsafe fn ResetAllPlayerLinkStates() {
    for i in 0..MAX_LINK_PLAYERS {
        sPlayerLinkStates[i] = PLAYER_LINK_STATE_IDLE;
    }
}
unsafe fn AreAllPlayersInLinkState(state: u16) -> u32 {
    let count: i32 = gFieldLinkPlayerCount as i32;
    for i in 0..count {
        if sPlayerLinkStates[i] as u16 != state {
            return FALSE as u32;
        }
    }
    TRUE as u32
}
unsafe fn IsAnyPlayerInLinkState(state: u16) -> u32 {
    let count: i32 = gFieldLinkPlayerCount as i32;
    for i in 0..count {
        if sPlayerLinkStates[i] as u16 == state {
            return TRUE as u32;
        }
    }
    FALSE as u32
}
unsafe fn HandleLinkPlayerKeyInput(
    playerId: u32,
    key: u16,
    trainer: *mut CableClubPlayer,
    forceFacing: *mut u16,
) {
    let mut script: *mut u8 = null_mut();
    if sPlayerLinkStates[playerId] == PLAYER_LINK_STATE_IDLE {
        script = TryGetTileEventScript(trainer);
        if !script.is_null() {
            *forceFacing = GetDirectionForEventScript(script);
            sPlayerLinkStates[playerId] = PLAYER_LINK_STATE_BUSY;
            if (*trainer).isLocalPlayer != 0 {
                SetKeyInterceptCallback(Some(KeyInterCB_DeferToEventScript));
                RunInteractLocalPlayerScript(script);
            }
            return;
        }
        if IsAnyPlayerInLinkState(PLAYER_LINK_STATE_EXITING_ROOM) == TRUE as u32 {
            sPlayerLinkStates[playerId] = PLAYER_LINK_STATE_BUSY;
            if (*trainer).isLocalPlayer != 0 {
                SetKeyInterceptCallback(Some(KeyInterCB_DeferToEventScript));
                RunTerminateLinkScript();
            }
            return;
        }
        match key {
            LINK_KEY_CODE_START_BUTTON => {
                if CanCableClubPlayerPressStart(trainer) != 0 {
                    sPlayerLinkStates[playerId] = PLAYER_LINK_STATE_BUSY;
                    if (*trainer).isLocalPlayer != 0 {
                        SetKeyInterceptCallback(Some(KeyInterCB_DeferToEventScript));
                        InitLinkRoomStartMenuScript();
                    }
                }
            }
            LINK_KEY_CODE_DPAD_DOWN => {
                if PlayerIsAtSouthExit(trainer) == TRUE as u32 {
                    sPlayerLinkStates[playerId] = PLAYER_LINK_STATE_BUSY;
                    if (*trainer).isLocalPlayer != 0 {
                        SetKeyInterceptCallback(Some(KeyInterCB_DeferToEventScript));
                        RunConfirmLeaveCableClubScript();
                    }
                }
            }
            LINK_KEY_CODE_A_BUTTON => {
                script = TryInteractWithPlayer(trainer);
                if !script.is_null() {
                    sPlayerLinkStates[playerId] = PLAYER_LINK_STATE_BUSY;
                    if (*trainer).isLocalPlayer != 0 {
                        SetKeyInterceptCallback(Some(KeyInterCB_DeferToEventScript));
                        InitMenuBasedScript(script);
                    }
                }
            }
            LINK_KEY_CODE_HANDLE_RECV_QUEUE => {
                if IsCableClubPlayerUnfrozen(trainer) != 0 {
                    sPlayerLinkStates[playerId] = PLAYER_LINK_STATE_BUSY;
                    if (*trainer).isLocalPlayer != 0 {
                        SetKeyInterceptCallback(Some(KeyInterCB_DeferToRecvQueue));
                        InitLinkPlayerQueueScript();
                    }
                }
            }
            LINK_KEY_CODE_HANDLE_SEND_QUEUE if IsCableClubPlayerUnfrozen(trainer) != 0 => {
                sPlayerLinkStates[playerId] = PLAYER_LINK_STATE_BUSY;
                if (*trainer).isLocalPlayer != 0 {
                    SetKeyInterceptCallback(Some(KeyInterCB_DeferToSendQueue));
                    InitLinkPlayerQueueScript();
                }
            }
            _ => {}
        }
    }
    match key {
        LINK_KEY_CODE_EXIT_ROOM => {
            sPlayerLinkStates[playerId] = PLAYER_LINK_STATE_EXITING_ROOM as u8;
        }
        LINK_KEY_CODE_READY => {
            sPlayerLinkStates[playerId] = PLAYER_LINK_STATE_READY;
        }
        LINK_KEY_CODE_IDLE => {
            sPlayerLinkStates[playerId] = PLAYER_LINK_STATE_IDLE;
            if (*trainer).isLocalPlayer != 0 {
                SetKeyInterceptCallback(Some(KeyInterCB_SelfIdle));
            }
        }
        LINK_KEY_CODE_EXIT_SEAT if sPlayerLinkStates[playerId] == PLAYER_LINK_STATE_READY => {
            sPlayerLinkStates[playerId] = PLAYER_LINK_STATE_BUSY;
        }
        _ => {}
    }
}
unsafe fn UpdateAllLinkPlayers(keys: *mut u16, selfId: i32) {
    let mut trainer: CableClubPlayer = zeroed();
    for i in 0..MAX_LINK_PLAYERS {
        let key: u8 = *keys.at(i) as u8;
        let mut setFacing: u16 = FACING_NONE;
        LoadCableClubPlayer(i, selfId, &raw mut trainer);
        HandleLinkPlayerKeyInput(i as u32, key as u16, &raw mut trainer, &raw mut setFacing);
        if sPlayerLinkStates[i] == PLAYER_LINK_STATE_IDLE {
            setFacing = GetDirectionForDpadKey(key as u16);
        }
        SetPlayerFacingDirection(i as u8, setFacing as u8);
    }
}
unsafe fn UpdateHeldKeyCode(key: u16) {
    if (LINK_KEY_CODE_EMPTY..LINK_KEY_CODE_UNK_8).contains(&key) {
        gHeldKeyCodeToSend = key;
    } else {
        gHeldKeyCodeToSend = LINK_KEY_CODE_EMPTY;
    }
    if gWirelessCommType != 0
        && GetLinkSendQueueLength() > 1
        && IsOverworldLinkActive() == TRUE as u32
        && IsSendingKeysToLink() == TRUE as u32
    {
        match key {
            LINK_KEY_CODE_EMPTY
            | LINK_KEY_CODE_DPAD_DOWN
            | LINK_KEY_CODE_DPAD_UP
            | LINK_KEY_CODE_DPAD_LEFT
            | LINK_KEY_CODE_DPAD_RIGHT
            | LINK_KEY_CODE_START_BUTTON
            | LINK_KEY_CODE_A_BUTTON => {
                gHeldKeyCodeToSend = LINK_KEY_CODE_NULL;
            }
            _ => {}
        }
    }
}
unsafe fn KeyInterCB_ReadButtons(key: u32) -> u16 {
    if gMain.heldKeys as i32 & DPAD_UP != 0 {
        return LINK_KEY_CODE_DPAD_UP;
    }
    if gMain.heldKeys as i32 & DPAD_DOWN != 0 {
        return LINK_KEY_CODE_DPAD_DOWN;
    }
    if gMain.heldKeys as i32 & DPAD_LEFT != 0 {
        return LINK_KEY_CODE_DPAD_LEFT;
    }
    if gMain.heldKeys as i32 & DPAD_RIGHT != 0 {
        return LINK_KEY_CODE_DPAD_RIGHT;
    }
    if gMain.newKeys as i32 & START_BUTTON != 0 {
        return LINK_KEY_CODE_START_BUTTON;
    }
    if gMain.newKeys as i32 & A_BUTTON != 0 {
        return LINK_KEY_CODE_A_BUTTON;
    }
    LINK_KEY_CODE_EMPTY
}
fn GetDirectionForDpadKey(key: u16) -> u16 {
    match key {
        LINK_KEY_CODE_DPAD_RIGHT => {
            return FACING_RIGHT;
        }
        LINK_KEY_CODE_DPAD_LEFT => {
            return FACING_LEFT;
        }
        LINK_KEY_CODE_DPAD_UP => {
            return FACING_UP as u16;
        }
        LINK_KEY_CODE_DPAD_DOWN => {
            return FACING_DOWN;
        }
        _ => {
            return FACING_NONE;
        }
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
unsafe fn ResetPlayerHeldKeys(keys: *mut u16) {
    for i in 0..4i32 {
        *keys.at(i) = LINK_KEY_CODE_EMPTY;
    }
}
pub(crate) unsafe fn KeyInterCB_SelfIdle(key: u32) -> u16 {
    if ArePlayerFieldControlsLocked() == TRUE {
        return LINK_KEY_CODE_EMPTY;
    }
    if GetLinkRecvQueueLength() > 4 {
        return LINK_KEY_CODE_HANDLE_RECV_QUEUE;
    }
    if GetLinkSendQueueLength() <= 4 {
        return KeyInterCB_ReadButtons(key);
    }
    LINK_KEY_CODE_HANDLE_SEND_QUEUE
}
pub(crate) unsafe fn KeyInterCB_Idle(key: u32) -> u16 {
    CheckRfuKeepAliveTimer();
    LINK_KEY_CODE_EMPTY
}
pub(crate) unsafe fn KeyInterCB_DeferToEventScript(key: u32) -> u16 {
    let mut retVal: u16 = 0;
    if ArePlayerFieldControlsLocked() == TRUE {
        retVal = LINK_KEY_CODE_EMPTY;
    } else {
        retVal = LINK_KEY_CODE_IDLE;
        SetKeyInterceptCallback(Some(KeyInterCB_Idle));
    }
    retVal
}
pub(crate) unsafe fn KeyInterCB_DeferToRecvQueue(key: u32) -> u16 {
    let mut retVal: u16 = 0;
    if GetLinkRecvQueueLength() >= OVERWORLD_RECV_QUEUE_MAX {
        retVal = LINK_KEY_CODE_EMPTY;
    } else {
        retVal = LINK_KEY_CODE_IDLE;
        UnlockPlayerFieldControls();
        SetKeyInterceptCallback(Some(KeyInterCB_Idle));
    }
    retVal
}
pub(crate) unsafe fn KeyInterCB_DeferToSendQueue(key: u32) -> u16 {
    let mut retVal: u16 = 0;
    if GetLinkSendQueueLength() > 2 {
        retVal = LINK_KEY_CODE_EMPTY;
    } else {
        retVal = LINK_KEY_CODE_IDLE;
        UnlockPlayerFieldControls();
        SetKeyInterceptCallback(Some(KeyInterCB_Idle));
    }
    retVal
}
pub(crate) unsafe fn KeyInterCB_ExitingSeat(key: u32) -> u16 {
    CheckRfuKeepAliveTimer();
    LINK_KEY_CODE_EMPTY
}
pub(crate) unsafe fn KeyInterCB_Ready(keyOrPlayerId: u32) -> u16 {
    if sPlayerLinkStates[keyOrPlayerId] == PLAYER_LINK_STATE_READY {
        if gMain.newKeys as i32 & B_BUTTON != 0 {
            SetKeyInterceptCallback(Some(KeyInterCB_ExitingSeat));
            return LINK_KEY_CODE_EXIT_SEAT;
        } else {
            return LINK_KEY_CODE_EMPTY;
        }
    } else {
        CheckRfuKeepAliveTimer();
        return LINK_KEY_CODE_EMPTY;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
pub(crate) unsafe fn KeyInterCB_SetReady(key: u32) -> u16 {
    SetKeyInterceptCallback(Some(KeyInterCB_Ready));
    LINK_KEY_CODE_READY
}
pub(crate) fn KeyInterCB_SendNothing(key: u32) -> u16 {
    LINK_KEY_CODE_EMPTY
}
pub(crate) unsafe fn KeyInterCB_WaitForPlayersToExit(keyOrPlayerId: u32) -> u16 {
    if sPlayerLinkStates[keyOrPlayerId] != PLAYER_LINK_STATE_EXITING_ROOM as u8 {
        CheckRfuKeepAliveTimer();
    }
    if AreAllPlayersInLinkState(PLAYER_LINK_STATE_EXITING_ROOM) == TRUE as u32 {
        ScriptContext_SetupScript(
            (*crate::asmdata::EventScript_DoLinkRoomExit.cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
        );
        SetKeyInterceptCallback(Some(KeyInterCB_SendNothing));
    }
    LINK_KEY_CODE_EMPTY
}
pub(crate) unsafe fn KeyInterCB_SendExitRoomKey(key: u32) -> u16 {
    SetKeyInterceptCallback(Some(KeyInterCB_WaitForPlayersToExit));
    LINK_KEY_CODE_EXIT_ROOM
}
pub(crate) unsafe fn KeyInterCB_InLinkActivity(key: u32) -> u16 {
    LINK_KEY_CODE_EMPTY
}
pub unsafe fn GetCableClubPartnersReady() -> u32 {
    if IsAnyPlayerInLinkState(PLAYER_LINK_STATE_EXITING_ROOM) == TRUE as u32 {
        return CABLE_SEAT_FAILED;
    }
    if sPlayerKeyInterceptCallback == Some(KeyInterCB_Ready as unsafe fn(u32) -> u16)
        && sPlayerLinkStates[gLocalLinkPlayerId] != PLAYER_LINK_STATE_READY
    {
        return CABLE_SEAT_WAITING;
    }
    if sPlayerKeyInterceptCallback == Some(KeyInterCB_ExitingSeat as unsafe fn(u32) -> u16)
        && sPlayerLinkStates[gLocalLinkPlayerId] == PLAYER_LINK_STATE_BUSY
    {
        return CABLE_SEAT_FAILED;
    }
    if AreAllPlayersInLinkState(PLAYER_LINK_STATE_READY as u16) != 0 {
        return CABLE_SEAT_SUCCESS;
    }
    CABLE_SEAT_WAITING
}
unsafe fn IsAnyPlayerExitingCableClub() -> u32 {
    IsAnyPlayerInLinkState(PLAYER_LINK_STATE_EXITING_ROOM)
}
pub unsafe fn SetInCableClubSeat() -> u16 {
    SetKeyInterceptCallback(Some(KeyInterCB_SetReady));
    0
}
pub unsafe fn SetLinkWaitingForScript() -> u16 {
    SetKeyInterceptCallback(Some(KeyInterCB_DeferToEventScript));
    0
}
pub unsafe fn QueueExitLinkRoomKey() -> u16 {
    SetKeyInterceptCallback(Some(KeyInterCB_SendExitRoomKey));
    0
}
pub unsafe fn SetStartedCableClubActivity() -> u16 {
    SetKeyInterceptCallback(Some(KeyInterCB_InLinkActivity));
    0
}
unsafe fn LoadCableClubPlayer(linkPlayerId: i32, myPlayerId: i32, trainer: *mut CableClubPlayer) {
    let mut x: i16 = 0;
    let mut y: i16 = 0;
    (*trainer).playerId = linkPlayerId as u8;
    (*trainer).isLocalPlayer = (if linkPlayerId == myPlayerId { 1 } else { 0 }) as u8;
    (*trainer).movementMode = gLinkPlayerObjectEvents[linkPlayerId].movementMode;
    (*trainer).facing = GetLinkPlayerFacingDirection(linkPlayerId as u8);
    GetLinkPlayerCoords(linkPlayerId as u8, &raw mut x, &raw mut y);
    (*trainer).pos.x = x;
    (*trainer).pos.y = y;
    (*trainer).pos.elevation = GetLinkPlayerElevation(linkPlayerId as u8) as i8;
    (*trainer).metatileBehavior = MapGridGetMetatileBehaviorAt(x as i32, y as i32) as u16;
}
unsafe fn IsCableClubPlayerUnfrozen(player: *mut CableClubPlayer) -> u32 {
    let mode: u8 = (*player).movementMode;
    if mode == MOVEMENT_MODE_SCRIPTED || mode == MOVEMENT_MODE_FREE {
        return TRUE as u32;
    } else {
        return FALSE as u32;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
unsafe fn CanCableClubPlayerPressStart(player: *mut CableClubPlayer) -> u32 {
    let mode: u8 = (*player).movementMode;
    if mode == MOVEMENT_MODE_SCRIPTED || mode == MOVEMENT_MODE_FREE {
        return TRUE as u32;
    } else {
        return FALSE as u32;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
unsafe fn TryGetTileEventScript(player: *mut CableClubPlayer) -> *mut u8 {
    if (*player).movementMode != MOVEMENT_MODE_SCRIPTED {
        return null_mut();
    }
    GetCoordEventScriptAtMapPosition(&raw mut (*player).pos)
}
unsafe fn PlayerIsAtSouthExit(player: *mut CableClubPlayer) -> u32 {
    if (*player).movementMode != MOVEMENT_MODE_SCRIPTED
        && (*player).movementMode != MOVEMENT_MODE_FREE
    {
        return FALSE as u32;
    } else if MetatileBehavior_IsSouthArrowWarp((*player).metatileBehavior as u8) == 0 {
        return FALSE as u32;
    } else if (*player).facing != DIR_SOUTH {
        return FALSE as u32;
    } else {
        return TRUE as u32;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
unsafe fn TryInteractWithPlayer(player: *mut CableClubPlayer) -> *mut u8 {
    if (*player).movementMode != MOVEMENT_MODE_FREE
        && (*player).movementMode != MOVEMENT_MODE_SCRIPTED
    {
        return null_mut();
    }
    let mut otherPlayerPos: MapPosition = (*player).pos;
    otherPlayerPos.x += gDirectionToVectors[(*player).facing].x as i16;
    otherPlayerPos.y += gDirectionToVectors[(*player).facing].y as i16;
    otherPlayerPos.elevation = ELEVATION_TRANSITION as i8;
    let linkPlayerId: u8 = GetLinkPlayerIdAt(otherPlayerPos.x, otherPlayerPos.y);
    if linkPlayerId != MAX_LINK_PLAYERS as u8 {
        if (*player).isLocalPlayer == 0 {
            return (*crate::asmdata::CableClub_EventScript_TooBusyToNotice
                .cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut();
        } else if sPlayerLinkStates[linkPlayerId] != PLAYER_LINK_STATE_IDLE {
            return (*crate::asmdata::CableClub_EventScript_TooBusyToNotice
                .cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut();
        } else if GetLinkTrainerCardColor(linkPlayerId) == 0 {
            return (*crate::asmdata::CableClub_EventScript_ReadTrainerCard
                .cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut();
        } else {
            return (*crate::asmdata::CableClub_EventScript_ReadTrainerCardColored
                .cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut();
        }
    }
    GetInteractedLinkPlayerScript(
        &raw mut otherPlayerPos,
        (*player).metatileBehavior as u8,
        (*player).facing,
    )
}
unsafe fn GetDirectionForEventScript(script: *mut u8) -> u16 {
    if script
        == (*crate::asmdata::EventScript_BattleColosseum_4P_PlayerSpot0.cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut()
    {
        return FACING_FORCED_RIGHT;
    } else if script
        == (*crate::asmdata::EventScript_BattleColosseum_4P_PlayerSpot1.cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut()
    {
        return FACING_FORCED_LEFT;
    } else if script
        == (*crate::asmdata::EventScript_BattleColosseum_4P_PlayerSpot2.cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut()
    {
        return FACING_FORCED_RIGHT;
    } else if script
        == (*crate::asmdata::EventScript_BattleColosseum_4P_PlayerSpot3.cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut()
    {
        return FACING_FORCED_LEFT;
    } else if script
        == (*crate::asmdata::EventScript_RecordCenter_Spot0.cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut()
    {
        return FACING_FORCED_RIGHT;
    } else if script
        == (*crate::asmdata::EventScript_RecordCenter_Spot1.cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut()
    {
        return FACING_FORCED_LEFT;
    } else if script
        == (*crate::asmdata::EventScript_RecordCenter_Spot2.cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut()
    {
        return FACING_FORCED_RIGHT;
    } else if script
        == (*crate::asmdata::EventScript_RecordCenter_Spot3.cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut()
    {
        return FACING_FORCED_LEFT;
    } else if script
        == (*crate::asmdata::EventScript_BattleColosseum_2P_PlayerSpot0.cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut()
    {
        return FACING_FORCED_RIGHT;
    } else if script
        == (*crate::asmdata::EventScript_BattleColosseum_2P_PlayerSpot1.cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut()
    {
        return FACING_FORCED_LEFT;
    } else if script
        == (*crate::asmdata::EventScript_TradeCenter_Chair0.cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut()
    {
        return FACING_FORCED_RIGHT;
    } else if script
        == (*crate::asmdata::EventScript_TradeCenter_Chair1.cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut()
    {
        return FACING_FORCED_LEFT;
    } else {
        return FACING_NONE;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
unsafe fn InitLinkPlayerQueueScript() {
    LockPlayerFieldControls();
}
unsafe fn InitLinkRoomStartMenuScript() {
    PlaySE(SE_WIN_OPEN);
    ShowStartMenu();
    LockPlayerFieldControls();
}
unsafe fn RunInteractLocalPlayerScript(script: *mut u8) {
    PlaySE(SE_SELECT);
    ScriptContext_SetupScript(script);
    LockPlayerFieldControls();
}
unsafe fn RunConfirmLeaveCableClubScript() {
    PlaySE(SE_WIN_OPEN);
    ScriptContext_SetupScript(
        (*crate::asmdata::EventScript_ConfirmLeaveCableClubRoom.cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
    );
    LockPlayerFieldControls();
}
unsafe fn InitMenuBasedScript(script: *mut u8) {
    PlaySE(SE_SELECT);
    ScriptContext_SetupScript(script);
    LockPlayerFieldControls();
}
unsafe fn RunTerminateLinkScript() {
    ScriptContext_SetupScript(
        (*crate::asmdata::EventScript_TerminateLink.cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
    );
    LockPlayerFieldControls();
}
pub unsafe fn Overworld_IsRecvQueueAtMax() -> u32 {
    if IsOverworldLinkActive() == 0 {
        return FALSE as u32;
    }
    if GetLinkRecvQueueLength() >= OVERWORLD_RECV_QUEUE_MAX {
        sReceivingFromLink.set(TRUE);
    } else {
        sReceivingFromLink.set(FALSE);
    }
    sReceivingFromLink.get() as u32
}
#[unsafe(no_mangle)]
pub unsafe fn Overworld_RecvKeysFromLinkIsRunning() -> u32 {
    if GetLinkRecvQueueLength() < 2 {
        return FALSE as u32;
    } else if IsOverworldLinkActive() != TRUE as u32 {
        return FALSE as u32;
    } else if IsSendingKeysToLink() != TRUE as u32 {
        return FALSE as u32;
    } else if sPlayerKeyInterceptCallback
        == Some(KeyInterCB_DeferToRecvQueue as unsafe fn(u32) -> u16)
    {
        return TRUE as u32;
    } else if sPlayerKeyInterceptCallback
        != Some(KeyInterCB_DeferToEventScript as unsafe fn(u32) -> u16)
    {
        return FALSE as u32;
    }
    let temp: u8 = sReceivingFromLink.get();
    sReceivingFromLink.set(FALSE);
    if temp == TRUE {
        return TRUE as u32;
    } else if gPaletteFade.active() != 0 && gPaletteFade.softwareFadeFinishing() != 0 {
        return TRUE as u32;
    } else {
        return FALSE as u32;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
#[unsafe(no_mangle)]
pub unsafe fn Overworld_SendKeysToLinkIsRunning() -> u32 {
    if GetLinkSendQueueLength() < 2 {
        return FALSE as u32;
    } else if IsOverworldLinkActive() != TRUE as u32 {
        return FALSE as u32;
    } else if IsSendingKeysToLink() != TRUE as u32 {
        return FALSE as u32;
    } else if sPlayerKeyInterceptCallback
        == Some(KeyInterCB_DeferToSendQueue as unsafe fn(u32) -> u16)
    {
        return TRUE as u32;
    } else {
        return FALSE as u32;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
pub unsafe fn IsSendingKeysOverCable() -> u32 {
    if gWirelessCommType != 0 {
        return FALSE as u32;
    } else if IsSendingKeysToLink() == 0 {
        return FALSE as u32;
    } else {
        return TRUE as u32;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
unsafe fn GetLinkSendQueueLength() -> u32 {
    if gWirelessCommType != 0 {
        return (&raw mut gRfu.sendQueue.count).read_volatile() as u32;
    } else {
        return gLink.sendQueue.count as u32;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
unsafe fn ZeroLinkPlayerObjectEvent(linkPlayerObjEvent: *mut LinkPlayerObjectEvent) {
    memset(linkPlayerObjEvent as *mut u8, 0, 4);
}
pub unsafe fn ClearLinkPlayerObjectEvents() {
    memset(gLinkPlayerObjectEvents.as_mut_ptr() as *mut u8, 0, 16);
}
unsafe fn ZeroObjectEvent(objEvent: *mut ObjectEvent) {
    memset(objEvent as *mut u8, 0, 36);
}
unsafe fn SpawnLinkPlayerObjectEvent(linkPlayerId: u8, x: i16, y: i16, gender: u8) {
    let objEventId: u8 = GetFirstInactiveObjectEventId();
    let linkPlayerObjEvent: *mut LinkPlayerObjectEvent =
        &raw mut gLinkPlayerObjectEvents[linkPlayerId];
    let objEvent: *mut ObjectEvent = &raw mut gObjectEvents[objEventId];
    ZeroLinkPlayerObjectEvent(linkPlayerObjEvent);
    ZeroObjectEvent(objEvent);
    (*linkPlayerObjEvent).active = TRUE;
    (*linkPlayerObjEvent).linkPlayerId = linkPlayerId;
    (*linkPlayerObjEvent).objEventId = objEventId;
    (*linkPlayerObjEvent).movementMode = MOVEMENT_MODE_FREE;
    (*objEvent).set_active(TRUE as u32);
    (*objEvent).set_singleMovementActive(gender as u32);
    *(objEvent as *mut u8).at(25) = DIR_NORTH;
    (*objEvent).spriteId = MAX_SPRITES;
    InitLinkPlayerObjectEventPos(objEvent, x, y);
}
unsafe fn InitLinkPlayerObjectEventPos(objEvent: *mut ObjectEvent, x: i16, y: i16) {
    (*objEvent).currentCoords.x = x;
    (*objEvent).currentCoords.y = y;
    (*objEvent).previousCoords.x = x;
    (*objEvent).previousCoords.y = y;
    SetSpritePosToMapCoords(
        x,
        y,
        &raw mut (*objEvent).initialCoords.x,
        &raw mut (*objEvent).initialCoords.y,
    );
    (*objEvent).initialCoords.x += 8;
    ObjectEventUpdateElevation(objEvent);
}
unsafe fn SetLinkPlayerObjectRange(linkPlayerId: u8, dir: u8) {
    if gLinkPlayerObjectEvents[linkPlayerId].active != 0 {
        let objEventId: u8 = gLinkPlayerObjectEvents[linkPlayerId].objEventId;
        let objEvent: *mut ObjectEvent = &raw mut gObjectEvents[objEventId];
        *(objEvent as *mut u8).at(25) = dir;
    }
}
unsafe fn DestroyLinkPlayerObject(linkPlayerId: u8) {
    let linkPlayerObjEvent: *mut LinkPlayerObjectEvent =
        &raw mut gLinkPlayerObjectEvents[linkPlayerId];
    let objEventId: u8 = (*linkPlayerObjEvent).objEventId;
    let objEvent: *mut ObjectEvent = &raw mut gObjectEvents[objEventId];
    if (*objEvent).spriteId != MAX_SPRITES {
        DestroySprite(&raw mut gSprites[(*objEvent).spriteId]);
    }
    (*linkPlayerObjEvent).active = 0;
    (*objEvent).set_active(0);
}
unsafe fn GetSpriteForLinkedPlayer(linkPlayerId: u8) -> u8 {
    let objEventId: u8 = gLinkPlayerObjectEvents[linkPlayerId].objEventId;
    let objEvent: *mut ObjectEvent = &raw mut gObjectEvents[objEventId];
    (*objEvent).spriteId
}
unsafe fn GetLinkPlayerCoords(linkPlayerId: u8, x: *mut i16, y: *mut i16) {
    let objEventId: u8 = gLinkPlayerObjectEvents[linkPlayerId].objEventId;
    let objEvent: *mut ObjectEvent = &raw mut gObjectEvents[objEventId];
    *x = (*objEvent).currentCoords.x;
    *y = (*objEvent).currentCoords.y;
}
unsafe fn GetLinkPlayerFacingDirection(linkPlayerId: u8) -> u8 {
    let objEventId: u8 = gLinkPlayerObjectEvents[linkPlayerId].objEventId;
    let objEvent: *mut ObjectEvent = &raw mut gObjectEvents[objEventId];
    *(objEvent as *mut u8).at(25)
}
unsafe fn GetLinkPlayerElevation(linkPlayerId: u8) -> u8 {
    let objEventId: u8 = gLinkPlayerObjectEvents[linkPlayerId].objEventId;
    let objEvent: *mut ObjectEvent = &raw mut gObjectEvents[objEventId];
    (*objEvent).currentElevation()
}
unsafe fn GetLinkPlayerObjectStepTimer(linkPlayerId: u8) -> i16 {
    let objEventId: u8 = gLinkPlayerObjectEvents[linkPlayerId].objEventId;
    let objEvent: *mut ObjectEvent = &raw mut gObjectEvents[objEventId];
    16 - (*objEvent).directionSequenceIndex as i8 as i16
}
unsafe fn GetLinkPlayerIdAt(x: i16, y: i16) -> u8 {
    for i in 0..(MAX_LINK_PLAYERS as u8) {
        if gLinkPlayerObjectEvents[i].active != 0
            && (gLinkPlayerObjectEvents[i].movementMode == 0
                || gLinkPlayerObjectEvents[i].movementMode == 2)
        {
            let objEvent: *mut ObjectEvent =
                &raw mut gObjectEvents[gLinkPlayerObjectEvents[i].objEventId];
            if (*objEvent).currentCoords.x == x && (*objEvent).currentCoords.y == y {
                return i;
            }
        }
    }
    4
}
unsafe fn SetPlayerFacingDirection(linkPlayerId: u8, facing: u8) {
    let linkPlayerObjEvent: *mut LinkPlayerObjectEvent =
        &raw mut gLinkPlayerObjectEvents[linkPlayerId];
    let objEventId: u8 = (*linkPlayerObjEvent).objEventId;
    let objEvent: *mut ObjectEvent = &raw mut gObjectEvents[objEventId];
    if (*linkPlayerObjEvent).active == 0 {
        return;
    }
    if facing > FACING_FORCED_RIGHT as u8 {
        (*objEvent).set_triggerGroundEffectsOnMove(TRUE as u32);
        return;
    }
    sMovementStatusHandler[sLinkPlayerMovementModes[(*linkPlayerObjEvent).movementMode]
        .unwrap_unchecked()(linkPlayerObjEvent, objEvent, facing)]
    .unwrap_unchecked()(linkPlayerObjEvent, objEvent);
}
pub(crate) unsafe fn MovementEventModeCB_Normal(
    linkPlayerObjEvent: *mut LinkPlayerObjectEvent,
    objEvent: *mut ObjectEvent,
    dir: u8,
) -> u8 {
    sLinkPlayerFacingHandlers[dir].unwrap_unchecked()(linkPlayerObjEvent, objEvent, dir)
}
pub(crate) fn MovementEventModeCB_Ignored(
    linkPlayerObjEvent: *mut LinkPlayerObjectEvent,
    objEvent: *mut ObjectEvent,
    dir: u8,
) -> u8 {
    FACING_UP
}
pub(crate) unsafe fn MovementEventModeCB_Scripted(
    linkPlayerObjEvent: *mut LinkPlayerObjectEvent,
    objEvent: *mut ObjectEvent,
    dir: u8,
) -> u8 {
    sLinkPlayerFacingHandlers[dir].unwrap_unchecked()(linkPlayerObjEvent, objEvent, dir)
}
pub(crate) fn FacingHandler_DoNothing(
    linkPlayerObjEvent: *mut LinkPlayerObjectEvent,
    objEvent: *mut ObjectEvent,
    dir: u8,
) -> u8 {
    FALSE
}
pub(crate) unsafe fn FacingHandler_DpadMovement(
    linkPlayerObjEvent: *mut LinkPlayerObjectEvent,
    objEvent: *mut ObjectEvent,
    dir: u8,
) -> u8 {
    let mut x: i16 = 0;
    let mut y: i16 = 0;
    *(objEvent as *mut u8).at(25) = FlipVerticalAndClearForced(dir, *(objEvent as *mut u8).at(25));
    ObjectEventMoveDestCoords(
        objEvent,
        *(objEvent as *mut u8).at(25) as u32,
        &raw mut x,
        &raw mut y,
    );
    if LinkPlayerGetCollision(
        (*linkPlayerObjEvent).objEventId,
        *(objEvent as *mut u8).at(25),
        x,
        y,
    ) != 0
    {
        return FALSE;
    } else {
        (*objEvent).directionSequenceIndex = 16;
        ShiftObjectEventCoords(objEvent, x, y);
        ObjectEventUpdateElevation(objEvent);
        return TRUE;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
pub(crate) unsafe fn FacingHandler_ForcedFacingChange(
    linkPlayerObjEvent: *mut LinkPlayerObjectEvent,
    objEvent: *mut ObjectEvent,
    dir: u8,
) -> u8 {
    *(objEvent as *mut u8).at(25) = FlipVerticalAndClearForced(dir, *(objEvent as *mut u8).at(25));
    FALSE
}
pub(crate) unsafe fn MovementStatusHandler_EnterFreeMode(
    linkPlayerObjEvent: *mut LinkPlayerObjectEvent,
    objEvent: *mut ObjectEvent,
) {
    (*linkPlayerObjEvent).movementMode = MOVEMENT_MODE_FREE;
}
pub(crate) unsafe fn MovementStatusHandler_TryAdvanceScript(
    linkPlayerObjEvent: *mut LinkPlayerObjectEvent,
    objEvent: *mut ObjectEvent,
) {
    (*objEvent).directionSequenceIndex -= 1;
    (*linkPlayerObjEvent).movementMode = MOVEMENT_MODE_FROZEN;
    MoveCoords(
        *(objEvent as *mut u8).at(25),
        &raw mut (*objEvent).initialCoords.x,
        &raw mut (*objEvent).initialCoords.y,
    );
    if (*objEvent).directionSequenceIndex == 0 {
        ShiftStillObjectEventCoords(objEvent);
        (*linkPlayerObjEvent).movementMode = MOVEMENT_MODE_SCRIPTED;
    }
}
unsafe fn FlipVerticalAndClearForced(newFacing: u8, oldFacing: u8) -> u8 {
    match newFacing {
        FACING_UP | FACING_FORCED_UP => {
            return DIR_NORTH;
        }
        2 | FACING_FORCED_DOWN => {
            return DIR_SOUTH;
        }
        3 | 9 => {
            return DIR_WEST;
        }
        4 | 10 => {
            return DIR_EAST;
        }
        _ => {}
    }
    oldFacing
}
unsafe fn LinkPlayerGetCollision(selfObjEventId: u8, direction: u8, x: i16, y: i16) -> u8 {
    for i in 0..OBJECT_EVENTS_COUNT {
        if i != selfObjEventId
            && (gObjectEvents[i].currentCoords.x == x && gObjectEvents[i].currentCoords.y == y
                || gObjectEvents[i].previousCoords.x == x && gObjectEvents[i].previousCoords.y == y)
        {
            return 1;
        }
    }
    MapGridGetCollisionAt(x as i32, y as i32)
}
unsafe fn CreateLinkPlayerSprite(linkPlayerId: u8, gameVersion: u8) {
    let linkPlayerObjEvent: *mut LinkPlayerObjectEvent =
        &raw mut gLinkPlayerObjectEvents[linkPlayerId];
    let objEventId: u8 = (*linkPlayerObjEvent).objEventId;
    let objEvent: *mut ObjectEvent = &raw mut gObjectEvents[objEventId];
    let mut sprite: *mut Sprite = null_mut();
    if (*linkPlayerObjEvent).active != 0 {
        match gameVersion {
            4 | 5 => {
                (*objEvent).spriteId = CreateObjectGraphicsSprite(
                    GetFRLGAvatarGraphicsIdByGender((*objEvent).singleMovementActive() as u8)
                        as u16,
                    Some(SpriteCB_LinkPlayer),
                    0,
                    0,
                    0,
                );
            }
            2 | 1 => {
                (*objEvent).spriteId = CreateObjectGraphicsSprite(
                    GetRSAvatarGraphicsIdByGender((*objEvent).singleMovementActive() as u8) as u16,
                    Some(SpriteCB_LinkPlayer),
                    0,
                    0,
                    0,
                );
            }
            VERSION_EMERALD => {
                (*objEvent).spriteId = CreateObjectGraphicsSprite(
                    GetRivalAvatarGraphicsIdByStateIdAndGender(
                        PLAYER_AVATAR_STATE_NORMAL,
                        (*objEvent).singleMovementActive() as u8,
                    ) as u16,
                    Some(SpriteCB_LinkPlayer),
                    0,
                    0,
                    0,
                );
            }
            _ => {}
        }
        sprite = &raw mut gSprites[(*objEvent).spriteId];
        (*sprite).set_coordOffsetEnabled(TRUE as u16);
        (*sprite).data[0] = linkPlayerId as i16;
        (*objEvent).set_triggerGroundEffectsOnMove(FALSE as u32);
    }
}
pub(crate) unsafe fn SpriteCB_LinkPlayer(sprite: *mut Sprite) {
    let linkPlayerObjEvent: *mut LinkPlayerObjectEvent =
        &raw mut gLinkPlayerObjectEvents[(*sprite).data[0]];
    let objEvent: *mut ObjectEvent = &raw mut gObjectEvents[(*linkPlayerObjEvent).objEventId];
    (*sprite).x = (*objEvent).initialCoords.x;
    (*sprite).y = (*objEvent).initialCoords.y;
    SetObjectSubpriorityByElevation((*objEvent).previousElevation(), sprite, 1);
    (*sprite)
        .oam
        .set_priority(ElevationToPriority((*objEvent).previousElevation()) as u16);
    if (*linkPlayerObjEvent).movementMode == MOVEMENT_MODE_FREE {
        StartSpriteAnim(
            sprite,
            GetFaceDirectionAnimNum(*(objEvent as *mut u8).at(25)),
        );
    } else {
        StartSpriteAnimIfDifferent(
            sprite,
            GetMoveDirectionAnimNum(*(objEvent as *mut u8).at(25)),
        );
    }
    UpdateObjectEventSpriteInvisibility(sprite, FALSE);
    if (*objEvent).triggerGroundEffectsOnMove() != 0 {
        (*sprite).set_invisible((((*sprite).data[7] as i32 & 4) >> 2) as u16);
        (*sprite).data[7] += 1;
    }
}
