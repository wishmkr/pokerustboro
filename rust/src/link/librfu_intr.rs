//! Translated from `src/librfu_intr.c` by tools/rustport/c2rs.py.
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
    clippy::type_complexity,
    improper_ctypes_definitions,
    unused_assignments
)]

#[allow(unused_imports)]
use crate::c::*;
#[allow(unused_imports)]
use crate::consts::*;
use crate::librfu_stwi::gSTWIStatus;
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use core::ffi::c_void;
#[allow(unused_imports)]
use core::mem::zeroed;
#[allow(unused_imports)]
use core::ptr::null_mut;

#[cfg_attr(target_arch = "arm", instruction_set(arm::a32))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IntrSIO32() {
    if (&raw mut (*gSTWIStatus).state).read_volatile() == 10 {
        if (*gSTWIStatus).callbackID.is_some() {
            Callback_Dummy_ID((*gSTWIStatus).callbackID);
        }
    } else {
        if (&raw mut (*gSTWIStatus).msMode).read_volatile() == AGB_CLK_MASTER {
            sio32intr_clock_master();
        } else {
            sio32intr_clock_slave();
        }
    }
}
#[cfg_attr(target_arch = "arm", instruction_set(arm::a32))]
unsafe fn sio32intr_clock_master() {
    let mut ackLen: u32 = 0;
    STWI_set_timer_in_RAM(80);
    let regSIODATA32: u32 = (67109152_usize as *mut u32).read_volatile();
    if (&raw mut (*gSTWIStatus).state).read_volatile() == 0 {
        if regSIODATA32 == 0x80000000 {
            if (*gSTWIStatus).reqNext <= (*gSTWIStatus).reqLength {
                volatile_write(
                    67109152_usize as *mut u32,
                    *((*(*gSTWIStatus).txPacket).rfuPacket8.data.as_mut_ptr() as *mut u32)
                        .at((*gSTWIStatus).reqNext),
                );
                (*gSTWIStatus).reqNext += 1;
            } else {
                volatile_write(&raw mut (*gSTWIStatus).state, 1);
                volatile_write(67109152_usize as *mut u32, 0x80000000);
            }
        } else {
            STWI_stop_timer_in_RAM();
            STWI_set_timer_in_RAM(130);
            return;
        }
    } else if (&raw mut (*gSTWIStatus).state).read_volatile() == 1 {
        if regSIODATA32 & 0xFFFF0000 == 0x99660000 {
            (*gSTWIStatus).ackNext = 0;
            *((*gSTWIStatus).rxPacket as *mut u32).at((*gSTWIStatus).ackNext) = regSIODATA32;
            (*gSTWIStatus).ackNext += 1;
            (*gSTWIStatus).ackActiveCommand = regSIODATA32 as u8;
            (*gSTWIStatus).ackLength = ({
                ackLen = regSIODATA32 >> 8;
                ackLen
            }) as u8;
            if ({
                ackLen = (*gSTWIStatus).ackLength as u32;
                ackLen
            }) >= (*gSTWIStatus).ackNext as u32
            {
                volatile_write(&raw mut (*gSTWIStatus).state, 2);
                volatile_write(67109152_usize as *mut u32, 0x80000000);
            } else {
                volatile_write(&raw mut (*gSTWIStatus).state, 3);
            }
        } else {
            STWI_stop_timer_in_RAM();
            STWI_set_timer_in_RAM(130);
            return;
        }
    } else if (&raw mut (*gSTWIStatus).state).read_volatile() == 2 {
        *((*gSTWIStatus).rxPacket as *mut u32).at((*gSTWIStatus).ackNext) = regSIODATA32;
        (*gSTWIStatus).ackNext += 1;
        if (*gSTWIStatus).ackLength < (*gSTWIStatus).ackNext {
            volatile_write(&raw mut (*gSTWIStatus).state, 3);
        } else {
            volatile_write(67109152_usize as *mut u32, 0x80000000);
        }
    }
    if handshake_wait(1) == 1 {
        return;
    }
    volatile_write(67109160_usize as *mut u16, 20491);
    if handshake_wait(0) == 1 {
        return;
    }
    STWI_stop_timer_in_RAM();
    if (&raw mut (*gSTWIStatus).state).read_volatile() == 3 {
        if (*gSTWIStatus).ackActiveCommand == 167
            || (*gSTWIStatus).ackActiveCommand == 165
            || (*gSTWIStatus).ackActiveCommand == 181
            || (*gSTWIStatus).ackActiveCommand == 183
        {
            volatile_write(&raw mut (*gSTWIStatus).msMode, AGB_CLK_SLAVE);
            volatile_write(67109152_usize as *mut u32, 0x80000000);
            volatile_write(67109160_usize as *mut u16, 20482);
            volatile_write(67109160_usize as *mut u16, 20610);
            volatile_write(&raw mut (*gSTWIStatus).state, 5);
        } else {
            if (*gSTWIStatus).ackActiveCommand == 0xEE {
                volatile_write(67109160_usize as *mut u16, 20483);
                volatile_write(&raw mut (*gSTWIStatus).state, 4);
                volatile_write(&raw mut (*gSTWIStatus).error, ERR_REQ_CMD_ACK_REJECTION);
            } else {
                volatile_write(67109160_usize as *mut u16, 20483);
                volatile_write(&raw mut (*gSTWIStatus).state, 4);
            }
        }
        volatile_write(&raw mut (*gSTWIStatus).sending, 0);
        if (*gSTWIStatus).callbackM.is_some() {
            Callback_Dummy_M(
                (*gSTWIStatus).reqActiveCommand as i32,
                (&raw mut (*gSTWIStatus).error).read_volatile() as i32,
                (*gSTWIStatus).callbackM,
            );
        }
    } else {
        volatile_write(67109160_usize as *mut u16, 20483);
        volatile_write(67109160_usize as *mut u16, 20611);
    }
}
#[cfg_attr(target_arch = "arm", instruction_set(arm::a32))]
unsafe fn sio32intr_clock_slave() {
    let mut r0: u32 = 0;
    let mut reqLen: u32 = 0;
    volatile_write(&raw mut (*gSTWIStatus).timerActive, 0);
    STWI_set_timer_in_RAM(100);
    if handshake_wait(0) == 1 {
        return;
    }
    volatile_write(67109160_usize as *mut u16, 20490);
    let regSIODATA32: u32 = (67109152_usize as *mut u32).read_volatile();
    if (&raw mut (*gSTWIStatus).state).read_volatile() == 5 {
        *((*gSTWIStatus).rxPacket as *mut u32) = regSIODATA32;
        (*gSTWIStatus).reqNext = 1;
        r0 = 0x99660000;
        reqLen = regSIODATA32 >> 16;
        if reqLen == r0 >> 16 {
            (*gSTWIStatus).reqLength = ({
                reqLen = regSIODATA32 >> 8;
                reqLen
            }) as u8;
            (*gSTWIStatus).reqActiveCommand = ({
                reqLen = regSIODATA32;
                reqLen
            }) as u8;
            if (*gSTWIStatus).reqLength == 0 {
                if (*gSTWIStatus).reqActiveCommand == ID_MS_CHANGE_REQ
                    || (*gSTWIStatus).reqActiveCommand == ID_DATA_READY_AND_CHANGE_REQ
                    || (*gSTWIStatus).reqActiveCommand == ID_DISCONNECTED_AND_CHANGE_REQ
                    || (*gSTWIStatus).reqActiveCommand == ID_UNK36_REQ
                {
                    (*gSTWIStatus).ackActiveCommand = (*gSTWIStatus).reqActiveCommand + 0x80;
                    *((*gSTWIStatus).txPacket as *mut u32) =
                        0x99660000 + (*gSTWIStatus).ackActiveCommand as u32;
                    (*gSTWIStatus).ackLength = 0;
                } else {
                    *((*gSTWIStatus).txPacket as *mut u32) = 0x996601EE;
                    if (*gSTWIStatus).reqActiveCommand >= 0x10
                        && (*gSTWIStatus).reqActiveCommand <= 0x3D
                    {
                        *((*gSTWIStatus).txPacket as *mut u32).at(1) = 1;
                    } else {
                        *((*gSTWIStatus).txPacket as *mut u32).at(1) = 2;
                    }
                    (*gSTWIStatus).ackLength = 1;
                    volatile_write(&raw mut (*gSTWIStatus).error, ERR_REQ_CMD_ACK_REJECTION);
                }
                volatile_write(
                    67109152_usize as *mut u32,
                    *((*gSTWIStatus).txPacket as *mut u32),
                );
                (*gSTWIStatus).ackNext = 1;
                volatile_write(&raw mut (*gSTWIStatus).state, 7);
            } else {
                volatile_write(67109152_usize as *mut u32, 0x80000000);
                (*gSTWIStatus).reqNext = 1;
                volatile_write(&raw mut (*gSTWIStatus).state, 6);
            }
        } else {
            STWI_stop_timer_in_RAM();
            STWI_set_timer_in_RAM(100);
            return;
        }
    } else if (&raw mut (*gSTWIStatus).state).read_volatile() == 6 {
        *((*gSTWIStatus).rxPacket as *mut u32).at((*gSTWIStatus).reqNext) = regSIODATA32;
        (*gSTWIStatus).reqNext += 1;
        if (*gSTWIStatus).reqLength < (*gSTWIStatus).reqNext {
            if (*gSTWIStatus).reqActiveCommand == ID_DATA_READY_AND_CHANGE_REQ
                || (*gSTWIStatus).reqActiveCommand == ID_DISCONNECTED_AND_CHANGE_REQ
                || (*gSTWIStatus).reqActiveCommand == ID_UNK36_REQ
            {
                (*gSTWIStatus).ackActiveCommand = (*gSTWIStatus).reqActiveCommand + 0x80;
                *((*gSTWIStatus).txPacket as *mut u32) =
                    0x99660000 | (*gSTWIStatus).ackActiveCommand as u32;
                (*gSTWIStatus).ackLength = 0;
            } else {
                *((*gSTWIStatus).txPacket as *mut u32) = 0x996601EE;
                if (*gSTWIStatus).reqActiveCommand >= 0x10
                    && (*gSTWIStatus).reqActiveCommand <= 0x3D
                {
                    *((*gSTWIStatus).txPacket as *mut u32).at(1) = 1;
                } else {
                    *((*gSTWIStatus).txPacket as *mut u32).at(1) = 2;
                }
                (*gSTWIStatus).ackLength = 1;
                volatile_write(&raw mut (*gSTWIStatus).error, ERR_REQ_CMD_ACK_REJECTION);
            }
            volatile_write(
                67109152_usize as *mut u32,
                *((*gSTWIStatus).txPacket as *mut u32),
            );
            (*gSTWIStatus).ackNext = 1;
            volatile_write(&raw mut (*gSTWIStatus).state, 7);
        } else {
            volatile_write(67109152_usize as *mut u32, 0x80000000);
        }
    } else if (&raw mut (*gSTWIStatus).state).read_volatile() == 7 {
        if regSIODATA32 == 0x80000000 {
            if (*gSTWIStatus).ackLength < (*gSTWIStatus).ackNext {
                volatile_write(&raw mut (*gSTWIStatus).state, 8);
            } else {
                volatile_write(
                    67109152_usize as *mut u32,
                    *((*gSTWIStatus).txPacket as *mut u32).at((*gSTWIStatus).ackNext),
                );
                (*gSTWIStatus).ackNext += 1;
            }
        } else {
            STWI_stop_timer_in_RAM();
            STWI_set_timer_in_RAM(100);
            return;
        }
    }
    if handshake_wait(1) == 1 {
        return;
    }
    if (&raw mut (*gSTWIStatus).state).read_volatile() == 8 {
        volatile_write(67109160_usize as *mut u16, 20482);
        STWI_stop_timer_in_RAM();
        if (&raw mut (*gSTWIStatus).error).read_volatile() == ERR_REQ_CMD_ACK_REJECTION {
            STWI_init_slave();
            if (*gSTWIStatus).callbackS.is_some() {
                Callback_Dummy_S(0x1EE, (*gSTWIStatus).callbackS);
            }
        } else {
            volatile_write(67109152_usize as *mut u32, 0);
            volatile_write(67109160_usize as *mut u16, 0);
            volatile_write(67109160_usize as *mut u16, 20483);
            volatile_write(&raw mut (*gSTWIStatus).msMode, AGB_CLK_MASTER);
            volatile_write(&raw mut (*gSTWIStatus).state, 0);
            if (*gSTWIStatus).callbackS.is_some() {
                Callback_Dummy_S(
                    ((*gSTWIStatus).reqLength as u16) << 8 | (*gSTWIStatus).reqActiveCommand as u16,
                    (*gSTWIStatus).callbackS,
                );
            }
        }
    } else {
        volatile_write(67109384_usize as *mut u16, 0);
        if (67109122_usize as *mut u16).read_volatile() as i32 & TIMER_ENABLE as i32 != 0 {
            if (67109122_usize as *mut u16).read_volatile() as i32 & 0x03 == TIMER_1CLK {
                while (0x4000100_usize as *mut u16).read_volatile() > 0xFF9B {}
            } else {
                while (0x4000100_usize as *mut u16).read_volatile() > 0xFFFE {}
            }
        }
        volatile_write(67109160_usize as *mut u16, 20482);
        volatile_write(67109160_usize as *mut u16, 20610);
        volatile_write(67109384_usize as *mut u16, 1);
    }
}
#[cfg_attr(target_arch = "arm", instruction_set(arm::a32))]
unsafe fn handshake_wait(slot: u16) -> u16 {
    loop {
        if (&raw mut (*gSTWIStatus).timerActive).read_volatile() as i32 & 0xFF == 1 {
            volatile_write(&raw mut (*gSTWIStatus).timerActive, 0);
            return 1;
        }
        if (67109160_usize as *mut u16).read_volatile() as i32 & SIO_MULTI_SI == (slot as i32) << 2
        {
            break;
        }
    }
    0
}
#[cfg_attr(target_arch = "arm", instruction_set(arm::a32))]
unsafe fn STWI_set_timer_in_RAM(count: u8) {
    let regTMCNTL: *mut u16 =
        (0x4000100 + (*gSTWIStatus).timerSelect as i32 * 4) as usize as *mut u16;
    let regTMCNTH: *mut u16 =
        (67109122 + (*gSTWIStatus).timerSelect as i32 * 4) as usize as *mut u16;
    volatile_write(67109384_usize as *mut u16, 0);
    match count {
        50 => {
            volatile_write(regTMCNTL, 0xFCCB);
            (*gSTWIStatus).timerState = 1;
        }
        80 => {
            volatile_write(regTMCNTL, 0xFAE0);
            (*gSTWIStatus).timerState = 2;
        }
        100 => {
            volatile_write(regTMCNTL, 0xF996);
            (*gSTWIStatus).timerState = 3;
        }
        130 => {
            volatile_write(regTMCNTL, 0xF7AD);
            (*gSTWIStatus).timerState = 4;
        }
        _ => {}
    }
    volatile_write(regTMCNTH, 195);
    volatile_write(
        67109378_usize as *mut u16,
        shl_i32(INTR_FLAG_TIMER0, (*gSTWIStatus).timerSelect as u32) as u16,
    );
    volatile_write(67109384_usize as *mut u16, 1);
}
#[cfg_attr(target_arch = "arm", instruction_set(arm::a32))]
unsafe fn STWI_stop_timer_in_RAM() {
    (*gSTWIStatus).timerState = 0;
    volatile_write(
        (0x4000100 + (*gSTWIStatus).timerSelect as i32 * 4) as usize as *mut u16,
        0,
    );
    volatile_write(
        (67109122 + (*gSTWIStatus).timerSelect as i32 * 4) as usize as *mut u16,
        0,
    );
}
#[cfg_attr(target_arch = "arm", instruction_set(arm::a32))]
unsafe fn STWI_init_slave() {
    volatile_write(&raw mut (*gSTWIStatus).state, 5);
    volatile_write(&raw mut (*gSTWIStatus).msMode, AGB_CLK_SLAVE);
    (*gSTWIStatus).reqLength = 0;
    (*gSTWIStatus).reqNext = 0;
    (*gSTWIStatus).reqActiveCommand = 0;
    (*gSTWIStatus).ackLength = 0;
    (*gSTWIStatus).ackNext = 0;
    (*gSTWIStatus).ackActiveCommand = 0;
    (*gSTWIStatus).timerState = 0;
    volatile_write(&raw mut (*gSTWIStatus).timerActive, 0);
    volatile_write(&raw mut (*gSTWIStatus).error, 0);
    (*gSTWIStatus).recoveryCount = 0;
    volatile_write(67109160_usize as *mut u16, 20610);
}
// hand-written: tools/rustport/overrides/librfu_intr/Callback_Dummy_M.rs
/// `Callback_Dummy_M`: a naked `bx r2` in C, i.e. a tail call of `callbackM`
/// with this function's own arguments still in r0-r2.
unsafe extern "C" fn Callback_Dummy_M(
    req_command_id: i32,
    error: i32,
    callback_m: Option<unsafe fn()>,
) {
    unsafe {
        let f: unsafe fn(i32, i32, Option<unsafe fn()>) =
            core::mem::transmute(callback_m.unwrap_unchecked());
        f(req_command_id, error, callback_m)
    }
}

// hand-written: tools/rustport/overrides/librfu_intr/Callback_Dummy_S.rs
/// `Callback_Dummy_S`: a naked `bx r1`, i.e. `callbackS(reqCommandId)`.
unsafe extern "C" fn Callback_Dummy_S(req_command_id: u16, callback_s: Option<unsafe fn(u16)>) {
    unsafe {
        let f: unsafe fn(u16, Option<unsafe fn(u16)>) =
            core::mem::transmute(callback_s.unwrap_unchecked());
        f(req_command_id, callback_s)
    }
}

// hand-written: tools/rustport/overrides/librfu_intr/Callback_Dummy_ID.rs
/// `Callback_Dummy_ID`: a naked `bx r0`, i.e. `callbackId()`.
unsafe extern "C" fn Callback_Dummy_ID(callback_id: Option<unsafe fn()>) {
    unsafe {
        let f: unsafe fn(Option<unsafe fn()>) =
            core::mem::transmute(callback_id.unwrap_unchecked());
        f(callback_id)
    }
}
