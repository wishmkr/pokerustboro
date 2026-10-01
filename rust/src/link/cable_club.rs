//! Translated from `src/cable_club.c` by tools/rustport/c2rs.py.
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
    clippy::useless_transmute,
    dead_code,
    unused_assignments,
    unused_variables
)]

use crate::agb_main::gMain;
use crate::battle_main::{CB2_InitBattle, gBattleOutcome, gBattleTypeFlags};
use crate::battle_records::UpdatePlayerLinkBattleRecords;
use crate::battle_setup::gTrainerBattleOpponent_A;
#[allow(unused_imports)]
use crate::c::*;
#[allow(unused_imports)]
use crate::consts::*;
use crate::ffi::{gSpecialVar_0x8004, gSpecialVar_0x8005, gSpecialVar_0x8006, gSpecialVar_Result};
use crate::field_message_box::{
    GetFieldMessageBoxMode, HideFieldMessageBox, IsFieldMessageBoxHidden, StopFieldMessage,
};
use crate::field_message_box::{ShowFieldAutoScrollMessage, ShowFieldMessage};
use crate::field_specials::UpdateTrainerFansAfterLinkBattle;
use crate::field_weather::FadeScreen;
use crate::international_string_util::GetStringCenterAlignXOffset;
use crate::link::{
    CB2_LinkError, CheckLinkPlayersMatchSaved, CheckShouldAdvanceLinkState, ClearLinkCallback_2,
    CloseLink, ConvertLinkPlayerName, DoesLinkPlayerCountMatchSaved, GetBlockReceivedStatus,
    GetLinkPlayerCount, GetLinkPlayerCount_2, GetLinkPlayerCountAsBitFlags,
    GetLinkPlayerDataExchangeStatusTimed, GetMultiplayerId, GetSavedLinkPlayerCountAsBitFlags,
    GetSavedPlayerCount, GetSioMultiSI, HasLinkErrorOccurred, IsLinkConnectionEstablished,
    IsLinkMaster, IsLinkPlayerDataExchangeComplete, IsLinkTaskFinished,
    Link_AnyPartnersPlayingRubyOrSapphire, OpenLink, OpenLinkTimed, ResetBlockReceivedFlag,
    ResetBlockReceivedFlags, ResetLinkPlayerCount, ResetLinkPlayers, SaveLinkPlayers, SendBlock,
    SendBlockRequest, SetCloseLinkCallback, SetCloseLinkCallbackHandleJP, SetLinkStandbyCallback,
    SetLocalLinkPlayerId, SetSuppressLinkErrorMessage, StartSendingKeysToLink, gLinkPlayers,
    gLinkType, gLocalLinkPlayer, gReceivedRemoteLinkPlayers, gWirelessCommType,
};
use crate::link::{gBlockRecvBuffer, gBlockSendBuffer};
use crate::link_rfu_2::ClearLinkRfuCallback;
use crate::load_save::gSaveBlock2Ptr;
use crate::load_save::{LoadPlayerParty, SavePlayerBag};
use crate::m4a::m4aMPlayAllStop;
use crate::menu::{ClearStdWindowAndFrame, EraseFieldMessageBox, SetStandardWindowBorderStyle};
use crate::mystery_gift::MysteryGift_TryIncrementStat;
use crate::overworld::{
    CB2_ReturnToField, CB2_ReturnToFieldContinueScriptPlayMapMusic,
    CB2_ReturnToFieldFromMultiplayer, CleanupOverworldWindowsAndTilemaps,
    GetCableClubPartnersReady, Overworld_ResetMapMusic, QueueExitLinkRoomKey, SetInCableClubSeat,
    SetLinkWaitingForScript, SetStartedCableClubActivity, SetWarpDestinationToDynamicWarp,
    gFieldLinkPlayerCount, gLocalLinkPlayerId,
};
use crate::palette::gPaletteFade;
use crate::party_menu::gSelectedOrderFromParty;
use crate::pokemon::{GetMonData3, PlayMapChosenOrBattleBGM, gPlayerParty};
use crate::script::{LockPlayerFieldControls, ScriptContext_Enable, ScriptContext_Stop};
use crate::script_pokemon_util::ReducePlayerPartyToSelectedMons;
use crate::sound::PlaySE;
use crate::start_menu::{CB2_SetUpSaveAfterLinkBattle, SaveGame};
use crate::string_util::{ConvertIntToDecimalStringN, StringCopy, StringExpandPlaceholders};
use crate::string_util::{gStringVar1, gStringVar2, gStringVar4};
use crate::task::gTasks;
use crate::task::{DestroyTask, RunTasks, SwitchTaskToFollowupFunc};
use crate::task::{task_get, task_set, task_set_func};
use crate::trade::CB2_StartCreateTradeMenu;
use crate::trade::gSelectedTradeMonPositions;
use crate::trainer_card::{
    CopyTrainerCardData, GetTrainerCardStars, ShowTrainerCardInLink,
    TrainerCard_GenerateCardForLinkPlayer, gTrainerCards,
};
#[allow(unused_imports)]
use crate::types::*;
use crate::union_room::{CreateTask_CreateTradeMenu, InUnionRoom};
use crate::window::{CopyWindowToVram, RemoveWindow};
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
/// `SetTaskFuncWithFollowupFunc` with this module's view of its types.
#[inline]
unsafe fn SetTaskFuncWithFollowupFunc(
    a0: u8,
    a1: Option<unsafe fn(u8)>,
    a2: Option<unsafe fn(u8)>,
) {
    unsafe {
        crate::task::SetTaskFuncWithFollowupFunc(
            a0,
            core::mem::transmute(a1),
            core::mem::transmute(a2),
        );
    }
}
// The C's names for task and sprite data slots.
const tState: usize = 0;
const tMinPlayers: usize = 1;
const tMaxPlayers: usize = 2;
const tNumPlayers: usize = 3;
const tWindowId: usize = 5;
// Data tables (translate with cdata.py): sWindowTemplate_LinkPlayerCount sTrainerCardColorNames

