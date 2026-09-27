//! Translated from `src/field_screen_effect.c` by tools/rustport/c2rs.py, then reviewed.
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

// Data tables (translate with cdata.py): sFlashLevelToRadius gMaxFlashLevel sFlashEffectParams
#[allow(unused_imports)]
use crate::data::field_screen_effect::*;

unsafe extern "C" {
    static mut gFieldCallback: u8;
    static mut gObjectEvents: u8;
    static mut gOrbEffectBackgroundLayerFlags: u8;
    static mut gPaletteFade: u8;
    static mut gPlttBufferFaded: u8;
    static mut gReceivedRemoteLinkPlayers: u8;
    static mut gSaveBlock2Ptr: u8;
    static mut gScanlineEffect: u8;
    static mut gScanlineEffectRegBuffers: u8;
    static mut gSpecialVar_Result: u8;
    static mut gTasks: u8;
    fn BGMusicStopped() -> u8;
    fn BgDmaFill(a0: u32, a1: u8, a2: i32, a3: i32);
    fn CB2_LoadMap();
    fn CB2_ReturnToFieldCableClub();
    fn CB2_ReturnToFieldContestHall();
    fn ClearGpuRegBits(a0: u8, a1: u16);
    fn ClearLinkCallback_2();
    fn ClearMirageTowerPulseBlendEffect();
    fn CpuFastSet(a0: *mut u8, a1: *mut u8, a2: u32);
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
    fn GetDestinationWarpMapHeader() -> *mut u8;
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
    fn LoadPalette(a0: *mut u8, a1: u16, a2: u16);
    fn LockPlayerFieldControls();
    fn MapGridGetMetatileBehaviorAt(a0: i32, a1: i32) -> i32;
    fn MetatileBehavior_IsDoor(a0: u8) -> u8;
    fn MetatileBehavior_IsNonAnimDoor(a0: u8) -> u8;
    fn ObjectEventClearHeldMovementIfActive(a0: *mut u8);
    fn ObjectEventClearHeldMovementIfFinished(a0: *mut u8) -> u8;
    fn ObjectEventSetHeldMovement(a0: *mut u8, a1: u8) -> u8;
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
    fn ScanlineEffect_SetParams(a0: crate::c::Rec4<12>);
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
    unsafe {
        {
            let mut tmp: u32 = 0u32;
            (&raw mut tmp).write_volatile(2147450879u32);
            'l1: loop {
                'l2: {
                    CpuFastSet(
                        (&raw mut tmp).cast::<u8>(),
                        (((&raw mut gPlttBufferFaded).cast::<u16>()).cast::<u16>()).cast::<u8>(),
                        ((16777216i32
                            | (crate::c::div_i32(1024i32, crate::c::div_i32(32i32, 8i32))
                                & 2097151i32)) as u32),
                    );
                }
                if !((0i32) != 0) {
                    break 'l1;
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn FillPalBufferBlack() {
    unsafe {
        {
            let mut tmp: u32 = 0u32;
            (&raw mut tmp).write_volatile(0u32);
            'l1: loop {
                'l2: {
                    CpuFastSet(
                        (&raw mut tmp).cast::<u8>(),
                        (((&raw mut gPlttBufferFaded).cast::<u16>()).cast::<u16>()).cast::<u8>(),
                        ((16777216i32
                            | (crate::c::div_i32(1024i32, crate::c::div_i32(32i32, 8i32))
                                & 2097151i32)) as u32),
                    );
                }
                if !((0i32) != 0) {
                    break 'l1;
                }
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn WarpFadeInScreen() {
    unsafe {
        let mut previousMapType: u8 = GetLastUsedWarpMapType();
        'l1: {
            let __sw1 = ((GetMapPairFadeFromType(previousMapType, GetCurrentMapType())) as i32);
            if __sw1 == 0i32 {
                FillPalBufferBlack();
                FadeScreen(0u8, 0i8);
                break 'l1;
            }
            if __sw1 == 1i32 {
                FillPalBufferWhite();
                FadeScreen(2u8, 0i8);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FadeInFromWhite() {
    unsafe {
        FillPalBufferWhite();
        FadeScreen(2u8, 8i8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FadeInFromBlack() {
    unsafe {
        FillPalBufferBlack();
        FadeScreen(0u8, 0i8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn WarpFadeOutScreen() {
    unsafe {
        let mut currentMapType: u8 = GetCurrentMapType();
        'l1: {
            let __sw1 = ((GetMapPairFadeToType(
                currentMapType,
                ((GetDestinationWarpMapHeader()).wrapping_add(23)).read(),
            )) as i32);
            if __sw1 == 0i32 {
                FadeScreen(1u8, 0i8);
                break 'l1;
            }
            if __sw1 == 1i32 {
                FadeScreen(3u8, 0i8);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SetPlayerVisibility(visible: u8) {
    unsafe {
        let mut visible = visible;
        SetPlayerInvisibility(((!((visible) != 0)) as u8));
    }
}
pub(crate) unsafe extern "C" fn Task_WaitForUnionRoomFade(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if WaitForWeatherFadeIn() == 1u32 {
            DestroyTask(taskId);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FieldCB_ContinueScriptUnionRoom() {
    unsafe {
        LockPlayerFieldControls();
        Overworld_PlaySpecialMapMusic();
        FadeInFromBlack();
        CreateTask(Some(Task_WaitForUnionRoomFade), 10u8);
    }
}
pub(crate) unsafe extern "C" fn Task_WaitForFadeAndEnableScriptCtx(taskID: u8) {
    unsafe {
        let mut taskID = taskID;
        if WaitForWeatherFadeIn() == 1u32 {
            DestroyTask(taskID);
            ScriptContext_Enable();
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FieldCB_ContinueScriptHandleMusic() {
    unsafe {
        LockPlayerFieldControls();
        Overworld_PlaySpecialMapMusic();
        FadeInFromBlack();
        CreateTask(Some(Task_WaitForFadeAndEnableScriptCtx), 10u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FieldCB_ContinueScript() {
    unsafe {
        LockPlayerFieldControls();
        FadeInFromBlack();
        CreateTask(Some(Task_WaitForFadeAndEnableScriptCtx), 10u8);
    }
}
pub(crate) unsafe extern "C" fn Task_ReturnToFieldCableLink(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut task: *mut u8 =
            ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40);
        'l1: {
            let __sw1 = (((((task).wrapping_add(8)).cast::<i16>()).read()) as i32);
            if __sw1 == 0i32 {
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1))
                    .write(((CreateTask_ReestablishCableClubLink()) as i16));
                let __p2 = ((task).wrapping_add(8)).cast::<i16>();
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                if ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
                    ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
                        as isize
                        * 40,
                ))
                .wrapping_add(4))
                .read()) as i32)
                    != 1i32
                {
                    WarpFadeInScreen();
                    let __p3 = ((task).wrapping_add(8)).cast::<i16>();
                    (__p3).write(((__p3).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                if WaitForWeatherFadeIn() == 1u32 {
                    UnlockPlayerFieldControls();
                    DestroyTask(taskId);
                }
                break 'l1;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FieldCB_ReturnToFieldCableLink() {
    unsafe {
        LockPlayerFieldControls();
        Overworld_PlaySpecialMapMusic();
        FillPalBufferBlack();
        CreateTask(Some(Task_ReturnToFieldCableLink), 10u8);
    }
}
pub(crate) unsafe extern "C" fn Task_ReturnToFieldWirelessLink(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut task: *mut u8 =
            ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40);
        'l1: {
            let __sw1 = (((((task).wrapping_add(8)).cast::<i16>()).read()) as i32);
            if __sw1 == 0i32 {
                SetLinkStandbyCallback();
                let __p2 = ((task).wrapping_add(8)).cast::<i16>();
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                if !((IsLinkTaskFinished()) != 0) {
                    if (({
                        let __p3 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1);
                        let __t4 = ((__p3).read()).wrapping_add(1);
                        (__p3).write(__t4);
                        __t4
                    }) as i32)
                        > 1800i32
                    {
                        RfuSetErrorParams(24576u32);
                    }
                } else {
                    WarpFadeInScreen();
                    let __p5 = ((task).wrapping_add(8)).cast::<i16>();
                    (__p5).write(((__p5).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                if WaitForWeatherFadeIn() == 1u32 {
                    StartSendingKeysToLink();
                    UnlockPlayerFieldControls();
                    DestroyTask(taskId);
                }
                break 'l1;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Task_ReturnToFieldRecordMixing(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut task: *mut u8 =
            ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40);
        'l1: {
            let __sw1 = (((((task).wrapping_add(8)).cast::<i16>()).read()) as i32);
            if __sw1 == 0i32 {
                SetLinkStandbyCallback();
                let __p2 = ((task).wrapping_add(8)).cast::<i16>();
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                if (IsLinkTaskFinished()) != 0 {
                    let __p3 = ((task).wrapping_add(8)).cast::<i16>();
                    (__p3).write(((__p3).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                StartSendingKeysToLink();
                ResetAllMultiplayerState();
                UnlockPlayerFieldControls();
                DestroyTask(taskId);
                break 'l1;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FieldCB_ReturnToFieldWirelessLink() {
    unsafe {
        LockPlayerFieldControls();
        Overworld_PlaySpecialMapMusic();
        FillPalBufferBlack();
        CreateTask(Some(Task_ReturnToFieldWirelessLink), 10u8);
    }
}
pub(crate) unsafe extern "C" fn SetUpWarpExitTask() {
    unsafe {
        let mut x: i16 = 0i16;
        let mut y: i16 = 0i16;
        let mut behavior: u8 = 0u8;
        let mut func: Option<unsafe extern "C" fn(u8)> = None;
        PlayerGetDestCoords(&raw mut x, &raw mut y);
        behavior = ((MapGridGetMetatileBehaviorAt(((x) as i32), ((y) as i32))) as u8);
        if ((MetatileBehavior_IsDoor(behavior)) as i32) == 1i32 {
            func = Some(Task_ExitDoor);
        } else {
            if ((MetatileBehavior_IsNonAnimDoor(behavior)) as i32) == 1i32 {
                func = Some(Task_ExitNonAnimDoor);
            } else {
                func = Some(Task_ExitNonDoor);
            }
        }
        CreateTask(func, 10u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FieldCB_DefaultWarpExit() {
    unsafe {
        Overworld_PlaySpecialMapMusic();
        WarpFadeInScreen();
        SetUpWarpExitTask();
        LockPlayerFieldControls();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FieldCB_WarpExitFadeFromWhite() {
    unsafe {
        Overworld_PlaySpecialMapMusic();
        FadeInFromWhite();
        SetUpWarpExitTask();
        LockPlayerFieldControls();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FieldCB_WarpExitFadeFromBlack() {
    unsafe {
        if !((OnTrainerHillEReaderChallengeFloor()) != 0) {
            Overworld_PlaySpecialMapMusic();
        }
        FadeInFromBlack();
        SetUpWarpExitTask();
        LockPlayerFieldControls();
    }
}
pub(crate) unsafe extern "C" fn FieldCB_SpinEnterWarp() {
    unsafe {
        Overworld_PlaySpecialMapMusic();
        WarpFadeInScreen();
        PlaySE(46u16);
        CreateTask(Some(Task_SpinEnterWarp), 10u8);
        LockPlayerFieldControls();
    }
}
pub(crate) unsafe extern "C" fn FieldCB_MossdeepGymWarpExit() {
    unsafe {
        Overworld_PlaySpecialMapMusic();
        WarpFadeInScreen();
        PlaySE(46u16);
        CreateTask(Some(Task_ExitNonDoor), 10u8);
        LockPlayerFieldControls();
        SetObjectEventLoadFlag(14u8);
    }
}
pub(crate) unsafe extern "C" fn Task_ExitDoor(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut task: *mut u8 =
            ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40);
        let mut x: *mut i16 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2);
        let mut y: *mut i16 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3);
        'l1: {
            let __sw1 = (((((task).wrapping_add(8)).cast::<i16>()).read()) as i32);
            if __sw1 == 0i32 {
                SetPlayerVisibility(0u8);
                FreezeObjectEvents();
                PlayerGetDestCoords(x, y);
                FieldSetDoorOpened((((x).read()) as u32), (((y).read()) as u32));
                (((task).wrapping_add(8)).cast::<i16>()).write(1i16);
                break 'l1;
            }
            if __sw1 == 1i32 {
                if (WaitForWeatherFadeIn()) != 0 {
                    let mut objEventId: u8 = 0u8;
                    SetPlayerVisibility(1u8);
                    objEventId = GetObjectEventIdByLocalIdAndMap(255u8, 0u8, 0u8);
                    ObjectEventSetHeldMovement(
                        ((&raw mut gObjectEvents).cast::<u8>())
                            .wrapping_offset(((objEventId) as i32) as isize * 36),
                        8u8,
                    );
                    (((task).wrapping_add(8)).cast::<i16>()).write(2i16);
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                if (IsPlayerStandingStill()) != 0 {
                    let mut objEventId: u8 = 0u8;
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(
                        ((FieldAnimateDoorClose((((x).read()) as u32), (((y).read()) as u32)))
                            as i16),
                    );
                    objEventId = GetObjectEventIdByLocalIdAndMap(255u8, 0u8, 0u8);
                    ObjectEventClearHeldMovementIfFinished(
                        ((&raw mut gObjectEvents).cast::<u8>())
                            .wrapping_offset(((objEventId) as i32) as isize * 36),
                    );
                    (((task).wrapping_add(8)).cast::<i16>()).write(3i16);
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                if (((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
                    < 0i32)
                    || (((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
                        ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read())
                            as i32) as isize
                            * 40,
                    ))
                    .wrapping_add(4))
                    .read()) as i32)
                        != 1i32)
                {
                    UnfreezeObjectEvents();
                    (((task).wrapping_add(8)).cast::<i16>()).write(4i16);
                }
                break 'l1;
            }
            if __sw1 == 4i32 {
                UnlockPlayerFieldControls();
                DestroyTask(taskId);
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_ExitNonAnimDoor(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut task: *mut u8 =
            ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40);
        let mut x: *mut i16 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2);
        let mut y: *mut i16 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3);
        'l1: {
            let __sw1 = (((((task).wrapping_add(8)).cast::<i16>()).read()) as i32);
            if __sw1 == 0i32 {
                SetPlayerVisibility(0u8);
                FreezeObjectEvents();
                PlayerGetDestCoords(x, y);
                (((task).wrapping_add(8)).cast::<i16>()).write(1i16);
                break 'l1;
            }
            if __sw1 == 1i32 {
                if (WaitForWeatherFadeIn()) != 0 {
                    let mut objEventId: u8 = 0u8;
                    SetPlayerVisibility(1u8);
                    objEventId = GetObjectEventIdByLocalIdAndMap(255u8, 0u8, 0u8);
                    ObjectEventSetHeldMovement(
                        ((&raw mut gObjectEvents).cast::<u8>())
                            .wrapping_offset(((objEventId) as i32) as isize * 36),
                        GetWalkNormalMovementAction(((GetPlayerFacingDirection()) as u32)),
                    );
                    (((task).wrapping_add(8)).cast::<i16>()).write(2i16);
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                if (IsPlayerStandingStill()) != 0 {
                    UnfreezeObjectEvents();
                    (((task).wrapping_add(8)).cast::<i16>()).write(3i16);
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                UnlockPlayerFieldControls();
                DestroyTask(taskId);
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_ExitNonDoor(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        'l1: {
            let __sw1 = (((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .read()) as i32);
            if __sw1 == 0i32 {
                FreezeObjectEvents();
                LockPlayerFieldControls();
                let __p2 = ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>();
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                if (WaitForWeatherFadeIn()) != 0 {
                    UnfreezeObjectEvents();
                    UnlockPlayerFieldControls();
                    DestroyTask(taskId);
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_WaitForFadeShowStartMenu(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if WaitForWeatherFadeIn() == 1u32 {
            DestroyTask(taskId);
            CreateTask(Some(Task_ShowStartMenu), 80u8);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ReturnToFieldOpenStartMenu() {
    unsafe {
        FadeInFromBlack();
        CreateTask(Some(Task_WaitForFadeShowStartMenu), 80u8);
        LockPlayerFieldControls();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FieldCB_ReturnToFieldOpenStartMenu() -> u8 {
    unsafe {
        ShowReturnToFieldStartMenu();
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn Task_ReturnToFieldNoScript(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if WaitForWeatherFadeIn() == 1u32 {
            UnlockPlayerFieldControls();
            DestroyTask(taskId);
            ScriptUnfreezeObjectEvents();
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FieldCB_ReturnToFieldNoScript() {
    unsafe {
        LockPlayerFieldControls();
        FadeInFromBlack();
        CreateTask(Some(Task_ReturnToFieldNoScript), 10u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FieldCB_ReturnToFieldNoScriptCheckMusic() {
    unsafe {
        LockPlayerFieldControls();
        Overworld_PlaySpecialMapMusic();
        FadeInFromBlack();
        CreateTask(Some(Task_ReturnToFieldNoScript), 10u8);
    }
}
pub(crate) unsafe extern "C" fn PaletteFadeActive() -> u32 {
    unsafe {
        return ((crate::c::bf_read(
            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
            7,
            1,
            false,
        ) as u16) as u32);
    }
}
pub(crate) unsafe extern "C" fn WaitForWeatherFadeIn() -> u32 {
    unsafe {
        if ((IsWeatherNotFadingIn()) as i32) == 1i32 {
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DoWarp() {
    unsafe {
        LockPlayerFieldControls();
        TryFadeOutOldMapMusic();
        WarpFadeOutScreen();
        PlayRainStoppingSoundEffect();
        PlaySE(9u16);
        ((&raw mut gFieldCallback).cast::<Option<unsafe extern "C" fn()>>())
            .write(Some(FieldCB_DefaultWarpExit));
        CreateTask(Some(Task_WarpAndLoadMap), 10u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DoDiveWarp() {
    unsafe {
        LockPlayerFieldControls();
        TryFadeOutOldMapMusic();
        WarpFadeOutScreen();
        PlayRainStoppingSoundEffect();
        ((&raw mut gFieldCallback).cast::<Option<unsafe extern "C" fn()>>())
            .write(Some(FieldCB_DefaultWarpExit));
        CreateTask(Some(Task_WarpAndLoadMap), 10u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DoWhiteFadeWarp() {
    unsafe {
        LockPlayerFieldControls();
        TryFadeOutOldMapMusic();
        FadeScreen(3u8, 8i8);
        PlayRainStoppingSoundEffect();
        ((&raw mut gFieldCallback).cast::<Option<unsafe extern "C" fn()>>())
            .write(Some(FieldCB_WarpExitFadeFromWhite));
        CreateTask(Some(Task_WarpAndLoadMap), 10u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DoDoorWarp() {
    unsafe {
        LockPlayerFieldControls();
        ((&raw mut gFieldCallback).cast::<Option<unsafe extern "C" fn()>>())
            .write(Some(FieldCB_DefaultWarpExit));
        CreateTask(Some(Task_DoDoorWarp), 10u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DoFallWarp() {
    unsafe {
        DoDiveWarp();
        ((&raw mut gFieldCallback).cast::<Option<unsafe extern "C" fn()>>())
            .write(Some(FieldCB_FallWarpExit));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DoEscalatorWarp(metatileBehavior: u8) {
    unsafe {
        let mut metatileBehavior = metatileBehavior;
        LockPlayerFieldControls();
        StartEscalatorWarp(metatileBehavior, 10u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DoLavaridgeGymB1FWarp() {
    unsafe {
        LockPlayerFieldControls();
        StartLavaridgeGymB1FWarp(10u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DoLavaridgeGym1FWarp() {
    unsafe {
        LockPlayerFieldControls();
        StartLavaridgeGym1FWarp(10u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DoTeleportTileWarp() {
    unsafe {
        LockPlayerFieldControls();
        TryFadeOutOldMapMusic();
        WarpFadeOutScreen();
        PlaySE(45u16);
        CreateTask(Some(Task_WarpAndLoadMap), 10u8);
        ((&raw mut gFieldCallback).cast::<Option<unsafe extern "C" fn()>>())
            .write(Some(FieldCB_SpinEnterWarp));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DoMossdeepGymWarp() {
    unsafe {
        SetObjectEventLoadFlag(1u8);
        LockPlayerFieldControls();
        SaveObjectEvents();
        TryFadeOutOldMapMusic();
        WarpFadeOutScreen();
        PlaySE(45u16);
        CreateTask(Some(Task_WarpAndLoadMap), 10u8);
        ((&raw mut gFieldCallback).cast::<Option<unsafe extern "C" fn()>>())
            .write(Some(FieldCB_MossdeepGymWarpExit));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DoPortholeWarp() {
    unsafe {
        LockPlayerFieldControls();
        WarpFadeOutScreen();
        CreateTask(Some(Task_WarpAndLoadMap), 10u8);
        ((&raw mut gFieldCallback).cast::<Option<unsafe extern "C" fn()>>())
            .write(Some(FieldCB_ShowPortholeView));
    }
}
pub(crate) unsafe extern "C" fn Task_DoCableClubWarp(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut task: *mut u8 =
            ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40);
        'l1: {
            let __sw1 = (((((task).wrapping_add(8)).cast::<i16>()).read()) as i32);
            if __sw1 == 0i32 {
                LockPlayerFieldControls();
                let __p2 = ((task).wrapping_add(8)).cast::<i16>();
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                if (!((PaletteFadeActive()) != 0)) && ((BGMusicStopped()) != 0) {
                    let __p3 = ((task).wrapping_add(8)).cast::<i16>();
                    (__p3).write(((__p3).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                WarpIntoMap();
                SetMainCallback2(Some(CB2_ReturnToFieldCableClub));
                DestroyTask(taskId);
                break 'l1;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DoCableClubWarp() {
    unsafe {
        LockPlayerFieldControls();
        TryFadeOutOldMapMusic();
        WarpFadeOutScreen();
        PlaySE(9u16);
        CreateTask(Some(Task_DoCableClubWarp), 10u8);
    }
}
pub(crate) unsafe extern "C" fn Task_ReturnToWorldFromLinkRoom(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        'l1: {
            let __sw1 = (((data).read()) as i32);
            if __sw1 == 0i32 {
                ClearLinkCallback_2();
                FadeScreen(1u8, 0i8);
                TryFadeOutOldMapMusic();
                PlaySE(9u16);
                (data).write(((data).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                if (!((PaletteFadeActive()) != 0)) && ((BGMusicStopped()) != 0) {
                    SetCloseLinkCallback();
                    (data).write(((data).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                if !((((&raw mut gReceivedRemoteLinkPlayers).cast::<u8>()).read()) != 0) {
                    WarpIntoMap();
                    SetMainCallback2(Some(CB2_LoadMap));
                    DestroyTask(taskId);
                }
                break 'l1;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ReturnFromLinkRoom() {
    unsafe {
        CreateTask(Some(Task_ReturnToWorldFromLinkRoom), 10u8);
    }
}
pub(crate) unsafe extern "C" fn Task_WarpAndLoadMap(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut task: *mut u8 =
            ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40);
        'l1: {
            let __sw1 = (((((task).wrapping_add(8)).cast::<i16>()).read()) as i32);
            if __sw1 == 0i32 {
                FreezeObjectEvents();
                LockPlayerFieldControls();
                let __p2 = ((task).wrapping_add(8)).cast::<i16>();
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                if !((PaletteFadeActive()) != 0) {
                    if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read())
                        as i32)
                        == 0i32
                    {
                        ClearMirageTowerPulseBlendEffect();
                        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(1i16);
                    }
                    if (BGMusicStopped()) != 0 {
                        let __p3 = ((task).wrapping_add(8)).cast::<i16>();
                        (__p3).write(((__p3).read()).wrapping_add(1));
                    }
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                WarpIntoMap();
                SetMainCallback2(Some(CB2_LoadMap));
                DestroyTask(taskId);
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_DoDoorWarp(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut task: *mut u8 =
            ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40);
        let mut x: *mut i16 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2);
        let mut y: *mut i16 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3);
        'l1: {
            let __sw1 = (((((task).wrapping_add(8)).cast::<i16>()).read()) as i32);
            if __sw1 == 0i32 {
                FreezeObjectEvents();
                PlayerGetDestCoords(x, y);
                PlaySE(
                    ((GetDoorSoundEffect(
                        (((x).read()) as u32),
                        (((((y).read()) as i32).wrapping_sub(1i32)) as u32),
                    )) as u16),
                );
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(
                    ((FieldAnimateDoorOpen(
                        (((x).read()) as u32),
                        (((((y).read()) as i32).wrapping_sub(1i32)) as u32),
                    )) as i16),
                );
                (((task).wrapping_add(8)).cast::<i16>()).write(1i16);
                break 'l1;
            }
            if __sw1 == 1i32 {
                if (((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
                    < 0i32)
                    || (((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
                        ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read())
                            as i32) as isize
                            * 40,
                    ))
                    .wrapping_add(4))
                    .read()) as i32)
                        != 1i32)
                {
                    let mut objEventId: u8 = 0u8;
                    objEventId = GetObjectEventIdByLocalIdAndMap(255u8, 0u8, 0u8);
                    ObjectEventClearHeldMovementIfActive(
                        ((&raw mut gObjectEvents).cast::<u8>())
                            .wrapping_offset(((objEventId) as i32) as isize * 36),
                    );
                    objEventId = GetObjectEventIdByLocalIdAndMap(255u8, 0u8, 0u8);
                    ObjectEventSetHeldMovement(
                        ((&raw mut gObjectEvents).cast::<u8>())
                            .wrapping_offset(((objEventId) as i32) as isize * 36),
                        9u8,
                    );
                    (((task).wrapping_add(8)).cast::<i16>()).write(2i16);
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                if (IsPlayerStandingStill()) != 0 {
                    let mut objEventId: u8 = 0u8;
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(
                        ((FieldAnimateDoorClose(
                            (((x).read()) as u32),
                            (((((y).read()) as i32).wrapping_sub(1i32)) as u32),
                        )) as i16),
                    );
                    objEventId = GetObjectEventIdByLocalIdAndMap(255u8, 0u8, 0u8);
                    ObjectEventClearHeldMovementIfFinished(
                        ((&raw mut gObjectEvents).cast::<u8>())
                            .wrapping_offset(((objEventId) as i32) as isize * 36),
                    );
                    SetPlayerVisibility(0u8);
                    (((task).wrapping_add(8)).cast::<i16>()).write(3i16);
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                if (((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
                    < 0i32)
                    || (((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
                        ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read())
                            as i32) as isize
                            * 40,
                    ))
                    .wrapping_add(4))
                    .read()) as i32)
                        != 1i32)
                {
                    (((task).wrapping_add(8)).cast::<i16>()).write(4i16);
                }
                break 'l1;
            }
            if __sw1 == 4i32 {
                TryFadeOutOldMapMusic();
                WarpFadeOutScreen();
                PlayRainStoppingSoundEffect();
                (((task).wrapping_add(8)).cast::<i16>()).write(0i16);
                ((task).cast::<Option<unsafe extern "C" fn(u8)>>())
                    .write(Some(Task_WarpAndLoadMap));
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_DoContestHallWarp(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut task: *mut u8 =
            ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40);
        'l1: {
            let __sw1 = (((((task).wrapping_add(8)).cast::<i16>()).read()) as i32);
            if __sw1 == 0i32 {
                FreezeObjectEvents();
                LockPlayerFieldControls();
                let __p2 = ((task).wrapping_add(8)).cast::<i16>();
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                if (!((PaletteFadeActive()) != 0)) && ((BGMusicStopped()) != 0) {
                    let __p3 = ((task).wrapping_add(8)).cast::<i16>();
                    (__p3).write(((__p3).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                WarpIntoMap();
                SetMainCallback2(Some(CB2_ReturnToFieldContestHall));
                DestroyTask(taskId);
                break 'l1;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DoContestHallWarp() {
    unsafe {
        LockPlayerFieldControls();
        TryFadeOutOldMapMusic();
        WarpFadeOutScreen();
        PlayRainStoppingSoundEffect();
        PlaySE(9u16);
        ((&raw mut gFieldCallback).cast::<Option<unsafe extern "C" fn()>>())
            .write(Some(FieldCB_WarpExitFadeFromBlack));
        CreateTask(Some(Task_DoContestHallWarp), 10u8);
    }
}
pub(crate) unsafe extern "C" fn SetFlashScanlineEffectWindowBoundary(
    dest: *mut u16,
    y: u32,
    left: i32,
    right: i32,
) {
    unsafe {
        let mut dest = dest;
        let mut y = y;
        let mut left = left;
        let mut right = right;
        if y <= 160u32 {
            if left < 0i32 {
                left = 0i32;
            }
            if left > 255i32 {
                left = 255i32;
            }
            if right < 0i32 {
                right = 0i32;
            }
            if right > 255i32 {
                right = 255i32;
            }
            ((dest).wrapping_offset(((y) as i32) as isize)).write((((left << 8) | right) as u16));
        }
    }
}
pub(crate) unsafe extern "C" fn SetFlashScanlineEffectWindowBoundaries(
    dest: *mut u16,
    centerX: i32,
    centerY: i32,
    radius: i32,
) {
    unsafe {
        let mut dest = dest;
        let mut centerX = centerX;
        let mut centerY = centerY;
        let mut radius = radius;
        let mut r: i32 = radius;
        let mut v2: i32 = radius;
        let mut v3: i32 = 0i32;
        'l1: loop {
            if !(r >= v3) {
                break 'l1;
            }
            SetFlashScanlineEffectWindowBoundary(
                dest,
                (((centerY).wrapping_sub(v3)) as u32),
                (centerX).wrapping_sub(r),
                (centerX).wrapping_add(r),
            );
            SetFlashScanlineEffectWindowBoundary(
                dest,
                (((centerY).wrapping_add(v3)) as u32),
                (centerX).wrapping_sub(r),
                (centerX).wrapping_add(r),
            );
            SetFlashScanlineEffectWindowBoundary(
                dest,
                (((centerY).wrapping_sub(r)) as u32),
                (centerX).wrapping_sub(v3),
                (centerX).wrapping_add(v3),
            );
            SetFlashScanlineEffectWindowBoundary(
                dest,
                (((centerY).wrapping_add(r)) as u32),
                (centerX).wrapping_sub(v3),
                (centerX).wrapping_add(v3),
            );
            v2 = (v2).wrapping_sub(((v3).wrapping_mul(2i32)).wrapping_sub(1i32));
            v3 = (v3).wrapping_add(1);
            if v2 < 0i32 {
                v2 = (v2).wrapping_add((2i32).wrapping_mul((r).wrapping_sub(1i32)));
                r = (r).wrapping_sub(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SetOrbFlashScanlineEffectWindowBoundary(
    dest: *mut u16,
    y: u32,
    left: i32,
    right: i32,
) {
    unsafe {
        let mut dest = dest;
        let mut y = y;
        let mut left = left;
        let mut right = right;
        if y <= 160u32 {
            if left < 0i32 {
                left = 0i32;
            }
            if left > 240i32 {
                left = 240i32;
            }
            if right < 0i32 {
                right = 0i32;
            }
            if right > 240i32 {
                right = 240i32;
            }
            ((dest).wrapping_offset(((y) as i32) as isize)).write((((left << 8) | right) as u16));
        }
    }
}
pub(crate) unsafe extern "C" fn SetOrbFlashScanlineEffectWindowBoundaries(
    dest: *mut u16,
    centerX: i32,
    centerY: i32,
    radius: i32,
) {
    unsafe {
        let mut dest = dest;
        let mut centerX = centerX;
        let mut centerY = centerY;
        let mut radius = radius;
        let mut r: i32 = radius;
        let mut v2: i32 = radius;
        let mut v3: i32 = 0i32;
        'l1: loop {
            if !(r >= v3) {
                break 'l1;
            }
            SetOrbFlashScanlineEffectWindowBoundary(
                dest,
                (((centerY).wrapping_sub(v3)) as u32),
                (centerX).wrapping_sub(r),
                (centerX).wrapping_add(r),
            );
            SetOrbFlashScanlineEffectWindowBoundary(
                dest,
                (((centerY).wrapping_add(v3)) as u32),
                (centerX).wrapping_sub(r),
                (centerX).wrapping_add(r),
            );
            SetOrbFlashScanlineEffectWindowBoundary(
                dest,
                (((centerY).wrapping_sub(r)) as u32),
                (centerX).wrapping_sub(v3),
                (centerX).wrapping_add(v3),
            );
            SetOrbFlashScanlineEffectWindowBoundary(
                dest,
                (((centerY).wrapping_add(r)) as u32),
                (centerX).wrapping_sub(v3),
                (centerX).wrapping_add(v3),
            );
            v2 = (v2).wrapping_sub(((v3).wrapping_mul(2i32)).wrapping_sub(1i32));
            v3 = (v3).wrapping_add(1);
            if v2 < 0i32 {
                v2 = (v2).wrapping_add((2i32).wrapping_mul((r).wrapping_sub(1i32)));
                r = (r).wrapping_sub(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn UpdateFlashLevelEffect(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        'l1: {
            let __sw1 = (((data).read()) as i32);
            if __sw1 == 0i32 {
                SetFlashScanlineEffectWindowBoundaries(
                    (((&raw mut gScanlineEffectRegBuffers).cast::<u8>()).wrapping_offset(
                        (((((&raw mut gScanlineEffect).cast::<u8>()).wrapping_add(20)).read())
                            as i32) as isize
                            * 1920,
                    ))
                    .cast::<u16>(),
                    ((((data).wrapping_offset(1)).read()) as i32),
                    ((((data).wrapping_offset(2)).read()) as i32),
                    ((((data).wrapping_offset(3)).read()) as i32),
                );
                (data).write(1i16);
                break 'l1;
            }
            if __sw1 == 1i32 {
                SetFlashScanlineEffectWindowBoundaries(
                    (((&raw mut gScanlineEffectRegBuffers).cast::<u8>()).wrapping_offset(
                        (((((&raw mut gScanlineEffect).cast::<u8>()).wrapping_add(20)).read())
                            as i32) as isize
                            * 1920,
                    ))
                    .cast::<u16>(),
                    ((((data).wrapping_offset(1)).read()) as i32),
                    ((((data).wrapping_offset(2)).read()) as i32),
                    ((((data).wrapping_offset(3)).read()) as i32),
                );
                (data).write(0i16);
                let __p2 = (data).wrapping_offset(3);
                (__p2).write(
                    (((((__p2).read()) as i32)
                        .wrapping_add(((((data).wrapping_offset(5)).read()) as i32)))
                        as i16),
                );
                if ((((data).wrapping_offset(3)).read()) as i32)
                    > ((((data).wrapping_offset(4)).read()) as i32)
                {
                    if ((((data).wrapping_offset(6)).read()) as i32) == 1i32 {
                        ScanlineEffect_Stop();
                        (data).write(2i16);
                    } else {
                        DestroyTask(taskId);
                    }
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                ScanlineEffect_Clear();
                DestroyTask(taskId);
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn UpdateOrbFlashEffect(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        'l1: {
            let __sw1 = (((data).read()) as i32);
            if __sw1 == 0i32 {
                SetOrbFlashScanlineEffectWindowBoundaries(
                    (((&raw mut gScanlineEffectRegBuffers).cast::<u8>()).wrapping_offset(
                        (((((&raw mut gScanlineEffect).cast::<u8>()).wrapping_add(20)).read())
                            as i32) as isize
                            * 1920,
                    ))
                    .cast::<u16>(),
                    ((((data).wrapping_offset(1)).read()) as i32),
                    ((((data).wrapping_offset(2)).read()) as i32),
                    ((((data).wrapping_offset(3)).read()) as i32),
                );
                (data).write(1i16);
                break 'l1;
            }
            if __sw1 == 1i32 {
                SetOrbFlashScanlineEffectWindowBoundaries(
                    (((&raw mut gScanlineEffectRegBuffers).cast::<u8>()).wrapping_offset(
                        (((((&raw mut gScanlineEffect).cast::<u8>()).wrapping_add(20)).read())
                            as i32) as isize
                            * 1920,
                    ))
                    .cast::<u16>(),
                    ((((data).wrapping_offset(1)).read()) as i32),
                    ((((data).wrapping_offset(2)).read()) as i32),
                    ((((data).wrapping_offset(3)).read()) as i32),
                );
                (data).write(0i16);
                let __p2 = (data).wrapping_offset(3);
                (__p2).write(
                    (((((__p2).read()) as i32)
                        .wrapping_add(((((data).wrapping_offset(5)).read()) as i32)))
                        as i16),
                );
                if ((((data).wrapping_offset(3)).read()) as i32)
                    > ((((data).wrapping_offset(4)).read()) as i32)
                {
                    if ((((data).wrapping_offset(6)).read()) as i32) == 1i32 {
                        ScanlineEffect_Stop();
                        (data).write(2i16);
                    } else {
                        DestroyTask(taskId);
                    }
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                ScanlineEffect_Clear();
                DestroyTask(taskId);
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_WaitForFlashUpdate(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if !((FuncIsActiveTask(Some(UpdateFlashLevelEffect))) != 0) {
            ScriptContext_Enable();
            DestroyTask(taskId);
        }
    }
}
pub(crate) unsafe extern "C" fn StartWaitForFlashUpdate() {
    unsafe {
        if !((FuncIsActiveTask(Some(Task_WaitForFlashUpdate))) != 0) {
            CreateTask(Some(Task_WaitForFlashUpdate), 80u8);
        }
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
    unsafe {
        let mut centerX = centerX;
        let mut centerY = centerY;
        let mut initialFlashRadius = initialFlashRadius;
        let mut destFlashRadius = destFlashRadius;
        let mut clearScanlineEffect = clearScanlineEffect;
        let mut delta = delta;
        let mut taskId: u8 = CreateTask(Some(UpdateFlashLevelEffect), 80u8);
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        ((data).wrapping_offset(3)).write(((initialFlashRadius) as i16));
        ((data).wrapping_offset(4)).write(((destFlashRadius) as i16));
        ((data).wrapping_offset(1)).write(((centerX) as i16));
        ((data).wrapping_offset(2)).write(((centerY) as i16));
        ((data).wrapping_offset(6)).write(((clearScanlineEffect) as i16));
        if initialFlashRadius < destFlashRadius {
            ((data).wrapping_offset(5)).write(((delta) as i16));
        } else {
            ((data).wrapping_offset(5)).write(((((delta) as i32).wrapping_neg()) as i16));
        }
        return taskId;
    }
}
pub(crate) unsafe extern "C" fn StartUpdateOrbFlashEffect(
    centerX: i32,
    centerY: i32,
    initialFlashRadius: i32,
    destFlashRadius: i32,
    clearScanlineEffect: i32,
    delta: u8,
) -> u8 {
    unsafe {
        let mut centerX = centerX;
        let mut centerY = centerY;
        let mut initialFlashRadius = initialFlashRadius;
        let mut destFlashRadius = destFlashRadius;
        let mut clearScanlineEffect = clearScanlineEffect;
        let mut delta = delta;
        let mut taskId: u8 = CreateTask(Some(UpdateOrbFlashEffect), 80u8);
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        ((data).wrapping_offset(3)).write(((initialFlashRadius) as i16));
        ((data).wrapping_offset(4)).write(((destFlashRadius) as i16));
        ((data).wrapping_offset(1)).write(((centerX) as i16));
        ((data).wrapping_offset(2)).write(((centerY) as i16));
        ((data).wrapping_offset(6)).write(((clearScanlineEffect) as i16));
        if initialFlashRadius < destFlashRadius {
            ((data).wrapping_offset(5)).write(((delta) as i16));
        } else {
            ((data).wrapping_offset(5)).write(((((delta) as i32).wrapping_neg()) as i16));
        }
        return taskId;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimateFlash(newFlashLevel: u8) {
    unsafe {
        let mut newFlashLevel = newFlashLevel;
        let mut curFlashLevel: u8 = GetFlashLevel();
        let mut fullBrightness: u8 = 0u8;
        if ((newFlashLevel) as i32) == 0i32 {
            fullBrightness = 1u8;
        }
        StartUpdateFlashLevelEffect(
            crate::c::div_i32(240i32, 2i32),
            crate::c::div_i32(160i32, 2i32),
            ((((((&raw const sFlashLevelToRadius)
                .cast::<u8>()
                .cast_mut()
                .cast::<u16>())
            .cast::<u16>())
            .wrapping_offset(((curFlashLevel) as i32) as isize))
            .read()) as i32),
            ((((((&raw const sFlashLevelToRadius)
                .cast::<u8>()
                .cast_mut()
                .cast::<u16>())
            .cast::<u16>())
            .wrapping_offset(((newFlashLevel) as i32) as isize))
            .read()) as i32),
            ((fullBrightness) as i32),
            1u8,
        );
        StartWaitForFlashUpdate();
        LockPlayerFieldControls();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn WriteFlashScanlineEffectBuffer(flashLevel: u8) {
    unsafe {
        let mut flashLevel = flashLevel;
        if (flashLevel) != 0 {
            SetFlashScanlineEffectWindowBoundaries(
                ((&raw mut gScanlineEffectRegBuffers).cast::<u8>()).cast::<u16>(),
                crate::c::div_i32(240i32, 2i32),
                crate::c::div_i32(160i32, 2i32),
                ((((((&raw const sFlashLevelToRadius)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<u16>())
                .cast::<u16>())
                .wrapping_offset(((flashLevel) as i32) as isize))
                .read()) as i32),
            );
            'l1: loop {
                'l2: {
                    CpuFastSet(
                        (((&raw mut gScanlineEffectRegBuffers).cast::<u8>()).cast::<u16>())
                            .cast::<u8>(),
                        ((((&raw mut gScanlineEffectRegBuffers).cast::<u8>())
                            .wrapping_offset(1920))
                        .cast::<u16>())
                        .cast::<u8>(),
                        480u32,
                    );
                }
                if !((0i32) != 0) {
                    break 'l1;
                }
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn WriteBattlePyramidViewScanlineEffectBuffer() {
    unsafe {
        SetFlashScanlineEffectWindowBoundaries(
            ((&raw mut gScanlineEffectRegBuffers).cast::<u8>()).cast::<u16>(),
            crate::c::div_i32(240i32, 2i32),
            crate::c::div_i32(160i32, 2i32),
            (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(2076))
            .read()) as i32),
        );
        'l1: loop {
            'l2: {
                CpuFastSet(
                    (((&raw mut gScanlineEffectRegBuffers).cast::<u8>()).cast::<u16>())
                        .cast::<u8>(),
                    ((((&raw mut gScanlineEffectRegBuffers).cast::<u8>()).wrapping_offset(1920))
                        .cast::<u16>())
                    .cast::<u8>(),
                    480u32,
                );
            }
            if !((0i32) != 0) {
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_SpinEnterWarp(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        'l1: {
            let __sw1 = (((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .read()) as i32);
            if __sw1 == 0i32 {
                FreezeObjectEvents();
                LockPlayerFieldControls();
                DoPlayerSpinEntrance();
                let __p2 = ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>();
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                if ((WaitForWeatherFadeIn()) != 0) && (IsPlayerSpinEntranceActive() != 1u32) {
                    UnfreezeObjectEvents();
                    UnlockPlayerFieldControls();
                    DestroyTask(taskId);
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_SpinExitWarp(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut task: *mut u8 =
            ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40);
        'l1: {
            let __sw1 = (((((task).wrapping_add(8)).cast::<i16>()).read()) as i32);
            if __sw1 == 0i32 {
                FreezeObjectEvents();
                LockPlayerFieldControls();
                PlaySE(45u16);
                DoPlayerSpinExit();
                let __p2 = ((task).wrapping_add(8)).cast::<i16>();
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                if !((IsPlayerSpinExitActive()) != 0) {
                    WarpFadeOutScreen();
                    let __p3 = ((task).wrapping_add(8)).cast::<i16>();
                    (__p3).write(((__p3).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                if (!((PaletteFadeActive()) != 0)) && ((BGMusicStopped()) != 0) {
                    let __p4 = ((task).wrapping_add(8)).cast::<i16>();
                    (__p4).write(((__p4).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                WarpIntoMap();
                SetMainCallback2(Some(CB2_LoadMap));
                DestroyTask(taskId);
                break 'l1;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DoSpinEnterWarp() {
    unsafe {
        LockPlayerFieldControls();
        CreateTask(Some(Task_WarpAndLoadMap), 10u8);
        ((&raw mut gFieldCallback).cast::<Option<unsafe extern "C" fn()>>())
            .write(Some(FieldCB_SpinEnterWarp));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DoSpinExitWarp() {
    unsafe {
        LockPlayerFieldControls();
        ((&raw mut gFieldCallback).cast::<Option<unsafe extern "C" fn()>>())
            .write(Some(FieldCB_DefaultWarpExit));
        CreateTask(Some(Task_SpinExitWarp), 10u8);
    }
}
pub(crate) unsafe extern "C" fn LoadOrbEffectPalette(blueOrb: u8) {
    unsafe {
        let mut blueOrb = blueOrb;
        let mut i: i32 = 0i32;
        let mut color = crate::ffi::Align4([0u8; 2]);
        if !((blueOrb) != 0) {
            ((&raw mut color).cast::<u16>()).write(31u16);
        } else {
            ((&raw mut color).cast::<u16>()).write(31744u16);
        }
        {
            i = 0i32;
            'l1: loop {
                if !(i < 16i32) {
                    break 'l1;
                }
                'l2: {
                    LoadPalette(
                        ((&raw mut color).cast::<u16>()).cast::<u8>(),
                        (((240i32).wrapping_add(i)) as u16),
                        2u16,
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn UpdateOrbEffectBlend(shakeDir: u16) -> u8 {
    unsafe {
        let mut shakeDir = shakeDir;
        let mut lo: u8 =
            ((((((67108946i32) as usize as *mut u16).read_volatile()) as i32) & 255i32) as u8);
        let mut hi: u8 =
            ((((((67108946i32) as usize as *mut u16).read_volatile()) as i32) >> 8) as u8);
        if ((shakeDir) as i32) != 0i32 {
            if (lo) != 0 {
                lo = (lo).wrapping_sub(1);
            }
        } else {
            if ((hi) as i32) < 16i32 {
                hi = (hi).wrapping_add(1);
            }
        }
        SetGpuReg(82u8, (((((hi) as i32) << 8) | ((lo) as i32)) as u16));
        if (((lo) as i32) == 0i32) && (((hi) as i32) == 16i32) {
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
pub(crate) unsafe extern "C" fn Task_OrbEffect(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        'l1: {
            let __sw1 = (((data).read()) as i32);
            if __sw1 == 0i32 {
                ((data).wrapping_offset(6))
                    .write(((((67108864i32) as usize as *mut u16).read_volatile()) as i16));
                ((data).wrapping_offset(7))
                    .write(((((67108944i32) as usize as *mut u16).read_volatile()) as i16));
                ((data).wrapping_offset(8))
                    .write(((((67108946i32) as usize as *mut u16).read_volatile()) as i16));
                ((data).wrapping_offset(9))
                    .write(((((67108936i32) as usize as *mut u16).read_volatile()) as i16));
                ((data).wrapping_offset(10))
                    .write(((((67108938i32) as usize as *mut u16).read_volatile()) as i16));
                ClearGpuRegBits(0u8, 16384u16);
                SetGpuRegBits(
                    80u8,
                    (((&raw mut gOrbEffectBackgroundLayerFlags).cast::<u16>()).cast::<u16>())
                        .read(),
                );
                SetGpuReg(82u8, 1804u16);
                SetGpuReg(72u8, 63u16);
                SetGpuReg(74u8, 30u16);
                SetBgTilemapPalette(
                    0u8,
                    0u8,
                    0u8,
                    ((crate::c::div_i32(240i32, 8i32)) as u8),
                    ((crate::c::div_i32(160i32, 8i32)) as u8),
                    15u8,
                );
                ScheduleBgCopyTilemapToVram(0u8);
                SetOrbFlashScanlineEffectWindowBoundaries(
                    ((&raw mut gScanlineEffectRegBuffers).cast::<u8>()).cast::<u16>(),
                    ((((data).wrapping_offset(2)).read()) as i32),
                    ((((data).wrapping_offset(3)).read()) as i32),
                    1i32,
                );
                'l2: loop {
                    'l3: {
                        CpuFastSet(
                            (((&raw mut gScanlineEffectRegBuffers).cast::<u8>()).cast::<u16>())
                                .cast::<u8>(),
                            ((((&raw mut gScanlineEffectRegBuffers).cast::<u8>())
                                .wrapping_offset(1920))
                            .cast::<u16>())
                            .cast::<u8>(),
                            480u32,
                        );
                    }
                    if !((0i32) != 0) {
                        break 'l2;
                    }
                }
                ScanlineEffect_SetParams(
                    (&raw const sFlashEffectParams)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<crate::c::Rec4<12>>()
                        .read_unaligned(),
                );
                (data).write(1i16);
                break 'l1;
            }
            if __sw1 == 1i32 {
                BgDmaFill(0u32, 17u8, 0i32, 1i32);
                LoadOrbEffectPalette(((((data).wrapping_offset(1)).read()) as u8));
                StartUpdateOrbFlashEffect(
                    ((((data).wrapping_offset(2)).read()) as i32),
                    ((((data).wrapping_offset(3)).read()) as i32),
                    1i32,
                    160i32,
                    1i32,
                    2u8,
                );
                (data).write(2i16);
                break 'l1;
            }
            if __sw1 == 2i32 {
                if !((FuncIsActiveTask(Some(UpdateOrbFlashEffect))) != 0) {
                    ScriptContext_Enable();
                    (data).write(3i16);
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                InstallCameraPanAheadCallback();
                SetCameraPanningCallback(None);
                ((data).wrapping_offset(5)).write(0i16);
                ((data).wrapping_offset(4)).write(4i16);
                (data).write(4i16);
                break 'l1;
            }
            if __sw1 == 4i32 {
                if (({
                    let __p2 = (data).wrapping_offset(4);
                    let __t3 = ((__p2).read()).wrapping_sub(1);
                    (__p2).write(__t3);
                    __t3
                }) as i32)
                    == 0i32
                {
                    let mut panning: i32 = 0i32;
                    ((data).wrapping_offset(4)).write(4i16);
                    let __p4 = (data).wrapping_offset(5);
                    (__p4).write((((((__p4).read()) as i32) ^ 1i32) as i16));
                    if (((data).wrapping_offset(5)).read()) != 0 {
                        panning = 4i32;
                    } else {
                        panning = (-4i32);
                    }
                    SetCameraPanning(0i16, ((panning) as i16));
                }
                break 'l1;
            }
            if __sw1 == 6i32 {
                InstallCameraPanAheadCallback();
                ((data).wrapping_offset(4)).write(8i16);
                (data).write(7i16);
                break 'l1;
            }
            if __sw1 == 7i32 {
                if (({
                    let __p5 = (data).wrapping_offset(4);
                    let __t6 = ((__p5).read()).wrapping_sub(1);
                    (__p5).write(__t6);
                    __t6
                }) as i32)
                    == 0i32
                {
                    ((data).wrapping_offset(4)).write(8i16);
                    let __p7 = (data).wrapping_offset(5);
                    (__p7).write((((((__p7).read()) as i32) ^ 1i32) as i16));
                    if ((UpdateOrbEffectBlend(((((data).wrapping_offset(5)).read()) as u16)))
                        as i32)
                        == 1i32
                    {
                        (data).write(5i16);
                        BgDmaFill(0u32, 0u8, 0i32, 1i32);
                    }
                }
                break 'l1;
            }
            if __sw1 == 5i32 {
                SetGpuReg(64u8, 255u16);
                SetGpuReg(0u8, ((((data).wrapping_offset(6)).read()) as u16));
                SetGpuReg(80u8, ((((data).wrapping_offset(7)).read()) as u16));
                SetGpuReg(82u8, ((((data).wrapping_offset(8)).read()) as u16));
                SetGpuReg(72u8, ((((data).wrapping_offset(9)).read()) as u16));
                SetGpuReg(74u8, ((((data).wrapping_offset(10)).read()) as u16));
                ScriptContext_Enable();
                DestroyTask(taskId);
                break 'l1;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DoOrbEffect() {
    unsafe {
        let mut taskId: u8 = CreateTask(Some(Task_OrbEffect), 80u8);
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        if ((((&raw mut gSpecialVar_Result).cast::<u16>()).read()) as i32) == 0i32 {
            ((data).wrapping_offset(1)).write(0i16);
            ((data).wrapping_offset(2)).write(104i16);
        } else {
            if ((((&raw mut gSpecialVar_Result).cast::<u16>()).read()) as i32) == 1i32 {
                ((data).wrapping_offset(1)).write(1i16);
                ((data).wrapping_offset(2)).write(136i16);
            } else {
                if ((((&raw mut gSpecialVar_Result).cast::<u16>()).read()) as i32) == 2i32 {
                    ((data).wrapping_offset(1)).write(0i16);
                    ((data).wrapping_offset(2)).write(120i16);
                } else {
                    ((data).wrapping_offset(1)).write(1i16);
                    ((data).wrapping_offset(2)).write(120i16);
                }
            }
        }
        ((data).wrapping_offset(3)).write(80i16);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FadeOutOrbEffect() {
    unsafe {
        let mut taskId: u8 = FindTaskIdByFunc(Some(Task_OrbEffect));
        (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .write(6i16);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Script_FadeOutMapMusic() {
    unsafe {
        Overworld_FadeOutMapMusic();
        CreateTask(Some(Task_EnableScriptAfterMusicFade), 80u8);
    }
}
pub(crate) unsafe extern "C" fn Task_EnableScriptAfterMusicFade(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if ((BGMusicStopped()) as i32) == 1i32 {
            DestroyTask(taskId);
            ScriptContext_Enable();
        }
    }
}
