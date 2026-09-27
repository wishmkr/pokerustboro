//! The Mystery Gift client. It runs a tiny script of commands, most of which
//! the server sends over the link, and reports back to the UI through its
//! return value.

use crate::ffi::{AllocZeroed, CpuSet, Free};
use crate::mystery_event_script::{
    InitMysteryEventScriptContext, RunMysteryEventScriptContextCommand,
};
use crate::mystery_gift_link::{
    MysteryGiftLink_Init, MysteryGiftLink_InitRecv, MysteryGiftLink_InitSend, MysteryGiftLink_Recv,
    MysteryGiftLink_Send,
};

const MG_LINK_BUFFER_SIZE: u32 = 0x400;
const CLIENT_MAX_MSG_SIZE: u32 = 0x40;

// Client function states.
const FUNC_INIT: u32 = 0;
const FUNC_DONE: u32 = 1;
const FUNC_RECV: u32 = 2;
const FUNC_SEND: u32 = 3;
const FUNC_RUN: u32 = 4;
const FUNC_WAIT: u32 = 5;
const FUNC_RUN_MEVENT: u32 = 6;
const FUNC_RUN_BUFFER: u32 = 7;

// Return values seen by the UI.
const CLI_RET_INIT: u32 = 0;
const CLI_RET_ACTIVE: u32 = 1;
const CLI_RET_YES_NO: u32 = 2;
const CLI_RET_PRINT_MSG: u32 = 3;
const CLI_RET_ASK_TOSS: u32 = 4;
const CLI_RET_COPY_MSG: u32 = 5;
const CLI_RET_END: u32 = 6;

// Script instructions.
const CLI_NONE: u32 = 0x00;
const CLI_RETURN: u32 = 0x01;
const CLI_RECV: u32 = 0x02;
const CLI_SEND_LOADED: u32 = 0x03;
const CLI_COPY_RECV: u32 = 0x04;
const CLI_YES_NO: u32 = 0x05;
const CLI_COPY_RECV_IF_N: u32 = 0x06;
const CLI_COPY_RECV_IF: u32 = 0x07;
const CLI_LOAD_GAME_DATA: u32 = 0x08;
const CLI_SAVE_NEWS: u32 = 0x09;
const CLI_SAVE_CARD: u32 = 0x0a;
const CLI_PRINT_MSG: u32 = 0x0b;
const CLI_COPY_MSG: u32 = 0x0c;
const CLI_ASK_TOSS: u32 = 0x0d;
const CLI_LOAD_TOSS_RESPONSE: u32 = 0x0e;
const CLI_RUN_MEVENT_SCRIPT: u32 = 0x0f;
const CLI_SAVE_STAMP: u32 = 0x10;
const CLI_SAVE_RAM_SCRIPT: u32 = 0x11;
const CLI_RECV_EREADER_TRAINER: u32 = 0x12;
const CLI_SEND_STAT: u32 = 0x13;
const CLI_SEND_READY_END: u32 = 0x14;
const CLI_RUN_BUFFER_SCRIPT: u32 = 0x15;

const MG_LINKID_GAME_DATA: u32 = 0x11;
const MG_LINKID_GAME_STAT: u32 = 0x12;
const MG_LINKID_RESPONSE: u32 = 0x13;
const MG_LINKID_READY_END: u32 = 0x14;

const LINK_GAME_DATA_SIZE: u32 = 100;
const RAM_SCRIPT_DATA_SIZE: u16 = 1000;
/// `offsetof(struct SaveBlock2, frontier.ereaderTrainer)` and its size.
const SAVE2_EREADER_TRAINER: usize = 0xbec;
const EREADER_TRAINER_SIZE: usize = 0xbc;

/// `struct MysteryGiftClient`, 80 bytes.
const C_SIZE: u32 = 0x50;
const C_UNUSED: usize = 0x00;
const C_PARAM: usize = 0x04;
const C_FUNC_ID: usize = 0x08;
const C_FUNC_STATE: usize = 0x0c;
const C_CMDIDX: usize = 0x10;
const C_SEND_BUFFER: usize = 0x14;
const C_RECV_BUFFER: usize = 0x18;
const C_SCRIPT: usize = 0x1c;
const C_MSG: usize = 0x20;
const C_LINK: usize = 0x24;
const C_IS_WONDER_NEWS: usize = 0x4c;
/// `struct MysteryGiftClientCmd { u32 instr; u32 parameter; }`
const CMD_SIZE: usize = 8;

#[unsafe(link_section = "ewram_data")]
static mut CLIENT: *mut u8 = core::ptr::null_mut();

