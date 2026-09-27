//! Translated from `src/librfu_stwi.c` by tools/rustport/c2rs.py, then reviewed.
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

#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gSTWIStatus: *mut u8 = core::ptr::null_mut();

unsafe extern "C" {
    static mut IntrEnable: u8;
    fn CpuSet(a0: *mut u8, a1: *mut u8, a2: u32);
    fn IntrSIO32();
}

// hand-written: tools/rustport/overrides/librfu_stwi/STWI_init_all.rs
// c2rs-uses: IntrSIO32 gSTWIStatus STWI_init_Callback_M STWI_init_Callback_S IntrEnable
/// `STWI_init_all`, with the interrupt handler kept in ROM (see below).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn STWI_init_all(
    interruptStruct: *mut u8,
    interrupt: *mut Option<unsafe extern "C" fn()>,
    copyInterruptToRam: u8,
) {
    unsafe {
        let mut interruptStruct = interruptStruct;
        let mut interrupt = interrupt;
        let mut copyInterruptToRam = copyInterruptToRam;
        if ((copyInterruptToRam) as i32) == 1i32 {
            // The C DMAs 0x960 bytes from IntrSIO32 into block1 and runs the
            // interrupt from there: the size of the original librfu_intr
            // binary, whose functions call each other PC-relatively. The Rust
            // functions are not laid out as one block, so a copy would not
            // run; the handler runs from ROM instead (as in the C's other
            // branch) and block2 still holds the STWIStatus, so the RAM
            // layout is unchanged.
            (interrupt).write(Some(IntrSIO32 as unsafe extern "C" fn()));
            ((&raw mut gSTWIStatus).cast::<u8>().cast::<*mut u8>())
                .write((interruptStruct).wrapping_add(2632));
        } else {
            (interrupt).write(Some(IntrSIO32 as unsafe extern "C" fn()));
            ((&raw mut gSTWIStatus).cast::<u8>().cast::<*mut u8>())
                .write(((interruptStruct).wrapping_add(232)).cast::<u8>());
        }
        ((((&raw mut gSTWIStatus).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(40)
            .cast::<*mut u8>())
        .write((interruptStruct));
        ((((&raw mut gSTWIStatus).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(36)
            .cast::<*mut u8>())
        .write((interruptStruct).wrapping_add(116));
        crate::c::volatile_write(
            (((&raw mut gSTWIStatus).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(20),
            1u8,
        );
        crate::c::volatile_write(
            (((&raw mut gSTWIStatus).cast::<u8>().cast::<*mut u8>()).read()).cast::<i32>(),
            0i32,
        );
        ((((&raw mut gSTWIStatus).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(4))
            .write(0u8);
        ((((&raw mut gSTWIStatus).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(5))
            .write(0u8);
        ((((&raw mut gSTWIStatus).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(7))
            .write(0u8);
        ((((&raw mut gSTWIStatus).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(8))
            .write(0u8);
        ((((&raw mut gSTWIStatus).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(9))
            .write(0u8);
        ((((&raw mut gSTWIStatus).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(12)
            .cast::<i32>())
        .write(0i32);
        crate::c::volatile_write(
            (((&raw mut gSTWIStatus).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(16),
            0u8,
        );
        crate::c::volatile_write(
            (((&raw mut gSTWIStatus).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(18)
                .cast::<u16>(),
            0u16,
        );
        ((((&raw mut gSTWIStatus).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(21))
            .write(0u8);
        crate::c::volatile_write(
            (((&raw mut gSTWIStatus).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(44),
            0u8,
        );
        crate::c::volatile_write(((67109172i32) as usize as *mut u16), 256u16);
        crate::c::volatile_write(((67109160i32) as usize as *mut u16), 20483u16);
        STWI_init_Callback_M();
        STWI_init_Callback_S();
        {
            let mut imeTemp: u16 = 0u16;
            imeTemp = ((67109384i32) as usize as *mut u16).read_volatile();
            crate::c::volatile_write(((67109384i32) as usize as *mut u16), 0u16);
            let __p1 = ((67109376i32) as usize as *mut u16);
            crate::c::volatile_write(__p1, (((((__p1).read_volatile()) as i32) | 128i32) as u16));
            crate::c::volatile_write(((67109384i32) as usize as *mut u16), imeTemp);
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn STWI_init_timer(
    interrupt: *mut Option<unsafe extern "C" fn()>,
    timerSelect: i32,
) {
    unsafe {
        let mut interrupt = interrupt;
        let mut timerSelect = timerSelect;
        (interrupt).write(Some(STWI_intr_timer as unsafe extern "C" fn()));
        ((((&raw mut gSTWIStatus).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(10))
            .write(((timerSelect) as u8));
        {
            let mut imeTemp: u16 = 0u16;
            imeTemp = ((67109384i32) as usize as *mut u16).read_volatile();
            crate::c::volatile_write(((67109384i32) as usize as *mut u16), 0u16);
            let __p1 = ((67109376i32) as usize as *mut u16);
            crate::c::volatile_write(
                __p1,
                (((((__p1).read_volatile()) as i32)
                    | crate::c::shl_i32(
                        8i32,
                        ((((((&raw mut gSTWIStatus).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(10))
                        .read()) as u32),
                    )) as u16),
            );
            crate::c::volatile_write(((67109384i32) as usize as *mut u16), imeTemp);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AgbRFU_SoftReset() {
    unsafe {
        let mut timerL: *mut u16 = core::ptr::null_mut();
        let mut timerH: *mut u16 = core::ptr::null_mut();
        crate::c::volatile_write(((67109172i32) as usize as *mut u16), 32768u16);
        crate::c::volatile_write(((67109172i32) as usize as *mut u16), 32928u16);
        timerL = (((67109120i32).wrapping_add(
            ((((((&raw mut gSTWIStatus).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(10))
                .read()) as i32)
                .wrapping_mul(4i32),
        )) as usize as *mut u16);
        timerH = (((67109122i32).wrapping_add(
            ((((((&raw mut gSTWIStatus).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(10))
                .read()) as i32)
                .wrapping_mul(4i32),
        )) as usize as *mut u16);
        crate::c::volatile_write(timerH, 0u16);
        crate::c::volatile_write(timerL, 0u16);
        crate::c::volatile_write(timerH, 131u16);
        'l1: loop {
            if !((((timerL).read_volatile()) as i32) <= 17i32) {
                break 'l1;
            }
            crate::c::volatile_write(((67109172i32) as usize as *mut u16), 32930u16);
        }
        crate::c::volatile_write(timerH, 3u16);
        crate::c::volatile_write(((67109172i32) as usize as *mut u16), 32928u16);
        crate::c::volatile_write(((67109160i32) as usize as *mut u16), 20483u16);
        crate::c::volatile_write(
            (((&raw mut gSTWIStatus).cast::<u8>().cast::<*mut u8>()).read()).cast::<i32>(),
            0i32,
        );
        ((((&raw mut gSTWIStatus).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(4))
            .write(0u8);
        ((((&raw mut gSTWIStatus).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(5))
            .write(0u8);
        ((((&raw mut gSTWIStatus).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(6))
            .write(0u8);
        ((((&raw mut gSTWIStatus).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(7))
            .write(0u8);
        ((((&raw mut gSTWIStatus).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(8))
            .write(0u8);
        ((((&raw mut gSTWIStatus).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(9))
            .write(0u8);
        ((((&raw mut gSTWIStatus).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(12)
            .cast::<i32>())
        .write(0i32);
        crate::c::volatile_write(
            (((&raw mut gSTWIStatus).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(16),
            0u8,
        );
        crate::c::volatile_write(
            (((&raw mut gSTWIStatus).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(18)
                .cast::<u16>(),
            0u16,
        );
        crate::c::volatile_write(
            (((&raw mut gSTWIStatus).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(20),
            1u8,
        );
        ((((&raw mut gSTWIStatus).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(21))
            .write(0u8);
        crate::c::volatile_write(
            (((&raw mut gSTWIStatus).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(44),
            0u8,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn STWI_set_MS_mode(mode: u8) {
    unsafe {
        let mut mode = mode;
        crate::c::volatile_write(
            (((&raw mut gSTWIStatus).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(20),
            mode,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn STWI_read_status(index: u8) -> u16 {
    unsafe {
        let mut index = index;
        'l1: {
            let __sw1 = ((index) as i32);
            let __matched = __sw1 == 0i32 || __sw1 == 1i32 || __sw1 == 2i32 || __sw1 == 3i32;
            if __sw1 == 0i32 {
                return ((((&raw mut gSTWIStatus).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(18)
                    .cast::<u16>())
                .read_volatile();
            }
            if __sw1 == 1i32 {
                return ((((((&raw mut gSTWIStatus).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(20))
                .read_volatile()) as u16);
            }
            if __sw1 == 2i32 {
                return ((((((&raw mut gSTWIStatus).cast::<u8>().cast::<*mut u8>()).read())
                    .cast::<i32>())
                .read_volatile()) as u16);
            }
            if __sw1 == 3i32 {
                return ((((((&raw mut gSTWIStatus).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(6))
                .read()) as u16);
            }
            if !__matched {
                return 65535u16;
            }
        }
        #[allow(unreachable_code)]
        {
            return 0u16;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn STWI_init_Callback_M() {
    unsafe {
        STWI_set_Callback_M(core::ptr::null_mut());
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn STWI_init_Callback_S() {
    unsafe {
        STWI_set_Callback_S(None);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn STWI_set_Callback_M(callbackM: *mut u8) {
    unsafe {
        let mut callbackM = callbackM;
        ((((&raw mut gSTWIStatus).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(24)
            .cast::<Option<unsafe extern "C" fn()>>())
        .write(core::mem::transmute::<_, Option<unsafe extern "C" fn()>>(
            callbackM,
        ));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn STWI_set_Callback_S(callbackS: Option<unsafe extern "C" fn(u16)>) {
    unsafe {
        let mut callbackS = callbackS;
        ((((&raw mut gSTWIStatus).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(u16)>>())
        .write(callbackS);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn STWI_set_Callback_ID(func: Option<unsafe extern "C" fn()>) {
    unsafe {
        let mut func = func;
        ((((&raw mut gSTWIStatus).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(32)
            .cast::<Option<unsafe extern "C" fn()>>())
        .write(func);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn STWI_poll_CommandEnd() -> u16 {
    unsafe {
        'l1: loop {
            if !(((((((&raw mut gSTWIStatus).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(44))
            .read_volatile()) as i32)
                == 1i32)
            {
                break 'l1;
            }
        }
        return ((((&raw mut gSTWIStatus).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(18)
            .cast::<u16>())
        .read_volatile();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn STWI_send_ResetREQ() {
    unsafe {
        if !((STWI_init(16u8)) != 0) {
            ((((&raw mut gSTWIStatus).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(4))
                .write(0u8);
            STWI_start_Command();
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn STWI_send_LinkStatusREQ() {
    unsafe {
        if !((STWI_init(17u8)) != 0) {
            ((((&raw mut gSTWIStatus).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(4))
                .write(0u8);
            STWI_start_Command();
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn STWI_send_VersionStatusREQ() {
    unsafe {
        if !((STWI_init(18u8)) != 0) {
            ((((&raw mut gSTWIStatus).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(4))
                .write(0u8);
            STWI_start_Command();
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn STWI_send_SystemStatusREQ() {
    unsafe {
        if !((STWI_init(19u8)) != 0) {
            ((((&raw mut gSTWIStatus).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(4))
                .write(0u8);
            STWI_start_Command();
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn STWI_send_SlotStatusREQ() {
    unsafe {
        if !((STWI_init(20u8)) != 0) {
            ((((&raw mut gSTWIStatus).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(4))
                .write(0u8);
            STWI_start_Command();
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn STWI_send_ConfigStatusREQ() {
    unsafe {
        if !((STWI_init(21u8)) != 0) {
            ((((&raw mut gSTWIStatus).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(4))
                .write(0u8);
            STWI_start_Command();
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn STWI_send_GameConfigREQ(serial_gname: *mut u8, uname: *mut u8) {
    unsafe {
        let mut serial_gname = serial_gname;
        let mut uname = uname;
        let mut packetBytes: *mut u8 = core::ptr::null_mut();
        let mut i: i32 = 0i32;
        if !((STWI_init(22u8)) != 0) {
            ((((&raw mut gSTWIStatus).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(4))
                .write(6u8);
            packetBytes = (((((&raw mut gSTWIStatus).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(36)
                .cast::<*mut u8>())
            .read())
            .cast::<u8>();
            packetBytes = (packetBytes).wrapping_offset(4);
            ((packetBytes).cast::<u16>()).write(((serial_gname).cast::<u16>()).read());
            packetBytes = (packetBytes).wrapping_offset(2);
            serial_gname = (serial_gname).wrapping_offset(2);
            {
                i = 0i32;
                'l1: loop {
                    if !(i < 14i32) {
                        break 'l1;
                    }
                    'l2: {
                        (packetBytes).write((serial_gname).read());
                        packetBytes = (packetBytes).wrapping_offset(1);
                        serial_gname = (serial_gname).wrapping_offset(1);
                    }
                    i = (i).wrapping_add(1);
                }
            }
            {
                i = 0i32;
                'l3: loop {
                    if !(i < 8i32) {
                        break 'l3;
                    }
                    'l4: {
                        (packetBytes).write((uname).read());
                        packetBytes = (packetBytes).wrapping_offset(1);
                        uname = (uname).wrapping_offset(1);
                    }
                    i = (i).wrapping_add(1);
                }
            }
            STWI_start_Command();
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn STWI_send_SystemConfigREQ(availSlotFlag: u16, maxMFrame: u8, mcTimer: u8) {
    unsafe {
        let mut availSlotFlag = availSlotFlag;
        let mut maxMFrame = maxMFrame;
        let mut mcTimer = mcTimer;
        if !((STWI_init(23u8)) != 0) {
            let mut packetBytes: *mut u8 = core::ptr::null_mut();
            ((((&raw mut gSTWIStatus).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(4))
                .write(1u8);
            packetBytes = (((((&raw mut gSTWIStatus).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(36)
                .cast::<*mut u8>())
            .read())
            .cast::<u8>();
            packetBytes = (packetBytes).wrapping_offset(4);
            ({
                let __t1 = packetBytes;
                packetBytes = (packetBytes).wrapping_offset(1);
                __t1
            })
            .write(mcTimer);
            ({
                let __t2 = packetBytes;
                packetBytes = (packetBytes).wrapping_offset(1);
                __t2
            })
            .write(maxMFrame);
            ((packetBytes).cast::<u16>()).write(availSlotFlag);
            STWI_start_Command();
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn STWI_send_SC_StartREQ() {
    unsafe {
        if !((STWI_init(25u8)) != 0) {
            ((((&raw mut gSTWIStatus).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(4))
                .write(0u8);
            STWI_start_Command();
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn STWI_send_SC_PollingREQ() {
    unsafe {
        if !((STWI_init(26u8)) != 0) {
            ((((&raw mut gSTWIStatus).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(4))
                .write(0u8);
            STWI_start_Command();
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn STWI_send_SC_EndREQ() {
    unsafe {
        if !((STWI_init(27u8)) != 0) {
            ((((&raw mut gSTWIStatus).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(4))
                .write(0u8);
            STWI_start_Command();
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn STWI_send_SP_StartREQ() {
    unsafe {
        if !((STWI_init(28u8)) != 0) {
            ((((&raw mut gSTWIStatus).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(4))
                .write(0u8);
            STWI_start_Command();
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn STWI_send_SP_PollingREQ() {
    unsafe {
        if !((STWI_init(29u8)) != 0) {
            ((((&raw mut gSTWIStatus).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(4))
                .write(0u8);
            STWI_start_Command();
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn STWI_send_SP_EndREQ() {
    unsafe {
        if !((STWI_init(30u8)) != 0) {
            ((((&raw mut gSTWIStatus).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(4))
                .write(0u8);
            STWI_start_Command();
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn STWI_send_CP_StartREQ(unk1: u16) {
    unsafe {
        let mut unk1 = unk1;
        if !((STWI_init(31u8)) != 0) {
            ((((&raw mut gSTWIStatus).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(4))
                .write(1u8);
            (((((((&raw mut gSTWIStatus).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(36)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(4))
            .cast::<u32>())
            .write(((unk1) as u32));
            STWI_start_Command();
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn STWI_send_CP_PollingREQ() {
    unsafe {
        if !((STWI_init(32u8)) != 0) {
            ((((&raw mut gSTWIStatus).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(4))
                .write(0u8);
            STWI_start_Command();
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn STWI_send_CP_EndREQ() {
    unsafe {
        if !((STWI_init(33u8)) != 0) {
            ((((&raw mut gSTWIStatus).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(4))
                .write(0u8);
            STWI_start_Command();
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn STWI_send_DataTxREQ(r#in: *mut u8, size: u8) {
    unsafe {
        let mut r#in = r#in;
        let mut size = size;
        if !((STWI_init(36u8)) != 0) {
            let mut reqLength: u8 = ((crate::c::div_u32(((size) as u32), 4u32)) as u8);
            if (((size) as u32) & 3u32) != 0 {
                reqLength = ((((reqLength) as i32).wrapping_add(1i32)) as u8);
            }
            ((((&raw mut gSTWIStatus).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(4))
                .write(reqLength);
            'l1: loop {
                'l2: {
                    'l3: loop {
                        'l4: {
                            CpuSet(
                                r#in,
                                (((((((&raw mut gSTWIStatus).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(36)
                                .cast::<*mut u8>())
                                .read())
                                .wrapping_add(4))
                                .cast::<u32>())
                                .cast::<u8>(),
                                (67108864u32
                                    | (crate::c::div_u32(
                                        ((((((&raw mut gSTWIStatus)
                                            .cast::<u8>()
                                            .cast::<*mut u8>())
                                        .read())
                                        .wrapping_add(4))
                                        .read()) as u32)
                                            .wrapping_mul(4u32),
                                        ((crate::c::div_i32(32i32, 8i32)) as u32),
                                    ) & 2097151u32)),
                            );
                        }
                        if !((0i32) != 0) {
                            break 'l3;
                        }
                    }
                }
                if !((0i32) != 0) {
                    break 'l1;
                }
            }
            STWI_start_Command();
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn STWI_send_DataTxAndChangeREQ(r#in: *mut u8, size: u8) {
    unsafe {
        let mut r#in = r#in;
        let mut size = size;
        if !((STWI_init(37u8)) != 0) {
            let mut reqLength: u8 = ((crate::c::div_u32(((size) as u32), 4u32)) as u8);
            if (((size) as u32) & 3u32) != 0 {
                reqLength = ((((reqLength) as i32).wrapping_add(1i32)) as u8);
            }
            ((((&raw mut gSTWIStatus).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(4))
                .write(reqLength);
            'l1: loop {
                'l2: {
                    'l3: loop {
                        'l4: {
                            CpuSet(
                                r#in,
                                (((((((&raw mut gSTWIStatus).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(36)
                                .cast::<*mut u8>())
                                .read())
                                .wrapping_add(4))
                                .cast::<u32>())
                                .cast::<u8>(),
                                (67108864u32
                                    | (crate::c::div_u32(
                                        ((((((&raw mut gSTWIStatus)
                                            .cast::<u8>()
                                            .cast::<*mut u8>())
                                        .read())
                                        .wrapping_add(4))
                                        .read()) as u32)
                                            .wrapping_mul(4u32),
                                        ((crate::c::div_i32(32i32, 8i32)) as u32),
                                    ) & 2097151u32)),
                            );
                        }
                        if !((0i32) != 0) {
                            break 'l3;
                        }
                    }
                }
                if !((0i32) != 0) {
                    break 'l1;
                }
            }
            STWI_start_Command();
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn STWI_send_DataRxREQ() {
    unsafe {
        if !((STWI_init(38u8)) != 0) {
            ((((&raw mut gSTWIStatus).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(4))
                .write(0u8);
            STWI_start_Command();
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn STWI_send_MS_ChangeREQ() {
    unsafe {
        if !((STWI_init(39u8)) != 0) {
            ((((&raw mut gSTWIStatus).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(4))
                .write(0u8);
            STWI_start_Command();
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn STWI_send_DataReadyAndChangeREQ(unk: u8) {
    unsafe {
        let mut unk = unk;
        if !((STWI_init(40u8)) != 0) {
            if !((unk) != 0) {
                ((((&raw mut gSTWIStatus).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(4))
                    .write(0u8);
            } else {
                let mut packetBytes: *mut u8 = core::ptr::null_mut();
                ((((&raw mut gSTWIStatus).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(4))
                    .write(1u8);
                packetBytes = (((((&raw mut gSTWIStatus).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(36)
                    .cast::<*mut u8>())
                .read())
                .cast::<u8>();
                packetBytes = (packetBytes).wrapping_offset(4);
                ({
                    let __t1 = packetBytes;
                    packetBytes = (packetBytes).wrapping_offset(1);
                    __t1
                })
                .write(unk);
                ({
                    let __t2 = packetBytes;
                    packetBytes = (packetBytes).wrapping_offset(1);
                    __t2
                })
                .write(0u8);
                ({
                    let __t3 = packetBytes;
                    packetBytes = (packetBytes).wrapping_offset(1);
                    __t3
                })
                .write(0u8);
                (packetBytes).write(0u8);
            }
            STWI_start_Command();
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn STWI_send_DisconnectedAndChangeREQ(unk0: u8, unk1: u8) {
    unsafe {
        let mut unk0 = unk0;
        let mut unk1 = unk1;
        if !((STWI_init(41u8)) != 0) {
            let mut packetBytes: *mut u8 = core::ptr::null_mut();
            ((((&raw mut gSTWIStatus).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(4))
                .write(1u8);
            packetBytes = (((((&raw mut gSTWIStatus).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(36)
                .cast::<*mut u8>())
            .read())
            .cast::<u8>();
            packetBytes = (packetBytes).wrapping_offset(4);
            ({
                let __t1 = packetBytes;
                packetBytes = (packetBytes).wrapping_offset(1);
                __t1
            })
            .write(unk0);
            ({
                let __t2 = packetBytes;
                packetBytes = (packetBytes).wrapping_offset(1);
                __t2
            })
            .write(unk1);
            ({
                let __t3 = packetBytes;
                packetBytes = (packetBytes).wrapping_offset(1);
                __t3
            })
            .write(0u8);
            (packetBytes).write(0u8);
            STWI_start_Command();
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn STWI_send_ResumeRetransmitAndChangeREQ() {
    unsafe {
        if !((STWI_init(55u8)) != 0) {
            ((((&raw mut gSTWIStatus).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(4))
                .write(0u8);
            STWI_start_Command();
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn STWI_send_DisconnectREQ(unk: u8) {
    unsafe {
        let mut unk = unk;
        if !((STWI_init(48u8)) != 0) {
            ((((&raw mut gSTWIStatus).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(4))
                .write(1u8);
            (((((((&raw mut gSTWIStatus).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(36)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(4))
            .cast::<u32>())
            .write(((unk) as u32));
            STWI_start_Command();
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn STWI_send_TestModeREQ(unk0: u8, unk1: u8) {
    unsafe {
        let mut unk0 = unk0;
        let mut unk1 = unk1;
        if !((STWI_init(49u8)) != 0) {
            ((((&raw mut gSTWIStatus).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(4))
                .write(1u8);
            (((((((&raw mut gSTWIStatus).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(36)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(4))
            .cast::<u32>())
            .write(((((unk0) as i32) | (((unk1) as i32) << 8)) as u32));
            STWI_start_Command();
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn STWI_send_CPR_StartREQ(unk0: u16, unk1: u16, unk2: u8) {
    unsafe {
        let mut unk0 = unk0;
        let mut unk1 = unk1;
        let mut unk2 = unk2;
        let mut packetData: *mut u32 = core::ptr::null_mut();
        let mut arg1: u32 = 0u32;
        if !((STWI_init(50u8)) != 0) {
            ((((&raw mut gSTWIStatus).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(4))
                .write(2u8);
            arg1 = ((((unk1) as i32) | (((unk0) as i32) << 16)) as u32);
            packetData = ((((((&raw mut gSTWIStatus).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(36)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(4))
            .cast::<u32>();
            (packetData).write(arg1);
            ((packetData).wrapping_offset(1)).write(((unk2) as u32));
            STWI_start_Command();
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn STWI_send_CPR_PollingREQ() {
    unsafe {
        if !((STWI_init(51u8)) != 0) {
            ((((&raw mut gSTWIStatus).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(4))
                .write(0u8);
            STWI_start_Command();
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn STWI_send_CPR_EndREQ() {
    unsafe {
        if !((STWI_init(52u8)) != 0) {
            ((((&raw mut gSTWIStatus).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(4))
                .write(0u8);
            STWI_start_Command();
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn STWI_send_StopModeREQ() {
    unsafe {
        if !((STWI_init(61u8)) != 0) {
            ((((&raw mut gSTWIStatus).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(4))
                .write(0u8);
            STWI_start_Command();
        }
    }
}
pub(crate) unsafe extern "C" fn STWI_intr_timer() {
    unsafe {
        'l1: {
            let __sw1 = ((((&raw mut gSTWIStatus).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(12)
                .cast::<i32>())
            .read();
            if __sw1 == 2i32 {
                crate::c::volatile_write(
                    (((&raw mut gSTWIStatus).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(16),
                    1u8,
                );
                STWI_set_timer(50u8);
                break 'l1;
            }
            if __sw1 == 1i32 || __sw1 == 4i32 {
                STWI_stop_timer();
                STWI_restart_Command();
                break 'l1;
            }
            if __sw1 == 3i32 {
                crate::c::volatile_write(
                    (((&raw mut gSTWIStatus).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(16),
                    1u8,
                );
                STWI_stop_timer();
                STWI_reset_ClockCounter();
                if core::mem::transmute::<_, usize>(
                    ((((&raw mut gSTWIStatus).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(24)
                        .cast::<Option<unsafe extern "C" fn()>>())
                    .read(),
                ) != 0usize
                {
                    core::mem::transmute::<_, unsafe extern "C" fn(i32, i32)>(
                        (((((&raw mut gSTWIStatus).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(24)
                            .cast::<Option<unsafe extern "C" fn()>>())
                        .read())
                        .unwrap_unchecked(),
                    )(255i32, 0i32);
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn STWI_set_timer(count: u8) {
    unsafe {
        let mut count = count;
        let mut timerL: *mut u16 = (((67109120i32).wrapping_add(
            ((((((&raw mut gSTWIStatus).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(10))
                .read()) as i32)
                .wrapping_mul(4i32),
        )) as usize as *mut u16);
        let mut timerH: *mut u16 = (((67109122i32).wrapping_add(
            ((((((&raw mut gSTWIStatus).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(10))
                .read()) as i32)
                .wrapping_mul(4i32),
        )) as usize as *mut u16);
        crate::c::volatile_write(((67109384i32) as usize as *mut u16), 0u16);
        'l1: {
            let __sw1 = ((count) as i32);
            if __sw1 == 50i32 {
                crate::c::volatile_write(timerL, 64715u16);
                ((((&raw mut gSTWIStatus).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(12)
                    .cast::<i32>())
                .write(1i32);
                break 'l1;
            }
            if __sw1 == 80i32 {
                crate::c::volatile_write(timerL, 64224u16);
                ((((&raw mut gSTWIStatus).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(12)
                    .cast::<i32>())
                .write(2i32);
                break 'l1;
            }
            if __sw1 == 100i32 {
                crate::c::volatile_write(timerL, 63894u16);
                ((((&raw mut gSTWIStatus).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(12)
                    .cast::<i32>())
                .write(3i32);
                break 'l1;
            }
            if __sw1 == 130i32 {
                crate::c::volatile_write(timerL, 63405u16);
                ((((&raw mut gSTWIStatus).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(12)
                    .cast::<i32>())
                .write(4i32);
                break 'l1;
            }
        }
        crate::c::volatile_write(timerH, 195u16);
        crate::c::volatile_write(
            ((67109378i32) as usize as *mut u16),
            ((crate::c::shl_i32(
                8i32,
                ((((((&raw mut gSTWIStatus).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(10))
                .read()) as u32),
            )) as u16),
        );
        crate::c::volatile_write(((67109384i32) as usize as *mut u16), 1u16);
    }
}
pub(crate) unsafe extern "C" fn STWI_stop_timer() {
    unsafe {
        ((((&raw mut gSTWIStatus).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(12)
            .cast::<i32>())
        .write(0i32);
        crate::c::volatile_write(
            (((67109120i32).wrapping_add(
                ((((((&raw mut gSTWIStatus).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(10))
                .read()) as i32)
                    .wrapping_mul(4i32),
            )) as usize as *mut u16),
            0u16,
        );
        crate::c::volatile_write(
            (((67109122i32).wrapping_add(
                ((((((&raw mut gSTWIStatus).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(10))
                .read()) as i32)
                    .wrapping_mul(4i32),
            )) as usize as *mut u16),
            0u16,
        );
    }
}
pub(crate) unsafe extern "C" fn STWI_init(request: u8) -> u16 {
    unsafe {
        let mut request = request;
        if !((((67109384i32) as usize as *mut u16).read_volatile()) != 0) {
            crate::c::volatile_write(
                (((&raw mut gSTWIStatus).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(18)
                    .cast::<u16>(),
                6u16,
            );
            if core::mem::transmute::<_, usize>(
                ((((&raw mut gSTWIStatus).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(24)
                    .cast::<Option<unsafe extern "C" fn()>>())
                .read(),
            ) != 0usize
            {
                core::mem::transmute::<_, unsafe extern "C" fn(i32, i32)>(
                    (((((&raw mut gSTWIStatus).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(24)
                        .cast::<Option<unsafe extern "C" fn()>>())
                    .read())
                    .unwrap_unchecked(),
                )(
                    ((request) as i32),
                    ((((((&raw mut gSTWIStatus).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(18)
                        .cast::<u16>())
                    .read_volatile()) as i32),
                );
            }
            return 1u16;
        } else {
            if ((((((&raw mut gSTWIStatus).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(44))
            .read_volatile()) as i32)
                == 1i32
            {
                crate::c::volatile_write(
                    (((&raw mut gSTWIStatus).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(18)
                        .cast::<u16>(),
                    2u16,
                );
                crate::c::volatile_write(
                    (((&raw mut gSTWIStatus).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(44),
                    0u8,
                );
                if core::mem::transmute::<_, usize>(
                    ((((&raw mut gSTWIStatus).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(24)
                        .cast::<Option<unsafe extern "C" fn()>>())
                    .read(),
                ) != 0usize
                {
                    core::mem::transmute::<_, unsafe extern "C" fn(i32, i32)>(
                        (((((&raw mut gSTWIStatus).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(24)
                            .cast::<Option<unsafe extern "C" fn()>>())
                        .read())
                        .unwrap_unchecked(),
                    )(
                        ((request) as i32),
                        ((((((&raw mut gSTWIStatus).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(18)
                            .cast::<u16>())
                        .read_volatile()) as i32),
                    );
                }
                return 1u16;
            } else {
                if ((((((&raw mut gSTWIStatus).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(20))
                .read_volatile()) as i32)
                    == 0i32
                {
                    crate::c::volatile_write(
                        (((&raw mut gSTWIStatus).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(18)
                            .cast::<u16>(),
                        4u16,
                    );
                    if core::mem::transmute::<_, usize>(
                        ((((&raw mut gSTWIStatus).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(24)
                            .cast::<Option<unsafe extern "C" fn()>>())
                        .read(),
                    ) != 0usize
                    {
                        core::mem::transmute::<_, unsafe extern "C" fn(i32, i32, *mut u8)>(
                            (((((&raw mut gSTWIStatus).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(24)
                                .cast::<Option<unsafe extern "C" fn()>>())
                            .read())
                            .unwrap_unchecked(),
                        )(
                            ((request) as i32),
                            ((((((&raw mut gSTWIStatus).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(18)
                                .cast::<u16>())
                            .read_volatile()) as i32),
                            ((&raw mut gSTWIStatus).cast::<u8>().cast::<*mut u8>()).read(),
                        );
                    }
                    return 1u16;
                } else {
                    crate::c::volatile_write(
                        (((&raw mut gSTWIStatus).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(44),
                        1u8,
                    );
                    ((((&raw mut gSTWIStatus).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(6))
                    .write(request);
                    crate::c::volatile_write(
                        (((&raw mut gSTWIStatus).cast::<u8>().cast::<*mut u8>()).read())
                            .cast::<i32>(),
                        0i32,
                    );
                    ((((&raw mut gSTWIStatus).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(4))
                    .write(0u8);
                    ((((&raw mut gSTWIStatus).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(5))
                    .write(0u8);
                    ((((&raw mut gSTWIStatus).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(7))
                    .write(0u8);
                    ((((&raw mut gSTWIStatus).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(8))
                    .write(0u8);
                    ((((&raw mut gSTWIStatus).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(9))
                    .write(0u8);
                    ((((&raw mut gSTWIStatus).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(12)
                        .cast::<i32>())
                    .write(0i32);
                    crate::c::volatile_write(
                        (((&raw mut gSTWIStatus).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(16),
                        0u8,
                    );
                    crate::c::volatile_write(
                        (((&raw mut gSTWIStatus).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(18)
                            .cast::<u16>(),
                        0u16,
                    );
                    ((((&raw mut gSTWIStatus).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(21))
                    .write(0u8);
                    crate::c::volatile_write(((67109172i32) as usize as *mut u16), 256u16);
                    crate::c::volatile_write(((67109160i32) as usize as *mut u16), 20483u16);
                    return 0u16;
                }
            }
        }
        #[allow(unreachable_code)]
        {
            return 0u16;
        }
    }
}
pub(crate) unsafe extern "C" fn STWI_start_Command() -> i32 {
    unsafe {
        let mut imeTemp: u16 = 0u16;
        (((((((&raw mut gSTWIStatus).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(36)
            .cast::<*mut u8>())
        .read())
        .cast::<u8>())
        .cast::<u32>())
        .write(
            ((2573598720u32
                | ((((((((&raw mut gSTWIStatus).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(4))
                .read()) as i32)
                    << 8) as u32))
                | ((((((&raw mut gSTWIStatus).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(6))
                .read()) as u32)),
        );
        crate::c::volatile_write(
            ((67109152i32) as usize as *mut u32),
            ((((((&raw mut gSTWIStatus).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(36)
                .cast::<*mut u8>())
            .read())
            .cast::<u32>())
            .read(),
        );
        crate::c::volatile_write(
            (((&raw mut gSTWIStatus).cast::<u8>().cast::<*mut u8>()).read()).cast::<i32>(),
            0i32,
        );
        ((((&raw mut gSTWIStatus).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(5))
            .write(1u8);
        imeTemp = ((67109384i32) as usize as *mut u16).read_volatile();
        crate::c::volatile_write(((67109384i32) as usize as *mut u16), 0u16);
        let __p1 = ((67109376i32) as usize as *mut u16);
        crate::c::volatile_write(
            __p1,
            (((((__p1).read_volatile()) as i32)
                | crate::c::shl_i32(
                    8i32,
                    ((((((&raw mut gSTWIStatus).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(10))
                    .read()) as u32),
                )) as u16),
        );
        let __p2 = ((67109376i32) as usize as *mut u16);
        crate::c::volatile_write(__p2, (((((__p2).read_volatile()) as i32) | 128i32) as u16));
        crate::c::volatile_write(((67109384i32) as usize as *mut u16), imeTemp);
        crate::c::volatile_write(((67109160i32) as usize as *mut u16), 20611u16);
        return 0i32;
    }
}
pub(crate) unsafe extern "C" fn STWI_restart_Command() -> i32 {
    unsafe {
        if ((((((&raw mut gSTWIStatus).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(21))
            .read()) as i32)
            < 2i32
        {
            let __p1 =
                (((&raw mut gSTWIStatus).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(21);
            (__p1).write(((__p1).read()).wrapping_add(1));
            STWI_start_Command();
        } else {
            if (((((((((&raw mut gSTWIStatus).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(6))
            .read()) as i32)
                == 39i32)
                || (((((((&raw mut gSTWIStatus).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(6))
                .read()) as i32)
                    == 37i32))
                || (((((((&raw mut gSTWIStatus).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(6))
                .read()) as i32)
                    == 53i32))
                || (((((((&raw mut gSTWIStatus).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(6))
                .read()) as i32)
                    == 55i32)
            {
                crate::c::volatile_write(
                    (((&raw mut gSTWIStatus).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(18)
                        .cast::<u16>(),
                    1u16,
                );
                crate::c::volatile_write(
                    (((&raw mut gSTWIStatus).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(44),
                    0u8,
                );
                if core::mem::transmute::<_, usize>(
                    ((((&raw mut gSTWIStatus).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(24)
                        .cast::<Option<unsafe extern "C" fn()>>())
                    .read(),
                ) != 0usize
                {
                    core::mem::transmute::<_, unsafe extern "C" fn(i32, i32)>(
                        (((((&raw mut gSTWIStatus).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(24)
                            .cast::<Option<unsafe extern "C" fn()>>())
                        .read())
                        .unwrap_unchecked(),
                    )(
                        ((((((&raw mut gSTWIStatus).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(6))
                        .read()) as i32),
                        ((((((&raw mut gSTWIStatus).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(18)
                            .cast::<u16>())
                        .read_volatile()) as i32),
                    );
                }
            } else {
                crate::c::volatile_write(
                    (((&raw mut gSTWIStatus).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(18)
                        .cast::<u16>(),
                    1u16,
                );
                crate::c::volatile_write(
                    (((&raw mut gSTWIStatus).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(44),
                    0u8,
                );
                if core::mem::transmute::<_, usize>(
                    ((((&raw mut gSTWIStatus).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(24)
                        .cast::<Option<unsafe extern "C" fn()>>())
                    .read(),
                ) != 0usize
                {
                    core::mem::transmute::<_, unsafe extern "C" fn(i32, i32)>(
                        (((((&raw mut gSTWIStatus).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(24)
                            .cast::<Option<unsafe extern "C" fn()>>())
                        .read())
                        .unwrap_unchecked(),
                    )(
                        ((((((&raw mut gSTWIStatus).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(6))
                        .read()) as i32),
                        ((((((&raw mut gSTWIStatus).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(18)
                            .cast::<u16>())
                        .read_volatile()) as i32),
                    );
                }
                crate::c::volatile_write(
                    (((&raw mut gSTWIStatus).cast::<u8>().cast::<*mut u8>()).read()).cast::<i32>(),
                    4i32,
                );
            }
        }
        return 0i32;
    }
}
pub(crate) unsafe extern "C" fn STWI_reset_ClockCounter() -> i32 {
    unsafe {
        crate::c::volatile_write(
            (((&raw mut gSTWIStatus).cast::<u8>().cast::<*mut u8>()).read()).cast::<i32>(),
            5i32,
        );
        ((((&raw mut gSTWIStatus).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(4))
            .write(0u8);
        ((((&raw mut gSTWIStatus).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(5))
            .write(0u8);
        crate::c::volatile_write(((67109152i32) as usize as *mut u32), 2147483648u32);
        crate::c::volatile_write(((67109160i32) as usize as *mut u16), 0u16);
        crate::c::volatile_write(((67109160i32) as usize as *mut u16), 20483u16);
        crate::c::volatile_write(((67109160i32) as usize as *mut u16), 20610u16);
        return 0i32;
    }
}
