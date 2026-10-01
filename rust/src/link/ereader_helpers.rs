//! Translated from `src/ereader_helpers.c` by tools/rustport/c2rs.py.
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
    clippy::if_same_then_else,
    dead_code,
    unused_assignments
)]

#[allow(unused_imports)]
use crate::c::*;
#[allow(unused_imports)]
use crate::consts::*;
use crate::link::gShouldAdvanceLinkState;
use crate::load_save::gSaveBlock1Ptr;
use crate::save::{TryReadSpecialSaveSector, TryWriteSpecialSaveSector};
#[allow(unused_imports)]
use crate::types::*;
use crate::util::CalcByteArraySum;
#[allow(unused_imports)]
use core::ffi::c_void;
#[allow(unused_imports)]
use core::mem::zeroed;
#[allow(unused_imports)]
use core::ptr::null_mut;
/// `Free` with this module's view of its types.
#[inline]
unsafe fn Free(a0: *mut c_void) {
    unsafe {
        crate::malloc::Free(a0 as _);
    }
}
// Data tables (translate with cdata.py): sTrainerHillTrainerTemplates_JP

/// `struct SendRecvMgr`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SendRecvMgr {
    pub isParent: u8,
    pub state: u8,
    pub xferState: u8,
    pub checksumResult: u8,
    pub cancellationReason: u8,
    pub data: *mut u32,
    pub cursor: i32,
    pub size: i32,
    pub checksum: i32,
}

unsafe impl Sync for SendRecvMgr {}

#[cfg(target_arch = "arm")]
const _: () = {
    #[allow(unused_imports)]
    use core::mem::{offset_of, size_of};
    assert!(size_of::<SendRecvMgr>() == 24);
    assert!(offset_of!(SendRecvMgr, isParent) == 0);
    assert!(offset_of!(SendRecvMgr, state) == 1);
    assert!(offset_of!(SendRecvMgr, xferState) == 2);
    assert!(offset_of!(SendRecvMgr, checksumResult) == 3);
    assert!(offset_of!(SendRecvMgr, cancellationReason) == 4);
    assert!(offset_of!(SendRecvMgr, data) == 8);
    assert!(offset_of!(SendRecvMgr, cursor) == 12);
    assert!(offset_of!(SendRecvMgr, size) == 16);
    assert!(offset_of!(SendRecvMgr, checksum) == 20);
};

static sTrainerHillTrainerTemplates_JP: Table<CArray<TrainerHillTrainer, 4>> =
    Table((&raw const crate::data::ereader_helpers::sTrainerHillTrainerTemplates_JP).cast());

pub(crate) static mut sSendRecvMgr: SendRecvMgr = unsafe { zeroed() };
pub(crate) static sJoyNewOrRepeated: crate::global::Global<u16> = crate::global::Global::new(0);
pub(crate) static sJoyNew: crate::global::Global<u16> = crate::global::Global::new(0);
pub(crate) static sSendRecvStatus: crate::global::Global<u16> = crate::global::Global::new(0);
pub(crate) static sCounter1: crate::global::Global<u16> = crate::global::Global::new(0);
pub(crate) static sCounter2: crate::global::Global<u32> = crate::global::Global::new(0);
pub(crate) static mut sSavedIme: u16 = 0;
pub(crate) static sSavedIe: crate::global::Global<u16> = crate::global::Global::new(0);
pub(crate) static sSavedTm3Cnt: crate::global::Global<u16> = crate::global::Global::new(0);
pub(crate) static sSavedSioCnt: crate::global::Global<u16> = crate::global::Global::new(0);
pub(crate) static sSavedRCnt: crate::global::Global<u16> = crate::global::Global::new(0);

/// `AllocZeroed` with this module's view of its types.
#[inline]
unsafe fn AllocZeroed(a0: u32) -> *mut c_void {
    unsafe { crate::malloc::AllocZeroed(a0) as *mut c_void }
}
/// `CpuSet` with this module's view of its types.
#[inline]
unsafe fn CpuSet(a0: *mut c_void, a1: *mut c_void, a2: u32) {
    unsafe {
        crate::syscall::CpuSet(a0 as _, a1 as _, a2);
    }
}
/// `VBlankIntrWait` with this module's view of its types.
#[inline]
unsafe fn VBlankIntrWait() {
    unsafe {
        crate::syscall::VBlankIntrWait();
    }
}

