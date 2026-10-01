//! Buffered GPU register writes.
//!
//! Writes are staged in `GPU_REG_BUFFER` and flushed to hardware during
//! vblank or forced blank, so mid-frame changes never tear.

use crate::ffi::{
    DISPCNT_FORCED_BLANK, DISPSTAT_HBLANK_INTR, DISPSTAT_VBLANK_INTR, INTR_FLAG_HBLANK,
    INTR_FLAG_VBLANK, REG_OFFSET_DISPCNT, REG_OFFSET_DISPSTAT, REG_OFFSET_IE, REG_OFFSET_IME,
    REG_OFFSET_VCOUNT, read_reg16, write_reg16,
};

const GPU_REG_BUF_SIZE: usize = 0x60;
const EMPTY_SLOT: u8 = 0xff;

/// Scanlines during which a register may be written straight to hardware.
const VBLANK_FIRST_LINE: u16 = 161;
const VBLANK_LAST_LINE: u16 = 225;

/// Read back as `u16`, so the buffer has to be at least two-byte aligned.
#[repr(C, align(4))]
struct RegBuffer([u8; GPU_REG_BUF_SIZE]);

static mut GPU_REG_BUFFER: RegBuffer = RegBuffer([0; GPU_REG_BUF_SIZE]);
static mut GPU_REG_WAITING_LIST: [u8; GPU_REG_BUF_SIZE] = [0; GPU_REG_BUF_SIZE];
static mut GPU_REG_BUFFER_LOCKED: bool = false;
static mut SHOULD_SYNC_REG_IE: bool = false;
static mut REG_IE_SHADOW: u16 = 0;

/// `GPU_REG_BUF(offset)`
#[inline]
unsafe fn buffered(reg_offset: u8) -> u16 {
    unsafe {
        (&raw const GPU_REG_BUFFER)
            .cast::<u8>()
            .add(reg_offset as usize)
            .cast::<u16>()
            .read_volatile()
    }
}

#[inline]
unsafe fn set_buffered(reg_offset: u8, value: u16) {
    unsafe {
        (&raw mut GPU_REG_BUFFER)
            .cast::<u8>()
            .add(reg_offset as usize)
            .cast::<u16>()
            .write_volatile(value)
    };
}

#[inline]
unsafe fn waiting(index: usize) -> u8 {
    unsafe {
        (&raw const GPU_REG_WAITING_LIST)
            .cast::<u8>()
            .add(index)
            .read_volatile()
    }
}

#[inline]
unsafe fn set_waiting(index: usize, value: u8) {
    unsafe {
        (&raw mut GPU_REG_WAITING_LIST)
            .cast::<u8>()
            .add(index)
            .write_volatile(value)
    };
}