static sTrainerCardColorNames: Table<CArray<*mut u8, 4>> =
    Table((&raw const crate::data::cable_club::sTrainerCardColorNames).cast());
static sWindowTemplate_LinkPlayerCount: Table<WindowTemplate> =
    Table((&raw const crate::data::cable_club::sWindowTemplate_LinkPlayerCount).cast());

/// `AddTextPrinterParameterized` with this module's view of its types.
#[inline]
unsafe fn AddTextPrinterParameterized(
    a0: u8,
    a1: u8,
    a2: *mut u8,
    a3: u8,
    a4: u8,
    a5: u8,
    a6: Option<unsafe fn(*mut TextPrinterTemplate, u16)>,
) -> u16 {
    unsafe {
        crate::text::AddTextPrinterParameterized(
            a0,
            a1,
            a2 as _,
            a3,
            a4,
            a5,
            core::mem::transmute(a6),
        )
    }
}
/// `SetMainCallback2` with this module's view of its types.
#[inline]
unsafe fn SetMainCallback2(a0: Option<unsafe fn()>) {
    unsafe {
        crate::agb_main::SetMainCallback2(core::mem::transmute(a0));
    }
}

unsafe fn CreateLinkupTask(minPlayers: u8, maxPlayers: u8) {
    if FindTaskIdByFunc(Some(Task_LinkupStart)) == TASK_NONE {
        let taskId1: u8 = CreateTask(Some(Task_LinkupStart), 80);
        task_set(taskId1, tMinPlayers, minPlayers as i16);
        task_set(taskId1, tMaxPlayers, maxPlayers as i16);
    }
}
unsafe fn PrintNumPlayersInLink(windowId: u16, numPlayers: u32) {
    ConvertIntToDecimalStringN(
        gStringVar1.as_mut_ptr(),
        numPlayers as i32,
        STR_CONV_MODE_LEFT_ALIGN,
        1,
    );
    SetStandardWindowBorderStyle(windowId as u8, FALSE);
    StringExpandPlaceholders(
        gStringVar4.as_mut_ptr(),
        (*(&raw const crate::data::strings::gText_NumPlayerLink).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
    );
    let xPos: u8 =
        GetStringCenterAlignXOffset(FONT_NORMAL as i32, gStringVar4.as_mut_ptr(), 88) as u8;
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
unsafe fn ClearLinkPlayerCountWindow(windowId: u16) {
    ClearStdWindowAndFrame(windowId as u8, FALSE);
    CopyWindowToVram(windowId as u8, COPYWIN_FULL);
}
unsafe fn UpdateLinkPlayerCountDisplay(taskId: u8, numPlayers: u8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    if numPlayers as i16 != *data.at(3) {
        if numPlayers <= 1 {
            ClearLinkPlayerCountWindow(*data.at(5) as u16);
        } else {
            PrintNumPlayersInLink(*data.at(5) as u16, numPlayers as u32);
        }
        *data.at(3) = numPlayers as i16;
    }
}
unsafe fn ExchangeDataAndGetLinkupStatus(minPlayers: u8, maxPlayers: u8) -> u32 {
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
        0
    }
}
fn CheckLinkErrored(taskId: u8) -> u32 {
    if HasLinkErrorOccurred() == TRUE {
        task_set_func(taskId, Some(Task_LinkupConnectionError));
        return TRUE as u32;
    }
    FALSE as u32
}
unsafe fn CheckLinkCanceledBeforeConnection(taskId: u8) -> u32 {
    if gMain.newKeys as i32 & B_BUTTON != 0 && IsLinkConnectionEstablished() == FALSE {
        gLinkType = 0;
        task_set_func(taskId, Some(Task_LinkupFailed));
        return TRUE as u32;
    }
    FALSE as u32
}
unsafe fn CheckLinkCanceled(taskId: u8) -> u32 {
    if IsLinkConnectionEstablished() != 0 {
        SetSuppressLinkErrorMessage(TRUE);
    }
    if gMain.newKeys as i32 & B_BUTTON != 0 {
        gLinkType = 0;
        task_set_func(taskId, Some(Task_LinkupFailed));
        return TRUE as u32;
    }
    FALSE as u32
}
unsafe fn CheckSioErrored(taskId: u8) -> u32 {
    if GetSioMultiSI() == TRUE {
        task_set_func(taskId, Some(Task_LinkupConnectionError));
        return TRUE as u32;
    }
    FALSE as u32
}
unsafe fn Task_DelayedBlockRequest(taskId: u8) {
    task_set(taskId, 0, task_get(taskId, 0) + 1);
    if task_get(taskId, 0) == 10 {
        SendBlockRequest(BLOCK_REQ_SIZE_100);
        DestroyTask(taskId);
    }
}
pub(crate) unsafe fn Task_LinkupStart(taskId: u8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    if *data == 0 {
        OpenLinkTimed();
        ResetLinkPlayerCount();
        ResetLinkPlayers();
        *data.at(5) = AddWindow((&raw const *sWindowTemplate_LinkPlayerCount).cast_mut()) as i16;
    } else if *data > 9 {
        task_set_func(taskId, Some(Task_LinkupAwaitConnection));
    }
    *data += 1;
}
pub(crate) unsafe fn Task_LinkupAwaitConnection(taskId: u8) {
    let playerCount: u32 = GetLinkPlayerCount_2() as u32;
    if CheckLinkCanceledBeforeConnection(taskId) == TRUE as u32
        || CheckLinkCanceled(taskId) == TRUE as u32
        || playerCount < 2
    {
        return;
    }
    SetSuppressLinkErrorMessage(TRUE);
    task_set(taskId, 3, 0);
    if IsLinkMaster() == TRUE {
        PlaySE(SE_PIN);
        ShowFieldAutoScrollMessage(
            (*crate::asmdata::gText_ConfirmLinkWhenPlayersReady.cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
        );
        task_set_func(taskId, Some(Task_LinkupConfirmWhenReady));
    } else {
        PlaySE(SE_BOO);
        ShowFieldAutoScrollMessage(
            (*crate::asmdata::gText_AwaitingLinkup.cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
        );
        task_set_func(taskId, Some(Task_LinkupExchangeDataWithLeader));
    }
}
pub(crate) unsafe fn Task_LinkupConfirmWhenReady(taskId: u8) {
    if CheckLinkCanceledBeforeConnection(taskId) == TRUE as u32
        || CheckSioErrored(taskId) == TRUE as u32
        || CheckLinkErrored(taskId) == TRUE as u32
    {
        return;
    }
    if GetFieldMessageBoxMode() == FIELD_MESSAGE_BOX_HIDDEN {
        task_set(taskId, tNumPlayers, 0);
        task_set_func(taskId, Some(Task_LinkupAwaitConfirmation));
    }
}
pub(crate) unsafe fn Task_LinkupAwaitConfirmation(taskId: u8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    let linkPlayerCount: i32 = GetLinkPlayerCount_2() as i32;
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
    ShowFieldAutoScrollMessage(
        (*crate::asmdata::gText_ConfirmStartLinkWithXPlayers.cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
    );
    task_set_func(taskId, Some(Task_LinkupTryConfirmation));
}
pub(crate) unsafe fn Task_LinkupTryConfirmation(taskId: u8) {
    if CheckLinkCanceledBeforeConnection(taskId) == TRUE as u32
        || CheckSioErrored(taskId) == TRUE as u32
        || CheckLinkErrored(taskId) == TRUE as u32
    {
        return;
    }
    if GetFieldMessageBoxMode() == FIELD_MESSAGE_BOX_HIDDEN {
        if GetSavedPlayerCount() != GetLinkPlayerCount_2() {
            ShowFieldAutoScrollMessage(
                (*crate::asmdata::gText_ConfirmLinkWhenPlayersReady.cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut(),
            );
            task_set_func(taskId, Some(Task_LinkupConfirmWhenReady));
        } else if gMain.heldKeys as i32 & B_BUTTON != 0 {
            ShowFieldAutoScrollMessage(
                (*crate::asmdata::gText_ConfirmLinkWhenPlayersReady.cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut(),
            );
            task_set_func(taskId, Some(Task_LinkupConfirmWhenReady));
        } else if gMain.heldKeys as i32 & A_BUTTON != 0 {
            PlaySE(SE_SELECT);
            CheckShouldAdvanceLinkState();
            task_set_func(taskId, Some(Task_LinkupConfirm));
        }
    }
}
pub(crate) unsafe fn Task_LinkupConfirm(taskId: u8) {
    let minPlayers: u8 = task_get(taskId, tMinPlayers) as u8;
    let maxPlayers: u8 = task_get(taskId, tMaxPlayers) as u8;
    if CheckLinkErrored(taskId) == TRUE as u32 || TryLinkTimeout(taskId) == TRUE {
        return;
    }
    if GetLinkPlayerCount_2() != GetSavedPlayerCount() {
        task_set_func(taskId, Some(Task_LinkupConnectionError));
    } else {
        gSpecialVar_Result = ExchangeDataAndGetLinkupStatus(minPlayers, maxPlayers) as u16;
        if gSpecialVar_Result != LINKUP_ONGOING {
            task_set_func(taskId, Some(Task_LinkupCheckStatusAfterConfirm));
        }
    }
}
pub(crate) unsafe fn Task_LinkupExchangeDataWithLeader(taskId: u8) {
    let mut card: *mut TrainerCard = null_mut();
    let minPlayers: u8 = task_get(taskId, tMinPlayers) as u8;
    let maxPlayers: u8 = task_get(taskId, tMaxPlayers) as u8;
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
        task_set_func(taskId, Some(Task_StopLinkup));
    } else if gSpecialVar_Result == LINKUP_PLAYER_NOT_READY
        || gSpecialVar_Result == LINKUP_PARTNER_NOT_READY
    {
        CloseLink();
        HideFieldMessageBox();
        task_set_func(taskId, Some(Task_StopLinkup));
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
        task_set_func(taskId, Some(Task_LinkupAwaitTrainerCardData));
    }
}
pub(crate) unsafe fn Task_LinkupCheckStatusAfterConfirm(taskId: u8) {
    let mut card: *mut TrainerCard = null_mut();
    if CheckLinkErrored(taskId) == TRUE as u32 {
        return;
    }
    if gSpecialVar_Result == LINKUP_WRONG_NUM_PLAYERS {
        if Link_AnyPartnersPlayingRubyOrSapphire() == 0 {
            SetCloseLinkCallback();
            HideFieldMessageBox();
            task_set_func(taskId, Some(Task_StopLinkup));
        } else {
            CloseLink();
            HideFieldMessageBox();
            task_set_func(taskId, Some(Task_StopLinkup));
        }
    } else if gSpecialVar_Result == LINKUP_DIFF_SELECTIONS {
        SetCloseLinkCallback();
        HideFieldMessageBox();
        task_set_func(taskId, Some(Task_StopLinkup));
    } else if gSpecialVar_Result == LINKUP_PLAYER_NOT_READY
        || gSpecialVar_Result == LINKUP_PARTNER_NOT_READY
    {
        CloseLink();
        HideFieldMessageBox();
        task_set_func(taskId, Some(Task_StopLinkup));
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
        task_set_func(taskId, Some(Task_LinkupAwaitTrainerCardData));
        SendBlockRequest(BLOCK_REQ_SIZE_100);
    }
}
pub unsafe fn AreBattleTowerLinkSpeciesSame(speciesList1: *mut u16, speciesList2: *mut u16) -> u32 {
    let mut haveSameSpecies: u32 = FALSE as u32;
    let mut numSameSpecies: i32 = 0;
    gStringVar1[0] = EOS;
    gStringVar2[0] = EOS;
    for i in 0..FRONTIER_MULTI_PARTY_SIZE {
        for j in 0..FRONTIER_MULTI_PARTY_SIZE {
            if *speciesList1.at(i) == *speciesList2.at(j) {
                if numSameSpecies == 0 {
                    StringCopy(
                        gStringVar1.as_mut_ptr(),
                        (*(&raw const crate::data::data_tables::gSpeciesNames).cast::<CArray<
                            CArray<u8, 11>,
                            0,
                        >>(
                        ))[*speciesList1.at(i)]
                        .as_ptr()
                        .cast_mut(),
                    );
                    haveSameSpecies = TRUE as u32;
                }
                if numSameSpecies == 1 {
                    StringCopy(
                        gStringVar2.as_mut_ptr(),
                        (*(&raw const crate::data::data_tables::gSpeciesNames).cast::<CArray<
                            CArray<u8, 11>,
                            0,
                        >>(
                        ))[*speciesList1.at(i)]
                        .as_ptr()
                        .cast_mut(),
                    );
                    haveSameSpecies = TRUE as u32;
                }
                numSameSpecies += 1;
            }
        }
    }
    gSpecialVar_0x8005 = numSameSpecies as u16;
    haveSameSpecies
}
unsafe fn FinishLinkup(linkupStatus: *mut u16, taskId: u32) {
    let trainerCards: *mut TrainerCard = gTrainerCards.as_mut_ptr();
    if *linkupStatus == LINKUP_SUCCESS {
        if gLinkType == LINKTYPE_BATTLE_TOWER_50 || gLinkType == LINKTYPE_BATTLE_TOWER_OPEN {
            if AreBattleTowerLinkSpeciesSame(
                (*trainerCards).monSpecies.as_mut_ptr(),
                (*trainerCards.at(1)).monSpecies.as_mut_ptr(),
            ) != 0
            {
                *linkupStatus = LINKUP_FAILED_BATTLE_TOWER;
                SetCloseLinkCallback();
                task_set_func(taskId, Some(Task_StopLinkup));
            } else {
                ClearLinkPlayerCountWindow(task_get(taskId, tWindowId) as u16);
                ScriptContext_Enable();
                DestroyTask(taskId as u8);
            }
        } else {
            ClearLinkPlayerCountWindow(task_get(taskId, tWindowId) as u16);
            ScriptContext_Enable();
            DestroyTask(taskId as u8);
        }
    } else {
        SetCloseLinkCallback();
        task_set_func(taskId, Some(Task_StopLinkup));
    }
}
pub(crate) unsafe fn Task_LinkupAwaitTrainerCardData(taskId: u8) {
    if CheckLinkErrored(taskId) == TRUE as u32 {
        return;
    }
    if GetBlockReceivedStatus() != GetSavedLinkPlayerCountAsBitFlags() {
        return;
    }
    let mut index: u8 = 0;
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
pub(crate) unsafe fn Task_StopLinkup(taskId: u8) {
    if gReceivedRemoteLinkPlayers == 0 {
        ClearLinkPlayerCountWindow(task_get(taskId, tWindowId) as u16);
        ScriptContext_Enable();
        RemoveWindow(task_get(taskId, tWindowId) as u8);
        DestroyTask(taskId);
    }
}
pub(crate) unsafe fn Task_LinkupFailed(taskId: u8) {
    gSpecialVar_Result = LINKUP_FAILED;
    ClearLinkPlayerCountWindow(task_get(taskId, tWindowId) as u16);
    StopFieldMessage();
    RemoveWindow(task_get(taskId, tWindowId) as u8);
    ScriptContext_Enable();
    DestroyTask(taskId);
}
pub(crate) unsafe fn Task_LinkupConnectionError(taskId: u8) {
    gSpecialVar_Result = LINKUP_CONNECTION_ERROR;
    ClearLinkPlayerCountWindow(task_get(taskId, tWindowId) as u16);
    RemoveWindow(task_get(taskId, tWindowId) as u8);
    HideFieldMessageBox();
    ScriptContext_Enable();
    DestroyTask(taskId);
}
fn TryLinkTimeout(taskId: u8) -> u8 {
    task_set(taskId, 4, task_get(taskId, 4) + 1);
    if task_get(taskId, 4) > 600 {
        task_set_func(taskId, Some(Task_LinkupConnectionError));
        return TRUE;
    }
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn TryBattleLinkup() {
    let mut minPlayers: u8 = 2;
    let mut maxPlayers: u8 = 2;
    match *(&raw const crate::ffi::gSpecialVar_0x8004)
        .cast::<u16>()
        .cast_mut()
    {
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
pub unsafe fn TryTradeLinkup() {
    gLinkType = LINKTYPE_TRADE_SETUP;
    gBattleTypeFlags = 0;
    CreateLinkupTask(2, 2);
}
#[unsafe(no_mangle)]
pub unsafe fn TryRecordMixLinkup() {
    gSpecialVar_Result = LINKUP_ONGOING;
    gLinkType = LINKTYPE_RECORD_MIX_BEFORE;
    gBattleTypeFlags = 0;
    CreateLinkupTask(2, 4);
}
#[unsafe(no_mangle)]
pub unsafe fn ValidateMixingGameLanguage() {
    let mut taskId: u32 = FindTaskIdByFunc(Some(Task_ValidateMixingGameLanguage)) as u32;
    if taskId == TASK_NONE as u32 {
        taskId = CreateTask(Some(Task_ValidateMixingGameLanguage), 80) as u32;
        task_set(taskId, tState, 0);
    }
}
pub(crate) unsafe fn Task_ValidateMixingGameLanguage(taskId: u8) {
    let mut playerCount: i32 = 0;
    match task_get(taskId, tState) {
        0 => {
            if gSpecialVar_Result == LINKUP_SUCCESS {
                let mut mixingForeignGames: u32 = FALSE as u32;
                let mut isEnglishRSLinked: u32 = FALSE as u32;
                let mut isJapaneseEmeraldLinked: u32 = FALSE as u32;
                playerCount = GetLinkPlayerCount() as i32;
                for i in 0..playerCount {
                    let version: u32 = gLinkPlayers[i].version as u8 as u32;
                    let language: u32 = gLinkPlayers[i].language as u32;
                    if version == VERSION_RUBY as u32 || version == VERSION_SAPPHIRE as u32 {
                        if language == LANGUAGE_JAPANESE as u32 {
                            mixingForeignGames = TRUE as u32;
                            break;
                        } else {
                            isEnglishRSLinked = TRUE as u32;
                        }
                    } else if version == VERSION_EMERALD as u32
                        && language == LANGUAGE_JAPANESE as u32
                    {
                        isJapaneseEmeraldLinked = TRUE as u32;
                    }
                }
                if isEnglishRSLinked != 0 && isJapaneseEmeraldLinked != 0 {
                    mixingForeignGames = TRUE as u32;
                }
                if mixingForeignGames != 0 {
                    gSpecialVar_Result = LINKUP_FOREIGN_GAME;
                    SetCloseLinkCallbackHandleJP();
                    task_set(taskId, tState, 1);
                    return;
                }
            }
            ScriptContext_Enable();
            DestroyTask(taskId);
        }
        1 if gReceivedRemoteLinkPlayers == 0 => {
            ScriptContext_Enable();
            DestroyTask(taskId);
        }
        _ => {}
    }
}
#[unsafe(no_mangle)]
pub unsafe fn TryBerryBlenderLinkup() {
    gLinkType = LINKTYPE_BERRY_BLENDER_SETUP;
    gBattleTypeFlags = 0;
    CreateLinkupTask(2, 4);
}
#[unsafe(no_mangle)]
pub unsafe fn TryContestGModeLinkup() {
    gLinkType = LINKTYPE_CONTEST_GMODE;
    gBattleTypeFlags = 0;
    CreateLinkupTask(4, 4);
}
#[unsafe(no_mangle)]
pub unsafe fn TryContestEModeLinkup() {
    gLinkType = LINKTYPE_CONTEST_EMODE;
    gBattleTypeFlags = 0;
    CreateLinkupTask(2, 4);
}
pub unsafe fn CreateTask_ReestablishCableClubLink() -> u8 {
    if FuncIsActiveTask(Some(Task_ReestablishLink)) != FALSE {
        return TASK_NONE;
    }
    match *(&raw const crate::ffi::gSpecialVar_0x8004)
        .cast::<u16>()
        .cast_mut()
    {
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
    CreateTask(Some(Task_ReestablishLink), 80)
}
pub(crate) unsafe fn Task_ReestablishLink(taskId: u8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    if *data == 0 {
        OpenLink();
        ResetLinkPlayers();
        CreateTask(Some(Task_WaitForLinkPlayerConnection), 80);
    } else if *data >= 10 {
        task_set_func(taskId, Some(Task_ReestablishLinkAwaitConnection));
    }
    *data += 1;
}
pub(crate) unsafe fn Task_ReestablishLinkAwaitConnection(taskId: u8) {
    if GetLinkPlayerCount_2() >= 2 {
        if IsLinkMaster() == TRUE {
            task_set_func(taskId, Some(Task_ReestablishLinkLeader));
        } else {
            task_set_func(taskId, Some(Task_ReestablishLinkAwaitConfirmation));
        }
    }
}
pub(crate) unsafe fn Task_ReestablishLinkLeader(taskId: u8) {
    if GetSavedPlayerCount() == GetLinkPlayerCount_2() {
        CheckShouldAdvanceLinkState();
        task_set_func(taskId, Some(Task_ReestablishLinkAwaitConfirmation));
    }
}
pub(crate) unsafe fn Task_ReestablishLinkAwaitConfirmation(taskId: u8) {
    if gReceivedRemoteLinkPlayers == TRUE && IsLinkPlayerDataExchangeComplete() == TRUE {
        CheckLinkPlayersMatchSaved();
        StartSendingKeysToLink();
        DestroyTask(taskId);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn CableClubSaveGame() {
    SaveGame();
}
unsafe fn SetLinkBattleTypeFlags(linkService: i32) {
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
pub(crate) unsafe fn Task_StartWiredCableClubBattle(taskId: u8) {
    let task: *mut Task = &raw mut (*gTasks.as_ptr())[taskId];
    match (*task).data[tState] {
        0 => {
            FadeScreen(FADE_TO_BLACK, 0);
            gLinkType = LINKTYPE_BATTLE;
            ClearLinkCallback_2();
            (*task).data[tState] += 1;
        }
        1 => {
            if gPaletteFade.active() == 0 {
                (*task).data[tState] += 1;
            }
        }
        2 => {
            (*task).data[1] += 1;
            if (*task).data[1] > 20 {
                (*task).data[tState] += 1;
            }
        }
        3 => {
            SetCloseLinkCallback();
            (*task).data[tState] += 1;
        }
        4 => {
            if gReceivedRemoteLinkPlayers == 0 {
                (*task).data[tState] += 1;
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
pub(crate) unsafe fn Task_StartWirelessCableClubBattle(taskId: u8) {
    let mut i: i32 = 0;
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
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
                    let player: *mut LinkPlayer =
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
pub(crate) unsafe fn CB2_ReturnFromUnionRoomBattle() {
    let mut playerCount: u8 = 0;
    let mut linkedWithFRLG: u32 = 0;
    match gMain.state {
        0 => {
            playerCount = GetLinkPlayerCount();
            linkedWithFRLG = FALSE as u32;
            for i in 0..(playerCount as i32) {
                let version: u32 = gLinkPlayers[i].version as u8 as u32;
                if version == VERSION_FIRE_RED as u32 || version == VERSION_LEAF_GREEN as u32 {
                    linkedWithFRLG = TRUE as u32;
                    break;
                }
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
pub unsafe fn CB2_ReturnFromCableClubBattle() {
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
pub unsafe fn CleanupLinkRoomState() {
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
pub unsafe fn ExitLinkRoom() {
    QueueExitLinkRoomKey();
}
pub(crate) unsafe fn Task_EnterCableClubSeat(taskId: u8) {
    let task: *mut Task = &raw mut (*gTasks.as_ptr())[taskId];
    match (*task).data[tState] {
        0 => {
            ShowFieldMessage(
                (*crate::asmdata::gText_PleaseWaitForLink.cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut(),
            );
            (*task).data[tState] = 1;
        }
        1 => {
            if IsFieldMessageBoxHidden() != 0 {
                SetInCableClubSeat();
                SetLocalLinkPlayerId(gSpecialVar_0x8005 as u8);
                (*task).data[tState] = 2;
            }
        }
        2 => match GetCableClubPartnersReady() {
            CABLE_SEAT_WAITING => {}
            CABLE_SEAT_SUCCESS => {
                HideFieldMessageBox();
                (*task).data[tState] = 0;
                SetStartedCableClubActivity();
                SwitchTaskToFollowupFunc(taskId);
            }
            CABLE_SEAT_FAILED => {
                (*task).data[tState] = 3;
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
pub unsafe fn CreateTask_EnterCableClubSeat(followupFunc: Option<unsafe fn(u8)>) {
    let taskId: u8 = CreateTask(Some(Task_EnterCableClubSeat), 80);
    SetTaskFuncWithFollowupFunc(taskId, Some(Task_EnterCableClubSeat), followupFunc);
    ScriptContext_Stop();
}
pub(crate) unsafe fn Task_StartWiredTrade(taskId: u8) {
    let task: *mut Task = &raw mut (*gTasks.as_ptr())[taskId];
    match (*task).data[tState] {
        0 => {
            LockPlayerFieldControls();
            FadeScreen(FADE_TO_BLACK, 0);
            ClearLinkCallback_2();
            (*task).data[tState] += 1;
        }
        1 => {
            if gPaletteFade.active() == 0 {
                (*task).data[tState] += 1;
            }
        }
        2 => {
            gSelectedTradeMonPositions[0] = 0;
            gSelectedTradeMonPositions[1] = 0;
            m4aMPlayAllStop();
            SetCloseLinkCallback();
            (*task).data[tState] += 1;
        }
        3 if gReceivedRemoteLinkPlayers == 0 => {
            SetMainCallback2(Some(CB2_StartCreateTradeMenu));
            DestroyTask(taskId);
        }
        _ => {}
    }
}
pub(crate) unsafe fn Task_StartWirelessTrade(taskId: u8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
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
        3 if IsLinkTaskFinished() != 0 => {
            CreateTask_CreateTradeMenu();
            DestroyTask(taskId);
        }
        _ => {}
    }
}
#[unsafe(no_mangle)]
pub unsafe fn PlayerEnteredTradeSeat() {
    if gWirelessCommType != 0 {
        CreateTask_EnterCableClubSeat(Some(Task_StartWirelessTrade));
    } else {
        CreateTask_EnterCableClubSeat(Some(Task_StartWiredTrade));
    }
}
unsafe fn CreateTask_StartWiredTrade() {
    CreateTask(Some(Task_StartWiredTrade), 80);
}
#[unsafe(no_mangle)]
pub fn Script_StartWiredTrade() {}
#[unsafe(no_mangle)]
pub unsafe fn ColosseumPlayerSpotTriggered() {
    gLinkType = LINKTYPE_BATTLE;
    if gWirelessCommType != 0 {
        CreateTask_EnterCableClubSeat(Some(Task_StartWirelessCableClubBattle));
    } else {
        CreateTask_EnterCableClubSeat(Some(Task_StartWiredCableClubBattle));
    }
}
unsafe fn CreateTask_EnterCableClubSeatNoFollowup() {
    let taskId: u8 = CreateTask(Some(Task_EnterCableClubSeat), 80);
    ScriptContext_Stop();
}
#[unsafe(no_mangle)]
pub unsafe fn Script_ShowLinkTrainerCard() {
    ShowTrainerCardInLink(
        gSpecialVar_0x8006 as u8,
        Some(CB2_ReturnToFieldContinueScriptPlayMapMusic),
    );
}
pub unsafe fn GetLinkTrainerCardColor(linkPlayerIndex: u8) -> u32 {
    gSpecialVar_0x8006 = linkPlayerIndex as u16;
    StringCopy(
        gStringVar1.as_mut_ptr(),
        gLinkPlayers[linkPlayerIndex].name.as_mut_ptr(),
    );
    let numStars: u32 = GetTrainerCardStars(linkPlayerIndex) as u32;
    if numStars == 0 {
        return FALSE as u32;
    }
    StringCopy(
        gStringVar2.as_mut_ptr(),
        sTrainerCardColorNames[numStars - 1],
    );
    TRUE as u32
}
pub unsafe fn Task_WaitForLinkPlayerConnection(taskId: u8) {
    let task: *mut Task = &raw mut (*gTasks.as_ptr())[taskId];
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
pub(crate) unsafe fn Task_WaitExitToScript(taskId: u8) {
    if gReceivedRemoteLinkPlayers == 0 {
        ScriptContext_Enable();
        DestroyTask(taskId);
    }
}
unsafe fn ExitLinkToScript(taskId: u8) {
    SetCloseLinkCallback();
    task_set_func(taskId, Some(Task_WaitExitToScript));
}
pub unsafe fn Task_ReconnectWithLinkPlayers(taskId: u8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
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
        3 if gReceivedRemoteLinkPlayers == TRUE && IsLinkPlayerDataExchangeComplete() == TRUE => {
            DestroyTask(taskId);
        }
        _ => {}
    }
}
#[unsafe(no_mangle)]
pub unsafe fn TrySetBattleTowerLinkType() {
    if gWirelessCommType == 0 {
        gLinkType = LINKTYPE_BATTLE_TOWER;
    }
}
