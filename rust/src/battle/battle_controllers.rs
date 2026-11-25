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

#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub(crate) static sLinkSendTaskId: crate::global::Global<u8> = crate::global::Global::new(0);
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub(crate) static sLinkReceiveTaskId: crate::global::Global<u8> = crate::global::Global::new(0);
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub(crate) static sUnused: crate::global::Global<u8> = crate::global::Global::new(0);
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub static gUnusedControllerStruct: crate::global::Global<UnusedControllerStruct> =
    crate::global::Global::new(unsafe { zeroed() });
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub(crate) static sBattleBuffersTransferData: crate::global::Global<Aligned<CArray<u8, 256>>> =
    crate::global::Global::new(Aligned(unsafe { zeroed() }));

pub unsafe fn HandleLinkBattleSetup() {
    if gBattleTypeFlags.get() & BATTLE_TYPE_LINK != 0 {
        if gWirelessCommType.get() != 0 {
            SetWirelessCommType1();
        }
        if gReceivedRemoteLinkPlayers.get() == 0 {
            OpenLink();
        }
        CreateTask(Some(Task_WaitForLinkPlayerConnection), 0);
        CreateTasksForSendRecvLinkBuffers();
    }
}
pub unsafe fn SetUpBattleVarsAndBirchZigzagoon() {
    crate::gl!(gBattleMainFunc = Some(BeginBattleIntroDummy));
    let mut i: i32 = 0;
    while i < MAX_BATTLERS_COUNT as i32 {
        crate::gl!(gBattlerControllerFuncs[i] = Some(BattleControllerDummy));
        crate::gl!(gBattlerPositions[i] = 0xFF);
        crate::gl!(gActionSelectionCursor[i] = 0);
        crate::gl!(gMoveSelectionCursor[i] = 0);
        i += 1;
    }
    HandleLinkBattleSetup();
    gBattleControllerExecFlags.set(0);
    ClearBattleAnimationVars();
    ClearBattleMonForms();
    gActiveBattler.set(0);
    BattleAI_HandleItemUseBeforeAISetup(0xF);
    if gBattleTypeFlags.get() & BATTLE_TYPE_FIRST_BATTLE != 0 {
        ZeroEnemyPartyMons();
        CreateMon(
            crate::gl!(&raw gEnemyParty[0]),
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
            crate::gl!(&raw gEnemyParty[0]),
            MON_DATA_HELD_ITEM,
            &raw mut i as *mut c_void,
        );
    }
    gUnusedFirstBattleVar1.set(0);
    gUnusedFirstBattleVar2.set(0);
}
pub unsafe fn InitBattleControllers() {
    let mut i: i32 = 0;
    if gBattleTypeFlags.get() & BATTLE_TYPE_RECORDED == 0 {
        RecordedBattle_Init(B_RECORD_MODE_RECORDING);
    } else {
        RecordedBattle_Init(B_RECORD_MODE_PLAYBACK);
    }
    if gBattleTypeFlags.get() & BATTLE_TYPE_RECORDED == 0 {
        RecordedBattle_SaveParties();
    }
    if gBattleTypeFlags.get() & BATTLE_TYPE_LINK != 0 {
        InitLinkBtlControllers();
    } else {
        InitSinglePlayerBtlControllers();
    }
    SetBattlePartyIds();
    if gBattleTypeFlags.get() & BATTLE_TYPE_MULTI == 0 {
        i = 0;
        while i < gBattlersCount.get() as i32 {
            BufferBattlePartyCurrentOrderBySide(i as u8, 0);
            i += 1;
        }
    }
    for i in 0..96i32 {
        *(&raw mut (*crate::gl!(gBattleStruct)).tvMovePoints as *mut u8).at(i) = 0;
    }
    for i in 0..104i32 {
        *(&raw mut (*crate::gl!(gBattleStruct)).tv as *mut u8).at(i) = 0;
    }
}
unsafe fn InitSinglePlayerBtlControllers() {
    if gBattleTypeFlags.get() & BATTLE_TYPE_INGAME_PARTNER != 0 {
        crate::gl!(gBattleMainFunc = Some(BeginBattleIntro));
        if gBattleTypeFlags.get() & BATTLE_TYPE_RECORDED != 0 {
            crate::gl!(gBattlerControllerFuncs[0] = Some(SetControllerToRecordedPlayer));
            crate::gl!(gBattlerPositions[0] = B_POSITION_PLAYER_LEFT);
            crate::gl!(gBattlerControllerFuncs[1] = Some(SetControllerToOpponent));
            crate::gl!(gBattlerPositions[1] = B_POSITION_OPPONENT_LEFT);
            crate::gl!(gBattlerControllerFuncs[2] = Some(SetControllerToPlayerPartner));
            crate::gl!(gBattlerPositions[2] = B_POSITION_PLAYER_RIGHT);
            crate::gl!(gBattlerControllerFuncs[3] = Some(SetControllerToOpponent));
            crate::gl!(gBattlerPositions[3] = B_POSITION_OPPONENT_RIGHT);
        } else {
            crate::gl!(gBattlerControllerFuncs[0] = Some(SetControllerToPlayer));
            crate::gl!(gBattlerPositions[0] = B_POSITION_PLAYER_LEFT);
            crate::gl!(gBattlerControllerFuncs[1] = Some(SetControllerToOpponent));
            crate::gl!(gBattlerPositions[1] = B_POSITION_OPPONENT_LEFT);
            crate::gl!(gBattlerControllerFuncs[2] = Some(SetControllerToPlayerPartner));
            crate::gl!(gBattlerPositions[2] = B_POSITION_PLAYER_RIGHT);
            crate::gl!(gBattlerControllerFuncs[3] = Some(SetControllerToOpponent));
            crate::gl!(gBattlerPositions[3] = B_POSITION_OPPONENT_RIGHT);
        }
        gBattlersCount.set(MAX_BATTLERS_COUNT);
        BufferBattlePartyCurrentOrderBySide(0, 0);
        BufferBattlePartyCurrentOrderBySide(1, 0);
        BufferBattlePartyCurrentOrderBySide(2, 1);
        BufferBattlePartyCurrentOrderBySide(3, 1);
        crate::gl!(gBattlerPartyIndexes[0] = 0);
        crate::gl!(gBattlerPartyIndexes[1] = 0);
        crate::gl!(gBattlerPartyIndexes[2] = 3);
        crate::gl!(gBattlerPartyIndexes[3] = 3);
    } else if gBattleTypeFlags.get() & BATTLE_TYPE_DOUBLE == 0 {
        crate::gl!(gBattleMainFunc = Some(BeginBattleIntro));
        if gBattleTypeFlags.get() & BATTLE_TYPE_SAFARI != 0 {
            crate::gl!(gBattlerControllerFuncs[0] = Some(SetControllerToSafari));
        } else if gBattleTypeFlags.get() & BATTLE_TYPE_WALLY_TUTORIAL != 0 {
            crate::gl!(gBattlerControllerFuncs[0] = Some(SetControllerToWally));
        } else {
            crate::gl!(gBattlerControllerFuncs[0] = Some(SetControllerToPlayer));
        }
        crate::gl!(gBattlerPositions[0] = B_POSITION_PLAYER_LEFT);
        crate::gl!(gBattlerControllerFuncs[1] = Some(SetControllerToOpponent));
        crate::gl!(gBattlerPositions[1] = B_POSITION_OPPONENT_LEFT);
        gBattlersCount.set(2);
        if gBattleTypeFlags.get() & BATTLE_TYPE_RECORDED != 0 {
            if gBattleTypeFlags.get() & BATTLE_TYPE_RECORDED_LINK != 0 {
                if gBattleTypeFlags.get() & 0x80000000 != 0 {
                    crate::gl!(gBattleMainFunc = Some(BeginBattleIntro));
                    crate::gl!(gBattlerControllerFuncs[0] = Some(SetControllerToRecordedPlayer));
                    crate::gl!(gBattlerPositions[0] = B_POSITION_PLAYER_LEFT);
                    crate::gl!(gBattlerControllerFuncs[1] = Some(SetControllerToRecordedOpponent));
                    crate::gl!(gBattlerPositions[1] = B_POSITION_OPPONENT_LEFT);
                    gBattlersCount.set(2);
                } else {
                    crate::gl!(gBattlerControllerFuncs[1] = Some(SetControllerToRecordedPlayer));
                    crate::gl!(gBattlerPositions[1] = B_POSITION_PLAYER_LEFT);
                    crate::gl!(gBattlerControllerFuncs[0] = Some(SetControllerToRecordedOpponent));
                    crate::gl!(gBattlerPositions[0] = B_POSITION_OPPONENT_LEFT);
                    gBattlersCount.set(2);
                }
            } else {
                crate::gl!(gBattlerControllerFuncs[0] = Some(SetControllerToRecordedPlayer));
                crate::gl!(gBattlerPositions[0] = B_POSITION_PLAYER_LEFT);
                crate::gl!(gBattlerControllerFuncs[1] = Some(SetControllerToOpponent));
                crate::gl!(gBattlerPositions[1] = B_POSITION_OPPONENT_LEFT);
            }
        }
    } else {
        crate::gl!(gBattleMainFunc = Some(BeginBattleIntro));
        crate::gl!(gBattlerControllerFuncs[0] = Some(SetControllerToPlayer));
        crate::gl!(gBattlerPositions[0] = B_POSITION_PLAYER_LEFT);
        crate::gl!(gBattlerControllerFuncs[1] = Some(SetControllerToOpponent));
        crate::gl!(gBattlerPositions[1] = B_POSITION_OPPONENT_LEFT);
        crate::gl!(gBattlerControllerFuncs[2] = Some(SetControllerToPlayer));
        crate::gl!(gBattlerPositions[2] = B_POSITION_PLAYER_RIGHT);
        crate::gl!(gBattlerControllerFuncs[3] = Some(SetControllerToOpponent));
        crate::gl!(gBattlerPositions[3] = B_POSITION_OPPONENT_RIGHT);
        gBattlersCount.set(MAX_BATTLERS_COUNT);
        if gBattleTypeFlags.get() & BATTLE_TYPE_RECORDED != 0 {
            if gBattleTypeFlags.get() & BATTLE_TYPE_MULTI != 0
                && gBattleTypeFlags.get() & BATTLE_TYPE_BATTLE_TOWER != 0
            {
                crate::gl!(gBattleMainFunc = Some(BeginBattleIntro));
                crate::gl!(gBattlerControllerFuncs[0] = Some(SetControllerToRecordedPlayer));
                crate::gl!(gBattlerPositions[0] = B_POSITION_PLAYER_LEFT);
                crate::gl!(gBattlerControllerFuncs[1] = Some(SetControllerToOpponent));
                crate::gl!(gBattlerPositions[1] = B_POSITION_OPPONENT_LEFT);
                crate::gl!(gBattlerControllerFuncs[2] = Some(SetControllerToRecordedPlayer));
                crate::gl!(gBattlerPositions[2] = B_POSITION_PLAYER_RIGHT);
                crate::gl!(gBattlerControllerFuncs[3] = Some(SetControllerToOpponent));
                crate::gl!(gBattlerPositions[3] = B_POSITION_OPPONENT_RIGHT);
                gBattlersCount.set(MAX_BATTLERS_COUNT);
                BufferBattlePartyCurrentOrderBySide(0, 0);
                BufferBattlePartyCurrentOrderBySide(1, 0);
                BufferBattlePartyCurrentOrderBySide(2, 1);
                BufferBattlePartyCurrentOrderBySide(3, 1);
                crate::gl!(gBattlerPartyIndexes[0] = 0);
                crate::gl!(gBattlerPartyIndexes[1] = 0);
                crate::gl!(gBattlerPartyIndexes[2] = 3);
                crate::gl!(gBattlerPartyIndexes[3] = 3);
            } else if gBattleTypeFlags.get() & BATTLE_TYPE_MULTI != 0 {
                let multiplayerId: u8 = gRecordedBattleMultiplayerId.get();
                for i in 0..MAX_LINK_PLAYERS {
                    match crate::gl!(gLinkPlayers[i].id) {
                        0 | 3 => {
                            BufferBattlePartyCurrentOrderBySide(
                                crate::gl!(gLinkPlayers[i].id) as u8,
                                0,
                            );
                        }
                        1 | 2 => {
                            BufferBattlePartyCurrentOrderBySide(
                                crate::gl!(gLinkPlayers[i].id) as u8,
                                1,
                            );
                        }
                        _ => {}
                    }
                    if i == multiplayerId as i32 {
                        (*gBattlerControllerFuncs.as_ptr())[crate::gl!(gLinkPlayers[i].id)] =
                            Some(SetControllerToRecordedPlayer);
                        match crate::gl!(gLinkPlayers[i].id) {
                            0 | 3 => {
                                {
                                    let gl_v = B_POSITION_PLAYER_LEFT;
                                    let gl_i0 = crate::gl!(gLinkPlayers[i].id);
                                    crate::gl!(gBattlerPositions[gl_i0] = gl_v)
                                };
                                (*gBattlerPartyIndexes.as_ptr())[crate::gl!(gLinkPlayers[i].id)] =
                                    0;
                            }
                            1 | 2 => {
                                {
                                    let gl_v = B_POSITION_PLAYER_RIGHT;
                                    let gl_i1 = crate::gl!(gLinkPlayers[i].id);
                                    crate::gl!(gBattlerPositions[gl_i1] = gl_v)
                                };
                                (*gBattlerPartyIndexes.as_ptr())[crate::gl!(gLinkPlayers[i].id)] =
                                    3;
                            }
                            _ => {}
                        }
                    } else if crate::gl!(gLinkPlayers[i].id) as i32 & 1 == 0
                        && crate::gl!(gLinkPlayers[multiplayerId].id) as i32 & 1 == 0
                        || crate::gl!(gLinkPlayers[i].id) as i32 & 1 != 0
                            && crate::gl!(gLinkPlayers[multiplayerId].id) as i32 & 1 != 0
                    {
                        (*gBattlerControllerFuncs.as_ptr())[crate::gl!(gLinkPlayers[i].id)] =
                            Some(SetControllerToRecordedPlayer);
                        match crate::gl!(gLinkPlayers[i].id) {
                            0 | 3 => {
                                {
                                    let gl_v = B_POSITION_PLAYER_LEFT;
                                    let gl_i2 = crate::gl!(gLinkPlayers[i].id);
                                    crate::gl!(gBattlerPositions[gl_i2] = gl_v)
                                };
                                (*gBattlerPartyIndexes.as_ptr())[crate::gl!(gLinkPlayers[i].id)] =
                                    0;
                            }
                            1 | 2 => {
                                {
                                    let gl_v = B_POSITION_PLAYER_RIGHT;
                                    let gl_i3 = crate::gl!(gLinkPlayers[i].id);
                                    crate::gl!(gBattlerPositions[gl_i3] = gl_v)
                                };
                                (*gBattlerPartyIndexes.as_ptr())[crate::gl!(gLinkPlayers[i].id)] =
                                    3;
                            }
                            _ => {}
                        }
                    } else {
                        (*gBattlerControllerFuncs.as_ptr())[crate::gl!(gLinkPlayers[i].id)] =
                            Some(SetControllerToRecordedOpponent);
                        match crate::gl!(gLinkPlayers[i].id) {
                            0 | 3 => {
                                {
                                    let gl_v = B_POSITION_OPPONENT_LEFT;
                                    let gl_i4 = crate::gl!(gLinkPlayers[i].id);
                                    crate::gl!(gBattlerPositions[gl_i4] = gl_v)
                                };
                                (*gBattlerPartyIndexes.as_ptr())[crate::gl!(gLinkPlayers[i].id)] =
                                    0;
                            }
                            1 | 2 => {
                                {
                                    let gl_v = B_POSITION_OPPONENT_RIGHT;
                                    let gl_i5 = crate::gl!(gLinkPlayers[i].id);
                                    crate::gl!(gBattlerPositions[gl_i5] = gl_v)
                                };
                                (*gBattlerPartyIndexes.as_ptr())[crate::gl!(gLinkPlayers[i].id)] =
                                    3;
                            }
                            _ => {}
                        }
                    }
                }
            } else if gBattleTypeFlags.get() & BATTLE_TYPE_IS_MASTER != 0 {
                crate::gl!(gBattlerControllerFuncs[0] = Some(SetControllerToRecordedPlayer));
                crate::gl!(gBattlerPositions[0] = B_POSITION_PLAYER_LEFT);
                crate::gl!(gBattlerControllerFuncs[2] = Some(SetControllerToRecordedPlayer));
                crate::gl!(gBattlerPositions[2] = B_POSITION_PLAYER_RIGHT);
                if gBattleTypeFlags.get() & BATTLE_TYPE_RECORDED_LINK != 0 {
                    crate::gl!(gBattlerControllerFuncs[1] = Some(SetControllerToRecordedOpponent));
                    crate::gl!(gBattlerPositions[1] = B_POSITION_OPPONENT_LEFT);
                    crate::gl!(gBattlerControllerFuncs[3] = Some(SetControllerToRecordedOpponent));
                    crate::gl!(gBattlerPositions[3] = B_POSITION_OPPONENT_RIGHT);
                } else {
                    crate::gl!(gBattlerControllerFuncs[1] = Some(SetControllerToOpponent));
                    crate::gl!(gBattlerPositions[1] = B_POSITION_OPPONENT_LEFT);
                    crate::gl!(gBattlerControllerFuncs[3] = Some(SetControllerToOpponent));
                    crate::gl!(gBattlerPositions[3] = B_POSITION_OPPONENT_RIGHT);
                }
            } else {
                crate::gl!(gBattlerControllerFuncs[1] = Some(SetControllerToRecordedPlayer));
                crate::gl!(gBattlerPositions[1] = B_POSITION_PLAYER_LEFT);
                crate::gl!(gBattlerControllerFuncs[3] = Some(SetControllerToRecordedPlayer));
                crate::gl!(gBattlerPositions[3] = B_POSITION_PLAYER_RIGHT);
                if gBattleTypeFlags.get() & BATTLE_TYPE_RECORDED_LINK != 0 {
                    crate::gl!(gBattlerControllerFuncs[0] = Some(SetControllerToRecordedOpponent));
                    crate::gl!(gBattlerPositions[0] = B_POSITION_OPPONENT_LEFT);
                    crate::gl!(gBattlerControllerFuncs[2] = Some(SetControllerToRecordedOpponent));
                    crate::gl!(gBattlerPositions[2] = B_POSITION_OPPONENT_RIGHT);
                } else {
                    crate::gl!(gBattlerControllerFuncs[0] = Some(SetControllerToOpponent));
                    crate::gl!(gBattlerPositions[0] = B_POSITION_OPPONENT_LEFT);
                    crate::gl!(gBattlerControllerFuncs[2] = Some(SetControllerToOpponent));
                    crate::gl!(gBattlerPositions[2] = B_POSITION_OPPONENT_RIGHT);
                }
            }
        }
    }
}
unsafe fn InitLinkBtlControllers() {
    let mut multiplayerId: u8 = 0;
    if gBattleTypeFlags.get() & BATTLE_TYPE_DOUBLE == 0 {
        if gBattleTypeFlags.get() & BATTLE_TYPE_IS_MASTER != 0 {
            crate::gl!(gBattleMainFunc = Some(BeginBattleIntro));
            crate::gl!(gBattlerControllerFuncs[0] = Some(SetControllerToPlayer));
            crate::gl!(gBattlerPositions[0] = B_POSITION_PLAYER_LEFT);
            crate::gl!(gBattlerControllerFuncs[1] = Some(SetControllerToLinkOpponent));
            crate::gl!(gBattlerPositions[1] = B_POSITION_OPPONENT_LEFT);
            gBattlersCount.set(2);
        } else {
            crate::gl!(gBattlerControllerFuncs[1] = Some(SetControllerToPlayer));
            crate::gl!(gBattlerPositions[1] = B_POSITION_PLAYER_LEFT);
            crate::gl!(gBattlerControllerFuncs[0] = Some(SetControllerToLinkOpponent));
            crate::gl!(gBattlerPositions[0] = B_POSITION_OPPONENT_LEFT);
            gBattlersCount.set(2);
        }
    } else if gBattleTypeFlags.get() & BATTLE_TYPE_MULTI == 0
        && gBattleTypeFlags.get() & BATTLE_TYPE_DOUBLE != 0
    {
        if gBattleTypeFlags.get() & BATTLE_TYPE_IS_MASTER != 0 {
            crate::gl!(gBattleMainFunc = Some(BeginBattleIntro));
            crate::gl!(gBattlerControllerFuncs[0] = Some(SetControllerToPlayer));
            crate::gl!(gBattlerPositions[0] = B_POSITION_PLAYER_LEFT);
            crate::gl!(gBattlerControllerFuncs[1] = Some(SetControllerToLinkOpponent));
            crate::gl!(gBattlerPositions[1] = B_POSITION_OPPONENT_LEFT);
            crate::gl!(gBattlerControllerFuncs[2] = Some(SetControllerToPlayer));
            crate::gl!(gBattlerPositions[2] = B_POSITION_PLAYER_RIGHT);
            crate::gl!(gBattlerControllerFuncs[3] = Some(SetControllerToLinkOpponent));
            crate::gl!(gBattlerPositions[3] = B_POSITION_OPPONENT_RIGHT);
            gBattlersCount.set(MAX_BATTLERS_COUNT);
        } else {
            crate::gl!(gBattlerControllerFuncs[1] = Some(SetControllerToPlayer));
            crate::gl!(gBattlerPositions[1] = B_POSITION_PLAYER_LEFT);
            crate::gl!(gBattlerControllerFuncs[0] = Some(SetControllerToLinkOpponent));
            crate::gl!(gBattlerPositions[0] = B_POSITION_OPPONENT_LEFT);
            crate::gl!(gBattlerControllerFuncs[3] = Some(SetControllerToPlayer));
            crate::gl!(gBattlerPositions[3] = B_POSITION_PLAYER_RIGHT);
            crate::gl!(gBattlerControllerFuncs[2] = Some(SetControllerToLinkOpponent));
            crate::gl!(gBattlerPositions[2] = B_POSITION_OPPONENT_RIGHT);
            gBattlersCount.set(MAX_BATTLERS_COUNT);
        }
    } else if gBattleTypeFlags.get() & BATTLE_TYPE_BATTLE_TOWER != 0 {
        if gBattleTypeFlags.get() & BATTLE_TYPE_IS_MASTER != 0 {
            crate::gl!(gBattleMainFunc = Some(BeginBattleIntro));
            crate::gl!(gBattlerControllerFuncs[0] = Some(SetControllerToPlayer));
            crate::gl!(gBattlerPositions[0] = B_POSITION_PLAYER_LEFT);
            crate::gl!(gBattlerControllerFuncs[1] = Some(SetControllerToOpponent));
            crate::gl!(gBattlerPositions[1] = B_POSITION_OPPONENT_LEFT);
            crate::gl!(gBattlerControllerFuncs[2] = Some(SetControllerToLinkPartner));
            crate::gl!(gBattlerPositions[2] = B_POSITION_PLAYER_RIGHT);
            crate::gl!(gBattlerControllerFuncs[3] = Some(SetControllerToOpponent));
            crate::gl!(gBattlerPositions[3] = B_POSITION_OPPONENT_RIGHT);
            gBattlersCount.set(MAX_BATTLERS_COUNT);
        } else {
            crate::gl!(gBattlerControllerFuncs[0] = Some(SetControllerToLinkPartner));
            crate::gl!(gBattlerPositions[0] = B_POSITION_PLAYER_LEFT);
            crate::gl!(gBattlerControllerFuncs[1] = Some(SetControllerToLinkOpponent));
            crate::gl!(gBattlerPositions[1] = B_POSITION_OPPONENT_LEFT);
            crate::gl!(gBattlerControllerFuncs[2] = Some(SetControllerToPlayer));
            crate::gl!(gBattlerPositions[2] = B_POSITION_PLAYER_RIGHT);
            crate::gl!(gBattlerControllerFuncs[3] = Some(SetControllerToLinkOpponent));
            crate::gl!(gBattlerPositions[3] = B_POSITION_OPPONENT_RIGHT);
            gBattlersCount.set(MAX_BATTLERS_COUNT);
        }
        BufferBattlePartyCurrentOrderBySide(0, 0);
        BufferBattlePartyCurrentOrderBySide(1, 0);
        BufferBattlePartyCurrentOrderBySide(2, 1);
        BufferBattlePartyCurrentOrderBySide(3, 1);
        crate::gl!(gBattlerPartyIndexes[0] = 0);
        crate::gl!(gBattlerPartyIndexes[1] = 0);
        crate::gl!(gBattlerPartyIndexes[2] = 3);
        crate::gl!(gBattlerPartyIndexes[3] = 3);
    } else {
        multiplayerId = GetMultiplayerId();
        if gBattleTypeFlags.get() & BATTLE_TYPE_IS_MASTER != 0 {
            crate::gl!(gBattleMainFunc = Some(BeginBattleIntro));
        }
        for i in 0..MAX_LINK_PLAYERS {
            match crate::gl!(gLinkPlayers[i].id) {
                0 | 3 => {
                    BufferBattlePartyCurrentOrderBySide(crate::gl!(gLinkPlayers[i].id) as u8, 0);
                }
                1 | 2 => {
                    BufferBattlePartyCurrentOrderBySide(crate::gl!(gLinkPlayers[i].id) as u8, 1);
                }
                _ => {}
            }
            if i == multiplayerId as i32 {
                (*gBattlerControllerFuncs.as_ptr())[crate::gl!(gLinkPlayers[i].id)] =
                    Some(SetControllerToPlayer);
                match crate::gl!(gLinkPlayers[i].id) {
                    0 | 3 => {
                        {
                            let gl_v = B_POSITION_PLAYER_LEFT;
                            let gl_i6 = crate::gl!(gLinkPlayers[i].id);
                            crate::gl!(gBattlerPositions[gl_i6] = gl_v)
                        };
                        (*gBattlerPartyIndexes.as_ptr())[crate::gl!(gLinkPlayers[i].id)] = 0;
                    }
                    1 | 2 => {
                        {
                            let gl_v = B_POSITION_PLAYER_RIGHT;
                            let gl_i7 = crate::gl!(gLinkPlayers[i].id);
                            crate::gl!(gBattlerPositions[gl_i7] = gl_v)
                        };
                        (*gBattlerPartyIndexes.as_ptr())[crate::gl!(gLinkPlayers[i].id)] = 3;
                    }
                    _ => {}
                }
            } else {
                if crate::gl!(gLinkPlayers[i].id) as i32 & 1 == 0
                    && crate::gl!(gLinkPlayers[multiplayerId].id) as i32 & 1 == 0
                    || crate::gl!(gLinkPlayers[i].id) as i32 & 1 != 0
                        && crate::gl!(gLinkPlayers[multiplayerId].id) as i32 & 1 != 0
                {
                    (*gBattlerControllerFuncs.as_ptr())[crate::gl!(gLinkPlayers[i].id)] =
                        Some(SetControllerToLinkPartner);
                    match crate::gl!(gLinkPlayers[i].id) {
                        0 | 3 => {
                            {
                                let gl_v = B_POSITION_PLAYER_LEFT;
                                let gl_i8 = crate::gl!(gLinkPlayers[i].id);
                                crate::gl!(gBattlerPositions[gl_i8] = gl_v)
                            };
                            (*gBattlerPartyIndexes.as_ptr())[crate::gl!(gLinkPlayers[i].id)] = 0;
                        }
                        1 | 2 => {
                            {
                                let gl_v = B_POSITION_PLAYER_RIGHT;
                                let gl_i9 = crate::gl!(gLinkPlayers[i].id);
                                crate::gl!(gBattlerPositions[gl_i9] = gl_v)
                            };
                            (*gBattlerPartyIndexes.as_ptr())[crate::gl!(gLinkPlayers[i].id)] = 3;
                        }
                        _ => {}
                    }
                } else {
                    (*gBattlerControllerFuncs.as_ptr())[crate::gl!(gLinkPlayers[i].id)] =
                        Some(SetControllerToLinkOpponent);
                    match crate::gl!(gLinkPlayers[i].id) {
                        0 | 3 => {
                            {
                                let gl_v = B_POSITION_OPPONENT_LEFT;
                                let gl_i10 = crate::gl!(gLinkPlayers[i].id);
                                crate::gl!(gBattlerPositions[gl_i10] = gl_v)
                            };
                            (*gBattlerPartyIndexes.as_ptr())[crate::gl!(gLinkPlayers[i].id)] = 0;
                        }
                        1 | 2 => {
                            {
                                let gl_v = B_POSITION_OPPONENT_RIGHT;
                                let gl_i11 = crate::gl!(gLinkPlayers[i].id);
                                crate::gl!(gBattlerPositions[gl_i11] = gl_v)
                            };
                            (*gBattlerPartyIndexes.as_ptr())[crate::gl!(gLinkPlayers[i].id)] = 3;
                        }
                        _ => {}
                    }
                }
            }
        }
        gBattlersCount.set(MAX_BATTLERS_COUNT);
    }
}
unsafe fn SetBattlePartyIds() {
    let mut i: i32 = 0;
    if gBattleTypeFlags.get() & BATTLE_TYPE_MULTI == 0 {
        i = 0;
        while i < gBattlersCount.get() as i32 {
            for j in 0..PARTY_SIZE {
                if i < 2 {
                    if crate::gl!(gBattlerPositions[i]) as i32 & 1 == B_SIDE_PLAYER as i32 {
                        if GetMonData2(crate::gl!(&raw gPlayerParty[j]), MON_DATA_HP) != 0
                            && GetMonData2(
                                crate::gl!(&raw gPlayerParty[j]),
                                MON_DATA_SPECIES_OR_EGG,
                            ) != SPECIES_NONE as u32
                            && GetMonData2(
                                crate::gl!(&raw gPlayerParty[j]),
                                MON_DATA_SPECIES_OR_EGG,
                            ) != SPECIES_EGG
                            && GetMonData2(crate::gl!(&raw gPlayerParty[j]), MON_DATA_IS_EGG) == 0
                        {
                            crate::gl!(gBattlerPartyIndexes[i] = j as u16);
                            break;
                        }
                    } else {
                        if GetMonData2(crate::gl!(&raw gEnemyParty[j]), MON_DATA_HP) != 0
                            && GetMonData2(crate::gl!(&raw gEnemyParty[j]), MON_DATA_SPECIES_OR_EGG)
                                != SPECIES_NONE as u32
                            && GetMonData2(crate::gl!(&raw gEnemyParty[j]), MON_DATA_SPECIES_OR_EGG)
                                != SPECIES_EGG
                            && GetMonData2(crate::gl!(&raw gEnemyParty[j]), MON_DATA_IS_EGG) == 0
                        {
                            crate::gl!(gBattlerPartyIndexes[i] = j as u16);
                            break;
                        }
                    }
                } else {
                    if crate::gl!(gBattlerPositions[i]) as i32 & 1 == B_SIDE_PLAYER as i32 {
                        if GetMonData2(crate::gl!(&raw gPlayerParty[j]), MON_DATA_HP) != 0
                            && GetMonData2(crate::gl!(&raw gPlayerParty[j]), MON_DATA_SPECIES)
                                != SPECIES_NONE as u32
                            && GetMonData2(
                                crate::gl!(&raw gPlayerParty[j]),
                                MON_DATA_SPECIES_OR_EGG,
                            ) != SPECIES_EGG
                            && GetMonData2(crate::gl!(&raw gPlayerParty[j]), MON_DATA_IS_EGG) == 0
                            && ({
                                let gl_i0 = i - 2;
                                crate::gl!(gBattlerPartyIndexes[gl_i0])
                            }) as i32
                                != j
                        {
                            crate::gl!(gBattlerPartyIndexes[i] = j as u16);
                            break;
                        }
                    } else {
                        if GetMonData2(crate::gl!(&raw gEnemyParty[j]), MON_DATA_HP) != 0
                            && GetMonData2(crate::gl!(&raw gEnemyParty[j]), MON_DATA_SPECIES_OR_EGG)
                                != SPECIES_NONE as u32
                            && GetMonData2(crate::gl!(&raw gEnemyParty[j]), MON_DATA_SPECIES_OR_EGG)
                                != SPECIES_EGG
                            && GetMonData2(crate::gl!(&raw gEnemyParty[j]), MON_DATA_IS_EGG) == 0
                            && ({
                                let gl_i1 = i - 2;
                                crate::gl!(gBattlerPartyIndexes[gl_i1])
                            }) as i32
                                != j
                        {
                            crate::gl!(gBattlerPartyIndexes[i] = j as u16);
                            break;
                        }
                    }
                }
            }
            i += 1;
        }
        if gBattleTypeFlags.get() & BATTLE_TYPE_TWO_OPPONENTS != 0 {
            crate::gl!(gBattlerPartyIndexes[1] = 0);
            crate::gl!(gBattlerPartyIndexes[3] = 3);
        }
    }
}
unsafe fn PrepareBufferDataTransfer(bufferId: u8, mut data: *mut u8, size: u16) {
    if gBattleTypeFlags.get() & BATTLE_TYPE_LINK != 0 {
        PrepareBufferDataTransferLink(bufferId, size, data);
    } else {
        match bufferId {
            B_COMM_TO_CONTROLLER => {
                for i in 0..(size as i32) {
                    {
                        let gl_i2 = gActiveBattler.get();
                        crate::gl!(gBattleBufferA[gl_i2][i] = *data)
                    };
                    data = data.at(1);
                }
            }
            B_COMM_TO_ENGINE => {
                for i in 0..(size as i32) {
                    {
                        let gl_v = *data;
                        let gl_i12 = gActiveBattler.get();
                        crate::gl!(gBattleBufferB[gl_i12][i] = gl_v)
                    };
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
    crate::gl!(sUnused = 0);
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
    *(*gLinkBattleSendBuffer.as_ptr())
        .at(task_get(sLinkSendTaskId.get(), tCurrentBlock_End) as i32 + LINK_BUFF_BUFFER_ID) =
        bufferId;
    *(*gLinkBattleSendBuffer.as_ptr())
        .at(task_get(sLinkSendTaskId.get(), tCurrentBlock_End) as i32 + LINK_BUFF_ACTIVE_BATTLER) =
        gActiveBattler.get();
    *(*gLinkBattleSendBuffer.as_ptr())
        .at(task_get(sLinkSendTaskId.get(), tCurrentBlock_End) as i32 + LINK_BUFF_ATTACKER) =
        gBattlerAttacker.get();
    *(*gLinkBattleSendBuffer.as_ptr())
        .at(task_get(sLinkSendTaskId.get(), tCurrentBlock_End) as i32 + LINK_BUFF_TARGET) =
        gBattlerTarget.get();
    *(*gLinkBattleSendBuffer.as_ptr())
        .at(task_get(sLinkSendTaskId.get(), tCurrentBlock_End) as i32 + LINK_BUFF_SIZE_LO) =
        alignedSize as u8;
    *(*gLinkBattleSendBuffer.as_ptr())
        .at(task_get(sLinkSendTaskId.get(), tCurrentBlock_End) as i32 + LINK_BUFF_SIZE_HI) =
        ((alignedSize & 0x0000FF00) >> 8) as u8;
    *(*gLinkBattleSendBuffer.as_ptr())
        .at(task_get(sLinkSendTaskId.get(), tCurrentBlock_End) as i32
            + LINK_BUFF_ABSENT_BATTLER_FLAGS) = gAbsentBattlerFlags.get();
    *(*gLinkBattleSendBuffer.as_ptr())
        .at(task_get(sLinkSendTaskId.get(), tCurrentBlock_End) as i32 + LINK_BUFF_EFFECT_BATTLER) =
        gEffectBattler.get();
    for i in 0..(size as i32) {
        *(*gLinkBattleSendBuffer.as_ptr())
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
                if gWirelessCommType.get() != 0 {
                    task_set(taskId, tState, task_get(taskId, tState) + 1);
                } else {
                    if gBattleTypeFlags.get() & BATTLE_TYPE_BATTLE_TOWER != 0 {
                        numPlayers = 2;
                    } else {
                        numPlayers = (if gBattleTypeFlags.get() & BATTLE_TYPE_MULTI != 0 {
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
                        blockSize = (*(*gLinkBattleSendBuffer.as_ptr()).at(task_get(
                            taskId,
                            tCurrentBlock_Start,
                        )
                            as i32
                            + LINK_BUFF_SIZE_LO) as u16
                            | (*(*gLinkBattleSendBuffer.as_ptr()).at(task_get(
                                taskId,
                                tCurrentBlock_Start,
                            )
                                as i32
                                + LINK_BUFF_SIZE_HI) as u16)
                                << 8)
                            + LINK_BUFF_DATA as u16;
                        SendBlock(
                            BitmaskAllOtherLinkPlayers(),
                            (*gLinkBattleSendBuffer.as_ptr()).at(task_get(
                                taskId,
                                tCurrentBlock_Start,
                            )
                                as i32) as *mut c_void,
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
                    blockSize =
                        *(*gLinkBattleSendBuffer.as_ptr())
                            .at(task_get(taskId, tCurrentBlock_Start) as i32 + LINK_BUFF_SIZE_LO)
                            as u16
                            | (*(*gLinkBattleSendBuffer.as_ptr()).at(task_get(
                                taskId,
                                tCurrentBlock_Start,
                            )
                                as i32
                                + LINK_BUFF_SIZE_HI) as u16)
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
    if gReceivedRemoteLinkPlayers.get() != 0
        && gBattleTypeFlags.get() & BATTLE_TYPE_LINK_IN_BATTLE != 0
    {
        DestroyTask_RfuIdle();
        i = 0;
        while i < GetLinkPlayerCount() {
            if GetBlockReceivedStatus() as u32
                & (*(&raw const crate::util::gBitTable).cast::<CArray<u32, 0>>())[i]
                != 0
            {
                ResetBlockReceivedFlag(i);
                recvBuffer = (*gBlockRecvBuffer.as_ptr())[i].as_mut_ptr() as *mut u8;
                {
                    let dataSize: u16 = crate::gl!(gBlockRecvBuffer[i][2]);
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
                    let dest: *mut u8 = (*gLinkBattleRecvBuffer.as_ptr())
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
        battler = *(*gLinkBattleRecvBuffer.as_ptr())
            .at(task_get(taskId, tCurrentBlock_Start) as i32 + LINK_BUFF_ACTIVE_BATTLER);
        blockSize = *(*gLinkBattleRecvBuffer.as_ptr())
            .at(task_get(taskId, tCurrentBlock_Start) as i32 + LINK_BUFF_SIZE_LO)
            as u16
            | (*(*gLinkBattleRecvBuffer.as_ptr())
                .at(task_get(taskId, tCurrentBlock_Start) as i32 + LINK_BUFF_SIZE_HI)
                as u16)
                << 8;
        match *(*gLinkBattleRecvBuffer.as_ptr()).at(task_get(taskId, tCurrentBlock_Start) as i32) {
            B_COMM_TO_CONTROLLER => {
                if gBattleControllerExecFlags.get()
                    & (*(&raw const crate::util::gBitTable).cast::<CArray<u32, 0>>())[battler]
                    != 0
                {
                    return;
                }
                memcpy(
                    (*gBattleBufferA.as_ptr())[battler].as_mut_ptr(),
                    (*gLinkBattleRecvBuffer.as_ptr())
                        .at(task_get(taskId, tCurrentBlock_Start) as i32 + LINK_BUFF_DATA),
                    blockSize as u32,
                );
                MarkBattlerReceivedLinkData(battler);
                if gBattleTypeFlags.get() & BATTLE_TYPE_IS_MASTER == 0 {
                    gBattlerAttacker.set(*(*gLinkBattleRecvBuffer.as_ptr()).at(task_get(
                        taskId,
                        tCurrentBlock_Start,
                    )
                        as i32
                        + LINK_BUFF_ATTACKER));
                    gBattlerTarget.set(*(*gLinkBattleRecvBuffer.as_ptr()).at(task_get(
                        taskId,
                        tCurrentBlock_Start,
                    )
                        as i32
                        + LINK_BUFF_TARGET));
                    gAbsentBattlerFlags.set(*(*gLinkBattleRecvBuffer.as_ptr()).at(task_get(
                        taskId,
                        tCurrentBlock_Start,
                    )
                        as i32
                        + LINK_BUFF_ABSENT_BATTLER_FLAGS));
                    gEffectBattler.set(*(*gLinkBattleRecvBuffer.as_ptr()).at(task_get(
                        taskId,
                        tCurrentBlock_Start,
                    )
                        as i32
                        + LINK_BUFF_EFFECT_BATTLER));
                }
            }
            B_COMM_TO_ENGINE => {
                memcpy(
                    (*gBattleBufferB.as_ptr())[battler].as_mut_ptr(),
                    (*gLinkBattleRecvBuffer.as_ptr())
                        .at(task_get(taskId, tCurrentBlock_Start) as i32 + LINK_BUFF_DATA),
                    blockSize as u32,
                );
            }
            B_COMM_CONTROLLER_IS_DONE => {
                playerId =
                    *(*gLinkBattleRecvBuffer.as_ptr())
                        .at(task_get(taskId, tCurrentBlock_Start) as i32 + LINK_BUFF_DATA);
                {
                    let rhs = !shl_u32(gBitTable[battler], playerId as u32 * 4);
                    gBattleControllerExecFlags.set(gBattleControllerExecFlags.get() & rhs)
                };
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
    crate::gl!(sBattleBuffersTransferData[0] = CONTROLLER_GETMONDATA);
    crate::gl!(sBattleBuffersTransferData[1] = requestId);
    crate::gl!(sBattleBuffersTransferData[2] = monToCheck);
    crate::gl!(sBattleBuffersTransferData[3] = 0);
    PrepareBufferDataTransfer(
        bufferId,
        (*sBattleBuffersTransferData.as_ptr()).as_mut_ptr(),
        4,
    );
}
unsafe fn BtlController_EmitGetRawMonData(bufferId: u8, monId: u8, bytes: u8) {
    crate::gl!(sBattleBuffersTransferData[0] = CONTROLLER_GETRAWMONDATA);
    crate::gl!(sBattleBuffersTransferData[1] = monId);
    crate::gl!(sBattleBuffersTransferData[2] = bytes);
    crate::gl!(sBattleBuffersTransferData[3] = 0);
    PrepareBufferDataTransfer(
        bufferId,
        (*sBattleBuffersTransferData.as_ptr()).as_mut_ptr(),
        4,
    );
}
#[unsafe(no_mangle)]
pub unsafe fn BtlController_EmitSetMonData(
    bufferId: u8,
    requestId: u8,
    monToCheck: u8,
    bytes: u8,
    mut data: *mut c_void,
) {
    crate::gl!(sBattleBuffersTransferData[0] = CONTROLLER_SETMONDATA);
    crate::gl!(sBattleBuffersTransferData[1] = requestId);
    crate::gl!(sBattleBuffersTransferData[2] = monToCheck);
    for i in 0..(bytes as i32) {
        {
            let gl_v = *(({
                let t2 = data;
                data = (data as *mut u8).at(1) as *mut c_void;
                t2
            }) as *mut u8);
            let gl_i13 = 3 + i;
            crate::gl!(sBattleBuffersTransferData[gl_i13] = gl_v)
        };
    }
    PrepareBufferDataTransfer(
        bufferId,
        (*sBattleBuffersTransferData.as_ptr()).as_mut_ptr(),
        3 + bytes as u16,
    );
}
unsafe fn BtlController_EmitSetRawMonData(
    bufferId: u8,
    monId: u8,
    bytes: u8,
    mut data: *mut c_void,
) {
    crate::gl!(sBattleBuffersTransferData[0] = CONTROLLER_SETRAWMONDATA);
    crate::gl!(sBattleBuffersTransferData[1] = monId);
    crate::gl!(sBattleBuffersTransferData[2] = bytes);
    for i in 0..(bytes as i32) {
        {
            let gl_v = *(({
                let t2 = data;
                data = (data as *mut u8).at(1) as *mut c_void;
                t2
            }) as *mut u8);
            let gl_i14 = 3 + i;
            crate::gl!(sBattleBuffersTransferData[gl_i14] = gl_v)
        };
    }
    PrepareBufferDataTransfer(
        bufferId,
        (*sBattleBuffersTransferData.as_ptr()).as_mut_ptr(),
        bytes as u16 + 3,
    );
}
pub unsafe fn BtlController_EmitLoadMonSprite(bufferId: u8) {
    crate::gl!(sBattleBuffersTransferData[0] = CONTROLLER_LOADMONSPRITE);
    crate::gl!(sBattleBuffersTransferData[1] = CONTROLLER_LOADMONSPRITE);
    crate::gl!(sBattleBuffersTransferData[2] = CONTROLLER_LOADMONSPRITE);
    crate::gl!(sBattleBuffersTransferData[3] = CONTROLLER_LOADMONSPRITE);
    PrepareBufferDataTransfer(
        bufferId,
        (*sBattleBuffersTransferData.as_ptr()).as_mut_ptr(),
        4,
    );
}
pub unsafe fn BtlController_EmitSwitchInAnim(
    bufferId: u8,
    partyId: u8,
    dontClearSubstituteBit: u8,
) {
    crate::gl!(sBattleBuffersTransferData[0] = CONTROLLER_SWITCHINANIM);
    crate::gl!(sBattleBuffersTransferData[1] = partyId);
    crate::gl!(sBattleBuffersTransferData[2] = dontClearSubstituteBit);
    crate::gl!(sBattleBuffersTransferData[3] = 5);
    PrepareBufferDataTransfer(
        bufferId,
        (*sBattleBuffersTransferData.as_ptr()).as_mut_ptr(),
        4,
    );
}
pub unsafe fn BtlController_EmitReturnMonToBall(bufferId: u8, skipAnim: u8) {
    crate::gl!(sBattleBuffersTransferData[0] = CONTROLLER_RETURNMONTOBALL);
    crate::gl!(sBattleBuffersTransferData[1] = skipAnim);
    PrepareBufferDataTransfer(
        bufferId,
        (*sBattleBuffersTransferData.as_ptr()).as_mut_ptr(),
        2,
    );
}
pub unsafe fn BtlController_EmitDrawTrainerPic(bufferId: u8) {
    crate::gl!(sBattleBuffersTransferData[0] = CONTROLLER_DRAWTRAINERPIC);
    crate::gl!(sBattleBuffersTransferData[1] = CONTROLLER_DRAWTRAINERPIC);
    crate::gl!(sBattleBuffersTransferData[2] = CONTROLLER_DRAWTRAINERPIC);
    crate::gl!(sBattleBuffersTransferData[3] = CONTROLLER_DRAWTRAINERPIC);
    PrepareBufferDataTransfer(
        bufferId,
        (*sBattleBuffersTransferData.as_ptr()).as_mut_ptr(),
        4,
    );
}
pub unsafe fn BtlController_EmitTrainerSlide(bufferId: u8) {
    crate::gl!(sBattleBuffersTransferData[0] = CONTROLLER_TRAINERSLIDE);
    crate::gl!(sBattleBuffersTransferData[1] = CONTROLLER_TRAINERSLIDE);
    crate::gl!(sBattleBuffersTransferData[2] = CONTROLLER_TRAINERSLIDE);
    crate::gl!(sBattleBuffersTransferData[3] = CONTROLLER_TRAINERSLIDE);
    PrepareBufferDataTransfer(
        bufferId,
        (*sBattleBuffersTransferData.as_ptr()).as_mut_ptr(),
        4,
    );
}
pub unsafe fn BtlController_EmitTrainerSlideBack(bufferId: u8) {
    crate::gl!(sBattleBuffersTransferData[0] = CONTROLLER_TRAINERSLIDEBACK);
    crate::gl!(sBattleBuffersTransferData[1] = CONTROLLER_TRAINERSLIDEBACK);
    crate::gl!(sBattleBuffersTransferData[2] = CONTROLLER_TRAINERSLIDEBACK);
    crate::gl!(sBattleBuffersTransferData[3] = CONTROLLER_TRAINERSLIDEBACK);
    PrepareBufferDataTransfer(
        bufferId,
        (*sBattleBuffersTransferData.as_ptr()).as_mut_ptr(),
        4,
    );
}
pub unsafe fn BtlController_EmitFaintAnimation(bufferId: u8) {
    crate::gl!(sBattleBuffersTransferData[0] = CONTROLLER_FAINTANIMATION);
    crate::gl!(sBattleBuffersTransferData[1] = CONTROLLER_FAINTANIMATION);
    crate::gl!(sBattleBuffersTransferData[2] = CONTROLLER_FAINTANIMATION);
    crate::gl!(sBattleBuffersTransferData[3] = CONTROLLER_FAINTANIMATION);
    PrepareBufferDataTransfer(
        bufferId,
        (*sBattleBuffersTransferData.as_ptr()).as_mut_ptr(),
        4,
    );
}
unsafe fn BtlController_EmitPaletteFade(bufferId: u8) {
    crate::gl!(sBattleBuffersTransferData[0] = CONTROLLER_PALETTEFADE);
    crate::gl!(sBattleBuffersTransferData[1] = CONTROLLER_PALETTEFADE);
    crate::gl!(sBattleBuffersTransferData[2] = CONTROLLER_PALETTEFADE);
    crate::gl!(sBattleBuffersTransferData[3] = CONTROLLER_PALETTEFADE);
    PrepareBufferDataTransfer(
        bufferId,
        (*sBattleBuffersTransferData.as_ptr()).as_mut_ptr(),
        4,
    );
}
unsafe fn BtlController_EmitSuccessBallThrowAnim(bufferId: u8) {
    crate::gl!(sBattleBuffersTransferData[0] = CONTROLLER_SUCCESSBALLTHROWANIM);
    crate::gl!(sBattleBuffersTransferData[1] = CONTROLLER_SUCCESSBALLTHROWANIM);
    crate::gl!(sBattleBuffersTransferData[2] = CONTROLLER_SUCCESSBALLTHROWANIM);
    crate::gl!(sBattleBuffersTransferData[3] = CONTROLLER_SUCCESSBALLTHROWANIM);
    PrepareBufferDataTransfer(
        bufferId,
        (*sBattleBuffersTransferData.as_ptr()).as_mut_ptr(),
        4,
    );
}
pub unsafe fn BtlController_EmitBallThrowAnim(bufferId: u8, caseId: u8) {
    crate::gl!(sBattleBuffersTransferData[0] = CONTROLLER_BALLTHROWANIM);
    crate::gl!(sBattleBuffersTransferData[1] = caseId);
    PrepareBufferDataTransfer(
        bufferId,
        (*sBattleBuffersTransferData.as_ptr()).as_mut_ptr(),
        2,
    );
}
unsafe fn BtlController_EmitPause(bufferId: u8, toWait: u8, mut data: *mut c_void) {
    crate::gl!(sBattleBuffersTransferData[0] = CONTROLLER_PAUSE);
    crate::gl!(sBattleBuffersTransferData[1] = toWait);
    for i in 0..(toWait as i32 * 3) {
        {
            let gl_v = *(({
                let t2 = data;
                data = (data as *mut u8).at(1) as *mut c_void;
                t2
            }) as *mut u8);
            let gl_i15 = 2 + i;
            crate::gl!(sBattleBuffersTransferData[gl_i15] = gl_v)
        };
    }
    PrepareBufferDataTransfer(
        bufferId,
        (*sBattleBuffersTransferData.as_ptr()).as_mut_ptr(),
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
    crate::gl!(sBattleBuffersTransferData[0] = CONTROLLER_MOVEANIMATION);
    crate::gl!(sBattleBuffersTransferData[1] = r#move as u8);
    crate::gl!(sBattleBuffersTransferData[2] = ((r#move as i32 & 0xFF00) >> 8) as u8);
    crate::gl!(sBattleBuffersTransferData[3] = turnOfMove);
    crate::gl!(sBattleBuffersTransferData[4] = movePower as u8);
    crate::gl!(sBattleBuffersTransferData[5] = ((movePower as i32 & 0xFF00) >> 8) as u8);
    crate::gl!(sBattleBuffersTransferData[6] = dmg as u8);
    crate::gl!(sBattleBuffersTransferData[7] = ((dmg & 0x0000FF00) >> 8) as u8);
    crate::gl!(sBattleBuffersTransferData[8] = ((dmg & 0x00FF0000) >> 16) as u8);
    crate::gl!(sBattleBuffersTransferData[9] = ((dmg as u32 & 0xFF000000) >> 24) as u8);
    crate::gl!(sBattleBuffersTransferData[10] = friendship);
    crate::gl!(sBattleBuffersTransferData[11] = multihit);
    if AbilityBattleEffects(14, 0, 13, 0, 0) == 0 && AbilityBattleEffects(14, 0, 77, 0, 0) == 0 {
        crate::gl!(sBattleBuffersTransferData[12] = gBattleWeather.get() as u8);
        crate::gl!(
            sBattleBuffersTransferData[13] = ((gBattleWeather.get() as i32 & 0xFF00) >> 8) as u8
        );
    } else {
        crate::gl!(sBattleBuffersTransferData[12] = 0);
        crate::gl!(sBattleBuffersTransferData[13] = 0);
    }
    crate::gl!(sBattleBuffersTransferData[14] = 0);
    crate::gl!(sBattleBuffersTransferData[15] = 0);
    memcpy(
        crate::gl!(&raw sBattleBuffersTransferData[16]),
        disableStructPtr as *mut u8,
        28,
    );
    PrepareBufferDataTransfer(
        bufferId,
        (*sBattleBuffersTransferData.as_ptr()).as_mut_ptr(),
        44,
    );
}
pub unsafe fn BtlController_EmitPrintString(bufferId: u8, stringId: u16) {
    crate::gl!(sBattleBuffersTransferData[0] = CONTROLLER_PRINTSTRING);
    crate::gl!(sBattleBuffersTransferData[1] = gBattleOutcome.get());
    crate::gl!(sBattleBuffersTransferData[2] = stringId as u8);
    crate::gl!(sBattleBuffersTransferData[3] = ((stringId as i32 & 0xFF00) >> 8) as u8);
    let stringInfo: *mut BattleMsgData =
        crate::gl!(&raw sBattleBuffersTransferData[4]) as *mut BattleMsgData;
    (*stringInfo).currentMove = gCurrentMove.get();
    (*stringInfo).originallyUsedMove = gChosenMove.get();
    (*stringInfo).lastItem = gLastUsedItem.get();
    (*stringInfo).lastAbility = gLastUsedAbility.get();
    (*stringInfo).scrActive = crate::gl!(gBattleScripting.battler);
    (*stringInfo).bakScriptPartyIdx = (*crate::gl!(gBattleStruct)).scriptPartyIdx;
    (*stringInfo).hpScale = (*crate::gl!(gBattleStruct)).hpScale;
    (*stringInfo).itemEffectBattler = gPotentialItemEffectBattler.get();
    (*stringInfo).moveType = (*(&raw const crate::data::pokemon::gBattleMoves)
        .cast::<CArray<BattleMove, 0>>())[gCurrentMove.get()]
    .r#type;
    for i in 0..(MAX_BATTLERS_COUNT as i32) {
        (*stringInfo).abilities[i] = crate::gl!(gBattleMons[i].ability);
    }
    let mut i: i32 = 0;
    while i
        < (if 16 >= (if 14 >= 11 { 14 } else { 11 }) {
            16
        } else {
            if 14 >= 11 { 14 } else { 11 }
        })
    {
        (*stringInfo).textBuffs[0][i] = crate::gl!(gBattleTextBuff1[i]);
        (*stringInfo).textBuffs[1][i] = crate::gl!(gBattleTextBuff2[i]);
        (*stringInfo).textBuffs[2][i] = crate::gl!(gBattleTextBuff3[i]);
        i += 1;
    }
    PrepareBufferDataTransfer(
        bufferId,
        (*sBattleBuffersTransferData.as_ptr()).as_mut_ptr(),
        68,
    );
}
pub unsafe fn BtlController_EmitPrintSelectionString(bufferId: u8, stringId: u16) {
    crate::gl!(sBattleBuffersTransferData[0] = CONTROLLER_PRINTSTRINGPLAYERONLY);
    crate::gl!(sBattleBuffersTransferData[1] = CONTROLLER_PRINTSTRINGPLAYERONLY);
    crate::gl!(sBattleBuffersTransferData[2] = stringId as u8);
    crate::gl!(sBattleBuffersTransferData[3] = ((stringId as i32 & 0xFF00) >> 8) as u8);
    let stringInfo: *mut BattleMsgData =
        crate::gl!(&raw sBattleBuffersTransferData[4]) as *mut BattleMsgData;
    (*stringInfo).currentMove = gCurrentMove.get();
    (*stringInfo).originallyUsedMove = gChosenMove.get();
    (*stringInfo).lastItem = gLastUsedItem.get();
    (*stringInfo).lastAbility = gLastUsedAbility.get();
    (*stringInfo).scrActive = crate::gl!(gBattleScripting.battler);
    (*stringInfo).bakScriptPartyIdx = (*crate::gl!(gBattleStruct)).scriptPartyIdx;
    for i in 0..(MAX_BATTLERS_COUNT as i32) {
        (*stringInfo).abilities[i] = crate::gl!(gBattleMons[i].ability);
    }
    let mut i: i32 = 0;
    while i
        < (if 16 >= (if 14 >= 11 { 14 } else { 11 }) {
            16
        } else {
            if 14 >= 11 { 14 } else { 11 }
        })
    {
        (*stringInfo).textBuffs[0][i] = crate::gl!(gBattleTextBuff1[i]);
        (*stringInfo).textBuffs[1][i] = crate::gl!(gBattleTextBuff2[i]);
        (*stringInfo).textBuffs[2][i] = crate::gl!(gBattleTextBuff3[i]);
        i += 1;
    }
    PrepareBufferDataTransfer(
        bufferId,
        (*sBattleBuffersTransferData.as_ptr()).as_mut_ptr(),
        68,
    );
}
pub unsafe fn BtlController_EmitChooseAction(bufferId: u8, action: u8, itemId: u16) {
    crate::gl!(sBattleBuffersTransferData[0] = CONTROLLER_CHOOSEACTION);
    crate::gl!(sBattleBuffersTransferData[1] = action);
    crate::gl!(sBattleBuffersTransferData[2] = itemId as u8);
    crate::gl!(sBattleBuffersTransferData[3] = ((itemId as i32 & 0xFF00) >> 8) as u8);
    PrepareBufferDataTransfer(
        bufferId,
        (*sBattleBuffersTransferData.as_ptr()).as_mut_ptr(),
        4,
    );
}
pub unsafe fn BtlController_EmitYesNoBox(bufferId: u8) {
    crate::gl!(sBattleBuffersTransferData[0] = CONTROLLER_YESNOBOX);
    crate::gl!(sBattleBuffersTransferData[1] = CONTROLLER_YESNOBOX);
    crate::gl!(sBattleBuffersTransferData[2] = CONTROLLER_YESNOBOX);
    crate::gl!(sBattleBuffersTransferData[3] = CONTROLLER_YESNOBOX);
    PrepareBufferDataTransfer(
        bufferId,
        (*sBattleBuffersTransferData.as_ptr()).as_mut_ptr(),
        4,
    );
}
pub unsafe fn BtlController_EmitChooseMove(
    bufferId: u8,
    isDoubleBattle: u8,
    noPPNumber: u8,
    movePPData: *mut ChooseMoveStruct,
) {
    crate::gl!(sBattleBuffersTransferData[0] = CONTROLLER_CHOOSEMOVE);
    crate::gl!(sBattleBuffersTransferData[1] = isDoubleBattle);
    crate::gl!(sBattleBuffersTransferData[2] = noPPNumber);
    crate::gl!(sBattleBuffersTransferData[3] = 0);
    for i in 0..20i32 {
        {
            let gl_v = *(movePPData as *mut u8).at(i);
            let gl_i16 = 4 + i;
            crate::gl!(sBattleBuffersTransferData[gl_i16] = gl_v)
        };
    }
    PrepareBufferDataTransfer(
        bufferId,
        (*sBattleBuffersTransferData.as_ptr()).as_mut_ptr(),
        24,
    );
}
pub unsafe fn BtlController_EmitChooseItem(bufferId: u8, battlePartyOrder: *mut u8) {
    crate::gl!(sBattleBuffersTransferData[0] = CONTROLLER_OPENBAG);
    for i in 0..3i32 {
        {
            let gl_v = *battlePartyOrder.at(i);
            let gl_i17 = 1 + i;
            crate::gl!(sBattleBuffersTransferData[gl_i17] = gl_v)
        };
    }
    PrepareBufferDataTransfer(
        bufferId,
        (*sBattleBuffersTransferData.as_ptr()).as_mut_ptr(),
        4,
    );
}
pub unsafe fn BtlController_EmitChoosePokemon(
    bufferId: u8,
    caseId: u8,
    slotId: u8,
    abilityId: u8,
    data: *mut u8,
) {
    crate::gl!(sBattleBuffersTransferData[0] = CONTROLLER_CHOOSEPOKEMON);
    crate::gl!(sBattleBuffersTransferData[1] = caseId);
    crate::gl!(sBattleBuffersTransferData[2] = slotId);
    crate::gl!(sBattleBuffersTransferData[3] = abilityId);
    for i in 0..3i32 {
        {
            let gl_v = *data.at(i);
            let gl_i18 = 4 + i;
            crate::gl!(sBattleBuffersTransferData[gl_i18] = gl_v)
        };
    }
    PrepareBufferDataTransfer(
        bufferId,
        (*sBattleBuffersTransferData.as_ptr()).as_mut_ptr(),
        8,
    );
}
unsafe fn BtlController_EmitCmd23(bufferId: u8) {
    crate::gl!(sBattleBuffersTransferData[0] = CONTROLLER_23);
    crate::gl!(sBattleBuffersTransferData[1] = CONTROLLER_23);
    crate::gl!(sBattleBuffersTransferData[2] = CONTROLLER_23);
    crate::gl!(sBattleBuffersTransferData[3] = CONTROLLER_23);
    PrepareBufferDataTransfer(
        bufferId,
        (*sBattleBuffersTransferData.as_ptr()).as_mut_ptr(),
        4,
    );
}
pub unsafe fn BtlController_EmitHealthBarUpdate(bufferId: u8, hpValue: u16) {
    crate::gl!(sBattleBuffersTransferData[0] = CONTROLLER_HEALTHBARUPDATE);
    crate::gl!(sBattleBuffersTransferData[1] = 0);
    crate::gl!(sBattleBuffersTransferData[2] = hpValue as i16 as u8);
    crate::gl!(sBattleBuffersTransferData[3] = ((hpValue as i16 as i32 & 0xFF00) >> 8) as u8);
    PrepareBufferDataTransfer(
        bufferId,
        (*sBattleBuffersTransferData.as_ptr()).as_mut_ptr(),
        4,
    );
}
pub unsafe fn BtlController_EmitExpUpdate(bufferId: u8, partyId: u8, expPoints: u16) {
    crate::gl!(sBattleBuffersTransferData[0] = CONTROLLER_EXPUPDATE);
    crate::gl!(sBattleBuffersTransferData[1] = partyId);
    crate::gl!(sBattleBuffersTransferData[2] = expPoints as i16 as u8);
    crate::gl!(sBattleBuffersTransferData[3] = ((expPoints as i16 as i32 & 0xFF00) >> 8) as u8);
    PrepareBufferDataTransfer(
        bufferId,
        (*sBattleBuffersTransferData.as_ptr()).as_mut_ptr(),
        4,
    );
}
pub unsafe fn BtlController_EmitStatusIconUpdate(bufferId: u8, status1: u32, status2: u32) {
    crate::gl!(sBattleBuffersTransferData[0] = CONTROLLER_STATUSICONUPDATE);
    crate::gl!(sBattleBuffersTransferData[1] = status1 as u8);
    crate::gl!(sBattleBuffersTransferData[2] = ((status1 & 0x0000FF00) >> 8) as u8);
    crate::gl!(sBattleBuffersTransferData[3] = ((status1 & 0x00FF0000) >> 16) as u8);
    crate::gl!(sBattleBuffersTransferData[4] = ((status1 & 0xFF000000) >> 24) as u8);
    crate::gl!(sBattleBuffersTransferData[5] = status2 as u8);
    crate::gl!(sBattleBuffersTransferData[6] = ((status2 & 0x0000FF00) >> 8) as u8);
    crate::gl!(sBattleBuffersTransferData[7] = ((status2 & 0x00FF0000) >> 16) as u8);
    crate::gl!(sBattleBuffersTransferData[8] = ((status2 & 0xFF000000) >> 24) as u8);
    PrepareBufferDataTransfer(
        bufferId,
        (*sBattleBuffersTransferData.as_ptr()).as_mut_ptr(),
        9,
    );
}
pub unsafe fn BtlController_EmitStatusAnimation(bufferId: u8, status2: u8, status: u32) {
    crate::gl!(sBattleBuffersTransferData[0] = CONTROLLER_STATUSANIMATION);
    crate::gl!(sBattleBuffersTransferData[1] = status2);
    crate::gl!(sBattleBuffersTransferData[2] = status as u8);
    crate::gl!(sBattleBuffersTransferData[3] = ((status & 0x0000FF00) >> 8) as u8);
    crate::gl!(sBattleBuffersTransferData[4] = ((status & 0x00FF0000) >> 16) as u8);
    crate::gl!(sBattleBuffersTransferData[5] = ((status & 0xFF000000) >> 24) as u8);
    PrepareBufferDataTransfer(
        bufferId,
        (*sBattleBuffersTransferData.as_ptr()).as_mut_ptr(),
        6,
    );
}
unsafe fn BtlController_EmitStatusXor(bufferId: u8, b: u8) {
    crate::gl!(sBattleBuffersTransferData[0] = CONTROLLER_STATUSXOR);
    crate::gl!(sBattleBuffersTransferData[1] = b);
    PrepareBufferDataTransfer(
        bufferId,
        (*sBattleBuffersTransferData.as_ptr()).as_mut_ptr(),
        2,
    );
}
pub unsafe fn BtlController_EmitDataTransfer(bufferId: u8, size: u16, mut data: *mut c_void) {
    crate::gl!(sBattleBuffersTransferData[0] = CONTROLLER_DATATRANSFER);
    crate::gl!(sBattleBuffersTransferData[1] = CONTROLLER_DATATRANSFER);
    crate::gl!(sBattleBuffersTransferData[2] = size as u8);
    crate::gl!(sBattleBuffersTransferData[3] = ((size as i32 & 0xFF00) >> 8) as u8);
    for i in 0..(size as i32) {
        {
            let gl_v = *(({
                let t2 = data;
                data = (data as *mut u8).at(1) as *mut c_void;
                t2
            }) as *mut u8);
            let gl_i19 = 4 + i;
            crate::gl!(sBattleBuffersTransferData[gl_i19] = gl_v)
        };
    }
    PrepareBufferDataTransfer(
        bufferId,
        (*sBattleBuffersTransferData.as_ptr()).as_mut_ptr(),
        size + 4,
    );
}
unsafe fn BtlController_EmitDMA3Transfer(
    bufferId: u8,
    dst: *mut c_void,
    size: u16,
    mut data: *mut c_void,
) {
    crate::gl!(sBattleBuffersTransferData[0] = CONTROLLER_DMA3TRANSFER);
    crate::gl!(sBattleBuffersTransferData[1] = dst as usize as u32 as u8);
    crate::gl!(sBattleBuffersTransferData[2] = ((dst as usize as u32 & 0x0000FF00) >> 8) as u8);
    crate::gl!(sBattleBuffersTransferData[3] = ((dst as usize as u32 & 0x00FF0000) >> 16) as u8);
    crate::gl!(sBattleBuffersTransferData[4] = ((dst as usize as u32 & 0xFF000000) >> 24) as u8);
    crate::gl!(sBattleBuffersTransferData[5] = size as u8);
    crate::gl!(sBattleBuffersTransferData[6] = ((size as i32 & 0xFF00) >> 8) as u8);
    for i in 0..(size as i32) {
        {
            let gl_v = *(({
                let t2 = data;
                data = (data as *mut u8).at(1) as *mut c_void;
                t2
            }) as *mut u8);
            let gl_i20 = 7 + i;
            crate::gl!(sBattleBuffersTransferData[gl_i20] = gl_v)
        };
    }
    PrepareBufferDataTransfer(
        bufferId,
        (*sBattleBuffersTransferData.as_ptr()).as_mut_ptr(),
        size + 7,
    );
}
unsafe fn BtlController_EmitPlayBGM(bufferId: u8, songId: u16, mut data: *mut c_void) {
    crate::gl!(sBattleBuffersTransferData[0] = CONTROLLER_PLAYBGM);
    crate::gl!(sBattleBuffersTransferData[1] = songId as u8);
    crate::gl!(sBattleBuffersTransferData[2] = ((songId as i32 & 0xFF00) >> 8) as u8);
    for i in 0..(songId as i32) {
        {
            let gl_v = *(({
                let t2 = data;
                data = (data as *mut u8).at(1) as *mut c_void;
                t2
            }) as *mut u8);
            let gl_i21 = 3 + i;
            crate::gl!(sBattleBuffersTransferData[gl_i21] = gl_v)
        };
    }
    PrepareBufferDataTransfer(
        bufferId,
        (*sBattleBuffersTransferData.as_ptr()).as_mut_ptr(),
        songId + 3,
    );
}
unsafe fn BtlController_EmitCmd32(bufferId: u8, size: u16, mut data: *mut c_void) {
    crate::gl!(sBattleBuffersTransferData[0] = CONTROLLER_32);
    crate::gl!(sBattleBuffersTransferData[1] = size as u8);
    crate::gl!(sBattleBuffersTransferData[2] = ((size as i32 & 0xFF00) >> 8) as u8);
    for i in 0..(size as i32) {
        {
            let gl_v = *(({
                let t2 = data;
                data = (data as *mut u8).at(1) as *mut c_void;
                t2
            }) as *mut u8);
            let gl_i22 = 3 + i;
            crate::gl!(sBattleBuffersTransferData[gl_i22] = gl_v)
        };
    }
    PrepareBufferDataTransfer(
        bufferId,
        (*sBattleBuffersTransferData.as_ptr()).as_mut_ptr(),
        size + 3,
    );
}
pub unsafe fn BtlController_EmitTwoReturnValues(bufferId: u8, ret8: u8, ret16: u16) {
    crate::gl!(sBattleBuffersTransferData[0] = CONTROLLER_TWORETURNVALUES);
    crate::gl!(sBattleBuffersTransferData[1] = ret8);
    crate::gl!(sBattleBuffersTransferData[2] = ret16 as u8);
    crate::gl!(sBattleBuffersTransferData[3] = ((ret16 as i32 & 0xFF00) >> 8) as u8);
    PrepareBufferDataTransfer(
        bufferId,
        (*sBattleBuffersTransferData.as_ptr()).as_mut_ptr(),
        4,
    );
}
pub unsafe fn BtlController_EmitChosenMonReturnValue(
    bufferId: u8,
    partyId: u8,
    battlePartyOrder: *mut u8,
) {
    crate::gl!(sBattleBuffersTransferData[0] = CONTROLLER_CHOSENMONRETURNVALUE);
    crate::gl!(sBattleBuffersTransferData[1] = partyId);
    for i in 0..3i32 {
        {
            let gl_v = *battlePartyOrder.at(i);
            let gl_i23 = 2 + i;
            crate::gl!(sBattleBuffersTransferData[gl_i23] = gl_v)
        };
    }
    PrepareBufferDataTransfer(
        bufferId,
        (*sBattleBuffersTransferData.as_ptr()).as_mut_ptr(),
        5,
    );
}
pub unsafe fn BtlController_EmitOneReturnValue(bufferId: u8, ret: u16) {
    crate::gl!(sBattleBuffersTransferData[0] = CONTROLLER_ONERETURNVALUE);
    crate::gl!(sBattleBuffersTransferData[1] = ret as u8);
    crate::gl!(sBattleBuffersTransferData[2] = ((ret as i32 & 0xFF00) >> 8) as u8);
    crate::gl!(sBattleBuffersTransferData[3] = 0);
    PrepareBufferDataTransfer(
        bufferId,
        (*sBattleBuffersTransferData.as_ptr()).as_mut_ptr(),
        4,
    );
}
pub unsafe fn BtlController_EmitOneReturnValue_Duplicate(bufferId: u8, ret: u16) {
    crate::gl!(sBattleBuffersTransferData[0] = CONTROLLER_ONERETURNVALUE_DUPLICATE);
    crate::gl!(sBattleBuffersTransferData[1] = ret as u8);
    crate::gl!(sBattleBuffersTransferData[2] = ((ret as i32 & 0xFF00) >> 8) as u8);
    crate::gl!(sBattleBuffersTransferData[3] = 0);
    PrepareBufferDataTransfer(
        bufferId,
        (*sBattleBuffersTransferData.as_ptr()).as_mut_ptr(),
        4,
    );
}
unsafe fn BtlController_EmitClearUnkVar(bufferId: u8) {
    crate::gl!(sBattleBuffersTransferData[0] = CONTROLLER_CLEARUNKVAR);
    crate::gl!(sBattleBuffersTransferData[1] = CONTROLLER_CLEARUNKVAR);
    crate::gl!(sBattleBuffersTransferData[2] = CONTROLLER_CLEARUNKVAR);
    crate::gl!(sBattleBuffersTransferData[3] = CONTROLLER_CLEARUNKVAR);
    PrepareBufferDataTransfer(
        bufferId,
        (*sBattleBuffersTransferData.as_ptr()).as_mut_ptr(),
        4,
    );
}
unsafe fn BtlController_EmitSetUnkVar(bufferId: u8, b: u8) {
    crate::gl!(sBattleBuffersTransferData[0] = CONTROLLER_SETUNKVAR);
    crate::gl!(sBattleBuffersTransferData[1] = b);
    PrepareBufferDataTransfer(
        bufferId,
        (*sBattleBuffersTransferData.as_ptr()).as_mut_ptr(),
        2,
    );
}
unsafe fn BtlController_EmitClearUnkFlag(bufferId: u8) {
    crate::gl!(sBattleBuffersTransferData[0] = CONTROLLER_CLEARUNKFLAG);
    crate::gl!(sBattleBuffersTransferData[1] = CONTROLLER_CLEARUNKFLAG);
    crate::gl!(sBattleBuffersTransferData[2] = CONTROLLER_CLEARUNKFLAG);
    crate::gl!(sBattleBuffersTransferData[3] = CONTROLLER_CLEARUNKFLAG);
    PrepareBufferDataTransfer(
        bufferId,
        (*sBattleBuffersTransferData.as_ptr()).as_mut_ptr(),
        4,
    );
}
unsafe fn BtlController_EmitToggleUnkFlag(bufferId: u8) {
    crate::gl!(sBattleBuffersTransferData[0] = CONTROLLER_TOGGLEUNKFLAG);
    crate::gl!(sBattleBuffersTransferData[1] = CONTROLLER_TOGGLEUNKFLAG);
    crate::gl!(sBattleBuffersTransferData[2] = CONTROLLER_TOGGLEUNKFLAG);
    crate::gl!(sBattleBuffersTransferData[3] = CONTROLLER_TOGGLEUNKFLAG);
    PrepareBufferDataTransfer(
        bufferId,
        (*sBattleBuffersTransferData.as_ptr()).as_mut_ptr(),
        4,
    );
}
pub unsafe fn BtlController_EmitHitAnimation(bufferId: u8) {
    crate::gl!(sBattleBuffersTransferData[0] = CONTROLLER_HITANIMATION);
    crate::gl!(sBattleBuffersTransferData[1] = CONTROLLER_HITANIMATION);
    crate::gl!(sBattleBuffersTransferData[2] = CONTROLLER_HITANIMATION);
    crate::gl!(sBattleBuffersTransferData[3] = CONTROLLER_HITANIMATION);
    PrepareBufferDataTransfer(
        bufferId,
        (*sBattleBuffersTransferData.as_ptr()).as_mut_ptr(),
        4,
    );
}
pub unsafe fn BtlController_EmitCantSwitch(bufferId: u8) {
    crate::gl!(sBattleBuffersTransferData[0] = CONTROLLER_CANTSWITCH);
    crate::gl!(sBattleBuffersTransferData[1] = CONTROLLER_CANTSWITCH);
    crate::gl!(sBattleBuffersTransferData[2] = CONTROLLER_CANTSWITCH);
    crate::gl!(sBattleBuffersTransferData[3] = CONTROLLER_CANTSWITCH);
    PrepareBufferDataTransfer(
        bufferId,
        (*sBattleBuffersTransferData.as_ptr()).as_mut_ptr(),
        4,
    );
}
pub unsafe fn BtlController_EmitPlaySE(bufferId: u8, songId: u16) {
    crate::gl!(sBattleBuffersTransferData[0] = CONTROLLER_PLAYSE);
    crate::gl!(sBattleBuffersTransferData[1] = songId as u8);
    crate::gl!(sBattleBuffersTransferData[2] = ((songId as i32 & 0xFF00) >> 8) as u8);
    crate::gl!(sBattleBuffersTransferData[3] = 0);
    PrepareBufferDataTransfer(
        bufferId,
        (*sBattleBuffersTransferData.as_ptr()).as_mut_ptr(),
        4,
    );
}
pub unsafe fn BtlController_EmitPlayFanfareOrBGM(bufferId: u8, songId: u16, playBGM: u8) {
    crate::gl!(sBattleBuffersTransferData[0] = CONTROLLER_PLAYFANFAREORBGM);
    crate::gl!(sBattleBuffersTransferData[1] = songId as u8);
    crate::gl!(sBattleBuffersTransferData[2] = ((songId as i32 & 0xFF00) >> 8) as u8);
    crate::gl!(sBattleBuffersTransferData[3] = playBGM);
    PrepareBufferDataTransfer(
        bufferId,
        (*sBattleBuffersTransferData.as_ptr()).as_mut_ptr(),
        4,
    );
}
pub unsafe fn BtlController_EmitFaintingCry(bufferId: u8) {
    crate::gl!(sBattleBuffersTransferData[0] = CONTROLLER_FAINTINGCRY);
    crate::gl!(sBattleBuffersTransferData[1] = CONTROLLER_FAINTINGCRY);
    crate::gl!(sBattleBuffersTransferData[2] = CONTROLLER_FAINTINGCRY);
    crate::gl!(sBattleBuffersTransferData[3] = CONTROLLER_FAINTINGCRY);
    PrepareBufferDataTransfer(
        bufferId,
        (*sBattleBuffersTransferData.as_ptr()).as_mut_ptr(),
        4,
    );
}
pub unsafe fn BtlController_EmitIntroSlide(bufferId: u8, environmentId: u8) {
    crate::gl!(sBattleBuffersTransferData[0] = CONTROLLER_INTROSLIDE);
    crate::gl!(sBattleBuffersTransferData[1] = environmentId);
    PrepareBufferDataTransfer(
        bufferId,
        (*sBattleBuffersTransferData.as_ptr()).as_mut_ptr(),
        2,
    );
}
pub unsafe fn BtlController_EmitIntroTrainerBallThrow(bufferId: u8) {
    crate::gl!(sBattleBuffersTransferData[0] = CONTROLLER_INTROTRAINERBALLTHROW);
    crate::gl!(sBattleBuffersTransferData[1] = CONTROLLER_INTROTRAINERBALLTHROW);
    crate::gl!(sBattleBuffersTransferData[2] = CONTROLLER_INTROTRAINERBALLTHROW);
    crate::gl!(sBattleBuffersTransferData[3] = CONTROLLER_INTROTRAINERBALLTHROW);
    PrepareBufferDataTransfer(
        bufferId,
        (*sBattleBuffersTransferData.as_ptr()).as_mut_ptr(),
        4,
    );
}
pub unsafe fn BtlController_EmitDrawPartyStatusSummary(
    bufferId: u8,
    hpAndStatus: *mut HpAndStatus,
    flags: u8,
) {
    crate::gl!(sBattleBuffersTransferData[0] = CONTROLLER_DRAWPARTYSTATUSSUMMARY);
    crate::gl!(sBattleBuffersTransferData[1] = flags & 127);
    crate::gl!(
        sBattleBuffersTransferData[2] =
            ((flags as i32 & PARTY_SUMM_SKIP_DRAW_DELAY as i32) >> 7) as u8
    );
    crate::gl!(sBattleBuffersTransferData[3] = CONTROLLER_DRAWPARTYSTATUSSUMMARY);
    for i in 0..48i32 {
        {
            let gl_v = *(hpAndStatus as *mut u8).at(i);
            let gl_i24 = 4 + i;
            crate::gl!(sBattleBuffersTransferData[gl_i24] = gl_v)
        };
    }
    PrepareBufferDataTransfer(
        bufferId,
        (*sBattleBuffersTransferData.as_ptr()).as_mut_ptr(),
        52,
    );
}
pub unsafe fn BtlController_EmitHidePartyStatusSummary(bufferId: u8) {
    crate::gl!(sBattleBuffersTransferData[0] = CONTROLLER_HIDEPARTYSTATUSSUMMARY);
    crate::gl!(sBattleBuffersTransferData[1] = CONTROLLER_HIDEPARTYSTATUSSUMMARY);
    crate::gl!(sBattleBuffersTransferData[2] = CONTROLLER_HIDEPARTYSTATUSSUMMARY);
    crate::gl!(sBattleBuffersTransferData[3] = CONTROLLER_HIDEPARTYSTATUSSUMMARY);
    PrepareBufferDataTransfer(
        bufferId,
        (*sBattleBuffersTransferData.as_ptr()).as_mut_ptr(),
        4,
    );
}
pub unsafe fn BtlController_EmitEndBounceEffect(bufferId: u8) {
    crate::gl!(sBattleBuffersTransferData[0] = CONTROLLER_ENDBOUNCE);
    crate::gl!(sBattleBuffersTransferData[1] = CONTROLLER_ENDBOUNCE);
    crate::gl!(sBattleBuffersTransferData[2] = CONTROLLER_ENDBOUNCE);
    crate::gl!(sBattleBuffersTransferData[3] = CONTROLLER_ENDBOUNCE);
    PrepareBufferDataTransfer(
        bufferId,
        (*sBattleBuffersTransferData.as_ptr()).as_mut_ptr(),
        4,
    );
}
pub unsafe fn BtlController_EmitSpriteInvisibility(bufferId: u8, isInvisible: u8) {
    crate::gl!(sBattleBuffersTransferData[0] = CONTROLLER_SPRITEINVISIBILITY);
    crate::gl!(sBattleBuffersTransferData[1] = isInvisible);
    crate::gl!(sBattleBuffersTransferData[2] = CONTROLLER_SPRITEINVISIBILITY);
    crate::gl!(sBattleBuffersTransferData[3] = CONTROLLER_SPRITEINVISIBILITY);
    PrepareBufferDataTransfer(
        bufferId,
        (*sBattleBuffersTransferData.as_ptr()).as_mut_ptr(),
        4,
    );
}
pub unsafe fn BtlController_EmitBattleAnimation(bufferId: u8, animationId: u8, argument: u16) {
    crate::gl!(sBattleBuffersTransferData[0] = CONTROLLER_BATTLEANIMATION);
    crate::gl!(sBattleBuffersTransferData[1] = animationId);
    crate::gl!(sBattleBuffersTransferData[2] = argument as u8);
    crate::gl!(sBattleBuffersTransferData[3] = ((argument as i32 & 0xFF00) >> 8) as u8);
    PrepareBufferDataTransfer(
        bufferId,
        (*sBattleBuffersTransferData.as_ptr()).as_mut_ptr(),
        4,
    );
}
pub unsafe fn BtlController_EmitLinkStandbyMsg(bufferId: u8, mode: u8, record: u32) {
    let record_: u8 = record as u8;
    crate::gl!(sBattleBuffersTransferData[0] = CONTROLLER_LINKSTANDBYMSG);
    crate::gl!(sBattleBuffersTransferData[1] = mode);
    if record_ != 0 {
        crate::gl!(
            sBattleBuffersTransferData[3] = {
                crate::gl!(
                    sBattleBuffersTransferData[2] = RecordedBattle_BufferNewBattlerData(
                        crate::gl!(&raw sBattleBuffersTransferData[4])
                    )
                );
                crate::gl!(sBattleBuffersTransferData[2])
            }
        );
    } else {
        crate::gl!(
            sBattleBuffersTransferData[3] = {
                crate::gl!(sBattleBuffersTransferData[2] = 0);
                crate::gl!(sBattleBuffersTransferData[2])
            }
        );
    }
    PrepareBufferDataTransfer(
        bufferId,
        (*sBattleBuffersTransferData.as_ptr()).as_mut_ptr(),
        crate::gl!(sBattleBuffersTransferData[2]) as u16 + 4,
    );
}
pub unsafe fn BtlController_EmitResetActionMoveSelection(bufferId: u8, caseId: u8) {
    crate::gl!(sBattleBuffersTransferData[0] = CONTROLLER_RESETACTIONMOVESELECTION);
    crate::gl!(sBattleBuffersTransferData[1] = caseId);
    PrepareBufferDataTransfer(
        bufferId,
        (*sBattleBuffersTransferData.as_ptr()).as_mut_ptr(),
        2,
    );
}
pub unsafe fn BtlController_EmitEndLinkBattle(bufferId: u8, battleOutcome: u8) {
    crate::gl!(sBattleBuffersTransferData[0] = CONTROLLER_ENDLINKBATTLE);
    crate::gl!(sBattleBuffersTransferData[1] = battleOutcome);
    crate::gl!(
        sBattleBuffersTransferData[2] =
            (*crate::gl!(gSaveBlock2Ptr)).frontier.disableRecordBattle()
    );
    crate::gl!(
        sBattleBuffersTransferData[3] =
            (*crate::gl!(gSaveBlock2Ptr)).frontier.disableRecordBattle()
    );
    crate::gl!(
        sBattleBuffersTransferData[5] = {
            crate::gl!(
                sBattleBuffersTransferData[4] = RecordedBattle_BufferNewBattlerData(
                    crate::gl!(&raw sBattleBuffersTransferData[6])
                )
            );
            crate::gl!(sBattleBuffersTransferData[4])
        }
    );
    PrepareBufferDataTransfer(
        bufferId,
        (*sBattleBuffersTransferData.as_ptr()).as_mut_ptr(),
        crate::gl!(sBattleBuffersTransferData[4]) as u16 + 6,
    );
}
