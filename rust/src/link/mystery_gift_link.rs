//! Block-transfer protocol for the Mystery Gift client and server: a small
//! header (identifier, CRC, size), then the payload in 252-byte blocks, then a
//! CRC check.

use crate::util::CalcCRC16WithTable;

const MG_LINK_BUFFER_SIZE: u32 = 0x400;
/// Payload bytes per link block.
const BLOCK_PAYLOAD: u32 = 252;
/// `sizeof(gBlockRecvBuffer[0])`
const BLOCK_BUFFER_SIZE: usize = 0x100;

/// `struct MysteryGiftLink`, 40 bytes.
const L_STATE: usize = 0x00;
const L_SEND_PLAYER_ID: usize = 0x04;
const L_RECV_PLAYER_ID: usize = 0x05;
const L_RECV_IDENT: usize = 0x06;
const L_RECV_COUNTER: usize = 0x08;
const L_RECV_CRC: usize = 0x0a;
const L_RECV_SIZE: usize = 0x0c;
const L_SEND_IDENT: usize = 0x0e;
const L_SEND_COUNTER: usize = 0x10;
const L_SEND_CRC: usize = 0x12;
const L_SEND_SIZE: usize = 0x14;
const L_RECV_BUFFER: usize = 0x18;
const L_SEND_BUFFER: usize = 0x1c;
const L_RECV_FUNC: usize = 0x20;
const L_SEND_FUNC: usize = 0x24;

type LinkFunc = unsafe fn(*mut u8) -> u32;

/// `struct SendRecvHeader { u16 ident, crc, size; }`. Six used bytes, but
/// APCS rounds it to eight and `sizeof` is what goes over the link.
#[repr(C, align(4))]
#[derive(Default)]
struct SendRecvHeader {
    ident: u16,
    crc: u16,
    size: u16,
}
const HEADER_SIZE: u16 = 8;

/// `GetBlockReceivedStatus` with this module's view of its types.
#[inline]
unsafe fn GetBlockReceivedStatus() -> u8 {
    unsafe { crate::link::GetBlockReceivedStatus() }
}
/// `ResetBlockReceivedFlag` with this module's view of its types.
#[inline]
unsafe fn ResetBlockReceivedFlag(a0: u8) {
    unsafe {
        crate::link::ResetBlockReceivedFlag(a0);
    }
}
/// `SendBlock` with this module's view of its types.
#[inline]
unsafe fn SendBlock(a0: u8, a1: *const u8, a2: u16) -> u8 {
    unsafe { crate::link::SendBlock(a0, a1 as _, a2) }
}
/// `IsLinkTaskFinished` with this module's view of its types.
#[inline]
unsafe fn IsLinkTaskFinished() -> u8 {
    unsafe { crate::link::IsLinkTaskFinished() }
}
/// `LinkRfu_FatalError` with this module's view of its types.
#[inline]
unsafe fn LinkRfu_FatalError() {
    unsafe {
        crate::link_rfu_2::LinkRfu_FatalError();
    }
}

#[inline]
unsafe fn u8_at(link: *mut u8, offset: usize) -> u8 {
    unsafe { link.add(offset).read_volatile() }
}

#[inline]
unsafe fn u16_at(link: *mut u8, offset: usize) -> u16 {
    unsafe { link.add(offset).cast::<u16>().read_volatile() }
}

#[inline]
unsafe fn set_u16(link: *mut u8, offset: usize, value: u16) {
    unsafe { link.add(offset).cast::<u16>().write_volatile(value) };
}

#[inline]
unsafe fn state(link: *mut u8) -> i32 {
    unsafe { link.add(L_STATE).cast::<i32>().read_volatile() }
}

#[inline]
unsafe fn set_state(link: *mut u8, value: i32) {
    unsafe { link.add(L_STATE).cast::<i32>().write_volatile(value) };
}

#[inline]
unsafe fn ptr_at(link: *mut u8, offset: usize) -> *mut u8 {
    unsafe { link.add(offset).cast::<*mut u8>().read() }
}

#[unsafe(no_mangle)]
pub unsafe fn MysteryGiftLink_Recv(link: *mut u8) -> u32 {
    let func = unsafe { link.add(L_RECV_FUNC).cast::<LinkFunc>().read() };
    unsafe { func(link) }
}

#[unsafe(no_mangle)]
pub unsafe fn MysteryGiftLink_Send(link: *mut u8) -> u32 {
    let func = unsafe { link.add(L_SEND_FUNC).cast::<LinkFunc>().read() };
    unsafe { func(link) }
}

