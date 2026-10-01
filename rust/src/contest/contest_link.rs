//! Translated from `src/contest_link.c` by tools/rustport/c2rs.py.
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
    unused_assignments
)]

#[allow(unused_imports)]
use crate::c::*;
#[allow(unused_imports)]
use crate::consts::*;
use crate::contest::{
    StripPlayerAndMonNamesForLinkContest, gContestLinkLeaderIndex, gContestMons,
    gContestPlayerMonIndex, gContestResources, gContestRngValue, gLinkContestFlags,
    gNumLinkContestPlayers,
};
use crate::contest::{
    gContestFinalStandings, gContestMonAppealPointTotals, gContestMonRound1Points,
    gContestMonRound2Points, gContestMonTotalPoints, gContestantTurnOrder,
};
use crate::link::{
    BitmaskAllOtherLinkPlayers, GetBlockReceivedStatus, GetLinkPlayerCount,
    GetLinkPlayerCountAsBitFlags, GetMultiplayerId, IsLinkTaskFinished, ResetBlockReceivedFlag,
    ResetBlockReceivedFlags, SendBlock, SendBlockRequest, SetLinkStandbyCallback, gLinkPlayers,
    gReceivedRemoteLinkPlayers, gWirelessCommType,
};
use crate::link::{gBlockRecvBuffer, gBlockSendBuffer};
use crate::task::SwitchTaskToFollowupFunc;
use crate::task::{task_data_ptr, task_get, task_set, task_set_func};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use core::ffi::c_void;
#[allow(unused_imports)]
use core::mem::zeroed;
#[allow(unused_imports)]
use core::ptr::null_mut;
// The C's names for task and sprite data slots.
const tState: usize = 0;
const tDelayTimer: usize = 1;
const tCategory: usize = 9;
const tTimer: usize = 11;
const tStandbyState: usize = 12;

