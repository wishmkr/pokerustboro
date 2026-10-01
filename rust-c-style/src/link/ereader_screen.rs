//! Translated from `src/ereader_screen.c` by tools/rustport/c2rs.py.
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

/// `struct EReaderData`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct EReaderData {
    pub status: u16,
    pub size: u32,
    pub data: *mut u32,
}

unsafe impl Sync for EReaderData {}

/// `struct EReaderTaskData`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct EReaderTaskData {
    pub timer: u16,
    pub unused1: u16,
    pub unused2: u16,
    pub unused3: u16,
    pub state: u8,
    pub textState: u8,
    pub unused4: u8,
    pub unused5: u8,
    pub unused6: u8,
    pub unused7: u8,
    pub status: u8,
    pub unusedBuffer: *mut u8,
}

unsafe impl Sync for EReaderTaskData {}

#[cfg(target_arch = "arm")]
const _: () = {
    #[allow(unused_imports)]
    use core::mem::{offset_of, size_of};
    assert!(size_of::<EReaderData>() == 12);
    assert!(offset_of!(EReaderData, status) == 0);
    assert!(offset_of!(EReaderData, size) == 4);
    assert!(offset_of!(EReaderData, data) == 8);
    assert!(size_of::<EReaderTaskData>() == 20);
    assert!(offset_of!(EReaderTaskData, timer) == 0);
    assert!(offset_of!(EReaderTaskData, unused1) == 2);
    assert!(offset_of!(EReaderTaskData, unused2) == 4);
    assert!(offset_of!(EReaderTaskData, unused3) == 6);
    assert!(offset_of!(EReaderTaskData, state) == 8);
    assert!(offset_of!(EReaderTaskData, textState) == 9);
    assert!(offset_of!(EReaderTaskData, unused4) == 10);
    assert!(offset_of!(EReaderTaskData, unused5) == 11);
    assert!(offset_of!(EReaderTaskData, unused6) == 12);
    assert!(offset_of!(EReaderTaskData, unused7) == 13);
    assert!(offset_of!(EReaderTaskData, status) == 14);
    assert!(offset_of!(EReaderTaskData, unusedBuffer) == 16);
};

const ER_STATE_CANCELED_CARD_READ: u8 = 23;
const ER_STATE_CONNECTING: u8 = 8;
const ER_STATE_END: u8 = 26;
const ER_STATE_INCORRECT_LINK: u8 = 7;
const ER_STATE_INIT_LINK: u8 = 1;
const ER_STATE_INIT_LINK_CHECK: u8 = 3;
const ER_STATE_INIT_LINK_WAIT: u8 = 2;
const ER_STATE_LINK_ERROR: u8 = 20;
const ER_STATE_LINK_ERROR_TRY_AGAIN: u8 = 21;
const ER_STATE_LOAD_CARD: u8 = 13;
const ER_STATE_LOAD_CARD_START: u8 = 12;
const ER_STATE_MSG_SELECT_CONNECT: u8 = 4;
const ER_STATE_MSG_SELECT_CONNECT_WAIT: u8 = 5;
const ER_STATE_SAVE: u8 = 17;
const ER_STATE_SAVE_FAILED: u8 = 22;
const ER_STATE_START: u8 = 0;
const ER_STATE_SUCCESS_END: u8 = 19;
const ER_STATE_SUCCESS_MSG: u8 = 18;
const ER_STATE_TRANSFER: u8 = 9;
const ER_STATE_TRANSFER_END: u8 = 10;
const ER_STATE_TRANSFER_SUCCESS: u8 = 11;
const ER_STATE_TRY_LINK: u8 = 6;
const ER_STATE_VALIDATE_CARD: u8 = 15;
const ER_STATE_WAIT_DISCONNECT: u8 = 16;
const ER_STATE_WAIT_RECV_CARD: u8 = 14;
const RECV_ACTIVE: u32 = 0;
const RECV_CANCELED: u32 = 1;
const RECV_DISCONNECTED: u32 = 4;
const RECV_ERROR: u32 = 3;
const RECV_STATE_EXCHANGE: u8 = 3;
const RECV_STATE_INIT: u8 = 0;
const RECV_STATE_START: u8 = 2;
const RECV_STATE_START_DISCONNECT: u8 = 4;
const RECV_STATE_WAIT_DISCONNECT: u8 = 5;
const RECV_STATE_WAIT_START: u8 = 1;
const RECV_SUCCESS: u32 = 2;
const RECV_TIMEOUT: u32 = 5;
const TRANSFER_ACTIVE: u8 = 0;
const TRANSFER_CANCELED: u8 = 2;
const TRANSFER_SUCCESS: u8 = 1;
const TRANSFER_TIMEOUT: u8 = 3;

