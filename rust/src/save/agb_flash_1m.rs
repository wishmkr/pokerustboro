//! Translated from `src/agb_flash_1m.c` by tools/rustport/c2rs.py.
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
    unused_assignments
)]

use crate::agb_flash::{
    EraseFlashChip, EraseFlashSector, PollFlashStatus, ProgramFlashByte, ProgramFlashSector,
    ReadFlashId, StartFlashTimer, StopFlashTimer, WaitForFlashWrite, gFlash, gFlashMaxTime,
    gFlashTimeoutFlag,
};
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
// Data tables (translate with cdata.py): AgbLibFlashVersion sSetupInfos

static sSetupInfos: Table<CArray<*mut FlashSetupInfo, 3>> =
    Table((&raw const crate::data::agb_flash_1m::sSetupInfos).cast());

#[unsafe(no_mangle)]
pub unsafe fn IdentifyFlash() -> u16 {
    volatile_write(
        67109380_usize as *mut u16,
        (67109380_usize as *mut u16).read_volatile() & 65532 | 3,
    );
    let flashId: u16 = ReadFlashId();
    let mut setupInfo: *mut *mut FlashSetupInfo = sSetupInfos.as_ptr().cast_mut();
    let mut result: u16 = 1;
    loop {
        if (*(*setupInfo)).r#type.ids.separate.makerId == 0 {
            break;
        }
        if flashId == (*(*setupInfo)).r#type.ids.joined {
            result = 0;
            break;
        }
        setupInfo = setupInfo.at(1);
    }
    ProgramFlashByte = (*(*setupInfo)).programFlashByte;
    ProgramFlashSector = (*(*setupInfo)).programFlashSector;
    EraseFlashChip = (*(*setupInfo)).eraseFlashChip;
    EraseFlashSector = (*(*setupInfo)).eraseFlashSector;
    WaitForFlashWrite = (*(*setupInfo)).WaitForFlashWrite;
    gFlashMaxTime = (*(*setupInfo)).maxTime;
    gFlash = &raw mut (*(*setupInfo)).r#type;
    result
}
pub unsafe fn WaitForFlashWrite_Common(phase: u8, addr: *mut u8, lastData: u8) -> u16 {
    let mut result: u16 = 0;
    let mut status: u8 = 0;
    StartFlashTimer(phase);
    while ({
        status = PollFlashStatus.unwrap_unchecked()(addr);
        status
    }) != lastData
    {
        if status as i32 & 0x20 != 0 {
            if PollFlashStatus.unwrap_unchecked()(addr) == lastData {
                break;
            }
            volatile_write((0xE000000_usize as *mut u8).at(21845), 0xF0);
            result = phase as u16 | 0xA000;
            break;
        }
        if gFlashTimeoutFlag != 0 {
            if PollFlashStatus.unwrap_unchecked()(addr) == lastData {
                break;
            }
            volatile_write((0xE000000_usize as *mut u8).at(21845), 0xF0);
            result = phase as u16 | 0xC000;
            break;
        }
    }
    StopFlashTimer();
    result
}