unsafe extern "C" {
    static gMysteryGiftClientScript_Init: u8;
    static mut gDecompressionBuffer: u8;
    static mut gSaveBlock1Ptr: *mut u8;
    static mut gSaveBlock2Ptr: *mut u8;

    fn GetGameStat(index: u8) -> u32;
    fn MysteryGift_LoadLinkGameData(data: *mut u8, is_wonder_news: u32);
    fn SaveWonderCard(card: *const u8) -> u32;
    fn SaveWonderNews(news: *const u8) -> u32;
    fn IsWonderNewsSameAsSaved(news: *const u8) -> u32;
    fn MysteryGift_TrySaveStamp(stamp: *const u16) -> u32;
    fn InitRamScript_NoObjectEvent(script: *mut u8, size: u16);
    fn ValidateEReaderTrainer();
}

#[inline]
unsafe fn field(client: *mut u8, offset: usize) -> u32 {
    unsafe { client.add(offset).cast::<u32>().read_volatile() }
}

#[inline]
unsafe fn set_field(client: *mut u8, offset: usize, value: u32) {
    unsafe { client.add(offset).cast::<u32>().write_volatile(value) };
}

#[inline]
unsafe fn buffer(client: *mut u8, offset: usize) -> *mut u8 {
    unsafe { client.add(offset).cast::<*mut u8>().read() }
}

#[inline]
unsafe fn link(client: *mut u8) -> *mut u8 {
    unsafe { client.add(C_LINK) }
}

#[inline]
unsafe fn go_to(client: *mut u8, func_id: u32) {
    unsafe { set_field(client, C_FUNC_ID, func_id) };
    unsafe { set_field(client, C_FUNC_STATE, 0) };
}

