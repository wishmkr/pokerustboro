//! Translated from `src/link.c` by tools/rustport/c2rs.py.
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

pub(crate) static mut sBlockSend: BlockTransfer = unsafe { zeroed() };
pub(crate) static mut sBlockRecv: CArray<BlockTransfer, 4> = unsafe { zeroed() };
pub(crate) static mut sBlockSendDelayCounter: u32 = 0;
pub(crate) static mut sDummy1: u32 = 0;
pub(crate) static mut sDummy2: u8 = 0;
pub(crate) static mut sPlayerDataExchangeStatus: u32 = 0;
pub(crate) static mut sDummy3: u32 = 0;
pub(crate) static mut sLinkTestLastBlockSendPos: u8 = 0;
pub(crate) static mut sLinkTestLastBlockRecvPos: Aligned<CArray<u8, 4>> =
    Aligned(unsafe { zeroed() });
pub(crate) static mut sNumVBlanksWithoutSerialIntr: u8 = 0;
pub(crate) static mut sSendBufferEmpty: u8 = 0;
pub(crate) static mut sSendNonzeroCheck: u16 = 0;
pub(crate) static mut sRecvNonzeroCheck: u16 = 0;
pub(crate) static mut sChecksumAvailable: u8 = 0;
pub(crate) static mut sHandshakePlayerCount: u8 = 0;
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gLinkPartnersHeldKeys: Aligned<CArray<u16, 6>> = Aligned(unsafe { zeroed() });
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gLinkDebugSeed: u32 = 0;
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gLocalLinkPlayerBlock: LinkPlayerBlock = unsafe { zeroed() };
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gLinkErrorOccurred: u8 = 0;
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gLinkDebugFlags: u32 = 0;
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gLinkFiller1: u32 = 0;
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gRemoteLinkPlayersNotReceived: Aligned<CArray<u8, 4>> = Aligned(unsafe { zeroed() });
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gBlockReceivedStatus: Aligned<CArray<u8, 4>> = Aligned(unsafe { zeroed() });
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gLinkFiller2: u32 = 0;
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gLinkHeldKeys: u16 = 0;
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gRecvCmds: Aligned<CArray<CArray<u16, 8>, 5>> = Aligned(unsafe { zeroed() });
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gLinkStatus: u32 = 0;
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gLinkDummy1: u8 = 0;
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gLinkDummy2: u8 = 0;
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gReadyToExitStandby: Aligned<CArray<u8, 4>> = Aligned(unsafe { zeroed() });
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gReadyToCloseLink: Aligned<CArray<u8, 4>> = Aligned(unsafe { zeroed() });
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gReadyCloseLinkType: u16 = 0;
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gSuppressLinkErrorMessage: u8 = 0;
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gWirelessCommType: u8 = 0;
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gSavedLinkPlayerCount: u8 = 0;
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gSendCmd: Aligned<CArray<u16, 8>> = Aligned(unsafe { zeroed() });
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gSavedMultiplayerId: u8 = 0;
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gReceivedRemoteLinkPlayers: u8 = 0;
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gLinkTestBGInfo: LinkTestBGInfo = unsafe { zeroed() };
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gLinkCallback: Option<unsafe extern "C" fn()> = None;
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gShouldAdvanceLinkState: u8 = 0;
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gLinkTestBlockChecksums: Aligned<CArray<u16, 4>> = Aligned(unsafe { zeroed() });
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gBlockRequestType: u8 = 0;
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gLinkFiller3: u32 = 0;
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gLinkFiller4: u32 = 0;
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gLinkFiller5: u32 = 0;
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gLastSendQueueCount: u8 = 0;
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gLink: Link = unsafe { zeroed() };
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gLastRecvQueueCount: u8 = 0;
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gLinkSavedIme: u16 = 0;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sLinkTestDebugValuesEnabled: u8 = 0;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sDummyFlag: u8 = 0;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gBerryBlenderKeySendAttempts: u32 = 0;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gBlockRecvBuffer: Aligned<CArray<CArray<u16, 128>, 5>> =
    Aligned(unsafe { zeroed() });
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gBlockSendBuffer: Aligned<CArray<u8, 256>> = Aligned(unsafe { zeroed() });
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sLinkOpen: u8 = 0;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gLinkType: u16 = 0;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sTimeOutCounter: u16 = 0;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gLocalLinkPlayer: LinkPlayer = unsafe { zeroed() };
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gLinkPlayers: CArray<LinkPlayer, 5> = unsafe { zeroed() };
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sSavedLinkPlayers: CArray<LinkPlayer, 5> = unsafe { zeroed() };
pub(crate) static mut sLinkErrorBuffer: sLinkErrorBuffer_t = unsafe { zeroed() };
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sReadyCloseLinkAttempts: u16 = 0;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sLinkErrorBgTilemapBuffer: *mut c_void = null_mut();

