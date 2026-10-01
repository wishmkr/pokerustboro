//! Translated from `src/battle_controllers.c` by tools/rustport/c2rs.py.
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
    clippy::missing_transmute_annotations,
    clippy::too_many_arguments,
    dead_code,
    unused_assignments
)]

use crate::battle_ai_script_commands::BattleAI_HandleItemUseBeforeAISetup;
use crate::battle_anim::ClearBattleAnimationVars;
use crate::battle_controller_link_opponent::SetControllerToLinkOpponent;
use crate::battle_controller_link_partner::SetControllerToLinkPartner;
use crate::battle_controller_opponent::SetControllerToOpponent;
use crate::battle_controller_player::{BattleControllerDummy, SetControllerToPlayer};
use crate::battle_controller_player_partner::SetControllerToPlayerPartner;
use crate::battle_controller_recorded_opponent::SetControllerToRecordedOpponent;
use crate::battle_controller_recorded_player::SetControllerToRecordedPlayer;
use crate::battle_controller_safari::SetControllerToSafari;
use crate::battle_controller_wally::SetControllerToWally;
use crate::battle_main::{
    BeginBattleIntro, BeginBattleIntroDummy, gAbsentBattlerFlags, gActiveBattler,
    gBattleControllerExecFlags, gBattleMainFunc, gBattleMons, gBattleOutcome, gBattleScripting,
    gBattleStruct, gBattleTypeFlags, gBattleWeather, gBattlerAttacker, gBattlerControllerFuncs,
    gBattlerTarget, gBattlersCount, gChosenMove, gCurrentMove, gEffectBattler, gLastUsedAbility,
    gLastUsedItem, gLinkBattleRecvBuffer, gLinkBattleSendBuffer, gPotentialItemEffectBattler,
    gUnusedFirstBattleVar1, gUnusedFirstBattleVar2,
};
use crate::battle_main::{
    gActionSelectionCursor, gBattleBufferA, gBattleBufferB, gBattleTextBuff1, gBattleTextBuff2,
    gBattleTextBuff3, gBattlerPartyIndexes, gBattlerPositions, gMoveSelectionCursor,
};
use crate::battle_util::{AbilityBattleEffects, MarkBattlerReceivedLinkData};
#[allow(unused_imports)]
use crate::c::*;
use crate::cable_club::Task_WaitForLinkPlayerConnection;
#[allow(unused_imports)]
use crate::consts::*;
use crate::link::gBlockRecvBuffer;
use crate::link::{
    BitmaskAllOtherLinkPlayers, CheckShouldAdvanceLinkState, GetBlockReceivedStatus,
    GetLinkPlayerCount, GetLinkPlayerCount_2, GetMultiplayerId, IsLinkMaster, IsLinkTaskFinished,
    OpenLink, ResetBlockReceivedFlag, SendBlock, SetWirelessCommType1, gLinkPlayers,
    gReceivedRemoteLinkPlayers, gWirelessCommType,
};
use crate::link_rfu_2::DestroyTask_RfuIdle;
use crate::load_save::gSaveBlock2Ptr;
use crate::party_menu::BufferBattlePartyCurrentOrderBySide;
use crate::pokemon::{
    ClearBattleMonForms, CreateMon, GetMonData2, SetMonData, ZeroEnemyPartyMons, gEnemyParty,
    gPlayerParty,
};
use crate::recorded_battle::{
    RecordedBattle_BufferNewBattlerData, RecordedBattle_Init, RecordedBattle_SaveParties,
    gRecordedBattleMultiplayerId,
};
use crate::task::{task_get, task_set};
#[allow(unused_imports)]
use crate::types::*;
use crate::util::gBitTable;
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
// The C's names for task and sprite data slots.
const tInitialDelayTimer: usize = 10;
const tState: usize = 11;
const tCurrentBlock_WrapFrom: usize = 12;
const tBlockSendDelayTimer: usize = 13;
const tCurrentBlock_End: usize = 14;
const tCurrentBlock_Start: usize = 15;

const LINK_BUFF_ABSENT_BATTLER_FLAGS: i32 = 6;
const LINK_BUFF_ACTIVE_BATTLER: i32 = 1;
const LINK_BUFF_ATTACKER: i32 = 2;
const LINK_BUFF_BUFFER_ID: i32 = 0;
const LINK_BUFF_DATA: i32 = 8;
const LINK_BUFF_EFFECT_BATTLER: i32 = 7;
const LINK_BUFF_SIZE_HI: i32 = 5;
const LINK_BUFF_SIZE_LO: i32 = 4;
const LINK_BUFF_TARGET: i32 = 3;
const SENDTASK_STATE_BEGIN_SEND_BLOCK: i16 = 3;
const SENDTASK_STATE_COUNT_PLAYERS: i16 = 2;
const SENDTASK_STATE_FINISH_SEND_BLOCK: i16 = 4;
const SENDTASK_STATE_INITIALIZE: i16 = 0;
const SENDTASK_STATE_INITIAL_DELAY: i16 = 1;
const SENDTASK_STATE_UNUSED_STATE: i16 = 5;

#[unsafe(link_section = "ewram_data")]
pub(crate) static sLinkSendTaskId: crate::global::Global<u8> = crate::global::Global::new(0);
#[unsafe(link_section = "ewram_data")]
pub(crate) static sLinkReceiveTaskId: crate::global::Global<u8> = crate::global::Global::new(0);
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sUnused: u8 = 0;
#[unsafe(link_section = "ewram_data")]
pub static mut gUnusedControllerStruct: UnusedControllerStruct = unsafe { zeroed() };
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sBattleBuffersTransferData: Aligned<CArray<u8, 256>> =
    Aligned(unsafe { zeroed() });

