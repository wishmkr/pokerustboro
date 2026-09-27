//! Translated from `src/cable_club.c` by tools/rustport/c2rs.py, then reviewed.
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

// Data tables (translate with cdata.py): sWindowTemplate_LinkPlayerCount sTrainerCardColorNames
#[allow(unused_imports)]
use crate::data::cable_club::*;

unsafe extern "C" {
    static mut gBattleOutcome: u8;
    static mut gBattleTypeFlags: u8;
    static mut gBlockRecvBuffer: u8;
    static mut gBlockSendBuffer: u8;
    static mut gFieldLinkPlayerCount: u8;
    static mut gLinkPlayers: u8;
    static mut gLinkType: u8;
    static mut gLocalLinkPlayer: u8;
    static mut gLocalLinkPlayerId: u8;
    static mut gMain: u8;
    static mut gPaletteFade: u8;
    static mut gPlayerParty: u8;
    static mut gReceivedRemoteLinkPlayers: u8;
    static mut gSaveBlock2Ptr: u8;
    static mut gSelectedOrderFromParty: u8;
    static mut gSelectedTradeMonPositions: u8;
    static mut gSpecialVar_0x8004: u8;
    static mut gSpecialVar_0x8005: u8;
    static mut gSpecialVar_0x8006: u8;
    static mut gSpecialVar_Result: u8;
    static mut gSpeciesNames: u8;
    static mut gStringVar1: u8;
    static mut gStringVar2: u8;
    static mut gStringVar4: u8;
    static mut gTasks: u8;
    static mut gText_AwaitingLinkup: u8;
    static mut gText_ConfirmLinkWhenPlayersReady: u8;
    static mut gText_ConfirmStartLinkWithXPlayers: u8;
    static mut gText_NumPlayerLink: u8;
    static mut gText_PleaseWaitForLink: u8;
    static mut gTrainerBattleOpponent_A: u8;
    static mut gTrainerCards: u8;
    static mut gWirelessCommType: u8;
    fn AddTextPrinterParameterized(
        a0: u8,
        a1: u8,
        a2: *mut u8,
        a3: u8,
        a4: u8,
        a5: u8,
        a6: Option<unsafe extern "C" fn(*mut u8, u16)>,
    ) -> u16;
    fn AddWindow(a0: *mut u8) -> u16;
    fn CB2_InitBattle();
    fn CB2_LinkError();
    fn CB2_ReturnToField();
    fn CB2_ReturnToFieldContinueScriptPlayMapMusic();
    fn CB2_ReturnToFieldFromMultiplayer();
    fn CB2_SetUpSaveAfterLinkBattle();
    fn CB2_StartCreateTradeMenu();
    fn CheckLinkPlayersMatchSaved();
    fn CheckShouldAdvanceLinkState();
    fn CleanupOverworldWindowsAndTilemaps();
    fn ClearLinkCallback_2();
    fn ClearLinkRfuCallback();
    fn ClearStdWindowAndFrame(a0: u8, a1: u8);
    fn CloseLink();
    fn ConvertIntToDecimalStringN(a0: *mut u8, a1: i32, a2: i32, a3: u8) -> *mut u8;
    fn ConvertLinkPlayerName(a0: *mut u8);
    fn CopyTrainerCardData(a0: *mut u8, a1: *mut u8, a2: u8);
    fn CopyWindowToVram(a0: u8, a1: u8);
    fn CreateTask(a0: Option<unsafe extern "C" fn(u8)>, a1: u8) -> u8;
    fn CreateTask_CreateTradeMenu() -> u8;
    fn DestroyTask(a0: u8);
    fn DoesLinkPlayerCountMatchSaved() -> u8;
    fn EraseFieldMessageBox(a0: u8);
    fn FadeScreen(a0: u8, a1: i8);
    fn FindTaskIdByFunc(a0: Option<unsafe extern "C" fn(u8)>) -> u8;
    fn FuncIsActiveTask(a0: Option<unsafe extern "C" fn(u8)>) -> u8;
    fn GetBlockReceivedStatus() -> u8;
    fn GetCableClubPartnersReady() -> u32;
    fn GetFieldMessageBoxMode() -> u8;
    fn GetLinkPlayerCount() -> u8;
    fn GetLinkPlayerCountAsBitFlags() -> u8;
    fn GetLinkPlayerCount_2() -> u8;
    fn GetLinkPlayerDataExchangeStatusTimed(a0: i32, a1: i32) -> u8;
    fn GetMonData3(a0: *mut u8, a1: i32, a2: *mut u8) -> u32;
    fn GetMultiplayerId() -> u8;
    fn GetSavedLinkPlayerCountAsBitFlags() -> u8;
    fn GetSavedPlayerCount() -> u8;
    fn GetSioMultiSI() -> u8;
    fn GetStringCenterAlignXOffset(a0: i32, a1: *mut u8, a2: i32) -> i32;
    fn GetTrainerCardStars(a0: u8) -> u8;
    fn HasLinkErrorOccurred() -> u8;
    fn HideFieldMessageBox();
    fn InUnionRoom() -> u32;
    fn IsFieldMessageBoxHidden() -> u8;
    fn IsLinkConnectionEstablished() -> u8;
    fn IsLinkMaster() -> u8;
    fn IsLinkPlayerDataExchangeComplete() -> u8;
    fn IsLinkTaskFinished() -> u8;
    fn Link_AnyPartnersPlayingRubyOrSapphire() -> u32;
    fn LoadPlayerParty();
    fn LockPlayerFieldControls();
    fn MysteryGift_TryIncrementStat(a0: u32, a1: u32);
    fn OpenLink();
    fn OpenLinkTimed();
    fn Overworld_ResetMapMusic();
    fn PlayMapChosenOrBattleBGM(a0: u16);
    fn PlaySE(a0: u16);
    fn QueueExitLinkRoomKey() -> u16;
    fn ReducePlayerPartyToSelectedMons();
    fn RemoveWindow(a0: u8);
    fn ResetBlockReceivedFlag(a0: u8);
    fn ResetBlockReceivedFlags();
    fn ResetLinkPlayerCount();
    fn ResetLinkPlayers();
    fn RunTasks();
    fn SaveGame();
    fn SaveLinkPlayers(a0: u8);
    fn SavePlayerBag();
    fn ScriptContext_Enable();
    fn ScriptContext_Stop();
    fn SendBlock(a0: u8, a1: *mut u8, a2: u16) -> u8;
    fn SendBlockRequest(a0: u8) -> u8;
    fn SetCloseLinkCallback();
    fn SetCloseLinkCallbackHandleJP();
    fn SetInCableClubSeat() -> u16;
    fn SetLinkStandbyCallback();
    fn SetLinkWaitingForScript() -> u16;
    fn SetLocalLinkPlayerId(a0: u8);
    fn SetMainCallback2(a0: Option<unsafe extern "C" fn()>);
    fn SetStandardWindowBorderStyle(a0: u8, a1: u8);
    fn SetStartedCableClubActivity() -> u16;
    fn SetSuppressLinkErrorMessage(a0: u8);
    fn SetTaskFuncWithFollowupFunc(
        a0: u8,
        a1: Option<unsafe extern "C" fn(u8)>,
        a2: Option<unsafe extern "C" fn(u8)>,
    );
    fn SetWarpDestinationToDynamicWarp(a0: u8);
    fn ShowFieldAutoScrollMessage(a0: *mut u8) -> u8;
    fn ShowFieldMessage(a0: *mut u8) -> u8;
    fn ShowTrainerCardInLink(a0: u8, a1: Option<unsafe extern "C" fn()>);
    fn StartSendingKeysToLink();
    fn StopFieldMessage();
    fn StringCopy(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn StringExpandPlaceholders(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn SwitchTaskToFollowupFunc(a0: u8);
    fn TrainerCard_GenerateCardForLinkPlayer(a0: *mut u8);
    fn UpdatePlayerLinkBattleRecords(a0: i32);
    fn UpdateTrainerFansAfterLinkBattle();
    fn m4aMPlayAllStop();
}

pub(crate) unsafe extern "C" fn CreateLinkupTask(minPlayers: u8, maxPlayers: u8) {
    unsafe {
        let mut minPlayers = minPlayers;
        let mut maxPlayers = maxPlayers;
        if ((FindTaskIdByFunc(Some(Task_LinkupStart))) as i32) == 255i32 {
            let mut taskId1: u8 = 0u8;
            taskId1 = CreateTask(Some(Task_LinkupStart), 80u8);
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId1) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(1))
            .write(((minPlayers) as i16));
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId1) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(2))
            .write(((maxPlayers) as i16));
        }
    }
}
pub(crate) unsafe extern "C" fn PrintNumPlayersInLink(windowId: u16, numPlayers: u32) {
    unsafe {
        let mut windowId = windowId;
        let mut numPlayers = numPlayers;
        let mut xPos: u8 = 0u8;
        ConvertIntToDecimalStringN(
            (&raw mut gStringVar1).cast::<u8>(),
            ((numPlayers) as i32),
            0i32,
            1u8,
        );
        SetStandardWindowBorderStyle(((windowId) as u8), 0u8);
        StringExpandPlaceholders(
            (&raw mut gStringVar4).cast::<u8>(),
            (&raw mut gText_NumPlayerLink).cast::<u8>(),
        );
        xPos =
            ((GetStringCenterAlignXOffset(1i32, (&raw mut gStringVar4).cast::<u8>(), 88i32)) as u8);
        AddTextPrinterParameterized(
            ((windowId) as u8),
            1u8,
            (&raw mut gStringVar4).cast::<u8>(),
            xPos,
            1u8,
            255u8,
            None,
        );
        CopyWindowToVram(((windowId) as u8), 3u8);
    }
}
pub(crate) unsafe extern "C" fn ClearLinkPlayerCountWindow(windowId: u16) {
    unsafe {
        let mut windowId = windowId;
        ClearStdWindowAndFrame(((windowId) as u8), 0u8);
        CopyWindowToVram(((windowId) as u8), 3u8);
    }
}
pub(crate) unsafe extern "C" fn UpdateLinkPlayerCountDisplay(taskId: u8, numPlayers: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut numPlayers = numPlayers;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        if ((numPlayers) as i32) != ((((data).wrapping_offset(3)).read()) as i32) {
            if ((numPlayers) as i32) <= 1i32 {
                ClearLinkPlayerCountWindow(((((data).wrapping_offset(5)).read()) as u16));
            } else {
                PrintNumPlayersInLink(
                    ((((data).wrapping_offset(5)).read()) as u16),
                    ((numPlayers) as u32),
                );
            }
            ((data).wrapping_offset(3)).write(((numPlayers) as i16));
        }
    }
}
pub(crate) unsafe extern "C" fn ExchangeDataAndGetLinkupStatus(
    minPlayers: u8,
    maxPlayers: u8,
) -> u32 {
    unsafe {
        let mut minPlayers = minPlayers;
        let mut maxPlayers = maxPlayers;
        'l1: {
            let __sw1 = ((GetLinkPlayerDataExchangeStatusTimed(
                ((minPlayers) as i32),
                ((maxPlayers) as i32),
            )) as i32);
            let __matched = __sw1 == 1i32
                || __sw1 == 3i32
                || __sw1 == 4i32
                || __sw1 == 5i32
                || __sw1 == 6i32
                || __sw1 == 7i32
                || __sw1 == 2i32;
            if __sw1 == 1i32 {
                return 1u32;
            }
            if __sw1 == 3i32 {
                return 3u32;
            }
            if __sw1 == 4i32 {
                return 7u32;
            }
            if __sw1 == 5i32 {
                return 9u32;
            }
            if __sw1 == 6i32 {
                ConvertIntToDecimalStringN(
                    (&raw mut gStringVar1).cast::<u8>(),
                    ((GetLinkPlayerCount_2()) as i32),
                    0i32,
                    1u8,
                );
                return 4u32;
            }
            if __sw1 == 7i32 {
                return 10u32;
            }
            if __sw1 == 2i32 || !__matched {
                return 0u32;
            }
        }
        #[allow(unreachable_code)]
        {
            return 0u32;
        }
    }
}
pub(crate) unsafe extern "C" fn CheckLinkErrored(taskId: u8) -> u32 {
    unsafe {
        let mut taskId = taskId;
        if ((HasLinkErrorOccurred()) as i32) == 1i32 {
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_LinkupConnectionError));
            return 1u32;
        }
        return 0u32;
    }
}
pub(crate) unsafe extern "C" fn CheckLinkCanceledBeforeConnection(taskId: u8) -> u32 {
    unsafe {
        let mut taskId = taskId;
        if (((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(46)
            .cast::<u16>())
        .read()) as i32)
            & 2i32)
            != 0)
            && (((IsLinkConnectionEstablished()) as i32) == 0i32)
        {
            ((&raw mut gLinkType).cast::<u16>()).write(0u16);
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_LinkupFailed));
            return 1u32;
        }
        return 0u32;
    }
}
pub(crate) unsafe extern "C" fn CheckLinkCanceled(taskId: u8) -> u32 {
    unsafe {
        let mut taskId = taskId;
        if (IsLinkConnectionEstablished()) != 0 {
            SetSuppressLinkErrorMessage(1u8);
        }
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(46)
            .cast::<u16>())
        .read()) as i32)
            & 2i32)
            != 0
        {
            ((&raw mut gLinkType).cast::<u16>()).write(0u16);
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_LinkupFailed));
            return 1u32;
        }
        return 0u32;
    }
}
pub(crate) unsafe extern "C" fn CheckSioErrored(taskId: u8) -> u32 {
    unsafe {
        let mut taskId = taskId;
        if ((GetSioMultiSI()) as i32) == 1i32 {
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_LinkupConnectionError));
            return 1u32;
        }
        return 0u32;
    }
}
pub(crate) unsafe extern "C" fn Task_DelayedBlockRequest(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let __p1 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        (__p1).write(((__p1).read()).wrapping_add(1));
        if (((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .read()) as i32)
            == 10i32
        {
            SendBlockRequest(2u8);
            DestroyTask(taskId);
        }
    }
}
pub(crate) unsafe extern "C" fn Task_LinkupStart(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        if (((data).read()) as i32) == 0i32 {
            OpenLinkTimed();
            ResetLinkPlayerCount();
            ResetLinkPlayers();
            ((data).wrapping_offset(5)).write(
                ((AddWindow(
                    (&raw const sWindowTemplate_LinkPlayerCount)
                        .cast::<u8>()
                        .cast_mut(),
                )) as i16),
            );
        } else {
            if (((data).read()) as i32) > 9i32 {
                ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
                .write(Some(Task_LinkupAwaitConnection));
            }
        }
        (data).write(((data).read()).wrapping_add(1));
    }
}
pub(crate) unsafe extern "C" fn Task_LinkupAwaitConnection(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut playerCount: u32 = ((GetLinkPlayerCount_2()) as u32);
        if ((CheckLinkCanceledBeforeConnection(taskId) == 1u32)
            || (CheckLinkCanceled(taskId) == 1u32))
            || (playerCount < 2u32)
        {
            return;
        }
        SetSuppressLinkErrorMessage(1u8);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(3))
        .write(0i16);
        if ((IsLinkMaster()) as i32) == 1i32 {
            PlaySE(21u16);
            ShowFieldAutoScrollMessage((&raw mut gText_ConfirmLinkWhenPlayersReady).cast::<u8>());
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_LinkupConfirmWhenReady));
        } else {
            PlaySE(22u16);
            ShowFieldAutoScrollMessage((&raw mut gText_AwaitingLinkup).cast::<u8>());
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_LinkupExchangeDataWithLeader));
        }
    }
}
pub(crate) unsafe extern "C" fn Task_LinkupConfirmWhenReady(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if ((CheckLinkCanceledBeforeConnection(taskId) == 1u32)
            || (CheckSioErrored(taskId) == 1u32))
            || (CheckLinkErrored(taskId) == 1u32)
        {
            return;
        }
        if ((GetFieldMessageBoxMode()) as i32) == 0i32 {
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(3))
            .write(0i16);
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_LinkupAwaitConfirmation));
        }
    }
}
pub(crate) unsafe extern "C" fn Task_LinkupAwaitConfirmation(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        let mut linkPlayerCount: i32 = ((GetLinkPlayerCount_2()) as i32);
        if ((CheckLinkCanceledBeforeConnection(taskId) == 1u32)
            || (CheckSioErrored(taskId) == 1u32))
            || (CheckLinkErrored(taskId) == 1u32)
        {
            return;
        }
        UpdateLinkPlayerCountDisplay(taskId, ((linkPlayerCount) as u8));
        if !(((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(46)
            .cast::<u16>())
        .read()) as i32)
            & 1i32)
            != 0)
        {
            return;
        }
        if linkPlayerCount < ((((data).wrapping_offset(1)).read()) as i32) {
            return;
        }
        SaveLinkPlayers(((linkPlayerCount) as u8));
        ClearLinkPlayerCountWindow(((((data).wrapping_offset(5)).read()) as u16));
        ConvertIntToDecimalStringN(
            (&raw mut gStringVar1).cast::<u8>(),
            linkPlayerCount,
            0i32,
            1u8,
        );
        ShowFieldAutoScrollMessage((&raw mut gText_ConfirmStartLinkWithXPlayers).cast::<u8>());
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Task_LinkupTryConfirmation));
    }
}
pub(crate) unsafe extern "C" fn Task_LinkupTryConfirmation(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if ((CheckLinkCanceledBeforeConnection(taskId) == 1u32)
            || (CheckSioErrored(taskId) == 1u32))
            || (CheckLinkErrored(taskId) == 1u32)
        {
            return;
        }
        if ((GetFieldMessageBoxMode()) as i32) == 0i32 {
            if ((GetSavedPlayerCount()) as i32) != ((GetLinkPlayerCount_2()) as i32) {
                ShowFieldAutoScrollMessage(
                    (&raw mut gText_ConfirmLinkWhenPlayersReady).cast::<u8>(),
                );
                ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
                .write(Some(Task_LinkupConfirmWhenReady));
            } else {
                if ((((((&raw mut gMain).cast::<u8>())
                    .wrapping_add(44)
                    .cast::<u16>())
                .read()) as i32)
                    & 2i32)
                    != 0
                {
                    ShowFieldAutoScrollMessage(
                        (&raw mut gText_ConfirmLinkWhenPlayersReady).cast::<u8>(),
                    );
                    ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .cast::<Option<unsafe extern "C" fn(u8)>>())
                    .write(Some(Task_LinkupConfirmWhenReady));
                } else {
                    if ((((((&raw mut gMain).cast::<u8>())
                        .wrapping_add(44)
                        .cast::<u16>())
                    .read()) as i32)
                        & 1i32)
                        != 0
                    {
                        PlaySE(5u16);
                        CheckShouldAdvanceLinkState();
                        ((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .cast::<Option<unsafe extern "C" fn(u8)>>())
                        .write(Some(Task_LinkupConfirm));
                    }
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_LinkupConfirm(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut minPlayers: u8 = ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .read()) as u8);
        let mut maxPlayers: u8 = ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(2))
        .read()) as u8);
        if (CheckLinkErrored(taskId) == 1u32) || (((TryLinkTimeout(taskId)) as i32) == 1i32) {
            return;
        }
        if ((GetLinkPlayerCount_2()) as i32) != ((GetSavedPlayerCount()) as i32) {
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_LinkupConnectionError));
        } else {
            ((&raw mut gSpecialVar_Result).cast::<u16>())
                .write(((ExchangeDataAndGetLinkupStatus(minPlayers, maxPlayers)) as u16));
            if ((((&raw mut gSpecialVar_Result).cast::<u16>()).read()) as i32) != 0i32 {
                ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
                .write(Some(Task_LinkupCheckStatusAfterConfirm));
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_LinkupExchangeDataWithLeader(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut minPlayers: u8 = 0u8;
        let mut maxPlayers: u8 = 0u8;
        let mut card: *mut u8 = core::ptr::null_mut();
        minPlayers = ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .read()) as u8);
        maxPlayers = ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(2))
        .read()) as u8);
        if (CheckLinkCanceledBeforeConnection(taskId) == 1u32) || (CheckLinkErrored(taskId) == 1u32)
        {
            return;
        }
        ((&raw mut gSpecialVar_Result).cast::<u16>())
            .write(((ExchangeDataAndGetLinkupStatus(minPlayers, maxPlayers)) as u16));
        if ((((&raw mut gSpecialVar_Result).cast::<u16>()).read()) as i32) == 0i32 {
            return;
        }
        if (((((&raw mut gSpecialVar_Result).cast::<u16>()).read()) as i32) == 3i32)
            || (((((&raw mut gSpecialVar_Result).cast::<u16>()).read()) as i32) == 4i32)
        {
            SetCloseLinkCallback();
            HideFieldMessageBox();
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_StopLinkup));
        } else {
            if (((((&raw mut gSpecialVar_Result).cast::<u16>()).read()) as i32) == 7i32)
                || (((((&raw mut gSpecialVar_Result).cast::<u16>()).read()) as i32) == 9i32)
            {
                CloseLink();
                HideFieldMessageBox();
                ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
                .write(Some(Task_StopLinkup));
            } else {
                ((&raw mut gFieldLinkPlayerCount).cast::<u8>()).write(GetLinkPlayerCount_2());
                ((&raw mut gLocalLinkPlayerId).cast::<u8>()).write(GetMultiplayerId());
                SaveLinkPlayers(((&raw mut gFieldLinkPlayerCount).cast::<u8>()).read());
                card = (&raw mut gBlockSendBuffer).cast::<u8>();
                TrainerCard_GenerateCardForLinkPlayer(card);
                (((card).wrapping_add(84)).cast::<u16>()).write(
                    ((GetMonData3(
                        ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                            (((((&raw mut gSelectedOrderFromParty).cast::<u8>()).read()) as i32)
                                .wrapping_sub(1i32)) as isize
                                * 100,
                        ),
                        11i32,
                        core::ptr::null_mut(),
                    )) as u16),
                );
                ((((card).wrapping_add(84)).cast::<u16>()).wrapping_offset(1)).write(
                    ((GetMonData3(
                        ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                            ((((((&raw mut gSelectedOrderFromParty).cast::<u8>())
                                .wrapping_offset(1))
                            .read()) as i32)
                                .wrapping_sub(1i32)) as isize
                                * 100,
                        ),
                        11i32,
                        core::ptr::null_mut(),
                    )) as u16),
                );
                ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
                .write(Some(Task_LinkupAwaitTrainerCardData));
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_LinkupCheckStatusAfterConfirm(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut card: *mut u8 = core::ptr::null_mut();
        if CheckLinkErrored(taskId) == 1u32 {
            return;
        }
        if ((((&raw mut gSpecialVar_Result).cast::<u16>()).read()) as i32) == 4i32 {
            if !((Link_AnyPartnersPlayingRubyOrSapphire()) != 0) {
                SetCloseLinkCallback();
                HideFieldMessageBox();
                ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
                .write(Some(Task_StopLinkup));
            } else {
                CloseLink();
                HideFieldMessageBox();
                ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
                .write(Some(Task_StopLinkup));
            }
        } else {
            if ((((&raw mut gSpecialVar_Result).cast::<u16>()).read()) as i32) == 3i32 {
                SetCloseLinkCallback();
                HideFieldMessageBox();
                ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
                .write(Some(Task_StopLinkup));
            } else {
                if (((((&raw mut gSpecialVar_Result).cast::<u16>()).read()) as i32) == 7i32)
                    || (((((&raw mut gSpecialVar_Result).cast::<u16>()).read()) as i32) == 9i32)
                {
                    CloseLink();
                    HideFieldMessageBox();
                    ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .cast::<Option<unsafe extern "C" fn(u8)>>())
                    .write(Some(Task_StopLinkup));
                } else {
                    ((&raw mut gFieldLinkPlayerCount).cast::<u8>()).write(GetLinkPlayerCount_2());
                    ((&raw mut gLocalLinkPlayerId).cast::<u8>()).write(GetMultiplayerId());
                    SaveLinkPlayers(((&raw mut gFieldLinkPlayerCount).cast::<u8>()).read());
                    card = (&raw mut gBlockSendBuffer).cast::<u8>();
                    TrainerCard_GenerateCardForLinkPlayer(card);
                    (((card).wrapping_add(84)).cast::<u16>()).write(
                        ((GetMonData3(
                            ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                                (((((&raw mut gSelectedOrderFromParty).cast::<u8>()).read())
                                    as i32)
                                    .wrapping_sub(1i32)) as isize
                                    * 100,
                            ),
                            11i32,
                            core::ptr::null_mut(),
                        )) as u16),
                    );
                    ((((card).wrapping_add(84)).cast::<u16>()).wrapping_offset(1)).write(
                        ((GetMonData3(
                            ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                                ((((((&raw mut gSelectedOrderFromParty).cast::<u8>())
                                    .wrapping_offset(1))
                                .read()) as i32)
                                    .wrapping_sub(1i32)) as isize
                                    * 100,
                            ),
                            11i32,
                            core::ptr::null_mut(),
                        )) as u16),
                    );
                    ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .cast::<Option<unsafe extern "C" fn(u8)>>())
                    .write(Some(Task_LinkupAwaitTrainerCardData));
                    SendBlockRequest(2u8);
                }
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AreBattleTowerLinkSpeciesSame(
    speciesList1: *mut u16,
    speciesList2: *mut u16,
) -> u32 {
    unsafe {
        let mut speciesList1 = speciesList1;
        let mut speciesList2 = speciesList2;
        let mut i: i32 = 0i32;
        let mut j: i32 = 0i32;
        let mut haveSameSpecies: u32 = 0u32;
        let mut numSameSpecies: i32 = 0i32;
        ((&raw mut gStringVar1).cast::<u8>()).write(255u8);
        ((&raw mut gStringVar2).cast::<u8>()).write(255u8);
        {
            i = 0i32;
            'l1: loop {
                if !(i < 2i32) {
                    break 'l1;
                }
                'l2: {
                    {
                        j = 0i32;
                        'l3: loop {
                            if !(j < 2i32) {
                                break 'l3;
                            }
                            'l4: {
                                if ((((speciesList1).wrapping_offset((i) as isize)).read()) as i32)
                                    == ((((speciesList2).wrapping_offset((j) as isize)).read())
                                        as i32)
                                {
                                    if numSameSpecies == 0i32 {
                                        StringCopy(
                                            (&raw mut gStringVar1).cast::<u8>(),
                                            (((&raw mut gSpeciesNames).cast::<u8>())
                                                .wrapping_offset(
                                                    ((((speciesList1)
                                                        .wrapping_offset((i) as isize))
                                                    .read())
                                                        as i32)
                                                        as isize
                                                        * 11,
                                                ))
                                            .cast::<u8>(),
                                        );
                                        haveSameSpecies = 1u32;
                                    }
                                    if numSameSpecies == 1i32 {
                                        StringCopy(
                                            (&raw mut gStringVar2).cast::<u8>(),
                                            (((&raw mut gSpeciesNames).cast::<u8>())
                                                .wrapping_offset(
                                                    ((((speciesList1)
                                                        .wrapping_offset((i) as isize))
                                                    .read())
                                                        as i32)
                                                        as isize
                                                        * 11,
                                                ))
                                            .cast::<u8>(),
                                        );
                                        haveSameSpecies = 1u32;
                                    }
                                    numSameSpecies = (numSameSpecies).wrapping_add(1);
                                }
                            }
                            j = (j).wrapping_add(1);
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        ((&raw mut gSpecialVar_0x8005).cast::<u16>()).write(((numSameSpecies) as u16));
        return haveSameSpecies;
    }
}
pub(crate) unsafe extern "C" fn FinishLinkup(linkupStatus: *mut u16, taskId: u32) {
    unsafe {
        let mut linkupStatus = linkupStatus;
        let mut taskId = taskId;
        let mut trainerCards: *mut u8 = (&raw mut gTrainerCards).cast::<u8>();
        if (((linkupStatus).read()) as i32) == 1i32 {
            if (((((&raw mut gLinkType).cast::<u16>()).read()) as i32) == 8806i32)
                || (((((&raw mut gLinkType).cast::<u16>()).read()) as i32) == 8823i32)
            {
                if (AreBattleTowerLinkSpeciesSame(
                    ((trainerCards).wrapping_add(84)).cast::<u16>(),
                    (((trainerCards).wrapping_offset(100)).wrapping_add(84)).cast::<u16>(),
                )) != 0
                {
                    (linkupStatus).write(11u16);
                    SetCloseLinkCallback();
                    ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .cast::<Option<unsafe extern "C" fn(u8)>>())
                    .write(Some(Task_StopLinkup));
                } else {
                    ClearLinkPlayerCountWindow(
                        ((((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(5))
                        .read()) as u16),
                    );
                    ScriptContext_Enable();
                    DestroyTask(((taskId) as u8));
                }
            } else {
                ClearLinkPlayerCountWindow(
                    ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(5))
                    .read()) as u16),
                );
                ScriptContext_Enable();
                DestroyTask(((taskId) as u8));
            }
        } else {
            SetCloseLinkCallback();
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_StopLinkup));
        }
    }
}
pub(crate) unsafe extern "C" fn Task_LinkupAwaitTrainerCardData(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut index: u8 = 0u8;
        if CheckLinkErrored(taskId) == 1u32 {
            return;
        }
        if ((GetBlockReceivedStatus()) as i32) != ((GetSavedLinkPlayerCountAsBitFlags()) as i32) {
            return;
        }
        {
            index = 0u8;
            'l1: loop {
                if !(((index) as i32) < ((GetLinkPlayerCount()) as i32)) {
                    break 'l1;
                }
                'l2: {
                    CopyTrainerCardData(
                        ((&raw mut gTrainerCards).cast::<u8>())
                            .wrapping_offset(((index) as i32) as isize * 100),
                        ((((&raw mut gBlockRecvBuffer).cast::<u8>())
                            .wrapping_offset(((index) as i32) as isize * 256))
                        .cast::<u16>())
                        .cast::<u8>(),
                        ((((((&raw mut gLinkPlayers).cast::<u8>())
                            .wrapping_offset(((index) as i32) as isize * 28))
                        .cast::<u16>())
                        .read()) as u8),
                    );
                }
                index = (index).wrapping_add(1);
            }
        }
        SetSuppressLinkErrorMessage(0u8);
        ResetBlockReceivedFlags();
        FinishLinkup(
            (&raw mut gSpecialVar_Result).cast::<u16>(),
            ((taskId) as u32),
        );
    }
}
pub(crate) unsafe extern "C" fn Task_StopLinkup(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if !((((&raw mut gReceivedRemoteLinkPlayers).cast::<u8>()).read()) != 0) {
            ClearLinkPlayerCountWindow(
                ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(5))
                .read()) as u16),
            );
            ScriptContext_Enable();
            RemoveWindow(
                ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(5))
                .read()) as u8),
            );
            DestroyTask(taskId);
        }
    }
}
pub(crate) unsafe extern "C" fn Task_LinkupFailed(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        ((&raw mut gSpecialVar_Result).cast::<u16>()).write(5u16);
        ClearLinkPlayerCountWindow(
            ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(5))
            .read()) as u16),
        );
        StopFieldMessage();
        RemoveWindow(
            ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(5))
            .read()) as u8),
        );
        ScriptContext_Enable();
        DestroyTask(taskId);
    }
}
pub(crate) unsafe extern "C" fn Task_LinkupConnectionError(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        ((&raw mut gSpecialVar_Result).cast::<u16>()).write(6u16);
        ClearLinkPlayerCountWindow(
            ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(5))
            .read()) as u16),
        );
        RemoveWindow(
            ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(5))
            .read()) as u8),
        );
        HideFieldMessageBox();
        ScriptContext_Enable();
        DestroyTask(taskId);
    }
}
pub(crate) unsafe extern "C" fn TryLinkTimeout(taskId: u8) -> u8 {
    unsafe {
        let mut taskId = taskId;
        let __p1 = (((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(4);
        (__p1).write(((__p1).read()).wrapping_add(1));
        if ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(4))
        .read()) as i32)
            > 600i32
        {
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_LinkupConnectionError));
            return 1u8;
        }
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn TryBattleLinkup() {
    unsafe {
        let mut minPlayers: u8 = 2u8;
        let mut maxPlayers: u8 = 2u8;
        'l1: {
            let __sw1 = ((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32);
            if __sw1 == 1i32 {
                minPlayers = 2u8;
                ((&raw mut gLinkType).cast::<u16>()).write(8755u16);
                break 'l1;
            }
            if __sw1 == 2i32 {
                minPlayers = 2u8;
                ((&raw mut gLinkType).cast::<u16>()).write(8772u16);
                break 'l1;
            }
            if __sw1 == 5i32 {
                minPlayers = 4u8;
                maxPlayers = 4u8;
                ((&raw mut gLinkType).cast::<u16>()).write(8789u16);
                break 'l1;
            }
            if __sw1 == 9i32 {
                minPlayers = 2u8;
                if ((crate::c::bf_read(
                    ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                        .wrapping_add(1629),
                    0,
                    2,
                    false,
                ) as u8) as i32)
                    == 0i32
                {
                    ((&raw mut gLinkType).cast::<u16>()).write(8806u16);
                } else {
                    ((&raw mut gLinkType).cast::<u16>()).write(8823u16);
                }
                break 'l1;
            }
        }
        CreateLinkupTask(minPlayers, maxPlayers);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn TryTradeLinkup() {
    unsafe {
        ((&raw mut gLinkType).cast::<u16>()).write(4403u16);
        ((&raw mut gBattleTypeFlags).cast::<u32>()).write(0u32);
        CreateLinkupTask(2u8, 2u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn TryRecordMixLinkup() {
    unsafe {
        ((&raw mut gSpecialVar_Result).cast::<u16>()).write(0u16);
        ((&raw mut gLinkType).cast::<u16>()).write(13073u16);
        ((&raw mut gBattleTypeFlags).cast::<u32>()).write(0u32);
        CreateLinkupTask(2u8, 4u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ValidateMixingGameLanguage() {
    unsafe {
        let mut taskId: u32 = ((FindTaskIdByFunc(Some(Task_ValidateMixingGameLanguage))) as u32);
        if taskId == 255u32 {
            taskId = ((CreateTask(Some(Task_ValidateMixingGameLanguage), 80u8)) as u32);
            (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
            .cast::<i16>())
            .write(0i16);
        }
    }
}
pub(crate) unsafe extern "C" fn Task_ValidateMixingGameLanguage(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut playerCount: i32 = 0i32;
        let mut i: i32 = 0i32;
        'l1: {
            let __sw1 = (((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .read()) as i32);
            if __sw1 == 0i32 {
                if ((((&raw mut gSpecialVar_Result).cast::<u16>()).read()) as i32) == 1i32 {
                    let mut mixingForeignGames: u32 = 0u32;
                    let mut isEnglishRSLinked: u32 = 0u32;
                    let mut isJapaneseEmeraldLinked: u32 = 0u32;
                    playerCount = ((GetLinkPlayerCount()) as i32);
                    {
                        i = 0i32;
                        'l2: loop {
                            if !(i < playerCount) {
                                break 'l2;
                            }
                            'l3: {
                                let mut version: u32 =
                                    (((((((&raw mut gLinkPlayers).cast::<u8>())
                                        .wrapping_offset((i) as isize * 28))
                                    .cast::<u16>())
                                    .read()) as u8) as u32);
                                let mut language: u32 =
                                    ((((((&raw mut gLinkPlayers).cast::<u8>())
                                        .wrapping_offset((i) as isize * 28))
                                    .wrapping_add(26)
                                    .cast::<u16>())
                                    .read()) as u32);
                                if (version == 2u32) || (version == 1u32) {
                                    if language == 1u32 {
                                        mixingForeignGames = 1u32;
                                        break 'l2;
                                    } else {
                                        isEnglishRSLinked = 1u32;
                                    }
                                } else {
                                    if version == 3u32 {
                                        if language == 1u32 {
                                            isJapaneseEmeraldLinked = 1u32;
                                        }
                                    }
                                }
                            }
                            i = (i).wrapping_add(1);
                        }
                    }
                    if ((isEnglishRSLinked) != 0) && ((isJapaneseEmeraldLinked) != 0) {
                        mixingForeignGames = 1u32;
                    }
                    if (mixingForeignGames) != 0 {
                        ((&raw mut gSpecialVar_Result).cast::<u16>()).write(12u16);
                        SetCloseLinkCallbackHandleJP();
                        (((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .write(1i16);
                        return;
                    }
                }
                ScriptContext_Enable();
                DestroyTask(taskId);
                break 'l1;
            }
            if __sw1 == 1i32 {
                if !((((&raw mut gReceivedRemoteLinkPlayers).cast::<u8>()).read()) != 0) {
                    ScriptContext_Enable();
                    DestroyTask(taskId);
                }
                break 'l1;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn TryBerryBlenderLinkup() {
    unsafe {
        ((&raw mut gLinkType).cast::<u16>()).write(17425u16);
        ((&raw mut gBattleTypeFlags).cast::<u32>()).write(0u32);
        CreateLinkupTask(2u8, 4u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn TryContestGModeLinkup() {
    unsafe {
        ((&raw mut gLinkType).cast::<u16>()).write(26113u16);
        ((&raw mut gBattleTypeFlags).cast::<u32>()).write(0u32);
        CreateLinkupTask(4u8, 4u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn TryContestEModeLinkup() {
    unsafe {
        ((&raw mut gLinkType).cast::<u16>()).write(26114u16);
        ((&raw mut gBattleTypeFlags).cast::<u32>()).write(0u32);
        CreateLinkupTask(2u8, 4u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CreateTask_ReestablishCableClubLink() -> u8 {
    unsafe {
        if ((FuncIsActiveTask(Some(Task_ReestablishLink))) as i32) != 0i32 {
            return 255u8;
        }
        'l1: {
            let __sw1 = ((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32);
            if __sw1 == 1i32 {
                ((&raw mut gLinkType).cast::<u16>()).write(8755u16);
                break 'l1;
            }
            if __sw1 == 2i32 {
                ((&raw mut gLinkType).cast::<u16>()).write(8772u16);
                break 'l1;
            }
            if __sw1 == 5i32 {
                ((&raw mut gLinkType).cast::<u16>()).write(8789u16);
                break 'l1;
            }
            if __sw1 == 9i32 {
                if ((crate::c::bf_read(
                    ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                        .wrapping_add(1629),
                    0,
                    2,
                    false,
                ) as u8) as i32)
                    == 0i32
                {
                    ((&raw mut gLinkType).cast::<u16>()).write(8806u16);
                } else {
                    ((&raw mut gLinkType).cast::<u16>()).write(8823u16);
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                ((&raw mut gLinkType).cast::<u16>()).write(4369u16);
                break 'l1;
            }
            if __sw1 == 4i32 {
                ((&raw mut gLinkType).cast::<u16>()).write(13090u16);
                break 'l1;
            }
        }
        return CreateTask(Some(Task_ReestablishLink), 80u8);
    }
}
pub(crate) unsafe extern "C" fn Task_ReestablishLink(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        if (((data).read()) as i32) == 0i32 {
            OpenLink();
            ResetLinkPlayers();
            CreateTask(Some(Task_WaitForLinkPlayerConnection), 80u8);
        } else {
            if (((data).read()) as i32) >= 10i32 {
                ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
                .write(Some(Task_ReestablishLinkAwaitConnection));
            }
        }
        (data).write(((data).read()).wrapping_add(1));
    }
}
pub(crate) unsafe extern "C" fn Task_ReestablishLinkAwaitConnection(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if ((GetLinkPlayerCount_2()) as i32) >= 2i32 {
            if ((IsLinkMaster()) as i32) == 1i32 {
                ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
                .write(Some(Task_ReestablishLinkLeader));
            } else {
                ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
                .write(Some(Task_ReestablishLinkAwaitConfirmation));
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_ReestablishLinkLeader(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if ((GetSavedPlayerCount()) as i32) == ((GetLinkPlayerCount_2()) as i32) {
            CheckShouldAdvanceLinkState();
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_ReestablishLinkAwaitConfirmation));
        }
    }
}
pub(crate) unsafe extern "C" fn Task_ReestablishLinkAwaitConfirmation(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if (((((&raw mut gReceivedRemoteLinkPlayers).cast::<u8>()).read()) as i32) == 1i32)
            && (((IsLinkPlayerDataExchangeComplete()) as i32) == 1i32)
        {
            CheckLinkPlayersMatchSaved();
            StartSendingKeysToLink();
            DestroyTask(taskId);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CableClubSaveGame() {
    unsafe {
        SaveGame();
    }
}
pub(crate) unsafe extern "C" fn SetLinkBattleTypeFlags(linkService: i32) {
    unsafe {
        let mut linkService = linkService;
        'l1: {
            let __sw1 = linkService;
            if __sw1 == 1i32 {
                ((&raw mut gBattleTypeFlags).cast::<u32>()).write(10u32);
                break 'l1;
            }
            if __sw1 == 2i32 {
                ((&raw mut gBattleTypeFlags).cast::<u32>()).write(11u32);
                break 'l1;
            }
            if __sw1 == 5i32 {
                ReducePlayerPartyToSelectedMons();
                ((&raw mut gBattleTypeFlags).cast::<u32>()).write(75u32);
                break 'l1;
            }
            if __sw1 == 9i32 {
                ((&raw mut gBattleTypeFlags).cast::<u32>()).write(331u32);
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_StartWiredCableClubBattle(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut task: *mut u8 =
            ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40);
        'l1: {
            let __sw1 = (((((task).wrapping_add(8)).cast::<i16>()).read()) as i32);
            if __sw1 == 0i32 {
                FadeScreen(1u8, 0i8);
                ((&raw mut gLinkType).cast::<u16>()).write(8721u16);
                ClearLinkCallback_2();
                let __p2 = ((task).wrapping_add(8)).cast::<i16>();
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                if !((crate::c::bf_read(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
                    7,
                    1,
                    false,
                ) as u16)
                    != 0)
                {
                    let __p3 = ((task).wrapping_add(8)).cast::<i16>();
                    (__p3).write(((__p3).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                let __p4 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1);
                (__p4).write(((__p4).read()).wrapping_add(1));
                if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
                    > 20i32
                {
                    let __p5 = ((task).wrapping_add(8)).cast::<i16>();
                    (__p5).write(((__p5).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                SetCloseLinkCallback();
                let __p6 = ((task).wrapping_add(8)).cast::<i16>();
                (__p6).write(((__p6).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 4i32 {
                if !((((&raw mut gReceivedRemoteLinkPlayers).cast::<u8>()).read()) != 0) {
                    let __p7 = ((task).wrapping_add(8)).cast::<i16>();
                    (__p7).write(((__p7).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 5i32 {
                if ((((&raw mut gLinkPlayers).cast::<u8>())
                    .wrapping_add(4)
                    .cast::<u32>())
                .read()
                    & 1u32)
                    != 0
                {
                    PlayMapChosenOrBattleBGM(477u16);
                } else {
                    PlayMapChosenOrBattleBGM(476u16);
                }
                SetLinkBattleTypeFlags(
                    ((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32),
                );
                CleanupOverworldWindowsAndTilemaps();
                ((&raw mut gTrainerBattleOpponent_A).cast::<u16>()).write(2048u16);
                SetMainCallback2(Some(CB2_InitBattle));
                (((&raw mut gMain).cast::<u8>())
                    .wrapping_add(8)
                    .cast::<Option<unsafe extern "C" fn()>>())
                .write(Some(CB2_ReturnFromCableClubBattle));
                DestroyTask(taskId);
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_StartWirelessCableClubBattle(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut i: i32 = 0i32;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        'l1: {
            let __sw1 = (((data).read()) as i32);
            if __sw1 == 0i32 {
                FadeScreen(1u8, 0i8);
                ((&raw mut gLinkType).cast::<u16>()).write(8721u16);
                ClearLinkCallback_2();
                (data).write(1i16);
                break 'l1;
            }
            if __sw1 == 1i32 {
                if !((crate::c::bf_read(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
                    7,
                    1,
                    false,
                ) as u16)
                    != 0)
                {
                    (data).write(2i16);
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                SendBlock(0u8, (&raw mut gLocalLinkPlayer).cast::<u8>(), 28u16);
                (data).write(3i16);
                break 'l1;
            }
            if __sw1 == 3i32 {
                if ((GetBlockReceivedStatus()) as i32) == ((GetLinkPlayerCountAsBitFlags()) as i32)
                {
                    {
                        i = 0i32;
                        'l2: loop {
                            if !(i < ((GetLinkPlayerCount()) as i32)) {
                                break 'l2;
                            }
                            'l3: {
                                let mut player: *mut u8 = ((((&raw mut gBlockRecvBuffer)
                                    .cast::<u8>())
                                .wrapping_offset((i) as isize * 256))
                                .cast::<u16>())
                                .cast::<u8>();
                                ((&raw mut gLinkPlayers).cast::<u8>())
                                    .wrapping_offset((i) as isize * 28)
                                    .cast::<crate::c::Rec4<28>>()
                                    .write_unaligned(
                                        player.cast::<crate::c::Rec4<28>>().read_unaligned(),
                                    );
                                ConvertLinkPlayerName(
                                    ((&raw mut gLinkPlayers).cast::<u8>())
                                        .wrapping_offset((i) as isize * 28),
                                );
                                ResetBlockReceivedFlag(((i) as u8));
                            }
                            i = (i).wrapping_add(1);
                        }
                    }
                    (data).write(4i16);
                }
                break 'l1;
            }
            if __sw1 == 4i32 {
                let __p2 = (data).wrapping_offset(1);
                (__p2).write(((__p2).read()).wrapping_add(1));
                if ((((data).wrapping_offset(1)).read()) as i32) > 20i32 {
                    (data).write(5i16);
                }
                break 'l1;
            }
            if __sw1 == 5i32 {
                SetLinkStandbyCallback();
                (data).write(6i16);
                break 'l1;
            }
            if __sw1 == 6i32 {
                if (IsLinkTaskFinished()) != 0 {
                    (data).write(7i16);
                }
                break 'l1;
            }
            if __sw1 == 7i32 {
                if ((((&raw mut gLinkPlayers).cast::<u8>())
                    .wrapping_add(4)
                    .cast::<u32>())
                .read()
                    & 1u32)
                    != 0
                {
                    PlayMapChosenOrBattleBGM(477u16);
                } else {
                    PlayMapChosenOrBattleBGM(476u16);
                }
                (((&raw mut gLinkPlayers).cast::<u8>())
                    .wrapping_add(20)
                    .cast::<u32>())
                .write(8721u32);
                SetLinkBattleTypeFlags(
                    ((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32),
                );
                CleanupOverworldWindowsAndTilemaps();
                ((&raw mut gTrainerBattleOpponent_A).cast::<u16>()).write(2048u16);
                SetMainCallback2(Some(CB2_InitBattle));
                (((&raw mut gMain).cast::<u8>())
                    .wrapping_add(8)
                    .cast::<Option<unsafe extern "C" fn()>>())
                .write(Some(CB2_ReturnFromCableClubBattle));
                DestroyTask(taskId);
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn CB2_ReturnFromUnionRoomBattle() {
    unsafe {
        let mut playerCount: u8 = 0u8;
        let mut i: i32 = 0i32;
        let mut linkedWithFRLG: u32 = 0u32;
        'l1: {
            let __sw1 = (((((&raw mut gMain).cast::<u8>()).wrapping_add(1080)).read()) as i32);
            if __sw1 == 0i32 {
                playerCount = GetLinkPlayerCount();
                linkedWithFRLG = 0u32;
                {
                    i = 0i32;
                    'l2: loop {
                        if !(i < ((playerCount) as i32)) {
                            break 'l2;
                        }
                        'l3: {
                            let mut version: u32 = (((((((&raw mut gLinkPlayers).cast::<u8>())
                                .wrapping_offset((i) as isize * 28))
                            .cast::<u16>())
                            .read()) as u8)
                                as u32);
                            if (version == 4u32) || (version == 5u32) {
                                linkedWithFRLG = 1u32;
                                break 'l2;
                            }
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                if (linkedWithFRLG) != 0 {
                    (((&raw mut gMain).cast::<u8>()).wrapping_add(1080)).write(2u8);
                } else {
                    SetCloseLinkCallback();
                    (((&raw mut gMain).cast::<u8>()).wrapping_add(1080)).write(1u8);
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                if !((((&raw mut gReceivedRemoteLinkPlayers).cast::<u8>()).read()) != 0) {
                    SetMainCallback2(Some(CB2_ReturnToField));
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                SetMainCallback2(Some(CB2_ReturnToField));
                break 'l1;
            }
        }
        RunTasks();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CB2_ReturnFromCableClubBattle() {
    unsafe {
        let __p1 = (&raw mut gBattleTypeFlags).cast::<u32>();
        (__p1).write(((__p1).read() & 4294967263u32));
        Overworld_ResetMapMusic();
        LoadPlayerParty();
        SavePlayerBag();
        UpdateTrainerFansAfterLinkBattle();
        if (((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32) == 1i32)
            || (((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32) == 2i32)
        {
            UpdatePlayerLinkBattleRecords(
                (((((&raw mut gLocalLinkPlayerId).cast::<u8>()).read()) as i32) ^ 1i32),
            );
            if (((&raw mut gWirelessCommType).cast::<u8>()).read()) != 0 {
                'l1: {
                    let __sw2 = ((((&raw mut gBattleOutcome).cast::<u8>()).read()) as i32);
                    if __sw2 == 1i32 {
                        MysteryGift_TryIncrementStat(
                            0u32,
                            ((((&raw mut gLinkPlayers).cast::<u8>()).wrapping_offset(
                                (((GetMultiplayerId()) as i32) ^ 1i32) as isize * 28,
                            ))
                            .wrapping_add(4)
                            .cast::<u32>())
                            .read(),
                        );
                        break 'l1;
                    }
                    if __sw2 == 2i32 {
                        MysteryGift_TryIncrementStat(
                            1u32,
                            ((((&raw mut gLinkPlayers).cast::<u8>()).wrapping_offset(
                                (((GetMultiplayerId()) as i32) ^ 1i32) as isize * 28,
                            ))
                            .wrapping_add(4)
                            .cast::<u32>())
                            .read(),
                        );
                        break 'l1;
                    }
                }
            }
        }
        if InUnionRoom() == 1u32 {
            (((&raw mut gMain).cast::<u8>())
                .wrapping_add(8)
                .cast::<Option<unsafe extern "C" fn()>>())
            .write(Some(CB2_ReturnFromUnionRoomBattle));
        } else {
            (((&raw mut gMain).cast::<u8>())
                .wrapping_add(8)
                .cast::<Option<unsafe extern "C" fn()>>())
            .write(Some(CB2_ReturnToFieldFromMultiplayer));
        }
        SetMainCallback2(Some(CB2_SetUpSaveAfterLinkBattle));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CleanupLinkRoomState() {
    unsafe {
        if (((((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32) == 1i32)
            || (((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32) == 2i32))
            || (((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32) == 5i32))
            || (((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32) == 9i32)
        {
            LoadPlayerParty();
            SavePlayerBag();
        }
        SetWarpDestinationToDynamicWarp(127u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ExitLinkRoom() {
    unsafe {
        QueueExitLinkRoomKey();
    }
}
pub(crate) unsafe extern "C" fn Task_EnterCableClubSeat(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut task: *mut u8 =
            ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40);
        'l1: {
            let __sw1 = (((((task).wrapping_add(8)).cast::<i16>()).read()) as i32);
            if __sw1 == 0i32 {
                ShowFieldMessage((&raw mut gText_PleaseWaitForLink).cast::<u8>());
                (((task).wrapping_add(8)).cast::<i16>()).write(1i16);
                break 'l1;
            }
            if __sw1 == 1i32 {
                if (IsFieldMessageBoxHidden()) != 0 {
                    SetInCableClubSeat();
                    SetLocalLinkPlayerId(
                        ((((&raw mut gSpecialVar_0x8005).cast::<u16>()).read()) as u8),
                    );
                    (((task).wrapping_add(8)).cast::<i16>()).write(2i16);
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                'l2: {
                    let __sw2 = GetCableClubPartnersReady();
                    if __sw2 == 0u32 {
                        break 'l2;
                    }
                    if __sw2 == 1u32 {
                        HideFieldMessageBox();
                        (((task).wrapping_add(8)).cast::<i16>()).write(0i16);
                        SetStartedCableClubActivity();
                        SwitchTaskToFollowupFunc(taskId);
                        break 'l2;
                    }
                    if __sw2 == 2u32 {
                        (((task).wrapping_add(8)).cast::<i16>()).write(3i16);
                        break 'l2;
                    }
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                SetLinkWaitingForScript();
                EraseFieldMessageBox(1u8);
                DestroyTask(taskId);
                ScriptContext_Enable();
                break 'l1;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CreateTask_EnterCableClubSeat(
    followupFunc: Option<unsafe extern "C" fn(u8)>,
) {
    unsafe {
        let mut followupFunc = followupFunc;
        let mut taskId: u8 = CreateTask(Some(Task_EnterCableClubSeat), 80u8);
        SetTaskFuncWithFollowupFunc(taskId, Some(Task_EnterCableClubSeat), followupFunc);
        ScriptContext_Stop();
    }
}
pub(crate) unsafe extern "C" fn Task_StartWiredTrade(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut task: *mut u8 =
            ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40);
        'l1: {
            let __sw1 = (((((task).wrapping_add(8)).cast::<i16>()).read()) as i32);
            if __sw1 == 0i32 {
                LockPlayerFieldControls();
                FadeScreen(1u8, 0i8);
                ClearLinkCallback_2();
                let __p2 = ((task).wrapping_add(8)).cast::<i16>();
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                if !((crate::c::bf_read(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
                    7,
                    1,
                    false,
                ) as u16)
                    != 0)
                {
                    let __p3 = ((task).wrapping_add(8)).cast::<i16>();
                    (__p3).write(((__p3).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                ((&raw mut gSelectedTradeMonPositions).cast::<u8>()).write(0u8);
                (((&raw mut gSelectedTradeMonPositions).cast::<u8>()).wrapping_offset(1))
                    .write(0u8);
                m4aMPlayAllStop();
                SetCloseLinkCallback();
                let __p4 = ((task).wrapping_add(8)).cast::<i16>();
                (__p4).write(((__p4).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 3i32 {
                if !((((&raw mut gReceivedRemoteLinkPlayers).cast::<u8>()).read()) != 0) {
                    SetMainCallback2(Some(CB2_StartCreateTradeMenu));
                    DestroyTask(taskId);
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_StartWirelessTrade(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        'l1: {
            let __sw1 = (((data).read()) as i32);
            if __sw1 == 0i32 {
                LockPlayerFieldControls();
                FadeScreen(1u8, 0i8);
                ClearLinkRfuCallback();
                (data).write(((data).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                if !((crate::c::bf_read(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
                    7,
                    1,
                    false,
                ) as u16)
                    != 0)
                {
                    (data).write(((data).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                ((&raw mut gSelectedTradeMonPositions).cast::<u8>()).write(0u8);
                (((&raw mut gSelectedTradeMonPositions).cast::<u8>()).wrapping_offset(1))
                    .write(0u8);
                m4aMPlayAllStop();
                SetLinkStandbyCallback();
                (data).write(((data).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 3i32 {
                if (IsLinkTaskFinished()) != 0 {
                    CreateTask_CreateTradeMenu();
                    DestroyTask(taskId);
                }
                break 'l1;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PlayerEnteredTradeSeat() {
    unsafe {
        if ((((&raw mut gWirelessCommType).cast::<u8>()).read()) as i32) != 0i32 {
            CreateTask_EnterCableClubSeat(Some(Task_StartWirelessTrade));
        } else {
            CreateTask_EnterCableClubSeat(Some(Task_StartWiredTrade));
        }
    }
}
pub(crate) unsafe extern "C" fn CreateTask_StartWiredTrade() {
    unsafe {
        CreateTask(Some(Task_StartWiredTrade), 80u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Script_StartWiredTrade() {
    unsafe {}
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ColosseumPlayerSpotTriggered() {
    unsafe {
        ((&raw mut gLinkType).cast::<u16>()).write(8721u16);
        if (((&raw mut gWirelessCommType).cast::<u8>()).read()) != 0 {
            CreateTask_EnterCableClubSeat(Some(Task_StartWirelessCableClubBattle));
        } else {
            CreateTask_EnterCableClubSeat(Some(Task_StartWiredCableClubBattle));
        }
    }
}
pub(crate) unsafe extern "C" fn CreateTask_EnterCableClubSeatNoFollowup() {
    unsafe {
        let mut taskId: u8 = CreateTask(Some(Task_EnterCableClubSeat), 80u8);
        ScriptContext_Stop();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Script_ShowLinkTrainerCard() {
    unsafe {
        ShowTrainerCardInLink(
            ((((&raw mut gSpecialVar_0x8006).cast::<u16>()).read()) as u8),
            Some(CB2_ReturnToFieldContinueScriptPlayMapMusic),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetLinkTrainerCardColor(linkPlayerIndex: u8) -> u32 {
    unsafe {
        let mut linkPlayerIndex = linkPlayerIndex;
        let mut numStars: u32 = 0u32;
        ((&raw mut gSpecialVar_0x8006).cast::<u16>()).write(((linkPlayerIndex) as u16));
        StringCopy(
            (&raw mut gStringVar1).cast::<u8>(),
            ((((&raw mut gLinkPlayers).cast::<u8>())
                .wrapping_offset(((linkPlayerIndex) as i32) as isize * 28))
            .wrapping_add(8))
            .cast::<u8>(),
        );
        numStars = ((GetTrainerCardStars(linkPlayerIndex)) as u32);
        if numStars == 0u32 {
            return 0u32;
        }
        StringCopy(
            (&raw mut gStringVar2).cast::<u8>(),
            ((((&raw const sTrainerCardColorNames)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u8>())
            .cast::<*mut u8>())
            .wrapping_offset((((numStars).wrapping_sub(1u32)) as i32) as isize))
            .read(),
        );
        return 1u32;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Task_WaitForLinkPlayerConnection(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut task: *mut u8 =
            ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40);
        let __p1 = ((task).wrapping_add(8)).cast::<i16>();
        (__p1).write(((__p1).read()).wrapping_add(1));
        if (((((task).wrapping_add(8)).cast::<i16>()).read()) as i32) > 300i32 {
            CloseLink();
            SetMainCallback2(Some(CB2_LinkError));
            DestroyTask(taskId);
        }
        if (((&raw mut gReceivedRemoteLinkPlayers).cast::<u8>()).read()) != 0 {
            if ((((&raw mut gWirelessCommType).cast::<u8>()).read()) as i32) == 0i32 {
                if !((DoesLinkPlayerCountMatchSaved()) != 0) {
                    CloseLink();
                    SetMainCallback2(Some(CB2_LinkError));
                }
                DestroyTask(taskId);
            } else {
                DestroyTask(taskId);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_WaitExitToScript(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if !((((&raw mut gReceivedRemoteLinkPlayers).cast::<u8>()).read()) != 0) {
            ScriptContext_Enable();
            DestroyTask(taskId);
        }
    }
}
pub(crate) unsafe extern "C" fn ExitLinkToScript(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        SetCloseLinkCallback();
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Task_WaitExitToScript));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Task_ReconnectWithLinkPlayers(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        'l1: {
            let __sw1 = (((data).read()) as i32);
            if __sw1 == 0i32 {
                if ((((&raw mut gWirelessCommType).cast::<u8>()).read()) as i32) != 0i32 {
                    DestroyTask(taskId);
                } else {
                    OpenLink();
                    CreateTask(Some(Task_WaitForLinkPlayerConnection), 1u8);
                    (data).write(((data).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                if (({
                    let __p2 = (data).wrapping_offset(1);
                    let __t3 = ((__p2).read()).wrapping_add(1);
                    (__p2).write(__t3);
                    __t3
                }) as i32)
                    > 11i32
                {
                    ((data).wrapping_offset(1)).write(0i16);
                    (data).write(((data).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                if ((GetLinkPlayerCount_2()) as i32) >= ((GetSavedPlayerCount()) as i32) {
                    if (IsLinkMaster()) != 0 {
                        if (({
                            let __p4 = (data).wrapping_offset(1);
                            let __t5 = ((__p4).read()).wrapping_add(1);
                            (__p4).write(__t5);
                            __t5
                        }) as i32)
                            > 30i32
                        {
                            CheckShouldAdvanceLinkState();
                            (data).write(((data).read()).wrapping_add(1));
                        }
                    } else {
                        (data).write(((data).read()).wrapping_add(1));
                    }
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                if (((((&raw mut gReceivedRemoteLinkPlayers).cast::<u8>()).read()) as i32) == 1i32)
                    && (((IsLinkPlayerDataExchangeComplete()) as i32) == 1i32)
                {
                    DestroyTask(taskId);
                }
                break 'l1;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn TrySetBattleTowerLinkType() {
    unsafe {
        if ((((&raw mut gWirelessCommType).cast::<u8>()).read()) as i32) == 0i32 {
            ((&raw mut gLinkType).cast::<u16>()).write(8840u16);
        }
    }
}
