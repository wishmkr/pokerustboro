//! Translated from `src/multiboot.c` by tools/rustport/c2rs.py.
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

pub(crate) static mut MultiBoot_required_data: Aligned<CArray<u16, 3>> =
    Aligned(unsafe { zeroed() });

unsafe extern "C" {
    fn MultiBoot(a0: *mut MultiBootParam) -> i32;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn MultiBootInit(mp: *mut MultiBootParam) {
    (*mp).client_bit = 0;
    (*mp).probe_count = 0;
    (*mp).response_bit = 0;
    (*mp).check_wait = MULTIBOOT_CONNECTION_CHECK_WAIT;
    (*mp).sendflag = 0;
    (*mp).handshake_timeout = 0;
    volatile_write(67109172 as usize as *mut u16, 0);
    volatile_write(67109160 as usize as *mut u16, 8195);
    volatile_write(67109162 as usize as *mut u16, 0);
}
// hand-written: tools/rustport/overrides/multiboot/MultiBootMain.rs
// c2rs-uses: MultiBootCheckComplete MultiBootInit MultiBootHandShake MultiBootWaitSendDone MultiBootStartProbe MultiBootSend MultiBoot
/// `MultiBootMain`. The C jumps between switch cases: `case 0` into
/// `case 1` (`goto case_1`), several cases to the shared tails
/// `output_master_info` / `output_header`, and back to the top
/// (`goto output_burst`). Here the shared tails are the `Tail` steps and
/// the jump back is `continue 'output_burst`.
/// struct MultiBootParam offsets are GCC-probed (see the constants).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MultiBootMain(mp: *mut u8) -> i32 {
    const HANDSHAKE_DATA: usize = 0x14;
    const HANDSHAKE_TIMEOUT: usize = 0x16;
    const PROBE_COUNT: usize = 0x18;
    const CLIENT_DATA: usize = 0x19;
    const PALETTE_DATA: usize = 0x1C;
    const RESPONSE_BIT: usize = 0x1D;
    const CLIENT_BIT: usize = 0x1E;
    const MASTERP: usize = 0x28;
    const SENDFLAG: usize = 0x48;
    const PROBE_TARGET_BIT: usize = 0x49;
    const CHECK_WAIT: usize = 0x4A;
    const SERVER_TYPE: usize = 0x4B;

    const MULTIBOOT_NCHILD: i32 = 3;
    const MULTIBOOT_CONNECTION_CHECK_WAIT: u8 = 15;
    const MULTIBOOT_CLIENT_INFO: i32 = 0x72;
    const MULTIBOOT_CLIENT_DLREADY: i32 = 0x73;
    const MULTIBOOT_MASTER_INFO: i32 = 0x62;
    const MULTIBOOT_MASTER_START_PROBE: i32 = 0x61;
    const MULTIBOOT_MASTER_REQUEST_DLREADY: i32 = 0x63;
    const MULTIBOOT_MASTER_START_DL: i32 = 0x64;
    const MULTIBOOT_SERVER_TYPE_QUICK: u8 = 1;
    const MULTIBOOT_HANDSHAKE_TIMEOUT: u16 = 400;
    const MULTIBOOT_ERROR_HANDSHAKE_FAILURE: i32 = 0x71;
    const MULTIBOOT_ERROR_NO_DLREADY: i32 = 0x60;
    const MULTIBOOT_ERROR_BOOT_FAILURE: i32 = 0x70;
    const MULTIBOOT_ERROR_NO_PROBE_TARGET: i32 = 0x50;
    const SIO_MULTI_BUSY: i32 = 0x80;
    const SIO_ERROR: i32 = 0x40;
    const SIO_ID: i32 = 0x30;
    const SIO_MULTI_SD: i32 = 0x08;
    const SIO_MULTI_SI: i32 = 0x04;

    enum Tail {
        MasterInfo,
        Case1,
        Header,
    }

    unsafe {
        let b = |off: usize| mp.add(off);
        let get = |off: usize| mp.add(off).read();
        let set = |off: usize, v: u8| mp.add(off).write(v);
        let siocnt = || i32::from((0x0400_0128 as *const u16).read_volatile());
        let siomulti =
            |i: i32| i32::from((0x0400_0120 as *const u16).add(i as usize).read_volatile());
        let required = (&raw mut MultiBoot_required_data).cast::<u16>();
        let masterp = || b(MASTERP).cast::<*const u8>().read();

        if MultiBootCheckComplete(mp.cast()) != 0 {
            return 0;
        }
        if get(CHECK_WAIT) > MULTIBOOT_CONNECTION_CHECK_WAIT {
            set(CHECK_WAIT, get(CHECK_WAIT).wrapping_sub(1));
            return 0;
        }

        'output_burst: loop {
            if get(SENDFLAG) != 0 {
                set(SENDFLAG, 0);
                let i =
                    siocnt() & (SIO_MULTI_BUSY | SIO_ERROR | SIO_ID | SIO_MULTI_SD | SIO_MULTI_SI);
                if i != SIO_MULTI_SD {
                    MultiBootInit(mp.cast());
                    return i ^ SIO_MULTI_SD;
                }
            }

            if get(PROBE_COUNT) >= 0xE0 {
                let i = MultiBootHandShake(mp.cast());
                if i != 0 {
                    return i;
                }
                if get(SERVER_TYPE) == MULTIBOOT_SERVER_TYPE_QUICK
                    && get(PROBE_COUNT) > 0xE1
                    && MultiBootCheckComplete(mp.cast()) == 0
                {
                    MultiBootWaitSendDone();
                    continue 'output_burst;
                }
                if MultiBootCheckComplete(mp.cast()) == 0 {
                    let timeout = b(HANDSHAKE_TIMEOUT).cast::<u16>();
                    if timeout.read() == 0 {
                        MultiBootInit(mp.cast());
                        return MULTIBOOT_ERROR_HANDSHAKE_FAILURE;
                    }
                    timeout.write(timeout.read().wrapping_sub(1));
                }
                return 0;
            }

            let tail = match get(PROBE_COUNT) {
                0 => {
                    let mut k: i32 = 0x0E;
                    let mut i = MULTIBOOT_NCHILD;
                    while i != 0 {
                        if siomulti(i) != 0xFFFF {
                            break;
                        }
                        k >>= 1;
                        i -= 1;
                    }
                    k &= 0x0E;
                    set(RESPONSE_BIT, k as u8);

                    let mut i = MULTIBOOT_NCHILD;
                    while i != 0 {
                        let j = siomulti(i);
                        if i32::from(get(CLIENT_BIT)) & (1 << i) != 0
                            && j != ((MULTIBOOT_CLIENT_INFO << 8) | (1 << i))
                        {
                            k = 0;
                            break;
                        }
                        i -= 1;
                    }
                    set(CLIENT_BIT, get(CLIENT_BIT) & k as u8);
                    if k == 0 {
                        set(CHECK_WAIT, MULTIBOOT_CONNECTION_CHECK_WAIT);
                    }
                    if get(CHECK_WAIT) != 0 {
                        set(CHECK_WAIT, get(CHECK_WAIT).wrapping_sub(1));
                        Tail::MasterInfo
                    } else if get(RESPONSE_BIT) != get(CLIENT_BIT) {
                        MultiBootStartProbe(mp.cast());
                        Tail::Case1 // goto case_1
                    } else {
                        Tail::MasterInfo
                    }
                }
                1 => Tail::Case1,
                2 => {
                    let mut i = MULTIBOOT_NCHILD;
                    while i != 0 {
                        if i32::from(get(PROBE_TARGET_BIT)) & (1 << i) != 0 {
                            let j = siomulti(i);
                            if j != i32::from(required.add((i - 1) as usize).read()) {
                                set(PROBE_TARGET_BIT, get(PROBE_TARGET_BIT) ^ (1 << i) as u8);
                            }
                        }
                        i -= 1;
                    }
                    Tail::Header
                }
                0xD0 => {
                    let mut k: i32 = 1;
                    let mut i = MULTIBOOT_NCHILD;
                    while i != 0 {
                        let j = siomulti(i);
                        set(CLIENT_DATA + (i - 1) as usize, j as u8);
                        if i32::from(get(PROBE_TARGET_BIT)) & (1 << i) != 0 {
                            if (j >> 8) != MULTIBOOT_CLIENT_INFO
                                && (j >> 8) != MULTIBOOT_CLIENT_DLREADY
                            {
                                MultiBootInit(mp.cast());
                                return MULTIBOOT_ERROR_NO_DLREADY;
                            }
                            if j == i32::from(required.add((i - 1) as usize).read()) {
                                k = 0;
                            }
                        }
                        i -= 1;
                    }
                    if k == 0 {
                        return MultiBootSend(
                            mp.cast(),
                            ((MULTIBOOT_MASTER_REQUEST_DLREADY << 8) | i32::from(get(PALETTE_DATA)))
                                as u16,
                        );
                    }
                    set(PROBE_COUNT, 0xD1);
                    let mut k: i32 = 0x11;
                    let mut i = MULTIBOOT_NCHILD;
                    while i != 0 {
                        k += i32::from(get(CLIENT_DATA + (i - 1) as usize));
                        i -= 1;
                    }
                    set(HANDSHAKE_DATA, k as u8);
                    return MultiBootSend(
                        mp.cast(),
                        ((MULTIBOOT_MASTER_START_DL << 8) | (k & 0xFF)) as u16,
                    );
                }
                0xD1 => {
                    let mut i = MULTIBOOT_NCHILD;
                    while i != 0 {
                        let j = siomulti(i);
                        if i32::from(get(PROBE_TARGET_BIT)) & (1 << i) != 0
                            && (j >> 8) != MULTIBOOT_CLIENT_DLREADY
                        {
                            MultiBootInit(mp.cast());
                            return MULTIBOOT_ERROR_NO_DLREADY;
                        }
                        i -= 1;
                    }
                    let i = MultiBoot(mp.cast());
                    if i == 0 {
                        set(PROBE_COUNT, 0xE0);
                        b(HANDSHAKE_TIMEOUT)
                            .cast::<u16>()
                            .write(MULTIBOOT_HANDSHAKE_TIMEOUT);
                        return 0;
                    }
                    MultiBootInit(mp.cast());
                    set(CHECK_WAIT, MULTIBOOT_CONNECTION_CHECK_WAIT * 2);
                    return MULTIBOOT_ERROR_BOOT_FAILURE;
                }
                _ => {
                    let mut i = MULTIBOOT_NCHILD;
                    while i != 0 {
                        if i32::from(get(PROBE_TARGET_BIT)) & (1 << i) != 0 {
                            let j = siomulti(i);
                            if (j >> 8)
                                != (MULTIBOOT_MASTER_START_PROBE + 1
                                    - (i32::from(get(PROBE_COUNT)) >> 1))
                                || (j & 0xFF) != (1 << i)
                            {
                                set(PROBE_TARGET_BIT, get(PROBE_TARGET_BIT) ^ (1 << i) as u8);
                            }
                        }
                        i -= 1;
                    }
                    if get(PROBE_COUNT) == 0xC4 {
                        set(CLIENT_BIT, get(PROBE_TARGET_BIT) & 0x0E);
                        set(PROBE_COUNT, 0);
                        Tail::MasterInfo
                    } else {
                        Tail::Header
                    }
                }
            };

            // case_1: may still end in output_master_info
            let tail = match tail {
                Tail::Case1 => {
                    set(PROBE_TARGET_BIT, 0);
                    let mut i = MULTIBOOT_NCHILD;
                    while i != 0 {
                        let mut j = siomulti(i);
                        if (j >> 8) == MULTIBOOT_CLIENT_INFO {
                            required.add((i - 1) as usize).write(j as u16);
                            j &= 0xFF;
                            if j == (1 << i) {
                                set(PROBE_TARGET_BIT, get(PROBE_TARGET_BIT) | j as u8);
                            }
                        }
                        i -= 1;
                    }
                    if get(RESPONSE_BIT) != get(PROBE_TARGET_BIT) {
                        Tail::MasterInfo
                    } else {
                        set(PROBE_COUNT, 2);
                        return MultiBootSend(
                            mp.cast(),
                            ((MULTIBOOT_MASTER_START_PROBE << 8) | i32::from(get(PROBE_TARGET_BIT)))
                                as u16,
                        );
                    }
                }
                other => other,
            };

            match tail {
                Tail::MasterInfo | Tail::Case1 => {
                    return MultiBootSend(
                        mp.cast(),
                        ((MULTIBOOT_MASTER_INFO << 8) | i32::from(get(CLIENT_BIT))) as u16,
                    );
                }
                Tail::Header => {
                    // output_header:
                    if get(PROBE_TARGET_BIT) == 0 {
                        MultiBootInit(mp.cast());
                        return MULTIBOOT_ERROR_NO_PROBE_TARGET;
                    }
                    set(PROBE_COUNT, get(PROBE_COUNT).wrapping_add(2));
                    if get(PROBE_COUNT) == 0xC4 {
                        return MultiBootSend(
                            mp.cast(),
                            ((MULTIBOOT_MASTER_INFO << 8) | i32::from(get(CLIENT_BIT))) as u16,
                        );
                    }
                    let n = usize::from(get(PROBE_COUNT));
                    let m = masterp();
                    let i = MultiBootSend(
                        mp.cast(),
                        ((u16::from(m.add(n - 4 + 1).read())) << 8)
                            | u16::from(m.add(n - 4).read()),
                    );
                    if i != 0 {
                        return i;
                    }
                    if get(SERVER_TYPE) == MULTIBOOT_SERVER_TYPE_QUICK {
                        MultiBootWaitSendDone();
                        continue 'output_burst;
                    }
                    return 0;
                }
            }
        }
    }
}

