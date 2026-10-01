//! Translated from `src/librfu_stwi.c` by tools/rustport/c2rs.py.
#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    static_mut_refs,
    unsafe_op_in_unsafe_fn,
    clashing_extern_declarations,
    unpredictable_function_pointer_comparisons,
    dangerous_implicit_autorefs,
    overflowing_literals,
    clippy::missing_transmute_annotations,
    clippy::redundant_locals,
    dead_code,
    unused_assignments,
    unused_variables
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

#[unsafe(link_section = "common_data")]
pub static mut gSTWIStatus: *mut STWIStatus = null_mut();

/// `CpuSet` with this module's view of its types.
#[inline]
unsafe fn CpuSet(a0: *mut c_void, a1: *mut c_void, a2: u32) {
    unsafe {
        crate::syscall::CpuSet(a0 as _, a1 as _, a2);
    }
}
use crate::librfu_intr::IntrSIO32;

// hand-written: tools/rustport/overrides/librfu_stwi/STWI_init_all.rs
// c2rs-uses: IntrSIO32 gSTWIStatus STWI_init_Callback_M STWI_init_Callback_S IntrEnable
/// `STWI_init_all`, with the interrupt handler kept in ROM (see below).
#[unsafe(no_mangle)]
pub unsafe fn STWI_init_all(
    interruptStruct: *mut u8,
    interrupt: *mut Option<crate::agb_main::IntrFunc>,
    copyInterruptToRam: u8,
) {
    unsafe {
        let interruptStruct = interruptStruct;
        let interrupt = interrupt;
        let copyInterruptToRam = copyInterruptToRam;
        if ((copyInterruptToRam) as i32) == 1i32 {
            // The C DMAs 0x960 bytes from IntrSIO32 into block1 and runs the
            // interrupt from there: the size of the original librfu_intr
            // binary, whose functions call each other PC-relatively. The Rust
            // functions are not laid out as one block, so a copy would not
            // run; the handler runs from ROM instead (as in the C's other
            // branch) and block2 still holds the STWIStatus, so the RAM
            // layout is unchanged.
            (interrupt).write(Some(IntrSIO32 as crate::agb_main::IntrFunc));
            ((&raw mut gSTWIStatus).cast::<u8>().cast::<*mut u8>())
                .write((interruptStruct).wrapping_add(2632));
        } else {
            (interrupt).write(Some(IntrSIO32 as crate::agb_main::IntrFunc));
            ((&raw mut gSTWIStatus).cast::<u8>().cast::<*mut u8>())
                .write(((interruptStruct).wrapping_add(232)).cast::<u8>());
        }
        ((((&raw mut gSTWIStatus).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(40)
            .cast::<*mut u8>())
        .write(interruptStruct);
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
        crate::c::volatile_write((67109172i32) as usize as *mut u16, 256u16);
        crate::c::volatile_write((67109160i32) as usize as *mut u16, 20483u16);
        STWI_init_Callback_M();
        STWI_init_Callback_S();
        {
            let imeTemp: u16 = ((67109384i32) as usize as *mut u16).read_volatile();
            crate::c::volatile_write((67109384i32) as usize as *mut u16, 0u16);
            let __p1 = (67109376i32) as usize as *mut u16;
            crate::c::volatile_write(__p1, ((((__p1).read_volatile()) as i32) | 128i32) as u16);
            crate::c::volatile_write((67109384i32) as usize as *mut u16, imeTemp);
        }
    }
}

pub unsafe fn STWI_init_timer(interrupt: *mut Option<crate::agb_main::IntrFunc>, timerSelect: i32) {
    *interrupt = Some(STWI_intr_timer);
    (*gSTWIStatus).timerSelect = timerSelect as u8;
    {
        let imeTemp: u16 = (67109384_usize as *mut u16).read_volatile();
        volatile_write(67109384_usize as *mut u16, 0);
        volatile_write(
            0x4000200_usize as *mut u16,
            (0x4000200_usize as *mut u16).read_volatile()
                | shl_i32(INTR_FLAG_TIMER0, (*gSTWIStatus).timerSelect as u32) as u16,
        );
        volatile_write(67109384_usize as *mut u16, imeTemp);
    }
}
pub unsafe fn AgbRFU_SoftReset() {
    volatile_write(67109172_usize as *mut u16, 0x8000);
    volatile_write(67109172_usize as *mut u16, 0x80A0);
    let timerL: *mut u16 = (0x4000100 + (*gSTWIStatus).timerSelect as i32 * 4) as usize as *mut u16;
    let timerH: *mut u16 = (67109122 + (*gSTWIStatus).timerSelect as i32 * 4) as usize as *mut u16;
    volatile_write(timerH, 0);
    volatile_write(timerL, 0);
    volatile_write(timerH, 131);
    while (timerL).read_volatile() <= 0x11 {
        volatile_write(67109172_usize as *mut u16, 0x80A2);
    }
    volatile_write(timerH, 3);
    volatile_write(67109172_usize as *mut u16, 0x80A0);
    volatile_write(67109160_usize as *mut u16, 20483);
    volatile_write(&raw mut (*gSTWIStatus).state, 0);
    (*gSTWIStatus).reqLength = 0;
    (*gSTWIStatus).reqNext = 0;
    (*gSTWIStatus).reqActiveCommand = 0;
    (*gSTWIStatus).ackLength = 0;
    (*gSTWIStatus).ackNext = 0;
    (*gSTWIStatus).ackActiveCommand = 0;
    (*gSTWIStatus).timerState = 0;
    volatile_write(&raw mut (*gSTWIStatus).timerActive, 0);
    volatile_write(&raw mut (*gSTWIStatus).error, 0);
    volatile_write(&raw mut (*gSTWIStatus).msMode, AGB_CLK_MASTER);
    (*gSTWIStatus).recoveryCount = 0;
    volatile_write(&raw mut (*gSTWIStatus).sending, 0);
}
pub unsafe fn STWI_set_MS_mode(mode: u8) {
    volatile_write(&raw mut (*gSTWIStatus).msMode, mode);
}
pub unsafe fn STWI_read_status(index: u8) -> u16 {
    match index {
        0 => {
            return (&raw mut (*gSTWIStatus).error).read_volatile();
        }
        1 => {
            return (&raw mut (*gSTWIStatus).msMode).read_volatile() as u16;
        }
        2 => {
            return (&raw mut (*gSTWIStatus).state).read_volatile() as u16;
        }
        3 => {
            return (*gSTWIStatus).reqActiveCommand as u16;
        }
        _ => {
            return 0xFFFF;
        }
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
pub unsafe fn STWI_init_Callback_M() {
    STWI_set_Callback_M(null_mut());
}
pub unsafe fn STWI_init_Callback_S() {
    STWI_set_Callback_S(None);
}
pub unsafe fn STWI_set_Callback_M(callbackM: *mut c_void) {
    (*gSTWIStatus).callbackM = core::mem::transmute::<_, Option<unsafe fn()>>(callbackM);
}
pub unsafe fn STWI_set_Callback_S(callbackS: Option<unsafe fn(u16)>) {
    (*gSTWIStatus).callbackS = callbackS;
}
pub unsafe fn STWI_set_Callback_ID(func: Option<unsafe fn()>) {
    (*gSTWIStatus).callbackID = func;
}
pub unsafe fn STWI_poll_CommandEnd() -> u16 {
    while (&raw mut (*gSTWIStatus).sending).read_volatile() == 1 {}
    (&raw mut (*gSTWIStatus).error).read_volatile()
}
pub unsafe fn STWI_send_ResetREQ() {
    if STWI_init(ID_RESET_REQ as u8) == 0 {
        (*gSTWIStatus).reqLength = 0;
        STWI_start_Command();
    }
}
pub unsafe fn STWI_send_LinkStatusREQ() {
    if STWI_init(ID_LINK_STATUS_REQ) == 0 {
        (*gSTWIStatus).reqLength = 0;
        STWI_start_Command();
    }
}
pub unsafe fn STWI_send_VersionStatusREQ() {
    if STWI_init(ID_VERSION_STATUS_REQ) == 0 {
        (*gSTWIStatus).reqLength = 0;
        STWI_start_Command();
    }
}
pub unsafe fn STWI_send_SystemStatusREQ() {
    if STWI_init(ID_SYSTEM_STATUS_REQ) == 0 {
        (*gSTWIStatus).reqLength = 0;
        STWI_start_Command();
    }
}
pub unsafe fn STWI_send_SlotStatusREQ() {
    if STWI_init(ID_SLOT_STATUS_REQ) == 0 {
        (*gSTWIStatus).reqLength = 0;
        STWI_start_Command();
    }
}
pub unsafe fn STWI_send_ConfigStatusREQ() {
    if STWI_init(ID_CONFIG_STATUS_REQ) == 0 {
        (*gSTWIStatus).reqLength = 0;
        STWI_start_Command();
    }
}
pub unsafe fn STWI_send_GameConfigREQ(mut serial_gname: *mut u8, mut uname: *mut u8) {
    let mut packetBytes: *mut u8 = null_mut();
    if STWI_init(ID_GAME_CONFIG_REQ as u8) == 0 {
        (*gSTWIStatus).reqLength = 6;
        packetBytes = (*(*gSTWIStatus).txPacket).rfuPacket8.data.as_mut_ptr();
        packetBytes = packetBytes.at(4);
        *(packetBytes as *mut u16) = *(serial_gname as *mut u16);
        packetBytes = packetBytes.at(2);
        serial_gname = serial_gname.at(2);
        for i in 0..14i32 {
            *packetBytes = *serial_gname;
            packetBytes = packetBytes.at(1);
            serial_gname = serial_gname.at(1);
        }
        for i in 0..8i32 {
            *packetBytes = *uname;
            packetBytes = packetBytes.at(1);
            uname = uname.at(1);
        }
        STWI_start_Command();
    }
}
pub unsafe fn STWI_send_SystemConfigREQ(availSlotFlag: u16, maxMFrame: u8, mcTimer: u8) {
    if STWI_init(ID_SYSTEM_CONFIG_REQ as u8) == 0 {
        (*gSTWIStatus).reqLength = 1;
        let mut packetBytes: *mut u8 = (*(*gSTWIStatus).txPacket).rfuPacket8.data.as_mut_ptr();
        packetBytes = packetBytes.at(4);
        *({
            let t1 = packetBytes;
            packetBytes = packetBytes.at(1);
            t1
        }) = mcTimer;
        *({
            let t2 = packetBytes;
            packetBytes = packetBytes.at(1);
            t2
        }) = maxMFrame;
        *(packetBytes as *mut u16) = availSlotFlag;
        STWI_start_Command();
    }
}
pub unsafe fn STWI_send_SC_StartREQ() {
    if STWI_init(ID_SC_START_REQ) == 0 {
        (*gSTWIStatus).reqLength = 0;
        STWI_start_Command();
    }
}
pub unsafe fn STWI_send_SC_PollingREQ() {
    if STWI_init(ID_SC_POLL_REQ) == 0 {
        (*gSTWIStatus).reqLength = 0;
        STWI_start_Command();
    }
}
pub unsafe fn STWI_send_SC_EndREQ() {
    if STWI_init(ID_SC_END_REQ) == 0 {
        (*gSTWIStatus).reqLength = 0;
        STWI_start_Command();
    }
}
pub unsafe fn STWI_send_SP_StartREQ() {
    if STWI_init(ID_SP_START_REQ as u8) == 0 {
        (*gSTWIStatus).reqLength = 0;
        STWI_start_Command();
    }
}
pub unsafe fn STWI_send_SP_PollingREQ() {
    if STWI_init(ID_SP_POLL_REQ as u8) == 0 {
        (*gSTWIStatus).reqLength = 0;
        STWI_start_Command();
    }
}
pub unsafe fn STWI_send_SP_EndREQ() {
    if STWI_init(ID_SP_END_REQ as u8) == 0 {
        (*gSTWIStatus).reqLength = 0;
        STWI_start_Command();
    }
}
pub unsafe fn STWI_send_CP_StartREQ(unk1: u16) {
    if STWI_init(ID_CP_START_REQ) == 0 {
        (*gSTWIStatus).reqLength = 1;
        (*(*gSTWIStatus).txPacket).rfuPacket32.data[0] = unk1 as u32;
        STWI_start_Command();
    }
}
pub unsafe fn STWI_send_CP_PollingREQ() {
    if STWI_init(ID_CP_POLL_REQ as u8) == 0 {
        (*gSTWIStatus).reqLength = 0;
        STWI_start_Command();
    }
}
pub unsafe fn STWI_send_CP_EndREQ() {
    if STWI_init(ID_CP_END_REQ as u8) == 0 {
        (*gSTWIStatus).reqLength = 0;
        STWI_start_Command();
    }
}
pub unsafe fn STWI_send_DataTxREQ(r#in: *mut c_void, size: u8) {
    if STWI_init(ID_DATA_TX_REQ) == 0 {
        let mut reqLength: u8 = size / 4;
        if size as u32 & 3 != 0 {
            reqLength += 1;
        }
        (*gSTWIStatus).reqLength = reqLength;
        CpuSet(
            r#in,
            (*(*gSTWIStatus).txPacket).rfuPacket32.data.as_mut_ptr() as *mut c_void,
            0x04000000 | ((*gSTWIStatus).reqLength as u32 * 4 / 4) & 0x1FFFFF,
        );
        STWI_start_Command();
    }
}
pub unsafe fn STWI_send_DataTxAndChangeREQ(r#in: *mut c_void, size: u8) {
    if STWI_init(ID_DATA_TX_AND_CHANGE_REQ) == 0 {
        let mut reqLength: u8 = size / 4;
        if size as u32 & 3 != 0 {
            reqLength += 1;
        }
        (*gSTWIStatus).reqLength = reqLength;
        CpuSet(
            r#in,
            (*(*gSTWIStatus).txPacket).rfuPacket32.data.as_mut_ptr() as *mut c_void,
            0x04000000 | ((*gSTWIStatus).reqLength as u32 * 4 / 4) & 0x1FFFFF,
        );
        STWI_start_Command();
    }
}
pub unsafe fn STWI_send_DataRxREQ() {
    if STWI_init(ID_DATA_RX_REQ as u8) == 0 {
        (*gSTWIStatus).reqLength = 0;
        STWI_start_Command();
    }
}
pub unsafe fn STWI_send_MS_ChangeREQ() {
    if STWI_init(ID_MS_CHANGE_REQ) == 0 {
        (*gSTWIStatus).reqLength = 0;
        STWI_start_Command();
    }
}
pub unsafe fn STWI_send_DataReadyAndChangeREQ(unk: u8) {
    if STWI_init(ID_DATA_READY_AND_CHANGE_REQ) == 0 {
        if unk == 0 {
            (*gSTWIStatus).reqLength = 0;
        } else {
            (*gSTWIStatus).reqLength = 1;
            let mut packetBytes: *mut u8 = (*(*gSTWIStatus).txPacket).rfuPacket8.data.as_mut_ptr();
            packetBytes = packetBytes.at(4);
            *({
                let t1 = packetBytes;
                packetBytes = packetBytes.at(1);
                t1
            }) = unk;
            *({
                let t2 = packetBytes;
                packetBytes = packetBytes.at(1);
                t2
            }) = 0;
            *({
                let t3 = packetBytes;
                packetBytes = packetBytes.at(1);
                t3
            }) = 0;
            *packetBytes = 0;
        }
        STWI_start_Command();
    }
}
pub unsafe fn STWI_send_DisconnectedAndChangeREQ(unk0: u8, unk1: u8) {
    if STWI_init(ID_DISCONNECTED_AND_CHANGE_REQ) == 0 {
        (*gSTWIStatus).reqLength = 1;
        let mut packetBytes: *mut u8 = (*(*gSTWIStatus).txPacket).rfuPacket8.data.as_mut_ptr();
        packetBytes = packetBytes.at(4);
        *({
            let t1 = packetBytes;
            packetBytes = packetBytes.at(1);
            t1
        }) = unk0;
        *({
            let t2 = packetBytes;
            packetBytes = packetBytes.at(1);
            t2
        }) = unk1;
        *({
            let t3 = packetBytes;
            packetBytes = packetBytes.at(1);
            t3
        }) = 0;
        *packetBytes = 0;
        STWI_start_Command();
    }
}
pub unsafe fn STWI_send_ResumeRetransmitAndChangeREQ() {
    if STWI_init(ID_RESUME_RETRANSMIT_AND_CHANGE_REQ) == 0 {
        (*gSTWIStatus).reqLength = 0;
        STWI_start_Command();
    }
}
pub unsafe fn STWI_send_DisconnectREQ(unk: u8) {
    if STWI_init(ID_DISCONNECT_REQ as u8) == 0 {
        (*gSTWIStatus).reqLength = 1;
        (*(*gSTWIStatus).txPacket).rfuPacket32.data[0] = unk as u32;
        STWI_start_Command();
    }
}
pub unsafe fn STWI_send_TestModeREQ(unk0: u8, unk1: u8) {
    if STWI_init(ID_TEST_MODE_REQ) == 0 {
        (*gSTWIStatus).reqLength = 1;
        (*(*gSTWIStatus).txPacket).rfuPacket32.data[0] = unk0 as u32 | (unk1 as u32) << 8;
        STWI_start_Command();
    }
}
pub unsafe fn STWI_send_CPR_StartREQ(unk0: u16, unk1: u16, unk2: u8) {
    let mut packetData: *mut u32 = null_mut();
    let mut arg1: u32 = 0;
    if STWI_init(ID_CPR_START_REQ as u8) == 0 {
        (*gSTWIStatus).reqLength = 2;
        arg1 = unk1 as u32 | (unk0 as u32) << 16;
        packetData = (*(*gSTWIStatus).txPacket).rfuPacket32.data.as_mut_ptr();
        *packetData = arg1;
        *packetData.at(1) = unk2 as u32;
        STWI_start_Command();
    }
}
pub unsafe fn STWI_send_CPR_PollingREQ() {
    if STWI_init(ID_CPR_POLL_REQ as u8) == 0 {
        (*gSTWIStatus).reqLength = 0;
        STWI_start_Command();
    }
}
pub unsafe fn STWI_send_CPR_EndREQ() {
    if STWI_init(ID_CPR_END_REQ as u8) == 0 {
        (*gSTWIStatus).reqLength = 0;
        STWI_start_Command();
    }
}
pub unsafe fn STWI_send_StopModeREQ() {
    if STWI_init(ID_STOP_MODE_REQ as u8) == 0 {
        (*gSTWIStatus).reqLength = 0;
        STWI_start_Command();
    }
}
pub(crate) unsafe extern "C" fn STWI_intr_timer() {
    match (*gSTWIStatus).timerState {
        2 => {
            volatile_write(&raw mut (*gSTWIStatus).timerActive, 1);
            STWI_set_timer(50);
        }
        1 | 4 => {
            STWI_stop_timer();
            STWI_restart_Command();
        }
        3 => {
            volatile_write(&raw mut (*gSTWIStatus).timerActive, 1);
            STWI_stop_timer();
            STWI_reset_ClockCounter();
            if (*gSTWIStatus).callbackM.is_some() {
                core::mem::transmute::<_, unsafe fn(i32, i32)>(
                    (*gSTWIStatus).callbackM.unwrap_unchecked(),
                )(ID_CLOCK_SLAVE_MS_CHANGE_ERROR_BY_DMA_REQ as i32, 0);
            }
        }
        _ => {}
    }
}
unsafe fn STWI_set_timer(count: u8) {
    let timerL: *mut u16 = (0x4000100 + (*gSTWIStatus).timerSelect as i32 * 4) as usize as *mut u16;
    let timerH: *mut u16 = (67109122 + (*gSTWIStatus).timerSelect as i32 * 4) as usize as *mut u16;
    volatile_write(67109384_usize as *mut u16, 0);
    match count {
        50 => {
            volatile_write(timerL, 0xFCCB);
            (*gSTWIStatus).timerState = 1;
        }
        80 => {
            volatile_write(timerL, 0xFAE0);
            (*gSTWIStatus).timerState = 2;
        }
        100 => {
            volatile_write(timerL, 0xF996);
            (*gSTWIStatus).timerState = 3;
        }
        130 => {
            volatile_write(timerL, 0xF7AD);
            (*gSTWIStatus).timerState = 4;
        }
        _ => {}
    }
    volatile_write(timerH, 195);
    volatile_write(
        67109378_usize as *mut u16,
        shl_i32(INTR_FLAG_TIMER0, (*gSTWIStatus).timerSelect as u32) as u16,
    );
    volatile_write(67109384_usize as *mut u16, 1);
}
unsafe fn STWI_stop_timer() {
    (*gSTWIStatus).timerState = 0;
    volatile_write(
        (0x4000100 + (*gSTWIStatus).timerSelect as i32 * 4) as usize as *mut u16,
        0,
    );
    volatile_write(
        (67109122 + (*gSTWIStatus).timerSelect as i32 * 4) as usize as *mut u16,
        0,
    );
}
unsafe fn STWI_init(request: u8) -> u16 {
    if (67109384_usize as *mut u16).read_volatile() == 0 {
        volatile_write(&raw mut (*gSTWIStatus).error, ERR_REQ_CMD_IME_DISABLE);
        if (*gSTWIStatus).callbackM.is_some() {
            core::mem::transmute::<_, unsafe fn(i32, i32)>(
                (*gSTWIStatus).callbackM.unwrap_unchecked(),
            )(
                request as i32,
                (&raw mut (*gSTWIStatus).error).read_volatile() as i32,
            );
        }
        return TRUE as u16;
    } else if (&raw mut (*gSTWIStatus).sending).read_volatile() == 1 {
        volatile_write(&raw mut (*gSTWIStatus).error, ERR_REQ_CMD_SENDING);
        volatile_write(&raw mut (*gSTWIStatus).sending, 0);
        if (*gSTWIStatus).callbackM.is_some() {
            core::mem::transmute::<_, unsafe fn(i32, i32)>(
                (*gSTWIStatus).callbackM.unwrap_unchecked(),
            )(
                request as i32,
                (&raw mut (*gSTWIStatus).error).read_volatile() as i32,
            );
        }
        return TRUE as u16;
    } else if (&raw mut (*gSTWIStatus).msMode).read_volatile() == AGB_CLK_SLAVE {
        volatile_write(&raw mut (*gSTWIStatus).error, ERR_REQ_CMD_CLOCK_SLAVE);
        if (*gSTWIStatus).callbackM.is_some() {
            core::mem::transmute::<_, unsafe fn(i32, i32, *mut STWIStatus)>(
                (*gSTWIStatus).callbackM.unwrap_unchecked(),
            )(
                request as i32,
                (&raw mut (*gSTWIStatus).error).read_volatile() as i32,
                gSTWIStatus,
            );
        }
        return TRUE as u16;
    } else {
        volatile_write(&raw mut (*gSTWIStatus).sending, 1);
        (*gSTWIStatus).reqActiveCommand = request;
        volatile_write(&raw mut (*gSTWIStatus).state, 0);
        (*gSTWIStatus).reqLength = 0;
        (*gSTWIStatus).reqNext = 0;
        (*gSTWIStatus).ackLength = 0;
        (*gSTWIStatus).ackNext = 0;
        (*gSTWIStatus).ackActiveCommand = 0;
        (*gSTWIStatus).timerState = 0;
        volatile_write(&raw mut (*gSTWIStatus).timerActive, 0);
        volatile_write(&raw mut (*gSTWIStatus).error, 0);
        (*gSTWIStatus).recoveryCount = 0;
        volatile_write(67109172_usize as *mut u16, 0x100);
        volatile_write(67109160_usize as *mut u16, 20483);
        return FALSE as u16;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
unsafe fn STWI_start_Command() -> i32 {
    *((*(*gSTWIStatus).txPacket).rfuPacket8.data.as_mut_ptr() as *mut u32) = 0x99660000
        | ((*gSTWIStatus).reqLength as u32) << 8
        | (*gSTWIStatus).reqActiveCommand as u32;
    volatile_write(
        67109152_usize as *mut u32,
        (*(*gSTWIStatus).txPacket).rfuPacket32.command,
    );
    volatile_write(&raw mut (*gSTWIStatus).state, 0);
    (*gSTWIStatus).reqNext = 1;
    let imeTemp: u16 = (67109384_usize as *mut u16).read_volatile();
    volatile_write(67109384_usize as *mut u16, 0);
    volatile_write(
        0x4000200_usize as *mut u16,
        (0x4000200_usize as *mut u16).read_volatile()
            | shl_i32(INTR_FLAG_TIMER0, (*gSTWIStatus).timerSelect as u32) as u16,
    );
    volatile_write(
        0x4000200_usize as *mut u16,
        (0x4000200_usize as *mut u16).read_volatile() | INTR_FLAG_SERIAL,
    );
    volatile_write(67109384_usize as *mut u16, imeTemp);
    volatile_write(67109160_usize as *mut u16, 20611);
    0
}
unsafe fn STWI_restart_Command() -> i32 {
    if (*gSTWIStatus).recoveryCount < 2 {
        (*gSTWIStatus).recoveryCount += 1;
        STWI_start_Command();
    } else {
        if (*gSTWIStatus).reqActiveCommand == ID_MS_CHANGE_REQ
            || (*gSTWIStatus).reqActiveCommand == ID_DATA_TX_AND_CHANGE_REQ
            || (*gSTWIStatus).reqActiveCommand == ID_UNK35_REQ
            || (*gSTWIStatus).reqActiveCommand == ID_RESUME_RETRANSMIT_AND_CHANGE_REQ
        {
            volatile_write(&raw mut (*gSTWIStatus).error, ERR_REQ_CMD_CLOCK_DRIFT);
            volatile_write(&raw mut (*gSTWIStatus).sending, 0);
            if (*gSTWIStatus).callbackM.is_some() {
                core::mem::transmute::<_, unsafe fn(i32, i32)>(
                    (*gSTWIStatus).callbackM.unwrap_unchecked(),
                )(
                    (*gSTWIStatus).reqActiveCommand as i32,
                    (&raw mut (*gSTWIStatus).error).read_volatile() as i32,
                );
            }
        } else {
            volatile_write(&raw mut (*gSTWIStatus).error, ERR_REQ_CMD_CLOCK_DRIFT);
            volatile_write(&raw mut (*gSTWIStatus).sending, 0);
            if (*gSTWIStatus).callbackM.is_some() {
                core::mem::transmute::<_, unsafe fn(i32, i32)>(
                    (*gSTWIStatus).callbackM.unwrap_unchecked(),
                )(
                    (*gSTWIStatus).reqActiveCommand as i32,
                    (&raw mut (*gSTWIStatus).error).read_volatile() as i32,
                );
            }
            volatile_write(&raw mut (*gSTWIStatus).state, 4);
        }
    }
    0
}
unsafe fn STWI_reset_ClockCounter() -> i32 {
    volatile_write(&raw mut (*gSTWIStatus).state, 5);
    (*gSTWIStatus).reqLength = 0;
    (*gSTWIStatus).reqNext = 0;
    volatile_write(67109152_usize as *mut u32, 0x80000000);
    volatile_write(67109160_usize as *mut u16, 0);
    volatile_write(67109160_usize as *mut u16, 20483);
    volatile_write(67109160_usize as *mut u16, 20610);
    0
}
