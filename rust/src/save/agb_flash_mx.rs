//! Translated from `src/agb_flash_mx.c` by tools/rustport/c2rs.py.
#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    static_mut_refs,
    unsafe_op_in_unsafe_fn,
    clashing_extern_declarations,
    unpredictable_function_pointer_comparisons,
    dangerous_implicit_autorefs,
    overflowing_literals
)]

use crate::agb_flash::{
    SetReadFlash1, SwitchFlashBank, WaitForFlashWrite, gFlash, gFlashNumRemainingBytes,
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
// Data tables (translate with cdata.py): mxMaxTime MX29L010 DefaultFlash

pub unsafe fn EraseFlashChip_MX() -> u16 {
    let mut readFlash1Buffer: CArray<u16, 32> = zeroed();
    volatile_write(
        67109380_usize as *mut u16,
        (67109380_usize as *mut u16).read_volatile() & 65532 | (*gFlash).wait[0],
    );
    volatile_write((0xE000000_usize as *mut u8).at(21845), 0xAA);
    volatile_write((0xE000000_usize as *mut u8).at(10922), 0x55);
    volatile_write((0xE000000_usize as *mut u8).at(21845), 0x80);
    volatile_write((0xE000000_usize as *mut u8).at(21845), 0xAA);
    volatile_write((0xE000000_usize as *mut u8).at(10922), 0x55);
    volatile_write((0xE000000_usize as *mut u8).at(21845), 0x10);
    SetReadFlash1(readFlash1Buffer.as_mut_ptr());
    let result: u16 = WaitForFlashWrite.unwrap_unchecked()(3, 0xE000000_usize as *mut u8, 0xFF);
    volatile_write(
        67109380_usize as *mut u16,
        (67109380_usize as *mut u16).read_volatile() & 65532 | 3,
    );
    result
}
// hand-written: tools/rustport/overrides/agb_flash_mx/EraseFlashSector_MX.rs
// c2rs-uses: gFlash SwitchFlashBank SetReadFlash1 WaitForFlashWrite
pub fn EraseFlashSector_MX(sector_num: u16) -> u16 {
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
                .cast::<Option<unsafe fn(u8, *mut u8, u8) -> u16>>()
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

pub unsafe fn ProgramFlashByte_MX(mut sectorNum: u16, offset: u32, data: u8) -> u16 {
    let mut readFlash1Buffer: CArray<u16, 32> = zeroed();
    if offset >= (*gFlash).sector.size {
        return 0x8000;
    }
    SwitchFlashBank((sectorNum as i32 / 16) as u8);
    sectorNum = (sectorNum as i32 % 16) as u16;
    let addr: *mut u8 = (0xE000000_usize as *mut u8)
        .at(shl_i32(sectorNum as i32, (*gFlash).sector.shift as u32))
        .at(offset);
    SetReadFlash1(readFlash1Buffer.as_mut_ptr());
    volatile_write(
        67109380_usize as *mut u16,
        (67109380_usize as *mut u16).read_volatile() & 65532 | (*gFlash).wait[0],
    );
    volatile_write((0xE000000_usize as *mut u8).at(21845), 0xAA);
    volatile_write((0xE000000_usize as *mut u8).at(10922), 0x55);
    volatile_write((0xE000000_usize as *mut u8).at(21845), 0xA0);
    *addr = data;
    WaitForFlashWrite.unwrap_unchecked()(1, addr, data)
}
unsafe fn ProgramByte(src: *mut u8, dest: *mut u8) -> u16 {
    volatile_write((0xE000000_usize as *mut u8).at(21845), 0xAA);
    volatile_write((0xE000000_usize as *mut u8).at(10922), 0x55);
    volatile_write((0xE000000_usize as *mut u8).at(21845), 0xA0);
    *dest = *src;
    WaitForFlashWrite.unwrap_unchecked()(1, dest, *src)
}
pub unsafe fn ProgramFlashSector_MX(mut sectorNum: u16, mut src: *mut u8) -> u16 {
    let mut readFlash1Buffer: CArray<u16, 32> = zeroed();
    if sectorNum >= (*gFlash).sector.count {
        return 0x80FF;
    }
    let mut result: u16 = EraseFlashSector_MX(sectorNum);
    if result != 0 {
        return result;
    }
    SwitchFlashBank((sectorNum as i32 / 16) as u8);
    sectorNum = (sectorNum as i32 % 16) as u16;
    SetReadFlash1(readFlash1Buffer.as_mut_ptr());
    volatile_write(
        67109380_usize as *mut u16,
        (67109380_usize as *mut u16).read_volatile() & 65532 | (*gFlash).wait[0],
    );
    gFlashNumRemainingBytes = (*gFlash).sector.size as u16;
    let mut dest: *mut u8 =
        (0xE000000_usize as *mut u8).at(shl_i32(sectorNum as i32, (*gFlash).sector.shift as u32));
    while gFlashNumRemainingBytes > 0 {
        result = ProgramByte(src, dest);
        if result != 0 {
            break;
        }
        gFlashNumRemainingBytes -= 1;
        src = src.at(1);
        dest = dest.at(1);
    }
    result
}
