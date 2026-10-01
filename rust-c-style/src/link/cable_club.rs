//! Translated from `src/cable_club.c` by tools/rustport/c2rs.py.
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
// Data tables (translate with cdata.py): sWindowTemplate_LinkPlayerCount sTrainerCardColorNames

static sTrainerCardColorNames: Table<CArray<*mut u8, 4>> =
    Table((&raw const crate::data::cable_club::sTrainerCardColorNames).cast());
static sWindowTemplate_LinkPlayerCount: Table<WindowTemplate> =
    Table((&raw const crate::data::cable_club::sWindowTemplate_LinkPlayerCount).cast());

unsafe extern "C" {
    static mut gBattleOutcome: u8;
    static mut gBattleTypeFlags: u32;
    static mut gBlockRecvBuffer: CArray<CArray<u16, 128>, 5>;
    static mut gBlockSendBuffer: CArray<u8, 256>;
    static mut gFieldLinkPlayerCount: u8;
    static mut gLinkPlayers: CArray<LinkPlayer, 5>;
    static mut gLinkType: u16;
    static mut gLocalLinkPlayer: LinkPlayer;
    static mut gLocalLinkPlayerId: u8;
    static mut gMain: Main;
    static mut gPaletteFade: PaletteFadeControl;
    static mut gPlayerParty: CArray<Pokemon, 6>;
    static mut gReceivedRemoteLinkPlayers: u8;
    static mut gSaveBlock2Ptr: *mut SaveBlock2;
    static mut gSelectedOrderFromParty: CArray<u8, 4>;
    static mut gSelectedTradeMonPositions: CArray<u8, 2>;
    static mut gSpecialVar_0x8004: u16;
    static mut gSpecialVar_0x8005: u16;
    static mut gSpecialVar_0x8006: u16;
    static mut gSpecialVar_Result: u16;
    static gSpeciesNames: CArray<CArray<u8, 11>, 0>;
    static mut gStringVar1: CArray<u8, 256>;
    static mut gStringVar2: CArray<u8, 256>;
    static mut gStringVar4: CArray<u8, 1000>;
    static mut gTasks: CArray<Task, 0>;
    static gText_AwaitingLinkup: CArray<u8, 0>;
    static gText_ConfirmLinkWhenPlayersReady: CArray<u8, 0>;
    static gText_ConfirmStartLinkWithXPlayers: CArray<u8, 0>;
    static gText_NumPlayerLink: CArray<u8, 0>;
    static gText_PleaseWaitForLink: CArray<u8, 0>;
    static mut gTrainerBattleOpponent_A: u16;
    static mut gTrainerCards: CArray<TrainerCard, 4>;
    static mut gWirelessCommType: u8;
    fn AddTextPrinterParameterized(
        a0: u8,
        a1: u8,
        a2: *mut u8,
        a3: u8,
        a4: u8,
        a5: u8,
        a6: Option<unsafe extern "C" fn(*mut TextPrinterTemplate, u16)>,
    ) -> u16;
    fn AddWindow(a0: *mut WindowTemplate) -> u16;
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
    fn ConvertLinkPlayerName(a0: *mut LinkPlayer);
    fn CopyTrainerCardData(a0: *mut TrainerCard, a1: *mut TrainerCard, a2: u8);
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
    fn GetMonData3(a0: *mut Pokemon, a1: i32, a2: *mut u8) -> u32;
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
    fn SendBlock(a0: u8, a1: *mut c_void, a2: u16) -> u8;
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
    fn TrainerCard_GenerateCardForLinkPlayer(a0: *mut TrainerCard);
    fn UpdatePlayerLinkBattleRecords(a0: i32);
    fn UpdateTrainerFansAfterLinkBattle();
    fn m4aMPlayAllStop();
}