unsafe fn GetTrainerHillUnkVal() -> u8 {
    (((*gSaveBlock1Ptr).trainerHill.unused as i32 + 1) % 256) as u8
}
unsafe fn ValidateTrainerChecksum(hillTrainer: *mut EReaderTrainerHillTrainer) -> u32 {
    let checksum: i32 = CalcByteArraySum(hillTrainer as *mut u8, 624) as i32;
    if checksum as u32 != (*hillTrainer).checksum {
        return FALSE as u32;
    }
    TRUE as u32
}
pub unsafe fn ValidateTrainerHillData(hillSet: *mut EReaderTrainerHillSet) -> u8 {
    let numTrainers: i32 = (*hillSet).numTrainers as i32;
    if !(1..=NUM_TRAINER_HILL_TRAINERS).contains(&numTrainers) {
        return FALSE;
    }
    let mut i: u32 = 0;
    while i < numTrainers as u32 {
        if ValidateTrainerChecksum(&raw mut (*hillSet).trainers[i]) == 0 {
            return FALSE;
        }
        i += 1;
    }
    let checksum: u32 = CalcByteArraySum(
        (*hillSet).trainers.as_mut_ptr() as *mut u8,
        numTrainers as u32 * 628,
    );
    if checksum != (*hillSet).checksum {
        return FALSE;
    }
    TRUE
}
unsafe fn ValidateTrainerHillChecksum(hillSet: *mut EReaderTrainerHillSet) -> u32 {
    let numTrainers: i32 = (*hillSet).numTrainers as i32;
    if !(1..=NUM_TRAINER_HILL_TRAINERS).contains(&numTrainers) {
        return FALSE as u32;
    }
    let checksum: u32 = CalcByteArraySum((*hillSet).trainers.as_mut_ptr() as *mut u8, 3808);
    if checksum != (*hillSet).checksum {
        return FALSE as u32;
    }
    TRUE as u32
}
unsafe fn TryWriteTrainerHill_Internal(
    hillSet: *mut EReaderTrainerHillSet,
    challenge: *mut TrainerHillChallenge,
) -> u32 {
    memset(challenge as *mut u8, 0, SECTOR_SIZE);
    (*challenge).numTrainers = (*hillSet).numTrainers;
    (*challenge).unused1 = GetTrainerHillUnkVal();
    (*challenge).numFloors = (((*hillSet).numTrainers as i32 + 1) / 2) as u8;
    let mut i: i32 = 0;
    while i < (*hillSet).numTrainers as i32 {
        if i & 1 == 0 {
            (*challenge).floors[i / 2].trainerNum1 = (*hillSet).trainers[i].trainerNum;
            (*challenge).floors[i / 2].map = (*hillSet).trainers[i].map;
            (*challenge).floors[i / 2].trainers[0] = (*hillSet).trainers[i].trainer;
        } else {
            (*challenge).floors[i / 2].trainerNum2 = (*hillSet).trainers[i].trainerNum;
            (*challenge).floors[i / 2].trainers[1] = (*hillSet).trainers[i].trainer;
        }
        i += 1;
    }
    if i & 1 != 0 {
        (*challenge).floors[i / 2].trainers[1] = sTrainerHillTrainerTemplates_JP[i / 2];
    }
    (*challenge).checksum = CalcByteArraySum((*challenge).floors.as_mut_ptr() as *mut u8, 3808);
    if TryWriteSpecialSaveSector(SECTOR_ID_TRAINER_HILL, challenge as *mut u8)
        != SAVE_STATUS_OK as u32
    {
        return FALSE as u32;
    }
    TRUE as u32
}
pub unsafe fn TryWriteTrainerHill(hillSet: *mut EReaderTrainerHillSet) -> u32 {
    let buffer: *mut c_void = AllocZeroed(SECTOR_SIZE);
    let result: u32 = TryWriteTrainerHill_Internal(hillSet, buffer as *mut TrainerHillChallenge);
    Free(buffer);
    result
}
unsafe fn TryReadTrainerHill_Internal(dest: *mut EReaderTrainerHillSet, buffer: *mut u8) -> u32 {
    if TryReadSpecialSaveSector(SECTOR_ID_TRAINER_HILL, buffer) != SAVE_STATUS_OK as u32 {
        return FALSE as u32;
    }
    memcpy(dest as *mut u8, buffer, 3816);
    if ValidateTrainerHillChecksum(dest) == 0 {
        return FALSE as u32;
    }
    TRUE as u32
}
unsafe fn TryReadTrainerHill(hillSet: *mut EReaderTrainerHillSet) -> u32 {
    let buffer: *mut u8 = AllocZeroed(SECTOR_SIZE) as *mut u8;
    let result: u32 = TryReadTrainerHill_Internal(hillSet, buffer);
    Free(buffer as *mut c_void);
    result
}
pub unsafe fn ReadTrainerHillAndValidate() -> u32 {
    let hillSet: *mut EReaderTrainerHillSet =
        AllocZeroed(SECTOR_SIZE) as *mut EReaderTrainerHillSet;
    let result: u32 = TryReadTrainerHill(hillSet);
    Free(hillSet as *mut c_void);
    result
}
pub unsafe fn EReader_Send(size: i32, src: *mut c_void) -> i32 {
    let mut result: i32 = 0;
    let mut sendStatus: i32 = 0;
    EReaderHelper_SaveRegsState();
    loop {
        GetKeyInput();
        if sJoyNew.get() as i32 & B_BUTTON != 0 {
            gShouldAdvanceLinkState = 2;
        }
        sendStatus = EReaderHandleTransfer(1, size as u32, src, null_mut());
        sSendRecvStatus.set(sendStatus as u16);
        if sSendRecvStatus.get() as i32 & EREADER_XFER_MASK == 0
            && sSendRecvStatus.get() as i32 & EREADER_CHECKSUM_OK_MASK != 0
        {
            result = 0;
            break;
        } else if sSendRecvStatus.get() as i32 & EREADER_CANCEL_KEY_MASK != 0 {
            result = 1;
            break;
        } else if sSendRecvStatus.get() as i32 & EREADER_CANCEL_TIMEOUT_MASK != 0 {
            result = 2;
            break;
        } else {
            gShouldAdvanceLinkState = 0;
            VBlankIntrWait();
        }
    }
    {
        {
            let mut tmp: u32 = 0;
            volatile_write(&raw mut tmp, 0);
            CpuSet(
                &raw mut tmp as *mut c_void,
                &raw mut sSendRecvMgr as *mut c_void,
                0x5000006,
            );
        }
    }
    EReaderHelper_RestoreRegsState();
    result
}
pub unsafe fn EReader_Recv(dest: *mut c_void) -> i32 {
    let mut result: i32 = 0;
    let mut recvStatus: i32 = 0;
    EReaderHelper_SaveRegsState();
    loop {
        GetKeyInput();
        if sJoyNew.get() as i32 & B_BUTTON != 0 {
            gShouldAdvanceLinkState = 2;
        }
        recvStatus = EReaderHandleTransfer(0, 0, null_mut(), dest);
        sSendRecvStatus.set(recvStatus as u16);
        if sSendRecvStatus.get() as i32 & EREADER_XFER_MASK == 0
            && sSendRecvStatus.get() as i32 & EREADER_CHECKSUM_OK_MASK != 0
        {
            result = 0;
            break;
        } else if sSendRecvStatus.get() as i32 & EREADER_CANCEL_KEY_MASK != 0 {
            result = 1;
            break;
        } else if sSendRecvStatus.get() as i32 & EREADER_CANCEL_TIMEOUT_MASK != 0 {
            result = 2;
            break;
        } else {
            gShouldAdvanceLinkState = 0;
            VBlankIntrWait();
        }
    }
    {
        {
            let mut tmp: u32 = 0;
            volatile_write(&raw mut tmp, 0);
            CpuSet(
                &raw mut tmp as *mut c_void,
                &raw mut sSendRecvMgr as *mut c_void,
                0x5000006,
            );
        }
    }
    EReaderHelper_RestoreRegsState();
    result
}
unsafe fn CloseSerial() {
    volatile_write(67109384_usize as *mut u16, 0);
    volatile_write(
        0x4000200_usize as *mut u16,
        (0x4000200_usize as *mut u16).read_volatile() & 65343,
    );
    volatile_write(67109384_usize as *mut u16, 1);
    volatile_write(67109160_usize as *mut u16, 0);
    volatile_write(67109134_usize as *mut u16, 0);
    volatile_write(67109378_usize as *mut u16, 192);
}
unsafe fn OpenSerialMulti() {
    volatile_write(67109384_usize as *mut u16, 0);
    volatile_write(
        0x4000200_usize as *mut u16,
        (0x4000200_usize as *mut u16).read_volatile() & 65343,
    );
    volatile_write(67109384_usize as *mut u16, 1);
    volatile_write(67109172_usize as *mut u16, 0);
    volatile_write(67109160_usize as *mut u16, SIO_MULTI_MODE);
    volatile_write(
        67109160_usize as *mut u16,
        (67109160_usize as *mut u16).read_volatile() | 16387,
    );
    volatile_write(67109384_usize as *mut u16, 0);
    volatile_write(
        0x4000200_usize as *mut u16,
        (0x4000200_usize as *mut u16).read_volatile() | INTR_FLAG_SERIAL,
    );
    volatile_write(67109384_usize as *mut u16, 1);
    if sSendRecvMgr.state == 0 {
        {
            {
                let mut tmp: u32 = 0;
                volatile_write(&raw mut tmp, 0);
                CpuSet(
                    &raw mut tmp as *mut c_void,
                    &raw mut sSendRecvMgr as *mut c_void,
                    0x5000006,
                );
            }
        }
    }
}
unsafe fn OpenSerial32() {
    volatile_write(67109172_usize as *mut u16, 0);
    volatile_write(67109160_usize as *mut u16, 20480);
    volatile_write(
        67109160_usize as *mut u16,
        (67109160_usize as *mut u16).read_volatile() | SIO_MULTI_SD as u16,
    );
    gShouldAdvanceLinkState = 0;
    sCounter1.set(0);
    sCounter2.set(0);
}
pub unsafe fn EReaderHandleTransfer(
    mode: u8,
    size: u32,
    data: *mut c_void,
    recvBuffer: *mut c_void,
) -> i32 {
    'l1: {
        let sw1: u8 = sSendRecvMgr.state;
        let mut fall = false;
        if sw1 == EREADER_XFR_STATE_INIT {
            OpenSerialMulti();
            sSendRecvMgr.xferState = EREADER_XFER_EXE;
            sSendRecvMgr.state = EREADER_XFR_STATE_HANDSHAKE;
            break 'l1;
        }
        if sw1 == EREADER_XFR_STATE_HANDSHAKE {
            if DetermineSendRecvState(mode) != 0 {
                EnableSio();
            }
            if gShouldAdvanceLinkState == 2 {
                sSendRecvMgr.cancellationReason = EREADER_CANCEL_KEY;
                sSendRecvMgr.state = EREADER_XFR_STATE_DONE;
            }
            break 'l1;
        }
        if sw1 == EREADER_XFR_STATE_START {
            fall = true;
            OpenSerial32();
            SetUpTransferManager(size, data, recvBuffer);
            sSendRecvMgr.state = EREADER_XFR_STATE_TRANSFER;
        }
        if fall || sw1 == EREADER_XFR_STATE_TRANSFER {
            if gShouldAdvanceLinkState == 2 {
                sSendRecvMgr.cancellationReason = EREADER_CANCEL_KEY;
                sSendRecvMgr.state = EREADER_XFR_STATE_DONE;
            } else {
                sCounter1.set(sCounter1.get() + 1);
                sCounter2.set(sCounter2.get() + 1);
                if sSendRecvMgr.isParent == 0 && sCounter2.get() > 60 {
                    sSendRecvMgr.cancellationReason = EREADER_CANCEL_TIMEOUT;
                    sSendRecvMgr.state = EREADER_XFR_STATE_DONE;
                }
                if sSendRecvMgr.xferState != EREADER_XFER_CHK {
                    if sSendRecvMgr.isParent != 0 && sCounter1.get() > 2 {
                        EnableSio();
                        sSendRecvMgr.xferState = EREADER_XFER_CHK;
                    } else {
                        EnableSio();
                        sSendRecvMgr.xferState = EREADER_XFER_CHK;
                    }
                }
            }
            break 'l1;
        }
        if sw1 == EREADER_XFR_STATE_TRANSFER_DONE {
            OpenSerialMulti();
            sSendRecvMgr.state = EREADER_XFR_STATE_CHECKSUM;
            break 'l1;
        }
        if sw1 == EREADER_XFR_STATE_CHECKSUM {
            if sSendRecvMgr.isParent == TRUE && sCounter1.get() > 2 {
                EnableSio();
            }
            if ({
                sCounter1.set(sCounter1.get() + 1);
                sCounter1.get()
            }) > 60
            {
                sSendRecvMgr.cancellationReason = EREADER_CANCEL_TIMEOUT;
                sSendRecvMgr.state = EREADER_XFR_STATE_DONE;
            }
            break 'l1;
        }
        if sw1 == EREADER_XFR_STATE_DONE {
            if sSendRecvMgr.xferState != 0 {
                CloseSerial();
                sSendRecvMgr.xferState = 0;
            }
            break 'l1;
        }
    }
    (sSendRecvMgr.xferState as i32)
        | (sSendRecvMgr.cancellationReason as i32) << 2
        | (sSendRecvMgr.checksumResult as i32) << 4
}
unsafe fn DetermineSendRecvState(mode: u8) -> u16 {
    let mut resp: u16 = 0;
    if (REG_ADDR_SIOCNT as usize as *mut u32).read_volatile() & 12 == SIO_MULTI_SD as u32
        && mode != 0
    {
        resp = ({
            sSendRecvMgr.isParent = TRUE;
            sSendRecvMgr.isParent
        }) as u16;
    } else {
        resp = ({
            sSendRecvMgr.isParent = FALSE;
            sSendRecvMgr.isParent
        }) as u16;
    }
    resp
}
unsafe fn SetUpTransferManager(size: u32, data: *mut c_void, recvBuffer: *mut c_void) {
    if sSendRecvMgr.isParent != 0 {
        volatile_write(
            67109160_usize as *mut u16,
            (67109160_usize as *mut u16).read_volatile() | SIO_38400_BPS,
        );
        sSendRecvMgr.data = data as *mut u32;
        volatile_write(67109152_usize as *mut u32, size);
        sSendRecvMgr.size = (size / 4) as i32 + 1;
        StartTm3();
    } else {
        volatile_write(
            67109160_usize as *mut u16,
            (67109160_usize as *mut u16).read_volatile(),
        );
        sSendRecvMgr.data = recvBuffer as *mut u32;
    }
}
unsafe fn StartTm3() {
    volatile_write(67109132_usize as *mut u16, 64935);
    volatile_write(67109134_usize as *mut u16, TIMER_INTR_ENABLE);
    volatile_write(67109384_usize as *mut u16, 0);
    volatile_write(
        0x4000200_usize as *mut u16,
        (0x4000200_usize as *mut u16).read_volatile() | INTR_FLAG_TIMER3,
    );
    volatile_write(67109384_usize as *mut u16, 1);
}
pub unsafe extern "C" fn EReaderHelper_Timer3Callback() {
    DisableTm3();
    EnableSio();
}
pub unsafe extern "C" fn EReaderHelper_SerialCallback() {
    let mut cnt1: u16 = 0;
    let mut cnt2: u16 = 0;
    let mut recv32: u32 = 0;
    let mut recv: CArray<u16, 4> = zeroed();
    match sSendRecvMgr.state {
        EREADER_XFR_STATE_HANDSHAKE => {
            volatile_write(67109162_usize as *mut u16, EREADER_HANDSHAKE);
            *(recv.as_mut_ptr() as *mut u64) = (67109152_usize as *mut u64).read_volatile();
            cnt1 = 0;
            cnt2 = 0;
            for i in 0..4u16 {
                if recv[i] == EREADER_HANDSHAKE {
                    cnt1 += 1;
                } else if recv[i] != 0xFFFF {
                    cnt2 += 1;
                }
            }
            if cnt1 == 2 && cnt2 == 0 {
                sSendRecvMgr.state = 2;
            }
        }
        EREADER_XFR_STATE_TRANSFER => {
            recv32 = (67109152_usize as *mut u32).read_volatile();
            if sSendRecvMgr.cursor == 0 && sSendRecvMgr.isParent == 0 {
                sSendRecvMgr.size = (recv32 / 4) as i32 + 1;
            }
            if sSendRecvMgr.isParent == TRUE {
                if sSendRecvMgr.cursor < sSendRecvMgr.size {
                    volatile_write(
                        67109152_usize as *mut u32,
                        *sSendRecvMgr.data.at(sSendRecvMgr.cursor),
                    );
                    sSendRecvMgr.checksum += *sSendRecvMgr.data.at(sSendRecvMgr.cursor) as i32;
                } else {
                    volatile_write(67109152_usize as *mut u32, sSendRecvMgr.checksum as u32);
                }
            } else {
                if sSendRecvMgr.cursor > 0 && sSendRecvMgr.cursor < sSendRecvMgr.size + 1 {
                    *sSendRecvMgr.data.at(sSendRecvMgr.cursor - 1) = recv32;
                    sSendRecvMgr.checksum += recv32 as i32;
                } else if sSendRecvMgr.cursor != 0 {
                    if sSendRecvMgr.checksum as u32 == recv32 {
                        sSendRecvMgr.checksumResult = EREADER_CHECKSUM_OK;
                    } else {
                        sSendRecvMgr.checksumResult = EREADER_CHECKSUM_ERR;
                    }
                }
                sCounter2.set(0);
            }
            if ({
                sSendRecvMgr.cursor += 1;
                sSendRecvMgr.cursor
            }) < sSendRecvMgr.size + 2
            {
                if sSendRecvMgr.isParent != 0 {
                    volatile_write(
                        67109134_usize as *mut u16,
                        (67109134_usize as *mut u16).read_volatile() | TIMER_ENABLE,
                    );
                } else {
                    EnableSio();
                }
            } else {
                sSendRecvMgr.state = EREADER_XFR_STATE_TRANSFER_DONE;
                sCounter1.set(0);
            }
        }
        EREADER_XFR_STATE_CHECKSUM => {
            if sSendRecvMgr.isParent == 0 {
                volatile_write(
                    67109162_usize as *mut u16,
                    sSendRecvMgr.checksumResult as u16,
                );
            }
            volatile_write(
                recv.as_mut_ptr() as *mut u64,
                (67109152_usize as *mut u64).read_volatile(),
            );
            if recv[1] == 1 || recv[1] == EREADER_CHECKSUM_ERR as u16 {
                if sSendRecvMgr.isParent == TRUE {
                    sSendRecvMgr.checksumResult = recv[1] as u8;
                }
                sSendRecvMgr.state = EREADER_XFR_STATE_DONE;
            }
        }
        _ => {}
    }
}
unsafe fn EnableSio() {
    volatile_write(
        67109160_usize as *mut u16,
        (67109160_usize as *mut u16).read_volatile() | SIO_ENABLE,
    );
}
unsafe fn DisableTm3() {
    volatile_write(
        67109134_usize as *mut u16,
        (67109134_usize as *mut u16).read_volatile() & 65407,
    );
    volatile_write(67109132_usize as *mut u16, 0xFDA7);
}
unsafe fn GetKeyInput() {
    let rawKeys: i32 = (67109168_usize as *mut u16).read_volatile() as i32 ^ KEYS_MASK;
    sJoyNew.set(rawKeys as u16 & !sJoyNewOrRepeated.get());
    sJoyNewOrRepeated.set(rawKeys as u16);
}
pub unsafe fn EReaderHelper_SaveRegsState() {
    sSavedIme = (67109384_usize as *mut u16).read_volatile();
    sSavedIe.set((0x4000200_usize as *mut u16).read_volatile());
    sSavedTm3Cnt.set((67109134_usize as *mut u16).read_volatile());
    sSavedSioCnt.set((67109160_usize as *mut u16).read_volatile());
    sSavedRCnt.set((67109172_usize as *mut u16).read_volatile());
}
pub unsafe fn EReaderHelper_RestoreRegsState() {
    volatile_write(67109384_usize as *mut u16, sSavedIme);
    volatile_write(0x4000200_usize as *mut u16, sSavedIe.get());
    volatile_write(67109134_usize as *mut u16, sSavedTm3Cnt.get());
    volatile_write(67109160_usize as *mut u16, sSavedSioCnt.get());
    volatile_write(67109172_usize as *mut u16, sSavedRCnt.get());
}
pub unsafe fn EReaderHelper_ClearSendRecvMgr() {
    {
        {
            let mut tmp: u32 = 0;
            volatile_write(&raw mut tmp, 0);
            CpuSet(
                &raw mut tmp as *mut c_void,
                &raw mut sSendRecvMgr as *mut c_void,
                0x5000006,
            );
        }
    }
}