#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gUnknownSpace: Aligned<CArray<u8, 64>> = Aligned(unsafe { zeroed() });
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gEReaderData: EReaderData = unsafe { zeroed() };

unsafe extern "C" {
    static mut gDecompressionBuffer: CArray<u8, 16384>;
    static mut gIntrTable: CArray<Option<unsafe extern "C" fn()>, 0>;
    static gJPText_AllowEReaderToLoadCard: CArray<u8, 0>;
    static gJPText_CardReadingHasBeenHalted: CArray<u8, 0>;
    static gJPText_Connecting: CArray<u8, 0>;
    static gJPText_ConnectionComplete: CArray<u8, 0>;
    static gJPText_ConnectionErrorCheckLink: CArray<u8, 0>;
    static gJPText_ConnectionErrorTryAgain: CArray<u8, 0>;
    static gJPText_LinkIsIncorrect: CArray<u8, 0>;
    static gJPText_NewTrainerHasComeToHoenn: CArray<u8, 0>;
    static gJPText_PleaseWaitAMoment: CArray<u8, 0>;
    static gJPText_ReceiveMysteryGiftWithEReader: CArray<u8, 0>;
    static gJPText_SelectConnectFromEReaderMenu: CArray<u8, 0>;
    static gJPText_SelectConnectWithGBA: CArray<u8, 0>;
    static gJPText_WriteErrorUnableToSaveData: CArray<u8, 0>;
    static mut gLink: Link;
    static mut gLinkType: u16;
    static mut gMain: Main;
    static gMultiBootProgram_EReader_End: CArray<u8, 0>;
    static gMultiBootProgram_EReader_Start: CArray<u8, 0>;
    static mut gReceivedRemoteLinkPlayers: u8;
    static mut gShouldAdvanceLinkState: u8;
    static mut gTasks: CArray<Task, 0>;
    fn AllocZeroed(a0: u32) -> *mut c_void;
    fn CheckShouldAdvanceLinkState();
    fn CloseLink();
    fn CreateTask(a0: Option<unsafe extern "C" fn(u8)>, a1: u8) -> u8;
    fn DestroyTask(a0: u8);
    fn EReaderHandleTransfer(a0: u8, a1: u32, a2: *mut c_void, a3: *mut c_void) -> i32;
    fn EReaderHelper_ClearSendRecvMgr();
    fn EReaderHelper_RestoreRegsState();
    fn EReaderHelper_SaveRegsState();
    fn EReaderHelper_SerialCallback();
    fn EReaderHelper_Timer3Callback();
    fn Free(a0: *mut c_void);
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
    fn TryWriteTrainerHill(a0: *mut EReaderTrainerHillSet) -> u32;
    fn ValidateTrainerHillData(a0: *mut EReaderTrainerHillSet) -> u8;
}

