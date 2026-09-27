//! Translated from `src/librfu_rfu.c` by tools/rustport/c2rs.py, then reviewed.
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

// Data tables (translate with cdata.py): llsf_struct version_string str_checkMbootLL
#[allow(unused_imports)]
use crate::data::librfu_rfu::*;

#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gRfuSlotStatusUNI: crate::ffi::Align4<[u8; 16]> = crate::ffi::Align4([0; 16]);
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gRfuSlotStatusNI: crate::ffi::Align4<[u8; 16]> = crate::ffi::Align4([0; 16]);
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gRfuLinkStatus: *mut u8 = core::ptr::null_mut();
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gRfuStatic: *mut u8 = core::ptr::null_mut();
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gRfuFixed: *mut u8 = core::ptr::null_mut();

unsafe extern "C" {
    static mut gSTWIStatus: u8;
    fn AgbRFU_SoftReset();
    fn AgbRFU_checkID(a0: u8) -> i32;
    fn CpuSet(a0: *mut u8, a1: *mut u8, a2: u32);
    fn Div(a0: i32, a1: i32) -> i32;
    fn STWI_init_all(a0: *mut u8, a1: *mut Option<unsafe extern "C" fn()>, a2: u8);
    fn STWI_init_timer(a0: *mut Option<unsafe extern "C" fn()>, a1: i32);
    fn STWI_poll_CommandEnd() -> u16;
    fn STWI_read_status(a0: u8) -> u16;
    fn STWI_send_CPR_EndREQ();
    fn STWI_send_CPR_PollingREQ();
    fn STWI_send_CPR_StartREQ(a0: u16, a1: u16, a2: u8);
    fn STWI_send_CP_EndREQ();
    fn STWI_send_CP_PollingREQ();
    fn STWI_send_CP_StartREQ(a0: u16);
    fn STWI_send_DataRxREQ();
    fn STWI_send_DataTxAndChangeREQ(a0: *mut u8, a1: u8);
    fn STWI_send_DataTxREQ(a0: *mut u8, a1: u8);
    fn STWI_send_DisconnectREQ(a0: u8);
    fn STWI_send_GameConfigREQ(a0: *mut u8, a1: *mut u8);
    fn STWI_send_LinkStatusREQ();
    fn STWI_send_MS_ChangeREQ();
    fn STWI_send_ResetREQ();
    fn STWI_send_ResumeRetransmitAndChangeREQ();
    fn STWI_send_SC_EndREQ();
    fn STWI_send_SC_PollingREQ();
    fn STWI_send_SC_StartREQ();
    fn STWI_send_SP_EndREQ();
    fn STWI_send_SP_PollingREQ();
    fn STWI_send_SP_StartREQ();
    fn STWI_send_SlotStatusREQ();
    fn STWI_send_StopModeREQ();
    fn STWI_send_SystemConfigREQ(a0: u16, a1: u8, a2: u8);
    fn STWI_send_SystemStatusREQ();
    fn STWI_send_TestModeREQ(a0: u8, a1: u8);
    fn STWI_set_Callback_M(a0: *mut u8);
    fn STWI_set_Callback_S(a0: Option<unsafe extern "C" fn(u16)>);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn rfu_initializeAPI(
    APIBuffer: *mut u32,
    buffByteSize: u16,
    sioIntrTable_p: *mut Option<unsafe extern "C" fn()>,
    copyInterruptToRam: u8,
) -> u16 {
    unsafe {
        let mut APIBuffer = APIBuffer;
        let mut buffByteSize = buffByteSize;
        let mut sioIntrTable_p = sioIntrTable_p;
        let mut copyInterruptToRam = copyInterruptToRam;
        let mut i: u16 = 0u16;
        let mut dst: *mut u16 = core::ptr::null_mut();
        let mut src: *mut u16 = core::ptr::null_mut();
        let mut buffByteSizeMax: u16 = 0u16;
        if ((((APIBuffer) as usize as u32) & 251658240u32) == 33554432u32)
            && ((copyInterruptToRam) != 0)
        {
            return 2u16;
        }
        if (((APIBuffer) as usize as u32) & 3u32) != 0 {
            return 2u16;
        }
        if (copyInterruptToRam) != 0 {
            buffByteSizeMax = 3684u16;
            if ((buffByteSize) as i32) < ((buffByteSizeMax) as i32) {
                return 1u16;
            }
        }
        if !((copyInterruptToRam) != 0) {
            buffByteSizeMax = 1284u16;
            if ((buffByteSize) as i32) < ((buffByteSizeMax) as i32) {
                return 1u16;
            }
        }
        ((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>()).write((APIBuffer).cast::<u8>());
        ((&raw mut gRfuStatic).cast::<u8>().cast::<*mut u8>())
            .write(((APIBuffer).cast::<u8>()).wrapping_offset(180));
        ((&raw mut gRfuFixed).cast::<u8>().cast::<*mut u8>())
            .write(((APIBuffer).cast::<u8>()).wrapping_offset(220));
        (((&raw mut gRfuSlotStatusNI).cast::<u8>().cast::<*mut u8>()).cast::<*mut u8>())
            .write(((APIBuffer).cast::<u8>()).wrapping_offset(444));
        (((&raw mut gRfuSlotStatusUNI).cast::<u8>().cast::<*mut u8>()).cast::<*mut u8>())
            .write(((APIBuffer).cast::<u8>()).wrapping_offset(892));
        {
            i = 1u16;
            'l1: loop {
                if !(((i) as i32) < 4i32) {
                    break 'l1;
                }
                'l2: {
                    ((((&raw mut gRfuSlotStatusNI).cast::<u8>().cast::<*mut u8>())
                        .cast::<*mut u8>())
                    .wrapping_offset(((i) as i32) as isize))
                    .write(
                        (((((&raw mut gRfuSlotStatusNI).cast::<u8>().cast::<*mut u8>())
                            .cast::<*mut u8>())
                        .wrapping_offset((((i) as i32).wrapping_sub(1i32)) as isize))
                        .read())
                        .wrapping_offset(112),
                    );
                    ((((&raw mut gRfuSlotStatusUNI).cast::<u8>().cast::<*mut u8>())
                        .cast::<*mut u8>())
                    .wrapping_offset(((i) as i32) as isize))
                    .write(
                        (((((&raw mut gRfuSlotStatusUNI).cast::<u8>().cast::<*mut u8>())
                            .cast::<*mut u8>())
                        .wrapping_offset((((i) as i32).wrapping_sub(1i32)) as isize))
                        .read())
                        .wrapping_offset(28),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        ((((&raw mut gRfuFixed).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(220)
            .cast::<*mut u8>())
        .write(
            (((((&raw mut gRfuSlotStatusUNI).cast::<u8>().cast::<*mut u8>()).cast::<*mut u8>())
                .wrapping_offset(3))
            .read())
            .wrapping_offset(28),
        );
        STWI_init_all(
            (((((&raw mut gRfuSlotStatusUNI).cast::<u8>().cast::<*mut u8>()).cast::<*mut u8>())
                .wrapping_offset(3))
            .read())
            .wrapping_offset(28),
            sioIntrTable_p,
            copyInterruptToRam,
        );
        rfu_STC_clearAPIVariables();
        {
            i = 0u16;
            'l3: loop {
                if !(((i) as i32) < 4i32) {
                    break 'l3;
                }
                'l4: {
                    ((((((&raw mut gRfuSlotStatusNI).cast::<u8>().cast::<*mut u8>())
                        .cast::<*mut u8>())
                    .wrapping_offset(((i) as i32) as isize))
                    .read())
                    .wrapping_add(104)
                    .cast::<*mut u8>())
                    .write(core::ptr::null_mut());
                    ((((((&raw mut gRfuSlotStatusNI).cast::<u8>().cast::<*mut u8>())
                        .cast::<*mut u8>())
                    .wrapping_offset(((i) as i32) as isize))
                    .read())
                    .wrapping_add(108)
                    .cast::<u32>())
                    .write(0u32);
                    ((((((&raw mut gRfuSlotStatusUNI).cast::<u8>().cast::<*mut u8>())
                        .cast::<*mut u8>())
                    .wrapping_offset(((i) as i32) as isize))
                    .read())
                    .wrapping_add(20)
                    .cast::<*mut u8>())
                    .write(core::ptr::null_mut());
                    ((((((&raw mut gRfuSlotStatusUNI).cast::<u8>().cast::<*mut u8>())
                        .cast::<*mut u8>())
                    .wrapping_offset(((i) as i32) as isize))
                    .read())
                    .wrapping_add(24)
                    .cast::<u32>())
                    .write(0u32);
                }
                i = (i).wrapping_add(1);
            }
        }
        'l5: loop {
            'l6: {
                let mut _src: *mut u16 = (((core::mem::transmute::<_, usize>(Some(
                    rfu_STC_fastCopy as unsafe extern "C" fn(*mut *mut u8, *mut *mut u8, i32),
                )) as u32)
                    & 4294967294u32) as usize
                    as *mut u16);
                let mut _dst: *mut u16 = ((((&raw mut gRfuFixed).cast::<u8>().cast::<*mut u8>())
                    .read())
                .wrapping_add(8))
                .cast::<u16>();
                buffByteSizeMax = ((crate::c::div_u32(96u32, 2u32)) as u16);
                'l7: loop {
                    if !((({
                        let __t1 = buffByteSizeMax;
                        buffByteSizeMax = (buffByteSizeMax).wrapping_sub(1);
                        __t1
                    }) as i32)
                        != 0i32)
                    {
                        break 'l7;
                    }
                    ({
                        let __t2 = _dst;
                        _dst = (_dst).wrapping_offset(1);
                        __t2
                    })
                    .write(
                        ({
                            let __t4 = _src;
                            _src = (_src).wrapping_offset(1);
                            __t4
                        })
                        .read(),
                    );
                }
            }
            if !((0i32) != 0) {
                break 'l5;
            }
        }
        ((((&raw mut gRfuFixed).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(4)
            .cast::<Option<unsafe extern "C" fn(*mut *mut u8, *mut *mut u8, i32)>>())
        .write(core::mem::transmute::<
            _,
            Option<unsafe extern "C" fn(*mut *mut u8, *mut *mut u8, i32)>,
        >(
            ((((((&raw mut gRfuFixed).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(8))
                .cast::<u16>())
            .cast::<u8>())
            .wrapping_offset(1),
        ));
        return 0u16;
    }
}
pub(crate) unsafe extern "C" fn rfu_STC_clearAPIVariables() {
    unsafe {
        let mut IMEBackup: u16 = ((67109384i32) as usize as *mut u16).read_volatile();
        let mut i: u8 = 0u8;
        let mut flags: u8 = 0u8;
        crate::c::volatile_write(((67109384i32) as usize as *mut u16), 0u16);
        flags = (((&raw mut gRfuStatic).cast::<u8>().cast::<*mut u8>()).read()).read();
        'l1: loop {
            'l2: {
                {
                    let mut tmp: u16 = 0u16;
                    (&raw mut tmp).write_volatile(0u16);
                    'l3: loop {
                        'l4: {
                            CpuSet(
                                (&raw mut tmp).cast::<u8>(),
                                ((&raw mut gRfuStatic).cast::<u8>().cast::<*mut u8>()).read(),
                                (16777216u32
                                    | (crate::c::div_u32(
                                        40u32,
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
        (((&raw mut gRfuStatic).cast::<u8>().cast::<*mut u8>()).read())
            .write(((((flags) as i32) & 8i32) as u8));
        'l5: loop {
            'l6: {
                {
                    let mut tmp: u16 = 0u16;
                    (&raw mut tmp).write_volatile(0u16);
                    'l7: loop {
                        'l8: {
                            CpuSet(
                                (&raw mut tmp).cast::<u8>(),
                                ((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>()).read(),
                                (16777216u32
                                    | (crate::c::div_u32(
                                        180u32,
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
        ((((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(9))
            .write(4u8);
        ((((&raw mut gRfuStatic).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(6))
            .write(0u8);
        (((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>()).read()).write(255u8);
        rfu_clearAllSlot();
        ((((&raw mut gRfuStatic).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(9))
            .write(0u8);
        {
            i = 0u8;
            'l9: loop {
                if !(((i) as i32) < 4i32) {
                    break 'l9;
                }
                'l10: {
                    ((((((&raw mut gRfuStatic).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(18))
                    .cast::<u16>())
                    .wrapping_offset(((i) as i32) as isize))
                    .write(0u16);
                }
                i = (i).wrapping_add(1);
            }
        }
        crate::c::volatile_write(((67109384i32) as usize as *mut u16), IMEBackup);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rfu_REQ_PARENT_resumeRetransmitAndChange() {
    unsafe {
        STWI_set_Callback_M(core::mem::transmute::<
            Option<unsafe extern "C" fn(u8, u16)>,
            *mut u8,
        >(Some(
            rfu_STC_REQ_callback as unsafe extern "C" fn(u8, u16),
        )));
        STWI_send_ResumeRetransmitAndChangeREQ();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rfu_UNI_PARENT_getDRAC_ACK(ackFlag: *mut u8) -> u16 {
    unsafe {
        let mut ackFlag = ackFlag;
        let mut buf: *mut u8 = core::ptr::null_mut();
        (ackFlag).write(0u8);
        if (((((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>()).read()).read()) as i32)
            != 1i32
        {
            return 768u16;
        }
        buf = rfu_getSTWIRecvBuffer();
        'l1: {
            let __sw1 = (((buf).read()) as i32);
            let __matched = __sw1 == 40i32 || __sw1 == 54i32;
            if __sw1 == 40i32 || __sw1 == 54i32 {
                if ((((buf).wrapping_offset(1)).read()) as i32) == 0i32 {
                    (ackFlag).write(
                        ((((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(2))
                        .read(),
                    );
                } else {
                    (ackFlag).write(((buf).wrapping_offset(4)).read());
                }
                return 0u16;
            }
            if !__matched {
                return 16u16;
            }
        }
        #[allow(unreachable_code)]
        {
            return 0u16;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rfu_setTimerInterrupt(
    timerNo: u8,
    timerIntrTable_p: *mut Option<unsafe extern "C" fn()>,
) {
    unsafe {
        let mut timerNo = timerNo;
        let mut timerIntrTable_p = timerIntrTable_p;
        STWI_init_timer(timerIntrTable_p, ((timerNo) as i32));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rfu_getSTWIRecvBuffer() -> *mut u8 {
    unsafe {
        return ((((&raw mut gRfuFixed).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(220)
            .cast::<*mut u8>())
        .read();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rfu_setMSCCallback(callback: Option<unsafe extern "C" fn(u16)>) {
    unsafe {
        let mut callback = callback;
        STWI_set_Callback_S(callback);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rfu_setREQCallback(callback: Option<unsafe extern "C" fn(u16, u16)>) {
    unsafe {
        let mut callback = callback;
        ((((&raw mut gRfuFixed).cast::<u8>().cast::<*mut u8>()).read())
            .cast::<Option<unsafe extern "C" fn(u16, u16)>>())
        .write(callback);
        rfu_enableREQCallback(((core::mem::transmute::<_, usize>(callback) != 0usize) as u8));
    }
}
pub(crate) unsafe extern "C" fn rfu_enableREQCallback(enable: u8) {
    unsafe {
        let mut enable = enable;
        if (enable) != 0 {
            let __p1 = (((&raw mut gRfuStatic).cast::<u8>().cast::<*mut u8>()).read());
            (__p1).write((((((__p1).read()) as i32) | 8i32) as u8));
        } else {
            let __p2 = (((&raw mut gRfuStatic).cast::<u8>().cast::<*mut u8>()).read());
            (__p2).write((((((__p2).read()) as i32) & 247i32) as u8));
        }
    }
}
pub(crate) unsafe extern "C" fn rfu_STC_REQ_callback(reqCommand: u8, reqResult: u16) {
    unsafe {
        let mut reqCommand = reqCommand;
        let mut reqResult = reqResult;
        STWI_set_Callback_M(core::mem::transmute::<
            Option<unsafe extern "C" fn(u8, u16)>,
            *mut u8,
        >(Some(
            rfu_CB_defaultCallback as unsafe extern "C" fn(u8, u16),
        )));
        ((((&raw mut gRfuStatic).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(28)
            .cast::<u16>())
        .write(reqResult);
        if ((((((&raw mut gRfuStatic).cast::<u8>().cast::<*mut u8>()).read()).read()) as i32)
            & 8i32)
            != 0
        {
            (((((&raw mut gRfuFixed).cast::<u8>().cast::<*mut u8>()).read())
                .cast::<Option<unsafe extern "C" fn(u16, u16)>>())
            .read())
            .unwrap_unchecked()(((reqCommand) as u16), reqResult);
        }
    }
}
pub(crate) unsafe extern "C" fn rfu_CB_defaultCallback(reqCommand: u8, reqResult: u16) {
    unsafe {
        let mut reqCommand = reqCommand;
        let mut reqResult = reqResult;
        let mut bmSlotFlags: i32 = 0i32;
        let mut i: u8 = 0u8;
        if ((reqCommand) as i32) == 255i32 {
            if ((((((&raw mut gRfuStatic).cast::<u8>().cast::<*mut u8>()).read()).read()) as i32)
                & 8i32)
                != 0
            {
                (((((&raw mut gRfuFixed).cast::<u8>().cast::<*mut u8>()).read())
                    .cast::<Option<unsafe extern "C" fn(u16, u16)>>())
                .read())
                .unwrap_unchecked()(((reqCommand) as u16), reqResult);
            }
            bmSlotFlags = (((((((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(2))
            .read()) as i32)
                | ((((((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(3))
                .read()) as i32));
            {
                i = 0u8;
                'l1: loop {
                    if !(((i) as i32) < 4i32) {
                        break 'l1;
                    }
                    'l2: {
                        if (crate::c::shr_i32(bmSlotFlags, ((i) as u32)) & 1i32) != 0 {
                            rfu_STC_removeLinkData(i, 1u8);
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
            (((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>()).read()).write(255u8);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rfu_waitREQComplete() -> u16 {
    unsafe {
        STWI_poll_CommandEnd();
        return ((((&raw mut gRfuStatic).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(28)
            .cast::<u16>())
        .read();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rfu_REQ_RFUStatus() {
    unsafe {
        STWI_set_Callback_M(core::mem::transmute::<
            Option<unsafe extern "C" fn(u8, u16)>,
            *mut u8,
        >(Some(
            rfu_STC_REQ_callback as unsafe extern "C" fn(u8, u16),
        )));
        STWI_send_SystemStatusREQ();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rfu_getRFUStatus(rfuState: *mut u8) -> u16 {
    unsafe {
        let mut rfuState = rfuState;
        if ((((((((&raw mut gRfuFixed).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(220)
            .cast::<*mut u8>())
        .read())
        .cast::<u8>())
        .read()) as i32)
            != 147i32
        {
            return 16u16;
        }
        if ((STWI_poll_CommandEnd()) as i32) == 0i32 {
            (rfuState).write(
                (((((((&raw mut gRfuFixed).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(220)
                    .cast::<*mut u8>())
                .read())
                .cast::<u8>())
                .wrapping_offset(7))
                .read(),
            );
        } else {
            (rfuState).write(255u8);
        }
        return 0u16;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rfu_MBOOT_CHILD_inheritanceLinkStatus() -> u16 {
    unsafe {
        let mut s1: *mut u8 = ((&raw const str_checkMbootLL).cast::<u8>().cast_mut()).cast::<u8>();
        let mut s2: *mut u8 = ((50331888i32) as usize as *mut u8);
        let mut checksum: u16 = 0u16;
        let mut mb_buff_iwram_p: *mut u16 = core::ptr::null_mut();
        let mut i: u8 = 0u8;
        'l1: loop {
            if !((((s1).read()) as i32) != 0i32) {
                break 'l1;
            }
            if ((({
                let __t2 = s1;
                s1 = (s1).wrapping_offset(1);
                __t2
            })
            .read()) as i32)
                != ((({
                    let __t4 = s2;
                    s2 = (s2).wrapping_offset(1);
                    __t4
                })
                .read()) as i32)
            {
                return 1u16;
            }
        }
        mb_buff_iwram_p = ((50331648i32) as usize as *mut u16);
        checksum = 0u16;
        {
            i = 0u8;
            'l2: loop {
                if !(((i) as i32) < crate::c::div_i32(180i32, 2i32)) {
                    break 'l2;
                }
                'l3: {
                    checksum = ((((checksum) as i32).wrapping_add(
                        ((({
                            let __t6 = mb_buff_iwram_p;
                            mb_buff_iwram_p = (mb_buff_iwram_p).wrapping_offset(1);
                            __t6
                        })
                        .read()) as i32),
                    )) as u16);
                }
                i = (i).wrapping_add(1);
            }
        }
        if ((checksum) as i32) != ((((50331898i32) as usize as *mut u16).read()) as i32) {
            return 1u16;
        }
        'l4: loop {
            'l5: {
                'l6: loop {
                    'l7: {
                        CpuSet(
                            ((50331648i32) as usize as *mut u16).cast::<u8>(),
                            ((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>()).read(),
                            (0u32
                                | (crate::c::div_u32(
                                    180u32,
                                    ((crate::c::div_i32(16i32, 8i32)) as u32),
                                ) & 2097151u32)),
                        );
                    }
                    if !((0i32) != 0) {
                        break 'l6;
                    }
                }
            }
            if !((0i32) != 0) {
                break 'l4;
            }
        }
        let __p7 = (((&raw mut gRfuStatic).cast::<u8>().cast::<*mut u8>()).read());
        (__p7).write((((((__p7).read()) as i32) | 128i32) as u8));
        return 0u16;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rfu_REQ_stopMode() {
    unsafe {
        let mut timerReg: *mut u32 = core::ptr::null_mut();
        if ((((67109384i32) as usize as *mut u16).read_volatile()) as i32) == 0i32 {
            rfu_STC_REQ_callback(61u8, 6u16);
            crate::c::volatile_write(
                (((&raw mut gSTWIStatus).cast::<*mut u8>()).read())
                    .wrapping_add(18)
                    .cast::<u16>(),
                6u16,
            );
        } else {
            AgbRFU_SoftReset();
            rfu_STC_clearAPIVariables();
            if AgbRFU_checkID(8u8) == 32769i32 {
                timerReg =
                    (((67109120i32).wrapping_add(
                        ((((((&raw mut gSTWIStatus).cast::<*mut u8>()).read()).wrapping_add(10))
                            .read()) as i32)
                            .wrapping_mul(4i32),
                    )) as usize as *mut u32);
                crate::c::volatile_write(timerReg, 0u32);
                crate::c::volatile_write(timerReg, 8585216u32);
                'l1: loop {
                    if !(((timerReg).read_volatile() << 16) < 17170432u32) {
                        break 'l1;
                    }
                }
                crate::c::volatile_write(timerReg, 0u32);
                STWI_set_Callback_M(core::mem::transmute::<
                    Option<unsafe extern "C" fn(u8, u16)>,
                    *mut u8,
                >(Some(
                    rfu_CB_stopMode as unsafe extern "C" fn(u8, u16),
                )));
                STWI_send_StopModeREQ();
            } else {
                crate::c::volatile_write(((67109160i32) as usize as *mut u16), 8192u16);
                rfu_STC_REQ_callback(61u8, 0u16);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn rfu_CB_stopMode(reqCommand: u8, reqResult: u16) {
    unsafe {
        let mut reqCommand = reqCommand;
        let mut reqResult = reqResult;
        if ((reqResult) as i32) == 0i32 {
            crate::c::volatile_write(((67109160i32) as usize as *mut u16), 8192u16);
        }
        rfu_STC_REQ_callback(reqCommand, reqResult);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rfu_REQBN_softReset_and_checkID() -> u32 {
    unsafe {
        let mut id: u32 = 0u32;
        if ((((67109384i32) as usize as *mut u16).read_volatile()) as i32) == 0i32 {
            return 4294967295u32;
        }
        AgbRFU_SoftReset();
        rfu_STC_clearAPIVariables();
        if {
            let __v1 = ((AgbRFU_checkID(30u8)) as u32);
            id = __v1;
            __v1
        } == 0u32
        {
            crate::c::volatile_write(((67109160i32) as usize as *mut u16), 8192u16);
        }
        return id;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rfu_REQ_reset() {
    unsafe {
        STWI_set_Callback_M(core::mem::transmute::<
            Option<unsafe extern "C" fn(u8, u16)>,
            *mut u8,
        >(Some(
            rfu_CB_reset as unsafe extern "C" fn(u8, u16),
        )));
        STWI_send_ResetREQ();
    }
}
pub(crate) unsafe extern "C" fn rfu_CB_reset(reqCommand: u8, reqResult: u16) {
    unsafe {
        let mut reqCommand = reqCommand;
        let mut reqResult = reqResult;
        if ((reqResult) as i32) == 0i32 {
            rfu_STC_clearAPIVariables();
        }
        rfu_STC_REQ_callback(reqCommand, reqResult);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rfu_REQ_configSystem(availSlotFlag: u16, maxMFrame: u8, mcTimer: u8) {
    unsafe {
        let mut availSlotFlag = availSlotFlag;
        let mut maxMFrame = maxMFrame;
        let mut mcTimer = mcTimer;
        STWI_set_Callback_M(core::mem::transmute::<
            Option<unsafe extern "C" fn(u8, u16)>,
            *mut u8,
        >(Some(
            rfu_STC_REQ_callback as unsafe extern "C" fn(u8, u16),
        )));
        STWI_send_SystemConfigREQ(
            (((((availSlotFlag) as i32) & 3i32) | 60i32) as u16),
            maxMFrame,
            mcTimer,
        );
        if ((mcTimer) as i32) == 0i32 {
            ((((&raw mut gRfuStatic).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(26)
                .cast::<u16>())
            .write(1u16);
        } else {
            let mut IMEBackup: u16 = ((67109384i32) as usize as *mut u16).read_volatile();
            crate::c::volatile_write(((67109384i32) as usize as *mut u16), 0u16);
            ((((&raw mut gRfuStatic).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(26)
                .cast::<u16>())
            .write(((Div(600i32, ((mcTimer) as i32))) as u16));
            crate::c::volatile_write(((67109384i32) as usize as *mut u16), IMEBackup);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rfu_REQ_configGameData(
    mbootFlag: u8,
    serialNo: u16,
    gname: *mut u8,
    uname: *mut u8,
) {
    unsafe {
        let mut mbootFlag = mbootFlag;
        let mut serialNo = serialNo;
        let mut gname = gname;
        let mut uname = uname;
        let mut packet = crate::ffi::Align4([0u8; 16]);
        let mut i: u8 = 0u8;
        let mut check_sum: u8 = 0u8;
        let mut gnameBackup: *mut u8 = gname;
        let mut unameBackup: *mut u8 = core::ptr::null_mut();
        ((&raw mut packet).cast::<u8>()).write(((serialNo) as u8));
        (((&raw mut packet).cast::<u8>()).wrapping_offset(1))
            .write(((((serialNo) as i32) >> 8) as u8));
        if ((mbootFlag) as i32) != 0i32 {
            (((&raw mut packet).cast::<u8>()).wrapping_offset(1))
                .write((((((serialNo) as i32) >> 8) | 128i32) as u8));
        }
        {
            i = 2u8;
            'l1: loop {
                if !(((i) as i32) < 15i32) {
                    break 'l1;
                }
                'l2: {
                    (((&raw mut packet).cast::<u8>()).wrapping_offset(((i) as i32) as isize))
                        .write(
                            ({
                                let __t2 = gname;
                                gname = (gname).wrapping_offset(1);
                                __t2
                            })
                            .read(),
                        );
                }
                i = (i).wrapping_add(1);
            }
        }
        check_sum = 0u8;
        unameBackup = uname;
        {
            i = 0u8;
            'l3: loop {
                if !(((i) as i32) < 8i32) {
                    break 'l3;
                }
                'l4: {
                    check_sum = ((((check_sum) as i32).wrapping_add(
                        ((({
                            let __t4 = unameBackup;
                            unameBackup = (unameBackup).wrapping_offset(1);
                            __t4
                        })
                        .read()) as i32),
                    )) as u8);
                    check_sum = ((((check_sum) as i32).wrapping_add(
                        ((({
                            let __t6 = gnameBackup;
                            gnameBackup = (gnameBackup).wrapping_offset(1);
                            __t6
                        })
                        .read()) as i32),
                    )) as u8);
                }
                i = (i).wrapping_add(1);
            }
        }
        (((&raw mut packet).cast::<u8>()).wrapping_offset(15))
            .write(((!((check_sum) as i32)) as u8));
        if ((mbootFlag) as i32) != 0i32 {
            (((&raw mut packet).cast::<u8>()).wrapping_offset(14)).write(0u8);
        }
        STWI_set_Callback_M(core::mem::transmute::<
            Option<unsafe extern "C" fn(u8, u16)>,
            *mut u8,
        >(Some(
            rfu_CB_configGameData as unsafe extern "C" fn(u8, u16),
        )));
        STWI_send_GameConfigREQ((&raw mut packet).cast::<u8>(), uname);
    }
}
pub(crate) unsafe extern "C" fn rfu_CB_configGameData(reqCommand: u8, reqResult: u16) {
    unsafe {
        let mut reqCommand = reqCommand;
        let mut reqResult = reqResult;
        let mut serialNo: i32 = 0i32;
        let mut gname_uname_p: *mut u8 = core::ptr::null_mut();
        let mut i: u8 = 0u8;
        let mut packet_p: *mut u8 = core::ptr::null_mut();
        if ((reqResult) as i32) == 0i32 {
            packet_p = (((((&raw mut gSTWIStatus).cast::<*mut u8>()).read())
                .wrapping_add(36)
                .cast::<*mut u8>())
            .read())
            .cast::<u8>();
            serialNo = (({
                let __v1 = ((((packet_p).wrapping_offset(4)).read()) as u16);
                (((((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(148))
                .wrapping_add(4)
                .cast::<u16>())
                .write(__v1);
                __v1
            }) as i32);
            (((((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(148))
            .wrapping_add(4)
            .cast::<u16>())
            .write((((((((packet_p).wrapping_offset(5)).read()) as i32) << 8) | serialNo) as u16));
            gname_uname_p = (packet_p).wrapping_offset(6);
            if ((((((((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(148))
            .wrapping_add(4)
            .cast::<u16>())
            .read()) as i32)
                & 32768i32)
                != 0
            {
                (((((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(148))
                .wrapping_add(4)
                .cast::<u16>())
                .write(
                    (((((((((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(148))
                    .wrapping_add(4)
                    .cast::<u16>())
                    .read()) as i32)
                        ^ 32768i32) as u16),
                );
                (((((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(148))
                .wrapping_add(3))
                .write(1u8);
            } else {
                (((((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(148))
                .wrapping_add(3))
                .write(0u8);
            }
            {
                i = 0u8;
                'l1: loop {
                    if !(((i) as i32) < 13i32) {
                        break 'l1;
                    }
                    'l2: {
                        (((((((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(148))
                        .wrapping_add(6))
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize))
                        .write(
                            ({
                                let __t3 = gname_uname_p;
                                gname_uname_p = (gname_uname_p).wrapping_offset(1);
                                __t3
                            })
                            .read(),
                        );
                    }
                    i = (i).wrapping_add(1);
                }
            }
            gname_uname_p = (gname_uname_p).wrapping_offset(1);
            {
                i = 0u8;
                'l3: loop {
                    if !(((i) as i32) < 8i32) {
                        break 'l3;
                    }
                    'l4: {
                        (((((((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(148))
                        .wrapping_add(21))
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize))
                        .write(
                            ({
                                let __t5 = gname_uname_p;
                                gname_uname_p = (gname_uname_p).wrapping_offset(1);
                                __t5
                            })
                            .read(),
                        );
                    }
                    i = (i).wrapping_add(1);
                }
            }
        }
        rfu_STC_REQ_callback(reqCommand, reqResult);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rfu_REQ_startSearchChild() {
    unsafe {
        let mut result: u16 = 0u16;
        let mut i: u16 = 0u16;
        {
            i = 0u16;
            'l1: loop {
                if !(((i) as i32) < 4i32) {
                    break 'l1;
                }
                'l2: {
                    ((((((&raw mut gRfuStatic).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(14))
                    .cast::<u8>())
                    .wrapping_offset(((i) as i32) as isize))
                    .write(0u8);
                }
                i = (i).wrapping_add(1);
            }
        }
        STWI_set_Callback_M(core::mem::transmute::<
            Option<unsafe extern "C" fn(u8, u16)>,
            *mut u8,
        >(Some(
            rfu_CB_defaultCallback as unsafe extern "C" fn(u8, u16),
        )));
        STWI_send_SystemStatusREQ();
        result = STWI_poll_CommandEnd();
        if ((result) as i32) == 0i32 {
            if (((((((((&raw mut gRfuFixed).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(220)
                .cast::<*mut u8>())
            .read())
            .cast::<u8>())
            .wrapping_offset(7))
            .read()) as i32)
                == 0i32
            {
                rfu_STC_clearLinkStatus(1u8);
            }
        } else {
            rfu_STC_REQ_callback(25u8, result);
        }
        STWI_set_Callback_M(core::mem::transmute::<
            Option<unsafe extern "C" fn(u8, u16)>,
            *mut u8,
        >(Some(
            rfu_CB_startSearchChild as unsafe extern "C" fn(u8, u16),
        )));
        STWI_send_SC_StartREQ();
    }
}
pub(crate) unsafe extern "C" fn rfu_CB_startSearchChild(reqCommand: u8, reqResult: u16) {
    unsafe {
        let mut reqCommand = reqCommand;
        let mut reqResult = reqResult;
        if ((reqResult) as i32) == 0i32 {
            ((((&raw mut gRfuStatic).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(9))
                .write(1u8);
        }
        rfu_STC_REQ_callback(reqCommand, reqResult);
    }
}
pub(crate) unsafe extern "C" fn rfu_STC_clearLinkStatus(parentChild: u8) {
    unsafe {
        let mut parentChild = parentChild;
        let mut i: u8 = 0u8;
        rfu_clearAllSlot();
        if ((parentChild) as i32) != 0i32 {
            'l1: loop {
                'l2: {
                    {
                        let mut tmp: u16 = 0u16;
                        (&raw mut tmp).write_volatile(0u16);
                        'l3: loop {
                            'l4: {
                                CpuSet(
                                    (&raw mut tmp).cast::<u8>(),
                                    ((((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(20))
                                    .cast::<u8>(),
                                    (16777216u32
                                        | (crate::c::div_u32(
                                            128u32,
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
            ((((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(8))
                .write(0u8);
        }
        {
            i = 0u8;
            'l5: loop {
                if !(((i) as i32) < 4i32) {
                    break 'l5;
                }
                'l6: {
                    ((((((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(10))
                    .cast::<u8>())
                    .wrapping_offset(((i) as i32) as isize))
                    .write(0u8);
                }
                i = (i).wrapping_add(1);
            }
        }
        ((((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1))
            .write(0u8);
        ((((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(2))
            .write(0u8);
        ((((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(3))
            .write(0u8);
        ((((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(7))
            .write(0u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rfu_REQ_pollSearchChild() {
    unsafe {
        STWI_set_Callback_M(core::mem::transmute::<
            Option<unsafe extern "C" fn(u8, u16)>,
            *mut u8,
        >(Some(
            rfu_CB_pollAndEndSearchChild as unsafe extern "C" fn(u8, u16),
        )));
        STWI_send_SC_PollingREQ();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rfu_REQ_endSearchChild() {
    unsafe {
        STWI_set_Callback_M(core::mem::transmute::<
            Option<unsafe extern "C" fn(u8, u16)>,
            *mut u8,
        >(Some(
            rfu_CB_pollAndEndSearchChild as unsafe extern "C" fn(u8, u16),
        )));
        STWI_send_SC_EndREQ();
    }
}
pub(crate) unsafe extern "C" fn rfu_CB_pollAndEndSearchChild(reqCommand: u8, reqResult: u16) {
    unsafe {
        let mut reqCommand = reqCommand;
        let mut reqResult = reqResult;
        if ((reqResult) as i32) == 0i32 {
            rfu_STC_readChildList();
        }
        if ((reqCommand) as i32) == 26i32 {
            if (((((((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(148))
            .cast::<u16>())
            .read()) as i32)
                == 0i32
            {
                STWI_set_Callback_M(core::mem::transmute::<
                    Option<unsafe extern "C" fn(u8, u16)>,
                    *mut u8,
                >(Some(
                    rfu_CB_defaultCallback as unsafe extern "C" fn(u8, u16),
                )));
                STWI_send_SystemStatusREQ();
                if ((STWI_poll_CommandEnd()) as i32) == 0i32 {
                    (((((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(148))
                    .cast::<u16>())
                    .write(
                        ((((((((&raw mut gRfuFixed).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(220)
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_add(4))
                        .cast::<u32>())
                        .cast::<u16>())
                        .read(),
                    );
                }
            }
        } else {
            if ((reqCommand) as i32) == 27i32 {
                if (((((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>()).read()).read())
                    as i32)
                    == 255i32
                {
                    (((((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(148))
                    .cast::<u16>())
                    .write(0u16);
                }
                ((((&raw mut gRfuStatic).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(9))
                    .write(0u8);
            }
        }
        rfu_STC_REQ_callback(reqCommand, reqResult);
    }
}
pub(crate) unsafe extern "C" fn rfu_STC_readChildList() {
    unsafe {
        let mut stwiParam: u32 = 0u32;
        let mut numSlots: u8 = (((((((&raw mut gRfuFixed).cast::<u8>().cast::<*mut u8>())
            .read())
        .wrapping_add(220)
        .cast::<*mut u8>())
        .read())
        .cast::<u8>())
        .wrapping_offset(1))
        .read();
        let mut data_p: *mut u8 = core::ptr::null_mut();
        let mut i: u8 = 0u8;
        let mut bm_slot_id: u8 = 0u8;
        {
            data_p = ((((((&raw mut gRfuFixed).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(220)
                .cast::<*mut u8>())
            .read())
            .cast::<u8>())
            .wrapping_offset(4);
            'l1: loop {
                if !(((numSlots) as i32) != 0i32) {
                    break 'l1;
                }
                'l2: {
                    bm_slot_id = ((data_p).wrapping_offset(2)).read();
                    if ((((bm_slot_id) as i32) < 4i32)
                        && (!((crate::c::shr_i32(
                            ((((((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(2))
                            .read()) as i32),
                            ((bm_slot_id) as u32),
                        ) & 1i32)
                            != 0)))
                        && (!((crate::c::shr_i32(
                            ((((((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(3))
                            .read()) as i32),
                            ((bm_slot_id) as u32),
                        ) & 1i32)
                            != 0))
                    {
                        ((((((&raw mut gRfuStatic).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(14))
                        .cast::<u8>())
                        .wrapping_offset(((bm_slot_id) as i32) as isize))
                        .write(240u8);
                        ((((((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(10))
                        .cast::<u8>())
                        .wrapping_offset(((bm_slot_id) as i32) as isize))
                        .write(16u8);
                        let __p1 = (((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(2);
                        (__p1).write(
                            (((((__p1).read()) as i32)
                                | crate::c::shl_i32(1i32, ((bm_slot_id) as u32)))
                                as u8),
                        );
                        let __p2 = (((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(1);
                        (__p2).write(((__p2).read()).wrapping_add(1));
                        (((((((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(20))
                        .cast::<u8>())
                        .wrapping_offset(((bm_slot_id) as i32) as isize * 32))
                        .cast::<u16>())
                        .write(((data_p).cast::<u16>()).read());
                        (((((((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(20))
                        .cast::<u8>())
                        .wrapping_offset(((bm_slot_id) as i32) as isize * 32))
                        .wrapping_add(2))
                        .write(bm_slot_id);
                        (((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>()).read())
                            .write(1u8);
                        let __p3 = (((&raw mut gRfuStatic).cast::<u8>().cast::<*mut u8>()).read());
                        (__p3).write((((((__p3).read()) as i32) & 127i32) as u8));
                        ((((((&raw mut gRfuStatic).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(18))
                        .cast::<u16>())
                        .wrapping_offset(((bm_slot_id) as i32) as isize))
                        .write(
                            (((((((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(20))
                            .cast::<u8>())
                            .wrapping_offset(((bm_slot_id) as i32) as isize * 32))
                            .cast::<u16>())
                            .read(),
                        );
                    }
                    numSlots = (numSlots).wrapping_sub(1);
                }
                data_p = (data_p).wrapping_offset(4);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rfu_REQ_startSearchParent() {
    unsafe {
        STWI_set_Callback_M(core::mem::transmute::<
            Option<unsafe extern "C" fn(u8, u16)>,
            *mut u8,
        >(Some(
            rfu_CB_startSearchParent as unsafe extern "C" fn(u8, u16),
        )));
        STWI_send_SP_StartREQ();
    }
}
pub(crate) unsafe extern "C" fn rfu_CB_startSearchParent(reqCommand: u8, reqResult: u16) {
    unsafe {
        let mut reqCommand = reqCommand;
        let mut reqResult = reqResult;
        if ((reqResult) as i32) == 0i32 {
            rfu_STC_clearLinkStatus(0u8);
        }
        rfu_STC_REQ_callback(reqCommand, reqResult);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rfu_REQ_pollSearchParent() {
    unsafe {
        STWI_set_Callback_M(core::mem::transmute::<
            Option<unsafe extern "C" fn(u8, u16)>,
            *mut u8,
        >(Some(
            rfu_CB_pollSearchParent as unsafe extern "C" fn(u8, u16),
        )));
        STWI_send_SP_PollingREQ();
    }
}
pub(crate) unsafe extern "C" fn rfu_CB_pollSearchParent(reqCommand: u8, reqResult: u16) {
    unsafe {
        let mut reqCommand = reqCommand;
        let mut reqResult = reqResult;
        if ((reqResult) as i32) == 0i32 {
            rfu_STC_readParentCandidateList();
        }
        rfu_STC_REQ_callback(reqCommand, reqResult);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rfu_REQ_endSearchParent() {
    unsafe {
        STWI_set_Callback_M(core::mem::transmute::<
            Option<unsafe extern "C" fn(u8, u16)>,
            *mut u8,
        >(Some(
            rfu_STC_REQ_callback as unsafe extern "C" fn(u8, u16),
        )));
        STWI_send_SP_EndREQ();
    }
}
pub(crate) unsafe extern "C" fn rfu_STC_readParentCandidateList() {
    unsafe {
        let mut numSlots: u8 = 0u8;
        let mut i: u8 = 0u8;
        let mut check_sum: u8 = 0u8;
        let mut my_check_sum: u8 = 0u8;
        let mut j: u8 = 0u8;
        let mut uname_p: *mut u8 = core::ptr::null_mut();
        let mut packet_p: *mut u8 = core::ptr::null_mut();
        let mut target: *mut u8 = core::ptr::null_mut();
        'l1: loop {
            'l2: {
                {
                    let mut tmp: u16 = 0u16;
                    (&raw mut tmp).write_volatile(0u16);
                    'l3: loop {
                        'l4: {
                            CpuSet(
                                (&raw mut tmp).cast::<u8>(),
                                ((((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(20))
                                .cast::<u8>(),
                                (16777216u32
                                    | (crate::c::div_u32(
                                        128u32,
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
        packet_p = (((((&raw mut gRfuFixed).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(220)
            .cast::<*mut u8>())
        .read())
        .cast::<u8>();
        numSlots = ((packet_p).wrapping_offset(1)).read();
        packet_p = (packet_p).wrapping_offset(4);
        ((((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(8))
            .write(0u8);
        {
            i = 0u8;
            'l5: loop {
                if !((((i) as i32) < 4i32) && (((numSlots) as i32) != 0i32)) {
                    break 'l5;
                }
                'l6: {
                    numSlots = ((((numSlots) as i32).wrapping_sub(7i32)) as u8);
                    uname_p = (packet_p).wrapping_offset(6);
                    packet_p = (packet_p).wrapping_offset(19);
                    check_sum = ((!(((packet_p).read()) as i32)) as u8);
                    packet_p = (packet_p).wrapping_offset(1);
                    my_check_sum = 0u8;
                    {
                        j = 0u8;
                        'l7: loop {
                            if !(((j) as i32) < 8i32) {
                                break 'l7;
                            }
                            'l8: {
                                my_check_sum = ((((my_check_sum) as i32).wrapping_add(
                                    ((({
                                        let __t2 = packet_p;
                                        packet_p = (packet_p).wrapping_offset(1);
                                        __t2
                                    })
                                    .read()) as i32),
                                )) as u8);
                                my_check_sum = ((((my_check_sum) as i32).wrapping_add(
                                    ((({
                                        let __t4 = uname_p;
                                        uname_p = (uname_p).wrapping_offset(1);
                                        __t4
                                    })
                                    .read()) as i32),
                                )) as u8);
                            }
                            j = (j).wrapping_add(1);
                        }
                    }
                    if ((my_check_sum) as i32) == ((check_sum) as i32) {
                        packet_p = (packet_p).wrapping_offset(-28);
                        target = (((((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(20))
                        .cast::<u8>())
                        .wrapping_offset(
                            ((((((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(8))
                            .read()) as i32) as isize
                                * 32,
                        );
                        ((target).cast::<u16>()).write(((packet_p).cast::<u16>()).read());
                        packet_p = (packet_p).wrapping_offset(2);
                        ((target).wrapping_add(2)).write((packet_p).read());
                        packet_p = (packet_p).wrapping_offset(2);
                        ((target).wrapping_add(4).cast::<u16>()).write(
                            ((((((packet_p).cast::<u16>()).read()) as i32) & 32767i32) as u16),
                        );
                        if (((((packet_p).cast::<u16>()).read()) as i32) & 32768i32) != 0 {
                            ((target).wrapping_add(3)).write(1u8);
                        } else {
                            ((target).wrapping_add(3)).write(0u8);
                        }
                        packet_p = (packet_p).wrapping_offset(2);
                        {
                            j = 0u8;
                            'l9: loop {
                                if !(((j) as i32) < 13i32) {
                                    break 'l9;
                                }
                                'l10: {
                                    ((((target).wrapping_add(6)).cast::<u8>())
                                        .wrapping_offset(((j) as i32) as isize))
                                    .write(
                                        ({
                                            let __t6 = packet_p;
                                            packet_p = (packet_p).wrapping_offset(1);
                                            __t6
                                        })
                                        .read(),
                                    );
                                }
                                j = (j).wrapping_add(1);
                            }
                        }
                        packet_p = (packet_p).wrapping_offset(1);
                        {
                            j = 0u8;
                            'l11: loop {
                                if !(((j) as i32) < 8i32) {
                                    break 'l11;
                                }
                                'l12: {
                                    ((((target).wrapping_add(21)).cast::<u8>())
                                        .wrapping_offset(((j) as i32) as isize))
                                    .write(
                                        ({
                                            let __t8 = packet_p;
                                            packet_p = (packet_p).wrapping_offset(1);
                                            __t8
                                        })
                                        .read(),
                                    );
                                }
                                j = (j).wrapping_add(1);
                            }
                        }
                        let __p9 = (((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(8);
                        (__p9).write(((__p9).read()).wrapping_add(1));
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rfu_REQ_startConnectParent(pid: u16) {
    unsafe {
        let mut pid = pid;
        let mut result: u16 = 0u16;
        let mut i: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !((((i) as i32) < 4i32)
                    && ((((((((((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>())
                        .read())
                    .wrapping_add(20))
                    .cast::<u8>())
                    .wrapping_offset(((i) as i32) as isize * 32))
                    .cast::<u16>())
                    .read()) as i32)
                        != ((pid) as i32)))
                {
                    break 'l1;
                }
                'l2: {}
                i = (i).wrapping_add(1);
            }
        }
        if ((i) as i32) == 4i32 {
            result = 256u16;
        }
        if ((result) as i32) == 0i32 {
            ((((&raw mut gRfuStatic).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(30)
                .cast::<u16>())
            .write(pid);
            STWI_set_Callback_M(core::mem::transmute::<
                Option<unsafe extern "C" fn(u8, u16)>,
                *mut u8,
            >(Some(
                rfu_STC_REQ_callback as unsafe extern "C" fn(u8, u16),
            )));
            STWI_send_CP_StartREQ(pid);
        } else {
            rfu_STC_REQ_callback(31u8, result);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rfu_REQ_pollConnectParent() {
    unsafe {
        STWI_set_Callback_M(core::mem::transmute::<
            Option<unsafe extern "C" fn(u8, u16)>,
            *mut u8,
        >(Some(
            rfu_CB_pollConnectParent as unsafe extern "C" fn(u8, u16),
        )));
        STWI_send_CP_PollingREQ();
    }
}
pub(crate) unsafe extern "C" fn rfu_CB_pollConnectParent(reqCommand: u8, reqResult: u16) {
    unsafe {
        let mut reqCommand = reqCommand;
        let mut reqResult = reqResult;
        let mut id: u16 = 0u16;
        let mut slot: u8 = 0u8;
        let mut bm_slot_flag: u8 = 0u8;
        let mut i: u8 = 0u8;
        let mut target_p: *mut u8 = core::ptr::null_mut();
        let mut target_local = crate::ffi::Align4([0u8; 32]);
        if ((reqResult) as i32) == 0i32 {
            id = (((((((((&raw mut gRfuFixed).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(220)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(4))
            .cast::<u32>())
            .read()) as u16);
            slot = (((((((&raw mut gRfuFixed).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(220)
                .cast::<*mut u8>())
            .read())
            .cast::<u8>())
            .wrapping_offset(6))
            .read();
            if (((((((((&raw mut gRfuFixed).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(220)
                .cast::<*mut u8>())
            .read())
            .cast::<u8>())
            .wrapping_offset(7))
            .read()) as i32)
                == 0i32
            {
                bm_slot_flag = ((crate::c::shl_i32(1i32, ((slot) as u32))) as u8);
                if !((((bm_slot_flag) as i32)
                    & ((((((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(2))
                    .read()) as i32))
                    != 0)
                {
                    let __p1 = (((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(2);
                    (__p1).write((((((__p1).read()) as i32) | ((bm_slot_flag) as i32)) as u8));
                    let __p2 = (((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(3);
                    (__p2).write((((((__p2).read()) as i32) & !((bm_slot_flag) as i32)) as u8));
                    (((((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(148))
                    .cast::<u16>())
                    .write(id);
                    let __p3 = (((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(1);
                    (__p3).write(((__p3).read()).wrapping_add(1));
                    (((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>()).read()).write(0u8);
                    let __p4 = (((&raw mut gRfuStatic).cast::<u8>().cast::<*mut u8>()).read());
                    (__p4).write((((((__p4).read()) as i32) | 128i32) as u8));
                    {
                        i = 0u8;
                        'l1: loop {
                            if !(((i) as i32) < 4i32) {
                                break 'l1;
                            }
                            'l2: {
                                if (((((((((&raw mut gRfuLinkStatus)
                                    .cast::<u8>()
                                    .cast::<*mut u8>())
                                .read())
                                .wrapping_add(20))
                                .cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize * 32))
                                .cast::<u16>())
                                .read()) as i32)
                                    == ((((((&raw mut gRfuStatic).cast::<u8>().cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(30)
                                    .cast::<u16>())
                                    .read()) as i32)
                                {
                                    if ((((((&raw mut gRfuLinkStatus)
                                        .cast::<u8>()
                                        .cast::<*mut u8>())
                                    .read())
                                    .wrapping_add(8))
                                    .read()) as i32)
                                        != 0i32
                                    {
                                        target_p = (&raw mut target_local).cast::<u8>();
                                        'l3: loop {
                                            'l4: {
                                                'l5: loop {
                                                    'l6: {
                                                        CpuSet(
                                                            (((((&raw mut gRfuLinkStatus)
                                                                .cast::<u8>()
                                                                .cast::<*mut u8>())
                                                            .read())
                                                            .wrapping_add(20))
                                                            .cast::<u8>())
                                                            .wrapping_offset(
                                                                ((i) as i32) as isize * 32,
                                                            ),
                                                            (&raw mut target_local).cast::<u8>(),
                                                            (0u32
                                                                | (crate::c::div_u32(
                                                                    32u32,
                                                                    ((crate::c::div_i32(
                                                                        16i32, 8i32,
                                                                    ))
                                                                        as u32),
                                                                ) & 2097151u32)),
                                                        );
                                                    }
                                                    if !((0i32) != 0) {
                                                        break 'l5;
                                                    }
                                                }
                                            }
                                            if !((0i32) != 0) {
                                                break 'l3;
                                            }
                                        }
                                        'l7: loop {
                                            'l8: {
                                                {
                                                    let mut tmp: u16 = 0u16;
                                                    (&raw mut tmp).write_volatile(0u16);
                                                    'l9: loop {
                                                        'l10: {
                                                            CpuSet(
                                                                (&raw mut tmp).cast::<u8>(),
                                                                ((((&raw mut gRfuLinkStatus)
                                                                    .cast::<u8>()
                                                                    .cast::<*mut u8>())
                                                                .read())
                                                                .wrapping_add(20))
                                                                .cast::<u8>(),
                                                                (16777216u32
                                                                    | (crate::c::div_u32(
                                                                        128u32,
                                                                        ((crate::c::div_i32(
                                                                            16i32, 8i32,
                                                                        ))
                                                                            as u32),
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
                                        ((((&raw mut gRfuLinkStatus)
                                            .cast::<u8>()
                                            .cast::<*mut u8>())
                                        .read())
                                        .wrapping_add(8))
                                        .write(0u8);
                                    } else {
                                        target_p = (((((&raw mut gRfuLinkStatus)
                                            .cast::<u8>()
                                            .cast::<*mut u8>())
                                        .read())
                                        .wrapping_add(20))
                                        .cast::<u8>())
                                        .wrapping_offset(((i) as i32) as isize * 32);
                                    }
                                    break 'l1;
                                }
                            }
                            i = (i).wrapping_add(1);
                        }
                    }
                    if ((i) as i32) < 4i32 {
                        'l11: loop {
                            'l12: {
                                'l13: loop {
                                    'l14: {
                                        CpuSet(
                                            target_p,
                                            (((((&raw mut gRfuLinkStatus)
                                                .cast::<u8>()
                                                .cast::<*mut u8>())
                                            .read())
                                            .wrapping_add(20))
                                            .cast::<u8>())
                                            .wrapping_offset(((slot) as i32) as isize * 32),
                                            (0u32
                                                | (crate::c::div_u32(
                                                    32u32,
                                                    ((crate::c::div_i32(16i32, 8i32)) as u32),
                                                ) & 2097151u32)),
                                        );
                                    }
                                    if !((0i32) != 0) {
                                        break 'l13;
                                    }
                                }
                            }
                            if !((0i32) != 0) {
                                break 'l11;
                            }
                        }
                        (((((((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(20))
                        .cast::<u8>())
                        .wrapping_offset(((slot) as i32) as isize * 32))
                        .wrapping_add(2))
                        .write(slot);
                    }
                }
            }
        }
        rfu_STC_REQ_callback(reqCommand, reqResult);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rfu_getConnectParentStatus(
    status: *mut u8,
    connectSlotNo: *mut u8,
) -> u16 {
    unsafe {
        let mut status = status;
        let mut connectSlotNo = connectSlotNo;
        let mut packet_p: *mut u8 = core::ptr::null_mut();
        (status).write(255u8);
        packet_p = (((((&raw mut gRfuFixed).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(220)
            .cast::<*mut u8>())
        .read())
        .cast::<u8>();
        if ((((packet_p).read()) as i32) == 160i32) || ((((packet_p).read()) as i32) == 161i32) {
            packet_p = (packet_p).wrapping_offset(6);
            (connectSlotNo).write((packet_p).read());
            (status).write(((packet_p).wrapping_offset(1)).read());
            return 0u16;
        }
        return 16u16;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rfu_REQ_endConnectParent() {
    unsafe {
        STWI_set_Callback_M(core::mem::transmute::<
            Option<unsafe extern "C" fn(u8, u16)>,
            *mut u8,
        >(Some(
            rfu_CB_pollConnectParent as unsafe extern "C" fn(u8, u16),
        )));
        STWI_send_CP_EndREQ();
        if (((((((((&raw mut gRfuFixed).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(220)
            .cast::<*mut u8>())
        .read())
        .cast::<u8>())
        .wrapping_offset(6))
        .read()) as i32)
            < 4i32
        {
            ((((((&raw mut gRfuStatic).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(10))
                .cast::<u8>())
            .wrapping_offset(
                (((((((((&raw mut gRfuFixed).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(220)
                    .cast::<*mut u8>())
                .read())
                .cast::<u8>())
                .wrapping_offset(6))
                .read()) as i32) as isize,
            ))
            .write(0u8);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rfu_syncVBlank() -> u16 {
    unsafe {
        let mut masterSlave: u8 = 0u8;
        let mut i: u8 = 0u8;
        let mut bmSlotFlag: i32 = 0i32;
        rfu_NI_checkCommFailCounter();
        if (((((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>()).read()).read()) as i32)
            == 255i32
        {
            return 0u16;
        }
        if ((((((&raw mut gRfuStatic).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(6))
            .read()) as i32)
            != 0i32
        {
            let __p1 =
                (((&raw mut gRfuStatic).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(6);
            (__p1).write(((__p1).read()).wrapping_sub(1));
        }
        masterSlave = rfu_getMasterSlave();
        if !(((((((&raw mut gRfuStatic).cast::<u8>().cast::<*mut u8>()).read()).read()) as i32)
            & 2i32)
            != 0)
        {
            if ((masterSlave) as i32) == 0i32 {
                let __p2 = (((&raw mut gRfuStatic).cast::<u8>().cast::<*mut u8>()).read());
                (__p2).write((((((__p2).read()) as i32) | 4i32) as u8));
                ((((&raw mut gRfuStatic).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(32)
                    .cast::<u16>())
                .write(360u16);
            }
        } else {
            if ((masterSlave) as i32) != 0i32 {
                let __p3 = (((&raw mut gRfuStatic).cast::<u8>().cast::<*mut u8>()).read());
                (__p3).write((((((__p3).read()) as i32) & 251i32) as u8));
            }
        }
        if ((masterSlave) as i32) != 0i32 {
            let __p4 = (((&raw mut gRfuStatic).cast::<u8>().cast::<*mut u8>()).read());
            (__p4).write((((((__p4).read()) as i32) & 253i32) as u8));
        } else {
            let __p5 = (((&raw mut gRfuStatic).cast::<u8>().cast::<*mut u8>()).read());
            (__p5).write((((((__p5).read()) as i32) | 2i32) as u8));
        }
        if !(((((((&raw mut gRfuStatic).cast::<u8>().cast::<*mut u8>()).read()).read()) as i32)
            & 4i32)
            != 0)
        {
            return 0u16;
        }
        if ((((((&raw mut gRfuStatic).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(32)
            .cast::<u16>())
        .read()) as i32)
            == 0i32
        {
            let __p6 = (((&raw mut gRfuStatic).cast::<u8>().cast::<*mut u8>()).read());
            (__p6).write((((((__p6).read()) as i32) & 251i32) as u8));
            bmSlotFlag = (((((((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(2))
            .read()) as i32)
                | ((((((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(3))
                .read()) as i32));
            {
                i = 0u8;
                'l1: loop {
                    if !(((i) as i32) < 4i32) {
                        break 'l1;
                    }
                    'l2: {
                        if (crate::c::shr_i32(bmSlotFlag, ((i) as u32)) & 1i32) != 0 {
                            rfu_STC_removeLinkData(i, 1u8);
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
            (((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>()).read()).write(255u8);
            return 1u16;
        }
        let __p7 = (((&raw mut gRfuStatic).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(32)
            .cast::<u16>();
        (__p7).write(((__p7).read()).wrapping_sub(1));
        return 0u16;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rfu_REQBN_watchLink(
    reqCommandId: u16,
    bmLinkLossSlot: *mut u8,
    linkLossReason: *mut u8,
    parentBmLinkRecoverySlot: *mut u8,
) -> u16 {
    unsafe {
        let mut reqCommandId = reqCommandId;
        let mut bmLinkLossSlot = bmLinkLossSlot;
        let mut linkLossReason = linkLossReason;
        let mut parentBmLinkRecoverySlot = parentBmLinkRecoverySlot;
        let mut reasonMaybe: u8 = 0u8;
        let mut reqResult: u8 = 0u8;
        let mut i: u8 = 0u8;
        let mut stwiCommand: i32 = 0i32;
        let mut stwiParam: i32 = 0i32;
        let mut packet_p: *mut u8 = core::ptr::null_mut();
        let mut to_req_disconnect: u8 = 0u8;
        let mut newLinkLossFlag: u8 = 0u8;
        let mut num_packets: u8 = 0u8;
        let mut connSlotFlag: u8 = 0u8;
        let mut to_disconnect: u8 = 0u8;
        (bmLinkLossSlot).write(0u8);
        (linkLossReason).write(0u8);
        (parentBmLinkRecoverySlot).write(0u8);
        if ((((((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>()).read()).read()) as i32)
            == 255i32)
            || (((((((&raw mut gSTWIStatus).cast::<*mut u8>()).read()).wrapping_add(20))
                .read_volatile()) as i32)
                == 0i32)
        {
            return 0u16;
        }
        if ((((((&raw mut gRfuStatic).cast::<u8>().cast::<*mut u8>()).read()).read()) as i32)
            & 4i32)
            != 0
        {
            ((((&raw mut gRfuStatic).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(32)
                .cast::<u16>())
            .write(360u16);
        }
        if ((((((&raw mut gRfuStatic).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(6))
            .read()) as i32)
            == 0i32
        {
            ((((&raw mut gRfuStatic).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(6))
                .write(4u8);
            reasonMaybe = 1u8;
        }
        if (((reqCommandId) as u8) as i32) == 41i32 {
            let mut packet_p_2: *mut u8 =
                (((((&raw mut gRfuFixed).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(220)
                    .cast::<*mut u8>())
                .read())
                .cast::<u8>();
            (bmLinkLossSlot).write(((packet_p_2).wrapping_offset(4)).read());
            (linkLossReason).write(((packet_p_2).wrapping_offset(5)).read());
            if (((linkLossReason).read()) as i32) == 1i32 {
                (bmLinkLossSlot).write(
                    ((((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(2))
                    .read(),
                );
            }
            reasonMaybe = 2u8;
        } else {
            if ((reqCommandId) as i32) == 310i32 {
                newLinkLossFlag = (((((((&raw mut gRfuFixed).cast::<u8>().cast::<*mut u8>())
                    .read())
                .wrapping_add(220)
                .cast::<*mut u8>())
                .read())
                .cast::<u8>())
                .wrapping_offset(5))
                .read();
                newLinkLossFlag = ((((newLinkLossFlag) as i32)
                    ^ ((((((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(2))
                    .read()) as i32)) as u8);
                (bmLinkLossSlot).write(
                    ((((newLinkLossFlag) as i32)
                        & ((((((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(2))
                        .read()) as i32)) as u8),
                );
                (linkLossReason).write(1u8);
                {
                    i = 0u8;
                    'l1: loop {
                        if !(((i) as i32) < 4i32) {
                            break 'l1;
                        }
                        'l2: {
                            if (crate::c::shr_i32((((bmLinkLossSlot).read()) as i32), ((i) as u32))
                                & 1i32)
                                != 0
                            {
                                ((((((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(10))
                                .cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize))
                                .write(0u8);
                                rfu_STC_removeLinkData(i, 0u8);
                            }
                        }
                        i = (i).wrapping_add(1);
                    }
                }
            }
            if ((reasonMaybe) as i32) == 0i32 {
                return 0u16;
            }
        }
        stwiCommand = ((((((((&raw mut gRfuFixed).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(220)
            .cast::<*mut u8>())
        .read())
        .cast::<u32>())
        .read()) as i32);
        stwiParam = (((((((((&raw mut gRfuFixed).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(220)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(4))
        .cast::<u32>())
        .read()) as i32);
        STWI_set_Callback_M(core::mem::transmute::<
            Option<unsafe extern "C" fn(u8, u16)>,
            *mut u8,
        >(Some(
            rfu_CB_defaultCallback as unsafe extern "C" fn(u8, u16),
        )));
        STWI_send_LinkStatusREQ();
        reqResult = ((STWI_poll_CommandEnd()) as u8);
        if ((reqResult) as i32) == 0i32 {
            packet_p = ((((((&raw mut gRfuFixed).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(220)
                .cast::<*mut u8>())
            .read())
            .cast::<u8>())
            .wrapping_offset(4);
            {
                i = 0u8;
                'l3: loop {
                    if !(((i) as i32) < 4i32) {
                        break 'l3;
                    }
                    'l4: {
                        ((((((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(10))
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize))
                        .write(
                            ({
                                let __t2 = packet_p;
                                packet_p = (packet_p).wrapping_offset(1);
                                __t2
                            })
                            .read(),
                        );
                    }
                    i = (i).wrapping_add(1);
                }
            }
            to_req_disconnect = 0u8;
            i = 0u8;
        } else {
            rfu_STC_REQ_callback(17u8, ((reqResult) as u16));
            return ((reqResult) as u16);
        }
        {
            'l5: loop {
                if !(((i) as i32) < 4i32) {
                    break 'l5;
                }
                'l6: {
                    if ((((((((&raw mut gRfuStatic).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(14))
                    .cast::<u8>())
                    .wrapping_offset(((i) as i32) as isize))
                    .read()) as i32)
                        != 0i32
                    {
                        let __p3 = (((((&raw mut gRfuStatic).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(14))
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize);
                        (__p3).write((((((__p3).read()) as i32).wrapping_sub(4i32)) as u8));
                        if ((((((((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(10))
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize))
                        .read()) as i32)
                            <= 15i32
                        {
                            ((((((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(10))
                            .cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize))
                            .write(16u8);
                        }
                    }
                    newLinkLossFlag = ((crate::c::shl_i32(1i32, ((i) as u32))) as u8);
                    if ((reqResult) as i32) == 0i32 {
                        if (((reasonMaybe) as i32) == 1i32)
                            && ((((((((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(2))
                            .read()) as i32)
                                & ((newLinkLossFlag) as i32))
                                != 0)
                        {
                            if ((((((((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(10))
                            .cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize))
                            .read()) as i32)
                                == 0i32
                            {
                                if (((((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .read()) as i32)
                                    == 1i32
                                {
                                    let __p4 = (((((&raw mut gRfuStatic)
                                        .cast::<u8>()
                                        .cast::<*mut u8>())
                                    .read())
                                    .wrapping_add(10))
                                    .cast::<u8>())
                                    .wrapping_offset(((i) as i32) as isize);
                                    (__p4).write(((__p4).read()).wrapping_add(1));
                                    if ((((((((&raw mut gRfuStatic)
                                        .cast::<u8>()
                                        .cast::<*mut u8>())
                                    .read())
                                    .wrapping_add(10))
                                    .cast::<u8>())
                                    .wrapping_offset(((i) as i32) as isize))
                                    .read()) as i32)
                                        > 3i32
                                    {
                                        (bmLinkLossSlot).write(
                                            (((((bmLinkLossSlot).read()) as i32)
                                                | ((newLinkLossFlag) as i32))
                                                as u8),
                                        );
                                        (linkLossReason).write(1u8);
                                    }
                                } else {
                                    STWI_send_SystemStatusREQ();
                                    if ((STWI_poll_CommandEnd()) as i32) == 0i32 {
                                        if (((((((((&raw mut gRfuFixed)
                                            .cast::<u8>()
                                            .cast::<*mut u8>())
                                        .read())
                                        .wrapping_add(220)
                                        .cast::<*mut u8>())
                                        .read())
                                        .cast::<u8>())
                                        .wrapping_offset(7))
                                        .read()) as i32)
                                            == 0i32
                                        {
                                            (bmLinkLossSlot).write(
                                                (((((bmLinkLossSlot).read()) as i32)
                                                    | ((newLinkLossFlag) as i32))
                                                    as u8),
                                            );
                                            (linkLossReason).write(1u8);
                                        } else {
                                            if (({
                                                let __p5 = (((((&raw mut gRfuStatic)
                                                    .cast::<u8>()
                                                    .cast::<*mut u8>())
                                                .read())
                                                .wrapping_add(10))
                                                .cast::<u8>())
                                                .wrapping_offset(((i) as i32) as isize);
                                                let __t6 = ((__p5).read()).wrapping_add(1);
                                                (__p5).write(__t6);
                                                __t6
                                            })
                                                as i32)
                                                > ((((((&raw mut gRfuStatic)
                                                    .cast::<u8>()
                                                    .cast::<*mut u8>())
                                                .read())
                                                .wrapping_add(26)
                                                .cast::<u16>())
                                                .read())
                                                    as i32)
                                            {
                                                ((((((&raw mut gRfuStatic)
                                                    .cast::<u8>()
                                                    .cast::<*mut u8>())
                                                .read())
                                                .wrapping_add(10))
                                                .cast::<u8>())
                                                .wrapping_offset(((i) as i32) as isize))
                                                .write(0u8);
                                                STWI_send_DisconnectREQ(
                                                    ((((&raw mut gRfuLinkStatus)
                                                        .cast::<u8>()
                                                        .cast::<*mut u8>())
                                                    .read())
                                                    .wrapping_add(2))
                                                    .read(),
                                                );
                                                STWI_poll_CommandEnd();
                                                (bmLinkLossSlot).write(
                                                    (((((bmLinkLossSlot).read()) as i32)
                                                        | ((newLinkLossFlag) as i32))
                                                        as u8),
                                                );
                                                (linkLossReason).write(1u8);
                                            }
                                        }
                                    }
                                }
                            } else {
                                ((((((&raw mut gRfuStatic).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(10))
                                .cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize))
                                .write(0u8);
                            }
                        }
                        if ((((((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>()).read())
                            .read()) as i32)
                            == 1i32)
                            && (((((((((&raw mut gRfuLinkStatus)
                                .cast::<u8>()
                                .cast::<*mut u8>())
                            .read())
                            .wrapping_add(10))
                            .cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize))
                            .read()) as i32)
                                != 0i32)
                        {
                            if (((newLinkLossFlag) as i32)
                                & ((((((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(3))
                                .read()) as i32))
                                != 0
                            {
                                if ((((((((&raw mut gRfuLinkStatus)
                                    .cast::<u8>()
                                    .cast::<*mut u8>())
                                .read())
                                .wrapping_add(10))
                                .cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize))
                                .read()) as i32)
                                    > 10i32
                                {
                                    (parentBmLinkRecoverySlot).write(
                                        (((((parentBmLinkRecoverySlot).read()) as i32)
                                            | ((newLinkLossFlag) as i32))
                                            as u8),
                                    );
                                    let __p7 = (((&raw mut gRfuLinkStatus)
                                        .cast::<u8>()
                                        .cast::<*mut u8>())
                                    .read())
                                    .wrapping_add(2);
                                    (__p7).write(
                                        (((((__p7).read()) as i32) | ((newLinkLossFlag) as i32))
                                            as u8),
                                    );
                                    let __p8 = (((&raw mut gRfuLinkStatus)
                                        .cast::<u8>()
                                        .cast::<*mut u8>())
                                    .read())
                                    .wrapping_add(3);
                                    (__p8).write(
                                        (((((__p8).read()) as i32) & !((newLinkLossFlag) as i32))
                                            as u8),
                                    );
                                    let __p9 = (((&raw mut gRfuLinkStatus)
                                        .cast::<u8>()
                                        .cast::<*mut u8>())
                                    .read())
                                    .wrapping_add(1);
                                    (__p9).write(((__p9).read()).wrapping_add(1));
                                    ((((((&raw mut gRfuStatic).cast::<u8>().cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(10))
                                    .cast::<u8>())
                                    .wrapping_offset(((i) as i32) as isize))
                                    .write(0u8);
                                } else {
                                    ((((((&raw mut gRfuLinkStatus)
                                        .cast::<u8>()
                                        .cast::<*mut u8>())
                                    .read())
                                    .wrapping_add(10))
                                    .cast::<u8>())
                                    .wrapping_offset(((i) as i32) as isize))
                                    .write(0u8);
                                }
                            } else {
                                if !(((((((((&raw mut gRfuLinkStatus)
                                    .cast::<u8>()
                                    .cast::<*mut u8>())
                                .read())
                                .wrapping_add(2))
                                .read()) as i32)
                                    | ((((((&raw mut gRfuLinkStatus)
                                        .cast::<u8>()
                                        .cast::<*mut u8>())
                                    .read())
                                    .wrapping_add(3))
                                    .read()) as i32))
                                    & ((newLinkLossFlag) as i32))
                                    != 0)
                                {
                                    STWI_send_SlotStatusREQ();
                                    STWI_poll_CommandEnd();
                                    packet_p =
                                        (((((&raw mut gRfuFixed).cast::<u8>().cast::<*mut u8>())
                                            .read())
                                        .wrapping_add(220)
                                        .cast::<*mut u8>())
                                        .read())
                                        .cast::<u8>();
                                    num_packets = ((((((packet_p).wrapping_offset(1)).read())
                                        as i32)
                                        .wrapping_sub(1i32))
                                        as u8);
                                    {
                                        packet_p = (packet_p).wrapping_offset(8);
                                        'l7: loop {
                                            if !(((num_packets) as i32) != 0i32) {
                                                break 'l7;
                                            }
                                            'l8: {
                                                let mut cid: u16 =
                                                    ((packet_p).cast::<u16>()).read();
                                                if (((((packet_p).wrapping_offset(2)).read())
                                                    as i32)
                                                    == ((i) as i32))
                                                    && (((cid) as i32)
                                                        == ((((((((&raw mut gRfuStatic)
                                                            .cast::<u8>()
                                                            .cast::<*mut u8>())
                                                        .read())
                                                        .wrapping_add(18))
                                                        .cast::<u16>())
                                                        .wrapping_offset(((i) as i32) as isize))
                                                        .read())
                                                            as i32))
                                                {
                                                    to_req_disconnect = ((((to_req_disconnect)
                                                        as i32)
                                                        | crate::c::shl_i32(1i32, ((i) as u32)))
                                                        as u8);
                                                    break 'l7;
                                                }
                                            }
                                            packet_p = (packet_p).wrapping_offset(4);
                                            num_packets = (num_packets).wrapping_sub(1);
                                        }
                                    }
                                }
                            }
                        }
                    }
                    connSlotFlag = ((((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>())
                        .read())
                    .wrapping_add(2))
                    .read();
                    to_disconnect = (bmLinkLossSlot).read();
                    to_disconnect = ((((to_disconnect) as i32) & ((connSlotFlag) as i32)) as u8);
                    if (((newLinkLossFlag) as i32) & ((to_disconnect) as i32)) != 0 {
                        rfu_STC_removeLinkData(i, 0u8);
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        if ((to_req_disconnect) as i32) != 0i32 {
            STWI_send_DisconnectREQ(to_req_disconnect);
            STWI_poll_CommandEnd();
        }
        (((((((&raw mut gRfuFixed).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(220)
            .cast::<*mut u8>())
        .read())
        .cast::<u8>())
        .cast::<u32>())
        .write(((stwiCommand) as u32));
        (((((((&raw mut gRfuFixed).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(220)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(4))
        .cast::<u32>())
        .write(((stwiParam) as u32));
        return 0u16;
    }
}
pub(crate) unsafe extern "C" fn rfu_STC_removeLinkData(bmConnectedPartnerId: u8, bmDisconnect: u8) {
    unsafe {
        let mut bmConnectedPartnerId = bmConnectedPartnerId;
        let mut bmDisconnect = bmDisconnect;
        let mut bmLinkLossFlag: u8 =
            ((crate::c::shl_i32(1i32, ((bmConnectedPartnerId) as u32))) as u8);
        let mut bmLinkRetainedFlag: i32 = 0i32;
        ((((((&raw mut gRfuStatic).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(14))
            .cast::<u8>())
        .wrapping_offset(((bmConnectedPartnerId) as i32) as isize))
        .write(0u8);
        if ((((((((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(2))
        .read()) as i32)
            & ((bmLinkLossFlag) as i32))
            != 0)
            && (((((((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(1))
            .read()) as i32)
                != 0i32)
        {
            let __p1 =
                (((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1);
            (__p1).write(((__p1).read()).wrapping_sub(1));
        }
        let __p2 =
            (((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(2);
        (__p2).write(
            (((((__p2).read()) as i32) & {
                let __v3 = !((bmLinkLossFlag) as i32);
                bmLinkRetainedFlag = __v3;
                __v3
            }) as u8),
        );
        let __p4 =
            (((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(3);
        (__p4).write((((((__p4).read()) as i32) | ((bmLinkLossFlag) as i32)) as u8));
        if ((((((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>()).read()).read()) as i32)
            == 0i32)
            && (((((((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(2))
            .read()) as i32)
                == 0i32)
        {
            (((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>()).read()).write(255u8);
        }
        if (bmDisconnect) != 0 {
            'l1: loop {
                'l2: {
                    {
                        let mut tmp: u16 = 0u16;
                        (&raw mut tmp).write_volatile(0u16);
                        'l3: loop {
                            'l4: {
                                CpuSet(
                                    (&raw mut tmp).cast::<u8>(),
                                    (((((&raw mut gRfuLinkStatus)
                                        .cast::<u8>()
                                        .cast::<*mut u8>())
                                    .read())
                                    .wrapping_add(20))
                                    .cast::<u8>())
                                    .wrapping_offset(((bmConnectedPartnerId) as i32) as isize * 32),
                                    (16777216u32
                                        | (crate::c::div_u32(
                                            32u32,
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
            let __p5 =
                (((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(3);
            (__p5).write((((((__p5).read()) as i32) & bmLinkRetainedFlag) as u8));
            let __p6 =
                (((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(7);
            (__p6).write((((((__p6).read()) as i32) & bmLinkRetainedFlag) as u8));
            ((((((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(10))
            .cast::<u8>())
            .wrapping_offset(((bmConnectedPartnerId) as i32) as isize))
            .write(0u8);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rfu_REQ_disconnect(bmDisconnectSlot: u8) {
    unsafe {
        let mut bmDisconnectSlot = bmDisconnectSlot;
        let mut result: u16 = 0u16;
        if ((((((((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(2))
        .read()) as i32)
            | ((((((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(3))
            .read()) as i32))
            & ((bmDisconnectSlot) as i32))
            != 0
        {
            ((((&raw mut gRfuStatic).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(5))
                .write(bmDisconnectSlot);
            if ((((((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>()).read()).read())
                as i32)
                == 255i32)
                && (((((((&raw mut gRfuStatic).cast::<u8>().cast::<*mut u8>()).read()).read())
                    as i32)
                    & 128i32)
                    != 0)
            {
                if (((((((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(3))
                .read()) as i32)
                    & ((bmDisconnectSlot) as i32))
                    != 0
                {
                    rfu_CB_disconnect(48u8, 0u16);
                }
            } else {
                if ((((((&raw mut gRfuStatic).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(9))
                .read())
                    != 0)
                    && ({
                        let _ = STWI_set_Callback_M(core::mem::transmute::<
                            Option<unsafe extern "C" fn(u8, u16)>,
                            *mut u8,
                        >(Some(
                            rfu_CB_defaultCallback as unsafe extern "C" fn(u8, u16),
                        )));
                        let _ = STWI_send_SC_EndREQ();
                        (({
                            let __v1 = STWI_poll_CommandEnd();
                            result = __v1;
                            __v1
                        }) as i32)
                            != 0i32
                    })
                {
                    rfu_STC_REQ_callback(27u8, result);
                } else {
                    STWI_set_Callback_M(core::mem::transmute::<
                        Option<unsafe extern "C" fn(u8, u16)>,
                        *mut u8,
                    >(Some(
                        rfu_CB_disconnect as unsafe extern "C" fn(u8, u16),
                    )));
                    STWI_send_DisconnectREQ(bmDisconnectSlot);
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn rfu_CB_disconnect(reqCommand: u8, reqResult: u16) {
    unsafe {
        let mut reqCommand = reqCommand;
        let mut reqResult = reqResult;
        let mut i: u8 = 0u8;
        let mut bm_slot_flag: u8 = 0u8;
        if (((reqResult) as i32) == 3i32)
            && ((((((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>()).read()).read())
                as i32)
                == 0i32)
        {
            STWI_set_Callback_M(core::mem::transmute::<
                Option<unsafe extern "C" fn(u8, u16)>,
                *mut u8,
            >(Some(
                rfu_CB_defaultCallback as unsafe extern "C" fn(u8, u16),
            )));
            STWI_send_SystemStatusREQ();
            if (((STWI_poll_CommandEnd()) as i32) == 0i32)
                && ((((((((((&raw mut gRfuFixed).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(220)
                    .cast::<*mut u8>())
                .read())
                .cast::<u8>())
                .wrapping_offset(7))
                .read()) as i32)
                    == 0i32)
            {
                reqResult = 0u16;
            }
        }
        let __p1 = (((&raw mut gRfuStatic).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(5);
        (__p1).write(
            (((((__p1).read()) as i32)
                & (((((((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(2))
                .read()) as i32)
                    | ((((((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(3))
                    .read()) as i32))) as u8),
        );
        (((((((&raw mut gRfuFixed).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(220)
            .cast::<*mut u8>())
        .read())
        .cast::<u8>())
        .wrapping_offset(8))
        .write(
            ((((&raw mut gRfuStatic).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(5))
                .read(),
        );
        if ((reqResult) as i32) == 0i32 {
            {
                i = 0u8;
                'l1: loop {
                    if !(((i) as i32) < 4i32) {
                        break 'l1;
                    }
                    'l2: {
                        bm_slot_flag = ((crate::c::shl_i32(1i32, ((i) as u32))) as u8);
                        if (((bm_slot_flag) as i32)
                            & ((((((&raw mut gRfuStatic).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(5))
                            .read()) as i32))
                            != 0
                        {
                            rfu_STC_removeLinkData(i, 1u8);
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
        }
        if (((((((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(2))
            .read()) as i32)
            | ((((((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(3))
            .read()) as i32))
            == 0i32
        {
            (((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>()).read()).write(255u8);
        }
        rfu_STC_REQ_callback(reqCommand, reqResult);
        if (((((&raw mut gRfuStatic).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(9))
            .read())
            != 0
        {
            STWI_set_Callback_M(core::mem::transmute::<
                Option<unsafe extern "C" fn(u8, u16)>,
                *mut u8,
            >(Some(
                rfu_CB_defaultCallback as unsafe extern "C" fn(u8, u16),
            )));
            STWI_send_SC_StartREQ();
            reqResult = STWI_poll_CommandEnd();
            if ((reqResult) as i32) != 0i32 {
                rfu_STC_REQ_callback(25u8, reqResult);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rfu_REQ_CHILD_startConnectRecovery(bmRecoverySlot: u8) {
    unsafe {
        let mut bmRecoverySlot = bmRecoverySlot;
        let mut i: u8 = 0u8;
        ((((&raw mut gRfuStatic).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(5))
            .write(bmRecoverySlot);
        {
            i = 0u8;
            'l1: loop {
                if !((((i) as i32) < 4i32)
                    && (!((crate::c::shr_i32(((bmRecoverySlot) as i32), ((i) as u32)) & 1i32)
                        != 0)))
                {
                    break 'l1;
                }
                'l2: {}
                i = (i).wrapping_add(1);
            }
        }
        STWI_set_Callback_M(core::mem::transmute::<
            Option<unsafe extern "C" fn(u8, u16)>,
            *mut u8,
        >(Some(
            rfu_STC_REQ_callback as unsafe extern "C" fn(u8, u16),
        )));
        STWI_send_CPR_StartREQ(
            (((((((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(20))
            .cast::<u8>())
            .wrapping_offset(((i) as i32) as isize * 32))
            .cast::<u16>())
            .read(),
            (((((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(148))
            .cast::<u16>())
            .read(),
            bmRecoverySlot,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rfu_REQ_CHILD_pollConnectRecovery() {
    unsafe {
        STWI_set_Callback_M(core::mem::transmute::<
            Option<unsafe extern "C" fn(u8, u16)>,
            *mut u8,
        >(Some(
            rfu_CB_CHILD_pollConnectRecovery as unsafe extern "C" fn(u8, u16),
        )));
        STWI_send_CPR_PollingREQ();
    }
}
pub(crate) unsafe extern "C" fn rfu_CB_CHILD_pollConnectRecovery(reqCommand: u8, reqResult: u16) {
    unsafe {
        let mut reqCommand = reqCommand;
        let mut reqResult = reqResult;
        let mut bm_slot_flag: u8 = 0u8;
        let mut i: u8 = 0u8;
        let mut rfuLinkStatus: *mut u8 = core::ptr::null_mut();
        if ((((reqResult) as i32) == 0i32)
            && ((((((((((&raw mut gRfuFixed).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(220)
                .cast::<*mut u8>())
            .read())
            .cast::<u8>())
            .wrapping_offset(4))
            .read()) as i32)
                == 0i32))
            && ((((((&raw mut gRfuStatic).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(5))
                .read())
                != 0)
        {
            (((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>()).read()).write(0u8);
            {
                i = 0u8;
                'l1: loop {
                    if !(((i) as i32) < 4i32) {
                        break 'l1;
                    }
                    'l2: {
                        bm_slot_flag = ((crate::c::shl_i32(1i32, ((i) as u32))) as u8);
                        rfuLinkStatus =
                            ((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>()).read();
                        if ((((((((&raw mut gRfuStatic).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(5))
                        .read()) as i32)
                            & ((bm_slot_flag) as i32))
                            & ((((rfuLinkStatus).wrapping_add(3)).read()) as i32))
                            != 0
                        {
                            let __p1 = (((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(2);
                            (__p1).write(
                                (((((__p1).read()) as i32) | ((bm_slot_flag) as i32)) as u8),
                            );
                            let __p2 = (((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(3);
                            (__p2).write(
                                (((((__p2).read()) as i32) & !((bm_slot_flag) as i32)) as u8),
                            );
                            let __p3 = (((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(1);
                            (__p3).write(((__p3).read()).wrapping_add(1));
                            ((((((&raw mut gRfuStatic).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(10))
                            .cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize))
                            .write(0u8);
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
            ((((&raw mut gRfuStatic).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(5))
                .write(0u8);
        }
        rfu_STC_REQ_callback(reqCommand, reqResult);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rfu_CHILD_getConnectRecoveryStatus(status: *mut u8) -> u16 {
    unsafe {
        let mut status = status;
        (status).write(255u8);
        if (((((((((&raw mut gRfuFixed).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(220)
            .cast::<*mut u8>())
        .read())
        .cast::<u8>())
        .read()) as i32)
            == 179i32)
            || (((((((((&raw mut gRfuFixed).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(220)
                .cast::<*mut u8>())
            .read())
            .cast::<u8>())
            .read()) as i32)
                == 180i32)
        {
            (status).write(
                (((((((&raw mut gRfuFixed).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(220)
                    .cast::<*mut u8>())
                .read())
                .cast::<u8>())
                .wrapping_offset(4))
                .read(),
            );
            return 0u16;
        }
        return 16u16;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rfu_REQ_CHILD_endConnectRecovery() {
    unsafe {
        STWI_set_Callback_M(core::mem::transmute::<
            Option<unsafe extern "C" fn(u8, u16)>,
            *mut u8,
        >(Some(
            rfu_CB_CHILD_pollConnectRecovery as unsafe extern "C" fn(u8, u16),
        )));
        STWI_send_CPR_EndREQ();
    }
}
// hand-written: tools/rustport/overrides/librfu_rfu/rfu_STC_fastCopy.rs
/// `rfu_STC_fastCopy`. rfu_initializeAPI copies this function's code (0x60
/// bytes) into gRfuFixed->fastCopyBuffer and calls the copy, so it must not
/// call anything: the volatile reads keep the loop from becoming a memcpy
/// call. rustcheck.sh checks its size and that it makes no calls.
#[unsafe(no_mangle)]
unsafe extern "C" fn rfu_STC_fastCopy(src_p: *mut *mut u8, dst_p: *mut *mut u8, size: i32) {
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

#[unsafe(no_mangle)]
pub unsafe extern "C" fn rfu_REQ_changeMasterSlave() {
    unsafe {
        if ((STWI_read_status(1u8)) as i32) == 1i32 {
            STWI_set_Callback_M(core::mem::transmute::<
                Option<unsafe extern "C" fn(u8, u16)>,
                *mut u8,
            >(Some(
                rfu_STC_REQ_callback as unsafe extern "C" fn(u8, u16),
            )));
            STWI_send_MS_ChangeREQ();
        } else {
            rfu_STC_REQ_callback(39u8, 0u16);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rfu_getMasterSlave() -> u8 {
    unsafe {
        let mut masterSlave: u8 = ((STWI_read_status(1u8)) as u8);
        if ((masterSlave) as i32) == 1i32 {
            if (((((&raw mut gSTWIStatus).cast::<*mut u8>()).read()).wrapping_add(44))
                .read_volatile())
                != 0
            {
                if ((((((((&raw mut gSTWIStatus).cast::<*mut u8>()).read()).wrapping_add(6)).read())
                    as i32)
                    == 39i32)
                    || (((((((&raw mut gSTWIStatus).cast::<*mut u8>()).read()).wrapping_add(6))
                        .read()) as i32)
                        == 37i32))
                    || (((((((&raw mut gSTWIStatus).cast::<*mut u8>()).read()).wrapping_add(6))
                        .read()) as i32)
                        == 55i32)
                {
                    masterSlave = 0u8;
                }
            }
        }
        return masterSlave;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rfu_clearAllSlot() {
    unsafe {
        let mut i: u16 = 0u16;
        let mut IMEBackup: u16 = ((67109384i32) as usize as *mut u16).read_volatile();
        crate::c::volatile_write(((67109384i32) as usize as *mut u16), 0u16);
        {
            i = 0u16;
            'l1: loop {
                if !(((i) as i32) < 4i32) {
                    break 'l1;
                }
                'l2: {
                    'l3: loop {
                        'l4: {
                            {
                                let mut tmp: u16 = 0u16;
                                (&raw mut tmp).write_volatile(0u16);
                                'l5: loop {
                                    'l6: {
                                        CpuSet(
                                            (&raw mut tmp).cast::<u8>(),
                                            ((((&raw mut gRfuSlotStatusNI)
                                                .cast::<u8>()
                                                .cast::<*mut u8>())
                                            .cast::<*mut u8>())
                                            .wrapping_offset(((i) as i32) as isize))
                                            .read(),
                                            (16777216u32
                                                | (crate::c::div_u32(
                                                    104u32,
                                                    ((crate::c::div_i32(16i32, 8i32)) as u32),
                                                ) & 2097151u32)),
                                        );
                                    }
                                    if !((0i32) != 0) {
                                        break 'l5;
                                    }
                                }
                            }
                        }
                        if !((0i32) != 0) {
                            break 'l3;
                        }
                    }
                    'l7: loop {
                        'l8: {
                            {
                                let mut tmp: u16 = 0u16;
                                (&raw mut tmp).write_volatile(0u16);
                                'l9: loop {
                                    'l10: {
                                        CpuSet(
                                            (&raw mut tmp).cast::<u8>(),
                                            ((((&raw mut gRfuSlotStatusUNI)
                                                .cast::<u8>()
                                                .cast::<*mut u8>())
                                            .cast::<*mut u8>())
                                            .wrapping_offset(((i) as i32) as isize))
                                            .read(),
                                            (16777216u32
                                                | (crate::c::div_u32(
                                                    20u32,
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
                    ((((((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(16))
                    .cast::<u8>())
                    .wrapping_offset(((i) as i32) as isize))
                    .write(16u8);
                }
                i = (i).wrapping_add(1);
            }
        }
        ((((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(15))
            .write(87u8);
        ((((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(4))
            .write(0u8);
        ((((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(5))
            .write(0u8);
        ((((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(6))
            .write(0u8);
        ((((&raw mut gRfuStatic).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(2))
            .write(0u8);
        crate::c::volatile_write(((67109384i32) as usize as *mut u16), IMEBackup);
    }
}
pub(crate) unsafe extern "C" fn rfu_STC_releaseFrame(
    bm_slot_id: u8,
    send_recv: u8,
    NI_comm: *mut u8,
) {
    unsafe {
        let mut bm_slot_id = bm_slot_id;
        let mut send_recv = send_recv;
        let mut NI_comm = NI_comm;
        if !(((((((&raw mut gRfuStatic).cast::<u8>().cast::<*mut u8>()).read()).read()) as i32)
            & 128i32)
            != 0)
        {
            if ((send_recv) as i32) == 0i32 {
                let __p1 = (((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(15);
                (__p1).write(
                    (((((__p1).read()) as i32)
                        .wrapping_add(((((NI_comm).wrapping_add(46).cast::<u16>()).read()) as i32)))
                        as u8),
                );
            }
            let __p2 = (((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(15);
            (__p2).write((((((__p2).read()) as i32).wrapping_add(3i32)) as u8));
        } else {
            if ((send_recv) as i32) == 0i32 {
                let __p3 = (((((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(16))
                .cast::<u8>())
                .wrapping_offset(((bm_slot_id) as i32) as isize);
                (__p3).write(
                    (((((__p3).read()) as i32)
                        .wrapping_add(((((NI_comm).wrapping_add(46).cast::<u16>()).read()) as i32)))
                        as u8),
                );
            }
            let __p4 = (((((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(16))
            .cast::<u8>())
            .wrapping_offset(((bm_slot_id) as i32) as isize);
            (__p4).write((((((__p4).read()) as i32).wrapping_add(2i32)) as u8));
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rfu_clearSlot(connTypeFlag: u8, slotStatusIndex: u8) -> u16 {
    unsafe {
        let mut connTypeFlag = connTypeFlag;
        let mut slotStatusIndex = slotStatusIndex;
        let mut imeBak: u16 = 0u16;
        let mut send_recv: u16 = 0u16;
        let mut i: u16 = 0u16;
        let mut NI_comm: *mut u8 = core::ptr::null_mut();
        if ((slotStatusIndex) as i32) >= 4i32 {
            return 1024u16;
        }
        if !((((connTypeFlag) as i32) & 15i32) != 0) {
            return 1536u16;
        }
        imeBak = ((67109384i32) as usize as *mut u16).read_volatile();
        crate::c::volatile_write(((67109384i32) as usize as *mut u16), 0u16);
        if (((connTypeFlag) as i32) & 12i32) != 0 {
            {
                send_recv = 0u16;
                'l1: loop {
                    if !(((send_recv) as i32) < 2i32) {
                        break 'l1;
                    }
                    'l2: {
                        NI_comm = core::ptr::null_mut();
                        if ((send_recv) as i32) == 0i32 {
                            if (((connTypeFlag) as i32) & 4i32) != 0 {
                                NI_comm = (((((&raw mut gRfuSlotStatusNI)
                                    .cast::<u8>()
                                    .cast::<*mut u8>())
                                .cast::<*mut u8>())
                                .wrapping_offset(((slotStatusIndex) as i32) as isize))
                                .read());
                                let __p1 =
                                    (((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(4);
                                (__p1).write(
                                    (((((__p1).read()) as i32)
                                        & !((((NI_comm).wrapping_add(44)).read()) as i32))
                                        as u8),
                                );
                            }
                        } else {
                            if (((connTypeFlag) as i32) & 8i32) != 0 {
                                NI_comm = (((((&raw mut gRfuSlotStatusNI)
                                    .cast::<u8>()
                                    .cast::<*mut u8>())
                                .cast::<*mut u8>())
                                .wrapping_offset(((slotStatusIndex) as i32) as isize))
                                .read())
                                .wrapping_add(52);
                                let __p2 =
                                    (((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(5);
                                (__p2).write(
                                    (((((__p2).read()) as i32)
                                        & !(crate::c::shl_i32(1i32, ((slotStatusIndex) as u32))))
                                        as u8),
                                );
                            }
                        }
                        if ((NI_comm) as usize) != 0usize {
                            if (((((NI_comm).cast::<u16>()).read()) as i32) & 32768i32) != 0 {
                                rfu_STC_releaseFrame(slotStatusIndex, ((send_recv) as u8), NI_comm);
                                {
                                    i = 0u16;
                                    'l3: loop {
                                        if !(((i) as i32) < 4i32) {
                                            break 'l3;
                                        }
                                        'l4: {
                                            if (crate::c::shr_i32(
                                                ((((NI_comm).wrapping_add(44)).read()) as i32),
                                                ((i) as u32),
                                            ) & 1i32)
                                                != 0
                                            {
                                                ((NI_comm).wrapping_add(2).cast::<u16>())
                                                    .write(0u16);
                                            }
                                        }
                                        i = (i).wrapping_add(1);
                                    }
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
                                                    NI_comm,
                                                    (16777216u32
                                                        | (crate::c::div_u32(
                                                            52u32,
                                                            ((crate::c::div_i32(16i32, 8i32))
                                                                as u32),
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
                    send_recv = (send_recv).wrapping_add(1);
                }
            }
        }
        if (((connTypeFlag) as i32) & 1i32) != 0 {
            let mut slotStatusUNI: *mut u8 =
                ((((&raw mut gRfuSlotStatusUNI).cast::<u8>().cast::<*mut u8>()).cast::<*mut u8>())
                    .wrapping_offset(((slotStatusIndex) as i32) as isize))
                .read();
            if (((((slotStatusUNI).cast::<u16>()).read()) as i32) & 32768i32) != 0 {
                if !(((((((&raw mut gRfuStatic).cast::<u8>().cast::<*mut u8>()).read()).read())
                    as i32)
                    & 128i32)
                    != 0)
                {
                    let __p3 = (((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(15);
                    (__p3).write(
                        (((((__p3).read()) as i32).wrapping_add((3i32).wrapping_add(
                            (((((slotStatusUNI).wrapping_add(4).cast::<u16>()).read()) as u8)
                                as i32),
                        ))) as u8),
                    );
                } else {
                    let __p4 = (((((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>())
                        .read())
                    .wrapping_add(16))
                    .cast::<u8>())
                    .wrapping_offset(((slotStatusIndex) as i32) as isize);
                    (__p4).write(
                        (((((__p4).read()) as i32).wrapping_add((2i32).wrapping_add(
                            (((((slotStatusUNI).wrapping_add(4).cast::<u16>()).read()) as u8)
                                as i32),
                        ))) as u8),
                    );
                }
                let __p5 = (((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(6);
                (__p5).write(
                    (((((__p5).read()) as i32)
                        & !((((slotStatusUNI).wrapping_add(3)).read()) as i32))
                        as u8),
                );
            }
            'l9: loop {
                'l10: {
                    {
                        let mut tmp: u16 = 0u16;
                        (&raw mut tmp).write_volatile(0u16);
                        'l11: loop {
                            'l12: {
                                CpuSet(
                                    (&raw mut tmp).cast::<u8>(),
                                    (slotStatusUNI),
                                    (16777216u32
                                        | (crate::c::div_u32(
                                            12u32,
                                            ((crate::c::div_i32(16i32, 8i32)) as u32),
                                        ) & 2097151u32)),
                                );
                            }
                            if !((0i32) != 0) {
                                break 'l11;
                            }
                        }
                    }
                }
                if !((0i32) != 0) {
                    break 'l9;
                }
            }
        }
        if (((connTypeFlag) as i32) & 2i32) != 0 {
            'l13: loop {
                'l14: {
                    {
                        let mut tmp: u16 = 0u16;
                        (&raw mut tmp).write_volatile(0u16);
                        'l15: loop {
                            'l16: {
                                CpuSet(
                                    (&raw mut tmp).cast::<u8>(),
                                    (((((&raw mut gRfuSlotStatusUNI)
                                        .cast::<u8>()
                                        .cast::<*mut u8>())
                                    .cast::<*mut u8>())
                                    .wrapping_offset(((slotStatusIndex) as i32) as isize))
                                    .read())
                                    .wrapping_add(12),
                                    (16777216u32
                                        | (crate::c::div_u32(
                                            8u32,
                                            ((crate::c::div_i32(16i32, 8i32)) as u32),
                                        ) & 2097151u32)),
                                );
                            }
                            if !((0i32) != 0) {
                                break 'l15;
                            }
                        }
                    }
                }
                if !((0i32) != 0) {
                    break 'l13;
                }
            }
        }
        crate::c::volatile_write(((67109384i32) as usize as *mut u16), imeBak);
        return 0u16;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rfu_setRecvBuffer(
    connType: u8,
    slotNo: u8,
    buffer: *mut u8,
    buffSize: u32,
) -> u16 {
    unsafe {
        let mut connType = connType;
        let mut slotNo = slotNo;
        let mut buffer = buffer;
        let mut buffSize = buffSize;
        if ((slotNo) as i32) >= 4i32 {
            return 1024u16;
        }
        if (((connType) as i32) & 32i32) != 0 {
            ((((((&raw mut gRfuSlotStatusNI).cast::<u8>().cast::<*mut u8>()).cast::<*mut u8>())
                .wrapping_offset(((slotNo) as i32) as isize))
            .read())
            .wrapping_add(104)
            .cast::<*mut u8>())
            .write(buffer);
            ((((((&raw mut gRfuSlotStatusNI).cast::<u8>().cast::<*mut u8>()).cast::<*mut u8>())
                .wrapping_offset(((slotNo) as i32) as isize))
            .read())
            .wrapping_add(108)
            .cast::<u32>())
            .write(buffSize);
        } else {
            if !((((connType) as i32) & 16i32) != 0) {
                return 1536u16;
            } else {
                ((((((&raw mut gRfuSlotStatusUNI).cast::<u8>().cast::<*mut u8>())
                    .cast::<*mut u8>())
                .wrapping_offset(((slotNo) as i32) as isize))
                .read())
                .wrapping_add(20)
                .cast::<*mut u8>())
                .write(buffer);
                ((((((&raw mut gRfuSlotStatusUNI).cast::<u8>().cast::<*mut u8>())
                    .cast::<*mut u8>())
                .wrapping_offset(((slotNo) as i32) as isize))
                .read())
                .wrapping_add(24)
                .cast::<u32>())
                .write(buffSize);
            }
        }
        return 0u16;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rfu_NI_setSendData(
    bmSendSlot: u8,
    subFrameSize: u8,
    src: *mut u8,
    size: u32,
) -> u16 {
    unsafe {
        let mut bmSendSlot = bmSendSlot;
        let mut subFrameSize = subFrameSize;
        let mut src = src;
        let mut size = size;
        return rfu_STC_setSendData_org(32u8, bmSendSlot, subFrameSize, src, size);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rfu_UNI_setSendData(bmSendSlot: u8, src: *mut u8, size: u8) -> u16 {
    unsafe {
        let mut bmSendSlot = bmSendSlot;
        let mut src = src;
        let mut size = size;
        let mut subFrameSize: u8 = 0u8;
        if (((((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>()).read()).read()) as i32)
            == 1i32
        {
            subFrameSize = ((((size) as i32).wrapping_add(3i32)) as u8);
        } else {
            subFrameSize = ((((size) as i32).wrapping_add(2i32)) as u8);
        }
        return rfu_STC_setSendData_org(16u8, bmSendSlot, subFrameSize, src, 0u32);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rfu_NI_CHILD_setSendGameName(slotNo: u8, subFrameSize: u8) -> u16 {
    unsafe {
        let mut slotNo = slotNo;
        let mut subFrameSize = subFrameSize;
        return rfu_STC_setSendData_org(
            64u8,
            ((crate::c::shl_i32(1i32, ((slotNo) as u32))) as u8),
            subFrameSize,
            (((((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(148))
            .wrapping_add(4)
            .cast::<u16>())
            .cast::<u8>(),
            26u32,
        );
    }
}
pub(crate) unsafe extern "C" fn rfu_STC_setSendData_org(
    ni_or_uni: u8,
    bmSendSlot: u8,
    subFrameSize: u8,
    src: *mut u8,
    dataSize: u32,
) -> u16 {
    unsafe {
        let mut ni_or_uni = ni_or_uni;
        let mut bmSendSlot = bmSendSlot;
        let mut subFrameSize = subFrameSize;
        let mut src = src;
        let mut dataSize = dataSize;
        let mut bm_slot_id: u8 = 0u8;
        let mut sendSlotFlag: u8 = 0u8;
        let mut frameSize: u8 = 0u8;
        let mut llFrameSize_p: *mut u8 = core::ptr::null_mut();
        let mut sending: u8 = 0u8;
        let mut i: u8 = 0u8;
        let mut imeBak: u16 = 0u16;
        let mut slotStatus_UNI: *mut u8 = core::ptr::null_mut();
        let mut slotStatus_NI: *mut u8 = core::ptr::null_mut();
        if (((((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>()).read()).read()) as i32)
            == 255i32
        {
            return 769u16;
        }
        if !((((bmSendSlot) as i32) & 15i32) != 0) {
            return 1024u16;
        }
        if ((((((((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(2))
        .read()) as i32)
            | ((((((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(3))
            .read()) as i32))
            & ((bmSendSlot) as i32))
            != ((bmSendSlot) as i32)
        {
            return 1025u16;
        }
        if (((ni_or_uni) as i32) & 16i32) != 0 {
            sendSlotFlag = ((((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(6))
            .read();
        } else {
            sendSlotFlag = ((((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(4))
            .read();
        }
        if (((sendSlotFlag) as i32) & ((bmSendSlot) as i32)) != 0 {
            return 1026u16;
        }
        {
            bm_slot_id = 0u8;
            'l1: loop {
                if !((((bm_slot_id) as i32) < 4i32)
                    && (!((crate::c::shr_i32(((bmSendSlot) as i32), ((bm_slot_id) as u32))
                        & 1i32)
                        != 0)))
                {
                    break 'l1;
                }
                'l2: {}
                bm_slot_id = (bm_slot_id).wrapping_add(1);
            }
        }
        if (((((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>()).read()).read()) as i32)
            == 1i32
        {
            llFrameSize_p = (((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(15);
        } else {
            if (((((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>()).read()).read()) as i32)
                == 0i32
            {
                llFrameSize_p = (((((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>())
                    .read())
                .wrapping_add(16))
                .cast::<u8>())
                .wrapping_offset(((bm_slot_id) as i32) as isize);
            }
        }
        frameSize = ((((&raw const llsf_struct).cast::<u8>().cast_mut()).cast::<u8>())
            .wrapping_offset(
                (((((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>()).read()).read())
                    as i32) as isize
                    * 16,
            ))
        .read();
        if ((!(llFrameSize_p).is_null())
            && (((subFrameSize) as i32) > (((llFrameSize_p).read()) as i32)))
            || (((subFrameSize) as i32) <= ((frameSize) as i32))
        {
            return 1280u16;
        }
        imeBak = ((67109384i32) as usize as *mut u16).read_volatile();
        crate::c::volatile_write(((67109384i32) as usize as *mut u16), 0u16);
        sending = ((((ni_or_uni) as i32) & 32i32) as u8);
        if ((sending) != 0) || (((ni_or_uni) as i32) == 64i32) {
            slotStatus_NI = ((((&raw mut gRfuSlotStatusNI).cast::<u8>().cast::<*mut u8>())
                .cast::<*mut u8>())
            .wrapping_offset(((bm_slot_id) as i32) as isize))
            .read();
            slotStatus_UNI = core::ptr::null_mut();
            ((slotStatus_NI).wrapping_add(24).cast::<u16>()).write(0u16);
            (((slotStatus_NI).wrapping_add(4)).cast::<*mut u8>())
                .write((slotStatus_NI).wrapping_add(45));
            ((slotStatus_NI).wrapping_add(20).cast::<u32>()).write(7u32);
            ((slotStatus_NI).wrapping_add(44)).write(bmSendSlot);
            ((slotStatus_NI).wrapping_add(26)).write(bmSendSlot);
            ((slotStatus_NI).wrapping_add(46).cast::<u16>())
                .write(((((subFrameSize) as i32).wrapping_sub(((frameSize) as i32))) as u16));
            if ((sending) as i32) != 0i32 {
                ((slotStatus_NI).wrapping_add(45)).write(0u8);
            } else {
                ((slotStatus_NI).wrapping_add(45)).write(1u8);
            }
            ((slotStatus_NI).wrapping_add(48).cast::<u32>()).write(dataSize);
            ((slotStatus_NI).wrapping_add(40).cast::<*mut u8>()).write(src);
            ((slotStatus_NI).wrapping_add(31)).write(0u8);
            ((slotStatus_NI).wrapping_add(32)).write(0u8);
            {
                i = 0u8;
                'l3: loop {
                    if !(((i) as i32) < 4i32) {
                        break 'l3;
                    }
                    'l4: {
                        ((((slotStatus_NI).wrapping_add(27)).cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize))
                        .write(0u8);
                        ((((slotStatus_NI).wrapping_add(33)).cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize))
                        .write(1u8);
                    }
                    i = (i).wrapping_add(1);
                }
            }
            {
                bm_slot_id = 0u8;
                'l5: loop {
                    if !(((bm_slot_id) as i32) < 4i32) {
                        break 'l5;
                    }
                    'l6: {
                        'l7: loop {
                            'l8: {
                                if (crate::c::shr_i32(((bmSendSlot) as i32), ((bm_slot_id) as u32))
                                    & 1i32)
                                    != 0
                                {
                                    ((((((&raw mut gRfuSlotStatusNI)
                                        .cast::<u8>()
                                        .cast::<*mut u8>())
                                    .cast::<*mut u8>())
                                    .wrapping_offset(((bm_slot_id) as i32) as isize))
                                    .read())
                                    .wrapping_add(2)
                                    .cast::<u16>())
                                    .write(0u16);
                                }
                            }
                            if !((0i32) != 0) {
                                break 'l7;
                            }
                        }
                    }
                    bm_slot_id = (bm_slot_id).wrapping_add(1);
                }
            }
            let __p1 =
                (((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(4);
            (__p1).write((((((__p1).read()) as i32) | ((bmSendSlot) as i32)) as u8));
            if !(llFrameSize_p).is_null() {
                (llFrameSize_p).write(
                    (((((llFrameSize_p).read()) as i32).wrapping_sub(((subFrameSize) as i32)))
                        as u8),
                );
            }
            ((slotStatus_NI).cast::<u16>()).write(32801u16);
        } else {
            if (((ni_or_uni) as i32) & 16i32) != 0 {
                slotStatus_UNI = ((((&raw mut gRfuSlotStatusUNI).cast::<u8>().cast::<*mut u8>())
                    .cast::<*mut u8>())
                .wrapping_offset(((bm_slot_id) as i32) as isize))
                .read();
                ((slotStatus_UNI).wrapping_add(3)).write(bmSendSlot);
                ((slotStatus_UNI).wrapping_add(8).cast::<*mut u8>()).write(src);
                ((slotStatus_UNI).wrapping_add(4).cast::<u16>())
                    .write(((((subFrameSize) as i32).wrapping_sub(((frameSize) as i32))) as u16));
                if !(llFrameSize_p).is_null() {
                    (llFrameSize_p).write(
                        (((((llFrameSize_p).read()) as i32).wrapping_sub(((subFrameSize) as i32)))
                            as u8),
                    );
                }
                ((slotStatus_UNI).cast::<u16>()).write(32804u16);
                let __p2 = (((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(6);
                (__p2).write((((((__p2).read()) as i32) | ((bmSendSlot) as i32)) as u8));
            }
        }
        crate::c::volatile_write(((67109384i32) as usize as *mut u16), imeBak);
        return 0u16;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rfu_changeSendTarget(
    connType: u8,
    slotStatusIndex: u8,
    bmNewTgtSlot: u8,
) -> u16 {
    unsafe {
        let mut connType = connType;
        let mut slotStatusIndex = slotStatusIndex;
        let mut bmNewTgtSlot = bmNewTgtSlot;
        let mut slotStatusNI: *mut u8 = core::ptr::null_mut();
        let mut imeBak: u16 = 0u16;
        let mut i: u8 = 0u8;
        if ((slotStatusIndex) as i32) >= 4i32 {
            return 1024u16;
        }
        if ((connType) as i32) == 32i32 {
            slotStatusNI = ((((&raw mut gRfuSlotStatusNI).cast::<u8>().cast::<*mut u8>())
                .cast::<*mut u8>())
            .wrapping_offset(((slotStatusIndex) as i32) as isize))
            .read();
            if ((((((slotStatusNI).cast::<u16>()).read()) as i32) & 32768i32) != 0)
                && ((((((slotStatusNI).cast::<u16>()).read()) as i32) & 32i32) != 0)
            {
                connType = ((((bmNewTgtSlot) as i32)
                    ^ ((((slotStatusNI).wrapping_add(26)).read()) as i32))
                    as u8);
                if !((((connType) as i32) & ((bmNewTgtSlot) as i32)) != 0) {
                    if (connType) != 0 {
                        imeBak = ((67109384i32) as usize as *mut u16).read_volatile();
                        crate::c::volatile_write(((67109384i32) as usize as *mut u16), 0u16);
                        {
                            i = 0u8;
                            'l1: loop {
                                if !(((i) as i32) < 4i32) {
                                    break 'l1;
                                }
                                'l2: {
                                    if (crate::c::shr_i32(((connType) as i32), ((i) as u32)) & 1i32)
                                        != 0
                                    {
                                        ((((((&raw mut gRfuSlotStatusNI)
                                            .cast::<u8>()
                                            .cast::<*mut u8>())
                                        .cast::<*mut u8>())
                                        .wrapping_offset(((i) as i32) as isize))
                                        .read())
                                        .wrapping_add(2)
                                        .cast::<u16>())
                                        .write(0u16);
                                    }
                                }
                                i = (i).wrapping_add(1);
                            }
                        }
                        let __p1 = (((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(4);
                        (__p1).write((((((__p1).read()) as i32) & !((connType) as i32)) as u8));
                        ((slotStatusNI).wrapping_add(26)).write(bmNewTgtSlot);
                        if ((((slotStatusNI).wrapping_add(26)).read()) as i32) == 0i32 {
                            rfu_STC_releaseFrame(slotStatusIndex, 0u8, (slotStatusNI));
                            ((slotStatusNI).cast::<u16>()).write(39u16);
                        }
                        crate::c::volatile_write(((67109384i32) as usize as *mut u16), imeBak);
                    }
                } else {
                    return 1028u16;
                }
            } else {
                return 1027u16;
            }
        } else {
            if ((connType) as i32) == 16i32 {
                let mut bmSlot: i32 = 0i32;
                if ((((((((&raw mut gRfuSlotStatusUNI).cast::<u8>().cast::<*mut u8>())
                    .cast::<*mut u8>())
                .wrapping_offset(((slotStatusIndex) as i32) as isize))
                .read())
                .cast::<u16>())
                .read()) as i32)
                    != 32804i32
                {
                    return 1027u16;
                }
                {
                    bmSlot = 0i32;
                    i = 0u8;
                    'l3: loop {
                        if !(((i) as i32) < 4i32) {
                            break 'l3;
                        }
                        'l4: {
                            if ((i) as i32) != ((slotStatusIndex) as i32) {
                                bmSlot = (bmSlot
                                    | ((((((((&raw mut gRfuSlotStatusUNI)
                                        .cast::<u8>()
                                        .cast::<*mut u8>())
                                    .cast::<*mut u8>())
                                    .wrapping_offset(((i) as i32) as isize))
                                    .read())
                                    .wrapping_add(3))
                                    .read()) as i32));
                            }
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                if (((bmNewTgtSlot) as i32) & bmSlot) != 0 {
                    return 1028u16;
                }
                imeBak = ((67109384i32) as usize as *mut u16).read_volatile();
                crate::c::volatile_write(((67109384i32) as usize as *mut u16), 0u16);
                let __p2 = (((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(6);
                (__p2).write(
                    (((((__p2).read()) as i32)
                        & !((((((((&raw mut gRfuSlotStatusUNI).cast::<u8>().cast::<*mut u8>())
                            .cast::<*mut u8>())
                        .wrapping_offset(((slotStatusIndex) as i32) as isize))
                        .read())
                        .wrapping_add(3))
                        .read()) as i32)) as u8),
                );
                let __p3 = (((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(6);
                (__p3).write((((((__p3).read()) as i32) | ((bmNewTgtSlot) as i32)) as u8));
                ((((((&raw mut gRfuSlotStatusUNI).cast::<u8>().cast::<*mut u8>())
                    .cast::<*mut u8>())
                .wrapping_offset(((slotStatusIndex) as i32) as isize))
                .read())
                .wrapping_add(3))
                .write(bmNewTgtSlot);
                crate::c::volatile_write(((67109384i32) as usize as *mut u16), imeBak);
            } else {
                return 1536u16;
            }
        }
        return 0u16;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rfu_NI_stopReceivingData(slotStatusIndex: u8) -> u16 {
    unsafe {
        let mut slotStatusIndex = slotStatusIndex;
        let mut imeBak: u16 = 0u16;
        let mut NI_comm: *mut u8 = core::ptr::null_mut();
        if ((slotStatusIndex) as i32) >= 4i32 {
            return 1024u16;
        }
        NI_comm = (((((&raw mut gRfuSlotStatusNI).cast::<u8>().cast::<*mut u8>())
            .cast::<*mut u8>())
        .wrapping_offset(((slotStatusIndex) as i32) as isize))
        .read())
        .wrapping_add(52);
        imeBak = ((67109384i32) as usize as *mut u16).read_volatile();
        crate::c::volatile_write(((67109384i32) as usize as *mut u16), 0u16);
        if (((((NI_comm).cast::<u16>()).read()) as i32) & 32768i32) != 0 {
            if ((((NI_comm).cast::<u16>()).read()) as i32) == 32835i32 {
                ((NI_comm).cast::<u16>()).write(72u16);
            } else {
                ((NI_comm).cast::<u16>()).write(71u16);
            }
            let __p1 =
                (((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(5);
            (__p1).write(
                (((((__p1).read()) as i32) & !(crate::c::shl_i32(1i32, ((slotStatusIndex) as u32))))
                    as u8),
            );
            rfu_STC_releaseFrame(slotStatusIndex, 1u8, NI_comm);
        }
        crate::c::volatile_write(((67109384i32) as usize as *mut u16), imeBak);
        return 0u16;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rfu_UNI_changeAndReadySendData(
    slotStatusIndex: u8,
    src: *mut u8,
    size: u8,
) -> u16 {
    unsafe {
        let mut slotStatusIndex = slotStatusIndex;
        let mut src = src;
        let mut size = size;
        let mut UNI_send: *mut u8 = core::ptr::null_mut();
        let mut frame_p: *mut u8 = core::ptr::null_mut();
        let mut imeBak: u16 = 0u16;
        let mut frameEnd: u8 = 0u8;
        if ((slotStatusIndex) as i32) >= 4i32 {
            return 1024u16;
        }
        UNI_send = (((((&raw mut gRfuSlotStatusUNI).cast::<u8>().cast::<*mut u8>())
            .cast::<*mut u8>())
        .wrapping_offset(((slotStatusIndex) as i32) as isize))
        .read());
        if ((((UNI_send).cast::<u16>()).read()) as i32) != 32804i32 {
            return 1027u16;
        }
        if (((((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>()).read()).read()) as i32)
            == 1i32
        {
            frame_p = (((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(15);
            frameEnd = ((((((((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(15))
            .read()) as i32)
                .wrapping_add((((((UNI_send).wrapping_add(4).cast::<u16>()).read()) as u8) as i32)))
                as u8);
        } else {
            frame_p = (((((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(16))
            .cast::<u8>())
            .wrapping_offset(((slotStatusIndex) as i32) as isize);
            frameEnd = ((((((((((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>())
                .read())
            .wrapping_add(16))
            .cast::<u8>())
            .wrapping_offset(((slotStatusIndex) as i32) as isize))
            .read()) as i32)
                .wrapping_add((((((UNI_send).wrapping_add(4).cast::<u16>()).read()) as u8) as i32)))
                as u8);
        }
        if ((frameEnd) as i32) < ((size) as i32) {
            return 1280u16;
        }
        imeBak = ((67109384i32) as usize as *mut u16).read_volatile();
        crate::c::volatile_write(((67109384i32) as usize as *mut u16), 0u16);
        ((UNI_send).wrapping_add(8).cast::<*mut u8>()).write(src);
        (frame_p).write(((((frameEnd) as i32).wrapping_sub(((size) as i32))) as u8));
        ((UNI_send).wrapping_add(4).cast::<u16>()).write(((size) as u16));
        ((UNI_send).wrapping_add(2)).write(1u8);
        crate::c::volatile_write(((67109384i32) as usize as *mut u16), imeBak);
        return 0u16;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rfu_UNI_readySendData(slotStatusIndex: u8) {
    unsafe {
        let mut slotStatusIndex = slotStatusIndex;
        if ((slotStatusIndex) as i32) < 4i32 {
            if ((((((((&raw mut gRfuSlotStatusUNI).cast::<u8>().cast::<*mut u8>())
                .cast::<*mut u8>())
            .wrapping_offset(((slotStatusIndex) as i32) as isize))
            .read())
            .cast::<u16>())
            .read()) as i32)
                == 32804i32
            {
                ((((((&raw mut gRfuSlotStatusUNI).cast::<u8>().cast::<*mut u8>())
                    .cast::<*mut u8>())
                .wrapping_offset(((slotStatusIndex) as i32) as isize))
                .read())
                .wrapping_add(2))
                .write(1u8);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rfu_UNI_clearRecvNewDataFlag(slotStatusIndex: u8) {
    unsafe {
        let mut slotStatusIndex = slotStatusIndex;
        if ((slotStatusIndex) as i32) < 4i32 {
            (((((((&raw mut gRfuSlotStatusUNI).cast::<u8>().cast::<*mut u8>())
                .cast::<*mut u8>())
            .wrapping_offset(((slotStatusIndex) as i32) as isize))
            .read())
            .wrapping_add(12))
            .wrapping_add(6))
            .write(0u8);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rfu_REQ_sendData(clockChangeFlag: u8) {
    unsafe {
        let mut clockChangeFlag = clockChangeFlag;
        if (((((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>()).read()).read()) as i32)
            != 255i32
        {
            if ((((((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>()).read()).read())
                as i32)
                == 1i32)
                && (!(((((((((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(4))
                .read()) as i32)
                    | ((((((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(5))
                    .read()) as i32))
                    | ((((((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(6))
                    .read()) as i32))
                    != 0))
            {
                if (((((&raw mut gRfuStatic).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(3))
                .read())
                    != 0
                {
                    ((((&raw mut gRfuStatic).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(8))
                    .write(16u8);
                    ((((&raw mut gRfuStatic).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(7))
                    .write(0u8);
                }
                if (((((&raw mut gRfuStatic).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(8))
                .read())
                    != 0
                {
                    let __p1 = (((&raw mut gRfuStatic).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(8);
                    (__p1).write(((__p1).read()).wrapping_sub(1));
                } else {
                    let __p2 = (((&raw mut gRfuStatic).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(7);
                    (__p2).write(((__p2).read()).wrapping_add(1));
                }
                if ((((((&raw mut gRfuStatic).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(8))
                .read())
                    != 0)
                    || (!((((((((&raw mut gRfuStatic).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(7))
                    .read()) as i32)
                        & 15i32)
                        != 0))
                {
                    (((((&raw mut gRfuFixed).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(104))
                    .cast::<u32>())
                    .write(1u32);
                    ((((((&raw mut gRfuFixed).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(104))
                    .cast::<u32>())
                    .wrapping_offset(4))
                    .write(255u32);
                    STWI_set_Callback_M(core::mem::transmute::<
                        Option<unsafe extern "C" fn(u8, u16)>,
                        *mut u8,
                    >(Some(
                        rfu_CB_sendData3 as unsafe extern "C" fn(u8, u16),
                    )));
                    if !((clockChangeFlag) != 0) {
                        STWI_send_DataTxREQ(
                            (((((&raw mut gRfuFixed).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(104))
                            .cast::<u32>())
                            .cast::<u8>(),
                            1u8,
                        );
                    } else {
                        STWI_send_DataTxAndChangeREQ(
                            (((((&raw mut gRfuFixed).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(104))
                            .cast::<u32>())
                            .cast::<u8>(),
                            1u8,
                        );
                    }
                    return;
                }
            } else {
                if !((((((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(14))
                .read_volatile())
                    != 0)
                {
                    rfu_constructSendLLFrame();
                }
                if (((((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(14))
                .read_volatile())
                    != 0
                {
                    STWI_set_Callback_M(core::mem::transmute::<
                        Option<unsafe extern "C" fn(u8, u16)>,
                        *mut u8,
                    >(Some(
                        rfu_CB_sendData as unsafe extern "C" fn(u8, u16),
                    )));
                    if (clockChangeFlag) != 0 {
                        STWI_send_DataTxAndChangeREQ(
                            (((((&raw mut gRfuFixed).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(104))
                            .cast::<u32>())
                            .cast::<u8>(),
                            (((((((&raw mut gRfuStatic).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(36)
                                .cast::<u32>())
                            .read())
                            .wrapping_add(4u32)) as u8),
                        );
                        return;
                    }
                    STWI_send_DataTxREQ(
                        (((((&raw mut gRfuFixed).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(104))
                        .cast::<u32>())
                        .cast::<u8>(),
                        (((((((&raw mut gRfuStatic).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(36)
                            .cast::<u32>())
                        .read())
                        .wrapping_add(4u32)) as u8),
                    );
                }
            }
            if (clockChangeFlag) != 0 {
                if (((((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>()).read()).read())
                    as i32)
                    == 1i32
                {
                    if core::mem::transmute::<_, usize>(
                        ((((&raw mut gSTWIStatus).cast::<*mut u8>()).read())
                            .wrapping_add(28)
                            .cast::<Option<unsafe extern "C" fn(u16)>>())
                        .read(),
                    ) != 0usize
                    {
                        (((((&raw mut gSTWIStatus).cast::<*mut u8>()).read())
                            .wrapping_add(28)
                            .cast::<Option<unsafe extern "C" fn(u16)>>())
                        .read())
                        .unwrap_unchecked()(39u16);
                    }
                } else {
                    STWI_set_Callback_M(core::mem::transmute::<
                        Option<unsafe extern "C" fn(u8, u16)>,
                        *mut u8,
                    >(Some(
                        rfu_CB_sendData2 as unsafe extern "C" fn(u8, u16),
                    )));
                    STWI_send_MS_ChangeREQ();
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn rfu_CB_sendData(reqCommand: u8, reqResult: u16) {
    unsafe {
        let mut reqCommand = reqCommand;
        let mut reqResult = reqResult;
        let mut i: u8 = 0u8;
        let mut NI_comm: *mut u8 = core::ptr::null_mut();
        if ((reqResult) as i32) == 0i32 {
            {
                i = 0u8;
                'l1: loop {
                    if !(((i) as i32) < 4i32) {
                        break 'l1;
                    }
                    'l2: {
                        if (((((((&raw mut gRfuSlotStatusUNI).cast::<u8>().cast::<*mut u8>())
                            .cast::<*mut u8>())
                        .wrapping_offset(((i) as i32) as isize))
                        .read())
                        .wrapping_add(2))
                        .read())
                            != 0
                        {
                            ((((((&raw mut gRfuSlotStatusUNI).cast::<u8>().cast::<*mut u8>())
                                .cast::<*mut u8>())
                            .wrapping_offset(((i) as i32) as isize))
                            .read())
                            .wrapping_add(2))
                            .write(0u8);
                        }
                        NI_comm =
                            (((((&raw mut gRfuSlotStatusNI).cast::<u8>().cast::<*mut u8>())
                                .cast::<*mut u8>())
                            .wrapping_offset(((i) as i32) as isize))
                            .read());
                        if ((((NI_comm).cast::<u16>()).read()) as i32) == 32800i32 {
                            rfu_STC_releaseFrame(i, 0u8, NI_comm);
                            let __p1 = (((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(4);
                            (__p1).write(
                                (((((__p1).read()) as i32)
                                    & !((((NI_comm).wrapping_add(26)).read()) as i32))
                                    as u8),
                            );
                            if ((((NI_comm).wrapping_add(45)).read()) as i32) == 1i32 {
                                let __p2 =
                                    (((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(7);
                                (__p2).write(
                                    (((((__p2).read()) as i32)
                                        | crate::c::shl_i32(1i32, ((i) as u32)))
                                        as u8),
                                );
                            }
                            ((NI_comm).cast::<u16>()).write(38u16);
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
        }
        crate::c::volatile_write(
            (((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(14),
            0u8,
        );
        rfu_STC_REQ_callback(36u8, reqResult);
    }
}
pub(crate) unsafe extern "C" fn rfu_CB_sendData2(reqCommand: u8, reqResult: u16) {
    unsafe {
        let mut reqCommand = reqCommand;
        let mut reqResult = reqResult;
        rfu_STC_REQ_callback(36u8, reqResult);
    }
}
pub(crate) unsafe extern "C" fn rfu_CB_sendData3(reqCommand: u8, reqResult: u16) {
    unsafe {
        let mut reqCommand = reqCommand;
        let mut reqResult = reqResult;
        if ((reqResult) as i32) != 0i32 {
            rfu_STC_REQ_callback(36u8, reqResult);
        } else {
            if ((reqCommand) as i32) == 255i32 {
                rfu_STC_REQ_callback(255u8, 0u16);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn rfu_constructSendLLFrame() {
    unsafe {
        let mut pakcketSize: u32 = 0u32;
        let mut currSize: u32 = 0u32;
        let mut i: u8 = 0u8;
        let mut llf_p: *mut u8 = core::ptr::null_mut();
        if ((((((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>()).read()).read()) as i32)
            != 255i32)
            && (((((((((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(4))
            .read()) as i32)
                | ((((((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(5))
                .read()) as i32))
                | ((((((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(6))
                .read()) as i32))
                != 0)
        {
            crate::c::volatile_write(
                (((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(14),
                0u8,
            );
            pakcketSize = 0u32;
            llf_p = ((((((&raw mut gRfuFixed).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(104))
            .cast::<u32>())
            .wrapping_offset(1))
            .cast::<u8>();
            {
                i = 0u8;
                'l1: loop {
                    if !(((i) as i32) < 4i32) {
                        break 'l1;
                    }
                    'l2: {
                        currSize = 0u32;
                        if (((((((((&raw mut gRfuSlotStatusNI).cast::<u8>().cast::<*mut u8>())
                            .cast::<*mut u8>())
                        .wrapping_offset(((i) as i32) as isize))
                        .read())
                        .cast::<u16>())
                        .read()) as i32)
                            & 32768i32)
                            != 0
                        {
                            currSize = ((rfu_STC_NI_constructLLSF(
                                i,
                                &raw mut llf_p,
                                (((((&raw mut gRfuSlotStatusNI).cast::<u8>().cast::<*mut u8>())
                                    .cast::<*mut u8>())
                                .wrapping_offset(((i) as i32) as isize))
                                .read()),
                            )) as u32);
                        }
                        if ((((((((((&raw mut gRfuSlotStatusNI)
                            .cast::<u8>()
                            .cast::<*mut u8>())
                        .cast::<*mut u8>())
                        .wrapping_offset(((i) as i32) as isize))
                        .read())
                        .wrapping_add(52))
                        .cast::<u16>())
                        .read()) as i32)
                            & 32768i32)
                            != 0
                        {
                            currSize = (currSize).wrapping_add(
                                ((rfu_STC_NI_constructLLSF(
                                    i,
                                    &raw mut llf_p,
                                    (((((&raw mut gRfuSlotStatusNI)
                                        .cast::<u8>()
                                        .cast::<*mut u8>())
                                    .cast::<*mut u8>())
                                    .wrapping_offset(((i) as i32) as isize))
                                    .read())
                                    .wrapping_add(52),
                                )) as u32),
                            );
                        }
                        if ((((((((&raw mut gRfuSlotStatusUNI).cast::<u8>().cast::<*mut u8>())
                            .cast::<*mut u8>())
                        .wrapping_offset(((i) as i32) as isize))
                        .read())
                        .cast::<u16>())
                        .read()) as i32)
                            == 32804i32
                        {
                            currSize = (currSize).wrapping_add(
                                ((rfu_STC_UNI_constructLLSF(i, &raw mut llf_p)) as u32),
                            );
                        }
                        if currSize != 0u32 {
                            if (((((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .read()) as i32)
                                == 1i32
                            {
                                pakcketSize = (pakcketSize).wrapping_add(currSize);
                            } else {
                                pakcketSize = (pakcketSize
                                    | crate::c::shl_u32(
                                        currSize,
                                        ((((5i32).wrapping_mul(((i) as i32))).wrapping_add(8i32))
                                            as u32),
                                    ));
                            }
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
            if pakcketSize != 0u32 {
                'l3: loop {
                    if !((((llf_p) as usize as u32) & 3u32) != 0) {
                        break 'l3;
                    }
                    ({
                        let __t1 = llf_p;
                        llf_p = (llf_p).wrapping_offset(1);
                        __t1
                    })
                    .write(0u8);
                }
                (((((&raw mut gRfuFixed).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(104))
                .cast::<u32>())
                .write(pakcketSize);
                if (((((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>()).read()).read())
                    as i32)
                    == 0i32
                {
                    let mut maxSize: *mut u8 = (llf_p).wrapping_offset(-108);
                    pakcketSize = ((((maxSize) as usize).wrapping_sub(
                        (((&raw mut gRfuFixed).cast::<u8>().cast::<*mut u8>()).read_volatile())
                            as usize,
                    ) as i32
                        / 1) as u32);
                }
            }
            ((((&raw mut gRfuStatic).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(36)
                .cast::<u32>())
            .write(pakcketSize);
        }
    }
}
pub(crate) unsafe extern "C" fn rfu_STC_NI_constructLLSF(
    bm_slot_id: u8,
    dest_pp: *mut *mut u8,
    NI_comm: *mut u8,
) -> u16 {
    unsafe {
        let mut bm_slot_id = bm_slot_id;
        let mut dest_pp = dest_pp;
        let mut NI_comm = NI_comm;
        let mut size: u16 = 0u16;
        let mut frame: u32 = 0u32;
        let mut i: u8 = 0u8;
        let mut frame8_p: *mut u8 = core::ptr::null_mut();
        let mut llsf: *mut u8 = (((&raw const llsf_struct).cast::<u8>().cast_mut()).cast::<u8>())
            .wrapping_offset(
                (((((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>()).read()).read())
                    as i32) as isize
                    * 16,
            );
        if ((((NI_comm).cast::<u16>()).read()) as i32) == 32802i32 {
            'l1: loop {
                if !(((((((NI_comm).wrapping_add(4)).cast::<*mut u8>())
                    .wrapping_offset(((((NI_comm).wrapping_add(32)).read()) as i32) as isize))
                .read()) as usize)
                    >= (((((NI_comm).wrapping_add(40).cast::<*mut u8>()).read()).wrapping_offset(
                        ((((NI_comm).wrapping_add(48).cast::<u32>()).read()) as i32) as isize,
                    )) as usize))
                {
                    break 'l1;
                }
                let __p1 = (NI_comm).wrapping_add(32);
                (__p1).write(((__p1).read()).wrapping_add(1));
                if ((((NI_comm).wrapping_add(32)).read()) as i32) == 4i32 {
                    ((NI_comm).wrapping_add(32)).write(0u8);
                }
            }
        }
        if (((((NI_comm).cast::<u16>()).read()) as i32) & 64i32) != 0 {
            size = 0u16;
        } else {
            if ((((NI_comm).cast::<u16>()).read()) as i32) == 32802i32 {
                if (((((((NI_comm).wrapping_add(4)).cast::<*mut u8>())
                    .wrapping_offset(((((NI_comm).wrapping_add(32)).read()) as i32) as isize))
                .read())
                .wrapping_offset(
                    ((((NI_comm).wrapping_add(46).cast::<u16>()).read()) as i32) as isize,
                )) as usize)
                    > (((((NI_comm).wrapping_add(40).cast::<*mut u8>()).read()).wrapping_offset(
                        ((((NI_comm).wrapping_add(48).cast::<u32>()).read()) as i32) as isize,
                    )) as usize)
                {
                    size = (((((((NI_comm).wrapping_add(40).cast::<*mut u8>()).read())
                        .wrapping_offset(
                            ((((NI_comm).wrapping_add(48).cast::<u32>()).read()) as i32) as isize,
                        )) as usize)
                        .wrapping_sub(
                            (((((NI_comm).wrapping_add(4)).cast::<*mut u8>()).wrapping_offset(
                                ((((NI_comm).wrapping_add(32)).read()) as i32) as isize,
                            ))
                            .read()) as usize,
                        ) as i32
                        / 1) as u16);
                } else {
                    size = ((NI_comm).wrapping_add(46).cast::<u16>()).read();
                }
            } else {
                if ((NI_comm).wrapping_add(20).cast::<u32>()).read()
                    >= ((((NI_comm).wrapping_add(46).cast::<u16>()).read()) as u32)
                {
                    size = ((NI_comm).wrapping_add(46).cast::<u16>()).read();
                } else {
                    size = ((((NI_comm).wrapping_add(20).cast::<u32>()).read()) as u16);
                }
            }
        }
        frame = (((((crate::c::shl_i32(
            (((((NI_comm).cast::<u16>()).read()) as i32) & 15i32),
            ((((llsf).wrapping_add(3)).read()) as u32),
        ) | crate::c::shl_i32(
            ((((NI_comm).wrapping_add(31)).read()) as i32),
            ((((llsf).wrapping_add(4)).read()) as u32),
        )) | crate::c::shl_i32(
            ((((NI_comm).wrapping_add(32)).read()) as i32),
            ((((llsf).wrapping_add(5)).read()) as u32),
        )) | crate::c::shl_i32(
            ((((((NI_comm).wrapping_add(33)).cast::<u8>())
                .wrapping_offset(((((NI_comm).wrapping_add(32)).read()) as i32) as isize))
            .read()) as i32),
            ((((llsf).wrapping_add(6)).read()) as u32),
        )) | ((size) as i32)) as u32);
        if (((((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>()).read()).read()) as i32)
            == 1i32
        {
            frame = (frame | ((((((NI_comm).wrapping_add(26)).read()) as i32) << 18) as u32));
        }
        frame8_p = (&raw mut frame).cast::<u8>();
        {
            i = 0u8;
            'l2: loop {
                if !(((i) as i32) < (((llsf).read()) as i32)) {
                    break 'l2;
                }
                'l3: {
                    ({
                        let __t2 = (dest_pp).read();
                        (dest_pp).write(((dest_pp).read()).wrapping_offset(1));
                        __t2
                    })
                    .write(
                        ({
                            let __t4 = frame8_p;
                            frame8_p = (frame8_p).wrapping_offset(1);
                            __t4
                        })
                        .read(),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        if ((size) as i32) != 0i32 {
            let mut src: *mut u8 = ((((NI_comm).wrapping_add(4)).cast::<*mut u8>())
                .wrapping_offset(((((NI_comm).wrapping_add(32)).read()) as i32) as isize))
            .read();
            (((((&raw mut gRfuFixed).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(4)
                .cast::<Option<unsafe extern "C" fn(*mut *mut u8, *mut *mut u8, i32)>>())
            .read())
            .unwrap_unchecked()(&raw mut src, dest_pp, ((size) as i32));
        }
        if ((((NI_comm).cast::<u16>()).read()) as i32) == 32802i32 {
            let __p5 = (NI_comm).wrapping_add(32);
            (__p5).write(((__p5).read()).wrapping_add(1));
            if ((((NI_comm).wrapping_add(32)).read()) as i32) == 4i32 {
                ((NI_comm).wrapping_add(32)).write(0u8);
            }
        }
        if (((((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>()).read()).read()) as i32)
            == 1i32
        {
            crate::c::volatile_write(
                (((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(14),
                1u8,
            );
        } else {
            let __p6 = (((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(14);
            crate::c::volatile_write(
                __p6,
                (((((__p6).read_volatile()) as i32)
                    | crate::c::shl_i32(1i32, ((bm_slot_id) as u32))) as u8),
            );
        }
        return ((((size) as i32).wrapping_add((((llsf).read()) as i32))) as u16);
    }
}
pub(crate) unsafe extern "C" fn rfu_STC_UNI_constructLLSF(
    bm_slot_id: u8,
    dest_p: *mut *mut u8,
) -> u16 {
    unsafe {
        let mut bm_slot_id = bm_slot_id;
        let mut dest_p = dest_p;
        let mut llsf: *mut u8 = core::ptr::null_mut();
        let mut src_p: *mut u8 = core::ptr::null_mut();
        let mut frame: u32 = 0u32;
        let mut frame8_p: *mut u8 = core::ptr::null_mut();
        let mut i: u8 = 0u8;
        let mut UNI_send: *mut u8 =
            (((((&raw mut gRfuSlotStatusUNI).cast::<u8>().cast::<*mut u8>()).cast::<*mut u8>())
                .wrapping_offset(((bm_slot_id) as i32) as isize))
            .read());
        if (!((((UNI_send).wrapping_add(2)).read()) != 0))
            || (!((((UNI_send).wrapping_add(3)).read()) != 0))
        {
            return 0u16;
        }
        llsf = (((&raw const llsf_struct).cast::<u8>().cast_mut()).cast::<u8>()).wrapping_offset(
            (((((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>()).read()).read()) as i32)
                as isize
                * 16,
        );
        frame = ((crate::c::shl_i32(
            (((((UNI_send).cast::<u16>()).read()) as i32) & 15i32),
            ((((llsf).wrapping_add(3)).read()) as u32),
        ) | ((((UNI_send).wrapping_add(4).cast::<u16>()).read()) as i32)) as u32);
        if (((((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>()).read()).read()) as i32)
            == 1i32
        {
            frame = (frame | ((((((UNI_send).wrapping_add(3)).read()) as i32) << 18) as u32));
        }
        frame8_p = (&raw mut frame).cast::<u8>();
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < (((llsf).read()) as i32)) {
                    break 'l1;
                }
                'l2: {
                    ({
                        let __t1 = (dest_p).read();
                        (dest_p).write(((dest_p).read()).wrapping_offset(1));
                        __t1
                    })
                    .write(
                        ({
                            let __t3 = frame8_p;
                            frame8_p = (frame8_p).wrapping_offset(1);
                            __t3
                        })
                        .read(),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        src_p = ((UNI_send).wrapping_add(8).cast::<*mut u8>()).read();
        (((((&raw mut gRfuFixed).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(4)
            .cast::<Option<unsafe extern "C" fn(*mut *mut u8, *mut *mut u8, i32)>>())
        .read())
        .unwrap_unchecked()(
            &raw mut src_p,
            dest_p,
            ((((UNI_send).wrapping_add(4).cast::<u16>()).read()) as i32),
        );
        if (((((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>()).read()).read()) as i32)
            == 1i32
        {
            crate::c::volatile_write(
                (((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(14),
                16u8,
            );
        } else {
            let __p4 = (((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(14);
            crate::c::volatile_write(
                __p4,
                (((((__p4).read_volatile()) as i32)
                    | crate::c::shl_i32(16i32, ((bm_slot_id) as u32))) as u8),
            );
        }
        return (((((llsf).read()) as i32)
            .wrapping_add(((((UNI_send).wrapping_add(4).cast::<u16>()).read()) as i32)))
            as u16);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rfu_REQ_recvData() {
    unsafe {
        if (((((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>()).read()).read()) as i32)
            != 255i32
        {
            ((((&raw mut gRfuStatic).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(3))
                .write(
                    (((((((((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(4))
                    .read()) as i32)
                        | ((((((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(5))
                        .read()) as i32))
                        | ((((((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(6))
                        .read()) as i32)) as u8),
                );
            ((((&raw mut gRfuStatic).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(4))
                .write(0u8);
            STWI_set_Callback_M(core::mem::transmute::<
                Option<unsafe extern "C" fn(u8, u16)>,
                *mut u8,
            >(Some(
                rfu_CB_recvData as unsafe extern "C" fn(u8, u16),
            )));
            STWI_send_DataRxREQ();
        }
    }
}
pub(crate) unsafe extern "C" fn rfu_CB_recvData(reqCommand: u8, reqResult: u16) {
    unsafe {
        let mut reqCommand = reqCommand;
        let mut reqResult = reqResult;
        let mut i: u8 = 0u8;
        let mut slotStatusNI: *mut u8 = core::ptr::null_mut();
        let mut NI_comm: *mut u8 = core::ptr::null_mut();
        if (((reqResult) as i32) == 0i32)
            && (((((((((&raw mut gRfuFixed).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(220)
                .cast::<*mut u8>())
            .read())
            .cast::<u8>())
            .wrapping_offset(1))
            .read())
                != 0)
        {
            ((((&raw mut gRfuStatic).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1))
                .write(0u8);
            if (((((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>()).read()).read()) as i32)
                == 1i32
            {
                rfu_STC_PARENT_analyzeRecvPacket();
            } else {
                rfu_STC_CHILD_analyzeRecvPacket();
            }
            {
                i = 0u8;
                'l1: loop {
                    if !(((i) as i32) < 4i32) {
                        break 'l1;
                    }
                    'l2: {
                        slotStatusNI =
                            ((((&raw mut gRfuSlotStatusNI).cast::<u8>().cast::<*mut u8>())
                                .cast::<*mut u8>())
                            .wrapping_offset(((i) as i32) as isize))
                            .read();
                        if ((((((slotStatusNI).wrapping_add(52)).cast::<u16>()).read()) as i32)
                            == 32835i32)
                            && (!((crate::c::shr_i32(
                                ((((((&raw mut gRfuStatic).cast::<u8>().cast::<*mut u8>()).read())
                                    .wrapping_add(1))
                                .read()) as i32),
                                ((i) as u32),
                            ) & 1i32)
                                != 0))
                        {
                            NI_comm = (slotStatusNI).wrapping_add(52);
                            if ((((NI_comm).wrapping_add(45)).read()) as i32) == 1i32 {
                                let __p1 =
                                    (((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(7);
                                (__p1).write(
                                    (((((__p1).read()) as i32)
                                        | crate::c::shl_i32(1i32, ((i) as u32)))
                                        as u8),
                                );
                            }
                            rfu_STC_releaseFrame(i, 1u8, NI_comm);
                            let __p2 = (((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(5);
                            (__p2).write(
                                (((((__p2).read()) as i32)
                                    & !((((NI_comm).wrapping_add(26)).read()) as i32))
                                    as u8),
                            );
                            (((slotStatusNI).wrapping_add(52)).cast::<u16>()).write(70u16);
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
            if (((((&raw mut gRfuStatic).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(4))
                .read())
                != 0
            {
                reqResult = ((((((((&raw mut gRfuStatic).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(4))
                .read()) as i32)
                    | 1792i32) as u16);
            }
        }
        rfu_STC_REQ_callback(reqCommand, reqResult);
    }
}
pub(crate) unsafe extern "C" fn rfu_STC_PARENT_analyzeRecvPacket() {
    unsafe {
        let mut frames32: u32 = 0u32;
        let mut bm_slot_id: u8 = 0u8;
        let mut frame_counts = crate::ffi::Align4([0u8; 4]);
        let mut packet_p: *mut u8 = core::ptr::null_mut();
        frames32 = ((((((((&raw mut gRfuFixed).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(220)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(4))
        .cast::<u32>())
        .read()
            >> 8);
        {
            bm_slot_id = 0u8;
            'l1: loop {
                if !(((bm_slot_id) as i32) < 4i32) {
                    break 'l1;
                }
                'l2: {
                    (((&raw mut frame_counts).cast::<u8>())
                        .wrapping_offset(((bm_slot_id) as i32) as isize))
                    .write(((frames32 & 31u32) as u8));
                    frames32 = (frames32 >> 5);
                    if (((((&raw mut frame_counts).cast::<u8>())
                        .wrapping_offset(((bm_slot_id) as i32) as isize))
                    .read()) as i32)
                        == 0i32
                    {
                        let __p1 = (((&raw mut gRfuStatic).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(1);
                        (__p1).write(
                            (((((__p1).read()) as i32)
                                | crate::c::shl_i32(1i32, ((bm_slot_id) as u32)))
                                as u8),
                        );
                    }
                }
                bm_slot_id = (bm_slot_id).wrapping_add(1);
            }
        }
        packet_p = ((((((&raw mut gRfuFixed).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(220)
            .cast::<*mut u8>())
        .read())
        .cast::<u8>())
        .wrapping_offset(8);
        {
            bm_slot_id = 0u8;
            'l3: loop {
                if !(((bm_slot_id) as i32) < 4i32) {
                    break 'l3;
                }
                'l4: {
                    if ((((&raw mut frame_counts).cast::<u8>())
                        .wrapping_offset(((bm_slot_id) as i32) as isize))
                    .read())
                        != 0
                    {
                        let mut frames_p: *mut u8 = ((&raw mut frame_counts).cast::<u8>())
                            .wrapping_offset(((bm_slot_id) as i32) as isize);
                        'l5: loop {
                            'l6: {
                                let mut analyzed_frames: u8 = ((rfu_STC_analyzeLLSF(
                                    bm_slot_id,
                                    packet_p,
                                    (((frames_p).read()) as u16),
                                ))
                                    as u8);
                                packet_p =
                                    (packet_p).wrapping_offset(((analyzed_frames) as i32) as isize);
                                (frames_p).write(
                                    (((((frames_p).read()) as i32)
                                        .wrapping_sub(((analyzed_frames) as i32)))
                                        as u8),
                                );
                            }
                            if !((!(((((frames_p).read()) as i32) & 128i32) != 0))
                                && (((frames_p).read()) != 0))
                            {
                                break 'l5;
                            }
                        }
                    }
                }
                bm_slot_id = (bm_slot_id).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn rfu_STC_CHILD_analyzeRecvPacket() {
    unsafe {
        let mut frames_remaining: u16 = 0u16;
        let mut packet_p: *mut u8 = core::ptr::null_mut();
        let mut analyzed_frames: u16 = 0u16;
        frames_remaining = ((((((((((((&raw mut gRfuFixed).cast::<u8>().cast::<*mut u8>())
            .read())
        .wrapping_add(220)
        .cast::<*mut u8>())
        .read())
        .cast::<u8>())
        .wrapping_offset(4))
        .cast::<u16>())
        .read()) as i32)
            & 127i32) as u16);
        packet_p = ((((((&raw mut gRfuFixed).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(220)
            .cast::<*mut u8>())
        .read())
        .cast::<u8>())
        .wrapping_offset(8);
        if ((frames_remaining) as i32) == 0i32 {
            ((((&raw mut gRfuStatic).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1))
                .write(15u8);
        }
        'l1: loop {
            'l2: {
                if ((frames_remaining) as i32) == 0i32 {
                    break 'l1;
                }
                analyzed_frames = rfu_STC_analyzeLLSF(0u8, packet_p, frames_remaining);
                packet_p = (packet_p).wrapping_offset(((analyzed_frames) as i32) as isize);
                frames_remaining =
                    ((((frames_remaining) as i32).wrapping_sub(((analyzed_frames) as i32))) as u16);
            }
            if !(!((((frames_remaining) as i32) & 32768i32) != 0)) {
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn rfu_STC_analyzeLLSF(
    slot_id: u8,
    src: *mut u8,
    last_frame: u16,
) -> u16 {
    unsafe {
        let mut slot_id = slot_id;
        let mut src = src;
        let mut last_frame = last_frame;
        let mut llsf_NI = crate::ffi::Align4([0u8; 8]);
        let mut llsf_p: *mut u8 = core::ptr::null_mut();
        let mut frames: u32 = 0u32;
        let mut i: u8 = 0u8;
        let mut retVal: u16 = 0u16;
        llsf_p = (((&raw const llsf_struct).cast::<u8>().cast_mut()).cast::<u8>()).wrapping_offset(
            (!(((((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>()).read()).read()) as i32)
                & 1i32) as isize
                * 16,
        );
        if ((last_frame) as i32) < (((llsf_p).read()) as i32) {
            return last_frame;
        }
        frames = 0u32;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < (((llsf_p).read()) as i32)) {
                    break 'l1;
                }
                'l2: {
                    frames = (frames
                        | ((crate::c::shl_i32(
                            ((({
                                let __t2 = src;
                                src = (src).wrapping_offset(1);
                                __t2
                            })
                            .read()) as i32),
                            (((8i32).wrapping_mul(((i) as i32))) as u32),
                        )) as u32));
                }
                i = (i).wrapping_add(1);
            }
        }
        ((&raw mut llsf_NI).cast::<u8>()).write(
            ((crate::c::shr_u32(frames, ((((llsf_p).wrapping_add(1)).read()) as u32))
                & ((((llsf_p).wrapping_add(7)).read()) as u32)) as u8),
        );
        (((&raw mut llsf_NI).cast::<u8>()).wrapping_add(1)).write(
            ((crate::c::shr_u32(frames, ((((llsf_p).wrapping_add(2)).read()) as u32))
                & ((((llsf_p).wrapping_add(8)).read()) as u32)) as u8),
        );
        (((&raw mut llsf_NI).cast::<u8>()).wrapping_add(2)).write(
            ((crate::c::shr_u32(frames, ((((llsf_p).wrapping_add(3)).read()) as u32))
                & ((((llsf_p).wrapping_add(9)).read()) as u32)) as u8),
        );
        (((&raw mut llsf_NI).cast::<u8>()).wrapping_add(3)).write(
            ((crate::c::shr_u32(frames, ((((llsf_p).wrapping_add(4)).read()) as u32))
                & ((((llsf_p).wrapping_add(10)).read()) as u32)) as u8),
        );
        (((&raw mut llsf_NI).cast::<u8>()).wrapping_add(4)).write(
            ((crate::c::shr_u32(frames, ((((llsf_p).wrapping_add(5)).read()) as u32))
                & ((((llsf_p).wrapping_add(11)).read()) as u32)) as u8),
        );
        (((&raw mut llsf_NI).cast::<u8>()).wrapping_add(5)).write(
            ((crate::c::shr_u32(frames, ((((llsf_p).wrapping_add(6)).read()) as u32))
                & ((((llsf_p).wrapping_add(12)).read()) as u32)) as u8),
        );
        (((&raw mut llsf_NI).cast::<u8>())
            .wrapping_add(6)
            .cast::<u16>())
        .write(
            (((frames & ((((llsf_p).wrapping_add(14).cast::<u16>()).read()) as u32)) & frames)
                as u16),
        );
        retVal = (((((((&raw mut llsf_NI).cast::<u8>())
            .wrapping_add(6)
            .cast::<u16>())
        .read()) as i32)
            .wrapping_add((((llsf_p).read()) as i32))) as u16);
        if ((((&raw mut llsf_NI).cast::<u8>()).read()) as i32) == 0i32 {
            if (((((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>()).read()).read()) as i32)
                == 1i32
            {
                if (crate::c::shr_i32(
                    ((((((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(2))
                    .read()) as i32),
                    ((slot_id) as u32),
                ) & 1i32)
                    != 0
                {
                    if (((((&raw mut llsf_NI).cast::<u8>()).wrapping_add(2)).read()) as i32) == 4i32
                    {
                        rfu_STC_UNI_receive(slot_id, (&raw mut llsf_NI).cast::<u8>(), src);
                    } else {
                        if (((((&raw mut llsf_NI).cast::<u8>()).wrapping_add(3)).read()) as i32)
                            == 0i32
                        {
                            rfu_STC_NI_receive_Receiver(
                                slot_id,
                                (&raw mut llsf_NI).cast::<u8>(),
                                src,
                            );
                        } else {
                            {
                                i = 0u8;
                                'l3: loop {
                                    if !(((i) as i32) < 4i32) {
                                        break 'l3;
                                    }
                                    'l4: {
                                        if ((crate::c::shr_i32(
                                            ((((((((&raw mut gRfuSlotStatusNI)
                                                .cast::<u8>()
                                                .cast::<*mut u8>())
                                            .cast::<*mut u8>())
                                            .wrapping_offset(((i) as i32) as isize))
                                            .read())
                                            .wrapping_add(26))
                                            .read())
                                                as i32),
                                            ((slot_id) as u32),
                                        ) & 1i32)
                                            != 0)
                                            && ((crate::c::shr_i32(
                                                ((((((&raw mut gRfuLinkStatus)
                                                    .cast::<u8>()
                                                    .cast::<*mut u8>())
                                                .read())
                                                .wrapping_add(4))
                                                .read())
                                                    as i32),
                                                ((slot_id) as u32),
                                            ) & 1i32)
                                                != 0)
                                        {
                                            break 'l3;
                                        }
                                    }
                                    i = (i).wrapping_add(1);
                                }
                            }
                            if ((i) as i32) < 4i32 {
                                rfu_STC_NI_receive_Sender(
                                    i,
                                    slot_id,
                                    (&raw mut llsf_NI).cast::<u8>(),
                                    src,
                                );
                            }
                        }
                    }
                }
            } else {
                let mut conSlots: i32 =
                    (((((((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(2))
                    .read()) as i32)
                        & (((((&raw mut llsf_NI).cast::<u8>()).wrapping_add(1)).read()) as i32));
                if (conSlots) != 0 {
                    {
                        i = 0u8;
                        'l5: loop {
                            if !(((i) as i32) < 4i32) {
                                break 'l5;
                            }
                            'l6: {
                                if (crate::c::shr_i32(conSlots, ((i) as u32)) & 1i32) != 0 {
                                    if (((((&raw mut llsf_NI).cast::<u8>()).wrapping_add(2)).read())
                                        as i32)
                                        == 4i32
                                    {
                                        rfu_STC_UNI_receive(
                                            i,
                                            (&raw mut llsf_NI).cast::<u8>(),
                                            src,
                                        );
                                    } else {
                                        if (((((&raw mut llsf_NI).cast::<u8>()).wrapping_add(3))
                                            .read())
                                            as i32)
                                            == 0i32
                                        {
                                            rfu_STC_NI_receive_Receiver(
                                                i,
                                                (&raw mut llsf_NI).cast::<u8>(),
                                                src,
                                            );
                                        } else {
                                            if (crate::c::shr_i32(
                                                ((((((&raw mut gRfuLinkStatus)
                                                    .cast::<u8>()
                                                    .cast::<*mut u8>())
                                                .read())
                                                .wrapping_add(4))
                                                .read())
                                                    as i32),
                                                ((i) as u32),
                                            ) & 1i32)
                                                != 0
                                            {
                                                rfu_STC_NI_receive_Sender(
                                                    i,
                                                    i,
                                                    (&raw mut llsf_NI).cast::<u8>(),
                                                    src,
                                                );
                                            }
                                        }
                                    }
                                }
                            }
                            i = (i).wrapping_add(1);
                        }
                    }
                }
            }
        }
        return retVal;
    }
}
pub(crate) unsafe extern "C" fn rfu_STC_UNI_receive(
    bm_slot_id: u8,
    llsf_NI: *mut u8,
    src: *mut u8,
) {
    unsafe {
        let mut bm_slot_id = bm_slot_id;
        let mut llsf_NI = llsf_NI;
        let mut src = src;
        let mut dest: *mut u8 = core::ptr::null_mut();
        let mut size: u32 = 0u32;
        let mut slotStatusUNI: *mut u8 =
            ((((&raw mut gRfuSlotStatusUNI).cast::<u8>().cast::<*mut u8>()).cast::<*mut u8>())
                .wrapping_offset(((bm_slot_id) as i32) as isize))
            .read();
        let mut UNI_recv: *mut u8 = (slotStatusUNI).wrapping_add(12);
        ((UNI_recv).wrapping_add(2).cast::<u16>()).write(0u16);
        'g_force_tail_merge: {
            if ((((((&raw mut gRfuSlotStatusUNI).cast::<u8>().cast::<*mut u8>())
                .cast::<*mut u8>())
            .wrapping_offset(((bm_slot_id) as i32) as isize))
            .read())
            .wrapping_add(24)
            .cast::<u32>())
            .read()
                < ((((llsf_NI).wrapping_add(6).cast::<u16>()).read()) as u32)
            {
                (((slotStatusUNI).wrapping_add(12)).cast::<u16>()).write(73u16);
                ((UNI_recv).wrapping_add(2).cast::<u16>()).write(1793u16);
            } else {
                if (((UNI_recv).wrapping_add(7)).read()) != 0 {
                    if (((UNI_recv).wrapping_add(6)).read()) != 0 {
                        ((UNI_recv).wrapping_add(2).cast::<u16>()).write(1801u16);
                        break 'g_force_tail_merge;
                    }
                } else {
                    if (((UNI_recv).wrapping_add(6)).read()) != 0 {
                        ((UNI_recv).wrapping_add(2).cast::<u16>()).write(1800u16);
                    }
                }
                ((UNI_recv).cast::<u16>()).write(32834u16);
                size = (({
                    let __v1 = ((llsf_NI).wrapping_add(6).cast::<u16>()).read();
                    ((UNI_recv).wrapping_add(4).cast::<u16>()).write(__v1);
                    __v1
                }) as u32);
                dest = ((((((&raw mut gRfuSlotStatusUNI).cast::<u8>().cast::<*mut u8>())
                    .cast::<*mut u8>())
                .wrapping_offset(((bm_slot_id) as i32) as isize))
                .read())
                .wrapping_add(20)
                .cast::<*mut u8>())
                .read();
                (((((&raw mut gRfuFixed).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(4)
                    .cast::<Option<unsafe extern "C" fn(*mut *mut u8, *mut *mut u8, i32)>>())
                .read())
                .unwrap_unchecked()(&raw mut src, &raw mut dest, ((size) as i32));
                ((UNI_recv).wrapping_add(6)).write(1u8);
                ((UNI_recv).cast::<u16>()).write(0u16);
            }
        }
        if (((UNI_recv).wrapping_add(2).cast::<u16>()).read()) != 0 {
            let __p2 =
                (((&raw mut gRfuStatic).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(4);
            (__p2).write(
                (((((__p2).read()) as i32) | crate::c::shl_i32(16i32, ((bm_slot_id) as u32)))
                    as u8),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn rfu_STC_NI_receive_Sender(
    NI_slot: u8,
    bm_flag: u8,
    llsf_NI: *mut u8,
    data_p: *mut u8,
) {
    unsafe {
        let mut NI_slot = NI_slot;
        let mut bm_flag = bm_flag;
        let mut llsf_NI = llsf_NI;
        let mut data_p = data_p;
        let mut NI_comm: *mut u8 =
            (((((&raw mut gRfuSlotStatusNI).cast::<u8>().cast::<*mut u8>()).cast::<*mut u8>())
                .wrapping_offset(((NI_slot) as i32) as isize))
            .read());
        let mut state: u16 = ((NI_comm).cast::<u16>()).read();
        let mut n: u8 = ((((NI_comm).wrapping_add(33)).cast::<u8>())
            .wrapping_offset(((((llsf_NI).wrapping_add(4)).read()) as i32) as isize))
        .read();
        let mut i: u8 = 0u8;
        let mut imeBak: u16 = 0u16;
        if (((((((llsf_NI).wrapping_add(2)).read()) as i32) == 2i32)
            && (((state) as i32) == 32802i32))
            || ((((((llsf_NI).wrapping_add(2)).read()) as i32) == 1i32)
                && (((state) as i32) == 32801i32)))
            || ((((((llsf_NI).wrapping_add(2)).read()) as i32) == 3i32)
                && (((state) as i32) == 32803i32))
        {
            if ((((((NI_comm).wrapping_add(33)).cast::<u8>())
                .wrapping_offset(((((llsf_NI).wrapping_add(4)).read()) as i32) as isize))
            .read()) as i32)
                == ((((llsf_NI).wrapping_add(5)).read()) as i32)
            {
                let __p1 = (((NI_comm).wrapping_add(27)).cast::<u8>())
                    .wrapping_offset(((((llsf_NI).wrapping_add(4)).read()) as i32) as isize);
                (__p1).write(
                    (((((__p1).read()) as i32) | crate::c::shl_i32(1i32, ((bm_flag) as u32)))
                        as u8),
                );
            }
        }
        if (((((((NI_comm).wrapping_add(27)).cast::<u8>())
            .wrapping_offset(((((llsf_NI).wrapping_add(4)).read()) as i32) as isize))
        .read()) as i32)
            & ((((NI_comm).wrapping_add(26)).read()) as i32))
            == ((((NI_comm).wrapping_add(26)).read()) as i32)
        {
            ((((NI_comm).wrapping_add(33)).cast::<u8>())
                .wrapping_offset(((((llsf_NI).wrapping_add(4)).read()) as i32) as isize))
            .write(
                ((((((((NI_comm).wrapping_add(33)).cast::<u8>())
                    .wrapping_offset(((((llsf_NI).wrapping_add(4)).read()) as i32) as isize))
                .read()) as i32)
                    .wrapping_add(1i32)
                    & 3i32) as u8),
            );
            ((((NI_comm).wrapping_add(27)).cast::<u8>())
                .wrapping_offset(((((llsf_NI).wrapping_add(4)).read()) as i32) as isize))
            .write(0u8);
            if (((((((NI_comm).cast::<u16>()).read()) as i32).wrapping_add((-32801i32))) as u16)
                as i32)
                <= 1i32
            {
                if ((((NI_comm).cast::<u16>()).read()) as i32) == 32801i32 {
                    let __p2 = (((NI_comm).wrapping_add(4)).cast::<*mut u8>())
                        .wrapping_offset(((((llsf_NI).wrapping_add(4)).read()) as i32) as isize);
                    (__p2).write(((__p2).read()).wrapping_offset(
                        ((((NI_comm).wrapping_add(46).cast::<u16>()).read()) as i32) as isize,
                    ));
                } else {
                    let __p3 = (((NI_comm).wrapping_add(4)).cast::<*mut u8>())
                        .wrapping_offset(((((llsf_NI).wrapping_add(4)).read()) as i32) as isize);
                    (__p3).write(((__p3).read()).wrapping_offset(
                        (((((NI_comm).wrapping_add(46).cast::<u16>()).read()) as i32) << 2)
                            as isize,
                    ));
                }
                let __p4 = (NI_comm).wrapping_add(20).cast::<u32>();
                (__p4)
                    .write(((__p4).read()).wrapping_sub(
                        ((((NI_comm).wrapping_add(46).cast::<u16>()).read()) as u32),
                    ));
                'l1: {
                    let __sw5 = ((NI_comm).wrapping_add(20).cast::<u32>()).read();
                    let __matched = __sw5 == 0u32 || (1u32..=2147483647u32).contains(&__sw5);
                    if __sw5 == 0u32 || !__matched {
                        ((NI_comm).wrapping_add(32)).write(0u8);
                        if ((((NI_comm).cast::<u16>()).read()) as i32) == 32801i32 {
                            {
                                i = 0u8;
                                'l2: loop {
                                    if !(((i) as i32) < 4i32) {
                                        break 'l2;
                                    }
                                    'l3: {
                                        ((((NI_comm).wrapping_add(33)).cast::<u8>())
                                            .wrapping_offset(((i) as i32) as isize))
                                        .write(1u8);
                                        ((((NI_comm).wrapping_add(4)).cast::<*mut u8>())
                                            .wrapping_offset(((i) as i32) as isize))
                                        .write(
                                            (((NI_comm).wrapping_add(40).cast::<*mut u8>()).read())
                                                .wrapping_offset(
                                                    (((((NI_comm).wrapping_add(46).cast::<u16>())
                                                        .read())
                                                        as i32)
                                                        .wrapping_mul(((i) as i32)))
                                                        as isize
                                                        * 1,
                                                ),
                                        );
                                    }
                                    i = (i).wrapping_add(1);
                                }
                            }
                            ((NI_comm).wrapping_add(20).cast::<u32>())
                                .write(((NI_comm).wrapping_add(48).cast::<u32>()).read());
                            ((NI_comm).cast::<u16>()).write(32802u16);
                        } else {
                            (((NI_comm).wrapping_add(33)).cast::<u8>()).write(0u8);
                            ((NI_comm).wrapping_add(20).cast::<u32>()).write(0u32);
                            ((NI_comm).cast::<u16>()).write(32803u16);
                        }
                        break 'l1;
                    }
                    if (1u32..=2147483647u32).contains(&__sw5) {
                        break 'l1;
                    }
                }
            } else {
                if ((((NI_comm).cast::<u16>()).read()) as i32) == 32803i32 {
                    ((NI_comm).cast::<u16>()).write(32800u16);
                }
            }
        }
        if ((((((NI_comm).cast::<u16>()).read()) as i32) != ((state) as i32))
            || (((((((NI_comm).wrapping_add(33)).cast::<u8>())
                .wrapping_offset(((((llsf_NI).wrapping_add(4)).read()) as i32) as isize))
            .read()) as i32)
                != ((n) as i32)))
            || ((crate::c::shr_i32(
                ((((((NI_comm).wrapping_add(27)).cast::<u8>())
                    .wrapping_offset(((((llsf_NI).wrapping_add(4)).read()) as i32) as isize))
                .read()) as i32),
                ((bm_flag) as u32),
            ) & 1i32)
                != 0)
        {
            imeBak = ((67109384i32) as usize as *mut u16).read_volatile();
            crate::c::volatile_write(((67109384i32) as usize as *mut u16), 0u16);
            let __p6 =
                (((&raw mut gRfuStatic).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(2);
            (__p6).write(
                (((((__p6).read()) as i32) | crate::c::shl_i32(16i32, ((bm_flag) as u32))) as u8),
            );
            ((((((&raw mut gRfuSlotStatusNI).cast::<u8>().cast::<*mut u8>()).cast::<*mut u8>())
                .wrapping_offset(((bm_flag) as i32) as isize))
            .read())
            .wrapping_add(2)
            .cast::<u16>())
            .write(0u16);
            crate::c::volatile_write(((67109384i32) as usize as *mut u16), imeBak);
        }
    }
}
pub(crate) unsafe extern "C" fn rfu_STC_NI_receive_Receiver(
    bm_slot_id: u8,
    llsf_NI: *mut u8,
    data_p: *mut u8,
) {
    unsafe {
        let mut bm_slot_id = bm_slot_id;
        let mut llsf_NI = llsf_NI;
        let mut data_p = data_p;
        let mut imeBak: u16 = 0u16;
        let mut state_check: u32 = 0u32;
        let mut slotStatus_NI: *mut u8 =
            ((((&raw mut gRfuSlotStatusNI).cast::<u8>().cast::<*mut u8>()).cast::<*mut u8>())
                .wrapping_offset(((bm_slot_id) as i32) as isize))
            .read();
        let mut recvSlot: *mut u8 = (slotStatus_NI).wrapping_add(52);
        let mut state: u16 = (((slotStatus_NI).wrapping_add(52)).cast::<u16>()).read();
        let mut n: u8 = (((((slotStatus_NI).wrapping_add(52)).wrapping_add(33)).cast::<u8>())
            .wrapping_offset(((((llsf_NI).wrapping_add(4)).read()) as i32) as isize))
        .read();
        if ((((llsf_NI).wrapping_add(2)).read()) as i32) == 3i32 {
            let __p1 =
                (((&raw mut gRfuStatic).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1);
            (__p1).write(
                (((((__p1).read()) as i32) | crate::c::shl_i32(1i32, ((bm_slot_id) as u32))) as u8),
            );
            if (((((slotStatus_NI).wrapping_add(52)).cast::<u16>()).read()) as i32) == 32834i32 {
                (((slotStatus_NI).wrapping_add(52)).wrapping_add(32)).write(0u8);
                ((((slotStatus_NI).wrapping_add(52)).wrapping_add(33)).cast::<u8>()).write(0u8);
                (((slotStatus_NI).wrapping_add(52)).cast::<u16>()).write(32835u16);
            }
        } else {
            if ((((llsf_NI).wrapping_add(2)).read()) as i32) == 2i32 {
                if (((state) as i32) == 32833i32)
                    && (!((((recvSlot).wrapping_add(20).cast::<u32>()).read()) != 0))
                {
                    rfu_STC_NI_initSlot_asRecvDataEntity(bm_slot_id, recvSlot);
                }
                if ((((recvSlot).cast::<u16>()).read()) as i32) == 32834i32 {
                    state_check = 1u32;
                }
            } else {
                if ((((llsf_NI).wrapping_add(2)).read()) as i32) == 1i32 {
                    if ((state) as i32) == 32833i32 {
                        state_check = 1u32;
                    } else {
                        rfu_STC_NI_initSlot_asRecvControllData(bm_slot_id, recvSlot);
                        if (((((slotStatus_NI).wrapping_add(52)).cast::<u16>()).read()) as i32)
                            != 32833i32
                        {
                            return;
                        }
                        state_check = 1u32;
                    }
                }
            }
        }
        if state_check != 0u32 {
            if ((((llsf_NI).wrapping_add(5)).read()) as i32)
                == (((((((recvSlot).wrapping_add(33)).cast::<u8>())
                    .wrapping_offset(((((llsf_NI).wrapping_add(4)).read()) as i32) as isize))
                .read()) as i32)
                    .wrapping_add(1i32)
                    & 3i32)
            {
                (((((&raw mut gRfuFixed).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(4)
                    .cast::<Option<unsafe extern "C" fn(*mut *mut u8, *mut *mut u8, i32)>>())
                .read())
                .unwrap_unchecked()(
                    &raw mut data_p,
                    (((recvSlot).wrapping_add(4)).cast::<*mut u8>())
                        .wrapping_offset(((((llsf_NI).wrapping_add(4)).read()) as i32) as isize),
                    ((((llsf_NI).wrapping_add(6).cast::<u16>()).read()) as i32),
                );
                if ((((recvSlot).cast::<u16>()).read()) as i32) == 32834i32 {
                    let __p2 = (((recvSlot).wrapping_add(4)).cast::<*mut u8>())
                        .wrapping_offset(((((llsf_NI).wrapping_add(4)).read()) as i32) as isize);
                    (__p2).write(((__p2).read()).wrapping_offset(
                        ((3i32).wrapping_mul(
                            ((((recvSlot).wrapping_add(46).cast::<u16>()).read()) as i32),
                        )) as isize,
                    ));
                }
                let __p3 = (recvSlot).wrapping_add(20).cast::<u32>();
                (__p3).write(
                    ((__p3).read())
                        .wrapping_sub(((((llsf_NI).wrapping_add(6).cast::<u16>()).read()) as u32)),
                );
                ((((recvSlot).wrapping_add(33)).cast::<u8>())
                    .wrapping_offset(((((llsf_NI).wrapping_add(4)).read()) as i32) as isize))
                .write(((llsf_NI).wrapping_add(5)).read());
            }
        }
        if ((((recvSlot).wrapping_add(24).cast::<u16>()).read()) as i32) == 0i32 {
            ((recvSlot).wrapping_add(32)).write(((llsf_NI).wrapping_add(4)).read());
            if ((((((recvSlot).cast::<u16>()).read()) as i32) != ((state) as i32))
                || (((((((recvSlot).wrapping_add(33)).cast::<u8>())
                    .wrapping_offset(((((llsf_NI).wrapping_add(4)).read()) as i32) as isize))
                .read()) as i32)
                    != ((n) as i32)))
                || (((((((recvSlot).wrapping_add(33)).cast::<u8>())
                    .wrapping_offset(((((llsf_NI).wrapping_add(4)).read()) as i32) as isize))
                .read()) as i32)
                    == ((((llsf_NI).wrapping_add(5)).read()) as i32))
            {
                imeBak = ((67109384i32) as usize as *mut u16).read_volatile();
                crate::c::volatile_write(((67109384i32) as usize as *mut u16), 0u16);
                let __p4 =
                    (((&raw mut gRfuStatic).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(2);
                (__p4).write(
                    (((((__p4).read()) as i32) | crate::c::shl_i32(1i32, ((bm_slot_id) as u32)))
                        as u8),
                );
                ((recvSlot).wrapping_add(2).cast::<u16>()).write(0u16);
                crate::c::volatile_write(((67109384i32) as usize as *mut u16), imeBak);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn rfu_STC_NI_initSlot_asRecvControllData(
    bm_slot_id: u8,
    NI_comm: *mut u8,
) {
    unsafe {
        let mut bm_slot_id = bm_slot_id;
        let mut NI_comm = NI_comm;
        let mut llFrameSize_p: *mut u8 = core::ptr::null_mut();
        let mut llFrameSize: u32 = 0u32;
        let mut bm_slot_flag: u8 = 0u8;
        if (((((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>()).read()).read()) as i32)
            == 1i32
        {
            llFrameSize = 3u32;
            llFrameSize_p = (((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(15);
        } else {
            llFrameSize = 2u32;
            llFrameSize_p = (((((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(16))
            .cast::<u8>())
            .wrapping_offset(((bm_slot_id) as i32) as isize);
        }
        bm_slot_flag = ((crate::c::shl_i32(1i32, ((bm_slot_id) as u32))) as u8);
        if ((((NI_comm).cast::<u16>()).read()) as i32) == 0i32 {
            if (((llFrameSize_p).read()) as u32) < llFrameSize {
                ((NI_comm).cast::<u16>()).write(73u16);
                ((NI_comm).wrapping_add(24).cast::<u16>()).write(1794u16);
                let __p1 =
                    (((&raw mut gRfuStatic).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(4);
                (__p1).write((((((__p1).read()) as i32) | ((bm_slot_flag) as i32)) as u8));
            } else {
                ((NI_comm).wrapping_add(24).cast::<u16>()).write(0u16);
                (llFrameSize_p)
                    .write((((((llFrameSize_p).read()) as u32).wrapping_sub(llFrameSize)) as u8));
                (((NI_comm).wrapping_add(4)).cast::<*mut u8>()).write((NI_comm).wrapping_add(45));
                ((NI_comm).wrapping_add(20).cast::<u32>()).write(7u32);
                ((NI_comm).wrapping_add(31)).write(1u8);
                ((NI_comm).wrapping_add(46).cast::<u16>()).write(0u16);
                ((NI_comm).wrapping_add(26)).write(bm_slot_flag);
                ((NI_comm).cast::<u16>()).write(32833u16);
                let __p2 = (((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(5);
                (__p2).write((((((__p2).read()) as i32) | ((bm_slot_flag) as i32)) as u8));
            }
        }
    }
}
pub(crate) unsafe extern "C" fn rfu_STC_NI_initSlot_asRecvDataEntity(
    bm_slot_id: u8,
    NI_comm: *mut u8,
) {
    unsafe {
        let mut bm_slot_id = bm_slot_id;
        let mut NI_comm = NI_comm;
        let mut bm_slot_flag: u8 = 0u8;
        let mut win_id: u8 = 0u8;
        if ((((NI_comm).wrapping_add(45)).read()) as i32) == 1i32 {
            (((NI_comm).wrapping_add(4)).cast::<*mut u8>()).write(
                (((((((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(20))
                .cast::<u8>())
                .wrapping_offset(((bm_slot_id) as i32) as isize * 32))
                .wrapping_add(4)
                .cast::<u16>())
                .cast::<u8>(),
            );
        } else {
            if ((NI_comm).wrapping_add(48).cast::<u32>()).read()
                > ((((((&raw mut gRfuSlotStatusNI).cast::<u8>().cast::<*mut u8>())
                    .cast::<*mut u8>())
                .wrapping_offset(((bm_slot_id) as i32) as isize))
                .read())
                .wrapping_add(108)
                .cast::<u32>())
                .read()
            {
                bm_slot_flag = ((crate::c::shl_i32(1i32, ((bm_slot_id) as u32))) as u8);
                let __p1 =
                    (((&raw mut gRfuStatic).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(4);
                (__p1).write((((((__p1).read()) as i32) | ((bm_slot_flag) as i32)) as u8));
                let __p2 = (((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(5);
                (__p2).write((((((__p2).read()) as i32) & !((bm_slot_flag) as i32)) as u8));
                ((NI_comm).wrapping_add(24).cast::<u16>()).write(1793u16);
                ((NI_comm).cast::<u16>()).write(71u16);
                rfu_STC_releaseFrame(bm_slot_id, 1u8, NI_comm);
                return;
            }
            (((NI_comm).wrapping_add(4)).cast::<*mut u8>()).write(
                ((((((&raw mut gRfuSlotStatusNI).cast::<u8>().cast::<*mut u8>())
                    .cast::<*mut u8>())
                .wrapping_offset(((bm_slot_id) as i32) as isize))
                .read())
                .wrapping_add(104)
                .cast::<*mut u8>())
                .read(),
            );
        }
        {
            win_id = 0u8;
            'l1: loop {
                if !(((win_id) as i32) < 4i32) {
                    break 'l1;
                }
                'l2: {
                    ((((NI_comm).wrapping_add(33)).cast::<u8>())
                        .wrapping_offset(((win_id) as i32) as isize))
                    .write(0u8);
                    ((((NI_comm).wrapping_add(4)).cast::<*mut u8>())
                        .wrapping_offset(((win_id) as i32) as isize))
                    .write(
                        ((((NI_comm).wrapping_add(4)).cast::<*mut u8>()).read()).wrapping_offset(
                            (((((NI_comm).wrapping_add(46).cast::<u16>()).read()) as i32)
                                .wrapping_mul(((win_id) as i32)))
                                as isize,
                        ),
                    );
                }
                win_id = (win_id).wrapping_add(1);
            }
        }
        ((NI_comm).wrapping_add(20).cast::<u32>())
            .write(((NI_comm).wrapping_add(48).cast::<u32>()).read());
        ((NI_comm).cast::<u16>()).write(32834u16);
    }
}
pub(crate) unsafe extern "C" fn rfu_NI_checkCommFailCounter() {
    unsafe {
        let mut imeBak: u16 = 0u16;
        let mut recvRenewalFlag: u32 = 0u32;
        let mut bm_slot_flag: u8 = 0u8;
        let mut bm_slot_id: u8 = 0u8;
        if (((((((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(4))
            .read()) as i32)
            | ((((((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(5))
            .read()) as i32))
            != 0
        {
            imeBak = ((67109384i32) as usize as *mut u16).read_volatile();
            crate::c::volatile_write(((67109384i32) as usize as *mut u16), 0u16);
            recvRenewalFlag = ((((((((&raw mut gRfuStatic).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(2))
            .read()) as i32)
                >> 4) as u32);
            {
                bm_slot_id = 0u8;
                'l1: loop {
                    if !(((bm_slot_id) as i32) < 4i32) {
                        break 'l1;
                    }
                    'l2: {
                        bm_slot_flag = ((crate::c::shl_i32(1i32, ((bm_slot_id) as u32))) as u8);
                        if ((((((((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(4))
                        .read()) as i32)
                            & ((bm_slot_flag) as i32))
                            != 0)
                            && (!((((((((&raw mut gRfuStatic).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(2))
                            .read()) as i32)
                                & ((bm_slot_flag) as i32))
                                != 0))
                        {
                            let __p1 =
                                (((((&raw mut gRfuSlotStatusNI).cast::<u8>().cast::<*mut u8>())
                                    .cast::<*mut u8>())
                                .wrapping_offset(((bm_slot_id) as i32) as isize))
                                .read())
                                .wrapping_add(2)
                                .cast::<u16>();
                            (__p1).write(((__p1).read()).wrapping_add(1));
                        }
                        if ((((((((&raw mut gRfuLinkStatus).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(5))
                        .read()) as i32)
                            & ((bm_slot_flag) as i32))
                            != 0)
                            && (!((recvRenewalFlag & ((bm_slot_flag) as u32)) != 0))
                        {
                            let __p2 = ((((((&raw mut gRfuSlotStatusNI)
                                .cast::<u8>()
                                .cast::<*mut u8>())
                            .cast::<*mut u8>())
                            .wrapping_offset(((bm_slot_id) as i32) as isize))
                            .read())
                            .wrapping_add(52))
                            .wrapping_add(2)
                            .cast::<u16>();
                            (__p2).write(((__p2).read()).wrapping_add(1));
                        }
                    }
                    bm_slot_id = (bm_slot_id).wrapping_add(1);
                }
            }
            ((((&raw mut gRfuStatic).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(2))
                .write(0u8);
            crate::c::volatile_write(((67109384i32) as usize as *mut u16), imeBak);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rfu_REQ_noise() {
    unsafe {
        STWI_set_Callback_M(core::mem::transmute::<
            Option<unsafe extern "C" fn(u8, u16)>,
            *mut u8,
        >(Some(
            rfu_STC_REQ_callback as unsafe extern "C" fn(u8, u16),
        )));
        STWI_send_TestModeREQ(1u8, 0u8);
    }
}
