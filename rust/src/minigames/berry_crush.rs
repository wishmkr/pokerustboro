//! Translated from `src/berry_crush.c` by tools/rustport/c2rs.py, then reviewed.
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

// Data tables (translate with cdata.py): sBitTable sSyncPressBonus sIntroOutroVibrationData sVibrationData sMessages sBgTemplates sTextColorTable sWindowTemplate_Rankings sWindowTemplates_PlayerNames sWindowTemplates_Results sResultsWindowHeights sPressingSpeedConversionTable sCrusherBase_Pal sEffects_Pal sTimerDigits_Pal sCrusherBase_Gfx sImpact_Gfx sSparkle_Gfx sTimerDigits_Gfx sCrusherTop_Tilemap sContainerCap_Tilemap sBg_Tilemap sPlayerIdToPosId sPlayerCoords sImpactCoords sSparkleCoords sPlayerBerrySpriteTags sSpriteSheets sSpritePals sAnim_CrusherBase sAnim_Impact_Small sAnim_Impact_Big sAnim_Sparkle_Small sAnim_Sparkle_Big sAnim_Timer sAnim_PlayerBerry sAffineAnim_PlayerBerry_0 sAffineAnim_PlayerBerry_1 sAnims_CrusherBase sAnims_Impact sAnims_Sparkle sAnims_Timer sAnims_PlayerBerry sAffineAnims_PlayerBerry sSpriteTemplate_CrusherBase sSpriteTemplate_Impact sSpriteTemplate_Sparkle sSpriteTemplate_Timer sSpriteTemplate_PlayerBerry sDigitObjTemplates sResultsTexts sBerryCrushCommands sSparkleThresholds sBigSparkleThresholds sReceivedPlayerBitmasks
#[allow(unused_imports)]
use crate::data::berry_crush::*;

#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sGame: *mut u8 = core::ptr::null_mut();

