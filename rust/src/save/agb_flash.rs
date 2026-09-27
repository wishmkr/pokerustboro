//! Translated from `src/agb_flash.c` by tools/rustport/c2rs.py, then reviewed.
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

pub(crate) static mut sTimerNum: u8 = 0u8;
pub(crate) static mut sTimerCount: u16 = 0u16;
pub(crate) static mut sTimerReg: *mut u16 = core::ptr::null_mut();
pub(crate) static mut sSavedIme: u16 = 0u16;
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gFlashTimeoutFlag: u8 = 0u8;
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
pub static mut gFlash: *mut u8 = core::ptr::null_mut();
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut ProgramFlashByte: Option<unsafe extern "C" fn(u16, u32, u8) -> u16> = None;
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gFlashNumRemainingBytes: u16 = 0u16;
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut EraseFlashChip: Option<unsafe extern "C" fn() -> u16> = None;
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut EraseFlashSector: Option<unsafe extern "C" fn(u16) -> u16> = None;
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gFlashMaxTime: *mut u16 = core::ptr::null_mut();

#[unsafe(no_mangle)]
pub unsafe extern "C" fn SwitchFlashBank(bankNum: u8) {
    unsafe {
        let mut bankNum = bankNum;
        crate::c::volatile_write(
            ((234881024i32) as usize as *mut u8).wrapping_offset(21845),
            170u8,
        );
        crate::c::volatile_write(
            ((234881024i32) as usize as *mut u8).wrapping_offset(10922),
            85u8,
        );
        crate::c::volatile_write(
            ((234881024i32) as usize as *mut u8).wrapping_offset(21845),
            176u8,
        );
        crate::c::volatile_write(((234881024i32) as usize as *mut u8), bankNum);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ReadFlashId() -> u16 {
    unsafe {
        let mut flashId: u16 = 0u16;
        let mut readFlash1Buffer = crate::ffi::Align4([0u8; 64]);
        let mut readFlash1: Option<unsafe extern "C" fn(*mut u8) -> u8> = None;
        SetReadFlash1((&raw mut readFlash1Buffer).cast::<u16>());
        readFlash1 = (core::mem::transmute::<usize, Option<unsafe extern "C" fn(*mut u8) -> u8>>(
            ((((&raw mut readFlash1Buffer).cast::<u16>()) as usize as i32).wrapping_add(1i32))
                as usize,
        ));
        crate::c::volatile_write(
            ((234881024i32) as usize as *mut u8).wrapping_offset(21845),
            170u8,
        );
        crate::c::volatile_write(
            ((234881024i32) as usize as *mut u8).wrapping_offset(10922),
            85u8,
        );
        crate::c::volatile_write(
            ((234881024i32) as usize as *mut u8).wrapping_offset(21845),
            144u8,
        );
        'l1: loop {
            'l2: {
                let mut i: u16 = 0u16;
                (&raw mut i).write_volatile(0u16);
                {
                    crate::c::volatile_write((&raw mut i), 20000u16);
                    'l3: loop {
                        if !((((&raw mut i).read_volatile()) as i32) != 0i32) {
                            break 'l3;
                        }
                        'l4: {}
                        crate::c::volatile_write(
                            (&raw mut i),
                            ((&raw mut i).read_volatile()).wrapping_sub(1),
                        );
                    }
                }
            }
            if !((0i32) != 0) {
                break 'l1;
            }
        }
        flashId = (((((readFlash1).unwrap_unchecked()(
            ((234881024i32) as usize as *mut u8).wrapping_offset(1),
        )) as i32)
            << 8) as u16);
        flashId = ((((flashId) as i32)
            | (((readFlash1).unwrap_unchecked()(((234881024i32) as usize as *mut u8))) as i32))
            as u16);
        crate::c::volatile_write(
            ((234881024i32) as usize as *mut u8).wrapping_offset(21845),
            170u8,
        );
        crate::c::volatile_write(
            ((234881024i32) as usize as *mut u8).wrapping_offset(10922),
            85u8,
        );
        crate::c::volatile_write(
            ((234881024i32) as usize as *mut u8).wrapping_offset(21845),
            240u8,
        );
        crate::c::volatile_write(
            ((234881024i32) as usize as *mut u8).wrapping_offset(21845),
            240u8,
        );
        'l5: loop {
            'l6: {
                let mut i: u16 = 0u16;
                (&raw mut i).write_volatile(0u16);
                {
                    crate::c::volatile_write((&raw mut i), 20000u16);
                    'l7: loop {
                        if !((((&raw mut i).read_volatile()) as i32) != 0i32) {
                            break 'l7;
                        }
                        'l8: {}
                        crate::c::volatile_write(
                            (&raw mut i),
                            ((&raw mut i).read_volatile()).wrapping_sub(1),
                        );
                    }
                }
            }
            if !((0i32) != 0) {
                break 'l5;
            }
        }
        return flashId;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FlashTimerIntr() {
    unsafe {
        if (((((&raw mut sTimerCount).cast::<u8>().cast::<u16>()).read()) as i32) != 0i32)
            && ((({
                let __p1 = (&raw mut sTimerCount).cast::<u8>().cast::<u16>();
                let __t2 = ((__p1).read()).wrapping_sub(1);
                (__p1).write(__t2);
                __t2
            }) as i32)
                == 0i32)
        {
            ((&raw mut gFlashTimeoutFlag).cast::<u8>().cast::<u8>()).write(1u8);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetFlashTimerIntr(
    timerNum: u8,
    intrFunc: *mut Option<unsafe extern "C" fn()>,
) -> u16 {
    unsafe {
        let mut timerNum = timerNum;
        let mut intrFunc = intrFunc;
        if ((timerNum) as i32) >= 4i32 {
            return 1u16;
        }
        ((&raw mut sTimerNum).cast::<u8>().cast::<u8>()).write(timerNum);
        ((&raw mut sTimerReg).cast::<u8>().cast::<*mut u16>()).write(
            (((67109120i32).wrapping_add(
                ((((&raw mut sTimerNum).cast::<u8>().cast::<u8>()).read()) as i32)
                    .wrapping_mul(4i32),
            )) as usize as *mut u16),
        );
        (intrFunc).write(Some(FlashTimerIntr));
        return 0u16;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn StartFlashTimer(phase: u8) {
    unsafe {
        let mut phase = phase;
        let mut maxTime: *mut u16 = (((&raw mut gFlashMaxTime).cast::<u8>().cast::<*mut u16>())
            .read())
        .wrapping_offset((((phase) as i32).wrapping_mul(3i32)) as isize);
        ((&raw mut sSavedIme).cast::<u8>().cast::<u16>())
            .write(((67109384i32) as usize as *mut u16).read_volatile());
        crate::c::volatile_write(((67109384i32) as usize as *mut u16), 0u16);
        crate::c::volatile_write(
            (((&raw mut sTimerReg).cast::<u8>().cast::<*mut u16>()).read()).wrapping_offset(1),
            0u16,
        );
        let __p1 = ((67109376i32) as usize as *mut u16);
        crate::c::volatile_write(
            __p1,
            (((((__p1).read_volatile()) as i32)
                | crate::c::shl_i32(
                    8i32,
                    ((((&raw mut sTimerNum).cast::<u8>().cast::<u8>()).read()) as u32),
                )) as u16),
        );
        ((&raw mut gFlashTimeoutFlag).cast::<u8>().cast::<u8>()).write(0u8);
        ((&raw mut sTimerCount).cast::<u8>().cast::<u16>()).write(
            ({
                let __t3 = maxTime;
                maxTime = (maxTime).wrapping_offset(1);
                __t3
            })
            .read(),
        );
        crate::c::volatile_write(
            {
                let __p4 = (&raw mut sTimerReg).cast::<u8>().cast::<*mut u16>();
                let __t5 = (__p4).read();
                (__p4).write(((__p4).read()).wrapping_offset(1));
                __t5
            },
            ({
                let __t7 = maxTime;
                maxTime = (maxTime).wrapping_offset(1);
                __t7
            })
            .read(),
        );
        crate::c::volatile_write(
            {
                let __p8 = (&raw mut sTimerReg).cast::<u8>().cast::<*mut u16>();
                let __t9 = (__p8).read();
                (__p8).write(((__p8).read()).wrapping_offset(-1));
                __t9
            },
            ({
                let __t11 = maxTime;
                maxTime = (maxTime).wrapping_offset(1);
                __t11
            })
            .read(),
        );
        crate::c::volatile_write(
            ((67109378i32) as usize as *mut u16),
            ((crate::c::shl_i32(
                8i32,
                ((((&raw mut sTimerNum).cast::<u8>().cast::<u8>()).read()) as u32),
            )) as u16),
        );
        crate::c::volatile_write(((67109384i32) as usize as *mut u16), 1u16);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn StopFlashTimer() {
    unsafe {
        crate::c::volatile_write(((67109384i32) as usize as *mut u16), 0u16);
        crate::c::volatile_write(
            {
                let __p1 = (&raw mut sTimerReg).cast::<u8>().cast::<*mut u16>();
                let __t2 = (__p1).read();
                (__p1).write(((__p1).read()).wrapping_offset(1));
                __t2
            },
            0u16,
        );
        crate::c::volatile_write(
            {
                let __p3 = (&raw mut sTimerReg).cast::<u8>().cast::<*mut u16>();
                let __t4 = (__p3).read();
                (__p3).write(((__p3).read()).wrapping_offset(-1));
                __t4
            },
            0u16,
        );
        let __p5 = ((67109376i32) as usize as *mut u16);
        crate::c::volatile_write(
            __p5,
            (((((__p5).read_volatile()) as i32)
                & !(crate::c::shl_i32(
                    8i32,
                    ((((&raw mut sTimerNum).cast::<u8>().cast::<u8>()).read()) as u32),
                ))) as u16),
        );
        crate::c::volatile_write(
            ((67109384i32) as usize as *mut u16),
            ((&raw mut sSavedIme).cast::<u8>().cast::<u16>()).read(),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ReadFlash1(addr: *mut u8) -> u8 {
    unsafe {
        let mut addr = addr;
        return (addr).read();
    }
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
pub unsafe extern "C" fn ReadFlash_Core(src: *mut u8, dest: *mut u8, size: u32) {
    unsafe {
        let mut src = src;
        let mut dest = dest;
        let mut size = size;
        'l1: loop {
            if !({
                let __t1 = size;
                size = (size).wrapping_sub(1);
                __t1
            } != 0u32)
            {
                break 'l1;
            }
            ({
                let __t2 = dest;
                dest = (dest).wrapping_offset(1);
                __t2
            })
            .write(
                ({
                    let __t4 = src;
                    src = (src).wrapping_offset(1);
                    __t4
                })
                .read_volatile(),
            );
        }
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
    unsafe { u32::from(gFlash.add(8).read()) }
}

unsafe fn flash_sector_size() -> u32 {
    unsafe { gFlash.add(4).cast::<u32>().read() }
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
pub unsafe extern "C" fn VerifyFlashSector_Core(src: *mut u8, tgt: *mut u8, size: u32) -> u32 {
    unsafe {
        let mut src = src;
        let mut tgt = tgt;
        let mut size = size;
        'l1: loop {
            if !({
                let __t1 = size;
                size = (size).wrapping_sub(1);
                __t1
            } != 0u32)
            {
                break 'l1;
            }
            if ((({
                let __t3 = tgt;
                tgt = (tgt).wrapping_offset(1);
                __t3
            })
            .read()) as i32)
                != ((({
                    let __t5 = src;
                    src = (src).wrapping_offset(1);
                    __t5
                })
                .read()) as i32)
            {
                return (((tgt).wrapping_offset(-1)) as usize as u32);
            }
        }
        return 0u32;
    }
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
    unsafe {
        let mut sectorNum = sectorNum;
        let mut src = src;
        let mut i: u8 = 0u8;
        let mut result: u32 = 0u32;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 3i32) {
                    break 'l1;
                }
                'l2: {
                    result = (((((&raw mut ProgramFlashSector)
                        .cast::<u8>()
                        .cast::<Option<unsafe extern "C" fn(u16, *mut u8) -> u16>>())
                    .read())
                    .unwrap_unchecked()(sectorNum, src)) as u32);
                    if result != 0u32 {
                        break 'l2;
                    }
                    result = VerifyFlashSector(sectorNum, src);
                    if result == 0u32 {
                        break 'l1;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return result;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ProgramFlashSectorAndVerifyNBytes(
    sectorNum: u16,
    src: *mut u8,
    n: u32,
) -> u32 {
    unsafe {
        let mut sectorNum = sectorNum;
        let mut src = src;
        let mut n = n;
        let mut i: u8 = 0u8;
        let mut result: u32 = 0u32;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 3i32) {
                    break 'l1;
                }
                'l2: {
                    result = (((((&raw mut ProgramFlashSector)
                        .cast::<u8>()
                        .cast::<Option<unsafe extern "C" fn(u16, *mut u8) -> u16>>())
                    .read())
                    .unwrap_unchecked()(sectorNum, src)) as u32);
                    if result != 0u32 {
                        break 'l2;
                    }
                    result = VerifyFlashSectorNBytes(sectorNum, src, n);
                    if result == 0u32 {
                        break 'l1;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return result;
    }
}
