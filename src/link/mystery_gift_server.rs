//! The Mystery Gift server: the sending side of a Wonder Card or Wonder News
//! exchange. It runs a script of commands from ROM, which may jump to other
//! scripts, and drives the link protocol.

use crate::ffi::{AllocZeroed, Free};
use crate::mystery_gift_link::{
    MysteryGiftLink_Init, MysteryGiftLink_InitRecv, MysteryGiftLink_InitSend, MysteryGiftLink_Recv,
    MysteryGiftLink_Send,
};

const MG_LINK_BUFFER_SIZE: u32 = 0x400;

const FUNC_INIT: u32 = 0;
const FUNC_DONE: u32 = 1;
const FUNC_RECV: u32 = 2;
const FUNC_SEND: u32 = 3;
const FUNC_RUN: u32 = 4;

const SVR_RET_INIT: u32 = 0;
const SVR_RET_ACTIVE: u32 = 1;
const SVR_RET_END: u32 = 3;

const SVR_RETURN: u32 = 0x00;
const SVR_SEND: u32 = 0x01;
const SVR_RECV: u32 = 0x02;
const SVR_GOTO: u32 = 0x03;
const SVR_GOTO_IF_EQ: u32 = 0x04;
const SVR_COPY_GAME_DATA: u32 = 0x05;
const SVR_CHECK_GAME_DATA_CARD: u32 = 0x06;
const SVR_CHECK_EXISTING_CARD: u32 = 0x07;
const SVR_READ_RESPONSE: u32 = 0x08;
const SVR_CHECK_EXISTING_STAMPS: u32 = 0x09;
const SVR_GET_CARD_STAT: u32 = 0x0a;
const SVR_CHECK_QUESTIONNAIRE: u32 = 0x0b;
const SVR_COMPARE: u32 = 0x0c;
const SVR_LOAD_CARD: u32 = 0x0d;
const SVR_LOAD_NEWS: u32 = 0x0e;
const SVR_LOAD_RAM_SCRIPT: u32 = 0x0f;
const SVR_LOAD_STAMP: u32 = 0x10;
const SVR_LOAD_UNK_2: u32 = 0x11;
const SVR_LOAD_CLIENT_SCRIPT: u32 = 0x12;
const SVR_LOAD_EREADER_TRAINER: u32 = 0x13;
const SVR_LOAD_MSG: u32 = 0x14;
const SVR_COPY_STAMP: u32 = 0x15;
const SVR_COPY_CARD: u32 = 0x16;
const SVR_COPY_NEWS: u32 = 0x17;
const SVR_SET_RAM_SCRIPT: u32 = 0x18;
const SVR_SET_CLIENT_SCRIPT: u32 = 0x19;
const SVR_COPY_SAVED_CARD: u32 = 0x1a;
const SVR_COPY_SAVED_NEWS: u32 = 0x1b;
const SVR_COPY_SAVED_RAM_SCRIPT: u32 = 0x1c;
const SVR_LOAD_UNK_1: u32 = 0x1d;
const SVR_CHECK_GAME_DATA_NEWS: u32 = 0x1e;

const MG_LINKID_CLIENT_SCRIPT: u32 = 0x10;
const MG_LINKID_DYNAMIC_MSG: u32 = 0x15;
const MG_LINKID_CARD: u32 = 0x16;
const MG_LINKID_NEWS: u32 = 0x17;
const MG_LINKID_STAMP: u32 = 0x18;
const MG_LINKID_RAM_SCRIPT: u32 = 0x19;
const MG_LINKID_EREADER_TRAINER: u32 = 0x1a;
const MG_LINKID_UNK_1: u32 = 0x1b;
const MG_LINKID_UNK_2: u32 = 0x1c;

const WONDER_CARD_SIZE: u32 = 0x14c;
const WONDER_NEWS_SIZE: u32 = 0x1bc;
const LINK_GAME_DATA_SIZE: u32 = 100;
const EREADER_TRAINER_SIZE: u32 = 0xbc;

