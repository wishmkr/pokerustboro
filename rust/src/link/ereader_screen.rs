//! Translated from `src/ereader_screen.c` by tools/rustport/c2rs.py.
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
    clippy::useless_transmute,
    dead_code,
    unused_assignments,
    unused_variables
)]

use crate::agb_main::RestoreSerialTimer3IntrHandlers;
use crate::agb_main::gMain;
#[allow(unused_imports)]
use crate::c::*;
#[allow(unused_imports)]
use crate::consts::*;
use crate::ereader_helpers::{
    EReaderHandleTransfer, EReaderHelper_ClearSendRecvMgr, EReaderHelper_RestoreRegsState,
    EReaderHelper_SaveRegsState, EReaderHelper_SerialCallback, EReaderHelper_Timer3Callback,
    TryWriteTrainerHill, ValidateTrainerHillData,
};
use crate::link::{
    CheckShouldAdvanceLinkState, CloseLink, GetBlockReceivedStatus, GetLinkPlayerCount_2,
    HasLinkErrorOccurred, IsLinkConnectionEstablished, IsLinkMaster,
    IsLinkPlayerDataExchangeComplete, OpenLink, ResetBlockReceivedFlags,
    SetCloseLinkCallbackAndType, SetSuppressLinkErrorMessage, gLink, gLinkType,
    gReceivedRemoteLinkPlayers, gShouldAdvanceLinkState,
};
use crate::mystery_gift_menu::{
    MG_AddMessageTextPrinter, MainCB_FreeAllBuffersAndReturnToInitTitleScreen,
    PrintMysteryGiftMenuMessage,
};
use crate::sound::{IsFanfareTaskInactive, PlayFanfare, PlaySE};
use crate::task::DestroyTask;
use crate::task::gTasks;
#[allow(unused_imports)]
use crate::types::*;
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
/// `Free` with this module's view of its types.
#[inline]
unsafe fn Free(a0: *mut c_void) {
    unsafe {
        crate::malloc::Free(a0 as _);
    }
}

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

#[unsafe(link_section = "common_data")]
pub static mut gUnknownSpace: Aligned<CArray<u8, 64>> = Aligned(unsafe { zeroed() });
#[unsafe(link_section = "common_data")]
pub static mut gEReaderData: EReaderData = unsafe { zeroed() };

/// `AllocZeroed` with this module's view of its types.
#[inline]
unsafe fn AllocZeroed(a0: u32) -> *mut c_void {
    unsafe { crate::malloc::AllocZeroed(a0) as *mut c_void }
}
/// `SetMainCallback2` with this module's view of its types.
#[inline]
unsafe fn SetMainCallback2(a0: Option<unsafe fn()>) {
    unsafe {
        crate::agb_main::SetMainCallback2(core::mem::transmute(a0));
    }
}

