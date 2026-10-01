//! Translated from `src/field_screen_effect.c` by tools/rustport/c2rs.py.
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
    clippy::manual_clamp,
    clippy::missing_transmute_annotations,
    clippy::useless_transmute,
    unused_assignments
)]

#[allow(unused_imports)]
use crate::c::*;
use crate::cable_club::CreateTask_ReestablishCableClubLink;
#[allow(unused_imports)]
use crate::consts::*;
use crate::event_object_lock::{IsPlayerStandingStill, ScriptUnfreezeObjectEvents};
use crate::event_object_movement::{
    FreezeObjectEvents, GetObjectEventIdByLocalIdAndMap, GetWalkNormalMovementAction,
    ObjectEventClearHeldMovementIfActive, ObjectEventClearHeldMovementIfFinished,
    ObjectEventSetHeldMovement, UnfreezeObjectEvents,
};
use crate::ffi::gSpecialVar_Result;
use crate::field_camera::{
    InstallCameraPanAheadCallback, SetCameraPanning, SetCameraPanningCallback,
};
use crate::field_door::{
    FieldAnimateDoorClose, FieldAnimateDoorOpen, FieldSetDoorOpened, GetDoorSoundEffect,
};
use crate::field_effect::{
    FieldCB_FallWarpExit, StartEscalatorWarp, StartLavaridgeGym1FWarp, StartLavaridgeGymB1FWarp,
};
use crate::field_player_avatar::{
    DoPlayerSpinEntrance, DoPlayerSpinExit, GetPlayerFacingDirection, IsPlayerSpinEntranceActive,
    IsPlayerSpinExitActive, PlayerGetDestCoords, SetPlayerInvisibility, gObjectEvents,
};
use crate::field_special_scene::FieldCB_ShowPortholeView;
use crate::field_weather::{FadeScreen, IsWeatherNotFadingIn, PlayRainStoppingSoundEffect};
use crate::fieldmap::MapGridGetMetatileBehaviorAt;
use crate::fldeff_flash::{GetMapPairFadeFromType, GetMapPairFadeToType};
use crate::gpu_regs::{ClearGpuRegBits, SetGpuReg, SetGpuRegBits};
use crate::link::{
    ClearLinkCallback_2, IsLinkTaskFinished, SetCloseLinkCallback, SetLinkStandbyCallback,
    StartSendingKeysToLink, gReceivedRemoteLinkPlayers,
};
use crate::link_rfu_2::RfuSetErrorParams;
use crate::load_save::SaveObjectEvents;
use crate::load_save::gSaveBlock2Ptr;
use crate::menu::{BgDmaFill, ScheduleBgCopyTilemapToVram, SetBgTilemapPalette};
use crate::metatile_behavior::{MetatileBehavior_IsDoor, MetatileBehavior_IsNonAnimDoor};
use crate::mirage_tower::ClearMirageTowerPulseBlendEffect;
use crate::overworld::{
    BGMusicStopped, CB2_LoadMap, CB2_ReturnToFieldCableClub, CB2_ReturnToFieldContestHall,
    GetCurrentMapType, GetDestinationWarpMapHeader, GetFlashLevel, GetLastUsedWarpMapType,
    Overworld_FadeOutMapMusic, Overworld_PlaySpecialMapMusic, ResetAllMultiplayerState,
    SetObjectEventLoadFlag, TryFadeOutOldMapMusic, WarpIntoMap, gFieldCallback,
};
use crate::palette::gPlttBufferFaded;
use crate::palette::{LoadPalette, gPaletteFade};
use crate::scanline_effect::{ScanlineEffect_Clear, ScanlineEffect_Stop};
use crate::script::{LockPlayerFieldControls, ScriptContext_Enable, UnlockPlayerFieldControls};
use crate::sound::PlaySE;
use crate::start_menu::{ShowReturnToFieldStartMenu, Task_ShowStartMenu};
use crate::task::DestroyTask;
use crate::task::gTasks;
use crate::task::{task_get, task_set};
use crate::trainer_hill::OnTrainerHillEReaderChallengeFloor;
#[allow(unused_imports)]
use crate::types::*;
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
/// `FindTaskIdByFunc` with this module's view of its types.
#[inline]
unsafe fn FindTaskIdByFunc(a0: Option<unsafe fn(u8)>) -> u8 {
    unsafe { crate::task::FindTaskIdByFunc(core::mem::transmute(a0)) }
}
/// `FuncIsActiveTask` with this module's view of its types.
#[inline]
unsafe fn FuncIsActiveTask(a0: Option<unsafe fn(u8)>) -> u8 {
    unsafe { crate::task::FuncIsActiveTask(core::mem::transmute(a0)) }
}
/// `ScanlineEffect_SetParams` with this module's view of its types.
#[inline]
unsafe fn ScanlineEffect_SetParams(a0: ScanlineEffectParams) {
    unsafe {
        crate::scanline_effect::ScanlineEffect_SetParams(core::mem::transmute(a0));
    }
}
// The C's names for task and sprite data slots.
const tState: usize = 0;
// Data tables (translate with cdata.py): sFlashLevelToRadius gMaxFlashLevel sFlashEffectParams

