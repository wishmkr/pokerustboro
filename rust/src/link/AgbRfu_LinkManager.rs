//! Translated from `src/AgbRfu_LinkManager.c` by tools/rustport/c2rs.py, then reviewed.
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
pub static mut lman: crate::ffi::Align4<[u8; 72]> = crate::ffi::Align4([0; 72]);

unsafe extern "C" {
    static mut gRfuLinkStatus: u8;
    static mut gRfuSlotStatusNI: u8;
    static mut gRfuSlotStatusUNI: u8;
    fn CpuSet(a0: *mut u8, a1: *mut u8, a2: u32);
    fn rfu_CHILD_getConnectRecoveryStatus(a0: *mut u8) -> u16;
    fn rfu_NI_CHILD_setSendGameName(a0: u8, a1: u8) -> u16;
    fn rfu_NI_stopReceivingData(a0: u8) -> u16;
    fn rfu_REQBN_softReset_and_checkID() -> u32;
    fn rfu_REQBN_watchLink(a0: u16, a1: *mut u8, a2: *mut u8, a3: *mut u8) -> u16;
    fn rfu_REQ_CHILD_endConnectRecovery();
    fn rfu_REQ_CHILD_pollConnectRecovery();
    fn rfu_REQ_CHILD_startConnectRecovery(a0: u8);
    fn rfu_REQ_RFUStatus();
    fn rfu_REQ_changeMasterSlave();
    fn rfu_REQ_configGameData(a0: u8, a1: u16, a2: *mut u8, a3: *mut u8);
    fn rfu_REQ_configSystem(a0: u16, a1: u8, a2: u8);
    fn rfu_REQ_disconnect(a0: u8);
    fn rfu_REQ_endConnectParent();
    fn rfu_REQ_endSearchChild();
    fn rfu_REQ_endSearchParent();
    fn rfu_REQ_pollConnectParent();
    fn rfu_REQ_pollSearchChild();
    fn rfu_REQ_pollSearchParent();
    fn rfu_REQ_reset();
    fn rfu_REQ_sendData(a0: u8);
    fn rfu_REQ_startConnectParent(a0: u16);
    fn rfu_REQ_startSearchChild();
    fn rfu_REQ_startSearchParent();
    fn rfu_REQ_stopMode();
    fn rfu_UNI_PARENT_getDRAC_ACK(a0: *mut u8) -> u16;
    fn rfu_changeSendTarget(a0: u8, a1: u8, a2: u8) -> u16;
    fn rfu_clearSlot(a0: u8, a1: u8) -> u16;
    fn rfu_getConnectParentStatus(a0: *mut u8, a1: *mut u8) -> u16;
    fn rfu_getMasterSlave() -> u8;
    fn rfu_getRFUStatus(a0: *mut u8) -> u16;
    fn rfu_getSTWIRecvBuffer() -> *mut u8;
    fn rfu_setMSCCallback(a0: Option<unsafe extern "C" fn(u16)>);
    fn rfu_setREQCallback(a0: Option<unsafe extern "C" fn(u16, u16)>);
    fn rfu_syncVBlank() -> u16;
    fn rfu_waitREQComplete() -> u16;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn rfu_LMAN_REQBN_softReset_and_checkID() -> u32 {
    unsafe {
        let mut id: u32 = rfu_REQBN_softReset_and_checkID();
        if id == 32769u32 {
            (((&raw mut lman).cast::<u8>()).wrapping_add(8)).write(1u8);
        }
        if ((((((&raw mut lman).cast::<u8>()).wrapping_add(4)).read()) as i32) != 23i32)
            && ((((((&raw mut lman).cast::<u8>()).wrapping_add(4)).read()) as i32) != 1i32)
        {
            (((&raw mut lman).cast::<u8>()).wrapping_add(4)).write({
                let __v1 = 0u8;
                (((&raw mut lman).cast::<u8>()).wrapping_add(5)).write(__v1);
                __v1
            });
        }
        (((&raw mut lman).cast::<u8>()).wrapping_add(7)).write(0u8);
        (((&raw mut lman).cast::<u8>()).wrapping_add(13)).write(0u8);
        (((&raw mut lman).cast::<u8>()).wrapping_add(1)).write(0u8);
        ((&raw mut lman).cast::<u8>()).write(0u8);
        (((&raw mut lman).cast::<u8>()).wrapping_add(6)).write(255u8);
        rfu_LMAN_managerChangeAgbClockMaster();
        return id;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rfu_LMAN_REQ_sendData(clockChangeFlag: u8) {
    unsafe {
        let mut clockChangeFlag = clockChangeFlag;
        if (((((&raw mut gRfuLinkStatus).cast::<*mut u8>()).read()).read()) as i32) == 0i32 {
            if (((((&raw mut lman).cast::<u8>()).wrapping_add(2)).read_volatile()) as i32) == 1i32 {
                clockChangeFlag = 1u8;
            } else {
                clockChangeFlag = 0u8;
            }
        } else {
            crate::c::volatile_write(((&raw mut lman).cast::<u8>()).wrapping_add(3), 0u8);
        }
        rfu_REQ_sendData(clockChangeFlag);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rfu_LMAN_initializeManager(
    LMAN_callback_p: Option<unsafe extern "C" fn(u8, u8)>,
    MSC_callback_p: Option<unsafe extern "C" fn(u16)>,
) -> u8 {
    unsafe {
        let mut LMAN_callback_p = LMAN_callback_p;
        let mut MSC_callback_p = MSC_callback_p;
        if core::mem::transmute::<_, usize>(LMAN_callback_p) == 0usize {
            return 4u8;
        }
        'l1: loop {
            'l2: {
                {
                    let mut tmp: u16 = 0u16;
                    (&raw mut tmp).write_volatile(0u16);
                    'l3: loop {
                        'l4: {
                            CpuSet(
                                (&raw mut tmp).cast::<u8>(),
                                (&raw mut lman).cast::<u8>(),
                                (16777216u32
                                    | (crate::c::div_u32(
                                        72u32,
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
        (((&raw mut lman).cast::<u8>()).wrapping_add(6)).write(255u8);
        (((&raw mut lman).cast::<u8>())
            .wrapping_add(64)
            .cast::<Option<unsafe extern "C" fn(u8, u8)>>())
        .write(LMAN_callback_p);
        (((&raw mut lman).cast::<u8>())
            .wrapping_add(68)
            .cast::<Option<unsafe extern "C" fn(u16)>>())
        .write(MSC_callback_p);
        rfu_setMSCCallback(Some(rfu_LMAN_MSC_callback));
        rfu_setREQCallback(Some(rfu_LMAN_REQ_callback));
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn rfu_LMAN_endManager() {
    unsafe {
        'l1: loop {
            'l2: {
                {
                    let mut tmp: u16 = 0u16;
                    (&raw mut tmp).write_volatile(0u16);
                    'l3: loop {
                        'l4: {
                            CpuSet(
                                (&raw mut tmp).cast::<u8>(),
                                (&raw mut lman).cast::<u8>(),
                                (16777216u32
                                    | (crate::c::div_u32(
                                        64u32,
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
        (((&raw mut lman).cast::<u8>()).wrapping_add(6)).write(255u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rfu_LMAN_initializeRFU(init_parameters: *mut u8) {
    unsafe {
        let mut init_parameters = init_parameters;
        rfu_LMAN_clearVariables();
        (((&raw mut lman).cast::<u8>()).wrapping_add(4)).write(1u8);
        (((&raw mut lman).cast::<u8>()).wrapping_add(5)).write(2u8);
        (((&raw mut lman).cast::<u8>())
            .wrapping_add(60)
            .cast::<*mut u8>())
        .write(init_parameters);
        (((&raw mut lman).cast::<u8>()).wrapping_add(9))
            .write(((init_parameters).wrapping_add(17)).read());
        ((((&raw mut lman).cast::<u8>()).wrapping_add(48))
            .wrapping_add(2)
            .cast::<u16>())
        .write(((init_parameters).wrapping_add(18).cast::<u16>()).read());
        (((&raw mut lman).cast::<u8>())
            .wrapping_add(24)
            .cast::<u16>())
        .write(((init_parameters).wrapping_add(20).cast::<u16>()).read());
        if (((init_parameters).wrapping_add(16)).read()) != 0 {
            (((&raw mut lman).cast::<u8>()).wrapping_add(11)).write(1u8);
        }
    }
}
pub(crate) unsafe extern "C" fn rfu_LMAN_clearVariables() {
    unsafe {
        let mut i: u8 = 0u8;
        (((&raw mut lman).cast::<u8>()).wrapping_add(4)).write({
            let __v1 = 0u8;
            (((&raw mut lman).cast::<u8>()).wrapping_add(5)).write(__v1);
            __v1
        });
        (((&raw mut lman).cast::<u8>()).wrapping_add(6)).write(255u8);
        (((&raw mut lman).cast::<u8>()).wrapping_add(7)).write(0u8);
        (((&raw mut lman).cast::<u8>()).wrapping_add(16)).write(0u8);
        (((&raw mut lman).cast::<u8>()).wrapping_add(12)).write(0u8);
        (((&raw mut lman).cast::<u8>()).wrapping_add(36)).write(0u8);
        (((&raw mut lman).cast::<u8>()).wrapping_add(48)).write(0u8);
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 4i32) {
                    break 'l1;
                }
                'l2: {
                    ((((((&raw mut lman).cast::<u8>()).wrapping_add(36)).wrapping_add(4))
                        .cast::<u16>())
                    .wrapping_offset(((i) as i32) as isize))
                    .write(0u16);
                    ((((((&raw mut lman).cast::<u8>()).wrapping_add(48)).wrapping_add(4))
                        .cast::<u16>())
                    .wrapping_offset(((i) as i32) as isize))
                    .write(0u16);
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rfu_LMAN_powerDownRFU() {
    unsafe {
        (((&raw mut lman).cast::<u8>()).wrapping_add(4)).write(21u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rfu_LMAN_establishConnection(
    parent_child: u8,
    connect_period: u16,
    name_accept_period: u16,
    acceptable_serialNo_list: *mut u16,
) -> u8 {
    unsafe {
        let mut parent_child = parent_child;
        let mut connect_period = connect_period;
        let mut name_accept_period = name_accept_period;
        let mut acceptable_serialNo_list = acceptable_serialNo_list;
        let mut i: u8 = 0u8;
        let mut serial_list: *mut u16 = core::ptr::null_mut();
        if ((((((&raw mut lman).cast::<u8>()).wrapping_add(4)).read()) as i32) != 0i32)
            && (((((((&raw mut lman).cast::<u8>()).wrapping_add(4)).read()) as i32) != 8i32)
                || (((parent_child) as i32) != 1i32))
        {
            ((((&raw mut lman).cast::<u8>()).wrapping_add(20)).cast::<u16>()).write(1u16);
            rfu_LMAN_occureCallback(243u8, 1u8);
            return 1u8;
        }
        if ((rfu_getMasterSlave()) as i32) == 0i32 {
            ((((&raw mut lman).cast::<u8>()).wrapping_add(20)).cast::<u16>()).write(2u16);
            rfu_LMAN_occureCallback(243u8, 1u8);
            return 2u8;
        }
        {
            i = 0u8;
            serial_list = acceptable_serialNo_list;
            'l1: loop {
                if !(((i) as i32) < 16i32) {
                    break 'l1;
                }
                'l2: {
                    if ((({
                        let __t2 = serial_list;
                        serial_list = (serial_list).wrapping_offset(1);
                        __t2
                    })
                    .read()) as i32)
                        == 65535i32
                    {
                        break 'l1;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        if ((i) as i32) == 16i32 {
            ((((&raw mut lman).cast::<u8>()).wrapping_add(20)).cast::<u16>()).write(4u16);
            rfu_LMAN_occureCallback(243u8, 1u8);
            return 4u8;
        }
        if ((parent_child) as i32) > 1i32 {
            (((&raw mut lman).cast::<u8>()).wrapping_add(7)).write(1u8);
            parent_child = 1u8;
            connect_period = 0u16;
        } else {
            (((&raw mut lman).cast::<u8>()).wrapping_add(7)).write(0u8);
        }
        if ((parent_child) as i32) != 0i32 {
            (((&raw mut lman).cast::<u8>()).wrapping_add(4)).write(5u8);
        } else {
            (((&raw mut lman).cast::<u8>()).wrapping_add(4)).write(9u8);
            if ((((&raw mut lman).cast::<u8>()).wrapping_add(11)).read()) != 0 {
                (((&raw mut lman).cast::<u8>()).wrapping_add(11)).write(2u8);
            }
        }
        (((&raw mut lman).cast::<u8>()).wrapping_add(6)).write(parent_child);
        (((&raw mut lman).cast::<u8>())
            .wrapping_add(26)
            .cast::<u16>())
        .write(connect_period);
        ((((&raw mut lman).cast::<u8>()).wrapping_add(36))
            .wrapping_add(2)
            .cast::<u16>())
        .write(name_accept_period);
        (((&raw mut lman).cast::<u8>())
            .wrapping_add(32)
            .cast::<*mut u16>())
        .write(acceptable_serialNo_list);
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rfu_LMAN_CHILD_connectParent(parentId: u16, connect_period: u16) -> u8 {
    unsafe {
        let mut parentId = parentId;
        let mut connect_period = connect_period;
        let mut i: u8 = 0u8;
        if ((((((&raw mut lman).cast::<u8>()).wrapping_add(4)).read()) as i32) != 0i32)
            && (((((((&raw mut lman).cast::<u8>()).wrapping_add(4)).read()) as i32) < 9i32)
                || ((((((&raw mut lman).cast::<u8>()).wrapping_add(4)).read()) as i32) > 11i32))
        {
            ((((&raw mut lman).cast::<u8>()).wrapping_add(20)).cast::<u16>()).write(1u16);
            rfu_LMAN_occureCallback(243u8, 1u8);
            return 1u8;
        }
        if ((rfu_getMasterSlave()) as i32) == 0i32 {
            ((((&raw mut lman).cast::<u8>()).wrapping_add(20)).cast::<u16>()).write(2u16);
            rfu_LMAN_occureCallback(243u8, 1u8);
            return 2u8;
        }
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32)
                    < ((((((&raw mut gRfuLinkStatus).cast::<*mut u8>()).read()).wrapping_add(8))
                        .read()) as i32))
                {
                    break 'l1;
                }
                'l2: {
                    if (((((((((&raw mut gRfuLinkStatus).cast::<*mut u8>()).read())
                        .wrapping_add(20))
                    .cast::<u8>())
                    .wrapping_offset(((i) as i32) as isize * 32))
                    .cast::<u16>())
                    .read()) as i32)
                        == ((parentId) as i32)
                    {
                        break 'l1;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        if (((((((&raw mut gRfuLinkStatus).cast::<*mut u8>()).read()).wrapping_add(8)).read())
            as i32)
            == 0i32)
            || (((i) as i32)
                == ((((((&raw mut gRfuLinkStatus).cast::<*mut u8>()).read()).wrapping_add(8))
                    .read()) as i32))
        {
            ((((&raw mut lman).cast::<u8>()).wrapping_add(20)).cast::<u16>()).write(3u16);
            rfu_LMAN_occureCallback(243u8, 1u8);
            return 3u8;
        }
        if ((((((&raw mut lman).cast::<u8>()).wrapping_add(4)).read()) as i32) == 0i32)
            || ((((((&raw mut lman).cast::<u8>()).wrapping_add(4)).read()) as i32) == 9i32)
        {
            (((&raw mut lman).cast::<u8>()).wrapping_add(4)).write(12u8);
            (((&raw mut lman).cast::<u8>()).wrapping_add(5)).write(13u8);
        } else {
            (((&raw mut lman).cast::<u8>()).wrapping_add(4)).write(11u8);
            (((&raw mut lman).cast::<u8>()).wrapping_add(5)).write(12u8);
        }
        (((&raw mut lman).cast::<u8>())
            .wrapping_add(30)
            .cast::<u16>())
        .write(parentId);
        (((&raw mut lman).cast::<u8>())
            .wrapping_add(26)
            .cast::<u16>())
        .write(connect_period);
        if (((((&raw mut lman).cast::<u8>()).wrapping_add(7)).read()) as i32) != 0i32 {
            (((&raw mut lman).cast::<u8>()).wrapping_add(7)).write(7u8);
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn rfu_LMAN_PARENT_stopWaitLinkRecoveryAndDisconnect(
    bm_targetSlot: u8,
) {
    unsafe {
        let mut bm_targetSlot = bm_targetSlot;
        let mut i: u8 = 0u8;
        if (((bm_targetSlot) as i32)
            & (((((&raw mut lman).cast::<u8>()).wrapping_add(48)).read()) as i32))
            == 0i32
        {
            return;
        }
        let __p1 = (((&raw mut lman).cast::<u8>()).wrapping_add(48));
        (__p1).write((((((__p1).read()) as i32) & !((bm_targetSlot) as i32)) as u8));
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 4i32) {
                    break 'l1;
                }
                'l2: {
                    if (crate::c::shr_i32(((bm_targetSlot) as i32), ((i) as u32)) & 1i32) != 0 {
                        ((((((&raw mut lman).cast::<u8>()).wrapping_add(48)).wrapping_add(4))
                            .cast::<u16>())
                        .wrapping_offset(((i) as i32) as isize))
                        .write(0u16);
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        i = ((((((((&raw mut gRfuLinkStatus).cast::<*mut u8>()).read()).wrapping_add(3)).read())
            as i32)
            & ((bm_targetSlot) as i32)) as u8);
        if (i) != 0 {
            rfu_LMAN_disconnect(i);
        }
        ((((&raw mut lman).cast::<u8>()).wrapping_add(20)).cast::<u16>()).write(((i) as u16));
        rfu_LMAN_occureCallback(51u8, i);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rfu_LMAN_stopManager(forced_stop_and_RFU_reset_flag: u8) {
    unsafe {
        let mut forced_stop_and_RFU_reset_flag = forced_stop_and_RFU_reset_flag;
        let mut msg: u8 = 0u8;
        (((&raw mut lman).cast::<u8>()).wrapping_add(7)).write(0u8);
        if (forced_stop_and_RFU_reset_flag) != 0 {
            rfu_LMAN_clearVariables();
            (((&raw mut lman).cast::<u8>()).wrapping_add(4)).write(23u8);
            return;
        }
        'l1: {
            let __sw1 = (((((&raw mut lman).cast::<u8>()).wrapping_add(4)).read()) as i32);
            let __matched = __sw1 == 5i32
                || __sw1 == 6i32
                || __sw1 == 7i32
                || __sw1 == 8i32
                || __sw1 == 9i32
                || __sw1 == 10i32
                || __sw1 == 11i32
                || __sw1 == 12i32
                || __sw1 == 13i32
                || __sw1 == 14i32
                || __sw1 == 15i32
                || __sw1 == 16i32
                || __sw1 == 17i32
                || __sw1 == 18i32;
            if __sw1 == 5i32 {
                (((&raw mut lman).cast::<u8>()).wrapping_add(4)).write(8u8);
                (((&raw mut lman).cast::<u8>()).wrapping_add(5)).write(0u8);
                msg = 19u8;
                break 'l1;
            }
            if __sw1 == 6i32 {
                (((&raw mut lman).cast::<u8>()).wrapping_add(4)).write(7u8);
                (((&raw mut lman).cast::<u8>()).wrapping_add(5)).write(8u8);
                break 'l1;
            }
            if __sw1 == 7i32 {
                (((&raw mut lman).cast::<u8>()).wrapping_add(4)).write(7u8);
                (((&raw mut lman).cast::<u8>()).wrapping_add(5)).write(8u8);
                break 'l1;
            }
            if __sw1 == 8i32 {
                break 'l1;
            }
            if __sw1 == 9i32 {
                (((&raw mut lman).cast::<u8>()).wrapping_add(4)).write({
                    let __v2 = 0u8;
                    (((&raw mut lman).cast::<u8>()).wrapping_add(5)).write(__v2);
                    __v2
                });
                msg = 33u8;
                break 'l1;
            }
            if __sw1 == 10i32 {
                (((&raw mut lman).cast::<u8>()).wrapping_add(4)).write(11u8);
                (((&raw mut lman).cast::<u8>()).wrapping_add(5)).write(0u8);
                break 'l1;
            }
            if __sw1 == 11i32 {
                (((&raw mut lman).cast::<u8>()).wrapping_add(4)).write(11u8);
                (((&raw mut lman).cast::<u8>()).wrapping_add(5)).write(0u8);
                break 'l1;
            }
            if __sw1 == 12i32 {
                (((&raw mut lman).cast::<u8>()).wrapping_add(4)).write({
                    let __v3 = 0u8;
                    (((&raw mut lman).cast::<u8>()).wrapping_add(5)).write(__v3);
                    __v3
                });
                msg = 35u8;
                break 'l1;
            }
            if __sw1 == 13i32 {
                (((&raw mut lman).cast::<u8>()).wrapping_add(4)).write(14u8);
                break 'l1;
            }
            if __sw1 == 14i32 {
                (((&raw mut lman).cast::<u8>()).wrapping_add(4)).write(14u8);
                break 'l1;
            }
            if __sw1 == 15i32 {
                break 'l1;
            }
            if __sw1 == 16i32 {
                (((&raw mut lman).cast::<u8>()).wrapping_add(4))
                    .write(((((&raw mut lman).cast::<u8>()).wrapping_add(17)).cast::<u8>()).read());
                (((&raw mut lman).cast::<u8>()).wrapping_add(5)).write(
                    (((((&raw mut lman).cast::<u8>()).wrapping_add(17)).cast::<u8>())
                        .wrapping_offset(1))
                    .read(),
                );
                rfu_LMAN_disconnect(
                    ((((&raw mut gRfuLinkStatus).cast::<*mut u8>()).read()).wrapping_add(3)).read(),
                );
                ((((&raw mut lman).cast::<u8>()).wrapping_add(20)).cast::<u16>()).write(
                    ((((((&raw mut gRfuLinkStatus).cast::<*mut u8>()).read()).wrapping_add(3))
                        .read()) as u16),
                );
                rfu_LMAN_occureCallback(51u8, 1u8);
                return;
            }
            if __sw1 == 17i32 {
                (((&raw mut lman).cast::<u8>()).wrapping_add(4)).write(18u8);
                break 'l1;
            }
            if __sw1 == 18i32 {
                (((&raw mut lman).cast::<u8>()).wrapping_add(4)).write(18u8);
                break 'l1;
            }
            if !__matched {
                (((&raw mut lman).cast::<u8>()).wrapping_add(4)).write({
                    let __v4 = 0u8;
                    (((&raw mut lman).cast::<u8>()).wrapping_add(5)).write(__v4);
                    __v4
                });
                msg = 67u8;
                break 'l1;
            }
        }
        if (((((&raw mut lman).cast::<u8>()).wrapping_add(4)).read()) as i32) == 0i32 {
            rfu_LMAN_occureCallback(msg, 0u8);
        }
    }
}
pub(crate) unsafe extern "C" fn rfu_LMAN_linkWatcher(REQ_commandID: u16) -> u8 {
    unsafe {
        let mut REQ_commandID = REQ_commandID;
        let mut i: u8 = 0u8;
        let mut bm_linkLossSlot: u8 = 0u8;
        let mut reason: u8 = 0u8;
        let mut bm_linkRecoverySlot: u8 = 0u8;
        let mut bm_disconnectSlot: u8 = 0u8;
        let mut disconnect_occure_flag: u8 = 0u8;
        rfu_REQBN_watchLink(
            REQ_commandID,
            &raw mut bm_linkLossSlot,
            &raw mut reason,
            &raw mut bm_linkRecoverySlot,
        );
        if (bm_linkLossSlot) != 0 {
            ((((&raw mut lman).cast::<u8>()).wrapping_add(20)).cast::<u16>())
                .write(((bm_linkLossSlot) as u16));
            (((((&raw mut lman).cast::<u8>()).wrapping_add(20)).cast::<u16>()).wrapping_offset(1))
                .write(((reason) as u16));
            if ((((&raw mut lman).cast::<u8>()).wrapping_add(9)).read()) != 0 {
                (((&raw mut lman).cast::<u8>()).wrapping_add(10)).write(1u8);
                if ((((((&raw mut lman).cast::<u8>()).wrapping_add(6)).read()) as i32) == 0i32)
                    && (((reason) as i32) == 0i32)
                {
                    (((&raw mut lman).cast::<u8>()).wrapping_add(10)).write(4u8);
                }
                if (((((&raw mut lman).cast::<u8>()).wrapping_add(10)).read()) as i32) == 1i32 {
                    {
                        i = 0u8;
                        'l1: loop {
                            if !(((i) as i32) < 4i32) {
                                break 'l1;
                            }
                            'l2: {
                                if (crate::c::shr_i32(((bm_linkLossSlot) as i32), ((i) as u32))
                                    & 1i32)
                                    != 0
                                {
                                    let __p1 = (((&raw mut lman).cast::<u8>()).wrapping_add(48));
                                    (__p1).write(
                                        (((((__p1).read()) as i32)
                                            | crate::c::shl_i32(1i32, ((i) as u32)))
                                            as u8),
                                    );
                                    ((((((&raw mut lman).cast::<u8>()).wrapping_add(48))
                                        .wrapping_add(4))
                                    .cast::<u16>())
                                    .wrapping_offset(((i) as i32) as isize))
                                    .write(
                                        ((((&raw mut lman).cast::<u8>()).wrapping_add(48))
                                            .wrapping_add(2)
                                            .cast::<u16>())
                                        .read(),
                                    );
                                }
                            }
                            i = (i).wrapping_add(1);
                        }
                    }
                    rfu_LMAN_occureCallback(49u8, 1u8);
                } else {
                    (((&raw mut lman).cast::<u8>()).wrapping_add(10)).write(0u8);
                    rfu_LMAN_disconnect(bm_linkLossSlot);
                    disconnect_occure_flag = 1u8;
                    rfu_LMAN_occureCallback(51u8, 1u8);
                }
            } else {
                rfu_LMAN_disconnect(bm_linkLossSlot);
                disconnect_occure_flag = 1u8;
                rfu_LMAN_occureCallback(48u8, 2u8);
            }
            rfu_LMAN_managerChangeAgbClockMaster();
        }
        if (((((&raw mut gRfuLinkStatus).cast::<*mut u8>()).read()).read()) as i32) == 1i32 {
            if (bm_linkRecoverySlot) != 0 {
                {
                    i = 0u8;
                    'l3: loop {
                        if !(((i) as i32) < 4i32) {
                            break 'l3;
                        }
                        'l4: {
                            if ((crate::c::shr_i32(
                                (((((&raw mut lman).cast::<u8>()).wrapping_add(48)).read()) as i32),
                                ((i) as u32),
                            ) & 1i32)
                                != 0)
                                && ((crate::c::shr_i32(
                                    ((bm_linkRecoverySlot) as i32),
                                    ((i) as u32),
                                ) & 1i32)
                                    != 0)
                            {
                                ((((((&raw mut lman).cast::<u8>()).wrapping_add(48))
                                    .wrapping_add(4))
                                .cast::<u16>())
                                .wrapping_offset(((i) as i32) as isize))
                                .write(0u16);
                            }
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                let __p2 = (((&raw mut lman).cast::<u8>()).wrapping_add(48));
                (__p2).write((((((__p2).read()) as i32) & !((bm_linkRecoverySlot) as i32)) as u8));
                ((((&raw mut lman).cast::<u8>()).wrapping_add(20)).cast::<u16>())
                    .write(((bm_linkRecoverySlot) as u16));
                rfu_LMAN_occureCallback(50u8, 1u8);
            }
            if ((((&raw mut lman).cast::<u8>()).wrapping_add(48)).read()) != 0 {
                bm_disconnectSlot = 0u8;
                {
                    i = 0u8;
                    'l5: loop {
                        if !(((i) as i32) < 4i32) {
                            break 'l5;
                        }
                        'l6: {
                            if (((crate::c::shr_i32(
                                (((((&raw mut lman).cast::<u8>()).wrapping_add(48)).read()) as i32),
                                ((i) as u32),
                            ) & 1i32)
                                != 0)
                                && ((((((((&raw mut lman).cast::<u8>()).wrapping_add(48))
                                    .wrapping_add(4))
                                .cast::<u16>())
                                .wrapping_offset(((i) as i32) as isize))
                                .read())
                                    != 0))
                                && ((({
                                    let __p3 = (((((&raw mut lman).cast::<u8>())
                                        .wrapping_add(48))
                                    .wrapping_add(4))
                                    .cast::<u16>())
                                    .wrapping_offset(((i) as i32) as isize);
                                    let __t4 = ((__p3).read()).wrapping_sub(1);
                                    (__p3).write(__t4);
                                    __t4
                                }) as i32)
                                    == 0i32)
                            {
                                let __p5 = (((&raw mut lman).cast::<u8>()).wrapping_add(48));
                                (__p5).write(
                                    (((((__p5).read()) as i32)
                                        & !(crate::c::shl_i32(1i32, ((i) as u32))))
                                        as u8),
                                );
                                bm_disconnectSlot = ((((bm_disconnectSlot) as i32)
                                    | crate::c::shl_i32(1i32, ((i) as u32)))
                                    as u8);
                            }
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                if (bm_disconnectSlot) != 0 {
                    rfu_LMAN_disconnect(bm_disconnectSlot);
                    disconnect_occure_flag = 1u8;
                    ((((&raw mut lman).cast::<u8>()).wrapping_add(20)).cast::<u16>())
                        .write(((bm_disconnectSlot) as u16));
                    rfu_LMAN_occureCallback(51u8, 1u8);
                }
            }
            if !(((((&raw mut lman).cast::<u8>()).wrapping_add(48)).read()) != 0) {
                (((&raw mut lman).cast::<u8>()).wrapping_add(10)).write(0u8);
            }
        }
        return disconnect_occure_flag;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rfu_LMAN_syncVBlank() {
    unsafe {
        if (rfu_syncVBlank()) != 0 {
            rfu_LMAN_occureCallback(241u8, 0u8);
            rfu_LMAN_managerChangeAgbClockMaster();
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rfu_LMAN_manager_entity(rand: u32) {
    unsafe {
        let mut rand = rand;
        let mut msg: u8 = 0u8;
        if (core::mem::transmute::<_, usize>(
            (((&raw mut lman).cast::<u8>())
                .wrapping_add(64)
                .cast::<Option<unsafe extern "C" fn(u8, u8)>>())
            .read(),
        ) == 0usize)
            && ((((((&raw mut lman).cast::<u8>()).wrapping_add(4)).read()) as i32) != 0i32)
        {
            (((&raw mut lman).cast::<u8>()).wrapping_add(4)).write(0u8);
            return;
        }
        if ((((&raw mut lman).cast::<u8>()).wrapping_add(7)).read()) != 0 {
            rfu_LMAN_settingPCSWITCH(rand);
        }
        'l1: loop {
            if !((1i32) != 0) {
                break 'l1;
            }
            if (((((&raw mut lman).cast::<u8>()).wrapping_add(4)).read()) as i32) != 0i32 {
                rfu_waitREQComplete();
                (((&raw mut lman).cast::<u8>()).wrapping_add(14)).write(1u8);
                'l2: {
                    let __sw1 = (((((&raw mut lman).cast::<u8>()).wrapping_add(4)).read()) as i32);
                    let __matched = __sw1 == 23i32
                        || __sw1 == 1i32
                        || __sw1 == 2i32
                        || __sw1 == 3i32
                        || __sw1 == 4i32
                        || __sw1 == 5i32
                        || __sw1 == 6i32
                        || __sw1 == 7i32
                        || __sw1 == 8i32
                        || __sw1 == 9i32
                        || __sw1 == 10i32
                        || __sw1 == 11i32
                        || __sw1 == 12i32
                        || __sw1 == 13i32
                        || __sw1 == 14i32
                        || __sw1 == 15i32
                        || __sw1 == 16i32
                        || __sw1 == 17i32
                        || __sw1 == 18i32
                        || __sw1 == 19i32
                        || __sw1 == 20i32
                        || __sw1 == 21i32
                        || __sw1 == 22i32;
                    if __sw1 == 23i32 {
                        if rfu_LMAN_REQBN_softReset_and_checkID() == 32769u32 {
                            msg = 68u8;
                        } else {
                            msg = 255u8;
                        }
                        (((&raw mut lman).cast::<u8>()).wrapping_add(4)).write({
                            let __v2 = 0u8;
                            (((&raw mut lman).cast::<u8>()).wrapping_add(5)).write(__v2);
                            __v2
                        });
                        rfu_LMAN_occureCallback(msg, 0u8);
                        break 'l2;
                    }
                    if __sw1 == 1i32 {
                        if rfu_LMAN_REQBN_softReset_and_checkID() == 32769u32 {
                            (((&raw mut lman).cast::<u8>()).wrapping_add(4))
                                .write((((&raw mut lman).cast::<u8>()).wrapping_add(5)).read());
                            (((&raw mut lman).cast::<u8>()).wrapping_add(5)).write(3u8);
                        } else {
                            (((&raw mut lman).cast::<u8>()).wrapping_add(4)).write({
                                let __v3 = 0u8;
                                (((&raw mut lman).cast::<u8>()).wrapping_add(5)).write(__v3);
                                __v3
                            });
                            rfu_LMAN_occureCallback(255u8, 0u8);
                        }
                        break 'l2;
                    }
                    if __sw1 == 2i32 {
                        rfu_REQ_reset();
                        break 'l2;
                    }
                    if __sw1 == 3i32 {
                        rfu_REQ_configSystem(
                            (((((&raw mut lman).cast::<u8>())
                                .wrapping_add(60)
                                .cast::<*mut u8>())
                            .read())
                            .wrapping_add(2)
                            .cast::<u16>())
                            .read(),
                            ((((&raw mut lman).cast::<u8>())
                                .wrapping_add(60)
                                .cast::<*mut u8>())
                            .read())
                            .read(),
                            (((((&raw mut lman).cast::<u8>())
                                .wrapping_add(60)
                                .cast::<*mut u8>())
                            .read())
                            .wrapping_add(1))
                            .read(),
                        );
                        break 'l2;
                    }
                    if __sw1 == 4i32 {
                        rfu_REQ_configGameData(
                            (((((&raw mut lman).cast::<u8>())
                                .wrapping_add(60)
                                .cast::<*mut u8>())
                            .read())
                            .wrapping_add(4))
                            .read(),
                            (((((&raw mut lman).cast::<u8>())
                                .wrapping_add(60)
                                .cast::<*mut u8>())
                            .read())
                            .wrapping_add(6)
                            .cast::<u16>())
                            .read(),
                            (((((&raw mut lman).cast::<u8>())
                                .wrapping_add(60)
                                .cast::<*mut u8>())
                            .read())
                            .wrapping_add(8)
                            .cast::<*mut u8>())
                            .read(),
                            (((((&raw mut lman).cast::<u8>())
                                .wrapping_add(60)
                                .cast::<*mut u8>())
                            .read())
                            .wrapping_add(12)
                            .cast::<*mut u8>())
                            .read(),
                        );
                        break 'l2;
                    }
                    if __sw1 == 5i32 {
                        rfu_REQ_startSearchChild();
                        break 'l2;
                    }
                    if __sw1 == 6i32 {
                        rfu_REQ_pollSearchChild();
                        break 'l2;
                    }
                    if __sw1 == 7i32 {
                        rfu_REQ_endSearchChild();
                        break 'l2;
                    }
                    if __sw1 == 8i32 {
                        break 'l2;
                    }
                    if __sw1 == 9i32 {
                        rfu_REQ_startSearchParent();
                        break 'l2;
                    }
                    if __sw1 == 10i32 {
                        rfu_REQ_pollSearchParent();
                        break 'l2;
                    }
                    if __sw1 == 11i32 {
                        rfu_REQ_endSearchParent();
                        break 'l2;
                    }
                    if __sw1 == 12i32 {
                        rfu_REQ_startConnectParent(
                            (((&raw mut lman).cast::<u8>())
                                .wrapping_add(30)
                                .cast::<u16>())
                            .read(),
                        );
                        break 'l2;
                    }
                    if __sw1 == 13i32 {
                        rfu_REQ_pollConnectParent();
                        break 'l2;
                    }
                    if __sw1 == 14i32 {
                        rfu_REQ_endConnectParent();
                        break 'l2;
                    }
                    if __sw1 == 15i32 {
                        break 'l2;
                    }
                    if __sw1 == 16i32 {
                        rfu_REQ_CHILD_startConnectRecovery(
                            ((((&raw mut gRfuLinkStatus).cast::<*mut u8>()).read())
                                .wrapping_add(3))
                            .read(),
                        );
                        break 'l2;
                    }
                    if __sw1 == 17i32 {
                        rfu_REQ_CHILD_pollConnectRecovery();
                        break 'l2;
                    }
                    if __sw1 == 18i32 {
                        rfu_REQ_CHILD_endConnectRecovery();
                        break 'l2;
                    }
                    if __sw1 == 19i32 {
                        rfu_REQ_changeMasterSlave();
                        break 'l2;
                    }
                    if __sw1 == 20i32 {
                        break 'l2;
                    }
                    if __sw1 == 21i32 {
                        rfu_REQ_stopMode();
                        break 'l2;
                    }
                    if __sw1 == 22i32 {
                        break 'l2;
                    }
                    if !__matched {
                        break 'l2;
                    }
                }
                rfu_waitREQComplete();
                (((&raw mut lman).cast::<u8>()).wrapping_add(14)).write(0u8);
            }
            if ((((((&raw mut lman).cast::<u8>()).wrapping_add(4)).read()) as i32) == 18i32)
                || ((((((&raw mut lman).cast::<u8>()).wrapping_add(4)).read()) as i32) == 19i32)
            {
            } else {
                break 'l1;
            }
        }
        if (((((&raw mut gRfuLinkStatus).cast::<*mut u8>()).read()).read()) as i32) == 1i32 {
            if (rfu_LMAN_linkWatcher(0u16)) != 0 {
                return;
            }
        }
        rfu_LMAN_PARENT_checkRecvChildName();
        rfu_LMAN_CHILD_checkSendChildName();
        rfu_LMAN_CHILD_linkRecoveryProcess();
        rfu_LMAN_checkNICommunicateStatus();
    }
}
pub(crate) unsafe extern "C" fn rfu_LMAN_settingPCSWITCH(rand: u32) {
    unsafe {
        let mut rand = rand;
        if (((((&raw mut lman).cast::<u8>()).wrapping_add(7)).read()) as i32) == 5i32 {
            (((&raw mut lman).cast::<u8>()).wrapping_add(6)).write(1u8);
            (((&raw mut lman).cast::<u8>()).wrapping_add(4)).write(5u8);
            (((&raw mut lman).cast::<u8>())
                .wrapping_add(26)
                .cast::<u16>())
            .write(
                (((&raw mut lman).cast::<u8>())
                    .wrapping_add(28)
                    .cast::<u16>())
                .read(),
            );
            if ((((&raw mut lman).cast::<u8>())
                .wrapping_add(26)
                .cast::<u16>())
            .read())
                != 0
            {
                (((&raw mut lman).cast::<u8>()).wrapping_add(7)).write(6u8);
            } else {
                (((&raw mut lman).cast::<u8>()).wrapping_add(7)).write(1u8);
            }
        }
        if (((((&raw mut lman).cast::<u8>()).wrapping_add(7)).read()) as i32) == 1i32 {
            (((&raw mut lman).cast::<u8>()).wrapping_add(6)).write(1u8);
            (((&raw mut lman).cast::<u8>()).wrapping_add(4)).write(5u8);
            (((&raw mut lman).cast::<u8>())
                .wrapping_add(26)
                .cast::<u16>())
            .write(((crate::c::rem_u32(rand, 140u32)) as u16));
            (((&raw mut lman).cast::<u8>())
                .wrapping_add(28)
                .cast::<u16>())
            .write(
                (((140i32).wrapping_sub(
                    (((((&raw mut lman).cast::<u8>())
                        .wrapping_add(26)
                        .cast::<u16>())
                    .read()) as i32),
                )) as u16),
            );
            if ((((&raw mut lman).cast::<u8>())
                .wrapping_add(26)
                .cast::<u16>())
            .read())
                != 0
            {
                (((&raw mut lman).cast::<u8>()).wrapping_add(7)).write(2u8);
            } else {
                (((&raw mut lman).cast::<u8>()).wrapping_add(7)).write(3u8);
            }
        }
        if (((((&raw mut lman).cast::<u8>()).wrapping_add(7)).read()) as i32) == 3i32 {
            (((&raw mut lman).cast::<u8>()).wrapping_add(6)).write(0u8);
            (((&raw mut lman).cast::<u8>())
                .wrapping_add(26)
                .cast::<u16>())
            .write(40u16);
            (((&raw mut lman).cast::<u8>()).wrapping_add(7)).write(4u8);
            (((&raw mut lman).cast::<u8>()).wrapping_add(4)).write(9u8);
        }
    }
}
pub(crate) unsafe extern "C" fn rfu_LMAN_REQ_callback(reqCommandId: u16, reqResult: u16) {
    unsafe {
        let mut reqCommandId = reqCommandId;
        let mut reqResult = reqResult;
        let mut status: u8 = 0u8;
        let mut stwiRecvBuffer: *mut u8 = core::ptr::null_mut();
        let mut i: u8 = 0u8;
        if (((((&raw mut lman).cast::<u8>()).wrapping_add(14)).read()) as i32) != 0i32 {
            (((&raw mut lman).cast::<u8>()).wrapping_add(14)).write(0u8);
            'l1: {
                let __sw1 = ((reqCommandId) as i32);
                if __sw1 == 16i32 {
                    if ((reqResult) as i32) == 0i32 {
                        (((&raw mut lman).cast::<u8>()).wrapping_add(4))
                            .write((((&raw mut lman).cast::<u8>()).wrapping_add(5)).read());
                        (((&raw mut lman).cast::<u8>()).wrapping_add(5)).write(4u8);
                    }
                    break 'l1;
                }
                if __sw1 == 23i32 {
                    if ((reqResult) as i32) == 0i32 {
                        (((&raw mut lman).cast::<u8>()).wrapping_add(4))
                            .write((((&raw mut lman).cast::<u8>()).wrapping_add(5)).read());
                        (((&raw mut lman).cast::<u8>()).wrapping_add(5)).write(0u8);
                    }
                    break 'l1;
                }
                if __sw1 == 22i32 {
                    if ((reqResult) as i32) == 0i32 {
                        (((&raw mut lman).cast::<u8>()).wrapping_add(4)).write({
                            let __v2 = 0u8;
                            (((&raw mut lman).cast::<u8>()).wrapping_add(5)).write(__v2);
                            __v2
                        });
                        rfu_LMAN_occureCallback(0u8, 0u8);
                    }
                    break 'l1;
                }
                if __sw1 == 25i32 {
                    if ((reqResult) as i32) == 0i32 {
                        (((&raw mut lman).cast::<u8>()).wrapping_add(4)).write({
                            let __v3 = 6u8;
                            (((&raw mut lman).cast::<u8>()).wrapping_add(5)).write(__v3);
                            __v3
                        });
                    }
                    break 'l1;
                }
                if __sw1 == 26i32 {
                    if (((((&raw mut lman).cast::<u8>())
                        .wrapping_add(26)
                        .cast::<u16>())
                    .read())
                        != 0)
                        && ((({
                            let __p4 = ((&raw mut lman).cast::<u8>())
                                .wrapping_add(26)
                                .cast::<u16>();
                            let __t5 = ((__p4).read()).wrapping_sub(1);
                            (__p4).write(__t5);
                            __t5
                        }) as i32)
                            == 0i32)
                    {
                        (((&raw mut lman).cast::<u8>()).wrapping_add(4)).write(7u8);
                        (((&raw mut lman).cast::<u8>()).wrapping_add(5)).write(8u8);
                    }
                    break 'l1;
                }
                if __sw1 == 27i32 {
                    if ((reqResult) as i32) == 0i32 {
                        (((&raw mut lman).cast::<u8>()).wrapping_add(4))
                            .write((((&raw mut lman).cast::<u8>()).wrapping_add(5)).read());
                        (((&raw mut lman).cast::<u8>()).wrapping_add(5)).write(0u8);
                        if (((((&raw mut lman).cast::<u8>()).wrapping_add(7)).read()) as i32)
                            == 0i32
                        {
                            rfu_LMAN_occureCallback(19u8, 0u8);
                        }
                    }
                    break 'l1;
                }
                if __sw1 == 28i32 {
                    if ((reqResult) as i32) == 0i32 {
                        if (((((&raw mut lman).cast::<u8>()).wrapping_add(11)).read()) as i32)
                            == 1i32
                        {
                            if (((((&raw mut lman).cast::<u8>())
                                .wrapping_add(26)
                                .cast::<u16>())
                            .read()) as i32)
                                > 1i32
                            {
                                let __p6 = ((&raw mut lman).cast::<u8>())
                                    .wrapping_add(26)
                                    .cast::<u16>();
                                (__p6).write(((__p6).read()).wrapping_sub(1));
                            }
                        }
                        (((&raw mut lman).cast::<u8>()).wrapping_add(4)).write({
                            let __v7 = 10u8;
                            (((&raw mut lman).cast::<u8>()).wrapping_add(5)).write(__v7);
                            __v7
                        });
                    }
                    break 'l1;
                }
                if __sw1 == 29i32 {
                    if ((reqResult) as i32) == 0i32 {
                        status = rfu_LMAN_CHILD_checkEnableParentCandidate();
                        ((((&raw mut lman).cast::<u8>()).wrapping_add(20)).cast::<u16>())
                            .write(((status) as u16));
                        if (status) != 0 {
                            rfu_LMAN_occureCallback(32u8, 1u8);
                        }
                        if ((((((&raw mut lman).cast::<u8>()).wrapping_add(11)).read()) != 0)
                            && ((((((&raw mut lman).cast::<u8>())
                                .wrapping_add(26)
                                .cast::<u16>())
                            .read()) as i32)
                                != 1i32))
                            && (((((((&raw mut gRfuLinkStatus).cast::<*mut u8>()).read())
                                .wrapping_add(8))
                            .read()) as i32)
                                == 4i32)
                        {
                            rfu_REQ_endSearchParent();
                            rfu_waitREQComplete();
                            (((&raw mut lman).cast::<u8>()).wrapping_add(4)).write(9u8);
                            (((&raw mut lman).cast::<u8>()).wrapping_add(11)).write(1u8);
                        }
                    }
                    if (((((&raw mut lman).cast::<u8>())
                        .wrapping_add(26)
                        .cast::<u16>())
                    .read())
                        != 0)
                        && ((({
                            let __p8 = ((&raw mut lman).cast::<u8>())
                                .wrapping_add(26)
                                .cast::<u16>();
                            let __t9 = ((__p8).read()).wrapping_sub(1);
                            (__p8).write(__t9);
                            __t9
                        }) as i32)
                            == 0i32)
                    {
                        (((&raw mut lman).cast::<u8>()).wrapping_add(4)).write(11u8);
                        (((&raw mut lman).cast::<u8>()).wrapping_add(5)).write(0u8);
                    }
                    break 'l1;
                }
                if __sw1 == 30i32 {
                    if ((reqResult) as i32) == 0i32 {
                        (((&raw mut lman).cast::<u8>()).wrapping_add(4))
                            .write((((&raw mut lman).cast::<u8>()).wrapping_add(5)).read());
                        if (((((&raw mut lman).cast::<u8>()).wrapping_add(7)).read()) as i32)
                            == 0i32
                        {
                            if (((((&raw mut lman).cast::<u8>()).wrapping_add(4)).read()) as i32)
                                == 0i32
                            {
                                rfu_LMAN_occureCallback(33u8, 0u8);
                            }
                        } else {
                            if (((((&raw mut lman).cast::<u8>()).wrapping_add(7)).read()) as i32)
                                != 7i32
                            {
                                (((&raw mut lman).cast::<u8>()).wrapping_add(4)).write(5u8);
                                (((&raw mut lman).cast::<u8>()).wrapping_add(7)).write(5u8);
                            }
                        }
                    }
                    break 'l1;
                }
                if __sw1 == 31i32 {
                    if ((reqResult) as i32) == 0i32 {
                        (((&raw mut lman).cast::<u8>()).wrapping_add(4)).write({
                            let __v10 = 13u8;
                            (((&raw mut lman).cast::<u8>()).wrapping_add(5)).write(__v10);
                            __v10
                        });
                    }
                    break 'l1;
                }
                if __sw1 == 32i32 {
                    if ((((reqResult) as i32) == 0i32)
                        && (!((rfu_getConnectParentStatus(
                            &raw mut status,
                            ((&raw mut lman).cast::<u8>()).wrapping_add(16),
                        )) != 0)))
                        && (!((status) != 0))
                    {
                        (((&raw mut lman).cast::<u8>()).wrapping_add(4)).write(14u8);
                    }
                    if (((((&raw mut lman).cast::<u8>())
                        .wrapping_add(26)
                        .cast::<u16>())
                    .read())
                        != 0)
                        && ((({
                            let __p11 = ((&raw mut lman).cast::<u8>())
                                .wrapping_add(26)
                                .cast::<u16>();
                            let __t12 = ((__p11).read()).wrapping_sub(1);
                            (__p11).write(__t12);
                            __t12
                        }) as i32)
                            == 0i32)
                    {
                        (((&raw mut lman).cast::<u8>()).wrapping_add(4)).write(14u8);
                    }
                    break 'l1;
                }
                if __sw1 == 33i32 {
                    if (((reqResult) as i32) == 0i32)
                        && (!((rfu_getConnectParentStatus(
                            &raw mut status,
                            ((&raw mut lman).cast::<u8>()).wrapping_add(16),
                        )) != 0))
                    {
                        if !((status) != 0) {
                            (((&raw mut lman).cast::<u8>()).wrapping_add(4)).write(19u8);
                            (((&raw mut lman).cast::<u8>()).wrapping_add(5)).write(15u8);
                            (((&raw mut lman).cast::<u8>())
                                .wrapping_add(30)
                                .cast::<u16>())
                            .write(34u16);
                            ((((&raw mut lman).cast::<u8>()).wrapping_add(20)).cast::<u16>())
                                .write(
                                    (((((&raw mut lman).cast::<u8>()).wrapping_add(16)).read())
                                        as u16),
                                );
                        } else {
                            (((&raw mut lman).cast::<u8>()).wrapping_add(4)).write({
                                let __v13 = 0u8;
                                (((&raw mut lman).cast::<u8>()).wrapping_add(5)).write(__v13);
                                __v13
                            });
                            (((&raw mut lman).cast::<u8>())
                                .wrapping_add(30)
                                .cast::<u16>())
                            .write(35u16);
                            ((((&raw mut lman).cast::<u8>()).wrapping_add(20)).cast::<u16>())
                                .write(((status) as u16));
                            if ((((&raw mut lman).cast::<u8>()).wrapping_add(7)).read()) != 0 {
                                (((&raw mut lman).cast::<u8>()).wrapping_add(7)).write(3u8);
                                (((&raw mut lman).cast::<u8>()).wrapping_add(4)).write(9u8);
                            }
                        }
                        rfu_LMAN_occureCallback(
                            (((((&raw mut lman).cast::<u8>())
                                .wrapping_add(30)
                                .cast::<u16>())
                            .read()) as u8),
                            1u8,
                        );
                        (((&raw mut lman).cast::<u8>())
                            .wrapping_add(30)
                            .cast::<u16>())
                        .write(0u16);
                    }
                    break 'l1;
                }
                if __sw1 == 50i32 {
                    if ((reqResult) as i32) == 0i32 {
                        ((((&raw mut lman).cast::<u8>()).wrapping_add(20)).cast::<u16>()).write(
                            ((((((&raw mut gRfuLinkStatus).cast::<*mut u8>()).read())
                                .wrapping_add(3))
                            .read()) as u16),
                        );
                        (((&raw mut lman).cast::<u8>()).wrapping_add(4)).write({
                            let __v14 = 17u8;
                            (((&raw mut lman).cast::<u8>()).wrapping_add(5)).write(__v14);
                            __v14
                        });
                        {
                            (((&raw mut lman).cast::<u8>()).wrapping_add(16)).write(0u8);
                            'l2: loop {
                                if !((((((&raw mut lman).cast::<u8>()).wrapping_add(16)).read())
                                    as i32)
                                    < 4i32)
                                {
                                    break 'l2;
                                }
                                'l3: {
                                    if (crate::c::shr_i32(
                                        ((((((&raw mut gRfuLinkStatus).cast::<*mut u8>()).read())
                                            .wrapping_add(3))
                                        .read()) as i32),
                                        (((((&raw mut lman).cast::<u8>()).wrapping_add(16)).read())
                                            as u32),
                                    ) & 1i32)
                                        != 0
                                    {
                                        break 'l2;
                                    }
                                }
                                let __p15 = ((&raw mut lman).cast::<u8>()).wrapping_add(16);
                                (__p15).write(((__p15).read()).wrapping_add(1));
                            }
                        }
                    }
                    break 'l1;
                }
                if __sw1 == 51i32 {
                    if ((((reqResult) as i32) == 0i32)
                        && (!((rfu_CHILD_getConnectRecoveryStatus(&raw mut status)) != 0)))
                        && (((status) as i32) < 2i32)
                    {
                        (((&raw mut lman).cast::<u8>()).wrapping_add(4)).write(18u8);
                    }
                    if ((((((((&raw mut lman).cast::<u8>()).wrapping_add(48)).wrapping_add(4))
                        .cast::<u16>())
                    .wrapping_offset(
                        (((((&raw mut lman).cast::<u8>()).wrapping_add(16)).read()) as i32)
                            as isize,
                    ))
                    .read())
                        != 0)
                        && ((({
                            let __p16 = (((((&raw mut lman).cast::<u8>()).wrapping_add(48))
                                .wrapping_add(4))
                            .cast::<u16>())
                            .wrapping_offset(
                                (((((&raw mut lman).cast::<u8>()).wrapping_add(16)).read()) as i32)
                                    as isize,
                            );
                            let __t17 = ((__p16).read()).wrapping_sub(1);
                            (__p16).write(__t17);
                            __t17
                        }) as i32)
                            == 0i32)
                    {
                        (((&raw mut lman).cast::<u8>()).wrapping_add(4)).write(18u8);
                    }
                    break 'l1;
                }
                if __sw1 == 52i32 {
                    if (((reqResult) as i32) == 0i32)
                        && (!((rfu_CHILD_getConnectRecoveryStatus(&raw mut status)) != 0))
                    {
                        if !((status) != 0) {
                            (((&raw mut lman).cast::<u8>()).wrapping_add(4)).write(19u8);
                            (((&raw mut lman).cast::<u8>()).wrapping_add(5)).write(22u8);
                            (((&raw mut lman).cast::<u8>())
                                .wrapping_add(30)
                                .cast::<u16>())
                            .write(50u16);
                        } else {
                            (((&raw mut lman).cast::<u8>()).wrapping_add(4)).write({
                                let __v18 = 0u8;
                                (((&raw mut lman).cast::<u8>()).wrapping_add(5)).write(__v18);
                                __v18
                            });
                            rfu_LMAN_disconnect(
                                ((((&raw mut gRfuLinkStatus).cast::<*mut u8>()).read())
                                    .wrapping_add(3))
                                .read(),
                            );
                            (((&raw mut lman).cast::<u8>())
                                .wrapping_add(30)
                                .cast::<u16>())
                            .write(51u16);
                        }
                        ((((((&raw mut lman).cast::<u8>()).wrapping_add(48)).wrapping_add(4))
                            .cast::<u16>())
                        .wrapping_offset(
                            (((((&raw mut lman).cast::<u8>()).wrapping_add(16)).read()) as i32)
                                as isize,
                        ))
                        .write(0u16);
                        (((&raw mut lman).cast::<u8>()).wrapping_add(48)).write(0u8);
                        (((&raw mut lman).cast::<u8>()).wrapping_add(10)).write(0u8);
                        rfu_LMAN_occureCallback(
                            (((((&raw mut lman).cast::<u8>())
                                .wrapping_add(30)
                                .cast::<u16>())
                            .read()) as u8),
                            1u8,
                        );
                        (((&raw mut lman).cast::<u8>())
                            .wrapping_add(30)
                            .cast::<u16>())
                        .write(0u16);
                    }
                    break 'l1;
                }
                if __sw1 == 39i32 {
                    if ((reqResult) as i32) == 0i32 {
                        if (((((&raw mut lman).cast::<u8>()).wrapping_add(5)).read()) as i32)
                            == 22i32
                        {
                            (((&raw mut lman).cast::<u8>()).wrapping_add(4)).write(
                                ((((&raw mut lman).cast::<u8>()).wrapping_add(17)).cast::<u8>())
                                    .read(),
                            );
                            (((&raw mut lman).cast::<u8>()).wrapping_add(5)).write(
                                (((((&raw mut lman).cast::<u8>()).wrapping_add(17)).cast::<u8>())
                                    .wrapping_offset(1))
                                .read(),
                            );
                            crate::c::volatile_write(
                                ((&raw mut lman).cast::<u8>()).wrapping_add(2),
                                1u8,
                            );
                            rfu_LMAN_occureCallback(65u8, 0u8);
                        } else {
                            if (((((&raw mut lman).cast::<u8>()).wrapping_add(5)).read()) as i32)
                                == 15i32
                            {
                                (((&raw mut lman).cast::<u8>()).wrapping_add(4))
                                    .write((((&raw mut lman).cast::<u8>()).wrapping_add(5)).read());
                                crate::c::volatile_write(
                                    ((&raw mut lman).cast::<u8>()).wrapping_add(2),
                                    1u8,
                                );
                                rfu_LMAN_occureCallback(65u8, 0u8);
                                let __p19 = (((&raw mut lman).cast::<u8>()).wrapping_add(36));
                                (__p19).write(
                                    (((((__p19).read()) as i32)
                                        | crate::c::shl_i32(
                                            1i32,
                                            (((((&raw mut lman).cast::<u8>()).wrapping_add(16))
                                                .read())
                                                as u32),
                                        )) as u8),
                                );
                                ((((((&raw mut lman).cast::<u8>()).wrapping_add(36))
                                    .wrapping_add(4))
                                .cast::<u16>())
                                .wrapping_offset(
                                    (((((&raw mut lman).cast::<u8>()).wrapping_add(16)).read())
                                        as i32) as isize,
                                ))
                                .write(
                                    ((((&raw mut lman).cast::<u8>()).wrapping_add(36))
                                        .wrapping_add(2)
                                        .cast::<u16>())
                                    .read(),
                                );
                                rfu_clearSlot(
                                    4u8,
                                    (((&raw mut lman).cast::<u8>()).wrapping_add(16)).read(),
                                );
                                status = ((rfu_NI_CHILD_setSendGameName(
                                    (((&raw mut lman).cast::<u8>()).wrapping_add(16)).read(),
                                    14u8,
                                )) as u8);
                                if (status) != 0 {
                                    (((&raw mut lman).cast::<u8>()).wrapping_add(4)).write({
                                        let __v20 = 0u8;
                                        (((&raw mut lman).cast::<u8>()).wrapping_add(5))
                                            .write(__v20);
                                        __v20
                                    });
                                    rfu_LMAN_managerChangeAgbClockMaster();
                                    rfu_LMAN_disconnect(
                                        ((((((((&raw mut gRfuLinkStatus).cast::<*mut u8>())
                                            .read())
                                        .wrapping_add(2))
                                        .read()) as i32)
                                            | ((((((&raw mut gRfuLinkStatus).cast::<*mut u8>())
                                                .read())
                                            .wrapping_add(3))
                                            .read())
                                                as i32))
                                            as u8),
                                    );
                                    ((((&raw mut lman).cast::<u8>()).wrapping_add(20))
                                        .cast::<u16>())
                                    .write(((status) as u16));
                                    rfu_LMAN_occureCallback(37u8, 1u8);
                                }
                            }
                        }
                    }
                    break 'l1;
                }
                if __sw1 == 61i32 {
                    if ((reqResult) as i32) == 0i32 {
                        (((&raw mut lman).cast::<u8>()).wrapping_add(4)).write({
                            let __v21 = 0u8;
                            (((&raw mut lman).cast::<u8>()).wrapping_add(5)).write(__v21);
                            __v21
                        });
                        rfu_LMAN_occureCallback(66u8, 0u8);
                    }
                    break 'l1;
                }
            }
            (((&raw mut lman).cast::<u8>()).wrapping_add(14)).write(1u8);
        } else {
            if ((((reqResult) as i32) == 3i32)
                && (((((&raw mut lman).cast::<u8>()).wrapping_add(15)).read()) != 0))
                && (((((reqCommandId) as i32) == 36i32) || (((reqCommandId) as i32) == 38i32))
                    || (((reqCommandId) as i32) == 39i32))
            {
                rfu_REQ_RFUStatus();
                rfu_waitREQComplete();
                rfu_getRFUStatus(&raw mut status);
                if (((status) as i32) == 0i32)
                    && ((((((&raw mut gRfuLinkStatus).cast::<*mut u8>()).read()).read()) as i32)
                        == 0i32)
                {
                    stwiRecvBuffer = (rfu_getSTWIRecvBuffer()).wrapping_offset(4);
                    ({
                        let __t22 = stwiRecvBuffer;
                        stwiRecvBuffer = (stwiRecvBuffer).wrapping_offset(1);
                        __t22
                    })
                    .write(
                        ((((&raw mut gRfuLinkStatus).cast::<*mut u8>()).read()).wrapping_add(2))
                            .read(),
                    );
                    (stwiRecvBuffer).write(1u8);
                    rfu_LMAN_linkWatcher(41u16);
                    reqResult = 0u16;
                }
            }
        }
        'l4: {
            let __sw23 = ((reqCommandId) as i32);
            if __sw23 == 48i32 {
                if ((reqResult) as i32) == 0i32 {
                    ((((&raw mut lman).cast::<u8>()).wrapping_add(20)).cast::<u16>())
                        .write(((((rfu_getSTWIRecvBuffer()).wrapping_offset(8)).read()) as u16));
                    rfu_LMAN_reflectCommunicationStatus(
                        ((((((&raw mut lman).cast::<u8>()).wrapping_add(20)).cast::<u16>()).read())
                            as u8),
                    );
                    if ((((&raw mut lman).cast::<u8>()).wrapping_add(48)).read()) != 0 {
                        let __p24 = (((&raw mut lman).cast::<u8>()).wrapping_add(48));
                        (__p24).write(
                            (((((__p24).read()) as i32)
                                & !((((((&raw mut lman).cast::<u8>()).wrapping_add(20))
                                    .cast::<u16>())
                                .read()) as i32)) as u8),
                        );
                        {
                            i = 0u8;
                            'l5: loop {
                                if !(((i) as i32) < 4i32) {
                                    break 'l5;
                                }
                                'l6: {
                                    if (crate::c::shr_i32(
                                        ((((((&raw mut lman).cast::<u8>()).wrapping_add(20))
                                            .cast::<u16>())
                                        .read()) as i32),
                                        ((i) as u32),
                                    ) & 1i32)
                                        != 0
                                    {
                                        ((((((&raw mut lman).cast::<u8>()).wrapping_add(48))
                                            .wrapping_add(4))
                                        .cast::<u16>())
                                        .wrapping_offset(((i) as i32) as isize))
                                        .write(0u16);
                                    }
                                }
                                i = (i).wrapping_add(1);
                            }
                        }
                        if (((((&raw mut lman).cast::<u8>()).wrapping_add(6)).read()) as i32)
                            == 0i32
                        {
                            (((&raw mut lman).cast::<u8>()).wrapping_add(4)).write({
                                let __v25 = 0u8;
                                (((&raw mut lman).cast::<u8>()).wrapping_add(5)).write(__v25);
                                __v25
                            });
                        }
                    }
                    status = ((((((&raw mut lman).cast::<u8>()).read()) as i32)
                        & ((((((&raw mut lman).cast::<u8>()).wrapping_add(20)).cast::<u16>())
                            .read()) as i32)) as u8);
                    {
                        i = 0u8;
                        'l7: loop {
                            if !(((i) as i32) < 4i32) {
                                break 'l7;
                            }
                            'l8: {
                                if ((crate::c::shr_i32(((status) as i32), ((i) as u32)) & 1i32)
                                    != 0)
                                    && (((((&raw mut lman).cast::<u8>()).wrapping_add(1)).read())
                                        != 0)
                                {
                                    let __p26 = ((&raw mut lman).cast::<u8>()).wrapping_add(1);
                                    (__p26).write(((__p26).read()).wrapping_sub(1));
                                }
                            }
                            i = (i).wrapping_add(1);
                        }
                    }
                    let __p27 = ((&raw mut lman).cast::<u8>());
                    (__p27).write(
                        (((((__p27).read()) as i32)
                            & !((((((&raw mut lman).cast::<u8>()).wrapping_add(20)).cast::<u16>())
                                .read()) as i32)) as u8),
                    );
                    if ((((&raw mut lman).cast::<u8>()).wrapping_add(7)).read()) != 0 {
                        if (((((&raw mut gRfuLinkStatus).cast::<*mut u8>()).read()).read()) as i32)
                            == 255i32
                        {
                            if (((((&raw mut lman).cast::<u8>()).wrapping_add(7)).read()) as i32)
                                == 8i32
                            {
                                (((&raw mut lman).cast::<u8>())
                                    .wrapping_add(26)
                                    .cast::<u16>())
                                .write(
                                    (((&raw mut lman).cast::<u8>())
                                        .wrapping_add(28)
                                        .cast::<u16>())
                                    .read(),
                                );
                                (((&raw mut lman).cast::<u8>()).wrapping_add(7)).write(6u8);
                                (((&raw mut lman).cast::<u8>()).wrapping_add(4)).write(6u8);
                            } else {
                                if ((((((&raw mut lman).cast::<u8>()).wrapping_add(4)).read())
                                    as i32)
                                    != 6i32)
                                    && ((((((&raw mut lman).cast::<u8>()).wrapping_add(4)).read())
                                        as i32)
                                        != 7i32)
                                {
                                    (((&raw mut lman).cast::<u8>()).wrapping_add(7)).write(1u8);
                                    (((&raw mut lman).cast::<u8>()).wrapping_add(4)).write(5u8);
                                }
                            }
                        }
                    }
                    if (((((&raw mut gRfuLinkStatus).cast::<*mut u8>()).read()).read()) as i32)
                        == 255i32
                    {
                        if (((((&raw mut lman).cast::<u8>()).wrapping_add(4)).read()) as i32)
                            == 0i32
                        {
                            (((&raw mut lman).cast::<u8>()).wrapping_add(6)).write(255u8);
                        }
                    }
                    if (((((&raw mut lman).cast::<u8>()).wrapping_add(14)).read()) as i32) == 0i32 {
                        rfu_LMAN_occureCallback(64u8, 1u8);
                    }
                }
                break 'l4;
            }
            if __sw23 == 38i32 {
                rfu_LMAN_CHILD_checkSendChildName2();
                if (((((&raw mut gRfuLinkStatus).cast::<*mut u8>()).read()).read()) as i32)
                    != 255i32
                {
                    rfu_LMAN_occureCallback(80u8, 0u8);
                }
                break 'l4;
            }
            if __sw23 == 16i32 || __sw23 == 61i32 {
                if ((reqResult) as i32) == 0i32 {
                    (((&raw mut lman).cast::<u8>()).wrapping_add(13)).write(0u8);
                    (((&raw mut lman).cast::<u8>()).wrapping_add(1)).write(0u8);
                    ((&raw mut lman).cast::<u8>()).write(0u8);
                    (((&raw mut lman).cast::<u8>()).wrapping_add(6)).write(255u8);
                    rfu_LMAN_managerChangeAgbClockMaster();
                    if ((reqCommandId) as i32) == 61i32 {
                        rfu_LMAN_endManager();
                    }
                }
                break 'l4;
            }
        }
        if ((reqResult) as i32) != 0i32 {
            if ((((reqCommandId) as i32) == 28i32) && (((reqResult) as i32) != 0i32))
                && ((((((&raw mut lman).cast::<u8>()).wrapping_add(7)).read()) as i32) == 4i32)
            {
                (((&raw mut gRfuLinkStatus).cast::<*mut u8>()).read()).write(1u8);
                ((((&raw mut gRfuLinkStatus).cast::<*mut u8>()).read()).wrapping_add(2))
                    .write(15u8);
                rfu_LMAN_disconnect(15u8);
                rfu_waitREQComplete();
                return;
            } else {
                ((((&raw mut lman).cast::<u8>()).wrapping_add(20)).cast::<u16>())
                    .write(reqCommandId);
                (((((&raw mut lman).cast::<u8>()).wrapping_add(20)).cast::<u16>())
                    .wrapping_offset(1))
                .write(reqResult);
                if ((((&raw mut lman).cast::<u8>()).wrapping_add(14)).read()) != 0 {
                    (((&raw mut lman).cast::<u8>()).wrapping_add(4)).write({
                        let __v28 = 0u8;
                        (((&raw mut lman).cast::<u8>()).wrapping_add(5)).write(__v28);
                        __v28
                    });
                }
                rfu_LMAN_occureCallback(240u8, 2u8);
                rfu_LMAN_managerChangeAgbClockMaster();
            }
        }
        if ((reqCommandId) as i32) == 255i32 {
            rfu_LMAN_occureCallback(242u8, 0u8);
            rfu_LMAN_managerChangeAgbClockMaster();
        }
    }
}
pub(crate) unsafe extern "C" fn rfu_LMAN_MSC_callback(reqCommandId: u16) {
    unsafe {
        let mut reqCommandId = reqCommandId;
        let mut active_bak: u8 = 0u8;
        let mut thisAck_flag: u8 = 0u8;
        active_bak = (((&raw mut lman).cast::<u8>()).wrapping_add(14)).read();
        (((&raw mut lman).cast::<u8>()).wrapping_add(14)).write(0u8);
        (((&raw mut lman).cast::<u8>()).wrapping_add(15)).write(1u8);
        if (((((&raw mut gRfuLinkStatus).cast::<*mut u8>()).read()).read()) as i32) == 0i32 {
            rfu_LMAN_linkWatcher(reqCommandId);
            if (((((&raw mut lman).cast::<u8>()).wrapping_add(2)).read_volatile()) as i32) != 1i32 {
                rfu_LMAN_managerChangeAgbClockMaster();
                (((&raw mut lman).cast::<u8>()).wrapping_add(15)).write(0u8);
                (((&raw mut lman).cast::<u8>()).wrapping_add(14)).write(active_bak);
                return;
            }
        } else {
            if !((rfu_UNI_PARENT_getDRAC_ACK(&raw mut thisAck_flag)) != 0) {
                let __p1 = ((&raw mut lman).cast::<u8>()).wrapping_add(3);
                crate::c::volatile_write(
                    __p1,
                    (((((__p1).read_volatile()) as i32) | ((thisAck_flag) as i32)) as u8),
                );
            }
        }
        if core::mem::transmute::<_, usize>(
            (((&raw mut lman).cast::<u8>())
                .wrapping_add(68)
                .cast::<Option<unsafe extern "C" fn(u16)>>())
            .read(),
        ) != 0usize
        {
            ((((&raw mut lman).cast::<u8>())
                .wrapping_add(68)
                .cast::<Option<unsafe extern "C" fn(u16)>>())
            .read())
            .unwrap_unchecked()(reqCommandId);
            rfu_waitREQComplete();
            if (((((&raw mut lman).cast::<u8>()).wrapping_add(2)).read_volatile()) as i32) == 2i32 {
                rfu_LMAN_managerChangeAgbClockMaster();
            }
        }
        (((&raw mut lman).cast::<u8>()).wrapping_add(15)).write(0u8);
        (((&raw mut lman).cast::<u8>()).wrapping_add(14)).write(active_bak);
    }
}
pub(crate) unsafe extern "C" fn rfu_LMAN_PARENT_checkRecvChildName() {
    unsafe {
        let mut newSlot: u8 = 0u8;
        let mut newAcceptSlot: u8 = 0u8;
        let mut i: u8 = 0u8;
        let mut flags: u8 = 0u8;
        let mut tgtSlot: u8 = 0u8;
        let mut ptr: *mut u16 = core::ptr::null_mut();
        if ((((((((&raw mut lman).cast::<u8>()).wrapping_add(4)).read()) as i32) == 5i32)
            || ((((((&raw mut lman).cast::<u8>()).wrapping_add(4)).read()) as i32) == 6i32))
            || ((((((&raw mut lman).cast::<u8>()).wrapping_add(4)).read()) as i32) == 7i32))
            || ((((((&raw mut lman).cast::<u8>()).wrapping_add(4)).read()) as i32) == 8i32)
        {
            newSlot = ((((((((((&raw mut gRfuLinkStatus).cast::<*mut u8>()).read())
                .wrapping_add(2))
            .read()) as i32)
                ^ (((((&raw mut lman).cast::<u8>()).wrapping_add(12)).read()) as i32))
                & ((((((&raw mut gRfuLinkStatus).cast::<*mut u8>()).read()).wrapping_add(2)).read())
                    as i32))
                & !((((((&raw mut gRfuLinkStatus).cast::<*mut u8>()).read()).wrapping_add(7))
                    .read()) as i32)) as u8);
            (((&raw mut lman).cast::<u8>()).wrapping_add(12)).write(
                ((((&raw mut gRfuLinkStatus).cast::<*mut u8>()).read()).wrapping_add(2)).read(),
            );
            if (newSlot) != 0 {
                ((((&raw mut lman).cast::<u8>()).wrapping_add(20)).cast::<u16>())
                    .write(((newSlot) as u16));
                rfu_LMAN_occureCallback(16u8, 1u8);
            }
            newAcceptSlot = 0u8;
            {
                i = 0u8;
                'l1: loop {
                    if !(((i) as i32) < 4i32) {
                        break 'l1;
                    }
                    'l2: {
                        tgtSlot = ((crate::c::shl_i32(1i32, ((i) as u32))) as u8);
                        flags = 0u8;
                        if (((newSlot) as i32) & ((tgtSlot) as i32)) != 0 {
                            ((((((&raw mut lman).cast::<u8>()).wrapping_add(36)).wrapping_add(4))
                                .cast::<u16>())
                            .wrapping_offset(((i) as i32) as isize))
                            .write(
                                ((((&raw mut lman).cast::<u8>()).wrapping_add(36))
                                    .wrapping_add(2)
                                    .cast::<u16>())
                                .read(),
                            );
                            let __p1 = (((&raw mut lman).cast::<u8>()).wrapping_add(36));
                            (__p1).write((((((__p1).read()) as i32) | ((tgtSlot) as i32)) as u8));
                        } else {
                            if ((((((&raw mut lman).cast::<u8>()).wrapping_add(36)).read()) as i32)
                                & ((tgtSlot) as i32))
                                != 0
                            {
                                if (((((((((&raw mut gRfuSlotStatusNI).cast::<*mut u8>())
                                    .cast::<*mut u8>())
                                .wrapping_offset(((i) as i32) as isize))
                                .read())
                                .wrapping_add(52))
                                .cast::<u16>())
                                .read()) as i32)
                                    == 70i32
                                {
                                    if (((((((((&raw mut gRfuSlotStatusNI).cast::<*mut u8>())
                                        .cast::<*mut u8>())
                                    .wrapping_offset(((i) as i32) as isize))
                                    .read())
                                    .wrapping_add(52))
                                    .wrapping_add(45))
                                    .read()) as i32)
                                        == 1i32
                                    {
                                        flags = 2u8;
                                        {
                                            ptr = (((&raw mut lman).cast::<u8>())
                                                .wrapping_add(32)
                                                .cast::<*mut u16>())
                                            .read();
                                            'l3: loop {
                                                if !((((ptr).read()) as i32) != 65535i32) {
                                                    break 'l3;
                                                }
                                                'l4: {
                                                    if (((((((((&raw mut gRfuLinkStatus)
                                                        .cast::<*mut u8>())
                                                    .read())
                                                    .wrapping_add(20))
                                                    .cast::<u8>())
                                                    .wrapping_offset(((i) as i32) as isize * 32))
                                                    .wrapping_add(4)
                                                    .cast::<u16>())
                                                    .read())
                                                        as i32)
                                                        == (((ptr).read()) as i32)
                                                    {
                                                        let __p2 = ((&raw mut lman).cast::<u8>());
                                                        (__p2).write(
                                                            (((((__p2).read()) as i32)
                                                                | ((tgtSlot) as i32))
                                                                as u8),
                                                        );
                                                        let __p3 = ((&raw mut lman).cast::<u8>())
                                                            .wrapping_add(1);
                                                        (__p3)
                                                            .write(((__p3).read()).wrapping_add(1));
                                                        newAcceptSlot = ((((newAcceptSlot) as i32)
                                                            | ((tgtSlot) as i32))
                                                            as u8);
                                                        flags = ((((flags) as i32) | 1i32) as u8);
                                                        break 'l3;
                                                    }
                                                }
                                                ptr = (ptr).wrapping_offset(1);
                                            }
                                        }
                                        if !((((flags) as i32) & 1i32) != 0) {
                                            flags = ((((flags) as i32) | 4i32) as u8);
                                        }
                                    }
                                } else {
                                    if (({
                                        let __p4 = (((((&raw mut lman).cast::<u8>())
                                            .wrapping_add(36))
                                        .wrapping_add(4))
                                        .cast::<u16>())
                                        .wrapping_offset(((i) as i32) as isize);
                                        let __t5 = ((__p4).read()).wrapping_sub(1);
                                        (__p4).write(__t5);
                                        __t5
                                    }) as i32)
                                        == 0i32
                                    {
                                        flags = 6u8;
                                    }
                                }
                                if (((flags) as i32) & 2i32) != 0 {
                                    let __p6 = (((&raw mut lman).cast::<u8>()).wrapping_add(36));
                                    (__p6).write(
                                        (((((__p6).read()) as i32) & !((tgtSlot) as i32)) as u8),
                                    );
                                    ((((((&raw mut lman).cast::<u8>()).wrapping_add(36))
                                        .wrapping_add(4))
                                    .cast::<u16>())
                                    .wrapping_offset(((i) as i32) as isize))
                                    .write(0u16);
                                    rfu_clearSlot(8u8, i);
                                }
                                if (((flags) as i32) & 4i32) != 0 {
                                    let __p7 = ((&raw mut lman).cast::<u8>()).wrapping_add(13);
                                    (__p7).write(
                                        (((((__p7).read()) as i32) | ((tgtSlot) as i32)) as u8),
                                    );
                                }
                            }
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
            if (newAcceptSlot) != 0 {
                ((((&raw mut lman).cast::<u8>()).wrapping_add(20)).cast::<u16>())
                    .write(((newAcceptSlot) as u16));
                rfu_LMAN_occureCallback(17u8, 1u8);
            }
            if ((((&raw mut lman).cast::<u8>()).wrapping_add(13)).read()) != 0 {
                flags = 1u8;
                if (((((&raw mut gRfuLinkStatus).cast::<*mut u8>()).read()).wrapping_add(6)).read())
                    != 0
                {
                    if ((((((&raw mut lman).cast::<u8>()).wrapping_add(3)).read_volatile()) as i32)
                        & ((((&raw mut lman).cast::<u8>()).read()) as i32))
                        != ((((&raw mut lman).cast::<u8>()).read()) as i32)
                    {
                        flags = 0u8;
                    }
                }
                if (flags) != 0 {
                    rfu_LMAN_disconnect((((&raw mut lman).cast::<u8>()).wrapping_add(13)).read());
                    ((((&raw mut lman).cast::<u8>()).wrapping_add(20)).cast::<u16>())
                        .write((((((&raw mut lman).cast::<u8>()).wrapping_add(13)).read()) as u16));
                    (((&raw mut lman).cast::<u8>()).wrapping_add(13)).write(0u8);
                    rfu_LMAN_occureCallback(18u8, 1u8);
                }
            }
            if ((((((&raw mut lman).cast::<u8>()).wrapping_add(36)).read()) as i32) == 0i32)
                && ((((((&raw mut lman).cast::<u8>()).wrapping_add(4)).read()) as i32) == 8i32)
            {
                if (((((&raw mut lman).cast::<u8>()).wrapping_add(7)).read()) as i32) == 0i32 {
                    (((&raw mut lman).cast::<u8>()).wrapping_add(4)).write({
                        let __v8 = 0u8;
                        (((&raw mut lman).cast::<u8>()).wrapping_add(5)).write(__v8);
                        __v8
                    });
                    rfu_LMAN_occureCallback(20u8, 0u8);
                } else {
                    if (((((&raw mut lman).cast::<u8>()).wrapping_add(7)).read()) as i32) == 2i32 {
                        (((&raw mut lman).cast::<u8>()).wrapping_add(7)).write(3u8);
                        (((&raw mut lman).cast::<u8>()).wrapping_add(4)).write(9u8);
                    } else {
                        (((&raw mut lman).cast::<u8>()).wrapping_add(7)).write(1u8);
                        (((&raw mut lman).cast::<u8>()).wrapping_add(4)).write(5u8);
                    }
                    if (((&raw mut lman).cast::<u8>()).read()) != 0 {
                        (((&raw mut lman).cast::<u8>())
                            .wrapping_add(26)
                            .cast::<u16>())
                        .write(0u16);
                        (((&raw mut lman).cast::<u8>()).wrapping_add(7)).write(8u8);
                        (((&raw mut lman).cast::<u8>()).wrapping_add(4)).write(5u8);
                    }
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn rfu_LMAN_CHILD_checkSendChildName() {
    unsafe {
        let mut imeBak: u16 = ((67109384i32) as usize as *mut u16).read_volatile();
        crate::c::volatile_write(((67109384i32) as usize as *mut u16), 0u16);
        if (((((&raw mut lman).cast::<u8>()).wrapping_add(4)).read()) as i32) == 15i32 {
            if ((({
                let __p1 = (((((&raw mut lman).cast::<u8>()).wrapping_add(36)).wrapping_add(4))
                    .cast::<u16>())
                .wrapping_offset(
                    (((((&raw mut lman).cast::<u8>()).wrapping_add(16)).read()) as i32) as isize,
                );
                let __t2 = ((__p1).read()).wrapping_sub(1);
                (__p1).write(__t2);
                __t2
            }) as i32)
                == 0i32)
                || (((((((((&raw mut gRfuSlotStatusNI).cast::<*mut u8>()).cast::<*mut u8>())
                    .wrapping_offset(
                        (((((&raw mut lman).cast::<u8>()).wrapping_add(16)).read()) as i32)
                            as isize,
                    ))
                .read())
                .cast::<u16>())
                .read()) as i32)
                    == 39i32)
            {
                rfu_LMAN_requestChangeAgbClockMaster();
                (((&raw mut lman).cast::<u8>()).wrapping_add(4)).write(24u8);
                rfu_clearSlot(
                    4u8,
                    (((&raw mut lman).cast::<u8>()).wrapping_add(16)).read(),
                );
                let __p3 = (((&raw mut lman).cast::<u8>()).wrapping_add(36));
                (__p3).write(
                    (((((__p3).read()) as i32)
                        & !(crate::c::shl_i32(
                            1i32,
                            (((((&raw mut lman).cast::<u8>()).wrapping_add(16)).read()) as u32),
                        ))) as u8),
                );
                ((((((&raw mut lman).cast::<u8>()).wrapping_add(36)).wrapping_add(4))
                    .cast::<u16>())
                .wrapping_offset(
                    (((((&raw mut lman).cast::<u8>()).wrapping_add(16)).read()) as i32) as isize,
                ))
                .write(0u16);
            }
        }
        crate::c::volatile_write(((67109384i32) as usize as *mut u16), imeBak);
        if (((((&raw mut lman).cast::<u8>()).wrapping_add(4)).read()) as i32) == 24i32 {
            if (((((&raw mut lman).cast::<u8>()).wrapping_add(2)).read_volatile()) as i32) == 1i32 {
                rfu_LMAN_requestChangeAgbClockMaster();
            }
            if (((((&raw mut lman).cast::<u8>()).wrapping_add(2)).read_volatile()) as i32) == 0i32 {
                (((&raw mut lman).cast::<u8>()).wrapping_add(4)).write({
                    let __v4 = 0u8;
                    (((&raw mut lman).cast::<u8>()).wrapping_add(5)).write(__v4);
                    __v4
                });
                rfu_LMAN_disconnect(
                    ((((((((&raw mut gRfuLinkStatus).cast::<*mut u8>()).read()).wrapping_add(2))
                        .read()) as i32)
                        | ((((((&raw mut gRfuLinkStatus).cast::<*mut u8>()).read())
                            .wrapping_add(3))
                        .read()) as i32)) as u8),
                );
                ((((&raw mut lman).cast::<u8>()).wrapping_add(20)).cast::<u16>()).write(0u16);
                rfu_LMAN_occureCallback(37u8, 1u8);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn rfu_LMAN_CHILD_checkSendChildName2() {
    unsafe {
        if ((((((&raw mut lman).cast::<u8>()).wrapping_add(4)).read()) as i32) == 15i32)
            && (((((((((&raw mut gRfuSlotStatusNI).cast::<*mut u8>()).cast::<*mut u8>())
                .wrapping_offset(
                    (((((&raw mut lman).cast::<u8>()).wrapping_add(16)).read()) as i32) as isize,
                ))
            .read())
            .cast::<u16>())
            .read()) as i32)
                == 38i32)
        {
            (((&raw mut lman).cast::<u8>()).wrapping_add(4)).write({
                let __v1 = 0u8;
                (((&raw mut lman).cast::<u8>()).wrapping_add(5)).write(__v1);
                __v1
            });
            rfu_clearSlot(
                4u8,
                (((&raw mut lman).cast::<u8>()).wrapping_add(16)).read(),
            );
            let __p2 = (((&raw mut lman).cast::<u8>()).wrapping_add(36));
            (__p2).write(
                (((((__p2).read()) as i32)
                    & !(crate::c::shl_i32(
                        1i32,
                        (((((&raw mut lman).cast::<u8>()).wrapping_add(16)).read()) as u32),
                    ))) as u8),
            );
            ((((((&raw mut lman).cast::<u8>()).wrapping_add(36)).wrapping_add(4)).cast::<u16>())
                .wrapping_offset(
                    (((((&raw mut lman).cast::<u8>()).wrapping_add(16)).read()) as i32) as isize,
                ))
            .write(0u16);
            rfu_LMAN_occureCallback(36u8, 0u8);
        }
    }
}
pub(crate) unsafe extern "C" fn rfu_LMAN_CHILD_linkRecoveryProcess() {
    unsafe {
        if ((((((&raw mut lman).cast::<u8>()).wrapping_add(6)).read()) as i32) == 0i32)
            && ((((((&raw mut lman).cast::<u8>()).wrapping_add(10)).read()) as i32) == 1i32)
        {
            ((((&raw mut lman).cast::<u8>()).wrapping_add(17)).cast::<u8>())
                .write((((&raw mut lman).cast::<u8>()).wrapping_add(4)).read());
            (((((&raw mut lman).cast::<u8>()).wrapping_add(17)).cast::<u8>()).wrapping_offset(1))
                .write((((&raw mut lman).cast::<u8>()).wrapping_add(5)).read());
            (((&raw mut lman).cast::<u8>()).wrapping_add(4)).write(16u8);
            (((&raw mut lman).cast::<u8>()).wrapping_add(5)).write(17u8);
            (((&raw mut lman).cast::<u8>()).wrapping_add(10)).write(2u8);
        }
    }
}
pub(crate) unsafe extern "C" fn rfu_LMAN_CHILD_checkEnableParentCandidate() -> u8 {
    unsafe {
        let mut i: u8 = 0u8;
        let mut serialNo: *mut u16 = core::ptr::null_mut();
        let mut flags: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32)
                    < ((((((&raw mut gRfuLinkStatus).cast::<*mut u8>()).read()).wrapping_add(8))
                        .read()) as i32))
                {
                    break 'l1;
                }
                'l2: {
                    {
                        serialNo = (((&raw mut lman).cast::<u8>())
                            .wrapping_add(32)
                            .cast::<*mut u16>())
                        .read();
                        'l3: loop {
                            if !((((serialNo).read()) as i32) != 65535i32) {
                                break 'l3;
                            }
                            'l4: {
                                if (((((((((&raw mut gRfuLinkStatus).cast::<*mut u8>()).read())
                                    .wrapping_add(20))
                                .cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize * 32))
                                .wrapping_add(4)
                                .cast::<u16>())
                                .read()) as i32)
                                    == (((serialNo).read()) as i32)
                                {
                                    flags = ((((flags) as i32)
                                        | crate::c::shl_i32(1i32, ((i) as u32)))
                                        as u8);
                                }
                            }
                            serialNo = (serialNo).wrapping_offset(1);
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return flags;
    }
}
pub(crate) unsafe extern "C" fn rfu_LMAN_occureCallback(msg: u8, param_count: u8) {
    unsafe {
        let mut msg = msg;
        let mut param_count = param_count;
        if core::mem::transmute::<_, usize>(
            (((&raw mut lman).cast::<u8>())
                .wrapping_add(64)
                .cast::<Option<unsafe extern "C" fn(u8, u8)>>())
            .read(),
        ) != 0usize
        {
            ((((&raw mut lman).cast::<u8>())
                .wrapping_add(64)
                .cast::<Option<unsafe extern "C" fn(u8, u8)>>())
            .read())
            .unwrap_unchecked()(msg, param_count);
        }
        ((((&raw mut lman).cast::<u8>()).wrapping_add(20)).cast::<u16>()).write({
            let __v1 = 0u16;
            (((((&raw mut lman).cast::<u8>()).wrapping_add(20)).cast::<u16>()).wrapping_offset(1))
                .write(__v1);
            __v1
        });
    }
}
pub(crate) unsafe extern "C" fn rfu_LMAN_disconnect(bm_disconnectedSlot: u8) {
    unsafe {
        let mut bm_disconnectedSlot = bm_disconnectedSlot;
        let mut active_bak: u8 = (((&raw mut lman).cast::<u8>()).wrapping_add(14)).read();
        (((&raw mut lman).cast::<u8>()).wrapping_add(14)).write(1u8);
        rfu_REQ_disconnect(bm_disconnectedSlot);
        rfu_waitREQComplete();
        (((&raw mut lman).cast::<u8>()).wrapping_add(14)).write(active_bak);
    }
}
pub(crate) unsafe extern "C" fn rfu_LMAN_reflectCommunicationStatus(bm_disconnectedSlot: u8) {
    unsafe {
        let mut bm_disconnectedSlot = bm_disconnectedSlot;
        let mut i: u8 = 0u8;
        if (((((&raw mut gRfuLinkStatus).cast::<*mut u8>()).read()).wrapping_add(4)).read()) != 0 {
            {
                i = 0u8;
                'l1: loop {
                    if !(((i) as i32) < 4i32) {
                        break 'l1;
                    }
                    'l2: {
                        if ((((((((((&raw mut gRfuSlotStatusNI).cast::<*mut u8>())
                            .cast::<*mut u8>())
                        .wrapping_offset(((i) as i32) as isize))
                        .read())
                        .cast::<u16>())
                        .read()) as i32)
                            & 32768i32)
                            != 0)
                            && ((((((((((&raw mut gRfuSlotStatusNI).cast::<*mut u8>())
                                .cast::<*mut u8>())
                            .wrapping_offset(((i) as i32) as isize))
                            .read())
                            .wrapping_add(26))
                            .read()) as i32)
                                & ((bm_disconnectedSlot) as i32))
                                != 0)
                        {
                            rfu_changeSendTarget(
                                32u8,
                                i,
                                ((((((((((&raw mut gRfuSlotStatusNI).cast::<*mut u8>())
                                    .cast::<*mut u8>())
                                .wrapping_offset(((i) as i32) as isize))
                                .read())
                                .wrapping_add(26))
                                .read()) as i32)
                                    & !((bm_disconnectedSlot) as i32))
                                    as u8),
                            );
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
        }
        if (((((&raw mut gRfuLinkStatus).cast::<*mut u8>()).read()).wrapping_add(5)).read()) != 0 {
            {
                i = 0u8;
                'l3: loop {
                    if !(((i) as i32) < 4i32) {
                        break 'l3;
                    }
                    'l4: {
                        if (((((((((((&raw mut gRfuSlotStatusNI).cast::<*mut u8>())
                            .cast::<*mut u8>())
                        .wrapping_offset(((i) as i32) as isize))
                        .read())
                        .wrapping_add(52))
                        .cast::<u16>())
                        .read()) as i32)
                            & 32768i32)
                            != 0)
                            && (((((((((((&raw mut gRfuSlotStatusNI).cast::<*mut u8>())
                                .cast::<*mut u8>())
                            .wrapping_offset(((i) as i32) as isize))
                            .read())
                            .wrapping_add(52))
                            .wrapping_add(26))
                            .read()) as i32)
                                & ((bm_disconnectedSlot) as i32))
                                != 0)
                        {
                            rfu_NI_stopReceivingData(i);
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
        }
        if (((((&raw mut gRfuLinkStatus).cast::<*mut u8>()).read()).wrapping_add(6)).read()) != 0 {
            let __p1 = (((&raw mut gRfuLinkStatus).cast::<*mut u8>()).read()).wrapping_add(6);
            (__p1).write((((((__p1).read()) as i32) & !((bm_disconnectedSlot) as i32)) as u8));
            {
                i = 0u8;
                'l5: loop {
                    if !(((i) as i32) < 4i32) {
                        break 'l5;
                    }
                    'l6: {
                        if (((((((((&raw mut gRfuSlotStatusUNI).cast::<*mut u8>())
                            .cast::<*mut u8>())
                        .wrapping_offset(((i) as i32) as isize))
                        .read())
                        .cast::<u16>())
                        .read()) as i32)
                            == 32804i32)
                            && ((((bm_disconnectedSlot) as i32)
                                & ((((((((&raw mut gRfuSlotStatusUNI).cast::<*mut u8>())
                                    .cast::<*mut u8>())
                                .wrapping_offset(((i) as i32) as isize))
                                .read())
                                .wrapping_add(3))
                                .read()) as i32))
                                != 0)
                        {
                            let __p2 = (((((&raw mut gRfuSlotStatusUNI).cast::<*mut u8>())
                                .cast::<*mut u8>())
                            .wrapping_offset(((i) as i32) as isize))
                            .read())
                            .wrapping_add(3);
                            (__p2).write(
                                (((((__p2).read()) as i32) & !((bm_disconnectedSlot) as i32))
                                    as u8),
                            );
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn rfu_LMAN_checkNICommunicateStatus() {
    unsafe {
        let mut i: u8 = 0u8;
        let mut j: u8 = 0u8;
        let mut flags: u8 = 0u8;
        if ((((&raw mut lman).cast::<u8>())
            .wrapping_add(24)
            .cast::<u16>())
        .read())
            != 0
        {
            if (((((&raw mut gRfuLinkStatus).cast::<*mut u8>()).read()).wrapping_add(4)).read())
                != 0
            {
                {
                    i = 0u8;
                    'l1: loop {
                        if !(((i) as i32) < 4i32) {
                            break 'l1;
                        }
                        'l2: {
                            if (((((((((&raw mut gRfuSlotStatusNI).cast::<*mut u8>())
                                .cast::<*mut u8>())
                            .wrapping_offset(((i) as i32) as isize))
                            .read())
                            .cast::<u16>())
                            .read()) as i32)
                                & 32768i32)
                                != 0
                            {
                                flags = 0u8;
                                {
                                    j = 0u8;
                                    'l3: loop {
                                        if !(((j) as i32) < 4i32) {
                                            break 'l3;
                                        }
                                        'l4: {
                                            if ((crate::c::shr_i32(
                                                ((((((((&raw mut gRfuSlotStatusNI)
                                                    .cast::<*mut u8>())
                                                .cast::<*mut u8>())
                                                .wrapping_offset(((i) as i32) as isize))
                                                .read())
                                                .wrapping_add(26))
                                                .read())
                                                    as i32),
                                                ((j) as u32),
                                            ) & 1i32)
                                                != 0)
                                                && (((((((((&raw mut gRfuSlotStatusNI)
                                                    .cast::<*mut u8>())
                                                .cast::<*mut u8>())
                                                .wrapping_offset(((j) as i32) as isize))
                                                .read())
                                                .wrapping_add(2)
                                                .cast::<u16>())
                                                .read())
                                                    as i32)
                                                    > (((((&raw mut lman).cast::<u8>())
                                                        .wrapping_add(24)
                                                        .cast::<u16>())
                                                    .read())
                                                        as i32))
                                            {
                                                flags = ((((flags) as i32)
                                                    | crate::c::shl_i32(1i32, ((j) as u32)))
                                                    as u8);
                                            }
                                            if (flags) != 0 {
                                                rfu_changeSendTarget(
                                                    32u8,
                                                    i,
                                                    ((((flags) as i32)
                                                        ^ ((((((((&raw mut gRfuSlotStatusNI)
                                                            .cast::<*mut u8>())
                                                        .cast::<*mut u8>())
                                                        .wrapping_offset(((i) as i32) as isize))
                                                        .read())
                                                        .wrapping_add(26))
                                                        .read())
                                                            as i32))
                                                        as u8),
                                                );
                                            }
                                        }
                                        j = (j).wrapping_add(1);
                                    }
                                }
                            }
                        }
                        i = (i).wrapping_add(1);
                    }
                }
            }
            if (((((&raw mut gRfuLinkStatus).cast::<*mut u8>()).read()).wrapping_add(5)).read())
                != 0
            {
                {
                    i = 0u8;
                    'l5: loop {
                        if !(((i) as i32) < 4i32) {
                            break 'l5;
                        }
                        'l6: {
                            if (((((((((((&raw mut gRfuSlotStatusNI).cast::<*mut u8>())
                                .cast::<*mut u8>())
                            .wrapping_offset(((i) as i32) as isize))
                            .read())
                            .wrapping_add(52))
                            .cast::<u16>())
                            .read()) as i32)
                                & 32768i32)
                                != 0)
                                && ((((((((((&raw mut gRfuSlotStatusNI).cast::<*mut u8>())
                                    .cast::<*mut u8>())
                                .wrapping_offset(((i) as i32) as isize))
                                .read())
                                .wrapping_add(52))
                                .wrapping_add(2)
                                .cast::<u16>())
                                .read()) as i32)
                                    > (((((&raw mut lman).cast::<u8>())
                                        .wrapping_add(24)
                                        .cast::<u16>())
                                    .read()) as i32))
                            {
                                rfu_NI_stopReceivingData(i);
                            }
                        }
                        i = (i).wrapping_add(1);
                    }
                }
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rfu_LMAN_setMSCCallback(
    MSC_callback_p: Option<unsafe extern "C" fn(u16)>,
) {
    unsafe {
        let mut MSC_callback_p = MSC_callback_p;
        (((&raw mut lman).cast::<u8>())
            .wrapping_add(68)
            .cast::<Option<unsafe extern "C" fn(u16)>>())
        .write(MSC_callback_p);
        rfu_setMSCCallback(Some(rfu_LMAN_MSC_callback));
    }
}
pub(crate) unsafe extern "C" fn rfu_LMAN_setLMANCallback(
    func: Option<unsafe extern "C" fn(u8, u8)>,
) {
    unsafe {
        let mut func = func;
        (((&raw mut lman).cast::<u8>())
            .wrapping_add(64)
            .cast::<Option<unsafe extern "C" fn(u8, u8)>>())
        .write(func);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rfu_LMAN_setLinkRecovery(enable_flag: u8, recovery_period: u16) -> u8 {
    unsafe {
        let mut enable_flag = enable_flag;
        let mut recovery_period = recovery_period;
        let mut imeBak: u16 = 0u16;
        if ((((((&raw mut lman).cast::<u8>()).wrapping_add(9)).read()) != 0)
            && (((enable_flag) as i32) == 0i32))
            && (((((&raw mut lman).cast::<u8>()).wrapping_add(48)).read()) != 0)
        {
            return 5u8;
        }
        imeBak = ((67109384i32) as usize as *mut u16).read_volatile();
        crate::c::volatile_write(((67109384i32) as usize as *mut u16), 0u16);
        (((&raw mut lman).cast::<u8>()).wrapping_add(9)).write(enable_flag);
        ((((&raw mut lman).cast::<u8>()).wrapping_add(48))
            .wrapping_add(2)
            .cast::<u16>())
        .write(recovery_period);
        crate::c::volatile_write(((67109384i32) as usize as *mut u16), imeBak);
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn rfu_LMAN_setNIFailCounterLimit(NI_failCounter_limit: u16) -> u8 {
    unsafe {
        let mut NI_failCounter_limit = NI_failCounter_limit;
        if (((((((&raw mut gRfuLinkStatus).cast::<*mut u8>()).read()).wrapping_add(4)).read())
            as i32)
            | ((((((&raw mut gRfuLinkStatus).cast::<*mut u8>()).read()).wrapping_add(5)).read())
                as i32))
            != 0
        {
            ((((&raw mut lman).cast::<u8>()).wrapping_add(20)).cast::<u16>()).write(6u16);
            rfu_LMAN_occureCallback(243u8, 1u8);
            return 6u8;
        }
        (((&raw mut lman).cast::<u8>())
            .wrapping_add(24)
            .cast::<u16>())
        .write(NI_failCounter_limit);
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn rfu_LMAN_setFastSearchParent(enable_flag: u8) -> u8 {
    unsafe {
        let mut enable_flag = enable_flag;
        if (((((((&raw mut lman).cast::<u8>()).wrapping_add(4)).read()) as i32) == 9i32)
            || ((((((&raw mut lman).cast::<u8>()).wrapping_add(4)).read()) as i32) == 10i32))
            || ((((((&raw mut lman).cast::<u8>()).wrapping_add(4)).read()) as i32) == 11i32)
        {
            ((((&raw mut lman).cast::<u8>()).wrapping_add(20)).cast::<u16>()).write(7u16);
            rfu_LMAN_occureCallback(243u8, 1u8);
            return 7u8;
        }
        if (enable_flag) != 0 {
            (((&raw mut lman).cast::<u8>()).wrapping_add(11)).write(1u8);
        } else {
            (((&raw mut lman).cast::<u8>()).wrapping_add(11)).write(0u8);
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn rfu_LMAN_managerChangeAgbClockMaster() {
    unsafe {
        if (((((&raw mut lman).cast::<u8>()).wrapping_add(2)).read_volatile()) as i32) != 0i32 {
            crate::c::volatile_write(((&raw mut lman).cast::<u8>()).wrapping_add(2), 0u8);
            rfu_LMAN_occureCallback(69u8, 0u8);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rfu_LMAN_requestChangeAgbClockMaster() {
    unsafe {
        if (((((&raw mut lman).cast::<u8>()).wrapping_add(2)).read_volatile()) as i32) == 0i32 {
            rfu_LMAN_occureCallback(69u8, 0u8);
        } else {
            if (((((&raw mut lman).cast::<u8>()).wrapping_add(2)).read_volatile()) as i32) == 1i32 {
                crate::c::volatile_write(((&raw mut lman).cast::<u8>()).wrapping_add(2), 2u8);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rfu_LMAN_forceChangeSP() {
    unsafe {
        if ((((&raw mut lman).cast::<u8>()).wrapping_add(7)).read()) != 0 {
            'l1: {
                let __sw1 = (((((&raw mut lman).cast::<u8>()).wrapping_add(4)).read()) as i32);
                if __sw1 == 5i32 {
                    (((&raw mut lman).cast::<u8>()).wrapping_add(7)).write(3u8);
                    (((&raw mut lman).cast::<u8>()).wrapping_add(4)).write(9u8);
                    break 'l1;
                }
                if __sw1 == 6i32 {
                    (((&raw mut lman).cast::<u8>()).wrapping_add(7)).write(2u8);
                    (((&raw mut lman).cast::<u8>())
                        .wrapping_add(26)
                        .cast::<u16>())
                    .write(1u16);
                    break 'l1;
                }
                if __sw1 == 7i32 || __sw1 == 8i32 {
                    (((&raw mut lman).cast::<u8>()).wrapping_add(7)).write(2u8);
                    break 'l1;
                }
                if __sw1 == 9i32 || __sw1 == 10i32 {
                    (((&raw mut lman).cast::<u8>())
                        .wrapping_add(26)
                        .cast::<u16>())
                    .write(40u16);
                    break 'l1;
                }
                if __sw1 == 11i32 {
                    (((&raw mut lman).cast::<u8>())
                        .wrapping_add(26)
                        .cast::<u16>())
                    .write(40u16);
                    (((&raw mut lman).cast::<u8>()).wrapping_add(4)).write(10u8);
                    break 'l1;
                }
            }
        }
    }
}
