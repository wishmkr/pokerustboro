//! Translated from `src/librfu_rfu.c` by tools/rustport/c2rs.py.
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
    dead_code,
    unused_assignments,
    unused_variables
)]

#[allow(unused_imports)]
use crate::c::*;
#[allow(unused_imports)]
use crate::consts::*;
use crate::librfu_sio32id::AgbRFU_checkID;
use crate::librfu_stwi::{
    AgbRFU_SoftReset, STWI_init_timer, STWI_poll_CommandEnd, STWI_read_status, STWI_send_CP_EndREQ,
    STWI_send_CP_PollingREQ, STWI_send_CP_StartREQ, STWI_send_CPR_EndREQ, STWI_send_CPR_PollingREQ,
    STWI_send_CPR_StartREQ, STWI_send_DataRxREQ, STWI_send_DataTxAndChangeREQ, STWI_send_DataTxREQ,
    STWI_send_DisconnectREQ, STWI_send_GameConfigREQ, STWI_send_LinkStatusREQ,
    STWI_send_MS_ChangeREQ, STWI_send_ResetREQ, STWI_send_ResumeRetransmitAndChangeREQ,
    STWI_send_SC_EndREQ, STWI_send_SC_PollingREQ, STWI_send_SC_StartREQ, STWI_send_SP_EndREQ,
    STWI_send_SP_PollingREQ, STWI_send_SP_StartREQ, STWI_send_SlotStatusREQ, STWI_send_StopModeREQ,
    STWI_send_SystemConfigREQ, STWI_send_SystemStatusREQ, STWI_send_TestModeREQ,
    STWI_set_Callback_M, STWI_set_Callback_S, gSTWIStatus,
};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use core::ffi::c_void;
#[allow(unused_imports)]
use core::mem::zeroed;
#[allow(unused_imports)]
use core::ptr::null_mut;
/// `STWI_init_all` with this module's view of its types.
#[inline]
unsafe fn STWI_init_all(
    a0: *mut RfuIntrStruct,
    a1: *mut Option<crate::agb_main::IntrFunc>,
    a2: u8,
) {
    unsafe {
        crate::librfu_stwi::STWI_init_all(a0 as _, a1 as _, a2);
    }
}
// Data tables (translate with cdata.py): llsf_struct version_string str_checkMbootLL

/// `struct LLSFStruct`
#[repr(C, align(4))]
#[derive(Clone, Copy)]
pub struct LLSFStruct {
    pub frameSize: u8,
    pub recvFirstShift: u8,
    pub connSlotFlagShift: u8,
    pub slotStateShift: u8,
    pub ackShift: u8,
    pub phaseShift: u8,
    pub nShift: u8,
    pub recvFirstMask: u8,
    pub connSlotFlagMask: u8,
    pub slotStateMask: u8,
    pub ackMask: u8,
    pub phaseMask: u8,
    pub nMask: u8,
    pub framesMask: u16,
}

unsafe impl Sync for LLSFStruct {}

/// `struct RfuLocalStruct`
#[repr(C, align(4))]
#[derive(Clone, Copy)]
pub struct RfuLocalStruct {
    pub recvFirst: u8,
    pub connSlotFlag: u8,
    pub slotState: u8,
    pub ack: u8,
    pub phase: u8,
    pub n: u8,
    pub frame: u16,
}

unsafe impl Sync for RfuLocalStruct {}

#[cfg(target_arch = "arm")]
const _: () = {
    #[allow(unused_imports)]
    use core::mem::{offset_of, size_of};
    assert!(size_of::<LLSFStruct>() == 16);
    assert!(offset_of!(LLSFStruct, frameSize) == 0);
    assert!(offset_of!(LLSFStruct, recvFirstShift) == 1);
    assert!(offset_of!(LLSFStruct, connSlotFlagShift) == 2);
    assert!(offset_of!(LLSFStruct, slotStateShift) == 3);
    assert!(offset_of!(LLSFStruct, ackShift) == 4);
    assert!(offset_of!(LLSFStruct, phaseShift) == 5);
    assert!(offset_of!(LLSFStruct, nShift) == 6);
    assert!(offset_of!(LLSFStruct, recvFirstMask) == 7);
    assert!(offset_of!(LLSFStruct, connSlotFlagMask) == 8);
    assert!(offset_of!(LLSFStruct, slotStateMask) == 9);
    assert!(offset_of!(LLSFStruct, ackMask) == 10);
    assert!(offset_of!(LLSFStruct, phaseMask) == 11);
    assert!(offset_of!(LLSFStruct, nMask) == 12);
    assert!(offset_of!(LLSFStruct, framesMask) == 14);
    assert!(size_of::<RfuLocalStruct>() == 8);
    assert!(offset_of!(RfuLocalStruct, recvFirst) == 0);
    assert!(offset_of!(RfuLocalStruct, connSlotFlag) == 1);
    assert!(offset_of!(RfuLocalStruct, slotState) == 2);
    assert!(offset_of!(RfuLocalStruct, ack) == 3);
    assert!(offset_of!(RfuLocalStruct, phase) == 4);
    assert!(offset_of!(RfuLocalStruct, n) == 5);
    assert!(offset_of!(RfuLocalStruct, frame) == 6);
};

static llsf_struct: Table<CArray<LLSFStruct, 2>> =
    Table((&raw const crate::data::librfu_rfu::llsf_struct).cast());
static str_checkMbootLL: Table<CArray<u8, 10>> =
    Table((&raw const crate::data::librfu_rfu::str_checkMbootLL).cast());

#[unsafe(link_section = "common_data")]
pub static mut gRfuSlotStatusUNI: CArray<*mut RfuSlotStatusUNI, 4> = unsafe { zeroed() };
#[unsafe(link_section = "common_data")]
pub static mut gRfuSlotStatusNI: CArray<*mut RfuSlotStatusNI, 4> = unsafe { zeroed() };
#[unsafe(link_section = "common_data")]
pub static mut gRfuLinkStatus: *mut RfuLinkStatus = null_mut();
#[unsafe(link_section = "common_data")]
pub static mut gRfuStatic: *mut RfuStatic = null_mut();
#[unsafe(link_section = "common_data")]
pub static mut gRfuFixed: *mut RfuFixed = null_mut();

/// `CpuSet` with this module's view of its types.
#[inline]
unsafe fn CpuSet(a0: *mut c_void, a1: *mut c_void, a2: u32) {
    unsafe {
        crate::syscall::CpuSet(a0 as _, a1 as _, a2);
    }
}
/// `Div` with this module's view of its types.
#[inline]
unsafe fn Div(a0: i32, a1: i32) -> i32 {
    unsafe { crate::syscall::Div(a0, a1) }
}

