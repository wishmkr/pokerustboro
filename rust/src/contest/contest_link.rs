//! Translated from `src/contest_link.c` by tools/rustport/c2rs.py, then reviewed.
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
    static mut gBlockSendBuffer: u8;
    static mut gContestFinalStandings: u8;
    static mut gContestLinkLeaderIndex: u8;
    static mut gContestMonAppealPointTotals: u8;
    static mut gContestMonRound1Points: u8;
    static mut gContestMonRound2Points: u8;
    static mut gContestMonTotalPoints: u8;
    static mut gContestMons: u8;
    static mut gContestPlayerMonIndex: u8;
    static mut gContestResources: u8;
    static mut gContestRngValue: u8;
    static mut gContestantTurnOrder: u8;
    static mut gDecompressionBuffer: u8;
    static mut gLinkContestFlags: u8;
    static mut gLinkPlayers: u8;
    static mut gNumLinkContestPlayers: u8;
    static mut gReceivedRemoteLinkPlayers: u8;
    static mut gRngValue: u8;
    static mut gTasks: u8;
    static mut gWirelessCommType: u8;
    fn BitmaskAllOtherLinkPlayers() -> u8;
    fn GetBlockReceivedStatus() -> u8;
    fn GetLinkPlayerCount() -> u8;
    fn GetLinkPlayerCountAsBitFlags() -> u8;
    fn GetMultiplayerId() -> u8;
    fn IsLinkTaskFinished() -> u8;
    fn ResetBlockReceivedFlag(a0: u8);
    fn ResetBlockReceivedFlags();
    fn SendBlock(a0: u8, a1: *mut u8, a2: u16) -> u8;
    fn SendBlockRequest(a0: u8) -> u8;
    fn SetLinkStandbyCallback();
    fn StripPlayerAndMonNamesForLinkContest(a0: *mut u8, a1: i32);
    fn SwitchTaskToFollowupFunc(a0: u8);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn LinkContest_SendBlock(src: *mut u8, size: u16) -> u32 {
    unsafe {
        let mut src = src;
        let mut size = size;
        crate::c::memcpy(
            (&raw mut gDecompressionBuffer).cast::<u8>(),
            src,
            ((size) as u32),
        );
        if (SendBlock(
            BitmaskAllOtherLinkPlayers(),
            (&raw mut gDecompressionBuffer).cast::<u8>(),
            size,
        )) != 0
        {
            return 1u32;
        } else {
            return 0u32;
        }
        #[allow(unreachable_code)]
        {
            return 0u32;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn LinkContest_GetBlockReceived(flag: u8) -> u8 {
    unsafe {
        let mut flag = flag;
        let mut mask: u8 = ((crate::c::shl_i32(1i32, ((flag) as u32))) as u8);
        if !((((GetBlockReceivedStatus()) as i32) & ((mask) as i32)) != 0) {
            return 0u8;
        } else {
            ResetBlockReceivedFlag(flag);
            return 1u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn LinkContest_GetBlockReceivedFromAllPlayers() -> u8 {
    unsafe {
        if ((GetBlockReceivedStatus()) as i32) == ((GetLinkPlayerCountAsBitFlags()) as i32) {
            ResetBlockReceivedFlags();
            return 1u8;
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Task_LinkContest_Init(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut i: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 4i32) {
                    break 'l1;
                }
                'l2: {
                    ((((&raw mut gBlockRecvBuffer).cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 256))
                    .cast::<u16>())
                    .write(255u16);
                }
                i = (i).wrapping_add(1);
            }
        }
        (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .write(0i16);
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Task_LinkContest_StartInitFlags));
    }
}
pub(crate) unsafe extern "C" fn Task_LinkContest_StartInitFlags(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Task_LinkContest_InitFlags));
    }
}
pub(crate) unsafe extern "C" fn Task_LinkContest_InitFlags(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut i: i32 = 0i32;
        if !((((&raw mut gReceivedRemoteLinkPlayers).cast::<u8>()).read()) != 0) {
            return;
        }
        ((&raw mut gContestPlayerMonIndex).cast::<u8>()).write(GetMultiplayerId());
        ((&raw mut gNumLinkContestPlayers).cast::<u8>()).write(GetLinkPlayerCount());
        ((&raw mut gLinkContestFlags).cast::<u8>()).write(1u8);
        if ((((&raw mut gWirelessCommType).cast::<u8>()).read()) as i32) == 1i32 {
            ((&raw mut gLinkContestFlags).cast::<u8>()).write(3u8);
        }
        {
            i = 0i32;
            'l1: loop {
                if !((i < ((((&raw mut gNumLinkContestPlayers).cast::<u8>()).read()) as i32))
                    && (((((((((&raw mut gLinkPlayers).cast::<u8>())
                        .wrapping_offset((i) as isize * 28))
                    .cast::<u16>())
                    .read()) as i32)
                        & 255i32) as u32)
                        .wrapping_sub(1u32)
                        > 1u32))
                {
                    break 'l1;
                }
                'l2: {}
                i = (i).wrapping_add(1);
            }
        }
        if i < ((((&raw mut gNumLinkContestPlayers).cast::<u8>()).read()) as i32) {
            let __p1 = (&raw mut gLinkContestFlags).cast::<u8>();
            (__p1).write((((((__p1).read()) as i32) | 4i32) as u8));
        }
        SwitchTaskToFollowupFunc(taskId);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn LinkContest_TryLinkStandby(state: *mut i16) -> u32 {
    unsafe {
        let mut state = state;
        if (((((&raw mut gLinkContestFlags).cast::<u8>()).read()) as i32) & 4i32) != 0 {
            return 1u32;
        }
        'l1: {
            let __sw1 = (((state).read()) as i32);
            let __matched = __sw1 == 0i32 || __sw1 == 1i32;
            if __sw1 == 0i32 {
                if (IsLinkTaskFinished()) != 0 {
                    SetLinkStandbyCallback();
                    (state).write(((state).read()).wrapping_add(1));
                }
                return 0u32;
            }
            if __sw1 == 1i32 {
                (state).write(((state).read()).wrapping_add(1));
                return 0u32;
            }
            if !__matched {
                if ((IsLinkTaskFinished()) as i32) != 1i32 {
                    return 0u32;
                } else {
                    return 1u32;
                }
            }
        }
        #[allow(unreachable_code)]
        {
            return 0u32;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Task_LinkContest_CommunicateMonsRS(taskId: u8) {
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
            let __matched = __sw1 == 0i32 || __sw1 == 1i32 || __sw1 == 10i32;
            if __sw1 == 0i32 {
                if ((GetMultiplayerId()) as i32) == 0i32 {
                    if (IsLinkTaskFinished()) != 0 {
                        crate::c::memcpy(
                            (&raw mut gBlockSendBuffer).cast::<u8>(),
                            ((&raw mut gContestMons).cast::<u8>()).wrapping_offset(
                                ((((&raw mut gContestPlayerMonIndex).cast::<u8>()).read()) as i32)
                                    as isize
                                    * 64,
                            ),
                            64u32,
                        );
                        (((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .write(10i16);
                    }
                } else {
                    crate::c::memcpy(
                        (&raw mut gBlockSendBuffer).cast::<u8>(),
                        ((&raw mut gContestMons).cast::<u8>()).wrapping_offset(
                            ((((&raw mut gContestPlayerMonIndex).cast::<u8>()).read()) as i32)
                                as isize
                                * 64,
                        ),
                        64u32,
                    );
                    (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .write(1i16);
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
                    let __p2 = ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>();
                    (__p2).write(((__p2).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 10i32 {
                if (({
                    let __p3 = (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(11);
                    let __t4 = ((__p3).read()).wrapping_add(1);
                    (__p3).write(__t4);
                    __t4
                }) as i32)
                    > 300i32
                {
                    SendBlockRequest(2u8);
                    (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .write(1i16);
                }
                break 'l1;
            }
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
                .wrapping_offset(11))
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
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Task_LinkContest_CommunicateRngRS(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        'l1: {
            let __sw1 = (((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .read()) as i32);
            let __matched = __sw1 == 0i32 || __sw1 == 1i32;
            if __sw1 == 0i32 {
                if ((GetMultiplayerId()) as i32) == 0i32 {
                    if ((IsLinkTaskFinished()) != 0)
                        && (LinkContest_SendBlock(
                            ((&raw mut gRngValue).cast::<u32>()).cast::<u8>(),
                            4u16,
                        ) == 1u32)
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
            if !__matched {
                (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .write(0i16);
                SwitchTaskToFollowupFunc(taskId);
                break 'l1;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Task_LinkContest_CommunicateCategoryRS(taskId: u8) {
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
            let __matched = __sw1 == 0i32 || __sw1 == 1i32 || __sw1 == 10i32;
            if __sw1 == 0i32 {
                ((&raw mut gBlockSendBuffer).cast::<u8>()).write(
                    ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(9))
                    .read()) as u8),
                );
                if ((GetMultiplayerId()) as i32) == 0i32 {
                    if (IsLinkTaskFinished()) != 0 {
                        (((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .write(10i16);
                    }
                } else {
                    let __p2 = ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>();
                    (__p2).write(((__p2).read()).wrapping_add(1));
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
                                ((((((&raw mut gTasks).cast::<u8>())
                                    .wrapping_offset(((taskId) as i32) as isize * 40))
                                .wrapping_add(8))
                                .cast::<i16>())
                                .wrapping_offset(((i).wrapping_add(1i32)) as isize))
                                .write(
                                    ((((((&raw mut gBlockRecvBuffer).cast::<u8>())
                                        .wrapping_offset((i) as isize * 256))
                                    .cast::<u16>())
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
            if __sw1 == 10i32 {
                if (({
                    let __p4 = (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(11);
                    let __t5 = ((__p4).read()).wrapping_add(1);
                    (__p4).write(__t5);
                    __t5
                }) as i32)
                    > 10i32
                {
                    SendBlockRequest(2u8);
                    (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .write(1i16);
                }
                break 'l1;
            }
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
                .wrapping_offset(11))
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
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Task_LinkContest_CommunicateMonIdxs(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        'l1: {
            let __sw1 = (((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .read()) as i32);
            let __matched = __sw1 == 0i32 || __sw1 == 1i32;
            if __sw1 == 0i32 {
                if (IsLinkTaskFinished()) != 0 {
                    if LinkContest_SendBlock((&raw mut gContestPlayerMonIndex).cast::<u8>(), 1u16)
                        == 1u32
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
                    let __p3 = ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>();
                    (__p3).write(((__p3).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if !__matched {
                (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .write(0i16);
                SwitchTaskToFollowupFunc(taskId);
                break 'l1;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Task_LinkContest_CommunicateMoveSelections(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut i: i32 = 0i32;
        'l1: {
            let __sw1 = (((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .read()) as i32);
            let __matched = __sw1 == 0i32 || __sw1 == 1i32;
            if __sw1 == 0i32 {
                if (IsLinkTaskFinished()) != 0 {
                    if LinkContest_SendBlock(
                        (((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                            .wrapping_add(4)
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_offset(
                            ((((&raw mut gContestPlayerMonIndex).cast::<u8>()).read()) as i32)
                                as isize
                                * 28,
                        ))
                        .wrapping_add(6)
                        .cast::<u16>())
                        .cast::<u8>(),
                        2u16,
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
                                (((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                                    .wrapping_add(4)
                                    .cast::<*mut u8>())
                                .read())
                                .wrapping_offset((i) as isize * 28))
                                .wrapping_add(6)
                                .cast::<u16>())
                                .write(
                                    ((((&raw mut gBlockRecvBuffer).cast::<u8>())
                                        .wrapping_offset((i) as isize * 256))
                                    .cast::<u16>())
                                    .read(),
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
            if !__matched {
                (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .write(0i16);
                SwitchTaskToFollowupFunc(taskId);
                break 'l1;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Task_LinkContest_CommunicateFinalStandings(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        'l1: {
            let __sw1 = (((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .read()) as i32);
            let __matched = __sw1 == 0i32
                || __sw1 == 1i32
                || __sw1 == 2i32
                || __sw1 == 5i32
                || __sw1 == 8i32
                || __sw1 == 11i32
                || __sw1 == 3i32
                || __sw1 == 4i32
                || __sw1 == 6i32
                || __sw1 == 7i32
                || __sw1 == 9i32
                || __sw1 == 10i32;
            if __sw1 == 0i32 {
                if (IsLinkTaskFinished()) != 0 {
                    if LinkContest_SendBlock(
                        (((&raw mut gContestMonTotalPoints).cast::<i16>()).cast::<i16>())
                            .cast::<u8>(),
                        8u16,
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
                    crate::c::memcpy(
                        (((&raw mut gContestMonTotalPoints).cast::<i16>()).cast::<i16>())
                            .cast::<u8>(),
                        ((((&raw mut gBlockRecvBuffer).cast::<u8>()).wrapping_offset(
                            ((((&raw mut gContestLinkLeaderIndex).cast::<u8>()).read()) as i32)
                                as isize
                                * 256,
                        ))
                        .cast::<u16>())
                        .cast::<u8>(),
                        8u32,
                    );
                    let __p3 = ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>();
                    (__p3).write(((__p3).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 2i32 || __sw1 == 5i32 || __sw1 == 8i32 || __sw1 == 11i32 {
                if (({
                    let __p4 = (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(1);
                    let __t5 = (__p4).read();
                    (__p4).write(((__p4).read()).wrapping_add(1));
                    __t5
                }) as i32)
                    > 10i32
                {
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(1))
                    .write(0i16);
                    let __p6 = ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>();
                    (__p6).write(((__p6).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                if (IsLinkTaskFinished()) != 0 {
                    if LinkContest_SendBlock(
                        (((&raw mut gContestMonAppealPointTotals).cast::<i16>()).cast::<i16>())
                            .cast::<u8>(),
                        8u16,
                    ) == 1u32
                    {
                        let __p7 = ((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>();
                        (__p7).write(((__p7).read()).wrapping_add(1));
                    }
                }
                break 'l1;
            }
            if __sw1 == 4i32 {
                if (LinkContest_GetBlockReceivedFromAllPlayers()) != 0 {
                    crate::c::memcpy(
                        (((&raw mut gContestMonAppealPointTotals).cast::<i16>()).cast::<i16>())
                            .cast::<u8>(),
                        ((((&raw mut gBlockRecvBuffer).cast::<u8>()).wrapping_offset(
                            ((((&raw mut gContestLinkLeaderIndex).cast::<u8>()).read()) as i32)
                                as isize
                                * 256,
                        ))
                        .cast::<u16>())
                        .cast::<u8>(),
                        8u32,
                    );
                    let __p8 = ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>();
                    (__p8).write(((__p8).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 6i32 {
                if (IsLinkTaskFinished()) != 0 {
                    if LinkContest_SendBlock(
                        (((&raw mut gContestMonRound2Points).cast::<i16>()).cast::<i16>())
                            .cast::<u8>(),
                        8u16,
                    ) == 1u32
                    {
                        let __p9 = ((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>();
                        (__p9).write(((__p9).read()).wrapping_add(1));
                    }
                }
                break 'l1;
            }
            if __sw1 == 7i32 {
                if (LinkContest_GetBlockReceivedFromAllPlayers()) != 0 {
                    crate::c::memcpy(
                        (((&raw mut gContestMonRound2Points).cast::<i16>()).cast::<i16>())
                            .cast::<u8>(),
                        ((((&raw mut gBlockRecvBuffer).cast::<u8>()).wrapping_offset(
                            ((((&raw mut gContestLinkLeaderIndex).cast::<u8>()).read()) as i32)
                                as isize
                                * 256,
                        ))
                        .cast::<u16>())
                        .cast::<u8>(),
                        8u32,
                    );
                    let __p10 = ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>();
                    (__p10).write(((__p10).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 9i32 {
                if (IsLinkTaskFinished()) != 0 {
                    if LinkContest_SendBlock((&raw mut gContestFinalStandings).cast::<u8>(), 4u16)
                        == 1u32
                    {
                        let __p11 = ((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>();
                        (__p11).write(((__p11).read()).wrapping_add(1));
                    }
                }
                break 'l1;
            }
            if __sw1 == 10i32 {
                if (LinkContest_GetBlockReceivedFromAllPlayers()) != 0 {
                    crate::c::memcpy(
                        (&raw mut gContestFinalStandings).cast::<u8>(),
                        ((((&raw mut gBlockRecvBuffer).cast::<u8>()).wrapping_offset(
                            ((((&raw mut gContestLinkLeaderIndex).cast::<u8>()).read()) as i32)
                                as isize
                                * 256,
                        ))
                        .cast::<u16>())
                        .cast::<u8>(),
                        4u32,
                    );
                    let __p12 = ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>();
                    (__p12).write(((__p12).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if !__matched {
                (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .write(0i16);
                SwitchTaskToFollowupFunc(taskId);
                break 'l1;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Task_LinkContest_CommunicateAppealsState(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        'l1: {
            let __sw1 = (((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .read()) as i32);
            let __matched = __sw1 == 0i32
                || __sw1 == 1i32
                || __sw1 == 2i32
                || __sw1 == 5i32
                || __sw1 == 8i32
                || __sw1 == 11i32
                || __sw1 == 3i32
                || __sw1 == 4i32
                || __sw1 == 6i32
                || __sw1 == 7i32
                || __sw1 == 9i32
                || __sw1 == 10i32;
            if __sw1 == 0i32 {
                if (IsLinkTaskFinished()) != 0 {
                    if LinkContest_SendBlock(
                        ((((&raw mut gContestResources).cast::<*mut u8>()).read())
                            .wrapping_add(4)
                            .cast::<*mut u8>())
                        .read(),
                        112u16,
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
                    crate::c::memcpy(
                        ((((&raw mut gContestResources).cast::<*mut u8>()).read())
                            .wrapping_add(4)
                            .cast::<*mut u8>())
                        .read(),
                        ((((&raw mut gBlockRecvBuffer).cast::<u8>()).wrapping_offset(
                            ((((&raw mut gContestLinkLeaderIndex).cast::<u8>()).read()) as i32)
                                as isize
                                * 256,
                        ))
                        .cast::<u16>())
                        .cast::<u8>(),
                        112u32,
                    );
                    let __p3 = ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>();
                    (__p3).write(((__p3).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 2i32 || __sw1 == 5i32 || __sw1 == 8i32 || __sw1 == 11i32 {
                if (({
                    let __p4 = (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(1);
                    let __t5 = (__p4).read();
                    (__p4).write(((__p4).read()).wrapping_add(1));
                    __t5
                }) as i32)
                    > 10i32
                {
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(1))
                    .write(0i16);
                    let __p6 = ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>();
                    (__p6).write(((__p6).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                if (IsLinkTaskFinished()) != 0 {
                    if LinkContest_SendBlock(
                        ((((&raw mut gContestResources).cast::<*mut u8>()).read())
                            .wrapping_add(8)
                            .cast::<*mut u8>())
                        .read(),
                        20u16,
                    ) == 1u32
                    {
                        let __p7 = ((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>();
                        (__p7).write(((__p7).read()).wrapping_add(1));
                    }
                }
                break 'l1;
            }
            if __sw1 == 4i32 {
                if (LinkContest_GetBlockReceivedFromAllPlayers()) != 0 {
                    crate::c::memcpy(
                        ((((&raw mut gContestResources).cast::<*mut u8>()).read())
                            .wrapping_add(8)
                            .cast::<*mut u8>())
                        .read(),
                        ((((&raw mut gBlockRecvBuffer).cast::<u8>()).wrapping_offset(
                            ((((&raw mut gContestLinkLeaderIndex).cast::<u8>()).read()) as i32)
                                as isize
                                * 256,
                        ))
                        .cast::<u16>())
                        .cast::<u8>(),
                        20u32,
                    );
                    let __p8 = ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>();
                    (__p8).write(((__p8).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 6i32 {
                if (IsLinkTaskFinished()) != 0 {
                    if LinkContest_SendBlock(
                        ((((&raw mut gContestResources).cast::<*mut u8>()).read())
                            .wrapping_add(16)
                            .cast::<*mut u8>())
                        .read(),
                        4u16,
                    ) == 1u32
                    {
                        let __p9 = ((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>();
                        (__p9).write(((__p9).read()).wrapping_add(1));
                    }
                }
                break 'l1;
            }
            if __sw1 == 7i32 {
                if (LinkContest_GetBlockReceivedFromAllPlayers()) != 0 {
                    crate::c::memcpy(
                        ((((&raw mut gContestResources).cast::<*mut u8>()).read())
                            .wrapping_add(16)
                            .cast::<*mut u8>())
                        .read(),
                        ((((&raw mut gBlockRecvBuffer).cast::<u8>()).wrapping_offset(
                            ((((&raw mut gContestLinkLeaderIndex).cast::<u8>()).read()) as i32)
                                as isize
                                * 256,
                        ))
                        .cast::<u16>())
                        .cast::<u8>(),
                        4u32,
                    );
                    let __p10 = ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>();
                    (__p10).write(((__p10).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 9i32 {
                if (IsLinkTaskFinished()) != 0 {
                    if LinkContest_SendBlock((&raw mut gContestantTurnOrder).cast::<u8>(), 4u16)
                        == 1u32
                    {
                        let __p11 = ((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>();
                        (__p11).write(((__p11).read()).wrapping_add(1));
                    }
                }
                break 'l1;
            }
            if __sw1 == 10i32 {
                if (LinkContest_GetBlockReceivedFromAllPlayers()) != 0 {
                    crate::c::memcpy(
                        (&raw mut gContestantTurnOrder).cast::<u8>(),
                        ((((&raw mut gBlockRecvBuffer).cast::<u8>()).wrapping_offset(
                            ((((&raw mut gContestLinkLeaderIndex).cast::<u8>()).read()) as i32)
                                as isize
                                * 256,
                        ))
                        .cast::<u16>())
                        .cast::<u8>(),
                        4u32,
                    );
                    let __p12 = ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>();
                    (__p12).write(((__p12).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if !__matched {
                (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .write(0i16);
                SwitchTaskToFollowupFunc(taskId);
                break 'l1;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Task_LinkContest_CommunicateLeaderIdsRS(taskId: u8) {
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
            let __matched = __sw1 == 0i32 || __sw1 == 1i32 || __sw1 == 10i32;
            if __sw1 == 0i32 {
                ((&raw mut gBlockSendBuffer).cast::<u8>()).write(110u8);
                if ((GetMultiplayerId()) as i32) == 0i32 {
                    if (IsLinkTaskFinished()) != 0 {
                        (((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .write(10i16);
                    }
                } else {
                    let __p2 = ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>();
                    (__p2).write(((__p2).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                if (LinkContest_GetBlockReceivedFromAllPlayers()) != 0 {
                    {
                        i = 0i32;
                        'l2: loop {
                            if !(i < 4i32) {
                                break 'l2;
                            }
                            'l3: {
                                ((((((&raw mut gTasks).cast::<u8>())
                                    .wrapping_offset(((taskId) as i32) as isize * 40))
                                .wrapping_add(8))
                                .cast::<i16>())
                                .wrapping_offset(((i).wrapping_add(5i32)) as isize))
                                .write(
                                    ((((((&raw mut gBlockRecvBuffer).cast::<u8>())
                                        .wrapping_offset((i) as isize * 256))
                                    .cast::<u16>())
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
            if __sw1 == 10i32 {
                if (({
                    let __p4 = (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(11);
                    let __t5 = ((__p4).read()).wrapping_add(1);
                    (__p4).write(__t5);
                    __t5
                }) as i32)
                    > 10i32
                {
                    SendBlockRequest(2u8);
                    (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .write(1i16);
                }
                break 'l1;
            }
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
                .wrapping_offset(11))
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
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Task_LinkContest_CommunicateRound1Points(taskId: u8) {
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
            if __sw1 == 0i32 {
                if (IsLinkTaskFinished()) != 0 {
                    if LinkContest_SendBlock(
                        (((&raw mut gContestMonRound1Points).cast::<i16>()).cast::<i16>())
                            .cast::<u8>(),
                        8u16,
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
                    crate::c::memcpy(
                        (((&raw mut gContestMonRound1Points).cast::<i16>()).cast::<i16>())
                            .cast::<u8>(),
                        ((((&raw mut gBlockRecvBuffer).cast::<u8>()).wrapping_offset(
                            ((((&raw mut gContestLinkLeaderIndex).cast::<u8>()).read()) as i32)
                                as isize
                                * 256,
                        ))
                        .cast::<u16>())
                        .cast::<u8>(),
                        8u32,
                    );
                    let __p3 = ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>();
                    (__p3).write(((__p3).read()).wrapping_add(1));
                }
                break 'l1;
            }
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
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Task_LinkContest_CommunicateTurnOrder(taskId: u8) {
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
            if __sw1 == 0i32 {
                if (IsLinkTaskFinished()) != 0 {
                    if LinkContest_SendBlock((&raw mut gContestantTurnOrder).cast::<u8>(), 4u16)
                        == 1u32
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
                    crate::c::memcpy(
                        (&raw mut gContestantTurnOrder).cast::<u8>(),
                        ((((&raw mut gBlockRecvBuffer).cast::<u8>()).wrapping_offset(
                            ((((&raw mut gContestLinkLeaderIndex).cast::<u8>()).read()) as i32)
                                as isize
                                * 256,
                        ))
                        .cast::<u16>())
                        .cast::<u8>(),
                        4u32,
                    );
                    let __p3 = ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>();
                    (__p3).write(((__p3).read()).wrapping_add(1));
                }
                break 'l1;
            }
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
        }
    }
}
