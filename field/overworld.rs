//! Translated from `src/overworld.c` by tools/rustport/c2rs.py, then reviewed.
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

// Data tables (translate with cdata.py): sDummyWarpData sUnusedData gDirectionToVectors sOverworldBgTemplates sFlashEffectParams sLinkPlayerMovementModes sLinkPlayerFacingHandlers sMovementStatusHandler
#[allow(unused_imports)]
use crate::data::overworld::*;

pub(crate) static mut sUnusedOverworldCallback: *mut u8 = core::ptr::null_mut();
pub(crate) static mut sPlayerLinkStates: crate::ffi::Align4<[u8; 4]> = crate::ffi::Align4([0; 4]);
pub(crate) static mut sPlayerKeyInterceptCallback: Option<unsafe extern "C" fn(u32) -> u16> = None;
pub(crate) static mut sReceivingFromLink: u8 = 0u8;
pub(crate) static mut sRfuKeepAliveTimer: u8 = 0u8;
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gOverworldTilemapBuffer_Bg2: *mut u16 = core::ptr::null_mut();
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gOverworldTilemapBuffer_Bg1: *mut u16 = core::ptr::null_mut();
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gOverworldTilemapBuffer_Bg3: *mut u16 = core::ptr::null_mut();
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gHeldKeyCodeToSend: u16 = 0u16;
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gFieldCallback: Option<unsafe extern "C" fn()> = None;
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gFieldCallback2: Option<unsafe extern "C" fn() -> u8> = None;
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gLocalLinkPlayerId: u8 = 0u8;
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gFieldLinkPlayerCount: u8 = 0u8;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sObjectEventLoadFlag: u8 = 0u8;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gLastUsedWarp: crate::ffi::Align4<[u8; 8]> = crate::ffi::Align4([0; 8]);
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sWarpDestination: crate::ffi::Align4<[u8; 8]> = crate::ffi::Align4([0; 8]);
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sFixedDiveWarp: crate::ffi::Align4<[u8; 8]> = crate::ffi::Align4([0; 8]);
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sFixedHoleWarp: crate::ffi::Align4<[u8; 8]> = crate::ffi::Align4([0; 8]);
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sLastMapSectionId: u16 = 0u16;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sInitialPlayerAvatarState: crate::ffi::Align4<[u8; 4]> =
    crate::ffi::Align4([0; 4]);
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sAmbientCrySpecies: u16 = 0u16;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sIsAmbientCryWaterMon: u8 = 0u8;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gLinkPlayerObjectEvents: crate::ffi::Align4<[u8; 16]> = crate::ffi::Align4([0; 16]);

