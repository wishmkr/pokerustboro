//! Translated from `src/link_rfu_2.c` by tools/rustport/c2rs.py, then reviewed.
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

// Data tables (translate with cdata.py): sRfuReqConfigTemplate sAvailSlots sAllBlocksReceived sSlotToLinkPlayerTableId sPlayerBitsToCount sPlayerBitsToNewChildIdx sBlockRequests sAcceptedSerialNos sASCII_RfuCmds sASCII_RecoverCmds sShutdownTasks sASCII_PokemonSioInfo sASCII_LinkLossDisconnect sASCII_LinkLossRecoveryNow sASCII_30Spaces sASCII_15Spaces sASCII_8Spaces sASCII_Space sASCII_Asterisk sASCII_NowSlot sASCII_ClockCmds sASCII_ChildParentSearch
#[allow(unused_imports)]
use crate::data::link_rfu_2::*;

#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gRfuAPIBuffer: crate::ffi::Align4<[u8; 3684]> = crate::ffi::Align4([0; 3684]);
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gRfu: crate::ffi::Align4<[u8; 3316]> = crate::ffi::Align4([0; 3316]);
pub(crate) static mut sHeldKeyCount: u8 = 0u8;
pub(crate) static mut sResendBlock8: crate::ffi::Align4<[u8; 16]> = crate::ffi::Align4([0; 16]);
pub(crate) static mut sResendBlock16: crate::ffi::Align4<[u8; 16]> = crate::ffi::Align4([0; 16]);
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gHostRfuGameData: crate::ffi::Align4<[u8; 14]> = crate::ffi::Align4([0; 14]);
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gHostRfuUsername: crate::ffi::Align4<[u8; 8]> = crate::ffi::Align4([0; 8]);
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sRfuReqConfig: crate::ffi::Align4<[u8; 24]> = crate::ffi::Align4([0; 24]);
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sRfuDebug: crate::ffi::Align4<[u8; 220]> = crate::ffi::Align4([0; 220]);

