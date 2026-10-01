//! Translated from `src/AgbRfu_LinkManager.c` by tools/rustport/c2rs.py.
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
    dead_code,
    unused_assignments
)]

#[allow(unused_imports)]
use crate::c::*;
#[allow(unused_imports)]
use crate::consts::*;
use crate::librfu_rfu::{
    gRfuLinkStatus, gRfuSlotStatusNI, gRfuSlotStatusUNI, rfu_CHILD_getConnectRecoveryStatus,
    rfu_NI_CHILD_setSendGameName, rfu_NI_stopReceivingData, rfu_REQ_CHILD_endConnectRecovery,
    rfu_REQ_CHILD_pollConnectRecovery, rfu_REQ_CHILD_startConnectRecovery, rfu_REQ_RFUStatus,
    rfu_REQ_changeMasterSlave, rfu_REQ_configGameData, rfu_REQ_configSystem, rfu_REQ_disconnect,
    rfu_REQ_endConnectParent, rfu_REQ_endSearchChild, rfu_REQ_endSearchParent,
    rfu_REQ_pollConnectParent, rfu_REQ_pollSearchChild, rfu_REQ_pollSearchParent, rfu_REQ_reset,
    rfu_REQ_sendData, rfu_REQ_startConnectParent, rfu_REQ_startSearchChild,
    rfu_REQ_startSearchParent, rfu_REQ_stopMode, rfu_REQBN_softReset_and_checkID,
    rfu_REQBN_watchLink, rfu_UNI_PARENT_getDRAC_ACK, rfu_changeSendTarget, rfu_clearSlot,
    rfu_getConnectParentStatus, rfu_getMasterSlave, rfu_getRFUStatus, rfu_getSTWIRecvBuffer,
    rfu_setMSCCallback, rfu_setREQCallback, rfu_syncVBlank, rfu_waitREQComplete,
};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use core::ffi::c_void;
#[allow(unused_imports)]
use core::mem::zeroed;
#[allow(unused_imports)]
use core::ptr::null_mut;

const FSP_ON: u8 = 1;
const FSP_START: u8 = 2;
const LINK_RECOVERY_EXE: u8 = 2;
const LINK_RECOVERY_IMPOSSIBLE: u8 = 4;
const LINK_RECOVERY_START: u8 = 1;
const RN_ACCEPT: u8 = 1;
const RN_DISCONNECT: u8 = 4;
const RN_NAME_TIMER_CLEAR: u8 = 2;

#[unsafe(link_section = "common_data")]
pub static mut lman: linkManagerTag = unsafe { zeroed() };

/// `CpuSet` with this module's view of its types.
#[inline]
unsafe fn CpuSet(a0: *mut c_void, a1: *mut c_void, a2: u32) {
    unsafe {
        crate::syscall::CpuSet(a0 as _, a1 as _, a2);
    }
}

