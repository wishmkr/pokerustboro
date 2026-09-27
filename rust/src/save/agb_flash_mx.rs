//! Translated from `src/agb_flash_mx.c` by tools/rustport/c2rs.py, then reviewed.
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

// Data tables (translate with cdata.py): mxMaxTime MX29L010 DefaultFlash
#[allow(unused_imports)]
use crate::data::agb_flash_mx::*;

unsafe extern "C" {
    static mut WaitForFlashWrite: u8;
    static mut gFlash: u8;
    static mut gFlashNumRemainingBytes: u8;
    fn SetReadFlash1(a0: *mut u16);
    fn SwitchFlashBank(a0: u8);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn EraseFlashChip_MX() -> u16 {
    unsafe {
        let mut result: u16 = 0u16;
        let mut readFlash1Buffer = crate::ffi::Align4([0u8; 64]);
        crate::c::volatile_write(
            ((67109380i32) as usize as *mut u16),
            (((((((67109380i32) as usize as *mut u16).read_volatile()) as i32) & (-4i32))
                | (((((((&raw mut gFlash).cast::<*mut u8>()).read()).wrapping_add(16))
                    .cast::<u16>())
                .read()) as i32)) as u16),
        );
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
            128u8,
        );
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
            16u8,
        );
        SetReadFlash1((&raw mut readFlash1Buffer).cast::<u16>());
        result = (((&raw mut WaitForFlashWrite)
            .cast::<Option<unsafe extern "C" fn(u8, *mut u8, u8) -> u16>>())
        .read())
        .unwrap_unchecked()(3u8, ((234881024i32) as usize as *mut u8), 255u8);
        crate::c::volatile_write(
            ((67109380i32) as usize as *mut u16),
            (((((((67109380i32) as usize as *mut u16).read_volatile()) as i32) & (-4i32)) | 3i32)
                as u16),
        );
        return result;
    }
}
// hand-written: tools/rustport/overrides/agb_flash_mx/EraseFlashSector_MX.rs
// c2rs-uses: SetReadFlash1 SwitchFlashBank gFlash WaitForFlashWrite
/// `EraseFlashSector_MX`. The C retries with `goto try_erase` and leaves
/// with `goto done`; here that is a `loop` with `break`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn EraseFlashSector_MX(sector_num: u16) -> u16 {
    const FLASH_BASE: usize = 0x0E00_0000;
    const REG_WAITCNT: *mut u16 = 0x0400_0204 as *mut u16;
    const WAITCNT_SRAM_MASK: u16 = 3;
    const WAITCNT_SRAM_8: u16 = 3;
    const SECTORS_PER_BANK: u16 = 16;
    unsafe {
        let flash = (&raw mut gFlash).cast::<*mut u8>().read();
        // struct FlashType: sector.shift @8, sector.count @10, wait[0] @16
        let sector_count = flash.add(10).cast::<u16>().read();
        if sector_num >= sector_count {
            return 0x80FF;
        }
        SwitchFlashBank((sector_num / SECTORS_PER_BANK) as u8);
        let sector_num = sector_num % SECTORS_PER_BANK;

        let flash_write =
            |addr: usize, data: u8| crate::c::volatile_write((FLASH_BASE + addr) as *mut u8, data);
        let mut read_flash1_buffer = crate::ffi::Align4([0u16; 0x20]);
        let mut num_tries: u16 = 0;
        let result = loop {
            let wait0 = flash.add(16).cast::<u16>().read();
            let v = REG_WAITCNT.read_volatile();
            crate::c::volatile_write(REG_WAITCNT, (v & !WAITCNT_SRAM_MASK) | wait0);

            let shift = u32::from(flash.add(8).read());
            let addr = (FLASH_BASE + ((u32::from(sector_num) << shift) as usize)) as *mut u8;

            flash_write(0x5555, 0xAA);
            flash_write(0x2AAA, 0x55);
            flash_write(0x5555, 0x80);
            flash_write(0x5555, 0xAA);
            flash_write(0x2AAA, 0x55);
            crate::c::volatile_write(addr, 0x30);

            SetReadFlash1(read_flash1_buffer.0.as_mut_ptr());

            let wait_for_write = (&raw mut WaitForFlashWrite)
                .cast::<Option<unsafe extern "C" fn(u8, *mut u8, u8) -> u16>>()
                .read();
            let result = wait_for_write.unwrap_unchecked()(2, addr, 0xFF);

            if result & 0xA000 == 0 || num_tries > 3 {
                break result; // goto done
            }
            num_tries += 1; // goto try_erase
        };

        let v = REG_WAITCNT.read_volatile();
        crate::c::volatile_write(REG_WAITCNT, (v & !WAITCNT_SRAM_MASK) | WAITCNT_SRAM_8);
        result
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ProgramFlashByte_MX(sectorNum: u16, offset: u32, data: u8) -> u16 {
    unsafe {
        let mut sectorNum = sectorNum;
        let mut offset = offset;
        let mut data = data;
        let mut addr: *mut u8 = core::ptr::null_mut();
        let mut readFlash1Buffer = crate::ffi::Align4([0u8; 64]);
        if offset
            >= (((((&raw mut gFlash).cast::<*mut u8>()).read()).wrapping_add(4)).cast::<u32>())
                .read()
        {
            return 32768u16;
        }
        SwitchFlashBank(((crate::c::div_i32(((sectorNum) as i32), 16i32)) as u8));
        sectorNum = ((crate::c::rem_i32(((sectorNum) as i32), 16i32)) as u16);
        addr = (((234881024i32) as usize as *mut u8).wrapping_offset(
            (crate::c::shl_i32(
                ((sectorNum) as i32),
                (((((((&raw mut gFlash).cast::<*mut u8>()).read()).wrapping_add(4))
                    .wrapping_add(4))
                .read()) as u32),
            )) as isize,
        ))
        .wrapping_offset(((offset) as i32) as isize);
        SetReadFlash1((&raw mut readFlash1Buffer).cast::<u16>());
        crate::c::volatile_write(
            ((67109380i32) as usize as *mut u16),
            (((((((67109380i32) as usize as *mut u16).read_volatile()) as i32) & (-4i32))
                | (((((((&raw mut gFlash).cast::<*mut u8>()).read()).wrapping_add(16))
                    .cast::<u16>())
                .read()) as i32)) as u16),
        );
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
            160u8,
        );
        (addr).write(data);
        return (((&raw mut WaitForFlashWrite)
            .cast::<Option<unsafe extern "C" fn(u8, *mut u8, u8) -> u16>>())
        .read())
        .unwrap_unchecked()(1u8, addr, data);
    }
}
pub(crate) unsafe extern "C" fn ProgramByte(src: *mut u8, dest: *mut u8) -> u16 {
    unsafe {
        let mut src = src;
        let mut dest = dest;
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
            160u8,
        );
        (dest).write((src).read());
        return (((&raw mut WaitForFlashWrite)
            .cast::<Option<unsafe extern "C" fn(u8, *mut u8, u8) -> u16>>())
        .read())
        .unwrap_unchecked()(1u8, dest, (src).read());
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ProgramFlashSector_MX(sectorNum: u16, src: *mut u8) -> u16 {
    unsafe {
        let mut sectorNum = sectorNum;
        let mut src = src;
        let mut result: u16 = 0u16;
        let mut dest: *mut u8 = core::ptr::null_mut();
        let mut readFlash1Buffer = crate::ffi::Align4([0u8; 64]);
        if ((sectorNum) as i32)
            >= (((((((&raw mut gFlash).cast::<*mut u8>()).read()).wrapping_add(4))
                .wrapping_add(6)
                .cast::<u16>())
            .read()) as i32)
        {
            return 33023u16;
        }
        result = EraseFlashSector_MX(sectorNum);
        if ((result) as i32) != 0i32 {
            return result;
        }
        SwitchFlashBank(((crate::c::div_i32(((sectorNum) as i32), 16i32)) as u8));
        sectorNum = ((crate::c::rem_i32(((sectorNum) as i32), 16i32)) as u16);
        SetReadFlash1((&raw mut readFlash1Buffer).cast::<u16>());
        crate::c::volatile_write(
            ((67109380i32) as usize as *mut u16),
            (((((((67109380i32) as usize as *mut u16).read_volatile()) as i32) & (-4i32))
                | (((((((&raw mut gFlash).cast::<*mut u8>()).read()).wrapping_add(16))
                    .cast::<u16>())
                .read()) as i32)) as u16),
        );
        ((&raw mut gFlashNumRemainingBytes).cast::<u16>()).write(
            (((((((&raw mut gFlash).cast::<*mut u8>()).read()).wrapping_add(4)).cast::<u32>())
                .read()) as u16),
        );
        dest = ((234881024i32) as usize as *mut u8).wrapping_offset(
            (crate::c::shl_i32(
                ((sectorNum) as i32),
                (((((((&raw mut gFlash).cast::<*mut u8>()).read()).wrapping_add(4))
                    .wrapping_add(4))
                .read()) as u32),
            )) as isize,
        );
        'l1: loop {
            if !(((((&raw mut gFlashNumRemainingBytes).cast::<u16>()).read()) as i32) > 0i32) {
                break 'l1;
            }
            result = ProgramByte(src, dest);
            if ((result) as i32) != 0i32 {
                break 'l1;
            }
            let __p1 = (&raw mut gFlashNumRemainingBytes).cast::<u16>();
            (__p1).write(((__p1).read()).wrapping_sub(1));
            src = (src).wrapping_offset(1);
            dest = (dest).wrapping_offset(1);
        }
        return result;
    }
}
