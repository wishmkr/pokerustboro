//! Translated from `src/berry_blender.c` by tools/rustport/c2rs.py, then reviewed.
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

// Data tables (translate with cdata.py): sBlenderCenter_Pal sBlenderCenter_Tilemap sBlenderOuter_Pal sUnused_Pal sEmpty_Pal sUnusedText_YesNo sUnusedText_2 sUnusedText_Space sUnusedText_Terminating sUnusedText_LinkPartnerNotFound sText_BerryBlenderStart sText_NewParagraph sText_WasMade sText_Mister sText_Laddie sText_Lassie sText_Master sText_Dude sText_Miss sBlenderOpponentsNames sText_PressAToStart sText_PleaseWaitAWhile sText_CommunicationStandby sText_WouldLikeToBlendAnotherBerry sText_RunOutOfBerriesForBlending sText_YourPokeblockCaseIsFull sText_HasNoBerriesToPut sText_ApostropheSPokeblockCaseIsFull sText_BlendingResults sText_BerryUsed sText_SpaceBerry sText_Time sText_Min sText_Sec sText_MaximumSpeed sText_RPM sText_Dot sText_NewLine sText_Space sText_Ranking sText_TheLevelIs sText_TheFeelIs sText_Dot2 sBgTemplates sWindowTemplates sYesNoWindowTemplate_ContinuePlaying sPlayerArrowQuadrant sPlayerArrowPos sPlayerIdMap sArrowStartPos sArrowStartPosIds sArrowHitRangeStart sLocalOpponentTasks sOam_PlayerArrow sAnim_PlayerArrow_TopLeft sAnim_PlayerArrow_TopRight sAnim_PlayerArrow_BottomLeft sAnim_PlayerArrow_BottomRight sAnim_PlayerArrow_TopLeft_Flash sAnim_PlayerArrow_TopRight_Flash sAnim_PlayerArrow_BottomLeft_Flash sAnim_PlayerArrow_BottomRight_Flash sAnim_PlayerArrow_TopLeft_Off sAnim_PlayerArrow_TopRight_Off sAnim_PlayerArrow_BottomLeft_Off sAnim_PlayerArrow_BottomRight_Off sAnims_PlayerArrow sSpriteSheet_PlayerArrow sSpritePal_BlenderMisc sSpritePal_PlayerArrow sSpriteTemplate_PlayerArrow sOam_ScoreSymbols sAnim_ScoreSymbols_Good sAnim_ScoreSymbols_Miss sAnim_ScoreSymbols_BestFlash sAnim_ScoreSymbols_BestStatic sAnims_ScoreSymbols sSpriteSheet_ScoreSymbols sSpriteTemplate_ScoreSymbols sOam_Particles sAnim_SparkleCrossToX sAnim_SparkleXToCross sAnim_SparkleFull sAnim_GreenArrow sAnim_GreenDot sAnims_Particles sSpriteSheet_Particles sSpriteTemplate_Particles sOam_CountdownNumbers sAnim_CountdownNumbers_3 sAnim_CountdownNumbers_2 sAnim_CountdownNumbers_1 sAnims_CountdownNumbers sSpriteSheet_CountdownNumbers sSpriteTemplate_CountdownNumbers sOam_Start sAnim_Start sAnims_Start sSpriteSheet_Start sSpriteTemplate_Start sBerrySpriteData sOpponentBerrySets sBerryMasterBerries sNumPlayersToSpeedDivisor sBlackPokeblockFlavorFlags sJPText_GoodTvReady sJPText_BadTvReady sJPText_Flavors sUnused sBlenderRecordWindowTemplate
#[allow(unused_imports)]
use crate::data::berry_blender::*;

#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sBerryBlender: *mut u8 = core::ptr::null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sDebug_PokeblockFactorFlavors: crate::ffi::Align4<[u8; 20]> =
    crate::ffi::Align4([0; 20]);
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sDebug_PokeblockFactorFlavorsAfterRPM: crate::ffi::Align4<[u8; 20]> =
    crate::ffi::Align4([0; 20]);
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sDebug_PokeblockFactorRPM: u32 = 0u32;
pub(crate) static mut sPokeblockFlavors: crate::ffi::Align4<[u8; 12]> = crate::ffi::Align4([0; 12]);
pub(crate) static mut sPokeblockPresentFlavors: crate::ffi::Align4<[u8; 12]> =
    crate::ffi::Align4([0; 12]);
pub(crate) static mut sDebug_MaxRPMStage: i16 = 0i16;
pub(crate) static mut sDebug_GameTimeStage: i16 = 0i16;
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gInGameOpponentsNo: u8 = 0u8;

