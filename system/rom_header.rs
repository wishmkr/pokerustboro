//! The cartridge header at 0x08000000 (was src/rom_header.s).
//!
//! Title, codes and checksum are left zero here and filled in after linking
//! by `gbafix` (see the Makefile), as before. The header's symbols
//! (`RomHeaderGameCode`, `GPIOPortDirection`...) are fixed addresses defined
//! in ld_script_modern.ld.
//!
//! The first word must be a branch (`b`, 0xEA..: emulators and flash carts
//! check for it) that reaches the start-up code. The original `b Init` is
//! PC-relative to a symbol and needs the assembler; as data it is a fixed
//! `b 0xD0` into the header's unused padding, where `ldr pc, [pc, #-4]`
//! jumps to the address stored right after it.

use crate::ffi::RomPtr;

#[repr(C)]
pub struct RomHeader {
    /// `b 0xD0`
    entry: u32,
    /// Nintendo logo, title, codes, version, checksum (filled by gbafix).
    body: [u8; 0xBC],
    /// 0xC0: unused word, then the GPIO port registers (the RTC) at 0xC4.
    gpio: [u8; 0x10],
    /// 0xD0: `ldr pc, [pc, #-4]`, i.e. jump to `init`.
    trampoline: u32,
    /// 0xD4
    init: RomPtr<u8>,
    rest: [u8; 0x28],
}

#[unsafe(link_section = ".text.rom_header")]
#[used]
static ROM_HEADER: RomHeader = RomHeader {
    entry: 0xEA00_0000 | ((0xD0 - 8) / 4),
    body: [0; 0xBC],
    gpio: [0; 0x10],
    trampoline: 0xE51F_F004,
    init: RomPtr(crate::crt0::Init as *const u8),
    rest: [0; 0x28],
};

#[cfg(target_arch = "arm")]
const _: () = assert!(core::mem::size_of::<RomHeader>() == 0x100);
