//! Translated from `src/overworld.c` by tools/rustport/c2rs.py.
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
    CArray<
        Option<unsafe extern "C" fn(*mut LinkPlayerObjectEvent, *mut ObjectEvent, u8) -> u8>,
        11,
    >,
> = Table((&raw const crate::data::overworld::sLinkPlayerFacingHandlers).cast());
static sLinkPlayerMovementModes: Table<
    CArray<Option<unsafe extern "C" fn(*mut LinkPlayerObjectEvent, *mut ObjectEvent, u8) -> u8>, 3>,
> = Table((&raw const crate::data::overworld::sLinkPlayerMovementModes).cast());
static sMovementStatusHandler: Table<
    CArray<Option<unsafe extern "C" fn(*mut LinkPlayerObjectEvent, *mut ObjectEvent)>, 2>,
> = Table((&raw const crate::data::overworld::sMovementStatusHandler).cast());
static sOverworldBgTemplates: Table<CArray<BgTemplate, 4>> =
    Table((&raw const crate::data::overworld::sOverworldBgTemplates).cast());

pub(crate) static mut sUnusedOverworldCallback: *mut c_void = null_mut();
pub(crate) static mut sPlayerLinkStates: Aligned<CArray<u8, 4>> = Aligned(unsafe { zeroed() });
pub(crate) static mut sPlayerKeyInterceptCallback: Option<unsafe extern "C" fn(u32) -> u16> = None;
pub(crate) static mut sReceivingFromLink: u8 = 0;
pub(crate) static mut sRfuKeepAliveTimer: u8 = 0;
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gOverworldTilemapBuffer_Bg2: *mut u16 = null_mut();
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gOverworldTilemapBuffer_Bg1: *mut u16 = null_mut();
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gOverworldTilemapBuffer_Bg3: *mut u16 = null_mut();
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gHeldKeyCodeToSend: u16 = 0;
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gFieldCallback: Option<unsafe extern "C" fn()> = None;
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gFieldCallback2: Option<unsafe extern "C" fn() -> u8> = None;
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gLocalLinkPlayerId: u8 = 0;
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gFieldLinkPlayerCount: u8 = 0;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sObjectEventLoadFlag: u8 = 0;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gLastUsedWarp: WarpData = unsafe { zeroed() };
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sWarpDestination: WarpData = unsafe { zeroed() };
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sFixedDiveWarp: WarpData = unsafe { zeroed() };
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sFixedHoleWarp: WarpData = unsafe { zeroed() };
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sLastMapSectionId: u16 = 0;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sInitialPlayerAvatarState: InitialPlayerAvatarState = unsafe { zeroed() };
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sAmbientCrySpecies: u16 = 0;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sIsAmbientCryWaterMon: u8 = 0;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gLinkPlayerObjectEvents: CArray<LinkPlayerObjectEvent, 4> = unsafe { zeroed() };