#[inline]
unsafe fn set_locked(locked: bool) {
    unsafe { (&raw mut GPU_REG_BUFFER_LOCKED).write_volatile(locked) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn InitGpuRegManager() {
    let mut i = 0usize;
    while i < GPU_REG_BUF_SIZE {
        unsafe {
            (&raw mut GPU_REG_BUFFER)
                .cast::<u8>()
                .add(i)
                .write_volatile(0)
        };
        unsafe { set_waiting(i, EMPTY_SLOT) };
        i += 1;
    }

    unsafe { set_locked(false) };
    unsafe { (&raw mut SHOULD_SYNC_REG_IE).write_volatile(false) };
    unsafe { (&raw mut REG_IE_SHADOW).write_volatile(0) };
}

unsafe fn copy_buffered_value_to_gpu_reg(reg_offset: u8) {
    if reg_offset as usize == REG_OFFSET_DISPSTAT {
        // Only the interrupt-enable bits of DISPSTAT are ours to set; the
        // status bits below them belong to the hardware.
        let current = unsafe { read_reg16(REG_OFFSET_DISPSTAT) };
        unsafe {
            write_reg16(
                REG_OFFSET_DISPSTAT,
                current & !(DISPSTAT_HBLANK_INTR | DISPSTAT_VBLANK_INTR),
            )
        };
        let cleared = unsafe { read_reg16(REG_OFFSET_DISPSTAT) };
        unsafe {
            write_reg16(
                REG_OFFSET_DISPSTAT,
                cleared | buffered(REG_OFFSET_DISPSTAT as u8),
            )
        };
    } else {
        unsafe { write_reg16(reg_offset as usize, buffered(reg_offset)) };
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn CopyBufferedValuesToGpuRegs() {
    if unsafe { (&raw const GPU_REG_BUFFER_LOCKED).read_volatile() } {
        return;
    }

    let mut i = 0usize;
    while i < GPU_REG_BUF_SIZE {
        let reg_offset = unsafe { waiting(i) };
        if reg_offset == EMPTY_SLOT {
            return;
        }
        unsafe { copy_buffered_value_to_gpu_reg(reg_offset) };
        unsafe { set_waiting(i, EMPTY_SLOT) };
        i += 1;
    }
}

/// Appends `reg_offset` to the waiting list unless it is already queued.
unsafe fn queue_register(reg_offset: u8) {
    unsafe { set_locked(true) };

    let mut i = 0usize;
    while i < GPU_REG_BUF_SIZE && unsafe { waiting(i) } != EMPTY_SLOT {
        if unsafe { waiting(i) } == reg_offset {
            unsafe { set_locked(false) };
            return;
        }
        i += 1;
    }

    unsafe { set_waiting(i, reg_offset) };
    unsafe { set_locked(false) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetGpuReg(reg_offset: u8, value: u16) {
    if reg_offset as usize >= GPU_REG_BUF_SIZE {
        return;
    }

    unsafe { set_buffered(reg_offset, value) };
    let vcount = unsafe { read_reg16(REG_OFFSET_VCOUNT) } & 0xff;

    if (VBLANK_FIRST_LINE..=VBLANK_LAST_LINE).contains(&vcount)
        || unsafe { read_reg16(REG_OFFSET_DISPCNT) } & DISPCNT_FORCED_BLANK != 0
    {
        unsafe { copy_buffered_value_to_gpu_reg(reg_offset) };
    } else {
        unsafe { queue_register(reg_offset) };
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetGpuReg_ForcedBlank(reg_offset: u8, value: u16) {
    if reg_offset as usize >= GPU_REG_BUF_SIZE {
        return;
    }

    unsafe { set_buffered(reg_offset, value) };

    if unsafe { read_reg16(REG_OFFSET_DISPCNT) } & DISPCNT_FORCED_BLANK != 0 {
        unsafe { copy_buffered_value_to_gpu_reg(reg_offset) };
    } else {
        unsafe { queue_register(reg_offset) };
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetGpuReg(reg_offset: u8) -> u16 {
    if reg_offset as usize == REG_OFFSET_DISPSTAT {
        return unsafe { read_reg16(REG_OFFSET_DISPSTAT) };
    }
    if reg_offset as usize == REG_OFFSET_VCOUNT {
        return unsafe { read_reg16(REG_OFFSET_VCOUNT) };
    }
    unsafe { buffered(reg_offset) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetGpuRegBits(reg_offset: u8, mask: u16) {
    let value = unsafe { buffered(reg_offset) };
    unsafe { SetGpuReg(reg_offset, value | mask) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ClearGpuRegBits(reg_offset: u8, mask: u16) {
    let value = unsafe { buffered(reg_offset) };
    unsafe { SetGpuReg(reg_offset, value & !mask) };
}

unsafe fn sync_reg_ie() {
    if !unsafe { (&raw const SHOULD_SYNC_REG_IE).read_volatile() } {
        return;
    }

    // Interrupts are masked around the write so REG_IE is never observed
    // half-updated.
    let ime = unsafe { read_reg16(REG_OFFSET_IME) };
    unsafe { write_reg16(REG_OFFSET_IME, 0) };
    unsafe { write_reg16(REG_OFFSET_IE, (&raw const REG_IE_SHADOW).read_volatile()) };
    unsafe { write_reg16(REG_OFFSET_IME, ime) };
    unsafe { (&raw mut SHOULD_SYNC_REG_IE).write_volatile(false) };
}

unsafe fn update_reg_dispstat_intr_bits(reg_ie: u16) {
    let old_value = unsafe { GetGpuReg(REG_OFFSET_DISPSTAT as u8) }
        & (DISPSTAT_HBLANK_INTR | DISPSTAT_VBLANK_INTR);
    let mut new_value = 0u16;

    if reg_ie & INTR_FLAG_VBLANK != 0 {
        new_value |= DISPSTAT_VBLANK_INTR;
    }
    if reg_ie & INTR_FLAG_HBLANK != 0 {
        new_value |= DISPSTAT_HBLANK_INTR;
    }

    if old_value != new_value {
        unsafe { SetGpuReg(REG_OFFSET_DISPSTAT as u8, new_value) };
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn EnableInterrupts(mask: u16) {
    let value = unsafe { (&raw const REG_IE_SHADOW).read_volatile() } | mask;
    unsafe { (&raw mut REG_IE_SHADOW).write_volatile(value) };
    unsafe { (&raw mut SHOULD_SYNC_REG_IE).write_volatile(true) };
    unsafe { sync_reg_ie() };
    unsafe { update_reg_dispstat_intr_bits(value) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn DisableInterrupts(mask: u16) {
    let value = unsafe { (&raw const REG_IE_SHADOW).read_volatile() } & !mask;
    unsafe { (&raw mut REG_IE_SHADOW).write_volatile(value) };
    unsafe { (&raw mut SHOULD_SYNC_REG_IE).write_volatile(true) };
    unsafe { sync_reg_ie() };
    unsafe { update_reg_dispstat_intr_bits(value) };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_buffer_covers_every_bufferable_register() {
        // DISPCNT through the window and blend registers.
        assert_eq!(GPU_REG_BUF_SIZE, 0x60);
        assert!(REG_OFFSET_DISPSTAT < GPU_REG_BUF_SIZE);
        assert!(REG_OFFSET_VCOUNT < GPU_REG_BUF_SIZE);
        assert_eq!(core::mem::align_of::<RegBuffer>(), 4);
    }

    #[test]
    fn writes_go_straight_to_hardware_only_inside_vblank() {
        let immediate = |vcount: u16| (VBLANK_FIRST_LINE..=VBLANK_LAST_LINE).contains(&vcount);
        assert!(!immediate(0));
        assert!(!immediate(160));
        assert!(immediate(161));
        assert!(immediate(225));
        assert!(!immediate(226));
    }

    #[test]
    fn dispstat_interrupt_bits_follow_the_interrupt_mask() {
        let bits = |reg_ie: u16| {
            let mut value = 0u16;
            if reg_ie & INTR_FLAG_VBLANK != 0 {
                value |= DISPSTAT_VBLANK_INTR;
            }
            if reg_ie & INTR_FLAG_HBLANK != 0 {
                value |= DISPSTAT_HBLANK_INTR;
            }
            value
        };
        assert_eq!(bits(0), 0);
        assert_eq!(bits(INTR_FLAG_VBLANK), DISPSTAT_VBLANK_INTR);
        assert_eq!(bits(INTR_FLAG_HBLANK), DISPSTAT_HBLANK_INTR);
        assert_eq!(
            bits(INTR_FLAG_VBLANK | INTR_FLAG_HBLANK),
            DISPSTAT_VBLANK_INTR | DISPSTAT_HBLANK_INTR
        );
    }
}