unsafe extern "C" {
    static mut gBerries: u8;
    static mut gBerryCrush_BerryData: u8;
    static mut gBerryCrush_Crusher_Gfx: u8;
    static mut gBerryCrush_Crusher_Pal: u8;
    static mut gBerryCrush_TextWindows_Tilemap: u8;
    static mut gBlockRecvBuffer: u8;
    static mut gDecompressionBuffer: u8;
    static mut gLinkPlayers: u8;
    static mut gMain: u8;
    static mut gPaletteFade: u8;
    static mut gReceivedRemoteLinkPlayers: u8;
    static mut gRecvCmds: u8;
    static mut gReservedSpritePaletteCount: u8;
    static mut gRfu: u8;
    static mut gSaveBlock2Ptr: u8;
    static mut gSpecialVar_ItemId: u8;
    static mut gSpriteCoordOffsetX: u8;
    static mut gSpriteCoordOffsetY: u8;
    static mut gSprites: u8;
    static mut gStringVar1: u8;
    static mut gStringVar2: u8;
    static mut gStringVar3: u8;
    static mut gStringVar4: u8;
    static mut gTasks: u8;
    static mut gTextFlags: u8;
    static mut gText_1DotBlueF700: u8;
    static mut gText_1DotF700: u8;
    static mut gText_BerryCrush2: u8;
    static mut gText_CrushingResults: u8;
    static mut gText_PressesRankings: u8;
    static mut gText_PressingSpeed: u8;
    static mut gText_PressingSpeedRankings: u8;
    static mut gText_SavingDontTurnOffPower: u8;
    static mut gText_Silkiness: u8;
    static mut gText_SpaceMin: u8;
    static mut gText_SpaceSec: u8;
    static mut gText_StrVar1: u8;
    static mut gText_TimeColon: u8;
    static mut gText_TimesPerSec: u8;
    static mut gText_Var1Percent: u8;
    static mut gText_Var1Players: u8;
    static mut gText_XDotY2: u8;
    static mut gText_XDotY3: u8;
    static mut gWirelessCommType: u8;
    fn AddCustomItemIconSprite(a0: *mut u8, a1: u16, a2: u16, a3: u16) -> u8;
    fn AddTextPrinterParameterized2(
        a0: u8,
        a1: u8,
        a2: *mut u8,
        a3: u8,
        a4: Option<unsafe extern "C" fn(*mut u8, u16)>,
        a5: u8,
        a6: u8,
        a7: u8,
    ) -> u16;
    fn AddTextPrinterParameterized3(
        a0: u8,
        a1: u8,
        a2: u8,
        a3: u8,
        a4: *mut u8,
        a5: i8,
        a6: *mut u8,
    );
    fn AddTextPrinterParameterized4(
        a0: u8,
        a1: u8,
        a2: u8,
        a3: u8,
        a4: u8,
        a5: u8,
        a6: *mut u8,
        a7: i8,
        a8: *mut u8,
    );
    fn AddWindow(a0: *mut u8) -> u16;
    fn AllocZeroed(a0: u32) -> *mut u8;
    fn AnimateSprites();
    fn BeginNormalPaletteFade(a0: u32, a1: i8, a2: u8, a3: u8, a4: u16) -> u8;
    fn BlendPalettes(a0: u32, a1: u8, a2: u16);
    fn BuildOamBuffer();
    fn CB1_Overworld();
    fn CB2_ReturnToField();
    fn ChangeBgX(a0: u8, a1: i32, a2: u8) -> i32;
    fn ChangeBgY(a0: u8, a1: i32, a2: u8) -> i32;
    fn ChooseBerryForMachine(a0: Option<unsafe extern "C" fn()>);
    fn ClearDialogWindowAndFrame(a0: u8, a1: u8);
    fn ClearGpuRegBits(a0: u8, a1: u16);
    fn ClearRecvCommands();
    fn ClearStdWindowAndFrameToTransparent(a0: u8, a1: u8);
    fn ClearWindowTilemap(a0: u8);
    fn ConvertIntToDecimalStringN(a0: *mut u8, a1: i32, a2: i32, a3: u8) -> *mut u8;
    fn CopyBgTilemapBufferToVram(a0: u8);
    fn CopyToBgTilemapBuffer(a0: u8, a1: *mut u8, a2: u16, a3: u16);
    fn CopyToBgTilemapBufferRect(a0: u8, a1: *mut u8, a2: u8, a3: u8, a4: u8, a5: u8);
    fn CopyWindowToVram(a0: u8, a1: u8);
    fn CpuSet(a0: *mut u8, a1: *mut u8, a2: u32);
    fn CreateSprite(a0: *mut u8, a1: i16, a2: i16, a3: u8) -> u8;
    fn CreateTask(a0: Option<unsafe extern "C" fn(u8)>, a1: u8) -> u8;
    fn CreateWirelessStatusIndicatorSprite(a0: u8, a1: u8);
    fn DecompressAndCopyTileDataToVram(a0: u8, a1: *mut u8, a2: u32, a3: u16, a4: u8) -> *mut u8;
    fn DestroySprite(a0: *mut u8);
    fn DestroyTask(a0: u8);
    fn DestroyWirelessStatusIndicatorSprite();
    fn DigitObjUtil_CreatePrinter(a0: u32, a1: i32, a2: *mut u8) -> u32;
    fn DigitObjUtil_DeletePrinter(a0: u32);
    fn DigitObjUtil_Free();
    fn DigitObjUtil_HideOrShow(a0: u32, a1: u32);
    fn DigitObjUtil_Init(a0: u32) -> u32;
    fn DigitObjUtil_PrintNumOn(a0: u32, a1: i32);
    fn DisplayYesNoMenuDefaultYes();
    fn DrawDialogueFrame(a0: u8, a1: u8);
    fn DrawStdFrameWithCustomTileAndPalette(a0: u8, a1: u8, a2: u16, a3: u8);
    fn DynamicPlaceholderTextUtil_ExpandPlaceholders(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn DynamicPlaceholderTextUtil_Reset();
    fn DynamicPlaceholderTextUtil_SetPlaceholderPtr(a0: u8, a1: *mut u8);
    fn FillBgTilemapBufferRect_Palette0(a0: u8, a1: u16, a2: u8, a3: u8, a4: u8, a5: u8);
    fn FillWindowPixelBuffer(a0: u8, a1: u8);
    fn Free(a0: *mut u8);
    fn FreeAllSpritePalettes();
    fn FreeAllWindowBuffers();
    fn FreeSpriteOamMatrix(a0: *mut u8);
    fn FreeSpritePaletteByTag(a0: u16);
    fn FreeSpriteTilesByTag(a0: u16);
    fn FreeTempTileDataBuffersIfPossible() -> u8;
    fn FuncIsActiveTask(a0: Option<unsafe extern "C" fn(u8)>) -> u8;
    fn GetBerryPowder() -> u32;
    fn GetBlockReceivedStatus() -> u8;
    fn GetLinkPlayerCount() -> u8;
    fn GetMultiplayerId() -> u8;
    fn GetStringRightAlignXOffset(a0: i32, a1: *mut u8, a2: i32) -> i32;
    fn GetStringWidth(a0: u8, a1: *mut u8, a2: i16) -> i32;
    fn GetWindowAttribute(a0: u8, a1: u8) -> u32;
    fn GiveBerryPowder(a0: u32) -> u8;
    fn HasAtLeastOneBerry() -> u8;
    fn HideBg(a0: u8);
    fn IncrementGameStat(a0: u8);
    fn InitBgsFromTemplates(a0: u8, a1: *mut u8, a2: u8);
    fn InitStandardTextBoxWindows();
    fn InitTextBoxGfxAndPrinters();
    fn IsLinkTaskFinished() -> u8;
    fn IsMinigameCountdownRunning() -> u32;
    fn IsTextPrinterActive(a0: u8) -> u16;
    fn LZ77UnCompWram(a0: *mut u32, a1: *mut u8);
    fn LoadCompressedSpriteSheet(a0: *mut u8) -> u16;
    fn LoadOam();
    fn LoadPalette(a0: *mut u8, a1: u16, a2: u16);
    fn LoadSpritePalettes(a0: *mut u8);
    fn LoadUserWindowBorderGfx_(a0: u8, a1: u16, a2: u8);
    fn LoadWirelessStatusIndicatorSpriteGfx();
    fn LockPlayerFieldControls();
    fn MathUtil_Div16Shift(a0: u8, a1: i16, a2: i16) -> i16;
    fn MathUtil_Div32(a0: i32, a1: i32) -> i32;
    fn MathUtil_Mul16(a0: i16, a1: i16) -> i16;
    fn MathUtil_Mul16Shift(a0: u8, a1: i16, a2: i16) -> i16;
    fn MathUtil_Mul32(a0: i32, a1: i32) -> i32;
    fn Menu_ProcessInputNoWrapClearOnChoose() -> i8;
    fn PlayNewMapMusic(a0: u16);
    fn PlaySE(a0: u16);
    fn ProcessSpriteCopyRequests();
    fn PutWindowTilemap(a0: u8);
    fn Random() -> u16;
    fn RemoveBagItem(a0: u16, a1: u16) -> u8;
    fn RemoveWindow(a0: u8);
    fn ResetBgsAndClearDma3BusyFlags(a0: u32);
    fn ResetBlockReceivedFlags();
    fn ResetPaletteFade();
    fn ResetSpriteData();
    fn ResetTempTileDataBuffers();
    fn Rfu_SendPacket(a0: *mut u8);
    fn Rfu_SetLinkStandbyCallback();
    fn RunTasks();
    fn RunTextPrinters();
    fn ScanlineEffect_Stop();
    fn ScriptContext_Enable();
    fn SendBlock(a0: u8, a1: *mut u8, a2: u16) -> u8;
    fn SetBgTilemapBuffer(a0: u8, a1: *mut u8);
    fn SetCloseLinkCallback();
    fn SetGpuReg(a0: u8, a1: u16);
    fn SetGpuRegBits(a0: u8, a1: u16);
    fn SetHBlankCallback(a0: Option<unsafe extern "C" fn()>);
    fn SetMainCallback1(a0: Option<unsafe extern "C" fn()>);
    fn SetMainCallback2(a0: Option<unsafe extern "C" fn()>);
    fn SetVBlankCallback(a0: Option<unsafe extern "C" fn()>);
    fn ShowBg(a0: u8);
    fn Sin(a0: i16, a1: i16) -> i16;
    fn SpriteCallbackDummy(a0: *mut u8);
    fn StartMinigameCountdown(a0: u16, a1: u16, a2: i16, a3: i16, a4: u8);
    fn StartSpriteAffineAnim(a0: *mut u8, a1: u8);
    fn StartSpriteAnim(a0: *mut u8, a1: u8);
    fn StringAppend(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn StringCopy(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn StringExpandPlaceholders(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn Task_LinkFullSave(a0: u8);
    fn TransferPlttBuffer();
    fn UnlockPlayerFieldControls();
    fn UnsetBgTilemapBuffer(a0: u8);
    fn UpdatePaletteFade() -> u8;
}

pub(crate) unsafe extern "C" fn GetBerryCrushGame() -> *mut u8 {
    unsafe {
        return ((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read();
    }
}
pub(crate) unsafe extern "C" fn QuitBerryCrush(
    exitCallback: Option<unsafe extern "C" fn()>,
) -> u32 {
    unsafe {
        let mut exitCallback = exitCallback;
        if !(!(((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).is_null()) {
            return 2u32;
        }
        if !((exitCallback).is_some()) {
            exitCallback = ((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                .cast::<Option<unsafe extern "C" fn()>>())
            .read();
        }
        DestroyTask(
            ((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(10)).read(),
        );
        {
            Free(((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read());
            ((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).write(core::ptr::null_mut());
        }
        SetMainCallback2(exitCallback);
        if core::mem::transmute::<_, usize>(exitCallback)
            == (CB2_ReturnToField as *const () as usize)
        {
            crate::c::bf_write(
                ((&raw mut gTextFlags).cast::<u8>()).wrapping_add(0),
                2,
                1,
                (1u8) as i32,
            );
            PlayNewMapMusic(400u16);
            SetMainCallback1(Some(CB1_Overworld));
        }
        return 0u32;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn StartBerryCrush(exitCallback: Option<unsafe extern "C" fn()>) {
    unsafe {
        let mut exitCallback = exitCallback;
        let mut playerCount: u8 = 0u8;
        let mut multiplayerId: u8 = 0u8;
        if (!((((&raw mut gReceivedRemoteLinkPlayers).cast::<u8>()).read()) != 0))
            || (((((&raw mut gWirelessCommType).cast::<u8>()).read()) as i32) == 0i32)
        {
            {
                SetMainCallback2(exitCallback);
                (((&raw mut gRfu).cast::<u8>())
                    .wrapping_add(16)
                    .cast::<u16>())
                .write(0u16);
                (((&raw mut gRfu).cast::<u8>())
                    .wrapping_add(18)
                    .cast::<u16>())
                .write(0u16);
                crate::c::volatile_write(((&raw mut gRfu).cast::<u8>()).wrapping_add(238), 1u8);
            }
            return;
        }
        playerCount = GetLinkPlayerCount();
        multiplayerId = GetMultiplayerId();
        if (((playerCount) as i32) < 2i32) || (((multiplayerId) as i32) >= ((playerCount) as i32)) {
            {
                SetMainCallback2(exitCallback);
                (((&raw mut gRfu).cast::<u8>())
                    .wrapping_add(16)
                    .cast::<u16>())
                .write(0u16);
                (((&raw mut gRfu).cast::<u8>())
                    .wrapping_add(18)
                    .cast::<u16>())
                .write(0u16);
                crate::c::volatile_write(((&raw mut gRfu).cast::<u8>()).wrapping_add(238), 1u8);
            }
            return;
        }
        ((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).write(AllocZeroed(16832u32));
        if !(!(((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).is_null()) {
            {
                SetMainCallback2(exitCallback);
                (((&raw mut gRfu).cast::<u8>())
                    .wrapping_add(16)
                    .cast::<u16>())
                .write(0u16);
                (((&raw mut gRfu).cast::<u8>())
                    .wrapping_add(18)
                    .cast::<u16>())
                .write(0u16);
                crate::c::volatile_write(((&raw mut gRfu).cast::<u8>()).wrapping_add(238), 1u8);
            }
            return;
        }
        ((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
            .cast::<Option<unsafe extern "C" fn()>>())
        .write(exitCallback);
        ((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(8))
            .write(multiplayerId);
        ((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(9))
            .write(playerCount);
        SetNamesAndTextSpeed(((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read());
        ((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(18)
            .cast::<u16>())
        .write(1u16);
        ((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(14)).write(1u8);
        ((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(15)).write(6u8);
        SetPaletteFadeArgs(
            ((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(54))
                .cast::<u8>(),
            1u8,
            4294967295u32,
            0i8,
            16u8,
            0u8,
            0u16,
        );
        RunOrScheduleCommand(
            4u16,
            1u8,
            ((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(54))
                .cast::<u8>(),
        );
        SetMainCallback2(Some(MainCB));
        ((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(10))
            .write(CreateTask(Some(MainTask), 8u8));
        crate::c::bf_write(
            ((&raw mut gTextFlags).cast::<u8>()).wrapping_add(0),
            2,
            1,
            (0u8) as i32,
        );
    }
}
pub(crate) unsafe extern "C" fn GetBerryFromBag() {
    unsafe {
        if (((((&raw mut gSpecialVar_ItemId).cast::<u16>()).read()) as i32) < 133i32)
            || (((((&raw mut gSpecialVar_ItemId).cast::<u16>()).read()) as i32) > 176i32)
        {
            ((&raw mut gSpecialVar_ItemId).cast::<u16>()).write(133u16);
        } else {
            RemoveBagItem(((&raw mut gSpecialVar_ItemId).cast::<u16>()).read(), 1u16);
        }
        (((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(152))
            .cast::<u8>())
        .wrapping_offset(
            ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(8)).read())
                as i32) as isize
                * 32,
        ))
        .wrapping_add(12)
        .cast::<u16>())
        .write(
            ((((((&raw mut gSpecialVar_ItemId).cast::<u16>()).read()) as i32).wrapping_sub(133i32))
                as u16),
        );
        ((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(14)).write(1u8);
        ((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(15)).write(9u8);
        SetPaletteFadeArgs(
            ((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(54))
                .cast::<u8>(),
            0u8,
            4294967295u32,
            0i8,
            16u8,
            0u8,
            0u16,
        );
        RunOrScheduleCommand(
            4u16,
            1u8,
            ((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(54))
                .cast::<u8>(),
        );
        ((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(10))
            .write(CreateTask(Some(MainTask), 8u8));
        SetMainCallback2(Some(MainCB));
    }
}
pub(crate) unsafe extern "C" fn ChooseBerry() {
    unsafe {
        DestroyTask(
            ((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(10)).read(),
        );
        ChooseBerryForMachine(Some(GetBerryFromBag));
    }
}
pub(crate) unsafe extern "C" fn BerryCrush_SetVBlankCB() {
    unsafe {
        SetVBlankCallback(Some(VBlankCB));
    }
}
pub(crate) unsafe extern "C" fn BerryCrush_InitVBlankCB() {
    unsafe {
        SetVBlankCallback(None);
    }
}
pub(crate) unsafe extern "C" fn SaveResults() {
    unsafe {
        let mut time: u32 = 0u32;
        let mut presses: u32 = 0u32;
        time = (((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(104))
            .wrapping_add(4)
            .cast::<u16>())
        .read()) as u32);
        time = (((time << 8) as i32) as u32);
        time = ((MathUtil_Div32(((time) as i32), 15360i32)) as u32);
        presses = (((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(104))
        .wrapping_add(10)
        .cast::<u16>())
        .read()) as u32);
        presses = (((presses << 8) as i32) as u32);
        presses = ((MathUtil_Div32(((presses) as i32), ((time) as i32)) & 65535i32) as u32);
        ((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(22)
            .cast::<u16>())
        .write(((presses) as u16));
        'l1: {
            let __sw1 = ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(9))
            .read()) as i32);
            if __sw1 == 2i32 {
                if ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(22)
                    .cast::<u16>())
                .read()) as i32)
                    > (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(492))
                    .cast::<u16>())
                    .read()) as i32)
                {
                    crate::c::bf_write(
                        (((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(37),
                        1,
                        1,
                        (1u8) as i32,
                    );
                    (((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(492))
                        .cast::<u16>())
                    .write(
                        ((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(22)
                            .cast::<u16>())
                        .read(),
                    );
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                if ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(22)
                    .cast::<u16>())
                .read()) as i32)
                    > ((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(492))
                    .cast::<u16>())
                    .wrapping_offset(1))
                    .read()) as i32)
                {
                    crate::c::bf_write(
                        (((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(37),
                        1,
                        1,
                        (1u8) as i32,
                    );
                    ((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(492))
                        .cast::<u16>())
                    .wrapping_offset(1))
                    .write(
                        ((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(22)
                            .cast::<u16>())
                        .read(),
                    );
                }
                break 'l1;
            }
            if __sw1 == 4i32 {
                if ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(22)
                    .cast::<u16>())
                .read()) as i32)
                    > ((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(492))
                    .cast::<u16>())
                    .wrapping_offset(2))
                    .read()) as i32)
                {
                    crate::c::bf_write(
                        (((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(37),
                        1,
                        1,
                        (1u8) as i32,
                    );
                    ((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(492))
                        .cast::<u16>())
                    .wrapping_offset(2))
                    .write(
                        ((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(22)
                            .cast::<u16>())
                        .read(),
                    );
                }
                break 'l1;
            }
            if __sw1 == 5i32 {
                if ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(22)
                    .cast::<u16>())
                .read()) as i32)
                    > ((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(492))
                    .cast::<u16>())
                    .wrapping_offset(3))
                    .read()) as i32)
                {
                    crate::c::bf_write(
                        (((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(37),
                        1,
                        1,
                        (1u8) as i32,
                    );
                    ((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(492))
                        .cast::<u16>())
                    .wrapping_offset(3))
                    .write(
                        ((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(22)
                            .cast::<u16>())
                        .read(),
                    );
                }
                break 'l1;
            }
        }
        ((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(28)
            .cast::<i32>())
        .write(
            (((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(104))
                .cast::<u32>())
            .read()) as i32),
        );
        if (GiveBerryPowder(
            ((((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(28)
                .cast::<i32>())
            .read()) as u32),
        )) != 0
        {
            return;
        }
        crate::c::bf_write(
            (((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(37),
            0,
            1,
            (1u8) as i32,
        );
    }
}
pub(crate) unsafe extern "C" fn VBlankCB() {
    unsafe {
        TransferPlttBuffer();
        LoadOam();
        ProcessSpriteCopyRequests();
    }
}
pub(crate) unsafe extern "C" fn MainCB() {
    unsafe {
        RunTasks();
        RunTextPrinters();
        AnimateSprites();
        BuildOamBuffer();
    }
}
pub(crate) unsafe extern "C" fn MainTask(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if (((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(4)
            .cast::<Option<unsafe extern "C" fn(*mut u8, *mut u8) -> u32>>())
        .read())
        .is_some()
        {
            (((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(4)
                .cast::<Option<unsafe extern "C" fn(*mut u8, *mut u8) -> u32>>())
            .read())
            .unwrap_unchecked()(
                ((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read(),
                ((((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(54))
                    .cast::<u8>(),
            );
        }
        UpdateGame(((&raw mut sGame).cast::<u8>().cast::<*mut u8>()).read());
    }
}
pub(crate) unsafe extern "C" fn SetNamesAndTextSpeed(game: *mut u8) {
    unsafe {
        let mut game = game;
        let mut i: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < ((((game).wrapping_add(9)).read()) as i32)) {
                    break 'l1;
                }
                'l2: {
                    StringCopy(
                        ((((game).wrapping_add(152)).cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 32))
                        .cast::<u8>(),
                        ((((&raw mut gLinkPlayers).cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 28))
                        .wrapping_add(8))
                        .cast::<u8>(),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        {
            'l3: loop {
                if !(((i) as i32) < 5i32) {
                    break 'l3;
                }
                'l4: {
                    crate::c::memset(
                        ((((game).wrapping_add(152)).cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 32))
                        .cast::<u8>(),
                        1i32,
                        7u32,
                    );
                    ((((((game).wrapping_add(152)).cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 32))
                    .cast::<u8>())
                    .wrapping_offset(7))
                    .write(255u8);
                }
                i = (i).wrapping_add(1);
            }
        }
        'l5: {
            let __sw1 = ((crate::c::bf_read(
                (((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(20),
                0,
                3,
                false,
            ) as u16) as i32);
            if __sw1 == 0i32 {
                ((game).wrapping_add(11)).write(8u8);
                break 'l5;
            }
            if __sw1 == 1i32 {
                ((game).wrapping_add(11)).write(4u8);
                break 'l5;
            }
            if __sw1 == 2i32 {
                ((game).wrapping_add(11)).write(1u8);
                break 'l5;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn ShowGameDisplay() -> i32 {
    unsafe {
        let mut game: *mut u8 = GetBerryCrushGame();
        if !(!(game).is_null()) {
            return (-1i32);
        }
        'l1: {
            let __sw1 = ((((game).wrapping_add(12)).read()) as i32);
            if __sw1 == 0i32 {
                SetVBlankCallback(None);
                SetHBlankCallback(None);
                SetGpuReg(0u8, 0u16);
                ScanlineEffect_Stop();
                ResetTempTileDataBuffers();
                break 'l1;
            }
            if __sw1 == 1i32 {
                'l2: loop {
                    'l3: {
                        {
                            let mut tmp: u16 = 0u16;
                            (&raw mut tmp).write_volatile(0u16);
                            'l4: loop {
                                'l5: {
                                    CpuSet(
                                        (&raw mut tmp).cast::<u8>(),
                                        ((117440512i32) as usize as *mut u8),
                                        ((16777216i32
                                            | (crate::c::div_i32(
                                                1024i32,
                                                crate::c::div_i32(16i32, 8i32),
                                            ) & 2097151i32))
                                            as u32),
                                    );
                                }
                                if !((0i32) != 0) {
                                    break 'l4;
                                }
                            }
                        }
                    }
                    if !((0i32) != 0) {
                        break 'l2;
                    }
                }
                ((&raw mut gReservedSpritePaletteCount).cast::<u8>()).write(0u8);
                DigitObjUtil_Init(3u32);
                break 'l1;
            }
            if __sw1 == 2i32 {
                ResetPaletteFade();
                ResetSpriteData();
                FreeAllSpritePalettes();
                break 'l1;
            }
            if __sw1 == 3i32 {
                ResetBgsAndClearDma3BusyFlags(0u32);
                InitBgsFromTemplates(
                    0u8,
                    ((&raw const sBgTemplates).cast::<u8>().cast_mut()).cast::<u8>(),
                    ((crate::c::div_u32(16u32, 4u32)) as u8),
                );
                SetBgTilemapBuffer(
                    1u8,
                    (((((game).wrapping_add(312)).wrapping_add(136)).cast::<u8>()).cast::<u16>())
                        .cast::<u8>(),
                );
                SetBgTilemapBuffer(
                    2u8,
                    ((((((game).wrapping_add(312)).wrapping_add(136)).cast::<u8>())
                        .wrapping_offset(8192))
                    .cast::<u16>())
                    .cast::<u8>(),
                );
                SetBgTilemapBuffer(
                    3u8,
                    ((((((game).wrapping_add(312)).wrapping_add(136)).cast::<u8>())
                        .wrapping_offset(12288))
                    .cast::<u16>())
                    .cast::<u8>(),
                );
                ChangeBgX(0u8, 0i32, 0u8);
                ChangeBgY(0u8, 0i32, 0u8);
                ChangeBgX(2u8, 0i32, 0u8);
                ChangeBgY(2u8, 0i32, 0u8);
                ChangeBgX(3u8, 0i32, 0u8);
                ChangeBgY(3u8, 0i32, 0u8);
                SetGpuReg(80u8, 0u16);
                SetGpuReg(82u8, 0u16);
                break 'l1;
            }
            if __sw1 == 4i32 {
                FillBgTilemapBufferRect_Palette0(0u8, 0u16, 0u8, 0u8, 32u8, 32u8);
                FillBgTilemapBufferRect_Palette0(1u8, 0u16, 0u8, 0u8, 32u8, 64u8);
                FillBgTilemapBufferRect_Palette0(2u8, 0u16, 0u8, 0u8, 32u8, 32u8);
                FillBgTilemapBufferRect_Palette0(3u8, 0u16, 0u8, 0u8, 32u8, 32u8);
                break 'l1;
            }
            if __sw1 == 5i32 {
                CopyBgTilemapBufferToVram(0u8);
                CopyBgTilemapBufferToVram(1u8);
                CopyBgTilemapBufferToVram(2u8);
                CopyBgTilemapBufferToVram(3u8);
                DecompressAndCopyTileDataToVram(
                    1u8,
                    (((&raw mut gBerryCrush_Crusher_Gfx).cast::<u32>()).cast::<u32>()).cast::<u8>(),
                    0u32,
                    0u16,
                    0u8,
                );
                break 'l1;
            }
            if __sw1 == 6i32 {
                if (FreeTempTileDataBuffersIfPossible()) != 0 {
                    return 0i32;
                }
                InitStandardTextBoxWindows();
                InitTextBoxGfxAndPrinters();
                CreatePlayerNameWindows(game);
                DrawPlayerNameWindows(game);
                crate::c::bf_write(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(8),
                    7,
                    1,
                    (1u16) as i32,
                );
                break 'l1;
            }
            if __sw1 == 7i32 {
                LoadPalette(
                    (((&raw mut gBerryCrush_Crusher_Pal).cast::<u16>()).cast::<u16>()).cast::<u8>(),
                    0u16,
                    384u16,
                );
                CopyToBgTilemapBuffer(
                    1u8,
                    ((&raw const sCrusherTop_Tilemap).cast::<u8>().cast_mut()).cast::<u8>(),
                    0u16,
                    0u16,
                );
                CopyToBgTilemapBuffer(
                    2u8,
                    ((&raw const sContainerCap_Tilemap).cast::<u8>().cast_mut()).cast::<u8>(),
                    0u16,
                    0u16,
                );
                CopyToBgTilemapBuffer(
                    3u8,
                    ((&raw const sBg_Tilemap).cast::<u8>().cast_mut()).cast::<u8>(),
                    0u16,
                    0u16,
                );
                CopyPlayerNameWindowGfxToBg(game);
                CopyBgTilemapBufferToVram(1u8);
                CopyBgTilemapBufferToVram(2u8);
                CopyBgTilemapBufferToVram(3u8);
                break 'l1;
            }
            if __sw1 == 8i32 {
                LoadWirelessStatusIndicatorSpriteGfx();
                CreateWirelessStatusIndicatorSprite(0u8, 0u8);
                CreateGameSprites(game);
                SetGpuReg(
                    22u8,
                    ((((((&raw mut gSpriteCoordOffsetY).cast::<i16>()).read()) as i32)
                        .wrapping_neg()) as u16),
                );
                ChangeBgX(1u8, 0i32, 0u8);
                ChangeBgY(1u8, 0i32, 0u8);
                break 'l1;
            }
            if __sw1 == 9i32 {
                crate::c::bf_write(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(8),
                    7,
                    1,
                    (0u16) as i32,
                );
                BlendPalettes(4294967295u32, 16u8, 0u16);
                ShowBg(0u8);
                ShowBg(1u8);
                ShowBg(2u8);
                ShowBg(3u8);
                SetGpuRegBits(0u8, 4160u16);
                BerryCrush_SetVBlankCB();
                ((game).wrapping_add(12)).write(0u8);
                return 1i32;
            }
        }
        let __p2 = (game).wrapping_add(12);
        (__p2).write(((__p2).read()).wrapping_add(1));
        return 0i32;
    }
}
pub(crate) unsafe extern "C" fn HideGameDisplay() -> i32 {
    unsafe {
        let mut game: *mut u8 = GetBerryCrushGame();
        if !(!(game).is_null()) {
            return (-1i32);
        }
        'l1: {
            let __sw1 = ((((game).wrapping_add(12)).read()) as i32);
            let mut __fall = false;
            if __sw1 == 0i32 {
                __fall = true;
                Rfu_SetLinkStandbyCallback();
                break 'l1;
            }
            if __sw1 == 1i32 {
                __fall = true;
                if !((IsLinkTaskFinished()) != 0) {
                    return 0i32;
                }
            }
            if __fall || __sw1 == 2i32 {
                __fall = true;
                BeginNormalPaletteFade(4294967295u32, 0i8, 0u8, 16u8, 0u16);
                UpdatePaletteFade();
                break 'l1;
            }
            if __sw1 == 3i32 {
                __fall = true;
                if (UpdatePaletteFade()) != 0 {
                    return 0i32;
                }
                break 'l1;
            }
            if __sw1 == 4i32 {
                __fall = true;
                FillBgTilemapBufferRect_Palette0(0u8, 0u16, 0u8, 0u8, 32u8, 32u8);
                FillBgTilemapBufferRect_Palette0(1u8, 0u16, 0u8, 0u8, 32u8, 32u8);
                FillBgTilemapBufferRect_Palette0(2u8, 0u16, 0u8, 0u8, 32u8, 32u8);
                FillBgTilemapBufferRect_Palette0(3u8, 0u16, 0u8, 0u8, 32u8, 32u8);
                CopyBgTilemapBufferToVram(0u8);
                CopyBgTilemapBufferToVram(1u8);
                CopyBgTilemapBufferToVram(2u8);
                CopyBgTilemapBufferToVram(3u8);
                break 'l1;
            }
            if __sw1 == 5i32 {
                __fall = true;
                FreeAllWindowBuffers();
                HideBg(0u8);
                UnsetBgTilemapBuffer(0u8);
                HideBg(1u8);
                UnsetBgTilemapBuffer(1u8);
                HideBg(2u8);
                UnsetBgTilemapBuffer(2u8);
                HideBg(3u8);
                UnsetBgTilemapBuffer(3u8);
                ClearGpuRegBits(0u8, 4160u16);
                break 'l1;
            }
            if __sw1 == 6i32 {
                __fall = true;
                DestroyWirelessStatusIndicatorSprite();
                DestroyGameSprites(game);
                DigitObjUtil_Free();
                break 'l1;
            }
            if __sw1 == 7i32 {
                __fall = true;
                ((game).wrapping_add(12)).write(0u8);
                return 1i32;
            }
        }
        let __p2 = (game).wrapping_add(12);
        (__p2).write(((__p2).read()).wrapping_add(1));
        return 0i32;
    }
}
pub(crate) unsafe extern "C" fn UpdateGame(game: *mut u8) -> i32 {
    unsafe {
        let mut game = game;
        ((&raw mut gSpriteCoordOffsetY).cast::<i16>()).write(
            ((((((game).wrapping_add(42).cast::<i16>()).read()) as i32)
                .wrapping_add(((((game).wrapping_add(44).cast::<i16>()).read()) as i32)))
                as i16),
        );
        SetGpuReg(
            22u8,
            ((((((&raw mut gSpriteCoordOffsetY).cast::<i16>()).read()) as i32).wrapping_neg())
                as u16),
        );
        if ((((game).wrapping_add(18).cast::<u16>()).read()) as i32) == 7i32 {
            PrintTimer(
                (game).wrapping_add(312),
                ((game).wrapping_add(40).cast::<u16>()).read(),
            );
        }
        return 0i32;
    }
}
pub(crate) unsafe extern "C" fn ResetCrusherPos(game: *mut u8) {
    unsafe {
        let mut game = game;
        ((game).wrapping_add(42).cast::<i16>()).write((-104i16));
        ((game).wrapping_add(44).cast::<i16>()).write(0i16);
        ((&raw mut gSpriteCoordOffsetX).cast::<i16>()).write(0i16);
        ((&raw mut gSpriteCoordOffsetY).cast::<i16>()).write((-104i16));
    }
}
pub(crate) unsafe extern "C" fn CreateBerrySprites(game: *mut u8, gfx: *mut u8) {
    unsafe {
        let mut game = game;
        let mut gfx = gfx;
        let mut i: u8 = 0u8;
        let mut spriteId: u8 = 0u8;
        let mut distance: i16 = 0i16;
        let mut var1: i16 = 0i16;
        let mut data: *mut i16 = core::ptr::null_mut();
        let mut speed: i16 = 0i16;
        let mut var2: u32 = 0u32;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < ((((game).wrapping_add(9)).read()) as i32)) {
                    break 'l1;
                }
                'l2: {
                    spriteId = AddCustomItemIconSprite(
                        (&raw const sSpriteTemplate_PlayerBerry)
                            .cast::<u8>()
                            .cast_mut(),
                        ((((&raw const sPlayerBerrySpriteTags)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<u16>())
                        .cast::<u16>())
                        .wrapping_offset(((i) as i32) as isize))
                        .read(),
                        ((((&raw const sPlayerBerrySpriteTags)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<u16>())
                        .cast::<u16>())
                        .wrapping_offset(((i) as i32) as isize))
                        .read(),
                        (((((((((game).wrapping_add(152)).cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 32))
                        .wrapping_add(12)
                        .cast::<u16>())
                        .read()) as i32)
                            .wrapping_add(133i32)) as u16),
                    );
                    ((((gfx).wrapping_add(56)).cast::<*mut u8>())
                        .wrapping_offset(((i) as i32) as isize))
                    .write(
                        ((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((spriteId) as i32) as isize * 68),
                    );
                    crate::c::bf_write(
                        (((((gfx).wrapping_add(56)).cast::<*mut u8>())
                            .wrapping_offset(((i) as i32) as isize))
                        .read())
                        .wrapping_add(5),
                        2,
                        2,
                        (3u16) as i32,
                    );
                    crate::c::bf_write(
                        (((((gfx).wrapping_add(56)).cast::<*mut u8>())
                            .wrapping_offset(((i) as i32) as isize))
                        .read())
                        .wrapping_add(44),
                        7,
                        1,
                        (1u8) as i32,
                    );
                    ((((((gfx).wrapping_add(56)).cast::<*mut u8>())
                        .wrapping_offset(((i) as i32) as isize))
                    .read())
                    .wrapping_add(32)
                    .cast::<i16>())
                    .write(
                        ((((((((((gfx).wrapping_add(12)).cast::<*mut u8>())
                            .wrapping_offset(((i) as i32) as isize))
                        .read())
                        .wrapping_add(8)
                        .cast::<i16>())
                        .read()) as i32)
                            .wrapping_add(120i32)) as i16),
                    );
                    ((((((gfx).wrapping_add(56)).cast::<*mut u8>())
                        .wrapping_offset(((i) as i32) as isize))
                    .read())
                    .wrapping_add(34)
                    .cast::<i16>())
                    .write((-16i16));
                    data = ((((((gfx).wrapping_add(56)).cast::<*mut u8>())
                        .wrapping_offset(((i) as i32) as isize))
                    .read())
                    .wrapping_add(46))
                    .cast::<i16>();
                    speed = 512i16;
                    ((data).wrapping_offset(1)).write(speed);
                    ((data).wrapping_offset(2)).write(32i16);
                    ((data).wrapping_offset(7)).write(112i16);
                    distance = ((((((((((gfx).wrapping_add(12)).cast::<*mut u8>())
                        .wrapping_offset(((i) as i32) as isize))
                    .read())
                    .wrapping_add(10)
                    .cast::<i16>())
                    .read()) as i32)
                        .wrapping_sub(
                            ((((((((gfx).wrapping_add(12)).cast::<*mut u8>())
                                .wrapping_offset(((i) as i32) as isize))
                            .read())
                            .wrapping_add(8)
                            .cast::<i16>())
                            .read()) as i32),
                        )) as i16);
                    ((data).wrapping_offset(6))
                        .write(((crate::c::div_i32(((distance) as i32), 4i32)) as i16));
                    distance = ((((distance) as i32).wrapping_mul(128i32)) as i16);
                    var2 = ((((speed) as i32).wrapping_add(32i32)) as u32);
                    var2 = crate::c::div_u32(var2, 2u32);
                    var1 = MathUtil_Div16Shift(
                        7u8,
                        ((((63.5f32) as f32) * ((256i32) as f32)) as i16),
                        ((var2) as i16),
                    );
                    (data).write(
                        (((((((((((gfx).wrapping_add(56)).cast::<*mut u8>())
                            .wrapping_offset(((i) as i32) as isize))
                        .read())
                        .wrapping_add(32)
                        .cast::<i16>())
                        .read()) as u16) as i32)
                            .wrapping_mul(128i32)) as i16),
                    );
                    ((data).wrapping_offset(3)).write(MathUtil_Div16Shift(7u8, distance, var1));
                    var1 = MathUtil_Mul16Shift(7u8, var1, 85i16);
                    ((data).wrapping_offset(4)).write(0i16);
                    ((data).wrapping_offset(5)).write(MathUtil_Div16Shift(
                        7u8,
                        ((((63.5f32) as f32) * ((256i32) as f32)) as i16),
                        var1,
                    ));
                    let __p1 = (data).wrapping_offset(7);
                    (__p1).write((((((__p1).read()) as i32) | 32768i32) as i16));
                    if ((((((((gfx).wrapping_add(12)).cast::<*mut u8>())
                        .wrapping_offset(((i) as i32) as isize))
                    .read())
                    .wrapping_add(8)
                    .cast::<i16>())
                    .read()) as i32)
                        < 0i32
                    {
                        StartSpriteAffineAnim(
                            ((((gfx).wrapping_add(56)).cast::<*mut u8>())
                                .wrapping_offset(((i) as i32) as isize))
                            .read(),
                            1u8,
                        );
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_DropBerryIntoCrusher(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut data: *mut i16 = ((sprite).wrapping_add(46)).cast::<i16>();
        let __p1 = (data).wrapping_offset(1);
        (__p1).write(
            (((((__p1).read()) as i32).wrapping_add(((((data).wrapping_offset(2)).read()) as i32)))
                as i16),
        );
        let __p2 = (sprite).wrapping_add(38).cast::<i16>();
        (__p2).write(
            (((((__p2).read()) as i32)
                .wrapping_add((((((data).wrapping_offset(1)).read()) as i32) >> 8)))
                as i16),
        );
        if (((((data).wrapping_offset(7)).read()) as i32) & 32768i32) != 0 {
            let __p3 = ((sprite).wrapping_add(46)).cast::<i16>();
            (__p3).write(
                (((((__p3).read()) as i32)
                    .wrapping_add(((((data).wrapping_offset(3)).read()) as i32)))
                    as i16),
            );
            let __p4 = (data).wrapping_offset(4);
            (__p4).write(
                (((((__p4).read()) as i32)
                    .wrapping_add(((((data).wrapping_offset(5)).read()) as i32)))
                    as i16),
            );
            ((sprite).wrapping_add(36).cast::<i16>()).write(Sin(
                ((((((data).wrapping_offset(4)).read()) as i32) >> 7) as i16),
                ((data).wrapping_offset(6)).read(),
            ));
            if ((((((data).wrapping_offset(7)).read()) as i32) & 32768i32) != 0)
                && ((((((data).wrapping_offset(4)).read()) as i32) >> 7) > 126i32)
            {
                ((sprite).wrapping_add(36).cast::<i16>()).write(0i16);
                let __p5 = (data).wrapping_offset(7);
                (__p5).write((((((__p5).read()) as i32) & 32767i32) as i16));
            }
        }
        ((sprite).wrapping_add(32).cast::<i16>()).write((((((data).read()) as i32) >> 7) as i16));
        if ((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32)
            .wrapping_add(((((sprite).wrapping_add(38).cast::<i16>()).read()) as i32))
            >= (((((data).wrapping_offset(7)).read()) as i32) & 32767i32)
        {
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(SpriteCallbackDummy));
            FreeSpriteOamMatrix(sprite);
            DestroySprite(sprite);
        }
    }
}
pub(crate) unsafe extern "C" fn BerryCrushFreeBerrySpriteGfx(game: *mut u8, gfx: *mut u8) {
    unsafe {
        let mut game = game;
        let mut gfx = gfx;
        let mut i: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < ((((game).wrapping_add(9)).read()) as i32)) {
                    break 'l1;
                }
                'l2: {
                    FreeSpritePaletteByTag(
                        ((((&raw const sPlayerBerrySpriteTags)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<u16>())
                        .cast::<u16>())
                        .wrapping_offset(((i) as i32) as isize))
                        .read(),
                    );
                    FreeSpriteTilesByTag(
                        ((((&raw const sPlayerBerrySpriteTags)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<u16>())
                        .cast::<u16>())
                        .wrapping_offset(((i) as i32) as isize))
                        .read(),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn UpdateInputEffects(game: *mut u8, gfx: *mut u8) {
    unsafe {
        let mut game = game;
        let mut gfx = gfx;
        let mut numPlayersPressed: u8 = 0u8;
        let mut linkState: *mut u8 = core::ptr::null_mut();
        let mut i: u8 = 0u8;
        let mut temp1: u16 = 0u16;
        let mut xModifier: u16 = 0u16;
        numPlayersPressed = 0u8;
        linkState = (((game).wrapping_add(78)).cast::<u16>()).cast::<u8>();
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < ((((game).wrapping_add(9)).read()) as i32)) {
                    break 'l1;
                }
                'l2: {
                    temp1 = ((crate::c::shr_i32(
                        ((((linkState).wrapping_add(10).cast::<u16>()).read()) as i32),
                        ((((i) as i32).wrapping_mul(3i32)) as u32),
                    )) as u16);
                    temp1 = ((((temp1) as i32) & 7i32) as u16);
                    if (temp1) != 0 {
                        numPlayersPressed = (numPlayersPressed).wrapping_add(1);
                        if (((temp1) as i32) & 4i32) != 0 {
                            StartSpriteAnim(
                                ((((gfx).wrapping_add(36)).cast::<*mut u8>())
                                    .wrapping_offset(((i) as i32) as isize))
                                .read(),
                                1u8,
                            );
                        } else {
                            StartSpriteAnim(
                                ((((gfx).wrapping_add(36)).cast::<*mut u8>())
                                    .wrapping_offset(((i) as i32) as isize))
                                .read(),
                                0u8,
                            );
                        }
                        crate::c::bf_write(
                            (((((gfx).wrapping_add(36)).cast::<*mut u8>())
                                .wrapping_offset(((i) as i32) as isize))
                            .read())
                            .wrapping_add(62),
                            2,
                            1,
                            (0u16) as i32,
                        );
                        crate::c::bf_write(
                            (((((gfx).wrapping_add(36)).cast::<*mut u8>())
                                .wrapping_offset(((i) as i32) as isize))
                            .read())
                            .wrapping_add(44),
                            6,
                            1,
                            (0u8) as i32,
                        );
                        ((((((gfx).wrapping_add(36)).cast::<*mut u8>())
                            .wrapping_offset(((i) as i32) as isize))
                        .read())
                        .wrapping_add(36)
                        .cast::<i16>())
                        .write(
                            (((((((&raw const sImpactCoords).cast::<u8>().cast_mut())
                                .cast::<u8>())
                            .wrapping_offset(
                                (((crate::c::rem_u32(
                                    ((temp1) as u32),
                                    (crate::c::div_u32(6u32, 2u32)).wrapping_add(1u32),
                                ))
                                .wrapping_sub(1u32)) as i32)
                                    as isize
                                    * 2,
                            ))
                            .cast::<i8>())
                            .read()) as i16),
                        );
                        ((((((gfx).wrapping_add(36)).cast::<*mut u8>())
                            .wrapping_offset(((i) as i32) as isize))
                        .read())
                        .wrapping_add(38)
                        .cast::<i16>())
                        .write(
                            ((((((((&raw const sImpactCoords).cast::<u8>().cast_mut())
                                .cast::<u8>())
                            .wrapping_offset(
                                (((crate::c::rem_u32(
                                    ((temp1) as u32),
                                    (crate::c::div_u32(6u32, 2u32)).wrapping_add(1u32),
                                ))
                                .wrapping_sub(1u32)) as i32)
                                    as isize
                                    * 2,
                            ))
                            .cast::<i8>())
                            .wrapping_offset(1))
                            .read()) as i16),
                        );
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        if ((numPlayersPressed) as i32) == 0i32 {
            crate::c::bf_write((game).wrapping_add(37), 2, 1, (0u8) as i32);
        } else {
            temp1 = (((crate::c::rem_i32(
                ((((game).wrapping_add(40).cast::<u16>()).read()) as i32),
                3i32,
            )) as u8) as u16);
            xModifier = temp1;
            {
                i = 0u8;
                'l3: loop {
                    if !(((i) as i32)
                        < (((((linkState).wrapping_add(12).cast::<u16>()).read()) as i32)
                            .wrapping_mul(2i32))
                        .wrapping_add(3i32))
                    {
                        break 'l3;
                    }
                    'l4: {
                        if (crate::c::bf_read(
                            (((((gfx).wrapping_add(76)).cast::<*mut u8>())
                                .wrapping_offset(((i) as i32) as isize))
                            .read())
                            .wrapping_add(62),
                            2,
                            1,
                            false,
                        ) as u16)
                            != 0
                        {
                            ((((((gfx).wrapping_add(76)).cast::<*mut u8>())
                                .wrapping_offset(((i) as i32) as isize))
                            .read())
                            .wrapping_add(28)
                            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                            .write(Some(SpriteCB_Sparkle_Init));
                            ((((((gfx).wrapping_add(76)).cast::<*mut u8>())
                                .wrapping_offset(((i) as i32) as isize))
                            .read())
                            .wrapping_add(32)
                            .cast::<i16>())
                            .write(
                                (((((((((&raw const sSparkleCoords).cast::<u8>().cast_mut())
                                    .cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize * 2))
                                .cast::<i8>())
                                .read()) as i32)
                                    .wrapping_add(120i32)) as i16),
                            );
                            ((((((gfx).wrapping_add(76)).cast::<*mut u8>())
                                .wrapping_offset(((i) as i32) as isize))
                            .read())
                            .wrapping_add(34)
                            .cast::<i16>())
                            .write(
                                (((((((((((&raw const sSparkleCoords).cast::<u8>().cast_mut())
                                    .cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize * 2))
                                .cast::<i8>())
                                .wrapping_offset(1))
                                .read()) as i32)
                                    .wrapping_add(136i32))
                                .wrapping_sub(((temp1) as i32).wrapping_mul(4i32)))
                                    as i16),
                            );
                            ((((((gfx).wrapping_add(76)).cast::<*mut u8>())
                                .wrapping_offset(((i) as i32) as isize))
                            .read())
                            .wrapping_add(36)
                            .cast::<i16>())
                            .write(
                                (((((((((&raw const sSparkleCoords).cast::<u8>().cast_mut())
                                    .cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize * 2))
                                .cast::<i8>())
                                .read()) as i32)
                                    .wrapping_add(crate::c::div_i32(
                                        (((((((&raw const sSparkleCoords)
                                            .cast::<u8>()
                                            .cast_mut())
                                        .cast::<u8>())
                                        .wrapping_offset(((i) as i32) as isize * 2))
                                        .cast::<i8>())
                                        .read()) as i32),
                                        ((xModifier) as i32).wrapping_mul(4i32),
                                    ))) as i16),
                            );
                            ((((((gfx).wrapping_add(76)).cast::<*mut u8>())
                                .wrapping_offset(((i) as i32) as isize))
                            .read())
                            .wrapping_add(38)
                            .cast::<i16>())
                            .write(
                                ((((((((&raw const sSparkleCoords).cast::<u8>().cast_mut())
                                    .cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize * 2))
                                .cast::<i8>())
                                .wrapping_offset(1))
                                .read()) as i16),
                            );
                            if (crate::c::bf_read((linkState).wrapping_add(4), 1, 1, false) as u8)
                                != 0
                            {
                                StartSpriteAnim(
                                    ((((gfx).wrapping_add(76)).cast::<*mut u8>())
                                        .wrapping_offset(((i) as i32) as isize))
                                    .read(),
                                    1u8,
                                );
                            } else {
                                StartSpriteAnim(
                                    ((((gfx).wrapping_add(76)).cast::<*mut u8>())
                                        .wrapping_offset(((i) as i32) as isize))
                                    .read(),
                                    0u8,
                                );
                            }
                            temp1 = (temp1).wrapping_add(1);
                            if ((temp1) as i32) > 3i32 {
                                temp1 = 0u16;
                            }
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
            if (crate::c::bf_read((game).wrapping_add(37), 2, 1, false) as u8) != 0 {
                crate::c::bf_write((game).wrapping_add(37), 2, 1, (0u8) as i32);
            } else {
                if ((numPlayersPressed) as i32) == 1i32 {
                    PlaySE(78u16);
                } else {
                    PlaySE(77u16);
                }
                crate::c::bf_write((game).wrapping_add(37), 2, 1, (1u8) as i32);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn AreEffectsFinished(game: *mut u8, gfx: *mut u8) -> u32 {
    unsafe {
        let mut game = game;
        let mut gfx = gfx;
        let mut i: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < ((((game).wrapping_add(9)).read()) as i32)) {
                    break 'l1;
                }
                'l2: {
                    if !((crate::c::bf_read(
                        (((((gfx).wrapping_add(36)).cast::<*mut u8>())
                            .wrapping_offset(((i) as i32) as isize))
                        .read())
                        .wrapping_add(62),
                        2,
                        1,
                        false,
                    ) as u16)
                        != 0)
                    {
                        return 0u32;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        {
            i = 0u8;
            'l3: loop {
                if !(((i) as u32) < crate::c::div_u32(44u32, 4u32)) {
                    break 'l3;
                }
                'l4: {
                    if !((crate::c::bf_read(
                        (((((gfx).wrapping_add(76)).cast::<*mut u8>())
                            .wrapping_offset(((i) as i32) as isize))
                        .read())
                        .wrapping_add(62),
                        2,
                        1,
                        false,
                    ) as u16)
                        != 0)
                    {
                        return 0u32;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        if ((((game).wrapping_add(44).cast::<i16>()).read()) as i32) != 0i32 {
            ((game).wrapping_add(44).cast::<i16>()).write(0i16);
        }
        return 1u32;
    }
}
pub(crate) unsafe extern "C" fn FramesToMinSec(gfx: *mut u8, frames: u16) {
    unsafe {
        let mut gfx = gfx;
        let mut frames = frames;
        let mut i: u8 = 0u8;
        let mut fractionalFrames: u32 = 0u32;
        let mut r3: i16 = 0i16;
        ((gfx).wrapping_add(4).cast::<i16>())
            .write(((crate::c::div_i32(((frames) as i32), 3600i32)) as i16));
        ((gfx).wrapping_add(6).cast::<i16>()).write(
            ((crate::c::div_i32(crate::c::rem_i32(((frames) as i32), 3600i32), 60i32)) as i16),
        );
        r3 = MathUtil_Mul16(
            (((crate::c::rem_i32(((frames) as i32), 60i32)).wrapping_mul(256i32)) as i16),
            4i16,
        );
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 8i32) {
                    break 'l1;
                }
                'l2: {
                    if (crate::c::shr_i32(
                        ((r3) as i32),
                        (((7i32).wrapping_sub(((i) as i32))) as u32),
                    ) & 1i32)
                        != 0
                    {
                        fractionalFrames = (fractionalFrames).wrapping_add(
                            ((((&raw const sPressingSpeedConversionTable)
                                .cast::<u8>()
                                .cast_mut()
                                .cast::<u32>())
                            .cast::<u32>())
                            .wrapping_offset(((i) as i32) as isize))
                            .read(),
                        );
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        ((gfx).wrapping_add(8).cast::<i16>())
            .write(((crate::c::div_u32(fractionalFrames, 1000000u32)) as i16));
    }
}
pub(crate) unsafe extern "C" fn PrintTextCentered(
    windowId: u8,
    left: u8,
    colorId: u8,
    string: *mut u8,
) {
    unsafe {
        let mut windowId = windowId;
        let mut left = left;
        let mut colorId = colorId;
        let mut string = string;
        left = ((((((left) as i32).wrapping_mul(4i32)) as u32).wrapping_sub(crate::c::div_u32(
            ((GetStringWidth(2u8, string, (-1i16))) as u32),
            2u32,
        ))) as u8);
        AddTextPrinterParameterized3(
            windowId,
            2u8,
            left,
            0u8,
            ((((&raw const sTextColorTable).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((colorId) as i32) as isize * 3))
            .cast::<u8>(),
            0i8,
            string,
        );
    }
}
pub(crate) unsafe extern "C" fn PrintResultsText(game: *mut u8, page: u8, sp14: u8, baseY: u8) {
    unsafe {
        let mut game = game;
        let mut page = page;
        let mut sp14 = sp14;
        let mut baseY = baseY;
        let mut i: u8 = 0u8;
        let mut j: u8 = 0u8;
        let mut playerId: u8 = 0u8;
        let mut ranking: u8 = 0u8;
        let mut x: i32 = 0i32;
        let mut stat: u8 = 0u8;
        let mut results: *mut u8 = (game).wrapping_add(104);
        let mut xOffset: u32 = 0u32;
        let mut y: i32 = 0i32;
        baseY = ((((baseY) as i32).wrapping_sub(16i32)) as u8);
        if ((page) as i32) == 2i32 {
            baseY = ((((baseY) as i32).wrapping_sub(42i32)) as u8);
        }
        y = ((baseY) as i32)
            .wrapping_sub((14i32).wrapping_mul(((((game).wrapping_add(9)).read()) as i32)));
        if y > 0i32 {
            y = (crate::c::div_i32(y, 2i32)).wrapping_add(16i32);
        } else {
            y = 16i32;
        }
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < ((((game).wrapping_add(9)).read()) as i32)) {
                    break 'l1;
                }
                'l2: {
                    DynamicPlaceholderTextUtil_Reset();
                    'l3: {
                        let __sw1 = ((page) as i32);
                        if __sw1 == 0i32 {
                            playerId = ((((((results).wrapping_add(32)).cast::<u8>())
                                .wrapping_offset(((page) as i32) as isize * 8))
                            .cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize))
                            .read();
                            if (((i) as i32) != 0i32)
                                && (((((((((results).wrapping_add(12)).cast::<u8>())
                                    .wrapping_offset(((page) as i32) as isize * 10))
                                .cast::<u16>())
                                .wrapping_offset(((i) as i32) as isize))
                                .read()) as i32)
                                    != ((((((((results).wrapping_add(12)).cast::<u8>())
                                        .wrapping_offset(((page) as i32) as isize * 10))
                                    .cast::<u16>())
                                    .wrapping_offset((((i) as i32).wrapping_sub(1i32)) as isize))
                                    .read()) as i32))
                            {
                                ranking = i;
                            }
                            ConvertIntToDecimalStringN(
                                (&raw mut gStringVar4).cast::<u8>(),
                                ((((((((results).wrapping_add(12)).cast::<u8>())
                                    .wrapping_offset(((page) as i32) as isize * 10))
                                .cast::<u16>())
                                .wrapping_offset(((i) as i32) as isize))
                                .read()) as i32),
                                1i32,
                                4u8,
                            );
                            StringAppend(
                                (&raw mut gStringVar4).cast::<u8>(),
                                ((((&raw const sResultsTexts)
                                    .cast::<u8>()
                                    .cast_mut()
                                    .cast::<*mut u8>())
                                .cast::<*mut u8>())
                                .wrapping_offset(((page) as i32) as isize))
                                .read(),
                            );
                            break 'l3;
                        }
                        if __sw1 == 1i32 {
                            playerId = ((((((results).wrapping_add(32)).cast::<u8>())
                                .wrapping_offset(((page) as i32) as isize * 8))
                            .cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize))
                            .read();
                            if (((i) as i32) != 0i32)
                                && (((((((((results).wrapping_add(12)).cast::<u8>())
                                    .wrapping_offset(((page) as i32) as isize * 10))
                                .cast::<u16>())
                                .wrapping_offset(((i) as i32) as isize))
                                .read()) as i32)
                                    != ((((((((results).wrapping_add(12)).cast::<u8>())
                                        .wrapping_offset(((page) as i32) as isize * 10))
                                    .cast::<u16>())
                                    .wrapping_offset((((i) as i32).wrapping_sub(1i32)) as isize))
                                    .read()) as i32))
                            {
                                ranking = i;
                            }
                            ConvertIntToDecimalStringN(
                                (&raw mut gStringVar1).cast::<u8>(),
                                (((((((((results).wrapping_add(12)).cast::<u8>())
                                    .wrapping_offset(((page) as i32) as isize * 10))
                                .cast::<u16>())
                                .wrapping_offset(((i) as i32) as isize))
                                .read()) as i32)
                                    >> 4),
                                1i32,
                                3u8,
                            );
                            xOffset = 0u32;
                            stat = ((((((((((results).wrapping_add(12)).cast::<u8>())
                                .wrapping_offset(((page) as i32) as isize * 10))
                            .cast::<u16>())
                            .wrapping_offset(((i) as i32) as isize))
                            .read()) as i32)
                                & 15i32) as u8);
                            {
                                j = 0u8;
                                'l4: loop {
                                    if !(((j) as i32) < 4i32) {
                                        break 'l4;
                                    }
                                    'l5: {
                                        if (crate::c::shr_i32(
                                            ((stat) as i32),
                                            (((3i32).wrapping_sub(((j) as i32))) as u32),
                                        ) & 1i32)
                                            != 0
                                        {
                                            xOffset = (xOffset).wrapping_add(
                                                ((((&raw const sPressingSpeedConversionTable)
                                                    .cast::<u8>()
                                                    .cast_mut()
                                                    .cast::<u32>())
                                                .cast::<u32>())
                                                .wrapping_offset(((j) as i32) as isize))
                                                .read(),
                                            );
                                        }
                                    }
                                    j = (j).wrapping_add(1);
                                }
                            }
                            stat = ((crate::c::div_u32(xOffset, 1000000u32)) as u8);
                            ConvertIntToDecimalStringN(
                                (&raw mut gStringVar2).cast::<u8>(),
                                ((stat) as i32),
                                2i32,
                                2u8,
                            );
                            StringExpandPlaceholders(
                                (&raw mut gStringVar4).cast::<u8>(),
                                ((((&raw const sResultsTexts)
                                    .cast::<u8>()
                                    .cast_mut()
                                    .cast::<*mut u8>())
                                .cast::<*mut u8>())
                                .wrapping_offset(((page) as i32) as isize))
                                .read(),
                            );
                            break 'l3;
                        }
                        if __sw1 == 2i32 {
                            playerId = i;
                            ranking = i;
                            j = (((((((game).wrapping_add(152)).cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize * 32))
                            .wrapping_add(12)
                            .cast::<u16>())
                            .read()) as u8);
                            if ((j) as i32) >= 44i32 {
                                j = 0u8;
                            }
                            StringCopy(
                                (&raw mut gStringVar1).cast::<u8>(),
                                (((&raw mut gBerries).cast::<u8>())
                                    .wrapping_offset(((j) as i32) as isize * 28))
                                .cast::<u8>(),
                            );
                            StringExpandPlaceholders(
                                (&raw mut gStringVar4).cast::<u8>(),
                                ((((&raw const sResultsTexts)
                                    .cast::<u8>()
                                    .cast_mut()
                                    .cast::<*mut u8>())
                                .cast::<*mut u8>())
                                .wrapping_offset(((page) as i32) as isize))
                                .read(),
                            );
                            break 'l3;
                        }
                    }
                    x = GetStringRightAlignXOffset(
                        2i32,
                        (&raw mut gStringVar4).cast::<u8>(),
                        ((sp14) as i32).wrapping_sub(4i32),
                    );
                    AddTextPrinterParameterized3(
                        (((game).wrapping_add(312)).wrapping_add(130)).read(),
                        2u8,
                        ((x) as u8),
                        ((y) as u8),
                        (((&raw const sTextColorTable).cast::<u8>().cast_mut()).cast::<u8>())
                            .cast::<u8>(),
                        0i8,
                        (&raw mut gStringVar4).cast::<u8>(),
                    );
                    if ((playerId) as i32) == ((((game).wrapping_add(8)).read()) as i32) {
                        StringCopy(
                            (&raw mut gStringVar3).cast::<u8>(),
                            (&raw mut gText_1DotBlueF700).cast::<u8>(),
                        );
                    } else {
                        StringCopy(
                            (&raw mut gStringVar3).cast::<u8>(),
                            (&raw mut gText_1DotF700).cast::<u8>(),
                        );
                    }
                    ((&raw mut gStringVar3).cast::<u8>())
                        .write(((((ranking) as i32).wrapping_add(162i32)) as u8));
                    DynamicPlaceholderTextUtil_SetPlaceholderPtr(
                        0u8,
                        ((((game).wrapping_add(152)).cast::<u8>())
                            .wrapping_offset(((playerId) as i32) as isize * 32))
                        .cast::<u8>(),
                    );
                    DynamicPlaceholderTextUtil_ExpandPlaceholders(
                        (&raw mut gStringVar4).cast::<u8>(),
                        (&raw mut gStringVar3).cast::<u8>(),
                    );
                    AddTextPrinterParameterized3(
                        (((game).wrapping_add(312)).wrapping_add(130)).read(),
                        2u8,
                        4u8,
                        ((y) as u8),
                        (((&raw const sTextColorTable).cast::<u8>().cast_mut()).cast::<u8>())
                            .cast::<u8>(),
                        0i8,
                        (&raw mut gStringVar4).cast::<u8>(),
                    );
                }
                y = (y).wrapping_add(14i32);
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn PrintCrushingResults(game: *mut u8) {
    unsafe {
        let mut game = game;
        let mut i: u8 = 0u8;
        let mut x: u8 = 0u8;
        let mut pressingSpeedFrac: u32 = 0u32;
        let mut results: *mut u8 = (game).wrapping_add(104);
        let mut y: u8 =
            ((((GetWindowAttribute((((game).wrapping_add(312)).wrapping_add(130)).read(), 4u8))
                .wrapping_mul(8u32))
            .wrapping_sub(42u32)) as u8);
        FramesToMinSec(
            (game).wrapping_add(312),
            ((results).wrapping_add(4).cast::<u16>()).read(),
        );
        AddTextPrinterParameterized3(
            (((game).wrapping_add(312)).wrapping_add(130)).read(),
            2u8,
            x,
            y,
            (((&raw const sTextColorTable).cast::<u8>().cast_mut()).cast::<u8>()).cast::<u8>(),
            0i8,
            (&raw mut gText_TimeColon).cast::<u8>(),
        );
        x = (((176i32).wrapping_sub(
            (((GetStringWidth(2u8, (&raw mut gText_SpaceSec).cast::<u8>(), (-1i16))) as u8) as i32),
        )) as u8);
        AddTextPrinterParameterized3(
            (((game).wrapping_add(312)).wrapping_add(130)).read(),
            2u8,
            x,
            y,
            (((&raw const sTextColorTable).cast::<u8>().cast_mut()).cast::<u8>()).cast::<u8>(),
            0i8,
            (&raw mut gText_SpaceSec).cast::<u8>(),
        );
        ConvertIntToDecimalStringN(
            (&raw mut gStringVar1).cast::<u8>(),
            (((((game).wrapping_add(312)).wrapping_add(6).cast::<i16>()).read()) as i32),
            2i32,
            2u8,
        );
        ConvertIntToDecimalStringN(
            (&raw mut gStringVar2).cast::<u8>(),
            (((((game).wrapping_add(312)).wrapping_add(8).cast::<i16>()).read()) as i32),
            2i32,
            2u8,
        );
        StringExpandPlaceholders(
            (&raw mut gStringVar4).cast::<u8>(),
            (&raw mut gText_XDotY2).cast::<u8>(),
        );
        x = ((((x) as i32).wrapping_sub(GetStringWidth(
            2u8,
            (&raw mut gStringVar4).cast::<u8>(),
            (-1i16),
        ))) as u8);
        AddTextPrinterParameterized3(
            (((game).wrapping_add(312)).wrapping_add(130)).read(),
            2u8,
            x,
            y,
            (((&raw const sTextColorTable).cast::<u8>().cast_mut()).cast::<u8>()).cast::<u8>(),
            0i8,
            (&raw mut gStringVar4).cast::<u8>(),
        );
        x = ((((x) as i32).wrapping_sub(GetStringWidth(
            2u8,
            (&raw mut gText_SpaceMin).cast::<u8>(),
            (-1i16),
        ))) as u8);
        AddTextPrinterParameterized3(
            (((game).wrapping_add(312)).wrapping_add(130)).read(),
            2u8,
            x,
            y,
            (((&raw const sTextColorTable).cast::<u8>().cast_mut()).cast::<u8>()).cast::<u8>(),
            0i8,
            (&raw mut gText_SpaceMin).cast::<u8>(),
        );
        ConvertIntToDecimalStringN(
            (&raw mut gStringVar1).cast::<u8>(),
            (((((game).wrapping_add(312)).wrapping_add(4).cast::<i16>()).read()) as i32),
            2i32,
            1u8,
        );
        StringExpandPlaceholders(
            (&raw mut gStringVar4).cast::<u8>(),
            (&raw mut gText_StrVar1).cast::<u8>(),
        );
        x = ((((x) as i32).wrapping_sub(GetStringWidth(
            2u8,
            (&raw mut gStringVar4).cast::<u8>(),
            (-1i16),
        ))) as u8);
        AddTextPrinterParameterized3(
            (((game).wrapping_add(312)).wrapping_add(130)).read(),
            2u8,
            x,
            y,
            (((&raw const sTextColorTable).cast::<u8>().cast_mut()).cast::<u8>()).cast::<u8>(),
            0i8,
            (&raw mut gStringVar4).cast::<u8>(),
        );
        y = ((((y) as i32).wrapping_add(14i32)) as u8);
        AddTextPrinterParameterized3(
            (((game).wrapping_add(312)).wrapping_add(130)).read(),
            2u8,
            0u8,
            y,
            (((&raw const sTextColorTable).cast::<u8>().cast_mut()).cast::<u8>()).cast::<u8>(),
            0i8,
            (&raw mut gText_PressingSpeed).cast::<u8>(),
        );
        x = (((176i32).wrapping_sub(
            (((GetStringWidth(2u8, (&raw mut gText_TimesPerSec).cast::<u8>(), (-1i16))) as u8)
                as i32),
        )) as u8);
        AddTextPrinterParameterized3(
            (((game).wrapping_add(312)).wrapping_add(130)).read(),
            2u8,
            x,
            y,
            (((&raw const sTextColorTable).cast::<u8>().cast_mut()).cast::<u8>()).cast::<u8>(),
            0i8,
            (&raw mut gText_TimesPerSec).cast::<u8>(),
        );
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 8i32) {
                    break 'l1;
                }
                'l2: {
                    if (crate::c::shr_i32(
                        (((((game).wrapping_add(22).cast::<u16>()).read()) as u8) as i32),
                        (((7i32).wrapping_sub(((i) as i32))) as u32),
                    ) & 1i32)
                        != 0
                    {
                        pressingSpeedFrac = (pressingSpeedFrac).wrapping_add(
                            ((((&raw const sPressingSpeedConversionTable)
                                .cast::<u8>()
                                .cast_mut()
                                .cast::<u32>())
                            .cast::<u32>())
                            .wrapping_offset(((i) as i32) as isize))
                            .read(),
                        );
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        ConvertIntToDecimalStringN(
            (&raw mut gStringVar1).cast::<u8>(),
            (((((game).wrapping_add(22).cast::<u16>()).read()) as i32) >> 8),
            1i32,
            3u8,
        );
        ConvertIntToDecimalStringN(
            (&raw mut gStringVar2).cast::<u8>(),
            ((crate::c::div_u32(pressingSpeedFrac, 1000000u32)) as i32),
            2i32,
            2u8,
        );
        StringExpandPlaceholders(
            (&raw mut gStringVar4).cast::<u8>(),
            (&raw mut gText_XDotY3).cast::<u8>(),
        );
        x = ((((x) as i32).wrapping_sub(GetStringWidth(
            2u8,
            (&raw mut gStringVar4).cast::<u8>(),
            (-1i16),
        ))) as u8);
        if (crate::c::bf_read((game).wrapping_add(37), 1, 1, false) as u8) != 0 {
            AddTextPrinterParameterized3(
                (((game).wrapping_add(312)).wrapping_add(130)).read(),
                2u8,
                x,
                y,
                ((((&raw const sTextColorTable).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset(15))
                .cast::<u8>(),
                0i8,
                (&raw mut gStringVar4).cast::<u8>(),
            );
        } else {
            AddTextPrinterParameterized3(
                (((game).wrapping_add(312)).wrapping_add(130)).read(),
                2u8,
                x,
                y,
                (((&raw const sTextColorTable).cast::<u8>().cast_mut()).cast::<u8>()).cast::<u8>(),
                0i8,
                (&raw mut gStringVar4).cast::<u8>(),
            );
        }
        y = ((((y) as i32).wrapping_add(14i32)) as u8);
        AddTextPrinterParameterized3(
            (((game).wrapping_add(312)).wrapping_add(130)).read(),
            2u8,
            0u8,
            y,
            (((&raw const sTextColorTable).cast::<u8>().cast_mut()).cast::<u8>()).cast::<u8>(),
            0i8,
            (&raw mut gText_Silkiness).cast::<u8>(),
        );
        ConvertIntToDecimalStringN(
            (&raw mut gStringVar1).cast::<u8>(),
            ((((results).wrapping_add(8).cast::<u16>()).read()) as i32),
            1i32,
            3u8,
        );
        StringExpandPlaceholders(
            (&raw mut gStringVar4).cast::<u8>(),
            (&raw mut gText_Var1Percent).cast::<u8>(),
        );
        x = (((176i32).wrapping_sub(
            (((GetStringWidth(2u8, (&raw mut gStringVar4).cast::<u8>(), (-1i16))) as u8) as i32),
        )) as u8);
        AddTextPrinterParameterized3(
            (((game).wrapping_add(312)).wrapping_add(130)).read(),
            2u8,
            x,
            y,
            (((&raw const sTextColorTable).cast::<u8>().cast_mut()).cast::<u8>()).cast::<u8>(),
            0i8,
            (&raw mut gStringVar4).cast::<u8>(),
        );
    }
}
pub(crate) unsafe extern "C" fn OpenResultsWindow(game: *mut u8, gfx: *mut u8) -> u32 {
    unsafe {
        let mut game = game;
        let mut gfx = gfx;
        let mut playerCountIdx: u8 = 0u8;
        let mut template = crate::ffi::Align4([0u8; 8]);
        'l1: {
            let __sw1 = ((((gfx).wrapping_add(128)).read()) as i32);
            if __sw1 == 0i32 {
                playerCountIdx =
                    ((((((game).wrapping_add(9)).read()) as i32).wrapping_sub(2i32)) as u8);
                HideTimer(gfx);
                crate::c::memcpy(
                    (&raw mut template).cast::<u8>(),
                    (((&raw const sWindowTemplates_Results)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>())
                    .wrapping_offset(
                        (((((game).wrapping_add(18).cast::<u16>()).read()) as i32)
                            .wrapping_sub(11i32)) as isize
                            * 8,
                    ),
                    8u32,
                );
                if ((((game).wrapping_add(18).cast::<u16>()).read()) as i32) == 13i32 {
                    (((&raw mut template).cast::<u8>()).wrapping_add(4)).write(
                        ((((((&raw const sResultsWindowHeights).cast::<u8>().cast_mut())
                            .cast::<u8>())
                        .wrapping_offset(4))
                        .cast::<u8>())
                        .wrapping_offset(((playerCountIdx) as i32) as isize))
                        .read(),
                    );
                } else {
                    (((&raw mut template).cast::<u8>()).wrapping_add(4)).write(
                        (((((&raw const sResultsWindowHeights).cast::<u8>().cast_mut())
                            .cast::<u8>())
                        .cast::<u8>())
                        .wrapping_offset(((playerCountIdx) as i32) as isize))
                        .read(),
                    );
                }
                ((gfx).wrapping_add(130))
                    .write(((AddWindow((&raw mut template).cast::<u8>())) as u8));
                break 'l1;
            }
            if __sw1 == 1i32 {
                PutWindowTilemap(((gfx).wrapping_add(130)).read());
                FillWindowPixelBuffer(((gfx).wrapping_add(130)).read(), 0u8);
                break 'l1;
            }
            if __sw1 == 2i32 {
                LoadUserWindowBorderGfx_(((gfx).wrapping_add(130)).read(), 541u16, 208u8);
                DrawStdFrameWithCustomTileAndPalette(
                    ((gfx).wrapping_add(130)).read(),
                    0u8,
                    541u16,
                    13u8,
                );
                break 'l1;
            }
            if __sw1 == 3i32 {
                playerCountIdx =
                    ((((((game).wrapping_add(9)).read()) as i32).wrapping_sub(2i32)) as u8);
                'l2: {
                    let __sw2 = ((((game).wrapping_add(18).cast::<u16>()).read()) as i32);
                    if __sw2 == 11i32 {
                        PrintTextCentered(
                            ((gfx).wrapping_add(130)).read(),
                            20u8,
                            3u8,
                            (&raw mut gText_PressesRankings).cast::<u8>(),
                        );
                        PrintResultsText(
                            game,
                            0u8,
                            160u8,
                            (((8i32).wrapping_mul(
                                (((((((&raw const sResultsWindowHeights)
                                    .cast::<u8>()
                                    .cast_mut())
                                .cast::<u8>())
                                .cast::<u8>())
                                .wrapping_offset(((playerCountIdx) as i32) as isize))
                                .read()) as i32),
                            )) as u8),
                        );
                        ((gfx).wrapping_add(128)).write(5u8);
                        return 0u32;
                    }
                    if __sw2 == 12i32 {
                        PrintTextCentered(
                            ((gfx).wrapping_add(130)).read(),
                            20u8,
                            4u8,
                            ((((&raw const sResultsTexts)
                                .cast::<u8>()
                                .cast_mut()
                                .cast::<*mut u8>())
                            .cast::<*mut u8>())
                            .wrapping_offset(
                                (((((((((game).wrapping_add(104)).wrapping_add(32)).cast::<u8>())
                                    .cast::<u8>())
                                .wrapping_offset(7))
                                .read()) as i32)
                                    .wrapping_add(3i32)) as isize,
                            ))
                            .read(),
                        );
                        PrintResultsText(
                            game,
                            1u8,
                            160u8,
                            (((8i32).wrapping_mul(
                                (((((((&raw const sResultsWindowHeights)
                                    .cast::<u8>()
                                    .cast_mut())
                                .cast::<u8>())
                                .cast::<u8>())
                                .wrapping_offset(((playerCountIdx) as i32) as isize))
                                .read()) as i32),
                            )) as u8),
                        );
                        ((gfx).wrapping_add(128)).write(5u8);
                        return 0u32;
                    }
                    if __sw2 == 13i32 {
                        PrintTextCentered(
                            ((gfx).wrapping_add(130)).read(),
                            22u8,
                            3u8,
                            (&raw mut gText_CrushingResults).cast::<u8>(),
                        );
                        PrintResultsText(
                            game,
                            2u8,
                            176u8,
                            (((8i32).wrapping_mul(
                                ((((((((&raw const sResultsWindowHeights)
                                    .cast::<u8>()
                                    .cast_mut())
                                .cast::<u8>())
                                .wrapping_offset(4))
                                .cast::<u8>())
                                .wrapping_offset(((playerCountIdx) as i32) as isize))
                                .read()) as i32),
                            )) as u8),
                        );
                        break 'l2;
                    }
                }
                break 'l1;
            }
            if __sw1 == 4i32 {
                PrintCrushingResults(game);
                break 'l1;
            }
            if __sw1 == 5i32 {
                CopyWindowToVram(((gfx).wrapping_add(130)).read(), 3u8);
                ((gfx).wrapping_add(128)).write(0u8);
                return 1u32;
            }
        }
        let __p3 = (gfx).wrapping_add(128);
        (__p3).write(((__p3).read()).wrapping_add(1));
        return 0u32;
    }
}
pub(crate) unsafe extern "C" fn CloseResultsWindow(game: *mut u8) {
    unsafe {
        let mut game = game;
        ClearStdWindowAndFrameToTransparent(
            (((game).wrapping_add(312)).wrapping_add(130)).read(),
            1u8,
        );
        RemoveWindow((((game).wrapping_add(312)).wrapping_add(130)).read());
        DrawPlayerNameWindows(game);
    }
}
pub(crate) unsafe extern "C" fn Task_ShowRankings(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut i: u8 = 0u8;
        let mut j: u8 = 0u8;
        let mut xPos: u8 = 0u8;
        let mut yPos: u8 = 0u8;
        let mut score: u32 = 0u32;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        'l1: {
            let __sw1 = (((data).read()) as i32);
            let mut __fall = false;
            if __sw1 == 0i32 {
                __fall = true;
                ((data).wrapping_offset(1)).write(
                    ((AddWindow(
                        (&raw const sWindowTemplate_Rankings)
                            .cast::<u8>()
                            .cast_mut(),
                    )) as i16),
                );
                PutWindowTilemap(((((data).wrapping_offset(1)).read()) as u8));
                FillWindowPixelBuffer(((((data).wrapping_offset(1)).read()) as u8), 0u8);
                LoadUserWindowBorderGfx_(
                    ((((data).wrapping_offset(1)).read()) as u8),
                    541u16,
                    208u8,
                );
                DrawStdFrameWithCustomTileAndPalette(
                    ((((data).wrapping_offset(1)).read()) as u8),
                    0u8,
                    541u16,
                    13u8,
                );
                break 'l1;
            }
            if __sw1 == 1i32 {
                __fall = true;
                xPos = (((96u32).wrapping_sub(crate::c::div_u32(
                    ((GetStringWidth(1u8, (&raw mut gText_BerryCrush2).cast::<u8>(), (-1i16)))
                        as u32),
                    2u32,
                ))) as u8);
                AddTextPrinterParameterized3(
                    ((((data).wrapping_offset(1)).read()) as u8),
                    1u8,
                    xPos,
                    1u8,
                    ((((&raw const sTextColorTable).cast::<u8>().cast_mut()).cast::<u8>())
                        .wrapping_offset(9))
                    .cast::<u8>(),
                    0i8,
                    (&raw mut gText_BerryCrush2).cast::<u8>(),
                );
                xPos = (((96u32).wrapping_sub(crate::c::div_u32(
                    ((GetStringWidth(
                        1u8,
                        (&raw mut gText_PressingSpeedRankings).cast::<u8>(),
                        (-1i16),
                    )) as u32),
                    2u32,
                ))) as u8);
                AddTextPrinterParameterized3(
                    ((((data).wrapping_offset(1)).read()) as u8),
                    1u8,
                    xPos,
                    17u8,
                    ((((&raw const sTextColorTable).cast::<u8>().cast_mut()).cast::<u8>())
                        .wrapping_offset(9))
                    .cast::<u8>(),
                    0i8,
                    (&raw mut gText_PressingSpeedRankings).cast::<u8>(),
                );
                yPos = 41u8;
                {
                    i = 0u8;
                    'l2: loop {
                        if !(((i) as i32) < 4i32) {
                            break 'l2;
                        }
                        'l3: {
                            ConvertIntToDecimalStringN(
                                (&raw mut gStringVar1).cast::<u8>(),
                                ((i) as i32).wrapping_add(2i32),
                                0i32,
                                1u8,
                            );
                            StringExpandPlaceholders(
                                (&raw mut gStringVar4).cast::<u8>(),
                                (&raw mut gText_Var1Players).cast::<u8>(),
                            );
                            AddTextPrinterParameterized3(
                                ((((data).wrapping_offset(1)).read()) as u8),
                                1u8,
                                0u8,
                                yPos,
                                (((&raw const sTextColorTable).cast::<u8>().cast_mut())
                                    .cast::<u8>())
                                .cast::<u8>(),
                                0i8,
                                (&raw mut gStringVar4).cast::<u8>(),
                            );
                            xPos = (((192i32).wrapping_sub(
                                (((GetStringWidth(
                                    1u8,
                                    (&raw mut gText_TimesPerSec).cast::<u8>(),
                                    (-1i16),
                                )) as u8) as i32),
                            )) as u8);
                            AddTextPrinterParameterized3(
                                ((((data).wrapping_offset(1)).read()) as u8),
                                1u8,
                                xPos,
                                yPos,
                                (((&raw const sTextColorTable).cast::<u8>().cast_mut())
                                    .cast::<u8>())
                                .cast::<u8>(),
                                0i8,
                                (&raw mut gText_TimesPerSec).cast::<u8>(),
                            );
                            {
                                j = 0u8;
                                'l4: loop {
                                    if !(((j) as i32) < 8i32) {
                                        break 'l4;
                                    }
                                    'l5: {
                                        if (crate::c::shr_i32(
                                            (((((data).wrapping_offset(
                                                ((2i32).wrapping_add(((i) as i32))) as isize,
                                            ))
                                            .read())
                                                as i32)
                                                & 255i32),
                                            (((7i32).wrapping_sub(((j) as i32))) as u32),
                                        ) & 1i32)
                                            != 0
                                        {
                                            score = (score).wrapping_add(
                                                ((((&raw const sPressingSpeedConversionTable)
                                                    .cast::<u8>()
                                                    .cast_mut()
                                                    .cast::<u32>())
                                                .cast::<u32>())
                                                .wrapping_offset(((j) as i32) as isize))
                                                .read(),
                                            );
                                        }
                                    }
                                    j = (j).wrapping_add(1);
                                }
                            }
                            ConvertIntToDecimalStringN(
                                (&raw mut gStringVar1).cast::<u8>(),
                                ((((((data)
                                    .wrapping_offset(((2i32).wrapping_add(((i) as i32))) as isize))
                                .read()) as u16) as i32)
                                    >> 8),
                                1i32,
                                3u8,
                            );
                            ConvertIntToDecimalStringN(
                                (&raw mut gStringVar2).cast::<u8>(),
                                ((crate::c::div_u32(score, 1000000u32)) as i32),
                                2i32,
                                2u8,
                            );
                            StringExpandPlaceholders(
                                (&raw mut gStringVar4).cast::<u8>(),
                                (&raw mut gText_XDotY3).cast::<u8>(),
                            );
                            xPos = ((((xPos) as i32).wrapping_sub(GetStringWidth(
                                1u8,
                                (&raw mut gStringVar4).cast::<u8>(),
                                (-1i16),
                            ))) as u8);
                            AddTextPrinterParameterized3(
                                ((((data).wrapping_offset(1)).read()) as u8),
                                1u8,
                                xPos,
                                yPos,
                                (((&raw const sTextColorTable).cast::<u8>().cast_mut())
                                    .cast::<u8>())
                                .cast::<u8>(),
                                0i8,
                                (&raw mut gStringVar4).cast::<u8>(),
                            );
                            yPos = ((((yPos) as i32).wrapping_add(16i32)) as u8);
                            score = 0u32;
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                CopyWindowToVram(((((data).wrapping_offset(1)).read()) as u8), 3u8);
                break 'l1;
            }
            if __sw1 == 2i32 {
                __fall = true;
                if ((((((&raw mut gMain).cast::<u8>())
                    .wrapping_add(46)
                    .cast::<u16>())
                .read()) as i32)
                    & 3i32)
                    != 0
                {
                    break 'l1;
                } else {
                    return;
                }
            }
            if __fall || __sw1 == 3i32 {
                __fall = true;
                ClearStdWindowAndFrameToTransparent(
                    ((((data).wrapping_offset(1)).read()) as u8),
                    1u8,
                );
                ClearWindowTilemap(((((data).wrapping_offset(1)).read()) as u8));
                RemoveWindow(((((data).wrapping_offset(1)).read()) as u8));
                DestroyTask(taskId);
                ScriptContext_Enable();
                UnlockPlayerFieldControls();
                (data).write(0i16);
                return;
            }
        }
        (data).write(((data).read()).wrapping_add(1));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ShowBerryCrushRankings() {
    unsafe {
        let mut taskId: u8 = 0u8;
        LockPlayerFieldControls();
        taskId = CreateTask(Some(Task_ShowRankings), 0u8);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(2))
        .write(
            (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(492))
                .cast::<u16>())
            .read()) as i16),
        );
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(3))
        .write(
            ((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(492))
                .cast::<u16>())
            .wrapping_offset(1))
            .read()) as i16),
        );
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(4))
        .write(
            ((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(492))
                .cast::<u16>())
            .wrapping_offset(2))
            .read()) as i16),
        );
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(5))
        .write(
            ((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(492))
                .cast::<u16>())
            .wrapping_offset(3))
            .read()) as i16),
        );
    }
}
pub(crate) unsafe extern "C" fn PrintTimer(gfx: *mut u8, timer: u16) {
    unsafe {
        let mut gfx = gfx;
        let mut timer = timer;
        FramesToMinSec(gfx, timer);
        DigitObjUtil_PrintNumOn(
            0u32,
            ((((gfx).wrapping_add(4).cast::<i16>()).read()) as i32),
        );
        DigitObjUtil_PrintNumOn(
            1u32,
            ((((gfx).wrapping_add(6).cast::<i16>()).read()) as i32),
        );
        DigitObjUtil_PrintNumOn(
            2u32,
            ((((gfx).wrapping_add(8).cast::<i16>()).read()) as i32),
        );
    }
}
pub(crate) unsafe extern "C" fn HideTimer(gfx: *mut u8) {
    unsafe {
        let mut gfx = gfx;
        crate::c::bf_write(
            ((((gfx).wrapping_add(120)).cast::<*mut u8>()).read()).wrapping_add(62),
            2,
            1,
            (1u16) as i32,
        );
        crate::c::bf_write(
            (((((gfx).wrapping_add(120)).cast::<*mut u8>()).wrapping_offset(1)).read())
                .wrapping_add(62),
            2,
            1,
            (1u16) as i32,
        );
        DigitObjUtil_HideOrShow(2u32, 1u32);
        DigitObjUtil_HideOrShow(1u32, 1u32);
        DigitObjUtil_HideOrShow(0u32, 1u32);
    }
}
pub(crate) unsafe extern "C" fn CreatePlayerNameWindows(game: *mut u8) {
    unsafe {
        let mut game = game;
        let mut i: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < ((((game).wrapping_add(9)).read()) as i32)) {
                    break 'l1;
                }
                'l2: {
                    (((((game).wrapping_add(312)).wrapping_add(12)).cast::<*mut u8>())
                        .wrapping_offset(((i) as i32) as isize))
                    .write(
                        (((&raw const sPlayerCoords).cast::<u8>().cast_mut()).cast::<u8>())
                            .wrapping_offset(
                                ((((((((&raw const sPlayerIdToPosId).cast::<u8>().cast_mut())
                                    .cast::<u8>())
                                .wrapping_offset(
                                    (((((game).wrapping_add(9)).read()) as i32).wrapping_sub(2i32))
                                        as isize
                                        * 5,
                                ))
                                .cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize))
                                .read()) as i32) as isize
                                    * 12,
                            ),
                    );
                    (((((game).wrapping_add(312)).wrapping_add(131)).cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize))
                    .write(
                        ((AddWindow(
                            (((&raw const sWindowTemplates_PlayerNames)
                                .cast::<u8>()
                                .cast_mut())
                            .cast::<u8>())
                            .wrapping_offset(
                                ((((((((game).wrapping_add(312)).wrapping_add(12))
                                    .cast::<*mut u8>())
                                .wrapping_offset(((i) as i32) as isize))
                                .read())
                                .read()) as i32) as isize
                                    * 8,
                            ),
                        )) as u8),
                    );
                    PutWindowTilemap(
                        (((((game).wrapping_add(312)).wrapping_add(131)).cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize))
                        .read(),
                    );
                    FillWindowPixelBuffer(
                        (((((game).wrapping_add(312)).wrapping_add(131)).cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize))
                        .read(),
                        0u8,
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn DrawPlayerNameWindows(game: *mut u8) {
    unsafe {
        let mut game = game;
        let mut i: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < ((((game).wrapping_add(9)).read()) as i32)) {
                    break 'l1;
                }
                'l2: {
                    PutWindowTilemap(
                        (((((game).wrapping_add(312)).wrapping_add(131)).cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize))
                        .read(),
                    );
                    if ((i) as i32) == ((((game).wrapping_add(8)).read()) as i32) {
                        AddTextPrinterParameterized4(
                            (((((game).wrapping_add(312)).wrapping_add(131)).cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize))
                            .read(),
                            2u8,
                            (((36u32).wrapping_sub(crate::c::div_u32(
                                ((GetStringWidth(
                                    2u8,
                                    ((((game).wrapping_add(152)).cast::<u8>())
                                        .wrapping_offset(((i) as i32) as isize * 32))
                                    .cast::<u8>(),
                                    0i16,
                                )) as u32),
                                2u32,
                            ))) as u8),
                            1u8,
                            0u8,
                            0u8,
                            ((((&raw const sTextColorTable).cast::<u8>().cast_mut()).cast::<u8>())
                                .wrapping_offset(3))
                            .cast::<u8>(),
                            0i8,
                            ((((game).wrapping_add(152)).cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize * 32))
                            .cast::<u8>(),
                        );
                    } else {
                        AddTextPrinterParameterized4(
                            (((((game).wrapping_add(312)).wrapping_add(131)).cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize))
                            .read(),
                            2u8,
                            (((36u32).wrapping_sub(crate::c::div_u32(
                                ((GetStringWidth(
                                    2u8,
                                    ((((game).wrapping_add(152)).cast::<u8>())
                                        .wrapping_offset(((i) as i32) as isize * 32))
                                    .cast::<u8>(),
                                    0i16,
                                )) as u32),
                                2u32,
                            ))) as u8),
                            1u8,
                            0u8,
                            0u8,
                            ((((&raw const sTextColorTable).cast::<u8>().cast_mut()).cast::<u8>())
                                .wrapping_offset(6))
                            .cast::<u8>(),
                            0i8,
                            ((((game).wrapping_add(152)).cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize * 32))
                            .cast::<u8>(),
                        );
                    }
                    CopyWindowToVram(
                        (((((game).wrapping_add(312)).wrapping_add(131)).cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize))
                        .read(),
                        3u8,
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        CopyBgTilemapBufferToVram(0u8);
    }
}
pub(crate) unsafe extern "C" fn CopyPlayerNameWindowGfxToBg(game: *mut u8) {
    unsafe {
        let mut game = game;
        let mut i: u8 = 0u8;
        let mut windowGfx: *mut u8 = core::ptr::null_mut();
        LZ77UnCompWram(
            ((&raw mut gBerryCrush_TextWindows_Tilemap).cast::<u32>()).cast::<u32>(),
            (&raw mut gDecompressionBuffer).cast::<u8>(),
        );
        {
            windowGfx = (&raw mut gDecompressionBuffer).cast::<u8>();
            'l1: loop {
                if !(((i) as i32) < ((((game).wrapping_add(9)).read()) as i32)) {
                    break 'l1;
                }
                'l2: {
                    CopyToBgTilemapBufferRect(
                        3u8,
                        (windowGfx).wrapping_offset(
                            (((((((((game).wrapping_add(312)).wrapping_add(12))
                                .cast::<*mut u8>())
                            .wrapping_offset(((i) as i32) as isize))
                            .read())
                            .read()) as i32)
                                .wrapping_mul(40i32)) as isize,
                        ),
                        (((((((game).wrapping_add(312)).wrapping_add(12)).cast::<*mut u8>())
                            .wrapping_offset(((i) as i32) as isize))
                        .read())
                        .wrapping_add(1))
                        .read(),
                        (((((((game).wrapping_add(312)).wrapping_add(12)).cast::<*mut u8>())
                            .wrapping_offset(((i) as i32) as isize))
                        .read())
                        .wrapping_add(2))
                        .read(),
                        10u8,
                        2u8,
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        CopyBgTilemapBufferToVram(3u8);
    }
}
pub(crate) unsafe extern "C" fn CreateGameSprites(game: *mut u8) {
    unsafe {
        let mut game = game;
        let mut i: u8 = 0u8;
        let mut spriteId: u8 = 0u8;
        ((game).wrapping_add(42).cast::<i16>()).write((-104i16));
        ((game).wrapping_add(44).cast::<i16>()).write(0i16);
        ((&raw mut gSpriteCoordOffsetX).cast::<i16>()).write(0i16);
        ((&raw mut gSpriteCoordOffsetY).cast::<i16>()).write((-104i16));
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as u32) < (crate::c::div_u32(40u32, 8u32)).wrapping_sub(1u32)) {
                    break 'l1;
                }
                'l2: {
                    LoadCompressedSpriteSheet(
                        (((&raw const sSpriteSheets).cast::<u8>().cast_mut()).cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 8),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        LoadSpritePalettes(((&raw const sSpritePals).cast::<u8>().cast_mut()).cast::<u8>());
        spriteId = CreateSprite(
            (&raw const sSpriteTemplate_CrusherBase)
                .cast::<u8>()
                .cast_mut(),
            120i16,
            88i16,
            5u8,
        );
        (((game).wrapping_add(312))
            .wrapping_add(32)
            .cast::<*mut u8>())
        .write(
            ((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68),
        );
        crate::c::bf_write(
            ((((game).wrapping_add(312))
                .wrapping_add(32)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(5),
            2,
            2,
            (3u16) as i32,
        );
        crate::c::bf_write(
            ((((game).wrapping_add(312))
                .wrapping_add(32)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(62),
            1,
            1,
            (1u16) as i32,
        );
        crate::c::bf_write(
            ((((game).wrapping_add(312))
                .wrapping_add(32)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(44),
            6,
            1,
            (1u8) as i32,
        );
        {
            i = 0u8;
            'l3: loop {
                if !(((i) as i32) < ((((game).wrapping_add(9)).read()) as i32)) {
                    break 'l3;
                }
                'l4: {
                    spriteId = CreateSprite(
                        (&raw const sSpriteTemplate_Impact).cast::<u8>().cast_mut(),
                        (((((((((((game).wrapping_add(312)).wrapping_add(12)).cast::<*mut u8>())
                            .wrapping_offset(((i) as i32) as isize))
                        .read())
                        .wrapping_add(4)
                        .cast::<i16>())
                        .read()) as i32)
                            .wrapping_add(120i32)) as i16),
                        (((((((((((game).wrapping_add(312)).wrapping_add(12)).cast::<*mut u8>())
                            .wrapping_offset(((i) as i32) as isize))
                        .read())
                        .wrapping_add(6)
                        .cast::<i16>())
                        .read()) as i32)
                            .wrapping_add(32i32)) as i16),
                        0u8,
                    );
                    (((((game).wrapping_add(312)).wrapping_add(36)).cast::<*mut u8>())
                        .wrapping_offset(((i) as i32) as isize))
                    .write(
                        ((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((spriteId) as i32) as isize * 68),
                    );
                    crate::c::bf_write(
                        ((((((game).wrapping_add(312)).wrapping_add(36)).cast::<*mut u8>())
                            .wrapping_offset(((i) as i32) as isize))
                        .read())
                        .wrapping_add(5),
                        2,
                        2,
                        (1u16) as i32,
                    );
                    crate::c::bf_write(
                        ((((((game).wrapping_add(312)).wrapping_add(36)).cast::<*mut u8>())
                            .wrapping_offset(((i) as i32) as isize))
                        .read())
                        .wrapping_add(62),
                        2,
                        1,
                        (1u16) as i32,
                    );
                    crate::c::bf_write(
                        ((((((game).wrapping_add(312)).wrapping_add(36)).cast::<*mut u8>())
                            .wrapping_offset(((i) as i32) as isize))
                        .read())
                        .wrapping_add(62),
                        1,
                        1,
                        (1u16) as i32,
                    );
                    crate::c::bf_write(
                        ((((((game).wrapping_add(312)).wrapping_add(36)).cast::<*mut u8>())
                            .wrapping_offset(((i) as i32) as isize))
                        .read())
                        .wrapping_add(44),
                        6,
                        1,
                        (1u8) as i32,
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        {
            i = 0u8;
            'l5: loop {
                if !(((i) as u32) < crate::c::div_u32(44u32, 4u32)) {
                    break 'l5;
                }
                'l6: {
                    spriteId = CreateSprite(
                        (&raw const sSpriteTemplate_Sparkle).cast::<u8>().cast_mut(),
                        (((((((((&raw const sSparkleCoords).cast::<u8>().cast_mut())
                            .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 2))
                        .cast::<i8>())
                        .read()) as i32)
                            .wrapping_add(120i32)) as i16),
                        ((((((((((&raw const sSparkleCoords).cast::<u8>().cast_mut())
                            .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 2))
                        .cast::<i8>())
                        .wrapping_offset(1))
                        .read()) as i32)
                            .wrapping_add(136i32)) as i16),
                        6u8,
                    );
                    (((((game).wrapping_add(312)).wrapping_add(76)).cast::<*mut u8>())
                        .wrapping_offset(((i) as i32) as isize))
                    .write(
                        ((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((spriteId) as i32) as isize * 68),
                    );
                    crate::c::bf_write(
                        ((((((game).wrapping_add(312)).wrapping_add(76)).cast::<*mut u8>())
                            .wrapping_offset(((i) as i32) as isize))
                        .read())
                        .wrapping_add(5),
                        2,
                        2,
                        (3u16) as i32,
                    );
                    crate::c::bf_write(
                        ((((((game).wrapping_add(312)).wrapping_add(76)).cast::<*mut u8>())
                            .wrapping_offset(((i) as i32) as isize))
                        .read())
                        .wrapping_add(62),
                        2,
                        1,
                        (1u16) as i32,
                    );
                    crate::c::bf_write(
                        ((((((game).wrapping_add(312)).wrapping_add(76)).cast::<*mut u8>())
                            .wrapping_offset(((i) as i32) as isize))
                        .read())
                        .wrapping_add(44),
                        6,
                        1,
                        (1u8) as i32,
                    );
                    ((((((((game).wrapping_add(312)).wrapping_add(76)).cast::<*mut u8>())
                        .wrapping_offset(((i) as i32) as isize))
                    .read())
                    .wrapping_add(46))
                    .cast::<i16>())
                    .write(((i) as i16));
                }
                i = (i).wrapping_add(1);
            }
        }
        {
            i = 0u8;
            'l7: loop {
                if !(((i) as u32) < crate::c::div_u32(8u32, 4u32)) {
                    break 'l7;
                }
                'l8: {
                    spriteId = CreateSprite(
                        (&raw const sSpriteTemplate_Timer).cast::<u8>().cast_mut(),
                        ((((24i32).wrapping_mul(((i) as i32))).wrapping_add(176i32)) as i16),
                        8i16,
                        0u8,
                    );
                    (((((game).wrapping_add(312)).wrapping_add(120)).cast::<*mut u8>())
                        .wrapping_offset(((i) as i32) as isize))
                    .write(
                        ((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((spriteId) as i32) as isize * 68),
                    );
                    crate::c::bf_write(
                        ((((((game).wrapping_add(312)).wrapping_add(120)).cast::<*mut u8>())
                            .wrapping_offset(((i) as i32) as isize))
                        .read())
                        .wrapping_add(5),
                        2,
                        2,
                        (0u16) as i32,
                    );
                    crate::c::bf_write(
                        ((((((game).wrapping_add(312)).wrapping_add(120)).cast::<*mut u8>())
                            .wrapping_offset(((i) as i32) as isize))
                        .read())
                        .wrapping_add(62),
                        2,
                        1,
                        (0u16) as i32,
                    );
                    crate::c::bf_write(
                        ((((((game).wrapping_add(312)).wrapping_add(120)).cast::<*mut u8>())
                            .wrapping_offset(((i) as i32) as isize))
                        .read())
                        .wrapping_add(44),
                        6,
                        1,
                        (0u8) as i32,
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        DigitObjUtil_CreatePrinter(
            0u32,
            0i32,
            ((&raw const sDigitObjTemplates).cast::<u8>().cast_mut()).cast::<u8>(),
        );
        DigitObjUtil_CreatePrinter(
            1u32,
            0i32,
            (((&raw const sDigitObjTemplates).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(16),
        );
        DigitObjUtil_CreatePrinter(
            2u32,
            0i32,
            (((&raw const sDigitObjTemplates).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(32),
        );
        if ((((game).wrapping_add(18).cast::<u16>()).read()) as i32) == 1i32 {
            HideTimer((game).wrapping_add(312));
        }
    }
}
pub(crate) unsafe extern "C" fn DestroyGameSprites(game: *mut u8) {
    unsafe {
        let mut game = game;
        let mut i: u8 = 0u8;
        FreeSpriteTilesByTag(4u16);
        FreeSpriteTilesByTag(3u16);
        FreeSpriteTilesByTag(2u16);
        FreeSpriteTilesByTag(1u16);
        FreeSpritePaletteByTag(4u16);
        FreeSpritePaletteByTag(2u16);
        FreeSpritePaletteByTag(1u16);
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as u32) < crate::c::div_u32(8u32, 4u32)) {
                    break 'l1;
                }
                'l2: {
                    DestroySprite(
                        (((((game).wrapping_add(312)).wrapping_add(120)).cast::<*mut u8>())
                            .wrapping_offset(((i) as i32) as isize))
                        .read(),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        DigitObjUtil_DeletePrinter(2u32);
        DigitObjUtil_DeletePrinter(1u32);
        DigitObjUtil_DeletePrinter(0u32);
        {
            i = 0u8;
            'l3: loop {
                if !(((i) as u32) < crate::c::div_u32(44u32, 4u32)) {
                    break 'l3;
                }
                'l4: {
                    DestroySprite(
                        (((((game).wrapping_add(312)).wrapping_add(76)).cast::<*mut u8>())
                            .wrapping_offset(((i) as i32) as isize))
                        .read(),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        {
            i = 0u8;
            'l5: loop {
                if !(((i) as i32) < ((((game).wrapping_add(9)).read()) as i32)) {
                    break 'l5;
                }
                'l6: {
                    DestroySprite(
                        (((((game).wrapping_add(312)).wrapping_add(36)).cast::<*mut u8>())
                            .wrapping_offset(((i) as i32) as isize))
                        .read(),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        if (crate::c::bf_read(
            ((((game).wrapping_add(312))
                .wrapping_add(32)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(62),
            0,
            1,
            false,
        ) as u16)
            != 0
        {
            DestroySprite(
                (((game).wrapping_add(312))
                    .wrapping_add(32)
                    .cast::<*mut u8>())
                .read(),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_Impact(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (crate::c::bf_read((sprite).wrapping_add(63), 4, 1, false) as u16) != 0 {
            crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (1u16) as i32);
            crate::c::bf_write((sprite).wrapping_add(44), 6, 1, (1u8) as i32);
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_Sparkle_End(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut i: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as u32) < crate::c::div_u32(16u32, 2u32)) {
                    break 'l1;
                }
                'l2: {
                    ((((sprite).wrapping_add(46)).cast::<i16>())
                        .wrapping_offset(((i) as i32) as isize))
                    .write(0i16);
                }
                i = (i).wrapping_add(1);
            }
        }
        ((sprite).wrapping_add(36).cast::<i16>()).write(0i16);
        ((sprite).wrapping_add(38).cast::<i16>()).write(0i16);
        crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (1u16) as i32);
        crate::c::bf_write((sprite).wrapping_add(44), 6, 1, (1u8) as i32);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(SpriteCallbackDummy));
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_Sparkle(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut data: *mut i16 = ((sprite).wrapping_add(46)).cast::<i16>();
        let __p1 = (data).wrapping_offset(1);
        (__p1).write(
            (((((__p1).read()) as i32).wrapping_add(((((data).wrapping_offset(2)).read()) as i32)))
                as i16),
        );
        let __p2 = (sprite).wrapping_add(38).cast::<i16>();
        (__p2).write(
            (((((__p2).read()) as i32)
                .wrapping_add((((((data).wrapping_offset(1)).read()) as i32) >> 8)))
                as i16),
        );
        if (((((data).wrapping_offset(7)).read()) as i32) & 32768i32) != 0 {
            let __p3 = ((sprite).wrapping_add(46)).cast::<i16>();
            (__p3).write(
                (((((__p3).read()) as i32)
                    .wrapping_add(((((data).wrapping_offset(3)).read()) as i32)))
                    as i16),
            );
            let __p4 = (data).wrapping_offset(4);
            (__p4).write(
                (((((__p4).read()) as i32)
                    .wrapping_add(((((data).wrapping_offset(5)).read()) as i32)))
                    as i16),
            );
            ((sprite).wrapping_add(36).cast::<i16>()).write(Sin(
                ((((((data).wrapping_offset(4)).read()) as i32) >> 7) as i16),
                ((data).wrapping_offset(6)).read(),
            ));
            if ((((((data).wrapping_offset(7)).read()) as i32) & 32768i32) != 0)
                && ((((((data).wrapping_offset(4)).read()) as i32) >> 7) > 126i32)
            {
                ((sprite).wrapping_add(36).cast::<i16>()).write(0i16);
                let __p5 = (data).wrapping_offset(7);
                (__p5).write((((((__p5).read()) as i32) & 32767i32) as i16));
            }
        }
        ((sprite).wrapping_add(32).cast::<i16>()).write((((((data).read()) as i32) >> 7) as i16));
        if ((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32)
            .wrapping_add(((((sprite).wrapping_add(38).cast::<i16>()).read()) as i32))
            > (((((data).wrapping_offset(7)).read()) as i32) & 32767i32)
        {
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(SpriteCB_Sparkle_End));
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_Sparkle_Init(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut data: *mut i16 = ((sprite).wrapping_add(46)).cast::<i16>();
        let mut xMult: i16 = 0i16;
        let mut xDiv: i16 = 0i16;
        let mut var: i32 = 0i32;
        let mut zero: u32 = 0u32;
        var = 640i32;
        ((data).wrapping_offset(1)).write(((var) as i16));
        ((data).wrapping_offset(2)).write(32i16);
        ((data).wrapping_offset(7)).write(168i16);
        xMult = ((((((sprite).wrapping_add(36).cast::<i16>()).read()) as i32).wrapping_mul(128i32))
            as i16);
        xDiv = MathUtil_Div16Shift(
            7u8,
            (((168i32).wrapping_sub(((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32))
                << 7) as i16),
            (((var).wrapping_add(32i32) >> 1) as i16),
        );
        (((sprite).wrapping_add(46)).cast::<i16>())
            .write(((((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32) << 7) as i16));
        ((data).wrapping_offset(3)).write(MathUtil_Div16Shift(7u8, xMult, xDiv));
        var = ((MathUtil_Mul16Shift(7u8, xDiv, 85i16)) as i32);
        ((data).wrapping_offset(4)).write(((zero) as i16));
        ((data).wrapping_offset(5)).write(MathUtil_Div16Shift(
            7u8,
            ((((63.5f32) as f32) * ((256i32) as f32)) as i16),
            ((var) as i16),
        ));
        ((data).wrapping_offset(6)).write(
            ((crate::c::div_i32(
                ((((sprite).wrapping_add(36).cast::<i16>()).read()) as i32),
                4i32,
            )) as i16),
        );
        let __p1 = (data).wrapping_offset(7);
        (__p1).write((((((__p1).read()) as i32) | 32768i32) as i16));
        ((sprite).wrapping_add(38).cast::<i16>()).write(((zero) as i16));
        ((sprite).wrapping_add(36).cast::<i16>()).write(((zero) as i16));
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(SpriteCB_Sparkle));
        crate::c::bf_write((sprite).wrapping_add(44), 6, 1, (0u8) as i32);
        crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (0u16) as i32);
    }
}
pub(crate) unsafe extern "C" fn RunOrScheduleCommand(cmdId: u16, mode: u8, args: *mut u8) {
    unsafe {
        let mut cmdId = cmdId;
        let mut mode = mode;
        let mut args = args;
        let mut game: *mut u8 = GetBerryCrushGame();
        if ((cmdId) as u32) >= crate::c::div_u32(104u32, 4u32) {
            cmdId = 0u16;
        }
        'l1: {
            let __sw1 = ((mode) as i32);
            if __sw1 == 0i32 {
                if ((cmdId) as i32) != 0i32 {
                    (((((&raw const sBerryCrushCommands)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<Option<unsafe extern "C" fn(*mut u8, *mut u8) -> u32>>())
                    .cast::<Option<unsafe extern "C" fn(*mut u8, *mut u8) -> u32>>())
                    .wrapping_offset(((cmdId) as i32) as isize))
                    .read())
                    .unwrap_unchecked()(game, args);
                }
                if ((((game).wrapping_add(14)).read()) as u32) >= crate::c::div_u32(104u32, 4u32) {
                    ((game).wrapping_add(14)).write(0u8);
                }
                ((game)
                    .wrapping_add(4)
                    .cast::<Option<unsafe extern "C" fn(*mut u8, *mut u8) -> u32>>())
                .write(
                    ((((&raw const sBerryCrushCommands)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<Option<unsafe extern "C" fn(*mut u8, *mut u8) -> u32>>())
                    .cast::<Option<unsafe extern "C" fn(*mut u8, *mut u8) -> u32>>())
                    .wrapping_offset(((((game).wrapping_add(14)).read()) as i32) as isize))
                    .read(),
                );
                break 'l1;
            }
            if __sw1 == 1i32 {
                ((game)
                    .wrapping_add(4)
                    .cast::<Option<unsafe extern "C" fn(*mut u8, *mut u8) -> u32>>())
                .write(
                    ((((&raw const sBerryCrushCommands)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<Option<unsafe extern "C" fn(*mut u8, *mut u8) -> u32>>())
                    .cast::<Option<unsafe extern "C" fn(*mut u8, *mut u8) -> u32>>())
                    .wrapping_offset(((cmdId) as i32) as isize))
                    .read(),
                );
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Cmd_BeginNormalPaletteFade(game: *mut u8, args: *mut u8) -> u32 {
    unsafe {
        let mut game = game;
        let mut args = args;
        let mut color: u16 = 0u16;
        let mut selectedPals = crate::ffi::Align4([0u8; 8]);
        ((&raw mut selectedPals).cast::<u32>()).write((((args).read()) as u32));
        (((&raw mut selectedPals).cast::<u32>()).wrapping_offset(1))
            .write(((((args).wrapping_offset(1)).read()) as u32));
        let __p1 = ((&raw mut selectedPals).cast::<u32>()).wrapping_offset(1);
        (__p1).write(((__p1).read() << 8));
        let __p2 = (&raw mut selectedPals).cast::<u32>();
        (__p2).write(
            ((__p2).read() | (((&raw mut selectedPals).cast::<u32>()).wrapping_offset(1)).read()),
        );
        (((&raw mut selectedPals).cast::<u32>()).wrapping_offset(1))
            .write(((((args).wrapping_offset(2)).read()) as u32));
        let __p3 = ((&raw mut selectedPals).cast::<u32>()).wrapping_offset(1);
        (__p3).write(((__p3).read() << 16));
        let __p4 = (&raw mut selectedPals).cast::<u32>();
        (__p4).write(
            ((__p4).read() | (((&raw mut selectedPals).cast::<u32>()).wrapping_offset(1)).read()),
        );
        (((&raw mut selectedPals).cast::<u32>()).wrapping_offset(1))
            .write(((((args).wrapping_offset(3)).read()) as u32));
        let __p5 = ((&raw mut selectedPals).cast::<u32>()).wrapping_offset(1);
        (__p5).write(((__p5).read() << 24));
        let __p6 = (&raw mut selectedPals).cast::<u32>();
        (__p6).write(
            ((__p6).read() | (((&raw mut selectedPals).cast::<u32>()).wrapping_offset(1)).read()),
        );
        (args).write(((args).wrapping_offset(9)).read());
        color = ((((args).wrapping_offset(8)).read()) as u16);
        color = ((((color) as i32) << 8) as u16);
        color = ((((color) as i32) | ((((args).wrapping_offset(7)).read()) as i32)) as u16);
        crate::c::bf_write(
            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(8),
            7,
            1,
            (0u16) as i32,
        );
        BeginNormalPaletteFade(
            ((&raw mut selectedPals).cast::<u32>()).read(),
            ((((args).wrapping_offset(4)).read()) as i8),
            ((args).wrapping_offset(5)).read(),
            ((args).wrapping_offset(6)).read(),
            color,
        );
        UpdatePaletteFade();
        ((game).wrapping_add(14)).write(2u8);
        return 0u32;
    }
}
pub(crate) unsafe extern "C" fn Cmd_WaitPaletteFade(game: *mut u8, args: *mut u8) -> u32 {
    unsafe {
        let mut game = game;
        let mut args = args;
        'l1: {
            let __sw1 = ((((game).wrapping_add(12)).read()) as i32);
            let __matched = __sw1 == 0i32 || __sw1 == 1i32 || __sw1 == 2i32 || __sw1 == 3i32;
            if __sw1 == 0i32 {
                if (UpdatePaletteFade()) != 0 {
                    return 0u32;
                }
                if (((args).read()) as i32) != 0i32 {
                    let __p2 = (game).wrapping_add(12);
                    (__p2).write(((__p2).read()).wrapping_add(1));
                } else {
                    ((game).wrapping_add(12)).write(3u8);
                }
                return 0u32;
            }
            if __sw1 == 1i32 {
                Rfu_SetLinkStandbyCallback();
                let __p3 = (game).wrapping_add(12);
                (__p3).write(((__p3).read()).wrapping_add(1));
                return 0u32;
            }
            if __sw1 == 2i32 {
                if (IsLinkTaskFinished()) != 0 {
                    let __p4 = (game).wrapping_add(12);
                    (__p4).write(((__p4).read()).wrapping_add(1));
                    return 0u32;
                }
                return 0u32;
            }
            if __sw1 == 3i32 {
                RunOrScheduleCommand(
                    ((((game).wrapping_add(15)).read()) as u16),
                    1u8,
                    core::ptr::null_mut(),
                );
                ((game).wrapping_add(12)).write(0u8);
                return 0u32;
            }
            if !__matched {
                let __p5 = (game).wrapping_add(12);
                (__p5).write(((__p5).read()).wrapping_add(1));
                return 0u32;
            }
        }
        #[allow(unreachable_code)]
        {
            return 0u32;
        }
    }
}
pub(crate) unsafe extern "C" fn Cmd_PrintMessage(game: *mut u8, args: *mut u8) -> u32 {
    unsafe {
        let mut game = game;
        let mut args = args;
        let mut keys: u16 = ((((args).wrapping_offset(3)).read()) as u16);
        keys = ((((keys) as i32) << 8) as u16);
        keys = ((((keys) as i32) | ((((args).wrapping_offset(2)).read()) as i32)) as u16);
        'l1: {
            let __sw1 = ((((game).wrapping_add(12)).read()) as i32);
            if __sw1 == 0i32 {
                DrawDialogueFrame(0u8, 0u8);
                if (((((args).wrapping_offset(1)).read()) as i32) & 2i32) != 0 {
                    StringExpandPlaceholders(
                        (&raw mut gStringVar4).cast::<u8>(),
                        ((((&raw const sMessages)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<*mut u8>())
                        .cast::<*mut u8>())
                        .wrapping_offset((((args).read()) as i32) as isize))
                        .read(),
                    );
                    AddTextPrinterParameterized2(
                        0u8,
                        1u8,
                        (&raw mut gStringVar4).cast::<u8>(),
                        ((game).wrapping_add(11)).read(),
                        None,
                        2u8,
                        1u8,
                        3u8,
                    );
                } else {
                    AddTextPrinterParameterized2(
                        0u8,
                        1u8,
                        ((((&raw const sMessages)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<*mut u8>())
                        .cast::<*mut u8>())
                        .wrapping_offset((((args).read()) as i32) as isize))
                        .read(),
                        ((game).wrapping_add(11)).read(),
                        None,
                        2u8,
                        1u8,
                        3u8,
                    );
                }
                CopyWindowToVram(0u8, 3u8);
                break 'l1;
            }
            if __sw1 == 1i32 {
                if !((IsTextPrinterActive(0u8)) != 0) {
                    if ((keys) as i32) == 0i32 {
                        let __p2 = (game).wrapping_add(12);
                        (__p2).write(((__p2).read()).wrapping_add(1));
                    }
                    break 'l1;
                }
                return 0u32;
            }
            if __sw1 == 2i32 {
                if !(((((((&raw mut gMain).cast::<u8>())
                    .wrapping_add(46)
                    .cast::<u16>())
                .read()) as i32)
                    & ((keys) as i32))
                    != 0)
                {
                    return 0u32;
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                if (((((args).wrapping_offset(1)).read()) as i32) & 1i32) != 0 {
                    ClearDialogWindowAndFrame(0u8, 1u8);
                }
                RunOrScheduleCommand(
                    ((((game).wrapping_add(14)).read()) as u16),
                    1u8,
                    core::ptr::null_mut(),
                );
                ((game).wrapping_add(12)).write(((args).wrapping_offset(4)).read());
                return 0u32;
            }
        }
        let __p3 = (game).wrapping_add(12);
        (__p3).write(((__p3).read()).wrapping_add(1));
        return 0u32;
    }
}
pub(crate) unsafe extern "C" fn Cmd_ShowGameDisplay(game: *mut u8, args: *mut u8) -> u32 {
    unsafe {
        let mut game = game;
        let mut args = args;
        if (ShowGameDisplay()) != 0 {
            RunOrScheduleCommand(
                ((((game).wrapping_add(14)).read()) as u16),
                0u8,
                ((game).wrapping_add(54)).cast::<u8>(),
            );
        }
        return 0u32;
    }
}
pub(crate) unsafe extern "C" fn Cmd_HideGameDisplay(game: *mut u8, args: *mut u8) -> u32 {
    unsafe {
        let mut game = game;
        let mut args = args;
        if (HideGameDisplay()) != 0 {
            RunOrScheduleCommand(
                ((((game).wrapping_add(14)).read()) as u16),
                0u8,
                ((game).wrapping_add(54)).cast::<u8>(),
            );
        }
        return 0u32;
    }
}
pub(crate) unsafe extern "C" fn Cmd_SignalReadyToBegin(game: *mut u8, args: *mut u8) -> u32 {
    unsafe {
        let mut game = game;
        let mut args = args;
        'l1: {
            let __sw1 = ((((game).wrapping_add(12)).read()) as i32);
            if __sw1 == 0i32 {
                Rfu_SetLinkStandbyCallback();
                break 'l1;
            }
            if __sw1 == 1i32 {
                if (IsLinkTaskFinished()) != 0 {
                    PlayNewMapMusic(485u16);
                    RunOrScheduleCommand(7u16, 1u8, core::ptr::null_mut());
                    ((game).wrapping_add(18).cast::<u16>()).write(3u16);
                    ((game).wrapping_add(12)).write(0u8);
                }
                return 0u32;
            }
        }
        let __p2 = (game).wrapping_add(12);
        (__p2).write(((__p2).read()).wrapping_add(1));
        return 0u32;
    }
}
pub(crate) unsafe extern "C" fn Cmd_AskPickBerry(game: *mut u8, args: *mut u8) -> u32 {
    unsafe {
        let mut game = game;
        let mut args = args;
        'l1: {
            let __sw1 = ((((game).wrapping_add(12)).read()) as i32);
            let __matched = __sw1 == 0i32 || __sw1 == 1i32;
            if !__matched {
                let __p2 = (game).wrapping_add(12);
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 0i32 {
                ResetGame(game);
                SetPrintMessageArgs(args, 0u8, 1u8, 0u16, 1u8);
                ((game).wrapping_add(14)).write(7u8);
                RunOrScheduleCommand(3u16, 1u8, core::ptr::null_mut());
                break 'l1;
            }
            if __sw1 == 1i32 {
                ((game).wrapping_add(14)).write(8u8);
                RunOrScheduleCommand(5u16, 1u8, core::ptr::null_mut());
                ((game).wrapping_add(12)).write(2u8);
                break 'l1;
            }
        }
        return 0u32;
    }
}
pub(crate) unsafe extern "C" fn Cmd_GoToBerryPouch(game: *mut u8, args: *mut u8) -> u32 {
    unsafe {
        let mut game = game;
        let mut args = args;
        ((game)
            .wrapping_add(4)
            .cast::<Option<unsafe extern "C" fn(*mut u8, *mut u8) -> u32>>())
        .write(None);
        SetMainCallback2(Some(ChooseBerry));
        return 0u32;
    }
}
pub(crate) unsafe extern "C" fn Cmd_WaitForOthersToPickBerries(
    game: *mut u8,
    args: *mut u8,
) -> u32 {
    unsafe {
        let mut game = game;
        let mut args = args;
        let mut i: u8 = 0u8;
        'l1: {
            let __sw1 = ((((game).wrapping_add(12)).read()) as i32);
            if __sw1 == 0i32 {
                SetPrintMessageArgs(args, 1u8, 0u8, 0u16, 1u8);
                ((game).wrapping_add(14)).write(9u8);
                RunOrScheduleCommand(3u16, 1u8, core::ptr::null_mut());
                return 0u32;
            }
            if __sw1 == 1i32 {
                Rfu_SetLinkStandbyCallback();
                break 'l1;
            }
            if __sw1 == 2i32 {
                if !((IsLinkTaskFinished()) != 0) {
                    return 0u32;
                }
                crate::c::memset(
                    (((game).wrapping_add(66)).cast::<u16>()).cast::<u8>(),
                    0i32,
                    12u32,
                );
                (((game).wrapping_add(66)).cast::<u16>()).write(
                    (((((game).wrapping_add(152)).cast::<u8>()).wrapping_offset(
                        ((((game).wrapping_add(8)).read()) as i32) as isize * 32,
                    ))
                    .wrapping_add(12)
                    .cast::<u16>())
                    .read(),
                );
                SendBlock(
                    0u8,
                    (((game).wrapping_add(66)).cast::<u16>()).cast::<u8>(),
                    2u16,
                );
                break 'l1;
            }
            if __sw1 == 3i32 {
                if !((IsLinkTaskFinished()) != 0) {
                    return 0u32;
                }
                ((game).wrapping_add(16).cast::<u16>()).write(0u16);
                break 'l1;
            }
            if __sw1 == 4i32 {
                if ((GetBlockReceivedStatus()) as i32)
                    != ((((((&raw const sReceivedPlayerBitmasks).cast::<u8>().cast_mut())
                        .cast::<u8>())
                    .wrapping_offset(
                        (((((game).wrapping_add(9)).read()) as i32).wrapping_sub(2i32)) as isize,
                    ))
                    .read()) as i32)
                {
                    return 0u32;
                }
                {
                    i = 0u8;
                    'l2: loop {
                        if !(((i) as i32) < ((((game).wrapping_add(9)).read()) as i32)) {
                            break 'l2;
                        }
                        'l3: {
                            (((((game).wrapping_add(152)).cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize * 32))
                            .wrapping_add(12)
                            .cast::<u16>())
                            .write(
                                ((((&raw mut gBlockRecvBuffer).cast::<u8>())
                                    .wrapping_offset(((i) as i32) as isize * 256))
                                .cast::<u16>())
                                .read(),
                            );
                            if (((((((game).wrapping_add(152)).cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize * 32))
                            .wrapping_add(12)
                            .cast::<u16>())
                            .read()) as i32)
                                > 176i32
                            {
                                (((((game).wrapping_add(152)).cast::<u8>())
                                    .wrapping_offset(((i) as i32) as isize * 32))
                                .wrapping_add(12)
                                .cast::<u16>())
                                .write(0u16);
                            }
                            let __p2 = (game).wrapping_add(24).cast::<i16>();
                            (__p2).write(
                                (((((__p2).read()) as i32).wrapping_add(
                                    (((((&raw mut gBerryCrush_BerryData).cast::<u8>())
                                        .wrapping_offset(
                                            (((((((game).wrapping_add(152)).cast::<u8>())
                                                .wrapping_offset(((i) as i32) as isize * 32))
                                            .wrapping_add(12)
                                            .cast::<u16>())
                                            .read())
                                                as i32)
                                                as isize
                                                * 4,
                                        ))
                                    .read()) as i32),
                                )) as i16),
                            );
                            let __p3 = (game).wrapping_add(28).cast::<i32>();
                            (__p3).write(
                                ((__p3).read()).wrapping_add(
                                    ((((((&raw mut gBerryCrush_BerryData).cast::<u8>())
                                        .wrapping_offset(
                                            (((((((game).wrapping_add(152)).cast::<u8>())
                                                .wrapping_offset(((i) as i32) as isize * 32))
                                            .wrapping_add(12)
                                            .cast::<u16>())
                                            .read())
                                                as i32)
                                                as isize
                                                * 4,
                                        ))
                                    .wrapping_add(2)
                                    .cast::<u16>())
                                    .read()) as i32),
                                ),
                            );
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                ((game).wrapping_add(16).cast::<u16>()).write(0u16);
                ResetBlockReceivedFlags();
                ((game).wrapping_add(32).cast::<i32>()).write(MathUtil_Div32(
                    (((((game).wrapping_add(24).cast::<i16>()).read()) as i32) << 8),
                    8192i32,
                ));
                break 'l1;
            }
            if __sw1 == 5i32 {
                ClearDialogWindowAndFrame(0u8, 1u8);
                RunOrScheduleCommand(10u16, 1u8, core::ptr::null_mut());
                ((game).wrapping_add(18).cast::<u16>()).write(4u16);
                ((game).wrapping_add(12)).write(0u8);
                return 0u32;
            }
        }
        let __p4 = (game).wrapping_add(12);
        (__p4).write(((__p4).read()).wrapping_add(1));
        return 0u32;
    }
}
pub(crate) unsafe extern "C" fn Cmd_DropBerriesIntoCrusher(game: *mut u8, args: *mut u8) -> u32 {
    unsafe {
        let mut game = game;
        let mut args = args;
        'l1: {
            let __sw1 = ((((game).wrapping_add(12)).read()) as i32);
            if __sw1 == 0i32 {
                CreateBerrySprites(game, (game).wrapping_add(312));
                Rfu_SetLinkStandbyCallback();
                break 'l1;
            }
            if __sw1 == 1i32 {
                if !((IsLinkTaskFinished()) != 0) {
                    return 0u32;
                }
                ((game).wrapping_add(312)).write(0u8);
                (((game).wrapping_add(312)).wrapping_add(1)).write(0u8);
                (((game).wrapping_add(312)).wrapping_add(2)).write(0u8);
                (((game).wrapping_add(312)).wrapping_add(3)).write(0u8);
                break 'l1;
            }
            if __sw1 == 2i32 {
                (((((((game).wrapping_add(312)).wrapping_add(56)).cast::<*mut u8>())
                    .wrapping_offset(((((game).wrapping_add(312)).read()) as i32) as isize))
                .read())
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                .write(Some(SpriteCB_DropBerryIntoCrusher));
                crate::c::bf_write(
                    ((((((game).wrapping_add(312)).wrapping_add(56)).cast::<*mut u8>())
                        .wrapping_offset(((((game).wrapping_add(312)).read()) as i32) as isize))
                    .read())
                    .wrapping_add(44),
                    7,
                    1,
                    (0u8) as i32,
                );
                PlaySE(61u16);
                break 'l1;
            }
            if __sw1 == 3i32 {
                if core::mem::transmute::<_, usize>(
                    (((((((game).wrapping_add(312)).wrapping_add(56)).cast::<*mut u8>())
                        .wrapping_offset(((((game).wrapping_add(312)).read()) as i32) as isize))
                    .read())
                    .wrapping_add(28)
                    .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                    .read(),
                ) == (SpriteCB_DropBerryIntoCrusher as *const () as usize)
                {
                    return 0u32;
                }
                (((((game).wrapping_add(312)).wrapping_add(56)).cast::<*mut u8>())
                    .wrapping_offset(((((game).wrapping_add(312)).read()) as i32) as isize))
                .write(core::ptr::null_mut());
                let __p2 = ((game).wrapping_add(312));
                (__p2).write(((__p2).read()).wrapping_add(1));
                Rfu_SetLinkStandbyCallback();
                break 'l1;
            }
            if __sw1 == 4i32 {
                if !((IsLinkTaskFinished()) != 0) {
                    return 0u32;
                }
                if ((((game).wrapping_add(312)).read()) as i32)
                    < ((((game).wrapping_add(9)).read()) as i32)
                {
                    ((game).wrapping_add(12)).write(2u8);
                    return 0u32;
                }
                ((game).wrapping_add(312)).write(0u8);
                break 'l1;
            }
            if __sw1 == 5i32 {
                BerryCrushFreeBerrySpriteGfx(game, (game).wrapping_add(312));
                Rfu_SetLinkStandbyCallback();
                break 'l1;
            }
            if __sw1 == 6i32 {
                if !((IsLinkTaskFinished()) != 0) {
                    return 0u32;
                }
                PlaySE(43u16);
                RunOrScheduleCommand(11u16, 1u8, core::ptr::null_mut());
                ((game).wrapping_add(18).cast::<u16>()).write(5u16);
                ((game).wrapping_add(12)).write(0u8);
                return 0u32;
            }
        }
        let __p3 = (game).wrapping_add(12);
        (__p3).write(((__p3).read()).wrapping_add(1));
        return 0u32;
    }
}
pub(crate) unsafe extern "C" fn Cmd_DropLid(game: *mut u8, args: *mut u8) -> u32 {
    unsafe {
        let mut game = game;
        let mut args = args;
        'l1: {
            let __sw1 = ((((game).wrapping_add(12)).read()) as i32);
            if __sw1 == 0i32 {
                let __p2 = (game).wrapping_add(42).cast::<i16>();
                (__p2).write((((((__p2).read()) as i32).wrapping_add(4i32)) as i16));
                if ((((game).wrapping_add(42).cast::<i16>()).read()) as i32) < 0i32 {
                    return 0u32;
                }
                ((game).wrapping_add(42).cast::<i16>()).write(0i16);
                (((game).wrapping_add(312)).wrapping_add(1)).write(4u8);
                ((game).wrapping_add(312)).write(0u8);
                (((game).wrapping_add(312)).wrapping_add(2)).write(
                    (((((((&raw const sIntroOutroVibrationData)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>())
                    .wrapping_offset(
                        (((((game).wrapping_add(312)).wrapping_add(1)).read()) as i32) as isize * 7,
                    ))
                    .cast::<i8>())
                    .read()) as u8),
                );
                PlaySE(214u16);
                break 'l1;
            }
            if __sw1 == 1i32 {
                ((game).wrapping_add(44).cast::<i16>()).write(
                    ((((((((&raw const sIntroOutroVibrationData)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>())
                    .wrapping_offset(
                        (((((game).wrapping_add(312)).wrapping_add(1)).read()) as i32) as isize * 7,
                    ))
                    .cast::<i8>())
                    .wrapping_offset(((((game).wrapping_add(312)).read()) as i32) as isize))
                    .read()) as i16),
                );
                SetGpuReg(
                    18u8,
                    ((((((game).wrapping_add(44).cast::<i16>()).read()) as i32).wrapping_neg())
                        as u16),
                );
                SetGpuReg(
                    26u8,
                    ((((((game).wrapping_add(44).cast::<i16>()).read()) as i32).wrapping_neg())
                        as u16),
                );
                SetGpuReg(
                    30u8,
                    ((((((game).wrapping_add(44).cast::<i16>()).read()) as i32).wrapping_neg())
                        as u16),
                );
                let __p3 = ((game).wrapping_add(312));
                (__p3).write(((__p3).read()).wrapping_add(1));
                if ((((game).wrapping_add(312)).read()) as i32)
                    < (((((game).wrapping_add(312)).wrapping_add(2)).read()) as i32)
                {
                    return 0u32;
                }
                if (((((game).wrapping_add(312)).wrapping_add(1)).read()) as i32) == 0i32 {
                    break 'l1;
                }
                let __p4 = ((game).wrapping_add(312)).wrapping_add(1);
                (__p4).write(((__p4).read()).wrapping_sub(1));
                (((game).wrapping_add(312)).wrapping_add(2)).write(
                    (((((((&raw const sIntroOutroVibrationData)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>())
                    .wrapping_offset(
                        (((((game).wrapping_add(312)).wrapping_add(1)).read()) as i32) as isize * 7,
                    ))
                    .cast::<i8>())
                    .read()) as u8),
                );
                ((game).wrapping_add(312)).write(0u8);
                return 0u32;
            }
            if __sw1 == 2i32 {
                ((game).wrapping_add(44).cast::<i16>()).write(0i16);
                SetGpuReg(18u8, 0u16);
                SetGpuReg(26u8, 0u16);
                SetGpuReg(30u8, 0u16);
                Rfu_SetLinkStandbyCallback();
                break 'l1;
            }
            if __sw1 == 3i32 {
                if !((IsLinkTaskFinished()) != 0) {
                    return 0u32;
                }
                RunOrScheduleCommand(12u16, 1u8, core::ptr::null_mut());
                ((game).wrapping_add(18).cast::<u16>()).write(6u16);
                ((game).wrapping_add(12)).write(0u8);
                return 0u32;
            }
        }
        let __p5 = (game).wrapping_add(12);
        (__p5).write(((__p5).read()).wrapping_add(1));
        return 0u32;
    }
}
pub(crate) unsafe extern "C" fn Cmd_Countdown(game: *mut u8, args: *mut u8) -> u32 {
    unsafe {
        let mut game = game;
        let mut args = args;
        'l1: {
            let __sw1 = ((((game).wrapping_add(12)).read()) as i32);
            let mut __fall = false;
            if __sw1 == 1i32 {
                __fall = true;
                if !((IsLinkTaskFinished()) != 0) {
                    return 0u32;
                }
                StartMinigameCountdown(4096u16, 4096u16, 120i16, 80i16, 0u8);
                break 'l1;
            }
            if __sw1 == 2i32 {
                __fall = true;
                if (IsMinigameCountdownRunning()) != 0 {
                    return 0u32;
                }
            }
            if __fall || __sw1 == 0i32 {
                __fall = true;
                Rfu_SetLinkStandbyCallback();
                break 'l1;
            }
            if __sw1 == 3i32 {
                __fall = true;
                if !((IsLinkTaskFinished()) != 0) {
                    return 0u32;
                }
                ((game).wrapping_add(312)).write(0u8);
                (((game).wrapping_add(312)).wrapping_add(1)).write(0u8);
                (((game).wrapping_add(312)).wrapping_add(2)).write(0u8);
                (((game).wrapping_add(312)).wrapping_add(3)).write(0u8);
                ((game).wrapping_add(16).cast::<u16>()).write(0u16);
                if ((((game).wrapping_add(8)).read()) as i32) == 0i32 {
                    RunOrScheduleCommand(13u16, 1u8, core::ptr::null_mut());
                } else {
                    RunOrScheduleCommand(14u16, 1u8, core::ptr::null_mut());
                }
                ((game).wrapping_add(18).cast::<u16>()).write(7u16);
                ((game).wrapping_add(12)).write(0u8);
                return 0u32;
            }
        }
        let __p2 = (game).wrapping_add(12);
        (__p2).write(((__p2).read()).wrapping_add(1));
        return 0u32;
    }
}
pub(crate) unsafe extern "C" fn HandlePartnerInput(game: *mut u8) {
    unsafe {
        let mut game = game;
        let mut numPlayersPressed: u8 = 0u8;
        let mut i: u8 = 0u8;
        let mut timeDiff: u16 = 0u16;
        let mut temp: i32 = 0i32;
        let mut linkState: *mut u8 = core::ptr::null_mut();
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < ((((game).wrapping_add(9)).read()) as i32)) {
                    break 'l1;
                }
                'l2: {
                    linkState = ((((&raw mut gRecvCmds).cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 16))
                    .cast::<u16>())
                    .cast::<u8>();
                    if (((((linkState).cast::<u16>()).read()) as i32) & 65280i32) != 12032i32 {
                        break 'l2;
                    }
                    if ((((linkState).wrapping_add(2).cast::<u16>()).read()) as i32) != 2i32 {
                        break 'l2;
                    }
                    if (crate::c::bf_read((linkState).wrapping_add(4), 2, 1, false) as u8) != 0 {
                        crate::c::bf_write(
                            ((game).wrapping_add(92)).wrapping_add(2),
                            3,
                            5,
                            ((((crate::c::bf_read(
                                ((game).wrapping_add(92)).wrapping_add(2),
                                3,
                                5,
                                false,
                            ) as u8) as i32)
                                | ((((((&raw const sBitTable).cast::<u8>().cast_mut())
                                    .cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize))
                                .read()) as i32)) as u8) as i32,
                        );
                        (((((game).wrapping_add(152)).cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 32))
                        .wrapping_add(29))
                        .write(1u8);
                        let __p1 = ((((game).wrapping_add(152)).cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 32))
                        .wrapping_add(22)
                        .cast::<u16>();
                        (__p1).write(((__p1).read()).wrapping_add(1));
                        numPlayersPressed = (numPlayersPressed).wrapping_add(1);
                        timeDiff = ((((((game).wrapping_add(40).cast::<u16>()).read()) as i32)
                            .wrapping_sub(
                                (((((((game).wrapping_add(152)).cast::<u8>())
                                    .wrapping_offset(((i) as i32) as isize * 32))
                                .wrapping_add(14)
                                .cast::<u16>())
                                .read()) as i32),
                            )) as u16);
                        if (((timeDiff) as i32)
                            >= (((((((game).wrapping_add(152)).cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize * 32))
                            .wrapping_add(18)
                            .cast::<u16>())
                            .read()) as i32)
                                .wrapping_sub(1i32))
                            && (((timeDiff) as i32)
                                <= (((((((game).wrapping_add(152)).cast::<u8>())
                                    .wrapping_offset(((i) as i32) as isize * 32))
                                .wrapping_add(18)
                                .cast::<u16>())
                                .read()) as i32)
                                    .wrapping_add(1i32))
                        {
                            let __p2 = ((((game).wrapping_add(152)).cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize * 32))
                            .wrapping_add(16)
                            .cast::<u16>();
                            (__p2).write(((__p2).read()).wrapping_add(1));
                            (((((game).wrapping_add(152)).cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize * 32))
                            .wrapping_add(18)
                            .cast::<u16>())
                            .write(timeDiff);
                            if (((((((game).wrapping_add(152)).cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize * 32))
                            .wrapping_add(16)
                            .cast::<u16>())
                            .read()) as i32)
                                > (((((((game).wrapping_add(152)).cast::<u8>())
                                    .wrapping_offset(((i) as i32) as isize * 32))
                                .wrapping_add(20)
                                .cast::<u16>())
                                .read()) as i32)
                            {
                                (((((game).wrapping_add(152)).cast::<u8>())
                                    .wrapping_offset(((i) as i32) as isize * 32))
                                .wrapping_add(20)
                                .cast::<u16>())
                                .write(
                                    (((((game).wrapping_add(152)).cast::<u8>())
                                        .wrapping_offset(((i) as i32) as isize * 32))
                                    .wrapping_add(16)
                                    .cast::<u16>())
                                    .read(),
                                );
                            }
                        } else {
                            (((((game).wrapping_add(152)).cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize * 32))
                            .wrapping_add(16)
                            .cast::<u16>())
                            .write(0u16);
                            (((((game).wrapping_add(152)).cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize * 32))
                            .wrapping_add(18)
                            .cast::<u16>())
                            .write(timeDiff);
                        }
                        (((((game).wrapping_add(152)).cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 32))
                        .wrapping_add(14)
                        .cast::<u16>())
                        .write(((game).wrapping_add(40).cast::<u16>()).read());
                        let __p3 = ((((game).wrapping_add(152)).cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 32))
                        .wrapping_add(28);
                        (__p3).write(((__p3).read()).wrapping_add(1));
                        if (((((((game).wrapping_add(152)).cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 32))
                        .wrapping_add(28))
                        .read()) as i32)
                            > 2i32
                        {
                            (((((game).wrapping_add(152)).cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize * 32))
                            .wrapping_add(28))
                            .write(0u8);
                        }
                    } else {
                        (((((game).wrapping_add(152)).cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 32))
                        .wrapping_add(29))
                        .write(0u8);
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        if ((numPlayersPressed) as i32) > 1i32 {
            {
                i = 0u8;
                'l3: loop {
                    if !(((i) as i32) < ((((game).wrapping_add(9)).read()) as i32)) {
                        break 'l3;
                    }
                    'l4: {
                        if (((((((game).wrapping_add(152)).cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 32))
                        .wrapping_add(29))
                        .read()) as i32)
                            == 0i32
                        {
                            break 'l4;
                        }
                        let __p4 = ((((game).wrapping_add(152)).cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 32))
                        .wrapping_add(29);
                        (__p4).write((((((__p4).read()) as i32) | 2i32) as u8));
                        let __p5 = ((((game).wrapping_add(152)).cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 32))
                        .wrapping_add(24)
                        .cast::<u16>();
                        (__p5).write(((__p5).read()).wrapping_add(1));
                    }
                    i = (i).wrapping_add(1);
                }
            }
        }
        if ((numPlayersPressed) as i32) == 0i32 {
            return;
        }
        let __p6 = (game).wrapping_add(46).cast::<i16>();
        (__p6)
            .write((((((__p6).read()) as i32).wrapping_add(((numPlayersPressed) as i32))) as i16));
        numPlayersPressed = ((((numPlayersPressed) as i32).wrapping_add(
            ((((((&raw const sSyncPressBonus).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset((((numPlayersPressed) as i32).wrapping_sub(1i32)) as isize))
            .read()) as i32),
        )) as u8);
        let __p7 = (game).wrapping_add(52).cast::<i16>();
        (__p7)
            .write((((((__p7).read()) as i32).wrapping_add(((numPlayersPressed) as i32))) as i16));
        let __p8 = (game).wrapping_add(26).cast::<i16>();
        (__p8)
            .write((((((__p8).read()) as i32).wrapping_add(((numPlayersPressed) as i32))) as i16));
        if ((((game).wrapping_add(24).cast::<i16>()).read()) as i32)
            .wrapping_sub(((((game).wrapping_add(26).cast::<i16>()).read()) as i32))
            > 0i32
        {
            temp = ((((game).wrapping_add(26).cast::<i16>()).read()) as i32);
            temp = (temp << 8);
            temp = MathUtil_Div32(temp, ((game).wrapping_add(32).cast::<i32>()).read());
            temp = (temp >> 8);
            ((game).wrapping_add(36)).write(((temp) as u8));
            return;
        }
        ((game).wrapping_add(36)).write(32u8);
        crate::c::bf_write(
            ((game).wrapping_add(92)).wrapping_add(2),
            0,
            1,
            (1u8) as i32,
        );
    }
}
pub(crate) unsafe extern "C" fn UpdateLeaderGameState(game: *mut u8) {
    unsafe {
        let mut game = game;
        let mut numPlayersPressed: u8 = 0u8;
        let mut flags: u16 = 0u16;
        let mut temp: u16 = 0u16;
        let mut i: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < ((((game).wrapping_add(9)).read()) as i32)) {
                    break 'l1;
                }
                'l2: {
                    if (((((((game).wrapping_add(152)).cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 32))
                    .wrapping_add(29))
                    .read()) as i32)
                        != 0i32
                    {
                        numPlayersPressed = (numPlayersPressed).wrapping_add(1);
                        flags = (((((((((game).wrapping_add(152)).cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 32))
                        .wrapping_add(28))
                        .read()) as i32)
                            .wrapping_add(1i32)) as u16);
                        if ((((((((game).wrapping_add(152)).cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 32))
                        .wrapping_add(29))
                        .read()) as i32)
                            & 2i32)
                            != 0
                        {
                            flags = ((((flags) as i32) | 4i32) as u16);
                        }
                        flags = ((crate::c::shl_i32(
                            ((flags) as i32),
                            (((3i32).wrapping_mul(((i) as i32))) as u32),
                        )) as u16);
                        let __p1 = ((game).wrapping_add(92)).wrapping_add(8).cast::<u16>();
                        (__p1).write((((((__p1).read()) as i32) | ((flags) as i32)) as u16));
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        temp = ((((game).wrapping_add(36)).read()) as u16);
        (((game).wrapping_add(92)).wrapping_add(4).cast::<u16>()).write(temp);
        if ((numPlayersPressed) as i32) == 0i32 {
            if ((((game).wrapping_add(312)).wrapping_add(3)).read()) != 0 {
                let __p2 = ((game).wrapping_add(312));
                (__p2).write(((__p2).read()).wrapping_add(1));
            }
        } else {
            if ((((game).wrapping_add(312)).wrapping_add(3)).read()) != 0 {
                if ((numPlayersPressed) as i32)
                    != (((((game).wrapping_add(312)).wrapping_add(1)).read()) as i32)
                {
                    (((game).wrapping_add(312)).wrapping_add(1))
                        .write(((((numPlayersPressed) as i32).wrapping_sub(1i32)) as u8));
                    (((game).wrapping_add(312)).wrapping_add(2)).write(
                        (((((&raw const sVibrationData).cast::<u8>().cast_mut()).cast::<u8>())
                            .wrapping_offset(
                                (((numPlayersPressed) as i32).wrapping_sub(1i32)) as isize * 4,
                            ))
                        .cast::<u8>())
                        .read(),
                    );
                } else {
                    let __p3 = ((game).wrapping_add(312));
                    (__p3).write(((__p3).read()).wrapping_add(1));
                }
            } else {
                ((game).wrapping_add(312)).write(0u8);
                (((game).wrapping_add(312)).wrapping_add(1))
                    .write(((((numPlayersPressed) as i32).wrapping_sub(1i32)) as u8));
                (((game).wrapping_add(312)).wrapping_add(2)).write(
                    (((((&raw const sVibrationData).cast::<u8>().cast_mut()).cast::<u8>())
                        .wrapping_offset(
                            (((numPlayersPressed) as i32).wrapping_sub(1i32)) as isize * 4,
                        ))
                    .cast::<u8>())
                    .read(),
                );
                (((game).wrapping_add(312)).wrapping_add(3)).write(1u8);
            }
        }
        if ((((game).wrapping_add(312)).wrapping_add(3)).read()) != 0 {
            if ((((game).wrapping_add(312)).read()) as i32)
                >= (((((game).wrapping_add(312)).wrapping_add(2)).read()) as i32)
            {
                ((game).wrapping_add(312)).write(0u8);
                (((game).wrapping_add(312)).wrapping_add(1)).write(0u8);
                (((game).wrapping_add(312)).wrapping_add(2)).write(0u8);
                (((game).wrapping_add(312)).wrapping_add(3)).write(0u8);
                temp = 0u16;
            } else {
                temp = ((((((((&raw const sVibrationData).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset(
                        (((((game).wrapping_add(312)).wrapping_add(1)).read()) as i32) as isize * 4,
                    ))
                .cast::<u8>())
                .wrapping_offset(
                    (((((game).wrapping_add(312)).read()) as i32).wrapping_add(1i32)) as isize,
                ))
                .read()) as u16);
            }
            (((game).wrapping_add(92)).wrapping_add(3).cast::<i8>()).write((((temp) as u8) as i8));
        } else {
            (((game).wrapping_add(92)).wrapping_add(3).cast::<i8>()).write(0i8);
        }
        (((game).wrapping_add(92)).wrapping_add(6).cast::<u16>())
            .write(((game).wrapping_add(38).cast::<u16>()).read());
    }
}
pub(crate) unsafe extern "C" fn HandlePlayerInput(game: *mut u8) {
    unsafe {
        let mut game = game;
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(46)
            .cast::<u16>())
        .read()) as i32)
            & 1i32)
            != 0
        {
            crate::c::bf_write(
                ((game).wrapping_add(92)).wrapping_add(2),
                2,
                1,
                (1u8) as i32,
            );
        }
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(44)
            .cast::<u16>())
        .read()) as i32)
            & 1i32)
            != 0
        {
            if (((((((game).wrapping_add(152)).cast::<u8>())
                .wrapping_offset(((((game).wrapping_add(8)).read()) as i32) as isize * 32))
            .wrapping_add(26)
            .cast::<u16>())
            .read()) as i32)
                < ((((game).wrapping_add(40).cast::<u16>()).read()) as i32)
            {
                let __p1 = ((((game).wrapping_add(152)).cast::<u8>())
                    .wrapping_offset(((((game).wrapping_add(8)).read()) as i32) as isize * 32))
                .wrapping_add(26)
                .cast::<u16>();
                (__p1).write(((__p1).read()).wrapping_add(1));
            }
        }
        if (((((game).wrapping_add(8)).read()) as i32) != 0i32)
            && (!((crate::c::bf_read(((game).wrapping_add(92)).wrapping_add(2), 2, 1, false)
                as u8)
                != 0))
        {
            return;
        }
        (((game).wrapping_add(92)).cast::<u16>()).write(2u16);
        if crate::c::rem_i32(
            ((((game).wrapping_add(40).cast::<u16>()).read()) as i32),
            30i32,
        ) == 0i32
        {
            if ((((game).wrapping_add(46).cast::<i16>()).read()) as i32)
                > ((((((&raw const sBigSparkleThresholds).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset(
                        (((((game).wrapping_add(9)).read()) as i32).wrapping_sub(2i32)) as isize,
                    ))
                .read()) as i32)
            {
                let __p2 = (game).wrapping_add(48).cast::<i16>();
                (__p2).write(((__p2).read()).wrapping_add(1));
                crate::c::bf_write((game).wrapping_add(37), 4, 1, (1u8) as i32);
            } else {
                crate::c::bf_write((game).wrapping_add(37), 4, 1, (0u8) as i32);
            }
            ((game).wrapping_add(46).cast::<i16>()).write(0i16);
            let __p3 = (game).wrapping_add(50).cast::<i16>();
            (__p3).write(((__p3).read()).wrapping_add(1));
        }
        if crate::c::rem_i32(
            ((((game).wrapping_add(40).cast::<u16>()).read()) as i32),
            15i32,
        ) == 0i32
        {
            if ((((game).wrapping_add(52).cast::<i16>()).read()) as i32)
                < (((((((&raw const sSparkleThresholds).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset(
                        (((((game).wrapping_add(9)).read()) as i32).wrapping_sub(2i32)) as isize
                            * 4,
                    ))
                .cast::<u8>())
                .read()) as i32)
            {
                crate::c::bf_write((game).wrapping_add(37), 5, 3, (0u8) as i32);
            } else {
                if ((((game).wrapping_add(52).cast::<i16>()).read()) as i32)
                    < ((((((((&raw const sSparkleThresholds).cast::<u8>().cast_mut())
                        .cast::<u8>())
                    .wrapping_offset(
                        (((((game).wrapping_add(9)).read()) as i32).wrapping_sub(2i32)) as isize
                            * 4,
                    ))
                    .cast::<u8>())
                    .wrapping_offset(1))
                    .read()) as i32)
                {
                    crate::c::bf_write((game).wrapping_add(37), 5, 3, (1u8) as i32);
                } else {
                    if ((((game).wrapping_add(52).cast::<i16>()).read()) as i32)
                        < ((((((((&raw const sSparkleThresholds).cast::<u8>().cast_mut())
                            .cast::<u8>())
                        .wrapping_offset(
                            (((((game).wrapping_add(9)).read()) as i32).wrapping_sub(2i32))
                                as isize
                                * 4,
                        ))
                        .cast::<u8>())
                        .wrapping_offset(2))
                        .read()) as i32)
                    {
                        ((game).wrapping_add(52).cast::<i16>()).write(2i16);
                    } else {
                        if ((((game).wrapping_add(52).cast::<i16>()).read()) as i32)
                            < ((((((((&raw const sSparkleThresholds).cast::<u8>().cast_mut())
                                .cast::<u8>())
                            .wrapping_offset(
                                (((((game).wrapping_add(9)).read()) as i32).wrapping_sub(2i32))
                                    as isize
                                    * 4,
                            ))
                            .cast::<u8>())
                            .wrapping_offset(3))
                            .read()) as i32)
                        {
                            ((game).wrapping_add(52).cast::<i16>()).write(3i16);
                        } else {
                            crate::c::bf_write((game).wrapping_add(37), 5, 3, (4u8) as i32);
                        }
                    }
                }
            }
            ((game).wrapping_add(52).cast::<i16>()).write(0i16);
        } else {
            let __p4 = (game).wrapping_add(16).cast::<u16>();
            (__p4).write(((__p4).read()).wrapping_add(1));
            if ((((game).wrapping_add(16).cast::<u16>()).read()) as i32) > 60i32 {
                if ((((game).wrapping_add(16).cast::<u16>()).read()) as i32) > 70i32 {
                    ClearRecvCommands();
                    ((game).wrapping_add(16).cast::<u16>()).write(0u16);
                } else {
                    if ((crate::c::bf_read(((game).wrapping_add(92)).wrapping_add(2), 3, 5, false)
                        as u8) as i32)
                        == 0i32
                    {
                        ClearRecvCommands();
                        ((game).wrapping_add(16).cast::<u16>()).write(0u16);
                    }
                }
            }
        }
        if ((((game).wrapping_add(40).cast::<u16>()).read()) as i32) >= 36000i32 {
            crate::c::bf_write(
                ((game).wrapping_add(92)).wrapping_add(2),
                0,
                1,
                (1u8) as i32,
            );
        }
        crate::c::bf_write(
            ((game).wrapping_add(92)).wrapping_add(2),
            1,
            1,
            (crate::c::bf_read((game).wrapping_add(37), 4, 1, false) as u8) as i32,
        );
        (((game).wrapping_add(92)).wrapping_add(10).cast::<u16>())
            .write(((crate::c::bf_read((game).wrapping_add(37), 5, 3, false) as u8) as u16));
        crate::c::memcpy(
            (((game).wrapping_add(66)).cast::<u16>()).cast::<u8>(),
            (game).wrapping_add(92),
            12u32,
        );
        Rfu_SendPacket((((game).wrapping_add(66)).cast::<u16>()).cast::<u8>());
    }
}
pub(crate) unsafe extern "C" fn RecvLinkData(game: *mut u8) {
    unsafe {
        let mut game = game;
        let mut i: u8 = 0u8;
        let mut linkState: *mut u8 = core::ptr::null_mut();
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < ((((game).wrapping_add(9)).read()) as i32)) {
                    break 'l1;
                }
                'l2: {
                    (((((game).wrapping_add(152)).cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 32))
                    .wrapping_add(29))
                    .write(0u8);
                }
                i = (i).wrapping_add(1);
            }
        }
        if ((((((&raw mut gRecvCmds).cast::<u8>()).cast::<u16>()).read()) as i32) & 65280i32)
            != 12032i32
        {
            crate::c::bf_write((game).wrapping_add(37), 2, 1, (0u8) as i32);
            return;
        }
        if ((((((&raw mut gRecvCmds).cast::<u8>()).cast::<u16>()).wrapping_offset(1)).read())
            as i32)
            != 2i32
        {
            crate::c::bf_write((game).wrapping_add(37), 2, 1, (0u8) as i32);
            return;
        }
        crate::c::memcpy(
            (((game).wrapping_add(78)).cast::<u16>()).cast::<u8>(),
            (((&raw mut gRecvCmds).cast::<u8>()).cast::<u16>()).cast::<u8>(),
            14u32,
        );
        linkState = (((game).wrapping_add(78)).cast::<u16>()).cast::<u8>();
        ((game).wrapping_add(42).cast::<i16>())
            .write(((((linkState).wrapping_add(6).cast::<u16>()).read()) as i16));
        ((game).wrapping_add(44).cast::<i16>())
            .write(((((linkState).wrapping_add(5).cast::<i8>()).read()) as i16));
        ((game).wrapping_add(40).cast::<u16>())
            .write(((linkState).wrapping_add(8).cast::<u16>()).read());
        UpdateInputEffects(game, (game).wrapping_add(312));
        if (crate::c::bf_read((linkState).wrapping_add(4), 0, 1, false) as u8) != 0 {
            crate::c::bf_write((game).wrapping_add(37), 3, 1, (1u8) as i32);
        }
    }
}
pub(crate) unsafe extern "C" fn Cmd_PlayGame_Leader(game: *mut u8, args: *mut u8) -> u32 {
    unsafe {
        let mut game = game;
        let mut args = args;
        crate::c::memset((game).wrapping_add(92), 0i32, 12u32);
        crate::c::memset(
            (((game).wrapping_add(78)).cast::<u16>()).cast::<u8>(),
            0i32,
            14u32,
        );
        RecvLinkData(game);
        SetGpuReg(
            18u8,
            ((((((game).wrapping_add(44).cast::<i16>()).read()) as i32).wrapping_neg()) as u16),
        );
        SetGpuReg(
            26u8,
            ((((((game).wrapping_add(44).cast::<i16>()).read()) as i32).wrapping_neg()) as u16),
        );
        SetGpuReg(
            30u8,
            ((((((game).wrapping_add(44).cast::<i16>()).read()) as i32).wrapping_neg()) as u16),
        );
        if (crate::c::bf_read((game).wrapping_add(37), 3, 1, false) as u8) != 0 {
            if ((((game).wrapping_add(40).cast::<u16>()).read()) as i32) >= 36000i32 {
                ((game).wrapping_add(40).cast::<u16>()).write(36000u16);
                RunOrScheduleCommand(16u16, 1u8, core::ptr::null_mut());
            } else {
                RunOrScheduleCommand(15u16, 1u8, core::ptr::null_mut());
            }
            ((game).wrapping_add(16).cast::<u16>()).write(0u16);
            ((game).wrapping_add(12)).write(0u8);
            return 0u32;
        } else {
            let __p1 = (game).wrapping_add(38).cast::<u16>();
            (__p1).write(((__p1).read()).wrapping_add(1));
            HandlePartnerInput(game);
            UpdateLeaderGameState(game);
            HandlePlayerInput(game);
            return 0u32;
        }
        #[allow(unreachable_code)]
        {
            return 0u32;
        }
    }
}
pub(crate) unsafe extern "C" fn Cmd_PlayGame_Member(game: *mut u8, args: *mut u8) -> u32 {
    unsafe {
        let mut game = game;
        let mut args = args;
        crate::c::memset((game).wrapping_add(92), 0i32, 12u32);
        crate::c::memset(
            (((game).wrapping_add(78)).cast::<u16>()).cast::<u8>(),
            0i32,
            14u32,
        );
        RecvLinkData(game);
        SetGpuReg(
            18u8,
            ((((((game).wrapping_add(44).cast::<i16>()).read()) as i32).wrapping_neg()) as u16),
        );
        SetGpuReg(
            26u8,
            ((((((game).wrapping_add(44).cast::<i16>()).read()) as i32).wrapping_neg()) as u16),
        );
        SetGpuReg(
            30u8,
            ((((((game).wrapping_add(44).cast::<i16>()).read()) as i32).wrapping_neg()) as u16),
        );
        if (crate::c::bf_read((game).wrapping_add(37), 3, 1, false) as u8) != 0 {
            if ((((game).wrapping_add(40).cast::<u16>()).read()) as i32) >= 36000i32 {
                ((game).wrapping_add(40).cast::<u16>()).write(36000u16);
                RunOrScheduleCommand(16u16, 1u8, core::ptr::null_mut());
            } else {
                RunOrScheduleCommand(15u16, 1u8, core::ptr::null_mut());
            }
            ((game).wrapping_add(16).cast::<u16>()).write(0u16);
            ((game).wrapping_add(12)).write(0u8);
            return 0u32;
        } else {
            HandlePlayerInput(game);
            return 0u32;
        }
        #[allow(unreachable_code)]
        {
            return 0u32;
        }
    }
}
pub(crate) unsafe extern "C" fn Cmd_FinishGame(game: *mut u8, args: *mut u8) -> u32 {
    unsafe {
        let mut game = game;
        let mut args = args;
        'l1: {
            let __sw1 = ((((game).wrapping_add(12)).read()) as i32);
            if __sw1 == 0i32 {
                ((game).wrapping_add(18).cast::<u16>()).write(8u16);
                PlaySE(214u16);
                BlendPalettes(4294967295u32, 8u8, 1023u16);
                ((game).wrapping_add(312)).write(2u8);
                break 'l1;
            }
            if __sw1 == 1i32 {
                if (({
                    let __p2 = ((game).wrapping_add(312));
                    let __t3 = ((__p2).read()).wrapping_sub(1);
                    (__p2).write(__t3);
                    __t3
                }) as i32)
                    != 255i32
                {
                    return 0u32;
                }
                BlendPalettes(4294967295u32, 0u8, 1023u16);
                (((game).wrapping_add(312)).wrapping_add(1)).write(4u8);
                ((game).wrapping_add(312)).write(0u8);
                (((game).wrapping_add(312)).wrapping_add(2)).write(
                    (((((((&raw const sIntroOutroVibrationData)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>())
                    .wrapping_offset(
                        (((((game).wrapping_add(312)).wrapping_add(1)).read()) as i32) as isize * 7,
                    ))
                    .cast::<i8>())
                    .read()) as u8),
                );
                break 'l1;
            }
            if __sw1 == 2i32 {
                ((game).wrapping_add(44).cast::<i16>()).write(
                    ((((((((&raw const sIntroOutroVibrationData)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>())
                    .wrapping_offset(
                        (((((game).wrapping_add(312)).wrapping_add(1)).read()) as i32) as isize * 7,
                    ))
                    .cast::<i8>())
                    .wrapping_offset(((((game).wrapping_add(312)).read()) as i32) as isize))
                    .read()) as i16),
                );
                SetGpuReg(
                    18u8,
                    ((((((game).wrapping_add(44).cast::<i16>()).read()) as i32).wrapping_neg())
                        as u16),
                );
                SetGpuReg(
                    26u8,
                    ((((((game).wrapping_add(44).cast::<i16>()).read()) as i32).wrapping_neg())
                        as u16),
                );
                SetGpuReg(
                    30u8,
                    ((((((game).wrapping_add(44).cast::<i16>()).read()) as i32).wrapping_neg())
                        as u16),
                );
                if (({
                    let __p4 = ((game).wrapping_add(312));
                    let __t5 = ((__p4).read()).wrapping_add(1);
                    (__p4).write(__t5);
                    __t5
                }) as i32)
                    < (((((game).wrapping_add(312)).wrapping_add(2)).read()) as i32)
                {
                    return 0u32;
                }
                if (((((game).wrapping_add(312)).wrapping_add(1)).read()) as i32) != 0i32 {
                    let __p6 = ((game).wrapping_add(312)).wrapping_add(1);
                    (__p6).write(((__p6).read()).wrapping_sub(1));
                    (((game).wrapping_add(312)).wrapping_add(2)).write(
                        (((((((&raw const sIntroOutroVibrationData)
                            .cast::<u8>()
                            .cast_mut())
                        .cast::<u8>())
                        .wrapping_offset(
                            (((((game).wrapping_add(312)).wrapping_add(1)).read()) as i32) as isize
                                * 7,
                        ))
                        .cast::<i8>())
                        .read()) as u8),
                    );
                    ((game).wrapping_add(312)).write(0u8);
                    return 0u32;
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                ((game).wrapping_add(44).cast::<i16>()).write(0i16);
                SetGpuReg(18u8, 0u16);
                SetGpuReg(26u8, 0u16);
                SetGpuReg(30u8, 0u16);
                break 'l1;
            }
            if __sw1 == 4i32 {
                if !((AreEffectsFinished(game, (game).wrapping_add(312))) != 0) {
                    return 0u32;
                }
                Rfu_SetLinkStandbyCallback();
                ((game).wrapping_add(16).cast::<u16>()).write(0u16);
                break 'l1;
            }
            if __sw1 == 5i32 {
                if !((IsLinkTaskFinished()) != 0) {
                    return 0u32;
                }
                RunOrScheduleCommand(17u16, 1u8, core::ptr::null_mut());
                ((game).wrapping_add(16).cast::<u16>()).write(0u16);
                ((game).wrapping_add(12)).write(0u8);
                return 0u32;
            }
        }
        let __p7 = (game).wrapping_add(12);
        (__p7).write(((__p7).read()).wrapping_add(1));
        return 0u32;
    }
}
pub(crate) unsafe extern "C" fn Cmd_HandleTimeUp(game: *mut u8, args: *mut u8) -> u32 {
    unsafe {
        let mut game = game;
        let mut args = args;
        'l1: {
            let __sw1 = ((((game).wrapping_add(12)).read()) as i32);
            if __sw1 == 0i32 {
                ((game).wrapping_add(18).cast::<u16>()).write(9u16);
                PlaySE(32u16);
                BlendPalettes(4294967295u32, 8u8, 31u16);
                ((game).wrapping_add(312)).write(4u8);
                break 'l1;
            }
            if __sw1 == 1i32 {
                if (({
                    let __p2 = ((game).wrapping_add(312));
                    let __t3 = ((__p2).read()).wrapping_sub(1);
                    (__p2).write(__t3);
                    __t3
                }) as i32)
                    != 255i32
                {
                    return 0u32;
                }
                BlendPalettes(4294967295u32, 0u8, 31u16);
                ((game).wrapping_add(312)).write(0u8);
                break 'l1;
            }
            if __sw1 == 2i32 {
                if !((AreEffectsFinished(game, (game).wrapping_add(312))) != 0) {
                    return 0u32;
                }
                Rfu_SetLinkStandbyCallback();
                ((game).wrapping_add(16).cast::<u16>()).write(0u16);
                SetGpuReg(18u8, 0u16);
                SetGpuReg(26u8, 0u16);
                SetGpuReg(30u8, 0u16);
                break 'l1;
            }
            if __sw1 == 3i32 {
                if !((IsLinkTaskFinished()) != 0) {
                    return 0u32;
                }
                ConvertIntToDecimalStringN(
                    (&raw mut gStringVar1).cast::<u8>(),
                    ((game).wrapping_add(28).cast::<i32>()).read(),
                    0i32,
                    6u8,
                );
                SetPrintMessageArgs(args, 7u8, 1u8, 0u16, 0u8);
                ((game).wrapping_add(14)).write(19u8);
                RunOrScheduleCommand(3u16, 1u8, core::ptr::null_mut());
                ((game).wrapping_add(16).cast::<u16>()).write(0u16);
                ((game).wrapping_add(12)).write(0u8);
                return 0u32;
            }
        }
        let __p4 = (game).wrapping_add(12);
        (__p4).write(((__p4).read()).wrapping_add(1));
        return 0u32;
    }
}
pub(crate) unsafe extern "C" fn Cmd_TabulateResults(game: *mut u8, args: *mut u8) -> u32 {
    unsafe {
        let mut game = game;
        let mut args = args;
        let mut i: u8 = 0u8;
        let mut j: u8 = 0u8;
        let mut tempPlayerId: u8 = 0u8;
        let mut temp1: i32 = 0i32;
        let mut temp2: i32 = 0i32;
        let mut tempStat: u16 = 0u16;
        'l1: {
            let __sw1 = ((((game).wrapping_add(12)).read()) as i32);
            if __sw1 == 0i32 {
                crate::c::memset(
                    (((game).wrapping_add(66)).cast::<u16>()).cast::<u8>(),
                    0i32,
                    4u32,
                );
                if (((((((game).wrapping_add(152)).cast::<u8>())
                    .wrapping_offset(((((game).wrapping_add(8)).read()) as i32) as isize * 32))
                .wrapping_add(26)
                .cast::<u16>())
                .read()) as i32)
                    > ((((game).wrapping_add(40).cast::<u16>()).read()) as i32)
                {
                    (((((game).wrapping_add(152)).cast::<u8>()).wrapping_offset(
                        ((((game).wrapping_add(8)).read()) as i32) as isize * 32,
                    ))
                    .wrapping_add(26)
                    .cast::<u16>())
                    .write(((game).wrapping_add(40).cast::<u16>()).read());
                }
                (((game).wrapping_add(66)).cast::<u16>()).write(
                    (((((game).wrapping_add(152)).cast::<u8>()).wrapping_offset(
                        ((((game).wrapping_add(8)).read()) as i32) as isize * 32,
                    ))
                    .wrapping_add(26)
                    .cast::<u16>())
                    .read(),
                );
                SendBlock(
                    0u8,
                    (((game).wrapping_add(66)).cast::<u16>()).cast::<u8>(),
                    2u16,
                );
                break 'l1;
            }
            if __sw1 == 1i32 {
                if !((IsLinkTaskFinished()) != 0) {
                    return 0u32;
                }
                ((game).wrapping_add(16).cast::<u16>()).write(0u16);
                break 'l1;
            }
            if __sw1 == 2i32 {
                if ((GetBlockReceivedStatus()) as i32)
                    != ((((((&raw const sReceivedPlayerBitmasks).cast::<u8>().cast_mut())
                        .cast::<u8>())
                    .wrapping_offset(
                        (((((game).wrapping_add(9)).read()) as i32).wrapping_sub(2i32)) as isize,
                    ))
                    .read()) as i32)
                {
                    return 0u32;
                }
                {
                    i = 0u8;
                    'l2: loop {
                        if !(((i) as i32) < ((((game).wrapping_add(9)).read()) as i32)) {
                            break 'l2;
                        }
                        'l3: {
                            (((((game).wrapping_add(152)).cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize * 32))
                            .wrapping_add(26)
                            .cast::<u16>())
                            .write(
                                ((((&raw mut gBlockRecvBuffer).cast::<u8>())
                                    .wrapping_offset(((i) as i32) as isize * 256))
                                .cast::<u16>())
                                .read(),
                            );
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                ((game).wrapping_add(16).cast::<u16>()).write(0u16);
                (((game).wrapping_add(66)).cast::<u16>()).write(0u16);
                ResetBlockReceivedFlags();
                if ((((game).wrapping_add(8)).read()) as i32) == 0i32 {
                    ((game).wrapping_add(12)).write(3u8);
                } else {
                    ((game).wrapping_add(12)).write(6u8);
                }
                return 0u32;
            }
            if __sw1 == 3i32 {
                crate::c::memset((game).wrapping_add(104), 0i32, 48u32);
                (((game).wrapping_add(104)).wrapping_add(4).cast::<u16>())
                    .write(((game).wrapping_add(40).cast::<u16>()).read());
                (((game).wrapping_add(104)).wrapping_add(6).cast::<u16>()).write(
                    ((crate::c::div_i32(
                        ((((game).wrapping_add(24).cast::<i16>()).read()) as i32),
                        crate::c::div_i32(
                            ((((game).wrapping_add(40).cast::<u16>()).read()) as i32),
                            60i32,
                        ),
                    )) as u16),
                );
                temp1 = MathUtil_Mul32(
                    (((((game).wrapping_add(48).cast::<i16>()).read()) as i32) << 8),
                    12800i32,
                );
                temp1 = (MathUtil_Div32(
                    temp1,
                    (((((game).wrapping_add(50).cast::<i16>()).read()) as i32) << 8),
                ))
                .wrapping_add(12800i32);
                temp1 = (temp1 >> 8);
                (((game).wrapping_add(104)).wrapping_add(8).cast::<u16>())
                    .write(((temp1 & 127i32) as u16));
                temp1 = (temp1 << 8);
                temp1 = MathUtil_Div32(temp1, 25600i32);
                temp2 = ((((game).wrapping_add(28).cast::<i32>()).read())
                    .wrapping_mul(((((game).wrapping_add(9)).read()) as i32))
                    << 8);
                temp2 = MathUtil_Mul32(temp2, temp1);
                (((game).wrapping_add(104)).cast::<u32>()).write(((temp2 >> 8) as u32));
                ((((((game).wrapping_add(104)).wrapping_add(32)).cast::<u8>()).cast::<u8>())
                    .wrapping_offset(7))
                .write(((crate::c::rem_i32(((Random()) as i32), 3i32)) as u8));
                {
                    i = 0u8;
                    'l4: loop {
                        if !(((i) as i32) < ((((game).wrapping_add(9)).read()) as i32)) {
                            break 'l4;
                        }
                        'l5: {
                            ((((((game).wrapping_add(104)).wrapping_add(32)).cast::<u8>())
                                .cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize))
                            .write(i);
                            (((((((game).wrapping_add(104)).wrapping_add(32)).cast::<u8>())
                                .wrapping_offset(8))
                            .cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize))
                            .write(i);
                            ((((((game).wrapping_add(104)).wrapping_add(12)).cast::<u8>())
                                .cast::<u16>())
                            .wrapping_offset(((i) as i32) as isize))
                            .write(
                                (((((game).wrapping_add(152)).cast::<u8>())
                                    .wrapping_offset(((i) as i32) as isize * 32))
                                .wrapping_add(22)
                                .cast::<u16>())
                                .read(),
                            );
                            let __p2 = ((game).wrapping_add(104)).wrapping_add(10).cast::<u16>();
                            (__p2).write(
                                (((((__p2).read()) as i32).wrapping_add(
                                    ((((((((game).wrapping_add(104)).wrapping_add(12))
                                        .cast::<u8>())
                                    .cast::<u16>())
                                    .wrapping_offset(((i) as i32) as isize))
                                    .read()) as i32),
                                )) as u16),
                            );
                            'l6: {
                                let __sw3 = ((((((((game).wrapping_add(104)).wrapping_add(32))
                                    .cast::<u8>())
                                .cast::<u8>())
                                .wrapping_offset(7))
                                .read()) as i32);
                                if __sw3 == 0i32 {
                                    if (((((((game).wrapping_add(152)).cast::<u8>())
                                        .wrapping_offset(((i) as i32) as isize * 32))
                                    .wrapping_add(22)
                                    .cast::<u16>())
                                    .read()) as i32)
                                        != 0i32
                                    {
                                        temp1 = (((((((game).wrapping_add(152)).cast::<u8>())
                                            .wrapping_offset(((i) as i32) as isize * 32))
                                        .wrapping_add(20)
                                        .cast::<u16>())
                                        .read())
                                            as i32);
                                        temp1 = (temp1 << 8);
                                        temp1 = MathUtil_Mul32(temp1, 25600i32);
                                        temp2 = (((((((game).wrapping_add(152)).cast::<u8>())
                                            .wrapping_offset(((i) as i32) as isize * 32))
                                        .wrapping_add(22)
                                        .cast::<u16>())
                                        .read())
                                            as i32);
                                        temp2 = (temp2 << 8);
                                        temp2 = MathUtil_Div32(temp1, temp2);
                                    } else {
                                        temp2 = 0i32;
                                    }
                                    break 'l6;
                                }
                                if __sw3 == 1i32 {
                                    if (((((((game).wrapping_add(152)).cast::<u8>())
                                        .wrapping_offset(((i) as i32) as isize * 32))
                                    .wrapping_add(22)
                                    .cast::<u16>())
                                    .read()) as i32)
                                        != 0i32
                                    {
                                        temp1 = (((((((game).wrapping_add(152)).cast::<u8>())
                                            .wrapping_offset(((i) as i32) as isize * 32))
                                        .wrapping_add(24)
                                        .cast::<u16>())
                                        .read())
                                            as i32);
                                        temp1 = (temp1 << 8);
                                        temp1 = MathUtil_Mul32(temp1, 25600i32);
                                        temp2 = (((((((game).wrapping_add(152)).cast::<u8>())
                                            .wrapping_offset(((i) as i32) as isize * 32))
                                        .wrapping_add(22)
                                        .cast::<u16>())
                                        .read())
                                            as i32);
                                        temp2 = (temp2 << 8);
                                        temp2 = MathUtil_Div32(temp1, temp2);
                                    } else {
                                        temp2 = 0i32;
                                    }
                                    break 'l6;
                                }
                                if __sw3 == 2i32 {
                                    if (((((((game).wrapping_add(152)).cast::<u8>())
                                        .wrapping_offset(((i) as i32) as isize * 32))
                                    .wrapping_add(22)
                                    .cast::<u16>())
                                    .read()) as i32)
                                        == 0i32
                                    {
                                        temp2 = 0i32;
                                    } else {
                                        if (((((((game).wrapping_add(152)).cast::<u8>())
                                            .wrapping_offset(((i) as i32) as isize * 32))
                                        .wrapping_add(26)
                                        .cast::<u16>())
                                        .read()) as i32)
                                            >= ((((game).wrapping_add(40).cast::<u16>()).read())
                                                as i32)
                                        {
                                            temp2 = 25600i32;
                                        } else {
                                            temp1 = (((((((game).wrapping_add(152)).cast::<u8>())
                                                .wrapping_offset(((i) as i32) as isize * 32))
                                            .wrapping_add(26)
                                            .cast::<u16>())
                                            .read())
                                                as i32);
                                            temp1 = (temp1 << 8);
                                            temp1 = MathUtil_Mul32(temp1, 25600i32);
                                            temp2 = ((((game).wrapping_add(40).cast::<u16>())
                                                .read())
                                                as i32);
                                            temp2 = (temp2 << 8);
                                            temp2 = MathUtil_Div32(temp1, temp2);
                                        }
                                    }
                                    break 'l6;
                                }
                            }
                            temp2 = (temp2 >> 4);
                            (((((((game).wrapping_add(104)).wrapping_add(12)).cast::<u8>())
                                .wrapping_offset(10))
                            .cast::<u16>())
                            .wrapping_offset(((i) as i32) as isize))
                            .write(((temp2) as u16));
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                break 'l1;
            }
            if __sw1 == 4i32 {
                {
                    i = 0u8;
                    'l7: loop {
                        if !(((i) as i32)
                            < ((((game).wrapping_add(9)).read()) as i32).wrapping_sub(1i32))
                        {
                            break 'l7;
                        }
                        'l8: {
                            {
                                j = ((((((game).wrapping_add(9)).read()) as i32).wrapping_sub(1i32))
                                    as u8);
                                'l9: loop {
                                    if !(((j) as i32) > ((i) as i32)) {
                                        break 'l9;
                                    }
                                    'l10: {
                                        if ((((((((game).wrapping_add(104)).wrapping_add(12))
                                            .cast::<u8>())
                                        .cast::<u16>())
                                        .wrapping_offset(
                                            (((j) as i32).wrapping_sub(1i32)) as isize,
                                        ))
                                        .read()) as i32)
                                            < ((((((((game).wrapping_add(104)).wrapping_add(12))
                                                .cast::<u8>())
                                            .cast::<u16>())
                                            .wrapping_offset(((j) as i32) as isize))
                                            .read())
                                                as i32)
                                        {
                                            {
                                                tempStat = ((((((game).wrapping_add(104))
                                                    .wrapping_add(12))
                                                .cast::<u8>())
                                                .cast::<u16>())
                                                .wrapping_offset(((j) as i32) as isize))
                                                .read();
                                                ((((((game).wrapping_add(104)).wrapping_add(12))
                                                    .cast::<u8>())
                                                .cast::<u16>())
                                                .wrapping_offset(((j) as i32) as isize))
                                                .write(
                                                    ((((((game).wrapping_add(104))
                                                        .wrapping_add(12))
                                                    .cast::<u8>())
                                                    .cast::<u16>())
                                                    .wrapping_offset(
                                                        (((j) as i32).wrapping_sub(1i32)) as isize,
                                                    ))
                                                    .read(),
                                                );
                                                ((((((game).wrapping_add(104)).wrapping_add(12))
                                                    .cast::<u8>())
                                                .cast::<u16>())
                                                .wrapping_offset(
                                                    (((j) as i32).wrapping_sub(1i32)) as isize,
                                                ))
                                                .write(tempStat);
                                            }
                                            {
                                                tempPlayerId = ((((((game).wrapping_add(104))
                                                    .wrapping_add(32))
                                                .cast::<u8>())
                                                .cast::<u8>())
                                                .wrapping_offset(((j) as i32) as isize))
                                                .read();
                                                ((((((game).wrapping_add(104)).wrapping_add(32))
                                                    .cast::<u8>())
                                                .cast::<u8>())
                                                .wrapping_offset(((j) as i32) as isize))
                                                .write(
                                                    ((((((game).wrapping_add(104))
                                                        .wrapping_add(32))
                                                    .cast::<u8>())
                                                    .cast::<u8>())
                                                    .wrapping_offset(
                                                        (((j) as i32).wrapping_sub(1i32)) as isize,
                                                    ))
                                                    .read(),
                                                );
                                                ((((((game).wrapping_add(104)).wrapping_add(32))
                                                    .cast::<u8>())
                                                .cast::<u8>())
                                                .wrapping_offset(
                                                    (((j) as i32).wrapping_sub(1i32)) as isize,
                                                ))
                                                .write(tempPlayerId);
                                            }
                                        }
                                        if (((((((((game).wrapping_add(104)).wrapping_add(12))
                                            .cast::<u8>())
                                        .wrapping_offset(10))
                                        .cast::<u16>())
                                        .wrapping_offset(
                                            (((j) as i32).wrapping_sub(1i32)) as isize,
                                        ))
                                        .read()) as i32)
                                            < (((((((((game).wrapping_add(104))
                                                .wrapping_add(12))
                                            .cast::<u8>())
                                            .wrapping_offset(10))
                                            .cast::<u16>())
                                            .wrapping_offset(((j) as i32) as isize))
                                            .read())
                                                as i32)
                                        {
                                            {
                                                tempStat = (((((((game).wrapping_add(104))
                                                    .wrapping_add(12))
                                                .cast::<u8>())
                                                .wrapping_offset(10))
                                                .cast::<u16>())
                                                .wrapping_offset(((j) as i32) as isize))
                                                .read();
                                                (((((((game).wrapping_add(104))
                                                    .wrapping_add(12))
                                                .cast::<u8>())
                                                .wrapping_offset(10))
                                                .cast::<u16>())
                                                .wrapping_offset(((j) as i32) as isize))
                                                .write(
                                                    (((((((game).wrapping_add(104))
                                                        .wrapping_add(12))
                                                    .cast::<u8>())
                                                    .wrapping_offset(10))
                                                    .cast::<u16>())
                                                    .wrapping_offset(
                                                        (((j) as i32).wrapping_sub(1i32)) as isize,
                                                    ))
                                                    .read(),
                                                );
                                                (((((((game).wrapping_add(104))
                                                    .wrapping_add(12))
                                                .cast::<u8>())
                                                .wrapping_offset(10))
                                                .cast::<u16>())
                                                .wrapping_offset(
                                                    (((j) as i32).wrapping_sub(1i32)) as isize,
                                                ))
                                                .write(tempStat);
                                            }
                                            {
                                                tempPlayerId = (((((((game).wrapping_add(104))
                                                    .wrapping_add(32))
                                                .cast::<u8>())
                                                .wrapping_offset(8))
                                                .cast::<u8>())
                                                .wrapping_offset(((j) as i32) as isize))
                                                .read();
                                                (((((((game).wrapping_add(104))
                                                    .wrapping_add(32))
                                                .cast::<u8>())
                                                .wrapping_offset(8))
                                                .cast::<u8>())
                                                .wrapping_offset(((j) as i32) as isize))
                                                .write(
                                                    (((((((game).wrapping_add(104))
                                                        .wrapping_add(32))
                                                    .cast::<u8>())
                                                    .wrapping_offset(8))
                                                    .cast::<u8>())
                                                    .wrapping_offset(
                                                        (((j) as i32).wrapping_sub(1i32)) as isize,
                                                    ))
                                                    .read(),
                                                );
                                                (((((((game).wrapping_add(104))
                                                    .wrapping_add(32))
                                                .cast::<u8>())
                                                .wrapping_offset(8))
                                                .cast::<u8>())
                                                .wrapping_offset(
                                                    (((j) as i32).wrapping_sub(1i32)) as isize,
                                                ))
                                                .write(tempPlayerId);
                                            }
                                        }
                                    }
                                    j = (j).wrapping_sub(1);
                                }
                            }
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                SendBlock(0u8, (game).wrapping_add(104), 48u16);
                break 'l1;
            }
            if __sw1 == 5i32 {
                if !((IsLinkTaskFinished()) != 0) {
                    return 0u32;
                }
                ((game).wrapping_add(16).cast::<u16>()).write(0u16);
                break 'l1;
            }
            if __sw1 == 6i32 {
                if ((GetBlockReceivedStatus()) as i32) != 1i32 {
                    return 0u32;
                }
                crate::c::memset((game).wrapping_add(104), 0i32, 48u32);
                crate::c::memcpy(
                    (game).wrapping_add(104),
                    (&raw mut gBlockRecvBuffer).cast::<u8>(),
                    48u32,
                );
                ResetBlockReceivedFlags();
                ((game).wrapping_add(16).cast::<u16>()).write(0u16);
                break 'l1;
            }
            if __sw1 == 7i32 {
                SaveResults();
                RunOrScheduleCommand(18u16, 1u8, core::ptr::null_mut());
                ((game).wrapping_add(18).cast::<u16>()).write(11u16);
                ((game).wrapping_add(12)).write(0u8);
                ((game).wrapping_add(36)).write(0u8);
                return 0u32;
            }
        }
        let __p4 = (game).wrapping_add(12);
        (__p4).write(((__p4).read()).wrapping_add(1));
        return 0u32;
    }
}
pub(crate) unsafe extern "C" fn Cmd_ShowResults(game: *mut u8, args: *mut u8) -> u32 {
    unsafe {
        let mut game = game;
        let mut args = args;
        'l1: {
            let __sw1 = ((((game).wrapping_add(12)).read()) as i32);
            if __sw1 == 0i32 {
                if !((OpenResultsWindow(game, (game).wrapping_add(312))) != 0) {
                    return 0u32;
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                CopyBgTilemapBufferToVram(0u8);
                ((game).wrapping_add(312)).write(30u8);
                break 'l1;
            }
            if __sw1 == 2i32 {
                if ((((game).wrapping_add(312)).read()) as i32) != 0i32 {
                    let __p2 = ((game).wrapping_add(312));
                    (__p2).write(((__p2).read()).wrapping_sub(1));
                    return 0u32;
                }
                if !(((((((&raw mut gMain).cast::<u8>())
                    .wrapping_add(46)
                    .cast::<u16>())
                .read()) as i32)
                    & 1i32)
                    != 0)
                {
                    return 0u32;
                }
                PlaySE(5u16);
                CloseResultsWindow(game);
                break 'l1;
            }
            if __sw1 == 3i32 {
                if ((((game).wrapping_add(18).cast::<u16>()).read()) as i32) < 13i32 {
                    let __p3 = (game).wrapping_add(18).cast::<u16>();
                    (__p3).write(((__p3).read()).wrapping_add(1));
                    ((game).wrapping_add(12)).write(0u8);
                    return 0u32;
                }
                break 'l1;
            }
            if __sw1 == 4i32 {
                ConvertIntToDecimalStringN(
                    (&raw mut gStringVar1).cast::<u8>(),
                    ((game).wrapping_add(28).cast::<i32>()).read(),
                    0i32,
                    6u8,
                );
                ConvertIntToDecimalStringN(
                    (&raw mut gStringVar2).cast::<u8>(),
                    ((GetBerryPowder()) as i32),
                    0i32,
                    6u8,
                );
                SetPrintMessageArgs(args, 2u8, 3u8, 0u16, 0u8);
                ((game).wrapping_add(14)).write(19u8);
                RunOrScheduleCommand(3u16, 1u8, core::ptr::null_mut());
                ((game).wrapping_add(12)).write(0u8);
                return 0u32;
            }
        }
        let __p4 = (game).wrapping_add(12);
        (__p4).write(((__p4).read()).wrapping_add(1));
        return 0u32;
    }
}
pub(crate) unsafe extern "C" fn Cmd_SaveGame(game: *mut u8, args: *mut u8) -> u32 {
    unsafe {
        let mut game = game;
        let mut args = args;
        'l1: {
            let __sw1 = ((((game).wrapping_add(12)).read()) as i32);
            if __sw1 == 0i32 {
                if ((((game).wrapping_add(40).cast::<u16>()).read()) as i32) >= 36000i32 {
                    HideTimer((game).wrapping_add(312));
                }
                SetPrintMessageArgs(args, 8u8, 0u8, 0u16, 1u8);
                ((game).wrapping_add(14)).write(19u8);
                RunOrScheduleCommand(3u16, 1u8, core::ptr::null_mut());
                ((game).wrapping_add(12)).write(0u8);
                return 0u32;
            }
            if __sw1 == 1i32 {
                Rfu_SetLinkStandbyCallback();
                break 'l1;
            }
            if __sw1 == 2i32 {
                if !((IsLinkTaskFinished()) != 0) {
                    return 0u32;
                }
                DrawDialogueFrame(0u8, 0u8);
                AddTextPrinterParameterized2(
                    0u8,
                    1u8,
                    (&raw mut gText_SavingDontTurnOffPower).cast::<u8>(),
                    0u8,
                    None,
                    2u8,
                    1u8,
                    3u8,
                );
                CopyWindowToVram(0u8, 3u8);
                CreateTask(Some(Task_LinkFullSave), 0u8);
                break 'l1;
            }
            if __sw1 == 3i32 {
                if (FuncIsActiveTask(Some(Task_LinkFullSave))) != 0 {
                    return 0u32;
                }
                break 'l1;
            }
            if __sw1 == 4i32 {
                RunOrScheduleCommand(20u16, 1u8, core::ptr::null_mut());
                ((game).wrapping_add(18).cast::<u16>()).write(15u16);
                ((game).wrapping_add(12)).write(0u8);
                return 0u32;
            }
        }
        let __p2 = (game).wrapping_add(12);
        (__p2).write(((__p2).read()).wrapping_add(1));
        return 0u32;
    }
}
pub(crate) unsafe extern "C" fn Cmd_AskPlayAgain(game: *mut u8, args: *mut u8) -> u32 {
    unsafe {
        let mut game = game;
        let mut args = args;
        let mut input: i8 = 0i8;
        'l1: {
            let __sw1 = ((((game).wrapping_add(12)).read()) as i32);
            if __sw1 == 0i32 {
                SetPrintMessageArgs(args, 4u8, 0u8, 0u16, 1u8);
                ((game).wrapping_add(14)).write(20u8);
                RunOrScheduleCommand(3u16, 1u8, core::ptr::null_mut());
                ((game).wrapping_add(12)).write(0u8);
                return 0u32;
            }
            if __sw1 == 1i32 {
                DisplayYesNoMenuDefaultYes();
                break 'l1;
            }
            if __sw1 == 2i32 {
                input = Menu_ProcessInputNoWrapClearOnChoose();
                if ((input) as i32) != (-2i32) {
                    crate::c::memset(
                        (((game).wrapping_add(66)).cast::<u16>()).cast::<u8>(),
                        0i32,
                        12u32,
                    );
                    if ((input) as i32) == 0i32 {
                        if (HasAtLeastOneBerry()) != 0 {
                            ((game).wrapping_add(20).cast::<u16>()).write(0u16);
                        } else {
                            ((game).wrapping_add(20).cast::<u16>()).write(3u16);
                        }
                    } else {
                        ((game).wrapping_add(20).cast::<u16>()).write(1u16);
                    }
                    ClearDialogWindowAndFrame(0u8, 1u8);
                    SetPrintMessageArgs(args, 8u8, 0u8, 0u16, 0u8);
                    ((game).wrapping_add(14)).write(21u8);
                    RunOrScheduleCommand(3u16, 1u8, core::ptr::null_mut());
                    ((game).wrapping_add(12)).write(0u8);
                }
                return 0u32;
            }
        }
        let __p2 = (game).wrapping_add(12);
        (__p2).write(((__p2).read()).wrapping_add(1));
        return 0u32;
    }
}
pub(crate) unsafe extern "C" fn Cmd_CommunicatePlayAgainResponses(
    game: *mut u8,
    args: *mut u8,
) -> u32 {
    unsafe {
        let mut game = game;
        let mut args = args;
        let mut i: u8 = 0u8;
        'l1: {
            let __sw1 = ((((game).wrapping_add(12)).read()) as i32);
            if __sw1 == 0i32 {
                Rfu_SetLinkStandbyCallback();
                break 'l1;
            }
            if __sw1 == 1i32 {
                if !((IsLinkTaskFinished()) != 0) {
                    return 0u32;
                }
                (((game).wrapping_add(66)).cast::<u16>())
                    .write(((game).wrapping_add(20).cast::<u16>()).read());
                (((game).wrapping_add(78)).cast::<u16>()).write(0u16);
                SendBlock(
                    0u8,
                    (((game).wrapping_add(66)).cast::<u16>()).cast::<u8>(),
                    2u16,
                );
                break 'l1;
            }
            if __sw1 == 2i32 {
                if !((IsLinkTaskFinished()) != 0) {
                    return 0u32;
                }
                ((game).wrapping_add(16).cast::<u16>()).write(0u16);
                break 'l1;
            }
            if __sw1 == 3i32 {
                if ((GetBlockReceivedStatus()) as i32)
                    != ((((((&raw const sReceivedPlayerBitmasks).cast::<u8>().cast_mut())
                        .cast::<u8>())
                    .wrapping_offset(
                        (((((game).wrapping_add(9)).read()) as i32).wrapping_sub(2i32)) as isize,
                    ))
                    .read()) as i32)
                {
                    return 0u32;
                }
                {
                    i = 0u8;
                    'l2: loop {
                        if !(((i) as i32) < ((((game).wrapping_add(9)).read()) as i32)) {
                            break 'l2;
                        }
                        'l3: {
                            let __p2 = ((game).wrapping_add(78)).cast::<u16>();
                            (__p2).write(
                                (((((__p2).read()) as i32).wrapping_add(
                                    ((((((&raw mut gBlockRecvBuffer).cast::<u8>())
                                        .wrapping_offset(((i) as i32) as isize * 256))
                                    .cast::<u16>())
                                    .read()) as i32),
                                )) as u16),
                            );
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                if (((((game).wrapping_add(78)).cast::<u16>()).read()) as i32) != 0i32 {
                    RunOrScheduleCommand(23u16, 1u8, core::ptr::null_mut());
                } else {
                    RunOrScheduleCommand(22u16, 1u8, core::ptr::null_mut());
                }
                ResetBlockReceivedFlags();
                (((game).wrapping_add(66)).cast::<u16>()).write(0u16);
                (((game).wrapping_add(78)).cast::<u16>()).write(0u16);
                ((game).wrapping_add(16).cast::<u16>()).write(0u16);
                ((game).wrapping_add(12)).write(0u8);
                return 0u32;
            }
        }
        let __p3 = (game).wrapping_add(12);
        (__p3).write(((__p3).read()).wrapping_add(1));
        return 0u32;
    }
}
pub(crate) unsafe extern "C" fn Cmd_PlayAgain(game: *mut u8, args: *mut u8) -> u32 {
    unsafe {
        let mut game = game;
        let mut args = args;
        'l1: {
            let __sw1 = ((((game).wrapping_add(12)).read()) as i32);
            if __sw1 == 0i32 {
                BeginNormalPaletteFade(4294967295u32, 1i8, 0u8, 16u8, 0u16);
                UpdatePaletteFade();
                break 'l1;
            }
            if __sw1 == 1i32 {
                if (UpdatePaletteFade()) != 0 {
                    return 0u32;
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                ClearDialogWindowAndFrame(0u8, 1u8);
                ResetCrusherPos(game);
                BeginNormalPaletteFade(4294967295u32, 0i8, 16u8, 0u8, 0u16);
                UpdatePaletteFade();
                break 'l1;
            }
            if __sw1 == 3i32 {
                if (UpdatePaletteFade()) != 0 {
                    return 0u32;
                }
                RunOrScheduleCommand(7u16, 1u8, core::ptr::null_mut());
                ((game).wrapping_add(18).cast::<u16>()).write(3u16);
                ((game).wrapping_add(12)).write(0u8);
                return 0u32;
            }
        }
        let __p2 = (game).wrapping_add(12);
        (__p2).write(((__p2).read()).wrapping_add(1));
        return 0u32;
    }
}
pub(crate) unsafe extern "C" fn Cmd_StopGame(game: *mut u8, args: *mut u8) -> u32 {
    unsafe {
        let mut game = game;
        let mut args = args;
        'l1: {
            let __sw1 = ((((game).wrapping_add(12)).read()) as i32);
            if __sw1 == 0i32 {
                DrawDialogueFrame(0u8, 0u8);
                if ((((game).wrapping_add(20).cast::<u16>()).read()) as i32) == 3i32 {
                    AddTextPrinterParameterized2(
                        0u8,
                        1u8,
                        ((((&raw const sMessages)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<*mut u8>())
                        .cast::<*mut u8>())
                        .wrapping_offset(5))
                        .read(),
                        ((game).wrapping_add(11)).read(),
                        None,
                        2u8,
                        1u8,
                        3u8,
                    );
                } else {
                    AddTextPrinterParameterized2(
                        0u8,
                        1u8,
                        ((((&raw const sMessages)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<*mut u8>())
                        .cast::<*mut u8>())
                        .wrapping_offset(6))
                        .read(),
                        ((game).wrapping_add(11)).read(),
                        None,
                        2u8,
                        1u8,
                        3u8,
                    );
                }
                CopyWindowToVram(0u8, 3u8);
                break 'l1;
            }
            if __sw1 == 1i32 {
                if (IsTextPrinterActive(0u8)) != 0 {
                    return 0u32;
                }
                ((game).wrapping_add(312)).write(120u8);
                break 'l1;
            }
            if __sw1 == 2i32 {
                if ((((game).wrapping_add(312)).read()) as i32) != 0i32 {
                    let __p2 = ((game).wrapping_add(312));
                    (__p2).write(((__p2).read()).wrapping_sub(1));
                } else {
                    RunOrScheduleCommand(24u16, 1u8, core::ptr::null_mut());
                    ((game).wrapping_add(12)).write(0u8);
                }
                return 0u32;
            }
        }
        let __p3 = (game).wrapping_add(12);
        (__p3).write(((__p3).read()).wrapping_add(1));
        return 0u32;
    }
}
pub(crate) unsafe extern "C" fn Cmd_CloseLink(game: *mut u8, args: *mut u8) -> u32 {
    unsafe {
        let mut game = game;
        let mut args = args;
        'l1: {
            let __sw1 = ((((game).wrapping_add(12)).read()) as i32);
            if __sw1 == 0i32 {
                Rfu_SetLinkStandbyCallback();
                break 'l1;
            }
            if __sw1 == 1i32 {
                if !((IsLinkTaskFinished()) != 0) {
                    return 0u32;
                }
                SetCloseLinkCallback();
                break 'l1;
            }
            if __sw1 == 2i32 {
                if (((&raw mut gReceivedRemoteLinkPlayers).cast::<u8>()).read()) != 0 {
                    return 0u32;
                }
                ((game).wrapping_add(14)).write(25u8);
                RunOrScheduleCommand(5u16, 1u8, core::ptr::null_mut());
                ((game).wrapping_add(12)).write(2u8);
                return 0u32;
            }
        }
        let __p2 = (game).wrapping_add(12);
        (__p2).write(((__p2).read()).wrapping_add(1));
        return 0u32;
    }
}
pub(crate) unsafe extern "C" fn Cmd_Quit(game: *mut u8, args: *mut u8) -> u32 {
    unsafe {
        let mut game = game;
        let mut args = args;
        QuitBerryCrush(None);
        return 0u32;
    }
}
pub(crate) unsafe extern "C" fn ResetGame(game: *mut u8) {
    unsafe {
        let mut game = game;
        let mut i: u8 = 0u8;
        IncrementGameStat(51u8);
        ((game).wrapping_add(13)).write(0u8);
        ((game).wrapping_add(16).cast::<u16>()).write(0u16);
        ((game).wrapping_add(18).cast::<u16>()).write(2u16);
        ((game).wrapping_add(20).cast::<u16>()).write(0u16);
        ((game).wrapping_add(28).cast::<i32>()).write(0i32);
        ((game).wrapping_add(24).cast::<i16>()).write(0i16);
        ((game).wrapping_add(26).cast::<i16>()).write(0i16);
        ((game).wrapping_add(32).cast::<i32>()).write(0i32);
        ((game).wrapping_add(36)).write(0u8);
        crate::c::bf_write((game).wrapping_add(37), 0, 1, (0u8) as i32);
        crate::c::bf_write((game).wrapping_add(37), 1, 1, (0u8) as i32);
        crate::c::bf_write((game).wrapping_add(37), 2, 1, (0u8) as i32);
        crate::c::bf_write((game).wrapping_add(37), 3, 1, (0u8) as i32);
        crate::c::bf_write((game).wrapping_add(37), 4, 1, (0u8) as i32);
        crate::c::bf_write((game).wrapping_add(37), 5, 3, (0u8) as i32);
        ((game).wrapping_add(38).cast::<u16>()).write(0u16);
        ((game).wrapping_add(40).cast::<u16>()).write(0u16);
        ((game).wrapping_add(46).cast::<i16>()).write(0i16);
        ((game).wrapping_add(50).cast::<i16>()).write((-1i16));
        ((game).wrapping_add(48).cast::<i16>()).write(0i16);
        ((game).wrapping_add(52).cast::<i16>()).write(0i16);
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 5i32) {
                    break 'l1;
                }
                'l2: {
                    (((((game).wrapping_add(152)).cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 32))
                    .wrapping_add(12)
                    .cast::<u16>())
                    .write(65535u16);
                    (((((game).wrapping_add(152)).cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 32))
                    .wrapping_add(14)
                    .cast::<u16>())
                    .write(0u16);
                    (((((game).wrapping_add(152)).cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 32))
                    .wrapping_add(16)
                    .cast::<u16>())
                    .write(0u16);
                    (((((game).wrapping_add(152)).cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 32))
                    .wrapping_add(18)
                    .cast::<u16>())
                    .write(1u16);
                    (((((game).wrapping_add(152)).cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 32))
                    .wrapping_add(20)
                    .cast::<u16>())
                    .write(0u16);
                    (((((game).wrapping_add(152)).cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 32))
                    .wrapping_add(22)
                    .cast::<u16>())
                    .write(0u16);
                    (((((game).wrapping_add(152)).cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 32))
                    .wrapping_add(24)
                    .cast::<u16>())
                    .write(0u16);
                    (((((game).wrapping_add(152)).cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 32))
                    .wrapping_add(26)
                    .cast::<u16>())
                    .write(0u16);
                    (((((game).wrapping_add(152)).cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 32))
                    .wrapping_add(28))
                    .write(0u8);
                    (((((game).wrapping_add(152)).cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 32))
                    .wrapping_add(29))
                    .write(0u8);
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SetPaletteFadeArgs(
    args: *mut u8,
    communicateAfter: u8,
    selectedPals: u32,
    delay: i8,
    startY: u8,
    targetY: u8,
    palette: u16,
) {
    unsafe {
        let mut args = args;
        let mut communicateAfter = communicateAfter;
        let mut selectedPals = selectedPals;
        let mut delay = delay;
        let mut startY = startY;
        let mut targetY = targetY;
        let mut palette = palette;
        (args).write(((&raw mut selectedPals).cast::<u8>()).read());
        ((args).wrapping_offset(1))
            .write((((&raw mut selectedPals).cast::<u8>()).wrapping_offset(1)).read());
        ((args).wrapping_offset(2))
            .write((((&raw mut selectedPals).cast::<u8>()).wrapping_offset(2)).read());
        ((args).wrapping_offset(3))
            .write((((&raw mut selectedPals).cast::<u8>()).wrapping_offset(3)).read());
        ((args).wrapping_offset(4)).write(((delay) as u8));
        ((args).wrapping_offset(5)).write(startY);
        ((args).wrapping_offset(6)).write(targetY);
        ((args).wrapping_offset(7)).write(((&raw mut palette).cast::<u8>()).read());
        ((args).wrapping_offset(8))
            .write((((&raw mut palette).cast::<u8>()).wrapping_offset(1)).read());
        ((args).wrapping_offset(9)).write(communicateAfter);
    }
}
pub(crate) unsafe extern "C" fn SetPrintMessageArgs(
    args: *mut u8,
    msgId: u8,
    flags: u8,
    waitKeys: u16,
    followupState: u8,
) {
    unsafe {
        let mut args = args;
        let mut msgId = msgId;
        let mut flags = flags;
        let mut waitKeys = waitKeys;
        let mut followupState = followupState;
        (args).write(msgId);
        ((args).wrapping_offset(1)).write(flags);
        ((args).wrapping_offset(2)).write(((&raw mut waitKeys).cast::<u8>()).read());
        ((args).wrapping_offset(3))
            .write((((&raw mut waitKeys).cast::<u8>()).wrapping_offset(1)).read());
        ((args).wrapping_offset(4)).write(followupState);
    }
}
