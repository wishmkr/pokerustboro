//! Translated from `src/agb_flash_1m.c` by tools/rustport/c2rs.py.
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
// Data tables (translate with cdata.py): AgbLibFlashVersion sSetupInfos

static sSetupInfos: Table<CArray<*mut FlashSetupInfo, 3>> =
    Table((&raw const crate::data::agb_flash_1m::sSetupInfos).cast());

unsafe extern "C" {
    static mut EraseFlashChip: Option<unsafe extern "C" fn() -> u16>;
    static mut EraseFlashSector: Option<unsafe extern "C" fn(u16) -> u16>;
    static mut PollFlashStatus: Option<unsafe extern "C" fn(*mut u8) -> u8>;
    static mut ProgramFlashByte: Option<unsafe extern "C" fn(u16, u32, u8) -> u16>;
    static mut ProgramFlashSector: Option<unsafe extern "C" fn(u16, *mut u8) -> u16>;
    static mut WaitForFlashWrite: Option<unsafe extern "C" fn(u8, *mut u8, u8) -> u16>;
    static mut gFlash: *mut FlashType;
    static mut gFlashMaxTime: *mut u16;
    static mut gFlashTimeoutFlag: u8;
    fn ReadFlashId() -> u16;
    fn StartFlashTimer(a0: u8);
    fn StopFlashTimer();
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn IdentifyFlash() -> u16 {
    let mut result: u16 = 0;
    let mut flashId: u16 = 0;
    let mut setupInfo: *mut *mut FlashSetupInfo = null_mut();
    volatile_write(
        67109380 as usize as *mut u16,
        (67109380 as usize as *mut u16).read_volatile() & 65532 | 3,
    );
    flashId = ReadFlashId();
    setupInfo = sSetupInfos.as_ptr().cast_mut();
    result = 1;
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
    return result;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn WaitForFlashWrite_Common(phase: u8, addr: *mut u8, lastData: u8) -> u16 {
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
            volatile_write((0xE000000 as usize as *mut u8).at(21845), 0xF0);
            result = phase as u16 | 0xA000;
            break;
        }
        if gFlashTimeoutFlag != 0 {
            if PollFlashStatus.unwrap_unchecked()(addr) == lastData {
                break;
            }
            volatile_write((0xE000000 as usize as *mut u8).at(21845), 0xF0);
            result = phase as u16 | 0xC000;
            break;
        }
    }
    StopFlashTimer();
    return result;
}
