//! Receiving and booting a multiboot image from a GameCube over the link
//! cable's JOY Bus (was src/libgcnmultiboot.s).
//!
//! `struct GameCubeMultiBoot` (include/gba/gcn_multiboot.h) is used through
//! byte offsets, as in the assembly. The serial interrupt runs a state
//! machine whose next step the assembly kept as a code address in the
//! struct (`serialIntrHandler`); here that word holds a Rust function
//! pointer instead. The rest of the game only tests it against zero.

// struct GameCubeMultiBoot
const COUNTER1: usize = 0x00;
const COUNTER2: usize = 0x01;
const MBPROGRESS: usize = 0x02;
const SAVEDVCOUNT: usize = 0x03;
const KEYA: usize = 0x04;
const KEYB: usize = 0x08;
const KEYC: usize = 0x0C;
const BOOT_KEY: usize = 0x10;
const IMAGE_SIZE: usize = 0x12;
const SESSION_KEY: usize = 0x14;
const HASH_VAL: usize = 0x18;
const KEYC_DERIVATION: usize = 0x1C;
const BASE_DEST_PTR: usize = 0x20;
const CUR_DEST_PTR: usize = 0x24;
const SERIAL_INTR_HANDLER: usize = 0x28;

const LOGO_OFFSET: u32 = 0x04;
const LOGO_LENGTH: u32 = 0x98;
const LOGO_END: u32 = 0xA0;

const MBPROGRESS_NONE: u8 = 0;
const MBPROGRESS_LOGO_CORRECT: u8 = 1;
const MBPROGRESS_READY_TO_BOOT: u8 = 2;

const MAGIC_BOOTKEY_HASHVAL: u32 = 0xBB;
const MAGIC_BOOTKEY: u32 = 0xBB;
const MAGIC_COUNTER2: u32 = 0xCC;
const MAGIC_KEYA: u32 = 0xDD;
const MAGIC_KEYB: u32 = 0xEE;
const MAGIC_KEYC_DERIVATION: u32 = 0xFF;

const HASH_POLY: u32 = 0xA1C1;
/// "Kawa" (the BIOS author's name), the multiboot session-key multiplier.
const KAWA: u32 = u32::from_le_bytes(*b"Kawa");
/// "AXVE": Ruby (USA)'s game code, which the GameCube must echo.
const RUBY_USA_GAME_CODE: u32 = u32::from_le_bytes(*b"AXVE");
const MAXIMUM_IMAGE_SIZE_U32S: u32 = 0x4000;
const MULTIBOOT_LOAD_ADDR: u32 = 0x0200_0000; // EWRAM_START

const REG_IE: *mut u16 = 0x0400_0200 as *mut u16;
const REG_IF: *mut u16 = 0x0400_0202 as *mut u16;
const REG_IME: *mut u16 = 0x0400_0208 as *mut u16;
const REG_VCOUNT: *const u16 = 0x0400_0006 as *const u16;
const REG_RCNT: *mut u16 = 0x0400_0134 as *mut u16;
const REG_JOYCNT: *mut u16 = 0x0400_0140 as *mut u16;
const REG_JOY_RECV: *mut u32 = 0x0400_0150 as *mut u32;
const REG_JOY_TRANS: *mut u32 = 0x0400_0154 as *mut u32;
const REG_JOYSTAT: *mut u16 = 0x0400_0158 as *mut u16;
const INTR_FLAG_SERIAL: u16 = 0x80;

type Handler = unsafe extern "C" fn(*mut u8, u32);

unsafe fn get8(mb: *mut u8, off: usize) -> u8 {
    unsafe { mb.add(off).read_volatile() }
}
unsafe fn set8(mb: *mut u8, off: usize, v: u8) {
    unsafe { mb.add(off).write_volatile(v) }
}
unsafe fn get16(mb: *mut u8, off: usize) -> u16 {
    unsafe { mb.add(off).cast::<u16>().read_volatile() }
}
unsafe fn set16(mb: *mut u8, off: usize, v: u16) {
    unsafe { mb.add(off).cast::<u16>().write_volatile(v) }
}
unsafe fn get32(mb: *mut u8, off: usize) -> u32 {
    unsafe { mb.add(off).cast::<u32>().read_volatile() }
}
unsafe fn set32(mb: *mut u8, off: usize, v: u32) {
    unsafe { mb.add(off).cast::<u32>().write_volatile(v) }
}
unsafe fn wr16(reg: *mut u16, v: u16) {
    unsafe { crate::c::volatile_write(reg, v) }
}
unsafe fn wr32(reg: *mut u32, v: u32) {
    unsafe { crate::c::volatile_write(reg, v) }
}

