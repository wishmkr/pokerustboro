//! Translated from `src/ereader_screen.c` by tools/rustport/c2rs.py, then reviewed.
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

#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gUnknownSpace: crate::ffi::Align4<[u8; 64]> = crate::ffi::Align4([0; 64]);
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gEReaderData: crate::ffi::Align4<[u8; 12]> = crate::ffi::Align4([0; 12]);

unsafe extern "C" {
    static mut gDecompressionBuffer: u8;
    static mut gIntrTable: u8;
    static mut gJPText_AllowEReaderToLoadCard: u8;
    static mut gJPText_CardReadingHasBeenHalted: u8;
    static mut gJPText_Connecting: u8;
    static mut gJPText_ConnectionComplete: u8;
    static mut gJPText_ConnectionErrorCheckLink: u8;
    static mut gJPText_ConnectionErrorTryAgain: u8;
    static mut gJPText_LinkIsIncorrect: u8;
    static mut gJPText_NewTrainerHasComeToHoenn: u8;
    static mut gJPText_PleaseWaitAMoment: u8;
    static mut gJPText_ReceiveMysteryGiftWithEReader: u8;
    static mut gJPText_SelectConnectFromEReaderMenu: u8;
    static mut gJPText_SelectConnectWithGBA: u8;
    static mut gJPText_WriteErrorUnableToSaveData: u8;
    static mut gLink: u8;
    static mut gLinkType: u8;
    static mut gMain: u8;
    static mut gMultiBootProgram_EReader_End: u8;
    static mut gMultiBootProgram_EReader_Start: u8;
    static mut gReceivedRemoteLinkPlayers: u8;
    static mut gShouldAdvanceLinkState: u8;
    static mut gTasks: u8;
    fn AllocZeroed(a0: u32) -> *mut u8;
    fn CheckShouldAdvanceLinkState();
    fn CloseLink();
    fn CreateTask(a0: Option<unsafe extern "C" fn(u8)>, a1: u8) -> u8;
    fn DestroyTask(a0: u8);
    fn EReaderHandleTransfer(a0: u8, a1: u32, a2: *mut u8, a3: *mut u8) -> i32;
    fn EReaderHelper_ClearSendRecvMgr();
    fn EReaderHelper_RestoreRegsState();
    fn EReaderHelper_SaveRegsState();
    fn EReaderHelper_SerialCallback();
    fn EReaderHelper_Timer3Callback();
    fn Free(a0: *mut u8);
    fn GetBlockReceivedStatus() -> u8;
    fn GetLinkPlayerCount_2() -> u8;
    fn HasLinkErrorOccurred() -> u8;
    fn IsFanfareTaskInactive() -> u8;
    fn IsLinkConnectionEstablished() -> u8;
    fn IsLinkMaster() -> u8;
    fn IsLinkPlayerDataExchangeComplete() -> u8;
    fn MG_AddMessageTextPrinter(a0: *mut u8);
    fn MainCB_FreeAllBuffersAndReturnToInitTitleScreen();
    fn OpenLink();
    fn PlayFanfare(a0: u16);
    fn PlaySE(a0: u16);
    fn PrintMysteryGiftMenuMessage(a0: *mut u8, a1: *mut u8) -> u32;
    fn ResetBlockReceivedFlags();
    fn RestoreSerialTimer3IntrHandlers();
    fn SetCloseLinkCallbackAndType(a0: u16);
    fn SetMainCallback2(a0: Option<unsafe extern "C" fn()>);
    fn SetSuppressLinkErrorMessage(a0: u8);
    fn TryWriteTrainerHill(a0: *mut u8) -> u32;
    fn ValidateTrainerHillData(a0: *mut u8) -> u8;
}