unsafe fn EReader_Load(eReader: *mut EReaderData, size: i32, data: *mut u32) {
    let mut backupIME: u16 = 0;
    volatile_write(
        &raw mut backupIME,
        (67109384_usize as *mut u16).read_volatile(),
    );
    volatile_write(67109384_usize as *mut u16, 0);
    (*(&raw const crate::agb_main::gIntrTable)
        .cast::<CArray<Option<crate::agb_main::IntrFunc>, 0>>()
        .cast_mut())[1] = Some(EReaderHelper_SerialCallback);
    (*(&raw const crate::agb_main::gIntrTable)
        .cast::<CArray<Option<crate::agb_main::IntrFunc>, 0>>()
        .cast_mut())[2] = Some(EReaderHelper_Timer3Callback);
    EReaderHelper_SaveRegsState();
    EReaderHelper_ClearSendRecvMgr();
    volatile_write(
        0x4000200_usize as *mut u16,
        (0x4000200_usize as *mut u16).read_volatile() | INTR_FLAG_VCOUNT,
    );
    volatile_write(
        67109384_usize as *mut u16,
        (&raw mut backupIME).read_volatile(),
    );
    (*eReader).status = 0;
    (*eReader).size = size as u32;
    (*eReader).data = data;
}
unsafe fn EReader_Reset(eReader: *mut EReaderData) {
    let mut backupIME: u16 = 0;
    volatile_write(
        &raw mut backupIME,
        (67109384_usize as *mut u16).read_volatile(),
    );
    volatile_write(67109384_usize as *mut u16, 0);
    EReaderHelper_ClearSendRecvMgr();
    EReaderHelper_RestoreRegsState();
    RestoreSerialTimer3IntrHandlers();
    volatile_write(
        67109384_usize as *mut u16,
        (&raw mut backupIME).read_volatile(),
    );
}
unsafe fn EReader_Transfer(eReader: *mut EReaderData) -> u8 {
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
    transferStatus
}
unsafe fn OpenEReaderLink() {
    memset(
        (*(&raw const crate::decompress::gDecompressionBuffer)
            .cast::<CArray<u8, 16384>>()
            .cast_mut())
        .as_mut_ptr(),
        0,
        0x2000,
    );
    gLinkType = LINKTYPE_EREADER_EM;
    OpenLink();
    SetSuppressLinkErrorMessage(TRUE);
}
unsafe fn ValidateEReaderConnection() -> u32 {
    let mut backupIME: u16 = 0;
    volatile_write(&raw mut backupIME, 0);
    let mut handshakes: CArray<u16, 4> = zeroed();
    volatile_write(
        &raw mut backupIME,
        (67109384_usize as *mut u16).read_volatile(),
    );
    volatile_write(67109384_usize as *mut u16, 0);
    *(handshakes.as_mut_ptr() as *mut u64) = *(gLink.handshakeBuffer.as_mut_ptr() as *mut u64);
    volatile_write(
        67109384_usize as *mut u16,
        (&raw mut backupIME).read_volatile(),
    );
    if handshakes[0] == SLAVE_HANDSHAKE
        && handshakes[1] == EREADER_HANDSHAKE
        && handshakes[2] == 0xFFFF
        && handshakes[3] == 0xFFFF
    {
        return TRUE as u32;
    }
    FALSE as u32
}
unsafe fn IsChildConnected() -> u32 {
    if IsLinkMaster() != 0 && GetLinkPlayerCount_2() == 2 {
        return TRUE as u32;
    }
    FALSE as u32
}
unsafe fn TryReceiveCard(state: *mut u8, timer: *mut u16) -> u32 {
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
    RECV_ACTIVE
}
pub unsafe fn CreateEReaderTask() {
    let mut data: *mut EReaderTaskData = null_mut();
    let taskId: u8 = CreateTask(Some(Task_EReader), 0);
    data = (*gTasks.as_ptr())[taskId].data.as_mut_ptr() as *mut EReaderTaskData;
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
unsafe fn ResetTimer(timer: *mut u16) {
    *timer = 0;
}
unsafe fn UpdateTimer(timer: *mut u16, time: u16) -> u32 {
    if ({
        *timer += 1;
        *timer
    }) > time
    {
        *timer = 0;
        return TRUE as u32;
    }
    FALSE as u32
}
pub(crate) unsafe fn Task_EReader(taskId: u8) {
    let data: *mut EReaderTaskData =
        (*gTasks.as_ptr())[taskId].data.as_mut_ptr() as *mut EReaderTaskData;
    match (*data).state {
        ER_STATE_START => {
            if PrintMysteryGiftMenuMessage(
                &raw mut (*data).textState,
                (*(&raw const crate::data::strings::gJPText_ReceiveMysteryGiftWithEReader)
                    .cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
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
                (*(&raw const crate::data::strings::gJPText_SelectConnectFromEReaderMenu)
                    .cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
            ) != 0
            {
                MG_AddMessageTextPrinter(
                    (*(&raw const crate::data::strings::gJPText_SelectConnectWithGBA)
                        .cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut(),
                );
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
                (*(&raw const crate::data::strings::gJPText_LinkIsIncorrect)
                    .cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
            ) != 0
            {
                (*data).state = ER_STATE_MSG_SELECT_CONNECT;
            }
        }
        ER_STATE_CONNECTING => {
            MG_AddMessageTextPrinter(
                (*(&raw const crate::data::strings::gJPText_Connecting).cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut(),
            );
            EReader_Load(
                &raw mut gEReaderData,
                ((*crate::asmdata::gMultiBootProgram_EReader_End.cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut() as usize)
                    .wrapping_sub(
                        (*crate::asmdata::gMultiBootProgram_EReader_Start.cast::<CArray<u8, 0>>())
                            .as_ptr()
                            .cast_mut() as usize,
                    ) as i32,
                (*crate::asmdata::gMultiBootProgram_EReader_Start.cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut() as *mut u32,
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
                MG_AddMessageTextPrinter(
                    (*(&raw const crate::data::strings::gJPText_PleaseWaitAMoment)
                        .cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut(),
                );
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
            MG_AddMessageTextPrinter(
                (*(&raw const crate::data::strings::gJPText_AllowEReaderToLoadCard)
                    .cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
            );
            (*data).state = ER_STATE_LOAD_CARD;
        }
        ER_STATE_LOAD_CARD => {
            match TryReceiveCard(&raw mut (*data).textState, &raw mut (*data).timer) {
                RECV_ACTIVE => {}
                RECV_SUCCESS => {
                    MG_AddMessageTextPrinter(
                        (*(&raw const crate::data::strings::gJPText_Connecting)
                            .cast::<CArray<u8, 0>>())
                        .as_ptr()
                        .cast_mut(),
                    );
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
                (*(&raw const crate::decompress::gDecompressionBuffer)
                    .cast::<CArray<u8, 16384>>()
                    .cast_mut())
                .as_mut_ptr() as *mut EReaderTrainerHillSet,
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
            if TryWriteTrainerHill(
                &raw mut (*(&raw const crate::decompress::gDecompressionBuffer)
                    .cast::<CArray<u8, 16384>>()
                    .cast_mut()) as *mut EReaderTrainerHillSet,
            ) != 0
            {
                MG_AddMessageTextPrinter(
                    (*(&raw const crate::data::strings::gJPText_ConnectionComplete)
                        .cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut(),
                );
                ResetTimer(&raw mut (*data).timer);
                (*data).state = ER_STATE_SUCCESS_MSG;
            } else {
                (*data).state = ER_STATE_SAVE_FAILED;
            }
        }
        ER_STATE_SUCCESS_MSG => {
            if UpdateTimer(&raw mut (*data).timer, 120) != 0 {
                MG_AddMessageTextPrinter(
                    (*(&raw const crate::data::strings::gJPText_NewTrainerHasComeToHoenn)
                        .cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut(),
                );
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
                (*(&raw const crate::data::strings::gJPText_CardReadingHasBeenHalted)
                    .cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
            ) != 0
            {
                (*data).state = ER_STATE_END;
            }
        }
        ER_STATE_LINK_ERROR => {
            if PrintMysteryGiftMenuMessage(
                &raw mut (*data).textState,
                (*(&raw const crate::data::strings::gJPText_ConnectionErrorCheckLink)
                    .cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
            ) != 0
            {
                (*data).state = ER_STATE_START;
            }
        }
        ER_STATE_LINK_ERROR_TRY_AGAIN => {
            if PrintMysteryGiftMenuMessage(
                &raw mut (*data).textState,
                (*(&raw const crate::data::strings::gJPText_ConnectionErrorTryAgain)
                    .cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
            ) != 0
            {
                (*data).state = ER_STATE_START;
            }
        }
        ER_STATE_SAVE_FAILED => {
            if PrintMysteryGiftMenuMessage(
                &raw mut (*data).textState,
                (*(&raw const crate::data::strings::gJPText_WriteErrorUnableToSaveData)
                    .cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
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