/// `GameCubeMultiBoot_Hash`: 32 rounds of a CRC-like shift/xor.
fn hash(value: u32, data: u32) -> u32 {
    let mut h = value ^ data;
    for _ in 0..32 {
        let carry = h & 1 != 0;
        h >>= 1;
        if carry {
            h ^= HASH_POLY;
        }
    }
    h
}

/// `void GameCubeMultiBoot_Main(struct GameCubeMultiBoot *mb)`, once a frame.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GameCubeMultiBoot_Main(mb: *mut u8) {
    unsafe {
        // The assembly branches to Init on the carry flag left by these
        // counters: always when there is no handler yet, and when counter1
        // has reached 11 frames without a serial interrupt resetting it.
        let reinit = if get32(mb, SERIAL_INTR_HANDLER) == 0 {
            true
        } else {
            set8(mb, COUNTER2, get8(mb, COUNTER2).wrapping_add(1));
            if get8(mb, MBPROGRESS) == MBPROGRESS_READY_TO_BOOT {
                return;
            }
            let ime = REG_IME.read_volatile();
            wr16(REG_IME, 0);
            let counter1 = get8(mb, COUNTER1);
            let stale = counter1 > 10; // ldrb zero-extends; `bgt` on 0..255
            if !stale {
                set8(mb, COUNTER1, counter1.wrapping_add(1));
            }
            wr16(REG_IME, ime);
            stale
        };
        if reinit {
            GameCubeMultiBoot_Init(mb);
            return;
        }

        if get8(mb, MBPROGRESS) == MBPROGRESS_NONE {
            // Check the Nintendo logo in the received header against ours.
            let base = get32(mb, BASE_DEST_PTR);
            let received = get32(mb, CUR_DEST_PTR).wrapping_sub(base);
            if received == 0 || received < LOGO_END {
                return;
            }
            let mut theirs = (base + LOGO_OFFSET) as *const u32;
            let mut ours = (&raw const RomHeaderNintendoLogo).cast::<u32>();
            let mut left = LOGO_LENGTH;
            let mut matched = true;
            while left != 0 {
                let (a, b) = (theirs.read_volatile(), ours.read_volatile());
                theirs = theirs.add(1);
                ours = ours.add(1);
                if a != b {
                    matched = false;
                    break;
                }
                left -= 4;
            }
            if matched {
                // and the first three bytes of the word after it
                let (a, b) = (theirs.read_volatile(), ours.read_volatile());
                theirs = theirs.add(1);
                matched = (a ^ b) >> 8 == 0;
                set32(mb, BASE_DEST_PTR, theirs as u32);
            }
            if !matched {
                GameCubeMultiBoot_Init(mb);
                return;
            }
            set8(mb, MBPROGRESS, MBPROGRESS_LOGO_CORRECT);
            let h = get32(mb, KEYA) ^ get32(mb, KEYB);
            set32(mb, HASH_VAL, h);
            set32(mb, SESSION_KEY, h.wrapping_mul(KAWA).wrapping_add(1));
            return;
        }

        // The logo matched: decrypt and hash what has arrived since.
        let cur = get32(mb, CUR_DEST_PTR);
        let mut h = get32(mb, HASH_VAL);
        let mut p = get32(mb, BASE_DEST_PTR);
        let mut session = get32(mb, SESSION_KEY);
        while p < cur {
            let word = p as *mut u32;
            let plain = (word.read_volatile() ^ session).wrapping_add(h);
            word.write_volatile(plain);
            p += 4;
            h = hash(h, plain);
            session = session.wrapping_mul(KAWA).wrapping_add(1);
        }
        set32(mb, BASE_DEST_PTR, p);
        set32(mb, SESSION_KEY, session);
        set32(mb, HASH_VAL, h);

        if get16(mb, IMAGE_SIZE) != 0 || get32(mb, CUR_DEST_PTR) != get32(mb, BASE_DEST_PTR) {
            return;
        }
        let key_c = get32(mb, KEYC);
        if key_c == 0 {
            // KeyC = (SavedVCount << 24) - 1; send its hash with the image's.
            let key_c = (u32::from(get8(mb, SAVEDVCOUNT)) << 24).wrapping_sub(1);
            set32(mb, KEYC, key_c);
            let derivation = hash(h, key_c);
            set32(
                mb,
                KEYC_DERIVATION,
                (derivation << 8).wrapping_add(MAGIC_KEYC_DERIVATION),
            );
            return;
        }
        if get16(mb, BOOT_KEY) == 0 {
            return;
        }
        let boot_key = hash(key_c, MAGIC_BOOTKEY_HASHVAL);
        if u32::from(get16(mb, BOOT_KEY)) != boot_key {
            GameCubeMultiBoot_Init(mb);
            return;
        }
        set8(mb, MBPROGRESS, MBPROGRESS_READY_TO_BOOT);
    }
}

