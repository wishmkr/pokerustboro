//! Translated from `src/contest_link_util.c` by tools/rustport/c2rs.py.
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
    unused_assignments
)]

#[allow(unused_imports)]
use crate::c::*;
#[allow(unused_imports)]
use crate::consts::*;
use crate::contest::{
    CalculateRound1Points, SetLinkAIContestants, SortContestants,
    StripPlayerAndMonNamesForLinkContest, gContestLinkLeaderIndex, gContestMonPartyIndex,
    gContestMons, gContestPlayerMonIndex, gContestRngValue, gHighestRibbonRank,
    gNumLinkContestPlayers, gSpecialVar_ContestCategory,
};
use crate::contest_link::{
    LinkContest_GetBlockReceived, LinkContest_GetBlockReceivedFromAllPlayers,
    LinkContest_SendBlock, LinkContest_TryLinkStandby, Task_LinkContest_CommunicateRound1Points,
    Task_LinkContest_CommunicateTurnOrder,
};
use crate::contest_util::{LinkContest_GetLeaderIndex, Task_LinkContest_FinalizeConnection};
use crate::event_data::FlagGet;
use crate::ffi::gSpecialVar_0x8004;
use crate::link::gBlockRecvBuffer;
use crate::link::{GetMultiplayerId, IsLinkTaskFinished, gLinkPlayers};
use crate::pokemon::{GetMonData2, gPlayerParty};
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
const tCategory: usize = 9;

