//! The game's entry point and frame loop (`AgbMain`, called from crt0.s),
//! interrupt handlers, key reading and soft reset. The interrupt dispatcher
//! itself (`IntrMain`, crt0.rs) is copied to IWRAM here.

use crate::bg::ResetBgs;
use crate::dma3_manager::{ClearDma3Requests, ProcessDma3Requests};
use crate::ffi::{Align4, MainCallback};
use crate::gpu_regs::{
    CopyBufferedValuesToGpuRegs, EnableInterrupts, GetGpuReg, InitGpuRegManager, SetGpuReg,
};
use crate::load_save::{
    CheckForFlashMemory, gFlashMemoryPresent, gPokemonStorage, gPokemonStoragePtr, gSaveBlock2Ptr,
    gSaveblock2,
};
use crate::malloc::{InitHeap, gHeap};
use crate::play_time::PlayTimeCounter_Update;
use crate::rtc::RtcInit;
use crate::scanline_effect::ScanlineEffect_Stop;
use crate::sprite::ClearSpriteCopyRequests;
use crate::text::SetDefaultFontsPointer;
use crate::{Random, SeedRng};

type IntrFunc = unsafe extern "C" fn();

const MAIN_SIZE: usize = 0x43c;
const M_CALLBACK1: usize = 0x000;
const M_CALLBACK2: usize = 0x004;
const M_VBLANK_CALLBACK: usize = 0x00c;
const M_HBLANK_CALLBACK: usize = 0x010;
const M_VCOUNT_CALLBACK: usize = 0x014;
const M_SERIAL_CALLBACK: usize = 0x018;
const M_INTR_CHECK: usize = 0x01c;
const M_VBLANK_COUNTER1: usize = 0x020;
const M_VBLANK_COUNTER2: usize = 0x024;
const M_HELD_KEYS_RAW: usize = 0x028;
const M_NEW_KEYS_RAW: usize = 0x02a;
const M_HELD_KEYS: usize = 0x02c;
const M_NEW_KEYS: usize = 0x02e;
const M_NEW_AND_REPEATED_KEYS: usize = 0x030;
const M_KEY_REPEAT_COUNTER: usize = 0x032;
const M_WATCHED_KEYS_PRESSED: usize = 0x034;
const M_WATCHED_KEYS_MASK: usize = 0x036;
const M_STATE: usize = 0x438;
/// `oamLoadDisabled:1`, `inBattle:1`, `anyLinkBattlerHasFrontierPass:1`.
const M_FLAGS: usize = 0x439;
const M_IN_BATTLE: u8 = 0x02;

const INTR_FLAG_VBLANK: u16 = 1 << 0;
const INTR_FLAG_HBLANK: u16 = 1 << 1;
const INTR_FLAG_VCOUNT: u16 = 1 << 2;
const INTR_FLAG_SERIAL: u16 = 1 << 7;
const DISPSTAT_VCOUNT_INTR: u16 = 0x20;
const REG_OFFSET_DISPSTAT: u8 = 0x04;

const A_BUTTON: u16 = 1 << 0;
const B_BUTTON: u16 = 1 << 1;
const SELECT_BUTTON: u16 = 1 << 2;
const START_BUTTON: u16 = 1 << 3;
const L_BUTTON: u16 = 1 << 9;
const B_START_SELECT: u16 = B_BUTTON | START_BUTTON | SELECT_BUTTON;
const KEYS_MASK: u16 = 0x03ff;

