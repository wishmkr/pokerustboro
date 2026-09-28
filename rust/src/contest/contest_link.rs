//! Translated from `src/contest_link.c` by tools/rustport/c2rs.py.
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

unsafe extern "C" {
    static mut gBlockRecvBuffer: CArray<CArray<u16, 128>, 5>;
    static mut gBlockSendBuffer: CArray<u8, 256>;
    static mut gContestFinalStandings: CArray<u8, 4>;
    static mut gContestLinkLeaderIndex: u8;
    static mut gContestMonAppealPointTotals: CArray<i16, 4>;
    static mut gContestMonRound1Points: CArray<i16, 4>;
    static mut gContestMonRound2Points: CArray<i16, 4>;
    static mut gContestMonTotalPoints: CArray<i16, 4>;
    static mut gContestMons: CArray<ContestPokemon, 4>;
    static mut gContestPlayerMonIndex: u8;
    static mut gContestResources: *mut ContestResources;
    static mut gContestRngValue: u32;
    static mut gContestantTurnOrder: CArray<u8, 4>;
    static mut gDecompressionBuffer: CArray<u8, 16384>;
    static mut gLinkContestFlags: u8;
    static mut gLinkPlayers: CArray<LinkPlayer, 5>;
    static mut gNumLinkContestPlayers: u8;
    static mut gReceivedRemoteLinkPlayers: u8;
    static mut gRngValue: u32;
    static mut gTasks: CArray<Task, 0>;
    static mut gWirelessCommType: u8;
    fn BitmaskAllOtherLinkPlayers() -> u8;
    fn GetBlockReceivedStatus() -> u8;
    fn GetLinkPlayerCount() -> u8;
    fn GetLinkPlayerCountAsBitFlags() -> u8;
    fn GetMultiplayerId() -> u8;
    fn IsLinkTaskFinished() -> u8;
    fn ResetBlockReceivedFlag(a0: u8);
    fn ResetBlockReceivedFlags();
    fn SendBlock(a0: u8, a1: *mut c_void, a2: u16) -> u8;
    fn SendBlockRequest(a0: u8) -> u8;
    fn SetLinkStandbyCallback();
    fn StripPlayerAndMonNamesForLinkContest(a0: *mut ContestPokemon, a1: i32);
    fn SwitchTaskToFollowupFunc(a0: u8);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn LinkContest_SendBlock(src: *mut c_void, size: u16) -> u32 {
    memcpy(
        gDecompressionBuffer.as_mut_ptr(),
        src as *mut u8,
        size as u32,
    );
    if SendBlock(
        BitmaskAllOtherLinkPlayers(),
        gDecompressionBuffer.as_mut_ptr() as *mut c_void,
        size,
    ) != 0
    {
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
pub unsafe extern "C" fn LinkContest_GetBlockReceived(flag: u8) -> u8 {
    let mut mask: u8 = shl_i32(1, flag as u32) as u8;
    if GetBlockReceivedStatus() as i32 & mask as i32 == 0 {
        return FALSE;
    } else {
        ResetBlockReceivedFlag(flag);
        return TRUE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn LinkContest_GetBlockReceivedFromAllPlayers() -> u8 {
    if GetBlockReceivedStatus() == GetLinkPlayerCountAsBitFlags() {
        ResetBlockReceivedFlags();
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Task_LinkContest_Init(taskId: u8) {
    let mut i: u8 = 0;
    i = 0;
    while i < CONTESTANT_COUNT as u8 {
        gBlockRecvBuffer[i][0] = 0xFF;
        i += 1;
    }
    gTasks[taskId].data[0] = 0;
    gTasks[taskId].func = Some(Task_LinkContest_StartInitFlags);
}
pub(crate) unsafe extern "C" fn Task_LinkContest_StartInitFlags(taskId: u8) {
    gTasks[taskId].func = Some(Task_LinkContest_InitFlags);
}
pub(crate) unsafe extern "C" fn Task_LinkContest_InitFlags(taskId: u8) {
    let mut i: i32 = 0;
    if gReceivedRemoteLinkPlayers == 0 {
        return;
    }
    gContestPlayerMonIndex = GetMultiplayerId();
    gNumLinkContestPlayers = GetLinkPlayerCount();
    gLinkContestFlags = LINK_CONTEST_FLAG_IS_LINK as u8;
    if gWirelessCommType == 1 {
        gLinkContestFlags = 3;
    }
    i = 0;
    while i < gNumLinkContestPlayers as i32 && (gLinkPlayers[i].version as u32 & 0xFF) - 1 > 1 {
        i += 1;
    }
    if i < gNumLinkContestPlayers as i32 {
        gLinkContestFlags |= LINK_CONTEST_FLAG_HAS_RS_PLAYER as u8;
    }
    SwitchTaskToFollowupFunc(taskId);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn LinkContest_TryLinkStandby(state: *mut i16) -> u32 {
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
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Task_LinkContest_CommunicateMonsRS(taskId: u8) {
    let mut i: i32 = 0;
    if LinkContest_TryLinkStandby(&raw mut gTasks[taskId].data[12]) == 0 {
        return;
    }
    match gTasks[taskId].data[0] {
        0 => {
            if GetMultiplayerId() == 0 {
                if IsLinkTaskFinished() != 0 {
                    memcpy(
                        gBlockSendBuffer.as_mut_ptr(),
                        &raw mut gContestMons[gContestPlayerMonIndex] as *mut u8,
                        64,
                    );
                    gTasks[taskId].data[0] = 10;
                }
            } else {
                memcpy(
                    gBlockSendBuffer.as_mut_ptr(),
                    &raw mut gContestMons[gContestPlayerMonIndex] as *mut u8,
                    64,
                );
                gTasks[taskId].data[0] = 1;
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
                gTasks[taskId].data[0] += 1;
            }
        }
        10 => {
            if ({
                gTasks[taskId].data[11] += 1;
                gTasks[taskId].data[11]
            }) > 300
            {
                SendBlockRequest(BLOCK_REQ_SIZE_100);
                gTasks[taskId].data[0] = 1;
            }
        }
        _ => {
            gTasks[taskId].data[0] = 0;
            gTasks[taskId].data[11] = 0;
            gTasks[taskId].data[12] = 0;
            SwitchTaskToFollowupFunc(taskId);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Task_LinkContest_CommunicateRngRS(taskId: u8) {
    match gTasks[taskId].data[0] {
        0 => {
            if GetMultiplayerId() == 0 {
                if IsLinkTaskFinished() != 0
                    && LinkContest_SendBlock(&raw mut gRngValue as *mut c_void, 4) == TRUE as u32
                {
                    gTasks[taskId].data[0] += 1;
                }
            } else {
                gTasks[taskId].data[0] += 1;
            }
        }
        1 => {
            if LinkContest_GetBlockReceived(0) != 0 {
                memcpy(
                    &raw mut gRngValue as *mut u8,
                    gBlockRecvBuffer[0].as_mut_ptr() as *mut u8,
                    4,
                );
                memcpy(
                    &raw mut gContestRngValue as *mut u8,
                    gBlockRecvBuffer[0].as_mut_ptr() as *mut u8,
                    4,
                );
                gTasks[taskId].data[0] += 1;
            }
        }
        _ => {
            gTasks[taskId].data[0] = 0;
            SwitchTaskToFollowupFunc(taskId);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Task_LinkContest_CommunicateCategoryRS(taskId: u8) {
    let mut i: i32 = 0;
    if LinkContest_TryLinkStandby(&raw mut gTasks[taskId].data[12]) == 0 {
        return;
    }
    match gTasks[taskId].data[0] {
        0 => {
            gBlockSendBuffer[0] = gTasks[taskId].data[9] as u8;
            if GetMultiplayerId() == 0 {
                if IsLinkTaskFinished() != 0 {
                    gTasks[taskId].data[0] = 10;
                }
            } else {
                gTasks[taskId].data[0] += 1;
            }
        }
        1 => {
            if LinkContest_GetBlockReceivedFromAllPlayers() != 0 {
                i = 0;
                while i < gNumLinkContestPlayers as i32 {
                    gTasks[taskId].data[i + 1] = gBlockRecvBuffer[i][0] as i16;
                    i += 1;
                }
                gTasks[taskId].data[0] += 1;
            }
        }
        10 => {
            if ({
                gTasks[taskId].data[11] += 1;
                gTasks[taskId].data[11]
            }) > 10
            {
                SendBlockRequest(BLOCK_REQ_SIZE_100);
                gTasks[taskId].data[0] = 1;
            }
        }
        _ => {
            gTasks[taskId].data[0] = 0;
            gTasks[taskId].data[11] = 0;
            gTasks[taskId].data[12] = 0;
            SwitchTaskToFollowupFunc(taskId);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Task_LinkContest_CommunicateMonIdxs(taskId: u8) {
    match gTasks[taskId].data[0] {
        0 => {
            if IsLinkTaskFinished() != 0 {
                if LinkContest_SendBlock(&raw mut gContestPlayerMonIndex as *mut c_void, 1)
                    == TRUE as u32
                {
                    gTasks[taskId].data[0] += 1;
                }
            }
        }
        1 => {
            if LinkContest_GetBlockReceivedFromAllPlayers() != 0 {
                gTasks[taskId].data[0] += 1;
            }
        }
        _ => {
            gTasks[taskId].data[0] = 0;
            SwitchTaskToFollowupFunc(taskId);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Task_LinkContest_CommunicateMoveSelections(taskId: u8) {
    let mut i: i32 = 0;
    match gTasks[taskId].data[0] {
        0 => {
            if IsLinkTaskFinished() != 0 {
                if LinkContest_SendBlock(
                    &raw mut (*(*gContestResources).status.at(gContestPlayerMonIndex)).currMove
                        as *mut c_void,
                    2,
                ) == TRUE as u32
                {
                    gTasks[taskId].data[0] += 1;
                }
            }
        }
        1 => {
            if LinkContest_GetBlockReceivedFromAllPlayers() != 0 {
                i = 0;
                while i < gNumLinkContestPlayers as i32 {
                    (*(*gContestResources).status.at(i)).currMove = gBlockRecvBuffer[i][0];
                    i += 1;
                }
                gTasks[taskId].data[0] += 1;
            }
        }
        _ => {
            gTasks[taskId].data[0] = 0;
            SwitchTaskToFollowupFunc(taskId);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Task_LinkContest_CommunicateFinalStandings(taskId: u8) {
    match gTasks[taskId].data[0] {
        0 => {
            if IsLinkTaskFinished() != 0 {
                if LinkContest_SendBlock(gContestMonTotalPoints.as_mut_ptr() as *mut c_void, 8) == 1
                {
                    gTasks[taskId].data[0] += 1;
                }
            }
        }
        1 => {
            if LinkContest_GetBlockReceivedFromAllPlayers() != 0 {
                memcpy(
                    gContestMonTotalPoints.as_mut_ptr() as *mut u8,
                    gBlockRecvBuffer[gContestLinkLeaderIndex].as_mut_ptr() as *mut u8,
                    8,
                );
                gTasks[taskId].data[0] += 1;
            }
        }
        2 | 5 | 8 | 11 => {
            if ({
                let t1 = gTasks[taskId].data[1];
                gTasks[taskId].data[1] += 1;
                t1
            }) > 10
            {
                gTasks[taskId].data[1] = 0;
                gTasks[taskId].data[0] += 1;
            }
        }
        3 => {
            if IsLinkTaskFinished() != 0 {
                if LinkContest_SendBlock(
                    gContestMonAppealPointTotals.as_mut_ptr() as *mut c_void,
                    8,
                ) == 1
                {
                    gTasks[taskId].data[0] += 1;
                }
            }
        }
        4 => {
            if LinkContest_GetBlockReceivedFromAllPlayers() != 0 {
                memcpy(
                    gContestMonAppealPointTotals.as_mut_ptr() as *mut u8,
                    gBlockRecvBuffer[gContestLinkLeaderIndex].as_mut_ptr() as *mut u8,
                    8,
                );
                gTasks[taskId].data[0] += 1;
            }
        }
        6 => {
            if IsLinkTaskFinished() != 0 {
                if LinkContest_SendBlock(gContestMonRound2Points.as_mut_ptr() as *mut c_void, 8)
                    == 1
                {
                    gTasks[taskId].data[0] += 1;
                }
            }
        }
        7 => {
            if LinkContest_GetBlockReceivedFromAllPlayers() != 0 {
                memcpy(
                    gContestMonRound2Points.as_mut_ptr() as *mut u8,
                    gBlockRecvBuffer[gContestLinkLeaderIndex].as_mut_ptr() as *mut u8,
                    8,
                );
                gTasks[taskId].data[0] += 1;
            }
        }
        9 => {
            if IsLinkTaskFinished() != 0 {
                if LinkContest_SendBlock(gContestFinalStandings.as_mut_ptr() as *mut c_void, 4) == 1
                {
                    gTasks[taskId].data[0] += 1;
                }
            }
        }
        10 => {
            if LinkContest_GetBlockReceivedFromAllPlayers() != 0 {
                memcpy(
                    gContestFinalStandings.as_mut_ptr(),
                    gBlockRecvBuffer[gContestLinkLeaderIndex].as_mut_ptr() as *mut u8,
                    4,
                );
                gTasks[taskId].data[0] += 1;
            }
        }
        _ => {
            gTasks[taskId].data[0] = 0;
            SwitchTaskToFollowupFunc(taskId);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Task_LinkContest_CommunicateAppealsState(taskId: u8) {
    match gTasks[taskId].data[0] {
        0 => {
            if IsLinkTaskFinished() != 0 {
                if LinkContest_SendBlock((*gContestResources).status as *mut c_void, 112) == 1 {
                    gTasks[taskId].data[0] += 1;
                }
            }
        }
        1 => {
            if LinkContest_GetBlockReceivedFromAllPlayers() != 0 {
                memcpy(
                    (*gContestResources).status as *mut u8,
                    gBlockRecvBuffer[gContestLinkLeaderIndex].as_mut_ptr() as *mut u8,
                    112,
                );
                gTasks[taskId].data[0] += 1;
            }
        }
        2 | 5 | 8 | 11 => {
            if ({
                let t1 = gTasks[taskId].data[1];
                gTasks[taskId].data[1] += 1;
                t1
            }) > 10
            {
                gTasks[taskId].data[1] = 0;
                gTasks[taskId].data[0] += 1;
            }
        }
        3 => {
            if IsLinkTaskFinished() != 0 {
                if LinkContest_SendBlock((*gContestResources).appealResults as *mut c_void, 20) == 1
                {
                    gTasks[taskId].data[0] += 1;
                }
            }
        }
        4 => {
            if LinkContest_GetBlockReceivedFromAllPlayers() != 0 {
                memcpy(
                    (*gContestResources).appealResults as *mut u8,
                    gBlockRecvBuffer[gContestLinkLeaderIndex].as_mut_ptr() as *mut u8,
                    20,
                );
                gTasks[taskId].data[0] += 1;
            }
        }
        6 => {
            if IsLinkTaskFinished() != 0 {
                if LinkContest_SendBlock((*gContestResources).excitement as *mut c_void, 4) == 1 {
                    gTasks[taskId].data[0] += 1;
                }
            }
        }
        7 => {
            if LinkContest_GetBlockReceivedFromAllPlayers() != 0 {
                memcpy(
                    (*gContestResources).excitement as *mut u8,
                    gBlockRecvBuffer[gContestLinkLeaderIndex].as_mut_ptr() as *mut u8,
                    4,
                );
                gTasks[taskId].data[0] += 1;
            }
        }
        9 => {
            if IsLinkTaskFinished() != 0 {
                if LinkContest_SendBlock(gContestantTurnOrder.as_mut_ptr() as *mut c_void, 4) == 1 {
                    gTasks[taskId].data[0] += 1;
                }
            }
        }
        10 => {
            if LinkContest_GetBlockReceivedFromAllPlayers() != 0 {
                memcpy(
                    gContestantTurnOrder.as_mut_ptr(),
                    gBlockRecvBuffer[gContestLinkLeaderIndex].as_mut_ptr() as *mut u8,
                    4,
                );
                gTasks[taskId].data[0] += 1;
            }
        }
        _ => {
            gTasks[taskId].data[0] = 0;
            SwitchTaskToFollowupFunc(taskId);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Task_LinkContest_CommunicateLeaderIdsRS(taskId: u8) {
    let mut i: i32 = 0;
    if LinkContest_TryLinkStandby(&raw mut gTasks[taskId].data[12]) == 0 {
        return;
    }
    match gTasks[taskId].data[0] {
        0 => {
            gBlockSendBuffer[0] = 0x6E;
            if GetMultiplayerId() == 0 {
                if IsLinkTaskFinished() != 0 {
                    gTasks[taskId].data[0] = 10;
                }
            } else {
                gTasks[taskId].data[0] += 1;
            }
        }
        1 => {
            if LinkContest_GetBlockReceivedFromAllPlayers() != 0 {
                i = 0;
                while i < CONTESTANT_COUNT {
                    gTasks[taskId].data[i + 5] = gBlockRecvBuffer[i][0] as i16;
                    i += 1;
                }
                gTasks[taskId].data[0] += 1;
            }
        }
        10 => {
            if ({
                gTasks[taskId].data[11] += 1;
                gTasks[taskId].data[11]
            }) > 10
            {
                SendBlockRequest(BLOCK_REQ_SIZE_100);
                gTasks[taskId].data[0] = 1;
            }
        }
        _ => {
            gTasks[taskId].data[0] = 0;
            gTasks[taskId].data[11] = 0;
            gTasks[taskId].data[12] = 0;
            SwitchTaskToFollowupFunc(taskId);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Task_LinkContest_CommunicateRound1Points(taskId: u8) {
    if LinkContest_TryLinkStandby(&raw mut gTasks[taskId].data[12]) == 0 {
        return;
    }
    match gTasks[taskId].data[0] {
        0 => {
            if IsLinkTaskFinished() != 0 {
                if LinkContest_SendBlock(gContestMonRound1Points.as_mut_ptr() as *mut c_void, 8)
                    == 1
                {
                    gTasks[taskId].data[0] += 1;
                }
            }
        }
        1 => {
            if LinkContest_GetBlockReceivedFromAllPlayers() != 0 {
                memcpy(
                    gContestMonRound1Points.as_mut_ptr() as *mut u8,
                    gBlockRecvBuffer[gContestLinkLeaderIndex].as_mut_ptr() as *mut u8,
                    8,
                );
                gTasks[taskId].data[0] += 1;
            }
        }
        _ => {
            gTasks[taskId].data[0] = 0;
            gTasks[taskId].data[12] = 0;
            SwitchTaskToFollowupFunc(taskId);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Task_LinkContest_CommunicateTurnOrder(taskId: u8) {
    if LinkContest_TryLinkStandby(&raw mut gTasks[taskId].data[12]) == 0 {
        return;
    }
    match gTasks[taskId].data[0] {
        0 => {
            if IsLinkTaskFinished() != 0 {
                if LinkContest_SendBlock(gContestantTurnOrder.as_mut_ptr() as *mut c_void, 4) == 1 {
                    gTasks[taskId].data[0] += 1;
                }
            }
        }
        1 => {
            if LinkContest_GetBlockReceivedFromAllPlayers() != 0 {
                memcpy(
                    gContestantTurnOrder.as_mut_ptr(),
                    gBlockRecvBuffer[gContestLinkLeaderIndex].as_mut_ptr() as *mut u8,
                    4,
                );
                gTasks[taskId].data[0] += 1;
            }
        }
        _ => {
            gTasks[taskId].data[0] = 0;
            gTasks[taskId].data[12] = 0;
            SwitchTaskToFollowupFunc(taskId);
        }
    }
}
