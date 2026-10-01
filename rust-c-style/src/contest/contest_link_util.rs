//! Translated from `src/contest_link_util.c` by tools/rustport/c2rs.py.
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
    static mut gContestLinkLeaderIndex: u8;
    static mut gContestMonPartyIndex: u8;
    static mut gContestMons: CArray<ContestPokemon, 4>;
    static mut gContestPlayerMonIndex: u8;
    static mut gContestRngValue: u32;
    static mut gHighestRibbonRank: u8;
    static mut gLinkPlayers: CArray<LinkPlayer, 5>;
    static mut gNumLinkContestPlayers: u8;
    static mut gPlayerParty: CArray<Pokemon, 6>;
    static mut gRngValue: u32;
    static mut gSpecialVar_0x8004: u16;
    static mut gSpecialVar_ContestCategory: u16;
    static mut gTasks: CArray<Task, 0>;
    fn CalculateRound1Points(a0: u8);
    fn FlagGet(a0: u16) -> u8;
    fn GetMonData2(a0: *mut Pokemon, a1: i32) -> u32;
    fn GetMultiplayerId() -> u8;
    fn IsLinkTaskFinished() -> u8;
    fn LinkContest_GetBlockReceived(a0: u8) -> u8;
    fn LinkContest_GetBlockReceivedFromAllPlayers() -> u8;
    fn LinkContest_GetLeaderIndex(a0: *mut u8) -> u8;
    fn LinkContest_SendBlock(a0: *mut c_void, a1: u16) -> u32;
    fn LinkContest_TryLinkStandby(a0: *mut i16) -> u32;
    fn SetLinkAIContestants(a0: u8, a1: u8, a2: u32);
    fn SetTaskFuncWithFollowupFunc(
        a0: u8,
        a1: Option<unsafe extern "C" fn(u8)>,
        a2: Option<unsafe extern "C" fn(u8)>,
    );
    fn SortContestants(a0: u8);
    fn StripPlayerAndMonNamesForLinkContest(a0: *mut ContestPokemon, a1: i32);
    fn SwitchTaskToFollowupFunc(a0: u8);
    fn Task_LinkContest_CommunicateRound1Points(a0: u8);
    fn Task_LinkContest_CommunicateTurnOrder(a0: u8);
    fn Task_LinkContest_FinalizeConnection(a0: u8);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn Task_LinkContest_StartCommunicationEm(taskId: u8) {
    let mut gameCleared: i32 = 0;
    match gTasks[taskId].data[9] {
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
    gameCleared = (FlagGet(FLAG_SYS_GAME_CLEAR) > 0) as i32;
    gContestMons[gContestPlayerMonIndex].gameCleared = gameCleared as u8;
    SetTaskFuncWithFollowupFunc(
        taskId,
        Some(Task_LinkContest_CommunicateMonsEm),
        Some(Task_LinkContest_StartCommunicateRngEm),
    );
}
pub(crate) unsafe extern "C" fn Task_LinkContest_StartCommunicateRngEm(taskId: u8) {
    SetTaskFuncWithFollowupFunc(
        taskId,
        Some(Task_LinkContest_CommunicateRngEm),
        Some(Task_LinkContest_StartCommunicateLeaderIdsEm),
    );
}
pub(crate) unsafe extern "C" fn Task_LinkContest_StartCommunicateLeaderIdsEm(taskId: u8) {
    SetTaskFuncWithFollowupFunc(
        taskId,
        Some(Task_LinkContest_CommunicateLeaderIdsEm),
        Some(Task_LinkContest_StartCommunicateCategoryEm),
    );
}
pub(crate) unsafe extern "C" fn Task_LinkContest_StartCommunicateCategoryEm(taskId: u8) {
    SetTaskFuncWithFollowupFunc(
        taskId,
        Some(Task_LinkContest_CommunicateCategoryEm),
        Some(Task_LinkContest_SetUpContestEm),
    );
}
pub(crate) unsafe extern "C" fn Task_LinkContest_SetUpContestEm(taskId: u8) {
    let mut i: u8 = 0;
    let mut rank: u8 = 0;
    let mut gameCleared: i32 = 0;
    let mut categories: CArray<u8, 4> = zeroed();
    let mut leaderIds: CArray<u8, 4> = zeroed();
    memset(categories.as_mut_ptr(), 0, 4);
    memset(leaderIds.as_mut_ptr(), 0, 4);
    i = 0;
    while i < gNumLinkContestPlayers {
        categories[i] = gTasks[taskId].data[i as i32 + 1] as u8;
        i += 1;
    }
    i = 0;
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
        leaderIds[i] = gTasks[taskId].data[i as i32 + 5] as u8;
        i += 1;
    }
    if gNumLinkContestPlayers != CONTESTANT_COUNT as u8 && GetMultiplayerId() == 0 {
        rank = gContestMons[0].highestRank;
        i = 1;
        while i < gNumLinkContestPlayers {
            if rank < gContestMons[i].highestRank {
                rank = gContestMons[i].highestRank;
            }
            i += 1;
        }
        if rank != 0 {
            rank -= 1;
        }
        gameCleared = TRUE as i32;
        i = 0;
        while i < gNumLinkContestPlayers {
            if gContestMons[i].gameCleared == 0 {
                gameCleared = FALSE as i32;
                break;
            }
            i += 1;
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
        gTasks[taskId].func = Some(Task_LinkContest_CalculateRound1Em);
    }
}
pub(crate) unsafe extern "C" fn Task_LinkContest_CalculateRound1Em(taskId: u8) {
    CalculateRound1Points(gSpecialVar_ContestCategory as u8);
    SetTaskFuncWithFollowupFunc(
        taskId,
        Some(Task_LinkContest_CommunicateRound1Points),
        Some(Task_LinkContest_CalculateTurnOrderEm),
    );
}
pub(crate) unsafe extern "C" fn Task_LinkContest_CalculateTurnOrderEm(taskId: u8) {
    SortContestants(FALSE);
    SetTaskFuncWithFollowupFunc(
        taskId,
        Some(Task_LinkContest_CommunicateTurnOrder),
        Some(Task_LinkContest_FinalizeConnection),
    );
}
pub(crate) unsafe extern "C" fn Task_LinkContest_CommunicateMonsEm(taskId: u8) {
    let mut i: i32 = 0;
    if LinkContest_TryLinkStandby(&raw mut gTasks[taskId].data[12]) == 0 {
        return;
    }
    match gTasks[taskId].data[0] {
        0 => {
            if IsLinkTaskFinished() != 0 {
                if LinkContest_SendBlock(
                    &raw mut gContestMons[gContestPlayerMonIndex] as *mut c_void,
                    64,
                ) == 1
                {
                    gTasks[taskId].data[0] += 1;
                }
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
        _ => {
            gTasks[taskId].data[0] = 0;
            gTasks[taskId].data[12] = 0;
            SwitchTaskToFollowupFunc(taskId);
        }
    }
}
pub(crate) unsafe extern "C" fn Task_LinkContest_CommunicateRngEm(taskId: u8) {
    if LinkContest_TryLinkStandby(&raw mut gTasks[taskId].data[12]) == 0 {
        return;
    }
    match gTasks[taskId].data[0] {
        0 => {
            if GetMultiplayerId() == 0 {
                if IsLinkTaskFinished() == 0 {
                    return;
                }
                if LinkContest_SendBlock(&raw mut gRngValue as *mut c_void, 4) == 1 {
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
            gTasks[taskId].data[12] = 0;
            SwitchTaskToFollowupFunc(taskId);
        }
    }
}
pub(crate) unsafe extern "C" fn Task_LinkContest_CommunicateLeaderIdsEm(taskId: u8) {
    let mut i: i32 = 0;
    let mut data: CArray<u16, 4> = zeroed();
    let mut leaderId: u16 = 0;
    if LinkContest_TryLinkStandby(&raw mut gTasks[taskId].data[12]) == 0 {
        return;
    }
    match gTasks[taskId].data[0] {
        0 => {
            if IsLinkTaskFinished() != 0 {
                leaderId = 0x6E;
                if LinkContest_SendBlock(&raw mut leaderId as *mut c_void, 2) == 1 {
                    gTasks[taskId].data[0] += 1;
                }
            }
        }
        1 => {
            if LinkContest_GetBlockReceivedFromAllPlayers() != 0 {
                i = 0;
                while i < gNumLinkContestPlayers as i32 {
                    data[i] = gBlockRecvBuffer[i][0];
                    gTasks[taskId].data[i + 5] = data[i] as i16;
                    i += 1;
                }
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
pub(crate) unsafe extern "C" fn Task_LinkContest_CommunicateCategoryEm(taskId: u8) {
    let mut i: i32 = 0;
    let mut data: CArray<u16, 4> = zeroed();
    let mut category: u16 = 0;
    if LinkContest_TryLinkStandby(&raw mut gTasks[taskId].data[12]) == 0 {
        return;
    }
    match gTasks[taskId].data[0] {
        0 => {
            if IsLinkTaskFinished() != 0 {
                category = gTasks[taskId].data[9] as u16;
                if LinkContest_SendBlock(&raw mut category as *mut c_void, 2) == 1 {
                    gTasks[taskId].data[0] += 1;
                }
            }
        }
        1 => {
            if LinkContest_GetBlockReceivedFromAllPlayers() != 0 {
                i = 0;
                while i < gNumLinkContestPlayers as i32 {
                    data[i] = gBlockRecvBuffer[i][0];
                    gTasks[taskId].data[i + 1] = data[i] as i16;
                    i += 1;
                }
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
pub(crate) unsafe extern "C" fn Task_LinkContest_CommunicateAIMonsEm(taskId: u8) {
    let mut i: i32 = 0;
    if LinkContest_TryLinkStandby(&raw mut gTasks[taskId].data[12]) == 0 {
        return;
    }
    match gTasks[taskId].data[0] {
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
                    gTasks[taskId].data[0] += 1;
                }
            } else {
                gTasks[taskId].data[0] += 1;
            }
        }
        1 => {
            if LinkContest_GetBlockReceived(0) != 0 {
                memcpy(
                    &raw mut gContestMons[gNumLinkContestPlayers] as *mut u8,
                    gBlockRecvBuffer[0].as_mut_ptr() as *mut u8,
                    (CONTESTANT_COUNT as u32 - gNumLinkContestPlayers as u32) * 64,
                );
                i = gNumLinkContestPlayers as i32;
                while i < CONTESTANT_COUNT {
                    StripPlayerAndMonNamesForLinkContest(
                        &raw mut gContestMons[i],
                        gLinkPlayers[0].language as i32,
                    );
                    i += 1;
                }
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
