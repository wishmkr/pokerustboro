//! Translated from `src/link.c` by tools/rustport/c2rs.py.
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
    clippy::if_same_then_else,
    clippy::manual_c_str_literals,
    clippy::missing_transmute_annotations,
    clippy::useless_transmute,
    dead_code,
    unused_assignments,
    unused_variables
)]

use crate::AgbRfu_LinkManager::rfu_LMAN_REQBN_softReset_and_checkID;
use crate::agb_main::gMain;
use crate::agb_main::{
    RestoreSerialTimer3IntrHandlers, SetVBlankCallback, gGameLanguage, gGameVersion,
    gLinkTransferringData, gLinkVSyncDisabled, gSoftResetDisabled,
};
use crate::battle_main::gBattleTypeFlags;
use crate::bg::{CopyBgTilemapBufferToVram, ResetBgsAndClearDma3BusyFlags, ShowBg};
use crate::bg::{CopyToBgTilemapBuffer, LoadBgTiles};
#[allow(unused_imports)]
use crate::c::*;
#[allow(unused_imports)]
use crate::consts::*;
use crate::event_data::{FlagGet, IsNationalPokedexEnabled};
use crate::ffi::gSpecialVar_0x8005;
use crate::gpu_regs::{ClearGpuRegBits, DisableInterrupts, EnableInterrupts, SetGpuReg};
use crate::item_menu::gSpecialVar_ItemId;
use crate::librfu_rfu::{rfu_REQ_stopMode, rfu_waitREQComplete};
use crate::link_rfu_2::{
    ClearLinkRfuCallback, GetRfuRecvQueueLength, InitRFUAPI, IsLinkRfuTaskFinished,
    IsRfuRecvQueueEmpty, IsSendingKeysToRfu, LinkRfu_Shutdown, ResetLinkRfuGFLayer,
    Rfu_GetBlockReceivedStatus, Rfu_GetLinkPlayerCount, Rfu_GetMultiplayerId, Rfu_InitBlockSend,
    Rfu_IsMaster, Rfu_ResetBlockReceivedFlag, Rfu_SendBlockRequest,
    Rfu_SetBerryBlenderLinkCallback, Rfu_SetBlockReceivedFlag, Rfu_SetCloseLinkCallback,
    Rfu_SetLinkStandbyCallback, RfuMain1, RfuMain2, StartSendingKeysToRfu,
};
use crate::load_save::gSaveBlock2Ptr;
use crate::m4a::{gMPlayInfo_SE1, gMPlayInfo_SE2, gMPlayInfo_SE3, m4aMPlayStop};
use crate::menu::{
    AddTextPrinterParameterized3, DecompressAndLoadBgGfxUsingHeap, ResetTempTileDataBuffers,
};
use crate::overworld::{IsSendingKeysOverCable, gHeldKeyCodeToSend};
use crate::palette::{
    BeginNormalPaletteFade, FillPalette, LoadPalette, ResetPaletteFadeControl, TransferPlttBuffer,
    UpdatePaletteFade,
};
use crate::random::{Random, SeedRng};
use crate::reload_save::ReloadSave;
use crate::save::TrySavingData;
use crate::scanline_effect::ScanlineEffect_Stop;
use crate::sound::{PlaySE, StopMapMusic};
use crate::sprite::{
    AnimateSprites, BuildOamBuffer, FreeAllSpritePalettes, LoadOam, ProcessSpriteCopyRequests,
    ResetSpriteData,
};
use crate::string_util::ConvertInternationalString;
use crate::string_util::{StringCompare, StringCopy};
use crate::task::{DestroyTask, ResetTasks, RunTasks};
use crate::task::{task_get, task_set};
use crate::text::DeactivateAllTextPrinters;
use crate::trade::GetGameProgressForLinkTrade;
#[allow(unused_imports)]
use crate::types::*;
use crate::window::{CopyWindowToVram, FillWindowPixelBuffer, PutWindowTilemap};
#[allow(unused_imports)]
use core::ffi::c_void;
#[allow(unused_imports)]
use core::mem::zeroed;
#[allow(unused_imports)]
use core::ptr::null_mut;
/// `CreateTask` with this module's view of its types.
#[inline]
unsafe fn CreateTask(a0: Option<unsafe fn(u8)>, a1: u8) -> u8 {
    unsafe { crate::task::CreateTask(core::mem::transmute(a0), a1) }
}
/// `InitBgsFromTemplates` with this module's view of its types.
#[inline]
fn InitBgsFromTemplates(a0: u8, a1: *mut BgTemplate, a2: u8) {
    unsafe {
        crate::bg::InitBgsFromTemplates(a0, a1 as _, a2);
    }
}
/// `InitHeap` with this module's view of its types.
#[inline]
fn InitHeap(a0: *mut c_void, a1: u32) {
    unsafe {
        crate::malloc::InitHeap(a0 as _, a1);
    }
}
/// `InitWindows` with this module's view of its types.
#[inline]
fn InitWindows(a0: *mut WindowTemplate) -> u16 {
    unsafe { crate::window::InitWindows(a0 as _) }
}
/// `SetBgTilemapBuffer` with this module's view of its types.
#[inline]
unsafe fn SetBgTilemapBuffer(a0: u8, a1: *mut c_void) {
    unsafe {
        crate::bg::SetBgTilemapBuffer(a0, a1 as _);
    }
}
// Data tables (translate with cdata.py): sWirelessLinkDisplayPal sWirelessLinkDisplayGfx sWirelessLinkDisplayTilemap sLinkTestDigitsPal sLinkTestDigitsGfx sUnusedTransparentWhite sCommErrorBg_Gfx sBlockRequests sBGControlRegs sASCIIGameFreakInc sASCIITestPrint sLinkErrorBgTemplates sLinkErrorWindowTemplates sTextColors sUnusedData

/// `struct BlockTransfer`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct BlockTransfer {
    pub pos: u16,
    pub size: u16,
    pub src: *mut u8,
    pub active: u8,
    pub multiplayerId: u8,
}

unsafe impl Sync for BlockTransfer {}

/// `struct LinkTestBGInfo`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct LinkTestBGInfo {
    pub screenBaseBlock: u32,
    pub paletteNum: u32,
    pub baseChar: u32,
    pub unused: u32,
}

unsafe impl Sync for LinkTestBGInfo {}

/// `__typeof__(sLinkErrorBuffer)`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct sLinkErrorBuffer_t {
    pub status: u32,
    pub lastRecvQueueCount: u8,
    pub lastSendQueueCount: u8,
    pub disconnected: u8,
}

unsafe impl Sync for sLinkErrorBuffer_t {}

#[cfg(target_arch = "arm")]
const _: () = {
    #[allow(unused_imports)]
    use core::mem::{offset_of, size_of};
    assert!(size_of::<BlockTransfer>() == 12);
    assert!(offset_of!(BlockTransfer, pos) == 0);
    assert!(offset_of!(BlockTransfer, size) == 2);
    assert!(offset_of!(BlockTransfer, src) == 4);
    assert!(offset_of!(BlockTransfer, active) == 8);
    assert!(offset_of!(BlockTransfer, multiplayerId) == 9);
    assert!(size_of::<LinkTestBGInfo>() == 16);
    assert!(offset_of!(LinkTestBGInfo, screenBaseBlock) == 0);
    assert!(offset_of!(LinkTestBGInfo, paletteNum) == 4);
    assert!(offset_of!(LinkTestBGInfo, baseChar) == 8);
    assert!(offset_of!(LinkTestBGInfo, unused) == 12);
    assert!(size_of::<sLinkErrorBuffer_t>() == 8);
    assert!(offset_of!(sLinkErrorBuffer_t, status) == 0);
    assert!(offset_of!(sLinkErrorBuffer_t, lastRecvQueueCount) == 4);
    assert!(offset_of!(sLinkErrorBuffer_t, lastSendQueueCount) == 5);
    assert!(offset_of!(sLinkErrorBuffer_t, disconnected) == 6);
};

const WIN_LINK_ERROR_BOTTOM: u8 = 2;
const WIN_LINK_ERROR_MID: u8 = 1;
const WIN_LINK_ERROR_TOP: u8 = 0;

static sASCIIGameFreakInc: Table<CArray<u8, 15>> =
    Table((&raw const crate::data::link::sASCIIGameFreakInc).cast());
static sASCIITestPrint: Table<CArray<u8, 23>> =
    Table((&raw const crate::data::link::sASCIITestPrint).cast());
static sBGControlRegs: Table<CArray<u8, 4>> =
    Table((&raw const crate::data::link::sBGControlRegs).cast());
static sBlockRequests: Table<CArray<BlockRequest, 5>> =
    Table((&raw const crate::data::link::sBlockRequests).cast());
static sCommErrorBg_Gfx: Table<CArray<u16, 32>> =
    Table((&raw const crate::data::link::sCommErrorBg_Gfx).cast());
static sLinkErrorBgTemplates: Table<CArray<BgTemplate, 2>> =
    Table((&raw const crate::data::link::sLinkErrorBgTemplates).cast());
static sLinkErrorWindowTemplates: Table<CArray<WindowTemplate, 4>> =
    Table((&raw const crate::data::link::sLinkErrorWindowTemplates).cast());
static sLinkTestDigitsGfx: Table<CArray<u16, 272>> =
    Table((&raw const crate::data::link::sLinkTestDigitsGfx).cast());
static sLinkTestDigitsPal: Table<CArray<u16, 16>> =
    Table((&raw const crate::data::link::sLinkTestDigitsPal).cast());
static sTextColors: Table<CArray<u8, 3>> =
    Table((&raw const crate::data::link::sTextColors).cast());
static sWirelessLinkDisplayGfx: Table<CArray<u32, 298>> =
    Table((&raw const crate::data::link::sWirelessLinkDisplayGfx).cast());
static sWirelessLinkDisplayPal: Table<CArray<u16, 16>> =
    Table((&raw const crate::data::link::sWirelessLinkDisplayPal).cast());
static sWirelessLinkDisplayTilemap: Table<CArray<u32, 123>> =
    Table((&raw const crate::data::link::sWirelessLinkDisplayTilemap).cast());

pub(crate) static sBlockSend: crate::global::Global<BlockTransfer> =
    crate::global::Global::new(unsafe { zeroed() });
pub(crate) static sBlockRecv: crate::global::Global<CArray<BlockTransfer, 4>> =
    crate::global::Global::new(unsafe { zeroed() });
pub(crate) static sBlockSendDelayCounter: crate::global::Global<u32> =
    crate::global::Global::new(0);
pub(crate) static sDummy1: crate::global::Global<u32> = crate::global::Global::new(0);
pub(crate) static sDummy2: crate::global::Global<u8> = crate::global::Global::new(0);
pub(crate) static sPlayerDataExchangeStatus: crate::global::Global<u32> =
    crate::global::Global::new(0);
pub(crate) static sDummy3: crate::global::Global<u32> = crate::global::Global::new(0);
pub(crate) static sLinkTestLastBlockSendPos: crate::global::Global<u8> =
    crate::global::Global::new(0);
pub(crate) static sLinkTestLastBlockRecvPos: crate::global::Global<Aligned<CArray<u8, 4>>> =
    crate::global::Global::new(Aligned(unsafe { zeroed() }));
pub(crate) static sNumVBlanksWithoutSerialIntr: crate::global::Global<u8> =
    crate::global::Global::new(0);
pub(crate) static sSendBufferEmpty: crate::global::Global<u8> = crate::global::Global::new(0);
pub(crate) static sSendNonzeroCheck: crate::global::Global<u16> = crate::global::Global::new(0);
pub(crate) static sRecvNonzeroCheck: crate::global::Global<u16> = crate::global::Global::new(0);
pub(crate) static sChecksumAvailable: crate::global::Global<u8> = crate::global::Global::new(0);
pub(crate) static sHandshakePlayerCount: crate::global::Global<u8> = crate::global::Global::new(0);
#[unsafe(no_mangle)]
#[cfg_attr(target_os = "none", unsafe(link_section = "common_data"))]
pub static gLinkPartnersHeldKeys: crate::global::Global<Aligned<CArray<u16, 6>>> =
    crate::global::Global::new(Aligned(unsafe { zeroed() }));
#[cfg_attr(target_os = "none", unsafe(link_section = "common_data"))]
pub static gLinkDebugSeed: crate::global::Global<u32> = crate::global::Global::new(0);
#[cfg_attr(target_os = "none", unsafe(link_section = "common_data"))]
pub static gLocalLinkPlayerBlock: crate::global::Global<LinkPlayerBlock> =
    crate::global::Global::new(unsafe { zeroed() });
#[cfg_attr(target_os = "none", unsafe(link_section = "common_data"))]
pub static gLinkErrorOccurred: crate::global::Global<u8> = crate::global::Global::new(0);
#[cfg_attr(target_os = "none", unsafe(link_section = "common_data"))]
pub static gLinkDebugFlags: crate::global::Global<u32> = crate::global::Global::new(0);
#[cfg_attr(target_os = "none", unsafe(link_section = "common_data"))]
pub static gLinkFiller1: crate::global::Global<u32> = crate::global::Global::new(0);
#[cfg_attr(target_os = "none", unsafe(link_section = "common_data"))]
pub static gRemoteLinkPlayersNotReceived: crate::global::Global<Aligned<CArray<u8, 4>>> =
    crate::global::Global::new(Aligned(unsafe { zeroed() }));
