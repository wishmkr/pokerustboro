//! Translated from `src/link.c` by tools/rustport/c2rs.py, then reviewed.
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

// Data tables (translate with cdata.py): sWirelessLinkDisplayPal sWirelessLinkDisplayGfx sWirelessLinkDisplayTilemap sLinkTestDigitsPal sLinkTestDigitsGfx sUnusedTransparentWhite sCommErrorBg_Gfx sBlockRequests sBGControlRegs sASCIIGameFreakInc sASCIITestPrint sLinkErrorBgTemplates sLinkErrorWindowTemplates sTextColors sUnusedData
#[allow(unused_imports)]
use crate::data::link::*;

pub(crate) static mut sBlockSend: crate::ffi::Align4<[u8; 12]> = crate::ffi::Align4([0; 12]);
pub(crate) static mut sBlockRecv: crate::ffi::Align4<[u8; 48]> = crate::ffi::Align4([0; 48]);
pub(crate) static mut sBlockSendDelayCounter: u32 = 0u32;
pub(crate) static mut sDummy1: u32 = 0u32;
pub(crate) static mut sDummy2: u8 = 0u8;
pub(crate) static mut sPlayerDataExchangeStatus: u32 = 0u32;
pub(crate) static mut sDummy3: u32 = 0u32;
pub(crate) static mut sLinkTestLastBlockSendPos: u8 = 0u8;
pub(crate) static mut sLinkTestLastBlockRecvPos: crate::ffi::Align4<[u8; 4]> =
    crate::ffi::Align4([0; 4]);
pub(crate) static mut sNumVBlanksWithoutSerialIntr: u8 = 0u8;
pub(crate) static mut sSendBufferEmpty: u8 = 0u8;
pub(crate) static mut sSendNonzeroCheck: u16 = 0u16;
pub(crate) static mut sRecvNonzeroCheck: u16 = 0u16;
pub(crate) static mut sChecksumAvailable: u8 = 0u8;
pub(crate) static mut sHandshakePlayerCount: u8 = 0u8;
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gLinkPartnersHeldKeys: crate::ffi::Align4<[u8; 12]> = crate::ffi::Align4([0; 12]);
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gLinkDebugSeed: u32 = 0u32;
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gLocalLinkPlayerBlock: crate::ffi::Align4<[u8; 60]> = crate::ffi::Align4([0; 60]);
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gLinkErrorOccurred: u8 = 0u8;
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gLinkDebugFlags: u32 = 0u32;
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gLinkFiller1: u32 = 0u32;
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gRemoteLinkPlayersNotReceived: crate::ffi::Align4<[u8; 4]> =
    crate::ffi::Align4([0; 4]);
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gBlockReceivedStatus: crate::ffi::Align4<[u8; 4]> = crate::ffi::Align4([0; 4]);
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gLinkFiller2: u32 = 0u32;
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gLinkHeldKeys: u16 = 0u16;
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gRecvCmds: crate::ffi::Align4<[u8; 80]> = crate::ffi::Align4([0; 80]);
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gLinkStatus: u32 = 0u32;
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gLinkDummy1: u8 = 0u8;
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gLinkDummy2: u8 = 0u8;
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gReadyToExitStandby: crate::ffi::Align4<[u8; 4]> = crate::ffi::Align4([0; 4]);
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gReadyToCloseLink: crate::ffi::Align4<[u8; 4]> = crate::ffi::Align4([0; 4]);
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gReadyCloseLinkType: u16 = 0u16;
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gSuppressLinkErrorMessage: u8 = 0u8;
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gWirelessCommType: u8 = 0u8;
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gSavedLinkPlayerCount: u8 = 0u8;
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gSendCmd: crate::ffi::Align4<[u8; 16]> = crate::ffi::Align4([0; 16]);
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gSavedMultiplayerId: u8 = 0u8;
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gReceivedRemoteLinkPlayers: u8 = 0u8;
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gLinkTestBGInfo: crate::ffi::Align4<[u8; 16]> = crate::ffi::Align4([0; 16]);
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gLinkCallback: Option<unsafe extern "C" fn()> = None;
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gShouldAdvanceLinkState: u8 = 0u8;
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gLinkTestBlockChecksums: crate::ffi::Align4<[u8; 8]> = crate::ffi::Align4([0; 8]);
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gBlockRequestType: u8 = 0u8;
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gLinkFiller3: u32 = 0u32;
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gLinkFiller4: u32 = 0u32;
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gLinkFiller5: u32 = 0u32;
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gLastSendQueueCount: u8 = 0u8;
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gLink: crate::ffi::Align4<[u8; 4032]> = crate::ffi::Align4([0; 4032]);
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gLastRecvQueueCount: u8 = 0u8;
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gLinkSavedIme: u16 = 0u16;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sLinkTestDebugValuesEnabled: u8 = 0u8;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sDummyFlag: u8 = 0u8;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gBerryBlenderKeySendAttempts: u32 = 0u32;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gBlockRecvBuffer: crate::ffi::Align4<[u8; 1280]> = crate::ffi::Align4([0; 1280]);
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gBlockSendBuffer: crate::ffi::Align4<[u8; 256]> = crate::ffi::Align4([0; 256]);
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sLinkOpen: u8 = 0u8;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gLinkType: u16 = 0u16;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sTimeOutCounter: u16 = 0u16;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gLocalLinkPlayer: crate::ffi::Align4<[u8; 28]> = crate::ffi::Align4([0; 28]);
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gLinkPlayers: crate::ffi::Align4<[u8; 140]> = crate::ffi::Align4([0; 140]);
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sSavedLinkPlayers: crate::ffi::Align4<[u8; 140]> =
    crate::ffi::Align4([0; 140]);
pub(crate) static mut sLinkErrorBuffer: crate::ffi::Align4<[u8; 8]> = crate::ffi::Align4([0; 8]);
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sReadyCloseLinkAttempts: u16 = 0u16;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sLinkErrorBgTilemapBuffer: *mut u8 = core::ptr::null_mut();

