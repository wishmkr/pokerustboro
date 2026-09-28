//! Translated from `src/agb_flash.c` by tools/rustport/c2rs.py.
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

pub(crate) static mut sTimerNum: u8 = 0;
pub(crate) static mut sTimerCount: u16 = 0;
pub(crate) static mut sTimerReg: *mut u16 = null_mut();
pub(crate) static mut sSavedIme: u16 = 0;
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gFlashTimeoutFlag: u8 = 0;
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut PollFlashStatus: Option<unsafe extern "C" fn(*mut u8) -> u8> = None;
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut WaitForFlashWrite: Option<unsafe extern "C" fn(u8, *mut u8, u8) -> u16> = None;
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut ProgramFlashSector: Option<unsafe extern "C" fn(u16, *mut u8) -> u16> = None;
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gFlash: *mut FlashType = null_mut();
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut ProgramFlashByte: Option<unsafe extern "C" fn(u16, u32, u8) -> u16> = None;
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gFlashNumRemainingBytes: u16 = 0;
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut EraseFlashChip: Option<unsafe extern "C" fn() -> u16> = None;
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut EraseFlashSector: Option<unsafe extern "C" fn(u16) -> u16> = None;
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gFlashMaxTime: *mut u16 = null_mut();

#[unsafe(no_mangle)]
pub unsafe extern "C" fn SwitchFlashBank(bankNum: u8) {
    volatile_write((0xE000000 as usize as *mut u8).at(21845), 0xAA);
    volatile_write((0xE000000 as usize as *mut u8).at(10922), 0x55);
    volatile_write((0xE000000 as usize as *mut u8).at(21845), 0xB0);
    volatile_write(0xE000000 as usize as *mut u8, bankNum);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ReadFlashId() -> u16 {
    let mut flashId: u16 = 0;
    let mut readFlash1Buffer: CArray<u16, 32> = zeroed();
    let mut readFlash1: Option<unsafe extern "C" fn(*mut u8) -> u8> = None;
    SetReadFlash1(readFlash1Buffer.as_mut_ptr());
    readFlash1 = core::mem::transmute::<usize, Option<unsafe extern "C" fn(*mut u8) -> u8>>(
        (readFlash1Buffer.as_mut_ptr() as usize as i32 + 1) as usize,
    );
    volatile_write((0xE000000 as usize as *mut u8).at(21845), 0xAA);
    volatile_write((0xE000000 as usize as *mut u8).at(10922), 0x55);
    volatile_write((0xE000000 as usize as *mut u8).at(21845), 0x90);
    {
        let mut i: u16 = 0;
        volatile_write(&raw mut i, 0);
        volatile_write(&raw mut i, 20000);
        while (&raw mut i).read_volatile() != 0 {
            volatile_write(&raw mut i, (&raw mut i).read_volatile() - 1);
        }
    }
    flashId = (readFlash1.unwrap_unchecked()((0xE000000 as usize as *mut u8).at(1)) as u16) << 8;
    flashId |= readFlash1.unwrap_unchecked()(0xE000000 as usize as *mut u8) as u16;
    volatile_write((0xE000000 as usize as *mut u8).at(21845), 0xAA);
    volatile_write((0xE000000 as usize as *mut u8).at(10922), 0x55);
    volatile_write((0xE000000 as usize as *mut u8).at(21845), 0xF0);
    volatile_write((0xE000000 as usize as *mut u8).at(21845), 0xF0);
    {
        let mut i: u16 = 0;
        volatile_write(&raw mut i, 0);
        volatile_write(&raw mut i, 20000);
        while (&raw mut i).read_volatile() != 0 {
            volatile_write(&raw mut i, (&raw mut i).read_volatile() - 1);
        }
    }
    return flashId;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FlashTimerIntr() {
    if sTimerCount != 0
        && ({
            sTimerCount -= 1;
            sTimerCount
        }) == 0
    {
        gFlashTimeoutFlag = 1;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetFlashTimerIntr(
    timerNum: u8,
    intrFunc: *mut Option<unsafe extern "C" fn()>,
) -> u16 {
    if timerNum >= 4 {
        return 1;
    }
    sTimerNum = timerNum;
    sTimerReg = (0x4000100 + sTimerNum as i32 * 4) as usize as *mut u16;
    *intrFunc = Some(FlashTimerIntr);
    return 0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn StartFlashTimer(phase: u8) {
    let mut maxTime: *mut u16 = gFlashMaxTime.at(phase as i32 * 3);
    sSavedIme = (67109384 as usize as *mut u16).read_volatile();
    volatile_write(67109384 as usize as *mut u16, 0);
    volatile_write(sTimerReg.at(1), 0);
    volatile_write(
        0x4000200 as usize as *mut u16,
        (0x4000200 as usize as *mut u16).read_volatile()
            | shl_i32(INTR_FLAG_TIMER0, sTimerNum as u32) as u16,
    );
    gFlashTimeoutFlag = 0;
    sTimerCount = *({
        let t2 = maxTime;
        maxTime = maxTime.at(1);
        t2
    });
    volatile_write(
        {
            let t3 = sTimerReg;
            sTimerReg = sTimerReg.at(1);
            t3
        },
        *({
            let t5 = maxTime;
            maxTime = maxTime.at(1);
            t5
        }),
    );
    volatile_write(
        {
            let t6 = sTimerReg;
            sTimerReg = sTimerReg.at(-1);
            t6
        },
        *({
            let t8 = maxTime;
            maxTime = maxTime.at(1);
            t8
        }),
    );
    volatile_write(
        67109378 as usize as *mut u16,
        shl_i32(INTR_FLAG_TIMER0, sTimerNum as u32) as u16,
    );
    volatile_write(67109384 as usize as *mut u16, 1);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn StopFlashTimer() {
    volatile_write(67109384 as usize as *mut u16, 0);
    volatile_write(
        {
            let t1 = sTimerReg;
            sTimerReg = sTimerReg.at(1);
            t1
        },
        0,
    );
    volatile_write(
        {
            let t2 = sTimerReg;
            sTimerReg = sTimerReg.at(-1);
            t2
        },
        0,
    );
    volatile_write(
        0x4000200 as usize as *mut u16,
        (0x4000200 as usize as *mut u16).read_volatile()
            & !(shl_i32(INTR_FLAG_TIMER0, sTimerNum as u32) as u16),
    );
    volatile_write(67109384 as usize as *mut u16, sSavedIme);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ReadFlash1(addr: *mut u8) -> u8 {
    return *addr;
}
// hand-written: tools/rustport/overrides/agb_flash/SetReadFlash1.rs
// c2rs-uses: ReadFlash1
/// `SetReadFlash1`: copy `ReadFlash1` into `dest` (0x20 halfwords) and point
/// `PollFlashStatus` at the copy, so flash is polled from RAM. See
/// overrides/agb_flash/_common.txt.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetReadFlash1(dest: *mut u16) {
    unsafe {
        let src = (ReadFlash1 as *const () as usize & !1) as *const u16;
        let mut i = 0usize;
        while i < 0x20 {
            dest.add(i).write_volatile(src.add(i).read_volatile());
            i += 1;
        }
        PollFlashStatus = core::mem::transmute::<usize, Option<unsafe extern "C" fn(*mut u8) -> u8>>(
            dest as usize + 1,
        );
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ReadFlash_Core(mut src: *mut u8, mut dest: *mut u8, mut size: u32) {
    while ({
        let t1 = size;
        size -= 1;
        t1
    }) != 0
    {
        *({
            let t2 = dest;
            dest = dest.at(1);
            t2
        }) = ({
            let t4 = src;
            src = src.at(1);
            t4
        })
        .read_volatile();
    }
}
// hand-written: tools/rustport/overrides/agb_flash/ReadFlash.rs
// c2rs-uses: ReadFlash_Core SwitchFlashBank
/// `ReadFlash`: runs `ReadFlash_Core` from a copy on the stack (see
/// overrides/agb_flash/_common.txt).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ReadFlash(sector_num: u16, offset: u32, dest: *mut u8, size: u32) {
    unsafe {
        let mut sector_num = sector_num;
        flash_set_sram_wait_8();
        if flash_rom_size() == FLASH_ROM_SIZE_1M {
            SwitchFlashBank((sector_num / SECTORS_PER_BANK) as u8);
            sector_num %= SECTORS_PER_BANK;
        }
        let mut buffer = crate::ffi::Align4([0u16; 0x40]);
        let core: unsafe extern "C" fn(*mut u8, *mut u8, u32) = copy_to_ram(
            ReadFlash_Core as *const () as usize,
            buffer.0.as_mut_ptr(),
            0x40,
        );
        let src = FLASH_BASE_ADDR
            .wrapping_add((u32::from(sector_num) << flash_sector_shift()) as usize)
            .wrapping_add(offset as usize);
        core(src as *mut u8, dest, size);
    }
}

const FLASH_BASE_ADDR: usize = 0x0E00_0000;
const FLASH_ROM_SIZE_1M: u32 = 0x2_0000;
const SECTORS_PER_BANK: u16 = 16;
const REG_WAITCNT: *mut u16 = 0x0400_0204 as *mut u16;
const WAITCNT_SRAM_MASK: u16 = 3;
const WAITCNT_SRAM_8: u16 = 3;

/// `REG_WAITCNT = (REG_WAITCNT & ~WAITCNT_SRAM_MASK) | WAITCNT_SRAM_8`
unsafe fn flash_set_sram_wait_8() {
    unsafe {
        let v = REG_WAITCNT.read_volatile();
        crate::c::volatile_write(REG_WAITCNT, (v & !WAITCNT_SRAM_MASK) | WAITCNT_SRAM_8);
    }
}

// struct FlashType: romSize @0, sector.size @4, sector.shift @8 (GCC-probed).
unsafe fn flash_rom_size() -> u32 {
    unsafe { gFlash.cast::<u32>().read() }
}

unsafe fn flash_sector_shift() -> u32 {
    unsafe { u32::from(gFlash.cast::<u8>().add(8).read()) }
}

unsafe fn flash_sector_size() -> u32 {
    unsafe { gFlash.cast::<u8>().add(4).cast::<u32>().read() }
}

/// Copies `halfwords` halfwords of Thumb code starting at `func` into `buf`
/// and returns the copy as a callable (Thumb, hence +1).
unsafe fn copy_to_ram<F: Copy>(func: usize, buf: *mut u16, halfwords: usize) -> F {
    unsafe {
        let src = (func & !1) as *const u16;
        let mut i = 0usize;
        while i < halfwords {
            buf.add(i).write_volatile(src.add(i).read_volatile());
            i += 1;
        }
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
        core::mem::transmute_copy::<usize, F>(&(buf as usize + 1))
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn VerifyFlashSector_Core(
    mut src: *mut u8,
    mut tgt: *mut u8,
    mut size: u32,
) -> u32 {
    while ({
        let t1 = size;
        size -= 1;
        t1
    }) != 0
    {
        if *({
            let t3 = tgt;
            tgt = tgt.at(1);
            t3
        }) != *({
            let t5 = src;
            src = src.at(1);
            t5
        }) {
            return tgt.at(-1) as usize as u32;
        }
    }
    return 0;
}
// hand-written: tools/rustport/overrides/agb_flash/VerifyFlashSector.rs
// c2rs-uses: VerifyFlashSector_Core SwitchFlashBank
/// `VerifyFlashSector`: runs `VerifyFlashSector_Core` from a stack copy.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn VerifyFlashSector(sector_num: u16, src: *mut u8) -> u32 {
    unsafe {
        let mut sector_num = sector_num;
        flash_set_sram_wait_8();
        if flash_rom_size() == FLASH_ROM_SIZE_1M {
            SwitchFlashBank((sector_num / SECTORS_PER_BANK) as u8);
            sector_num %= SECTORS_PER_BANK;
        }
        let mut buffer = crate::ffi::Align4([0u16; 0x80]);
        let core: unsafe extern "C" fn(*mut u8, *mut u8, u32) -> u32 = copy_to_ram(
            VerifyFlashSector_Core as *const () as usize,
            buffer.0.as_mut_ptr(),
            0x80,
        );
        let tgt =
            FLASH_BASE_ADDR.wrapping_add((u32::from(sector_num) << flash_sector_shift()) as usize);
        // `u16 size = gFlash->sector.size;` truncates, as in C.
        let size = flash_sector_size() as u16;
        core(src, tgt as *mut u8, u32::from(size))
    }
}

// hand-written: tools/rustport/overrides/agb_flash/VerifyFlashSectorNBytes.rs
// c2rs-uses: VerifyFlashSector_Core SwitchFlashBank
/// `VerifyFlashSectorNBytes`: like `VerifyFlashSector` for the first `n`
/// bytes (note C switches bank before setting the wait state here).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn VerifyFlashSectorNBytes(sector_num: u16, src: *mut u8, n: u32) -> u32 {
    unsafe {
        let mut sector_num = sector_num;
        if flash_rom_size() == FLASH_ROM_SIZE_1M {
            SwitchFlashBank((sector_num / SECTORS_PER_BANK) as u8);
            sector_num %= SECTORS_PER_BANK;
        }
        flash_set_sram_wait_8();
        let mut buffer = crate::ffi::Align4([0u16; 0x80]);
        let core: unsafe extern "C" fn(*mut u8, *mut u8, u32) -> u32 = copy_to_ram(
            VerifyFlashSector_Core as *const () as usize,
            buffer.0.as_mut_ptr(),
            0x80,
        );
        let tgt =
            FLASH_BASE_ADDR.wrapping_add((u32::from(sector_num) << flash_sector_shift()) as usize);
        core(src, tgt as *mut u8, n)
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ProgramFlashSectorAndVerify(sectorNum: u16, src: *mut u8) -> u32 {
    let mut i: u8 = 0;
    let mut result: u32 = 0;
    i = 0;
    'l2: while i < 3 {
        'l1: {
            result = ProgramFlashSector.unwrap_unchecked()(sectorNum, src) as u32;
            if result != 0 {
                break 'l1;
            }
            result = VerifyFlashSector(sectorNum, src);
            if result == 0 {
                break 'l2;
            }
        }
        i += 1;
    }
    return result;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ProgramFlashSectorAndVerifyNBytes(
    sectorNum: u16,
    src: *mut u8,
    n: u32,
) -> u32 {
    let mut i: u8 = 0;
    let mut result: u32 = 0;
    i = 0;
    'l2: while i < 3 {
        'l1: {
            result = ProgramFlashSector.unwrap_unchecked()(sectorNum, src) as u32;
            if result != 0 {
                break 'l1;
            }
            result = VerifyFlashSectorNBytes(sectorNum, src, n);
            if result == 0 {
                break 'l2;
            }
        }
        i += 1;
    }
    return result;
}
