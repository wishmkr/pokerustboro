//! Translated from `src/contest_link_util.c` by tools/rustport/c2rs.py, then reviewed.
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

unsafe extern "C" {
    static mut gBlockRecvBuffer: u8;
    static mut gContestLinkLeaderIndex: u8;
    static mut gContestMonPartyIndex: u8;
    static mut gContestMons: u8;
    static mut gContestPlayerMonIndex: u8;
    static mut gContestRngValue: u8;
    static mut gHighestRibbonRank: u8;
    static mut gLinkPlayers: u8;
    static mut gNumLinkContestPlayers: u8;
    static mut gPlayerParty: u8;
    static mut gRngValue: u8;
    static mut gSpecialVar_0x8004: u8;
    static mut gSpecialVar_ContestCategory: u8;
    static mut gTasks: u8;
    fn CalculateRound1Points(a0: u8);
    fn FlagGet(a0: u16) -> u8;
    fn GetMonData2(a0: *mut u8, a1: i32) -> u32;
    fn GetMultiplayerId() -> u8;
    fn IsLinkTaskFinished() -> u8;
    fn LinkContest_GetBlockReceived(a0: u8) -> u8;
    fn LinkContest_GetBlockReceivedFromAllPlayers() -> u8;
    fn LinkContest_GetLeaderIndex(a0: *mut u8) -> u8;
    fn LinkContest_SendBlock(a0: *mut u8, a1: u16) -> u32;
    fn LinkContest_TryLinkStandby(a0: *mut i16) -> u32;
    fn SetLinkAIContestants(a0: u8, a1: u8, a2: u32);
    fn SetTaskFuncWithFollowupFunc(
        a0: u8,
        a1: Option<unsafe extern "C" fn(u8)>,
        a2: Option<unsafe extern "C" fn(u8)>,
    );
    fn SortContestants(a0: u8);
    fn StripPlayerAndMonNamesForLinkContest(a0: *mut u8, a1: i32);
    fn SwitchTaskToFollowupFunc(a0: u8);
    fn Task_LinkContest_CommunicateRound1Points(a0: u8);
    fn Task_LinkContest_CommunicateTurnOrder(a0: u8);
    fn Task_LinkContest_FinalizeConnection(a0: u8);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn Task_LinkContest_StartCommunicationEm(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut gameCleared: i32 = 0i32;
        'l1: {
            let __sw1 = ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(9))
            .read()) as i32);
            let __matched =
                __sw1 == 0i32 || __sw1 == 1i32 || __sw1 == 2i32 || __sw1 == 3i32 || __sw1 == 4i32;
            if __sw1 == 0i32 {
                ((&raw mut gHighestRibbonRank).cast::<u8>()).write(
                    ((GetMonData2(
                        ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                            ((((&raw mut gContestMonPartyIndex).cast::<u8>()).read()) as i32)
                                as isize
                                * 100,
                        ),
                        50i32,
                    )) as u8),
                );
                break 'l1;
            }
            if __sw1 == 1i32 {
                ((&raw mut gHighestRibbonRank).cast::<u8>()).write(
                    ((GetMonData2(
                        ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                            ((((&raw mut gContestMonPartyIndex).cast::<u8>()).read()) as i32)
                                as isize
                                * 100,
                        ),
                        51i32,
                    )) as u8),
                );
                break 'l1;
            }
            if __sw1 == 2i32 {
                ((&raw mut gHighestRibbonRank).cast::<u8>()).write(
                    ((GetMonData2(
                        ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                            ((((&raw mut gContestMonPartyIndex).cast::<u8>()).read()) as i32)
                                as isize
                                * 100,
                        ),
                        52i32,
                    )) as u8),
                );
                break 'l1;
            }
            if __sw1 == 3i32 {
                ((&raw mut gHighestRibbonRank).cast::<u8>()).write(
                    ((GetMonData2(
                        ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                            ((((&raw mut gContestMonPartyIndex).cast::<u8>()).read()) as i32)
                                as isize
                                * 100,
                        ),
                        53i32,
                    )) as u8),
                );
                break 'l1;
            }
            if __sw1 == 4i32 || !__matched {
                ((&raw mut gHighestRibbonRank).cast::<u8>()).write(
                    ((GetMonData2(
                        ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                            ((((&raw mut gContestMonPartyIndex).cast::<u8>()).read()) as i32)
                                as isize
                                * 100,
                        ),
                        54i32,
                    )) as u8),
                );
                break 'l1;
            }
        }
        ((((&raw mut gContestMons).cast::<u8>()).wrapping_offset(
            ((((&raw mut gContestPlayerMonIndex).cast::<u8>()).read()) as i32) as isize * 64,
        ))
        .wrapping_add(44))
        .write(((&raw mut gHighestRibbonRank).cast::<u8>()).read());
        gameCleared = ((((FlagGet(2148u16)) as i32) > 0i32) as i32);
        ((((&raw mut gContestMons).cast::<u8>()).wrapping_offset(
            ((((&raw mut gContestPlayerMonIndex).cast::<u8>()).read()) as i32) as isize * 64,
        ))
        .wrapping_add(45))
        .write(((gameCleared) as u8));
        SetTaskFuncWithFollowupFunc(
            taskId,
            Some(Task_LinkContest_CommunicateMonsEm),
            Some(Task_LinkContest_StartCommunicateRngEm),
        );
    }
}
pub(crate) unsafe extern "C" fn Task_LinkContest_StartCommunicateRngEm(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        SetTaskFuncWithFollowupFunc(
            taskId,
            Some(Task_LinkContest_CommunicateRngEm),
            Some(Task_LinkContest_StartCommunicateLeaderIdsEm),
        );
    }
}
pub(crate) unsafe extern "C" fn Task_LinkContest_StartCommunicateLeaderIdsEm(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        SetTaskFuncWithFollowupFunc(
            taskId,
            Some(Task_LinkContest_CommunicateLeaderIdsEm),
            Some(Task_LinkContest_StartCommunicateCategoryEm),
        );
    }
}
pub(crate) unsafe extern "C" fn Task_LinkContest_StartCommunicateCategoryEm(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        SetTaskFuncWithFollowupFunc(
            taskId,
            Some(Task_LinkContest_CommunicateCategoryEm),
            Some(Task_LinkContest_SetUpContestEm),
        );
    }
}
pub(crate) unsafe extern "C" fn Task_LinkContest_SetUpContestEm(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut i: u8 = 0u8;
        let mut rank: u8 = 0u8;
        let mut gameCleared: i32 = 0i32;
        let mut categories = crate::ffi::Align4([0u8; 4]);
        let mut leaderIds = crate::ffi::Align4([0u8; 4]);
        crate::c::memset((&raw mut categories).cast::<u8>(), 0i32, 4u32);
        crate::c::memset((&raw mut leaderIds).cast::<u8>(), 0i32, 4u32);
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32)
                    < ((((&raw mut gNumLinkContestPlayers).cast::<u8>()).read()) as i32))
                {
                    break 'l1;
                }
                'l2: {
                    (((&raw mut categories).cast::<u8>()).wrapping_offset(((i) as i32) as isize))
                        .write(
                            ((((((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .wrapping_offset((((i) as i32).wrapping_add(1i32)) as isize))
                            .read()) as u8),
                        );
                }
                i = (i).wrapping_add(1);
            }
        }
        {
            i = 0u8;
            'l3: loop {
                if !((((i) as i32)
                    < ((((&raw mut gNumLinkContestPlayers).cast::<u8>()).read()) as i32))
                    && (((((&raw mut categories).cast::<u8>()).read()) as i32)
                        == (((((&raw mut categories).cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize))
                        .read()) as i32)))
                {
                    break 'l3;
                }
                'l4: {}
                i = (i).wrapping_add(1);
            }
        }
        if ((i) as i32) == ((((&raw mut gNumLinkContestPlayers).cast::<u8>()).read()) as i32) {
            ((&raw mut gSpecialVar_0x8004).cast::<u16>()).write(0u16);
        } else {
            ((&raw mut gSpecialVar_0x8004).cast::<u16>()).write(1u16);
        }
        {
            i = 0u8;
            'l5: loop {
                if !(((i) as i32)
                    < ((((&raw mut gNumLinkContestPlayers).cast::<u8>()).read()) as i32))
                {
                    break 'l5;
                }
                'l6: {
                    (((&raw mut leaderIds).cast::<u8>()).wrapping_offset(((i) as i32) as isize))
                        .write(
                            ((((((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .wrapping_offset((((i) as i32).wrapping_add(5i32)) as isize))
                            .read()) as u8),
                        );
                }
                i = (i).wrapping_add(1);
            }
        }
        if (((((&raw mut gNumLinkContestPlayers).cast::<u8>()).read()) as i32) != 4i32)
            && (((GetMultiplayerId()) as i32) == 0i32)
        {
            rank = (((&raw mut gContestMons).cast::<u8>()).wrapping_add(44)).read();
            {
                i = 1u8;
                'l7: loop {
                    if !(((i) as i32)
                        < ((((&raw mut gNumLinkContestPlayers).cast::<u8>()).read()) as i32))
                    {
                        break 'l7;
                    }
                    'l8: {
                        if ((rank) as i32)
                            < ((((((&raw mut gContestMons).cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize * 64))
                            .wrapping_add(44))
                            .read()) as i32)
                        {
                            rank = ((((&raw mut gContestMons).cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize * 64))
                            .wrapping_add(44))
                            .read();
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
            if (rank) != 0 {
                rank = (rank).wrapping_sub(1);
            }
            gameCleared = 1i32;
            {
                i = 0u8;
                'l9: loop {
                    if !(((i) as i32)
                        < ((((&raw mut gNumLinkContestPlayers).cast::<u8>()).read()) as i32))
                    {
                        break 'l9;
                    }
                    'l10: {
                        if !((((((&raw mut gContestMons).cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 64))
                        .wrapping_add(45))
                        .read())
                            != 0)
                        {
                            gameCleared = 0i32;
                            break 'l9;
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
            SetLinkAIContestants(
                ((&raw mut categories).cast::<u8>()).read(),
                rank,
                ((gameCleared) as u32),
            );
        }
        ((&raw mut gContestLinkLeaderIndex).cast::<u8>()).write(LinkContest_GetLeaderIndex(
            (&raw mut leaderIds).cast::<u8>(),
        ));
        if ((((&raw mut gNumLinkContestPlayers).cast::<u8>()).read()) as i32) < 4i32 {
            SetTaskFuncWithFollowupFunc(
                taskId,
                Some(Task_LinkContest_CommunicateAIMonsEm),
                Some(Task_LinkContest_CalculateRound1Em),
            );
        } else {
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_LinkContest_CalculateRound1Em));
        }
    }
}
pub(crate) unsafe extern "C" fn Task_LinkContest_CalculateRound1Em(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        CalculateRound1Points(
            ((((&raw mut gSpecialVar_ContestCategory).cast::<u16>()).read()) as u8),
        );
        SetTaskFuncWithFollowupFunc(
            taskId,
            Some(Task_LinkContest_CommunicateRound1Points),
            Some(Task_LinkContest_CalculateTurnOrderEm),
        );
    }
}
pub(crate) unsafe extern "C" fn Task_LinkContest_CalculateTurnOrderEm(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        SortContestants(0u8);
        SetTaskFuncWithFollowupFunc(
            taskId,
            Some(Task_LinkContest_CommunicateTurnOrder),
            Some(Task_LinkContest_FinalizeConnection),
        );
    }
}
pub(crate) unsafe extern "C" fn Task_LinkContest_CommunicateMonsEm(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut i: i32 = 0i32;
        if !((LinkContest_TryLinkStandby(
            (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(12),
        )) != 0)
        {
            return;
        }
        'l1: {
            let __sw1 = (((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .read()) as i32);
            let __matched = __sw1 == 0i32 || __sw1 == 1i32;
            if !__matched {
                (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .write(0i16);
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(12))
                .write(0i16);
                SwitchTaskToFollowupFunc(taskId);
                break 'l1;
            }
            if __sw1 == 0i32 {
                if (IsLinkTaskFinished()) != 0 {
                    if LinkContest_SendBlock(
                        ((&raw mut gContestMons).cast::<u8>()).wrapping_offset(
                            ((((&raw mut gContestPlayerMonIndex).cast::<u8>()).read()) as i32)
                                as isize
                                * 64,
                        ),
                        64u16,
                    ) == 1u32
                    {
                        let __p2 = ((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>();
                        (__p2).write(((__p2).read()).wrapping_add(1));
                    }
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                if (LinkContest_GetBlockReceivedFromAllPlayers()) != 0 {
                    {
                        i = 0i32;
                        'l2: loop {
                            if !(i
                                < ((((&raw mut gNumLinkContestPlayers).cast::<u8>()).read())
                                    as i32))
                            {
                                break 'l2;
                            }
                            'l3: {
                                crate::c::memcpy(
                                    ((&raw mut gContestMons).cast::<u8>())
                                        .wrapping_offset((i) as isize * 64),
                                    ((((&raw mut gBlockRecvBuffer).cast::<u8>())
                                        .wrapping_offset((i) as isize * 256))
                                    .cast::<u16>())
                                    .cast::<u8>(),
                                    64u32,
                                );
                                StripPlayerAndMonNamesForLinkContest(
                                    ((&raw mut gContestMons).cast::<u8>())
                                        .wrapping_offset((i) as isize * 64),
                                    ((((((&raw mut gLinkPlayers).cast::<u8>())
                                        .wrapping_offset((i) as isize * 28))
                                    .wrapping_add(26)
                                    .cast::<u16>())
                                    .read()) as i32),
                                );
                            }
                            i = (i).wrapping_add(1);
                        }
                    }
                    let __p3 = ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>();
                    (__p3).write(((__p3).read()).wrapping_add(1));
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_LinkContest_CommunicateRngEm(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if !((LinkContest_TryLinkStandby(
            (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(12),
        )) != 0)
        {
            return;
        }
        'l1: {
            let __sw1 = (((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .read()) as i32);
            let __matched = __sw1 == 0i32 || __sw1 == 1i32;
            if !__matched {
                (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .write(0i16);
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(12))
                .write(0i16);
                SwitchTaskToFollowupFunc(taskId);
                break 'l1;
            }
            if __sw1 == 0i32 {
                if ((GetMultiplayerId()) as i32) == 0i32 {
                    if !((IsLinkTaskFinished()) != 0) {
                        return;
                    }
                    if LinkContest_SendBlock(
                        ((&raw mut gRngValue).cast::<u32>()).cast::<u8>(),
                        4u16,
                    ) == 1u32
                    {
                        let __p2 = ((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>();
                        (__p2).write(((__p2).read()).wrapping_add(1));
                    }
                } else {
                    let __p3 = ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>();
                    (__p3).write(((__p3).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                if (LinkContest_GetBlockReceived(0u8)) != 0 {
                    crate::c::memcpy(
                        ((&raw mut gRngValue).cast::<u32>()).cast::<u8>(),
                        (((&raw mut gBlockRecvBuffer).cast::<u8>()).cast::<u16>()).cast::<u8>(),
                        4u32,
                    );
                    crate::c::memcpy(
                        ((&raw mut gContestRngValue).cast::<u32>()).cast::<u8>(),
                        (((&raw mut gBlockRecvBuffer).cast::<u8>()).cast::<u16>()).cast::<u8>(),
                        4u32,
                    );
                    let __p4 = ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>();
                    (__p4).write(((__p4).read()).wrapping_add(1));
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_LinkContest_CommunicateLeaderIdsEm(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut i: i32 = 0i32;
        let mut data = crate::ffi::Align4([0u8; 8]);
        let mut leaderId: u16 = 0u16;
        if !((LinkContest_TryLinkStandby(
            (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(12),
        )) != 0)
        {
            return;
        }
        'l1: {
            let __sw1 = (((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .read()) as i32);
            let __matched = __sw1 == 0i32 || __sw1 == 1i32;
            if !__matched {
                (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .write(0i16);
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(12))
                .write(0i16);
                SwitchTaskToFollowupFunc(taskId);
                break 'l1;
            }
            if __sw1 == 0i32 {
                if (IsLinkTaskFinished()) != 0 {
                    leaderId = 110u16;
                    if LinkContest_SendBlock((&raw mut leaderId).cast::<u8>(), 2u16) == 1u32 {
                        let __p2 = ((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>();
                        (__p2).write(((__p2).read()).wrapping_add(1));
                    }
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                if (LinkContest_GetBlockReceivedFromAllPlayers()) != 0 {
                    {
                        i = 0i32;
                        'l2: loop {
                            if !(i
                                < ((((&raw mut gNumLinkContestPlayers).cast::<u8>()).read())
                                    as i32))
                            {
                                break 'l2;
                            }
                            'l3: {
                                (((&raw mut data).cast::<u16>()).wrapping_offset((i) as isize))
                                    .write(
                                        ((((&raw mut gBlockRecvBuffer).cast::<u8>())
                                            .wrapping_offset((i) as isize * 256))
                                        .cast::<u16>())
                                        .read(),
                                    );
                                ((((((&raw mut gTasks).cast::<u8>())
                                    .wrapping_offset(((taskId) as i32) as isize * 40))
                                .wrapping_add(8))
                                .cast::<i16>())
                                .wrapping_offset(((i).wrapping_add(5i32)) as isize))
                                .write(
                                    (((((&raw mut data).cast::<u16>())
                                        .wrapping_offset((i) as isize))
                                    .read()) as i16),
                                );
                            }
                            i = (i).wrapping_add(1);
                        }
                    }
                    let __p3 = ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>();
                    (__p3).write(((__p3).read()).wrapping_add(1));
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_LinkContest_CommunicateCategoryEm(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut i: i32 = 0i32;
        let mut data = crate::ffi::Align4([0u8; 8]);
        let mut category: u16 = 0u16;
        if !((LinkContest_TryLinkStandby(
            (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(12),
        )) != 0)
        {
            return;
        }
        'l1: {
            let __sw1 = (((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .read()) as i32);
            let __matched = __sw1 == 0i32 || __sw1 == 1i32;
            if !__matched {
                (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .write(0i16);
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(12))
                .write(0i16);
                SwitchTaskToFollowupFunc(taskId);
                break 'l1;
            }
            if __sw1 == 0i32 {
                if (IsLinkTaskFinished()) != 0 {
                    category = ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(9))
                    .read()) as u16);
                    if LinkContest_SendBlock((&raw mut category).cast::<u8>(), 2u16) == 1u32 {
                        let __p2 = ((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>();
                        (__p2).write(((__p2).read()).wrapping_add(1));
                    }
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                if (LinkContest_GetBlockReceivedFromAllPlayers()) != 0 {
                    {
                        i = 0i32;
                        'l2: loop {
                            if !(i
                                < ((((&raw mut gNumLinkContestPlayers).cast::<u8>()).read())
                                    as i32))
                            {
                                break 'l2;
                            }
                            'l3: {
                                (((&raw mut data).cast::<u16>()).wrapping_offset((i) as isize))
                                    .write(
                                        ((((&raw mut gBlockRecvBuffer).cast::<u8>())
                                            .wrapping_offset((i) as isize * 256))
                                        .cast::<u16>())
                                        .read(),
                                    );
                                ((((((&raw mut gTasks).cast::<u8>())
                                    .wrapping_offset(((taskId) as i32) as isize * 40))
                                .wrapping_add(8))
                                .cast::<i16>())
                                .wrapping_offset(((i).wrapping_add(1i32)) as isize))
                                .write(
                                    (((((&raw mut data).cast::<u16>())
                                        .wrapping_offset((i) as isize))
                                    .read()) as i16),
                                );
                            }
                            i = (i).wrapping_add(1);
                        }
                    }
                    let __p3 = ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>();
                    (__p3).write(((__p3).read()).wrapping_add(1));
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_LinkContest_CommunicateAIMonsEm(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut i: i32 = 0i32;
        if !((LinkContest_TryLinkStandby(
            (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(12),
        )) != 0)
        {
            return;
        }
        'l1: {
            let __sw1 = (((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .read()) as i32);
            let __matched = __sw1 == 0i32 || __sw1 == 1i32;
            if !__matched {
                (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .write(0i16);
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(12))
                .write(0i16);
                SwitchTaskToFollowupFunc(taskId);
                break 'l1;
            }
            if __sw1 == 0i32 {
                if ((GetMultiplayerId()) as i32) == 0i32 {
                    if !((IsLinkTaskFinished()) != 0) {
                        return;
                    }
                    if LinkContest_SendBlock(
                        ((&raw mut gContestMons).cast::<u8>()).wrapping_offset(
                            ((((&raw mut gNumLinkContestPlayers).cast::<u8>()).read()) as i32)
                                as isize
                                * 64,
                        ),
                        (((((4i32).wrapping_sub(
                            ((((&raw mut gNumLinkContestPlayers).cast::<u8>()).read()) as i32),
                        )) as u32)
                            .wrapping_mul(64u32)) as u16),
                    ) == 1u32
                    {
                        let __p2 = ((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>();
                        (__p2).write(((__p2).read()).wrapping_add(1));
                    }
                } else {
                    let __p3 = ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>();
                    (__p3).write(((__p3).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                if (LinkContest_GetBlockReceived(0u8)) != 0 {
                    crate::c::memcpy(
                        ((&raw mut gContestMons).cast::<u8>()).wrapping_offset(
                            ((((&raw mut gNumLinkContestPlayers).cast::<u8>()).read()) as i32)
                                as isize
                                * 64,
                        ),
                        (((&raw mut gBlockRecvBuffer).cast::<u8>()).cast::<u16>()).cast::<u8>(),
                        (((4i32).wrapping_sub(
                            ((((&raw mut gNumLinkContestPlayers).cast::<u8>()).read()) as i32),
                        )) as u32)
                            .wrapping_mul(64u32),
                    );
                    {
                        i = ((((&raw mut gNumLinkContestPlayers).cast::<u8>()).read()) as i32);
                        'l2: loop {
                            if !(i < 4i32) {
                                break 'l2;
                            }
                            'l3: {
                                StripPlayerAndMonNamesForLinkContest(
                                    ((&raw mut gContestMons).cast::<u8>())
                                        .wrapping_offset((i) as isize * 64),
                                    (((((&raw mut gLinkPlayers).cast::<u8>())
                                        .wrapping_add(26)
                                        .cast::<u16>())
                                    .read()) as i32),
                                );
                            }
                            i = (i).wrapping_add(1);
                        }
                    }
                    let __p4 = ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>();
                    (__p4).write(((__p4).read()).wrapping_add(1));
                }
                break 'l1;
            }
        }
    }
}