unsafe extern "C" {
    static mut gBattleTypeFlags: u8;
    static mut gBerryBlenderKeySendAttempts: u8;
    static mut gBlockRecvBuffer: u8;
    static mut gBlockSendBuffer: u8;
    static mut gHeldKeyCodeToSend: u8;
    static mut gIntrTable: u8;
    static mut gLinkPartnersHeldKeys: u8;
    static mut gLinkPlayers: u8;
    static mut gLinkTransferringData: u8;
    static mut gLinkType: u8;
    static mut gMain: u8;
    static mut gReceivedRemoteLinkPlayers: u8;
    static mut gRecvCmds: u8;
    static mut gRfuLinkStatus: u8;
    static mut gRfuSlotStatusNI: u8;
    static mut gRfuSlotStatusUNI: u8;
    static mut gSaveBlock2Ptr: u8;
    static mut gSendCmd: u8;
    static mut gTasks: u8;
    static mut gWirelessCommType: u8;
    static mut lman: u8;
    fn AnimateSprites();
    fn BuildOamBuffer();
    fn CB2_LinkError();
    fn CB2_MysteryGiftEReader();
    fn ClearSavedLinkPlayers();
    fn CloseLink();
    fn ConvertLinkPlayerName(a0: *mut u8);
    fn CpuSet(a0: *mut u8, a1: *mut u8, a2: u32);
    fn CreateTask(a0: Option<unsafe extern "C" fn(u8)>, a1: u8) -> u8;
    fn DestroyTask(a0: u8);
    fn FindTaskIdByFunc(a0: Option<unsafe extern "C" fn(u8)>) -> u8;
    fn FreeAllSpritePalettes();
    fn FuncIsActiveTask(a0: Option<unsafe extern "C" fn(u8)>) -> u8;
    fn GetBlenderArrowPosition() -> u16;
    fn GetBlockReceivedStatus() -> u8;
    fn GetLinkPlayerCount() -> u8;
    fn GetMultiplayerId() -> u8;
    fn InitHostRfuGameData(a0: *mut u8, a1: u8, a2: u32, a3: i32);
    fn IsLinkTaskFinished() -> u8;
    fn IsWirelessAdapterConnected() -> u8;
    fn LinkPlayerFromBlock(a0: u32);
    fn LoadOam();
    fn LocalLinkPlayerToBlock();
    fn OpenLink();
    fn ProcessSpriteCopyRequests();
    fn Random() -> u16;
    fn Random2() -> u16;
    fn ResetBlockReceivedFlag(a0: u8);
    fn ResetBlockReceivedFlags();
    fn ResetPaletteFade();
    fn ResetSpriteData();
    fn ResetTasks();
    fn RfuBackupQueue_Dequeue(a0: *mut u8, a1: *mut u8) -> u8;
    fn RfuBackupQueue_Enqueue(a0: *mut u8, a1: *mut u8);
    fn RfuRecvQueue_Dequeue(a0: *mut u8, a1: *mut u8) -> u8;
    fn RfuRecvQueue_Enqueue(a0: *mut u8, a1: *mut u8);
    fn RfuRecvQueue_Reset(a0: *mut u8);
    fn RfuSendQueue_Dequeue(a0: *mut u8, a1: *mut u8) -> u8;
    fn RfuSendQueue_Enqueue(a0: *mut u8, a1: *mut u8);
    fn RfuSendQueue_Reset(a0: *mut u8);
    fn RunTasks();
    fn SeedRng(a0: u16);
    fn SendBlock(a0: u8, a1: *mut u8, a2: u16) -> u8;
    fn SetGpuReg(a0: u8, a1: u16);
    fn SetLinkErrorBuffer(a0: u32, a1: u8, a2: u8, a3: u8);
    fn SetMainCallback2(a0: Option<unsafe extern "C" fn()>);
    fn SetVBlankCallback(a0: Option<unsafe extern "C" fn()>);
    fn SetWirelessCommType1();
    fn StringCompare(a0: *mut u8, a1: *mut u8) -> i32;
    fn StringCopy(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn TransferPlttBuffer();
    fn UpdatePaletteFade() -> u8;
    fn rfu_LMAN_CHILD_connectParent(a0: u16, a1: u16) -> u8;
    fn rfu_LMAN_REQ_sendData(a0: u8);
    fn rfu_LMAN_establishConnection(a0: u8, a1: u16, a2: u16, a3: *mut u16) -> u8;
    fn rfu_LMAN_forceChangeSP();
    fn rfu_LMAN_initializeManager(
        a0: Option<unsafe extern "C" fn(u8, u8)>,
        a1: Option<unsafe extern "C" fn(u16)>,
    ) -> u8;
    fn rfu_LMAN_initializeRFU(a0: *mut u8);
    fn rfu_LMAN_manager_entity(a0: u32);
    fn rfu_LMAN_powerDownRFU();
    fn rfu_LMAN_requestChangeAgbClockMaster();
    fn rfu_LMAN_setLinkRecovery(a0: u8, a1: u16) -> u8;
    fn rfu_LMAN_setMSCCallback(a0: Option<unsafe extern "C" fn(u16)>);
    fn rfu_LMAN_stopManager(a0: u8);
    fn rfu_LMAN_syncVBlank();
    fn rfu_NI_setSendData(a0: u8, a1: u8, a2: *mut u8, a3: u32) -> u16;
    fn rfu_REQ_PARENT_resumeRetransmitAndChange();
    fn rfu_REQ_configGameData(a0: u8, a1: u16, a2: *mut u8, a3: *mut u8);
    fn rfu_REQ_disconnect(a0: u8);
    fn rfu_REQ_recvData();
    fn rfu_REQ_stopMode();
    fn rfu_UNI_clearRecvNewDataFlag(a0: u8);
    fn rfu_UNI_readySendData(a0: u8);
    fn rfu_UNI_setSendData(a0: u8, a1: *mut u8, a2: u8) -> u16;
    fn rfu_clearAllSlot();
    fn rfu_clearSlot(a0: u8, a1: u8) -> u16;
    fn rfu_initializeAPI(
        a0: *mut u32,
        a1: u16,
        a2: *mut Option<unsafe extern "C" fn()>,
        a3: u8,
    ) -> u16;
    fn rfu_setRecvBuffer(a0: u8, a1: u8, a2: *mut u8, a3: u32) -> u16;
    fn rfu_setTimerInterrupt(a0: u8, a1: *mut Option<unsafe extern "C" fn()>);
    fn rfu_waitREQComplete() -> u16;
}

pub(crate) unsafe extern "C" fn Debug_PrintString(str: *mut u8, x: u8, y: u8) {
    unsafe {
        let mut str = str;
        let mut x = x;
        let mut y = y;
    }
}
pub(crate) unsafe extern "C" fn Debug_PrintNum(num: u16, x: u8, y: u8, numDigits: u8) {
    unsafe {
        let mut num = num;
        let mut x = x;
        let mut y = y;
        let mut numDigits = numDigits;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ResetLinkRfuGFLayer() {
    unsafe {
        let mut i: i32 = 0i32;
        let mut errorState: u8 = (((&raw mut gRfu).cast::<u8>()).wrapping_add(238)).read_volatile();
        'l1: loop {
            'l2: {
                {
                    let mut tmp: u16 = 0u16;
                    (&raw mut tmp).write_volatile(0u16);
                    'l3: loop {
                        'l4: {
                            CpuSet(
                                (&raw mut tmp).cast::<u8>(),
                                (&raw mut gRfu).cast::<u8>(),
                                (16777216u32
                                    | (crate::c::div_u32(
                                        3316u32,
                                        ((crate::c::div_i32(16i32, 8i32)) as u32),
                                    ) & 2097151u32)),
                            );
                        }
                        if !((0i32) != 0) {
                            break 'l3;
                        }
                    }
                }
            }
            if !((0i32) != 0) {
                break 'l1;
            }
        }
        crate::c::volatile_write(((&raw mut gRfu).cast::<u8>()).wrapping_add(238), errorState);
        (((&raw mut gRfu).cast::<u8>()).wrapping_add(12)).write(255u8);
        if (((((&raw mut gRfu).cast::<u8>()).wrapping_add(238)).read_volatile()) as i32) != 4i32 {
            crate::c::volatile_write(((&raw mut gRfu).cast::<u8>()).wrapping_add(238), 0u8);
        }
        {
            i = 0i32;
            'l5: loop {
                if !(i < 5i32) {
                    break 'l5;
                }
                'l6: {
                    ResetSendDataManager(
                        ((((&raw mut gRfu).cast::<u8>()).wrapping_add(128)).cast::<u8>())
                            .wrapping_offset((i) as isize * 20),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        ResetSendDataManager(((&raw mut gRfu).cast::<u8>()).wrapping_add(108));
        RfuRecvQueue_Reset(((&raw mut gRfu).cast::<u8>()).wrapping_add(292));
        RfuSendQueue_Reset(((&raw mut gRfu).cast::<u8>()).wrapping_add(2536));
        'l7: loop {
            'l8: {
                {
                    let mut tmp: u16 = 0u16;
                    (&raw mut tmp).write_volatile(0u16);
                    'l9: loop {
                        'l10: {
                            CpuSet(
                                (&raw mut tmp).cast::<u8>(),
                                (((&raw mut gSendCmd).cast::<u16>()).cast::<u16>()).cast::<u8>(),
                                (16777216u32
                                    | (crate::c::div_u32(
                                        16u32,
                                        ((crate::c::div_i32(16i32, 8i32)) as u32),
                                    ) & 2097151u32)),
                            );
                        }
                        if !((0i32) != 0) {
                            break 'l9;
                        }
                    }
                }
            }
            if !((0i32) != 0) {
                break 'l7;
            }
        }
        'l11: loop {
            'l12: {
                {
                    let mut tmp: u16 = 0u16;
                    (&raw mut tmp).write_volatile(0u16);
                    'l13: loop {
                        'l14: {
                            CpuSet(
                                (&raw mut tmp).cast::<u8>(),
                                (&raw mut gRecvCmds).cast::<u8>(),
                                (16777216u32
                                    | (crate::c::div_u32(
                                        80u32,
                                        ((crate::c::div_i32(16i32, 8i32)) as u32),
                                    ) & 2097151u32)),
                            );
                        }
                        if !((0i32) != 0) {
                            break 'l13;
                        }
                    }
                }
            }
            if !((0i32) != 0) {
                break 'l11;
            }
        }
        'l15: loop {
            'l16: {
                {
                    let mut tmp: u16 = 0u16;
                    (&raw mut tmp).write_volatile(0u16);
                    'l17: loop {
                        'l18: {
                            CpuSet(
                                (&raw mut tmp).cast::<u8>(),
                                (&raw mut gLinkPlayers).cast::<u8>(),
                                (16777216u32
                                    | (crate::c::div_u32(
                                        140u32,
                                        ((crate::c::div_i32(16i32, 8i32)) as u32),
                                    ) & 2097151u32)),
                            );
                        }
                        if !((0i32) != 0) {
                            break 'l17;
                        }
                    }
                }
            }
            if !((0i32) != 0) {
                break 'l15;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn InitRFU() {
    unsafe {
        let mut serialIntr: Option<unsafe extern "C" fn()> = ((((&raw mut gIntrTable)
            .cast::<Option<unsafe extern "C" fn()>>())
        .cast::<Option<unsafe extern "C" fn()>>())
        .wrapping_offset(1))
        .read();
        let mut timerIntr: Option<unsafe extern "C" fn()> = ((((&raw mut gIntrTable)
            .cast::<Option<unsafe extern "C" fn()>>())
        .cast::<Option<unsafe extern "C" fn()>>())
        .wrapping_offset(2))
        .read();
        InitRFUAPI();
        rfu_REQ_stopMode();
        rfu_waitREQComplete();
        crate::c::volatile_write(((67109384i32) as usize as *mut u16), 0u16);
        ((((&raw mut gIntrTable).cast::<Option<unsafe extern "C" fn()>>())
            .cast::<Option<unsafe extern "C" fn()>>())
        .wrapping_offset(1))
        .write(serialIntr);
        ((((&raw mut gIntrTable).cast::<Option<unsafe extern "C" fn()>>())
            .cast::<Option<unsafe extern "C" fn()>>())
        .wrapping_offset(2))
        .write(timerIntr);
        crate::c::volatile_write(((67109384i32) as usize as *mut u16), 1u16);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn InitRFUAPI() {
    unsafe {
        if !((rfu_initializeAPI(
            ((((&raw mut gRfuAPIBuffer).cast::<u8>().cast::<u32>()).cast::<u32>()).cast::<u8>())
                .cast::<u32>(),
            3684u16,
            (((&raw mut gIntrTable).cast::<Option<unsafe extern "C" fn()>>())
                .cast::<Option<unsafe extern "C" fn()>>())
            .wrapping_offset(1),
            1u8,
        )) != 0)
        {
            ((&raw mut gLinkType).cast::<u16>()).write(0u16);
            ClearSavedLinkPlayers();
            RfuSetIgnoreError(0u32);
            ResetLinkRfuGFLayer();
            rfu_setTimerInterrupt(
                3u8,
                (((&raw mut gIntrTable).cast::<Option<unsafe extern "C" fn()>>())
                    .cast::<Option<unsafe extern "C" fn()>>())
                .wrapping_offset(2),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn Task_ParentSearchForChildren(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        UpdateChildStatuses();
        'l1: {
            let __sw1 =
                (((((&raw mut gRfu).cast::<u8>()).wrapping_add(4).cast::<u16>()).read()) as i32);
            if __sw1 == 0i32 {
                rfu_LMAN_initializeRFU((&raw mut sRfuReqConfig).cast::<u8>());
                (((&raw mut gRfu).cast::<u8>()).wrapping_add(4).cast::<u16>()).write(1u16);
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(1))
                .write(1i16);
                break 'l1;
            }
            if __sw1 == 1i32 {
                break 'l1;
            }
            if __sw1 == 2i32 {
                rfu_LMAN_establishConnection(
                    (((&raw mut gRfu).cast::<u8>()).wrapping_add(12)).read(),
                    0u16,
                    240u16,
                    ((&raw const sAcceptedSerialNos)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<u16>())
                    .cast::<u16>(),
                );
                (((&raw mut gRfu).cast::<u8>()).wrapping_add(4).cast::<u16>()).write(3u16);
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(1))
                .write(6i16);
                break 'l1;
            }
            if __sw1 == 3i32 {
                break 'l1;
            }
            if __sw1 == 4i32 {
                rfu_LMAN_stopManager(0u8);
                (((&raw mut gRfu).cast::<u8>()).wrapping_add(4).cast::<u16>()).write(5u16);
                break 'l1;
            }
            if __sw1 == 5i32 {
                break 'l1;
            }
            if __sw1 == 18i32 {
                crate::c::volatile_write(((&raw mut gRfu).cast::<u8>()).wrapping_add(3291), 0u8);
                rfu_LMAN_setMSCCallback(Some(MSCCallback_Parent));
                InitChildRecvBuffers();
                InitParentSendData();
                (((&raw mut gRfu).cast::<u8>()).wrapping_add(4).cast::<u16>()).write(20u16);
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(1))
                .write(8i16);
                CreateTask(Some(Task_PlayerExchange), 5u8);
                DestroyTask(taskId);
                break 'l1;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Rfu_GetIndexOfNewestChild(bits: u8) -> i32 {
    unsafe {
        let mut bits = bits;
        return ((((((&raw const sPlayerBitsToNewChildIdx)
            .cast::<u8>()
            .cast_mut())
        .cast::<u8>())
        .wrapping_offset(((bits) as i32) as isize))
        .read()) as i32);
    }
}
pub(crate) unsafe extern "C" fn SetLinkPlayerIdsFromSlots(baseSlots: i32, addSlots: i32) {
    unsafe {
        let mut baseSlots = baseSlots;
        let mut addSlots = addSlots;
        let mut i: u8 = 0u8;
        let mut baseId: u8 = 1u8;
        let mut baseSlotsCopy: i32 = baseSlots;
        let mut newId: i32 = 0i32;
        if addSlots == (-1i32) {
            {
                i = 0u8;
                'l1: loop {
                    if !(((i) as i32) < 4i32) {
                        break 'l1;
                    }
                    'l2: {
                        if (baseSlots & 1i32) != 0 {
                            (((((&raw mut gRfu).cast::<u8>()).wrapping_add(3294)).cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize))
                            .write(baseId);
                            baseId = (baseId).wrapping_add(1);
                        }
                    }
                    baseSlots = (baseSlots >> 1);
                    i = (i).wrapping_add(1);
                }
            }
        } else {
            {
                i = 0u8;
                'l3: loop {
                    if !(((i) as i32) < 4i32) {
                        break 'l3;
                    }
                    'l4: {
                        if !((baseSlotsCopy & 1i32) != 0) {
                            (((((&raw mut gRfu).cast::<u8>()).wrapping_add(3294)).cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize))
                            .write(0u8);
                        }
                    }
                    baseSlotsCopy = (baseSlotsCopy >> 1);
                    i = (i).wrapping_add(1);
                }
            }
            {
                baseId = 4u8;
                'l5: loop {
                    if !(((baseId) as i32) != 0i32) {
                        break 'l5;
                    }
                    'l6: {
                        {
                            i = 0u8;
                            'l7: loop {
                                if !((((i) as i32) < 4i32)
                                    && ((((((((&raw mut gRfu).cast::<u8>()).wrapping_add(3294))
                                        .cast::<u8>())
                                    .wrapping_offset(((i) as i32) as isize))
                                    .read()) as i32)
                                        != ((baseId) as i32)))
                                {
                                    break 'l7;
                                }
                                'l8: {}
                                i = (i).wrapping_add(1);
                            }
                        }
                        if ((i) as i32) == 4i32 {
                            newId = ((baseId) as i32);
                        }
                    }
                    baseId = (baseId).wrapping_sub(1);
                }
            }
            {
                addSlots = (addSlots & !(baseSlots));
                i = 0u8;
                'l9: loop {
                    if !(((i) as i32) < 4i32) {
                        break 'l9;
                    }
                    'l10: {
                        if (addSlots & 1i32) != 0 {
                            (((((&raw mut gRfu).cast::<u8>()).wrapping_add(3294)).cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize))
                            .write(
                                (({
                                    let __t1 = newId;
                                    newId = (newId).wrapping_add(1);
                                    __t1
                                }) as u8),
                            );
                        }
                    }
                    addSlots = (addSlots >> 1);
                    i = (i).wrapping_add(1);
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_ChildSearchForParent(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        'l1: {
            let __sw1 =
                (((((&raw mut gRfu).cast::<u8>()).wrapping_add(4).cast::<u16>()).read()) as i32);
            if __sw1 == 0i32 {
                rfu_LMAN_initializeRFU((&raw const sRfuReqConfigTemplate).cast::<u8>().cast_mut());
                (((&raw mut gRfu).cast::<u8>()).wrapping_add(4).cast::<u16>()).write(1u16);
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(1))
                .write(1i16);
                break 'l1;
            }
            if __sw1 == 1i32 {
                break 'l1;
            }
            if __sw1 == 6i32 {
                rfu_LMAN_establishConnection(
                    (((&raw mut gRfu).cast::<u8>()).wrapping_add(12)).read(),
                    0u16,
                    240u16,
                    ((&raw const sAcceptedSerialNos)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<u16>())
                    .cast::<u16>(),
                );
                (((&raw mut gRfu).cast::<u8>()).wrapping_add(4).cast::<u16>()).write(7u16);
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(1))
                .write(7i16);
                break 'l1;
            }
            if __sw1 == 7i32 {
                break 'l1;
            }
            if __sw1 == 9i32 {
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(1))
                .write(10i16);
                break 'l1;
            }
            if __sw1 == 11i32 {
                'l2: {
                    let __sw2 = GetJoinGroupStatus();
                    if __sw2 == 5i32 {
                        (((&raw mut gRfu).cast::<u8>()).wrapping_add(4).cast::<u16>()).write(12u16);
                        break 'l2;
                    }
                    if __sw2 == 6i32 || __sw2 == 9i32 {
                        rfu_LMAN_requestChangeAgbClockMaster();
                        (((&raw mut gRfu).cast::<u8>()).wrapping_add(3300)).write(2u8);
                        DestroyTask(taskId);
                        break 'l2;
                    }
                }
                break 'l1;
            }
            if __sw1 == 12i32 {
                {
                    let mut bmChildSlot: u8 = ((crate::c::shl_i32(
                        1i32,
                        (((((&raw mut gRfu).cast::<u8>()).wrapping_add(3134)).read_volatile())
                            as u32),
                    )) as u8);
                    rfu_clearSlot(
                        12u8,
                        (((&raw mut gRfu).cast::<u8>()).wrapping_add(3134)).read_volatile(),
                    );
                    rfu_setRecvBuffer(
                        16u8,
                        (((&raw mut gRfu).cast::<u8>()).wrapping_add(3134)).read_volatile(),
                        (((&raw mut gRfu).cast::<u8>()).wrapping_add(3135)).cast::<u8>(),
                        70u32,
                    );
                    rfu_UNI_setSendData(
                        bmChildSlot,
                        (((&raw mut gRfu).cast::<u8>()).wrapping_add(76)).cast::<u8>(),
                        14u8,
                    );
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(1))
                    .write(8i16);
                    DestroyTask(taskId);
                    if (((((&raw mut sRfuDebug).cast::<u8>()).wrapping_add(15)).read()) as i32)
                        == 0i32
                    {
                        Debug_PrintEmpty();
                        let __p3 = ((&raw mut sRfuDebug).cast::<u8>()).wrapping_add(15);
                        (__p3).write(((__p3).read()).wrapping_add(1));
                    }
                    CreateTask(Some(Task_PlayerExchange), 5u8);
                    break 'l1;
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn InitChildRecvBuffers() {
    unsafe {
        let mut i: u8 = 0u8;
        let mut acceptSlot: u8 = ((&raw mut lman).cast::<u8>()).read();
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 4i32) {
                    break 'l1;
                }
                'l2: {
                    if (((acceptSlot) as i32) & 1i32) != 0 {
                        rfu_setRecvBuffer(
                            16u8,
                            i,
                            (((((&raw mut gRfu).cast::<u8>()).wrapping_add(20)).cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize * 14))
                            .cast::<u8>(),
                            14u32,
                        );
                        rfu_clearSlot(3u8, i);
                    }
                    acceptSlot = ((((acceptSlot) as i32) >> 1) as u8);
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn InitParentSendData() {
    unsafe {
        let mut acceptSlot: u8 = ((&raw mut lman).cast::<u8>()).read();
        rfu_UNI_setSendData(
            acceptSlot,
            (((&raw mut gRfu).cast::<u8>()).wrapping_add(3207)).cast::<u8>(),
            70u8,
        );
        (((&raw mut gRfu).cast::<u8>()).wrapping_add(3290))
            .write(((Rfu_GetIndexOfNewestChild(acceptSlot)) as u8));
        (((&raw mut gRfu).cast::<u8>()).wrapping_add(3298)).write(acceptSlot);
        SetLinkPlayerIdsFromSlots(((acceptSlot) as i32), (-1i32));
        (((&raw mut gRfu).cast::<u8>()).wrapping_add(12)).write(1u8);
    }
}
pub(crate) unsafe extern "C" fn Task_UnionRoomListen(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if (((crate::c::bf_read((GetHostRfuGameData()).wrapping_add(10), 0, 7, false) as u8)
            as i32)
            == 84i32)
            && (((RfuGetStatus()) as i32) == 4i32)
        {
            rfu_REQ_disconnect(((&raw mut lman).cast::<u8>()).read());
            rfu_waitREQComplete();
            RfuSetStatus(0u8, 0u16);
        }
        'l1: {
            let __sw1 =
                (((((&raw mut gRfu).cast::<u8>()).wrapping_add(4).cast::<u16>()).read()) as i32);
            if __sw1 == 0i32 {
                rfu_LMAN_initializeRFU((&raw mut sRfuReqConfig).cast::<u8>());
                (((&raw mut gRfu).cast::<u8>()).wrapping_add(4).cast::<u16>()).write(1u16);
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(1))
                .write(1i16);
                break 'l1;
            }
            if __sw1 == 1i32 {
                break 'l1;
            }
            if __sw1 == 17i32 {
                rfu_LMAN_establishConnection(
                    2u8,
                    0u16,
                    240u16,
                    ((&raw const sAcceptedSerialNos)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<u16>())
                    .cast::<u16>(),
                );
                rfu_LMAN_setMSCCallback(Some(MSCCallback_Child));
                (((&raw mut gRfu).cast::<u8>()).wrapping_add(4).cast::<u16>()).write(18u16);
                break 'l1;
            }
            if __sw1 == 18i32 {
                break 'l1;
            }
            if __sw1 == 13i32 {
                if ((rfu_UNI_setSendData(
                    ((crate::c::shl_i32(
                        1i32,
                        (((((&raw mut gRfu).cast::<u8>()).wrapping_add(3134)).read_volatile())
                            as u32),
                    )) as u8),
                    (((&raw mut gRfu).cast::<u8>()).wrapping_add(76)).cast::<u8>(),
                    14u8,
                )) as i32)
                    == 0i32
                {
                    (((&raw mut gRfu).cast::<u8>()).wrapping_add(12)).write(0u8);
                    DestroyTask(taskId);
                    if (((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(7))
                    .read())
                        != 0
                    {
                        CreateTask(Some(Task_PlayerExchangeChat), 1u8);
                    } else {
                        CreateTask(Some(Task_PlayerExchange), 5u8);
                    }
                }
                break 'l1;
            }
            if __sw1 == 14i32 {
                rfu_LMAN_stopManager(0u8);
                (((&raw mut gRfu).cast::<u8>()).wrapping_add(4).cast::<u16>()).write(15u16);
                break 'l1;
            }
            if __sw1 == 15i32 {
                break 'l1;
            }
            if __sw1 == 16i32 {
                crate::c::volatile_write(((&raw mut gRfu).cast::<u8>()).wrapping_add(3291), 0u8);
                rfu_LMAN_setMSCCallback(Some(MSCCallback_Parent));
                UpdateGameData_GroupLockedIn(1u8);
                InitChildRecvBuffers();
                InitParentSendData();
                (((&raw mut gRfu).cast::<u8>()).wrapping_add(4).cast::<u16>()).write(20u16);
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(1))
                .write(8i16);
                (((&raw mut gRfu).cast::<u8>()).wrapping_add(12)).write(1u8);
                CreateTask(Some(Task_PlayerExchange), 5u8);
                (((&raw mut gRfu).cast::<u8>()).wrapping_add(3304)).write(1u8);
                DestroyTask(taskId);
                break 'l1;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn LinkRfu_CreateConnectionAsParent() {
    unsafe {
        rfu_LMAN_establishConnection(
            1u8,
            0u16,
            240u16,
            ((&raw const sAcceptedSerialNos)
                .cast::<u8>()
                .cast_mut()
                .cast::<u16>())
            .cast::<u16>(),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn LinkRfu_StopManagerBeforeEnteringChat() {
    unsafe {
        rfu_LMAN_stopManager(0u8);
    }
}
pub(crate) unsafe extern "C" fn MSCCallback_Child(REQ_commandID: u16) {
    unsafe {
        let mut REQ_commandID = REQ_commandID;
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 14i32) {
                    break 'l1;
                }
                'l2: {
                    (((((&raw mut gRfu).cast::<u8>()).wrapping_add(76)).cast::<u8>())
                        .wrapping_offset((i) as isize))
                    .write(0u8);
                }
                i = (i).wrapping_add(1);
            }
        }
        rfu_REQ_recvData();
        rfu_waitREQComplete();
        if ((((((((&raw mut gRfuSlotStatusUNI).cast::<*mut u8>()).cast::<*mut u8>())
            .wrapping_offset(
                (((((&raw mut gRfu).cast::<u8>()).wrapping_add(3134)).read_volatile()) as i32)
                    as isize,
            ))
        .read())
        .wrapping_add(12))
        .wrapping_add(6))
        .read())
            != 0
        {
            let __p1 = ((&raw mut gRfu).cast::<u8>()).wrapping_add(3280);
            crate::c::volatile_write(__p1, ((__p1).read_volatile()).wrapping_add(1));
            RfuRecvQueue_Enqueue(
                ((&raw mut gRfu).cast::<u8>()).wrapping_add(292),
                (((&raw mut gRfu).cast::<u8>()).wrapping_add(3135)).cast::<u8>(),
            );
            let __p2 = ((&raw mut sRfuDebug).cast::<u8>())
                .wrapping_add(6)
                .cast::<u16>();
            (__p2).write(((__p2).read()).wrapping_add(1));
            UpdateBackupQueue();
            rfu_UNI_readySendData(
                (((&raw mut gRfu).cast::<u8>()).wrapping_add(3134)).read_volatile(),
            );
            rfu_UNI_clearRecvNewDataFlag(
                (((&raw mut gRfu).cast::<u8>()).wrapping_add(3134)).read_volatile(),
            );
        }
        rfu_LMAN_REQ_sendData(1u8);
    }
}
pub(crate) unsafe extern "C" fn MSCCallback_Parent(REQ_commandID: u16) {
    unsafe {
        let mut REQ_commandID = REQ_commandID;
        crate::c::volatile_write(((&raw mut gRfu).cast::<u8>()).wrapping_add(3291), 1u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn LinkRfu_Shutdown() {
    unsafe {
        let mut i: u8 = 0u8;
        rfu_LMAN_powerDownRFU();
        if (((((&raw mut gRfu).cast::<u8>()).wrapping_add(12)).read()) as i32) == 1i32 {
            if ((FuncIsActiveTask(Some(Task_ParentSearchForChildren))) as i32) == 1i32 {
                DestroyTask((((&raw mut gRfu).cast::<u8>()).wrapping_add(103)).read());
                ResetLinkRfuGFLayer();
            }
        } else {
            if (((((&raw mut gRfu).cast::<u8>()).wrapping_add(12)).read()) as i32) == 0i32 {
                if ((FuncIsActiveTask(Some(Task_ChildSearchForParent))) as i32) == 1i32 {
                    DestroyTask((((&raw mut gRfu).cast::<u8>()).wrapping_add(103)).read());
                    ResetLinkRfuGFLayer();
                }
            } else {
                if (((((&raw mut gRfu).cast::<u8>()).wrapping_add(12)).read()) as i32) == 2i32 {
                    if ((FuncIsActiveTask(Some(Task_UnionRoomListen))) as i32) == 1i32 {
                        DestroyTask((((&raw mut gRfu).cast::<u8>()).wrapping_add(103)).read());
                        ResetLinkRfuGFLayer();
                    }
                }
            }
        }
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as u32) < crate::c::div_u32(12u32, 4u32)) {
                    break 'l1;
                }
                'l2: {
                    if ((FuncIsActiveTask(
                        ((((&raw const sShutdownTasks)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<Option<unsafe extern "C" fn(u8)>>())
                        .cast::<Option<unsafe extern "C" fn(u8)>>())
                        .wrapping_offset(((i) as i32) as isize))
                        .read(),
                    )) as i32)
                        == 1i32
                    {
                        DestroyTask(FindTaskIdByFunc(
                            ((((&raw const sShutdownTasks)
                                .cast::<u8>()
                                .cast_mut()
                                .cast::<Option<unsafe extern "C" fn(u8)>>())
                            .cast::<Option<unsafe extern "C" fn(u8)>>())
                            .wrapping_offset(((i) as i32) as isize))
                            .read(),
                        ));
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn CreateTask_ParentSearchForChildren() {
    unsafe {
        (((&raw mut gRfu).cast::<u8>()).wrapping_add(103))
            .write(CreateTask(Some(Task_ParentSearchForChildren), 1u8));
    }
}
pub(crate) unsafe extern "C" fn CanTryReconnectParent() -> u8 {
    unsafe {
        if ((((((&raw mut gRfu).cast::<u8>()).wrapping_add(4).cast::<u16>()).read()) as i32)
            == 7i32)
            && (((((&raw mut gRfu).cast::<u8>()).wrapping_add(3277)).read()) != 0)
        {
            return 1u8;
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn TryReconnectParent() -> u32 {
    unsafe {
        if ((((((&raw mut gRfu).cast::<u8>()).wrapping_add(4).cast::<u16>()).read()) as i32)
            == 7i32)
            && (!((rfu_LMAN_CHILD_connectParent(
                (((((((&raw mut gRfuLinkStatus).cast::<*mut u8>()).read()).wrapping_add(20))
                    .cast::<u8>())
                .wrapping_offset(
                    (((((&raw mut gRfu).cast::<u8>()).wrapping_add(3133)).read()) as i32) as isize
                        * 32,
                ))
                .cast::<u16>())
                .read(),
                240u16,
            )) != 0))
        {
            (((&raw mut gRfu).cast::<u8>()).wrapping_add(4).cast::<u16>()).write(9u16);
            return 1u32;
        }
        return 0u32;
    }
}
pub(crate) unsafe extern "C" fn CreateTask_ChildSearchForParent() {
    unsafe {
        (((&raw mut gRfu).cast::<u8>()).wrapping_add(103))
            .write(CreateTask(Some(Task_ChildSearchForParent), 1u8));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn LmanAcceptSlotFlagIsNotZero() -> u8 {
    unsafe {
        if (((&raw mut lman).cast::<u8>()).read()) != 0 {
            return 1u8;
        }
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn LinkRfu_StopManagerAndFinalizeSlots() {
    unsafe {
        (((&raw mut gRfu).cast::<u8>()).wrapping_add(4).cast::<u16>()).write(4u16);
        (((&raw mut gRfu).cast::<u8>()).wrapping_add(3303))
            .write(((&raw mut lman).cast::<u8>()).read());
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn WaitRfuState(force: u32) -> u32 {
    unsafe {
        let mut force = force;
        if ((((((&raw mut gRfu).cast::<u8>()).wrapping_add(4).cast::<u16>()).read()) as i32)
            == 17i32)
            || ((force) != 0)
        {
            (((&raw mut gRfu).cast::<u8>()).wrapping_add(4).cast::<u16>()).write(18u16);
            return 1u32;
        }
        return 0u32;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn StopUnionRoomLinkManager() {
    unsafe {
        (((&raw mut gRfu).cast::<u8>()).wrapping_add(4).cast::<u16>()).write(14u16);
    }
}
pub(crate) unsafe extern "C" fn ReadySendDataForSlots(slots: u8) {
    unsafe {
        let mut slots = slots;
        let mut i: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 4i32) {
                    break 'l1;
                }
                'l2: {
                    if (((slots) as i32) & 1i32) != 0 {
                        rfu_UNI_readySendData(i);
                        break 'l1;
                    }
                    slots = ((((slots) as i32) >> 1) as u8);
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn ReadAllPlayerRecvCmds() {
    unsafe {
        let mut i: i32 = 0i32;
        let mut j: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 5i32) {
                    break 'l1;
                }
                'l2: {
                    let mut rfu: *mut u8 = (&raw mut gRfu).cast::<u8>();
                    {
                        j = 0i32;
                        'l3: loop {
                            if !(j < 7i32) {
                                break 'l3;
                            }
                            'l4: {
                                ((((((((rfu).wrapping_add(3207)).cast::<u8>())
                                    .wrapping_offset((i) as isize * 14))
                                .cast::<u8>())
                                .wrapping_offset((j) as isize * 2))
                                .cast::<u8>())
                                .wrapping_offset(1))
                                .write(
                                    (((((((((&raw mut gRecvCmds).cast::<u8>())
                                        .wrapping_offset((i) as isize * 16))
                                    .cast::<u16>())
                                    .wrapping_offset((j) as isize))
                                    .read()) as i32)
                                        >> 8) as u8),
                                );
                                (((((((rfu).wrapping_add(3207)).cast::<u8>())
                                    .wrapping_offset((i) as isize * 14))
                                .cast::<u8>())
                                .wrapping_offset((j) as isize * 2))
                                .cast::<u8>())
                                .write(
                                    (((((((&raw mut gRecvCmds).cast::<u8>())
                                        .wrapping_offset((i) as isize * 16))
                                    .cast::<u16>())
                                    .wrapping_offset((j) as isize))
                                    .read()) as u8),
                                );
                            }
                            j = (j).wrapping_add(1);
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        'l5: loop {
            'l6: {
                {
                    let mut tmp: u16 = 0u16;
                    (&raw mut tmp).write_volatile(0u16);
                    'l7: loop {
                        'l8: {
                            CpuSet(
                                (&raw mut tmp).cast::<u8>(),
                                (&raw mut gRecvCmds).cast::<u8>(),
                                (16777216u32
                                    | (crate::c::div_u32(
                                        80u32,
                                        ((crate::c::div_i32(16i32, 8i32)) as u32),
                                    ) & 2097151u32)),
                            );
                        }
                        if !((0i32) != 0) {
                            break 'l7;
                        }
                    }
                }
            }
            if !((0i32) != 0) {
                break 'l5;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn MoveSendCmdToRecv() {
    unsafe {
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 7i32) {
                    break 'l1;
                }
                'l2: {
                    ((((&raw mut gRecvCmds).cast::<u8>()).cast::<u16>())
                        .wrapping_offset((i) as isize))
                    .write(
                        ((((&raw mut gSendCmd).cast::<u16>()).cast::<u16>())
                            .wrapping_offset((i) as isize))
                        .read(),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        {
            i = 0i32;
            'l3: loop {
                if !(i < 7i32) {
                    break 'l3;
                }
                'l4: {
                    ((((&raw mut gSendCmd).cast::<u16>()).cast::<u16>())
                        .wrapping_offset((i) as isize))
                    .write(0u16);
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn UpdateBackupQueue() {
    unsafe {
        if ((((&raw mut gRfu).cast::<u8>()).wrapping_add(3132)).read_volatile()) != 0 {
            let mut backupEmpty: u8 = RfuBackupQueue_Dequeue(
                ((&raw mut gRfu).cast::<u8>()).wrapping_add(3100),
                (((&raw mut gRfu).cast::<u8>()).wrapping_add(76)).cast::<u8>(),
            );
            if ((((((&raw mut gRfu).cast::<u8>()).wrapping_add(3100)).wrapping_add(30))
                .read_volatile()) as i32)
                == 0i32
            {
                crate::c::volatile_write(((&raw mut gRfu).cast::<u8>()).wrapping_add(3132), 0u8);
            }
            if (backupEmpty) != 0 {
                return;
            }
        }
        if !(((((&raw mut gRfu).cast::<u8>()).wrapping_add(3132)).read_volatile()) != 0) {
            RfuSendQueue_Dequeue(
                ((&raw mut gRfu).cast::<u8>()).wrapping_add(2536),
                (((&raw mut gRfu).cast::<u8>()).wrapping_add(76)).cast::<u8>(),
            );
            RfuBackupQueue_Enqueue(
                ((&raw mut gRfu).cast::<u8>()).wrapping_add(3100),
                (((&raw mut gRfu).cast::<u8>()).wrapping_add(76)).cast::<u8>(),
            );
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsRfuRecvQueueEmpty() -> u32 {
    unsafe {
        let mut i: i32 = 0i32;
        let mut j: i32 = 0i32;
        if !((((((&raw mut gRfuLinkStatus).cast::<*mut u8>()).read()).wrapping_add(6)).read()) != 0)
        {
            return 0u32;
        }
        {
            i = 0i32;
            'l1: loop {
                if !(i < 5i32) {
                    break 'l1;
                }
                'l2: {
                    {
                        j = 0i32;
                        'l3: loop {
                            if !(j < 7i32) {
                                break 'l3;
                            }
                            'l4: {
                                if (((((((&raw mut gRecvCmds).cast::<u8>())
                                    .wrapping_offset((i) as isize * 16))
                                .cast::<u16>())
                                .wrapping_offset((j) as isize))
                                .read()) as i32)
                                    != 0i32
                                {
                                    return 0u32;
                                }
                            }
                            j = (j).wrapping_add(1);
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return 1u32;
    }
}
pub(crate) unsafe extern "C" fn RfuMain1_Parent() -> u32 {
    unsafe {
        if (((((&raw mut gRfu).cast::<u8>()).wrapping_add(4).cast::<u16>()).read()) as i32) < 20i32
        {
            rfu_REQ_recvData();
            rfu_waitREQComplete();
            rfu_LMAN_REQ_sendData(0u8);
        } else {
            crate::c::volatile_write(((&raw mut gRfu).cast::<u8>()).wrapping_add(3291), 0u8);
            if (((((((&raw mut gRfu).cast::<u8>()).wrapping_add(3298)).read()) as i32)
                & ((((((&raw mut gRfuLinkStatus).cast::<*mut u8>()).read()).wrapping_add(2)).read())
                    as i32))
                == (((((&raw mut gRfu).cast::<u8>()).wrapping_add(3298)).read()) as i32))
                && (((((((&raw mut gRfu).cast::<u8>()).wrapping_add(3298)).read()) as i32)
                    & ((((((&raw mut gRfuLinkStatus).cast::<*mut u8>()).read()).wrapping_add(2))
                        .read()) as i32))
                    != 0)
            {
                if !(((((&raw mut gRfu).cast::<u8>()).wrapping_add(3292)).read_volatile()) != 0) {
                    if ((((&raw mut gRfu).cast::<u8>()).wrapping_add(3299)).read()) != 0 {
                        RfuReqDisconnectSlot(
                            (((((&raw mut gRfu).cast::<u8>()).wrapping_add(3299)).read()) as u32),
                        );
                        (((&raw mut gRfu).cast::<u8>()).wrapping_add(3299)).write(0u8);
                        if (((((&raw mut gRfu).cast::<u8>()).wrapping_add(3300)).read()) as i32)
                            == 1i32
                        {
                            RfuSetStatus(2u8, 32768u16);
                            RfuSetErrorParams(32768u32);
                            return 0u32;
                        }
                        if !((((&raw mut lman).cast::<u8>()).read()) != 0) {
                            LinkRfu_Shutdown();
                            ((&raw mut gReceivedRemoteLinkPlayers).cast::<u8>()).write(0u8);
                            return 0u32;
                        }
                    }
                    ReadAllPlayerRecvCmds();
                    rfu_UNI_readySendData(
                        (((&raw mut gRfu).cast::<u8>()).wrapping_add(3290)).read(),
                    );
                    rfu_LMAN_REQ_sendData(1u8);
                } else {
                    rfu_REQ_PARENT_resumeRetransmitAndChange();
                }
                (((&raw mut gRfu).cast::<u8>()).wrapping_add(14)).write(1u8);
            }
        }
        return 0u32;
    }
}
pub(crate) unsafe extern "C" fn RfuMain2_Parent() -> u32 {
    unsafe {
        let mut i: u16 = 0u16;
        let mut flags: u16 = 0u16;
        let mut r0: u8 = 0u8;
        let mut j: u16 = 0u16;
        let mut failed: u8 = 0u8;
        if ((((((&raw mut gRfu).cast::<u8>()).wrapping_add(4).cast::<u16>()).read()) as i32)
            >= 20i32)
            && ((((((&raw mut gRfu).cast::<u8>()).wrapping_add(14)).read()) as i32) == 1i32)
        {
            rfu_waitREQComplete();
            'l1: loop {
                if !((((((&raw mut gRfu).cast::<u8>()).wrapping_add(3291)).read_volatile()) as i32)
                    == 0i32)
                {
                    break 'l1;
                }
                if (((((&raw mut gRfu).cast::<u8>()).wrapping_add(238)).read_volatile()) as i32)
                    != 0i32
                {
                    return 0u32;
                }
            }
            rfu_REQ_recvData();
            rfu_waitREQComplete();
            if ((((((&raw mut lman).cast::<u8>()).wrapping_add(3)).read_volatile()) as i32)
                & (((((&raw mut gRfu).cast::<u8>()).wrapping_add(3298)).read()) as i32))
                == (((((&raw mut gRfu).cast::<u8>()).wrapping_add(3298)).read()) as i32)
            {
                crate::c::volatile_write(((&raw mut gRfu).cast::<u8>()).wrapping_add(3292), 0u8);
                let __p1 = ((&raw mut sRfuDebug).cast::<u8>())
                    .wrapping_add(6)
                    .cast::<u16>();
                (__p1).write(((__p1).read()).wrapping_add(1));
                flags = ((((&raw mut lman).cast::<u8>()).read()) as u16);
                {
                    i = 0u16;
                    'l2: loop {
                        if !(((i) as i32) < 4i32) {
                            break 'l2;
                        }
                        'l3: {
                            if (((flags) as i32) & 1i32) != 0 {
                                if ((((((((&raw mut gRfu).cast::<u8>()).wrapping_add(20))
                                    .cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize * 14))
                                .cast::<u8>())
                                .wrapping_offset(1))
                                .read())
                                    != 0
                                {
                                    if ((((((((&raw mut gRfu).cast::<u8>()).wrapping_add(3310))
                                        .cast::<u8>())
                                    .wrapping_offset(((i) as i32) as isize))
                                    .read()) as i32)
                                        != 255i32)
                                        && ((((((((((&raw mut gRfu).cast::<u8>())
                                            .wrapping_add(20))
                                        .cast::<u8>())
                                        .wrapping_offset(((i) as i32) as isize * 14))
                                        .cast::<u8>())
                                        .read())
                                            as i32)
                                            >> 5)
                                            != ((((((((&raw mut gRfu).cast::<u8>())
                                                .wrapping_add(3310))
                                            .cast::<u8>())
                                            .wrapping_offset(((i) as i32) as isize))
                                            .read())
                                                as i32)
                                                .wrapping_add(1i32)
                                                & 7i32))
                                    {
                                        if (({
                                            let __p2 = ((((&raw mut gRfu).cast::<u8>())
                                                .wrapping_add(3306))
                                            .cast::<u8>())
                                            .wrapping_offset(((i) as i32) as isize);
                                            let __t3 = ((__p2).read()).wrapping_add(1);
                                            (__p2).write(__t3);
                                            __t3
                                        }) as i32)
                                            > 4i32
                                        {
                                            RfuSetErrorParams(33024u32);
                                        }
                                    } else {
                                        (((((&raw mut gRfu).cast::<u8>()).wrapping_add(3310))
                                            .cast::<u8>())
                                        .wrapping_offset(((i) as i32) as isize))
                                        .write(
                                            ((crate::c::div_i32(
                                                ((((((((&raw mut gRfu).cast::<u8>())
                                                    .wrapping_add(20))
                                                .cast::<u8>())
                                                .wrapping_offset(((i) as i32) as isize * 14))
                                                .cast::<u8>())
                                                .read())
                                                    as i32),
                                                32i32,
                                            )) as u8),
                                        );
                                        (((((&raw mut gRfu).cast::<u8>()).wrapping_add(3306))
                                            .cast::<u8>())
                                        .wrapping_offset(((i) as i32) as isize))
                                        .write(0u8);
                                        let __p4 = (((((&raw mut gRfu).cast::<u8>())
                                            .wrapping_add(20))
                                        .cast::<u8>())
                                        .wrapping_offset(((i) as i32) as isize * 14))
                                        .cast::<u8>();
                                        (__p4).write((((((__p4).read()) as i32) & 31i32) as u8));
                                        r0 = (((((&raw mut gRfu).cast::<u8>())
                                            .wrapping_add(3294))
                                        .cast::<u8>())
                                        .wrapping_offset(((i) as i32) as isize))
                                        .read();
                                        {
                                            j = 0u16;
                                            'l4: loop {
                                                if !(((j) as i32) < 7i32) {
                                                    break 'l4;
                                                }
                                                'l5: {
                                                    (((((&raw mut gRecvCmds).cast::<u8>())
                                                        .wrapping_offset(
                                                            ((r0) as i32) as isize * 16,
                                                        ))
                                                    .cast::<u16>())
                                                    .wrapping_offset(((j) as i32) as isize))
                                                    .write(
                                                        ((((((((((((&raw mut gRfu)
                                                            .cast::<u8>())
                                                        .wrapping_add(20))
                                                        .cast::<u8>())
                                                        .wrapping_offset(
                                                            ((i) as i32) as isize * 14,
                                                        ))
                                                        .cast::<u8>())
                                                        .wrapping_offset(
                                                            ((((j) as i32) << 1).wrapping_add(1i32))
                                                                as isize,
                                                        ))
                                                        .read())
                                                            as i32)
                                                            << 8)
                                                            | (((((((((&raw mut gRfu)
                                                                .cast::<u8>())
                                                            .wrapping_add(20))
                                                            .cast::<u8>())
                                                            .wrapping_offset(
                                                                ((i) as i32) as isize * 14,
                                                            ))
                                                            .cast::<u8>())
                                                            .wrapping_offset(
                                                                ((((j) as i32) << 1)
                                                                    .wrapping_add(0i32))
                                                                    as isize,
                                                            ))
                                                            .read())
                                                                as i32))
                                                            as u16),
                                                    );
                                                    (((((((&raw mut gRfu).cast::<u8>())
                                                        .wrapping_add(20))
                                                    .cast::<u8>())
                                                    .wrapping_offset(((i) as i32) as isize * 14))
                                                    .cast::<u8>())
                                                    .wrapping_offset(
                                                        ((((j) as i32) << 1).wrapping_add(1i32))
                                                            as isize,
                                                    ))
                                                    .write(0u8);
                                                    (((((((&raw mut gRfu).cast::<u8>())
                                                        .wrapping_add(20))
                                                    .cast::<u8>())
                                                    .wrapping_offset(((i) as i32) as isize * 14))
                                                    .cast::<u8>())
                                                    .wrapping_offset(
                                                        ((((j) as i32) << 1).wrapping_add(0i32))
                                                            as isize,
                                                    ))
                                                    .write(0u8);
                                                }
                                                j = (j).wrapping_add(1);
                                            }
                                        }
                                    }
                                }
                                rfu_UNI_clearRecvNewDataFlag(((i) as u8));
                            }
                            flags = ((((flags) as i32) >> 1) as u16);
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                MoveSendCmdToRecv();
                RfuHandleReceiveCommand(0u8);
                CallRfuFunc();
                if (((((&raw mut gRfu).cast::<u8>()).wrapping_add(3301)).read()) != 0)
                    && (!(((((&raw mut gRfu).cast::<u8>()).wrapping_add(3289)).read()) != 0))
                {
                    crate::c::volatile_write(
                        ((&raw mut sRfuDebug).cast::<u8>()).wrapping_add(14),
                        0u8,
                    );
                    rfu_clearSlot(
                        3u8,
                        (((&raw mut gRfu).cast::<u8>()).wrapping_add(3290)).read(),
                    );
                    {
                        i = 0u16;
                        'l6: loop {
                            if !(((i) as i32) < 4i32) {
                                break 'l6;
                            }
                            'l7: {
                                if (crate::c::shr_i32(
                                    (((((&raw mut gRfu).cast::<u8>()).wrapping_add(3301)).read())
                                        as i32),
                                    ((i) as u32),
                                ) & 1i32)
                                    != 0
                                {
                                    rfu_setRecvBuffer(
                                        16u8,
                                        ((i) as u8),
                                        (((((&raw mut gRfu).cast::<u8>()).wrapping_add(20))
                                            .cast::<u8>())
                                        .wrapping_offset(((i) as i32) as isize * 14))
                                        .cast::<u8>(),
                                        14u32,
                                    );
                                }
                            }
                            i = (i).wrapping_add(1);
                        }
                    }
                    SetLinkPlayerIdsFromSlots(
                        (((((&raw mut gRfu).cast::<u8>()).wrapping_add(3298)).read()) as i32),
                        ((((((&raw mut gRfu).cast::<u8>()).wrapping_add(3298)).read()) as i32)
                            | (((((&raw mut gRfu).cast::<u8>()).wrapping_add(3301)).read())
                                as i32)),
                    );
                    (((&raw mut gRfu).cast::<u8>()).wrapping_add(3305))
                        .write((((&raw mut gRfu).cast::<u8>()).wrapping_add(3301)).read());
                    let __p5 = ((&raw mut gRfu).cast::<u8>()).wrapping_add(3298);
                    (__p5).write(
                        (((((__p5).read()) as i32)
                            | (((((&raw mut gRfu).cast::<u8>()).wrapping_add(3301)).read()) as i32))
                            as u8),
                    );
                    (((&raw mut gRfu).cast::<u8>()).wrapping_add(3301)).write(0u8);
                    rfu_UNI_setSendData(
                        (((&raw mut gRfu).cast::<u8>()).wrapping_add(3298)).read(),
                        (((&raw mut gRfu).cast::<u8>()).wrapping_add(3207)).cast::<u8>(),
                        70u8,
                    );
                    (((&raw mut gRfu).cast::<u8>()).wrapping_add(3290)).write(
                        ((Rfu_GetIndexOfNewestChild(
                            (((&raw mut gRfu).cast::<u8>()).wrapping_add(3298)).read(),
                        )) as u8),
                    );
                    CreateTask(Some(Task_PlayerExchangeUpdate), 0u8);
                }
            } else {
                crate::c::volatile_write(((&raw mut gRfu).cast::<u8>()).wrapping_add(3292), 1u8);
                (((&raw mut gRfu).cast::<u8>()).wrapping_add(14)).write(0u8);
            }
            (((&raw mut gRfu).cast::<u8>()).wrapping_add(14)).write(0u8);
        }
        failed = (((&raw mut gRfu).cast::<u8>()).wrapping_add(3292)).read_volatile();
        return ((if (((((&raw mut gRfuLinkStatus).cast::<*mut u8>()).read()).wrapping_add(6))
            .read())
            != 0
        {
            (((failed) as i32) & 1i32)
        } else {
            0i32
        }) as u32);
    }
}
pub(crate) unsafe extern "C" fn ChildBuildSendCmd(sendCmd: *mut u16, dst: *mut u8) {
    unsafe {
        let mut sendCmd = sendCmd;
        let mut dst = dst;
        let mut i: i32 = 0i32;
        if ((sendCmd).read()) != 0 {
            (sendCmd).write(
                (((((sendCmd).read()) as i32)
                    | ((((((&raw mut gRfu).cast::<u8>()).wrapping_add(258)).read()) as i32) << 5))
                    as u16),
            );
            (((&raw mut gRfu).cast::<u8>()).wrapping_add(258)).write(
                (((((((&raw mut gRfu).cast::<u8>()).wrapping_add(258)).read()) as i32)
                    .wrapping_add(1i32)
                    & 7i32) as u8),
            );
            {
                i = 0i32;
                'l1: loop {
                    if !(i < 7i32) {
                        break 'l1;
                    }
                    'l2: {
                        ((dst).wrapping_offset(
                            (((2i32).wrapping_mul(i)).wrapping_add(1i32)) as isize,
                        ))
                        .write(
                            ((((((sendCmd).wrapping_offset((i) as isize)).read()) as i32) >> 8)
                                as u8),
                        );
                        ((dst).wrapping_offset(
                            (((2i32).wrapping_mul(i)).wrapping_add(0i32)) as isize,
                        ))
                        .write(((((sendCmd).wrapping_offset((i) as isize)).read()) as u8));
                    }
                    i = (i).wrapping_add(1);
                }
            }
        } else {
            {
                i = 0i32;
                'l3: loop {
                    if !(i < 14i32) {
                        break 'l3;
                    }
                    'l4: {
                        ((dst).wrapping_offset((i) as isize)).write(0u8);
                    }
                    i = (i).wrapping_add(1);
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn RfuMain1_Child() -> u32 {
    unsafe {
        let mut i: u8 = 0u8;
        let mut j: u8 = 0u8;
        let mut recv = crate::ffi::Align4([0u8; 70]);
        let mut send = crate::ffi::Align4([0u8; 14]);
        let mut status: u8 = 0u8;
        RfuRecvQueue_Dequeue(
            ((&raw mut gRfu).cast::<u8>()).wrapping_add(292),
            (&raw mut recv).cast::<u8>(),
        );
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 5i32) {
                    break 'l1;
                }
                'l2: {
                    {
                        j = 0u8;
                        'l3: loop {
                            if !(((j) as i32) < 7i32) {
                                break 'l3;
                            }
                            'l4: {
                                (((((&raw mut gRecvCmds).cast::<u8>())
                                    .wrapping_offset(((i) as i32) as isize * 16))
                                .cast::<u16>())
                                .wrapping_offset(((j) as i32) as isize))
                                .write(
                                    ((((((((&raw mut recv).cast::<u8>()).wrapping_offset(
                                        (((((i) as i32).wrapping_mul(14i32))
                                            .wrapping_add(((j) as i32).wrapping_mul(2i32)))
                                        .wrapping_add(1i32))
                                            as isize,
                                    ))
                                    .read()) as i32)
                                        << 8)
                                        | (((((&raw mut recv).cast::<u8>()).wrapping_offset(
                                            (((((i) as i32).wrapping_mul(14i32))
                                                .wrapping_add(((j) as i32).wrapping_mul(2i32)))
                                            .wrapping_add(0i32))
                                                as isize,
                                        ))
                                        .read()) as i32))
                                        as u16),
                                );
                            }
                            j = (j).wrapping_add(1);
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        RfuHandleReceiveCommand(0u8);
        if ((((((&raw mut lman).cast::<u8>()).wrapping_add(2)).read_volatile()) as i32) == 0i32)
            && ((((((&raw mut gRfu).cast::<u8>()).wrapping_add(3300)).read()) as i32) != 0i32)
        {
            rfu_REQ_disconnect(
                ((((((((&raw mut gRfuLinkStatus).cast::<*mut u8>()).read()).wrapping_add(2)).read())
                    as i32)
                    | ((((((&raw mut gRfuLinkStatus).cast::<*mut u8>()).read()).wrapping_add(3))
                        .read()) as i32)) as u8),
            );
            rfu_waitREQComplete();
            status = RfuGetStatus();
            if ((((status) as i32) != 1i32) && (((status) as i32) != 6i32))
                && (((status) as i32) != 9i32)
            {
                RfuSetStatus(2u8, 36864u16);
            }
            rfu_clearAllSlot();
            ((&raw mut gReceivedRemoteLinkPlayers).cast::<u8>()).write(0u8);
            (((&raw mut gRfu).cast::<u8>()).cast::<Option<unsafe extern "C" fn()>>()).write(None);
            if (((((&raw mut gRfu).cast::<u8>()).wrapping_add(3300)).read()) as i32) == 1i32 {
                RfuSetStatus(2u8, 36864u16);
                RfuSetErrorParams(36864u32);
            }
            (((&raw mut lman).cast::<u8>()).wrapping_add(4)).write({
                let __v1 = 0u8;
                (((&raw mut lman).cast::<u8>()).wrapping_add(5)).write(__v1);
                __v1
            });
            (((&raw mut gRfu).cast::<u8>()).wrapping_add(3300)).write(0u8);
        }
        if ((((&raw mut gRfu).cast::<u8>()).wrapping_add(3280)).read_volatile()) != 0 {
            let __p2 = ((&raw mut gRfu).cast::<u8>()).wrapping_add(3280);
            crate::c::volatile_write(__p2, ((__p2).read_volatile()).wrapping_sub(1));
            CallRfuFunc();
            ChildBuildSendCmd(
                ((&raw mut gSendCmd).cast::<u16>()).cast::<u16>(),
                (&raw mut send).cast::<u8>(),
            );
            RfuSendQueue_Enqueue(
                ((&raw mut gRfu).cast::<u8>()).wrapping_add(2536),
                (&raw mut send).cast::<u8>(),
            );
            {
                i = 0u8;
                'l5: loop {
                    if !(((i) as i32) < 7i32) {
                        break 'l5;
                    }
                    'l6: {
                        ((((&raw mut gSendCmd).cast::<u16>()).cast::<u16>())
                            .wrapping_offset(((i) as i32) as isize))
                        .write(0u16);
                    }
                    i = (i).wrapping_add(1);
                }
            }
        }
        return IsRfuRecvQueueEmpty();
    }
}
pub(crate) unsafe extern "C" fn HandleSendFailure(unused: u8, flags: u32) {
    unsafe {
        let mut unused = unused;
        let mut flags = flags;
        let mut i: i32 = 0i32;
        let mut j: i32 = 0i32;
        let mut temp: i32 = 0i32;
        let mut payload: *mut u8 = ((((&raw mut gRfu).cast::<u8>()).wrapping_add(108))
            .wrapping_add(4)
            .cast::<*mut u8>())
        .read();
        {
            i = 0i32;
            'l1: loop {
                if !(i
                    < ((((((&raw mut gRfu).cast::<u8>()).wrapping_add(108))
                        .wrapping_add(2)
                        .cast::<u16>())
                    .read()) as i32))
                {
                    break 'l1;
                }
                'l2: {
                    if !((flags & 1u32) != 0) {
                        (((&raw mut sResendBlock16).cast::<u8>().cast::<u16>()).cast::<u16>())
                            .write(((35072i32 | i) as u16));
                        {
                            j = 0i32;
                            'l3: loop {
                                if !(j < 7i32) {
                                    break 'l3;
                                }
                                'l4: {
                                    temp = (j).wrapping_mul(2i32);
                                    ((((&raw mut sResendBlock16).cast::<u8>().cast::<u16>())
                                        .cast::<u16>())
                                    .wrapping_offset(((j).wrapping_add(1i32)) as isize))
                                    .write(
                                        (((((((payload).wrapping_offset(
                                            ((((12i32).wrapping_mul(i)).wrapping_add(temp))
                                                .wrapping_add(1i32))
                                                as isize,
                                        ))
                                        .read())
                                            as i32)
                                            << 8)
                                            | ((((payload).wrapping_offset(
                                                ((((12i32).wrapping_mul(i)).wrapping_add(temp))
                                                    .wrapping_add(0i32))
                                                    as isize,
                                            ))
                                            .read())
                                                as i32))
                                            as u16),
                                    );
                                }
                                j = (j).wrapping_add(1);
                            }
                        }
                        {
                            j = 0i32;
                            'l5: loop {
                                if !(j < 7i32) {
                                    break 'l5;
                                }
                                'l6: {
                                    temp = (j).wrapping_mul(2i32);
                                    ((((&raw mut sResendBlock8).cast::<u8>()).cast::<u8>())
                                        .wrapping_offset(((temp).wrapping_add(1i32)) as isize))
                                    .write(
                                        ((((((((&raw mut sResendBlock16)
                                            .cast::<u8>()
                                            .cast::<u16>())
                                        .cast::<u16>())
                                        .wrapping_offset((j) as isize))
                                        .read()) as i32)
                                            >> 8) as u8),
                                    );
                                    ((((&raw mut sResendBlock8).cast::<u8>()).cast::<u8>())
                                        .wrapping_offset(((temp).wrapping_add(0i32)) as isize))
                                    .write(
                                        ((((((&raw mut sResendBlock16)
                                            .cast::<u8>()
                                            .cast::<u16>())
                                        .cast::<u16>())
                                        .wrapping_offset((j) as isize))
                                        .read()) as u8),
                                    );
                                }
                                j = (j).wrapping_add(1);
                            }
                        }
                        RfuSendQueue_Enqueue(
                            ((&raw mut gRfu).cast::<u8>()).wrapping_add(2536),
                            ((&raw mut sResendBlock8).cast::<u8>()).cast::<u8>(),
                        );
                        let __p1 = (((&raw mut gRfu).cast::<u8>()).wrapping_add(108))
                            .wrapping_add(12)
                            .cast::<u32>();
                        (__p1).write(
                            ((__p1).read() | ((crate::c::shl_i32(1i32, ((i) as u32))) as u32)),
                        );
                    }
                    flags = (flags >> 1);
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Rfu_SetBlockReceivedFlag(linkPlayerId: u8) {
    unsafe {
        let mut linkPlayerId = linkPlayerId;
        if ((((((&raw mut gRfu).cast::<u8>()).wrapping_add(12)).read()) as i32) == 1i32)
            && ((linkPlayerId) != 0)
        {
            (((((&raw mut gRfu).cast::<u8>()).wrapping_add(97)).cast::<u8>())
                .wrapping_offset(((linkPlayerId) as i32) as isize))
            .write(1u8);
        } else {
            (((((&raw mut gRfu).cast::<u8>()).wrapping_add(92)).cast::<u8>())
                .wrapping_offset(((linkPlayerId) as i32) as isize))
            .write(1u8);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Rfu_ResetBlockReceivedFlag(linkPlayerId: u8) {
    unsafe {
        let mut linkPlayerId = linkPlayerId;
        (((((&raw mut gRfu).cast::<u8>()).wrapping_add(92)).cast::<u8>())
            .wrapping_offset(((linkPlayerId) as i32) as isize))
        .write(0u8);
        ((((((&raw mut gRfu).cast::<u8>()).wrapping_add(128)).cast::<u8>())
            .wrapping_offset(((linkPlayerId) as i32) as isize * 20))
        .wrapping_add(18))
        .write(0u8);
    }
}
pub(crate) unsafe extern "C" fn LoadLinkPlayerIds(ids: *mut u8) -> u8 {
    unsafe {
        let mut ids = ids;
        let mut i: u8 = 0u8;
        if (((((&raw mut gRfu).cast::<u8>()).wrapping_add(12)).read()) as i32) == 1i32 {
            return 0u8;
        }
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 4i32) {
                    break 'l1;
                }
                'l2: {
                    (((((&raw mut gRfu).cast::<u8>()).wrapping_add(3294)).cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize))
                    .write(((ids).wrapping_offset(((i) as i32) as isize)).read());
                }
                i = (i).wrapping_add(1);
            }
        }
        return ((ids).wrapping_offset(
            (((((&raw mut gRfu).cast::<u8>()).wrapping_add(3134)).read_volatile()) as i32) as isize,
        ))
        .read();
    }
}
pub(crate) unsafe extern "C" fn SendKeysToRfu() {
    unsafe {
        if (((((&raw mut gReceivedRemoteLinkPlayers).cast::<u8>()).read()) != 0)
            && (((((&raw mut gHeldKeyCodeToSend).cast::<u16>()).read()) as i32) != 0i32))
            && (((((&raw mut gLinkTransferringData).cast::<u8>()).read()) as i32) != 1i32)
        {
            let __p1 = (&raw mut sHeldKeyCount).cast::<u8>().cast::<u8>();
            (__p1).write(((__p1).read()).wrapping_add(1));
            let __p2 = (&raw mut gHeldKeyCodeToSend).cast::<u16>();
            (__p2).write(
                (((((__p2).read()) as i32)
                    | (((((&raw mut sHeldKeyCount).cast::<u8>().cast::<u8>()).read()) as i32) << 8))
                    as u16),
            );
            RfuPrepareSendBuffer(48640u16);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetHostRfuGameData() -> *mut u8 {
    unsafe {
        return (&raw mut gHostRfuGameData).cast::<u8>();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsSendingKeysToRfu() -> u32 {
    unsafe {
        return ((core::mem::transmute::<_, usize>(
            (((&raw mut gRfu).cast::<u8>()).cast::<Option<unsafe extern "C" fn()>>()).read(),
        ) == (SendKeysToRfu as *const () as usize)) as u32);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn StartSendingKeysToRfu() {
    unsafe {
        (((&raw mut gRfu).cast::<u8>()).cast::<Option<unsafe extern "C" fn()>>())
            .write(Some(SendKeysToRfu));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ClearLinkRfuCallback() {
    unsafe {
        (((&raw mut gRfu).cast::<u8>()).cast::<Option<unsafe extern "C" fn()>>()).write(None);
    }
}
pub(crate) unsafe extern "C" fn Rfu_BerryBlenderSendHeldKeys() {
    unsafe {
        RfuPrepareSendBuffer(17408u16);
        if ((GetMultiplayerId()) as i32) == 0i32 {
            ((((&raw mut gSendCmd).cast::<u16>()).cast::<u16>()).wrapping_offset(6))
                .write(GetBlenderArrowPosition());
        }
        let __p1 = (&raw mut gBerryBlenderKeySendAttempts).cast::<u32>();
        (__p1).write(((__p1).read()).wrapping_add(1));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Rfu_SetBerryBlenderLinkCallback() {
    unsafe {
        if core::mem::transmute::<_, usize>(
            (((&raw mut gRfu).cast::<u8>()).cast::<Option<unsafe extern "C" fn()>>()).read(),
        ) == 0usize
        {
            (((&raw mut gRfu).cast::<u8>()).cast::<Option<unsafe extern "C" fn()>>())
                .write(Some(Rfu_BerryBlenderSendHeldKeys));
        }
    }
}
pub(crate) unsafe extern "C" fn RfuHandleReceiveCommand(unused: u8) {
    unsafe {
        let mut unused = unused;
        let mut i: u16 = 0u16;
        let mut j: u16 = 0u16;
        {
            i = 0u16;
            'l1: loop {
                if !(((i) as i32) < 5i32) {
                    break 'l1;
                }
                'l2: {
                    'l3: {
                        let __sw1 = (((((((&raw mut gRecvCmds).cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 16))
                        .cast::<u16>())
                        .read()) as i32)
                            & 65280i32);
                        let mut __fall = false;
                        if __sw1 == 30720i32 {
                            __fall = true;
                            if ((((((&raw mut gRfu).cast::<u8>()).wrapping_add(12)).read()) as i32)
                                == 0i32)
                                && ((((&raw mut gReceivedRemoteLinkPlayers).cast::<u8>()).read())
                                    != 0)
                            {
                                return;
                            }
                        }
                        if __fall || __sw1 == 30464i32 {
                            __fall = true;
                            if (((((&raw mut gRfuLinkStatus).cast::<*mut u8>()).read()).read())
                                as i32)
                                == 0i32
                            {
                                (((&raw mut gRfu).cast::<u8>()).wrapping_add(13)).write(
                                    (((((((&raw mut gRecvCmds).cast::<u8>())
                                        .wrapping_offset(((i) as i32) as isize * 16))
                                    .cast::<u16>())
                                    .wrapping_offset(1))
                                    .read()) as u8),
                                );
                                (((&raw mut gRfu).cast::<u8>()).wrapping_add(3278)).write(
                                    LoadLinkPlayerIds(
                                        (((((&raw mut gRecvCmds).cast::<u8>())
                                            .wrapping_offset(((i) as i32) as isize * 16))
                                        .cast::<u16>())
                                        .wrapping_offset(2))
                                        .cast::<u8>(),
                                    ),
                                );
                            }
                            break 'l3;
                        }
                        if __sw1 == 34816i32 {
                            __fall = true;
                            if ((((((((&raw mut gRfu).cast::<u8>()).wrapping_add(128))
                                .cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 20))
                            .wrapping_add(18))
                            .read()) as i32)
                                == 0i32
                            {
                                ((((((&raw mut gRfu).cast::<u8>()).wrapping_add(128))
                                    .cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize * 20))
                                .cast::<u16>())
                                .write(0u16);
                                ((((((&raw mut gRfu).cast::<u8>()).wrapping_add(128))
                                    .cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize * 20))
                                .wrapping_add(2)
                                .cast::<u16>())
                                .write(
                                    (((((&raw mut gRecvCmds).cast::<u8>())
                                        .wrapping_offset(((i) as i32) as isize * 16))
                                    .cast::<u16>())
                                    .wrapping_offset(1))
                                    .read(),
                                );
                                ((((((&raw mut gRfu).cast::<u8>()).wrapping_add(128))
                                    .cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize * 20))
                                .wrapping_add(17))
                                .write(
                                    (((((((&raw mut gRecvCmds).cast::<u8>())
                                        .wrapping_offset(((i) as i32) as isize * 16))
                                    .cast::<u16>())
                                    .wrapping_offset(2))
                                    .read()) as u8),
                                );
                                ((((((&raw mut gRfu).cast::<u8>()).wrapping_add(128))
                                    .cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize * 20))
                                .wrapping_add(8)
                                .cast::<u32>())
                                .write(0u32);
                                ((((((&raw mut gRfu).cast::<u8>()).wrapping_add(128))
                                    .cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize * 20))
                                .wrapping_add(18))
                                .write(1u8);
                                (((((&raw mut gRfu).cast::<u8>()).wrapping_add(92)).cast::<u8>())
                                    .wrapping_offset(((i) as i32) as isize))
                                .write(0u8);
                            }
                            break 'l3;
                        }
                        if __sw1 == 35072i32 {
                            __fall = true;
                            if ((((((((&raw mut gRfu).cast::<u8>()).wrapping_add(128))
                                .cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 20))
                            .wrapping_add(18))
                            .read()) as i32)
                                == 1i32
                            {
                                ((((((&raw mut gRfu).cast::<u8>()).wrapping_add(128))
                                    .cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize * 20))
                                .cast::<u16>())
                                .write(
                                    ((((((((&raw mut gRecvCmds).cast::<u8>())
                                        .wrapping_offset(((i) as i32) as isize * 16))
                                    .cast::<u16>())
                                    .read()) as i32)
                                        & 255i32) as u16),
                                );
                                let __p2 = (((((&raw mut gRfu).cast::<u8>()).wrapping_add(128))
                                    .cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize * 20))
                                .wrapping_add(8)
                                .cast::<u32>();
                                (__p2).write(
                                    ((__p2).read()
                                        | ((crate::c::shl_i32(
                                            1i32,
                                            ((((((((&raw mut gRfu).cast::<u8>())
                                                .wrapping_add(128))
                                            .cast::<u8>())
                                            .wrapping_offset(((i) as i32) as isize * 20))
                                            .cast::<u16>())
                                            .read())
                                                as u32),
                                        )) as u32)),
                                );
                                {
                                    j = 0u16;
                                    'l4: loop {
                                        if !(((j) as i32) < 6i32) {
                                            break 'l4;
                                        }
                                        'l5: {
                                            (((((&raw mut gBlockRecvBuffer).cast::<u8>())
                                                .wrapping_offset(((i) as i32) as isize * 256))
                                            .cast::<u16>())
                                            .wrapping_offset(
                                                ((((((((((&raw mut gRfu).cast::<u8>())
                                                    .wrapping_add(128))
                                                .cast::<u8>())
                                                .wrapping_offset(((i) as i32) as isize * 20))
                                                .cast::<u16>())
                                                .read())
                                                    as i32)
                                                    .wrapping_mul(6i32))
                                                .wrapping_add(((j) as i32)))
                                                    as isize,
                                            ))
                                            .write(
                                                (((((&raw mut gRecvCmds).cast::<u8>())
                                                    .wrapping_offset(((i) as i32) as isize * 16))
                                                .cast::<u16>())
                                                .wrapping_offset(
                                                    (((j) as i32).wrapping_add(1i32)) as isize,
                                                ))
                                                .read(),
                                            );
                                        }
                                        j = (j).wrapping_add(1);
                                    }
                                }
                                if ((((((&raw mut gRfu).cast::<u8>()).wrapping_add(128))
                                    .cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize * 20))
                                .wrapping_add(8)
                                .cast::<u32>())
                                .read()
                                    == ((((&raw const sAllBlocksReceived)
                                        .cast::<u8>()
                                        .cast_mut()
                                        .cast::<u32>())
                                    .cast::<u32>())
                                    .wrapping_offset(
                                        ((((((((&raw mut gRfu).cast::<u8>()).wrapping_add(128))
                                            .cast::<u8>())
                                        .wrapping_offset(((i) as i32) as isize * 20))
                                        .wrapping_add(2)
                                        .cast::<u16>())
                                        .read()) as i32)
                                            as isize,
                                    ))
                                    .read()
                                {
                                    ((((((&raw mut gRfu).cast::<u8>()).wrapping_add(128))
                                        .cast::<u8>())
                                    .wrapping_offset(((i) as i32) as isize * 20))
                                    .wrapping_add(18))
                                    .write(2u8);
                                    Rfu_SetBlockReceivedFlag(((i) as u8));
                                    if ((((crate::c::bf_read(
                                        (GetHostRfuGameData()).wrapping_add(10),
                                        0,
                                        7,
                                        false,
                                    ) as u8) as i32)
                                        == 69i32)
                                        && ((((&raw mut gReceivedRemoteLinkPlayers).cast::<u8>())
                                            .read())
                                            != 0))
                                        && ((((((&raw mut gRfu).cast::<u8>()).wrapping_add(12))
                                            .read())
                                            as i32)
                                            == 0i32)
                                    {
                                        ValidateAndReceivePokemonSioInfo(
                                            (&raw mut gBlockRecvBuffer).cast::<u8>(),
                                        );
                                    }
                                }
                            }
                            break 'l3;
                        }
                        if __sw1 == 41216i32 {
                            __fall = true;
                            Rfu_InitBlockSend(
                                (((((&raw const sBlockRequests).cast::<u8>().cast_mut())
                                    .cast::<u8>())
                                .wrapping_offset(
                                    (((((((&raw mut gRecvCmds).cast::<u8>())
                                        .wrapping_offset(((i) as i32) as isize * 16))
                                    .cast::<u16>())
                                    .wrapping_offset(1))
                                    .read()) as i32) as isize
                                        * 8,
                                ))
                                .cast::<*mut u8>())
                                .read(),
                                ((((((((&raw const sBlockRequests).cast::<u8>().cast_mut())
                                    .cast::<u8>())
                                .wrapping_offset(
                                    (((((((&raw mut gRecvCmds).cast::<u8>())
                                        .wrapping_offset(((i) as i32) as isize * 16))
                                    .cast::<u16>())
                                    .wrapping_offset(1))
                                    .read()) as i32) as isize
                                        * 8,
                                ))
                                .wrapping_add(4)
                                .cast::<u32>())
                                .read()) as u16) as u32),
                            );
                            break 'l3;
                        }
                        if __sw1 == 24320i32 {
                            __fall = true;
                            (((((&raw mut gRfu).cast::<u8>()).wrapping_add(228)).cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize))
                            .write(1u8);
                            break 'l3;
                        }
                        if __sw1 == 26112i32 {
                            __fall = true;
                            if (((((&raw mut gRfu).cast::<u8>())
                                .wrapping_add(256)
                                .cast::<u16>())
                            .read()) as i32)
                                == (((((((&raw mut gRecvCmds).cast::<u8>())
                                    .wrapping_offset(((i) as i32) as isize * 16))
                                .cast::<u16>())
                                .wrapping_offset(1))
                                .read()) as i32)
                            {
                                (((((&raw mut gRfu).cast::<u8>()).wrapping_add(233)).cast::<u8>())
                                    .wrapping_offset(((i) as i32) as isize))
                                .write(1u8);
                            }
                            break 'l3;
                        }
                        if __sw1 == 60672i32 {
                            __fall = true;
                            if (((((&raw mut gRfu).cast::<u8>()).wrapping_add(12)).read()) as i32)
                                == 0i32
                            {
                                if (((&raw mut gReceivedRemoteLinkPlayers).cast::<u8>()).read())
                                    != 0
                                {
                                    if ((((((((&raw mut gRecvCmds).cast::<u8>())
                                        .wrapping_offset(((i) as i32) as isize * 16))
                                    .cast::<u16>())
                                    .wrapping_offset(1))
                                    .read()) as i32)
                                        & ((((((&raw mut gRfuLinkStatus).cast::<*mut u8>())
                                            .read())
                                        .wrapping_add(2))
                                        .read()) as i32))
                                        != 0
                                    {
                                        ((&raw mut gReceivedRemoteLinkPlayers).cast::<u8>())
                                            .write(0u8);
                                        rfu_LMAN_requestChangeAgbClockMaster();
                                        (((&raw mut gRfu).cast::<u8>()).wrapping_add(3300)).write(
                                            (((((((&raw mut gRecvCmds).cast::<u8>())
                                                .wrapping_offset(((i) as i32) as isize * 16))
                                            .cast::<u16>())
                                            .wrapping_offset(2))
                                            .read())
                                                as u8),
                                        );
                                    }
                                    (((&raw mut gRfu).cast::<u8>()).wrapping_add(13)).write(
                                        (((((((&raw mut gRecvCmds).cast::<u8>())
                                            .wrapping_offset(((i) as i32) as isize * 16))
                                        .cast::<u16>())
                                        .wrapping_offset(3))
                                        .read()) as u8),
                                    );
                                    ClearSelectedLinkPlayerIds(
                                        (((((&raw mut gRecvCmds).cast::<u8>())
                                            .wrapping_offset(((i) as i32) as isize * 16))
                                        .cast::<u16>())
                                        .wrapping_offset(1))
                                        .read(),
                                    );
                                }
                            } else {
                                RfuPrepareSendBuffer(60928u16);
                                ((((&raw mut gSendCmd).cast::<u16>()).cast::<u16>())
                                    .wrapping_offset(1))
                                .write(
                                    (((((&raw mut gRecvCmds).cast::<u8>())
                                        .wrapping_offset(((i) as i32) as isize * 16))
                                    .cast::<u16>())
                                    .wrapping_offset(1))
                                    .read(),
                                );
                                ((((&raw mut gSendCmd).cast::<u16>()).cast::<u16>())
                                    .wrapping_offset(2))
                                .write(
                                    (((((&raw mut gRecvCmds).cast::<u8>())
                                        .wrapping_offset(((i) as i32) as isize * 16))
                                    .cast::<u16>())
                                    .wrapping_offset(2))
                                    .read(),
                                );
                                ((((&raw mut gSendCmd).cast::<u16>()).cast::<u16>())
                                    .wrapping_offset(3))
                                .write(
                                    (((((&raw mut gRecvCmds).cast::<u8>())
                                        .wrapping_offset(((i) as i32) as isize * 16))
                                    .cast::<u16>())
                                    .wrapping_offset(3))
                                    .read(),
                                );
                            }
                            break 'l3;
                        }
                        if __sw1 == 60928i32 {
                            __fall = true;
                            if (((((&raw mut gRfu).cast::<u8>()).wrapping_add(12)).read()) as i32)
                                == 1i32
                            {
                                let __p3 = ((&raw mut gRfu).cast::<u8>()).wrapping_add(3299);
                                (__p3).write(
                                    (((((__p3).read()) as i32)
                                        | (((((((&raw mut gRecvCmds).cast::<u8>())
                                            .wrapping_offset(((i) as i32) as isize * 16))
                                        .cast::<u16>())
                                        .wrapping_offset(1))
                                        .read()) as i32))
                                        as u8),
                                );
                                (((&raw mut gRfu).cast::<u8>()).wrapping_add(3300)).write(
                                    (((((((&raw mut gRecvCmds).cast::<u8>())
                                        .wrapping_offset(((i) as i32) as isize * 16))
                                    .cast::<u16>())
                                    .wrapping_offset(2))
                                    .read()) as u8),
                                );
                                ClearSelectedLinkPlayerIds(
                                    (((((&raw mut gRecvCmds).cast::<u8>())
                                        .wrapping_offset(((i) as i32) as isize * 16))
                                    .cast::<u16>())
                                    .wrapping_offset(1))
                                    .read(),
                                );
                            }
                            break 'l3;
                        }
                        if __sw1 == 17408i32 || __sw1 == 48640i32 {
                            __fall = true;
                            ((((&raw mut gLinkPartnersHeldKeys).cast::<u16>()).cast::<u16>())
                                .wrapping_offset(((i) as i32) as isize))
                            .write(
                                (((((&raw mut gRecvCmds).cast::<u8>())
                                    .wrapping_offset(((i) as i32) as isize * 16))
                                .cast::<u16>())
                                .wrapping_offset(1))
                                .read(),
                            );
                            break 'l3;
                        }
                    }
                    if ((((((&raw mut gRfu).cast::<u8>()).wrapping_add(12)).read()) as i32) == 1i32)
                        && (((((((&raw mut gRfu).cast::<u8>()).wrapping_add(97)).cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize))
                        .read())
                            != 0)
                    {
                        if (((((((&raw mut gRfu).cast::<u8>()).wrapping_add(97)).cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize))
                        .read()) as i32)
                            == 4i32
                        {
                            (((((&raw mut gRfu).cast::<u8>()).wrapping_add(92)).cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize))
                            .write(1u8);
                            (((((&raw mut gRfu).cast::<u8>()).wrapping_add(97)).cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize))
                            .write(0u8);
                        } else {
                            let __p4 = ((((&raw mut gRfu).cast::<u8>()).wrapping_add(97))
                                .cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize);
                            (__p4).write(((__p4).read()).wrapping_add(1));
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn AreAllPlayersReadyToReceive() -> u8 {
    unsafe {
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 5i32) {
                    break 'l1;
                }
                'l2: {
                    if ((((((((&raw mut gRfu).cast::<u8>()).wrapping_add(128)).cast::<u8>())
                        .wrapping_offset((i) as isize * 20))
                    .wrapping_add(18))
                    .read()) as i32)
                        != 0i32
                    {
                        return 0u8;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn AreAllPlayersFinishedReceiving() -> u8 {
    unsafe {
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < (((((&raw mut gRfu).cast::<u8>()).wrapping_add(13)).read()) as i32)) {
                    break 'l1;
                }
                'l2: {
                    if (((((((((&raw mut gRfu).cast::<u8>()).wrapping_add(128)).cast::<u8>())
                        .wrapping_offset((i) as isize * 20))
                    .wrapping_add(18))
                    .read()) as i32)
                        != 2i32)
                        || ((((((((&raw mut gRfu).cast::<u8>()).wrapping_add(92)).cast::<u8>())
                            .wrapping_offset((i) as isize))
                        .read()) as i32)
                            != 1i32)
                    {
                        return 0u8;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn ResetSendDataManager(data: *mut u8) {
    unsafe {
        let mut data = data;
        ((data).cast::<u16>()).write(0u16);
        ((data).wrapping_add(2).cast::<u16>()).write(0u16);
        ((data).wrapping_add(4).cast::<*mut u8>()).write(core::ptr::null_mut());
        ((data).wrapping_add(8).cast::<u32>()).write(0u32);
        ((data).wrapping_add(16)).write(0u8);
        ((data).wrapping_add(17)).write(0u8);
        ((data).wrapping_add(18)).write(0u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Rfu_GetBlockReceivedStatus() -> u8 {
    unsafe {
        let mut flags: u8 = 0u8;
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 5i32) {
                    break 'l1;
                }
                'l2: {
                    if (((((((((&raw mut gRfu).cast::<u8>()).wrapping_add(128)).cast::<u8>())
                        .wrapping_offset((i) as isize * 20))
                    .wrapping_add(18))
                    .read()) as i32)
                        == 2i32)
                        && ((((((((&raw mut gRfu).cast::<u8>()).wrapping_add(92)).cast::<u8>())
                            .wrapping_offset((i) as isize))
                        .read()) as i32)
                            == 1i32)
                    {
                        flags = ((((flags) as i32) | crate::c::shl_i32(1i32, ((i) as u32))) as u8);
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return flags;
    }
}
pub(crate) unsafe extern "C" fn RfuPrepareSendBuffer(command: u16) {
    unsafe {
        let mut command = command;
        let mut i: u8 = 0u8;
        let mut buff: *mut u8 = core::ptr::null_mut();
        let mut tmp: u8 = 0u8;
        (((&raw mut gSendCmd).cast::<u16>()).cast::<u16>()).write(command);
        'l1: {
            let __sw1 = ((command) as i32);
            if __sw1 == 34816i32 {
                ((((&raw mut gSendCmd).cast::<u16>()).cast::<u16>()).wrapping_offset(1)).write(
                    ((((&raw mut gRfu).cast::<u8>()).wrapping_add(108))
                        .wrapping_add(2)
                        .cast::<u16>())
                    .read(),
                );
                ((((&raw mut gSendCmd).cast::<u16>()).cast::<u16>()).wrapping_offset(2)).write(
                    ((((((((&raw mut gRfu).cast::<u8>()).wrapping_add(108)).wrapping_add(17))
                        .read()) as i32)
                        .wrapping_add(128i32)) as u16),
                );
                break 'l1;
            }
            if __sw1 == 41216i32 {
                if (AreAllPlayersReadyToReceive()) != 0 {
                    ((((&raw mut gSendCmd).cast::<u16>()).cast::<u16>()).wrapping_offset(1))
                        .write((((((&raw mut gRfu).cast::<u8>()).wrapping_add(90)).read()) as u16));
                }
                break 'l1;
            }
            if __sw1 == 30464i32 || __sw1 == 30720i32 {
                tmp = (((((((&raw mut gRfu).cast::<u8>()).wrapping_add(3298)).read()) as i32)
                    ^ (((((&raw mut gRfu).cast::<u8>()).wrapping_add(3299)).read()) as i32))
                    as u8);
                (((&raw mut gRfu).cast::<u8>()).wrapping_add(13)).write(
                    ((((((((&raw const sPlayerBitsToCount).cast::<u8>().cast_mut()).cast::<u8>())
                        .wrapping_offset(((tmp) as i32) as isize))
                    .read()) as i32)
                        .wrapping_add(1i32)) as u8),
                );
                ((((&raw mut gSendCmd).cast::<u16>()).cast::<u16>()).wrapping_offset(1))
                    .write((((((&raw mut gRfu).cast::<u8>()).wrapping_add(13)).read()) as u16));
                buff = ((((&raw mut gSendCmd).cast::<u16>()).cast::<u16>()).wrapping_offset(2))
                    .cast::<u8>();
                {
                    i = 0u8;
                    'l2: loop {
                        if !(((i) as i32) < 4i32) {
                            break 'l2;
                        }
                        'l3: {
                            ((buff).wrapping_offset(((i) as i32) as isize)).write(
                                (((((&raw mut gRfu).cast::<u8>()).wrapping_add(3294))
                                    .cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize))
                                .read(),
                            );
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                break 'l1;
            }
            if __sw1 == 26112i32 || __sw1 == 24320i32 {
                ((((&raw mut gSendCmd).cast::<u16>()).cast::<u16>()).wrapping_offset(1)).write(
                    (((&raw mut gRfu).cast::<u8>())
                        .wrapping_add(256)
                        .cast::<u16>())
                    .read(),
                );
                break 'l1;
            }
            if __sw1 == 17408i32 {
                (((&raw mut gSendCmd).cast::<u16>()).cast::<u16>()).write(command);
                ((((&raw mut gSendCmd).cast::<u16>()).cast::<u16>()).wrapping_offset(1)).write(
                    (((&raw mut gMain).cast::<u8>())
                        .wrapping_add(44)
                        .cast::<u16>())
                    .read(),
                );
                break 'l1;
            }
            if __sw1 == 12032i32 {
                {
                    i = 0u8;
                    'l4: loop {
                        if !(((i) as i32) < 6i32) {
                            break 'l4;
                        }
                        'l5: {
                            ((((&raw mut gSendCmd).cast::<u16>()).cast::<u16>())
                                .wrapping_offset(((1i32).wrapping_add(((i) as i32))) as isize))
                            .write(
                                (((((&raw mut gRfu).cast::<u8>()).wrapping_add(242))
                                    .cast::<u16>())
                                .wrapping_offset(((i) as i32) as isize))
                                .read(),
                            );
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                break 'l1;
            }
            if __sw1 == 48640i32 {
                ((((&raw mut gSendCmd).cast::<u16>()).cast::<u16>()).wrapping_offset(1))
                    .write(((&raw mut gHeldKeyCodeToSend).cast::<u16>()).read());
                break 'l1;
            }
            if __sw1 == 60928i32 || __sw1 == 60672i32 {
                break 'l1;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Rfu_SendPacket(data: *mut u8) {
    unsafe {
        let mut data = data;
        if ((((((&raw mut gSendCmd).cast::<u16>()).cast::<u16>()).read()) as i32) == 0i32)
            && (!((RfuHasErrored()) != 0))
        {
            crate::c::memcpy(
                ((((&raw mut gRfu).cast::<u8>()).wrapping_add(242)).cast::<u16>()).cast::<u8>(),
                data,
                12u32,
            );
            RfuPrepareSendBuffer(12032u16);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Rfu_InitBlockSend(src: *mut u8, size: u32) -> u32 {
    unsafe {
        let mut src = src;
        let mut size = size;
        let mut r4: u8 = 0u8;
        if core::mem::transmute::<_, usize>(
            (((&raw mut gRfu).cast::<u8>()).cast::<Option<unsafe extern "C" fn()>>()).read(),
        ) != 0usize
        {
            return 0u32;
        }
        if (((((&raw mut gSendCmd).cast::<u16>()).cast::<u16>()).read()) as i32) != 0i32 {
            return 0u32;
        }
        if (((((&raw mut gRfu).cast::<u8>()).wrapping_add(108)).wrapping_add(16)).read()) != 0 {
            let __p1 = ((&raw mut sRfuDebug).cast::<u8>()).wrapping_add(131);
            (__p1).write(((__p1).read()).wrapping_add(1));
            return 0u32;
        }
        r4 = ((crate::c::rem_u32(size, 12u32) != 0u32) as u8);
        ((((&raw mut gRfu).cast::<u8>()).wrapping_add(108)).wrapping_add(17))
            .write(GetMultiplayerId());
        ((((&raw mut gRfu).cast::<u8>()).wrapping_add(108)).wrapping_add(16)).write(1u8);
        ((((&raw mut gRfu).cast::<u8>()).wrapping_add(108))
            .wrapping_add(2)
            .cast::<u16>())
        .write((((crate::c::div_u32(size, 12u32)).wrapping_add(((r4) as u32))) as u16));
        ((((&raw mut gRfu).cast::<u8>()).wrapping_add(108)).cast::<u16>()).write(0u16);
        if size > 256u32 {
            ((((&raw mut gRfu).cast::<u8>()).wrapping_add(108))
                .wrapping_add(4)
                .cast::<*mut u8>())
            .write(src);
        } else {
            if ((src) as usize) != (((&raw mut gBlockSendBuffer).cast::<u8>()) as usize) {
                crate::c::memcpy((&raw mut gBlockSendBuffer).cast::<u8>(), src, size);
            }
            ((((&raw mut gRfu).cast::<u8>()).wrapping_add(108))
                .wrapping_add(4)
                .cast::<*mut u8>())
            .write((&raw mut gBlockSendBuffer).cast::<u8>());
        }
        RfuPrepareSendBuffer(34816u16);
        (((&raw mut gRfu).cast::<u8>()).cast::<Option<unsafe extern "C" fn()>>())
            .write(Some(HandleBlockSend));
        (((&raw mut gRfu).cast::<u8>()).wrapping_add(91)).write(0u8);
        return 1u32;
    }
}
pub(crate) unsafe extern "C" fn HandleBlockSend() {
    unsafe {
        if (((((&raw mut gSendCmd).cast::<u16>()).cast::<u16>()).read()) as i32) == 0i32 {
            RfuPrepareSendBuffer(34816u16);
            if (((((&raw mut gRfu).cast::<u8>()).wrapping_add(12)).read()) as i32) == 1i32 {
                if (({
                    let __p1 = ((&raw mut gRfu).cast::<u8>()).wrapping_add(91);
                    let __t2 = ((__p1).read()).wrapping_add(1);
                    (__p1).write(__t2);
                    __t2
                }) as i32)
                    > 2i32
                {
                    (((&raw mut gRfu).cast::<u8>()).cast::<Option<unsafe extern "C" fn()>>())
                        .write(Some(SendNextBlock));
                }
            } else {
                if (((((((&raw mut gRecvCmds).cast::<u8>())
                    .wrapping_offset(((GetMultiplayerId()) as i32) as isize * 16))
                .cast::<u16>())
                .read()) as i32)
                    & 65280i32)
                    == 34816i32
                {
                    (((&raw mut gRfu).cast::<u8>()).cast::<Option<unsafe extern "C" fn()>>())
                        .write(Some(SendNextBlock));
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SendNextBlock() {
    unsafe {
        let mut i: i32 = 0i32;
        let mut src: *mut u8 = ((((&raw mut gRfu).cast::<u8>()).wrapping_add(108))
            .wrapping_add(4)
            .cast::<*mut u8>())
        .read();
        (((&raw mut gSendCmd).cast::<u16>()).cast::<u16>()).write(
            ((35072i32
                | ((((((&raw mut gRfu).cast::<u8>()).wrapping_add(108)).cast::<u16>()).read())
                    as i32)) as u16),
        );
        {
            i = 0i32;
            'l1: loop {
                if !(i < 7i32) {
                    break 'l1;
                }
                'l2: {
                    ((((&raw mut gSendCmd).cast::<u16>()).cast::<u16>())
                        .wrapping_offset(((i).wrapping_add(1i32)) as isize))
                    .write(
                        (((((((src).wrapping_offset(
                            (((i << 1).wrapping_add(
                                ((((((&raw mut gRfu).cast::<u8>()).wrapping_add(108))
                                    .cast::<u16>())
                                .read()) as i32)
                                    .wrapping_mul(12i32),
                            ))
                            .wrapping_add(1i32)) as isize,
                        ))
                        .read()) as i32)
                            << 8)
                            | ((((src).wrapping_offset(
                                (((i << 1).wrapping_add(
                                    ((((((&raw mut gRfu).cast::<u8>()).wrapping_add(108))
                                        .cast::<u16>())
                                    .read()) as i32)
                                        .wrapping_mul(12i32),
                                ))
                                .wrapping_add(0i32)) as isize,
                            ))
                            .read()) as i32)) as u16),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        let __p1 = (((&raw mut gRfu).cast::<u8>()).wrapping_add(108)).cast::<u16>();
        (__p1).write(((__p1).read()).wrapping_add(1));
        if ((((((&raw mut gRfu).cast::<u8>()).wrapping_add(108))
            .wrapping_add(2)
            .cast::<u16>())
        .read()) as i32)
            <= ((((((&raw mut gRfu).cast::<u8>()).wrapping_add(108)).cast::<u16>()).read()) as i32)
        {
            ((((&raw mut gRfu).cast::<u8>()).wrapping_add(108)).wrapping_add(16)).write(0u8);
            (((&raw mut gRfu).cast::<u8>()).cast::<Option<unsafe extern "C" fn()>>())
                .write(Some(SendLastBlock));
        }
    }
}
pub(crate) unsafe extern "C" fn SendLastBlock() {
    unsafe {
        let mut src: *mut u8 = ((((&raw mut gRfu).cast::<u8>()).wrapping_add(108))
            .wrapping_add(4)
            .cast::<*mut u8>())
        .read();
        let mut mpId: u8 = GetMultiplayerId();
        let mut i: i32 = 0i32;
        if (((((&raw mut gRfu).cast::<u8>()).wrapping_add(12)).read()) as i32) == 0i32 {
            (((&raw mut gSendCmd).cast::<u16>()).cast::<u16>()).write(
                ((35072i32
                    | ((((((&raw mut gRfu).cast::<u8>()).wrapping_add(108))
                        .wrapping_add(2)
                        .cast::<u16>())
                    .read()) as i32)
                        .wrapping_sub(1i32)) as u16),
            );
            {
                i = 0i32;
                'l1: loop {
                    if !(i < 7i32) {
                        break 'l1;
                    }
                    'l2: {
                        ((((&raw mut gSendCmd).cast::<u16>()).cast::<u16>())
                            .wrapping_offset(((i).wrapping_add(1i32)) as isize))
                        .write(
                            (((((((src).wrapping_offset(
                                (((i << 1).wrapping_add(
                                    (((((((&raw mut gRfu).cast::<u8>()).wrapping_add(108))
                                        .wrapping_add(2)
                                        .cast::<u16>())
                                    .read()) as i32)
                                        .wrapping_sub(1i32))
                                    .wrapping_mul(12i32),
                                ))
                                .wrapping_add(1i32)) as isize,
                            ))
                            .read()) as i32)
                                << 8)
                                | ((((src).wrapping_offset(
                                    (((i << 1).wrapping_add(
                                        (((((((&raw mut gRfu).cast::<u8>()).wrapping_add(108))
                                            .wrapping_add(2)
                                            .cast::<u16>())
                                        .read()) as i32)
                                            .wrapping_sub(1i32))
                                        .wrapping_mul(12i32),
                                    ))
                                    .wrapping_add(0i32))
                                        as isize,
                                ))
                                .read()) as i32)) as u16),
                        );
                    }
                    i = (i).wrapping_add(1);
                }
            }
            if (((((((&raw mut gRecvCmds).cast::<u8>())
                .wrapping_offset(((mpId) as i32) as isize * 16))
            .cast::<u16>())
            .read()) as u8) as i32)
                == ((((((&raw mut gRfu).cast::<u8>()).wrapping_add(108))
                    .wrapping_add(2)
                    .cast::<u16>())
                .read()) as i32)
                    .wrapping_sub(1i32)
            {
                if ((((((&raw mut gRfu).cast::<u8>()).wrapping_add(128)).cast::<u8>())
                    .wrapping_offset(((mpId) as i32) as isize * 20))
                .wrapping_add(8)
                .cast::<u32>())
                .read()
                    != ((((&raw const sAllBlocksReceived)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<u32>())
                    .cast::<u32>())
                    .wrapping_offset(
                        ((((((((&raw mut gRfu).cast::<u8>()).wrapping_add(128)).cast::<u8>())
                            .wrapping_offset(((mpId) as i32) as isize * 20))
                        .wrapping_add(2)
                        .cast::<u16>())
                        .read()) as i32) as isize,
                    ))
                    .read()
                {
                    HandleSendFailure(
                        mpId,
                        ((((((&raw mut gRfu).cast::<u8>()).wrapping_add(128)).cast::<u8>())
                            .wrapping_offset(((mpId) as i32) as isize * 20))
                        .wrapping_add(8)
                        .cast::<u32>())
                        .read(),
                    );
                    let __p1 = ((&raw mut sRfuDebug).cast::<u8>())
                        .wrapping_add(100)
                        .cast::<u16>();
                    (__p1).write(((__p1).read()).wrapping_add(1));
                } else {
                    (((&raw mut gRfu).cast::<u8>()).cast::<Option<unsafe extern "C" fn()>>())
                        .write(None);
                }
            }
        } else {
            (((&raw mut gRfu).cast::<u8>()).cast::<Option<unsafe extern "C" fn()>>()).write(None);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Rfu_SendBlockRequest(r#type: u8) -> u8 {
    unsafe {
        let mut r#type = r#type;
        (((&raw mut gRfu).cast::<u8>()).wrapping_add(90)).write(r#type);
        RfuPrepareSendBuffer(41216u16);
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn RfuShutdownAfterDisconnect() {
    unsafe {
        rfu_clearAllSlot();
        rfu_LMAN_powerDownRFU();
        ((&raw mut gReceivedRemoteLinkPlayers).cast::<u8>()).write(0u8);
        (((&raw mut gRfu).cast::<u8>()).wrapping_add(239)).write(1u8);
        (((&raw mut gRfu).cast::<u8>()).cast::<Option<unsafe extern "C" fn()>>()).write(None);
    }
}
pub(crate) unsafe extern "C" fn DisconnectRfu() {
    unsafe {
        rfu_REQ_disconnect(
            ((((((((&raw mut gRfuLinkStatus).cast::<*mut u8>()).read()).wrapping_add(2)).read())
                as i32)
                | ((((((&raw mut gRfuLinkStatus).cast::<*mut u8>()).read()).wrapping_add(3)).read())
                    as i32)) as u8),
        );
        rfu_waitREQComplete();
        RfuShutdownAfterDisconnect();
    }
}
pub(crate) unsafe extern "C" fn TryDisconnectRfu() {
    unsafe {
        if (((((&raw mut gRfu).cast::<u8>()).wrapping_add(12)).read()) as i32) == 0i32 {
            rfu_LMAN_requestChangeAgbClockMaster();
            (((&raw mut gRfu).cast::<u8>()).wrapping_add(3300)).write(2u8);
        } else {
            (((&raw mut gRfu).cast::<u8>()).cast::<Option<unsafe extern "C" fn()>>())
                .write(Some(DisconnectRfu));
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn LinkRfu_FatalError() {
    unsafe {
        rfu_LMAN_requestChangeAgbClockMaster();
        (((&raw mut gRfu).cast::<u8>()).wrapping_add(3300)).write(1u8);
        (((&raw mut gRfu).cast::<u8>()).wrapping_add(3299)).write(
            ((((((((&raw mut gRfuLinkStatus).cast::<*mut u8>()).read()).wrapping_add(2)).read())
                as i32)
                | ((((((&raw mut gRfuLinkStatus).cast::<*mut u8>()).read()).wrapping_add(3)).read())
                    as i32)) as u8),
        );
    }
}
pub(crate) unsafe extern "C" fn WaitAllReadyToCloseLink() {
    unsafe {
        let mut i: i32 = 0i32;
        let mut playerCount: u8 = (((&raw mut gRfu).cast::<u8>()).wrapping_add(13)).read();
        let mut count: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 5i32) {
                    break 'l1;
                }
                'l2: {
                    if ((((((&raw mut gRfu).cast::<u8>()).wrapping_add(228)).cast::<u8>())
                        .wrapping_offset((i) as isize))
                    .read())
                        != 0
                    {
                        count = (count).wrapping_add(1);
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        if count == ((playerCount) as i32) {
            let __p1 = (&raw mut gBattleTypeFlags).cast::<u32>();
            (__p1).write(((__p1).read() & 4294967263u32));
            if (((((&raw mut gRfu).cast::<u8>()).wrapping_add(12)).read()) as i32) == 0i32 {
                crate::c::volatile_write(((&raw mut gRfu).cast::<u8>()).wrapping_add(238), 3u8);
                TryDisconnectRfu();
            } else {
                (((&raw mut gRfu).cast::<u8>()).cast::<Option<unsafe extern "C" fn()>>())
                    .write(Some(TryDisconnectRfu));
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SendReadyCloseLink() {
    unsafe {
        if ((((((&raw mut gSendCmd).cast::<u16>()).cast::<u16>()).read()) as i32) == 0i32)
            && (!(((((&raw mut gRfu).cast::<u8>()).wrapping_add(3304)).read()) != 0))
        {
            RfuPrepareSendBuffer(24320u16);
            (((&raw mut gRfu).cast::<u8>()).cast::<Option<unsafe extern "C" fn()>>())
                .write(Some(WaitAllReadyToCloseLink));
        }
    }
}
pub(crate) unsafe extern "C" fn Task_TryReadyCloseLink(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if core::mem::transmute::<_, usize>(
            (((&raw mut gRfu).cast::<u8>()).cast::<Option<unsafe extern "C" fn()>>()).read(),
        ) == 0usize
        {
            (((&raw mut gRfu).cast::<u8>()).wrapping_add(3289)).write(1u8);
            (((&raw mut gRfu).cast::<u8>()).cast::<Option<unsafe extern "C" fn()>>())
                .write(Some(SendReadyCloseLink));
            DestroyTask(taskId);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Rfu_SetCloseLinkCallback() {
    unsafe {
        if !((FuncIsActiveTask(Some(Task_TryReadyCloseLink))) != 0) {
            CreateTask(Some(Task_TryReadyCloseLink), 5u8);
        }
    }
}
pub(crate) unsafe extern "C" fn SendReadyExitStandbyUntilAllReady() {
    unsafe {
        let mut playerCount: u8 = 0u8;
        let mut i: u8 = 0u8;
        if ((GetMultiplayerId()) as i32) != 0i32 {
            if (((((((&raw mut gRfu).cast::<u8>()).wrapping_add(292)).wrapping_add(2242))
                .read_volatile()) as i32)
                == 0i32)
                && ((((((&raw mut gRfu).cast::<u8>())
                    .wrapping_add(254)
                    .cast::<u16>())
                .read()) as i32)
                    > 60i32)
            {
                RfuPrepareSendBuffer(26112u16);
                (((&raw mut gRfu).cast::<u8>())
                    .wrapping_add(254)
                    .cast::<u16>())
                .write(0u16);
            }
        }
        playerCount = GetLinkPlayerCount();
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < ((playerCount) as i32)) {
                    break 'l1;
                }
                'l2: {
                    if !(((((((&raw mut gRfu).cast::<u8>()).wrapping_add(233)).cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize))
                    .read())
                        != 0)
                    {
                        break 'l1;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        if ((i) as i32) == ((playerCount) as i32) {
            {
                i = 0u8;
                'l3: loop {
                    if !(((i) as i32) < 5i32) {
                        break 'l3;
                    }
                    'l4: {
                        (((((&raw mut gRfu).cast::<u8>()).wrapping_add(233)).cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize))
                        .write(0u8);
                    }
                    i = (i).wrapping_add(1);
                }
            }
            let __p1 = ((&raw mut gRfu).cast::<u8>())
                .wrapping_add(256)
                .cast::<u16>();
            (__p1).write(((__p1).read()).wrapping_add(1));
            (((&raw mut gRfu).cast::<u8>()).cast::<Option<unsafe extern "C" fn()>>()).write(None);
        }
        let __p2 = ((&raw mut gRfu).cast::<u8>())
            .wrapping_add(254)
            .cast::<u16>();
        (__p2).write(((__p2).read()).wrapping_add(1));
    }
}
pub(crate) unsafe extern "C" fn LinkLeaderReadyToExitStandby() {
    unsafe {
        if (((((((&raw mut gRfu).cast::<u8>()).wrapping_add(292)).wrapping_add(2242))
            .read_volatile()) as i32)
            == 0i32)
            && ((((((&raw mut gSendCmd).cast::<u16>()).cast::<u16>()).read()) as i32) == 0i32)
        {
            RfuPrepareSendBuffer(26112u16);
            (((&raw mut gRfu).cast::<u8>()).cast::<Option<unsafe extern "C" fn()>>())
                .write(Some(SendReadyExitStandbyUntilAllReady));
        }
    }
}
pub(crate) unsafe extern "C" fn Rfu_LinkStandby() {
    unsafe {
        let mut i: u8 = 0u8;
        let mut playerCount: u8 = 0u8;
        if ((GetMultiplayerId()) as i32) != 0i32 {
            if (((((((&raw mut gRfu).cast::<u8>()).wrapping_add(292)).wrapping_add(2242))
                .read_volatile()) as i32)
                == 0i32)
                && ((((((&raw mut gSendCmd).cast::<u16>()).cast::<u16>()).read()) as i32) == 0i32)
            {
                RfuPrepareSendBuffer(26112u16);
                (((&raw mut gRfu).cast::<u8>()).cast::<Option<unsafe extern "C" fn()>>())
                    .write(Some(SendReadyExitStandbyUntilAllReady));
            }
        } else {
            playerCount = GetLinkPlayerCount();
            {
                i = 1u8;
                'l1: loop {
                    if !(((i) as i32) < ((playerCount) as i32)) {
                        break 'l1;
                    }
                    'l2: {
                        if !(((((((&raw mut gRfu).cast::<u8>()).wrapping_add(233)).cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize))
                        .read())
                            != 0)
                        {
                            break 'l1;
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
            if ((i) as i32) == ((playerCount) as i32) {
                if (((((((&raw mut gRfu).cast::<u8>()).wrapping_add(292)).wrapping_add(2242))
                    .read_volatile()) as i32)
                    == 0i32)
                    && ((((((&raw mut gSendCmd).cast::<u16>()).cast::<u16>()).read()) as i32)
                        == 0i32)
                {
                    RfuPrepareSendBuffer(26112u16);
                    (((&raw mut gRfu).cast::<u8>()).cast::<Option<unsafe extern "C" fn()>>())
                        .write(Some(LinkLeaderReadyToExitStandby));
                }
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Rfu_SetLinkStandbyCallback() {
    unsafe {
        if core::mem::transmute::<_, usize>(
            (((&raw mut gRfu).cast::<u8>()).cast::<Option<unsafe extern "C" fn()>>()).read(),
        ) == 0usize
        {
            (((&raw mut gRfu).cast::<u8>()).cast::<Option<unsafe extern "C" fn()>>())
                .write(Some(Rfu_LinkStandby));
            (((&raw mut gRfu).cast::<u8>())
                .wrapping_add(254)
                .cast::<u16>())
            .write(0u16);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsRfuSerialNumberValid(serialNo: u32) -> u32 {
    unsafe {
        let mut serialNo = serialNo;
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(((((((&raw const sAcceptedSerialNos)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<u16>())
                .cast::<u16>())
                .wrapping_offset((i) as isize))
                .read()) as u32)
                    != serialNo)
                {
                    break 'l1;
                }
                'l2: {
                    if ((((((&raw const sAcceptedSerialNos)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<u16>())
                    .cast::<u16>())
                    .wrapping_offset((i) as isize))
                    .read()) as i32)
                        == 65535i32
                    {
                        return 0u32;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return 1u32;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Rfu_SetLinkRecovery(enable: u32) -> u8 {
    unsafe {
        let mut enable = enable;
        if enable == 0u32 {
            return rfu_LMAN_setLinkRecovery(0u8, 0u16);
        }
        rfu_LMAN_setLinkRecovery(1u8, 600u16);
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Rfu_StopPartnerSearch() {
    unsafe {
        (((&raw mut gRfu).cast::<u8>()).wrapping_add(3289)).write(1u8);
        rfu_LMAN_stopManager(0u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Rfu_GetMultiplayerId() -> u8 {
    unsafe {
        if (((((&raw mut gRfu).cast::<u8>()).wrapping_add(12)).read()) as i32) == 1i32 {
            return 0u8;
        }
        return (((&raw mut gRfu).cast::<u8>()).wrapping_add(3278)).read();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Rfu_GetLinkPlayerCount() -> u8 {
    unsafe {
        return (((&raw mut gRfu).cast::<u8>()).wrapping_add(13)).read();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsLinkRfuTaskFinished() -> u8 {
    unsafe {
        if (((((&raw mut gRfu).cast::<u8>()).wrapping_add(241)).read()) as i32) == 2i32 {
            return 0u8;
        }
        return ((if ((((&raw mut gRfu).cast::<u8>()).cast::<Option<unsafe extern "C" fn()>>())
            .read())
        .is_some()
        {
            0i32
        } else {
            1i32
        }) as u8);
    }
}
pub(crate) unsafe extern "C" fn CallRfuFunc() {
    unsafe {
        if ((((&raw mut gRfu).cast::<u8>()).cast::<Option<unsafe extern "C" fn()>>()).read())
            .is_some()
        {
            ((((&raw mut gRfu).cast::<u8>()).cast::<Option<unsafe extern "C" fn()>>()).read())
                .unwrap_unchecked()();
        }
    }
}
pub(crate) unsafe extern "C" fn CheckForLeavingGroupMembers() -> u8 {
    unsafe {
        let mut i: i32 = 0i32;
        let mut memberLeft: u8 = 0u8;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 4i32) {
                    break 'l1;
                }
                'l2: {
                    if ((((((((&raw mut gRfu).cast::<u8>()).wrapping_add(3281)).cast::<u8>())
                        .wrapping_offset((i) as isize))
                    .read()) as i32)
                        < 5i32)
                        || ((((((((&raw mut gRfu).cast::<u8>()).wrapping_add(3281)).cast::<u8>())
                            .wrapping_offset((i) as isize))
                        .read()) as i32)
                            > 6i32)
                    {
                        if ((((((((((&raw mut gRfuSlotStatusNI).cast::<*mut u8>())
                            .cast::<*mut u8>())
                        .wrapping_offset((i) as isize))
                        .read())
                        .wrapping_add(52))
                        .cast::<u16>())
                        .read()) as i32)
                            == 70i32)
                            || ((((((((((&raw mut gRfuSlotStatusNI).cast::<*mut u8>())
                                .cast::<*mut u8>())
                            .wrapping_offset((i) as isize))
                            .read())
                            .wrapping_add(52))
                            .cast::<u16>())
                            .read()) as i32)
                                == 72i32)
                        {
                            if (((((((&raw mut gRfu).cast::<u8>()).wrapping_add(3285))
                                .cast::<u8>())
                            .wrapping_offset((i) as isize))
                            .read()) as i32)
                                == 8i32
                            {
                                (((((&raw mut gRfu).cast::<u8>()).wrapping_add(3281))
                                    .cast::<u8>())
                                .wrapping_offset((i) as isize))
                                .write(9u8);
                                (((((&raw mut gRfu).cast::<u8>()).wrapping_add(3285))
                                    .cast::<u8>())
                                .wrapping_offset((i) as isize))
                                .write(10u8);
                                rfu_clearSlot(8u8, ((i) as u8));
                                rfu_NI_setSendData(
                                    ((crate::c::shl_i32(1i32, ((i) as u32))) as u8),
                                    8u8,
                                    ((((&raw mut gRfu).cast::<u8>()).wrapping_add(3281))
                                        .cast::<u8>())
                                    .wrapping_offset((i) as isize),
                                    1u32,
                                );
                                memberLeft = 1u8;
                            }
                        } else {
                            if (((((((((&raw mut gRfuSlotStatusNI).cast::<*mut u8>())
                                .cast::<*mut u8>())
                            .wrapping_offset(
                                (((((&raw mut gRfu).cast::<u8>()).wrapping_add(3134))
                                    .read_volatile()) as i32)
                                    as isize,
                            ))
                            .read())
                            .wrapping_add(52))
                            .cast::<u16>())
                            .read()) as i32)
                                == 71i32
                            {
                                rfu_clearSlot(8u8, ((i) as u8));
                            }
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return memberLeft;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn RfuTryDisconnectLeavingChildren() -> u32 {
    unsafe {
        let mut childrenLeaving: u8 = 0u8;
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 4i32) {
                    break 'l1;
                }
                'l2: {
                    if (((((((&raw mut gRfu).cast::<u8>()).wrapping_add(3285)).cast::<u8>())
                        .wrapping_offset((i) as isize))
                    .read()) as i32)
                        == 11i32
                    {
                        childrenLeaving = ((((childrenLeaving) as i32)
                            | crate::c::shl_i32(1i32, ((i) as u32)))
                            as u8);
                        (((((&raw mut gRfu).cast::<u8>()).wrapping_add(3285)).cast::<u8>())
                            .wrapping_offset((i) as isize))
                        .write(0u8);
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        if (childrenLeaving) != 0 {
            rfu_REQ_disconnect(childrenLeaving);
            rfu_waitREQComplete();
        }
        {
            i = 0i32;
            'l3: loop {
                if !(i < 4i32) {
                    break 'l3;
                }
                'l4: {
                    if ((((((((&raw mut gRfu).cast::<u8>()).wrapping_add(3285)).cast::<u8>())
                        .wrapping_offset((i) as isize))
                    .read()) as i32)
                        == 10i32)
                        || ((((((((&raw mut gRfu).cast::<u8>()).wrapping_add(3285)).cast::<u8>())
                            .wrapping_offset((i) as isize))
                        .read()) as i32)
                            == 11i32)
                    {
                        return 1u32;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return 0u32;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn HasTrainerLeftPartnersList(trainerId: u16, name: *mut u8) -> u32 {
    unsafe {
        let mut trainerId = trainerId;
        let mut name = name;
        let mut idx: u8 = GetPartnerIndexByNameAndTrainerID(name, trainerId);
        if ((idx) as i32) == 255i32 {
            return 1u32;
        }
        if (((((((&raw mut gRfu).cast::<u8>()).wrapping_add(3281)).cast::<u8>())
            .wrapping_offset(((idx) as i32) as isize))
        .read()) as i32)
            == 9i32
        {
            return 1u32;
        }
        return 0u32;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SendRfuStatusToPartner(status: u8, trainerId: u16, name: *mut u8) {
    unsafe {
        let mut status = status;
        let mut trainerId = trainerId;
        let mut name = name;
        let mut idx: u8 = GetPartnerIndexByNameAndTrainerID(name, trainerId);
        (((((&raw mut gRfu).cast::<u8>()).wrapping_add(3281)).cast::<u8>())
            .wrapping_offset(((idx) as i32) as isize))
        .write(status);
        rfu_clearSlot(4u8, idx);
        rfu_NI_setSendData(
            ((crate::c::shl_i32(1i32, ((idx) as u32))) as u8),
            8u8,
            ((((&raw mut gRfu).cast::<u8>()).wrapping_add(3281)).cast::<u8>())
                .wrapping_offset(((idx) as i32) as isize),
            1u32,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SendLeaveGroupNotice() {
    unsafe {
        (((&raw mut gRfu).cast::<u8>()).wrapping_add(3205)).write(8u8);
        rfu_clearSlot(
            4u8,
            (((&raw mut gRfu).cast::<u8>()).wrapping_add(3134)).read_volatile(),
        );
        rfu_NI_setSendData(
            ((crate::c::shl_i32(
                1i32,
                (((((&raw mut gRfu).cast::<u8>()).wrapping_add(3134)).read_volatile()) as u32),
            )) as u8),
            8u8,
            ((&raw mut gRfu).cast::<u8>()).wrapping_add(3205),
            1u32,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn WaitSendRfuStatusToPartner(trainerId: u16, name: *mut u8) -> u32 {
    unsafe {
        let mut trainerId = trainerId;
        let mut name = name;
        let mut idx: u8 = GetPartnerIndexByNameAndTrainerID(name, trainerId);
        if ((idx) as i32) == 255i32 {
            return 2u32;
        }
        if ((((((((&raw mut gRfuSlotStatusNI).cast::<*mut u8>()).cast::<*mut u8>())
            .wrapping_offset(((idx) as i32) as isize))
        .read())
        .cast::<u16>())
        .read()) as i32)
            == 0i32
        {
            return 1u32;
        }
        return 0u32;
    }
}
pub(crate) unsafe extern "C" fn UpdateChildStatuses() {
    unsafe {
        let mut i: i32 = 0i32;
        CheckForLeavingGroupMembers();
        {
            i = 0i32;
            'l1: loop {
                if !(i < 4i32) {
                    break 'l1;
                }
                'l2: {
                    if (((((((((&raw mut gRfuSlotStatusNI).cast::<*mut u8>()).cast::<*mut u8>())
                        .wrapping_offset((i) as isize))
                    .read())
                    .cast::<u16>())
                    .read()) as i32)
                        == 38i32)
                        || (((((((((&raw mut gRfuSlotStatusNI).cast::<*mut u8>())
                            .cast::<*mut u8>())
                        .wrapping_offset((i) as isize))
                        .read())
                        .cast::<u16>())
                        .read()) as i32)
                            == 39i32)
                    {
                        if (((((((&raw mut gRfu).cast::<u8>()).wrapping_add(3285)).cast::<u8>())
                            .wrapping_offset((i) as isize))
                        .read()) as i32)
                            == 10i32
                        {
                            (((((&raw mut gRfu).cast::<u8>()).wrapping_add(3285)).cast::<u8>())
                                .wrapping_offset((i) as isize))
                            .write(11u8);
                        }
                        rfu_clearSlot(4u8, ((i) as u8));
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn GetJoinGroupStatus() -> i32 {
    unsafe {
        let mut status: i32 = 0i32;
        if (((((&raw mut gRfu).cast::<u8>()).wrapping_add(3205)).read()) as i32) == 8i32 {
            if (((((((((&raw mut gRfuSlotStatusNI).cast::<*mut u8>()).cast::<*mut u8>())
                .wrapping_offset(
                    (((((&raw mut gRfu).cast::<u8>()).wrapping_add(3134)).read_volatile()) as i32)
                        as isize,
                ))
            .read())
            .cast::<u16>())
            .read()) as i32)
                == 38i32)
                || (((((((((&raw mut gRfuSlotStatusNI).cast::<*mut u8>()).cast::<*mut u8>())
                    .wrapping_offset(
                        (((((&raw mut gRfu).cast::<u8>()).wrapping_add(3134)).read_volatile())
                            as i32) as isize,
                    ))
                .read())
                .cast::<u16>())
                .read()) as i32)
                    == 39i32)
            {
                rfu_clearSlot(
                    4u8,
                    (((&raw mut gRfu).cast::<u8>()).wrapping_add(3134)).read_volatile(),
                );
            }
        }
        if ((((((((((&raw mut gRfuSlotStatusNI).cast::<*mut u8>()).cast::<*mut u8>())
            .wrapping_offset(
                (((((&raw mut gRfu).cast::<u8>()).wrapping_add(3134)).read_volatile()) as i32)
                    as isize,
            ))
        .read())
        .wrapping_add(52))
        .cast::<u16>())
        .read()) as i32)
            == 70i32)
            || ((((((((((&raw mut gRfuSlotStatusNI).cast::<*mut u8>()).cast::<*mut u8>())
                .wrapping_offset(
                    (((((&raw mut gRfu).cast::<u8>()).wrapping_add(3134)).read_volatile()) as i32)
                        as isize,
                ))
            .read())
            .wrapping_add(52))
            .cast::<u16>())
            .read()) as i32)
                == 72i32)
        {
            rfu_clearSlot(
                8u8,
                (((&raw mut gRfu).cast::<u8>()).wrapping_add(3134)).read_volatile(),
            );
            RfuSetStatus(
                (((&raw mut gRfu).cast::<u8>()).wrapping_add(3206)).read(),
                0u16,
            );
            status = (((((&raw mut gRfu).cast::<u8>()).wrapping_add(3206)).read()) as i32);
        } else {
            if (((((((((&raw mut gRfuSlotStatusNI).cast::<*mut u8>()).cast::<*mut u8>())
                .wrapping_offset(
                    (((((&raw mut gRfu).cast::<u8>()).wrapping_add(3134)).read_volatile()) as i32)
                        as isize,
                ))
            .read())
            .wrapping_add(52))
            .cast::<u16>())
            .read()) as i32)
                == 71i32
            {
                rfu_clearSlot(
                    8u8,
                    (((&raw mut gRfu).cast::<u8>()).wrapping_add(3134)).read_volatile(),
                );
                status = 6i32;
            }
        }
        return status;
    }
}
pub(crate) unsafe extern "C" fn Task_PlayerExchange(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut i: i32 = 0i32;
        if ((((((&raw mut gRfu).cast::<u8>()).wrapping_add(241)).read()) as i32) == 1i32)
            || ((((((&raw mut gRfu).cast::<u8>()).wrapping_add(241)).read()) as i32) == 2i32)
        {
            (((&raw mut gRfu).cast::<u8>()).wrapping_add(3304)).write(0u8);
            DestroyTask(taskId);
        }
        'l1: {
            let __sw1 = (((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .read()) as i32);
            if __sw1 == 0i32 {
                if (AreAllPlayersReadyToReceive()) != 0 {
                    ResetBlockReceivedFlags();
                    LocalLinkPlayerToBlock();
                    let __p2 = ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>();
                    (__p2).write(((__p2).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                if (((((&raw mut gRfu).cast::<u8>()).wrapping_add(12)).read()) as i32) == 1i32 {
                    if (((&raw mut gReceivedRemoteLinkPlayers).cast::<u8>()).read()) != 0 {
                        RfuPrepareSendBuffer(30720u16);
                    } else {
                        RfuPrepareSendBuffer(30464u16);
                    }
                    (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .write(101i16);
                } else {
                    (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .write(2i16);
                }
                break 'l1;
            }
            if __sw1 == 101i32 {
                if (((((&raw mut gSendCmd).cast::<u16>()).cast::<u16>()).read()) as i32) == 0i32 {
                    (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .write(2i16);
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                if ((((&raw mut gRfu).cast::<u8>()).wrapping_add(13)).read()) != 0 {
                    let __p3 = ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>();
                    (__p3).write(((__p3).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                if (((((&raw mut gRfu).cast::<u8>()).wrapping_add(12)).read()) as i32) == 1i32 {
                    if (AreAllPlayersReadyToReceive()) != 0 {
                        (((&raw mut gRfu).cast::<u8>()).wrapping_add(90)).write(0u8);
                        RfuPrepareSendBuffer(41216u16);
                        let __p4 = ((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>();
                        (__p4).write(((__p4).read()).wrapping_add(1));
                    }
                } else {
                    let __p5 = ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>();
                    (__p5).write(((__p5).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 4i32 {
                if (AreAllPlayersFinishedReceiving()) != 0 {
                    let __p6 = ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>();
                    (__p6).write(((__p6).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 5i32 {
                {
                    i = 0i32;
                    'l2: loop {
                        if !(i
                            < (((((&raw mut gRfu).cast::<u8>()).wrapping_add(13)).read()) as i32))
                        {
                            break 'l2;
                        }
                        'l3: {
                            LinkPlayerFromBlock(((i) as u32));
                            Rfu_ResetBlockReceivedFlag(((i) as u8));
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                let __p7 = ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>();
                (__p7).write(((__p7).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 6i32 {
                DestroyTask(taskId);
                ((&raw mut gReceivedRemoteLinkPlayers).cast::<u8>()).write(1u8);
                (((&raw mut gRfu).cast::<u8>()).wrapping_add(3304)).write(0u8);
                rfu_LMAN_setLinkRecovery(1u8, 600u16);
                if ((((&raw mut gRfu).cast::<u8>()).wrapping_add(3302)).read()) != 0 {
                    {
                        i = 0i32;
                        'l4: loop {
                            if !(i < 4i32) {
                                break 'l4;
                            }
                            'l5: {
                                if (crate::c::shr_i32(
                                    (((((&raw mut gRfu).cast::<u8>()).wrapping_add(3302)).read())
                                        as i32),
                                    ((i) as u32),
                                ) & 1i32)
                                    != 0
                                {
                                    (((&raw mut gRfu).cast::<u8>()).wrapping_add(3301))
                                        .write(((crate::c::shl_i32(1i32, ((i) as u32))) as u8));
                                    let __p8 = ((&raw mut gRfu).cast::<u8>()).wrapping_add(3302);
                                    (__p8).write(
                                        (((((__p8).read()) as i32)
                                            ^ crate::c::shl_i32(1i32, ((i) as u32)))
                                            as u8),
                                    );
                                }
                            }
                            i = (i).wrapping_add(1);
                        }
                    }
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn ClearSelectedLinkPlayerIds(selected: u16) {
    unsafe {
        let mut selected = selected;
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 4i32) {
                    break 'l1;
                }
                'l2: {
                    if (crate::c::shr_i32(((selected) as i32), ((i) as u32)) & 1i32) != 0 {
                        (((((&raw mut gRfu).cast::<u8>()).wrapping_add(3294)).cast::<u8>())
                            .wrapping_offset((i) as isize))
                        .write(0u8);
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn ReceiveRfuLinkPlayers(sioInfo: *mut u8) {
    unsafe {
        let mut sioInfo = sioInfo;
        let mut i: i32 = 0i32;
        (((&raw mut gRfu).cast::<u8>()).wrapping_add(13))
            .write(((sioInfo).wrapping_add(15)).read());
        {
            i = 0i32;
            'l1: loop {
                if !(i < 4i32) {
                    break 'l1;
                }
                'l2: {
                    (((((&raw mut gRfu).cast::<u8>()).wrapping_add(3294)).cast::<u8>())
                        .wrapping_offset((i) as isize))
                    .write(
                        ((((sioInfo).wrapping_add(16)).cast::<u8>()).wrapping_offset((i) as isize))
                            .read(),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        {
            i = 0i32;
            'l3: loop {
                if !(i < 5i32) {
                    break 'l3;
                }
                'l4: {
                    ((&raw mut gLinkPlayers).cast::<u8>())
                        .wrapping_offset((i) as isize * 28)
                        .cast::<crate::c::Rec4<28>>()
                        .write_unaligned(
                            (((sioInfo).wrapping_add(20)).cast::<u8>())
                                .wrapping_offset((i) as isize * 28)
                                .cast::<crate::c::Rec4<28>>()
                                .read_unaligned(),
                        );
                    ConvertLinkPlayerName(
                        ((&raw mut gLinkPlayers).cast::<u8>()).wrapping_offset((i) as isize * 28),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn ValidateAndReceivePokemonSioInfo(recvBuffer: *mut u8) {
    unsafe {
        let mut recvBuffer = recvBuffer;
        if crate::c::strcmp(
            ((&raw const sASCII_PokemonSioInfo).cast::<u8>().cast_mut()).cast::<u8>(),
            recvBuffer,
        ) == 0i32
        {
            ReceiveRfuLinkPlayers(recvBuffer);
            'l1: loop {
                'l2: {
                    {
                        let mut tmp: u16 = 0u16;
                        (&raw mut tmp).write_volatile(0u16);
                        'l3: loop {
                            'l4: {
                                CpuSet(
                                    (&raw mut tmp).cast::<u8>(),
                                    recvBuffer,
                                    (16777216u32
                                        | (crate::c::div_u32(
                                            252u32,
                                            ((crate::c::div_i32(16i32, 8i32)) as u32),
                                        ) & 2097151u32)),
                                );
                            }
                            if !((0i32) != 0) {
                                break 'l3;
                            }
                        }
                    }
                }
                if !((0i32) != 0) {
                    break 'l1;
                }
            }
            ResetBlockReceivedFlag(0u8);
        }
    }
}
pub(crate) unsafe extern "C" fn Task_PlayerExchangeUpdate(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut i: i32 = 0i32;
        let mut playerBlock: *mut u8 = core::ptr::null_mut();
        let mut sio: *mut u8 = core::ptr::null_mut();
        let mut playerId: u8 = (((((&raw mut gRfu).cast::<u8>()).wrapping_add(3294)).cast::<u8>())
            .wrapping_offset(
                ((((((&raw const sSlotToLinkPlayerTableId)
                    .cast::<u8>()
                    .cast_mut())
                .cast::<u8>())
                .wrapping_offset(
                    (((((&raw mut gRfu).cast::<u8>()).wrapping_add(3305)).read()) as i32) as isize,
                ))
                .read()) as i32) as isize,
            ))
        .read();
        if ((((((&raw mut gRfu).cast::<u8>()).wrapping_add(241)).read()) as i32) == 1i32)
            || ((((((&raw mut gRfu).cast::<u8>()).wrapping_add(241)).read()) as i32) == 2i32)
        {
            (((&raw mut gRfu).cast::<u8>()).wrapping_add(3304)).write(0u8);
            DestroyTask(taskId);
        }
        'l1: {
            let __sw1 = (((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .read()) as i32);
            let mut __fall = false;
            if __sw1 == 0i32 {
                __fall = true;
                if (((((&raw mut gSendCmd).cast::<u16>()).cast::<u16>()).read()) as i32) == 0i32 {
                    ResetBlockReceivedFlag(playerId);
                    RfuPrepareSendBuffer(30720u16);
                    let __p2 = ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>();
                    (__p2).write(((__p2).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                __fall = true;
                if (((((&raw mut gSendCmd).cast::<u16>()).cast::<u16>()).read()) as i32) == 0i32 {
                    let __p3 = ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>();
                    (__p3).write(((__p3).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                __fall = true;
                if (crate::c::shr_i32(((GetBlockReceivedStatus()) as i32), ((playerId) as u32))
                    & 1i32)
                    != 0
                {
                    ResetBlockReceivedFlag(playerId);
                    playerBlock = ((((&raw mut gBlockRecvBuffer).cast::<u8>())
                        .wrapping_offset(((playerId) as i32) as isize * 256))
                    .cast::<u16>())
                    .cast::<u8>();
                    ((&raw mut gLinkPlayers).cast::<u8>())
                        .wrapping_offset(((playerId) as i32) as isize * 28)
                        .cast::<crate::c::Rec4<28>>()
                        .write_unaligned(
                            (playerBlock)
                                .wrapping_add(16)
                                .cast::<crate::c::Rec4<28>>()
                                .read_unaligned(),
                        );
                    ConvertLinkPlayerName(
                        ((&raw mut gLinkPlayers).cast::<u8>())
                            .wrapping_offset(((playerId) as i32) as isize * 28),
                    );
                    let __p4 = ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>();
                    (__p4).write(((__p4).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                __fall = true;
                sio = (&raw mut gBlockSendBuffer).cast::<u8>();
                crate::c::memcpy(
                    (sio).cast::<u8>(),
                    ((&raw const sASCII_PokemonSioInfo).cast::<u8>().cast_mut()).cast::<u8>(),
                    15u32,
                );
                ((sio).wrapping_add(15))
                    .write((((&raw mut gRfu).cast::<u8>()).wrapping_add(13)).read());
                {
                    i = 0i32;
                    'l2: loop {
                        if !(i < 4i32) {
                            break 'l2;
                        }
                        'l3: {
                            ((((sio).wrapping_add(16)).cast::<u8>()).wrapping_offset((i) as isize))
                                .write(
                                    (((((&raw mut gRfu).cast::<u8>()).wrapping_add(3294))
                                        .cast::<u8>())
                                    .wrapping_offset((i) as isize))
                                    .read(),
                                );
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                crate::c::memcpy(
                    ((sio).wrapping_add(20)).cast::<u8>(),
                    (&raw mut gLinkPlayers).cast::<u8>(),
                    140u32,
                );
                let __p5 = ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>();
                (__p5).write(((__p5).read()).wrapping_add(1));
            }
            if __fall || __sw1 == 4i32 {
                __fall = true;
                sio = (&raw mut gBlockSendBuffer).cast::<u8>();
                ((sio).wrapping_add(15))
                    .write((((&raw mut gRfu).cast::<u8>()).wrapping_add(13)).read());
                {
                    i = 0i32;
                    'l4: loop {
                        if !(i < 4i32) {
                            break 'l4;
                        }
                        'l5: {
                            ((((sio).wrapping_add(16)).cast::<u8>()).wrapping_offset((i) as isize))
                                .write(
                                    (((((&raw mut gRfu).cast::<u8>()).wrapping_add(3294))
                                        .cast::<u8>())
                                    .wrapping_offset((i) as isize))
                                    .read(),
                                );
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                crate::c::memcpy(
                    ((sio).wrapping_add(20)).cast::<u8>(),
                    (&raw mut gLinkPlayers).cast::<u8>(),
                    140u32,
                );
                if (SendBlock(0u8, (&raw mut gBlockSendBuffer).cast::<u8>(), 160u16)) != 0 {
                    let __p6 = ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>();
                    (__p6).write(((__p6).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 5i32 {
                __fall = true;
                if ((IsLinkTaskFinished()) != 0)
                    && ((((GetBlockReceivedStatus()) as i32) & 1i32) != 0)
                {
                    'l6: loop {
                        'l7: {
                            {
                                let mut tmp: u16 = 0u16;
                                (&raw mut tmp).write_volatile(0u16);
                                'l8: loop {
                                    'l9: {
                                        CpuSet(
                                            (&raw mut tmp).cast::<u8>(),
                                            (&raw mut gBlockRecvBuffer).cast::<u8>(),
                                            (16777216u32
                                                | (crate::c::div_u32(
                                                    252u32,
                                                    ((crate::c::div_i32(16i32, 8i32)) as u32),
                                                ) & 2097151u32)),
                                        );
                                    }
                                    if !((0i32) != 0) {
                                        break 'l8;
                                    }
                                }
                            }
                        }
                        if !((0i32) != 0) {
                            break 'l6;
                        }
                    }
                    ResetBlockReceivedFlag(0u8);
                    (((&raw mut gRfu).cast::<u8>()).wrapping_add(3304)).write(0u8);
                    if ((((&raw mut gRfu).cast::<u8>()).wrapping_add(3302)).read()) != 0 {
                        {
                            i = 0i32;
                            'l10: loop {
                                if !(i < 4i32) {
                                    break 'l10;
                                }
                                'l11: {
                                    if (crate::c::shr_i32(
                                        (((((&raw mut gRfu).cast::<u8>()).wrapping_add(3302))
                                            .read())
                                            as i32),
                                        ((i) as u32),
                                    ) & 1i32)
                                        != 0
                                    {
                                        (((&raw mut gRfu).cast::<u8>()).wrapping_add(3301))
                                            .write(((crate::c::shl_i32(1i32, ((i) as u32))) as u8));
                                        let __p7 =
                                            ((&raw mut gRfu).cast::<u8>()).wrapping_add(3302);
                                        (__p7).write(
                                            (((((__p7).read()) as i32)
                                                ^ crate::c::shl_i32(1i32, ((i) as u32)))
                                                as u8),
                                        );
                                        (((&raw mut gRfu).cast::<u8>()).wrapping_add(3304))
                                            .write(1u8);
                                        break 'l10;
                                    }
                                }
                                i = (i).wrapping_add(1);
                            }
                        }
                    }
                    DestroyTask(taskId);
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_PlayerExchangeChat(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if ((((((&raw mut gRfu).cast::<u8>()).wrapping_add(241)).read()) as i32) == 1i32)
            || ((((((&raw mut gRfu).cast::<u8>()).wrapping_add(241)).read()) as i32) == 2i32)
        {
            DestroyTask(taskId);
        }
        'l1: {
            let __sw1 = (((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .read()) as i32);
            if __sw1 == 0i32 {
                if ((((&raw mut gRfu).cast::<u8>()).wrapping_add(13)).read()) != 0 {
                    LocalLinkPlayerToBlock();
                    SendBlock(0u8, (&raw mut gBlockSendBuffer).cast::<u8>(), 60u16);
                    let __p2 = ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>();
                    (__p2).write(((__p2).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                if (IsLinkTaskFinished()) != 0 {
                    let __p3 = ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>();
                    (__p3).write(((__p3).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                if (((GetBlockReceivedStatus()) as i32) & 1i32) != 0 {
                    ReceiveRfuLinkPlayers((&raw mut gBlockRecvBuffer).cast::<u8>());
                    ResetBlockReceivedFlag(0u8);
                    ((&raw mut gReceivedRemoteLinkPlayers).cast::<u8>()).write(1u8);
                    DestroyTask(taskId);
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn RfuCheckErrorStatus() {
    unsafe {
        if ((((((&raw mut gRfu).cast::<u8>()).wrapping_add(238)).read_volatile()) as i32) == 1i32)
            && ((((((&raw mut lman).cast::<u8>()).wrapping_add(2)).read_volatile()) as i32) == 0i32)
        {
            if (core::mem::transmute::<_, usize>(
                (((&raw mut gMain).cast::<u8>())
                    .wrapping_add(4)
                    .cast::<Option<unsafe extern "C" fn()>>())
                .read(),
            ) == (CB2_MysteryGiftEReader as *const () as usize))
                || (((((((&raw mut lman).cast::<u8>())
                    .wrapping_add(60)
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(4))
                .read())
                    != 0)
            {
                ((&raw mut gWirelessCommType).cast::<u8>()).write(2u8);
            }
            SetMainCallback2(Some(CB2_LinkError));
            (((&raw mut gMain).cast::<u8>())
                .wrapping_add(8)
                .cast::<Option<unsafe extern "C" fn()>>())
            .write(Some(CB2_LinkError));
            SetLinkErrorBuffer(
                (((((((((&raw mut gRfu).cast::<u8>())
                    .wrapping_add(10)
                    .cast::<u16>())
                .read()) as i32)
                    << 16)
                    | ((((((&raw mut gRfu).cast::<u8>())
                        .wrapping_add(16)
                        .cast::<u16>())
                    .read()) as i32)
                        << 8))
                    | (((((&raw mut gRfu).cast::<u8>())
                        .wrapping_add(18)
                        .cast::<u16>())
                    .read()) as i32)) as u32),
                ((((&raw mut gRfu).cast::<u8>()).wrapping_add(292)).wrapping_add(2242))
                    .read_volatile(),
                ((((&raw mut gRfu).cast::<u8>()).wrapping_add(2536)).wrapping_add(562))
                    .read_volatile(),
                ((((RfuGetStatus()) as i32) == 2i32) as u8),
            );
            crate::c::volatile_write(((&raw mut gRfu).cast::<u8>()).wrapping_add(238), 2u8);
            CloseLink();
        } else {
            if (((((((&raw mut gRfu).cast::<u8>()).wrapping_add(2536)).wrapping_add(563))
                .read_volatile()) as i32)
                == 1i32)
                || (((((((&raw mut gRfu).cast::<u8>()).wrapping_add(292)).wrapping_add(2243))
                    .read_volatile()) as i32)
                    == 1i32)
            {
                if ((((&raw mut lman).cast::<u8>()).wrapping_add(2)).read_volatile()) != 0 {
                    rfu_LMAN_requestChangeAgbClockMaster();
                }
                RfuSetStatus(1u8, 28672u16);
                RfuSetErrorParams(28672u32);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn RfuMain1_UnionRoom() {
    unsafe {
        if (((((&raw mut lman).cast::<u8>()).wrapping_add(6)).read()) as i32) == 1i32 {
            rfu_REQ_recvData();
            rfu_waitREQComplete();
            rfu_LMAN_REQ_sendData(0u8);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn RfuMain1() -> u32 {
    unsafe {
        let mut retval: u32 = 0u32;
        (((&raw mut gRfu).cast::<u8>()).wrapping_add(3277)).write(0u8);
        rfu_LMAN_manager_entity(((Random2()) as u32));
        if !(((((&raw mut gRfu).cast::<u8>()).wrapping_add(239)).read()) != 0) {
            'l1: {
                let __sw1 = (((((&raw mut gRfu).cast::<u8>()).wrapping_add(12)).read()) as i32);
                if __sw1 == 1i32 {
                    RfuMain1_Parent();
                    break 'l1;
                }
                if __sw1 == 0i32 {
                    retval = RfuMain1_Child();
                    break 'l1;
                }
                if __sw1 == 2i32 {
                    RfuMain1_UnionRoom();
                    break 'l1;
                }
            }
        }
        return retval;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn RfuMain2() -> u32 {
    unsafe {
        let mut retval: u32 = 0u32;
        if !(((((&raw mut gRfu).cast::<u8>()).wrapping_add(239)).read()) != 0) {
            if (((((&raw mut gRfu).cast::<u8>()).wrapping_add(12)).read()) as i32) == 1i32 {
                retval = RfuMain2_Parent();
            }
            RfuCheckErrorStatus();
        }
        return retval;
    }
}
pub(crate) unsafe extern "C" fn SetHostRfuUsername() {
    unsafe {
        StringCopy(
            ((&raw mut gHostRfuUsername).cast::<u8>()).cast::<u8>(),
            (((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).cast::<u8>(),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ResetHostRfuGameData() {
    unsafe {
        crate::c::memset((&raw mut gHostRfuGameData).cast::<u8>(), 0i32, 13u32);
        InitHostRfuGameData((&raw mut gHostRfuGameData).cast::<u8>(), 0u8, 0u32, 0i32);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetHostRfuGameData(activity: u8, partnerInfo: u32, startedActivity: u32) {
    unsafe {
        let mut activity = activity;
        let mut partnerInfo = partnerInfo;
        let mut startedActivity = startedActivity;
        InitHostRfuGameData(
            (&raw mut gHostRfuGameData).cast::<u8>(),
            activity,
            startedActivity,
            ((partnerInfo) as i32),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetHostRfuWonderFlags(hasNews: u32, hasCard: u32) {
    unsafe {
        let mut hasNews = hasNews;
        let mut hasCard = hasCard;
        crate::c::bf_write(
            ((&raw mut gHostRfuGameData).cast::<u8>()).wrapping_add(0),
            4,
            1,
            ((hasNews) as u16) as i32,
        );
        crate::c::bf_write(
            ((&raw mut gHostRfuGameData).cast::<u8>()).wrapping_add(0),
            5,
            1,
            ((hasCard) as u16) as i32,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetTradeBoardRegisteredMonInfo(r#type: u32, species: u32, level: u32) {
    unsafe {
        let mut r#type = r#type;
        let mut species = species;
        let mut level = level;
        crate::c::bf_write(
            ((&raw mut gHostRfuGameData).cast::<u8>()).wrapping_add(9),
            2,
            6,
            ((r#type) as u16) as i32,
        );
        crate::c::bf_write(
            ((&raw mut gHostRfuGameData).cast::<u8>()).wrapping_add(8),
            0,
            10,
            ((species) as u16) as i32,
        );
        crate::c::bf_write(
            ((&raw mut gHostRfuGameData).cast::<u8>()).wrapping_add(11),
            1,
            7,
            ((level) as u8) as i32,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetLinkPlayerInfoFlags(playerId: i32) -> u8 {
    unsafe {
        let mut playerId = playerId;
        let mut retval: u8 = 128u8;
        retval = ((((retval) as i32)
            | (((((((&raw mut gLinkPlayers).cast::<u8>())
                .wrapping_offset((playerId) as isize * 28))
            .wrapping_add(19))
            .read()) as i32)
                << 3)) as u8);
        retval = ((((retval) as u32)
            | (((((&raw mut gLinkPlayers).cast::<u8>())
                .wrapping_offset((playerId) as isize * 28))
            .wrapping_add(4)
            .cast::<u32>())
            .read()
                & 7u32)) as u8);
        return retval;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetOtherPlayersInfoFlags() {
    unsafe {
        let mut data: *mut u8 = (&raw mut gHostRfuGameData).cast::<u8>();
        let mut i: i32 = 0i32;
        {
            i = 1i32;
            'l1: loop {
                if !(i < ((GetLinkPlayerCount()) as i32)) {
                    break 'l1;
                }
                'l2: {
                    ((((data).wrapping_add(4)).cast::<u8>())
                        .wrapping_offset(((i).wrapping_sub(1i32)) as isize))
                    .write(GetLinkPlayerInfoFlags(i));
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn UpdateGameData_GroupLockedIn(startedActivity: u8) {
    unsafe {
        let mut startedActivity = startedActivity;
        crate::c::bf_write(
            ((&raw mut gHostRfuGameData).cast::<u8>()).wrapping_add(10),
            7,
            1,
            (startedActivity) as i32,
        );
        rfu_REQ_configGameData(
            0u8,
            2u16,
            (&raw mut gHostRfuGameData).cast::<u8>(),
            ((&raw mut gHostRfuUsername).cast::<u8>()).cast::<u8>(),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn UpdateGameData_SetActivity(
    activity: u8,
    partnerInfo: u32,
    startedActivity: u32,
) {
    unsafe {
        let mut activity = activity;
        let mut partnerInfo = partnerInfo;
        let mut startedActivity = startedActivity;
        if ((activity) as i32) != 0i32 {
            SetHostRfuGameData(activity, partnerInfo, startedActivity);
        }
        rfu_REQ_configGameData(
            0u8,
            2u16,
            (&raw mut gHostRfuGameData).cast::<u8>(),
            ((&raw mut gHostRfuUsername).cast::<u8>()).cast::<u8>(),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetUnionRoomChatPlayerData(numPlayers: u32) {
    unsafe {
        let mut numPlayers = numPlayers;
        let mut i: i32 = 0i32;
        let mut numConnectedChildren: u32 = 0u32;
        let mut partnerInfo: u32 = 0u32;
        let mut slots: i32 = 0i32;
        if ((crate::c::bf_read((GetHostRfuGameData()).wrapping_add(10), 0, 7, false) as u8) as i32)
            == 69i32
        {
            numConnectedChildren = 0u32;
            partnerInfo = 0u32;
            slots = ((((((&raw mut gRfu).cast::<u8>()).wrapping_add(3298)).read()) as i32)
                ^ (((((&raw mut gRfu).cast::<u8>()).wrapping_add(3299)).read()) as i32));
            {
                i = 0i32;
                'l1: loop {
                    if !(i < 4i32) {
                        break 'l1;
                    }
                    'l2: {
                        if (crate::c::shr_i32(slots, ((i) as u32)) & 1i32) != 0 {
                            partnerInfo = (partnerInfo
                                | crate::c::shl_u32(
                                    (((128i32
                                        | ((((((((&raw mut gLinkPlayers).cast::<u8>())
                                            .wrapping_offset(
                                                (((((((&raw mut gRfu).cast::<u8>())
                                                    .wrapping_add(3294))
                                                .cast::<u8>())
                                                .wrapping_offset((i) as isize))
                                                .read())
                                                    as i32)
                                                    as isize
                                                    * 28,
                                            ))
                                        .wrapping_add(19))
                                        .read())
                                            as i32)
                                            & 1i32)
                                            << 3)) as u32)
                                        | (((((&raw mut gLinkPlayers).cast::<u8>())
                                            .wrapping_offset(
                                                (((((((&raw mut gRfu).cast::<u8>())
                                                    .wrapping_add(3294))
                                                .cast::<u8>())
                                                .wrapping_offset((i) as isize))
                                                .read())
                                                    as i32)
                                                    as isize
                                                    * 28,
                                            ))
                                        .wrapping_add(4)
                                        .cast::<u32>())
                                        .read()
                                            & 7u32)),
                                    (numConnectedChildren).wrapping_mul(8u32),
                                ));
                            numConnectedChildren = (numConnectedChildren).wrapping_add(1);
                            if numConnectedChildren == (numPlayers).wrapping_sub(1u32) {
                                break 'l1;
                            }
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
            UpdateGameData_SetActivity(69u8, partnerInfo, 0u32);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn RfuSetErrorParams(errorInfo: u32) {
    unsafe {
        let mut errorInfo = errorInfo;
        if (((((&raw mut gRfu).cast::<u8>()).wrapping_add(238)).read_volatile()) as i32) == 0i32 {
            (((&raw mut gRfu).cast::<u8>())
                .wrapping_add(16)
                .cast::<u16>())
            .write(((((&raw mut lman).cast::<u8>()).wrapping_add(20)).cast::<u16>()).read());
            (((&raw mut gRfu).cast::<u8>())
                .wrapping_add(18)
                .cast::<u16>())
            .write(
                (((((&raw mut lman).cast::<u8>()).wrapping_add(20)).cast::<u16>())
                    .wrapping_offset(1))
                .read(),
            );
            (((&raw mut gRfu).cast::<u8>())
                .wrapping_add(10)
                .cast::<u16>())
            .write(((errorInfo) as u16));
            crate::c::volatile_write(((&raw mut gRfu).cast::<u8>()).wrapping_add(238), 1u8);
        }
    }
}
pub(crate) unsafe extern "C" fn ResetErrorState() {
    unsafe {
        crate::c::volatile_write(((&raw mut gRfu).cast::<u8>()).wrapping_add(238), 0u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn RfuSetIgnoreError(enable: u32) {
    unsafe {
        let mut enable = enable;
        if !((enable) != 0) {
            crate::c::volatile_write(((&raw mut gRfu).cast::<u8>()).wrapping_add(238), 0u8);
        } else {
            crate::c::volatile_write(((&raw mut gRfu).cast::<u8>()).wrapping_add(238), 4u8);
        }
    }
}
pub(crate) unsafe extern "C" fn DisconnectNewChild() {
    unsafe {
        SendDisconnectCommand(((((&raw mut lman).cast::<u8>()).read()) as u32), 1u32);
        (((&raw mut gRfu).cast::<u8>()).cast::<Option<unsafe extern "C" fn()>>()).write(None);
    }
}
pub(crate) unsafe extern "C" fn StartDisconnectNewChild() {
    unsafe {
        (((&raw mut gRfu).cast::<u8>()).cast::<Option<unsafe extern "C" fn()>>())
            .write(Some(DisconnectNewChild));
    }
}
pub(crate) unsafe extern "C" fn LinkManagerCB_Parent(msg: u8, paramCount: u8) {
    unsafe {
        let mut msg = msg;
        let mut paramCount = paramCount;
        let mut i: u8 = 0u8;
        let mut disconnectFlag: u8 = 0u8;
        'l1: {
            let __sw1 = ((msg) as i32);
            if __sw1 == 0i32 {
                (((&raw mut gRfu).cast::<u8>()).wrapping_add(4).cast::<u16>()).write(2u16);
                break 'l1;
            }
            if __sw1 == 16i32 {
                break 'l1;
            }
            if __sw1 == 17i32 {
                ParentResetChildRecvMetadata(
                    ((((((&raw mut lman).cast::<u8>()).wrapping_add(20)).cast::<u16>()).read())
                        as i32),
                );
                {
                    i = 0u8;
                    'l2: loop {
                        if !(((i) as i32) < 4i32) {
                            break 'l2;
                        }
                        'l3: {
                            if (crate::c::shr_i32(
                                ((((((&raw mut lman).cast::<u8>()).wrapping_add(20)).cast::<u16>())
                                    .read()) as i32),
                                ((i) as u32),
                            ) & 1i32)
                                != 0
                            {
                                let mut data: *mut u8 =
                                    (((((((&raw mut gRfuLinkStatus).cast::<*mut u8>()).read())
                                        .wrapping_add(20))
                                    .cast::<u8>())
                                    .wrapping_offset(((i) as i32) as isize * 32))
                                    .wrapping_add(6))
                                    .cast::<u8>();
                                if ((crate::c::bf_read((data).wrapping_add(10), 0, 7, false) as u8)
                                    as i32)
                                    == ((crate::c::bf_read(
                                        (GetHostRfuGameData()).wrapping_add(10),
                                        0,
                                        7,
                                        false,
                                    ) as u8) as i32)
                                {
                                    (((((&raw mut gRfu).cast::<u8>()).wrapping_add(3281))
                                        .cast::<u8>())
                                    .wrapping_offset(((i) as i32) as isize))
                                    .write(0u8);
                                    (((((&raw mut gRfu).cast::<u8>()).wrapping_add(3285))
                                        .cast::<u8>())
                                    .wrapping_offset(((i) as i32) as isize))
                                    .write(0u8);
                                    rfu_setRecvBuffer(
                                        32u8,
                                        i,
                                        ((((&raw mut gRfu).cast::<u8>()).wrapping_add(3285))
                                            .cast::<u8>())
                                        .wrapping_offset(((i) as i32) as isize),
                                        1u32,
                                    );
                                } else {
                                    disconnectFlag = ((((disconnectFlag) as i32)
                                        | crate::c::shl_i32(1i32, ((i) as u32)))
                                        as u8);
                                }
                            }
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                if (disconnectFlag) != 0 {
                    rfu_REQ_disconnect(disconnectFlag);
                    rfu_waitREQComplete();
                }
                break 'l1;
            }
            if __sw1 == 18i32 {
                break 'l1;
            }
            if __sw1 == 19i32 {
                break 'l1;
            }
            if __sw1 == 20i32 {
                if (((((&raw mut gRfu).cast::<u8>()).wrapping_add(3303)).read()) as i32)
                    != ((((&raw mut lman).cast::<u8>()).read()) as i32)
                {
                    rfu_REQ_disconnect(
                        (((((((&raw mut gRfu).cast::<u8>()).wrapping_add(3303)).read()) as i32)
                            ^ ((((&raw mut lman).cast::<u8>()).read()) as i32))
                            as u8),
                    );
                    rfu_waitREQComplete();
                }
                (((&raw mut gRfu).cast::<u8>()).wrapping_add(4).cast::<u16>()).write(17u16);
                break 'l1;
            }
            if __sw1 == 49i32 {
                (((&raw mut gRfu).cast::<u8>()).wrapping_add(240)).write(1u8);
                break 'l1;
            }
            if __sw1 == 50i32 {
                (((&raw mut gRfu).cast::<u8>()).wrapping_add(240)).write(3u8);
                break 'l1;
            }
            if __sw1 == 48i32 || __sw1 == 51i32 {
                (((&raw mut gRfu).cast::<u8>()).wrapping_add(240)).write(4u8);
                let __p2 = ((&raw mut gRfu).cast::<u8>()).wrapping_add(3298);
                (__p2).write(
                    (((((__p2).read()) as i32)
                        & !((((((&raw mut lman).cast::<u8>()).wrapping_add(20)).cast::<u16>())
                            .read()) as i32)) as u8),
                );
                if ((((&raw mut gReceivedRemoteLinkPlayers).cast::<u8>()).read()) as i32) == 1i32 {
                    if (((((&raw mut gRfu).cast::<u8>()).wrapping_add(3298)).read()) as i32) == 0i32
                    {
                        RfuSetErrorParams(((msg) as u32));
                    } else {
                        StartDisconnectNewChild();
                    }
                }
                RfuSetStatus(2u8, ((msg) as u16));
                break 'l1;
            }
            if __sw1 == 52i32 || __sw1 == 66i32 || __sw1 == 67i32 || __sw1 == 68i32 {
                break 'l1;
            }
            if __sw1 == 243i32 {
                RfuSetStatus(1u8, ((msg) as u16));
                RfuSetErrorParams(((msg) as u32));
                (((&raw mut gRfu).cast::<u8>()).wrapping_add(239)).write(1u8);
                break 'l1;
            }
            if __sw1 == 240i32 || __sw1 == 241i32 || __sw1 == 242i32 || __sw1 == 255i32 {
                RfuSetErrorParams(((msg) as u32));
                RfuSetStatus(1u8, ((msg) as u16));
                crate::c::volatile_write(((&raw mut gRfu).cast::<u8>()).wrapping_add(3291), 1u8);
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn LinkManagerCB_Child(msg: u8, unused1: u8) {
    unsafe {
        let mut msg = msg;
        let mut unused1 = unused1;
        'l1: {
            let __sw1 = ((msg) as i32);
            let mut __fall = false;
            if __sw1 == 0i32 {
                __fall = true;
                (((&raw mut gRfu).cast::<u8>()).wrapping_add(4).cast::<u16>()).write(6u16);
                break 'l1;
            }
            if __sw1 == 32i32 {
                __fall = true;
                (((&raw mut gRfu).cast::<u8>()).wrapping_add(3277)).write(
                    ((((((&raw mut lman).cast::<u8>()).wrapping_add(20)).cast::<u16>()).read())
                        as u8),
                );
                break 'l1;
            }
            if __sw1 == 33i32 {
                __fall = true;
                break 'l1;
            }
            if __sw1 == 34i32 {
                __fall = true;
                crate::c::volatile_write(
                    ((&raw mut gRfu).cast::<u8>()).wrapping_add(3134),
                    ((((((&raw mut lman).cast::<u8>()).wrapping_add(20)).cast::<u16>()).read())
                        as u8),
                );
                break 'l1;
            }
            if __sw1 == 35i32 {
                __fall = true;
                RfuSetStatus(2u8, ((msg) as u16));
                break 'l1;
            }
            if __sw1 == 36i32 {
                __fall = true;
                (((&raw mut gRfu).cast::<u8>()).wrapping_add(4).cast::<u16>()).write(11u16);
                (((&raw mut gRfu).cast::<u8>()).wrapping_add(3205)).write(0u8);
                (((&raw mut gRfu).cast::<u8>()).wrapping_add(3206)).write(0u8);
                rfu_setRecvBuffer(
                    32u8,
                    (((&raw mut gRfu).cast::<u8>()).wrapping_add(3134)).read_volatile(),
                    ((&raw mut gRfu).cast::<u8>()).wrapping_add(3206),
                    1u32,
                );
                rfu_setRecvBuffer(
                    16u8,
                    (((&raw mut gRfu).cast::<u8>()).wrapping_add(3134)).read_volatile(),
                    (((&raw mut gRfu).cast::<u8>()).wrapping_add(3135)).cast::<u8>(),
                    70u32,
                );
                break 'l1;
            }
            if __sw1 == 37i32 {
                __fall = true;
                RfuSetStatus(2u8, ((msg) as u16));
                break 'l1;
            }
            if __sw1 == 48i32 {
                __fall = true;
                (((&raw mut gRfu).cast::<u8>()).wrapping_add(240)).write(2u8);
                if (((((&raw mut gRfu).cast::<u8>()).wrapping_add(3206)).read()) as i32) == 6i32 {
                    break 'l1;
                }
            }
            if __fall || __sw1 == 51i32 {
                __fall = true;
                if (((((&raw mut gRfu).cast::<u8>()).wrapping_add(240)).read()) as i32) != 2i32 {
                    (((&raw mut gRfu).cast::<u8>()).wrapping_add(240)).write(4u8);
                }
                if (((((&raw mut gRfu).cast::<u8>()).wrapping_add(3206)).read()) as i32) != 9i32 {
                    RfuSetStatus(2u8, ((msg) as u16));
                }
                Debug_PrintString(
                    ((&raw const sASCII_LinkLossDisconnect)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>(),
                    5u8,
                    5u8,
                );
                if ((((&raw mut gReceivedRemoteLinkPlayers).cast::<u8>()).read()) as i32) == 1i32 {
                    RfuSetErrorParams(((msg) as u32));
                }
                break 'l1;
            }
            if __sw1 == 49i32 {
                __fall = true;
                (((&raw mut gRfu).cast::<u8>()).wrapping_add(240)).write(1u8);
                Debug_PrintString(
                    ((&raw const sASCII_LinkLossRecoveryNow)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>(),
                    5u8,
                    5u8,
                );
                break 'l1;
            }
            if __sw1 == 50i32 {
                __fall = true;
                (((&raw mut gRfu).cast::<u8>()).wrapping_add(240)).write(3u8);
                crate::c::volatile_write(((&raw mut gRfu).cast::<u8>()).wrapping_add(3132), 1u8);
                break 'l1;
            }
            if __sw1 == 52i32 {
                __fall = true;
                break 'l1;
            }
            if __sw1 == 66i32 || __sw1 == 67i32 || __sw1 == 68i32 {
                __fall = true;
                break 'l1;
            }
            if __sw1 == 243i32 {
                __fall = true;
                RfuSetStatus(1u8, ((msg) as u16));
                RfuSetErrorParams(((msg) as u32));
                (((&raw mut gRfu).cast::<u8>()).wrapping_add(239)).write(1u8);
                break 'l1;
            }
            if __sw1 == 240i32 || __sw1 == 241i32 || __sw1 == 242i32 || __sw1 == 255i32 {
                __fall = true;
                RfuSetStatus(1u8, ((msg) as u16));
                RfuSetErrorParams(((msg) as u32));
                crate::c::volatile_write(((&raw mut gRfu).cast::<u8>()).wrapping_add(3291), 1u8);
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn ParentResetChildRecvMetadata(slot: i32) {
    unsafe {
        let mut slot = slot;
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 4i32) {
                    break 'l1;
                }
                'l2: {
                    if (crate::c::shr_i32(slot, ((i) as u32)) & 1i32) != 0 {
                        (((((&raw mut gRfu).cast::<u8>()).wrapping_add(3306)).cast::<u8>())
                            .wrapping_offset((i) as isize))
                        .write(0u8);
                        (((((&raw mut gRfu).cast::<u8>()).wrapping_add(3310)).cast::<u8>())
                            .wrapping_offset((i) as isize))
                        .write(255u8);
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn GetNewChildrenInUnionRoomChat(emptySlotMask: i32) -> u8 {
    unsafe {
        let mut emptySlotMask = emptySlotMask;
        let mut ret: u8 = 0u8;
        let mut i: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 4i32) {
                    break 'l1;
                }
                'l2: {
                    if (crate::c::shr_i32(emptySlotMask, ((i) as u32)) & 1i32) != 0 {
                        let mut data: *mut u8 =
                            (((((((&raw mut gRfuLinkStatus).cast::<*mut u8>()).read())
                                .wrapping_add(20))
                            .cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 32))
                            .wrapping_add(6))
                            .cast::<u8>();
                        if ((crate::c::bf_read((data).wrapping_add(10), 0, 7, false) as u8) as i32)
                            == 69i32
                        {
                            ret = ((((ret) as i32) | crate::c::shl_i32(1i32, ((i) as u32))) as u8);
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return ret;
    }
}
pub(crate) unsafe extern "C" fn LinkManagerCB_UnionRoom(msg: u8, paramCount: u8) {
    unsafe {
        let mut msg = msg;
        let mut paramCount = paramCount;
        let mut acceptSlot: u8 = 0u8;
        'l1: {
            let __sw1 = ((msg) as i32);
            let mut __fall = false;
            if __sw1 == 0i32 {
                __fall = true;
                (((&raw mut gRfu).cast::<u8>()).wrapping_add(4).cast::<u16>()).write(17u16);
                break 'l1;
            }
            if __sw1 == 16i32 {
                __fall = true;
                RfuSetStatus(4u8, 0u16);
                break 'l1;
            }
            if __sw1 == 17i32 {
                __fall = true;
                if (((crate::c::bf_read((GetHostRfuGameData()).wrapping_add(10), 0, 7, false) as u8)
                    as i32)
                    == 69i32)
                    && (!(((((&raw mut gRfu).cast::<u8>()).wrapping_add(3289)).read()) != 0))
                {
                    let mut newChildren: u8 = GetNewChildrenInUnionRoomChat(
                        ((((((&raw mut lman).cast::<u8>()).wrapping_add(20)).cast::<u16>()).read())
                            as i32),
                    );
                    if ((newChildren) as i32) != 0i32 {
                        acceptSlot = ((crate::c::shl_i32(
                            1i32,
                            ((Rfu_GetIndexOfNewestChild(newChildren)) as u32),
                        )) as u8);
                        if ((((((&raw mut gRfu).cast::<u8>()).wrapping_add(3302)).read()) as i32)
                            == 0i32)
                            && (!(((((&raw mut gRfu).cast::<u8>()).wrapping_add(3304)).read())
                                != 0))
                        {
                            (((&raw mut gRfu).cast::<u8>()).wrapping_add(3301)).write(acceptSlot);
                            let __p2 = ((&raw mut gRfu).cast::<u8>()).wrapping_add(3302);
                            (__p2).write(
                                (((((__p2).read()) as i32)
                                    | (((acceptSlot) as i32) ^ ((newChildren) as i32)))
                                    as u8),
                            );
                            (((&raw mut gRfu).cast::<u8>()).wrapping_add(3304)).write(1u8);
                        } else {
                            let __p3 = ((&raw mut gRfu).cast::<u8>()).wrapping_add(3302);
                            (__p3)
                                .write((((((__p3).read()) as i32) | ((newChildren) as i32)) as u8));
                        }
                    }
                    if ((newChildren) as i32)
                        != ((((((&raw mut lman).cast::<u8>()).wrapping_add(20)).cast::<u16>())
                            .read()) as i32)
                    {
                        let __p4 = ((&raw mut gRfu).cast::<u8>()).wrapping_add(3299);
                        (__p4).write(
                            (((((__p4).read()) as i32)
                                | (((newChildren) as i32)
                                    ^ ((((((&raw mut lman).cast::<u8>()).wrapping_add(20))
                                        .cast::<u16>())
                                    .read()) as i32))) as u8),
                        );
                        (((&raw mut gRfu).cast::<u8>()).wrapping_add(3300)).write(2u8);
                    }
                } else {
                    if ((crate::c::bf_read((GetHostRfuGameData()).wrapping_add(10), 0, 7, false)
                        as u8) as i32)
                        == 84i32
                    {
                        rfu_REQ_disconnect(((&raw mut lman).cast::<u8>()).read());
                        rfu_waitREQComplete();
                    }
                }
                ParentResetChildRecvMetadata(
                    ((((((&raw mut lman).cast::<u8>()).wrapping_add(20)).cast::<u16>()).read())
                        as i32),
                );
                break 'l1;
            }
            if __sw1 == 18i32 {
                __fall = true;
                break 'l1;
            }
            if __sw1 == 19i32 {
                __fall = true;
                break 'l1;
            }
            if __sw1 == 20i32 {
                __fall = true;
                if (((crate::c::bf_read((GetHostRfuGameData()).wrapping_add(10), 0, 7, false) as u8)
                    as i32)
                    != 69i32)
                    && ((((((&raw mut lman).cast::<u8>()).wrapping_add(1)).read()) as i32) > 1i32)
                {
                    acceptSlot = ((crate::c::shl_i32(
                        1i32,
                        ((Rfu_GetIndexOfNewestChild(
                            ((((((&raw mut lman).cast::<u8>()).wrapping_add(20)).cast::<u16>())
                                .read()) as u8),
                        )) as u32),
                    )) as u8);
                    rfu_REQ_disconnect(
                        ((((((&raw mut lman).cast::<u8>()).read()) as i32) ^ ((acceptSlot) as i32))
                            as u8),
                    );
                    rfu_waitREQComplete();
                }
                if (((((&raw mut gRfu).cast::<u8>()).wrapping_add(4).cast::<u16>()).read()) as i32)
                    == 15i32
                {
                    (((&raw mut gRfu).cast::<u8>()).wrapping_add(4).cast::<u16>()).write(16u16);
                }
                break 'l1;
                break 'l1;
            }
            if __sw1 == 32i32 {
                __fall = true;
                (((&raw mut gRfu).cast::<u8>()).wrapping_add(3277)).write(
                    ((((((&raw mut lman).cast::<u8>()).wrapping_add(20)).cast::<u16>()).read())
                        as u8),
                );
                break 'l1;
            }
            if __sw1 == 33i32 {
                __fall = true;
                break 'l1;
            }
            if __sw1 == 34i32 {
                __fall = true;
                crate::c::volatile_write(
                    ((&raw mut gRfu).cast::<u8>()).wrapping_add(3134),
                    ((((((&raw mut lman).cast::<u8>()).wrapping_add(20)).cast::<u16>()).read())
                        as u8),
                );
                break 'l1;
            }
            if __sw1 == 35i32 {
                __fall = true;
                (((&raw mut gRfu).cast::<u8>()).wrapping_add(4).cast::<u16>()).write(18u16);
                if (((((&raw mut gRfu).cast::<u8>()).wrapping_add(3279)).read()) as i32) < 2i32 {
                    let __p5 = ((&raw mut gRfu).cast::<u8>()).wrapping_add(3279);
                    (__p5).write(((__p5).read()).wrapping_add(1));
                    CreateTask(Some(Task_TryConnectToUnionRoomParent), 2u8);
                } else {
                    RfuSetStatus(2u8, ((msg) as u16));
                }
                break 'l1;
            }
            if __sw1 == 36i32 {
                __fall = true;
                (((&raw mut gRfu).cast::<u8>()).wrapping_add(4).cast::<u16>()).write(13u16);
                RfuSetStatus(3u8, 0u16);
                rfu_setRecvBuffer(
                    16u8,
                    (((&raw mut gRfu).cast::<u8>()).wrapping_add(3134)).read_volatile(),
                    (((&raw mut gRfu).cast::<u8>()).wrapping_add(3135)).cast::<u8>(),
                    70u32,
                );
                break 'l1;
            }
            if __sw1 == 37i32 {
                __fall = true;
                RfuSetStatus(2u8, ((msg) as u16));
                break 'l1;
            }
            if __sw1 == 49i32 {
                __fall = true;
                if (((((&raw mut lman).cast::<u8>()).read()) as i32)
                    & ((((((&raw mut lman).cast::<u8>()).wrapping_add(20)).cast::<u16>()).read())
                        as i32))
                    != 0
                {
                    (((&raw mut gRfu).cast::<u8>()).wrapping_add(240)).write(1u8);
                }
                break 'l1;
            }
            if __sw1 == 50i32 {
                __fall = true;
                (((&raw mut gRfu).cast::<u8>()).wrapping_add(240)).write(3u8);
                if (((((&raw mut gRfuLinkStatus).cast::<*mut u8>()).read()).read()) as i32) == 0i32
                {
                    crate::c::volatile_write(
                        ((&raw mut gRfu).cast::<u8>()).wrapping_add(3132),
                        1u8,
                    );
                }
                break 'l1;
            }
            if __sw1 == 48i32 {
                __fall = true;
                (((&raw mut gRfu).cast::<u8>()).wrapping_add(240)).write(2u8);
            }
            if __fall || __sw1 == 51i32 {
                __fall = true;
                if (((((&raw mut gRfu).cast::<u8>()).wrapping_add(240)).read()) as i32) != 2i32 {
                    (((&raw mut gRfu).cast::<u8>()).wrapping_add(240)).write(4u8);
                }
                if (((((&raw mut gRfu).cast::<u8>()).wrapping_add(12)).read()) as i32) == 1i32 {
                    if ((((&raw mut gReceivedRemoteLinkPlayers).cast::<u8>()).read()) as i32)
                        == 1i32
                    {
                        let __p6 = ((&raw mut gRfu).cast::<u8>()).wrapping_add(3298);
                        (__p6).write(
                            (((((__p6).read()) as i32)
                                & !((((((&raw mut lman).cast::<u8>()).wrapping_add(20))
                                    .cast::<u16>())
                                .read()) as i32)) as u8),
                        );
                        if (((((&raw mut gRfu).cast::<u8>()).wrapping_add(3298)).read()) as i32)
                            == 0i32
                        {
                            RfuSetErrorParams(((msg) as u32));
                        } else {
                            StartDisconnectNewChild();
                        }
                    }
                } else {
                    if ((((((&raw mut gRfu).cast::<u8>()).wrapping_add(3300)).read()) as i32)
                        != 2i32)
                        && (((((&raw mut gReceivedRemoteLinkPlayers).cast::<u8>()).read()) as i32)
                            == 1i32)
                    {
                        RfuSetErrorParams(((msg) as u32));
                        rfu_LMAN_stopManager(0u8);
                    }
                }
                if (((((((&raw mut gRfuLinkStatus).cast::<*mut u8>()).read()).read()) as i32)
                    == 255i32)
                    && (!(((((&raw mut lman).cast::<u8>()).wrapping_add(7)).read()) != 0)))
                    && (((FuncIsActiveTask(Some(Task_UnionRoomListen))) as i32) == 1i32)
                {
                    (((&raw mut gRfu).cast::<u8>()).wrapping_add(4).cast::<u16>()).write(17u16);
                }
                RfuSetStatus(2u8, ((msg) as u16));
                break 'l1;
            }
            if __sw1 == 64i32 {
                __fall = true;
                (((&raw mut gRfu).cast::<u8>()).wrapping_add(3299)).write(0u8);
                break 'l1;
            }
            if __sw1 == 66i32 || __sw1 == 67i32 || __sw1 == 68i32 {
                __fall = true;
                break 'l1;
            }
            if __sw1 == 243i32 {
                __fall = true;
                RfuSetStatus(1u8, ((msg) as u16));
                RfuSetErrorParams(((msg) as u32));
                (((&raw mut gRfu).cast::<u8>()).wrapping_add(239)).write(1u8);
                break 'l1;
            }
            if __sw1 == 240i32 || __sw1 == 241i32 || __sw1 == 242i32 || __sw1 == 255i32 {
                __fall = true;
                RfuSetErrorParams(((msg) as u32));
                RfuSetStatus(1u8, ((msg) as u16));
                crate::c::volatile_write(((&raw mut gRfu).cast::<u8>()).wrapping_add(3291), 0u8);
                break 'l1;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn RfuSetNormalDisconnectMode() {
    unsafe {
        (((&raw mut gRfu).cast::<u8>()).wrapping_add(3300)).write(2u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn RfuSetStatus(status: u8, errorInfo: u16) {
    unsafe {
        let mut status = status;
        let mut errorInfo = errorInfo;
        (((&raw mut gRfu).cast::<u8>()).wrapping_add(241)).write(status);
        (((&raw mut gRfu).cast::<u8>())
            .wrapping_add(10)
            .cast::<u16>())
        .write(errorInfo);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn RfuGetStatus() -> u8 {
    unsafe {
        return (((&raw mut gRfu).cast::<u8>()).wrapping_add(241)).read();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn RfuHasErrored() -> u32 {
    unsafe {
        let mut status: u32 = ((RfuGetStatus()) as u32);
        if (status == 1u32) || (status == 2u32) {
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
pub unsafe extern "C" fn Rfu_IsPlayerExchangeActive() -> u32 {
    unsafe {
        return (((((&raw mut gRfu).cast::<u8>()).wrapping_add(3304)).read()) as u32);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Rfu_IsMaster() -> u8 {
    unsafe {
        return (((&raw mut gRfu).cast::<u8>()).wrapping_add(12)).read();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn RfuVSync() {
    unsafe {
        rfu_LMAN_syncVBlank();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ClearRecvCommands() {
    unsafe {
        'l1: loop {
            'l2: {
                {
                    let mut tmp: u32 = 0u32;
                    (&raw mut tmp).write_volatile(0u32);
                    'l3: loop {
                        'l4: {
                            CpuSet(
                                (&raw mut tmp).cast::<u8>(),
                                (&raw mut gRecvCmds).cast::<u8>(),
                                (83886080u32
                                    | (crate::c::div_u32(
                                        80u32,
                                        ((crate::c::div_i32(32i32, 8i32)) as u32),
                                    ) & 2097151u32)),
                            );
                        }
                        if !((0i32) != 0) {
                            break 'l3;
                        }
                    }
                }
            }
            if !((0i32) != 0) {
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn VBlank_RfuIdle() {
    unsafe {
        LoadOam();
        ProcessSpriteCopyRequests();
        TransferPlttBuffer();
    }
}
pub(crate) unsafe extern "C" fn Debug_RfuIdle() {
    unsafe {
        let mut i: i32 = 0i32;
        ResetSpriteData();
        FreeAllSpritePalettes();
        ResetTasks();
        ResetPaletteFade();
        SetVBlankCallback(Some(VBlank_RfuIdle));
        if (IsWirelessAdapterConnected()) != 0 {
            ((&raw mut gLinkType).cast::<u16>()).write(4369u16);
            SetWirelessCommType1();
            OpenLink();
            SeedRng(
                (((((&raw mut gMain).cast::<u8>())
                    .wrapping_add(36)
                    .cast::<u32>())
                .read()) as u16),
            );
            {
                i = 0i32;
                'l1: loop {
                    if !(i < 4i32) {
                        break 'l1;
                    }
                    'l2: {
                        ((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(10))
                        .cast::<u8>())
                        .wrapping_offset((i) as isize))
                        .write(((crate::c::rem_i32(((Random()) as i32), 256i32)) as u8));
                    }
                    i = (i).wrapping_add(1);
                }
            }
            SetGpuReg(0u8, 5440u16);
            RunTasks();
            AnimateSprites();
            BuildOamBuffer();
            UpdatePaletteFade();
            CreateTask_RfuIdle();
            SetMainCallback2(Some(CB2_RfuIdle));
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsUnionRoomListenTaskActive() -> u32 {
    unsafe {
        return ((FuncIsActiveTask(Some(Task_UnionRoomListen))) as u32);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CreateTask_RfuIdle() {
    unsafe {
        if !((FuncIsActiveTask(Some(Task_Idle))) != 0) {
            (((&raw mut gRfu).cast::<u8>()).wrapping_add(102))
                .write(CreateTask(Some(Task_Idle), 0u8));
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DestroyTask_RfuIdle() {
    unsafe {
        if ((FuncIsActiveTask(Some(Task_Idle))) as i32) == 1i32 {
            DestroyTask((((&raw mut gRfu).cast::<u8>()).wrapping_add(102)).read());
        }
    }
}
pub(crate) unsafe extern "C" fn CB2_RfuIdle() {
    unsafe {
        RunTasks();
        AnimateSprites();
        BuildOamBuffer();
        UpdatePaletteFade();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn InitializeRfuLinkManager_LinkLeader(groupMax: u32) {
    unsafe {
        let mut groupMax = groupMax;
        (((&raw mut gRfu).cast::<u8>()).wrapping_add(12)).write(1u8);
        SetHostRfuUsername();
        rfu_LMAN_initializeManager(Some(LinkManagerCB_Parent), None);
        (&raw mut sRfuReqConfig)
            .cast::<u8>()
            .cast::<crate::c::Rec4<24>>()
            .write_unaligned(
                (&raw const sRfuReqConfigTemplate)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<crate::c::Rec4<24>>()
                    .read_unaligned(),
            );
        (((&raw mut sRfuReqConfig).cast::<u8>())
            .wrapping_add(2)
            .cast::<u16>())
        .write(
            ((((((&raw const sAvailSlots).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset((((groupMax).wrapping_sub(1u32)) as i32) as isize))
            .read()) as u16),
        );
        CreateTask_ParentSearchForChildren();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn InitializeRfuLinkManager_JoinGroup() {
    unsafe {
        (((&raw mut gRfu).cast::<u8>()).wrapping_add(12)).write(0u8);
        SetHostRfuUsername();
        rfu_LMAN_initializeManager(Some(LinkManagerCB_Child), Some(MSCCallback_Child));
        CreateTask_ChildSearchForParent();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn InitializeRfuLinkManager_EnterUnionRoom() {
    unsafe {
        (((&raw mut gRfu).cast::<u8>()).wrapping_add(12)).write(2u8);
        SetHostRfuUsername();
        rfu_LMAN_initializeManager(Some(LinkManagerCB_UnionRoom), None);
        (&raw mut sRfuReqConfig)
            .cast::<u8>()
            .cast::<crate::c::Rec4<24>>()
            .write_unaligned(
                (&raw const sRfuReqConfigTemplate)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<crate::c::Rec4<24>>()
                    .read_unaligned(),
            );
        (((&raw mut sRfuReqConfig).cast::<u8>()).wrapping_add(17)).write(0u8);
        (((&raw mut sRfuReqConfig).cast::<u8>())
            .wrapping_add(18)
            .cast::<u16>())
        .write(600u16);
        (((&raw mut gRfu).cast::<u8>()).wrapping_add(103))
            .write(CreateTask(Some(Task_UnionRoomListen), 1u8));
    }
}
pub(crate) unsafe extern "C" fn ReadU16(ptr: *mut u8) -> u16 {
    unsafe {
        let mut ptr = ptr;
        let mut ptr_: *mut u8 = ptr;
        return (((((((ptr_).wrapping_offset(1)).read()) as i32) << 8) | (((ptr_).read()) as i32))
            as u16);
    }
}
pub(crate) unsafe extern "C" fn GetPartnerIndexByNameAndTrainerID(name: *mut u8, id: u16) -> u8 {
    unsafe {
        let mut name = name;
        let mut id = id;
        let mut i: u8 = 0u8;
        let mut idx: u8 = 255u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 4i32) {
                    break 'l1;
                }
                'l2: {
                    let mut trainerId: u16 = ReadU16(
                        (((((((((&raw mut gRfuLinkStatus).cast::<*mut u8>()).read())
                            .wrapping_add(20))
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 32))
                        .wrapping_add(6))
                        .cast::<u8>())
                        .wrapping_add(2))
                        .cast::<u8>(),
                    );
                    if (((IsRfuSerialNumberValid(
                        (((((((((&raw mut gRfuLinkStatus).cast::<*mut u8>()).read())
                            .wrapping_add(20))
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 32))
                        .wrapping_add(4)
                        .cast::<u16>())
                        .read()) as u32),
                    )) != 0)
                        && (!((StringCompare(
                            name,
                            (((((((&raw mut gRfuLinkStatus).cast::<*mut u8>()).read())
                                .wrapping_add(20))
                            .cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 32))
                            .wrapping_add(21))
                            .cast::<u8>(),
                        )) != 0)))
                        && (((id) as i32) == ((trainerId) as i32))
                    {
                        idx = i;
                        if (((((((((&raw mut gRfuLinkStatus).cast::<*mut u8>()).read())
                            .wrapping_add(20))
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 32))
                        .wrapping_add(2))
                        .read()) as i32)
                            != 255i32
                        {
                            break 'l1;
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return idx;
    }
}
pub(crate) unsafe extern "C" fn RfuReqDisconnectSlot(slot: u32) {
    unsafe {
        let mut slot = slot;
        rfu_REQ_disconnect(((slot) as u8));
        rfu_waitREQComplete();
        let __p1 = ((&raw mut gRfu).cast::<u8>()).wrapping_add(3298);
        (__p1).write((((((__p1).read()) as u32) & !(slot)) as u8));
        rfu_clearSlot(
            1u8,
            (((&raw mut gRfu).cast::<u8>()).wrapping_add(3290)).read(),
        );
        rfu_UNI_setSendData(
            (((&raw mut gRfu).cast::<u8>()).wrapping_add(3298)).read(),
            (((&raw mut gRfu).cast::<u8>()).wrapping_add(3207)).cast::<u8>(),
            70u8,
        );
        (((&raw mut gRfu).cast::<u8>()).wrapping_add(3290)).write(
            ((Rfu_GetIndexOfNewestChild((((&raw mut gRfu).cast::<u8>()).wrapping_add(3298)).read()))
                as u8),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn RequestDisconnectSlotByTrainerNameAndId(name: *mut u8, id: u16) {
    unsafe {
        let mut name = name;
        let mut id = id;
        let mut index: u8 = GetPartnerIndexByNameAndTrainerID(name, id);
        if ((index) as i32) != 255i32 {
            RfuReqDisconnectSlot(((crate::c::shl_i32(1i32, ((index) as u32))) as u32));
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Rfu_DisconnectPlayerById(playerIdx: u32) {
    unsafe {
        let mut playerIdx = playerIdx;
        if playerIdx != 0u32 {
            let mut i: i32 = 0i32;
            let mut toDisconnect: u8 = 0u8;
            {
                i = 0i32;
                'l1: loop {
                    if !(i < 4i32) {
                        break 'l1;
                    }
                    'l2: {
                        if ((((((((&raw mut gRfu).cast::<u8>()).wrapping_add(3294)).cast::<u8>())
                            .wrapping_offset((i) as isize))
                        .read()) as u32)
                            == playerIdx)
                            && ((crate::c::shr_i32(
                                (((((&raw mut gRfu).cast::<u8>()).wrapping_add(3298)).read())
                                    as i32),
                                ((i) as u32),
                            ) & 1i32)
                                != 0)
                        {
                            toDisconnect = ((((toDisconnect) as i32)
                                | crate::c::shl_i32(1i32, ((i) as u32)))
                                as u8);
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
            if (toDisconnect) != 0 {
                SendDisconnectCommand(((toDisconnect) as u32), 2u32);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_SendDisconnectCommand(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if ((((((&raw mut gSendCmd).cast::<u16>()).cast::<u16>()).read()) as i32) == 0i32)
            && (!(((((&raw mut gRfu).cast::<u8>()).wrapping_add(3304)).read()) != 0))
        {
            RfuPrepareSendBuffer(60672u16);
            ((((&raw mut gSendCmd).cast::<u16>()).cast::<u16>()).wrapping_offset(1)).write(
                (((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .read()) as u16),
            );
            ((((&raw mut gSendCmd).cast::<u16>()).cast::<u16>()).wrapping_offset(2)).write(
                ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(1))
                .read()) as u16),
            );
            let __p1 = ((&raw mut gRfu).cast::<u8>()).wrapping_add(13);
            (__p1).write(
                (((((__p1).read()) as i32).wrapping_sub(
                    ((((((&raw const sPlayerBitsToCount).cast::<u8>().cast_mut()).cast::<u8>())
                        .wrapping_offset(
                            (((((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .read()) as i32) as isize,
                        ))
                    .read()) as i32),
                )) as u8),
            );
            ((((&raw mut gSendCmd).cast::<u16>()).cast::<u16>()).wrapping_offset(3))
                .write((((((&raw mut gRfu).cast::<u8>()).wrapping_add(13)).read()) as u16));
            DestroyTask(taskId);
        }
    }
}
pub(crate) unsafe extern "C" fn SendDisconnectCommand(
    playersToDisconnect: u32,
    disconnectMode: u32,
) {
    unsafe {
        let mut playersToDisconnect = playersToDisconnect;
        let mut disconnectMode = disconnectMode;
        let mut taskId: u8 = FindTaskIdByFunc(Some(Task_SendDisconnectCommand));
        if ((taskId) as i32) == 255i32 {
            taskId = CreateTask(Some(Task_SendDisconnectCommand), 5u8);
            (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
            .cast::<i16>())
            .write(((playersToDisconnect) as i16));
        } else {
            let __p1 = ((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>();
            (__p1).write((((((__p1).read()) as u32) | playersToDisconnect) as i16));
        }
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .write(((disconnectMode) as i16));
    }
}
pub(crate) unsafe extern "C" fn Task_RfuReconnectWithParent(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        if (CanTryReconnectParent()) != 0 {
            let mut id: u8 = GetPartnerIndexByNameAndTrainerID(
                (data).cast::<u8>(),
                ReadU16(((data).wrapping_offset(8)).cast::<u8>()),
            );
            if ((id) as i32) != 255i32 {
                if (((((((((&raw mut gRfuLinkStatus).cast::<*mut u8>()).read())
                    .wrapping_add(20))
                .cast::<u8>())
                .wrapping_offset(((id) as i32) as isize * 32))
                .wrapping_add(2))
                .read()) as i32)
                    != 255i32
                {
                    (((&raw mut gRfu).cast::<u8>()).wrapping_add(3133)).write(id);
                    if (TryReconnectParent()) != 0 {
                        DestroyTask(taskId);
                    }
                } else {
                    if (((crate::c::bf_read((GetHostRfuGameData()).wrapping_add(10), 0, 7, false)
                        as u8) as i32)
                        == 21i32)
                        || (((crate::c::bf_read(
                            (GetHostRfuGameData()).wrapping_add(10),
                            0,
                            7,
                            false,
                        ) as u8) as i32)
                            == 22i32)
                    {
                        let __p1 = (data).wrapping_offset(15);
                        (__p1).write(((__p1).read()).wrapping_add(1));
                    } else {
                        RfuSetStatus(2u8, 28672u16);
                        DestroyTask(taskId);
                    }
                }
            } else {
                let __p2 = (data).wrapping_offset(15);
                (__p2).write(((__p2).read()).wrapping_add(1));
                (((&raw mut gRfu).cast::<u8>()).wrapping_add(3133)).write(id);
            }
        } else {
            let __p3 = (data).wrapping_offset(15);
            (__p3).write(((__p3).read()).wrapping_add(1));
        }
        if ((((data).wrapping_offset(15)).read()) as i32) > 240i32 {
            RfuSetStatus(2u8, 28672u16);
            DestroyTask(taskId);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CreateTask_RfuReconnectWithParent(name: *mut u8, trainerId: u16) {
    unsafe {
        let mut name = name;
        let mut trainerId = trainerId;
        let mut taskId: u8 = 0u8;
        let mut data: *mut i16 = core::ptr::null_mut();
        (((&raw mut gRfu).cast::<u8>()).wrapping_add(241)).write(0u8);
        taskId = CreateTask(Some(Task_RfuReconnectWithParent), 3u8);
        data = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        StringCopy((data).cast::<u8>(), name);
        ((data).wrapping_offset(8)).write(((trainerId) as i16));
    }
}
pub(crate) unsafe extern "C" fn IsPartnerActivityIncompatible(
    activity: i16,
    partner: *mut u8,
) -> u32 {
    unsafe {
        let mut activity = activity;
        let mut partner = partner;
        if ((crate::c::bf_read((GetHostRfuGameData()).wrapping_add(10), 0, 7, false) as u8) as i32)
            == 69i32
        {
            if ((crate::c::bf_read((partner).wrapping_add(10), 0, 7, false) as u8) as i32) != 69i32
            {
                return 1u32;
            }
        } else {
            if ((crate::c::bf_read((partner).wrapping_add(10), 0, 7, false) as u8) as i32) != 64i32
            {
                return 1u32;
            } else {
                if ((activity) as i32) == 68i32 {
                    let mut original: *mut u8 = ((&raw mut gRfu).cast::<u8>()).wrapping_add(266);
                    if ((crate::c::bf_read((original).wrapping_add(8), 0, 10, false) as u16) as i32)
                        == 412i32
                    {
                        if ((crate::c::bf_read((partner).wrapping_add(8), 0, 10, false) as u16)
                            as i32)
                            == ((crate::c::bf_read((original).wrapping_add(8), 0, 10, false) as u16)
                                as i32)
                        {
                            return 0u32;
                        } else {
                            return 1u32;
                        }
                    } else {
                        if ((((crate::c::bf_read((partner).wrapping_add(8), 0, 10, false) as u16)
                            as i32)
                            != ((crate::c::bf_read((original).wrapping_add(8), 0, 10, false) as u16)
                                as i32))
                            || (((crate::c::bf_read((partner).wrapping_add(11), 1, 7, false) as u8)
                                as i32)
                                != ((crate::c::bf_read((original).wrapping_add(11), 1, 7, false)
                                    as u8) as i32)))
                            || (((crate::c::bf_read((partner).wrapping_add(9), 2, 6, false) as u16)
                                as i32)
                                != ((crate::c::bf_read((original).wrapping_add(9), 2, 6, false)
                                    as u16) as i32))
                        {
                            return 1u32;
                        }
                    }
                }
            }
        }
        return 0u32;
    }
}
pub(crate) unsafe extern "C" fn Task_TryConnectToUnionRoomParent(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if (((((&raw mut gRfu).cast::<u8>()).wrapping_add(241)).read()) as i32) == 4i32 {
            DestroyTask(taskId);
        }
        if (({
            let __p1 = ((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>();
            let __t2 = ((__p1).read()).wrapping_add(1);
            (__p1).write(__t2);
            __t2
        }) as i32)
            > 300i32
        {
            RfuSetStatus(2u8, 28672u16);
            DestroyTask(taskId);
        }
        if ((((((&raw mut gRfu).cast::<u8>()).wrapping_add(3277)).read()) as i32) != 0i32)
            && ((((((&raw mut lman).cast::<u8>()).wrapping_add(6)).read()) as i32) == 0i32)
        {
            let mut trainerId: u16 = ReadU16(
                ((((&raw mut gRfu).cast::<u8>()).wrapping_add(266)).wrapping_add(2)).cast::<u8>(),
            );
            let mut id: u8 = GetPartnerIndexByNameAndTrainerID(
                (((&raw mut gRfu).cast::<u8>()).wrapping_add(281)).cast::<u8>(),
                trainerId,
            );
            if ((id) as i32) != 255i32 {
                if !((IsPartnerActivityIncompatible(
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(1))
                    .read(),
                    (((((((&raw mut gRfuLinkStatus).cast::<*mut u8>()).read()).wrapping_add(20))
                        .cast::<u8>())
                    .wrapping_offset(((id) as i32) as isize * 32))
                    .wrapping_add(6))
                    .cast::<u8>(),
                )) != 0)
                {
                    if ((((((((((&raw mut gRfuLinkStatus).cast::<*mut u8>()).read())
                        .wrapping_add(20))
                    .cast::<u8>())
                    .wrapping_offset(((id) as i32) as isize * 32))
                    .wrapping_add(2))
                    .read()) as i32)
                        != 255i32)
                        && (!((rfu_LMAN_CHILD_connectParent(
                            (((((((&raw mut gRfuLinkStatus).cast::<*mut u8>()).read())
                                .wrapping_add(20))
                            .cast::<u8>())
                            .wrapping_offset(((id) as i32) as isize * 32))
                            .cast::<u16>())
                            .read(),
                            90u16,
                        )) != 0))
                    {
                        (((&raw mut gRfu).cast::<u8>()).wrapping_add(4).cast::<u16>()).write(10u16);
                        DestroyTask(taskId);
                    }
                } else {
                    RfuSetStatus(2u8, 28672u16);
                    DestroyTask(taskId);
                }
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn TryConnectToUnionRoomParent(name: *mut u8, parent: *mut u8, activity: u8) {
    unsafe {
        let mut name = name;
        let mut parent = parent;
        let mut activity = activity;
        let mut taskId: u8 = 0u8;
        let mut listenTaskId: u8 = 0u8;
        (((&raw mut gRfu).cast::<u8>()).wrapping_add(3279)).write(0u8);
        (((&raw mut gRfu).cast::<u8>()).wrapping_add(241)).write(0u8);
        StringCopy(
            (((&raw mut gRfu).cast::<u8>()).wrapping_add(281)).cast::<u8>(),
            name,
        );
        crate::c::memcpy(
            ((&raw mut gRfu).cast::<u8>()).wrapping_add(266),
            parent,
            13u32,
        );
        rfu_LMAN_forceChangeSP();
        taskId = CreateTask(Some(Task_TryConnectToUnionRoomParent), 2u8);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .write(((activity) as i16));
        listenTaskId = FindTaskIdByFunc(Some(Task_UnionRoomListen));
        if ((activity) as i32) == 69i32 {
            if ((listenTaskId) as i32) != 255i32 {
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((listenTaskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(7))
                .write(1i16);
            }
        } else {
            if ((listenTaskId) as i32) != 255i32 {
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((listenTaskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(7))
                .write(0i16);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsRfuRecoveringFromLinkLoss() -> u8 {
    unsafe {
        if (((((&raw mut gRfu).cast::<u8>()).wrapping_add(240)).read()) as i32) == 1i32 {
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
pub unsafe extern "C" fn IsRfuCommunicatingWithAllChildren() -> u32 {
    unsafe {
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 4i32) {
                    break 'l1;
                }
                'l2: {
                    if ((crate::c::shr_i32(
                        ((((&raw mut lman).cast::<u8>()).read()) as i32),
                        ((i) as u32),
                    ) & 1i32)
                        != 0)
                        && ((((((((&raw mut gRfu).cast::<u8>()).wrapping_add(3281)).cast::<u8>())
                            .wrapping_offset((i) as isize))
                        .read()) as i32)
                            == 0i32)
                    {
                        return 0u32;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return 1u32;
    }
}
pub(crate) unsafe extern "C" fn Debug_PrintEmpty() {
    unsafe {
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 20i32) {
                    break 'l1;
                }
                'l2: {
                    Debug_PrintString(
                        ((&raw const sASCII_30Spaces).cast::<u8>().cast_mut()).cast::<u8>(),
                        0u8,
                        ((i) as u8),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Debug_PrintStatus() {
    unsafe {
        let mut i: i32 = 0i32;
        let mut j: i32 = 0i32;
        Debug_PrintNum(((GetBlockReceivedStatus()) as u16), 28u8, 19u8, 2u8);
        Debug_PrintNum(
            ((((((&raw mut gRfuLinkStatus).cast::<*mut u8>()).read()).wrapping_add(2)).read())
                as u16),
            20u8,
            1u8,
            1u8,
        );
        Debug_PrintNum(
            ((((((&raw mut gRfuLinkStatus).cast::<*mut u8>()).read()).wrapping_add(3)).read())
                as u16),
            23u8,
            1u8,
            1u8,
        );
        if (((((&raw mut gRfu).cast::<u8>()).wrapping_add(12)).read()) as i32) == 1i32 {
            {
                i = 0i32;
                'l1: loop {
                    if !(i < 4i32) {
                        break 'l1;
                    }
                    'l2: {
                        if (crate::c::shr_i32(
                            ((((((&raw mut gRfuLinkStatus).cast::<*mut u8>()).read())
                                .wrapping_add(7))
                            .read()) as i32),
                            ((i) as u32),
                        ) & 1i32)
                            != 0
                        {
                            Debug_PrintNum(
                                (((((((&raw mut gRfuLinkStatus).cast::<*mut u8>()).read())
                                    .wrapping_add(20))
                                .cast::<u8>())
                                .wrapping_offset((i) as isize * 32))
                                .wrapping_add(4)
                                .cast::<u16>())
                                .read(),
                                1u8,
                                (((i).wrapping_add(3i32)) as u8),
                                4u8,
                            );
                            Debug_PrintString(
                                (((((((&raw mut gRfuLinkStatus).cast::<*mut u8>()).read())
                                    .wrapping_add(20))
                                .cast::<u8>())
                                .wrapping_offset((i) as isize * 32))
                                .wrapping_add(6))
                                .cast::<u8>(),
                                6u8,
                                (((i).wrapping_add(3i32)) as u8),
                            );
                            Debug_PrintString(
                                (((((((&raw mut gRfuLinkStatus).cast::<*mut u8>()).read())
                                    .wrapping_add(20))
                                .cast::<u8>())
                                .wrapping_offset((i) as isize * 32))
                                .wrapping_add(21))
                                .cast::<u8>(),
                                22u8,
                                (((i).wrapping_add(3i32)) as u8),
                            );
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
            {
                i = 0i32;
                'l3: loop {
                    if !(i < 4i32) {
                        break 'l3;
                    }
                    'l4: {
                        {
                            j = 0i32;
                            'l5: loop {
                                if !(j < 14i32) {
                                    break 'l5;
                                }
                                'l6: {
                                    Debug_PrintNum(
                                        (((((((((&raw mut gRfu).cast::<u8>()).wrapping_add(20))
                                            .cast::<u8>())
                                        .wrapping_offset((i) as isize * 14))
                                        .cast::<u8>())
                                        .wrapping_offset((j) as isize))
                                        .read()) as u16),
                                        (((j).wrapping_mul(2i32)) as u8),
                                        (((i).wrapping_add(11i32)) as u8),
                                        2u8,
                                    );
                                }
                                j = (j).wrapping_add(1);
                            }
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
            Debug_PrintString(
                ((&raw const sASCII_NowSlot).cast::<u8>().cast_mut()).cast::<u8>(),
                1u8,
                15u8,
            );
        } else {
            if (((((((&raw mut gRfuLinkStatus).cast::<*mut u8>()).read()).wrapping_add(2)).read())
                as i32)
                != 0i32)
                && (((((((&raw mut gRfuLinkStatus).cast::<*mut u8>()).read()).wrapping_add(7))
                    .read()) as i32)
                    != 0i32)
            {
                {
                    i = 0i32;
                    'l7: loop {
                        if !(i < 4i32) {
                            break 'l7;
                        }
                        'l8: {
                            Debug_PrintNum(0u16, 1u8, (((i).wrapping_add(3i32)) as u8), 4u8);
                            Debug_PrintString(
                                ((&raw const sASCII_15Spaces).cast::<u8>().cast_mut()).cast::<u8>(),
                                6u8,
                                (((i).wrapping_add(3i32)) as u8),
                            );
                            Debug_PrintString(
                                ((&raw const sASCII_8Spaces).cast::<u8>().cast_mut()).cast::<u8>(),
                                22u8,
                                (((i).wrapping_add(3i32)) as u8),
                            );
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                Debug_PrintNum(
                    (((((((&raw mut gRfuLinkStatus).cast::<*mut u8>()).read()).wrapping_add(20))
                        .cast::<u8>())
                    .wrapping_offset(
                        (((((&raw mut gRfu).cast::<u8>()).wrapping_add(3134)).read_volatile())
                            as i32) as isize
                            * 32,
                    ))
                    .wrapping_add(4)
                    .cast::<u16>())
                    .read(),
                    1u8,
                    3u8,
                    4u8,
                );
                Debug_PrintString(
                    (((((((&raw mut gRfuLinkStatus).cast::<*mut u8>()).read()).wrapping_add(20))
                        .cast::<u8>())
                    .wrapping_offset(
                        (((((&raw mut gRfu).cast::<u8>()).wrapping_add(3134)).read_volatile())
                            as i32) as isize
                            * 32,
                    ))
                    .wrapping_add(6))
                    .cast::<u8>(),
                    6u8,
                    3u8,
                );
                Debug_PrintString(
                    (((((((&raw mut gRfuLinkStatus).cast::<*mut u8>()).read()).wrapping_add(20))
                        .cast::<u8>())
                    .wrapping_offset(
                        (((((&raw mut gRfu).cast::<u8>()).wrapping_add(3134)).read_volatile())
                            as i32) as isize
                            * 32,
                    ))
                    .wrapping_add(21))
                    .cast::<u8>(),
                    22u8,
                    3u8,
                );
            } else {
                {
                    i = 0i32;
                    'l9: loop {
                        if !(i
                            < ((((((&raw mut gRfuLinkStatus).cast::<*mut u8>()).read())
                                .wrapping_add(8))
                            .read()) as i32))
                        {
                            break 'l9;
                        }
                        'l10: {
                            if (((((((((&raw mut gRfuLinkStatus).cast::<*mut u8>()).read())
                                .wrapping_add(20))
                            .cast::<u8>())
                            .wrapping_offset((i) as isize * 32))
                            .wrapping_add(2))
                            .read()) as i32)
                                != 255i32
                            {
                                Debug_PrintNum(
                                    (((((((&raw mut gRfuLinkStatus).cast::<*mut u8>()).read())
                                        .wrapping_add(20))
                                    .cast::<u8>())
                                    .wrapping_offset((i) as isize * 32))
                                    .wrapping_add(4)
                                    .cast::<u16>())
                                    .read(),
                                    1u8,
                                    (((i).wrapping_add(3i32)) as u8),
                                    4u8,
                                );
                                Debug_PrintNum(
                                    (((((((&raw mut gRfuLinkStatus).cast::<*mut u8>()).read())
                                        .wrapping_add(20))
                                    .cast::<u8>())
                                    .wrapping_offset((i) as isize * 32))
                                    .cast::<u16>())
                                    .read(),
                                    6u8,
                                    (((i).wrapping_add(3i32)) as u8),
                                    4u8,
                                );
                                Debug_PrintString(
                                    (((((((&raw mut gRfuLinkStatus).cast::<*mut u8>()).read())
                                        .wrapping_add(20))
                                    .cast::<u8>())
                                    .wrapping_offset((i) as isize * 32))
                                    .wrapping_add(21))
                                    .cast::<u8>(),
                                    22u8,
                                    (((i).wrapping_add(3i32)) as u8),
                                );
                            }
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                {
                    'l11: loop {
                        if !(i < 4i32) {
                            break 'l11;
                        }
                        'l12: {
                            Debug_PrintNum(0u16, 1u8, (((i).wrapping_add(3i32)) as u8), 4u8);
                            Debug_PrintString(
                                ((&raw const sASCII_15Spaces).cast::<u8>().cast_mut()).cast::<u8>(),
                                6u8,
                                (((i).wrapping_add(3i32)) as u8),
                            );
                            Debug_PrintString(
                                ((&raw const sASCII_8Spaces).cast::<u8>().cast_mut()).cast::<u8>(),
                                22u8,
                                (((i).wrapping_add(3i32)) as u8),
                            );
                        }
                        i = (i).wrapping_add(1);
                    }
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn GetRfuSendQueueLength() -> u32 {
    unsafe {
        return ((((((&raw mut gRfu).cast::<u8>()).wrapping_add(2536)).wrapping_add(562))
            .read_volatile()) as u32);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetRfuRecvQueueLength() -> u32 {
    unsafe {
        return ((((((&raw mut gRfu).cast::<u8>()).wrapping_add(292)).wrapping_add(2242))
            .read_volatile()) as u32);
    }
}
pub(crate) unsafe extern "C" fn Task_Idle(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
    }
}
