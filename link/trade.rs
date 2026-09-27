//! Translated from `src/trade.c` by tools/rustport/c2rs.py, then reviewed.
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

// Data tables (translate with cdata.py): sUnusedStructSizes sTradeMovesBoxTilemap sTradePartyBoxTilemap sTradeStripesBG2Tilemap sTradeStripesBG3Tilemap sText_EmptyString sText_UnusedTextFormat gText_MaleSymbol4 gText_FemaleSymbol4 gText_GenderlessSymbol sText_SpaceMove sText_NewLine sText_Slash sText_Lv sText_ThreeDashes sText_FourQuestionMarks sText_UnusedEmpty sText_IsThisTradeOkay sText_Cancel sText_ChooseAPkmn sText_Summary sText_Trade sText_CancelTrade sJPText_PressBButtonToQuit sText_Summary2 sText_Trade2 sText_CommunicationStandby sText_TheTradeHasBeenCanceled sText_OnlyPkmnForBattle sText_WaitingForYourFriend sText_YourFriendWantsToTrade sOamData_MenuText sOamData_Cursor sAnim_Cursor_Normal sAnim_Cursor_OnCancel sAnims_Cursor sCursor_SpriteSheet sCursor_SpritePalette sAnim_MenuText_0 sAnim_MenuText_1 sAnim_MenuText_2 sAnim_MenuText_3 sAnim_MenuText_4 sAnim_MenuText_5 sAnims_MenuText sSpriteTemplate_Cursor sSpriteTemplate_MenuText sMenuText_Pal sSpritePalette_MenuText sCursorMoveDestinations sTradeMonSpriteCoords sTradeMonLevelCoords sTradeMonBoxCoords sUnusedCoords sActionTexts sSelectTradeMonActions sMessages sTradeTextColors sBgTemplates sWindowTemplates sTradeYesNoWindowTemplate sText_ShedinjaJP sSelectedMonLevelGenderCoords sPokeball_Pal sPokeball_Gfx sPokeballSymbol_Gfx sCableCloseup_Map sPokeballSymbol_Map sUnusedPal1 sGba_Pal sUnusedPal2 sWirelessSignalNone_Pal_Unused sLinkMon_Pal sLinkMonGlow_Gfx sLinkMonShadow_Gfx sCableEnd_Gfx sGbaScreen_Gfx gTradePlatform_Tilemap sGbaAffine_Gfx sEmptyGfx sGbaAffineMapCable sGbaAffineMapWireless sGbaMapWireless sGbaMapCable sWirelessCloseup_Map sWirelessSignalSend_Pal sWirelessSignalRecv_Pal sWirelessSignalNone_Pal sWirelessSignal_Gfx sWirelessSignal_Tilemap sOamData_Pokeball sAnim_Pokeball_SpinOnce sAnim_Pokeball_SpinTwice sAnims_Pokeball sAffineAnim_Pokeball_Normal sAffineAnim_Pokeball_Squish sAffineAnim_Pokeball_Unsquish sAffineAnims_Pokeball sPokeBallSpriteSheet sPokeBallSpritePalette sSpriteTemplate_Pokeball sOamData_LinkMonGlow sAnim_LinkMonGlow sAnims_LinkMonGlow sAffineAnim_LinkMonGlow sAffineAnims_LinkMonGlow sSpriteSheet_LinkMonGlow sSpritePalette_LinkMon sSpritePalette_Gba sSpriteTemplate_LinkMonGlow sOamData_LinkMonShadow sAnim_LinkMonShadow_Big sAnim_LinkMonShadow_Small sAnims_LinkMonShadow sSpriteSheet_LinkMonShadow sSpriteTemplate_LinkMonShadow sOamData_CableEnd sAnim_CableEnd sAnims_CableEnd sSpriteSheet_CableEnd sSpriteTemplate_CableEnd sOamData_GbaScreen sAnim_GbaScreen_Long sAnim_GbaScreen_Short sAnims_GbaScreen_Long sAnims_GbaScreen_Short sSpriteSheet_GbaScreen sSpriteTemplate_GbaScreenFlash_Long sSpriteTemplate_GbaScreenFlash_Short sLinkMonShadow_Pal sAffineAnim_CrossingMonPic sAffineAnims_CrossingMonPics sIngameTrades sIngameTradeMail sTradeSequenceWindowTemplates gTradeEvolutionSceneYesNoWindowTemplate sTradeSequenceBgTemplates sTradeBallVerticalVelocityTable sWirelessSignalAnimParams
#[allow(unused_imports)]
use crate::data::trade::*;

#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sMenuTextTileBuffer: *mut u8 = core::ptr::null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sMenuTextTileBuffers: crate::ffi::Align4<[u8; 56]> =
    crate::ffi::Align4([0; 56]);
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gTradeMail: crate::ffi::Align4<[u8; 216]> = crate::ffi::Align4([0; 216]);
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gSelectedTradeMonPositions: crate::ffi::Align4<[u8; 2]> = crate::ffi::Align4([0; 2]);
pub(crate) static mut sTradeMenu: *mut u8 = core::ptr::null_mut();
pub(crate) static mut sTradeAnim: *mut u8 = core::ptr::null_mut();

unsafe extern "C" {
    static mut gBattleTextboxPalette: u8;
    static mut gBattleTextboxTilemap: u8;
    static mut gBattleTextboxTiles: u8;
    static mut gBlockRecvBuffer: u8;
    static mut gBlockSendBuffer: u8;
    static mut gCB2_AfterEvolution: u8;
    static mut gDecompressionBuffer: u8;
    static mut gEnemyParty: u8;
    static mut gEnemyPartyCount: u8;
    static mut gFieldCallback: u8;
    static mut gLastViewedMonIndex: u8;
    static mut gLinkPlayers: u8;
    static mut gLinkType: u8;
    static mut gMain: u8;
    static mut gMonFrontPicCoords: u8;
    static mut gMonFrontPicTable: u8;
    static mut gMonSpritesGfxPtr: u8;
    static mut gMoveNames: u8;
    static mut gMultiuseSpriteTemplate: u8;
    static mut gPaletteFade: u8;
    static mut gPlayerCurrActivity: u8;
    static mut gPlayerParty: u8;
    static mut gPlayerPartyCount: u8;
    static mut gReceivedRemoteLinkPlayers: u8;
    static mut gRfuSlotStatusNI: u8;
    static mut gSaveBlock1Ptr: u8;
    static mut gSaveBlock2Ptr: u8;
    static mut gSoftResetDisabled: u8;
    static mut gSpecialVar_0x8004: u8;
    static mut gSpecialVar_0x8005: u8;
    static mut gSpeciesInfo: u8;
    static mut gSpeciesNames: u8;
    static mut gSprites: u8;
    static mut gStandardMenuPalette: u8;
    static mut gStringVar1: u8;
    static mut gStringVar2: u8;
    static mut gStringVar3: u8;
    static mut gStringVar4: u8;
    static mut gTasks: u8;
    static mut gText_ByeByeVar1: u8;
    static mut gText_CommunicationStandby5: u8;
    static mut gText_SavingDontTurnOffPower: u8;
    static mut gText_TakeGoodCareOfX: u8;
    static mut gText_XSentOverY: u8;
    static mut gText_XWillBeSentToY: u8;
    static mut gTradeGba2_Pal: u8;
    static mut gTradeGba_Gfx: u8;
    static mut gTradeMenuMonBox_Tilemap: u8;
    static mut gTradeMenu_Gfx: u8;
    static mut gTradeMenu_Pal: u8;
    static mut gTradeMenu_Tilemap: u8;
    static mut gWirelessCommType: u8;
    static mut lman: u8;
    fn AddTextPrinterParameterized(
        a0: u8,
        a1: u8,
        a2: *mut u8,
        a3: u8,
        a4: u8,
        a5: u8,
        a6: Option<unsafe extern "C" fn(*mut u8, u16)>,
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
    fn Alloc(a0: u32) -> *mut u8;
    fn AllocZeroed(a0: u32) -> *mut u8;
    fn AllocateMonSpritesGfx();
    fn AnimateSprites();
    fn BeginNormalPaletteFade(a0: u32, a1: i8, a2: u8, a3: u8, a4: u16) -> u8;
    fn BitmaskAllOtherLinkPlayers() -> u8;
    fn BlendPalettes(a0: u32, a1: u8, a2: u16);
    fn BuildOamBuffer();
    fn CB2_LinkError();
    fn CB2_ReturnToField();
    fn CB2_ReturnToFieldFromMultiplayer();
    fn CalcCenterToCornerVec(a0: *mut u8, a1: u8, a2: u8, a3: u8);
    fn CalculateEnemyPartyCount() -> u8;
    fn CalculateMonStats(a0: *mut u8);
    fn CalculatePlayerPartyCount() -> u8;
    fn ChangeBgX(a0: u8, a1: i32, a2: u8) -> i32;
    fn ChangeBgY(a0: u8, a1: i32, a2: u8) -> i32;
    fn CheckShouldAdvanceLinkState();
    fn ClearContinueGameWarpStatus2();
    fn ClearGpuRegBits(a0: u8, a1: u16);
    fn ClearMail(a0: *mut u8);
    fn ClearWindowTilemap(a0: u8);
    fn CloseLink();
    fn CopyBgTilemapBufferToVram(a0: u8);
    fn CopyToBgTilemapBuffer(a0: u8, a1: *mut u8, a2: u16, a3: u16);
    fn CopyToBgTilemapBufferRect_ChangePalette(
        a0: u8,
        a1: *mut u8,
        a2: u8,
        a3: u8,
        a4: u8,
        a5: u8,
        a6: u8,
    );
    fn CopyWindowToVram(a0: u8, a1: u8);
    fn CreateMon(a0: *mut u8, a1: u16, a2: u8, a3: u8, a4: u8, a5: u32, a6: u8, a7: u32);
    fn CreateMonIcon(
        a0: u16,
        a1: Option<unsafe extern "C" fn(*mut u8)>,
        a2: i16,
        a3: i16,
        a4: u8,
        a5: u32,
        a6: u32,
    ) -> u8;
    fn CreatePokeballSpriteToReleaseMon(
        a0: u8,
        a1: u8,
        a2: u8,
        a3: u8,
        a4: u8,
        a5: u8,
        a6: u8,
        a7: u32,
        a8: u16,
    );
    fn CreateSprite(a0: *mut u8, a1: i16, a2: i16, a3: u8) -> u8;
    fn CreateTask(a0: Option<unsafe extern "C" fn(u8)>, a1: u8) -> u8;
    fn CreateTask_RfuIdle();
    fn CreateTradePokeballSprite(
        a0: u8,
        a1: u8,
        a2: u8,
        a3: u8,
        a4: u8,
        a5: u8,
        a6: u8,
        a7: u32,
    ) -> u8;
    fn CreateWirelessStatusIndicatorSprite(a0: u8, a1: u8);
    fn CreateYesNoMenu(a0: *mut u8, a1: u16, a2: u8, a3: u8);
    fn DeactivateAllTextPrinters();
    fn DecompressAndLoadBgGfxUsingHeap(a0: u8, a1: *mut u8, a2: u32, a3: u16, a4: u8);
    fn DestroySprite(a0: *mut u8);
    fn DestroyTask(a0: u8);
    fn DestroyTask_RfuIdle();
    fn DestroyWirelessStatusIndicatorSprite();
    fn DoBgAffineSet(a0: *mut u8, a1: u32, a2: u32, a3: i16, a4: i16, a5: i16, a6: i16, a7: u16);
    fn DrawHeldItemIconsForTrade(a0: *mut u8, a1: *mut u8, a2: u8);
    fn DrawTextBorderOuter(a0: u8, a1: u16, a2: u8);
    fn DrawTextWindowAndBufferTiles(a0: *mut u8, a1: *mut u8, a2: u8, a3: u8, a4: i32);
    fn FadeOutBGM(a0: u8);
    fn FieldCB_ContinueScriptHandleMusic();
    fn FillBgTilemapBufferRect(a0: u8, a1: u16, a2: u8, a3: u8, a4: u8, a5: u8, a6: u8);
    fn FillBgTilemapBufferRect_Palette0(a0: u8, a1: u16, a2: u8, a3: u8, a4: u8, a5: u8);
    fn FillWindowPixelBuffer(a0: u8, a1: u8);
    fn FlagSet(a0: u16) -> u8;
    fn Free(a0: *mut u8);
    fn FreeAllSpritePalettes();
    fn FreeAllWindowBuffers();
    fn FreeMonSpritesGfx();
    fn FreeSpriteOamMatrix(a0: *mut u8);
    fn FuncIsActiveTask(a0: Option<unsafe extern "C" fn(u8)>) -> u8;
    fn GetBgTilemapBuffer(a0: u8) -> *mut u8;
    fn GetBlockReceivedStatus() -> u8;
    fn GetCurrentMapMusic() -> u16;
    fn GetEvolutionTargetSpecies(a0: *mut u8, a1: u8, a2: u16) -> u16;
    fn GetFontAttribute(a0: u8, a1: u8) -> u8;
    fn GetGpuReg(a0: u8) -> u16;
    fn GetHPBarLevel(a0: i16, a1: i16) -> u8;
    fn GetLinkPlayerCount() -> u8;
    fn GetLinkPlayerCount_2() -> u8;
    fn GetMonData2(a0: *mut u8, a1: i32) -> u32;
    fn GetMonData3(a0: *mut u8, a1: i32, a2: *mut u8) -> u32;
    fn GetMonGender(a0: *mut u8) -> u8;
    fn GetMonSpritePalStruct(a0: *mut u8) -> *mut u8;
    fn GetMultiplayerId() -> u8;
    fn GetSavedPlayerCount() -> u8;
    fn GetSetPokedexFlag(a0: u16, a1: u8) -> i8;
    fn GetStringCenterAlignXOffset(a0: i32, a1: *mut u8, a2: i32) -> i32;
    fn GetStringWidth(a0: u8, a1: *mut u8, a2: i16) -> i32;
    fn GiveMailToMon(a0: *mut u8, a1: *mut u8) -> u8;
    fn HandleLoadSpecialPokePic_2(a0: *mut u8, a1: *mut u8, a2: i32, a3: u32);
    fn HandleLoadSpecialPokePic_DontHandleDeoxys(a0: *mut u8, a1: *mut u8, a2: i32, a3: u32);
    fn HandleSetPokedexFlag(a0: u16, a1: u8, a2: u32);
    fn HasLinkErrorOccurred() -> u8;
    fn InUnionRoom() -> u32;
    fn IncrementGameStat(a0: u8);
    fn InitBgsFromTemplates(a0: u8, a1: *mut u8, a2: u8);
    fn InitMenuInUpperLeftCornerNormal(a0: u8, a1: u8, a2: u8) -> u8;
    fn InitWindows(a0: *mut u8) -> u16;
    fn IsBGMStopped() -> u8;
    fn IsCryFinished() -> u8;
    fn IsLinkMaster() -> u8;
    fn IsLinkPlayerDataExchangeComplete() -> u8;
    fn IsLinkRfuTaskFinished() -> u8;
    fn IsLinkTaskFinished() -> u8;
    fn IsMonSpriteNotFlipped(a0: u16) -> u8;
    fn IsNationalPokedexEnabled() -> u32;
    fn IsSpeciesInHoennDex(a0: u16) -> u32;
    fn ItemIsMail(a0: u16) -> u8;
    fn LZ77UnCompVram(a0: *mut u32, a1: *mut u8);
    fn LZDecompressWram(a0: *mut u32, a1: *mut u8);
    fn LinkFullSave_Init() -> u8;
    fn LinkFullSave_ReplaceLastSector() -> u8;
    fn LinkFullSave_SetLastSectorSignature() -> u8;
    fn LinkFullSave_WriteSector() -> u8;
    fn LoadBgTilemap(a0: u8, a1: *mut u8, a2: u16, a3: u16) -> u16;
    fn LoadBgTiles(a0: u8, a1: *mut u8, a2: u16, a3: u16) -> u16;
    fn LoadCompressedPalette(a0: *mut u32, a1: u16, a2: u16);
    fn LoadCompressedSpritePalette(a0: *mut u8);
    fn LoadHeldItemIcons();
    fn LoadMonIconPalettes();
    fn LoadOam();
    fn LoadPalette(a0: *mut u8, a1: u16, a2: u16);
    fn LoadSpritePalette(a0: *mut u8) -> u8;
    fn LoadSpriteSheet(a0: *mut u8) -> u16;
    fn LoadUserWindowBorderGfx(a0: u8, a1: u16, a2: u8);
    fn LoadUserWindowBorderGfx_(a0: u8, a1: u16, a2: u8);
    fn LoadWirelessStatusIndicatorSpriteGfx();
    fn LockPlayerFieldControls();
    fn Menu_ProcessInputNoWrap() -> i8;
    fn Menu_ProcessInputNoWrapClearOnChoose() -> i8;
    fn MysteryGift_TryIncrementStat(a0: u32, a1: u32);
    fn NameHasGenderSymbol(a0: *mut u8, a1: u8) -> u8;
    fn OpenLink();
    fn PadNameString(a0: *mut u8, a1: u8);
    fn PlayBGM(a0: u16);
    fn PlayCry_Normal(a0: u16, a1: i8);
    fn PlayFanfare(a0: u16);
    fn PlayNewMapMusic(a0: u16);
    fn PlaySE(a0: u16);
    fn PrintMenuTable(a0: u8, a1: u8, a2: *mut u8);
    fn ProcessSpriteCopyRequests();
    fn PutWindowTilemap(a0: u8);
    fn Random() -> u16;
    fn ResetBgsAndClearDma3BusyFlags(a0: u32);
    fn ResetBlockReceivedFlag(a0: u8);
    fn ResetBlockReceivedFlags();
    fn ResetPaletteFade();
    fn ResetSpriteData();
    fn ResetTasks();
    fn Rfu_GetIndexOfNewestChild(a0: u8) -> i32;
    fn Rfu_SetLinkRecovery(a0: u32) -> u8;
    fn RunTasks();
    fn RunTextPrinters();
    fn RunTextPrintersAndIsPrinter0Active() -> u16;
    fn SendBlock(a0: u8, a1: *mut u8, a2: u16) -> u8;
    fn SendBlockRequest(a0: u8) -> u8;
    fn SetBgTilemapBuffer(a0: u8, a1: *mut u8);
    fn SetCloseLinkCallback();
    fn SetCloseLinkCallbackAndType(a0: u16);
    fn SetContinueGameWarpStatusToDynamicWarp();
    fn SetGpuReg(a0: u8, a1: u16);
    fn SetGpuRegBits(a0: u8, a1: u16);
    fn SetLinkStandbyCallback();
    fn SetMainCallback2(a0: Option<unsafe extern "C" fn()>);
    fn SetMonData(a0: *mut u8, a1: i32, a2: *mut u8);
    fn SetMultiuseSpriteTemplateToPokemon(a0: u16, a1: u8);
    fn SetPartyHPBarSprite(a0: *mut u8, a1: u8);
    fn SetVBlankCallback(a0: Option<unsafe extern "C" fn()>);
    fn SetWirelessCommType1();
    fn ShowBg(a0: u8);
    fn ShowPokemonSummaryScreen(
        a0: u8,
        a1: *mut u8,
        a2: u8,
        a3: u8,
        a4: Option<unsafe extern "C" fn()>,
    );
    fn SpeciesToNationalPokedexNum(a0: u16) -> u16;
    fn SpriteCB_MonIcon(a0: *mut u8);
    fn SpriteCallbackDummy(a0: *mut u8);
    fn StartSpriteAffineAnim(a0: *mut u8, a1: u8);
    fn StartSpriteAnim(a0: *mut u8, a1: u8);
    fn StoreSpriteCallbackInData6(a0: *mut u8, a1: Option<unsafe extern "C" fn(*mut u8)>);
    fn StringAppend(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn StringCompareWithoutExtCtrlCodes(a0: *mut u8, a1: *mut u8) -> i32;
    fn StringCopy(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn StringCopy_Nickname(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn StringExpandPlaceholders(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn Task_WaitForLinkPlayerConnection(a0: u8);
    fn TradeEvolutionScene(a0: *mut u8, a1: u16, a2: u8, a3: u8);
    fn Trade_MoveSelectedMonToTarget(a0: *mut u8);
    fn TransferPlttBuffer();
    fn UpdatePaletteFade() -> u8;
    fn rbox_fill_rectangle(a0: u8);
    fn rfu_NI_setSendData(a0: u8, a1: u8, a2: *mut u8, a3: u32) -> u16;
    fn rfu_clearSlot(a0: u8, a1: u8) -> u16;
}

pub(crate) unsafe extern "C" fn SendLinkData(linkData: *mut u8, size: u32) -> u8 {
    unsafe {
        let mut linkData = linkData;
        let mut size = size;
        if ((((&raw mut gPlayerCurrActivity).cast::<u8>()).read()) as i32) == 29i32 {
            rfu_NI_setSendData(((&raw mut lman).cast::<u8>()).read(), 84u8, linkData, size);
            return 1u8;
        } else {
            return SendBlock(0u8, linkData, ((size) as u16));
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
pub(crate) unsafe extern "C" fn RequestLinkData(r#type: u8) {
    unsafe {
        let mut r#type = r#type;
        SendBlockRequest(r#type);
    }
}
pub(crate) unsafe extern "C" fn IsLinkTradeTaskFinished() -> u32 {
    unsafe {
        if ((((&raw mut gPlayerCurrActivity).cast::<u8>()).read()) as i32) == 29i32 {
            if ((((((((&raw mut gRfuSlotStatusNI).cast::<*mut u8>()).cast::<*mut u8>())
                .wrapping_offset(
                    (Rfu_GetIndexOfNewestChild(((&raw mut lman).cast::<u8>()).read())) as isize,
                ))
            .read())
            .cast::<u16>())
            .read()) as i32)
                == 0i32
            {
                return 1u32;
            } else {
                return 0u32;
            }
        } else {
            return ((IsLinkTaskFinished()) as u32);
        }
        #[allow(unreachable_code)]
        {
            return 0u32;
        }
    }
}
pub(crate) unsafe extern "C" fn _GetBlockReceivedStatus() -> u32 {
    unsafe {
        return ((GetBlockReceivedStatus()) as u32);
    }
}
pub(crate) unsafe extern "C" fn TradeResetReceivedFlags() {
    unsafe {
        if (IsWirelessTrade()) != 0 {
            rfu_clearSlot(12u8, ((&raw mut lman).cast::<u8>()).read());
        } else {
            ResetBlockReceivedFlags();
        }
    }
}
pub(crate) unsafe extern "C" fn TradeResetReceivedFlag(who: u32) {
    unsafe {
        let mut who = who;
        if (IsWirelessTrade()) != 0 {
            rfu_clearSlot(12u8, ((&raw mut lman).cast::<u8>()).read());
        } else {
            ResetBlockReceivedFlag(((who) as u8));
        }
    }
}
pub(crate) unsafe extern "C" fn IsWirelessTrade() -> u32 {
    unsafe {
        if ((((&raw mut gWirelessCommType).cast::<u8>()).read()) != 0)
            && (((((&raw mut gPlayerCurrActivity).cast::<u8>()).read()) as i32) == 29i32)
        {
            return 1u32;
        } else {
            return 0u32;
        }
        #[allow(unreachable_code)]
        {
            return 0u32;
        }
    }
}
pub(crate) unsafe extern "C" fn SetTradeLinkStandbyCallback(unused: u8) {
    unsafe {
        let mut unused = unused;
        SetLinkStandbyCallback();
    }
}
pub(crate) unsafe extern "C" fn _IsLinkTaskFinished() -> u32 {
    unsafe {
        return ((IsLinkTaskFinished()) as u32);
    }
}
pub(crate) unsafe extern "C" fn InitTradeMenu() {
    unsafe {
        ResetSpriteData();
        FreeAllSpritePalettes();
        ResetTasks();
        ResetPaletteFade();
        crate::c::bf_write(
            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(8),
            7,
            1,
            (1u16) as i32,
        );
        SetVBlankCallback(Some(VBlankCB_TradeMenu));
        LoadPalette(
            (((&raw mut gStandardMenuPalette).cast::<u16>()).cast::<u16>()).cast::<u8>(),
            240u16,
            20u16,
        );
        LoadPalette(
            (((&raw mut gStandardMenuPalette).cast::<u16>()).cast::<u16>()).cast::<u8>(),
            208u16,
            20u16,
        );
        ResetBgsAndClearDma3BusyFlags(0u32);
        InitBgsFromTemplates(
            0u8,
            ((&raw const sBgTemplates).cast::<u8>().cast_mut()).cast::<u8>(),
            ((crate::c::div_u32(16u32, 4u32)) as u8),
        );
        SetBgTilemapBuffer(
            1u8,
            (((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(2288))
                .cast::<u16>())
            .cast::<u8>(),
        );
        if (InitWindows(((&raw const sWindowTemplates).cast::<u8>().cast_mut()).cast::<u8>())) != 0
        {
            let mut i: u32 = 0u32;
            DeactivateAllTextPrinters();
            {
                i = 0u32;
                'l1: loop {
                    if !(i < (crate::c::div_u32(152u32, 8u32)).wrapping_sub(1u32)) {
                        break 'l1;
                    }
                    'l2: {
                        ClearWindowTilemap(((i) as u8));
                        FillWindowPixelBuffer(((i) as u8), 0u8);
                    }
                    i = (i).wrapping_add(1);
                }
            }
            FillBgTilemapBufferRect(
                0u8,
                0u16,
                0u8,
                0u8,
                ((crate::c::div_i32(240i32, 8i32)) as u8),
                ((crate::c::div_i32(160i32, 8i32)) as u8),
                15u8,
            );
            LoadUserWindowBorderGfx_(0u8, 20u16, 192u8);
            LoadUserWindowBorderGfx(2u8, 1u16, 224u8);
            LoadMonIconPalettes();
            ((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(105))
                .write(0u8);
            ((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(111))
                .write(0u8);
            ((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(112))
                .write(0u8);
            (((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(116))
                .cast::<u8>())
            .write(0u8);
            ((((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(116))
                .cast::<u8>())
            .wrapping_offset(1))
            .write(0u8);
            ((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(122))
                .write(0u8);
            ((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(123))
                .write(0u8);
            ((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(168))
                .write(0u8);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CB2_StartCreateTradeMenu() {
    unsafe {
        SetMainCallback2(Some(CB2_CreateTradeMenu));
        (((&raw mut gMain).cast::<u8>()).cast::<Option<unsafe extern "C" fn()>>()).write(None);
        ((&raw mut gEnemyPartyCount).cast::<u8>()).write(0u8);
    }
}
pub(crate) unsafe extern "C" fn CB2_CreateTradeMenu() {
    unsafe {
        let mut i: i32 = 0i32;
        let mut temp = crate::ffi::Align4([0u8; 24]);
        let mut id: u8 = 0u8;
        let mut xPos: u32 = 0u32;
        'l1: {
            let __sw1 = (((((&raw mut gMain).cast::<u8>()).wrapping_add(1080)).read()) as i32);
            let mut __fall = false;
            if __sw1 == 0i32 {
                __fall = true;
                ((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).write(AllocZeroed(4336u32));
                InitTradeMenu();
                ((&raw mut sMenuTextTileBuffer)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .write(AllocZeroed(3584u32));
                {
                    i = 0i32;
                    'l2: loop {
                        if !(i < 14i32) {
                            break 'l2;
                        }
                        'l3: {
                            ((((&raw mut sMenuTextTileBuffers)
                                .cast::<u8>()
                                .cast::<*mut u8>())
                            .cast::<*mut u8>())
                            .wrapping_offset((i) as isize))
                            .write(
                                (((&raw mut sMenuTextTileBuffer)
                                    .cast::<u8>()
                                    .cast::<*mut u8>())
                                .read())
                                .wrapping_offset(((i).wrapping_mul(256i32)) as isize),
                            );
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                let __p2 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                __fall = true;
                crate::c::bf_write(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(8),
                    7,
                    1,
                    (0u16) as i32,
                );
                {
                    i = 0i32;
                    'l4: loop {
                        if !(i < 6i32) {
                            break 'l4;
                        }
                        'l5: {
                            CreateMon(
                                ((&raw mut gEnemyParty).cast::<u8>())
                                    .wrapping_offset((i) as isize * 100),
                                0u16,
                                0u8,
                                32u8,
                                0u8,
                                0u32,
                                0u8,
                                0u32,
                            );
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                PrintTradeMessage(0u8);
                ShowBg(0u8);
                if !((((&raw mut gReceivedRemoteLinkPlayers).cast::<u8>()).read()) != 0) {
                    ((&raw mut gLinkType).cast::<u16>()).write(4386u16);
                    ((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(168))
                    .write(0u8);
                    if (((&raw mut gWirelessCommType).cast::<u8>()).read()) != 0 {
                        SetWirelessCommType1();
                        OpenLink();
                        CreateTask_RfuIdle();
                    } else {
                        OpenLink();
                        let __p3 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                        (__p3).write(((__p3).read()).wrapping_add(1));
                        CreateTask(Some(Task_WaitForLinkPlayerConnection), 1u8);
                    }
                } else {
                    (((&raw mut gMain).cast::<u8>()).wrapping_add(1080)).write(4u8);
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                __fall = true;
                let __p4 = (((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(168);
                (__p4).write(((__p4).read()).wrapping_add(1));
                if ((((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(168))
                .read()) as i32)
                    > 11i32
                {
                    ((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(168))
                    .write(0u8);
                    let __p5 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                    (__p5).write(((__p5).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                __fall = true;
                if ((GetLinkPlayerCount_2()) as i32) >= ((GetSavedPlayerCount()) as i32) {
                    if (IsLinkMaster()) != 0 {
                        if (({
                            let __p6 = (((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(168);
                            let __t7 = ((__p6).read()).wrapping_add(1);
                            (__p6).write(__t7);
                            __t7
                        }) as i32)
                            > 30i32
                        {
                            CheckShouldAdvanceLinkState();
                            let __p8 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                            (__p8).write(((__p8).read()).wrapping_add(1));
                        }
                    } else {
                        let __p9 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                        (__p9).write(((__p9).read()).wrapping_add(1));
                    }
                }
                break 'l1;
            }
            if __sw1 == 4i32 {
                __fall = true;
                if (((((&raw mut gReceivedRemoteLinkPlayers).cast::<u8>()).read()) as i32) == 1i32)
                    && (((IsLinkPlayerDataExchangeComplete()) as i32) == 1i32)
                {
                    DestroyTask_RfuIdle();
                    CalculatePlayerPartyCount();
                    let __p10 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                    (__p10).write(((__p10).read()).wrapping_add(1));
                    ((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(168))
                    .write(0u8);
                    if (((&raw mut gWirelessCommType).cast::<u8>()).read()) != 0 {
                        Rfu_SetLinkRecovery(1u32);
                        SetLinkStandbyCallback();
                    }
                }
                break 'l1;
            }
            if __sw1 == 5i32 {
                __fall = true;
                if (((&raw mut gWirelessCommType).cast::<u8>()).read()) != 0 {
                    if (IsLinkRfuTaskFinished()) != 0 {
                        let __p11 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                        (__p11).write(((__p11).read()).wrapping_add(1));
                        LoadWirelessStatusIndicatorSpriteGfx();
                        CreateWirelessStatusIndicatorSprite(0u8, 0u8);
                    }
                } else {
                    let __p12 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                    (__p12).write(((__p12).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 6i32 {
                __fall = true;
                if (BufferTradeParties()) != 0 {
                    SaveTradeGiftRibbons();
                    let __p13 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                    (__p13).write(((__p13).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 7i32 {
                __fall = true;
                CalculateEnemyPartyCount();
                SetGpuReg(0u8, 0u16);
                SetGpuReg(80u8, 0u16);
                (((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(54))
                .cast::<u8>())
                .write(((&raw mut gPlayerPartyCount).cast::<u8>()).read());
                ((((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(54))
                .cast::<u8>())
                .wrapping_offset(1))
                .write(((&raw mut gEnemyPartyCount).cast::<u8>()).read());
                {
                    i = 0i32;
                    'l6: loop {
                        if !(i
                            < (((((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(54))
                            .cast::<u8>())
                            .read()) as i32))
                        {
                            break 'l6;
                        }
                        'l7: {
                            let mut mon: *mut u8 = ((&raw mut gPlayerParty).cast::<u8>())
                                .wrapping_offset((i) as isize * 100);
                            (((((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(40))
                            .cast::<u8>())
                            .cast::<u8>())
                            .wrapping_offset((i) as isize))
                            .write(CreateMonIcon(
                                ((GetMonData2(mon, 65i32)) as u16),
                                Some(SpriteCB_MonIcon),
                                ((((((((((&raw const sTradeMonSpriteCoords)
                                    .cast::<u8>()
                                    .cast_mut())
                                .cast::<u8>())
                                .wrapping_offset((i) as isize * 2))
                                .cast::<u8>())
                                .read()) as i32)
                                    .wrapping_mul(8i32))
                                .wrapping_add(14i32)) as i16),
                                (((((((((((&raw const sTradeMonSpriteCoords)
                                    .cast::<u8>()
                                    .cast_mut())
                                .cast::<u8>())
                                .wrapping_offset((i) as isize * 2))
                                .cast::<u8>())
                                .wrapping_offset(1))
                                .read()) as i32)
                                    .wrapping_mul(8i32))
                                .wrapping_sub(12i32)) as i16),
                                1u8,
                                GetMonData2(mon, 0i32),
                                1u32,
                            ));
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                {
                    i = 0i32;
                    'l8: loop {
                        if !(i
                            < ((((((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(54))
                            .cast::<u8>())
                            .wrapping_offset(1))
                            .read()) as i32))
                        {
                            break 'l8;
                        }
                        'l9: {
                            let mut mon: *mut u8 = ((&raw mut gEnemyParty).cast::<u8>())
                                .wrapping_offset((i) as isize * 100);
                            ((((((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(40))
                            .cast::<u8>())
                            .wrapping_offset(6))
                            .cast::<u8>())
                            .wrapping_offset((i) as isize))
                            .write(CreateMonIcon(
                                ((GetMonData3(mon, 65i32, core::ptr::null_mut())) as u16),
                                Some(SpriteCB_MonIcon),
                                ((((((((((&raw const sTradeMonSpriteCoords)
                                    .cast::<u8>()
                                    .cast_mut())
                                .cast::<u8>())
                                .wrapping_offset(((i).wrapping_add(6i32)) as isize * 2))
                                .cast::<u8>())
                                .read()) as i32)
                                    .wrapping_mul(8i32))
                                .wrapping_add(14i32)) as i16),
                                (((((((((((&raw const sTradeMonSpriteCoords)
                                    .cast::<u8>()
                                    .cast_mut())
                                .cast::<u8>())
                                .wrapping_offset(((i).wrapping_add(6i32)) as isize * 2))
                                .cast::<u8>())
                                .wrapping_offset(1))
                                .read()) as i32)
                                    .wrapping_mul(8i32))
                                .wrapping_sub(12i32)) as i16),
                                1u8,
                                GetMonData2(mon, 0i32),
                                0u32,
                            ));
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                let __p14 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p14).write(((__p14).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 8i32 {
                __fall = true;
                LoadHeldItemIcons();
                DrawHeldItemIconsForTrade(
                    ((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(54))
                    .cast::<u8>(),
                    (((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(40))
                    .cast::<u8>())
                    .cast::<u8>(),
                    0u8,
                );
                let __p15 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p15).write(((__p15).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 9i32 {
                __fall = true;
                DrawHeldItemIconsForTrade(
                    ((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(54))
                    .cast::<u8>(),
                    (((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(40))
                    .cast::<u8>())
                    .cast::<u8>(),
                    1u8,
                );
                let __p16 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p16).write(((__p16).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 10i32 {
                __fall = true;
                DrawTextWindowAndBufferTiles(
                    (((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).cast::<u8>(),
                    (((&raw mut sMenuTextTileBuffers)
                        .cast::<u8>()
                        .cast::<*mut u8>())
                    .cast::<*mut u8>())
                    .read(),
                    0u8,
                    0u8,
                    3i32,
                );
                id = GetMultiplayerId();
                DrawTextWindowAndBufferTiles(
                    ((((&raw mut gLinkPlayers).cast::<u8>())
                        .wrapping_offset((((id) as i32) ^ 1i32) as isize * 28))
                    .wrapping_add(8))
                    .cast::<u8>(),
                    ((((&raw mut sMenuTextTileBuffers)
                        .cast::<u8>()
                        .cast::<*mut u8>())
                    .cast::<*mut u8>())
                    .wrapping_offset(3))
                    .read(),
                    0u8,
                    0u8,
                    3i32,
                );
                DrawTextWindowAndBufferTiles(
                    (((&raw const sActionTexts)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<*mut u8>())
                    .cast::<*mut u8>())
                    .read(),
                    ((((&raw mut sMenuTextTileBuffers)
                        .cast::<u8>()
                        .cast::<*mut u8>())
                    .cast::<*mut u8>())
                    .wrapping_offset(6))
                    .read(),
                    0u8,
                    0u8,
                    2i32,
                );
                DrawBottomRowText(
                    ((((&raw const sActionTexts)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<*mut u8>())
                    .cast::<*mut u8>())
                    .wrapping_offset(1))
                    .read(),
                    ((((&raw mut sMenuTextTileBuffers)
                        .cast::<u8>()
                        .cast::<*mut u8>())
                    .cast::<*mut u8>())
                    .wrapping_offset(8))
                    .read(),
                    24u8,
                );
                let __p17 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p17).write(((__p17).read()).wrapping_add(1));
                ((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(168))
                    .write(0u8);
                break 'l1;
            }
            if __sw1 == 11i32 {
                __fall = true;
                if (LoadUISpriteGfx()) != 0 {
                    let __p18 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                    (__p18).write(((__p18).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 12i32 {
                __fall = true;
                xPos = ((GetStringCenterAlignXOffset(
                    1i32,
                    (((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).cast::<u8>(),
                    120i32,
                )) as u32);
                {
                    i = 0i32;
                    'l10: loop {
                        if !(i < 3i32) {
                            break 'l10;
                        }
                        'l11: {
                            (&raw mut temp)
                                .cast::<u8>()
                                .cast::<crate::c::Rec4<24>>()
                                .write_unaligned(
                                    (&raw const sSpriteTemplate_MenuText)
                                        .cast::<u8>()
                                        .cast_mut()
                                        .cast::<crate::c::Rec4<24>>()
                                        .read_unaligned(),
                                );
                            let __p19 = ((&raw mut temp).cast::<u8>()).cast::<u16>();
                            (__p19).write(
                                (((((__p19).read()) as i32).wrapping_add((i).wrapping_add(0i32)))
                                    as u16),
                            );
                            CreateSprite(
                                (&raw mut temp).cast::<u8>(),
                                ((((xPos).wrapping_add((((i).wrapping_mul(32i32)) as u32)))
                                    .wrapping_add(16u32)) as i16),
                                10i16,
                                1u8,
                            );
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                xPos = ((GetStringCenterAlignXOffset(
                    1i32,
                    ((((&raw mut gLinkPlayers).cast::<u8>())
                        .wrapping_offset((((GetMultiplayerId()) as i32) ^ 1i32) as isize * 28))
                    .wrapping_add(8))
                    .cast::<u8>(),
                    120i32,
                )) as u32);
                {
                    i = 0i32;
                    'l12: loop {
                        if !(i < 3i32) {
                            break 'l12;
                        }
                        'l13: {
                            (&raw mut temp)
                                .cast::<u8>()
                                .cast::<crate::c::Rec4<24>>()
                                .write_unaligned(
                                    (&raw const sSpriteTemplate_MenuText)
                                        .cast::<u8>()
                                        .cast_mut()
                                        .cast::<crate::c::Rec4<24>>()
                                        .read_unaligned(),
                                );
                            let __p20 = ((&raw mut temp).cast::<u8>()).cast::<u16>();
                            (__p20).write(
                                (((((__p20).read()) as i32).wrapping_add((i).wrapping_add(3i32)))
                                    as u16),
                            );
                            CreateSprite(
                                (&raw mut temp).cast::<u8>(),
                                ((((xPos).wrapping_add((((i).wrapping_mul(32i32)) as u32)))
                                    .wrapping_add(136u32)) as i16),
                                10i16,
                                1u8,
                            );
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                let __p21 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p21).write(((__p21).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 13i32 {
                __fall = true;
                (&raw mut temp)
                    .cast::<u8>()
                    .cast::<crate::c::Rec4<24>>()
                    .write_unaligned(
                        (&raw const sSpriteTemplate_MenuText)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<crate::c::Rec4<24>>()
                            .read_unaligned(),
                    );
                let __p22 = ((&raw mut temp).cast::<u8>()).cast::<u16>();
                (__p22).write((((((__p22).read()) as i32).wrapping_add(6i32)) as u16));
                CreateSprite((&raw mut temp).cast::<u8>(), 215i16, 152i16, 1u8);
                (&raw mut temp)
                    .cast::<u8>()
                    .cast::<crate::c::Rec4<24>>()
                    .write_unaligned(
                        (&raw const sSpriteTemplate_MenuText)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<crate::c::Rec4<24>>()
                            .read_unaligned(),
                    );
                let __p23 = ((&raw mut temp).cast::<u8>()).cast::<u16>();
                (__p23).write((((((__p23).read()) as i32).wrapping_add(7i32)) as u16));
                CreateSprite((&raw mut temp).cast::<u8>(), 247i16, 152i16, 1u8);
                {
                    i = 0i32;
                    'l14: loop {
                        if !(i < 6i32) {
                            break 'l14;
                        }
                        'l15: {
                            (&raw mut temp)
                                .cast::<u8>()
                                .cast::<crate::c::Rec4<24>>()
                                .write_unaligned(
                                    (&raw const sSpriteTemplate_MenuText)
                                        .cast::<u8>()
                                        .cast_mut()
                                        .cast::<crate::c::Rec4<24>>()
                                        .read_unaligned(),
                                );
                            let __p24 = ((&raw mut temp).cast::<u8>()).cast::<u16>();
                            (__p24).write(
                                (((((__p24).read()) as i32).wrapping_add((i).wrapping_add(8i32)))
                                    as u16),
                            );
                            CreateSprite(
                                (&raw mut temp).cast::<u8>(),
                                ((((i).wrapping_mul(32i32)).wrapping_add(24i32)) as i16),
                                150i16,
                                1u8,
                            );
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                ((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(52))
                    .write(CreateSprite(
                        (&raw const sSpriteTemplate_Cursor).cast::<u8>().cast_mut(),
                        (((((((((&raw const sTradeMonSpriteCoords).cast::<u8>().cast_mut())
                            .cast::<u8>())
                        .cast::<u8>())
                        .read()) as i32)
                            .wrapping_mul(8i32))
                        .wrapping_add(32i32)) as i16),
                        (((((((((&raw const sTradeMonSpriteCoords).cast::<u8>().cast_mut())
                            .cast::<u8>())
                        .cast::<u8>())
                        .wrapping_offset(1))
                        .read()) as i32)
                            .wrapping_mul(8i32)) as i16),
                        2u8,
                    ));
                ((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(53))
                    .write(0u8);
                let __p25 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p25).write(((__p25).read()).wrapping_add(1));
                rbox_fill_rectangle(0u8);
                break 'l1;
            }
            if __sw1 == 14i32 {
                __fall = true;
                ComputePartyTradeableFlags(0u8);
                PrintPartyNicknames(0u8);
                (((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read()).write(0u8);
                ((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1))
                    .write(0u8);
                SetActiveMenuOptions();
                let __p26 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p26).write(((__p26).read()).wrapping_add(1));
                PlayBGM(435u16);
                break 'l1;
            }
            if __sw1 == 15i32 {
                __fall = true;
                ComputePartyTradeableFlags(1u8);
                PrintPartyNicknames(1u8);
                let __p27 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p27).write(((__p27).read()).wrapping_add(1));
            }
            if __fall || __sw1 == 16i32 {
                __fall = true;
                LoadTradeBgGfx(0u8);
                let __p28 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p28).write(((__p28).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 17i32 {
                __fall = true;
                LoadTradeBgGfx(1u8);
                let __p29 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p29).write(((__p29).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 18i32 {
                __fall = true;
                BeginNormalPaletteFade(4294967295u32, 0i8, 16u8, 0u8, 0u16);
                let __p30 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p30).write(((__p30).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 19i32 {
                __fall = true;
                SetGpuReg(0u8, 4160u16);
                LoadTradeBgGfx(2u8);
                let __p31 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p31).write(((__p31).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 20i32 {
                __fall = true;
                ComputePartyHPBarLevels(0u8);
                let __p32 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p32).write(((__p32).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 21i32 {
                __fall = true;
                ComputePartyHPBarLevels(1u8);
                SetTradePartyHPBarSprites();
                let __p33 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p33).write(((__p33).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 22i32 {
                __fall = true;
                if !((crate::c::bf_read(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
                    7,
                    1,
                    false,
                ) as u16)
                    != 0)
                {
                    (((&raw mut gMain).cast::<u8>()).cast::<Option<unsafe extern "C" fn()>>())
                        .write(Some(CB1_UpdateLink));
                    SetMainCallback2(Some(CB2_TradeMenu));
                }
                break 'l1;
            }
        }
        RunTextPrinters();
        RunTasks();
        AnimateSprites();
        BuildOamBuffer();
        UpdatePaletteFade();
    }
}
pub(crate) unsafe extern "C" fn CB2_ReturnToTradeMenu() {
    unsafe {
        let mut i: i32 = 0i32;
        let mut temp = crate::ffi::Align4([0u8; 24]);
        let mut id: u8 = 0u8;
        let mut xPos: u32 = 0u32;
        'l1: {
            let __sw1 = (((((&raw mut gMain).cast::<u8>()).wrapping_add(1080)).read()) as i32);
            if __sw1 == 0i32 {
                InitTradeMenu();
                let __p2 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                let __p3 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p3).write(((__p3).read()).wrapping_add(1));
                ((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(168))
                    .write(0u8);
                break 'l1;
            }
            if __sw1 == 2i32 {
                let __p4 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p4).write(((__p4).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 3i32 {
                let __p5 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p5).write(((__p5).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 4i32 {
                CalculatePlayerPartyCount();
                let __p6 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p6).write(((__p6).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 5i32 {
                if (((&raw mut gWirelessCommType).cast::<u8>()).read()) != 0 {
                    LoadWirelessStatusIndicatorSpriteGfx();
                    CreateWirelessStatusIndicatorSprite(0u8, 0u8);
                }
                let __p7 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p7).write(((__p7).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 6i32 {
                let __p8 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p8).write(((__p8).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 7i32 {
                CalculateEnemyPartyCount();
                (((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(54))
                .cast::<u8>())
                .write(((&raw mut gPlayerPartyCount).cast::<u8>()).read());
                ((((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(54))
                .cast::<u8>())
                .wrapping_offset(1))
                .write(((&raw mut gEnemyPartyCount).cast::<u8>()).read());
                ClearWindowTilemap(0u8);
                PrintPartyNicknames(0u8);
                PrintPartyNicknames(1u8);
                {
                    i = 0i32;
                    'l2: loop {
                        if !(i
                            < (((((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(54))
                            .cast::<u8>())
                            .read()) as i32))
                        {
                            break 'l2;
                        }
                        'l3: {
                            let mut mon: *mut u8 = ((&raw mut gPlayerParty).cast::<u8>())
                                .wrapping_offset((i) as isize * 100);
                            (((((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(40))
                            .cast::<u8>())
                            .cast::<u8>())
                            .wrapping_offset((i) as isize))
                            .write(CreateMonIcon(
                                ((GetMonData3(mon, 65i32, core::ptr::null_mut())) as u16),
                                Some(SpriteCB_MonIcon),
                                ((((((((((&raw const sTradeMonSpriteCoords)
                                    .cast::<u8>()
                                    .cast_mut())
                                .cast::<u8>())
                                .wrapping_offset((i) as isize * 2))
                                .cast::<u8>())
                                .read()) as i32)
                                    .wrapping_mul(8i32))
                                .wrapping_add(14i32)) as i16),
                                (((((((((((&raw const sTradeMonSpriteCoords)
                                    .cast::<u8>()
                                    .cast_mut())
                                .cast::<u8>())
                                .wrapping_offset((i) as isize * 2))
                                .cast::<u8>())
                                .wrapping_offset(1))
                                .read()) as i32)
                                    .wrapping_mul(8i32))
                                .wrapping_sub(12i32)) as i16),
                                1u8,
                                GetMonData2(mon, 0i32),
                                1u32,
                            ));
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                {
                    i = 0i32;
                    'l4: loop {
                        if !(i
                            < ((((((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(54))
                            .cast::<u8>())
                            .wrapping_offset(1))
                            .read()) as i32))
                        {
                            break 'l4;
                        }
                        'l5: {
                            let mut mon: *mut u8 = ((&raw mut gEnemyParty).cast::<u8>())
                                .wrapping_offset((i) as isize * 100);
                            ((((((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(40))
                            .cast::<u8>())
                            .wrapping_offset(6))
                            .cast::<u8>())
                            .wrapping_offset((i) as isize))
                            .write(CreateMonIcon(
                                ((GetMonData3(mon, 65i32, core::ptr::null_mut())) as u16),
                                Some(SpriteCB_MonIcon),
                                ((((((((((&raw const sTradeMonSpriteCoords)
                                    .cast::<u8>()
                                    .cast_mut())
                                .cast::<u8>())
                                .wrapping_offset(((i).wrapping_add(6i32)) as isize * 2))
                                .cast::<u8>())
                                .read()) as i32)
                                    .wrapping_mul(8i32))
                                .wrapping_add(14i32)) as i16),
                                (((((((((((&raw const sTradeMonSpriteCoords)
                                    .cast::<u8>()
                                    .cast_mut())
                                .cast::<u8>())
                                .wrapping_offset(((i).wrapping_add(6i32)) as isize * 2))
                                .cast::<u8>())
                                .wrapping_offset(1))
                                .read()) as i32)
                                    .wrapping_mul(8i32))
                                .wrapping_sub(12i32)) as i16),
                                1u8,
                                GetMonData2(mon, 0i32),
                                0u32,
                            ));
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                let __p9 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p9).write(((__p9).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 8i32 {
                LoadHeldItemIcons();
                DrawHeldItemIconsForTrade(
                    ((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(54))
                    .cast::<u8>(),
                    (((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(40))
                    .cast::<u8>())
                    .cast::<u8>(),
                    0u8,
                );
                let __p10 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p10).write(((__p10).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 9i32 {
                DrawHeldItemIconsForTrade(
                    ((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(54))
                    .cast::<u8>(),
                    (((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(40))
                    .cast::<u8>())
                    .cast::<u8>(),
                    1u8,
                );
                let __p11 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p11).write(((__p11).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 10i32 {
                DrawTextWindowAndBufferTiles(
                    (((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).cast::<u8>(),
                    (((&raw mut sMenuTextTileBuffers)
                        .cast::<u8>()
                        .cast::<*mut u8>())
                    .cast::<*mut u8>())
                    .read(),
                    0u8,
                    0u8,
                    3i32,
                );
                id = GetMultiplayerId();
                DrawTextWindowAndBufferTiles(
                    ((((&raw mut gLinkPlayers).cast::<u8>())
                        .wrapping_offset((((id) as i32) ^ 1i32) as isize * 28))
                    .wrapping_add(8))
                    .cast::<u8>(),
                    ((((&raw mut sMenuTextTileBuffers)
                        .cast::<u8>()
                        .cast::<*mut u8>())
                    .cast::<*mut u8>())
                    .wrapping_offset(3))
                    .read(),
                    0u8,
                    0u8,
                    3i32,
                );
                DrawTextWindowAndBufferTiles(
                    (((&raw const sActionTexts)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<*mut u8>())
                    .cast::<*mut u8>())
                    .read(),
                    ((((&raw mut sMenuTextTileBuffers)
                        .cast::<u8>()
                        .cast::<*mut u8>())
                    .cast::<*mut u8>())
                    .wrapping_offset(6))
                    .read(),
                    0u8,
                    0u8,
                    2i32,
                );
                DrawBottomRowText(
                    ((((&raw const sActionTexts)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<*mut u8>())
                    .cast::<*mut u8>())
                    .wrapping_offset(1))
                    .read(),
                    ((((&raw mut sMenuTextTileBuffers)
                        .cast::<u8>()
                        .cast::<*mut u8>())
                    .cast::<*mut u8>())
                    .wrapping_offset(8))
                    .read(),
                    24u8,
                );
                let __p12 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p12).write(((__p12).read()).wrapping_add(1));
                ((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(168))
                    .write(0u8);
                break 'l1;
            }
            if __sw1 == 11i32 {
                if (LoadUISpriteGfx()) != 0 {
                    let __p13 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                    (__p13).write(((__p13).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 12i32 {
                xPos = ((GetStringCenterAlignXOffset(
                    1i32,
                    (((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).cast::<u8>(),
                    120i32,
                )) as u32);
                {
                    i = 0i32;
                    'l6: loop {
                        if !(i < 3i32) {
                            break 'l6;
                        }
                        'l7: {
                            (&raw mut temp)
                                .cast::<u8>()
                                .cast::<crate::c::Rec4<24>>()
                                .write_unaligned(
                                    (&raw const sSpriteTemplate_MenuText)
                                        .cast::<u8>()
                                        .cast_mut()
                                        .cast::<crate::c::Rec4<24>>()
                                        .read_unaligned(),
                                );
                            let __p14 = ((&raw mut temp).cast::<u8>()).cast::<u16>();
                            (__p14).write(
                                (((((__p14).read()) as i32).wrapping_add((i).wrapping_add(0i32)))
                                    as u16),
                            );
                            CreateSprite(
                                (&raw mut temp).cast::<u8>(),
                                ((((xPos).wrapping_add((((i).wrapping_mul(32i32)) as u32)))
                                    .wrapping_add(16u32)) as i16),
                                10i16,
                                1u8,
                            );
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                xPos = ((GetStringCenterAlignXOffset(
                    1i32,
                    ((((&raw mut gLinkPlayers).cast::<u8>())
                        .wrapping_offset((((GetMultiplayerId()) as i32) ^ 1i32) as isize * 28))
                    .wrapping_add(8))
                    .cast::<u8>(),
                    120i32,
                )) as u32);
                {
                    i = 0i32;
                    'l8: loop {
                        if !(i < 3i32) {
                            break 'l8;
                        }
                        'l9: {
                            (&raw mut temp)
                                .cast::<u8>()
                                .cast::<crate::c::Rec4<24>>()
                                .write_unaligned(
                                    (&raw const sSpriteTemplate_MenuText)
                                        .cast::<u8>()
                                        .cast_mut()
                                        .cast::<crate::c::Rec4<24>>()
                                        .read_unaligned(),
                                );
                            let __p15 = ((&raw mut temp).cast::<u8>()).cast::<u16>();
                            (__p15).write(
                                (((((__p15).read()) as i32).wrapping_add((i).wrapping_add(3i32)))
                                    as u16),
                            );
                            CreateSprite(
                                (&raw mut temp).cast::<u8>(),
                                ((((xPos).wrapping_add((((i).wrapping_mul(32i32)) as u32)))
                                    .wrapping_add(136u32)) as i16),
                                10i16,
                                1u8,
                            );
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                let __p16 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p16).write(((__p16).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 13i32 {
                (&raw mut temp)
                    .cast::<u8>()
                    .cast::<crate::c::Rec4<24>>()
                    .write_unaligned(
                        (&raw const sSpriteTemplate_MenuText)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<crate::c::Rec4<24>>()
                            .read_unaligned(),
                    );
                let __p17 = ((&raw mut temp).cast::<u8>()).cast::<u16>();
                (__p17).write((((((__p17).read()) as i32).wrapping_add(6i32)) as u16));
                CreateSprite((&raw mut temp).cast::<u8>(), 215i16, 152i16, 1u8);
                (&raw mut temp)
                    .cast::<u8>()
                    .cast::<crate::c::Rec4<24>>()
                    .write_unaligned(
                        (&raw const sSpriteTemplate_MenuText)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<crate::c::Rec4<24>>()
                            .read_unaligned(),
                    );
                let __p18 = ((&raw mut temp).cast::<u8>()).cast::<u16>();
                (__p18).write((((((__p18).read()) as i32).wrapping_add(7i32)) as u16));
                CreateSprite((&raw mut temp).cast::<u8>(), 247i16, 152i16, 1u8);
                {
                    i = 0i32;
                    'l10: loop {
                        if !(i < 6i32) {
                            break 'l10;
                        }
                        'l11: {
                            (&raw mut temp)
                                .cast::<u8>()
                                .cast::<crate::c::Rec4<24>>()
                                .write_unaligned(
                                    (&raw const sSpriteTemplate_MenuText)
                                        .cast::<u8>()
                                        .cast_mut()
                                        .cast::<crate::c::Rec4<24>>()
                                        .read_unaligned(),
                                );
                            let __p19 = ((&raw mut temp).cast::<u8>()).cast::<u16>();
                            (__p19).write(
                                (((((__p19).read()) as i32).wrapping_add((i).wrapping_add(8i32)))
                                    as u16),
                            );
                            CreateSprite(
                                (&raw mut temp).cast::<u8>(),
                                ((((i).wrapping_mul(32i32)).wrapping_add(24i32)) as i16),
                                150i16,
                                1u8,
                            );
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                if ((((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(53))
                .read()) as i32)
                    < 6i32
                {
                    ((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(53))
                    .write(((&raw mut gLastViewedMonIndex).cast::<u8>()).read());
                } else {
                    ((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(53))
                    .write(
                        ((((((&raw mut gLastViewedMonIndex).cast::<u8>()).read()) as i32)
                            .wrapping_add(6i32)) as u8),
                    );
                }
                ((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(52))
                    .write(CreateSprite(
                        (&raw const sSpriteTemplate_Cursor).cast::<u8>().cast_mut(),
                        ((((((((((&raw const sTradeMonSpriteCoords).cast::<u8>().cast_mut())
                            .cast::<u8>())
                        .wrapping_offset(
                            ((((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(53))
                            .read()) as i32) as isize
                                * 2,
                        ))
                        .cast::<u8>())
                        .read()) as i32)
                            .wrapping_mul(8i32))
                        .wrapping_add(32i32)) as i16),
                        ((((((((((&raw const sTradeMonSpriteCoords).cast::<u8>().cast_mut())
                            .cast::<u8>())
                        .wrapping_offset(
                            ((((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(53))
                            .read()) as i32) as isize
                                * 2,
                        ))
                        .cast::<u8>())
                        .wrapping_offset(1))
                        .read()) as i32)
                            .wrapping_mul(8i32)) as i16),
                        2u8,
                    ));
                (((&raw mut gMain).cast::<u8>()).wrapping_add(1080)).write(16u8);
                break 'l1;
            }
            if __sw1 == 16i32 {
                LoadTradeBgGfx(0u8);
                let __p20 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p20).write(((__p20).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 17i32 {
                LoadTradeBgGfx(1u8);
                (((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read()).write(0u8);
                ((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1))
                    .write(0u8);
                SetActiveMenuOptions();
                let __p21 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p21).write(((__p21).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 18i32 {
                crate::c::bf_write(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(8),
                    7,
                    1,
                    (0u16) as i32,
                );
                BlendPalettes(4294967295u32, 16u8, 0u16);
                BeginNormalPaletteFade(4294967295u32, 0i8, 16u8, 0u8, 0u16);
                let __p22 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p22).write(((__p22).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 19i32 {
                SetGpuReg(0u8, 4160u16);
                LoadTradeBgGfx(2u8);
                let __p23 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p23).write(((__p23).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 20i32 {
                let __p24 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p24).write(((__p24).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 21i32 {
                SetTradePartyHPBarSprites();
                let __p25 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p25).write(((__p25).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 22i32 {
                if !((crate::c::bf_read(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
                    7,
                    1,
                    false,
                ) as u16)
                    != 0)
                {
                    SetMainCallback2(Some(CB2_TradeMenu));
                }
                break 'l1;
            }
        }
        RunTasks();
        AnimateSprites();
        BuildOamBuffer();
        UpdatePaletteFade();
    }
}
pub(crate) unsafe extern "C" fn VBlankCB_TradeMenu() {
    unsafe {
        LoadOam();
        ProcessSpriteCopyRequests();
        TransferPlttBuffer();
    }
}
pub(crate) unsafe extern "C" fn CB_FadeToStartTrade() {
    unsafe {
        if (({
            let __p1 =
                (((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(168);
            let __t2 = ((__p1).read()).wrapping_add(1);
            (__p1).write(__t2);
            __t2
        }) as i32)
            > 15i32
        {
            BeginNormalPaletteFade(4294967295u32, 0i8, 0u8, 16u8, 0u16);
            ((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(111))
                .write(10u8);
        }
    }
}
pub(crate) unsafe extern "C" fn CB_WaitToStartTrade() {
    unsafe {
        if !((crate::c::bf_read(
            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
            7,
            1,
            false,
        ) as u16)
            != 0)
        {
            (((&raw mut gSelectedTradeMonPositions).cast::<u8>()).cast::<u8>()).write(
                ((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(53))
                    .read(),
            );
            ((((&raw mut gSelectedTradeMonPositions).cast::<u8>()).cast::<u8>())
                .wrapping_offset(1))
            .write(
                ((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(126))
                    .read(),
            );
            if (((&raw mut gWirelessCommType).cast::<u8>()).read()) != 0 {
                ((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(111))
                    .write(16u8);
            } else {
                SetCloseLinkCallbackAndType(32u16);
                ((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(111))
                    .write(13u8);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn CB_StartLinkTrade() {
    unsafe {
        (((&raw mut gMain).cast::<u8>())
            .wrapping_add(8)
            .cast::<Option<unsafe extern "C" fn()>>())
        .write(Some(CB2_StartCreateTradeMenu));
        if (((&raw mut gWirelessCommType).cast::<u8>()).read()) != 0 {
            if (IsLinkRfuTaskFinished()) != 0 {
                Free(
                    ((&raw mut sMenuTextTileBuffer)
                        .cast::<u8>()
                        .cast::<*mut u8>())
                    .read(),
                );
                FreeAllWindowBuffers();
                Free(((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read());
                (((&raw mut gMain).cast::<u8>()).cast::<Option<unsafe extern "C" fn()>>())
                    .write(None);
                DestroyWirelessStatusIndicatorSprite();
                SetMainCallback2(Some(CB2_LinkTrade));
            }
        } else {
            if !((((&raw mut gReceivedRemoteLinkPlayers).cast::<u8>()).read()) != 0) {
                Free(
                    ((&raw mut sMenuTextTileBuffer)
                        .cast::<u8>()
                        .cast::<*mut u8>())
                    .read(),
                );
                FreeAllWindowBuffers();
                Free(((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read());
                (((&raw mut gMain).cast::<u8>()).cast::<Option<unsafe extern "C" fn()>>())
                    .write(None);
                SetMainCallback2(Some(CB2_LinkTrade));
            }
        }
    }
}
pub(crate) unsafe extern "C" fn CB2_TradeMenu() {
    unsafe {
        RunTradeMenuCallback();
        DoQueuedActions();
        DrawSelectedMonScreen(0u8);
        DrawSelectedMonScreen(1u8);
        SetGpuReg(
            24u8,
            (({
                let __p1 = (((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read());
                let __t2 = (__p1).read();
                (__p1).write(((__p1).read()).wrapping_add(1));
                __t2
            }) as u16),
        );
        SetGpuReg(
            28u8,
            (({
                let __p3 =
                    (((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1);
                let __t4 = (__p3).read();
                (__p3).write(((__p3).read()).wrapping_sub(1));
                __t4
            }) as u16),
        );
        RunTextPrintersAndIsPrinter0Active();
        RunTasks();
        AnimateSprites();
        BuildOamBuffer();
        UpdatePaletteFade();
    }
}
pub(crate) unsafe extern "C" fn LoadTradeBgGfx(state: u8) {
    unsafe {
        let mut state = state;
        let mut i: i32 = 0i32;
        'l1: {
            let __sw1 = ((state) as i32);
            if __sw1 == 0i32 {
                LoadPalette(
                    (((&raw mut gTradeMenu_Pal).cast::<u16>()).cast::<u16>()).cast::<u8>(),
                    0u16,
                    96u16,
                );
                LoadBgTiles(1u8, (&raw mut gTradeMenu_Gfx).cast::<u8>(), 4736u16, 0u16);
                CopyToBgTilemapBufferRect_ChangePalette(
                    1u8,
                    (((&raw mut gTradeMenu_Tilemap).cast::<u16>()).cast::<u16>()).cast::<u8>(),
                    0u8,
                    0u8,
                    32u8,
                    20u8,
                    0u8,
                );
                LoadBgTilemap(
                    2u8,
                    ((&raw const sTradeStripesBG2Tilemap).cast::<u8>().cast_mut()).cast::<u8>(),
                    2048u16,
                    0u16,
                );
                break 'l1;
            }
            if __sw1 == 1i32 {
                LoadBgTilemap(
                    3u8,
                    ((&raw const sTradeStripesBG3Tilemap).cast::<u8>().cast_mut()).cast::<u8>(),
                    2048u16,
                    0u16,
                );
                PrintPartyLevelsAndGenders(0u8);
                PrintPartyLevelsAndGenders(1u8);
                CopyBgTilemapBufferToVram(1u8);
                break 'l1;
            }
            if __sw1 == 2i32 {
                {
                    i = 0i32;
                    'l2: loop {
                        if !(i < 4i32) {
                            break 'l2;
                        }
                        'l3: {
                            SetGpuReg((((16i32).wrapping_add((i).wrapping_mul(2i32))) as u8), 0u16);
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                ShowBg(0u8);
                ShowBg(1u8);
                ShowBg(2u8);
                ShowBg(3u8);
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SetActiveMenuOptions() {
    unsafe {
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 6i32) {
                    break 'l1;
                }
                'l2: {
                    if i < (((((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(54))
                    .cast::<u8>())
                    .read()) as i32)
                    {
                        crate::c::bf_write(
                            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                (((((((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(40))
                                .cast::<u8>())
                                .cast::<u8>())
                                .wrapping_offset((i) as isize))
                                .read()) as i32) as isize
                                    * 68,
                            ))
                            .wrapping_add(62),
                            2,
                            1,
                            (0u16) as i32,
                        );
                        ((((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(56))
                        .cast::<u8>())
                        .wrapping_offset((i) as isize))
                        .write(1u8);
                    } else {
                        ((((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(56))
                        .cast::<u8>())
                        .wrapping_offset((i) as isize))
                        .write(0u8);
                    }
                    if i < ((((((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(54))
                    .cast::<u8>())
                    .wrapping_offset(1))
                    .read()) as i32)
                    {
                        crate::c::bf_write(
                            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                ((((((((((&raw mut sTradeMenu)
                                    .cast::<u8>()
                                    .cast::<*mut u8>())
                                .read())
                                .wrapping_add(40))
                                .cast::<u8>())
                                .wrapping_offset(6))
                                .cast::<u8>())
                                .wrapping_offset((i) as isize))
                                .read()) as i32) as isize
                                    * 68,
                            ))
                            .wrapping_add(62),
                            2,
                            1,
                            (0u16) as i32,
                        );
                        ((((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(56))
                        .cast::<u8>())
                        .wrapping_offset(((i).wrapping_add(6i32)) as isize))
                        .write(1u8);
                    } else {
                        ((((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(56))
                        .cast::<u8>())
                        .wrapping_offset(((i).wrapping_add(6i32)) as isize))
                        .write(0u8);
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        ((((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(56))
            .cast::<u8>())
        .wrapping_offset(12))
        .write(1u8);
    }
}
pub(crate) unsafe extern "C" fn Trade_Memcpy(dest: *mut u8, src: *mut u8, size: u32) {
    unsafe {
        let mut dest = dest;
        let mut src = src;
        let mut size = size;
        let mut _dest: *mut u8 = dest;
        let mut _src: *mut u8 = src;
        let mut i: u32 = 0u32;
        {
            i = 0u32;
            'l1: loop {
                if !(i < size) {
                    break 'l1;
                }
                'l2: {
                    ((_dest).wrapping_offset(((i) as i32) as isize))
                        .write(((_src).wrapping_offset(((i) as i32) as isize)).read());
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn BufferTradeParties() -> u8 {
    unsafe {
        let mut id: u8 = GetMultiplayerId();
        let mut i: i32 = 0i32;
        let mut mon: *mut u8 = core::ptr::null_mut();
        'l1: {
            let __sw1 = ((((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(105))
            .read()) as i32);
            if __sw1 == 0i32 {
                Trade_Memcpy(
                    (&raw mut gBlockSendBuffer).cast::<u8>(),
                    (&raw mut gPlayerParty).cast::<u8>(),
                    200u32,
                );
                let __p2 = (((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(105);
                (__p2).write(((__p2).read()).wrapping_add(1));
                ((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(168))
                    .write(0u8);
                break 'l1;
            }
            if __sw1 == 1i32 {
                if (IsLinkTradeTaskFinished()) != 0 {
                    if _GetBlockReceivedStatus() == 0u32 {
                        let __p3 = (((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(105);
                        (__p3).write(((__p3).read()).wrapping_add(1));
                    } else {
                        TradeResetReceivedFlags();
                        let __p4 = (((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(105);
                        (__p4).write(((__p4).read()).wrapping_add(1));
                    }
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                if ((id) as i32) == 0i32 {
                    RequestLinkData(1u8);
                }
                let __p5 = (((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(105);
                (__p5).write(((__p5).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 4i32 {
                if _GetBlockReceivedStatus() == 3u32 {
                    Trade_Memcpy(
                        (&raw mut gEnemyParty).cast::<u8>(),
                        ((((&raw mut gBlockRecvBuffer).cast::<u8>())
                            .wrapping_offset((((id) as i32) ^ 1i32) as isize * 256))
                        .cast::<u16>())
                        .cast::<u8>(),
                        200u32,
                    );
                    TradeResetReceivedFlags();
                    let __p6 = (((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(105);
                    (__p6).write(((__p6).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 5i32 {
                Trade_Memcpy(
                    (&raw mut gBlockSendBuffer).cast::<u8>(),
                    ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(200),
                    200u32,
                );
                let __p7 = (((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(105);
                (__p7).write(((__p7).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 7i32 {
                if ((id) as i32) == 0i32 {
                    RequestLinkData(1u8);
                }
                let __p8 = (((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(105);
                (__p8).write(((__p8).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 8i32 {
                if _GetBlockReceivedStatus() == 3u32 {
                    Trade_Memcpy(
                        ((&raw mut gEnemyParty).cast::<u8>()).wrapping_offset(200),
                        ((((&raw mut gBlockRecvBuffer).cast::<u8>())
                            .wrapping_offset((((id) as i32) ^ 1i32) as isize * 256))
                        .cast::<u16>())
                        .cast::<u8>(),
                        200u32,
                    );
                    TradeResetReceivedFlags();
                    let __p9 = (((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(105);
                    (__p9).write(((__p9).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 9i32 {
                Trade_Memcpy(
                    (&raw mut gBlockSendBuffer).cast::<u8>(),
                    ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(400),
                    200u32,
                );
                let __p10 = (((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(105);
                (__p10).write(((__p10).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 11i32 {
                if ((id) as i32) == 0i32 {
                    RequestLinkData(1u8);
                }
                let __p11 = (((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(105);
                (__p11).write(((__p11).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 12i32 {
                if _GetBlockReceivedStatus() == 3u32 {
                    Trade_Memcpy(
                        ((&raw mut gEnemyParty).cast::<u8>()).wrapping_offset(400),
                        ((((&raw mut gBlockRecvBuffer).cast::<u8>())
                            .wrapping_offset((((id) as i32) ^ 1i32) as isize * 256))
                        .cast::<u16>())
                        .cast::<u8>(),
                        200u32,
                    );
                    TradeResetReceivedFlags();
                    let __p12 = (((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(105);
                    (__p12).write(((__p12).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 13i32 {
                Trade_Memcpy(
                    (&raw mut gBlockSendBuffer).cast::<u8>(),
                    ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(11232))
                        .cast::<u8>(),
                    220u32,
                );
                let __p13 = (((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(105);
                (__p13).write(((__p13).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 15i32 {
                if ((id) as i32) == 0i32 {
                    RequestLinkData(3u8);
                }
                let __p14 = (((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(105);
                (__p14).write(((__p14).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 16i32 {
                if _GetBlockReceivedStatus() == 3u32 {
                    Trade_Memcpy(
                        ((&raw mut gTradeMail).cast::<u8>()).cast::<u8>(),
                        ((((&raw mut gBlockRecvBuffer).cast::<u8>())
                            .wrapping_offset((((id) as i32) ^ 1i32) as isize * 256))
                        .cast::<u16>())
                        .cast::<u8>(),
                        216u32,
                    );
                    TradeResetReceivedFlags();
                    let __p15 = (((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(105);
                    (__p15).write(((__p15).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 17i32 {
                Trade_Memcpy(
                    (&raw mut gBlockSendBuffer).cast::<u8>(),
                    ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(12712))
                        .cast::<u8>(),
                    11u32,
                );
                let __p16 = (((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(105);
                (__p16).write(((__p16).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 19i32 {
                if ((id) as i32) == 0i32 {
                    RequestLinkData(4u8);
                }
                let __p17 = (((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(105);
                (__p17).write(((__p17).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 20i32 {
                if _GetBlockReceivedStatus() == 3u32 {
                    Trade_Memcpy(
                        ((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(169))
                        .cast::<u8>(),
                        ((((&raw mut gBlockRecvBuffer).cast::<u8>())
                            .wrapping_offset((((id) as i32) ^ 1i32) as isize * 256))
                        .cast::<u16>())
                        .cast::<u8>(),
                        11u32,
                    );
                    TradeResetReceivedFlags();
                    let __p18 = (((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(105);
                    (__p18).write(((__p18).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 21i32 {
                {
                    i = 0i32;
                    mon = (&raw mut gEnemyParty).cast::<u8>();
                    'l2: loop {
                        if !(i < 6i32) {
                            break 'l2;
                        }
                        'l3: {
                            let mut name = crate::ffi::Align4([0u8; 11]);
                            let mut species: u16 = ((GetMonData2(mon, 11i32)) as u16);
                            if ((species) as i32) != 0i32 {
                                if (((species) as i32) == 303i32)
                                    && (GetMonData2(mon, 3i32) != 1u32)
                                {
                                    GetMonData3(mon, 2i32, (&raw mut name).cast::<u8>());
                                    if !((StringCompareWithoutExtCtrlCodes(
                                        (&raw mut name).cast::<u8>(),
                                        ((&raw const sText_ShedinjaJP).cast::<u8>().cast_mut())
                                            .cast::<u8>(),
                                    )) != 0)
                                    {
                                        SetMonData(
                                            mon,
                                            2i32,
                                            (((&raw mut gSpeciesNames).cast::<u8>())
                                                .wrapping_offset(3333))
                                            .cast::<u8>(),
                                        );
                                    }
                                }
                            }
                        }
                        mon = (mon).wrapping_offset(100);
                        i = (i).wrapping_add(1);
                    }
                }
                return 1u8;
            }
            if __sw1 == 2i32 || __sw1 == 6i32 || __sw1 == 10i32 || __sw1 == 14i32 || __sw1 == 18i32
            {
                let __p19 = (((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(168);
                (__p19).write(((__p19).read()).wrapping_add(1));
                if ((((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(168))
                .read()) as i32)
                    > 10i32
                {
                    ((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(168))
                    .write(0u8);
                    let __p20 = (((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(105);
                    (__p20).write(((__p20).read()).wrapping_add(1));
                }
                break 'l1;
            }
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn PrintIsThisTradeOkay() {
    unsafe {
        DrawBottomRowText(
            ((&raw const sText_IsThisTradeOkay).cast::<u8>().cast_mut()).cast::<u8>(),
            (((100728832i32).wrapping_add(
                ((((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(114)
                    .cast::<u16>())
                .read()) as i32)
                    .wrapping_mul(32i32),
            )) as usize as *mut u8),
            24u8,
        );
    }
}
pub(crate) unsafe extern "C" fn Leader_ReadLinkBuffer(mpId: u8, status: u8) {
    unsafe {
        let mut mpId = mpId;
        let mut status = status;
        if (((status) as i32) & 1i32) != 0 {
            'l1: {
                let __sw1 =
                    (((((&raw mut gBlockRecvBuffer).cast::<u8>()).cast::<u16>()).read()) as i32);
                if __sw1 == 61098i32 {
                    ((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(120))
                    .write(2u8);
                    break 'l1;
                }
                if __sw1 == 43707i32 {
                    ((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(120))
                    .write(1u8);
                    break 'l1;
                }
                if __sw1 == 48059i32 {
                    ((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(122))
                    .write(1u8);
                    break 'l1;
                }
                if __sw1 == 48076i32 {
                    ((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(122))
                    .write(2u8);
                    break 'l1;
                }
            }
            TradeResetReceivedFlag(0u32);
        }
        if (((status) as i32) & 2i32) != 0 {
            'l2: {
                let __sw2 = ((((((&raw mut gBlockRecvBuffer).cast::<u8>()).wrapping_offset(256))
                    .cast::<u16>())
                .read()) as i32);
                if __sw2 == 61098i32 {
                    ((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(121))
                    .write(2u8);
                    break 'l2;
                }
                if __sw2 == 43707i32 {
                    ((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(126))
                    .write(
                        (((((((((&raw mut gBlockRecvBuffer).cast::<u8>()).wrapping_offset(256))
                            .cast::<u16>())
                        .wrapping_offset(1))
                        .read()) as i32)
                            .wrapping_add(6i32)) as u8),
                    );
                    ((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(121))
                    .write(1u8);
                    break 'l2;
                }
                if __sw2 == 48059i32 {
                    ((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(123))
                    .write(1u8);
                    break 'l2;
                }
                if __sw2 == 48076i32 {
                    ((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(123))
                    .write(2u8);
                    break 'l2;
                }
            }
            TradeResetReceivedFlag(1u32);
        }
    }
}
pub(crate) unsafe extern "C" fn Follower_ReadLinkBuffer(mpId: u8, status: u8) {
    unsafe {
        let mut mpId = mpId;
        let mut status = status;
        if (((status) as i32) & 1i32) != 0 {
            'l1: {
                let __sw1 =
                    (((((&raw mut gBlockRecvBuffer).cast::<u8>()).cast::<u16>()).read()) as i32);
                if __sw1 == 61115i32 {
                    BeginNormalPaletteFade(4294967295u32, 0i8, 0u8, 16u8, 0u16);
                    PrintTradeMessage(4u8);
                    ((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(111))
                    .write(11u8);
                    break 'l1;
                }
                if __sw1 == 61132i32 {
                    PrintTradeMessage(5u8);
                    ((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(111))
                    .write(8u8);
                    break 'l1;
                }
                if __sw1 == 56797i32 {
                    ((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(126))
                    .write(
                        ((((((((&raw mut gBlockRecvBuffer).cast::<u8>()).cast::<u16>())
                            .wrapping_offset(1))
                        .read()) as i32)
                            .wrapping_add(6i32)) as u8),
                    );
                    rbox_fill_rectangle(0u8);
                    SetSelectedMon(
                        ((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(53))
                        .read(),
                    );
                    SetSelectedMon(
                        ((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(126))
                        .read(),
                    );
                    ((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(111))
                    .write(7u8);
                    break 'l1;
                }
                if __sw1 == 52445i32 {
                    BeginNormalPaletteFade(4294967295u32, 0i8, 0u8, 16u8, 0u16);
                    ((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(111))
                    .write(10u8);
                    break 'l1;
                }
                if __sw1 == 56814i32 {
                    PrintTradeMessage(1u8);
                    ((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(111))
                    .write(8u8);
                }
            }
            TradeResetReceivedFlag(0u32);
        }
        if (((status) as i32) & 2i32) != 0 {
            TradeResetReceivedFlag(1u32);
        }
    }
}
pub(crate) unsafe extern "C" fn Leader_HandleCommunication() {
    unsafe {
        if (((((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(120))
            .read()) as i32)
            != 0i32)
            && (((((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(121))
            .read()) as i32)
                != 0i32)
        {
            if (((((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(120))
            .read()) as i32)
                == 1i32)
                && (((((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(121))
                .read()) as i32)
                    == 1i32)
            {
                ((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(111))
                    .write(6u8);
                (((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(128))
                .cast::<u16>())
                .write(56797u16);
                ((((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(128))
                .cast::<u16>())
                .wrapping_offset(1))
                .write(
                    ((((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(53))
                    .read()) as u16),
                );
                QueueAction(5u16, 0u8);
                ((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(120))
                    .write({
                        let __v1 = 0u8;
                        ((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(121))
                        .write(__v1);
                        __v1
                    });
            } else {
                if (((((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(120))
                .read()) as i32)
                    == 1i32)
                    && (((((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(121))
                    .read()) as i32)
                        == 2i32)
                {
                    PrintTradeMessage(1u8);
                    (((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(128))
                    .cast::<u16>())
                    .write(61132u16);
                    ((((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(128))
                    .cast::<u16>())
                    .wrapping_offset(1))
                    .write(0u16);
                    QueueAction(5u16, 0u8);
                    ((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(122))
                    .write({
                        let __v2 = 0u8;
                        ((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(123))
                        .write(__v2);
                        __v2
                    });
                    ((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(120))
                    .write({
                        let __v3 = 0u8;
                        ((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(121))
                        .write(__v3);
                        __v3
                    });
                    ((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(111))
                    .write(8u8);
                } else {
                    if (((((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(120))
                    .read()) as i32)
                        == 2i32)
                        && (((((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(121))
                        .read()) as i32)
                            == 1i32)
                    {
                        PrintTradeMessage(5u8);
                        (((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(128))
                        .cast::<u16>())
                        .write(56814u16);
                        ((((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(128))
                        .cast::<u16>())
                        .wrapping_offset(1))
                        .write(0u16);
                        QueueAction(5u16, 0u8);
                        ((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(122))
                        .write({
                            let __v4 = 0u8;
                            ((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(123))
                            .write(__v4);
                            __v4
                        });
                        ((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(120))
                        .write({
                            let __v5 = 0u8;
                            ((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(121))
                            .write(__v5);
                            __v5
                        });
                        ((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(111))
                        .write(8u8);
                    } else {
                        if (((((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(120))
                        .read()) as i32)
                            == 2i32)
                            && (((((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(121))
                            .read()) as i32)
                                == 2i32)
                        {
                            (((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(128))
                            .cast::<u16>())
                            .write(61115u16);
                            ((((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(128))
                            .cast::<u16>())
                            .wrapping_offset(1))
                            .write(0u16);
                            QueueAction(5u16, 0u8);
                            BeginNormalPaletteFade(4294967295u32, 0i8, 0u8, 16u8, 0u16);
                            ((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(120))
                            .write({
                                let __v6 = 0u8;
                                ((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                                    .wrapping_add(121))
                                .write(__v6);
                                __v6
                            });
                            ((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(111))
                            .write(11u8);
                        }
                    }
                }
            }
        }
        if (((((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(122))
            .read()) as i32)
            != 0i32)
            && (((((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(123))
            .read()) as i32)
                != 0i32)
        {
            if (((((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(122))
            .read()) as i32)
                == 1i32)
                && (((((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(123))
                .read()) as i32)
                    == 1i32)
            {
                (((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(128))
                .cast::<u16>())
                .write(52445u16);
                ((((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(128))
                .cast::<u16>())
                .wrapping_offset(1))
                .write(0u16);
                QueueAction(5u16, 0u8);
                ((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(122))
                    .write(0u8);
                ((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(123))
                    .write(0u8);
                ((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(111))
                    .write(9u8);
            }
            if (((((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(122))
            .read()) as i32)
                == 2i32)
                || (((((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(123))
                .read()) as i32)
                    == 2i32)
            {
                PrintTradeMessage(1u8);
                (((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(128))
                .cast::<u16>())
                .write(56814u16);
                ((((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(128))
                .cast::<u16>())
                .wrapping_offset(1))
                .write(0u16);
                QueueAction(5u16, 0u8);
                ((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(122))
                    .write(0u8);
                ((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(123))
                    .write(0u8);
                ((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(111))
                    .write(8u8);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn _SetLinkData(
    linkData: *mut u16,
    linkCmd: u16,
    cursorPosition: u16,
) {
    unsafe {
        let mut linkData = linkData;
        let mut linkCmd = linkCmd;
        let mut cursorPosition = cursorPosition;
        (linkData).write(linkCmd);
        ((linkData).wrapping_offset(1)).write(cursorPosition);
        QueueAction(5u16, 0u8);
    }
}
pub(crate) unsafe extern "C" fn SetLinkData(linkCmd: u16, cursorPosition: u16) {
    unsafe {
        let mut linkCmd = linkCmd;
        let mut cursorPosition = cursorPosition;
        _SetLinkData(
            ((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(128))
                .cast::<u16>(),
            linkCmd,
            cursorPosition,
        );
    }
}
pub(crate) unsafe extern "C" fn CB1_UpdateLink() {
    unsafe {
        let mut mpId: u8 = GetMultiplayerId();
        let mut status: u8 = 0u8;
        if ({
            let __v1 = ((_GetBlockReceivedStatus()) as u8);
            status = __v1;
            __v1
        }) != 0
        {
            if ((mpId) as i32) == 0i32 {
                Leader_ReadLinkBuffer(mpId, status);
            } else {
                Follower_ReadLinkBuffer(mpId, status);
            }
        }
        if ((mpId) as i32) == 0i32 {
            Leader_HandleCommunication();
        }
    }
}
pub(crate) unsafe extern "C" fn GetNewCursorPosition(oldPosition: u8, direction: u8) -> u8 {
    unsafe {
        let mut oldPosition = oldPosition;
        let mut direction = direction;
        let mut i: i32 = 0i32;
        let mut newPosition: u8 = 0u8;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 6i32) {
                    break 'l1;
                }
                'l2: {
                    if ((((((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(56))
                    .cast::<u8>())
                    .wrapping_offset(
                        ((((((((((&raw const sCursorMoveDestinations)
                            .cast::<u8>()
                            .cast_mut())
                        .cast::<u8>())
                        .wrapping_offset(((oldPosition) as i32) as isize * 24))
                        .cast::<u8>())
                        .wrapping_offset(((direction) as i32) as isize * 6))
                        .cast::<u8>())
                        .wrapping_offset((i) as isize))
                        .read()) as i32) as isize,
                    ))
                    .read()) as i32)
                        == 1i32
                    {
                        newPosition = ((((((((&raw const sCursorMoveDestinations)
                            .cast::<u8>()
                            .cast_mut())
                        .cast::<u8>())
                        .wrapping_offset(((oldPosition) as i32) as isize * 24))
                        .cast::<u8>())
                        .wrapping_offset(((direction) as i32) as isize * 6))
                        .cast::<u8>())
                        .wrapping_offset((i) as isize))
                        .read();
                        break 'l1;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return newPosition;
    }
}
pub(crate) unsafe extern "C" fn TradeMenuMoveCursor(cursorPosition: *mut u8, direction: u8) {
    unsafe {
        let mut cursorPosition = cursorPosition;
        let mut direction = direction;
        let mut newPosition: u8 = GetNewCursorPosition((cursorPosition).read(), direction);
        if ((newPosition) as i32) == 12i32 {
            StartSpriteAnim(
                ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(52))
                    .read()) as i32) as isize
                        * 68,
                ),
                1u8,
            );
            ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(52))
                .read()) as i32) as isize
                    * 68,
            ))
            .wrapping_add(32)
            .cast::<i16>())
            .write(224i16);
            ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(52))
                .read()) as i32) as isize
                    * 68,
            ))
            .wrapping_add(34)
            .cast::<i16>())
            .write(160i16);
        } else {
            StartSpriteAnim(
                ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(52))
                    .read()) as i32) as isize
                        * 68,
                ),
                0u8,
            );
            ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(52))
                .read()) as i32) as isize
                    * 68,
            ))
            .wrapping_add(32)
            .cast::<i16>())
            .write(
                ((((((((((&raw const sTradeMonSpriteCoords).cast::<u8>().cast_mut())
                    .cast::<u8>())
                .wrapping_offset(((newPosition) as i32) as isize * 2))
                .cast::<u8>())
                .read()) as i32)
                    .wrapping_mul(8i32))
                .wrapping_add(32i32)) as i16),
            );
            ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(52))
                .read()) as i32) as isize
                    * 68,
            ))
            .wrapping_add(34)
            .cast::<i16>())
            .write(
                ((((((((((&raw const sTradeMonSpriteCoords).cast::<u8>().cast_mut())
                    .cast::<u8>())
                .wrapping_offset(((newPosition) as i32) as isize * 2))
                .cast::<u8>())
                .wrapping_offset(1))
                .read()) as i32)
                    .wrapping_mul(8i32)) as i16),
            );
        }
        if (((cursorPosition).read()) as i32) != ((newPosition) as i32) {
            PlaySE(5u16);
        }
        (cursorPosition).write(newPosition);
    }
}
pub(crate) unsafe extern "C" fn SetReadyToTrade() {
    unsafe {
        PrintTradeMessage(0u8);
        ((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(111))
            .write(100u8);
        if ((GetMultiplayerId()) as i32) == 1i32 {
            SetLinkData(
                43707u16,
                ((((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(53))
                .read()) as u16),
            );
        } else {
            ((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(120))
                .write(1u8);
        }
    }
}
pub(crate) unsafe extern "C" fn CB_ProcessMenuInput() {
    unsafe {
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(48)
            .cast::<u16>())
        .read()) as i32)
            & 64i32)
            != 0
        {
            TradeMenuMoveCursor(
                (((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(53),
                0u8,
            );
        } else {
            if ((((((&raw mut gMain).cast::<u8>())
                .wrapping_add(48)
                .cast::<u16>())
            .read()) as i32)
                & 128i32)
                != 0
            {
                TradeMenuMoveCursor(
                    (((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(53),
                    1u8,
                );
            } else {
                if ((((((&raw mut gMain).cast::<u8>())
                    .wrapping_add(48)
                    .cast::<u16>())
                .read()) as i32)
                    & 32i32)
                    != 0
                {
                    TradeMenuMoveCursor(
                        (((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(53),
                        2u8,
                    );
                } else {
                    if ((((((&raw mut gMain).cast::<u8>())
                        .wrapping_add(48)
                        .cast::<u16>())
                    .read()) as i32)
                        & 16i32)
                        != 0
                    {
                        TradeMenuMoveCursor(
                            (((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(53),
                            3u8,
                        );
                    }
                }
            }
        }
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(46)
            .cast::<u16>())
        .read()) as i32)
            & 1i32)
            != 0
        {
            PlaySE(5u16);
            if ((((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(53))
                .read()) as i32)
                < 6i32
            {
                DrawTextBorderOuter(1u8, 1u16, 14u8);
                FillWindowPixelBuffer(1u8, 17u8);
                PrintMenuTable(
                    1u8,
                    ((crate::c::div_u32(16u32, 8u32)) as u8),
                    ((&raw const sSelectTradeMonActions).cast::<u8>().cast_mut()).cast::<u8>(),
                );
                InitMenuInUpperLeftCornerNormal(1u8, ((crate::c::div_u32(16u32, 8u32)) as u8), 0u8);
                PutWindowTilemap(1u8);
                CopyWindowToVram(1u8, 3u8);
                ((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(111))
                    .write(1u8);
            } else {
                if ((((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(53))
                .read()) as i32)
                    < 12i32
                {
                    BeginNormalPaletteFade(4294967295u32, 0i8, 0u8, 16u8, 0u16);
                    ((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(111))
                    .write(2u8);
                } else {
                    if ((((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(53))
                    .read()) as i32)
                        == 12i32
                    {
                        CreateYesNoMenu(
                            (&raw const sTradeYesNoWindowTemplate)
                                .cast::<u8>()
                                .cast_mut(),
                            1u16,
                            14u8,
                            0u8,
                        );
                        ((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(111))
                        .write(4u8);
                        DrawBottomRowText(
                            ((((&raw const sActionTexts)
                                .cast::<u8>()
                                .cast_mut()
                                .cast::<*mut u8>())
                            .cast::<*mut u8>())
                            .wrapping_offset(4))
                            .read(),
                            (((100728832i32).wrapping_add(
                                ((((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                                    .wrapping_add(114)
                                    .cast::<u16>())
                                .read()) as i32)
                                    .wrapping_mul(32i32),
                            )) as usize as *mut u8),
                            24u8,
                        );
                    }
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn RedrawChooseAPokemonWindow() {
    unsafe {
        PrintTradePartnerPartyNicknames();
        ((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(111))
            .write(0u8);
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(52))
                .read()) as i32) as isize
                    * 68,
            ))
            .wrapping_add(62),
            2,
            1,
            (0u16) as i32,
        );
        DrawBottomRowText(
            ((((&raw const sActionTexts)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u8>())
            .cast::<*mut u8>())
            .wrapping_offset(1))
            .read(),
            (((100728832i32).wrapping_add(
                ((((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(114)
                    .cast::<u16>())
                .read()) as i32)
                    .wrapping_mul(32i32),
            )) as usize as *mut u8),
            24u8,
        );
    }
}
pub(crate) unsafe extern "C" fn CB_ProcessSelectedMonInput() {
    unsafe {
        'l1: {
            let __sw1 = ((Menu_ProcessInputNoWrap()) as i32);
            if __sw1 == (-1i32) {
                PlaySE(5u16);
                RedrawChooseAPokemonWindow();
                break 'l1;
            }
            if __sw1 == (-2i32) {
                break 'l1;
            }
            if __sw1 == 0i32 {
                BeginNormalPaletteFade(4294967295u32, 0i8, 0u8, 16u8, 0u16);
                ((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(111))
                    .write(2u8);
                break 'l1;
            }
            if __sw1 == 1i32 {
                'l2: {
                    let __sw2 = CanTradeSelectedMon(
                        (&raw mut gPlayerParty).cast::<u8>(),
                        ((((&raw mut gPlayerPartyCount).cast::<u8>()).read()) as i32),
                        ((((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(53))
                        .read()) as i32),
                    );
                    if __sw2 == 0u32 {
                        SetReadyToTrade();
                        crate::c::bf_write(
                            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                ((((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                                    .wrapping_add(52))
                                .read()) as i32) as isize
                                    * 68,
                            ))
                            .wrapping_add(62),
                            2,
                            1,
                            (1u16) as i32,
                        );
                        break 'l2;
                    }
                    if __sw2 == 1u32 {
                        QueueAction(3u16, 3u8);
                        ((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(111))
                        .write(8u8);
                        break 'l2;
                    }
                    if __sw2 == 2u32 || __sw2 == 4u32 {
                        QueueAction(3u16, 6u8);
                        ((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(111))
                        .write(8u8);
                        break 'l2;
                    }
                    if __sw2 == 3u32 || __sw2 == 5u32 {
                        QueueAction(3u16, 7u8);
                        ((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(111))
                        .write(8u8);
                        break 'l2;
                    }
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn CB_ChooseMonAfterButtonPress() {
    unsafe {
        if (((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(46)
            .cast::<u16>())
        .read()) as i32)
            & 1i32)
            != 0)
            || (((((((&raw mut gMain).cast::<u8>())
                .wrapping_add(46)
                .cast::<u16>())
            .read()) as i32)
                & 2i32)
                != 0)
        {
            PlaySE(5u16);
            RedrawChooseAPokemonWindow();
        }
    }
}
pub(crate) unsafe extern "C" fn CB_ShowTradeMonSummaryScreen() {
    unsafe {
        if !((crate::c::bf_read(
            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
            7,
            1,
            false,
        ) as u16)
            != 0)
        {
            if ((((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(53))
                .read()) as i32)
                < 6i32
            {
                ShowPokemonSummaryScreen(
                    1u8,
                    (&raw mut gPlayerParty).cast::<u8>(),
                    ((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(53))
                    .read(),
                    (((((((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(54))
                    .cast::<u8>())
                    .read()) as i32)
                        .wrapping_sub(1i32)) as u8),
                    Some(CB2_ReturnToTradeMenu),
                );
            } else {
                ShowPokemonSummaryScreen(
                    1u8,
                    (&raw mut gEnemyParty).cast::<u8>(),
                    ((((((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(53))
                    .read()) as i32)
                        .wrapping_sub(6i32)) as u8),
                    ((((((((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(54))
                    .cast::<u8>())
                    .wrapping_offset(1))
                    .read()) as i32)
                        .wrapping_sub(1i32)) as u8),
                    Some(CB2_ReturnToTradeMenu),
                );
            }
            FreeAllWindowBuffers();
        }
    }
}
pub(crate) unsafe extern "C" fn CheckValidityOfTradeMons(
    aliveMons: *mut u8,
    playerPartyCount: u8,
    playerMonIdx: u8,
    partnerMonIdx: u8,
) -> u8 {
    unsafe {
        let mut aliveMons = aliveMons;
        let mut playerPartyCount = playerPartyCount;
        let mut playerMonIdx = playerMonIdx;
        let mut partnerMonIdx = partnerMonIdx;
        let mut i: i32 = 0i32;
        let mut partnerSpecies: u16 = 0u16;
        let mut hasLiveMon: u8 = 0u8;
        {
            i = 0i32;
            'l1: loop {
                if !(i < ((playerPartyCount) as i32)) {
                    break 'l1;
                }
                'l2: {
                    if ((playerMonIdx) as i32) != i {
                        hasLiveMon = ((((hasLiveMon) as i32).wrapping_add(
                            ((((aliveMons).wrapping_offset((i) as isize)).read()) as i32),
                        )) as u8);
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        partnerMonIdx = ((crate::c::rem_i32(((partnerMonIdx) as i32), 6i32)) as u8);
        partnerSpecies = ((GetMonData2(
            ((&raw mut gEnemyParty).cast::<u8>())
                .wrapping_offset(((partnerMonIdx) as i32) as isize * 100),
            11i32,
        )) as u16);
        if (((partnerSpecies) as i32) == 410i32) || (((partnerSpecies) as i32) == 151i32) {
            if !((GetMonData2(
                ((&raw mut gEnemyParty).cast::<u8>())
                    .wrapping_offset(((partnerMonIdx) as i32) as isize * 100),
                80i32,
            )) != 0)
            {
                return 2u8;
            }
        }
        if !((IsNationalPokedexEnabled()) != 0) {
            if ((((((((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(81))
            .cast::<u8>())
            .wrapping_offset(6))
            .cast::<u8>())
            .wrapping_offset(((partnerMonIdx) as i32) as isize))
            .read())
                != 0)
                || (!((IsSpeciesInHoennDex(partnerSpecies)) != 0))
            {
                return 2u8;
            }
        }
        if (hasLiveMon) != 0 {
            hasLiveMon = 1u8;
        }
        return hasLiveMon;
    }
}
pub(crate) unsafe extern "C" fn CheckMonsBeforeTrade() -> u32 {
    unsafe {
        let mut i: i32 = 0i32;
        let mut aliveMons = crate::ffi::Align4([0u8; 12]);
        {
            i = 0i32;
            'l1: loop {
                if !(i
                    < (((((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(54))
                    .cast::<u8>())
                    .read()) as i32))
                {
                    break 'l1;
                }
                'l2: {
                    (((&raw mut aliveMons).cast::<u8>()).wrapping_offset((i) as isize)).write(
                        (((((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(69))
                        .cast::<u8>())
                        .cast::<u8>())
                        .wrapping_offset((i) as isize))
                        .read(),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        'l3: {
            let __sw1 = ((CheckValidityOfTradeMons(
                (&raw mut aliveMons).cast::<u8>(),
                (((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(54))
                .cast::<u8>())
                .read(),
                ((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(53))
                    .read(),
                ((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(126))
                    .read(),
            )) as i32);
            if __sw1 == 0i32 {
                QueueAction(3u16, 3u8);
                SetLinkData(48076u16, 0u16);
                break 'l3;
            }
            if __sw1 == 1i32 {
                QueueAction(3u16, 1u8);
                SetLinkData(48059u16, 0u16);
                break 'l3;
            }
            if __sw1 == 2i32 {
                QueueAction(3u16, 8u8);
                return 1u32;
            }
        }
        return 0u32;
    }
}
pub(crate) unsafe extern "C" fn CB_ProcessConfirmTradeInput() {
    unsafe {
        'l1: {
            let __sw1 = ((Menu_ProcessInputNoWrapClearOnChoose()) as i32);
            if __sw1 == 0i32 {
                if !((CheckMonsBeforeTrade()) != 0) {
                    ((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(111))
                    .write(100u8);
                } else {
                    ((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(111))
                    .write(17u8);
                }
                PutWindowTilemap(17u8);
                break 'l1;
            }
            if __sw1 == 1i32 || __sw1 == (-1i32) {
                QueueAction(3u16, 1u8);
                if (IsLinkTradeTaskFinished()) != 0 {
                    SetLinkData(48076u16, 0u16);
                }
                ((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(111))
                    .write(100u8);
                PutWindowTilemap(17u8);
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn RestoreNicknamesCoveredByYesNo() {
    unsafe {
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i
                    < ((((((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(54))
                    .cast::<u8>())
                    .wrapping_offset(1))
                    .read()) as i32)
                        .wrapping_sub(4i32))
                {
                    break 'l1;
                }
                'l2: {
                    PutWindowTilemap((((i).wrapping_add(12i32)) as u8));
                    CopyWindowToVram((((i).wrapping_add(12i32)) as u8), 1u8);
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn CB_ProcessCancelTradeInput() {
    unsafe {
        'l1: {
            let __sw1 = ((Menu_ProcessInputNoWrapClearOnChoose()) as i32);
            if __sw1 == 0i32 {
                PrintTradeMessage(4u8);
                SetLinkData(61098u16, 0u16);
                crate::c::bf_write(
                    (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(52))
                        .read()) as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(62),
                    2,
                    1,
                    (1u16) as i32,
                );
                ((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(111))
                    .write(100u8);
                RestoreNicknamesCoveredByYesNo();
                break 'l1;
            }
            if __sw1 == 1i32 || __sw1 == (-1i32) {
                PlaySE(5u16);
                RedrawChooseAPokemonWindow();
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn CB_SetSelectedMons() {
    unsafe {
        if ((GetMultiplayerId()) as i32) == 0i32 {
            rbox_fill_rectangle(0u8);
            SetSelectedMon(
                ((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(53))
                    .read(),
            );
            SetSelectedMon(
                ((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(126))
                    .read(),
            );
        }
        ((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(111))
            .write(7u8);
    }
}
pub(crate) unsafe extern "C" fn CB_PrintIsThisTradeOkay() {
    unsafe {
        if ((((((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(116))
            .cast::<u8>())
        .read()) as i32)
            == 5i32)
            && (((((((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(116))
            .cast::<u8>())
            .wrapping_offset(1))
            .read()) as i32)
                == 5i32)
        {
            PrintIsThisTradeOkay();
            ((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(111))
                .write(14u8);
        }
    }
}
pub(crate) unsafe extern "C" fn CB_InitConfirmTradePrompt() {
    unsafe {
        let __p1 =
            (((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(168);
        (__p1).write(((__p1).read()).wrapping_add(1));
        if ((((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(168))
            .read()) as i32)
            > 120i32
        {
            CreateYesNoMenu(
                (&raw const sTradeYesNoWindowTemplate)
                    .cast::<u8>()
                    .cast_mut(),
                1u16,
                14u8,
                0u8,
            );
            ((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(168))
                .write(0u8);
            ((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(111))
                .write(3u8);
        }
    }
}
pub(crate) unsafe extern "C" fn CB_HandleTradeCanceled() {
    unsafe {
        let mut i: i32 = 0i32;
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(46)
            .cast::<u16>())
        .read()) as i32)
            & 1i32)
            != 0
        {
            PlaySE(5u16);
            rbox_fill_rectangle(0u8);
            rbox_fill_rectangle(1u8);
            {
                i = 0i32;
                'l1: loop {
                    if !(i < 4i32) {
                        break 'l1;
                    }
                    'l2: {
                        FillWindowPixelBuffer((((i).wrapping_add(14i32)) as u8), 0u8);
                        rbox_fill_rectangle((((i).wrapping_add(14i32)) as u8));
                    }
                    i = (i).wrapping_add(1);
                }
            }
            RedrawPartyWindow(0u8);
            RedrawPartyWindow(1u8);
            ((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(111))
                .write(0u8);
            crate::c::bf_write(
                (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(52))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(62),
                2,
                1,
                (0u16) as i32,
            );
        }
    }
}
pub(crate) unsafe extern "C" fn CB_InitExitCanceledTrade() {
    unsafe {
        if !((crate::c::bf_read(
            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
            7,
            1,
            false,
        ) as u16)
            != 0)
        {
            if (((&raw mut gWirelessCommType).cast::<u8>()).read()) != 0 {
                SetLinkStandbyCallback();
            } else {
                SetCloseLinkCallbackAndType(12u16);
            }
            ((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(111))
                .write(12u8);
        }
    }
}
pub(crate) unsafe extern "C" fn CB_ExitCanceledTrade() {
    unsafe {
        if (((&raw mut gWirelessCommType).cast::<u8>()).read()) != 0 {
            if ((IsLinkTradeTaskFinished()) != 0) && (GetNumQueuedActions() == 0u32) {
                Free(
                    ((&raw mut sMenuTextTileBuffer)
                        .cast::<u8>()
                        .cast::<*mut u8>())
                    .read(),
                );
                Free(((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read());
                FreeAllWindowBuffers();
                DestroyWirelessStatusIndicatorSprite();
                SetMainCallback2(Some(CB2_ReturnToFieldFromMultiplayer));
            }
        } else {
            if !((((&raw mut gReceivedRemoteLinkPlayers).cast::<u8>()).read()) != 0) {
                Free(
                    ((&raw mut sMenuTextTileBuffer)
                        .cast::<u8>()
                        .cast::<*mut u8>())
                    .read(),
                );
                Free(((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read());
                FreeAllWindowBuffers();
                SetMainCallback2(Some(CB2_ReturnToFieldFromMultiplayer));
            }
        }
    }
}
pub(crate) unsafe extern "C" fn CB_WaitToStartRfuTrade() {
    unsafe {
        if (!((Rfu_SetLinkRecovery(0u32)) != 0)) && (GetNumQueuedActions() == 0u32) {
            SetLinkStandbyCallback();
            ((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(111))
                .write(13u8);
        }
    }
}
pub(crate) unsafe extern "C" fn CB_PartnersMonWasInvalid() {
    unsafe {
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(46)
            .cast::<u16>())
        .read()) as i32)
            & 1i32)
            != 0
        {
            SetLinkData(48076u16, 0u16);
            ((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(111))
                .write(100u8);
        }
    }
}
pub(crate) unsafe extern "C" fn RunTradeMenuCallback() {
    unsafe {
        'l1: {
            let __sw1 = ((((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(111))
            .read()) as i32);
            if __sw1 == 0i32 {
                CB_ProcessMenuInput();
                break 'l1;
            }
            if __sw1 == 1i32 {
                CB_ProcessSelectedMonInput();
                break 'l1;
            }
            if __sw1 == 2i32 {
                CB_ShowTradeMonSummaryScreen();
                break 'l1;
            }
            if __sw1 == 3i32 {
                CB_ProcessConfirmTradeInput();
                break 'l1;
            }
            if __sw1 == 4i32 {
                CB_ProcessCancelTradeInput();
                break 'l1;
            }
            if __sw1 == 6i32 {
                CB_SetSelectedMons();
                break 'l1;
            }
            if __sw1 == 7i32 {
                CB_PrintIsThisTradeOkay();
                break 'l1;
            }
            if __sw1 == 8i32 {
                CB_HandleTradeCanceled();
                break 'l1;
            }
            if __sw1 == 9i32 {
                CB_FadeToStartTrade();
                break 'l1;
            }
            if __sw1 == 10i32 {
                CB_WaitToStartTrade();
                break 'l1;
            }
            if __sw1 == 11i32 {
                CB_InitExitCanceledTrade();
                break 'l1;
            }
            if __sw1 == 12i32 {
                CB_ExitCanceledTrade();
                break 'l1;
            }
            if __sw1 == 13i32 {
                CB_StartLinkTrade();
                break 'l1;
            }
            if __sw1 == 14i32 {
                CB_InitConfirmTradePrompt();
                break 'l1;
            }
            if __sw1 == 15i32 {
                CB_ChooseMonAfterButtonPress();
                break 'l1;
            }
            if __sw1 == 16i32 {
                CB_WaitToStartRfuTrade();
                break 'l1;
            }
            if __sw1 == 17i32 {
                CB_PartnersMonWasInvalid();
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SetSelectedMon(cursorPosition: u8) {
    unsafe {
        let mut cursorPosition = cursorPosition;
        let mut whichParty: u8 = ((crate::c::div_i32(((cursorPosition) as i32), 6i32)) as u8);
        if ((((((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(116))
        .cast::<u8>())
        .wrapping_offset(((whichParty) as i32) as isize))
        .read()) as i32)
            == 0i32
        {
            ((((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(116))
                .cast::<u8>())
            .wrapping_offset(((whichParty) as i32) as isize))
            .write(1u8);
            ((((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(118))
                .cast::<u8>())
            .wrapping_offset(((whichParty) as i32) as isize))
            .write(cursorPosition);
        }
    }
}
pub(crate) unsafe extern "C" fn DrawSelectedMonScreen(whichParty: u8) {
    unsafe {
        let mut whichParty = whichParty;
        let mut nameStringWidth: i8 = 0i8;
        let mut nickname = crate::ffi::Align4([0u8; 20]);
        let mut movesString = crate::ffi::Align4([0u8; 56]);
        let mut i: u8 = 0u8;
        let mut partyIdx: u8 = 0u8;
        let mut selectedMonParty: u8 = 0u8;
        let mut selectedMonIdx: u8 =
            ((((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(118))
                .cast::<u8>())
            .wrapping_offset(((whichParty) as i32) as isize))
            .read();
        selectedMonParty = 1u8;
        if ((((((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(118))
        .cast::<u8>())
        .wrapping_offset(((whichParty) as i32) as isize))
        .read()) as i32)
            < 6i32
        {
            selectedMonParty = 0u8;
        }
        partyIdx = ((crate::c::rem_i32(((selectedMonIdx) as i32), 6i32)) as u8);
        nameStringWidth = 0i8;
        'l1: {
            let __sw1 = ((((((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(116))
            .cast::<u8>())
            .wrapping_offset(((whichParty) as i32) as isize))
            .read()) as i32);
            let __matched = __sw1 == 1i32 || __sw1 == 2i32 || __sw1 == 3i32 || __sw1 == 4i32;
            if !__matched {
                break 'l1;
            }
            if __sw1 == 1i32 {
                {
                    i = 0u8;
                    'l2: loop {
                        if !(((i) as i32)
                            < ((((((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(54))
                            .cast::<u8>())
                            .wrapping_offset(((whichParty) as i32) as isize))
                            .read()) as i32))
                        {
                            break 'l2;
                        }
                        'l3: {
                            crate::c::bf_write(
                                (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                    ((((((((((&raw mut sTradeMenu)
                                        .cast::<u8>()
                                        .cast::<*mut u8>())
                                    .read())
                                    .wrapping_add(40))
                                    .cast::<u8>())
                                    .wrapping_offset(((selectedMonParty) as i32) as isize * 6))
                                    .cast::<u8>())
                                    .wrapping_offset(((i) as i32) as isize))
                                    .read()) as i32) as isize
                                        * 68,
                                ))
                                .wrapping_add(62),
                                2,
                                1,
                                (1u16) as i32,
                            );
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                {
                    i = 0u8;
                    'l4: loop {
                        if !(((i) as i32) < 6i32) {
                            break 'l4;
                        }
                        'l5: {
                            ClearWindowTilemap(
                                ((((i) as i32).wrapping_add(
                                    (((whichParty) as i32).wrapping_mul(6i32)).wrapping_add(2i32),
                                )) as u8),
                            );
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                crate::c::bf_write(
                    (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(40))
                        .cast::<u8>())
                        .wrapping_offset(((selectedMonParty) as i32) as isize * 6))
                        .cast::<u8>())
                        .wrapping_offset(((partyIdx) as i32) as isize))
                        .read()) as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(62),
                    2,
                    1,
                    (0u16) as i32,
                );
                (((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(40))
                    .cast::<u8>())
                    .wrapping_offset(((selectedMonParty) as i32) as isize * 6))
                    .cast::<u8>())
                    .wrapping_offset(((partyIdx) as i32) as isize))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(46))
                .cast::<i16>())
                .write(20i16);
                ((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(40))
                    .cast::<u8>())
                    .wrapping_offset(((selectedMonParty) as i32) as isize * 6))
                    .cast::<u8>())
                    .wrapping_offset(((partyIdx) as i32) as isize))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(46))
                .cast::<i16>())
                .wrapping_offset(2))
                .write(
                    ((((crate::c::div_i32(
                        (((((((&raw const sTradeMonSpriteCoords).cast::<u8>().cast_mut())
                            .cast::<u8>())
                        .wrapping_offset(
                            (((selectedMonParty) as i32).wrapping_mul(6i32)) as isize * 2,
                        ))
                        .cast::<u8>())
                        .read()) as i32)
                            .wrapping_add(
                                (((((((&raw const sTradeMonSpriteCoords)
                                    .cast::<u8>()
                                    .cast_mut())
                                .cast::<u8>())
                                .wrapping_offset(
                                    ((((selectedMonParty) as i32).wrapping_mul(6i32))
                                        .wrapping_add(1i32))
                                        as isize
                                        * 2,
                                ))
                                .cast::<u8>())
                                .read()) as i32),
                            ),
                        2i32,
                    ))
                    .wrapping_mul(8i32))
                    .wrapping_add(14i32)) as i16),
                );
                ((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(40))
                    .cast::<u8>())
                    .wrapping_offset(((selectedMonParty) as i32) as isize * 6))
                    .cast::<u8>())
                    .wrapping_offset(((partyIdx) as i32) as isize))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(46))
                .cast::<i16>())
                .wrapping_offset(4))
                .write(
                    (((((((((((&raw const sTradeMonSpriteCoords).cast::<u8>().cast_mut())
                        .cast::<u8>())
                    .wrapping_offset(
                        (((selectedMonParty) as i32).wrapping_mul(6i32)) as isize * 2,
                    ))
                    .cast::<u8>())
                    .wrapping_offset(1))
                    .read()) as i32)
                        .wrapping_mul(8i32))
                    .wrapping_sub(12i32)) as i16),
                );
                StoreSpriteCallbackInData6(
                    ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(40))
                        .cast::<u8>())
                        .wrapping_offset(((selectedMonParty) as i32) as isize * 6))
                        .cast::<u8>())
                        .wrapping_offset(((partyIdx) as i32) as isize))
                        .read()) as i32) as isize
                            * 68,
                    ),
                    Some(SpriteCB_MonIcon),
                );
                let __p2 = (((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(116))
                .cast::<u8>())
                .wrapping_offset(((whichParty) as i32) as isize);
                (__p2).write(((__p2).read()).wrapping_add(1));
                Trade_MoveSelectedMonToTarget(
                    ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(40))
                        .cast::<u8>())
                        .wrapping_offset(((selectedMonParty) as i32) as isize * 6))
                        .cast::<u8>())
                        .wrapping_offset(((partyIdx) as i32) as isize))
                        .read()) as i32) as isize
                            * 68,
                    ),
                );
                CopyToBgTilemapBufferRect_ChangePalette(
                    1u8,
                    (((&raw const sTradePartyBoxTilemap)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<u16>())
                    .cast::<u16>())
                    .cast::<u8>(),
                    ((((whichParty) as i32).wrapping_mul(15i32)) as u8),
                    0u8,
                    15u8,
                    17u8,
                    0u8,
                );
                CopyBgTilemapBufferToVram(1u8);
                CopyBgTilemapBufferToVram(0u8);
                if ((selectedMonParty) as i32) == 0i32 {
                    PrintTradePartnerPartyNicknames();
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                if core::mem::transmute::<_, usize>(
                    ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(40))
                        .cast::<u8>())
                        .wrapping_offset(((selectedMonParty) as i32) as isize * 6))
                        .cast::<u8>())
                        .wrapping_offset(((partyIdx) as i32) as isize))
                        .read()) as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(28)
                    .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                    .read(),
                ) == (SpriteCB_MonIcon as *const () as usize)
                {
                    ((((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(116))
                    .cast::<u8>())
                    .wrapping_offset(((whichParty) as i32) as isize))
                    .write(3u8);
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                CopyToBgTilemapBufferRect_ChangePalette(
                    1u8,
                    (((&raw const sTradeMovesBoxTilemap)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<u16>())
                    .cast::<u16>())
                    .cast::<u8>(),
                    ((((selectedMonParty) as i32).wrapping_mul(15i32)) as u8),
                    0u8,
                    15u8,
                    17u8,
                    0u8,
                );
                CopyBgTilemapBufferToVram(1u8);
                ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(40))
                    .cast::<u8>())
                    .wrapping_offset(((selectedMonParty) as i32) as isize * 6))
                    .cast::<u8>())
                    .wrapping_offset(((partyIdx) as i32) as isize))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(32)
                .cast::<i16>())
                .write(
                    ((((crate::c::div_i32(
                        (((((((&raw const sTradeMonSpriteCoords).cast::<u8>().cast_mut())
                            .cast::<u8>())
                        .wrapping_offset(
                            (((selectedMonParty) as i32).wrapping_mul(6i32)) as isize * 2,
                        ))
                        .cast::<u8>())
                        .read()) as i32)
                            .wrapping_add(
                                (((((((&raw const sTradeMonSpriteCoords)
                                    .cast::<u8>()
                                    .cast_mut())
                                .cast::<u8>())
                                .wrapping_offset(
                                    ((((selectedMonParty) as i32).wrapping_mul(6i32))
                                        .wrapping_add(1i32))
                                        as isize
                                        * 2,
                                ))
                                .cast::<u8>())
                                .read()) as i32),
                            ),
                        2i32,
                    ))
                    .wrapping_mul(8i32))
                    .wrapping_add(14i32)) as i16),
                );
                ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(40))
                    .cast::<u8>())
                    .wrapping_offset(((selectedMonParty) as i32) as isize * 6))
                    .cast::<u8>())
                    .wrapping_offset(((partyIdx) as i32) as isize))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(34)
                .cast::<i16>())
                .write(
                    (((((((((((&raw const sTradeMonSpriteCoords).cast::<u8>().cast_mut())
                        .cast::<u8>())
                    .wrapping_offset(
                        (((selectedMonParty) as i32).wrapping_mul(6i32)) as isize * 2,
                    ))
                    .cast::<u8>())
                    .wrapping_offset(1))
                    .read()) as i32)
                        .wrapping_mul(8i32))
                    .wrapping_sub(12i32)) as i16),
                );
                ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(40))
                    .cast::<u8>())
                    .wrapping_offset(((selectedMonParty) as i32) as isize * 6))
                    .cast::<u8>())
                    .wrapping_offset(((partyIdx) as i32) as isize))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(36)
                .cast::<i16>())
                .write(0i16);
                ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(40))
                    .cast::<u8>())
                    .wrapping_offset(((selectedMonParty) as i32) as isize * 6))
                    .cast::<u8>())
                    .wrapping_offset(((partyIdx) as i32) as isize))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(38)
                .cast::<i16>())
                .write(0i16);
                nameStringWidth = ((GetMonNicknameWidth(
                    (&raw mut nickname).cast::<u8>(),
                    selectedMonParty,
                    partyIdx,
                )) as i8);
                AddTextPrinterParameterized3(
                    (((((whichParty) as i32).wrapping_mul(2i32)).wrapping_add(14i32)) as u8),
                    0u8,
                    ((crate::c::div_i32((80i32).wrapping_sub(((nameStringWidth) as i32)), 2i32))
                        as u8),
                    4u8,
                    ((&raw const sTradeTextColors).cast::<u8>().cast_mut()).cast::<u8>(),
                    0i8,
                    (&raw mut nickname).cast::<u8>(),
                );
                BufferMovesString(
                    (&raw mut movesString).cast::<u8>(),
                    selectedMonParty,
                    partyIdx,
                );
                AddTextPrinterParameterized4(
                    (((((whichParty) as i32).wrapping_mul(2i32)).wrapping_add(15i32)) as u8),
                    1u8,
                    0u8,
                    0u8,
                    0u8,
                    0u8,
                    ((&raw const sTradeTextColors).cast::<u8>().cast_mut()).cast::<u8>(),
                    0i8,
                    (&raw mut movesString).cast::<u8>(),
                );
                PutWindowTilemap(
                    (((((whichParty) as i32).wrapping_mul(2i32)).wrapping_add(14i32)) as u8),
                );
                CopyWindowToVram(
                    (((((whichParty) as i32).wrapping_mul(2i32)).wrapping_add(14i32)) as u8),
                    3u8,
                );
                PutWindowTilemap(
                    (((((whichParty) as i32).wrapping_mul(2i32)).wrapping_add(15i32)) as u8),
                );
                CopyWindowToVram(
                    (((((whichParty) as i32).wrapping_mul(2i32)).wrapping_add(15i32)) as u8),
                    3u8,
                );
                let __p3 = (((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(116))
                .cast::<u8>())
                .wrapping_offset(((whichParty) as i32) as isize);
                (__p3).write(((__p3).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 4i32 {
                PrintLevelAndGender(
                    whichParty,
                    partyIdx,
                    (((((((((&raw const sSelectedMonLevelGenderCoords)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>())
                    .wrapping_offset(((whichParty) as i32) as isize * 2))
                    .cast::<u8>())
                    .read()) as i32)
                        .wrapping_add(4i32)) as u8),
                    ((((((((((&raw const sSelectedMonLevelGenderCoords)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>())
                    .wrapping_offset(((whichParty) as i32) as isize * 2))
                    .cast::<u8>())
                    .wrapping_offset(1))
                    .read()) as i32)
                        .wrapping_add(1i32)) as u8),
                    (((((&raw const sSelectedMonLevelGenderCoords)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>())
                    .wrapping_offset(((whichParty) as i32) as isize * 2))
                    .cast::<u8>())
                    .read(),
                    ((((((&raw const sSelectedMonLevelGenderCoords)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>())
                    .wrapping_offset(((whichParty) as i32) as isize * 2))
                    .cast::<u8>())
                    .wrapping_offset(1))
                    .read(),
                );
                let __p4 = (((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(116))
                .cast::<u8>())
                .wrapping_offset(((whichParty) as i32) as isize);
                (__p4).write(((__p4).read()).wrapping_add(1));
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn GetMonNicknameWidth(
    str: *mut u8,
    whichParty: u8,
    partyIdx: u8,
) -> u8 {
    unsafe {
        let mut str = str;
        let mut whichParty = whichParty;
        let mut partyIdx = partyIdx;
        let mut nickname = crate::ffi::Align4([0u8; 11]);
        if ((whichParty) as i32) == 0i32 {
            GetMonData3(
                ((&raw mut gPlayerParty).cast::<u8>())
                    .wrapping_offset(((partyIdx) as i32) as isize * 100),
                2i32,
                (&raw mut nickname).cast::<u8>(),
            );
        } else {
            GetMonData3(
                ((&raw mut gEnemyParty).cast::<u8>())
                    .wrapping_offset(((partyIdx) as i32) as isize * 100),
                2i32,
                (&raw mut nickname).cast::<u8>(),
            );
        }
        StringCopy_Nickname(str, (&raw mut nickname).cast::<u8>());
        return ((GetStringWidth(0u8, str, ((GetFontAttribute(0u8, 2u8)) as i16))) as u8);
    }
}
pub(crate) unsafe extern "C" fn BufferMovesString(str: *mut u8, whichParty: u8, partyIdx: u8) {
    unsafe {
        let mut str = str;
        let mut whichParty = whichParty;
        let mut partyIdx = partyIdx;
        let mut moves = crate::ffi::Align4([0u8; 8]);
        let mut i: u16 = 0u16;
        if !((((((((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(81))
        .cast::<u8>())
        .wrapping_offset(((whichParty) as i32) as isize * 6))
        .cast::<u8>())
        .wrapping_offset(((partyIdx) as i32) as isize))
        .read())
            != 0)
        {
            {
                i = 0u16;
                'l1: loop {
                    if !(((i) as i32) < 4i32) {
                        break 'l1;
                    }
                    'l2: {
                        if ((whichParty) as i32) == 0i32 {
                            (((&raw mut moves).cast::<u16>())
                                .wrapping_offset(((i) as i32) as isize))
                            .write(
                                ((GetMonData3(
                                    ((&raw mut gPlayerParty).cast::<u8>())
                                        .wrapping_offset(((partyIdx) as i32) as isize * 100),
                                    ((i) as i32).wrapping_add(13i32),
                                    core::ptr::null_mut(),
                                )) as u16),
                            );
                        } else {
                            (((&raw mut moves).cast::<u16>())
                                .wrapping_offset(((i) as i32) as isize))
                            .write(
                                ((GetMonData3(
                                    ((&raw mut gEnemyParty).cast::<u8>())
                                        .wrapping_offset(((partyIdx) as i32) as isize * 100),
                                    ((i) as i32).wrapping_add(13i32),
                                    core::ptr::null_mut(),
                                )) as u16),
                            );
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
            StringCopy(
                str,
                ((&raw const sText_EmptyString).cast::<u8>().cast_mut()).cast::<u8>(),
            );
            {
                i = 0u16;
                'l3: loop {
                    if !(((i) as i32) < 4i32) {
                        break 'l3;
                    }
                    'l4: {
                        if (((((&raw mut moves).cast::<u16>())
                            .wrapping_offset(((i) as i32) as isize))
                        .read()) as i32)
                            != 0i32
                        {
                            StringAppend(
                                str,
                                (((&raw mut gMoveNames).cast::<u8>()).wrapping_offset(
                                    (((((&raw mut moves).cast::<u16>())
                                        .wrapping_offset(((i) as i32) as isize))
                                    .read()) as i32) as isize
                                        * 13,
                                ))
                                .cast::<u8>(),
                            );
                        }
                        StringAppend(
                            str,
                            ((&raw const sText_NewLine).cast::<u8>().cast_mut()).cast::<u8>(),
                        );
                    }
                    i = (i).wrapping_add(1);
                }
            }
        } else {
            StringCopy(
                str,
                ((&raw const sText_EmptyString).cast::<u8>().cast_mut()).cast::<u8>(),
            );
            StringAppend(
                str,
                ((&raw const sText_FourQuestionMarks).cast::<u8>().cast_mut()).cast::<u8>(),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn PrintPartyMonNickname(
    whichParty: u8,
    windowId: u8,
    nickname: *mut u8,
) {
    unsafe {
        let mut whichParty = whichParty;
        let mut windowId = windowId;
        let mut nickname = nickname;
        let mut xPos: u8 = 0u8;
        windowId = ((((windowId) as i32)
            .wrapping_add((((whichParty) as i32).wrapping_mul(6i32)).wrapping_add(2i32)))
            as u8);
        xPos = ((GetStringCenterAlignXOffset(0i32, nickname, 64i32)) as u8);
        AddTextPrinterParameterized3(
            windowId,
            0u8,
            xPos,
            4u8,
            ((&raw const sTradeTextColors).cast::<u8>().cast_mut()).cast::<u8>(),
            0i8,
            nickname,
        );
        PutWindowTilemap(windowId);
        CopyWindowToVram(windowId, 3u8);
    }
}
pub(crate) unsafe extern "C" fn PrintPartyNicknames(whichParty: u8) {
    unsafe {
        let mut whichParty = whichParty;
        let mut i: u8 = 0u8;
        let mut nickname = crate::ffi::Align4([0u8; 20]);
        let mut str = crate::ffi::Align4([0u8; 32]);
        let mut party: *mut u8 = (if ((whichParty) as i32) == 0i32 {
            (&raw mut gPlayerParty).cast::<u8>()
        } else {
            (&raw mut gEnemyParty).cast::<u8>()
        });
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32)
                    < ((((((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(54))
                    .cast::<u8>())
                    .wrapping_offset(((whichParty) as i32) as isize))
                    .read()) as i32))
                {
                    break 'l1;
                }
                'l2: {
                    GetMonData3(
                        (party).wrapping_offset(((i) as i32) as isize * 100),
                        2i32,
                        (&raw mut nickname).cast::<u8>(),
                    );
                    StringCopy_Nickname(
                        (&raw mut str).cast::<u8>(),
                        (&raw mut nickname).cast::<u8>(),
                    );
                    PrintPartyMonNickname(whichParty, i, (&raw mut str).cast::<u8>());
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn PrintLevelAndGender(
    whichParty: u8,
    monIdx: u8,
    x: u8,
    y: u8,
    width: u8,
    height: u8,
) {
    unsafe {
        let mut whichParty = whichParty;
        let mut monIdx = monIdx;
        let mut x = x;
        let mut y = y;
        let mut width = width;
        let mut height = height;
        let mut level: u8 = 0u8;
        let mut symbolTile: u32 = 0u32;
        let mut gender: u8 = 0u8;
        let mut nickname = crate::ffi::Align4([0u8; 11]);
        CopyToBgTilemapBufferRect_ChangePalette(
            1u8,
            (((&raw mut gTradeMenuMonBox_Tilemap).cast::<u16>()).cast::<u16>()).cast::<u8>(),
            width,
            height,
            6u8,
            3u8,
            0u8,
        );
        CopyBgTilemapBufferToVram(1u8);
        if ((whichParty) as i32) == 0i32 {
            level = ((GetMonData3(
                ((&raw mut gPlayerParty).cast::<u8>())
                    .wrapping_offset(((monIdx) as i32) as isize * 100),
                56i32,
                core::ptr::null_mut(),
            )) as u8);
        } else {
            level = ((GetMonData3(
                ((&raw mut gEnemyParty).cast::<u8>())
                    .wrapping_offset(((monIdx) as i32) as isize * 100),
                56i32,
                core::ptr::null_mut(),
            )) as u8);
        }
        if !((((((((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(81))
        .cast::<u8>())
        .wrapping_offset(((whichParty) as i32) as isize * 6))
        .cast::<u8>())
        .wrapping_offset(((monIdx) as i32) as isize))
        .read())
            != 0)
        {
            if crate::c::div_i32(((level) as i32), 10i32) != 0i32 {
                ((((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(2288))
                .cast::<u16>())
                .wrapping_offset(
                    (((x) as i32).wrapping_add(((y) as i32).wrapping_mul(32i32))) as isize,
                ))
                .write((((crate::c::div_i32(((level) as i32), 10i32)).wrapping_add(96i32)) as u16));
            }
            ((((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(2288))
            .cast::<u16>())
            .wrapping_offset(
                ((((x) as i32).wrapping_add(((y) as i32).wrapping_mul(32i32))).wrapping_add(1i32))
                    as isize,
            ))
            .write((((crate::c::rem_i32(((level) as i32), 10i32)).wrapping_add(112i32)) as u16));
        } else {
            ((((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(2288))
            .cast::<u16>())
            .wrapping_offset(
                ((((x) as i32).wrapping_add(((y) as i32).wrapping_mul(32i32))).wrapping_sub(32i32))
                    as isize,
            ))
            .write(
                ((((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(2288))
                .cast::<u16>())
                .wrapping_offset(
                    ((((x) as i32).wrapping_add(((y) as i32).wrapping_mul(32i32)))
                        .wrapping_sub(33i32)) as isize,
                ))
                .read(),
            );
            ((((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(2288))
            .cast::<u16>())
            .wrapping_offset(
                ((((x) as i32).wrapping_add(((y) as i32).wrapping_mul(32i32))).wrapping_sub(31i32))
                    as isize,
            ))
            .write(
                ((((((((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(2288))
                .cast::<u16>())
                .wrapping_offset(
                    ((((x) as i32).wrapping_add(((y) as i32).wrapping_mul(32i32)))
                        .wrapping_sub(36i32)) as isize,
                ))
                .read()) as i32)
                    | 1024i32) as u16),
            );
        }
        if (((((((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(81))
        .cast::<u8>())
        .wrapping_offset(((whichParty) as i32) as isize * 6))
        .cast::<u8>())
        .wrapping_offset(((monIdx) as i32) as isize))
        .read())
            != 0
        {
            symbolTile = 1152u32;
        } else {
            if ((whichParty) as i32) == 0i32 {
                gender = GetMonGender(
                    ((&raw mut gPlayerParty).cast::<u8>())
                        .wrapping_offset(((monIdx) as i32) as isize * 100),
                );
                GetMonData3(
                    ((&raw mut gPlayerParty).cast::<u8>())
                        .wrapping_offset(((monIdx) as i32) as isize * 100),
                    2i32,
                    (&raw mut nickname).cast::<u8>(),
                );
            } else {
                gender = GetMonGender(
                    ((&raw mut gEnemyParty).cast::<u8>())
                        .wrapping_offset(((monIdx) as i32) as isize * 100),
                );
                GetMonData3(
                    ((&raw mut gEnemyParty).cast::<u8>())
                        .wrapping_offset(((monIdx) as i32) as isize * 100),
                    2i32,
                    (&raw mut nickname).cast::<u8>(),
                );
            }
            'l1: {
                let __sw1 = ((gender) as i32);
                let __matched = __sw1 == 0i32 || __sw1 == 254i32;
                if __sw1 == 0i32 {
                    symbolTile =
                        ((if !((NameHasGenderSymbol((&raw mut nickname).cast::<u8>(), 0u8)) != 0) {
                            132i32
                        } else {
                            131i32
                        }) as u32);
                    break 'l1;
                }
                if __sw1 == 254i32 {
                    symbolTile =
                        ((if !((NameHasGenderSymbol((&raw mut nickname).cast::<u8>(), 254u8)) != 0)
                        {
                            133i32
                        } else {
                            131i32
                        }) as u32);
                    break 'l1;
                }
                if !__matched {
                    symbolTile = 131u32;
                    break 'l1;
                }
            }
        }
        ((((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(2288))
            .cast::<u16>())
        .wrapping_offset(
            ((((((y) as i32).wrapping_sub(1i32)).wrapping_mul(32i32)).wrapping_add(((x) as i32)))
                .wrapping_add(1i32)) as isize,
        ))
        .write(((symbolTile) as u16));
    }
}
pub(crate) unsafe extern "C" fn PrintPartyLevelsAndGenders(whichParty: u8) {
    unsafe {
        let mut whichParty = whichParty;
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i
                    < ((((((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(54))
                    .cast::<u8>())
                    .wrapping_offset(((whichParty) as i32) as isize))
                    .read()) as i32))
                {
                    break 'l1;
                }
                'l2: {
                    let mut j: i32 = (i).wrapping_add((6i32).wrapping_mul(((whichParty) as i32)));
                    PrintLevelAndGender(
                        whichParty,
                        ((i) as u8),
                        (((((&raw const sTradeMonLevelCoords).cast::<u8>().cast_mut())
                            .cast::<u8>())
                        .wrapping_offset((j) as isize * 2))
                        .cast::<u8>())
                        .read(),
                        ((((((&raw const sTradeMonLevelCoords).cast::<u8>().cast_mut())
                            .cast::<u8>())
                        .wrapping_offset((j) as isize * 2))
                        .cast::<u8>())
                        .wrapping_offset(1))
                        .read(),
                        (((((&raw const sTradeMonBoxCoords).cast::<u8>().cast_mut())
                            .cast::<u8>())
                        .wrapping_offset((j) as isize * 2))
                        .cast::<u8>())
                        .read(),
                        ((((((&raw const sTradeMonBoxCoords).cast::<u8>().cast_mut())
                            .cast::<u8>())
                        .wrapping_offset((j) as isize * 2))
                        .cast::<u8>())
                        .wrapping_offset(1))
                        .read(),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn ShowTradePartyMonIcons(whichParty: u8) {
    unsafe {
        let mut whichParty = whichParty;
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i
                    < ((((((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(54))
                    .cast::<u8>())
                    .wrapping_offset(((whichParty) as i32) as isize))
                    .read()) as i32))
                {
                    break 'l1;
                }
                'l2: {
                    crate::c::bf_write(
                        (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            ((((((((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(40))
                            .cast::<u8>())
                            .wrapping_offset(((whichParty) as i32) as isize * 6))
                            .cast::<u8>())
                            .wrapping_offset((i) as isize))
                            .read()) as i32) as isize
                                * 68,
                        ))
                        .wrapping_add(62),
                        2,
                        1,
                        (0u16) as i32,
                    );
                    ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(40))
                        .cast::<u8>())
                        .wrapping_offset(((whichParty) as i32) as isize * 6))
                        .cast::<u8>())
                        .wrapping_offset((i) as isize))
                        .read()) as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(32)
                    .cast::<i16>())
                    .write(
                        ((((((((((&raw const sTradeMonSpriteCoords).cast::<u8>().cast_mut())
                            .cast::<u8>())
                        .wrapping_offset(
                            ((((whichParty) as i32).wrapping_mul(6i32)).wrapping_add(i)) as isize
                                * 2,
                        ))
                        .cast::<u8>())
                        .read()) as i32)
                            .wrapping_mul(8i32))
                        .wrapping_add(14i32)) as i16),
                    );
                    ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(40))
                        .cast::<u8>())
                        .wrapping_offset(((whichParty) as i32) as isize * 6))
                        .cast::<u8>())
                        .wrapping_offset((i) as isize))
                        .read()) as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(34)
                    .cast::<i16>())
                    .write(
                        (((((((((((&raw const sTradeMonSpriteCoords).cast::<u8>().cast_mut())
                            .cast::<u8>())
                        .wrapping_offset(
                            ((((whichParty) as i32).wrapping_mul(6i32)).wrapping_add(i)) as isize
                                * 2,
                        ))
                        .cast::<u8>())
                        .wrapping_offset(1))
                        .read()) as i32)
                            .wrapping_mul(8i32))
                        .wrapping_sub(12i32)) as i16),
                    );
                    ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(40))
                        .cast::<u8>())
                        .wrapping_offset(((whichParty) as i32) as isize * 6))
                        .cast::<u8>())
                        .wrapping_offset((i) as isize))
                        .read()) as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(36)
                    .cast::<i16>())
                    .write(0i16);
                    ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(40))
                        .cast::<u8>())
                        .wrapping_offset(((whichParty) as i32) as isize * 6))
                        .cast::<u8>())
                        .wrapping_offset((i) as isize))
                        .read()) as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(38)
                    .cast::<i16>())
                    .write(0i16);
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn PrintTradePartnerPartyNicknames() {
    unsafe {
        rbox_fill_rectangle(1u8);
        PrintPartyNicknames(1u8);
    }
}
pub(crate) unsafe extern "C" fn RedrawPartyWindow(whichParty: u8) {
    unsafe {
        let mut whichParty = whichParty;
        CopyToBgTilemapBufferRect_ChangePalette(
            1u8,
            (((&raw const sTradePartyBoxTilemap)
                .cast::<u8>()
                .cast_mut()
                .cast::<u16>())
            .cast::<u16>())
            .cast::<u8>(),
            ((((whichParty) as i32).wrapping_mul(15i32)) as u8),
            0u8,
            15u8,
            17u8,
            0u8,
        );
        CopyBgTilemapBufferToVram(1u8);
        PrintPartyLevelsAndGenders(whichParty);
        PrintPartyNicknames(whichParty);
        ShowTradePartyMonIcons(whichParty);
        DrawBottomRowText(
            ((((&raw const sActionTexts)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u8>())
            .cast::<*mut u8>())
            .wrapping_offset(1))
            .read(),
            (((100728832i32).wrapping_add(
                ((((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(114)
                    .cast::<u16>())
                .read()) as i32)
                    .wrapping_mul(32i32),
            )) as usize as *mut u8),
            24u8,
        );
        ((((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(116))
            .cast::<u8>())
        .wrapping_offset(((whichParty) as i32) as isize))
        .write(0u8);
    }
}
pub(crate) unsafe extern "C" fn Task_DrawSelectionSummary(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        FillBgTilemapBufferRect_Palette0(
            0u8,
            0u16,
            0u8,
            0u8,
            ((crate::c::div_i32(240i32, 8i32)) as u8),
            ((crate::c::div_i32(160i32, 8i32)) as u8),
        );
        CopyBgTilemapBufferToVram(0u8);
    }
}
pub(crate) unsafe extern "C" fn Task_DrawSelectionTrade(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        FillBgTilemapBufferRect_Palette0(
            0u8,
            0u16,
            0u8,
            0u8,
            ((crate::c::div_i32(240i32, 8i32)) as u8),
            ((crate::c::div_i32(160i32, 8i32)) as u8),
        );
        CopyBgTilemapBufferToVram(0u8);
    }
}
pub(crate) unsafe extern "C" fn QueueAction(delay: u16, actionId: u8) {
    unsafe {
        let mut delay = delay;
        let mut actionId = actionId;
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < ((crate::c::div_u32(32u32, 8u32)) as i32)) {
                    break 'l1;
                }
                'l2: {
                    if !((((((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(2256))
                    .cast::<u8>())
                    .wrapping_offset((i) as isize * 8))
                    .read())
                        != 0)
                    {
                        (((((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(2256))
                        .cast::<u8>())
                        .wrapping_offset((i) as isize * 8))
                        .wrapping_add(2)
                        .cast::<u16>())
                        .write(delay);
                        (((((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(2256))
                        .cast::<u8>())
                        .wrapping_offset((i) as isize * 8))
                        .wrapping_add(4))
                        .write(actionId);
                        ((((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(2256))
                        .cast::<u8>())
                        .wrapping_offset((i) as isize * 8))
                        .write(1u8);
                        break 'l1;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn GetNumQueuedActions() -> u32 {
    unsafe {
        let mut numActions: u32 = 0u32;
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < ((crate::c::div_u32(32u32, 8u32)) as i32)) {
                    break 'l1;
                }
                'l2: {
                    numActions = (numActions).wrapping_add(
                        ((((((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(2256))
                        .cast::<u8>())
                        .wrapping_offset((i) as isize * 8))
                        .read()) as u32),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        return numActions;
    }
}
pub(crate) unsafe extern "C" fn DoQueuedActions() {
    unsafe {
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < ((crate::c::div_u32(32u32, 8u32)) as i32)) {
                    break 'l1;
                }
                'l2: {
                    if (((((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(2256))
                    .cast::<u8>())
                    .wrapping_offset((i) as isize * 8))
                    .read())
                        != 0
                    {
                        if (((((((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(2256))
                        .cast::<u8>())
                        .wrapping_offset((i) as isize * 8))
                        .wrapping_add(2)
                        .cast::<u16>())
                        .read()) as i32)
                            != 0i32
                        {
                            let __p1 =
                                ((((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(2256))
                                .cast::<u8>())
                                .wrapping_offset((i) as isize * 8))
                                .wrapping_add(2)
                                .cast::<u16>();
                            (__p1).write(((__p1).read()).wrapping_sub(1));
                        } else {
                            'l3: {
                                let __sw2 = (((((((((&raw mut sTradeMenu)
                                    .cast::<u8>()
                                    .cast::<*mut u8>())
                                .read())
                                .wrapping_add(2256))
                                .cast::<u8>())
                                .wrapping_offset((i) as isize * 8))
                                .wrapping_add(4))
                                .read()) as i32);
                                if __sw2 == 0i32 {
                                    SendLinkData(
                                        (((((&raw mut sTradeMenu)
                                            .cast::<u8>()
                                            .cast::<*mut u8>())
                                        .read())
                                        .wrapping_add(128))
                                        .cast::<u16>())
                                        .cast::<u8>(),
                                        20u32,
                                    );
                                    break 'l3;
                                }
                                if __sw2 == 1i32 {
                                    PrintTradeMessage(0u8);
                                    break 'l3;
                                }
                                if __sw2 == 2i32 {
                                    PrintTradeMessage(2u8);
                                    break 'l3;
                                }
                                if __sw2 == 3i32 || __sw2 == 4i32 || __sw2 == 5i32 {
                                    PrintTradeMessage(3u8);
                                    break 'l3;
                                }
                                if __sw2 == 6i32 {
                                    PrintTradeMessage(6u8);
                                    break 'l3;
                                }
                                if __sw2 == 7i32 {
                                    PrintTradeMessage(7u8);
                                    break 'l3;
                                }
                                if __sw2 == 8i32 {
                                    PrintTradeMessage(8u8);
                                    break 'l3;
                                }
                            }
                            ((((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(2256))
                            .cast::<u8>())
                            .wrapping_offset((i) as isize * 8))
                            .write(0u8);
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn PrintTradeMessage(messageId: u8) {
    unsafe {
        let mut messageId = messageId;
        FillWindowPixelBuffer(0u8, 17u8);
        AddTextPrinterParameterized(
            0u8,
            1u8,
            ((((&raw const sMessages)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u8>())
            .cast::<*mut u8>())
            .wrapping_offset(((messageId) as i32) as isize))
            .read(),
            0u8,
            1u8,
            255u8,
            None,
        );
        DrawTextBorderOuter(0u8, 20u16, 12u8);
        PutWindowTilemap(0u8);
        CopyWindowToVram(0u8, 3u8);
    }
}
pub(crate) unsafe extern "C" fn LoadUISpriteGfx() -> u8 {
    unsafe {
        let mut sheet = crate::ffi::Align4([0u8; 8]);
        if ((((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(168))
            .read()) as i32)
            < 14i32
        {
            (((&raw mut sheet).cast::<u8>()).cast::<*mut u8>()).write(
                ((((&raw mut sMenuTextTileBuffers)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .cast::<*mut u8>())
                .wrapping_offset(
                    ((((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(168))
                    .read()) as i32) as isize,
                ))
                .read(),
            );
            (((&raw mut sheet).cast::<u8>())
                .wrapping_add(4)
                .cast::<u16>())
            .write(256u16);
            (((&raw mut sheet).cast::<u8>())
                .wrapping_add(6)
                .cast::<u16>())
            .write(
                (((200i32).wrapping_add(
                    ((((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(168))
                    .read()) as i32),
                )) as u16),
            );
        }
        'l1: {
            let __sw1 = ((((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(168))
            .read()) as i32);
            if __sw1 == 0i32
                || __sw1 == 1i32
                || __sw1 == 2i32
                || __sw1 == 3i32
                || __sw1 == 4i32
                || __sw1 == 5i32
                || __sw1 == 6i32
                || __sw1 == 7i32
            {
                LoadSpriteSheet((&raw mut sheet).cast::<u8>());
                let __p2 = (((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(168);
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 8i32 {
                ((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(114)
                    .cast::<u16>())
                .write(LoadSpriteSheet((&raw mut sheet).cast::<u8>()));
                let __p3 = (((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(168);
                (__p3).write(((__p3).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 9i32 || __sw1 == 10i32 || __sw1 == 11i32 || __sw1 == 12i32 || __sw1 == 13i32
            {
                LoadSpriteSheet((&raw mut sheet).cast::<u8>());
                let __p4 = (((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(168);
                (__p4).write(((__p4).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 14i32 {
                LoadSpritePalette((&raw const sSpritePalette_MenuText).cast::<u8>().cast_mut());
                let __p5 = (((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(168);
                (__p5).write(((__p5).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 15i32 {
                LoadSpritePalette((&raw const sCursor_SpritePalette).cast::<u8>().cast_mut());
                let __p6 = (((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(168);
                (__p6).write(((__p6).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 16i32 {
                LoadSpriteSheet((&raw const sCursor_SpriteSheet).cast::<u8>().cast_mut());
                let __p7 = (((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(168);
                (__p7).write(((__p7).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 17i32 {
                ((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(168))
                    .write(0u8);
                return 1u8;
            }
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn DrawBottomRowText(str: *mut u8, dest: *mut u8, unused: u8) {
    unsafe {
        let mut str = str;
        let mut dest = dest;
        let mut unused = unused;
        DrawTextWindowAndBufferTiles(str, dest, 0u8, 0u8, 6i32);
    }
}
pub(crate) unsafe extern "C" fn ComputePartyTradeableFlags(whichParty: u8) {
    unsafe {
        let mut whichParty = whichParty;
        let mut i: i32 = 0i32;
        'l1: {
            let __sw1 = ((whichParty) as i32);
            if __sw1 == 0i32 {
                {
                    i = 0i32;
                    'l2: loop {
                        if !(i
                            < ((((((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(54))
                            .cast::<u8>())
                            .wrapping_offset(((whichParty) as i32) as isize))
                            .read()) as i32))
                        {
                            break 'l2;
                        }
                        'l3: {
                            if GetMonData2(
                                ((&raw mut gPlayerParty).cast::<u8>())
                                    .wrapping_offset((i) as isize * 100),
                                45i32,
                            ) == 1u32
                            {
                                ((((((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(69))
                                .cast::<u8>())
                                .wrapping_offset(((whichParty) as i32) as isize * 6))
                                .cast::<u8>())
                                .wrapping_offset((i) as isize))
                                .write(0u8);
                                ((((((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(81))
                                .cast::<u8>())
                                .wrapping_offset(((whichParty) as i32) as isize * 6))
                                .cast::<u8>())
                                .wrapping_offset((i) as isize))
                                .write(1u8);
                            } else {
                                if GetMonData2(
                                    ((&raw mut gPlayerParty).cast::<u8>())
                                        .wrapping_offset((i) as isize * 100),
                                    57i32,
                                ) == 0u32
                                {
                                    ((((((((&raw mut sTradeMenu)
                                        .cast::<u8>()
                                        .cast::<*mut u8>())
                                    .read())
                                    .wrapping_add(69))
                                    .cast::<u8>())
                                    .wrapping_offset(((whichParty) as i32) as isize * 6))
                                    .cast::<u8>())
                                    .wrapping_offset((i) as isize))
                                    .write(0u8);
                                    ((((((((&raw mut sTradeMenu)
                                        .cast::<u8>()
                                        .cast::<*mut u8>())
                                    .read())
                                    .wrapping_add(81))
                                    .cast::<u8>())
                                    .wrapping_offset(((whichParty) as i32) as isize * 6))
                                    .cast::<u8>())
                                    .wrapping_offset((i) as isize))
                                    .write(0u8);
                                } else {
                                    ((((((((&raw mut sTradeMenu)
                                        .cast::<u8>()
                                        .cast::<*mut u8>())
                                    .read())
                                    .wrapping_add(69))
                                    .cast::<u8>())
                                    .wrapping_offset(((whichParty) as i32) as isize * 6))
                                    .cast::<u8>())
                                    .wrapping_offset((i) as isize))
                                    .write(1u8);
                                    ((((((((&raw mut sTradeMenu)
                                        .cast::<u8>()
                                        .cast::<*mut u8>())
                                    .read())
                                    .wrapping_add(81))
                                    .cast::<u8>())
                                    .wrapping_offset(((whichParty) as i32) as isize * 6))
                                    .cast::<u8>())
                                    .wrapping_offset((i) as isize))
                                    .write(0u8);
                                }
                            }
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                {
                    i = 0i32;
                    'l4: loop {
                        if !(i
                            < ((((((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(54))
                            .cast::<u8>())
                            .wrapping_offset(((whichParty) as i32) as isize))
                            .read()) as i32))
                        {
                            break 'l4;
                        }
                        'l5: {
                            if GetMonData2(
                                ((&raw mut gEnemyParty).cast::<u8>())
                                    .wrapping_offset((i) as isize * 100),
                                45i32,
                            ) == 1u32
                            {
                                ((((((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(69))
                                .cast::<u8>())
                                .wrapping_offset(((whichParty) as i32) as isize * 6))
                                .cast::<u8>())
                                .wrapping_offset((i) as isize))
                                .write(0u8);
                                ((((((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(81))
                                .cast::<u8>())
                                .wrapping_offset(((whichParty) as i32) as isize * 6))
                                .cast::<u8>())
                                .wrapping_offset((i) as isize))
                                .write(1u8);
                            } else {
                                if GetMonData2(
                                    ((&raw mut gEnemyParty).cast::<u8>())
                                        .wrapping_offset((i) as isize * 100),
                                    57i32,
                                ) == 0u32
                                {
                                    ((((((((&raw mut sTradeMenu)
                                        .cast::<u8>()
                                        .cast::<*mut u8>())
                                    .read())
                                    .wrapping_add(69))
                                    .cast::<u8>())
                                    .wrapping_offset(((whichParty) as i32) as isize * 6))
                                    .cast::<u8>())
                                    .wrapping_offset((i) as isize))
                                    .write(0u8);
                                    ((((((((&raw mut sTradeMenu)
                                        .cast::<u8>()
                                        .cast::<*mut u8>())
                                    .read())
                                    .wrapping_add(81))
                                    .cast::<u8>())
                                    .wrapping_offset(((whichParty) as i32) as isize * 6))
                                    .cast::<u8>())
                                    .wrapping_offset((i) as isize))
                                    .write(0u8);
                                } else {
                                    ((((((((&raw mut sTradeMenu)
                                        .cast::<u8>()
                                        .cast::<*mut u8>())
                                    .read())
                                    .wrapping_add(69))
                                    .cast::<u8>())
                                    .wrapping_offset(((whichParty) as i32) as isize * 6))
                                    .cast::<u8>())
                                    .wrapping_offset((i) as isize))
                                    .write(1u8);
                                    ((((((((&raw mut sTradeMenu)
                                        .cast::<u8>()
                                        .cast::<*mut u8>())
                                    .read())
                                    .wrapping_add(81))
                                    .cast::<u8>())
                                    .wrapping_offset(((whichParty) as i32) as isize * 6))
                                    .cast::<u8>())
                                    .wrapping_offset((i) as isize))
                                    .write(0u8);
                                }
                            }
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn ComputePartyHPBarLevels(whichParty: u8) {
    unsafe {
        let mut whichParty = whichParty;
        let mut i: u16 = 0u16;
        let mut curHp: u16 = 0u16;
        let mut maxHp: u16 = 0u16;
        'l1: {
            let __sw1 = ((whichParty) as i32);
            if __sw1 == 0i32 {
                {
                    i = 0u16;
                    'l2: loop {
                        if !(((i) as i32)
                            < (((((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(54))
                            .cast::<u8>())
                            .read()) as i32))
                        {
                            break 'l2;
                        }
                        'l3: {
                            curHp = ((GetMonData2(
                                ((&raw mut gPlayerParty).cast::<u8>())
                                    .wrapping_offset(((i) as i32) as isize * 100),
                                57i32,
                            )) as u16);
                            maxHp = ((GetMonData2(
                                ((&raw mut gPlayerParty).cast::<u8>())
                                    .wrapping_offset(((i) as i32) as isize * 100),
                                58i32,
                            )) as u16);
                            (((((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(93))
                            .cast::<u8>())
                            .cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize))
                            .write(GetHPBarLevel(((curHp) as i16), ((maxHp) as i16)));
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                {
                    i = 0u16;
                    'l4: loop {
                        if !(((i) as i32)
                            < ((((((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(54))
                            .cast::<u8>())
                            .wrapping_offset(1))
                            .read()) as i32))
                        {
                            break 'l4;
                        }
                        'l5: {
                            curHp = ((GetMonData2(
                                ((&raw mut gEnemyParty).cast::<u8>())
                                    .wrapping_offset(((i) as i32) as isize * 100),
                                57i32,
                            )) as u16);
                            maxHp = ((GetMonData2(
                                ((&raw mut gEnemyParty).cast::<u8>())
                                    .wrapping_offset(((i) as i32) as isize * 100),
                                58i32,
                            )) as u16);
                            ((((((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(93))
                            .cast::<u8>())
                            .wrapping_offset(6))
                            .cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize))
                            .write(GetHPBarLevel(((curHp) as i16), ((maxHp) as i16)));
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SetTradePartyHPBarSprites() {
    unsafe {
        let mut i: i32 = 0i32;
        let mut j: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 2i32) {
                    break 'l1;
                }
                'l2: {
                    {
                        j = 0i32;
                        'l3: loop {
                            if !(j
                                < ((((((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(54))
                                .cast::<u8>())
                                .wrapping_offset((i) as isize))
                                .read()) as i32))
                            {
                                break 'l3;
                            }
                            'l4: {
                                SetPartyHPBarSprite(
                                    ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                        ((((((((((&raw mut sTradeMenu)
                                            .cast::<u8>()
                                            .cast::<*mut u8>())
                                        .read())
                                        .wrapping_add(40))
                                        .cast::<u8>())
                                        .wrapping_offset((i) as isize * 6))
                                        .cast::<u8>())
                                        .wrapping_offset((j) as isize))
                                        .read()) as i32)
                                            as isize
                                            * 68,
                                    ),
                                    (((4i32).wrapping_sub(
                                        ((((((((((&raw mut sTradeMenu)
                                            .cast::<u8>()
                                            .cast::<*mut u8>())
                                        .read())
                                        .wrapping_add(93))
                                        .cast::<u8>())
                                        .wrapping_offset((i) as isize * 6))
                                        .cast::<u8>())
                                        .wrapping_offset((j) as isize))
                                        .read()) as i32),
                                    )) as u8),
                                );
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
pub(crate) unsafe extern "C" fn SaveTradeGiftRibbons() {
    unsafe {
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < ((crate::c::div_u32(11u32, 1u32)) as i32)) {
                    break 'l1;
                }
                'l2: {
                    if (((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(12712))
                    .cast::<u8>())
                    .wrapping_offset((i) as isize))
                    .read()) as i32)
                        == 0i32)
                        && (((((((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(169))
                        .cast::<u8>())
                        .wrapping_offset((i) as isize))
                        .read()) as i32)
                            != 0i32)
                    {
                        if ((((((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(169))
                        .cast::<u8>())
                        .wrapping_offset((i) as isize))
                        .read()) as i32)
                            < 64i32
                        {
                            ((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(12712))
                            .cast::<u8>())
                            .wrapping_offset((i) as isize))
                            .write(
                                ((((((&raw mut sTradeMenu).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(169))
                                .cast::<u8>())
                                .wrapping_offset((i) as isize))
                                .read(),
                            );
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn CanTradeSelectedMon(
    playerParty: *mut u8,
    partyCount: i32,
    monIdx: i32,
) -> u32 {
    unsafe {
        let mut playerParty = playerParty;
        let mut partyCount = partyCount;
        let mut monIdx = monIdx;
        let mut i: i32 = 0i32;
        let mut numMonsLeft: i32 = 0i32;
        let mut partner: *mut u8 = core::ptr::null_mut();
        let mut species = crate::ffi::Align4([0u8; 24]);
        let mut species2 = crate::ffi::Align4([0u8; 24]);
        {
            i = 0i32;
            'l1: loop {
                if !(i < partyCount) {
                    break 'l1;
                }
                'l2: {
                    (((&raw mut species2).cast::<u32>()).wrapping_offset((i) as isize)).write(
                        GetMonData2((playerParty).wrapping_offset((i) as isize * 100), 65i32),
                    );
                    (((&raw mut species).cast::<u32>()).wrapping_offset((i) as isize)).write(
                        GetMonData2((playerParty).wrapping_offset((i) as isize * 100), 11i32),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        if !((IsNationalPokedexEnabled()) != 0) {
            if (((&raw mut species2).cast::<u32>()).wrapping_offset((monIdx) as isize)).read()
                == 412u32
            {
                return 3u32;
            }
            if !((IsSpeciesInHoennDex(
                (((((&raw mut species2).cast::<u32>()).wrapping_offset((monIdx) as isize)).read())
                    as u16),
            )) != 0)
            {
                return 2u32;
            }
        }
        partner = ((&raw mut gLinkPlayers).cast::<u8>())
            .wrapping_offset((((GetMultiplayerId()) as i32) ^ 1i32) as isize * 28);
        if ((((((partner).cast::<u16>()).read()) as i32) & 255i32) != 2i32)
            && ((((((partner).cast::<u16>()).read()) as i32) & 255i32) != 1i32)
        {
            if !((((((partner).wrapping_add(18)).read()) as i32) & 15i32) != 0) {
                if (((&raw mut species2).cast::<u32>()).wrapping_offset((monIdx) as isize)).read()
                    == 412u32
                {
                    return 5u32;
                }
                if !((IsSpeciesInHoennDex(
                    (((((&raw mut species2).cast::<u32>()).wrapping_offset((monIdx) as isize))
                        .read()) as u16),
                )) != 0)
                {
                    return 4u32;
                }
            }
        }
        if ((((&raw mut species).cast::<u32>()).wrapping_offset((monIdx) as isize)).read()
            == 410u32)
            || ((((&raw mut species).cast::<u32>()).wrapping_offset((monIdx) as isize)).read()
                == 151u32)
        {
            if !((GetMonData2(
                (playerParty).wrapping_offset((monIdx) as isize * 100),
                80i32,
            )) != 0)
            {
                return 4u32;
            }
        }
        {
            i = 0i32;
            'l3: loop {
                if !(i < partyCount) {
                    break 'l3;
                }
                'l4: {
                    if (((&raw mut species2).cast::<u32>()).wrapping_offset((i) as isize)).read()
                        == 412u32
                    {
                        (((&raw mut species2).cast::<u32>()).wrapping_offset((i) as isize))
                            .write(0u32);
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        {
            numMonsLeft = 0i32;
            i = 0i32;
            'l5: loop {
                if !(i < partyCount) {
                    break 'l5;
                }
                'l6: {
                    if i != monIdx {
                        numMonsLeft = ((((numMonsLeft) as u32).wrapping_add(
                            (((&raw mut species2).cast::<u32>()).wrapping_offset((i) as isize))
                                .read(),
                        )) as i32);
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        if numMonsLeft != 0i32 {
            return 0u32;
        } else {
            return 1u32;
        }
        #[allow(unreachable_code)]
        {
            return 0u32;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetGameProgressForLinkTrade() -> i32 {
    unsafe {
        let mut versionId: i32 = 0i32;
        let mut version: u16 = 0u16;
        if (((&raw mut gReceivedRemoteLinkPlayers).cast::<u8>()).read()) != 0 {
            versionId = 0i32;
            version = ((((((((&raw mut gLinkPlayers).cast::<u8>())
                .wrapping_offset((((GetMultiplayerId()) as i32) ^ 1i32) as isize * 28))
            .cast::<u16>())
            .read()) as i32)
                & 255i32) as u16);
            if ((((version) as i32) == 2i32) || (((version) as i32) == 1i32))
                || (((version) as i32) == 3i32)
            {
                versionId = 0i32;
            } else {
                if (((version) as i32) == 4i32) || (((version) as i32) == 5i32) {
                    versionId = 2i32;
                }
            }
            if versionId > 0i32 {
                if (((((((&raw mut gLinkPlayers).cast::<u8>())
                    .wrapping_offset(((GetMultiplayerId()) as i32) as isize * 28))
                .wrapping_add(18))
                .read()) as i32)
                    & 240i32)
                    != 0
                {
                    if versionId == 2i32 {
                        if (((((((&raw mut gLinkPlayers).cast::<u8>()).wrapping_offset(
                            (((GetMultiplayerId()) as i32) ^ 1i32) as isize * 28,
                        ))
                        .wrapping_add(18))
                        .read()) as i32)
                            & 240i32)
                            != 0
                        {
                            return 0i32;
                        } else {
                            return 2i32;
                        }
                    }
                } else {
                    return 1i32;
                }
            }
        }
        return 0i32;
    }
}
pub(crate) unsafe extern "C" fn IsDeoxysOrMewUntradable(
    species: u16,
    isModernFatefulEncounter: u8,
) -> u32 {
    unsafe {
        let mut species = species;
        let mut isModernFatefulEncounter = isModernFatefulEncounter;
        if (((species) as i32) == 410i32) || (((species) as i32) == 151i32) {
            if !((isModernFatefulEncounter) != 0) {
                return 1u32;
            }
        }
        return 0u32;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetUnionRoomTradeMessageId(
    player__v: crate::c::Rec4<4>,
    partner__v: crate::c::Rec4<4>,
    playerSpecies2: u16,
    partnerSpecies: u16,
    requestedType: u8,
    playerSpecies: u16,
    isModernFatefulEncounter: u8,
) -> i32 {
    unsafe {
        let mut player = crate::ffi::Align4([0u8; 4]);
        (&raw mut player)
            .cast::<crate::c::Rec4<4>>()
            .write_unaligned(player__v);
        let mut partner = crate::ffi::Align4([0u8; 4]);
        (&raw mut partner)
            .cast::<crate::c::Rec4<4>>()
            .write_unaligned(partner__v);
        let mut playerSpecies2 = playerSpecies2;
        let mut partnerSpecies = partnerSpecies;
        let mut requestedType = requestedType;
        let mut playerSpecies = playerSpecies;
        let mut isModernFatefulEncounter = isModernFatefulEncounter;
        let mut playerHasNationalDex: u8 = ((crate::c::bf_read(
            ((&raw mut player).cast::<u8>()).wrapping_add(1),
            0,
            1,
            false,
        ) as u16) as u8);
        let mut playerCanLinkNationally: u8 = ((crate::c::bf_read(
            ((&raw mut player).cast::<u8>()).wrapping_add(0),
            7,
            1,
            false,
        ) as u16) as u8);
        let mut partnerHasNationalDex: u8 = ((crate::c::bf_read(
            ((&raw mut partner).cast::<u8>()).wrapping_add(1),
            0,
            1,
            false,
        ) as u16) as u8);
        let mut partnerCanLinkNationally: u8 = ((crate::c::bf_read(
            ((&raw mut partner).cast::<u8>()).wrapping_add(0),
            7,
            1,
            false,
        ) as u16) as u8);
        let mut partnerVersion: u8 = ((crate::c::bf_read(
            ((&raw mut partner).cast::<u8>()).wrapping_add(1),
            2,
            4,
            false,
        ) as u16) as u8);
        if ((partnerVersion) as i32) != 3i32 {
            if !((playerCanLinkNationally) != 0) {
                return 8i32;
            } else {
                if !((partnerCanLinkNationally) != 0) {
                    return 9i32;
                }
            }
        }
        if (IsDeoxysOrMewUntradable(playerSpecies, isModernFatefulEncounter)) != 0 {
            return 4i32;
        }
        if ((partnerSpecies) as i32) == 412i32 {
            if ((playerSpecies2) as i32) != ((partnerSpecies) as i32) {
                return 2i32;
            }
        } else {
            if ((((((((&raw mut gSpeciesInfo).cast::<u8>())
                .wrapping_offset(((playerSpecies2) as i32) as isize * 28))
            .wrapping_add(6))
            .cast::<u8>())
            .read()) as i32)
                != ((requestedType) as i32))
                && (((((((((&raw mut gSpeciesInfo).cast::<u8>())
                    .wrapping_offset(((playerSpecies2) as i32) as isize * 28))
                .wrapping_add(6))
                .cast::<u8>())
                .wrapping_offset(1))
                .read()) as i32)
                    != ((requestedType) as i32))
            {
                return 1i32;
            }
        }
        if (((playerSpecies2) as i32) == 412i32)
            && (((playerSpecies2) as i32) != ((partnerSpecies) as i32))
        {
            return 3i32;
        }
        if !((playerHasNationalDex) != 0) {
            if ((playerSpecies2) as i32) == 412i32 {
                return 6i32;
            }
            if !((IsSpeciesInHoennDex(playerSpecies2)) != 0) {
                return 4i32;
            }
            if !((IsSpeciesInHoennDex(partnerSpecies)) != 0) {
                return 5i32;
            }
        }
        if (!((partnerHasNationalDex) != 0)) && (!((IsSpeciesInHoennDex(playerSpecies2)) != 0)) {
            return 7i32;
        }
        return 0i32;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CanRegisterMonForTradingBoard(
    player__v: crate::c::Rec4<4>,
    species2: u16,
    species: u16,
    isModernFatefulEncounter: u8,
) -> i32 {
    unsafe {
        let mut player = crate::ffi::Align4([0u8; 4]);
        (&raw mut player)
            .cast::<crate::c::Rec4<4>>()
            .write_unaligned(player__v);
        let mut species2 = species2;
        let mut species = species;
        let mut isModernFatefulEncounter = isModernFatefulEncounter;
        let mut hasNationalDex: u8 = ((crate::c::bf_read(
            ((&raw mut player).cast::<u8>()).wrapping_add(1),
            0,
            1,
            false,
        ) as u16) as u8);
        if (IsDeoxysOrMewUntradable(species, isModernFatefulEncounter)) != 0 {
            return 1i32;
        }
        if (hasNationalDex) != 0 {
            return 0i32;
        }
        if ((species2) as i32) == 412i32 {
            return 2i32;
        }
        if (IsSpeciesInHoennDex(species2)) != 0 {
            return 0i32;
        }
        return 1i32;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CanSpinTradeMon(mon: *mut u8, monIdx: u16) -> i32 {
    unsafe {
        let mut mon = mon;
        let mut monIdx = monIdx;
        let mut i: i32 = 0i32;
        let mut version: i32 = 0i32;
        let mut versions: i32 = 0i32;
        let mut canTradeAnyMon: i32 = 0i32;
        let mut numMonsLeft: i32 = 0i32;
        let mut speciesArray = crate::ffi::Align4([0u8; 24]);
        {
            i = 0i32;
            'l1: loop {
                if !(i < ((((&raw mut gPlayerPartyCount).cast::<u8>()).read()) as i32)) {
                    break 'l1;
                }
                'l2: {
                    (((&raw mut speciesArray).cast::<i32>()).wrapping_offset((i) as isize)).write(
                        ((GetMonData2((mon).wrapping_offset((i) as isize * 100), 65i32)) as i32),
                    );
                    if (((&raw mut speciesArray).cast::<i32>()).wrapping_offset((i) as isize))
                        .read()
                        == 412i32
                    {
                        (((&raw mut speciesArray).cast::<i32>()).wrapping_offset((i) as isize))
                            .write(0i32);
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        versions = 0i32;
        canTradeAnyMon = 1i32;
        {
            i = 0i32;
            'l3: loop {
                if !(i < ((GetLinkPlayerCount()) as i32)) {
                    break 'l3;
                }
                'l4: {
                    version = (((((((&raw mut gLinkPlayers).cast::<u8>())
                        .wrapping_offset((i) as isize * 28))
                    .cast::<u16>())
                    .read()) as i32)
                        & 255i32);
                    if (version == 4i32) || (version == 5i32) {
                        versions = 0i32;
                    } else {
                        versions = (versions | 1i32);
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        {
            i = 0i32;
            'l5: loop {
                if !(i < ((GetLinkPlayerCount()) as i32)) {
                    break 'l5;
                }
                'l6: {
                    let mut player: *mut u8 =
                        ((&raw mut gLinkPlayers).cast::<u8>()).wrapping_offset((i) as isize * 28);
                    'l7: loop {
                        'l8: {
                            if !((((((player).wrapping_add(16)).read()) as i32) & 15i32) != 0) {
                                canTradeAnyMon = 0i32;
                            }
                            if ((versions) != 0)
                                && ((crate::c::div_i32(
                                    ((((player).wrapping_add(16)).read()) as i32),
                                    16i32,
                                )) != 0)
                            {
                                canTradeAnyMon = 0i32;
                            }
                        }
                        if !((0i32) != 0) {
                            break 'l7;
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        if canTradeAnyMon == 0i32 {
            if !((IsSpeciesInHoennDex(
                (((((&raw mut speciesArray).cast::<i32>())
                    .wrapping_offset(((monIdx) as i32) as isize))
                .read()) as u16),
            )) != 0)
            {
                return 2i32;
            }
            if (((&raw mut speciesArray).cast::<i32>()).wrapping_offset(((monIdx) as i32) as isize))
                .read()
                == 0i32
            {
                return 3i32;
            }
        }
        numMonsLeft = 0i32;
        {
            i = 0i32;
            'l9: loop {
                if !(i < ((((&raw mut gPlayerPartyCount).cast::<u8>()).read()) as i32)) {
                    break 'l9;
                }
                'l10: {
                    if ((monIdx) as i32) != i {
                        numMonsLeft = (numMonsLeft).wrapping_add(
                            (((&raw mut speciesArray).cast::<i32>()).wrapping_offset((i) as isize))
                                .read(),
                        );
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        if !((numMonsLeft) != 0) {
            return 1i32;
        } else {
            return 0i32;
        }
        #[allow(unreachable_code)]
        {
            return 0i32;
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_LinkMonGlow(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (({
            let __p1 = ((sprite).wrapping_add(46)).cast::<i16>();
            let __t2 = ((__p1).read()).wrapping_add(1);
            (__p1).write(__t2);
            __t2
        }) as i32)
            == 10i32
        {
            PlaySE(23u16);
            (((sprite).wrapping_add(46)).cast::<i16>()).write(0i16);
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_LinkMonGlowWireless(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (!((crate::c::bf_read((sprite).wrapping_add(62), 2, 1, false) as u16) != 0))
            && ((({
                let __p1 = ((sprite).wrapping_add(46)).cast::<i16>();
                let __t2 = ((__p1).read()).wrapping_add(1);
                (__p1).write(__t2);
                __t2
            }) as i32)
                == 10i32)
        {
            PlaySE(194u16);
            (((sprite).wrapping_add(46)).cast::<i16>()).write(0i16);
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_LinkMonShadow(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if !((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) != 0) {
            if (({
                let __p1 = ((sprite).wrapping_add(46)).cast::<i16>();
                let __t2 = ((__p1).read()).wrapping_add(1);
                (__p1).write(__t2);
                __t2
            }) as i32)
                == 12i32
            {
                (((sprite).wrapping_add(46)).cast::<i16>()).write(0i16);
            }
            LoadPalette(
                ((((&raw const sLinkMonShadow_Pal)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<u16>())
                .cast::<u16>())
                .wrapping_offset(
                    (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) as isize,
                ))
                .cast::<u8>(),
                ((((((crate::c::bf_read((sprite).wrapping_add(5), 4, 4, false) as u16) as i32)
                    .wrapping_add(16i32))
                .wrapping_mul(16i32))
                .wrapping_add(4i32)) as u16),
                2u16,
            );
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_CableEndSending(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let __p1 = ((sprite).wrapping_add(46)).cast::<i16>();
        (__p1).write(((__p1).read()).wrapping_add(1));
        let __p2 = (sprite).wrapping_add(38).cast::<i16>();
        (__p2).write(((__p2).read()).wrapping_add(1));
        if (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) == 10i32 {
            DestroySprite(sprite);
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_CableEndReceiving(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let __p1 = ((sprite).wrapping_add(46)).cast::<i16>();
        (__p1).write(((__p1).read()).wrapping_add(1));
        let __p2 = (sprite).wrapping_add(38).cast::<i16>();
        (__p2).write(((__p2).read()).wrapping_sub(1));
        if (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) == 10i32 {
            DestroySprite(sprite);
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_GbaScreen(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (({
            let __p1 = ((sprite).wrapping_add(46)).cast::<i16>();
            let __t2 = ((__p1).read()).wrapping_add(1);
            (__p1).write(__t2);
            __t2
        }) as i32)
            == 15i32
        {
            PlaySE(204u16);
            (((sprite).wrapping_add(46)).cast::<i16>()).write(0i16);
        }
    }
}
pub(crate) unsafe extern "C" fn SetTradeBGAffine() {
    unsafe {
        let mut affine = crate::ffi::Align4([0u8; 16]);
        DoBgAffineSet(
            (&raw mut affine).cast::<u8>(),
            ((((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(212)
                .cast::<u16>())
            .read()) as i32)
                .wrapping_mul(256i32)) as u32),
            ((((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(214)
                .cast::<u16>())
            .read()) as i32)
                .wrapping_mul(256i32)) as u32),
            ((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(220)
                .cast::<u16>())
            .read()) as i16),
            ((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(222)
                .cast::<u16>())
            .read()) as i16),
            ((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(232)
                .cast::<u16>())
            .read()) as i16),
            ((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(232)
                .cast::<u16>())
            .read()) as i16),
            ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(236)
                .cast::<u16>())
            .read(),
        );
        SetGpuReg(
            32u8,
            (((((&raw mut affine).cast::<u8>()).cast::<i16>()).read()) as u16),
        );
        SetGpuReg(
            34u8,
            (((((&raw mut affine).cast::<u8>())
                .wrapping_add(2)
                .cast::<i16>())
            .read()) as u16),
        );
        SetGpuReg(
            36u8,
            (((((&raw mut affine).cast::<u8>())
                .wrapping_add(4)
                .cast::<i16>())
            .read()) as u16),
        );
        SetGpuReg(
            38u8,
            (((((&raw mut affine).cast::<u8>())
                .wrapping_add(6)
                .cast::<i16>())
            .read()) as u16),
        );
        SetGpuReg(
            40u8,
            (((((&raw mut affine).cast::<u8>())
                .wrapping_add(8)
                .cast::<i32>())
            .read()) as u16),
        );
        SetGpuReg(
            42u8,
            (((((&raw mut affine).cast::<u8>())
                .wrapping_add(8)
                .cast::<i32>())
            .read()
                >> 16) as u16),
        );
        SetGpuReg(
            44u8,
            (((((&raw mut affine).cast::<u8>())
                .wrapping_add(12)
                .cast::<i32>())
            .read()) as u16),
        );
        SetGpuReg(
            46u8,
            (((((&raw mut affine).cast::<u8>())
                .wrapping_add(12)
                .cast::<i32>())
            .read()
                >> 16) as u16),
        );
    }
}
pub(crate) unsafe extern "C" fn SetTradeGpuRegs() {
    unsafe {
        let mut dispcnt: u16 = 0u16;
        SetGpuReg(
            22u8,
            ((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(224)
                .cast::<i16>())
            .read()) as u16),
        );
        SetGpuReg(
            20u8,
            ((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(226)
                .cast::<i16>())
            .read()) as u16),
        );
        dispcnt = GetGpuReg(0u8);
        if (((dispcnt) as i32) & 7i32) == 0i32 {
            SetGpuReg(
                26u8,
                ((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(228)
                    .cast::<i16>())
                .read()) as u16),
            );
            SetGpuReg(
                24u8,
                ((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(230)
                    .cast::<i16>())
                .read()) as u16),
            );
        } else {
            SetTradeBGAffine();
        }
    }
}
pub(crate) unsafe extern "C" fn VBlankCB_TradeAnim() {
    unsafe {
        SetTradeGpuRegs();
        LoadOam();
        ProcessSpriteCopyRequests();
        TransferPlttBuffer();
    }
}
pub(crate) unsafe extern "C" fn ClearLinkTimeoutTimer() {
    unsafe {
        ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(138)
            .cast::<u16>())
        .write(0u16);
        ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(136))
            .write(0u8);
        ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(137))
            .write(0u8);
    }
}
pub(crate) unsafe extern "C" fn CheckForLinkTimeout() {
    unsafe {
        if ((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(136))
            .read()) as i32)
            == ((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(137))
            .read()) as i32)
        {
            let __p1 = (((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(138)
                .cast::<u16>();
            (__p1).write(((__p1).read()).wrapping_add(1));
        } else {
            ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(138)
                .cast::<u16>())
            .write(0u16);
        }
        if ((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(138)
            .cast::<u16>())
        .read()) as i32)
            > 300i32
        {
            CloseLink();
            SetMainCallback2(Some(CB2_LinkError));
            ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(138)
                .cast::<u16>())
            .write(0u16);
            ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(137))
                .write(0u8);
            ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(136))
                .write(0u8);
        }
        ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(137)).write(
            ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(136))
                .read(),
        );
    }
}
pub(crate) unsafe extern "C" fn TradeGetMultiplayerId() -> u32 {
    unsafe {
        if (((&raw mut gReceivedRemoteLinkPlayers).cast::<u8>()).read()) != 0 {
            return ((GetMultiplayerId()) as u32);
        }
        return 0u32;
    }
}
pub(crate) unsafe extern "C" fn LoadTradeMonPic(whichParty: u8, state: u8) {
    unsafe {
        let mut whichParty = whichParty;
        let mut state = state;
        let mut pos: i32 = 0i32;
        let mut mon: *mut u8 = core::ptr::null_mut();
        let mut species: u16 = 0u16;
        let mut personality: u32 = 0u32;
        if ((whichParty) as i32) == 0i32 {
            mon = ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                (((((&raw mut gSelectedTradeMonPositions).cast::<u8>()).cast::<u8>()).read())
                    as i32) as isize
                    * 100,
            );
            pos = 1i32;
        }
        if ((whichParty) as i32) == 1i32 {
            mon = ((&raw mut gEnemyParty).cast::<u8>()).wrapping_offset(
                (crate::c::rem_i32(
                    ((((((&raw mut gSelectedTradeMonPositions).cast::<u8>()).cast::<u8>())
                        .wrapping_offset(1))
                    .read()) as i32),
                    6i32,
                )) as isize
                    * 100,
            );
            pos = 3i32;
        }
        'l1: {
            let __sw1 = ((state) as i32);
            if __sw1 == 0i32 {
                species = ((GetMonData2(mon, 65i32)) as u16);
                personality = GetMonData2(mon, 0i32);
                if ((whichParty) as i32) == 0i32 {
                    HandleLoadSpecialPokePic_2(
                        ((&raw mut gMonFrontPicTable).cast::<u8>())
                            .wrapping_offset(((species) as i32) as isize * 8),
                        ((((((&raw mut gMonSpritesGfxPtr).cast::<*mut u8>()).read())
                            .wrapping_add(4))
                        .cast::<*mut u8>())
                        .wrapping_offset(1))
                        .read(),
                        ((species) as i32),
                        personality,
                    );
                } else {
                    HandleLoadSpecialPokePic_DontHandleDeoxys(
                        ((&raw mut gMonFrontPicTable).cast::<u8>())
                            .wrapping_offset(((species) as i32) as isize * 8),
                        ((((((&raw mut gMonSpritesGfxPtr).cast::<*mut u8>()).read())
                            .wrapping_add(4))
                        .cast::<*mut u8>())
                        .wrapping_offset(
                            ((((whichParty) as i32).wrapping_mul(2i32)).wrapping_add(1i32))
                                as isize,
                        ))
                        .read(),
                        ((species) as i32),
                        personality,
                    );
                }
                LoadCompressedSpritePalette(GetMonSpritePalStruct(mon));
                ((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(240))
                .cast::<u16>())
                .wrapping_offset(((whichParty) as i32) as isize))
                .write(species);
                ((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(104))
                .cast::<u32>())
                .wrapping_offset(((whichParty) as i32) as isize))
                .write(personality);
                break 'l1;
            }
            if __sw1 == 1i32 {
                SetMultiuseSpriteTemplateToPokemon(
                    ((GetMonSpritePalStruct(mon)).wrapping_add(4).cast::<u16>()).read(),
                    ((pos) as u8),
                );
                ((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(142))
                .cast::<u8>())
                .wrapping_offset(((whichParty) as i32) as isize))
                .write(CreateSprite(
                    (&raw mut gMultiuseSpriteTemplate).cast::<u8>(),
                    120i16,
                    60i16,
                    6u8,
                ));
                crate::c::bf_write(
                    (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(142))
                        .cast::<u8>())
                        .wrapping_offset(((whichParty) as i32) as isize))
                        .read()) as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(62),
                    2,
                    1,
                    (1u16) as i32,
                );
                ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(142))
                    .cast::<u8>())
                    .wrapping_offset(((whichParty) as i32) as isize))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                .write(Some(SpriteCallbackDummy));
                break 'l1;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CB2_LinkTrade() {
    unsafe {
        'l1: {
            let __sw1 = (((((&raw mut gMain).cast::<u8>()).wrapping_add(1080)).read()) as i32);
            if __sw1 == 0i32 {
                if !((((&raw mut gReceivedRemoteLinkPlayers).cast::<u8>()).read()) != 0) {
                    ((&raw mut gLinkType).cast::<u16>()).write(4420u16);
                    CloseLink();
                }
                ((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).write(AllocZeroed(256u32));
                AllocateMonSpritesGfx();
                ResetTasks();
                ResetSpriteData();
                FreeAllSpritePalettes();
                SetVBlankCallback(Some(VBlankCB_TradeAnim));
                TradeAnimInit_LoadGfx();
                ClearLinkTimeoutTimer();
                let __p2 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p2).write(((__p2).read()).wrapping_add(1));
                ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(140)
                    .cast::<u16>())
                .write(0u16);
                ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(148)
                    .cast::<u16>())
                .write(0u16);
                ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(238))
                    .write(1u8);
                ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(212)
                    .cast::<u16>())
                .write(64u16);
                ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(214)
                    .cast::<u16>())
                .write(64u16);
                ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(216)
                    .cast::<u16>())
                .write(0u16);
                ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(218)
                    .cast::<u16>())
                .write(0u16);
                ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(220)
                    .cast::<u16>())
                .write(((crate::c::div_i32(240i32, 2i32)) as u16));
                ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(222)
                    .cast::<u16>())
                .write(((crate::c::div_i32(160i32, 2i32)) as u16));
                ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(232)
                    .cast::<u16>())
                .write(256u16);
                ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(236)
                    .cast::<u16>())
                .write(0u16);
                break 'l1;
            }
            if __sw1 == 1i32 {
                if !((((&raw mut gReceivedRemoteLinkPlayers).cast::<u8>()).read()) != 0) {
                    ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(250))
                    .write(1u8);
                    OpenLink();
                    let __p3 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                    (__p3).write(((__p3).read()).wrapping_add(1));
                    ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(100)
                        .cast::<u32>())
                    .write(0u32);
                } else {
                    (((&raw mut gMain).cast::<u8>()).wrapping_add(1080)).write(4u8);
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                if {
                    let __p4 = (((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(100)
                        .cast::<u32>();
                    let __t5 = ((__p4).read()).wrapping_add(1);
                    (__p4).write(__t5);
                    __t5
                } > 60u32
                {
                    ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(100)
                        .cast::<u32>())
                    .write(0u32);
                    let __p6 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                    (__p6).write(((__p6).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                if (IsLinkMaster()) != 0 {
                    if ((GetLinkPlayerCount_2()) as i32) >= ((GetSavedPlayerCount()) as i32) {
                        if {
                            let __p7 = (((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(100)
                            .cast::<u32>();
                            let __t8 = ((__p7).read()).wrapping_add(1);
                            (__p7).write(__t8);
                            __t8
                        } > 30u32
                        {
                            CheckShouldAdvanceLinkState();
                            let __p9 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                            (__p9).write(((__p9).read()).wrapping_add(1));
                        }
                    } else {
                        CheckForLinkTimeout();
                    }
                } else {
                    let __p10 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                    (__p10).write(((__p10).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 4i32 {
                CheckForLinkTimeout();
                if (((((&raw mut gReceivedRemoteLinkPlayers).cast::<u8>()).read()) as i32) == 1i32)
                    && (((IsLinkPlayerDataExchangeComplete()) as i32) == 1i32)
                {
                    let __p11 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                    (__p11).write(((__p11).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 5i32 {
                ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(114))
                    .write(0u8);
                ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(115))
                    .write(0u8);
                ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(147))
                    .write(0u8);
                LoadTradeMonPic(0u8, 0u8);
                let __p12 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p12).write(((__p12).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 6i32 {
                LoadTradeMonPic(0u8, 1u8);
                let __p13 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p13).write(((__p13).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 7i32 {
                LoadTradeMonPic(1u8, 0u8);
                let __p14 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p14).write(((__p14).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 8i32 {
                LoadTradeMonPic(1u8, 1u8);
                LinkTradeDrawWindow();
                let __p15 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p15).write(((__p15).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 9i32 {
                LoadTradeSequenceSpriteSheetsAndPalettes();
                LoadSpriteSheet((&raw const sPokeBallSpriteSheet).cast::<u8>().cast_mut());
                LoadSpritePalette((&raw const sPokeBallSpritePalette).cast::<u8>().cast_mut());
                let __p16 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p16).write(((__p16).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 10i32 {
                BeginNormalPaletteFade(4294967295u32, 0i8, 16u8, 0u8, 0u16);
                ShowBg(0u8);
                let __p17 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p17).write(((__p17).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 11i32 {
                InitTradeSequenceBgGpuRegs();
                BufferTradeSceneStrings();
                let __p18 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p18).write(((__p18).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 12i32 {
                if !((crate::c::bf_read(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
                    7,
                    1,
                    false,
                ) as u16)
                    != 0)
                {
                    if (((&raw mut gWirelessCommType).cast::<u8>()).read()) != 0 {
                        LoadWirelessStatusIndicatorSpriteGfx();
                        CreateWirelessStatusIndicatorSprite(0u8, 0u8);
                    }
                    SetMainCallback2(Some(CB2_UpdateLinkTrade));
                }
                break 'l1;
            }
        }
        RunTasks();
        RunTextPrinters();
        AnimateSprites();
        BuildOamBuffer();
        UpdatePaletteFade();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn InitTradeSequenceBgGpuRegs() {
    unsafe {
        SetTradeSequenceBgGpuRegs(5u8);
        SetTradeSequenceBgGpuRegs(0u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn LinkTradeDrawWindow() {
    unsafe {
        FillWindowPixelBuffer(0u8, 255u8);
        PutWindowTilemap(0u8);
        CopyWindowToVram(0u8, 3u8);
    }
}
pub(crate) unsafe extern "C" fn TradeAnimInit_LoadGfx() {
    unsafe {
        SetGpuReg(0u8, 0u16);
        ResetBgsAndClearDma3BusyFlags(0u32);
        InitBgsFromTemplates(
            0u8,
            ((&raw const sTradeSequenceBgTemplates)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>(),
            ((crate::c::div_u32(16u32, 4u32)) as u8),
        );
        ChangeBgX(0u8, 0i32, 0u8);
        ChangeBgY(0u8, 0i32, 0u8);
        SetBgTilemapBuffer(0u8, Alloc(2048u32));
        SetBgTilemapBuffer(1u8, Alloc(2048u32));
        SetBgTilemapBuffer(3u8, Alloc(2048u32));
        DeactivateAllTextPrinters();
        DecompressAndLoadBgGfxUsingHeap(
            0u8,
            (((&raw mut gBattleTextboxTiles).cast::<u32>()).cast::<u32>()).cast::<u8>(),
            0u32,
            0u16,
            0u8,
        );
        LZDecompressWram(
            ((&raw mut gBattleTextboxTilemap).cast::<u32>()).cast::<u32>(),
            (&raw mut gDecompressionBuffer).cast::<u8>(),
        );
        CopyToBgTilemapBuffer(
            0u8,
            (&raw mut gDecompressionBuffer).cast::<u8>(),
            2048u16,
            0u16,
        );
        LoadCompressedPalette(
            ((&raw mut gBattleTextboxPalette).cast::<u32>()).cast::<u32>(),
            0u16,
            32u16,
        );
        InitWindows(
            ((&raw const sTradeSequenceWindowTemplates)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>(),
        );
        DecompressAndLoadBgGfxUsingHeap(
            0u8,
            (((&raw mut gBattleTextboxTiles).cast::<u32>()).cast::<u32>()).cast::<u8>(),
            0u32,
            0u16,
            0u8,
        );
        LZDecompressWram(
            ((&raw mut gBattleTextboxTilemap).cast::<u32>()).cast::<u32>(),
            (&raw mut gDecompressionBuffer).cast::<u8>(),
        );
        CopyToBgTilemapBuffer(
            0u8,
            (&raw mut gDecompressionBuffer).cast::<u8>(),
            2048u16,
            0u16,
        );
        LoadCompressedPalette(
            ((&raw mut gBattleTextboxPalette).cast::<u32>()).cast::<u32>(),
            0u16,
            32u16,
        );
    }
}
pub(crate) unsafe extern "C" fn CB2_InitInGameTrade() {
    unsafe {
        let mut otName = crate::ffi::Align4([0u8; 11]);
        'l1: {
            let __sw1 = (((((&raw mut gMain).cast::<u8>()).wrapping_add(1080)).read()) as i32);
            if __sw1 == 0i32 {
                (((&raw mut gSelectedTradeMonPositions).cast::<u8>()).cast::<u8>())
                    .write(((((&raw mut gSpecialVar_0x8005).cast::<u16>()).read()) as u8));
                ((((&raw mut gSelectedTradeMonPositions).cast::<u8>()).cast::<u8>())
                    .wrapping_offset(1))
                .write(6u8);
                StringCopy(
                    (((&raw mut gLinkPlayers).cast::<u8>()).wrapping_add(8)).cast::<u8>(),
                    (((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).cast::<u8>(),
                );
                GetMonData3(
                    (&raw mut gEnemyParty).cast::<u8>(),
                    7i32,
                    (&raw mut otName).cast::<u8>(),
                );
                StringCopy(
                    ((((&raw mut gLinkPlayers).cast::<u8>()).wrapping_offset(28)).wrapping_add(8))
                        .cast::<u8>(),
                    (&raw mut otName).cast::<u8>(),
                );
                (((&raw mut gLinkPlayers).cast::<u8>())
                    .wrapping_add(26)
                    .cast::<u16>())
                .write(2u16);
                ((((&raw mut gLinkPlayers).cast::<u8>()).wrapping_offset(28))
                    .wrapping_add(26)
                    .cast::<u16>())
                .write(((GetMonData2((&raw mut gEnemyParty).cast::<u8>(), 3i32)) as u16));
                ((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).write(AllocZeroed(256u32));
                AllocateMonSpritesGfx();
                ResetTasks();
                ResetSpriteData();
                FreeAllSpritePalettes();
                SetVBlankCallback(Some(VBlankCB_TradeAnim));
                TradeAnimInit_LoadGfx();
                ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(238))
                    .write(0u8);
                ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(140)
                    .cast::<u16>())
                .write(0u16);
                ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(148)
                    .cast::<u16>())
                .write(0u16);
                ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(212)
                    .cast::<u16>())
                .write(64u16);
                ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(214)
                    .cast::<u16>())
                .write(64u16);
                ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(216)
                    .cast::<u16>())
                .write(0u16);
                ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(218)
                    .cast::<u16>())
                .write(0u16);
                ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(220)
                    .cast::<u16>())
                .write(((crate::c::div_i32(240i32, 2i32)) as u16));
                ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(222)
                    .cast::<u16>())
                .write(((crate::c::div_i32(160i32, 2i32)) as u16));
                ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(232)
                    .cast::<u16>())
                .write(256u16);
                ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(236)
                    .cast::<u16>())
                .write(0u16);
                ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(100)
                    .cast::<u32>())
                .write(0u32);
                (((&raw mut gMain).cast::<u8>()).wrapping_add(1080)).write(5u8);
                break 'l1;
            }
            if __sw1 == 5i32 {
                LoadTradeMonPic(0u8, 0u8);
                let __p2 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 6i32 {
                LoadTradeMonPic(0u8, 1u8);
                let __p3 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p3).write(((__p3).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 7i32 {
                LoadTradeMonPic(1u8, 0u8);
                ShowBg(0u8);
                let __p4 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p4).write(((__p4).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 8i32 {
                LoadTradeMonPic(1u8, 1u8);
                FillWindowPixelBuffer(0u8, 255u8);
                PutWindowTilemap(0u8);
                CopyWindowToVram(0u8, 3u8);
                let __p5 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p5).write(((__p5).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 9i32 {
                LoadTradeSequenceSpriteSheetsAndPalettes();
                LoadSpriteSheet((&raw const sPokeBallSpriteSheet).cast::<u8>().cast_mut());
                LoadSpritePalette((&raw const sPokeBallSpritePalette).cast::<u8>().cast_mut());
                let __p6 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p6).write(((__p6).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 10i32 {
                ShowBg(0u8);
                let __p7 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p7).write(((__p7).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 11i32 {
                SetTradeSequenceBgGpuRegs(5u8);
                SetTradeSequenceBgGpuRegs(0u8);
                BufferTradeSceneStrings();
                let __p8 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p8).write(((__p8).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 12i32 {
                SetMainCallback2(Some(CB2_InGameTrade));
                break 'l1;
            }
        }
        RunTasks();
        RunTextPrinters();
        AnimateSprites();
        BuildOamBuffer();
        UpdatePaletteFade();
    }
}
pub(crate) unsafe extern "C" fn UpdatePokedexForReceivedMon(partyIdx: u8) {
    unsafe {
        let mut partyIdx = partyIdx;
        let mut mon: *mut u8 = ((&raw mut gPlayerParty).cast::<u8>())
            .wrapping_offset(((partyIdx) as i32) as isize * 100);
        if !((GetMonData2(mon, 45i32)) != 0) {
            let mut species: u16 = ((GetMonData3(mon, 11i32, core::ptr::null_mut())) as u16);
            let mut personality: u32 = GetMonData3(mon, 0i32, core::ptr::null_mut());
            species = SpeciesToNationalPokedexNum(species);
            GetSetPokedexFlag(species, 2u8);
            HandleSetPokedexFlag(species, 3u8, personality);
        }
    }
}
pub(crate) unsafe extern "C" fn TryEnableNationalDexFromLinkPartner() {
    unsafe {
        let mut mpId: u8 = GetMultiplayerId();
    }
}
pub(crate) unsafe extern "C" fn TradeMons(playerPartyIdx: u8, partnerPartyIdx: u8) {
    unsafe {
        let mut playerPartyIdx = playerPartyIdx;
        let mut partnerPartyIdx = partnerPartyIdx;
        let mut friendship: u8 = 0u8;
        let mut playerMon: *mut u8 = ((&raw mut gPlayerParty).cast::<u8>())
            .wrapping_offset(((playerPartyIdx) as i32) as isize * 100);
        let mut playerMail: u16 = ((GetMonData2(playerMon, 64i32)) as u16);
        let mut partnerMon: *mut u8 = ((&raw mut gEnemyParty).cast::<u8>())
            .wrapping_offset(((partnerPartyIdx) as i32) as isize * 100);
        let mut partnerMail: u16 = ((GetMonData2(partnerMon, 64i32)) as u16);
        if ((playerMail) as i32) != 255i32 {
            ClearMail(
                (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(11232))
                    .cast::<u8>())
                .wrapping_offset(((playerMail) as i32) as isize * 36),
            );
        }
        {
            (((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                .cast::<crate::c::Rec4<100>>()
                .write_unaligned(playerMon.cast::<crate::c::Rec4<100>>().read_unaligned());
            playerMon
                .cast::<crate::c::Rec4<100>>()
                .write_unaligned(partnerMon.cast::<crate::c::Rec4<100>>().read_unaligned());
            partnerMon.cast::<crate::c::Rec4<100>>().write_unaligned(
                (((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                    .cast::<crate::c::Rec4<100>>()
                    .read_unaligned(),
            );
        }
        friendship = 70u8;
        if !((GetMonData2(playerMon, 45i32)) != 0) {
            SetMonData(playerMon, 32i32, &raw mut friendship);
        }
        if ((partnerMail) as i32) != 255i32 {
            GiveMailToMon(
                playerMon,
                (((&raw mut gTradeMail).cast::<u8>()).cast::<u8>())
                    .wrapping_offset(((partnerMail) as i32) as isize * 36),
            );
        }
        UpdatePokedexForReceivedMon(playerPartyIdx);
        if (((&raw mut gReceivedRemoteLinkPlayers).cast::<u8>()).read()) != 0 {
            TryEnableNationalDexFromLinkPartner();
        }
    }
}
pub(crate) unsafe extern "C" fn HandleLinkDataSend() {
    unsafe {
        'l1: {
            let __sw1 = ((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(147))
            .read()) as i32);
            let mut __fall = false;
            if __sw1 == 1i32 {
                __fall = true;
                if (IsLinkTaskFinished()) != 0 {
                    SendBlock(
                        BitmaskAllOtherLinkPlayers(),
                        (((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(116))
                        .cast::<u16>())
                        .cast::<u8>(),
                        20u16,
                    );
                    let __p2 = (((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(147);
                    (__p2).write(((__p2).read()).wrapping_add(1));
                }
            }
            if __fall || __sw1 == 2i32 {
                __fall = true;
                ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(147))
                    .write(0u8);
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn CB2_InGameTrade() {
    unsafe {
        DoTradeAnim();
        RunTasks();
        RunTextPrinters();
        AnimateSprites();
        BuildOamBuffer();
        UpdatePaletteFade();
    }
}
pub(crate) unsafe extern "C" fn SetTradeSequenceBgGpuRegs(state: u8) {
    unsafe {
        let mut state = state;
        'l1: {
            let __sw1 = ((state) as i32);
            if __sw1 == 0i32 {
                ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(228)
                    .cast::<i16>())
                .write(0i16);
                ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(230)
                    .cast::<i16>())
                .write(180i16);
                SetGpuReg(0u8, 5440u16);
                SetGpuReg(12u8, 20998u16);
                LoadPalette(
                    (((&raw mut gTradeGba2_Pal).cast::<u16>()).cast::<u16>()).cast::<u8>(),
                    16u16,
                    96u16,
                );
                {
                    let mut _src: *mut u8 = (&raw mut gTradeGba_Gfx).cast::<u8>();
                    let mut _dest: *mut u8 = ((100679680i32) as usize as *mut u8);
                    let mut _size: u32 = 5152u32;
                    'l2: loop {
                        if !((1i32) != 0) {
                            break 'l2;
                        }
                        if _size <= 4096u32 {
                            'l3: loop {
                                'l4: {
                                    'l5: loop {
                                        'l6: {
                                            {
                                                let mut dmaRegs: *mut u32 =
                                                    ((67109076i32) as usize as *mut u32);
                                                crate::c::volatile_write(
                                                    dmaRegs,
                                                    ((_src) as usize as u32),
                                                );
                                                crate::c::volatile_write(
                                                    (dmaRegs).wrapping_offset(1),
                                                    ((_dest) as usize as u32),
                                                );
                                                crate::c::volatile_write(
                                                    (dmaRegs).wrapping_offset(2),
                                                    (2147483648u32
                                                        | crate::c::div_u32(
                                                            _size,
                                                            ((crate::c::div_i32(16i32, 8i32))
                                                                as u32),
                                                        )),
                                                );
                                                let _ =
                                                    ((dmaRegs).wrapping_offset(2)).read_volatile();
                                            }
                                        }
                                        if !((0i32) != 0) {
                                            break 'l5;
                                        }
                                    }
                                }
                                if !((0i32) != 0) {
                                    break 'l3;
                                }
                            }
                            break 'l2;
                        }
                        'l7: loop {
                            'l8: {
                                'l9: loop {
                                    'l10: {
                                        {
                                            let mut dmaRegs: *mut u32 =
                                                ((67109076i32) as usize as *mut u32);
                                            crate::c::volatile_write(
                                                dmaRegs,
                                                ((_src) as usize as u32),
                                            );
                                            crate::c::volatile_write(
                                                (dmaRegs).wrapping_offset(1),
                                                ((_dest) as usize as u32),
                                            );
                                            crate::c::volatile_write(
                                                (dmaRegs).wrapping_offset(2),
                                                (((-2147483648i32)
                                                    | crate::c::div_i32(
                                                        4096i32,
                                                        crate::c::div_i32(16i32, 8i32),
                                                    ))
                                                    as u32),
                                            );
                                            let _ = ((dmaRegs).wrapping_offset(2)).read_volatile();
                                        }
                                    }
                                    if !((0i32) != 0) {
                                        break 'l9;
                                    }
                                }
                            }
                            if !((0i32) != 0) {
                                break 'l7;
                            }
                        }
                        _src = (_src).wrapping_offset(4096);
                        _dest = (_dest).wrapping_offset(4096);
                        _size = (_size).wrapping_sub(4096u32);
                    }
                }
                {
                    let mut _src: *mut u8 = (((&raw const gTradePlatform_Tilemap)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<u16>())
                    .cast::<u16>())
                    .cast::<u8>();
                    let mut _dest: *mut u8 = ((100700160i32) as usize as *mut u8);
                    let mut _size: u32 = 4096u32;
                    'l11: loop {
                        'l12: {
                            'l13: loop {
                                'l14: {
                                    {
                                        let mut dmaRegs: *mut u32 =
                                            ((67109076i32) as usize as *mut u32);
                                        crate::c::volatile_write(dmaRegs, ((_src) as usize as u32));
                                        crate::c::volatile_write(
                                            (dmaRegs).wrapping_offset(1),
                                            ((_dest) as usize as u32),
                                        );
                                        crate::c::volatile_write(
                                            (dmaRegs).wrapping_offset(2),
                                            (2147483648u32
                                                | crate::c::div_u32(
                                                    _size,
                                                    ((crate::c::div_i32(16i32, 8i32)) as u32),
                                                )),
                                        );
                                        let _ = ((dmaRegs).wrapping_offset(2)).read_volatile();
                                    }
                                }
                                if !((0i32) != 0) {
                                    break 'l13;
                                }
                            }
                        }
                        if !((0i32) != 0) {
                            break 'l11;
                        }
                    }
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(226)
                    .cast::<i16>())
                .write(0i16);
                ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(224)
                    .cast::<i16>())
                .write(348i16);
                SetGpuReg(22u8, 348u16);
                SetGpuReg(10u8, 34050u16);
                SetGpuReg(12u8, 37382u16);
                if (((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(250))
                .read())
                    != 0
                {
                    {
                        let mut _src: *mut u8 = (((&raw const sGbaMapCable)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<u16>())
                        .cast::<u16>())
                        .cast::<u8>();
                        let mut _dest: *mut u8 = ((100673536i32) as usize as *mut u8);
                        let mut _size: u32 = 4096u32;
                        'l15: loop {
                            'l16: {
                                'l17: loop {
                                    'l18: {
                                        {
                                            let mut dmaRegs: *mut u32 =
                                                ((67109076i32) as usize as *mut u32);
                                            crate::c::volatile_write(
                                                dmaRegs,
                                                ((_src) as usize as u32),
                                            );
                                            crate::c::volatile_write(
                                                (dmaRegs).wrapping_offset(1),
                                                ((_dest) as usize as u32),
                                            );
                                            crate::c::volatile_write(
                                                (dmaRegs).wrapping_offset(2),
                                                (2147483648u32
                                                    | crate::c::div_u32(
                                                        _size,
                                                        ((crate::c::div_i32(16i32, 8i32)) as u32),
                                                    )),
                                            );
                                            let _ = ((dmaRegs).wrapping_offset(2)).read_volatile();
                                        }
                                    }
                                    if !((0i32) != 0) {
                                        break 'l17;
                                    }
                                }
                            }
                            if !((0i32) != 0) {
                                break 'l15;
                            }
                        }
                    }
                } else {
                    {
                        let mut _src: *mut u8 = (((&raw const sGbaMapWireless)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<u16>())
                        .cast::<u16>())
                        .cast::<u8>();
                        let mut _dest: *mut u8 = ((100673536i32) as usize as *mut u8);
                        let mut _size: u32 = 4096u32;
                        'l19: loop {
                            'l20: {
                                'l21: loop {
                                    'l22: {
                                        {
                                            let mut dmaRegs: *mut u32 =
                                                ((67109076i32) as usize as *mut u32);
                                            crate::c::volatile_write(
                                                dmaRegs,
                                                ((_src) as usize as u32),
                                            );
                                            crate::c::volatile_write(
                                                (dmaRegs).wrapping_offset(1),
                                                ((_dest) as usize as u32),
                                            );
                                            crate::c::volatile_write(
                                                (dmaRegs).wrapping_offset(2),
                                                (2147483648u32
                                                    | crate::c::div_u32(
                                                        _size,
                                                        ((crate::c::div_i32(16i32, 8i32)) as u32),
                                                    )),
                                            );
                                            let _ = ((dmaRegs).wrapping_offset(2)).read_volatile();
                                        }
                                    }
                                    if !((0i32) != 0) {
                                        break 'l21;
                                    }
                                }
                            }
                            if !((0i32) != 0) {
                                break 'l19;
                            }
                        }
                    }
                }
                {
                    let mut _src: *mut u8 = (&raw mut gTradeGba_Gfx).cast::<u8>();
                    let mut _dest: *mut u8 = ((100663296i32) as usize as *mut u8);
                    let mut _size: u32 = 5152u32;
                    'l23: loop {
                        if !((1i32) != 0) {
                            break 'l23;
                        }
                        if _size <= 4096u32 {
                            'l24: loop {
                                'l25: {
                                    'l26: loop {
                                        'l27: {
                                            {
                                                let mut dmaRegs: *mut u32 =
                                                    ((67109076i32) as usize as *mut u32);
                                                crate::c::volatile_write(
                                                    dmaRegs,
                                                    ((_src) as usize as u32),
                                                );
                                                crate::c::volatile_write(
                                                    (dmaRegs).wrapping_offset(1),
                                                    ((_dest) as usize as u32),
                                                );
                                                crate::c::volatile_write(
                                                    (dmaRegs).wrapping_offset(2),
                                                    (2147483648u32
                                                        | crate::c::div_u32(
                                                            _size,
                                                            ((crate::c::div_i32(16i32, 8i32))
                                                                as u32),
                                                        )),
                                                );
                                                let _ =
                                                    ((dmaRegs).wrapping_offset(2)).read_volatile();
                                            }
                                        }
                                        if !((0i32) != 0) {
                                            break 'l26;
                                        }
                                    }
                                }
                                if !((0i32) != 0) {
                                    break 'l24;
                                }
                            }
                            break 'l23;
                        }
                        'l28: loop {
                            'l29: {
                                'l30: loop {
                                    'l31: {
                                        {
                                            let mut dmaRegs: *mut u32 =
                                                ((67109076i32) as usize as *mut u32);
                                            crate::c::volatile_write(
                                                dmaRegs,
                                                ((_src) as usize as u32),
                                            );
                                            crate::c::volatile_write(
                                                (dmaRegs).wrapping_offset(1),
                                                ((_dest) as usize as u32),
                                            );
                                            crate::c::volatile_write(
                                                (dmaRegs).wrapping_offset(2),
                                                (((-2147483648i32)
                                                    | crate::c::div_i32(
                                                        4096i32,
                                                        crate::c::div_i32(16i32, 8i32),
                                                    ))
                                                    as u32),
                                            );
                                            let _ = ((dmaRegs).wrapping_offset(2)).read_volatile();
                                        }
                                    }
                                    if !((0i32) != 0) {
                                        break 'l30;
                                    }
                                }
                            }
                            if !((0i32) != 0) {
                                break 'l28;
                            }
                        }
                        _src = (_src).wrapping_offset(4096);
                        _dest = (_dest).wrapping_offset(4096);
                        _size = (_size).wrapping_sub(4096u32);
                    }
                }
                SetGpuReg(0u8, 4672u16);
                break 'l1;
            }
            if __sw1 == 2i32 {
                ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(224)
                    .cast::<i16>())
                .write(0i16);
                ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(226)
                    .cast::<i16>())
                .write(0i16);
                if !((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(250))
                .read())
                    != 0)
                {
                    SetGpuReg(0u8, 4673u16);
                    LZ77UnCompVram(
                        ((&raw const sWirelessCloseup_Map)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<u32>())
                        .cast::<u32>(),
                        ((100673536i32) as usize as *mut u8),
                    );
                    BlendPalettes(8u32, 16u8, 0u16);
                } else {
                    SetGpuReg(0u8, 4673u16);
                    {
                        let mut _src: *mut u8 = (((&raw const sCableCloseup_Map)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<u16>())
                        .cast::<u16>())
                        .cast::<u8>();
                        let mut _dest: *mut u8 = ((100673536i32) as usize as *mut u8);
                        let mut _size: u32 = 2048u32;
                        'l32: loop {
                            'l33: {
                                'l34: loop {
                                    'l35: {
                                        {
                                            let mut dmaRegs: *mut u32 =
                                                ((67109076i32) as usize as *mut u32);
                                            crate::c::volatile_write(
                                                dmaRegs,
                                                ((_src) as usize as u32),
                                            );
                                            crate::c::volatile_write(
                                                (dmaRegs).wrapping_offset(1),
                                                ((_dest) as usize as u32),
                                            );
                                            crate::c::volatile_write(
                                                (dmaRegs).wrapping_offset(2),
                                                (2147483648u32
                                                    | crate::c::div_u32(
                                                        _size,
                                                        ((crate::c::div_i32(16i32, 8i32)) as u32),
                                                    )),
                                            );
                                            let _ = ((dmaRegs).wrapping_offset(2)).read_volatile();
                                        }
                                    }
                                    if !((0i32) != 0) {
                                        break 'l34;
                                    }
                                }
                            }
                            if !((0i32) != 0) {
                                break 'l32;
                            }
                        }
                    }
                    BlendPalettes(1u32, 16u8, 0u16);
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                LoadPalette(
                    (((&raw const sWirelessSignalNone_Pal)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<u16>())
                    .cast::<u16>())
                    .cast::<u8>(),
                    48u16,
                    32u16,
                );
                LZ77UnCompVram(
                    ((&raw const sWirelessSignal_Gfx)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<u32>())
                    .cast::<u32>(),
                    ((100679680i32) as usize as *mut u8),
                );
                LZ77UnCompVram(
                    ((&raw const sWirelessSignal_Tilemap)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<u32>())
                    .cast::<u32>(),
                    ((100700160i32) as usize as *mut u8),
                );
                ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(228)
                    .cast::<i16>())
                .write(80i16);
                SetGpuReg(0u8, 5696u16);
                break 'l1;
            }
            if __sw1 == 4i32 {
                SetGpuReg(0u8, 5185u16);
                SetGpuReg(12u8, 4743u16);
                ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(212)
                    .cast::<u16>())
                .write(64u16);
                ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(214)
                    .cast::<u16>())
                .write(92u16);
                ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(232)
                    .cast::<u16>())
                .write(32u16);
                ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(234)
                    .cast::<u16>())
                .write(1024u16);
                ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(236)
                    .cast::<u16>())
                .write(0u16);
                {
                    let mut _src: *mut u8 =
                        ((&raw const sGbaAffine_Gfx).cast::<u8>().cast_mut()).cast::<u8>();
                    let mut _dest: *mut u8 = ((100679680i32) as usize as *mut u8);
                    let mut _size: u32 = 10304u32;
                    'l36: loop {
                        if !((1i32) != 0) {
                            break 'l36;
                        }
                        if _size <= 4096u32 {
                            'l37: loop {
                                'l38: {
                                    'l39: loop {
                                        'l40: {
                                            {
                                                let mut dmaRegs: *mut u32 =
                                                    ((67109076i32) as usize as *mut u32);
                                                crate::c::volatile_write(
                                                    dmaRegs,
                                                    ((_src) as usize as u32),
                                                );
                                                crate::c::volatile_write(
                                                    (dmaRegs).wrapping_offset(1),
                                                    ((_dest) as usize as u32),
                                                );
                                                crate::c::volatile_write(
                                                    (dmaRegs).wrapping_offset(2),
                                                    (2147483648u32
                                                        | crate::c::div_u32(
                                                            _size,
                                                            ((crate::c::div_i32(16i32, 8i32))
                                                                as u32),
                                                        )),
                                                );
                                                let _ =
                                                    ((dmaRegs).wrapping_offset(2)).read_volatile();
                                            }
                                        }
                                        if !((0i32) != 0) {
                                            break 'l39;
                                        }
                                    }
                                }
                                if !((0i32) != 0) {
                                    break 'l37;
                                }
                            }
                            break 'l36;
                        }
                        'l41: loop {
                            'l42: {
                                'l43: loop {
                                    'l44: {
                                        {
                                            let mut dmaRegs: *mut u32 =
                                                ((67109076i32) as usize as *mut u32);
                                            crate::c::volatile_write(
                                                dmaRegs,
                                                ((_src) as usize as u32),
                                            );
                                            crate::c::volatile_write(
                                                (dmaRegs).wrapping_offset(1),
                                                ((_dest) as usize as u32),
                                            );
                                            crate::c::volatile_write(
                                                (dmaRegs).wrapping_offset(2),
                                                (((-2147483648i32)
                                                    | crate::c::div_i32(
                                                        4096i32,
                                                        crate::c::div_i32(16i32, 8i32),
                                                    ))
                                                    as u32),
                                            );
                                            let _ = ((dmaRegs).wrapping_offset(2)).read_volatile();
                                        }
                                    }
                                    if !((0i32) != 0) {
                                        break 'l43;
                                    }
                                }
                            }
                            if !((0i32) != 0) {
                                break 'l41;
                            }
                        }
                        _src = (_src).wrapping_offset(4096);
                        _dest = (_dest).wrapping_offset(4096);
                        _size = (_size).wrapping_sub(4096u32);
                    }
                }
                if (((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(250))
                .read())
                    != 0
                {
                    {
                        let mut _src: *mut u8 =
                            ((&raw const sGbaAffineMapCable).cast::<u8>().cast_mut()).cast::<u8>();
                        let mut _dest: *mut u8 = ((100700160i32) as usize as *mut u8);
                        let mut _size: u32 = 256u32;
                        'l45: loop {
                            'l46: {
                                'l47: loop {
                                    'l48: {
                                        {
                                            let mut dmaRegs: *mut u32 =
                                                ((67109076i32) as usize as *mut u32);
                                            crate::c::volatile_write(
                                                dmaRegs,
                                                ((_src) as usize as u32),
                                            );
                                            crate::c::volatile_write(
                                                (dmaRegs).wrapping_offset(1),
                                                ((_dest) as usize as u32),
                                            );
                                            crate::c::volatile_write(
                                                (dmaRegs).wrapping_offset(2),
                                                (2147483648u32
                                                    | crate::c::div_u32(
                                                        _size,
                                                        ((crate::c::div_i32(16i32, 8i32)) as u32),
                                                    )),
                                            );
                                            let _ = ((dmaRegs).wrapping_offset(2)).read_volatile();
                                        }
                                    }
                                    if !((0i32) != 0) {
                                        break 'l47;
                                    }
                                }
                            }
                            if !((0i32) != 0) {
                                break 'l45;
                            }
                        }
                    }
                } else {
                    {
                        let mut _src: *mut u8 =
                            ((&raw const sGbaAffineMapWireless).cast::<u8>().cast_mut())
                                .cast::<u8>();
                        let mut _dest: *mut u8 = ((100700160i32) as usize as *mut u8);
                        let mut _size: u32 = 256u32;
                        'l49: loop {
                            'l50: {
                                'l51: loop {
                                    'l52: {
                                        {
                                            let mut dmaRegs: *mut u32 =
                                                ((67109076i32) as usize as *mut u32);
                                            crate::c::volatile_write(
                                                dmaRegs,
                                                ((_src) as usize as u32),
                                            );
                                            crate::c::volatile_write(
                                                (dmaRegs).wrapping_offset(1),
                                                ((_dest) as usize as u32),
                                            );
                                            crate::c::volatile_write(
                                                (dmaRegs).wrapping_offset(2),
                                                (2147483648u32
                                                    | crate::c::div_u32(
                                                        _size,
                                                        ((crate::c::div_i32(16i32, 8i32)) as u32),
                                                    )),
                                            );
                                            let _ = ((dmaRegs).wrapping_offset(2)).read_volatile();
                                        }
                                    }
                                    if !((0i32) != 0) {
                                        break 'l51;
                                    }
                                }
                            }
                            if !((0i32) != 0) {
                                break 'l49;
                            }
                        }
                    }
                }
                break 'l1;
            }
            if __sw1 == 5i32 {
                ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(224)
                    .cast::<i16>())
                .write(0i16);
                ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(226)
                    .cast::<i16>())
                .write(0i16);
                break 'l1;
            }
            if __sw1 == 6i32 {
                SetGpuReg(0u8, 5185u16);
                SetGpuReg(12u8, 4743u16);
                ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(212)
                    .cast::<u16>())
                .write(64u16);
                ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(214)
                    .cast::<u16>())
                .write(92u16);
                ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(232)
                    .cast::<u16>())
                .write(256u16);
                ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(234)
                    .cast::<u16>())
                .write(128u16);
                ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(220)
                    .cast::<u16>())
                .write(120u16);
                ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(222)
                    .cast::<u16>())
                .write(80u16);
                ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(236)
                    .cast::<u16>())
                .write(0u16);
                {
                    let mut _src: *mut u8 =
                        ((&raw const sGbaAffine_Gfx).cast::<u8>().cast_mut()).cast::<u8>();
                    let mut _dest: *mut u8 = ((100679680i32) as usize as *mut u8);
                    let mut _size: u32 = 10304u32;
                    'l53: loop {
                        if !((1i32) != 0) {
                            break 'l53;
                        }
                        if _size <= 4096u32 {
                            'l54: loop {
                                'l55: {
                                    'l56: loop {
                                        'l57: {
                                            {
                                                let mut dmaRegs: *mut u32 =
                                                    ((67109076i32) as usize as *mut u32);
                                                crate::c::volatile_write(
                                                    dmaRegs,
                                                    ((_src) as usize as u32),
                                                );
                                                crate::c::volatile_write(
                                                    (dmaRegs).wrapping_offset(1),
                                                    ((_dest) as usize as u32),
                                                );
                                                crate::c::volatile_write(
                                                    (dmaRegs).wrapping_offset(2),
                                                    (2147483648u32
                                                        | crate::c::div_u32(
                                                            _size,
                                                            ((crate::c::div_i32(16i32, 8i32))
                                                                as u32),
                                                        )),
                                                );
                                                let _ =
                                                    ((dmaRegs).wrapping_offset(2)).read_volatile();
                                            }
                                        }
                                        if !((0i32) != 0) {
                                            break 'l56;
                                        }
                                    }
                                }
                                if !((0i32) != 0) {
                                    break 'l54;
                                }
                            }
                            break 'l53;
                        }
                        'l58: loop {
                            'l59: {
                                'l60: loop {
                                    'l61: {
                                        {
                                            let mut dmaRegs: *mut u32 =
                                                ((67109076i32) as usize as *mut u32);
                                            crate::c::volatile_write(
                                                dmaRegs,
                                                ((_src) as usize as u32),
                                            );
                                            crate::c::volatile_write(
                                                (dmaRegs).wrapping_offset(1),
                                                ((_dest) as usize as u32),
                                            );
                                            crate::c::volatile_write(
                                                (dmaRegs).wrapping_offset(2),
                                                (((-2147483648i32)
                                                    | crate::c::div_i32(
                                                        4096i32,
                                                        crate::c::div_i32(16i32, 8i32),
                                                    ))
                                                    as u32),
                                            );
                                            let _ = ((dmaRegs).wrapping_offset(2)).read_volatile();
                                        }
                                    }
                                    if !((0i32) != 0) {
                                        break 'l60;
                                    }
                                }
                            }
                            if !((0i32) != 0) {
                                break 'l58;
                            }
                        }
                        _src = (_src).wrapping_offset(4096);
                        _dest = (_dest).wrapping_offset(4096);
                        _size = (_size).wrapping_sub(4096u32);
                    }
                }
                if (((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(250))
                .read())
                    != 0
                {
                    {
                        let mut _src: *mut u8 =
                            ((&raw const sGbaAffineMapCable).cast::<u8>().cast_mut()).cast::<u8>();
                        let mut _dest: *mut u8 = ((100700160i32) as usize as *mut u8);
                        let mut _size: u32 = 256u32;
                        'l62: loop {
                            'l63: {
                                'l64: loop {
                                    'l65: {
                                        {
                                            let mut dmaRegs: *mut u32 =
                                                ((67109076i32) as usize as *mut u32);
                                            crate::c::volatile_write(
                                                dmaRegs,
                                                ((_src) as usize as u32),
                                            );
                                            crate::c::volatile_write(
                                                (dmaRegs).wrapping_offset(1),
                                                ((_dest) as usize as u32),
                                            );
                                            crate::c::volatile_write(
                                                (dmaRegs).wrapping_offset(2),
                                                (2147483648u32
                                                    | crate::c::div_u32(
                                                        _size,
                                                        ((crate::c::div_i32(16i32, 8i32)) as u32),
                                                    )),
                                            );
                                            let _ = ((dmaRegs).wrapping_offset(2)).read_volatile();
                                        }
                                    }
                                    if !((0i32) != 0) {
                                        break 'l64;
                                    }
                                }
                            }
                            if !((0i32) != 0) {
                                break 'l62;
                            }
                        }
                    }
                } else {
                    {
                        let mut _src: *mut u8 =
                            ((&raw const sGbaAffineMapWireless).cast::<u8>().cast_mut())
                                .cast::<u8>();
                        let mut _dest: *mut u8 = ((100700160i32) as usize as *mut u8);
                        let mut _size: u32 = 256u32;
                        'l66: loop {
                            'l67: {
                                'l68: loop {
                                    'l69: {
                                        {
                                            let mut dmaRegs: *mut u32 =
                                                ((67109076i32) as usize as *mut u32);
                                            crate::c::volatile_write(
                                                dmaRegs,
                                                ((_src) as usize as u32),
                                            );
                                            crate::c::volatile_write(
                                                (dmaRegs).wrapping_offset(1),
                                                ((_dest) as usize as u32),
                                            );
                                            crate::c::volatile_write(
                                                (dmaRegs).wrapping_offset(2),
                                                (2147483648u32
                                                    | crate::c::div_u32(
                                                        _size,
                                                        ((crate::c::div_i32(16i32, 8i32)) as u32),
                                                    )),
                                            );
                                            let _ = ((dmaRegs).wrapping_offset(2)).read_volatile();
                                        }
                                    }
                                    if !((0i32) != 0) {
                                        break 'l68;
                                    }
                                }
                            }
                            if !((0i32) != 0) {
                                break 'l66;
                            }
                        }
                    }
                }
                break 'l1;
            }
            if __sw1 == 7i32 {
                ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(228)
                    .cast::<i16>())
                .write(0i16);
                ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(230)
                    .cast::<i16>())
                .write(0i16);
                SetGpuReg(80u8, 0u16);
                SetGpuReg(12u8, 20998u16);
                LoadPalette(
                    (((&raw mut gTradeGba2_Pal).cast::<u16>()).cast::<u16>()).cast::<u8>(),
                    16u16,
                    96u16,
                );
                {
                    let mut _src: *mut u8 = (&raw mut gTradeGba_Gfx).cast::<u8>();
                    let mut _dest: *mut u8 = ((100679680i32) as usize as *mut u8);
                    let mut _size: u32 = 5152u32;
                    'l70: loop {
                        if !((1i32) != 0) {
                            break 'l70;
                        }
                        if _size <= 4096u32 {
                            'l71: loop {
                                'l72: {
                                    'l73: loop {
                                        'l74: {
                                            {
                                                let mut dmaRegs: *mut u32 =
                                                    ((67109076i32) as usize as *mut u32);
                                                crate::c::volatile_write(
                                                    dmaRegs,
                                                    ((_src) as usize as u32),
                                                );
                                                crate::c::volatile_write(
                                                    (dmaRegs).wrapping_offset(1),
                                                    ((_dest) as usize as u32),
                                                );
                                                crate::c::volatile_write(
                                                    (dmaRegs).wrapping_offset(2),
                                                    (2147483648u32
                                                        | crate::c::div_u32(
                                                            _size,
                                                            ((crate::c::div_i32(16i32, 8i32))
                                                                as u32),
                                                        )),
                                                );
                                                let _ =
                                                    ((dmaRegs).wrapping_offset(2)).read_volatile();
                                            }
                                        }
                                        if !((0i32) != 0) {
                                            break 'l73;
                                        }
                                    }
                                }
                                if !((0i32) != 0) {
                                    break 'l71;
                                }
                            }
                            break 'l70;
                        }
                        'l75: loop {
                            'l76: {
                                'l77: loop {
                                    'l78: {
                                        {
                                            let mut dmaRegs: *mut u32 =
                                                ((67109076i32) as usize as *mut u32);
                                            crate::c::volatile_write(
                                                dmaRegs,
                                                ((_src) as usize as u32),
                                            );
                                            crate::c::volatile_write(
                                                (dmaRegs).wrapping_offset(1),
                                                ((_dest) as usize as u32),
                                            );
                                            crate::c::volatile_write(
                                                (dmaRegs).wrapping_offset(2),
                                                (((-2147483648i32)
                                                    | crate::c::div_i32(
                                                        4096i32,
                                                        crate::c::div_i32(16i32, 8i32),
                                                    ))
                                                    as u32),
                                            );
                                            let _ = ((dmaRegs).wrapping_offset(2)).read_volatile();
                                        }
                                    }
                                    if !((0i32) != 0) {
                                        break 'l77;
                                    }
                                }
                            }
                            if !((0i32) != 0) {
                                break 'l75;
                            }
                        }
                        _src = (_src).wrapping_offset(4096);
                        _dest = (_dest).wrapping_offset(4096);
                        _size = (_size).wrapping_sub(4096u32);
                    }
                }
                {
                    let mut _src: *mut u8 = (((&raw const gTradePlatform_Tilemap)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<u16>())
                    .cast::<u16>())
                    .cast::<u8>();
                    let mut _dest: *mut u8 = ((100700160i32) as usize as *mut u8);
                    let mut _size: u32 = 4096u32;
                    'l79: loop {
                        'l80: {
                            'l81: loop {
                                'l82: {
                                    {
                                        let mut dmaRegs: *mut u32 =
                                            ((67109076i32) as usize as *mut u32);
                                        crate::c::volatile_write(dmaRegs, ((_src) as usize as u32));
                                        crate::c::volatile_write(
                                            (dmaRegs).wrapping_offset(1),
                                            ((_dest) as usize as u32),
                                        );
                                        crate::c::volatile_write(
                                            (dmaRegs).wrapping_offset(2),
                                            (2147483648u32
                                                | crate::c::div_u32(
                                                    _size,
                                                    ((crate::c::div_i32(16i32, 8i32)) as u32),
                                                )),
                                        );
                                        let _ = ((dmaRegs).wrapping_offset(2)).read_volatile();
                                    }
                                }
                                if !((0i32) != 0) {
                                    break 'l81;
                                }
                            }
                        }
                        if !((0i32) != 0) {
                            break 'l79;
                        }
                    }
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn LoadTradeSequenceSpriteSheetsAndPalettes() {
    unsafe {
        LoadSpriteSheet(
            (&raw const sSpriteSheet_LinkMonGlow)
                .cast::<u8>()
                .cast_mut(),
        );
        LoadSpriteSheet(
            (&raw const sSpriteSheet_LinkMonShadow)
                .cast::<u8>()
                .cast_mut(),
        );
        LoadSpriteSheet((&raw const sSpriteSheet_CableEnd).cast::<u8>().cast_mut());
        LoadSpriteSheet((&raw const sSpriteSheet_GbaScreen).cast::<u8>().cast_mut());
        LoadSpritePalette((&raw const sSpritePalette_LinkMon).cast::<u8>().cast_mut());
        LoadSpritePalette((&raw const sSpritePalette_Gba).cast::<u8>().cast_mut());
    }
}
pub(crate) unsafe extern "C" fn BufferTradeSceneStrings() {
    unsafe {
        let mut mpId: u8 = 0u8;
        let mut name = crate::ffi::Align4([0u8; 20]);
        let mut ingameTrade: *mut u8 = core::ptr::null_mut();
        if (((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(238))
            .read())
            != 0
        {
            mpId = GetMultiplayerId();
            StringCopy(
                (&raw mut gStringVar1).cast::<u8>(),
                ((((&raw mut gLinkPlayers).cast::<u8>())
                    .wrapping_offset((((mpId) as i32) ^ 1i32) as isize * 28))
                .wrapping_add(8))
                .cast::<u8>(),
            );
            GetMonData3(
                ((&raw mut gEnemyParty).cast::<u8>()).wrapping_offset(
                    (crate::c::rem_i32(
                        ((((((&raw mut gSelectedTradeMonPositions).cast::<u8>()).cast::<u8>())
                            .wrapping_offset(1))
                        .read()) as i32),
                        6i32,
                    )) as isize
                        * 100,
                ),
                2i32,
                (&raw mut name).cast::<u8>(),
            );
            StringCopy_Nickname(
                (&raw mut gStringVar3).cast::<u8>(),
                (&raw mut name).cast::<u8>(),
            );
            GetMonData3(
                ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                    (((((&raw mut gSelectedTradeMonPositions).cast::<u8>()).cast::<u8>()).read())
                        as i32) as isize
                        * 100,
                ),
                2i32,
                (&raw mut name).cast::<u8>(),
            );
            StringCopy_Nickname(
                (&raw mut gStringVar2).cast::<u8>(),
                (&raw mut name).cast::<u8>(),
            );
        } else {
            ingameTrade = (((&raw const sIngameTrades).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(
                    ((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32) as isize * 60,
                );
            StringCopy(
                (&raw mut gStringVar1).cast::<u8>(),
                ((ingameTrade).wrapping_add(43)).cast::<u8>(),
            );
            StringCopy_Nickname(
                (&raw mut gStringVar3).cast::<u8>(),
                (ingameTrade).cast::<u8>(),
            );
            GetMonData3(
                ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                    ((((&raw mut gSpecialVar_0x8005).cast::<u16>()).read()) as i32) as isize * 100,
                ),
                2i32,
                (&raw mut name).cast::<u8>(),
            );
            StringCopy_Nickname(
                (&raw mut gStringVar2).cast::<u8>(),
                (&raw mut name).cast::<u8>(),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn DoTradeAnim() -> u8 {
    unsafe {
        if (((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(250))
            .read())
            != 0
        {
            return DoTradeAnim_Cable();
        } else {
            return DoTradeAnim_Wireless();
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
pub(crate) unsafe extern "C" fn DoTradeAnim_Cable() -> u8 {
    unsafe {
        let mut evoTarget: u16 = 0u16;
        'l1: {
            let __sw1 = ((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(148)
                .cast::<u16>())
            .read()) as i32);
            if __sw1 == 0i32 {
                crate::c::bf_write(
                    (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        (((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(142))
                        .cast::<u8>())
                        .read()) as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(62),
                    2,
                    1,
                    (0u16) as i32,
                );
                ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    (((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(142))
                    .cast::<u8>())
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(36)
                .cast::<i16>())
                .write((-180i16));
                ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    (((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(142))
                    .cast::<u8>())
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(38)
                .cast::<i16>())
                .write(
                    ((((((&raw mut gMonFrontPicCoords).cast::<u8>()).wrapping_offset(
                        (((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(240))
                        .cast::<u16>())
                        .read()) as i32) as isize
                            * 4,
                    ))
                    .wrapping_add(1))
                    .read()) as i16),
                );
                let __p2 = (((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(148)
                    .cast::<u16>();
                (__p2).write(((__p2).read()).wrapping_add(1));
                ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(244)
                    .cast::<u16>())
                .write(GetCurrentMapMusic());
                PlayNewMapMusic(377u16);
                break 'l1;
            }
            if __sw1 == 1i32 {
                if ((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(230)
                    .cast::<i16>())
                .read()) as i32)
                    > 0i32
                {
                    let __p3 = (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        (((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(142))
                        .cast::<u8>())
                        .read()) as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(36)
                    .cast::<i16>();
                    (__p3).write((((((__p3).read()) as i32).wrapping_add(3i32)) as i16));
                    let __p4 = (((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(230)
                        .cast::<i16>();
                    (__p4).write((((((__p4).read()) as i32).wrapping_sub(3i32)) as i16));
                } else {
                    ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        (((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(142))
                        .cast::<u8>())
                        .read()) as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(36)
                    .cast::<i16>())
                    .write(0i16);
                    ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(230)
                        .cast::<i16>())
                    .write(0i16);
                    ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(148)
                        .cast::<u16>())
                    .write(10u16);
                }
                break 'l1;
            }
            if __sw1 == 10i32 {
                StringExpandPlaceholders(
                    (&raw mut gStringVar4).cast::<u8>(),
                    (&raw mut gText_XWillBeSentToY).cast::<u8>(),
                );
                DrawTextOnTradeWindow(0u8, (&raw mut gStringVar4).cast::<u8>(), 0u8);
                if (((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(240))
                .cast::<u16>())
                .read()) as i32)
                    != 412i32
                {
                    PlayCry_Normal(
                        (((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(240))
                        .cast::<u16>())
                        .read(),
                        0i8,
                    );
                }
                ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(148)
                    .cast::<u16>())
                .write(11u16);
                ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(100)
                    .cast::<u32>())
                .write(0u32);
                break 'l1;
            }
            if __sw1 == 11i32 {
                if {
                    let __p5 = (((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(100)
                        .cast::<u32>();
                    let __t6 = ((__p5).read()).wrapping_add(1);
                    (__p5).write(__t6);
                    __t6
                } == 80u32
                {
                    ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(210))
                    .write(CreateTradePokeballSprite(
                        (((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(142))
                        .cast::<u8>())
                        .read(),
                        ((crate::c::bf_read(
                            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                (((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(142))
                                .cast::<u8>())
                                .read()) as i32) as isize
                                    * 68,
                            ))
                            .wrapping_add(5),
                            4,
                            4,
                            false,
                        ) as u16) as u8),
                        120u8,
                        32u8,
                        2u8,
                        1u8,
                        20u8,
                        1048575u32,
                    ));
                    let __p7 = (((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(148)
                        .cast::<u16>();
                    (__p7).write(((__p7).read()).wrapping_add(1));
                    StringExpandPlaceholders(
                        (&raw mut gStringVar4).cast::<u8>(),
                        (&raw mut gText_ByeByeVar1).cast::<u8>(),
                    );
                    DrawTextOnTradeWindow(0u8, (&raw mut gStringVar4).cast::<u8>(), 0u8);
                }
                break 'l1;
            }
            if __sw1 == 12i32 {
                if core::mem::transmute::<_, usize>(
                    ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(210))
                        .read()) as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(28)
                    .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                    .read(),
                ) == (SpriteCallbackDummy as *const () as usize)
                {
                    ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(211))
                    .write(CreateSprite(
                        (&raw const sSpriteTemplate_Pokeball)
                            .cast::<u8>()
                            .cast_mut(),
                        120i16,
                        32i16,
                        0u8,
                    ));
                    ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(211))
                        .read()) as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(28)
                    .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                    .write(Some(SpriteCB_BouncingPokeballDepart));
                    DestroySprite(
                        ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            ((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(210))
                            .read()) as i32) as isize
                                * 68,
                        ),
                    );
                    let __p8 = (((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(148)
                        .cast::<u16>();
                    (__p8).write(((__p8).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 13i32 {
                break 'l1;
            }
            if __sw1 == 14i32 {
                BeginNormalPaletteFade(4294967295u32, 0i8, 0u8, 16u8, 0u16);
                ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(148)
                    .cast::<u16>())
                .write(20u16);
                break 'l1;
            }
            if __sw1 == 20i32 {
                if !((crate::c::bf_read(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
                    7,
                    1,
                    false,
                ) as u16)
                    != 0)
                {
                    SetTradeSequenceBgGpuRegs(4u8);
                    FillWindowPixelBuffer(0u8, 255u8);
                    CopyWindowToVram(0u8, 3u8);
                    let __p9 = (((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(148)
                        .cast::<u16>();
                    (__p9).write(((__p9).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 21i32 {
                BeginNormalPaletteFade(4294967295u32, (-1i8), 16u8, 0u8, 0u16);
                let __p10 = (((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(148)
                    .cast::<u16>();
                (__p10).write(((__p10).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 22i32 {
                if !((crate::c::bf_read(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
                    7,
                    1,
                    false,
                ) as u16)
                    != 0)
                {
                    ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(148)
                        .cast::<u16>())
                    .write(23u16);
                }
                break 'l1;
            }
            if __sw1 == 23i32 {
                if ((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(234)
                    .cast::<u16>())
                .read()) as i32)
                    > 256i32
                {
                    let __p11 = (((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(234)
                        .cast::<u16>();
                    (__p11).write((((((__p11).read()) as i32).wrapping_sub(52i32)) as u16));
                } else {
                    SetTradeSequenceBgGpuRegs(1u8);
                    ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(234)
                        .cast::<u16>())
                    .write(128u16);
                    let __p12 = (((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(148)
                        .cast::<u16>();
                    (__p12).write(((__p12).read()).wrapping_add(1));
                    ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(100)
                        .cast::<u32>())
                    .write(0u32);
                }
                ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(232)
                    .cast::<u16>())
                .write(
                    ((crate::c::div_i32(
                        32768i32,
                        ((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(234)
                            .cast::<u16>())
                        .read()) as i32),
                    )) as u16),
                );
                break 'l1;
            }
            if __sw1 == 24i32 {
                if {
                    let __p13 = (((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(100)
                        .cast::<u32>();
                    let __t14 = ((__p13).read()).wrapping_add(1);
                    (__p13).write(__t14);
                    __t14
                } > 20u32
                {
                    SetTradeBGAffine();
                    ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(145))
                    .write(CreateSprite(
                        (&raw const sSpriteTemplate_GbaScreenFlash_Long)
                            .cast::<u8>()
                            .cast_mut(),
                        120i16,
                        80i16,
                        0u8,
                    ));
                    let __p15 = (((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(148)
                        .cast::<u16>();
                    (__p15).write(((__p15).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 25i32 {
                if (crate::c::bf_read(
                    (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(145))
                        .read()) as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(63),
                    4,
                    1,
                    false,
                ) as u16)
                    != 0
                {
                    DestroySprite(
                        ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            ((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(145))
                            .read()) as i32) as isize
                                * 68,
                        ),
                    );
                    SetGpuReg(80u8, 1600u16);
                    SetGpuReg(82u8, 1036u16);
                    let __p16 = (((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(148)
                        .cast::<u16>();
                    (__p16).write(((__p16).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 26i32 {
                if (({
                    let __p17 = (((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(224)
                        .cast::<i16>();
                    let __t18 = ((__p17).read()).wrapping_sub(1);
                    (__p17).write(__t18);
                    __t18
                }) as i32)
                    == 316i32
                {
                    let __p19 = (((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(148)
                        .cast::<u16>();
                    (__p19).write(((__p19).read()).wrapping_add(1));
                }
                if ((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(224)
                    .cast::<i16>())
                .read()) as i32)
                    == 328i32
                {
                    ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(146))
                    .write(CreateSprite(
                        (&raw const sSpriteTemplate_CableEnd)
                            .cast::<u8>()
                            .cast_mut(),
                        128i16,
                        65i16,
                        0u8,
                    ));
                }
                break 'l1;
            }
            if __sw1 == 27i32 {
                ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(144))
                    .write(CreateSprite(
                        (&raw const sSpriteTemplate_LinkMonGlow)
                            .cast::<u8>()
                            .cast_mut(),
                        128i16,
                        80i16,
                        3u8,
                    ));
                ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(145))
                    .write(CreateSprite(
                        (&raw const sSpriteTemplate_LinkMonShadow)
                            .cast::<u8>()
                            .cast_mut(),
                        128i16,
                        80i16,
                        0u8,
                    ));
                StartSpriteAnim(
                    ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(145))
                        .read()) as i32) as isize
                            * 68,
                    ),
                    1u8,
                );
                let __p20 = (((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(148)
                    .cast::<u16>();
                (__p20).write(((__p20).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 28i32 {
                if (({
                    let __p21 = (((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(224)
                        .cast::<i16>();
                    let __v22 = (((((__p21).read()) as i32).wrapping_sub(2i32)) as i16);
                    (__p21).write(__v22);
                    __v22
                }) as i32)
                    == 166i32
                {
                    ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(148)
                        .cast::<u16>())
                    .write(200u16);
                }
                SetGpuReg(0u8, 4673u16);
                break 'l1;
            }
            if __sw1 == 200i32 {
                let __p23 = (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(144))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(34)
                .cast::<i16>();
                (__p23).write((((((__p23).read()) as i32).wrapping_sub(2i32)) as i16));
                let __p24 = (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(145))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(34)
                .cast::<i16>();
                (__p24).write((((((__p24).read()) as i32).wrapping_sub(2i32)) as i16));
                if ((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(144))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(34)
                .cast::<i16>())
                .read()) as i32)
                    < (-8i32)
                {
                    ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(148)
                        .cast::<u16>())
                    .write(29u16);
                }
                break 'l1;
            }
            if __sw1 == 29i32 {
                BeginNormalPaletteFade(4294967295u32, (-1i8), 0u8, 16u8, 0u16);
                ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(148)
                    .cast::<u16>())
                .write(30u16);
                break 'l1;
            }
            if __sw1 == 30i32 {
                if !((crate::c::bf_read(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
                    7,
                    1,
                    false,
                ) as u16)
                    != 0)
                {
                    DestroySprite(
                        ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            ((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(144))
                            .read()) as i32) as isize
                                * 68,
                        ),
                    );
                    DestroySprite(
                        ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            ((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(145))
                            .read()) as i32) as isize
                                * 68,
                        ),
                    );
                    SetTradeSequenceBgGpuRegs(2u8);
                    let __p25 = (((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(148)
                        .cast::<u16>();
                    (__p25).write(((__p25).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 31i32 {
                BeginNormalPaletteFade(4294967295u32, (-1i8), 16u8, 0u8, 0u16);
                ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(144))
                    .write(CreateSprite(
                        (&raw const sSpriteTemplate_LinkMonShadow)
                            .cast::<u8>()
                            .cast_mut(),
                        111i16,
                        170i16,
                        0u8,
                    ));
                ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(145))
                    .write(CreateSprite(
                        (&raw const sSpriteTemplate_LinkMonShadow)
                            .cast::<u8>()
                            .cast_mut(),
                        129i16,
                        (-10i16),
                        0u8,
                    ));
                let __p26 = (((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(148)
                    .cast::<u16>();
                (__p26).write(((__p26).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 32i32 {
                if !((crate::c::bf_read(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
                    7,
                    1,
                    false,
                ) as u16)
                    != 0)
                {
                    PlaySE(46u16);
                    let __p27 = (((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(148)
                        .cast::<u16>();
                    (__p27).write(((__p27).read()).wrapping_add(1));
                }
                let __p28 = (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(144))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(38)
                .cast::<i16>();
                (__p28).write((((((__p28).read()) as i32).wrapping_sub(3i32)) as i16));
                let __p29 = (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(145))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(38)
                .cast::<i16>();
                (__p29).write((((((__p29).read()) as i32).wrapping_add(3i32)) as i16));
                break 'l1;
            }
            if __sw1 == 33i32 {
                let __p30 = (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(144))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(38)
                .cast::<i16>();
                (__p30).write((((((__p30).read()) as i32).wrapping_sub(3i32)) as i16));
                let __p31 = (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(145))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(38)
                .cast::<i16>();
                (__p31).write((((((__p31).read()) as i32).wrapping_add(3i32)) as i16));
                if ((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(144))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(38)
                .cast::<i16>())
                .read()) as i32)
                    <= (-90i32)
                {
                    ((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(144))
                        .read()) as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(46))
                    .cast::<i16>())
                    .wrapping_offset(1))
                    .write(1i16);
                    ((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(145))
                        .read()) as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(46))
                    .cast::<i16>())
                    .wrapping_offset(1))
                    .write(1i16);
                    let __p32 = (((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(148)
                        .cast::<u16>();
                    (__p32).write(((__p32).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 34i32 {
                BlendPalettes(1u32, 16u8, 65535u16);
                let __p33 = (((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(148)
                    .cast::<u16>();
                (__p33).write(((__p33).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 35i32 {
                BlendPalettes(1u32, 0u8, 65535u16);
                let __p34 = (((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(148)
                    .cast::<u16>();
                (__p34).write(((__p34).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 36i32 {
                BlendPalettes(1u32, 16u8, 65535u16);
                let __p35 = (((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(148)
                    .cast::<u16>();
                (__p35).write(((__p35).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 37i32 {
                if !((IsMonSpriteNotFlipped(
                    (((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(240))
                    .cast::<u16>())
                    .read(),
                )) != 0)
                {
                    ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        (((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(142))
                        .cast::<u8>())
                        .read()) as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(16)
                    .cast::<*mut *mut u8>())
                    .write(
                        ((&raw const sAffineAnims_CrossingMonPics)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<*mut u8>())
                        .cast::<*mut u8>(),
                    );
                    crate::c::bf_write(
                        (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            (((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(142))
                            .cast::<u8>())
                            .read()) as i32) as isize
                                * 68,
                        ))
                        .wrapping_add(1),
                        0,
                        2,
                        (3u32) as i32,
                    );
                    CalcCenterToCornerVec(
                        ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            (((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(142))
                            .cast::<u8>())
                            .read()) as i32) as isize
                                * 68,
                        ),
                        0u8,
                        3u8,
                        3u8,
                    );
                    StartSpriteAffineAnim(
                        ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            (((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(142))
                            .cast::<u8>())
                            .read()) as i32) as isize
                                * 68,
                        ),
                        0u8,
                    );
                } else {
                    StartSpriteAffineAnim(
                        ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            (((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(142))
                            .cast::<u8>())
                            .read()) as i32) as isize
                                * 68,
                        ),
                        0u8,
                    );
                }
                StartSpriteAffineAnim(
                    ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(142))
                        .cast::<u8>())
                        .wrapping_offset(1))
                        .read()) as i32) as isize
                            * 68,
                    ),
                    0u8,
                );
                ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    (((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(142))
                    .cast::<u8>())
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(32)
                .cast::<i16>())
                .write(60i16);
                ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(142))
                    .cast::<u8>())
                    .wrapping_offset(1))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(32)
                .cast::<i16>())
                .write(180i16);
                ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    (((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(142))
                    .cast::<u8>())
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(34)
                .cast::<i16>())
                .write(192i16);
                ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(142))
                    .cast::<u8>())
                    .wrapping_offset(1))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(34)
                .cast::<i16>())
                .write((-32i16));
                crate::c::bf_write(
                    (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        (((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(142))
                        .cast::<u8>())
                        .read()) as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(62),
                    2,
                    1,
                    (0u16) as i32,
                );
                crate::c::bf_write(
                    (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(142))
                        .cast::<u8>())
                        .wrapping_offset(1))
                        .read()) as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(62),
                    2,
                    1,
                    (0u16) as i32,
                );
                let __p36 = (((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(148)
                    .cast::<u16>();
                (__p36).write(((__p36).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 38i32 {
                let __p37 = (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    (((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(142))
                    .cast::<u8>())
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(38)
                .cast::<i16>();
                (__p37).write((((((__p37).read()) as i32).wrapping_sub(3i32)) as i16));
                let __p38 = (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(142))
                    .cast::<u8>())
                    .wrapping_offset(1))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(38)
                .cast::<i16>();
                (__p38).write((((((__p38).read()) as i32).wrapping_add(3i32)) as i16));
                if (((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    (((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(142))
                    .cast::<u8>())
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(38)
                .cast::<i16>())
                .read()) as i32)
                    < (-160i32))
                    && (((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        (((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(142))
                        .cast::<u8>())
                        .read()) as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(38)
                    .cast::<i16>())
                    .read()) as i32)
                        >= (-163i32))
                {
                    PlaySE(45u16);
                }
                if ((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    (((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(142))
                    .cast::<u8>())
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(38)
                .cast::<i16>())
                .read()) as i32)
                    < (-222i32)
                {
                    ((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(144))
                        .read()) as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(46))
                    .cast::<i16>())
                    .wrapping_offset(1))
                    .write(0i16);
                    ((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(145))
                        .read()) as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(46))
                    .cast::<i16>())
                    .wrapping_offset(1))
                    .write(0i16);
                    let __p39 = (((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(148)
                        .cast::<u16>();
                    (__p39).write(((__p39).read()).wrapping_add(1));
                    crate::c::bf_write(
                        (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            (((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(142))
                            .cast::<u8>())
                            .read()) as i32) as isize
                                * 68,
                        ))
                        .wrapping_add(62),
                        2,
                        1,
                        (1u16) as i32,
                    );
                    crate::c::bf_write(
                        (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            ((((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(142))
                            .cast::<u8>())
                            .wrapping_offset(1))
                            .read()) as i32) as isize
                                * 68,
                        ))
                        .wrapping_add(62),
                        2,
                        1,
                        (1u16) as i32,
                    );
                    BlendPalettes(1u32, 0u8, 65535u16);
                }
                break 'l1;
            }
            if __sw1 == 39i32 {
                let __p40 = (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(144))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(38)
                .cast::<i16>();
                (__p40).write((((((__p40).read()) as i32).wrapping_sub(3i32)) as i16));
                let __p41 = (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(145))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(38)
                .cast::<i16>();
                (__p41).write((((((__p41).read()) as i32).wrapping_add(3i32)) as i16));
                if ((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(144))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(38)
                .cast::<i16>())
                .read()) as i32)
                    <= (-222i32)
                {
                    BeginNormalPaletteFade(4294967295u32, (-1i8), 0u8, 16u8, 0u16);
                    let __p42 = (((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(148)
                        .cast::<u16>();
                    (__p42).write(((__p42).read()).wrapping_add(1));
                    DestroySprite(
                        ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            ((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(144))
                            .read()) as i32) as isize
                                * 68,
                        ),
                    );
                    DestroySprite(
                        ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            ((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(145))
                            .read()) as i32) as isize
                                * 68,
                        ),
                    );
                }
                break 'l1;
            }
            if __sw1 == 40i32 {
                if !((crate::c::bf_read(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
                    7,
                    1,
                    false,
                ) as u16)
                    != 0)
                {
                    let __p43 = (((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(148)
                        .cast::<u16>();
                    (__p43).write(((__p43).read()).wrapping_add(1));
                    SetTradeSequenceBgGpuRegs(1u8);
                    ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(224)
                        .cast::<i16>())
                    .write(166i16);
                    ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(144))
                    .write(CreateSprite(
                        (&raw const sSpriteTemplate_LinkMonGlow)
                            .cast::<u8>()
                            .cast_mut(),
                        128i16,
                        (-20i16),
                        3u8,
                    ));
                    ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(145))
                    .write(CreateSprite(
                        (&raw const sSpriteTemplate_LinkMonShadow)
                            .cast::<u8>()
                            .cast_mut(),
                        128i16,
                        (-20i16),
                        0u8,
                    ));
                    StartSpriteAnim(
                        ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            ((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(145))
                            .read()) as i32) as isize
                                * 68,
                        ),
                        1u8,
                    );
                }
                break 'l1;
            }
            if __sw1 == 41i32 {
                BeginNormalPaletteFade(4294967295u32, (-1i8), 16u8, 0u8, 0u16);
                let __p44 = (((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(148)
                    .cast::<u16>();
                (__p44).write(((__p44).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 42i32 {
                SetGpuReg(0u8, 4672u16);
                if !((crate::c::bf_read(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
                    7,
                    1,
                    false,
                ) as u16)
                    != 0)
                {
                    let __p45 = (((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(148)
                        .cast::<u16>();
                    (__p45).write(((__p45).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 43i32 {
                let __p46 = (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(144))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(38)
                .cast::<i16>();
                (__p46).write((((((__p46).read()) as i32).wrapping_add(3i32)) as i16));
                let __p47 = (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(145))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(38)
                .cast::<i16>();
                (__p47).write((((((__p47).read()) as i32).wrapping_add(3i32)) as i16));
                if ((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(144))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(38)
                .cast::<i16>())
                .read()) as i32)
                    .wrapping_add(
                        ((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            ((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(144))
                            .read()) as i32) as isize
                                * 68,
                        ))
                        .wrapping_add(34)
                        .cast::<i16>())
                        .read()) as i32),
                    )
                    == 64i32
                {
                    let __p48 = (((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(148)
                        .cast::<u16>();
                    (__p48).write(((__p48).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 44i32 {
                if (({
                    let __p49 = (((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(224)
                        .cast::<i16>();
                    let __v50 = (((((__p49).read()) as i32).wrapping_add(2i32)) as i16);
                    (__p49).write(__v50);
                    __v50
                }) as i32)
                    > 316i32
                {
                    ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(224)
                        .cast::<i16>())
                    .write(316i16);
                    let __p51 = (((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(148)
                        .cast::<u16>();
                    (__p51).write(((__p51).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 45i32 {
                DestroySprite(
                    ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(144))
                        .read()) as i32) as isize
                            * 68,
                    ),
                );
                DestroySprite(
                    ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(145))
                        .read()) as i32) as isize
                            * 68,
                    ),
                );
                let __p52 = (((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(148)
                    .cast::<u16>();
                (__p52).write(((__p52).read()).wrapping_add(1));
                ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(100)
                    .cast::<u32>())
                .write(0u32);
                break 'l1;
            }
            if __sw1 == 46i32 {
                if {
                    let __p53 = (((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(100)
                        .cast::<u32>();
                    let __t54 = ((__p53).read()).wrapping_add(1);
                    (__p53).write(__t54);
                    __t54
                } == 10u32
                {
                    let __p55 = (((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(148)
                        .cast::<u16>();
                    (__p55).write(((__p55).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 47i32 {
                if (({
                    let __p56 = (((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(224)
                        .cast::<i16>();
                    let __t57 = ((__p56).read()).wrapping_add(1);
                    (__p56).write(__t57);
                    __t57
                }) as i32)
                    > 348i32
                {
                    ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(224)
                        .cast::<i16>())
                    .write(348i16);
                    let __p58 = (((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(148)
                        .cast::<u16>();
                    (__p58).write(((__p58).read()).wrapping_add(1));
                }
                if (((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(224)
                    .cast::<i16>())
                .read()) as i32)
                    == 328i32)
                    && ((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(250))
                    .read())
                        != 0)
                {
                    ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(146))
                    .write(CreateSprite(
                        (&raw const sSpriteTemplate_CableEnd)
                            .cast::<u8>()
                            .cast_mut(),
                        128i16,
                        65i16,
                        0u8,
                    ));
                    ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(146))
                        .read()) as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(28)
                    .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                    .write(Some(SpriteCB_CableEndReceiving));
                }
                break 'l1;
            }
            if __sw1 == 48i32 {
                ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(145))
                    .write(CreateSprite(
                        (&raw const sSpriteTemplate_GbaScreenFlash_Long)
                            .cast::<u8>()
                            .cast_mut(),
                        120i16,
                        80i16,
                        0u8,
                    ));
                ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(148)
                    .cast::<u16>())
                .write(50u16);
                break 'l1;
            }
            if __sw1 == 50i32 {
                if (crate::c::bf_read(
                    (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(145))
                        .read()) as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(63),
                    4,
                    1,
                    false,
                ) as u16)
                    != 0
                {
                    DestroySprite(
                        ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            ((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(145))
                            .read()) as i32) as isize
                                * 68,
                        ),
                    );
                    SetTradeSequenceBgGpuRegs(6u8);
                    let __p59 = (((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(148)
                        .cast::<u16>();
                    (__p59).write(((__p59).read()).wrapping_add(1));
                    PlaySE(159u16);
                }
                break 'l1;
            }
            if __sw1 == 51i32 {
                if ((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(234)
                    .cast::<u16>())
                .read()) as i32)
                    < 1024i32
                {
                    let __p60 = (((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(234)
                        .cast::<u16>();
                    (__p60).write((((((__p60).read()) as i32).wrapping_add(52i32)) as u16));
                } else {
                    ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(234)
                        .cast::<u16>())
                    .write(1024u16);
                    let __p61 = (((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(148)
                        .cast::<u16>();
                    (__p61).write(((__p61).read()).wrapping_add(1));
                }
                ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(232)
                    .cast::<u16>())
                .write(
                    ((crate::c::div_i32(
                        32768i32,
                        ((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(234)
                            .cast::<u16>())
                        .read()) as i32),
                    )) as u16),
                );
                break 'l1;
            }
            if __sw1 == 52i32 {
                BeginNormalPaletteFade(4294967295u32, 0i8, 0u8, 16u8, 0u16);
                ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(148)
                    .cast::<u16>())
                .write(60u16);
                break 'l1;
            }
            if __sw1 == 60i32 {
                if !((crate::c::bf_read(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
                    7,
                    1,
                    false,
                ) as u16)
                    != 0)
                {
                    SetTradeSequenceBgGpuRegs(5u8);
                    SetTradeSequenceBgGpuRegs(7u8);
                    crate::c::bf_write(
                        ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(8),
                        7,
                        1,
                        (1u16) as i32,
                    );
                    let __p62 = (((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(148)
                        .cast::<u16>();
                    (__p62).write(((__p62).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 61i32 {
                crate::c::bf_write(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(8),
                    7,
                    1,
                    (0u16) as i32,
                );
                BeginNormalPaletteFade(4294967295u32, 0i8, 16u8, 0u8, 0u16);
                let __p63 = (((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(148)
                    .cast::<u16>();
                (__p63).write(((__p63).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 62i32 {
                SetGpuReg(0u8, 5184u16);
                if !((crate::c::bf_read(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
                    7,
                    1,
                    false,
                ) as u16)
                    != 0)
                {
                    let __p64 = (((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(148)
                        .cast::<u16>();
                    (__p64).write(((__p64).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 63i32 {
                ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(211))
                    .write(CreateSprite(
                        (&raw const sSpriteTemplate_Pokeball)
                            .cast::<u8>()
                            .cast_mut(),
                        120i16,
                        (-8i16),
                        0u8,
                    ));
                ((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(211))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(46))
                .cast::<i16>())
                .wrapping_offset(3))
                .write(74i16);
                ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(211))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                .write(Some(SpriteCB_BouncingPokeballArrive));
                StartSpriteAnim(
                    ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(211))
                        .read()) as i32) as isize
                            * 68,
                    ),
                    1u8,
                );
                StartSpriteAffineAnim(
                    ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(211))
                        .read()) as i32) as isize
                            * 68,
                    ),
                    2u8,
                );
                BlendPalettes(
                    ((crate::c::shl_i32(
                        1i32,
                        (((16i32).wrapping_add(
                            ((crate::c::bf_read(
                                (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                    ((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(211))
                                    .read()) as i32) as isize
                                        * 68,
                                ))
                                .wrapping_add(5),
                                4,
                                4,
                                false,
                            ) as u16) as i32),
                        )) as u32),
                    )) as u32),
                    16u8,
                    65535u16,
                );
                let __p65 = (((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(148)
                    .cast::<u16>();
                (__p65).write(((__p65).read()).wrapping_add(1));
                ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(100)
                    .cast::<u32>())
                .write(0u32);
                break 'l1;
            }
            if __sw1 == 64i32 {
                BeginNormalPaletteFade(
                    ((crate::c::shl_i32(
                        1i32,
                        (((16i32).wrapping_add(
                            ((crate::c::bf_read(
                                (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                    ((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(211))
                                    .read()) as i32) as isize
                                        * 68,
                                ))
                                .wrapping_add(5),
                                4,
                                4,
                                false,
                            ) as u16) as i32),
                        )) as u32),
                    )) as u32),
                    1i8,
                    16u8,
                    0u8,
                    65535u16,
                );
                let __p66 = (((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(148)
                    .cast::<u16>();
                (__p66).write(((__p66).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 65i32 {
                if core::mem::transmute::<_, usize>(
                    ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(211))
                        .read()) as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(28)
                    .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                    .read(),
                ) == (SpriteCallbackDummy as *const () as usize)
                {
                    HandleLoadSpecialPokePic_2(
                        ((&raw mut gMonFrontPicTable).cast::<u8>()).wrapping_offset(
                            ((((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(240))
                            .cast::<u16>())
                            .wrapping_offset(1))
                            .read()) as i32) as isize
                                * 8,
                        ),
                        ((((((&raw mut gMonSpritesGfxPtr).cast::<*mut u8>()).read())
                            .wrapping_add(4))
                        .cast::<*mut u8>())
                        .wrapping_offset(3))
                        .read(),
                        ((((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(240))
                        .cast::<u16>())
                        .wrapping_offset(1))
                        .read()) as i32),
                        ((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(104))
                        .cast::<u32>())
                        .wrapping_offset(1))
                        .read(),
                    );
                    let __p67 = (((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(148)
                        .cast::<u16>();
                    (__p67).write(((__p67).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 66i32 {
                ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(142))
                    .cast::<u8>())
                    .wrapping_offset(1))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(32)
                .cast::<i16>())
                .write(120i16);
                ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(142))
                    .cast::<u8>())
                    .wrapping_offset(1))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(34)
                .cast::<i16>())
                .write(
                    ((((((((&raw mut gMonFrontPicCoords).cast::<u8>()).wrapping_offset(
                        ((((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(240))
                        .cast::<u16>())
                        .wrapping_offset(1))
                        .read()) as i32) as isize
                            * 4,
                    ))
                    .wrapping_add(1))
                    .read()) as i32)
                        .wrapping_add(60i32)) as i16),
                );
                ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(142))
                    .cast::<u8>())
                    .wrapping_offset(1))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(36)
                .cast::<i16>())
                .write(0i16);
                ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(142))
                    .cast::<u8>())
                    .wrapping_offset(1))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(38)
                .cast::<i16>())
                .write(0i16);
                StartSpriteAnim(
                    ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(142))
                        .cast::<u8>())
                        .wrapping_offset(1))
                        .read()) as i32) as isize
                            * 68,
                    ),
                    0u8,
                );
                CreatePokeballSpriteToReleaseMon(
                    ((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(142))
                    .cast::<u8>())
                    .wrapping_offset(1))
                    .read(),
                    ((crate::c::bf_read(
                        (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            ((((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(142))
                            .cast::<u8>())
                            .wrapping_offset(1))
                            .read()) as i32) as isize
                                * 68,
                        ))
                        .wrapping_add(5),
                        4,
                        4,
                        false,
                    ) as u16) as u8),
                    120u8,
                    84u8,
                    2u8,
                    1u8,
                    20u8,
                    1048575u32,
                    ((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(240))
                    .cast::<u16>())
                    .wrapping_offset(1))
                    .read(),
                );
                FreeSpriteOamMatrix(
                    ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(211))
                        .read()) as i32) as isize
                            * 68,
                    ),
                );
                DestroySprite(
                    ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(211))
                        .read()) as i32) as isize
                            * 68,
                    ),
                );
                let __p68 = (((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(148)
                    .cast::<u16>();
                (__p68).write(((__p68).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 67i32 {
                SetGpuReg(0u8, 5440u16);
                StringExpandPlaceholders(
                    (&raw mut gStringVar4).cast::<u8>(),
                    (&raw mut gText_XSentOverY).cast::<u8>(),
                );
                DrawTextOnTradeWindow(0u8, (&raw mut gStringVar4).cast::<u8>(), 0u8);
                ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(148)
                    .cast::<u16>())
                .write(167u16);
                ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(100)
                    .cast::<u32>())
                .write(0u32);
                break 'l1;
            }
            if __sw1 == 167i32 {
                if {
                    let __p69 = (((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(100)
                        .cast::<u32>();
                    let __t70 = ((__p69).read()).wrapping_add(1);
                    (__p69).write(__t70);
                    __t70
                } > 60u32
                {
                    ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(148)
                        .cast::<u16>())
                    .write(267u16);
                    ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(100)
                        .cast::<u32>())
                    .write(0u32);
                }
                break 'l1;
            }
            if __sw1 == 267i32 {
                if (IsCryFinished()) != 0 {
                    ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(148)
                        .cast::<u16>())
                    .write(68u16);
                }
                break 'l1;
            }
            if __sw1 == 68i32 {
                if {
                    let __p71 = (((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(100)
                        .cast::<u32>();
                    let __t72 = ((__p71).read()).wrapping_add(1);
                    (__p71).write(__t72);
                    __t72
                } == 10u32
                {
                    PlayFanfare(371u16);
                }
                if ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(100)
                    .cast::<u32>())
                .read()
                    == 250u32
                {
                    let __p73 = (((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(148)
                        .cast::<u16>();
                    (__p73).write(((__p73).read()).wrapping_add(1));
                    StringExpandPlaceholders(
                        (&raw mut gStringVar4).cast::<u8>(),
                        (&raw mut gText_TakeGoodCareOfX).cast::<u8>(),
                    );
                    DrawTextOnTradeWindow(0u8, (&raw mut gStringVar4).cast::<u8>(), 0u8);
                    ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(100)
                        .cast::<u32>())
                    .write(0u32);
                }
                break 'l1;
            }
            if __sw1 == 69i32 {
                if {
                    let __p74 = (((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(100)
                        .cast::<u32>();
                    let __t75 = ((__p74).read()).wrapping_add(1);
                    (__p74).write(__t75);
                    __t75
                } == 60u32
                {
                    let __p76 = (((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(148)
                        .cast::<u16>();
                    (__p76).write(((__p76).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 70i32 {
                CheckPartnersMonForRibbons();
                let __p77 = (((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(148)
                    .cast::<u16>();
                (__p77).write(((__p77).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 71i32 {
                if (((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(238))
                .read())
                    != 0
                {
                    return 1u8;
                } else {
                    if ((((((&raw mut gMain).cast::<u8>())
                        .wrapping_add(46)
                        .cast::<u16>())
                    .read()) as i32)
                        & 1i32)
                        != 0
                    {
                        let __p78 = (((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(148)
                            .cast::<u16>();
                        (__p78).write(((__p78).read()).wrapping_add(1));
                    }
                }
                break 'l1;
            }
            if __sw1 == 72i32 {
                TradeMons(
                    ((((&raw mut gSpecialVar_0x8005).cast::<u16>()).read()) as u8),
                    0u8,
                );
                ((&raw mut gCB2_AfterEvolution).cast::<Option<unsafe extern "C" fn()>>())
                    .write(Some(CB2_InGameTrade));
                evoTarget = GetEvolutionTargetSpecies(
                    ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                        (((((&raw mut gSelectedTradeMonPositions).cast::<u8>()).cast::<u8>())
                            .read()) as i32) as isize
                            * 100,
                    ),
                    1u8,
                    0u16,
                );
                if ((evoTarget) as i32) != 0i32 {
                    TradeEvolutionScene(
                        ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                            (((((&raw mut gSelectedTradeMonPositions).cast::<u8>()).cast::<u8>())
                                .read()) as i32) as isize
                                * 100,
                        ),
                        evoTarget,
                        ((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(142))
                        .cast::<u8>())
                        .wrapping_offset(1))
                        .read(),
                        (((&raw mut gSelectedTradeMonPositions).cast::<u8>()).cast::<u8>()).read(),
                    );
                }
                let __p79 = (((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(148)
                    .cast::<u16>();
                (__p79).write(((__p79).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 73i32 {
                BeginNormalPaletteFade(4294967295u32, 0i8, 0u8, 16u8, 0u16);
                let __p80 = (((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(148)
                    .cast::<u16>();
                (__p80).write(((__p80).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 74i32 {
                if !((crate::c::bf_read(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
                    7,
                    1,
                    false,
                ) as u16)
                    != 0)
                {
                    PlayNewMapMusic(
                        ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(244)
                            .cast::<u16>())
                        .read(),
                    );
                    if !(((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read()).is_null() {
                        FreeAllWindowBuffers();
                        Free(GetBgTilemapBuffer(3u8));
                        Free(GetBgTilemapBuffer(1u8));
                        Free(GetBgTilemapBuffer(0u8));
                        FreeMonSpritesGfx();
                        {
                            Free(((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read());
                            ((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>())
                                .write(core::ptr::null_mut());
                        }
                    }
                    SetMainCallback2(Some(CB2_ReturnToField));
                    BufferInGameTradeMonName();
                }
                break 'l1;
            }
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn DoTradeAnim_Wireless() -> u8 {
    unsafe {
        let mut evoTarget: u16 = 0u16;
        'l1: {
            let __sw1 = ((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(148)
                .cast::<u16>())
            .read()) as i32);
            if __sw1 == 0i32 {
                crate::c::bf_write(
                    (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        (((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(142))
                        .cast::<u8>())
                        .read()) as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(62),
                    2,
                    1,
                    (0u16) as i32,
                );
                ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    (((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(142))
                    .cast::<u8>())
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(36)
                .cast::<i16>())
                .write((-180i16));
                ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    (((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(142))
                    .cast::<u8>())
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(38)
                .cast::<i16>())
                .write(
                    ((((((&raw mut gMonFrontPicCoords).cast::<u8>()).wrapping_offset(
                        (((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(240))
                        .cast::<u16>())
                        .read()) as i32) as isize
                            * 4,
                    ))
                    .wrapping_add(1))
                    .read()) as i16),
                );
                let __p2 = (((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(148)
                    .cast::<u16>();
                (__p2).write(((__p2).read()).wrapping_add(1));
                ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(244)
                    .cast::<u16>())
                .write(GetCurrentMapMusic());
                PlayNewMapMusic(377u16);
                break 'l1;
            }
            if __sw1 == 1i32 {
                if ((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(230)
                    .cast::<i16>())
                .read()) as i32)
                    > 0i32
                {
                    let __p3 = (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        (((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(142))
                        .cast::<u8>())
                        .read()) as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(36)
                    .cast::<i16>();
                    (__p3).write((((((__p3).read()) as i32).wrapping_add(3i32)) as i16));
                    let __p4 = (((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(230)
                        .cast::<i16>();
                    (__p4).write((((((__p4).read()) as i32).wrapping_sub(3i32)) as i16));
                } else {
                    ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        (((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(142))
                        .cast::<u8>())
                        .read()) as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(36)
                    .cast::<i16>())
                    .write(0i16);
                    ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(230)
                        .cast::<i16>())
                    .write(0i16);
                    ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(148)
                        .cast::<u16>())
                    .write(10u16);
                }
                break 'l1;
            }
            if __sw1 == 10i32 {
                StringExpandPlaceholders(
                    (&raw mut gStringVar4).cast::<u8>(),
                    (&raw mut gText_XWillBeSentToY).cast::<u8>(),
                );
                DrawTextOnTradeWindow(0u8, (&raw mut gStringVar4).cast::<u8>(), 0u8);
                if (((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(240))
                .cast::<u16>())
                .read()) as i32)
                    != 412i32
                {
                    PlayCry_Normal(
                        (((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(240))
                        .cast::<u16>())
                        .read(),
                        0i8,
                    );
                }
                ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(148)
                    .cast::<u16>())
                .write(11u16);
                ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(100)
                    .cast::<u32>())
                .write(0u32);
                break 'l1;
            }
            if __sw1 == 11i32 {
                if {
                    let __p5 = (((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(100)
                        .cast::<u32>();
                    let __t6 = ((__p5).read()).wrapping_add(1);
                    (__p5).write(__t6);
                    __t6
                } == 80u32
                {
                    ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(210))
                    .write(CreateTradePokeballSprite(
                        (((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(142))
                        .cast::<u8>())
                        .read(),
                        ((crate::c::bf_read(
                            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                (((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(142))
                                .cast::<u8>())
                                .read()) as i32) as isize
                                    * 68,
                            ))
                            .wrapping_add(5),
                            4,
                            4,
                            false,
                        ) as u16) as u8),
                        120u8,
                        32u8,
                        2u8,
                        1u8,
                        20u8,
                        1048575u32,
                    ));
                    let __p7 = (((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(148)
                        .cast::<u16>();
                    (__p7).write(((__p7).read()).wrapping_add(1));
                    StringExpandPlaceholders(
                        (&raw mut gStringVar4).cast::<u8>(),
                        (&raw mut gText_ByeByeVar1).cast::<u8>(),
                    );
                    DrawTextOnTradeWindow(0u8, (&raw mut gStringVar4).cast::<u8>(), 0u8);
                }
                break 'l1;
            }
            if __sw1 == 12i32 {
                if core::mem::transmute::<_, usize>(
                    ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(210))
                        .read()) as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(28)
                    .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                    .read(),
                ) == (SpriteCallbackDummy as *const () as usize)
                {
                    ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(211))
                    .write(CreateSprite(
                        (&raw const sSpriteTemplate_Pokeball)
                            .cast::<u8>()
                            .cast_mut(),
                        120i16,
                        32i16,
                        0u8,
                    ));
                    ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(211))
                        .read()) as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(28)
                    .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                    .write(Some(SpriteCB_BouncingPokeballDepart));
                    DestroySprite(
                        ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            ((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(210))
                            .read()) as i32) as isize
                                * 68,
                        ),
                    );
                    let __p8 = (((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(148)
                        .cast::<u16>();
                    (__p8).write(((__p8).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 13i32 {
                break 'l1;
            }
            if __sw1 == 14i32 {
                BeginNormalPaletteFade(4294967295u32, 0i8, 0u8, 16u8, 0u16);
                ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(148)
                    .cast::<u16>())
                .write(20u16);
                break 'l1;
            }
            if __sw1 == 20i32 {
                if !((crate::c::bf_read(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
                    7,
                    1,
                    false,
                ) as u16)
                    != 0)
                {
                    SetTradeSequenceBgGpuRegs(4u8);
                    FillWindowPixelBuffer(0u8, 255u8);
                    CopyWindowToVram(0u8, 3u8);
                    let __p9 = (((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(148)
                        .cast::<u16>();
                    (__p9).write(((__p9).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 21i32 {
                BeginNormalPaletteFade(4294967295u32, (-1i8), 16u8, 0u8, 0u16);
                let __p10 = (((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(148)
                    .cast::<u16>();
                (__p10).write(((__p10).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 22i32 {
                if !((crate::c::bf_read(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
                    7,
                    1,
                    false,
                ) as u16)
                    != 0)
                {
                    ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(148)
                        .cast::<u16>())
                    .write(23u16);
                }
                break 'l1;
            }
            if __sw1 == 23i32 {
                if ((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(234)
                    .cast::<u16>())
                .read()) as i32)
                    > 256i32
                {
                    let __p11 = (((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(234)
                        .cast::<u16>();
                    (__p11).write((((((__p11).read()) as i32).wrapping_sub(52i32)) as u16));
                } else {
                    SetTradeSequenceBgGpuRegs(1u8);
                    ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(234)
                        .cast::<u16>())
                    .write(128u16);
                    ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(148)
                        .cast::<u16>())
                    .write(124u16);
                    ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(100)
                        .cast::<u32>())
                    .write(0u32);
                }
                ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(232)
                    .cast::<u16>())
                .write(
                    ((crate::c::div_i32(
                        32768i32,
                        ((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(234)
                            .cast::<u16>())
                        .read()) as i32),
                    )) as u16),
                );
                break 'l1;
            }
            if __sw1 == 124i32 {
                if {
                    let __p12 = (((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(100)
                        .cast::<u32>();
                    let __t13 = ((__p12).read()).wrapping_add(1);
                    (__p12).write(__t13);
                    __t13
                } > 20u32
                {
                    SetTradeSequenceBgGpuRegs(3u8);
                    ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(145))
                    .write(CreateSprite(
                        (&raw const sSpriteTemplate_GbaScreenFlash_Short)
                            .cast::<u8>()
                            .cast_mut(),
                        120i16,
                        80i16,
                        0u8,
                    ));
                    let __p14 = (((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(148)
                        .cast::<u16>();
                    (__p14).write(((__p14).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 125i32 {
                if (crate::c::bf_read(
                    (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(145))
                        .read()) as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(63),
                    4,
                    1,
                    false,
                ) as u16)
                    != 0
                {
                    DestroySprite(
                        ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            ((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(145))
                            .read()) as i32) as isize
                                * 68,
                        ),
                    );
                    SetGpuReg(80u8, 1106u16);
                    SetGpuReg(82u8, 1040u16);
                    CreateTask(Some(Task_AnimateWirelessSignal), 5u8);
                    let __p15 = (((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(148)
                        .cast::<u16>();
                    (__p15).write(((__p15).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 126i32 {
                if !((FuncIsActiveTask(Some(Task_AnimateWirelessSignal))) != 0) {
                    ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(148)
                        .cast::<u16>())
                    .write(26u16);
                }
                break 'l1;
            }
            if __sw1 == 26i32 {
                if (({
                    let __p16 = (((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(224)
                        .cast::<i16>();
                    let __t17 = ((__p16).read()).wrapping_sub(1);
                    (__p16).write(__t17);
                    __t17
                }) as i32)
                    == 316i32
                {
                    let __p18 = (((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(148)
                        .cast::<u16>();
                    (__p18).write(((__p18).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 27i32 {
                ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(144))
                    .write(CreateSprite(
                        (&raw const sSpriteTemplate_LinkMonGlow)
                            .cast::<u8>()
                            .cast_mut(),
                        120i16,
                        80i16,
                        3u8,
                    ));
                ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(144))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                .write(Some(SpriteCB_LinkMonGlowWireless));
                ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(145))
                    .write(CreateSprite(
                        (&raw const sSpriteTemplate_LinkMonShadow)
                            .cast::<u8>()
                            .cast_mut(),
                        120i16,
                        80i16,
                        0u8,
                    ));
                StartSpriteAnim(
                    ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(145))
                        .read()) as i32) as isize
                            * 68,
                    ),
                    1u8,
                );
                let __p19 = (((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(148)
                    .cast::<u16>();
                (__p19).write(((__p19).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 28i32 {
                if (({
                    let __p20 = (((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(224)
                        .cast::<i16>();
                    let __v21 = (((((__p20).read()) as i32).wrapping_sub(3i32)) as i16);
                    (__p20).write(__v21);
                    __v21
                }) as i32)
                    == 166i32
                {
                    ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(148)
                        .cast::<u16>())
                    .write(200u16);
                }
                SetGpuReg(0u8, 4673u16);
                break 'l1;
            }
            if __sw1 == 200i32 {
                let __p22 = (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(144))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(34)
                .cast::<i16>();
                (__p22).write((((((__p22).read()) as i32).wrapping_sub(2i32)) as i16));
                let __p23 = (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(145))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(34)
                .cast::<i16>();
                (__p23).write((((((__p23).read()) as i32).wrapping_sub(2i32)) as i16));
                if ((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(144))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(34)
                .cast::<i16>())
                .read()) as i32)
                    < (-8i32)
                {
                    ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(148)
                        .cast::<u16>())
                    .write(29u16);
                }
                break 'l1;
            }
            if __sw1 == 29i32 {
                BeginNormalPaletteFade(4294967295u32, (-1i8), 0u8, 16u8, 0u16);
                ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(148)
                    .cast::<u16>())
                .write(30u16);
                break 'l1;
            }
            if __sw1 == 30i32 {
                if !((crate::c::bf_read(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
                    7,
                    1,
                    false,
                ) as u16)
                    != 0)
                {
                    DestroySprite(
                        ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            ((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(144))
                            .read()) as i32) as isize
                                * 68,
                        ),
                    );
                    DestroySprite(
                        ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            ((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(145))
                            .read()) as i32) as isize
                                * 68,
                        ),
                    );
                    SetTradeSequenceBgGpuRegs(2u8);
                    let __p24 = (((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(148)
                        .cast::<u16>();
                    (__p24).write(((__p24).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 31i32 {
                BeginNormalPaletteFade(4294967295u32, (-1i8), 16u8, 0u8, 0u16);
                ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(144))
                    .write(CreateSprite(
                        (&raw const sSpriteTemplate_LinkMonShadow)
                            .cast::<u8>()
                            .cast_mut(),
                        111i16,
                        170i16,
                        0u8,
                    ));
                ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(145))
                    .write(CreateSprite(
                        (&raw const sSpriteTemplate_LinkMonShadow)
                            .cast::<u8>()
                            .cast_mut(),
                        129i16,
                        (-10i16),
                        0u8,
                    ));
                let __p25 = (((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(148)
                    .cast::<u16>();
                (__p25).write(((__p25).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 32i32 {
                if !((crate::c::bf_read(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
                    7,
                    1,
                    false,
                ) as u16)
                    != 0)
                {
                    PlaySE(46u16);
                    let __p26 = (((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(148)
                        .cast::<u16>();
                    (__p26).write(((__p26).read()).wrapping_add(1));
                }
                let __p27 = (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(144))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(38)
                .cast::<i16>();
                (__p27).write((((((__p27).read()) as i32).wrapping_sub(3i32)) as i16));
                let __p28 = (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(145))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(38)
                .cast::<i16>();
                (__p28).write((((((__p28).read()) as i32).wrapping_add(3i32)) as i16));
                break 'l1;
            }
            if __sw1 == 33i32 {
                let __p29 = (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(144))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(38)
                .cast::<i16>();
                (__p29).write((((((__p29).read()) as i32).wrapping_sub(3i32)) as i16));
                let __p30 = (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(145))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(38)
                .cast::<i16>();
                (__p30).write((((((__p30).read()) as i32).wrapping_add(3i32)) as i16));
                if ((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(144))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(38)
                .cast::<i16>())
                .read()) as i32)
                    <= (-90i32)
                {
                    ((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(144))
                        .read()) as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(46))
                    .cast::<i16>())
                    .wrapping_offset(1))
                    .write(1i16);
                    ((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(145))
                        .read()) as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(46))
                    .cast::<i16>())
                    .wrapping_offset(1))
                    .write(1i16);
                    let __p31 = (((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(148)
                        .cast::<u16>();
                    (__p31).write(((__p31).read()).wrapping_add(1));
                    CreateTask(Some(Task_OpenCenterWhiteColumn), 5u8);
                }
                break 'l1;
            }
            if __sw1 == 34i32 {
                BlendPalettes(8u32, 16u8, 65535u16);
                let __p32 = (((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(148)
                    .cast::<u16>();
                (__p32).write(((__p32).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 35i32 {
                BlendPalettes(8u32, 16u8, 65535u16);
                let __p33 = (((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(148)
                    .cast::<u16>();
                (__p33).write(((__p33).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 36i32 {
                BlendPalettes(8u32, 16u8, 65535u16);
                let __p34 = (((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(148)
                    .cast::<u16>();
                (__p34).write(((__p34).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 37i32 {
                if !((IsMonSpriteNotFlipped(
                    (((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(240))
                    .cast::<u16>())
                    .read(),
                )) != 0)
                {
                    ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        (((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(142))
                        .cast::<u8>())
                        .read()) as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(16)
                    .cast::<*mut *mut u8>())
                    .write(
                        ((&raw const sAffineAnims_CrossingMonPics)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<*mut u8>())
                        .cast::<*mut u8>(),
                    );
                    crate::c::bf_write(
                        (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            (((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(142))
                            .cast::<u8>())
                            .read()) as i32) as isize
                                * 68,
                        ))
                        .wrapping_add(1),
                        0,
                        2,
                        (3u32) as i32,
                    );
                    CalcCenterToCornerVec(
                        ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            (((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(142))
                            .cast::<u8>())
                            .read()) as i32) as isize
                                * 68,
                        ),
                        0u8,
                        3u8,
                        3u8,
                    );
                    StartSpriteAffineAnim(
                        ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            (((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(142))
                            .cast::<u8>())
                            .read()) as i32) as isize
                                * 68,
                        ),
                        0u8,
                    );
                } else {
                    StartSpriteAffineAnim(
                        ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            (((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(142))
                            .cast::<u8>())
                            .read()) as i32) as isize
                                * 68,
                        ),
                        0u8,
                    );
                }
                StartSpriteAffineAnim(
                    ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(142))
                        .cast::<u8>())
                        .wrapping_offset(1))
                        .read()) as i32) as isize
                            * 68,
                    ),
                    0u8,
                );
                ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    (((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(142))
                    .cast::<u8>())
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(32)
                .cast::<i16>())
                .write(40i16);
                ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(142))
                    .cast::<u8>())
                    .wrapping_offset(1))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(32)
                .cast::<i16>())
                .write(200i16);
                ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    (((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(142))
                    .cast::<u8>())
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(34)
                .cast::<i16>())
                .write(192i16);
                ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(142))
                    .cast::<u8>())
                    .wrapping_offset(1))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(34)
                .cast::<i16>())
                .write((-32i16));
                crate::c::bf_write(
                    (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        (((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(142))
                        .cast::<u8>())
                        .read()) as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(62),
                    2,
                    1,
                    (0u16) as i32,
                );
                crate::c::bf_write(
                    (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(142))
                        .cast::<u8>())
                        .wrapping_offset(1))
                        .read()) as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(62),
                    2,
                    1,
                    (0u16) as i32,
                );
                let __p35 = (((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(148)
                    .cast::<u16>();
                (__p35).write(((__p35).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 38i32 {
                let __p36 = (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    (((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(142))
                    .cast::<u8>())
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(38)
                .cast::<i16>();
                (__p36).write((((((__p36).read()) as i32).wrapping_sub(3i32)) as i16));
                let __p37 = (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(142))
                    .cast::<u8>())
                    .wrapping_offset(1))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(38)
                .cast::<i16>();
                (__p37).write((((((__p37).read()) as i32).wrapping_add(3i32)) as i16));
                if (((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    (((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(142))
                    .cast::<u8>())
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(38)
                .cast::<i16>())
                .read()) as i32)
                    < (-160i32))
                    && (((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        (((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(142))
                        .cast::<u8>())
                        .read()) as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(38)
                    .cast::<i16>())
                    .read()) as i32)
                        >= (-163i32))
                {
                    PlaySE(45u16);
                }
                if ((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    (((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(142))
                    .cast::<u8>())
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(38)
                .cast::<i16>())
                .read()) as i32)
                    < (-222i32)
                {
                    ((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(144))
                        .read()) as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(46))
                    .cast::<i16>())
                    .wrapping_offset(1))
                    .write(0i16);
                    ((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(145))
                        .read()) as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(46))
                    .cast::<i16>())
                    .wrapping_offset(1))
                    .write(0i16);
                    let __p38 = (((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(148)
                        .cast::<u16>();
                    (__p38).write(((__p38).read()).wrapping_add(1));
                    crate::c::bf_write(
                        (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            (((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(142))
                            .cast::<u8>())
                            .read()) as i32) as isize
                                * 68,
                        ))
                        .wrapping_add(62),
                        2,
                        1,
                        (1u16) as i32,
                    );
                    crate::c::bf_write(
                        (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            ((((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(142))
                            .cast::<u8>())
                            .wrapping_offset(1))
                            .read()) as i32) as isize
                                * 68,
                        ))
                        .wrapping_add(62),
                        2,
                        1,
                        (1u16) as i32,
                    );
                    CreateTask(Some(Task_CloseCenterWhiteColumn), 5u8);
                }
                break 'l1;
            }
            if __sw1 == 39i32 {
                let __p39 = (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(144))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(38)
                .cast::<i16>();
                (__p39).write((((((__p39).read()) as i32).wrapping_sub(3i32)) as i16));
                let __p40 = (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(145))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(38)
                .cast::<i16>();
                (__p40).write((((((__p40).read()) as i32).wrapping_add(3i32)) as i16));
                if ((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(144))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(38)
                .cast::<i16>())
                .read()) as i32)
                    <= (-222i32)
                {
                    BeginNormalPaletteFade(4294967295u32, (-1i8), 0u8, 16u8, 0u16);
                    let __p41 = (((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(148)
                        .cast::<u16>();
                    (__p41).write(((__p41).read()).wrapping_add(1));
                    DestroySprite(
                        ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            ((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(144))
                            .read()) as i32) as isize
                                * 68,
                        ),
                    );
                    DestroySprite(
                        ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            ((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(145))
                            .read()) as i32) as isize
                                * 68,
                        ),
                    );
                }
                break 'l1;
            }
            if __sw1 == 40i32 {
                if !((crate::c::bf_read(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
                    7,
                    1,
                    false,
                ) as u16)
                    != 0)
                {
                    let __p42 = (((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(148)
                        .cast::<u16>();
                    (__p42).write(((__p42).read()).wrapping_add(1));
                    SetTradeSequenceBgGpuRegs(1u8);
                    ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(224)
                        .cast::<i16>())
                    .write(166i16);
                    SetTradeSequenceBgGpuRegs(3u8);
                    ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(228)
                        .cast::<i16>())
                    .write(412i16);
                    ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(144))
                    .write(CreateSprite(
                        (&raw const sSpriteTemplate_LinkMonGlow)
                            .cast::<u8>()
                            .cast_mut(),
                        120i16,
                        (-20i16),
                        3u8,
                    ));
                    ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(144))
                        .read()) as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(28)
                    .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                    .write(Some(SpriteCB_LinkMonGlowWireless));
                    ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(145))
                    .write(CreateSprite(
                        (&raw const sSpriteTemplate_LinkMonShadow)
                            .cast::<u8>()
                            .cast_mut(),
                        120i16,
                        (-20i16),
                        0u8,
                    ));
                    StartSpriteAnim(
                        ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            ((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(145))
                            .read()) as i32) as isize
                                * 68,
                        ),
                        1u8,
                    );
                }
                break 'l1;
            }
            if __sw1 == 41i32 {
                BeginNormalPaletteFade(4294967295u32, (-1i8), 16u8, 0u8, 0u16);
                let __p43 = (((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(148)
                    .cast::<u16>();
                (__p43).write(((__p43).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 42i32 {
                SetGpuReg(0u8, 4672u16);
                if !((crate::c::bf_read(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
                    7,
                    1,
                    false,
                ) as u16)
                    != 0)
                {
                    let __p44 = (((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(148)
                        .cast::<u16>();
                    (__p44).write(((__p44).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 43i32 {
                let __p45 = (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(144))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(38)
                .cast::<i16>();
                (__p45).write((((((__p45).read()) as i32).wrapping_add(4i32)) as i16));
                let __p46 = (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(145))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(38)
                .cast::<i16>();
                (__p46).write((((((__p46).read()) as i32).wrapping_add(4i32)) as i16));
                if ((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(144))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(38)
                .cast::<i16>())
                .read()) as i32)
                    .wrapping_add(
                        ((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            ((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(144))
                            .read()) as i32) as isize
                                * 68,
                        ))
                        .wrapping_add(34)
                        .cast::<i16>())
                        .read()) as i32),
                    )
                    == 64i32
                {
                    ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(148)
                        .cast::<u16>())
                    .write(144u16);
                    ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(100)
                        .cast::<u32>())
                    .write(0u32);
                }
                break 'l1;
            }
            if __sw1 == 144i32 {
                SetGpuReg(0u8, 5696u16);
                let __p47 = (((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(224)
                    .cast::<i16>();
                (__p47).write((((((__p47).read()) as i32).wrapping_add(3i32)) as i16));
                let __p48 = (((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(228)
                    .cast::<i16>();
                (__p48).write((((((__p48).read()) as i32).wrapping_add(3i32)) as i16));
                if {
                    let __p49 = (((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(100)
                        .cast::<u32>();
                    let __t50 = ((__p49).read()).wrapping_add(1);
                    (__p49).write(__t50);
                    __t50
                } == 10u32
                {
                    let mut taskId: u8 = CreateTask(Some(Task_AnimateWirelessSignal), 5u8);
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(2))
                    .write(1i16);
                }
                if ((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(224)
                    .cast::<i16>())
                .read()) as i32)
                    > 316i32
                {
                    ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(224)
                        .cast::<i16>())
                    .write(316i16);
                    let __p51 = (((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(148)
                        .cast::<u16>();
                    (__p51).write(((__p51).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 145i32 {
                DestroySprite(
                    ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(144))
                        .read()) as i32) as isize
                            * 68,
                    ),
                );
                DestroySprite(
                    ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(145))
                        .read()) as i32) as isize
                            * 68,
                    ),
                );
                let __p52 = (((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(148)
                    .cast::<u16>();
                (__p52).write(((__p52).read()).wrapping_add(1));
                ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(100)
                    .cast::<u32>())
                .write(0u32);
                break 'l1;
            }
            if __sw1 == 146i32 {
                if !((FuncIsActiveTask(Some(Task_AnimateWirelessSignal))) != 0) {
                    ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(148)
                        .cast::<u16>())
                    .write(46u16);
                    ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(100)
                        .cast::<u32>())
                    .write(0u32);
                }
                break 'l1;
            }
            if __sw1 == 46i32 {
                if {
                    let __p53 = (((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(100)
                        .cast::<u32>();
                    let __t54 = ((__p53).read()).wrapping_add(1);
                    (__p53).write(__t54);
                    __t54
                } == 10u32
                {
                    let __p55 = (((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(148)
                        .cast::<u16>();
                    (__p55).write(((__p55).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 47i32 {
                if (({
                    let __p56 = (((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(224)
                        .cast::<i16>();
                    let __t57 = ((__p56).read()).wrapping_add(1);
                    (__p56).write(__t57);
                    __t57
                }) as i32)
                    > 348i32
                {
                    ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(224)
                        .cast::<i16>())
                    .write(348i16);
                    let __p58 = (((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(148)
                        .cast::<u16>();
                    (__p58).write(((__p58).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 48i32 {
                ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(145))
                    .write(CreateSprite(
                        (&raw const sSpriteTemplate_GbaScreenFlash_Long)
                            .cast::<u8>()
                            .cast_mut(),
                        120i16,
                        80i16,
                        0u8,
                    ));
                ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(148)
                    .cast::<u16>())
                .write(50u16);
                break 'l1;
            }
            if __sw1 == 50i32 {
                if (crate::c::bf_read(
                    (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(145))
                        .read()) as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(63),
                    4,
                    1,
                    false,
                ) as u16)
                    != 0
                {
                    DestroySprite(
                        ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            ((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(145))
                            .read()) as i32) as isize
                                * 68,
                        ),
                    );
                    SetTradeSequenceBgGpuRegs(6u8);
                    let __p59 = (((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(148)
                        .cast::<u16>();
                    (__p59).write(((__p59).read()).wrapping_add(1));
                    PlaySE(159u16);
                }
                break 'l1;
            }
            if __sw1 == 51i32 {
                if ((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(234)
                    .cast::<u16>())
                .read()) as i32)
                    < 1024i32
                {
                    let __p60 = (((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(234)
                        .cast::<u16>();
                    (__p60).write((((((__p60).read()) as i32).wrapping_add(52i32)) as u16));
                } else {
                    ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(234)
                        .cast::<u16>())
                    .write(1024u16);
                    let __p61 = (((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(148)
                        .cast::<u16>();
                    (__p61).write(((__p61).read()).wrapping_add(1));
                }
                ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(232)
                    .cast::<u16>())
                .write(
                    ((crate::c::div_i32(
                        32768i32,
                        ((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(234)
                            .cast::<u16>())
                        .read()) as i32),
                    )) as u16),
                );
                break 'l1;
            }
            if __sw1 == 52i32 {
                BeginNormalPaletteFade(4294967295u32, 0i8, 0u8, 16u8, 0u16);
                ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(148)
                    .cast::<u16>())
                .write(60u16);
                break 'l1;
            }
            if __sw1 == 60i32 {
                if !((crate::c::bf_read(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
                    7,
                    1,
                    false,
                ) as u16)
                    != 0)
                {
                    SetTradeSequenceBgGpuRegs(5u8);
                    SetTradeSequenceBgGpuRegs(7u8);
                    crate::c::bf_write(
                        ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(8),
                        7,
                        1,
                        (1u16) as i32,
                    );
                    let __p62 = (((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(148)
                        .cast::<u16>();
                    (__p62).write(((__p62).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 61i32 {
                crate::c::bf_write(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(8),
                    7,
                    1,
                    (0u16) as i32,
                );
                BeginNormalPaletteFade(4294967295u32, 0i8, 16u8, 0u8, 0u16);
                let __p63 = (((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(148)
                    .cast::<u16>();
                (__p63).write(((__p63).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 62i32 {
                SetGpuReg(0u8, 5184u16);
                if !((crate::c::bf_read(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
                    7,
                    1,
                    false,
                ) as u16)
                    != 0)
                {
                    let __p64 = (((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(148)
                        .cast::<u16>();
                    (__p64).write(((__p64).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 63i32 {
                ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(211))
                    .write(CreateSprite(
                        (&raw const sSpriteTemplate_Pokeball)
                            .cast::<u8>()
                            .cast_mut(),
                        120i16,
                        (-8i16),
                        0u8,
                    ));
                ((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(211))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(46))
                .cast::<i16>())
                .wrapping_offset(3))
                .write(74i16);
                ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(211))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                .write(Some(SpriteCB_BouncingPokeballArrive));
                StartSpriteAnim(
                    ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(211))
                        .read()) as i32) as isize
                            * 68,
                    ),
                    1u8,
                );
                StartSpriteAffineAnim(
                    ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(211))
                        .read()) as i32) as isize
                            * 68,
                    ),
                    2u8,
                );
                BlendPalettes(
                    ((crate::c::shl_i32(
                        1i32,
                        (((16i32).wrapping_add(
                            ((crate::c::bf_read(
                                (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                    ((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(211))
                                    .read()) as i32) as isize
                                        * 68,
                                ))
                                .wrapping_add(5),
                                4,
                                4,
                                false,
                            ) as u16) as i32),
                        )) as u32),
                    )) as u32),
                    16u8,
                    65535u16,
                );
                let __p65 = (((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(148)
                    .cast::<u16>();
                (__p65).write(((__p65).read()).wrapping_add(1));
                ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(100)
                    .cast::<u32>())
                .write(0u32);
                break 'l1;
            }
            if __sw1 == 64i32 {
                BeginNormalPaletteFade(
                    ((crate::c::shl_i32(
                        1i32,
                        (((16i32).wrapping_add(
                            ((crate::c::bf_read(
                                (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                    ((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(211))
                                    .read()) as i32) as isize
                                        * 68,
                                ))
                                .wrapping_add(5),
                                4,
                                4,
                                false,
                            ) as u16) as i32),
                        )) as u32),
                    )) as u32),
                    1i8,
                    16u8,
                    0u8,
                    65535u16,
                );
                let __p66 = (((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(148)
                    .cast::<u16>();
                (__p66).write(((__p66).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 65i32 {
                if core::mem::transmute::<_, usize>(
                    ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(211))
                        .read()) as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(28)
                    .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                    .read(),
                ) == (SpriteCallbackDummy as *const () as usize)
                {
                    HandleLoadSpecialPokePic_2(
                        ((&raw mut gMonFrontPicTable).cast::<u8>()).wrapping_offset(
                            ((((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(240))
                            .cast::<u16>())
                            .wrapping_offset(1))
                            .read()) as i32) as isize
                                * 8,
                        ),
                        ((((((&raw mut gMonSpritesGfxPtr).cast::<*mut u8>()).read())
                            .wrapping_add(4))
                        .cast::<*mut u8>())
                        .wrapping_offset(3))
                        .read(),
                        ((((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(240))
                        .cast::<u16>())
                        .wrapping_offset(1))
                        .read()) as i32),
                        ((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(104))
                        .cast::<u32>())
                        .wrapping_offset(1))
                        .read(),
                    );
                    let __p67 = (((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(148)
                        .cast::<u16>();
                    (__p67).write(((__p67).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 66i32 {
                ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(142))
                    .cast::<u8>())
                    .wrapping_offset(1))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(32)
                .cast::<i16>())
                .write(120i16);
                ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(142))
                    .cast::<u8>())
                    .wrapping_offset(1))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(34)
                .cast::<i16>())
                .write(
                    ((((((((&raw mut gMonFrontPicCoords).cast::<u8>()).wrapping_offset(
                        ((((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(240))
                        .cast::<u16>())
                        .wrapping_offset(1))
                        .read()) as i32) as isize
                            * 4,
                    ))
                    .wrapping_add(1))
                    .read()) as i32)
                        .wrapping_add(60i32)) as i16),
                );
                ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(142))
                    .cast::<u8>())
                    .wrapping_offset(1))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(36)
                .cast::<i16>())
                .write(0i16);
                ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(142))
                    .cast::<u8>())
                    .wrapping_offset(1))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(38)
                .cast::<i16>())
                .write(0i16);
                StartSpriteAnim(
                    ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(142))
                        .cast::<u8>())
                        .wrapping_offset(1))
                        .read()) as i32) as isize
                            * 68,
                    ),
                    0u8,
                );
                CreatePokeballSpriteToReleaseMon(
                    ((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(142))
                    .cast::<u8>())
                    .wrapping_offset(1))
                    .read(),
                    ((crate::c::bf_read(
                        (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            ((((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(142))
                            .cast::<u8>())
                            .wrapping_offset(1))
                            .read()) as i32) as isize
                                * 68,
                        ))
                        .wrapping_add(5),
                        4,
                        4,
                        false,
                    ) as u16) as u8),
                    120u8,
                    84u8,
                    2u8,
                    1u8,
                    20u8,
                    1048575u32,
                    ((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(240))
                    .cast::<u16>())
                    .wrapping_offset(1))
                    .read(),
                );
                FreeSpriteOamMatrix(
                    ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(211))
                        .read()) as i32) as isize
                            * 68,
                    ),
                );
                DestroySprite(
                    ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(211))
                        .read()) as i32) as isize
                            * 68,
                    ),
                );
                let __p68 = (((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(148)
                    .cast::<u16>();
                (__p68).write(((__p68).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 67i32 {
                SetGpuReg(0u8, 5440u16);
                StringExpandPlaceholders(
                    (&raw mut gStringVar4).cast::<u8>(),
                    (&raw mut gText_XSentOverY).cast::<u8>(),
                );
                DrawTextOnTradeWindow(0u8, (&raw mut gStringVar4).cast::<u8>(), 0u8);
                ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(148)
                    .cast::<u16>())
                .write(167u16);
                ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(100)
                    .cast::<u32>())
                .write(0u32);
                break 'l1;
            }
            if __sw1 == 167i32 {
                if {
                    let __p69 = (((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(100)
                        .cast::<u32>();
                    let __t70 = ((__p69).read()).wrapping_add(1);
                    (__p69).write(__t70);
                    __t70
                } > 60u32
                {
                    ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(148)
                        .cast::<u16>())
                    .write(267u16);
                    ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(100)
                        .cast::<u32>())
                    .write(0u32);
                }
                break 'l1;
            }
            if __sw1 == 267i32 {
                if (IsCryFinished()) != 0 {
                    ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(148)
                        .cast::<u16>())
                    .write(68u16);
                }
                break 'l1;
            }
            if __sw1 == 68i32 {
                if {
                    let __p71 = (((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(100)
                        .cast::<u32>();
                    let __t72 = ((__p71).read()).wrapping_add(1);
                    (__p71).write(__t72);
                    __t72
                } == 10u32
                {
                    PlayFanfare(371u16);
                }
                if ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(100)
                    .cast::<u32>())
                .read()
                    == 250u32
                {
                    let __p73 = (((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(148)
                        .cast::<u16>();
                    (__p73).write(((__p73).read()).wrapping_add(1));
                    StringExpandPlaceholders(
                        (&raw mut gStringVar4).cast::<u8>(),
                        (&raw mut gText_TakeGoodCareOfX).cast::<u8>(),
                    );
                    DrawTextOnTradeWindow(0u8, (&raw mut gStringVar4).cast::<u8>(), 0u8);
                    ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(100)
                        .cast::<u32>())
                    .write(0u32);
                }
                break 'l1;
            }
            if __sw1 == 69i32 {
                if {
                    let __p74 = (((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(100)
                        .cast::<u32>();
                    let __t75 = ((__p74).read()).wrapping_add(1);
                    (__p74).write(__t75);
                    __t75
                } == 60u32
                {
                    let __p76 = (((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(148)
                        .cast::<u16>();
                    (__p76).write(((__p76).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 70i32 {
                CheckPartnersMonForRibbons();
                let __p77 = (((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(148)
                    .cast::<u16>();
                (__p77).write(((__p77).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 71i32 {
                if (((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(238))
                .read())
                    != 0
                {
                    return 1u8;
                } else {
                    if ((((((&raw mut gMain).cast::<u8>())
                        .wrapping_add(46)
                        .cast::<u16>())
                    .read()) as i32)
                        & 1i32)
                        != 0
                    {
                        let __p78 = (((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(148)
                            .cast::<u16>();
                        (__p78).write(((__p78).read()).wrapping_add(1));
                    }
                }
                break 'l1;
            }
            if __sw1 == 72i32 {
                TradeMons(
                    ((((&raw mut gSpecialVar_0x8005).cast::<u16>()).read()) as u8),
                    0u8,
                );
                ((&raw mut gCB2_AfterEvolution).cast::<Option<unsafe extern "C" fn()>>())
                    .write(Some(CB2_InGameTrade));
                evoTarget = GetEvolutionTargetSpecies(
                    ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                        (((((&raw mut gSelectedTradeMonPositions).cast::<u8>()).cast::<u8>())
                            .read()) as i32) as isize
                            * 100,
                    ),
                    1u8,
                    0u16,
                );
                if ((evoTarget) as i32) != 0i32 {
                    TradeEvolutionScene(
                        ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                            (((((&raw mut gSelectedTradeMonPositions).cast::<u8>()).cast::<u8>())
                                .read()) as i32) as isize
                                * 100,
                        ),
                        evoTarget,
                        ((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(142))
                        .cast::<u8>())
                        .wrapping_offset(1))
                        .read(),
                        (((&raw mut gSelectedTradeMonPositions).cast::<u8>()).cast::<u8>()).read(),
                    );
                }
                let __p79 = (((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(148)
                    .cast::<u16>();
                (__p79).write(((__p79).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 73i32 {
                BeginNormalPaletteFade(4294967295u32, 0i8, 0u8, 16u8, 0u16);
                let __p80 = (((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(148)
                    .cast::<u16>();
                (__p80).write(((__p80).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 74i32 {
                if !((crate::c::bf_read(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
                    7,
                    1,
                    false,
                ) as u16)
                    != 0)
                {
                    PlayNewMapMusic(
                        ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(244)
                            .cast::<u16>())
                        .read(),
                    );
                    if !(((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read()).is_null() {
                        FreeAllWindowBuffers();
                        Free(GetBgTilemapBuffer(3u8));
                        Free(GetBgTilemapBuffer(1u8));
                        Free(GetBgTilemapBuffer(0u8));
                        FreeMonSpritesGfx();
                        {
                            Free(((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read());
                            ((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>())
                                .write(core::ptr::null_mut());
                        }
                    }
                    SetMainCallback2(Some(CB2_ReturnToField));
                    BufferInGameTradeMonName();
                }
                break 'l1;
            }
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn CB2_TryLinkTradeEvolution() {
    unsafe {
        let mut evoTarget: u16 = 0u16;
        'l1: {
            let __sw1 = (((((&raw mut gMain).cast::<u8>()).wrapping_add(1080)).read()) as i32);
            if __sw1 == 0i32 {
                (((&raw mut gMain).cast::<u8>()).wrapping_add(1080)).write(4u8);
                ((&raw mut gSoftResetDisabled).cast::<u8>()).write(1u8);
                break 'l1;
            }
            if __sw1 == 4i32 {
                ((&raw mut gCB2_AfterEvolution).cast::<Option<unsafe extern "C" fn()>>())
                    .write(Some(CB2_SaveAndEndTrade));
                evoTarget = GetEvolutionTargetSpecies(
                    ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                        (((((&raw mut gSelectedTradeMonPositions).cast::<u8>()).cast::<u8>())
                            .read()) as i32) as isize
                            * 100,
                    ),
                    1u8,
                    0u16,
                );
                if ((evoTarget) as i32) != 0i32 {
                    TradeEvolutionScene(
                        ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                            (((((&raw mut gSelectedTradeMonPositions).cast::<u8>()).cast::<u8>())
                                .read()) as i32) as isize
                                * 100,
                        ),
                        evoTarget,
                        ((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(142))
                        .cast::<u8>())
                        .wrapping_offset(1))
                        .read(),
                        (((&raw mut gSelectedTradeMonPositions).cast::<u8>()).cast::<u8>()).read(),
                    );
                } else {
                    if (IsWirelessTrade()) != 0 {
                        SetMainCallback2(Some(CB2_SaveAndEndWirelessTrade));
                    } else {
                        SetMainCallback2(Some(CB2_SaveAndEndTrade));
                    }
                }
                (((&raw mut gSelectedTradeMonPositions).cast::<u8>()).cast::<u8>()).write(255u8);
                break 'l1;
            }
        }
        if !((HasLinkErrorOccurred()) != 0) {
            RunTasks();
        }
        AnimateSprites();
        BuildOamBuffer();
        UpdatePaletteFade();
    }
}
pub(crate) unsafe extern "C" fn HandleLinkDataReceive() {
    unsafe {
        let mut recvStatus: u8 = 0u8;
        TradeGetMultiplayerId();
        recvStatus = GetBlockReceivedStatus();
        if (((recvStatus) as i32) & 1i32) != 0 {
            if (((((&raw mut gBlockRecvBuffer).cast::<u8>()).cast::<u16>()).read()) as i32)
                == 56506i32
            {
                SetMainCallback2(Some(CB2_TryLinkTradeEvolution));
            }
            if (((((&raw mut gBlockRecvBuffer).cast::<u8>()).cast::<u16>()).read()) as i32)
                == 43981i32
            {
                ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(114))
                    .write(1u8);
            }
            ResetBlockReceivedFlag(0u8);
        }
        if (((recvStatus) as i32) & 2i32) != 0 {
            if ((((((&raw mut gBlockRecvBuffer).cast::<u8>()).wrapping_offset(256)).cast::<u16>())
                .read()) as i32)
                == 43981i32
            {
                ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(115))
                    .write(1u8);
            }
            ResetBlockReceivedFlag(1u8);
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_BouncingPokeball(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let __p1 = (sprite).wrapping_add(34).cast::<i16>();
        (__p1).write(
            (((((__p1).read()) as i32).wrapping_add(crate::c::div_i32(
                (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32),
                10i32,
            ))) as i16),
        );
        let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5);
        (__p2).write(
            (((((__p2).read()) as i32).wrapping_add(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32),
            )) as i16),
        );
        ((sprite).wrapping_add(32).cast::<i16>()).write(
            ((crate::c::div_i32(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read()) as i32),
                10i32,
            )) as i16),
        );
        if ((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32) > 76i32 {
            ((sprite).wrapping_add(34).cast::<i16>()).write(76i16);
            (((sprite).wrapping_add(46)).cast::<i16>()).write(
                ((crate::c::div_i32(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32).wrapping_mul(
                        ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                            as i32),
                    ))
                    .wrapping_neg(),
                    100i32,
                )) as i16),
            );
            let __p3 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3);
            (__p3).write(((__p3).read()).wrapping_add(1));
        }
        if ((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32) == 120i32 {
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(0i16);
        }
        let __p4 = ((sprite).wrapping_add(46)).cast::<i16>();
        (__p4).write(
            (((((__p4).read()) as i32).wrapping_add(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read()) as i32),
            )) as i16),
        );
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read()) as i32)
            == 4i32
        {
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(1i16);
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(SpriteCallbackDummy));
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_BouncingPokeballDepart(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let __p1 = (sprite).wrapping_add(38).cast::<i16>();
        (__p1).write(
            (((((__p1).read()) as i32).wrapping_add(
                ((((((&raw const sTradeBallVerticalVelocityTable)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<i8>())
                .cast::<i8>())
                .wrapping_offset(
                    (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) as isize,
                ))
                .read()) as i32),
            )) as i16),
        );
        if (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) == 22i32 {
            PlaySE(56u16);
        }
        if (({
            let __p2 = ((sprite).wrapping_add(46)).cast::<i16>();
            let __t3 = ((__p2).read()).wrapping_add(1);
            (__p2).write(__t3);
            __t3
        }) as i32)
            == 44i32
        {
            PlaySE(140u16);
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(SpriteCB_BouncingPokeballDepartEnd));
            (((sprite).wrapping_add(46)).cast::<i16>()).write(0i16);
            BeginNormalPaletteFade(
                ((crate::c::shl_i32(
                    1i32,
                    (((16i32).wrapping_add(
                        ((crate::c::bf_read((sprite).wrapping_add(5), 4, 4, false) as u16) as i32),
                    )) as u32),
                )) as u32),
                (-1i8),
                0u8,
                16u8,
                65535u16,
            );
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_BouncingPokeballDepartEnd(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
            == 20i32
        {
            StartSpriteAffineAnim(sprite, 1u8);
        }
        if (({
            let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
            let __t2 = ((__p1).read()).wrapping_add(1);
            (__p1).write(__t2);
            __t2
        }) as i32)
            > 20i32
        {
            let __p3 = (sprite).wrapping_add(38).cast::<i16>();
            (__p3).write(
                (((((__p3).read()) as i32).wrapping_sub(
                    ((((((&raw const sTradeBallVerticalVelocityTable)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<i8>())
                    .cast::<i8>())
                    .wrapping_offset(
                        (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) as isize,
                    ))
                    .read()) as i32),
                )) as i16),
            );
            if (({
                let __p4 = ((sprite).wrapping_add(46)).cast::<i16>();
                let __t5 = ((__p4).read()).wrapping_add(1);
                (__p4).write(__t5);
                __t5
            }) as i32)
                == 23i32
            {
                DestroySprite(sprite);
                ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(148)
                    .cast::<u16>())
                .write(14u16);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_BouncingPokeballArrive(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
            == 0i32
        {
            if (({
                let __p1 = (sprite).wrapping_add(34).cast::<i16>();
                let __v2 = (((((__p1).read()) as i32).wrapping_add(4i32)) as i16);
                (__p1).write(__v2);
                __v2
            }) as i32)
                > ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read()) as i32)
            {
                let __p3 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
                (__p3).write(((__p3).read()).wrapping_add(1));
                (((sprite).wrapping_add(46)).cast::<i16>()).write(22i16);
                PlaySE(56u16);
            }
        } else {
            if (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) == 66i32 {
                PlaySE(57u16);
            }
            if (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) == 92i32 {
                PlaySE(58u16);
            }
            if (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) == 107i32 {
                PlaySE(59u16);
            }
            let __p4 = (sprite).wrapping_add(38).cast::<i16>();
            (__p4).write(
                (((((__p4).read()) as i32).wrapping_add(
                    ((((((&raw const sTradeBallVerticalVelocityTable)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<i8>())
                    .cast::<i8>())
                    .wrapping_offset(
                        (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) as isize,
                    ))
                    .read()) as i32),
                )) as i16),
            );
            if (({
                let __p5 = ((sprite).wrapping_add(46)).cast::<i16>();
                let __t6 = ((__p5).read()).wrapping_add(1);
                (__p5).write(__t6);
                __t6
            }) as i32)
                == 108i32
            {
                ((sprite)
                    .wrapping_add(28)
                    .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                .write(Some(SpriteCallbackDummy));
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetInGameTradeSpeciesInfo() -> u16 {
    unsafe {
        let mut inGameTrade: *mut u8 =
            (((&raw const sIngameTrades).cast::<u8>().cast_mut()).cast::<u8>()).wrapping_offset(
                ((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32) as isize * 60,
            );
        StringCopy(
            (&raw mut gStringVar1).cast::<u8>(),
            (((&raw mut gSpeciesNames).cast::<u8>()).wrapping_offset(
                ((((inGameTrade).wrapping_add(56).cast::<u16>()).read()) as i32) as isize * 11,
            ))
            .cast::<u8>(),
        );
        StringCopy(
            (&raw mut gStringVar2).cast::<u8>(),
            (((&raw mut gSpeciesNames).cast::<u8>()).wrapping_offset(
                ((((inGameTrade).wrapping_add(12).cast::<u16>()).read()) as i32) as isize * 11,
            ))
            .cast::<u8>(),
        );
        return ((inGameTrade).wrapping_add(56).cast::<u16>()).read();
    }
}
pub(crate) unsafe extern "C" fn BufferInGameTradeMonName() {
    unsafe {
        let mut nickname = crate::ffi::Align4([0u8; 32]);
        let mut inGameTrade: *mut u8 =
            (((&raw const sIngameTrades).cast::<u8>().cast_mut()).cast::<u8>()).wrapping_offset(
                ((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32) as isize * 60,
            );
        GetMonData3(
            ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                ((((&raw mut gSpecialVar_0x8005).cast::<u16>()).read()) as i32) as isize * 100,
            ),
            2i32,
            (&raw mut nickname).cast::<u8>(),
        );
        StringCopy_Nickname(
            (&raw mut gStringVar1).cast::<u8>(),
            (&raw mut nickname).cast::<u8>(),
        );
        StringCopy(
            (&raw mut gStringVar2).cast::<u8>(),
            (((&raw mut gSpeciesNames).cast::<u8>()).wrapping_offset(
                ((((inGameTrade).wrapping_add(12).cast::<u16>()).read()) as i32) as isize * 11,
            ))
            .cast::<u8>(),
        );
    }
}
pub(crate) unsafe extern "C" fn CreateInGameTradePokemonInternal(
    whichPlayerMon: u8,
    whichInGameTrade: u8,
) {
    unsafe {
        let mut whichPlayerMon = whichPlayerMon;
        let mut whichInGameTrade = whichInGameTrade;
        let mut inGameTrade: *mut u8 = (((&raw const sIngameTrades).cast::<u8>().cast_mut())
            .cast::<u8>())
        .wrapping_offset(((whichInGameTrade) as i32) as isize * 60);
        let mut level: u8 = ((GetMonData2(
            ((&raw mut gPlayerParty).cast::<u8>())
                .wrapping_offset(((whichPlayerMon) as i32) as isize * 100),
            56i32,
        )) as u8);
        let mut mail = crate::ffi::Align4([0u8; 36]);
        let mut metLocation: u8 = 254u8;
        let mut mailNum: u8 = 0u8;
        let mut pokemon: *mut u8 = (&raw mut gEnemyParty).cast::<u8>();
        CreateMon(
            pokemon,
            ((inGameTrade).wrapping_add(12).cast::<u16>()).read(),
            level,
            32u8,
            1u8,
            ((inGameTrade).wrapping_add(36).cast::<u32>()).read(),
            1u8,
            ((inGameTrade).wrapping_add(24).cast::<u32>()).read(),
        );
        SetMonData(
            pokemon,
            39i32,
            ((inGameTrade).wrapping_add(14)).cast::<u8>(),
        );
        SetMonData(
            pokemon,
            40i32,
            (((inGameTrade).wrapping_add(14)).cast::<u8>()).wrapping_offset(1),
        );
        SetMonData(
            pokemon,
            41i32,
            (((inGameTrade).wrapping_add(14)).cast::<u8>()).wrapping_offset(2),
        );
        SetMonData(
            pokemon,
            42i32,
            (((inGameTrade).wrapping_add(14)).cast::<u8>()).wrapping_offset(3),
        );
        SetMonData(
            pokemon,
            43i32,
            (((inGameTrade).wrapping_add(14)).cast::<u8>()).wrapping_offset(4),
        );
        SetMonData(
            pokemon,
            44i32,
            (((inGameTrade).wrapping_add(14)).cast::<u8>()).wrapping_offset(5),
        );
        SetMonData(pokemon, 2i32, (inGameTrade).cast::<u8>());
        SetMonData(pokemon, 7i32, ((inGameTrade).wrapping_add(43)).cast::<u8>());
        SetMonData(pokemon, 49i32, (inGameTrade).wrapping_add(54));
        SetMonData(pokemon, 46i32, (inGameTrade).wrapping_add(20));
        SetMonData(
            pokemon,
            23i32,
            (((inGameTrade).wrapping_add(28)).cast::<u8>()).wrapping_offset(1),
        );
        SetMonData(
            pokemon,
            24i32,
            (((inGameTrade).wrapping_add(28)).cast::<u8>()).wrapping_offset(2),
        );
        SetMonData(
            pokemon,
            22i32,
            ((inGameTrade).wrapping_add(28)).cast::<u8>(),
        );
        SetMonData(
            pokemon,
            33i32,
            (((inGameTrade).wrapping_add(28)).cast::<u8>()).wrapping_offset(3),
        );
        SetMonData(
            pokemon,
            47i32,
            (((inGameTrade).wrapping_add(28)).cast::<u8>()).wrapping_offset(4),
        );
        SetMonData(pokemon, 48i32, (inGameTrade).wrapping_add(55));
        SetMonData(pokemon, 35i32, &raw mut metLocation);
        mailNum = 0u8;
        if ((((inGameTrade).wrapping_add(40).cast::<u16>()).read()) as i32) != 0i32 {
            if (ItemIsMail(((inGameTrade).wrapping_add(40).cast::<u16>()).read())) != 0 {
                GetInGameTradeMail((&raw mut mail).cast::<u8>(), inGameTrade);
                ((&raw mut gTradeMail).cast::<u8>())
                    .cast::<u8>()
                    .cast::<crate::c::Rec4<36>>()
                    .write_unaligned(
                        (&raw mut mail)
                            .cast::<u8>()
                            .cast::<crate::c::Rec4<36>>()
                            .read_unaligned(),
                    );
                SetMonData(pokemon, 64i32, &raw mut mailNum);
                SetMonData(
                    pokemon,
                    12i32,
                    ((inGameTrade).wrapping_add(40).cast::<u16>()).cast::<u8>(),
                );
            } else {
                SetMonData(
                    pokemon,
                    12i32,
                    ((inGameTrade).wrapping_add(40).cast::<u16>()).cast::<u8>(),
                );
            }
        }
        CalculateMonStats((&raw mut gEnemyParty).cast::<u8>());
    }
}
pub(crate) unsafe extern "C" fn GetInGameTradeMail(mail: *mut u8, trade: *mut u8) {
    unsafe {
        let mut mail = mail;
        let mut trade = trade;
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 9i32) {
                    break 'l1;
                }
                'l2: {
                    (((mail).cast::<u16>()).wrapping_offset((i) as isize)).write(
                        ((((((&raw const sIngameTradeMail).cast::<u8>().cast_mut())
                            .cast::<u8>())
                        .wrapping_offset(
                            ((((trade).wrapping_add(42)).read()) as i32) as isize * 20,
                        ))
                        .cast::<u16>())
                        .wrapping_offset((i) as isize))
                        .read(),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        StringCopy(
            ((mail).wrapping_add(18)).cast::<u8>(),
            ((trade).wrapping_add(43)).cast::<u8>(),
        );
        PadNameString(((mail).wrapping_add(18)).cast::<u8>(), 0u8);
        (((mail).wrapping_add(26)).cast::<u8>())
            .write(((((trade).wrapping_add(24).cast::<u32>()).read() >> 24) as u8));
        ((((mail).wrapping_add(26)).cast::<u8>()).wrapping_offset(1))
            .write(((((trade).wrapping_add(24).cast::<u32>()).read() >> 16) as u8));
        ((((mail).wrapping_add(26)).cast::<u8>()).wrapping_offset(2))
            .write(((((trade).wrapping_add(24).cast::<u32>()).read() >> 8) as u8));
        ((((mail).wrapping_add(26)).cast::<u8>()).wrapping_offset(3))
            .write(((((trade).wrapping_add(24).cast::<u32>()).read()) as u8));
        ((mail).wrapping_add(30).cast::<u16>())
            .write(((trade).wrapping_add(12).cast::<u16>()).read());
        ((mail).wrapping_add(32).cast::<u16>())
            .write(((trade).wrapping_add(40).cast::<u16>()).read());
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetTradeSpecies() -> u16 {
    unsafe {
        if (GetMonData2(
            ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                ((((&raw mut gSpecialVar_0x8005).cast::<u16>()).read()) as i32) as isize * 100,
            ),
            45i32,
        )) != 0
        {
            return 0u16;
        }
        return ((GetMonData2(
            ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                ((((&raw mut gSpecialVar_0x8005).cast::<u16>()).read()) as i32) as isize * 100,
            ),
            11i32,
        )) as u16);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CreateInGameTradePokemon() {
    unsafe {
        CreateInGameTradePokemonInternal(
            ((((&raw mut gSpecialVar_0x8005).cast::<u16>()).read()) as u8),
            ((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as u8),
        );
    }
}
pub(crate) unsafe extern "C" fn CB2_UpdateLinkTrade() {
    unsafe {
        if ((DoTradeAnim()) as i32) == 1i32 {
            DestroySprite(
                ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    (((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(142))
                    .cast::<u8>())
                    .read()) as i32) as isize
                        * 68,
                ),
            );
            FreeSpriteOamMatrix(
                ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(142))
                    .cast::<u8>())
                    .wrapping_offset(1))
                    .read()) as i32) as isize
                        * 68,
                ),
            );
            TradeMons(
                (((&raw mut gSelectedTradeMonPositions).cast::<u8>()).cast::<u8>()).read(),
                ((crate::c::rem_i32(
                    ((((((&raw mut gSelectedTradeMonPositions).cast::<u8>()).cast::<u8>())
                        .wrapping_offset(1))
                    .read()) as i32),
                    6i32,
                )) as u8),
            );
            if !((IsWirelessTrade()) != 0) {
                (((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(116))
                .cast::<u16>())
                .write(43981u16);
                ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(147))
                    .write(1u8);
            }
            SetMainCallback2(Some(CB2_WaitTradeComplete));
        }
        HandleLinkDataSend();
        HandleLinkDataReceive();
        RunTasks();
        RunTextPrinters();
        AnimateSprites();
        BuildOamBuffer();
        UpdatePaletteFade();
    }
}
pub(crate) unsafe extern "C" fn CB2_WaitTradeComplete() {
    unsafe {
        let mut mpId: u8 = ((TradeGetMultiplayerId()) as u8);
        if (IsWirelessTrade()) != 0 {
            SetMainCallback2(Some(CB2_TryLinkTradeEvolution));
        } else {
            HandleLinkDataReceive();
            if ((((mpId) as i32) == 0i32)
                && (((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(114))
                .read()) as i32)
                    == 1i32))
                && (((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(115))
                .read()) as i32)
                    == 1i32)
            {
                (((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(116))
                .cast::<u16>())
                .write(56506u16);
                SendBlock(
                    BitmaskAllOtherLinkPlayers(),
                    (((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(116))
                    .cast::<u16>())
                    .cast::<u8>(),
                    20u16,
                );
                ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(114))
                    .write(2u8);
                ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(115))
                    .write(2u8);
            }
        }
        RunTasks();
        AnimateSprites();
        BuildOamBuffer();
        UpdatePaletteFade();
    }
}
pub(crate) unsafe extern "C" fn CB2_SaveAndEndTrade() {
    unsafe {
        'l1: {
            let __sw1 = (((((&raw mut gMain).cast::<u8>()).wrapping_add(1080)).read()) as i32);
            if __sw1 == 0i32 {
                let __p2 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p2).write(((__p2).read()).wrapping_add(1));
                StringExpandPlaceholders(
                    (&raw mut gStringVar4).cast::<u8>(),
                    (&raw mut gText_CommunicationStandby5).cast::<u8>(),
                );
                DrawTextOnTradeWindow(0u8, (&raw mut gStringVar4).cast::<u8>(), 0u8);
                break 'l1;
            }
            if __sw1 == 1i32 {
                SetTradeLinkStandbyCallback(0u8);
                (((&raw mut gMain).cast::<u8>()).wrapping_add(1080)).write(100u8);
                ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(100)
                    .cast::<u32>())
                .write(0u32);
                break 'l1;
            }
            if __sw1 == 100i32 {
                if {
                    let __p3 = (((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(100)
                        .cast::<u32>();
                    let __t4 = ((__p3).read()).wrapping_add(1);
                    (__p3).write(__t4);
                    __t4
                } > 180u32
                {
                    (((&raw mut gMain).cast::<u8>()).wrapping_add(1080)).write(101u8);
                    ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(100)
                        .cast::<u32>())
                    .write(0u32);
                }
                if (_IsLinkTaskFinished()) != 0 {
                    (((&raw mut gMain).cast::<u8>()).wrapping_add(1080)).write(2u8);
                }
                break 'l1;
            }
            if __sw1 == 101i32 {
                if (_IsLinkTaskFinished()) != 0 {
                    (((&raw mut gMain).cast::<u8>()).wrapping_add(1080)).write(2u8);
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                (((&raw mut gMain).cast::<u8>()).wrapping_add(1080)).write(50u8);
                StringExpandPlaceholders(
                    (&raw mut gStringVar4).cast::<u8>(),
                    (&raw mut gText_SavingDontTurnOffPower).cast::<u8>(),
                );
                DrawTextOnTradeWindow(0u8, (&raw mut gStringVar4).cast::<u8>(), 0u8);
                break 'l1;
            }
            if __sw1 == 50i32 {
                if !((InUnionRoom()) != 0) {
                    IncrementGameStat(21u8);
                }
                if (((&raw mut gWirelessCommType).cast::<u8>()).read()) != 0 {
                    MysteryGift_TryIncrementStat(
                        2u32,
                        ((((&raw mut gLinkPlayers).cast::<u8>()).wrapping_offset(
                            (((GetMultiplayerId()) as i32) ^ 1i32) as isize * 28,
                        ))
                        .wrapping_add(4)
                        .cast::<u32>())
                        .read(),
                    );
                }
                SetContinueGameWarpStatusToDynamicWarp();
                LinkFullSave_Init();
                let __p5 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p5).write(((__p5).read()).wrapping_add(1));
                ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(100)
                    .cast::<u32>())
                .write(0u32);
                break 'l1;
            }
            if __sw1 == 51i32 {
                if {
                    let __p6 = (((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(100)
                        .cast::<u32>();
                    let __t7 = ((__p6).read()).wrapping_add(1);
                    (__p6).write(__t7);
                    __t7
                } == 5u32
                {
                    let __p8 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                    (__p8).write(((__p8).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 52i32 {
                if (LinkFullSave_WriteSector()) != 0 {
                    ClearContinueGameWarpStatus2();
                    (((&raw mut gMain).cast::<u8>()).wrapping_add(1080)).write(4u8);
                } else {
                    ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(100)
                        .cast::<u32>())
                    .write(0u32);
                    (((&raw mut gMain).cast::<u8>()).wrapping_add(1080)).write(51u8);
                }
                break 'l1;
            }
            if __sw1 == 4i32 {
                LinkFullSave_ReplaceLastSector();
                (((&raw mut gMain).cast::<u8>()).wrapping_add(1080)).write(40u8);
                ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(100)
                    .cast::<u32>())
                .write(0u32);
                break 'l1;
            }
            if __sw1 == 40i32 {
                if {
                    let __p9 = (((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(100)
                        .cast::<u32>();
                    let __t10 = ((__p9).read()).wrapping_add(1);
                    (__p9).write(__t10);
                    __t10
                } > 50u32
                {
                    if ((GetMultiplayerId()) as i32) == 0i32 {
                        ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(100)
                            .cast::<u32>())
                        .write(((crate::c::rem_i32(((Random()) as i32), 30i32)) as u32));
                    } else {
                        ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(100)
                            .cast::<u32>())
                        .write(0u32);
                    }
                    (((&raw mut gMain).cast::<u8>()).wrapping_add(1080)).write(41u8);
                }
                break 'l1;
            }
            if __sw1 == 41i32 {
                if ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(100)
                    .cast::<u32>())
                .read()
                    == 0u32
                {
                    SetTradeLinkStandbyCallback(1u8);
                    (((&raw mut gMain).cast::<u8>()).wrapping_add(1080)).write(42u8);
                } else {
                    let __p11 = (((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(100)
                        .cast::<u32>();
                    (__p11).write(((__p11).read()).wrapping_sub(1));
                }
                break 'l1;
            }
            if __sw1 == 42i32 {
                if (_IsLinkTaskFinished()) != 0 {
                    LinkFullSave_SetLastSectorSignature();
                    (((&raw mut gMain).cast::<u8>()).wrapping_add(1080)).write(5u8);
                }
                break 'l1;
            }
            if __sw1 == 5i32 {
                if {
                    let __p12 = (((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(100)
                        .cast::<u32>();
                    let __t13 = ((__p12).read()).wrapping_add(1);
                    (__p12).write(__t13);
                    __t13
                } > 60u32
                {
                    let __p14 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                    (__p14).write(((__p14).read()).wrapping_add(1));
                    SetTradeLinkStandbyCallback(2u8);
                }
                break 'l1;
            }
            if __sw1 == 6i32 {
                if (_IsLinkTaskFinished()) != 0 {
                    BeginNormalPaletteFade(4294967295u32, 0i8, 0u8, 16u8, 0u16);
                    let __p15 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                    (__p15).write(((__p15).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 7i32 {
                if !((crate::c::bf_read(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
                    7,
                    1,
                    false,
                ) as u16)
                    != 0)
                {
                    FadeOutBGM(3u8);
                    let __p16 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                    (__p16).write(((__p16).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 8i32 {
                if ((IsBGMStopped()) as i32) == 1i32 {
                    if ((((&raw mut gWirelessCommType).cast::<u8>()).read()) != 0)
                        && (core::mem::transmute::<_, usize>(
                            (((&raw mut gMain).cast::<u8>())
                                .wrapping_add(8)
                                .cast::<Option<unsafe extern "C" fn()>>())
                            .read(),
                        ) == (CB2_StartCreateTradeMenu as *const () as usize))
                    {
                        SetTradeLinkStandbyCallback(3u8);
                    } else {
                        SetCloseLinkCallback();
                    }
                    let __p17 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                    (__p17).write(((__p17).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 9i32 {
                if ((((&raw mut gWirelessCommType).cast::<u8>()).read()) != 0)
                    && (core::mem::transmute::<_, usize>(
                        (((&raw mut gMain).cast::<u8>())
                            .wrapping_add(8)
                            .cast::<Option<unsafe extern "C" fn()>>())
                        .read(),
                    ) == (CB2_StartCreateTradeMenu as *const () as usize))
                {
                    if (_IsLinkTaskFinished()) != 0 {
                        ((&raw mut gSoftResetDisabled).cast::<u8>()).write(0u8);
                        SetMainCallback2(Some(CB2_FreeTradeAnim));
                    }
                } else {
                    if !((((&raw mut gReceivedRemoteLinkPlayers).cast::<u8>()).read()) != 0) {
                        ((&raw mut gSoftResetDisabled).cast::<u8>()).write(0u8);
                        SetMainCallback2(Some(CB2_FreeTradeAnim));
                    }
                }
                break 'l1;
            }
        }
        if !((HasLinkErrorOccurred()) != 0) {
            RunTasks();
        }
        AnimateSprites();
        BuildOamBuffer();
        UpdatePaletteFade();
    }
}
pub(crate) unsafe extern "C" fn CB2_FreeTradeAnim() {
    unsafe {
        if !((crate::c::bf_read(
            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
            7,
            1,
            false,
        ) as u16)
            != 0)
        {
            FreeAllWindowBuffers();
            Free(GetBgTilemapBuffer(3u8));
            Free(GetBgTilemapBuffer(1u8));
            Free(GetBgTilemapBuffer(0u8));
            FreeMonSpritesGfx();
            {
                Free(((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read());
                ((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).write(core::ptr::null_mut());
            }
            if (((&raw mut gWirelessCommType).cast::<u8>()).read()) != 0 {
                DestroyWirelessStatusIndicatorSprite();
            }
            SetMainCallback2(
                (((&raw mut gMain).cast::<u8>())
                    .wrapping_add(8)
                    .cast::<Option<unsafe extern "C" fn()>>())
                .read(),
            );
        }
        RunTasks();
        AnimateSprites();
        BuildOamBuffer();
        UpdatePaletteFade();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DoInGameTradeScene() {
    unsafe {
        LockPlayerFieldControls();
        CreateTask(Some(Task_InGameTrade), 10u8);
        BeginNormalPaletteFade(4294967295u32, 0i8, 0u8, 16u8, 0u16);
    }
}
pub(crate) unsafe extern "C" fn Task_InGameTrade(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if !((crate::c::bf_read(
            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
            7,
            1,
            false,
        ) as u16)
            != 0)
        {
            SetMainCallback2(Some(CB2_InitInGameTrade));
            ((&raw mut gFieldCallback).cast::<Option<unsafe extern "C" fn()>>())
                .write(Some(FieldCB_ContinueScriptHandleMusic));
            DestroyTask(taskId);
        }
    }
}
pub(crate) unsafe extern "C" fn CheckPartnersMonForRibbons() {
    unsafe {
        let mut i: u8 = 0u8;
        let mut numRibbons: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 12i32) {
                    break 'l1;
                }
                'l2: {
                    numRibbons = ((((numRibbons) as u32).wrapping_add(GetMonData2(
                        ((&raw mut gEnemyParty).cast::<u8>()).wrapping_offset(
                            (crate::c::rem_i32(
                                ((((((&raw mut gSelectedTradeMonPositions).cast::<u8>())
                                    .cast::<u8>())
                                .wrapping_offset(1))
                                .read()) as i32),
                                6i32,
                            )) as isize
                                * 100,
                        ),
                        (67i32).wrapping_add(((i) as i32)),
                    ))) as u8);
                }
                i = (i).wrapping_add(1);
            }
        }
        if ((numRibbons) as i32) != 0i32 {
            FlagSet(2203u16);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn LoadTradeAnimGfx() {
    unsafe {
        TradeAnimInit_LoadGfx();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DrawTextOnTradeWindow(windowId: u8, str: *mut u8, speed: u8) {
    unsafe {
        let mut windowId = windowId;
        let mut str = str;
        let mut speed = speed;
        FillWindowPixelBuffer(windowId, 255u8);
        (((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(246))
            .cast::<u8>())
        .write(15u8);
        ((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(246))
            .cast::<u8>())
        .wrapping_offset(1))
        .write(1u8);
        ((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(246))
            .cast::<u8>())
        .wrapping_offset(2))
        .write(6u8);
        AddTextPrinterParameterized4(
            windowId,
            1u8,
            0u8,
            2u8,
            0u8,
            0u8,
            ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(246))
                .cast::<u8>(),
            ((speed) as i8),
            str,
        );
        CopyWindowToVram(windowId, 3u8);
    }
}
pub(crate) unsafe extern "C" fn Task_AnimateWirelessSignal(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        let mut paletteIdx: u16 = (((((((((&raw const sWirelessSignalAnimParams)
            .cast::<u8>()
            .cast_mut())
        .cast::<u8>())
        .wrapping_offset((((data).read()) as i32) as isize * 2))
        .cast::<u8>())
        .read()) as i32)
            .wrapping_mul(16i32)) as u16);
        if !((((data).wrapping_offset(2)).read()) != 0) {
            if ((paletteIdx) as i32) == 256i32 {
                LoadPalette(
                    (((&raw const sWirelessSignalNone_Pal)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<u16>())
                    .cast::<u16>())
                    .cast::<u8>(),
                    48u16,
                    32u16,
                );
            } else {
                LoadPalette(
                    ((((&raw const sWirelessSignalSend_Pal)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<u16>())
                    .cast::<u16>())
                    .wrapping_offset(((paletteIdx) as i32) as isize))
                    .cast::<u8>(),
                    48u16,
                    32u16,
                );
            }
        } else {
            if ((paletteIdx) as i32) == 256i32 {
                LoadPalette(
                    (((&raw const sWirelessSignalNone_Pal)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<u16>())
                    .cast::<u16>())
                    .cast::<u8>(),
                    48u16,
                    32u16,
                );
            } else {
                LoadPalette(
                    ((((&raw const sWirelessSignalRecv_Pal)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<u16>())
                    .cast::<u16>())
                    .wrapping_offset(((paletteIdx) as i32) as isize))
                    .cast::<u8>(),
                    48u16,
                    32u16,
                );
            }
        }
        if ((((((((&raw const sWirelessSignalAnimParams)
            .cast::<u8>()
            .cast_mut())
        .cast::<u8>())
        .wrapping_offset((((data).read()) as i32) as isize * 2))
        .cast::<u8>())
        .read()) as i32)
            == 0i32)
            && (((((data).wrapping_offset(1)).read()) as i32) == 0i32)
        {
            PlaySE(195u16);
        }
        if ((((data).wrapping_offset(1)).read()) as i32)
            == ((((((((&raw const sWirelessSignalAnimParams)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>())
            .wrapping_offset((((data).read()) as i32) as isize * 2))
            .cast::<u8>())
            .wrapping_offset(1))
            .read()) as i32)
        {
            (data).write(((data).read()).wrapping_add(1));
            ((data).wrapping_offset(1)).write(0i16);
            if ((((((((&raw const sWirelessSignalAnimParams)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>())
            .wrapping_offset((((data).read()) as i32) as isize * 2))
            .cast::<u8>())
            .wrapping_offset(1))
            .read()) as i32)
                == 255i32
            {
                DestroyTask(taskId);
            }
        } else {
            let __p1 = (data).wrapping_offset(1);
            (__p1).write(((__p1).read()).wrapping_add(1));
        }
    }
}
pub(crate) unsafe extern "C" fn Task_OpenCenterWhiteColumn(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        if (((data).read()) as i32) == 0i32 {
            ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(251))
                .write({
                    let __v1 = ((crate::c::div_i32(240i32, 2i32)) as u8);
                    ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(253))
                    .write(__v1);
                    __v1
                });
            ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(252))
                .write(0u8);
            ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(254))
                .write(160u8);
            SetGpuRegBits(0u8, 8192u16);
            SetGpuReg(74u8, 16u16);
            SetGpuReg(72u8, 19u16);
        }
        SetGpuReg(
            64u8,
            ((((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(253))
                .read()) as i32)
                | (((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(251))
                .read()) as i32)
                    << 8)) as u16),
        );
        SetGpuReg(
            68u8,
            ((((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(254))
                .read()) as i32)
                | (((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(252))
                .read()) as i32)
                    << 8)) as u16),
        );
        (data).write(((data).read()).wrapping_add(1));
        let __p2 =
            (((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(251);
        (__p2).write((((((__p2).read()) as i32).wrapping_sub(5i32)) as u8));
        let __p3 =
            (((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(253);
        (__p3).write((((((__p3).read()) as i32).wrapping_add(5i32)) as u8));
        if ((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(251))
            .read()) as i32)
            < 80i32
        {
            DestroyTask(taskId);
        }
    }
}
pub(crate) unsafe extern "C" fn Task_CloseCenterWhiteColumn(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        if (((data).read()) as i32) == 0i32 {
            ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(251))
                .write(80u8);
            ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(253))
                .write(160u8);
            SetGpuReg(74u8, 16u16);
            SetGpuReg(72u8, 19u16);
        }
        SetGpuReg(
            64u8,
            ((((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(253))
                .read()) as i32)
                | (((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(251))
                .read()) as i32)
                    << 8)) as u16),
        );
        SetGpuReg(
            68u8,
            ((((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(254))
                .read()) as i32)
                | (((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(252))
                .read()) as i32)
                    << 8)) as u16),
        );
        if ((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(251))
            .read()) as i32)
            != crate::c::div_i32(240i32, 2i32)
        {
            (data).write(((data).read()).wrapping_add(1));
            let __p1 =
                (((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(251);
            (__p1).write((((((__p1).read()) as i32).wrapping_add(5i32)) as u8));
            let __p2 =
                (((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(253);
            (__p2).write((((((__p2).read()) as i32).wrapping_sub(5i32)) as u8));
            if ((((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(251))
            .read()) as i32)
                > (crate::c::div_i32(240i32, 2i32)).wrapping_sub(5i32)
            {
                BlendPalettes(8u32, 0u8, 65535u16);
            }
        } else {
            ClearGpuRegBits(0u8, 8192u16);
            DestroyTask(taskId);
        }
    }
}
pub(crate) unsafe extern "C" fn CB2_SaveAndEndWirelessTrade() {
    unsafe {
        'l1: {
            let __sw1 = (((((&raw mut gMain).cast::<u8>()).wrapping_add(1080)).read()) as i32);
            if __sw1 == 0i32 {
                (((&raw mut gMain).cast::<u8>()).wrapping_add(1080)).write(1u8);
                StringExpandPlaceholders(
                    (&raw mut gStringVar4).cast::<u8>(),
                    (&raw mut gText_CommunicationStandby5).cast::<u8>(),
                );
                DrawTextOnTradeWindow(0u8, (&raw mut gStringVar4).cast::<u8>(), 0u8);
                break 'l1;
            }
            if __sw1 == 1i32 {
                SetTradeLinkStandbyCallback(0u8);
                (((&raw mut gMain).cast::<u8>()).wrapping_add(1080)).write(2u8);
                ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(100)
                    .cast::<u32>())
                .write(0u32);
                break 'l1;
            }
            if __sw1 == 2i32 {
                if (_IsLinkTaskFinished()) != 0 {
                    (((&raw mut gMain).cast::<u8>()).wrapping_add(1080)).write(3u8);
                    StringExpandPlaceholders(
                        (&raw mut gStringVar4).cast::<u8>(),
                        (&raw mut gText_SavingDontTurnOffPower).cast::<u8>(),
                    );
                    DrawTextOnTradeWindow(0u8, (&raw mut gStringVar4).cast::<u8>(), 0u8);
                    IncrementGameStat(21u8);
                    LinkFullSave_Init();
                    ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(100)
                        .cast::<u32>())
                    .write(0u32);
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                if {
                    let __p2 = (((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(100)
                        .cast::<u32>();
                    let __t3 = ((__p2).read()).wrapping_add(1);
                    (__p2).write(__t3);
                    __t3
                } == 5u32
                {
                    (((&raw mut gMain).cast::<u8>()).wrapping_add(1080)).write(4u8);
                }
                break 'l1;
            }
            if __sw1 == 4i32 {
                if (LinkFullSave_WriteSector()) != 0 {
                    (((&raw mut gMain).cast::<u8>()).wrapping_add(1080)).write(5u8);
                } else {
                    ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(100)
                        .cast::<u32>())
                    .write(0u32);
                    (((&raw mut gMain).cast::<u8>()).wrapping_add(1080)).write(3u8);
                }
                break 'l1;
            }
            if __sw1 == 5i32 {
                LinkFullSave_ReplaceLastSector();
                (((&raw mut gMain).cast::<u8>()).wrapping_add(1080)).write(6u8);
                ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(100)
                    .cast::<u32>())
                .write(0u32);
                break 'l1;
            }
            if __sw1 == 6i32 {
                if {
                    let __p4 = (((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(100)
                        .cast::<u32>();
                    let __t5 = ((__p4).read()).wrapping_add(1);
                    (__p4).write(__t5);
                    __t5
                } > 10u32
                {
                    if ((GetMultiplayerId()) as i32) == 0i32 {
                        ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(100)
                            .cast::<u32>())
                        .write(((crate::c::rem_i32(((Random()) as i32), 30i32)) as u32));
                    } else {
                        ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(100)
                            .cast::<u32>())
                        .write(0u32);
                    }
                    (((&raw mut gMain).cast::<u8>()).wrapping_add(1080)).write(7u8);
                }
                break 'l1;
            }
            if __sw1 == 7i32 {
                if ((((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(100)
                    .cast::<u32>())
                .read()
                    == 0u32
                {
                    SetTradeLinkStandbyCallback(1u8);
                    (((&raw mut gMain).cast::<u8>()).wrapping_add(1080)).write(8u8);
                } else {
                    let __p6 = (((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(100)
                        .cast::<u32>();
                    (__p6).write(((__p6).read()).wrapping_sub(1));
                }
                break 'l1;
            }
            if __sw1 == 8i32 {
                if (_IsLinkTaskFinished()) != 0 {
                    LinkFullSave_SetLastSectorSignature();
                    (((&raw mut gMain).cast::<u8>()).wrapping_add(1080)).write(9u8);
                }
                break 'l1;
            }
            if __sw1 == 9i32 {
                if {
                    let __p7 = (((&raw mut sTradeAnim).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(100)
                        .cast::<u32>();
                    let __t8 = ((__p7).read()).wrapping_add(1);
                    (__p7).write(__t8);
                    __t8
                } > 60u32
                {
                    let __p9 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                    (__p9).write(((__p9).read()).wrapping_add(1));
                    SetTradeLinkStandbyCallback(2u8);
                }
                break 'l1;
            }
            if __sw1 == 10i32 {
                if (_IsLinkTaskFinished()) != 0 {
                    FadeOutBGM(3u8);
                    BeginNormalPaletteFade(4294967295u32, 0i8, 0u8, 16u8, 0u16);
                    (((&raw mut gMain).cast::<u8>()).wrapping_add(1080)).write(11u8);
                }
                break 'l1;
            }
            if __sw1 == 11i32 {
                if (!((crate::c::bf_read(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
                    7,
                    1,
                    false,
                ) as u16)
                    != 0))
                    && (((IsBGMStopped()) as i32) == 1i32)
                {
                    SetTradeLinkStandbyCallback(3u8);
                    (((&raw mut gMain).cast::<u8>()).wrapping_add(1080)).write(12u8);
                }
                break 'l1;
            }
            if __sw1 == 12i32 {
                if (_IsLinkTaskFinished()) != 0 {
                    ((&raw mut gSoftResetDisabled).cast::<u8>()).write(0u8);
                    SetMainCallback2(Some(CB2_FreeTradeAnim));
                }
                break 'l1;
            }
        }
        if !((HasLinkErrorOccurred()) != 0) {
            RunTasks();
        }
        AnimateSprites();
        BuildOamBuffer();
        UpdatePaletteFade();
    }
}