static sFlashEffectParams: Table<ScanlineEffectParams> =
    Table((&raw const crate::data::field_screen_effect::sFlashEffectParams).cast());
static sFlashLevelToRadius: Table<CArray<u16, 9>> =
    Table((&raw const crate::data::field_screen_effect::sFlashLevelToRadius).cast());

/// `CpuFastSet` with this module's view of its types.
#[inline]
unsafe fn CpuFastSet(a0: *mut c_void, a1: *mut c_void, a2: u32) {
    unsafe {
        crate::syscall::CpuFastSet(a0 as _, a1 as _, a2);
    }
}
/// `SetMainCallback2` with this module's view of its types.
#[inline]
unsafe fn SetMainCallback2(a0: Option<unsafe fn()>) {
    unsafe {
        crate::agb_main::SetMainCallback2(core::mem::transmute(a0));
    }
}

unsafe fn FillPalBufferWhite() {
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
unsafe fn FillPalBufferBlack() {
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
pub unsafe fn WarpFadeInScreen() {
    let previousMapType: u8 = GetLastUsedWarpMapType();
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
pub unsafe fn FadeInFromWhite() {
    FillPalBufferWhite();
    FadeScreen(FADE_FROM_WHITE, 8);
}
#[unsafe(no_mangle)]
pub unsafe fn FadeInFromBlack() {
    FillPalBufferBlack();
    FadeScreen(0, 0);
}
pub unsafe fn WarpFadeOutScreen() {
    let currentMapType: u8 = GetCurrentMapType();
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
unsafe fn SetPlayerVisibility(visible: u8) {
    SetPlayerInvisibility((visible == 0) as u8);
}
pub(crate) unsafe fn Task_WaitForUnionRoomFade(taskId: u8) {
    if WaitForWeatherFadeIn() == TRUE as u32 {
        DestroyTask(taskId);
    }
}
pub unsafe fn FieldCB_ContinueScriptUnionRoom() {
    LockPlayerFieldControls();
    Overworld_PlaySpecialMapMusic();
    FadeInFromBlack();
    CreateTask(Some(Task_WaitForUnionRoomFade), 10);
}
pub(crate) unsafe fn Task_WaitForFadeAndEnableScriptCtx(taskID: u8) {
    if WaitForWeatherFadeIn() == TRUE as u32 {
        DestroyTask(taskID);
        ScriptContext_Enable();
    }
}
#[unsafe(no_mangle)]
pub unsafe fn FieldCB_ContinueScriptHandleMusic() {
    LockPlayerFieldControls();
    Overworld_PlaySpecialMapMusic();
    FadeInFromBlack();
    CreateTask(Some(Task_WaitForFadeAndEnableScriptCtx), 10);
}
pub unsafe fn FieldCB_ContinueScript() {
    LockPlayerFieldControls();
    FadeInFromBlack();
    CreateTask(Some(Task_WaitForFadeAndEnableScriptCtx), 10);
}
pub(crate) unsafe fn Task_ReturnToFieldCableLink(taskId: u8) {
    let task: *mut Task = &raw mut (*gTasks.as_ptr())[taskId];
    match (*task).data[tState] {
        0 => {
            (*task).data[1] = CreateTask_ReestablishCableClubLink() as i16;
            (*task).data[tState] += 1;
        }
        1 => {
            if (*gTasks.as_ptr())[(*task).data[1]].isActive != 1 {
                WarpFadeInScreen();
                (*task).data[tState] += 1;
            }
        }
        2 if WaitForWeatherFadeIn() == TRUE as u32 => {
            UnlockPlayerFieldControls();
            DestroyTask(taskId);
        }
        _ => {}
    }
}
pub unsafe fn FieldCB_ReturnToFieldCableLink() {
    LockPlayerFieldControls();
    Overworld_PlaySpecialMapMusic();
    FillPalBufferBlack();
    CreateTask(Some(Task_ReturnToFieldCableLink), 10);
}
pub(crate) unsafe fn Task_ReturnToFieldWirelessLink(taskId: u8) {
    let task: *mut Task = &raw mut (*gTasks.as_ptr())[taskId];
    match (*task).data[tState] {
        0 => {
            SetLinkStandbyCallback();
            (*task).data[tState] += 1;
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
                (*task).data[tState] += 1;
            }
        }
        2 if WaitForWeatherFadeIn() == TRUE as u32 => {
            StartSendingKeysToLink();
            UnlockPlayerFieldControls();
            DestroyTask(taskId);
        }
        _ => {}
    }
}
pub unsafe fn Task_ReturnToFieldRecordMixing(taskId: u8) {
    let task: *mut Task = &raw mut (*gTasks.as_ptr())[taskId];
    match (*task).data[tState] {
        0 => {
            SetLinkStandbyCallback();
            (*task).data[tState] += 1;
        }
        1 => {
            if IsLinkTaskFinished() != 0 {
                (*task).data[tState] += 1;
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
pub unsafe fn FieldCB_ReturnToFieldWirelessLink() {
    LockPlayerFieldControls();
    Overworld_PlaySpecialMapMusic();
    FillPalBufferBlack();
    CreateTask(Some(Task_ReturnToFieldWirelessLink), 10);
}
unsafe fn SetUpWarpExitTask() {
    let mut x: i16 = 0;
    let mut y: i16 = 0;
    let mut func: Option<unsafe fn(u8)> = None;
    PlayerGetDestCoords(&raw mut x, &raw mut y);
    let behavior: u8 = MapGridGetMetatileBehaviorAt(x as i32, y as i32) as u8;
    if MetatileBehavior_IsDoor(behavior) == TRUE {
        func = Some(Task_ExitDoor);
    } else if MetatileBehavior_IsNonAnimDoor(behavior) == TRUE {
        func = Some(Task_ExitNonAnimDoor);
    } else {
        func = Some(Task_ExitNonDoor);
    }
    CreateTask(func, 10);
}
pub unsafe fn FieldCB_DefaultWarpExit() {
    Overworld_PlaySpecialMapMusic();
    WarpFadeInScreen();
    SetUpWarpExitTask();
    LockPlayerFieldControls();
}
pub unsafe fn FieldCB_WarpExitFadeFromWhite() {
    Overworld_PlaySpecialMapMusic();
    FadeInFromWhite();
    SetUpWarpExitTask();
    LockPlayerFieldControls();
}
pub unsafe fn FieldCB_WarpExitFadeFromBlack() {
    if OnTrainerHillEReaderChallengeFloor() == 0 {
        Overworld_PlaySpecialMapMusic();
    }
    FadeInFromBlack();
    SetUpWarpExitTask();
    LockPlayerFieldControls();
}
pub(crate) unsafe fn FieldCB_SpinEnterWarp() {
    Overworld_PlaySpecialMapMusic();
    WarpFadeInScreen();
    PlaySE(SE_WARP_OUT);
    CreateTask(Some(Task_SpinEnterWarp), 10);
    LockPlayerFieldControls();
}
pub(crate) unsafe fn FieldCB_MossdeepGymWarpExit() {
    Overworld_PlaySpecialMapMusic();
    WarpFadeInScreen();
    PlaySE(SE_WARP_OUT);
    CreateTask(Some(Task_ExitNonDoor), 10);
    LockPlayerFieldControls();
    SetObjectEventLoadFlag(14);
}
pub(crate) unsafe fn Task_ExitDoor(taskId: u8) {
    let task: *mut Task = &raw mut (*gTasks.as_ptr())[taskId];
    let x: *mut i16 = &raw mut (*task).data[2];
    let y: *mut i16 = &raw mut (*task).data[3];
    match (*task).data[tState] {
        0 => {
            SetPlayerVisibility(FALSE);
            FreezeObjectEvents();
            PlayerGetDestCoords(x, y);
            FieldSetDoorOpened(*x as u32, *y as u32);
            (*task).data[tState] = 1;
        }
        1 => {
            if WaitForWeatherFadeIn() != 0 {
                SetPlayerVisibility(TRUE);
                let objEventId: u8 = GetObjectEventIdByLocalIdAndMap(LOCALID_PLAYER, 0, 0);
                ObjectEventSetHeldMovement(
                    &raw mut gObjectEvents[objEventId],
                    MOVEMENT_ACTION_WALK_NORMAL_DOWN,
                );
                (*task).data[tState] = 2;
            }
        }
        2 => {
            if IsPlayerStandingStill() != 0 {
                (*task).data[1] = FieldAnimateDoorClose(*x as u32, *y as u32) as i16;
                let objEventId: u8 = GetObjectEventIdByLocalIdAndMap(LOCALID_PLAYER, 0, 0);
                ObjectEventClearHeldMovementIfFinished(&raw mut gObjectEvents[objEventId]);
                (*task).data[tState] = 3;
            }
        }
        3 => {
            if (*task).data[1] < 0 || (*gTasks.as_ptr())[(*task).data[1]].isActive != 1 {
                UnfreezeObjectEvents();
                (*task).data[tState] = 4;
            }
        }
        4 => {
            UnlockPlayerFieldControls();
            DestroyTask(taskId);
        }
        _ => {}
    }
}
pub(crate) unsafe fn Task_ExitNonAnimDoor(taskId: u8) {
    let task: *mut Task = &raw mut (*gTasks.as_ptr())[taskId];
    let x: *mut i16 = &raw mut (*task).data[2];
    let y: *mut i16 = &raw mut (*task).data[3];
    match (*task).data[tState] {
        0 => {
            SetPlayerVisibility(FALSE);
            FreezeObjectEvents();
            PlayerGetDestCoords(x, y);
            (*task).data[tState] = 1;
        }
        1 => {
            if WaitForWeatherFadeIn() != 0 {
                SetPlayerVisibility(TRUE);
                let objEventId: u8 = GetObjectEventIdByLocalIdAndMap(LOCALID_PLAYER, 0, 0);
                ObjectEventSetHeldMovement(
                    &raw mut gObjectEvents[objEventId],
                    GetWalkNormalMovementAction(GetPlayerFacingDirection() as u32),
                );
                (*task).data[tState] = 2;
            }
        }
        2 => {
            if IsPlayerStandingStill() != 0 {
                UnfreezeObjectEvents();
                (*task).data[tState] = 3;
            }
        }
        3 => {
            UnlockPlayerFieldControls();
            DestroyTask(taskId);
        }
        _ => {}
    }
}
pub(crate) unsafe fn Task_ExitNonDoor(taskId: u8) {
    match task_get(taskId, tState) {
        0 => {
            FreezeObjectEvents();
            LockPlayerFieldControls();
            task_set(taskId, tState, task_get(taskId, tState) + 1);
        }
        1 if WaitForWeatherFadeIn() != 0 => {
            UnfreezeObjectEvents();
            UnlockPlayerFieldControls();
            DestroyTask(taskId);
        }
        _ => {}
    }
}
pub(crate) unsafe fn Task_WaitForFadeShowStartMenu(taskId: u8) {
    if WaitForWeatherFadeIn() == TRUE as u32 {
        DestroyTask(taskId);
        CreateTask(Some(Task_ShowStartMenu), 80);
    }
}
pub unsafe fn ReturnToFieldOpenStartMenu() {
    FadeInFromBlack();
    CreateTask(Some(Task_WaitForFadeShowStartMenu), 0x50);
    LockPlayerFieldControls();
}
pub unsafe fn FieldCB_ReturnToFieldOpenStartMenu() -> u8 {
    ShowReturnToFieldStartMenu();
    FALSE
}
pub(crate) unsafe fn Task_ReturnToFieldNoScript(taskId: u8) {
    if WaitForWeatherFadeIn() == 1 {
        UnlockPlayerFieldControls();
        DestroyTask(taskId);
        ScriptUnfreezeObjectEvents();
    }
}
pub unsafe fn FieldCB_ReturnToFieldNoScript() {
    LockPlayerFieldControls();
    FadeInFromBlack();
    CreateTask(Some(Task_ReturnToFieldNoScript), 10);
}
#[unsafe(no_mangle)]
pub unsafe fn FieldCB_ReturnToFieldNoScriptCheckMusic() {
    LockPlayerFieldControls();
    Overworld_PlaySpecialMapMusic();
    FadeInFromBlack();
    CreateTask(Some(Task_ReturnToFieldNoScript), 10);
}
unsafe fn PaletteFadeActive() -> u32 {
    gPaletteFade.active() as u32
}
unsafe fn WaitForWeatherFadeIn() -> u32 {
    if IsWeatherNotFadingIn() == TRUE {
        return TRUE as u32;
    } else {
        return FALSE as u32;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
pub unsafe fn DoWarp() {
    LockPlayerFieldControls();
    TryFadeOutOldMapMusic();
    WarpFadeOutScreen();
    PlayRainStoppingSoundEffect();
    PlaySE(SE_EXIT);
    gFieldCallback = Some(FieldCB_DefaultWarpExit);
    CreateTask(Some(Task_WarpAndLoadMap), 10);
}
#[unsafe(no_mangle)]
pub unsafe fn DoDiveWarp() {
    LockPlayerFieldControls();
    TryFadeOutOldMapMusic();
    WarpFadeOutScreen();
    PlayRainStoppingSoundEffect();
    gFieldCallback = Some(FieldCB_DefaultWarpExit);
    CreateTask(Some(Task_WarpAndLoadMap), 10);
}
pub unsafe fn DoWhiteFadeWarp() {
    LockPlayerFieldControls();
    TryFadeOutOldMapMusic();
    FadeScreen(FADE_TO_WHITE, 8);
    PlayRainStoppingSoundEffect();
    gFieldCallback = Some(FieldCB_WarpExitFadeFromWhite);
    CreateTask(Some(Task_WarpAndLoadMap), 10);
}
pub unsafe fn DoDoorWarp() {
    LockPlayerFieldControls();
    gFieldCallback = Some(FieldCB_DefaultWarpExit);
    CreateTask(Some(Task_DoDoorWarp), 10);
}
#[unsafe(no_mangle)]
pub unsafe fn DoFallWarp() {
    DoDiveWarp();
    gFieldCallback = Some(FieldCB_FallWarpExit);
}
pub unsafe fn DoEscalatorWarp(metatileBehavior: u8) {
    LockPlayerFieldControls();
    StartEscalatorWarp(metatileBehavior, 10);
}
pub unsafe fn DoLavaridgeGymB1FWarp() {
    LockPlayerFieldControls();
    StartLavaridgeGymB1FWarp(10);
}
pub unsafe fn DoLavaridgeGym1FWarp() {
    LockPlayerFieldControls();
    StartLavaridgeGym1FWarp(10);
}
pub unsafe fn DoTeleportTileWarp() {
    LockPlayerFieldControls();
    TryFadeOutOldMapMusic();
    WarpFadeOutScreen();
    PlaySE(SE_WARP_IN);
    CreateTask(Some(Task_WarpAndLoadMap), 10);
    gFieldCallback = Some(FieldCB_SpinEnterWarp);
}
pub unsafe fn DoMossdeepGymWarp() {
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
pub unsafe fn DoPortholeWarp() {
    LockPlayerFieldControls();
    WarpFadeOutScreen();
    CreateTask(Some(Task_WarpAndLoadMap), 10);
    gFieldCallback = Some(FieldCB_ShowPortholeView);
}
pub(crate) unsafe fn Task_DoCableClubWarp(taskId: u8) {
    let task: *mut Task = &raw mut (*gTasks.as_ptr())[taskId];
    match (*task).data[tState] {
        0 => {
            LockPlayerFieldControls();
            (*task).data[tState] += 1;
        }
        1 => {
            if PaletteFadeActive() == 0 && BGMusicStopped() != 0 {
                (*task).data[tState] += 1;
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
pub unsafe fn DoCableClubWarp() {
    LockPlayerFieldControls();
    TryFadeOutOldMapMusic();
    WarpFadeOutScreen();
    PlaySE(SE_EXIT);
    CreateTask(Some(Task_DoCableClubWarp), 10);
}
pub(crate) unsafe fn Task_ReturnToWorldFromLinkRoom(taskId: u8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
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
        2 if gReceivedRemoteLinkPlayers == 0 => {
            WarpIntoMap();
            SetMainCallback2(Some(CB2_LoadMap));
            DestroyTask(taskId);
        }
        _ => {}
    }
}
#[unsafe(no_mangle)]
pub unsafe fn ReturnFromLinkRoom() {
    CreateTask(Some(Task_ReturnToWorldFromLinkRoom), 10);
}
pub(crate) unsafe fn Task_WarpAndLoadMap(taskId: u8) {
    let task: *mut Task = &raw mut (*gTasks.as_ptr())[taskId];
    match (*task).data[tState] {
        0 => {
            FreezeObjectEvents();
            LockPlayerFieldControls();
            (*task).data[tState] += 1;
        }
        1 => {
            if PaletteFadeActive() == 0 {
                if (*task).data[1] == 0 {
                    ClearMirageTowerPulseBlendEffect();
                    (*task).data[1] = 1;
                }
                if BGMusicStopped() != 0 {
                    (*task).data[tState] += 1;
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
pub(crate) unsafe fn Task_DoDoorWarp(taskId: u8) {
    let task: *mut Task = &raw mut (*gTasks.as_ptr())[taskId];
    let x: *mut i16 = &raw mut (*task).data[2];
    let y: *mut i16 = &raw mut (*task).data[3];
    match (*task).data[tState] {
        0 => {
            FreezeObjectEvents();
            PlayerGetDestCoords(x, y);
            PlaySE(GetDoorSoundEffect(*x as u32, *y as u32 - 1) as u16);
            (*task).data[1] = FieldAnimateDoorOpen(*x as u32, *y as u32 - 1) as i16;
            (*task).data[tState] = 1;
        }
        1 => {
            if (*task).data[1] < 0 || (*gTasks.as_ptr())[(*task).data[1]].isActive != 1 {
                let mut objEventId: u8 = GetObjectEventIdByLocalIdAndMap(LOCALID_PLAYER, 0, 0);
                ObjectEventClearHeldMovementIfActive(&raw mut gObjectEvents[objEventId]);
                objEventId = GetObjectEventIdByLocalIdAndMap(LOCALID_PLAYER, 0, 0);
                ObjectEventSetHeldMovement(
                    &raw mut gObjectEvents[objEventId],
                    MOVEMENT_ACTION_WALK_NORMAL_UP,
                );
                (*task).data[tState] = 2;
            }
        }
        2 => {
            if IsPlayerStandingStill() != 0 {
                (*task).data[1] = FieldAnimateDoorClose(*x as u32, *y as u32 - 1) as i16;
                let objEventId: u8 = GetObjectEventIdByLocalIdAndMap(LOCALID_PLAYER, 0, 0);
                ObjectEventClearHeldMovementIfFinished(&raw mut gObjectEvents[objEventId]);
                SetPlayerVisibility(FALSE);
                (*task).data[tState] = 3;
            }
        }
        3 => {
            if (*task).data[1] < 0 || (*gTasks.as_ptr())[(*task).data[1]].isActive != 1 {
                (*task).data[tState] = 4;
            }
        }
        4 => {
            TryFadeOutOldMapMusic();
            WarpFadeOutScreen();
            PlayRainStoppingSoundEffect();
            (*task).data[tState] = 0;
            (*task).func = Some(Task_WarpAndLoadMap);
        }
        _ => {}
    }
}
pub(crate) unsafe fn Task_DoContestHallWarp(taskId: u8) {
    let task: *mut Task = &raw mut (*gTasks.as_ptr())[taskId];
    match (*task).data[tState] {
        0 => {
            FreezeObjectEvents();
            LockPlayerFieldControls();
            (*task).data[tState] += 1;
        }
        1 => {
            if PaletteFadeActive() == 0 && BGMusicStopped() != 0 {
                (*task).data[tState] += 1;
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
pub unsafe fn DoContestHallWarp() {
    LockPlayerFieldControls();
    TryFadeOutOldMapMusic();
    WarpFadeOutScreen();
    PlayRainStoppingSoundEffect();
    PlaySE(SE_EXIT);
    gFieldCallback = Some(FieldCB_WarpExitFadeFromBlack);
    CreateTask(Some(Task_DoContestHallWarp), 10);
}
unsafe fn SetFlashScanlineEffectWindowBoundary(
    dest: *mut u16,
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
unsafe fn SetFlashScanlineEffectWindowBoundaries(
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
unsafe fn SetOrbFlashScanlineEffectWindowBoundary(
    dest: *mut u16,
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
unsafe fn SetOrbFlashScanlineEffectWindowBoundaries(
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
pub(crate) unsafe fn UpdateFlashLevelEffect(taskId: u8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    match *data {
        0 => {
            SetFlashScanlineEffectWindowBoundaries(
                (*(&raw const crate::scanline_effect::gScanlineEffectRegBuffers)
                    .cast::<CArray<CArray<u16, 960>, 2>>()
                    .cast_mut())[(*(&raw const crate::scanline_effect::gScanlineEffect)
                    .cast::<ScanlineEffect>()
                    .cast_mut())
                .srcBuffer]
                    .as_mut_ptr(),
                *data.at(1) as i32,
                *data.at(2) as i32,
                *data.at(3) as i32,
            );
            *data = 1;
        }
        1 => {
            SetFlashScanlineEffectWindowBoundaries(
                (*(&raw const crate::scanline_effect::gScanlineEffectRegBuffers)
                    .cast::<CArray<CArray<u16, 960>, 2>>()
                    .cast_mut())[(*(&raw const crate::scanline_effect::gScanlineEffect)
                    .cast::<ScanlineEffect>()
                    .cast_mut())
                .srcBuffer]
                    .as_mut_ptr(),
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
pub(crate) unsafe fn UpdateOrbFlashEffect(taskId: u8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    match *data {
        0 => {
            SetOrbFlashScanlineEffectWindowBoundaries(
                (*(&raw const crate::scanline_effect::gScanlineEffectRegBuffers)
                    .cast::<CArray<CArray<u16, 960>, 2>>()
                    .cast_mut())[(*(&raw const crate::scanline_effect::gScanlineEffect)
                    .cast::<ScanlineEffect>()
                    .cast_mut())
                .srcBuffer]
                    .as_mut_ptr(),
                *data.at(1) as i32,
                *data.at(2) as i32,
                *data.at(3) as i32,
            );
            *data = 1;
        }
        1 => {
            SetOrbFlashScanlineEffectWindowBoundaries(
                (*(&raw const crate::scanline_effect::gScanlineEffectRegBuffers)
                    .cast::<CArray<CArray<u16, 960>, 2>>()
                    .cast_mut())[(*(&raw const crate::scanline_effect::gScanlineEffect)
                    .cast::<ScanlineEffect>()
                    .cast_mut())
                .srcBuffer]
                    .as_mut_ptr(),
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
pub(crate) unsafe fn Task_WaitForFlashUpdate(taskId: u8) {
    if FuncIsActiveTask(Some(UpdateFlashLevelEffect)) == 0 {
        ScriptContext_Enable();
        DestroyTask(taskId);
    }
}
unsafe fn StartWaitForFlashUpdate() {
    if FuncIsActiveTask(Some(Task_WaitForFlashUpdate)) == 0 {
        CreateTask(Some(Task_WaitForFlashUpdate), 80);
    }
}
unsafe fn StartUpdateFlashLevelEffect(
    centerX: i32,
    centerY: i32,
    initialFlashRadius: i32,
    destFlashRadius: i32,
    clearScanlineEffect: i32,
    delta: u8,
) -> u8 {
    let taskId: u8 = CreateTask(Some(UpdateFlashLevelEffect), 80);
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
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
    taskId
}
unsafe fn StartUpdateOrbFlashEffect(
    centerX: i32,
    centerY: i32,
    initialFlashRadius: i32,
    destFlashRadius: i32,
    clearScanlineEffect: i32,
    delta: u8,
) -> u8 {
    let taskId: u8 = CreateTask(Some(UpdateOrbFlashEffect), 80);
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
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
    taskId
}
pub unsafe fn AnimateFlash(newFlashLevel: u8) {
    let curFlashLevel: u8 = GetFlashLevel();
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
pub unsafe fn WriteFlashScanlineEffectBuffer(flashLevel: u8) {
    if flashLevel != 0 {
        SetFlashScanlineEffectWindowBoundaries(
            &raw mut (*(&raw const crate::scanline_effect::gScanlineEffectRegBuffers)
                .cast::<CArray<CArray<u16, 960>, 2>>()
                .cast_mut())[0][0],
            120,
            80,
            sFlashLevelToRadius[flashLevel] as i32,
        );
        CpuFastSet(
            &raw mut (*(&raw const crate::scanline_effect::gScanlineEffectRegBuffers)
                .cast::<CArray<CArray<u16, 960>, 2>>()
                .cast_mut())[0] as *mut c_void,
            &raw mut (*(&raw const crate::scanline_effect::gScanlineEffectRegBuffers)
                .cast::<CArray<CArray<u16, 960>, 2>>()
                .cast_mut())[1] as *mut c_void,
            480,
        );
    }
}
pub unsafe fn WriteBattlePyramidViewScanlineEffectBuffer() {
    SetFlashScanlineEffectWindowBoundaries(
        &raw mut (*(&raw const crate::scanline_effect::gScanlineEffectRegBuffers)
            .cast::<CArray<CArray<u16, 960>, 2>>()
            .cast_mut())[0][0],
        120,
        80,
        (*gSaveBlock2Ptr).frontier.pyramidLightRadius as i32,
    );
    CpuFastSet(
        &raw mut (*(&raw const crate::scanline_effect::gScanlineEffectRegBuffers)
            .cast::<CArray<CArray<u16, 960>, 2>>()
            .cast_mut())[0] as *mut c_void,
        &raw mut (*(&raw const crate::scanline_effect::gScanlineEffectRegBuffers)
            .cast::<CArray<CArray<u16, 960>, 2>>()
            .cast_mut())[1] as *mut c_void,
        480,
    );
}
pub(crate) unsafe fn Task_SpinEnterWarp(taskId: u8) {
    match task_get(taskId, tState) {
        0 => {
            FreezeObjectEvents();
            LockPlayerFieldControls();
            DoPlayerSpinEntrance();
            task_set(taskId, tState, task_get(taskId, tState) + 1);
        }
        1 if WaitForWeatherFadeIn() != 0 && IsPlayerSpinEntranceActive() != TRUE as u32 => {
            UnfreezeObjectEvents();
            UnlockPlayerFieldControls();
            DestroyTask(taskId);
        }
        _ => {}
    }
}
pub(crate) unsafe fn Task_SpinExitWarp(taskId: u8) {
    let task: *mut Task = &raw mut (*gTasks.as_ptr())[taskId];
    match (*task).data[tState] {
        0 => {
            FreezeObjectEvents();
            LockPlayerFieldControls();
            PlaySE(SE_WARP_IN);
            DoPlayerSpinExit();
            (*task).data[tState] += 1;
        }
        1 => {
            if IsPlayerSpinExitActive() == 0 {
                WarpFadeOutScreen();
                (*task).data[tState] += 1;
            }
        }
        2 => {
            if PaletteFadeActive() == 0 && BGMusicStopped() != 0 {
                (*task).data[tState] += 1;
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
pub unsafe fn DoSpinEnterWarp() {
    LockPlayerFieldControls();
    CreateTask(Some(Task_WarpAndLoadMap), 10);
    gFieldCallback = Some(FieldCB_SpinEnterWarp);
}
pub unsafe fn DoSpinExitWarp() {
    LockPlayerFieldControls();
    gFieldCallback = Some(FieldCB_DefaultWarpExit);
    CreateTask(Some(Task_SpinExitWarp), 10);
}
unsafe fn LoadOrbEffectPalette(blueOrb: u8) {
    let mut color: CArray<u16, 1> = zeroed();
    if blueOrb == 0 {
        color[0] = 31;
    } else {
        color[0] = 31744;
    }
    for i in 0..16i32 {
        LoadPalette(color.as_mut_ptr() as *mut c_void, 240 + i as u16, 2);
    }
}
unsafe fn UpdateOrbEffectBlend(shakeDir: u16) -> u8 {
    let mut lo: u8 = (67108946_usize as *mut u16).read_volatile() as u8;
    let mut hi: u8 = ((67108946_usize as *mut u16).read_volatile() >> 8) as u8;
    if shakeDir != 0 {
        lo = lo.saturating_sub(1);
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
        0
    }
}
pub(crate) unsafe fn Task_OrbEffect(taskId: u8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    match *data {
        0 => {
            *data.at(6) = (0x4000000_usize as *mut u16).read_volatile() as i16;
            *data.at(7) = (67108944_usize as *mut u16).read_volatile() as i16;
            *data.at(8) = (67108946_usize as *mut u16).read_volatile() as i16;
            *data.at(9) = (67108936_usize as *mut u16).read_volatile() as i16;
            *data.at(10) = (67108938_usize as *mut u16).read_volatile() as i16;
            ClearGpuRegBits(REG_OFFSET_DISPCNT, DISPCNT_WIN1_ON);
            SetGpuRegBits(
                REG_OFFSET_BLDCNT,
                (*(&raw const crate::io_reg::gOrbEffectBackgroundLayerFlags)
                    .cast::<CArray<u16, 0>>())[0],
            );
            SetGpuReg(REG_OFFSET_BLDALPHA, 1804);
            SetGpuReg(REG_OFFSET_WININ, 63);
            SetGpuReg(REG_OFFSET_WINOUT, 30);
            SetBgTilemapPalette(0, 0, 0, DISPLAY_TILE_WIDTH, DISPLAY_TILE_HEIGHT, 0xF);
            ScheduleBgCopyTilemapToVram(0);
            SetOrbFlashScanlineEffectWindowBoundaries(
                &raw mut (*(&raw const crate::scanline_effect::gScanlineEffectRegBuffers)
                    .cast::<CArray<CArray<u16, 960>, 2>>()
                    .cast_mut())[0][0],
                *data.at(2) as i32,
                *data.at(3) as i32,
                1,
            );
            CpuFastSet(
                &raw mut (*(&raw const crate::scanline_effect::gScanlineEffectRegBuffers)
                    .cast::<CArray<CArray<u16, 960>, 2>>()
                    .cast_mut())[0] as *mut c_void,
                &raw mut (*(&raw const crate::scanline_effect::gScanlineEffectRegBuffers)
                    .cast::<CArray<CArray<u16, 960>, 2>>()
                    .cast_mut())[1] as *mut c_void,
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
pub unsafe fn DoOrbEffect() {
    let taskId: u8 = CreateTask(Some(Task_OrbEffect), 80);
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
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
pub unsafe fn FadeOutOrbEffect() {
    let taskId: u8 = FindTaskIdByFunc(Some(Task_OrbEffect));
    task_set(taskId, tState, 6);
}
#[unsafe(no_mangle)]
pub unsafe fn Script_FadeOutMapMusic() {
    Overworld_FadeOutMapMusic();
    CreateTask(Some(Task_EnableScriptAfterMusicFade), 80);
}
pub(crate) unsafe fn Task_EnableScriptAfterMusicFade(taskId: u8) {
    if BGMusicStopped() == TRUE {
        DestroyTask(taskId);
        ScriptContext_Enable();
    }
}