unsafe extern "C" {
    static mut CableClub_EventScript_ReadTrainerCard: u8;
    static mut CableClub_EventScript_ReadTrainerCardColored: u8;
    static mut CableClub_EventScript_TooBusyToNotice: u8;
    static mut EventScript_BattleColosseum_2P_PlayerSpot0: u8;
    static mut EventScript_BattleColosseum_2P_PlayerSpot1: u8;
    static mut EventScript_BattleColosseum_4P_PlayerSpot0: u8;
    static mut EventScript_BattleColosseum_4P_PlayerSpot1: u8;
    static mut EventScript_BattleColosseum_4P_PlayerSpot2: u8;
    static mut EventScript_BattleColosseum_4P_PlayerSpot3: u8;
    static mut EventScript_ConfirmLeaveCableClubRoom: u8;
    static mut EventScript_DoLinkRoomExit: u8;
    static mut EventScript_RecordCenter_Spot0: u8;
    static mut EventScript_RecordCenter_Spot1: u8;
    static mut EventScript_RecordCenter_Spot2: u8;
    static mut EventScript_RecordCenter_Spot3: u8;
    static mut EventScript_ResetMrBriney: u8;
    static mut EventScript_TerminateLink: u8;
    static mut EventScript_TradeCenter_Chair0: u8;
    static mut EventScript_TradeCenter_Chair1: u8;
    static mut EventScript_WhiteOut: u8;
    static mut gBackupMapLayout: u8;
    static mut gLink: u8;
    static mut gLinkPartnersHeldKeys: u8;
    static mut gLinkPlayers: u8;
    static mut gMain: u8;
    static mut gMapGroups: u8;
    static mut gMapHeader: u8;
    static mut gMapLayouts: u8;
    static mut gMaxFlashLevel: u8;
    static mut gObjectEvents: u8;
    static mut gOverworldBackgroundLayerFlags: u8;
    static mut gPaletteFade: u8;
    static mut gPlayerAvatar: u8;
    static mut gPlayerParty: u8;
    static mut gRfu: u8;
    static mut gSaveBlock1Ptr: u8;
    static mut gSaveBlock2Ptr: u8;
    static mut gSaveFileStatus: u8;
    static mut gSprites: u8;
    static mut gTotalCameraPixelOffsetX: u8;
    static mut gTotalCameraPixelOffsetY: u8;
    static mut gWirelessCommType: u8;
    fn AllocZeroed(a0: u32) -> *mut u8;
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
    fn CopyMapTilesetsToVram(a0: *mut u8);
    fn CopyPrimaryTilesetToVram(a0: *mut u8);
    fn CopySecondaryTilesetToVram(a0: *mut u8);
    fn CopySecondaryTilesetToVramUsingHeap(a0: *mut u8);
    fn CpuSet(a0: *mut u8, a1: *mut u8, a2: u32);
    fn CreateObjectGraphicsSprite(
        a0: u16,
        a1: Option<unsafe extern "C" fn(*mut u8)>,
        a2: i16,
        a3: i16,
        a4: u8,
    ) -> u8;
    fn CreateWirelessStatusIndicatorSprite(a0: u8, a1: u8);
    fn DestroySprite(a0: *mut u8);
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
    fn FieldClearPlayerInput(a0: *mut u8);
    fn FieldEffectActiveListClear();
    fn FieldGetPlayerInput(a0: *mut u8, a1: u16, a2: u16);
    fn FieldUpdateBgTilemapScroll();
    fn FlagClear(a0: u16) -> u8;
    fn FlagGet(a0: u16) -> u8;
    fn Free(a0: *mut u8);
    fn FreeAllOverworldWindowBuffers();
    fn FreeTempTileDataBuffersIfPossible() -> u8;
    fn GetCameraFocusCoords(a0: *mut u16, a1: *mut u16);
    fn GetCoordEventScriptAtMapPosition(a0: *mut u8) -> *mut u8;
    fn GetCurrentMapMusic() -> u16;
    fn GetCurrentTrainerHillMapId() -> u8;
    fn GetFRLGAvatarGraphicsIdByGender(a0: u8) -> u8;
    fn GetFaceDirectionAnimNum(a0: u8) -> u8;
    fn GetFirstInactiveObjectEventId() -> u8;
    fn GetHealLocation(a0: u32) -> *mut u8;
    fn GetInteractedLinkPlayerScript(a0: *mut u8, a1: u8, a2: u8) -> *mut u8;
    fn GetLinkRecvQueueLength() -> u32;
    fn GetLinkTrainerCardColor(a0: u8) -> u32;
    fn GetLocalWaterMon() -> u16;
    fn GetLocalWildMon(a0: *mut u8) -> u16;
    fn GetMonAbility(a0: *mut u8) -> u8;
    fn GetMonData2(a0: *mut u8, a1: i32) -> u32;
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
    fn InitBgsFromTemplates(a0: u8, a1: *mut u8, a2: u8);
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
    fn LoadMapTilesetPalettes(a0: *mut u8);
    fn LoadOam();
    fn LoadSecondaryTilesetPalette(a0: *mut u8);
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
    fn ObjectEventMoveDestCoords(a0: *mut u8, a1: u32, a2: *mut i16, a3: *mut i16);
    fn ObjectEventUpdateElevation(a0: *mut u8);
    fn PlayCry_NormalNoDucking(a0: u16, a1: i8, a2: i8, a3: u8);
    fn PlayNewMapMusic(a0: u16);
    fn PlaySE(a0: u16);
    fn PlayTimeCounter_Start();
    fn PlayerGetDestCoords(a0: *mut i16, a1: *mut i16);
    fn PlayerStep(a0: u8, a1: u16, a2: u16);
    fn ProcessPlayerFieldInput(a0: *mut u8) -> i32;
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
    fn ScanlineEffect_SetParams(a0: crate::c::Rec4<12>);
    fn ScanlineEffect_Stop();
    fn ScheduleBgCopyTilemapToVram(a0: u8);
    fn ScriptContext_Init();
    fn ScriptContext_RunScript() -> u8;
    fn ScriptContext_SetupScript(a0: *mut u8);
    fn SecretBaseMapPopupEnabled() -> u8;
    fn SetBgAttribute(a0: u8, a1: u8, a2: u8);
    fn SetBgTilemapBuffer(a0: u8, a1: *mut u8);
    fn SetCameraFocusCoords(a0: u16, a1: u16);
    fn SetGpuReg(a0: u8, a1: u16);
    fn SetHBlankCallback(a0: Option<unsafe extern "C" fn()>);
    fn SetMainCallback2(a0: Option<unsafe extern "C" fn()>);
    fn SetMoney(a0: *mut u32, a1: u32);
    fn SetObjectSubpriorityByElevation(a0: u8, a1: *mut u8, a2: u8);
    fn SetPlayerAvatarTransitionFlags(a0: u16);
    fn SetSavedWeatherFromCurrMapHeader();
    fn SetSpritePosToMapCoords(a0: i16, a1: i16, a2: *mut i16, a3: *mut i16);
    fn SetUpFieldTasks();
    fn SetVBlankCallback(a0: Option<unsafe extern "C" fn()>);
    fn ShiftObjectEventCoords(a0: *mut u8, a1: i16, a2: i16);
    fn ShiftStillObjectEventCoords(a0: *mut u8);
    fn ShowBg(a0: u8);
    fn ShowMapNamePopup();
    fn ShowStartMenu();
    fn SpawnObjectEventsOnReturnToField(a0: i16, a1: i16);
    fn StartSpriteAnim(a0: *mut u8, a1: u8);
    fn StartSpriteAnimIfDifferent(a0: *mut u8, a1: u8);
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
    fn UpdateObjectEventSpriteInvisibility(a0: *mut u8, a1: u8);
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
    unsafe {
        RunScriptImmediately((&raw mut EventScript_WhiteOut).cast::<u8>());
        SetMoney(
            (((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                .wrapping_add(1168)
                .cast::<u32>(),
            crate::c::div_u32(
                GetMoney(
                    (((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(1168)
                        .cast::<u32>(),
                ),
                2u32,
            ),
        );
        HealPlayerParty();
        Overworld_ResetStateAfterWhiteOut();
        SetWarpDestinationToLastHealLocation();
        WarpIntoMap();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Overworld_ResetStateAfterFly() {
    unsafe {
        ResetInitialPlayerAvatarState();
        FlagClear(2187u16);
        FlagClear(2189u16);
        FlagClear(2188u16);
        FlagClear(2185u16);
        FlagClear(2184u16);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Overworld_ResetStateAfterTeleport() {
    unsafe {
        ResetInitialPlayerAvatarState();
        FlagClear(2187u16);
        FlagClear(2189u16);
        FlagClear(2188u16);
        FlagClear(2185u16);
        FlagClear(2184u16);
        RunScriptImmediately((&raw mut EventScript_ResetMrBriney).cast::<u8>());
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Overworld_ResetStateAfterDigEscRope() {
    unsafe {
        ResetInitialPlayerAvatarState();
        FlagClear(2187u16);
        FlagClear(2189u16);
        FlagClear(2188u16);
        FlagClear(2185u16);
        FlagClear(2184u16);
    }
}
pub(crate) unsafe extern "C" fn Overworld_ResetStateAfterWhiteOut() {
    unsafe {
        ResetInitialPlayerAvatarState();
        FlagClear(2187u16);
        FlagClear(2189u16);
        FlagClear(2188u16);
        FlagClear(2185u16);
        FlagClear(2184u16);
        if ((VarGet(16441u16)) as i32) == 1i32 {
            VarSet(16441u16, 0u16);
            VarSet(16439u16, 0u16);
        }
    }
}
pub(crate) unsafe extern "C" fn UpdateMiscOverworldStates() {
    unsafe {
        FlagClear(2188u16);
        ChooseAmbientCrySpecies();
        ResetCyclingRoadChallengeData();
        UpdateLocationHistoryForRoamer();
        RoamerMoveToOtherLocationSet();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ResetGameStats() {
    unsafe {
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 64i32) {
                    break 'l1;
                }
                'l2: {
                    SetGameStat(((i) as u8), 0u32);
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IncrementGameStat(index: u8) {
    unsafe {
        let mut index = index;
        if ((index) as i32) < 52i32 {
            let mut statVal: u32 = GetGameStat(index);
            if statVal < 16777215u32 {
                statVal = (statVal).wrapping_add(1);
            } else {
                statVal = 16777215u32;
            }
            SetGameStat(index, statVal);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetGameStat(index: u8) -> u32 {
    unsafe {
        let mut index = index;
        if ((index) as i32) >= 52i32 {
            return 0u32;
        }
        return (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(5532))
            .cast::<u32>())
        .wrapping_offset(((index) as i32) as isize))
        .read()
            ^ ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                .wrapping_add(172)
                .cast::<u32>())
            .read());
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetGameStat(index: u8, value: u32) {
    unsafe {
        let mut index = index;
        let mut value = value;
        if ((index) as i32) < 52i32 {
            ((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(5532))
                .cast::<u32>())
            .wrapping_offset(((index) as i32) as isize))
            .write(
                (value
                    ^ ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(172)
                        .cast::<u32>())
                    .read()),
            );
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ApplyNewEncryptionKeyToGameStats(newKey: u32) {
    unsafe {
        let mut newKey = newKey;
        let mut i: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 64i32) {
                    break 'l1;
                }
                'l2: {
                    ApplyNewEncryptionKeyToWord(
                        (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(5532))
                        .cast::<u32>())
                        .wrapping_offset(((i) as i32) as isize),
                        newKey,
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn LoadObjEventTemplatesFromHeader() {
    unsafe {
        'l1: loop {
            'l2: {
                {
                    let mut tmp: u32 = 0u32;
                    (&raw mut tmp).write_volatile(0u32);
                    'l3: loop {
                        'l4: {
                            CpuSet(
                                (&raw mut tmp).cast::<u8>(),
                                ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                                    .wrapping_add(3184))
                                .cast::<u8>(),
                                (83886080u32
                                    | (crate::c::div_u32(
                                        1536u32,
                                        ((crate::c::div_i32(32i32, 8i32)) as u32),
                                    ) & 2097151u32)),
                            );
                        }
                        if !((0i32) != 0) {
                            break 'l3;
                        }
                    }
                }
            }
            if !((0i32) != 0) {
                break 'l1;
            }
        }
        'l5: loop {
            'l6: {
                'l7: loop {
                    'l8: {
                        CpuSet(
                            (((((&raw mut gMapHeader).cast::<u8>())
                                .wrapping_add(4)
                                .cast::<*mut u8>())
                            .read())
                            .wrapping_add(4)
                            .cast::<*mut u8>())
                            .read(),
                            ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(3184))
                            .cast::<u8>(),
                            (67108864u32
                                | (crate::c::div_u32(
                                    ((((((&raw mut gMapHeader).cast::<u8>())
                                        .wrapping_add(4)
                                        .cast::<*mut u8>())
                                    .read())
                                    .read()) as u32)
                                        .wrapping_mul(24u32),
                                    ((crate::c::div_i32(32i32, 8i32)) as u32),
                                ) & 2097151u32)),
                        );
                    }
                    if !((0i32) != 0) {
                        break 'l7;
                    }
                }
            }
            if !((0i32) != 0) {
                break 'l5;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn LoadSaveblockObjEventScripts() {
    unsafe {
        let mut mapHeaderObjTemplates: *mut u8 = (((((&raw mut gMapHeader).cast::<u8>())
            .wrapping_add(4)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(4)
        .cast::<*mut u8>())
        .read();
        let mut savObjTemplates: *mut u8 = ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
            .wrapping_add(3184))
        .cast::<u8>();
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 64i32) {
                    break 'l1;
                }
                'l2: {
                    (((savObjTemplates).wrapping_offset((i) as isize * 24))
                        .wrapping_add(16)
                        .cast::<*mut u8>())
                    .write(
                        (((mapHeaderObjTemplates).wrapping_offset((i) as isize * 24))
                            .wrapping_add(16)
                            .cast::<*mut u8>())
                        .read(),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetObjEventTemplateCoords(localId: u8, x: i16, y: i16) {
    unsafe {
        let mut localId = localId;
        let mut x = x;
        let mut y = y;
        let mut i: i32 = 0i32;
        let mut savObjTemplates: *mut u8 = ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
            .wrapping_add(3184))
        .cast::<u8>();
        {
            i = 0i32;
            'l1: loop {
                if !(i < 64i32) {
                    break 'l1;
                }
                'l2: {
                    let mut objectEventTemplate: *mut u8 =
                        (savObjTemplates).wrapping_offset((i) as isize * 24);
                    if (((objectEventTemplate).read()) as i32) == ((localId) as i32) {
                        ((objectEventTemplate).wrapping_add(4).cast::<i16>()).write(x);
                        ((objectEventTemplate).wrapping_add(6).cast::<i16>()).write(y);
                        return;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetObjEventTemplateMovementType(localId: u8, movementType: u8) {
    unsafe {
        let mut localId = localId;
        let mut movementType = movementType;
        let mut i: i32 = 0i32;
        let mut savObjTemplates: *mut u8 = ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
            .wrapping_add(3184))
        .cast::<u8>();
        {
            i = 0i32;
            'l1: loop {
                if !(i < 64i32) {
                    break 'l1;
                }
                'l2: {
                    let mut objectEventTemplate: *mut u8 =
                        (savObjTemplates).wrapping_offset((i) as isize * 24);
                    if (((objectEventTemplate).read()) as i32) == ((localId) as i32) {
                        ((objectEventTemplate).wrapping_add(9)).write(movementType);
                        return;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn InitMapView() {
    unsafe {
        ResetFieldCamera();
        CopyMapTilesetsToVram((((&raw mut gMapHeader).cast::<u8>()).cast::<*mut u8>()).read());
        LoadMapTilesetPalettes((((&raw mut gMapHeader).cast::<u8>()).cast::<*mut u8>()).read());
        DrawWholeMapView();
        InitTilesetAnimations();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetMapLayout() -> *mut u8 {
    unsafe {
        let mut mapLayoutId: u16 = ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
            .wrapping_add(50)
            .cast::<u16>())
        .read();
        if (mapLayoutId) != 0 {
            return ((((&raw mut gMapLayouts).cast::<*mut u8>()).cast::<*mut u8>())
                .wrapping_offset((((mapLayoutId) as i32).wrapping_sub(1i32)) as isize))
            .read();
        }
        return core::ptr::null_mut();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ApplyCurrentWarp() {
    unsafe {
        (&raw mut gLastUsedWarp)
            .cast::<u8>()
            .cast::<crate::c::Rec4<8>>()
            .write_unaligned(
                (((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                    .wrapping_add(4)
                    .cast::<crate::c::Rec4<8>>()
                    .read_unaligned(),
            );
        (((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
            .wrapping_add(4)
            .cast::<crate::c::Rec4<8>>()
            .write_unaligned(
                (&raw mut sWarpDestination)
                    .cast::<u8>()
                    .cast::<crate::c::Rec4<8>>()
                    .read_unaligned(),
            );
        (&raw mut sFixedDiveWarp)
            .cast::<u8>()
            .cast::<crate::c::Rec4<8>>()
            .write_unaligned(
                (&raw const sDummyWarpData)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<crate::c::Rec4<8>>()
                    .read_unaligned(),
            );
        (&raw mut sFixedHoleWarp)
            .cast::<u8>()
            .cast::<crate::c::Rec4<8>>()
            .write_unaligned(
                (&raw const sDummyWarpData)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<crate::c::Rec4<8>>()
                    .read_unaligned(),
            );
    }
}
pub(crate) unsafe extern "C" fn ClearDiveAndHoleWarps() {
    unsafe {
        (&raw mut sFixedDiveWarp)
            .cast::<u8>()
            .cast::<crate::c::Rec4<8>>()
            .write_unaligned(
                (&raw const sDummyWarpData)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<crate::c::Rec4<8>>()
                    .read_unaligned(),
            );
        (&raw mut sFixedHoleWarp)
            .cast::<u8>()
            .cast::<crate::c::Rec4<8>>()
            .write_unaligned(
                (&raw const sDummyWarpData)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<crate::c::Rec4<8>>()
                    .read_unaligned(),
            );
    }
}
pub(crate) unsafe extern "C" fn SetWarpData(
    warp: *mut u8,
    mapGroup: i8,
    mapNum: i8,
    warpId: i8,
    x: i8,
    y: i8,
) {
    unsafe {
        let mut warp = warp;
        let mut mapGroup = mapGroup;
        let mut mapNum = mapNum;
        let mut warpId = warpId;
        let mut x = x;
        let mut y = y;
        ((warp).cast::<i8>()).write(mapGroup);
        ((warp).wrapping_add(1).cast::<i8>()).write(mapNum);
        ((warp).wrapping_add(2).cast::<i8>()).write(warpId);
        ((warp).wrapping_add(4).cast::<i16>()).write(((x) as i16));
        ((warp).wrapping_add(6).cast::<i16>()).write(((y) as i16));
    }
}
pub(crate) unsafe extern "C" fn IsDummyWarp(warp: *mut u8) -> u32 {
    unsafe {
        let mut warp = warp;
        if ((((warp).cast::<i8>()).read()) as i32) != (-1i32) {
            return 0u32;
        } else {
            if ((((warp).wrapping_add(1).cast::<i8>()).read()) as i32) != (-1i32) {
                return 0u32;
            } else {
                if ((((warp).wrapping_add(2).cast::<i8>()).read()) as i32) != (-1i32) {
                    return 0u32;
                } else {
                    if ((((warp).wrapping_add(4).cast::<i16>()).read()) as i32) != (-1i32) {
                        return 0u32;
                    } else {
                        if ((((warp).wrapping_add(6).cast::<i16>()).read()) as i32) != (-1i32) {
                            return 0u32;
                        } else {
                            return 1u32;
                        }
                    }
                }
            }
        }
        #[allow(unreachable_code)]
        {
            return 0u32;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Overworld_GetMapHeaderByGroupAndId(mapGroup: u16, mapNum: u16) -> *mut u8 {
    unsafe {
        let mut mapGroup = mapGroup;
        let mut mapNum = mapNum;
        return ((((((&raw mut gMapGroups).cast::<*mut *mut u8>()).cast::<*mut *mut u8>())
            .wrapping_offset(((mapGroup) as i32) as isize))
        .read())
        .wrapping_offset(((mapNum) as i32) as isize))
        .read();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetDestinationWarpMapHeader() -> *mut u8 {
    unsafe {
        return Overworld_GetMapHeaderByGroupAndId(
            (((((&raw mut sWarpDestination).cast::<u8>()).cast::<i8>()).read()) as u16),
            (((((&raw mut sWarpDestination).cast::<u8>())
                .wrapping_add(1)
                .cast::<i8>())
            .read()) as u16),
        );
    }
}
pub(crate) unsafe extern "C" fn LoadCurrentMapData() {
    unsafe {
        ((&raw mut sLastMapSectionId).cast::<u8>().cast::<u16>())
            .write((((((&raw mut gMapHeader).cast::<u8>()).wrapping_add(20)).read()) as u16));
        (&raw mut gMapHeader)
            .cast::<u8>()
            .cast::<crate::c::Rec4<28>>()
            .write_unaligned(
                Overworld_GetMapHeaderByGroupAndId(
                    (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4))
                        .cast::<i8>())
                    .read()) as u16),
                    (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4))
                        .wrapping_add(1)
                        .cast::<i8>())
                    .read()) as u16),
                )
                .cast::<crate::c::Rec4<28>>()
                .read_unaligned(),
            );
        ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
            .wrapping_add(50)
            .cast::<u16>())
        .write(
            (((&raw mut gMapHeader).cast::<u8>())
                .wrapping_add(18)
                .cast::<u16>())
            .read(),
        );
        (((&raw mut gMapHeader).cast::<u8>()).cast::<*mut u8>()).write(GetMapLayout());
    }
}
pub(crate) unsafe extern "C" fn LoadSaveblockMapHeader() {
    unsafe {
        (&raw mut gMapHeader)
            .cast::<u8>()
            .cast::<crate::c::Rec4<28>>()
            .write_unaligned(
                Overworld_GetMapHeaderByGroupAndId(
                    (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4))
                        .cast::<i8>())
                    .read()) as u16),
                    (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4))
                        .wrapping_add(1)
                        .cast::<i8>())
                    .read()) as u16),
                )
                .cast::<crate::c::Rec4<28>>()
                .read_unaligned(),
            );
        (((&raw mut gMapHeader).cast::<u8>()).cast::<*mut u8>()).write(GetMapLayout());
    }
}
pub(crate) unsafe extern "C" fn SetPlayerCoordsFromWarp() {
    unsafe {
        if ((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4))
            .wrapping_add(2)
            .cast::<i8>())
        .read()) as i32)
            >= 0i32)
            && ((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4))
                .wrapping_add(2)
                .cast::<i8>())
            .read()) as i32)
                < (((((((&raw mut gMapHeader).cast::<u8>())
                    .wrapping_add(4)
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(1))
                .read()) as i32))
        {
            ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).cast::<i16>()).write(
                ((((((((&raw mut gMapHeader).cast::<u8>())
                    .wrapping_add(4)
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(8)
                .cast::<*mut u8>())
                .read())
                .wrapping_offset(
                    (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4))
                        .wrapping_add(2)
                        .cast::<i8>())
                    .read()) as i32) as isize
                        * 8,
                ))
                .cast::<i16>())
                .read(),
            );
            ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                .wrapping_add(2)
                .cast::<i16>())
            .write(
                ((((((((&raw mut gMapHeader).cast::<u8>())
                    .wrapping_add(4)
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(8)
                .cast::<*mut u8>())
                .read())
                .wrapping_offset(
                    (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4))
                        .wrapping_add(2)
                        .cast::<i8>())
                    .read()) as i32) as isize
                        * 8,
                ))
                .wrapping_add(2)
                .cast::<i16>())
                .read(),
            );
        } else {
            if ((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4))
                .wrapping_add(4)
                .cast::<i16>())
            .read()) as i32)
                >= 0i32)
                && ((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4))
                    .wrapping_add(6)
                    .cast::<i16>())
                .read()) as i32)
                    >= 0i32)
            {
                ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).cast::<i16>()).write(
                    (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4))
                        .wrapping_add(4)
                        .cast::<i16>())
                    .read(),
                );
                ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                    .wrapping_add(2)
                    .cast::<i16>())
                .write(
                    (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4))
                        .wrapping_add(6)
                        .cast::<i16>())
                    .read(),
                );
            } else {
                ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).cast::<i16>()).write(
                    ((crate::c::div_i32(
                        (((((&raw mut gMapHeader).cast::<u8>()).cast::<*mut u8>()).read())
                            .cast::<i32>())
                        .read(),
                        2i32,
                    )) as i16),
                );
                ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                    .wrapping_add(2)
                    .cast::<i16>())
                .write(
                    ((crate::c::div_i32(
                        (((((&raw mut gMapHeader).cast::<u8>()).cast::<*mut u8>()).read())
                            .wrapping_add(4)
                            .cast::<i32>())
                        .read(),
                        2i32,
                    )) as i16),
                );
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn WarpIntoMap() {
    unsafe {
        ApplyCurrentWarp();
        LoadCurrentMapData();
        SetPlayerCoordsFromWarp();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetWarpDestination(mapGroup: i8, mapNum: i8, warpId: i8, x: i8, y: i8) {
    unsafe {
        let mut mapGroup = mapGroup;
        let mut mapNum = mapNum;
        let mut warpId = warpId;
        let mut x = x;
        let mut y = y;
        SetWarpData(
            (&raw mut sWarpDestination).cast::<u8>(),
            mapGroup,
            mapNum,
            warpId,
            x,
            y,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetWarpDestinationToMapWarp(mapGroup: i8, mapNum: i8, warpId: i8) {
    unsafe {
        let mut mapGroup = mapGroup;
        let mut mapNum = mapNum;
        let mut warpId = warpId;
        SetWarpDestination(mapGroup, mapNum, warpId, (-1i8), (-1i8));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetDynamicWarp(unused: i32, mapGroup: i8, mapNum: i8, warpId: i8) {
    unsafe {
        let mut unused = unused;
        let mut mapGroup = mapGroup;
        let mut mapNum = mapNum;
        let mut warpId = warpId;
        SetWarpData(
            (((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(20),
            mapGroup,
            mapNum,
            warpId,
            ((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).cast::<i16>()).read()) as i8),
            ((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                .wrapping_add(2)
                .cast::<i16>())
            .read()) as i8),
        );
    }
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
    unsafe {
        let mut unused = unused;
        let mut mapGroup = mapGroup;
        let mut mapNum = mapNum;
        let mut warpId = warpId;
        let mut x = x;
        let mut y = y;
        SetWarpData(
            (((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(20),
            mapGroup,
            mapNum,
            warpId,
            x,
            y,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetWarpDestinationToDynamicWarp(unusedWarpId: u8) {
    unsafe {
        let mut unusedWarpId = unusedWarpId;
        (&raw mut sWarpDestination)
            .cast::<u8>()
            .cast::<crate::c::Rec4<8>>()
            .write_unaligned(
                (((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                    .wrapping_add(20)
                    .cast::<crate::c::Rec4<8>>()
                    .read_unaligned(),
            );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetWarpDestinationToHealLocation(healLocationId: u8) {
    unsafe {
        let mut healLocationId = healLocationId;
        let mut healLocation: *mut u8 = GetHealLocation(((healLocationId) as u32));
        if !(healLocation).is_null() {
            SetWarpDestination(
                ((healLocation).cast::<i8>()).read(),
                ((healLocation).wrapping_add(1).cast::<i8>()).read(),
                (-1i8),
                ((((healLocation).wrapping_add(2).cast::<u16>()).read()) as i8),
                ((((healLocation).wrapping_add(4).cast::<u16>()).read()) as i8),
            );
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetWarpDestinationToLastHealLocation() {
    unsafe {
        (&raw mut sWarpDestination)
            .cast::<u8>()
            .cast::<crate::c::Rec4<8>>()
            .write_unaligned(
                (((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                    .wrapping_add(28)
                    .cast::<crate::c::Rec4<8>>()
                    .read_unaligned(),
            );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetLastHealLocationWarp(healLocationId: u8) {
    unsafe {
        let mut healLocationId = healLocationId;
        let mut healLocation: *mut u8 = GetHealLocation(((healLocationId) as u32));
        if !(healLocation).is_null() {
            SetWarpData(
                (((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(28),
                ((healLocation).cast::<i8>()).read(),
                ((healLocation).wrapping_add(1).cast::<i8>()).read(),
                (-1i8),
                ((((healLocation).wrapping_add(2).cast::<u16>()).read()) as i8),
                ((((healLocation).wrapping_add(4).cast::<u16>()).read()) as i8),
            );
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn UpdateEscapeWarp(x: i16, y: i16) {
    unsafe {
        let mut x = x;
        let mut y = y;
        let mut currMapType: u8 = GetCurrentMapType();
        let mut destMapType: u8 = GetMapTypeByGroupAndId(
            (((&raw mut sWarpDestination).cast::<u8>()).cast::<i8>()).read(),
            (((&raw mut sWarpDestination).cast::<u8>())
                .wrapping_add(1)
                .cast::<i8>())
            .read(),
        );
        if ((IsMapTypeOutdoors(currMapType)) != 0)
            && (((IsMapTypeOutdoors(destMapType)) as i32) != 1i32)
        {
            SetEscapeWarp(
                (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4))
                    .cast::<i8>())
                .read(),
                (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4))
                    .wrapping_add(1)
                    .cast::<i8>())
                .read(),
                (-1i8),
                ((((x) as i32).wrapping_sub(7i32)) as i8),
                (((((y) as i32).wrapping_sub(7i32)).wrapping_add(1i32)) as i8),
            );
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetEscapeWarp(mapGroup: i8, mapNum: i8, warpId: i8, x: i8, y: i8) {
    unsafe {
        let mut mapGroup = mapGroup;
        let mut mapNum = mapNum;
        let mut warpId = warpId;
        let mut x = x;
        let mut y = y;
        SetWarpData(
            (((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(36),
            mapGroup,
            mapNum,
            warpId,
            x,
            y,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetWarpDestinationToEscapeWarp() {
    unsafe {
        (&raw mut sWarpDestination)
            .cast::<u8>()
            .cast::<crate::c::Rec4<8>>()
            .write_unaligned(
                (((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                    .wrapping_add(36)
                    .cast::<crate::c::Rec4<8>>()
                    .read_unaligned(),
            );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetFixedDiveWarp(mapGroup: i8, mapNum: i8, warpId: i8, x: i8, y: i8) {
    unsafe {
        let mut mapGroup = mapGroup;
        let mut mapNum = mapNum;
        let mut warpId = warpId;
        let mut x = x;
        let mut y = y;
        SetWarpData(
            (&raw mut sFixedDiveWarp).cast::<u8>(),
            mapGroup,
            mapNum,
            warpId,
            x,
            y,
        );
    }
}
pub(crate) unsafe extern "C" fn SetWarpDestinationToDiveWarp() {
    unsafe {
        (&raw mut sWarpDestination)
            .cast::<u8>()
            .cast::<crate::c::Rec4<8>>()
            .write_unaligned(
                (&raw mut sFixedDiveWarp)
                    .cast::<u8>()
                    .cast::<crate::c::Rec4<8>>()
                    .read_unaligned(),
            );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetFixedHoleWarp(mapGroup: i8, mapNum: i8, warpId: i8, x: i8, y: i8) {
    unsafe {
        let mut mapGroup = mapGroup;
        let mut mapNum = mapNum;
        let mut warpId = warpId;
        let mut x = x;
        let mut y = y;
        SetWarpData(
            (&raw mut sFixedHoleWarp).cast::<u8>(),
            mapGroup,
            mapNum,
            warpId,
            x,
            y,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetWarpDestinationToFixedHoleWarp(x: i16, y: i16) {
    unsafe {
        let mut x = x;
        let mut y = y;
        if IsDummyWarp((&raw mut sFixedHoleWarp).cast::<u8>()) == 1u32 {
            (&raw mut sWarpDestination)
                .cast::<u8>()
                .cast::<crate::c::Rec4<8>>()
                .write_unaligned(
                    (&raw mut gLastUsedWarp)
                        .cast::<u8>()
                        .cast::<crate::c::Rec4<8>>()
                        .read_unaligned(),
                );
        } else {
            SetWarpDestination(
                (((&raw mut sFixedHoleWarp).cast::<u8>()).cast::<i8>()).read(),
                (((&raw mut sFixedHoleWarp).cast::<u8>())
                    .wrapping_add(1)
                    .cast::<i8>())
                .read(),
                (-1i8),
                ((x) as i8),
                ((y) as i8),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn SetWarpDestinationToContinueGameWarp() {
    unsafe {
        (&raw mut sWarpDestination)
            .cast::<u8>()
            .cast::<crate::c::Rec4<8>>()
            .write_unaligned(
                (((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                    .wrapping_add(12)
                    .cast::<crate::c::Rec4<8>>()
                    .read_unaligned(),
            );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetContinueGameWarp(mapGroup: i8, mapNum: i8, warpId: i8, x: i8, y: i8) {
    unsafe {
        let mut mapGroup = mapGroup;
        let mut mapNum = mapNum;
        let mut warpId = warpId;
        let mut x = x;
        let mut y = y;
        SetWarpData(
            (((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(12),
            mapGroup,
            mapNum,
            warpId,
            x,
            y,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetContinueGameWarpToHealLocation(healLocationId: u8) {
    unsafe {
        let mut healLocationId = healLocationId;
        let mut healLocation: *mut u8 = GetHealLocation(((healLocationId) as u32));
        if !(healLocation).is_null() {
            SetWarpData(
                (((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(12),
                ((healLocation).cast::<i8>()).read(),
                ((healLocation).wrapping_add(1).cast::<i8>()).read(),
                (-1i8),
                ((((healLocation).wrapping_add(2).cast::<u16>()).read()) as i8),
                ((((healLocation).wrapping_add(4).cast::<u16>()).read()) as i8),
            );
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetContinueGameWarpToDynamicWarp(unused: i32) {
    unsafe {
        let mut unused = unused;
        (((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
            .wrapping_add(12)
            .cast::<crate::c::Rec4<8>>()
            .write_unaligned(
                (((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                    .wrapping_add(20)
                    .cast::<crate::c::Rec4<8>>()
                    .read_unaligned(),
            );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetMapConnection(dir: u8) -> *mut u8 {
    unsafe {
        let mut dir = dir;
        let mut i: i32 = 0i32;
        let mut count: i32 = (((((&raw mut gMapHeader).cast::<u8>())
            .wrapping_add(12)
            .cast::<*mut u8>())
        .read())
        .cast::<i32>())
        .read();
        let mut connection: *mut u8 = (((((&raw mut gMapHeader).cast::<u8>())
            .wrapping_add(12)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(4)
        .cast::<*mut u8>())
        .read();
        if ((connection) as usize) == 0usize {
            return core::ptr::null_mut();
        }
        {
            i = 0i32;
            'l1: loop {
                if !(i < count) {
                    break 'l1;
                }
                'l2: {
                    if (((connection).read()) as i32) == ((dir) as i32) {
                        return connection;
                    }
                }
                i = (i).wrapping_add(1);
                connection = (connection).wrapping_offset(12);
            }
        }
        return core::ptr::null_mut();
    }
}
pub(crate) unsafe extern "C" fn SetDiveWarp(dir: u8, x: u16, y: u16) -> u8 {
    unsafe {
        let mut dir = dir;
        let mut x = x;
        let mut y = y;
        let mut connection: *mut u8 = GetMapConnection(dir);
        if ((connection) as usize) != 0usize {
            SetWarpDestination(
                ((((connection).wrapping_add(8)).read()) as i8),
                ((((connection).wrapping_add(9)).read()) as i8),
                (-1i8),
                ((x) as i8),
                ((y) as i8),
            );
        } else {
            RunOnDiveWarpMapScript();
            if (IsDummyWarp((&raw mut sFixedDiveWarp).cast::<u8>())) != 0 {
                return 0u8;
            }
            SetWarpDestinationToDiveWarp();
        }
        return 1u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetDiveWarpEmerge(x: u16, y: u16) -> u8 {
    unsafe {
        let mut x = x;
        let mut y = y;
        return SetDiveWarp(6u8, x, y);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetDiveWarpDive(x: u16, y: u16) -> u8 {
    unsafe {
        let mut x = x;
        let mut y = y;
        return SetDiveWarp(5u8, x, y);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn LoadMapFromCameraTransition(mapGroup: u8, mapNum: u8) {
    unsafe {
        let mut mapGroup = mapGroup;
        let mut mapNum = mapNum;
        let mut paletteIndex: i32 = 0i32;
        SetWarpDestination(((mapGroup) as i8), ((mapNum) as i8), (-1i8), (-1i8), (-1i8));
        if (((((&raw mut gMapHeader).cast::<u8>()).wrapping_add(20)).read()) as i32) != 58i32 {
            TransitionMapMusic();
        }
        ApplyCurrentWarp();
        LoadCurrentMapData();
        LoadObjEventTemplatesFromHeader();
        TrySetMapSaveWarpStatus();
        ClearTempFieldEventData();
        ResetCyclingRoadChallengeData();
        RestartWildEncounterImmunitySteps();
        TryUpdateRandomTrainerRematches(((mapGroup) as u16), ((mapNum) as u16));
        DoTimeBasedEvents();
        SetSavedWeatherFromCurrMapHeader();
        ChooseAmbientCrySpecies();
        SetDefaultFlashLevel();
        Overworld_ClearSavedMusic();
        RunOnTransitionMapScript();
        InitMap();
        CopySecondaryTilesetToVramUsingHeap(
            (((&raw mut gMapHeader).cast::<u8>()).cast::<*mut u8>()).read(),
        );
        LoadSecondaryTilesetPalette(
            (((&raw mut gMapHeader).cast::<u8>()).cast::<*mut u8>()).read(),
        );
        {
            paletteIndex = 6i32;
            'l1: loop {
                if !(paletteIndex < 13i32) {
                    break 'l1;
                }
                'l2: {
                    ApplyWeatherColorMapToPal(((paletteIndex) as u8));
                }
                paletteIndex = (paletteIndex).wrapping_add(1);
            }
        }
        InitSecondaryTilesetAnimation();
        UpdateLocationHistoryForRoamer();
        RoamerMove();
        DoCurrentWeather();
        ResetFieldTasksArgs();
        RunOnResumeMapScript();
        if ((((((&raw mut gMapHeader).cast::<u8>()).wrapping_add(20)).read()) as i32) != 58i32)
            || ((((((&raw mut gMapHeader).cast::<u8>()).wrapping_add(20)).read()) as i32)
                != ((((&raw mut sLastMapSectionId).cast::<u8>().cast::<u16>()).read()) as i32))
        {
            ShowMapNamePopup();
        }
    }
}
pub(crate) unsafe extern "C" fn LoadMapFromWarp(a1: u32) {
    unsafe {
        let mut a1 = a1;
        let mut isOutdoors: u8 = 0u8;
        let mut isIndoors: u8 = 0u8;
        LoadCurrentMapData();
        if !((((((&raw mut sObjectEventLoadFlag).cast::<u8>().cast::<u8>()).read()) as i32) & 1i32)
            != 0)
        {
            if (((((&raw mut gMapHeader).cast::<u8>())
                .wrapping_add(18)
                .cast::<u16>())
            .read()) as i32)
                == 361i32
            {
                LoadBattlePyramidObjectEventTemplates();
            } else {
                if (InTrainerHill()) != 0 {
                    LoadTrainerHillObjectEventTemplates();
                } else {
                    LoadObjEventTemplatesFromHeader();
                }
            }
        }
        isOutdoors =
            IsMapTypeOutdoors((((&raw mut gMapHeader).cast::<u8>()).wrapping_add(23)).read());
        isIndoors =
            IsMapTypeIndoors((((&raw mut gMapHeader).cast::<u8>()).wrapping_add(23)).read());
        CheckLeftFriendsSecretBase();
        TrySetMapSaveWarpStatus();
        ClearTempFieldEventData();
        ResetCyclingRoadChallengeData();
        RestartWildEncounterImmunitySteps();
        TryUpdateRandomTrainerRematches(
            (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4))
                .cast::<i8>())
            .read()) as u16),
            (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4))
                .wrapping_add(1)
                .cast::<i8>())
            .read()) as u16),
        );
        if a1 != 1u32 {
            DoTimeBasedEvents();
        }
        SetSavedWeatherFromCurrMapHeader();
        ChooseAmbientCrySpecies();
        if (isOutdoors) != 0 {
            FlagClear(2184u16);
        }
        SetDefaultFlashLevel();
        Overworld_ClearSavedMusic();
        RunOnTransitionMapScript();
        UpdateLocationHistoryForRoamer();
        RoamerMoveToOtherLocationSet();
        if (((((&raw mut gMapHeader).cast::<u8>())
            .wrapping_add(18)
            .cast::<u16>())
        .read()) as i32)
            == 361i32
        {
            InitBattlePyramidMap(0u8);
        } else {
            if (InTrainerHill()) != 0 {
                InitTrainerHillMap();
            } else {
                InitMap();
            }
        }
        if (a1 != 1u32) && ((isIndoors) != 0) {
            UpdateTVScreensOnMap(
                (((&raw mut gBackupMapLayout).cast::<u8>()).cast::<i32>()).read(),
                (((&raw mut gBackupMapLayout).cast::<u8>())
                    .wrapping_add(4)
                    .cast::<i32>())
                .read(),
            );
            InitSecretBaseAppearance(1u8);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ResetInitialPlayerAvatarState() {
    unsafe {
        (((&raw mut sInitialPlayerAvatarState).cast::<u8>()).wrapping_add(1)).write(1u8);
        ((&raw mut sInitialPlayerAvatarState).cast::<u8>()).write(1u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn StoreInitialPlayerAvatarState() {
    unsafe {
        (((&raw mut sInitialPlayerAvatarState).cast::<u8>()).wrapping_add(1))
            .write(GetPlayerFacingDirection());
        if (TestPlayerAvatarFlags(2u8)) != 0 {
            ((&raw mut sInitialPlayerAvatarState).cast::<u8>()).write(2u8);
        } else {
            if (TestPlayerAvatarFlags(4u8)) != 0 {
                ((&raw mut sInitialPlayerAvatarState).cast::<u8>()).write(4u8);
            } else {
                if (TestPlayerAvatarFlags(8u8)) != 0 {
                    ((&raw mut sInitialPlayerAvatarState).cast::<u8>()).write(8u8);
                } else {
                    if (TestPlayerAvatarFlags(16u8)) != 0 {
                        ((&raw mut sInitialPlayerAvatarState).cast::<u8>()).write(16u8);
                    } else {
                        ((&raw mut sInitialPlayerAvatarState).cast::<u8>()).write(1u8);
                    }
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn GetInitialPlayerAvatarState() -> *mut u8 {
    unsafe {
        let mut playerStruct = crate::ffi::Align4([0u8; 4]);
        let mut mapType: u8 = GetCurrentMapType();
        let mut metatileBehavior: u16 = GetCenterScreenMetatileBehavior();
        let mut transitionFlags: u8 = GetAdjustedInitialTransitionFlags(
            (&raw mut sInitialPlayerAvatarState).cast::<u8>(),
            metatileBehavior,
            mapType,
        );
        ((&raw mut playerStruct).cast::<u8>()).write(transitionFlags);
        (((&raw mut playerStruct).cast::<u8>()).wrapping_add(1)).write(
            GetAdjustedInitialDirection(
                (&raw mut sInitialPlayerAvatarState).cast::<u8>(),
                transitionFlags,
                metatileBehavior,
                mapType,
            ),
        );
        (&raw mut sInitialPlayerAvatarState)
            .cast::<u8>()
            .cast::<crate::c::Rec4<4>>()
            .write_unaligned(
                (&raw mut playerStruct)
                    .cast::<u8>()
                    .cast::<crate::c::Rec4<4>>()
                    .read_unaligned(),
            );
        return (&raw mut sInitialPlayerAvatarState).cast::<u8>();
    }
}
pub(crate) unsafe extern "C" fn GetAdjustedInitialTransitionFlags(
    playerStruct: *mut u8,
    metatileBehavior: u16,
    mapType: u8,
) -> u8 {
    unsafe {
        let mut playerStruct = playerStruct;
        let mut metatileBehavior = metatileBehavior;
        let mut mapType = mapType;
        if (((mapType) as i32) != 8i32) && ((FlagGet(2189u16)) != 0) {
            return 1u8;
        } else {
            if ((mapType) as i32) == 5i32 {
                return 16u8;
            } else {
                if ((MetatileBehavior_IsSurfableWaterOrUnderwater(((metatileBehavior) as u8)))
                    as i32)
                    == 1i32
                {
                    return 8u8;
                } else {
                    if Overworld_IsBikingAllowed() != 1u32 {
                        return 1u8;
                    } else {
                        if (((playerStruct).read()) as i32) == 2i32 {
                            return 2u8;
                        } else {
                            if (((playerStruct).read()) as i32) != 4i32 {
                                return 1u8;
                            } else {
                                return 4u8;
                            }
                        }
                    }
                }
            }
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
pub(crate) unsafe extern "C" fn GetAdjustedInitialDirection(
    playerStruct: *mut u8,
    transitionFlags: u8,
    metatileBehavior: u16,
    mapType: u8,
) -> u8 {
    unsafe {
        let mut playerStruct = playerStruct;
        let mut transitionFlags = transitionFlags;
        let mut metatileBehavior = metatileBehavior;
        let mut mapType = mapType;
        if ((FlagGet(2189u16)) != 0) && (((mapType) as i32) == 6i32) {
            return 4u8;
        } else {
            if ((MetatileBehavior_IsDeepSouthWarp(((metatileBehavior) as u8))) as i32) == 1i32 {
                return 2u8;
            } else {
                if (((MetatileBehavior_IsNonAnimDoor(((metatileBehavior) as u8))) as i32) == 1i32)
                    || (((MetatileBehavior_IsDoor(((metatileBehavior) as u8))) as i32) == 1i32)
                {
                    return 1u8;
                } else {
                    if ((MetatileBehavior_IsSouthArrowWarp(((metatileBehavior) as u8))) as i32)
                        == 1i32
                    {
                        return 2u8;
                    } else {
                        if ((MetatileBehavior_IsNorthArrowWarp(((metatileBehavior) as u8))) as i32)
                            == 1i32
                        {
                            return 1u8;
                        } else {
                            if ((MetatileBehavior_IsWestArrowWarp(((metatileBehavior) as u8)))
                                as i32)
                                == 1i32
                            {
                                return 4u8;
                            } else {
                                if ((MetatileBehavior_IsEastArrowWarp(((metatileBehavior) as u8)))
                                    as i32)
                                    == 1i32
                                {
                                    return 3u8;
                                } else {
                                    if (((((playerStruct).read()) as i32) == 16i32)
                                        && (((transitionFlags) as i32) == 8i32))
                                        || (((((playerStruct).read()) as i32) == 8i32)
                                            && (((transitionFlags) as i32) == 16i32))
                                    {
                                        return ((playerStruct).wrapping_add(1)).read();
                                    } else {
                                        if ((MetatileBehavior_IsLadder(((metatileBehavior) as u8)))
                                            as i32)
                                            == 1i32
                                        {
                                            return ((playerStruct).wrapping_add(1)).read();
                                        } else {
                                            return 1u8;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
pub(crate) unsafe extern "C" fn GetCenterScreenMetatileBehavior() -> u16 {
    unsafe {
        return ((MapGridGetMetatileBehaviorAt(
            ((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).cast::<i16>()).read())
                as i32)
                .wrapping_add(7i32),
            ((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                .wrapping_add(2)
                .cast::<i16>())
            .read()) as i32)
                .wrapping_add(7i32),
        )) as u16);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Overworld_IsBikingAllowed() -> u32 {
    unsafe {
        if !((crate::c::bf_read(
            ((&raw mut gMapHeader).cast::<u8>()).wrapping_add(26),
            0,
            1,
            false,
        ) as u8)
            != 0)
        {
            return 0u32;
        } else {
            return 1u32;
        }
        #[allow(unreachable_code)]
        {
            return 0u32;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetDefaultFlashLevel() {
    unsafe {
        if !(((((&raw mut gMapHeader).cast::<u8>()).wrapping_add(21)).read()) != 0) {
            ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(48)).write(0u8);
        } else {
            if (FlagGet(2184u16)) != 0 {
                ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(48))
                    .write(1u8);
            } else {
                ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(48)).write(
                    (((((&raw mut gMaxFlashLevel).cast::<i32>()).read()).wrapping_sub(1i32)) as u8),
                );
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetFlashLevel(flashLevel: i32) {
    unsafe {
        let mut flashLevel = flashLevel;
        if (flashLevel < 0i32) || (flashLevel > ((&raw mut gMaxFlashLevel).cast::<i32>()).read()) {
            flashLevel = 0i32;
        }
        ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(48))
            .write(((flashLevel) as u8));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetFlashLevel() -> u8 {
    unsafe {
        return ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(48)).read();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetCurrentMapLayout(mapLayoutId: u16) {
    unsafe {
        let mut mapLayoutId = mapLayoutId;
        ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
            .wrapping_add(50)
            .cast::<u16>())
        .write(mapLayoutId);
        (((&raw mut gMapHeader).cast::<u8>()).cast::<*mut u8>()).write(GetMapLayout());
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetObjectEventLoadFlag(flag: u8) {
    unsafe {
        let mut flag = flag;
        ((&raw mut sObjectEventLoadFlag).cast::<u8>().cast::<u8>()).write(flag);
    }
}
pub(crate) unsafe extern "C" fn GetObjectEventLoadFlag() -> u8 {
    unsafe {
        return ((&raw mut sObjectEventLoadFlag).cast::<u8>().cast::<u8>()).read();
    }
}
pub(crate) unsafe extern "C" fn ShouldLegendaryMusicPlayAtLocation(warp: *mut u8) -> u16 {
    unsafe {
        let mut warp = warp;
        if !((FlagGet(2186u16)) != 0) {
            return 0u16;
        }
        if ((((warp).cast::<i8>()).read()) as i32) == 0i32 {
            'l1: {
                let __sw1 = ((((warp).wrapping_add(1).cast::<i8>()).read()) as i32);
                let __matched = __sw1 == 5i32
                    || __sw1 == 6i32
                    || __sw1 == 7i32
                    || __sw1 == 8i32
                    || __sw1 == 39i32
                    || __sw1 == 40i32
                    || __sw1 == 41i32
                    || __sw1 == 42i32
                    || __sw1 == 43i32;
                if __sw1 == 5i32
                    || __sw1 == 6i32
                    || __sw1 == 7i32
                    || __sw1 == 8i32
                    || __sw1 == 39i32
                    || __sw1 == 40i32
                    || __sw1 == 41i32
                    || __sw1 == 42i32
                    || __sw1 == 43i32
                {
                    return 1u16;
                }
                if !__matched {
                    if ((VarGet(16478u16)) as i32) < 4i32 {
                        return 0u16;
                    }
                    'l2: {
                        let __sw2 = ((((warp).wrapping_add(1).cast::<i8>()).read()) as i32);
                        if __sw2 == 44i32 || __sw2 == 45i32 || __sw2 == 46i32 {
                            return 1u16;
                        }
                    }
                }
            }
        }
        return 0u16;
    }
}
pub(crate) unsafe extern "C" fn NoMusicInSootopolisWithLegendaries(warp: *mut u8) -> u16 {
    unsafe {
        let mut warp = warp;
        if ((VarGet(16586u16)) as i32) != 1i32 {
            return 0u16;
        } else {
            if ((((warp).cast::<i8>()).read()) as i32) != 0i32 {
                return 0u16;
            } else {
                if ((((warp).wrapping_add(1).cast::<i8>()).read()) as i32) == 7i32 {
                    return 1u16;
                } else {
                    return 0u16;
                }
            }
        }
        #[allow(unreachable_code)]
        {
            return 0u16;
        }
    }
}
pub(crate) unsafe extern "C" fn IsInfiltratedWeatherInstitute(warp: *mut u8) -> u16 {
    unsafe {
        let mut warp = warp;
        if (VarGet(16563u16)) != 0 {
            return 0u16;
        } else {
            if ((((warp).cast::<i8>()).read()) as i32) != 32i32 {
                return 0u16;
            } else {
                if (((((warp).wrapping_add(1).cast::<i8>()).read()) as i32) == 0i32)
                    || (((((warp).wrapping_add(1).cast::<i8>()).read()) as i32) == 1i32)
                {
                    return 1u16;
                } else {
                    return 0u16;
                }
            }
        }
        #[allow(unreachable_code)]
        {
            return 0u16;
        }
    }
}
pub(crate) unsafe extern "C" fn IsInfiltratedSpaceCenter(warp: *mut u8) -> u16 {
    unsafe {
        let mut warp = warp;
        if ((VarGet(16477u16)) as i32) == 0i32 {
            return 0u16;
        } else {
            if ((VarGet(16477u16)) as i32) > 2i32 {
                return 0u16;
            } else {
                if ((((warp).cast::<i8>()).read()) as i32) != 14i32 {
                    return 0u16;
                } else {
                    if (((((warp).wrapping_add(1).cast::<i8>()).read()) as i32) == 9i32)
                        || (((((warp).wrapping_add(1).cast::<i8>()).read()) as i32) == 10i32)
                    {
                        return 1u16;
                    }
                }
            }
        }
        return 0u16;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetLocationMusic(warp: *mut u8) -> u16 {
    unsafe {
        let mut warp = warp;
        if ((NoMusicInSootopolisWithLegendaries(warp)) as i32) == 1i32 {
            return 65535u16;
        } else {
            if ((ShouldLegendaryMusicPlayAtLocation(warp)) as i32) == 1i32 {
                return 443u16;
            } else {
                if ((IsInfiltratedSpaceCenter(warp)) as i32) == 1i32 {
                    return 441u16;
                } else {
                    if ((IsInfiltratedWeatherInstitute(warp)) as i32) == 1i32 {
                        return 406u16;
                    } else {
                        return ((Overworld_GetMapHeaderByGroupAndId(
                            ((((warp).cast::<i8>()).read()) as u16),
                            ((((warp).wrapping_add(1).cast::<i8>()).read()) as u16),
                        ))
                        .wrapping_add(16)
                        .cast::<u16>())
                        .read();
                    }
                }
            }
        }
        #[allow(unreachable_code)]
        {
            return 0u16;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetCurrLocationDefaultMusic() -> u16 {
    unsafe {
        let mut music: u16 = 0u16;
        if (((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4))
            .cast::<i8>())
        .read()) as i32)
            == 0i32)
            && ((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4))
                .wrapping_add(1)
                .cast::<i8>())
            .read()) as i32)
                == 26i32))
            && (((GetSavedWeather()) as i32) == 8i32)
        {
            return 409u16;
        }
        music = GetLocationMusic(
            (((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4),
        );
        if ((music) as i32) != 32767i32 {
            return music;
        } else {
            if ((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).cast::<i16>()).read())
                as i32)
                < 24i32
            {
                return 360u16;
            } else {
                return 402u16;
            }
        }
        #[allow(unreachable_code)]
        {
            return 0u16;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetWarpDestinationMusic() -> u16 {
    unsafe {
        let mut music: u16 = GetLocationMusic((&raw mut sWarpDestination).cast::<u8>());
        if ((music) as i32) != 32767i32 {
            return music;
        } else {
            if ((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4))
                .cast::<i8>())
            .read()) as i32)
                == 0i32)
                && ((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4))
                    .wrapping_add(1)
                    .cast::<i8>())
                .read()) as i32)
                    == 2i32)
            {
                return 360u16;
            } else {
                return 402u16;
            }
        }
        #[allow(unreachable_code)]
        {
            return 0u16;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Overworld_ResetMapMusic() {
    unsafe {
        ResetMapMusic();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Overworld_PlaySpecialMapMusic() {
    unsafe {
        let mut music: u16 = GetCurrLocationDefaultMusic();
        if (((music) as i32) != 443i32) && (((music) as i32) != 65535i32) {
            if (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                .wrapping_add(44)
                .cast::<u16>())
            .read())
                != 0
            {
                music = ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                    .wrapping_add(44)
                    .cast::<u16>())
                .read();
            } else {
                if ((GetCurrentMapType()) as i32) == 5i32 {
                    music = 411u16;
                } else {
                    if (TestPlayerAvatarFlags(8u8)) != 0 {
                        music = 365u16;
                    }
                }
            }
        }
        if ((music) as i32) != ((GetCurrentMapMusic()) as i32) {
            PlayNewMapMusic(music);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Overworld_SetSavedMusic(songNum: u16) {
    unsafe {
        let mut songNum = songNum;
        ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
            .wrapping_add(44)
            .cast::<u16>())
        .write(songNum);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Overworld_ClearSavedMusic() {
    unsafe {
        ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
            .wrapping_add(44)
            .cast::<u16>())
        .write(0u16);
    }
}
pub(crate) unsafe extern "C" fn TransitionMapMusic() {
    unsafe {
        if ((FlagGet(16385u16)) as i32) != 1i32 {
            let mut newMusic: u16 = GetWarpDestinationMusic();
            let mut currentMusic: u16 = GetCurrentMapMusic();
            if (((newMusic) as i32) != 443i32) && (((newMusic) as i32) != 65535i32) {
                if (((currentMusic) as i32) == 411i32) || (((currentMusic) as i32) == 365i32) {
                    return;
                }
                if (TestPlayerAvatarFlags(8u8)) != 0 {
                    newMusic = 365u16;
                }
            }
            if ((newMusic) as i32) != ((currentMusic) as i32) {
                if (TestPlayerAvatarFlags(6u8)) != 0 {
                    FadeOutAndFadeInNewMapMusic(newMusic, 4u8, 4u8);
                } else {
                    FadeOutAndPlayNewMapMusic(newMusic, 8u8);
                }
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Overworld_ChangeMusicToDefault() {
    unsafe {
        let mut currentMusic: u16 = GetCurrentMapMusic();
        if ((currentMusic) as i32) != ((GetCurrLocationDefaultMusic()) as i32) {
            FadeOutAndPlayNewMapMusic(GetCurrLocationDefaultMusic(), 8u8);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Overworld_ChangeMusicTo(newMusic: u16) {
    unsafe {
        let mut newMusic = newMusic;
        let mut currentMusic: u16 = GetCurrentMapMusic();
        if (((currentMusic) as i32) != ((newMusic) as i32)) && (((currentMusic) as i32) != 443i32) {
            FadeOutAndPlayNewMapMusic(newMusic, 8u8);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetMapMusicFadeoutSpeed() -> u8 {
    unsafe {
        let mut mapHeader: *mut u8 = GetDestinationWarpMapHeader();
        if ((IsMapTypeIndoors(((mapHeader).wrapping_add(23)).read())) as i32) == 1i32 {
            return 2u8;
        } else {
            return 4u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn TryFadeOutOldMapMusic() {
    unsafe {
        let mut currentMusic: u16 = GetCurrentMapMusic();
        let mut warpMusic: u16 = GetWarpDestinationMusic();
        if (((FlagGet(16385u16)) as i32) != 1i32)
            && (((warpMusic) as i32) != ((GetCurrentMapMusic()) as i32))
        {
            if (((((((((currentMusic) as i32) == 365i32)
                && (((VarGet(16586u16)) as i32) == 2i32))
                && ((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4))
                    .cast::<i8>())
                .read()) as i32)
                    == 0i32))
                && ((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4))
                    .wrapping_add(1)
                    .cast::<i8>())
                .read()) as i32)
                    == 7i32))
                && ((((((&raw mut sWarpDestination).cast::<u8>()).cast::<i8>()).read()) as i32)
                    == 0i32))
                && ((((((&raw mut sWarpDestination).cast::<u8>())
                    .wrapping_add(1)
                    .cast::<i8>())
                .read()) as i32)
                    == 7i32))
                && ((((((&raw mut sWarpDestination).cast::<u8>())
                    .wrapping_add(4)
                    .cast::<i16>())
                .read()) as i32)
                    == 29i32))
                && ((((((&raw mut sWarpDestination).cast::<u8>())
                    .wrapping_add(6)
                    .cast::<i16>())
                .read()) as i32)
                    == 53i32)
            {
                return;
            }
            FadeOutMapMusic(GetMapMusicFadeoutSpeed());
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BGMusicStopped() -> u8 {
    unsafe {
        return IsNotWaitingForBGMStop();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Overworld_FadeOutMapMusic() {
    unsafe {
        FadeOutMapMusic(4u8);
    }
}
pub(crate) unsafe extern "C" fn PlayAmbientCry() {
    unsafe {
        let mut x: i16 = 0i16;
        let mut y: i16 = 0i16;
        let mut pan: i8 = 0i8;
        let mut volume: i8 = 0i8;
        PlayerGetDestCoords(&raw mut x, &raw mut y);
        if (((((&raw mut sIsAmbientCryWaterMon).cast::<u8>().cast::<u8>()).read()) as i32) == 1i32)
            && (!((MetatileBehavior_IsSurfableWaterOrUnderwater(
                ((MapGridGetMetatileBehaviorAt(((x) as i32), ((y) as i32))) as u8),
            )) != 0))
        {
            return;
        }
        pan = (((crate::c::rem_i32(((Random()) as i32), 88i32)).wrapping_add(212i32)) as i8);
        volume = (((crate::c::rem_i32(((Random()) as i32), 30i32)).wrapping_add(50i32)) as i8);
        PlayCry_NormalNoDucking(
            ((&raw mut sAmbientCrySpecies).cast::<u8>().cast::<u16>()).read(),
            pan,
            volume,
            1u8,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn UpdateAmbientCry(state: *mut i16, delayCounter: *mut u16) {
    unsafe {
        let mut state = state;
        let mut delayCounter = delayCounter;
        let mut i: u8 = 0u8;
        let mut monsCount: u8 = 0u8;
        let mut divBy: u8 = 0u8;
        'l1: {
            let __sw1 = (((state).read()) as i32);
            if __sw1 == 0i32 {
                if ((((&raw mut sAmbientCrySpecies).cast::<u8>().cast::<u16>()).read()) as i32)
                    == 0i32
                {
                    (state).write(4i16);
                } else {
                    (state).write(1i16);
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                (delayCounter).write(
                    (((crate::c::rem_i32(((Random()) as i32), 2400i32)).wrapping_add(1200i32))
                        as u16),
                );
                (state).write(3i16);
                break 'l1;
            }
            if __sw1 == 2i32 {
                divBy = 1u8;
                monsCount = CalculatePlayerPartyCount();
                {
                    i = 0u8;
                    'l2: loop {
                        if !(((i) as i32) < ((monsCount) as i32)) {
                            break 'l2;
                        }
                        'l3: {
                            if (!((GetMonData2(
                                ((&raw mut gPlayerParty).cast::<u8>())
                                    .wrapping_offset(((i) as i32) as isize * 100),
                                6i32,
                            )) != 0))
                                && (((GetMonAbility((&raw mut gPlayerParty).cast::<u8>())) as i32)
                                    == 68i32)
                            {
                                divBy = 2u8;
                                break 'l2;
                            }
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                (delayCounter).write(
                    ((crate::c::div_i32(
                        (crate::c::rem_i32(((Random()) as i32), 1200i32)).wrapping_add(1200i32),
                        ((divBy) as i32),
                    )) as u16),
                );
                (state).write(3i16);
                break 'l1;
            }
            if __sw1 == 3i32 {
                if (({
                    let __t2 = ((delayCounter).read()).wrapping_sub(1);
                    (delayCounter).write(__t2);
                    __t2
                }) as i32)
                    == 0i32
                {
                    PlayAmbientCry();
                    (state).write(2i16);
                }
                break 'l1;
            }
            if __sw1 == 4i32 {
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn ChooseAmbientCrySpecies() {
    unsafe {
        if (((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4))
            .cast::<i8>())
        .read()) as i32)
            == 0i32)
            && ((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4))
                .wrapping_add(1)
                .cast::<i8>())
            .read()) as i32)
                == 45i32))
            && (!((IsMirageIslandPresent()) != 0))
        {
            ((&raw mut sIsAmbientCryWaterMon).cast::<u8>().cast::<u8>()).write(1u8);
            ((&raw mut sAmbientCrySpecies).cast::<u8>().cast::<u16>()).write(GetLocalWaterMon());
        } else {
            ((&raw mut sAmbientCrySpecies).cast::<u8>().cast::<u16>()).write(GetLocalWildMon(
                (&raw mut sIsAmbientCryWaterMon).cast::<u8>().cast::<u8>(),
            ));
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetMapTypeByGroupAndId(mapGroup: i8, mapNum: i8) -> u8 {
    unsafe {
        let mut mapGroup = mapGroup;
        let mut mapNum = mapNum;
        return ((Overworld_GetMapHeaderByGroupAndId(((mapGroup) as u16), ((mapNum) as u16)))
            .wrapping_add(23))
        .read();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetMapTypeByWarpData(warp: *mut u8) -> u8 {
    unsafe {
        let mut warp = warp;
        return GetMapTypeByGroupAndId(
            ((warp).cast::<i8>()).read(),
            ((warp).wrapping_add(1).cast::<i8>()).read(),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetCurrentMapType() -> u8 {
    unsafe {
        return GetMapTypeByWarpData(
            (((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetLastUsedWarpMapType() -> u8 {
    unsafe {
        return GetMapTypeByWarpData((&raw mut gLastUsedWarp).cast::<u8>());
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsMapTypeOutdoors(mapType: u8) -> u8 {
    unsafe {
        let mut mapType = mapType;
        if ((((((mapType) as i32) == 3i32) || (((mapType) as i32) == 1i32))
            || (((mapType) as i32) == 5i32))
            || (((mapType) as i32) == 2i32))
            || (((mapType) as i32) == 6i32)
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Overworld_MapTypeAllowsTeleportAndFly(mapType: u8) -> u8 {
    unsafe {
        let mut mapType = mapType;
        if (((((mapType) as i32) == 3i32) || (((mapType) as i32) == 1i32))
            || (((mapType) as i32) == 6i32))
            || (((mapType) as i32) == 2i32)
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsMapTypeIndoors(mapType: u8) -> u8 {
    unsafe {
        let mut mapType = mapType;
        if (((mapType) as i32) == 8i32) || (((mapType) as i32) == 9i32) {
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetSavedWarpRegionMapSectionId() -> u8 {
    unsafe {
        return ((Overworld_GetMapHeaderByGroupAndId(
            (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(20))
                .cast::<i8>())
            .read()) as u16),
            (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(20))
                .wrapping_add(1)
                .cast::<i8>())
            .read()) as u16),
        ))
        .wrapping_add(20))
        .read();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetCurrentRegionMapSectionId() -> u8 {
    unsafe {
        return ((Overworld_GetMapHeaderByGroupAndId(
            (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4))
                .cast::<i8>())
            .read()) as u16),
            (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4))
                .wrapping_add(1)
                .cast::<i8>())
            .read()) as u16),
        ))
        .wrapping_add(20))
        .read();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetCurrentMapBattleScene() -> u8 {
    unsafe {
        return ((Overworld_GetMapHeaderByGroupAndId(
            (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4))
                .cast::<i8>())
            .read()) as u16),
            (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4))
                .wrapping_add(1)
                .cast::<i8>())
            .read()) as u16),
        ))
        .wrapping_add(27))
        .read();
    }
}
pub(crate) unsafe extern "C" fn InitOverworldBgs() {
    unsafe {
        InitBgsFromTemplates(
            0u8,
            ((&raw const sOverworldBgTemplates).cast::<u8>().cast_mut()).cast::<u8>(),
            ((crate::c::div_u32(16u32, 4u32)) as u8),
        );
        SetBgAttribute(1u8, 5u8, 1u8);
        SetBgAttribute(2u8, 5u8, 1u8);
        SetBgAttribute(3u8, 5u8, 1u8);
        ((&raw mut gOverworldTilemapBuffer_Bg1)
            .cast::<u8>()
            .cast::<*mut u16>())
        .write((AllocZeroed(2048u32)).cast::<u16>());
        ((&raw mut gOverworldTilemapBuffer_Bg2)
            .cast::<u8>()
            .cast::<*mut u16>())
        .write((AllocZeroed(2048u32)).cast::<u16>());
        ((&raw mut gOverworldTilemapBuffer_Bg3)
            .cast::<u8>()
            .cast::<*mut u16>())
        .write((AllocZeroed(2048u32)).cast::<u16>());
        SetBgTilemapBuffer(
            1u8,
            (((&raw mut gOverworldTilemapBuffer_Bg1)
                .cast::<u8>()
                .cast::<*mut u16>())
            .read())
            .cast::<u8>(),
        );
        SetBgTilemapBuffer(
            2u8,
            (((&raw mut gOverworldTilemapBuffer_Bg2)
                .cast::<u8>()
                .cast::<*mut u16>())
            .read())
            .cast::<u8>(),
        );
        SetBgTilemapBuffer(
            3u8,
            (((&raw mut gOverworldTilemapBuffer_Bg3)
                .cast::<u8>()
                .cast::<*mut u16>())
            .read())
            .cast::<u8>(),
        );
        InitStandardTextBoxWindows();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CleanupOverworldWindowsAndTilemaps() {
    unsafe {
        ClearMirageTowerPulseBlendEffect();
        FreeAllOverworldWindowBuffers();
        if ((((&raw mut gOverworldTilemapBuffer_Bg3)
            .cast::<u8>()
            .cast::<*mut u16>())
        .read()) as usize)
            != 0usize
        {
            Free(
                (((&raw mut gOverworldTilemapBuffer_Bg3)
                    .cast::<u8>()
                    .cast::<*mut u16>())
                .read())
                .cast::<u8>(),
            );
            ((&raw mut gOverworldTilemapBuffer_Bg3)
                .cast::<u8>()
                .cast::<*mut u16>())
            .write(core::ptr::null_mut());
        }
        if ((((&raw mut gOverworldTilemapBuffer_Bg2)
            .cast::<u8>()
            .cast::<*mut u16>())
        .read()) as usize)
            != 0usize
        {
            Free(
                (((&raw mut gOverworldTilemapBuffer_Bg2)
                    .cast::<u8>()
                    .cast::<*mut u16>())
                .read())
                .cast::<u8>(),
            );
            ((&raw mut gOverworldTilemapBuffer_Bg2)
                .cast::<u8>()
                .cast::<*mut u16>())
            .write(core::ptr::null_mut());
        }
        if ((((&raw mut gOverworldTilemapBuffer_Bg1)
            .cast::<u8>()
            .cast::<*mut u16>())
        .read()) as usize)
            != 0usize
        {
            Free(
                (((&raw mut gOverworldTilemapBuffer_Bg1)
                    .cast::<u8>()
                    .cast::<*mut u16>())
                .read())
                .cast::<u8>(),
            );
            ((&raw mut gOverworldTilemapBuffer_Bg1)
                .cast::<u8>()
                .cast::<*mut u16>())
            .write(core::ptr::null_mut());
        }
    }
}
pub(crate) unsafe extern "C" fn ResetSafariZoneFlag_() {
    unsafe {
        ResetSafariZoneFlag();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsOverworldLinkActive() -> u32 {
    unsafe {
        if core::mem::transmute::<_, usize>(
            (((&raw mut gMain).cast::<u8>()).cast::<Option<unsafe extern "C" fn()>>()).read(),
        ) == (CB1_OverworldLink as *const () as usize)
        {
            return 1u32;
        } else {
            return 0u32;
        }
        #[allow(unreachable_code)]
        {
            return 0u32;
        }
    }
}
pub(crate) unsafe extern "C" fn DoCB1_Overworld(newKeys: u16, heldKeys: u16) {
    unsafe {
        let mut newKeys = newKeys;
        let mut heldKeys = heldKeys;
        let mut inputStruct = crate::ffi::Align4([0u8; 4]);
        UpdatePlayerAvatarTransitionState();
        FieldClearPlayerInput((&raw mut inputStruct).cast::<u8>());
        FieldGetPlayerInput((&raw mut inputStruct).cast::<u8>(), newKeys, heldKeys);
        if !((ArePlayerFieldControlsLocked()) != 0) {
            if ProcessPlayerFieldInput((&raw mut inputStruct).cast::<u8>()) == 1i32 {
                LockPlayerFieldControls();
                HideMapNamePopUpWindow();
            } else {
                PlayerStep(
                    (((&raw mut inputStruct).cast::<u8>()).wrapping_add(2)).read(),
                    newKeys,
                    heldKeys,
                );
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CB1_Overworld() {
    unsafe {
        if core::mem::transmute::<_, usize>(
            (((&raw mut gMain).cast::<u8>())
                .wrapping_add(4)
                .cast::<Option<unsafe extern "C" fn()>>())
            .read(),
        ) == (CB2_Overworld as *const () as usize)
        {
            DoCB1_Overworld(
                (((&raw mut gMain).cast::<u8>())
                    .wrapping_add(46)
                    .cast::<u16>())
                .read(),
                (((&raw mut gMain).cast::<u8>())
                    .wrapping_add(44)
                    .cast::<u16>())
                .read(),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn OverworldBasic() {
    unsafe {
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
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CB2_OverworldBasic() {
    unsafe {
        OverworldBasic();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CB2_Overworld() {
    unsafe {
        let mut fading: u32 = ((((crate::c::bf_read(
            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
            7,
            1,
            false,
        ) as u16) as i32)
            != 0i32) as u32);
        if (fading) != 0 {
            SetVBlankCallback(None);
        }
        OverworldBasic();
        if (fading) != 0 {
            SetFieldVBlankCallback();
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetMainCallback1(cb: Option<unsafe extern "C" fn()>) {
    unsafe {
        let mut cb = cb;
        (((&raw mut gMain).cast::<u8>()).cast::<Option<unsafe extern "C" fn()>>()).write(cb);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetUnusedCallback(func: *mut u8) {
    unsafe {
        let mut func = func;
        ((&raw mut sUnusedOverworldCallback)
            .cast::<u8>()
            .cast::<*mut u8>())
        .write(func);
    }
}
pub(crate) unsafe extern "C" fn RunFieldCallback() -> u8 {
    unsafe {
        if (((&raw mut gFieldCallback2)
            .cast::<u8>()
            .cast::<Option<unsafe extern "C" fn() -> u8>>())
        .read())
        .is_some()
        {
            if !(((((&raw mut gFieldCallback2)
                .cast::<u8>()
                .cast::<Option<unsafe extern "C" fn() -> u8>>())
            .read())
            .unwrap_unchecked()())
                != 0)
            {
                return 0u8;
            } else {
                ((&raw mut gFieldCallback2)
                    .cast::<u8>()
                    .cast::<Option<unsafe extern "C" fn() -> u8>>())
                .write(None);
                ((&raw mut gFieldCallback)
                    .cast::<u8>()
                    .cast::<Option<unsafe extern "C" fn()>>())
                .write(None);
            }
        } else {
            if (((&raw mut gFieldCallback)
                .cast::<u8>()
                .cast::<Option<unsafe extern "C" fn()>>())
            .read())
            .is_some()
            {
                (((&raw mut gFieldCallback)
                    .cast::<u8>()
                    .cast::<Option<unsafe extern "C" fn()>>())
                .read())
                .unwrap_unchecked()();
            } else {
                FieldCB_DefaultWarpExit();
            }
            ((&raw mut gFieldCallback)
                .cast::<u8>()
                .cast::<Option<unsafe extern "C" fn()>>())
            .write(None);
        }
        return 1u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CB2_NewGame() {
    unsafe {
        FieldClearVBlankHBlankCallbacks();
        StopMapMusic();
        ResetSafariZoneFlag_();
        NewGameInitData();
        ResetInitialPlayerAvatarState();
        PlayTimeCounter_Start();
        ScriptContext_Init();
        UnlockPlayerFieldControls();
        ((&raw mut gFieldCallback)
            .cast::<u8>()
            .cast::<Option<unsafe extern "C" fn()>>())
        .write(Some(ExecuteTruckSequence));
        ((&raw mut gFieldCallback2)
            .cast::<u8>()
            .cast::<Option<unsafe extern "C" fn() -> u8>>())
        .write(None);
        DoMapLoadLoop(((&raw mut gMain).cast::<u8>()).wrapping_add(1080));
        SetFieldVBlankCallback();
        SetMainCallback1(Some(CB1_Overworld));
        SetMainCallback2(Some(CB2_Overworld));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CB2_WhiteOut() {
    unsafe {
        let mut state: u8 = 0u8;
        if (({
            let __p1 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
            let __t2 = ((__p1).read()).wrapping_add(1);
            (__p1).write(__t2);
            __t2
        }) as i32)
            >= 120i32
        {
            FieldClearVBlankHBlankCallbacks();
            StopMapMusic();
            ResetSafariZoneFlag_();
            DoWhiteOut();
            ResetInitialPlayerAvatarState();
            ScriptContext_Init();
            UnlockPlayerFieldControls();
            ((&raw mut gFieldCallback)
                .cast::<u8>()
                .cast::<Option<unsafe extern "C" fn()>>())
            .write(Some(FieldCB_WarpExitFadeFromBlack));
            state = 0u8;
            DoMapLoadLoop(&raw mut state);
            SetFieldVBlankCallback();
            SetMainCallback1(Some(CB1_Overworld));
            SetMainCallback2(Some(CB2_Overworld));
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CB2_LoadMap() {
    unsafe {
        FieldClearVBlankHBlankCallbacks();
        ScriptContext_Init();
        UnlockPlayerFieldControls();
        SetMainCallback1(None);
        SetMainCallback2(Some(CB2_DoChangeMap));
        (((&raw mut gMain).cast::<u8>())
            .wrapping_add(8)
            .cast::<Option<unsafe extern "C" fn()>>())
        .write(Some(CB2_LoadMap2));
    }
}
pub(crate) unsafe extern "C" fn CB2_LoadMap2() {
    unsafe {
        DoMapLoadLoop(((&raw mut gMain).cast::<u8>()).wrapping_add(1080));
        SetFieldVBlankCallback();
        SetMainCallback1(Some(CB1_Overworld));
        SetMainCallback2(Some(CB2_Overworld));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CB2_ReturnToFieldContestHall() {
    unsafe {
        if !(((((&raw mut gMain).cast::<u8>()).wrapping_add(1080)).read()) != 0) {
            FieldClearVBlankHBlankCallbacks();
            ScriptContext_Init();
            UnlockPlayerFieldControls();
            SetMainCallback1(None);
        }
        if (LoadMapInStepsLocal(((&raw mut gMain).cast::<u8>()).wrapping_add(1080), 1u32)) != 0 {
            SetFieldVBlankCallback();
            SetMainCallback1(Some(CB1_Overworld));
            SetMainCallback2(Some(CB2_Overworld));
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CB2_ReturnToFieldCableClub() {
    unsafe {
        FieldClearVBlankHBlankCallbacks();
        ((&raw mut gFieldCallback)
            .cast::<u8>()
            .cast::<Option<unsafe extern "C" fn()>>())
        .write(Some(FieldCB_ReturnToFieldWirelessLink));
        SetMainCallback2(Some(CB2_LoadMapOnReturnToFieldCableClub));
    }
}
pub(crate) unsafe extern "C" fn CB2_LoadMapOnReturnToFieldCableClub() {
    unsafe {
        if (LoadMapInStepsLink(((&raw mut gMain).cast::<u8>()).wrapping_add(1080))) != 0 {
            SetFieldVBlankCallback();
            SetMainCallback1(Some(CB1_OverworldLink));
            ResetAllMultiplayerState();
            SetMainCallback2(Some(CB2_Overworld));
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CB2_ReturnToField() {
    unsafe {
        if IsOverworldLinkActive() == 1u32 {
            SetMainCallback2(Some(CB2_ReturnToFieldLink));
        } else {
            FieldClearVBlankHBlankCallbacks();
            SetMainCallback2(Some(CB2_ReturnToFieldLocal));
        }
    }
}
pub(crate) unsafe extern "C" fn CB2_ReturnToFieldLocal() {
    unsafe {
        if (ReturnToFieldLocal(((&raw mut gMain).cast::<u8>()).wrapping_add(1080))) != 0 {
            SetFieldVBlankCallback();
            SetMainCallback2(Some(CB2_Overworld));
        }
    }
}
pub(crate) unsafe extern "C" fn CB2_ReturnToFieldLink() {
    unsafe {
        if (!((Overworld_IsRecvQueueAtMax()) != 0))
            && ((ReturnToFieldLink(((&raw mut gMain).cast::<u8>()).wrapping_add(1080))) != 0)
        {
            SetMainCallback2(Some(CB2_Overworld));
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CB2_ReturnToFieldFromMultiplayer() {
    unsafe {
        FieldClearVBlankHBlankCallbacks();
        StopMapMusic();
        SetMainCallback1(Some(CB1_OverworldLink));
        ResetAllMultiplayerState();
        if ((((&raw mut gWirelessCommType).cast::<u8>()).read()) as i32) != 0i32 {
            ((&raw mut gFieldCallback)
                .cast::<u8>()
                .cast::<Option<unsafe extern "C" fn()>>())
            .write(Some(FieldCB_ReturnToFieldWirelessLink));
        } else {
            ((&raw mut gFieldCallback)
                .cast::<u8>()
                .cast::<Option<unsafe extern "C" fn()>>())
            .write(Some(FieldCB_ReturnToFieldCableLink));
        }
        ScriptContext_Init();
        UnlockPlayerFieldControls();
        CB2_ReturnToField();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CB2_ReturnToFieldWithOpenMenu() {
    unsafe {
        FieldClearVBlankHBlankCallbacks();
        ((&raw mut gFieldCallback2)
            .cast::<u8>()
            .cast::<Option<unsafe extern "C" fn() -> u8>>())
        .write(Some(FieldCB_ReturnToFieldOpenStartMenu));
        CB2_ReturnToField();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CB2_ReturnToFieldContinueScript() {
    unsafe {
        FieldClearVBlankHBlankCallbacks();
        ((&raw mut gFieldCallback)
            .cast::<u8>()
            .cast::<Option<unsafe extern "C" fn()>>())
        .write(Some(FieldCB_ContinueScript));
        CB2_ReturnToField();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CB2_ReturnToFieldContinueScriptPlayMapMusic() {
    unsafe {
        FieldClearVBlankHBlankCallbacks();
        ((&raw mut gFieldCallback)
            .cast::<u8>()
            .cast::<Option<unsafe extern "C" fn()>>())
        .write(Some(FieldCB_ContinueScriptHandleMusic));
        CB2_ReturnToField();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CB2_ReturnToFieldFadeFromBlack() {
    unsafe {
        FieldClearVBlankHBlankCallbacks();
        ((&raw mut gFieldCallback)
            .cast::<u8>()
            .cast::<Option<unsafe extern "C" fn()>>())
        .write(Some(FieldCB_WarpExitFadeFromBlack));
        CB2_ReturnToField();
    }
}
pub(crate) unsafe extern "C" fn FieldCB_FadeTryShowMapPopup() {
    unsafe {
        if (((crate::c::bf_read(
            ((&raw mut gMapHeader).cast::<u8>()).wrapping_add(26),
            3,
            5,
            false,
        ) as u8) as i32)
            == 1i32)
            && (((SecretBaseMapPopupEnabled()) as i32) == 1i32)
        {
            ShowMapNamePopup();
        }
        FieldCB_WarpExitFadeFromBlack();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CB2_ContinueSavedGame() {
    unsafe {
        let mut trainerHillMapId: u8 = 0u8;
        FieldClearVBlankHBlankCallbacks();
        StopMapMusic();
        ResetSafariZoneFlag_();
        if ((((&raw mut gSaveFileStatus).cast::<u16>()).read()) as i32) == 255i32 {
            ResetWinStreaks();
        }
        LoadSaveblockMapHeader();
        ClearDiveAndHoleWarps();
        trainerHillMapId = GetCurrentTrainerHillMapId();
        if (((((&raw mut gMapHeader).cast::<u8>())
            .wrapping_add(18)
            .cast::<u16>())
        .read()) as i32)
            == 361i32
        {
            LoadBattlePyramidFloorObjectEventScripts();
        } else {
            if (((trainerHillMapId) as i32) != 0i32) && (((trainerHillMapId) as i32) != 6i32) {
                LoadTrainerHillFloorObjectEventScripts();
            } else {
                LoadSaveblockObjEventScripts();
            }
        }
        UnfreezeObjectEvents();
        DoTimeBasedEvents();
        UpdateMiscOverworldStates();
        if (((((&raw mut gMapHeader).cast::<u8>())
            .wrapping_add(18)
            .cast::<u16>())
        .read()) as i32)
            == 361i32
        {
            InitBattlePyramidMap(1u8);
        } else {
            if ((trainerHillMapId) as i32) != 0i32 {
                InitTrainerHillMap();
            } else {
                InitMapFromSavedGame();
            }
        }
        PlayTimeCounter_Start();
        ScriptContext_Init();
        UnlockPlayerFieldControls();
        InitMatchCallCounters();
        if UseContinueGameWarp() == 1u32 {
            ClearContinueGameWarpStatus();
            SetWarpDestinationToContinueGameWarp();
            WarpIntoMap();
            TryPutTodaysRivalTrainerOnAir();
            SetMainCallback2(Some(CB2_LoadMap));
        } else {
            TryPutTodaysRivalTrainerOnAir();
            ((&raw mut gFieldCallback)
                .cast::<u8>()
                .cast::<Option<unsafe extern "C" fn()>>())
            .write(Some(FieldCB_FadeTryShowMapPopup));
            SetMainCallback1(Some(CB1_Overworld));
            CB2_ReturnToField();
        }
    }
}
pub(crate) unsafe extern "C" fn FieldClearVBlankHBlankCallbacks() {
    unsafe {
        if ((UsedPokemonCenterWarp()) as i32) == 1i32 {
            CloseLink();
        }
        if ((((&raw mut gWirelessCommType).cast::<u8>()).read()) as i32) != 0i32 {
            EnableInterrupts(197u16);
            DisableInterrupts(2u16);
        } else {
            let mut savedIme: u16 = ((67109384i32) as usize as *mut u16).read_volatile();
            crate::c::volatile_write(((67109384i32) as usize as *mut u16), 0u16);
            let __p1 = ((67109376i32) as usize as *mut u16);
            crate::c::volatile_write(__p1, (((((__p1).read_volatile()) as i32) & (-3i32)) as u16));
            let __p2 = ((67109376i32) as usize as *mut u16);
            crate::c::volatile_write(__p2, (((((__p2).read_volatile()) as i32) | 1i32) as u16));
            crate::c::volatile_write(((67109384i32) as usize as *mut u16), savedIme);
        }
        SetVBlankCallback(None);
        SetHBlankCallback(None);
    }
}
pub(crate) unsafe extern "C" fn SetFieldVBlankCallback() {
    unsafe {
        SetVBlankCallback(Some(VBlankCB_Field));
    }
}
pub(crate) unsafe extern "C" fn VBlankCB_Field() {
    unsafe {
        LoadOam();
        ProcessSpriteCopyRequests();
        ScanlineEffect_InitHBlankDmaTransfer();
        FieldUpdateBgTilemapScroll();
        TransferPlttBuffer();
        TransferTilesetAnimsBuffer();
    }
}
pub(crate) unsafe extern "C" fn InitCurrentFlashLevelScanlineEffect() {
    unsafe {
        let mut flashLevel: u8 = 0u8;
        if (InBattlePyramid_()) != 0 {
            WriteBattlePyramidViewScanlineEffectBuffer();
            ScanlineEffect_SetParams(
                (&raw const sFlashEffectParams)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<crate::c::Rec4<12>>()
                    .read_unaligned(),
            );
        } else {
            if ({
                let __v1 = GetFlashLevel();
                flashLevel = __v1;
                __v1
            }) != 0
            {
                WriteFlashScanlineEffectBuffer(flashLevel);
                ScanlineEffect_SetParams(
                    (&raw const sFlashEffectParams)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<crate::c::Rec4<12>>()
                        .read_unaligned(),
                );
            }
        }
    }
}
pub(crate) unsafe extern "C" fn LoadMapInStepsLink(state: *mut u8) -> u32 {
    unsafe {
        let mut state = state;
        'l1: {
            let __sw1 = (((state).read()) as i32);
            if __sw1 == 0i32 {
                InitOverworldBgs();
                ScriptContext_Init();
                UnlockPlayerFieldControls();
                ResetMirageTowerAndSaveBlockPtrs();
                ResetScreenForMapLoad();
                (state).write(((state).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                LoadMapFromWarp(1u32);
                (state).write(((state).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 2i32 {
                ResumeMap(1u32);
                (state).write(((state).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 3i32 {
                OffsetCameraFocusByLinkPlayerId();
                InitObjectEventsLink();
                SpawnLinkPlayers();
                SetCameraToTrackGuestPlayer();
                (state).write(((state).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 4i32 {
                InitCurrentFlashLevelScanlineEffect();
                InitOverworldGraphicsRegisters();
                InitTextBoxGfxAndPrinters();
                (state).write(((state).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 5i32 {
                ResetFieldCamera();
                (state).write(((state).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 6i32 {
                CopyPrimaryTilesetToVram(
                    (((&raw mut gMapHeader).cast::<u8>()).cast::<*mut u8>()).read(),
                );
                (state).write(((state).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 7i32 {
                CopySecondaryTilesetToVram(
                    (((&raw mut gMapHeader).cast::<u8>()).cast::<*mut u8>()).read(),
                );
                (state).write(((state).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 8i32 {
                if ((FreeTempTileDataBuffersIfPossible()) as i32) != 1i32 {
                    LoadMapTilesetPalettes(
                        (((&raw mut gMapHeader).cast::<u8>()).cast::<*mut u8>()).read(),
                    );
                    (state).write(((state).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 9i32 {
                DrawWholeMapView();
                (state).write(((state).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 10i32 {
                InitTilesetAnimations();
                (state).write(((state).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 11i32 {
                if ((((&raw mut gWirelessCommType).cast::<u8>()).read()) as i32) != 0i32 {
                    LoadWirelessStatusIndicatorSpriteGfx();
                    CreateWirelessStatusIndicatorSprite(0u8, 0u8);
                }
                (state).write(((state).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 12i32 {
                if (RunFieldCallback()) != 0 {
                    (state).write(((state).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 13i32 {
                return 1u32;
            }
        }
        return 0u32;
    }
}
pub(crate) unsafe extern "C" fn LoadMapInStepsLocal(state: *mut u8, a2: u32) -> u32 {
    unsafe {
        let mut state = state;
        let mut a2 = a2;
        'l1: {
            let __sw1 = (((state).read()) as i32);
            if __sw1 == 0i32 {
                FieldClearVBlankHBlankCallbacks();
                LoadMapFromWarp(a2);
                (state).write(((state).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                ResetMirageTowerAndSaveBlockPtrs();
                ResetScreenForMapLoad();
                (state).write(((state).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 2i32 {
                ResumeMap(a2);
                (state).write(((state).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 3i32 {
                InitObjectEventsLocal();
                SetCameraToTrackPlayer();
                (state).write(((state).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 4i32 {
                InitCurrentFlashLevelScanlineEffect();
                InitOverworldGraphicsRegisters();
                InitTextBoxGfxAndPrinters();
                (state).write(((state).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 5i32 {
                ResetFieldCamera();
                (state).write(((state).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 6i32 {
                CopyPrimaryTilesetToVram(
                    (((&raw mut gMapHeader).cast::<u8>()).cast::<*mut u8>()).read(),
                );
                (state).write(((state).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 7i32 {
                CopySecondaryTilesetToVram(
                    (((&raw mut gMapHeader).cast::<u8>()).cast::<*mut u8>()).read(),
                );
                (state).write(((state).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 8i32 {
                if ((FreeTempTileDataBuffersIfPossible()) as i32) != 1i32 {
                    LoadMapTilesetPalettes(
                        (((&raw mut gMapHeader).cast::<u8>()).cast::<*mut u8>()).read(),
                    );
                    (state).write(((state).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 9i32 {
                DrawWholeMapView();
                (state).write(((state).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 10i32 {
                InitTilesetAnimations();
                (state).write(((state).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 11i32 {
                if (((crate::c::bf_read(
                    ((&raw mut gMapHeader).cast::<u8>()).wrapping_add(26),
                    3,
                    5,
                    false,
                ) as u8) as i32)
                    == 1i32)
                    && (((SecretBaseMapPopupEnabled()) as i32) == 1i32)
                {
                    ShowMapNamePopup();
                }
                (state).write(((state).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 12i32 {
                if (RunFieldCallback()) != 0 {
                    (state).write(((state).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 13i32 {
                return 1u32;
            }
        }
        return 0u32;
    }
}
pub(crate) unsafe extern "C" fn ReturnToFieldLocal(state: *mut u8) -> u32 {
    unsafe {
        let mut state = state;
        'l1: {
            let __sw1 = (((state).read()) as i32);
            if __sw1 == 0i32 {
                ResetMirageTowerAndSaveBlockPtrs();
                ResetScreenForMapLoad();
                ResumeMap(0u32);
                InitObjectEventsReturnToField();
                SetCameraToTrackPlayer();
                (state).write(((state).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                InitViewGraphics();
                TryLoadTrainerHillEReaderPalette();
                (state).write(((state).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 2i32 {
                if (RunFieldCallback()) != 0 {
                    (state).write(((state).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                return 1u32;
            }
        }
        return 0u32;
    }
}
pub(crate) unsafe extern "C" fn ReturnToFieldLink(state: *mut u8) -> u32 {
    unsafe {
        let mut state = state;
        'l1: {
            let __sw1 = (((state).read()) as i32);
            if __sw1 == 0i32 {
                FieldClearVBlankHBlankCallbacks();
                ResetMirageTowerAndSaveBlockPtrs();
                ResetScreenForMapLoad();
                (state).write(((state).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                ResumeMap(1u32);
                (state).write(((state).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 2i32 {
                CreateLinkPlayerSprites();
                InitObjectEventsReturnToField();
                SetCameraToTrackGuestPlayer_2();
                (state).write(((state).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 3i32 {
                InitCurrentFlashLevelScanlineEffect();
                InitOverworldGraphicsRegisters();
                InitTextBoxGfxAndPrinters();
                (state).write(((state).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 4i32 {
                ResetFieldCamera();
                (state).write(((state).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 5i32 {
                CopyPrimaryTilesetToVram(
                    (((&raw mut gMapHeader).cast::<u8>()).cast::<*mut u8>()).read(),
                );
                (state).write(((state).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 6i32 {
                CopySecondaryTilesetToVram(
                    (((&raw mut gMapHeader).cast::<u8>()).cast::<*mut u8>()).read(),
                );
                (state).write(((state).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 7i32 {
                if ((FreeTempTileDataBuffersIfPossible()) as i32) != 1i32 {
                    LoadMapTilesetPalettes(
                        (((&raw mut gMapHeader).cast::<u8>()).cast::<*mut u8>()).read(),
                    );
                    (state).write(((state).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 8i32 {
                DrawWholeMapView();
                (state).write(((state).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 9i32 {
                InitTilesetAnimations();
                (state).write(((state).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 11i32 {
                if ((((&raw mut gWirelessCommType).cast::<u8>()).read()) as i32) != 0i32 {
                    LoadWirelessStatusIndicatorSpriteGfx();
                    CreateWirelessStatusIndicatorSprite(0u8, 0u8);
                }
                (state).write(((state).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 12i32 {
                if (RunFieldCallback()) != 0 {
                    (state).write(((state).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 10i32 {
                (state).write(((state).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 13i32 {
                SetFieldVBlankCallback();
                (state).write(((state).read()).wrapping_add(1));
                return 1u32;
            }
        }
        return 0u32;
    }
}
pub(crate) unsafe extern "C" fn DoMapLoadLoop(state: *mut u8) {
    unsafe {
        let mut state = state;
        'l1: loop {
            if !(!((LoadMapInStepsLocal(state, 0u32)) != 0)) {
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn ResetMirageTowerAndSaveBlockPtrs() {
    unsafe {
        ClearMirageTowerPulseBlend();
        MoveSaveBlocks_ResetHeap();
    }
}
pub(crate) unsafe extern "C" fn ResetScreenForMapLoad() {
    unsafe {
        SetGpuReg(0u8, 0u16);
        ScanlineEffect_Stop();
        'l1: loop {
            'l2: {
                {
                    let mut _dest: *mut u16 = ((83886082i32) as usize as *mut u16);
                    let mut _size: u32 = 1022u32;
                    'l3: loop {
                        'l4: {
                            {
                                let mut tmp: u16 = 0u16;
                                (&raw mut tmp).write_volatile(0u16);
                                'l5: loop {
                                    'l6: {
                                        {
                                            let mut dmaRegs: *mut u32 =
                                                ((67109076i32) as usize as *mut u32);
                                            crate::c::volatile_write(
                                                dmaRegs,
                                                ((&raw mut tmp) as usize as u32),
                                            );
                                            crate::c::volatile_write(
                                                (dmaRegs).wrapping_offset(1),
                                                ((_dest) as usize as u32),
                                            );
                                            crate::c::volatile_write(
                                                (dmaRegs).wrapping_offset(2),
                                                (2164260864u32
                                                    | crate::c::div_u32(
                                                        _size,
                                                        ((crate::c::div_i32(16i32, 8i32)) as u32),
                                                    )),
                                            );
                                            let _ = ((dmaRegs).wrapping_offset(2)).read_volatile();
                                        }
                                    }
                                    if !((0i32) != 0) {
                                        break 'l5;
                                    }
                                }
                            }
                        }
                        if !((0i32) != 0) {
                            break 'l3;
                        }
                    }
                }
            }
            if !((0i32) != 0) {
                break 'l1;
            }
        }
        {
            let mut _dest: *mut u8 = ((100663296i32) as usize as *mut u8);
            let mut _size: u32 = 98304u32;
            'l7: loop {
                if !((1i32) != 0) {
                    break 'l7;
                }
                'l8: loop {
                    'l9: {
                        {
                            let mut tmp: u16 = 0u16;
                            (&raw mut tmp).write_volatile(0u16);
                            'l10: loop {
                                'l11: {
                                    {
                                        let mut dmaRegs: *mut u32 =
                                            ((67109076i32) as usize as *mut u32);
                                        crate::c::volatile_write(
                                            dmaRegs,
                                            ((&raw mut tmp) as usize as u32),
                                        );
                                        crate::c::volatile_write(
                                            (dmaRegs).wrapping_offset(1),
                                            ((_dest) as usize as u32),
                                        );
                                        crate::c::volatile_write(
                                            (dmaRegs).wrapping_offset(2),
                                            (((-2130706432i32)
                                                | crate::c::div_i32(
                                                    4096i32,
                                                    crate::c::div_i32(16i32, 8i32),
                                                ))
                                                as u32),
                                        );
                                        let _ = ((dmaRegs).wrapping_offset(2)).read_volatile();
                                    }
                                }
                                if !((0i32) != 0) {
                                    break 'l10;
                                }
                            }
                        }
                    }
                    if !((0i32) != 0) {
                        break 'l8;
                    }
                }
                _dest = (_dest).wrapping_offset(4096);
                _size = (_size).wrapping_sub(4096u32);
                if _size <= 4096u32 {
                    'l12: loop {
                        'l13: {
                            {
                                let mut tmp: u16 = 0u16;
                                (&raw mut tmp).write_volatile(0u16);
                                'l14: loop {
                                    'l15: {
                                        {
                                            let mut dmaRegs: *mut u32 =
                                                ((67109076i32) as usize as *mut u32);
                                            crate::c::volatile_write(
                                                dmaRegs,
                                                ((&raw mut tmp) as usize as u32),
                                            );
                                            crate::c::volatile_write(
                                                (dmaRegs).wrapping_offset(1),
                                                ((_dest) as usize as u32),
                                            );
                                            crate::c::volatile_write(
                                                (dmaRegs).wrapping_offset(2),
                                                (2164260864u32
                                                    | crate::c::div_u32(
                                                        _size,
                                                        ((crate::c::div_i32(16i32, 8i32)) as u32),
                                                    )),
                                            );
                                            let _ = ((dmaRegs).wrapping_offset(2)).read_volatile();
                                        }
                                    }
                                    if !((0i32) != 0) {
                                        break 'l14;
                                    }
                                }
                            }
                        }
                        if !((0i32) != 0) {
                            break 'l12;
                        }
                    }
                    break 'l7;
                }
            }
        }
        ResetOamRange(0u8, 128u8);
        LoadOam();
    }
}
pub(crate) unsafe extern "C" fn InitViewGraphics() {
    unsafe {
        InitCurrentFlashLevelScanlineEffect();
        InitOverworldGraphicsRegisters();
        InitTextBoxGfxAndPrinters();
        InitMapView();
    }
}
pub(crate) unsafe extern "C" fn InitOverworldGraphicsRegisters() {
    unsafe {
        ClearScheduledBgCopiesToVram();
        ResetTempTileDataBuffers();
        SetGpuReg(76u8, 0u16);
        SetGpuReg(72u8, 7967u16);
        SetGpuReg(74u8, 257u16);
        SetGpuReg(64u8, 255u16);
        SetGpuReg(68u8, 255u16);
        SetGpuReg(66u8, 65535u16);
        SetGpuReg(70u8, 65535u16);
        SetGpuReg(
            80u8,
            (((((((((((&raw mut gOverworldBackgroundLayerFlags).cast::<u16>()).cast::<u16>())
                .wrapping_offset(1))
            .read()) as i32)
                | ((((((&raw mut gOverworldBackgroundLayerFlags).cast::<u16>()).cast::<u16>())
                    .wrapping_offset(2))
                .read()) as i32))
                | ((((((&raw mut gOverworldBackgroundLayerFlags).cast::<u16>()).cast::<u16>())
                    .wrapping_offset(3))
                .read()) as i32))
                | 4096i32)
                | 64i32) as u16),
        );
        SetGpuReg(82u8, 1805u16);
        InitOverworldBgs();
        ScheduleBgCopyTilemapToVram(1u8);
        ScheduleBgCopyTilemapToVram(2u8);
        ScheduleBgCopyTilemapToVram(3u8);
        ChangeBgX(0u8, 0i32, 0u8);
        ChangeBgY(0u8, 0i32, 0u8);
        ChangeBgX(1u8, 0i32, 0u8);
        ChangeBgY(1u8, 0i32, 0u8);
        ChangeBgX(2u8, 0i32, 0u8);
        ChangeBgY(2u8, 0i32, 0u8);
        ChangeBgX(3u8, 0i32, 0u8);
        ChangeBgY(3u8, 0i32, 0u8);
        SetGpuReg(0u8, 28768u16);
        ShowBg(0u8);
        ShowBg(1u8);
        ShowBg(2u8);
        ShowBg(3u8);
        InitFieldMessageBox();
    }
}
pub(crate) unsafe extern "C" fn ResumeMap(a1: u32) {
    unsafe {
        let mut a1 = a1;
        ResetTasks();
        ResetSpriteData();
        ResetPaletteFade();
        ScanlineEffect_Clear();
        ResetAllPicSprites();
        ResetCameraUpdateInfo();
        InstallCameraPanAheadCallback();
        if !((a1) != 0) {
            InitObjectEventPalettes(0u8);
        } else {
            InitObjectEventPalettes(1u8);
        }
        FieldEffectActiveListClear();
        StartWeather();
        ResumePausedWeather();
        if !((a1) != 0) {
            SetUpFieldTasks();
        }
        RunOnResumeMapScript();
        TryStartMirageTowerPulseBlendEffect();
    }
}
pub(crate) unsafe extern "C" fn InitObjectEventsLink() {
    unsafe {
        ((&raw mut gTotalCameraPixelOffsetX).cast::<u16>()).write(0u16);
        ((&raw mut gTotalCameraPixelOffsetY).cast::<u16>()).write(0u16);
        ResetObjectEvents();
        TrySpawnObjectEvents(0i16, 0i16);
        TryRunOnWarpIntoMapScript();
    }
}
pub(crate) unsafe extern "C" fn InitObjectEventsLocal() {
    unsafe {
        let mut x: u16 = 0u16;
        let mut y: u16 = 0u16;
        let mut player: *mut u8 = core::ptr::null_mut();
        ((&raw mut gTotalCameraPixelOffsetX).cast::<u16>()).write(0u16);
        ((&raw mut gTotalCameraPixelOffsetY).cast::<u16>()).write(0u16);
        ResetObjectEvents();
        GetCameraFocusCoords(&raw mut x, &raw mut y);
        player = GetInitialPlayerAvatarState();
        InitPlayerAvatar(
            ((x) as i16),
            ((y) as i16),
            ((player).wrapping_add(1)).read(),
            ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(8)).read(),
        );
        SetPlayerAvatarTransitionFlags((((player).read()) as u16));
        ResetInitialPlayerAvatarState();
        TrySpawnObjectEvents(0i16, 0i16);
        TryRunOnWarpIntoMapScript();
    }
}
pub(crate) unsafe extern "C" fn InitObjectEventsReturnToField() {
    unsafe {
        SpawnObjectEventsOnReturnToField(0i16, 0i16);
        RotatingGate_InitPuzzleAndGraphics();
        RunOnReturnToFieldMapScript();
    }
}
pub(crate) unsafe extern "C" fn SetCameraToTrackPlayer() {
    unsafe {
        crate::c::bf_write(
            (((&raw mut gObjectEvents).cast::<u8>()).wrapping_offset(
                (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(5)).read()) as i32)
                    as isize
                    * 36,
            ))
            .wrapping_add(1),
            7,
            1,
            (1u32) as i32,
        );
        InitCameraUpdateCallback((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(4)).read());
    }
}
pub(crate) unsafe extern "C" fn SetCameraToTrackGuestPlayer() {
    unsafe {
        InitCameraUpdateCallback(GetSpriteForLinkedPlayer(
            ((&raw mut gLocalLinkPlayerId).cast::<u8>().cast::<u8>()).read(),
        ));
    }
}
pub(crate) unsafe extern "C" fn SetCameraToTrackGuestPlayer_2() {
    unsafe {
        InitCameraUpdateCallback(GetSpriteForLinkedPlayer(
            ((&raw mut gLocalLinkPlayerId).cast::<u8>().cast::<u8>()).read(),
        ));
    }
}
pub(crate) unsafe extern "C" fn OffsetCameraFocusByLinkPlayerId() {
    unsafe {
        let mut x: u16 = 0u16;
        let mut y: u16 = 0u16;
        GetCameraFocusCoords(&raw mut x, &raw mut y);
        SetCameraFocusCoords(
            ((((x) as i32).wrapping_add(
                ((((&raw mut gLocalLinkPlayerId).cast::<u8>().cast::<u8>()).read()) as i32),
            )) as u16),
            y,
        );
    }
}
pub(crate) unsafe extern "C" fn SpawnLinkPlayers() {
    unsafe {
        let mut i: u16 = 0u16;
        let mut x: u16 = 0u16;
        let mut y: u16 = 0u16;
        GetCameraFocusCoords(&raw mut x, &raw mut y);
        x = ((((x) as i32).wrapping_sub(
            ((((&raw mut gLocalLinkPlayerId).cast::<u8>().cast::<u8>()).read()) as i32),
        )) as u16);
        {
            i = 0u16;
            'l1: loop {
                if !(((i) as i32)
                    < ((((&raw mut gFieldLinkPlayerCount).cast::<u8>().cast::<u8>()).read())
                        as i32))
                {
                    break 'l1;
                }
                'l2: {
                    SpawnLinkPlayerObjectEvent(
                        ((i) as u8),
                        ((((i) as i32).wrapping_add(((x) as i32))) as i16),
                        ((y) as i16),
                        ((((&raw mut gLinkPlayers).cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 28))
                        .wrapping_add(19))
                        .read(),
                    );
                    CreateLinkPlayerSprite(
                        ((i) as u8),
                        ((((((&raw mut gLinkPlayers).cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 28))
                        .cast::<u16>())
                        .read()) as u8),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        ClearAllPlayerKeys();
    }
}
pub(crate) unsafe extern "C" fn CreateLinkPlayerSprites() {
    unsafe {
        let mut i: u16 = 0u16;
        {
            i = 0u16;
            'l1: loop {
                if !(((i) as i32)
                    < ((((&raw mut gFieldLinkPlayerCount).cast::<u8>().cast::<u8>()).read())
                        as i32))
                {
                    break 'l1;
                }
                'l2: {
                    CreateLinkPlayerSprite(
                        ((i) as u8),
                        ((((((&raw mut gLinkPlayers).cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 28))
                        .cast::<u16>())
                        .read()) as u8),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn CB1_OverworldLink() {
    unsafe {
        if ((((((&raw mut gWirelessCommType).cast::<u8>()).read()) as i32) == 0i32)
            || (!((IsRfuRecvQueueEmpty()) != 0)))
            || (!((IsSendingKeysToLink()) != 0))
        {
            let mut selfId: u8 = ((&raw mut gLocalLinkPlayerId).cast::<u8>().cast::<u8>()).read();
            UpdateAllLinkPlayers(
                ((&raw mut gLinkPartnersHeldKeys).cast::<u16>()).cast::<u16>(),
                ((selfId) as i32),
            );
            UpdateHeldKeyCode((((&raw mut sPlayerKeyInterceptCallback)
                .cast::<u8>()
                .cast::<Option<unsafe extern "C" fn(u32) -> u16>>())
            .read())
            .unwrap_unchecked()(((selfId) as u32)));
            ClearAllPlayerKeys();
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ResetAllMultiplayerState() {
    unsafe {
        ResetAllPlayerLinkStates();
        SetKeyInterceptCallback(Some(KeyInterCB_SelfIdle));
    }
}
pub(crate) unsafe extern "C" fn ClearAllPlayerKeys() {
    unsafe {
        ResetPlayerHeldKeys(((&raw mut gLinkPartnersHeldKeys).cast::<u16>()).cast::<u16>());
    }
}
pub(crate) unsafe extern "C" fn SetKeyInterceptCallback(
    func: Option<unsafe extern "C" fn(u32) -> u16>,
) {
    unsafe {
        let mut func = func;
        ((&raw mut sRfuKeepAliveTimer).cast::<u8>().cast::<u8>()).write(0u8);
        ((&raw mut sPlayerKeyInterceptCallback)
            .cast::<u8>()
            .cast::<Option<unsafe extern "C" fn(u32) -> u16>>())
        .write(func);
    }
}
pub(crate) unsafe extern "C" fn CheckRfuKeepAliveTimer() {
    unsafe {
        if (((((&raw mut gWirelessCommType).cast::<u8>()).read()) as i32) != 0i32)
            && ((({
                let __p1 = (&raw mut sRfuKeepAliveTimer).cast::<u8>().cast::<u8>();
                let __t2 = ((__p1).read()).wrapping_add(1);
                (__p1).write(__t2);
                __t2
            }) as i32)
                > 60i32)
        {
            LinkRfu_FatalError();
        }
    }
}
pub(crate) unsafe extern "C" fn ResetAllPlayerLinkStates() {
    unsafe {
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 4i32) {
                    break 'l1;
                }
                'l2: {
                    ((((&raw mut sPlayerLinkStates).cast::<u8>()).cast::<u8>())
                        .wrapping_offset((i) as isize))
                    .write(128u8);
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn AreAllPlayersInLinkState(state: u16) -> u32 {
    unsafe {
        let mut state = state;
        let mut i: i32 = 0i32;
        let mut count: i32 =
            ((((&raw mut gFieldLinkPlayerCount).cast::<u8>().cast::<u8>()).read()) as i32);
        {
            i = 0i32;
            'l1: loop {
                if !(i < count) {
                    break 'l1;
                }
                'l2: {
                    if ((((((&raw mut sPlayerLinkStates).cast::<u8>()).cast::<u8>())
                        .wrapping_offset((i) as isize))
                    .read()) as i32)
                        != ((state) as i32)
                    {
                        return 0u32;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return 1u32;
    }
}
pub(crate) unsafe extern "C" fn IsAnyPlayerInLinkState(state: u16) -> u32 {
    unsafe {
        let mut state = state;
        let mut i: i32 = 0i32;
        let mut count: i32 =
            ((((&raw mut gFieldLinkPlayerCount).cast::<u8>().cast::<u8>()).read()) as i32);
        {
            i = 0i32;
            'l1: loop {
                if !(i < count) {
                    break 'l1;
                }
                'l2: {
                    if ((((((&raw mut sPlayerLinkStates).cast::<u8>()).cast::<u8>())
                        .wrapping_offset((i) as isize))
                    .read()) as i32)
                        == ((state) as i32)
                    {
                        return 1u32;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return 0u32;
    }
}
pub(crate) unsafe extern "C" fn HandleLinkPlayerKeyInput(
    playerId: u32,
    key: u16,
    trainer: *mut u8,
    forceFacing: *mut u16,
) {
    unsafe {
        let mut playerId = playerId;
        let mut key = key;
        let mut trainer = trainer;
        let mut forceFacing = forceFacing;
        let mut script: *mut u8 = core::ptr::null_mut();
        if ((((((&raw mut sPlayerLinkStates).cast::<u8>()).cast::<u8>())
            .wrapping_offset(((playerId) as i32) as isize))
        .read()) as i32)
            == 128i32
        {
            script = TryGetTileEventScript(trainer);
            if !(script).is_null() {
                (forceFacing).write(GetDirectionForEventScript(script));
                ((((&raw mut sPlayerLinkStates).cast::<u8>()).cast::<u8>())
                    .wrapping_offset(((playerId) as i32) as isize))
                .write(129u8);
                if (((trainer).wrapping_add(1)).read()) != 0 {
                    SetKeyInterceptCallback(Some(KeyInterCB_DeferToEventScript));
                    RunInteractLocalPlayerScript(script);
                }
                return;
            }
            if IsAnyPlayerInLinkState(131u16) == 1u32 {
                ((((&raw mut sPlayerLinkStates).cast::<u8>()).cast::<u8>())
                    .wrapping_offset(((playerId) as i32) as isize))
                .write(129u8);
                if (((trainer).wrapping_add(1)).read()) != 0 {
                    SetKeyInterceptCallback(Some(KeyInterCB_DeferToEventScript));
                    RunTerminateLinkScript();
                }
                return;
            }
            'l1: {
                let __sw1 = ((key) as i32);
                if __sw1 == 24i32 {
                    if (CanCableClubPlayerPressStart(trainer)) != 0 {
                        ((((&raw mut sPlayerLinkStates).cast::<u8>()).cast::<u8>())
                            .wrapping_offset(((playerId) as i32) as isize))
                        .write(129u8);
                        if (((trainer).wrapping_add(1)).read()) != 0 {
                            SetKeyInterceptCallback(Some(KeyInterCB_DeferToEventScript));
                            InitLinkRoomStartMenuScript();
                        }
                    }
                    break 'l1;
                }
                if __sw1 == 18i32 {
                    if PlayerIsAtSouthExit(trainer) == 1u32 {
                        ((((&raw mut sPlayerLinkStates).cast::<u8>()).cast::<u8>())
                            .wrapping_offset(((playerId) as i32) as isize))
                        .write(129u8);
                        if (((trainer).wrapping_add(1)).read()) != 0 {
                            SetKeyInterceptCallback(Some(KeyInterCB_DeferToEventScript));
                            RunConfirmLeaveCableClubScript();
                        }
                    }
                    break 'l1;
                }
                if __sw1 == 25i32 {
                    script = TryInteractWithPlayer(trainer);
                    if !(script).is_null() {
                        ((((&raw mut sPlayerLinkStates).cast::<u8>()).cast::<u8>())
                            .wrapping_offset(((playerId) as i32) as isize))
                        .write(129u8);
                        if (((trainer).wrapping_add(1)).read()) != 0 {
                            SetKeyInterceptCallback(Some(KeyInterCB_DeferToEventScript));
                            InitMenuBasedScript(script);
                        }
                    }
                    break 'l1;
                }
                if __sw1 == 27i32 {
                    if (IsCableClubPlayerUnfrozen(trainer)) != 0 {
                        ((((&raw mut sPlayerLinkStates).cast::<u8>()).cast::<u8>())
                            .wrapping_offset(((playerId) as i32) as isize))
                        .write(129u8);
                        if (((trainer).wrapping_add(1)).read()) != 0 {
                            SetKeyInterceptCallback(Some(KeyInterCB_DeferToRecvQueue));
                            InitLinkPlayerQueueScript();
                        }
                    }
                    break 'l1;
                }
                if __sw1 == 28i32 {
                    if (IsCableClubPlayerUnfrozen(trainer)) != 0 {
                        ((((&raw mut sPlayerLinkStates).cast::<u8>()).cast::<u8>())
                            .wrapping_offset(((playerId) as i32) as isize))
                        .write(129u8);
                        if (((trainer).wrapping_add(1)).read()) != 0 {
                            SetKeyInterceptCallback(Some(KeyInterCB_DeferToSendQueue));
                            InitLinkPlayerQueueScript();
                        }
                    }
                    break 'l1;
                }
            }
        }
        'l2: {
            let __sw2 = ((key) as i32);
            if __sw2 == 23i32 {
                ((((&raw mut sPlayerLinkStates).cast::<u8>()).cast::<u8>())
                    .wrapping_offset(((playerId) as i32) as isize))
                .write(131u8);
                break 'l2;
            }
            if __sw2 == 22i32 {
                ((((&raw mut sPlayerLinkStates).cast::<u8>()).cast::<u8>())
                    .wrapping_offset(((playerId) as i32) as isize))
                .write(130u8);
                break 'l2;
            }
            if __sw2 == 26i32 {
                ((((&raw mut sPlayerLinkStates).cast::<u8>()).cast::<u8>())
                    .wrapping_offset(((playerId) as i32) as isize))
                .write(128u8);
                if (((trainer).wrapping_add(1)).read()) != 0 {
                    SetKeyInterceptCallback(Some(KeyInterCB_SelfIdle));
                }
                break 'l2;
            }
            if __sw2 == 29i32 {
                if ((((((&raw mut sPlayerLinkStates).cast::<u8>()).cast::<u8>())
                    .wrapping_offset(((playerId) as i32) as isize))
                .read()) as i32)
                    == 130i32
                {
                    ((((&raw mut sPlayerLinkStates).cast::<u8>()).cast::<u8>())
                        .wrapping_offset(((playerId) as i32) as isize))
                    .write(129u8);
                }
                break 'l2;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn UpdateAllLinkPlayers(keys: *mut u16, selfId: i32) {
    unsafe {
        let mut keys = keys;
        let mut selfId = selfId;
        let mut trainer = crate::ffi::Align4([0u8; 16]);
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 4i32) {
                    break 'l1;
                }
                'l2: {
                    let mut key: u8 = ((((keys).wrapping_offset((i) as isize)).read()) as u8);
                    let mut setFacing: u16 = 0u16;
                    LoadCableClubPlayer(i, selfId, (&raw mut trainer).cast::<u8>());
                    HandleLinkPlayerKeyInput(
                        ((i) as u32),
                        ((key) as u16),
                        (&raw mut trainer).cast::<u8>(),
                        &raw mut setFacing,
                    );
                    if ((((((&raw mut sPlayerLinkStates).cast::<u8>()).cast::<u8>())
                        .wrapping_offset((i) as isize))
                    .read()) as i32)
                        == 128i32
                    {
                        setFacing = GetDirectionForDpadKey(((key) as u16));
                    }
                    SetPlayerFacingDirection(((i) as u8), ((setFacing) as u8));
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn UpdateHeldKeyCode(key: u16) {
    unsafe {
        let mut key = key;
        if (((key) as i32) >= 17i32) && (((key) as i32) < 30i32) {
            ((&raw mut gHeldKeyCodeToSend).cast::<u8>().cast::<u16>()).write(key);
        } else {
            ((&raw mut gHeldKeyCodeToSend).cast::<u8>().cast::<u16>()).write(17u16);
        }
        if (((((((&raw mut gWirelessCommType).cast::<u8>()).read()) as i32) != 0i32)
            && (GetLinkSendQueueLength() > 1u32))
            && (IsOverworldLinkActive() == 1u32))
            && (IsSendingKeysToLink() == 1u32)
        {
            'l1: {
                let __sw1 = ((key) as i32);
                if __sw1 == 17i32
                    || __sw1 == 18i32
                    || __sw1 == 19i32
                    || __sw1 == 20i32
                    || __sw1 == 21i32
                    || __sw1 == 24i32
                    || __sw1 == 25i32
                {
                    ((&raw mut gHeldKeyCodeToSend).cast::<u8>().cast::<u16>()).write(0u16);
                    break 'l1;
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn KeyInterCB_ReadButtons(key: u32) -> u16 {
    unsafe {
        let mut key = key;
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(44)
            .cast::<u16>())
        .read()) as i32)
            & 64i32)
            != 0
        {
            return 19u16;
        }
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(44)
            .cast::<u16>())
        .read()) as i32)
            & 128i32)
            != 0
        {
            return 18u16;
        }
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(44)
            .cast::<u16>())
        .read()) as i32)
            & 32i32)
            != 0
        {
            return 20u16;
        }
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(44)
            .cast::<u16>())
        .read()) as i32)
            & 16i32)
            != 0
        {
            return 21u16;
        }
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(46)
            .cast::<u16>())
        .read()) as i32)
            & 8i32)
            != 0
        {
            return 24u16;
        }
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(46)
            .cast::<u16>())
        .read()) as i32)
            & 1i32)
            != 0
        {
            return 25u16;
        }
        return 17u16;
    }
}
pub(crate) unsafe extern "C" fn GetDirectionForDpadKey(key: u16) -> u16 {
    unsafe {
        let mut key = key;
        'l1: {
            let __sw1 = ((key) as i32);
            let __matched = __sw1 == 21i32 || __sw1 == 20i32 || __sw1 == 19i32 || __sw1 == 18i32;
            if __sw1 == 21i32 {
                return 4u16;
            }
            if __sw1 == 20i32 {
                return 3u16;
            }
            if __sw1 == 19i32 {
                return 1u16;
            }
            if __sw1 == 18i32 {
                return 2u16;
            }
            if !__matched {
                return 0u16;
            }
        }
        #[allow(unreachable_code)]
        {
            return 0u16;
        }
    }
}
pub(crate) unsafe extern "C" fn ResetPlayerHeldKeys(keys: *mut u16) {
    unsafe {
        let mut keys = keys;
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 4i32) {
                    break 'l1;
                }
                'l2: {
                    ((keys).wrapping_offset((i) as isize)).write(17u16);
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn KeyInterCB_SelfIdle(key: u32) -> u16 {
    unsafe {
        let mut key = key;
        if ((ArePlayerFieldControlsLocked()) as i32) == 1i32 {
            return 17u16;
        }
        if GetLinkRecvQueueLength() > 4u32 {
            return 27u16;
        }
        if GetLinkSendQueueLength() <= 4u32 {
            return KeyInterCB_ReadButtons(key);
        }
        return 28u16;
    }
}
pub(crate) unsafe extern "C" fn KeyInterCB_Idle(key: u32) -> u16 {
    unsafe {
        let mut key = key;
        CheckRfuKeepAliveTimer();
        return 17u16;
    }
}
pub(crate) unsafe extern "C" fn KeyInterCB_DeferToEventScript(key: u32) -> u16 {
    unsafe {
        let mut key = key;
        let mut retVal: u16 = 0u16;
        if ((ArePlayerFieldControlsLocked()) as i32) == 1i32 {
            retVal = 17u16;
        } else {
            retVal = 26u16;
            SetKeyInterceptCallback(Some(KeyInterCB_Idle));
        }
        return retVal;
    }
}
pub(crate) unsafe extern "C" fn KeyInterCB_DeferToRecvQueue(key: u32) -> u16 {
    unsafe {
        let mut key = key;
        let mut retVal: u16 = 0u16;
        if GetLinkRecvQueueLength() >= 3u32 {
            retVal = 17u16;
        } else {
            retVal = 26u16;
            UnlockPlayerFieldControls();
            SetKeyInterceptCallback(Some(KeyInterCB_Idle));
        }
        return retVal;
    }
}
pub(crate) unsafe extern "C" fn KeyInterCB_DeferToSendQueue(key: u32) -> u16 {
    unsafe {
        let mut key = key;
        let mut retVal: u16 = 0u16;
        if GetLinkSendQueueLength() > 2u32 {
            retVal = 17u16;
        } else {
            retVal = 26u16;
            UnlockPlayerFieldControls();
            SetKeyInterceptCallback(Some(KeyInterCB_Idle));
        }
        return retVal;
    }
}
pub(crate) unsafe extern "C" fn KeyInterCB_ExitingSeat(key: u32) -> u16 {
    unsafe {
        let mut key = key;
        CheckRfuKeepAliveTimer();
        return 17u16;
    }
}
pub(crate) unsafe extern "C" fn KeyInterCB_Ready(keyOrPlayerId: u32) -> u16 {
    unsafe {
        let mut keyOrPlayerId = keyOrPlayerId;
        if ((((((&raw mut sPlayerLinkStates).cast::<u8>()).cast::<u8>())
            .wrapping_offset(((keyOrPlayerId) as i32) as isize))
        .read()) as i32)
            == 130i32
        {
            if ((((((&raw mut gMain).cast::<u8>())
                .wrapping_add(46)
                .cast::<u16>())
            .read()) as i32)
                & 2i32)
                != 0
            {
                SetKeyInterceptCallback(Some(KeyInterCB_ExitingSeat));
                return 29u16;
            } else {
                return 17u16;
            }
        } else {
            CheckRfuKeepAliveTimer();
            return 17u16;
        }
        #[allow(unreachable_code)]
        {
            return 0u16;
        }
    }
}
pub(crate) unsafe extern "C" fn KeyInterCB_SetReady(key: u32) -> u16 {
    unsafe {
        let mut key = key;
        SetKeyInterceptCallback(Some(KeyInterCB_Ready));
        return 22u16;
    }
}
pub(crate) unsafe extern "C" fn KeyInterCB_SendNothing(key: u32) -> u16 {
    unsafe {
        let mut key = key;
        return 17u16;
    }
}
pub(crate) unsafe extern "C" fn KeyInterCB_WaitForPlayersToExit(keyOrPlayerId: u32) -> u16 {
    unsafe {
        let mut keyOrPlayerId = keyOrPlayerId;
        if ((((((&raw mut sPlayerLinkStates).cast::<u8>()).cast::<u8>())
            .wrapping_offset(((keyOrPlayerId) as i32) as isize))
        .read()) as i32)
            != 131i32
        {
            CheckRfuKeepAliveTimer();
        }
        if AreAllPlayersInLinkState(131u16) == 1u32 {
            ScriptContext_SetupScript((&raw mut EventScript_DoLinkRoomExit).cast::<u8>());
            SetKeyInterceptCallback(Some(KeyInterCB_SendNothing));
        }
        return 17u16;
    }
}
pub(crate) unsafe extern "C" fn KeyInterCB_SendExitRoomKey(key: u32) -> u16 {
    unsafe {
        let mut key = key;
        SetKeyInterceptCallback(Some(KeyInterCB_WaitForPlayersToExit));
        return 23u16;
    }
}
pub(crate) unsafe extern "C" fn KeyInterCB_InLinkActivity(key: u32) -> u16 {
    unsafe {
        let mut key = key;
        return 17u16;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetCableClubPartnersReady() -> u32 {
    unsafe {
        if IsAnyPlayerInLinkState(131u16) == 1u32 {
            return 2u32;
        }
        if (core::mem::transmute::<_, usize>(
            ((&raw mut sPlayerKeyInterceptCallback)
                .cast::<u8>()
                .cast::<Option<unsafe extern "C" fn(u32) -> u16>>())
            .read(),
        ) == (KeyInterCB_Ready as *const () as usize))
            && (((((((&raw mut sPlayerLinkStates).cast::<u8>()).cast::<u8>()).wrapping_offset(
                ((((&raw mut gLocalLinkPlayerId).cast::<u8>().cast::<u8>()).read()) as i32)
                    as isize,
            ))
            .read()) as i32)
                != 130i32)
        {
            return 0u32;
        }
        if (core::mem::transmute::<_, usize>(
            ((&raw mut sPlayerKeyInterceptCallback)
                .cast::<u8>()
                .cast::<Option<unsafe extern "C" fn(u32) -> u16>>())
            .read(),
        ) == (KeyInterCB_ExitingSeat as *const () as usize))
            && (((((((&raw mut sPlayerLinkStates).cast::<u8>()).cast::<u8>()).wrapping_offset(
                ((((&raw mut gLocalLinkPlayerId).cast::<u8>().cast::<u8>()).read()) as i32)
                    as isize,
            ))
            .read()) as i32)
                == 129i32)
        {
            return 2u32;
        }
        if (AreAllPlayersInLinkState(130u16)) != 0 {
            return 1u32;
        }
        return 0u32;
    }
}
pub(crate) unsafe extern "C" fn IsAnyPlayerExitingCableClub() -> u32 {
    unsafe {
        return IsAnyPlayerInLinkState(131u16);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetInCableClubSeat() -> u16 {
    unsafe {
        SetKeyInterceptCallback(Some(KeyInterCB_SetReady));
        return 0u16;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetLinkWaitingForScript() -> u16 {
    unsafe {
        SetKeyInterceptCallback(Some(KeyInterCB_DeferToEventScript));
        return 0u16;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn QueueExitLinkRoomKey() -> u16 {
    unsafe {
        SetKeyInterceptCallback(Some(KeyInterCB_SendExitRoomKey));
        return 0u16;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetStartedCableClubActivity() -> u16 {
    unsafe {
        SetKeyInterceptCallback(Some(KeyInterCB_InLinkActivity));
        return 0u16;
    }
}
pub(crate) unsafe extern "C" fn LoadCableClubPlayer(
    linkPlayerId: i32,
    myPlayerId: i32,
    trainer: *mut u8,
) {
    unsafe {
        let mut linkPlayerId = linkPlayerId;
        let mut myPlayerId = myPlayerId;
        let mut trainer = trainer;
        let mut x: i16 = 0i16;
        let mut y: i16 = 0i16;
        (trainer).write(((linkPlayerId) as u8));
        ((trainer).wrapping_add(1)).write(
            ((if linkPlayerId == myPlayerId {
                1i32
            } else {
                0i32
            }) as u8),
        );
        ((trainer).wrapping_add(2)).write(
            (((((&raw mut gLinkPlayerObjectEvents).cast::<u8>()).cast::<u8>())
                .wrapping_offset((linkPlayerId) as isize * 4))
            .wrapping_add(3))
            .read(),
        );
        ((trainer).wrapping_add(3)).write(GetLinkPlayerFacingDirection(((linkPlayerId) as u8)));
        GetLinkPlayerCoords(((linkPlayerId) as u8), &raw mut x, &raw mut y);
        (((trainer).wrapping_add(4)).cast::<i16>()).write(x);
        (((trainer).wrapping_add(4)).wrapping_add(2).cast::<i16>()).write(y);
        (((trainer).wrapping_add(4)).wrapping_add(4).cast::<i8>())
            .write(((GetLinkPlayerElevation(((linkPlayerId) as u8))) as i8));
        ((trainer).wrapping_add(12).cast::<u16>())
            .write(((MapGridGetMetatileBehaviorAt(((x) as i32), ((y) as i32))) as u16));
    }
}
pub(crate) unsafe extern "C" fn IsCableClubPlayerUnfrozen(player: *mut u8) -> u32 {
    unsafe {
        let mut player = player;
        let mut mode: u8 = ((player).wrapping_add(2)).read();
        if (((mode) as i32) == 2i32) || (((mode) as i32) == 0i32) {
            return 1u32;
        } else {
            return 0u32;
        }
        #[allow(unreachable_code)]
        {
            return 0u32;
        }
    }
}
pub(crate) unsafe extern "C" fn CanCableClubPlayerPressStart(player: *mut u8) -> u32 {
    unsafe {
        let mut player = player;
        let mut mode: u8 = ((player).wrapping_add(2)).read();
        if (((mode) as i32) == 2i32) || (((mode) as i32) == 0i32) {
            return 1u32;
        } else {
            return 0u32;
        }
        #[allow(unreachable_code)]
        {
            return 0u32;
        }
    }
}
pub(crate) unsafe extern "C" fn TryGetTileEventScript(player: *mut u8) -> *mut u8 {
    unsafe {
        let mut player = player;
        if ((((player).wrapping_add(2)).read()) as i32) != 2i32 {
            return core::ptr::null_mut();
        }
        return GetCoordEventScriptAtMapPosition((player).wrapping_add(4));
    }
}
pub(crate) unsafe extern "C" fn PlayerIsAtSouthExit(player: *mut u8) -> u32 {
    unsafe {
        let mut player = player;
        if (((((player).wrapping_add(2)).read()) as i32) != 2i32)
            && (((((player).wrapping_add(2)).read()) as i32) != 0i32)
        {
            return 0u32;
        } else {
            if !((MetatileBehavior_IsSouthArrowWarp(
                ((((player).wrapping_add(12).cast::<u16>()).read()) as u8),
            )) != 0)
            {
                return 0u32;
            } else {
                if ((((player).wrapping_add(3)).read()) as i32) != 1i32 {
                    return 0u32;
                } else {
                    return 1u32;
                }
            }
        }
        #[allow(unreachable_code)]
        {
            return 0u32;
        }
    }
}
pub(crate) unsafe extern "C" fn TryInteractWithPlayer(player: *mut u8) -> *mut u8 {
    unsafe {
        let mut player = player;
        let mut otherPlayerPos = crate::ffi::Align4([0u8; 8]);
        let mut linkPlayerId: u8 = 0u8;
        if (((((player).wrapping_add(2)).read()) as i32) != 0i32)
            && (((((player).wrapping_add(2)).read()) as i32) != 2i32)
        {
            return core::ptr::null_mut();
        }
        (&raw mut otherPlayerPos)
            .cast::<u8>()
            .cast::<crate::c::Rec4<8>>()
            .write_unaligned(
                (player)
                    .wrapping_add(4)
                    .cast::<crate::c::Rec4<8>>()
                    .read_unaligned(),
            );
        let __p1 = ((&raw mut otherPlayerPos).cast::<u8>()).cast::<i16>();
        (__p1).write(
            (((((__p1).read()) as u32).wrapping_add(
                (((((&raw const gDirectionToVectors).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset(((((player).wrapping_add(3)).read()) as i32) as isize * 8))
                .cast::<u32>())
                .read(),
            )) as i16),
        );
        let __p2 = ((&raw mut otherPlayerPos).cast::<u8>())
            .wrapping_add(2)
            .cast::<i16>();
        (__p2).write(
            (((((__p2).read()) as u32).wrapping_add(
                (((((&raw const gDirectionToVectors).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset(((((player).wrapping_add(3)).read()) as i32) as isize * 8))
                .wrapping_add(4)
                .cast::<u32>())
                .read(),
            )) as i16),
        );
        (((&raw mut otherPlayerPos).cast::<u8>())
            .wrapping_add(4)
            .cast::<i8>())
        .write(0i8);
        linkPlayerId = GetLinkPlayerIdAt(
            (((&raw mut otherPlayerPos).cast::<u8>()).cast::<i16>()).read(),
            (((&raw mut otherPlayerPos).cast::<u8>())
                .wrapping_add(2)
                .cast::<i16>())
            .read(),
        );
        if ((linkPlayerId) as i32) != 4i32 {
            if !((((player).wrapping_add(1)).read()) != 0) {
                return (&raw mut CableClub_EventScript_TooBusyToNotice).cast::<u8>();
            } else {
                if ((((((&raw mut sPlayerLinkStates).cast::<u8>()).cast::<u8>())
                    .wrapping_offset(((linkPlayerId) as i32) as isize))
                .read()) as i32)
                    != 128i32
                {
                    return (&raw mut CableClub_EventScript_TooBusyToNotice).cast::<u8>();
                } else {
                    if !((GetLinkTrainerCardColor(linkPlayerId)) != 0) {
                        return (&raw mut CableClub_EventScript_ReadTrainerCard).cast::<u8>();
                    } else {
                        return (&raw mut CableClub_EventScript_ReadTrainerCardColored)
                            .cast::<u8>();
                    }
                }
            }
        }
        return GetInteractedLinkPlayerScript(
            (&raw mut otherPlayerPos).cast::<u8>(),
            ((((player).wrapping_add(12).cast::<u16>()).read()) as u8),
            ((player).wrapping_add(3)).read(),
        );
    }
}
pub(crate) unsafe extern "C" fn GetDirectionForEventScript(script: *mut u8) -> u16 {
    unsafe {
        let mut script = script;
        if ((script) as usize)
            == (((&raw mut EventScript_BattleColosseum_4P_PlayerSpot0).cast::<u8>()) as usize)
        {
            return 10u16;
        } else {
            if ((script) as usize)
                == (((&raw mut EventScript_BattleColosseum_4P_PlayerSpot1).cast::<u8>()) as usize)
            {
                return 9u16;
            } else {
                if ((script) as usize)
                    == (((&raw mut EventScript_BattleColosseum_4P_PlayerSpot2).cast::<u8>())
                        as usize)
                {
                    return 10u16;
                } else {
                    if ((script) as usize)
                        == (((&raw mut EventScript_BattleColosseum_4P_PlayerSpot3).cast::<u8>())
                            as usize)
                    {
                        return 9u16;
                    } else {
                        if ((script) as usize)
                            == (((&raw mut EventScript_RecordCenter_Spot0).cast::<u8>()) as usize)
                        {
                            return 10u16;
                        } else {
                            if ((script) as usize)
                                == (((&raw mut EventScript_RecordCenter_Spot1).cast::<u8>())
                                    as usize)
                            {
                                return 9u16;
                            } else {
                                if ((script) as usize)
                                    == (((&raw mut EventScript_RecordCenter_Spot2).cast::<u8>())
                                        as usize)
                                {
                                    return 10u16;
                                } else {
                                    if ((script) as usize)
                                        == (((&raw mut EventScript_RecordCenter_Spot3).cast::<u8>())
                                            as usize)
                                    {
                                        return 9u16;
                                    } else {
                                        if ((script) as usize) == ((((&raw mut EventScript_BattleColosseum_2P_PlayerSpot0)).cast::<u8>()) as usize) {
return 10u16;
} else {
if ((script) as usize) == ((((&raw mut EventScript_BattleColosseum_2P_PlayerSpot1)).cast::<u8>()) as usize) {
return 9u16;
} else {
if ((script) as usize) == ((((&raw mut EventScript_TradeCenter_Chair0)).cast::<u8>()) as usize) {
return 10u16;
} else {
if ((script) as usize) == ((((&raw mut EventScript_TradeCenter_Chair1)).cast::<u8>()) as usize) {
return 9u16;
} else {
return 0u16;
}
}
}
}
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        #[allow(unreachable_code)]
        {
            return 0u16;
        }
    }
}
pub(crate) unsafe extern "C" fn InitLinkPlayerQueueScript() {
    unsafe {
        LockPlayerFieldControls();
    }
}
pub(crate) unsafe extern "C" fn InitLinkRoomStartMenuScript() {
    unsafe {
        PlaySE(6u16);
        ShowStartMenu();
        LockPlayerFieldControls();
    }
}
pub(crate) unsafe extern "C" fn RunInteractLocalPlayerScript(script: *mut u8) {
    unsafe {
        let mut script = script;
        PlaySE(5u16);
        ScriptContext_SetupScript(script);
        LockPlayerFieldControls();
    }
}
pub(crate) unsafe extern "C" fn RunConfirmLeaveCableClubScript() {
    unsafe {
        PlaySE(6u16);
        ScriptContext_SetupScript((&raw mut EventScript_ConfirmLeaveCableClubRoom).cast::<u8>());
        LockPlayerFieldControls();
    }
}
pub(crate) unsafe extern "C" fn InitMenuBasedScript(script: *mut u8) {
    unsafe {
        let mut script = script;
        PlaySE(5u16);
        ScriptContext_SetupScript(script);
        LockPlayerFieldControls();
    }
}
pub(crate) unsafe extern "C" fn RunTerminateLinkScript() {
    unsafe {
        ScriptContext_SetupScript((&raw mut EventScript_TerminateLink).cast::<u8>());
        LockPlayerFieldControls();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Overworld_IsRecvQueueAtMax() -> u32 {
    unsafe {
        if !((IsOverworldLinkActive()) != 0) {
            return 0u32;
        }
        if GetLinkRecvQueueLength() >= 3u32 {
            ((&raw mut sReceivingFromLink).cast::<u8>().cast::<u8>()).write(1u8);
        } else {
            ((&raw mut sReceivingFromLink).cast::<u8>().cast::<u8>()).write(0u8);
        }
        return ((((&raw mut sReceivingFromLink).cast::<u8>().cast::<u8>()).read()) as u32);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Overworld_RecvKeysFromLinkIsRunning() -> u32 {
    unsafe {
        let mut temp: u8 = 0u8;
        if GetLinkRecvQueueLength() < 2u32 {
            return 0u32;
        } else {
            if IsOverworldLinkActive() != 1u32 {
                return 0u32;
            } else {
                if IsSendingKeysToLink() != 1u32 {
                    return 0u32;
                } else {
                    if core::mem::transmute::<_, usize>(
                        ((&raw mut sPlayerKeyInterceptCallback)
                            .cast::<u8>()
                            .cast::<Option<unsafe extern "C" fn(u32) -> u16>>())
                        .read(),
                    ) == (KeyInterCB_DeferToRecvQueue as *const () as usize)
                    {
                        return 1u32;
                    } else {
                        if core::mem::transmute::<_, usize>(
                            ((&raw mut sPlayerKeyInterceptCallback)
                                .cast::<u8>()
                                .cast::<Option<unsafe extern "C" fn(u32) -> u16>>())
                            .read(),
                        ) != (KeyInterCB_DeferToEventScript as *const () as usize)
                        {
                            return 0u32;
                        }
                    }
                }
            }
        }
        temp = ((&raw mut sReceivingFromLink).cast::<u8>().cast::<u8>()).read();
        ((&raw mut sReceivingFromLink).cast::<u8>().cast::<u8>()).write(0u8);
        if ((temp) as i32) == 1i32 {
            return 1u32;
        } else {
            if ((crate::c::bf_read(
                ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
                7,
                1,
                false,
            ) as u16)
                != 0)
                && ((crate::c::bf_read(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(10),
                    5,
                    1,
                    false,
                ) as u16)
                    != 0)
            {
                return 1u32;
            } else {
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
pub unsafe extern "C" fn Overworld_SendKeysToLinkIsRunning() -> u32 {
    unsafe {
        if GetLinkSendQueueLength() < 2u32 {
            return 0u32;
        } else {
            if IsOverworldLinkActive() != 1u32 {
                return 0u32;
            } else {
                if IsSendingKeysToLink() != 1u32 {
                    return 0u32;
                } else {
                    if core::mem::transmute::<_, usize>(
                        ((&raw mut sPlayerKeyInterceptCallback)
                            .cast::<u8>()
                            .cast::<Option<unsafe extern "C" fn(u32) -> u16>>())
                        .read(),
                    ) == (KeyInterCB_DeferToSendQueue as *const () as usize)
                    {
                        return 1u32;
                    } else {
                        return 0u32;
                    }
                }
            }
        }
        #[allow(unreachable_code)]
        {
            return 0u32;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsSendingKeysOverCable() -> u32 {
    unsafe {
        if ((((&raw mut gWirelessCommType).cast::<u8>()).read()) as i32) != 0i32 {
            return 0u32;
        } else {
            if !((IsSendingKeysToLink()) != 0) {
                return 0u32;
            } else {
                return 1u32;
            }
        }
        #[allow(unreachable_code)]
        {
            return 0u32;
        }
    }
}
pub(crate) unsafe extern "C" fn GetLinkSendQueueLength() -> u32 {
    unsafe {
        if ((((&raw mut gWirelessCommType).cast::<u8>()).read()) as i32) != 0i32 {
            return ((((((&raw mut gRfu).cast::<u8>()).wrapping_add(2536)).wrapping_add(562))
                .read_volatile()) as u32);
        } else {
            return ((((((&raw mut gLink).cast::<u8>()).wrapping_add(24)).wrapping_add(801)).read())
                as u32);
        }
        #[allow(unreachable_code)]
        {
            return 0u32;
        }
    }
}
pub(crate) unsafe extern "C" fn ZeroLinkPlayerObjectEvent(linkPlayerObjEvent: *mut u8) {
    unsafe {
        let mut linkPlayerObjEvent = linkPlayerObjEvent;
        crate::c::memset(linkPlayerObjEvent, 0i32, 4u32);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ClearLinkPlayerObjectEvents() {
    unsafe {
        crate::c::memset(
            ((&raw mut gLinkPlayerObjectEvents).cast::<u8>()).cast::<u8>(),
            0i32,
            16u32,
        );
    }
}
pub(crate) unsafe extern "C" fn ZeroObjectEvent(objEvent: *mut u8) {
    unsafe {
        let mut objEvent = objEvent;
        crate::c::memset(objEvent, 0i32, 36u32);
    }
}
pub(crate) unsafe extern "C" fn SpawnLinkPlayerObjectEvent(
    linkPlayerId: u8,
    x: i16,
    y: i16,
    gender: u8,
) {
    unsafe {
        let mut linkPlayerId = linkPlayerId;
        let mut x = x;
        let mut y = y;
        let mut gender = gender;
        let mut objEventId: u8 = GetFirstInactiveObjectEventId();
        let mut linkPlayerObjEvent: *mut u8 = (((&raw mut gLinkPlayerObjectEvents).cast::<u8>())
            .cast::<u8>())
        .wrapping_offset(((linkPlayerId) as i32) as isize * 4);
        let mut objEvent: *mut u8 = ((&raw mut gObjectEvents).cast::<u8>())
            .wrapping_offset(((objEventId) as i32) as isize * 36);
        ZeroLinkPlayerObjectEvent(linkPlayerObjEvent);
        ZeroObjectEvent(objEvent);
        (linkPlayerObjEvent).write(1u8);
        ((linkPlayerObjEvent).wrapping_add(1)).write(linkPlayerId);
        ((linkPlayerObjEvent).wrapping_add(2)).write(objEventId);
        ((linkPlayerObjEvent).wrapping_add(3)).write(0u8);
        crate::c::bf_write((objEvent).wrapping_add(0), 0, 1, (1u32) as i32);
        crate::c::bf_write((objEvent).wrapping_add(0), 1, 1, ((gender) as u32) as i32);
        ((objEvent).wrapping_offset(25)).write(2u8);
        ((objEvent).wrapping_add(4)).write(64u8);
        InitLinkPlayerObjectEventPos(objEvent, x, y);
    }
}
pub(crate) unsafe extern "C" fn InitLinkPlayerObjectEventPos(objEvent: *mut u8, x: i16, y: i16) {
    unsafe {
        let mut objEvent = objEvent;
        let mut x = x;
        let mut y = y;
        (((objEvent).wrapping_add(16)).cast::<i16>()).write(x);
        (((objEvent).wrapping_add(16)).wrapping_add(2).cast::<i16>()).write(y);
        (((objEvent).wrapping_add(20)).cast::<i16>()).write(x);
        (((objEvent).wrapping_add(20)).wrapping_add(2).cast::<i16>()).write(y);
        SetSpritePosToMapCoords(
            x,
            y,
            ((objEvent).wrapping_add(12)).cast::<i16>(),
            ((objEvent).wrapping_add(12)).wrapping_add(2).cast::<i16>(),
        );
        let __p1 = ((objEvent).wrapping_add(12)).cast::<i16>();
        (__p1).write((((((__p1).read()) as i32).wrapping_add(8i32)) as i16));
        ObjectEventUpdateElevation(objEvent);
    }
}
pub(crate) unsafe extern "C" fn SetLinkPlayerObjectRange(linkPlayerId: u8, dir: u8) {
    unsafe {
        let mut linkPlayerId = linkPlayerId;
        let mut dir = dir;
        if (((((&raw mut gLinkPlayerObjectEvents).cast::<u8>()).cast::<u8>())
            .wrapping_offset(((linkPlayerId) as i32) as isize * 4))
        .read())
            != 0
        {
            let mut objEventId: u8 = (((((&raw mut gLinkPlayerObjectEvents).cast::<u8>())
                .cast::<u8>())
            .wrapping_offset(((linkPlayerId) as i32) as isize * 4))
            .wrapping_add(2))
            .read();
            let mut objEvent: *mut u8 = ((&raw mut gObjectEvents).cast::<u8>())
                .wrapping_offset(((objEventId) as i32) as isize * 36);
            ((objEvent).wrapping_offset(25)).write(dir);
        }
    }
}
pub(crate) unsafe extern "C" fn DestroyLinkPlayerObject(linkPlayerId: u8) {
    unsafe {
        let mut linkPlayerId = linkPlayerId;
        let mut linkPlayerObjEvent: *mut u8 = (((&raw mut gLinkPlayerObjectEvents).cast::<u8>())
            .cast::<u8>())
        .wrapping_offset(((linkPlayerId) as i32) as isize * 4);
        let mut objEventId: u8 = ((linkPlayerObjEvent).wrapping_add(2)).read();
        let mut objEvent: *mut u8 = ((&raw mut gObjectEvents).cast::<u8>())
            .wrapping_offset(((objEventId) as i32) as isize * 36);
        if ((((objEvent).wrapping_add(4)).read()) as i32) != 64i32 {
            DestroySprite(
                ((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((((objEvent).wrapping_add(4)).read()) as i32) as isize * 68),
            );
        }
        (linkPlayerObjEvent).write(0u8);
        crate::c::bf_write((objEvent).wrapping_add(0), 0, 1, (0u32) as i32);
    }
}
pub(crate) unsafe extern "C" fn GetSpriteForLinkedPlayer(linkPlayerId: u8) -> u8 {
    unsafe {
        let mut linkPlayerId = linkPlayerId;
        let mut objEventId: u8 = (((((&raw mut gLinkPlayerObjectEvents).cast::<u8>())
            .cast::<u8>())
        .wrapping_offset(((linkPlayerId) as i32) as isize * 4))
        .wrapping_add(2))
        .read();
        let mut objEvent: *mut u8 = ((&raw mut gObjectEvents).cast::<u8>())
            .wrapping_offset(((objEventId) as i32) as isize * 36);
        return ((objEvent).wrapping_add(4)).read();
    }
}
pub(crate) unsafe extern "C" fn GetLinkPlayerCoords(linkPlayerId: u8, x: *mut i16, y: *mut i16) {
    unsafe {
        let mut linkPlayerId = linkPlayerId;
        let mut x = x;
        let mut y = y;
        let mut objEventId: u8 = (((((&raw mut gLinkPlayerObjectEvents).cast::<u8>())
            .cast::<u8>())
        .wrapping_offset(((linkPlayerId) as i32) as isize * 4))
        .wrapping_add(2))
        .read();
        let mut objEvent: *mut u8 = ((&raw mut gObjectEvents).cast::<u8>())
            .wrapping_offset(((objEventId) as i32) as isize * 36);
        (x).write((((objEvent).wrapping_add(16)).cast::<i16>()).read());
        (y).write((((objEvent).wrapping_add(16)).wrapping_add(2).cast::<i16>()).read());
    }
}
pub(crate) unsafe extern "C" fn GetLinkPlayerFacingDirection(linkPlayerId: u8) -> u8 {
    unsafe {
        let mut linkPlayerId = linkPlayerId;
        let mut objEventId: u8 = (((((&raw mut gLinkPlayerObjectEvents).cast::<u8>())
            .cast::<u8>())
        .wrapping_offset(((linkPlayerId) as i32) as isize * 4))
        .wrapping_add(2))
        .read();
        let mut objEvent: *mut u8 = ((&raw mut gObjectEvents).cast::<u8>())
            .wrapping_offset(((objEventId) as i32) as isize * 36);
        return ((objEvent).wrapping_offset(25)).read();
    }
}
pub(crate) unsafe extern "C" fn GetLinkPlayerElevation(linkPlayerId: u8) -> u8 {
    unsafe {
        let mut linkPlayerId = linkPlayerId;
        let mut objEventId: u8 = (((((&raw mut gLinkPlayerObjectEvents).cast::<u8>())
            .cast::<u8>())
        .wrapping_offset(((linkPlayerId) as i32) as isize * 4))
        .wrapping_add(2))
        .read();
        let mut objEvent: *mut u8 = ((&raw mut gObjectEvents).cast::<u8>())
            .wrapping_offset(((objEventId) as i32) as isize * 36);
        return (crate::c::bf_read((objEvent).wrapping_add(11), 0, 4, false) as u8);
    }
}
pub(crate) unsafe extern "C" fn GetLinkPlayerObjectStepTimer(linkPlayerId: u8) -> i16 {
    unsafe {
        let mut linkPlayerId = linkPlayerId;
        let mut objEventId: u8 = (((((&raw mut gLinkPlayerObjectEvents).cast::<u8>())
            .cast::<u8>())
        .wrapping_offset(((linkPlayerId) as i32) as isize * 4))
        .wrapping_add(2))
        .read();
        let mut objEvent: *mut u8 = ((&raw mut gObjectEvents).cast::<u8>())
            .wrapping_offset(((objEventId) as i32) as isize * 36);
        return (((16i32).wrapping_sub((((((objEvent).wrapping_add(33)).read()) as i8) as i32)))
            as i16);
    }
}
pub(crate) unsafe extern "C" fn GetLinkPlayerIdAt(x: i16, y: i16) -> u8 {
    unsafe {
        let mut x = x;
        let mut y = y;
        let mut i: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 4i32) {
                    break 'l1;
                }
                'l2: {
                    if ((((((&raw mut gLinkPlayerObjectEvents).cast::<u8>()).cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 4))
                    .read())
                        != 0)
                        && (((((((((&raw mut gLinkPlayerObjectEvents).cast::<u8>()).cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 4))
                        .wrapping_add(3))
                        .read()) as i32)
                            == 0i32)
                            || ((((((((&raw mut gLinkPlayerObjectEvents).cast::<u8>())
                                .cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 4))
                            .wrapping_add(3))
                            .read()) as i32)
                                == 2i32))
                    {
                        let mut objEvent: *mut u8 = ((&raw mut gObjectEvents).cast::<u8>())
                            .wrapping_offset(
                                (((((((&raw mut gLinkPlayerObjectEvents).cast::<u8>())
                                    .cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize * 4))
                                .wrapping_add(2))
                                .read()) as i32) as isize
                                    * 36,
                            );
                        if ((((((objEvent).wrapping_add(16)).cast::<i16>()).read()) as i32)
                            == ((x) as i32))
                            && ((((((objEvent).wrapping_add(16)).wrapping_add(2).cast::<i16>())
                                .read()) as i32)
                                == ((y) as i32))
                        {
                            return i;
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return 4u8;
    }
}
pub(crate) unsafe extern "C" fn SetPlayerFacingDirection(linkPlayerId: u8, facing: u8) {
    unsafe {
        let mut linkPlayerId = linkPlayerId;
        let mut facing = facing;
        let mut linkPlayerObjEvent: *mut u8 = (((&raw mut gLinkPlayerObjectEvents).cast::<u8>())
            .cast::<u8>())
        .wrapping_offset(((linkPlayerId) as i32) as isize * 4);
        let mut objEventId: u8 = ((linkPlayerObjEvent).wrapping_add(2)).read();
        let mut objEvent: *mut u8 = ((&raw mut gObjectEvents).cast::<u8>())
            .wrapping_offset(((objEventId) as i32) as isize * 36);
        if !(((linkPlayerObjEvent).read()) != 0) {
            return;
        }
        if ((facing) as i32) > 10i32 {
            crate::c::bf_write((objEvent).wrapping_add(0), 2, 1, (1u32) as i32);
            return;
        }
        (((((&raw const sMovementStatusHandler)
            .cast::<u8>()
            .cast_mut()
            .cast::<Option<unsafe extern "C" fn(*mut u8, *mut u8)>>())
        .cast::<Option<unsafe extern "C" fn(*mut u8, *mut u8)>>())
        .wrapping_offset(
            (((((((&raw const sLinkPlayerMovementModes)
                .cast::<u8>()
                .cast_mut()
                .cast::<Option<unsafe extern "C" fn(*mut u8, *mut u8, u8) -> u8>>())
            .cast::<Option<unsafe extern "C" fn(*mut u8, *mut u8, u8) -> u8>>())
            .wrapping_offset(((((linkPlayerObjEvent).wrapping_add(3)).read()) as i32) as isize))
            .read())
            .unwrap_unchecked()(linkPlayerObjEvent, objEvent, facing)) as i32) as isize,
        ))
        .read())
        .unwrap_unchecked()(linkPlayerObjEvent, objEvent);
    }
}
pub(crate) unsafe extern "C" fn MovementEventModeCB_Normal(
    linkPlayerObjEvent: *mut u8,
    objEvent: *mut u8,
    dir: u8,
) -> u8 {
    unsafe {
        let mut linkPlayerObjEvent = linkPlayerObjEvent;
        let mut objEvent = objEvent;
        let mut dir = dir;
        return (((((&raw const sLinkPlayerFacingHandlers)
            .cast::<u8>()
            .cast_mut()
            .cast::<Option<unsafe extern "C" fn(*mut u8, *mut u8, u8) -> u8>>())
        .cast::<Option<unsafe extern "C" fn(*mut u8, *mut u8, u8) -> u8>>())
        .wrapping_offset(((dir) as i32) as isize))
        .read())
        .unwrap_unchecked()(linkPlayerObjEvent, objEvent, dir);
    }
}
pub(crate) unsafe extern "C" fn MovementEventModeCB_Ignored(
    linkPlayerObjEvent: *mut u8,
    objEvent: *mut u8,
    dir: u8,
) -> u8 {
    unsafe {
        let mut linkPlayerObjEvent = linkPlayerObjEvent;
        let mut objEvent = objEvent;
        let mut dir = dir;
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn MovementEventModeCB_Scripted(
    linkPlayerObjEvent: *mut u8,
    objEvent: *mut u8,
    dir: u8,
) -> u8 {
    unsafe {
        let mut linkPlayerObjEvent = linkPlayerObjEvent;
        let mut objEvent = objEvent;
        let mut dir = dir;
        return (((((&raw const sLinkPlayerFacingHandlers)
            .cast::<u8>()
            .cast_mut()
            .cast::<Option<unsafe extern "C" fn(*mut u8, *mut u8, u8) -> u8>>())
        .cast::<Option<unsafe extern "C" fn(*mut u8, *mut u8, u8) -> u8>>())
        .wrapping_offset(((dir) as i32) as isize))
        .read())
        .unwrap_unchecked()(linkPlayerObjEvent, objEvent, dir);
    }
}
pub(crate) unsafe extern "C" fn FacingHandler_DoNothing(
    linkPlayerObjEvent: *mut u8,
    objEvent: *mut u8,
    dir: u8,
) -> u8 {
    unsafe {
        let mut linkPlayerObjEvent = linkPlayerObjEvent;
        let mut objEvent = objEvent;
        let mut dir = dir;
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn FacingHandler_DpadMovement(
    linkPlayerObjEvent: *mut u8,
    objEvent: *mut u8,
    dir: u8,
) -> u8 {
    unsafe {
        let mut linkPlayerObjEvent = linkPlayerObjEvent;
        let mut objEvent = objEvent;
        let mut dir = dir;
        let mut x: i16 = 0i16;
        let mut y: i16 = 0i16;
        ((objEvent).wrapping_offset(25)).write(FlipVerticalAndClearForced(
            dir,
            ((objEvent).wrapping_offset(25)).read(),
        ));
        ObjectEventMoveDestCoords(
            objEvent,
            ((((objEvent).wrapping_offset(25)).read()) as u32),
            &raw mut x,
            &raw mut y,
        );
        if (LinkPlayerGetCollision(
            ((linkPlayerObjEvent).wrapping_add(2)).read(),
            ((objEvent).wrapping_offset(25)).read(),
            x,
            y,
        )) != 0
        {
            return 0u8;
        } else {
            ((objEvent).wrapping_add(33)).write(16u8);
            ShiftObjectEventCoords(objEvent, x, y);
            ObjectEventUpdateElevation(objEvent);
            return 1u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
pub(crate) unsafe extern "C" fn FacingHandler_ForcedFacingChange(
    linkPlayerObjEvent: *mut u8,
    objEvent: *mut u8,
    dir: u8,
) -> u8 {
    unsafe {
        let mut linkPlayerObjEvent = linkPlayerObjEvent;
        let mut objEvent = objEvent;
        let mut dir = dir;
        ((objEvent).wrapping_offset(25)).write(FlipVerticalAndClearForced(
            dir,
            ((objEvent).wrapping_offset(25)).read(),
        ));
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn MovementStatusHandler_EnterFreeMode(
    linkPlayerObjEvent: *mut u8,
    objEvent: *mut u8,
) {
    unsafe {
        let mut linkPlayerObjEvent = linkPlayerObjEvent;
        let mut objEvent = objEvent;
        ((linkPlayerObjEvent).wrapping_add(3)).write(0u8);
    }
}
pub(crate) unsafe extern "C" fn MovementStatusHandler_TryAdvanceScript(
    linkPlayerObjEvent: *mut u8,
    objEvent: *mut u8,
) {
    unsafe {
        let mut linkPlayerObjEvent = linkPlayerObjEvent;
        let mut objEvent = objEvent;
        let __p1 = (objEvent).wrapping_add(33);
        (__p1).write(((__p1).read()).wrapping_sub(1));
        ((linkPlayerObjEvent).wrapping_add(3)).write(1u8);
        MoveCoords(
            ((objEvent).wrapping_offset(25)).read(),
            ((objEvent).wrapping_add(12)).cast::<i16>(),
            ((objEvent).wrapping_add(12)).wrapping_add(2).cast::<i16>(),
        );
        if !((((objEvent).wrapping_add(33)).read()) != 0) {
            ShiftStillObjectEventCoords(objEvent);
            ((linkPlayerObjEvent).wrapping_add(3)).write(2u8);
        }
    }
}
pub(crate) unsafe extern "C" fn FlipVerticalAndClearForced(newFacing: u8, oldFacing: u8) -> u8 {
    unsafe {
        let mut newFacing = newFacing;
        let mut oldFacing = oldFacing;
        'l1: {
            let __sw1 = ((newFacing) as i32);
            if __sw1 == 1i32 || __sw1 == 7i32 {
                return 2u8;
            }
            if __sw1 == 2i32 || __sw1 == 8i32 {
                return 1u8;
            }
            if __sw1 == 3i32 || __sw1 == 9i32 {
                return 3u8;
            }
            if __sw1 == 4i32 || __sw1 == 10i32 {
                return 4u8;
            }
        }
        return oldFacing;
    }
}
pub(crate) unsafe extern "C" fn LinkPlayerGetCollision(
    selfObjEventId: u8,
    direction: u8,
    x: i16,
    y: i16,
) -> u8 {
    unsafe {
        let mut selfObjEventId = selfObjEventId;
        let mut direction = direction;
        let mut x = x;
        let mut y = y;
        let mut i: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 16i32) {
                    break 'l1;
                }
                'l2: {
                    if ((i) as i32) != ((selfObjEventId) as i32) {
                        if (((((((((&raw mut gObjectEvents).cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 36))
                        .wrapping_add(16))
                        .cast::<i16>())
                        .read()) as i32)
                            == ((x) as i32))
                            && ((((((((&raw mut gObjectEvents).cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize * 36))
                            .wrapping_add(16))
                            .wrapping_add(2)
                            .cast::<i16>())
                            .read()) as i32)
                                == ((y) as i32)))
                            || (((((((((&raw mut gObjectEvents).cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize * 36))
                            .wrapping_add(20))
                            .cast::<i16>())
                            .read()) as i32)
                                == ((x) as i32))
                                && ((((((((&raw mut gObjectEvents).cast::<u8>())
                                    .wrapping_offset(((i) as i32) as isize * 36))
                                .wrapping_add(20))
                                .wrapping_add(2)
                                .cast::<i16>())
                                .read()) as i32)
                                    == ((y) as i32)))
                        {
                            return 1u8;
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return MapGridGetCollisionAt(((x) as i32), ((y) as i32));
    }
}
pub(crate) unsafe extern "C" fn CreateLinkPlayerSprite(linkPlayerId: u8, gameVersion: u8) {
    unsafe {
        let mut linkPlayerId = linkPlayerId;
        let mut gameVersion = gameVersion;
        let mut linkPlayerObjEvent: *mut u8 = (((&raw mut gLinkPlayerObjectEvents).cast::<u8>())
            .cast::<u8>())
        .wrapping_offset(((linkPlayerId) as i32) as isize * 4);
        let mut objEventId: u8 = ((linkPlayerObjEvent).wrapping_add(2)).read();
        let mut objEvent: *mut u8 = ((&raw mut gObjectEvents).cast::<u8>())
            .wrapping_offset(((objEventId) as i32) as isize * 36);
        let mut sprite: *mut u8 = core::ptr::null_mut();
        if ((linkPlayerObjEvent).read()) != 0 {
            'l1: {
                let __sw1 = ((gameVersion) as i32);
                if __sw1 == 4i32 || __sw1 == 5i32 {
                    ((objEvent).wrapping_add(4)).write(CreateObjectGraphicsSprite(
                        ((GetFRLGAvatarGraphicsIdByGender(
                            ((crate::c::bf_read((objEvent).wrapping_add(0), 1, 1, false) as u32)
                                as u8),
                        )) as u16),
                        Some(SpriteCB_LinkPlayer),
                        0i16,
                        0i16,
                        0u8,
                    ));
                    break 'l1;
                }
                if __sw1 == 2i32 || __sw1 == 1i32 {
                    ((objEvent).wrapping_add(4)).write(CreateObjectGraphicsSprite(
                        ((GetRSAvatarGraphicsIdByGender(
                            ((crate::c::bf_read((objEvent).wrapping_add(0), 1, 1, false) as u32)
                                as u8),
                        )) as u16),
                        Some(SpriteCB_LinkPlayer),
                        0i16,
                        0i16,
                        0u8,
                    ));
                    break 'l1;
                }
                if __sw1 == 3i32 {
                    ((objEvent).wrapping_add(4)).write(CreateObjectGraphicsSprite(
                        ((GetRivalAvatarGraphicsIdByStateIdAndGender(
                            0u8,
                            ((crate::c::bf_read((objEvent).wrapping_add(0), 1, 1, false) as u32)
                                as u8),
                        )) as u16),
                        Some(SpriteCB_LinkPlayer),
                        0i16,
                        0i16,
                        0u8,
                    ));
                    break 'l1;
                }
            }
            sprite = ((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((((objEvent).wrapping_add(4)).read()) as i32) as isize * 68);
            crate::c::bf_write((sprite).wrapping_add(62), 1, 1, (1u16) as i32);
            (((sprite).wrapping_add(46)).cast::<i16>()).write(((linkPlayerId) as i16));
            crate::c::bf_write((objEvent).wrapping_add(0), 2, 1, (0u32) as i32);
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_LinkPlayer(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut linkPlayerObjEvent: *mut u8 =
            (((&raw mut gLinkPlayerObjectEvents).cast::<u8>()).cast::<u8>()).wrapping_offset(
                (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) as isize * 4,
            );
        let mut objEvent: *mut u8 = ((&raw mut gObjectEvents).cast::<u8>()).wrapping_offset(
            ((((linkPlayerObjEvent).wrapping_add(2)).read()) as i32) as isize * 36,
        );
        ((sprite).wrapping_add(32).cast::<i16>())
            .write((((objEvent).wrapping_add(12)).cast::<i16>()).read());
        ((sprite).wrapping_add(34).cast::<i16>())
            .write((((objEvent).wrapping_add(12)).wrapping_add(2).cast::<i16>()).read());
        SetObjectSubpriorityByElevation(
            (crate::c::bf_read((objEvent).wrapping_add(11), 4, 4, false) as u8),
            sprite,
            1u8,
        );
        crate::c::bf_write(
            (sprite).wrapping_add(5),
            2,
            2,
            ((ElevationToPriority(
                (crate::c::bf_read((objEvent).wrapping_add(11), 4, 4, false) as u8),
            )) as u16) as i32,
        );
        if ((((linkPlayerObjEvent).wrapping_add(3)).read()) as i32) == 0i32 {
            StartSpriteAnim(
                sprite,
                GetFaceDirectionAnimNum(((objEvent).wrapping_offset(25)).read()),
            );
        } else {
            StartSpriteAnimIfDifferent(
                sprite,
                GetMoveDirectionAnimNum(((objEvent).wrapping_offset(25)).read()),
            );
        }
        UpdateObjectEventSpriteInvisibility(sprite, 0u8);
        if (crate::c::bf_read((objEvent).wrapping_add(0), 2, 1, false) as u32) != 0 {
            crate::c::bf_write(
                (sprite).wrapping_add(62),
                2,
                1,
                (((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read())
                    as i32)
                    & 4i32)
                    >> 2) as u16) as i32,
            );
            let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7);
            (__p1).write(((__p1).read()).wrapping_add(1));
        }
    }
}