pub(crate) unsafe extern "C" fn EReader_Load(eReader: *mut EReaderData, size: i32, data: *mut u32) {
    let mut backupIME: u16 = 0;
    volatile_write(
        &raw mut backupIME,
        (67109384 as usize as *mut u16).read_volatile(),
    );
    volatile_write(67109384 as usize as *mut u16, 0);
    gIntrTable[1] = Some(EReaderHelper_SerialCallback);
    gIntrTable[2] = Some(EReaderHelper_Timer3Callback);
    EReaderHelper_SaveRegsState();
    EReaderHelper_ClearSendRecvMgr();
    volatile_write(
        0x4000200 as usize as *mut u16,
        (0x4000200 as usize as *mut u16).read_volatile() | INTR_FLAG_VCOUNT,
    );
    volatile_write(
        67109384 as usize as *mut u16,
        (&raw mut backupIME).read_volatile(),
    );
    (*eReader).status = 0;
    (*eReader).size = size as u32;
    (*eReader).data = data;
}
pub(crate) unsafe extern "C" fn EReader_Reset(eReader: *mut EReaderData) {
    let mut backupIME: u16 = 0;
    volatile_write(
        &raw mut backupIME,
        (67109384 as usize as *mut u16).read_volatile(),
    );
    volatile_write(67109384 as usize as *mut u16, 0);
    EReaderHelper_ClearSendRecvMgr();
    EReaderHelper_RestoreRegsState();
    RestoreSerialTimer3IntrHandlers();
    volatile_write(
        67109384 as usize as *mut u16,
        (&raw mut backupIME).read_volatile(),
    );
}
pub(crate) unsafe extern "C" fn EReader_Transfer(eReader: *mut EReaderData) -> u8 {
    let mut transferStatus: u8 = TRANSFER_ACTIVE;
    (*eReader).status = EReaderHandleTransfer(
        TRUE,
        (*eReader).size,
        (*eReader).data as *mut c_void,
        null_mut(),
    ) as u16;
    if (*eReader).status as i32 & EREADER_XFER_MASK == 0
        && (*eReader).status as i32 & EREADER_CHECKSUM_OK_MASK != 0
    {
        transferStatus = TRANSFER_SUCCESS;
    }
    if (*eReader).status as i32 & EREADER_CANCEL_KEY_MASK != 0 {
        transferStatus = TRANSFER_CANCELED;
    }
    if (*eReader).status as i32 & EREADER_CANCEL_TIMEOUT_MASK != 0 {
        transferStatus = TRANSFER_TIMEOUT;
    }
    gShouldAdvanceLinkState = 0;
    return transferStatus;
}
pub(crate) unsafe extern "C" fn OpenEReaderLink() {
    memset(gDecompressionBuffer.as_mut_ptr(), 0, 0x2000);
    gLinkType = LINKTYPE_EREADER_EM;
    OpenLink();
    SetSuppressLinkErrorMessage(TRUE);
}
pub(crate) unsafe extern "C" fn ValidateEReaderConnection() -> u32 {
    let mut backupIME: u16 = 0;
    volatile_write(&raw mut backupIME, 0);
    let mut handshakes: CArray<u16, 4> = zeroed();
    volatile_write(
        &raw mut backupIME,
        (67109384 as usize as *mut u16).read_volatile(),
    );
    volatile_write(67109384 as usize as *mut u16, 0);
    *(handshakes.as_mut_ptr() as *mut u64) = *(gLink.handshakeBuffer.as_mut_ptr() as *mut u64);
    volatile_write(
        67109384 as usize as *mut u16,
        (&raw mut backupIME).read_volatile(),
    );
    if handshakes[0] == SLAVE_HANDSHAKE
        && handshakes[1] == EREADER_HANDSHAKE
        && handshakes[2] == 0xFFFF
        && handshakes[3] == 0xFFFF
    {
        return TRUE as u32;
    }
    return FALSE as u32;
}
pub(crate) unsafe extern "C" fn IsChildConnected() -> u32 {
    if IsLinkMaster() != 0 && GetLinkPlayerCount_2() == 2 {
        return TRUE as u32;
    }
    return FALSE as u32;
}
pub(crate) unsafe extern "C" fn TryReceiveCard(state: *mut u8, timer: *mut u16) -> u32 {
    if *state >= RECV_STATE_EXCHANGE
        && *state <= RECV_STATE_WAIT_DISCONNECT
        && HasLinkErrorOccurred() != 0
    {
        *state = 0;
        return RECV_ERROR;
    }
    match *state {
        RECV_STATE_INIT => {
            if IsLinkMaster() != 0 && GetLinkPlayerCount_2() > 1 {
                *state = RECV_STATE_WAIT_START;
            } else if gMain.newKeys as i32 & B_BUTTON != 0 {
                *state = 0;
                return RECV_CANCELED;
            }
        }
        RECV_STATE_WAIT_START => {
            if ({
                *timer += 1;
                *timer
            }) > 5
            {
                *timer = 0;
                *state = RECV_STATE_START;
            }
        }
        RECV_STATE_START => {
            if GetLinkPlayerCount_2() == 2 {
                PlaySE(SE_DING_DONG);
                CheckShouldAdvanceLinkState();
                *timer = 0;
                *state = RECV_STATE_EXCHANGE;
            } else if gMain.newKeys as i32 & B_BUTTON != 0 {
                *state = 0;
                return RECV_CANCELED;
            }
        }
        RECV_STATE_EXCHANGE => {
            if ({
                *timer += 1;
                *timer
            }) > 30
            {
                *state = 0;
                return RECV_TIMEOUT;
            }
            if IsLinkConnectionEstablished() != 0 {
                if gReceivedRemoteLinkPlayers != 0 {
                    if IsLinkPlayerDataExchangeComplete() != 0 {
                        *state = 0;
                        return RECV_SUCCESS;
                    } else {
                        *state = RECV_STATE_START_DISCONNECT;
                    }
                } else {
                    *state = RECV_STATE_EXCHANGE;
                }
            }
        }
        RECV_STATE_START_DISCONNECT => {
            SetCloseLinkCallbackAndType(0);
            *state = RECV_STATE_WAIT_DISCONNECT;
        }
        RECV_STATE_WAIT_DISCONNECT => {
            if gReceivedRemoteLinkPlayers == 0 {
                *state = 0;
                return RECV_DISCONNECTED;
            }
        }
        _ => {
            return RECV_ACTIVE;
        }
    }
    return RECV_ACTIVE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CreateEReaderTask() {
    let mut data: *mut EReaderTaskData = null_mut();
    let mut taskId: u8 = CreateTask(Some(Task_EReader), 0);
    data = gTasks[taskId].data.as_mut_ptr() as *mut EReaderTaskData;
    (*data).state = 0;
    (*data).textState = 0;
    (*data).unused4 = 0;
    (*data).unused5 = 0;
    (*data).unused6 = 0;
    (*data).unused7 = 0;
    (*data).timer = 0;
    (*data).unused1 = 0;
    (*data).unused2 = 0;
    (*data).unused3 = 0;
    (*data).status = 0;
    (*data).unusedBuffer = AllocZeroed(CLIENT_MAX_MSG_SIZE) as *mut u8;
}
pub(crate) unsafe extern "C" fn ResetTimer(timer: *mut u16) {
    *timer = 0;
}
pub(crate) unsafe extern "C" fn UpdateTimer(timer: *mut u16, time: u16) -> u32 {
    if ({
        *timer += 1;
        *timer
    }) > time
    {
        *timer = 0;
        return TRUE as u32;
    }
    return FALSE as u32;
}
pub(crate) unsafe extern "C" fn Task_EReader(taskId: u8) {
    let mut data: *mut EReaderTaskData = gTasks[taskId].data.as_mut_ptr() as *mut EReaderTaskData;
    match (*data).state {
        ER_STATE_START => {
            if PrintMysteryGiftMenuMessage(
                &raw mut (*data).textState,
                gJPText_ReceiveMysteryGiftWithEReader.as_ptr().cast_mut(),
            ) != 0
            {
                (*data).state = ER_STATE_INIT_LINK;
            }
        }
        ER_STATE_INIT_LINK => {
            OpenEReaderLink();
            ResetTimer(&raw mut (*data).timer);
            (*data).state = ER_STATE_INIT_LINK_WAIT;
        }
        ER_STATE_INIT_LINK_WAIT => {
            if UpdateTimer(&raw mut (*data).timer, 10) != 0 {
                (*data).state = ER_STATE_INIT_LINK_CHECK;
            }
        }
        ER_STATE_INIT_LINK_CHECK => {
            if IsChildConnected() == 0 {
                CloseLink();
                (*data).state = ER_STATE_MSG_SELECT_CONNECT;
            } else {
                (*data).state = ER_STATE_LOAD_CARD;
            }
        }
        ER_STATE_MSG_SELECT_CONNECT => {
            if PrintMysteryGiftMenuMessage(
                &raw mut (*data).textState,
                gJPText_SelectConnectFromEReaderMenu.as_ptr().cast_mut(),
            ) != 0
            {
                MG_AddMessageTextPrinter(gJPText_SelectConnectWithGBA.as_ptr().cast_mut());
                ResetTimer(&raw mut (*data).timer);
                (*data).state = ER_STATE_MSG_SELECT_CONNECT_WAIT;
            }
        }
        ER_STATE_MSG_SELECT_CONNECT_WAIT => {
            if UpdateTimer(&raw mut (*data).timer, 90) != 0 {
                OpenEReaderLink();
                (*data).state = ER_STATE_TRY_LINK;
            } else if gMain.newKeys as i32 & B_BUTTON != 0 {
                ResetTimer(&raw mut (*data).timer);
                PlaySE(SE_SELECT);
                (*data).state = ER_STATE_CANCELED_CARD_READ;
            }
        }
        ER_STATE_TRY_LINK => {
            if gMain.newKeys as i32 & B_BUTTON != 0 {
                PlaySE(SE_SELECT);
                CloseLink();
                ResetTimer(&raw mut (*data).timer);
                (*data).state = ER_STATE_CANCELED_CARD_READ;
            } else if GetLinkPlayerCount_2() > 1 {
                ResetTimer(&raw mut (*data).timer);
                CloseLink();
                (*data).state = ER_STATE_INCORRECT_LINK;
            } else if ValidateEReaderConnection() != 0 {
                PlaySE(SE_SELECT);
                CloseLink();
                ResetTimer(&raw mut (*data).timer);
                (*data).state = ER_STATE_CONNECTING;
            } else if UpdateTimer(&raw mut (*data).timer, 10) != 0 {
                CloseLink();
                OpenEReaderLink();
                ResetTimer(&raw mut (*data).timer);
            }
        }
        ER_STATE_INCORRECT_LINK => {
            if PrintMysteryGiftMenuMessage(
                &raw mut (*data).textState,
                gJPText_LinkIsIncorrect.as_ptr().cast_mut(),
            ) != 0
            {
                (*data).state = ER_STATE_MSG_SELECT_CONNECT;
            }
        }
        ER_STATE_CONNECTING => {
            MG_AddMessageTextPrinter(gJPText_Connecting.as_ptr().cast_mut());
            EReader_Load(
                &raw mut gEReaderData,
                (gMultiBootProgram_EReader_End.as_ptr().cast_mut() as usize)
                    .wrapping_sub(gMultiBootProgram_EReader_Start.as_ptr().cast_mut() as usize)
                    as i32,
                gMultiBootProgram_EReader_Start.as_ptr().cast_mut() as *mut u32,
            );
            (*data).state = ER_STATE_TRANSFER;
        }
        ER_STATE_TRANSFER => {
            (*data).status = EReader_Transfer(&raw mut gEReaderData);
            if (*data).status != TRANSFER_ACTIVE {
                (*data).state = ER_STATE_TRANSFER_END;
            }
        }
        ER_STATE_TRANSFER_END => {
            EReader_Reset(&raw mut gEReaderData);
            if (*data).status == TRANSFER_TIMEOUT {
                (*data).state = ER_STATE_LINK_ERROR;
            } else if (*data).status == TRANSFER_SUCCESS {
                ResetTimer(&raw mut (*data).timer);
                MG_AddMessageTextPrinter(gJPText_PleaseWaitAMoment.as_ptr().cast_mut());
                (*data).state = ER_STATE_TRANSFER_SUCCESS;
            } else {
                (*data).state = ER_STATE_START;
            }
        }
        ER_STATE_TRANSFER_SUCCESS => {
            if UpdateTimer(&raw mut (*data).timer, 840) != 0 {
                (*data).state = ER_STATE_LOAD_CARD_START;
            }
        }
        ER_STATE_LOAD_CARD_START => {
            OpenEReaderLink();
            MG_AddMessageTextPrinter(gJPText_AllowEReaderToLoadCard.as_ptr().cast_mut());
            (*data).state = ER_STATE_LOAD_CARD;
        }
        ER_STATE_LOAD_CARD => {
            match TryReceiveCard(&raw mut (*data).textState, &raw mut (*data).timer) {
                RECV_ACTIVE => {}
                RECV_SUCCESS => {
                    MG_AddMessageTextPrinter(gJPText_Connecting.as_ptr().cast_mut());
                    (*data).state = ER_STATE_WAIT_RECV_CARD;
                }
                RECV_CANCELED => {
                    PlaySE(SE_SELECT);
                    CloseLink();
                    (*data).state = ER_STATE_CANCELED_CARD_READ;
                }
                RECV_TIMEOUT => {
                    CloseLink();
                    (*data).state = ER_STATE_LINK_ERROR_TRY_AGAIN;
                }
                RECV_ERROR | RECV_DISCONNECTED => {
                    CloseLink();
                    (*data).state = ER_STATE_LINK_ERROR;
                }
                _ => {}
            }
        }
        ER_STATE_WAIT_RECV_CARD => {
            if HasLinkErrorOccurred() != 0 {
                CloseLink();
                (*data).state = ER_STATE_LINK_ERROR;
            } else if GetBlockReceivedStatus() != 0 {
                ResetBlockReceivedFlags();
                (*data).state = ER_STATE_VALIDATE_CARD;
            }
        }
        ER_STATE_VALIDATE_CARD => {
            (*data).status = ValidateTrainerHillData(
                gDecompressionBuffer.as_mut_ptr() as *mut EReaderTrainerHillSet
            );
            SetCloseLinkCallbackAndType((*data).status as u16);
            (*data).state = ER_STATE_WAIT_DISCONNECT;
        }
        ER_STATE_WAIT_DISCONNECT => {
            if gReceivedRemoteLinkPlayers == 0 {
                if (*data).status == TRUE {
                    (*data).state = ER_STATE_SAVE;
                } else {
                    (*data).state = ER_STATE_LINK_ERROR;
                }
            }
        }
        ER_STATE_SAVE => {
            if TryWriteTrainerHill(&raw mut gDecompressionBuffer as *mut EReaderTrainerHillSet) != 0
            {
                MG_AddMessageTextPrinter(gJPText_ConnectionComplete.as_ptr().cast_mut());
                ResetTimer(&raw mut (*data).timer);
                (*data).state = ER_STATE_SUCCESS_MSG;
            } else {
                (*data).state = ER_STATE_SAVE_FAILED;
            }
        }
        ER_STATE_SUCCESS_MSG => {
            if UpdateTimer(&raw mut (*data).timer, 120) != 0 {
                MG_AddMessageTextPrinter(gJPText_NewTrainerHasComeToHoenn.as_ptr().cast_mut());
                PlayFanfare(MUS_OBTAIN_ITEM);
                (*data).state = ER_STATE_SUCCESS_END;
            }
        }
        ER_STATE_SUCCESS_END => {
            if IsFanfareTaskInactive() != 0 && gMain.newKeys as i32 & 3 != 0 {
                (*data).state = ER_STATE_END;
            }
        }
        ER_STATE_CANCELED_CARD_READ => {
            if PrintMysteryGiftMenuMessage(
                &raw mut (*data).textState,
                gJPText_CardReadingHasBeenHalted.as_ptr().cast_mut(),
            ) != 0
            {
                (*data).state = ER_STATE_END;
            }
        }
        ER_STATE_LINK_ERROR => {
            if PrintMysteryGiftMenuMessage(
                &raw mut (*data).textState,
                gJPText_ConnectionErrorCheckLink.as_ptr().cast_mut(),
            ) != 0
            {
                (*data).state = ER_STATE_START;
            }
        }
        ER_STATE_LINK_ERROR_TRY_AGAIN => {
            if PrintMysteryGiftMenuMessage(
                &raw mut (*data).textState,
                gJPText_ConnectionErrorTryAgain.as_ptr().cast_mut(),
            ) != 0
            {
                (*data).state = ER_STATE_START;
            }
        }
        ER_STATE_SAVE_FAILED => {
            if PrintMysteryGiftMenuMessage(
                &raw mut (*data).textState,
                gJPText_WriteErrorUnableToSaveData.as_ptr().cast_mut(),
            ) != 0
            {
                (*data).state = ER_STATE_START;
            }
        }
        ER_STATE_END => {
            Free((*data).unusedBuffer as *mut c_void);
            DestroyTask(taskId);
            SetMainCallback2(Some(MainCB_FreeAllBuffersAndReturnToInitTitleScreen));
        }
        _ => {}
    }
}