pub unsafe fn LinkContest_SendBlock(src: *mut c_void, size: u16) -> u32 {
    memcpy(
        (*(&raw const crate::decompress::gDecompressionBuffer)
            .cast::<CArray<u8, 16384>>()
            .cast_mut())
        .as_mut_ptr(),
        src as *mut u8,
        size as u32,
    );
    if SendBlock(
        BitmaskAllOtherLinkPlayers(),
        (*(&raw const crate::decompress::gDecompressionBuffer)
            .cast::<CArray<u8, 16384>>()
            .cast_mut())
        .as_mut_ptr() as *mut c_void,
        size,
    ) != 0
    {
        return TRUE as u32;
    } else {
        return FALSE as u32;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
pub unsafe fn LinkContest_GetBlockReceived(flag: u8) -> u8 {
    let mask: u8 = shl_i32(1, flag as u32) as u8;
    if GetBlockReceivedStatus() as i32 & mask as i32 == 0 {
        return FALSE;
    } else {
        ResetBlockReceivedFlag(flag);
        return TRUE;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
pub unsafe fn LinkContest_GetBlockReceivedFromAllPlayers() -> u8 {
    if GetBlockReceivedStatus() == GetLinkPlayerCountAsBitFlags() {
        ResetBlockReceivedFlags();
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
pub unsafe fn Task_LinkContest_Init(taskId: u8) {
    for i in 0..(CONTESTANT_COUNT as u8) {
        gBlockRecvBuffer[i][0] = 0xFF;
    }
    task_set(taskId, tState, 0);
    task_set_func(taskId, Some(Task_LinkContest_StartInitFlags));
}
pub(crate) fn Task_LinkContest_StartInitFlags(taskId: u8) {
    task_set_func(taskId, Some(Task_LinkContest_InitFlags));
}
pub(crate) unsafe fn Task_LinkContest_InitFlags(taskId: u8) {
    if gReceivedRemoteLinkPlayers == 0 {
        return;
    }
    gContestPlayerMonIndex = GetMultiplayerId();
    gNumLinkContestPlayers = GetLinkPlayerCount();
    gLinkContestFlags = LINK_CONTEST_FLAG_IS_LINK as u8;
    if gWirelessCommType == 1 {
        gLinkContestFlags = 3;
    }
    let mut i: i32 = 0;
    while i < gNumLinkContestPlayers as i32 && (gLinkPlayers[i].version as u32 & 0xFF) - 1 > 1 {
        i += 1;
    }
    if i < gNumLinkContestPlayers as i32 {
        gLinkContestFlags |= LINK_CONTEST_FLAG_HAS_RS_PLAYER as u8;
    }
    SwitchTaskToFollowupFunc(taskId);
}
pub unsafe fn LinkContest_TryLinkStandby(state: *mut i16) -> u32 {
    if gLinkContestFlags as i32 & LINK_CONTEST_FLAG_HAS_RS_PLAYER != 0 {
        return TRUE as u32;
    }
    match *state {
        0 => {
            if IsLinkTaskFinished() != 0 {
                SetLinkStandbyCallback();
                *state += 1;
            }
            return FALSE as u32;
        }
        1 => {
            *state += 1;
            return FALSE as u32;
        }
        _ => {
            if IsLinkTaskFinished() != TRUE {
                return FALSE as u32;
            } else {
                return TRUE as u32;
            }
        }
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
pub unsafe fn Task_LinkContest_CommunicateMonsRS(taskId: u8) {
    let mut i: i32 = 0;
    if LinkContest_TryLinkStandby(task_data_ptr(taskId, tStandbyState)) == 0 {
        return;
    }
    match task_get(taskId, tState) {
        0 => {
            if GetMultiplayerId() == 0 {
                if IsLinkTaskFinished() != 0 {
                    memcpy(
                        gBlockSendBuffer.as_mut_ptr(),
                        &raw mut gContestMons[gContestPlayerMonIndex] as *mut u8,
                        64,
                    );
                    task_set(taskId, tState, 10);
                }
            } else {
                memcpy(
                    gBlockSendBuffer.as_mut_ptr(),
                    &raw mut gContestMons[gContestPlayerMonIndex] as *mut u8,
                    64,
                );
                task_set(taskId, tState, 1);
            }
        }
        1 => {
            if LinkContest_GetBlockReceivedFromAllPlayers() != 0 {
                i = 0;
                while i < gNumLinkContestPlayers as i32 {
                    memcpy(
                        &raw mut gContestMons[i] as *mut u8,
                        gBlockRecvBuffer[i].as_mut_ptr() as *mut u8,
                        64,
                    );
                    StripPlayerAndMonNamesForLinkContest(
                        &raw mut gContestMons[i],
                        gLinkPlayers[i].language as i32,
                    );
                    i += 1;
                }
                task_set(taskId, tState, task_get(taskId, tState) + 1);
            }
        }
        10 => {
            if ({
                task_set(taskId, tTimer, task_get(taskId, tTimer) + 1);
                task_get(taskId, tTimer)
            }) > 300
            {
                SendBlockRequest(BLOCK_REQ_SIZE_100);
                task_set(taskId, tState, 1);
            }
        }
        _ => {
            task_set(taskId, tState, 0);
            task_set(taskId, tTimer, 0);
            task_set(taskId, tStandbyState, 0);
            SwitchTaskToFollowupFunc(taskId);
        }
    }
}
pub unsafe fn Task_LinkContest_CommunicateRngRS(taskId: u8) {
    match task_get(taskId, tState) {
        0 => {
            if GetMultiplayerId() == 0 {
                if IsLinkTaskFinished() != 0
                    && LinkContest_SendBlock(
                        &raw mut (*crate::random::gRngValue.as_ptr().cast::<u32>()) as *mut c_void,
                        4,
                    ) == TRUE as u32
                {
                    task_set(taskId, tState, task_get(taskId, tState) + 1);
                }
            } else {
                task_set(taskId, tState, task_get(taskId, tState) + 1);
            }
        }
        1 => {
            if LinkContest_GetBlockReceived(0) != 0 {
                memcpy(
                    &raw mut (*crate::random::gRngValue.as_ptr().cast::<u32>()) as *mut u8,
                    gBlockRecvBuffer[0].as_mut_ptr() as *mut u8,
                    4,
                );
                memcpy(
                    &raw mut gContestRngValue as *mut u8,
                    gBlockRecvBuffer[0].as_mut_ptr() as *mut u8,
                    4,
                );
                task_set(taskId, tState, task_get(taskId, tState) + 1);
            }
        }
        _ => {
            task_set(taskId, tState, 0);
            SwitchTaskToFollowupFunc(taskId);
        }
    }
}
pub unsafe fn Task_LinkContest_CommunicateCategoryRS(taskId: u8) {
    if LinkContest_TryLinkStandby(task_data_ptr(taskId, tStandbyState)) == 0 {
        return;
    }
    match task_get(taskId, tState) {
        0 => {
            gBlockSendBuffer[0] = task_get(taskId, tCategory) as u8;
            if GetMultiplayerId() == 0 {
                if IsLinkTaskFinished() != 0 {
                    task_set(taskId, tState, 10);
                }
            } else {
                task_set(taskId, tState, task_get(taskId, tState) + 1);
            }
        }
        1 => {
            if LinkContest_GetBlockReceivedFromAllPlayers() != 0 {
                for i in 0..(gNumLinkContestPlayers as i32) {
                    task_set(taskId, i + 1, gBlockRecvBuffer[i][0] as i16);
                }
                task_set(taskId, tState, task_get(taskId, tState) + 1);
            }
        }
        10 => {
            if ({
                task_set(taskId, tTimer, task_get(taskId, tTimer) + 1);
                task_get(taskId, tTimer)
            }) > 10
            {
                SendBlockRequest(BLOCK_REQ_SIZE_100);
                task_set(taskId, tState, 1);
            }
        }
        _ => {
            task_set(taskId, tState, 0);
            task_set(taskId, tTimer, 0);
            task_set(taskId, tStandbyState, 0);
            SwitchTaskToFollowupFunc(taskId);
        }
    }
}
pub unsafe fn Task_LinkContest_CommunicateMonIdxs(taskId: u8) {
    match task_get(taskId, tState) {
        0 => {
            if IsLinkTaskFinished() != 0
                && LinkContest_SendBlock(&raw mut gContestPlayerMonIndex as *mut c_void, 1)
                    == TRUE as u32
            {
                task_set(taskId, tState, task_get(taskId, tState) + 1);
            }
        }
        1 => {
            if LinkContest_GetBlockReceivedFromAllPlayers() != 0 {
                task_set(taskId, tState, task_get(taskId, tState) + 1);
            }
        }
        _ => {
            task_set(taskId, tState, 0);
            SwitchTaskToFollowupFunc(taskId);
        }
    }
}
pub unsafe fn Task_LinkContest_CommunicateMoveSelections(taskId: u8) {
    match task_get(taskId, tState) {
        0 => {
            if IsLinkTaskFinished() != 0
                && LinkContest_SendBlock(
                    &raw mut (*(*gContestResources).status.at(gContestPlayerMonIndex)).currMove
                        as *mut c_void,
                    2,
                ) == TRUE as u32
            {
                task_set(taskId, tState, task_get(taskId, tState) + 1);
            }
        }
        1 => {
            if LinkContest_GetBlockReceivedFromAllPlayers() != 0 {
                for i in 0..(gNumLinkContestPlayers as i32) {
                    (*(*gContestResources).status.at(i)).currMove = gBlockRecvBuffer[i][0];
                }
                task_set(taskId, tState, task_get(taskId, tState) + 1);
            }
        }
        _ => {
            task_set(taskId, tState, 0);
            SwitchTaskToFollowupFunc(taskId);
        }
    }
}
pub unsafe fn Task_LinkContest_CommunicateFinalStandings(taskId: u8) {
    match task_get(taskId, tState) {
        0 => {
            if IsLinkTaskFinished() != 0
                && LinkContest_SendBlock(gContestMonTotalPoints.as_mut_ptr() as *mut c_void, 8) == 1
            {
                task_set(taskId, tState, task_get(taskId, tState) + 1);
            }
        }
        1 => {
            if LinkContest_GetBlockReceivedFromAllPlayers() != 0 {
                memcpy(
                    gContestMonTotalPoints.as_mut_ptr() as *mut u8,
                    gBlockRecvBuffer[gContestLinkLeaderIndex].as_mut_ptr() as *mut u8,
                    8,
                );
                task_set(taskId, tState, task_get(taskId, tState) + 1);
            }
        }
        2 | 5 | 8 | 11 => {
            if ({
                let t1 = task_get(taskId, tDelayTimer);
                task_set(taskId, tDelayTimer, task_get(taskId, tDelayTimer) + 1);
                t1
            }) > 10
            {
                task_set(taskId, tDelayTimer, 0);
                task_set(taskId, tState, task_get(taskId, tState) + 1);
            }
        }
        3 => {
            if IsLinkTaskFinished() != 0
                && LinkContest_SendBlock(
                    gContestMonAppealPointTotals.as_mut_ptr() as *mut c_void,
                    8,
                ) == 1
            {
                task_set(taskId, tState, task_get(taskId, tState) + 1);
            }
        }
        4 => {
            if LinkContest_GetBlockReceivedFromAllPlayers() != 0 {
                memcpy(
                    gContestMonAppealPointTotals.as_mut_ptr() as *mut u8,
                    gBlockRecvBuffer[gContestLinkLeaderIndex].as_mut_ptr() as *mut u8,
                    8,
                );
                task_set(taskId, tState, task_get(taskId, tState) + 1);
            }
        }
        6 => {
            if IsLinkTaskFinished() != 0
                && LinkContest_SendBlock(gContestMonRound2Points.as_mut_ptr() as *mut c_void, 8)
                    == 1
            {
                task_set(taskId, tState, task_get(taskId, tState) + 1);
            }
        }
        7 => {
            if LinkContest_GetBlockReceivedFromAllPlayers() != 0 {
                memcpy(
                    gContestMonRound2Points.as_mut_ptr() as *mut u8,
                    gBlockRecvBuffer[gContestLinkLeaderIndex].as_mut_ptr() as *mut u8,
                    8,
                );
                task_set(taskId, tState, task_get(taskId, tState) + 1);
            }
        }
        9 => {
            if IsLinkTaskFinished() != 0
                && LinkContest_SendBlock(gContestFinalStandings.as_mut_ptr() as *mut c_void, 4) == 1
            {
                task_set(taskId, tState, task_get(taskId, tState) + 1);
            }
        }
        10 => {
            if LinkContest_GetBlockReceivedFromAllPlayers() != 0 {
                memcpy(
                    gContestFinalStandings.as_mut_ptr(),
                    gBlockRecvBuffer[gContestLinkLeaderIndex].as_mut_ptr() as *mut u8,
                    4,
                );
                task_set(taskId, tState, task_get(taskId, tState) + 1);
            }
        }
        _ => {
            task_set(taskId, tState, 0);
            SwitchTaskToFollowupFunc(taskId);
        }
    }
}
pub unsafe fn Task_LinkContest_CommunicateAppealsState(taskId: u8) {
    match task_get(taskId, tState) {
        0 => {
            if IsLinkTaskFinished() != 0
                && LinkContest_SendBlock((*gContestResources).status as *mut c_void, 112) == 1
            {
                task_set(taskId, tState, task_get(taskId, tState) + 1);
            }
        }
        1 => {
            if LinkContest_GetBlockReceivedFromAllPlayers() != 0 {
                memcpy(
                    (*gContestResources).status as *mut u8,
                    gBlockRecvBuffer[gContestLinkLeaderIndex].as_mut_ptr() as *mut u8,
                    112,
                );
                task_set(taskId, tState, task_get(taskId, tState) + 1);
            }
        }
        2 | 5 | 8 | 11 => {
            if ({
                let t1 = task_get(taskId, tDelayTimer);
                task_set(taskId, tDelayTimer, task_get(taskId, tDelayTimer) + 1);
                t1
            }) > 10
            {
                task_set(taskId, tDelayTimer, 0);
                task_set(taskId, tState, task_get(taskId, tState) + 1);
            }
        }
        3 => {
            if IsLinkTaskFinished() != 0
                && LinkContest_SendBlock((*gContestResources).appealResults as *mut c_void, 20) == 1
            {
                task_set(taskId, tState, task_get(taskId, tState) + 1);
            }
        }
        4 => {
            if LinkContest_GetBlockReceivedFromAllPlayers() != 0 {
                memcpy(
                    (*gContestResources).appealResults as *mut u8,
                    gBlockRecvBuffer[gContestLinkLeaderIndex].as_mut_ptr() as *mut u8,
                    20,
                );
                task_set(taskId, tState, task_get(taskId, tState) + 1);
            }
        }
        6 => {
            if IsLinkTaskFinished() != 0
                && LinkContest_SendBlock((*gContestResources).excitement as *mut c_void, 4) == 1
            {
                task_set(taskId, tState, task_get(taskId, tState) + 1);
            }
        }
        7 => {
            if LinkContest_GetBlockReceivedFromAllPlayers() != 0 {
                memcpy(
                    (*gContestResources).excitement as *mut u8,
                    gBlockRecvBuffer[gContestLinkLeaderIndex].as_mut_ptr() as *mut u8,
                    4,
                );
                task_set(taskId, tState, task_get(taskId, tState) + 1);
            }
        }
        9 => {
            if IsLinkTaskFinished() != 0
                && LinkContest_SendBlock(gContestantTurnOrder.as_mut_ptr() as *mut c_void, 4) == 1
            {
                task_set(taskId, tState, task_get(taskId, tState) + 1);
            }
        }
        10 => {
            if LinkContest_GetBlockReceivedFromAllPlayers() != 0 {
                memcpy(
                    gContestantTurnOrder.as_mut_ptr(),
                    gBlockRecvBuffer[gContestLinkLeaderIndex].as_mut_ptr() as *mut u8,
                    4,
                );
                task_set(taskId, tState, task_get(taskId, tState) + 1);
            }
        }
        _ => {
            task_set(taskId, tState, 0);
            SwitchTaskToFollowupFunc(taskId);
        }
    }
}
pub unsafe fn Task_LinkContest_CommunicateLeaderIdsRS(taskId: u8) {
    if LinkContest_TryLinkStandby(task_data_ptr(taskId, tStandbyState)) == 0 {
        return;
    }
    match task_get(taskId, tState) {
        0 => {
            gBlockSendBuffer[0] = 0x6E;
            if GetMultiplayerId() == 0 {
                if IsLinkTaskFinished() != 0 {
                    task_set(taskId, tState, 10);
                }
            } else {
                task_set(taskId, tState, task_get(taskId, tState) + 1);
            }
        }
        1 => {
            if LinkContest_GetBlockReceivedFromAllPlayers() != 0 {
                for i in 0..CONTESTANT_COUNT {
                    task_set(taskId, i + 5, gBlockRecvBuffer[i][0] as i16);
                }
                task_set(taskId, tState, task_get(taskId, tState) + 1);
            }
        }
        10 => {
            if ({
                task_set(taskId, tTimer, task_get(taskId, tTimer) + 1);
                task_get(taskId, tTimer)
            }) > 10
            {
                SendBlockRequest(BLOCK_REQ_SIZE_100);
                task_set(taskId, tState, 1);
            }
        }
        _ => {
            task_set(taskId, tState, 0);
            task_set(taskId, tTimer, 0);
            task_set(taskId, tStandbyState, 0);
            SwitchTaskToFollowupFunc(taskId);
        }
    }
}
pub unsafe fn Task_LinkContest_CommunicateRound1Points(taskId: u8) {
    if LinkContest_TryLinkStandby(task_data_ptr(taskId, tStandbyState)) == 0 {
        return;
    }
    match task_get(taskId, tState) {
        0 => {
            if IsLinkTaskFinished() != 0
                && LinkContest_SendBlock(gContestMonRound1Points.as_mut_ptr() as *mut c_void, 8)
                    == 1
            {
                task_set(taskId, tState, task_get(taskId, tState) + 1);
            }
        }
        1 => {
            if LinkContest_GetBlockReceivedFromAllPlayers() != 0 {
                memcpy(
                    gContestMonRound1Points.as_mut_ptr() as *mut u8,
                    gBlockRecvBuffer[gContestLinkLeaderIndex].as_mut_ptr() as *mut u8,
                    8,
                );
                task_set(taskId, tState, task_get(taskId, tState) + 1);
            }
        }
        _ => {
            task_set(taskId, tState, 0);
            task_set(taskId, tStandbyState, 0);
            SwitchTaskToFollowupFunc(taskId);
        }
    }
}
pub unsafe fn Task_LinkContest_CommunicateTurnOrder(taskId: u8) {
    if LinkContest_TryLinkStandby(task_data_ptr(taskId, tStandbyState)) == 0 {
        return;
    }
    match task_get(taskId, tState) {
        0 => {
            if IsLinkTaskFinished() != 0
                && LinkContest_SendBlock(gContestantTurnOrder.as_mut_ptr() as *mut c_void, 4) == 1
            {
                task_set(taskId, tState, task_get(taskId, tState) + 1);
            }
        }
        1 => {
            if LinkContest_GetBlockReceivedFromAllPlayers() != 0 {
                memcpy(
                    gContestantTurnOrder.as_mut_ptr(),
                    gBlockRecvBuffer[gContestLinkLeaderIndex].as_mut_ptr() as *mut u8,
                    4,
                );
                task_set(taskId, tState, task_get(taskId, tState) + 1);
            }
        }
        _ => {
            task_set(taskId, tState, 0);
            task_set(taskId, tStandbyState, 0);
            SwitchTaskToFollowupFunc(taskId);
        }
    }
}