pub unsafe fn rfu_LMAN_REQBN_softReset_and_checkID() -> u32 {
    let id: u32 = rfu_REQBN_softReset_and_checkID();
    if id == RFU_ID {
        lman.RFU_powerOn_flag = 1;
    }
    if lman.state != LMAN_FORCED_STOP_AND_RFU_RESET
        && lman.state != LMAN_STATE_SOFT_RESET_AND_CHECK_ID
    {
        lman.state = {
            lman.next_state = LMAN_STATE_READY;
            lman.next_state
        };
    }
    lman.pcswitch_flag = 0;
    lman.reserveDisconnectSlot_flag = 0;
    lman.acceptCount = 0;
    lman.acceptSlot_flag = 0;
    lman.parent_child = MODE_NEUTRAL;
    rfu_LMAN_managerChangeAgbClockMaster();
    id
}
pub unsafe fn rfu_LMAN_REQ_sendData(mut clockChangeFlag: u8) {
    if (*gRfuLinkStatus).parentChild == MODE_CHILD {
        if (&raw mut lman.childClockSlave_flag).read_volatile() == RFU_CHILD_CLOCK_SLAVE_ON {
            clockChangeFlag = TRUE;
        } else {
            clockChangeFlag = FALSE;
        }
    } else {
        volatile_write(&raw mut lman.parentAck_flag, 0);
    }
    rfu_REQ_sendData(clockChangeFlag);
}
pub unsafe fn rfu_LMAN_initializeManager(
    LMAN_callback_p: Option<unsafe fn(u8, u8)>,
    MSC_callback_p: Option<unsafe fn(u16)>,
) -> u8 {
    if LMAN_callback_p.is_none() {
        return LMAN_ERROR_ILLEGAL_PARAMETER;
    }
    {
        {
            let mut tmp: u16 = 0;
            volatile_write(&raw mut tmp, 0);
            CpuSet(
                &raw mut tmp as *mut c_void,
                &raw mut lman as *mut c_void,
                0x1000024,
            );
        }
    }
    lman.parent_child = MODE_NEUTRAL;
    lman.LMAN_callback = LMAN_callback_p;
    lman.MSC_callback = MSC_callback_p;
    rfu_setMSCCallback(Some(rfu_LMAN_MSC_callback));
    rfu_setREQCallback(Some(rfu_LMAN_REQ_callback));
    0
}
unsafe fn rfu_LMAN_endManager() {
    {
        {
            let mut tmp: u16 = 0;
            volatile_write(&raw mut tmp, 0);
            CpuSet(
                &raw mut tmp as *mut c_void,
                &raw mut lman as *mut c_void,
                0x1000020,
            );
        }
    }
    lman.parent_child = MODE_NEUTRAL;
}
pub unsafe fn rfu_LMAN_initializeRFU(init_parameters: *mut InitializeParametersTag) {
    rfu_LMAN_clearVariables();
    lman.state = LMAN_STATE_SOFT_RESET_AND_CHECK_ID;
    lman.next_state = LMAN_STATE_RESET;
    lman.init_param = init_parameters;
    lman.linkRecovery_enable = (*init_parameters).linkRecovery_enable;
    lman.linkRecoveryTimer.count_max = (*init_parameters).linkRecovery_period;
    lman.NI_failCounter_limit = (*init_parameters).NI_failCounter_limit;
    if (*init_parameters).fastSearchParent_flag != 0 {
        lman.fastSearchParent_flag = FSP_ON;
    }
}
unsafe fn rfu_LMAN_clearVariables() {
    lman.state = {
        lman.next_state = LMAN_STATE_READY;
        lman.next_state
    };
    lman.parent_child = MODE_NEUTRAL;
    lman.pcswitch_flag = 0;
    lman.child_slot = 0;
    lman.connectSlot_flag_old = 0;
    lman.nameAcceptTimer.active = 0;
    lman.linkRecoveryTimer.active = 0;
    for i in 0..RFU_CHILD_MAX {
        lman.nameAcceptTimer.count[i] = 0;
        lman.linkRecoveryTimer.count[i] = 0;
    }
}
pub unsafe fn rfu_LMAN_powerDownRFU() {
    lman.state = LMAN_STATE_STOP_MODE;
}
pub unsafe fn rfu_LMAN_establishConnection(
    mut parent_child: u8,
    mut connect_period: u16,
    name_accept_period: u16,
    acceptable_serialNo_list: *mut u16,
) -> u8 {
    if lman.state != LMAN_STATE_READY
        && (lman.state != LMAN_STATE_WAIT_RECV_CHILD_NAME || parent_child != MODE_PARENT)
    {
        lman.param[0] = 1;
        rfu_LMAN_occureCallback(LMAN_MSG_LMAN_API_ERROR_RETURN, 1);
        return LMAN_ERROR_MANAGER_BUSY;
    }
    if rfu_getMasterSlave() == AGB_CLK_SLAVE {
        lman.param[0] = 2;
        rfu_LMAN_occureCallback(LMAN_MSG_LMAN_API_ERROR_RETURN, 1);
        return LMAN_ERROR_AGB_CLK_SLAVE;
    }
    let mut i: u8 = 0;
    let mut serial_list: *mut u16 = acceptable_serialNo_list;
    while i < 16 {
        if *({
            let t2 = serial_list;
            serial_list = serial_list.at(1);
            t2
        }) == 0xFFFF
        {
            break;
        }
        i += 1;
    }
    if i == 16 {
        lman.param[0] = 4;
        rfu_LMAN_occureCallback(LMAN_MSG_LMAN_API_ERROR_RETURN, 1);
        return LMAN_ERROR_ILLEGAL_PARAMETER;
    }
    if parent_child > MODE_PARENT {
        lman.pcswitch_flag = PCSWITCH_1ST_SC_START;
        parent_child = MODE_PARENT;
        connect_period = 0;
    } else {
        lman.pcswitch_flag = 0;
    }
    if parent_child != MODE_CHILD {
        lman.state = LMAN_STATE_START_SEARCH_CHILD;
    } else {
        lman.state = LMAN_STATE_START_SEARCH_PARENT;
        if lman.fastSearchParent_flag != 0 {
            lman.fastSearchParent_flag = FSP_START;
        }
    }
    lman.parent_child = parent_child;
    lman.connect_period = connect_period;
    lman.nameAcceptTimer.count_max = name_accept_period;
    lman.acceptable_serialNo_list = acceptable_serialNo_list;
    0
}
pub unsafe fn rfu_LMAN_CHILD_connectParent(parentId: u16, connect_period: u16) -> u8 {
    if lman.state != LMAN_STATE_READY && (lman.state < 9 || lman.state > 11) {
        lman.param[0] = 1;
        rfu_LMAN_occureCallback(LMAN_MSG_LMAN_API_ERROR_RETURN, 1);
        return LMAN_ERROR_MANAGER_BUSY;
    }
    if rfu_getMasterSlave() == AGB_CLK_SLAVE {
        lman.param[0] = 2;
        rfu_LMAN_occureCallback(LMAN_MSG_LMAN_API_ERROR_RETURN, 1);
        return LMAN_ERROR_AGB_CLK_SLAVE;
    }
    let mut i: u8 = 0;
    while i < (*gRfuLinkStatus).findParentCount {
        if (*gRfuLinkStatus).partner[i].id == parentId {
            break;
        }
        i += 1;
    }
    if (*gRfuLinkStatus).findParentCount == 0 || i == (*gRfuLinkStatus).findParentCount {
        lman.param[0] = 3;
        rfu_LMAN_occureCallback(LMAN_MSG_LMAN_API_ERROR_RETURN, 1);
        return LMAN_ERROR_PID_NOT_FOUND;
    }
    if lman.state == LMAN_STATE_READY || lman.state == LMAN_STATE_START_SEARCH_PARENT {
        lman.state = LMAN_STATE_START_CONNECT_PARENT;
        lman.next_state = LMAN_STATE_POLL_CONNECT_PARENT;
    } else {
        lman.state = LMAN_STATE_END_SEARCH_PARENT;
        lman.next_state = LMAN_STATE_START_CONNECT_PARENT;
    }
    lman.work = parentId;
    lman.connect_period = connect_period;
    if lman.pcswitch_flag != 0 {
        lman.pcswitch_flag = PCSWITCH_CP;
    }
    0
}
unsafe fn rfu_LMAN_PARENT_stopWaitLinkRecoveryAndDisconnect(bm_targetSlot: u8) {
    if bm_targetSlot as i32 & lman.linkRecoveryTimer.active as i32 == 0 {
        return;
    }
    lman.linkRecoveryTimer.active &= !bm_targetSlot;
    for i in 0..RFU_CHILD_MAX {
        if shr_i32(bm_targetSlot as i32, i as u32) & 1 != 0 {
            lman.linkRecoveryTimer.count[i] = 0;
        }
    }
    let i: u8 = (*gRfuLinkStatus).linkLossSlotFlag & bm_targetSlot;
    if i != 0 {
        rfu_LMAN_disconnect(i);
    }
    lman.param[0] = i as u16;
    rfu_LMAN_occureCallback(LMAN_MSG_LINK_RECOVERY_FAILED_AND_DISCONNECTED, i);
}
pub unsafe fn rfu_LMAN_stopManager(forced_stop_and_RFU_reset_flag: u8) {
    let mut msg: u8 = 0;
    lman.pcswitch_flag = 0;
    if forced_stop_and_RFU_reset_flag != 0 {
        rfu_LMAN_clearVariables();
        lman.state = LMAN_FORCED_STOP_AND_RFU_RESET;
        return;
    }
    match lman.state {
        LMAN_STATE_START_SEARCH_CHILD => {
            lman.state = LMAN_STATE_WAIT_RECV_CHILD_NAME;
            lman.next_state = LMAN_STATE_READY;
            msg = LMAN_MSG_SEARCH_CHILD_PERIOD_EXPIRED;
        }
        LMAN_STATE_POLL_SEARCH_CHILD => {
            lman.state = LMAN_STATE_END_SEARCH_CHILD;
            lman.next_state = LMAN_STATE_WAIT_RECV_CHILD_NAME;
        }
        LMAN_STATE_END_SEARCH_CHILD => {
            lman.state = LMAN_STATE_END_SEARCH_CHILD;
            lman.next_state = LMAN_STATE_WAIT_RECV_CHILD_NAME;
        }
        LMAN_STATE_WAIT_RECV_CHILD_NAME => {}
        LMAN_STATE_START_SEARCH_PARENT => {
            lman.state = {
                lman.next_state = LMAN_STATE_READY;
                lman.next_state
            };
            msg = LMAN_MSG_SEARCH_PARENT_PERIOD_EXPIRED;
        }
        LMAN_STATE_POLL_SEARCH_PARENT => {
            lman.state = LMAN_STATE_END_SEARCH_PARENT;
            lman.next_state = LMAN_STATE_READY;
        }
        LMAN_STATE_END_SEARCH_PARENT => {
            lman.state = LMAN_STATE_END_SEARCH_PARENT;
            lman.next_state = LMAN_STATE_READY;
        }
        LMAN_STATE_START_CONNECT_PARENT => {
            lman.state = {
                lman.next_state = LMAN_STATE_READY;
                lman.next_state
            };
            msg = LMAN_MSG_CONNECT_PARENT_FAILED;
        }
        LMAN_STATE_POLL_CONNECT_PARENT => {
            lman.state = LMAN_STATE_END_CONNECT_PARENT;
        }
        LMAN_STATE_END_CONNECT_PARENT => {
            lman.state = LMAN_STATE_END_CONNECT_PARENT;
        }
        LMAN_STATE_SEND_CHILD_NAME => {}
        LMAN_STATE_START_LINK_RECOVERY => {
            lman.state = lman.state_bak[0];
            lman.next_state = lman.state_bak[1];
            rfu_LMAN_disconnect((*gRfuLinkStatus).linkLossSlotFlag);
            lman.param[0] = (*gRfuLinkStatus).linkLossSlotFlag as u16;
            rfu_LMAN_occureCallback(LMAN_MSG_LINK_RECOVERY_FAILED_AND_DISCONNECTED, 1);
            return;
        }
        LMAN_STATE_POLL_LINK_RECOVERY => {
            lman.state = LMAN_STATE_END_LINK_RECOVERY;
        }
        LMAN_STATE_END_LINK_RECOVERY => {
            lman.state = LMAN_STATE_END_LINK_RECOVERY;
        }
        _ => {
            lman.state = {
                lman.next_state = LMAN_STATE_READY;
                lman.next_state
            };
            msg = LMAN_MSG_MANAGER_STOPPED;
        }
    }
    if lman.state == LMAN_STATE_READY {
        rfu_LMAN_occureCallback(msg, 0);
    }
}
unsafe fn rfu_LMAN_linkWatcher(REQ_commandID: u16) -> u8 {
    let mut bm_linkLossSlot: u8 = 0;
    let mut reason: u8 = 0;
    let mut bm_linkRecoverySlot: u8 = 0;
    let mut bm_disconnectSlot: u8 = 0;
    let mut disconnect_occure_flag: u8 = FALSE;
    rfu_REQBN_watchLink(
        REQ_commandID,
        &raw mut bm_linkLossSlot,
        &raw mut reason,
        &raw mut bm_linkRecoverySlot,
    );
    if bm_linkLossSlot != 0 {
        lman.param[0] = bm_linkLossSlot as u16;
        lman.param[1] = reason as u16;
        if lman.linkRecovery_enable != 0 {
            lman.linkRecovery_start_flag = LINK_RECOVERY_START;
            if lman.parent_child == 0x00 && reason == 0x00 {
                lman.linkRecovery_start_flag = LINK_RECOVERY_IMPOSSIBLE;
            }
            if lman.linkRecovery_start_flag == LINK_RECOVERY_START {
                for i in 0..RFU_CHILD_MAX {
                    if shr_i32(bm_linkLossSlot as i32, i as u32) & 1 != 0 {
                        lman.linkRecoveryTimer.active |= shl_i32(1, i as u32) as u8;
                        lman.linkRecoveryTimer.count[i] = lman.linkRecoveryTimer.count_max;
                    }
                }
                rfu_LMAN_occureCallback(LMAN_MSG_LINK_LOSS_DETECTED_AND_START_RECOVERY, 1);
            } else {
                lman.linkRecovery_start_flag = 0;
                rfu_LMAN_disconnect(bm_linkLossSlot);
                disconnect_occure_flag = TRUE;
                rfu_LMAN_occureCallback(LMAN_MSG_LINK_RECOVERY_FAILED_AND_DISCONNECTED, 1);
            }
        } else {
            rfu_LMAN_disconnect(bm_linkLossSlot);
            disconnect_occure_flag = TRUE;
            rfu_LMAN_occureCallback(LMAN_MSG_LINK_LOSS_DETECTED_AND_DISCONNECTED, 2);
        }
        rfu_LMAN_managerChangeAgbClockMaster();
    }
    if (*gRfuLinkStatus).parentChild == MODE_PARENT {
        if bm_linkRecoverySlot != 0 {
            for i in 0..RFU_CHILD_MAX {
                if shr_i32(lman.linkRecoveryTimer.active as i32, i as u32) & 1 != 0
                    && shr_i32(bm_linkRecoverySlot as i32, i as u32) & 1 != 0
                {
                    lman.linkRecoveryTimer.count[i] = 0;
                }
            }
            lman.linkRecoveryTimer.active &= !bm_linkRecoverySlot;
            lman.param[0] = bm_linkRecoverySlot as u16;
            rfu_LMAN_occureCallback(LMAN_MSG_LINK_RECOVERY_SUCCESSED, 1);
        }
        if lman.linkRecoveryTimer.active != 0 {
            bm_disconnectSlot = 0;
            for i in 0..RFU_CHILD_MAX {
                if shr_i32(lman.linkRecoveryTimer.active as i32, i as u32) & 1 != 0
                    && lman.linkRecoveryTimer.count[i] != 0
                    && ({
                        lman.linkRecoveryTimer.count[i] -= 1;
                        lman.linkRecoveryTimer.count[i]
                    }) == 0
                {
                    lman.linkRecoveryTimer.active &= !(shl_i32(1, i as u32) as u8);
                    bm_disconnectSlot |= shl_i32(1, i as u32) as u8;
                }
            }
            if bm_disconnectSlot != 0 {
                rfu_LMAN_disconnect(bm_disconnectSlot);
                disconnect_occure_flag = TRUE;
                lman.param[0] = bm_disconnectSlot as u16;
                rfu_LMAN_occureCallback(LMAN_MSG_LINK_RECOVERY_FAILED_AND_DISCONNECTED, 1);
            }
        }
        if lman.linkRecoveryTimer.active == 0 {
            lman.linkRecovery_start_flag = 0;
        }
    }
    disconnect_occure_flag
}
pub unsafe fn rfu_LMAN_syncVBlank() {
    if rfu_syncVBlank() != 0 {
        rfu_LMAN_occureCallback(LMAN_MSG_WATCH_DOG_TIMER_ERROR, 0);
        rfu_LMAN_managerChangeAgbClockMaster();
    }
}
pub unsafe fn rfu_LMAN_manager_entity(rand: u32) {
    let mut msg: u8 = 0;
    if lman.LMAN_callback.is_none() && lman.state != LMAN_STATE_READY {
        lman.state = LMAN_STATE_READY;
        return;
    }
    if lman.pcswitch_flag != 0 {
        rfu_LMAN_settingPCSWITCH(rand);
    }
    loop {
        if lman.state != LMAN_STATE_READY {
            rfu_waitREQComplete();
            lman.active = 1;
            match lman.state {
                LMAN_FORCED_STOP_AND_RFU_RESET => {
                    if rfu_LMAN_REQBN_softReset_and_checkID() == RFU_ID {
                        msg = LMAN_MSG_MANAGER_FORCED_STOPPED_AND_RFU_RESET;
                    } else {
                        msg = LMAN_MSG_RFU_FATAL_ERROR;
                    }
                    lman.state = {
                        lman.next_state = LMAN_STATE_READY;
                        lman.next_state
                    };
                    rfu_LMAN_occureCallback(msg, 0);
                }
                LMAN_STATE_SOFT_RESET_AND_CHECK_ID => {
                    if rfu_LMAN_REQBN_softReset_and_checkID() == RFU_ID {
                        lman.state = lman.next_state;
                        lman.next_state = LMAN_STATE_CONFIG_SYSTEM;
                    } else {
                        lman.state = {
                            lman.next_state = LMAN_STATE_READY;
                            lman.next_state
                        };
                        rfu_LMAN_occureCallback(LMAN_MSG_RFU_FATAL_ERROR, 0);
                    }
                }
                LMAN_STATE_RESET => {
                    rfu_REQ_reset();
                }
                LMAN_STATE_CONFIG_SYSTEM => {
                    rfu_REQ_configSystem(
                        (*lman.init_param).availSlot_flag,
                        (*lman.init_param).maxMFrame,
                        (*lman.init_param).MC_TimerCount,
                    );
                }
                LMAN_STATE_CONFIG_GAME_DATA => {
                    rfu_REQ_configGameData(
                        (*lman.init_param).mboot_flag,
                        (*lman.init_param).serialNo,
                        (*lman.init_param).gameName,
                        (*lman.init_param).userName,
                    );
                }
                LMAN_STATE_START_SEARCH_CHILD => {
                    rfu_REQ_startSearchChild();
                }
                LMAN_STATE_POLL_SEARCH_CHILD => {
                    rfu_REQ_pollSearchChild();
                }
                LMAN_STATE_END_SEARCH_CHILD => {
                    rfu_REQ_endSearchChild();
                }
                LMAN_STATE_WAIT_RECV_CHILD_NAME => {}
                LMAN_STATE_START_SEARCH_PARENT => {
                    rfu_REQ_startSearchParent();
                }
                LMAN_STATE_POLL_SEARCH_PARENT => {
                    rfu_REQ_pollSearchParent();
                }
                LMAN_STATE_END_SEARCH_PARENT => {
                    rfu_REQ_endSearchParent();
                }
                LMAN_STATE_START_CONNECT_PARENT => {
                    rfu_REQ_startConnectParent(lman.work);
                }
                LMAN_STATE_POLL_CONNECT_PARENT => {
                    rfu_REQ_pollConnectParent();
                }
                LMAN_STATE_END_CONNECT_PARENT => {
                    rfu_REQ_endConnectParent();
                }
                LMAN_STATE_SEND_CHILD_NAME => {}
                LMAN_STATE_START_LINK_RECOVERY => {
                    rfu_REQ_CHILD_startConnectRecovery((*gRfuLinkStatus).linkLossSlotFlag);
                }
                LMAN_STATE_POLL_LINK_RECOVERY => {
                    rfu_REQ_CHILD_pollConnectRecovery();
                }
                LMAN_STATE_END_LINK_RECOVERY => {
                    rfu_REQ_CHILD_endConnectRecovery();
                }
                LMAN_STATE_MS_CHANGE => {
                    rfu_REQ_changeMasterSlave();
                }
                LMAN_STATE_WAIT_CLOCK_MASTER => {}
                LMAN_STATE_STOP_MODE => {
                    rfu_REQ_stopMode();
                }
                LMAN_STATE_BACK_STATE => {}
                _ => {}
            }
            rfu_waitREQComplete();
            lman.active = 0;
        }
        if lman.state == LMAN_STATE_END_LINK_RECOVERY || lman.state == LMAN_STATE_MS_CHANGE {
        } else {
            break;
        }
    }
    if (*gRfuLinkStatus).parentChild == MODE_PARENT && rfu_LMAN_linkWatcher(0) != 0 {
        return;
    }
    rfu_LMAN_PARENT_checkRecvChildName();
    rfu_LMAN_CHILD_checkSendChildName();
    rfu_LMAN_CHILD_linkRecoveryProcess();
    rfu_LMAN_checkNICommunicateStatus();
}
unsafe fn rfu_LMAN_settingPCSWITCH(rand: u32) {
    if lman.pcswitch_flag == PCSWITCH_3RD_SC_START {
        lman.parent_child = MODE_PARENT;
        lman.state = LMAN_STATE_START_SEARCH_CHILD;
        lman.connect_period = lman.pcswitch_period_bak;
        if lman.connect_period != 0 {
            lman.pcswitch_flag = PCSWITCH_3RD_SC;
        } else {
            lman.pcswitch_flag = PCSWITCH_1ST_SC_START;
        }
    }
    if lman.pcswitch_flag == PCSWITCH_1ST_SC_START {
        lman.parent_child = MODE_PARENT;
        lman.state = LMAN_STATE_START_SEARCH_CHILD;
        lman.connect_period = (rand % 140) as u16;
        lman.pcswitch_period_bak = 140 - lman.connect_period;
        if lman.connect_period != 0 {
            lman.pcswitch_flag = PCSWITCH_1ST_SC;
        } else {
            lman.pcswitch_flag = PCSWITCH_2ND_SP_START;
        }
    }
    if lman.pcswitch_flag == PCSWITCH_2ND_SP_START {
        lman.parent_child = MODE_CHILD;
        lman.connect_period = PCSWITCH_SP_PERIOD;
        lman.pcswitch_flag = PCSWITCH_2ND_SP;
        lman.state = LMAN_STATE_START_SEARCH_PARENT;
    }
}
pub(crate) unsafe fn rfu_LMAN_REQ_callback(reqCommandId: u16, mut reqResult: u16) {
    let mut status: u8 = 0;
    let mut stwiRecvBuffer: *mut u8 = null_mut();
    if lman.active != 0 {
        lman.active = 0;
        match reqCommandId {
            ID_RESET_REQ => {
                if reqResult == 0 {
                    lman.state = lman.next_state;
                    lman.next_state = LMAN_STATE_CONFIG_GAME_DATA;
                }
            }
            ID_SYSTEM_CONFIG_REQ => {
                if reqResult == 0 {
                    lman.state = lman.next_state;
                    lman.next_state = LMAN_STATE_READY;
                }
            }
            ID_GAME_CONFIG_REQ => {
                if reqResult == 0 {
                    lman.state = {
                        lman.next_state = LMAN_STATE_READY;
                        lman.next_state
                    };
                    rfu_LMAN_occureCallback(0x00, 0);
                }
            }
            25 => {
                if reqResult == 0 {
                    lman.state = {
                        lman.next_state = LMAN_STATE_POLL_SEARCH_CHILD;
                        lman.next_state
                    };
                }
            }
            26 => {
                if lman.connect_period != 0
                    && ({
                        lman.connect_period -= 1;
                        lman.connect_period
                    }) == 0
                {
                    lman.state = LMAN_STATE_END_SEARCH_CHILD;
                    lman.next_state = LMAN_STATE_WAIT_RECV_CHILD_NAME;
                }
            }
            27 => {
                if reqResult == 0 {
                    lman.state = lman.next_state;
                    lman.next_state = LMAN_STATE_READY;
                    if lman.pcswitch_flag == 0 {
                        rfu_LMAN_occureCallback(LMAN_MSG_SEARCH_CHILD_PERIOD_EXPIRED, 0);
                    }
                }
            }
            ID_SP_START_REQ => {
                if reqResult == 0 {
                    if lman.fastSearchParent_flag == FSP_ON && lman.connect_period > 1 {
                        lman.connect_period -= 1;
                    }
                    lman.state = {
                        lman.next_state = LMAN_STATE_POLL_SEARCH_PARENT;
                        lman.next_state
                    };
                }
            }
            ID_SP_POLL_REQ => {
                if reqResult == 0 {
                    status = rfu_LMAN_CHILD_checkEnableParentCandidate();
                    lman.param[0] = status as u16;
                    if status != 0 {
                        rfu_LMAN_occureCallback(LMAN_MSG_PARENT_FOUND, 1);
                    }
                    if lman.fastSearchParent_flag != 0
                        && lman.connect_period != 1
                        && (*gRfuLinkStatus).findParentCount == RFU_CHILD_MAX
                    {
                        rfu_REQ_endSearchParent();
                        rfu_waitREQComplete();
                        lman.state = LMAN_STATE_START_SEARCH_PARENT;
                        lman.fastSearchParent_flag = FSP_ON;
                    }
                }
                if lman.connect_period != 0
                    && ({
                        lman.connect_period -= 1;
                        lman.connect_period
                    }) == 0
                {
                    lman.state = LMAN_STATE_END_SEARCH_PARENT;
                    lman.next_state = LMAN_STATE_READY;
                }
            }
            ID_SP_END_REQ => {
                if reqResult == 0 {
                    lman.state = lman.next_state;
                    if lman.pcswitch_flag == 0 {
                        if lman.state == LMAN_STATE_READY {
                            rfu_LMAN_occureCallback(LMAN_MSG_SEARCH_PARENT_PERIOD_EXPIRED, 0);
                        }
                    } else if lman.pcswitch_flag != PCSWITCH_CP {
                        lman.state = LMAN_STATE_START_SEARCH_CHILD;
                        lman.pcswitch_flag = PCSWITCH_3RD_SC_START;
                    }
                }
            }
            31 => {
                if reqResult == 0 {
                    lman.state = {
                        lman.next_state = LMAN_STATE_POLL_CONNECT_PARENT;
                        lman.next_state
                    };
                }
            }
            ID_CP_POLL_REQ => {
                if reqResult == 0
                    && rfu_getConnectParentStatus(&raw mut status, &raw mut lman.child_slot) == 0
                    && status == 0
                {
                    lman.state = LMAN_STATE_END_CONNECT_PARENT;
                }
                if lman.connect_period != 0
                    && ({
                        lman.connect_period -= 1;
                        lman.connect_period
                    }) == 0
                {
                    lman.state = LMAN_STATE_END_CONNECT_PARENT;
                }
            }
            ID_CP_END_REQ => {
                if reqResult == 0
                    && rfu_getConnectParentStatus(&raw mut status, &raw mut lman.child_slot) == 0
                {
                    if status == 0 {
                        lman.state = LMAN_STATE_MS_CHANGE;
                        lman.next_state = LMAN_STATE_SEND_CHILD_NAME;
                        lman.work = 0x22;
                        lman.param[0] = lman.child_slot as u16;
                    } else {
                        lman.state = {
                            lman.next_state = LMAN_STATE_READY;
                            lman.next_state
                        };
                        lman.work = 0x23;
                        lman.param[0] = status as u16;
                        if lman.pcswitch_flag != 0 {
                            lman.pcswitch_flag = PCSWITCH_2ND_SP_START;
                            lman.state = LMAN_STATE_START_SEARCH_PARENT;
                        }
                    }
                    rfu_LMAN_occureCallback(lman.work as u8, 0x01);
                    lman.work = 0;
                }
            }
            ID_CPR_START_REQ => {
                if reqResult == 0 {
                    lman.param[0] = (*gRfuLinkStatus).linkLossSlotFlag as u16;
                    lman.state = {
                        lman.next_state = LMAN_STATE_POLL_LINK_RECOVERY;
                        lman.next_state
                    };
                    lman.child_slot = 0;
                    while lman.child_slot < RFU_CHILD_MAX {
                        if shr_i32(
                            (*gRfuLinkStatus).linkLossSlotFlag as i32,
                            lman.child_slot as u32,
                        ) & 1
                            != 0
                        {
                            break;
                        }
                        lman.child_slot += 1;
                    }
                }
            }
            ID_CPR_POLL_REQ => {
                if reqResult == 0
                    && rfu_CHILD_getConnectRecoveryStatus(&raw mut status) == 0
                    && status < 2
                {
                    lman.state = LMAN_STATE_END_LINK_RECOVERY;
                }
                if lman.linkRecoveryTimer.count[lman.child_slot] != 0
                    && ({
                        lman.linkRecoveryTimer.count[lman.child_slot] -= 1;
                        lman.linkRecoveryTimer.count[lman.child_slot]
                    }) == 0
                {
                    lman.state = LMAN_STATE_END_LINK_RECOVERY;
                }
            }
            ID_CPR_END_REQ => {
                if reqResult == 0 && rfu_CHILD_getConnectRecoveryStatus(&raw mut status) == 0 {
                    if status == 0 {
                        lman.state = LMAN_STATE_MS_CHANGE;
                        lman.next_state = LMAN_STATE_BACK_STATE;
                        lman.work = 0x32;
                    } else {
                        lman.state = {
                            lman.next_state = LMAN_STATE_READY;
                            lman.next_state
                        };
                        rfu_LMAN_disconnect((*gRfuLinkStatus).linkLossSlotFlag);
                        lman.work = 0x33;
                    }
                    lman.linkRecoveryTimer.count[lman.child_slot] = 0;
                    lman.linkRecoveryTimer.active = 0;
                    lman.linkRecovery_start_flag = 0;
                    rfu_LMAN_occureCallback(lman.work as u8, 0x01);
                    lman.work = 0;
                }
            }
            39 => {
                if reqResult == 0 {
                    if lman.next_state == LMAN_STATE_BACK_STATE {
                        lman.state = lman.state_bak[0];
                        lman.next_state = lman.state_bak[1];
                        volatile_write(
                            &raw mut lman.childClockSlave_flag,
                            RFU_CHILD_CLOCK_SLAVE_ON,
                        );
                        rfu_LMAN_occureCallback(LMAN_MSG_CHANGE_AGB_CLOCK_SLAVE, 0);
                    } else if lman.next_state == LMAN_STATE_SEND_CHILD_NAME {
                        lman.state = lman.next_state;
                        volatile_write(
                            &raw mut lman.childClockSlave_flag,
                            RFU_CHILD_CLOCK_SLAVE_ON,
                        );
                        rfu_LMAN_occureCallback(LMAN_MSG_CHANGE_AGB_CLOCK_SLAVE, 0);
                        lman.nameAcceptTimer.active |= shl_i32(1, lman.child_slot as u32) as u8;
                        lman.nameAcceptTimer.count[lman.child_slot] =
                            lman.nameAcceptTimer.count_max;
                        rfu_clearSlot(TYPE_NI_SEND, lman.child_slot);
                        status = rfu_NI_CHILD_setSendGameName(lman.child_slot, 0x0e) as u8;
                        if status != 0 {
                            lman.state = {
                                lman.next_state = LMAN_STATE_READY;
                                lman.next_state
                            };
                            rfu_LMAN_managerChangeAgbClockMaster();
                            rfu_LMAN_disconnect(
                                (*gRfuLinkStatus).connSlotFlag | (*gRfuLinkStatus).linkLossSlotFlag,
                            );
                            lman.param[0] = status as u16;
                            rfu_LMAN_occureCallback(
                                LMAN_MSG_CHILD_NAME_SEND_FAILED_AND_DISCONNECTED,
                                1,
                            );
                        }
                    }
                }
            }
            ID_STOP_MODE_REQ if reqResult == 0 => {
                lman.state = {
                    lman.next_state = LMAN_STATE_READY;
                    lman.next_state
                };
                rfu_LMAN_occureCallback(LMAN_MSG_RFU_POWER_DOWN, 0);
            }
            _ => {}
        }
        lman.active = 1;
    } else if reqResult == 3
        && lman.msc_exe_flag != 0
        && (reqCommandId == ID_DATA_TX_REQ as u16
            || reqCommandId == ID_DATA_RX_REQ
            || reqCommandId == ID_MS_CHANGE_REQ as u16)
    {
        rfu_REQ_RFUStatus();
        rfu_waitREQComplete();
        rfu_getRFUStatus(&raw mut status);
        if status == 0 && (*gRfuLinkStatus).parentChild == 0x00 {
            stwiRecvBuffer = rfu_getSTWIRecvBuffer().at(4);
            *({
                let t5 = stwiRecvBuffer;
                stwiRecvBuffer = stwiRecvBuffer.at(1);
                t5
            }) = (*gRfuLinkStatus).connSlotFlag;
            *stwiRecvBuffer = REASON_LINK_LOSS;
            rfu_LMAN_linkWatcher(ID_DISCONNECTED_AND_CHANGE_REQ as u16);
            reqResult = 0;
        }
    }
    match reqCommandId {
        ID_DISCONNECT_REQ => {
            if reqResult == 0 {
                lman.param[0] = *rfu_getSTWIRecvBuffer().at(8) as u16;
                rfu_LMAN_reflectCommunicationStatus(lman.param[0] as u8);
                if lman.linkRecoveryTimer.active != 0 {
                    lman.linkRecoveryTimer.active &= !(lman.param[0] as u8);
                    for i in 0..RFU_CHILD_MAX {
                        if shr_i32(lman.param[0] as i32, i as u32) & 1 != 0 {
                            lman.linkRecoveryTimer.count[i] = 0;
                        }
                    }
                    if lman.parent_child == MODE_CHILD {
                        lman.state = {
                            lman.next_state = LMAN_STATE_READY;
                            lman.next_state
                        };
                    }
                }
                status = lman.acceptSlot_flag & lman.param[0] as u8;
                for i in 0..RFU_CHILD_MAX {
                    if shr_i32(status as i32, i as u32) & 1 != 0 && lman.acceptCount != 0 {
                        lman.acceptCount -= 1;
                    }
                }
                lman.acceptSlot_flag &= !(lman.param[0] as u8);
                if lman.pcswitch_flag != 0 && (*gRfuLinkStatus).parentChild == MODE_NEUTRAL {
                    if lman.pcswitch_flag == PCSWITCH_SC_LOCK {
                        lman.connect_period = lman.pcswitch_period_bak;
                        lman.pcswitch_flag = PCSWITCH_3RD_SC;
                        lman.state = LMAN_STATE_POLL_SEARCH_CHILD;
                    } else if lman.state != LMAN_STATE_POLL_SEARCH_CHILD
                        && lman.state != LMAN_STATE_END_SEARCH_CHILD
                    {
                        lman.pcswitch_flag = PCSWITCH_1ST_SC_START;
                        lman.state = LMAN_STATE_START_SEARCH_CHILD;
                    }
                }
                if (*gRfuLinkStatus).parentChild == MODE_NEUTRAL && lman.state == LMAN_STATE_READY {
                    lman.parent_child = MODE_NEUTRAL;
                }
                if lman.active == 0 {
                    rfu_LMAN_occureCallback(LMAN_MSG_LINK_DISCONNECTED_BY_USER, 1);
                }
            }
        }
        ID_DATA_RX_REQ => {
            rfu_LMAN_CHILD_checkSendChildName2();
            if (*gRfuLinkStatus).parentChild != MODE_NEUTRAL {
                rfu_LMAN_occureCallback(LMAN_MSG_RECV_DATA_REQ_COMPLETED, 0);
            }
        }
        ID_RESET_REQ | ID_STOP_MODE_REQ if reqResult == 0 => {
            lman.reserveDisconnectSlot_flag = 0;
            lman.acceptCount = 0;
            lman.acceptSlot_flag = 0;
            lman.parent_child = MODE_NEUTRAL;
            rfu_LMAN_managerChangeAgbClockMaster();
            if reqCommandId == ID_STOP_MODE_REQ {
                rfu_LMAN_endManager();
            }
        }
        _ => {}
    }
    if reqResult != 0 {
        if reqCommandId == ID_SP_START_REQ
            && reqResult != 0
            && lman.pcswitch_flag == PCSWITCH_2ND_SP
        {
            (*gRfuLinkStatus).parentChild = MODE_PARENT;
            (*gRfuLinkStatus).connSlotFlag = 0xF;
            rfu_LMAN_disconnect(15);
            rfu_waitREQComplete();
            return;
        } else {
            lman.param[0] = reqCommandId;
            lman.param[1] = reqResult;
            if lman.active != 0 {
                lman.state = {
                    lman.next_state = LMAN_STATE_READY;
                    lman.next_state
                };
            }
            rfu_LMAN_occureCallback(LMAN_MSG_REQ_API_ERROR, 2);
            rfu_LMAN_managerChangeAgbClockMaster();
        }
    }
    if reqCommandId == ID_CLOCK_SLAVE_MS_CHANGE_ERROR_BY_DMA_REQ as u16 {
        rfu_LMAN_occureCallback(LMAN_MSG_CLOCK_SLAVE_MS_CHANGE_ERROR_BY_DMA, 0);
        rfu_LMAN_managerChangeAgbClockMaster();
    }
}
pub(crate) unsafe fn rfu_LMAN_MSC_callback(reqCommandId: u16) {
    let mut thisAck_flag: u8 = 0;
    let active_bak: u8 = lman.active;
    lman.active = 0;
    lman.msc_exe_flag = 1;
    if (*gRfuLinkStatus).parentChild == MODE_CHILD {
        rfu_LMAN_linkWatcher(reqCommandId);
        if (&raw mut lman.childClockSlave_flag).read_volatile() != RFU_CHILD_CLOCK_SLAVE_ON {
            rfu_LMAN_managerChangeAgbClockMaster();
            lman.msc_exe_flag = 0;
            lman.active = active_bak;
            return;
        }
    } else {
        if rfu_UNI_PARENT_getDRAC_ACK(&raw mut thisAck_flag) == 0 {
            volatile_write(
                &raw mut lman.parentAck_flag,
                (&raw mut lman.parentAck_flag).read_volatile() | thisAck_flag,
            );
        }
    }
    if lman.MSC_callback.is_some() {
        lman.MSC_callback.unwrap_unchecked()(reqCommandId);
        rfu_waitREQComplete();
        if (&raw mut lman.childClockSlave_flag).read_volatile() == RFU_CHILD_CLOCK_SLAVE_OFF_REQ {
            rfu_LMAN_managerChangeAgbClockMaster();
        }
    }
    lman.msc_exe_flag = 0;
    lman.active = active_bak;
}
unsafe fn rfu_LMAN_PARENT_checkRecvChildName() {
    let mut newSlot: u8 = 0;
    let mut newAcceptSlot: u8 = 0;
    let mut flags: u8 = 0;
    let mut tgtSlot: u8 = 0;
    let mut ptr: *mut u16 = null_mut();
    if lman.state == LMAN_STATE_START_SEARCH_CHILD
        || lman.state == LMAN_STATE_POLL_SEARCH_CHILD
        || lman.state == LMAN_STATE_END_SEARCH_CHILD
        || lman.state == LMAN_STATE_WAIT_RECV_CHILD_NAME
    {
        newSlot = ((*gRfuLinkStatus).connSlotFlag ^ lman.connectSlot_flag_old)
            & (*gRfuLinkStatus).connSlotFlag
            & !(*gRfuLinkStatus).getNameFlag;
        lman.connectSlot_flag_old = (*gRfuLinkStatus).connSlotFlag;
        if newSlot != 0 {
            lman.param[0] = newSlot as u16;
            rfu_LMAN_occureCallback(LMAN_MSG_NEW_CHILD_CONNECT_DETECTED, 1);
        }
        newAcceptSlot = 0x00;
        for i in 0..RFU_CHILD_MAX {
            tgtSlot = shl_i32(1, i as u32) as u8;
            flags = 0x00;
            if newSlot as i32 & tgtSlot as i32 != 0 {
                lman.nameAcceptTimer.count[i] = lman.nameAcceptTimer.count_max;
                lman.nameAcceptTimer.active |= tgtSlot;
            } else if lman.nameAcceptTimer.active as i32 & tgtSlot as i32 != 0 {
                if (*gRfuSlotStatusNI[i]).recv.state == SLOT_STATE_RECV_SUCCESS {
                    if (*gRfuSlotStatusNI[i]).recv.dataType == 1 {
                        flags = RN_NAME_TIMER_CLEAR;
                        ptr = lman.acceptable_serialNo_list;
                        while *ptr != 0xFFFF {
                            if (*gRfuLinkStatus).partner[i].serialNo == *ptr {
                                lman.acceptSlot_flag |= tgtSlot;
                                lman.acceptCount += 1;
                                newAcceptSlot |= tgtSlot;
                                flags |= RN_ACCEPT;
                                break;
                            }
                            ptr = ptr.at(1);
                        }
                        if flags as i32 & RN_ACCEPT as i32 == 0 {
                            flags |= RN_DISCONNECT;
                        }
                    }
                } else if ({
                    lman.nameAcceptTimer.count[i] -= 1;
                    lman.nameAcceptTimer.count[i]
                }) == 0
                {
                    flags = 6;
                }
                if flags as i32 & RN_NAME_TIMER_CLEAR as i32 != 0 {
                    lman.nameAcceptTimer.active &= !tgtSlot;
                    lman.nameAcceptTimer.count[i] = 0;
                    rfu_clearSlot(TYPE_NI_RECV, i);
                }
                if flags as i32 & RN_DISCONNECT as i32 != 0 {
                    lman.reserveDisconnectSlot_flag |= tgtSlot;
                }
            }
        }
        if newAcceptSlot != 0 {
            lman.param[0] = newAcceptSlot as u16;
            rfu_LMAN_occureCallback(LMAN_MSG_NEW_CHILD_CONNECT_ACCEPTED, 1);
        }
        if lman.reserveDisconnectSlot_flag != 0 {
            flags = 1;
            if (*gRfuLinkStatus).sendSlotUNIFlag != 0
                && (&raw mut lman.parentAck_flag).read_volatile() as i32
                    & lman.acceptSlot_flag as i32
                    != lman.acceptSlot_flag as i32
            {
                flags = 0;
            }
            if flags != 0 {
                rfu_LMAN_disconnect(lman.reserveDisconnectSlot_flag);
                lman.param[0] = lman.reserveDisconnectSlot_flag as u16;
                lman.reserveDisconnectSlot_flag = 0;
                rfu_LMAN_occureCallback(LMAN_MSG_NEW_CHILD_CONNECT_REJECTED, 1);
            }
        }
        if lman.nameAcceptTimer.active == 0 && lman.state == LMAN_STATE_WAIT_RECV_CHILD_NAME {
            if lman.pcswitch_flag == 0 {
                lman.state = {
                    lman.next_state = LMAN_STATE_READY;
                    lman.next_state
                };
                rfu_LMAN_occureCallback(LMAN_MSG_END_WAIT_CHILD_NAME, 0);
            } else {
                if lman.pcswitch_flag == PCSWITCH_1ST_SC {
                    lman.pcswitch_flag = PCSWITCH_2ND_SP_START;
                    lman.state = LMAN_STATE_START_SEARCH_PARENT;
                } else {
                    lman.pcswitch_flag = PCSWITCH_1ST_SC_START;
                    lman.state = LMAN_STATE_START_SEARCH_CHILD;
                }
                if lman.acceptSlot_flag != 0 {
                    lman.connect_period = 0;
                    lman.pcswitch_flag = PCSWITCH_SC_LOCK;
                    lman.state = LMAN_STATE_START_SEARCH_CHILD;
                }
            }
        }
    }
}
unsafe fn rfu_LMAN_CHILD_checkSendChildName() {
    let imeBak: u16 = (67109384_usize as *mut u16).read_volatile();
    volatile_write(67109384_usize as *mut u16, 0);
    if lman.state == LMAN_STATE_SEND_CHILD_NAME
        && (({
            lman.nameAcceptTimer.count[lman.child_slot] -= 1;
            lman.nameAcceptTimer.count[lman.child_slot]
        }) == 0
            || (*gRfuSlotStatusNI[lman.child_slot]).send.state == SLOT_STATE_SEND_FAILED)
    {
        rfu_LMAN_requestChangeAgbClockMaster();
        lman.state = LMAN_STATE_WAIT_CHANGE_CLOCK_MASTER;
        rfu_clearSlot(TYPE_NI_SEND, lman.child_slot);
        lman.nameAcceptTimer.active &= !(shl_i32(1, lman.child_slot as u32) as u8);
        lman.nameAcceptTimer.count[lman.child_slot] = 0;
    }
    volatile_write(67109384_usize as *mut u16, imeBak);
    if lman.state == LMAN_STATE_WAIT_CHANGE_CLOCK_MASTER {
        if (&raw mut lman.childClockSlave_flag).read_volatile() == RFU_CHILD_CLOCK_SLAVE_ON {
            rfu_LMAN_requestChangeAgbClockMaster();
        }
        if (&raw mut lman.childClockSlave_flag).read_volatile() == RFU_CHILD_CLOCK_SLAVE_OFF {
            lman.state = {
                lman.next_state = LMAN_STATE_READY;
                lman.next_state
            };
            rfu_LMAN_disconnect(
                (*gRfuLinkStatus).connSlotFlag | (*gRfuLinkStatus).linkLossSlotFlag,
            );
            lman.param[0] = 0;
            rfu_LMAN_occureCallback(LMAN_MSG_CHILD_NAME_SEND_FAILED_AND_DISCONNECTED, 1);
        }
    }
}
unsafe fn rfu_LMAN_CHILD_checkSendChildName2() {
    if lman.state == LMAN_STATE_SEND_CHILD_NAME
        && (*gRfuSlotStatusNI[lman.child_slot]).send.state == SLOT_STATE_SEND_SUCCESS
    {
        lman.state = {
            lman.next_state = LMAN_STATE_READY;
            lman.next_state
        };
        rfu_clearSlot(TYPE_NI_SEND, lman.child_slot);
        lman.nameAcceptTimer.active &= !(shl_i32(1, lman.child_slot as u32) as u8);
        lman.nameAcceptTimer.count[lman.child_slot] = 0;
        rfu_LMAN_occureCallback(LMAN_MSG_CHILD_NAME_SEND_COMPLETED, 0);
    }
}
unsafe fn rfu_LMAN_CHILD_linkRecoveryProcess() {
    if lman.parent_child == MODE_CHILD && lman.linkRecovery_start_flag == LINK_RECOVERY_START {
        lman.state_bak[0] = lman.state;
        lman.state_bak[1] = lman.next_state;
        lman.state = LMAN_STATE_START_LINK_RECOVERY;
        lman.next_state = LMAN_STATE_POLL_LINK_RECOVERY;
        lman.linkRecovery_start_flag = LINK_RECOVERY_EXE;
    }
}
unsafe fn rfu_LMAN_CHILD_checkEnableParentCandidate() -> u8 {
    let mut serialNo: *mut u16 = null_mut();
    let mut flags: u8 = 0x00;
    let mut i: u8 = 0;
    while i < (*gRfuLinkStatus).findParentCount {
        serialNo = lman.acceptable_serialNo_list;
        while *serialNo != 0xFFFF {
            if (*gRfuLinkStatus).partner[i].serialNo == *serialNo {
                flags |= shl_i32(1, i as u32) as u8;
            }
            serialNo = serialNo.at(1);
        }
        i += 1;
    }
    flags
}
unsafe fn rfu_LMAN_occureCallback(msg: u8, param_count: u8) {
    if lman.LMAN_callback.is_some() {
        lman.LMAN_callback.unwrap_unchecked()(msg, param_count);
    }
    lman.param[0] = {
        lman.param[1] = 0;
        lman.param[1]
    };
}
unsafe fn rfu_LMAN_disconnect(bm_disconnectedSlot: u8) {
    let active_bak: u8 = lman.active;
    lman.active = 1;
    rfu_REQ_disconnect(bm_disconnectedSlot);
    rfu_waitREQComplete();
    lman.active = active_bak;
}
unsafe fn rfu_LMAN_reflectCommunicationStatus(bm_disconnectedSlot: u8) {
    if (*gRfuLinkStatus).sendSlotNIFlag != 0 {
        for i in 0..RFU_CHILD_MAX {
            if (*gRfuSlotStatusNI[i]).send.state as i32 & SLOT_BUSY_FLAG != 0
                && (*gRfuSlotStatusNI[i]).send.bmSlot as i32 & bm_disconnectedSlot as i32 != 0
            {
                rfu_changeSendTarget(
                    TYPE_NI,
                    i,
                    (*gRfuSlotStatusNI[i]).send.bmSlot & !bm_disconnectedSlot,
                );
            }
        }
    }
    if (*gRfuLinkStatus).recvSlotNIFlag != 0 {
        for i in 0..RFU_CHILD_MAX {
            if (*gRfuSlotStatusNI[i]).recv.state as i32 & SLOT_BUSY_FLAG != 0
                && (*gRfuSlotStatusNI[i]).recv.bmSlot as i32 & bm_disconnectedSlot as i32 != 0
            {
                rfu_NI_stopReceivingData(i);
            }
        }
    }
    if (*gRfuLinkStatus).sendSlotUNIFlag != 0 {
        (*gRfuLinkStatus).sendSlotUNIFlag &= !bm_disconnectedSlot;
        for i in 0..RFU_CHILD_MAX {
            if (*gRfuSlotStatusUNI[i]).send.state == SLOT_STATE_SEND_UNI
                && bm_disconnectedSlot as i32 & (*gRfuSlotStatusUNI[i]).send.bmSlot as i32 != 0
            {
                (*gRfuSlotStatusUNI[i]).send.bmSlot &= !bm_disconnectedSlot;
            }
        }
    }
}
unsafe fn rfu_LMAN_checkNICommunicateStatus() {
    let mut flags: u8 = 0;
    if lman.NI_failCounter_limit != 0 {
        if (*gRfuLinkStatus).sendSlotNIFlag != 0 {
            for i in 0..RFU_CHILD_MAX {
                if (*gRfuSlotStatusNI[i]).send.state as i32 & SLOT_BUSY_FLAG != 0 {
                    flags = 0;
                    for j in 0..RFU_CHILD_MAX {
                        if shr_i32((*gRfuSlotStatusNI[i]).send.bmSlot as i32, j as u32) & 1 != 0
                            && (*gRfuSlotStatusNI[j]).send.failCounter > lman.NI_failCounter_limit
                        {
                            flags |= shl_i32(1, j as u32) as u8;
                        }
                        if flags != 0 {
                            rfu_changeSendTarget(
                                TYPE_NI,
                                i,
                                flags ^ (*gRfuSlotStatusNI[i]).send.bmSlot,
                            );
                        }
                    }
                }
            }
        }
        if (*gRfuLinkStatus).recvSlotNIFlag != 0 {
            for i in 0..RFU_CHILD_MAX {
                if (*gRfuSlotStatusNI[i]).recv.state as i32 & SLOT_BUSY_FLAG != 0
                    && (*gRfuSlotStatusNI[i]).recv.failCounter > lman.NI_failCounter_limit
                {
                    rfu_NI_stopReceivingData(i);
                }
            }
        }
    }
}
pub unsafe fn rfu_LMAN_setMSCCallback(MSC_callback_p: Option<unsafe fn(u16)>) {
    lman.MSC_callback = MSC_callback_p;
    rfu_setMSCCallback(Some(rfu_LMAN_MSC_callback));
}
unsafe fn rfu_LMAN_setLMANCallback(func: Option<unsafe fn(u8, u8)>) {
    lman.LMAN_callback = func;
}
pub unsafe fn rfu_LMAN_setLinkRecovery(enable_flag: u8, recovery_period: u16) -> u8 {
    if lman.linkRecovery_enable != 0 && enable_flag == 0 && lman.linkRecoveryTimer.active != 0 {
        return LMAN_ERROR_NOW_LINK_RECOVERY;
    }
    let imeBak: u16 = (67109384_usize as *mut u16).read_volatile();
    volatile_write(67109384_usize as *mut u16, 0);
    lman.linkRecovery_enable = enable_flag;
    lman.linkRecoveryTimer.count_max = recovery_period;
    volatile_write(67109384_usize as *mut u16, imeBak);
    0
}
unsafe fn rfu_LMAN_setNIFailCounterLimit(NI_failCounter_limit: u16) -> u8 {
    if (*gRfuLinkStatus).sendSlotNIFlag as i32 | (*gRfuLinkStatus).recvSlotNIFlag as i32 != 0 {
        lman.param[0] = 6;
        rfu_LMAN_occureCallback(LMAN_MSG_LMAN_API_ERROR_RETURN, 1);
        return LMAN_ERROR_NOW_COMMUNICATION;
    }
    lman.NI_failCounter_limit = NI_failCounter_limit;
    0
}
unsafe fn rfu_LMAN_setFastSearchParent(enable_flag: u8) -> u8 {
    if lman.state == LMAN_STATE_START_SEARCH_PARENT
        || lman.state == LMAN_STATE_POLL_SEARCH_PARENT
        || lman.state == LMAN_STATE_END_SEARCH_PARENT
    {
        lman.param[0] = 7;
        rfu_LMAN_occureCallback(LMAN_MSG_LMAN_API_ERROR_RETURN, 1);
        return LMAN_ERROR_NOW_SEARCH_PARENT;
    }
    if enable_flag != 0 {
        lman.fastSearchParent_flag = FSP_ON;
    } else {
        lman.fastSearchParent_flag = 0;
    }
    0
}
unsafe fn rfu_LMAN_managerChangeAgbClockMaster() {
    if (&raw mut lman.childClockSlave_flag).read_volatile() != RFU_CHILD_CLOCK_SLAVE_OFF {
        volatile_write(
            &raw mut lman.childClockSlave_flag,
            RFU_CHILD_CLOCK_SLAVE_OFF,
        );
        rfu_LMAN_occureCallback(LMAN_MSG_CHANGE_AGB_CLOCK_MASTER, 0);
    }
}
pub unsafe fn rfu_LMAN_requestChangeAgbClockMaster() {
    if (&raw mut lman.childClockSlave_flag).read_volatile() == RFU_CHILD_CLOCK_SLAVE_OFF {
        rfu_LMAN_occureCallback(LMAN_MSG_CHANGE_AGB_CLOCK_MASTER, 0);
    } else if (&raw mut lman.childClockSlave_flag).read_volatile() == RFU_CHILD_CLOCK_SLAVE_ON {
        volatile_write(
            &raw mut lman.childClockSlave_flag,
            RFU_CHILD_CLOCK_SLAVE_OFF_REQ,
        );
    }
}
pub unsafe fn rfu_LMAN_forceChangeSP() {
    if lman.pcswitch_flag != 0 {
        match lman.state {
            LMAN_STATE_START_SEARCH_CHILD => {
                lman.pcswitch_flag = PCSWITCH_2ND_SP_START;
                lman.state = LMAN_STATE_START_SEARCH_PARENT;
            }
            LMAN_STATE_POLL_SEARCH_CHILD => {
                lman.pcswitch_flag = PCSWITCH_1ST_SC;
                lman.connect_period = 1;
            }
            LMAN_STATE_END_SEARCH_CHILD | LMAN_STATE_WAIT_RECV_CHILD_NAME => {
                lman.pcswitch_flag = PCSWITCH_1ST_SC;
            }
            LMAN_STATE_START_SEARCH_PARENT | LMAN_STATE_POLL_SEARCH_PARENT => {
                lman.connect_period = PCSWITCH_SP_PERIOD;
            }
            LMAN_STATE_END_SEARCH_PARENT => {
                lman.connect_period = PCSWITCH_SP_PERIOD;
                lman.state = LMAN_STATE_POLL_SEARCH_PARENT;
            }
            _ => {}
        }
    }
}