pub unsafe fn HandleLinkBattleSetup() {
    if gBattleTypeFlags & BATTLE_TYPE_LINK != 0 {
        if gWirelessCommType != 0 {
            SetWirelessCommType1();
        }
        if gReceivedRemoteLinkPlayers == 0 {
            OpenLink();
        }
        CreateTask(Some(Task_WaitForLinkPlayerConnection), 0);
        CreateTasksForSendRecvLinkBuffers();
    }
}
pub unsafe fn SetUpBattleVarsAndBirchZigzagoon() {
    gBattleMainFunc = Some(BeginBattleIntroDummy);
    let mut i: i32 = 0;
    while i < MAX_BATTLERS_COUNT as i32 {
        gBattlerControllerFuncs[i] = Some(BattleControllerDummy);
        gBattlerPositions[i] = 0xFF;
        gActionSelectionCursor[i] = 0;
        gMoveSelectionCursor[i] = 0;
        i += 1;
    }
    HandleLinkBattleSetup();
    gBattleControllerExecFlags = 0;
    ClearBattleAnimationVars();
    ClearBattleMonForms();
    gActiveBattler = 0;
    BattleAI_HandleItemUseBeforeAISetup(0xF);
    if gBattleTypeFlags & BATTLE_TYPE_FIRST_BATTLE != 0 {
        ZeroEnemyPartyMons();
        CreateMon(
            &raw mut gEnemyParty[0],
            SPECIES_ZIGZAGOON,
            2,
            USE_RANDOM_IVS,
            0,
            0,
            0,
            0,
        );
        i = 0;
        SetMonData(
            &raw mut gEnemyParty[0],
            MON_DATA_HELD_ITEM,
            &raw mut i as *mut c_void,
        );
    }
    gUnusedFirstBattleVar1 = 0;
    gUnusedFirstBattleVar2 = 0;
}
pub unsafe fn InitBattleControllers() {
    let mut i: i32 = 0;
    if gBattleTypeFlags & BATTLE_TYPE_RECORDED == 0 {
        RecordedBattle_Init(B_RECORD_MODE_RECORDING);
    } else {
        RecordedBattle_Init(B_RECORD_MODE_PLAYBACK);
    }
    if gBattleTypeFlags & BATTLE_TYPE_RECORDED == 0 {
        RecordedBattle_SaveParties();
    }
    if gBattleTypeFlags & BATTLE_TYPE_LINK != 0 {
        InitLinkBtlControllers();
    } else {
        InitSinglePlayerBtlControllers();
    }
    SetBattlePartyIds();
    if gBattleTypeFlags & BATTLE_TYPE_MULTI == 0 {
        i = 0;
        while i < gBattlersCount as i32 {
            BufferBattlePartyCurrentOrderBySide(i as u8, 0);
            i += 1;
        }
    }
    for i in 0..96i32 {
        *(&raw mut (*gBattleStruct).tvMovePoints as *mut u8).at(i) = 0;
    }
    for i in 0..104i32 {
        *(&raw mut (*gBattleStruct).tv as *mut u8).at(i) = 0;
    }
}
unsafe fn InitSinglePlayerBtlControllers() {
    if gBattleTypeFlags & BATTLE_TYPE_INGAME_PARTNER != 0 {
        gBattleMainFunc = Some(BeginBattleIntro);
        if gBattleTypeFlags & BATTLE_TYPE_RECORDED != 0 {
            gBattlerControllerFuncs[0] = Some(SetControllerToRecordedPlayer);
            gBattlerPositions[0] = B_POSITION_PLAYER_LEFT;
            gBattlerControllerFuncs[1] = Some(SetControllerToOpponent);
            gBattlerPositions[1] = B_POSITION_OPPONENT_LEFT;
            gBattlerControllerFuncs[2] = Some(SetControllerToPlayerPartner);
            gBattlerPositions[2] = B_POSITION_PLAYER_RIGHT;
            gBattlerControllerFuncs[3] = Some(SetControllerToOpponent);
            gBattlerPositions[3] = B_POSITION_OPPONENT_RIGHT;
        } else {
            gBattlerControllerFuncs[0] = Some(SetControllerToPlayer);
            gBattlerPositions[0] = B_POSITION_PLAYER_LEFT;
            gBattlerControllerFuncs[1] = Some(SetControllerToOpponent);
            gBattlerPositions[1] = B_POSITION_OPPONENT_LEFT;
            gBattlerControllerFuncs[2] = Some(SetControllerToPlayerPartner);
            gBattlerPositions[2] = B_POSITION_PLAYER_RIGHT;
            gBattlerControllerFuncs[3] = Some(SetControllerToOpponent);
            gBattlerPositions[3] = B_POSITION_OPPONENT_RIGHT;
        }
        gBattlersCount = MAX_BATTLERS_COUNT;
        BufferBattlePartyCurrentOrderBySide(0, 0);
        BufferBattlePartyCurrentOrderBySide(1, 0);
        BufferBattlePartyCurrentOrderBySide(2, 1);
        BufferBattlePartyCurrentOrderBySide(3, 1);
        gBattlerPartyIndexes[0] = 0;
        gBattlerPartyIndexes[1] = 0;
        gBattlerPartyIndexes[2] = 3;
        gBattlerPartyIndexes[3] = 3;
    } else if gBattleTypeFlags & BATTLE_TYPE_DOUBLE == 0 {
        gBattleMainFunc = Some(BeginBattleIntro);
        if gBattleTypeFlags & BATTLE_TYPE_SAFARI != 0 {
            gBattlerControllerFuncs[0] = Some(SetControllerToSafari);
        } else if gBattleTypeFlags & BATTLE_TYPE_WALLY_TUTORIAL != 0 {
            gBattlerControllerFuncs[0] = Some(SetControllerToWally);
        } else {
            gBattlerControllerFuncs[0] = Some(SetControllerToPlayer);
        }
        gBattlerPositions[0] = B_POSITION_PLAYER_LEFT;
        gBattlerControllerFuncs[1] = Some(SetControllerToOpponent);
        gBattlerPositions[1] = B_POSITION_OPPONENT_LEFT;
        gBattlersCount = 2;
        if gBattleTypeFlags & BATTLE_TYPE_RECORDED != 0 {
            if gBattleTypeFlags & BATTLE_TYPE_RECORDED_LINK != 0 {
                if gBattleTypeFlags & 0x80000000 != 0 {
                    gBattleMainFunc = Some(BeginBattleIntro);
                    gBattlerControllerFuncs[0] = Some(SetControllerToRecordedPlayer);
                    gBattlerPositions[0] = B_POSITION_PLAYER_LEFT;
                    gBattlerControllerFuncs[1] = Some(SetControllerToRecordedOpponent);
                    gBattlerPositions[1] = B_POSITION_OPPONENT_LEFT;
                    gBattlersCount = 2;
                } else {
                    gBattlerControllerFuncs[1] = Some(SetControllerToRecordedPlayer);
                    gBattlerPositions[1] = B_POSITION_PLAYER_LEFT;
                    gBattlerControllerFuncs[0] = Some(SetControllerToRecordedOpponent);
                    gBattlerPositions[0] = B_POSITION_OPPONENT_LEFT;
                    gBattlersCount = 2;
                }
            } else {
                gBattlerControllerFuncs[0] = Some(SetControllerToRecordedPlayer);
                gBattlerPositions[0] = B_POSITION_PLAYER_LEFT;
                gBattlerControllerFuncs[1] = Some(SetControllerToOpponent);
                gBattlerPositions[1] = B_POSITION_OPPONENT_LEFT;
            }
        }
    } else {
        gBattleMainFunc = Some(BeginBattleIntro);
        gBattlerControllerFuncs[0] = Some(SetControllerToPlayer);
        gBattlerPositions[0] = B_POSITION_PLAYER_LEFT;
        gBattlerControllerFuncs[1] = Some(SetControllerToOpponent);
        gBattlerPositions[1] = B_POSITION_OPPONENT_LEFT;
        gBattlerControllerFuncs[2] = Some(SetControllerToPlayer);
        gBattlerPositions[2] = B_POSITION_PLAYER_RIGHT;
        gBattlerControllerFuncs[3] = Some(SetControllerToOpponent);
        gBattlerPositions[3] = B_POSITION_OPPONENT_RIGHT;
        gBattlersCount = MAX_BATTLERS_COUNT;
        if gBattleTypeFlags & BATTLE_TYPE_RECORDED != 0 {
            if gBattleTypeFlags & BATTLE_TYPE_MULTI != 0
                && gBattleTypeFlags & BATTLE_TYPE_BATTLE_TOWER != 0
            {
                gBattleMainFunc = Some(BeginBattleIntro);
                gBattlerControllerFuncs[0] = Some(SetControllerToRecordedPlayer);
                gBattlerPositions[0] = B_POSITION_PLAYER_LEFT;
                gBattlerControllerFuncs[1] = Some(SetControllerToOpponent);
                gBattlerPositions[1] = B_POSITION_OPPONENT_LEFT;
                gBattlerControllerFuncs[2] = Some(SetControllerToRecordedPlayer);
                gBattlerPositions[2] = B_POSITION_PLAYER_RIGHT;
                gBattlerControllerFuncs[3] = Some(SetControllerToOpponent);
                gBattlerPositions[3] = B_POSITION_OPPONENT_RIGHT;
                gBattlersCount = MAX_BATTLERS_COUNT;
                BufferBattlePartyCurrentOrderBySide(0, 0);
                BufferBattlePartyCurrentOrderBySide(1, 0);
                BufferBattlePartyCurrentOrderBySide(2, 1);
                BufferBattlePartyCurrentOrderBySide(3, 1);
                gBattlerPartyIndexes[0] = 0;
                gBattlerPartyIndexes[1] = 0;
                gBattlerPartyIndexes[2] = 3;
                gBattlerPartyIndexes[3] = 3;
            } else if gBattleTypeFlags & BATTLE_TYPE_MULTI != 0 {
                let multiplayerId: u8 = gRecordedBattleMultiplayerId;
                for i in 0..MAX_LINK_PLAYERS {
                    match gLinkPlayers[i].id {
                        0 | 3 => {
                            BufferBattlePartyCurrentOrderBySide(gLinkPlayers[i].id as u8, 0);
                        }
                        1 | 2 => {
                            BufferBattlePartyCurrentOrderBySide(gLinkPlayers[i].id as u8, 1);
                        }
                        _ => {}
                    }
                    if i == multiplayerId as i32 {
                        gBattlerControllerFuncs[gLinkPlayers[i].id] =
                            Some(SetControllerToRecordedPlayer);
                        match gLinkPlayers[i].id {
                            0 | 3 => {
                                gBattlerPositions[gLinkPlayers[i].id] = B_POSITION_PLAYER_LEFT;
                                gBattlerPartyIndexes[gLinkPlayers[i].id] = 0;
                            }
                            1 | 2 => {
                                gBattlerPositions[gLinkPlayers[i].id] = B_POSITION_PLAYER_RIGHT;
                                gBattlerPartyIndexes[gLinkPlayers[i].id] = 3;
                            }
                            _ => {}
                        }
                    } else if gLinkPlayers[i].id as i32 & 1 == 0
                        && gLinkPlayers[multiplayerId].id as i32 & 1 == 0
                        || gLinkPlayers[i].id as i32 & 1 != 0
                            && gLinkPlayers[multiplayerId].id as i32 & 1 != 0
                    {
                        gBattlerControllerFuncs[gLinkPlayers[i].id] =
                            Some(SetControllerToRecordedPlayer);
                        match gLinkPlayers[i].id {
                            0 | 3 => {
                                gBattlerPositions[gLinkPlayers[i].id] = B_POSITION_PLAYER_LEFT;
                                gBattlerPartyIndexes[gLinkPlayers[i].id] = 0;
                            }
                            1 | 2 => {
                                gBattlerPositions[gLinkPlayers[i].id] = B_POSITION_PLAYER_RIGHT;
                                gBattlerPartyIndexes[gLinkPlayers[i].id] = 3;
                            }
                            _ => {}
                        }
                    } else {
                        gBattlerControllerFuncs[gLinkPlayers[i].id] =
                            Some(SetControllerToRecordedOpponent);
                        match gLinkPlayers[i].id {
                            0 | 3 => {
                                gBattlerPositions[gLinkPlayers[i].id] = B_POSITION_OPPONENT_LEFT;
                                gBattlerPartyIndexes[gLinkPlayers[i].id] = 0;
                            }
                            1 | 2 => {
                                gBattlerPositions[gLinkPlayers[i].id] = B_POSITION_OPPONENT_RIGHT;
                                gBattlerPartyIndexes[gLinkPlayers[i].id] = 3;
                            }
                            _ => {}
                        }
                    }
                }
            } else if gBattleTypeFlags & BATTLE_TYPE_IS_MASTER != 0 {
                gBattlerControllerFuncs[0] = Some(SetControllerToRecordedPlayer);
                gBattlerPositions[0] = B_POSITION_PLAYER_LEFT;
                gBattlerControllerFuncs[2] = Some(SetControllerToRecordedPlayer);
                gBattlerPositions[2] = B_POSITION_PLAYER_RIGHT;
                if gBattleTypeFlags & BATTLE_TYPE_RECORDED_LINK != 0 {
                    gBattlerControllerFuncs[1] = Some(SetControllerToRecordedOpponent);
                    gBattlerPositions[1] = B_POSITION_OPPONENT_LEFT;
                    gBattlerControllerFuncs[3] = Some(SetControllerToRecordedOpponent);
                    gBattlerPositions[3] = B_POSITION_OPPONENT_RIGHT;
                } else {
                    gBattlerControllerFuncs[1] = Some(SetControllerToOpponent);
                    gBattlerPositions[1] = B_POSITION_OPPONENT_LEFT;
                    gBattlerControllerFuncs[3] = Some(SetControllerToOpponent);
                    gBattlerPositions[3] = B_POSITION_OPPONENT_RIGHT;
                }
            } else {
                gBattlerControllerFuncs[1] = Some(SetControllerToRecordedPlayer);
                gBattlerPositions[1] = B_POSITION_PLAYER_LEFT;
                gBattlerControllerFuncs[3] = Some(SetControllerToRecordedPlayer);
                gBattlerPositions[3] = B_POSITION_PLAYER_RIGHT;
                if gBattleTypeFlags & BATTLE_TYPE_RECORDED_LINK != 0 {
                    gBattlerControllerFuncs[0] = Some(SetControllerToRecordedOpponent);
                    gBattlerPositions[0] = B_POSITION_OPPONENT_LEFT;
                    gBattlerControllerFuncs[2] = Some(SetControllerToRecordedOpponent);
                    gBattlerPositions[2] = B_POSITION_OPPONENT_RIGHT;
                } else {
                    gBattlerControllerFuncs[0] = Some(SetControllerToOpponent);
                    gBattlerPositions[0] = B_POSITION_OPPONENT_LEFT;
                    gBattlerControllerFuncs[2] = Some(SetControllerToOpponent);
                    gBattlerPositions[2] = B_POSITION_OPPONENT_RIGHT;
                }
            }
        }
    }
}
unsafe fn InitLinkBtlControllers() {
    let mut multiplayerId: u8 = 0;
    if gBattleTypeFlags & BATTLE_TYPE_DOUBLE == 0 {
        if gBattleTypeFlags & BATTLE_TYPE_IS_MASTER != 0 {
            gBattleMainFunc = Some(BeginBattleIntro);
            gBattlerControllerFuncs[0] = Some(SetControllerToPlayer);
            gBattlerPositions[0] = B_POSITION_PLAYER_LEFT;
            gBattlerControllerFuncs[1] = Some(SetControllerToLinkOpponent);
            gBattlerPositions[1] = B_POSITION_OPPONENT_LEFT;
            gBattlersCount = 2;
        } else {
            gBattlerControllerFuncs[1] = Some(SetControllerToPlayer);
            gBattlerPositions[1] = B_POSITION_PLAYER_LEFT;
            gBattlerControllerFuncs[0] = Some(SetControllerToLinkOpponent);
            gBattlerPositions[0] = B_POSITION_OPPONENT_LEFT;
            gBattlersCount = 2;
        }
    } else if gBattleTypeFlags & BATTLE_TYPE_MULTI == 0
        && gBattleTypeFlags & BATTLE_TYPE_DOUBLE != 0
    {
        if gBattleTypeFlags & BATTLE_TYPE_IS_MASTER != 0 {
            gBattleMainFunc = Some(BeginBattleIntro);
            gBattlerControllerFuncs[0] = Some(SetControllerToPlayer);
            gBattlerPositions[0] = B_POSITION_PLAYER_LEFT;
            gBattlerControllerFuncs[1] = Some(SetControllerToLinkOpponent);
            gBattlerPositions[1] = B_POSITION_OPPONENT_LEFT;
            gBattlerControllerFuncs[2] = Some(SetControllerToPlayer);
            gBattlerPositions[2] = B_POSITION_PLAYER_RIGHT;
            gBattlerControllerFuncs[3] = Some(SetControllerToLinkOpponent);
            gBattlerPositions[3] = B_POSITION_OPPONENT_RIGHT;
            gBattlersCount = MAX_BATTLERS_COUNT;
        } else {
            gBattlerControllerFuncs[1] = Some(SetControllerToPlayer);
            gBattlerPositions[1] = B_POSITION_PLAYER_LEFT;
            gBattlerControllerFuncs[0] = Some(SetControllerToLinkOpponent);
            gBattlerPositions[0] = B_POSITION_OPPONENT_LEFT;
            gBattlerControllerFuncs[3] = Some(SetControllerToPlayer);
            gBattlerPositions[3] = B_POSITION_PLAYER_RIGHT;
            gBattlerControllerFuncs[2] = Some(SetControllerToLinkOpponent);
            gBattlerPositions[2] = B_POSITION_OPPONENT_RIGHT;
            gBattlersCount = MAX_BATTLERS_COUNT;
        }
    } else if gBattleTypeFlags & BATTLE_TYPE_BATTLE_TOWER != 0 {
        if gBattleTypeFlags & BATTLE_TYPE_IS_MASTER != 0 {
            gBattleMainFunc = Some(BeginBattleIntro);
            gBattlerControllerFuncs[0] = Some(SetControllerToPlayer);
            gBattlerPositions[0] = B_POSITION_PLAYER_LEFT;
            gBattlerControllerFuncs[1] = Some(SetControllerToOpponent);
            gBattlerPositions[1] = B_POSITION_OPPONENT_LEFT;
            gBattlerControllerFuncs[2] = Some(SetControllerToLinkPartner);
            gBattlerPositions[2] = B_POSITION_PLAYER_RIGHT;
            gBattlerControllerFuncs[3] = Some(SetControllerToOpponent);
            gBattlerPositions[3] = B_POSITION_OPPONENT_RIGHT;
            gBattlersCount = MAX_BATTLERS_COUNT;
        } else {
            gBattlerControllerFuncs[0] = Some(SetControllerToLinkPartner);
            gBattlerPositions[0] = B_POSITION_PLAYER_LEFT;
            gBattlerControllerFuncs[1] = Some(SetControllerToLinkOpponent);
            gBattlerPositions[1] = B_POSITION_OPPONENT_LEFT;
            gBattlerControllerFuncs[2] = Some(SetControllerToPlayer);
            gBattlerPositions[2] = B_POSITION_PLAYER_RIGHT;
            gBattlerControllerFuncs[3] = Some(SetControllerToLinkOpponent);
            gBattlerPositions[3] = B_POSITION_OPPONENT_RIGHT;
            gBattlersCount = MAX_BATTLERS_COUNT;
        }
        BufferBattlePartyCurrentOrderBySide(0, 0);
        BufferBattlePartyCurrentOrderBySide(1, 0);
        BufferBattlePartyCurrentOrderBySide(2, 1);
        BufferBattlePartyCurrentOrderBySide(3, 1);
        gBattlerPartyIndexes[0] = 0;
        gBattlerPartyIndexes[1] = 0;
        gBattlerPartyIndexes[2] = 3;
        gBattlerPartyIndexes[3] = 3;
    } else {
        multiplayerId = GetMultiplayerId();
        if gBattleTypeFlags & BATTLE_TYPE_IS_MASTER != 0 {
            gBattleMainFunc = Some(BeginBattleIntro);
        }
        for i in 0..MAX_LINK_PLAYERS {
            match gLinkPlayers[i].id {
                0 | 3 => {
                    BufferBattlePartyCurrentOrderBySide(gLinkPlayers[i].id as u8, 0);
                }
                1 | 2 => {
                    BufferBattlePartyCurrentOrderBySide(gLinkPlayers[i].id as u8, 1);
                }
                _ => {}
            }
            if i == multiplayerId as i32 {
                gBattlerControllerFuncs[gLinkPlayers[i].id] = Some(SetControllerToPlayer);
                match gLinkPlayers[i].id {
                    0 | 3 => {
                        gBattlerPositions[gLinkPlayers[i].id] = B_POSITION_PLAYER_LEFT;
                        gBattlerPartyIndexes[gLinkPlayers[i].id] = 0;
                    }
                    1 | 2 => {
                        gBattlerPositions[gLinkPlayers[i].id] = B_POSITION_PLAYER_RIGHT;
                        gBattlerPartyIndexes[gLinkPlayers[i].id] = 3;
                    }
                    _ => {}
                }
            } else {
                if gLinkPlayers[i].id as i32 & 1 == 0
                    && gLinkPlayers[multiplayerId].id as i32 & 1 == 0
                    || gLinkPlayers[i].id as i32 & 1 != 0
                        && gLinkPlayers[multiplayerId].id as i32 & 1 != 0
                {
                    gBattlerControllerFuncs[gLinkPlayers[i].id] = Some(SetControllerToLinkPartner);
                    match gLinkPlayers[i].id {
                        0 | 3 => {
                            gBattlerPositions[gLinkPlayers[i].id] = B_POSITION_PLAYER_LEFT;
                            gBattlerPartyIndexes[gLinkPlayers[i].id] = 0;
                        }
                        1 | 2 => {
                            gBattlerPositions[gLinkPlayers[i].id] = B_POSITION_PLAYER_RIGHT;
                            gBattlerPartyIndexes[gLinkPlayers[i].id] = 3;
                        }
                        _ => {}
                    }
                } else {
                    gBattlerControllerFuncs[gLinkPlayers[i].id] = Some(SetControllerToLinkOpponent);
                    match gLinkPlayers[i].id {
                        0 | 3 => {
                            gBattlerPositions[gLinkPlayers[i].id] = B_POSITION_OPPONENT_LEFT;
                            gBattlerPartyIndexes[gLinkPlayers[i].id] = 0;
                        }
                        1 | 2 => {
                            gBattlerPositions[gLinkPlayers[i].id] = B_POSITION_OPPONENT_RIGHT;
                            gBattlerPartyIndexes[gLinkPlayers[i].id] = 3;
                        }
                        _ => {}
                    }
                }
            }
        }
        gBattlersCount = MAX_BATTLERS_COUNT;
    }
}
unsafe fn SetBattlePartyIds() {
    let mut i: i32 = 0;
    if gBattleTypeFlags & BATTLE_TYPE_MULTI == 0 {
        i = 0;
        while i < gBattlersCount as i32 {
            for j in 0..PARTY_SIZE {
                if i < 2 {
                    if gBattlerPositions[i] as i32 & 1 == B_SIDE_PLAYER as i32 {
                        if GetMonData2(&raw mut gPlayerParty[j], MON_DATA_HP) != 0
                            && GetMonData2(&raw mut gPlayerParty[j], MON_DATA_SPECIES_OR_EGG)
                                != SPECIES_NONE as u32
                            && GetMonData2(&raw mut gPlayerParty[j], MON_DATA_SPECIES_OR_EGG)
                                != SPECIES_EGG
                            && GetMonData2(&raw mut gPlayerParty[j], MON_DATA_IS_EGG) == 0
                        {
                            gBattlerPartyIndexes[i] = j as u16;
                            break;
                        }
                    } else {
                        if GetMonData2(&raw mut gEnemyParty[j], MON_DATA_HP) != 0
                            && GetMonData2(&raw mut gEnemyParty[j], MON_DATA_SPECIES_OR_EGG)
                                != SPECIES_NONE as u32
                            && GetMonData2(&raw mut gEnemyParty[j], MON_DATA_SPECIES_OR_EGG)
                                != SPECIES_EGG
                            && GetMonData2(&raw mut gEnemyParty[j], MON_DATA_IS_EGG) == 0
                        {
                            gBattlerPartyIndexes[i] = j as u16;
                            break;
                        }
                    }
                } else {
                    if gBattlerPositions[i] as i32 & 1 == B_SIDE_PLAYER as i32 {
                        if GetMonData2(&raw mut gPlayerParty[j], MON_DATA_HP) != 0
                            && GetMonData2(&raw mut gPlayerParty[j], MON_DATA_SPECIES)
                                != SPECIES_NONE as u32
                            && GetMonData2(&raw mut gPlayerParty[j], MON_DATA_SPECIES_OR_EGG)
                                != SPECIES_EGG
                            && GetMonData2(&raw mut gPlayerParty[j], MON_DATA_IS_EGG) == 0
                            && gBattlerPartyIndexes[i - 2] as i32 != j
                        {
                            gBattlerPartyIndexes[i] = j as u16;
                            break;
                        }
                    } else {
                        if GetMonData2(&raw mut gEnemyParty[j], MON_DATA_HP) != 0
                            && GetMonData2(&raw mut gEnemyParty[j], MON_DATA_SPECIES_OR_EGG)
                                != SPECIES_NONE as u32
                            && GetMonData2(&raw mut gEnemyParty[j], MON_DATA_SPECIES_OR_EGG)
                                != SPECIES_EGG
                            && GetMonData2(&raw mut gEnemyParty[j], MON_DATA_IS_EGG) == 0
                            && gBattlerPartyIndexes[i - 2] as i32 != j
                        {
                            gBattlerPartyIndexes[i] = j as u16;
                            break;
                        }
                    }
                }
            }
            i += 1;
        }
        if gBattleTypeFlags & BATTLE_TYPE_TWO_OPPONENTS != 0 {
            gBattlerPartyIndexes[1] = 0;
            gBattlerPartyIndexes[3] = 3;
        }
    }
}
unsafe fn PrepareBufferDataTransfer(bufferId: u8, mut data: *mut u8, size: u16) {
    if gBattleTypeFlags & BATTLE_TYPE_LINK != 0 {
        PrepareBufferDataTransferLink(bufferId, size, data);
    } else {
        match bufferId {
            B_COMM_TO_CONTROLLER => {
                for i in 0..(size as i32) {
                    gBattleBufferA[gActiveBattler][i] = *data;
                    data = data.at(1);
                }
            }
            B_COMM_TO_ENGINE => {
                for i in 0..(size as i32) {
                    gBattleBufferB[gActiveBattler][i] = *data;
                    data = data.at(1);
                }
            }
            _ => {}
        }
    }
}
unsafe fn CreateTasksForSendRecvLinkBuffers() {
    sLinkSendTaskId.set(CreateTask(Some(Task_HandleSendLinkBuffersData), 0));
    task_set(sLinkSendTaskId.get(), tState, 0);
    task_set(sLinkSendTaskId.get(), tCurrentBlock_WrapFrom, 0);
    task_set(sLinkSendTaskId.get(), tBlockSendDelayTimer, 0);
    task_set(sLinkSendTaskId.get(), tCurrentBlock_End, 0);
    task_set(sLinkSendTaskId.get(), tCurrentBlock_Start, 0);
    sLinkReceiveTaskId.set(CreateTask(Some(Task_HandleCopyReceivedLinkBuffersData), 0));
    task_set(sLinkReceiveTaskId.get(), tCurrentBlock_WrapFrom, 0);
    task_set(sLinkReceiveTaskId.get(), tBlockSendDelayTimer, 0);
    task_set(sLinkReceiveTaskId.get(), tCurrentBlock_End, 0);
    task_set(sLinkReceiveTaskId.get(), tCurrentBlock_Start, 0);
    sUnused = 0;
}
pub unsafe fn PrepareBufferDataTransferLink(bufferId: u8, size: u16, data: *mut u8) {
    let alignedSize: i32 = size as i32 - size as i32 % 4 + 4;
    if task_get(sLinkSendTaskId.get(), tCurrentBlock_End) as i32 + alignedSize + LINK_BUFF_DATA + 1
        > BATTLE_BUFFER_LINK_SIZE
    {
        task_set(
            sLinkSendTaskId.get(),
            tCurrentBlock_WrapFrom,
            task_get(sLinkSendTaskId.get(), tCurrentBlock_End),
        );
        task_set(sLinkSendTaskId.get(), tCurrentBlock_End, 0);
    }
    *gLinkBattleSendBuffer
        .at(task_get(sLinkSendTaskId.get(), tCurrentBlock_End) as i32 + LINK_BUFF_BUFFER_ID) =
        bufferId;
    *gLinkBattleSendBuffer
        .at(task_get(sLinkSendTaskId.get(), tCurrentBlock_End) as i32 + LINK_BUFF_ACTIVE_BATTLER) =
        gActiveBattler;
    *gLinkBattleSendBuffer
        .at(task_get(sLinkSendTaskId.get(), tCurrentBlock_End) as i32 + LINK_BUFF_ATTACKER) =
        gBattlerAttacker;
    *gLinkBattleSendBuffer
        .at(task_get(sLinkSendTaskId.get(), tCurrentBlock_End) as i32 + LINK_BUFF_TARGET) =
        gBattlerTarget;
    *gLinkBattleSendBuffer
        .at(task_get(sLinkSendTaskId.get(), tCurrentBlock_End) as i32 + LINK_BUFF_SIZE_LO) =
        alignedSize as u8;
    *gLinkBattleSendBuffer
        .at(task_get(sLinkSendTaskId.get(), tCurrentBlock_End) as i32 + LINK_BUFF_SIZE_HI) =
        ((alignedSize & 0x0000FF00) >> 8) as u8;
    *gLinkBattleSendBuffer
        .at(task_get(sLinkSendTaskId.get(), tCurrentBlock_End) as i32
            + LINK_BUFF_ABSENT_BATTLER_FLAGS) = gAbsentBattlerFlags;
    *gLinkBattleSendBuffer
        .at(task_get(sLinkSendTaskId.get(), tCurrentBlock_End) as i32 + LINK_BUFF_EFFECT_BATTLER) =
        gEffectBattler;
    for i in 0..(size as i32) {
        *gLinkBattleSendBuffer
            .at(task_get(sLinkSendTaskId.get(), tCurrentBlock_End) as i32 + LINK_BUFF_DATA + i) =
            *data.at(i);
    }
    task_set(
        sLinkSendTaskId.get(),
        tCurrentBlock_End,
        task_get(sLinkSendTaskId.get(), tCurrentBlock_End)
            + alignedSize as i16
            + LINK_BUFF_DATA as i16,
    );
}
pub(crate) unsafe fn Task_HandleSendLinkBuffersData(taskId: u8) {
    let mut numPlayers: u16 = 0;
    let mut blockSize: u16 = 0;
    'l1: {
        match task_get(taskId, tState) {
            SENDTASK_STATE_INITIALIZE => {
                task_set(taskId, tInitialDelayTimer, 100);
                task_set(taskId, tState, task_get(taskId, tState) + 1);
            }
            SENDTASK_STATE_INITIAL_DELAY => {
                task_set(
                    taskId,
                    tInitialDelayTimer,
                    task_get(taskId, tInitialDelayTimer) - 1,
                );
                if task_get(taskId, tInitialDelayTimer) == 0 {
                    task_set(taskId, tState, task_get(taskId, tState) + 1);
                }
            }
            SENDTASK_STATE_COUNT_PLAYERS => {
                if gWirelessCommType != 0 {
                    task_set(taskId, tState, task_get(taskId, tState) + 1);
                } else {
                    if gBattleTypeFlags & BATTLE_TYPE_BATTLE_TOWER != 0 {
                        numPlayers = 2;
                    } else {
                        numPlayers = (if gBattleTypeFlags & BATTLE_TYPE_MULTI != 0 {
                            4
                        } else {
                            2
                        }) as u16;
                    }
                    if GetLinkPlayerCount_2() as u16 >= numPlayers {
                        if IsLinkMaster() != 0 {
                            CheckShouldAdvanceLinkState();
                            task_set(taskId, tState, task_get(taskId, tState) + 1);
                        } else {
                            task_set(taskId, tState, task_get(taskId, tState) + 1);
                        }
                    }
                }
            }
            SENDTASK_STATE_BEGIN_SEND_BLOCK => {
                if task_get(taskId, tCurrentBlock_Start) != task_get(taskId, tCurrentBlock_End) {
                    if task_get(taskId, tBlockSendDelayTimer) == 0 {
                        if task_get(taskId, tCurrentBlock_Start)
                            > task_get(taskId, tCurrentBlock_End)
                            && task_get(taskId, tCurrentBlock_Start)
                                == task_get(taskId, tCurrentBlock_WrapFrom)
                        {
                            task_set(taskId, tCurrentBlock_WrapFrom, 0);
                            task_set(taskId, tCurrentBlock_Start, 0);
                        }
                        blockSize = (*gLinkBattleSendBuffer
                            .at(task_get(taskId, tCurrentBlock_Start) as i32 + LINK_BUFF_SIZE_LO)
                            as u16
                            | (*gLinkBattleSendBuffer
                                .at(task_get(taskId, tCurrentBlock_Start) as i32
                                    + LINK_BUFF_SIZE_HI) as u16)
                                << 8)
                            + LINK_BUFF_DATA as u16;
                        SendBlock(
                            BitmaskAllOtherLinkPlayers(),
                            gLinkBattleSendBuffer.at(task_get(taskId, tCurrentBlock_Start) as i32)
                                as *mut c_void,
                            blockSize,
                        );
                        task_set(taskId, tState, task_get(taskId, tState) + 1);
                    } else {
                        task_set(
                            taskId,
                            tBlockSendDelayTimer,
                            task_get(taskId, tBlockSendDelayTimer) - 1,
                        );
                        break 'l1;
                    }
                }
            }
            SENDTASK_STATE_FINISH_SEND_BLOCK => {
                if IsLinkTaskFinished() != 0 {
                    blockSize = *gLinkBattleSendBuffer
                        .at(task_get(taskId, tCurrentBlock_Start) as i32 + LINK_BUFF_SIZE_LO)
                        as u16
                        | (*gLinkBattleSendBuffer
                            .at(task_get(taskId, tCurrentBlock_Start) as i32 + LINK_BUFF_SIZE_HI)
                            as u16)
                            << 8;
                    task_set(taskId, tBlockSendDelayTimer, 1);
                    task_set(
                        taskId,
                        tCurrentBlock_Start,
                        task_get(taskId, tCurrentBlock_Start)
                            + blockSize as i16
                            + LINK_BUFF_DATA as i16,
                    );
                    task_set(taskId, tState, SENDTASK_STATE_BEGIN_SEND_BLOCK);
                }
            }
            SENDTASK_STATE_UNUSED_STATE
                if ({
                    task_set(
                        taskId,
                        tBlockSendDelayTimer,
                        task_get(taskId, tBlockSendDelayTimer) - 1,
                    );
                    task_get(taskId, tBlockSendDelayTimer)
                }) == 0 =>
            {
                task_set(taskId, tBlockSendDelayTimer, 1);
                task_set(taskId, tState, SENDTASK_STATE_BEGIN_SEND_BLOCK);
            }
            _ => {}
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe fn TryReceiveLinkBattleData() {
    let mut i: u8 = 0;
    let mut recvBuffer: *mut u8 = null_mut();
    if gReceivedRemoteLinkPlayers != 0 && gBattleTypeFlags & BATTLE_TYPE_LINK_IN_BATTLE != 0 {
        DestroyTask_RfuIdle();
        i = 0;
        while i < GetLinkPlayerCount() {
            if GetBlockReceivedStatus() as u32
                & (*(&raw const crate::util::gBitTable).cast::<CArray<u32, 0>>())[i]
                != 0
            {
                ResetBlockReceivedFlag(i);
                recvBuffer = gBlockRecvBuffer[i].as_mut_ptr() as *mut u8;
                {
                    let dataSize: u16 = gBlockRecvBuffer[i][2];
                    if task_get(sLinkReceiveTaskId.get(), tCurrentBlock_End) as i32
                        + 9
                        + dataSize as i32
                        > 0x1000
                    {
                        task_set(
                            sLinkReceiveTaskId.get(),
                            tCurrentBlock_WrapFrom,
                            task_get(sLinkReceiveTaskId.get(), tCurrentBlock_End),
                        );
                        task_set(sLinkReceiveTaskId.get(), tCurrentBlock_End, 0);
                    }
                    let dest: *mut u8 = gLinkBattleRecvBuffer
                        .at(task_get(sLinkReceiveTaskId.get(), tCurrentBlock_End));
                    let src: *mut u8 = recvBuffer;
                    for j in 0..(dataSize as i32 + 8) {
                        *dest.at(j) = *src.at(j);
                    }
                    task_set(
                        sLinkReceiveTaskId.get(),
                        tCurrentBlock_End,
                        task_get(sLinkReceiveTaskId.get(), tCurrentBlock_End) + dataSize as i16 + 8,
                    );
                }
            }
            i += 1;
        }
    }
}
pub(crate) unsafe fn Task_HandleCopyReceivedLinkBuffersData(taskId: u8) {
    let mut blockSize: u16 = 0;
    let mut battler: u8 = 0;
    let mut playerId: u8 = 0;
    if task_get(taskId, tCurrentBlock_Start) != task_get(taskId, tCurrentBlock_End) {
        if task_get(taskId, tCurrentBlock_Start) > task_get(taskId, tCurrentBlock_End)
            && task_get(taskId, tCurrentBlock_Start) == task_get(taskId, tCurrentBlock_WrapFrom)
        {
            task_set(taskId, tCurrentBlock_WrapFrom, 0);
            task_set(taskId, tCurrentBlock_Start, 0);
        }
        battler = *gLinkBattleRecvBuffer
            .at(task_get(taskId, tCurrentBlock_Start) as i32 + LINK_BUFF_ACTIVE_BATTLER);
        blockSize = *gLinkBattleRecvBuffer
            .at(task_get(taskId, tCurrentBlock_Start) as i32 + LINK_BUFF_SIZE_LO)
            as u16
            | (*gLinkBattleRecvBuffer
                .at(task_get(taskId, tCurrentBlock_Start) as i32 + LINK_BUFF_SIZE_HI)
                as u16)
                << 8;
        match *gLinkBattleRecvBuffer.at(task_get(taskId, tCurrentBlock_Start) as i32) {
            B_COMM_TO_CONTROLLER => {
                if gBattleControllerExecFlags
                    & (*(&raw const crate::util::gBitTable).cast::<CArray<u32, 0>>())[battler]
                    != 0
                {
                    return;
                }
                memcpy(
                    gBattleBufferA[battler].as_mut_ptr(),
                    gLinkBattleRecvBuffer
                        .at(task_get(taskId, tCurrentBlock_Start) as i32 + LINK_BUFF_DATA),
                    blockSize as u32,
                );
                MarkBattlerReceivedLinkData(battler);
                if gBattleTypeFlags & BATTLE_TYPE_IS_MASTER == 0 {
                    gBattlerAttacker =
                        *gLinkBattleRecvBuffer
                            .at(task_get(taskId, tCurrentBlock_Start) as i32 + LINK_BUFF_ATTACKER);
                    gBattlerTarget = *gLinkBattleRecvBuffer
                        .at(task_get(taskId, tCurrentBlock_Start) as i32 + LINK_BUFF_TARGET);
                    gAbsentBattlerFlags =
                        *gLinkBattleRecvBuffer.at(task_get(taskId, tCurrentBlock_Start) as i32
                            + LINK_BUFF_ABSENT_BATTLER_FLAGS);
                    gEffectBattler =
                        *gLinkBattleRecvBuffer
                            .at(task_get(taskId, tCurrentBlock_Start) as i32
                                + LINK_BUFF_EFFECT_BATTLER);
                }
            }
            B_COMM_TO_ENGINE => {
                memcpy(
                    gBattleBufferB[battler].as_mut_ptr(),
                    gLinkBattleRecvBuffer
                        .at(task_get(taskId, tCurrentBlock_Start) as i32 + LINK_BUFF_DATA),
                    blockSize as u32,
                );
            }
            B_COMM_CONTROLLER_IS_DONE => {
                playerId = *gLinkBattleRecvBuffer
                    .at(task_get(taskId, tCurrentBlock_Start) as i32 + LINK_BUFF_DATA);
                gBattleControllerExecFlags &= !shl_u32(gBitTable[battler], playerId as u32 * 4);
            }
            _ => {}
        }
        task_set(
            taskId,
            tCurrentBlock_Start,
            task_get(taskId, tCurrentBlock_Start) + blockSize as i16 + LINK_BUFF_DATA as i16,
        );
    }
}
pub unsafe fn BtlController_EmitGetMonData(bufferId: u8, requestId: u8, monToCheck: u8) {
    sBattleBuffersTransferData[0] = CONTROLLER_GETMONDATA;
    sBattleBuffersTransferData[1] = requestId;
    sBattleBuffersTransferData[2] = monToCheck;
    sBattleBuffersTransferData[3] = 0;
    PrepareBufferDataTransfer(bufferId, sBattleBuffersTransferData.as_mut_ptr(), 4);
}
unsafe fn BtlController_EmitGetRawMonData(bufferId: u8, monId: u8, bytes: u8) {
    sBattleBuffersTransferData[0] = CONTROLLER_GETRAWMONDATA;
    sBattleBuffersTransferData[1] = monId;
    sBattleBuffersTransferData[2] = bytes;
    sBattleBuffersTransferData[3] = 0;
    PrepareBufferDataTransfer(bufferId, sBattleBuffersTransferData.as_mut_ptr(), 4);
}
#[unsafe(no_mangle)]
pub unsafe fn BtlController_EmitSetMonData(
    bufferId: u8,
    requestId: u8,
    monToCheck: u8,
    bytes: u8,
    mut data: *mut c_void,
) {
    sBattleBuffersTransferData[0] = CONTROLLER_SETMONDATA;
    sBattleBuffersTransferData[1] = requestId;
    sBattleBuffersTransferData[2] = monToCheck;
    for i in 0..(bytes as i32) {
        sBattleBuffersTransferData[3 + i] = *(({
            let t2 = data;
            data = (data as *mut u8).at(1) as *mut c_void;
            t2
        }) as *mut u8);
    }
    PrepareBufferDataTransfer(
        bufferId,
        sBattleBuffersTransferData.as_mut_ptr(),
        3 + bytes as u16,
    );
}
unsafe fn BtlController_EmitSetRawMonData(
    bufferId: u8,
    monId: u8,
    bytes: u8,
    mut data: *mut c_void,
) {
    sBattleBuffersTransferData[0] = CONTROLLER_SETRAWMONDATA;
    sBattleBuffersTransferData[1] = monId;
    sBattleBuffersTransferData[2] = bytes;
    for i in 0..(bytes as i32) {
        sBattleBuffersTransferData[3 + i] = *(({
            let t2 = data;
            data = (data as *mut u8).at(1) as *mut c_void;
            t2
        }) as *mut u8);
    }
    PrepareBufferDataTransfer(
        bufferId,
        sBattleBuffersTransferData.as_mut_ptr(),
        bytes as u16 + 3,
    );
}
pub unsafe fn BtlController_EmitLoadMonSprite(bufferId: u8) {
    sBattleBuffersTransferData[0] = CONTROLLER_LOADMONSPRITE;
    sBattleBuffersTransferData[1] = CONTROLLER_LOADMONSPRITE;
    sBattleBuffersTransferData[2] = CONTROLLER_LOADMONSPRITE;
    sBattleBuffersTransferData[3] = CONTROLLER_LOADMONSPRITE;
    PrepareBufferDataTransfer(bufferId, sBattleBuffersTransferData.as_mut_ptr(), 4);
}
pub unsafe fn BtlController_EmitSwitchInAnim(
    bufferId: u8,
    partyId: u8,
    dontClearSubstituteBit: u8,
) {
    sBattleBuffersTransferData[0] = CONTROLLER_SWITCHINANIM;
    sBattleBuffersTransferData[1] = partyId;
    sBattleBuffersTransferData[2] = dontClearSubstituteBit;
    sBattleBuffersTransferData[3] = 5;
    PrepareBufferDataTransfer(bufferId, sBattleBuffersTransferData.as_mut_ptr(), 4);
}
pub unsafe fn BtlController_EmitReturnMonToBall(bufferId: u8, skipAnim: u8) {
    sBattleBuffersTransferData[0] = CONTROLLER_RETURNMONTOBALL;
    sBattleBuffersTransferData[1] = skipAnim;
    PrepareBufferDataTransfer(bufferId, sBattleBuffersTransferData.as_mut_ptr(), 2);
}
pub unsafe fn BtlController_EmitDrawTrainerPic(bufferId: u8) {
    sBattleBuffersTransferData[0] = CONTROLLER_DRAWTRAINERPIC;
    sBattleBuffersTransferData[1] = CONTROLLER_DRAWTRAINERPIC;
    sBattleBuffersTransferData[2] = CONTROLLER_DRAWTRAINERPIC;
    sBattleBuffersTransferData[3] = CONTROLLER_DRAWTRAINERPIC;
    PrepareBufferDataTransfer(bufferId, sBattleBuffersTransferData.as_mut_ptr(), 4);
}
pub unsafe fn BtlController_EmitTrainerSlide(bufferId: u8) {
    sBattleBuffersTransferData[0] = CONTROLLER_TRAINERSLIDE;
    sBattleBuffersTransferData[1] = CONTROLLER_TRAINERSLIDE;
    sBattleBuffersTransferData[2] = CONTROLLER_TRAINERSLIDE;
    sBattleBuffersTransferData[3] = CONTROLLER_TRAINERSLIDE;
    PrepareBufferDataTransfer(bufferId, sBattleBuffersTransferData.as_mut_ptr(), 4);
}
pub unsafe fn BtlController_EmitTrainerSlideBack(bufferId: u8) {
    sBattleBuffersTransferData[0] = CONTROLLER_TRAINERSLIDEBACK;
    sBattleBuffersTransferData[1] = CONTROLLER_TRAINERSLIDEBACK;
    sBattleBuffersTransferData[2] = CONTROLLER_TRAINERSLIDEBACK;
    sBattleBuffersTransferData[3] = CONTROLLER_TRAINERSLIDEBACK;
    PrepareBufferDataTransfer(bufferId, sBattleBuffersTransferData.as_mut_ptr(), 4);
}
pub unsafe fn BtlController_EmitFaintAnimation(bufferId: u8) {
    sBattleBuffersTransferData[0] = CONTROLLER_FAINTANIMATION;
    sBattleBuffersTransferData[1] = CONTROLLER_FAINTANIMATION;
    sBattleBuffersTransferData[2] = CONTROLLER_FAINTANIMATION;
    sBattleBuffersTransferData[3] = CONTROLLER_FAINTANIMATION;
    PrepareBufferDataTransfer(bufferId, sBattleBuffersTransferData.as_mut_ptr(), 4);
}
unsafe fn BtlController_EmitPaletteFade(bufferId: u8) {
    sBattleBuffersTransferData[0] = CONTROLLER_PALETTEFADE;
    sBattleBuffersTransferData[1] = CONTROLLER_PALETTEFADE;
    sBattleBuffersTransferData[2] = CONTROLLER_PALETTEFADE;
    sBattleBuffersTransferData[3] = CONTROLLER_PALETTEFADE;
    PrepareBufferDataTransfer(bufferId, sBattleBuffersTransferData.as_mut_ptr(), 4);
}
unsafe fn BtlController_EmitSuccessBallThrowAnim(bufferId: u8) {
    sBattleBuffersTransferData[0] = CONTROLLER_SUCCESSBALLTHROWANIM;
    sBattleBuffersTransferData[1] = CONTROLLER_SUCCESSBALLTHROWANIM;
    sBattleBuffersTransferData[2] = CONTROLLER_SUCCESSBALLTHROWANIM;
    sBattleBuffersTransferData[3] = CONTROLLER_SUCCESSBALLTHROWANIM;
    PrepareBufferDataTransfer(bufferId, sBattleBuffersTransferData.as_mut_ptr(), 4);
}
pub unsafe fn BtlController_EmitBallThrowAnim(bufferId: u8, caseId: u8) {
    sBattleBuffersTransferData[0] = CONTROLLER_BALLTHROWANIM;
    sBattleBuffersTransferData[1] = caseId;
    PrepareBufferDataTransfer(bufferId, sBattleBuffersTransferData.as_mut_ptr(), 2);
}
unsafe fn BtlController_EmitPause(bufferId: u8, toWait: u8, mut data: *mut c_void) {
    sBattleBuffersTransferData[0] = CONTROLLER_PAUSE;
    sBattleBuffersTransferData[1] = toWait;
    for i in 0..(toWait as i32 * 3) {
        sBattleBuffersTransferData[2 + i] = *(({
            let t2 = data;
            data = (data as *mut u8).at(1) as *mut c_void;
            t2
        }) as *mut u8);
    }
    PrepareBufferDataTransfer(
        bufferId,
        sBattleBuffersTransferData.as_mut_ptr(),
        toWait as u16 * 3 + 2,
    );
}
pub unsafe fn BtlController_EmitMoveAnimation(
    bufferId: u8,
    r#move: u16,
    turnOfMove: u8,
    movePower: u16,
    dmg: i32,
    friendship: u8,
    disableStructPtr: *mut DisableStruct,
    multihit: u8,
) {
    sBattleBuffersTransferData[0] = CONTROLLER_MOVEANIMATION;
    sBattleBuffersTransferData[1] = r#move as u8;
    sBattleBuffersTransferData[2] = ((r#move as i32 & 0xFF00) >> 8) as u8;
    sBattleBuffersTransferData[3] = turnOfMove;
    sBattleBuffersTransferData[4] = movePower as u8;
    sBattleBuffersTransferData[5] = ((movePower as i32 & 0xFF00) >> 8) as u8;
    sBattleBuffersTransferData[6] = dmg as u8;
    sBattleBuffersTransferData[7] = ((dmg & 0x0000FF00) >> 8) as u8;
    sBattleBuffersTransferData[8] = ((dmg & 0x00FF0000) >> 16) as u8;
    sBattleBuffersTransferData[9] = ((dmg as u32 & 0xFF000000) >> 24) as u8;
    sBattleBuffersTransferData[10] = friendship;
    sBattleBuffersTransferData[11] = multihit;
    if AbilityBattleEffects(14, 0, 13, 0, 0) == 0 && AbilityBattleEffects(14, 0, 77, 0, 0) == 0 {
        sBattleBuffersTransferData[12] = gBattleWeather as u8;
        sBattleBuffersTransferData[13] = ((gBattleWeather as i32 & 0xFF00) >> 8) as u8;
    } else {
        sBattleBuffersTransferData[12] = 0;
        sBattleBuffersTransferData[13] = 0;
    }
    sBattleBuffersTransferData[14] = 0;
    sBattleBuffersTransferData[15] = 0;
    memcpy(
        &raw mut sBattleBuffersTransferData[16],
        disableStructPtr as *mut u8,
        28,
    );
    PrepareBufferDataTransfer(bufferId, sBattleBuffersTransferData.as_mut_ptr(), 44);
}
pub unsafe fn BtlController_EmitPrintString(bufferId: u8, stringId: u16) {
    sBattleBuffersTransferData[0] = CONTROLLER_PRINTSTRING;
    sBattleBuffersTransferData[1] = gBattleOutcome;
    sBattleBuffersTransferData[2] = stringId as u8;
    sBattleBuffersTransferData[3] = ((stringId as i32 & 0xFF00) >> 8) as u8;
    let stringInfo: *mut BattleMsgData =
        &raw mut sBattleBuffersTransferData[4] as *mut BattleMsgData;
    (*stringInfo).currentMove = gCurrentMove;
    (*stringInfo).originallyUsedMove = gChosenMove;
    (*stringInfo).lastItem = gLastUsedItem;
    (*stringInfo).lastAbility = gLastUsedAbility;
    (*stringInfo).scrActive = gBattleScripting.battler;
    (*stringInfo).bakScriptPartyIdx = (*gBattleStruct).scriptPartyIdx;
    (*stringInfo).hpScale = (*gBattleStruct).hpScale;
    (*stringInfo).itemEffectBattler = gPotentialItemEffectBattler;
    (*stringInfo).moveType = (*(&raw const crate::data::pokemon::gBattleMoves)
        .cast::<CArray<BattleMove, 0>>())[gCurrentMove]
        .r#type;
    for i in 0..(MAX_BATTLERS_COUNT as i32) {
        (*stringInfo).abilities[i] = gBattleMons[i].ability;
    }
    let mut i: i32 = 0;
    while i
        < (if 16 >= (if 14 >= 11 { 14 } else { 11 }) {
            16
        } else {
            if 14 >= 11 { 14 } else { 11 }
        })
    {
        (*stringInfo).textBuffs[0][i] = gBattleTextBuff1[i];
        (*stringInfo).textBuffs[1][i] = gBattleTextBuff2[i];
        (*stringInfo).textBuffs[2][i] = gBattleTextBuff3[i];
        i += 1;
    }
    PrepareBufferDataTransfer(bufferId, sBattleBuffersTransferData.as_mut_ptr(), 68);
}
pub unsafe fn BtlController_EmitPrintSelectionString(bufferId: u8, stringId: u16) {
    sBattleBuffersTransferData[0] = CONTROLLER_PRINTSTRINGPLAYERONLY;
    sBattleBuffersTransferData[1] = CONTROLLER_PRINTSTRINGPLAYERONLY;
    sBattleBuffersTransferData[2] = stringId as u8;
    sBattleBuffersTransferData[3] = ((stringId as i32 & 0xFF00) >> 8) as u8;
    let stringInfo: *mut BattleMsgData =
        &raw mut sBattleBuffersTransferData[4] as *mut BattleMsgData;
    (*stringInfo).currentMove = gCurrentMove;
    (*stringInfo).originallyUsedMove = gChosenMove;
    (*stringInfo).lastItem = gLastUsedItem;
    (*stringInfo).lastAbility = gLastUsedAbility;
    (*stringInfo).scrActive = gBattleScripting.battler;
    (*stringInfo).bakScriptPartyIdx = (*gBattleStruct).scriptPartyIdx;
    for i in 0..(MAX_BATTLERS_COUNT as i32) {
        (*stringInfo).abilities[i] = gBattleMons[i].ability;
    }
    let mut i: i32 = 0;
    while i
        < (if 16 >= (if 14 >= 11 { 14 } else { 11 }) {
            16
        } else {
            if 14 >= 11 { 14 } else { 11 }
        })
    {
        (*stringInfo).textBuffs[0][i] = gBattleTextBuff1[i];
        (*stringInfo).textBuffs[1][i] = gBattleTextBuff2[i];
        (*stringInfo).textBuffs[2][i] = gBattleTextBuff3[i];
        i += 1;
    }
    PrepareBufferDataTransfer(bufferId, sBattleBuffersTransferData.as_mut_ptr(), 68);
}
pub unsafe fn BtlController_EmitChooseAction(bufferId: u8, action: u8, itemId: u16) {
    sBattleBuffersTransferData[0] = CONTROLLER_CHOOSEACTION;
    sBattleBuffersTransferData[1] = action;
    sBattleBuffersTransferData[2] = itemId as u8;
    sBattleBuffersTransferData[3] = ((itemId as i32 & 0xFF00) >> 8) as u8;
    PrepareBufferDataTransfer(bufferId, sBattleBuffersTransferData.as_mut_ptr(), 4);
}
pub unsafe fn BtlController_EmitYesNoBox(bufferId: u8) {
    sBattleBuffersTransferData[0] = CONTROLLER_YESNOBOX;
    sBattleBuffersTransferData[1] = CONTROLLER_YESNOBOX;
    sBattleBuffersTransferData[2] = CONTROLLER_YESNOBOX;
    sBattleBuffersTransferData[3] = CONTROLLER_YESNOBOX;
    PrepareBufferDataTransfer(bufferId, sBattleBuffersTransferData.as_mut_ptr(), 4);
}
pub unsafe fn BtlController_EmitChooseMove(
    bufferId: u8,
    isDoubleBattle: u8,
    noPPNumber: u8,
    movePPData: *mut ChooseMoveStruct,
) {
    sBattleBuffersTransferData[0] = CONTROLLER_CHOOSEMOVE;
    sBattleBuffersTransferData[1] = isDoubleBattle;
    sBattleBuffersTransferData[2] = noPPNumber;
    sBattleBuffersTransferData[3] = 0;
    for i in 0..20i32 {
        sBattleBuffersTransferData[4 + i] = *(movePPData as *mut u8).at(i);
    }
    PrepareBufferDataTransfer(bufferId, sBattleBuffersTransferData.as_mut_ptr(), 24);
}
pub unsafe fn BtlController_EmitChooseItem(bufferId: u8, battlePartyOrder: *mut u8) {
    sBattleBuffersTransferData[0] = CONTROLLER_OPENBAG;
    for i in 0..3i32 {
        sBattleBuffersTransferData[1 + i] = *battlePartyOrder.at(i);
    }
    PrepareBufferDataTransfer(bufferId, sBattleBuffersTransferData.as_mut_ptr(), 4);
}
pub unsafe fn BtlController_EmitChoosePokemon(
    bufferId: u8,
    caseId: u8,
    slotId: u8,
    abilityId: u8,
    data: *mut u8,
) {
    sBattleBuffersTransferData[0] = CONTROLLER_CHOOSEPOKEMON;
    sBattleBuffersTransferData[1] = caseId;
    sBattleBuffersTransferData[2] = slotId;
    sBattleBuffersTransferData[3] = abilityId;
    for i in 0..3i32 {
        sBattleBuffersTransferData[4 + i] = *data.at(i);
    }
    PrepareBufferDataTransfer(bufferId, sBattleBuffersTransferData.as_mut_ptr(), 8);
}
unsafe fn BtlController_EmitCmd23(bufferId: u8) {
    sBattleBuffersTransferData[0] = CONTROLLER_23;
    sBattleBuffersTransferData[1] = CONTROLLER_23;
    sBattleBuffersTransferData[2] = CONTROLLER_23;
    sBattleBuffersTransferData[3] = CONTROLLER_23;
    PrepareBufferDataTransfer(bufferId, sBattleBuffersTransferData.as_mut_ptr(), 4);
}
pub unsafe fn BtlController_EmitHealthBarUpdate(bufferId: u8, hpValue: u16) {
    sBattleBuffersTransferData[0] = CONTROLLER_HEALTHBARUPDATE;
    sBattleBuffersTransferData[1] = 0;
    sBattleBuffersTransferData[2] = hpValue as i16 as u8;
    sBattleBuffersTransferData[3] = ((hpValue as i16 as i32 & 0xFF00) >> 8) as u8;
    PrepareBufferDataTransfer(bufferId, sBattleBuffersTransferData.as_mut_ptr(), 4);
}
pub unsafe fn BtlController_EmitExpUpdate(bufferId: u8, partyId: u8, expPoints: u16) {
    sBattleBuffersTransferData[0] = CONTROLLER_EXPUPDATE;
    sBattleBuffersTransferData[1] = partyId;
    sBattleBuffersTransferData[2] = expPoints as i16 as u8;
    sBattleBuffersTransferData[3] = ((expPoints as i16 as i32 & 0xFF00) >> 8) as u8;
    PrepareBufferDataTransfer(bufferId, sBattleBuffersTransferData.as_mut_ptr(), 4);
}
pub unsafe fn BtlController_EmitStatusIconUpdate(bufferId: u8, status1: u32, status2: u32) {
    sBattleBuffersTransferData[0] = CONTROLLER_STATUSICONUPDATE;
    sBattleBuffersTransferData[1] = status1 as u8;
    sBattleBuffersTransferData[2] = ((status1 & 0x0000FF00) >> 8) as u8;
    sBattleBuffersTransferData[3] = ((status1 & 0x00FF0000) >> 16) as u8;
    sBattleBuffersTransferData[4] = ((status1 & 0xFF000000) >> 24) as u8;
    sBattleBuffersTransferData[5] = status2 as u8;
    sBattleBuffersTransferData[6] = ((status2 & 0x0000FF00) >> 8) as u8;
    sBattleBuffersTransferData[7] = ((status2 & 0x00FF0000) >> 16) as u8;
    sBattleBuffersTransferData[8] = ((status2 & 0xFF000000) >> 24) as u8;
    PrepareBufferDataTransfer(bufferId, sBattleBuffersTransferData.as_mut_ptr(), 9);
}
pub unsafe fn BtlController_EmitStatusAnimation(bufferId: u8, status2: u8, status: u32) {
    sBattleBuffersTransferData[0] = CONTROLLER_STATUSANIMATION;
    sBattleBuffersTransferData[1] = status2;
    sBattleBuffersTransferData[2] = status as u8;
    sBattleBuffersTransferData[3] = ((status & 0x0000FF00) >> 8) as u8;
    sBattleBuffersTransferData[4] = ((status & 0x00FF0000) >> 16) as u8;
    sBattleBuffersTransferData[5] = ((status & 0xFF000000) >> 24) as u8;
    PrepareBufferDataTransfer(bufferId, sBattleBuffersTransferData.as_mut_ptr(), 6);
}
unsafe fn BtlController_EmitStatusXor(bufferId: u8, b: u8) {
    sBattleBuffersTransferData[0] = CONTROLLER_STATUSXOR;
    sBattleBuffersTransferData[1] = b;
    PrepareBufferDataTransfer(bufferId, sBattleBuffersTransferData.as_mut_ptr(), 2);
}
pub unsafe fn BtlController_EmitDataTransfer(bufferId: u8, size: u16, mut data: *mut c_void) {
    sBattleBuffersTransferData[0] = CONTROLLER_DATATRANSFER;
    sBattleBuffersTransferData[1] = CONTROLLER_DATATRANSFER;
    sBattleBuffersTransferData[2] = size as u8;
    sBattleBuffersTransferData[3] = ((size as i32 & 0xFF00) >> 8) as u8;
    for i in 0..(size as i32) {
        sBattleBuffersTransferData[4 + i] = *(({
            let t2 = data;
            data = (data as *mut u8).at(1) as *mut c_void;
            t2
        }) as *mut u8);
    }
    PrepareBufferDataTransfer(bufferId, sBattleBuffersTransferData.as_mut_ptr(), size + 4);
}
unsafe fn BtlController_EmitDMA3Transfer(
    bufferId: u8,
    dst: *mut c_void,
    size: u16,
    mut data: *mut c_void,
) {
    sBattleBuffersTransferData[0] = CONTROLLER_DMA3TRANSFER;
    sBattleBuffersTransferData[1] = dst as usize as u32 as u8;
    sBattleBuffersTransferData[2] = ((dst as usize as u32 & 0x0000FF00) >> 8) as u8;
    sBattleBuffersTransferData[3] = ((dst as usize as u32 & 0x00FF0000) >> 16) as u8;
    sBattleBuffersTransferData[4] = ((dst as usize as u32 & 0xFF000000) >> 24) as u8;
    sBattleBuffersTransferData[5] = size as u8;
    sBattleBuffersTransferData[6] = ((size as i32 & 0xFF00) >> 8) as u8;
    for i in 0..(size as i32) {
        sBattleBuffersTransferData[7 + i] = *(({
            let t2 = data;
            data = (data as *mut u8).at(1) as *mut c_void;
            t2
        }) as *mut u8);
    }
    PrepareBufferDataTransfer(bufferId, sBattleBuffersTransferData.as_mut_ptr(), size + 7);
}
unsafe fn BtlController_EmitPlayBGM(bufferId: u8, songId: u16, mut data: *mut c_void) {
    sBattleBuffersTransferData[0] = CONTROLLER_PLAYBGM;
    sBattleBuffersTransferData[1] = songId as u8;
    sBattleBuffersTransferData[2] = ((songId as i32 & 0xFF00) >> 8) as u8;
    for i in 0..(songId as i32) {
        sBattleBuffersTransferData[3 + i] = *(({
            let t2 = data;
            data = (data as *mut u8).at(1) as *mut c_void;
            t2
        }) as *mut u8);
    }
    PrepareBufferDataTransfer(
        bufferId,
        sBattleBuffersTransferData.as_mut_ptr(),
        songId + 3,
    );
}
unsafe fn BtlController_EmitCmd32(bufferId: u8, size: u16, mut data: *mut c_void) {
    sBattleBuffersTransferData[0] = CONTROLLER_32;
    sBattleBuffersTransferData[1] = size as u8;
    sBattleBuffersTransferData[2] = ((size as i32 & 0xFF00) >> 8) as u8;
    for i in 0..(size as i32) {
        sBattleBuffersTransferData[3 + i] = *(({
            let t2 = data;
            data = (data as *mut u8).at(1) as *mut c_void;
            t2
        }) as *mut u8);
    }
    PrepareBufferDataTransfer(bufferId, sBattleBuffersTransferData.as_mut_ptr(), size + 3);
}
pub unsafe fn BtlController_EmitTwoReturnValues(bufferId: u8, ret8: u8, ret16: u16) {
    sBattleBuffersTransferData[0] = CONTROLLER_TWORETURNVALUES;
    sBattleBuffersTransferData[1] = ret8;
    sBattleBuffersTransferData[2] = ret16 as u8;
    sBattleBuffersTransferData[3] = ((ret16 as i32 & 0xFF00) >> 8) as u8;
    PrepareBufferDataTransfer(bufferId, sBattleBuffersTransferData.as_mut_ptr(), 4);
}
pub unsafe fn BtlController_EmitChosenMonReturnValue(
    bufferId: u8,
    partyId: u8,
    battlePartyOrder: *mut u8,
) {
    sBattleBuffersTransferData[0] = CONTROLLER_CHOSENMONRETURNVALUE;
    sBattleBuffersTransferData[1] = partyId;
    for i in 0..3i32 {
        sBattleBuffersTransferData[2 + i] = *battlePartyOrder.at(i);
    }
    PrepareBufferDataTransfer(bufferId, sBattleBuffersTransferData.as_mut_ptr(), 5);
}
pub unsafe fn BtlController_EmitOneReturnValue(bufferId: u8, ret: u16) {
    sBattleBuffersTransferData[0] = CONTROLLER_ONERETURNVALUE;
    sBattleBuffersTransferData[1] = ret as u8;
    sBattleBuffersTransferData[2] = ((ret as i32 & 0xFF00) >> 8) as u8;
    sBattleBuffersTransferData[3] = 0;
    PrepareBufferDataTransfer(bufferId, sBattleBuffersTransferData.as_mut_ptr(), 4);
}
pub unsafe fn BtlController_EmitOneReturnValue_Duplicate(bufferId: u8, ret: u16) {
    sBattleBuffersTransferData[0] = CONTROLLER_ONERETURNVALUE_DUPLICATE;
    sBattleBuffersTransferData[1] = ret as u8;
    sBattleBuffersTransferData[2] = ((ret as i32 & 0xFF00) >> 8) as u8;
    sBattleBuffersTransferData[3] = 0;
    PrepareBufferDataTransfer(bufferId, sBattleBuffersTransferData.as_mut_ptr(), 4);
}
unsafe fn BtlController_EmitClearUnkVar(bufferId: u8) {
    sBattleBuffersTransferData[0] = CONTROLLER_CLEARUNKVAR;
    sBattleBuffersTransferData[1] = CONTROLLER_CLEARUNKVAR;
    sBattleBuffersTransferData[2] = CONTROLLER_CLEARUNKVAR;
    sBattleBuffersTransferData[3] = CONTROLLER_CLEARUNKVAR;
    PrepareBufferDataTransfer(bufferId, sBattleBuffersTransferData.as_mut_ptr(), 4);
}
unsafe fn BtlController_EmitSetUnkVar(bufferId: u8, b: u8) {
    sBattleBuffersTransferData[0] = CONTROLLER_SETUNKVAR;
    sBattleBuffersTransferData[1] = b;
    PrepareBufferDataTransfer(bufferId, sBattleBuffersTransferData.as_mut_ptr(), 2);
}
unsafe fn BtlController_EmitClearUnkFlag(bufferId: u8) {
    sBattleBuffersTransferData[0] = CONTROLLER_CLEARUNKFLAG;
    sBattleBuffersTransferData[1] = CONTROLLER_CLEARUNKFLAG;
    sBattleBuffersTransferData[2] = CONTROLLER_CLEARUNKFLAG;
    sBattleBuffersTransferData[3] = CONTROLLER_CLEARUNKFLAG;
    PrepareBufferDataTransfer(bufferId, sBattleBuffersTransferData.as_mut_ptr(), 4);
}
unsafe fn BtlController_EmitToggleUnkFlag(bufferId: u8) {
    sBattleBuffersTransferData[0] = CONTROLLER_TOGGLEUNKFLAG;
    sBattleBuffersTransferData[1] = CONTROLLER_TOGGLEUNKFLAG;
    sBattleBuffersTransferData[2] = CONTROLLER_TOGGLEUNKFLAG;
    sBattleBuffersTransferData[3] = CONTROLLER_TOGGLEUNKFLAG;
    PrepareBufferDataTransfer(bufferId, sBattleBuffersTransferData.as_mut_ptr(), 4);
}
pub unsafe fn BtlController_EmitHitAnimation(bufferId: u8) {
    sBattleBuffersTransferData[0] = CONTROLLER_HITANIMATION;
    sBattleBuffersTransferData[1] = CONTROLLER_HITANIMATION;
    sBattleBuffersTransferData[2] = CONTROLLER_HITANIMATION;
    sBattleBuffersTransferData[3] = CONTROLLER_HITANIMATION;
    PrepareBufferDataTransfer(bufferId, sBattleBuffersTransferData.as_mut_ptr(), 4);
}
pub unsafe fn BtlController_EmitCantSwitch(bufferId: u8) {
    sBattleBuffersTransferData[0] = CONTROLLER_CANTSWITCH;
    sBattleBuffersTransferData[1] = CONTROLLER_CANTSWITCH;
    sBattleBuffersTransferData[2] = CONTROLLER_CANTSWITCH;
    sBattleBuffersTransferData[3] = CONTROLLER_CANTSWITCH;
    PrepareBufferDataTransfer(bufferId, sBattleBuffersTransferData.as_mut_ptr(), 4);
}
pub unsafe fn BtlController_EmitPlaySE(bufferId: u8, songId: u16) {
    sBattleBuffersTransferData[0] = CONTROLLER_PLAYSE;
    sBattleBuffersTransferData[1] = songId as u8;
    sBattleBuffersTransferData[2] = ((songId as i32 & 0xFF00) >> 8) as u8;
    sBattleBuffersTransferData[3] = 0;
    PrepareBufferDataTransfer(bufferId, sBattleBuffersTransferData.as_mut_ptr(), 4);
}
pub unsafe fn BtlController_EmitPlayFanfareOrBGM(bufferId: u8, songId: u16, playBGM: u8) {
    sBattleBuffersTransferData[0] = CONTROLLER_PLAYFANFAREORBGM;
    sBattleBuffersTransferData[1] = songId as u8;
    sBattleBuffersTransferData[2] = ((songId as i32 & 0xFF00) >> 8) as u8;
    sBattleBuffersTransferData[3] = playBGM;
    PrepareBufferDataTransfer(bufferId, sBattleBuffersTransferData.as_mut_ptr(), 4);
}
pub unsafe fn BtlController_EmitFaintingCry(bufferId: u8) {
    sBattleBuffersTransferData[0] = CONTROLLER_FAINTINGCRY;
    sBattleBuffersTransferData[1] = CONTROLLER_FAINTINGCRY;
    sBattleBuffersTransferData[2] = CONTROLLER_FAINTINGCRY;
    sBattleBuffersTransferData[3] = CONTROLLER_FAINTINGCRY;
    PrepareBufferDataTransfer(bufferId, sBattleBuffersTransferData.as_mut_ptr(), 4);
}
pub unsafe fn BtlController_EmitIntroSlide(bufferId: u8, environmentId: u8) {
    sBattleBuffersTransferData[0] = CONTROLLER_INTROSLIDE;
    sBattleBuffersTransferData[1] = environmentId;
    PrepareBufferDataTransfer(bufferId, sBattleBuffersTransferData.as_mut_ptr(), 2);
}
pub unsafe fn BtlController_EmitIntroTrainerBallThrow(bufferId: u8) {
    sBattleBuffersTransferData[0] = CONTROLLER_INTROTRAINERBALLTHROW;
    sBattleBuffersTransferData[1] = CONTROLLER_INTROTRAINERBALLTHROW;
    sBattleBuffersTransferData[2] = CONTROLLER_INTROTRAINERBALLTHROW;
    sBattleBuffersTransferData[3] = CONTROLLER_INTROTRAINERBALLTHROW;
    PrepareBufferDataTransfer(bufferId, sBattleBuffersTransferData.as_mut_ptr(), 4);
}
pub unsafe fn BtlController_EmitDrawPartyStatusSummary(
    bufferId: u8,
    hpAndStatus: *mut HpAndStatus,
    flags: u8,
) {
    sBattleBuffersTransferData[0] = CONTROLLER_DRAWPARTYSTATUSSUMMARY;
    sBattleBuffersTransferData[1] = flags & 127;
    sBattleBuffersTransferData[2] = ((flags as i32 & PARTY_SUMM_SKIP_DRAW_DELAY as i32) >> 7) as u8;
    sBattleBuffersTransferData[3] = CONTROLLER_DRAWPARTYSTATUSSUMMARY;
    for i in 0..48i32 {
        sBattleBuffersTransferData[4 + i] = *(hpAndStatus as *mut u8).at(i);
    }
    PrepareBufferDataTransfer(bufferId, sBattleBuffersTransferData.as_mut_ptr(), 52);
}
pub unsafe fn BtlController_EmitHidePartyStatusSummary(bufferId: u8) {
    sBattleBuffersTransferData[0] = CONTROLLER_HIDEPARTYSTATUSSUMMARY;
    sBattleBuffersTransferData[1] = CONTROLLER_HIDEPARTYSTATUSSUMMARY;
    sBattleBuffersTransferData[2] = CONTROLLER_HIDEPARTYSTATUSSUMMARY;
    sBattleBuffersTransferData[3] = CONTROLLER_HIDEPARTYSTATUSSUMMARY;
    PrepareBufferDataTransfer(bufferId, sBattleBuffersTransferData.as_mut_ptr(), 4);
}
pub unsafe fn BtlController_EmitEndBounceEffect(bufferId: u8) {
    sBattleBuffersTransferData[0] = CONTROLLER_ENDBOUNCE;
    sBattleBuffersTransferData[1] = CONTROLLER_ENDBOUNCE;
    sBattleBuffersTransferData[2] = CONTROLLER_ENDBOUNCE;
    sBattleBuffersTransferData[3] = CONTROLLER_ENDBOUNCE;
    PrepareBufferDataTransfer(bufferId, sBattleBuffersTransferData.as_mut_ptr(), 4);
}
pub unsafe fn BtlController_EmitSpriteInvisibility(bufferId: u8, isInvisible: u8) {
    sBattleBuffersTransferData[0] = CONTROLLER_SPRITEINVISIBILITY;
    sBattleBuffersTransferData[1] = isInvisible;
    sBattleBuffersTransferData[2] = CONTROLLER_SPRITEINVISIBILITY;
    sBattleBuffersTransferData[3] = CONTROLLER_SPRITEINVISIBILITY;
    PrepareBufferDataTransfer(bufferId, sBattleBuffersTransferData.as_mut_ptr(), 4);
}
pub unsafe fn BtlController_EmitBattleAnimation(bufferId: u8, animationId: u8, argument: u16) {
    sBattleBuffersTransferData[0] = CONTROLLER_BATTLEANIMATION;
    sBattleBuffersTransferData[1] = animationId;
    sBattleBuffersTransferData[2] = argument as u8;
    sBattleBuffersTransferData[3] = ((argument as i32 & 0xFF00) >> 8) as u8;
    PrepareBufferDataTransfer(bufferId, sBattleBuffersTransferData.as_mut_ptr(), 4);
}
pub unsafe fn BtlController_EmitLinkStandbyMsg(bufferId: u8, mode: u8, record: u32) {
    let record_: u8 = record as u8;
    sBattleBuffersTransferData[0] = CONTROLLER_LINKSTANDBYMSG;
    sBattleBuffersTransferData[1] = mode;
    if record_ != 0 {
        sBattleBuffersTransferData[3] = {
            sBattleBuffersTransferData[2] =
                RecordedBattle_BufferNewBattlerData(&raw mut sBattleBuffersTransferData[4]);
            sBattleBuffersTransferData[2]
        };
    } else {
        sBattleBuffersTransferData[3] = {
            sBattleBuffersTransferData[2] = 0;
            sBattleBuffersTransferData[2]
        };
    }
    PrepareBufferDataTransfer(
        bufferId,
        sBattleBuffersTransferData.as_mut_ptr(),
        sBattleBuffersTransferData[2] as u16 + 4,
    );
}
pub unsafe fn BtlController_EmitResetActionMoveSelection(bufferId: u8, caseId: u8) {
    sBattleBuffersTransferData[0] = CONTROLLER_RESETACTIONMOVESELECTION;
    sBattleBuffersTransferData[1] = caseId;
    PrepareBufferDataTransfer(bufferId, sBattleBuffersTransferData.as_mut_ptr(), 2);
}
pub unsafe fn BtlController_EmitEndLinkBattle(bufferId: u8, battleOutcome: u8) {
    sBattleBuffersTransferData[0] = CONTROLLER_ENDLINKBATTLE;
    sBattleBuffersTransferData[1] = battleOutcome;
    sBattleBuffersTransferData[2] = (*gSaveBlock2Ptr).frontier.disableRecordBattle();
    sBattleBuffersTransferData[3] = (*gSaveBlock2Ptr).frontier.disableRecordBattle();
    sBattleBuffersTransferData[5] = {
        sBattleBuffersTransferData[4] =
            RecordedBattle_BufferNewBattlerData(&raw mut sBattleBuffersTransferData[6]);
        sBattleBuffersTransferData[4]
    };
    PrepareBufferDataTransfer(
        bufferId,
        sBattleBuffersTransferData.as_mut_ptr(),
        sBattleBuffersTransferData[4] as u16 + 6,
    );
}