#[inline]
unsafe fn copy(src: *const u8, dest: *mut u8, size: u32) {
    unsafe { core::ptr::copy_nonoverlapping(src, dest, size as usize) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn MysteryGiftClient_Create(is_wonder_news: u32) {
    let client = unsafe { AllocZeroed(C_SIZE) };
    unsafe { (&raw mut CLIENT).write(client) };
    unsafe { client_init(client, 1, 0) };
    unsafe { set_field(client, C_IS_WONDER_NEWS, is_wonder_news) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn MysteryGiftClient_Run(end_val: *mut u16) -> u32 {
    let client = unsafe { (&raw const CLIENT).read() };
    if client.is_null() {
        return CLI_RET_END;
    }

    let result = unsafe { call_func(client) };
    if result == CLI_RET_END {
        unsafe { end_val.write(field(client, C_PARAM) as u16) };
        unsafe { client_free(client) };
        unsafe { Free(client) };
        unsafe { (&raw mut CLIENT).write(core::ptr::null_mut()) };
    }
    result
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn MysteryGiftClient_AdvanceState() {
    let client = unsafe { (&raw const CLIENT).read() };
    let state = unsafe { field(client, C_FUNC_STATE) };
    unsafe { set_field(client, C_FUNC_STATE, state.wrapping_add(1)) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn MysteryGiftClient_GetMsg() -> *mut u8 {
    unsafe { buffer((&raw const CLIENT).read(), C_MSG) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn MysteryGiftClient_SetParam(value: u32) {
    unsafe { set_field((&raw const CLIENT).read(), C_PARAM, value) };
}

unsafe fn client_init(client: *mut u8, send_player_id: u32, recv_player_id: u32) {
    unsafe { set_field(client, C_UNUSED, 0) };
    unsafe { go_to(client, FUNC_INIT) };
    for (offset, size) in [
        (C_SEND_BUFFER, MG_LINK_BUFFER_SIZE),
        (C_RECV_BUFFER, MG_LINK_BUFFER_SIZE),
        (C_SCRIPT, MG_LINK_BUFFER_SIZE),
        (C_MSG, CLIENT_MAX_MSG_SIZE),
    ] {
        let allocation = unsafe { AllocZeroed(size) };
        unsafe { client.add(offset).cast::<*mut u8>().write(allocation) };
    }
    unsafe { MysteryGiftLink_Init(link(client), send_player_id, recv_player_id) };
}

unsafe fn client_free(client: *mut u8) {
    for offset in [C_SEND_BUFFER, C_RECV_BUFFER, C_SCRIPT, C_MSG] {
        unsafe { Free(buffer(client, offset)) };
    }
}

unsafe fn copy_recv_script(client: *mut u8) {
    unsafe {
        copy(
            buffer(client, C_RECV_BUFFER),
            buffer(client, C_SCRIPT),
            MG_LINK_BUFFER_SIZE,
        )
    };
    unsafe { set_field(client, C_CMDIDX, 0) };
}

unsafe fn init_send_word(client: *mut u8, ident: u32, word: u32) {
    let send = unsafe { buffer(client, C_SEND_BUFFER) };
    // CpuFill32(0, sendBuffer, MG_LINK_BUFFER_SIZE)
    let zero = 0u32;
    unsafe {
        CpuSet(
            (&raw const zero).cast(),
            send.cast(),
            0x0500_0000 | (MG_LINK_BUFFER_SIZE / 4),
        )
    };
    unsafe { send.cast::<u32>().write(word) };
    unsafe { MysteryGiftLink_InitSend(link(client), ident, send, 4) };
}

unsafe fn client_run(client: *mut u8) -> u32 {
    let index = unsafe { field(client, C_CMDIDX) };
    let cmd = unsafe { buffer(client, C_SCRIPT).add(index as usize * CMD_SIZE) };
    unsafe { set_field(client, C_CMDIDX, index.wrapping_add(1)) };
    let instr = unsafe { cmd.cast::<u32>().read() };
    let parameter = unsafe { cmd.add(4).cast::<u32>().read() };

    let recv = unsafe { buffer(client, C_RECV_BUFFER) };
    let send = unsafe { buffer(client, C_SEND_BUFFER) };
    let param = unsafe { field(client, C_PARAM) };

    match instr {
        CLI_NONE => {}
        CLI_RETURN => {
            // Becomes the endVal handed back by MysteryGiftClient_Run.
            unsafe { set_field(client, C_PARAM, parameter) };
            unsafe { go_to(client, FUNC_DONE) };
        }
        CLI_RECV => {
            unsafe { MysteryGiftLink_InitRecv(link(client), parameter, recv) };
            unsafe { go_to(client, FUNC_RECV) };
        }
        // Sends whatever has already been loaded.
        CLI_SEND_LOADED => unsafe { go_to(client, FUNC_SEND) },
        CLI_SEND_READY_END => {
            unsafe { MysteryGiftLink_InitSend(link(client), MG_LINKID_READY_END, send, 0) };
            unsafe { go_to(client, FUNC_SEND) };
        }
        CLI_SEND_STAT => {
            let stat = unsafe { GetGameStat(parameter as u8) };
            unsafe { init_send_word(client, MG_LINKID_GAME_STAT, stat) };
            unsafe { go_to(client, FUNC_SEND) };
        }
        CLI_COPY_RECV_IF_N => {
            if param == 0 {
                unsafe { copy_recv_script(client) };
            }
        }
        CLI_COPY_RECV_IF => {
            if param == 1 {
                unsafe { copy_recv_script(client) };
            }
        }
        CLI_COPY_RECV => unsafe { copy_recv_script(client) },
        CLI_YES_NO | CLI_PRINT_MSG | CLI_COPY_MSG => {
            unsafe { copy(recv, buffer(client, C_MSG), CLIENT_MAX_MSG_SIZE) };
            unsafe { go_to(client, FUNC_WAIT) };
            return match instr {
                CLI_YES_NO => CLI_RET_YES_NO,
                CLI_PRINT_MSG => CLI_RET_PRINT_MSG,
                _ => CLI_RET_COPY_MSG,
            };
        }
        CLI_ASK_TOSS => {
            unsafe { go_to(client, FUNC_WAIT) };
            return CLI_RET_ASK_TOSS;
        }
        CLI_LOAD_GAME_DATA => {
            unsafe { MysteryGift_LoadLinkGameData(send, field(client, C_IS_WONDER_NEWS)) };
            unsafe {
                MysteryGiftLink_InitSend(
                    link(client),
                    MG_LINKID_GAME_DATA,
                    send,
                    LINK_GAME_DATA_SIZE,
                )
            };
        }
        // param was set by the toss prompt.
        CLI_LOAD_TOSS_RESPONSE => unsafe { init_send_word(client, MG_LINKID_RESPONSE, param) },
        CLI_SAVE_CARD => {
            unsafe { SaveWonderCard(recv) };
        }
        CLI_SAVE_NEWS => {
            // Answers FALSE when saved, TRUE when it was a duplicate or invalid.
            if unsafe { IsWonderNewsSameAsSaved(recv) } == 0 {
                unsafe { SaveWonderNews(recv) };
                unsafe { init_send_word(client, MG_LINKID_RESPONSE, 0) };
            } else {
                unsafe { init_send_word(client, MG_LINKID_RESPONSE, 1) };
            }
        }
        CLI_RUN_MEVENT_SCRIPT => unsafe { go_to(client, FUNC_RUN_MEVENT) },
        CLI_SAVE_STAMP => {
            unsafe { MysteryGift_TrySaveStamp(recv.cast()) };
        }
        CLI_SAVE_RAM_SCRIPT => unsafe { InitRamScript_NoObjectEvent(recv, RAM_SCRIPT_DATA_SIZE) },
        CLI_RECV_EREADER_TRAINER => {
            let save2 = unsafe { (&raw const gSaveBlock2Ptr).read() };
            unsafe {
                copy(
                    recv,
                    save2.add(SAVE2_EREADER_TRAINER),
                    EREADER_TRAINER_SIZE as u32,
                )
            };
            unsafe { ValidateEReaderTrainer() };
        }
        CLI_RUN_BUFFER_SCRIPT => {
            unsafe {
                copy(
                    recv,
                    (&raw mut gDecompressionBuffer).cast(),
                    MG_LINK_BUFFER_SIZE,
                )
            };
            unsafe { go_to(client, FUNC_RUN_BUFFER) };
        }
        _ => {}
    }

    CLI_RET_ACTIVE
}

unsafe fn call_func(client: *mut u8) -> u32 {
    match unsafe { field(client, C_FUNC_ID) } {
        FUNC_INIT => {
            unsafe {
                copy(
                    &raw const gMysteryGiftClientScript_Init,
                    buffer(client, C_SCRIPT),
                    MG_LINK_BUFFER_SIZE,
                )
            };
            unsafe { set_field(client, C_CMDIDX, 0) };
            unsafe { go_to(client, FUNC_RUN) };
            CLI_RET_INIT
        }
        FUNC_DONE => CLI_RET_END,
        FUNC_RECV => {
            if unsafe { MysteryGiftLink_Recv(link(client)) } != 0 {
                unsafe { go_to(client, FUNC_RUN) };
            }
            CLI_RET_ACTIVE
        }
        FUNC_SEND => {
            if unsafe { MysteryGiftLink_Send(link(client)) } != 0 {
                unsafe { go_to(client, FUNC_RUN) };
            }
            CLI_RET_ACTIVE
        }
        FUNC_RUN => unsafe { client_run(client) },
        FUNC_WAIT => {
            // The UI advances funcState once the player has responded.
            if unsafe { field(client, C_FUNC_STATE) } != 0 {
                unsafe { go_to(client, FUNC_RUN) };
            }
            CLI_RET_ACTIVE
        }
        FUNC_RUN_MEVENT => {
            match unsafe { field(client, C_FUNC_STATE) } {
                0 => {
                    unsafe { InitMysteryEventScriptContext(buffer(client, C_RECV_BUFFER)) };
                    unsafe { set_field(client, C_FUNC_STATE, 1) };
                }
                1 => {
                    let status = unsafe { client.add(C_PARAM).cast::<u32>() };
                    if unsafe { RunMysteryEventScriptContextCommand(status) } == 0 {
                        unsafe { go_to(client, FUNC_RUN) };
                    }
                }
                _ => {}
            }
            CLI_RET_ACTIVE
        }
        FUNC_RUN_BUFFER => {
            // Executes code the server sent. This is how the original works:
            // the received buffer is jumped into as a Thumb function.
            type BufferScript = unsafe extern "C" fn(*mut u32, *mut u8, *mut u8) -> u32;
            let entry = (&raw mut gDecompressionBuffer) as usize;
            let func: BufferScript = unsafe { core::mem::transmute::<usize, BufferScript>(entry) };
            let status = unsafe { client.add(C_PARAM).cast::<u32>() };
            let save1 = unsafe { (&raw const gSaveBlock1Ptr).read() };
            let save2 = unsafe { (&raw const gSaveBlock2Ptr).read() };
            if unsafe { func(status, save2, save1) } == 1 {
                unsafe { go_to(client, FUNC_RUN) };
            }
            CLI_RET_ACTIVE
        }
        _ => CLI_RET_ACTIVE,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn client_layout_matches_the_arm_structure() {
        // The link is 40 bytes, so isWonderNews follows it directly.
        assert_eq!(C_LINK + 40, C_IS_WONDER_NEWS);
        assert_eq!(C_IS_WONDER_NEWS + 4, C_SIZE as usize);
    }

    #[test]
    fn message_returns_are_distinct() {
        let all = [
            CLI_RET_INIT,
            CLI_RET_ACTIVE,
            CLI_RET_YES_NO,
            CLI_RET_PRINT_MSG,
            CLI_RET_ASK_TOSS,
            CLI_RET_COPY_MSG,
            CLI_RET_END,
        ];
        for i in 0..all.len() {
            assert_eq!(all[i], i as u32);
        }
    }
}