unsafe extern "C" {
    static mut gBattleTypeFlags: u8;
    static mut gDecompressionBuffer: u8;
    static mut gGameLanguage: u8;
    static mut gGameVersion: u8;
    static mut gHeap: u8;
    static mut gHeldKeyCodeToSend: u8;
    static mut gLinkTransferringData: u8;
    static mut gLinkVSyncDisabled: u8;
    static mut gMPlayInfo_SE1: u8;
    static mut gMPlayInfo_SE2: u8;
    static mut gMPlayInfo_SE3: u8;
    static mut gMain: u8;
    static mut gSaveBlock2Ptr: u8;
    static mut gSoftResetDisabled: u8;
    static mut gSpecialVar_0x8005: u8;
    static mut gSpecialVar_ItemId: u8;
    static mut gStandardMenuPalette: u8;
    static mut gTasks: u8;
    static mut gText_ABtnRegistrationCounter: u8;
    static mut gText_ABtnTitleScreen: u8;
    static mut gText_CommErrorCheckConnections: u8;
    static mut gText_CommErrorEllipsis: u8;
    static mut gText_MoveCloserToLinkPartner: u8;
    fn AddTextPrinterParameterized3(
        a0: u8,
        a1: u8,
        a2: u8,
        a3: u8,
        a4: *mut u8,
        a5: i8,
        a6: *mut u8,
    );
    fn Alloc(a0: u32) -> *mut u8;
    fn AnimateSprites();
    fn BeginNormalPaletteFade(a0: u32, a1: i8, a2: u8, a3: u8, a4: u16) -> u8;
    fn BuildOamBuffer();
    fn ClearGpuRegBits(a0: u8, a1: u16);
    fn ClearLinkRfuCallback();
    fn ConvertInternationalString(a0: *mut u8, a1: u8);
    fn CopyBgTilemapBufferToVram(a0: u8);
    fn CopyToBgTilemapBuffer(a0: u8, a1: *mut u8, a2: u16, a3: u16);
    fn CopyWindowToVram(a0: u8, a1: u8);
    fn CpuSet(a0: *mut u8, a1: *mut u8, a2: u32);
    fn CreateTask(a0: Option<unsafe extern "C" fn(u8)>, a1: u8) -> u8;
    fn DeactivateAllTextPrinters();
    fn DecompressAndLoadBgGfxUsingHeap(a0: u8, a1: *mut u8, a2: u32, a3: u16, a4: u8);
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
    fn InitBgsFromTemplates(a0: u8, a1: *mut u8, a2: u8);
    fn InitHeap(a0: *mut u8, a1: u32);
    fn InitRFUAPI();
    fn InitWindows(a0: *mut u8) -> u16;
    fn IsLinkRfuTaskFinished() -> u8;
    fn IsNationalPokedexEnabled() -> u32;
    fn IsRfuRecvQueueEmpty() -> u32;
    fn IsSendingKeysOverCable() -> u32;
    fn IsSendingKeysToRfu() -> u32;
    fn LinkRfu_Shutdown();
    fn LoadBgTiles(a0: u8, a1: *mut u8, a2: u16, a3: u16) -> u16;
    fn LoadOam();
    fn LoadPalette(a0: *mut u8, a1: u16, a2: u16);
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
    fn SetBgTilemapBuffer(a0: u8, a1: *mut u8);
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
    fn m4aMPlayStop(a0: *mut u8);
    fn rfu_LMAN_REQBN_softReset_and_checkID() -> u32;
    fn rfu_REQ_stopMode();
    fn rfu_waitREQComplete() -> u16;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsWirelessAdapterConnected() -> u8 {
    unsafe {
        SetWirelessCommType1();
        InitRFUAPI();
        if rfu_LMAN_REQBN_softReset_and_checkID() == 32769u32 {
            rfu_REQ_stopMode();
            rfu_waitREQComplete();
            return 1u8;
        }
        SetWirelessCommType0_Internal();
        CloseLink();
        RestoreSerialTimer3IntrHandlers();
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Task_DestroySelf(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        DestroyTask(taskId);
    }
}
pub(crate) unsafe extern "C" fn InitLinkTestBG(
    paletteNum: u8,
    bgNum: u8,
    screenBaseBlock: u8,
    charBaseBlock: u8,
    baseChar: u16,
) {
    unsafe {
        let mut paletteNum = paletteNum;
        let mut bgNum = bgNum;
        let mut screenBaseBlock = screenBaseBlock;
        let mut charBaseBlock = charBaseBlock;
        let mut baseChar = baseChar;
        LoadPalette(
            (((&raw const sLinkTestDigitsPal)
                .cast::<u8>()
                .cast_mut()
                .cast::<u16>())
            .cast::<u16>())
            .cast::<u8>(),
            (((0i32).wrapping_add(((paletteNum) as i32).wrapping_mul(16i32))) as u16),
            32u16,
        );
        'l1: loop {
            'l2: {
                'l3: loop {
                    'l4: {
                        {
                            let mut dmaRegs: *mut u32 = ((67109076i32) as usize as *mut u32);
                            crate::c::volatile_write(
                                dmaRegs,
                                ((((&raw const sLinkTestDigitsGfx)
                                    .cast::<u8>()
                                    .cast_mut()
                                    .cast::<u16>())
                                .cast::<u16>()) as usize as u32),
                            );
                            crate::c::volatile_write(
                                (dmaRegs).wrapping_offset(1),
                                (((((100663296i32).wrapping_add(
                                    (16384i32).wrapping_mul(((charBaseBlock) as i32)),
                                )) as usize as *mut u16)
                                    .wrapping_offset(
                                        ((16i32).wrapping_mul(((baseChar) as i32))) as isize,
                                    )) as usize as u32),
                            );
                            crate::c::volatile_write(
                                (dmaRegs).wrapping_offset(2),
                                (2147483648u32
                                    | crate::c::div_u32(
                                        544u32,
                                        ((crate::c::div_i32(16i32, 8i32)) as u32),
                                    )),
                            );
                            let _ = ((dmaRegs).wrapping_offset(2)).read_volatile();
                        }
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
        (((&raw mut gLinkTestBGInfo).cast::<u8>()).cast::<u32>()).write(((screenBaseBlock) as u32));
        (((&raw mut gLinkTestBGInfo).cast::<u8>())
            .wrapping_add(4)
            .cast::<u32>())
        .write(((paletteNum) as u32));
        (((&raw mut gLinkTestBGInfo).cast::<u8>())
            .wrapping_add(8)
            .cast::<u32>())
        .write(((baseChar) as u32));
        'l5: {
            let __sw1 = ((bgNum) as i32);
            if __sw1 == 1i32 {
                SetGpuReg(
                    10u8,
                    ((((((screenBaseBlock) as i32) << 8) | 1i32) | (((charBaseBlock) as i32) << 2))
                        as u16),
                );
                break 'l5;
            }
            if __sw1 == 2i32 {
                SetGpuReg(
                    12u8,
                    ((((((screenBaseBlock) as i32) << 8) | 1i32) | (((charBaseBlock) as i32) << 2))
                        as u16),
                );
                break 'l5;
            }
            if __sw1 == 3i32 {
                SetGpuReg(
                    14u8,
                    ((((((screenBaseBlock) as i32) << 8) | 1i32) | (((charBaseBlock) as i32) << 2))
                        as u16),
                );
                break 'l5;
            }
        }
        SetGpuReg(
            (((16i32).wrapping_add(((bgNum) as i32).wrapping_mul(4i32))) as u8),
            0u16,
        );
        SetGpuReg(
            (((18i32).wrapping_add(((bgNum) as i32).wrapping_mul(4i32))) as u8),
            0u16,
        );
    }
}
pub(crate) unsafe extern "C" fn LoadLinkTestBgGfx(
    paletteNum: u8,
    bgNum: u8,
    screenBaseBlock: u8,
    charBaseBlock: u8,
) {
    unsafe {
        let mut paletteNum = paletteNum;
        let mut bgNum = bgNum;
        let mut screenBaseBlock = screenBaseBlock;
        let mut charBaseBlock = charBaseBlock;
        LoadPalette(
            (((&raw const sLinkTestDigitsPal)
                .cast::<u8>()
                .cast_mut()
                .cast::<u16>())
            .cast::<u16>())
            .cast::<u8>(),
            (((0i32).wrapping_add(((paletteNum) as i32).wrapping_mul(16i32))) as u16),
            32u16,
        );
        'l1: loop {
            'l2: {
                'l3: loop {
                    'l4: {
                        {
                            let mut dmaRegs: *mut u32 = ((67109076i32) as usize as *mut u32);
                            crate::c::volatile_write(
                                dmaRegs,
                                ((((&raw const sLinkTestDigitsGfx)
                                    .cast::<u8>()
                                    .cast_mut()
                                    .cast::<u16>())
                                .cast::<u16>()) as usize as u32),
                            );
                            crate::c::volatile_write(
                                (dmaRegs).wrapping_offset(1),
                                ((((100663296i32).wrapping_add(
                                    (16384i32).wrapping_mul(((charBaseBlock) as i32)),
                                )) as usize as *mut u16) as usize
                                    as u32),
                            );
                            crate::c::volatile_write(
                                (dmaRegs).wrapping_offset(2),
                                (2147483648u32
                                    | crate::c::div_u32(
                                        544u32,
                                        ((crate::c::div_i32(16i32, 8i32)) as u32),
                                    )),
                            );
                            let _ = ((dmaRegs).wrapping_offset(2)).read_volatile();
                        }
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
        (((&raw mut gLinkTestBGInfo).cast::<u8>()).cast::<u32>()).write(((screenBaseBlock) as u32));
        (((&raw mut gLinkTestBGInfo).cast::<u8>())
            .wrapping_add(4)
            .cast::<u32>())
        .write(((paletteNum) as u32));
        (((&raw mut gLinkTestBGInfo).cast::<u8>())
            .wrapping_add(8)
            .cast::<u32>())
        .write(0u32);
        SetGpuReg(
            ((((&raw const sBGControlRegs).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((bgNum) as i32) as isize))
            .read(),
            (((((screenBaseBlock) as i32) << 8) | (((charBaseBlock) as i32) << 2)) as u16),
        );
    }
}
pub(crate) unsafe extern "C" fn LinkTestScreen() {
    unsafe {
        let mut i: i32 = 0i32;
        ResetSpriteData();
        FreeAllSpritePalettes();
        ResetTasks();
        SetVBlankCallback(Some(VBlankCB_LinkError));
        ResetBlockSend();
        ((&raw mut gLinkType).cast::<u8>().cast::<u16>()).write(4369u16);
        OpenLink();
        SeedRng(
            (((((&raw mut gMain).cast::<u8>())
                .wrapping_add(36)
                .cast::<u32>())
            .read()) as u16),
        );
        {
            i = 0i32;
            'l1: loop {
                if !(i < 4i32) {
                    break 'l1;
                }
                'l2: {
                    ((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(10))
                        .cast::<u8>())
                    .wrapping_offset((i) as isize))
                    .write(((crate::c::rem_i32(((Random()) as i32), 256i32)) as u8));
                }
                i = (i).wrapping_add(1);
            }
        }
        InitLinkTestBG(0u8, 2u8, 4u8, 0u8, 0u16);
        SetGpuReg(0u8, 5440u16);
        CreateTask(Some(Task_DestroySelf), 0u8);
        RunTasks();
        AnimateSprites();
        BuildOamBuffer();
        UpdatePaletteFade();
        ((&raw mut sDummy3).cast::<u8>().cast::<u32>()).write(0u32);
        InitLocalLinkPlayer();
        CreateTask(Some(Task_PrintTestData), 0u8);
        SetMainCallback2(Some(CB2_LinkTest));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetLocalLinkPlayerId(playerId: u8) {
    unsafe {
        let mut playerId = playerId;
        (((&raw mut gLocalLinkPlayer).cast::<u8>())
            .wrapping_add(24)
            .cast::<u16>())
        .write(((playerId) as u16));
    }
}
pub(crate) unsafe extern "C" fn InitLocalLinkPlayer() {
    unsafe {
        (((&raw mut gLocalLinkPlayer).cast::<u8>())
            .wrapping_add(4)
            .cast::<u32>())
        .write(
            (((((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(10))
                .cast::<u8>())
            .read()) as i32)
                | (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(10))
                    .cast::<u8>())
                .wrapping_offset(1))
                .read()) as i32)
                    << 8))
                | (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(10))
                    .cast::<u8>())
                .wrapping_offset(2))
                .read()) as i32)
                    << 16))
                | (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(10))
                    .cast::<u8>())
                .wrapping_offset(3))
                .read()) as i32)
                    << 24)) as u32),
        );
        StringCopy(
            (((&raw mut gLocalLinkPlayer).cast::<u8>()).wrapping_add(8)).cast::<u8>(),
            (((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).cast::<u8>(),
        );
        (((&raw mut gLocalLinkPlayer).cast::<u8>()).wrapping_add(19))
            .write(((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(8)).read());
        (((&raw mut gLocalLinkPlayer).cast::<u8>())
            .wrapping_add(20)
            .cast::<u32>())
        .write(((((&raw mut gLinkType).cast::<u8>().cast::<u16>()).read()) as u32));
        (((&raw mut gLocalLinkPlayer).cast::<u8>())
            .wrapping_add(26)
            .cast::<u16>())
        .write(((((&raw mut gGameLanguage).cast::<u8>()).read()) as u16));
        (((&raw mut gLocalLinkPlayer).cast::<u8>()).cast::<u16>()).write(
            ((((((&raw mut gGameVersion).cast::<u8>()).read()) as i32).wrapping_add(16384i32))
                as u16),
        );
        (((&raw mut gLocalLinkPlayer).cast::<u8>())
            .wrapping_add(2)
            .cast::<u16>())
        .write(32768u16);
        (((&raw mut gLocalLinkPlayer).cast::<u8>()).wrapping_add(16))
            .write(((IsNationalPokedexEnabled()) as u8));
        if (FlagGet(2175u16)) != 0 {
            let __p1 = ((&raw mut gLocalLinkPlayer).cast::<u8>()).wrapping_add(16);
            (__p1).write((((((__p1).read()) as i32) | 16i32) as u8));
        }
    }
}
pub(crate) unsafe extern "C" fn VBlankCB_LinkError() {
    unsafe {
        LoadOam();
        ProcessSpriteCopyRequests();
        TransferPlttBuffer();
    }
}
pub(crate) unsafe extern "C" fn InitLink() {
    unsafe {
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 8i32) {
                    break 'l1;
                }
                'l2: {
                    ((((&raw mut gSendCmd).cast::<u8>().cast::<u16>()).cast::<u16>())
                        .wrapping_offset((i) as isize))
                    .write(61439u16);
                }
                i = (i).wrapping_add(1);
            }
        }
        ((&raw mut sLinkOpen).cast::<u8>().cast::<u8>()).write(1u8);
        EnableSerial();
    }
}
pub(crate) unsafe extern "C" fn Task_TriggerHandshake(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if (({
            let __p1 = ((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>();
            let __t2 = ((__p1).read()).wrapping_add(1);
            (__p1).write(__t2);
            __t2
        }) as i32)
            == 5i32
        {
            ((&raw mut gShouldAdvanceLinkState).cast::<u8>().cast::<u8>()).write(1u8);
            DestroyTask(taskId);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn OpenLink() {
    unsafe {
        let mut i: i32 = 0i32;
        if !((((&raw mut gWirelessCommType).cast::<u8>().cast::<u8>()).read()) != 0) {
            ResetSerial();
            InitLink();
            ((&raw mut gLinkCallback)
                .cast::<u8>()
                .cast::<Option<unsafe extern "C" fn()>>())
            .write(Some(LinkCB_RequestPlayerDataExchange));
            ((&raw mut gLinkVSyncDisabled).cast::<u8>()).write(0u8);
            ((&raw mut gLinkErrorOccurred).cast::<u8>().cast::<u8>()).write(0u8);
            ((&raw mut gSuppressLinkErrorMessage)
                .cast::<u8>()
                .cast::<u8>())
            .write(0u8);
            ResetBlockReceivedFlags();
            ResetBlockSend();
            ((&raw mut sDummy1).cast::<u8>().cast::<u32>()).write(0u32);
            ((&raw mut gLinkDummy2).cast::<u8>().cast::<u8>()).write(0u8);
            ((&raw mut gLinkDummy1).cast::<u8>().cast::<u8>()).write(0u8);
            ((&raw mut gReadyCloseLinkType).cast::<u8>().cast::<u16>()).write(0u16);
            CreateTask(Some(Task_TriggerHandshake), 2u8);
        } else {
            InitRFUAPI();
        }
        ((&raw mut gReceivedRemoteLinkPlayers)
            .cast::<u8>()
            .cast::<u8>())
        .write(0u8);
        {
            i = 0i32;
            'l1: loop {
                if !(i < 4i32) {
                    break 'l1;
                }
                'l2: {
                    ((((&raw mut gRemoteLinkPlayersNotReceived).cast::<u8>()).cast::<u8>())
                        .wrapping_offset((i) as isize))
                    .write(1u8);
                    ((((&raw mut gReadyToCloseLink).cast::<u8>()).cast::<u8>())
                        .wrapping_offset((i) as isize))
                    .write(0u8);
                    ((((&raw mut gReadyToExitStandby).cast::<u8>()).cast::<u8>())
                        .wrapping_offset((i) as isize))
                    .write(0u8);
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CloseLink() {
    unsafe {
        ((&raw mut gReceivedRemoteLinkPlayers)
            .cast::<u8>()
            .cast::<u8>())
        .write(0u8);
        if (((&raw mut gWirelessCommType).cast::<u8>().cast::<u8>()).read()) != 0 {
            LinkRfu_Shutdown();
        }
        ((&raw mut sLinkOpen).cast::<u8>().cast::<u8>()).write(0u8);
        DisableSerial();
    }
}
pub(crate) unsafe extern "C" fn TestBlockTransfer(nothing: u8, is: u8, used: u8) {
    unsafe {
        let mut nothing = nothing;
        let mut is = is;
        let mut used = used;
        let mut i: u8 = 0u8;
        let mut status: u8 = 0u8;
        if ((((&raw mut sLinkTestLastBlockSendPos)
            .cast::<u8>()
            .cast::<u8>())
        .read()) as i32)
            != (((((&raw mut sBlockSend).cast::<u8>()).cast::<u16>()).read()) as i32)
        {
            LinkTest_PrintHex(
                (((((&raw mut sBlockSend).cast::<u8>()).cast::<u16>()).read()) as u32),
                2u8,
                3u8,
                2u8,
            );
            ((&raw mut sLinkTestLastBlockSendPos)
                .cast::<u8>()
                .cast::<u8>())
            .write((((((&raw mut sBlockSend).cast::<u8>()).cast::<u16>()).read()) as u8));
        }
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 4i32) {
                    break 'l1;
                }
                'l2: {
                    if ((((((&raw mut sLinkTestLastBlockRecvPos).cast::<u8>()).cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize))
                    .read()) as i32)
                        != (((((((&raw mut sBlockRecv).cast::<u8>()).cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 12))
                        .cast::<u16>())
                        .read()) as i32)
                    {
                        LinkTest_PrintHex(
                            (((((((&raw mut sBlockRecv).cast::<u8>()).cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize * 12))
                            .cast::<u16>())
                            .read()) as u32),
                            2u8,
                            ((((i) as i32).wrapping_add(4i32)) as u8),
                            2u8,
                        );
                        ((((&raw mut sLinkTestLastBlockRecvPos).cast::<u8>()).cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize))
                        .write(
                            (((((((&raw mut sBlockRecv).cast::<u8>()).cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize * 12))
                            .cast::<u16>())
                            .read()) as u8),
                        );
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        status = GetBlockReceivedStatus();
        if ((status) as i32) == 15i32 {
            {
                i = 0u8;
                'l3: loop {
                    if !(((i) as i32) < 4i32) {
                        break 'l3;
                    }
                    'l4: {
                        if (crate::c::shr_i32(((status) as i32), ((i) as u32)) & 1i32) != 0 {
                            ((((&raw mut gLinkTestBlockChecksums)
                                .cast::<u8>()
                                .cast::<u16>())
                            .cast::<u16>())
                            .wrapping_offset(((i) as i32) as isize))
                            .write(LinkTestCalcBlockChecksum(
                                ((((&raw mut gBlockRecvBuffer).cast::<u8>()).cast::<u8>())
                                    .wrapping_offset(((i) as i32) as isize * 256))
                                .cast::<u16>(),
                                (((((&raw mut sBlockRecv).cast::<u8>()).cast::<u8>())
                                    .wrapping_offset(((i) as i32) as isize * 12))
                                .wrapping_add(2)
                                .cast::<u16>())
                                .read(),
                            ));
                            ResetBlockReceivedFlag(i);
                            if ((((((&raw mut gLinkTestBlockChecksums)
                                .cast::<u8>()
                                .cast::<u16>())
                            .cast::<u16>())
                            .wrapping_offset(((i) as i32) as isize))
                            .read()) as i32)
                                != 834i32
                            {
                                ((&raw mut sLinkTestDebugValuesEnabled)
                                    .cast::<u8>()
                                    .cast::<u8>())
                                .write(0u8);
                                ((&raw mut sDummyFlag).cast::<u8>().cast::<u8>()).write(0u8);
                            }
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn LinkTestProcessKeyInput() {
    unsafe {
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(46)
            .cast::<u16>())
        .read()) as i32)
            & 1i32)
            != 0
        {
            ((&raw mut gShouldAdvanceLinkState).cast::<u8>().cast::<u8>()).write(1u8);
        }
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(44)
            .cast::<u16>())
        .read()) as i32)
            & 2i32)
            != 0
        {
            InitBlockSend(
                ((&raw mut gHeap).cast::<u8>()).wrapping_offset(16384),
                8196u32,
            );
        }
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(46)
            .cast::<u16>())
        .read()) as i32)
            & 512i32)
            != 0
        {
            BeginNormalPaletteFade(4294967295u32, 0i8, 16u8, 0u8, 2u16);
        }
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(46)
            .cast::<u16>())
        .read()) as i32)
            & 8i32)
            != 0
        {
            SetSuppressLinkErrorMessage(1u8);
        }
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(46)
            .cast::<u16>())
        .read()) as i32)
            & 256i32)
            != 0
        {
            TrySavingData(1u8);
        }
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(46)
            .cast::<u16>())
        .read()) as i32)
            & 4i32)
            != 0
        {
            SetCloseLinkCallback();
        }
        if (((&raw mut sLinkTestDebugValuesEnabled)
            .cast::<u8>()
            .cast::<u8>())
        .read())
            != 0
        {
            SetLinkDebugValues(
                (((&raw mut gMain).cast::<u8>())
                    .wrapping_add(36)
                    .cast::<u32>())
                .read(),
                ((if (((&raw mut gLinkCallback)
                    .cast::<u8>()
                    .cast::<Option<unsafe extern "C" fn()>>())
                .read())
                .is_some()
                {
                    ((((&raw mut gLinkVSyncDisabled).cast::<u8>()).read()) as i32)
                } else {
                    (((((&raw mut gLinkVSyncDisabled).cast::<u8>()).read()) as i32) | 16i32)
                }) as u32),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn CB2_LinkTest() {
    unsafe {
        LinkTestProcessKeyInput();
        TestBlockTransfer(1u8, 1u8, 0u8);
        RunTasks();
        AnimateSprites();
        BuildOamBuffer();
        UpdatePaletteFade();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn LinkMain2(heldKeys: *mut u16) -> u16 {
    unsafe {
        let mut heldKeys = heldKeys;
        let mut i: u8 = 0u8;
        if !((((&raw mut sLinkOpen).cast::<u8>().cast::<u8>()).read()) != 0) {
            return 0u16;
        }
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 8i32) {
                    break 'l1;
                }
                'l2: {
                    ((((&raw mut gSendCmd).cast::<u8>().cast::<u16>()).cast::<u16>())
                        .wrapping_offset(((i) as i32) as isize))
                    .write(0u16);
                }
                i = (i).wrapping_add(1);
            }
        }
        ((&raw mut gLinkHeldKeys).cast::<u8>().cast::<u16>()).write((heldKeys).read());
        if (((&raw mut gLinkStatus).cast::<u8>().cast::<u32>()).read() & 64u32) != 0 {
            ProcessRecvCmds(
                ((crate::c::bf_read(
                    ((67109160i32) as usize as *mut u8).wrapping_add(0),
                    4,
                    2,
                    false,
                ) as u16) as u8),
            );
            if core::mem::transmute::<_, usize>(
                ((&raw mut gLinkCallback)
                    .cast::<u8>()
                    .cast::<Option<unsafe extern "C" fn()>>())
                .read(),
            ) != 0usize
            {
                (((&raw mut gLinkCallback)
                    .cast::<u8>()
                    .cast::<Option<unsafe extern "C" fn()>>())
                .read())
                .unwrap_unchecked()();
            }
            TrySetLinkErrorBuffer();
        }
        return ((((&raw mut gLinkStatus).cast::<u8>().cast::<u32>()).read()) as u16);
    }
}
pub(crate) unsafe extern "C" fn HandleReceiveRemoteLinkPlayer(who: u8) {
    unsafe {
        let mut who = who;
        let mut i: i32 = 0i32;
        let mut count: i32 = 0i32;
        count = 0i32;
        ((((&raw mut gRemoteLinkPlayersNotReceived).cast::<u8>()).cast::<u8>())
            .wrapping_offset(((who) as i32) as isize))
        .write(0u8);
        {
            i = 0i32;
            'l1: loop {
                if !(i < ((GetLinkPlayerCount_2()) as i32)) {
                    break 'l1;
                }
                'l2: {
                    count = (count).wrapping_add(
                        ((((((&raw mut gRemoteLinkPlayersNotReceived).cast::<u8>()).cast::<u8>())
                            .wrapping_offset((i) as isize))
                        .read()) as i32),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        if (count == 0i32)
            && (((((&raw mut gReceivedRemoteLinkPlayers)
                .cast::<u8>()
                .cast::<u8>())
            .read()) as i32)
                == 0i32)
        {
            ((&raw mut gReceivedRemoteLinkPlayers)
                .cast::<u8>()
                .cast::<u8>())
            .write(1u8);
        }
    }
}
pub(crate) unsafe extern "C" fn ProcessRecvCmds(unused: u8) {
    unsafe {
        let mut unused = unused;
        let mut i: u16 = 0u16;
        {
            i = 0u16;
            'l1: loop {
                if !(((i) as i32) < 4i32) {
                    break 'l1;
                }
                'l2: {
                    ((((&raw mut gLinkPartnersHeldKeys).cast::<u8>().cast::<u16>()).cast::<u16>())
                        .wrapping_offset(((i) as i32) as isize))
                    .write(0u16);
                    if (((((((&raw mut gRecvCmds).cast::<u8>()).cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 16))
                    .cast::<u16>())
                    .read()) as i32)
                        == 0i32
                    {
                        break 'l2;
                    }
                    'l3: {
                        let __sw1 = (((((((&raw mut gRecvCmds).cast::<u8>()).cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 16))
                        .cast::<u16>())
                        .read()) as i32);
                        if __sw1 == 8738i32 {
                            {
                                let mut block: *mut u8 = core::ptr::null_mut();
                                InitLocalLinkPlayer();
                                block = (&raw mut gLocalLinkPlayerBlock).cast::<u8>();
                                (block)
                                    .wrapping_add(16)
                                    .cast::<crate::c::Rec4<28>>()
                                    .write_unaligned(
                                        (&raw mut gLocalLinkPlayer)
                                            .cast::<u8>()
                                            .cast::<crate::c::Rec4<28>>()
                                            .read_unaligned(),
                                    );
                                crate::c::memcpy(
                                    (block).cast::<u8>(),
                                    ((&raw const sASCIIGameFreakInc).cast::<u8>().cast_mut())
                                        .cast::<u8>(),
                                    15u32,
                                );
                                crate::c::memcpy(
                                    ((block).wrapping_add(44)).cast::<u8>(),
                                    ((&raw const sASCIIGameFreakInc).cast::<u8>().cast_mut())
                                        .cast::<u8>(),
                                    15u32,
                                );
                                InitBlockSend(block, 60u32);
                                break 'l3;
                            }
                        }
                        if __sw1 == 17476i32 {
                            ((((&raw mut gLinkPartnersHeldKeys).cast::<u8>().cast::<u16>())
                                .cast::<u16>())
                            .wrapping_offset(((i) as i32) as isize))
                            .write(
                                ((((((&raw mut gRecvCmds).cast::<u8>()).cast::<u8>())
                                    .wrapping_offset(((i) as i32) as isize * 16))
                                .cast::<u16>())
                                .wrapping_offset(1))
                                .read(),
                            );
                            break 'l3;
                        }
                        if __sw1 == 21845i32 {
                            ((&raw mut gLinkDummy2).cast::<u8>().cast::<u8>()).write(1u8);
                            break 'l3;
                        }
                        if __sw1 == 21862i32 {
                            ((&raw mut gLinkDummy2).cast::<u8>().cast::<u8>()).write(1u8);
                            break 'l3;
                        }
                        if __sw1 == 48059i32 {
                            {
                                let mut blockRecv: *mut u8 = core::ptr::null_mut();
                                blockRecv = (((&raw mut sBlockRecv).cast::<u8>()).cast::<u8>())
                                    .wrapping_offset(((i) as i32) as isize * 12);
                                ((blockRecv).cast::<u16>()).write(0u16);
                                ((blockRecv).wrapping_add(2).cast::<u16>()).write(
                                    ((((((&raw mut gRecvCmds).cast::<u8>()).cast::<u8>())
                                        .wrapping_offset(((i) as i32) as isize * 16))
                                    .cast::<u16>())
                                    .wrapping_offset(1))
                                    .read(),
                                );
                                ((blockRecv).wrapping_add(9)).write(
                                    ((((((((&raw mut gRecvCmds).cast::<u8>()).cast::<u8>())
                                        .wrapping_offset(((i) as i32) as isize * 16))
                                    .cast::<u16>())
                                    .wrapping_offset(2))
                                    .read()) as u8),
                                );
                                break 'l3;
                            }
                        }
                        if __sw1 == 34952i32 {
                            {
                                if (((((((&raw mut sBlockRecv).cast::<u8>()).cast::<u8>())
                                    .wrapping_offset(((i) as i32) as isize * 12))
                                .wrapping_add(2)
                                .cast::<u16>())
                                .read()) as i32)
                                    > 256i32
                                {
                                    let mut buffer: *mut u16 = core::ptr::null_mut();
                                    let mut j: u16 = 0u16;
                                    buffer = ((&raw mut gDecompressionBuffer).cast::<u8>())
                                        .cast::<u16>();
                                    {
                                        j = 0u16;
                                        'l4: loop {
                                            if !(((j) as i32) < 7i32) {
                                                break 'l4;
                                            }
                                            'l5: {
                                                ((buffer).wrapping_offset(
                                                    ((crate::c::div_i32(
                                                        (((((((&raw mut sBlockRecv)
                                                            .cast::<u8>())
                                                        .cast::<u8>())
                                                        .wrapping_offset(
                                                            ((i) as i32) as isize * 12,
                                                        ))
                                                        .cast::<u16>())
                                                        .read())
                                                            as i32),
                                                        2i32,
                                                    ))
                                                    .wrapping_add(((j) as i32)))
                                                        as isize,
                                                ))
                                                .write(
                                                    ((((((&raw mut gRecvCmds).cast::<u8>())
                                                        .cast::<u8>())
                                                    .wrapping_offset(((i) as i32) as isize * 16))
                                                    .cast::<u16>())
                                                    .wrapping_offset(
                                                        (((j) as i32).wrapping_add(1i32)) as isize,
                                                    ))
                                                    .read(),
                                                );
                                            }
                                            j = (j).wrapping_add(1);
                                        }
                                    }
                                } else {
                                    let mut j: u16 = 0u16;
                                    {
                                        j = 0u16;
                                        'l6: loop {
                                            if !(((j) as i32) < 7i32) {
                                                break 'l6;
                                            }
                                            'l7: {
                                                ((((((&raw mut gBlockRecvBuffer).cast::<u8>())
                                                    .cast::<u8>())
                                                .wrapping_offset(((i) as i32) as isize * 256))
                                                .cast::<u16>())
                                                .wrapping_offset(
                                                    ((crate::c::div_i32(
                                                        (((((((&raw mut sBlockRecv)
                                                            .cast::<u8>())
                                                        .cast::<u8>())
                                                        .wrapping_offset(
                                                            ((i) as i32) as isize * 12,
                                                        ))
                                                        .cast::<u16>())
                                                        .read())
                                                            as i32),
                                                        2i32,
                                                    ))
                                                    .wrapping_add(((j) as i32)))
                                                        as isize,
                                                ))
                                                .write(
                                                    ((((((&raw mut gRecvCmds).cast::<u8>())
                                                        .cast::<u8>())
                                                    .wrapping_offset(((i) as i32) as isize * 16))
                                                    .cast::<u16>())
                                                    .wrapping_offset(
                                                        (((j) as i32).wrapping_add(1i32)) as isize,
                                                    ))
                                                    .read(),
                                                );
                                            }
                                            j = (j).wrapping_add(1);
                                        }
                                    }
                                }
                                let __p2 = ((((&raw mut sBlockRecv).cast::<u8>()).cast::<u8>())
                                    .wrapping_offset(((i) as i32) as isize * 12))
                                .cast::<u16>();
                                (__p2)
                                    .write((((((__p2).read()) as i32).wrapping_add(14i32)) as u16));
                                if (((((((&raw mut sBlockRecv).cast::<u8>()).cast::<u8>())
                                    .wrapping_offset(((i) as i32) as isize * 12))
                                .cast::<u16>())
                                .read()) as i32)
                                    >= (((((((&raw mut sBlockRecv).cast::<u8>()).cast::<u8>())
                                        .wrapping_offset(((i) as i32) as isize * 12))
                                    .wrapping_add(2)
                                    .cast::<u16>())
                                    .read()) as i32)
                                {
                                    if ((((((&raw mut gRemoteLinkPlayersNotReceived)
                                        .cast::<u8>())
                                    .cast::<u8>())
                                    .wrapping_offset(((i) as i32) as isize))
                                    .read()) as i32)
                                        == 1i32
                                    {
                                        let mut block: *mut u8 = core::ptr::null_mut();
                                        let mut linkPlayer: *mut u8 = core::ptr::null_mut();
                                        block = (((((&raw mut gBlockRecvBuffer).cast::<u8>())
                                            .cast::<u8>())
                                        .wrapping_offset(((i) as i32) as isize * 256))
                                        .cast::<u16>())
                                        .cast::<u8>();
                                        linkPlayer = (((&raw mut gLinkPlayers).cast::<u8>())
                                            .cast::<u8>())
                                        .wrapping_offset(((i) as i32) as isize * 28);
                                        linkPlayer.cast::<crate::c::Rec4<28>>().write_unaligned(
                                            (block)
                                                .wrapping_add(16)
                                                .cast::<crate::c::Rec4<28>>()
                                                .read_unaligned(),
                                        );
                                        if ((((((linkPlayer).cast::<u16>()).read()) as i32)
                                            & 255i32)
                                            == 2i32)
                                            || ((((((linkPlayer).cast::<u16>()).read()) as i32)
                                                & 255i32)
                                                == 1i32)
                                        {
                                            ((linkPlayer).wrapping_add(18)).write(0u8);
                                            ((linkPlayer).wrapping_add(17)).write(0u8);
                                            ((linkPlayer).wrapping_add(16)).write(0u8);
                                        }
                                        ConvertLinkPlayerName(linkPlayer);
                                        if (crate::c::strcmp(
                                            (block).cast::<u8>(),
                                            ((&raw const sASCIIGameFreakInc)
                                                .cast::<u8>()
                                                .cast_mut())
                                            .cast::<u8>(),
                                        ) != 0i32)
                                            || (crate::c::strcmp(
                                                ((block).wrapping_add(44)).cast::<u8>(),
                                                ((&raw const sASCIIGameFreakInc)
                                                    .cast::<u8>()
                                                    .cast_mut())
                                                .cast::<u8>(),
                                            ) != 0i32)
                                        {
                                            SetMainCallback2(Some(CB2_LinkError));
                                        } else {
                                            HandleReceiveRemoteLinkPlayer(((i) as u8));
                                        }
                                    } else {
                                        SetBlockReceivedFlag(((i) as u8));
                                    }
                                }
                            }
                            break 'l3;
                        }
                        if __sw1 == 24575i32 {
                            ((((&raw mut gReadyToCloseLink).cast::<u8>()).cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize))
                            .write(1u8);
                            break 'l3;
                        }
                        if __sw1 == 12286i32 {
                            ((((&raw mut gReadyToExitStandby).cast::<u8>()).cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize))
                            .write(1u8);
                            break 'l3;
                        }
                        if __sw1 == 43690i32 {
                            SetBerryBlenderLinkCallback();
                            break 'l3;
                        }
                        if __sw1 == 52428i32 {
                            SendBlock(
                                0u8,
                                (((((&raw const sBlockRequests).cast::<u8>().cast_mut())
                                    .cast::<u8>())
                                .wrapping_offset(
                                    ((((((((&raw mut gRecvCmds).cast::<u8>()).cast::<u8>())
                                        .wrapping_offset(((i) as i32) as isize * 16))
                                    .cast::<u16>())
                                    .wrapping_offset(1))
                                    .read()) as i32) as isize
                                        * 8,
                                ))
                                .cast::<*mut u8>())
                                .read(),
                                (((((((&raw const sBlockRequests).cast::<u8>().cast_mut())
                                    .cast::<u8>())
                                .wrapping_offset(
                                    ((((((((&raw mut gRecvCmds).cast::<u8>()).cast::<u8>())
                                        .wrapping_offset(((i) as i32) as isize * 16))
                                    .cast::<u16>())
                                    .wrapping_offset(1))
                                    .read()) as i32) as isize
                                        * 8,
                                ))
                                .wrapping_add(4)
                                .cast::<u32>())
                                .read()) as u16),
                            );
                            break 'l3;
                        }
                        if __sw1 == 51966i32 {
                            ((((&raw mut gLinkPartnersHeldKeys).cast::<u8>().cast::<u16>())
                                .cast::<u16>())
                            .wrapping_offset(((i) as i32) as isize))
                            .write(
                                ((((((&raw mut gRecvCmds).cast::<u8>()).cast::<u8>())
                                    .wrapping_offset(((i) as i32) as isize * 16))
                                .cast::<u16>())
                                .wrapping_offset(1))
                                .read(),
                            );
                            break 'l3;
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn BuildSendCmd(command: u16) {
    unsafe {
        let mut command = command;
        'l1: {
            let __sw1 = ((command) as i32);
            if __sw1 == 8738i32 {
                (((&raw mut gSendCmd).cast::<u8>().cast::<u16>()).cast::<u16>()).write(8738u16);
                ((((&raw mut gSendCmd).cast::<u8>().cast::<u16>()).cast::<u16>())
                    .wrapping_offset(1))
                .write(((&raw mut gLinkType).cast::<u8>().cast::<u16>()).read());
                break 'l1;
            }
            if __sw1 == 12286i32 {
                (((&raw mut gSendCmd).cast::<u8>().cast::<u16>()).cast::<u16>()).write(12286u16);
                break 'l1;
            }
            if __sw1 == 17476i32 {
                (((&raw mut gSendCmd).cast::<u8>().cast::<u16>()).cast::<u16>()).write(17476u16);
                ((((&raw mut gSendCmd).cast::<u8>().cast::<u16>()).cast::<u16>())
                    .wrapping_offset(1))
                .write(
                    (((&raw mut gMain).cast::<u8>())
                        .wrapping_add(44)
                        .cast::<u16>())
                    .read(),
                );
                break 'l1;
            }
            if __sw1 == 21845i32 {
                (((&raw mut gSendCmd).cast::<u8>().cast::<u16>()).cast::<u16>()).write(21845u16);
                break 'l1;
            }
            if __sw1 == 26214i32 {
                (((&raw mut gSendCmd).cast::<u8>().cast::<u16>()).cast::<u16>()).write(26214u16);
                ((((&raw mut gSendCmd).cast::<u8>().cast::<u16>()).cast::<u16>())
                    .wrapping_offset(1))
                .write(0u16);
                break 'l1;
            }
            if __sw1 == 30583i32 {
                {
                    let mut i: u8 = 0u8;
                    (((&raw mut gSendCmd).cast::<u8>().cast::<u16>()).cast::<u16>())
                        .write(30583u16);
                    {
                        i = 0u8;
                        'l2: loop {
                            if !(((i) as i32) < 5i32) {
                                break 'l2;
                            }
                            'l3: {
                                ((((&raw mut gSendCmd).cast::<u8>().cast::<u16>()).cast::<u16>())
                                    .wrapping_offset((((i) as i32).wrapping_add(1i32)) as isize))
                                .write(238u16);
                            }
                            i = (i).wrapping_add(1);
                        }
                    }
                    break 'l1;
                }
            }
            if __sw1 == 48059i32 {
                (((&raw mut gSendCmd).cast::<u8>().cast::<u16>()).cast::<u16>()).write(48059u16);
                ((((&raw mut gSendCmd).cast::<u8>().cast::<u16>()).cast::<u16>())
                    .wrapping_offset(1))
                .write(
                    (((&raw mut sBlockSend).cast::<u8>())
                        .wrapping_add(2)
                        .cast::<u16>())
                    .read(),
                );
                ((((&raw mut gSendCmd).cast::<u8>().cast::<u16>()).cast::<u16>())
                    .wrapping_offset(2))
                .write(
                    (((((((&raw mut sBlockSend).cast::<u8>()).wrapping_add(9)).read()) as i32)
                        .wrapping_add(128i32)) as u16),
                );
                break 'l1;
            }
            if __sw1 == 43690i32 {
                (((&raw mut gSendCmd).cast::<u8>().cast::<u16>()).cast::<u16>()).write(43690u16);
                break 'l1;
            }
            if __sw1 == 43691i32 {
                (((&raw mut gSendCmd).cast::<u8>().cast::<u16>()).cast::<u16>()).write(43691u16);
                ((((&raw mut gSendCmd).cast::<u8>().cast::<u16>()).cast::<u16>())
                    .wrapping_offset(1))
                .write(((&raw mut gSpecialVar_ItemId).cast::<u16>()).read());
                break 'l1;
            }
            if __sw1 == 52428i32 {
                (((&raw mut gSendCmd).cast::<u8>().cast::<u16>()).cast::<u16>()).write(52428u16);
                ((((&raw mut gSendCmd).cast::<u8>().cast::<u16>()).cast::<u16>())
                    .wrapping_offset(1))
                .write(((((&raw mut gBlockRequestType).cast::<u8>().cast::<u8>()).read()) as u16));
                break 'l1;
            }
            if __sw1 == 24575i32 {
                (((&raw mut gSendCmd).cast::<u8>().cast::<u16>()).cast::<u16>()).write(24575u16);
                ((((&raw mut gSendCmd).cast::<u8>().cast::<u16>()).cast::<u16>())
                    .wrapping_offset(1))
                .write(((&raw mut gReadyCloseLinkType).cast::<u8>().cast::<u16>()).read());
                break 'l1;
            }
            if __sw1 == 21862i32 {
                (((&raw mut gSendCmd).cast::<u8>().cast::<u16>()).cast::<u16>()).write(21862u16);
                break 'l1;
            }
            if __sw1 == 51966i32 {
                if (((((&raw mut gHeldKeyCodeToSend).cast::<u16>()).read()) as i32) == 0i32)
                    || ((((&raw mut gLinkTransferringData).cast::<u8>()).read()) != 0)
                {
                    break 'l1;
                }
                (((&raw mut gSendCmd).cast::<u8>().cast::<u16>()).cast::<u16>()).write(51966u16);
                ((((&raw mut gSendCmd).cast::<u8>().cast::<u16>()).cast::<u16>())
                    .wrapping_offset(1))
                .write(((&raw mut gHeldKeyCodeToSend).cast::<u16>()).read());
                break 'l1;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn StartSendingKeysToLink() {
    unsafe {
        if (((&raw mut gWirelessCommType).cast::<u8>().cast::<u8>()).read()) != 0 {
            StartSendingKeysToRfu();
        }
        ((&raw mut gLinkCallback)
            .cast::<u8>()
            .cast::<Option<unsafe extern "C" fn()>>())
        .write(Some(LinkCB_SendHeldKeys));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsSendingKeysToLink() -> u32 {
    unsafe {
        if (((&raw mut gWirelessCommType).cast::<u8>().cast::<u8>()).read()) != 0 {
            return IsSendingKeysToRfu();
        }
        if core::mem::transmute::<_, usize>(
            ((&raw mut gLinkCallback)
                .cast::<u8>()
                .cast::<Option<unsafe extern "C" fn()>>())
            .read(),
        ) == (LinkCB_SendHeldKeys as *const () as usize)
        {
            return 1u32;
        }
        return 0u32;
    }
}
pub(crate) unsafe extern "C" fn LinkCB_SendHeldKeys() {
    unsafe {
        if ((((&raw mut gReceivedRemoteLinkPlayers)
            .cast::<u8>()
            .cast::<u8>())
        .read()) as i32)
            == 1i32
        {
            BuildSendCmd(51966u16);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ClearLinkCallback() {
    unsafe {
        if (((&raw mut gWirelessCommType).cast::<u8>().cast::<u8>()).read()) != 0 {
            ClearLinkRfuCallback();
        } else {
            ((&raw mut gLinkCallback)
                .cast::<u8>()
                .cast::<Option<unsafe extern "C" fn()>>())
            .write(None);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ClearLinkCallback_2() {
    unsafe {
        if (((&raw mut gWirelessCommType).cast::<u8>().cast::<u8>()).read()) != 0 {
            ClearLinkRfuCallback();
        } else {
            ((&raw mut gLinkCallback)
                .cast::<u8>()
                .cast::<Option<unsafe extern "C" fn()>>())
            .write(None);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetLinkPlayerCount() -> u8 {
    unsafe {
        if (((&raw mut gWirelessCommType).cast::<u8>().cast::<u8>()).read()) != 0 {
            return Rfu_GetLinkPlayerCount();
        }
        return (((((&raw mut gLinkStatus).cast::<u8>().cast::<u32>()).read() & 28u32) >> 2) as u8);
    }
}
pub(crate) unsafe extern "C" fn AreAnyLinkPlayersUsingVersions(
    version1: u32,
    version2: u32,
) -> i32 {
    unsafe {
        let mut version1 = version1;
        let mut version2 = version2;
        let mut i: i32 = 0i32;
        let mut nPlayers: u8 = 0u8;
        nPlayers = GetLinkPlayerCount();
        {
            i = 0i32;
            'l1: loop {
                if !(i < ((nPlayers) as i32)) {
                    break 'l1;
                }
                'l2: {
                    if ((((((((((&raw mut gLinkPlayers).cast::<u8>()).cast::<u8>())
                        .wrapping_offset((i) as isize * 28))
                    .cast::<u16>())
                    .read()) as i32)
                        & 255i32) as u32)
                        == version1)
                        || ((((((((((&raw mut gLinkPlayers).cast::<u8>()).cast::<u8>())
                            .wrapping_offset((i) as isize * 28))
                        .cast::<u16>())
                        .read()) as i32)
                            & 255i32) as u32)
                            == version2)
                    {
                        return 1i32;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return (-1i32);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn LinkDummy_Return2() -> u32 {
    unsafe {
        return 2u32;
    }
}
pub(crate) unsafe extern "C" fn IsFullLinkGroupWithNoRS() -> u32 {
    unsafe {
        if (((GetLinkPlayerCount()) as i32) != 4i32)
            || (AreAnyLinkPlayersUsingVersions(2u32, 1u32) < 0i32)
        {
            return 0u32;
        }
        return 1u32;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Link_AnyPartnersPlayingRubyOrSapphire() -> u32 {
    unsafe {
        if AreAnyLinkPlayersUsingVersions(2u32, 1u32) >= 0i32 {
            return 1u32;
        }
        return 0u32;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Link_AnyPartnersPlayingFRLG_JP() -> u32 {
    unsafe {
        let mut i: i32 = 0i32;
        i = AreAnyLinkPlayersUsingVersions(4u32, 5u32);
        if (i >= 0i32)
            && ((((((((&raw mut gLinkPlayers).cast::<u8>()).cast::<u8>())
                .wrapping_offset((i) as isize * 28))
            .wrapping_add(26)
            .cast::<u16>())
            .read()) as i32)
                == 1i32)
        {
            return 1u32;
        }
        return 0u32;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn OpenLinkTimed() {
    unsafe {
        ((&raw mut sPlayerDataExchangeStatus)
            .cast::<u8>()
            .cast::<u32>())
        .write(0u32);
        ((&raw mut sTimeOutCounter).cast::<u8>().cast::<u16>()).write(0u16);
        OpenLink();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetLinkPlayerDataExchangeStatusTimed(
    minPlayers: i32,
    maxPlayers: i32,
) -> u8 {
    unsafe {
        let mut minPlayers = minPlayers;
        let mut maxPlayers = maxPlayers;
        let mut i: i32 = 0i32;
        let mut count: i32 = 0i32;
        let mut index: u32 = 0u32;
        let mut numPlayers: u8 = 0u8;
        let mut linkType1: u32 = 0u32;
        let mut linkType2: u32 = 0u32;
        count = 0i32;
        if ((((&raw mut gReceivedRemoteLinkPlayers)
            .cast::<u8>()
            .cast::<u8>())
        .read()) as i32)
            == 1i32
        {
            numPlayers = GetLinkPlayerCount_2();
            if (minPlayers > ((numPlayers) as i32)) || (((numPlayers) as i32) > maxPlayers) {
                ((&raw mut sPlayerDataExchangeStatus)
                    .cast::<u8>()
                    .cast::<u32>())
                .write(6u32);
                return ((((&raw mut sPlayerDataExchangeStatus)
                    .cast::<u8>()
                    .cast::<u32>())
                .read()) as u8);
            } else {
                if ((GetLinkPlayerCount()) as i32) == 0i32 {
                    ((&raw mut gLinkErrorOccurred).cast::<u8>().cast::<u8>()).write(1u8);
                    CloseLink();
                }
                {
                    i = 0i32;
                    index = 0u32;
                    'l1: loop {
                        if !(i < ((GetLinkPlayerCount()) as i32)) {
                            break 'l1;
                        }
                        'l2: {
                            if (((((&raw mut gLinkPlayers).cast::<u8>()).cast::<u8>())
                                .wrapping_offset(((index) as i32) as isize * 28))
                            .wrapping_add(20)
                            .cast::<u32>())
                            .read()
                                == ((((&raw mut gLinkPlayers).cast::<u8>()).cast::<u8>())
                                    .wrapping_add(20)
                                    .cast::<u32>())
                                .read()
                            {
                                count = (count).wrapping_add(1);
                            }
                        }
                        index = (index).wrapping_add(1);
                        i = (i).wrapping_add(1);
                    }
                }
                if count == ((GetLinkPlayerCount()) as i32) {
                    if ((((&raw mut gLinkPlayers).cast::<u8>()).cast::<u8>())
                        .wrapping_add(20)
                        .cast::<u32>())
                    .read()
                        == 4403u32
                    {
                        'l3: {
                            let __sw1 = GetGameProgressForLinkTrade();
                            if __sw1 == 1i32 {
                                ((&raw mut sPlayerDataExchangeStatus)
                                    .cast::<u8>()
                                    .cast::<u32>())
                                .write(4u32);
                                break 'l3;
                            }
                            if __sw1 == 2i32 {
                                ((&raw mut sPlayerDataExchangeStatus)
                                    .cast::<u8>()
                                    .cast::<u32>())
                                .write(5u32);
                                break 'l3;
                            }
                            if __sw1 == 0i32 {
                                ((&raw mut sPlayerDataExchangeStatus)
                                    .cast::<u8>()
                                    .cast::<u32>())
                                .write(1u32);
                                break 'l3;
                            }
                        }
                    } else {
                        ((&raw mut sPlayerDataExchangeStatus)
                            .cast::<u8>()
                            .cast::<u32>())
                        .write(1u32);
                    }
                } else {
                    ((&raw mut sPlayerDataExchangeStatus)
                        .cast::<u8>()
                        .cast::<u32>())
                    .write(3u32);
                    linkType1 = (((((&raw mut gLinkPlayers).cast::<u8>()).cast::<u8>())
                        .wrapping_offset(((GetMultiplayerId()) as i32) as isize * 28))
                    .wrapping_add(20)
                    .cast::<u32>())
                    .read();
                    linkType2 = (((((&raw mut gLinkPlayers).cast::<u8>()).cast::<u8>())
                        .wrapping_offset((((GetMultiplayerId()) as i32) ^ 1i32) as isize * 28))
                    .wrapping_add(20)
                    .cast::<u32>())
                    .read();
                    if ((linkType1 == 8806u32) && (linkType2 == 8823u32))
                        || ((linkType1 == 8823u32) && (linkType2 == 8806u32))
                    {
                        ((&raw mut gSpecialVar_0x8005).cast::<u16>()).write(3u16);
                    }
                }
            }
        } else {
            if (({
                let __p2 = (&raw mut sTimeOutCounter).cast::<u8>().cast::<u16>();
                let __t3 = ((__p2).read()).wrapping_add(1);
                (__p2).write(__t3);
                __t3
            }) as i32)
                > 600i32
            {
                ((&raw mut sPlayerDataExchangeStatus)
                    .cast::<u8>()
                    .cast::<u32>())
                .write(2u32);
            }
        }
        return ((((&raw mut sPlayerDataExchangeStatus)
            .cast::<u8>()
            .cast::<u32>())
        .read()) as u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsLinkPlayerDataExchangeComplete() -> u8 {
    unsafe {
        let mut i: u8 = 0u8;
        let mut count: u8 = 0u8;
        let mut retval: u8 = 0u8;
        count = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < ((GetLinkPlayerCount()) as i32)) {
                    break 'l1;
                }
                'l2: {
                    if (((((&raw mut gLinkPlayers).cast::<u8>()).cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 28))
                    .wrapping_add(20)
                    .cast::<u32>())
                    .read()
                        == ((((&raw mut gLinkPlayers).cast::<u8>()).cast::<u8>())
                            .wrapping_add(20)
                            .cast::<u32>())
                        .read()
                    {
                        count = (count).wrapping_add(1);
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        if ((count) as i32) == ((GetLinkPlayerCount()) as i32) {
            retval = 1u8;
            ((&raw mut sPlayerDataExchangeStatus)
                .cast::<u8>()
                .cast::<u32>())
            .write(1u32);
        } else {
            retval = 0u8;
            ((&raw mut sPlayerDataExchangeStatus)
                .cast::<u8>()
                .cast::<u32>())
            .write(3u32);
        }
        return retval;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetLinkPlayerTrainerId(who: u8) -> u32 {
    unsafe {
        let mut who = who;
        return (((((&raw mut gLinkPlayers).cast::<u8>()).cast::<u8>())
            .wrapping_offset(((who) as i32) as isize * 28))
        .wrapping_add(4)
        .cast::<u32>())
        .read();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ResetLinkPlayers() {
    unsafe {
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i <= 4i32) {
                    break 'l1;
                }
                'l2: {
                    (((&raw mut gLinkPlayers).cast::<u8>()).cast::<u8>())
                        .wrapping_offset((i) as isize * 28)
                        .cast::<crate::c::Rec4<28>>()
                        .write_unaligned({
                            let mut __lit1 = crate::ffi::Align4([0u8; 28]);
                            (&raw mut __lit1)
                                .cast::<u8>()
                                .wrapping_add(0)
                                .cast::<u16>()
                                .write(0u16);
                            (&raw mut __lit1)
                                .cast::<crate::c::Rec4<28>>()
                                .read_unaligned()
                        });
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn ResetBlockSend() {
    unsafe {
        (((&raw mut sBlockSend).cast::<u8>()).wrapping_add(8)).write(0u8);
        (((&raw mut sBlockSend).cast::<u8>()).cast::<u16>()).write(0u16);
        (((&raw mut sBlockSend).cast::<u8>())
            .wrapping_add(2)
            .cast::<u16>())
        .write(0u16);
        (((&raw mut sBlockSend).cast::<u8>())
            .wrapping_add(4)
            .cast::<*mut u8>())
        .write(core::ptr::null_mut());
    }
}
pub(crate) unsafe extern "C" fn InitBlockSend(src: *mut u8, size: u32) -> u32 {
    unsafe {
        let mut src = src;
        let mut size = size;
        if ((((&raw mut sBlockSend).cast::<u8>()).wrapping_add(8)).read()) != 0 {
            return 0u32;
        }
        (((&raw mut sBlockSend).cast::<u8>()).wrapping_add(9)).write(GetMultiplayerId());
        (((&raw mut sBlockSend).cast::<u8>()).wrapping_add(8)).write(1u8);
        (((&raw mut sBlockSend).cast::<u8>())
            .wrapping_add(2)
            .cast::<u16>())
        .write(((size) as u16));
        (((&raw mut sBlockSend).cast::<u8>()).cast::<u16>()).write(0u16);
        if size > 256u32 {
            (((&raw mut sBlockSend).cast::<u8>())
                .wrapping_add(4)
                .cast::<*mut u8>())
            .write(src);
        } else {
            if ((src) as usize)
                != ((((&raw mut gBlockSendBuffer).cast::<u8>()).cast::<u8>()) as usize)
            {
                crate::c::memcpy(
                    ((&raw mut gBlockSendBuffer).cast::<u8>()).cast::<u8>(),
                    src,
                    size,
                );
            }
            (((&raw mut sBlockSend).cast::<u8>())
                .wrapping_add(4)
                .cast::<*mut u8>())
            .write(((&raw mut gBlockSendBuffer).cast::<u8>()).cast::<u8>());
        }
        BuildSendCmd(48059u16);
        ((&raw mut gLinkCallback)
            .cast::<u8>()
            .cast::<Option<unsafe extern "C" fn()>>())
        .write(Some(LinkCB_BlockSendBegin));
        ((&raw mut sBlockSendDelayCounter).cast::<u8>().cast::<u32>()).write(0u32);
        return 1u32;
    }
}
pub(crate) unsafe extern "C" fn LinkCB_BlockSendBegin() {
    unsafe {
        if {
            let __p1 = (&raw mut sBlockSendDelayCounter).cast::<u8>().cast::<u32>();
            let __t2 = ((__p1).read()).wrapping_add(1);
            (__p1).write(__t2);
            __t2
        } > 2u32
        {
            ((&raw mut gLinkCallback)
                .cast::<u8>()
                .cast::<Option<unsafe extern "C" fn()>>())
            .write(Some(LinkCB_BlockSend));
        }
    }
}
pub(crate) unsafe extern "C" fn LinkCB_BlockSend() {
    unsafe {
        let mut i: i32 = 0i32;
        let mut src: *mut u8 = core::ptr::null_mut();
        src = (((&raw mut sBlockSend).cast::<u8>())
            .wrapping_add(4)
            .cast::<*mut u8>())
        .read();
        (((&raw mut gSendCmd).cast::<u8>().cast::<u16>()).cast::<u16>()).write(34952u16);
        {
            i = 0i32;
            'l1: loop {
                if !(i < 7i32) {
                    break 'l1;
                }
                'l2: {
                    ((((&raw mut gSendCmd).cast::<u8>().cast::<u16>()).cast::<u16>())
                        .wrapping_offset(((i).wrapping_add(1i32)) as isize))
                    .write(
                        (((((((src).wrapping_offset(
                            (((((((&raw mut sBlockSend).cast::<u8>()).cast::<u16>()).read())
                                as i32)
                                .wrapping_add((i).wrapping_mul(2i32)))
                            .wrapping_add(1i32)) as isize,
                        ))
                        .read()) as i32)
                            << 8)
                            | ((((src).wrapping_offset(
                                ((((((&raw mut sBlockSend).cast::<u8>()).cast::<u16>()).read())
                                    as i32)
                                    .wrapping_add((i).wrapping_mul(2i32)))
                                    as isize,
                            ))
                            .read()) as i32)) as u16),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        let __p1 = ((&raw mut sBlockSend).cast::<u8>()).cast::<u16>();
        (__p1).write((((((__p1).read()) as i32).wrapping_add(14i32)) as u16));
        if (((((&raw mut sBlockSend).cast::<u8>())
            .wrapping_add(2)
            .cast::<u16>())
        .read()) as i32)
            <= (((((&raw mut sBlockSend).cast::<u8>()).cast::<u16>()).read()) as i32)
        {
            (((&raw mut sBlockSend).cast::<u8>()).wrapping_add(8)).write(0u8);
            ((&raw mut gLinkCallback)
                .cast::<u8>()
                .cast::<Option<unsafe extern "C" fn()>>())
            .write(Some(LinkCB_BlockSendEnd));
        }
    }
}
pub(crate) unsafe extern "C" fn LinkCB_BlockSendEnd() {
    unsafe {
        ((&raw mut gLinkCallback)
            .cast::<u8>()
            .cast::<Option<unsafe extern "C" fn()>>())
        .write(None);
    }
}
pub(crate) unsafe extern "C" fn LinkCB_BerryBlenderSendHeldKeys() {
    unsafe {
        GetMultiplayerId();
        BuildSendCmd(17476u16);
        let __p1 = (&raw mut gBerryBlenderKeySendAttempts)
            .cast::<u8>()
            .cast::<u32>();
        (__p1).write(((__p1).read()).wrapping_add(1));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetBerryBlenderLinkCallback() {
    unsafe {
        ((&raw mut gBerryBlenderKeySendAttempts)
            .cast::<u8>()
            .cast::<u32>())
        .write(0u32);
        if (((&raw mut gWirelessCommType).cast::<u8>().cast::<u8>()).read()) != 0 {
            Rfu_SetBerryBlenderLinkCallback();
        } else {
            ((&raw mut gLinkCallback)
                .cast::<u8>()
                .cast::<Option<unsafe extern "C" fn()>>())
            .write(Some(LinkCB_BerryBlenderSendHeldKeys));
        }
    }
}
pub(crate) unsafe extern "C" fn GetBerryBlenderKeySendAttempts() -> u32 {
    unsafe {
        return ((&raw mut gBerryBlenderKeySendAttempts)
            .cast::<u8>()
            .cast::<u32>())
        .read();
    }
}
pub(crate) unsafe extern "C" fn SendBerryBlenderNoSpaceForPokeblocks() {
    unsafe {
        BuildSendCmd(43690u16);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetMultiplayerId() -> u8 {
    unsafe {
        if ((((&raw mut gWirelessCommType).cast::<u8>().cast::<u8>()).read()) as i32) == 1i32 {
            return Rfu_GetMultiplayerId();
        }
        return ((crate::c::bf_read(
            ((67109160i32) as usize as *mut u8).wrapping_add(0),
            4,
            2,
            false,
        ) as u16) as u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BitmaskAllOtherLinkPlayers() -> u8 {
    unsafe {
        let mut mpId: u8 = 0u8;
        mpId = GetMultiplayerId();
        return ((15i32 ^ crate::c::shl_i32(1i32, ((mpId) as u32))) as u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SendBlock(unused: u8, src: *mut u8, size: u16) -> u8 {
    unsafe {
        let mut unused = unused;
        let mut src = src;
        let mut size = size;
        if ((((&raw mut gWirelessCommType).cast::<u8>().cast::<u8>()).read()) as i32) == 1i32 {
            return ((Rfu_InitBlockSend(src, ((size) as u32))) as u8);
        }
        return ((InitBlockSend(src, ((size) as u32))) as u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SendBlockRequest(blockReqType: u8) -> u8 {
    unsafe {
        let mut blockReqType = blockReqType;
        if ((((&raw mut gWirelessCommType).cast::<u8>().cast::<u8>()).read()) as i32) == 1i32 {
            return Rfu_SendBlockRequest(blockReqType);
        }
        if core::mem::transmute::<_, usize>(
            ((&raw mut gLinkCallback)
                .cast::<u8>()
                .cast::<Option<unsafe extern "C" fn()>>())
            .read(),
        ) == 0usize
        {
            ((&raw mut gBlockRequestType).cast::<u8>().cast::<u8>()).write(blockReqType);
            BuildSendCmd(52428u16);
            return 1u8;
        }
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsLinkTaskFinished() -> u8 {
    unsafe {
        if ((((&raw mut gWirelessCommType).cast::<u8>().cast::<u8>()).read()) as i32) == 1i32 {
            return IsLinkRfuTaskFinished();
        }
        return ((core::mem::transmute::<_, usize>(
            ((&raw mut gLinkCallback)
                .cast::<u8>()
                .cast::<Option<unsafe extern "C" fn()>>())
            .read(),
        ) == 0usize) as u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetBlockReceivedStatus() -> u8 {
    unsafe {
        if ((((&raw mut gWirelessCommType).cast::<u8>().cast::<u8>()).read()) as i32) == 1i32 {
            return Rfu_GetBlockReceivedStatus();
        }
        return (((((((((((&raw mut gBlockReceivedStatus).cast::<u8>()).cast::<u8>())
            .wrapping_offset(3))
        .read()) as i32)
            << 3)
            | (((((((&raw mut gBlockReceivedStatus).cast::<u8>()).cast::<u8>()).wrapping_offset(2))
                .read()) as i32)
                << 2))
            | (((((((&raw mut gBlockReceivedStatus).cast::<u8>()).cast::<u8>()).wrapping_offset(1))
                .read()) as i32)
                << 1))
            | ((((((&raw mut gBlockReceivedStatus).cast::<u8>()).cast::<u8>()).read()) as i32)
                << 0)) as u8);
    }
}
pub(crate) unsafe extern "C" fn SetBlockReceivedFlag(who: u8) {
    unsafe {
        let mut who = who;
        if ((((&raw mut gWirelessCommType).cast::<u8>().cast::<u8>()).read()) as i32) == 1i32 {
            Rfu_SetBlockReceivedFlag(who);
        } else {
            ((((&raw mut gBlockReceivedStatus).cast::<u8>()).cast::<u8>())
                .wrapping_offset(((who) as i32) as isize))
            .write(1u8);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ResetBlockReceivedFlags() {
    unsafe {
        let mut i: i32 = 0i32;
        if ((((&raw mut gWirelessCommType).cast::<u8>().cast::<u8>()).read()) as i32) == 1i32 {
            {
                i = 0i32;
                'l1: loop {
                    if !(i < 5i32) {
                        break 'l1;
                    }
                    'l2: {
                        Rfu_ResetBlockReceivedFlag(((i) as u8));
                    }
                    i = (i).wrapping_add(1);
                }
            }
        } else {
            {
                i = 0i32;
                'l3: loop {
                    if !(i < 4i32) {
                        break 'l3;
                    }
                    'l4: {
                        ((((&raw mut gBlockReceivedStatus).cast::<u8>()).cast::<u8>())
                            .wrapping_offset((i) as isize))
                        .write(0u8);
                    }
                    i = (i).wrapping_add(1);
                }
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ResetBlockReceivedFlag(who: u8) {
    unsafe {
        let mut who = who;
        if ((((&raw mut gWirelessCommType).cast::<u8>().cast::<u8>()).read()) as i32) == 1i32 {
            Rfu_ResetBlockReceivedFlag(who);
        } else {
            if (((((&raw mut gBlockReceivedStatus).cast::<u8>()).cast::<u8>())
                .wrapping_offset(((who) as i32) as isize))
            .read())
                != 0
            {
                ((((&raw mut gBlockReceivedStatus).cast::<u8>()).cast::<u8>())
                    .wrapping_offset(((who) as i32) as isize))
                .write(0u8);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CheckShouldAdvanceLinkState() {
    unsafe {
        if ((((&raw mut gLinkStatus).cast::<u8>().cast::<u32>()).read() & 32u32) != 0)
            && (((((&raw mut gLinkStatus).cast::<u8>().cast::<u32>()).read() & 28u32) >> 2) > 1u32)
        {
            ((&raw mut gShouldAdvanceLinkState).cast::<u8>().cast::<u8>()).write(1u8);
        }
    }
}
pub(crate) unsafe extern "C" fn LinkTestCalcBlockChecksum(src: *mut u16, size: u16) -> u16 {
    unsafe {
        let mut src = src;
        let mut size = size;
        let mut chksum: u16 = 0u16;
        let mut i: u16 = 0u16;
        chksum = 0u16;
        {
            i = 0u16;
            'l1: loop {
                if !(((i) as i32) < crate::c::div_i32(((size) as i32), 2i32)) {
                    break 'l1;
                }
                'l2: {
                    chksum = ((((chksum) as i32).wrapping_add(
                        ((((src).wrapping_offset(((i) as i32) as isize)).read()) as i32),
                    )) as u16);
                }
                i = (i).wrapping_add(1);
            }
        }
        return chksum;
    }
}
pub(crate) unsafe extern "C" fn LinkTest_PrintNumChar(val: u8, x: u8, y: u8) {
    unsafe {
        let mut val = val;
        let mut x = x;
        let mut y = y;
        let mut vAddr: *mut u16 = core::ptr::null_mut();
        vAddr =
            (((100663296u32)
                .wrapping_add((2048u32).wrapping_mul(
                    (((&raw mut gLinkTestBGInfo).cast::<u8>()).cast::<u32>()).read(),
                ))) as usize as *mut u16);
        ((vAddr).wrapping_offset(
            ((((y) as i32).wrapping_mul(32i32)).wrapping_add(((x) as i32))) as isize,
        ))
        .write(
            ((((((&raw mut gLinkTestBGInfo).cast::<u8>())
                .wrapping_add(4)
                .cast::<u32>())
            .read()
                << 12)
                | ((((val) as i32).wrapping_add(1i32)) as u32).wrapping_add(
                    (((&raw mut gLinkTestBGInfo).cast::<u8>())
                        .wrapping_add(8)
                        .cast::<u32>())
                    .read(),
                )) as u16),
        );
    }
}
pub(crate) unsafe extern "C" fn LinkTest_PrintChar(val: u8, x: u8, y: u8) {
    unsafe {
        let mut val = val;
        let mut x = x;
        let mut y = y;
        let mut vAddr: *mut u16 = core::ptr::null_mut();
        vAddr =
            (((100663296u32)
                .wrapping_add((2048u32).wrapping_mul(
                    (((&raw mut gLinkTestBGInfo).cast::<u8>()).cast::<u32>()).read(),
                ))) as usize as *mut u16);
        ((vAddr).wrapping_offset(
            ((((y) as i32).wrapping_mul(32i32)).wrapping_add(((x) as i32))) as isize,
        ))
        .write(
            ((((((&raw mut gLinkTestBGInfo).cast::<u8>())
                .wrapping_add(4)
                .cast::<u32>())
            .read()
                << 12)
                | ((val) as u32).wrapping_add(
                    (((&raw mut gLinkTestBGInfo).cast::<u8>())
                        .wrapping_add(8)
                        .cast::<u32>())
                    .read(),
                )) as u16),
        );
    }
}
pub(crate) unsafe extern "C" fn LinkTest_PrintHex(num: u32, x: u8, y: u8, length: u8) {
    unsafe {
        let mut num = num;
        let mut x = x;
        let mut y = y;
        let mut length = length;
        let mut buff = crate::ffi::Align4([0u8; 16]);
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < ((length) as i32)) {
                    break 'l1;
                }
                'l2: {
                    (((&raw mut buff).cast::<u8>()).wrapping_offset((i) as isize))
                        .write(((num & 15u32) as u8));
                    num = (num >> 4);
                }
                i = (i).wrapping_add(1);
            }
        }
        {
            i = ((length) as i32).wrapping_sub(1i32);
            'l3: loop {
                if !(i >= 0i32) {
                    break 'l3;
                }
                'l4: {
                    LinkTest_PrintNumChar(
                        (((&raw mut buff).cast::<u8>()).wrapping_offset((i) as isize)).read(),
                        x,
                        y,
                    );
                    x = (x).wrapping_add(1);
                }
                i = (i).wrapping_sub(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn LinkTest_PrintInt(num: i32, x: u8, y: u8, length: u8) {
    unsafe {
        let mut num = num;
        let mut x = x;
        let mut y = y;
        let mut length = length;
        let mut buff = crate::ffi::Align4([0u8; 16]);
        let mut negX: i32 = 0i32;
        let mut i: i32 = 0i32;
        negX = (-1i32);
        if num < 0i32 {
            negX = ((x) as i32);
            num = (num).wrapping_neg();
        }
        {
            i = 0i32;
            'l1: loop {
                if !(i < ((length) as i32)) {
                    break 'l1;
                }
                'l2: {
                    (((&raw mut buff).cast::<u8>()).wrapping_offset((i) as isize))
                        .write(((crate::c::rem_i32(num, 10i32)) as u8));
                    num = crate::c::div_i32(num, 10i32);
                }
                i = (i).wrapping_add(1);
            }
        }
        {
            i = ((length) as i32).wrapping_sub(1i32);
            'l3: loop {
                if !(i >= 0i32) {
                    break 'l3;
                }
                'l4: {
                    LinkTest_PrintNumChar(
                        (((&raw mut buff).cast::<u8>()).wrapping_offset((i) as isize)).read(),
                        x,
                        y,
                    );
                    x = (x).wrapping_add(1);
                }
                i = (i).wrapping_sub(1);
            }
        }
        if negX != (-1i32) {
            LinkTest_PrintNumChar(([10, 0u8].as_ptr().cast_mut()).read(), ((negX) as u8), y);
        }
    }
}
pub(crate) unsafe extern "C" fn LinkTest_PrintString(str: *mut u8, x: u8, y: u8) {
    unsafe {
        let mut str = str;
        let mut x = x;
        let mut y = y;
        let mut xOffset: i32 = 0i32;
        let mut i: i32 = 0i32;
        let mut yOffset: i32 = 0i32;
        yOffset = 0i32;
        xOffset = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(((((str).wrapping_offset((i) as isize)).read()) as i32) != 0i32) {
                    break 'l1;
                }
                'l2: {
                    if ((((str).wrapping_offset((i) as isize)).read()) as i32)
                        == ((([10, 0u8].as_ptr().cast_mut()).read()) as i32)
                    {
                        yOffset = (yOffset).wrapping_add(1);
                        xOffset = 0i32;
                    } else {
                        LinkTest_PrintChar(
                            ((str).wrapping_offset((i) as isize)).read(),
                            ((((x) as i32).wrapping_add(xOffset)) as u8),
                            ((((y) as i32).wrapping_add(yOffset)) as u8),
                        );
                        xOffset = (xOffset).wrapping_add(1);
                    }
                }
                str = (str).wrapping_offset(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn LinkCB_RequestPlayerDataExchange() {
    unsafe {
        if (((&raw mut gLinkStatus).cast::<u8>().cast::<u32>()).read() & 32u32) != 0 {
            BuildSendCmd(8738u16);
        }
        ((&raw mut gLinkCallback)
            .cast::<u8>()
            .cast::<Option<unsafe extern "C" fn()>>())
        .write(None);
    }
}
pub(crate) unsafe extern "C" fn Task_PrintTestData(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut testTitle = crate::ffi::Align4([0u8; 32]);
        let mut i: i32 = 0i32;
        crate::c::strcpy(
            (&raw mut testTitle).cast::<u8>(),
            ((&raw const sASCIITestPrint).cast::<u8>().cast_mut()).cast::<u8>(),
        );
        LinkTest_PrintString((&raw mut testTitle).cast::<u8>(), 5u8, 2u8);
        LinkTest_PrintHex(
            ((((&raw mut gShouldAdvanceLinkState).cast::<u8>().cast::<u8>()).read()) as u32),
            2u8,
            1u8,
            2u8,
        );
        LinkTest_PrintHex(
            ((&raw mut gLinkStatus).cast::<u8>().cast::<u32>()).read(),
            15u8,
            1u8,
            8u8,
        );
        LinkTest_PrintHex(
            (((((&raw mut gLink).cast::<u8>()).wrapping_add(1)).read()) as u32),
            2u8,
            10u8,
            2u8,
        );
        LinkTest_PrintHex(
            ((((&raw mut gLinkStatus).cast::<u8>().cast::<u32>()).read() & 28u32) >> 2),
            15u8,
            10u8,
            2u8,
        );
        LinkTest_PrintHex(((GetMultiplayerId()) as u32), 15u8, 12u8, 2u8);
        LinkTest_PrintHex(
            ((((&raw mut gLastSendQueueCount).cast::<u8>().cast::<u8>()).read()) as u32),
            25u8,
            1u8,
            2u8,
        );
        LinkTest_PrintHex(
            ((((&raw mut gLastRecvQueueCount).cast::<u8>().cast::<u8>()).read()) as u32),
            25u8,
            2u8,
            2u8,
        );
        LinkTest_PrintHex(((GetBlockReceivedStatus()) as u32), 15u8, 5u8, 2u8);
        LinkTest_PrintHex(
            ((&raw mut gLinkDebugSeed).cast::<u8>().cast::<u32>()).read(),
            2u8,
            12u8,
            8u8,
        );
        LinkTest_PrintHex(
            ((&raw mut gLinkDebugFlags).cast::<u8>().cast::<u32>()).read(),
            2u8,
            13u8,
            8u8,
        );
        LinkTest_PrintHex(((GetSioMultiSI()) as u32), 25u8, 5u8, 1u8);
        LinkTest_PrintHex(((IsSioMultiMaster()) as u32), 25u8, 6u8, 1u8);
        LinkTest_PrintHex(((IsLinkConnectionEstablished()) as u32), 25u8, 7u8, 1u8);
        LinkTest_PrintHex(((HasLinkErrorOccurred()) as u32), 25u8, 8u8, 1u8);
        {
            i = 0i32;
            'l1: loop {
                if !(i < 4i32) {
                    break 'l1;
                }
                'l2: {
                    LinkTest_PrintHex(
                        ((((((&raw mut gLinkTestBlockChecksums)
                            .cast::<u8>()
                            .cast::<u16>())
                        .cast::<u16>())
                        .wrapping_offset((i) as isize))
                        .read()) as u32),
                        10u8,
                        (((4i32).wrapping_add(i)) as u8),
                        4u8,
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetLinkDebugValues(seed: u32, flags: u32) {
    unsafe {
        let mut seed = seed;
        let mut flags = flags;
        ((&raw mut gLinkDebugSeed).cast::<u8>().cast::<u32>()).write(seed);
        ((&raw mut gLinkDebugFlags).cast::<u8>().cast::<u32>()).write(flags);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetSavedLinkPlayerCountAsBitFlags() -> u8 {
    unsafe {
        let mut i: i32 = 0i32;
        let mut flags: u8 = 0u8;
        flags = 0u8;
        {
            i = 0i32;
            'l1: loop {
                if !(i
                    < ((((&raw mut gSavedLinkPlayerCount).cast::<u8>().cast::<u8>()).read())
                        as i32))
                {
                    break 'l1;
                }
                'l2: {
                    flags = ((((flags) as i32) | crate::c::shl_i32(1i32, ((i) as u32))) as u8);
                }
                i = (i).wrapping_add(1);
            }
        }
        return flags;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetLinkPlayerCountAsBitFlags() -> u8 {
    unsafe {
        let mut i: i32 = 0i32;
        let mut flags: u8 = 0u8;
        flags = 0u8;
        {
            i = 0i32;
            'l1: loop {
                if !(i < ((GetLinkPlayerCount()) as i32)) {
                    break 'l1;
                }
                'l2: {
                    flags = ((((flags) as i32) | crate::c::shl_i32(1i32, ((i) as u32))) as u8);
                }
                i = (i).wrapping_add(1);
            }
        }
        return flags;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SaveLinkPlayers(playerCount: u8) {
    unsafe {
        let mut playerCount = playerCount;
        let mut i: i32 = 0i32;
        ((&raw mut gSavedLinkPlayerCount).cast::<u8>().cast::<u8>()).write(playerCount);
        ((&raw mut gSavedMultiplayerId).cast::<u8>().cast::<u8>()).write(GetMultiplayerId());
        {
            i = 0i32;
            'l1: loop {
                if !(i < 5i32) {
                    break 'l1;
                }
                'l2: {
                    (((&raw mut sSavedLinkPlayers).cast::<u8>()).cast::<u8>())
                        .wrapping_offset((i) as isize * 28)
                        .cast::<crate::c::Rec4<28>>()
                        .write_unaligned(
                            (((&raw mut gLinkPlayers).cast::<u8>()).cast::<u8>())
                                .wrapping_offset((i) as isize * 28)
                                .cast::<crate::c::Rec4<28>>()
                                .read_unaligned(),
                        );
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetSavedPlayerCount() -> u8 {
    unsafe {
        return ((&raw mut gSavedLinkPlayerCount).cast::<u8>().cast::<u8>()).read();
    }
}
pub(crate) unsafe extern "C" fn GetSavedMultiplayerId() -> u8 {
    unsafe {
        return ((&raw mut gSavedMultiplayerId).cast::<u8>().cast::<u8>()).read();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DoesLinkPlayerCountMatchSaved() -> u8 {
    unsafe {
        let mut i: i32 = 0i32;
        let mut count: u32 = 0u32;
        {
            i = 0i32;
            'l1: loop {
                if !(i
                    < ((((&raw mut gSavedLinkPlayerCount).cast::<u8>().cast::<u8>()).read())
                        as i32))
                {
                    break 'l1;
                }
                'l2: {
                    if (((((&raw mut gLinkPlayers).cast::<u8>()).cast::<u8>())
                        .wrapping_offset((i) as isize * 28))
                    .wrapping_add(4)
                    .cast::<u32>())
                    .read()
                        == (((((&raw mut sSavedLinkPlayers).cast::<u8>()).cast::<u8>())
                            .wrapping_offset((i) as isize * 28))
                        .wrapping_add(4)
                        .cast::<u32>())
                        .read()
                    {
                        if ((((&raw mut gLinkType).cast::<u8>().cast::<u16>()).read()) as i32)
                            == 8840i32
                        {
                            if ((((&raw mut gLinkType).cast::<u8>().cast::<u16>()).read()) as u32)
                                == (((((&raw mut gLinkPlayers).cast::<u8>()).cast::<u8>())
                                    .wrapping_offset((i) as isize * 28))
                                .wrapping_add(20)
                                .cast::<u32>())
                                .read()
                            {
                                count = (count).wrapping_add(1);
                            }
                        } else {
                            count = (count).wrapping_add(1);
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        if count == ((((&raw mut gSavedLinkPlayerCount).cast::<u8>().cast::<u8>()).read()) as u32) {
            if ((GetLinkPlayerCount_2()) as i32)
                == ((((&raw mut gSavedLinkPlayerCount).cast::<u8>().cast::<u8>()).read()) as i32)
            {
                return 1u8;
            }
        }
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ClearSavedLinkPlayers() {
    unsafe {
        crate::c::memset(
            ((&raw mut sSavedLinkPlayers).cast::<u8>()).cast::<u8>(),
            0i32,
            140u32,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CheckLinkPlayersMatchSaved() {
    unsafe {
        let mut i: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32)
                    < ((((&raw mut gSavedLinkPlayerCount).cast::<u8>().cast::<u8>()).read())
                        as i32))
                {
                    break 'l1;
                }
                'l2: {
                    if ((((((&raw mut sSavedLinkPlayers).cast::<u8>()).cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 28))
                    .wrapping_add(4)
                    .cast::<u32>())
                    .read()
                        != (((((&raw mut gLinkPlayers).cast::<u8>()).cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 28))
                        .wrapping_add(4)
                        .cast::<u32>())
                        .read())
                        || (StringCompare(
                            (((((&raw mut sSavedLinkPlayers).cast::<u8>()).cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize * 28))
                            .wrapping_add(8))
                            .cast::<u8>(),
                            (((((&raw mut gLinkPlayers).cast::<u8>()).cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize * 28))
                            .wrapping_add(8))
                            .cast::<u8>(),
                        ) != 0i32)
                    {
                        ((&raw mut gLinkErrorOccurred).cast::<u8>().cast::<u8>()).write(1u8);
                        CloseLink();
                        SetMainCallback2(Some(CB2_LinkError));
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ResetLinkPlayerCount() {
    unsafe {
        ((&raw mut gSavedLinkPlayerCount).cast::<u8>().cast::<u8>()).write(0u8);
        ((&raw mut gSavedMultiplayerId).cast::<u8>().cast::<u8>()).write(0u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetLinkPlayerCount_2() -> u8 {
    unsafe {
        return (((((&raw mut gLinkStatus).cast::<u8>().cast::<u32>()).read() & 28u32) >> 2) as u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsLinkMaster() -> u8 {
    unsafe {
        if (((&raw mut gWirelessCommType).cast::<u8>().cast::<u8>()).read()) != 0 {
            return Rfu_IsMaster();
        }
        return (((((&raw mut gLinkStatus).cast::<u8>().cast::<u32>()).read() >> 5) & 1u32) as u8);
    }
}
pub(crate) unsafe extern "C" fn GetDummy2() -> u8 {
    unsafe {
        return ((&raw mut sDummy2).cast::<u8>().cast::<u8>()).read();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetCloseLinkCallbackAndType(r#type: u16) {
    unsafe {
        let mut r#type = r#type;
        if ((((&raw mut gWirelessCommType).cast::<u8>().cast::<u8>()).read()) as i32) == 1i32 {
            Rfu_SetCloseLinkCallback();
        } else {
            if core::mem::transmute::<_, usize>(
                ((&raw mut gLinkCallback)
                    .cast::<u8>()
                    .cast::<Option<unsafe extern "C" fn()>>())
                .read(),
            ) == 0usize
            {
                ((&raw mut gLinkCallback)
                    .cast::<u8>()
                    .cast::<Option<unsafe extern "C" fn()>>())
                .write(Some(LinkCB_ReadyCloseLink));
                ((&raw mut gLinkDummy1).cast::<u8>().cast::<u8>()).write(0u8);
                ((&raw mut gReadyCloseLinkType).cast::<u8>().cast::<u16>()).write(r#type);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetCloseLinkCallback() {
    unsafe {
        if ((((&raw mut gWirelessCommType).cast::<u8>().cast::<u8>()).read()) as i32) == 1i32 {
            Rfu_SetCloseLinkCallback();
        } else {
            if core::mem::transmute::<_, usize>(
                ((&raw mut gLinkCallback)
                    .cast::<u8>()
                    .cast::<Option<unsafe extern "C" fn()>>())
                .read(),
            ) != 0usize
            {
                let __p1 = (&raw mut sReadyCloseLinkAttempts)
                    .cast::<u8>()
                    .cast::<u16>();
                (__p1).write(((__p1).read()).wrapping_add(1));
            } else {
                ((&raw mut gLinkCallback)
                    .cast::<u8>()
                    .cast::<Option<unsafe extern "C" fn()>>())
                .write(Some(LinkCB_ReadyCloseLink));
                ((&raw mut gLinkDummy1).cast::<u8>().cast::<u8>()).write(0u8);
                ((&raw mut gReadyCloseLinkType).cast::<u8>().cast::<u16>()).write(0u16);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn LinkCB_ReadyCloseLink() {
    unsafe {
        if ((((&raw mut gLastRecvQueueCount).cast::<u8>().cast::<u8>()).read()) as i32) == 0i32 {
            BuildSendCmd(24575u16);
            ((&raw mut gLinkCallback)
                .cast::<u8>()
                .cast::<Option<unsafe extern "C" fn()>>())
            .write(Some(LinkCB_WaitCloseLink));
        }
    }
}
pub(crate) unsafe extern "C" fn LinkCB_WaitCloseLink() {
    unsafe {
        let mut i: i32 = 0i32;
        let mut count: u32 = 0u32;
        let mut linkPlayerCount: u8 = GetLinkPlayerCount();
        count = 0u32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < ((linkPlayerCount) as i32)) {
                    break 'l1;
                }
                'l2: {
                    if (((((&raw mut gReadyToCloseLink).cast::<u8>()).cast::<u8>())
                        .wrapping_offset((i) as isize))
                    .read())
                        != 0
                    {
                        count = (count).wrapping_add(1);
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        if count == ((linkPlayerCount) as u32) {
            let __p1 = (&raw mut gBattleTypeFlags).cast::<u32>();
            (__p1).write(((__p1).read() & 4294967263u32));
            ((&raw mut gLinkVSyncDisabled).cast::<u8>()).write(1u8);
            CloseLink();
            ((&raw mut gLinkCallback)
                .cast::<u8>()
                .cast::<Option<unsafe extern "C" fn()>>())
            .write(None);
            ((&raw mut gLinkDummy1).cast::<u8>().cast::<u8>()).write(1u8);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetCloseLinkCallbackHandleJP() {
    unsafe {
        if ((((&raw mut gWirelessCommType).cast::<u8>().cast::<u8>()).read()) as i32) == 1i32 {
            Rfu_SetCloseLinkCallback();
        } else {
            if core::mem::transmute::<_, usize>(
                ((&raw mut gLinkCallback)
                    .cast::<u8>()
                    .cast::<Option<unsafe extern "C" fn()>>())
                .read(),
            ) != 0usize
            {
                let __p1 = (&raw mut sReadyCloseLinkAttempts)
                    .cast::<u8>()
                    .cast::<u16>();
                (__p1).write(((__p1).read()).wrapping_add(1));
            } else {
                ((&raw mut gLinkCallback)
                    .cast::<u8>()
                    .cast::<Option<unsafe extern "C" fn()>>())
                .write(Some(LinkCB_ReadyCloseLinkWithJP));
                ((&raw mut gLinkDummy1).cast::<u8>().cast::<u8>()).write(0u8);
                ((&raw mut gReadyCloseLinkType).cast::<u8>().cast::<u16>()).write(0u16);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn LinkCB_ReadyCloseLinkWithJP() {
    unsafe {
        if ((((&raw mut gLastRecvQueueCount).cast::<u8>().cast::<u8>()).read()) as i32) == 0i32 {
            BuildSendCmd(24575u16);
            ((&raw mut gLinkCallback)
                .cast::<u8>()
                .cast::<Option<unsafe extern "C" fn()>>())
            .write(Some(LinkCB_WaitCloseLinkWithJP));
        }
    }
}
pub(crate) unsafe extern "C" fn LinkCB_WaitCloseLinkWithJP() {
    unsafe {
        let mut i: i32 = 0i32;
        let mut count: u32 = 0u32;
        let mut linkPlayerCount: u8 = 0u8;
        linkPlayerCount = GetLinkPlayerCount();
        count = 0u32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < ((linkPlayerCount) as i32)) {
                    break 'l1;
                }
                'l2: {
                    if (((((((&raw mut gLinkPlayers).cast::<u8>()).cast::<u8>())
                        .wrapping_offset((i) as isize * 28))
                    .wrapping_add(26)
                    .cast::<u16>())
                    .read()) as i32)
                        == 1i32
                    {
                        count = (count).wrapping_add(1);
                    } else {
                        if (((((&raw mut gReadyToCloseLink).cast::<u8>()).cast::<u8>())
                            .wrapping_offset((i) as isize))
                        .read())
                            != 0
                        {
                            count = (count).wrapping_add(1);
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        if count == ((linkPlayerCount) as u32) {
            let __p1 = (&raw mut gBattleTypeFlags).cast::<u32>();
            (__p1).write(((__p1).read() & 4294967263u32));
            ((&raw mut gLinkVSyncDisabled).cast::<u8>()).write(1u8);
            CloseLink();
            ((&raw mut gLinkCallback)
                .cast::<u8>()
                .cast::<Option<unsafe extern "C" fn()>>())
            .write(None);
            ((&raw mut gLinkDummy1).cast::<u8>().cast::<u8>()).write(1u8);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetLinkStandbyCallback() {
    unsafe {
        if ((((&raw mut gWirelessCommType).cast::<u8>().cast::<u8>()).read()) as i32) == 1i32 {
            Rfu_SetLinkStandbyCallback();
        } else {
            if core::mem::transmute::<_, usize>(
                ((&raw mut gLinkCallback)
                    .cast::<u8>()
                    .cast::<Option<unsafe extern "C" fn()>>())
                .read(),
            ) == 0usize
            {
                ((&raw mut gLinkCallback)
                    .cast::<u8>()
                    .cast::<Option<unsafe extern "C" fn()>>())
                .write(Some(LinkCB_Standby));
            }
            ((&raw mut gLinkDummy1).cast::<u8>().cast::<u8>()).write(0u8);
        }
    }
}
pub(crate) unsafe extern "C" fn LinkCB_Standby() {
    unsafe {
        if ((((&raw mut gLastRecvQueueCount).cast::<u8>().cast::<u8>()).read()) as i32) == 0i32 {
            BuildSendCmd(12286u16);
            ((&raw mut gLinkCallback)
                .cast::<u8>()
                .cast::<Option<unsafe extern "C" fn()>>())
            .write(Some(LinkCB_StandbyForAll));
        }
    }
}
pub(crate) unsafe extern "C" fn LinkCB_StandbyForAll() {
    unsafe {
        let mut i: u8 = 0u8;
        let mut linkPlayerCount: u8 = GetLinkPlayerCount();
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < ((linkPlayerCount) as i32)) {
                    break 'l1;
                }
                'l2: {
                    if !((((((&raw mut gReadyToExitStandby).cast::<u8>()).cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize))
                    .read())
                        != 0)
                    {
                        break 'l1;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        if ((i) as i32) == ((linkPlayerCount) as i32) {
            {
                i = 0u8;
                'l3: loop {
                    if !(((i) as i32) < 4i32) {
                        break 'l3;
                    }
                    'l4: {
                        ((((&raw mut gReadyToExitStandby).cast::<u8>()).cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize))
                        .write(0u8);
                    }
                    i = (i).wrapping_add(1);
                }
            }
            ((&raw mut gLinkCallback)
                .cast::<u8>()
                .cast::<Option<unsafe extern "C" fn()>>())
            .write(None);
        }
    }
}
pub(crate) unsafe extern "C" fn TrySetLinkErrorBuffer() {
    unsafe {
        if ((((&raw mut sLinkOpen).cast::<u8>().cast::<u8>()).read()) != 0)
            && (((((&raw mut gLinkStatus).cast::<u8>().cast::<u32>()).read() & 520192u32) >> 12)
                != 0)
        {
            if !((((&raw mut gSuppressLinkErrorMessage)
                .cast::<u8>()
                .cast::<u8>())
            .read())
                != 0)
            {
                (((&raw mut sLinkErrorBuffer).cast::<u8>()).cast::<u32>())
                    .write(((&raw mut gLinkStatus).cast::<u8>().cast::<u32>()).read());
                (((&raw mut sLinkErrorBuffer).cast::<u8>()).wrapping_add(4))
                    .write(((&raw mut gLastRecvQueueCount).cast::<u8>().cast::<u8>()).read());
                (((&raw mut sLinkErrorBuffer).cast::<u8>()).wrapping_add(5))
                    .write(((&raw mut gLastSendQueueCount).cast::<u8>().cast::<u8>()).read());
                SetMainCallback2(Some(CB2_LinkError));
            }
            ((&raw mut gLinkErrorOccurred).cast::<u8>().cast::<u8>()).write(1u8);
            CloseLink();
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetLinkErrorBuffer(
    status: u32,
    lastSendQueueCount: u8,
    lastRecvQueueCount: u8,
    disconnected: u8,
) {
    unsafe {
        let mut status = status;
        let mut lastSendQueueCount = lastSendQueueCount;
        let mut lastRecvQueueCount = lastRecvQueueCount;
        let mut disconnected = disconnected;
        (((&raw mut sLinkErrorBuffer).cast::<u8>()).cast::<u32>()).write(status);
        (((&raw mut sLinkErrorBuffer).cast::<u8>()).wrapping_add(5)).write(lastSendQueueCount);
        (((&raw mut sLinkErrorBuffer).cast::<u8>()).wrapping_add(4)).write(lastRecvQueueCount);
        (((&raw mut sLinkErrorBuffer).cast::<u8>()).wrapping_add(6)).write(disconnected);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CB2_LinkError() {
    unsafe {
        let mut tilemapBuffer: *mut u8 = core::ptr::null_mut();
        SetGpuReg(0u8, 0u16);
        m4aMPlayStop((&raw mut gMPlayInfo_SE1).cast::<u8>());
        m4aMPlayStop((&raw mut gMPlayInfo_SE2).cast::<u8>());
        m4aMPlayStop((&raw mut gMPlayInfo_SE3).cast::<u8>());
        InitHeap((&raw mut gHeap).cast::<u8>(), 114688u32);
        ResetSpriteData();
        FreeAllSpritePalettes();
        ResetPaletteFadeControl();
        SetBackdropFromColor(0u16);
        ResetTasks();
        ScanlineEffect_Stop();
        if (((&raw mut gWirelessCommType).cast::<u8>().cast::<u8>()).read()) != 0 {
            if !(((((&raw mut sLinkErrorBuffer).cast::<u8>()).wrapping_add(6)).read()) != 0) {
                ((&raw mut gWirelessCommType).cast::<u8>().cast::<u8>()).write(3u8);
            }
            ResetLinkRfuGFLayer();
        }
        SetVBlankCallback(Some(VBlankCB_LinkError));
        ResetBgsAndClearDma3BusyFlags(0u32);
        InitBgsFromTemplates(
            0u8,
            ((&raw const sLinkErrorBgTemplates).cast::<u8>().cast_mut()).cast::<u8>(),
            ((crate::c::div_u32(8u32, 4u32)) as u8),
        );
        ((&raw mut sLinkErrorBgTilemapBuffer)
            .cast::<u8>()
            .cast::<*mut u8>())
        .write({
            let __v1 = Alloc(2048u32);
            tilemapBuffer = __v1;
            __v1
        });
        SetBgTilemapBuffer(1u8, tilemapBuffer);
        if (InitWindows(
            ((&raw const sLinkErrorWindowTemplates)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>(),
        )) != 0
        {
            DeactivateAllTextPrinters();
            ResetTempTileDataBuffers();
            SetGpuReg(80u8, 0u16);
            SetGpuReg(82u8, 0u16);
            SetGpuReg(16u8, 0u16);
            SetGpuReg(18u8, 0u16);
            SetGpuReg(20u8, 0u16);
            SetGpuReg(22u8, 0u16);
            ClearGpuRegBits(0u8, 57344u16);
            LoadPalette(
                (((&raw mut gStandardMenuPalette).cast::<u16>()).cast::<u16>()).cast::<u8>(),
                240u16,
                32u16,
            );
            ((&raw mut gSoftResetDisabled).cast::<u8>()).write(0u8);
            CreateTask(Some(Task_DestroySelf), 0u8);
            StopMapMusic();
            (((&raw mut gMain).cast::<u8>()).cast::<Option<unsafe extern "C" fn()>>()).write(None);
            RunTasks();
            AnimateSprites();
            BuildOamBuffer();
            UpdatePaletteFade();
            SetMainCallback2(Some(CB2_PrintErrorMessage));
        }
    }
}
pub(crate) unsafe extern "C" fn ErrorMsg_MoveCloserToPartner() {
    unsafe {
        LoadBgTiles(
            0u8,
            (((&raw const sCommErrorBg_Gfx)
                .cast::<u8>()
                .cast_mut()
                .cast::<u16>())
            .cast::<u16>())
            .cast::<u8>(),
            32u16,
            0u16,
        );
        DecompressAndLoadBgGfxUsingHeap(
            1u8,
            (((&raw const sWirelessLinkDisplayGfx)
                .cast::<u8>()
                .cast_mut()
                .cast::<u32>())
            .cast::<u32>())
            .cast::<u8>(),
            0u32,
            0u16,
            0u8,
        );
        CopyToBgTilemapBuffer(
            1u8,
            (((&raw const sWirelessLinkDisplayTilemap)
                .cast::<u8>()
                .cast_mut()
                .cast::<u32>())
            .cast::<u32>())
            .cast::<u8>(),
            0u16,
            0u16,
        );
        CopyBgTilemapBufferToVram(1u8);
        LoadPalette(
            (((&raw const sWirelessLinkDisplayPal)
                .cast::<u8>()
                .cast_mut()
                .cast::<u16>())
            .cast::<u16>())
            .cast::<u8>(),
            0u16,
            32u16,
        );
        FillWindowPixelBuffer(0u8, 0u8);
        FillWindowPixelBuffer(2u8, 0u8);
        AddTextPrinterParameterized3(
            0u8,
            3u8,
            2u8,
            6u8,
            ((&raw const sTextColors).cast::<u8>().cast_mut()).cast::<u8>(),
            0i8,
            (&raw mut gText_CommErrorEllipsis).cast::<u8>(),
        );
        AddTextPrinterParameterized3(
            2u8,
            3u8,
            2u8,
            1u8,
            ((&raw const sTextColors).cast::<u8>().cast_mut()).cast::<u8>(),
            0i8,
            (&raw mut gText_MoveCloserToLinkPartner).cast::<u8>(),
        );
        PutWindowTilemap(0u8);
        PutWindowTilemap(2u8);
        CopyWindowToVram(0u8, 0u8);
        CopyWindowToVram(2u8, 3u8);
    }
}
pub(crate) unsafe extern "C" fn ErrorMsg_CheckConnections() {
    unsafe {
        LoadBgTiles(
            0u8,
            (((&raw const sCommErrorBg_Gfx)
                .cast::<u8>()
                .cast_mut()
                .cast::<u16>())
            .cast::<u16>())
            .cast::<u8>(),
            32u16,
            0u16,
        );
        FillWindowPixelBuffer(1u8, 0u8);
        FillWindowPixelBuffer(2u8, 0u8);
        AddTextPrinterParameterized3(
            1u8,
            3u8,
            2u8,
            0u8,
            ((&raw const sTextColors).cast::<u8>().cast_mut()).cast::<u8>(),
            0i8,
            (&raw mut gText_CommErrorCheckConnections).cast::<u8>(),
        );
        PutWindowTilemap(1u8);
        PutWindowTilemap(2u8);
        CopyWindowToVram(1u8, 0u8);
        CopyWindowToVram(2u8, 3u8);
    }
}
pub(crate) unsafe extern "C" fn CB2_PrintErrorMessage() {
    unsafe {
        'l1: {
            let __sw1 = (((((&raw mut gMain).cast::<u8>()).wrapping_add(1080)).read()) as i32);
            if __sw1 == 0i32 {
                if ((((&raw mut sLinkErrorBuffer).cast::<u8>()).wrapping_add(6)).read()) != 0 {
                    ErrorMsg_MoveCloserToPartner();
                } else {
                    ErrorMsg_CheckConnections();
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                ShowBg(0u8);
                if ((((&raw mut sLinkErrorBuffer).cast::<u8>()).wrapping_add(6)).read()) != 0 {
                    ShowBg(1u8);
                }
                break 'l1;
            }
            if __sw1 == 30i32 {
                PlaySE(22u16);
                break 'l1;
            }
            if __sw1 == 60i32 {
                PlaySE(22u16);
                break 'l1;
            }
            if __sw1 == 90i32 {
                PlaySE(22u16);
                break 'l1;
            }
            if __sw1 == 130i32 {
                if ((((&raw mut gWirelessCommType).cast::<u8>().cast::<u8>()).read()) as i32)
                    == 2i32
                {
                    AddTextPrinterParameterized3(
                        0u8,
                        3u8,
                        2u8,
                        20u8,
                        ((&raw const sTextColors).cast::<u8>().cast_mut()).cast::<u8>(),
                        0i8,
                        (&raw mut gText_ABtnTitleScreen).cast::<u8>(),
                    );
                } else {
                    if ((((&raw mut gWirelessCommType).cast::<u8>().cast::<u8>()).read()) as i32)
                        == 1i32
                    {
                        AddTextPrinterParameterized3(
                            0u8,
                            3u8,
                            2u8,
                            20u8,
                            ((&raw const sTextColors).cast::<u8>().cast_mut()).cast::<u8>(),
                            0i8,
                            (&raw mut gText_ABtnRegistrationCounter).cast::<u8>(),
                        );
                    }
                }
                break 'l1;
            }
        }
        if (((((&raw mut gMain).cast::<u8>()).wrapping_add(1080)).read()) as i32) == 160i32 {
            if ((((&raw mut gWirelessCommType).cast::<u8>().cast::<u8>()).read()) as i32) == 1i32 {
                if ((((((&raw mut gMain).cast::<u8>())
                    .wrapping_add(46)
                    .cast::<u16>())
                .read()) as i32)
                    & 1i32)
                    != 0
                {
                    PlaySE(21u16);
                    ((&raw mut gWirelessCommType).cast::<u8>().cast::<u8>()).write(0u8);
                    (((&raw mut sLinkErrorBuffer).cast::<u8>()).wrapping_add(6)).write(0u8);
                    ReloadSave();
                }
            } else {
                if ((((&raw mut gWirelessCommType).cast::<u8>().cast::<u8>()).read()) as i32)
                    == 2i32
                {
                    if ((((((&raw mut gMain).cast::<u8>())
                        .wrapping_add(46)
                        .cast::<u16>())
                    .read()) as i32)
                        & 1i32)
                        != 0
                    {
                        rfu_REQ_stopMode();
                        rfu_waitREQComplete();
                        DoSoftReset();
                    }
                }
            }
        }
        if (((((&raw mut gMain).cast::<u8>()).wrapping_add(1080)).read()) as i32) != 160i32 {
            let __p2 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
            (__p2).write(((__p2).read()).wrapping_add(1));
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetSioMultiSI() -> u8 {
    unsafe {
        return (((((((67109160i32) as usize as *mut u16).read_volatile()) as i32) & 4i32) != 0i32)
            as u8);
    }
}
pub(crate) unsafe extern "C" fn IsSioMultiMaster() -> u8 {
    unsafe {
        return ((((((((67109160i32) as usize as *mut u16).read_volatile()) as i32) & 8i32) != 0)
            && ((((((67109160i32) as usize as *mut u16).read_volatile()) as i32) & 4i32) == 0i32))
            as u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsLinkConnectionEstablished() -> u8 {
    unsafe {
        return (((((&raw mut gLinkStatus).cast::<u8>().cast::<u32>()).read() >> 6) & 1u32) as u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetSuppressLinkErrorMessage(flag: u8) {
    unsafe {
        let mut flag = flag;
        ((&raw mut gSuppressLinkErrorMessage)
            .cast::<u8>()
            .cast::<u8>())
        .write(flag);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn HasLinkErrorOccurred() -> u8 {
    unsafe {
        return ((&raw mut gLinkErrorOccurred).cast::<u8>().cast::<u8>()).read();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn LocalLinkPlayerToBlock() {
    unsafe {
        let mut block: *mut u8 = core::ptr::null_mut();
        InitLocalLinkPlayer();
        block = (&raw mut gLocalLinkPlayerBlock).cast::<u8>();
        (block)
            .wrapping_add(16)
            .cast::<crate::c::Rec4<28>>()
            .write_unaligned(
                (&raw mut gLocalLinkPlayer)
                    .cast::<u8>()
                    .cast::<crate::c::Rec4<28>>()
                    .read_unaligned(),
            );
        crate::c::memcpy(
            (block).cast::<u8>(),
            ((&raw const sASCIIGameFreakInc).cast::<u8>().cast_mut()).cast::<u8>(),
            15u32,
        );
        crate::c::memcpy(
            ((block).wrapping_add(44)).cast::<u8>(),
            ((&raw const sASCIIGameFreakInc).cast::<u8>().cast_mut()).cast::<u8>(),
            15u32,
        );
        crate::c::memcpy(
            ((&raw mut gBlockSendBuffer).cast::<u8>()).cast::<u8>(),
            block,
            60u32,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn LinkPlayerFromBlock(who: u32) {
    unsafe {
        let mut who = who;
        let mut who_: u8 = ((who) as u8);
        let mut block: *mut u8 = core::ptr::null_mut();
        let mut player: *mut u8 = core::ptr::null_mut();
        block = (((((&raw mut gBlockRecvBuffer).cast::<u8>()).cast::<u8>())
            .wrapping_offset(((who_) as i32) as isize * 256))
        .cast::<u16>())
        .cast::<u8>();
        player = (((&raw mut gLinkPlayers).cast::<u8>()).cast::<u8>())
            .wrapping_offset(((who_) as i32) as isize * 28);
        player.cast::<crate::c::Rec4<28>>().write_unaligned(
            (block)
                .wrapping_add(16)
                .cast::<crate::c::Rec4<28>>()
                .read_unaligned(),
        );
        ConvertLinkPlayerName(player);
        if (crate::c::strcmp(
            (block).cast::<u8>(),
            ((&raw const sASCIIGameFreakInc).cast::<u8>().cast_mut()).cast::<u8>(),
        ) != 0i32)
            || (crate::c::strcmp(
                ((block).wrapping_add(44)).cast::<u8>(),
                ((&raw const sASCIIGameFreakInc).cast::<u8>().cast_mut()).cast::<u8>(),
            ) != 0i32)
        {
            SetMainCallback2(Some(CB2_LinkError));
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn HandleLinkConnection() -> u8 {
    unsafe {
        let mut main1Failed: u32 = 0u32;
        let mut main2Failed: u32 = 0u32;
        if ((((&raw mut gWirelessCommType).cast::<u8>().cast::<u8>()).read()) as i32) == 0i32 {
            ((&raw mut gLinkStatus).cast::<u8>().cast::<u32>()).write(LinkMain1(
                (&raw mut gShouldAdvanceLinkState).cast::<u8>().cast::<u8>(),
                ((&raw mut gSendCmd).cast::<u8>().cast::<u16>()).cast::<u16>(),
                ((&raw mut gRecvCmds).cast::<u8>()).cast::<u8>(),
            ));
            LinkMain2(
                ((&raw mut gMain).cast::<u8>())
                    .wrapping_add(44)
                    .cast::<u16>(),
            );
            if ((((&raw mut gLinkStatus).cast::<u8>().cast::<u32>()).read() & 256u32) != 0)
                && (IsSendingKeysOverCable() == 1u32)
            {
                return 1u8;
            }
        } else {
            main1Failed = RfuMain1();
            main2Failed = RfuMain2();
            if IsSendingKeysOverCable() == 1u32 {
                if ((main1Failed == 1u32) || ((IsRfuRecvQueueEmpty()) != 0)) || ((main2Failed) != 0)
                {
                    return 1u8;
                }
            }
        }
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetWirelessCommType1() {
    unsafe {
        if ((((&raw mut gReceivedRemoteLinkPlayers)
            .cast::<u8>()
            .cast::<u8>())
        .read()) as i32)
            == 0i32
        {
            ((&raw mut gWirelessCommType).cast::<u8>().cast::<u8>()).write(1u8);
        }
    }
}
pub(crate) unsafe extern "C" fn SetWirelessCommType0_Internal() {
    unsafe {
        if ((((&raw mut gReceivedRemoteLinkPlayers)
            .cast::<u8>()
            .cast::<u8>())
        .read()) as i32)
            == 0i32
        {
            ((&raw mut gWirelessCommType).cast::<u8>().cast::<u8>()).write(0u8);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetWirelessCommType0() {
    unsafe {
        if ((((&raw mut gReceivedRemoteLinkPlayers)
            .cast::<u8>()
            .cast::<u8>())
        .read()) as i32)
            == 0i32
        {
            ((&raw mut gWirelessCommType).cast::<u8>().cast::<u8>()).write(0u8);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetLinkRecvQueueLength() -> u32 {
    unsafe {
        if ((((&raw mut gWirelessCommType).cast::<u8>().cast::<u8>()).read()) as i32) != 0i32 {
            return GetRfuRecvQueueLength();
        }
        return ((((((&raw mut gLink).cast::<u8>()).wrapping_add(828)).wrapping_add(3201)).read())
            as u32);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsLinkRecvQueueAtOverworldMax() -> u32 {
    unsafe {
        if GetLinkRecvQueueLength() >= 3u32 {
            return 1u32;
        }
        return 0u32;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetWirelessCommType() -> u8 {
    unsafe {
        return ((&raw mut gWirelessCommType).cast::<u8>().cast::<u8>()).read();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ConvertLinkPlayerName(player: *mut u8) {
    unsafe {
        let mut player = player;
        ((player).wrapping_add(18)).write(((player).wrapping_add(16)).read());
        ConvertInternationalString(
            ((player).wrapping_add(8)).cast::<u8>(),
            ((((player).wrapping_add(26).cast::<u16>()).read()) as u8),
        );
    }
}
pub(crate) unsafe extern "C" fn DisableSerial() {
    unsafe {
        DisableInterrupts(192u16);
        crate::c::volatile_write(((67109160i32) as usize as *mut u16), 8192u16);
        crate::c::volatile_write(((67109134i32) as usize as *mut u16), 0u16);
        crate::c::volatile_write(((67109378i32) as usize as *mut u16), 192u16);
        crate::c::volatile_write(((67109162i32) as usize as *mut u16), 0u16);
        crate::c::volatile_write(((67109152i32) as usize as *mut u64), 0u64);
        'l1: loop {
            'l2: {
                {
                    let mut tmp: u32 = 0u32;
                    (&raw mut tmp).write_volatile(0u32);
                    'l3: loop {
                        'l4: {
                            CpuSet(
                                (&raw mut tmp).cast::<u8>(),
                                (&raw mut gLink).cast::<u8>(),
                                (83886080u32
                                    | (crate::c::div_u32(
                                        4032u32,
                                        ((crate::c::div_i32(32i32, 8i32)) as u32),
                                    ) & 2097151u32)),
                            );
                        }
                        if !((0i32) != 0) {
                            break 'l3;
                        }
                    }
                }
            }
            if !((0i32) != 0) {
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn EnableSerial() {
    unsafe {
        DisableInterrupts(192u16);
        crate::c::volatile_write(((67109172i32) as usize as *mut u16), 0u16);
        crate::c::volatile_write(((67109160i32) as usize as *mut u16), 8192u16);
        let __p1 = ((67109160i32) as usize as *mut u16);
        crate::c::volatile_write(
            __p1,
            (((((__p1).read_volatile()) as i32) | 16387i32) as u16),
        );
        EnableInterrupts(128u16);
        crate::c::volatile_write(((67109162i32) as usize as *mut u16), 0u16);
        'l1: loop {
            'l2: {
                {
                    let mut tmp: u32 = 0u32;
                    (&raw mut tmp).write_volatile(0u32);
                    'l3: loop {
                        'l4: {
                            CpuSet(
                                (&raw mut tmp).cast::<u8>(),
                                (&raw mut gLink).cast::<u8>(),
                                (83886080u32
                                    | (crate::c::div_u32(
                                        4032u32,
                                        ((crate::c::div_i32(32i32, 8i32)) as u32),
                                    ) & 2097151u32)),
                            );
                        }
                        if !((0i32) != 0) {
                            break 'l3;
                        }
                    }
                }
            }
            if !((0i32) != 0) {
                break 'l1;
            }
        }
        ((&raw mut sNumVBlanksWithoutSerialIntr)
            .cast::<u8>()
            .cast::<u8>())
        .write(0u8);
        ((&raw mut sSendNonzeroCheck).cast::<u8>().cast::<u16>()).write(0u16);
        ((&raw mut sRecvNonzeroCheck).cast::<u8>().cast::<u16>()).write(0u16);
        ((&raw mut sChecksumAvailable).cast::<u8>().cast::<u8>()).write(0u8);
        ((&raw mut sHandshakePlayerCount).cast::<u8>().cast::<u8>()).write(0u8);
        ((&raw mut gLastSendQueueCount).cast::<u8>().cast::<u8>()).write(0u8);
        ((&raw mut gLastRecvQueueCount).cast::<u8>().cast::<u8>()).write(0u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ResetSerial() {
    unsafe {
        EnableSerial();
        DisableSerial();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn LinkMain1(
    shouldAdvanceLinkState: *mut u8,
    sendCmd: *mut u16,
    recvCmds: *mut u8,
) -> u32 {
    unsafe {
        let mut shouldAdvanceLinkState = shouldAdvanceLinkState;
        let mut sendCmd = sendCmd;
        let mut recvCmds = recvCmds;
        let mut retVal: u32 = 0u32;
        let mut retVal2: u32 = 0u32;
        'l1: {
            let __sw1 = (((((&raw mut gLink).cast::<u8>()).wrapping_add(1)).read()) as i32);
            let mut __fall = false;
            if __sw1 == 0i32 {
                __fall = true;
                DisableSerial();
                (((&raw mut gLink).cast::<u8>()).wrapping_add(1)).write(1u8);
                break 'l1;
            }
            if __sw1 == 1i32 {
                __fall = true;
                if (((shouldAdvanceLinkState).read()) as i32) == 1i32 {
                    EnableSerial();
                    (((&raw mut gLink).cast::<u8>()).wrapping_add(1)).write(2u8);
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                __fall = true;
                'l2: {
                    let __sw2 = (((shouldAdvanceLinkState).read()) as i32);
                    let __matched = __sw2 == 1i32 || __sw2 == 2i32;
                    if !__matched {
                        CheckMasterOrSlave();
                        break 'l2;
                    }
                    if __sw2 == 1i32 {
                        if (((((&raw mut gLink).cast::<u8>()).read()) as i32) == 8i32)
                            && ((((((&raw mut gLink).cast::<u8>()).wrapping_add(3)).read()) as i32)
                                > 1i32)
                        {
                            (((&raw mut gLink).cast::<u8>()).wrapping_add(14)).write(1u8);
                        }
                        break 'l2;
                    }
                    if __sw2 == 2i32 {
                        (((&raw mut gLink).cast::<u8>()).wrapping_add(1)).write(0u8);
                        crate::c::volatile_write(((67109162i32) as usize as *mut u16), 0u16);
                        break 'l2;
                    }
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                __fall = true;
                InitTimer();
                (((&raw mut gLink).cast::<u8>()).wrapping_add(1)).write(4u8);
            }
            if __fall || __sw1 == 4i32 {
                __fall = true;
                EnqueueSendCmd(sendCmd);
                DequeueRecvCmds(recvCmds);
                break 'l1;
            }
        }
        (shouldAdvanceLinkState).write(0u8);
        retVal = (((((&raw mut gLink).cast::<u8>()).wrapping_add(2)).read()) as u32);
        retVal = (retVal
            | (((((((&raw mut gLink).cast::<u8>()).wrapping_add(3)).read()) as i32) << 2) as u32));
        if ((((&raw mut gLink).cast::<u8>()).read()) as i32) == 8i32 {
            retVal = (retVal | 32u32);
        }
        {
            let mut receivedNothing: u32 = (((((((&raw mut gLink).cast::<u8>()).wrapping_add(12))
                .read()) as i32)
                << 8) as u32);
            let mut link_field_F: u32 = (((((((&raw mut gLink).cast::<u8>()).wrapping_add(15))
                .read()) as i32)
                << 9) as u32);
            let mut hardwareError: u32 = (((((((&raw mut gLink).cast::<u8>()).wrapping_add(16))
                .read()) as i32)
                << 12) as u32);
            let mut badChecksum: u32 = (((((((&raw mut gLink).cast::<u8>()).wrapping_add(17))
                .read()) as i32)
                << 13) as u32);
            let mut queueFull: u32 = (((((((&raw mut gLink).cast::<u8>()).wrapping_add(18)).read())
                as i32)
                << 14) as u32);
            let mut val: u32 = 0u32;
            if (((((&raw mut gLink).cast::<u8>()).wrapping_add(1)).read()) as i32) == 4i32 {
                val = 64u32;
                val = (val | receivedNothing);
                val = (val | retVal);
                val = (val | link_field_F);
                val = (val | hardwareError);
                val = (val | badChecksum);
                val = (val | queueFull);
            } else {
                val = retVal;
                val = (val | receivedNothing);
                val = (val | link_field_F);
                val = (val | hardwareError);
                val = (val | badChecksum);
                val = (val | queueFull);
            }
            retVal = val;
        }
        if (((((&raw mut gLink).cast::<u8>()).wrapping_add(19)).read()) as i32) == 1i32 {
            retVal = (retVal | 65536u32);
        }
        if (((((&raw mut gLink).cast::<u8>()).wrapping_add(2)).read()) as i32) >= 4i32 {
            retVal = (retVal | 131072u32);
        }
        retVal2 = retVal;
        if (((((&raw mut gLink).cast::<u8>()).wrapping_add(19)).read()) as i32) == 2i32 {
            retVal2 = (retVal2 | 262144u32);
        }
        return retVal2;
    }
}
pub(crate) unsafe extern "C" fn CheckMasterOrSlave() {
    unsafe {
        let mut terminals: u32 = 0u32;
        terminals = (((67109160i32) as usize as *mut u32).read_volatile() & 12u32);
        if (terminals == 8u32)
            && ((((((&raw mut gLink).cast::<u8>()).wrapping_add(2)).read()) as i32) == 0i32)
        {
            ((&raw mut gLink).cast::<u8>()).write(8u8);
        } else {
            ((&raw mut gLink).cast::<u8>()).write(0u8);
        }
    }
}
pub(crate) unsafe extern "C" fn InitTimer() {
    unsafe {
        if (((&raw mut gLink).cast::<u8>()).read()) != 0 {
            crate::c::volatile_write(((67109132i32) as usize as *mut u16), 65339u16);
            crate::c::volatile_write(((67109134i32) as usize as *mut u16), 65u16);
            EnableInterrupts(64u16);
        }
    }
}
pub(crate) unsafe extern "C" fn EnqueueSendCmd(sendCmd: *mut u16) {
    unsafe {
        let mut sendCmd = sendCmd;
        let mut i: u8 = 0u8;
        let mut offset: u8 = 0u8;
        ((&raw mut gLinkSavedIme).cast::<u8>().cast::<u16>())
            .write(((67109384i32) as usize as *mut u16).read_volatile());
        crate::c::volatile_write(((67109384i32) as usize as *mut u16), 0u16);
        if ((((((&raw mut gLink).cast::<u8>()).wrapping_add(24)).wrapping_add(801)).read()) as i32)
            < 50i32
        {
            offset = ((((((((&raw mut gLink).cast::<u8>()).wrapping_add(24)).wrapping_add(800))
                .read()) as i32)
                .wrapping_add(
                    ((((((&raw mut gLink).cast::<u8>()).wrapping_add(24)).wrapping_add(801)).read())
                        as i32),
                )) as u8);
            if ((offset) as i32) >= 50i32 {
                offset = ((((offset) as i32).wrapping_sub(50i32)) as u8);
            }
            {
                i = 0u8;
                'l1: loop {
                    if !(((i) as i32) < 8i32) {
                        break 'l1;
                    }
                    'l2: {
                        let __p1 = (&raw mut sSendNonzeroCheck).cast::<u8>().cast::<u16>();
                        (__p1).write(
                            (((((__p1).read()) as i32) | (((sendCmd).read()) as i32)) as u16),
                        );
                        (((((((&raw mut gLink).cast::<u8>()).wrapping_add(24)).cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 100))
                        .cast::<u16>())
                        .wrapping_offset(((offset) as i32) as isize))
                        .write((sendCmd).read());
                        (sendCmd).write(0u16);
                        sendCmd = (sendCmd).wrapping_offset(1);
                    }
                    i = (i).wrapping_add(1);
                }
            }
        } else {
            (((&raw mut gLink).cast::<u8>()).wrapping_add(18)).write(1u8);
        }
        if (((&raw mut sSendNonzeroCheck).cast::<u8>().cast::<u16>()).read()) != 0 {
            let __p2 = (((&raw mut gLink).cast::<u8>()).wrapping_add(24)).wrapping_add(801);
            (__p2).write(((__p2).read()).wrapping_add(1));
            ((&raw mut sSendNonzeroCheck).cast::<u8>().cast::<u16>()).write(0u16);
        }
        crate::c::volatile_write(
            ((67109384i32) as usize as *mut u16),
            ((&raw mut gLinkSavedIme).cast::<u8>().cast::<u16>()).read(),
        );
        ((&raw mut gLastSendQueueCount).cast::<u8>().cast::<u8>())
            .write(((((&raw mut gLink).cast::<u8>()).wrapping_add(24)).wrapping_add(801)).read());
    }
}
pub(crate) unsafe extern "C" fn DequeueRecvCmds(recvCmds: *mut u8) {
    unsafe {
        let mut recvCmds = recvCmds;
        let mut i: u8 = 0u8;
        let mut j: u8 = 0u8;
        ((&raw mut gLinkSavedIme).cast::<u8>().cast::<u16>())
            .write(((67109384i32) as usize as *mut u16).read_volatile());
        crate::c::volatile_write(((67109384i32) as usize as *mut u16), 0u16);
        if ((((((&raw mut gLink).cast::<u8>()).wrapping_add(828)).wrapping_add(3201)).read())
            as i32)
            == 0i32
        {
            {
                i = 0u8;
                'l1: loop {
                    if !(((i) as i32)
                        < (((((&raw mut gLink).cast::<u8>()).wrapping_add(3)).read()) as i32))
                    {
                        break 'l1;
                    }
                    'l2: {
                        {
                            j = 0u8;
                            'l3: loop {
                                if !(((j) as i32) < 8i32) {
                                    break 'l3;
                                }
                                'l4: {
                                    ((((recvCmds).wrapping_offset(((i) as i32) as isize * 16))
                                        .cast::<u16>())
                                    .wrapping_offset(((j) as i32) as isize))
                                    .write(0u16);
                                }
                                j = (j).wrapping_add(1);
                            }
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
            (((&raw mut gLink).cast::<u8>()).wrapping_add(12)).write(1u8);
        } else {
            {
                i = 0u8;
                'l5: loop {
                    if !(((i) as i32)
                        < (((((&raw mut gLink).cast::<u8>()).wrapping_add(3)).read()) as i32))
                    {
                        break 'l5;
                    }
                    'l6: {
                        {
                            j = 0u8;
                            'l7: loop {
                                if !(((j) as i32) < 8i32) {
                                    break 'l7;
                                }
                                'l8: {
                                    ((((recvCmds).wrapping_offset(((i) as i32) as isize * 16))
                                        .cast::<u16>())
                                    .wrapping_offset(((j) as i32) as isize))
                                    .write(
                                        (((((((((&raw mut gLink).cast::<u8>())
                                            .wrapping_add(828))
                                        .cast::<u8>())
                                        .wrapping_offset(((i) as i32) as isize * 800))
                                        .cast::<u8>())
                                        .wrapping_offset(((j) as i32) as isize * 100))
                                        .cast::<u16>())
                                        .wrapping_offset(
                                            ((((((&raw mut gLink).cast::<u8>()).wrapping_add(828))
                                                .wrapping_add(3200))
                                            .read())
                                                as i32)
                                                as isize,
                                        ))
                                        .read(),
                                    );
                                }
                                j = (j).wrapping_add(1);
                            }
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
            let __p1 = (((&raw mut gLink).cast::<u8>()).wrapping_add(828)).wrapping_add(3201);
            (__p1).write(((__p1).read()).wrapping_sub(1));
            let __p2 = (((&raw mut gLink).cast::<u8>()).wrapping_add(828)).wrapping_add(3200);
            (__p2).write(((__p2).read()).wrapping_add(1));
            if ((((((&raw mut gLink).cast::<u8>()).wrapping_add(828)).wrapping_add(3200)).read())
                as i32)
                >= 50i32
            {
                ((((&raw mut gLink).cast::<u8>()).wrapping_add(828)).wrapping_add(3200)).write(0u8);
            }
            (((&raw mut gLink).cast::<u8>()).wrapping_add(12)).write(0u8);
        }
        crate::c::volatile_write(
            ((67109384i32) as usize as *mut u16),
            ((&raw mut gLinkSavedIme).cast::<u8>().cast::<u16>()).read(),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn LinkVSync() {
    unsafe {
        if (((&raw mut gLink).cast::<u8>()).read()) != 0 {
            'l1: {
                let __sw1 = (((((&raw mut gLink).cast::<u8>()).wrapping_add(1)).read()) as i32);
                if __sw1 == 4i32 {
                    if (((((&raw mut gLink).cast::<u8>())
                        .wrapping_add(13)
                        .cast::<i8>())
                    .read()) as i32)
                        < 9i32
                    {
                        if (((((&raw mut gLink).cast::<u8>()).wrapping_add(16)).read()) as i32)
                            != 1i32
                        {
                            (((&raw mut gLink).cast::<u8>()).wrapping_add(19)).write(1u8);
                        } else {
                            StartTransfer();
                        }
                    } else {
                        if (((((&raw mut gLink).cast::<u8>()).wrapping_add(19)).read()) as i32)
                            != 1i32
                        {
                            (((&raw mut gLink).cast::<u8>())
                                .wrapping_add(13)
                                .cast::<i8>())
                            .write(0i8);
                            StartTransfer();
                        }
                    }
                    break 'l1;
                }
                if __sw1 == 2i32 {
                    StartTransfer();
                    break 'l1;
                }
            }
        } else {
            if ((((((&raw mut gLink).cast::<u8>()).wrapping_add(1)).read()) as i32) == 4i32)
                || ((((((&raw mut gLink).cast::<u8>()).wrapping_add(1)).read()) as i32) == 2i32)
            {
                if (({
                    let __p2 = (&raw mut sNumVBlanksWithoutSerialIntr)
                        .cast::<u8>()
                        .cast::<u8>();
                    let __t3 = ((__p2).read()).wrapping_add(1);
                    (__p2).write(__t3);
                    __t3
                }) as i32)
                    > 10i32
                {
                    if (((((&raw mut gLink).cast::<u8>()).wrapping_add(1)).read()) as i32) == 4i32 {
                        (((&raw mut gLink).cast::<u8>()).wrapping_add(19)).write(2u8);
                    }
                    if (((((&raw mut gLink).cast::<u8>()).wrapping_add(1)).read()) as i32) == 2i32 {
                        (((&raw mut gLink).cast::<u8>()).wrapping_add(3)).write(0u8);
                        (((&raw mut gLink).cast::<u8>()).wrapping_add(15)).write(0u8);
                    }
                }
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Timer3Intr() {
    unsafe {
        StopTimer();
        StartTransfer();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SerialCB() {
    unsafe {
        (((&raw mut gLink).cast::<u8>()).wrapping_add(2)).write(
            ((crate::c::bf_read(
                ((67109160i32) as usize as *mut u8).wrapping_add(0),
                4,
                2,
                false,
            ) as u16) as u8),
        );
        'l1: {
            let __sw1 = (((((&raw mut gLink).cast::<u8>()).wrapping_add(1)).read()) as i32);
            if __sw1 == 4i32 {
                (((&raw mut gLink).cast::<u8>()).wrapping_add(16)).write(
                    ((crate::c::bf_read(
                        ((67109160i32) as usize as *mut u8).wrapping_add(0),
                        6,
                        1,
                        false,
                    ) as u16) as u8),
                );
                DoRecv();
                DoSend();
                SendRecvDone();
                break 'l1;
            }
            if __sw1 == 2i32 {
                if (DoHandshake()) != 0 {
                    if (((&raw mut gLink).cast::<u8>()).read()) != 0 {
                        (((&raw mut gLink).cast::<u8>()).wrapping_add(1)).write(3u8);
                        (((&raw mut gLink).cast::<u8>())
                            .wrapping_add(13)
                            .cast::<i8>())
                        .write(8i8);
                    } else {
                        (((&raw mut gLink).cast::<u8>()).wrapping_add(1)).write(4u8);
                    }
                }
                break 'l1;
            }
        }
        let __p2 = ((&raw mut gLink).cast::<u8>())
            .wrapping_add(13)
            .cast::<i8>();
        (__p2).write(((__p2).read()).wrapping_add(1));
        ((&raw mut sNumVBlanksWithoutSerialIntr)
            .cast::<u8>()
            .cast::<u8>())
        .write(0u8);
        if (((((&raw mut gLink).cast::<u8>())
            .wrapping_add(13)
            .cast::<i8>())
        .read()) as i32)
            == 8i32
        {
            ((&raw mut gLastRecvQueueCount).cast::<u8>().cast::<u8>()).write(
                ((((&raw mut gLink).cast::<u8>()).wrapping_add(828)).wrapping_add(3201)).read(),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn StartTransfer() {
    unsafe {
        let __p1 = ((67109160i32) as usize as *mut u16);
        crate::c::volatile_write(__p1, (((((__p1).read_volatile()) as i32) | 128i32) as u16));
    }
}
pub(crate) unsafe extern "C" fn DoHandshake() -> u8 {
    unsafe {
        let mut i: u8 = 0u8;
        let mut playerCount: u8 = 0u8;
        let mut minRecv: u16 = 0u16;
        let mut recvSiomlt: u64 = 0u64;
        playerCount = 0u8;
        minRecv = 65535u16;
        if (((((&raw mut gLink).cast::<u8>()).wrapping_add(14)).read()) as i32) == 1i32 {
            crate::c::volatile_write(((67109162i32) as usize as *mut u16), 36863u16);
        } else {
            crate::c::volatile_write(((67109162i32) as usize as *mut u16), 47520u16);
        }
        recvSiomlt = ((67109152i32) as usize as *mut u64).read_volatile();
        crate::c::memcpy(
            ((((&raw mut gLink).cast::<u8>()).wrapping_add(4)).cast::<u16>()).cast::<u8>(),
            (&raw mut recvSiomlt).cast::<u8>(),
            8u32,
        );
        crate::c::volatile_write(((67109152i32) as usize as *mut u64), 0u64);
        (((&raw mut gLink).cast::<u8>()).wrapping_add(14)).write(0u8);
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 4i32) {
                    break 'l1;
                }
                'l2: {
                    if (((((((((&raw mut gLink).cast::<u8>()).wrapping_add(4)).cast::<u16>())
                        .wrapping_offset(((i) as i32) as isize))
                    .read()) as i32)
                        & (-4i32))
                        == 47520i32)
                        || ((((((((&raw mut gLink).cast::<u8>()).wrapping_add(4)).cast::<u16>())
                            .wrapping_offset(((i) as i32) as isize))
                        .read()) as i32)
                            == 36863i32)
                    {
                        playerCount = (playerCount).wrapping_add(1);
                        if (((minRecv) as i32)
                            > (((((((&raw mut gLink).cast::<u8>()).wrapping_add(4)).cast::<u16>())
                                .wrapping_offset(((i) as i32) as isize))
                            .read()) as i32))
                            && ((((((((&raw mut gLink).cast::<u8>()).wrapping_add(4))
                                .cast::<u16>())
                            .wrapping_offset(((i) as i32) as isize))
                            .read()) as i32)
                                != 0i32)
                        {
                            minRecv = (((((&raw mut gLink).cast::<u8>()).wrapping_add(4))
                                .cast::<u16>())
                            .wrapping_offset(((i) as i32) as isize))
                            .read();
                        }
                    } else {
                        if (((((((&raw mut gLink).cast::<u8>()).wrapping_add(4)).cast::<u16>())
                            .wrapping_offset(((i) as i32) as isize))
                        .read()) as i32)
                            != 65535i32
                        {
                            playerCount = 0u8;
                        }
                        break 'l1;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        (((&raw mut gLink).cast::<u8>()).wrapping_add(3)).write(playerCount);
        if (((((((&raw mut gLink).cast::<u8>()).wrapping_add(3)).read()) as i32) > 1i32)
            && ((((((&raw mut gLink).cast::<u8>()).wrapping_add(3)).read()) as i32)
                == ((((&raw mut sHandshakePlayerCount).cast::<u8>().cast::<u8>()).read()) as i32)))
            && (((((((&raw mut gLink).cast::<u8>()).wrapping_add(4)).cast::<u16>()).read()) as i32)
                == 36863i32)
        {
            return 1u8;
        }
        if (((((&raw mut gLink).cast::<u8>()).wrapping_add(3)).read()) as i32) > 1i32 {
            (((&raw mut gLink).cast::<u8>()).wrapping_add(15))
                .write((((((minRecv) as i32) & 3i32).wrapping_add(1i32)) as u8));
        } else {
            (((&raw mut gLink).cast::<u8>()).wrapping_add(15)).write(0u8);
        }
        ((&raw mut sHandshakePlayerCount).cast::<u8>().cast::<u8>())
            .write((((&raw mut gLink).cast::<u8>()).wrapping_add(3)).read());
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn DoRecv() {
    unsafe {
        let mut recv = crate::ffi::Align4([0u8; 8]);
        let mut i: u8 = 0u8;
        let mut index: u8 = 0u8;
        let mut recvSiomlt: u64 = ((67109152i32) as usize as *mut u64).read_volatile();
        crate::c::memcpy(
            ((&raw mut recv).cast::<u16>()).cast::<u8>(),
            (&raw mut recvSiomlt).cast::<u8>(),
            8u32,
        );
        if (((((&raw mut gLink).cast::<u8>()).wrapping_add(22)).read()) as i32) == 0i32 {
            {
                i = 0u8;
                'l1: loop {
                    if !(((i) as i32)
                        < (((((&raw mut gLink).cast::<u8>()).wrapping_add(3)).read()) as i32))
                    {
                        break 'l1;
                    }
                    'l2: {
                        if ((((((&raw mut gLink).cast::<u8>())
                            .wrapping_add(20)
                            .cast::<u16>())
                        .read()) as i32)
                            != (((((&raw mut recv).cast::<u16>())
                                .wrapping_offset(((i) as i32) as isize))
                            .read()) as i32))
                            && ((((&raw mut sChecksumAvailable).cast::<u8>().cast::<u8>()).read())
                                != 0)
                        {
                            (((&raw mut gLink).cast::<u8>()).wrapping_add(17)).write(1u8);
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
            (((&raw mut gLink).cast::<u8>())
                .wrapping_add(20)
                .cast::<u16>())
            .write(0u16);
            ((&raw mut sChecksumAvailable).cast::<u8>().cast::<u8>()).write(1u8);
        } else {
            index = ((((((((&raw mut gLink).cast::<u8>()).wrapping_add(828)).wrapping_add(3200))
                .read()) as i32)
                .wrapping_add(
                    ((((((&raw mut gLink).cast::<u8>()).wrapping_add(828)).wrapping_add(3201))
                        .read()) as i32),
                )) as u8);
            if ((index) as i32) >= 50i32 {
                index = ((((index) as i32).wrapping_sub(50i32)) as u8);
            }
            if ((((((&raw mut gLink).cast::<u8>()).wrapping_add(828)).wrapping_add(3201)).read())
                as i32)
                < 50i32
            {
                {
                    i = 0u8;
                    'l3: loop {
                        if !(((i) as i32)
                            < (((((&raw mut gLink).cast::<u8>()).wrapping_add(3)).read()) as i32))
                        {
                            break 'l3;
                        }
                        'l4: {
                            let __p1 = ((&raw mut gLink).cast::<u8>())
                                .wrapping_add(20)
                                .cast::<u16>();
                            (__p1).write(
                                (((((__p1).read()) as i32).wrapping_add(
                                    (((((&raw mut recv).cast::<u16>())
                                        .wrapping_offset(((i) as i32) as isize))
                                    .read()) as i32),
                                )) as u16),
                            );
                            let __p2 = (&raw mut sRecvNonzeroCheck).cast::<u8>().cast::<u16>();
                            (__p2).write(
                                (((((__p2).read()) as i32)
                                    | (((((&raw mut recv).cast::<u16>())
                                        .wrapping_offset(((i) as i32) as isize))
                                    .read()) as i32)) as u16),
                            );
                            (((((((((&raw mut gLink).cast::<u8>()).wrapping_add(828))
                                .cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 800))
                            .cast::<u8>())
                            .wrapping_offset(
                                (((((&raw mut gLink).cast::<u8>()).wrapping_add(23)).read()) as i32)
                                    as isize
                                    * 100,
                            ))
                            .cast::<u16>())
                            .wrapping_offset(((index) as i32) as isize))
                            .write(
                                (((&raw mut recv).cast::<u16>())
                                    .wrapping_offset(((i) as i32) as isize))
                                .read(),
                            );
                        }
                        i = (i).wrapping_add(1);
                    }
                }
            } else {
                (((&raw mut gLink).cast::<u8>()).wrapping_add(18)).write(2u8);
            }
            let __p3 = ((&raw mut gLink).cast::<u8>()).wrapping_add(23);
            (__p3).write(((__p3).read()).wrapping_add(1));
            if ((((((&raw mut gLink).cast::<u8>()).wrapping_add(23)).read()) as i32) == 8i32)
                && ((((&raw mut sRecvNonzeroCheck).cast::<u8>().cast::<u16>()).read()) != 0)
            {
                let __p4 = (((&raw mut gLink).cast::<u8>()).wrapping_add(828)).wrapping_add(3201);
                (__p4).write(((__p4).read()).wrapping_add(1));
                ((&raw mut sRecvNonzeroCheck).cast::<u8>().cast::<u16>()).write(0u16);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn DoSend() {
    unsafe {
        if (((((&raw mut gLink).cast::<u8>()).wrapping_add(22)).read()) as i32) == 8i32 {
            crate::c::volatile_write(
                ((67109162i32) as usize as *mut u16),
                (((&raw mut gLink).cast::<u8>())
                    .wrapping_add(20)
                    .cast::<u16>())
                .read(),
            );
            if !((((&raw mut sSendBufferEmpty).cast::<u8>().cast::<u8>()).read()) != 0) {
                let __p1 = (((&raw mut gLink).cast::<u8>()).wrapping_add(24)).wrapping_add(801);
                (__p1).write(((__p1).read()).wrapping_sub(1));
                let __p2 = (((&raw mut gLink).cast::<u8>()).wrapping_add(24)).wrapping_add(800);
                (__p2).write(((__p2).read()).wrapping_add(1));
                if ((((((&raw mut gLink).cast::<u8>()).wrapping_add(24)).wrapping_add(800)).read())
                    as i32)
                    >= 50i32
                {
                    ((((&raw mut gLink).cast::<u8>()).wrapping_add(24)).wrapping_add(800))
                        .write(0u8);
                }
            } else {
                ((&raw mut sSendBufferEmpty).cast::<u8>().cast::<u8>()).write(0u8);
            }
        } else {
            if (!((((&raw mut sSendBufferEmpty).cast::<u8>().cast::<u8>()).read()) != 0))
                && (((((((&raw mut gLink).cast::<u8>()).wrapping_add(24)).wrapping_add(801)).read())
                    as i32)
                    == 0i32)
            {
                ((&raw mut sSendBufferEmpty).cast::<u8>().cast::<u8>()).write(1u8);
            }
            if (((&raw mut sSendBufferEmpty).cast::<u8>().cast::<u8>()).read()) != 0 {
                crate::c::volatile_write(((67109162i32) as usize as *mut u16), 0u16);
            } else {
                crate::c::volatile_write(
                    ((67109162i32) as usize as *mut u16),
                    (((((((&raw mut gLink).cast::<u8>()).wrapping_add(24)).cast::<u8>())
                        .wrapping_offset(
                            (((((&raw mut gLink).cast::<u8>()).wrapping_add(22)).read()) as i32)
                                as isize
                                * 100,
                        ))
                    .cast::<u16>())
                    .wrapping_offset(
                        ((((((&raw mut gLink).cast::<u8>()).wrapping_add(24)).wrapping_add(800))
                            .read()) as i32) as isize,
                    ))
                    .read(),
                );
            }
            let __p3 = ((&raw mut gLink).cast::<u8>()).wrapping_add(22);
            (__p3).write(((__p3).read()).wrapping_add(1));
        }
    }
}
pub(crate) unsafe extern "C" fn StopTimer() {
    unsafe {
        if (((&raw mut gLink).cast::<u8>()).read()) != 0 {
            let __p1 = ((67109134i32) as usize as *mut u16);
            crate::c::volatile_write(
                __p1,
                (((((__p1).read_volatile()) as i32) & (-129i32)) as u16),
            );
            crate::c::volatile_write(((67109132i32) as usize as *mut u16), 65339u16);
        }
    }
}
pub(crate) unsafe extern "C" fn SendRecvDone() {
    unsafe {
        if (((((&raw mut gLink).cast::<u8>()).wrapping_add(23)).read()) as i32) == 8i32 {
            (((&raw mut gLink).cast::<u8>()).wrapping_add(22)).write(0u8);
            (((&raw mut gLink).cast::<u8>()).wrapping_add(23)).write(0u8);
        } else {
            if (((&raw mut gLink).cast::<u8>()).read()) != 0 {
                let __p1 = ((67109134i32) as usize as *mut u16);
                crate::c::volatile_write(
                    __p1,
                    (((((__p1).read_volatile()) as i32) | 128i32) as u16),
                );
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ResetSendBuffer() {
    unsafe {
        let mut i: u8 = 0u8;
        let mut j: u8 = 0u8;
        ((((&raw mut gLink).cast::<u8>()).wrapping_add(24)).wrapping_add(801)).write(0u8);
        ((((&raw mut gLink).cast::<u8>()).wrapping_add(24)).wrapping_add(800)).write(0u8);
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 8i32) {
                    break 'l1;
                }
                'l2: {
                    {
                        j = 0u8;
                        'l3: loop {
                            if !(((j) as i32) < 50i32) {
                                break 'l3;
                            }
                            'l4: {
                                (((((((&raw mut gLink).cast::<u8>()).wrapping_add(24))
                                    .cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize * 100))
                                .cast::<u16>())
                                .wrapping_offset(((j) as i32) as isize))
                                .write(61439u16);
                            }
                            j = (j).wrapping_add(1);
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ResetRecvBuffer() {
    unsafe {
        let mut i: u8 = 0u8;
        let mut j: u8 = 0u8;
        let mut k: u8 = 0u8;
        ((((&raw mut gLink).cast::<u8>()).wrapping_add(828)).wrapping_add(3201)).write(0u8);
        ((((&raw mut gLink).cast::<u8>()).wrapping_add(828)).wrapping_add(3200)).write(0u8);
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 4i32) {
                    break 'l1;
                }
                'l2: {
                    {
                        j = 0u8;
                        'l3: loop {
                            if !(((j) as i32) < 8i32) {
                                break 'l3;
                            }
                            'l4: {
                                {
                                    k = 0u8;
                                    'l5: loop {
                                        if !(((k) as i32) < 50i32) {
                                            break 'l5;
                                        }
                                        'l6: {
                                            (((((((((&raw mut gLink).cast::<u8>())
                                                .wrapping_add(828))
                                            .cast::<u8>())
                                            .wrapping_offset(((i) as i32) as isize * 800))
                                            .cast::<u8>())
                                            .wrapping_offset(((j) as i32) as isize * 100))
                                            .cast::<u16>())
                                            .wrapping_offset(((k) as i32) as isize))
                                            .write(61439u16);
                                        }
                                        k = (k).wrapping_add(1);
                                    }
                                }
                            }
                            j = (j).wrapping_add(1);
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SetBackdropFromColor(color: u16) {
    unsafe {
        let mut color = color;
        FillPalette(color, 0u16, 2u16);
    }
}