unsafe extern "C" {
    static mut gBerryBlenderCenter_Gfx: u8;
    static mut gBerryBlenderOuter_Gfx: u8;
    static mut gBerryBlenderOuter_Tilemap: u8;
    static mut gBlockRecvBuffer: u8;
    static mut gBlockSendBuffer: u8;
    static mut gEnableContestDebugging: u8;
    static mut gLinkPlayers: u8;
    static mut gLinkType: u8;
    static mut gMPlayInfo_BGM: u8;
    static mut gMPlayInfo_SE2: u8;
    static mut gMain: u8;
    static mut gPaletteFade: u8;
    static mut gPokeblockNames: u8;
    static mut gReceivedRemoteLinkPlayers: u8;
    static mut gRecordsWindowId: u8;
    static mut gRecvCmds: u8;
    static mut gSaveBlock1Ptr: u8;
    static mut gSaveBlock2Ptr: u8;
    static mut gSendCmd: u8;
    static mut gSineTable: u8;
    static mut gSoftResetDisabled: u8;
    static mut gSpecialVar_0x8004: u8;
    static mut gSpecialVar_ItemId: u8;
    static mut gSprites: u8;
    static mut gStringVar4: u8;
    static mut gTasks: u8;
    static mut gText_234Players: u8;
    static mut gText_BlenderMaxSpeedRecord: u8;
    static mut gText_SavingDontTurnOff2: u8;
    static mut gText_Space: u8;
    static mut gWirelessCommType: u8;
    fn AddPokeblock(a0: *mut u8) -> u32;
    fn AddTextPrinterParameterized(
        a0: u8,
        a1: u8,
        a2: *mut u8,
        a3: u8,
        a4: u8,
        a5: u8,
        a6: Option<unsafe extern "C" fn(*mut u8, u16)>,
    ) -> u16;
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
    fn BeginFastPaletteFade(a0: u8);
    fn BeginNormalPaletteFade(a0: u32, a1: i8, a2: u8, a3: u8, a4: u16) -> u8;
    fn BuildOamBuffer();
    fn CB2_ReturnToFieldContinueScriptPlayMapMusic();
    fn ChangeBgX(a0: u8, a1: i32, a2: u8) -> i32;
    fn ChangeBgY(a0: u8, a1: i32, a2: u8) -> i32;
    fn ChooseBerryForMachine(a0: Option<unsafe extern "C" fn()>);
    fn ClearDialogWindowAndFrameToTransparent(a0: u8, a1: u8);
    fn ClearLinkCallback();
    fn ClearStdWindowAndFrameToTransparent(a0: u8, a1: u8);
    fn ConvertIntToDecimalStringN(a0: *mut u8, a1: i32, a2: i32, a3: u8) -> *mut u8;
    fn ConvertInternationalString(a0: *mut u8, a1: u8);
    fn CopyBgTilemapBufferToVram(a0: u8);
    fn CopyToBgTilemapBuffer(a0: u8, a1: *mut u8, a2: u16, a3: u16);
    fn CopyWindowToVram(a0: u8, a1: u8);
    fn CreateSpinningBerrySprite(a0: u8, a1: u8, a2: u8, a3: u8) -> u8;
    fn CreateSprite(a0: *mut u8, a1: i16, a2: i16, a3: u8) -> u8;
    fn CreateTask(a0: Option<unsafe extern "C" fn(u8)>, a1: u8) -> u8;
    fn CreateWirelessStatusIndicatorSprite(a0: u8, a1: u8);
    fn CreateYesNoMenu(a0: *mut u8, a1: u16, a2: u8, a3: u8);
    fn DeactivateAllTextPrinters();
    fn DestroySprite(a0: *mut u8);
    fn DestroyTask(a0: u8);
    fn DrawDialogFrameWithCustomTileAndPalette(a0: u8, a1: u8, a2: u16, a3: u8);
    fn DrawStdFrameWithCustomTileAndPalette(a0: u8, a1: u8, a2: u16, a3: u8);
    fn DrawStdWindowFrame(a0: u8, a1: u8);
    fn FillBgTilemapBufferRect_Palette0(a0: u8, a1: u16, a2: u8, a3: u8, a4: u8, a5: u8);
    fn FillWindowPixelBuffer(a0: u8, a1: u8);
    fn FlagGet(a0: u16) -> u8;
    fn Free(a0: *mut u8);
    fn FreeAllSpritePalettes();
    fn FreeAllWindowBuffers();
    fn GetBerryInfo(a0: u8) -> *mut u8;
    fn GetBlockReceivedStatus() -> u8;
    fn GetCurrentMapMusic() -> u16;
    fn GetDecompressedDataSize(a0: *mut u32) -> u32;
    fn GetFirstFreePokeblockSlot() -> i8;
    fn GetHighestPokeblocksFlavorLevel(a0: *mut u8) -> u8;
    fn GetLinkPlayerCount() -> u8;
    fn GetLinkPlayerCountAsBitFlags() -> u8;
    fn GetMultiplayerId() -> u8;
    fn GetPlayerTextSpeedDelay() -> u8;
    fn GetPokeblocksFeel(a0: *mut u8) -> u8;
    fn GetPokeblocksFlavor(a0: *mut u8) -> u8;
    fn GetStringCenterAlignXOffset(a0: i32, a1: *mut u8, a2: i32) -> i32;
    fn GetStringRightAlignXOffset(a0: i32, a1: *mut u8, a2: i32) -> i32;
    fn IncrementDailyBerryBlender();
    fn IncrementGameStat(a0: u8);
    fn InitBgsFromTemplates(a0: u8, a1: *mut u8, a2: u8);
    fn InitWindows(a0: *mut u8) -> u16;
    fn IsBagPocketNonEmpty(a0: u8) -> u8;
    fn IsFanfareTaskInactive() -> u8;
    fn IsLinkTaskFinished() -> u8;
    fn IsTextPrinterActive(a0: u8) -> u16;
    fn LZDecompressWram(a0: *mut u32, a1: *mut u8);
    fn LoadBgTiles(a0: u8, a1: *mut u8, a2: u16, a3: u16) -> u16;
    fn LoadMessageBoxGfx(a0: u8, a1: u16, a2: u8);
    fn LoadOam();
    fn LoadPalette(a0: *mut u8, a1: u16, a2: u16);
    fn LoadSpritePalette(a0: *mut u8) -> u8;
    fn LoadSpriteSheet(a0: *mut u8) -> u16;
    fn LoadUserWindowBorderGfx(a0: u8, a1: u16, a2: u8);
    fn LoadWirelessStatusIndicatorSpriteGfx();
    fn Menu_LoadStdPalAt(a0: u16);
    fn Menu_ProcessInputNoWrapClearOnChoose() -> i8;
    fn PlayBGM(a0: u16);
    fn PlayFanfare(a0: u16);
    fn PlaySE(a0: u16);
    fn ProcessSpriteCopyRequests();
    fn Put3CheersForPokeblocksOnTheAir(a0: *mut u8, a1: u8, a2: u8, a3: u8, a4: u8) -> u8;
    fn PutWindowTilemap(a0: u8);
    fn Random() -> u16;
    fn RemoveBagItem(a0: u16, a1: u16) -> u8;
    fn ResetBgsAndClearDma3BusyFlags(a0: u32);
    fn ResetBlockReceivedFlags();
    fn ResetSpriteData();
    fn ResetTasks();
    fn RunTasks();
    fn RunTextPrinters();
    fn SendBlock(a0: u8, a1: *mut u8, a2: u16) -> u8;
    fn SendBlockRequest(a0: u8) -> u8;
    fn SetBerryBlenderLinkCallback();
    fn SetBgAffine(a0: u8, a1: i32, a2: i32, a3: i16, a4: i16, a5: i16, a6: i16, a7: u16);
    fn SetBgTilemapBuffer(a0: u8, a1: *mut u8);
    fn SetCloseLinkCallback();
    fn SetGpuReg(a0: u8, a1: u16);
    fn SetGpuRegBits(a0: u8, a1: u16);
    fn SetLinkDebugValues(a0: u32, a1: u32);
    fn SetLinkStandbyCallback();
    fn SetMainCallback2(a0: Option<unsafe extern "C" fn()>);
    fn SetVBlankCallback(a0: Option<unsafe extern "C" fn()>);
    fn SetWirelessCommType0();
    fn ShowBg(a0: u8);
    fn SpriteCallbackDummy(a0: *mut u8);
    fn StartSpriteAnim(a0: *mut u8, a1: u8);
    fn StringAppend(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn StringCompare(a0: *mut u8, a1: *mut u8) -> i32;
    fn StringCopy(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn TransferPlttBuffer();
    fn UnsetBgTilemapBuffer(a0: u8);
    fn UpdatePaletteFade() -> u8;
    fn WriteSaveBlock1Sector() -> u8;
    fn WriteSaveBlock2() -> u8;
    fn m4aMPlayPitchControl(a0: *mut u8, a1: u16, a2: i16);
    fn m4aMPlayStop(a0: *mut u8);
    fn m4aMPlayTempoControl(a0: *mut u8, a1: u16);
}

pub(crate) unsafe extern "C" fn UpdateHitPitch() {
    unsafe {
        m4aMPlayPitchControl(
            (&raw mut gMPlayInfo_SE2).cast::<u8>(),
            65535u16,
            (((2i32).wrapping_mul(
                ((((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(76)
                    .cast::<i16>())
                .read()) as i32)
                    .wrapping_sub(128i32),
            )) as i16),
        );
    }
}
pub(crate) unsafe extern "C" fn VBlankCB_BerryBlender() {
    unsafe {
        SetBgPos();
        SetBgAffine(
            2u8,
            (((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(320))
            .cast::<i32>())
            .read(),
            (((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(320))
            .wrapping_add(4)
            .cast::<i32>())
            .read(),
            (((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(320))
            .wrapping_add(8)
            .cast::<i16>())
            .read(),
            (((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(320))
            .wrapping_add(10)
            .cast::<i16>())
            .read(),
            (((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(320))
            .wrapping_add(12)
            .cast::<i16>())
            .read(),
            (((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(320))
            .wrapping_add(14)
            .cast::<i16>())
            .read(),
            (((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(320))
            .wrapping_add(16)
            .cast::<u16>())
            .read(),
        );
        LoadOam();
        ProcessSpriteCopyRequests();
        TransferPlttBuffer();
    }
}
pub(crate) unsafe extern "C" fn LoadBerryBlenderGfx() -> u8 {
    unsafe {
        'l1: {
            let __sw1 = ((((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(1))
            .read()) as i32);
            if __sw1 == 0i32 {
                ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(4540)
                    .cast::<*mut u8>())
                .write(AllocZeroed(
                    (GetDecompressedDataSize(
                        ((&raw mut gBerryBlenderCenter_Gfx).cast::<u32>()).cast::<u32>(),
                    ))
                    .wrapping_add(100u32),
                ));
                LZDecompressWram(
                    ((&raw mut gBerryBlenderCenter_Gfx).cast::<u32>()).cast::<u32>(),
                    ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(4540)
                        .cast::<*mut u8>())
                    .read(),
                );
                let __p2 = (((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(1);
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                CopyToBgTilemapBuffer(
                    2u8,
                    ((&raw const sBlenderCenter_Tilemap).cast::<u8>().cast_mut()).cast::<u8>(),
                    1024u16,
                    0u16,
                );
                CopyBgTilemapBufferToVram(2u8);
                LoadPalette(
                    (((&raw const sBlenderCenter_Pal)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<u16>())
                    .cast::<u16>())
                    .cast::<u8>(),
                    0u16,
                    256u16,
                );
                let __p3 = (((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(1);
                (__p3).write(((__p3).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 2i32 {
                LoadBgTiles(
                    2u8,
                    ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(4540)
                        .cast::<*mut u8>())
                    .read(),
                    ((GetDecompressedDataSize(
                        ((&raw mut gBerryBlenderCenter_Gfx).cast::<u32>()).cast::<u32>(),
                    )) as u16),
                    0u16,
                );
                let __p4 = (((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(1);
                (__p4).write(((__p4).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 3i32 {
                LZDecompressWram(
                    ((&raw mut gBerryBlenderOuter_Gfx).cast::<u32>()).cast::<u32>(),
                    ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(4540)
                        .cast::<*mut u8>())
                    .read(),
                );
                let __p5 = (((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(1);
                (__p5).write(((__p5).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 4i32 {
                LoadBgTiles(
                    1u8,
                    ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(4540)
                        .cast::<*mut u8>())
                    .read(),
                    ((GetDecompressedDataSize(
                        ((&raw mut gBerryBlenderOuter_Gfx).cast::<u32>()).cast::<u32>(),
                    )) as u16),
                    0u16,
                );
                let __p6 = (((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(1);
                (__p6).write(((__p6).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 5i32 {
                LZDecompressWram(
                    ((&raw mut gBerryBlenderOuter_Tilemap).cast::<u32>()).cast::<u32>(),
                    ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(4540)
                        .cast::<*mut u8>())
                    .read(),
                );
                let __p7 = (((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(1);
                (__p7).write(((__p7).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 6i32 {
                CopyToBgTilemapBuffer(
                    1u8,
                    ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(4540)
                        .cast::<*mut u8>())
                    .read(),
                    ((GetDecompressedDataSize(
                        ((&raw mut gBerryBlenderOuter_Tilemap).cast::<u32>()).cast::<u32>(),
                    )) as u16),
                    0u16,
                );
                CopyBgTilemapBufferToVram(1u8);
                let __p8 = (((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(1);
                (__p8).write(((__p8).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 7i32 {
                LoadPalette(
                    (((&raw const sBlenderOuter_Pal)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<u16>())
                    .cast::<u16>())
                    .cast::<u8>(),
                    128u16,
                    32u16,
                );
                let __p9 = (((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(1);
                (__p9).write(((__p9).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 8i32 {
                LoadSpriteSheet(
                    (&raw const sSpriteSheet_PlayerArrow)
                        .cast::<u8>()
                        .cast_mut(),
                );
                LoadSpriteSheet((&raw const sSpriteSheet_Particles).cast::<u8>().cast_mut());
                LoadSpriteSheet(
                    (&raw const sSpriteSheet_ScoreSymbols)
                        .cast::<u8>()
                        .cast_mut(),
                );
                let __p10 = (((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(1);
                (__p10).write(((__p10).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 9i32 {
                LoadSpriteSheet(
                    (&raw const sSpriteSheet_CountdownNumbers)
                        .cast::<u8>()
                        .cast_mut(),
                );
                LoadSpriteSheet((&raw const sSpriteSheet_Start).cast::<u8>().cast_mut());
                LoadSpritePalette((&raw const sSpritePal_PlayerArrow).cast::<u8>().cast_mut());
                LoadSpritePalette((&raw const sSpritePal_BlenderMisc).cast::<u8>().cast_mut());
                Free(
                    ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(4540)
                        .cast::<*mut u8>())
                    .read(),
                );
                ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(1))
                .write(0u8);
                return 1u8;
            }
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn DrawBlenderBg() {
    unsafe {
        FillBgTilemapBufferRect_Palette0(
            0u8,
            0u16,
            0u8,
            0u8,
            ((crate::c::div_i32(240i32, 8i32)) as u8),
            ((crate::c::div_i32(160i32, 8i32)) as u8),
        );
        CopyBgTilemapBufferToVram(0u8);
        ShowBg(0u8);
        ShowBg(1u8);
        SetGpuRegBits(0u8, 4160u16);
        ChangeBgX(0u8, 0i32, 0u8);
        ChangeBgY(0u8, 0i32, 0u8);
        ChangeBgX(1u8, 0i32, 0u8);
        ChangeBgY(1u8, 0i32, 0u8);
    }
}
pub(crate) unsafe extern "C" fn InitBerryBlenderWindows() {
    unsafe {
        if (InitWindows(((&raw const sWindowTemplates).cast::<u8>().cast_mut()).cast::<u8>())) != 0
        {
            let mut i: i32 = 0i32;
            DeactivateAllTextPrinters();
            {
                i = 0i32;
                'l1: loop {
                    if !(i < 5i32) {
                        break 'l1;
                    }
                    'l2: {
                        FillWindowPixelBuffer(((i) as u8), 0u8);
                    }
                    i = (i).wrapping_add(1);
                }
            }
            FillBgTilemapBufferRect_Palette0(
                0u8,
                0u16,
                0u8,
                0u8,
                ((crate::c::div_i32(240i32, 8i32)) as u8),
                ((crate::c::div_i32(160i32, 8i32)) as u8),
            );
            Menu_LoadStdPalAt(224u16);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DoBerryBlending() {
    unsafe {
        if ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read()) as usize) == 0usize {
            ((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).write(AllocZeroed(4576u32));
        }
        ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(99))
            .write(0u8);
        (((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read()).write(0u8);
        ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(99))
            .write(0u8);
        InitLocalPlayers(((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as u8));
        SetMainCallback2(Some(CB2_LoadBerryBlender));
    }
}
pub(crate) unsafe extern "C" fn CB2_LoadBerryBlender() {
    unsafe {
        let mut i: i32 = 0i32;
        'l1: {
            let __sw1 = (((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read()).read())
                as i32);
            if __sw1 == 0i32 {
                SetGpuReg(0u8, 0u16);
                ResetSpriteData();
                FreeAllSpritePalettes();
                SetVBlankCallback(None);
                ResetBgsAndClearDma3BusyFlags(0u32);
                InitBgsFromTemplates(
                    1u8,
                    ((&raw const sBgTemplates).cast::<u8>().cast_mut()).cast::<u8>(),
                    ((crate::c::div_u32(12u32, 4u32)) as u8),
                );
                SetBgTilemapBuffer(
                    1u8,
                    (((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(440))
                    .cast::<u8>())
                    .cast::<u8>(),
                );
                SetBgTilemapBuffer(
                    2u8,
                    ((((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(440))
                    .cast::<u8>())
                    .wrapping_offset(2048))
                    .cast::<u8>(),
                );
                LoadUserWindowBorderGfx(0u8, 1u16, 208u8);
                LoadMessageBoxGfx(0u8, 20u16, 240u8);
                InitBerryBlenderWindows();
                let __p2 = (((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read());
                (__p2).write(((__p2).read()).wrapping_add(1));
                ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(280)
                    .cast::<u16>())
                .write(0u16);
                ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(278)
                    .cast::<u16>())
                .write(0u16);
                ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(282)
                    .cast::<u16>())
                .write(80u16);
                ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(284)
                    .cast::<u16>())
                .write(0u16);
                ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(286)
                    .cast::<u16>())
                .write(0u16);
                ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(1))
                .write(0u8);
                UpdateBlenderCenter();
                break 'l1;
            }
            if __sw1 == 1i32 {
                if (LoadBerryBlenderGfx()) != 0 {
                    {
                        i = 0i32;
                        'l2: loop {
                            if !(i < 4i32) {
                                break 'l2;
                            }
                            'l3: {
                                ((((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(80))
                                .cast::<u8>())
                                .wrapping_offset((i) as isize))
                                .write(CreateSprite(
                                    (&raw const sSpriteTemplate_PlayerArrow)
                                        .cast::<u8>()
                                        .cast_mut(),
                                    (((((((&raw const sPlayerArrowPos).cast::<u8>().cast_mut())
                                        .cast::<u8>())
                                    .wrapping_offset((i) as isize * 2))
                                    .cast::<u8>())
                                    .read()) as i16),
                                    ((((((((&raw const sPlayerArrowPos)
                                        .cast::<u8>()
                                        .cast_mut())
                                    .cast::<u8>())
                                    .wrapping_offset((i) as isize * 2))
                                    .cast::<u8>())
                                    .wrapping_offset(1))
                                    .read()) as i16),
                                    1u8,
                                ));
                                StartSpriteAnim(
                                    ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                        ((((((((&raw mut sBerryBlender)
                                            .cast::<u8>()
                                            .cast::<*mut u8>())
                                        .read())
                                        .wrapping_add(80))
                                        .cast::<u8>())
                                        .wrapping_offset((i) as isize))
                                        .read()) as i32)
                                            as isize
                                            * 68,
                                    ),
                                    (((i).wrapping_add(8i32)) as u8),
                                );
                            }
                            i = (i).wrapping_add(1);
                        }
                    }
                    if ((((&raw mut gReceivedRemoteLinkPlayers).cast::<u8>()).read()) != 0)
                        && ((((&raw mut gWirelessCommType).cast::<u8>()).read()) != 0)
                    {
                        LoadWirelessStatusIndicatorSpriteGfx();
                        CreateWirelessStatusIndicatorSprite(0u8, 0u8);
                    }
                    SetVBlankCallback(Some(VBlankCB_BerryBlender));
                    let __p3 = (((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read());
                    (__p3).write(((__p3).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                BeginNormalPaletteFade(4294967295u32, 0i8, 16u8, 0u8, 0u16);
                UpdateBlenderCenter();
                let __p4 = (((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read());
                (__p4).write(((__p4).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 3i32 {
                DrawBlenderBg();
                if !((crate::c::bf_read(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
                    7,
                    1,
                    false,
                ) as u16)
                    != 0)
                {
                    let __p5 = (((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read());
                    (__p5).write(((__p5).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 4i32 {
                if (PrintMessage(
                    (((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(4536)
                        .cast::<i16>(),
                    ((&raw const sText_BerryBlenderStart).cast::<u8>().cast_mut()).cast::<u8>(),
                    ((GetPlayerTextSpeedDelay()) as i32),
                )) != 0
                {
                    let __p6 = (((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read());
                    (__p6).write(((__p6).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 5i32 {
                BeginNormalPaletteFade(4294967295u32, 0i8, 0u8, 16u8, 0u16);
                let __p7 = (((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read());
                (__p7).write(((__p7).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 6i32 {
                if !((crate::c::bf_read(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
                    7,
                    1,
                    false,
                ) as u16)
                    != 0)
                {
                    FreeAllWindowBuffers();
                    UnsetBgTilemapBuffer(2u8);
                    UnsetBgTilemapBuffer(1u8);
                    SetVBlankCallback(None);
                    ChooseBerryForMachine(Some(StartBlender));
                    (((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read()).write(0u8);
                }
                break 'l1;
            }
        }
        AnimateSprites();
        BuildOamBuffer();
        RunTextPrinters();
        UpdatePaletteFade();
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_Berry(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
        (__p1).write(
            (((((__p1).read()) as i32).wrapping_add(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read()) as i32),
            )) as i16),
        );
        let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
        (__p2).write(
            (((((__p2).read()) as i32).wrapping_sub(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read()) as i32),
            )) as i16),
        );
        let __p3 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
        (__p3).write(
            (((((__p3).read()) as i32).wrapping_add(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read()) as i32),
            )) as i16),
        );
        let __p4 = ((sprite).wrapping_add(46)).cast::<i16>();
        (__p4).write(
            (((((__p4).read()) as i32).wrapping_add(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read()) as i32),
            )) as i16),
        );
        let __p5 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4);
        (__p5).write(((__p5).read()).wrapping_sub(1));
        if (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32)
            < ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
        {
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write({
                let __v6 = ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3))
                    .read()) as i32)
                    .wrapping_sub(1i32)) as i16);
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(__v6);
                __v6
            });
            if (({
                let __p7 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5);
                let __t8 = ((__p7).read()).wrapping_add(1);
                (__p7).write(__t8);
                __t8
            }) as i32)
                > 3i32
            {
                DestroySprite(sprite);
            } else {
                PlaySE(116u16);
            }
        }
        ((sprite).wrapping_add(32).cast::<i16>())
            .write(((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read());
        ((sprite).wrapping_add(34).cast::<i16>())
            .write(((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read());
    }
}
pub(crate) unsafe extern "C" fn SetBerrySpriteData(
    sprite: *mut u8,
    x: i16,
    y: i16,
    bounceSpeed: i16,
    xSpeed: i16,
    ySpeed: i16,
) {
    unsafe {
        let mut sprite = sprite;
        let mut x = x;
        let mut y = y;
        let mut bounceSpeed = bounceSpeed;
        let mut xSpeed = xSpeed;
        let mut ySpeed = ySpeed;
        (((sprite).wrapping_add(46)).cast::<i16>()).write(y);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(x);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(y);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(bounceSpeed);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(10i16);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write(0i16);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).write(xSpeed);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(ySpeed);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(SpriteCB_Berry));
    }
}
pub(crate) unsafe extern "C" fn CreateBerrySprite(itemId: u16, playerId: u8) {
    unsafe {
        let mut itemId = itemId;
        let mut playerId = playerId;
        let mut spriteId: u8 = CreateSpinningBerrySprite(
            ((((((itemId) as i32).wrapping_sub(133i32)).wrapping_add(1i32)).wrapping_sub(1i32))
                as u8),
            0u8,
            80u8,
            ((((playerId) as i32) & 1i32) as u8),
        );
        SetBerrySpriteData(
            ((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68),
            (((((&raw const sBerrySpriteData).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((playerId) as i32) as isize * 10))
            .cast::<i16>())
            .read(),
            ((((((&raw const sBerrySpriteData).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((playerId) as i32) as isize * 10))
            .cast::<i16>())
            .wrapping_offset(1))
            .read(),
            ((((((&raw const sBerrySpriteData).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((playerId) as i32) as isize * 10))
            .cast::<i16>())
            .wrapping_offset(2))
            .read(),
            ((((((&raw const sBerrySpriteData).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((playerId) as i32) as isize * 10))
            .cast::<i16>())
            .wrapping_offset(3))
            .read(),
            ((((((&raw const sBerrySpriteData).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((playerId) as i32) as isize * 10))
            .cast::<i16>())
            .wrapping_offset(4))
            .read(),
        );
    }
}
pub(crate) unsafe extern "C" fn ConvertItemToBlenderBerry(berry: *mut u8, itemId: u16) {
    unsafe {
        let mut berry = berry;
        let mut itemId = itemId;
        let mut berryInfo: *mut u8 =
            GetBerryInfo((((((itemId) as i32).wrapping_sub(133i32)).wrapping_add(1i32)) as u8));
        ((berry).cast::<u16>()).write(itemId);
        StringCopy(
            ((berry).wrapping_add(2)).cast::<u8>(),
            (berryInfo).cast::<u8>(),
        );
        (((berry).wrapping_add(9)).cast::<u8>()).write(((berryInfo).wrapping_add(21)).read());
        ((((berry).wrapping_add(9)).cast::<u8>()).wrapping_offset(1))
            .write(((berryInfo).wrapping_add(22)).read());
        ((((berry).wrapping_add(9)).cast::<u8>()).wrapping_offset(2))
            .write(((berryInfo).wrapping_add(23)).read());
        ((((berry).wrapping_add(9)).cast::<u8>()).wrapping_offset(3))
            .write(((berryInfo).wrapping_add(24)).read());
        ((((berry).wrapping_add(9)).cast::<u8>()).wrapping_offset(4))
            .write(((berryInfo).wrapping_add(25)).read());
        ((((berry).wrapping_add(9)).cast::<u8>()).wrapping_offset(5))
            .write(((berryInfo).wrapping_add(26)).read());
    }
}
pub(crate) unsafe extern "C" fn InitLocalPlayers(opponentsNum: u8) {
    unsafe {
        let mut opponentsNum = opponentsNum;
        'l1: {
            let __sw1 = ((opponentsNum) as i32);
            if __sw1 == 0i32 {
                ((&raw mut gInGameOpponentsNo).cast::<u8>().cast::<u8>()).write(0u8);
                break 'l1;
            }
            if __sw1 == 1i32 {
                ((&raw mut gInGameOpponentsNo).cast::<u8>().cast::<u8>()).write(1u8);
                ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(124))
                .write(2u8);
                StringCopy(
                    (((&raw mut gLinkPlayers).cast::<u8>()).wrapping_add(8)).cast::<u8>(),
                    (((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).cast::<u8>(),
                );
                if !((FlagGet(832u16)) != 0) {
                    StringCopy(
                        ((((&raw mut gLinkPlayers).cast::<u8>()).wrapping_offset(28))
                            .wrapping_add(8))
                        .cast::<u8>(),
                        ((((&raw const sBlenderOpponentsNames)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<*mut u8>())
                        .cast::<*mut u8>())
                        .wrapping_offset(3))
                        .read(),
                    );
                } else {
                    StringCopy(
                        ((((&raw mut gLinkPlayers).cast::<u8>()).wrapping_offset(28))
                            .wrapping_add(8))
                        .cast::<u8>(),
                        (((&raw const sBlenderOpponentsNames)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<*mut u8>())
                        .cast::<*mut u8>())
                        .read(),
                    );
                }
                (((&raw mut gLinkPlayers).cast::<u8>())
                    .wrapping_add(26)
                    .cast::<u16>())
                .write(2u16);
                ((((&raw mut gLinkPlayers).cast::<u8>()).wrapping_offset(28))
                    .wrapping_add(26)
                    .cast::<u16>())
                .write(2u16);
                break 'l1;
            }
            if __sw1 == 2i32 {
                ((&raw mut gInGameOpponentsNo).cast::<u8>().cast::<u8>()).write(2u8);
                ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(124))
                .write(3u8);
                StringCopy(
                    (((&raw mut gLinkPlayers).cast::<u8>()).wrapping_add(8)).cast::<u8>(),
                    (((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).cast::<u8>(),
                );
                StringCopy(
                    ((((&raw mut gLinkPlayers).cast::<u8>()).wrapping_offset(28)).wrapping_add(8))
                        .cast::<u8>(),
                    ((((&raw const sBlenderOpponentsNames)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<*mut u8>())
                    .cast::<*mut u8>())
                    .wrapping_offset(4))
                    .read(),
                );
                StringCopy(
                    ((((&raw mut gLinkPlayers).cast::<u8>()).wrapping_offset(56)).wrapping_add(8))
                        .cast::<u8>(),
                    ((((&raw const sBlenderOpponentsNames)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<*mut u8>())
                    .cast::<*mut u8>())
                    .wrapping_offset(2))
                    .read(),
                );
                (((&raw mut gLinkPlayers).cast::<u8>())
                    .wrapping_add(26)
                    .cast::<u16>())
                .write(2u16);
                ((((&raw mut gLinkPlayers).cast::<u8>()).wrapping_offset(28))
                    .wrapping_add(26)
                    .cast::<u16>())
                .write(2u16);
                ((((&raw mut gLinkPlayers).cast::<u8>()).wrapping_offset(56))
                    .wrapping_add(26)
                    .cast::<u16>())
                .write(2u16);
                break 'l1;
            }
            if __sw1 == 3i32 {
                ((&raw mut gInGameOpponentsNo).cast::<u8>().cast::<u8>()).write(3u8);
                ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(124))
                .write(4u8);
                StringCopy(
                    (((&raw mut gLinkPlayers).cast::<u8>()).wrapping_add(8)).cast::<u8>(),
                    (((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).cast::<u8>(),
                );
                StringCopy(
                    ((((&raw mut gLinkPlayers).cast::<u8>()).wrapping_offset(28)).wrapping_add(8))
                        .cast::<u8>(),
                    ((((&raw const sBlenderOpponentsNames)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<*mut u8>())
                    .cast::<*mut u8>())
                    .wrapping_offset(5))
                    .read(),
                );
                StringCopy(
                    ((((&raw mut gLinkPlayers).cast::<u8>()).wrapping_offset(56)).wrapping_add(8))
                        .cast::<u8>(),
                    ((((&raw const sBlenderOpponentsNames)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<*mut u8>())
                    .cast::<*mut u8>())
                    .wrapping_offset(1))
                    .read(),
                );
                StringCopy(
                    ((((&raw mut gLinkPlayers).cast::<u8>()).wrapping_offset(84)).wrapping_add(8))
                        .cast::<u8>(),
                    ((((&raw const sBlenderOpponentsNames)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<*mut u8>())
                    .cast::<*mut u8>())
                    .wrapping_offset(2))
                    .read(),
                );
                (((&raw mut gLinkPlayers).cast::<u8>())
                    .wrapping_add(26)
                    .cast::<u16>())
                .write(2u16);
                ((((&raw mut gLinkPlayers).cast::<u8>()).wrapping_offset(28))
                    .wrapping_add(26)
                    .cast::<u16>())
                .write(2u16);
                ((((&raw mut gLinkPlayers).cast::<u8>()).wrapping_offset(56))
                    .wrapping_add(26)
                    .cast::<u16>())
                .write(2u16);
                ((((&raw mut gLinkPlayers).cast::<u8>()).wrapping_offset(84))
                    .wrapping_add(26)
                    .cast::<u16>())
                .write(2u16);
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn StartBlender() {
    unsafe {
        let mut i: i32 = 0i32;
        SetGpuReg(0u8, 0u16);
        if ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read()) as usize) == 0usize {
            ((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).write(AllocZeroed(4576u32));
        }
        (((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read()).write(0u8);
        ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(268)
            .cast::<u32>())
        .write(0u32);
        {
            i = 0i32;
            'l1: loop {
                if !(i < 4i32) {
                    break 'l1;
                }
                'l2: {
                    ((((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(116))
                    .cast::<u16>())
                    .wrapping_offset((i) as isize))
                    .write(0u16);
                }
                i = (i).wrapping_add(1);
            }
        }
        InitLocalPlayers(((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as u8));
        if ((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32) == 0i32 {
            SetMainCallback2(Some(CB2_StartBlenderLink));
        } else {
            SetMainCallback2(Some(CB2_StartBlenderLocal));
        }
    }
}
pub(crate) unsafe extern "C" fn CB2_StartBlenderLink() {
    unsafe {
        let mut i: i32 = 0i32;
        let mut j: i32 = 0i32;
        'l1: {
            let __sw1 = (((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read()).read())
                as i32);
            if __sw1 == 0i32 {
                InitBlenderBgs();
                ((&raw mut gLinkType).cast::<u16>()).write(17442u16);
                ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(114))
                .write(0u8);
                {
                    i = 0i32;
                    'l2: loop {
                        if !(i < 4i32) {
                            break 'l2;
                        }
                        'l3: {
                            ((((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(100))
                            .cast::<u16>())
                            .wrapping_offset((i) as isize))
                            .write(0u16);
                            {
                                j = 0i32;
                                'l4: loop {
                                    if !(j < 3i32) {
                                        break 'l4;
                                    }
                                    'l5: {
                                        ((((((((&raw mut sBerryBlender)
                                            .cast::<u8>()
                                            .cast::<*mut u8>())
                                        .read())
                                        .wrapping_add(292))
                                        .cast::<u8>())
                                        .wrapping_offset((i) as isize * 6))
                                        .cast::<u16>())
                                        .wrapping_offset((j) as isize))
                                        .write(0u16);
                                    }
                                    j = (j).wrapping_add(1);
                                }
                            }
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(112)
                    .cast::<u16>())
                .write(0u16);
                ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(78)
                    .cast::<u16>())
                .write(0u16);
                ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(1))
                .write(0u8);
                let __p2 = (((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read());
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                if (LoadBerryBlenderGfx()) != 0 {
                    let __p3 = (((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read());
                    (__p3).write(((__p3).read()).wrapping_add(1));
                    UpdateBlenderCenter();
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                {
                    i = 0i32;
                    'l6: loop {
                        if !(i < 4i32) {
                            break 'l6;
                        }
                        'l7: {
                            ((((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(84))
                            .cast::<u8>())
                            .wrapping_offset((i) as isize))
                            .write(CreateSprite(
                                (&raw const sSpriteTemplate_PlayerArrow)
                                    .cast::<u8>()
                                    .cast_mut(),
                                (((((((&raw const sPlayerArrowPos).cast::<u8>().cast_mut())
                                    .cast::<u8>())
                                .wrapping_offset((i) as isize * 2))
                                .cast::<u8>())
                                .read()) as i16),
                                ((((((((&raw const sPlayerArrowPos).cast::<u8>().cast_mut())
                                    .cast::<u8>())
                                .wrapping_offset((i) as isize * 2))
                                .cast::<u8>())
                                .wrapping_offset(1))
                                .read()) as i16),
                                1u8,
                            ));
                            StartSpriteAnim(
                                ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                    ((((((((&raw mut sBerryBlender)
                                        .cast::<u8>()
                                        .cast::<*mut u8>())
                                    .read())
                                    .wrapping_add(84))
                                    .cast::<u8>())
                                    .wrapping_offset((i) as isize))
                                    .read()) as i32) as isize
                                        * 68,
                                ),
                                (((i).wrapping_add(8i32)) as u8),
                            );
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                if ((((&raw mut gReceivedRemoteLinkPlayers).cast::<u8>()).read()) != 0)
                    && ((((&raw mut gWirelessCommType).cast::<u8>()).read()) != 0)
                {
                    LoadWirelessStatusIndicatorSpriteGfx();
                    CreateWirelessStatusIndicatorSprite(0u8, 0u8);
                }
                let __p4 = (((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read());
                (__p4).write(((__p4).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 3i32 {
                BeginNormalPaletteFade(4294967295u32, 0i8, 16u8, 0u8, 0u16);
                let __p5 = (((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read());
                (__p5).write(((__p5).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 4i32 {
                DrawBlenderBg();
                if !((crate::c::bf_read(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
                    7,
                    1,
                    false,
                ) as u16)
                    != 0)
                {
                    let __p6 = (((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read());
                    (__p6).write(((__p6).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 5i32 {
                PrintMessage(
                    (((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(4536)
                        .cast::<i16>(),
                    ((&raw const sText_CommunicationStandby)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>(),
                    0i32,
                );
                (((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read()).write(8u8);
                ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(264)
                    .cast::<i32>())
                .write(0i32);
                break 'l1;
            }
            if __sw1 == 8i32 {
                let __p7 = (((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read());
                (__p7).write(((__p7).read()).wrapping_add(1));
                ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(276))
                .write(0u8);
                ConvertItemToBlenderBerry(
                    ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(344))
                    .cast::<u8>(),
                    ((&raw mut gSpecialVar_ItemId).cast::<u16>()).read(),
                );
                crate::c::memcpy(
                    (&raw mut gBlockSendBuffer).cast::<u8>(),
                    ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(344))
                    .cast::<u8>(),
                    16u32,
                );
                SetLinkStandbyCallback();
                ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(264)
                    .cast::<i32>())
                .write(0i32);
                break 'l1;
            }
            if __sw1 == 9i32 {
                if (IsLinkTaskFinished()) != 0 {
                    ResetBlockReceivedFlags();
                    if ((GetMultiplayerId()) as i32) == 0i32 {
                        SendBlockRequest(4u8);
                    }
                    let __p8 = (((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read());
                    (__p8).write(((__p8).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 10i32 {
                if {
                    let __p9 = (((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(264)
                        .cast::<i32>();
                    let __t10 = ((__p9).read()).wrapping_add(1);
                    (__p9).write(__t10);
                    __t10
                } > 20i32
                {
                    ClearDialogWindowAndFrameToTransparent(4u8, 1u8);
                    if ((GetBlockReceivedStatus()) as i32)
                        == ((GetLinkPlayerCountAsBitFlags()) as i32)
                    {
                        {
                            i = 0i32;
                            'l8: loop {
                                if !(i < ((GetLinkPlayerCount()) as i32)) {
                                    break 'l8;
                                }
                                'l9: {
                                    crate::c::memcpy(
                                        (((((&raw mut sBerryBlender)
                                            .cast::<u8>()
                                            .cast::<*mut u8>())
                                        .read())
                                        .wrapping_add(344))
                                        .cast::<u8>())
                                        .wrapping_offset((i) as isize * 16),
                                        ((((&raw mut gBlockRecvBuffer).cast::<u8>())
                                            .wrapping_offset((i) as isize * 256))
                                        .cast::<u16>())
                                        .cast::<u8>(),
                                        16u32,
                                    );
                                    ((((((&raw mut sBerryBlender)
                                        .cast::<u8>()
                                        .cast::<*mut u8>())
                                    .read())
                                    .wrapping_add(116))
                                    .cast::<u16>())
                                    .wrapping_offset((i) as isize))
                                    .write(
                                        (((((((&raw mut sBerryBlender)
                                            .cast::<u8>()
                                            .cast::<*mut u8>())
                                        .read())
                                        .wrapping_add(344))
                                        .cast::<u8>())
                                        .wrapping_offset((i) as isize * 16))
                                        .cast::<u16>())
                                        .read(),
                                    );
                                }
                                i = (i).wrapping_add(1);
                            }
                        }
                        ResetBlockReceivedFlags();
                        let __p11 =
                            (((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read());
                        (__p11).write(((__p11).read()).wrapping_add(1));
                    }
                }
                break 'l1;
            }
            if __sw1 == 11i32 {
                ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(124))
                .write(GetLinkPlayerCount());
                {
                    i = 0i32;
                    'l10: loop {
                        if !(i < 4i32) {
                            break 'l10;
                        }
                        'l11: {
                            if ((((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(276))
                            .read()) as i32)
                                == ((((((((&raw const sPlayerIdMap).cast::<u8>().cast_mut())
                                    .cast::<u8>())
                                .wrapping_offset(
                                    (((((((&raw mut sBerryBlender)
                                        .cast::<u8>()
                                        .cast::<*mut u8>())
                                    .read())
                                    .wrapping_add(124))
                                    .read()) as i32)
                                        .wrapping_sub(2i32))
                                        as isize
                                        * 4,
                                ))
                                .cast::<u8>())
                                .wrapping_offset((i) as isize))
                                .read()) as i32)
                            {
                                CreateBerrySprite(
                                    ((((((&raw mut sBerryBlender)
                                        .cast::<u8>()
                                        .cast::<*mut u8>())
                                    .read())
                                    .wrapping_add(116))
                                    .cast::<u16>())
                                    .wrapping_offset(
                                        ((((((&raw mut sBerryBlender)
                                            .cast::<u8>()
                                            .cast::<*mut u8>())
                                        .read())
                                        .wrapping_add(276))
                                        .read()) as i32)
                                            as isize,
                                    ))
                                    .read(),
                                    ((i) as u8),
                                );
                                break 'l10;
                            }
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(264)
                    .cast::<i32>())
                .write(0i32);
                let __p12 = (((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read());
                (__p12).write(((__p12).read()).wrapping_add(1));
                let __p13 = (((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(276);
                (__p13).write(((__p13).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 12i32 {
                if {
                    let __p14 = (((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(264)
                        .cast::<i32>();
                    let __t15 = ((__p14).read()).wrapping_add(1);
                    (__p14).write(__t15);
                    __t15
                } > 60i32
                {
                    if ((((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(276))
                    .read()) as i32)
                        >= ((((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(124))
                        .read()) as i32)
                    {
                        let __p16 =
                            (((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read());
                        (__p16).write(((__p16).read()).wrapping_add(1));
                        ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(74)
                            .cast::<u16>())
                        .write(
                            ((((((((&raw const sArrowStartPos)
                                .cast::<u8>()
                                .cast_mut()
                                .cast::<u16>())
                            .cast::<u16>())
                            .wrapping_offset(
                                ((((((&raw const sArrowStartPosIds).cast::<u8>().cast_mut())
                                    .cast::<u8>())
                                .wrapping_offset(
                                    (((((((&raw mut sBerryBlender)
                                        .cast::<u8>()
                                        .cast::<*mut u8>())
                                    .read())
                                    .wrapping_add(124))
                                    .read()) as i32)
                                        .wrapping_sub(2i32))
                                        as isize,
                                ))
                                .read()) as i32) as isize,
                            ))
                            .read()) as i32)
                                .wrapping_sub(22528i32)) as u16),
                        );
                    } else {
                        let __p17 =
                            (((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read());
                        (__p17).write(((__p17).read()).wrapping_sub(1));
                    }
                    ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(264)
                        .cast::<i32>())
                    .write(0i32);
                }
                break 'l1;
            }
            if __sw1 == 13i32 {
                if (IsLinkTaskFinished()) != 0 {
                    let __p18 = (((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read());
                    (__p18).write(((__p18).read()).wrapping_add(1));
                    DrawBlenderCenter(
                        (((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(320),
                    );
                    PlaySE(43u16);
                    ShowBg(2u8);
                }
                break 'l1;
            }
            if __sw1 == 14i32 {
                SetGpuRegBits(0u8, 1024u16);
                let __p19 = (((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(74)
                    .cast::<u16>();
                (__p19).write((((((__p19).read()) as i32).wrapping_add(512i32)) as u16));
                let __p20 = (((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(282)
                    .cast::<u16>();
                (__p20).write((((((__p20).read()) as i32).wrapping_add(4i32)) as u16));
                if ((((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(282)
                    .cast::<u16>())
                .read()) as i32)
                    > 255i32
                {
                    SetGpuRegBits(12u8, 2u16);
                    let __p21 = (((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read());
                    (__p21).write(((__p21).read()).wrapping_add(1));
                    ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(282)
                        .cast::<u16>())
                    .write(256u16);
                    ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(74)
                        .cast::<u16>())
                    .write(
                        ((((&raw const sArrowStartPos)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<u16>())
                        .cast::<u16>())
                        .wrapping_offset(
                            ((((((&raw const sArrowStartPosIds).cast::<u8>().cast_mut())
                                .cast::<u8>())
                            .wrapping_offset(
                                (((((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(124))
                                .read()) as i32)
                                    .wrapping_sub(2i32)) as isize,
                            ))
                            .read()) as i32) as isize,
                        ))
                        .read(),
                    );
                    ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(264)
                        .cast::<i32>())
                    .write(0i32);
                    PlaySE(52u16);
                    SetPlayerIdMaps();
                    PrintPlayerNames();
                }
                DrawBlenderCenter(
                    (((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(320),
                );
                break 'l1;
            }
            if __sw1 == 15i32 {
                if (UpdateBlenderLandScreenShake()) != 0 {
                    ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(264)
                        .cast::<i32>())
                    .write(0i32);
                    let __p22 = (((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read());
                    (__p22).write(((__p22).read()).wrapping_add(1));
                }
                DrawBlenderCenter(
                    (((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(320),
                );
                break 'l1;
            }
            if __sw1 == 16i32 {
                CreateSprite(
                    (&raw const sSpriteTemplate_CountdownNumbers)
                        .cast::<u8>()
                        .cast_mut(),
                    120i16,
                    (-16i16),
                    3u8,
                );
                let __p23 = (((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read());
                (__p23).write(((__p23).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 17i32 {
                break 'l1;
            }
            if __sw1 == 18i32 {
                let __p24 = (((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read());
                (__p24).write(((__p24).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 19i32 {
                SetLinkStandbyCallback();
                let __p25 = (((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read());
                (__p25).write(((__p25).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 20i32 {
                if (IsLinkTaskFinished()) != 0 {
                    SetBerryBlenderLinkCallback();
                    let __p26 = (((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read());
                    (__p26).write(((__p26).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 21i32 {
                ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(76)
                    .cast::<i16>())
                .write(128i16);
                ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(260)
                    .cast::<u32>())
                .write(0u32);
                SetMainCallback2(Some(CB2_PlayBlender));
                if ((GetCurrentMapMusic()) as i32) != 403i32 {
                    ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(340)
                        .cast::<u16>())
                    .write(GetCurrentMapMusic());
                }
                PlayBGM(403u16);
                break 'l1;
            }
        }
        Blender_DummiedOutFunc(
            ((((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(284)
                .cast::<u16>())
            .read()) as i16),
            ((((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(286)
                .cast::<u16>())
            .read()) as i16),
        );
        RunTasks();
        AnimateSprites();
        BuildOamBuffer();
        RunTextPrinters();
        UpdatePaletteFade();
    }
}
pub(crate) unsafe extern "C" fn InitBlenderBgs() {
    unsafe {
        SetGpuReg(0u8, 0u16);
        ResetSpriteData();
        FreeAllSpritePalettes();
        ResetTasks();
        SetVBlankCallback(Some(VBlankCB_BerryBlender));
        ResetBgsAndClearDma3BusyFlags(0u32);
        InitBgsFromTemplates(
            1u8,
            ((&raw const sBgTemplates).cast::<u8>().cast_mut()).cast::<u8>(),
            ((crate::c::div_u32(12u32, 4u32)) as u8),
        );
        SetBgTilemapBuffer(
            1u8,
            (((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(440))
            .cast::<u8>())
            .cast::<u8>(),
        );
        SetBgTilemapBuffer(
            2u8,
            ((((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(440))
            .cast::<u8>())
            .wrapping_offset(2048))
            .cast::<u8>(),
        );
        LoadUserWindowBorderGfx(0u8, 1u16, 208u8);
        LoadMessageBoxGfx(0u8, 20u16, 240u8);
        InitBerryBlenderWindows();
        ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(68)
            .cast::<u16>())
        .write(0u16);
        ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(76)
            .cast::<i16>())
        .write(0i16);
        ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(74)
            .cast::<u16>())
        .write(0u16);
        ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(78)
            .cast::<u16>())
        .write(0u16);
        ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(284)
            .cast::<u16>())
        .write(0u16);
        ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(286)
            .cast::<u16>())
        .write(0u16);
    }
}
pub(crate) unsafe extern "C" fn GetArrowProximity(arrowPos: u16, playerId: u8) -> u8 {
    unsafe {
        let mut arrowPos = arrowPos;
        let mut playerId = playerId;
        let mut pos: u32 =
            (((crate::c::div_i32(((arrowPos) as i32), 256i32)).wrapping_add(24i32)) as u32);
        let mut arrowId: u8 = ((((((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>())
            .read())
        .wrapping_add(150))
        .cast::<u16>())
        .wrapping_offset(((playerId) as i32) as isize))
        .read()) as u8);
        let mut hitRangeStart: u32 =
            ((((((&raw const sArrowHitRangeStart).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((arrowId) as i32) as isize))
            .read()) as u32);
        if (pos >= hitRangeStart) && (pos < (hitRangeStart).wrapping_add(48u32)) {
            if (pos >= (hitRangeStart).wrapping_add(20u32))
                && (pos < (hitRangeStart).wrapping_add(28u32))
            {
                return 2u8;
            } else {
                return 1u8;
            }
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn SetOpponentsBerryData(
    playerBerryItemId: u16,
    playersNum: u8,
    playerBerry: *mut u8,
) {
    unsafe {
        let mut playerBerryItemId = playerBerryItemId;
        let mut playersNum = playersNum;
        let mut playerBerry = playerBerry;
        let mut opponentSetId: u16 = 0u16;
        let mut opponentBerryId: u16 = 0u16;
        let mut berryMasterDiff: u16 = 0u16;
        let mut i: u16 = 0u16;
        if ((playerBerryItemId) as i32) == 175i32 {
            {
                i = 0u16;
                'l1: loop {
                    if !(((i) as i32) < 5i32) {
                        break 'l1;
                    }
                    'l2: {
                        if ((((((playerBerry).wrapping_add(9)).cast::<u8>())
                            .wrapping_offset(((opponentSetId) as i32) as isize))
                        .read()) as i32)
                            > ((((((playerBerry).wrapping_add(9)).cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize))
                            .read()) as i32)
                        {
                            opponentSetId = i;
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
            opponentSetId = ((((opponentSetId) as i32).wrapping_add(5i32)) as u16);
        } else {
            opponentSetId = ((((((playerBerryItemId) as i32).wrapping_sub(133i32))
                .wrapping_add(1i32))
            .wrapping_sub(1i32)) as u16);
            if ((opponentSetId) as i32) >= 5i32 {
                opponentSetId = (((crate::c::rem_i32(((opponentSetId) as i32), 5i32))
                    .wrapping_add(5i32)) as u16);
            }
        }
        {
            i = 0u16;
            'l3: loop {
                if !(((i) as i32) < ((playersNum) as i32).wrapping_sub(1i32)) {
                    break 'l3;
                }
                'l4: {
                    opponentBerryId =
                        ((((((((&raw const sOpponentBerrySets).cast::<u8>().cast_mut())
                            .cast::<u8>())
                        .wrapping_offset(((opponentSetId) as i32) as isize * 3))
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize))
                        .read()) as u16);
                    berryMasterDiff = ((((((playerBerryItemId) as i32).wrapping_sub(133i32))
                        .wrapping_add(1i32))
                    .wrapping_sub(31i32)) as u16);
                    if (!((FlagGet(832u16)) != 0))
                        && (((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32) == 1i32)
                    {
                        opponentSetId = ((crate::c::rem_u32(
                            ((opponentSetId) as u32),
                            crate::c::div_u32(5u32, 1u32),
                        )) as u16);
                        opponentBerryId =
                            ((((((&raw const sBerryMasterBerries).cast::<u8>().cast_mut())
                                .cast::<u8>())
                            .wrapping_offset(((opponentSetId) as i32) as isize))
                            .read()) as u16);
                        if ((berryMasterDiff) as u32) < crate::c::div_u32(5u32, 1u32) {
                            opponentBerryId = ((((opponentBerryId) as u32)
                                .wrapping_sub(crate::c::div_u32(5u32, 1u32)))
                                as u16);
                        }
                    }
                    SetPlayerBerryData(
                        ((((i) as i32).wrapping_add(1i32)) as u8),
                        ((((opponentBerryId) as i32).wrapping_add(133i32)) as u16),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SetPlayerIdMaps() {
    unsafe {
        let mut i: i32 = 0i32;
        let mut j: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 4i32) {
                    break 'l1;
                }
                'l2: {
                    ((((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(150))
                    .cast::<u16>())
                    .wrapping_offset((i) as isize))
                    .write(255u16);
                    ((((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(142))
                    .cast::<u16>())
                    .wrapping_offset((i) as isize))
                    .write(
                        ((((((((&raw const sPlayerIdMap).cast::<u8>().cast_mut()).cast::<u8>())
                            .wrapping_offset(
                                (((((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(124))
                                .read()) as i32)
                                    .wrapping_sub(2i32)) as isize
                                    * 4,
                            ))
                        .cast::<u8>())
                        .wrapping_offset((i) as isize))
                        .read()) as u16),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        {
            j = 0i32;
            'l3: loop {
                if !(j < 4i32) {
                    break 'l3;
                }
                'l4: {
                    {
                        i = 0i32;
                        'l5: loop {
                            if !(i < 4i32) {
                                break 'l5;
                            }
                            'l6: {
                                if ((((((((&raw mut sBerryBlender)
                                    .cast::<u8>()
                                    .cast::<*mut u8>())
                                .read())
                                .wrapping_add(142))
                                .cast::<u16>())
                                .wrapping_offset((i) as isize))
                                .read()) as i32)
                                    == j
                                {
                                    ((((((&raw mut sBerryBlender)
                                        .cast::<u8>()
                                        .cast::<*mut u8>())
                                    .read())
                                    .wrapping_add(150))
                                    .cast::<u16>())
                                    .wrapping_offset((j) as isize))
                                    .write(((i) as u16));
                                }
                            }
                            i = (i).wrapping_add(1);
                        }
                    }
                }
                j = (j).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn PrintPlayerNames() {
    unsafe {
        let mut i: i32 = 0i32;
        let mut xPos: i32 = 0i32;
        let mut playerId: u32 = 0u32;
        let mut text = crate::ffi::Align4([0u8; 20]);
        if (((&raw mut gReceivedRemoteLinkPlayers).cast::<u8>()).read()) != 0 {
            playerId = ((GetMultiplayerId()) as u32);
        }
        {
            i = 0i32;
            'l1: loop {
                if !(i < 4i32) {
                    break 'l1;
                }
                'l2: {
                    if ((((((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(142))
                    .cast::<u16>())
                    .wrapping_offset((i) as isize))
                    .read()) as i32)
                        != 255i32
                    {
                        ((((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(80))
                        .cast::<u8>())
                        .wrapping_offset(
                            ((((((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(142))
                            .cast::<u16>())
                            .wrapping_offset((i) as isize))
                            .read()) as i32) as isize,
                        ))
                        .write(
                            ((((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(84))
                            .cast::<u8>())
                            .wrapping_offset((i) as isize))
                            .read(),
                        );
                        StartSpriteAnim(
                            ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                ((((((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(80))
                                .cast::<u8>())
                                .wrapping_offset(
                                    ((((((((&raw mut sBerryBlender)
                                        .cast::<u8>()
                                        .cast::<*mut u8>())
                                    .read())
                                    .wrapping_add(142))
                                    .cast::<u16>())
                                    .wrapping_offset((i) as isize))
                                    .read()) as i32) as isize,
                                ))
                                .read()) as i32) as isize
                                    * 68,
                            ),
                            ((i) as u8),
                        );
                        ((&raw mut text).cast::<u8>()).write(255u8);
                        StringCopy(
                            (&raw mut text).cast::<u8>(),
                            ((((&raw mut gLinkPlayers).cast::<u8>()).wrapping_offset(
                                ((((((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(142))
                                .cast::<u16>())
                                .wrapping_offset((i) as isize))
                                .read()) as i32) as isize
                                    * 28,
                            ))
                            .wrapping_add(8))
                            .cast::<u8>(),
                        );
                        xPos =
                            GetStringCenterAlignXOffset(1i32, (&raw mut text).cast::<u8>(), 56i32);
                        if playerId
                            == ((((((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(142))
                            .cast::<u16>())
                            .wrapping_offset((i) as isize))
                            .read()) as u32)
                        {
                            Blender_AddTextPrinter(
                                ((i) as u8),
                                (&raw mut text).cast::<u8>(),
                                ((xPos) as u8),
                                1u8,
                                0i32,
                                2i32,
                            );
                        } else {
                            Blender_AddTextPrinter(
                                ((i) as u8),
                                (&raw mut text).cast::<u8>(),
                                ((xPos) as u8),
                                1u8,
                                0i32,
                                1i32,
                            );
                        }
                        PutWindowTilemap(((i) as u8));
                        CopyWindowToVram(((i) as u8), 3u8);
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn CB2_StartBlenderLocal() {
    unsafe {
        let mut i: i32 = 0i32;
        let mut j: i32 = 0i32;
        'l1: {
            let __sw1 = (((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read()).read())
                as i32);
            if __sw1 == 0i32 {
                SetWirelessCommType0();
                InitBlenderBgs();
                SetPlayerBerryData(0u8, ((&raw mut gSpecialVar_ItemId).cast::<u16>()).read());
                ConvertItemToBlenderBerry(
                    ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(344))
                    .cast::<u8>(),
                    ((&raw mut gSpecialVar_ItemId).cast::<u16>()).read(),
                );
                SetOpponentsBerryData(
                    ((&raw mut gSpecialVar_ItemId).cast::<u16>()).read(),
                    ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(124))
                    .read(),
                    ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(344))
                    .cast::<u8>(),
                );
                {
                    i = 0i32;
                    'l2: loop {
                        if !(i < 4i32) {
                            break 'l2;
                        }
                        'l3: {
                            ((((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(100))
                            .cast::<u16>())
                            .wrapping_offset((i) as isize))
                            .write(0u16);
                            {
                                j = 0i32;
                                'l4: loop {
                                    if !(j < 3i32) {
                                        break 'l4;
                                    }
                                    'l5: {
                                        ((((((((&raw mut sBerryBlender)
                                            .cast::<u8>()
                                            .cast::<*mut u8>())
                                        .read())
                                        .wrapping_add(292))
                                        .cast::<u8>())
                                        .wrapping_offset((i) as isize * 6))
                                        .cast::<u16>())
                                        .wrapping_offset((j) as isize))
                                        .write(0u16);
                                    }
                                    j = (j).wrapping_add(1);
                                }
                            }
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(112)
                    .cast::<u16>())
                .write(0u16);
                ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(1))
                .write(0u8);
                ((&raw mut gLinkType).cast::<u16>()).write(17442u16);
                let __p2 = (((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read());
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                if (LoadBerryBlenderGfx()) != 0 {
                    let __p3 = (((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read());
                    (__p3).write(((__p3).read()).wrapping_add(1));
                    UpdateBlenderCenter();
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                {
                    i = 0i32;
                    'l6: loop {
                        if !(i < 4i32) {
                            break 'l6;
                        }
                        'l7: {
                            ((((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(84))
                            .cast::<u8>())
                            .wrapping_offset((i) as isize))
                            .write(CreateSprite(
                                (&raw const sSpriteTemplate_PlayerArrow)
                                    .cast::<u8>()
                                    .cast_mut(),
                                (((((((&raw const sPlayerArrowPos).cast::<u8>().cast_mut())
                                    .cast::<u8>())
                                .wrapping_offset((i) as isize * 2))
                                .cast::<u8>())
                                .read()) as i16),
                                ((((((((&raw const sPlayerArrowPos).cast::<u8>().cast_mut())
                                    .cast::<u8>())
                                .wrapping_offset((i) as isize * 2))
                                .cast::<u8>())
                                .wrapping_offset(1))
                                .read()) as i16),
                                1u8,
                            ));
                            StartSpriteAnim(
                                ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                    ((((((((&raw mut sBerryBlender)
                                        .cast::<u8>()
                                        .cast::<*mut u8>())
                                    .read())
                                    .wrapping_add(84))
                                    .cast::<u8>())
                                    .wrapping_offset((i) as isize))
                                    .read()) as i32) as isize
                                        * 68,
                                ),
                                (((i).wrapping_add(8i32)) as u8),
                            );
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                let __p4 = (((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read());
                (__p4).write(((__p4).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 3i32 {
                BeginNormalPaletteFade(4294967295u32, 0i8, 16u8, 0u8, 0u16);
                let __p5 = (((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read());
                (__p5).write(((__p5).read()).wrapping_add(1));
                ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(264)
                    .cast::<i32>())
                .write(0i32);
                break 'l1;
            }
            if __sw1 == 4i32 {
                if {
                    let __p6 = (((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(264)
                        .cast::<i32>();
                    let __t7 = ((__p6).read()).wrapping_add(1);
                    (__p6).write(__t7);
                    __t7
                } == 2i32
                {
                    DrawBlenderBg();
                }
                if !((crate::c::bf_read(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
                    7,
                    1,
                    false,
                ) as u16)
                    != 0)
                {
                    (((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read()).write(8u8);
                }
                break 'l1;
            }
            if __sw1 == 8i32 {
                (((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read()).write(11u8);
                ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(276))
                .write(0u8);
                break 'l1;
            }
            if __sw1 == 11i32 {
                {
                    i = 0i32;
                    'l8: loop {
                        if !(i < 4i32) {
                            break 'l8;
                        }
                        'l9: {
                            let mut playerId: u32 = ((((((((&raw const sPlayerIdMap)
                                .cast::<u8>()
                                .cast_mut())
                            .cast::<u8>())
                            .wrapping_offset(
                                (((((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(124))
                                .read()) as i32)
                                    .wrapping_sub(2i32)) as isize
                                    * 4,
                            ))
                            .cast::<u8>())
                            .wrapping_offset((i) as isize))
                            .read()) as u32);
                            if ((((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(276))
                            .read()) as u32)
                                == playerId
                            {
                                CreateBerrySprite(
                                    ((((((&raw mut sBerryBlender)
                                        .cast::<u8>()
                                        .cast::<*mut u8>())
                                    .read())
                                    .wrapping_add(116))
                                    .cast::<u16>())
                                    .wrapping_offset(
                                        ((((((&raw mut sBerryBlender)
                                            .cast::<u8>()
                                            .cast::<*mut u8>())
                                        .read())
                                        .wrapping_add(276))
                                        .read()) as i32)
                                            as isize,
                                    ))
                                    .read(),
                                    ((i) as u8),
                                );
                                break 'l8;
                            }
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(264)
                    .cast::<i32>())
                .write(0i32);
                let __p8 = (((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read());
                (__p8).write(((__p8).read()).wrapping_add(1));
                let __p9 = (((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(276);
                (__p9).write(((__p9).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 12i32 {
                if {
                    let __p10 = (((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(264)
                        .cast::<i32>();
                    let __t11 = ((__p10).read()).wrapping_add(1);
                    (__p10).write(__t11);
                    __t11
                } > 60i32
                {
                    if ((((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(276))
                    .read()) as i32)
                        >= ((((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(124))
                        .read()) as i32)
                    {
                        ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(74)
                            .cast::<u16>())
                        .write(
                            ((((((((&raw const sArrowStartPos)
                                .cast::<u8>()
                                .cast_mut()
                                .cast::<u16>())
                            .cast::<u16>())
                            .wrapping_offset(
                                ((((((&raw const sArrowStartPosIds).cast::<u8>().cast_mut())
                                    .cast::<u8>())
                                .wrapping_offset(
                                    (((((((&raw mut sBerryBlender)
                                        .cast::<u8>()
                                        .cast::<*mut u8>())
                                    .read())
                                    .wrapping_add(124))
                                    .read()) as i32)
                                        .wrapping_sub(2i32))
                                        as isize,
                                ))
                                .read()) as i32) as isize,
                            ))
                            .read()) as i32)
                                .wrapping_sub(22528i32)) as u16),
                        );
                        let __p12 =
                            (((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read());
                        (__p12).write(((__p12).read()).wrapping_add(1));
                    } else {
                        let __p13 =
                            (((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read());
                        (__p13).write(((__p13).read()).wrapping_sub(1));
                    }
                    ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(264)
                        .cast::<i32>())
                    .write(0i32);
                }
                break 'l1;
            }
            if __sw1 == 13i32 {
                let __p14 = (((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read());
                (__p14).write(((__p14).read()).wrapping_add(1));
                SetPlayerIdMaps();
                PlaySE(43u16);
                DrawBlenderCenter(
                    (((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(320),
                );
                ShowBg(2u8);
                break 'l1;
            }
            if __sw1 == 14i32 {
                SetGpuRegBits(0u8, 1024u16);
                let __p15 = (((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(74)
                    .cast::<u16>();
                (__p15).write((((((__p15).read()) as i32).wrapping_add(512i32)) as u16));
                let __p16 = (((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(282)
                    .cast::<u16>();
                (__p16).write((((((__p16).read()) as i32).wrapping_add(4i32)) as u16));
                if ((((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(282)
                    .cast::<u16>())
                .read()) as i32)
                    > 255i32
                {
                    let __p17 = (((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read());
                    (__p17).write(((__p17).read()).wrapping_add(1));
                    ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(282)
                        .cast::<u16>())
                    .write(256u16);
                    ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(74)
                        .cast::<u16>())
                    .write(
                        ((((&raw const sArrowStartPos)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<u16>())
                        .cast::<u16>())
                        .wrapping_offset(
                            ((((((&raw const sArrowStartPosIds).cast::<u8>().cast_mut())
                                .cast::<u8>())
                            .wrapping_offset(
                                (((((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(124))
                                .read()) as i32)
                                    .wrapping_sub(2i32)) as isize,
                            ))
                            .read()) as i32) as isize,
                        ))
                        .read(),
                    );
                    SetGpuRegBits(12u8, 2u16);
                    ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(264)
                        .cast::<i32>())
                    .write(0i32);
                    PlaySE(52u16);
                    PrintPlayerNames();
                }
                DrawBlenderCenter(
                    (((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(320),
                );
                break 'l1;
            }
            if __sw1 == 15i32 {
                if (UpdateBlenderLandScreenShake()) != 0 {
                    let __p18 = (((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read());
                    (__p18).write(((__p18).read()).wrapping_add(1));
                }
                DrawBlenderCenter(
                    (((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(320),
                );
                break 'l1;
            }
            if __sw1 == 16i32 {
                CreateSprite(
                    (&raw const sSpriteTemplate_CountdownNumbers)
                        .cast::<u8>()
                        .cast_mut(),
                    120i16,
                    (-16i16),
                    3u8,
                );
                let __p19 = (((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read());
                (__p19).write(((__p19).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 17i32 {
                break 'l1;
            }
            if __sw1 == 18i32 {
                let __p20 = (((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read());
                (__p20).write(((__p20).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 19i32 {
                let __p21 = (((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read());
                (__p21).write(((__p21).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 20i32 {
                let __p22 = (((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read());
                (__p22).write(((__p22).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 21i32 {
                ResetLinkCmds();
                ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(76)
                    .cast::<i16>())
                .write(128i16);
                ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(260)
                    .cast::<u32>())
                .write(0u32);
                ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(291))
                .write(0u8);
                ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(114))
                .write(0u8);
                SetMainCallback2(Some(CB2_PlayBlender));
                if ((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32) == 1i32 {
                    if !((FlagGet(832u16)) != 0) {
                        (((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(288))
                        .cast::<u8>())
                        .write(CreateTask(Some(Task_HandleBerryMaster), 10u8));
                    } else {
                        (((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(288))
                        .cast::<u8>())
                        .write(CreateTask(
                            (((&raw const sLocalOpponentTasks)
                                .cast::<u8>()
                                .cast_mut()
                                .cast::<Option<unsafe extern "C" fn(u8)>>())
                            .cast::<Option<unsafe extern "C" fn(u8)>>())
                            .read(),
                            10u8,
                        ));
                    }
                }
                if ((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32) > 1i32 {
                    {
                        i = 0i32;
                        'l10: loop {
                            if !(i
                                < ((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32))
                            {
                                break 'l10;
                            }
                            'l11: {
                                ((((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(288))
                                .cast::<u8>())
                                .wrapping_offset((i) as isize))
                                .write(CreateTask(
                                    ((((&raw const sLocalOpponentTasks)
                                        .cast::<u8>()
                                        .cast_mut()
                                        .cast::<Option<unsafe extern "C" fn(u8)>>())
                                    .cast::<Option<unsafe extern "C" fn(u8)>>())
                                    .wrapping_offset((i) as isize))
                                    .read(),
                                    (((10i32).wrapping_add(i)) as u8),
                                ));
                            }
                            i = (i).wrapping_add(1);
                        }
                    }
                }
                if ((GetCurrentMapMusic()) as i32) != 403i32 {
                    ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(340)
                        .cast::<u16>())
                    .write(GetCurrentMapMusic());
                }
                PlayBGM(403u16);
                PlaySE(53u16);
                UpdateHitPitch();
                break 'l1;
            }
        }
        Blender_DummiedOutFunc(
            ((((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(284)
                .cast::<u16>())
            .read()) as i16),
            ((((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(286)
                .cast::<u16>())
            .read()) as i16),
        );
        RunTasks();
        AnimateSprites();
        BuildOamBuffer();
        RunTextPrinters();
        UpdatePaletteFade();
    }
}
pub(crate) unsafe extern "C" fn ResetLinkCmds() {
    unsafe {
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 4i32) {
                    break 'l1;
                }
                'l2: {
                    (((&raw mut gSendCmd).cast::<u16>()).cast::<u16>()).write(0u16);
                    ((((&raw mut gSendCmd).cast::<u16>()).cast::<u16>()).wrapping_offset(2))
                        .write(0u16);
                    ((((&raw mut gRecvCmds).cast::<u8>()).wrapping_offset((i) as isize * 16))
                        .cast::<u16>())
                    .write(0u16);
                    (((((&raw mut gRecvCmds).cast::<u8>()).wrapping_offset((i) as isize * 16))
                        .cast::<u16>())
                    .wrapping_offset(2))
                    .write(0u16);
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_OpponentMiss(taskId: u8) {
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
            > ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(1))
            .read()) as i32)
        {
            (((((&raw mut gRecvCmds).cast::<u8>()).wrapping_offset(
                ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(2))
                .read()) as i32) as isize
                    * 16,
            ))
            .cast::<u16>())
            .wrapping_offset(2))
            .write(9029u16);
            DestroyTask(taskId);
        }
    }
}
pub(crate) unsafe extern "C" fn CreateOpponentMissTask(playerId: u8, delay: u8) {
    unsafe {
        let mut playerId = playerId;
        let mut delay = delay;
        let mut taskId: u8 = CreateTask(Some(Task_OpponentMiss), 80u8);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .write(((delay) as i16));
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(2))
        .write(((playerId) as i16));
    }
}
pub(crate) unsafe extern "C" fn Task_HandleOpponent1(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if ((GetArrowProximity(
            ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(74)
                .cast::<u16>())
            .read(),
            1u8,
        )) as i32)
            == 2i32
        {
            if !(((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .read())
                != 0)
            {
                if !((((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(291))
                .read())
                    != 0)
                {
                    let mut rand: u8 = ((crate::c::div_i32(((Random()) as i32), 655i32)) as u8);
                    if ((((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(76)
                        .cast::<i16>())
                    .read()) as i32)
                        < 500i32
                    {
                        if ((rand) as i32) > 75i32 {
                            (((((&raw mut gRecvCmds).cast::<u8>()).wrapping_offset(16))
                                .cast::<u16>())
                            .wrapping_offset(2))
                            .write(17699u16);
                        } else {
                            (((((&raw mut gRecvCmds).cast::<u8>()).wrapping_offset(16))
                                .cast::<u16>())
                            .wrapping_offset(2))
                            .write(21554u16);
                        }
                        (((((&raw mut gRecvCmds).cast::<u8>()).wrapping_offset(16)).cast::<u16>())
                            .wrapping_offset(2))
                        .write(21554u16);
                    } else {
                        if ((((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(76)
                            .cast::<i16>())
                        .read()) as i32)
                            < 1500i32
                        {
                            if ((rand) as i32) > 80i32 {
                                (((((&raw mut gRecvCmds).cast::<u8>()).wrapping_offset(16))
                                    .cast::<u16>())
                                .wrapping_offset(2))
                                .write(17699u16);
                            } else {
                                let mut value: u8 = ((((rand) as i32).wrapping_sub(21i32)) as u8);
                                if ((value) as i32) < 60i32 {
                                    (((((&raw mut gRecvCmds).cast::<u8>()).wrapping_offset(16))
                                        .cast::<u16>())
                                    .wrapping_offset(2))
                                    .write(21554u16);
                                } else {
                                    if ((rand) as i32) < 10i32 {
                                        CreateOpponentMissTask(1u8, 5u8);
                                    }
                                }
                            }
                        } else {
                            if ((rand) as i32) <= 90i32 {
                                let mut value: u8 = ((((rand) as i32).wrapping_sub(71i32)) as u8);
                                if ((value) as i32) < 20i32 {
                                    (((((&raw mut gRecvCmds).cast::<u8>()).wrapping_offset(16))
                                        .cast::<u16>())
                                    .wrapping_offset(2))
                                    .write(21554u16);
                                } else {
                                    if ((rand) as i32) < 30i32 {
                                        CreateOpponentMissTask(1u8, 5u8);
                                    }
                                }
                            } else {
                                (((((&raw mut gRecvCmds).cast::<u8>()).wrapping_offset(16))
                                    .cast::<u16>())
                                .wrapping_offset(2))
                                .write(17699u16);
                            }
                        }
                    }
                } else {
                    (((((&raw mut gRecvCmds).cast::<u8>()).wrapping_offset(16)).cast::<u16>())
                        .wrapping_offset(2))
                    .write(17699u16);
                }
                (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .write(1i16);
            }
        } else {
            (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
            .cast::<i16>())
            .write(0i16);
        }
    }
}
pub(crate) unsafe extern "C" fn Task_HandleOpponent2(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut var1: u32 = ((((((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>())
            .read())
        .wrapping_add(74)
        .cast::<u16>())
        .read()) as i32)
            .wrapping_add(6144i32)
            & 65535i32) as u32);
        let mut arrowId: u8 = ((((((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>())
            .read())
        .wrapping_add(150))
        .cast::<u16>())
        .wrapping_offset(2))
        .read()) as u8);
        if ((var1 >> 8)
            > ((((((((&raw const sArrowHitRangeStart).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((arrowId) as i32) as isize))
            .read()) as i32)
                .wrapping_add(20i32)) as u32))
            && ((var1 >> 8)
                < ((((((((&raw const sArrowHitRangeStart).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset(((arrowId) as i32) as isize))
                .read()) as i32)
                    .wrapping_add(40i32)) as u32))
        {
            if !(((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .read())
                != 0)
            {
                if !((((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(291))
                .read())
                    != 0)
                {
                    let mut rand: u8 = ((crate::c::div_i32(((Random()) as i32), 655i32)) as u8);
                    if ((((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(76)
                        .cast::<i16>())
                    .read()) as i32)
                        < 500i32
                    {
                        if ((rand) as i32) > 66i32 {
                            (((((&raw mut gRecvCmds).cast::<u8>()).wrapping_offset(32))
                                .cast::<u16>())
                            .wrapping_offset(2))
                            .write(17699u16);
                        } else {
                            (((((&raw mut gRecvCmds).cast::<u8>()).wrapping_offset(32))
                                .cast::<u16>())
                            .wrapping_offset(2))
                            .write(21554u16);
                        }
                    } else {
                        if ((rand) as i32) > 65i32 {
                            (((((&raw mut gRecvCmds).cast::<u8>()).wrapping_offset(32))
                                .cast::<u16>())
                            .wrapping_offset(2))
                            .write(17699u16);
                        }
                        if (((rand) as i32) > 40i32) && (((rand) as i32) <= 65i32) {
                            (((((&raw mut gRecvCmds).cast::<u8>()).wrapping_offset(32))
                                .cast::<u16>())
                            .wrapping_offset(2))
                            .write(21554u16);
                        }
                        if ((rand) as i32) < 10i32 {
                            CreateOpponentMissTask(2u8, 5u8);
                        }
                    }
                    (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .write(1i16);
                } else {
                    (((((&raw mut gRecvCmds).cast::<u8>()).wrapping_offset(32)).cast::<u16>())
                        .wrapping_offset(2))
                    .write(17699u16);
                    (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .write(1i16);
                }
            }
        } else {
            (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
            .cast::<i16>())
            .write(0i16);
        }
    }
}
pub(crate) unsafe extern "C" fn Task_HandleOpponent3(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut var1: u32 = ((((((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>())
            .read())
        .wrapping_add(74)
        .cast::<u16>())
        .read()) as i32)
            .wrapping_add(6144i32)
            & 65535i32) as u32);
        let mut arrowId: u8 = ((((((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>())
            .read())
        .wrapping_add(150))
        .cast::<u16>())
        .wrapping_offset(3))
        .read()) as u8);
        if ((var1 >> 8)
            > ((((((((&raw const sArrowHitRangeStart).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((arrowId) as i32) as isize))
            .read()) as i32)
                .wrapping_add(20i32)) as u32))
            && ((var1 >> 8)
                < ((((((((&raw const sArrowHitRangeStart).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset(((arrowId) as i32) as isize))
                .read()) as i32)
                    .wrapping_add(40i32)) as u32))
        {
            if (((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .read()) as i32)
                == 0i32
            {
                if !((((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(291))
                .read())
                    != 0)
                {
                    let mut rand: u8 = ((crate::c::div_i32(((Random()) as i32), 655i32)) as u8);
                    if ((((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(76)
                        .cast::<i16>())
                    .read()) as i32)
                        < 500i32
                    {
                        if ((rand) as i32) > 88i32 {
                            (((((&raw mut gRecvCmds).cast::<u8>()).wrapping_offset(48))
                                .cast::<u16>())
                            .wrapping_offset(2))
                            .write(17699u16);
                        } else {
                            (((((&raw mut gRecvCmds).cast::<u8>()).wrapping_offset(48))
                                .cast::<u16>())
                            .wrapping_offset(2))
                            .write(21554u16);
                        }
                    } else {
                        if ((rand) as i32) > 60i32 {
                            (((((&raw mut gRecvCmds).cast::<u8>()).wrapping_offset(48))
                                .cast::<u16>())
                            .wrapping_offset(2))
                            .write(17699u16);
                        } else {
                            if (((rand) as i32) > 55i32) && (((rand) as i32) <= 60i32) {
                                (((((&raw mut gRecvCmds).cast::<u8>()).wrapping_offset(48))
                                    .cast::<u16>())
                                .wrapping_offset(2))
                                .write(21554u16);
                            }
                        }
                        if ((rand) as i32) < 5i32 {
                            CreateOpponentMissTask(3u8, 5u8);
                        }
                    }
                    (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .write(1i16);
                } else {
                    (((((&raw mut gRecvCmds).cast::<u8>()).wrapping_offset(48)).cast::<u16>())
                        .wrapping_offset(2))
                    .write(17699u16);
                    (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .write(1i16);
                }
            }
        } else {
            (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
            .cast::<i16>())
            .write(0i16);
        }
    }
}
pub(crate) unsafe extern "C" fn Task_HandleBerryMaster(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if ((GetArrowProximity(
            ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(74)
                .cast::<u16>())
            .read(),
            1u8,
        )) as i32)
            == 2i32
        {
            if !(((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .read())
                != 0)
            {
                (((((&raw mut gRecvCmds).cast::<u8>()).wrapping_offset(16)).cast::<u16>())
                    .wrapping_offset(2))
                .write(17699u16);
                (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .write(1i16);
            }
        } else {
            (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
            .cast::<i16>())
            .write(0i16);
        }
    }
}
pub(crate) unsafe extern "C" fn CreateScoreSymbolSprite(cmd: u16, arrowId: u8) {
    unsafe {
        let mut cmd = cmd;
        let mut arrowId = arrowId;
        let mut spriteId: u8 = 0u8;
        spriteId = CreateSprite(
            (&raw const sSpriteTemplate_ScoreSymbols)
                .cast::<u8>()
                .cast_mut(),
            (((((((((&raw const sPlayerArrowPos).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((arrowId) as i32) as isize * 2))
            .cast::<u8>())
            .read()) as i32)
                .wrapping_sub(
                    (10i32).wrapping_mul(
                        (((((((&raw const sPlayerArrowQuadrant).cast::<u8>().cast_mut())
                            .cast::<u8>())
                        .wrapping_offset(((arrowId) as i32) as isize * 2))
                        .cast::<i8>())
                        .read()) as i32),
                    ),
                )) as i16),
            ((((((((((&raw const sPlayerArrowPos).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((arrowId) as i32) as isize * 2))
            .cast::<u8>())
            .wrapping_offset(1))
            .read()) as i32)
                .wrapping_sub(
                    (10i32).wrapping_mul(
                        ((((((((&raw const sPlayerArrowQuadrant).cast::<u8>().cast_mut())
                            .cast::<u8>())
                        .wrapping_offset(((arrowId) as i32) as isize * 2))
                        .cast::<i8>())
                        .wrapping_offset(1))
                        .read()) as i32),
                    ),
                )) as i16),
            1u8,
        );
        if ((cmd) as i32) == 17699i32 {
            StartSpriteAnim(
                ((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68),
                2u8,
            );
            ((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(SpriteCB_ScoreSymbolBest));
            PlaySE(40u16);
        } else {
            if ((cmd) as i32) == 21554i32 {
                StartSpriteAnim(
                    ((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68),
                    0u8,
                );
                PlaySE(31u16);
            } else {
                if ((cmd) as i32) == 9029i32 {
                    StartSpriteAnim(
                        ((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((spriteId) as i32) as isize * 68),
                        1u8,
                    );
                    PlaySE(32u16);
                }
            }
        }
        CreateParticleSprites();
    }
}
pub(crate) unsafe extern "C" fn UpdateSpeedFromHit(cmd: u16) {
    unsafe {
        let mut cmd = cmd;
        UpdateHitPitch();
        'l1: {
            let __sw1 = ((cmd) as i32);
            if __sw1 == 17699i32 {
                if ((((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(76)
                    .cast::<i16>())
                .read()) as i32)
                    < 1500i32
                {
                    let __p2 = (((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(76)
                        .cast::<i16>();
                    (__p2).write(
                        (((((__p2).read()) as i32).wrapping_add(crate::c::div_i32(
                            384i32,
                            ((((((&raw const sNumPlayersToSpeedDivisor)
                                .cast::<u8>()
                                .cast_mut())
                            .cast::<u8>())
                            .wrapping_offset(
                                ((((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(124))
                                .read()) as i32) as isize,
                            ))
                            .read()) as i32),
                        ))) as i16),
                    );
                } else {
                    let __p3 = (((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(76)
                        .cast::<i16>();
                    (__p3).write(
                        (((((__p3).read()) as i32).wrapping_add(crate::c::div_i32(
                            128i32,
                            ((((((&raw const sNumPlayersToSpeedDivisor)
                                .cast::<u8>()
                                .cast_mut())
                            .cast::<u8>())
                            .wrapping_offset(
                                ((((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(124))
                                .read()) as i32) as isize,
                            ))
                            .read()) as i32),
                        ))) as i16),
                    );
                    ShakeBgCoordForHit(
                        ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(284)
                            .cast::<u16>())
                        .cast::<i16>(),
                        (((crate::c::div_i32(
                            ((((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(76)
                                .cast::<i16>())
                            .read()) as i32),
                            100i32,
                        ))
                        .wrapping_sub(10i32)) as u16),
                    );
                    ShakeBgCoordForHit(
                        ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(286)
                            .cast::<u16>())
                        .cast::<i16>(),
                        (((crate::c::div_i32(
                            ((((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(76)
                                .cast::<i16>())
                            .read()) as i32),
                            100i32,
                        ))
                        .wrapping_sub(10i32)) as u16),
                    );
                }
                break 'l1;
            }
            if __sw1 == 21554i32 {
                if ((((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(76)
                    .cast::<i16>())
                .read()) as i32)
                    < 1500i32
                {
                    let __p4 = (((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(76)
                        .cast::<i16>();
                    (__p4).write(
                        (((((__p4).read()) as i32).wrapping_add(crate::c::div_i32(
                            256i32,
                            ((((((&raw const sNumPlayersToSpeedDivisor)
                                .cast::<u8>()
                                .cast_mut())
                            .cast::<u8>())
                            .wrapping_offset(
                                ((((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(124))
                                .read()) as i32) as isize,
                            ))
                            .read()) as i32),
                        ))) as i16),
                    );
                }
                break 'l1;
            }
            if __sw1 == 9029i32 {
                let __p5 = (((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(76)
                    .cast::<i16>();
                (__p5).write(
                    (((((__p5).read()) as i32).wrapping_sub(crate::c::div_i32(
                        256i32,
                        ((((((&raw const sNumPlayersToSpeedDivisor)
                            .cast::<u8>()
                            .cast_mut())
                        .cast::<u8>())
                        .wrapping_offset(
                            ((((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(124))
                            .read()) as i32) as isize,
                        ))
                        .read()) as i32),
                    ))) as i16),
                );
                if ((((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(76)
                    .cast::<i16>())
                .read()) as i32)
                    < 128i32
                {
                    ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(76)
                        .cast::<i16>())
                    .write(128i16);
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn CheckRecvCmdMatches(
    recvCmd: u16,
    linkCmd: u16,
    rfuCmd: u16,
) -> u32 {
    unsafe {
        let mut recvCmd = recvCmd;
        let mut linkCmd = linkCmd;
        let mut rfuCmd = rfuCmd;
        if ((((&raw mut gReceivedRemoteLinkPlayers).cast::<u8>()).read()) != 0)
            && ((((&raw mut gWirelessCommType).cast::<u8>()).read()) != 0)
        {
            if (((recvCmd) as i32) & 65280i32) == ((rfuCmd) as i32) {
                return 1u32;
            }
        } else {
            if ((recvCmd) as i32) == ((linkCmd) as i32) {
                return 1u32;
            }
        }
        return 0u32;
    }
}
pub(crate) unsafe extern "C" fn UpdateOpponentScores() {
    unsafe {
        let mut i: i32 = 0i32;
        if ((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32) != 0i32 {
            if ((((((&raw mut gSendCmd).cast::<u16>()).cast::<u16>()).wrapping_offset(2)).read())
                as i32)
                != 0i32
            {
                ((((&raw mut gRecvCmds).cast::<u8>()).cast::<u16>()).wrapping_offset(2)).write(
                    ((((&raw mut gSendCmd).cast::<u16>()).cast::<u16>()).wrapping_offset(2)).read(),
                );
                (((&raw mut gRecvCmds).cast::<u8>()).cast::<u16>()).write(17476u16);
                ((((&raw mut gSendCmd).cast::<u16>()).cast::<u16>()).wrapping_offset(2))
                    .write(0u16);
            }
            {
                i = 1i32;
                'l1: loop {
                    if !(i < 4i32) {
                        break 'l1;
                    }
                    'l2: {
                        if (((((((&raw mut gRecvCmds).cast::<u8>())
                            .wrapping_offset((i) as isize * 16))
                        .cast::<u16>())
                        .wrapping_offset(2))
                        .read()) as i32)
                            != 0i32
                        {
                            ((((&raw mut gRecvCmds).cast::<u8>())
                                .wrapping_offset((i) as isize * 16))
                            .cast::<u16>())
                            .write(17476u16);
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
        }
        {
            i = 0i32;
            'l3: loop {
                if !(i
                    < ((((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(124))
                    .read()) as i32))
                {
                    break 'l3;
                }
                'l4: {
                    if (CheckRecvCmdMatches(
                        ((((&raw mut gRecvCmds).cast::<u8>()).wrapping_offset((i) as isize * 16))
                            .cast::<u16>())
                        .read(),
                        17476u16,
                        17408u16,
                    )) != 0
                    {
                        let mut arrowId: u32 =
                            ((((((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(150))
                            .cast::<u16>())
                            .wrapping_offset((i) as isize))
                            .read()) as u32);
                        if (((((((&raw mut gRecvCmds).cast::<u8>())
                            .wrapping_offset((i) as isize * 16))
                        .cast::<u16>())
                        .wrapping_offset(2))
                        .read()) as i32)
                            == 17699i32
                        {
                            UpdateSpeedFromHit(17699u16);
                            let __p1 = (((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(278)
                            .cast::<u16>();
                            (__p1).write(
                                (((((__p1).read()) as i32).wrapping_add(crate::c::div_i32(
                                    ((((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(76)
                                    .cast::<i16>())
                                    .read()) as i32),
                                    55i32,
                                ))) as u16),
                            );
                            if ((((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(278)
                            .cast::<u16>())
                            .read()) as i32)
                                >= 1000i32
                            {
                                ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(278)
                                .cast::<u16>())
                                .write(1000u16);
                            }
                            CreateScoreSymbolSprite(17699u16, ((arrowId) as u8));
                            let __p2 =
                                ((((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(292))
                                .cast::<u8>())
                                .wrapping_offset((i) as isize * 6))
                                .cast::<u16>();
                            (__p2).write(((__p2).read()).wrapping_add(1));
                        } else {
                            if (((((((&raw mut gRecvCmds).cast::<u8>())
                                .wrapping_offset((i) as isize * 16))
                            .cast::<u16>())
                            .wrapping_offset(2))
                            .read()) as i32)
                                == 21554i32
                            {
                                UpdateSpeedFromHit(21554u16);
                                let __p3 =
                                    (((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(278)
                                    .cast::<u16>();
                                (__p3).write(
                                    (((((__p3).read()) as i32).wrapping_add(crate::c::div_i32(
                                        ((((((&raw mut sBerryBlender)
                                            .cast::<u8>()
                                            .cast::<*mut u8>())
                                        .read())
                                        .wrapping_add(76)
                                        .cast::<i16>())
                                        .read()) as i32),
                                        70i32,
                                    ))) as u16),
                                );
                                CreateScoreSymbolSprite(21554u16, ((arrowId) as u8));
                                let __p4 = (((((((&raw mut sBerryBlender)
                                    .cast::<u8>()
                                    .cast::<*mut u8>())
                                .read())
                                .wrapping_add(292))
                                .cast::<u8>())
                                .wrapping_offset((i) as isize * 6))
                                .cast::<u16>())
                                .wrapping_offset(1);
                                (__p4).write(((__p4).read()).wrapping_add(1));
                            } else {
                                if (((((((&raw mut gRecvCmds).cast::<u8>())
                                    .wrapping_offset((i) as isize * 16))
                                .cast::<u16>())
                                .wrapping_offset(2))
                                .read()) as i32)
                                    == 9029i32
                                {
                                    CreateScoreSymbolSprite(9029u16, ((arrowId) as u8));
                                    UpdateSpeedFromHit(9029u16);
                                    if ((((((((((&raw mut sBerryBlender)
                                        .cast::<u8>()
                                        .cast::<*mut u8>())
                                    .read())
                                    .wrapping_add(292))
                                    .cast::<u8>())
                                    .wrapping_offset((i) as isize * 6))
                                    .cast::<u16>())
                                    .wrapping_offset(2))
                                    .read()) as i32)
                                        < 999i32
                                    {
                                        let __p5 = (((((((&raw mut sBerryBlender)
                                            .cast::<u8>()
                                            .cast::<*mut u8>())
                                        .read())
                                        .wrapping_add(292))
                                        .cast::<u8>())
                                        .wrapping_offset((i) as isize * 6))
                                        .cast::<u16>())
                                        .wrapping_offset(2);
                                        (__p5).write(((__p5).read()).wrapping_add(1));
                                    }
                                }
                            }
                        }
                        if (((((((((&raw mut gRecvCmds).cast::<u8>())
                            .wrapping_offset((i) as isize * 16))
                        .cast::<u16>())
                        .wrapping_offset(2))
                        .read()) as i32)
                            == 9029i32)
                            || ((((((((&raw mut gRecvCmds).cast::<u8>()).wrapping_offset(32))
                                .cast::<u16>())
                            .wrapping_offset((i) as isize))
                            .read()) as i32)
                                == 17699i32))
                            || ((((((((&raw mut gRecvCmds).cast::<u8>()).wrapping_offset(32))
                                .cast::<u16>())
                            .wrapping_offset((i) as isize))
                            .read()) as i32)
                                == 21554i32)
                        {
                            if ((((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(76)
                            .cast::<i16>())
                            .read()) as i32)
                                > 1500i32
                            {
                                m4aMPlayTempoControl(
                                    (&raw mut gMPlayInfo_BGM).cast::<u8>(),
                                    (((crate::c::div_i32(
                                        ((((((&raw mut sBerryBlender)
                                            .cast::<u8>()
                                            .cast::<*mut u8>())
                                        .read())
                                        .wrapping_add(76)
                                        .cast::<i16>())
                                        .read()) as i32)
                                            .wrapping_sub(750i32),
                                        20i32,
                                    ))
                                    .wrapping_add(256i32))
                                        as u16),
                                );
                            } else {
                                m4aMPlayTempoControl(
                                    (&raw mut gMPlayInfo_BGM).cast::<u8>(),
                                    256u16,
                                );
                            }
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        if ((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32) != 0i32 {
            {
                i = 0i32;
                'l5: loop {
                    if !(i
                        < ((((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(124))
                        .read()) as i32))
                    {
                        break 'l5;
                    }
                    'l6: {
                        ((((&raw mut gRecvCmds).cast::<u8>()).wrapping_offset((i) as isize * 16))
                            .cast::<u16>())
                        .write(0u16);
                        (((((&raw mut gRecvCmds).cast::<u8>())
                            .wrapping_offset((i) as isize * 16))
                        .cast::<u16>())
                        .wrapping_offset(2))
                        .write(0u16);
                    }
                    i = (i).wrapping_add(1);
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn HandlePlayerInput() {
    unsafe {
        let mut arrowId: u8 = 0u8;
        let mut pressedA: u8 = 0u8;
        let mut playerId: u8 = 0u8;
        if (((&raw mut gReceivedRemoteLinkPlayers).cast::<u8>()).read()) != 0 {
            playerId = GetMultiplayerId();
        }
        arrowId = ((((((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(150))
        .cast::<u16>())
        .wrapping_offset(((playerId) as i32) as isize))
        .read()) as u8);
        if ((((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(99))
            .read()) as i32)
            == 0i32
        {
            if (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(19)).read())
                as i32)
                == 2i32)
                && (((((((&raw mut gMain).cast::<u8>())
                    .wrapping_add(46)
                    .cast::<u16>())
                .read()) as i32)
                    & 1i32)
                    != 0)
            {
                if ((((((&raw mut gMain).cast::<u8>())
                    .wrapping_add(40)
                    .cast::<u16>())
                .read()) as i32)
                    & 513i32)
                    != 513i32
                {
                    pressedA = 1u8;
                }
            } else {
                if ((((((&raw mut gMain).cast::<u8>())
                    .wrapping_add(46)
                    .cast::<u16>())
                .read()) as i32)
                    & 1i32)
                    != 0
                {
                    pressedA = 1u8;
                }
            }
            if (pressedA) != 0 {
                let mut proximity: u8 = 0u8;
                StartSpriteAnim(
                    ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(80))
                        .cast::<u8>())
                        .wrapping_offset(
                            ((((((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(142))
                            .cast::<u16>())
                            .wrapping_offset(((arrowId) as i32) as isize))
                            .read()) as i32) as isize,
                        ))
                        .read()) as i32) as isize
                            * 68,
                    ),
                    ((((arrowId) as i32).wrapping_add(4i32)) as u8),
                );
                proximity = GetArrowProximity(
                    ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(74)
                        .cast::<u16>())
                    .read(),
                    playerId,
                );
                if ((proximity) as i32) == 2i32 {
                    ((((&raw mut gSendCmd).cast::<u16>()).cast::<u16>()).wrapping_offset(2))
                        .write(17699u16);
                } else {
                    if ((proximity) as i32) == 1i32 {
                        ((((&raw mut gSendCmd).cast::<u16>()).cast::<u16>()).wrapping_offset(2))
                            .write(21554u16);
                    } else {
                        ((((&raw mut gSendCmd).cast::<u16>()).cast::<u16>()).wrapping_offset(2))
                            .write(9029u16);
                    }
                }
            }
        }
        if (({
            let __p1 = (((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(114);
            let __t2 = ((__p1).read()).wrapping_add(1);
            (__p1).write(__t2);
            __t2
        }) as i32)
            > 5i32
        {
            if ((((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(76)
                .cast::<i16>())
            .read()) as i32)
                > 128i32
            {
                let __p3 = (((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(76)
                    .cast::<i16>();
                (__p3).write(((__p3).read()).wrapping_sub(1));
            }
            ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(114))
                .write(0u8);
        }
        if ((((&raw mut gEnableContestDebugging).cast::<u8>()).read()) != 0)
            && (((((((&raw mut gMain).cast::<u8>())
                .wrapping_add(46)
                .cast::<u16>())
            .read()) as i32)
                & 512i32)
                != 0)
        {
            let __p4 = (((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(291);
            (__p4).write((((((__p4).read()) as i32) ^ 1i32) as u8));
        }
    }
}
pub(crate) unsafe extern "C" fn CB2_PlayBlender() {
    unsafe {
        UpdateBlenderCenter();
        if ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(260)
            .cast::<u32>())
        .read()
            < 359940u32
        {
            let __p1 = (((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(260)
                .cast::<u32>();
            (__p1).write(((__p1).read()).wrapping_add(1));
        }
        HandlePlayerInput();
        SetLinkDebugValues(
            (((((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(76)
                .cast::<i16>())
            .read()) as u16) as u32),
            ((((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(278)
                .cast::<u16>())
            .read()) as u32),
        );
        UpdateOpponentScores();
        TryUpdateProgressBar(
            ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(278)
                .cast::<u16>())
            .read(),
            1000u16,
        );
        UpdateRPM(
            ((((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(76)
                .cast::<i16>())
            .read()) as u16),
        );
        RestoreBgCoords();
        ProcessLinkPlayerCmds();
        if (((((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(99))
            .read()) as i32)
            == 0i32)
            && (((((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(280)
                .cast::<u16>())
            .read()) as i32)
                >= 1000i32)
        {
            ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(278)
                .cast::<u16>())
            .write(1000u16);
            ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(99))
                .write(1u8);
            SetMainCallback2(Some(CB2_EndBlenderGame));
        }
        Blender_DummiedOutFunc(
            ((((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(284)
                .cast::<u16>())
            .read()) as i16),
            ((((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(286)
                .cast::<u16>())
            .read()) as i16),
        );
        RunTasks();
        AnimateSprites();
        BuildOamBuffer();
        RunTextPrinters();
        UpdatePaletteFade();
    }
}
pub(crate) unsafe extern "C" fn Blender_DummiedOutFunc(bgX: i16, bgY: i16) {
    unsafe {
        let mut bgX = bgX;
        let mut bgY = bgY;
    }
}
pub(crate) unsafe extern "C" fn AreBlenderBerriesSame(berries: *mut u8, a: u8, b: u8) -> u8 {
    unsafe {
        let mut berries = berries;
        let mut a = a;
        let mut b = b;
        if ((((((berries).wrapping_offset(((a) as i32) as isize * 16)).cast::<u16>()).read())
            as i32)
            != (((((berries).wrapping_offset(((b) as i32) as isize * 16)).cast::<u16>()).read())
                as i32))
            || ((StringCompare(
                (((berries).wrapping_offset(((a) as i32) as isize * 16)).wrapping_add(2))
                    .cast::<u8>(),
                (((berries).wrapping_offset(((b) as i32) as isize * 16)).wrapping_add(2))
                    .cast::<u8>(),
            ) == 0i32)
                && ((((((((((((berries).wrapping_offset(((a) as i32) as isize * 16))
                    .wrapping_add(9))
                .cast::<u8>())
                .read()) as i32)
                    == ((((((berries).wrapping_offset(((b) as i32) as isize * 16))
                        .wrapping_add(9))
                    .cast::<u8>())
                    .read()) as i32))
                    && ((((((((berries).wrapping_offset(((a) as i32) as isize * 16))
                        .wrapping_add(9))
                    .cast::<u8>())
                    .wrapping_offset(1))
                    .read()) as i32)
                        == (((((((berries).wrapping_offset(((b) as i32) as isize * 16))
                            .wrapping_add(9))
                        .cast::<u8>())
                        .wrapping_offset(1))
                        .read()) as i32)))
                    && ((((((((berries).wrapping_offset(((a) as i32) as isize * 16))
                        .wrapping_add(9))
                    .cast::<u8>())
                    .wrapping_offset(2))
                    .read()) as i32)
                        == (((((((berries).wrapping_offset(((b) as i32) as isize * 16))
                            .wrapping_add(9))
                        .cast::<u8>())
                        .wrapping_offset(2))
                        .read()) as i32)))
                    && ((((((((berries).wrapping_offset(((a) as i32) as isize * 16))
                        .wrapping_add(9))
                    .cast::<u8>())
                    .wrapping_offset(3))
                    .read()) as i32)
                        == (((((((berries).wrapping_offset(((b) as i32) as isize * 16))
                            .wrapping_add(9))
                        .cast::<u8>())
                        .wrapping_offset(3))
                        .read()) as i32)))
                    && ((((((((berries).wrapping_offset(((a) as i32) as isize * 16))
                        .wrapping_add(9))
                    .cast::<u8>())
                    .wrapping_offset(4))
                    .read()) as i32)
                        == (((((((berries).wrapping_offset(((b) as i32) as isize * 16))
                            .wrapping_add(9))
                        .cast::<u8>())
                        .wrapping_offset(4))
                        .read()) as i32)))
                    && ((((((((berries).wrapping_offset(((a) as i32) as isize * 16))
                        .wrapping_add(9))
                    .cast::<u8>())
                    .wrapping_offset(5))
                    .read()) as i32)
                        == (((((((berries).wrapping_offset(((b) as i32) as isize * 16))
                            .wrapping_add(9))
                        .cast::<u8>())
                        .wrapping_offset(5))
                        .read()) as i32))))
        {
            return 1u8;
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
pub(crate) unsafe extern "C" fn CalculatePokeblockColor(
    berries: *mut u8,
    _flavors: *mut i16,
    numPlayers: u8,
    negativeFlavors: u8,
) -> u32 {
    unsafe {
        let mut berries = berries;
        let mut _flavors = _flavors;
        let mut numPlayers = numPlayers;
        let mut negativeFlavors = negativeFlavors;
        let mut flavors = crate::ffi::Align4([0u8; 12]);
        let mut i: i32 = 0i32;
        let mut j: i32 = 0i32;
        let mut numFlavors: u8 = 0u8;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 6i32) {
                    break 'l1;
                }
                'l2: {
                    (((&raw mut flavors).cast::<i16>()).wrapping_offset((i) as isize))
                        .write(((_flavors).wrapping_offset((i) as isize)).read());
                }
                i = (i).wrapping_add(1);
            }
        }
        j = 0i32;
        {
            i = 0i32;
            'l3: loop {
                if !(i < 5i32) {
                    break 'l3;
                }
                'l4: {
                    if (((((&raw mut flavors).cast::<i16>()).wrapping_offset((i) as isize)).read())
                        as i32)
                        == 0i32
                    {
                        j = (j).wrapping_add(1);
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        if (j == 5i32) || (((negativeFlavors) as i32) > 3i32) {
            return 12u32;
        }
        {
            i = 0i32;
            'l5: loop {
                if !(i < ((numPlayers) as i32)) {
                    break 'l5;
                }
                'l6: {
                    {
                        j = 0i32;
                        'l7: loop {
                            if !(j < ((numPlayers) as i32)) {
                                break 'l7;
                            }
                            'l8: {
                                if (((((((berries).wrapping_offset((i) as isize * 16))
                                    .cast::<u16>())
                                .read()) as i32)
                                    == (((((berries).wrapping_offset((j) as isize * 16))
                                        .cast::<u16>())
                                    .read()) as i32))
                                    && (i != j))
                                    && (((((((berries).wrapping_offset((i) as isize * 16))
                                        .cast::<u16>())
                                    .read()) as i32)
                                        != 175i32)
                                        || ((AreBlenderBerriesSame(
                                            berries,
                                            ((i) as u8),
                                            ((j) as u8),
                                        )) != 0))
                                {
                                    return 12u32;
                                }
                            }
                            j = (j).wrapping_add(1);
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        numFlavors = 0u8;
        {
            numFlavors = 0u8;
            i = 0i32;
            'l9: loop {
                if !(i < 5i32) {
                    break 'l9;
                }
                'l10: {
                    if (((((&raw mut flavors).cast::<i16>()).wrapping_offset((i) as isize)).read())
                        as i32)
                        > 0i32
                    {
                        numFlavors = (numFlavors).wrapping_add(1);
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        if ((numFlavors) as i32) > 3i32 {
            return 13u32;
        }
        if ((numFlavors) as i32) == 3i32 {
            return 11u32;
        }
        {
            i = 0i32;
            'l11: loop {
                if !(i < 5i32) {
                    break 'l11;
                }
                'l12: {
                    if (((((&raw mut flavors).cast::<i16>()).wrapping_offset((i) as isize)).read())
                        as i32)
                        > 50i32
                    {
                        return 14u32;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        if (((numFlavors) as i32) == 1i32)
            && (((((&raw mut flavors).cast::<i16>()).read()) as i32) > 0i32)
        {
            return 1u32;
        }
        if (((numFlavors) as i32) == 1i32)
            && ((((((&raw mut flavors).cast::<i16>()).wrapping_offset(1)).read()) as i32) > 0i32)
        {
            return 2u32;
        }
        if (((numFlavors) as i32) == 1i32)
            && ((((((&raw mut flavors).cast::<i16>()).wrapping_offset(2)).read()) as i32) > 0i32)
        {
            return 3u32;
        }
        if (((numFlavors) as i32) == 1i32)
            && ((((((&raw mut flavors).cast::<i16>()).wrapping_offset(3)).read()) as i32) > 0i32)
        {
            return 4u32;
        }
        if (((numFlavors) as i32) == 1i32)
            && ((((((&raw mut flavors).cast::<i16>()).wrapping_offset(4)).read()) as i32) > 0i32)
        {
            return 5u32;
        }
        if ((numFlavors) as i32) == 2i32 {
            let mut idx: i32 = 0i32;
            {
                i = 0i32;
                'l13: loop {
                    if !(i < 5i32) {
                        break 'l13;
                    }
                    'l14: {
                        if (((((&raw mut flavors).cast::<i16>()).wrapping_offset((i) as isize))
                            .read()) as i32)
                            > 0i32
                        {
                            ((((&raw mut sPokeblockPresentFlavors)
                                .cast::<u8>()
                                .cast::<i16>())
                            .cast::<i16>())
                            .wrapping_offset(
                                ({
                                    let __t1 = idx;
                                    idx = (idx).wrapping_add(1);
                                    __t1
                                }) as isize,
                            ))
                            .write(((i) as i16));
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
            if (((((&raw mut flavors).cast::<i16>()).wrapping_offset(
                (((((&raw mut sPokeblockPresentFlavors)
                    .cast::<u8>()
                    .cast::<i16>())
                .cast::<i16>())
                .read()) as i32) as isize,
            ))
            .read()) as i32)
                >= (((((&raw mut flavors).cast::<i16>()).wrapping_offset(
                    ((((((&raw mut sPokeblockPresentFlavors)
                        .cast::<u8>()
                        .cast::<i16>())
                    .cast::<i16>())
                    .wrapping_offset(1))
                    .read()) as i32) as isize,
                ))
                .read()) as i32)
            {
                if (((((&raw mut sPokeblockPresentFlavors)
                    .cast::<u8>()
                    .cast::<i16>())
                .cast::<i16>())
                .read()) as i32)
                    == 0i32
                {
                    return (((((((((&raw mut sPokeblockPresentFlavors)
                        .cast::<u8>()
                        .cast::<i16>())
                    .cast::<i16>())
                    .wrapping_offset(1))
                    .read()) as i32)
                        << 16)
                        | 6i32) as u32);
                }
                if (((((&raw mut sPokeblockPresentFlavors)
                    .cast::<u8>()
                    .cast::<i16>())
                .cast::<i16>())
                .read()) as i32)
                    == 1i32
                {
                    return (((((((((&raw mut sPokeblockPresentFlavors)
                        .cast::<u8>()
                        .cast::<i16>())
                    .cast::<i16>())
                    .wrapping_offset(1))
                    .read()) as i32)
                        << 16)
                        | 7i32) as u32);
                }
                if (((((&raw mut sPokeblockPresentFlavors)
                    .cast::<u8>()
                    .cast::<i16>())
                .cast::<i16>())
                .read()) as i32)
                    == 2i32
                {
                    return (((((((((&raw mut sPokeblockPresentFlavors)
                        .cast::<u8>()
                        .cast::<i16>())
                    .cast::<i16>())
                    .wrapping_offset(1))
                    .read()) as i32)
                        << 16)
                        | 8i32) as u32);
                }
                if (((((&raw mut sPokeblockPresentFlavors)
                    .cast::<u8>()
                    .cast::<i16>())
                .cast::<i16>())
                .read()) as i32)
                    == 3i32
                {
                    return (((((((((&raw mut sPokeblockPresentFlavors)
                        .cast::<u8>()
                        .cast::<i16>())
                    .cast::<i16>())
                    .wrapping_offset(1))
                    .read()) as i32)
                        << 16)
                        | 9i32) as u32);
                }
                if (((((&raw mut sPokeblockPresentFlavors)
                    .cast::<u8>()
                    .cast::<i16>())
                .cast::<i16>())
                .read()) as i32)
                    == 4i32
                {
                    return (((((((((&raw mut sPokeblockPresentFlavors)
                        .cast::<u8>()
                        .cast::<i16>())
                    .cast::<i16>())
                    .wrapping_offset(1))
                    .read()) as i32)
                        << 16)
                        | 10i32) as u32);
                }
            } else {
                if ((((((&raw mut sPokeblockPresentFlavors)
                    .cast::<u8>()
                    .cast::<i16>())
                .cast::<i16>())
                .wrapping_offset(1))
                .read()) as i32)
                    == 0i32
                {
                    return ((((((((&raw mut sPokeblockPresentFlavors)
                        .cast::<u8>()
                        .cast::<i16>())
                    .cast::<i16>())
                    .read()) as i32)
                        << 16)
                        | 6i32) as u32);
                }
                if ((((((&raw mut sPokeblockPresentFlavors)
                    .cast::<u8>()
                    .cast::<i16>())
                .cast::<i16>())
                .wrapping_offset(1))
                .read()) as i32)
                    == 1i32
                {
                    return ((((((((&raw mut sPokeblockPresentFlavors)
                        .cast::<u8>()
                        .cast::<i16>())
                    .cast::<i16>())
                    .read()) as i32)
                        << 16)
                        | 7i32) as u32);
                }
                if ((((((&raw mut sPokeblockPresentFlavors)
                    .cast::<u8>()
                    .cast::<i16>())
                .cast::<i16>())
                .wrapping_offset(1))
                .read()) as i32)
                    == 2i32
                {
                    return ((((((((&raw mut sPokeblockPresentFlavors)
                        .cast::<u8>()
                        .cast::<i16>())
                    .cast::<i16>())
                    .read()) as i32)
                        << 16)
                        | 8i32) as u32);
                }
                if ((((((&raw mut sPokeblockPresentFlavors)
                    .cast::<u8>()
                    .cast::<i16>())
                .cast::<i16>())
                .wrapping_offset(1))
                .read()) as i32)
                    == 3i32
                {
                    return ((((((((&raw mut sPokeblockPresentFlavors)
                        .cast::<u8>()
                        .cast::<i16>())
                    .cast::<i16>())
                    .read()) as i32)
                        << 16)
                        | 9i32) as u32);
                }
                if ((((((&raw mut sPokeblockPresentFlavors)
                    .cast::<u8>()
                    .cast::<i16>())
                .cast::<i16>())
                .wrapping_offset(1))
                .read()) as i32)
                    == 4i32
                {
                    return ((((((((&raw mut sPokeblockPresentFlavors)
                        .cast::<u8>()
                        .cast::<i16>())
                    .cast::<i16>())
                    .read()) as i32)
                        << 16)
                        | 10i32) as u32);
                }
            }
        }
        return 0u32;
    }
}
pub(crate) unsafe extern "C" fn Debug_SetMaxRPMStage(value: i16) {
    unsafe {
        let mut value = value;
        ((&raw mut sDebug_MaxRPMStage).cast::<u8>().cast::<i16>()).write(value);
    }
}
pub(crate) unsafe extern "C" fn Debug_GetMaxRPMStage() -> i16 {
    unsafe {
        return ((&raw mut sDebug_MaxRPMStage).cast::<u8>().cast::<i16>()).read();
    }
}
pub(crate) unsafe extern "C" fn Debug_SetGameTimeStage(value: i16) {
    unsafe {
        let mut value = value;
        ((&raw mut sDebug_GameTimeStage).cast::<u8>().cast::<i16>()).write(value);
    }
}
pub(crate) unsafe extern "C" fn Debug_GetGameTimeStage() -> i16 {
    unsafe {
        return ((&raw mut sDebug_GameTimeStage).cast::<u8>().cast::<i16>()).read();
    }
}
pub(crate) unsafe extern "C" fn CalculatePokeblock(
    berries: *mut u8,
    pokeblock: *mut u8,
    numPlayers: u8,
    flavors: *mut u8,
    maxRPM: u16,
) {
    unsafe {
        let mut berries = berries;
        let mut pokeblock = pokeblock;
        let mut numPlayers = numPlayers;
        let mut flavors = flavors;
        let mut maxRPM = maxRPM;
        let mut i: i32 = 0i32;
        let mut j: i32 = 0i32;
        let mut multiuseVar: i32 = 0i32;
        let mut numNegatives: u8 = 0u8;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 6i32) {
                    break 'l1;
                }
                'l2: {
                    ((((&raw mut sPokeblockFlavors).cast::<u8>().cast::<i16>()).cast::<i16>())
                        .wrapping_offset((i) as isize))
                    .write(0i16);
                }
                i = (i).wrapping_add(1);
            }
        }
        {
            i = 0i32;
            'l3: loop {
                if !(i < ((numPlayers) as i32)) {
                    break 'l3;
                }
                'l4: {
                    {
                        j = 0i32;
                        'l5: loop {
                            if !(j < 6i32) {
                                break 'l5;
                            }
                            'l6: {
                                let __p1 =
                                    (((&raw mut sPokeblockFlavors).cast::<u8>().cast::<i16>())
                                        .cast::<i16>())
                                    .wrapping_offset((j) as isize);
                                (__p1).write(
                                    (((((__p1).read()) as i32).wrapping_add(
                                        (((((((berries).wrapping_offset((i) as isize * 16))
                                            .wrapping_add(9))
                                        .cast::<u8>())
                                        .wrapping_offset((j) as isize))
                                        .read()) as i32),
                                    )) as i16),
                                );
                            }
                            j = (j).wrapping_add(1);
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        multiuseVar = (((((&raw mut sPokeblockFlavors).cast::<u8>().cast::<i16>()).cast::<i16>())
            .read()) as i32);
        let __p2 = ((&raw mut sPokeblockFlavors).cast::<u8>().cast::<i16>()).cast::<i16>();
        (__p2).write(
            (((((__p2).read()) as i32).wrapping_sub(
                ((((((&raw mut sPokeblockFlavors).cast::<u8>().cast::<i16>()).cast::<i16>())
                    .wrapping_offset(1))
                .read()) as i32),
            )) as i16),
        );
        let __p3 = (((&raw mut sPokeblockFlavors).cast::<u8>().cast::<i16>()).cast::<i16>())
            .wrapping_offset(1);
        (__p3).write(
            (((((__p3).read()) as i32).wrapping_sub(
                ((((((&raw mut sPokeblockFlavors).cast::<u8>().cast::<i16>()).cast::<i16>())
                    .wrapping_offset(2))
                .read()) as i32),
            )) as i16),
        );
        let __p4 = (((&raw mut sPokeblockFlavors).cast::<u8>().cast::<i16>()).cast::<i16>())
            .wrapping_offset(2);
        (__p4).write(
            (((((__p4).read()) as i32).wrapping_sub(
                ((((((&raw mut sPokeblockFlavors).cast::<u8>().cast::<i16>()).cast::<i16>())
                    .wrapping_offset(3))
                .read()) as i32),
            )) as i16),
        );
        let __p5 = (((&raw mut sPokeblockFlavors).cast::<u8>().cast::<i16>()).cast::<i16>())
            .wrapping_offset(3);
        (__p5).write(
            (((((__p5).read()) as i32).wrapping_sub(
                ((((((&raw mut sPokeblockFlavors).cast::<u8>().cast::<i16>()).cast::<i16>())
                    .wrapping_offset(4))
                .read()) as i32),
            )) as i16),
        );
        let __p6 = (((&raw mut sPokeblockFlavors).cast::<u8>().cast::<i16>()).cast::<i16>())
            .wrapping_offset(4);
        (__p6).write((((((__p6).read()) as i32).wrapping_sub(multiuseVar)) as i16));
        multiuseVar = 0i32;
        {
            i = 0i32;
            'l7: loop {
                if !(i < 5i32) {
                    break 'l7;
                }
                'l8: {
                    if ((((((&raw mut sPokeblockFlavors).cast::<u8>().cast::<i16>())
                        .cast::<i16>())
                    .wrapping_offset((i) as isize))
                    .read()) as i32)
                        < 0i32
                    {
                        ((((&raw mut sPokeblockFlavors).cast::<u8>().cast::<i16>()).cast::<i16>())
                            .wrapping_offset((i) as isize))
                        .write(0i16);
                        multiuseVar = (multiuseVar).wrapping_add(1);
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        numNegatives = ((multiuseVar) as u8);
        {
            i = 0i32;
            'l9: loop {
                if !(i < 5i32) {
                    break 'l9;
                }
                'l10: {
                    if ((((((&raw mut sPokeblockFlavors).cast::<u8>().cast::<i16>())
                        .cast::<i16>())
                    .wrapping_offset((i) as isize))
                    .read()) as i32)
                        > 0i32
                    {
                        if ((((((&raw mut sPokeblockFlavors).cast::<u8>().cast::<i16>())
                            .cast::<i16>())
                        .wrapping_offset((i) as isize))
                        .read()) as i32)
                            < multiuseVar
                        {
                            ((((&raw mut sPokeblockFlavors).cast::<u8>().cast::<i16>())
                                .cast::<i16>())
                            .wrapping_offset((i) as isize))
                            .write(0i16);
                        } else {
                            let __p7 = (((&raw mut sPokeblockFlavors).cast::<u8>().cast::<i16>())
                                .cast::<i16>())
                            .wrapping_offset((i) as isize);
                            (__p7).write(
                                (((((__p7).read()) as i32).wrapping_sub(multiuseVar)) as i16),
                            );
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        {
            i = 0i32;
            'l11: loop {
                if !(i < 5i32) {
                    break 'l11;
                }
                'l12: {
                    ((((&raw mut sDebug_PokeblockFactorFlavors)
                        .cast::<u8>()
                        .cast::<i32>())
                    .cast::<i32>())
                    .wrapping_offset((i) as isize))
                    .write(
                        ((((((&raw mut sPokeblockFlavors).cast::<u8>().cast::<i16>())
                            .cast::<i16>())
                        .wrapping_offset((i) as isize))
                        .read()) as i32),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        ((&raw mut sDebug_PokeblockFactorRPM)
            .cast::<u8>()
            .cast::<u32>())
        .write(
            (({
                let __v8 = (crate::c::div_i32(((maxRPM) as i32), 333i32)).wrapping_add(100i32);
                multiuseVar = __v8;
                __v8
            }) as u32),
        );
        {
            i = 0i32;
            'l13: loop {
                if !(i < 5i32) {
                    break 'l13;
                }
                'l14: {
                    let mut remainder: i32 = 0i32;
                    let mut flavor: i32 =
                        ((((((&raw mut sPokeblockFlavors).cast::<u8>().cast::<i16>())
                            .cast::<i16>())
                        .wrapping_offset((i) as isize))
                        .read()) as i32);
                    flavor = crate::c::div_i32((flavor).wrapping_mul(multiuseVar), 10i32);
                    remainder = crate::c::rem_i32(flavor, 10i32);
                    flavor = crate::c::div_i32(flavor, 10i32);
                    if remainder > 4i32 {
                        flavor = (flavor).wrapping_add(1);
                    }
                    ((((&raw mut sPokeblockFlavors).cast::<u8>().cast::<i16>()).cast::<i16>())
                        .wrapping_offset((i) as isize))
                    .write(((flavor) as i16));
                }
                i = (i).wrapping_add(1);
            }
        }
        {
            i = 0i32;
            'l15: loop {
                if !(i < 5i32) {
                    break 'l15;
                }
                'l16: {
                    ((((&raw mut sDebug_PokeblockFactorFlavorsAfterRPM)
                        .cast::<u8>()
                        .cast::<i32>())
                    .cast::<i32>())
                    .wrapping_offset((i) as isize))
                    .write(
                        ((((((&raw mut sPokeblockFlavors).cast::<u8>().cast::<i16>())
                            .cast::<i16>())
                        .wrapping_offset((i) as isize))
                        .read()) as i32),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        (pokeblock).write(
            ((CalculatePokeblockColor(
                berries,
                ((&raw mut sPokeblockFlavors).cast::<u8>().cast::<i16>()).cast::<i16>(),
                numPlayers,
                numNegatives,
            )) as u8),
        );
        ((((&raw mut sPokeblockFlavors).cast::<u8>().cast::<i16>()).cast::<i16>())
            .wrapping_offset(5))
        .write(
            (((crate::c::div_i32(
                ((((((&raw mut sPokeblockFlavors).cast::<u8>().cast::<i16>()).cast::<i16>())
                    .wrapping_offset(5))
                .read()) as i32),
                ((numPlayers) as i32),
            ))
            .wrapping_sub(((numPlayers) as i32))) as i16),
        );
        if ((((((&raw mut sPokeblockFlavors).cast::<u8>().cast::<i16>()).cast::<i16>())
            .wrapping_offset(5))
        .read()) as i32)
            < 0i32
        {
            ((((&raw mut sPokeblockFlavors).cast::<u8>().cast::<i16>()).cast::<i16>())
                .wrapping_offset(5))
            .write(0i16);
        }
        if (((pokeblock).read()) as i32) == 12i32 {
            multiuseVar =
                ((crate::c::rem_u32(((Random()) as u32), crate::c::div_u32(10u32, 1u32))) as i32);
            {
                i = 0i32;
                'l17: loop {
                    if !(i < 5i32) {
                        break 'l17;
                    }
                    'l18: {
                        if (crate::c::shr_i32(
                            ((((((&raw const sBlackPokeblockFlavorFlags)
                                .cast::<u8>()
                                .cast_mut())
                            .cast::<u8>())
                            .wrapping_offset((multiuseVar) as isize))
                            .read()) as i32),
                            ((i) as u32),
                        ) & 1i32)
                            != 0
                        {
                            ((((&raw mut sPokeblockFlavors).cast::<u8>().cast::<i16>())
                                .cast::<i16>())
                            .wrapping_offset((i) as isize))
                            .write(2i16);
                        } else {
                            ((((&raw mut sPokeblockFlavors).cast::<u8>().cast::<i16>())
                                .cast::<i16>())
                            .wrapping_offset((i) as isize))
                            .write(0i16);
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
        }
        {
            i = 0i32;
            'l19: loop {
                if !(i < 6i32) {
                    break 'l19;
                }
                'l20: {
                    if ((((((&raw mut sPokeblockFlavors).cast::<u8>().cast::<i16>())
                        .cast::<i16>())
                    .wrapping_offset((i) as isize))
                    .read()) as i32)
                        > 255i32
                    {
                        ((((&raw mut sPokeblockFlavors).cast::<u8>().cast::<i16>()).cast::<i16>())
                            .wrapping_offset((i) as isize))
                        .write(255i16);
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        ((pokeblock).wrapping_add(1)).write(
            (((((&raw mut sPokeblockFlavors).cast::<u8>().cast::<i16>()).cast::<i16>()).read())
                as u8),
        );
        ((pokeblock).wrapping_add(2)).write(
            ((((((&raw mut sPokeblockFlavors).cast::<u8>().cast::<i16>()).cast::<i16>())
                .wrapping_offset(1))
            .read()) as u8),
        );
        ((pokeblock).wrapping_add(3)).write(
            ((((((&raw mut sPokeblockFlavors).cast::<u8>().cast::<i16>()).cast::<i16>())
                .wrapping_offset(2))
            .read()) as u8),
        );
        ((pokeblock).wrapping_add(4)).write(
            ((((((&raw mut sPokeblockFlavors).cast::<u8>().cast::<i16>()).cast::<i16>())
                .wrapping_offset(3))
            .read()) as u8),
        );
        ((pokeblock).wrapping_add(5)).write(
            ((((((&raw mut sPokeblockFlavors).cast::<u8>().cast::<i16>()).cast::<i16>())
                .wrapping_offset(4))
            .read()) as u8),
        );
        ((pokeblock).wrapping_add(6)).write(
            ((((((&raw mut sPokeblockFlavors).cast::<u8>().cast::<i16>()).cast::<i16>())
                .wrapping_offset(5))
            .read()) as u8),
        );
        {
            i = 0i32;
            'l21: loop {
                if !(i < 6i32) {
                    break 'l21;
                }
                'l22: {
                    ((flavors).wrapping_offset((i) as isize)).write(
                        ((((((&raw mut sPokeblockFlavors).cast::<u8>().cast::<i16>())
                            .cast::<i16>())
                        .wrapping_offset((i) as isize))
                        .read()) as u8),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Debug_CalculatePokeblock(
    berries: *mut u8,
    pokeblock: *mut u8,
    numPlayers: u8,
    flavors: *mut u8,
    maxRPM: u16,
) {
    unsafe {
        let mut berries = berries;
        let mut pokeblock = pokeblock;
        let mut numPlayers = numPlayers;
        let mut flavors = flavors;
        let mut maxRPM = maxRPM;
        CalculatePokeblock(berries, pokeblock, numPlayers, flavors, maxRPM);
    }
}
pub(crate) unsafe extern "C" fn Debug_SetStageVars() {
    unsafe {
        let mut frames: u32 = (((((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>())
            .read())
        .wrapping_add(260)
        .cast::<u32>())
        .read()) as u16) as u32);
        let mut maxRPM: u16 = ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(78)
            .cast::<u16>())
        .read();
        let mut stage: i16 = 0i16;
        if frames < 900u32 {
            stage = 5i16;
        } else {
            if ((((frames).wrapping_sub(900u32)) as u16) as i32) < 600i32 {
                stage = 4i16;
            } else {
                if ((((frames).wrapping_sub(1500u32)) as u16) as i32) < 600i32 {
                    stage = 3i16;
                } else {
                    if ((((frames).wrapping_sub(2100u32)) as u16) as i32) < 900i32 {
                        stage = 2i16;
                    } else {
                        if ((((frames).wrapping_sub(3300u32)) as u16) as i32) < 300i32 {
                            stage = 1i16;
                        }
                    }
                }
            }
        }
        Debug_SetGameTimeStage(stage);
        stage = 0i16;
        if ((maxRPM) as i32) <= 64i32 {
            if (((maxRPM) as i32) >= 50i32) && (((maxRPM) as i32) < 100i32) {
                stage = (-1i16);
            } else {
                if (((maxRPM) as i32) >= 100i32) && (((maxRPM) as i32) < 150i32) {
                    stage = (-2i16);
                } else {
                    if (((maxRPM) as i32) >= 150i32) && (((maxRPM) as i32) < 200i32) {
                        stage = (-3i16);
                    } else {
                        if (((maxRPM) as i32) >= 200i32) && (((maxRPM) as i32) < 250i32) {
                            stage = (-4i16);
                        } else {
                            if (((maxRPM) as i32) >= 250i32) && (((maxRPM) as i32) < 300i32) {
                                stage = (-5i16);
                            } else {
                                if (((maxRPM) as i32) >= 350i32) && (((maxRPM) as i32) < 400i32) {
                                    stage = (-6i16);
                                } else {
                                    if (((maxRPM) as i32) >= 400i32) && (((maxRPM) as i32) < 450i32)
                                    {
                                        stage = (-7i16);
                                    } else {
                                        if (((maxRPM) as i32) >= 500i32)
                                            && (((maxRPM) as i32) < 550i32)
                                        {
                                            stage = (-8i16);
                                        } else {
                                            if (((maxRPM) as i32) >= 550i32)
                                                && (((maxRPM) as i32) < 600i32)
                                            {
                                                stage = (-9i16);
                                            } else {
                                                if ((maxRPM) as i32) >= 600i32 {
                                                    stage = (-10i16);
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        Debug_SetMaxRPMStage(stage);
    }
}
pub(crate) unsafe extern "C" fn SendContinuePromptResponse(cmd: *mut u16) {
    unsafe {
        let mut cmd = cmd;
        if ((((&raw mut gReceivedRemoteLinkPlayers).cast::<u8>()).read()) != 0)
            && ((((&raw mut gWirelessCommType).cast::<u8>()).read()) != 0)
        {
            (cmd).write(12032u16);
        } else {
            (cmd).write(12287u16);
        }
    }
}
pub(crate) unsafe extern "C" fn CB2_EndBlenderGame() {
    unsafe {
        let mut i: u8 = 0u8;
        let mut j: u8 = 0u8;
        if ((((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(99))
            .read()) as i32)
            < 3i32
        {
            UpdateBlenderCenter();
        }
        GetMultiplayerId();
        'l1: {
            let __sw1 = ((((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(99))
            .read()) as i32);
            if __sw1 == 1i32 {
                m4aMPlayTempoControl((&raw mut gMPlayInfo_BGM).cast::<u8>(), 256u16);
                {
                    i = 0u8;
                    'l2: loop {
                        if !(((i) as i32)
                            < ((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32))
                        {
                            break 'l2;
                        }
                        'l3: {
                            DestroyTask(
                                ((((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(288))
                                .cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize))
                                .read(),
                            );
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                let __p2 = (((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(99);
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 2i32 {
                let __p3 = (((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(76)
                    .cast::<i16>();
                (__p3).write((((((__p3).read()) as i32).wrapping_sub(32i32)) as i16));
                if ((((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(76)
                    .cast::<i16>())
                .read()) as i32)
                    <= 0i32
                {
                    ClearLinkCallback();
                    ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(76)
                        .cast::<i16>())
                    .write(0i16);
                    if (((&raw mut gReceivedRemoteLinkPlayers).cast::<u8>()).read()) != 0 {
                        let __p4 = (((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(99);
                        (__p4).write(((__p4).read()).wrapping_add(1));
                    } else {
                        ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(99))
                        .write(5u8);
                    }
                    (((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read()).write(0u8);
                    m4aMPlayStop((&raw mut gMPlayInfo_SE2).cast::<u8>());
                }
                UpdateHitPitch();
                break 'l1;
            }
            if __sw1 == 3i32 {
                if ((GetMultiplayerId()) as i32) != 0i32 {
                    let __p5 = (((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(99);
                    (__p5).write(((__p5).read()).wrapping_add(1));
                } else {
                    if (IsLinkTaskFinished()) != 0 {
                        if ((((&raw mut gReceivedRemoteLinkPlayers).cast::<u8>()).read()) != 0)
                            && ((((&raw mut gWirelessCommType).cast::<u8>()).read()) != 0)
                        {
                            (((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(4544))
                            .cast::<u32>())
                            .write(
                                ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(260)
                                .cast::<u32>())
                                .read(),
                            );
                            (((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(4544))
                            .wrapping_add(4)
                            .cast::<u16>())
                            .write(
                                ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(78)
                                .cast::<u16>())
                                .read(),
                            );
                            {
                                i = 0u8;
                                'l4: loop {
                                    if !(((i) as i32) < 4i32) {
                                        break 'l4;
                                    }
                                    'l5: {
                                        {
                                            j = 0u8;
                                            'l6: loop {
                                                if !(((j) as i32) < 3i32) {
                                                    break 'l6;
                                                }
                                                'l7: {
                                                    (((((((((&raw mut sBerryBlender)
                                                        .cast::<u8>()
                                                        .cast::<*mut u8>())
                                                    .read())
                                                    .wrapping_add(4544))
                                                    .wrapping_add(8))
                                                    .cast::<u8>())
                                                    .wrapping_offset(((i) as i32) as isize * 6))
                                                    .cast::<u16>())
                                                    .wrapping_offset(((j) as i32) as isize))
                                                    .write(
                                                        ((((((((&raw mut sBerryBlender)
                                                            .cast::<u8>()
                                                            .cast::<*mut u8>())
                                                        .read())
                                                        .wrapping_add(292))
                                                        .cast::<u8>())
                                                        .wrapping_offset(
                                                            ((i) as i32) as isize * 6,
                                                        ))
                                                        .cast::<u16>())
                                                        .wrapping_offset(((j) as i32) as isize))
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
                            if (SendBlock(
                                0u8,
                                (((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                                    .wrapping_add(4544),
                                32u16,
                            )) != 0
                            {
                                let __p6 =
                                    (((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(99);
                                (__p6).write(((__p6).read()).wrapping_add(1));
                            }
                        } else {
                            (((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(408))
                            .cast::<u32>())
                            .write(
                                ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(260)
                                .cast::<u32>())
                                .read(),
                            );
                            (((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(408))
                            .wrapping_add(4)
                            .cast::<u16>())
                            .write(
                                ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(78)
                                .cast::<u16>())
                                .read(),
                            );
                            if (SendBlock(
                                0u8,
                                (((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                                    .wrapping_add(408),
                                40u16,
                            )) != 0
                            {
                                let __p7 =
                                    (((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(99);
                                (__p7).write(((__p7).read()).wrapping_add(1));
                            }
                        }
                    }
                }
                break 'l1;
            }
            if __sw1 == 4i32 {
                if (GetBlockReceivedStatus()) != 0 {
                    ResetBlockReceivedFlags();
                    let __p8 = (((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(99);
                    (__p8).write(((__p8).read()).wrapping_add(1));
                    if ((((&raw mut gReceivedRemoteLinkPlayers).cast::<u8>()).read()) != 0)
                        && ((((&raw mut gWirelessCommType).cast::<u8>()).read()) != 0)
                    {
                        let mut receivedBlock: *mut u8 =
                            ((&raw mut gBlockRecvBuffer).cast::<u8>()).cast::<u8>();
                        ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(78)
                            .cast::<u16>())
                        .write(((receivedBlock).wrapping_add(4).cast::<u16>()).read());
                        ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(260)
                            .cast::<u32>())
                        .write(((receivedBlock).cast::<u32>()).read());
                        {
                            i = 0u8;
                            'l8: loop {
                                if !(((i) as i32) < 4i32) {
                                    break 'l8;
                                }
                                'l9: {
                                    {
                                        j = 0u8;
                                        'l10: loop {
                                            if !(((j) as i32) < 3i32) {
                                                break 'l10;
                                            }
                                            'l11: {
                                                ((((((((&raw mut sBerryBlender)
                                                    .cast::<u8>()
                                                    .cast::<*mut u8>())
                                                .read())
                                                .wrapping_add(292))
                                                .cast::<u8>())
                                                .wrapping_offset(((i) as i32) as isize * 6))
                                                .cast::<u16>())
                                                .wrapping_offset(((j) as i32) as isize))
                                                .write(
                                                    ((((((receivedBlock).wrapping_add(8))
                                                        .cast::<u8>())
                                                    .wrapping_offset(((i) as i32) as isize * 6))
                                                    .cast::<u16>())
                                                    .wrapping_offset(((j) as i32) as isize))
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
                    } else {
                        let mut receivedBlock: *mut u8 =
                            ((&raw mut gBlockRecvBuffer).cast::<u8>()).cast::<u8>();
                        ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(78)
                            .cast::<u16>())
                        .write(((receivedBlock).wrapping_add(4).cast::<u16>()).read());
                        ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(260)
                            .cast::<u32>())
                        .write(((receivedBlock).cast::<u32>()).read());
                    }
                }
                break 'l1;
            }
            if __sw1 == 5i32 {
                if (PrintBlendingRanking()) != 0 {
                    let __p9 = (((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(99);
                    (__p9).write(((__p9).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 6i32 {
                if (PrintBlendingResults()) != 0 {
                    if ((((&raw mut gInGameOpponentsNo).cast::<u8>().cast::<u8>()).read()) as i32)
                        == 0i32
                    {
                        IncrementGameStat(34u8);
                    } else {
                        IncrementGameStat(33u8);
                    }
                    let __p10 = (((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(99);
                    (__p10).write(((__p10).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 7i32 {
                if (PrintMessage(
                    (((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(4536)
                        .cast::<i16>(),
                    ((&raw const sText_WouldLikeToBlendAnotherBerry)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>(),
                    ((GetPlayerTextSpeedDelay()) as i32),
                )) != 0
                {
                    let __p11 = (((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(99);
                    (__p11).write(((__p11).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 9i32 {
                ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(158))
                .write(0u8);
                CreateYesNoMenu(
                    (&raw const sYesNoWindowTemplate_ContinuePlaying)
                        .cast::<u8>()
                        .cast_mut(),
                    1u16,
                    13u8,
                    0u8,
                );
                let __p12 = (((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(99);
                (__p12).write(((__p12).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 10i32 {
                'l12: {
                    let __sw13 = ((Menu_ProcessInputNoWrapClearOnChoose()) as i32);
                    if __sw13 == 1i32 || __sw13 == (-1i32) {
                        ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(158))
                        .write(1u8);
                        let __p14 = (((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(99);
                        (__p14).write(((__p14).read()).wrapping_add(1));
                        {
                            i = 0u8;
                            'l13: loop {
                                if !(((i) as i32) < 4i32) {
                                    break 'l13;
                                }
                                'l14: {
                                    if ((((((((&raw mut sBerryBlender)
                                        .cast::<u8>()
                                        .cast::<*mut u8>())
                                    .read())
                                    .wrapping_add(142))
                                    .cast::<u16>())
                                    .wrapping_offset(((i) as i32) as isize))
                                    .read()) as i32)
                                        != 255i32
                                    {
                                        PutWindowTilemap(i);
                                        CopyWindowToVram(i, 3u8);
                                    }
                                }
                                i = (i).wrapping_add(1);
                            }
                        }
                        break 'l12;
                    }
                    if __sw13 == 0i32 {
                        ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(158))
                        .write(0u8);
                        let __p15 = (((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(99);
                        (__p15).write(((__p15).read()).wrapping_add(1));
                        {
                            i = 0u8;
                            'l15: loop {
                                if !(((i) as i32) < 4i32) {
                                    break 'l15;
                                }
                                'l16: {
                                    if ((((((((&raw mut sBerryBlender)
                                        .cast::<u8>()
                                        .cast::<*mut u8>())
                                    .read())
                                    .wrapping_add(142))
                                    .cast::<u16>())
                                    .wrapping_offset(((i) as i32) as isize))
                                    .read()) as i32)
                                        != 255i32
                                    {
                                        PutWindowTilemap(i);
                                        CopyWindowToVram(i, 3u8);
                                    }
                                }
                                i = (i).wrapping_add(1);
                            }
                        }
                        break 'l12;
                    }
                }
                break 'l1;
            }
            if __sw1 == 11i32 {
                SendContinuePromptResponse(((&raw mut gSendCmd).cast::<u16>()).cast::<u16>());
                if ((((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(158))
                .read()) as i32)
                    == 0i32
                {
                    if ((IsBagPocketNonEmpty(4u8)) as i32) == 0i32 {
                        ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(112)
                            .cast::<u16>())
                        .write(2u16);
                        ((((&raw mut gSendCmd).cast::<u16>()).cast::<u16>()).wrapping_offset(1))
                            .write(39321u16);
                    } else {
                        if ((GetFirstFreePokeblockSlot()) as i32) == (-1i32) {
                            ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(112)
                                .cast::<u16>())
                            .write(3u16);
                            ((((&raw mut gSendCmd).cast::<u16>()).cast::<u16>())
                                .wrapping_offset(1))
                            .write(43690u16);
                        } else {
                            ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(112)
                                .cast::<u16>())
                            .write(0u16);
                            ((((&raw mut gSendCmd).cast::<u16>()).cast::<u16>())
                                .wrapping_offset(1))
                            .write(30585u16);
                        }
                    }
                    let __p16 = (((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(99);
                    (__p16).write(((__p16).read()).wrapping_add(1));
                } else {
                    ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(112)
                        .cast::<u16>())
                    .write(1u16);
                    ((((&raw mut gSendCmd).cast::<u16>()).cast::<u16>()).wrapping_offset(1))
                        .write(34952u16);
                    let __p17 = (((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(99);
                    (__p17).write(((__p17).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 12i32 {
                if (((&raw mut gInGameOpponentsNo).cast::<u8>().cast::<u8>()).read()) != 0 {
                    SetMainCallback2(Some(CB2_CheckPlayAgainLocal));
                    ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(99))
                    .write(0u8);
                    (((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read()).write(0u8);
                } else {
                    let __p18 = (((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(99);
                    (__p18).write(((__p18).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 8i32 {
                let __p19 = (((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(99);
                (__p19).write(((__p19).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 13i32 {
                if (PrintMessage(
                    (((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(4536)
                        .cast::<i16>(),
                    ((&raw const sText_CommunicationStandby)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>(),
                    ((GetPlayerTextSpeedDelay()) as i32),
                )) != 0
                {
                    SetMainCallback2(Some(CB2_CheckPlayAgainLink));
                    ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(99))
                    .write(0u8);
                    (((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read()).write(0u8);
                }
                break 'l1;
            }
        }
        RestoreBgCoords();
        UpdateRPM(
            ((((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(76)
                .cast::<i16>())
            .read()) as u16),
        );
        ProcessLinkPlayerCmds();
        Blender_DummiedOutFunc(
            ((((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(284)
                .cast::<u16>())
            .read()) as i16),
            ((((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(286)
                .cast::<u16>())
            .read()) as i16),
        );
        RunTasks();
        AnimateSprites();
        BuildOamBuffer();
        RunTextPrinters();
        UpdatePaletteFade();
    }
}
pub(crate) unsafe extern "C" fn LinkPlayAgainHandleSaving() -> u8 {
    unsafe {
        'l1: {
            let __sw1 = ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(416)
                .cast::<u32>())
            .read();
            if __sw1 == 0u32 {
                SetLinkStandbyCallback();
                ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(416)
                    .cast::<u32>())
                .write(1u32);
                ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(264)
                    .cast::<i32>())
                .write(0i32);
                break 'l1;
            }
            if __sw1 == 1u32 {
                if (IsLinkTaskFinished()) != 0 {
                    let __p2 = (((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(416)
                        .cast::<u32>();
                    (__p2).write(((__p2).read()).wrapping_add(1));
                    ((&raw mut gSoftResetDisabled).cast::<u8>()).write(1u8);
                }
                break 'l1;
            }
            if __sw1 == 2u32 {
                WriteSaveBlock2();
                let __p3 = (((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(416)
                    .cast::<u32>();
                (__p3).write(((__p3).read()).wrapping_add(1));
                ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(264)
                    .cast::<i32>())
                .write(0i32);
                break 'l1;
            }
            if __sw1 == 3u32 {
                if {
                    let __p4 = (((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(264)
                        .cast::<i32>();
                    let __t5 = ((__p4).read()).wrapping_add(1);
                    (__p4).write(__t5);
                    __t5
                } == 10i32
                {
                    SetLinkStandbyCallback();
                    let __p6 = (((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(416)
                        .cast::<u32>();
                    (__p6).write(((__p6).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 4u32 {
                if (IsLinkTaskFinished()) != 0 {
                    if (WriteSaveBlock1Sector()) != 0 {
                        ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(416)
                            .cast::<u32>())
                        .write(5u32);
                    } else {
                        ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(264)
                            .cast::<i32>())
                        .write(0i32);
                        ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(416)
                            .cast::<u32>())
                        .write(3u32);
                    }
                }
                break 'l1;
            }
            if __sw1 == 5u32 {
                let __p7 = (((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(416)
                    .cast::<u32>();
                (__p7).write(((__p7).read()).wrapping_add(1));
                ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(264)
                    .cast::<i32>())
                .write(0i32);
                break 'l1;
            }
            if __sw1 == 6u32 {
                if {
                    let __p8 = (((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(264)
                        .cast::<i32>();
                    let __t9 = ((__p8).read()).wrapping_add(1);
                    (__p8).write(__t9);
                    __t9
                } > 5i32
                {
                    ((&raw mut gSoftResetDisabled).cast::<u8>()).write(0u8);
                    return 1u8;
                }
                break 'l1;
            }
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn CB2_CheckPlayAgainLink() {
    unsafe {
        'l1: {
            let __sw1 = ((((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(99))
            .read()) as i32);
            if __sw1 == 0i32 {
                if (((((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(100))
                .cast::<u16>())
                .read()) as i32)
                    == 8738i32
                {
                    ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(99))
                    .write(5u8);
                } else {
                    if (((((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(100))
                    .cast::<u16>())
                    .read()) as i32)
                        == 4369i32
                    {
                        if ((((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(108)
                            .cast::<u16>())
                        .read()) as i32)
                            == 39321i32
                        {
                            ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(99))
                            .write(2u8);
                        } else {
                            if ((((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(108)
                            .cast::<u16>())
                            .read()) as i32)
                                == 43690i32
                            {
                                ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(99))
                                .write(1u8);
                            } else {
                                ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(99))
                                .write(5u8);
                            }
                        }
                    }
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(99))
                .write(3u8);
                StringCopy(
                    (&raw mut gStringVar4).cast::<u8>(),
                    ((((&raw mut gLinkPlayers).cast::<u8>()).wrapping_offset(
                        ((((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(110)
                            .cast::<u16>())
                        .read()) as i32) as isize
                            * 28,
                    ))
                    .wrapping_add(8))
                    .cast::<u8>(),
                );
                StringAppend(
                    (&raw mut gStringVar4).cast::<u8>(),
                    ((&raw const sText_ApostropheSPokeblockCaseIsFull)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>(),
                );
                break 'l1;
            }
            if __sw1 == 2i32 {
                let __p2 = (((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(99);
                (__p2).write(((__p2).read()).wrapping_add(1));
                StringCopy(
                    (&raw mut gStringVar4).cast::<u8>(),
                    ((((&raw mut gLinkPlayers).cast::<u8>()).wrapping_offset(
                        ((((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(110)
                            .cast::<u16>())
                        .read()) as i32) as isize
                            * 28,
                    ))
                    .wrapping_add(8))
                    .cast::<u8>(),
                );
                StringAppend(
                    (&raw mut gStringVar4).cast::<u8>(),
                    ((&raw const sText_HasNoBerriesToPut).cast::<u8>().cast_mut()).cast::<u8>(),
                );
                break 'l1;
            }
            if __sw1 == 3i32 {
                if (PrintMessage(
                    (((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(4536)
                        .cast::<i16>(),
                    (&raw mut gStringVar4).cast::<u8>(),
                    ((GetPlayerTextSpeedDelay()) as i32),
                )) != 0
                {
                    ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(264)
                        .cast::<i32>())
                    .write(0i32);
                    let __p3 = (((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(99);
                    (__p3).write(((__p3).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 4i32 {
                if {
                    let __p4 = (((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(264)
                        .cast::<i32>();
                    let __t5 = ((__p4).read()).wrapping_add(1);
                    (__p4).write(__t5);
                    __t5
                } > 60i32
                {
                    ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(99))
                    .write(5u8);
                }
                break 'l1;
            }
            if __sw1 == 5i32 {
                PrintMessage(
                    (((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(4536)
                        .cast::<i16>(),
                    (&raw mut gText_SavingDontTurnOff2).cast::<u8>(),
                    0i32,
                );
                SetLinkStandbyCallback();
                let __p6 = (((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(99);
                (__p6).write(((__p6).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 6i32 {
                if (IsLinkTaskFinished()) != 0 {
                    ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(264)
                        .cast::<i32>())
                    .write(0i32);
                    let __p7 = (((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(99);
                    (__p7).write(((__p7).read()).wrapping_add(1));
                    ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(416)
                        .cast::<u32>())
                    .write(0u32);
                }
                break 'l1;
            }
            if __sw1 == 7i32 {
                if (LinkPlayAgainHandleSaving()) != 0 {
                    PlaySE(55u16);
                    let __p8 = (((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(99);
                    (__p8).write(((__p8).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 8i32 {
                let __p9 = (((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(99);
                (__p9).write(((__p9).read()).wrapping_add(1));
                SetLinkStandbyCallback();
                break 'l1;
            }
            if __sw1 == 9i32 {
                if (IsLinkTaskFinished()) != 0 {
                    BeginNormalPaletteFade(4294967295u32, 0i8, 0u8, 16u8, 0u16);
                    let __p10 = (((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(99);
                    (__p10).write(((__p10).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 10i32 {
                if !((crate::c::bf_read(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
                    7,
                    1,
                    false,
                ) as u16)
                    != 0)
                {
                    if (((((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(100))
                    .cast::<u16>())
                    .read()) as i32)
                        == 8738i32
                    {
                        FreeAllWindowBuffers();
                        UnsetBgTilemapBuffer(2u8);
                        UnsetBgTilemapBuffer(1u8);
                        {
                            Free(((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read());
                            ((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>())
                                .write(core::ptr::null_mut());
                        }
                        SetMainCallback2(Some(DoBerryBlending));
                    } else {
                        ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(264)
                            .cast::<i32>())
                        .write(0i32);
                        let __p11 = (((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(99);
                        (__p11).write(((__p11).read()).wrapping_add(1));
                    }
                }
                break 'l1;
            }
            if __sw1 == 11i32 {
                if {
                    let __p12 = (((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(264)
                        .cast::<i32>();
                    let __t13 = ((__p12).read()).wrapping_add(1);
                    (__p12).write(__t13);
                    __t13
                } > 30i32
                {
                    SetCloseLinkCallback();
                    let __p14 = (((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(99);
                    (__p14).write(((__p14).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 12i32 {
                if !((((&raw mut gReceivedRemoteLinkPlayers).cast::<u8>()).read()) != 0) {
                    {
                        Free(((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read());
                        ((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>())
                            .write(core::ptr::null_mut());
                    }
                    SetMainCallback2(Some(CB2_ReturnToFieldContinueScriptPlayMapMusic));
                }
                break 'l1;
            }
        }
        ProcessLinkPlayerCmds();
        Blender_DummiedOutFunc(
            ((((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(284)
                .cast::<u16>())
            .read()) as i16),
            ((((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(286)
                .cast::<u16>())
            .read()) as i16),
        );
        RunTasks();
        AnimateSprites();
        BuildOamBuffer();
        RunTextPrinters();
        UpdatePaletteFade();
    }
}
pub(crate) unsafe extern "C" fn CB2_CheckPlayAgainLocal() {
    unsafe {
        'l1: {
            let __sw1 = ((((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(99))
            .read()) as i32);
            if __sw1 == 0i32 {
                if (((((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(112)
                    .cast::<u16>())
                .read()) as i32)
                    == 0i32)
                    || (((((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(112)
                        .cast::<u16>())
                    .read()) as i32)
                        == 1i32)
                {
                    ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(99))
                    .write(9u8);
                }
                if ((((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(112)
                    .cast::<u16>())
                .read()) as i32)
                    == 2i32
                {
                    ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(99))
                    .write(2u8);
                }
                if ((((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(112)
                    .cast::<u16>())
                .read()) as i32)
                    == 3i32
                {
                    ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(99))
                    .write(1u8);
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(99))
                .write(3u8);
                ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(4536)
                    .cast::<i16>())
                .write(0i16);
                StringCopy(
                    (&raw mut gStringVar4).cast::<u8>(),
                    ((&raw const sText_YourPokeblockCaseIsFull)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>(),
                );
                break 'l1;
            }
            if __sw1 == 2i32 {
                let __p2 = (((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(99);
                (__p2).write(((__p2).read()).wrapping_add(1));
                ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(4536)
                    .cast::<i16>())
                .write(0i16);
                StringCopy(
                    (&raw mut gStringVar4).cast::<u8>(),
                    ((&raw const sText_RunOutOfBerriesForBlending)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>(),
                );
                break 'l1;
            }
            if __sw1 == 3i32 {
                if (PrintMessage(
                    (((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(4536)
                        .cast::<i16>(),
                    (&raw mut gStringVar4).cast::<u8>(),
                    ((GetPlayerTextSpeedDelay()) as i32),
                )) != 0
                {
                    ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(99))
                    .write(9u8);
                }
                break 'l1;
            }
            if __sw1 == 9i32 {
                BeginFastPaletteFade(3u8);
                let __p3 = (((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(99);
                (__p3).write(((__p3).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 10i32 {
                if !((crate::c::bf_read(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
                    7,
                    1,
                    false,
                ) as u16)
                    != 0)
                {
                    if ((((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(112)
                        .cast::<u16>())
                    .read()) as i32)
                        == 0i32
                    {
                        SetMainCallback2(Some(DoBerryBlending));
                    } else {
                        SetMainCallback2(Some(CB2_ReturnToFieldContinueScriptPlayMapMusic));
                    }
                    FreeAllWindowBuffers();
                    UnsetBgTilemapBuffer(2u8);
                    UnsetBgTilemapBuffer(1u8);
                    {
                        Free(((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read());
                        ((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>())
                            .write(core::ptr::null_mut());
                    }
                }
                break 'l1;
            }
        }
        ProcessLinkPlayerCmds();
        Blender_DummiedOutFunc(
            ((((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(284)
                .cast::<u16>())
            .read()) as i16),
            ((((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(286)
                .cast::<u16>())
            .read()) as i16),
        );
        RunTasks();
        AnimateSprites();
        BuildOamBuffer();
        RunTextPrinters();
        UpdatePaletteFade();
    }
}
pub(crate) unsafe extern "C" fn ProcessLinkPlayerCmds() {
    unsafe {
        if (((&raw mut gReceivedRemoteLinkPlayers).cast::<u8>()).read()) != 0 {
            if (CheckRecvCmdMatches(
                (((&raw mut gRecvCmds).cast::<u8>()).cast::<u16>()).read(),
                12287u16,
                12032u16,
            )) != 0
            {
                if ((((((&raw mut gRecvCmds).cast::<u8>()).cast::<u16>()).wrapping_offset(1))
                    .read()) as i32)
                    == 4369i32
                {
                    'l1: {
                        let __sw1 = ((((((&raw mut gRecvCmds).cast::<u8>()).cast::<u16>())
                            .wrapping_offset(2))
                        .read()) as i32);
                        if __sw1 == 34952i32 {
                            ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(108)
                                .cast::<u16>())
                            .write(34952u16);
                            ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(110)
                                .cast::<u16>())
                            .write(
                                ((((&raw mut gRecvCmds).cast::<u8>()).cast::<u16>())
                                    .wrapping_offset(3))
                                .read(),
                            );
                            break 'l1;
                        }
                        if __sw1 == 39321i32 {
                            ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(108)
                                .cast::<u16>())
                            .write(39321u16);
                            ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(110)
                                .cast::<u16>())
                            .write(
                                ((((&raw mut gRecvCmds).cast::<u8>()).cast::<u16>())
                                    .wrapping_offset(3))
                                .read(),
                            );
                            break 'l1;
                        }
                        if __sw1 == 43690i32 {
                            ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(108)
                                .cast::<u16>())
                            .write(43690u16);
                            ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(110)
                                .cast::<u16>())
                            .write(
                                ((((&raw mut gRecvCmds).cast::<u8>()).cast::<u16>())
                                    .wrapping_offset(3))
                                .read(),
                            );
                            break 'l1;
                        }
                    }
                    (((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(100))
                    .cast::<u16>())
                    .write(4369u16);
                } else {
                    if ((((((&raw mut gRecvCmds).cast::<u8>()).cast::<u16>()).wrapping_offset(1))
                        .read()) as i32)
                        == 8738i32
                    {
                        (((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(100))
                        .cast::<u16>())
                        .write(8738u16);
                    }
                }
            }
            if ((((GetMultiplayerId()) as i32) == 0i32)
                && ((((((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(100))
                .cast::<u16>())
                .read()) as i32)
                    != 4369i32))
                && ((((((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(100))
                .cast::<u16>())
                .read()) as i32)
                    != 8738i32)
            {
                let mut i: u8 = 0u8;
                {
                    i = 0u8;
                    'l2: loop {
                        if !(((i) as i32) < ((GetLinkPlayerCount()) as i32)) {
                            break 'l2;
                        }
                        'l3: {
                            if (CheckRecvCmdMatches(
                                ((((&raw mut gRecvCmds).cast::<u8>())
                                    .wrapping_offset(((i) as i32) as isize * 16))
                                .cast::<u16>())
                                .read(),
                                12287u16,
                                12032u16,
                            )) != 0
                            {
                                'l4: {
                                    let __sw2 = (((((((&raw mut gRecvCmds).cast::<u8>())
                                        .wrapping_offset(((i) as i32) as isize * 16))
                                    .cast::<u16>())
                                    .wrapping_offset(1))
                                    .read())
                                        as i32);
                                    if __sw2 == 34952i32 {
                                        ((((((&raw mut sBerryBlender)
                                            .cast::<u8>()
                                            .cast::<*mut u8>())
                                        .read())
                                        .wrapping_add(100))
                                        .cast::<u16>())
                                        .wrapping_offset(((i) as i32) as isize))
                                        .write(34952u16);
                                        break 'l4;
                                    }
                                    if __sw2 == 30585i32 {
                                        ((((((&raw mut sBerryBlender)
                                            .cast::<u8>()
                                            .cast::<*mut u8>())
                                        .read())
                                        .wrapping_add(100))
                                        .cast::<u16>())
                                        .wrapping_offset(((i) as i32) as isize))
                                        .write(30585u16);
                                        break 'l4;
                                    }
                                    if __sw2 == 39321i32 {
                                        ((((((&raw mut sBerryBlender)
                                            .cast::<u8>()
                                            .cast::<*mut u8>())
                                        .read())
                                        .wrapping_add(100))
                                        .cast::<u16>())
                                        .wrapping_offset(((i) as i32) as isize))
                                        .write(39321u16);
                                        break 'l4;
                                    }
                                    if __sw2 == 43690i32 {
                                        ((((((&raw mut sBerryBlender)
                                            .cast::<u8>()
                                            .cast::<*mut u8>())
                                        .read())
                                        .wrapping_add(100))
                                        .cast::<u16>())
                                        .wrapping_offset(((i) as i32) as isize))
                                        .write(43690u16);
                                        break 'l4;
                                    }
                                }
                            }
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                {
                    i = 0u8;
                    'l5: loop {
                        if !(((i) as i32) < ((GetLinkPlayerCount()) as i32)) {
                            break 'l5;
                        }
                        'l6: {
                            if ((((((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(100))
                            .cast::<u16>())
                            .wrapping_offset(((i) as i32) as isize))
                            .read()) as i32)
                                == 0i32
                            {
                                break 'l5;
                            }
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                if ((i) as i32) == ((GetLinkPlayerCount()) as i32) {
                    {
                        i = 0u8;
                        'l7: loop {
                            if !(((i) as i32) < ((GetLinkPlayerCount()) as i32)) {
                                break 'l7;
                            }
                            'l8: {
                                if ((((((((&raw mut sBerryBlender)
                                    .cast::<u8>()
                                    .cast::<*mut u8>())
                                .read())
                                .wrapping_add(100))
                                .cast::<u16>())
                                .wrapping_offset(((i) as i32) as isize))
                                .read()) as i32)
                                    != 30585i32
                                {
                                    break 'l7;
                                }
                            }
                            i = (i).wrapping_add(1);
                        }
                    }
                    SendContinuePromptResponse(((&raw mut gSendCmd).cast::<u16>()).cast::<u16>());
                    if ((i) as i32) == ((GetLinkPlayerCount()) as i32) {
                        ((((&raw mut gSendCmd).cast::<u16>()).cast::<u16>()).wrapping_offset(1))
                            .write(8738u16);
                    } else {
                        ((((&raw mut gSendCmd).cast::<u16>()).cast::<u16>()).wrapping_offset(1))
                            .write(4369u16);
                        ((((&raw mut gSendCmd).cast::<u16>()).cast::<u16>()).wrapping_offset(2))
                            .write(
                                ((((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(100))
                                .cast::<u16>())
                                .wrapping_offset(((i) as i32) as isize))
                                .read(),
                            );
                        ((((&raw mut gSendCmd).cast::<u16>()).cast::<u16>()).wrapping_offset(3))
                            .write(((i) as u16));
                    }
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn DrawBlenderCenter(dest: *mut u8) {
    unsafe {
        let mut dest = dest;
        let mut affineSrc = crate::ffi::Align4([0u8; 20]);
        (((&raw mut affineSrc).cast::<u8>()).cast::<i32>())
            .write((crate::c::div_i32(240i32, 2i32) << 8));
        (((&raw mut affineSrc).cast::<u8>())
            .wrapping_add(4)
            .cast::<i32>())
        .write((crate::c::div_i32(160i32, 2i32) << 8));
        (((&raw mut affineSrc).cast::<u8>())
            .wrapping_add(8)
            .cast::<i16>())
        .write(
            (((crate::c::div_i32(240i32, 2i32)).wrapping_sub(
                ((((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(284)
                    .cast::<u16>())
                .read()) as i32),
            )) as i16),
        );
        (((&raw mut affineSrc).cast::<u8>())
            .wrapping_add(10)
            .cast::<i16>())
        .write(
            (((crate::c::div_i32(160i32, 2i32)).wrapping_sub(
                ((((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(286)
                    .cast::<u16>())
                .read()) as i32),
            )) as i16),
        );
        (((&raw mut affineSrc).cast::<u8>())
            .wrapping_add(12)
            .cast::<i16>())
        .write(
            ((((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(282)
                .cast::<u16>())
            .read()) as i16),
        );
        (((&raw mut affineSrc).cast::<u8>())
            .wrapping_add(14)
            .cast::<i16>())
        .write(
            ((((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(282)
                .cast::<u16>())
            .read()) as i16),
        );
        (((&raw mut affineSrc).cast::<u8>())
            .wrapping_add(16)
            .cast::<u16>())
        .write(
            ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(74)
                .cast::<u16>())
            .read(),
        );
        dest.cast::<crate::c::Rec4<20>>().write_unaligned(
            (&raw mut affineSrc)
                .cast::<u8>()
                .cast::<crate::c::Rec4<20>>()
                .read_unaligned(),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetBlenderArrowPosition() -> u16 {
    unsafe {
        return ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(74)
            .cast::<u16>())
        .read();
    }
}
pub(crate) unsafe extern "C" fn UpdateBlenderCenter() {
    unsafe {
        let mut playerId: u8 = 0u8;
        if (((&raw mut gReceivedRemoteLinkPlayers).cast::<u8>()).read()) != 0 {
            playerId = GetMultiplayerId();
        }
        if ((((&raw mut gWirelessCommType).cast::<u8>()).read()) != 0)
            && ((((&raw mut gReceivedRemoteLinkPlayers).cast::<u8>()).read()) != 0)
        {
            if ((playerId) as i32) == 0i32 {
                let __p1 = (((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(74)
                    .cast::<u16>();
                (__p1).write(
                    (((((__p1).read()) as i32).wrapping_add(
                        ((((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(76)
                            .cast::<i16>())
                        .read()) as i32),
                    )) as u16),
                );
                ((((&raw mut gSendCmd).cast::<u16>()).cast::<u16>()).wrapping_offset(5)).write(
                    ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(278)
                        .cast::<u16>())
                    .read(),
                );
                ((((&raw mut gSendCmd).cast::<u16>()).cast::<u16>()).wrapping_offset(6)).write(
                    ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(74)
                        .cast::<u16>())
                    .read(),
                );
                DrawBlenderCenter(
                    (((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(320),
                );
            } else {
                if ((((((&raw mut gRecvCmds).cast::<u8>()).cast::<u16>()).read()) as i32)
                    & 65280i32)
                    == 17408i32
                {
                    ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(278)
                        .cast::<u16>())
                    .write(
                        ((((&raw mut gRecvCmds).cast::<u8>()).cast::<u16>()).wrapping_offset(5))
                            .read(),
                    );
                    ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(74)
                        .cast::<u16>())
                    .write(
                        ((((&raw mut gRecvCmds).cast::<u8>()).cast::<u16>()).wrapping_offset(6))
                            .read(),
                    );
                    DrawBlenderCenter(
                        (((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(320),
                    );
                }
            }
        } else {
            let __p2 = (((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(74)
                .cast::<u16>();
            (__p2).write(
                (((((__p2).read()) as i32).wrapping_add(
                    ((((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(76)
                        .cast::<i16>())
                    .read()) as i32),
                )) as u16),
            );
            DrawBlenderCenter(
                (((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(320),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn SetBgPos() {
    unsafe {
        SetGpuReg(
            20u8,
            ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(284)
                .cast::<u16>())
            .read(),
        );
        SetGpuReg(
            22u8,
            ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(286)
                .cast::<u16>())
            .read(),
        );
        SetGpuReg(
            16u8,
            ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(284)
                .cast::<u16>())
            .read(),
        );
        SetGpuReg(
            18u8,
            ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(286)
                .cast::<u16>())
            .read(),
        );
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_Particle(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
        (__p1).write(
            (((((__p1).read()) as i32)
                .wrapping_add((((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32)))
                as i16),
        );
        let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3);
        (__p2).write(
            (((((__p2).read()) as i32).wrapping_add(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32),
            )) as i16),
        );
        ((sprite).wrapping_add(36).cast::<i16>()).write(
            ((crate::c::div_i32(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32),
                8i32,
            )) as i16),
        );
        ((sprite).wrapping_add(38).cast::<i16>()).write(
            ((crate::c::div_i32(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read()) as i32),
                8i32,
            )) as i16),
        );
        if (crate::c::bf_read((sprite).wrapping_add(63), 4, 1, false) as u16) != 0 {
            DestroySprite(sprite);
        }
    }
}
pub(crate) unsafe extern "C" fn CreateParticleSprites() {
    unsafe {
        let mut limit: i32 = (crate::c::rem_i32(((Random()) as i32), 2i32)).wrapping_add(1i32);
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < limit) {
                    break 'l1;
                }
                'l2: {
                    let mut rand: u16 = 0u16;
                    let mut x: i32 = 0i32;
                    let mut y: i32 = 0i32;
                    let mut spriteId: u8 = 0u8;
                    rand = ((((((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(74)
                        .cast::<u16>())
                    .read()) as i32)
                        .wrapping_add(crate::c::rem_i32(((Random()) as i32), 20i32)))
                        as u16);
                    x = crate::c::div_i32(
                        ((((((&raw mut gSineTable).cast::<i16>()).cast::<i16>()).wrapping_offset(
                            ((((rand) as i32) & 255i32).wrapping_add(64i32)) as isize,
                        ))
                        .read()) as i32),
                        4i32,
                    );
                    y = crate::c::div_i32(
                        ((((((&raw mut gSineTable).cast::<i16>()).cast::<i16>())
                            .wrapping_offset((((rand) as i32) & 255i32) as isize))
                        .read()) as i32),
                        4i32,
                    );
                    spriteId = CreateSprite(
                        (&raw const sSpriteTemplate_Particles)
                            .cast::<u8>()
                            .cast_mut(),
                        (((x).wrapping_add(120i32)) as i16),
                        (((y).wrapping_add(80i32)) as i16),
                        1u8,
                    );
                    (((((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68))
                    .wrapping_add(46))
                    .cast::<i16>())
                    .write(
                        (((16i32).wrapping_sub(crate::c::rem_i32(((Random()) as i32), 32i32)))
                            as i16),
                    );
                    ((((((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68))
                    .wrapping_add(46))
                    .cast::<i16>())
                    .wrapping_offset(1))
                    .write(
                        (((16i32).wrapping_sub(crate::c::rem_i32(((Random()) as i32), 32i32)))
                            as i16),
                    );
                    ((((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68))
                    .wrapping_add(28)
                    .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                    .write(Some(SpriteCB_Particle));
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_ScoreSymbol(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let __p1 = ((sprite).wrapping_add(46)).cast::<i16>();
        (__p1).write(((__p1).read()).wrapping_add(1));
        ((sprite).wrapping_add(38).cast::<i16>()).write(
            (((crate::c::div_i32(
                (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32),
                3i32,
            ))
            .wrapping_neg()) as i16),
        );
        if (crate::c::bf_read((sprite).wrapping_add(63), 4, 1, false) as u16) != 0 {
            DestroySprite(sprite);
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_ScoreSymbolBest(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let __p1 = ((sprite).wrapping_add(46)).cast::<i16>();
        (__p1).write(((__p1).read()).wrapping_add(1));
        ((sprite).wrapping_add(38).cast::<i16>()).write(
            ((((((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32).wrapping_mul(2i32))
                .wrapping_neg()) as i16),
        );
        if ((((sprite).wrapping_add(38).cast::<i16>()).read()) as i32) < (-12i32) {
            ((sprite).wrapping_add(38).cast::<i16>()).write((-12i16));
        }
        if (crate::c::bf_read((sprite).wrapping_add(63), 4, 1, false) as u16) != 0 {
            DestroySprite(sprite);
        }
    }
}
pub(crate) unsafe extern "C" fn SetPlayerBerryData(playerId: u8, itemId: u16) {
    unsafe {
        let mut playerId = playerId;
        let mut itemId = itemId;
        ((((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(116))
            .cast::<u16>())
        .wrapping_offset(((playerId) as i32) as isize))
        .write(itemId);
        ConvertItemToBlenderBerry(
            (((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(344))
            .cast::<u8>())
            .wrapping_offset(((playerId) as i32) as isize * 16),
            itemId,
        );
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_CountdownNumber(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        'l1: {
            let __sw1 = (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32);
            if __sw1 == 0i32 {
                let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
                (__p2).write((((((__p2).read()) as i32).wrapping_add(8i32)) as i16));
                if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                    as i32)
                    > (crate::c::div_i32(160i32, 2i32)).wrapping_add(8i32)
                {
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1))
                        .write((((crate::c::div_i32(160i32, 2i32)).wrapping_add(8i32)) as i16));
                    let __p3 = ((sprite).wrapping_add(46)).cast::<i16>();
                    (__p3).write(((__p3).read()).wrapping_add(1));
                    PlaySE(56u16);
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                if (({
                    let __p4 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
                    let __t5 = ((__p4).read()).wrapping_add(1);
                    (__p4).write(__t5);
                    __t5
                }) as i32)
                    > 20i32
                {
                    let __p6 = ((sprite).wrapping_add(46)).cast::<i16>();
                    (__p6).write(((__p6).read()).wrapping_add(1));
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(0i16);
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                let __p7 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
                (__p7).write((((((__p7).read()) as i32).wrapping_add(4i32)) as i16));
                if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                    as i32)
                    > 176i32
                {
                    if (({
                        let __p8 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3);
                        let __t9 = ((__p8).read()).wrapping_add(1);
                        (__p8).write(__t9);
                        __t9
                    }) as i32)
                        == 3i32
                    {
                        DestroySprite(sprite);
                        CreateSprite(
                            (&raw const sSpriteTemplate_Start).cast::<u8>().cast_mut(),
                            120i16,
                            (-20i16),
                            2u8,
                        );
                    } else {
                        (((sprite).wrapping_add(46)).cast::<i16>()).write(0i16);
                        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1))
                            .write((-16i16));
                        StartSpriteAnim(
                            sprite,
                            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3))
                                .read()) as u8),
                        );
                    }
                }
                break 'l1;
            }
        }
        ((sprite).wrapping_add(38).cast::<i16>())
            .write(((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read());
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_Start(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        'l1: {
            let __sw1 = (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32);
            if __sw1 == 0i32 {
                let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
                (__p2).write((((((__p2).read()) as i32).wrapping_add(8i32)) as i16));
                if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                    as i32)
                    > 92i32
                {
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(92i16);
                    let __p3 = ((sprite).wrapping_add(46)).cast::<i16>();
                    (__p3).write(((__p3).read()).wrapping_add(1));
                    PlaySE(21u16);
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                let __p4 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
                (__p4).write((((((__p4).read()) as i32).wrapping_add(1i32)) as i16));
                if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                    as i32)
                    > 20i32
                {
                    let __p5 = ((sprite).wrapping_add(46)).cast::<i16>();
                    (__p5).write(((__p5).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                let __p6 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
                (__p6).write((((((__p6).read()) as i32).wrapping_add(4i32)) as i16));
                if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                    as i32)
                    > 176i32
                {
                    let __p7 = (((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read());
                    (__p7).write(((__p7).read()).wrapping_add(1));
                    DestroySprite(sprite);
                }
                break 'l1;
            }
        }
        ((sprite).wrapping_add(38).cast::<i16>())
            .write(((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read());
    }
}
pub(crate) unsafe extern "C" fn TryUpdateProgressBar(current: u16, limit: u16) {
    unsafe {
        let mut current = current;
        let mut limit = limit;
        if ((((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(280)
            .cast::<u16>())
        .read()) as i32)
            < ((current) as i32)
        {
            let __p1 = (((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(280)
                .cast::<u16>();
            (__p1).write((((((__p1).read()) as i32).wrapping_add(2i32)) as u16));
            UpdateProgressBar(
                ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(280)
                    .cast::<u16>())
                .read(),
                limit,
            );
        }
    }
}
pub(crate) unsafe extern "C" fn UpdateProgressBar(value: u16, limit: u16) {
    unsafe {
        let mut value = value;
        let mut limit = limit;
        let mut amountFilled: i32 = 0i32;
        let mut maxFilledSegment: i32 = 0i32;
        let mut subSegmentsFilled: i32 = 0i32;
        let mut i: i32 = 0i32;
        let mut vram: *mut u16 = core::ptr::null_mut();
        vram = ((100687872i32) as usize as *mut u16);
        amountFilled = crate::c::div_i32(((value) as i32).wrapping_mul(64i32), ((limit) as i32));
        maxFilledSegment = crate::c::div_i32(amountFilled, 8i32);
        {
            i = 0i32;
            'l1: loop {
                if !(i < maxFilledSegment) {
                    break 'l1;
                }
                'l2: {
                    ((vram).wrapping_offset(((11i32).wrapping_add(i)) as isize)).write(33001u16);
                    ((vram).wrapping_offset(((43i32).wrapping_add(i)) as isize)).write(33017u16);
                }
                i = (i).wrapping_add(1);
            }
        }
        subSegmentsFilled = crate::c::rem_i32(amountFilled, 8i32);
        if subSegmentsFilled != 0i32 {
            ((vram).wrapping_offset(((11i32).wrapping_add(i)) as isize))
                .write((((subSegmentsFilled).wrapping_add(32993i32)) as u16));
            ((vram).wrapping_offset(((43i32).wrapping_add(i)) as isize))
                .write((((subSegmentsFilled).wrapping_add(33009i32)) as u16));
            i = (i).wrapping_add(1);
        }
        {
            'l3: loop {
                if !(i < 8i32) {
                    break 'l3;
                }
                'l4: {
                    ((vram).wrapping_offset(((11i32).wrapping_add(i)) as isize)).write(32993u16);
                    ((vram).wrapping_offset(((43i32).wrapping_add(i)) as isize)).write(33009u16);
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn ArrowSpeedToRPM(speed: u16) -> u32 {
    unsafe {
        let mut speed = speed;
        return ((crate::c::div_i32((360000i32).wrapping_mul(((speed) as i32)), 65536i32)) as u32);
    }
}
pub(crate) unsafe extern "C" fn UpdateRPM(speed: u16) {
    unsafe {
        let mut speed = speed;
        let mut i: u8 = 0u8;
        let mut digits = crate::ffi::Align4([0u8; 5]);
        let mut currentRPM: u32 = ArrowSpeedToRPM(speed);
        if ((((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(78)
            .cast::<u16>())
        .read()) as u32)
            < currentRPM
        {
            ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(78)
                .cast::<u16>())
            .write(((currentRPM) as u16));
        }
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 5i32) {
                    break 'l1;
                }
                'l2: {
                    (((&raw mut digits).cast::<u8>()).wrapping_offset(((i) as i32) as isize))
                        .write(((crate::c::rem_u32(currentRPM, 10u32)) as u8));
                    currentRPM = crate::c::div_u32(currentRPM, 10u32);
                }
                i = (i).wrapping_add(1);
            }
        }
        ((100688984i32) as usize as *mut u16).write(
            (((((((&raw mut digits).cast::<u8>()).wrapping_offset(4)).read()) as i32)
                .wrapping_add(32882i32)) as u16),
        );
        ((100688986i32) as usize as *mut u16).write(
            (((((((&raw mut digits).cast::<u8>()).wrapping_offset(3)).read()) as i32)
                .wrapping_add(32882i32)) as u16),
        );
        ((100688988i32) as usize as *mut u16).write(
            (((((((&raw mut digits).cast::<u8>()).wrapping_offset(2)).read()) as i32)
                .wrapping_add(32882i32)) as u16),
        );
        ((100688992i32) as usize as *mut u16).write(
            (((((((&raw mut digits).cast::<u8>()).wrapping_offset(1)).read()) as i32)
                .wrapping_add(32882i32)) as u16),
        );
        ((100688994i32) as usize as *mut u16).write(
            ((((((&raw mut digits).cast::<u8>()).read()) as i32).wrapping_add(32882i32)) as u16),
        );
    }
}
pub(crate) unsafe extern "C" fn ShakeBgCoordForHit(coord: *mut i16, speed: u16) {
    unsafe {
        let mut coord = coord;
        let mut speed = speed;
        if (((coord).read()) as i32) == 0i32 {
            (coord).write(
                (((crate::c::rem_i32(((Random()) as i32), ((speed) as i32)))
                    .wrapping_sub(crate::c::div_i32(((speed) as i32), 2i32)))
                    as i16),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn RestoreBgCoord(coord: *mut i16) {
    unsafe {
        let mut coord = coord;
        if (((coord).read()) as i32) < 0i32 {
            (coord).write(((coord).read()).wrapping_add(1));
        }
        if (((coord).read()) as i32) > 0i32 {
            (coord).write(((coord).read()).wrapping_sub(1));
        }
    }
}
pub(crate) unsafe extern "C" fn RestoreBgCoords() {
    unsafe {
        RestoreBgCoord(
            ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(284)
                .cast::<u16>())
            .cast::<i16>(),
        );
        RestoreBgCoord(
            ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(286)
                .cast::<u16>())
            .cast::<i16>(),
        );
    }
}
pub(crate) unsafe extern "C" fn BlenderLandShakeBgCoord(coord: *mut i16, timer: u16) {
    unsafe {
        let mut coord = coord;
        let mut timer = timer;
        let mut strength: i32 = 0i32;
        if ((timer) as i32) < 10i32 {
            strength = 16i32;
        } else {
            strength = 8i32;
        }
        if (((coord).read()) as i32) == 0i32 {
            (coord).write(
                (((crate::c::rem_i32(((Random()) as i32), strength))
                    .wrapping_sub(crate::c::div_i32(strength, 2i32))) as i16),
            );
        } else {
            if (((coord).read()) as i32) < 0i32 {
                (coord).write(((coord).read()).wrapping_add(1));
            }
            if (((coord).read()) as i32) > 0i32 {
                (coord).write(((coord).read()).wrapping_sub(1));
            }
        }
    }
}
pub(crate) unsafe extern "C" fn UpdateBlenderLandScreenShake() -> u8 {
    unsafe {
        if ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(264)
            .cast::<i32>())
        .read()
            == 0i32
        {
            ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(284)
                .cast::<u16>())
            .write(0u16);
            ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(286)
                .cast::<u16>())
            .write(0u16);
        }
        let __p1 = (((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(264)
            .cast::<i32>();
        (__p1).write(((__p1).read()).wrapping_add(1));
        BlenderLandShakeBgCoord(
            ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(284)
                .cast::<u16>())
            .cast::<i16>(),
            ((((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(264)
                .cast::<i32>())
            .read()) as u16),
        );
        BlenderLandShakeBgCoord(
            ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(286)
                .cast::<u16>())
            .cast::<i16>(),
            ((((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(264)
                .cast::<i32>())
            .read()) as u16),
        );
        if ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(264)
            .cast::<i32>())
        .read()
            == 20i32
        {
            ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(284)
                .cast::<u16>())
            .write(0u16);
            ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(286)
                .cast::<u16>())
            .write(0u16);
            return 1u8;
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_PlayerArrow(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        ((sprite).wrapping_add(36).cast::<i16>()).write(
            ((((((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(284)
                .cast::<u16>())
            .read()) as i32)
                .wrapping_neg()) as i16),
        );
        ((sprite).wrapping_add(38).cast::<i16>()).write(
            ((((((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(286)
                .cast::<u16>())
            .read()) as i32)
                .wrapping_neg()) as i16),
        );
    }
}
pub(crate) unsafe extern "C" fn TryUpdateBerryBlenderRecord() {
    unsafe {
        if ((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(2492))
            .cast::<u16>())
        .wrapping_offset(
            (((((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(124))
            .read()) as i32)
                .wrapping_sub(2i32)) as isize,
        ))
        .read()) as i32)
            < ((((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(78)
                .cast::<u16>())
            .read()) as i32)
        {
            ((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(2492))
                .cast::<u16>())
            .wrapping_offset(
                (((((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(124))
                .read()) as i32)
                    .wrapping_sub(2i32)) as isize,
            ))
            .write(
                ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(78)
                    .cast::<u16>())
                .read(),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn PrintBlendingResults() -> u8 {
    unsafe {
        let mut i: u16 = 0u16;
        let mut xPos: i32 = 0i32;
        let mut yPos: i32 = 0i32;
        let mut pokeblock = crate::ffi::Align4([0u8; 8]);
        let mut flavors = crate::ffi::Align4([0u8; 6]);
        let mut text = crate::ffi::Align4([0u8; 40]);
        let mut berryIds = crate::ffi::Align4([0u8; 8]);
        'l1: {
            let __sw1 = (((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read()).read())
                as i32);
            if __sw1 == 0i32 {
                let __p2 = (((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read());
                (__p2).write(((__p2).read()).wrapping_add(1));
                ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(264)
                    .cast::<i32>())
                .write(17i32);
                break 'l1;
            }
            if __sw1 == 1i32 {
                let __p3 = (((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(264)
                    .cast::<i32>();
                (__p3).write(((__p3).read()).wrapping_sub(10i32));
                if ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(264)
                    .cast::<i32>())
                .read()
                    < 0i32
                {
                    ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(264)
                        .cast::<i32>())
                    .write(0i32);
                    let __p4 = (((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read());
                    (__p4).write(((__p4).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                if {
                    let __p5 = (((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(264)
                        .cast::<i32>();
                    let __t6 = ((__p5).read()).wrapping_add(1);
                    (__p5).write(__t6);
                    __t6
                } > 20i32
                {
                    {
                        i = 0u16;
                        'l2: loop {
                            if !(((i) as i32) < 3i32) {
                                break 'l2;
                            }
                            'l3: {
                                DestroySprite(
                                    ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                        ((((((((&raw mut sBerryBlender)
                                            .cast::<u8>()
                                            .cast::<*mut u8>())
                                        .read())
                                        .wrapping_add(70))
                                        .cast::<u8>())
                                        .wrapping_offset(((i) as i32) as isize))
                                        .read()) as i32)
                                            as isize
                                            * 68,
                                    ),
                                );
                            }
                            i = (i).wrapping_add(1);
                        }
                    }
                    ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(264)
                        .cast::<i32>())
                    .write(0i32);
                    let __p7 = (((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read());
                    (__p7).write(((__p7).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                {
                    let mut minutes: u16 = 0u16;
                    let mut seconds: u16 = 0u16;
                    let mut txtPtr: *mut u8 = core::ptr::null_mut();
                    xPos = GetStringCenterAlignXOffset(
                        1i32,
                        ((&raw const sText_BlendingResults).cast::<u8>().cast_mut()).cast::<u8>(),
                        168i32,
                    );
                    Blender_AddTextPrinter(
                        5u8,
                        ((&raw const sText_BlendingResults).cast::<u8>().cast_mut()).cast::<u8>(),
                        ((xPos) as u8),
                        1u8,
                        255i32,
                        0i32,
                    );
                    if ((((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(124))
                    .read()) as i32)
                        == 4i32
                    {
                        yPos = 17i32;
                    } else {
                        yPos = 21i32;
                    }
                    {
                        i = 0u16;
                        'l4: loop {
                            if !(((i) as i32)
                                < ((((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(124))
                                .read()) as i32))
                            {
                                break 'l4;
                            }
                            'l5: {
                                let mut place: u8 = ((((((&raw mut sBerryBlender)
                                    .cast::<u8>()
                                    .cast::<*mut u8>())
                                .read())
                                .wrapping_add(316))
                                .cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize))
                                .read();
                                ConvertIntToDecimalStringN(
                                    ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(159))
                                    .cast::<u8>(),
                                    ((i) as i32).wrapping_add(1i32),
                                    0i32,
                                    1u8,
                                );
                                StringAppend(
                                    ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(159))
                                    .cast::<u8>(),
                                    ((&raw const sText_Dot).cast::<u8>().cast_mut()).cast::<u8>(),
                                );
                                StringAppend(
                                    ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(159))
                                    .cast::<u8>(),
                                    (&raw mut gText_Space).cast::<u8>(),
                                );
                                StringAppend(
                                    ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(159))
                                    .cast::<u8>(),
                                    ((((&raw mut gLinkPlayers).cast::<u8>())
                                        .wrapping_offset(((place) as i32) as isize * 28))
                                    .wrapping_add(8))
                                    .cast::<u8>(),
                                );
                                Blender_AddTextPrinter(
                                    5u8,
                                    ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(159))
                                    .cast::<u8>(),
                                    8u8,
                                    ((yPos) as u8),
                                    255i32,
                                    3i32,
                                );
                                StringCopy(
                                    ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(159))
                                    .cast::<u8>(),
                                    (((((((&raw mut sBerryBlender)
                                        .cast::<u8>()
                                        .cast::<*mut u8>())
                                    .read())
                                    .wrapping_add(344))
                                    .cast::<u8>())
                                    .wrapping_offset(((place) as i32) as isize * 16))
                                    .wrapping_add(2))
                                    .cast::<u8>(),
                                );
                                ConvertInternationalString(
                                    ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(159))
                                    .cast::<u8>(),
                                    ((((((&raw mut gLinkPlayers).cast::<u8>())
                                        .wrapping_offset(((place) as i32) as isize * 28))
                                    .wrapping_add(26)
                                    .cast::<u16>())
                                    .read()) as u8),
                                );
                                StringAppend(
                                    ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(159))
                                    .cast::<u8>(),
                                    ((&raw const sText_SpaceBerry).cast::<u8>().cast_mut())
                                        .cast::<u8>(),
                                );
                                Blender_AddTextPrinter(
                                    5u8,
                                    ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(159))
                                    .cast::<u8>(),
                                    84u8,
                                    ((yPos) as u8),
                                    255i32,
                                    3i32,
                                );
                            }
                            yPos = (yPos).wrapping_add(16i32);
                            i = (i).wrapping_add(1);
                        }
                    }
                    Blender_AddTextPrinter(
                        5u8,
                        ((&raw const sText_MaximumSpeed).cast::<u8>().cast_mut()).cast::<u8>(),
                        0u8,
                        81u8,
                        255i32,
                        3i32,
                    );
                    ConvertIntToDecimalStringN(
                        ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(159))
                        .cast::<u8>(),
                        crate::c::div_i32(
                            ((((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(78)
                                .cast::<u16>())
                            .read()) as i32),
                            100i32,
                        ),
                        1i32,
                        3u8,
                    );
                    StringAppend(
                        ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(159))
                        .cast::<u8>(),
                        ((&raw const sText_Dot).cast::<u8>().cast_mut()).cast::<u8>(),
                    );
                    ConvertIntToDecimalStringN(
                        (&raw mut text).cast::<u8>(),
                        crate::c::rem_i32(
                            ((((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(78)
                                .cast::<u16>())
                            .read()) as i32),
                            100i32,
                        ),
                        2i32,
                        2u8,
                    );
                    StringAppend(
                        ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(159))
                        .cast::<u8>(),
                        (&raw mut text).cast::<u8>(),
                    );
                    StringAppend(
                        ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(159))
                        .cast::<u8>(),
                        ((&raw const sText_RPM).cast::<u8>().cast_mut()).cast::<u8>(),
                    );
                    xPos = GetStringRightAlignXOffset(
                        1i32,
                        ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(159))
                        .cast::<u8>(),
                        168i32,
                    );
                    Blender_AddTextPrinter(
                        5u8,
                        ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(159))
                        .cast::<u8>(),
                        ((xPos) as u8),
                        81u8,
                        255i32,
                        3i32,
                    );
                    Blender_AddTextPrinter(
                        5u8,
                        ((&raw const sText_Time).cast::<u8>().cast_mut()).cast::<u8>(),
                        0u8,
                        97u8,
                        255i32,
                        3i32,
                    );
                    seconds = ((crate::c::rem_u32(
                        crate::c::div_u32(
                            ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(260)
                                .cast::<u32>())
                            .read(),
                            60u32,
                        ),
                        60u32,
                    )) as u16);
                    minutes = ((crate::c::div_u32(
                        ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(260)
                            .cast::<u32>())
                        .read(),
                        3600u32,
                    )) as u16);
                    ConvertIntToDecimalStringN(
                        ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(159))
                        .cast::<u8>(),
                        ((minutes) as i32),
                        2i32,
                        2u8,
                    );
                    txtPtr = StringAppend(
                        ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(159))
                        .cast::<u8>(),
                        ((&raw const sText_Min).cast::<u8>().cast_mut()).cast::<u8>(),
                    );
                    ConvertIntToDecimalStringN(txtPtr, ((seconds) as i32), 2i32, 2u8);
                    StringAppend(
                        ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(159))
                        .cast::<u8>(),
                        ((&raw const sText_Sec).cast::<u8>().cast_mut()).cast::<u8>(),
                    );
                    xPos = GetStringRightAlignXOffset(
                        1i32,
                        ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(159))
                        .cast::<u8>(),
                        168i32,
                    );
                    Blender_AddTextPrinter(
                        5u8,
                        ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(159))
                        .cast::<u8>(),
                        ((xPos) as u8),
                        97u8,
                        255i32,
                        3i32,
                    );
                    ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(264)
                        .cast::<i32>())
                    .write(0i32);
                    let __p8 = (((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read());
                    (__p8).write(((__p8).read()).wrapping_add(1));
                    CopyWindowToVram(5u8, 2u8);
                }
                break 'l1;
            }
            if __sw1 == 4i32 {
                if ((((((&raw mut gMain).cast::<u8>())
                    .wrapping_add(46)
                    .cast::<u16>())
                .read()) as i32)
                    & 1i32)
                    != 0
                {
                    let __p9 = (((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read());
                    (__p9).write(((__p9).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 5i32 {
                ClearStdWindowAndFrameToTransparent(5u8, 1u8);
                {
                    i = 0u16;
                    'l6: loop {
                        if !(((i) as i32) < 4i32) {
                            break 'l6;
                        }
                        'l7: {
                            if ((((((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(116))
                            .cast::<u16>())
                            .wrapping_offset(((i) as i32) as isize))
                            .read()) as i32)
                                != 0i32
                            {
                                (((&raw mut berryIds).cast::<u16>())
                                    .wrapping_offset(((i) as i32) as isize))
                                .write(
                                    ((((((((((&raw mut sBerryBlender)
                                        .cast::<u8>()
                                        .cast::<*mut u8>())
                                    .read())
                                    .wrapping_add(116))
                                    .cast::<u16>())
                                    .wrapping_offset(((i) as i32) as isize))
                                    .read()) as i32)
                                        .wrapping_sub(133i32))
                                        as u16),
                                );
                            }
                            if ((((((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(142))
                            .cast::<u16>())
                            .wrapping_offset(((i) as i32) as isize))
                            .read()) as i32)
                                != 255i32
                            {
                                PutWindowTilemap(((i) as u8));
                                CopyWindowToVram(((i) as u8), 3u8);
                            }
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                Debug_SetStageVars();
                CalculatePokeblock(
                    ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(344))
                    .cast::<u8>(),
                    (&raw mut pokeblock).cast::<u8>(),
                    ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(124))
                    .read(),
                    (&raw mut flavors).cast::<u8>(),
                    ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(78)
                        .cast::<u16>())
                    .read(),
                );
                PrintMadePokeblockString(
                    (&raw mut pokeblock).cast::<u8>(),
                    ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(159))
                    .cast::<u8>(),
                );
                TryAddContestLinkTvShow(
                    (&raw mut pokeblock).cast::<u8>(),
                    (((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(424),
                );
                CreateTask(Some(Task_PlayPokeblockFanfare), 6u8);
                IncrementDailyBerryBlender();
                RemoveBagItem(((&raw mut gSpecialVar_ItemId).cast::<u16>()).read(), 1u16);
                AddPokeblock((&raw mut pokeblock).cast::<u8>());
                ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(4536)
                    .cast::<i16>())
                .write(0i16);
                let __p10 = (((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read());
                (__p10).write(((__p10).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 6i32 {
                if (PrintMessage(
                    (((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(4536)
                        .cast::<i16>(),
                    ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(159))
                    .cast::<u8>(),
                    ((GetPlayerTextSpeedDelay()) as i32),
                )) != 0
                {
                    TryUpdateBerryBlenderRecord();
                    return 1u8;
                }
                break 'l1;
            }
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn PrintMadePokeblockString(pokeblock: *mut u8, dst: *mut u8) {
    unsafe {
        let mut pokeblock = pokeblock;
        let mut dst = dst;
        let mut text = crate::ffi::Align4([0u8; 12]);
        let mut flavorLvl: u8 = 0u8;
        let mut feel: u8 = 0u8;
        (dst).write(255u8);
        StringCopy(
            dst,
            ((((&raw mut gPokeblockNames).cast::<*mut u8>()).cast::<*mut u8>())
                .wrapping_offset((((pokeblock).read()) as i32) as isize))
            .read(),
        );
        StringAppend(
            dst,
            ((&raw const sText_WasMade).cast::<u8>().cast_mut()).cast::<u8>(),
        );
        StringAppend(
            dst,
            ((&raw const sText_NewLine).cast::<u8>().cast_mut()).cast::<u8>(),
        );
        flavorLvl = GetHighestPokeblocksFlavorLevel(pokeblock);
        feel = GetPokeblocksFeel(pokeblock);
        StringAppend(
            dst,
            ((&raw const sText_TheLevelIs).cast::<u8>().cast_mut()).cast::<u8>(),
        );
        ConvertIntToDecimalStringN(
            (&raw mut text).cast::<u8>(),
            ((flavorLvl) as i32),
            0i32,
            3u8,
        );
        StringAppend(dst, (&raw mut text).cast::<u8>());
        StringAppend(
            dst,
            ((&raw const sText_TheFeelIs).cast::<u8>().cast_mut()).cast::<u8>(),
        );
        ConvertIntToDecimalStringN((&raw mut text).cast::<u8>(), ((feel) as i32), 0i32, 3u8);
        StringAppend(dst, (&raw mut text).cast::<u8>());
        StringAppend(
            dst,
            ((&raw const sText_Dot2).cast::<u8>().cast_mut()).cast::<u8>(),
        );
        StringAppend(
            dst,
            ((&raw const sText_NewParagraph).cast::<u8>().cast_mut()).cast::<u8>(),
        );
    }
}
pub(crate) unsafe extern "C" fn SortBasedOnPoints(
    places: *mut u8,
    playersNum: u8,
    scores: *mut u32,
) {
    unsafe {
        let mut places = places;
        let mut playersNum = playersNum;
        let mut scores = scores;
        let mut i: i32 = 0i32;
        let mut j: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < ((playersNum) as i32)) {
                    break 'l1;
                }
                'l2: {
                    {
                        j = 0i32;
                        'l3: loop {
                            if !(j < ((playersNum) as i32)) {
                                break 'l3;
                            }
                            'l4: {
                                if ((scores).wrapping_offset(
                                    ((((places).wrapping_offset((i) as isize)).read()) as i32)
                                        as isize,
                                ))
                                .read()
                                    > ((scores).wrapping_offset(
                                        ((((places).wrapping_offset((j) as isize)).read()) as i32)
                                            as isize,
                                    ))
                                    .read()
                                {
                                    let mut temp: u8 = 0u8;
                                    {
                                        temp = ((places).wrapping_offset((i) as isize)).read();
                                        ((places).wrapping_offset((i) as isize))
                                            .write(((places).wrapping_offset((j) as isize)).read());
                                        ((places).wrapping_offset((j) as isize)).write(temp);
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
pub(crate) unsafe extern "C" fn SortScores() {
    unsafe {
        let mut playerId: u8 = 0u8;
        let mut i: u8 = 0u8;
        let mut places = crate::ffi::Align4([0u8; 4]);
        let mut points = crate::ffi::Align4([0u8; 16]);
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32)
                    < ((((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(124))
                    .read()) as i32))
                {
                    break 'l1;
                }
                'l2: {
                    (((&raw mut places).cast::<u8>()).wrapping_offset(((i) as i32) as isize))
                        .write(i);
                }
                i = (i).wrapping_add(1);
            }
        }
        {
            i = 0u8;
            'l3: loop {
                if !(((i) as i32)
                    < ((((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(124))
                    .read()) as i32))
                {
                    break 'l3;
                }
                'l4: {
                    (((&raw mut points).cast::<u32>()).wrapping_offset(((i) as i32) as isize))
                        .write(
                            (((1000000i32).wrapping_mul(
                                (((((((((&raw mut sBerryBlender)
                                    .cast::<u8>()
                                    .cast::<*mut u8>())
                                .read())
                                .wrapping_add(292))
                                .cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize * 6))
                                .cast::<u16>())
                                .read()) as i32),
                            )) as u32),
                        );
                    let __p1 =
                        ((&raw mut points).cast::<u32>()).wrapping_offset(((i) as i32) as isize);
                    (__p1).write(
                        ((__p1).read()).wrapping_add(
                            (((1000i32).wrapping_mul(
                                ((((((((((&raw mut sBerryBlender)
                                    .cast::<u8>()
                                    .cast::<*mut u8>())
                                .read())
                                .wrapping_add(292))
                                .cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize * 6))
                                .cast::<u16>())
                                .wrapping_offset(1))
                                .read()) as i32),
                            )) as u32),
                        ),
                    );
                    let __p2 =
                        ((&raw mut points).cast::<u32>()).wrapping_offset(((i) as i32) as isize);
                    (__p2).write(
                        ((__p2).read()).wrapping_add(
                            (((1000i32).wrapping_sub(
                                ((((((((((&raw mut sBerryBlender)
                                    .cast::<u8>()
                                    .cast::<*mut u8>())
                                .read())
                                .wrapping_add(292))
                                .cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize * 6))
                                .cast::<u16>())
                                .wrapping_offset(2))
                                .read()) as i32),
                            )) as u32),
                        ),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        SortBasedOnPoints(
            (&raw mut places).cast::<u8>(),
            ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(124))
                .read(),
            (&raw mut points).cast::<u32>(),
        );
        {
            i = 0u8;
            'l5: loop {
                if !(((i) as i32)
                    < ((((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(124))
                    .read()) as i32))
                {
                    break 'l5;
                }
                'l6: {
                    ((((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(316))
                    .cast::<u8>())
                    .wrapping_offset(((i) as i32) as isize))
                    .write(
                        (((&raw mut places).cast::<u8>()).wrapping_offset(((i) as i32) as isize))
                            .read(),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        if !((((&raw mut gReceivedRemoteLinkPlayers).cast::<u8>()).read()) != 0) {
            playerId = 0u8;
        } else {
            playerId = GetMultiplayerId();
        }
        {
            i = 0u8;
            'l7: loop {
                if !(((i) as i32)
                    < ((((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(124))
                    .read()) as i32))
                {
                    break 'l7;
                }
                'l8: {
                    if ((((((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(316))
                    .cast::<u8>())
                    .wrapping_offset(((i) as i32) as isize))
                    .read()) as i32)
                        == ((playerId) as i32)
                    {
                        ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(420))
                        .write(i);
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn PrintBlendingRanking() -> u8 {
    unsafe {
        let mut i: u16 = 0u16;
        let mut xPos: i32 = 0i32;
        let mut yPos: i32 = 0i32;
        'l1: {
            let __sw1 = (((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read()).read())
                as i32);
            if __sw1 == 0i32 {
                let __p2 = (((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read());
                (__p2).write(((__p2).read()).wrapping_add(1));
                ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(264)
                    .cast::<i32>())
                .write(255i32);
                break 'l1;
            }
            if __sw1 == 1i32 {
                let __p3 = (((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(264)
                    .cast::<i32>();
                (__p3).write(((__p3).read()).wrapping_sub(10i32));
                if ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(264)
                    .cast::<i32>())
                .read()
                    < 0i32
                {
                    ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(264)
                        .cast::<i32>())
                    .write(0i32);
                    let __p4 = (((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read());
                    (__p4).write(((__p4).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                if {
                    let __p5 = (((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(264)
                        .cast::<i32>();
                    let __t6 = ((__p5).read()).wrapping_add(1);
                    (__p5).write(__t6);
                    __t6
                } > 20i32
                {
                    ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(264)
                        .cast::<i32>())
                    .write(0i32);
                    let __p7 = (((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read());
                    (__p7).write(((__p7).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                DrawStdFrameWithCustomTileAndPalette(5u8, 0u8, 1u16, 13u8);
                xPos = GetStringCenterAlignXOffset(
                    1i32,
                    ((&raw const sText_Ranking).cast::<u8>().cast_mut()).cast::<u8>(),
                    168i32,
                );
                Blender_AddTextPrinter(
                    5u8,
                    ((&raw const sText_Ranking).cast::<u8>().cast_mut()).cast::<u8>(),
                    ((xPos) as u8),
                    1u8,
                    255i32,
                    0i32,
                );
                (((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(70))
                .cast::<u8>())
                .write(CreateSprite(
                    (&raw const sSpriteTemplate_ScoreSymbols)
                        .cast::<u8>()
                        .cast_mut(),
                    128i16,
                    52i16,
                    0u8,
                ));
                StartSpriteAnim(
                    ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        (((((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(70))
                        .cast::<u8>())
                        .read()) as i32) as isize
                            * 68,
                    ),
                    3u8,
                );
                ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    (((((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(70))
                    .cast::<u8>())
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                .write(Some(SpriteCallbackDummy));
                ((((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(70))
                .cast::<u8>())
                .wrapping_offset(1))
                .write(CreateSprite(
                    (&raw const sSpriteTemplate_ScoreSymbols)
                        .cast::<u8>()
                        .cast_mut(),
                    160i16,
                    52i16,
                    0u8,
                ));
                ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(70))
                    .cast::<u8>())
                    .wrapping_offset(1))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                .write(Some(SpriteCallbackDummy));
                ((((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(70))
                .cast::<u8>())
                .wrapping_offset(2))
                .write(CreateSprite(
                    (&raw const sSpriteTemplate_ScoreSymbols)
                        .cast::<u8>()
                        .cast_mut(),
                    192i16,
                    52i16,
                    0u8,
                ));
                StartSpriteAnim(
                    ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(70))
                        .cast::<u8>())
                        .wrapping_offset(2))
                        .read()) as i32) as isize
                            * 68,
                    ),
                    1u8,
                );
                ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(70))
                    .cast::<u8>())
                    .wrapping_offset(2))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                .write(Some(SpriteCallbackDummy));
                SortScores();
                {
                    yPos = 41i32;
                    i = 0u16;
                    'l2: loop {
                        if !(((i) as i32)
                            < ((((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(124))
                            .read()) as i32))
                        {
                            break 'l2;
                        }
                        'l3: {
                            let mut place: u8 =
                                ((((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(316))
                                .cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize))
                                .read();
                            ConvertIntToDecimalStringN(
                                ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(159))
                                .cast::<u8>(),
                                ((i) as i32).wrapping_add(1i32),
                                0i32,
                                1u8,
                            );
                            StringAppend(
                                ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(159))
                                .cast::<u8>(),
                                ((&raw const sText_Dot).cast::<u8>().cast_mut()).cast::<u8>(),
                            );
                            StringAppend(
                                ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(159))
                                .cast::<u8>(),
                                (&raw mut gText_Space).cast::<u8>(),
                            );
                            StringAppend(
                                ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(159))
                                .cast::<u8>(),
                                ((((&raw mut gLinkPlayers).cast::<u8>())
                                    .wrapping_offset(((place) as i32) as isize * 28))
                                .wrapping_add(8))
                                .cast::<u8>(),
                            );
                            Blender_AddTextPrinter(
                                5u8,
                                ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(159))
                                .cast::<u8>(),
                                0u8,
                                ((yPos) as u8),
                                255i32,
                                3i32,
                            );
                            ConvertIntToDecimalStringN(
                                ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(159))
                                .cast::<u8>(),
                                (((((((((&raw mut sBerryBlender)
                                    .cast::<u8>()
                                    .cast::<*mut u8>())
                                .read())
                                .wrapping_add(292))
                                .cast::<u8>())
                                .wrapping_offset(((place) as i32) as isize * 6))
                                .cast::<u16>())
                                .read()) as i32),
                                1i32,
                                3u8,
                            );
                            Blender_AddTextPrinter(
                                5u8,
                                ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(159))
                                .cast::<u8>(),
                                78u8,
                                ((yPos) as u8),
                                255i32,
                                3i32,
                            );
                            ConvertIntToDecimalStringN(
                                ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(159))
                                .cast::<u8>(),
                                ((((((((((&raw mut sBerryBlender)
                                    .cast::<u8>()
                                    .cast::<*mut u8>())
                                .read())
                                .wrapping_add(292))
                                .cast::<u8>())
                                .wrapping_offset(((place) as i32) as isize * 6))
                                .cast::<u16>())
                                .wrapping_offset(1))
                                .read()) as i32),
                                1i32,
                                3u8,
                            );
                            Blender_AddTextPrinter(
                                5u8,
                                ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(159))
                                .cast::<u8>(),
                                110u8,
                                ((yPos) as u8),
                                255i32,
                                3i32,
                            );
                            ConvertIntToDecimalStringN(
                                ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(159))
                                .cast::<u8>(),
                                ((((((((((&raw mut sBerryBlender)
                                    .cast::<u8>()
                                    .cast::<*mut u8>())
                                .read())
                                .wrapping_add(292))
                                .cast::<u8>())
                                .wrapping_offset(((place) as i32) as isize * 6))
                                .cast::<u16>())
                                .wrapping_offset(2))
                                .read()) as i32),
                                1i32,
                                3u8,
                            );
                            Blender_AddTextPrinter(
                                5u8,
                                ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(159))
                                .cast::<u8>(),
                                142u8,
                                ((yPos) as u8),
                                255i32,
                                3i32,
                            );
                        }
                        yPos = (yPos).wrapping_add(16i32);
                        i = (i).wrapping_add(1);
                    }
                }
                PutWindowTilemap(5u8);
                CopyWindowToVram(5u8, 3u8);
                ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(264)
                    .cast::<i32>())
                .write(0i32);
                let __p8 = (((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read());
                (__p8).write(((__p8).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 4i32 {
                if {
                    let __p9 = (((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(264)
                        .cast::<i32>();
                    let __t10 = ((__p9).read()).wrapping_add(1);
                    (__p9).write(__t10);
                    __t10
                } > 20i32
                {
                    let __p11 = (((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read());
                    (__p11).write(((__p11).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 5i32 {
                if ((((((&raw mut gMain).cast::<u8>())
                    .wrapping_add(46)
                    .cast::<u16>())
                .read()) as i32)
                    & 1i32)
                    != 0
                {
                    PlaySE(5u16);
                    let __p12 = (((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read());
                    (__p12).write(((__p12).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 6i32 {
                (((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read()).write(0u8);
                return 1u8;
            }
        }
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ShowBerryBlenderRecordWindow() {
    unsafe {
        let mut i: i32 = 0i32;
        let mut xPos: i32 = 0i32;
        let mut yPos: i32 = 0i32;
        let mut winTemplate = crate::ffi::Align4([0u8; 8]);
        let mut text = crate::ffi::Align4([0u8; 32]);
        (&raw mut winTemplate)
            .cast::<u8>()
            .cast::<crate::c::Rec4<8>>()
            .write_unaligned(
                (&raw const sBlenderRecordWindowTemplate)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<crate::c::Rec4<8>>()
                    .read_unaligned(),
            );
        ((&raw mut gRecordsWindowId).cast::<u8>())
            .write(((AddWindow((&raw mut winTemplate).cast::<u8>())) as u8));
        DrawStdWindowFrame(((&raw mut gRecordsWindowId).cast::<u8>()).read(), 0u8);
        FillWindowPixelBuffer(((&raw mut gRecordsWindowId).cast::<u8>()).read(), 17u8);
        xPos = GetStringCenterAlignXOffset(
            1i32,
            (&raw mut gText_BlenderMaxSpeedRecord).cast::<u8>(),
            144i32,
        );
        AddTextPrinterParameterized(
            ((&raw mut gRecordsWindowId).cast::<u8>()).read(),
            1u8,
            (&raw mut gText_BlenderMaxSpeedRecord).cast::<u8>(),
            ((xPos) as u8),
            1u8,
            0u8,
            None,
        );
        AddTextPrinterParameterized(
            ((&raw mut gRecordsWindowId).cast::<u8>()).read(),
            1u8,
            (&raw mut gText_234Players).cast::<u8>(),
            4u8,
            41u8,
            0u8,
            None,
        );
        {
            i = 0i32;
            yPos = 41i32;
            'l1: loop {
                if !(i < 3i32) {
                    break 'l1;
                }
                'l2: {
                    let mut txtPtr: *mut u8 = core::ptr::null_mut();
                    let mut record: u32 = 0u32;
                    record = ((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(2492))
                    .cast::<u16>())
                    .wrapping_offset((i) as isize))
                    .read()) as u32);
                    txtPtr = ConvertIntToDecimalStringN(
                        (&raw mut text).cast::<u8>(),
                        ((crate::c::div_u32(record, 100u32)) as i32),
                        1i32,
                        3u8,
                    );
                    txtPtr = StringAppend(
                        txtPtr,
                        ((&raw const sText_Dot).cast::<u8>().cast_mut()).cast::<u8>(),
                    );
                    txtPtr = ConvertIntToDecimalStringN(
                        txtPtr,
                        ((crate::c::rem_u32(record, 100u32)) as i32),
                        2i32,
                        2u8,
                    );
                    txtPtr = StringAppend(
                        txtPtr,
                        ((&raw const sText_RPM).cast::<u8>().cast_mut()).cast::<u8>(),
                    );
                    xPos = GetStringRightAlignXOffset(1i32, (&raw mut text).cast::<u8>(), 140i32);
                    AddTextPrinterParameterized(
                        ((&raw mut gRecordsWindowId).cast::<u8>()).read(),
                        1u8,
                        (&raw mut text).cast::<u8>(),
                        ((xPos) as u8),
                        (((yPos).wrapping_add((i).wrapping_mul(16i32))) as u8),
                        0u8,
                        None,
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        PutWindowTilemap(((&raw mut gRecordsWindowId).cast::<u8>()).read());
        CopyWindowToVram(((&raw mut gRecordsWindowId).cast::<u8>()).read(), 3u8);
    }
}
pub(crate) unsafe extern "C" fn Task_PlayPokeblockFanfare(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if (((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .read()) as i32)
            == 0i32
        {
            PlayFanfare(367u16);
            let __p1 = ((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>();
            (__p1).write(((__p1).read()).wrapping_add(1));
        }
        if (IsFanfareTaskInactive()) != 0 {
            PlayBGM(
                ((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(340)
                    .cast::<u16>())
                .read(),
            );
            DestroyTask(taskId);
        }
    }
}
pub(crate) unsafe extern "C" fn TryAddContestLinkTvShow(
    pokeblock: *mut u8,
    tvBlender: *mut u8,
) -> u32 {
    unsafe {
        let mut pokeblock = pokeblock;
        let mut tvBlender = tvBlender;
        let mut flavorLevel: u8 = GetHighestPokeblocksFlavorLevel(pokeblock);
        let mut sheen: u16 = ((crate::c::div_i32(
            ((flavorLevel) as i32).wrapping_mul(10i32),
            ((GetPokeblocksFeel(pokeblock)) as i32),
        )) as u16);
        ((tvBlender).wrapping_add(13)).write(((sheen) as u8));
        ((tvBlender).wrapping_add(12)).write((pokeblock).read());
        ((tvBlender).cast::<u8>()).write(255u8);
        if (((&raw mut gReceivedRemoteLinkPlayers).cast::<u8>()).read()) != 0 {
            if (((((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(420))
            .read()) as i32)
                == 0i32)
                && (((sheen) as i32) > 20i32)
            {
                StringCopy(
                    (tvBlender).cast::<u8>(),
                    ((((&raw mut gLinkPlayers).cast::<u8>()).wrapping_offset(
                        ((((((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(316))
                        .cast::<u8>())
                        .wrapping_offset(
                            (((((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(124))
                            .read()) as i32)
                                .wrapping_sub(1i32)) as isize,
                        ))
                        .read()) as i32) as isize
                            * 28,
                    ))
                    .wrapping_add(8))
                    .cast::<u8>(),
                );
                ((tvBlender).wrapping_add(11)).write(GetPokeblocksFlavor(pokeblock));
                if (Put3CheersForPokeblocksOnTheAir(
                    (tvBlender).cast::<u8>(),
                    ((tvBlender).wrapping_add(11)).read(),
                    ((tvBlender).wrapping_add(12)).read(),
                    ((tvBlender).wrapping_add(13)).read(),
                    ((((((&raw mut gLinkPlayers).cast::<u8>()).wrapping_offset(
                        ((((((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(316))
                        .cast::<u8>())
                        .wrapping_offset(
                            (((((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(124))
                            .read()) as i32)
                                .wrapping_sub(1i32)) as isize,
                        ))
                        .read()) as i32) as isize
                            * 28,
                    ))
                    .wrapping_add(26)
                    .cast::<u16>())
                    .read()) as u8),
                )) != 0
                {
                    return 1u32;
                }
                return 0u32;
            } else {
                if (((((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(420))
                .read()) as i32)
                    == ((((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(124))
                    .read()) as i32)
                        .wrapping_sub(1i32))
                    && (((sheen) as i32) <= 20i32)
                {
                    StringCopy(
                        (tvBlender).cast::<u8>(),
                        ((((&raw mut gLinkPlayers).cast::<u8>()).wrapping_offset(
                            (((((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(316))
                            .cast::<u8>())
                            .read()) as i32) as isize
                                * 28,
                        ))
                        .wrapping_add(8))
                        .cast::<u8>(),
                    );
                    ((tvBlender).wrapping_add(11)).write(GetPokeblocksFlavor(pokeblock));
                    if (Put3CheersForPokeblocksOnTheAir(
                        (tvBlender).cast::<u8>(),
                        ((tvBlender).wrapping_add(11)).read(),
                        ((tvBlender).wrapping_add(12)).read(),
                        ((tvBlender).wrapping_add(13)).read(),
                        ((((((&raw mut gLinkPlayers).cast::<u8>()).wrapping_offset(
                            (((((((&raw mut sBerryBlender).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(316))
                            .cast::<u8>())
                            .read()) as i32) as isize
                                * 28,
                        ))
                        .wrapping_add(26)
                        .cast::<u16>())
                        .read()) as u8),
                    )) != 0
                    {
                        return 1u32;
                    }
                    return 0u32;
                }
            }
        }
        return 0u32;
    }
}
pub(crate) unsafe extern "C" fn Blender_AddTextPrinter(
    windowId: u8,
    string: *mut u8,
    x: u8,
    y: u8,
    speed: i32,
    caseId: i32,
) {
    unsafe {
        let mut windowId = windowId;
        let mut string = string;
        let mut x = x;
        let mut y = y;
        let mut speed = speed;
        let mut caseId = caseId;
        let mut txtColor = crate::ffi::Align4([0u8; 3]);
        let mut letterSpacing: u32 = 0u32;
        'l1: {
            let __sw1 = caseId;
            let __matched = __sw1 == 0i32 || __sw1 == 3i32 || __sw1 == 1i32 || __sw1 == 2i32;
            if __sw1 == 0i32 || __sw1 == 3i32 || !__matched {
                ((&raw mut txtColor).cast::<u8>()).write(1u8);
                (((&raw mut txtColor).cast::<u8>()).wrapping_offset(1)).write(2u8);
                (((&raw mut txtColor).cast::<u8>()).wrapping_offset(2)).write(3u8);
                break 'l1;
            }
            if __sw1 == 1i32 {
                ((&raw mut txtColor).cast::<u8>()).write(0u8);
                (((&raw mut txtColor).cast::<u8>()).wrapping_offset(1)).write(2u8);
                (((&raw mut txtColor).cast::<u8>()).wrapping_offset(2)).write(3u8);
                break 'l1;
            }
            if __sw1 == 2i32 {
                ((&raw mut txtColor).cast::<u8>()).write(0u8);
                (((&raw mut txtColor).cast::<u8>()).wrapping_offset(1)).write(4u8);
                (((&raw mut txtColor).cast::<u8>()).wrapping_offset(2)).write(5u8);
                break 'l1;
            }
        }
        if caseId != 3i32 {
            FillWindowPixelBuffer(
                windowId,
                ((((((&raw mut txtColor).cast::<u8>()).read()) as i32)
                    | (((((&raw mut txtColor).cast::<u8>()).read()) as i32) << 4))
                    as u8),
            );
        }
        AddTextPrinterParameterized4(
            windowId,
            1u8,
            x,
            y,
            ((letterSpacing) as u8),
            1u8,
            (&raw mut txtColor).cast::<u8>(),
            ((speed) as i8),
            string,
        );
    }
}
pub(crate) unsafe extern "C" fn PrintMessage(
    textState: *mut i16,
    string: *mut u8,
    textSpeed: i32,
) -> u32 {
    unsafe {
        let mut textState = textState;
        let mut string = string;
        let mut textSpeed = textSpeed;
        'l1: {
            let __sw1 = (((textState).read()) as i32);
            if __sw1 == 0i32 {
                DrawDialogFrameWithCustomTileAndPalette(4u8, 0u8, 20u16, 15u8);
                Blender_AddTextPrinter(4u8, string, 0u8, 1u8, textSpeed, 0i32);
                PutWindowTilemap(4u8);
                CopyWindowToVram(4u8, 3u8);
                (textState).write(((textState).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                if !((IsTextPrinterActive(4u8)) != 0) {
                    (textState).write(0i16);
                    return 1u32;
                }
                break 'l1;
            }
        }
        return 0u32;
    }
}