pub(crate) unsafe extern "C" fn MultiBootSend(mp: *mut MultiBootParam, data: u16) -> i32 {
    let mut i: i32 = 0;
    i = (67109160 as usize as *mut u16).read_volatile() as i32 & 140;
    if i != SIO_MULTI_SD {
        MultiBootInit(mp);
        return i ^ SIO_MULTI_SD;
    }
    volatile_write(67109162 as usize as *mut u16, data);
    volatile_write(67109160 as usize as *mut u16, 8323);
    (*mp).sendflag = 1;
    return 0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MultiBootStartProbe(mp: *mut MultiBootParam) {
    if (*mp).probe_count != 0 {
        MultiBootInit(mp);
        return;
    }
    (*mp).check_wait = 0;
    (*mp).client_bit = 0;
    (*mp).probe_count = 1;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MultiBootStartMaster(
    mp: *mut MultiBootParam,
    srcp: *mut u8,
    mut length: i32,
    palette_color: u8,
    palette_speed: i8,
) {
    let mut i: i32 = 0;
    if (*mp).probe_count != 0 || (*mp).client_bit == 0 || (*mp).check_wait != 0 {
        MultiBootInit(mp);
        return;
    }
    (*mp).boot_srcp = srcp;
    length = length + 15 & -16;
    if length < MULTIBOOT_SEND_SIZE_MIN || length > MULTIBOOT_SEND_SIZE_MAX {
        MultiBootInit(mp);
        return;
    }
    (*mp).boot_endp = srcp.at(length);
    match palette_speed {
        -4 | -3 | -2 | -1 => {
            i = (palette_color as i32) << 3 | 3 - palette_speed as i32;
        }
        0 => {
            i = 0x38 | palette_color as i32;
        }
        1 | 2 | 3 | 4 => {
            i = (palette_color as i32) << 3 | palette_speed as i32 - 1;
        }
        _ => {}
    }
    (*mp).palette_data = (i as u8 & 0x3f) << 1 | 0x81;
    (*mp).probe_count = 0xd0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MultiBootCheckComplete(mp: *mut MultiBootParam) -> i32 {
    if (*mp).probe_count == 0xe9 {
        return 1;
    }
    return 0;
}
// hand-written: tools/rustport/overrides/multiboot/MultiBootHandShake.rs
// c2rs-uses: MultiBootSend MultiBootInit
/// `MultiBootHandShake`. The C's `default:` case jumps back to `case 0xe0:`
/// (`goto case_0xe0`) and `case 0xe7/0xe8` jumps into the tail of `default`
/// (`goto output_common`); both become explicit steps here.
/// struct MultiBootParam: system_work @0 (send_data = [0], must_data = [1]),
/// probe_count @0x18, client_bit @0x1E, masterp @0x28.
unsafe extern "C" fn MultiBootHandShake(mp: *mut u8) -> i32 {
    const MULTIBOOT_NCHILD: i32 = 3;
    const MULTIBOOT_ERROR_HANDSHAKE_FAILURE: i32 = 0x71;
    enum Step {
        Start0xE0,
        OutputCommon,
    }
    unsafe {
        let send_data = mp.cast::<u32>();
        let must_data = mp.cast::<u32>().add(1);
        let probe_count = mp.add(0x18);
        let client_bit = mp.add(0x1E).read();
        let masterp = mp.add(0x28).cast::<*const u8>().read();
        let siomulti =
            |i: i32| i32::from((0x0400_0120 as *const u16).add(i as usize).read_volatile());

        let step = match probe_count.read() {
            0xE0 => Step::Start0xE0,
            0xE7 | 0xE8 => {
                let mut i = MULTIBOOT_NCHILD;
                while i != 0 {
                    let j = siomulti(i);
                    if i32::from(client_bit) & (1 << i) != 0 && j as u32 != must_data.read() {
                        MultiBootInit(mp.cast());
                        return MULTIBOOT_ERROR_HANDSHAKE_FAILURE;
                    }
                    i -= 1;
                }
                probe_count.write(probe_count.read().wrapping_add(1));
                if probe_count.read() == 0xE9 {
                    return 0;
                }
                send_data.write(
                    u32::from(masterp.add(0xAE).read())
                        | (u32::from(masterp.add(0xAF).read()) << 8),
                );
                must_data.write(send_data.read());
                Step::OutputCommon
            }
            _ => {
                let mut restart = false;
                let mut i = MULTIBOOT_NCHILD;
                while i != 0 {
                    let j = siomulti(i);
                    if i32::from(client_bit) & (1 << i) != 0 && j as u32 != must_data.read() {
                        restart = true; // goto case_0xe0
                        break;
                    }
                    i -= 1;
                }
                if restart {
                    Step::Start0xE0
                } else {
                    probe_count.write(probe_count.read().wrapping_add(1));
                    must_data.write(send_data.read() & 0xFFFF);
                    if send_data.read() == 0 {
                        must_data.write(
                            u32::from(masterp.add(0xAC).read())
                                | (u32::from(masterp.add(0xAD).read()) << 8),
                        );
                        send_data.write(must_data.read() << 5);
                    }
                    send_data.write(send_data.read() >> 5);
                    Step::OutputCommon
                }
            }
        };
        match step {
            Step::Start0xE0 => {
                probe_count.write(0xE1);
                must_data.write(0);
                send_data.write(0x10_0000);
                MultiBootSend(mp.cast(), 0)
            }
            Step::OutputCommon => MultiBootSend(mp.cast(), send_data.read() as u16),
        }
    }
}

// hand-written: tools/rustport/overrides/multiboot/MultiBootWaitCycles.rs
/// `MultiBootWaitCycles`: a busy wait of `cycles` CPU cycles. The loop's
/// cost per iteration depends on the memory it runs from (read from pc:
/// ROM 0x08.., EWRAM 0x02.., IWRAM), so like the original this stays a
/// hand-counted Thumb loop.
#[cfg(target_arch = "arm")]
#[unsafe(naked)]
unsafe extern "C" fn MultiBootWaitCycles(cycles: u32) {
    core::arch::naked_asm!(
        "mov  r2, pc",
        "lsrs r2, r2, #24",
        "movs r1, #12",
        "cmp  r2, #2",
        "beq  2f",
        "movs r1, #13",
        "cmp  r2, #8",
        "beq  2f",
        "movs r1, #4",
        "2:",
        "subs r0, r0, r1",
        "bgt  2b",
        "bx   lr",
    )
}

/// Host builds (unit tests) have no GBA timing to wait for.
#[cfg(not(target_arch = "arm"))]
unsafe extern "C" fn MultiBootWaitCycles(_cycles: u32) {}

pub(crate) unsafe extern "C" fn MultiBootWaitSendDone() {
    let mut i: i32 = 0;
    i = 0;
    while i < 31069 {
        if (67109160 as usize as *mut u16).read_volatile() as i32 & SIO_START as i32 == 0 {
            break;
        }
        i += 1;
    }
    MultiBootWaitCycles(600);
}