const REG_KEYINPUT: *const u16 = 0x0400_0130 as *const u16;
const REG_WAITCNT: *mut u16 = 0x0400_0204 as *mut u16;
const REG_IME: *mut u16 = 0x0400_0208 as *mut u16;
const REG_TM1CNT_L: *const u16 = 0x0400_0104 as *const u16;
const REG_TM1CNT_H: *mut u16 = 0x0400_0106 as *mut u16;
const REG_DMA_CNT_H: [usize; 3] = [0x0400_00c6, 0x0400_00d2, 0x0400_00de];
const INTR_VECTOR: *mut *mut u32 = 0x0300_7ffc as *mut *mut u32;
/// `INTR_CHECK`, the BIOS interrupt-wait flags.
const INTR_CHECK: *mut u16 = 0x0300_7ff8 as *mut u16;
const BG_PLTT: *mut u16 = 0x0500_0000 as *mut u16;
const WAITCNT_SETTING: u16 = 0x4014;
const RGB_WHITE: u16 = 0x7fff;
const HEAP_SIZE: u32 = 0x1c000;
const OPTIONS_BUTTON_MODE_L_EQUALS_A: u8 = 2;
const SB2_OPTIONS_BUTTON_MODE: usize = 0x13;
const BATTLE_TYPE_LINK_FRONTIER_RECORDED: u32 = 0x0101_3f02;
const MAX_POKEMON_CRIES: usize = 2;
const POKEMON_CRY_SONG_SIZE: usize = 0x34;
const SOUND_INFO_PCM_DMA_COUNTER: usize = 4;
const RESET_ALL: u32 = 0xff;
const INTR_COUNT: usize = 14;

#[unsafe(no_mangle)]
pub static gGameVersion: u8 = 3; // VERSION_EMERALD
#[unsafe(no_mangle)]
pub static gGameLanguage: u8 = 2; // LANGUAGE_ENGLISH
#[unsafe(no_mangle)]
pub static BuildDateTime: crate::ffi::RomBytes<17> = crate::ffi::RomBytes(*b"2005 02 21 11:10\0");

#[unsafe(no_mangle)]
pub static gIntrTableTemplate: [IntrFunc; INTR_COUNT] = [
    vcount_intr,
    serial_intr,
    Timer3Intr,
    hblank_intr,
    vblank_intr,
    intr_dummy,
    intr_dummy,
    intr_dummy,
    intr_dummy,
    intr_dummy,
    intr_dummy,
    intr_dummy,
    intr_dummy,
    intr_dummy,
];

#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gKeyRepeatStartDelay: u16 = 0;
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gLinkTransferringData: u8 = 0;
/// `struct Main`, 0x43c bytes. Exported under its C name; Rust code reaches it
/// through the byte-typed declaration in `ffi`.
#[unsafe(export_name = "gMain")]
#[unsafe(link_section = "common_data")]
pub static mut MAIN: Align4<[u8; MAIN_SIZE]> = Align4([0; MAIN_SIZE]);
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gKeyRepeatContinueDelay: u16 = 0;
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gSoftResetDisabled: u8 = 0;
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gIntrTable: Align4<[usize; INTR_COUNT]> = Align4([0; INTR_COUNT]);
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gLinkVSyncDisabled: u8 = 0;
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut IntrMain_Buffer: Align4<[u32; 0x200]> = Align4([0; 0x200]);
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gPcmDmaCounter: i8 = 0;

static mut UNUSED_VAR: u16 = 0;

#[unsafe(link_section = "ewram_data")]
static mut TRAINER_ID: u16 = 0;

unsafe extern "C" {
    static gWirelessCommType: u8;
    static mut gTrainerHillVBlankCounter: *mut u32;
    static mut gSoundInfo: u8;
    static gBattleTypeFlags: u32;
    static mut gPokemonCrySongs: u8;

    fn m4aSoundInit();
    fn m4aSoundMain();
    fn m4aSoundVSync();
    fn m4aSoundVSyncOff();
    fn InitRFU();
    fn InitMapMusic();
    fn MapMusicMain();
    fn rfu_REQ_stopMode();
    fn rfu_waitREQComplete();
    fn Overworld_SendKeysToLinkIsRunning() -> u8;
    fn Overworld_RecvKeysFromLinkIsRunning() -> u8;
    fn HandleLinkConnection() -> u8;
    fn CB2_InitCopyrightScreenAfterBootup();
    fn Timer3Intr();
    fn RfuVSync();
    fn LinkVSync();
    fn TryReceiveLinkBattleData();
    fn UpdateWirelessStatusIndicatorSprite();
    fn SetFlashTimerIntr(timer_num: u8, intr_func: *mut usize) -> u8;
    fn SiiRtcProtect();
    fn SoftReset(reset_flags: u32);
}

