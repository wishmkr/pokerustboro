//! Translated from `src/field_screen_effect.c` by tools/rustport/c2rs.py.
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
// Data tables (translate with cdata.py): sFlashLevelToRadius gMaxFlashLevel sFlashEffectParams

static sFlashEffectParams: Table<ScanlineEffectParams> =
    Table((&raw const crate::data::field_screen_effect::sFlashEffectParams).cast());
static sFlashLevelToRadius: Table<CArray<u16, 9>> =
    Table((&raw const crate::data::field_screen_effect::sFlashLevelToRadius).cast());

unsafe extern "C" {
    static mut gFieldCallback: Option<unsafe extern "C" fn()>;
    static mut gObjectEvents: CArray<ObjectEvent, 16>;
    static gOrbEffectBackgroundLayerFlags: CArray<u16, 0>;
    static mut gPaletteFade: PaletteFadeControl;
    static mut gPlttBufferFaded: CArray<u16, 512>;
    static mut gReceivedRemoteLinkPlayers: u8;
    static mut gSaveBlock2Ptr: *mut SaveBlock2;
    static mut gScanlineEffect: ScanlineEffect;
    static mut gScanlineEffectRegBuffers: CArray<CArray<u16, 960>, 2>;
    static mut gSpecialVar_Result: u16;
    static mut gTasks: CArray<Task, 0>;
    fn BGMusicStopped() -> u8;
    fn BgDmaFill(a0: u32, a1: u8, a2: i32, a3: i32);
    fn CB2_LoadMap();
    fn CB2_ReturnToFieldCableClub();
    fn CB2_ReturnToFieldContestHall();
    fn ClearGpuRegBits(a0: u8, a1: u16);
    fn ClearLinkCallback_2();
    fn ClearMirageTowerPulseBlendEffect();
    fn CpuFastSet(a0: *mut c_void, a1: *mut c_void, a2: u32);
    fn CreateTask(a0: Option<unsafe extern "C" fn(u8)>, a1: u8) -> u8;
    fn CreateTask_ReestablishCableClubLink() -> u8;
    fn DestroyTask(a0: u8);
    fn DoPlayerSpinEntrance();
    fn DoPlayerSpinExit();
    fn FadeScreen(a0: u8, a1: i8);
    fn FieldAnimateDoorClose(a0: u32, a1: u32) -> i8;
    fn FieldAnimateDoorOpen(a0: u32, a1: u32) -> i8;
    fn FieldCB_FallWarpExit();
    fn FieldCB_ShowPortholeView();
    fn FieldSetDoorOpened(a0: u32, a1: u32);
    fn FindTaskIdByFunc(a0: Option<unsafe extern "C" fn(u8)>) -> u8;
    fn FreezeObjectEvents();
    fn FuncIsActiveTask(a0: Option<unsafe extern "C" fn(u8)>) -> u8;
    fn GetCurrentMapType() -> u8;
    fn GetDestinationWarpMapHeader() -> *mut MapHeader;
    fn GetDoorSoundEffect(a0: u32, a1: u32) -> u32;
    fn GetFlashLevel() -> u8;
    fn GetLastUsedWarpMapType() -> u8;
    fn GetMapPairFadeFromType(a0: u8, a1: u8) -> u8;
    fn GetMapPairFadeToType(a0: u8, a1: u8) -> u8;
    fn GetObjectEventIdByLocalIdAndMap(a0: u8, a1: u8, a2: u8) -> u8;
    fn GetPlayerFacingDirection() -> u8;
    fn GetWalkNormalMovementAction(a0: u32) -> u8;
    fn InstallCameraPanAheadCallback();
    fn IsLinkTaskFinished() -> u8;
    fn IsPlayerSpinEntranceActive() -> u32;
    fn IsPlayerSpinExitActive() -> u32;
    fn IsPlayerStandingStill() -> u8;
    fn IsWeatherNotFadingIn() -> u8;
    fn LoadPalette(a0: *mut c_void, a1: u16, a2: u16);
    fn LockPlayerFieldControls();
    fn MapGridGetMetatileBehaviorAt(a0: i32, a1: i32) -> i32;
    fn MetatileBehavior_IsDoor(a0: u8) -> u8;
    fn MetatileBehavior_IsNonAnimDoor(a0: u8) -> u8;
    fn ObjectEventClearHeldMovementIfActive(a0: *mut ObjectEvent);
    fn ObjectEventClearHeldMovementIfFinished(a0: *mut ObjectEvent) -> u8;
    fn ObjectEventSetHeldMovement(a0: *mut ObjectEvent, a1: u8) -> u8;
    fn OnTrainerHillEReaderChallengeFloor() -> u32;
    fn Overworld_FadeOutMapMusic();
    fn Overworld_PlaySpecialMapMusic();
    fn PlayRainStoppingSoundEffect();
    fn PlaySE(a0: u16);
    fn PlayerGetDestCoords(a0: *mut i16, a1: *mut i16);
    fn ResetAllMultiplayerState();
    fn RfuSetErrorParams(a0: u32);
    fn SaveObjectEvents();
    fn ScanlineEffect_Clear();
    fn ScanlineEffect_SetParams(a0: ScanlineEffectParams);
    fn ScanlineEffect_Stop();
    fn ScheduleBgCopyTilemapToVram(a0: u8);
    fn ScriptContext_Enable();
    fn ScriptUnfreezeObjectEvents();
    fn SetBgTilemapPalette(a0: u8, a1: u8, a2: u8, a3: u8, a4: u8, a5: u8);
    fn SetCameraPanning(a0: i16, a1: i16);
    fn SetCameraPanningCallback(a0: Option<unsafe extern "C" fn()>);
    fn SetCloseLinkCallback();
    fn SetGpuReg(a0: u8, a1: u16);
    fn SetGpuRegBits(a0: u8, a1: u16);
    fn SetLinkStandbyCallback();
    fn SetMainCallback2(a0: Option<unsafe extern "C" fn()>);
    fn SetObjectEventLoadFlag(a0: u8);
    fn SetPlayerInvisibility(a0: u8);
    fn ShowReturnToFieldStartMenu();
    fn StartEscalatorWarp(a0: u8, a1: u8);
    fn StartLavaridgeGym1FWarp(a0: u8);
    fn StartLavaridgeGymB1FWarp(a0: u8);
    fn StartSendingKeysToLink();
    fn Task_ShowStartMenu(a0: u8);
    fn TryFadeOutOldMapMusic();
    fn UnfreezeObjectEvents();
    fn UnlockPlayerFieldControls();
    fn WarpIntoMap();
}

