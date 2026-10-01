//! GBA BIOS calls (was libagbsyscall, assembly).
//!
//! Each is the `svc` instruction for one BIOS function; arguments are already
//! in r0-r3 by the C calling convention, so a naked Thumb function holding
//! `svc #n; bx lr` is the whole call. Signatures follow include/gba/syscall.h.

#![allow(unused_variables)]

#[cfg(target_arch = "arm")]
#[unsafe(no_mangle)]
#[unsafe(naked)]
pub unsafe extern "C" fn ArcTan2(x: i16, y: i16) -> u16 {
    core::arch::naked_asm!("svc #10", "bx lr")
}

#[cfg(target_arch = "arm")]
#[unsafe(no_mangle)]
#[unsafe(naked)]
pub unsafe extern "C" fn BgAffineSet(src: *const u8, dest: *mut u8, count: i32) {
    core::arch::naked_asm!("svc #14", "bx lr")
}

#[cfg(target_arch = "arm")]
#[unsafe(no_mangle)]
#[unsafe(naked)]
pub unsafe extern "C" fn CpuFastSet(src: *const u8, dest: *mut u8, control: u32) {
    core::arch::naked_asm!("svc #12", "bx lr")
}

#[cfg(target_arch = "arm")]
#[unsafe(no_mangle)]
#[unsafe(naked)]
pub unsafe extern "C" fn CpuSet(src: *const u8, dest: *mut u8, control: u32) {
    core::arch::naked_asm!("svc #11", "bx lr")
}

#[cfg(target_arch = "arm")]
#[unsafe(no_mangle)]
#[unsafe(naked)]
pub unsafe extern "C" fn Div(num: i32, denom: i32) -> i32 {
    core::arch::naked_asm!("svc #6", "bx lr")
}

#[cfg(target_arch = "arm")]
#[unsafe(no_mangle)]
#[unsafe(naked)]
pub unsafe extern "C" fn LZ77UnCompVram(src: *const u8, dest: *mut u8) {
    core::arch::naked_asm!("svc #18", "bx lr")
}

#[cfg(target_arch = "arm")]
#[unsafe(no_mangle)]
#[unsafe(naked)]
pub unsafe extern "C" fn LZ77UnCompWram(src: *const u8, dest: *mut u8) {
    core::arch::naked_asm!("svc #17", "bx lr")
}

#[cfg(target_arch = "arm")]
#[unsafe(no_mangle)]
#[unsafe(naked)]
pub unsafe extern "C" fn MultiBoot(mp: *mut u8) -> i32 {
    core::arch::naked_asm!("movs r1, #1", "svc #37", "bx lr")
}

#[cfg(target_arch = "arm")]
#[unsafe(no_mangle)]
#[unsafe(naked)]
pub unsafe extern "C" fn ObjAffineSet(src: *const u8, dest: *mut u8, count: i32, offset: i32) {
    core::arch::naked_asm!("svc #15", "bx lr")
}

#[cfg(target_arch = "arm")]
#[unsafe(no_mangle)]
#[unsafe(naked)]
pub unsafe extern "C" fn RegisterRamReset(reset_flags: u32) {
    core::arch::naked_asm!("svc #1", "bx lr")
}

#[cfg(target_arch = "arm")]
#[unsafe(no_mangle)]
#[unsafe(naked)]
pub unsafe extern "C" fn RLUnCompVram(src: *const u8, dest: *mut u8) {
    core::arch::naked_asm!("svc #21", "bx lr")
}

#[cfg(target_arch = "arm")]
#[unsafe(no_mangle)]
#[unsafe(naked)]
pub unsafe extern "C" fn RLUnCompWram(src: *const u8, dest: *mut u8) {
    core::arch::naked_asm!("svc #20", "bx lr")
}

#[cfg(target_arch = "arm")]
#[unsafe(no_mangle)]
#[unsafe(naked)]
pub unsafe extern "C" fn Sqrt(num: u32) -> u16 {
    core::arch::naked_asm!("svc #8", "bx lr")
}

#[cfg(target_arch = "arm")]
#[unsafe(no_mangle)]
#[unsafe(naked)]
pub unsafe extern "C" fn VBlankIntrWait() {
    core::arch::naked_asm!("movs r2, #0", "svc #5", "bx lr")
}

/// `SoftReset(resetFlags)`: interrupts off, a known stack, clear what the
/// flags ask for, and restart through the BIOS.
#[cfg(target_arch = "arm")]
#[unsafe(no_mangle)]
#[unsafe(naked)]
pub unsafe extern "C" fn SoftReset(reset_flags: u32) -> ! {
    core::arch::naked_asm!(
        "ldr r3, =0x04000208", // REG_IME
        "movs r2, #0",
        "strb r2, [r3]",
        "ldr r1, =0x03007f00",
        "mov sp, r1",
        "svc #1", // RegisterRamReset(resetFlags)
        "svc #0", // SoftReset
        ".ltorg",
    )
}

// Host builds (unit tests) have no BIOS; these are never called there.
#[cfg(not(target_arch = "arm"))]
pub use host::*;

#[cfg(not(target_arch = "arm"))]
mod host {

    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn ArcTan2(x: i16, y: i16) -> u16 {
        0
    }

    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn BgAffineSet(src: *const u8, dest: *mut u8, count: i32) {}

    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn CpuFastSet(src: *const u8, dest: *mut u8, control: u32) {}

    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn CpuSet(src: *const u8, dest: *mut u8, control: u32) {}

    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn Div(num: i32, denom: i32) -> i32 {
        0
    }

    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn LZ77UnCompVram(src: *const u8, dest: *mut u8) {}

    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn LZ77UnCompWram(src: *const u8, dest: *mut u8) {}

    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn MultiBoot(mp: *mut u8) -> i32 {
        0
    }

    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn ObjAffineSet(src: *const u8, dest: *mut u8, count: i32, offset: i32) {}

    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn RegisterRamReset(reset_flags: u32) {}

    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn RLUnCompVram(src: *const u8, dest: *mut u8) {}

    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn RLUnCompWram(src: *const u8, dest: *mut u8) {}

    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn Sqrt(num: u32) -> u16 {
        0
    }

    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn VBlankIntrWait() {}

    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn SoftReset(reset_flags: u32) -> ! {
        loop {}
    }
}
