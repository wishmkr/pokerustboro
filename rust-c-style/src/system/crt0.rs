//! Start-up code and the interrupt dispatcher (was src/crt0.s).
//!
//! Both functions switch CPU modes and set stack pointers, which only
//! instructions can do, so their bodies stay ARM assembly inside naked Rust
//! functions. The ROM header's first instruction jumps to [`Init`] (see
//! rom_header.rs).

/// Reset entry: sets up the IRQ and System mode stacks, installs
/// [`IntrMain`] as the interrupt handler, clears RAM and registers with the
/// BIOS (`RegisterRamReset(RESET_ALL)`), copies the `.iwram_code` section
/// (the sound mixer) from ROM to IWRAM, then runs `AgbMain`, forever.
#[cfg(target_arch = "arm")]
#[unsafe(no_mangle)]
#[unsafe(naked)]
#[instruction_set(arm::a32)]
pub unsafe extern "C" fn Init() {
    core::arch::naked_asm!(
        "mov r0, #0x12",           // PSR_IRQ_MODE
        "msr cpsr_cf, r0",
        "ldr sp, =0x03007fa0",     // IWRAM_END - 0x60
        "mov r0, #0x1f",           // PSR_SYS_MODE
        "msr cpsr_cf, r0",
        "ldr sp, =0x03007e40",     // IWRAM_END - 0x1c0
        "ldr r1, =0x03007ffc",     // INTR_VECTOR
        "ldr r0, ={intr_main}",
        "str r0, [r1]",
        "mov r0, #255",            // RESET_ALL
        "svc #0x10000",            // RegisterRamReset (ARM encoding of svc 1)
        "ldr r0, =__iwram_code_lma", // copy the IWRAM code from ROM
        "ldr r1, =__iwram_code_start",
        "ldr r2, =__iwram_code_end",
        "1:",
        "cmp r1, r2",
        "ldrlo r3, [r0], #4",
        "strlo r3, [r1], #4",
        "blo 1b",
        "ldr r1, ={agb_main}",
        "mov lr, pc",
        "bx r1",
        "b {init}",
        ".ltorg",
        intr_main = sym IntrMain,
        agb_main = sym crate::agb_main::AgbMain,
        init = sym Init,
    )
}

/// The IRQ handler the BIOS calls. Finds the highest-priority pending
/// interrupt, acknowledges it, re-enables the nested ones allowed during it,
/// and calls its entry in `gIntrTable` in System mode.
///
/// InitIntrHandlers copies this function (0x800 bytes from its start) to
/// IWRAM and runs the copy, so it must stay position independent: only
/// PC-relative branches, and its literal pool right after it (`.ltorg`).
#[cfg(target_arch = "arm")]
#[unsafe(no_mangle)]
#[unsafe(naked)]
#[instruction_set(arm::a32)]
pub unsafe extern "C" fn IntrMain() {
    core::arch::naked_asm!(
        "mov r3, #0x04000000",     // REG_BASE
        "add r3, r3, #0x200",      // REG_IE
        "ldr r2, [r3]",            // IE | IF << 16
        "ldrh r1, [r3, #8]",       // IME
        "mrs r0, spsr",
        "stmfd sp!, {{r0-r3, lr}}",
        "mov r0, #0",
        "strh r0, [r3, #8]",       // IME = 0
        "and r1, r2, r2, lsr #16", // enabled & requested
        "mov r12, #0",
        "ands r0, r1, #0x4",       // VCOUNT
        "bne 2f",
        "add r12, r12, #4",
        "mov r0, #1",
        "strh r0, [r3, #8]",       // IME = 1: the rest may be interrupted
        "ands r0, r1, #0x80",      // SERIAL
        "bne 2f",
        "add r12, r12, #4",
        "ands r0, r1, #0x40",      // TIMER3
        "bne 2f",
        "add r12, r12, #4",
        "ands r0, r1, #0x2",       // HBLANK
        "bne 2f",
        "add r12, r12, #4",
        "ands r0, r1, #0x1",       // VBLANK
        "bne 2f",
        "add r12, r12, #4",
        "ands r0, r1, #0x8",       // TIMER0
        "bne 2f",
        "add r12, r12, #4",
        "ands r0, r1, #0x10",      // TIMER1
        "bne 2f",
        "add r12, r12, #4",
        "ands r0, r1, #0x20",      // TIMER2
        "bne 2f",
        "add r12, r12, #4",
        "ands r0, r1, #0x100",     // DMA0
        "bne 2f",
        "add r12, r12, #4",
        "ands r0, r1, #0x200",     // DMA1
        "bne 2f",
        "add r12, r12, #4",
        "ands r0, r1, #0x400",     // DMA2
        "bne 2f",
        "add r12, r12, #4",
        "ands r0, r1, #0x800",     // DMA3
        "bne 2f",
        "add r12, r12, #4",
        "ands r0, r1, #0x1000",    // KEYPAD
        "bne 2f",
        "add r12, r12, #4",
        "ands r0, r1, #0x2000",    // GAMEPAK: cartridge pulled
        "strbne r0, [r3, #-0x17c]", // REG_SOUNDCNT_X: sound off
        "3:",
        "bne 3b",                  // and spin
        "2:",
        "strh r0, [r3, #2]",       // acknowledge in IF
        "bic r2, r2, r0",
        "ldr r0, ={stwi_status}",
        "ldr r0, [r0]",
        "ldrb r0, [r0, #0xa]",     // gSTWIStatus->timerSelect
        "mov r1, #0x8",
        "lsl r0, r1, r0",          // its TIMER flag
        "orr r0, r0, #0x2000",
        "orr r1, r0, #0xc6",       // SERIAL | TIMER3 | VCOUNT | HBLANK
        "and r1, r1, r2",
        "strh r1, [r3]",           // IE: only those may nest
        "mrs r3, cpsr",
        "bic r3, r3, #0xdf",       // I, F and mode bits
        "orr r3, r3, #0x1f",       // System mode, IRQs on
        "msr cpsr_cf, r3",
        "ldr r1, ={intr_table}",
        "add r1, r1, r12",
        "ldr r0, [r1]",
        "stmfd sp!, {{lr}}",
        "adr lr, 4f",
        "bx r0",
        "4:",
        "ldmfd sp!, {{lr}}",
        "mrs r3, cpsr",
        "bic r3, r3, #0xdf",
        "orr r3, r3, #0x92",       // IRQ mode, IRQs off
        "msr cpsr_cf, r3",
        "ldmia sp!, {{r0-r3, lr}}",
        "strh r2, [r3]",           // restore IE
        "strh r1, [r3, #8]",       // restore IME
        "msr spsr_cf, r0",
        "bx lr",
        ".ltorg",
        stwi_status = sym crate::librfu_stwi::gSTWIStatus,
        intr_table = sym crate::agb_main::gIntrTable,
    )
}

/// Host builds (unit tests) never boot.
#[cfg(not(target_arch = "arm"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Init() {}

#[cfg(not(target_arch = "arm"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IntrMain() {}