#[unsafe(no_mangle)]
pub unsafe fn MysteryGiftLink_Init(link: *mut u8, send_player_id: u32, recv_player_id: u32) {
    unsafe {
        link.add(L_SEND_PLAYER_ID)
            .write_volatile(send_player_id as u8)
    };
    unsafe {
        link.add(L_RECV_PLAYER_ID)
            .write_volatile(recv_player_id as u8)
    };
    unsafe { set_state(link, 0) };
    for offset in [
        L_SEND_CRC,
        L_SEND_SIZE,
        L_SEND_COUNTER,
        L_RECV_CRC,
        L_RECV_SIZE,
        L_RECV_COUNTER,
    ] {
        unsafe { set_u16(link, offset, 0) };
    }
    unsafe {
        link.add(L_SEND_BUFFER)
            .cast::<*const u8>()
            .write(core::ptr::null())
    };
    unsafe {
        link.add(L_RECV_BUFFER)
            .cast::<*mut u8>()
            .write(core::ptr::null_mut())
    };
    unsafe { link.add(L_SEND_FUNC).cast::<LinkFunc>().write(mgl_send) };
    unsafe { link.add(L_RECV_FUNC).cast::<LinkFunc>().write(mgl_receive) };
}

#[unsafe(no_mangle)]
pub unsafe fn MysteryGiftLink_InitSend(link: *mut u8, ident: u32, src: *const u8, size: u32) {
    unsafe { set_state(link, 0) };
    unsafe { set_u16(link, L_SEND_IDENT, ident as u16) };
    unsafe { set_u16(link, L_SEND_COUNTER, 0) };
    unsafe { set_u16(link, L_SEND_CRC, 0) };
    // A zero size means "the whole buffer".
    let size = if size != 0 { size } else { MG_LINK_BUFFER_SIZE };
    unsafe { set_u16(link, L_SEND_SIZE, size as u16) };
    unsafe { link.add(L_SEND_BUFFER).cast::<*const u8>().write(src) };
}

#[unsafe(no_mangle)]
pub unsafe fn MysteryGiftLink_InitRecv(link: *mut u8, ident: u32, dest: *mut u8) {
    unsafe { set_state(link, 0) };
    unsafe { set_u16(link, L_RECV_IDENT, ident as u16) };
    unsafe { set_u16(link, L_RECV_COUNTER, 0) };
    unsafe { set_u16(link, L_RECV_CRC, 0) };
    unsafe { set_u16(link, L_RECV_SIZE, 0) };
    unsafe { link.add(L_RECV_BUFFER).cast::<*mut u8>().write(dest) };
}

#[inline]
unsafe fn receive_block(player_id: u8, dest: *mut u8, size: usize) {
    let src = unsafe {
        (&raw const (*(&raw const crate::link::gBlockRecvBuffer)
            .cast::<u8>()
            .cast_mut()))
            .add(player_id as usize * BLOCK_BUFFER_SIZE)
    };
    unsafe { core::ptr::copy_nonoverlapping(src, dest, size) };
}

#[inline]
unsafe fn has_received(player_id: u8) -> bool {
    (unsafe { GetBlockReceivedStatus() } >> player_id) & 1 != 0
}