pub(crate) unsafe extern "C" fn CreateLinkupTask(minPlayers: u8, maxPlayers: u8) {
    if FindTaskIdByFunc(Some(Task_LinkupStart)) == TASK_NONE {
        let mut taskId1: u8 = 0;
        taskId1 = CreateTask(Some(Task_LinkupStart), 80);
        gTasks[taskId1].data[1] = minPlayers as i16;
        gTasks[taskId1].data[2] = maxPlayers as i16;
    }
}
pub(crate) unsafe extern "C" fn PrintNumPlayersInLink(windowId: u16, numPlayers: u32) {
    let mut xPos: u8 = 0;
    ConvertIntToDecimalStringN(
        gStringVar1.as_mut_ptr(),
        numPlayers as i32,
        STR_CONV_MODE_LEFT_ALIGN,
        1,
    );
    SetStandardWindowBorderStyle(windowId as u8, FALSE);
    StringExpandPlaceholders(
        gStringVar4.as_mut_ptr(),
        gText_NumPlayerLink.as_ptr().cast_mut(),
    );
    xPos = GetStringCenterAlignXOffset(FONT_NORMAL as i32, gStringVar4.as_mut_ptr(), 88) as u8;
    AddTextPrinterParameterized(
        windowId as u8,
        FONT_NORMAL,
        gStringVar4.as_mut_ptr(),
        xPos,
        1,
        TEXT_SKIP_DRAW,
        None,
    );
    CopyWindowToVram(windowId as u8, COPYWIN_FULL);
}
pub(crate) unsafe extern "C" fn ClearLinkPlayerCountWindow(windowId: u16) {
    ClearStdWindowAndFrame(windowId as u8, FALSE);
    CopyWindowToVram(windowId as u8, COPYWIN_FULL);
}
pub(crate) unsafe extern "C" fn UpdateLinkPlayerCountDisplay(taskId: u8, numPlayers: u8) {
    let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
    if numPlayers as i16 != *data.at(3) {
        if numPlayers <= 1 {
            ClearLinkPlayerCountWindow(*data.at(5) as u16);
        } else {
            PrintNumPlayersInLink(*data.at(5) as u16, numPlayers as u32);
        }
        *data.at(3) = numPlayers as i16;
    }
}
pub(crate) unsafe extern "C" fn ExchangeDataAndGetLinkupStatus(
    minPlayers: u8,
    maxPlayers: u8,
) -> u32 {
    match GetLinkPlayerDataExchangeStatusTimed(minPlayers as i32, maxPlayers as i32) {
        1 => {
            return LINKUP_SUCCESS as u32;
        }
        3 => {
            return LINKUP_DIFF_SELECTIONS as u32;
        }
        EXCHANGE_PLAYER_NOT_READY => {
            return LINKUP_PLAYER_NOT_READY as u32;
        }
        EXCHANGE_PARTNER_NOT_READY => {
            return LINKUP_PARTNER_NOT_READY as u32;
        }
        EXCHANGE_WRONG_NUM_PLAYERS => {
            ConvertIntToDecimalStringN(
                gStringVar1.as_mut_ptr(),
                GetLinkPlayerCount_2() as i32,
                STR_CONV_MODE_LEFT_ALIGN,
                1,
            );
            return LINKUP_WRONG_NUM_PLAYERS as u32;
        }
        EXCHANGE_STAT_7 => {
            return LINKUP_FAILED_CONTEST_GMODE;
        }
        _ => {
            return LINKUP_ONGOING as u32;
        }
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn CheckLinkErrored(taskId: u8) -> u32 {
    if HasLinkErrorOccurred() == TRUE {
        gTasks[taskId].func = Some(Task_LinkupConnectionError);
        return TRUE as u32;
    }
    return FALSE as u32;
}
pub(crate) unsafe extern "C" fn CheckLinkCanceledBeforeConnection(taskId: u8) -> u32 {
    if gMain.newKeys as i32 & B_BUTTON != 0 && IsLinkConnectionEstablished() == FALSE {
        gLinkType = 0;
        gTasks[taskId].func = Some(Task_LinkupFailed);
        return TRUE as u32;
    }
    return FALSE as u32;
}
pub(crate) unsafe extern "C" fn CheckLinkCanceled(taskId: u8) -> u32 {
    if IsLinkConnectionEstablished() != 0 {
        SetSuppressLinkErrorMessage(TRUE);
    }
    if gMain.newKeys as i32 & B_BUTTON != 0 {
        gLinkType = 0;
        gTasks[taskId].func = Some(Task_LinkupFailed);
        return TRUE as u32;
    }
    return FALSE as u32;
}
pub(crate) unsafe extern "C" fn CheckSioErrored(taskId: u8) -> u32 {
    if GetSioMultiSI() == TRUE {
        gTasks[taskId].func = Some(Task_LinkupConnectionError);
        return TRUE as u32;
    }
    return FALSE as u32;
}
pub(crate) unsafe extern "C" fn Task_DelayedBlockRequest(taskId: u8) {
    gTasks[taskId].data[0] += 1;
    if gTasks[taskId].data[0] == 10 {
        SendBlockRequest(BLOCK_REQ_SIZE_100);
        DestroyTask(taskId);
    }
}
pub(crate) unsafe extern "C" fn Task_LinkupStart(taskId: u8) {
    let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
    if *data == 0 {
        OpenLinkTimed();
        ResetLinkPlayerCount();
        ResetLinkPlayers();
        *data.at(5) = AddWindow((&raw const *sWindowTemplate_LinkPlayerCount).cast_mut()) as i16;
    } else if *data > 9 {
        gTasks[taskId].func = Some(Task_LinkupAwaitConnection);
    }
    *data += 1;
}
pub(crate) unsafe extern "C" fn Task_LinkupAwaitConnection(taskId: u8) {
    let mut playerCount: u32 = GetLinkPlayerCount_2() as u32;
    if CheckLinkCanceledBeforeConnection(taskId) == TRUE as u32
        || CheckLinkCanceled(taskId) == TRUE as u32
        || playerCount < 2
    {
        return;
    }
    SetSuppressLinkErrorMessage(TRUE);
    gTasks[taskId].data[3] = 0;
    if IsLinkMaster() == TRUE {
        PlaySE(SE_PIN);
        ShowFieldAutoScrollMessage(gText_ConfirmLinkWhenPlayersReady.as_ptr().cast_mut());
        gTasks[taskId].func = Some(Task_LinkupConfirmWhenReady);
    } else {
        PlaySE(SE_BOO);
        ShowFieldAutoScrollMessage(gText_AwaitingLinkup.as_ptr().cast_mut());
        gTasks[taskId].func = Some(Task_LinkupExchangeDataWithLeader);
    }
}
pub(crate) unsafe extern "C" fn Task_LinkupConfirmWhenReady(taskId: u8) {
    if CheckLinkCanceledBeforeConnection(taskId) == TRUE as u32
        || CheckSioErrored(taskId) == TRUE as u32
        || CheckLinkErrored(taskId) == TRUE as u32
    {
        return;
    }
    if GetFieldMessageBoxMode() == FIELD_MESSAGE_BOX_HIDDEN {
        gTasks[taskId].data[3] = 0;
        gTasks[taskId].func = Some(Task_LinkupAwaitConfirmation);
    }
}
pub(crate) unsafe extern "C" fn Task_LinkupAwaitConfirmation(taskId: u8) {
    let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
    let mut linkPlayerCount: i32 = GetLinkPlayerCount_2() as i32;
    if CheckLinkCanceledBeforeConnection(taskId) == TRUE as u32
        || CheckSioErrored(taskId) == TRUE as u32
        || CheckLinkErrored(taskId) == TRUE as u32
    {
        return;
    }
    UpdateLinkPlayerCountDisplay(taskId, linkPlayerCount as u8);
    if gMain.newKeys as i32 & A_BUTTON == 0 {
        return;
    }
    if linkPlayerCount < *data.at(1) as i32 {
        return;
    }
    SaveLinkPlayers(linkPlayerCount as u8);
    ClearLinkPlayerCountWindow(*data.at(5) as u16);
    ConvertIntToDecimalStringN(
        gStringVar1.as_mut_ptr(),
        linkPlayerCount,
        STR_CONV_MODE_LEFT_ALIGN,
        1,
    );
    ShowFieldAutoScrollMessage(gText_ConfirmStartLinkWithXPlayers.as_ptr().cast_mut());
    gTasks[taskId].func = Some(Task_LinkupTryConfirmation);
}
pub(crate) unsafe extern "C" fn Task_LinkupTryConfirmation(taskId: u8) {
    if CheckLinkCanceledBeforeConnection(taskId) == TRUE as u32
        || CheckSioErrored(taskId) == TRUE as u32
        || CheckLinkErrored(taskId) == TRUE as u32
    {
        return;
    }
    if GetFieldMessageBoxMode() == FIELD_MESSAGE_BOX_HIDDEN {
        if GetSavedPlayerCount() != GetLinkPlayerCount_2() {
            ShowFieldAutoScrollMessage(gText_ConfirmLinkWhenPlayersReady.as_ptr().cast_mut());
            gTasks[taskId].func = Some(Task_LinkupConfirmWhenReady);
        } else if gMain.heldKeys as i32 & B_BUTTON != 0 {
            ShowFieldAutoScrollMessage(gText_ConfirmLinkWhenPlayersReady.as_ptr().cast_mut());
            gTasks[taskId].func = Some(Task_LinkupConfirmWhenReady);
        } else if gMain.heldKeys as i32 & A_BUTTON != 0 {
            PlaySE(SE_SELECT);
            CheckShouldAdvanceLinkState();
            gTasks[taskId].func = Some(Task_LinkupConfirm);
        }
    }
}
pub(crate) unsafe extern "C" fn Task_LinkupConfirm(taskId: u8) {
    let mut minPlayers: u8 = gTasks[taskId].data[1] as u8;
    let mut maxPlayers: u8 = gTasks[taskId].data[2] as u8;
    if CheckLinkErrored(taskId) == TRUE as u32 || TryLinkTimeout(taskId) == TRUE {
        return;
    }
    if GetLinkPlayerCount_2() != GetSavedPlayerCount() {
        gTasks[taskId].func = Some(Task_LinkupConnectionError);
    } else {
        gSpecialVar_Result = ExchangeDataAndGetLinkupStatus(minPlayers, maxPlayers) as u16;
        if gSpecialVar_Result != LINKUP_ONGOING {
            gTasks[taskId].func = Some(Task_LinkupCheckStatusAfterConfirm);
        }
    }
}
pub(crate) unsafe extern "C" fn Task_LinkupExchangeDataWithLeader(taskId: u8) {
    let mut minPlayers: u8 = 0;
    let mut maxPlayers: u8 = 0;
    let mut card: *mut TrainerCard = null_mut();
    minPlayers = gTasks[taskId].data[1] as u8;
    maxPlayers = gTasks[taskId].data[2] as u8;
    if CheckLinkCanceledBeforeConnection(taskId) == TRUE as u32
        || CheckLinkErrored(taskId) == TRUE as u32
    {
        return;
    }
    gSpecialVar_Result = ExchangeDataAndGetLinkupStatus(minPlayers, maxPlayers) as u16;
    if gSpecialVar_Result == LINKUP_ONGOING {
        return;
    }
    if gSpecialVar_Result == LINKUP_DIFF_SELECTIONS
        || gSpecialVar_Result == LINKUP_WRONG_NUM_PLAYERS
    {
        SetCloseLinkCallback();
        HideFieldMessageBox();
        gTasks[taskId].func = Some(Task_StopLinkup);
    } else if gSpecialVar_Result == LINKUP_PLAYER_NOT_READY
        || gSpecialVar_Result == LINKUP_PARTNER_NOT_READY
    {
        CloseLink();
        HideFieldMessageBox();
        gTasks[taskId].func = Some(Task_StopLinkup);
    } else {
        gFieldLinkPlayerCount = GetLinkPlayerCount_2();
        gLocalLinkPlayerId = GetMultiplayerId();
        SaveLinkPlayers(gFieldLinkPlayerCount);
        card = gBlockSendBuffer.as_mut_ptr() as *mut TrainerCard;
        TrainerCard_GenerateCardForLinkPlayer(card);
        (*card).monSpecies[0] = GetMonData3(
            &raw mut gPlayerParty[gSelectedOrderFromParty[0] as i32 - 1],
            MON_DATA_SPECIES,
            null_mut(),
        ) as u16;
        (*card).monSpecies[1] = GetMonData3(
            &raw mut gPlayerParty[gSelectedOrderFromParty[1] as i32 - 1],
            MON_DATA_SPECIES,
            null_mut(),
        ) as u16;
        gTasks[taskId].func = Some(Task_LinkupAwaitTrainerCardData);
    }
}
pub(crate) unsafe extern "C" fn Task_LinkupCheckStatusAfterConfirm(taskId: u8) {
    let mut card: *mut TrainerCard = null_mut();
    if CheckLinkErrored(taskId) == TRUE as u32 {
        return;
    }
    if gSpecialVar_Result == LINKUP_WRONG_NUM_PLAYERS {
        if Link_AnyPartnersPlayingRubyOrSapphire() == 0 {
            SetCloseLinkCallback();
            HideFieldMessageBox();
            gTasks[taskId].func = Some(Task_StopLinkup);
        } else {
            CloseLink();
            HideFieldMessageBox();
            gTasks[taskId].func = Some(Task_StopLinkup);
        }
    } else if gSpecialVar_Result == LINKUP_DIFF_SELECTIONS {
        SetCloseLinkCallback();
        HideFieldMessageBox();
        gTasks[taskId].func = Some(Task_StopLinkup);
    } else if gSpecialVar_Result == LINKUP_PLAYER_NOT_READY
        || gSpecialVar_Result == LINKUP_PARTNER_NOT_READY
    {
        CloseLink();
        HideFieldMessageBox();
        gTasks[taskId].func = Some(Task_StopLinkup);
    } else {
        gFieldLinkPlayerCount = GetLinkPlayerCount_2();
        gLocalLinkPlayerId = GetMultiplayerId();
        SaveLinkPlayers(gFieldLinkPlayerCount);
        card = gBlockSendBuffer.as_mut_ptr() as *mut TrainerCard;
        TrainerCard_GenerateCardForLinkPlayer(card);
        (*card).monSpecies[0] = GetMonData3(
            &raw mut gPlayerParty[gSelectedOrderFromParty[0] as i32 - 1],
            MON_DATA_SPECIES,
            null_mut(),
        ) as u16;
        (*card).monSpecies[1] = GetMonData3(
            &raw mut gPlayerParty[gSelectedOrderFromParty[1] as i32 - 1],
            MON_DATA_SPECIES,
            null_mut(),
        ) as u16;
        gTasks[taskId].func = Some(Task_LinkupAwaitTrainerCardData);
        SendBlockRequest(BLOCK_REQ_SIZE_100);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AreBattleTowerLinkSpeciesSame(
    speciesList1: *mut u16,
    speciesList2: *mut u16,
) -> u32 {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut haveSameSpecies: u32 = FALSE as u32;
    let mut numSameSpecies: i32 = 0;
    gStringVar1[0] = EOS;
    gStringVar2[0] = EOS;
    i = 0;
    while i < FRONTIER_MULTI_PARTY_SIZE {
        j = 0;
        while j < FRONTIER_MULTI_PARTY_SIZE {
            if *speciesList1.at(i) == *speciesList2.at(j) {
                if numSameSpecies == 0 {
                    StringCopy(
                        gStringVar1.as_mut_ptr(),
                        gSpeciesNames[*speciesList1.at(i)].as_ptr().cast_mut(),
                    );
                    haveSameSpecies = TRUE as u32;
                }
                if numSameSpecies == 1 {
                    StringCopy(
                        gStringVar2.as_mut_ptr(),
                        gSpeciesNames[*speciesList1.at(i)].as_ptr().cast_mut(),
                    );
                    haveSameSpecies = TRUE as u32;
                }
                numSameSpecies += 1;
            }
            j += 1;
        }
        i += 1;
    }
    gSpecialVar_0x8005 = numSameSpecies as u16;
    return haveSameSpecies;
}
pub(crate) unsafe extern "C" fn FinishLinkup(linkupStatus: *mut u16, taskId: u32) {
    let mut trainerCards: *mut TrainerCard = gTrainerCards.as_mut_ptr();
    if *linkupStatus == LINKUP_SUCCESS {
        if gLinkType == LINKTYPE_BATTLE_TOWER_50 || gLinkType == LINKTYPE_BATTLE_TOWER_OPEN {
            if AreBattleTowerLinkSpeciesSame(
                (*trainerCards).monSpecies.as_mut_ptr(),
                (*trainerCards.at(1)).monSpecies.as_mut_ptr(),
            ) != 0
            {
                *linkupStatus = LINKUP_FAILED_BATTLE_TOWER;
                SetCloseLinkCallback();
                gTasks[taskId].func = Some(Task_StopLinkup);
            } else {
                ClearLinkPlayerCountWindow(gTasks[taskId].data[5] as u16);
                ScriptContext_Enable();
                DestroyTask(taskId as u8);
            }
        } else {
            ClearLinkPlayerCountWindow(gTasks[taskId].data[5] as u16);
            ScriptContext_Enable();
            DestroyTask(taskId as u8);
        }
    } else {
        SetCloseLinkCallback();
        gTasks[taskId].func = Some(Task_StopLinkup);
    }
}
pub(crate) unsafe extern "C" fn Task_LinkupAwaitTrainerCardData(taskId: u8) {
    let mut index: u8 = 0;
    if CheckLinkErrored(taskId) == TRUE as u32 {
        return;
    }
    if GetBlockReceivedStatus() != GetSavedLinkPlayerCountAsBitFlags() {
        return;
    }
    index = 0;
    while index < GetLinkPlayerCount() {
        CopyTrainerCardData(
            &raw mut gTrainerCards[index],
            gBlockRecvBuffer[index].as_mut_ptr() as *mut TrainerCard,
            gLinkPlayers[index].version as u8,
        );
        index += 1;
    }
    SetSuppressLinkErrorMessage(FALSE);
    ResetBlockReceivedFlags();
    FinishLinkup(&raw mut gSpecialVar_Result, taskId as u32);
}
pub(crate) unsafe extern "C" fn Task_StopLinkup(taskId: u8) {
    if gReceivedRemoteLinkPlayers == 0 {
        ClearLinkPlayerCountWindow(gTasks[taskId].data[5] as u16);
        ScriptContext_Enable();
        RemoveWindow(gTasks[taskId].data[5] as u8);
        DestroyTask(taskId);
    }
}
pub(crate) unsafe extern "C" fn Task_LinkupFailed(taskId: u8) {
    gSpecialVar_Result = LINKUP_FAILED;
    ClearLinkPlayerCountWindow(gTasks[taskId].data[5] as u16);
    StopFieldMessage();
    RemoveWindow(gTasks[taskId].data[5] as u8);
    ScriptContext_Enable();
    DestroyTask(taskId);
}
pub(crate) unsafe extern "C" fn Task_LinkupConnectionError(taskId: u8) {
    gSpecialVar_Result = LINKUP_CONNECTION_ERROR;
    ClearLinkPlayerCountWindow(gTasks[taskId].data[5] as u16);
    RemoveWindow(gTasks[taskId].data[5] as u8);
    HideFieldMessageBox();
    ScriptContext_Enable();
    DestroyTask(taskId);
}
pub(crate) unsafe extern "C" fn TryLinkTimeout(taskId: u8) -> u8 {
    gTasks[taskId].data[4] += 1;
    if gTasks[taskId].data[4] > 600 {
        gTasks[taskId].func = Some(Task_LinkupConnectionError);
        return TRUE;
    }
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn TryBattleLinkup() {
    let mut minPlayers: u8 = 2;
    let mut maxPlayers: u8 = 2;
    match gSpecialVar_0x8004 {
        USING_SINGLE_BATTLE => {
            minPlayers = 2;
            gLinkType = LINKTYPE_SINGLE_BATTLE;
        }
        USING_DOUBLE_BATTLE => {
            minPlayers = 2;
            gLinkType = LINKTYPE_DOUBLE_BATTLE;
        }
        USING_MULTI_BATTLE => {
            minPlayers = 4;
            maxPlayers = 4;
            gLinkType = LINKTYPE_MULTI_BATTLE;
        }
        USING_BATTLE_TOWER => {
            minPlayers = 2;
            if (*gSaveBlock2Ptr).frontier.lvlMode() == FRONTIER_LVL_50 {
                gLinkType = LINKTYPE_BATTLE_TOWER_50;
            } else {
                gLinkType = LINKTYPE_BATTLE_TOWER_OPEN;
            }
        }
        _ => {}
    }
    CreateLinkupTask(minPlayers, maxPlayers);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn TryTradeLinkup() {
    gLinkType = LINKTYPE_TRADE_SETUP;
    gBattleTypeFlags = 0;
    CreateLinkupTask(2, 2);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn TryRecordMixLinkup() {
    gSpecialVar_Result = LINKUP_ONGOING;
    gLinkType = LINKTYPE_RECORD_MIX_BEFORE;
    gBattleTypeFlags = 0;
    CreateLinkupTask(2, 4);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ValidateMixingGameLanguage() {
    let mut taskId: u32 = FindTaskIdByFunc(Some(Task_ValidateMixingGameLanguage)) as u32;
    if taskId == TASK_NONE as u32 {
        taskId = CreateTask(Some(Task_ValidateMixingGameLanguage), 80) as u32;
        gTasks[taskId].data[0] = 0;
    }
}
pub(crate) unsafe extern "C" fn Task_ValidateMixingGameLanguage(taskId: u8) {
    let mut playerCount: i32 = 0;
    let mut i: i32 = 0;
    match gTasks[taskId].data[0] {
        0 => {
            if gSpecialVar_Result == LINKUP_SUCCESS {
                let mut mixingForeignGames: u32 = FALSE as u32;
                let mut isEnglishRSLinked: u32 = FALSE as u32;
                let mut isJapaneseEmeraldLinked: u32 = FALSE as u32;
                playerCount = GetLinkPlayerCount() as i32;
                i = 0;
                while i < playerCount {
                    let mut version: u32 = gLinkPlayers[i].version as u8 as u32;
                    let mut language: u32 = gLinkPlayers[i].language as u32;
                    if version == VERSION_RUBY as u32 || version == VERSION_SAPPHIRE as u32 {
                        if language == LANGUAGE_JAPANESE as u32 {
                            mixingForeignGames = TRUE as u32;
                            break;
                        } else {
                            isEnglishRSLinked = TRUE as u32;
                        }
                    } else if version == VERSION_EMERALD as u32 {
                        if language == LANGUAGE_JAPANESE as u32 {
                            isJapaneseEmeraldLinked = TRUE as u32;
                        }
                    }
                    i += 1;
                }
                if isEnglishRSLinked != 0 && isJapaneseEmeraldLinked != 0 {
                    mixingForeignGames = TRUE as u32;
                }
                if mixingForeignGames != 0 {
                    gSpecialVar_Result = LINKUP_FOREIGN_GAME;
                    SetCloseLinkCallbackHandleJP();
                    gTasks[taskId].data[0] = 1;
                    return;
                }
            }
            ScriptContext_Enable();
            DestroyTask(taskId);
        }
        1 => {
            if gReceivedRemoteLinkPlayers == 0 {
                ScriptContext_Enable();
                DestroyTask(taskId);
            }
        }
        _ => {}
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn TryBerryBlenderLinkup() {
    gLinkType = LINKTYPE_BERRY_BLENDER_SETUP;
    gBattleTypeFlags = 0;
    CreateLinkupTask(2, 4);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn TryContestGModeLinkup() {
    gLinkType = LINKTYPE_CONTEST_GMODE;
    gBattleTypeFlags = 0;
    CreateLinkupTask(4, 4);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn TryContestEModeLinkup() {
    gLinkType = LINKTYPE_CONTEST_EMODE;
    gBattleTypeFlags = 0;
    CreateLinkupTask(2, 4);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CreateTask_ReestablishCableClubLink() -> u8 {
    if FuncIsActiveTask(Some(Task_ReestablishLink)) != FALSE {
        return TASK_NONE;
    }
    match gSpecialVar_0x8004 {
        USING_SINGLE_BATTLE => {
            gLinkType = LINKTYPE_SINGLE_BATTLE;
        }
        USING_DOUBLE_BATTLE => {
            gLinkType = LINKTYPE_DOUBLE_BATTLE;
        }
        USING_MULTI_BATTLE => {
            gLinkType = LINKTYPE_MULTI_BATTLE;
        }
        USING_BATTLE_TOWER => {
            if (*gSaveBlock2Ptr).frontier.lvlMode() == FRONTIER_LVL_50 {
                gLinkType = LINKTYPE_BATTLE_TOWER_50;
            } else {
                gLinkType = LINKTYPE_BATTLE_TOWER_OPEN;
            }
        }
        USING_TRADE_CENTER => {
            gLinkType = LINKTYPE_TRADE;
        }
        USING_RECORD_CORNER => {
            gLinkType = LINKTYPE_RECORD_MIX_AFTER;
        }
        _ => {}
    }
    return CreateTask(Some(Task_ReestablishLink), 80);
}
pub(crate) unsafe extern "C" fn Task_ReestablishLink(taskId: u8) {
    let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
    if *data == 0 {
        OpenLink();
        ResetLinkPlayers();
        CreateTask(Some(Task_WaitForLinkPlayerConnection), 80);
    } else if *data >= 10 {
        gTasks[taskId].func = Some(Task_ReestablishLinkAwaitConnection);
    }
    *data += 1;
}
pub(crate) unsafe extern "C" fn Task_ReestablishLinkAwaitConnection(taskId: u8) {
    if GetLinkPlayerCount_2() >= 2 {
        if IsLinkMaster() == TRUE {
            gTasks[taskId].func = Some(Task_ReestablishLinkLeader);
        } else {
            gTasks[taskId].func = Some(Task_ReestablishLinkAwaitConfirmation);
        }
    }
}
pub(crate) unsafe extern "C" fn Task_ReestablishLinkLeader(taskId: u8) {
    if GetSavedPlayerCount() == GetLinkPlayerCount_2() {
        CheckShouldAdvanceLinkState();
        gTasks[taskId].func = Some(Task_ReestablishLinkAwaitConfirmation);
    }
}
pub(crate) unsafe extern "C" fn Task_ReestablishLinkAwaitConfirmation(taskId: u8) {
    if gReceivedRemoteLinkPlayers == TRUE && IsLinkPlayerDataExchangeComplete() == TRUE {
        CheckLinkPlayersMatchSaved();
        StartSendingKeysToLink();
        DestroyTask(taskId);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CableClubSaveGame() {
    SaveGame();
}
pub(crate) unsafe extern "C" fn SetLinkBattleTypeFlags(linkService: i32) {
    match linkService {
        1 => {
            gBattleTypeFlags = 10;
        }
        2 => {
            gBattleTypeFlags = 11;
        }
        5 => {
            ReducePlayerPartyToSelectedMons();
            gBattleTypeFlags = 75;
        }
        9 => {
            gBattleTypeFlags = 331;
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn Task_StartWiredCableClubBattle(taskId: u8) {
    let mut task: *mut Task = &raw mut gTasks[taskId];
    match (*task).data[0] {
        0 => {
            FadeScreen(FADE_TO_BLACK, 0);
            gLinkType = LINKTYPE_BATTLE;
            ClearLinkCallback_2();
            (*task).data[0] += 1;
        }
        1 => {
            if gPaletteFade.active() == 0 {
                (*task).data[0] += 1;
            }
        }
        2 => {
            (*task).data[1] += 1;
            if (*task).data[1] > 20 {
                (*task).data[0] += 1;
            }
        }
        3 => {
            SetCloseLinkCallback();
            (*task).data[0] += 1;
        }
        4 => {
            if gReceivedRemoteLinkPlayers == 0 {
                (*task).data[0] += 1;
            }
        }
        5 => {
            if gLinkPlayers[0].trainerId & 1 != 0 {
                PlayMapChosenOrBattleBGM(MUS_VS_GYM_LEADER);
            } else {
                PlayMapChosenOrBattleBGM(MUS_VS_TRAINER);
            }
            SetLinkBattleTypeFlags(gSpecialVar_0x8004 as i32);
            CleanupOverworldWindowsAndTilemaps();
            gTrainerBattleOpponent_A = TRAINER_LINK_OPPONENT;
            SetMainCallback2(Some(CB2_InitBattle));
            gMain.savedCallback = Some(CB2_ReturnFromCableClubBattle);
            DestroyTask(taskId);
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn Task_StartWirelessCableClubBattle(taskId: u8) {
    let mut i: i32 = 0;
    let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
    match *data {
        0 => {
            FadeScreen(FADE_TO_BLACK, 0);
            gLinkType = LINKTYPE_BATTLE;
            ClearLinkCallback_2();
            *data = 1;
        }
        1 => {
            if gPaletteFade.active() == 0 {
                *data = 2;
            }
        }
        2 => {
            SendBlock(0, &raw mut gLocalLinkPlayer as *mut c_void, 28);
            *data = 3;
        }
        3 => {
            if GetBlockReceivedStatus() == GetLinkPlayerCountAsBitFlags() {
                i = 0;
                while i < GetLinkPlayerCount() as i32 {
                    let mut player: *mut LinkPlayer =
                        gBlockRecvBuffer[i].as_mut_ptr() as *mut LinkPlayer;
                    gLinkPlayers[i] = *player;
                    ConvertLinkPlayerName(&raw mut gLinkPlayers[i]);
                    ResetBlockReceivedFlag(i as u8);
                    i += 1;
                }
                *data = 4;
            }
        }
        4 => {
            *data.at(1) += 1;
            if *data.at(1) > 20 {
                *data = 5;
            }
        }
        5 => {
            SetLinkStandbyCallback();
            *data = 6;
        }
        6 => {
            if IsLinkTaskFinished() != 0 {
                *data = 7;
            }
        }
        7 => {
            if gLinkPlayers[0].trainerId & 1 != 0 {
                PlayMapChosenOrBattleBGM(MUS_VS_GYM_LEADER);
            } else {
                PlayMapChosenOrBattleBGM(MUS_VS_TRAINER);
            }
            gLinkPlayers[0].linkType = LINKTYPE_BATTLE as u32;
            SetLinkBattleTypeFlags(gSpecialVar_0x8004 as i32);
            CleanupOverworldWindowsAndTilemaps();
            gTrainerBattleOpponent_A = TRAINER_LINK_OPPONENT;
            SetMainCallback2(Some(CB2_InitBattle));
            gMain.savedCallback = Some(CB2_ReturnFromCableClubBattle);
            DestroyTask(taskId);
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn CB2_ReturnFromUnionRoomBattle() {
    let mut playerCount: u8 = 0;
    let mut i: i32 = 0;
    let mut linkedWithFRLG: u32 = 0;
    match gMain.state {
        0 => {
            playerCount = GetLinkPlayerCount();
            linkedWithFRLG = FALSE as u32;
            i = 0;
            while i < playerCount as i32 {
                let mut version: u32 = gLinkPlayers[i].version as u8 as u32;
                if version == VERSION_FIRE_RED as u32 || version == VERSION_LEAF_GREEN as u32 {
                    linkedWithFRLG = TRUE as u32;
                    break;
                }
                i += 1;
            }
            if linkedWithFRLG != 0 {
                gMain.state = 2;
            } else {
                SetCloseLinkCallback();
                gMain.state = 1;
            }
        }
        1 => {
            if gReceivedRemoteLinkPlayers == 0 {
                SetMainCallback2(Some(CB2_ReturnToField));
            }
        }
        2 => {
            SetMainCallback2(Some(CB2_ReturnToField));
        }
        _ => {}
    }
    RunTasks();
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CB2_ReturnFromCableClubBattle() {
    gBattleTypeFlags &= 0xffffffdf;
    Overworld_ResetMapMusic();
    LoadPlayerParty();
    SavePlayerBag();
    UpdateTrainerFansAfterLinkBattle();
    if gSpecialVar_0x8004 == USING_SINGLE_BATTLE || gSpecialVar_0x8004 == USING_DOUBLE_BATTLE {
        UpdatePlayerLinkBattleRecords(gLocalLinkPlayerId as i32 ^ 1);
        if gWirelessCommType != 0 {
            match gBattleOutcome {
                B_OUTCOME_WON => {
                    MysteryGift_TryIncrementStat(
                        CARD_STAT_BATTLES_WON,
                        gLinkPlayers[GetMultiplayerId() as i32 ^ 1].trainerId,
                    );
                }
                B_OUTCOME_LOST => {
                    MysteryGift_TryIncrementStat(
                        1,
                        gLinkPlayers[GetMultiplayerId() as i32 ^ 1].trainerId,
                    );
                }
                _ => {}
            }
        }
    }
    if InUnionRoom() == TRUE as u32 {
        gMain.savedCallback = Some(CB2_ReturnFromUnionRoomBattle);
    } else {
        gMain.savedCallback = Some(CB2_ReturnToFieldFromMultiplayer);
    }
    SetMainCallback2(Some(CB2_SetUpSaveAfterLinkBattle));
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CleanupLinkRoomState() {
    if gSpecialVar_0x8004 == USING_SINGLE_BATTLE
        || gSpecialVar_0x8004 == USING_DOUBLE_BATTLE
        || gSpecialVar_0x8004 == USING_MULTI_BATTLE
        || gSpecialVar_0x8004 == USING_BATTLE_TOWER
    {
        LoadPlayerParty();
        SavePlayerBag();
    }
    SetWarpDestinationToDynamicWarp(WARP_ID_DYNAMIC);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ExitLinkRoom() {
    QueueExitLinkRoomKey();
}
pub(crate) unsafe extern "C" fn Task_EnterCableClubSeat(taskId: u8) {
    let mut task: *mut Task = &raw mut gTasks[taskId];
    match (*task).data[0] {
        0 => {
            ShowFieldMessage(gText_PleaseWaitForLink.as_ptr().cast_mut());
            (*task).data[0] = 1;
        }
        1 => {
            if IsFieldMessageBoxHidden() != 0 {
                SetInCableClubSeat();
                SetLocalLinkPlayerId(gSpecialVar_0x8005 as u8);
                (*task).data[0] = 2;
            }
        }
        2 => match GetCableClubPartnersReady() {
            CABLE_SEAT_WAITING => {}
            CABLE_SEAT_SUCCESS => {
                HideFieldMessageBox();
                (*task).data[0] = 0;
                SetStartedCableClubActivity();
                SwitchTaskToFollowupFunc(taskId);
            }
            CABLE_SEAT_FAILED => {
                (*task).data[0] = 3;
            }
            _ => {}
        },
        3 => {
            SetLinkWaitingForScript();
            EraseFieldMessageBox(TRUE);
            DestroyTask(taskId);
            ScriptContext_Enable();
        }
        _ => {}
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CreateTask_EnterCableClubSeat(
    followupFunc: Option<unsafe extern "C" fn(u8)>,
) {
    let mut taskId: u8 = CreateTask(Some(Task_EnterCableClubSeat), 80);
    SetTaskFuncWithFollowupFunc(taskId, Some(Task_EnterCableClubSeat), followupFunc);
    ScriptContext_Stop();
}
pub(crate) unsafe extern "C" fn Task_StartWiredTrade(taskId: u8) {
    let mut task: *mut Task = &raw mut gTasks[taskId];
    match (*task).data[0] {
        0 => {
            LockPlayerFieldControls();
            FadeScreen(FADE_TO_BLACK, 0);
            ClearLinkCallback_2();
            (*task).data[0] += 1;
        }
        1 => {
            if gPaletteFade.active() == 0 {
                (*task).data[0] += 1;
            }
        }
        2 => {
            gSelectedTradeMonPositions[0] = 0;
            gSelectedTradeMonPositions[1] = 0;
            m4aMPlayAllStop();
            SetCloseLinkCallback();
            (*task).data[0] += 1;
        }
        3 => {
            if gReceivedRemoteLinkPlayers == 0 {
                SetMainCallback2(Some(CB2_StartCreateTradeMenu));
                DestroyTask(taskId);
            }
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn Task_StartWirelessTrade(taskId: u8) {
    let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
    match *data {
        0 => {
            LockPlayerFieldControls();
            FadeScreen(FADE_TO_BLACK, 0);
            ClearLinkRfuCallback();
            *data += 1;
        }
        1 => {
            if gPaletteFade.active() == 0 {
                *data += 1;
            }
        }
        2 => {
            gSelectedTradeMonPositions[0] = 0;
            gSelectedTradeMonPositions[1] = 0;
            m4aMPlayAllStop();
            SetLinkStandbyCallback();
            *data += 1;
        }
        3 => {
            if IsLinkTaskFinished() != 0 {
                CreateTask_CreateTradeMenu();
                DestroyTask(taskId);
            }
        }
        _ => {}
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PlayerEnteredTradeSeat() {
    if gWirelessCommType != 0 {
        CreateTask_EnterCableClubSeat(Some(Task_StartWirelessTrade));
    } else {
        CreateTask_EnterCableClubSeat(Some(Task_StartWiredTrade));
    }
}
pub(crate) unsafe extern "C" fn CreateTask_StartWiredTrade() {
    CreateTask(Some(Task_StartWiredTrade), 80);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Script_StartWiredTrade() {}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ColosseumPlayerSpotTriggered() {
    gLinkType = LINKTYPE_BATTLE;
    if gWirelessCommType != 0 {
        CreateTask_EnterCableClubSeat(Some(Task_StartWirelessCableClubBattle));
    } else {
        CreateTask_EnterCableClubSeat(Some(Task_StartWiredCableClubBattle));
    }
}
pub(crate) unsafe extern "C" fn CreateTask_EnterCableClubSeatNoFollowup() {
    let mut taskId: u8 = CreateTask(Some(Task_EnterCableClubSeat), 80);
    ScriptContext_Stop();
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Script_ShowLinkTrainerCard() {
    ShowTrainerCardInLink(
        gSpecialVar_0x8006 as u8,
        Some(CB2_ReturnToFieldContinueScriptPlayMapMusic),
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetLinkTrainerCardColor(linkPlayerIndex: u8) -> u32 {
    let mut numStars: u32 = 0;
    gSpecialVar_0x8006 = linkPlayerIndex as u16;
    StringCopy(
        gStringVar1.as_mut_ptr(),
        gLinkPlayers[linkPlayerIndex].name.as_mut_ptr(),
    );
    numStars = GetTrainerCardStars(linkPlayerIndex) as u32;
    if numStars == 0 {
        return FALSE as u32;
    }
    StringCopy(
        gStringVar2.as_mut_ptr(),
        sTrainerCardColorNames[numStars - 1],
    );
    return TRUE as u32;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Task_WaitForLinkPlayerConnection(taskId: u8) {
    let mut task: *mut Task = &raw mut gTasks[taskId];
    (*task).data[0] += 1;
    if (*task).data[0] > 300 {
        CloseLink();
        SetMainCallback2(Some(CB2_LinkError));
        DestroyTask(taskId);
    }
    if gReceivedRemoteLinkPlayers != 0 {
        if gWirelessCommType == 0 {
            if DoesLinkPlayerCountMatchSaved() == 0 {
                CloseLink();
                SetMainCallback2(Some(CB2_LinkError));
            }
            DestroyTask(taskId);
        } else {
            DestroyTask(taskId);
        }
    }
}
pub(crate) unsafe extern "C" fn Task_WaitExitToScript(taskId: u8) {
    if gReceivedRemoteLinkPlayers == 0 {
        ScriptContext_Enable();
        DestroyTask(taskId);
    }
}
pub(crate) unsafe extern "C" fn ExitLinkToScript(taskId: u8) {
    SetCloseLinkCallback();
    gTasks[taskId].func = Some(Task_WaitExitToScript);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Task_ReconnectWithLinkPlayers(taskId: u8) {
    let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
    match *data {
        0 => {
            if gWirelessCommType != 0 {
                DestroyTask(taskId);
            } else {
                OpenLink();
                CreateTask(Some(Task_WaitForLinkPlayerConnection), 1);
                *data += 1;
            }
        }
        1 => {
            if ({
                *data.at(1) += 1;
                *data.at(1)
            }) > 11
            {
                *data.at(1) = 0;
                *data += 1;
            }
        }
        2 => {
            if GetLinkPlayerCount_2() >= GetSavedPlayerCount() {
                if IsLinkMaster() != 0 {
                    if ({
                        *data.at(1) += 1;
                        *data.at(1)
                    }) > 30
                    {
                        CheckShouldAdvanceLinkState();
                        *data += 1;
                    }
                } else {
                    *data += 1;
                }
            }
        }
        3 => {
            if gReceivedRemoteLinkPlayers == TRUE && IsLinkPlayerDataExchangeComplete() == TRUE {
                DestroyTask(taskId);
            }
        }
        _ => {}
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn TrySetBattleTowerLinkType() {
    if gWirelessCommType == 0 {
        gLinkType = LINKTYPE_BATTLE_TOWER;
    }
}