#[cfg_attr(target_os = "none", unsafe(link_section = "common_data"))]
pub static gBlockReceivedStatus: crate::global::Global<Aligned<CArray<u8, 4>>> =
    crate::global::Global::new(Aligned(unsafe { zeroed() }));
#[cfg_attr(target_os = "none", unsafe(link_section = "common_data"))]
pub static gLinkFiller2: crate::global::Global<u32> = crate::global::Global::new(0);
#[cfg_attr(target_os = "none", unsafe(link_section = "common_data"))]
pub static gLinkHeldKeys: crate::global::Global<u16> = crate::global::Global::new(0);
#[unsafe(no_mangle)]
#[cfg_attr(target_os = "none", unsafe(link_section = "common_data"))]
pub static gRecvCmds: crate::global::Global<Aligned<CArray<CArray<u16, 8>, 5>>> =
    crate::global::Global::new(Aligned(unsafe { zeroed() }));
#[unsafe(no_mangle)]
#[cfg_attr(target_os = "none", unsafe(link_section = "common_data"))]
pub static gLinkStatus: crate::global::Global<u32> = crate::global::Global::new(0);
#[cfg_attr(target_os = "none", unsafe(link_section = "common_data"))]
pub static gLinkDummy1: crate::global::Global<u8> = crate::global::Global::new(0);
#[cfg_attr(target_os = "none", unsafe(link_section = "common_data"))]
pub static gLinkDummy2: crate::global::Global<u8> = crate::global::Global::new(0);
#[cfg_attr(target_os = "none", unsafe(link_section = "common_data"))]
pub static gReadyToExitStandby: crate::global::Global<Aligned<CArray<u8, 4>>> =
    crate::global::Global::new(Aligned(unsafe { zeroed() }));
#[cfg_attr(target_os = "none", unsafe(link_section = "common_data"))]
pub static gReadyToCloseLink: crate::global::Global<Aligned<CArray<u8, 4>>> =
    crate::global::Global::new(Aligned(unsafe { zeroed() }));
#[cfg_attr(target_os = "none", unsafe(link_section = "common_data"))]
pub static gReadyCloseLinkType: crate::global::Global<u16> = crate::global::Global::new(0);
#[cfg_attr(target_os = "none", unsafe(link_section = "common_data"))]
pub static gSuppressLinkErrorMessage: crate::global::Global<u8> = crate::global::Global::new(0);
#[unsafe(no_mangle)]
#[cfg_attr(target_os = "none", unsafe(link_section = "common_data"))]
pub static gWirelessCommType: crate::global::Global<u8> = crate::global::Global::new(0);
#[cfg_attr(target_os = "none", unsafe(link_section = "common_data"))]
pub static gSavedLinkPlayerCount: crate::global::Global<u8> = crate::global::Global::new(0);
#[unsafe(no_mangle)]
#[cfg_attr(target_os = "none", unsafe(link_section = "common_data"))]
pub static gSendCmd: crate::global::Global<Aligned<CArray<u16, 8>>> =
    crate::global::Global::new(Aligned(unsafe { zeroed() }));
#[cfg_attr(target_os = "none", unsafe(link_section = "common_data"))]
pub static gSavedMultiplayerId: crate::global::Global<u8> = crate::global::Global::new(0);
#[unsafe(no_mangle)]
#[cfg_attr(target_os = "none", unsafe(link_section = "common_data"))]
pub static gReceivedRemoteLinkPlayers: crate::global::Global<u8> = crate::global::Global::new(0);
#[cfg_attr(target_os = "none", unsafe(link_section = "common_data"))]
pub static gLinkTestBGInfo: crate::global::Global<LinkTestBGInfo> =
    crate::global::Global::new(unsafe { zeroed() });
#[cfg_attr(target_os = "none", unsafe(link_section = "common_data"))]
pub static gLinkCallback: crate::global::Global<Option<unsafe fn()>> =
    crate::global::Global::new(None);
#[cfg_attr(target_os = "none", unsafe(link_section = "common_data"))]
pub static gShouldAdvanceLinkState: crate::global::Global<u8> = crate::global::Global::new(0);
#[cfg_attr(target_os = "none", unsafe(link_section = "common_data"))]
pub static gLinkTestBlockChecksums: crate::global::Global<Aligned<CArray<u16, 4>>> =
    crate::global::Global::new(Aligned(unsafe { zeroed() }));
#[cfg_attr(target_os = "none", unsafe(link_section = "common_data"))]
pub static gBlockRequestType: crate::global::Global<u8> = crate::global::Global::new(0);
#[cfg_attr(target_os = "none", unsafe(link_section = "common_data"))]
pub static gLinkFiller3: crate::global::Global<u32> = crate::global::Global::new(0);
#[cfg_attr(target_os = "none", unsafe(link_section = "common_data"))]
pub static gLinkFiller4: crate::global::Global<u32> = crate::global::Global::new(0);
#[cfg_attr(target_os = "none", unsafe(link_section = "common_data"))]
pub static gLinkFiller5: crate::global::Global<u32> = crate::global::Global::new(0);
#[cfg_attr(target_os = "none", unsafe(link_section = "common_data"))]
pub static gLastSendQueueCount: crate::global::Global<u8> = crate::global::Global::new(0);
#[cfg_attr(target_os = "none", unsafe(link_section = "common_data"))]
pub static gLink: crate::global::Global<Link> = crate::global::Global::new(unsafe { zeroed() });
#[cfg_attr(target_os = "none", unsafe(link_section = "common_data"))]
pub static gLastRecvQueueCount: crate::global::Global<u8> = crate::global::Global::new(0);
#[cfg_attr(target_os = "none", unsafe(link_section = "common_data"))]
pub static gLinkSavedIme: crate::global::Global<u16> = crate::global::Global::new(0);
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub(crate) static sLinkTestDebugValuesEnabled: crate::global::Global<u8> =
    crate::global::Global::new(0);
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub(crate) static sDummyFlag: crate::global::Global<u8> = crate::global::Global::new(0);
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub static gBerryBlenderKeySendAttempts: crate::global::Global<u32> = crate::global::Global::new(0);
#[unsafe(no_mangle)]
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub static gBlockRecvBuffer: crate::global::Global<Aligned<CArray<CArray<u16, 128>, 5>>> =
    crate::global::Global::new(Aligned(unsafe { zeroed() }));
#[unsafe(no_mangle)]
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub static gBlockSendBuffer: crate::global::Global<Aligned<CArray<u8, 256>>> =
    crate::global::Global::new(Aligned(unsafe { zeroed() }));
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub(crate) static sLinkOpen: crate::global::Global<u8> = crate::global::Global::new(0);
#[unsafe(no_mangle)]
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub static gLinkType: crate::global::Global<u16> = crate::global::Global::new(0);
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub(crate) static sTimeOutCounter: crate::global::Global<u16> = crate::global::Global::new(0);
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub static gLocalLinkPlayer: crate::global::Global<LinkPlayer> =
    crate::global::Global::new(unsafe { zeroed() });
#[unsafe(no_mangle)]
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub static gLinkPlayers: crate::global::Global<CArray<LinkPlayer, 5>> =
    crate::global::Global::new(unsafe { zeroed() });
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub(crate) static sSavedLinkPlayers: crate::global::Global<CArray<LinkPlayer, 5>> =
    crate::global::Global::new(unsafe { zeroed() });
pub(crate) static sLinkErrorBuffer: crate::global::Global<sLinkErrorBuffer_t> =
    crate::global::Global::new(unsafe { zeroed() });
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub(crate) static sReadyCloseLinkAttempts: crate::global::Global<u16> =
    crate::global::Global::new(0);
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub(crate) static sLinkErrorBgTilemapBuffer: crate::global::Global<*mut c_void> =
    crate::global::Global::new(null_mut());

/// `Alloc` with this module's view of its types.
#[inline]
unsafe fn Alloc(a0: u32) -> *mut c_void {
    unsafe { crate::malloc::Alloc(a0) as *mut c_void }
}
/// `CpuSet` with this module's view of its types.
#[inline]
fn CpuSet(a0: *mut c_void, a1: *mut c_void, a2: u32) {
    unsafe {
        crate::syscall::CpuSet(a0 as _, a1 as _, a2);
    }
}
/// `DoSoftReset` with this module's view of its types.
#[inline]
fn DoSoftReset() {
    unsafe {
        crate::agb_main::DoSoftReset();
    }
}
/// `SetMainCallback2` with this module's view of its types.
#[inline]
unsafe fn SetMainCallback2(a0: Option<unsafe fn()>) {
    unsafe {
        crate::agb_main::SetMainCallback2(core::mem::transmute(a0));
    }
}

