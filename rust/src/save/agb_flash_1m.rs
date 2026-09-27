//! Translated from `src/agb_flash_1m.c` by tools/rustport/c2rs.py, then reviewed.
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

// Data tables (translate with cdata.py): AgbLibFlashVersion sSetupInfos
#[allow(unused_imports)]
use crate::data::agb_flash_1m::*;

unsafe extern "C" {
    static mut EraseFlashChip: u8;
    static mut EraseFlashSector: u8;
    static mut PollFlashStatus: u8;
    static mut ProgramFlashByte: u8;
    static mut ProgramFlashSector: u8;
    static mut WaitForFlashWrite: u8;
    static mut gFlash: u8;
    static mut gFlashMaxTime: u8;
    static mut gFlashTimeoutFlag: u8;
    fn ReadFlashId() -> u16;
    fn StartFlashTimer(a0: u8);
    fn StopFlashTimer();
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn IdentifyFlash() -> u16 {
    unsafe {
        let mut result: u16 = 0u16;
        let mut flashId: u16 = 0u16;
        let mut setupInfo: *mut *mut u8 = core::ptr::null_mut();
        crate::c::volatile_write(
            ((67109380i32) as usize as *mut u16),
            (((((((67109380i32) as usize as *mut u16).read_volatile()) as i32) & (-4i32)) | 3i32)
                as u16),
        );
        flashId = ReadFlashId();
        setupInfo = ((&raw const sSetupInfos)
            .cast::<u8>()
            .cast_mut()
            .cast::<*mut u8>())
        .cast::<*mut u8>();
        result = 1u16;
        {
            'l1: loop {
                'l2: {
                    if ((((((setupInfo).read()).wrapping_add(24)).wrapping_add(20)).read()) as i32)
                        == 0i32
                    {
                        break 'l1;
                    }
                    if ((flashId) as i32)
                        == (((((((setupInfo).read()).wrapping_add(24)).wrapping_add(20))
                            .cast::<u16>())
                        .read()) as i32)
                    {
                        result = 0u16;
                        break 'l1;
                    }
                    setupInfo = (setupInfo).wrapping_offset(1);
                }
            }
        }
        ((&raw mut ProgramFlashByte).cast::<Option<unsafe extern "C" fn(u16, u32, u8) -> u16>>())
            .write(
                (((setupInfo).read()).cast::<Option<unsafe extern "C" fn(u16, u32, u8) -> u16>>())
                    .read(),
            );
        ((&raw mut ProgramFlashSector).cast::<Option<unsafe extern "C" fn(u16, *mut u8) -> u16>>())
            .write(
                (((setupInfo).read())
                    .wrapping_add(4)
                    .cast::<Option<unsafe extern "C" fn(u16, *mut u8) -> u16>>())
                .read(),
            );
        ((&raw mut EraseFlashChip).cast::<Option<unsafe extern "C" fn() -> u16>>()).write(
            (((setupInfo).read())
                .wrapping_add(8)
                .cast::<Option<unsafe extern "C" fn() -> u16>>())
            .read(),
        );
        ((&raw mut EraseFlashSector).cast::<Option<unsafe extern "C" fn(u16) -> u16>>()).write(
            (((setupInfo).read())
                .wrapping_add(12)
                .cast::<Option<unsafe extern "C" fn(u16) -> u16>>())
            .read(),
        );
        ((&raw mut WaitForFlashWrite)
            .cast::<Option<unsafe extern "C" fn(u8, *mut u8, u8) -> u16>>())
        .write(
            (((setupInfo).read())
                .wrapping_add(16)
                .cast::<Option<unsafe extern "C" fn(u8, *mut u8, u8) -> u16>>())
            .read(),
        );
        ((&raw mut gFlashMaxTime).cast::<*mut u16>())
            .write((((setupInfo).read()).wrapping_add(20).cast::<*mut u16>()).read());
        ((&raw mut gFlash).cast::<*mut u8>()).write(((setupInfo).read()).wrapping_add(24));
        return result;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn WaitForFlashWrite_Common(phase: u8, addr: *mut u8, lastData: u8) -> u16 {
    unsafe {
        let mut phase = phase;
        let mut addr = addr;
        let mut lastData = lastData;
        let mut result: u16 = 0u16;
        let mut status: u8 = 0u8;
        StartFlashTimer(phase);
        'l1: loop {
            if !((({
                let __v1 = (((&raw mut PollFlashStatus)
                    .cast::<Option<unsafe extern "C" fn(*mut u8) -> u8>>())
                .read())
                .unwrap_unchecked()(addr);
                status = __v1;
                __v1
            }) as i32)
                != ((lastData) as i32))
            {
                break 'l1;
            }
            if (((status) as i32) & 32i32) != 0 {
                if (((((&raw mut PollFlashStatus)
                    .cast::<Option<unsafe extern "C" fn(*mut u8) -> u8>>())
                .read())
                .unwrap_unchecked()(addr)) as i32)
                    == ((lastData) as i32)
                {
                    break 'l1;
                }
                crate::c::volatile_write(
                    ((234881024i32) as usize as *mut u8).wrapping_offset(21845),
                    240u8,
                );
                result = ((((phase) as u32) | 40960u32) as u16);
                break 'l1;
            }
            if (((&raw mut gFlashTimeoutFlag).cast::<u8>()).read()) != 0 {
                if (((((&raw mut PollFlashStatus)
                    .cast::<Option<unsafe extern "C" fn(*mut u8) -> u8>>())
                .read())
                .unwrap_unchecked()(addr)) as i32)
                    == ((lastData) as i32)
                {
                    break 'l1;
                }
                crate::c::volatile_write(
                    ((234881024i32) as usize as *mut u8).wrapping_offset(21845),
                    240u8,
                );
                result = ((((phase) as u32) | 49152u32) as u16);
                break 'l1;
            }
        }
        StopFlashTimer();
        return result;
    }
}