unsafe extern "C" {
    static CableClub_EventScript_ReadTrainerCard: CArray<u8, 0>;
    static CableClub_EventScript_ReadTrainerCardColored: CArray<u8, 0>;
    static CableClub_EventScript_TooBusyToNotice: CArray<u8, 0>;
    static EventScript_BattleColosseum_2P_PlayerSpot0: CArray<u8, 0>;
    static EventScript_BattleColosseum_2P_PlayerSpot1: CArray<u8, 0>;
    static EventScript_BattleColosseum_4P_PlayerSpot0: CArray<u8, 0>;
    static EventScript_BattleColosseum_4P_PlayerSpot1: CArray<u8, 0>;
    static EventScript_BattleColosseum_4P_PlayerSpot2: CArray<u8, 0>;
    static EventScript_BattleColosseum_4P_PlayerSpot3: CArray<u8, 0>;
    static EventScript_ConfirmLeaveCableClubRoom: CArray<u8, 0>;
    static EventScript_DoLinkRoomExit: CArray<u8, 0>;
    static EventScript_RecordCenter_Spot0: CArray<u8, 0>;
    static EventScript_RecordCenter_Spot1: CArray<u8, 0>;
    static EventScript_RecordCenter_Spot2: CArray<u8, 0>;
    static EventScript_RecordCenter_Spot3: CArray<u8, 0>;
    static EventScript_ResetMrBriney: CArray<u8, 0>;
    static EventScript_TerminateLink: CArray<u8, 0>;
    static EventScript_TradeCenter_Chair0: CArray<u8, 0>;
    static EventScript_TradeCenter_Chair1: CArray<u8, 0>;
    static EventScript_WhiteOut: CArray<u8, 0>;
    static mut gBackupMapLayout: BackupMapLayout;
    static mut gLink: Link;
    static mut gLinkPartnersHeldKeys: CArray<u16, 6>;
    static mut gLinkPlayers: CArray<LinkPlayer, 5>;
    static mut gMain: Main;
    static gMapGroups: CArray<*mut *mut MapHeader, 0>;
    static mut gMapHeader: MapHeader;
    static gMapLayouts: CArray<*mut MapLayout, 0>;
    static gMaxFlashLevel: i32;
    static mut gObjectEvents: CArray<ObjectEvent, 16>;
    static gOverworldBackgroundLayerFlags: CArray<u16, 0>;
    static mut gPaletteFade: PaletteFadeControl;
    static mut gPlayerAvatar: PlayerAvatar;
    static mut gPlayerParty: CArray<Pokemon, 6>;
    static mut gRfu: RfuManager;
    static mut gSaveBlock1Ptr: *mut SaveBlock1;
    static mut gSaveBlock2Ptr: *mut SaveBlock2;
    static mut gSaveFileStatus: u16;
    static mut gSprites: CArray<Sprite, 65>;
    static mut gTotalCameraPixelOffsetX: u16;
    static mut gTotalCameraPixelOffsetY: u16;
    static mut gWirelessCommType: u8;
    fn AllocZeroed(a0: u32) -> *mut c_void;
    fn AnimateSprites();
    fn ApplyNewEncryptionKeyToWord(a0: *mut u32, a1: u32);
    fn ApplyWeatherColorMapToPal(a0: u8);
    fn ArePlayerFieldControlsLocked() -> u8;
    fn BuildOamBuffer();
    fn CB2_DoChangeMap();
    fn CalculatePlayerPartyCount() -> u8;
    fn CameraUpdate();
    fn ChangeBgX(a0: u8, a1: i32, a2: u8) -> i32;
    fn ChangeBgY(a0: u8, a1: i32, a2: u8) -> i32;
    fn CheckLeftFriendsSecretBase();
    fn ClearContinueGameWarpStatus();
    fn ClearMirageTowerPulseBlend();
    fn ClearMirageTowerPulseBlendEffect();
    fn ClearScheduledBgCopiesToVram();
    fn ClearTempFieldEventData();
    fn CloseLink();
    fn CopyMapTilesetsToVram(a0: *mut MapLayout);
    fn CopyPrimaryTilesetToVram(a0: *mut MapLayout);
    fn CopySecondaryTilesetToVram(a0: *mut MapLayout);
    fn CopySecondaryTilesetToVramUsingHeap(a0: *mut MapLayout);
    fn CpuSet(a0: *mut c_void, a1: *mut c_void, a2: u32);
    fn CreateObjectGraphicsSprite(
        a0: u16,
        a1: Option<unsafe extern "C" fn(*mut Sprite)>,
        a2: i16,
        a3: i16,
        a4: u8,
    ) -> u8;
    fn CreateWirelessStatusIndicatorSprite(a0: u8, a1: u8);
    fn DestroySprite(a0: *mut Sprite);
    fn DisableInterrupts(a0: u16);
    fn DoCurrentWeather();
    fn DoScheduledBgTilemapCopiesToVram();
    fn DoTimeBasedEvents();
    fn DrawWholeMapView();
    fn ElevationToPriority(a0: u8) -> u8;
    fn EnableInterrupts(a0: u16);
    fn ExecuteTruckSequence();
    fn FadeOutAndFadeInNewMapMusic(a0: u16, a1: u8, a2: u8);
    fn FadeOutAndPlayNewMapMusic(a0: u16, a1: u8);
    fn FadeOutMapMusic(a0: u8);
    fn FieldCB_ContinueScript();
    fn FieldCB_ContinueScriptHandleMusic();
    fn FieldCB_DefaultWarpExit();
    fn FieldCB_ReturnToFieldCableLink();
    fn FieldCB_ReturnToFieldOpenStartMenu() -> u8;
    fn FieldCB_ReturnToFieldWirelessLink();
    fn FieldCB_WarpExitFadeFromBlack();
    fn FieldClearPlayerInput(a0: *mut FieldInput);
    fn FieldEffectActiveListClear();
    fn FieldGetPlayerInput(a0: *mut FieldInput, a1: u16, a2: u16);
    fn FieldUpdateBgTilemapScroll();
    fn FlagClear(a0: u16) -> u8;
    fn FlagGet(a0: u16) -> u8;
    fn Free(a0: *mut c_void);
    fn FreeAllOverworldWindowBuffers();
    fn FreeTempTileDataBuffersIfPossible() -> u8;
    fn GetCameraFocusCoords(a0: *mut u16, a1: *mut u16);
    fn GetCoordEventScriptAtMapPosition(a0: *mut MapPosition) -> *mut u8;
    fn GetCurrentMapMusic() -> u16;
    fn GetCurrentTrainerHillMapId() -> u8;
    fn GetFRLGAvatarGraphicsIdByGender(a0: u8) -> u8;
    fn GetFaceDirectionAnimNum(a0: u8) -> u8;
    fn GetFirstInactiveObjectEventId() -> u8;
    fn GetHealLocation(a0: u32) -> *mut HealLocation;
    fn GetInteractedLinkPlayerScript(a0: *mut MapPosition, a1: u8, a2: u8) -> *mut u8;
    fn GetLinkRecvQueueLength() -> u32;
    fn GetLinkTrainerCardColor(a0: u8) -> u32;
    fn GetLocalWaterMon() -> u16;
    fn GetLocalWildMon(a0: *mut u8) -> u16;
    fn GetMonAbility(a0: *mut Pokemon) -> u8;
    fn GetMonData2(a0: *mut Pokemon, a1: i32) -> u32;
    fn GetMoney(a0: *mut u32) -> u32;
    fn GetMoveDirectionAnimNum(a0: u8) -> u8;
    fn GetPlayerFacingDirection() -> u8;
    fn GetRSAvatarGraphicsIdByGender(a0: u8) -> u8;
    fn GetRivalAvatarGraphicsIdByStateIdAndGender(a0: u8, a1: u8) -> u8;
    fn GetSavedWeather() -> u8;
    fn HealPlayerParty();
    fn HideMapNamePopUpWindow();
    fn InBattlePyramid_() -> u8;
    fn InTrainerHill() -> u32;
    fn InitBattlePyramidMap(a0: u8);
    fn InitBgsFromTemplates(a0: u8, a1: *mut BgTemplate, a2: u8);
    fn InitCameraUpdateCallback(a0: u8) -> u32;
    fn InitFieldMessageBox();
    fn InitMap();
    fn InitMapFromSavedGame();
    fn InitMatchCallCounters();
    fn InitObjectEventPalettes(a0: u8);
    fn InitPlayerAvatar(a0: i16, a1: i16, a2: u8, a3: u8);
    fn InitSecondaryTilesetAnimation();
    fn InitSecretBaseAppearance(a0: u8);
    fn InitStandardTextBoxWindows();
    fn InitTextBoxGfxAndPrinters();
    fn InitTilesetAnimations();
    fn InitTrainerHillMap();
    fn InstallCameraPanAheadCallback();
    fn IsMirageIslandPresent() -> u8;
    fn IsNotWaitingForBGMStop() -> u8;
    fn IsRfuRecvQueueEmpty() -> u32;
    fn IsSendingKeysToLink() -> u32;
    fn LinkRfu_FatalError();
    fn LoadBattlePyramidFloorObjectEventScripts();
    fn LoadBattlePyramidObjectEventTemplates();
    fn LoadMapTilesetPalettes(a0: *mut MapLayout);
    fn LoadOam();
    fn LoadSecondaryTilesetPalette(a0: *mut MapLayout);
    fn LoadTrainerHillFloorObjectEventScripts() -> u32;
    fn LoadTrainerHillObjectEventTemplates();
    fn LoadWirelessStatusIndicatorSpriteGfx();
    fn LockPlayerFieldControls();
    fn MapGridGetCollisionAt(a0: i32, a1: i32) -> u8;
    fn MapGridGetMetatileBehaviorAt(a0: i32, a1: i32) -> i32;
    fn MetatileBehavior_IsDeepSouthWarp(a0: u8) -> u8;
    fn MetatileBehavior_IsDoor(a0: u8) -> u8;
    fn MetatileBehavior_IsEastArrowWarp(a0: u8) -> u8;
    fn MetatileBehavior_IsLadder(a0: u8) -> u8;
    fn MetatileBehavior_IsNonAnimDoor(a0: u8) -> u8;
    fn MetatileBehavior_IsNorthArrowWarp(a0: u8) -> u8;
    fn MetatileBehavior_IsSouthArrowWarp(a0: u8) -> u8;
    fn MetatileBehavior_IsSurfableWaterOrUnderwater(a0: u8) -> u8;
    fn MetatileBehavior_IsWestArrowWarp(a0: u8) -> u8;
    fn MoveCoords(a0: u8, a1: *mut i16, a2: *mut i16);
    fn MoveSaveBlocks_ResetHeap();
    fn NewGameInitData();
    fn ObjectEventMoveDestCoords(a0: *mut ObjectEvent, a1: u32, a2: *mut i16, a3: *mut i16);
    fn ObjectEventUpdateElevation(a0: *mut ObjectEvent);
    fn PlayCry_NormalNoDucking(a0: u16, a1: i8, a2: i8, a3: u8);
    fn PlayNewMapMusic(a0: u16);
    fn PlaySE(a0: u16);
    fn PlayTimeCounter_Start();
    fn PlayerGetDestCoords(a0: *mut i16, a1: *mut i16);
    fn PlayerStep(a0: u8, a1: u16, a2: u16);
    fn ProcessPlayerFieldInput(a0: *mut FieldInput) -> i32;
    fn ProcessSpriteCopyRequests();
    fn Random() -> u16;
    fn ResetAllPicSprites() -> u16;
    fn ResetCameraUpdateInfo();
    fn ResetCyclingRoadChallengeData();
    fn ResetFieldCamera();
    fn ResetFieldTasksArgs();
    fn ResetMapMusic();
    fn ResetOamRange(a0: u8, a1: u8);
    fn ResetObjectEvents();
    fn ResetPaletteFade();
    fn ResetSafariZoneFlag();
    fn ResetSpriteData();
    fn ResetTasks();
    fn ResetTempTileDataBuffers();
    fn ResetWinStreaks();
    fn RestartWildEncounterImmunitySteps();
    fn ResumePausedWeather();
    fn RoamerMove();
    fn RoamerMoveToOtherLocationSet();
    fn RotatingGate_InitPuzzleAndGraphics();
    fn RunOnDiveWarpMapScript();
    fn RunOnResumeMapScript();
    fn RunOnReturnToFieldMapScript();
    fn RunOnTransitionMapScript();
    fn RunScriptImmediately(a0: *mut u8);
    fn RunTasks();
    fn ScanlineEffect_Clear();
    fn ScanlineEffect_InitHBlankDmaTransfer();
    fn ScanlineEffect_SetParams(a0: ScanlineEffectParams);
    fn ScanlineEffect_Stop();
    fn ScheduleBgCopyTilemapToVram(a0: u8);
    fn ScriptContext_Init();
    fn ScriptContext_RunScript() -> u8;
    fn ScriptContext_SetupScript(a0: *mut u8);
    fn SecretBaseMapPopupEnabled() -> u8;
    fn SetBgAttribute(a0: u8, a1: u8, a2: u8);
    fn SetBgTilemapBuffer(a0: u8, a1: *mut c_void);
    fn SetCameraFocusCoords(a0: u16, a1: u16);
    fn SetGpuReg(a0: u8, a1: u16);
    fn SetHBlankCallback(a0: Option<unsafe extern "C" fn()>);
    fn SetMainCallback2(a0: Option<unsafe extern "C" fn()>);
    fn SetMoney(a0: *mut u32, a1: u32);
    fn SetObjectSubpriorityByElevation(a0: u8, a1: *mut Sprite, a2: u8);
    fn SetPlayerAvatarTransitionFlags(a0: u16);
    fn SetSavedWeatherFromCurrMapHeader();
    fn SetSpritePosToMapCoords(a0: i16, a1: i16, a2: *mut i16, a3: *mut i16);
    fn SetUpFieldTasks();
    fn SetVBlankCallback(a0: Option<unsafe extern "C" fn()>);
    fn ShiftObjectEventCoords(a0: *mut ObjectEvent, a1: i16, a2: i16);
    fn ShiftStillObjectEventCoords(a0: *mut ObjectEvent);
    fn ShowBg(a0: u8);
    fn ShowMapNamePopup();
    fn ShowStartMenu();
    fn SpawnObjectEventsOnReturnToField(a0: i16, a1: i16);
    fn StartSpriteAnim(a0: *mut Sprite, a1: u8);
    fn StartSpriteAnimIfDifferent(a0: *mut Sprite, a1: u8);
    fn StartWeather();
    fn StopMapMusic();
    fn TestPlayerAvatarFlags(a0: u8) -> u8;
    fn TransferPlttBuffer();
    fn TransferTilesetAnimsBuffer();
    fn TryLoadTrainerHillEReaderPalette();
    fn TryPutTodaysRivalTrainerOnAir();
    fn TryRunOnWarpIntoMapScript();
    fn TrySetMapSaveWarpStatus();
    fn TrySpawnObjectEvents(a0: i16, a1: i16);
    fn TryStartMirageTowerPulseBlendEffect();
    fn TryUpdateRandomTrainerRematches(a0: u16, a1: u16);
    fn UnfreezeObjectEvents();
    fn UnlockPlayerFieldControls();
    fn UpdateCameraPanning();
    fn UpdateLocationHistoryForRoamer();
    fn UpdateObjectEventSpriteInvisibility(a0: *mut Sprite, a1: u8);
    fn UpdatePaletteFade() -> u8;
    fn UpdatePlayerAvatarTransitionState();
    fn UpdateTVScreensOnMap(a0: i32, a1: i32);
    fn UpdateTilesetAnimations();
    fn UseContinueGameWarp() -> u32;
    fn UsedPokemonCenterWarp() -> u8;
    fn VarGet(a0: u16) -> u16;
    fn VarSet(a0: u16, a1: u16) -> u8;
    fn WriteBattlePyramidViewScanlineEffectBuffer();
    fn WriteFlashScanlineEffectBuffer(a0: u8);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn DoWhiteOut() {
    RunScriptImmediately(EventScript_WhiteOut.as_ptr().cast_mut());
    SetMoney(
        &raw mut (*gSaveBlock1Ptr).money,
        GetMoney(&raw mut (*gSaveBlock1Ptr).money) / 2,
    );
    HealPlayerParty();
    Overworld_ResetStateAfterWhiteOut();
    SetWarpDestinationToLastHealLocation();
    WarpIntoMap();
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Overworld_ResetStateAfterFly() {
    ResetInitialPlayerAvatarState();
    FlagClear(FLAG_SYS_CYCLING_ROAD);
    FlagClear(FLAG_SYS_CRUISE_MODE);
    FlagClear(FLAG_SYS_SAFARI_MODE);
    FlagClear(FLAG_SYS_USE_STRENGTH);
    FlagClear(FLAG_SYS_USE_FLASH);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Overworld_ResetStateAfterTeleport() {
    ResetInitialPlayerAvatarState();
    FlagClear(FLAG_SYS_CYCLING_ROAD);
    FlagClear(FLAG_SYS_CRUISE_MODE);
    FlagClear(FLAG_SYS_SAFARI_MODE);
    FlagClear(FLAG_SYS_USE_STRENGTH);
    FlagClear(FLAG_SYS_USE_FLASH);
    RunScriptImmediately(EventScript_ResetMrBriney.as_ptr().cast_mut());
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Overworld_ResetStateAfterDigEscRope() {
    ResetInitialPlayerAvatarState();
    FlagClear(FLAG_SYS_CYCLING_ROAD);
    FlagClear(FLAG_SYS_CRUISE_MODE);
    FlagClear(FLAG_SYS_SAFARI_MODE);
    FlagClear(FLAG_SYS_USE_STRENGTH);
    FlagClear(FLAG_SYS_USE_FLASH);
}
pub(crate) unsafe extern "C" fn Overworld_ResetStateAfterWhiteOut() {
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
pub(crate) unsafe extern "C" fn UpdateMiscOverworldStates() {
    FlagClear(FLAG_SYS_SAFARI_MODE);
    ChooseAmbientCrySpecies();
    ResetCyclingRoadChallengeData();
    UpdateLocationHistoryForRoamer();
    RoamerMoveToOtherLocationSet();
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ResetGameStats() {
    let mut i: i32 = 0;
    i = 0;
    while i < NUM_GAME_STATS {
        SetGameStat(i as u8, 0);
        i += 1;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IncrementGameStat(index: u8) {
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
pub unsafe extern "C" fn GetGameStat(index: u8) -> u32 {
    if index >= NUM_USED_GAME_STATS {
        return 0;
    }
    return (*gSaveBlock1Ptr).gameStats[index] ^ (*gSaveBlock2Ptr).encryptionKey;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetGameStat(index: u8, value: u32) {
    if index < NUM_USED_GAME_STATS {
        (*gSaveBlock1Ptr).gameStats[index] = value ^ (*gSaveBlock2Ptr).encryptionKey;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ApplyNewEncryptionKeyToGameStats(newKey: u32) {
    let mut i: u8 = 0;
    i = 0;
    while i < NUM_GAME_STATS as u8 {
        ApplyNewEncryptionKeyToWord(&raw mut (*gSaveBlock1Ptr).gameStats[i], newKey);
        i += 1;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn LoadObjEventTemplatesFromHeader() {
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
        0x04000000 | (*gMapHeader.events).objectEventCount as u32 * 24 / 4 & 0x1FFFFF,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn LoadSaveblockObjEventScripts() {
    let mut mapHeaderObjTemplates: *mut ObjectEventTemplate = (*gMapHeader.events).objectEvents;
    let mut savObjTemplates: *mut ObjectEventTemplate =
        (*gSaveBlock1Ptr).objectEventTemplates.as_mut_ptr();
    let mut i: i32 = 0;
    i = 0;
    while i < OBJECT_EVENT_TEMPLATES_COUNT {
        (*savObjTemplates.at(i)).script = (*mapHeaderObjTemplates.at(i)).script;
        i += 1;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetObjEventTemplateCoords(localId: u8, x: i16, y: i16) {
    let mut i: i32 = 0;
    let mut savObjTemplates: *mut ObjectEventTemplate =
        (*gSaveBlock1Ptr).objectEventTemplates.as_mut_ptr();
    i = 0;
    while i < OBJECT_EVENT_TEMPLATES_COUNT {
        let mut objectEventTemplate: *mut ObjectEventTemplate = savObjTemplates.at(i);
        if (*objectEventTemplate).localId == localId {
            (*objectEventTemplate).x = x;
            (*objectEventTemplate).y = y;
            return;
        }
        i += 1;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetObjEventTemplateMovementType(localId: u8, movementType: u8) {
    let mut i: i32 = 0;
    let mut savObjTemplates: *mut ObjectEventTemplate =
        (*gSaveBlock1Ptr).objectEventTemplates.as_mut_ptr();
    i = 0;
    while i < OBJECT_EVENT_TEMPLATES_COUNT {
        let mut objectEventTemplate: *mut ObjectEventTemplate = savObjTemplates.at(i);
        if (*objectEventTemplate).localId == localId {
            (*objectEventTemplate).movementType = movementType;
            return;
        }
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn InitMapView() {
    ResetFieldCamera();
    CopyMapTilesetsToVram(gMapHeader.mapLayout);
    LoadMapTilesetPalettes(gMapHeader.mapLayout);
    DrawWholeMapView();
    InitTilesetAnimations();
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetMapLayout() -> *mut MapLayout {
    let mut mapLayoutId: u16 = (*gSaveBlock1Ptr).mapLayoutId;
    if mapLayoutId != 0 {
        return gMapLayouts[mapLayoutId as i32 - 1];
    }
    return null_mut();
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ApplyCurrentWarp() {
    gLastUsedWarp = (*gSaveBlock1Ptr).location;
    (*gSaveBlock1Ptr).location = sWarpDestination;
    sFixedDiveWarp = *sDummyWarpData;
    sFixedHoleWarp = *sDummyWarpData;
}
pub(crate) unsafe extern "C" fn ClearDiveAndHoleWarps() {
    sFixedDiveWarp = *sDummyWarpData;
    sFixedHoleWarp = *sDummyWarpData;
}
pub(crate) unsafe extern "C" fn SetWarpData(
    warp: *mut WarpData,
    mapGroup: i8,
    mapNum: i8,
    warpId: i8,
    x: i8,
    y: i8,
) {
    (*warp).mapGroup = mapGroup;
    (*warp).mapNum = mapNum;
    (*warp).warpId = warpId;
    (*warp).x = x as i16;
    (*warp).y = y as i16;
}
pub(crate) unsafe extern "C" fn IsDummyWarp(warp: *mut WarpData) -> u32 {
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
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Overworld_GetMapHeaderByGroupAndId(
    mapGroup: u16,
    mapNum: u16,
) -> *mut MapHeader {
    return *gMapGroups[mapGroup].at(mapNum);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetDestinationWarpMapHeader() -> *mut MapHeader {
    return Overworld_GetMapHeaderByGroupAndId(
        sWarpDestination.mapGroup as u16,
        sWarpDestination.mapNum as u16,
    );
}
pub(crate) unsafe extern "C" fn LoadCurrentMapData() {
    sLastMapSectionId = gMapHeader.regionMapSectionId as u16;
    gMapHeader = *Overworld_GetMapHeaderByGroupAndId(
        (*gSaveBlock1Ptr).location.mapGroup as u16,
        (*gSaveBlock1Ptr).location.mapNum as u16,
    );
    (*gSaveBlock1Ptr).mapLayoutId = gMapHeader.mapLayoutId;
    gMapHeader.mapLayout = GetMapLayout();
}
pub(crate) unsafe extern "C" fn LoadSaveblockMapHeader() {
    gMapHeader = *Overworld_GetMapHeaderByGroupAndId(
        (*gSaveBlock1Ptr).location.mapGroup as u16,
        (*gSaveBlock1Ptr).location.mapNum as u16,
    );
    gMapHeader.mapLayout = GetMapLayout();
}
pub(crate) unsafe extern "C" fn SetPlayerCoordsFromWarp() {
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
pub unsafe extern "C" fn WarpIntoMap() {
    ApplyCurrentWarp();
    LoadCurrentMapData();
    SetPlayerCoordsFromWarp();
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetWarpDestination(mapGroup: i8, mapNum: i8, warpId: i8, x: i8, y: i8) {
    SetWarpData(&raw mut sWarpDestination, mapGroup, mapNum, warpId, x, y);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetWarpDestinationToMapWarp(mapGroup: i8, mapNum: i8, warpId: i8) {
    SetWarpDestination(mapGroup, mapNum, warpId, -1, -1);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetDynamicWarp(unused: i32, mapGroup: i8, mapNum: i8, warpId: i8) {
    SetWarpData(
        &raw mut (*gSaveBlock1Ptr).dynamicWarp,
        mapGroup,
        mapNum,
        warpId,
        (*gSaveBlock1Ptr).pos.x as i8,
        (*gSaveBlock1Ptr).pos.y as i8,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetDynamicWarpWithCoords(
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
pub unsafe extern "C" fn SetWarpDestinationToDynamicWarp(unusedWarpId: u8) {
    sWarpDestination = (*gSaveBlock1Ptr).dynamicWarp;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetWarpDestinationToHealLocation(healLocationId: u8) {
    let mut healLocation: *mut HealLocation = GetHealLocation(healLocationId as u32);
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetWarpDestinationToLastHealLocation() {
    sWarpDestination = (*gSaveBlock1Ptr).lastHealLocation;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetLastHealLocationWarp(healLocationId: u8) {
    let mut healLocation: *mut HealLocation = GetHealLocation(healLocationId as u32);
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn UpdateEscapeWarp(x: i16, y: i16) {
    let mut currMapType: u8 = GetCurrentMapType();
    let mut destMapType: u8 =
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetEscapeWarp(mapGroup: i8, mapNum: i8, warpId: i8, x: i8, y: i8) {
    SetWarpData(
        &raw mut (*gSaveBlock1Ptr).escapeWarp,
        mapGroup,
        mapNum,
        warpId,
        x,
        y,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetWarpDestinationToEscapeWarp() {
    sWarpDestination = (*gSaveBlock1Ptr).escapeWarp;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetFixedDiveWarp(mapGroup: i8, mapNum: i8, warpId: i8, x: i8, y: i8) {
    SetWarpData(&raw mut sFixedDiveWarp, mapGroup, mapNum, warpId, x, y);
}
pub(crate) unsafe extern "C" fn SetWarpDestinationToDiveWarp() {
    sWarpDestination = sFixedDiveWarp;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetFixedHoleWarp(mapGroup: i8, mapNum: i8, warpId: i8, x: i8, y: i8) {
    SetWarpData(&raw mut sFixedHoleWarp, mapGroup, mapNum, warpId, x, y);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetWarpDestinationToFixedHoleWarp(x: i16, y: i16) {
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
pub(crate) unsafe extern "C" fn SetWarpDestinationToContinueGameWarp() {
    sWarpDestination = (*gSaveBlock1Ptr).continueGameWarp;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetContinueGameWarp(mapGroup: i8, mapNum: i8, warpId: i8, x: i8, y: i8) {
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
pub unsafe extern "C" fn SetContinueGameWarpToHealLocation(healLocationId: u8) {
    let mut healLocation: *mut HealLocation = GetHealLocation(healLocationId as u32);
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
pub unsafe extern "C" fn SetContinueGameWarpToDynamicWarp(unused: i32) {
    (*gSaveBlock1Ptr).continueGameWarp = (*gSaveBlock1Ptr).dynamicWarp;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetMapConnection(dir: u8) -> *mut MapConnection {
    let mut i: i32 = 0;
    let mut count: i32 = (*gMapHeader.connections).count;
    let mut connection: *mut MapConnection = (*gMapHeader.connections).connections;
    if connection.is_null() {
        return null_mut();
    }
    i = 0;
    while i < count {
        if (*connection).direction == dir {
            return connection;
        }
        i += 1;
        connection = connection.at(1);
    }
    return null_mut();
}
pub(crate) unsafe extern "C" fn SetDiveWarp(dir: u8, x: u16, y: u16) -> u8 {
    let mut connection: *mut MapConnection = GetMapConnection(dir);
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
    return TRUE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetDiveWarpEmerge(x: u16, y: u16) -> u8 {
    return SetDiveWarp(CONNECTION_EMERGE, x, y);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetDiveWarpDive(x: u16, y: u16) -> u8 {
    return SetDiveWarp(CONNECTION_DIVE, x, y);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn LoadMapFromCameraTransition(mapGroup: u8, mapNum: u8) {
    let mut paletteIndex: i32 = 0;
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
    paletteIndex = NUM_PALS_IN_PRIMARY;
    while paletteIndex < NUM_PALS_TOTAL {
        ApplyWeatherColorMapToPal(paletteIndex as u8);
        paletteIndex += 1;
    }
    InitSecondaryTilesetAnimation();
    UpdateLocationHistoryForRoamer();
    RoamerMove();
    DoCurrentWeather();
    ResetFieldTasksArgs();
    RunOnResumeMapScript();
    if gMapHeader.regionMapSectionId != MAPSEC_BATTLE_FRONTIER
        || gMapHeader.regionMapSectionId as u16 != sLastMapSectionId
    {
        ShowMapNamePopup();
    }
}
pub(crate) unsafe extern "C" fn LoadMapFromWarp(a1: u32) {
    let mut isOutdoors: u8 = 0;
    let mut isIndoors: u8 = 0;
    LoadCurrentMapData();
    if sObjectEventLoadFlag as i32 & SKIP_OBJECT_EVENT_LOAD as i32 == 0 {
        if gMapHeader.mapLayoutId == LAYOUT_BATTLE_FRONTIER_BATTLE_PYRAMID_FLOOR {
            LoadBattlePyramidObjectEventTemplates();
        } else if InTrainerHill() != 0 {
            LoadTrainerHillObjectEventTemplates();
        } else {
            LoadObjEventTemplatesFromHeader();
        }
    }
    isOutdoors = IsMapTypeOutdoors(gMapHeader.mapType);
    isIndoors = IsMapTypeIndoors(gMapHeader.mapType);
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ResetInitialPlayerAvatarState() {
    sInitialPlayerAvatarState.direction = DIR_SOUTH;
    sInitialPlayerAvatarState.transitionFlags = PLAYER_AVATAR_FLAG_ON_FOOT;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn StoreInitialPlayerAvatarState() {
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
pub(crate) unsafe extern "C" fn GetInitialPlayerAvatarState() -> *mut InitialPlayerAvatarState {
    let mut playerStruct: InitialPlayerAvatarState = zeroed();
    let mut mapType: u8 = GetCurrentMapType();
    let mut metatileBehavior: u16 = GetCenterScreenMetatileBehavior();
    let mut transitionFlags: u8 = GetAdjustedInitialTransitionFlags(
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
    return &raw mut sInitialPlayerAvatarState;
}
pub(crate) unsafe extern "C" fn GetAdjustedInitialTransitionFlags(
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
        return 0;
    }
}
pub(crate) unsafe extern "C" fn GetAdjustedInitialDirection(
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
        return 0;
    }
}
pub(crate) unsafe extern "C" fn GetCenterScreenMetatileBehavior() -> u16 {
    return MapGridGetMetatileBehaviorAt(
        (*gSaveBlock1Ptr).pos.x as i32 + MAP_OFFSET,
        (*gSaveBlock1Ptr).pos.y as i32 + MAP_OFFSET,
    ) as u16;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Overworld_IsBikingAllowed() -> u32 {
    if gMapHeader.allowCycling() == 0 {
        return FALSE as u32;
    } else {
        return TRUE as u32;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetDefaultFlashLevel() {
    if gMapHeader.cave == 0 {
        (*gSaveBlock1Ptr).flashLevel = 0;
    } else if FlagGet(FLAG_SYS_USE_FLASH) != 0 {
        (*gSaveBlock1Ptr).flashLevel = 1;
    } else {
        (*gSaveBlock1Ptr).flashLevel = gMaxFlashLevel as u8 - 1;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetFlashLevel(mut flashLevel: i32) {
    if flashLevel < 0 || flashLevel > gMaxFlashLevel {
        flashLevel = 0;
    }
    (*gSaveBlock1Ptr).flashLevel = flashLevel as u8;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetFlashLevel() -> u8 {
    return (*gSaveBlock1Ptr).flashLevel;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetCurrentMapLayout(mapLayoutId: u16) {
    (*gSaveBlock1Ptr).mapLayoutId = mapLayoutId;
    gMapHeader.mapLayout = GetMapLayout();
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetObjectEventLoadFlag(flag: u8) {
    sObjectEventLoadFlag = flag;
}
pub(crate) unsafe extern "C" fn GetObjectEventLoadFlag() -> u8 {
    return sObjectEventLoadFlag;
}
pub(crate) unsafe extern "C" fn ShouldLegendaryMusicPlayAtLocation(warp: *mut WarpData) -> u16 {
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
                match (*warp).mapNum {
                    44 | 45 | 46 => {
                        return TRUE as u16;
                    }
                    _ => {}
                }
            }
        }
    }
    return FALSE as u16;
}
pub(crate) unsafe extern "C" fn NoMusicInSootopolisWithLegendaries(warp: *mut WarpData) -> u16 {
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
        return 0;
    }
}
pub(crate) unsafe extern "C" fn IsInfiltratedWeatherInstitute(warp: *mut WarpData) -> u16 {
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
        return 0;
    }
}
pub(crate) unsafe extern "C" fn IsInfiltratedSpaceCenter(warp: *mut WarpData) -> u16 {
    if VarGet(VAR_MOSSDEEP_CITY_STATE) == 0 {
        return FALSE as u16;
    } else if VarGet(VAR_MOSSDEEP_CITY_STATE) > 2 {
        return FALSE as u16;
    } else if (*warp).mapGroup != 14 {
        return FALSE as u16;
    } else if (*warp).mapNum == 9 || (*warp).mapNum == 10 {
        return TRUE as u16;
    }
    return FALSE as u16;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetLocationMusic(warp: *mut WarpData) -> u16 {
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
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetCurrLocationDefaultMusic() -> u16 {
    let mut music: u16 = 0;
    if (*gSaveBlock1Ptr).location.mapGroup == 0
        && (*gSaveBlock1Ptr).location.mapNum == 26
        && GetSavedWeather() == WEATHER_SANDSTORM
    {
        return MUS_DESERT;
    }
    music = GetLocationMusic(&raw mut (*gSaveBlock1Ptr).location);
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
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetWarpDestinationMusic() -> u16 {
    let mut music: u16 = GetLocationMusic(&raw mut sWarpDestination);
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
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Overworld_ResetMapMusic() {
    ResetMapMusic();
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Overworld_PlaySpecialMapMusic() {
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Overworld_SetSavedMusic(songNum: u16) {
    (*gSaveBlock1Ptr).savedMusic = songNum;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Overworld_ClearSavedMusic() {
    (*gSaveBlock1Ptr).savedMusic = MUS_DUMMY;
}
pub(crate) unsafe extern "C" fn TransitionMapMusic() {
    if FlagGet(FLAG_DONT_TRANSITION_MUSIC) != TRUE {
        let mut newMusic: u16 = GetWarpDestinationMusic();
        let mut currentMusic: u16 = GetCurrentMapMusic();
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Overworld_ChangeMusicToDefault() {
    let mut currentMusic: u16 = GetCurrentMapMusic();
    if currentMusic != GetCurrLocationDefaultMusic() {
        FadeOutAndPlayNewMapMusic(GetCurrLocationDefaultMusic(), 8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Overworld_ChangeMusicTo(newMusic: u16) {
    let mut currentMusic: u16 = GetCurrentMapMusic();
    if currentMusic != newMusic && currentMusic != MUS_ABNORMAL_WEATHER {
        FadeOutAndPlayNewMapMusic(newMusic, 8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetMapMusicFadeoutSpeed() -> u8 {
    let mut mapHeader: *mut MapHeader = GetDestinationWarpMapHeader();
    if IsMapTypeIndoors((*mapHeader).mapType) == TRUE {
        return 2;
    } else {
        return 4;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn TryFadeOutOldMapMusic() {
    let mut currentMusic: u16 = GetCurrentMapMusic();
    let mut warpMusic: u16 = GetWarpDestinationMusic();
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BGMusicStopped() -> u8 {
    return IsNotWaitingForBGMStop();
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Overworld_FadeOutMapMusic() {
    FadeOutMapMusic(4);
}
pub(crate) unsafe extern "C" fn PlayAmbientCry() {
    let mut x: i16 = 0;
    let mut y: i16 = 0;
    let mut pan: i8 = 0;
    let mut volume: i8 = 0;
    PlayerGetDestCoords(&raw mut x, &raw mut y);
    if sIsAmbientCryWaterMon == TRUE
        && MetatileBehavior_IsSurfableWaterOrUnderwater(MapGridGetMetatileBehaviorAt(
            x as i32, y as i32,
        ) as u8)
            == 0
    {
        return;
    }
    pan = (Random() as i32 % 88) as i8 + -44;
    volume = (Random() as i32 % 30) as i8 + 50;
    PlayCry_NormalNoDucking(sAmbientCrySpecies, pan, volume, CRY_PRIORITY_AMBIENT);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn UpdateAmbientCry(state: *mut i16, delayCounter: *mut u16) {
    let mut i: u8 = 0;
    let mut monsCount: u8 = 0;
    let mut divBy: u8 = 0;
    match *state {
        AMB_CRY_INIT => {
            if sAmbientCrySpecies == SPECIES_NONE {
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
            i = 0;
            while i < monsCount {
                if GetMonData2(&raw mut gPlayerParty[i], MON_DATA_SANITY_IS_EGG) == 0
                    && GetMonAbility(&raw mut gPlayerParty[0]) == ABILITY_SWARM
                {
                    divBy = 2;
                    break;
                }
                i += 1;
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
pub(crate) unsafe extern "C" fn ChooseAmbientCrySpecies() {
    if (*gSaveBlock1Ptr).location.mapGroup == 0
        && (*gSaveBlock1Ptr).location.mapNum == 45
        && IsMirageIslandPresent() == 0
    {
        sIsAmbientCryWaterMon = TRUE;
        sAmbientCrySpecies = GetLocalWaterMon();
    } else {
        sAmbientCrySpecies = GetLocalWildMon(&raw mut sIsAmbientCryWaterMon);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetMapTypeByGroupAndId(mapGroup: i8, mapNum: i8) -> u8 {
    return (*Overworld_GetMapHeaderByGroupAndId(mapGroup as u16, mapNum as u16)).mapType;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetMapTypeByWarpData(warp: *mut WarpData) -> u8 {
    return GetMapTypeByGroupAndId((*warp).mapGroup, (*warp).mapNum);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetCurrentMapType() -> u8 {
    return GetMapTypeByWarpData(&raw mut (*gSaveBlock1Ptr).location);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetLastUsedWarpMapType() -> u8 {
    return GetMapTypeByWarpData(&raw mut gLastUsedWarp);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsMapTypeOutdoors(mapType: u8) -> u8 {
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
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Overworld_MapTypeAllowsTeleportAndFly(mapType: u8) -> u8 {
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
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsMapTypeIndoors(mapType: u8) -> u8 {
    if mapType == MAP_TYPE_INDOOR || mapType == MAP_TYPE_SECRET_BASE {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetSavedWarpRegionMapSectionId() -> u8 {
    return (*Overworld_GetMapHeaderByGroupAndId(
        (*gSaveBlock1Ptr).dynamicWarp.mapGroup as u16,
        (*gSaveBlock1Ptr).dynamicWarp.mapNum as u16,
    ))
    .regionMapSectionId;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetCurrentRegionMapSectionId() -> u8 {
    return (*Overworld_GetMapHeaderByGroupAndId(
        (*gSaveBlock1Ptr).location.mapGroup as u16,
        (*gSaveBlock1Ptr).location.mapNum as u16,
    ))
    .regionMapSectionId;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetCurrentMapBattleScene() -> u8 {
    return (*Overworld_GetMapHeaderByGroupAndId(
        (*gSaveBlock1Ptr).location.mapGroup as u16,
        (*gSaveBlock1Ptr).location.mapNum as u16,
    ))
    .battleType;
}
pub(crate) unsafe extern "C" fn InitOverworldBgs() {
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CleanupOverworldWindowsAndTilemaps() {
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
pub(crate) unsafe extern "C" fn ResetSafariZoneFlag_() {
    ResetSafariZoneFlag();
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsOverworldLinkActive() -> u32 {
    if gMain.callback1 == Some(CB1_OverworldLink as unsafe extern "C" fn()) {
        return TRUE as u32;
    } else {
        return FALSE as u32;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn DoCB1_Overworld(newKeys: u16, heldKeys: u16) {
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CB1_Overworld() {
    if gMain.callback2 == Some(CB2_Overworld as unsafe extern "C" fn()) {
        DoCB1_Overworld(gMain.newKeys, gMain.heldKeys);
    }
}
pub(crate) unsafe extern "C" fn OverworldBasic() {
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CB2_OverworldBasic() {
    OverworldBasic();
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CB2_Overworld() {
    let mut fading: u32 = (gPaletteFade.active() != 0) as u32;
    if fading != 0 {
        SetVBlankCallback(None);
    }
    OverworldBasic();
    if fading != 0 {
        SetFieldVBlankCallback();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetMainCallback1(cb: Option<unsafe extern "C" fn()>) {
    gMain.callback1 = cb;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetUnusedCallback(func: *mut c_void) {
    sUnusedOverworldCallback = func;
}
pub(crate) unsafe extern "C" fn RunFieldCallback() -> u8 {
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
    return TRUE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CB2_NewGame() {
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
pub unsafe extern "C" fn CB2_WhiteOut() {
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
pub unsafe extern "C" fn CB2_LoadMap() {
    FieldClearVBlankHBlankCallbacks();
    ScriptContext_Init();
    UnlockPlayerFieldControls();
    SetMainCallback1(None);
    SetMainCallback2(Some(CB2_DoChangeMap));
    gMain.savedCallback = Some(CB2_LoadMap2);
}
pub(crate) unsafe extern "C" fn CB2_LoadMap2() {
    DoMapLoadLoop(&raw mut gMain.state);
    SetFieldVBlankCallback();
    SetMainCallback1(Some(CB1_Overworld));
    SetMainCallback2(Some(CB2_Overworld));
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CB2_ReturnToFieldContestHall() {
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CB2_ReturnToFieldCableClub() {
    FieldClearVBlankHBlankCallbacks();
    gFieldCallback = Some(FieldCB_ReturnToFieldWirelessLink);
    SetMainCallback2(Some(CB2_LoadMapOnReturnToFieldCableClub));
}
pub(crate) unsafe extern "C" fn CB2_LoadMapOnReturnToFieldCableClub() {
    if LoadMapInStepsLink(&raw mut gMain.state) != 0 {
        SetFieldVBlankCallback();
        SetMainCallback1(Some(CB1_OverworldLink));
        ResetAllMultiplayerState();
        SetMainCallback2(Some(CB2_Overworld));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CB2_ReturnToField() {
    if IsOverworldLinkActive() == TRUE as u32 {
        SetMainCallback2(Some(CB2_ReturnToFieldLink));
    } else {
        FieldClearVBlankHBlankCallbacks();
        SetMainCallback2(Some(CB2_ReturnToFieldLocal));
    }
}
pub(crate) unsafe extern "C" fn CB2_ReturnToFieldLocal() {
    if ReturnToFieldLocal(&raw mut gMain.state) != 0 {
        SetFieldVBlankCallback();
        SetMainCallback2(Some(CB2_Overworld));
    }
}
pub(crate) unsafe extern "C" fn CB2_ReturnToFieldLink() {
    if Overworld_IsRecvQueueAtMax() == 0 && ReturnToFieldLink(&raw mut gMain.state) != 0 {
        SetMainCallback2(Some(CB2_Overworld));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CB2_ReturnToFieldFromMultiplayer() {
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CB2_ReturnToFieldWithOpenMenu() {
    FieldClearVBlankHBlankCallbacks();
    gFieldCallback2 = Some(FieldCB_ReturnToFieldOpenStartMenu);
    CB2_ReturnToField();
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CB2_ReturnToFieldContinueScript() {
    FieldClearVBlankHBlankCallbacks();
    gFieldCallback = Some(FieldCB_ContinueScript);
    CB2_ReturnToField();
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CB2_ReturnToFieldContinueScriptPlayMapMusic() {
    FieldClearVBlankHBlankCallbacks();
    gFieldCallback = Some(FieldCB_ContinueScriptHandleMusic);
    CB2_ReturnToField();
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CB2_ReturnToFieldFadeFromBlack() {
    FieldClearVBlankHBlankCallbacks();
    gFieldCallback = Some(FieldCB_WarpExitFadeFromBlack);
    CB2_ReturnToField();
}
pub(crate) unsafe extern "C" fn FieldCB_FadeTryShowMapPopup() {
    if gMapHeader.showMapName() == TRUE && SecretBaseMapPopupEnabled() == TRUE {
        ShowMapNamePopup();
    }
    FieldCB_WarpExitFadeFromBlack();
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CB2_ContinueSavedGame() {
    let mut trainerHillMapId: u8 = 0;
    FieldClearVBlankHBlankCallbacks();
    StopMapMusic();
    ResetSafariZoneFlag_();
    if gSaveFileStatus == SAVE_STATUS_ERROR as u16 {
        ResetWinStreaks();
    }
    LoadSaveblockMapHeader();
    ClearDiveAndHoleWarps();
    trainerHillMapId = GetCurrentTrainerHillMapId();
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
pub(crate) unsafe extern "C" fn FieldClearVBlankHBlankCallbacks() {
    if UsedPokemonCenterWarp() == TRUE {
        CloseLink();
    }
    if gWirelessCommType != 0 {
        EnableInterrupts(197);
        DisableInterrupts(INTR_FLAG_HBLANK);
    } else {
        let mut savedIme: u16 = (67109384 as usize as *mut u16).read_volatile();
        volatile_write(67109384 as usize as *mut u16, 0);
        volatile_write(
            0x4000200 as usize as *mut u16,
            (0x4000200 as usize as *mut u16).read_volatile() & 65533,
        );
        volatile_write(
            0x4000200 as usize as *mut u16,
            (0x4000200 as usize as *mut u16).read_volatile() | INTR_FLAG_VBLANK,
        );
        volatile_write(67109384 as usize as *mut u16, savedIme);
    }
    SetVBlankCallback(None);
    SetHBlankCallback(None);
}
pub(crate) unsafe extern "C" fn SetFieldVBlankCallback() {
    SetVBlankCallback(Some(VBlankCB_Field));
}
pub(crate) unsafe extern "C" fn VBlankCB_Field() {
    LoadOam();
    ProcessSpriteCopyRequests();
    ScanlineEffect_InitHBlankDmaTransfer();
    FieldUpdateBgTilemapScroll();
    TransferPlttBuffer();
    TransferTilesetAnimsBuffer();
}
pub(crate) unsafe extern "C" fn InitCurrentFlashLevelScanlineEffect() {
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
pub(crate) unsafe extern "C" fn LoadMapInStepsLink(state: *mut u8) -> u32 {
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
    return FALSE as u32;
}
pub(crate) unsafe extern "C" fn LoadMapInStepsLocal(state: *mut u8, a2: u32) -> u32 {
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
    return FALSE as u32;
}
pub(crate) unsafe extern "C" fn ReturnToFieldLocal(state: *mut u8) -> u32 {
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
    return FALSE as u32;
}
pub(crate) unsafe extern "C" fn ReturnToFieldLink(state: *mut u8) -> u32 {
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
    return FALSE as u32;
}
pub(crate) unsafe extern "C" fn DoMapLoadLoop(state: *mut u8) {
    while LoadMapInStepsLocal(state, FALSE as u32) == 0 {}
}
pub(crate) unsafe extern "C" fn ResetMirageTowerAndSaveBlockPtrs() {
    ClearMirageTowerPulseBlend();
    MoveSaveBlocks_ResetHeap();
}
pub(crate) unsafe extern "C" fn ResetScreenForMapLoad() {
    SetGpuReg(0x0, 0);
    ScanlineEffect_Stop();
    {
        {
            let mut _dest: *mut u16 = 83886082 as usize as *mut u16;
            let mut _size: u32 = 1022;
            {
                {
                    let mut tmp: u16 = 0;
                    volatile_write(&raw mut tmp, 0);
                    {
                        {
                            let mut dmaRegs: *mut u32 = 67109076 as usize as *mut u32;
                            volatile_write(dmaRegs, &raw mut tmp as usize as u32);
                            volatile_write(dmaRegs.at(1), _dest as usize as u32);
                            volatile_write(dmaRegs.at(2), 0x81000000 | _size / 2);
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
                            let mut dmaRegs: *mut u32 = 67109076 as usize as *mut u32;
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
                                let mut dmaRegs: *mut u32 = 67109076 as usize as *mut u32;
                                volatile_write(dmaRegs, &raw mut tmp as usize as u32);
                                volatile_write(dmaRegs.at(1), _dest as usize as u32);
                                volatile_write(dmaRegs.at(2), 0x81000000 | _size / 2);
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
pub(crate) unsafe extern "C" fn InitViewGraphics() {
    InitCurrentFlashLevelScanlineEffect();
    InitOverworldGraphicsRegisters();
    InitTextBoxGfxAndPrinters();
    InitMapView();
}
pub(crate) unsafe extern "C" fn InitOverworldGraphicsRegisters() {
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
        gOverworldBackgroundLayerFlags[1]
            | gOverworldBackgroundLayerFlags[2]
            | gOverworldBackgroundLayerFlags[3]
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
pub(crate) unsafe extern "C" fn ResumeMap(a1: u32) {
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
pub(crate) unsafe extern "C" fn InitObjectEventsLink() {
    gTotalCameraPixelOffsetX = 0;
    gTotalCameraPixelOffsetY = 0;
    ResetObjectEvents();
    TrySpawnObjectEvents(0, 0);
    TryRunOnWarpIntoMapScript();
}
pub(crate) unsafe extern "C" fn InitObjectEventsLocal() {
    let mut x: u16 = 0;
    let mut y: u16 = 0;
    let mut player: *mut InitialPlayerAvatarState = null_mut();
    gTotalCameraPixelOffsetX = 0;
    gTotalCameraPixelOffsetY = 0;
    ResetObjectEvents();
    GetCameraFocusCoords(&raw mut x, &raw mut y);
    player = GetInitialPlayerAvatarState();
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
pub(crate) unsafe extern "C" fn InitObjectEventsReturnToField() {
    SpawnObjectEventsOnReturnToField(0, 0);
    RotatingGate_InitPuzzleAndGraphics();
    RunOnReturnToFieldMapScript();
}
pub(crate) unsafe extern "C" fn SetCameraToTrackPlayer() {
    gObjectEvents[gPlayerAvatar.objectEventId].set_trackedByCamera(TRUE as u32);
    InitCameraUpdateCallback(gPlayerAvatar.spriteId);
}
pub(crate) unsafe extern "C" fn SetCameraToTrackGuestPlayer() {
    InitCameraUpdateCallback(GetSpriteForLinkedPlayer(gLocalLinkPlayerId));
}
pub(crate) unsafe extern "C" fn SetCameraToTrackGuestPlayer_2() {
    InitCameraUpdateCallback(GetSpriteForLinkedPlayer(gLocalLinkPlayerId));
}
pub(crate) unsafe extern "C" fn OffsetCameraFocusByLinkPlayerId() {
    let mut x: u16 = 0;
    let mut y: u16 = 0;
    GetCameraFocusCoords(&raw mut x, &raw mut y);
    SetCameraFocusCoords(x + gLocalLinkPlayerId as u16, y);
}
pub(crate) unsafe extern "C" fn SpawnLinkPlayers() {
    let mut i: u16 = 0;
    let mut x: u16 = 0;
    let mut y: u16 = 0;
    GetCameraFocusCoords(&raw mut x, &raw mut y);
    x -= gLocalLinkPlayerId as u16;
    i = 0;
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
pub(crate) unsafe extern "C" fn CreateLinkPlayerSprites() {
    let mut i: u16 = 0;
    i = 0;
    while i < gFieldLinkPlayerCount as u16 {
        CreateLinkPlayerSprite(i as u8, gLinkPlayers[i].version as u8);
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn CB1_OverworldLink() {
    if gWirelessCommType == 0 || IsRfuRecvQueueEmpty() == 0 || IsSendingKeysToLink() == 0 {
        let mut selfId: u8 = gLocalLinkPlayerId;
        UpdateAllLinkPlayers(gLinkPartnersHeldKeys.as_mut_ptr(), selfId as i32);
        UpdateHeldKeyCode(sPlayerKeyInterceptCallback.unwrap_unchecked()(
            selfId as u32,
        ));
        ClearAllPlayerKeys();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ResetAllMultiplayerState() {
    ResetAllPlayerLinkStates();
    SetKeyInterceptCallback(Some(KeyInterCB_SelfIdle));
}
pub(crate) unsafe extern "C" fn ClearAllPlayerKeys() {
    ResetPlayerHeldKeys(gLinkPartnersHeldKeys.as_mut_ptr());
}
pub(crate) unsafe extern "C" fn SetKeyInterceptCallback(
    func: Option<unsafe extern "C" fn(u32) -> u16>,
) {
    sRfuKeepAliveTimer = 0;
    sPlayerKeyInterceptCallback = func;
}
pub(crate) unsafe extern "C" fn CheckRfuKeepAliveTimer() {
    if gWirelessCommType != 0
        && ({
            sRfuKeepAliveTimer += 1;
            sRfuKeepAliveTimer
        }) > 60
    {
        LinkRfu_FatalError();
    }
}
pub(crate) unsafe extern "C" fn ResetAllPlayerLinkStates() {
    let mut i: i32 = 0;
    i = 0;
    while i < MAX_LINK_PLAYERS {
        sPlayerLinkStates[i] = PLAYER_LINK_STATE_IDLE;
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn AreAllPlayersInLinkState(state: u16) -> u32 {
    let mut i: i32 = 0;
    let mut count: i32 = gFieldLinkPlayerCount as i32;
    i = 0;
    while i < count {
        if sPlayerLinkStates[i] as u16 != state {
            return FALSE as u32;
        }
        i += 1;
    }
    return TRUE as u32;
}
pub(crate) unsafe extern "C" fn IsAnyPlayerInLinkState(state: u16) -> u32 {
    let mut i: i32 = 0;
    let mut count: i32 = gFieldLinkPlayerCount as i32;
    i = 0;
    while i < count {
        if sPlayerLinkStates[i] as u16 == state {
            return TRUE as u32;
        }
        i += 1;
    }
    return FALSE as u32;
}
pub(crate) unsafe extern "C" fn HandleLinkPlayerKeyInput(
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
            LINK_KEY_CODE_HANDLE_SEND_QUEUE => {
                if IsCableClubPlayerUnfrozen(trainer) != 0 {
                    sPlayerLinkStates[playerId] = PLAYER_LINK_STATE_BUSY;
                    if (*trainer).isLocalPlayer != 0 {
                        SetKeyInterceptCallback(Some(KeyInterCB_DeferToSendQueue));
                        InitLinkPlayerQueueScript();
                    }
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
        LINK_KEY_CODE_EXIT_SEAT => {
            if sPlayerLinkStates[playerId] == PLAYER_LINK_STATE_READY {
                sPlayerLinkStates[playerId] = PLAYER_LINK_STATE_BUSY;
            }
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn UpdateAllLinkPlayers(keys: *mut u16, selfId: i32) {
    let mut trainer: CableClubPlayer = zeroed();
    let mut i: i32 = 0;
    i = 0;
    while i < MAX_LINK_PLAYERS {
        let mut key: u8 = *keys.at(i) as u8;
        let mut setFacing: u16 = FACING_NONE;
        LoadCableClubPlayer(i, selfId, &raw mut trainer);
        HandleLinkPlayerKeyInput(i as u32, key as u16, &raw mut trainer, &raw mut setFacing);
        if sPlayerLinkStates[i] == PLAYER_LINK_STATE_IDLE {
            setFacing = GetDirectionForDpadKey(key as u16);
        }
        SetPlayerFacingDirection(i as u8, setFacing as u8);
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn UpdateHeldKeyCode(key: u16) {
    if key >= LINK_KEY_CODE_EMPTY && key < LINK_KEY_CODE_UNK_8 {
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
pub(crate) unsafe extern "C" fn KeyInterCB_ReadButtons(key: u32) -> u16 {
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
    return LINK_KEY_CODE_EMPTY;
}
pub(crate) unsafe extern "C" fn GetDirectionForDpadKey(key: u16) -> u16 {
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
        return 0;
    }
}
pub(crate) unsafe extern "C" fn ResetPlayerHeldKeys(mut keys: *mut u16) {
    let mut i: i32 = 0;
    i = 0;
    while i < 4 {
        *keys.at(i) = LINK_KEY_CODE_EMPTY;
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn KeyInterCB_SelfIdle(key: u32) -> u16 {
    if ArePlayerFieldControlsLocked() == TRUE {
        return LINK_KEY_CODE_EMPTY;
    }
    if GetLinkRecvQueueLength() > 4 {
        return LINK_KEY_CODE_HANDLE_RECV_QUEUE;
    }
    if GetLinkSendQueueLength() <= 4 {
        return KeyInterCB_ReadButtons(key);
    }
    return LINK_KEY_CODE_HANDLE_SEND_QUEUE;
}
pub(crate) unsafe extern "C" fn KeyInterCB_Idle(key: u32) -> u16 {
    CheckRfuKeepAliveTimer();
    return LINK_KEY_CODE_EMPTY;
}
pub(crate) unsafe extern "C" fn KeyInterCB_DeferToEventScript(key: u32) -> u16 {
    let mut retVal: u16 = 0;
    if ArePlayerFieldControlsLocked() == TRUE {
        retVal = LINK_KEY_CODE_EMPTY;
    } else {
        retVal = LINK_KEY_CODE_IDLE;
        SetKeyInterceptCallback(Some(KeyInterCB_Idle));
    }
    return retVal;
}
pub(crate) unsafe extern "C" fn KeyInterCB_DeferToRecvQueue(key: u32) -> u16 {
    let mut retVal: u16 = 0;
    if GetLinkRecvQueueLength() >= OVERWORLD_RECV_QUEUE_MAX {
        retVal = LINK_KEY_CODE_EMPTY;
    } else {
        retVal = LINK_KEY_CODE_IDLE;
        UnlockPlayerFieldControls();
        SetKeyInterceptCallback(Some(KeyInterCB_Idle));
    }
    return retVal;
}
pub(crate) unsafe extern "C" fn KeyInterCB_DeferToSendQueue(key: u32) -> u16 {
    let mut retVal: u16 = 0;
    if GetLinkSendQueueLength() > 2 {
        retVal = LINK_KEY_CODE_EMPTY;
    } else {
        retVal = LINK_KEY_CODE_IDLE;
        UnlockPlayerFieldControls();
        SetKeyInterceptCallback(Some(KeyInterCB_Idle));
    }
    return retVal;
}
pub(crate) unsafe extern "C" fn KeyInterCB_ExitingSeat(key: u32) -> u16 {
    CheckRfuKeepAliveTimer();
    return LINK_KEY_CODE_EMPTY;
}
pub(crate) unsafe extern "C" fn KeyInterCB_Ready(keyOrPlayerId: u32) -> u16 {
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
        return 0;
    }
}
pub(crate) unsafe extern "C" fn KeyInterCB_SetReady(key: u32) -> u16 {
    SetKeyInterceptCallback(Some(KeyInterCB_Ready));
    return LINK_KEY_CODE_READY;
}
pub(crate) unsafe extern "C" fn KeyInterCB_SendNothing(key: u32) -> u16 {
    return LINK_KEY_CODE_EMPTY;
}
pub(crate) unsafe extern "C" fn KeyInterCB_WaitForPlayersToExit(keyOrPlayerId: u32) -> u16 {
    if sPlayerLinkStates[keyOrPlayerId] != PLAYER_LINK_STATE_EXITING_ROOM as u8 {
        CheckRfuKeepAliveTimer();
    }
    if AreAllPlayersInLinkState(PLAYER_LINK_STATE_EXITING_ROOM) == TRUE as u32 {
        ScriptContext_SetupScript(EventScript_DoLinkRoomExit.as_ptr().cast_mut());
        SetKeyInterceptCallback(Some(KeyInterCB_SendNothing));
    }
    return LINK_KEY_CODE_EMPTY;
}
pub(crate) unsafe extern "C" fn KeyInterCB_SendExitRoomKey(key: u32) -> u16 {
    SetKeyInterceptCallback(Some(KeyInterCB_WaitForPlayersToExit));
    return LINK_KEY_CODE_EXIT_ROOM;
}
pub(crate) unsafe extern "C" fn KeyInterCB_InLinkActivity(key: u32) -> u16 {
    return LINK_KEY_CODE_EMPTY;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetCableClubPartnersReady() -> u32 {
    if IsAnyPlayerInLinkState(PLAYER_LINK_STATE_EXITING_ROOM) == TRUE as u32 {
        return CABLE_SEAT_FAILED;
    }
    if sPlayerKeyInterceptCallback == Some(KeyInterCB_Ready as unsafe extern "C" fn(u32) -> u16)
        && sPlayerLinkStates[gLocalLinkPlayerId] != PLAYER_LINK_STATE_READY
    {
        return CABLE_SEAT_WAITING;
    }
    if sPlayerKeyInterceptCallback
        == Some(KeyInterCB_ExitingSeat as unsafe extern "C" fn(u32) -> u16)
        && sPlayerLinkStates[gLocalLinkPlayerId] == PLAYER_LINK_STATE_BUSY
    {
        return CABLE_SEAT_FAILED;
    }
    if AreAllPlayersInLinkState(PLAYER_LINK_STATE_READY as u16) != 0 {
        return CABLE_SEAT_SUCCESS;
    }
    return CABLE_SEAT_WAITING;
}
pub(crate) unsafe extern "C" fn IsAnyPlayerExitingCableClub() -> u32 {
    return IsAnyPlayerInLinkState(PLAYER_LINK_STATE_EXITING_ROOM);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetInCableClubSeat() -> u16 {
    SetKeyInterceptCallback(Some(KeyInterCB_SetReady));
    return 0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetLinkWaitingForScript() -> u16 {
    SetKeyInterceptCallback(Some(KeyInterCB_DeferToEventScript));
    return 0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn QueueExitLinkRoomKey() -> u16 {
    SetKeyInterceptCallback(Some(KeyInterCB_SendExitRoomKey));
    return 0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetStartedCableClubActivity() -> u16 {
    SetKeyInterceptCallback(Some(KeyInterCB_InLinkActivity));
    return 0;
}
pub(crate) unsafe extern "C" fn LoadCableClubPlayer(
    linkPlayerId: i32,
    myPlayerId: i32,
    trainer: *mut CableClubPlayer,
) {
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
pub(crate) unsafe extern "C" fn IsCableClubPlayerUnfrozen(player: *mut CableClubPlayer) -> u32 {
    let mut mode: u8 = (*player).movementMode;
    if mode == MOVEMENT_MODE_SCRIPTED || mode == MOVEMENT_MODE_FREE {
        return TRUE as u32;
    } else {
        return FALSE as u32;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn CanCableClubPlayerPressStart(player: *mut CableClubPlayer) -> u32 {
    let mut mode: u8 = (*player).movementMode;
    if mode == MOVEMENT_MODE_SCRIPTED || mode == MOVEMENT_MODE_FREE {
        return TRUE as u32;
    } else {
        return FALSE as u32;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn TryGetTileEventScript(player: *mut CableClubPlayer) -> *mut u8 {
    if (*player).movementMode != MOVEMENT_MODE_SCRIPTED {
        return null_mut();
    }
    return GetCoordEventScriptAtMapPosition(&raw mut (*player).pos);
}
pub(crate) unsafe extern "C" fn PlayerIsAtSouthExit(player: *mut CableClubPlayer) -> u32 {
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
        return 0;
    }
}
pub(crate) unsafe extern "C" fn TryInteractWithPlayer(player: *mut CableClubPlayer) -> *mut u8 {
    let mut otherPlayerPos: MapPosition = zeroed();
    let mut linkPlayerId: u8 = 0;
    if (*player).movementMode != MOVEMENT_MODE_FREE
        && (*player).movementMode != MOVEMENT_MODE_SCRIPTED
    {
        return null_mut();
    }
    otherPlayerPos = (*player).pos;
    otherPlayerPos.x += gDirectionToVectors[(*player).facing].x as i16;
    otherPlayerPos.y += gDirectionToVectors[(*player).facing].y as i16;
    otherPlayerPos.elevation = ELEVATION_TRANSITION as i8;
    linkPlayerId = GetLinkPlayerIdAt(otherPlayerPos.x, otherPlayerPos.y);
    if linkPlayerId != MAX_LINK_PLAYERS as u8 {
        if (*player).isLocalPlayer == 0 {
            return CableClub_EventScript_TooBusyToNotice.as_ptr().cast_mut();
        } else if sPlayerLinkStates[linkPlayerId] != PLAYER_LINK_STATE_IDLE {
            return CableClub_EventScript_TooBusyToNotice.as_ptr().cast_mut();
        } else if GetLinkTrainerCardColor(linkPlayerId) == 0 {
            return CableClub_EventScript_ReadTrainerCard.as_ptr().cast_mut();
        } else {
            return CableClub_EventScript_ReadTrainerCardColored
                .as_ptr()
                .cast_mut();
        }
    }
    return GetInteractedLinkPlayerScript(
        &raw mut otherPlayerPos,
        (*player).metatileBehavior as u8,
        (*player).facing,
    );
}
pub(crate) unsafe extern "C" fn GetDirectionForEventScript(script: *mut u8) -> u16 {
    if script
        == EventScript_BattleColosseum_4P_PlayerSpot0
            .as_ptr()
            .cast_mut()
    {
        return FACING_FORCED_RIGHT;
    } else if script
        == EventScript_BattleColosseum_4P_PlayerSpot1
            .as_ptr()
            .cast_mut()
    {
        return FACING_FORCED_LEFT;
    } else if script
        == EventScript_BattleColosseum_4P_PlayerSpot2
            .as_ptr()
            .cast_mut()
    {
        return FACING_FORCED_RIGHT;
    } else if script
        == EventScript_BattleColosseum_4P_PlayerSpot3
            .as_ptr()
            .cast_mut()
    {
        return FACING_FORCED_LEFT;
    } else if script == EventScript_RecordCenter_Spot0.as_ptr().cast_mut() {
        return FACING_FORCED_RIGHT;
    } else if script == EventScript_RecordCenter_Spot1.as_ptr().cast_mut() {
        return FACING_FORCED_LEFT;
    } else if script == EventScript_RecordCenter_Spot2.as_ptr().cast_mut() {
        return FACING_FORCED_RIGHT;
    } else if script == EventScript_RecordCenter_Spot3.as_ptr().cast_mut() {
        return FACING_FORCED_LEFT;
    } else if script
        == EventScript_BattleColosseum_2P_PlayerSpot0
            .as_ptr()
            .cast_mut()
    {
        return FACING_FORCED_RIGHT;
    } else if script
        == EventScript_BattleColosseum_2P_PlayerSpot1
            .as_ptr()
            .cast_mut()
    {
        return FACING_FORCED_LEFT;
    } else if script == EventScript_TradeCenter_Chair0.as_ptr().cast_mut() {
        return FACING_FORCED_RIGHT;
    } else if script == EventScript_TradeCenter_Chair1.as_ptr().cast_mut() {
        return FACING_FORCED_LEFT;
    } else {
        return FACING_NONE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn InitLinkPlayerQueueScript() {
    LockPlayerFieldControls();
}
pub(crate) unsafe extern "C" fn InitLinkRoomStartMenuScript() {
    PlaySE(SE_WIN_OPEN);
    ShowStartMenu();
    LockPlayerFieldControls();
}
pub(crate) unsafe extern "C" fn RunInteractLocalPlayerScript(script: *mut u8) {
    PlaySE(SE_SELECT);
    ScriptContext_SetupScript(script);
    LockPlayerFieldControls();
}
pub(crate) unsafe extern "C" fn RunConfirmLeaveCableClubScript() {
    PlaySE(SE_WIN_OPEN);
    ScriptContext_SetupScript(EventScript_ConfirmLeaveCableClubRoom.as_ptr().cast_mut());
    LockPlayerFieldControls();
}
pub(crate) unsafe extern "C" fn InitMenuBasedScript(script: *mut u8) {
    PlaySE(SE_SELECT);
    ScriptContext_SetupScript(script);
    LockPlayerFieldControls();
}
pub(crate) unsafe extern "C" fn RunTerminateLinkScript() {
    ScriptContext_SetupScript(EventScript_TerminateLink.as_ptr().cast_mut());
    LockPlayerFieldControls();
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Overworld_IsRecvQueueAtMax() -> u32 {
    if IsOverworldLinkActive() == 0 {
        return FALSE as u32;
    }
    if GetLinkRecvQueueLength() >= OVERWORLD_RECV_QUEUE_MAX {
        sReceivingFromLink = TRUE;
    } else {
        sReceivingFromLink = FALSE;
    }
    return sReceivingFromLink as u32;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Overworld_RecvKeysFromLinkIsRunning() -> u32 {
    let mut temp: u8 = 0;
    if GetLinkRecvQueueLength() < 2 {
        return FALSE as u32;
    } else if IsOverworldLinkActive() != TRUE as u32 {
        return FALSE as u32;
    } else if IsSendingKeysToLink() != TRUE as u32 {
        return FALSE as u32;
    } else if sPlayerKeyInterceptCallback
        == Some(KeyInterCB_DeferToRecvQueue as unsafe extern "C" fn(u32) -> u16)
    {
        return TRUE as u32;
    } else if sPlayerKeyInterceptCallback
        != Some(KeyInterCB_DeferToEventScript as unsafe extern "C" fn(u32) -> u16)
    {
        return FALSE as u32;
    }
    temp = sReceivingFromLink;
    sReceivingFromLink = FALSE;
    if temp == TRUE {
        return TRUE as u32;
    } else if gPaletteFade.active() != 0 && gPaletteFade.softwareFadeFinishing() != 0 {
        return TRUE as u32;
    } else {
        return FALSE as u32;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Overworld_SendKeysToLinkIsRunning() -> u32 {
    if GetLinkSendQueueLength() < 2 {
        return FALSE as u32;
    } else if IsOverworldLinkActive() != TRUE as u32 {
        return FALSE as u32;
    } else if IsSendingKeysToLink() != TRUE as u32 {
        return FALSE as u32;
    } else if sPlayerKeyInterceptCallback
        == Some(KeyInterCB_DeferToSendQueue as unsafe extern "C" fn(u32) -> u16)
    {
        return TRUE as u32;
    } else {
        return FALSE as u32;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsSendingKeysOverCable() -> u32 {
    if gWirelessCommType != 0 {
        return FALSE as u32;
    } else if IsSendingKeysToLink() == 0 {
        return FALSE as u32;
    } else {
        return TRUE as u32;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn GetLinkSendQueueLength() -> u32 {
    if gWirelessCommType != 0 {
        return (&raw mut gRfu.sendQueue.count).read_volatile() as u32;
    } else {
        return gLink.sendQueue.count as u32;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn ZeroLinkPlayerObjectEvent(
    linkPlayerObjEvent: *mut LinkPlayerObjectEvent,
) {
    memset(linkPlayerObjEvent as *mut u8, 0, 4);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ClearLinkPlayerObjectEvents() {
    memset(gLinkPlayerObjectEvents.as_mut_ptr() as *mut u8, 0, 16);
}
pub(crate) unsafe extern "C" fn ZeroObjectEvent(objEvent: *mut ObjectEvent) {
    memset(objEvent as *mut u8, 0, 36);
}
pub(crate) unsafe extern "C" fn SpawnLinkPlayerObjectEvent(
    linkPlayerId: u8,
    x: i16,
    y: i16,
    gender: u8,
) {
    let mut objEventId: u8 = GetFirstInactiveObjectEventId();
    let mut linkPlayerObjEvent: *mut LinkPlayerObjectEvent =
        &raw mut gLinkPlayerObjectEvents[linkPlayerId];
    let mut objEvent: *mut ObjectEvent = &raw mut gObjectEvents[objEventId];
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
pub(crate) unsafe extern "C" fn InitLinkPlayerObjectEventPos(
    objEvent: *mut ObjectEvent,
    x: i16,
    y: i16,
) {
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
pub(crate) unsafe extern "C" fn SetLinkPlayerObjectRange(linkPlayerId: u8, dir: u8) {
    if gLinkPlayerObjectEvents[linkPlayerId].active != 0 {
        let mut objEventId: u8 = gLinkPlayerObjectEvents[linkPlayerId].objEventId;
        let mut objEvent: *mut ObjectEvent = &raw mut gObjectEvents[objEventId];
        *(objEvent as *mut u8).at(25) = dir;
    }
}
pub(crate) unsafe extern "C" fn DestroyLinkPlayerObject(linkPlayerId: u8) {
    let mut linkPlayerObjEvent: *mut LinkPlayerObjectEvent =
        &raw mut gLinkPlayerObjectEvents[linkPlayerId];
    let mut objEventId: u8 = (*linkPlayerObjEvent).objEventId;
    let mut objEvent: *mut ObjectEvent = &raw mut gObjectEvents[objEventId];
    if (*objEvent).spriteId != MAX_SPRITES {
        DestroySprite(&raw mut gSprites[(*objEvent).spriteId]);
    }
    (*linkPlayerObjEvent).active = 0;
    (*objEvent).set_active(0);
}
pub(crate) unsafe extern "C" fn GetSpriteForLinkedPlayer(linkPlayerId: u8) -> u8 {
    let mut objEventId: u8 = gLinkPlayerObjectEvents[linkPlayerId].objEventId;
    let mut objEvent: *mut ObjectEvent = &raw mut gObjectEvents[objEventId];
    return (*objEvent).spriteId;
}
pub(crate) unsafe extern "C" fn GetLinkPlayerCoords(linkPlayerId: u8, x: *mut i16, y: *mut i16) {
    let mut objEventId: u8 = gLinkPlayerObjectEvents[linkPlayerId].objEventId;
    let mut objEvent: *mut ObjectEvent = &raw mut gObjectEvents[objEventId];
    *x = (*objEvent).currentCoords.x;
    *y = (*objEvent).currentCoords.y;
}
pub(crate) unsafe extern "C" fn GetLinkPlayerFacingDirection(linkPlayerId: u8) -> u8 {
    let mut objEventId: u8 = gLinkPlayerObjectEvents[linkPlayerId].objEventId;
    let mut objEvent: *mut ObjectEvent = &raw mut gObjectEvents[objEventId];
    return *(objEvent as *mut u8).at(25);
}
pub(crate) unsafe extern "C" fn GetLinkPlayerElevation(linkPlayerId: u8) -> u8 {
    let mut objEventId: u8 = gLinkPlayerObjectEvents[linkPlayerId].objEventId;
    let mut objEvent: *mut ObjectEvent = &raw mut gObjectEvents[objEventId];
    return (*objEvent).currentElevation();
}
pub(crate) unsafe extern "C" fn GetLinkPlayerObjectStepTimer(linkPlayerId: u8) -> i16 {
    let mut objEventId: u8 = gLinkPlayerObjectEvents[linkPlayerId].objEventId;
    let mut objEvent: *mut ObjectEvent = &raw mut gObjectEvents[objEventId];
    return 16 - (*objEvent).directionSequenceIndex as i8 as i16;
}
pub(crate) unsafe extern "C" fn GetLinkPlayerIdAt(x: i16, y: i16) -> u8 {
    let mut i: u8 = 0;
    i = 0;
    while i < MAX_LINK_PLAYERS as u8 {
        if gLinkPlayerObjectEvents[i].active != 0
            && (gLinkPlayerObjectEvents[i].movementMode == 0
                || gLinkPlayerObjectEvents[i].movementMode == 2)
        {
            let mut objEvent: *mut ObjectEvent =
                &raw mut gObjectEvents[gLinkPlayerObjectEvents[i].objEventId];
            if (*objEvent).currentCoords.x == x && (*objEvent).currentCoords.y == y {
                return i;
            }
        }
        i += 1;
    }
    return 4;
}
pub(crate) unsafe extern "C" fn SetPlayerFacingDirection(linkPlayerId: u8, facing: u8) {
    let mut linkPlayerObjEvent: *mut LinkPlayerObjectEvent =
        &raw mut gLinkPlayerObjectEvents[linkPlayerId];
    let mut objEventId: u8 = (*linkPlayerObjEvent).objEventId;
    let mut objEvent: *mut ObjectEvent = &raw mut gObjectEvents[objEventId];
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
pub(crate) unsafe extern "C" fn MovementEventModeCB_Normal(
    linkPlayerObjEvent: *mut LinkPlayerObjectEvent,
    objEvent: *mut ObjectEvent,
    dir: u8,
) -> u8 {
    return sLinkPlayerFacingHandlers[dir].unwrap_unchecked()(linkPlayerObjEvent, objEvent, dir);
}
pub(crate) unsafe extern "C" fn MovementEventModeCB_Ignored(
    linkPlayerObjEvent: *mut LinkPlayerObjectEvent,
    objEvent: *mut ObjectEvent,
    dir: u8,
) -> u8 {
    return FACING_UP;
}
pub(crate) unsafe extern "C" fn MovementEventModeCB_Scripted(
    linkPlayerObjEvent: *mut LinkPlayerObjectEvent,
    objEvent: *mut ObjectEvent,
    dir: u8,
) -> u8 {
    return sLinkPlayerFacingHandlers[dir].unwrap_unchecked()(linkPlayerObjEvent, objEvent, dir);
}
pub(crate) unsafe extern "C" fn FacingHandler_DoNothing(
    linkPlayerObjEvent: *mut LinkPlayerObjectEvent,
    objEvent: *mut ObjectEvent,
    dir: u8,
) -> u8 {
    return FALSE;
}
pub(crate) unsafe extern "C" fn FacingHandler_DpadMovement(
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
        return 0;
    }
}
pub(crate) unsafe extern "C" fn FacingHandler_ForcedFacingChange(
    linkPlayerObjEvent: *mut LinkPlayerObjectEvent,
    objEvent: *mut ObjectEvent,
    dir: u8,
) -> u8 {
    *(objEvent as *mut u8).at(25) = FlipVerticalAndClearForced(dir, *(objEvent as *mut u8).at(25));
    return FALSE;
}
pub(crate) unsafe extern "C" fn MovementStatusHandler_EnterFreeMode(
    linkPlayerObjEvent: *mut LinkPlayerObjectEvent,
    objEvent: *mut ObjectEvent,
) {
    (*linkPlayerObjEvent).movementMode = MOVEMENT_MODE_FREE;
}
pub(crate) unsafe extern "C" fn MovementStatusHandler_TryAdvanceScript(
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
pub(crate) unsafe extern "C" fn FlipVerticalAndClearForced(newFacing: u8, oldFacing: u8) -> u8 {
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
    return oldFacing;
}
pub(crate) unsafe extern "C" fn LinkPlayerGetCollision(
    selfObjEventId: u8,
    direction: u8,
    x: i16,
    y: i16,
) -> u8 {
    let mut i: u8 = 0;
    i = 0;
    while i < OBJECT_EVENTS_COUNT {
        if i != selfObjEventId {
            if gObjectEvents[i].currentCoords.x == x && gObjectEvents[i].currentCoords.y == y
                || gObjectEvents[i].previousCoords.x == x && gObjectEvents[i].previousCoords.y == y
            {
                return 1;
            }
        }
        i += 1;
    }
    return MapGridGetCollisionAt(x as i32, y as i32);
}
pub(crate) unsafe extern "C" fn CreateLinkPlayerSprite(linkPlayerId: u8, gameVersion: u8) {
    let mut linkPlayerObjEvent: *mut LinkPlayerObjectEvent =
        &raw mut gLinkPlayerObjectEvents[linkPlayerId];
    let mut objEventId: u8 = (*linkPlayerObjEvent).objEventId;
    let mut objEvent: *mut ObjectEvent = &raw mut gObjectEvents[objEventId];
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
pub(crate) unsafe extern "C" fn SpriteCB_LinkPlayer(sprite: *mut Sprite) {
    let mut linkPlayerObjEvent: *mut LinkPlayerObjectEvent =
        &raw mut gLinkPlayerObjectEvents[(*sprite).data[0]];
    let mut objEvent: *mut ObjectEvent = &raw mut gObjectEvents[(*linkPlayerObjEvent).objEventId];
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