/// `void GameCubeMultiBoot_ExecuteProgram(struct GameCubeMultiBoot *mb)`:
/// jump (in ARM state) past the received image's header; never returns
/// when an image is ready.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GameCubeMultiBoot_ExecuteProgram(mb: *mut u8) {
    unsafe {
        if get8(mb, MBPROGRESS) != MBPROGRESS_READY_TO_BOOT {
            return;
        }
        wr16(REG_IME, 0);
        let entry: unsafe extern "C" fn() -> ! =
            core::mem::transmute((MULTIBOOT_LOAD_ADDR + 0xC0) as usize);
        entry();
    }
}

/// `void GameCubeMultiBoot_Init(struct GameCubeMultiBoot *mb)`
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GameCubeMultiBoot_Init(mb: *mut u8) {
    unsafe {
        let ime = REG_IME.read_volatile();
        wr16(REG_IME, 0);
        // Until the first command is a device reset, every interrupt stops.
        set32(
            mb,
            SERIAL_INTR_HANDLER,
            handler_stop as Handler as usize as u32,
        );
        let saved_vcount = get8(mb, SAVEDVCOUNT);
        let counter2 = get8(mb, COUNTER2);
        // Clear everything before the destination pointers...
        let mut off = 0;
        while off < BASE_DEST_PTR {
            set32(mb, off, 0);
            off += 4;
        }
        // ...and keep two bytes of entropy, swapped as the assembly does.
        set8(mb, SAVEDVCOUNT, counter2 >> 1);
        set8(mb, COUNTER2, saved_vcount);

        wr16(REG_RCNT, 0x8000); // JOY Bus off
        wr16(REG_RCNT, 0xC000); // JOY Bus on
        wr16(REG_JOYCNT, 0x47);
        wr16(REG_JOYSTAT, 0);
        wr16(REG_IF, INTR_FLAG_SERIAL);
        wr16(REG_IE, REG_IE.read_volatile() | INTR_FLAG_SERIAL);
        wr16(REG_IME, ime);
    }
}

/// `void GameCubeMultiBoot_Quit(void)`
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GameCubeMultiBoot_Quit() {
    unsafe {
        let ime = REG_IME.read_volatile();
        wr16(REG_IME, 0);
        wr16(REG_JOYCNT, 0x7);
        wr16(REG_RCNT, 0x8000);
        wr16(REG_IF, INTR_FLAG_SERIAL);
        wr16(REG_IE, REG_IE.read_volatile() & !INTR_FLAG_SERIAL);
        wr16(REG_IME, ime);
    }
}

/// `void GameCubeMultiBoot_HandleSerialInterrupt(struct GameCubeMultiBoot *mb)`
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GameCubeMultiBoot_HandleSerialInterrupt(mb: *mut u8) {
    unsafe {
        // Acknowledge reset/receive/send.
        let joycnt = u32::from(REG_JOYCNT.read_volatile());
        wr16(REG_JOYCNT, joycnt as u16);
        set8(mb, COUNTER1, 0);
        let handler = get32(mb, SERIAL_INTR_HANDLER);
        if handler == 0 {
            return;
        }
        if joycnt & 1 != 0 {
            begin_handshake(mb);
            return;
        }
        // flags >> 1: bit 0 = receive complete, bit 1 = send complete
        let step: Handler = core::mem::transmute(handler as usize);
        step(mb, joycnt >> 1);
    }
}