/// `struct MysteryGiftServer`, 96 bytes.
const S_SIZE: u32 = 0x60;
const S_UNUSED: usize = 0x00;
const S_PARAM: usize = 0x04;
const S_FUNC_ID: usize = 0x08;
const S_CMDIDX: usize = 0x0c;
const S_SCRIPT: usize = 0x10;
const S_RECV_BUFFER: usize = 0x14;
const S_CARD: usize = 0x18;
const S_NEWS: usize = 0x1c;
const S_LINK_GAME_DATA: usize = 0x20;
const S_RAM_SCRIPT: usize = 0x24;
const S_RAM_SCRIPT_SIZE: usize = 0x28;
const S_CLIENT_SCRIPT: usize = 0x2c;
const S_CLIENT_SCRIPT_SIZE: usize = 0x30;
const S_STAMP: usize = 0x34;
const S_LINK: usize = 0x38;
/// `struct MysteryGiftServerCmd { u32 instr; u32 parameter; const void *ptr; }`
const CMD_SIZE: usize = 12;

#[unsafe(link_section = "ewram_data")]
static mut SERVER: *mut u8 = core::ptr::null_mut();

unsafe extern "C" {
    static gMysteryGiftServerScript_SendWonderNews: u8;
    static gMysteryGiftServerScript_SendWonderCard: u8;

    fn MysteryGift_ValidateLinkGameData(data: *const u8, is_wonder_news: u32) -> u32;
    fn MysteryGift_CompareCardFlags(flag_id: *const u16, data: *const u8, unused: *const u8)
    -> u32;
    fn MysteryGift_CheckStamps(stamp: *const u16, data: *const u8, unused: *const u8) -> u32;
    fn MysteryGift_DoesQuestionnaireMatch(data: *const u8, words: *const u16) -> u32;
    fn MysteryGift_GetCardStatFromLinkData(data: *const u8, stat: u32) -> u16;
    fn GetSavedWonderCard() -> *mut u8;
    fn GetSavedWonderNews() -> *mut u8;
    fn DisableWonderCardSending(card: *mut u8);
    fn GetSavedRamScriptIfValid() -> *mut u8;
}

#[inline]
unsafe fn field(server: *mut u8, offset: usize) -> u32 {
    unsafe { server.add(offset).cast::<u32>().read_volatile() }
}

#[inline]
unsafe fn set_field(server: *mut u8, offset: usize, value: u32) {
    unsafe { server.add(offset).cast::<u32>().write_volatile(value) };
}

#[inline]
unsafe fn ptr_field(server: *mut u8, offset: usize) -> *mut u8 {
    unsafe { server.add(offset).cast::<*mut u8>().read() }
}

#[inline]
unsafe fn set_ptr_field(server: *mut u8, offset: usize, value: *const u8) {
    unsafe { server.add(offset).cast::<*const u8>().write(value) };
}

#[inline]
unsafe fn link(server: *mut u8) -> *mut u8 {
    unsafe { server.add(S_LINK) }
}

/// A command's pointer overrides the server's own buffer when it is set.
#[inline]
fn send_data(dynamic: *const u8, default: *const u8) -> *const u8 {
    if dynamic.is_null() { default } else { dynamic }
}

/// Orders two pointers: 0 if `b < a`, 1 if equal, 2 if `b > a`.
#[inline]
fn compare(a: *const u8, b: *const u8) -> u32 {
    if (b as usize) < (a as usize) {
        0
    } else if b == a {
        1
    } else {
        2
    }
}