pub unsafe fn rfu_initializeAPI(
    APIBuffer: *mut u32,
    buffByteSize: u16,
    sioIntrTable_p: *mut Option<crate::agb_main::IntrFunc>,
    copyInterruptToRam: u8,
) -> u16 {
    let mut buffByteSizeMax: u16 = 0;
    if APIBuffer as usize as u32 & 0xF000000 == EWRAM_START && copyInterruptToRam != 0 {
        return ERR_RFU_API_BUFF_ADR;
    }
    if APIBuffer as usize as u32 & 3 != 0 {
        return ERR_RFU_API_BUFF_ADR;
    }
    if copyInterruptToRam != 0 {
        buffByteSizeMax = RFU_API_BUFF_SIZE_RAM;
        if buffByteSize < buffByteSizeMax {
            return ERR_RFU_API_BUFF_SIZE;
        }
    }
    if copyInterruptToRam == 0 {
        buffByteSizeMax = RFU_API_BUFF_SIZE_ROM;
        if buffByteSize < buffByteSizeMax {
            return ERR_RFU_API_BUFF_SIZE;
        }
    }
    gRfuLinkStatus = APIBuffer as *mut c_void as *mut RfuLinkStatus;
    gRfuStatic = (APIBuffer as *mut c_void as *mut u8).at(180) as *mut c_void as *mut RfuStatic;
    gRfuFixed = (APIBuffer as *mut c_void as *mut u8).at(220) as *mut c_void as *mut RfuFixed;
    gRfuSlotStatusNI[0] =
        (APIBuffer as *mut c_void as *mut u8).at(444) as *mut c_void as *mut RfuSlotStatusNI;
    gRfuSlotStatusUNI[0] =
        (APIBuffer as *mut c_void as *mut u8).at(892) as *mut c_void as *mut RfuSlotStatusUNI;
    let mut i: u16 = 1;
    while i < RFU_CHILD_MAX as u16 {
        gRfuSlotStatusNI[i] = gRfuSlotStatusNI[i as i32 - 1].at(1);
        gRfuSlotStatusUNI[i] = gRfuSlotStatusUNI[i as i32 - 1].at(1);
        i += 1;
    }
    (*gRfuFixed).STWIBuffer = gRfuSlotStatusUNI[3].at(1) as *mut RfuIntrStruct;
    STWI_init_all(
        gRfuSlotStatusUNI[3].at(1) as *mut RfuIntrStruct,
        sioIntrTable_p,
        copyInterruptToRam,
    );
    rfu_STC_clearAPIVariables();
    for i in 0..(RFU_CHILD_MAX as u16) {
        (*gRfuSlotStatusNI[i]).recvBuffer = null_mut();
        (*gRfuSlotStatusNI[i]).recvBufferSize = 0;
        (*gRfuSlotStatusUNI[i]).recvBuffer = null_mut();
        (*gRfuSlotStatusUNI[i]).recvBufferSize = 0;
    }
    {
        let mut _src: *mut u16 = (core::mem::transmute::<_, usize>(Some(
            rfu_STC_fastCopy as unsafe fn(*mut *mut u8, *mut *mut u8, i32),
        )) as u32
            & 0xfffffffe) as usize as *mut u16;
        let mut _dst: *mut u16 = (*gRfuFixed).fastCopyBuffer.as_mut_ptr();
        buffByteSizeMax = 48;
        while ({
            let t1 = buffByteSizeMax;
            buffByteSizeMax -= 1;
            t1
        }) != 0
        {
            *({
                let t2 = _dst;
                _dst = _dst.at(1);
                t2
            }) = *({
                let t4 = _src;
                _src = _src.at(1);
                t4
            });
        }
    }
    (*gRfuFixed).fastCopyPtr =
        core::mem::transmute::<_, Option<unsafe fn(*mut *mut u8, *mut *mut u8, i32)>>(
            ((*gRfuFixed).fastCopyBuffer.as_mut_ptr() as *mut c_void as *mut u8).at(1)
                as *mut c_void,
        );
    0
}
unsafe fn rfu_STC_clearAPIVariables() {
    let IMEBackup: u16 = (67109384_usize as *mut u16).read_volatile();
    let mut flags: u8 = 0;
    volatile_write(67109384_usize as *mut u16, 0);
    flags = (*gRfuStatic).flags;
    {
        {
            let mut tmp: u16 = 0;
            volatile_write(&raw mut tmp, 0);
            CpuSet(
                &raw mut tmp as *mut c_void,
                gRfuStatic as *mut c_void,
                0x1000014,
            );
        }
    }
    (*gRfuStatic).flags = flags & 8;
    {
        {
            let mut tmp: u16 = 0;
            volatile_write(&raw mut tmp, 0);
            CpuSet(
                &raw mut tmp as *mut c_void,
                gRfuLinkStatus as *mut c_void,
                0x100005a,
            );
        }
    }
    (*gRfuLinkStatus).watchInterval = 4;
    (*gRfuStatic).nowWatchInterval = 0;
    (*gRfuLinkStatus).parentChild = MODE_NEUTRAL;
    rfu_clearAllSlot();
    (*gRfuStatic).SCStartFlag = 0;
    for i in 0..RFU_CHILD_MAX {
        (*gRfuStatic).cidBak[i] = 0;
    }
    volatile_write(67109384_usize as *mut u16, IMEBackup);
}
pub unsafe fn rfu_REQ_PARENT_resumeRetransmitAndChange() {
    STWI_set_Callback_M(core::mem::transmute::<
        Option<unsafe fn(u8, u16)>,
        *mut c_void,
    >(Some(rfu_STC_REQ_callback)));
    STWI_send_ResumeRetransmitAndChangeREQ();
}
pub unsafe fn rfu_UNI_PARENT_getDRAC_ACK(ackFlag: *mut u8) -> u16 {
    *ackFlag = 0;
    if (*gRfuLinkStatus).parentChild != MODE_PARENT {
        return ERR_MODE_NOT_PARENT;
    }
    let buf: *mut u8 = rfu_getSTWIRecvBuffer();
    match *buf {
        40 | 54 => {
            if *buf.at(1) == 0 {
                *ackFlag = (*gRfuLinkStatus).connSlotFlag;
            } else {
                *ackFlag = *buf.at(4);
            }
            return 0;
        }
        _ => {
            return ERR_REQ_CMD_ID;
        }
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
pub unsafe fn rfu_setTimerInterrupt(
    timerNo: u8,
    timerIntrTable_p: *mut Option<crate::agb_main::IntrFunc>,
) {
    STWI_init_timer(timerIntrTable_p, timerNo as i32);
}
pub unsafe fn rfu_getSTWIRecvBuffer() -> *mut u8 {
    (*gRfuFixed).STWIBuffer as *mut u8
}
pub unsafe fn rfu_setMSCCallback(callback: Option<unsafe fn(u16)>) {
    STWI_set_Callback_S(callback);
}
pub unsafe fn rfu_setREQCallback(callback: Option<unsafe fn(u16, u16)>) {
    (*gRfuFixed).reqCallback = callback;
    rfu_enableREQCallback(callback.is_some() as u8);
}
unsafe fn rfu_enableREQCallback(enable: u8) {
    if enable != 0 {
        (*gRfuStatic).flags |= 8;
    } else {
        (*gRfuStatic).flags &= 0xF7;
    }
}
pub(crate) unsafe fn rfu_STC_REQ_callback(reqCommand: u8, reqResult: u16) {
    STWI_set_Callback_M(core::mem::transmute::<
        Option<unsafe fn(u8, u16)>,
        *mut c_void,
    >(Some(rfu_CB_defaultCallback)));
    (*gRfuStatic).reqResult = reqResult;
    if (*gRfuStatic).flags as i32 & 8 != 0 {
        (*gRfuFixed).reqCallback.unwrap_unchecked()(reqCommand as u16, reqResult);
    }
}
pub(crate) unsafe fn rfu_CB_defaultCallback(reqCommand: u8, reqResult: u16) {
    let mut bmSlotFlags: i32 = 0;
    if reqCommand == ID_CLOCK_SLAVE_MS_CHANGE_ERROR_BY_DMA_REQ {
        if (*gRfuStatic).flags as i32 & 8 != 0 {
            (*gRfuFixed).reqCallback.unwrap_unchecked()(reqCommand as u16, reqResult);
        }
        bmSlotFlags =
            (*gRfuLinkStatus).connSlotFlag as i32 | (*gRfuLinkStatus).linkLossSlotFlag as i32;
        for i in 0..RFU_CHILD_MAX {
            if shr_i32(bmSlotFlags, i as u32) & 1 != 0 {
                rfu_STC_removeLinkData(i, 1);
            }
        }
        (*gRfuLinkStatus).parentChild = MODE_NEUTRAL;
    }
}
#[unsafe(no_mangle)]
pub unsafe fn rfu_waitREQComplete() -> u16 {
    STWI_poll_CommandEnd();
    (*gRfuStatic).reqResult
}
pub unsafe fn rfu_REQ_RFUStatus() {
    STWI_set_Callback_M(core::mem::transmute::<
        Option<unsafe fn(u8, u16)>,
        *mut c_void,
    >(Some(rfu_STC_REQ_callback)));
    STWI_send_SystemStatusREQ();
}
pub unsafe fn rfu_getRFUStatus(rfuState: *mut u8) -> u16 {
    if (*(*gRfuFixed).STWIBuffer).rxPacketAlloc.rfuPacket8.data[0] != 0x93 {
        return ERR_REQ_CMD_ID;
    }
    if STWI_poll_CommandEnd() == 0 {
        *rfuState = (*(*gRfuFixed).STWIBuffer).rxPacketAlloc.rfuPacket8.data[7];
    } else {
        *rfuState = 0xFF;
    }
    0
}
pub unsafe fn rfu_MBOOT_CHILD_inheritanceLinkStatus() -> u16 {
    let mut s1: *mut u8 = str_checkMbootLL.as_ptr().cast_mut();
    let mut s2: *mut u8 = 50331888_usize as *mut u8;
    while *s1 != 0 {
        if *({
            let t2 = s1;
            s1 = s1.at(1);
            t2
        }) != *({
            let t4 = s2;
            s2 = s2.at(1);
            t4
        }) {
            return 1;
        }
    }
    let mut mb_buff_iwram_p: *mut u16 = IWRAM_START as usize as *mut u16;
    let mut checksum: u16 = 0;
    for i in 0..90u8 {
        checksum += *({
            let t6 = mb_buff_iwram_p;
            mb_buff_iwram_p = mb_buff_iwram_p.at(1);
            t6
        });
    }
    if checksum != *(50331898_usize as *mut u16) {
        return 1;
    }
    CpuSet(
        IWRAM_START as usize as *mut u16 as *mut c_void,
        gRfuLinkStatus as *mut c_void,
        90,
    );
    (*gRfuStatic).flags |= 0x80;
    0
}
#[unsafe(no_mangle)]
pub unsafe fn rfu_REQ_stopMode() {
    let mut timerReg: *mut u32 = null_mut();
    if (67109384_usize as *mut u16).read_volatile() == 0 {
        rfu_STC_REQ_callback(ID_STOP_MODE_REQ as u8, 6);
        volatile_write(&raw mut (*gSTWIStatus).error, ERR_REQ_CMD_IME_DISABLE);
    } else {
        AgbRFU_SoftReset();
        rfu_STC_clearAPIVariables();
        if AgbRFU_checkID(8) == RFU_ID as i32 {
            timerReg = (0x4000100 + (*gSTWIStatus).timerSelect as i32 * 4) as usize as *mut u32;
            volatile_write(timerReg, 0);
            volatile_write(timerReg, 0x830000);
            while (timerReg).read_volatile() << 16 < 0x1060000 {}
            volatile_write(timerReg, 0);
            STWI_set_Callback_M(core::mem::transmute::<
                Option<unsafe fn(u8, u16)>,
                *mut c_void,
            >(Some(rfu_CB_stopMode)));
            STWI_send_StopModeREQ();
        } else {
            volatile_write(67109160_usize as *mut u16, SIO_MULTI_MODE);
            rfu_STC_REQ_callback(ID_STOP_MODE_REQ as u8, 0);
        }
    }
}
pub(crate) unsafe fn rfu_CB_stopMode(reqCommand: u8, reqResult: u16) {
    if reqResult == 0 {
        volatile_write(67109160_usize as *mut u16, SIO_MULTI_MODE);
    }
    rfu_STC_REQ_callback(reqCommand, reqResult);
}
pub unsafe fn rfu_REQBN_softReset_and_checkID() -> u32 {
    let mut id: u32 = 0;
    if (67109384_usize as *mut u16).read_volatile() == 0 {
        return ERR_ID_CHECK_IME_DISABLE;
    }
    AgbRFU_SoftReset();
    rfu_STC_clearAPIVariables();
    if ({
        id = AgbRFU_checkID(30) as u32;
        id
    }) == 0
    {
        volatile_write(67109160_usize as *mut u16, SIO_MULTI_MODE);
    }
    id
}
pub unsafe fn rfu_REQ_reset() {
    STWI_set_Callback_M(core::mem::transmute::<
        Option<unsafe fn(u8, u16)>,
        *mut c_void,
    >(Some(rfu_CB_reset)));
    STWI_send_ResetREQ();
}
pub(crate) unsafe fn rfu_CB_reset(reqCommand: u8, reqResult: u16) {
    if reqResult == 0 {
        rfu_STC_clearAPIVariables();
    }
    rfu_STC_REQ_callback(reqCommand, reqResult);
}
pub unsafe fn rfu_REQ_configSystem(availSlotFlag: u16, maxMFrame: u8, mcTimer: u8) {
    STWI_set_Callback_M(core::mem::transmute::<
        Option<unsafe fn(u8, u16)>,
        *mut c_void,
    >(Some(rfu_STC_REQ_callback)));
    STWI_send_SystemConfigREQ(availSlotFlag & AVAIL_SLOT1 | 0x3C, maxMFrame, mcTimer);
    if mcTimer == 0 {
        (*gRfuStatic).linkEmergencyLimit = 1;
    } else {
        let IMEBackup: u16 = (67109384_usize as *mut u16).read_volatile();
        volatile_write(67109384_usize as *mut u16, 0);
        (*gRfuStatic).linkEmergencyLimit = Div(600, mcTimer as i32) as u16;
        volatile_write(67109384_usize as *mut u16, IMEBackup);
    }
}
pub unsafe fn rfu_REQ_configGameData(
    mbootFlag: u8,
    serialNo: u16,
    mut gname: *mut u8,
    uname: *mut u8,
) {
    let mut packet: CArray<u8, 16> = zeroed();
    let mut gnameBackup: *mut u8 = gname;
    packet[0] = serialNo as u8;
    packet[1] = (serialNo >> 8) as u8;
    if mbootFlag != 0 {
        packet[1] = (serialNo >> 8) as u8 | 0x80;
    }
    for i in 2..15u8 {
        packet[i] = *({
            let t2 = gname;
            gname = gname.at(1);
            t2
        });
    }
    let mut check_sum: u8 = 0;
    let mut unameBackup: *mut u8 = uname;
    for i in 0..8u8 {
        check_sum += *({
            let t4 = unameBackup;
            unameBackup = unameBackup.at(1);
            t4
        });
        check_sum += *({
            let t6 = gnameBackup;
            gnameBackup = gnameBackup.at(1);
            t6
        });
    }
    packet[15] = !check_sum;
    if mbootFlag != 0 {
        packet[14] = 0;
    }
    STWI_set_Callback_M(core::mem::transmute::<
        Option<unsafe fn(u8, u16)>,
        *mut c_void,
    >(Some(rfu_CB_configGameData)));
    STWI_send_GameConfigREQ(packet.as_mut_ptr(), uname);
}
pub(crate) unsafe fn rfu_CB_configGameData(reqCommand: u8, reqResult: u16) {
    let mut serialNo: i32 = 0;
    let mut gname_uname_p: *mut u8 = null_mut();
    let mut packet_p: *mut u8 = null_mut();
    if reqResult == 0 {
        packet_p = (*(*gSTWIStatus).txPacket).rfuPacket8.data.as_mut_ptr();
        serialNo = ({
            (*gRfuLinkStatus).my.serialNo = *packet_p.at(4) as u16;
            (*gRfuLinkStatus).my.serialNo
        }) as i32;
        (*gRfuLinkStatus).my.serialNo = (*packet_p.at(5) as u16) << 8 | serialNo as u16;
        gname_uname_p = packet_p.at(6);
        if (*gRfuLinkStatus).my.serialNo as i32 & 0x8000 != 0 {
            (*gRfuLinkStatus).my.serialNo ^= 0x8000;
            (*gRfuLinkStatus).my.mbootFlag = 1;
        } else {
            (*gRfuLinkStatus).my.mbootFlag = 0;
        }
        for i in 0..(RFU_GAME_NAME_LENGTH as u8) {
            (*gRfuLinkStatus).my.gname[i] = *({
                let t2 = gname_uname_p;
                gname_uname_p = gname_uname_p.at(1);
                t2
            });
        }
        gname_uname_p = gname_uname_p.at(1);
        for i in 0..8u8 {
            (*gRfuLinkStatus).my.uname[i] = *({
                let t4 = gname_uname_p;
                gname_uname_p = gname_uname_p.at(1);
                t4
            });
        }
    }
    rfu_STC_REQ_callback(reqCommand, reqResult);
}
pub unsafe fn rfu_REQ_startSearchChild() {
    for i in 0..(RFU_CHILD_MAX as u16) {
        (*gRfuStatic).lsFixedCount[i] = 0;
    }
    STWI_set_Callback_M(core::mem::transmute::<
        Option<unsafe fn(u8, u16)>,
        *mut c_void,
    >(Some(rfu_CB_defaultCallback)));
    STWI_send_SystemStatusREQ();
    let result: u16 = STWI_poll_CommandEnd();
    if result == 0 {
        if (*(*gRfuFixed).STWIBuffer).rxPacketAlloc.rfuPacket8.data[7] == 0 {
            rfu_STC_clearLinkStatus(MODE_PARENT);
        }
    } else {
        rfu_STC_REQ_callback(ID_SC_START_REQ, result);
    }
    STWI_set_Callback_M(core::mem::transmute::<
        Option<unsafe fn(u8, u16)>,
        *mut c_void,
    >(Some(rfu_CB_startSearchChild)));
    STWI_send_SC_StartREQ();
}
pub(crate) unsafe fn rfu_CB_startSearchChild(reqCommand: u8, reqResult: u16) {
    if reqResult == 0 {
        (*gRfuStatic).SCStartFlag = 1;
    }
    rfu_STC_REQ_callback(reqCommand, reqResult);
}
unsafe fn rfu_STC_clearLinkStatus(parentChild: u8) {
    rfu_clearAllSlot();
    if parentChild != MODE_CHILD {
        {
            {
                let mut tmp: u16 = 0;
                volatile_write(&raw mut tmp, 0);
                CpuSet(
                    &raw mut tmp as *mut c_void,
                    (*gRfuLinkStatus).partner.as_mut_ptr() as *mut c_void,
                    0x1000040,
                );
            }
        }
        (*gRfuLinkStatus).findParentCount = 0;
    }
    for i in 0..RFU_CHILD_MAX {
        (*gRfuLinkStatus).strength[i] = 0;
    }
    (*gRfuLinkStatus).connCount = 0;
    (*gRfuLinkStatus).connSlotFlag = 0;
    (*gRfuLinkStatus).linkLossSlotFlag = 0;
    (*gRfuLinkStatus).getNameFlag = 0;
}
pub unsafe fn rfu_REQ_pollSearchChild() {
    STWI_set_Callback_M(core::mem::transmute::<
        Option<unsafe fn(u8, u16)>,
        *mut c_void,
    >(Some(rfu_CB_pollAndEndSearchChild)));
    STWI_send_SC_PollingREQ();
}
pub unsafe fn rfu_REQ_endSearchChild() {
    STWI_set_Callback_M(core::mem::transmute::<
        Option<unsafe fn(u8, u16)>,
        *mut c_void,
    >(Some(rfu_CB_pollAndEndSearchChild)));
    STWI_send_SC_EndREQ();
}
pub(crate) unsafe fn rfu_CB_pollAndEndSearchChild(reqCommand: u8, reqResult: u16) {
    if reqResult == 0 {
        rfu_STC_readChildList();
    }
    if reqCommand == ID_SC_POLL_REQ {
        if (*gRfuLinkStatus).my.id == 0 {
            STWI_set_Callback_M(core::mem::transmute::<
                Option<unsafe fn(u8, u16)>,
                *mut c_void,
            >(Some(rfu_CB_defaultCallback)));
            STWI_send_SystemStatusREQ();
            if STWI_poll_CommandEnd() == 0 {
                (*gRfuLinkStatus).my.id =
                    *(&raw mut (*(*gRfuFixed).STWIBuffer).rxPacketAlloc.rfuPacket32.data[0]
                        as *mut u16);
            }
        }
    } else if reqCommand == ID_SC_END_REQ {
        if (*gRfuLinkStatus).parentChild == MODE_NEUTRAL {
            (*gRfuLinkStatus).my.id = 0;
        }
        (*gRfuStatic).SCStartFlag = 0;
    }
    rfu_STC_REQ_callback(reqCommand, reqResult);
}
unsafe fn rfu_STC_readChildList() {
    let mut numSlots: u8 = (*(*gRfuFixed).STWIBuffer).rxPacketAlloc.rfuPacket8.data[1];
    let mut bm_slot_id: u8 = 0;
    let mut data_p: *mut u8 = &raw mut (*(*gRfuFixed).STWIBuffer).rxPacketAlloc.rfuPacket8.data[4];
    while numSlots != 0 {
        bm_slot_id = *data_p.at(2);
        if bm_slot_id < RFU_CHILD_MAX
            && shr_i32((*gRfuLinkStatus).connSlotFlag as i32, bm_slot_id as u32) & 1 == 0
            && shr_i32((*gRfuLinkStatus).linkLossSlotFlag as i32, bm_slot_id as u32) & 1 == 0
        {
            (*gRfuStatic).lsFixedCount[bm_slot_id] = 0xF0;
            (*gRfuLinkStatus).strength[bm_slot_id] = 16;
            (*gRfuLinkStatus).connSlotFlag |= shl_i32(1, bm_slot_id as u32) as u8;
            (*gRfuLinkStatus).connCount += 1;
            (*gRfuLinkStatus).partner[bm_slot_id].id = *(data_p as *mut u16);
            (*gRfuLinkStatus).partner[bm_slot_id].slot = bm_slot_id;
            (*gRfuLinkStatus).parentChild = MODE_PARENT;
            (*gRfuStatic).flags &= 0x7F;
            (*gRfuStatic).cidBak[bm_slot_id] = (*gRfuLinkStatus).partner[bm_slot_id].id;
        }
        numSlots -= 1;
        data_p = data_p.at(4);
    }
}
pub unsafe fn rfu_REQ_startSearchParent() {
    STWI_set_Callback_M(core::mem::transmute::<
        Option<unsafe fn(u8, u16)>,
        *mut c_void,
    >(Some(rfu_CB_startSearchParent)));
    STWI_send_SP_StartREQ();
}
pub(crate) unsafe fn rfu_CB_startSearchParent(reqCommand: u8, reqResult: u16) {
    if reqResult == 0 {
        rfu_STC_clearLinkStatus(MODE_CHILD);
    }
    rfu_STC_REQ_callback(reqCommand, reqResult);
}
pub unsafe fn rfu_REQ_pollSearchParent() {
    STWI_set_Callback_M(core::mem::transmute::<
        Option<unsafe fn(u8, u16)>,
        *mut c_void,
    >(Some(rfu_CB_pollSearchParent)));
    STWI_send_SP_PollingREQ();
}
pub(crate) unsafe fn rfu_CB_pollSearchParent(reqCommand: u8, reqResult: u16) {
    if reqResult == 0 {
        rfu_STC_readParentCandidateList();
    }
    rfu_STC_REQ_callback(reqCommand, reqResult);
}
pub unsafe fn rfu_REQ_endSearchParent() {
    STWI_set_Callback_M(core::mem::transmute::<
        Option<unsafe fn(u8, u16)>,
        *mut c_void,
    >(Some(rfu_STC_REQ_callback)));
    STWI_send_SP_EndREQ();
}
unsafe fn rfu_STC_readParentCandidateList() {
    let mut check_sum: u8 = 0;
    let mut my_check_sum: u8 = 0;
    let mut j: u8 = 0;
    let mut uname_p: *mut u8 = null_mut();
    let mut target: *mut RfuTgtData = null_mut();
    {
        {
            let mut tmp: u16 = 0;
            volatile_write(&raw mut tmp, 0);
            CpuSet(
                &raw mut tmp as *mut c_void,
                (*gRfuLinkStatus).partner.as_mut_ptr() as *mut c_void,
                0x1000040,
            );
        }
    }
    let mut packet_p: *mut u8 =
        &raw mut (*(*gRfuFixed).STWIBuffer).rxPacketAlloc.rfuPacket8.data[0];
    let mut numSlots: u8 = *packet_p.at(1);
    packet_p = packet_p.at(4);
    (*gRfuLinkStatus).findParentCount = 0;
    let mut i: u8 = 0;
    while i < RFU_CHILD_MAX && numSlots != 0 {
        numSlots -= 7;
        uname_p = packet_p.at(6);
        packet_p = packet_p.at(19);
        check_sum = !*packet_p;
        packet_p = packet_p.at(1);
        my_check_sum = 0;
        j = 0;
        while j < 8 {
            my_check_sum += *({
                let t2 = packet_p;
                packet_p = packet_p.at(1);
                t2
            });
            my_check_sum += *({
                let t4 = uname_p;
                uname_p = uname_p.at(1);
                t4
            });
            j += 1;
        }
        if my_check_sum == check_sum {
            packet_p = packet_p.at(-28);
            target = &raw mut (*gRfuLinkStatus).partner[(*gRfuLinkStatus).findParentCount];
            (*target).id = *(packet_p as *mut u16);
            packet_p = packet_p.at(2);
            (*target).slot = *packet_p;
            packet_p = packet_p.at(2);
            (*target).serialNo = *(packet_p as *mut u16) & 0x7FFF;
            if *(packet_p as *mut u16) as i32 & 0x8000 != 0 {
                (*target).mbootFlag = 1;
            } else {
                (*target).mbootFlag = 0;
            }
            packet_p = packet_p.at(2);
            for j in 0..(RFU_GAME_NAME_LENGTH as u8) {
                (*target).gname[j] = *({
                    let t6 = packet_p;
                    packet_p = packet_p.at(1);
                    t6
                });
            }
            packet_p = packet_p.at(1);
            for j in 0..8u8 {
                (*target).uname[j] = *({
                    let t8 = packet_p;
                    packet_p = packet_p.at(1);
                    t8
                });
            }
            (*gRfuLinkStatus).findParentCount += 1;
        }
        i += 1;
    }
}
pub unsafe fn rfu_REQ_startConnectParent(pid: u16) {
    let mut result: u16 = 0;
    let mut i: u8 = 0;
    while i < RFU_CHILD_MAX && (*gRfuLinkStatus).partner[i].id != pid {
        i += 1;
    }
    if i == RFU_CHILD_MAX {
        result = ERR_PID_NOT_FOUND;
    }
    if result == 0 {
        (*gRfuStatic).tryPid = pid;
        STWI_set_Callback_M(core::mem::transmute::<
            Option<unsafe fn(u8, u16)>,
            *mut c_void,
        >(Some(rfu_STC_REQ_callback)));
        STWI_send_CP_StartREQ(pid);
    } else {
        rfu_STC_REQ_callback(ID_CP_START_REQ, result);
    }
}
pub unsafe fn rfu_REQ_pollConnectParent() {
    STWI_set_Callback_M(core::mem::transmute::<
        Option<unsafe fn(u8, u16)>,
        *mut c_void,
    >(Some(rfu_CB_pollConnectParent)));
    STWI_send_CP_PollingREQ();
}
pub(crate) unsafe fn rfu_CB_pollConnectParent(reqCommand: u8, reqResult: u16) {
    let mut id: u16 = 0;
    let mut slot: u8 = 0;
    let mut bm_slot_flag: u8 = 0;
    let mut i: u8 = 0;
    let mut target_p: *mut RfuTgtData = null_mut();
    let mut target_local: RfuTgtData = zeroed();
    if reqResult == 0 {
        id = (*(*gRfuFixed).STWIBuffer).rxPacketAlloc.rfuPacket32.data[0] as u16;
        slot = (*(*gRfuFixed).STWIBuffer).rxPacketAlloc.rfuPacket8.data[6];
        if (*(*gRfuFixed).STWIBuffer).rxPacketAlloc.rfuPacket8.data[7] == 0 {
            bm_slot_flag = shl_i32(1, slot as u32) as u8;
            if bm_slot_flag as i32 & (*gRfuLinkStatus).connSlotFlag as i32 == 0 {
                (*gRfuLinkStatus).connSlotFlag |= bm_slot_flag;
                (*gRfuLinkStatus).linkLossSlotFlag &= !bm_slot_flag;
                (*gRfuLinkStatus).my.id = id;
                (*gRfuLinkStatus).connCount += 1;
                (*gRfuLinkStatus).parentChild = MODE_CHILD;
                (*gRfuStatic).flags |= 0x80;
                i = 0;
                while i < RFU_CHILD_MAX {
                    if (*gRfuLinkStatus).partner[i].id == (*gRfuStatic).tryPid {
                        if (*gRfuLinkStatus).findParentCount != 0 {
                            target_p = &raw mut target_local;
                            CpuSet(
                                &raw mut (*gRfuLinkStatus).partner[i] as *mut c_void,
                                &raw mut target_local as *mut c_void,
                                16,
                            );
                            {
                                {
                                    let mut tmp: u16 = 0;
                                    volatile_write(&raw mut tmp, 0);
                                    CpuSet(
                                        &raw mut tmp as *mut c_void,
                                        (*gRfuLinkStatus).partner.as_mut_ptr() as *mut c_void,
                                        0x1000040,
                                    );
                                }
                            }
                            (*gRfuLinkStatus).findParentCount = 0;
                        } else {
                            target_p = &raw mut (*gRfuLinkStatus).partner[i];
                        }
                        break;
                    }
                    i += 1;
                }
                if i < RFU_CHILD_MAX {
                    CpuSet(
                        target_p as *mut c_void,
                        &raw mut (*gRfuLinkStatus).partner[slot] as *mut c_void,
                        16,
                    );
                    (*gRfuLinkStatus).partner[slot].slot = slot;
                }
            }
        }
    }
    rfu_STC_REQ_callback(reqCommand, reqResult);
}
pub unsafe fn rfu_getConnectParentStatus(status: *mut u8, connectSlotNo: *mut u8) -> u16 {
    *status = 0xFF;
    let mut packet_p: *mut u8 = (*(*gRfuFixed).STWIBuffer)
        .rxPacketAlloc
        .rfuPacket8
        .data
        .as_mut_ptr();
    if *packet_p == 0xa0 || *packet_p == 0xa1 {
        packet_p = packet_p.at(6);
        *connectSlotNo = *packet_p;
        *status = *packet_p.at(1);
        return 0;
    }
    ERR_REQ_CMD_ID
}
pub unsafe fn rfu_REQ_endConnectParent() {
    STWI_set_Callback_M(core::mem::transmute::<
        Option<unsafe fn(u8, u16)>,
        *mut c_void,
    >(Some(rfu_CB_pollConnectParent)));
    STWI_send_CP_EndREQ();
    if (*(*gRfuFixed).STWIBuffer).rxPacketAlloc.rfuPacket8.data[6] < 4 {
        (*gRfuStatic).linkEmergencyFlag
            [(*(*gRfuFixed).STWIBuffer).rxPacketAlloc.rfuPacket8.data[6]] = 0;
    }
}
pub unsafe fn rfu_syncVBlank() -> u16 {
    let mut bmSlotFlag: i32 = 0;
    rfu_NI_checkCommFailCounter();
    if (*gRfuLinkStatus).parentChild == MODE_NEUTRAL {
        return 0;
    }
    if (*gRfuStatic).nowWatchInterval != 0 {
        (*gRfuStatic).nowWatchInterval -= 1;
    }
    let masterSlave: u8 = rfu_getMasterSlave();
    if (*gRfuStatic).flags as i32 & 2 == 0 {
        if masterSlave == AGB_CLK_SLAVE {
            (*gRfuStatic).flags |= 4;
            (*gRfuStatic).watchdogTimer = 360;
        }
    } else if masterSlave != AGB_CLK_SLAVE {
        (*gRfuStatic).flags &= 0xFB;
    }
    if masterSlave != AGB_CLK_SLAVE {
        (*gRfuStatic).flags &= 0xFD;
    } else {
        (*gRfuStatic).flags |= 2;
    }
    if (*gRfuStatic).flags as i32 & 4 == 0 {
        return 0;
    }
    if (*gRfuStatic).watchdogTimer == 0 {
        (*gRfuStatic).flags &= 0xFB;
        bmSlotFlag =
            (*gRfuLinkStatus).connSlotFlag as i32 | (*gRfuLinkStatus).linkLossSlotFlag as i32;
        for i in 0..RFU_CHILD_MAX {
            if shr_i32(bmSlotFlag, i as u32) & 1 != 0 {
                rfu_STC_removeLinkData(i, 1);
            }
        }
        (*gRfuLinkStatus).parentChild = MODE_NEUTRAL;
        return 1;
    }
    (*gRfuStatic).watchdogTimer -= 1;
    0
}
pub unsafe fn rfu_REQBN_watchLink(
    reqCommandId: u16,
    bmLinkLossSlot: *mut u8,
    linkLossReason: *mut u8,
    parentBmLinkRecoverySlot: *mut u8,
) -> u16 {
    let mut reasonMaybe: u8 = 0;
    let mut i: u8 = 0;
    let mut packet_p: *mut u8 = null_mut();
    let mut to_req_disconnect: u8 = 0;
    let mut newLinkLossFlag: u8 = 0;
    let mut num_packets: u8 = 0;
    let mut connSlotFlag: u8 = 0;
    let mut to_disconnect: u8 = 0;
    *bmLinkLossSlot = 0;
    *linkLossReason = REASON_DISCONNECTED;
    *parentBmLinkRecoverySlot = 0;
    if (*gRfuLinkStatus).parentChild == MODE_NEUTRAL
        || (&raw mut (*gSTWIStatus).msMode).read_volatile() == 0
    {
        return 0;
    }
    if (*gRfuStatic).flags as i32 & 4 != 0 {
        (*gRfuStatic).watchdogTimer = 360;
    }
    if (*gRfuStatic).nowWatchInterval == 0 {
        (*gRfuStatic).nowWatchInterval = 4;
        reasonMaybe = 1;
    }
    if reqCommandId as u8 == ID_DISCONNECTED_AND_CHANGE_REQ {
        let packet_p_2: *mut u8 = (*(*gRfuFixed).STWIBuffer)
            .rxPacketAlloc
            .rfuPacket8
            .data
            .as_mut_ptr();
        *bmLinkLossSlot = *packet_p_2.at(4);
        *linkLossReason = *packet_p_2.at(5);
        if *linkLossReason == REASON_LINK_LOSS {
            *bmLinkLossSlot = (*gRfuLinkStatus).connSlotFlag;
        }
        reasonMaybe = 2;
    } else {
        if reqCommandId == 0x0136 {
            newLinkLossFlag = (*(*gRfuFixed).STWIBuffer).rxPacketAlloc.rfuPacket8.data[5];
            newLinkLossFlag ^= (*gRfuLinkStatus).connSlotFlag;
            *bmLinkLossSlot = newLinkLossFlag & (*gRfuLinkStatus).connSlotFlag;
            *linkLossReason = REASON_LINK_LOSS;
            for i in 0..RFU_CHILD_MAX {
                if shr_i32(*bmLinkLossSlot as i32, i as u32) & 1 != 0 {
                    (*gRfuLinkStatus).strength[i] = 0;
                    rfu_STC_removeLinkData(i, 0);
                }
            }
        }
        if reasonMaybe == 0 {
            return 0;
        }
    }
    let stwiCommand: i32 = (*(*gRfuFixed).STWIBuffer).rxPacketAlloc.rfuPacket32.command as i32;
    let stwiParam: i32 = (*(*gRfuFixed).STWIBuffer).rxPacketAlloc.rfuPacket32.data[0] as i32;
    STWI_set_Callback_M(core::mem::transmute::<
        Option<unsafe fn(u8, u16)>,
        *mut c_void,
    >(Some(rfu_CB_defaultCallback)));
    STWI_send_LinkStatusREQ();
    let reqResult: u8 = STWI_poll_CommandEnd() as u8;
    if reqResult == 0 {
        packet_p = &raw mut (*(*gRfuFixed).STWIBuffer).rxPacketAlloc.rfuPacket8.data[4];
        for i in 0..RFU_CHILD_MAX {
            (*gRfuLinkStatus).strength[i] = *({
                let t2 = packet_p;
                packet_p = packet_p.at(1);
                t2
            });
        }
        to_req_disconnect = 0;
        i = 0;
    } else {
        rfu_STC_REQ_callback(ID_LINK_STATUS_REQ, reqResult as u16);
        return reqResult as u16;
    }
    while i < RFU_CHILD_MAX {
        if (*gRfuStatic).lsFixedCount[i] != 0 {
            (*gRfuStatic).lsFixedCount[i] -= 4;
            if (*gRfuLinkStatus).strength[i] <= 15 {
                (*gRfuLinkStatus).strength[i] = 16;
            }
        }
        newLinkLossFlag = shl_i32(1, i as u32) as u8;
        if reqResult == 0 {
            if reasonMaybe == 1
                && (*gRfuLinkStatus).connSlotFlag as i32 & newLinkLossFlag as i32 != 0
            {
                if (*gRfuLinkStatus).strength[i] == 0 {
                    if (*gRfuLinkStatus).parentChild == MODE_PARENT {
                        (*gRfuStatic).linkEmergencyFlag[i] += 1;
                        if (*gRfuStatic).linkEmergencyFlag[i] > 3 {
                            *bmLinkLossSlot |= newLinkLossFlag;
                            *linkLossReason = REASON_LINK_LOSS;
                        }
                    } else {
                        STWI_send_SystemStatusREQ();
                        if STWI_poll_CommandEnd() == 0 {
                            if (*(*gRfuFixed).STWIBuffer).rxPacketAlloc.rfuPacket8.data[7] == 0 {
                                *bmLinkLossSlot |= newLinkLossFlag;
                                *linkLossReason = REASON_LINK_LOSS;
                            } else {
                                if ({
                                    (*gRfuStatic).linkEmergencyFlag[i] += 1;
                                    (*gRfuStatic).linkEmergencyFlag[i]
                                }) as u16
                                    > (*gRfuStatic).linkEmergencyLimit
                                {
                                    (*gRfuStatic).linkEmergencyFlag[i] = 0;
                                    STWI_send_DisconnectREQ((*gRfuLinkStatus).connSlotFlag);
                                    STWI_poll_CommandEnd();
                                    *bmLinkLossSlot |= newLinkLossFlag;
                                    *linkLossReason = REASON_LINK_LOSS;
                                }
                            }
                        }
                    }
                } else {
                    (*gRfuStatic).linkEmergencyFlag[i] = 0;
                }
            }
            if (*gRfuLinkStatus).parentChild == MODE_PARENT && (*gRfuLinkStatus).strength[i] != 0 {
                if newLinkLossFlag as i32 & (*gRfuLinkStatus).linkLossSlotFlag as i32 != 0 {
                    if (*gRfuLinkStatus).strength[i] > 10 {
                        *parentBmLinkRecoverySlot |= newLinkLossFlag;
                        (*gRfuLinkStatus).connSlotFlag |= newLinkLossFlag;
                        (*gRfuLinkStatus).linkLossSlotFlag &= !newLinkLossFlag;
                        (*gRfuLinkStatus).connCount += 1;
                        (*gRfuStatic).linkEmergencyFlag[i] = 0;
                    } else {
                        (*gRfuLinkStatus).strength[i] = 0;
                    }
                } else {
                    if ((*gRfuLinkStatus).connSlotFlag as i32
                        | (*gRfuLinkStatus).linkLossSlotFlag as i32)
                        & newLinkLossFlag as i32
                        == 0
                    {
                        STWI_send_SlotStatusREQ();
                        STWI_poll_CommandEnd();
                        packet_p = (*(*gRfuFixed).STWIBuffer)
                            .rxPacketAlloc
                            .rfuPacket8
                            .data
                            .as_mut_ptr();
                        num_packets = *packet_p.at(1) - 1;
                        packet_p = packet_p.at(8);
                        while num_packets != 0 {
                            let cid: u16 = *(packet_p as *mut u16);
                            if *packet_p.at(2) == i && cid == (*gRfuStatic).cidBak[i] {
                                to_req_disconnect |= shl_i32(1, i as u32) as u8;
                                break;
                            }
                            packet_p = packet_p.at(4);
                            num_packets -= 1;
                        }
                    }
                }
            }
        }
        connSlotFlag = (*gRfuLinkStatus).connSlotFlag;
        to_disconnect = *bmLinkLossSlot;
        to_disconnect &= connSlotFlag;
        if newLinkLossFlag as i32 & to_disconnect as i32 != 0 {
            rfu_STC_removeLinkData(i, 0);
        }
        i += 1;
    }
    if to_req_disconnect != 0 {
        STWI_send_DisconnectREQ(to_req_disconnect);
        STWI_poll_CommandEnd();
    }
    *((*(*gRfuFixed).STWIBuffer)
        .rxPacketAlloc
        .rfuPacket8
        .data
        .as_mut_ptr() as *mut u32) = stwiCommand as u32;
    (*(*gRfuFixed).STWIBuffer).rxPacketAlloc.rfuPacket32.data[0] = stwiParam as u32;
    0
}
unsafe fn rfu_STC_removeLinkData(bmConnectedPartnerId: u8, bmDisconnect: u8) {
    let bmLinkLossFlag: u8 = shl_i32(1, bmConnectedPartnerId as u32) as u8;
    let mut bmLinkRetainedFlag: i32 = 0;
    (*gRfuStatic).lsFixedCount[bmConnectedPartnerId] = 0;
    if (*gRfuLinkStatus).connSlotFlag as i32 & bmLinkLossFlag as i32 != 0
        && (*gRfuLinkStatus).connCount != 0
    {
        (*gRfuLinkStatus).connCount -= 1;
    }
    (*gRfuLinkStatus).connSlotFlag &= ({
        bmLinkRetainedFlag = !(bmLinkLossFlag as i32);
        bmLinkRetainedFlag
    }) as u8;
    (*gRfuLinkStatus).linkLossSlotFlag |= bmLinkLossFlag;
    if (*gRfuLinkStatus).parentChild == 0x00 && (*gRfuLinkStatus).connSlotFlag == 0 {
        (*gRfuLinkStatus).parentChild = MODE_NEUTRAL;
    }
    if bmDisconnect != 0 {
        {
            {
                let mut tmp: u16 = 0;
                volatile_write(&raw mut tmp, 0);
                CpuSet(
                    &raw mut tmp as *mut c_void,
                    &raw mut (*gRfuLinkStatus).partner[bmConnectedPartnerId] as *mut c_void,
                    0x1000010,
                );
            }
        }
        (*gRfuLinkStatus).linkLossSlotFlag &= bmLinkRetainedFlag as u8;
        (*gRfuLinkStatus).getNameFlag &= bmLinkRetainedFlag as u8;
        (*gRfuLinkStatus).strength[bmConnectedPartnerId] = 0;
    }
}
pub unsafe fn rfu_REQ_disconnect(bmDisconnectSlot: u8) {
    let mut result: u16 = 0;
    if ((*gRfuLinkStatus).connSlotFlag as i32 | (*gRfuLinkStatus).linkLossSlotFlag as i32)
        & bmDisconnectSlot as i32
        != 0
    {
        (*gRfuStatic).recoveryBmSlot = bmDisconnectSlot;
        if (*gRfuLinkStatus).parentChild == MODE_NEUTRAL && (*gRfuStatic).flags as i32 & 0x80 != 0 {
            if (*gRfuLinkStatus).linkLossSlotFlag as i32 & bmDisconnectSlot as i32 != 0 {
                rfu_CB_disconnect(48, 0);
            }
        } else if (*gRfuStatic).SCStartFlag != 0
            && ({
                STWI_set_Callback_M(core::mem::transmute::<
                    Option<unsafe fn(u8, u16)>,
                    *mut c_void,
                >(Some(rfu_CB_defaultCallback)));
                STWI_send_SC_EndREQ();
                ({
                    result = STWI_poll_CommandEnd();
                    result
                }) != 0
            })
        {
            rfu_STC_REQ_callback(ID_SC_END_REQ, result);
        } else {
            STWI_set_Callback_M(core::mem::transmute::<
                Option<unsafe fn(u8, u16)>,
                *mut c_void,
            >(Some(rfu_CB_disconnect)));
            STWI_send_DisconnectREQ(bmDisconnectSlot);
        }
    }
}
pub(crate) unsafe fn rfu_CB_disconnect(reqCommand: u8, mut reqResult: u16) {
    let mut bm_slot_flag: u8 = 0;
    if reqResult == 3 && (*gRfuLinkStatus).parentChild == MODE_CHILD {
        STWI_set_Callback_M(core::mem::transmute::<
            Option<unsafe fn(u8, u16)>,
            *mut c_void,
        >(Some(rfu_CB_defaultCallback)));
        STWI_send_SystemStatusREQ();
        if STWI_poll_CommandEnd() == 0
            && (*(*gRfuFixed).STWIBuffer).rxPacketAlloc.rfuPacket8.data[7] == 0
        {
            reqResult = 0;
        }
    }
    (*gRfuStatic).recoveryBmSlot &=
        (*gRfuLinkStatus).connSlotFlag | (*gRfuLinkStatus).linkLossSlotFlag;
    (*(*gRfuFixed).STWIBuffer).rxPacketAlloc.rfuPacket8.data[8] = (*gRfuStatic).recoveryBmSlot;
    if reqResult == 0 {
        for i in 0..RFU_CHILD_MAX {
            bm_slot_flag = shl_i32(1, i as u32) as u8;
            if bm_slot_flag as i32 & (*gRfuStatic).recoveryBmSlot as i32 != 0 {
                rfu_STC_removeLinkData(i, 1);
            }
        }
    }
    if (*gRfuLinkStatus).connSlotFlag as i32 | (*gRfuLinkStatus).linkLossSlotFlag as i32 == 0 {
        (*gRfuLinkStatus).parentChild = MODE_NEUTRAL;
    }
    rfu_STC_REQ_callback(reqCommand, reqResult);
    if (*gRfuStatic).SCStartFlag != 0 {
        STWI_set_Callback_M(core::mem::transmute::<
            Option<unsafe fn(u8, u16)>,
            *mut c_void,
        >(Some(rfu_CB_defaultCallback)));
        STWI_send_SC_StartREQ();
        reqResult = STWI_poll_CommandEnd();
        if reqResult != 0 {
            rfu_STC_REQ_callback(ID_SC_START_REQ, reqResult);
        }
    }
}
pub unsafe fn rfu_REQ_CHILD_startConnectRecovery(bmRecoverySlot: u8) {
    (*gRfuStatic).recoveryBmSlot = bmRecoverySlot;
    let mut i: u8 = 0;
    while i < RFU_CHILD_MAX && shr_i32(bmRecoverySlot as i32, i as u32) & 1 == 0 {
        i += 1;
    }
    STWI_set_Callback_M(core::mem::transmute::<
        Option<unsafe fn(u8, u16)>,
        *mut c_void,
    >(Some(rfu_STC_REQ_callback)));
    STWI_send_CPR_StartREQ(
        (*gRfuLinkStatus).partner[i].id,
        (*gRfuLinkStatus).my.id,
        bmRecoverySlot,
    );
}
pub unsafe fn rfu_REQ_CHILD_pollConnectRecovery() {
    STWI_set_Callback_M(core::mem::transmute::<
        Option<unsafe fn(u8, u16)>,
        *mut c_void,
    >(Some(rfu_CB_CHILD_pollConnectRecovery)));
    STWI_send_CPR_PollingREQ();
}
pub(crate) unsafe fn rfu_CB_CHILD_pollConnectRecovery(reqCommand: u8, reqResult: u16) {
    let mut bm_slot_flag: u8 = 0;
    let mut rfuLinkStatus: *mut RfuLinkStatus = null_mut();
    if reqResult == 0
        && (*(*gRfuFixed).STWIBuffer).rxPacketAlloc.rfuPacket8.data[4] == 0
        && (*gRfuStatic).recoveryBmSlot != 0
    {
        (*gRfuLinkStatus).parentChild = MODE_CHILD;
        for i in 0..RFU_CHILD_MAX {
            bm_slot_flag = shl_i32(1, i as u32) as u8;
            rfuLinkStatus = gRfuLinkStatus;
            if (*gRfuStatic).recoveryBmSlot as i32
                & bm_slot_flag as i32
                & (*rfuLinkStatus).linkLossSlotFlag as i32
                != 0
            {
                (*gRfuLinkStatus).connSlotFlag |= bm_slot_flag;
                (*gRfuLinkStatus).linkLossSlotFlag &= !bm_slot_flag;
                (*gRfuLinkStatus).connCount += 1;
                (*gRfuStatic).linkEmergencyFlag[i] = 0;
            }
        }
        (*gRfuStatic).recoveryBmSlot = 0;
    }
    rfu_STC_REQ_callback(reqCommand, reqResult);
}
pub unsafe fn rfu_CHILD_getConnectRecoveryStatus(status: *mut u8) -> u16 {
    *status = 0xFF;
    if (*(*gRfuFixed).STWIBuffer).rxPacketAlloc.rfuPacket8.data[0] == 0xB3
        || (*(*gRfuFixed).STWIBuffer).rxPacketAlloc.rfuPacket8.data[0] == 0xB4
    {
        *status = (*(*gRfuFixed).STWIBuffer).rxPacketAlloc.rfuPacket8.data[4];
        return 0;
    }
    ERR_REQ_CMD_ID
}
pub unsafe fn rfu_REQ_CHILD_endConnectRecovery() {
    STWI_set_Callback_M(core::mem::transmute::<
        Option<unsafe fn(u8, u16)>,
        *mut c_void,
    >(Some(rfu_CB_CHILD_pollConnectRecovery)));
    STWI_send_CPR_EndREQ();
}
// hand-written: tools/rustport/overrides/librfu_rfu/rfu_STC_fastCopy.rs
/// `rfu_STC_fastCopy`. rfu_initializeAPI copies this function's code (0x60
/// bytes) into gRfuFixed->fastCopyBuffer and calls the copy, so it must not
/// call anything: the volatile reads keep the loop from becoming a memcpy
/// call. rustcheck.sh checks its size and that it makes no calls.
#[unsafe(no_mangle)]
unsafe fn rfu_STC_fastCopy(src_p: *mut *mut u8, dst_p: *mut *mut u8, size: i32) {
    unsafe {
        let mut src = src_p.read();
        let mut dst = dst_p.read();
        let mut i = size - 1;
        while i != -1 {
            dst.write_volatile(src.read_volatile());
            dst = dst.add(1);
            src = src.add(1);
            i -= 1;
        }
        src_p.write(src);
        dst_p.write(dst);
    }
}

pub unsafe fn rfu_REQ_changeMasterSlave() {
    if STWI_read_status(1) == 1 {
        STWI_set_Callback_M(core::mem::transmute::<
            Option<unsafe fn(u8, u16)>,
            *mut c_void,
        >(Some(rfu_STC_REQ_callback)));
        STWI_send_MS_ChangeREQ();
    } else {
        rfu_STC_REQ_callback(ID_MS_CHANGE_REQ, 0);
    }
}
pub unsafe fn rfu_getMasterSlave() -> u8 {
    let mut masterSlave: u8 = STWI_read_status(1) as u8;
    if masterSlave == AGB_CLK_MASTER
        && (&raw mut (*gSTWIStatus).sending).read_volatile() != 0
        && ((*gSTWIStatus).reqActiveCommand == ID_MS_CHANGE_REQ
            || (*gSTWIStatus).reqActiveCommand == ID_DATA_TX_AND_CHANGE_REQ
            || (*gSTWIStatus).reqActiveCommand == ID_RESUME_RETRANSMIT_AND_CHANGE_REQ)
    {
        masterSlave = AGB_CLK_SLAVE;
    }
    masterSlave
}
pub unsafe fn rfu_clearAllSlot() {
    let IMEBackup: u16 = (67109384_usize as *mut u16).read_volatile();
    volatile_write(67109384_usize as *mut u16, 0);
    for i in 0..(RFU_CHILD_MAX as u16) {
        {
            {
                let mut tmp: u16 = 0;
                volatile_write(&raw mut tmp, 0);
                CpuSet(
                    &raw mut tmp as *mut c_void,
                    gRfuSlotStatusNI[i] as *mut c_void,
                    0x1000034,
                );
            }
        }
        {
            {
                let mut tmp: u16 = 0;
                volatile_write(&raw mut tmp, 0);
                CpuSet(
                    &raw mut tmp as *mut c_void,
                    gRfuSlotStatusUNI[i] as *mut c_void,
                    0x100000a,
                );
            }
        }
        (*gRfuLinkStatus).remainLLFrameSizeChild[i] = 16;
    }
    (*gRfuLinkStatus).remainLLFrameSizeParent = LLF_P_SIZE;
    (*gRfuLinkStatus).sendSlotNIFlag = 0;
    (*gRfuLinkStatus).recvSlotNIFlag = 0;
    (*gRfuLinkStatus).sendSlotUNIFlag = 0;
    (*gRfuStatic).recvRenewalFlag = 0;
    volatile_write(67109384_usize as *mut u16, IMEBackup);
}
unsafe fn rfu_STC_releaseFrame(bm_slot_id: u8, send_recv: u8, NI_comm: *mut NIComm) {
    if (*gRfuStatic).flags as i32 & 0x80 == 0 {
        if send_recv == 0 {
            (*gRfuLinkStatus).remainLLFrameSizeParent += (*NI_comm).payloadSize as u8;
        }
        (*gRfuLinkStatus).remainLLFrameSizeParent += 3;
    } else {
        if send_recv == 0 {
            (*gRfuLinkStatus).remainLLFrameSizeChild[bm_slot_id] += (*NI_comm).payloadSize as u8;
        }
        (*gRfuLinkStatus).remainLLFrameSizeChild[bm_slot_id] += 2;
    }
}
pub unsafe fn rfu_clearSlot(connTypeFlag: u8, slotStatusIndex: u8) -> u16 {
    let mut NI_comm: *mut NIComm = null_mut();
    if slotStatusIndex >= RFU_CHILD_MAX {
        return ERR_SLOT_NO;
    }
    if connTypeFlag as i32 & 15 == 0 {
        return ERR_COMM_TYPE;
    }
    let imeBak: u16 = (67109384_usize as *mut u16).read_volatile();
    volatile_write(67109384_usize as *mut u16, 0);
    if connTypeFlag as i32 & 12 != 0 {
        for send_recv in 0..2u16 {
            NI_comm = null_mut();
            if send_recv == 0 {
                if connTypeFlag as i32 & TYPE_NI_SEND as i32 != 0 {
                    NI_comm = &raw mut (*gRfuSlotStatusNI[slotStatusIndex]).send;
                    (*gRfuLinkStatus).sendSlotNIFlag &= !(*NI_comm).bmSlotOrg;
                }
            } else {
                if connTypeFlag as i32 & TYPE_NI_RECV as i32 != 0 {
                    NI_comm = &raw mut (*gRfuSlotStatusNI[slotStatusIndex]).recv;
                    (*gRfuLinkStatus).recvSlotNIFlag &= !(shl_i32(1, slotStatusIndex as u32) as u8);
                }
            }
            if !NI_comm.is_null() {
                if (*NI_comm).state as i32 & SLOT_BUSY_FLAG != 0 {
                    rfu_STC_releaseFrame(slotStatusIndex, send_recv as u8, NI_comm);
                    for i in 0..(RFU_CHILD_MAX as u16) {
                        if shr_i32((*NI_comm).bmSlotOrg as i32, i as u32) & 1 != 0 {
                            (*NI_comm).failCounter = 0;
                        }
                    }
                }
                {
                    {
                        let mut tmp: u16 = 0;
                        volatile_write(&raw mut tmp, 0);
                        CpuSet(
                            &raw mut tmp as *mut c_void,
                            NI_comm as *mut c_void,
                            0x100001a,
                        );
                    }
                }
            }
        }
    }
    if connTypeFlag as i32 & TYPE_UNI_SEND != 0 {
        let slotStatusUNI: *mut RfuSlotStatusUNI = gRfuSlotStatusUNI[slotStatusIndex];
        if (*slotStatusUNI).send.state as i32 & SLOT_BUSY_FLAG != 0 {
            if (*gRfuStatic).flags as i32 & 0x80 == 0 {
                (*gRfuLinkStatus).remainLLFrameSizeParent +=
                    3 + (*slotStatusUNI).send.payloadSize as u8;
            } else {
                (*gRfuLinkStatus).remainLLFrameSizeChild[slotStatusIndex] +=
                    2 + (*slotStatusUNI).send.payloadSize as u8;
            }
            (*gRfuLinkStatus).sendSlotUNIFlag &= !(*slotStatusUNI).send.bmSlot;
        }
        {
            {
                let mut tmp: u16 = 0;
                volatile_write(&raw mut tmp, 0);
                CpuSet(
                    &raw mut tmp as *mut c_void,
                    &raw mut (*slotStatusUNI).send as *mut c_void,
                    0x1000006,
                );
            }
        }
    }
    if connTypeFlag as i32 & TYPE_UNI_RECV != 0 {
        {
            {
                let mut tmp: u16 = 0;
                volatile_write(&raw mut tmp, 0);
                CpuSet(
                    &raw mut tmp as *mut c_void,
                    &raw mut (*gRfuSlotStatusUNI[slotStatusIndex]).recv as *mut c_void,
                    0x1000004,
                );
            }
        }
    }
    volatile_write(67109384_usize as *mut u16, imeBak);
    0
}
pub unsafe fn rfu_setRecvBuffer(
    connType: u8,
    slotNo: u8,
    buffer: *mut c_void,
    buffSize: u32,
) -> u16 {
    if slotNo >= RFU_CHILD_MAX {
        return ERR_SLOT_NO;
    }
    if connType as i32 & TYPE_NI as i32 != 0 {
        (*gRfuSlotStatusNI[slotNo]).recvBuffer = buffer;
        (*gRfuSlotStatusNI[slotNo]).recvBufferSize = buffSize;
    } else if connType as i32 & TYPE_UNI as i32 == 0 {
        return ERR_COMM_TYPE;
    } else {
        (*gRfuSlotStatusUNI[slotNo]).recvBuffer = buffer;
        (*gRfuSlotStatusUNI[slotNo]).recvBufferSize = buffSize;
    }
    0
}
pub unsafe fn rfu_NI_setSendData(
    bmSendSlot: u8,
    subFrameSize: u8,
    src: *mut c_void,
    size: u32,
) -> u16 {
    rfu_STC_setSendData_org(32, bmSendSlot, subFrameSize, src, size)
}
pub unsafe fn rfu_UNI_setSendData(bmSendSlot: u8, src: *mut c_void, size: u8) -> u16 {
    let mut subFrameSize: u8 = 0;
    if (*gRfuLinkStatus).parentChild == MODE_PARENT {
        subFrameSize = size + 3;
    } else {
        subFrameSize = size + 2;
    }
    rfu_STC_setSendData_org(16, bmSendSlot, subFrameSize, src, 0)
}
pub unsafe fn rfu_NI_CHILD_setSendGameName(slotNo: u8, subFrameSize: u8) -> u16 {
    rfu_STC_setSendData_org(
        64,
        shl_i32(1, slotNo as u32) as u8,
        subFrameSize,
        &raw mut (*gRfuLinkStatus).my.serialNo as *mut c_void,
        26,
    )
}
unsafe fn rfu_STC_setSendData_org(
    ni_or_uni: u8,
    bmSendSlot: u8,
    subFrameSize: u8,
    src: *mut c_void,
    dataSize: u32,
) -> u16 {
    let mut sendSlotFlag: u8 = 0;
    let mut frameSize: u8 = 0;
    let mut llFrameSize_p: *mut u8 = null_mut();
    let mut slotStatus_UNI: *mut RfuSlotStatusUNI = null_mut();
    let mut slotStatus_NI: *mut RfuSlotStatusNI = null_mut();
    if (*gRfuLinkStatus).parentChild == MODE_NEUTRAL {
        return ERR_MODE_NOT_CONNECTED;
    }
    if bmSendSlot as i32 & 0xF == 0 {
        return ERR_SLOT_NO;
    }
    if ((*gRfuLinkStatus).connSlotFlag as i32 | (*gRfuLinkStatus).linkLossSlotFlag as i32)
        & bmSendSlot as i32
        != bmSendSlot as i32
    {
        return ERR_SLOT_NOT_CONNECTED;
    }
    if ni_or_uni as i32 & 0x10 != 0 {
        sendSlotFlag = (*gRfuLinkStatus).sendSlotUNIFlag;
    } else {
        sendSlotFlag = (*gRfuLinkStatus).sendSlotNIFlag;
    }
    if sendSlotFlag as i32 & bmSendSlot as i32 != 0 {
        return ERR_SLOT_BUSY;
    }
    let mut bm_slot_id: u8 = 0;
    while bm_slot_id < RFU_CHILD_MAX && shr_i32(bmSendSlot as i32, bm_slot_id as u32) & 1 == 0 {
        bm_slot_id += 1;
    }
    if (*gRfuLinkStatus).parentChild == MODE_PARENT {
        llFrameSize_p = &raw mut (*gRfuLinkStatus).remainLLFrameSizeParent;
    } else if (*gRfuLinkStatus).parentChild == MODE_CHILD {
        llFrameSize_p = &raw mut (*gRfuLinkStatus).remainLLFrameSizeChild[bm_slot_id];
    }
    frameSize = llsf_struct[(*gRfuLinkStatus).parentChild].frameSize;
    if !llFrameSize_p.is_null() && subFrameSize > *llFrameSize_p || subFrameSize <= frameSize {
        return ERR_SUBFRAME_SIZE;
    }
    let imeBak: u16 = (67109384_usize as *mut u16).read_volatile();
    volatile_write(67109384_usize as *mut u16, 0);
    let sending: u8 = ni_or_uni & 0x20;
    if sending != 0 || ni_or_uni == 0x40 {
        slotStatus_NI = gRfuSlotStatusNI[bm_slot_id];
        (*slotStatus_NI).send.errorCode = 0;
        (*slotStatus_NI).send.now_p[0] = &raw mut (*slotStatus_NI).send.dataType;
        (*slotStatus_NI).send.remainSize = 7;
        (*slotStatus_NI).send.bmSlotOrg = bmSendSlot;
        (*slotStatus_NI).send.bmSlot = bmSendSlot;
        (*slotStatus_NI).send.payloadSize = subFrameSize as u16 - frameSize as u16;
        if sending != 0 {
            (*slotStatus_NI).send.dataType = 0;
        } else {
            (*slotStatus_NI).send.dataType = 1;
        }
        (*slotStatus_NI).send.dataSize = dataSize;
        (*slotStatus_NI).send.src = src;
        (*slotStatus_NI).send.ack = 0;
        (*slotStatus_NI).send.phase = 0;
        for i in 0..WINDOW_COUNT {
            (*slotStatus_NI).send.recvAckFlag[i] = 0;
            (*slotStatus_NI).send.n[i] = 1;
        }
        for bm_slot_id in 0..RFU_CHILD_MAX {
            if shr_i32(bmSendSlot as i32, bm_slot_id as u32) & 1 != 0 {
                (*gRfuSlotStatusNI[bm_slot_id]).send.failCounter = 0;
            }
        }
        (*gRfuLinkStatus).sendSlotNIFlag |= bmSendSlot;
        if !llFrameSize_p.is_null() {
            *llFrameSize_p -= subFrameSize;
        }
        (*slotStatus_NI).send.state = SLOT_STATE_SEND_START;
    } else if ni_or_uni as i32 & 0x10 != 0 {
        slotStatus_UNI = gRfuSlotStatusUNI[bm_slot_id];
        (*slotStatus_UNI).send.bmSlot = bmSendSlot;
        (*slotStatus_UNI).send.src = src;
        (*slotStatus_UNI).send.payloadSize = subFrameSize as u16 - frameSize as u16;
        if !llFrameSize_p.is_null() {
            *llFrameSize_p -= subFrameSize;
        }
        (*slotStatus_UNI).send.state = SLOT_STATE_SEND_UNI;
        (*gRfuLinkStatus).sendSlotUNIFlag |= bmSendSlot;
    }
    volatile_write(67109384_usize as *mut u16, imeBak);
    0
}
pub unsafe fn rfu_changeSendTarget(mut connType: u8, slotStatusIndex: u8, bmNewTgtSlot: u8) -> u16 {
    let mut slotStatusNI: *mut RfuSlotStatusNI = null_mut();
    let mut imeBak: u16 = 0;
    if slotStatusIndex >= RFU_CHILD_MAX {
        return ERR_SLOT_NO;
    }
    if connType == 0x20 {
        slotStatusNI = gRfuSlotStatusNI[slotStatusIndex];
        if (*slotStatusNI).send.state as i32 & SLOT_BUSY_FLAG != 0
            && (*slotStatusNI).send.state as i32 & SLOT_SEND_FLAG != 0
        {
            connType = bmNewTgtSlot ^ (*slotStatusNI).send.bmSlot;
            if connType as i32 & bmNewTgtSlot as i32 == 0 {
                if connType != 0 {
                    imeBak = (67109384_usize as *mut u16).read_volatile();
                    volatile_write(67109384_usize as *mut u16, 0);
                    for i in 0..RFU_CHILD_MAX {
                        if shr_i32(connType as i32, i as u32) & 1 != 0 {
                            (*gRfuSlotStatusNI[i]).send.failCounter = 0;
                        }
                    }
                    (*gRfuLinkStatus).sendSlotNIFlag &= !connType;
                    (*slotStatusNI).send.bmSlot = bmNewTgtSlot;
                    if (*slotStatusNI).send.bmSlot == 0 {
                        rfu_STC_releaseFrame(slotStatusIndex, 0, &raw mut (*slotStatusNI).send);
                        (*slotStatusNI).send.state = SLOT_STATE_SEND_FAILED;
                    }
                    volatile_write(67109384_usize as *mut u16, imeBak);
                }
            } else {
                return ERR_SLOT_TARGET;
            }
        } else {
            return ERR_SLOT_NOT_SENDING;
        }
    } else {
        if connType == 16 {
            if (*gRfuSlotStatusUNI[slotStatusIndex]).send.state != SLOT_STATE_SEND_UNI {
                return ERR_SLOT_NOT_SENDING;
            }
            let mut bmSlot: i32 = 0;
            for i in 0..RFU_CHILD_MAX {
                if i != slotStatusIndex {
                    bmSlot |= (*gRfuSlotStatusUNI[i]).send.bmSlot as i32;
                }
            }
            if bmNewTgtSlot as i32 & bmSlot != 0 {
                return ERR_SLOT_TARGET;
            }
            imeBak = (67109384_usize as *mut u16).read_volatile();
            volatile_write(67109384_usize as *mut u16, 0);
            (*gRfuLinkStatus).sendSlotUNIFlag &= !(*gRfuSlotStatusUNI[slotStatusIndex]).send.bmSlot;
            (*gRfuLinkStatus).sendSlotUNIFlag |= bmNewTgtSlot;
            (*gRfuSlotStatusUNI[slotStatusIndex]).send.bmSlot = bmNewTgtSlot;
            volatile_write(67109384_usize as *mut u16, imeBak);
        } else {
            return ERR_COMM_TYPE;
        }
    }
    0
}
pub unsafe fn rfu_NI_stopReceivingData(slotStatusIndex: u8) -> u16 {
    if slotStatusIndex >= RFU_CHILD_MAX {
        return ERR_SLOT_NO;
    }
    let NI_comm: *mut NIComm = &raw mut (*gRfuSlotStatusNI[slotStatusIndex]).recv;
    let imeBak: u16 = (67109384_usize as *mut u16).read_volatile();
    volatile_write(67109384_usize as *mut u16, 0);
    if (*NI_comm).state as i32 & SLOT_BUSY_FLAG != 0 {
        if (*NI_comm).state == SLOT_STATE_RECV_LAST {
            (*NI_comm).state = SLOT_STATE_RECV_SUCCESS_AND_SENDSIDE_UNKNOWN;
        } else {
            (*NI_comm).state = SLOT_STATE_RECV_FAILED;
        }
        (*gRfuLinkStatus).recvSlotNIFlag &= !(shl_i32(1, slotStatusIndex as u32) as u8);
        rfu_STC_releaseFrame(slotStatusIndex, 1, NI_comm);
    }
    volatile_write(67109384_usize as *mut u16, imeBak);
    0
}
pub unsafe fn rfu_UNI_changeAndReadySendData(
    slotStatusIndex: u8,
    src: *mut c_void,
    size: u8,
) -> u16 {
    let mut frame_p: *mut u8 = null_mut();
    let mut frameEnd: u8 = 0;
    if slotStatusIndex >= RFU_CHILD_MAX {
        return ERR_SLOT_NO;
    }
    let UNI_send: *mut UNISend = &raw mut (*gRfuSlotStatusUNI[slotStatusIndex]).send;
    if (*UNI_send).state != SLOT_STATE_SEND_UNI {
        return ERR_SLOT_NOT_SENDING;
    }
    if (*gRfuLinkStatus).parentChild == MODE_PARENT {
        frame_p = &raw mut (*gRfuLinkStatus).remainLLFrameSizeParent;
        frameEnd = (*gRfuLinkStatus).remainLLFrameSizeParent + (*UNI_send).payloadSize as u8;
    } else {
        frame_p = &raw mut (*gRfuLinkStatus).remainLLFrameSizeChild[slotStatusIndex];
        frameEnd = (*gRfuLinkStatus).remainLLFrameSizeChild[slotStatusIndex]
            + (*UNI_send).payloadSize as u8;
    }
    if frameEnd < size {
        return ERR_SUBFRAME_SIZE;
    }
    let imeBak: u16 = (67109384_usize as *mut u16).read_volatile();
    volatile_write(67109384_usize as *mut u16, 0);
    (*UNI_send).src = src;
    *frame_p = frameEnd - size;
    (*UNI_send).payloadSize = size as u16;
    (*UNI_send).dataReadyFlag = 1;
    volatile_write(67109384_usize as *mut u16, imeBak);
    0
}
pub unsafe fn rfu_UNI_readySendData(slotStatusIndex: u8) {
    if slotStatusIndex < RFU_CHILD_MAX
        && (*gRfuSlotStatusUNI[slotStatusIndex]).send.state == SLOT_STATE_SEND_UNI
    {
        (*gRfuSlotStatusUNI[slotStatusIndex]).send.dataReadyFlag = 1;
    }
}
pub unsafe fn rfu_UNI_clearRecvNewDataFlag(slotStatusIndex: u8) {
    if slotStatusIndex < RFU_CHILD_MAX {
        (*gRfuSlotStatusUNI[slotStatusIndex]).recv.newDataFlag = 0;
    }
}
pub unsafe fn rfu_REQ_sendData(clockChangeFlag: u8) {
    if (*gRfuLinkStatus).parentChild != MODE_NEUTRAL {
        if (*gRfuLinkStatus).parentChild == MODE_PARENT
            && (*gRfuLinkStatus).sendSlotNIFlag as i32
                | (*gRfuLinkStatus).recvSlotNIFlag as i32
                | (*gRfuLinkStatus).sendSlotUNIFlag as i32
                == 0
        {
            if (*gRfuStatic).commExistFlag != 0 {
                (*gRfuStatic).emberCount = 16;
                (*gRfuStatic).nullFrameCount = 0;
            }
            if (*gRfuStatic).emberCount != 0 {
                (*gRfuStatic).emberCount -= 1;
            } else {
                (*gRfuStatic).nullFrameCount += 1;
            }
            if (*gRfuStatic).emberCount != 0 || (*gRfuStatic).nullFrameCount as i32 & 0xF == 0 {
                (*gRfuFixed).LLFBuffer[0] = 1;
                (*gRfuFixed).LLFBuffer[4] = 0xFF;
                STWI_set_Callback_M(core::mem::transmute::<
                    Option<unsafe fn(u8, u16)>,
                    *mut c_void,
                >(Some(rfu_CB_sendData3)));
                if clockChangeFlag == 0 {
                    STWI_send_DataTxREQ((*gRfuFixed).LLFBuffer.as_mut_ptr() as *mut c_void, 1);
                } else {
                    STWI_send_DataTxAndChangeREQ(
                        (*gRfuFixed).LLFBuffer.as_mut_ptr() as *mut c_void,
                        1,
                    );
                }
                return;
            }
        } else {
            if (&raw mut (*gRfuLinkStatus).LLFReadyFlag).read_volatile() == 0 {
                rfu_constructSendLLFrame();
            }
            if (&raw mut (*gRfuLinkStatus).LLFReadyFlag).read_volatile() != 0 {
                STWI_set_Callback_M(core::mem::transmute::<
                    Option<unsafe fn(u8, u16)>,
                    *mut c_void,
                >(Some(rfu_CB_sendData)));
                if clockChangeFlag != 0 {
                    STWI_send_DataTxAndChangeREQ(
                        (*gRfuFixed).LLFBuffer.as_mut_ptr() as *mut c_void,
                        (*gRfuStatic).totalPacketSize as u8 + 4,
                    );
                    return;
                }
                STWI_send_DataTxREQ(
                    (*gRfuFixed).LLFBuffer.as_mut_ptr() as *mut c_void,
                    (*gRfuStatic).totalPacketSize as u8 + 4,
                );
            }
        }
        if clockChangeFlag != 0 {
            if (*gRfuLinkStatus).parentChild == MODE_PARENT {
                if (*gSTWIStatus).callbackS.is_some() {
                    (*gSTWIStatus).callbackS.unwrap_unchecked()(39);
                }
            } else {
                STWI_set_Callback_M(core::mem::transmute::<
                    Option<unsafe fn(u8, u16)>,
                    *mut c_void,
                >(Some(rfu_CB_sendData2)));
                STWI_send_MS_ChangeREQ();
            }
        }
    }
}
pub(crate) unsafe fn rfu_CB_sendData(reqCommand: u8, reqResult: u16) {
    let mut NI_comm: *mut NIComm = null_mut();
    if reqResult == 0 {
        for i in 0..RFU_CHILD_MAX {
            if (*gRfuSlotStatusUNI[i]).send.dataReadyFlag != 0 {
                (*gRfuSlotStatusUNI[i]).send.dataReadyFlag = 0;
            }
            NI_comm = &raw mut (*gRfuSlotStatusNI[i]).send;
            if (*NI_comm).state == SLOT_STATE_SEND_NULL {
                rfu_STC_releaseFrame(i, 0, NI_comm);
                (*gRfuLinkStatus).sendSlotNIFlag &= !(*NI_comm).bmSlot;
                if (*NI_comm).dataType == 1 {
                    (*gRfuLinkStatus).getNameFlag |= shl_i32(1, i as u32) as u8;
                }
                (*NI_comm).state = SLOT_STATE_SEND_SUCCESS;
            }
        }
    }
    volatile_write(&raw mut (*gRfuLinkStatus).LLFReadyFlag, 0);
    rfu_STC_REQ_callback(ID_DATA_TX_REQ, reqResult);
}
pub(crate) unsafe fn rfu_CB_sendData2(reqCommand: u8, reqResult: u16) {
    rfu_STC_REQ_callback(ID_DATA_TX_REQ, reqResult);
}
pub(crate) unsafe fn rfu_CB_sendData3(reqCommand: u8, reqResult: u16) {
    if reqResult != 0 {
        rfu_STC_REQ_callback(ID_DATA_TX_REQ, reqResult);
    } else if reqCommand == ID_CLOCK_SLAVE_MS_CHANGE_ERROR_BY_DMA_REQ {
        rfu_STC_REQ_callback(ID_CLOCK_SLAVE_MS_CHANGE_ERROR_BY_DMA_REQ, 0);
    }
}
unsafe fn rfu_constructSendLLFrame() {
    let mut pakcketSize: u32 = 0;
    let mut currSize: u32 = 0;
    let mut llf_p: *mut u8 = null_mut();
    if (*gRfuLinkStatus).parentChild != MODE_NEUTRAL
        && (*gRfuLinkStatus).sendSlotNIFlag as i32
            | (*gRfuLinkStatus).recvSlotNIFlag as i32
            | (*gRfuLinkStatus).sendSlotUNIFlag as i32
            != 0
    {
        volatile_write(&raw mut (*gRfuLinkStatus).LLFReadyFlag, 0);
        pakcketSize = 0;
        llf_p = &raw mut (*gRfuFixed).LLFBuffer[1] as *mut u8;
        for i in 0..RFU_CHILD_MAX {
            currSize = 0;
            if (*gRfuSlotStatusNI[i]).send.state as i32 & SLOT_BUSY_FLAG != 0 {
                currSize = rfu_STC_NI_constructLLSF(
                    i,
                    &raw mut llf_p,
                    &raw mut (*gRfuSlotStatusNI[i]).send,
                ) as u32;
            }
            if (*gRfuSlotStatusNI[i]).recv.state as i32 & SLOT_BUSY_FLAG != 0 {
                currSize += rfu_STC_NI_constructLLSF(
                    i,
                    &raw mut llf_p,
                    &raw mut (*gRfuSlotStatusNI[i]).recv,
                ) as u32;
            }
            if (*gRfuSlotStatusUNI[i]).send.state == SLOT_STATE_SEND_UNI {
                currSize += rfu_STC_UNI_constructLLSF(i, &raw mut llf_p) as u32;
            }
            if currSize != 0 {
                if (*gRfuLinkStatus).parentChild == MODE_PARENT {
                    pakcketSize += currSize;
                } else {
                    pakcketSize |= shl_u32(currSize, 5 * i as u32 + 8);
                }
            }
        }
        if pakcketSize != 0 {
            while llf_p as usize as u32 & 3 != 0 {
                *({
                    let t1 = llf_p;
                    llf_p = llf_p.at(1);
                    t1
                }) = 0;
            }
            (*gRfuFixed).LLFBuffer[0] = pakcketSize;
            if (*gRfuLinkStatus).parentChild == MODE_CHILD {
                let maxSize: *mut u8 = llf_p.at(-108);
                pakcketSize = (maxSize as usize)
                    .wrapping_sub((&raw mut gRfuFixed as *mut *mut u8).read_volatile() as usize)
                    as i32 as u32;
            }
        }
        (*gRfuStatic).totalPacketSize = pakcketSize;
    }
}
unsafe fn rfu_STC_NI_constructLLSF(
    bm_slot_id: u8,
    dest_pp: *mut *mut u8,
    NI_comm: *mut NIComm,
) -> u16 {
    let mut size: u16 = 0;
    let llsf: *mut LLSFStruct = (&raw const llsf_struct[(*gRfuLinkStatus).parentChild]).cast_mut();
    if (*NI_comm).state == SLOT_STATE_SENDING {
        while (*NI_comm).now_p[(*NI_comm).phase]
            >= ((*NI_comm).src as *mut u8).at((*NI_comm).dataSize)
        {
            (*NI_comm).phase += 1;
            if (*NI_comm).phase == 4 {
                (*NI_comm).phase = 0;
            }
        }
    }
    if (*NI_comm).state as i32 & SLOT_RECV_FLAG != 0 {
        size = 0;
    } else if (*NI_comm).state == SLOT_STATE_SENDING {
        if (*NI_comm).now_p[(*NI_comm).phase].at((*NI_comm).payloadSize)
            > ((*NI_comm).src as *mut u8).at((*NI_comm).dataSize)
        {
            size = (((*NI_comm).src as *mut u8).at((*NI_comm).dataSize) as usize)
                .wrapping_sub((*NI_comm).now_p[(*NI_comm).phase] as usize) as i32
                as u16;
        } else {
            size = (*NI_comm).payloadSize;
        }
    } else {
        if (*NI_comm).remainSize >= (*NI_comm).payloadSize as u32 {
            size = (*NI_comm).payloadSize;
        } else {
            size = (*NI_comm).remainSize as u16;
        }
    }
    let mut frame: u32 = shl_i32((*NI_comm).state as i32 & 0xF, (*llsf).slotStateShift as u32)
        as u32
        | shl_i32((*NI_comm).ack as i32, (*llsf).ackShift as u32) as u32
        | shl_i32((*NI_comm).phase as i32, (*llsf).phaseShift as u32) as u32
        | shl_i32((*NI_comm).n[(*NI_comm).phase] as i32, (*llsf).nShift as u32) as u32
        | size as u32;
    if (*gRfuLinkStatus).parentChild == MODE_PARENT {
        frame |= ((*NI_comm).bmSlot as u32) << 18;
    }
    let mut frame8_p: *mut u8 = &raw mut frame as *mut u8;
    for i in 0..(*llsf).frameSize {
        *({
            let t1 = *dest_pp;
            *dest_pp = (*dest_pp).at(1);
            t1
        }) = *({
            let t3 = frame8_p;
            frame8_p = frame8_p.at(1);
            t3
        });
    }
    if size != 0 {
        let mut src: *mut u8 = (*NI_comm).now_p[(*NI_comm).phase];
        (*gRfuFixed).fastCopyPtr.unwrap_unchecked()(&raw mut src, dest_pp, size as i32);
    }
    if (*NI_comm).state == SLOT_STATE_SENDING {
        (*NI_comm).phase += 1;
        if (*NI_comm).phase == 4 {
            (*NI_comm).phase = 0;
        }
    }
    if (*gRfuLinkStatus).parentChild == MODE_PARENT {
        volatile_write(&raw mut (*gRfuLinkStatus).LLFReadyFlag, 1);
    } else {
        volatile_write(
            &raw mut (*gRfuLinkStatus).LLFReadyFlag,
            (&raw mut (*gRfuLinkStatus).LLFReadyFlag).read_volatile()
                | shl_i32(1, bm_slot_id as u32) as u8,
        );
    }
    size + (*llsf).frameSize as u16
}
unsafe fn rfu_STC_UNI_constructLLSF(bm_slot_id: u8, dest_p: *mut *mut u8) -> u16 {
    let UNI_send: *mut UNISend = &raw mut (*gRfuSlotStatusUNI[bm_slot_id]).send;
    if (*UNI_send).dataReadyFlag == 0 || (*UNI_send).bmSlot == 0 {
        return 0;
    }
    let llsf: *mut LLSFStruct = (&raw const llsf_struct[(*gRfuLinkStatus).parentChild]).cast_mut();
    let mut frame: u32 = shl_i32(
        (*UNI_send).state as i32 & 0xF,
        (*llsf).slotStateShift as u32,
    ) as u32
        | (*UNI_send).payloadSize as u32;
    if (*gRfuLinkStatus).parentChild == MODE_PARENT {
        frame |= ((*UNI_send).bmSlot as u32) << 18;
    }
    let mut frame8_p: *mut u8 = &raw mut frame as *mut u8;
    for i in 0..(*llsf).frameSize {
        *({
            let t1 = *dest_p;
            *dest_p = (*dest_p).at(1);
            t1
        }) = *({
            let t3 = frame8_p;
            frame8_p = frame8_p.at(1);
            t3
        });
    }
    let mut src_p: *mut u8 = (*UNI_send).src as *mut u8;
    (*gRfuFixed).fastCopyPtr.unwrap_unchecked()(
        &raw mut src_p,
        dest_p,
        (*UNI_send).payloadSize as i32,
    );
    if (*gRfuLinkStatus).parentChild == MODE_PARENT {
        volatile_write(&raw mut (*gRfuLinkStatus).LLFReadyFlag, 16);
    } else {
        volatile_write(
            &raw mut (*gRfuLinkStatus).LLFReadyFlag,
            (&raw mut (*gRfuLinkStatus).LLFReadyFlag).read_volatile()
                | shl_i32(16, bm_slot_id as u32) as u8,
        );
    }
    (*llsf).frameSize as u16 + (*UNI_send).payloadSize
}
pub unsafe fn rfu_REQ_recvData() {
    if (*gRfuLinkStatus).parentChild != MODE_NEUTRAL {
        (*gRfuStatic).commExistFlag = (*gRfuLinkStatus).sendSlotNIFlag
            | (*gRfuLinkStatus).recvSlotNIFlag
            | (*gRfuLinkStatus).sendSlotUNIFlag;
        (*gRfuStatic).recvErrorFlag = 0;
        STWI_set_Callback_M(core::mem::transmute::<
            Option<unsafe fn(u8, u16)>,
            *mut c_void,
        >(Some(rfu_CB_recvData)));
        STWI_send_DataRxREQ();
    }
}
pub(crate) unsafe fn rfu_CB_recvData(reqCommand: u8, mut reqResult: u16) {
    let mut slotStatusNI: *mut RfuSlotStatusNI = null_mut();
    let mut NI_comm: *mut NIComm = null_mut();
    if reqResult == 0 && (*(*gRfuFixed).STWIBuffer).rxPacketAlloc.rfuPacket8.data[1] != 0 {
        (*gRfuStatic).NIEndRecvFlag = 0;
        if (*gRfuLinkStatus).parentChild == MODE_PARENT {
            rfu_STC_PARENT_analyzeRecvPacket();
        } else {
            rfu_STC_CHILD_analyzeRecvPacket();
        }
        for i in 0..RFU_CHILD_MAX {
            slotStatusNI = gRfuSlotStatusNI[i];
            if (*slotStatusNI).recv.state == SLOT_STATE_RECV_LAST
                && shr_i32((*gRfuStatic).NIEndRecvFlag as i32, i as u32) & 1 == 0
            {
                NI_comm = &raw mut (*slotStatusNI).recv;
                if (*NI_comm).dataType == 1 {
                    (*gRfuLinkStatus).getNameFlag |= shl_i32(1, i as u32) as u8;
                }
                rfu_STC_releaseFrame(i, 1, NI_comm);
                (*gRfuLinkStatus).recvSlotNIFlag &= !(*NI_comm).bmSlot;
                (*slotStatusNI).recv.state = SLOT_STATE_RECV_SUCCESS;
            }
        }
        if (*gRfuStatic).recvErrorFlag != 0 {
            reqResult = (*gRfuStatic).recvErrorFlag as u16 | ERR_DATA_RECV;
        }
    }
    rfu_STC_REQ_callback(reqCommand, reqResult);
}
unsafe fn rfu_STC_PARENT_analyzeRecvPacket() {
    let mut frame_counts: CArray<u8, 4> = zeroed();
    let mut frames32: u32 = (*(*gRfuFixed).STWIBuffer).rxPacketAlloc.rfuPacket32.data[0] >> 8;
    let mut bm_slot_id: u8 = 0;
    while bm_slot_id < RFU_CHILD_MAX {
        frame_counts[bm_slot_id] = frames32 as u8 & 0x1F;
        frames32 >>= 5;
        if frame_counts[bm_slot_id] == 0 {
            (*gRfuStatic).NIEndRecvFlag |= shl_i32(1, bm_slot_id as u32) as u8;
        }
        bm_slot_id += 1;
    }
    let mut packet_p: *mut u8 =
        &raw mut (*(*gRfuFixed).STWIBuffer).rxPacketAlloc.rfuPacket8.data[8];
    for bm_slot_id in 0..RFU_CHILD_MAX {
        if frame_counts[bm_slot_id] != 0 {
            let frames_p: *mut u8 = &raw mut frame_counts[bm_slot_id];
            loop {
                let analyzed_frames: u8 =
                    rfu_STC_analyzeLLSF(bm_slot_id, packet_p, *frames_p as u16) as u8;
                packet_p = packet_p.at(analyzed_frames);
                *frames_p -= analyzed_frames;
                if !(*frames_p as i32 & 0x80 == 0 && *frames_p != 0) {
                    break;
                }
            }
        }
    }
}
unsafe fn rfu_STC_CHILD_analyzeRecvPacket() {
    let mut analyzed_frames: u16 = 0;
    let mut frames_remaining: u16 =
        *(&raw mut (*(*gRfuFixed).STWIBuffer).rxPacketAlloc.rfuPacket8.data[4] as *mut u16) & 0x7F;
    let mut packet_p: *mut u8 =
        &raw mut (*(*gRfuFixed).STWIBuffer).rxPacketAlloc.rfuPacket8.data[8];
    if frames_remaining == 0 {
        (*gRfuStatic).NIEndRecvFlag = 15;
    }
    loop {
        if frames_remaining == 0 {
            break;
        }
        analyzed_frames = rfu_STC_analyzeLLSF(0, packet_p, frames_remaining);
        packet_p = packet_p.at(analyzed_frames);
        frames_remaining -= analyzed_frames;
        if frames_remaining as i32 & 0x8000 != 0 {
            break;
        }
    }
}
unsafe fn rfu_STC_analyzeLLSF(slot_id: u8, mut src: *mut u8, last_frame: u16) -> u16 {
    let mut llsf_NI: RfuLocalStruct = zeroed();
    let llsf_p: *mut LLSFStruct = (&raw const llsf_struct
        [!((*gRfuLinkStatus).parentChild as i32) & MODE_PARENT as i32])
        .cast_mut();
    if last_frame < (*llsf_p).frameSize as u16 {
        return last_frame;
    }
    let mut frames: u32 = 0;
    let mut i: u8 = 0;
    while i < (*llsf_p).frameSize {
        frames |= shl_i32(
            *({
                let t2 = src;
                src = src.at(1);
                t2
            }) as i32,
            8 * i as u32,
        ) as u32;
        i += 1;
    }
    llsf_NI.recvFirst =
        shr_u32(frames, (*llsf_p).recvFirstShift as u32) as u8 & (*llsf_p).recvFirstMask;
    llsf_NI.connSlotFlag =
        shr_u32(frames, (*llsf_p).connSlotFlagShift as u32) as u8 & (*llsf_p).connSlotFlagMask;
    llsf_NI.slotState =
        shr_u32(frames, (*llsf_p).slotStateShift as u32) as u8 & (*llsf_p).slotStateMask;
    llsf_NI.ack = shr_u32(frames, (*llsf_p).ackShift as u32) as u8 & (*llsf_p).ackMask;
    llsf_NI.phase = shr_u32(frames, (*llsf_p).phaseShift as u32) as u8 & (*llsf_p).phaseMask;
    llsf_NI.n = shr_u32(frames, (*llsf_p).nShift as u32) as u8 & (*llsf_p).nMask;
    llsf_NI.frame = frames as u16 & (*llsf_p).framesMask & frames as u16;
    let retVal: u16 = llsf_NI.frame + (*llsf_p).frameSize as u16;
    if llsf_NI.recvFirst == 0 {
        if (*gRfuLinkStatus).parentChild == MODE_PARENT {
            if shr_i32((*gRfuLinkStatus).connSlotFlag as i32, slot_id as u32) & 1 != 0 {
                if llsf_NI.slotState == LCOM_UNI {
                    rfu_STC_UNI_receive(slot_id, &raw mut llsf_NI, src);
                } else if llsf_NI.ack == 0 {
                    rfu_STC_NI_receive_Receiver(slot_id, &raw mut llsf_NI, src);
                } else {
                    i = 0;
                    while i < RFU_CHILD_MAX {
                        if shr_i32((*gRfuSlotStatusNI[i]).send.bmSlot as i32, slot_id as u32) & 1
                            != 0
                            && shr_i32((*gRfuLinkStatus).sendSlotNIFlag as i32, slot_id as u32) & 1
                                != 0
                        {
                            break;
                        }
                        i += 1;
                    }
                    if i < RFU_CHILD_MAX {
                        rfu_STC_NI_receive_Sender(i, slot_id, &raw mut llsf_NI, src);
                    }
                }
            }
        } else {
            let conSlots: i32 = (*gRfuLinkStatus).connSlotFlag as i32 & llsf_NI.connSlotFlag as i32;
            if conSlots != 0 {
                for i in 0..RFU_CHILD_MAX {
                    if shr_i32(conSlots, i as u32) & 1 != 0 {
                        if llsf_NI.slotState == LCOM_UNI {
                            rfu_STC_UNI_receive(i, &raw mut llsf_NI, src);
                        } else if llsf_NI.ack == 0 {
                            rfu_STC_NI_receive_Receiver(i, &raw mut llsf_NI, src);
                        } else if shr_i32((*gRfuLinkStatus).sendSlotNIFlag as i32, i as u32) & 1
                            != 0
                        {
                            rfu_STC_NI_receive_Sender(i, i, &raw mut llsf_NI, src);
                        }
                    }
                }
            }
        }
    }
    retVal
}
unsafe fn rfu_STC_UNI_receive(bm_slot_id: u8, llsf_NI: *mut RfuLocalStruct, mut src: *mut u8) {
    let mut dest: *mut u8 = null_mut();
    let mut size: u32 = 0;
    let slotStatusUNI: *mut RfuSlotStatusUNI = gRfuSlotStatusUNI[bm_slot_id];
    let UNI_recv: *mut UNIRecv = &raw mut (*slotStatusUNI).recv;
    (*UNI_recv).errorCode = 0;
    'force_tail_merge: {
        if (*gRfuSlotStatusUNI[bm_slot_id]).recvBufferSize < (*llsf_NI).frame as u32 {
            (*slotStatusUNI).recv.state = SLOT_STATE_RECV_IGNORE;
            (*UNI_recv).errorCode = ERR_RECV_BUFF_OVER;
        } else {
            if (*UNI_recv).dataBlockFlag != 0 {
                if (*UNI_recv).newDataFlag != 0 {
                    (*UNI_recv).errorCode = ERR_RECV_UNK;
                    break 'force_tail_merge;
                }
            } else {
                if (*UNI_recv).newDataFlag != 0 {
                    (*UNI_recv).errorCode = ERR_RECV_DATA_OVERWRITED;
                }
            }
            (*UNI_recv).state = SLOT_STATE_RECEIVING;
            size = ({
                (*UNI_recv).dataSize = (*llsf_NI).frame;
                (*UNI_recv).dataSize
            }) as u32;
            dest = (*gRfuSlotStatusUNI[bm_slot_id]).recvBuffer as *mut u8;
            (*gRfuFixed).fastCopyPtr.unwrap_unchecked()(&raw mut src, &raw mut dest, size as i32);
            (*UNI_recv).newDataFlag = 1;
            (*UNI_recv).state = 0;
        }
    }
    if (*UNI_recv).errorCode != 0 {
        (*gRfuStatic).recvErrorFlag |= shl_i32(16, bm_slot_id as u32) as u8;
    }
}
unsafe fn rfu_STC_NI_receive_Sender(
    NI_slot: u8,
    bm_flag: u8,
    llsf_NI: *mut RfuLocalStruct,
    data_p: *mut u8,
) {
    let NI_comm: *mut NIComm = &raw mut (*gRfuSlotStatusNI[NI_slot]).send;
    let state: u16 = (*NI_comm).state;
    let n: u8 = (*NI_comm).n[(*llsf_NI).phase];
    let mut imeBak: u16 = 0;
    if ((*llsf_NI).slotState == LCOM_NI && state == SLOT_STATE_SENDING
        || (*llsf_NI).slotState == LCOM_NI_START && state == SLOT_STATE_SEND_START
        || (*llsf_NI).slotState == LCOM_NI_END && state == SLOT_STATE_SEND_LAST)
        && (*NI_comm).n[(*llsf_NI).phase] == (*llsf_NI).n
    {
        (*NI_comm).recvAckFlag[(*llsf_NI).phase] |= shl_i32(1, bm_flag as u32) as u8;
    }
    if (*NI_comm).recvAckFlag[(*llsf_NI).phase] as i32 & (*NI_comm).bmSlot as i32
        == (*NI_comm).bmSlot as i32
    {
        (*NI_comm).n[(*llsf_NI).phase] = ((*NI_comm).n[(*llsf_NI).phase] + 1) & 3;
        (*NI_comm).recvAckFlag[(*llsf_NI).phase] = 0;
        if (*NI_comm).state as i32 + 32735 <= 1 {
            if (*NI_comm).state == SLOT_STATE_SEND_START {
                (*NI_comm).now_p[(*llsf_NI).phase] =
                    (*NI_comm).now_p[(*llsf_NI).phase].at((*NI_comm).payloadSize);
            } else {
                (*NI_comm).now_p[(*llsf_NI).phase] =
                    (*NI_comm).now_p[(*llsf_NI).phase].at(((*NI_comm).payloadSize as i32) << 2);
            }
            (*NI_comm).remainSize -= (*NI_comm).payloadSize as u32;
            match (*NI_comm).remainSize {
                1..=0x7fffffff => {}
                _ => {
                    (*NI_comm).phase = 0;
                    if (*NI_comm).state == SLOT_STATE_SEND_START {
                        for i in 0..WINDOW_COUNT {
                            (*NI_comm).n[i] = 1;
                            (*NI_comm).now_p[i] = ((*NI_comm).src as *mut u8)
                                .at((*NI_comm).payloadSize as i32 * i as i32)
                                as *mut c_void
                                as *mut u8;
                        }
                        (*NI_comm).remainSize = (*NI_comm).dataSize;
                        (*NI_comm).state = SLOT_STATE_SENDING;
                    } else {
                        (*NI_comm).n[0] = 0;
                        (*NI_comm).remainSize = 0;
                        (*NI_comm).state = SLOT_STATE_SEND_LAST;
                    }
                }
            }
        } else if (*NI_comm).state == SLOT_STATE_SEND_LAST {
            (*NI_comm).state = SLOT_STATE_SEND_NULL;
        }
    }
    if (*NI_comm).state != state
        || (*NI_comm).n[(*llsf_NI).phase] != n
        || shr_i32(
            (*NI_comm).recvAckFlag[(*llsf_NI).phase] as i32,
            bm_flag as u32,
        ) & 1
            != 0
    {
        imeBak = (67109384_usize as *mut u16).read_volatile();
        volatile_write(67109384_usize as *mut u16, 0);
        (*gRfuStatic).recvRenewalFlag |= shl_i32(16, bm_flag as u32) as u8;
        (*gRfuSlotStatusNI[bm_flag]).send.failCounter = 0;
        volatile_write(67109384_usize as *mut u16, imeBak);
    }
}
unsafe fn rfu_STC_NI_receive_Receiver(
    bm_slot_id: u8,
    llsf_NI: *mut RfuLocalStruct,
    mut data_p: *mut u8,
) {
    let mut imeBak: u16 = 0;
    let mut state_check: u32 = 0;
    let slotStatus_NI: *mut RfuSlotStatusNI = gRfuSlotStatusNI[bm_slot_id];
    let recvSlot: *mut NIComm = &raw mut (*slotStatus_NI).recv;
    let state: u16 = (*slotStatus_NI).recv.state;
    let n: u8 = (*slotStatus_NI).recv.n[(*llsf_NI).phase];
    if (*llsf_NI).slotState == LCOM_NI_END {
        (*gRfuStatic).NIEndRecvFlag |= shl_i32(1, bm_slot_id as u32) as u8;
        if (*slotStatus_NI).recv.state == SLOT_STATE_RECEIVING {
            (*slotStatus_NI).recv.phase = 0;
            (*slotStatus_NI).recv.n[0] = 0;
            (*slotStatus_NI).recv.state = SLOT_STATE_RECV_LAST;
        }
    } else if (*llsf_NI).slotState == LCOM_NI {
        if state == SLOT_STATE_RECV_START && (*recvSlot).remainSize == 0 {
            rfu_STC_NI_initSlot_asRecvDataEntity(bm_slot_id, recvSlot);
        }
        if (*recvSlot).state == SLOT_STATE_RECEIVING {
            state_check = 1;
        }
    } else if (*llsf_NI).slotState == LCOM_NI_START {
        if state == SLOT_STATE_RECV_START {
            state_check = 1;
        } else {
            rfu_STC_NI_initSlot_asRecvControllData(bm_slot_id, recvSlot);
            if (*slotStatus_NI).recv.state != SLOT_STATE_RECV_START {
                return;
            }
            state_check = 1;
        }
    }
    if state_check != 0 && (*llsf_NI).n as i32 == ((*recvSlot).n[(*llsf_NI).phase] as i32 + 1) & 3 {
        (*gRfuFixed).fastCopyPtr.unwrap_unchecked()(
            &raw mut data_p,
            &raw mut (*recvSlot).now_p[(*llsf_NI).phase],
            (*llsf_NI).frame as i32,
        );
        if (*recvSlot).state == SLOT_STATE_RECEIVING {
            (*recvSlot).now_p[(*llsf_NI).phase] =
                (*recvSlot).now_p[(*llsf_NI).phase].at(3 * (*recvSlot).payloadSize as i32);
        }
        (*recvSlot).remainSize -= (*llsf_NI).frame as u32;
        (*recvSlot).n[(*llsf_NI).phase] = (*llsf_NI).n;
    }
    if (*recvSlot).errorCode == 0 {
        (*recvSlot).phase = (*llsf_NI).phase;
        if (*recvSlot).state != state
            || (*recvSlot).n[(*llsf_NI).phase] != n
            || (*recvSlot).n[(*llsf_NI).phase] == (*llsf_NI).n
        {
            imeBak = (67109384_usize as *mut u16).read_volatile();
            volatile_write(67109384_usize as *mut u16, 0);
            (*gRfuStatic).recvRenewalFlag |= shl_i32(1, bm_slot_id as u32) as u8;
            (*recvSlot).failCounter = 0;
            volatile_write(67109384_usize as *mut u16, imeBak);
        }
    }
}
unsafe fn rfu_STC_NI_initSlot_asRecvControllData(bm_slot_id: u8, NI_comm: *mut NIComm) {
    let mut llFrameSize_p: *mut u8 = null_mut();
    let mut llFrameSize: u32 = 0;
    if (*gRfuLinkStatus).parentChild == MODE_PARENT {
        llFrameSize = 3;
        llFrameSize_p = &raw mut (*gRfuLinkStatus).remainLLFrameSizeParent;
    } else {
        llFrameSize = 2;
        llFrameSize_p = &raw mut (*gRfuLinkStatus).remainLLFrameSizeChild[bm_slot_id];
    }
    let bm_slot_flag: u8 = shl_i32(1, bm_slot_id as u32) as u8;
    if (*NI_comm).state == 0 {
        if (*llFrameSize_p as u32) < llFrameSize {
            (*NI_comm).state = SLOT_STATE_RECV_IGNORE;
            (*NI_comm).errorCode = ERR_RECV_REPLY_SUBFRAME_SIZE;
            (*gRfuStatic).recvErrorFlag |= bm_slot_flag;
        } else {
            (*NI_comm).errorCode = 0;
            *llFrameSize_p -= llFrameSize as u8;
            (*NI_comm).now_p[0] = &raw mut (*NI_comm).dataType;
            (*NI_comm).remainSize = 7;
            (*NI_comm).ack = 1;
            (*NI_comm).payloadSize = 0;
            (*NI_comm).bmSlot = bm_slot_flag;
            (*NI_comm).state = SLOT_STATE_RECV_START;
            (*gRfuLinkStatus).recvSlotNIFlag |= bm_slot_flag;
        }
    }
}
unsafe fn rfu_STC_NI_initSlot_asRecvDataEntity(bm_slot_id: u8, NI_comm: *mut NIComm) {
    let mut bm_slot_flag: u8 = 0;
    if (*NI_comm).dataType == 1 {
        (*NI_comm).now_p[0] =
            &raw mut (*gRfuLinkStatus).partner[bm_slot_id].serialNo as *mut c_void as *mut u8;
    } else {
        if (*NI_comm).dataSize > (*gRfuSlotStatusNI[bm_slot_id]).recvBufferSize {
            bm_slot_flag = shl_i32(1, bm_slot_id as u32) as u8;
            (*gRfuStatic).recvErrorFlag |= bm_slot_flag;
            (*gRfuLinkStatus).recvSlotNIFlag &= !bm_slot_flag;
            (*NI_comm).errorCode = ERR_RECV_BUFF_OVER;
            (*NI_comm).state = SLOT_STATE_RECV_FAILED;
            rfu_STC_releaseFrame(bm_slot_id, 1, NI_comm);
            return;
        }
        (*NI_comm).now_p[0] = (*gRfuSlotStatusNI[bm_slot_id]).recvBuffer as *mut u8;
    }
    for win_id in 0..WINDOW_COUNT {
        (*NI_comm).n[win_id] = 0;
        (*NI_comm).now_p[win_id] =
            (*NI_comm).now_p[0].at((*NI_comm).payloadSize as i32 * win_id as i32);
    }
    (*NI_comm).remainSize = (*NI_comm).dataSize;
    (*NI_comm).state = SLOT_STATE_RECEIVING;
}
unsafe fn rfu_NI_checkCommFailCounter() {
    let mut imeBak: u16 = 0;
    let mut recvRenewalFlag: u32 = 0;
    let mut bm_slot_flag: u8 = 0;
    if (*gRfuLinkStatus).sendSlotNIFlag as i32 | (*gRfuLinkStatus).recvSlotNIFlag as i32 != 0 {
        imeBak = (67109384_usize as *mut u16).read_volatile();
        volatile_write(67109384_usize as *mut u16, 0);
        recvRenewalFlag = ((*gRfuStatic).recvRenewalFlag >> 4) as u32;
        for bm_slot_id in 0..RFU_CHILD_MAX {
            bm_slot_flag = shl_i32(1, bm_slot_id as u32) as u8;
            if (*gRfuLinkStatus).sendSlotNIFlag as i32 & bm_slot_flag as i32 != 0
                && (*gRfuStatic).recvRenewalFlag as i32 & bm_slot_flag as i32 == 0
            {
                (*gRfuSlotStatusNI[bm_slot_id]).send.failCounter += 1;
            }
            if (*gRfuLinkStatus).recvSlotNIFlag as i32 & bm_slot_flag as i32 != 0
                && recvRenewalFlag & bm_slot_flag as u32 == 0
            {
                (*gRfuSlotStatusNI[bm_slot_id]).recv.failCounter += 1;
            }
        }
        (*gRfuStatic).recvRenewalFlag = 0;
        volatile_write(67109384_usize as *mut u16, imeBak);
    }
}
pub unsafe fn rfu_REQ_noise() {
    STWI_set_Callback_M(core::mem::transmute::<
        Option<unsafe fn(u8, u16)>,
        *mut c_void,
    >(Some(rfu_STC_REQ_callback)));
    STWI_send_TestModeREQ(1, 0);
}