fn received(flags: u32) -> bool {
    flags & 1 != 0
}

fn sent(flags: u32) -> bool {
    flags & 2 != 0
}

/// Next step: `handler`, and note the scanline (entropy for the keys).
unsafe fn set_handler(mb: *mut u8, handler: Option<Handler>) {
    unsafe {
        set32(
            mb,
            SERIAL_INTR_HANDLER,
            handler.map_or(0, |h| h as usize as u32),
        );
        set8(mb, SAVEDVCOUNT, REG_VCOUNT.read_volatile() as u8);
    }
}

/// `GcMbIntrHandler_Stop`: no more commands until Init.
unsafe fn stop(mb: *mut u8) {
    unsafe {
        wr16(REG_JOYSTAT, 0);
        set_handler(mb, None);
    }
}

unsafe extern "C" fn handler_stop(mb: *mut u8, _flags: u32) {
    unsafe { stop(mb) }
}

unsafe fn begin_handshake(mb: *mut u8) {
    unsafe {
        let _ = REG_JOY_RECV.read_volatile(); // throw away anything received
        wr32(REG_JOY_TRANS, RUBY_USA_GAME_CODE);
        wr16(REG_JOYSTAT, 0x10);
        set8(mb, KEYB + 1, get8(mb, SAVEDVCOUNT));
        if get8(mb, MBPROGRESS) != 0 {
            stop(mb);
            return;
        }
        set32(mb, BASE_DEST_PTR, MULTIBOOT_LOAD_ADDR);
        set32(mb, CUR_DEST_PTR, MULTIBOOT_LOAD_ADDR);
        set_handler(mb, Some(handler_check_game_code_sent));
    }
}

unsafe extern "C" fn handler_check_game_code_sent(mb: *mut u8, flags: u32) {
    unsafe {
        if !sent(flags) {
            stop(mb);
        } else if received(flags) {
            check_handshake_response(mb);
        } else {
            set_handler(mb, Some(handler_check_handshake_response));
        }
    }
}

unsafe extern "C" fn handler_check_handshake_response(mb: *mut u8, flags: u32) {
    unsafe {
        if !received(flags) {
            stop(mb);
        } else {
            check_handshake_response(mb);
        }
    }
}

unsafe fn check_handshake_response(mb: *mut u8) {
    unsafe {
        if REG_JOY_RECV.read_volatile() != RUBY_USA_GAME_CODE {
            stop(mb);
            return;
        }
        set8(mb, KEYB + 3, get8(mb, SAVEDVCOUNT));
        set_handler(mb, Some(handler_receive_key_a));
    }
}

unsafe extern "C" fn handler_receive_key_a(mb: *mut u8, flags: u32) {
    unsafe {
        if !received(flags) {
            stop(mb);
            return;
        }
        let key_a = REG_JOY_RECV.read_volatile();
        if key_a >> 24 != MAGIC_KEYA {
            stop(mb);
            return;
        }
        set32(mb, KEYA, key_a);
        set8(mb, KEYB + 2, get8(mb, COUNTER2));
        // KeyB must have between 7 and 14 set bits in its top three bytes
        // (the other side checks); if not, replace the byte just set.
        let ones = (get32(mb, KEYB) >> 8).count_ones();
        if ones > 14 {
            set8(mb, KEYB + 2, 0);
        } else if ones < 7 {
            set8(mb, KEYB + 2, 0xFF);
        }
        wr32(REG_JOY_TRANS, get32(mb, KEYB).wrapping_add(MAGIC_KEYB));
        wr16(REG_JOYSTAT, 0x30);
        set_handler(mb, Some(handler_check_key_b_sent));
    }
}

unsafe extern "C" fn handler_check_key_b_sent(mb: *mut u8, flags: u32) {
    unsafe {
        if !sent(flags) {
            stop(mb);
        } else if received(flags) {
            check_image_size_response(mb);
        } else {
            set_handler(mb, Some(handler_check_image_size_response));
        }
    }
}