unsafe fn create(script: *const u8) {
    let server = unsafe { AllocZeroed(S_SIZE) };
    unsafe { (&raw mut SERVER).write(server) };
    unsafe { server_init(server, script, 0, 1) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn MysterGiftServer_CreateForNews() {
    unsafe { create(&raw const gMysteryGiftServerScript_SendWonderNews) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn MysterGiftServer_CreateForCard() {
    unsafe { create(&raw const gMysteryGiftServerScript_SendWonderCard) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn MysterGiftServer_Run(end_val: *mut u16) -> u32 {
    let server = unsafe { (&raw const SERVER).read() };
    if server.is_null() {
        return SVR_RET_END;
    }

    let result = unsafe { call_func(server) };
    if result == SVR_RET_END {
        unsafe { end_val.write(field(server, S_PARAM) as u16) };
        unsafe { server_free(server) };
        unsafe { Free(server) };
        unsafe { (&raw mut SERVER).write(core::ptr::null_mut()) };
    }
    result
}

unsafe fn server_init(
    server: *mut u8,
    script: *const u8,
    send_player_id: u32,
    recv_player_id: u32,
) {
    unsafe { set_field(server, S_UNUSED, 0) };
    unsafe { set_field(server, S_FUNC_ID, FUNC_INIT) };
    for (offset, size) in [
        (S_CARD, WONDER_CARD_SIZE),
        (S_NEWS, WONDER_NEWS_SIZE),
        (S_RECV_BUFFER, MG_LINK_BUFFER_SIZE),
        (S_LINK_GAME_DATA, LINK_GAME_DATA_SIZE),
    ] {
        let allocation = unsafe { AllocZeroed(size) };
        unsafe { set_ptr_field(server, offset, allocation) };
    }
    unsafe { set_ptr_field(server, S_SCRIPT, script) };
    unsafe { set_field(server, S_CMDIDX, 0) };
    unsafe { MysteryGiftLink_Init(link(server), send_player_id, recv_player_id) };
}

unsafe fn server_free(server: *mut u8) {
    for offset in [S_CARD, S_NEWS, S_RECV_BUFFER, S_LINK_GAME_DATA] {
        unsafe { Free(ptr_field(server, offset)) };
    }
}

#[inline]
unsafe fn init_send(server: *mut u8, ident: u32, src: *const u8, size: u32) {
    unsafe { MysteryGiftLink_InitSend(link(server), ident, src, size) };
}

#[inline]
unsafe fn copy(src: *const u8, dest: *mut u8, size: u32) {
    unsafe { core::ptr::copy_nonoverlapping(src, dest, size as usize) };
}

unsafe fn server_run(server: *mut u8) -> u32 {
    let index = unsafe { field(server, S_CMDIDX) };
    let cmd = unsafe { ptr_field(server, S_SCRIPT).add(index as usize * CMD_SIZE) };
    unsafe { set_field(server, S_CMDIDX, index.wrapping_add(1)) };

    let instr = unsafe { cmd.cast::<u32>().read() };
    let parameter = unsafe { cmd.add(4).cast::<u32>().read() };
    let ptr = unsafe { cmd.add(8).cast::<*const u8>().read() };

    let recv = unsafe { ptr_field(server, S_RECV_BUFFER) };
    let card = unsafe { ptr_field(server, S_CARD) };
    let news = unsafe { ptr_field(server, S_NEWS) };
    let game_data = unsafe { ptr_field(server, S_LINK_GAME_DATA) };
    let stamp = unsafe { server.add(S_STAMP) };

    match instr {
        SVR_RETURN => {
            unsafe { set_field(server, S_FUNC_ID, FUNC_DONE) };
            // Becomes the endVal handed back by MysterGiftServer_Run.
            unsafe { set_field(server, S_PARAM, parameter) };
        }
        SVR_SEND => unsafe { set_field(server, S_FUNC_ID, FUNC_SEND) },
        SVR_RECV => {
            unsafe { MysteryGiftLink_InitRecv(link(server), parameter, recv) };
            unsafe { set_field(server, S_FUNC_ID, FUNC_RECV) };
        }
        SVR_GOTO => {
            unsafe { set_field(server, S_CMDIDX, 0) };
            unsafe { set_ptr_field(server, S_SCRIPT, ptr) };
        }
        SVR_COPY_GAME_DATA => unsafe { copy(recv, game_data, LINK_GAME_DATA_SIZE) },
        SVR_CHECK_GAME_DATA_CARD => {
            let valid = unsafe { MysteryGift_ValidateLinkGameData(game_data, 0) };
            unsafe { set_field(server, S_PARAM, valid) };
        }
        SVR_CHECK_GAME_DATA_NEWS => {
            let valid = unsafe { MysteryGift_ValidateLinkGameData(game_data, 1) };
            unsafe { set_field(server, S_PARAM, valid) };
        }
        SVR_GOTO_IF_EQ => {
            if unsafe { field(server, S_PARAM) } == parameter {
                unsafe { set_field(server, S_CMDIDX, 0) };
                unsafe { set_ptr_field(server, S_SCRIPT, ptr) };
            }
        }
        SVR_CHECK_EXISTING_CARD => {
            let data = send_data(ptr, card);
            let result = unsafe { MysteryGift_CompareCardFlags(data.cast(), game_data, data) };
            unsafe { set_field(server, S_PARAM, result) };
        }
        SVR_READ_RESPONSE => {
            let response = unsafe { recv.cast::<u32>().read() };
            unsafe { set_field(server, S_PARAM, response) };
        }
        SVR_CHECK_EXISTING_STAMPS => {
            let data = send_data(ptr, stamp);
            let result = unsafe { MysteryGift_CheckStamps(data.cast(), game_data, data) };
            unsafe { set_field(server, S_PARAM, result) };
        }
        SVR_GET_CARD_STAT => {
            let stat = unsafe { MysteryGift_GetCardStatFromLinkData(game_data, parameter) };
            unsafe { set_field(server, S_PARAM, u32::from(stat)) };
        }
        SVR_CHECK_QUESTIONNAIRE => {
            let matches = unsafe { MysteryGift_DoesQuestionnaireMatch(game_data, ptr.cast()) };
            unsafe { set_field(server, S_PARAM, matches) };
        }
        SVR_COMPARE => {
            let received = unsafe { recv.cast::<*const u8>().read() };
            unsafe { set_field(server, S_PARAM, compare(ptr, received)) };
        }
        SVR_LOAD_NEWS => unsafe {
            init_send(
                server,
                MG_LINKID_NEWS,
                send_data(ptr, news),
                WONDER_NEWS_SIZE,
            )
        },
        SVR_LOAD_CARD => unsafe {
            init_send(
                server,
                MG_LINKID_CARD,
                send_data(ptr, card),
                WONDER_CARD_SIZE,
            )
        },
        SVR_LOAD_STAMP => unsafe { init_send(server, MG_LINKID_STAMP, send_data(ptr, stamp), 4) },
        SVR_LOAD_RAM_SCRIPT => {
            if ptr.is_null() {
                unsafe {
                    init_send(
                        server,
                        MG_LINKID_RAM_SCRIPT,
                        ptr_field(server, S_RAM_SCRIPT),
                        field(server, S_RAM_SCRIPT_SIZE),
                    )
                };
            } else {
                unsafe { init_send(server, MG_LINKID_RAM_SCRIPT, ptr, parameter) };
            }
        }
        SVR_LOAD_CLIENT_SCRIPT => {
            if ptr.is_null() {
                unsafe {
                    init_send(
                        server,
                        MG_LINKID_CLIENT_SCRIPT,
                        ptr_field(server, S_CLIENT_SCRIPT),
                        field(server, S_CLIENT_SCRIPT_SIZE),
                    )
                };
            } else {
                unsafe { init_send(server, MG_LINKID_CLIENT_SCRIPT, ptr, parameter) };
            }
        }
        SVR_LOAD_EREADER_TRAINER => unsafe {
            init_send(server, MG_LINKID_EREADER_TRAINER, ptr, EREADER_TRAINER_SIZE)
        },
        SVR_LOAD_MSG => unsafe { init_send(server, MG_LINKID_DYNAMIC_MSG, ptr, parameter) },
        SVR_LOAD_UNK_2 => unsafe { init_send(server, MG_LINKID_UNK_2, ptr, parameter) },
        SVR_COPY_CARD => unsafe { copy(ptr, card, WONDER_CARD_SIZE) },
        SVR_COPY_NEWS => unsafe { copy(ptr, news, WONDER_NEWS_SIZE) },
        SVR_COPY_STAMP => {
            let value = unsafe { ptr.cast::<u32>().read() };
            unsafe { set_field(server, S_STAMP, value) };
        }
        SVR_SET_RAM_SCRIPT => {
            unsafe { set_ptr_field(server, S_RAM_SCRIPT, ptr) };
            unsafe { set_field(server, S_RAM_SCRIPT_SIZE, parameter) };
        }
        SVR_SET_CLIENT_SCRIPT => {
            unsafe { set_ptr_field(server, S_CLIENT_SCRIPT, ptr) };
            unsafe { set_field(server, S_CLIENT_SCRIPT_SIZE, parameter) };
        }
        SVR_COPY_SAVED_CARD => {
            unsafe { copy(GetSavedWonderCard(), card, WONDER_CARD_SIZE) };
            unsafe { DisableWonderCardSending(card) };
        }
        SVR_COPY_SAVED_NEWS => unsafe { copy(GetSavedWonderNews(), news, WONDER_NEWS_SIZE) },
        SVR_COPY_SAVED_RAM_SCRIPT => unsafe {
            set_ptr_field(server, S_RAM_SCRIPT, GetSavedRamScriptIfValid())
        },
        SVR_LOAD_UNK_1 => unsafe { init_send(server, MG_LINKID_UNK_1, ptr, parameter) },
        _ => {}
    }

    SVR_RET_ACTIVE
}

unsafe fn call_func(server: *mut u8) -> u32 {
    match unsafe { field(server, S_FUNC_ID) } {
        FUNC_INIT => {
            unsafe { set_field(server, S_FUNC_ID, FUNC_RUN) };
            SVR_RET_INIT
        }
        FUNC_DONE => SVR_RET_END,
        FUNC_RECV => {
            if unsafe { MysteryGiftLink_Recv(link(server)) } != 0 {
                unsafe { set_field(server, S_FUNC_ID, FUNC_RUN) };
            }
            SVR_RET_ACTIVE
        }
        FUNC_SEND => {
            if unsafe { MysteryGiftLink_Send(link(server)) } != 0 {
                unsafe { set_field(server, S_FUNC_ID, FUNC_RUN) };
            }
            SVR_RET_ACTIVE
        }
        FUNC_RUN => unsafe { server_run(server) },
        _ => SVR_RET_ACTIVE,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn server_layout_matches_the_arm_structure() {
        assert_eq!(S_LINK + 40, S_SIZE as usize);
        assert_eq!(CMD_SIZE, 12);
    }

    #[test]
    fn pointer_comparison_follows_the_original_ordering() {
        let data = [0u8; 4];
        let a = data.as_ptr();
        let b = unsafe { a.add(1) };
        assert_eq!(compare(b, a), 0);
        assert_eq!(compare(a, a), 1);
        assert_eq!(compare(a, b), 2);
    }

    #[test]
    fn a_null_command_pointer_falls_back_to_the_default() {
        let fallback = 1u8;
        let override_ = 2u8;
        assert_eq!(
            send_data(core::ptr::null(), &raw const fallback),
            &raw const fallback
        );
        assert_eq!(
            send_data(&raw const override_, &raw const fallback),
            &raw const override_
        );
    }
}