pub unsafe fn Task_LinkContest_StartCommunicationEm(taskId: u8) {
    match task_get(taskId, tCategory) {
        0 => {
            gHighestRibbonRank = GetMonData2(
                &raw mut gPlayerParty[gContestMonPartyIndex],
                MON_DATA_COOL_RIBBON,
            ) as u8;
        }
        1 => {
            gHighestRibbonRank = GetMonData2(
                &raw mut gPlayerParty[gContestMonPartyIndex],
                MON_DATA_BEAUTY_RIBBON,
            ) as u8;
        }
        2 => {
            gHighestRibbonRank = GetMonData2(
                &raw mut gPlayerParty[gContestMonPartyIndex],
                MON_DATA_CUTE_RIBBON,
            ) as u8;
        }
        3 => {
            gHighestRibbonRank = GetMonData2(
                &raw mut gPlayerParty[gContestMonPartyIndex],
                MON_DATA_SMART_RIBBON,
            ) as u8;
        }
        _ => {
            gHighestRibbonRank = GetMonData2(
                &raw mut gPlayerParty[gContestMonPartyIndex],
                MON_DATA_TOUGH_RIBBON,
            ) as u8;
        }
    }
    gContestMons[gContestPlayerMonIndex].highestRank = gHighestRibbonRank;
    let gameCleared: i32 = (FlagGet(FLAG_SYS_GAME_CLEAR) > 0) as i32;
    gContestMons[gContestPlayerMonIndex].gameCleared = gameCleared as u8;
    SetTaskFuncWithFollowupFunc(
        taskId,
        Some(Task_LinkContest_CommunicateMonsEm),
        Some(Task_LinkContest_StartCommunicateRngEm),
    );
}
pub(crate) unsafe fn Task_LinkContest_StartCommunicateRngEm(taskId: u8) {
    SetTaskFuncWithFollowupFunc(
        taskId,
        Some(Task_LinkContest_CommunicateRngEm),
        Some(Task_LinkContest_StartCommunicateLeaderIdsEm),
    );
}
pub(crate) unsafe fn Task_LinkContest_StartCommunicateLeaderIdsEm(taskId: u8) {
    SetTaskFuncWithFollowupFunc(
        taskId,
        Some(Task_LinkContest_CommunicateLeaderIdsEm),
        Some(Task_LinkContest_StartCommunicateCategoryEm),
    );
}
pub(crate) unsafe fn Task_LinkContest_StartCommunicateCategoryEm(taskId: u8) {
    SetTaskFuncWithFollowupFunc(
        taskId,
        Some(Task_LinkContest_CommunicateCategoryEm),
        Some(Task_LinkContest_SetUpContestEm),
    );
}
pub(crate) unsafe fn Task_LinkContest_SetUpContestEm(taskId: u8) {
    let mut rank: u8 = 0;
    let mut gameCleared: i32 = 0;
    let mut categories: CArray<u8, 4> = zeroed();
    let mut leaderIds: CArray<u8, 4> = zeroed();
    memset(categories.as_mut_ptr(), 0, 4);
    memset(leaderIds.as_mut_ptr(), 0, 4);
    for i in 0..gNumLinkContestPlayers {
        categories[i] = task_get(taskId, i as i32 + 1) as u8;
    }
    let mut i: u8 = 0;
    while i < gNumLinkContestPlayers && categories[0] == categories[i] {
        i += 1;
    }
    if i == gNumLinkContestPlayers {
        gSpecialVar_0x8004 = FALSE as u16;
    } else {
        gSpecialVar_0x8004 = TRUE as u16;
    }
    i = 0;
    while i < gNumLinkContestPlayers {
        leaderIds[i] = task_get(taskId, i as i32 + 5) as u8;
        i += 1;
    }
    if gNumLinkContestPlayers != CONTESTANT_COUNT as u8 && GetMultiplayerId() == 0 {
        rank = gContestMons[0].highestRank;
        for i in 1..gNumLinkContestPlayers {
            if rank < gContestMons[i].highestRank {
                rank = gContestMons[i].highestRank;
            }
        }
        rank = rank.saturating_sub(1);
        gameCleared = TRUE as i32;
        for i in 0..gNumLinkContestPlayers {
            if gContestMons[i].gameCleared == 0 {
                gameCleared = FALSE as i32;
                break;
            }
        }
        SetLinkAIContestants(categories[0], rank, gameCleared as u32);
    }
    gContestLinkLeaderIndex = LinkContest_GetLeaderIndex(leaderIds.as_mut_ptr());
    if gNumLinkContestPlayers < CONTESTANT_COUNT as u8 {
        SetTaskFuncWithFollowupFunc(
            taskId,
            Some(Task_LinkContest_CommunicateAIMonsEm),
            Some(Task_LinkContest_CalculateRound1Em),
        );
    } else {
        task_set_func(taskId, Some(Task_LinkContest_CalculateRound1Em));
    }
}
pub(crate) unsafe fn Task_LinkContest_CalculateRound1Em(taskId: u8) {
    CalculateRound1Points(gSpecialVar_ContestCategory as u8);
    SetTaskFuncWithFollowupFunc(
        taskId,
        Some(Task_LinkContest_CommunicateRound1Points),
        Some(Task_LinkContest_CalculateTurnOrderEm),
    );
}
pub(crate) unsafe fn Task_LinkContest_CalculateTurnOrderEm(taskId: u8) {
    SortContestants(FALSE);
    SetTaskFuncWithFollowupFunc(
        taskId,
        Some(Task_LinkContest_CommunicateTurnOrder),
        Some(Task_LinkContest_FinalizeConnection),
    );
}
pub(crate) unsafe fn Task_LinkContest_CommunicateMonsEm(taskId: u8) {
    let mut i: i32 = 0;
    if LinkContest_TryLinkStandby(task_data_ptr(taskId, 12)) == 0 {
        return;
    }
    match task_get(taskId, 0) {
        0 => {
            if IsLinkTaskFinished() != 0
                && LinkContest_SendBlock(
                    &raw mut gContestMons[gContestPlayerMonIndex] as *mut c_void,
                    64,
                ) == 1
            {
                task_set(taskId, 0, task_get(taskId, 0) + 1);
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
                task_set(taskId, 0, task_get(taskId, 0) + 1);
            }
        }
        _ => {
            task_set(taskId, 0, 0);
            task_set(taskId, 12, 0);
            SwitchTaskToFollowupFunc(taskId);
        }
    }
}
pub(crate) unsafe fn Task_LinkContest_CommunicateRngEm(taskId: u8) {
    if LinkContest_TryLinkStandby(task_data_ptr(taskId, 12)) == 0 {
        return;
    }
    match task_get(taskId, 0) {
        0 => {
            if GetMultiplayerId() == 0 {
                if IsLinkTaskFinished() == 0 {
                    return;
                }
                if LinkContest_SendBlock(
                    &raw mut (*crate::random::gRngValue.as_ptr().cast::<u32>()) as *mut c_void,
                    4,
                ) == 1
                {
                    task_set(taskId, 0, task_get(taskId, 0) + 1);
                }
            } else {
                task_set(taskId, 0, task_get(taskId, 0) + 1);
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
                task_set(taskId, 0, task_get(taskId, 0) + 1);
            }
        }
        _ => {
            task_set(taskId, 0, 0);
            task_set(taskId, 12, 0);
            SwitchTaskToFollowupFunc(taskId);
        }
    }
}
pub(crate) unsafe fn Task_LinkContest_CommunicateLeaderIdsEm(taskId: u8) {
    let mut data: CArray<u16, 4> = zeroed();
    let mut leaderId: u16 = 0;
    if LinkContest_TryLinkStandby(task_data_ptr(taskId, 12)) == 0 {
        return;
    }
    match task_get(taskId, 0) {
        0 => {
            if IsLinkTaskFinished() != 0 {
                leaderId = 0x6E;
                if LinkContest_SendBlock(&raw mut leaderId as *mut c_void, 2) == 1 {
                    task_set(taskId, 0, task_get(taskId, 0) + 1);
                }
            }
        }
        1 => {
            if LinkContest_GetBlockReceivedFromAllPlayers() != 0 {
                for i in 0..(gNumLinkContestPlayers as i32) {
                    data[i] = gBlockRecvBuffer[i][0];
                    task_set(taskId, i + 5, data[i] as i16);
                }
                task_set(taskId, 0, task_get(taskId, 0) + 1);
            }
        }
        _ => {
            task_set(taskId, 0, 0);
            task_set(taskId, 12, 0);
            SwitchTaskToFollowupFunc(taskId);
        }
    }
}
pub(crate) unsafe fn Task_LinkContest_CommunicateCategoryEm(taskId: u8) {
    let mut data: CArray<u16, 4> = zeroed();
    let mut category: u16 = 0;
    if LinkContest_TryLinkStandby(task_data_ptr(taskId, 12)) == 0 {
        return;
    }
    match task_get(taskId, 0) {
        0 => {
            if IsLinkTaskFinished() != 0 {
                category = task_get(taskId, tCategory) as u16;
                if LinkContest_SendBlock(&raw mut category as *mut c_void, 2) == 1 {
                    task_set(taskId, 0, task_get(taskId, 0) + 1);
                }
            }
        }
        1 => {
            if LinkContest_GetBlockReceivedFromAllPlayers() != 0 {
                for i in 0..(gNumLinkContestPlayers as i32) {
                    data[i] = gBlockRecvBuffer[i][0];
                    task_set(taskId, i + 1, data[i] as i16);
                }
                task_set(taskId, 0, task_get(taskId, 0) + 1);
            }
        }
        _ => {
            task_set(taskId, 0, 0);
            task_set(taskId, 12, 0);
            SwitchTaskToFollowupFunc(taskId);
        }
    }
}
pub(crate) unsafe fn Task_LinkContest_CommunicateAIMonsEm(taskId: u8) {
    if LinkContest_TryLinkStandby(task_data_ptr(taskId, 12)) == 0 {
        return;
    }
    match task_get(taskId, 0) {
        0 => {
            if GetMultiplayerId() == 0 {
                if IsLinkTaskFinished() == 0 {
                    return;
                }
                if LinkContest_SendBlock(
                    &raw mut gContestMons[gNumLinkContestPlayers] as *mut c_void,
                    (CONTESTANT_COUNT as u16 - gNumLinkContestPlayers as u16) * 64,
                ) == 1
                {
                    task_set(taskId, 0, task_get(taskId, 0) + 1);
                }
            } else {
                task_set(taskId, 0, task_get(taskId, 0) + 1);
            }
        }
        1 => {
            if LinkContest_GetBlockReceived(0) != 0 {
                memcpy(
                    &raw mut gContestMons[gNumLinkContestPlayers] as *mut u8,
                    gBlockRecvBuffer[0].as_mut_ptr() as *mut u8,
                    (CONTESTANT_COUNT as u32 - gNumLinkContestPlayers as u32) * 64,
                );
                for i in (gNumLinkContestPlayers as i32)..CONTESTANT_COUNT {
                    StripPlayerAndMonNamesForLinkContest(
                        &raw mut gContestMons[i],
                        gLinkPlayers[0].language as i32,
                    );
                }
                task_set(taskId, 0, task_get(taskId, 0) + 1);
            }
        }
        _ => {
            task_set(taskId, 0, 0);
            task_set(taskId, 12, 0);
            SwitchTaskToFollowupFunc(taskId);
        }
    }
}