pub(crate) unsafe extern "C" fn EReader_Load(eReader: *mut u8, size: i32, data: *mut u32) {
    unsafe {
        let mut eReader = eReader;
        let mut size = size;
        let mut data = data;
        let mut backupIME: u16 = 0u16;
        (&raw mut backupIME).write_volatile(((67109384i32) as usize as *mut u16).read_volatile());
        crate::c::volatile_write(((67109384i32) as usize as *mut u16), 0u16);
        ((((&raw mut gIntrTable).cast::<Option<unsafe extern "C" fn()>>())
            .cast::<Option<unsafe extern "C" fn()>>())
        .wrapping_offset(1))
        .write(Some(EReaderHelper_SerialCallback));
        ((((&raw mut gIntrTable).cast::<Option<unsafe extern "C" fn()>>())
            .cast::<Option<unsafe extern "C" fn()>>())
        .wrapping_offset(2))
        .write(Some(EReaderHelper_Timer3Callback));
        EReaderHelper_SaveRegsState();
        EReaderHelper_ClearSendRecvMgr();
        let __p1 = ((67109376i32) as usize as *mut u16);
        crate::c::volatile_write(__p1, (((((__p1).read_volatile()) as i32) | 4i32) as u16));
        crate::c::volatile_write(
            ((67109384i32) as usize as *mut u16),
            (&raw mut backupIME).read_volatile(),
        );
        ((eReader).cast::<u16>()).write(0u16);
        ((eReader).wrapping_add(4).cast::<u32>()).write(((size) as u32));
        ((eReader).wrapping_add(8).cast::<*mut u32>()).write(data);
    }
}
pub(crate) unsafe extern "C" fn EReader_Reset(eReader: *mut u8) {
    unsafe {
        let mut eReader = eReader;
        let mut backupIME: u16 = 0u16;
        (&raw mut backupIME).write_volatile(((67109384i32) as usize as *mut u16).read_volatile());
        crate::c::volatile_write(((67109384i32) as usize as *mut u16), 0u16);
        EReaderHelper_ClearSendRecvMgr();
        EReaderHelper_RestoreRegsState();
        RestoreSerialTimer3IntrHandlers();
        crate::c::volatile_write(
            ((67109384i32) as usize as *mut u16),
            (&raw mut backupIME).read_volatile(),
        );
    }
}
pub(crate) unsafe extern "C" fn EReader_Transfer(eReader: *mut u8) -> u8 {
    unsafe {
        let mut eReader = eReader;
        let mut transferStatus: u8 = 0u8;
        ((eReader).cast::<u16>()).write(
            ((EReaderHandleTransfer(
                1u8,
                ((eReader).wrapping_add(4).cast::<u32>()).read(),
                (((eReader).wrapping_add(8).cast::<*mut u32>()).read()).cast::<u8>(),
                core::ptr::null_mut(),
            )) as u16),
        );
        if ((((((eReader).cast::<u16>()).read()) as i32) & 3i32) == 0i32)
            && ((((((eReader).cast::<u16>()).read()) as i32) & 16i32) != 0)
        {
            transferStatus = 1u8;
        }
        if (((((eReader).cast::<u16>()).read()) as i32) & 8i32) != 0 {
            transferStatus = 2u8;
        }
        if (((((eReader).cast::<u16>()).read()) as i32) & 4i32) != 0 {
            transferStatus = 3u8;
        }
        ((&raw mut gShouldAdvanceLinkState).cast::<u8>()).write(0u8);
        return transferStatus;
    }
}
pub(crate) unsafe extern "C" fn OpenEReaderLink() {
    unsafe {
        crate::c::memset((&raw mut gDecompressionBuffer).cast::<u8>(), 0i32, 8192u32);
        ((&raw mut gLinkType).cast::<u16>()).write(21763u16);
        OpenLink();
        SetSuppressLinkErrorMessage(1u8);
    }
}
pub(crate) unsafe extern "C" fn ValidateEReaderConnection() -> u32 {
    unsafe {
        let mut backupIME: u16 = 0u16;
        (&raw mut backupIME).write_volatile(0u16);
        let mut handshakes = crate::ffi::Align4([0u8; 8]);
        crate::c::volatile_write(
            (&raw mut backupIME),
            ((67109384i32) as usize as *mut u16).read_volatile(),
        );
        crate::c::volatile_write(((67109384i32) as usize as *mut u16), 0u16);
        (((&raw mut handshakes).cast::<u16>()).cast::<u64>()).write(
            (((((&raw mut gLink).cast::<u8>()).wrapping_add(4)).cast::<u16>()).cast::<u64>())
                .read(),
        );
        crate::c::volatile_write(
            ((67109384i32) as usize as *mut u16),
            (&raw mut backupIME).read_volatile(),
        );
        if (((((((&raw mut handshakes).cast::<u16>()).read()) as i32) == 47520i32)
            && ((((((&raw mut handshakes).cast::<u16>()).wrapping_offset(1)).read()) as i32)
                == 52432i32))
            && ((((((&raw mut handshakes).cast::<u16>()).wrapping_offset(2)).read()) as i32)
                == 65535i32))
            && ((((((&raw mut handshakes).cast::<u16>()).wrapping_offset(3)).read()) as i32)
                == 65535i32)
        {
            return 1u32;
        }
        return 0u32;
    }
}
pub(crate) unsafe extern "C" fn IsChildConnected() -> u32 {
    unsafe {
        if ((IsLinkMaster()) != 0) && (((GetLinkPlayerCount_2()) as i32) == 2i32) {
            return 1u32;
        }
        return 0u32;
    }
}
pub(crate) unsafe extern "C" fn TryReceiveCard(state: *mut u8, timer: *mut u16) -> u32 {
    unsafe {
        let mut state = state;
        let mut timer = timer;
        if (((((state).read()) as i32) >= 3i32) && ((((state).read()) as i32) <= 5i32))
            && ((HasLinkErrorOccurred()) != 0)
        {
            (state).write(0u8);
            return 3u32;
        }
        'l1: {
            let __sw1 = (((state).read()) as i32);
            let __matched = __sw1 == 0i32
                || __sw1 == 1i32
                || __sw1 == 2i32
                || __sw1 == 3i32
                || __sw1 == 4i32
                || __sw1 == 5i32;
            if __sw1 == 0i32 {
                if ((IsLinkMaster()) != 0) && (((GetLinkPlayerCount_2()) as i32) > 1i32) {
                    (state).write(1u8);
                } else {
                    if ((((((&raw mut gMain).cast::<u8>())
                        .wrapping_add(46)
                        .cast::<u16>())
                    .read()) as i32)
                        & 2i32)
                        != 0
                    {
                        (state).write(0u8);
                        return 1u32;
                    }
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                if (({
                    let __t2 = ((timer).read()).wrapping_add(1);
                    (timer).write(__t2);
                    __t2
                }) as i32)
                    > 5i32
                {
                    (timer).write(0u16);
                    (state).write(2u8);
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                if ((GetLinkPlayerCount_2()) as i32) == 2i32 {
                    PlaySE(73u16);
                    CheckShouldAdvanceLinkState();
                    (timer).write(0u16);
                    (state).write(3u8);
                } else {
                    if ((((((&raw mut gMain).cast::<u8>())
                        .wrapping_add(46)
                        .cast::<u16>())
                    .read()) as i32)
                        & 2i32)
                        != 0
                    {
                        (state).write(0u8);
                        return 1u32;
                    }
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                if (({
                    let __t3 = ((timer).read()).wrapping_add(1);
                    (timer).write(__t3);
                    __t3
                }) as i32)
                    > 30i32
                {
                    (state).write(0u8);
                    return 5u32;
                }
                if (IsLinkConnectionEstablished()) != 0 {
                    if (((&raw mut gReceivedRemoteLinkPlayers).cast::<u8>()).read()) != 0 {
                        if (IsLinkPlayerDataExchangeComplete()) != 0 {
                            (state).write(0u8);
                            return 2u32;
                        } else {
                            (state).write(4u8);
                        }
                    } else {
                        (state).write(3u8);
                    }
                }
                break 'l1;
            }
            if __sw1 == 4i32 {
                SetCloseLinkCallbackAndType(0u16);
                (state).write(5u8);
                break 'l1;
            }
            if __sw1 == 5i32 {
                if !((((&raw mut gReceivedRemoteLinkPlayers).cast::<u8>()).read()) != 0) {
                    (state).write(0u8);
                    return 4u32;
                }
                break 'l1;
            }
            if !__matched {
                return 0u32;
            }
        }
        return 0u32;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CreateEReaderTask() {
    unsafe {
        let mut data: *mut u8 = core::ptr::null_mut();
        let mut taskId: u8 = CreateTask(Some(Task_EReader), 0u8);
        data = (((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .cast::<u8>();
        ((data).wrapping_add(8)).write(0u8);
        ((data).wrapping_add(9)).write(0u8);
        ((data).wrapping_add(10)).write(0u8);
        ((data).wrapping_add(11)).write(0u8);
        ((data).wrapping_add(12)).write(0u8);
        ((data).wrapping_add(13)).write(0u8);
        ((data).cast::<u16>()).write(0u16);
        ((data).wrapping_add(2).cast::<u16>()).write(0u16);
        ((data).wrapping_add(4).cast::<u16>()).write(0u16);
        ((data).wrapping_add(6).cast::<u16>()).write(0u16);
        ((data).wrapping_add(14)).write(0u8);
        ((data).wrapping_add(16).cast::<*mut u8>()).write(AllocZeroed(64u32));
    }
}
pub(crate) unsafe extern "C" fn ResetTimer(timer: *mut u16) {
    unsafe {
        let mut timer = timer;
        (timer).write(0u16);
    }
}
pub(crate) unsafe extern "C" fn UpdateTimer(timer: *mut u16, time: u16) -> u32 {
    unsafe {
        let mut timer = timer;
        let mut time = time;
        if (({
            let __t1 = ((timer).read()).wrapping_add(1);
            (timer).write(__t1);
            __t1
        }) as i32)
            > ((time) as i32)
        {
            (timer).write(0u16);
            return 1u32;
        }
        return 0u32;
    }
}
pub(crate) unsafe extern "C" fn Task_EReader(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut u8 = (((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .cast::<u8>();
        'l1: {
            let __sw1 = ((((data).wrapping_add(8)).read()) as i32);
            if __sw1 == 0i32 {
                if (PrintMysteryGiftMenuMessage(
                    (data).wrapping_add(9),
                    (&raw mut gJPText_ReceiveMysteryGiftWithEReader).cast::<u8>(),
                )) != 0
                {
                    ((data).wrapping_add(8)).write(1u8);
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                OpenEReaderLink();
                ResetTimer((data).cast::<u16>());
                ((data).wrapping_add(8)).write(2u8);
                break 'l1;
            }
            if __sw1 == 2i32 {
                if (UpdateTimer((data).cast::<u16>(), 10u16)) != 0 {
                    ((data).wrapping_add(8)).write(3u8);
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                if !((IsChildConnected()) != 0) {
                    CloseLink();
                    ((data).wrapping_add(8)).write(4u8);
                } else {
                    ((data).wrapping_add(8)).write(13u8);
                }
                break 'l1;
            }
            if __sw1 == 4i32 {
                if (PrintMysteryGiftMenuMessage(
                    (data).wrapping_add(9),
                    (&raw mut gJPText_SelectConnectFromEReaderMenu).cast::<u8>(),
                )) != 0
                {
                    MG_AddMessageTextPrinter((&raw mut gJPText_SelectConnectWithGBA).cast::<u8>());
                    ResetTimer((data).cast::<u16>());
                    ((data).wrapping_add(8)).write(5u8);
                }
                break 'l1;
            }
            if __sw1 == 5i32 {
                if (UpdateTimer((data).cast::<u16>(), 90u16)) != 0 {
                    OpenEReaderLink();
                    ((data).wrapping_add(8)).write(6u8);
                } else {
                    if ((((((&raw mut gMain).cast::<u8>())
                        .wrapping_add(46)
                        .cast::<u16>())
                    .read()) as i32)
                        & 2i32)
                        != 0
                    {
                        ResetTimer((data).cast::<u16>());
                        PlaySE(5u16);
                        ((data).wrapping_add(8)).write(23u8);
                    }
                }
                break 'l1;
            }
            if __sw1 == 6i32 {
                if ((((((&raw mut gMain).cast::<u8>())
                    .wrapping_add(46)
                    .cast::<u16>())
                .read()) as i32)
                    & 2i32)
                    != 0
                {
                    PlaySE(5u16);
                    CloseLink();
                    ResetTimer((data).cast::<u16>());
                    ((data).wrapping_add(8)).write(23u8);
                } else {
                    if ((GetLinkPlayerCount_2()) as i32) > 1i32 {
                        ResetTimer((data).cast::<u16>());
                        CloseLink();
                        ((data).wrapping_add(8)).write(7u8);
                    } else {
                        if (ValidateEReaderConnection()) != 0 {
                            PlaySE(5u16);
                            CloseLink();
                            ResetTimer((data).cast::<u16>());
                            ((data).wrapping_add(8)).write(8u8);
                        } else {
                            if (UpdateTimer((data).cast::<u16>(), 10u16)) != 0 {
                                CloseLink();
                                OpenEReaderLink();
                                ResetTimer((data).cast::<u16>());
                            }
                        }
                    }
                }
                break 'l1;
            }
            if __sw1 == 7i32 {
                if (PrintMysteryGiftMenuMessage(
                    (data).wrapping_add(9),
                    (&raw mut gJPText_LinkIsIncorrect).cast::<u8>(),
                )) != 0
                {
                    ((data).wrapping_add(8)).write(4u8);
                }
                break 'l1;
            }
            if __sw1 == 8i32 {
                MG_AddMessageTextPrinter((&raw mut gJPText_Connecting).cast::<u8>());
                EReader_Load(
                    (&raw mut gEReaderData).cast::<u8>(),
                    ((((&raw mut gMultiBootProgram_EReader_End).cast::<u8>()) as usize)
                        .wrapping_sub(
                            ((&raw mut gMultiBootProgram_EReader_Start).cast::<u8>()) as usize,
                        ) as i32
                        / 1),
                    ((&raw mut gMultiBootProgram_EReader_Start).cast::<u8>()).cast::<u32>(),
                );
                ((data).wrapping_add(8)).write(9u8);
                break 'l1;
            }
            if __sw1 == 9i32 {
                ((data).wrapping_add(14))
                    .write(EReader_Transfer((&raw mut gEReaderData).cast::<u8>()));
                if ((((data).wrapping_add(14)).read()) as i32) != 0i32 {
                    ((data).wrapping_add(8)).write(10u8);
                }
                break 'l1;
            }
            if __sw1 == 10i32 {
                EReader_Reset((&raw mut gEReaderData).cast::<u8>());
                if ((((data).wrapping_add(14)).read()) as i32) == 3i32 {
                    ((data).wrapping_add(8)).write(20u8);
                } else {
                    if ((((data).wrapping_add(14)).read()) as i32) == 1i32 {
                        ResetTimer((data).cast::<u16>());
                        MG_AddMessageTextPrinter((&raw mut gJPText_PleaseWaitAMoment).cast::<u8>());
                        ((data).wrapping_add(8)).write(11u8);
                    } else {
                        ((data).wrapping_add(8)).write(0u8);
                    }
                }
                break 'l1;
            }
            if __sw1 == 11i32 {
                if (UpdateTimer((data).cast::<u16>(), 840u16)) != 0 {
                    ((data).wrapping_add(8)).write(12u8);
                }
                break 'l1;
            }
            if __sw1 == 12i32 {
                OpenEReaderLink();
                MG_AddMessageTextPrinter((&raw mut gJPText_AllowEReaderToLoadCard).cast::<u8>());
                ((data).wrapping_add(8)).write(13u8);
                break 'l1;
            }
            if __sw1 == 13i32 {
                'l2: {
                    let __sw2 = TryReceiveCard((data).wrapping_add(9), (data).cast::<u16>());
                    if __sw2 == 0u32 {
                        break 'l2;
                    }
                    if __sw2 == 2u32 {
                        MG_AddMessageTextPrinter((&raw mut gJPText_Connecting).cast::<u8>());
                        ((data).wrapping_add(8)).write(14u8);
                        break 'l2;
                    }
                    if __sw2 == 1u32 {
                        PlaySE(5u16);
                        CloseLink();
                        ((data).wrapping_add(8)).write(23u8);
                        break 'l2;
                    }
                    if __sw2 == 5u32 {
                        CloseLink();
                        ((data).wrapping_add(8)).write(21u8);
                        break 'l2;
                    }
                    if __sw2 == 3u32 || __sw2 == 4u32 {
                        CloseLink();
                        ((data).wrapping_add(8)).write(20u8);
                        break 'l2;
                    }
                }
                break 'l1;
            }
            if __sw1 == 14i32 {
                if (HasLinkErrorOccurred()) != 0 {
                    CloseLink();
                    ((data).wrapping_add(8)).write(20u8);
                } else {
                    if (GetBlockReceivedStatus()) != 0 {
                        ResetBlockReceivedFlags();
                        ((data).wrapping_add(8)).write(15u8);
                    }
                }
                break 'l1;
            }
            if __sw1 == 15i32 {
                ((data).wrapping_add(14)).write(ValidateTrainerHillData(
                    (&raw mut gDecompressionBuffer).cast::<u8>(),
                ));
                SetCloseLinkCallbackAndType(((((data).wrapping_add(14)).read()) as u16));
                ((data).wrapping_add(8)).write(16u8);
                break 'l1;
            }
            if __sw1 == 16i32 {
                if !((((&raw mut gReceivedRemoteLinkPlayers).cast::<u8>()).read()) != 0) {
                    if ((((data).wrapping_add(14)).read()) as i32) == 1i32 {
                        ((data).wrapping_add(8)).write(17u8);
                    } else {
                        ((data).wrapping_add(8)).write(20u8);
                    }
                }
                break 'l1;
            }
            if __sw1 == 17i32 {
                if (TryWriteTrainerHill(
                    ((&raw mut gDecompressionBuffer).cast::<u8>()).cast::<u8>(),
                )) != 0
                {
                    MG_AddMessageTextPrinter((&raw mut gJPText_ConnectionComplete).cast::<u8>());
                    ResetTimer((data).cast::<u16>());
                    ((data).wrapping_add(8)).write(18u8);
                } else {
                    ((data).wrapping_add(8)).write(22u8);
                }
                break 'l1;
            }
            if __sw1 == 18i32 {
                if (UpdateTimer((data).cast::<u16>(), 120u16)) != 0 {
                    MG_AddMessageTextPrinter(
                        (&raw mut gJPText_NewTrainerHasComeToHoenn).cast::<u8>(),
                    );
                    PlayFanfare(370u16);
                    ((data).wrapping_add(8)).write(19u8);
                }
                break 'l1;
            }
            if __sw1 == 19i32 {
                if ((IsFanfareTaskInactive()) != 0)
                    && (((((((&raw mut gMain).cast::<u8>())
                        .wrapping_add(46)
                        .cast::<u16>())
                    .read()) as i32)
                        & 3i32)
                        != 0)
                {
                    ((data).wrapping_add(8)).write(26u8);
                }
                break 'l1;
            }
            if __sw1 == 23i32 {
                if (PrintMysteryGiftMenuMessage(
                    (data).wrapping_add(9),
                    (&raw mut gJPText_CardReadingHasBeenHalted).cast::<u8>(),
                )) != 0
                {
                    ((data).wrapping_add(8)).write(26u8);
                }
                break 'l1;
            }
            if __sw1 == 20i32 {
                if (PrintMysteryGiftMenuMessage(
                    (data).wrapping_add(9),
                    (&raw mut gJPText_ConnectionErrorCheckLink).cast::<u8>(),
                )) != 0
                {
                    ((data).wrapping_add(8)).write(0u8);
                }
                break 'l1;
            }
            if __sw1 == 21i32 {
                if (PrintMysteryGiftMenuMessage(
                    (data).wrapping_add(9),
                    (&raw mut gJPText_ConnectionErrorTryAgain).cast::<u8>(),
                )) != 0
                {
                    ((data).wrapping_add(8)).write(0u8);
                }
                break 'l1;
            }
            if __sw1 == 22i32 {
                if (PrintMysteryGiftMenuMessage(
                    (data).wrapping_add(9),
                    (&raw mut gJPText_WriteErrorUnableToSaveData).cast::<u8>(),
                )) != 0
                {
                    ((data).wrapping_add(8)).write(0u8);
                }
                break 'l1;
            }
            if __sw1 == 26i32 {
                Free(((data).wrapping_add(16).cast::<*mut u8>()).read());
                DestroyTask(taskId);
                SetMainCallback2(Some(MainCB_FreeAllBuffersAndReturnToInitTitleScreen));
                break 'l1;
            }
        }
    }
}