unsafe extern "C" {
    static mut gBattleTypeFlags: u32;
    static mut gDecompressionBuffer: CArray<u8, 16384>;
    static gGameLanguage: u8;
    static gGameVersion: u8;
    static mut gHeap: CArray<u8, 114688>;
    static mut gHeldKeyCodeToSend: u16;
    static mut gLinkTransferringData: u8;
    static mut gLinkVSyncDisabled: u8;
    static mut gMPlayInfo_SE1: MusicPlayerInfo;
    static mut gMPlayInfo_SE2: MusicPlayerInfo;
    static mut gMPlayInfo_SE3: MusicPlayerInfo;
    static mut gMain: Main;
    static mut gSaveBlock2Ptr: *mut SaveBlock2;
    static mut gSoftResetDisabled: u8;
    static mut gSpecialVar_0x8005: u16;
    static mut gSpecialVar_ItemId: u16;
    static gStandardMenuPalette: CArray<u16, 0>;
    static mut gTasks: CArray<Task, 0>;
    static gText_ABtnRegistrationCounter: CArray<u8, 0>;
    static gText_ABtnTitleScreen: CArray<u8, 0>;
    static gText_CommErrorCheckConnections: CArray<u8, 0>;
    static gText_CommErrorEllipsis: CArray<u8, 0>;
    static gText_MoveCloserToLinkPartner: CArray<u8, 0>;
    fn AddTextPrinterParameterized3(
        a0: u8,
        a1: u8,
        a2: u8,
        a3: u8,
        a4: *mut u8,
        a5: i8,
        a6: *mut u8,
    );
    fn Alloc(a0: u32) -> *mut c_void;
    fn AnimateSprites();
    fn BeginNormalPaletteFade(a0: u32, a1: i8, a2: u8, a3: u8, a4: u16) -> u8;
    fn BuildOamBuffer();
    fn ClearGpuRegBits(a0: u8, a1: u16);
    fn ClearLinkRfuCallback();
    fn ConvertInternationalString(a0: *mut u8, a1: u8);
    fn CopyBgTilemapBufferToVram(a0: u8);
    fn CopyToBgTilemapBuffer(a0: u8, a1: *mut c_void, a2: u16, a3: u16);
    fn CopyWindowToVram(a0: u8, a1: u8);
    fn CpuSet(a0: *mut c_void, a1: *mut c_void, a2: u32);
    fn CreateTask(a0: Option<unsafe extern "C" fn(u8)>, a1: u8) -> u8;
    fn DeactivateAllTextPrinters();
    fn DecompressAndLoadBgGfxUsingHeap(a0: u8, a1: *mut c_void, a2: u32, a3: u16, a4: u8);
    fn DestroyTask(a0: u8);
    fn DisableInterrupts(a0: u16);
    fn DoSoftReset();
    fn EnableInterrupts(a0: u16);
    fn FillPalette(a0: u16, a1: u16, a2: u16);
    fn FillWindowPixelBuffer(a0: u8, a1: u8);
    fn FlagGet(a0: u16) -> u8;
    fn FreeAllSpritePalettes();
    fn GetGameProgressForLinkTrade() -> i32;
    fn GetRfuRecvQueueLength() -> u32;
    fn InitBgsFromTemplates(a0: u8, a1: *mut BgTemplate, a2: u8);
    fn InitHeap(a0: *mut c_void, a1: u32);
    fn InitRFUAPI();
    fn InitWindows(a0: *mut WindowTemplate) -> u16;
    fn IsLinkRfuTaskFinished() -> u8;
    fn IsNationalPokedexEnabled() -> u32;
    fn IsRfuRecvQueueEmpty() -> u32;
    fn IsSendingKeysOverCable() -> u32;
    fn IsSendingKeysToRfu() -> u32;
    fn LinkRfu_Shutdown();
    fn LoadBgTiles(a0: u8, a1: *mut c_void, a2: u16, a3: u16) -> u16;
    fn LoadOam();
    fn LoadPalette(a0: *mut c_void, a1: u16, a2: u16);
    fn PlaySE(a0: u16);
    fn ProcessSpriteCopyRequests();
    fn PutWindowTilemap(a0: u8);
    fn Random() -> u16;
    fn ReloadSave();
    fn ResetBgsAndClearDma3BusyFlags(a0: u32);
    fn ResetLinkRfuGFLayer();
    fn ResetPaletteFadeControl();
    fn ResetSpriteData();
    fn ResetTasks();
    fn ResetTempTileDataBuffers();
    fn RestoreSerialTimer3IntrHandlers();
    fn RfuMain1() -> u32;
    fn RfuMain2() -> u32;
    fn Rfu_GetBlockReceivedStatus() -> u8;
    fn Rfu_GetLinkPlayerCount() -> u8;
    fn Rfu_GetMultiplayerId() -> u8;
    fn Rfu_InitBlockSend(a0: *mut u8, a1: u32) -> u32;
    fn Rfu_IsMaster() -> u8;
    fn Rfu_ResetBlockReceivedFlag(a0: u8);
    fn Rfu_SendBlockRequest(a0: u8) -> u8;
    fn Rfu_SetBerryBlenderLinkCallback();
    fn Rfu_SetBlockReceivedFlag(a0: u8);
    fn Rfu_SetCloseLinkCallback();
    fn Rfu_SetLinkStandbyCallback();
    fn RunTasks();
    fn ScanlineEffect_Stop();
    fn SeedRng(a0: u16);
    fn SetBgTilemapBuffer(a0: u8, a1: *mut c_void);
    fn SetGpuReg(a0: u8, a1: u16);
    fn SetMainCallback2(a0: Option<unsafe extern "C" fn()>);
    fn SetVBlankCallback(a0: Option<unsafe extern "C" fn()>);
    fn ShowBg(a0: u8);
    fn StartSendingKeysToRfu();
    fn StopMapMusic();
    fn StringCompare(a0: *mut u8, a1: *mut u8) -> i32;
    fn StringCopy(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn TransferPlttBuffer();
    fn TrySavingData(a0: u8) -> u8;
    fn UpdatePaletteFade() -> u8;
    fn m4aMPlayStop(a0: *mut MusicPlayerInfo);
    fn rfu_LMAN_REQBN_softReset_and_checkID() -> u32;
    fn rfu_REQ_stopMode();
    fn rfu_waitREQComplete() -> u16;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsWirelessAdapterConnected() -> u8 {
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
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Task_DestroySelf(taskId: u8) {
    DestroyTask(taskId);
}
pub(crate) unsafe extern "C" fn InitLinkTestBG(
    paletteNum: u8,
    bgNum: u8,
    screenBaseBlock: u8,
    charBaseBlock: u8,
    baseChar: u16,
) {
    LoadPalette(
        sLinkTestDigitsPal.as_ptr().cast_mut() as *mut c_void,
        0x000 + paletteNum as u16 * 16,
        32,
    );
    {
        {
            {
                let mut dmaRegs: *mut u32 = 67109076 as usize as *mut u32;
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
    gLinkTestBGInfo.screenBaseBlock = screenBaseBlock as u32;
    gLinkTestBGInfo.paletteNum = paletteNum as u32;
    gLinkTestBGInfo.baseChar = baseChar as u32;
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
pub(crate) unsafe extern "C" fn LoadLinkTestBgGfx(
    paletteNum: u8,
    bgNum: u8,
    screenBaseBlock: u8,
    charBaseBlock: u8,
) {
    LoadPalette(
        sLinkTestDigitsPal.as_ptr().cast_mut() as *mut c_void,
        0x000 + paletteNum as u16 * 16,
        32,
    );
    {
        {
            {
                let mut dmaRegs: *mut u32 = 67109076 as usize as *mut u32;
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
    gLinkTestBGInfo.screenBaseBlock = screenBaseBlock as u32;
    gLinkTestBGInfo.paletteNum = paletteNum as u32;
    gLinkTestBGInfo.baseChar = 0;
    SetGpuReg(
        sBGControlRegs[bgNum],
        (screenBaseBlock as u16) << 8 | (charBaseBlock as u16) << 2,
    );
}
pub(crate) unsafe extern "C" fn LinkTestScreen() {
    let mut i: i32 = 0;
    ResetSpriteData();
    FreeAllSpritePalettes();
    ResetTasks();
    SetVBlankCallback(Some(VBlankCB_LinkError));
    ResetBlockSend();
    gLinkType = LINKTYPE_TRADE;
    OpenLink();
    SeedRng(gMain.vblankCounter2 as u16);
    i = 0;
    while i < TRAINER_ID_LENGTH as i32 {
        (*gSaveBlock2Ptr).playerTrainerId[i] = (Random() as i32 % 256) as u8;
        i += 1;
    }
    InitLinkTestBG(0, 2, 4, 0, 0);
    SetGpuReg(0x0, 5440);
    CreateTask(Some(Task_DestroySelf), 0);
    RunTasks();
    AnimateSprites();
    BuildOamBuffer();
    UpdatePaletteFade();
    sDummy3 = FALSE as u32;
    InitLocalLinkPlayer();
    CreateTask(Some(Task_PrintTestData), 0);
    SetMainCallback2(Some(CB2_LinkTest));
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetLocalLinkPlayerId(playerId: u8) {
    gLocalLinkPlayer.id = playerId as u16;
}
pub(crate) unsafe extern "C" fn InitLocalLinkPlayer() {
    gLocalLinkPlayer.trainerId = (*gSaveBlock2Ptr).playerTrainerId[0] as u32
        | ((*gSaveBlock2Ptr).playerTrainerId[1] as u32) << 8
        | ((*gSaveBlock2Ptr).playerTrainerId[2] as u32) << 16
        | ((*gSaveBlock2Ptr).playerTrainerId[3] as u32) << 24;
    StringCopy(
        gLocalLinkPlayer.name.as_mut_ptr(),
        (*gSaveBlock2Ptr).playerName.as_mut_ptr(),
    );
    gLocalLinkPlayer.gender = (*gSaveBlock2Ptr).playerGender;
    gLocalLinkPlayer.linkType = gLinkType as u32;
    gLocalLinkPlayer.language = gGameLanguage as u16;
    gLocalLinkPlayer.version = gGameVersion as u16 + 0x4000;
    gLocalLinkPlayer.lp_field_2 = 0x8000;
    gLocalLinkPlayer.progressFlags = IsNationalPokedexEnabled() as u8;
    if FlagGet(FLAG_IS_CHAMPION) != 0 {
        gLocalLinkPlayer.progressFlags |= 0x10;
    }
}
pub(crate) unsafe extern "C" fn VBlankCB_LinkError() {
    LoadOam();
    ProcessSpriteCopyRequests();
    TransferPlttBuffer();
}
pub(crate) unsafe extern "C" fn InitLink() {
    let mut i: i32 = 0;
    i = 0;
    while i < CMD_LENGTH as i32 {
        gSendCmd[i] = LINKCMD_NONE;
        i += 1;
    }
    sLinkOpen = TRUE;
    EnableSerial();
}
pub(crate) unsafe extern "C" fn Task_TriggerHandshake(taskId: u8) {
    if ({
        gTasks[taskId].data[0] += 1;
        gTasks[taskId].data[0]
    }) == 5
    {
        gShouldAdvanceLinkState = 1;
        DestroyTask(taskId);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn OpenLink() {
    let mut i: i32 = 0;
    if gWirelessCommType == 0 {
        ResetSerial();
        InitLink();
        gLinkCallback = Some(LinkCB_RequestPlayerDataExchange);
        gLinkVSyncDisabled = FALSE;
        gLinkErrorOccurred = FALSE;
        gSuppressLinkErrorMessage = FALSE;
        ResetBlockReceivedFlags();
        ResetBlockSend();
        sDummy1 = FALSE as u32;
        gLinkDummy2 = FALSE;
        gLinkDummy1 = FALSE;
        gReadyCloseLinkType = 0;
        CreateTask(Some(Task_TriggerHandshake), 2);
    } else {
        InitRFUAPI();
    }
    gReceivedRemoteLinkPlayers = 0;
    i = 0;
    while i < MAX_LINK_PLAYERS {
        gRemoteLinkPlayersNotReceived[i] = TRUE;
        gReadyToCloseLink[i] = FALSE;
        gReadyToExitStandby[i] = FALSE;
        i += 1;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CloseLink() {
    gReceivedRemoteLinkPlayers = FALSE;
    if gWirelessCommType != 0 {
        LinkRfu_Shutdown();
    }
    sLinkOpen = FALSE;
    DisableSerial();
}
pub(crate) unsafe extern "C" fn TestBlockTransfer(nothing: u8, is: u8, used: u8) {
    let mut i: u8 = 0;
    let mut status: u8 = 0;
    if sLinkTestLastBlockSendPos as u16 != sBlockSend.pos {
        LinkTest_PrintHex(sBlockSend.pos as u32, 2, 3, 2);
        sLinkTestLastBlockSendPos = sBlockSend.pos as u8;
    }
    i = 0;
    while i < MAX_LINK_PLAYERS as u8 {
        if sLinkTestLastBlockRecvPos[i] as u16 != sBlockRecv[i].pos {
            LinkTest_PrintHex(sBlockRecv[i].pos as u32, 2, i + 4, 2);
            sLinkTestLastBlockRecvPos[i] = sBlockRecv[i].pos as u8;
        }
        i += 1;
    }
    status = GetBlockReceivedStatus();
    if status == 0xF {
        i = 0;
        while i < MAX_LINK_PLAYERS as u8 {
            if shr_i32(status as i32, i as u32) & 1 != 0 {
                gLinkTestBlockChecksums[i] =
                    LinkTestCalcBlockChecksum(gBlockRecvBuffer[i].as_mut_ptr(), sBlockRecv[i].size);
                ResetBlockReceivedFlag(i);
                if gLinkTestBlockChecksums[i] != 0x0342 {
                    sLinkTestDebugValuesEnabled = FALSE;
                    sDummyFlag = FALSE;
                }
            }
            i += 1;
        }
    }
}
pub(crate) unsafe extern "C" fn LinkTestProcessKeyInput() {
    if gMain.newKeys as i32 & A_BUTTON != 0 {
        gShouldAdvanceLinkState = 1;
    }
    if gMain.heldKeys as i32 & B_BUTTON != 0 {
        InitBlockSend(gHeap.as_mut_ptr().at(16384) as *mut c_void, 0x00002004);
    }
    if gMain.newKeys as i32 & L_BUTTON != 0 {
        BeginNormalPaletteFade(PALETTES_ALL, 0, 16, 0, 2);
    }
    if gMain.newKeys as i32 & START_BUTTON != 0 {
        SetSuppressLinkErrorMessage(TRUE);
    }
    if gMain.newKeys as i32 & R_BUTTON != 0 {
        TrySavingData(SAVE_LINK);
    }
    if gMain.newKeys as i32 & SELECT_BUTTON != 0 {
        SetCloseLinkCallback();
    }
    if sLinkTestDebugValuesEnabled != 0 {
        SetLinkDebugValues(
            gMain.vblankCounter2,
            (if gLinkCallback.is_some() {
                gLinkVSyncDisabled as i32
            } else {
                gLinkVSyncDisabled as i32 | 0x10
            }) as u32,
        );
    }
}
pub(crate) unsafe extern "C" fn CB2_LinkTest() {
    LinkTestProcessKeyInput();
    TestBlockTransfer(1, 1, 0);
    RunTasks();
    AnimateSprites();
    BuildOamBuffer();
    UpdatePaletteFade();
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn LinkMain2(heldKeys: *mut u16) -> u16 {
    let mut i: u8 = 0;
    if sLinkOpen == 0 {
        return 0;
    }
    i = 0;
    while i < CMD_LENGTH {
        gSendCmd[i] = 0;
        i += 1;
    }
    gLinkHeldKeys = *heldKeys;
    if gLinkStatus & LINK_STAT_CONN_ESTABLISHED != 0 {
        ProcessRecvCmds((*(67109160 as usize as *mut SioMultiCnt)).id() as u8);
        if gLinkCallback.is_some() {
            gLinkCallback.unwrap_unchecked()();
        }
        TrySetLinkErrorBuffer();
    }
    return gLinkStatus as u16;
}
pub(crate) unsafe extern "C" fn HandleReceiveRemoteLinkPlayer(who: u8) {
    let mut i: i32 = 0;
    let mut count: i32 = 0;
    count = 0;
    gRemoteLinkPlayersNotReceived[who] = FALSE;
    i = 0;
    while i < GetLinkPlayerCount_2() as i32 {
        count += gRemoteLinkPlayersNotReceived[i] as i32;
        i += 1;
    }
    if count == 0 && gReceivedRemoteLinkPlayers == 0 {
        gReceivedRemoteLinkPlayers = 1;
    }
}
pub(crate) unsafe extern "C" fn ProcessRecvCmds(unused: u8) {
    let mut i: u16 = 0;
    i = 0;
    while i < MAX_LINK_PLAYERS as u16 {
        'l1: {
            gLinkPartnersHeldKeys[i] = 0;
            if gRecvCmds[i][0] == 0 {
                break 'l1;
            }
            'l3: {
                match gRecvCmds[i][0] {
                    LINKCMD_SEND_LINK_TYPE => {
                        let mut block: *mut LinkPlayerBlock = null_mut();
                        InitLocalLinkPlayer();
                        block = &raw mut gLocalLinkPlayerBlock;
                        (*block).linkPlayer = gLocalLinkPlayer;
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
                        gLinkPartnersHeldKeys[i] = gRecvCmds[i][1];
                    }
                    LINKCMD_DUMMY_1 => {
                        gLinkDummy2 = TRUE;
                    }
                    LINKCMD_DUMMY_2 => {
                        gLinkDummy2 = TRUE;
                    }
                    LINKCMD_INIT_BLOCK => {
                        let mut blockRecv: *mut BlockTransfer = null_mut();
                        blockRecv = &raw mut sBlockRecv[i];
                        (*blockRecv).pos = 0;
                        (*blockRecv).size = gRecvCmds[i][1];
                        (*blockRecv).multiplayerId = gRecvCmds[i][2] as u8;
                        break 'l3;
                    }
                    LINKCMD_CONT_BLOCK => {
                        if sBlockRecv[i].size > BLOCK_BUFFER_SIZE as u16 {
                            let mut buffer: *mut u16 = null_mut();
                            let mut j: u16 = 0;
                            buffer = gDecompressionBuffer.as_mut_ptr() as *mut u16;
                            j = 0;
                            while j < 7 {
                                *buffer.at(sBlockRecv[i].pos as i32 / 2 + j as i32) =
                                    gRecvCmds[i][j as i32 + 1];
                                j += 1;
                            }
                        } else {
                            let mut j: u16 = 0;
                            j = 0;
                            while j < 7 {
                                gBlockRecvBuffer[i][sBlockRecv[i].pos as i32 / 2 + j as i32] =
                                    gRecvCmds[i][j as i32 + 1];
                                j += 1;
                            }
                        }
                        sBlockRecv[i].pos += 14;
                        if sBlockRecv[i].pos >= sBlockRecv[i].size {
                            if gRemoteLinkPlayersNotReceived[i] == TRUE {
                                let mut block: *mut LinkPlayerBlock = null_mut();
                                let mut linkPlayer: *mut LinkPlayer = null_mut();
                                block = &raw mut gBlockRecvBuffer[i] as *mut LinkPlayerBlock;
                                linkPlayer = &raw mut gLinkPlayers[i];
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
                        gReadyToCloseLink[i] = TRUE;
                    }
                    LINKCMD_READY_EXIT_STANDBY => {
                        gReadyToExitStandby[i] = TRUE;
                    }
                    LINKCMD_BLENDER_NO_PBLOCK_SPACE => {
                        SetBerryBlenderLinkCallback();
                    }
                    LINKCMD_SEND_BLOCK_REQ => {
                        SendBlock(
                            0,
                            sBlockRequests[gRecvCmds[i][1]].address,
                            sBlockRequests[gRecvCmds[i][1]].size as u16,
                        );
                    }
                    LINKCMD_SEND_HELD_KEYS => {
                        gLinkPartnersHeldKeys[i] = gRecvCmds[i][1];
                    }
                    _ => {}
                }
            }
        }
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn BuildSendCmd(command: u16) {
    'l1: {
        match command {
            LINKCMD_SEND_LINK_TYPE => {
                gSendCmd[0] = LINKCMD_SEND_LINK_TYPE;
                gSendCmd[1] = gLinkType;
            }
            LINKCMD_READY_EXIT_STANDBY => {
                gSendCmd[0] = LINKCMD_READY_EXIT_STANDBY;
            }
            LINKCMD_BLENDER_SEND_KEYS => {
                gSendCmd[0] = LINKCMD_BLENDER_SEND_KEYS;
                gSendCmd[1] = gMain.heldKeys;
            }
            LINKCMD_DUMMY_1 => {
                gSendCmd[0] = LINKCMD_DUMMY_1;
            }
            LINKCMD_SEND_EMPTY => {
                gSendCmd[0] = LINKCMD_SEND_EMPTY;
                gSendCmd[1] = 0;
            }
            LINKCMD_SEND_0xEE => {
                let mut i: u8 = 0;
                gSendCmd[0] = LINKCMD_SEND_0xEE;
                i = 0;
                while i < 5 {
                    gSendCmd[i as i32 + 1] = 0xEE;
                    i += 1;
                }
                break 'l1;
            }
            LINKCMD_INIT_BLOCK => {
                gSendCmd[0] = LINKCMD_INIT_BLOCK;
                gSendCmd[1] = sBlockSend.size;
                gSendCmd[2] = sBlockSend.multiplayerId as u16 + 0x80;
            }
            LINKCMD_BLENDER_NO_PBLOCK_SPACE => {
                gSendCmd[0] = LINKCMD_BLENDER_NO_PBLOCK_SPACE;
            }
            LINKCMD_SEND_ITEM => {
                gSendCmd[0] = LINKCMD_SEND_ITEM;
                gSendCmd[1] = gSpecialVar_ItemId;
            }
            LINKCMD_SEND_BLOCK_REQ => {
                gSendCmd[0] = LINKCMD_SEND_BLOCK_REQ;
                gSendCmd[1] = gBlockRequestType as u16;
            }
            LINKCMD_READY_CLOSE_LINK => {
                gSendCmd[0] = LINKCMD_READY_CLOSE_LINK;
                gSendCmd[1] = gReadyCloseLinkType;
            }
            LINKCMD_DUMMY_2 => {
                gSendCmd[0] = LINKCMD_DUMMY_2;
            }
            LINKCMD_SEND_HELD_KEYS => {
                if gHeldKeyCodeToSend == 0 || gLinkTransferringData != 0 {
                    break 'l1;
                }
                gSendCmd[0] = LINKCMD_SEND_HELD_KEYS;
                gSendCmd[1] = gHeldKeyCodeToSend;
            }
            _ => {}
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn StartSendingKeysToLink() {
    if gWirelessCommType != 0 {
        StartSendingKeysToRfu();
    }
    gLinkCallback = Some(LinkCB_SendHeldKeys);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsSendingKeysToLink() -> u32 {
    if gWirelessCommType != 0 {
        return IsSendingKeysToRfu();
    }
    if gLinkCallback == Some(LinkCB_SendHeldKeys as unsafe extern "C" fn()) {
        return TRUE as u32;
    }
    return FALSE as u32;
}
pub(crate) unsafe extern "C" fn LinkCB_SendHeldKeys() {
    if gReceivedRemoteLinkPlayers == TRUE {
        BuildSendCmd(LINKCMD_SEND_HELD_KEYS);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ClearLinkCallback() {
    if gWirelessCommType != 0 {
        ClearLinkRfuCallback();
    } else {
        gLinkCallback = None;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ClearLinkCallback_2() {
    if gWirelessCommType != 0 {
        ClearLinkRfuCallback();
    } else {
        gLinkCallback = None;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetLinkPlayerCount() -> u8 {
    if gWirelessCommType != 0 {
        return Rfu_GetLinkPlayerCount();
    }
    return ((gLinkStatus & 0x0000001C) >> 2) as u8;
}
pub(crate) unsafe extern "C" fn AreAnyLinkPlayersUsingVersions(
    version1: u32,
    version2: u32,
) -> i32 {
    let mut i: i32 = 0;
    let mut nPlayers: u8 = 0;
    nPlayers = GetLinkPlayerCount();
    i = 0;
    while i < nPlayers as i32 {
        if gLinkPlayers[i].version as u32 & 0xFF == version1
            || gLinkPlayers[i].version as u32 & 0xFF == version2
        {
            return 1;
        }
        i += 1;
    }
    return -1;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn LinkDummy_Return2() -> u32 {
    return 2;
}
pub(crate) unsafe extern "C" fn IsFullLinkGroupWithNoRS() -> u32 {
    if GetLinkPlayerCount() != MAX_LINK_PLAYERS as u8
        || AreAnyLinkPlayersUsingVersions(VERSION_RUBY as u32, VERSION_SAPPHIRE as u32) < 0
    {
        return FALSE as u32;
    }
    return TRUE as u32;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Link_AnyPartnersPlayingRubyOrSapphire() -> u32 {
    if AreAnyLinkPlayersUsingVersions(VERSION_RUBY as u32, VERSION_SAPPHIRE as u32) >= 0 {
        return TRUE as u32;
    }
    return FALSE as u32;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Link_AnyPartnersPlayingFRLG_JP() -> u32 {
    let mut i: i32 = 0;
    i = AreAnyLinkPlayersUsingVersions(VERSION_FIRE_RED as u32, VERSION_LEAF_GREEN as u32);
    if i >= 0 && gLinkPlayers[i].language == LANGUAGE_JAPANESE as u16 {
        return TRUE as u32;
    }
    return FALSE as u32;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn OpenLinkTimed() {
    sPlayerDataExchangeStatus = EXCHANGE_NOT_STARTED;
    sTimeOutCounter = 0;
    OpenLink();
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetLinkPlayerDataExchangeStatusTimed(
    minPlayers: i32,
    maxPlayers: i32,
) -> u8 {
    let mut i: i32 = 0;
    let mut count: i32 = 0;
    let mut index: u32 = 0;
    let mut numPlayers: u8 = 0;
    let mut linkType1: u32 = 0;
    let mut linkType2: u32 = 0;
    count = 0;
    if gReceivedRemoteLinkPlayers == TRUE {
        numPlayers = GetLinkPlayerCount_2();
        if minPlayers > numPlayers as i32 || numPlayers as i32 > maxPlayers {
            sPlayerDataExchangeStatus = EXCHANGE_WRONG_NUM_PLAYERS as u32;
            return sPlayerDataExchangeStatus as u8;
        } else {
            if GetLinkPlayerCount() == 0 {
                gLinkErrorOccurred = TRUE;
                CloseLink();
            }
            i = 0;
            index = 0;
            while i < GetLinkPlayerCount() as i32 {
                if gLinkPlayers[index].linkType == gLinkPlayers[0].linkType {
                    count += 1;
                }
                index += 1;
                i += 1;
            }
            if count == GetLinkPlayerCount() as i32 {
                if gLinkPlayers[0].linkType == LINKTYPE_TRADE_SETUP as u32 {
                    match GetGameProgressForLinkTrade() {
                        TRADE_PLAYER_NOT_READY => {
                            sPlayerDataExchangeStatus = EXCHANGE_PLAYER_NOT_READY as u32;
                        }
                        TRADE_PARTNER_NOT_READY => {
                            sPlayerDataExchangeStatus = EXCHANGE_PARTNER_NOT_READY as u32;
                        }
                        TRADE_BOTH_PLAYERS_READY => {
                            sPlayerDataExchangeStatus = EXCHANGE_COMPLETE;
                        }
                        _ => {}
                    }
                } else {
                    sPlayerDataExchangeStatus = EXCHANGE_COMPLETE;
                }
            } else {
                sPlayerDataExchangeStatus = EXCHANGE_DIFF_SELECTIONS;
                linkType1 = gLinkPlayers[GetMultiplayerId()].linkType;
                linkType2 = gLinkPlayers[GetMultiplayerId() as i32 ^ 1].linkType;
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
        sTimeOutCounter += 1;
        sTimeOutCounter
    }) > 600
    {
        sPlayerDataExchangeStatus = EXCHANGE_TIMED_OUT;
    }
    return sPlayerDataExchangeStatus as u8;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsLinkPlayerDataExchangeComplete() -> u8 {
    let mut i: u8 = 0;
    let mut count: u8 = 0;
    let mut retval: u8 = 0;
    count = 0;
    i = 0;
    while i < GetLinkPlayerCount() {
        if gLinkPlayers[i].linkType == gLinkPlayers[0].linkType {
            count += 1;
        }
        i += 1;
    }
    if count == GetLinkPlayerCount() {
        retval = TRUE;
        sPlayerDataExchangeStatus = EXCHANGE_COMPLETE;
    } else {
        retval = FALSE;
        sPlayerDataExchangeStatus = EXCHANGE_DIFF_SELECTIONS;
    }
    return retval;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetLinkPlayerTrainerId(who: u8) -> u32 {
    return gLinkPlayers[who].trainerId;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ResetLinkPlayers() {
    let mut i: i32 = 0;
    i = 0;
    while i <= MAX_LINK_PLAYERS {
        gLinkPlayers[i] = {
            let mut lit1: LinkPlayer = zeroed();
            lit1.version = 0;
            lit1
        };
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn ResetBlockSend() {
    sBlockSend.active = FALSE;
    sBlockSend.pos = 0;
    sBlockSend.size = 0;
    sBlockSend.src = null_mut();
}
pub(crate) unsafe extern "C" fn InitBlockSend(src: *mut c_void, size: u32) -> u32 {
    if sBlockSend.active != 0 {
        return FALSE as u32;
    }
    sBlockSend.multiplayerId = GetMultiplayerId();
    sBlockSend.active = TRUE;
    sBlockSend.size = size as u16;
    sBlockSend.pos = 0;
    if size > BLOCK_BUFFER_SIZE {
        sBlockSend.src = src as *mut u8;
    } else {
        if (src as usize) != (gBlockSendBuffer.as_mut_ptr() as usize) {
            memcpy(gBlockSendBuffer.as_mut_ptr(), src as *mut u8, size);
        }
        sBlockSend.src = gBlockSendBuffer.as_mut_ptr();
    }
    BuildSendCmd(LINKCMD_INIT_BLOCK);
    gLinkCallback = Some(LinkCB_BlockSendBegin);
    sBlockSendDelayCounter = 0;
    return TRUE as u32;
}
pub(crate) unsafe extern "C" fn LinkCB_BlockSendBegin() {
    if ({
        sBlockSendDelayCounter += 1;
        sBlockSendDelayCounter
    }) > 2
    {
        gLinkCallback = Some(LinkCB_BlockSend);
    }
}
pub(crate) unsafe extern "C" fn LinkCB_BlockSend() {
    let mut i: i32 = 0;
    let mut src: *mut u8 = null_mut();
    src = sBlockSend.src;
    gSendCmd[0] = LINKCMD_CONT_BLOCK;
    i = 0;
    while i < 7 {
        gSendCmd[i + 1] = (*src.at(sBlockSend.pos as i32 + i * 2 + 1) as u16) << 8
            | *src.at(sBlockSend.pos as i32 + i * 2) as u16;
        i += 1;
    }
    sBlockSend.pos += 14;
    if sBlockSend.size <= sBlockSend.pos {
        sBlockSend.active = FALSE;
        gLinkCallback = Some(LinkCB_BlockSendEnd);
    }
}
pub(crate) unsafe extern "C" fn LinkCB_BlockSendEnd() {
    gLinkCallback = None;
}
pub(crate) unsafe extern "C" fn LinkCB_BerryBlenderSendHeldKeys() {
    GetMultiplayerId();
    BuildSendCmd(LINKCMD_BLENDER_SEND_KEYS);
    gBerryBlenderKeySendAttempts += 1;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetBerryBlenderLinkCallback() {
    gBerryBlenderKeySendAttempts = 0;
    if gWirelessCommType != 0 {
        Rfu_SetBerryBlenderLinkCallback();
    } else {
        gLinkCallback = Some(LinkCB_BerryBlenderSendHeldKeys);
    }
}
pub(crate) unsafe extern "C" fn GetBerryBlenderKeySendAttempts() -> u32 {
    return gBerryBlenderKeySendAttempts;
}
pub(crate) unsafe extern "C" fn SendBerryBlenderNoSpaceForPokeblocks() {
    BuildSendCmd(LINKCMD_BLENDER_NO_PBLOCK_SPACE);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetMultiplayerId() -> u8 {
    if gWirelessCommType == TRUE {
        return Rfu_GetMultiplayerId();
    }
    return (*(67109160 as usize as *mut SioMultiCnt)).id() as u8;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BitmaskAllOtherLinkPlayers() -> u8 {
    let mut mpId: u8 = 0;
    mpId = GetMultiplayerId();
    return 15 ^ shl_i32(1, mpId as u32) as u8;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SendBlock(unused: u8, src: *mut c_void, size: u16) -> u8 {
    if gWirelessCommType == TRUE {
        return Rfu_InitBlockSend(src as *mut u8, size as u32) as u8;
    }
    return InitBlockSend(src, size as u32) as u8;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SendBlockRequest(blockReqType: u8) -> u8 {
    if gWirelessCommType == TRUE {
        return Rfu_SendBlockRequest(blockReqType);
    }
    if gLinkCallback.is_none() {
        gBlockRequestType = blockReqType;
        BuildSendCmd(LINKCMD_SEND_BLOCK_REQ);
        return TRUE;
    }
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsLinkTaskFinished() -> u8 {
    if gWirelessCommType == TRUE {
        return IsLinkRfuTaskFinished();
    }
    return gLinkCallback.is_none() as u8;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetBlockReceivedStatus() -> u8 {
    if gWirelessCommType == TRUE {
        return Rfu_GetBlockReceivedStatus();
    }
    return gBlockReceivedStatus[3] << 3
        | gBlockReceivedStatus[2] << 2
        | gBlockReceivedStatus[1] << 1
        | gBlockReceivedStatus[0] << 0;
}
pub(crate) unsafe extern "C" fn SetBlockReceivedFlag(who: u8) {
    if gWirelessCommType == TRUE {
        Rfu_SetBlockReceivedFlag(who);
    } else {
        gBlockReceivedStatus[who] = TRUE;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ResetBlockReceivedFlags() {
    let mut i: i32 = 0;
    if gWirelessCommType == TRUE {
        i = 0;
        while i < MAX_RFU_PLAYERS {
            Rfu_ResetBlockReceivedFlag(i as u8);
            i += 1;
        }
    } else {
        i = 0;
        while i < MAX_LINK_PLAYERS {
            gBlockReceivedStatus[i] = FALSE;
            i += 1;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ResetBlockReceivedFlag(who: u8) {
    if gWirelessCommType == TRUE {
        Rfu_ResetBlockReceivedFlag(who);
    } else if gBlockReceivedStatus[who] != 0 {
        gBlockReceivedStatus[who] = FALSE;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CheckShouldAdvanceLinkState() {
    if gLinkStatus & LINK_STAT_MASTER != 0 && (gLinkStatus & 0x0000001C) >> 2 > 1 {
        gShouldAdvanceLinkState = 1;
    }
}
pub(crate) unsafe extern "C" fn LinkTestCalcBlockChecksum(src: *mut u16, size: u16) -> u16 {
    let mut chksum: u16 = 0;
    let mut i: u16 = 0;
    chksum = 0;
    i = 0;
    while (i as i32) < size as i32 / 2 {
        chksum += *src.at(i);
        i += 1;
    }
    return chksum;
}
pub(crate) unsafe extern "C" fn LinkTest_PrintNumChar(val: u8, x: u8, y: u8) {
    let mut vAddr: *mut u16 = null_mut();
    vAddr = (0x6000000 + 0x800 * gLinkTestBGInfo.screenBaseBlock) as usize as *mut u16;
    *vAddr.at(y as i32 * 32 + x as i32) = (gLinkTestBGInfo.paletteNum as u16) << 12
        | val as u16 + 1 + gLinkTestBGInfo.baseChar as u16;
}
pub(crate) unsafe extern "C" fn LinkTest_PrintChar(val: u8, x: u8, y: u8) {
    let mut vAddr: *mut u16 = null_mut();
    vAddr = (0x6000000 + 0x800 * gLinkTestBGInfo.screenBaseBlock) as usize as *mut u16;
    *vAddr.at(y as i32 * 32 + x as i32) =
        (gLinkTestBGInfo.paletteNum as u16) << 12 | val as u16 + gLinkTestBGInfo.baseChar as u16;
}
pub(crate) unsafe extern "C" fn LinkTest_PrintHex(mut num: u32, mut x: u8, y: u8, length: u8) {
    let mut buff: CArray<u8, 16> = zeroed();
    let mut i: i32 = 0;
    i = 0;
    while i < length as i32 {
        buff[i] = num as u8 & 0xF;
        num >>= 4;
        i += 1;
    }
    i = length as i32 - 1;
    while i >= 0 {
        LinkTest_PrintNumChar(buff[i], x, y);
        x += 1;
        i -= 1;
    }
}
pub(crate) unsafe extern "C" fn LinkTest_PrintInt(mut num: i32, mut x: u8, y: u8, length: u8) {
    let mut buff: CArray<u8, 16> = zeroed();
    let mut negX: i32 = 0;
    let mut i: i32 = 0;
    negX = -1;
    if num < 0 {
        negX = x as i32;
        num = -num;
    }
    i = 0;
    while i < length as i32 {
        buff[i] = (num % 10) as u8;
        num = num / 10;
        i += 1;
    }
    i = length as i32 - 1;
    while i >= 0 {
        LinkTest_PrintNumChar(buff[i], x, y);
        x += 1;
        i -= 1;
    }
    if negX != -1 {
        LinkTest_PrintNumChar(*b"\x0a\0".as_ptr().cast_mut(), negX as u8, y);
    }
}
pub(crate) unsafe extern "C" fn LinkTest_PrintString(mut str: *mut u8, x: u8, y: u8) {
    let mut xOffset: i32 = 0;
    let mut i: i32 = 0;
    let mut yOffset: i32 = 0;
    yOffset = 0;
    xOffset = 0;
    i = 0;
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
pub(crate) unsafe extern "C" fn LinkCB_RequestPlayerDataExchange() {
    if gLinkStatus & LINK_STAT_MASTER != 0 {
        BuildSendCmd(LINKCMD_SEND_LINK_TYPE);
    }
    gLinkCallback = None;
}
pub(crate) unsafe extern "C" fn Task_PrintTestData(taskId: u8) {
    let mut testTitle: CArray<u8, 32> = zeroed();
    let mut i: i32 = 0;
    strcpy(testTitle.as_mut_ptr(), sASCIITestPrint.as_ptr().cast_mut());
    LinkTest_PrintString(testTitle.as_mut_ptr(), 5, 2);
    LinkTest_PrintHex(gShouldAdvanceLinkState as u32, 2, 1, 2);
    LinkTest_PrintHex(gLinkStatus, 15, 1, 8);
    LinkTest_PrintHex(gLink.state as u32, 2, 10, 2);
    LinkTest_PrintHex((gLinkStatus & 0x0000001C) >> 2, 15, 10, 2);
    LinkTest_PrintHex(GetMultiplayerId() as u32, 15, 12, 2);
    LinkTest_PrintHex(gLastSendQueueCount as u32, 25, 1, 2);
    LinkTest_PrintHex(gLastRecvQueueCount as u32, 25, 2, 2);
    LinkTest_PrintHex(GetBlockReceivedStatus() as u32, 15, 5, 2);
    LinkTest_PrintHex(gLinkDebugSeed, 2, 12, 8);
    LinkTest_PrintHex(gLinkDebugFlags, 2, 13, 8);
    LinkTest_PrintHex(GetSioMultiSI() as u32, 25, 5, 1);
    LinkTest_PrintHex(IsSioMultiMaster() as u32, 25, 6, 1);
    LinkTest_PrintHex(IsLinkConnectionEstablished() as u32, 25, 7, 1);
    LinkTest_PrintHex(HasLinkErrorOccurred() as u32, 25, 8, 1);
    i = 0;
    while i < MAX_LINK_PLAYERS {
        LinkTest_PrintHex(gLinkTestBlockChecksums[i] as u32, 10, 4 + i as u8, 4);
        i += 1;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetLinkDebugValues(seed: u32, flags: u32) {
    gLinkDebugSeed = seed;
    gLinkDebugFlags = flags;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetSavedLinkPlayerCountAsBitFlags() -> u8 {
    let mut i: i32 = 0;
    let mut flags: u8 = 0;
    flags = 0;
    i = 0;
    while i < gSavedLinkPlayerCount as i32 {
        flags |= shl_i32(1, i as u32) as u8;
        i += 1;
    }
    return flags;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetLinkPlayerCountAsBitFlags() -> u8 {
    let mut i: i32 = 0;
    let mut flags: u8 = 0;
    flags = 0;
    i = 0;
    while i < GetLinkPlayerCount() as i32 {
        flags |= shl_i32(1, i as u32) as u8;
        i += 1;
    }
    return flags;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SaveLinkPlayers(playerCount: u8) {
    let mut i: i32 = 0;
    gSavedLinkPlayerCount = playerCount;
    gSavedMultiplayerId = GetMultiplayerId();
    i = 0;
    while i < MAX_RFU_PLAYERS {
        sSavedLinkPlayers[i] = gLinkPlayers[i];
        i += 1;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetSavedPlayerCount() -> u8 {
    return gSavedLinkPlayerCount;
}
pub(crate) unsafe extern "C" fn GetSavedMultiplayerId() -> u8 {
    return gSavedMultiplayerId;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DoesLinkPlayerCountMatchSaved() -> u8 {
    let mut i: i32 = 0;
    let mut count: u32 = 0;
    i = 0;
    while i < gSavedLinkPlayerCount as i32 {
        if gLinkPlayers[i].trainerId == sSavedLinkPlayers[i].trainerId {
            if gLinkType == LINKTYPE_BATTLE_TOWER {
                if gLinkType as u32 == gLinkPlayers[i].linkType {
                    count += 1;
                }
            } else {
                count += 1;
            }
        }
        i += 1;
    }
    if count == gSavedLinkPlayerCount as u32 {
        if GetLinkPlayerCount_2() == gSavedLinkPlayerCount {
            return TRUE;
        }
    }
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ClearSavedLinkPlayers() {
    memset(sSavedLinkPlayers.as_mut_ptr() as *mut u8, 0, 140);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CheckLinkPlayersMatchSaved() {
    let mut i: u8 = 0;
    i = 0;
    while i < gSavedLinkPlayerCount {
        if sSavedLinkPlayers[i].trainerId != gLinkPlayers[i].trainerId
            || StringCompare(
                sSavedLinkPlayers[i].name.as_mut_ptr(),
                gLinkPlayers[i].name.as_mut_ptr(),
            ) != 0
        {
            gLinkErrorOccurred = TRUE;
            CloseLink();
            SetMainCallback2(Some(CB2_LinkError));
        }
        i += 1;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ResetLinkPlayerCount() {
    gSavedLinkPlayerCount = 0;
    gSavedMultiplayerId = 0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetLinkPlayerCount_2() -> u8 {
    return ((gLinkStatus & 0x0000001C) >> 2) as u8;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsLinkMaster() -> u8 {
    if gWirelessCommType != 0 {
        return Rfu_IsMaster();
    }
    return (gLinkStatus >> 5) as u8 & 1;
}
pub(crate) unsafe extern "C" fn GetDummy2() -> u8 {
    return sDummy2;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetCloseLinkCallbackAndType(r#type: u16) {
    if gWirelessCommType == TRUE {
        Rfu_SetCloseLinkCallback();
    } else {
        if gLinkCallback.is_none() {
            gLinkCallback = Some(LinkCB_ReadyCloseLink);
            gLinkDummy1 = FALSE;
            gReadyCloseLinkType = r#type;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetCloseLinkCallback() {
    if gWirelessCommType == TRUE {
        Rfu_SetCloseLinkCallback();
    } else {
        if gLinkCallback.is_some() {
            sReadyCloseLinkAttempts += 1;
        } else {
            gLinkCallback = Some(LinkCB_ReadyCloseLink);
            gLinkDummy1 = FALSE;
            gReadyCloseLinkType = 0;
        }
    }
}
pub(crate) unsafe extern "C" fn LinkCB_ReadyCloseLink() {
    if gLastRecvQueueCount == 0 {
        BuildSendCmd(LINKCMD_READY_CLOSE_LINK);
        gLinkCallback = Some(LinkCB_WaitCloseLink);
    }
}
pub(crate) unsafe extern "C" fn LinkCB_WaitCloseLink() {
    let mut i: i32 = 0;
    let mut count: u32 = 0;
    let mut linkPlayerCount: u8 = GetLinkPlayerCount();
    count = 0;
    i = 0;
    while i < linkPlayerCount as i32 {
        if gReadyToCloseLink[i] != 0 {
            count += 1;
        }
        i += 1;
    }
    if count == linkPlayerCount as u32 {
        gBattleTypeFlags &= 0xffffffdf;
        gLinkVSyncDisabled = TRUE;
        CloseLink();
        gLinkCallback = None;
        gLinkDummy1 = TRUE;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetCloseLinkCallbackHandleJP() {
    if gWirelessCommType == TRUE {
        Rfu_SetCloseLinkCallback();
    } else {
        if gLinkCallback.is_some() {
            sReadyCloseLinkAttempts += 1;
        } else {
            gLinkCallback = Some(LinkCB_ReadyCloseLinkWithJP);
            gLinkDummy1 = FALSE;
            gReadyCloseLinkType = 0;
        }
    }
}
pub(crate) unsafe extern "C" fn LinkCB_ReadyCloseLinkWithJP() {
    if gLastRecvQueueCount == 0 {
        BuildSendCmd(LINKCMD_READY_CLOSE_LINK);
        gLinkCallback = Some(LinkCB_WaitCloseLinkWithJP);
    }
}
pub(crate) unsafe extern "C" fn LinkCB_WaitCloseLinkWithJP() {
    let mut i: i32 = 0;
    let mut count: u32 = 0;
    let mut linkPlayerCount: u8 = 0;
    linkPlayerCount = GetLinkPlayerCount();
    count = 0;
    i = 0;
    while i < linkPlayerCount as i32 {
        if gLinkPlayers[i].language == LANGUAGE_JAPANESE as u16 {
            count += 1;
        } else if gReadyToCloseLink[i] != 0 {
            count += 1;
        }
        i += 1;
    }
    if count == linkPlayerCount as u32 {
        gBattleTypeFlags &= 0xffffffdf;
        gLinkVSyncDisabled = TRUE;
        CloseLink();
        gLinkCallback = None;
        gLinkDummy1 = TRUE;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetLinkStandbyCallback() {
    if gWirelessCommType == TRUE {
        Rfu_SetLinkStandbyCallback();
    } else {
        if gLinkCallback.is_none() {
            gLinkCallback = Some(LinkCB_Standby);
        }
        gLinkDummy1 = FALSE;
    }
}
pub(crate) unsafe extern "C" fn LinkCB_Standby() {
    if gLastRecvQueueCount == 0 {
        BuildSendCmd(LINKCMD_READY_EXIT_STANDBY);
        gLinkCallback = Some(LinkCB_StandbyForAll);
    }
}
pub(crate) unsafe extern "C" fn LinkCB_StandbyForAll() {
    let mut i: u8 = 0;
    let mut linkPlayerCount: u8 = GetLinkPlayerCount();
    i = 0;
    while i < linkPlayerCount {
        if gReadyToExitStandby[i] == 0 {
            break;
        }
        i += 1;
    }
    if i == linkPlayerCount {
        i = 0;
        while i < MAX_LINK_PLAYERS as u8 {
            gReadyToExitStandby[i] = FALSE;
            i += 1;
        }
        gLinkCallback = None;
    }
}
pub(crate) unsafe extern "C" fn TrySetLinkErrorBuffer() {
    if sLinkOpen != 0 && (gLinkStatus & 0x0007F000) >> 12 != 0 {
        if gSuppressLinkErrorMessage == 0 {
            sLinkErrorBuffer.status = gLinkStatus;
            sLinkErrorBuffer.lastRecvQueueCount = gLastRecvQueueCount;
            sLinkErrorBuffer.lastSendQueueCount = gLastSendQueueCount;
            SetMainCallback2(Some(CB2_LinkError));
        }
        gLinkErrorOccurred = TRUE;
        CloseLink();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetLinkErrorBuffer(
    status: u32,
    lastSendQueueCount: u8,
    lastRecvQueueCount: u8,
    disconnected: u8,
) {
    sLinkErrorBuffer.status = status;
    sLinkErrorBuffer.lastSendQueueCount = lastSendQueueCount;
    sLinkErrorBuffer.lastRecvQueueCount = lastRecvQueueCount;
    sLinkErrorBuffer.disconnected = disconnected;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CB2_LinkError() {
    let mut tilemapBuffer: *mut u8 = null_mut();
    SetGpuReg(0x0, 0);
    m4aMPlayStop(&raw mut gMPlayInfo_SE1);
    m4aMPlayStop(&raw mut gMPlayInfo_SE2);
    m4aMPlayStop(&raw mut gMPlayInfo_SE3);
    InitHeap(gHeap.as_mut_ptr() as *mut c_void, HEAP_SIZE);
    ResetSpriteData();
    FreeAllSpritePalettes();
    ResetPaletteFadeControl();
    SetBackdropFromColor(0);
    ResetTasks();
    ScanlineEffect_Stop();
    if gWirelessCommType != 0 {
        if sLinkErrorBuffer.disconnected == 0 {
            gWirelessCommType = 3;
        }
        ResetLinkRfuGFLayer();
    }
    SetVBlankCallback(Some(VBlankCB_LinkError));
    ResetBgsAndClearDma3BusyFlags(0);
    InitBgsFromTemplates(0, sLinkErrorBgTemplates.as_ptr().cast_mut(), 2);
    sLinkErrorBgTilemapBuffer = ({
        tilemapBuffer = Alloc(BG_SCREEN_SIZE) as *mut u8;
        tilemapBuffer
    }) as *mut c_void;
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
            gStandardMenuPalette.as_ptr().cast_mut() as *mut c_void,
            240,
            32,
        );
        gSoftResetDisabled = FALSE;
        CreateTask(Some(Task_DestroySelf), 0);
        StopMapMusic();
        gMain.callback1 = None;
        RunTasks();
        AnimateSprites();
        BuildOamBuffer();
        UpdatePaletteFade();
        SetMainCallback2(Some(CB2_PrintErrorMessage));
    }
}
pub(crate) unsafe extern "C" fn ErrorMsg_MoveCloserToPartner() {
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
        gText_CommErrorEllipsis.as_ptr().cast_mut(),
    );
    AddTextPrinterParameterized3(
        WIN_LINK_ERROR_BOTTOM,
        FONT_SHORT_COPY_1,
        2,
        1,
        sTextColors.as_ptr().cast_mut(),
        0,
        gText_MoveCloserToLinkPartner.as_ptr().cast_mut(),
    );
    PutWindowTilemap(WIN_LINK_ERROR_TOP);
    PutWindowTilemap(WIN_LINK_ERROR_BOTTOM);
    CopyWindowToVram(WIN_LINK_ERROR_TOP, COPYWIN_NONE);
    CopyWindowToVram(WIN_LINK_ERROR_BOTTOM, COPYWIN_FULL);
}
pub(crate) unsafe extern "C" fn ErrorMsg_CheckConnections() {
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
        gText_CommErrorCheckConnections.as_ptr().cast_mut(),
    );
    PutWindowTilemap(WIN_LINK_ERROR_MID);
    PutWindowTilemap(WIN_LINK_ERROR_BOTTOM);
    CopyWindowToVram(WIN_LINK_ERROR_MID, COPYWIN_NONE);
    CopyWindowToVram(WIN_LINK_ERROR_BOTTOM, COPYWIN_FULL);
}
pub(crate) unsafe extern "C" fn CB2_PrintErrorMessage() {
    match gMain.state {
        0 => {
            if sLinkErrorBuffer.disconnected != 0 {
                ErrorMsg_MoveCloserToPartner();
            } else {
                ErrorMsg_CheckConnections();
            }
        }
        2 => {
            ShowBg(0);
            if sLinkErrorBuffer.disconnected != 0 {
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
            if gWirelessCommType == 2 {
                AddTextPrinterParameterized3(
                    WIN_LINK_ERROR_TOP,
                    FONT_SHORT_COPY_1,
                    2,
                    20,
                    sTextColors.as_ptr().cast_mut(),
                    0,
                    gText_ABtnTitleScreen.as_ptr().cast_mut(),
                );
            } else if gWirelessCommType == 1 {
                AddTextPrinterParameterized3(
                    WIN_LINK_ERROR_TOP,
                    FONT_SHORT_COPY_1,
                    2,
                    20,
                    sTextColors.as_ptr().cast_mut(),
                    0,
                    gText_ABtnRegistrationCounter.as_ptr().cast_mut(),
                );
            }
        }
        _ => {}
    }
    if gMain.state == 160 {
        if gWirelessCommType == 1 {
            if gMain.newKeys as i32 & A_BUTTON != 0 {
                PlaySE(SE_PIN);
                gWirelessCommType = 0;
                sLinkErrorBuffer.disconnected = FALSE;
                ReloadSave();
            }
        } else if gWirelessCommType == 2 {
            if gMain.newKeys as i32 & A_BUTTON != 0 {
                rfu_REQ_stopMode();
                rfu_waitREQComplete();
                DoSoftReset();
            }
        }
    }
    if gMain.state != 160 {
        gMain.state += 1;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetSioMultiSI() -> u8 {
    return ((67109160 as usize as *mut u16).read_volatile() as i32 & SIO_MULTI_SI != 0) as u8;
}
pub(crate) unsafe extern "C" fn IsSioMultiMaster() -> u8 {
    return ((67109160 as usize as *mut u16).read_volatile() as i32 & SIO_MULTI_SD != 0
        && (67109160 as usize as *mut u16).read_volatile() as i32 & SIO_MULTI_SI == 0)
        as u8;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsLinkConnectionEstablished() -> u8 {
    return (gLinkStatus >> 6) as u8 & 1;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetSuppressLinkErrorMessage(flag: u8) {
    gSuppressLinkErrorMessage = flag;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn HasLinkErrorOccurred() -> u8 {
    return gLinkErrorOccurred;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn LocalLinkPlayerToBlock() {
    let mut block: *mut LinkPlayerBlock = null_mut();
    InitLocalLinkPlayer();
    block = &raw mut gLocalLinkPlayerBlock;
    (*block).linkPlayer = gLocalLinkPlayer;
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
    memcpy(gBlockSendBuffer.as_mut_ptr(), block as *mut u8, 60);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn LinkPlayerFromBlock(who: u32) {
    let mut who_: u8 = who as u8;
    let mut block: *mut LinkPlayerBlock = null_mut();
    let mut player: *mut LinkPlayer = null_mut();
    block = gBlockRecvBuffer[who_].as_mut_ptr() as *mut LinkPlayerBlock;
    player = &raw mut gLinkPlayers[who_];
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
pub unsafe extern "C" fn HandleLinkConnection() -> u8 {
    let mut main1Failed: u32 = 0;
    let mut main2Failed: u32 = 0;
    if gWirelessCommType == 0 {
        gLinkStatus = LinkMain1(
            &raw mut gShouldAdvanceLinkState,
            gSendCmd.as_mut_ptr(),
            gRecvCmds.as_mut_ptr(),
        );
        LinkMain2(&raw mut gMain.heldKeys);
        if gLinkStatus & LINK_STAT_RECEIVED_NOTHING != 0 && IsSendingKeysOverCable() == TRUE as u32
        {
            return TRUE;
        }
    } else {
        main1Failed = RfuMain1();
        main2Failed = RfuMain2();
        if IsSendingKeysOverCable() == TRUE as u32 {
            if main1Failed == TRUE as u32 || IsRfuRecvQueueEmpty() != 0 || main2Failed != 0 {
                return TRUE;
            }
        }
    }
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetWirelessCommType1() {
    if gReceivedRemoteLinkPlayers == 0 {
        gWirelessCommType = 1;
    }
}
pub(crate) unsafe extern "C" fn SetWirelessCommType0_Internal() {
    if gReceivedRemoteLinkPlayers == 0 {
        gWirelessCommType = 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetWirelessCommType0() {
    if gReceivedRemoteLinkPlayers == 0 {
        gWirelessCommType = 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetLinkRecvQueueLength() -> u32 {
    if gWirelessCommType != 0 {
        return GetRfuRecvQueueLength();
    }
    return gLink.recvQueue.count as u32;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsLinkRecvQueueAtOverworldMax() -> u32 {
    if GetLinkRecvQueueLength() >= OVERWORLD_RECV_QUEUE_MAX {
        return TRUE as u32;
    }
    return FALSE as u32;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetWirelessCommType() -> u8 {
    return gWirelessCommType;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ConvertLinkPlayerName(player: *mut LinkPlayer) {
    (*player).progressFlagsCopy = (*player).progressFlags;
    ConvertInternationalString((*player).name.as_mut_ptr(), (*player).language as u8);
}
pub(crate) unsafe extern "C" fn DisableSerial() {
    DisableInterrupts(192);
    volatile_write(67109160 as usize as *mut u16, SIO_MULTI_MODE);
    volatile_write(67109134 as usize as *mut u16, 0);
    volatile_write(67109378 as usize as *mut u16, 192);
    volatile_write(67109162 as usize as *mut u16, 0);
    volatile_write(67109152 as usize as *mut u64, 0);
    {
        {
            let mut tmp: u32 = 0;
            volatile_write(&raw mut tmp, 0);
            CpuSet(
                &raw mut tmp as *mut c_void,
                &raw mut gLink as *mut c_void,
                0x50003f0,
            );
        }
    }
}
pub(crate) unsafe extern "C" fn EnableSerial() {
    DisableInterrupts(192);
    volatile_write(67109172 as usize as *mut u16, 0);
    volatile_write(67109160 as usize as *mut u16, SIO_MULTI_MODE);
    volatile_write(
        67109160 as usize as *mut u16,
        (67109160 as usize as *mut u16).read_volatile() | 16387,
    );
    EnableInterrupts(INTR_FLAG_SERIAL);
    volatile_write(67109162 as usize as *mut u16, 0);
    {
        {
            let mut tmp: u32 = 0;
            volatile_write(&raw mut tmp, 0);
            CpuSet(
                &raw mut tmp as *mut c_void,
                &raw mut gLink as *mut c_void,
                0x50003f0,
            );
        }
    }
    sNumVBlanksWithoutSerialIntr = 0;
    sSendNonzeroCheck = 0;
    sRecvNonzeroCheck = 0;
    sChecksumAvailable = 0;
    sHandshakePlayerCount = 0;
    gLastSendQueueCount = 0;
    gLastRecvQueueCount = 0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ResetSerial() {
    EnableSerial();
    DisableSerial();
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn LinkMain1(
    shouldAdvanceLinkState: *mut u8,
    sendCmd: *mut u16,
    recvCmds: *mut CArray<u16, 8>,
) -> u32 {
    let mut retVal: u32 = 0;
    let mut retVal2: u32 = 0;
    'l1: {
        let sw1: u8 = gLink.state;
        let mut fall = false;
        if sw1 == LINK_STATE_START0 {
            fall = true;
            DisableSerial();
            gLink.state = 1;
            break 'l1;
        }
        if sw1 == LINK_STATE_START1 {
            fall = true;
            if *shouldAdvanceLinkState == 1 {
                EnableSerial();
                gLink.state = 2;
            }
            break 'l1;
        }
        if sw1 == LINK_STATE_HANDSHAKE {
            fall = true;
            match *shouldAdvanceLinkState {
                1 => {
                    if gLink.isMaster == LINK_MASTER && gLink.playerCount > 1 {
                        gLink.handshakeAsMaster = TRUE;
                    }
                }
                2 => {
                    gLink.state = LINK_STATE_START0;
                    volatile_write(67109162 as usize as *mut u16, 0);
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
            gLink.state = LINK_STATE_CONN_ESTABLISHED;
        }
        if fall || sw1 == LINK_STATE_CONN_ESTABLISHED {
            fall = true;
            EnqueueSendCmd(sendCmd);
            DequeueRecvCmds(recvCmds);
            break 'l1;
        }
    }
    *shouldAdvanceLinkState = 0;
    retVal = gLink.localId as u32;
    retVal |= (gLink.playerCount as u32) << 2;
    if gLink.isMaster == LINK_MASTER {
        retVal |= LINK_STAT_MASTER;
    }
    {
        let mut receivedNothing: u32 = (gLink.receivedNothing as u32) << 8;
        let mut link_field_F: u32 = (gLink.link_field_F as u32) << 9;
        let mut hardwareError: u32 = (gLink.hardwareError as u32) << 12;
        let mut badChecksum: u32 = (gLink.badChecksum as u32) << 13;
        let mut queueFull: u32 = (gLink.queueFull as u32) << 14;
        let mut val: u32 = 0;
        if gLink.state == LINK_STATE_CONN_ESTABLISHED {
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
    if gLink.lag == LAG_MASTER {
        retVal |= LINK_STAT_ERROR_LAG_MASTER;
    }
    if gLink.localId >= MAX_LINK_PLAYERS as u8 {
        retVal |= LINK_STAT_ERROR_INVALID_ID;
    }
    retVal2 = retVal;
    if gLink.lag == LAG_SLAVE {
        retVal2 |= LINK_STAT_ERROR_LAG_SLAVE;
    }
    return retVal2;
}
pub(crate) unsafe extern "C" fn CheckMasterOrSlave() {
    let mut terminals: u32 = 0;
    terminals = (REG_ADDR_SIOCNT as usize as *mut u32).read_volatile() & 12;
    if terminals == SIO_MULTI_SD as u32 && gLink.localId == 0 {
        gLink.isMaster = LINK_MASTER;
    } else {
        gLink.isMaster = LINK_SLAVE;
    }
}
pub(crate) unsafe extern "C" fn InitTimer() {
    if gLink.isMaster != 0 {
        volatile_write(67109132 as usize as *mut u16, 65339);
        volatile_write(67109134 as usize as *mut u16, 65);
        EnableInterrupts(INTR_FLAG_TIMER3);
    }
}
pub(crate) unsafe extern "C" fn EnqueueSendCmd(mut sendCmd: *mut u16) {
    let mut i: u8 = 0;
    let mut offset: u8 = 0;
    gLinkSavedIme = (67109384 as usize as *mut u16).read_volatile();
    volatile_write(67109384 as usize as *mut u16, 0);
    if gLink.sendQueue.count < QUEUE_CAPACITY {
        offset = gLink.sendQueue.pos + gLink.sendQueue.count;
        if offset >= QUEUE_CAPACITY {
            offset -= QUEUE_CAPACITY;
        }
        i = 0;
        while i < CMD_LENGTH {
            sSendNonzeroCheck |= *sendCmd;
            gLink.sendQueue.data[i][offset] = *sendCmd;
            *sendCmd = 0;
            sendCmd = sendCmd.at(1);
            i += 1;
        }
    } else {
        gLink.queueFull = QUEUE_FULL_SEND;
    }
    if sSendNonzeroCheck != 0 {
        gLink.sendQueue.count += 1;
        sSendNonzeroCheck = 0;
    }
    volatile_write(67109384 as usize as *mut u16, gLinkSavedIme);
    gLastSendQueueCount = gLink.sendQueue.count;
}
pub(crate) unsafe extern "C" fn DequeueRecvCmds(mut recvCmds: *mut CArray<u16, 8>) {
    let mut i: u8 = 0;
    let mut j: u8 = 0;
    gLinkSavedIme = (67109384 as usize as *mut u16).read_volatile();
    volatile_write(67109384 as usize as *mut u16, 0);
    if gLink.recvQueue.count == 0 {
        i = 0;
        while i < gLink.playerCount {
            j = 0;
            while j < CMD_LENGTH {
                (*recvCmds.at(i))[j] = 0;
                j += 1;
            }
            i += 1;
        }
        gLink.receivedNothing = TRUE;
    } else {
        i = 0;
        while i < gLink.playerCount {
            j = 0;
            while j < CMD_LENGTH {
                (*recvCmds.at(i))[j] = gLink.recvQueue.data[i][j][gLink.recvQueue.pos];
                j += 1;
            }
            i += 1;
        }
        gLink.recvQueue.count -= 1;
        gLink.recvQueue.pos += 1;
        if gLink.recvQueue.pos >= QUEUE_CAPACITY {
            gLink.recvQueue.pos = 0;
        }
        gLink.receivedNothing = FALSE;
    }
    volatile_write(67109384 as usize as *mut u16, gLinkSavedIme);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn LinkVSync() {
    if gLink.isMaster != 0 {
        match gLink.state {
            LINK_STATE_CONN_ESTABLISHED => {
                if gLink.serialIntrCounter < 9 {
                    if gLink.hardwareError != TRUE {
                        gLink.lag = LAG_MASTER;
                    } else {
                        StartTransfer();
                    }
                } else if gLink.lag != LAG_MASTER {
                    gLink.serialIntrCounter = 0;
                    StartTransfer();
                }
            }
            LINK_STATE_HANDSHAKE => {
                StartTransfer();
            }
            _ => {}
        }
    } else if gLink.state == LINK_STATE_CONN_ESTABLISHED || gLink.state == LINK_STATE_HANDSHAKE {
        if ({
            sNumVBlanksWithoutSerialIntr += 1;
            sNumVBlanksWithoutSerialIntr
        }) > 10
        {
            if gLink.state == LINK_STATE_CONN_ESTABLISHED {
                gLink.lag = LAG_SLAVE;
            }
            if gLink.state == LINK_STATE_HANDSHAKE {
                gLink.playerCount = 0;
                gLink.link_field_F = FALSE;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Timer3Intr() {
    StopTimer();
    StartTransfer();
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SerialCB() {
    gLink.localId = (*(67109160 as usize as *mut SioMultiCnt)).id() as u8;
    match gLink.state {
        LINK_STATE_CONN_ESTABLISHED => {
            gLink.hardwareError = (*(67109160 as usize as *mut SioMultiCnt)).error() as u8;
            DoRecv();
            DoSend();
            SendRecvDone();
        }
        LINK_STATE_HANDSHAKE => {
            if DoHandshake() != 0 {
                if gLink.isMaster != 0 {
                    gLink.state = LINK_STATE_INIT_TIMER;
                    gLink.serialIntrCounter = 8;
                } else {
                    gLink.state = LINK_STATE_CONN_ESTABLISHED;
                }
            }
        }
        _ => {}
    }
    gLink.serialIntrCounter += 1;
    sNumVBlanksWithoutSerialIntr = 0;
    if gLink.serialIntrCounter == 8 {
        gLastRecvQueueCount = gLink.recvQueue.count;
    }
}
pub(crate) unsafe extern "C" fn StartTransfer() {
    volatile_write(
        67109160 as usize as *mut u16,
        (67109160 as usize as *mut u16).read_volatile() | SIO_START,
    );
}
pub(crate) unsafe extern "C" fn DoHandshake() -> u8 {
    let mut i: u8 = 0;
    let mut playerCount: u8 = 0;
    let mut minRecv: u16 = 0;
    let mut recvSiomlt: u64 = 0;
    playerCount = 0;
    minRecv = 0xFFFF;
    if gLink.handshakeAsMaster == TRUE {
        volatile_write(67109162 as usize as *mut u16, MASTER_HANDSHAKE);
    } else {
        volatile_write(67109162 as usize as *mut u16, SLAVE_HANDSHAKE);
    }
    recvSiomlt = (67109152 as usize as *mut u64).read_volatile();
    memcpy(
        gLink.handshakeBuffer.as_mut_ptr() as *mut u8,
        &raw mut recvSiomlt as *mut u8,
        8,
    );
    volatile_write(67109152 as usize as *mut u64, 0);
    gLink.handshakeAsMaster = FALSE;
    i = 0;
    while i < MAX_LINK_PLAYERS as u8 {
        if gLink.handshakeBuffer[i] as i32 & -4 == SLAVE_HANDSHAKE as i32
            || gLink.handshakeBuffer[i] == MASTER_HANDSHAKE
        {
            playerCount += 1;
            if minRecv > gLink.handshakeBuffer[i] && gLink.handshakeBuffer[i] != 0 {
                minRecv = gLink.handshakeBuffer[i];
            }
        } else {
            if gLink.handshakeBuffer[i] != 0xFFFF {
                playerCount = 0;
            }
            break;
        }
        i += 1;
    }
    gLink.playerCount = playerCount;
    if gLink.playerCount > 1
        && gLink.playerCount == sHandshakePlayerCount
        && gLink.handshakeBuffer[0] == MASTER_HANDSHAKE
    {
        return TRUE;
    }
    if gLink.playerCount > 1 {
        gLink.link_field_F = (minRecv as u8 & 3) + 1;
    } else {
        gLink.link_field_F = 0;
    }
    sHandshakePlayerCount = gLink.playerCount;
    return FALSE;
}
pub(crate) unsafe extern "C" fn DoRecv() {
    let mut recv: CArray<u16, 4> = zeroed();
    let mut i: u8 = 0;
    let mut index: u8 = 0;
    let mut recvSiomlt: u64 = (67109152 as usize as *mut u64).read_volatile();
    memcpy(
        recv.as_mut_ptr() as *mut u8,
        &raw mut recvSiomlt as *mut u8,
        8,
    );
    if gLink.sendCmdIndex == 0 {
        i = 0;
        while i < gLink.playerCount {
            if gLink.checksum != recv[i] && sChecksumAvailable != 0 {
                gLink.badChecksum = TRUE;
            }
            i += 1;
        }
        gLink.checksum = 0;
        sChecksumAvailable = TRUE;
    } else {
        index = gLink.recvQueue.pos + gLink.recvQueue.count;
        if index >= QUEUE_CAPACITY {
            index -= QUEUE_CAPACITY;
        }
        if gLink.recvQueue.count < QUEUE_CAPACITY {
            i = 0;
            while i < gLink.playerCount {
                gLink.checksum += recv[i];
                sRecvNonzeroCheck |= recv[i];
                gLink.recvQueue.data[i][gLink.recvCmdIndex][index] = recv[i];
                i += 1;
            }
        } else {
            gLink.queueFull = QUEUE_FULL_RECV;
        }
        gLink.recvCmdIndex += 1;
        if gLink.recvCmdIndex == CMD_LENGTH && sRecvNonzeroCheck != 0 {
            gLink.recvQueue.count += 1;
            sRecvNonzeroCheck = 0;
        }
    }
}
pub(crate) unsafe extern "C" fn DoSend() {
    if gLink.sendCmdIndex == CMD_LENGTH {
        volatile_write(67109162 as usize as *mut u16, gLink.checksum);
        if sSendBufferEmpty == 0 {
            gLink.sendQueue.count -= 1;
            gLink.sendQueue.pos += 1;
            if gLink.sendQueue.pos >= QUEUE_CAPACITY {
                gLink.sendQueue.pos = 0;
            }
        } else {
            sSendBufferEmpty = FALSE;
        }
    } else {
        if sSendBufferEmpty == 0 && gLink.sendQueue.count == 0 {
            sSendBufferEmpty = TRUE;
        }
        if sSendBufferEmpty != 0 {
            volatile_write(67109162 as usize as *mut u16, 0);
        } else {
            volatile_write(
                67109162 as usize as *mut u16,
                gLink.sendQueue.data[gLink.sendCmdIndex][gLink.sendQueue.pos],
            );
        }
        gLink.sendCmdIndex += 1;
    }
}
pub(crate) unsafe extern "C" fn StopTimer() {
    if gLink.isMaster != 0 {
        volatile_write(
            67109134 as usize as *mut u16,
            (67109134 as usize as *mut u16).read_volatile() & 65407,
        );
        volatile_write(67109132 as usize as *mut u16, 65339);
    }
}
pub(crate) unsafe extern "C" fn SendRecvDone() {
    if gLink.recvCmdIndex == CMD_LENGTH {
        gLink.sendCmdIndex = 0;
        gLink.recvCmdIndex = 0;
    } else if gLink.isMaster != 0 {
        volatile_write(
            67109134 as usize as *mut u16,
            (67109134 as usize as *mut u16).read_volatile() | TIMER_ENABLE,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ResetSendBuffer() {
    let mut i: u8 = 0;
    let mut j: u8 = 0;
    gLink.sendQueue.count = 0;
    gLink.sendQueue.pos = 0;
    i = 0;
    while i < CMD_LENGTH {
        j = 0;
        while j < QUEUE_CAPACITY {
            gLink.sendQueue.data[i][j] = LINKCMD_NONE;
            j += 1;
        }
        i += 1;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ResetRecvBuffer() {
    let mut i: u8 = 0;
    let mut j: u8 = 0;
    let mut k: u8 = 0;
    gLink.recvQueue.count = 0;
    gLink.recvQueue.pos = 0;
    i = 0;
    while i < MAX_LINK_PLAYERS as u8 {
        j = 0;
        while j < CMD_LENGTH {
            k = 0;
            while k < QUEUE_CAPACITY {
                gLink.recvQueue.data[i][j][k] = LINKCMD_NONE;
                k += 1;
            }
            j += 1;
        }
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn SetBackdropFromColor(color: u16) {
    FillPalette(color, 0, 2);
}