unsafe extern "C" fn handler_check_image_size_response(mb: *mut u8, flags: u32) {
    unsafe {
        if !received(flags) {
            stop(mb);
        } else {
            check_image_size_response(mb);
        }
    }
}

unsafe fn check_image_size_response(mb: *mut u8) {
    unsafe {
        let size = REG_JOY_RECV.read_volatile();
        if size >= MAXIMUM_IMAGE_SIZE_U32S {
            stop(mb);
            return;
        }
        set16(
            mb,
            IMAGE_SIZE,
            (size.wrapping_add(1).wrapping_mul(2)) as u16,
        );
        if get8(mb, MBPROGRESS) != 0 {
            stop(mb);
            return;
        }
        set32(mb, BASE_DEST_PTR, MULTIBOOT_LOAD_ADDR);
        set32(mb, CUR_DEST_PTR, MULTIBOOT_LOAD_ADDR);
        set_handler(mb, Some(handler_check_image_response));
    }
}

unsafe extern "C" fn handler_check_image_response(mb: *mut u8, flags: u32) {
    unsafe {
        if !received(flags) {
            stop(mb);
            return;
        }
        let dest = get32(mb, CUR_DEST_PTR);
        wr16(REG_JOYSTAT, (((dest & 4) + 8) << 2) as u16);
        (dest as *mut u32).write_volatile(REG_JOY_RECV.read_volatile());
        set32(mb, CUR_DEST_PTR, dest + 4);
        let left = get16(mb, IMAGE_SIZE).wrapping_sub(1);
        set16(mb, IMAGE_SIZE, left);
        if left != 0 {
            // keep this handler
            set8(mb, SAVEDVCOUNT, REG_VCOUNT.read_volatile() as u8);
            return;
        }
        send_counter2(mb);
    }
}

unsafe fn send_counter2(mb: *mut u8) {
    unsafe {
        wr32(
            REG_JOY_TRANS,
            (u32::from(get8(mb, COUNTER2)) << 8).wrapping_add(MAGIC_COUNTER2),
        );
        set_handler(mb, Some(handler_check_counter2_sent));
    }
}

unsafe extern "C" fn handler_check_counter2_sent(mb: *mut u8, flags: u32) {
    unsafe {
        if !sent(flags) {
            stop(mb);
            return;
        }
        // Keep sending counter2 until Main has made the KeyC derivation.
        let derivation = get32(mb, KEYC_DERIVATION);
        if derivation == 0 {
            send_counter2(mb);
            return;
        }
        wr32(REG_JOY_TRANS, derivation);
        set_handler(mb, Some(handler_check_key_c_derivation_sent));
    }
}

unsafe extern "C" fn handler_check_key_c_derivation_sent(mb: *mut u8, flags: u32) {
    unsafe {
        if !sent(flags) {
            stop(mb);
        } else if received(flags) {
            check_boot_key_response(mb);
        } else {
            set_handler(mb, Some(handler_check_boot_key_response));
        }
    }
}

unsafe extern "C" fn handler_check_boot_key_response(mb: *mut u8, flags: u32) {
    unsafe {
        if !received(flags) {
            stop(mb);
        } else {
            check_boot_key_response(mb);
        }
    }
}

unsafe fn check_boot_key_response(mb: *mut u8) {
    unsafe {
        let key = REG_JOY_RECV.read_volatile();
        if key >> 24 != MAGIC_BOOTKEY {
            stop(mb);
            return;
        }
        set16(mb, BOOT_KEY, key as u16);
        // anything more is an error
        set_handler(mb, Some(handler_stop));
    }
}

unsafe extern "C" {
    /// The Nintendo logo in our own cartridge header (ld_script_modern.ld).
    static RomHeaderNintendoLogo: u8;
}

#[cfg(test)]
mod tests {
    #[test]
    fn hash_matches_the_assembly_loop() {
        // One round by hand: value 1 ^ data 0 shifts out a 1 -> xor poly.
        let mut h: u32 = 1;
        for _ in 0..32 {
            let c = h & 1 != 0;
            h >>= 1;
            if c {
                h ^= super::HASH_POLY;
            }
        }
        assert_eq!(super::hash(1, 0), h);
        assert_eq!(super::KAWA, 0x6177_614B);
        assert_eq!(super::RUBY_USA_GAME_CODE, 0x4556_5841);
    }
}