#[inline]
fn main_field(offset: usize) -> *mut u8 {
    (&raw mut MAIN).cast::<u8>().wrapping_add(offset)
}

#[inline]
unsafe fn u16_field(offset: usize) -> *mut u16 {
    main_field(offset).cast()
}

#[inline]
unsafe fn callback(offset: usize) -> Option<unsafe extern "C" fn()> {
    unsafe {
        main_field(offset)
            .cast::<Option<unsafe extern "C" fn()>>()
            .read_volatile()
    }
}

#[inline]
unsafe fn set_callback(offset: usize, callback: Option<unsafe extern "C" fn()>) {
    unsafe {
        main_field(offset)
            .cast::<Option<unsafe extern "C" fn()>>()
            .write_volatile(callback)
    };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn AgbMain() -> ! {
    // RegisterRamReset already ran in crt0 for the modern build.
    unsafe { BG_PLTT.write_volatile(RGB_WHITE) };
    unsafe { InitGpuRegManager() };
    unsafe { REG_WAITCNT.write_volatile(WAITCNT_SETTING) };
    unsafe { InitKeys() };
    unsafe { InitIntrHandlers() };
    unsafe { m4aSoundInit() };
    unsafe { EnableVCountIntrAtLine150() };
    unsafe { InitRFU() };
    unsafe { RtcInit() };
    unsafe { CheckForFlashMemory() };
    unsafe { init_main_callbacks() };
    unsafe { InitMapMusic() };
    unsafe { ClearDma3Requests() };
    unsafe { ResetBgs() };
    unsafe { SetDefaultFontsPointer() };
    unsafe { InitHeap((&raw mut gHeap).cast(), HEAP_SIZE) };
    unsafe { (&raw mut gSoftResetDisabled).write(0) };
    if unsafe { (&raw const gFlashMemoryPresent).read() } != 1 {
        unsafe { SetMainCallback2(None) };
    }
    unsafe { (&raw mut gLinkTransferringData).write(0) };
    unsafe { (&raw mut UNUSED_VAR).write(0xfc0) };

    loop {
        unsafe { read_keys() };
        let held_raw = unsafe { u16_field(M_HELD_KEYS_RAW).read() };
        if unsafe { (&raw const gSoftResetDisabled).read() } == 0
            && held_raw & A_BUTTON != 0
            && held_raw & B_START_SELECT == B_START_SELECT
        {
            unsafe { rfu_REQ_stopMode() };
            unsafe { rfu_waitREQComplete() };
            unsafe { DoSoftReset() };
        }

        if unsafe { Overworld_SendKeysToLinkIsRunning() } == 1 {
            unsafe { (&raw mut gLinkTransferringData).write(1) };
            unsafe { update_link_and_call_callbacks() };
            unsafe { (&raw mut gLinkTransferringData).write(0) };
        } else {
            unsafe { (&raw mut gLinkTransferringData).write(0) };
            unsafe { update_link_and_call_callbacks() };
            if unsafe { Overworld_RecvKeysFromLinkIsRunning() } == 1 {
                unsafe { u16_field(M_NEW_KEYS).write(0) };
                unsafe { ClearSpriteCopyRequests() };
                unsafe { (&raw mut gLinkTransferringData).write(1) };
                unsafe { update_link_and_call_callbacks() };
                unsafe { (&raw mut gLinkTransferringData).write(0) };
            }
        }
        unsafe { PlayTimeCounter_Update() };
        unsafe { MapMusicMain() };
        unsafe { wait_for_vblank() };
    }
}

unsafe fn update_link_and_call_callbacks() {
    if unsafe { HandleLinkConnection() } == 0 {
        unsafe { call_callbacks() };
    }
}

unsafe fn init_main_callbacks() {
    unsafe { main_field(M_VBLANK_COUNTER1).cast::<u32>().write(0) };
    unsafe { (&raw mut gTrainerHillVBlankCounter).write(core::ptr::null_mut()) };
    unsafe { main_field(M_VBLANK_COUNTER2).cast::<u32>().write(0) };
    unsafe { set_callback(M_CALLBACK1, None) };
    unsafe { SetMainCallback2(Some(CB2_InitCopyrightScreenAfterBootup)) };
    unsafe { (&raw mut gSaveBlock2Ptr).write((&raw mut gSaveblock2).cast()) };
    unsafe { (&raw mut gPokemonStoragePtr).write((&raw mut gPokemonStorage).cast()) };
}

unsafe fn call_callbacks() {
    if let Some(cb) = unsafe { callback(M_CALLBACK1) } {
        unsafe { cb() };
    }
    if let Some(cb) = unsafe { callback(M_CALLBACK2) } {
        unsafe { cb() };
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetMainCallback2(callback: Option<MainCallback>) {
    unsafe { set_callback(M_CALLBACK2, callback) };
    unsafe { main_field(M_STATE).write(0) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn StartTimer1() {
    unsafe { REG_TM1CNT_H.write_volatile(0x80) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn SeedRngAndSetTrainerId() {
    let value = unsafe { REG_TM1CNT_L.read_volatile() };
    unsafe { SeedRng(value) };
    unsafe { REG_TM1CNT_H.write_volatile(0) };
    unsafe { (&raw mut TRAINER_ID).write(value) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetGeneratedTrainerIdLower() -> u16 {
    unsafe { (&raw const TRAINER_ID).read() }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn EnableVCountIntrAtLine150() {
    let value = (unsafe { GetGpuReg(REG_OFFSET_DISPSTAT) } & 0xff) | (150 << 8);
    unsafe { SetGpuReg(REG_OFFSET_DISPSTAT, value | DISPSTAT_VCOUNT_INTR) };
    unsafe { EnableInterrupts(INTR_FLAG_VCOUNT) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn InitKeys() {
    unsafe { (&raw mut gKeyRepeatContinueDelay).write(5) };
    unsafe { (&raw mut gKeyRepeatStartDelay).write(40) };
    for field in [
        M_HELD_KEYS,
        M_NEW_KEYS,
        M_NEW_AND_REPEATED_KEYS,
        M_HELD_KEYS_RAW,
        M_NEW_KEYS_RAW,
    ] {
        unsafe { u16_field(field).write(0) };
    }
}

unsafe fn read_keys() {
    let key_input = unsafe { REG_KEYINPUT.read_volatile() } ^ KEYS_MASK;
    let held_raw = unsafe { u16_field(M_HELD_KEYS_RAW).read() };
    let new_raw = key_input & !held_raw;
    unsafe { u16_field(M_NEW_KEYS_RAW).write(new_raw) };
    unsafe { u16_field(M_NEW_KEYS).write(new_raw) };
    unsafe { u16_field(M_NEW_AND_REPEATED_KEYS).write(new_raw) };

    // Key repeat compares the raw input with the *remapped* held keys, so it
    // doesn't work for L in L=A mode; kept as in the original.
    if key_input != 0 && unsafe { u16_field(M_HELD_KEYS).read() } == key_input {
        let counter = unsafe { u16_field(M_KEY_REPEAT_COUNTER).read() }.wrapping_sub(1);
        unsafe { u16_field(M_KEY_REPEAT_COUNTER).write(counter) };
        if counter == 0 {
            unsafe { u16_field(M_NEW_AND_REPEATED_KEYS).write(key_input) };
            unsafe {
                u16_field(M_KEY_REPEAT_COUNTER).write((&raw const gKeyRepeatContinueDelay).read())
            };
        }
    } else {
        unsafe { u16_field(M_KEY_REPEAT_COUNTER).write((&raw const gKeyRepeatStartDelay).read()) };
    }

    unsafe { u16_field(M_HELD_KEYS_RAW).write(key_input) };
    unsafe { u16_field(M_HELD_KEYS).write(key_input) };

    let sb2 = unsafe { (&raw const gSaveBlock2Ptr).read() };
    if unsafe { sb2.add(SB2_OPTIONS_BUTTON_MODE).read() } == OPTIONS_BUTTON_MODE_L_EQUALS_A {
        if unsafe { u16_field(M_NEW_KEYS).read() } & L_BUTTON != 0 {
            unsafe { u16_field(M_NEW_KEYS).write(u16_field(M_NEW_KEYS).read() | A_BUTTON) };
        }
        if unsafe { u16_field(M_HELD_KEYS).read() } & L_BUTTON != 0 {
            unsafe { u16_field(M_HELD_KEYS).write(u16_field(M_HELD_KEYS).read() | A_BUTTON) };
        }
    }

    let watched = unsafe { u16_field(M_WATCHED_KEYS_MASK).read() };
    if unsafe { u16_field(M_NEW_KEYS).read() } & watched != 0 {
        unsafe { u16_field(M_WATCHED_KEYS_PRESSED).write(1) };
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn InitIntrHandlers() {
    let table = (&raw mut gIntrTable).cast::<usize>();
    for (i, handler) in gIntrTableTemplate.iter().enumerate() {
        unsafe { table.add(i).write(*handler as usize) };
    }
    // DmaCopy32(3, IntrMain, IntrMain_Buffer, sizeof(IntrMain_Buffer))
    let buffer = (&raw mut IntrMain_Buffer).cast::<u32>();
    let dma3 = 0x0400_00d4 as *mut u32;
    unsafe { dma3.write_volatile(crate::crt0::IntrMain as *const () as usize as u32) };
    unsafe { dma3.add(1).write_volatile(buffer as usize as u32) };
    unsafe { dma3.add(2).write_volatile((0x8400 << 16) | 0x200) };
    unsafe { dma3.add(2).read_volatile() };
    unsafe { INTR_VECTOR.write_volatile(buffer) };
    unsafe { SetVBlankCallback(None) };
    unsafe { SetHBlankCallback(None) };
    unsafe { SetSerialCallback(None) };
    unsafe { REG_IME.write_volatile(1) };
    unsafe { EnableInterrupts(INTR_FLAG_VBLANK) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetVBlankCallback(callback: Option<unsafe extern "C" fn()>) {
    unsafe { set_callback(M_VBLANK_CALLBACK, callback) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetHBlankCallback(callback: Option<unsafe extern "C" fn()>) {
    unsafe { set_callback(M_HBLANK_CALLBACK, callback) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetVCountCallback(callback: Option<unsafe extern "C" fn()>) {
    unsafe { set_callback(M_VCOUNT_CALLBACK, callback) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn RestoreSerialTimer3IntrHandlers() {
    let table = (&raw mut gIntrTable).cast::<usize>();
    unsafe { table.add(1).write(serial_intr as IntrFunc as usize) };
    unsafe { table.add(2).write(Timer3Intr as IntrFunc as usize) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetSerialCallback(callback: Option<unsafe extern "C" fn()>) {
    unsafe { set_callback(M_SERIAL_CALLBACK, callback) };
}

#[inline]
unsafe fn mark_interrupt(flag: u16) {
    unsafe { INTR_CHECK.write_volatile(INTR_CHECK.read_volatile() | flag) };
    let check = unsafe { u16_field(M_INTR_CHECK) };
    unsafe { check.write_volatile(check.read_volatile() | flag) };
}

unsafe extern "C" fn vblank_intr() {
    if unsafe { (&raw const gWirelessCommType).read_volatile() } != 0 {
        unsafe { RfuVSync() };
    } else if unsafe { (&raw const gLinkVSyncDisabled).read_volatile() } == 0 {
        unsafe { LinkVSync() };
    }

    let counter1 = main_field(M_VBLANK_COUNTER1).cast::<u32>();
    unsafe { counter1.write_volatile(counter1.read_volatile().wrapping_add(1)) };
    let hill = unsafe { (&raw const gTrainerHillVBlankCounter).read_volatile() };
    if !hill.is_null() && unsafe { hill.read_volatile() } < 0xffff_ffff {
        unsafe { hill.write_volatile(hill.read_volatile() + 1) };
    }
    if let Some(cb) = unsafe { callback(M_VBLANK_CALLBACK) } {
        unsafe { cb() };
    }
    let counter2 = main_field(M_VBLANK_COUNTER2).cast::<u32>();
    unsafe { counter2.write_volatile(counter2.read_volatile().wrapping_add(1)) };

    unsafe { CopyBufferedValuesToGpuRegs() };
    unsafe { ProcessDma3Requests() };
    let pcm = unsafe {
        (&raw const gSoundInfo)
            .add(SOUND_INFO_PCM_DMA_COUNTER)
            .read_volatile()
    };
    unsafe { (&raw mut gPcmDmaCounter).write_volatile(pcm as i8) };
    unsafe { m4aSoundMain() };
    unsafe { TryReceiveLinkBattleData() };

    let in_battle = unsafe { main_field(M_FLAGS).read_volatile() } & M_IN_BATTLE != 0;
    let flags = unsafe { (&raw const gBattleTypeFlags).read_volatile() };
    if !in_battle || flags & BATTLE_TYPE_LINK_FRONTIER_RECORDED == 0 {
        unsafe { Random() };
    }
    unsafe { UpdateWirelessStatusIndicatorSprite() };
    unsafe { mark_interrupt(INTR_FLAG_VBLANK) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn InitFlashTimer() {
    let table = (&raw mut gIntrTable).cast::<usize>();
    unsafe { SetFlashTimerIntr(2, table.add(7)) };
}

unsafe extern "C" fn hblank_intr() {
    if let Some(cb) = unsafe { callback(M_HBLANK_CALLBACK) } {
        unsafe { cb() };
    }
    unsafe { mark_interrupt(INTR_FLAG_HBLANK) };
}

unsafe extern "C" fn vcount_intr() {
    if let Some(cb) = unsafe { callback(M_VCOUNT_CALLBACK) } {
        unsafe { cb() };
    }
    unsafe { m4aSoundVSync() };
    unsafe { mark_interrupt(INTR_FLAG_VCOUNT) };
}

unsafe extern "C" fn serial_intr() {
    if let Some(cb) = unsafe { callback(M_SERIAL_CALLBACK) } {
        unsafe { cb() };
    }
    unsafe { mark_interrupt(INTR_FLAG_SERIAL) };
}

unsafe extern "C" fn intr_dummy() {}

unsafe fn wait_for_vblank() {
    let check = unsafe { u16_field(M_INTR_CHECK) };
    unsafe { check.write_volatile(check.read_volatile() & !INTR_FLAG_VBLANK) };
    while unsafe { check.read_volatile() } & INTR_FLAG_VBLANK == 0 {}
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetTrainerHillVBlankCounter(counter: *mut u32) {
    unsafe { (&raw mut gTrainerHillVBlankCounter).write(counter) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ClearTrainerHillVBlankCounter() {
    unsafe { (&raw mut gTrainerHillVBlankCounter).write(core::ptr::null_mut()) };
}

/// `DmaStop(n)`
unsafe fn dma_stop(control: usize) {
    let control = control as *mut u16;
    unsafe { control.write_volatile(control.read_volatile() & !(0x3000 | 0x0800 | 0x0200)) };
    unsafe { control.write_volatile(control.read_volatile() & !0x8000) };
    unsafe { control.read_volatile() };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn DoSoftReset() -> ! {
    unsafe { REG_IME.write_volatile(0) };
    unsafe { m4aSoundVSyncOff() };
    unsafe { ScanlineEffect_Stop() };
    for control in REG_DMA_CNT_H {
        unsafe { dma_stop(control) };
    }
    unsafe { SiiRtcProtect() };
    unsafe { SoftReset(RESET_ALL) };
    #[allow(clippy::empty_loop)]
    loop {}
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ClearPokemonCrySongs() {
    let bytes = MAX_POKEMON_CRIES * POKEMON_CRY_SONG_SIZE;
    unsafe { (&raw mut gPokemonCrySongs).write_bytes(0, bytes) };
}
