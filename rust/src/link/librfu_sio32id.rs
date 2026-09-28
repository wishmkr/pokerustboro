//! Translated from `src/librfu_sio32id.c` by tools/rustport/c2rs.py.
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
// Data tables (translate with cdata.py): Sio32ConnectionData Sio32IDLib_Var

/// `struct RfuSIO32Id`
#[repr(C, align(4))]
#[derive(Clone, Copy)]
pub struct RfuSIO32Id {
    pub MS_mode: u8,
    pub state: u8,
    pub count: u16,
    pub send_id: u16,
    pub recv_id: u16,
    pub unk8: u16,
    pub lastId: u16,
}

unsafe impl Sync for RfuSIO32Id {}

#[cfg(target_arch = "arm")]
const _: () = {
    #[allow(unused_imports)]
    use core::mem::{offset_of, size_of};
    assert!(size_of::<RfuSIO32Id>() == 12);
    assert!(offset_of!(RfuSIO32Id, MS_mode) == 0);
    assert!(offset_of!(RfuSIO32Id, state) == 1);
    assert!(offset_of!(RfuSIO32Id, count) == 2);
    assert!(offset_of!(RfuSIO32Id, send_id) == 4);
    assert!(offset_of!(RfuSIO32Id, recv_id) == 6);
    assert!(offset_of!(RfuSIO32Id, unk8) == 8);
    assert!(offset_of!(RfuSIO32Id, lastId) == 10);
};

static Sio32ConnectionData: Table<CArray<u16, 4>> =
    Table((&raw const crate::data::librfu_sio32id::Sio32ConnectionData).cast());

#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gRfuSIO32Id: RfuSIO32Id = unsafe { zeroed() };