unsafe fn mgl_receive(link: *mut u8) -> u32 {
    let player = unsafe { u8_at(link, L_RECV_PLAYER_ID) };

    match unsafe { state(link) } {
        0 => {
            if !unsafe { has_received(player) } {
                return 0;
            }
            let mut header = SendRecvHeader::default();
            unsafe { receive_block(player, (&raw mut header).cast(), HEADER_SIZE as usize) };
            unsafe { set_u16(link, L_RECV_SIZE, header.size) };
            unsafe { set_u16(link, L_RECV_CRC, header.crc) };

            if u32::from(header.size) > MG_LINK_BUFFER_SIZE
                || unsafe { u16_at(link, L_RECV_IDENT) } != header.ident
            {
                unsafe { LinkRfu_FatalError() };
                return 0;
            }

            unsafe { set_u16(link, L_RECV_COUNTER, 0) };
            unsafe { ResetBlockReceivedFlag(player) };
            unsafe { set_state(link, 1) };
        }
        1 => {
            if !unsafe { has_received(player) } {
                return 0;
            }
            let counter = unsafe { u16_at(link, L_RECV_COUNTER) };
            let offset = u32::from(counter) * BLOCK_PAYLOAD;
            let remaining = u32::from(unsafe { u16_at(link, L_RECV_SIZE) }).wrapping_sub(offset);
            let dest = unsafe { ptr_at(link, L_RECV_BUFFER).add(offset as usize) };

            // The last block carries whatever is left; earlier ones are full.
            if remaining <= BLOCK_PAYLOAD {
                unsafe { receive_block(player, dest, remaining as usize) };
                unsafe { set_u16(link, L_RECV_COUNTER, counter.wrapping_add(1)) };
                unsafe { set_state(link, 2) };
            } else {
                unsafe { receive_block(player, dest, BLOCK_PAYLOAD as usize) };
                unsafe { set_u16(link, L_RECV_COUNTER, counter.wrapping_add(1)) };
            }
            unsafe { ResetBlockReceivedFlag(player) };
        }
        2 => {
            let buffer = unsafe { ptr_at(link, L_RECV_BUFFER) };
            let size = u32::from(unsafe { u16_at(link, L_RECV_SIZE) });
            if unsafe { CalcCRC16WithTable(buffer, size) } != unsafe { u16_at(link, L_RECV_CRC) } {
                unsafe { LinkRfu_FatalError() };
                return 0;
            }
            unsafe { set_state(link, 0) };
            return 1;
        }
        _ => {}
    }

    0
}

unsafe fn mgl_send(link: *mut u8) -> u32 {
    let player = unsafe { u8_at(link, L_SEND_PLAYER_ID) };
    let buffer = unsafe { ptr_at(link, L_SEND_BUFFER) };
    let size = unsafe { u16_at(link, L_SEND_SIZE) };

    match unsafe { state(link) } {
        0 => {
            if unsafe { IsLinkTaskFinished() } == 0 {
                return 0;
            }
            let header = SendRecvHeader {
                ident: unsafe { u16_at(link, L_SEND_IDENT) },
                size,
                crc: unsafe { CalcCRC16WithTable(buffer, u32::from(size)) },
            };
            unsafe { set_u16(link, L_SEND_CRC, header.crc) };
            unsafe { set_u16(link, L_SEND_COUNTER, 0) };
            unsafe { SendBlock(0, (&raw const header).cast(), HEADER_SIZE) };
            unsafe { set_state(link, 1) };
        }
        1 => {
            if unsafe { IsLinkTaskFinished() } == 0 || !unsafe { has_received(player) } {
                return 0;
            }
            unsafe { ResetBlockReceivedFlag(player) };
            let counter = unsafe { u16_at(link, L_SEND_COUNTER) };
            let offset = BLOCK_PAYLOAD * u32::from(counter);
            let remaining = u32::from(size).wrapping_sub(offset);
            let src = unsafe { buffer.add(offset as usize) };

            if remaining <= BLOCK_PAYLOAD {
                unsafe { SendBlock(0, src, remaining as u16) };
                unsafe { set_u16(link, L_SEND_COUNTER, counter.wrapping_add(1)) };
                unsafe { set_state(link, 2) };
            } else {
                unsafe { SendBlock(0, src, BLOCK_PAYLOAD as u16) };
                unsafe { set_u16(link, L_SEND_COUNTER, counter.wrapping_add(1)) };
            }
        }
        2 => {
            if unsafe { IsLinkTaskFinished() } == 0 {
                return 0;
            }
            if unsafe { CalcCRC16WithTable(buffer, u32::from(size)) }
                != unsafe { u16_at(link, L_SEND_CRC) }
            {
                unsafe { LinkRfu_FatalError() };
            } else {
                unsafe { set_state(link, 3) };
            }
        }
        3 => {
            if unsafe { has_received(player) } {
                unsafe { ResetBlockReceivedFlag(player) };
                unsafe { set_state(link, 0) };
                return 1;
            }
        }
        _ => {}
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_header_is_padded_to_eight_bytes_like_apcs() {
        assert_eq!(core::mem::size_of::<SendRecvHeader>(), HEADER_SIZE as usize);
    }

    #[test]
    fn link_field_offsets_match_the_arm_structure() {
        assert_eq!(L_RECV_BUFFER, 0x18);
        assert_eq!(L_SEND_FUNC + 4, 0x28);
    }

    #[test]
    fn a_full_buffer_takes_five_blocks() {
        // 1024 bytes in 252-byte blocks: four full and a 16-byte tail.
        let blocks = MG_LINK_BUFFER_SIZE.div_ceil(BLOCK_PAYLOAD);
        assert_eq!(blocks, 5);
    }
}