#[unsafe(no_mangle)]
pub unsafe fn IsWirelessAdapterConnected() -> u8 {
    SetWirelessCommType1();
    InitRFUAPI();
    if rfu_LMAN_REQBN_softReset_and_checkID() == RFU_ID {
        rfu_REQ_stopMode();
        rfu_waitREQComplete();
        return TRUE;
    }
    SetWirelessCommType0_Internal();
    CloseLink();
    RestoreSerialTimer3IntrHandlers();
    FALSE
}
#[unsafe(no_mangle)]
pub fn Task_DestroySelf(taskId: u8) {
    DestroyTask(taskId);
}
unsafe fn InitLinkTestBG(
    paletteNum: u8,
    bgNum: u8,
    screenBaseBlock: u8,
    charBaseBlock: u8,
    baseChar: u16,
) {
    LoadPalette(
        sLinkTestDigitsPal.as_ptr().cast_mut() as *mut c_void,
        paletteNum as u16 * 16,
        32,
    );
    {
        {
            {
                let dmaRegs: *mut u32 = 67109076_usize as *mut u32;
                volatile_write(
                    dmaRegs,
                    sLinkTestDigitsGfx.as_ptr().cast_mut() as usize as u32,
                );
                volatile_write(
                    dmaRegs.at(1),
                    ((0x6000000 + 0x4000 * charBaseBlock as i32) as usize as *mut u16)
                        .at(16 * baseChar as i32) as usize as u32,
                );
                volatile_write(dmaRegs.at(2), 0x80000110);
                let _ = (dmaRegs.at(2)).read_volatile();
            }
        }
    }
    crate::gl!(gLinkTestBGInfo.screenBaseBlock = screenBaseBlock as u32);
    crate::gl!(gLinkTestBGInfo.paletteNum = paletteNum as u32);
    crate::gl!(gLinkTestBGInfo.baseChar = baseChar as u32);
    match bgNum {
        1 => {
            SetGpuReg(
                REG_OFFSET_BG1CNT,
                (screenBaseBlock as u16) << 8 | 1 | (charBaseBlock as u16) << 2,
            );
        }
        2 => {
            SetGpuReg(
                REG_OFFSET_BG2CNT,
                (screenBaseBlock as u16) << 8 | 1 | (charBaseBlock as u16) << 2,
            );
        }
        3 => {
            SetGpuReg(
                REG_OFFSET_BG3CNT,
                (screenBaseBlock as u16) << 8 | 1 | (charBaseBlock as u16) << 2,
            );
        }
        _ => {}
    }
    SetGpuReg(REG_OFFSET_BG0HOFS + bgNum * 4, 0);
    SetGpuReg(REG_OFFSET_BG0VOFS + bgNum * 4, 0);
}
unsafe fn LoadLinkTestBgGfx(paletteNum: u8, bgNum: u8, screenBaseBlock: u8, charBaseBlock: u8) {
    LoadPalette(
        sLinkTestDigitsPal.as_ptr().cast_mut() as *mut c_void,
        paletteNum as u16 * 16,
        32,
    );
    {
        {
            {
                let dmaRegs: *mut u32 = 67109076_usize as *mut u32;
                volatile_write(
                    dmaRegs,
                    sLinkTestDigitsGfx.as_ptr().cast_mut() as usize as u32,
                );
                volatile_write(
                    dmaRegs.at(1),
                    (0x6000000 + 0x4000 * charBaseBlock as i32) as usize as *mut u16 as usize
                        as u32,
                );
                volatile_write(dmaRegs.at(2), 0x80000110);
                let _ = (dmaRegs.at(2)).read_volatile();
            }
        }
    }
    crate::gl!(gLinkTestBGInfo.screenBaseBlock = screenBaseBlock as u32);
    crate::gl!(gLinkTestBGInfo.paletteNum = paletteNum as u32);
    crate::gl!(gLinkTestBGInfo.baseChar = 0);
    SetGpuReg(
        sBGControlRegs[bgNum],
        (screenBaseBlock as u16) << 8 | (charBaseBlock as u16) << 2,
    );
}
unsafe fn LinkTestScreen() {
    ResetSpriteData();
    FreeAllSpritePalettes();
    ResetTasks();
    SetVBlankCallback(Some(VBlankCB_LinkError));
    ResetBlockSend();
    gLinkType.set(LINKTYPE_TRADE);
    OpenLink();
    SeedRng(crate::gl!(gMain.vblankCounter2) as u16);
    for i in 0..(TRAINER_ID_LENGTH as i32) {
        (*crate::gl!(gSaveBlock2Ptr)).playerTrainerId[i] = (Random() as i32 % 256) as u8;
    }
    InitLinkTestBG(0, 2, 4, 0, 0);
    SetGpuReg(0x0, 5440);
    CreateTask(Some(Task_DestroySelf), 0);
    RunTasks();
    AnimateSprites();
    BuildOamBuffer();
    UpdatePaletteFade();
    sDummy3.set(FALSE as u32);
    InitLocalLinkPlayer();
    CreateTask(Some(Task_PrintTestData), 0);
    SetMainCallback2(Some(CB2_LinkTest));
}
pub unsafe fn SetLocalLinkPlayerId(playerId: u8) {
    crate::gl!(gLocalLinkPlayer.id = playerId as u16);
}
unsafe fn InitLocalLinkPlayer() {
    crate::gl!(
        gLocalLinkPlayer.trainerId = (*crate::gl!(gSaveBlock2Ptr)).playerTrainerId[0] as u32
            | ((*crate::gl!(gSaveBlock2Ptr)).playerTrainerId[1] as u32) << 8
            | ((*crate::gl!(gSaveBlock2Ptr)).playerTrainerId[2] as u32) << 16
            | ((*crate::gl!(gSaveBlock2Ptr)).playerTrainerId[3] as u32) << 24
    );
    StringCopy(
        (*gLocalLinkPlayer.as_ptr()).name.as_mut_ptr(),
        (*crate::gl!(gSaveBlock2Ptr)).playerName.as_mut_ptr(),
    );
    crate::gl!(gLocalLinkPlayer.gender = (*crate::gl!(gSaveBlock2Ptr)).playerGender);
    crate::gl!(gLocalLinkPlayer.linkType = gLinkType.get() as u32);
    crate::gl!(gLocalLinkPlayer.language = gGameLanguage as u16);
    crate::gl!(gLocalLinkPlayer.version = gGameVersion as u16 + 0x4000);
    crate::gl!(gLocalLinkPlayer.lp_field_2 = 0x8000);
    crate::gl!(gLocalLinkPlayer.progressFlags = IsNationalPokedexEnabled() as u8);
    if FlagGet(FLAG_IS_CHAMPION) != 0 {
        crate::gl!(gLocalLinkPlayer.progressFlags |= 0x10);
    }
}
pub(crate) unsafe fn VBlankCB_LinkError() {
    LoadOam();
    ProcessSpriteCopyRequests();
    TransferPlttBuffer();
}
unsafe fn InitLink() {
    for i in 0..(CMD_LENGTH as i32) {
        crate::gl!(gSendCmd[i] = LINKCMD_NONE);
    }
    sLinkOpen.set(TRUE);
    EnableSerial();
}
pub(crate) fn Task_TriggerHandshake(taskId: u8) {
    if ({
        task_set(taskId, 0, task_get(taskId, 0) + 1);
        task_get(taskId, 0)
    }) == 5
    {
        gShouldAdvanceLinkState.set(1);
        DestroyTask(taskId);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn OpenLink() {
    if gWirelessCommType.get() == 0 {
        ResetSerial();
        InitLink();
        crate::gl!(gLinkCallback = Some(LinkCB_RequestPlayerDataExchange));
        gLinkVSyncDisabled.set(FALSE);
        gLinkErrorOccurred.set(FALSE);
        gSuppressLinkErrorMessage.set(FALSE);
        ResetBlockReceivedFlags();
        ResetBlockSend();
        sDummy1.set(FALSE as u32);
        gLinkDummy2.set(FALSE);
        gLinkDummy1.set(FALSE);
        gReadyCloseLinkType.set(0);
        CreateTask(Some(Task_TriggerHandshake), 2);
    } else {
        InitRFUAPI();
    }
    gReceivedRemoteLinkPlayers.set(0);
    for i in 0..MAX_LINK_PLAYERS {
        crate::gl!(gRemoteLinkPlayersNotReceived[i] = TRUE);
        crate::gl!(gReadyToCloseLink[i] = FALSE);
        crate::gl!(gReadyToExitStandby[i] = FALSE);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn CloseLink() {
    gReceivedRemoteLinkPlayers.set(FALSE);
    if gWirelessCommType.get() != 0 {
        LinkRfu_Shutdown();
    }
    sLinkOpen.set(FALSE);
    DisableSerial();
}
unsafe fn TestBlockTransfer(nothing: u8, is: u8, used: u8) {
    if sLinkTestLastBlockSendPos.get() as u16 != crate::gl!(sBlockSend.pos) {
        LinkTest_PrintHex(crate::gl!(sBlockSend.pos) as u32, 2, 3, 2);
        sLinkTestLastBlockSendPos.set(crate::gl!(sBlockSend.pos) as u8);
    }
    let mut i: u8 = 0;
    while i < MAX_LINK_PLAYERS as u8 {
        if crate::gl!(sLinkTestLastBlockRecvPos[i]) as u16 != crate::gl!(sBlockRecv[i].pos) {
            LinkTest_PrintHex(crate::gl!(sBlockRecv[i].pos) as u32, 2, i + 4, 2);
            crate::gl!(sLinkTestLastBlockRecvPos[i] = crate::gl!(sBlockRecv[i].pos) as u8);
        }
        i += 1;
    }
    let status: u8 = GetBlockReceivedStatus();
    if status == 0xF {
        for i in 0..(MAX_LINK_PLAYERS as u8) {
            if shr_i32(status as i32, i as u32) & 1 != 0 {
                crate::gl!(
                    gLinkTestBlockChecksums[i] = LinkTestCalcBlockChecksum(
                        (*gBlockRecvBuffer.as_ptr())[i].as_mut_ptr(),
                        crate::gl!(sBlockRecv[i].size)
                    )
                );
                ResetBlockReceivedFlag(i);
                if crate::gl!(gLinkTestBlockChecksums[i]) != 0x0342 {
                    sLinkTestDebugValuesEnabled.set(FALSE);
                    sDummyFlag.set(FALSE);
                }
            }
        }
    }
}
unsafe fn LinkTestProcessKeyInput() {
    if crate::gl!(gMain.newKeys) as i32 & A_BUTTON != 0 {
        gShouldAdvanceLinkState.set(1);
    }
    if crate::gl!(gMain.heldKeys) as i32 & B_BUTTON != 0 {
        InitBlockSend(
            (*(crate::gl!(&raw @at crate::malloc::gHeap;).cast_const())
                .cast::<CArray<u8, 114688>>()
                .cast_mut())
            .as_mut_ptr()
            .at(16384) as *mut c_void,
            0x00002004,
        );
    }
    if crate::gl!(gMain.newKeys) as i32 & L_BUTTON != 0 {
        BeginNormalPaletteFade(PALETTES_ALL, 0, 16, 0, 2);
    }
    if crate::gl!(gMain.newKeys) as i32 & START_BUTTON != 0 {
        SetSuppressLinkErrorMessage(TRUE);
    }
    if crate::gl!(gMain.newKeys) as i32 & R_BUTTON != 0 {
        TrySavingData(SAVE_LINK);
    }
    if crate::gl!(gMain.newKeys) as i32 & SELECT_BUTTON != 0 {
        SetCloseLinkCallback();
    }
    if sLinkTestDebugValuesEnabled.get() != 0 {
        SetLinkDebugValues(
            crate::gl!(gMain.vblankCounter2),
            (if (*gLinkCallback.as_ptr()).is_some() {
                gLinkVSyncDisabled.get() as i32
            } else {
                gLinkVSyncDisabled.get() as i32 | 0x10
            }) as u32,
        );
    }
}
pub(crate) unsafe fn CB2_LinkTest() {
    LinkTestProcessKeyInput();
    TestBlockTransfer(1, 1, 0);
    RunTasks();
    AnimateSprites();
    BuildOamBuffer();
    UpdatePaletteFade();
}
pub unsafe fn LinkMain2(heldKeys: *mut u16) -> u16 {
    if sLinkOpen.get() == 0 {
        return 0;
    }
    for i in 0..CMD_LENGTH {
        crate::gl!(gSendCmd[i] = 0);
    }
    gLinkHeldKeys.set(*heldKeys);
    if gLinkStatus.get() & LINK_STAT_CONN_ESTABLISHED != 0 {
        ProcessRecvCmds((*(67109160_usize as *mut SioMultiCnt)).id() as u8);
        if (*gLinkCallback.as_ptr()).is_some() {
            (*gLinkCallback.as_ptr()).unwrap_unchecked()();
        }
        TrySetLinkErrorBuffer();
    }
    gLinkStatus.get() as u16
}
unsafe fn HandleReceiveRemoteLinkPlayer(who: u8) {
    let mut count: i32 = 0;
    crate::gl!(gRemoteLinkPlayersNotReceived[who] = FALSE);
    let mut i: i32 = 0;
    while i < GetLinkPlayerCount_2() as i32 {
        count += crate::gl!(gRemoteLinkPlayersNotReceived[i]) as i32;
        i += 1;
    }
    if count == 0 && gReceivedRemoteLinkPlayers.get() == 0 {
        gReceivedRemoteLinkPlayers.set(1);
    }
}
unsafe fn ProcessRecvCmds(unused: u8) {
    for i in 0..(MAX_LINK_PLAYERS as u16) {
        'l1: {
            crate::gl!(gLinkPartnersHeldKeys[i] = 0);
            if crate::gl!(gRecvCmds[i][0]) == 0 {
                break 'l1;
            }
            'l3: {
                match crate::gl!(gRecvCmds[i][0]) {
                    LINKCMD_SEND_LINK_TYPE => {
                        InitLocalLinkPlayer();
                        let block: *mut LinkPlayerBlock = crate::gl!(&raw gLocalLinkPlayerBlock);
                        (*block).linkPlayer = crate::gl!(gLocalLinkPlayer);
                        memcpy(
                            (*block).magic1.as_mut_ptr(),
                            sASCIIGameFreakInc.as_ptr().cast_mut(),
                            15,
                        );
                        memcpy(
                            (*block).magic2.as_mut_ptr(),
                            sASCIIGameFreakInc.as_ptr().cast_mut(),
                            15,
                        );
                        InitBlockSend(block as *mut c_void, 60);
                        break 'l3;
                    }
                    LINKCMD_BLENDER_SEND_KEYS => {
                        crate::gl!(gLinkPartnersHeldKeys[i] = crate::gl!(gRecvCmds[i][1]));
                    }
                    LINKCMD_DUMMY_1 => {
                        gLinkDummy2.set(TRUE);
                    }
                    LINKCMD_DUMMY_2 => {
                        gLinkDummy2.set(TRUE);
                    }
                    LINKCMD_INIT_BLOCK => {
                        let blockRecv: *mut BlockTransfer = crate::gl!(&raw sBlockRecv[i]);
                        (*blockRecv).pos = 0;
                        (*blockRecv).size = crate::gl!(gRecvCmds[i][1]);
                        (*blockRecv).multiplayerId = crate::gl!(gRecvCmds[i][2]) as u8;
                        break 'l3;
                    }
                    LINKCMD_CONT_BLOCK => {
                        if crate::gl!(sBlockRecv[i].size) > BLOCK_BUFFER_SIZE as u16 {
                            let buffer: *mut u16 =
                                (*(crate::gl!(&raw @at crate::decompress::gDecompressionBuffer;)
                                    .cast_const())
                                .cast::<CArray<u8, 16384>>()
                                .cast_mut())
                                .as_mut_ptr() as *mut u16;
                            for j in 0..7u16 {
                                *buffer.at(crate::gl!(sBlockRecv[i].pos) as i32 / 2 + j as i32) = {
                                    let gl_i0 = j as i32 + 1;
                                    crate::gl!(gRecvCmds[i][gl_i0])
                                };
                            }
                        } else {
                            for j in 0..7u16 {
                                (*gBlockRecvBuffer.as_ptr())[i]
                                    [crate::gl!(sBlockRecv[i].pos) as i32 / 2 + j as i32] = {
                                    let gl_i1 = j as i32 + 1;
                                    crate::gl!(gRecvCmds[i][gl_i1])
                                };
                            }
                        }
                        crate::gl!(sBlockRecv[i].pos += 14);
                        if crate::gl!(sBlockRecv[i].pos) >= crate::gl!(sBlockRecv[i].size) {
                            if crate::gl!(gRemoteLinkPlayersNotReceived[i]) == TRUE {
                                let block: *mut LinkPlayerBlock =
                                    crate::gl!(&raw gBlockRecvBuffer[i]) as *mut LinkPlayerBlock;
                                let linkPlayer: *mut LinkPlayer = crate::gl!(&raw gLinkPlayers[i]);
                                *linkPlayer = (*block).linkPlayer;
                                if (*linkPlayer).version as i32 & 0xFF == VERSION_RUBY
                                    || (*linkPlayer).version as i32 & 0xFF == VERSION_SAPPHIRE
                                {
                                    (*linkPlayer).progressFlagsCopy = 0;
                                    (*linkPlayer).neverRead = 0;
                                    (*linkPlayer).progressFlags = 0;
                                }
                                ConvertLinkPlayerName(linkPlayer);
                                if strcmp(
                                    (*block).magic1.as_mut_ptr(),
                                    sASCIIGameFreakInc.as_ptr().cast_mut(),
                                ) != 0
                                    || strcmp(
                                        (*block).magic2.as_mut_ptr(),
                                        sASCIIGameFreakInc.as_ptr().cast_mut(),
                                    ) != 0
                                {
                                    SetMainCallback2(Some(CB2_LinkError));
                                } else {
                                    HandleReceiveRemoteLinkPlayer(i as u8);
                                }
                            } else {
                                SetBlockReceivedFlag(i as u8);
                            }
                        }
                    }
                    LINKCMD_READY_CLOSE_LINK => {
                        crate::gl!(gReadyToCloseLink[i] = TRUE);
                    }
                    LINKCMD_READY_EXIT_STANDBY => {
                        crate::gl!(gReadyToExitStandby[i] = TRUE);
                    }
                    LINKCMD_BLENDER_NO_PBLOCK_SPACE => {
                        SetBerryBlenderLinkCallback();
                    }
                    LINKCMD_SEND_BLOCK_REQ => {
                        SendBlock(
                            0,
                            sBlockRequests[crate::gl!(gRecvCmds[i][1])].address,
                            sBlockRequests[crate::gl!(gRecvCmds[i][1])].size as u16,
                        );
                    }
                    LINKCMD_SEND_HELD_KEYS => {
                        crate::gl!(gLinkPartnersHeldKeys[i] = crate::gl!(gRecvCmds[i][1]));
                    }
                    _ => {}
                }
            }
        }
    }
}
fn BuildSendCmd(command: u16) {
    'l1: {
        match command {
            LINKCMD_SEND_LINK_TYPE => {
                crate::gl!(gSendCmd[0] = LINKCMD_SEND_LINK_TYPE);
                crate::gl!(gSendCmd[1] = gLinkType.get());
            }
            LINKCMD_READY_EXIT_STANDBY => {
                crate::gl!(gSendCmd[0] = LINKCMD_READY_EXIT_STANDBY);
            }
            LINKCMD_BLENDER_SEND_KEYS => {
                crate::gl!(gSendCmd[0] = LINKCMD_BLENDER_SEND_KEYS);
                crate::gl!(gSendCmd[1] = crate::gl!(gMain.heldKeys));
            }
            LINKCMD_DUMMY_1 => {
                crate::gl!(gSendCmd[0] = LINKCMD_DUMMY_1);
            }
            LINKCMD_SEND_EMPTY => {
                crate::gl!(gSendCmd[0] = LINKCMD_SEND_EMPTY);
                crate::gl!(gSendCmd[1] = 0);
            }
            LINKCMD_SEND_0xEE => {
                crate::gl!(gSendCmd[0] = LINKCMD_SEND_0xEE);
                for i in 0..5u8 {
                    {
                        let gl_v = 0xEE;
                        let gl_i2 = i as i32 + 1;
                        crate::gl!(gSendCmd[gl_i2] = gl_v)
                    };
                }
                break 'l1;
            }
            LINKCMD_INIT_BLOCK => {
                crate::gl!(gSendCmd[0] = LINKCMD_INIT_BLOCK);
                crate::gl!(gSendCmd[1] = crate::gl!(sBlockSend.size));
                crate::gl!(gSendCmd[2] = crate::gl!(sBlockSend.multiplayerId) as u16 + 0x80);
            }
            LINKCMD_BLENDER_NO_PBLOCK_SPACE => {
                crate::gl!(gSendCmd[0] = LINKCMD_BLENDER_NO_PBLOCK_SPACE);
            }
            LINKCMD_SEND_ITEM => {
                crate::gl!(gSendCmd[0] = LINKCMD_SEND_ITEM);
                crate::gl!(gSendCmd[1] = gSpecialVar_ItemId.get());
            }
            LINKCMD_SEND_BLOCK_REQ => {
                crate::gl!(gSendCmd[0] = LINKCMD_SEND_BLOCK_REQ);
                crate::gl!(gSendCmd[1] = gBlockRequestType.get() as u16);
            }
            LINKCMD_READY_CLOSE_LINK => {
                crate::gl!(gSendCmd[0] = LINKCMD_READY_CLOSE_LINK);
                crate::gl!(gSendCmd[1] = gReadyCloseLinkType.get());
            }
            LINKCMD_DUMMY_2 => {
                crate::gl!(gSendCmd[0] = LINKCMD_DUMMY_2);
            }
            LINKCMD_SEND_HELD_KEYS => {
                if gHeldKeyCodeToSend.get() == 0 || gLinkTransferringData.get() != 0 {
                    break 'l1;
                }
                crate::gl!(gSendCmd[0] = LINKCMD_SEND_HELD_KEYS);
                crate::gl!(gSendCmd[1] = gHeldKeyCodeToSend.get());
            }
            _ => {}
        }
    }
}
pub fn StartSendingKeysToLink() {
    if gWirelessCommType.get() != 0 {
        StartSendingKeysToRfu();
    }
    crate::gl!(gLinkCallback = Some(LinkCB_SendHeldKeys));
}
pub fn IsSendingKeysToLink() -> u32 {
    if gWirelessCommType.get() != 0 {
        return IsSendingKeysToRfu();
    }
    if crate::gl!(gLinkCallback) == Some(LinkCB_SendHeldKeys as unsafe fn()) {
        return TRUE as u32;
    }
    FALSE as u32
}
pub(crate) fn LinkCB_SendHeldKeys() {
    if gReceivedRemoteLinkPlayers.get() == TRUE {
        BuildSendCmd(LINKCMD_SEND_HELD_KEYS);
    }
}
pub fn ClearLinkCallback() {
    if gWirelessCommType.get() != 0 {
        ClearLinkRfuCallback();
    } else {
        crate::gl!(gLinkCallback = None);
    }
}
pub fn ClearLinkCallback_2() {
    if gWirelessCommType.get() != 0 {
        ClearLinkRfuCallback();
    } else {
        crate::gl!(gLinkCallback = None);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn GetLinkPlayerCount() -> u8 {
    if gWirelessCommType.get() != 0 {
        return Rfu_GetLinkPlayerCount();
    }
    ((gLinkStatus.get() & 0x0000001C) >> 2) as u8
}
unsafe fn AreAnyLinkPlayersUsingVersions(version1: u32, version2: u32) -> i32 {
    let nPlayers: u8 = GetLinkPlayerCount();
    for i in 0..(nPlayers as i32) {
        if crate::gl!(gLinkPlayers[i].version) as u32 & 0xFF == version1
            || crate::gl!(gLinkPlayers[i].version) as u32 & 0xFF == version2
        {
            return 1;
        }
    }
    -1
}
pub fn LinkDummy_Return2() -> u32 {
    2
}
unsafe fn IsFullLinkGroupWithNoRS() -> u32 {
    if GetLinkPlayerCount() != MAX_LINK_PLAYERS as u8
        || AreAnyLinkPlayersUsingVersions(VERSION_RUBY as u32, VERSION_SAPPHIRE as u32) < 0
    {
        return FALSE as u32;
    }
    TRUE as u32
}
pub unsafe fn Link_AnyPartnersPlayingRubyOrSapphire() -> u32 {
    if AreAnyLinkPlayersUsingVersions(VERSION_RUBY as u32, VERSION_SAPPHIRE as u32) >= 0 {
        return TRUE as u32;
    }
    FALSE as u32
}
pub unsafe fn Link_AnyPartnersPlayingFRLG_JP() -> u32 {
    let i: i32 = AreAnyLinkPlayersUsingVersions(VERSION_FIRE_RED as u32, VERSION_LEAF_GREEN as u32);
    if i >= 0 && crate::gl!(gLinkPlayers[i].language) == LANGUAGE_JAPANESE as u16 {
        return TRUE as u32;
    }
    FALSE as u32
}
pub unsafe fn OpenLinkTimed() {
    sPlayerDataExchangeStatus.set(EXCHANGE_NOT_STARTED);
    sTimeOutCounter.set(0);
    OpenLink();
}
#[unsafe(no_mangle)]
pub unsafe fn GetLinkPlayerDataExchangeStatusTimed(minPlayers: i32, maxPlayers: i32) -> u8 {
    let mut i: i32 = 0;
    let mut index: u32 = 0;
    let mut numPlayers: u8 = 0;
    let mut linkType1: u32 = 0;
    let mut linkType2: u32 = 0;
    let mut count: i32 = 0;
    if gReceivedRemoteLinkPlayers.get() == TRUE {
        numPlayers = GetLinkPlayerCount_2();
        if minPlayers > numPlayers as i32 || numPlayers as i32 > maxPlayers {
            sPlayerDataExchangeStatus.set(EXCHANGE_WRONG_NUM_PLAYERS as u32);
            return sPlayerDataExchangeStatus.get() as u8;
        } else {
            if GetLinkPlayerCount() == 0 {
                gLinkErrorOccurred.set(TRUE);
                CloseLink();
            }
            i = 0;
            index = 0;
            while i < GetLinkPlayerCount() as i32 {
                if crate::gl!(gLinkPlayers[index].linkType) == crate::gl!(gLinkPlayers[0].linkType)
                {
                    count += 1;
                }
                index += 1;
                i += 1;
            }
            if count == GetLinkPlayerCount() as i32 {
                if crate::gl!(gLinkPlayers[0].linkType) == LINKTYPE_TRADE_SETUP as u32 {
                    match GetGameProgressForLinkTrade() {
                        TRADE_PLAYER_NOT_READY => {
                            sPlayerDataExchangeStatus.set(EXCHANGE_PLAYER_NOT_READY as u32);
                        }
                        TRADE_PARTNER_NOT_READY => {
                            sPlayerDataExchangeStatus.set(EXCHANGE_PARTNER_NOT_READY as u32);
                        }
                        TRADE_BOTH_PLAYERS_READY => {
                            sPlayerDataExchangeStatus.set(EXCHANGE_COMPLETE);
                        }
                        _ => {}
                    }
                } else {
                    sPlayerDataExchangeStatus.set(EXCHANGE_COMPLETE);
                }
            } else {
                sPlayerDataExchangeStatus.set(EXCHANGE_DIFF_SELECTIONS);
                linkType1 = {
                    let gl_i0 = GetMultiplayerId();
                    crate::gl!(gLinkPlayers[gl_i0].linkType)
                };
                linkType2 = {
                    let gl_i1 = GetMultiplayerId() as i32 ^ 1;
                    crate::gl!(gLinkPlayers[gl_i1].linkType)
                };
                if linkType1 == LINKTYPE_BATTLE_TOWER_50 as u32
                    && linkType2 == LINKTYPE_BATTLE_TOWER_OPEN as u32
                    || linkType1 == LINKTYPE_BATTLE_TOWER_OPEN as u32
                        && linkType2 == LINKTYPE_BATTLE_TOWER_50 as u32
                {
                    gSpecialVar_0x8005 = 3;
                }
            }
        }
    } else if ({
        sTimeOutCounter.set(sTimeOutCounter.get() + 1);
        sTimeOutCounter.get()
    }) > 600
    {
        sPlayerDataExchangeStatus.set(EXCHANGE_TIMED_OUT);
    }
    sPlayerDataExchangeStatus.get() as u8
}
pub unsafe fn IsLinkPlayerDataExchangeComplete() -> u8 {
    let mut retval: u8 = 0;
    let mut count: u8 = 0;
    let mut i: u8 = 0;
    while i < GetLinkPlayerCount() {
        if crate::gl!(gLinkPlayers[i].linkType) == crate::gl!(gLinkPlayers[0].linkType) {
            count += 1;
        }
        i += 1;
    }
    if count == GetLinkPlayerCount() {
        retval = TRUE;
        sPlayerDataExchangeStatus.set(EXCHANGE_COMPLETE);
    } else {
        retval = FALSE;
        sPlayerDataExchangeStatus.set(EXCHANGE_DIFF_SELECTIONS);
    }
    retval
}
pub fn GetLinkPlayerTrainerId(who: u8) -> u32 {
    crate::gl!(gLinkPlayers[who].trainerId)
}
pub unsafe fn ResetLinkPlayers() {
    let mut i: i32 = 0;
    while i <= MAX_LINK_PLAYERS {
        crate::gl!(
            gLinkPlayers[i] = {
                let mut lit1: LinkPlayer = zeroed();
                lit1.version = 0;
                lit1
            }
        );
        i += 1;
    }
}
fn ResetBlockSend() {
    crate::gl!(sBlockSend.active = FALSE);
    crate::gl!(sBlockSend.pos = 0);
    crate::gl!(sBlockSend.size = 0);
    crate::gl!(sBlockSend.src = null_mut());
}
unsafe fn InitBlockSend(src: *mut c_void, size: u32) -> u32 {
    if crate::gl!(sBlockSend.active) != 0 {
        return FALSE as u32;
    }
    crate::gl!(sBlockSend.multiplayerId = GetMultiplayerId());
    crate::gl!(sBlockSend.active = TRUE);
    crate::gl!(sBlockSend.size = size as u16);
    crate::gl!(sBlockSend.pos = 0);
    if size > BLOCK_BUFFER_SIZE {
        crate::gl!(sBlockSend.src = src as *mut u8);
    } else {
        if !core::ptr::addr_eq(src, (*gBlockSendBuffer.as_ptr()).as_mut_ptr()) {
            memcpy(
                (*gBlockSendBuffer.as_ptr()).as_mut_ptr(),
                src as *mut u8,
                size,
            );
        }
        crate::gl!(sBlockSend.src = (*gBlockSendBuffer.as_ptr()).as_mut_ptr());
    }
    BuildSendCmd(LINKCMD_INIT_BLOCK);
    crate::gl!(gLinkCallback = Some(LinkCB_BlockSendBegin));
    sBlockSendDelayCounter.set(0);
    TRUE as u32
}
pub(crate) fn LinkCB_BlockSendBegin() {
    if ({
        sBlockSendDelayCounter.set(sBlockSendDelayCounter.get() + 1);
        sBlockSendDelayCounter.get()
    }) > 2
    {
        crate::gl!(gLinkCallback = Some(LinkCB_BlockSend));
    }
}
pub(crate) unsafe fn LinkCB_BlockSend() {
    let mut src: *mut u8 = null_mut();
    src = crate::gl!(sBlockSend.src);
    crate::gl!(gSendCmd[0] = LINKCMD_CONT_BLOCK);
    for i in 0..7i32 {
        {
            let gl_v = (*src.at(crate::gl!(sBlockSend.pos) as i32 + i * 2 + 1) as u16) << 8
                | *src.at(crate::gl!(sBlockSend.pos) as i32 + i * 2) as u16;
            let gl_i3 = i + 1;
            crate::gl!(gSendCmd[gl_i3] = gl_v)
        };
    }
    crate::gl!(sBlockSend.pos += 14);
    if crate::gl!(sBlockSend.size) <= crate::gl!(sBlockSend.pos) {
        crate::gl!(sBlockSend.active = FALSE);
        crate::gl!(gLinkCallback = Some(LinkCB_BlockSendEnd));
    }
}
pub(crate) fn LinkCB_BlockSendEnd() {
    crate::gl!(gLinkCallback = None);
}
pub(crate) unsafe fn LinkCB_BerryBlenderSendHeldKeys() {
    GetMultiplayerId();
    BuildSendCmd(LINKCMD_BLENDER_SEND_KEYS);
    gBerryBlenderKeySendAttempts.set(gBerryBlenderKeySendAttempts.get() + (1));
}
pub unsafe fn SetBerryBlenderLinkCallback() {
    gBerryBlenderKeySendAttempts.set(0);
    if gWirelessCommType.get() != 0 {
        Rfu_SetBerryBlenderLinkCallback();
    } else {
        crate::gl!(gLinkCallback = Some(LinkCB_BerryBlenderSendHeldKeys));
    }
}
fn GetBerryBlenderKeySendAttempts() -> u32 {
    gBerryBlenderKeySendAttempts.get()
}
fn SendBerryBlenderNoSpaceForPokeblocks() {
    BuildSendCmd(LINKCMD_BLENDER_NO_PBLOCK_SPACE);
}
#[unsafe(no_mangle)]
pub unsafe fn GetMultiplayerId() -> u8 {
    if gWirelessCommType.get() == TRUE {
        return Rfu_GetMultiplayerId();
    }
    (*(67109160_usize as *mut SioMultiCnt)).id() as u8
}
pub unsafe fn BitmaskAllOtherLinkPlayers() -> u8 {
    let mpId: u8 = GetMultiplayerId();
    15 ^ shl_i32(1, mpId as u32) as u8
}
#[unsafe(no_mangle)]
pub unsafe fn SendBlock(unused: u8, src: *mut c_void, size: u16) -> u8 {
    if gWirelessCommType.get() == TRUE {
        return Rfu_InitBlockSend(src as *mut u8, size as u32) as u8;
    }
    InitBlockSend(src, size as u32) as u8
}
pub unsafe fn SendBlockRequest(blockReqType: u8) -> u8 {
    if gWirelessCommType.get() == TRUE {
        return Rfu_SendBlockRequest(blockReqType);
    }
    if (*gLinkCallback.as_ptr()).is_none() {
        gBlockRequestType.set(blockReqType);
        BuildSendCmd(LINKCMD_SEND_BLOCK_REQ);
        return TRUE;
    }
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn IsLinkTaskFinished() -> u8 {
    if gWirelessCommType.get() == TRUE {
        return IsLinkRfuTaskFinished();
    }
    (*gLinkCallback.as_ptr()).is_none() as u8
}
#[unsafe(no_mangle)]
pub unsafe fn GetBlockReceivedStatus() -> u8 {
    if gWirelessCommType.get() == TRUE {
        return Rfu_GetBlockReceivedStatus();
    }
    crate::gl!(gBlockReceivedStatus[3]) << 3
        | crate::gl!(gBlockReceivedStatus[2]) << 2
        | crate::gl!(gBlockReceivedStatus[1]) << 1
        | crate::gl!(gBlockReceivedStatus[0])
}
fn SetBlockReceivedFlag(who: u8) {
    if gWirelessCommType.get() == TRUE {
        Rfu_SetBlockReceivedFlag(who);
    } else {
        crate::gl!(gBlockReceivedStatus[who] = TRUE);
    }
}
#[unsafe(no_mangle)]
pub fn ResetBlockReceivedFlags() {
    if gWirelessCommType.get() == TRUE {
        for i in 0..MAX_RFU_PLAYERS {
            Rfu_ResetBlockReceivedFlag(i as u8);
        }
    } else {
        for i in 0..MAX_LINK_PLAYERS {
            crate::gl!(gBlockReceivedStatus[i] = FALSE);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe fn ResetBlockReceivedFlag(who: u8) {
    if gWirelessCommType.get() == TRUE {
        Rfu_ResetBlockReceivedFlag(who);
    } else if crate::gl!(gBlockReceivedStatus[who]) != 0 {
        crate::gl!(gBlockReceivedStatus[who] = FALSE);
    }
}
#[unsafe(no_mangle)]
pub fn CheckShouldAdvanceLinkState() {
    if gLinkStatus.get() & LINK_STAT_MASTER != 0 && (gLinkStatus.get() & 0x0000001C) >> 2 > 1 {
        gShouldAdvanceLinkState.set(1);
    }
}
unsafe fn LinkTestCalcBlockChecksum(src: *mut u16, size: u16) -> u16 {
    let mut chksum: u16 = 0;
    let mut i: u16 = 0;
    while (i as i32) < size as i32 / 2 {
        chksum += *src.at(i);
        i += 1;
    }
    chksum
}
unsafe fn LinkTest_PrintNumChar(val: u8, x: u8, y: u8) {
    let vAddr: *mut u16 =
        (0x6000000 + 0x800 * crate::gl!(gLinkTestBGInfo.screenBaseBlock)) as usize as *mut u16;
    *vAddr.at(y as i32 * 32 + x as i32) = ((crate::gl!(gLinkTestBGInfo.paletteNum) as u16) << 12)
        | (val as u16 + 1 + crate::gl!(gLinkTestBGInfo.baseChar) as u16);
}
unsafe fn LinkTest_PrintChar(val: u8, x: u8, y: u8) {
    let vAddr: *mut u16 =
        (0x6000000 + 0x800 * crate::gl!(gLinkTestBGInfo.screenBaseBlock)) as usize as *mut u16;
    *vAddr.at(y as i32 * 32 + x as i32) = ((crate::gl!(gLinkTestBGInfo.paletteNum) as u16) << 12)
        | (val as u16 + crate::gl!(gLinkTestBGInfo.baseChar) as u16);
}
unsafe fn LinkTest_PrintHex(mut num: u32, mut x: u8, y: u8, length: u8) {
    let mut buff: CArray<u8, 16> = zeroed();
    for i in 0..(length as i32) {
        buff[i] = num as u8 & 0xF;
        num >>= 4;
    }
    let mut i: i32 = length as i32 - 1;
    while i >= 0 {
        LinkTest_PrintNumChar(buff[i], x, y);
        x += 1;
        i -= 1;
    }
}
unsafe fn LinkTest_PrintInt(mut num: i32, mut x: u8, y: u8, length: u8) {
    let mut buff: CArray<u8, 16> = zeroed();
    let mut negX: i32 = -1;
    if num < 0 {
        negX = x as i32;
        num = -num;
    }
    for i in 0..(length as i32) {
        buff[i] = (num % 10) as u8;
        num /= 10;
    }
    let mut i: i32 = length as i32 - 1;
    while i >= 0 {
        LinkTest_PrintNumChar(buff[i], x, y);
        x += 1;
        i -= 1;
    }
    if negX != -1 {
        LinkTest_PrintNumChar(*b"\x0a\0".as_ptr().cast_mut(), negX as u8, y);
    }
}
unsafe fn LinkTest_PrintString(mut str: *mut u8, x: u8, y: u8) {
    let mut yOffset: i32 = 0;
    let mut xOffset: i32 = 0;
    let i: i32 = 0;
    while *str.at(i) != 0 {
        if *str.at(i) == *b"\x0a\0".as_ptr().cast_mut() {
            yOffset += 1;
            xOffset = 0;
        } else {
            LinkTest_PrintChar(*str.at(i), x + xOffset as u8, y + yOffset as u8);
            xOffset += 1;
        }
        str = str.at(1);
    }
}
pub(crate) fn LinkCB_RequestPlayerDataExchange() {
    if gLinkStatus.get() & LINK_STAT_MASTER != 0 {
        BuildSendCmd(LINKCMD_SEND_LINK_TYPE);
    }
    crate::gl!(gLinkCallback = None);
}
pub(crate) unsafe fn Task_PrintTestData(taskId: u8) {
    let mut testTitle: CArray<u8, 32> = zeroed();
    strcpy(testTitle.as_mut_ptr(), sASCIITestPrint.as_ptr().cast_mut());
    LinkTest_PrintString(testTitle.as_mut_ptr(), 5, 2);
    LinkTest_PrintHex(gShouldAdvanceLinkState.get() as u32, 2, 1, 2);
    LinkTest_PrintHex(gLinkStatus.get(), 15, 1, 8);
    LinkTest_PrintHex(crate::gl!(gLink.state) as u32, 2, 10, 2);
    LinkTest_PrintHex((gLinkStatus.get() & 0x0000001C) >> 2, 15, 10, 2);
    LinkTest_PrintHex(GetMultiplayerId() as u32, 15, 12, 2);
    LinkTest_PrintHex(gLastSendQueueCount.get() as u32, 25, 1, 2);
    LinkTest_PrintHex(gLastRecvQueueCount.get() as u32, 25, 2, 2);
    LinkTest_PrintHex(GetBlockReceivedStatus() as u32, 15, 5, 2);
    LinkTest_PrintHex(gLinkDebugSeed.get(), 2, 12, 8);
    LinkTest_PrintHex(gLinkDebugFlags.get(), 2, 13, 8);
    LinkTest_PrintHex(GetSioMultiSI() as u32, 25, 5, 1);
    LinkTest_PrintHex(IsSioMultiMaster() as u32, 25, 6, 1);
    LinkTest_PrintHex(IsLinkConnectionEstablished() as u32, 25, 7, 1);
    LinkTest_PrintHex(HasLinkErrorOccurred() as u32, 25, 8, 1);
    for i in 0..MAX_LINK_PLAYERS {
        LinkTest_PrintHex(
            crate::gl!(gLinkTestBlockChecksums[i]) as u32,
            10,
            4 + i as u8,
            4,
        );
    }
}
pub fn SetLinkDebugValues(seed: u32, flags: u32) {
    gLinkDebugSeed.set(seed);
    gLinkDebugFlags.set(flags);
}
pub fn GetSavedLinkPlayerCountAsBitFlags() -> u8 {
    let mut flags: u8 = 0;
    let mut i: i32 = 0;
    while i < gSavedLinkPlayerCount.get() as i32 {
        flags |= shl_i32(1, i as u32) as u8;
        i += 1;
    }
    flags
}
pub unsafe fn GetLinkPlayerCountAsBitFlags() -> u8 {
    let mut flags: u8 = 0;
    let mut i: i32 = 0;
    while i < GetLinkPlayerCount() as i32 {
        flags |= shl_i32(1, i as u32) as u8;
        i += 1;
    }
    flags
}
pub unsafe fn SaveLinkPlayers(playerCount: u8) {
    gSavedLinkPlayerCount.set(playerCount);
    gSavedMultiplayerId.set(GetMultiplayerId());
    for i in 0..MAX_RFU_PLAYERS {
        crate::gl!(sSavedLinkPlayers[i] = crate::gl!(gLinkPlayers[i]));
    }
}
pub fn GetSavedPlayerCount() -> u8 {
    gSavedLinkPlayerCount.get()
}
fn GetSavedMultiplayerId() -> u8 {
    gSavedMultiplayerId.get()
}
pub unsafe fn DoesLinkPlayerCountMatchSaved() -> u8 {
    let mut count: u32 = 0;
    for i in 0..(gSavedLinkPlayerCount.get() as i32) {
        if crate::gl!(gLinkPlayers[i].trainerId) == crate::gl!(sSavedLinkPlayers[i].trainerId) {
            if gLinkType.get() == LINKTYPE_BATTLE_TOWER {
                if gLinkType.get() as u32 == crate::gl!(gLinkPlayers[i].linkType) {
                    count += 1;
                }
            } else {
                count += 1;
            }
        }
    }
    if count == gSavedLinkPlayerCount.get() as u32
        && GetLinkPlayerCount_2() == gSavedLinkPlayerCount.get()
    {
        return TRUE;
    }
    FALSE
}
pub unsafe fn ClearSavedLinkPlayers() {
    memset(
        (*sSavedLinkPlayers.as_ptr()).as_mut_ptr() as *mut u8,
        0,
        140,
    );
}
pub unsafe fn CheckLinkPlayersMatchSaved() {
    let mut i: u8 = 0;
    while i < gSavedLinkPlayerCount.get() {
        if crate::gl!(sSavedLinkPlayers[i].trainerId) != crate::gl!(gLinkPlayers[i].trainerId)
            || StringCompare(
                (*sSavedLinkPlayers.as_ptr())[i].name.as_mut_ptr(),
                (*gLinkPlayers.as_ptr())[i].name.as_mut_ptr(),
            ) != 0
        {
            gLinkErrorOccurred.set(TRUE);
            CloseLink();
            SetMainCallback2(Some(CB2_LinkError));
        }
        i += 1;
    }
}
pub fn ResetLinkPlayerCount() {
    gSavedLinkPlayerCount.set(0);
    gSavedMultiplayerId.set(0);
}
#[unsafe(no_mangle)]
pub unsafe fn GetLinkPlayerCount_2() -> u8 {
    ((gLinkStatus.get() & 0x0000001C) >> 2) as u8
}
#[unsafe(no_mangle)]
pub unsafe fn IsLinkMaster() -> u8 {
    if gWirelessCommType.get() != 0 {
        return Rfu_IsMaster();
    }
    (gLinkStatus.get() >> 5) as u8 & 1
}
fn GetDummy2() -> u8 {
    sDummy2.get()
}
pub unsafe fn SetCloseLinkCallbackAndType(r#type: u16) {
    if gWirelessCommType.get() == TRUE {
        Rfu_SetCloseLinkCallback();
    } else {
        if (*gLinkCallback.as_ptr()).is_none() {
            crate::gl!(gLinkCallback = Some(LinkCB_ReadyCloseLink));
            gLinkDummy1.set(FALSE);
            gReadyCloseLinkType.set(r#type);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe fn SetCloseLinkCallback() {
    if gWirelessCommType.get() == TRUE {
        Rfu_SetCloseLinkCallback();
    } else {
        if (*gLinkCallback.as_ptr()).is_some() {
            sReadyCloseLinkAttempts.set(sReadyCloseLinkAttempts.get() + 1);
        } else {
            crate::gl!(gLinkCallback = Some(LinkCB_ReadyCloseLink));
            gLinkDummy1.set(FALSE);
            gReadyCloseLinkType.set(0);
        }
    }
}
pub(crate) fn LinkCB_ReadyCloseLink() {
    if gLastRecvQueueCount.get() == 0 {
        BuildSendCmd(LINKCMD_READY_CLOSE_LINK);
        crate::gl!(gLinkCallback = Some(LinkCB_WaitCloseLink));
    }
}
pub(crate) unsafe fn LinkCB_WaitCloseLink() {
    let linkPlayerCount: u8 = GetLinkPlayerCount();
    let mut count: u32 = 0;
    for i in 0..(linkPlayerCount as i32) {
        if crate::gl!(gReadyToCloseLink[i]) != 0 {
            count += 1;
        }
    }
    if count == linkPlayerCount as u32 {
        gBattleTypeFlags.set(gBattleTypeFlags.get() & (0xffffffdf));
        gLinkVSyncDisabled.set(TRUE);
        CloseLink();
        crate::gl!(gLinkCallback = None);
        gLinkDummy1.set(TRUE);
    }
}
pub unsafe fn SetCloseLinkCallbackHandleJP() {
    if gWirelessCommType.get() == TRUE {
        Rfu_SetCloseLinkCallback();
    } else {
        if (*gLinkCallback.as_ptr()).is_some() {
            sReadyCloseLinkAttempts.set(sReadyCloseLinkAttempts.get() + 1);
        } else {
            crate::gl!(gLinkCallback = Some(LinkCB_ReadyCloseLinkWithJP));
            gLinkDummy1.set(FALSE);
            gReadyCloseLinkType.set(0);
        }
    }
}
pub(crate) fn LinkCB_ReadyCloseLinkWithJP() {
    if gLastRecvQueueCount.get() == 0 {
        BuildSendCmd(LINKCMD_READY_CLOSE_LINK);
        crate::gl!(gLinkCallback = Some(LinkCB_WaitCloseLinkWithJP));
    }
}
pub(crate) unsafe fn LinkCB_WaitCloseLinkWithJP() {
    let linkPlayerCount: u8 = GetLinkPlayerCount();
    let mut count: u32 = 0;
    for i in 0..(linkPlayerCount as i32) {
        if crate::gl!(gLinkPlayers[i].language) == LANGUAGE_JAPANESE as u16 {
            count += 1;
        } else if crate::gl!(gReadyToCloseLink[i]) != 0 {
            count += 1;
        }
    }
    if count == linkPlayerCount as u32 {
        gBattleTypeFlags.set(gBattleTypeFlags.get() & (0xffffffdf));
        gLinkVSyncDisabled.set(TRUE);
        CloseLink();
        crate::gl!(gLinkCallback = None);
        gLinkDummy1.set(TRUE);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn SetLinkStandbyCallback() {
    if gWirelessCommType.get() == TRUE {
        Rfu_SetLinkStandbyCallback();
    } else {
        if (*gLinkCallback.as_ptr()).is_none() {
            crate::gl!(gLinkCallback = Some(LinkCB_Standby));
        }
        gLinkDummy1.set(FALSE);
    }
}
pub(crate) fn LinkCB_Standby() {
    if gLastRecvQueueCount.get() == 0 {
        BuildSendCmd(LINKCMD_READY_EXIT_STANDBY);
        crate::gl!(gLinkCallback = Some(LinkCB_StandbyForAll));
    }
}
pub(crate) unsafe fn LinkCB_StandbyForAll() {
    let linkPlayerCount: u8 = GetLinkPlayerCount();
    let mut i: u8 = 0;
    while i < linkPlayerCount {
        if crate::gl!(gReadyToExitStandby[i]) == 0 {
            break;
        }
        i += 1;
    }
    if i == linkPlayerCount {
        for i in 0..(MAX_LINK_PLAYERS as u8) {
            crate::gl!(gReadyToExitStandby[i] = FALSE);
        }
        crate::gl!(gLinkCallback = None);
    }
}
unsafe fn TrySetLinkErrorBuffer() {
    if sLinkOpen.get() != 0 && (gLinkStatus.get() & 0x0007F000) >> 12 != 0 {
        if gSuppressLinkErrorMessage.get() == 0 {
            crate::gl!(sLinkErrorBuffer.status = gLinkStatus.get());
            crate::gl!(sLinkErrorBuffer.lastRecvQueueCount = gLastRecvQueueCount.get());
            crate::gl!(sLinkErrorBuffer.lastSendQueueCount = gLastSendQueueCount.get());
            SetMainCallback2(Some(CB2_LinkError));
        }
        gLinkErrorOccurred.set(TRUE);
        CloseLink();
    }
}
pub fn SetLinkErrorBuffer(
    status: u32,
    lastSendQueueCount: u8,
    lastRecvQueueCount: u8,
    disconnected: u8,
) {
    crate::gl!(sLinkErrorBuffer.status = status);
    crate::gl!(sLinkErrorBuffer.lastSendQueueCount = lastSendQueueCount);
    crate::gl!(sLinkErrorBuffer.lastRecvQueueCount = lastRecvQueueCount);
    crate::gl!(sLinkErrorBuffer.disconnected = disconnected);
}
pub unsafe fn CB2_LinkError() {
    let mut tilemapBuffer: *mut u8 = null_mut();
    SetGpuReg(0x0, 0);
    m4aMPlayStop(crate::gl!(&raw gMPlayInfo_SE1));
    m4aMPlayStop(crate::gl!(&raw gMPlayInfo_SE2));
    m4aMPlayStop(crate::gl!(&raw gMPlayInfo_SE3));
    InitHeap(
        (*(crate::gl!(&raw @at crate::malloc::gHeap;).cast_const())
            .cast::<CArray<u8, 114688>>()
            .cast_mut())
        .as_mut_ptr() as *mut c_void,
        HEAP_SIZE,
    );
    ResetSpriteData();
    FreeAllSpritePalettes();
    ResetPaletteFadeControl();
    SetBackdropFromColor(0);
    ResetTasks();
    ScanlineEffect_Stop();
    if gWirelessCommType.get() != 0 {
        if crate::gl!(sLinkErrorBuffer.disconnected) == 0 {
            gWirelessCommType.set(3);
        }
        ResetLinkRfuGFLayer();
    }
    SetVBlankCallback(Some(VBlankCB_LinkError));
    ResetBgsAndClearDma3BusyFlags(0);
    InitBgsFromTemplates(0, sLinkErrorBgTemplates.as_ptr().cast_mut(), 2);
    crate::gl!(
        sLinkErrorBgTilemapBuffer = ({
            tilemapBuffer = Alloc(BG_SCREEN_SIZE) as *mut u8;
            tilemapBuffer
        }) as *mut c_void
    );
    SetBgTilemapBuffer(1, tilemapBuffer as *mut c_void);
    if InitWindows(sLinkErrorWindowTemplates.as_ptr().cast_mut()) != 0 {
        DeactivateAllTextPrinters();
        ResetTempTileDataBuffers();
        SetGpuReg(REG_OFFSET_BLDCNT, 0);
        SetGpuReg(REG_OFFSET_BLDALPHA, 0);
        SetGpuReg(REG_OFFSET_BG0HOFS, 0);
        SetGpuReg(REG_OFFSET_BG0VOFS, 0);
        SetGpuReg(REG_OFFSET_BG1HOFS, 0);
        SetGpuReg(REG_OFFSET_BG1VOFS, 0);
        ClearGpuRegBits(REG_OFFSET_DISPCNT, 57344);
        LoadPalette(
            (*(&raw const crate::data::menu::gStandardMenuPalette).cast::<CArray<u16, 0>>())
                .as_ptr()
                .cast_mut() as *mut c_void,
            240,
            32,
        );
        gSoftResetDisabled.set(FALSE);
        CreateTask(Some(Task_DestroySelf), 0);
        StopMapMusic();
        crate::gl!(gMain.callback1 = None);
        RunTasks();
        AnimateSprites();
        BuildOamBuffer();
        UpdatePaletteFade();
        SetMainCallback2(Some(CB2_PrintErrorMessage));
    }
}
unsafe fn ErrorMsg_MoveCloserToPartner() {
    LoadBgTiles(
        0,
        sCommErrorBg_Gfx.as_ptr().cast_mut() as *mut c_void,
        0x20,
        0,
    );
    DecompressAndLoadBgGfxUsingHeap(
        1,
        sWirelessLinkDisplayGfx.as_ptr().cast_mut() as *mut c_void,
        0,
        0,
        0,
    );
    CopyToBgTilemapBuffer(
        1,
        sWirelessLinkDisplayTilemap.as_ptr().cast_mut() as *mut c_void,
        0,
        0,
    );
    CopyBgTilemapBufferToVram(1);
    LoadPalette(
        sWirelessLinkDisplayPal.as_ptr().cast_mut() as *mut c_void,
        0,
        32,
    );
    FillWindowPixelBuffer(WIN_LINK_ERROR_TOP, 0);
    FillWindowPixelBuffer(WIN_LINK_ERROR_BOTTOM, 0);
    AddTextPrinterParameterized3(
        WIN_LINK_ERROR_TOP,
        FONT_SHORT_COPY_1,
        2,
        6,
        sTextColors.as_ptr().cast_mut(),
        0,
        (*(&raw const crate::data::strings::gText_CommErrorEllipsis).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
    );
    AddTextPrinterParameterized3(
        WIN_LINK_ERROR_BOTTOM,
        FONT_SHORT_COPY_1,
        2,
        1,
        sTextColors.as_ptr().cast_mut(),
        0,
        (*(&raw const crate::data::strings::gText_MoveCloserToLinkPartner).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
    );
    PutWindowTilemap(WIN_LINK_ERROR_TOP);
    PutWindowTilemap(WIN_LINK_ERROR_BOTTOM);
    CopyWindowToVram(WIN_LINK_ERROR_TOP, COPYWIN_NONE);
    CopyWindowToVram(WIN_LINK_ERROR_BOTTOM, COPYWIN_FULL);
}
unsafe fn ErrorMsg_CheckConnections() {
    LoadBgTiles(
        0,
        sCommErrorBg_Gfx.as_ptr().cast_mut() as *mut c_void,
        0x20,
        0,
    );
    FillWindowPixelBuffer(WIN_LINK_ERROR_MID, 0);
    FillWindowPixelBuffer(WIN_LINK_ERROR_BOTTOM, 0);
    AddTextPrinterParameterized3(
        WIN_LINK_ERROR_MID,
        FONT_SHORT_COPY_1,
        2,
        0,
        sTextColors.as_ptr().cast_mut(),
        0,
        (*(&raw const crate::data::strings::gText_CommErrorCheckConnections)
            .cast::<CArray<u8, 0>>())
        .as_ptr()
        .cast_mut(),
    );
    PutWindowTilemap(WIN_LINK_ERROR_MID);
    PutWindowTilemap(WIN_LINK_ERROR_BOTTOM);
    CopyWindowToVram(WIN_LINK_ERROR_MID, COPYWIN_NONE);
    CopyWindowToVram(WIN_LINK_ERROR_BOTTOM, COPYWIN_FULL);
}
pub(crate) unsafe fn CB2_PrintErrorMessage() {
    match crate::gl!(gMain.state) {
        0 => {
            if crate::gl!(sLinkErrorBuffer.disconnected) != 0 {
                ErrorMsg_MoveCloserToPartner();
            } else {
                ErrorMsg_CheckConnections();
            }
        }
        2 => {
            ShowBg(0);
            if crate::gl!(sLinkErrorBuffer.disconnected) != 0 {
                ShowBg(1);
            }
        }
        30 => {
            PlaySE(SE_BOO);
        }
        60 => {
            PlaySE(SE_BOO);
        }
        90 => {
            PlaySE(SE_BOO);
        }
        130 => {
            if gWirelessCommType.get() == 2 {
                AddTextPrinterParameterized3(
                    WIN_LINK_ERROR_TOP,
                    FONT_SHORT_COPY_1,
                    2,
                    20,
                    sTextColors.as_ptr().cast_mut(),
                    0,
                    (*(&raw const crate::data::strings::gText_ABtnTitleScreen)
                        .cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut(),
                );
            } else if gWirelessCommType.get() == 1 {
                AddTextPrinterParameterized3(
                    WIN_LINK_ERROR_TOP,
                    FONT_SHORT_COPY_1,
                    2,
                    20,
                    sTextColors.as_ptr().cast_mut(),
                    0,
                    (*(&raw const crate::data::strings::gText_ABtnRegistrationCounter)
                        .cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut(),
                );
            }
        }
        _ => {}
    }
    if crate::gl!(gMain.state) == 160 {
        if gWirelessCommType.get() == 1 {
            if crate::gl!(gMain.newKeys) as i32 & A_BUTTON != 0 {
                PlaySE(SE_PIN);
                gWirelessCommType.set(0);
                crate::gl!(sLinkErrorBuffer.disconnected = FALSE);
                ReloadSave();
            }
        } else if gWirelessCommType.get() == 2 && crate::gl!(gMain.newKeys) as i32 & A_BUTTON != 0 {
            rfu_REQ_stopMode();
            rfu_waitREQComplete();
            DoSoftReset();
        }
    }
    if crate::gl!(gMain.state) != 160 {
        crate::gl!(gMain.state += 1);
    }
}
pub unsafe fn GetSioMultiSI() -> u8 {
    ((67109160_usize as *mut u16).read_volatile() as i32 & SIO_MULTI_SI != 0) as u8
}
unsafe fn IsSioMultiMaster() -> u8 {
    ((67109160_usize as *mut u16).read_volatile() as i32 & SIO_MULTI_SD != 0
        && (67109160_usize as *mut u16).read_volatile() as i32 & SIO_MULTI_SI == 0) as u8
}
#[unsafe(no_mangle)]
pub fn IsLinkConnectionEstablished() -> u8 {
    (gLinkStatus.get() >> 6) as u8 & 1
}
pub fn SetSuppressLinkErrorMessage(flag: u8) {
    gSuppressLinkErrorMessage.set(flag);
}
pub fn HasLinkErrorOccurred() -> u8 {
    gLinkErrorOccurred.get()
}
pub unsafe fn LocalLinkPlayerToBlock() {
    InitLocalLinkPlayer();
    let block: *mut LinkPlayerBlock = crate::gl!(&raw gLocalLinkPlayerBlock);
    (*block).linkPlayer = crate::gl!(gLocalLinkPlayer);
    memcpy(
        (*block).magic1.as_mut_ptr(),
        sASCIIGameFreakInc.as_ptr().cast_mut(),
        15,
    );
    memcpy(
        (*block).magic2.as_mut_ptr(),
        sASCIIGameFreakInc.as_ptr().cast_mut(),
        15,
    );
    memcpy(
        (*gBlockSendBuffer.as_ptr()).as_mut_ptr(),
        block as *mut u8,
        60,
    );
}
pub unsafe fn LinkPlayerFromBlock(who: u32) {
    let who_: u8 = who as u8;
    let block: *mut LinkPlayerBlock =
        (*gBlockRecvBuffer.as_ptr())[who_].as_mut_ptr() as *mut LinkPlayerBlock;
    let player: *mut LinkPlayer = crate::gl!(&raw gLinkPlayers[who_]);
    *player = (*block).linkPlayer;
    ConvertLinkPlayerName(player);
    if strcmp(
        (*block).magic1.as_mut_ptr(),
        sASCIIGameFreakInc.as_ptr().cast_mut(),
    ) != 0
        || strcmp(
            (*block).magic2.as_mut_ptr(),
            sASCIIGameFreakInc.as_ptr().cast_mut(),
        ) != 0
    {
        SetMainCallback2(Some(CB2_LinkError));
    }
}
#[unsafe(no_mangle)]
pub unsafe fn HandleLinkConnection() -> u8 {
    let mut main1Failed: u32 = 0;
    let mut main2Failed: u32 = 0;
    if gWirelessCommType.get() == 0 {
        gLinkStatus.set(LinkMain1(
            gShouldAdvanceLinkState.as_ptr(),
            (*gSendCmd.as_ptr()).as_mut_ptr(),
            (*gRecvCmds.as_ptr()).as_mut_ptr(),
        ));
        LinkMain2(crate::gl!(&raw gMain.heldKeys));
        if gLinkStatus.get() & LINK_STAT_RECEIVED_NOTHING != 0
            && IsSendingKeysOverCable() == TRUE as u32
        {
            return TRUE;
        }
    } else {
        main1Failed = RfuMain1();
        main2Failed = RfuMain2();
        if IsSendingKeysOverCable() == TRUE as u32
            && (main1Failed == TRUE as u32 || IsRfuRecvQueueEmpty() != 0 || main2Failed != 0)
        {
            return TRUE;
        }
    }
    FALSE
}
pub fn SetWirelessCommType1() {
    if gReceivedRemoteLinkPlayers.get() == 0 {
        gWirelessCommType.set(1);
    }
}
fn SetWirelessCommType0_Internal() {
    if gReceivedRemoteLinkPlayers.get() == 0 {
        gWirelessCommType.set(0);
    }
}
pub fn SetWirelessCommType0() {
    if gReceivedRemoteLinkPlayers.get() == 0 {
        gWirelessCommType.set(0);
    }
}
pub unsafe fn GetLinkRecvQueueLength() -> u32 {
    if gWirelessCommType.get() != 0 {
        return GetRfuRecvQueueLength();
    }
    crate::gl!(gLink.recvQueue.count) as u32
}
pub unsafe fn IsLinkRecvQueueAtOverworldMax() -> u32 {
    if GetLinkRecvQueueLength() >= OVERWORLD_RECV_QUEUE_MAX {
        return TRUE as u32;
    }
    FALSE as u32
}
#[unsafe(no_mangle)]
pub fn GetWirelessCommType() -> u8 {
    gWirelessCommType.get()
}
pub unsafe fn ConvertLinkPlayerName(player: *mut LinkPlayer) {
    (*player).progressFlagsCopy = (*player).progressFlags;
    ConvertInternationalString((*player).name.as_mut_ptr(), (*player).language as u8);
}
unsafe fn DisableSerial() {
    DisableInterrupts(192);
    volatile_write(67109160_usize as *mut u16, SIO_MULTI_MODE);
    volatile_write(67109134_usize as *mut u16, 0);
    volatile_write(67109378_usize as *mut u16, 192);
    volatile_write(67109162_usize as *mut u16, 0);
    volatile_write(67109152_usize as *mut u64, 0);
    {
        {
            let mut tmp: u32 = 0;
            volatile_write(&raw mut tmp, 0);
            CpuSet(
                &raw mut tmp as *mut c_void,
                crate::gl!(&raw gLink) as *mut c_void,
                0x50003f0,
            );
        }
    }
}
unsafe fn EnableSerial() {
    DisableInterrupts(192);
    volatile_write(67109172_usize as *mut u16, 0);
    volatile_write(67109160_usize as *mut u16, SIO_MULTI_MODE);
    volatile_write(
        67109160_usize as *mut u16,
        (67109160_usize as *mut u16).read_volatile() | 16387,
    );
    EnableInterrupts(INTR_FLAG_SERIAL);
    volatile_write(67109162_usize as *mut u16, 0);
    {
        {
            let mut tmp: u32 = 0;
            volatile_write(&raw mut tmp, 0);
            CpuSet(
                &raw mut tmp as *mut c_void,
                crate::gl!(&raw gLink) as *mut c_void,
                0x50003f0,
            );
        }
    }
    sNumVBlanksWithoutSerialIntr.set(0);
    sSendNonzeroCheck.set(0);
    sRecvNonzeroCheck.set(0);
    sChecksumAvailable.set(0);
    sHandshakePlayerCount.set(0);
    gLastSendQueueCount.set(0);
    gLastRecvQueueCount.set(0);
}
pub unsafe fn ResetSerial() {
    EnableSerial();
    DisableSerial();
}
pub unsafe fn LinkMain1(
    shouldAdvanceLinkState: *mut u8,
    sendCmd: *mut u16,
    recvCmds: *mut CArray<u16, 8>,
) -> u32 {
    'l1: {
        let sw1: u8 = crate::gl!(gLink.state);
        let mut fall = false;
        if sw1 == LINK_STATE_START0 {
            DisableSerial();
            crate::gl!(gLink.state = 1);
            break 'l1;
        }
        if sw1 == LINK_STATE_START1 {
            if *shouldAdvanceLinkState == 1 {
                EnableSerial();
                crate::gl!(gLink.state = 2);
            }
            break 'l1;
        }
        if sw1 == LINK_STATE_HANDSHAKE {
            match *shouldAdvanceLinkState {
                1 => {
                    if crate::gl!(gLink.isMaster) == LINK_MASTER
                        && crate::gl!(gLink.playerCount) > 1
                    {
                        crate::gl!(gLink.handshakeAsMaster = TRUE);
                    }
                }
                2 => {
                    crate::gl!(gLink.state = LINK_STATE_START0);
                    volatile_write(67109162_usize as *mut u16, 0);
                }
                _ => {
                    CheckMasterOrSlave();
                }
            }
            break 'l1;
        }
        if sw1 == LINK_STATE_INIT_TIMER {
            fall = true;
            InitTimer();
            crate::gl!(gLink.state = LINK_STATE_CONN_ESTABLISHED);
        }
        if fall || sw1 == LINK_STATE_CONN_ESTABLISHED {
            EnqueueSendCmd(sendCmd);
            DequeueRecvCmds(recvCmds);
            break 'l1;
        }
    }
    *shouldAdvanceLinkState = 0;
    let mut retVal: u32 = crate::gl!(gLink.localId) as u32;
    retVal |= (crate::gl!(gLink.playerCount) as u32) << 2;
    if crate::gl!(gLink.isMaster) == LINK_MASTER {
        retVal |= LINK_STAT_MASTER;
    }
    {
        let receivedNothing: u32 = (crate::gl!(gLink.receivedNothing) as u32) << 8;
        let link_field_F: u32 = (crate::gl!(gLink.link_field_F) as u32) << 9;
        let hardwareError: u32 = (crate::gl!(gLink.hardwareError) as u32) << 12;
        let badChecksum: u32 = (crate::gl!(gLink.badChecksum) as u32) << 13;
        let queueFull: u32 = (crate::gl!(gLink.queueFull) as u32) << 14;
        let mut val: u32 = 0;
        if crate::gl!(gLink.state) == LINK_STATE_CONN_ESTABLISHED {
            val = LINK_STAT_CONN_ESTABLISHED;
            val |= receivedNothing;
            val |= retVal;
            val |= link_field_F;
            val |= hardwareError;
            val |= badChecksum;
            val |= queueFull;
        } else {
            val = retVal;
            val |= receivedNothing;
            val |= link_field_F;
            val |= hardwareError;
            val |= badChecksum;
            val |= queueFull;
        }
        retVal = val;
    }
    if crate::gl!(gLink.lag) == LAG_MASTER {
        retVal |= LINK_STAT_ERROR_LAG_MASTER;
    }
    if crate::gl!(gLink.localId) >= MAX_LINK_PLAYERS as u8 {
        retVal |= LINK_STAT_ERROR_INVALID_ID;
    }
    let mut retVal2: u32 = retVal;
    if crate::gl!(gLink.lag) == LAG_SLAVE {
        retVal2 |= LINK_STAT_ERROR_LAG_SLAVE;
    }
    retVal2
}
unsafe fn CheckMasterOrSlave() {
    let terminals: u32 = (REG_ADDR_SIOCNT as usize as *mut u32).read_volatile() & 12;
    if terminals == SIO_MULTI_SD as u32 && crate::gl!(gLink.localId) == 0 {
        crate::gl!(gLink.isMaster = LINK_MASTER);
    } else {
        crate::gl!(gLink.isMaster = LINK_SLAVE);
    }
}
unsafe fn InitTimer() {
    if crate::gl!(gLink.isMaster) != 0 {
        volatile_write(67109132_usize as *mut u16, 65339);
        volatile_write(67109134_usize as *mut u16, 65);
        EnableInterrupts(INTR_FLAG_TIMER3);
    }
}
unsafe fn EnqueueSendCmd(mut sendCmd: *mut u16) {
    let mut offset: u8 = 0;
    gLinkSavedIme.set((67109384_usize as *mut u16).read_volatile());
    volatile_write(67109384_usize as *mut u16, 0);
    if crate::gl!(gLink.sendQueue.count) < QUEUE_CAPACITY {
        offset = crate::gl!(gLink.sendQueue.pos) + crate::gl!(gLink.sendQueue.count);
        if offset >= QUEUE_CAPACITY {
            offset -= QUEUE_CAPACITY;
        }
        for i in 0..CMD_LENGTH {
            sSendNonzeroCheck.set(sSendNonzeroCheck.get() | (*sendCmd));
            crate::gl!(gLink.sendQueue.data[i][offset] = *sendCmd);
            *sendCmd = 0;
            sendCmd = sendCmd.at(1);
        }
    } else {
        crate::gl!(gLink.queueFull = QUEUE_FULL_SEND);
    }
    if sSendNonzeroCheck.get() != 0 {
        crate::gl!(gLink.sendQueue.count += 1);
        sSendNonzeroCheck.set(0);
    }
    volatile_write(67109384_usize as *mut u16, gLinkSavedIme.get());
    gLastSendQueueCount.set(crate::gl!(gLink.sendQueue.count));
}
unsafe fn DequeueRecvCmds(recvCmds: *mut CArray<u16, 8>) {
    gLinkSavedIme.set((67109384_usize as *mut u16).read_volatile());
    volatile_write(67109384_usize as *mut u16, 0);
    if crate::gl!(gLink.recvQueue.count) == 0 {
        for i in 0..crate::gl!(gLink.playerCount) {
            for j in 0..CMD_LENGTH {
                (*recvCmds.at(i))[j] = 0;
            }
        }
        crate::gl!(gLink.receivedNothing = TRUE);
    } else {
        for i in 0..crate::gl!(gLink.playerCount) {
            for j in 0..CMD_LENGTH {
                (*recvCmds.at(i))[j] =
                    (*gLink.as_ptr()).recvQueue.data[i][j][crate::gl!(gLink.recvQueue.pos)];
            }
        }
        crate::gl!(gLink.recvQueue.count -= 1);
        crate::gl!(gLink.recvQueue.pos += 1);
        if crate::gl!(gLink.recvQueue.pos) >= QUEUE_CAPACITY {
            crate::gl!(gLink.recvQueue.pos = 0);
        }
        crate::gl!(gLink.receivedNothing = FALSE);
    }
    volatile_write(67109384_usize as *mut u16, gLinkSavedIme.get());
}
#[unsafe(no_mangle)]
pub unsafe fn LinkVSync() {
    if crate::gl!(gLink.isMaster) != 0 {
        match crate::gl!(gLink.state) {
            LINK_STATE_CONN_ESTABLISHED => {
                if crate::gl!(gLink.serialIntrCounter) < 9 {
                    if crate::gl!(gLink.hardwareError) != TRUE {
                        crate::gl!(gLink.lag = LAG_MASTER);
                    } else {
                        StartTransfer();
                    }
                } else if crate::gl!(gLink.lag) != LAG_MASTER {
                    crate::gl!(gLink.serialIntrCounter = 0);
                    StartTransfer();
                }
            }
            LINK_STATE_HANDSHAKE => {
                StartTransfer();
            }
            _ => {}
        }
    } else if (crate::gl!(gLink.state) == LINK_STATE_CONN_ESTABLISHED
        || crate::gl!(gLink.state) == LINK_STATE_HANDSHAKE)
        && ({
            sNumVBlanksWithoutSerialIntr.set(sNumVBlanksWithoutSerialIntr.get() + 1);
            sNumVBlanksWithoutSerialIntr.get()
        }) > 10
    {
        if crate::gl!(gLink.state) == LINK_STATE_CONN_ESTABLISHED {
            crate::gl!(gLink.lag = LAG_SLAVE);
        }
        if crate::gl!(gLink.state) == LINK_STATE_HANDSHAKE {
            crate::gl!(gLink.playerCount = 0);
            crate::gl!(gLink.link_field_F = FALSE);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Timer3Intr() {
    StopTimer();
    StartTransfer();
}
pub unsafe fn SerialCB() {
    crate::gl!(gLink.localId = (*(67109160_usize as *mut SioMultiCnt)).id() as u8);
    match crate::gl!(gLink.state) {
        LINK_STATE_CONN_ESTABLISHED => {
            crate::gl!(gLink.hardwareError = (*(67109160_usize as *mut SioMultiCnt)).error() as u8);
            DoRecv();
            DoSend();
            SendRecvDone();
        }
        LINK_STATE_HANDSHAKE if DoHandshake() != 0 => {
            if crate::gl!(gLink.isMaster) != 0 {
                crate::gl!(gLink.state = LINK_STATE_INIT_TIMER);
                crate::gl!(gLink.serialIntrCounter = 8);
            } else {
                crate::gl!(gLink.state = LINK_STATE_CONN_ESTABLISHED);
            }
        }
        _ => {}
    }
    crate::gl!(gLink.serialIntrCounter += 1);
    sNumVBlanksWithoutSerialIntr.set(0);
    if crate::gl!(gLink.serialIntrCounter) == 8 {
        gLastRecvQueueCount.set(crate::gl!(gLink.recvQueue.count));
    }
}
unsafe fn StartTransfer() {
    volatile_write(
        67109160_usize as *mut u16,
        (67109160_usize as *mut u16).read_volatile() | SIO_START,
    );
}
unsafe fn DoHandshake() -> u8 {
    let mut playerCount: u8 = 0;
    let mut minRecv: u16 = 0xFFFF;
    if crate::gl!(gLink.handshakeAsMaster) == TRUE {
        volatile_write(67109162_usize as *mut u16, MASTER_HANDSHAKE);
    } else {
        volatile_write(67109162_usize as *mut u16, SLAVE_HANDSHAKE);
    }
    let mut recvSiomlt: u64 = (67109152_usize as *mut u64).read_volatile();
    memcpy(
        (*gLink.as_ptr()).handshakeBuffer.as_mut_ptr() as *mut u8,
        &raw mut recvSiomlt as *mut u8,
        8,
    );
    volatile_write(67109152_usize as *mut u64, 0);
    crate::gl!(gLink.handshakeAsMaster = FALSE);
    for i in 0..(MAX_LINK_PLAYERS as u8) {
        if crate::gl!(gLink.handshakeBuffer[i]) as i32 & -4 == SLAVE_HANDSHAKE as i32
            || crate::gl!(gLink.handshakeBuffer[i]) == MASTER_HANDSHAKE
        {
            playerCount += 1;
            if minRecv > crate::gl!(gLink.handshakeBuffer[i])
                && crate::gl!(gLink.handshakeBuffer[i]) != 0
            {
                minRecv = crate::gl!(gLink.handshakeBuffer[i]);
            }
        } else {
            if crate::gl!(gLink.handshakeBuffer[i]) != 0xFFFF {
                playerCount = 0;
            }
            break;
        }
    }
    crate::gl!(gLink.playerCount = playerCount);
    if crate::gl!(gLink.playerCount) > 1
        && crate::gl!(gLink.playerCount) == sHandshakePlayerCount.get()
        && crate::gl!(gLink.handshakeBuffer[0]) == MASTER_HANDSHAKE
    {
        return TRUE;
    }
    if crate::gl!(gLink.playerCount) > 1 {
        crate::gl!(gLink.link_field_F = (minRecv as u8 & 3) + 1);
    } else {
        crate::gl!(gLink.link_field_F = 0);
    }
    sHandshakePlayerCount.set(crate::gl!(gLink.playerCount));
    FALSE
}
unsafe fn DoRecv() {
    let mut recv: CArray<u16, 4> = zeroed();
    let mut i: u8 = 0;
    let mut index: u8 = 0;
    let mut recvSiomlt: u64 = (67109152_usize as *mut u64).read_volatile();
    memcpy(
        recv.as_mut_ptr() as *mut u8,
        &raw mut recvSiomlt as *mut u8,
        8,
    );
    if crate::gl!(gLink.sendCmdIndex) == 0 {
        i = 0;
        while i < crate::gl!(gLink.playerCount) {
            if crate::gl!(gLink.checksum) != recv[i] && sChecksumAvailable.get() != 0 {
                crate::gl!(gLink.badChecksum = TRUE);
            }
            i += 1;
        }
        crate::gl!(gLink.checksum = 0);
        sChecksumAvailable.set(TRUE);
    } else {
        index = crate::gl!(gLink.recvQueue.pos) + crate::gl!(gLink.recvQueue.count);
        if index >= QUEUE_CAPACITY {
            index -= QUEUE_CAPACITY;
        }
        if crate::gl!(gLink.recvQueue.count) < QUEUE_CAPACITY {
            i = 0;
            while i < crate::gl!(gLink.playerCount) {
                crate::gl!(gLink.checksum += recv[i]);
                sRecvNonzeroCheck.set(sRecvNonzeroCheck.get() | (recv[i]));
                (*gLink.as_ptr()).recvQueue.data[i][crate::gl!(gLink.recvCmdIndex)][index] =
                    recv[i];
                i += 1;
            }
        } else {
            crate::gl!(gLink.queueFull = QUEUE_FULL_RECV);
        }
        crate::gl!(gLink.recvCmdIndex += 1);
        if crate::gl!(gLink.recvCmdIndex) == CMD_LENGTH && sRecvNonzeroCheck.get() != 0 {
            crate::gl!(gLink.recvQueue.count += 1);
            sRecvNonzeroCheck.set(0);
        }
    }
}
unsafe fn DoSend() {
    if crate::gl!(gLink.sendCmdIndex) == CMD_LENGTH {
        volatile_write(67109162_usize as *mut u16, crate::gl!(gLink.checksum));
        if sSendBufferEmpty.get() == 0 {
            crate::gl!(gLink.sendQueue.count -= 1);
            crate::gl!(gLink.sendQueue.pos += 1);
            if crate::gl!(gLink.sendQueue.pos) >= QUEUE_CAPACITY {
                crate::gl!(gLink.sendQueue.pos = 0);
            }
        } else {
            sSendBufferEmpty.set(FALSE);
        }
    } else {
        if sSendBufferEmpty.get() == 0 && crate::gl!(gLink.sendQueue.count) == 0 {
            sSendBufferEmpty.set(TRUE);
        }
        if sSendBufferEmpty.get() != 0 {
            volatile_write(67109162_usize as *mut u16, 0);
        } else {
            volatile_write(
                67109162_usize as *mut u16,
                (*gLink.as_ptr()).sendQueue.data[crate::gl!(gLink.sendCmdIndex)]
                    [crate::gl!(gLink.sendQueue.pos)],
            );
        }
        crate::gl!(gLink.sendCmdIndex += 1);
    }
}
unsafe fn StopTimer() {
    if crate::gl!(gLink.isMaster) != 0 {
        volatile_write(
            67109134_usize as *mut u16,
            (67109134_usize as *mut u16).read_volatile() & 65407,
        );
        volatile_write(67109132_usize as *mut u16, 65339);
    }
}
unsafe fn SendRecvDone() {
    if crate::gl!(gLink.recvCmdIndex) == CMD_LENGTH {
        crate::gl!(gLink.sendCmdIndex = 0);
        crate::gl!(gLink.recvCmdIndex = 0);
    } else if crate::gl!(gLink.isMaster) != 0 {
        volatile_write(
            67109134_usize as *mut u16,
            (67109134_usize as *mut u16).read_volatile() | TIMER_ENABLE,
        );
    }
}
pub fn ResetSendBuffer() {
    crate::gl!(gLink.sendQueue.count = 0);
    crate::gl!(gLink.sendQueue.pos = 0);
    for i in 0..CMD_LENGTH {
        for j in 0..QUEUE_CAPACITY {
            crate::gl!(gLink.sendQueue.data[i][j] = LINKCMD_NONE);
        }
    }
}
pub fn ResetRecvBuffer() {
    crate::gl!(gLink.recvQueue.count = 0);
    crate::gl!(gLink.recvQueue.pos = 0);
    for i in 0..(MAX_LINK_PLAYERS as u8) {
        for j in 0..CMD_LENGTH {
            for k in 0..QUEUE_CAPACITY {
                crate::gl!(gLink.recvQueue.data[i][j][k] = LINKCMD_NONE);
            }
        }
    }
}
pub(crate) unsafe fn SetBackdropFromColor(color: u16) {
    FillPalette(color, 0, 2);
}