unsafe extern "C" {
    static mut gSTWIStatus: *mut STWIStatus;
    fn CpuSet(a0: *mut c_void, a1: *mut c_void, a2: u32);
    fn STWI_set_Callback_ID(a0: Option<unsafe extern "C" fn()>);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn AgbRFU_checkID(mut maxTries: u8) -> i32 {
    let mut ieBak: u16 = 0;
    let mut regTMCNTL: *mut u16 = null_mut();
    let mut id: i32 = 0;
    if (67109384 as usize as *mut u16).read_volatile() == 0 {
        return -1;
    }
    ieBak = (0x4000200 as usize as *mut u16).read_volatile();
    volatile_write(&raw mut (*gSTWIStatus).state, 10);
    STWI_set_Callback_ID(Some(Sio32IDIntr));
    Sio32IDInit();
    regTMCNTL = (0x4000100 + (*gSTWIStatus).timerSelect as i32 * 4) as usize as *mut u16;
    maxTries *= 8;
    while ({
        maxTries -= 1;
        maxTries
    }) != 0xFF
    {
        id = Sio32IDMain();
        if id != 0 {
            break;
        }
        volatile_write(regTMCNTL.at(1), 0);
        volatile_write(regTMCNTL, 0);
        volatile_write(regTMCNTL.at(1), 131);
        while (regTMCNTL).read_volatile() < 32 {}
        volatile_write(regTMCNTL.at(1), 0);
        volatile_write(regTMCNTL, 0);
    }
    volatile_write(67109384 as usize as *mut u16, 0);
    volatile_write(0x4000200 as usize as *mut u16, ieBak);
    volatile_write(67109384 as usize as *mut u16, 1);
    volatile_write(&raw mut (*gSTWIStatus).state, 0);
    STWI_set_Callback_ID(None);
    return id;
}
pub(crate) unsafe extern "C" fn Sio32IDInit() {
    volatile_write(67109384 as usize as *mut u16, 0);
    volatile_write(
        0x4000200 as usize as *mut u16,
        (0x4000200 as usize as *mut u16).read_volatile()
            & !(shl_i32(8, (*gSTWIStatus).timerSelect as u32) as u16 | INTR_FLAG_SERIAL),
    );
    volatile_write(67109384 as usize as *mut u16, 1);
    volatile_write(67109172 as usize as *mut u16, 0);
    volatile_write(67109160 as usize as *mut u16, SIO_32BIT_MODE);
    volatile_write(
        67109160 as usize as *mut u16,
        (67109160 as usize as *mut u16).read_volatile() | 16512,
    );
    {
        {
            let mut tmp: u32 = 0;
            volatile_write(&raw mut tmp, 0);
            CpuSet(
                &raw mut tmp as *mut c_void,
                &raw mut gRfuSIO32Id as *mut c_void,
                0x5000003,
            );
        }
    }
    volatile_write(67109378 as usize as *mut u16, INTR_FLAG_SERIAL);
}
pub(crate) unsafe extern "C" fn Sio32IDMain() -> i32 {
    'l1: {
        let sw1: u8 = gRfuSIO32Id.state;
        let matched = sw1 == 0 || sw1 == 1;
        let mut fall = false;
        if sw1 == 0 {
            fall = true;
            gRfuSIO32Id.MS_mode = AGB_CLK_MASTER;
            volatile_write(
                67109160 as usize as *mut u16,
                (67109160 as usize as *mut u16).read_volatile() | SIO_38400_BPS,
            );
            volatile_write(67109384 as usize as *mut u16, 0);
            volatile_write(
                0x4000200 as usize as *mut u16,
                (0x4000200 as usize as *mut u16).read_volatile() | INTR_FLAG_SERIAL,
            );
            volatile_write(67109384 as usize as *mut u16, 1);
            gRfuSIO32Id.state = 1;
            volatile_write(
                67109160 as usize as *mut u16 as *mut u8,
                (67109160 as usize as *mut u16 as *mut u8).read_volatile() | SIO_ENABLE as u8,
            );
            break 'l1;
        }
        if sw1 == 1 {
            fall = true;
            if gRfuSIO32Id.lastId == 0 {
                if gRfuSIO32Id.MS_mode == AGB_CLK_MASTER {
                    if gRfuSIO32Id.count == 0 {
                        volatile_write(67109384 as usize as *mut u16, 0);
                        volatile_write(
                            67109160 as usize as *mut u16,
                            (67109160 as usize as *mut u16).read_volatile() | SIO_ENABLE,
                        );
                        volatile_write(67109384 as usize as *mut u16, 1);
                    }
                } else if gRfuSIO32Id.send_id != RFU_ID as u16 && gRfuSIO32Id.count == 0 {
                    volatile_write(67109384 as usize as *mut u16, 0);
                    volatile_write(
                        0x4000200 as usize as *mut u16,
                        (0x4000200 as usize as *mut u16).read_volatile() & 65407,
                    );
                    volatile_write(67109384 as usize as *mut u16, 1);
                    volatile_write(67109160 as usize as *mut u16, 0);
                    volatile_write(67109160 as usize as *mut u16, SIO_32BIT_MODE);
                    volatile_write(67109378 as usize as *mut u16, INTR_FLAG_SERIAL);
                    volatile_write(
                        67109160 as usize as *mut u16,
                        (67109160 as usize as *mut u16).read_volatile() | 16512,
                    );
                    volatile_write(67109384 as usize as *mut u16, 0);
                    volatile_write(
                        0x4000200 as usize as *mut u16,
                        (0x4000200 as usize as *mut u16).read_volatile() | INTR_FLAG_SERIAL,
                    );
                    volatile_write(67109384 as usize as *mut u16, 1);
                }
                break 'l1;
            } else {
                gRfuSIO32Id.state = 2;
            }
        }
        if fall || !matched {
            fall = true;
            return gRfuSIO32Id.lastId as i32;
        }
    }
    return 0;
}
pub(crate) unsafe extern "C" fn Sio32IDIntr() {
    let mut regSIODATA32: u32 = 0;
    let mut delay: u16 = 0;
    let mut rfuSIO32IdUnk0_times_16: u32 = 0;
    regSIODATA32 = (67109152 as usize as *mut u32).read_volatile();
    if gRfuSIO32Id.MS_mode != AGB_CLK_MASTER {
        volatile_write(
            67109160 as usize as *mut u16,
            (67109160 as usize as *mut u16).read_volatile() | SIO_ENABLE,
        );
    }
    rfuSIO32IdUnk0_times_16 = shl_u32(regSIODATA32, 16 * gRfuSIO32Id.MS_mode as u32) >> 16;
    regSIODATA32 = shl_u32(regSIODATA32, 16 * (1 - gRfuSIO32Id.MS_mode as u32)) >> 16;
    if gRfuSIO32Id.lastId == 0 {
        let mut backup: u16 = rfuSIO32IdUnk0_times_16 as u16;
        if backup == gRfuSIO32Id.recv_id {
            if gRfuSIO32Id.count < 4 {
                backup = !gRfuSIO32Id.send_id;
                if gRfuSIO32Id.recv_id == backup {
                    if regSIODATA32 == !(gRfuSIO32Id.recv_id as u32) {
                        gRfuSIO32Id.count += 1;
                    }
                }
            } else {
                gRfuSIO32Id.lastId = regSIODATA32 as u16;
            }
        } else {
            gRfuSIO32Id.count = 0;
        }
    }
    if gRfuSIO32Id.count < 4 {
        gRfuSIO32Id.send_id = *Sio32ConnectionData
            .as_ptr()
            .cast_mut()
            .at(gRfuSIO32Id.count);
    } else {
        gRfuSIO32Id.send_id = RFU_ID as u16;
    }
    gRfuSIO32Id.recv_id = !(regSIODATA32 as u16);
    volatile_write(
        67109152 as usize as *mut u32,
        shl_i32(
            gRfuSIO32Id.send_id as i32,
            16 * (1 - gRfuSIO32Id.MS_mode as u32),
        ) as u32
            + shl_i32(gRfuSIO32Id.recv_id as i32, 16 * gRfuSIO32Id.MS_mode as u32) as u32,
    );
    if gRfuSIO32Id.MS_mode == AGB_CLK_MASTER && (gRfuSIO32Id.count != 0 || regSIODATA32 == 0x494e) {
        delay = 0;
        while delay < 600 {
            delay += 1;
        }
        if gRfuSIO32Id.lastId == 0 {
            volatile_write(
                67109160 as usize as *mut u16,
                (67109160 as usize as *mut u16).read_volatile() | SIO_ENABLE,
            );
        }
    }
}