pub(crate) unsafe extern "C" fn FillPalBufferWhite() {
    {
        let mut tmp: u32 = 0;
        volatile_write(&raw mut tmp, 0x7fff7fff);
        CpuFastSet(
            &raw mut tmp as *mut c_void,
            gPlttBufferFaded.as_mut_ptr() as *mut c_void,
            0x1000100,
        );
    }
}
pub(crate) unsafe extern "C" fn FillPalBufferBlack() {
    {
        let mut tmp: u32 = 0;
        volatile_write(&raw mut tmp, 0);
        CpuFastSet(
            &raw mut tmp as *mut c_void,
            gPlttBufferFaded.as_mut_ptr() as *mut c_void,
            0x1000100,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn WarpFadeInScreen() {
    let mut previousMapType: u8 = GetLastUsedWarpMapType();
    match GetMapPairFadeFromType(previousMapType, GetCurrentMapType()) {
        0 => {
            FillPalBufferBlack();
            FadeScreen(0, 0);
        }
        1 => {
            FillPalBufferWhite();
            FadeScreen(FADE_FROM_WHITE, 0);
        }
        _ => {}
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FadeInFromWhite() {
    FillPalBufferWhite();
    FadeScreen(FADE_FROM_WHITE, 8);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FadeInFromBlack() {
    FillPalBufferBlack();
    FadeScreen(0, 0);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn WarpFadeOutScreen() {
    let mut currentMapType: u8 = GetCurrentMapType();
    match GetMapPairFadeToType(currentMapType, (*GetDestinationWarpMapHeader()).mapType) {
        0 => {
            FadeScreen(FADE_TO_BLACK, 0);
        }
        1 => {
            FadeScreen(FADE_TO_WHITE, 0);
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn SetPlayerVisibility(visible: u8) {
    SetPlayerInvisibility((visible == 0) as u8);
}
pub(crate) unsafe extern "C" fn Task_WaitForUnionRoomFade(taskId: u8) {
    if WaitForWeatherFadeIn() == TRUE as u32 {
        DestroyTask(taskId);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FieldCB_ContinueScriptUnionRoom() {
    LockPlayerFieldControls();
    Overworld_PlaySpecialMapMusic();
    FadeInFromBlack();
    CreateTask(Some(Task_WaitForUnionRoomFade), 10);
}
pub(crate) unsafe extern "C" fn Task_WaitForFadeAndEnableScriptCtx(taskID: u8) {
    if WaitForWeatherFadeIn() == TRUE as u32 {
        DestroyTask(taskID);
        ScriptContext_Enable();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FieldCB_ContinueScriptHandleMusic() {
    LockPlayerFieldControls();
    Overworld_PlaySpecialMapMusic();
    FadeInFromBlack();
    CreateTask(Some(Task_WaitForFadeAndEnableScriptCtx), 10);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FieldCB_ContinueScript() {
    LockPlayerFieldControls();
    FadeInFromBlack();
    CreateTask(Some(Task_WaitForFadeAndEnableScriptCtx), 10);
}
pub(crate) unsafe extern "C" fn Task_ReturnToFieldCableLink(taskId: u8) {
    let mut task: *mut Task = &raw mut gTasks[taskId];
    match (*task).data[0] {
        0 => {
            (*task).data[1] = CreateTask_ReestablishCableClubLink() as i16;
            (*task).data[0] += 1;
        }
        1 => {
            if gTasks[(*task).data[1]].isActive != 1 {
                WarpFadeInScreen();
                (*task).data[0] += 1;
            }
        }
        2 => {
            if WaitForWeatherFadeIn() == TRUE as u32 {
                UnlockPlayerFieldControls();
                DestroyTask(taskId);
            }
        }
        _ => {}
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FieldCB_ReturnToFieldCableLink() {
    LockPlayerFieldControls();
    Overworld_PlaySpecialMapMusic();
    FillPalBufferBlack();
    CreateTask(Some(Task_ReturnToFieldCableLink), 10);
}
pub(crate) unsafe extern "C" fn Task_ReturnToFieldWirelessLink(taskId: u8) {
    let mut task: *mut Task = &raw mut gTasks[taskId];
    match (*task).data[0] {
        0 => {
            SetLinkStandbyCallback();
            (*task).data[0] += 1;
        }
        1 => {
            if IsLinkTaskFinished() == 0 {
                if ({
                    (*task).data[1] += 1;
                    (*task).data[1]
                }) > 1800
                {
                    RfuSetErrorParams(24576);
                }
            } else {
                WarpFadeInScreen();
                (*task).data[0] += 1;
            }
        }
        2 => {
            if WaitForWeatherFadeIn() == TRUE as u32 {
                StartSendingKeysToLink();
                UnlockPlayerFieldControls();
                DestroyTask(taskId);
            }
        }
        _ => {}
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Task_ReturnToFieldRecordMixing(taskId: u8) {
    let mut task: *mut Task = &raw mut gTasks[taskId];
    match (*task).data[0] {
        0 => {
            SetLinkStandbyCallback();
            (*task).data[0] += 1;
        }
        1 => {
            if IsLinkTaskFinished() != 0 {
                (*task).data[0] += 1;
            }
        }
        2 => {
            StartSendingKeysToLink();
            ResetAllMultiplayerState();
            UnlockPlayerFieldControls();
            DestroyTask(taskId);
        }
        _ => {}
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FieldCB_ReturnToFieldWirelessLink() {
    LockPlayerFieldControls();
    Overworld_PlaySpecialMapMusic();
    FillPalBufferBlack();
    CreateTask(Some(Task_ReturnToFieldWirelessLink), 10);
}
pub(crate) unsafe extern "C" fn SetUpWarpExitTask() {
    let mut x: i16 = 0;
    let mut y: i16 = 0;
    let mut behavior: u8 = 0;
    let mut func: Option<unsafe extern "C" fn(u8)> = None;
    PlayerGetDestCoords(&raw mut x, &raw mut y);
    behavior = MapGridGetMetatileBehaviorAt(x as i32, y as i32) as u8;
    if MetatileBehavior_IsDoor(behavior) == TRUE {
        func = Some(Task_ExitDoor);
    } else if MetatileBehavior_IsNonAnimDoor(behavior) == TRUE {
        func = Some(Task_ExitNonAnimDoor);
    } else {
        func = Some(Task_ExitNonDoor);
    }
    CreateTask(func, 10);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FieldCB_DefaultWarpExit() {
    Overworld_PlaySpecialMapMusic();
    WarpFadeInScreen();
    SetUpWarpExitTask();
    LockPlayerFieldControls();
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FieldCB_WarpExitFadeFromWhite() {
    Overworld_PlaySpecialMapMusic();
    FadeInFromWhite();
    SetUpWarpExitTask();
    LockPlayerFieldControls();
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FieldCB_WarpExitFadeFromBlack() {
    if OnTrainerHillEReaderChallengeFloor() == 0 {
        Overworld_PlaySpecialMapMusic();
    }
    FadeInFromBlack();
    SetUpWarpExitTask();
    LockPlayerFieldControls();
}
pub(crate) unsafe extern "C" fn FieldCB_SpinEnterWarp() {
    Overworld_PlaySpecialMapMusic();
    WarpFadeInScreen();
    PlaySE(SE_WARP_OUT);
    CreateTask(Some(Task_SpinEnterWarp), 10);
    LockPlayerFieldControls();
}
pub(crate) unsafe extern "C" fn FieldCB_MossdeepGymWarpExit() {
    Overworld_PlaySpecialMapMusic();
    WarpFadeInScreen();
    PlaySE(SE_WARP_OUT);
    CreateTask(Some(Task_ExitNonDoor), 10);
    LockPlayerFieldControls();
    SetObjectEventLoadFlag(14);
}
pub(crate) unsafe extern "C" fn Task_ExitDoor(taskId: u8) {
    let mut task: *mut Task = &raw mut gTasks[taskId];
    let mut x: *mut i16 = &raw mut (*task).data[2];
    let mut y: *mut i16 = &raw mut (*task).data[3];
    match (*task).data[0] {
        0 => {
            SetPlayerVisibility(FALSE);
            FreezeObjectEvents();
            PlayerGetDestCoords(x, y);
            FieldSetDoorOpened(*x as u32, *y as u32);
            (*task).data[0] = 1;
        }
        1 => {
            if WaitForWeatherFadeIn() != 0 {
                let mut objEventId: u8 = 0;
                SetPlayerVisibility(TRUE);
                objEventId = GetObjectEventIdByLocalIdAndMap(LOCALID_PLAYER, 0, 0);
                ObjectEventSetHeldMovement(
                    &raw mut gObjectEvents[objEventId],
                    MOVEMENT_ACTION_WALK_NORMAL_DOWN,
                );
                (*task).data[0] = 2;
            }
        }
        2 => {
            if IsPlayerStandingStill() != 0 {
                let mut objEventId: u8 = 0;
                (*task).data[1] = FieldAnimateDoorClose(*x as u32, *y as u32) as i16;
                objEventId = GetObjectEventIdByLocalIdAndMap(LOCALID_PLAYER, 0, 0);
                ObjectEventClearHeldMovementIfFinished(&raw mut gObjectEvents[objEventId]);
                (*task).data[0] = 3;
            }
        }
        3 => {
            if (*task).data[1] < 0 || gTasks[(*task).data[1]].isActive != 1 {
                UnfreezeObjectEvents();
                (*task).data[0] = 4;
            }
        }
        4 => {
            UnlockPlayerFieldControls();
            DestroyTask(taskId);
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn Task_ExitNonAnimDoor(taskId: u8) {
    let mut task: *mut Task = &raw mut gTasks[taskId];
    let mut x: *mut i16 = &raw mut (*task).data[2];
    let mut y: *mut i16 = &raw mut (*task).data[3];
    match (*task).data[0] {
        0 => {
            SetPlayerVisibility(FALSE);
            FreezeObjectEvents();
            PlayerGetDestCoords(x, y);
            (*task).data[0] = 1;
        }
        1 => {
            if WaitForWeatherFadeIn() != 0 {
                let mut objEventId: u8 = 0;
                SetPlayerVisibility(TRUE);
                objEventId = GetObjectEventIdByLocalIdAndMap(LOCALID_PLAYER, 0, 0);
                ObjectEventSetHeldMovement(
                    &raw mut gObjectEvents[objEventId],
                    GetWalkNormalMovementAction(GetPlayerFacingDirection() as u32),
                );
                (*task).data[0] = 2;
            }
        }
        2 => {
            if IsPlayerStandingStill() != 0 {
                UnfreezeObjectEvents();
                (*task).data[0] = 3;
            }
        }
        3 => {
            UnlockPlayerFieldControls();
            DestroyTask(taskId);
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn Task_ExitNonDoor(taskId: u8) {
    match gTasks[taskId].data[0] {
        0 => {
            FreezeObjectEvents();
            LockPlayerFieldControls();
            gTasks[taskId].data[0] += 1;
        }
        1 => {
            if WaitForWeatherFadeIn() != 0 {
                UnfreezeObjectEvents();
                UnlockPlayerFieldControls();
                DestroyTask(taskId);
            }
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn Task_WaitForFadeShowStartMenu(taskId: u8) {
    if WaitForWeatherFadeIn() == TRUE as u32 {
        DestroyTask(taskId);
        CreateTask(Some(Task_ShowStartMenu), 80);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ReturnToFieldOpenStartMenu() {
    FadeInFromBlack();
    CreateTask(Some(Task_WaitForFadeShowStartMenu), 0x50);
    LockPlayerFieldControls();
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FieldCB_ReturnToFieldOpenStartMenu() -> u8 {
    ShowReturnToFieldStartMenu();
    return FALSE;
}
pub(crate) unsafe extern "C" fn Task_ReturnToFieldNoScript(taskId: u8) {
    if WaitForWeatherFadeIn() == 1 {
        UnlockPlayerFieldControls();
        DestroyTask(taskId);
        ScriptUnfreezeObjectEvents();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FieldCB_ReturnToFieldNoScript() {
    LockPlayerFieldControls();
    FadeInFromBlack();
    CreateTask(Some(Task_ReturnToFieldNoScript), 10);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FieldCB_ReturnToFieldNoScriptCheckMusic() {
    LockPlayerFieldControls();
    Overworld_PlaySpecialMapMusic();
    FadeInFromBlack();
    CreateTask(Some(Task_ReturnToFieldNoScript), 10);
}
pub(crate) unsafe extern "C" fn PaletteFadeActive() -> u32 {
    return gPaletteFade.active() as u32;
}
pub(crate) unsafe extern "C" fn WaitForWeatherFadeIn() -> u32 {
    if IsWeatherNotFadingIn() == TRUE {
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
pub unsafe extern "C" fn DoWarp() {
    LockPlayerFieldControls();
    TryFadeOutOldMapMusic();
    WarpFadeOutScreen();
    PlayRainStoppingSoundEffect();
    PlaySE(SE_EXIT);
    gFieldCallback = Some(FieldCB_DefaultWarpExit);
    CreateTask(Some(Task_WarpAndLoadMap), 10);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DoDiveWarp() {
    LockPlayerFieldControls();
    TryFadeOutOldMapMusic();
    WarpFadeOutScreen();
    PlayRainStoppingSoundEffect();
    gFieldCallback = Some(FieldCB_DefaultWarpExit);
    CreateTask(Some(Task_WarpAndLoadMap), 10);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DoWhiteFadeWarp() {
    LockPlayerFieldControls();
    TryFadeOutOldMapMusic();
    FadeScreen(FADE_TO_WHITE, 8);
    PlayRainStoppingSoundEffect();
    gFieldCallback = Some(FieldCB_WarpExitFadeFromWhite);
    CreateTask(Some(Task_WarpAndLoadMap), 10);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DoDoorWarp() {
    LockPlayerFieldControls();
    gFieldCallback = Some(FieldCB_DefaultWarpExit);
    CreateTask(Some(Task_DoDoorWarp), 10);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DoFallWarp() {
    DoDiveWarp();
    gFieldCallback = Some(FieldCB_FallWarpExit);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DoEscalatorWarp(metatileBehavior: u8) {
    LockPlayerFieldControls();
    StartEscalatorWarp(metatileBehavior, 10);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DoLavaridgeGymB1FWarp() {
    LockPlayerFieldControls();
    StartLavaridgeGymB1FWarp(10);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DoLavaridgeGym1FWarp() {
    LockPlayerFieldControls();
    StartLavaridgeGym1FWarp(10);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DoTeleportTileWarp() {
    LockPlayerFieldControls();
    TryFadeOutOldMapMusic();
    WarpFadeOutScreen();
    PlaySE(SE_WARP_IN);
    CreateTask(Some(Task_WarpAndLoadMap), 10);
    gFieldCallback = Some(FieldCB_SpinEnterWarp);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DoMossdeepGymWarp() {
    SetObjectEventLoadFlag(SKIP_OBJECT_EVENT_LOAD);
    LockPlayerFieldControls();
    SaveObjectEvents();
    TryFadeOutOldMapMusic();
    WarpFadeOutScreen();
    PlaySE(SE_WARP_IN);
    CreateTask(Some(Task_WarpAndLoadMap), 10);
    gFieldCallback = Some(FieldCB_MossdeepGymWarpExit);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DoPortholeWarp() {
    LockPlayerFieldControls();
    WarpFadeOutScreen();
    CreateTask(Some(Task_WarpAndLoadMap), 10);
    gFieldCallback = Some(FieldCB_ShowPortholeView);
}
pub(crate) unsafe extern "C" fn Task_DoCableClubWarp(taskId: u8) {
    let mut task: *mut Task = &raw mut gTasks[taskId];
    match (*task).data[0] {
        0 => {
            LockPlayerFieldControls();
            (*task).data[0] += 1;
        }
        1 => {
            if PaletteFadeActive() == 0 && BGMusicStopped() != 0 {
                (*task).data[0] += 1;
            }
        }
        2 => {
            WarpIntoMap();
            SetMainCallback2(Some(CB2_ReturnToFieldCableClub));
            DestroyTask(taskId);
        }
        _ => {}
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DoCableClubWarp() {
    LockPlayerFieldControls();
    TryFadeOutOldMapMusic();
    WarpFadeOutScreen();
    PlaySE(SE_EXIT);
    CreateTask(Some(Task_DoCableClubWarp), 10);
}
pub(crate) unsafe extern "C" fn Task_ReturnToWorldFromLinkRoom(taskId: u8) {
    let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
    match *data {
        0 => {
            ClearLinkCallback_2();
            FadeScreen(FADE_TO_BLACK, 0);
            TryFadeOutOldMapMusic();
            PlaySE(SE_EXIT);
            *data += 1;
        }
        1 => {
            if PaletteFadeActive() == 0 && BGMusicStopped() != 0 {
                SetCloseLinkCallback();
                *data += 1;
            }
        }
        2 => {
            if gReceivedRemoteLinkPlayers == 0 {
                WarpIntoMap();
                SetMainCallback2(Some(CB2_LoadMap));
                DestroyTask(taskId);
            }
        }
        _ => {}
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ReturnFromLinkRoom() {
    CreateTask(Some(Task_ReturnToWorldFromLinkRoom), 10);
}
pub(crate) unsafe extern "C" fn Task_WarpAndLoadMap(taskId: u8) {
    let mut task: *mut Task = &raw mut gTasks[taskId];
    match (*task).data[0] {
        0 => {
            FreezeObjectEvents();
            LockPlayerFieldControls();
            (*task).data[0] += 1;
        }
        1 => {
            if PaletteFadeActive() == 0 {
                if (*task).data[1] == 0 {
                    ClearMirageTowerPulseBlendEffect();
                    (*task).data[1] = 1;
                }
                if BGMusicStopped() != 0 {
                    (*task).data[0] += 1;
                }
            }
        }
        2 => {
            WarpIntoMap();
            SetMainCallback2(Some(CB2_LoadMap));
            DestroyTask(taskId);
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn Task_DoDoorWarp(taskId: u8) {
    let mut task: *mut Task = &raw mut gTasks[taskId];
    let mut x: *mut i16 = &raw mut (*task).data[2];
    let mut y: *mut i16 = &raw mut (*task).data[3];
    match (*task).data[0] {
        0 => {
            FreezeObjectEvents();
            PlayerGetDestCoords(x, y);
            PlaySE(GetDoorSoundEffect(*x as u32, *y as u32 - 1) as u16);
            (*task).data[1] = FieldAnimateDoorOpen(*x as u32, *y as u32 - 1) as i16;
            (*task).data[0] = 1;
        }
        1 => {
            if (*task).data[1] < 0 || gTasks[(*task).data[1]].isActive != 1 {
                let mut objEventId: u8 = 0;
                objEventId = GetObjectEventIdByLocalIdAndMap(LOCALID_PLAYER, 0, 0);
                ObjectEventClearHeldMovementIfActive(&raw mut gObjectEvents[objEventId]);
                objEventId = GetObjectEventIdByLocalIdAndMap(LOCALID_PLAYER, 0, 0);
                ObjectEventSetHeldMovement(
                    &raw mut gObjectEvents[objEventId],
                    MOVEMENT_ACTION_WALK_NORMAL_UP,
                );
                (*task).data[0] = 2;
            }
        }
        2 => {
            if IsPlayerStandingStill() != 0 {
                let mut objEventId: u8 = 0;
                (*task).data[1] = FieldAnimateDoorClose(*x as u32, *y as u32 - 1) as i16;
                objEventId = GetObjectEventIdByLocalIdAndMap(LOCALID_PLAYER, 0, 0);
                ObjectEventClearHeldMovementIfFinished(&raw mut gObjectEvents[objEventId]);
                SetPlayerVisibility(FALSE);
                (*task).data[0] = 3;
            }
        }
        3 => {
            if (*task).data[1] < 0 || gTasks[(*task).data[1]].isActive != 1 {
                (*task).data[0] = 4;
            }
        }
        4 => {
            TryFadeOutOldMapMusic();
            WarpFadeOutScreen();
            PlayRainStoppingSoundEffect();
            (*task).data[0] = 0;
            (*task).func = Some(Task_WarpAndLoadMap);
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn Task_DoContestHallWarp(taskId: u8) {
    let mut task: *mut Task = &raw mut gTasks[taskId];
    match (*task).data[0] {
        0 => {
            FreezeObjectEvents();
            LockPlayerFieldControls();
            (*task).data[0] += 1;
        }
        1 => {
            if PaletteFadeActive() == 0 && BGMusicStopped() != 0 {
                (*task).data[0] += 1;
            }
        }
        2 => {
            WarpIntoMap();
            SetMainCallback2(Some(CB2_ReturnToFieldContestHall));
            DestroyTask(taskId);
        }
        _ => {}
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DoContestHallWarp() {
    LockPlayerFieldControls();
    TryFadeOutOldMapMusic();
    WarpFadeOutScreen();
    PlayRainStoppingSoundEffect();
    PlaySE(SE_EXIT);
    gFieldCallback = Some(FieldCB_WarpExitFadeFromBlack);
    CreateTask(Some(Task_DoContestHallWarp), 10);
}
pub(crate) unsafe extern "C" fn SetFlashScanlineEffectWindowBoundary(
    mut dest: *mut u16,
    y: u32,
    mut left: i32,
    mut right: i32,
) {
    if y <= 160 {
        if left < 0 {
            left = 0;
        }
        if left > 255 {
            left = 255;
        }
        if right < 0 {
            right = 0;
        }
        if right > 255 {
            right = 255;
        }
        *dest.at(y) = (left as u16) << 8 | right as u16;
    }
}
pub(crate) unsafe extern "C" fn SetFlashScanlineEffectWindowBoundaries(
    dest: *mut u16,
    centerX: i32,
    centerY: i32,
    radius: i32,
) {
    let mut r: i32 = radius;
    let mut v2: i32 = radius;
    let mut v3: i32 = 0;
    while r >= v3 {
        SetFlashScanlineEffectWindowBoundary(
            dest,
            centerY as u32 - v3 as u32,
            centerX - r,
            centerX + r,
        );
        SetFlashScanlineEffectWindowBoundary(
            dest,
            centerY as u32 + v3 as u32,
            centerX - r,
            centerX + r,
        );
        SetFlashScanlineEffectWindowBoundary(
            dest,
            centerY as u32 - r as u32,
            centerX - v3,
            centerX + v3,
        );
        SetFlashScanlineEffectWindowBoundary(
            dest,
            centerY as u32 + r as u32,
            centerX - v3,
            centerX + v3,
        );
        v2 -= v3 * 2 - 1;
        v3 += 1;
        if v2 < 0 {
            v2 += 2 * (r - 1);
            r -= 1;
        }
    }
}
pub(crate) unsafe extern "C" fn SetOrbFlashScanlineEffectWindowBoundary(
    mut dest: *mut u16,
    y: u32,
    mut left: i32,
    mut right: i32,
) {
    if y <= 160 {
        if left < 0 {
            left = 0;
        }
        if left > 240 {
            left = 240;
        }
        if right < 0 {
            right = 0;
        }
        if right > 240 {
            right = 240;
        }
        *dest.at(y) = (left as u16) << 8 | right as u16;
    }
}
pub(crate) unsafe extern "C" fn SetOrbFlashScanlineEffectWindowBoundaries(
    dest: *mut u16,
    centerX: i32,
    centerY: i32,
    radius: i32,
) {
    let mut r: i32 = radius;
    let mut v2: i32 = radius;
    let mut v3: i32 = 0;
    while r >= v3 {
        SetOrbFlashScanlineEffectWindowBoundary(
            dest,
            centerY as u32 - v3 as u32,
            centerX - r,
            centerX + r,
        );
        SetOrbFlashScanlineEffectWindowBoundary(
            dest,
            centerY as u32 + v3 as u32,
            centerX - r,
            centerX + r,
        );
        SetOrbFlashScanlineEffectWindowBoundary(
            dest,
            centerY as u32 - r as u32,
            centerX - v3,
            centerX + v3,
        );
        SetOrbFlashScanlineEffectWindowBoundary(
            dest,
            centerY as u32 + r as u32,
            centerX - v3,
            centerX + v3,
        );
        v2 -= v3 * 2 - 1;
        v3 += 1;
        if v2 < 0 {
            v2 += 2 * (r - 1);
            r -= 1;
        }
    }
}
pub(crate) unsafe extern "C" fn UpdateFlashLevelEffect(taskId: u8) {
    let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
    match *data {
        0 => {
            SetFlashScanlineEffectWindowBoundaries(
                gScanlineEffectRegBuffers[gScanlineEffect.srcBuffer].as_mut_ptr(),
                *data.at(1) as i32,
                *data.at(2) as i32,
                *data.at(3) as i32,
            );
            *data = 1;
        }
        1 => {
            SetFlashScanlineEffectWindowBoundaries(
                gScanlineEffectRegBuffers[gScanlineEffect.srcBuffer].as_mut_ptr(),
                *data.at(1) as i32,
                *data.at(2) as i32,
                *data.at(3) as i32,
            );
            *data = 0;
            *data.at(3) += *data.at(5);
            if *data.at(3) > *data.at(4) {
                if *data.at(6) == 1 {
                    ScanlineEffect_Stop();
                    *data = 2;
                } else {
                    DestroyTask(taskId);
                }
            }
        }
        2 => {
            ScanlineEffect_Clear();
            DestroyTask(taskId);
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn UpdateOrbFlashEffect(taskId: u8) {
    let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
    match *data {
        0 => {
            SetOrbFlashScanlineEffectWindowBoundaries(
                gScanlineEffectRegBuffers[gScanlineEffect.srcBuffer].as_mut_ptr(),
                *data.at(1) as i32,
                *data.at(2) as i32,
                *data.at(3) as i32,
            );
            *data = 1;
        }
        1 => {
            SetOrbFlashScanlineEffectWindowBoundaries(
                gScanlineEffectRegBuffers[gScanlineEffect.srcBuffer].as_mut_ptr(),
                *data.at(1) as i32,
                *data.at(2) as i32,
                *data.at(3) as i32,
            );
            *data = 0;
            *data.at(3) += *data.at(5);
            if *data.at(3) > *data.at(4) {
                if *data.at(6) == 1 {
                    ScanlineEffect_Stop();
                    *data = 2;
                } else {
                    DestroyTask(taskId);
                }
            }
        }
        2 => {
            ScanlineEffect_Clear();
            DestroyTask(taskId);
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn Task_WaitForFlashUpdate(taskId: u8) {
    if FuncIsActiveTask(Some(UpdateFlashLevelEffect)) == 0 {
        ScriptContext_Enable();
        DestroyTask(taskId);
    }
}
pub(crate) unsafe extern "C" fn StartWaitForFlashUpdate() {
    if FuncIsActiveTask(Some(Task_WaitForFlashUpdate)) == 0 {
        CreateTask(Some(Task_WaitForFlashUpdate), 80);
    }
}
pub(crate) unsafe extern "C" fn StartUpdateFlashLevelEffect(
    centerX: i32,
    centerY: i32,
    initialFlashRadius: i32,
    destFlashRadius: i32,
    clearScanlineEffect: i32,
    delta: u8,
) -> u8 {
    let mut taskId: u8 = CreateTask(Some(UpdateFlashLevelEffect), 80);
    let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
    *data.at(3) = initialFlashRadius as i16;
    *data.at(4) = destFlashRadius as i16;
    *data.at(1) = centerX as i16;
    *data.at(2) = centerY as i16;
    *data.at(6) = clearScanlineEffect as i16;
    if initialFlashRadius < destFlashRadius {
        *data.at(5) = delta as i16;
    } else {
        *data.at(5) = -(delta as i16);
    }
    return taskId;
}
pub(crate) unsafe extern "C" fn StartUpdateOrbFlashEffect(
    centerX: i32,
    centerY: i32,
    initialFlashRadius: i32,
    destFlashRadius: i32,
    clearScanlineEffect: i32,
    delta: u8,
) -> u8 {
    let mut taskId: u8 = CreateTask(Some(UpdateOrbFlashEffect), 80);
    let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
    *data.at(3) = initialFlashRadius as i16;
    *data.at(4) = destFlashRadius as i16;
    *data.at(1) = centerX as i16;
    *data.at(2) = centerY as i16;
    *data.at(6) = clearScanlineEffect as i16;
    if initialFlashRadius < destFlashRadius {
        *data.at(5) = delta as i16;
    } else {
        *data.at(5) = -(delta as i16);
    }
    return taskId;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimateFlash(newFlashLevel: u8) {
    let mut curFlashLevel: u8 = GetFlashLevel();
    let mut fullBrightness: u8 = FALSE;
    if newFlashLevel == 0 {
        fullBrightness = TRUE;
    }
    StartUpdateFlashLevelEffect(
        120,
        80,
        sFlashLevelToRadius[curFlashLevel] as i32,
        sFlashLevelToRadius[newFlashLevel] as i32,
        fullBrightness as i32,
        1,
    );
    StartWaitForFlashUpdate();
    LockPlayerFieldControls();
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn WriteFlashScanlineEffectBuffer(flashLevel: u8) {
    if flashLevel != 0 {
        SetFlashScanlineEffectWindowBoundaries(
            &raw mut gScanlineEffectRegBuffers[0][0],
            120,
            80,
            sFlashLevelToRadius[flashLevel] as i32,
        );
        CpuFastSet(
            &raw mut gScanlineEffectRegBuffers[0] as *mut c_void,
            &raw mut gScanlineEffectRegBuffers[1] as *mut c_void,
            480,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn WriteBattlePyramidViewScanlineEffectBuffer() {
    SetFlashScanlineEffectWindowBoundaries(
        &raw mut gScanlineEffectRegBuffers[0][0],
        120,
        80,
        (*gSaveBlock2Ptr).frontier.pyramidLightRadius as i32,
    );
    CpuFastSet(
        &raw mut gScanlineEffectRegBuffers[0] as *mut c_void,
        &raw mut gScanlineEffectRegBuffers[1] as *mut c_void,
        480,
    );
}
pub(crate) unsafe extern "C" fn Task_SpinEnterWarp(taskId: u8) {
    match gTasks[taskId].data[0] {
        0 => {
            FreezeObjectEvents();
            LockPlayerFieldControls();
            DoPlayerSpinEntrance();
            gTasks[taskId].data[0] += 1;
        }
        1 => {
            if WaitForWeatherFadeIn() != 0 && IsPlayerSpinEntranceActive() != TRUE as u32 {
                UnfreezeObjectEvents();
                UnlockPlayerFieldControls();
                DestroyTask(taskId);
            }
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn Task_SpinExitWarp(taskId: u8) {
    let mut task: *mut Task = &raw mut gTasks[taskId];
    match (*task).data[0] {
        0 => {
            FreezeObjectEvents();
            LockPlayerFieldControls();
            PlaySE(SE_WARP_IN);
            DoPlayerSpinExit();
            (*task).data[0] += 1;
        }
        1 => {
            if IsPlayerSpinExitActive() == 0 {
                WarpFadeOutScreen();
                (*task).data[0] += 1;
            }
        }
        2 => {
            if PaletteFadeActive() == 0 && BGMusicStopped() != 0 {
                (*task).data[0] += 1;
            }
        }
        3 => {
            WarpIntoMap();
            SetMainCallback2(Some(CB2_LoadMap));
            DestroyTask(taskId);
        }
        _ => {}
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DoSpinEnterWarp() {
    LockPlayerFieldControls();
    CreateTask(Some(Task_WarpAndLoadMap), 10);
    gFieldCallback = Some(FieldCB_SpinEnterWarp);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DoSpinExitWarp() {
    LockPlayerFieldControls();
    gFieldCallback = Some(FieldCB_DefaultWarpExit);
    CreateTask(Some(Task_SpinExitWarp), 10);
}
pub(crate) unsafe extern "C" fn LoadOrbEffectPalette(blueOrb: u8) {
    let mut i: i32 = 0;
    let mut color: CArray<u16, 1> = zeroed();
    if blueOrb == 0 {
        color[0] = 31;
    } else {
        color[0] = 31744;
    }
    i = 0;
    while i < 16 {
        LoadPalette(color.as_mut_ptr() as *mut c_void, 240 + i as u16, 2);
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn UpdateOrbEffectBlend(shakeDir: u16) -> u8 {
    let mut lo: u8 = (67108946 as usize as *mut u16).read_volatile() as u8 & 0xFF;
    let mut hi: u8 = ((67108946 as usize as *mut u16).read_volatile() >> 8) as u8;
    if shakeDir != 0 {
        if lo != 0 {
            lo -= 1;
        }
    } else {
        if hi < 16 {
            hi += 1;
        }
    }
    SetGpuReg(REG_OFFSET_BLDALPHA, (hi as u16) << 8 | lo as u16);
    if lo == 0 && hi == 16 {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn Task_OrbEffect(taskId: u8) {
    let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
    match *data {
        0 => {
            *data.at(6) = (0x4000000 as usize as *mut u16).read_volatile() as i16;
            *data.at(7) = (67108944 as usize as *mut u16).read_volatile() as i16;
            *data.at(8) = (67108946 as usize as *mut u16).read_volatile() as i16;
            *data.at(9) = (67108936 as usize as *mut u16).read_volatile() as i16;
            *data.at(10) = (67108938 as usize as *mut u16).read_volatile() as i16;
            ClearGpuRegBits(REG_OFFSET_DISPCNT, DISPCNT_WIN1_ON);
            SetGpuRegBits(REG_OFFSET_BLDCNT, gOrbEffectBackgroundLayerFlags[0]);
            SetGpuReg(REG_OFFSET_BLDALPHA, 1804);
            SetGpuReg(REG_OFFSET_WININ, 63);
            SetGpuReg(REG_OFFSET_WINOUT, 30);
            SetBgTilemapPalette(0, 0, 0, DISPLAY_TILE_WIDTH, DISPLAY_TILE_HEIGHT, 0xF);
            ScheduleBgCopyTilemapToVram(0);
            SetOrbFlashScanlineEffectWindowBoundaries(
                &raw mut gScanlineEffectRegBuffers[0][0],
                *data.at(2) as i32,
                *data.at(3) as i32,
                1,
            );
            CpuFastSet(
                &raw mut gScanlineEffectRegBuffers[0] as *mut c_void,
                &raw mut gScanlineEffectRegBuffers[1] as *mut c_void,
                480,
            );
            ScanlineEffect_SetParams(*sFlashEffectParams);
            *data = 1;
        }
        1 => {
            BgDmaFill(0, 17, 0, 1);
            LoadOrbEffectPalette(*data.at(1) as u8);
            StartUpdateOrbFlashEffect(*data.at(2) as i32, *data.at(3) as i32, 1, 160, 1, 2);
            *data = 2;
        }
        2 => {
            if FuncIsActiveTask(Some(UpdateOrbFlashEffect)) == 0 {
                ScriptContext_Enable();
                *data = 3;
            }
        }
        3 => {
            InstallCameraPanAheadCallback();
            SetCameraPanningCallback(None);
            *data.at(5) = 0;
            *data.at(4) = 4;
            *data = 4;
        }
        4 => {
            if ({
                *data.at(4) -= 1;
                *data.at(4)
            }) == 0
            {
                let mut panning: i32 = 0;
                *data.at(4) = 4;
                *data.at(5) ^= 1;
                if *data.at(5) != 0 {
                    panning = 4;
                } else {
                    panning = -4;
                }
                SetCameraPanning(0, panning as i16);
            }
        }
        6 => {
            InstallCameraPanAheadCallback();
            *data.at(4) = 8;
            *data = 7;
        }
        7 => {
            if ({
                *data.at(4) -= 1;
                *data.at(4)
            }) == 0
            {
                *data.at(4) = 8;
                *data.at(5) ^= 1;
                if UpdateOrbEffectBlend(*data.at(5) as u16) == TRUE {
                    *data = 5;
                    BgDmaFill(0, 0, 0, 1);
                }
            }
        }
        5 => {
            SetGpuReg(REG_OFFSET_WIN0H, 255);
            SetGpuReg(REG_OFFSET_DISPCNT, *data.at(6) as u16);
            SetGpuReg(REG_OFFSET_BLDCNT, *data.at(7) as u16);
            SetGpuReg(REG_OFFSET_BLDALPHA, *data.at(8) as u16);
            SetGpuReg(REG_OFFSET_WININ, *data.at(9) as u16);
            SetGpuReg(REG_OFFSET_WINOUT, *data.at(10) as u16);
            ScriptContext_Enable();
            DestroyTask(taskId);
        }
        _ => {}
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DoOrbEffect() {
    let mut taskId: u8 = CreateTask(Some(Task_OrbEffect), 80);
    let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
    if gSpecialVar_Result == 0 {
        *data.at(1) = FALSE as i16;
        *data.at(2) = 104;
    } else if gSpecialVar_Result == 1 {
        *data.at(1) = TRUE as i16;
        *data.at(2) = 136;
    } else if gSpecialVar_Result == 2 {
        *data.at(1) = FALSE as i16;
        *data.at(2) = 120;
    } else {
        *data.at(1) = TRUE as i16;
        *data.at(2) = 120;
    }
    *data.at(3) = 80;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FadeOutOrbEffect() {
    let mut taskId: u8 = FindTaskIdByFunc(Some(Task_OrbEffect));
    gTasks[taskId].data[0] = 6;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Script_FadeOutMapMusic() {
    Overworld_FadeOutMapMusic();
    CreateTask(Some(Task_EnableScriptAfterMusicFade), 80);
}
pub(crate) unsafe extern "C" fn Task_EnableScriptAfterMusicFade(taskId: u8) {
    if BGMusicStopped() == TRUE {
        DestroyTask(taskId);
        ScriptContext_Enable();
    }
}
