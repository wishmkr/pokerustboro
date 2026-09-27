//! Translated from `src/party_menu.c` by tools/rustport/c2rs.py, then reviewed.
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

// Data tables (translate with cdata.py): gTutorMoves sTutorLearnsets sPartyMenuBgTemplates sPartyBoxInfoRects sPartyMenuSpriteCoords sConfirmButton_Tilemap sCancelButton_Tilemap sFontColorTable sSinglePartyMenuWindowTemplate sDoublePartyMenuWindowTemplate sMultiPartyMenuWindowTemplate sShowcaseMultiPartyMenuWindowTemplate sCancelButtonWindowTemplate sMultiCancelButtonWindowTemplate sConfirmButtonWindowTemplate sDefaultPartyMsgWindowTemplate sDoWhatWithMonMsgWindowTemplate sDoWhatWithItemMsgWindowTemplate sDoWhatWithMailMsgWindowTemplate sWhichMoveMsgWindowTemplate sAlreadyHoldingOneMsgWindowTemplate sItemGiveTakeWindowTemplate sMailReadTakeWindowTemplate sMoveSelectWindowTemplate sPartyMenuYesNoWindowTemplate sLevelUpStatsWindowTemplate sUnusedWindowTemplate1 sUnusedWindowTemplate2 sSlotTilemap_Main sSlotTilemap_MainNoHP sSlotTilemap_Wide sSlotTilemap_WideNoHP sSlotTilemap_WideEmpty sGenderPalOffsets sHPBarPalOffsets sPartyBoxPalOffsets1 sPartyBoxPalOffsets2 sPartyBoxNoMonPalOffsets sGenderMalePalIds sGenderFemalePalIds sHPBarGreenPalIds sHPBarYellowPalIds sHPBarRedPalIds sPartyBoxEmptySlotPalIds1 sPartyBoxMultiPalIds1 sPartyBoxFaintedPalIds1 sPartyBoxCurrSelectionPalIds1 sPartyBoxCurrSelectionMultiPalIds sPartyBoxCurrSelectionFaintedPalIds sPartyBoxSelectedForActionPalIds1 sPartyBoxEmptySlotPalIds2 sPartyBoxMultiPalIds2 sPartyBoxFaintedPalIds2 sPartyBoxCurrSelectionPalIds2 sPartyBoxSelectedForActionPalIds2 sPartyBoxNoMonPalIds sActionStringTable sDescriptionStringTable sUnusedData sCursorOptions sPartyMenuAction_SummarySwitchCancel sPartyMenuAction_ShiftSummaryCancel sPartyMenuAction_SendOutSummaryCancel sPartyMenuAction_SummaryCancel sPartyMenuAction_EnterSummaryCancel sPartyMenuAction_NoEntrySummaryCancel sPartyMenuAction_StoreSummaryCancel sPartyMenuAction_GiveTakeItemCancel sPartyMenuAction_ReadTakeMailCancel sPartyMenuAction_RegisterSummaryCancel sPartyMenuAction_TradeSummaryCancel1 sPartyMenuAction_TradeSummaryCancel2 sPartyMenuAction_TakeItemTossCancel sPartyMenuActions sPartyMenuActionCounts sFieldMoves sFieldMoveCursorCallbacks sUnionRoomTradeMessages sHeldItemGfx sHeldItemPalette sOamData_HeldItem sSpriteAnim_HeldItem sSpriteAnim_HeldMail sSpriteAnimTable_HeldItem sSpriteSheet_HeldItem sSpritePalette_HeldItem sSpriteTemplate_HeldItem sOamData_MenuPokeball sPokeballAnim_Closed sPokeballAnim_Open sSpriteAnimTable_MenuPokeball sSpriteSheet_MenuPokeball sSpritePalette_MenuPokeball sSpriteTemplate_MenuPokeball sOamData_MenuPokeballSmall sSmallPokeballAnim_Closed sSmallPokeballAnim_Open sSmallPokeballAnim_Blank1 sSmallPokeballAnim_Blank2 sSmallPokeballAnim_Blank3 sSmallPokeballAnim_Blank4 sSpriteAnimTable_MenuPokeballSmall sSpriteSheet_MenuPokeballSmall sSpriteTemplate_MenuPokeballSmall sOamData_StatusCondition sSpriteAnim_StatusPoison sSpriteAnim_StatusParalyzed sSpriteAnim_StatusSleep sSpriteAnim_StatusFrozen sSpriteAnim_StatusBurn sSpriteAnim_StatusPokerus sSpriteAnim_StatusFaint sSpriteAnim_Blank sSpriteTemplate_StatusCondition sSpriteSheet_StatusIcons sSpritePalette_StatusIcons sSpriteTemplate_StatusIcons sMultiBattlePartnersPartyMask sUnused_StatStrings sTMHMMoves
#[allow(unused_imports)]
use crate::data::party_menu::*;

#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sPartyMenuInternal: *mut u8 = core::ptr::null_mut();
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gPartyMenu: crate::ffi::Align4<[u8; 20]> = crate::ffi::Align4([0; 20]);
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sPartyMenuBoxes: *mut u8 = core::ptr::null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sPartyBgGfxTilemap: *mut u8 = core::ptr::null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sPartyBgTilemapBuffer: *mut u8 = core::ptr::null_mut();
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gPartyMenuUseExitCallback: u8 = 0u8;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gSelectedMonPartyId: u8 = 0u8;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gPostMenuFieldCallback: Option<unsafe extern "C" fn()> = None;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sSlot1TilemapBuffer: *mut u16 = core::ptr::null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sSlot2TilemapBuffer: *mut u16 = core::ptr::null_mut();
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gSelectedOrderFromParty: crate::ffi::Align4<[u8; 4]> = crate::ffi::Align4([0; 4]);
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sPartyMenuItemId: u16 = 0u16;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sUnused: u16 = 0u16;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gBattlePartyCurrentOrder: crate::ffi::Align4<[u8; 3]> = crate::ffi::Align4([0; 3]);
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gItemUseCB: Option<unsafe extern "C" fn(u8, Option<unsafe extern "C" fn(u8)>)> =
    None;

unsafe extern "C" {
    static mut gBattleStruct: u8;
    static mut gBattleTypeFlags: u8;
    static mut gBattlerInMenuId: u8;
    static mut gBattlerPartyIndexes: u8;
    static mut gBattlersCount: u8;
    static mut gCB2_AfterEvolution: u8;
    static mut gContestMonPartyIndex: u8;
    static mut gEnemyParty: u8;
    static mut gFieldCallback: u8;
    static mut gFieldCallback2: u8;
    static mut gFieldEffectArguments: u8;
    static mut gFrontierBannedSpecies: u8;
    static mut gItemEffectTable: u8;
    static mut gJPText_AreYouSureYouWantToSpinTradeMon: u8;
    static mut gLastViewedMonIndex: u8;
    static mut gMain: u8;
    static mut gMapHeader: u8;
    static mut gMenuText_Confirm: u8;
    static mut gMoveNames: u8;
    static mut gMoveToLearn: u8;
    static mut gMultiPartnerParty: u8;
    static mut gPPUpGetMask: u8;
    static mut gPaletteFade: u8;
    static mut gPartyMenuBg_Gfx: u8;
    static mut gPartyMenuBg_Pal: u8;
    static mut gPartyMenuBg_Tilemap: u8;
    static mut gPlayerPCItemPageInfo: u8;
    static mut gPlayerParty: u8;
    static mut gPlayerPartyCount: u8;
    static mut gPlttBufferFaded: u8;
    static mut gPlttBufferUnfaded: u8;
    static mut gPyramidBagMenuState: u8;
    static mut gRfuPartnerCompatibilityData: u8;
    static mut gSaveBlock1Ptr: u8;
    static mut gSpecialVar_0x8004: u8;
    static mut gSpecialVar_0x8005: u8;
    static mut gSpecialVar_ItemId: u8;
    static mut gSpecialVar_Result: u8;
    static mut gSpeciesNames: u8;
    static mut gSprites: u8;
    static mut gStandardMenuPalette: u8;
    static mut gStringVar1: u8;
    static mut gStringVar2: u8;
    static mut gStringVar4: u8;
    static mut gTasks: u8;
    static mut gTextFlags: u8;
    static mut gText_12PoofForgotMove: u8;
    static mut gText_Attack3: u8;
    static mut gText_BagFullCouldNotRemoveItem: u8;
    static mut gText_Cancel: u8;
    static mut gText_Cancel2: u8;
    static mut gText_CancelBattle: u8;
    static mut gText_CancelChallenge: u8;
    static mut gText_CancelParticipation: u8;
    static mut gText_CantSwitchWithAlly: u8;
    static mut gText_CantUseUntilNewBadge: u8;
    static mut gText_Defense3: u8;
    static mut gText_EggCantBattle: u8;
    static mut gText_EggCantBeTradedNow: u8;
    static mut gText_EscapeFromHere: u8;
    static mut gText_FemaleSymbol: u8;
    static mut gText_HP3: u8;
    static mut gText_ItemThrownAway: u8;
    static mut gText_LevelSymbol: u8;
    static mut gText_MailMessageWillBeLost: u8;
    static mut gText_MailSentToPC: u8;
    static mut gText_MailTakenFromPkmn: u8;
    static mut gText_MailTransferredFromMailbox: u8;
    static mut gText_MaleSymbol: u8;
    static mut gText_MoveNotLearned: u8;
    static mut gText_MovesPPIncreased: u8;
    static mut gText_NoMoreThanVar1Pkmn: u8;
    static mut gText_OnlyPkmnForBattle: u8;
    static mut gText_PCMailboxFull: u8;
    static mut gText_PPWasRestored: u8;
    static mut gText_PauseUntilPress: u8;
    static mut gText_PkmnAdoresBaseVar2Fell: u8;
    static mut gText_PkmnAlreadyHoldingItemSwitch: u8;
    static mut gText_PkmnAlreadyInBattle: u8;
    static mut gText_PkmnAlreadyKnows: u8;
    static mut gText_PkmnAlreadySelected: u8;
    static mut gText_PkmnBaseVar2StatIncreased: u8;
    static mut gText_PkmnBecameHealthy: u8;
    static mut gText_PkmnBurnHealed: u8;
    static mut gText_PkmnCantBeTradedNow: u8;
    static mut gText_PkmnCantLearnMove: u8;
    static mut gText_PkmnCantParticipate: u8;
    static mut gText_PkmnCantSwitchOut: u8;
    static mut gText_PkmnCuredOfParalysis: u8;
    static mut gText_PkmnCuredOfPoison: u8;
    static mut gText_PkmnElevatedToLvVar2: u8;
    static mut gText_PkmnFriendlyBaseVar2CantFall: u8;
    static mut gText_PkmnFriendlyBaseVar2Fell: u8;
    static mut gText_PkmnGotOverInfatuation: u8;
    static mut gText_PkmnHPRestoredByVar2: u8;
    static mut gText_PkmnHasNoEnergy: u8;
    static mut gText_PkmnHoldingItemCantHoldMail: u8;
    static mut gText_PkmnLearnedMove3: u8;
    static mut gText_PkmnNeedsToReplaceMove: u8;
    static mut gText_PkmnNotHolding: u8;
    static mut gText_PkmnSnappedOutOfConfusion: u8;
    static mut gText_PkmnThawedOut: u8;
    static mut gText_PkmnWasGivenItem: u8;
    static mut gText_PkmnWokeUp2: u8;
    static mut gText_ReceivedItemFromPkmn: u8;
    static mut gText_RemoveMailBeforeItem: u8;
    static mut gText_ReturnToHealingSpot: u8;
    static mut gText_ReturnToWaitingRoom: u8;
    static mut gText_SendMailToPC: u8;
    static mut gText_Slash: u8;
    static mut gText_SpAtk3: u8;
    static mut gText_SpDef3: u8;
    static mut gText_Speed2: u8;
    static mut gText_StopLearningMove2: u8;
    static mut gText_SwitchedPkmnItem: u8;
    static mut gText_ThrowAwayItem: u8;
    static mut gText_WhichMoveToForget: u8;
    static mut gText_WontHaveEffect: u8;
    static mut gUnionRoomOfferedSpecies: u8;
    static mut gUnionRoomRequestedMonType: u8;
    fn AddBagItem(a0: u16, a1: u16) -> u8;
    fn AddPCItem(a0: u16, a1: u16) -> u8;
    fn AddTextPrinterParameterized(
        a0: u8,
        a1: u8,
        a2: *mut u8,
        a3: u8,
        a4: u8,
        a5: u8,
        a6: Option<unsafe extern "C" fn(*mut u8, u16)>,
    ) -> u16;
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
    fn AdjustFriendship(a0: *mut u8, a1: u8);
    fn Alloc(a0: u32) -> *mut u8;
    fn AllocZeroed(a0: u32) -> *mut u8;
    fn AnimateSprites();
    fn AnyStorageMonWithMove(a0: u16) -> u32;
    fn AppendToList(a0: *mut u8, a1: *mut u8, a2: u8);
    fn BeginEvolutionScene(a0: *mut u8, a1: u16, a2: u8, a3: u8);
    fn BeginNormalPaletteFade(a0: u32, a1: i8, a2: u8, a3: u8, a4: u16) -> u8;
    fn BlendPalettes(a0: u32, a1: u8, a2: u16);
    fn BlitBitmapToWindow(a0: u8, a1: *mut u8, a2: u16, a3: u16, a4: u16, a5: u16);
    fn BuildOamBuffer();
    fn CB2_OpenFlyMap();
    fn CB2_ReturnToField();
    fn CB2_ReturnToFieldContinueScriptPlayMapMusic();
    fn CB2_ReturnToFieldWithOpenMenu();
    fn CB2_ReturnToPyramidBagMenu();
    fn CB2_SetUpReshowBattleScreenAfterMenu();
    fn CalculatePlayerPartyCount() -> u8;
    fn CanMonLearnTMHM(a0: *mut u8, a1: u8) -> u32;
    fn CanRegisterMonForTradingBoard(a0: crate::c::Rec4<4>, a1: u16, a2: u16, a3: u8) -> i32;
    fn CanSpinTradeMon(a0: *mut u8, a1: u16) -> i32;
    fn ChangeBgX(a0: u8, a1: i32, a2: u8) -> i32;
    fn CheckIfItemIsTMHMOrEvolutionStone(a0: u16) -> u8;
    fn CheckPartyPokerus(a0: *mut u8, a1: u8) -> u8;
    fn ChooseMonForSoftboiled(a0: u8);
    fn CleanupOverworldWindowsAndTilemaps();
    fn ClearMail(a0: *mut u8);
    fn ClearScheduledBgCopiesToVram();
    fn ClearStdWindowAndFrameToTransparent(a0: u8, a1: u8);
    fn ClearWindowTilemap(a0: u8);
    fn ConvertIntToDecimalStringN(a0: *mut u8, a1: i32, a2: i32, a3: u8) -> *mut u8;
    fn ConvertInternationalPlayerName(a0: *mut u8);
    fn CopyItemName(a0: u16, a1: *mut u8);
    fn CopyRectToBgTilemapBufferRect(
        a0: u8,
        a1: *mut u8,
        a2: u8,
        a3: u8,
        a4: u8,
        a5: u8,
        a6: u8,
        a7: u8,
        a8: u8,
        a9: u8,
        a10: u8,
        a11: i16,
        a12: i16,
    );
    fn CopyToBgTilemapBufferRect_ChangePalette(
        a0: u8,
        a1: *mut u8,
        a2: u8,
        a3: u8,
        a4: u8,
        a5: u8,
        a6: u8,
    );
    fn CopyToBufferFromBgTilemap(a0: u8, a1: *mut u16, a2: u8, a3: u8, a4: u8, a5: u8);
    fn CopyWindowToVram(a0: u8, a1: u8);
    fn CpuSet(a0: *mut u8, a1: *mut u8, a2: u32);
    fn CreateMonIcon(
        a0: u16,
        a1: Option<unsafe extern "C" fn(*mut u8)>,
        a2: i16,
        a3: i16,
        a4: u8,
        a5: u32,
        a6: u32,
    ) -> u8;
    fn CreateSprite(a0: *mut u8, a1: i16, a2: i16, a3: u8) -> u8;
    fn CreateTask(a0: Option<unsafe extern "C" fn(u8)>, a1: u8) -> u8;
    fn CreateYesNoMenu(a0: *mut u8, a1: u16, a2: u8, a3: u8);
    fn CurrentBattlePyramidLocation() -> u8;
    fn DeactivateAllTextPrinters();
    fn DestroyTask(a0: u8);
    fn DoEasyChatScreen(a0: u8, a1: *mut u16, a2: Option<unsafe extern "C" fn()>, a3: u8);
    fn DoScheduledBgTilemapCopiesToVram();
    fn DrawLevelUpWindowPg1(a0: u16, a1: *mut u16, a2: *mut u16, a3: u8, a4: u8, a5: u8);
    fn DrawLevelUpWindowPg2(a0: u16, a1: *mut u16, a2: u8, a3: u8, a4: u8);
    fn DrawStdFrameWithCustomTileAndPalette(a0: u8, a1: u8, a2: u16, a3: u8);
    fn ExecuteTableBasedItemEffect(a0: *mut u8, a1: u16, a2: u8, a3: u8) -> u8;
    fn FadeInFromBlack();
    fn FadeScreen(a0: u8, a1: i8);
    fn FieldCB_ContinueScriptHandleMusic();
    fn FieldEffectStart(a0: u8) -> u32;
    fn FillBgTilemapBufferRect(a0: u8, a1: u16, a2: u8, a3: u8, a4: u8, a5: u8, a6: u8);
    fn FillBgTilemapBufferRect_Palette0(a0: u8, a1: u16, a2: u8, a3: u8, a4: u8, a5: u8);
    fn FillWindowPixelBuffer(a0: u8, a1: u8);
    fn FillWindowPixelRect(a0: u8, a1: u8, a2: u16, a3: u16, a4: u16, a5: u16);
    fn FlagGet(a0: u16) -> u8;
    fn Free(a0: *mut u8);
    fn FreeAllSpritePalettes();
    fn FreeAllWindowBuffers();
    fn FuncIsActiveTask(a0: Option<unsafe extern "C" fn(u8)>) -> u8;
    fn GetBattlerAtPosition(a0: u8) -> u8;
    fn GetBattlerSide(a0: u8) -> u8;
    fn GetContestEntryEligibility(a0: *mut u8) -> u8;
    fn GetEvolutionTargetSpecies(a0: *mut u8, a1: u8, a2: u16) -> u16;
    fn GetFontAttribute(a0: u8, a1: u8) -> u8;
    fn GetHPBarLevel(a0: i16, a1: i16) -> u8;
    fn GetHostRfuGameData() -> *mut u8;
    fn GetLRKeysPressedAndHeld() -> u8;
    fn GetMapNameGeneric(a0: *mut u8, a1: u16) -> *mut u8;
    fn GetMenuCursorDimensionByFont(a0: u8, a1: u8) -> u8;
    fn GetMonData2(a0: *mut u8, a1: i32) -> u32;
    fn GetMonData3(a0: *mut u8, a1: i32, a2: *mut u8) -> u32;
    fn GetMonGender(a0: *mut u8) -> u8;
    fn GetMoveSlotToReplace() -> u8;
    fn GetNumberOfRelearnableMoves(a0: *mut u8) -> u8;
    fn GetOverworldTextboxPalettePtr() -> *mut u16;
    fn GetPlayerFlankId() -> u8;
    fn GetPlayerTextSpeedDelay() -> u8;
    fn GetPocketByItemId(a0: u16) -> u8;
    fn GetScaledHPFraction(a0: i16, a1: i16, a2: u8) -> u8;
    fn GetStringCenterAlignXOffset(a0: i32, a1: *mut u8, a2: i32) -> i32;
    fn GetTrainerPartnerName() -> *mut u8;
    fn GetUnionRoomTradeMessageId(
        a0: crate::c::Rec4<4>,
        a1: crate::c::Rec4<4>,
        a2: u16,
        a3: u16,
        a4: u8,
        a5: u16,
        a6: u8,
    ) -> i32;
    fn GetWindowAttribute(a0: u8, a1: u8) -> u32;
    fn GetXYCoordsOneStepInFrontOfPlayer(a0: *mut i16, a1: *mut i16);
    fn GiveMailToMon(a0: *mut u8, a1: *mut u8) -> u8;
    fn GiveMailToMonByItemId(a0: *mut u8, a1: u16) -> u8;
    fn GiveMoveToMon(a0: *mut u8, a1: u16) -> u16;
    fn GoToBagMenu(a0: u8, a1: u8, a2: Option<unsafe extern "C" fn()>);
    fn GoToBattlePyramidBagMenu(a0: u8, a1: Option<unsafe extern "C" fn()>);
    fn HandleBattleLowHpMusicChange();
    fn InBattlePike() -> u8;
    fn InMultiPartnerRoom() -> u8;
    fn InUnionRoom() -> u32;
    fn InitBgsFromTemplates(a0: u8, a1: *mut u8, a2: u8);
    fn InitMenuInUpperLeftCorner(a0: u8, a1: u8, a2: u8, a3: u8) -> u8;
    fn InitMenuInUpperLeftCornerNormal(a0: u8, a1: u8, a2: u8) -> u8;
    fn InitWindows(a0: *mut u8) -> u16;
    fn IsDma3ManagerBusyWithBgCopy() -> u8;
    fn IsDoubleBattle() -> u8;
    fn IsFanfareTaskInactive() -> u8;
    fn IsPlayerFacingSurfableFishableWater() -> u8;
    fn IsPlayerSurfingNorth() -> u8;
    fn IsSpeciesAllowedInPokemonJump(a0: u16) -> u32;
    fn IsWeatherNotFadingIn() -> u8;
    fn ItemIsMail(a0: u16) -> u8;
    fn LZDecompressWram(a0: *mut u32, a1: *mut u8);
    fn LoadBgTiles(a0: u8, a1: *mut u8, a2: u16, a3: u16) -> u16;
    fn LoadCompressedPalette(a0: *mut u32, a1: u16, a2: u16);
    fn LoadCompressedSpritePalette(a0: *mut u8);
    fn LoadCompressedSpriteSheet(a0: *mut u8) -> u16;
    fn LoadMonIconPalettes();
    fn LoadOam();
    fn LoadPalette(a0: *mut u8, a1: u16, a2: u16);
    fn LoadSpritePalette(a0: *mut u8) -> u8;
    fn LoadSpriteSheet(a0: *mut u8) -> u16;
    fn LoadUserWindowBorderGfx(a0: u8, a1: u16, a2: u8);
    fn LockPlayerFieldControls();
    fn Mailbox_ReturnToMailListAfterDeposit();
    fn MapGridGetMetatileBehaviorAt(a0: i32, a1: i32) -> i32;
    fn MenuHelpers_IsLinkActive() -> u8;
    fn MenuHelpers_ShouldWaitForLinkRecv() -> u8;
    fn Menu_GetCursorPos() -> u8;
    fn Menu_ProcessInput() -> i8;
    fn Menu_ProcessInputNoWrapAround_other() -> i8;
    fn Menu_ProcessInputNoWrapClearOnChoose() -> i8;
    fn MetatileBehavior_IsWaterfall(a0: u8) -> u8;
    fn MonTryLearningNewMove(a0: *mut u8, a1: u8) -> u16;
    fn Overworld_GetMapHeaderByGroupAndId(a0: u16, a1: u16) -> *mut u8;
    fn Overworld_MapTypeAllowsTeleportAndFly(a0: u8) -> u8;
    fn PartyHasMonWithSurf() -> u8;
    fn PlayFanfare(a0: u16);
    fn PlayFanfareByFanfareNum(a0: u8);
    fn PlaySE(a0: u16);
    fn ProcessMenuInput_other() -> i8;
    fn ProcessSpriteCopyRequests();
    fn PutWindowTilemap(a0: u8);
    fn ReadMail(a0: *mut u8, a1: Option<unsafe extern "C" fn()>, a2: u8);
    fn RemoveBagItem(a0: u16, a1: u16) -> u8;
    fn RemoveMonPPBonus(a0: *mut u8, a1: u8);
    fn RemovePCItem(a0: u8, a1: u16);
    fn RemoveWindow(a0: u8);
    fn ResetAllBgsCoordinates();
    fn ResetBgsAndClearDma3BusyFlags(a0: u32);
    fn ResetPaletteFade();
    fn ResetSpriteData();
    fn ResetTasks();
    fn ResetVramOamAndBgCntRegs();
    fn ReshowBattleScreenDummy();
    fn RunTasks();
    fn RunTextPrintersRetIsActive(a0: u8) -> u16;
    fn ScanlineEffect_Stop();
    fn ScheduleBgCopyTilemapToVram(a0: u8);
    fn ScriptContext_Enable();
    fn SetBgTilemapBuffer(a0: u8, a1: *mut u8);
    fn SetBgTilemapPalette(a0: u8, a1: u8, a2: u8, a3: u8, a4: u8, a5: u8);
    fn SetGpuReg(a0: u8, a1: u16);
    fn SetMainCallback2(a0: Option<unsafe extern "C" fn()>);
    fn SetMonData(a0: *mut u8, a1: i32, a2: *mut u8);
    fn SetMonMoveSlot(a0: *mut u8, a1: u16, a2: u8);
    fn SetMonPreventsSwitchingString();
    fn SetPartyHPBarSprite(a0: *mut u8, a1: u8);
    fn SetTaskFuncWithFollowupFunc(
        a0: u8,
        a1: Option<unsafe extern "C" fn(u8)>,
        a2: Option<unsafe extern "C" fn(u8)>,
    );
    fn SetVBlankCallback(a0: Option<unsafe extern "C" fn()>);
    fn SetVBlankHBlankCallbacksToNull();
    fn SetWindowTemplateFields(
        a0: *mut u8,
        a1: u8,
        a2: u8,
        a3: u8,
        a4: u8,
        a5: u8,
        a6: u8,
        a7: u16,
    );
    fn ShowBg(a0: u8);
    fn ShowPokemonSummaryScreen(
        a0: u8,
        a1: *mut u8,
        a2: u8,
        a3: u8,
        a4: Option<unsafe extern "C" fn()>,
    );
    fn ShowSelectMovePokemonSummaryScreen(
        a0: *mut u8,
        a1: u8,
        a2: u8,
        a3: Option<unsafe extern "C" fn()>,
        a4: u16,
    );
    fn SpriteCB_MonIcon(a0: *mut u8);
    fn StartSpriteAnim(a0: *mut u8, a1: u8);
    fn StringAppend(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn StringCompare(a0: *mut u8, a1: *mut u8) -> i32;
    fn StringCopy(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn StringExpandPlaceholders(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn StringGet_Nickname(a0: *mut u8) -> *mut u8;
    fn SwitchTaskToFollowupFunc(a0: u8);
    fn TakeMailFromMon(a0: *mut u8);
    fn TakeMailFromMonAndSave(a0: *mut u8) -> u8;
    fn Task_TryUseSoftboiledOnPartyMon(a0: u8);
    fn TestPlayerAvatarFlags(a0: u8) -> u8;
    fn TransferPlttBuffer();
    fn TrySetDiveWarp() -> u8;
    fn UnlockPlayerFieldControls();
    fn UpdateMonIconFrame(a0: *mut u8) -> u8;
    fn UpdatePaletteFade() -> u8;
    fn VarGet(a0: u16) -> u16;
    fn WaitFanfare(a0: u8) -> u8;
    fn malloc_and_decompress(a0: *mut u8, a1: *mut u32) -> *mut u8;
}

pub(crate) unsafe extern "C" fn InitPartyMenu(
    menuType: u8,
    layout: u8,
    partyAction: u8,
    keepCursorPos: u8,
    messageId: u8,
    task: Option<unsafe extern "C" fn(u8)>,
    callback: Option<unsafe extern "C" fn()>,
) {
    unsafe {
        let mut menuType = menuType;
        let mut layout = layout;
        let mut partyAction = partyAction;
        let mut keepCursorPos = keepCursorPos;
        let mut messageId = messageId;
        let mut task = task;
        let mut callback = callback;
        let mut i: u16 = 0u16;
        ResetPartyMenu();
        ((&raw mut sPartyMenuInternal).cast::<u8>().cast::<*mut u8>()).write(Alloc(568u32));
        if ((((&raw mut sPartyMenuInternal).cast::<u8>().cast::<*mut u8>()).read()) as usize)
            == 0usize
        {
            SetMainCallback2(callback);
        } else {
            crate::c::bf_write(
                ((&raw mut gPartyMenu).cast::<u8>()).wrapping_add(8),
                0,
                4,
                (menuType) as i32,
            );
            (((&raw mut gPartyMenu).cast::<u8>()).cast::<Option<unsafe extern "C" fn()>>())
                .write(callback);
            (((&raw mut gPartyMenu).cast::<u8>()).wrapping_add(11)).write(partyAction);
            crate::c::bf_write(
                (((&raw mut sPartyMenuInternal).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(10),
                2,
                14,
                ((messageId) as u32) as i32,
            );
            ((((&raw mut sPartyMenuInternal).cast::<u8>().cast::<*mut u8>()).read())
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(task);
            ((((&raw mut sPartyMenuInternal).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(4)
                .cast::<Option<unsafe extern "C" fn()>>())
            .write(None);
            crate::c::bf_write(
                (((&raw mut sPartyMenuInternal).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(8),
                1,
                3,
                (0u32) as i32,
            );
            crate::c::bf_write(
                (((&raw mut sPartyMenuInternal).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(8),
                4,
                7,
                (127u32) as i32,
            );
            crate::c::bf_write(
                (((&raw mut sPartyMenuInternal).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(9),
                3,
                7,
                (127u32) as i32,
            );
            if ((menuType) as i32) == 4i32 {
                crate::c::bf_write(
                    (((&raw mut sPartyMenuInternal).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(8),
                    0,
                    1,
                    (1u32) as i32,
                );
            } else {
                crate::c::bf_write(
                    (((&raw mut sPartyMenuInternal).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(8),
                    0,
                    1,
                    (0u32) as i32,
                );
            }
            if ((layout) as i32) != 255i32 {
                crate::c::bf_write(
                    ((&raw mut gPartyMenu).cast::<u8>()).wrapping_add(8),
                    4,
                    2,
                    (layout) as i32,
                );
            }
            {
                i = 0u16;
                'l1: loop {
                    if !(((i) as u32) < crate::c::div_u32(32u32, 2u32)) {
                        break 'l1;
                    }
                    'l2: {
                        ((((((&raw mut sPartyMenuInternal).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(536))
                        .cast::<i16>())
                        .wrapping_offset(((i) as i32) as isize))
                        .write(0i16);
                    }
                    i = (i).wrapping_add(1);
                }
            }
            {
                i = 0u16;
                'l3: loop {
                    if !(((i) as u32) < crate::c::div_u32(3u32, 1u32)) {
                        break 'l3;
                    }
                    'l4: {
                        ((((((&raw mut sPartyMenuInternal).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(12))
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize))
                        .write(255u8);
                    }
                    i = (i).wrapping_add(1);
                }
            }
            if !((keepCursorPos) != 0) {
                (((&raw mut gPartyMenu).cast::<u8>())
                    .wrapping_add(9)
                    .cast::<i8>())
                .write(0i8);
            } else {
                if ((((((&raw mut gPartyMenu).cast::<u8>())
                    .wrapping_add(9)
                    .cast::<i8>())
                .read()) as i32)
                    > 5i32)
                    || (GetMonData2(
                        ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                            (((((&raw mut gPartyMenu).cast::<u8>())
                                .wrapping_add(9)
                                .cast::<i8>())
                            .read()) as i32) as isize
                                * 100,
                        ),
                        11i32,
                    ) == 0u32)
                {
                    (((&raw mut gPartyMenu).cast::<u8>())
                        .wrapping_add(9)
                        .cast::<i8>())
                    .write(0i8);
                }
            }
            crate::c::bf_write(
                ((&raw mut gTextFlags).cast::<u8>()).wrapping_add(0),
                2,
                1,
                (0u8) as i32,
            );
            CalculatePlayerPartyCount();
            SetMainCallback2(Some(CB2_InitPartyMenu));
        }
    }
}
pub(crate) unsafe extern "C" fn CB2_UpdatePartyMenu() {
    unsafe {
        RunTasks();
        AnimateSprites();
        BuildOamBuffer();
        DoScheduledBgTilemapCopiesToVram();
        UpdatePaletteFade();
    }
}
pub(crate) unsafe extern "C" fn VBlankCB_PartyMenu() {
    unsafe {
        LoadOam();
        ProcessSpriteCopyRequests();
        TransferPlttBuffer();
    }
}
pub(crate) unsafe extern "C" fn CB2_InitPartyMenu() {
    unsafe {
        'l1: loop {
            if !((1i32) != 0) {
                break 'l1;
            }
            if ((((MenuHelpers_ShouldWaitForLinkRecv()) as i32) == 1i32)
                || (((ShowPartyMenu()) as i32) == 1i32))
                || (((MenuHelpers_IsLinkActive()) as i32) == 1i32)
            {
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn ShowPartyMenu() -> u8 {
    unsafe {
        'l1: {
            let __sw1 = (((((&raw mut gMain).cast::<u8>()).wrapping_add(1080)).read()) as i32);
            let __matched = __sw1 == 0i32
                || __sw1 == 1i32
                || __sw1 == 2i32
                || __sw1 == 3i32
                || __sw1 == 4i32
                || __sw1 == 5i32
                || __sw1 == 6i32
                || __sw1 == 7i32
                || __sw1 == 8i32
                || __sw1 == 9i32
                || __sw1 == 10i32
                || __sw1 == 11i32
                || __sw1 == 12i32
                || __sw1 == 13i32
                || __sw1 == 14i32
                || __sw1 == 15i32
                || __sw1 == 16i32
                || __sw1 == 17i32
                || __sw1 == 18i32
                || __sw1 == 19i32
                || __sw1 == 20i32
                || __sw1 == 21i32
                || __sw1 == 22i32;
            if __sw1 == 0i32 {
                SetVBlankHBlankCallbacksToNull();
                ResetVramOamAndBgCntRegs();
                ClearScheduledBgCopiesToVram();
                let __p2 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                ScanlineEffect_Stop();
                let __p3 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p3).write(((__p3).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 2i32 {
                ResetPaletteFade();
                crate::c::bf_write(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(8),
                    7,
                    1,
                    (1u16) as i32,
                );
                let __p4 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p4).write(((__p4).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 3i32 {
                ResetSpriteData();
                let __p5 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p5).write(((__p5).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 4i32 {
                FreeAllSpritePalettes();
                let __p6 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p6).write(((__p6).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 5i32 {
                if !((MenuHelpers_IsLinkActive()) != 0) {
                    ResetTasks();
                }
                let __p7 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p7).write(((__p7).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 6i32 {
                SetPartyMonsAllowedInMinigame();
                let __p8 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p8).write(((__p8).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 7i32 {
                if !((AllocPartyMenuBg()) != 0) {
                    ExitPartyMenu();
                    return 1u8;
                } else {
                    (((((&raw mut sPartyMenuInternal).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(536))
                    .cast::<i16>())
                    .write(0i16);
                    let __p9 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                    (__p9).write(((__p9).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 8i32 {
                if (AllocPartyMenuBgGfx()) != 0 {
                    let __p10 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                    (__p10).write(((__p10).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 9i32 {
                InitPartyMenuWindows(
                    (crate::c::bf_read(
                        ((&raw mut gPartyMenu).cast::<u8>()).wrapping_add(8),
                        4,
                        2,
                        false,
                    ) as u8),
                );
                let __p11 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p11).write(((__p11).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 10i32 {
                InitPartyMenuBoxes(
                    (crate::c::bf_read(
                        ((&raw mut gPartyMenu).cast::<u8>()).wrapping_add(8),
                        4,
                        2,
                        false,
                    ) as u8),
                );
                (((((&raw mut sPartyMenuInternal).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(536))
                .cast::<i16>())
                .write(0i16);
                let __p12 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p12).write(((__p12).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 11i32 {
                LoadHeldItemIcons();
                let __p13 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p13).write(((__p13).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 12i32 {
                LoadPartyMenuPokeballGfx();
                let __p14 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p14).write(((__p14).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 13i32 {
                LoadPartyMenuAilmentGfx();
                let __p15 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p15).write(((__p15).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 14i32 {
                LoadMonIconPalettes();
                let __p16 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p16).write(((__p16).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 15i32 {
                if (CreatePartyMonSpritesLoop()) != 0 {
                    (((((&raw mut sPartyMenuInternal).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(536))
                    .cast::<i16>())
                    .write(0i16);
                    let __p17 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                    (__p17).write(((__p17).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 16i32 {
                if (RenderPartyMenuBoxes()) != 0 {
                    (((((&raw mut sPartyMenuInternal).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(536))
                    .cast::<i16>())
                    .write(0i16);
                    let __p18 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                    (__p18).write(((__p18).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 17i32 {
                CreateCancelConfirmPokeballSprites();
                let __p19 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p19).write(((__p19).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 18i32 {
                CreateCancelConfirmWindows(
                    ((crate::c::bf_read(
                        (((&raw mut sPartyMenuInternal).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(8),
                        0,
                        1,
                        false,
                    ) as u32) as u8),
                );
                let __p20 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p20).write(((__p20).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 19i32 {
                let __p21 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p21).write(((__p21).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 20i32 {
                CreateTask(
                    ((((&raw mut sPartyMenuInternal).cast::<u8>().cast::<*mut u8>()).read())
                        .cast::<Option<unsafe extern "C" fn(u8)>>())
                    .read(),
                    0u8,
                );
                DisplayPartyMenuStdMessage(
                    (crate::c::bf_read(
                        (((&raw mut sPartyMenuInternal).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(10),
                        2,
                        14,
                        false,
                    ) as u32),
                );
                let __p22 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p22).write(((__p22).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 21i32 {
                BlendPalettes(4294967295u32, 16u8, 0u16);
                crate::c::bf_write(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(8),
                    7,
                    1,
                    (0u16) as i32,
                );
                let __p23 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p23).write(((__p23).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 22i32 {
                BeginNormalPaletteFade(4294967295u32, 0i8, 16u8, 0u8, 0u16);
                let __p24 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p24).write(((__p24).read()).wrapping_add(1));
                break 'l1;
            }
            if !__matched {
                SetVBlankCallback(Some(VBlankCB_PartyMenu));
                SetMainCallback2(Some(CB2_UpdatePartyMenu));
                return 1u8;
            }
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn ExitPartyMenu() {
    unsafe {
        BeginNormalPaletteFade(4294967295u32, 0i8, 0u8, 16u8, 0u16);
        CreateTask(Some(Task_ExitPartyMenu), 0u8);
        SetVBlankCallback(Some(VBlankCB_PartyMenu));
        SetMainCallback2(Some(CB2_UpdatePartyMenu));
    }
}
pub(crate) unsafe extern "C" fn Task_ExitPartyMenu(taskId: u8) {
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
            SetMainCallback2(
                (((&raw mut gPartyMenu).cast::<u8>()).cast::<Option<unsafe extern "C" fn()>>())
                    .read(),
            );
            FreePartyPointers();
            DestroyTask(taskId);
        }
    }
}
pub(crate) unsafe extern "C" fn ResetPartyMenu() {
    unsafe {
        ((&raw mut sPartyMenuInternal).cast::<u8>().cast::<*mut u8>()).write(core::ptr::null_mut());
        ((&raw mut sPartyBgTilemapBuffer)
            .cast::<u8>()
            .cast::<*mut u8>())
        .write(core::ptr::null_mut());
        ((&raw mut sPartyMenuBoxes).cast::<u8>().cast::<*mut u8>()).write(core::ptr::null_mut());
        ((&raw mut sPartyBgGfxTilemap).cast::<u8>().cast::<*mut u8>()).write(core::ptr::null_mut());
    }
}
pub(crate) unsafe extern "C" fn AllocPartyMenuBg() -> u8 {
    unsafe {
        ((&raw mut sPartyBgTilemapBuffer)
            .cast::<u8>()
            .cast::<*mut u8>())
        .write(Alloc(2048u32));
        if ((((&raw mut sPartyBgTilemapBuffer)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read()) as usize)
            == 0usize
        {
            return 0u8;
        }
        crate::c::memset(
            ((&raw mut sPartyBgTilemapBuffer)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read(),
            0i32,
            2048u32,
        );
        ResetBgsAndClearDma3BusyFlags(0u32);
        InitBgsFromTemplates(
            0u8,
            ((&raw const sPartyMenuBgTemplates).cast::<u8>().cast_mut()).cast::<u8>(),
            ((crate::c::div_u32(12u32, 4u32)) as u8),
        );
        SetBgTilemapBuffer(
            1u8,
            ((&raw mut sPartyBgTilemapBuffer)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read(),
        );
        ResetAllBgsCoordinates();
        ScheduleBgCopyTilemapToVram(1u8);
        SetGpuReg(0u8, 4160u16);
        SetGpuReg(80u8, 0u16);
        ShowBg(0u8);
        ShowBg(1u8);
        ShowBg(2u8);
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn AllocPartyMenuBgGfx() -> u8 {
    unsafe {
        let mut sizeout: u32 = 0u32;
        'l1: {
            let __sw1 = (((((((&raw mut sPartyMenuInternal).cast::<u8>().cast::<*mut u8>())
                .read())
            .wrapping_add(536))
            .cast::<i16>())
            .read()) as i32);
            let __matched = __sw1 == 0i32
                || __sw1 == 1i32
                || __sw1 == 2i32
                || __sw1 == 3i32
                || __sw1 == 4i32
                || __sw1 == 5i32
                || __sw1 == 6i32
                || __sw1 == 7i32;
            if __sw1 == 0i32 {
                ((&raw mut sPartyBgGfxTilemap).cast::<u8>().cast::<*mut u8>()).write(
                    malloc_and_decompress(
                        (((&raw mut gPartyMenuBg_Gfx).cast::<u32>()).cast::<u32>()).cast::<u8>(),
                        &raw mut sizeout,
                    ),
                );
                LoadBgTiles(
                    1u8,
                    ((&raw mut sPartyBgGfxTilemap).cast::<u8>().cast::<*mut u8>()).read(),
                    ((sizeout) as u16),
                    0u16,
                );
                let __p2 = ((((&raw mut sPartyMenuInternal).cast::<u8>().cast::<*mut u8>())
                    .read())
                .wrapping_add(536))
                .cast::<i16>();
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                if !((IsDma3ManagerBusyWithBgCopy()) != 0) {
                    LZDecompressWram(
                        ((&raw mut gPartyMenuBg_Tilemap).cast::<u32>()).cast::<u32>(),
                        ((&raw mut sPartyBgTilemapBuffer)
                            .cast::<u8>()
                            .cast::<*mut u8>())
                        .read(),
                    );
                    let __p3 = ((((&raw mut sPartyMenuInternal).cast::<u8>().cast::<*mut u8>())
                        .read())
                    .wrapping_add(536))
                    .cast::<i16>();
                    (__p3).write(((__p3).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                LoadCompressedPalette(
                    ((&raw mut gPartyMenuBg_Pal).cast::<u32>()).cast::<u32>(),
                    0u16,
                    352u16,
                );
                'l2: loop {
                    'l3: {
                        'l4: loop {
                            'l5: {
                                CpuSet(
                                    (((&raw mut gPlttBufferUnfaded).cast::<u16>()).cast::<u16>())
                                        .cast::<u8>(),
                                    (((((&raw mut sPartyMenuInternal)
                                        .cast::<u8>()
                                        .cast::<*mut u8>())
                                    .read())
                                    .wrapping_add(24))
                                    .cast::<u16>())
                                    .cast::<u8>(),
                                    (0u32
                                        | (crate::c::div_u32(
                                            352u32,
                                            ((crate::c::div_i32(16i32, 8i32)) as u32),
                                        ) & 2097151u32)),
                                );
                            }
                            if !((0i32) != 0) {
                                break 'l4;
                            }
                        }
                    }
                    if !((0i32) != 0) {
                        break 'l2;
                    }
                }
                let __p4 = ((((&raw mut sPartyMenuInternal).cast::<u8>().cast::<*mut u8>())
                    .read())
                .wrapping_add(536))
                .cast::<i16>();
                (__p4).write(((__p4).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 3i32 {
                PartyPaletteBufferCopy(4u8);
                let __p5 = ((((&raw mut sPartyMenuInternal).cast::<u8>().cast::<*mut u8>())
                    .read())
                .wrapping_add(536))
                .cast::<i16>();
                (__p5).write(((__p5).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 4i32 {
                PartyPaletteBufferCopy(5u8);
                let __p6 = ((((&raw mut sPartyMenuInternal).cast::<u8>().cast::<*mut u8>())
                    .read())
                .wrapping_add(536))
                .cast::<i16>();
                (__p6).write(((__p6).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 5i32 {
                PartyPaletteBufferCopy(6u8);
                let __p7 = ((((&raw mut sPartyMenuInternal).cast::<u8>().cast::<*mut u8>())
                    .read())
                .wrapping_add(536))
                .cast::<i16>();
                (__p7).write(((__p7).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 6i32 {
                PartyPaletteBufferCopy(7u8);
                let __p8 = ((((&raw mut sPartyMenuInternal).cast::<u8>().cast::<*mut u8>())
                    .read())
                .wrapping_add(536))
                .cast::<i16>();
                (__p8).write(((__p8).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 7i32 {
                PartyPaletteBufferCopy(8u8);
                let __p9 = ((((&raw mut sPartyMenuInternal).cast::<u8>().cast::<*mut u8>())
                    .read())
                .wrapping_add(536))
                .cast::<i16>();
                (__p9).write(((__p9).read()).wrapping_add(1));
                break 'l1;
            }
            if !__matched {
                return 1u8;
            }
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn PartyPaletteBufferCopy(palNum: u8) {
    unsafe {
        let mut palNum = palNum;
        let mut offset: u8 = ((((palNum) as i32).wrapping_mul(16i32)) as u8);
        'l1: loop {
            'l2: {
                'l3: loop {
                    'l4: {
                        CpuSet(
                            ((((&raw mut gPlttBufferUnfaded).cast::<u16>()).cast::<u16>())
                                .wrapping_offset(48))
                            .cast::<u8>(),
                            ((((&raw mut gPlttBufferUnfaded).cast::<u16>()).cast::<u16>())
                                .wrapping_offset(((offset) as i32) as isize))
                            .cast::<u8>(),
                            (0u32
                                | (crate::c::div_u32(
                                    32u32,
                                    ((crate::c::div_i32(16i32, 8i32)) as u32),
                                ) & 2097151u32)),
                        );
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
        'l5: loop {
            'l6: {
                'l7: loop {
                    'l8: {
                        CpuSet(
                            ((((&raw mut gPlttBufferUnfaded).cast::<u16>()).cast::<u16>())
                                .wrapping_offset(48))
                            .cast::<u8>(),
                            ((((&raw mut gPlttBufferFaded).cast::<u16>()).cast::<u16>())
                                .wrapping_offset(((offset) as i32) as isize))
                            .cast::<u8>(),
                            (0u32
                                | (crate::c::div_u32(
                                    32u32,
                                    ((crate::c::div_i32(16i32, 8i32)) as u32),
                                ) & 2097151u32)),
                        );
                    }
                    if !((0i32) != 0) {
                        break 'l7;
                    }
                }
            }
            if !((0i32) != 0) {
                break 'l5;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn FreePartyPointers() {
    unsafe {
        if !(((&raw mut sPartyMenuInternal).cast::<u8>().cast::<*mut u8>()).read()).is_null() {
            Free(((&raw mut sPartyMenuInternal).cast::<u8>().cast::<*mut u8>()).read());
        }
        if !(((&raw mut sPartyBgTilemapBuffer)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .is_null()
        {
            Free(
                ((&raw mut sPartyBgTilemapBuffer)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .read(),
            );
        }
        if !(((&raw mut sPartyBgGfxTilemap).cast::<u8>().cast::<*mut u8>()).read()).is_null() {
            Free(((&raw mut sPartyBgGfxTilemap).cast::<u8>().cast::<*mut u8>()).read());
        }
        if !(((&raw mut sPartyMenuBoxes).cast::<u8>().cast::<*mut u8>()).read()).is_null() {
            Free(((&raw mut sPartyMenuBoxes).cast::<u8>().cast::<*mut u8>()).read());
        }
        FreeAllWindowBuffers();
    }
}
pub(crate) unsafe extern "C" fn InitPartyMenuBoxes(layout: u8) {
    unsafe {
        let mut layout = layout;
        let mut i: u8 = 0u8;
        ((&raw mut sPartyMenuBoxes).cast::<u8>().cast::<*mut u8>()).write(Alloc(96u32));
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 6i32) {
                    break 'l1;
                }
                'l2: {
                    (((((&raw mut sPartyMenuBoxes).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(((i) as i32) as isize * 16))
                    .cast::<*mut u8>())
                    .write(
                        (((&raw const sPartyBoxInfoRects).cast::<u8>().cast_mut()).cast::<u8>())
                            .wrapping_offset(32),
                    );
                    (((((&raw mut sPartyMenuBoxes).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(((i) as i32) as isize * 16))
                    .wrapping_add(4)
                    .cast::<*mut u8>())
                    .write(
                        ((((((&raw const sPartyMenuSpriteCoords).cast::<u8>().cast_mut())
                            .cast::<u8>())
                        .wrapping_offset(((layout) as i32) as isize * 48))
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 8))
                        .cast::<u8>(),
                    );
                    (((((&raw mut sPartyMenuBoxes).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(((i) as i32) as isize * 16))
                    .wrapping_add(8))
                    .write(i);
                    (((((&raw mut sPartyMenuBoxes).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(((i) as i32) as isize * 16))
                    .wrapping_add(9))
                    .write(255u8);
                    (((((&raw mut sPartyMenuBoxes).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(((i) as i32) as isize * 16))
                    .wrapping_add(10))
                    .write(255u8);
                    (((((&raw mut sPartyMenuBoxes).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(((i) as i32) as isize * 16))
                    .wrapping_add(11))
                    .write(255u8);
                    (((((&raw mut sPartyMenuBoxes).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(((i) as i32) as isize * 16))
                    .wrapping_add(12))
                    .write(255u8);
                }
                i = (i).wrapping_add(1);
            }
        }
        ((((&raw mut sPartyMenuBoxes).cast::<u8>().cast::<*mut u8>()).read()).cast::<*mut u8>())
            .write(((&raw const sPartyBoxInfoRects).cast::<u8>().cast_mut()).cast::<u8>());
        if ((layout) as i32) == 3i32 {
            (((((&raw mut sPartyMenuBoxes).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_offset(48))
            .cast::<*mut u8>())
            .write(((&raw const sPartyBoxInfoRects).cast::<u8>().cast_mut()).cast::<u8>());
        } else {
            if ((layout) as i32) != 0i32 {
                (((((&raw mut sPartyMenuBoxes).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_offset(16))
                .cast::<*mut u8>())
                .write(((&raw const sPartyBoxInfoRects).cast::<u8>().cast_mut()).cast::<u8>());
            }
        }
    }
}
pub(crate) unsafe extern "C" fn RenderPartyMenuBox(slot: u8) {
    unsafe {
        let mut slot = slot;
        if (((crate::c::bf_read(
            ((&raw mut gPartyMenu).cast::<u8>()).wrapping_add(8),
            0,
            4,
            false,
        ) as u8) as i32)
            == 5i32)
            && (((slot) as i32) >= crate::c::div_i32(6i32, 2i32))
        {
            DisplayPartyPokemonDataForMultiBattle(slot);
            if ((((((&raw mut gMultiPartnerParty).cast::<u8>()).wrapping_offset(
                (((slot) as i32).wrapping_sub(crate::c::div_i32(6i32, 2i32))) as isize * 32,
            ))
            .cast::<u16>())
            .read()) as i32)
                == 0i32
            {
                LoadPartyBoxPalette(
                    (((&raw mut sPartyMenuBoxes).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(((slot) as i32) as isize * 16),
                    64u8,
                );
            } else {
                LoadPartyBoxPalette(
                    (((&raw mut sPartyMenuBoxes).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(((slot) as i32) as isize * 16),
                    8u8,
                );
            }
            CopyWindowToVram(
                (((((&raw mut sPartyMenuBoxes).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_offset(((slot) as i32) as isize * 16))
                .wrapping_add(8))
                .read(),
                2u8,
            );
            PutWindowTilemap(
                (((((&raw mut sPartyMenuBoxes).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_offset(((slot) as i32) as isize * 16))
                .wrapping_add(8))
                .read(),
            );
            ScheduleBgCopyTilemapToVram(2u8);
        } else {
            if GetMonData2(
                ((&raw mut gPlayerParty).cast::<u8>())
                    .wrapping_offset(((slot) as i32) as isize * 100),
                11i32,
            ) == 0u32
            {
                DrawEmptySlot(
                    (((((&raw mut sPartyMenuBoxes).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(((slot) as i32) as isize * 16))
                    .wrapping_add(8))
                    .read(),
                );
                LoadPartyBoxPalette(
                    (((&raw mut sPartyMenuBoxes).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(((slot) as i32) as isize * 16),
                    64u8,
                );
                CopyWindowToVram(
                    (((((&raw mut sPartyMenuBoxes).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(((slot) as i32) as isize * 16))
                    .wrapping_add(8))
                    .read(),
                    2u8,
                );
            } else {
                if ((crate::c::bf_read(
                    ((&raw mut gPartyMenu).cast::<u8>()).wrapping_add(8),
                    0,
                    4,
                    false,
                ) as u8) as i32)
                    == 7i32
                {
                    DisplayPartyPokemonDataForRelearner(slot);
                } else {
                    if ((crate::c::bf_read(
                        ((&raw mut gPartyMenu).cast::<u8>()).wrapping_add(8),
                        0,
                        4,
                        false,
                    ) as u8) as i32)
                        == 2i32
                    {
                        DisplayPartyPokemonDataForContest(slot);
                    } else {
                        if ((crate::c::bf_read(
                            ((&raw mut gPartyMenu).cast::<u8>()).wrapping_add(8),
                            0,
                            4,
                            false,
                        ) as u8) as i32)
                            == 4i32
                        {
                            DisplayPartyPokemonDataForChooseHalf(slot);
                        } else {
                            if ((crate::c::bf_read(
                                ((&raw mut gPartyMenu).cast::<u8>()).wrapping_add(8),
                                0,
                                4,
                                false,
                            ) as u8) as i32)
                                == 11i32
                            {
                                DisplayPartyPokemonDataForWirelessMinigame(slot);
                            } else {
                                if ((crate::c::bf_read(
                                    ((&raw mut gPartyMenu).cast::<u8>()).wrapping_add(8),
                                    0,
                                    4,
                                    false,
                                ) as u8) as i32)
                                    == 12i32
                                {
                                    DisplayPartyPokemonDataForBattlePyramidHeldItem(slot);
                                } else {
                                    if !((DisplayPartyPokemonDataForMoveTutorOrEvolutionItem(slot))
                                        != 0)
                                    {
                                        DisplayPartyPokemonData(slot);
                                    }
                                }
                            }
                        }
                    }
                }
                if ((crate::c::bf_read(
                    ((&raw mut gPartyMenu).cast::<u8>()).wrapping_add(8),
                    0,
                    4,
                    false,
                ) as u8) as i32)
                    == 5i32
                {
                    AnimatePartySlot(slot, 0u8);
                } else {
                    if (((((&raw mut gPartyMenu).cast::<u8>())
                        .wrapping_add(9)
                        .cast::<i8>())
                    .read()) as i32)
                        == ((slot) as i32)
                    {
                        AnimatePartySlot(slot, 1u8);
                    } else {
                        AnimatePartySlot(slot, 0u8);
                    }
                }
            }
            PutWindowTilemap(
                (((((&raw mut sPartyMenuBoxes).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_offset(((slot) as i32) as isize * 16))
                .wrapping_add(8))
                .read(),
            );
            ScheduleBgCopyTilemapToVram(0u8);
        }
    }
}
pub(crate) unsafe extern "C" fn DisplayPartyPokemonData(slot: u8) {
    unsafe {
        let mut slot = slot;
        if (GetMonData2(
            ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(((slot) as i32) as isize * 100),
            45i32,
        )) != 0
        {
            ((((((((&raw mut sPartyMenuBoxes).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_offset(((slot) as i32) as isize * 16))
            .cast::<*mut u8>())
            .read())
            .cast::<Option<unsafe extern "C" fn(u8, u8, u8, u8, u8, u8)>>())
            .read())
            .unwrap_unchecked()(
                (((((&raw mut sPartyMenuBoxes).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_offset(((slot) as i32) as isize * 16))
                .wrapping_add(8))
                .read(),
                0u8,
                0u8,
                0u8,
                0u8,
                1u8,
            );
            DisplayPartyPokemonNickname(
                ((&raw mut gPlayerParty).cast::<u8>())
                    .wrapping_offset(((slot) as i32) as isize * 100),
                (((&raw mut sPartyMenuBoxes).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_offset(((slot) as i32) as isize * 16),
                0u8,
            );
        } else {
            ((((((((&raw mut sPartyMenuBoxes).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_offset(((slot) as i32) as isize * 16))
            .cast::<*mut u8>())
            .read())
            .cast::<Option<unsafe extern "C" fn(u8, u8, u8, u8, u8, u8)>>())
            .read())
            .unwrap_unchecked()(
                (((((&raw mut sPartyMenuBoxes).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_offset(((slot) as i32) as isize * 16))
                .wrapping_add(8))
                .read(),
                0u8,
                0u8,
                0u8,
                0u8,
                0u8,
            );
            DisplayPartyPokemonNickname(
                ((&raw mut gPlayerParty).cast::<u8>())
                    .wrapping_offset(((slot) as i32) as isize * 100),
                (((&raw mut sPartyMenuBoxes).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_offset(((slot) as i32) as isize * 16),
                0u8,
            );
            DisplayPartyPokemonLevelCheck(
                ((&raw mut gPlayerParty).cast::<u8>())
                    .wrapping_offset(((slot) as i32) as isize * 100),
                (((&raw mut sPartyMenuBoxes).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_offset(((slot) as i32) as isize * 16),
                0u8,
            );
            DisplayPartyPokemonGenderNidoranCheck(
                ((&raw mut gPlayerParty).cast::<u8>())
                    .wrapping_offset(((slot) as i32) as isize * 100),
                (((&raw mut sPartyMenuBoxes).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_offset(((slot) as i32) as isize * 16),
                0u8,
            );
            DisplayPartyPokemonHPCheck(
                ((&raw mut gPlayerParty).cast::<u8>())
                    .wrapping_offset(((slot) as i32) as isize * 100),
                (((&raw mut sPartyMenuBoxes).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_offset(((slot) as i32) as isize * 16),
                0u8,
            );
            DisplayPartyPokemonMaxHPCheck(
                ((&raw mut gPlayerParty).cast::<u8>())
                    .wrapping_offset(((slot) as i32) as isize * 100),
                (((&raw mut sPartyMenuBoxes).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_offset(((slot) as i32) as isize * 16),
                0u8,
            );
            DisplayPartyPokemonHPBarCheck(
                ((&raw mut gPlayerParty).cast::<u8>())
                    .wrapping_offset(((slot) as i32) as isize * 100),
                (((&raw mut sPartyMenuBoxes).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_offset(((slot) as i32) as isize * 16),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn DisplayPartyPokemonDescriptionData(slot: u8, stringID: u8) {
    unsafe {
        let mut slot = slot;
        let mut stringID = stringID;
        let mut mon: *mut u8 =
            ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(((slot) as i32) as isize * 100);
        ((((((((&raw mut sPartyMenuBoxes).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_offset(((slot) as i32) as isize * 16))
        .cast::<*mut u8>())
        .read())
        .cast::<Option<unsafe extern "C" fn(u8, u8, u8, u8, u8, u8)>>())
        .read())
        .unwrap_unchecked()(
            (((((&raw mut sPartyMenuBoxes).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_offset(((slot) as i32) as isize * 16))
            .wrapping_add(8))
            .read(),
            0u8,
            0u8,
            0u8,
            0u8,
            1u8,
        );
        DisplayPartyPokemonNickname(
            mon,
            (((&raw mut sPartyMenuBoxes).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_offset(((slot) as i32) as isize * 16),
            0u8,
        );
        if !((GetMonData2(mon, 45i32)) != 0) {
            DisplayPartyPokemonLevelCheck(
                mon,
                (((&raw mut sPartyMenuBoxes).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_offset(((slot) as i32) as isize * 16),
                0u8,
            );
            DisplayPartyPokemonGenderNidoranCheck(
                mon,
                (((&raw mut sPartyMenuBoxes).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_offset(((slot) as i32) as isize * 16),
                0u8,
            );
        }
        DisplayPartyPokemonDescriptionText(
            stringID,
            (((&raw mut sPartyMenuBoxes).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_offset(((slot) as i32) as isize * 16),
            0u8,
        );
    }
}
pub(crate) unsafe extern "C" fn DisplayPartyPokemonDataForChooseHalf(slot: u8) {
    unsafe {
        let mut slot = slot;
        let mut i: u8 = 0u8;
        let mut mon: *mut u8 =
            ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(((slot) as i32) as isize * 100);
        let mut order: *mut u8 = ((&raw mut gSelectedOrderFromParty).cast::<u8>()).cast::<u8>();
        if !((GetBattleEntryEligibility(mon)) != 0) {
            DisplayPartyPokemonDescriptionData(slot, 7u8);
            return;
        } else {
            {
                i = 0u8;
                'l1: loop {
                    if !(((i) as i32) < ((GetMaxBattleEntries()) as i32)) {
                        break 'l1;
                    }
                    'l2: {
                        if (((((order).wrapping_offset(((i) as i32) as isize)).read()) as i32)
                            != 0i32)
                            && (((((order).wrapping_offset(((i) as i32) as isize)).read()) as i32)
                                .wrapping_sub(1i32)
                                == ((slot) as i32))
                        {
                            DisplayPartyPokemonDescriptionData(
                                slot,
                                ((((i) as i32).wrapping_add(2i32)) as u8),
                            );
                            return;
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
            DisplayPartyPokemonDescriptionData(slot, 1u8);
        }
    }
}
pub(crate) unsafe extern "C" fn DisplayPartyPokemonDataForContest(slot: u8) {
    unsafe {
        let mut slot = slot;
        'l1: {
            let __sw1 = ((GetContestEntryEligibility(
                ((&raw mut gPlayerParty).cast::<u8>())
                    .wrapping_offset(((slot) as i32) as isize * 100),
            )) as i32);
            if __sw1 == 0i32 || __sw1 == 3i32 || __sw1 == 4i32 {
                DisplayPartyPokemonDescriptionData(slot, 7u8);
                break 'l1;
            }
            if __sw1 == 1i32 || __sw1 == 2i32 {
                DisplayPartyPokemonDescriptionData(slot, 6u8);
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn DisplayPartyPokemonDataForRelearner(slot: u8) {
    unsafe {
        let mut slot = slot;
        if ((GetNumberOfRelearnableMoves(
            ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(((slot) as i32) as isize * 100),
        )) as i32)
            == 0i32
        {
            DisplayPartyPokemonDescriptionData(slot, 9u8);
        } else {
            DisplayPartyPokemonDescriptionData(slot, 8u8);
        }
    }
}
pub(crate) unsafe extern "C" fn DisplayPartyPokemonDataForWirelessMinigame(slot: u8) {
    unsafe {
        let mut slot = slot;
        if ((IsMonAllowedInMinigame(slot)) as i32) == 1i32 {
            DisplayPartyPokemonDescriptionData(slot, 6u8);
        } else {
            DisplayPartyPokemonDescriptionData(slot, 7u8);
        }
    }
}
pub(crate) unsafe extern "C" fn DisplayPartyPokemonDataForBattlePyramidHeldItem(slot: u8) {
    unsafe {
        let mut slot = slot;
        if (GetMonData2(
            ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(((slot) as i32) as isize * 100),
            12i32,
        )) != 0
        {
            DisplayPartyPokemonDescriptionData(slot, 11u8);
        } else {
            DisplayPartyPokemonDescriptionData(slot, 12u8);
        }
    }
}
pub(crate) unsafe extern "C" fn DisplayPartyPokemonDataForMoveTutorOrEvolutionItem(slot: u8) -> u8 {
    unsafe {
        let mut slot = slot;
        let mut currentPokemon: *mut u8 =
            ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(((slot) as i32) as isize * 100);
        let mut item: u16 = ((&raw mut gSpecialVar_ItemId).cast::<u16>()).read();
        if (((((&raw mut gPartyMenu).cast::<u8>()).wrapping_add(11)).read()) as i32) == 12i32 {
            ((&raw mut gSpecialVar_Result).cast::<u16>()).write(0u16);
            DisplayPartyPokemonDataToTeachMove(
                slot,
                0u16,
                ((((&raw mut gSpecialVar_0x8005).cast::<u16>()).read()) as u8),
            );
        } else {
            if (((((&raw mut gPartyMenu).cast::<u8>()).wrapping_add(11)).read()) as i32) != 3i32 {
                return 0u8;
            }
            'l1: {
                let __sw1 = ((CheckIfItemIsTMHMOrEvolutionStone(item)) as i32);
                let __matched = __sw1 == 1i32 || __sw1 == 2i32;
                if !__matched {
                    return 0u8;
                }
                if __sw1 == 1i32 {
                    DisplayPartyPokemonDataToTeachMove(slot, item, 0u8);
                    break 'l1;
                }
                if __sw1 == 2i32 {
                    if (!((GetMonData2(currentPokemon, 45i32)) != 0))
                        && (((GetEvolutionTargetSpecies(currentPokemon, 3u8, item)) as i32) != 0i32)
                    {
                        return 0u8;
                    }
                    DisplayPartyPokemonDescriptionData(slot, 0u8);
                    break 'l1;
                }
            }
        }
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn DisplayPartyPokemonDataToTeachMove(slot: u8, item: u16, tutor: u8) {
    unsafe {
        let mut slot = slot;
        let mut item = item;
        let mut tutor = tutor;
        'l1: {
            let __sw1 = ((CanMonLearnTMTutor(
                ((&raw mut gPlayerParty).cast::<u8>())
                    .wrapping_offset(((slot) as i32) as isize * 100),
                item,
                tutor,
            )) as i32);
            let __matched = __sw1 == 1i32 || __sw1 == 3i32 || __sw1 == 2i32;
            if __sw1 == 1i32 || __sw1 == 3i32 {
                DisplayPartyPokemonDescriptionData(slot, 9u8);
                break 'l1;
            }
            if __sw1 == 2i32 {
                DisplayPartyPokemonDescriptionData(slot, 10u8);
                break 'l1;
            }
            if !__matched {
                DisplayPartyPokemonDescriptionData(slot, 8u8);
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn DisplayPartyPokemonDataForMultiBattle(slot: u8) {
    unsafe {
        let mut slot = slot;
        let mut menuBox: *mut u8 = (((&raw mut sPartyMenuBoxes).cast::<u8>().cast::<*mut u8>())
            .read())
        .wrapping_offset(((slot) as i32) as isize * 16);
        let mut actualSlot: u8 =
            ((((slot) as i32).wrapping_sub(crate::c::div_i32(6i32, 2i32))) as u8);
        if ((((((&raw mut gMultiPartnerParty).cast::<u8>())
            .wrapping_offset(((actualSlot) as i32) as isize * 32))
        .cast::<u16>())
        .read()) as i32)
            == 0i32
        {
            DrawEmptySlot(((menuBox).wrapping_add(8)).read());
        } else {
            (((((menuBox).cast::<*mut u8>()).read())
                .cast::<Option<unsafe extern "C" fn(u8, u8, u8, u8, u8, u8)>>())
            .read())
            .unwrap_unchecked()(
                ((menuBox).wrapping_add(8)).read(), 0u8, 0u8, 0u8, 0u8, 0u8
            );
            StringCopy(
                (&raw mut gStringVar1).cast::<u8>(),
                ((((&raw mut gMultiPartnerParty).cast::<u8>())
                    .wrapping_offset(((actualSlot) as i32) as isize * 32))
                .wrapping_add(4))
                .cast::<u8>(),
            );
            StringGet_Nickname((&raw mut gStringVar1).cast::<u8>());
            ConvertInternationalPlayerName((&raw mut gStringVar1).cast::<u8>());
            DisplayPartyPokemonBarDetail(
                ((menuBox).wrapping_add(8)).read(),
                (&raw mut gStringVar1).cast::<u8>(),
                0u8,
                ((((menuBox).cast::<*mut u8>()).read()).wrapping_add(4)).cast::<u8>(),
            );
            DisplayPartyPokemonLevel(
                ((((&raw mut gMultiPartnerParty).cast::<u8>())
                    .wrapping_offset(((actualSlot) as i32) as isize * 32))
                .wrapping_add(15))
                .read(),
                menuBox,
            );
            DisplayPartyPokemonGender(
                ((((&raw mut gMultiPartnerParty).cast::<u8>())
                    .wrapping_offset(((actualSlot) as i32) as isize * 32))
                .wrapping_add(28))
                .read(),
                ((((&raw mut gMultiPartnerParty).cast::<u8>())
                    .wrapping_offset(((actualSlot) as i32) as isize * 32))
                .cast::<u16>())
                .read(),
                ((((&raw mut gMultiPartnerParty).cast::<u8>())
                    .wrapping_offset(((actualSlot) as i32) as isize * 32))
                .wrapping_add(4))
                .cast::<u8>(),
                menuBox,
            );
            DisplayPartyPokemonHP(
                ((((&raw mut gMultiPartnerParty).cast::<u8>())
                    .wrapping_offset(((actualSlot) as i32) as isize * 32))
                .wrapping_add(16)
                .cast::<u16>())
                .read(),
                menuBox,
            );
            DisplayPartyPokemonMaxHP(
                ((((&raw mut gMultiPartnerParty).cast::<u8>())
                    .wrapping_offset(((actualSlot) as i32) as isize * 32))
                .wrapping_add(18)
                .cast::<u16>())
                .read(),
                menuBox,
            );
            DisplayPartyPokemonHPBar(
                ((((&raw mut gMultiPartnerParty).cast::<u8>())
                    .wrapping_offset(((actualSlot) as i32) as isize * 32))
                .wrapping_add(16)
                .cast::<u16>())
                .read(),
                ((((&raw mut gMultiPartnerParty).cast::<u8>())
                    .wrapping_offset(((actualSlot) as i32) as isize * 32))
                .wrapping_add(18)
                .cast::<u16>())
                .read(),
                menuBox,
            );
        }
    }
}
pub(crate) unsafe extern "C" fn RenderPartyMenuBoxes() -> u8 {
    unsafe {
        RenderPartyMenuBox(
            (((((((&raw mut sPartyMenuInternal).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(536))
            .cast::<i16>())
            .read()) as u8),
        );
        if (({
            let __p1 = ((((&raw mut sPartyMenuInternal).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(536))
            .cast::<i16>();
            let __t2 = ((__p1).read()).wrapping_add(1);
            (__p1).write(__t2);
            __t2
        }) as i32)
            == 6i32
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
pub(crate) unsafe extern "C" fn GetPartyMenuBgTile(tileId: u16) -> *mut u8 {
    unsafe {
        let mut tileId = tileId;
        return (((&raw mut sPartyBgGfxTilemap).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_offset((((tileId) as i32) << 5) as isize);
    }
}
pub(crate) unsafe extern "C" fn CreatePartyMonSprites(slot: u8) {
    unsafe {
        let mut slot = slot;
        let mut actualSlot: u8 = 0u8;
        if (((crate::c::bf_read(
            ((&raw mut gPartyMenu).cast::<u8>()).wrapping_add(8),
            0,
            4,
            false,
        ) as u8) as i32)
            == 5i32)
            && (((slot) as i32) >= crate::c::div_i32(6i32, 2i32))
        {
            let mut status: u8 = 0u8;
            actualSlot = ((((slot) as i32).wrapping_sub(crate::c::div_i32(6i32, 2i32))) as u8);
            if ((((((&raw mut gMultiPartnerParty).cast::<u8>())
                .wrapping_offset(((actualSlot) as i32) as isize * 32))
            .cast::<u16>())
            .read()) as i32)
                != 0i32
            {
                CreatePartyMonIconSpriteParameterized(
                    ((((&raw mut gMultiPartnerParty).cast::<u8>())
                        .wrapping_offset(((actualSlot) as i32) as isize * 32))
                    .cast::<u16>())
                    .read(),
                    ((((&raw mut gMultiPartnerParty).cast::<u8>())
                        .wrapping_offset(((actualSlot) as i32) as isize * 32))
                    .wrapping_add(24)
                    .cast::<u32>())
                    .read(),
                    (((&raw mut sPartyMenuBoxes).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(((slot) as i32) as isize * 16),
                    0u8,
                    0u32,
                );
                CreatePartyMonHeldItemSpriteParameterized(
                    ((((&raw mut gMultiPartnerParty).cast::<u8>())
                        .wrapping_offset(((actualSlot) as i32) as isize * 32))
                    .cast::<u16>())
                    .read(),
                    ((((&raw mut gMultiPartnerParty).cast::<u8>())
                        .wrapping_offset(((actualSlot) as i32) as isize * 32))
                    .wrapping_add(2)
                    .cast::<u16>())
                    .read(),
                    (((&raw mut sPartyMenuBoxes).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(((slot) as i32) as isize * 16),
                );
                CreatePartyMonPokeballSpriteParameterized(
                    ((((&raw mut gMultiPartnerParty).cast::<u8>())
                        .wrapping_offset(((actualSlot) as i32) as isize * 32))
                    .cast::<u16>())
                    .read(),
                    (((&raw mut sPartyMenuBoxes).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(((slot) as i32) as isize * 16),
                );
                if ((((((&raw mut gMultiPartnerParty).cast::<u8>())
                    .wrapping_offset(((actualSlot) as i32) as isize * 32))
                .wrapping_add(16)
                .cast::<u16>())
                .read()) as i32)
                    == 0i32
                {
                    status = 7u8;
                } else {
                    status = GetAilmentFromStatus(
                        ((((&raw mut gMultiPartnerParty).cast::<u8>())
                            .wrapping_offset(((actualSlot) as i32) as isize * 32))
                        .wrapping_add(20)
                        .cast::<u32>())
                        .read(),
                    );
                }
                CreatePartyMonStatusSpriteParameterized(
                    ((((&raw mut gMultiPartnerParty).cast::<u8>())
                        .wrapping_offset(((actualSlot) as i32) as isize * 32))
                    .cast::<u16>())
                    .read(),
                    status,
                    (((&raw mut sPartyMenuBoxes).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(((slot) as i32) as isize * 16),
                );
            }
        } else {
            if GetMonData2(
                ((&raw mut gPlayerParty).cast::<u8>())
                    .wrapping_offset(((slot) as i32) as isize * 100),
                11i32,
            ) != 0u32
            {
                CreatePartyMonIconSprite(
                    ((&raw mut gPlayerParty).cast::<u8>())
                        .wrapping_offset(((slot) as i32) as isize * 100),
                    (((&raw mut sPartyMenuBoxes).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(((slot) as i32) as isize * 16),
                    ((slot) as u32),
                );
                CreatePartyMonHeldItemSprite(
                    ((&raw mut gPlayerParty).cast::<u8>())
                        .wrapping_offset(((slot) as i32) as isize * 100),
                    (((&raw mut sPartyMenuBoxes).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(((slot) as i32) as isize * 16),
                );
                CreatePartyMonPokeballSprite(
                    ((&raw mut gPlayerParty).cast::<u8>())
                        .wrapping_offset(((slot) as i32) as isize * 100),
                    (((&raw mut sPartyMenuBoxes).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(((slot) as i32) as isize * 16),
                );
                CreatePartyMonStatusSprite(
                    ((&raw mut gPlayerParty).cast::<u8>())
                        .wrapping_offset(((slot) as i32) as isize * 100),
                    (((&raw mut sPartyMenuBoxes).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(((slot) as i32) as isize * 16),
                );
            }
        }
    }
}
pub(crate) unsafe extern "C" fn CreatePartyMonSpritesLoop() -> u8 {
    unsafe {
        CreatePartyMonSprites(
            (((((((&raw mut sPartyMenuInternal).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(536))
            .cast::<i16>())
            .read()) as u8),
        );
        if (({
            let __p1 = ((((&raw mut sPartyMenuInternal).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(536))
            .cast::<i16>();
            let __t2 = ((__p1).read()).wrapping_add(1);
            (__p1).write(__t2);
            __t2
        }) as i32)
            == 6i32
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
pub(crate) unsafe extern "C" fn CreateCancelConfirmPokeballSprites() {
    unsafe {
        if ((crate::c::bf_read(
            ((&raw mut gPartyMenu).cast::<u8>()).wrapping_add(8),
            0,
            4,
            false,
        ) as u8) as i32)
            == 5i32
        {
            FillBgTilemapBufferRect(1u8, 14u16, 23u8, 17u8, 7u8, 2u8, 1u8);
        } else {
            if (crate::c::bf_read(
                (((&raw mut sPartyMenuInternal).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(8),
                0,
                1,
                false,
            ) as u32)
                != 0
            {
                crate::c::bf_write(
                    (((&raw mut sPartyMenuInternal).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(8),
                    4,
                    7,
                    ((CreateSmallPokeballButtonSprite(191u8, 136u8)) as u32) as i32,
                );
                DrawCancelConfirmButtons();
                crate::c::bf_write(
                    (((&raw mut sPartyMenuInternal).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(9),
                    3,
                    7,
                    ((CreateSmallPokeballButtonSprite(191u8, 152u8)) as u32) as i32,
                );
            } else {
                crate::c::bf_write(
                    (((&raw mut sPartyMenuInternal).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(9),
                    3,
                    7,
                    ((CreatePokeballButtonSprite(198u8, 148u8)) as u32) as i32,
                );
            }
            AnimatePartySlot(
                (((((&raw mut gPartyMenu).cast::<u8>())
                    .wrapping_add(9)
                    .cast::<i8>())
                .read()) as u8),
                1u8,
            );
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimatePartySlot(slot: u8, animNum: u8) {
    unsafe {
        let mut slot = slot;
        let mut animNum = animNum;
        let mut spriteId: u8 = 0u8;
        'l1: {
            let __sw1 = ((slot) as i32);
            let __matched = __sw1 == 6i32 || __sw1 == 7i32;
            if !__matched {
                if GetMonData2(
                    ((&raw mut gPlayerParty).cast::<u8>())
                        .wrapping_offset(((slot) as i32) as isize * 100),
                    11i32,
                ) != 0u32
                {
                    LoadPartyBoxPalette(
                        (((&raw mut sPartyMenuBoxes).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_offset(((slot) as i32) as isize * 16),
                        GetPartyBoxPaletteFlags(slot, animNum),
                    );
                    AnimateSelectedPartyIcon(
                        (((((&raw mut sPartyMenuBoxes).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_offset(((slot) as i32) as isize * 16))
                        .wrapping_add(9))
                        .read(),
                        animNum,
                    );
                    PartyMenuStartSpriteAnim(
                        (((((&raw mut sPartyMenuBoxes).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_offset(((slot) as i32) as isize * 16))
                        .wrapping_add(11))
                        .read(),
                        animNum,
                    );
                }
                return;
            }
            if __sw1 == 6i32 {
                if ((animNum) as i32) == 0i32 {
                    SetBgTilemapPalette(1u8, 23u8, 16u8, 7u8, 2u8, 1u8);
                } else {
                    SetBgTilemapPalette(1u8, 23u8, 16u8, 7u8, 2u8, 2u8);
                }
                spriteId = ((crate::c::bf_read(
                    (((&raw mut sPartyMenuInternal).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(8),
                    4,
                    7,
                    false,
                ) as u32) as u8);
                break 'l1;
            }
            if __sw1 == 7i32 {
                if !((crate::c::bf_read(
                    (((&raw mut sPartyMenuInternal).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(8),
                    0,
                    1,
                    false,
                ) as u32)
                    != 0)
                {
                    if ((animNum) as i32) == 0i32 {
                        SetBgTilemapPalette(1u8, 23u8, 17u8, 7u8, 2u8, 1u8);
                    } else {
                        SetBgTilemapPalette(1u8, 23u8, 17u8, 7u8, 2u8, 2u8);
                    }
                } else {
                    if ((animNum) as i32) == 0i32 {
                        SetBgTilemapPalette(1u8, 23u8, 18u8, 7u8, 2u8, 1u8);
                    } else {
                        SetBgTilemapPalette(1u8, 23u8, 18u8, 7u8, 2u8, 2u8);
                    }
                }
                spriteId = ((crate::c::bf_read(
                    (((&raw mut sPartyMenuInternal).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(9),
                    3,
                    7,
                    false,
                ) as u32) as u8);
                break 'l1;
            }
        }
        PartyMenuStartSpriteAnim(spriteId, animNum);
        ScheduleBgCopyTilemapToVram(1u8);
    }
}
pub(crate) unsafe extern "C" fn GetPartyBoxPaletteFlags(slot: u8, animNum: u8) -> u8 {
    unsafe {
        let mut slot = slot;
        let mut animNum = animNum;
        let mut palFlags: u8 = 0u8;
        if ((animNum) as i32) == 1i32 {
            palFlags = ((((palFlags) as i32) | 1i32) as u8);
        }
        if GetMonData2(
            ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(((slot) as i32) as isize * 100),
            57i32,
        ) == 0u32
        {
            palFlags = ((((palFlags) as i32) | 2i32) as u8);
        }
        if ((PartyBoxPal_ParnterOrDisqualifiedInArena(slot)) as i32) == 1i32 {
            palFlags = ((((palFlags) as i32) | 8i32) as u8);
        }
        if (((((&raw mut gPartyMenu).cast::<u8>()).wrapping_add(11)).read()) as i32) == 9i32 {
            palFlags = ((((palFlags) as i32) | 16i32) as u8);
        }
        if (((((&raw mut gPartyMenu).cast::<u8>()).wrapping_add(11)).read()) as i32) == 8i32 {
            if (((slot) as i32)
                == (((((&raw mut gPartyMenu).cast::<u8>())
                    .wrapping_add(9)
                    .cast::<i8>())
                .read()) as i32))
                || (((slot) as i32)
                    == (((((&raw mut gPartyMenu).cast::<u8>())
                        .wrapping_add(10)
                        .cast::<i8>())
                    .read()) as i32))
            {
                palFlags = ((((palFlags) as i32) | 4i32) as u8);
            }
        }
        if ((((((&raw mut gPartyMenu).cast::<u8>()).wrapping_add(11)).read()) as i32) == 10i32)
            && (((slot) as i32)
                == (((((&raw mut gPartyMenu).cast::<u8>())
                    .wrapping_add(9)
                    .cast::<i8>())
                .read()) as i32))
        {
            palFlags = ((((palFlags) as i32) | 32i32) as u8);
        }
        return palFlags;
    }
}
pub(crate) unsafe extern "C" fn PartyBoxPal_ParnterOrDisqualifiedInArena(slot: u8) -> u8 {
    unsafe {
        let mut slot = slot;
        if (((crate::c::bf_read(
            ((&raw mut gPartyMenu).cast::<u8>()).wrapping_add(8),
            4,
            2,
            false,
        ) as u8) as i32)
            == 2i32)
            && (((((slot) as i32) == 1i32) || (((slot) as i32) == 4i32))
                || (((slot) as i32) == 5i32))
        {
            return 1u8;
        }
        if (((((slot) as i32) < crate::c::div_i32(6i32, 2i32))
            && ((((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 262144u32) != 0))
            && ((crate::c::bf_read(
                ((&raw mut gMain).cast::<u8>()).wrapping_add(1081),
                1,
                1,
                false,
            ) as u8)
                != 0))
            && ((crate::c::shr_i32(
                ((((((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(672)).read())
                    as i32),
                ((GetPartyIdFromBattleSlot(slot)) as u32),
            ) & 1i32)
                != 0)
        {
            return 1u8;
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn DrawCancelConfirmButtons() {
    unsafe {
        CopyToBgTilemapBufferRect_ChangePalette(
            1u8,
            (((&raw const sConfirmButton_Tilemap)
                .cast::<u8>()
                .cast_mut()
                .cast::<u32>())
            .cast::<u32>())
            .cast::<u8>(),
            23u8,
            16u8,
            7u8,
            2u8,
            17u8,
        );
        CopyToBgTilemapBufferRect_ChangePalette(
            1u8,
            (((&raw const sCancelButton_Tilemap)
                .cast::<u8>()
                .cast_mut()
                .cast::<u32>())
            .cast::<u32>())
            .cast::<u8>(),
            23u8,
            18u8,
            7u8,
            2u8,
            17u8,
        );
        ScheduleBgCopyTilemapToVram(1u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsMultiBattle() -> u8 {
    unsafe {
        if ((((((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 64u32) != 0)
            && ((((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 1u32) != 0))
            && ((((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 8u32) != 0))
            && ((crate::c::bf_read(
                ((&raw mut gMain).cast::<u8>()).wrapping_add(1081),
                1,
                1,
                false,
            ) as u8)
                != 0)
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
pub(crate) unsafe extern "C" fn SwapPartyPokemon(mon1: *mut u8, mon2: *mut u8) {
    unsafe {
        let mut mon1 = mon1;
        let mut mon2 = mon2;
        let mut temp: *mut u8 = Alloc(100u32);
        temp.cast::<crate::c::Rec4<100>>()
            .write_unaligned(mon1.cast::<crate::c::Rec4<100>>().read_unaligned());
        mon1.cast::<crate::c::Rec4<100>>()
            .write_unaligned(mon2.cast::<crate::c::Rec4<100>>().read_unaligned());
        mon2.cast::<crate::c::Rec4<100>>()
            .write_unaligned(temp.cast::<crate::c::Rec4<100>>().read_unaligned());
        Free(temp);
    }
}
pub(crate) unsafe extern "C" fn Task_ClosePartyMenu(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        BeginNormalPaletteFade(4294967295u32, 0i8, 0u8, 16u8, 0u16);
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Task_ClosePartyMenuAndSetCB2));
    }
}
pub(crate) unsafe extern "C" fn Task_ClosePartyMenuAndSetCB2(taskId: u8) {
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
            if ((crate::c::bf_read(
                ((&raw mut gPartyMenu).cast::<u8>()).wrapping_add(8),
                0,
                4,
                false,
            ) as u8) as i32)
                == 1i32
            {
                UpdatePartyToFieldOrder();
            }
            if core::mem::transmute::<_, usize>(
                ((((&raw mut sPartyMenuInternal).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(4)
                    .cast::<Option<unsafe extern "C" fn()>>())
                .read(),
            ) != 0usize
            {
                SetMainCallback2(
                    ((((&raw mut sPartyMenuInternal).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(4)
                        .cast::<Option<unsafe extern "C" fn()>>())
                    .read(),
                );
            } else {
                SetMainCallback2(
                    (((&raw mut gPartyMenu).cast::<u8>()).cast::<Option<unsafe extern "C" fn()>>())
                        .read(),
                );
            }
            ResetSpriteData();
            FreePartyPointers();
            DestroyTask(taskId);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetCursorSelectionMonId() -> u8 {
    unsafe {
        return (((((&raw mut gPartyMenu).cast::<u8>())
            .wrapping_add(9)
            .cast::<i8>())
        .read()) as u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetPartyMenuType() -> u8 {
    unsafe {
        return (crate::c::bf_read(
            ((&raw mut gPartyMenu).cast::<u8>()).wrapping_add(8),
            0,
            4,
            false,
        ) as u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Task_HandleChooseMonInput(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if (!((crate::c::bf_read(
            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
            7,
            1,
            false,
        ) as u16)
            != 0))
            && (((MenuHelpers_ShouldWaitForLinkRecv()) as i32) != 1i32)
        {
            let mut slotPtr: *mut i8 = GetCurrentPartySlotPtr();
            'l1: {
                let __sw1 = ((PartyMenuButtonHandler(slotPtr)) as i32);
                if __sw1 == 1i32 {
                    HandleChooseMonSelection(taskId, slotPtr);
                    break 'l1;
                }
                if __sw1 == 2i32 {
                    HandleChooseMonCancel(taskId, slotPtr);
                    break 'l1;
                }
                if __sw1 == 8i32 {
                    if (crate::c::bf_read(
                        (((&raw mut sPartyMenuInternal).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(8),
                        0,
                        1,
                        false,
                    ) as u32)
                        != 0
                    {
                        PlaySE(5u16);
                        MoveCursorToConfirm();
                    }
                    break 'l1;
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn GetCurrentPartySlotPtr() -> *mut i8 {
    unsafe {
        if ((((((&raw mut gPartyMenu).cast::<u8>()).wrapping_add(11)).read()) as i32) == 8i32)
            || ((((((&raw mut gPartyMenu).cast::<u8>()).wrapping_add(11)).read()) as i32) == 10i32)
        {
            return ((&raw mut gPartyMenu).cast::<u8>())
                .wrapping_add(10)
                .cast::<i8>();
        } else {
            return ((&raw mut gPartyMenu).cast::<u8>())
                .wrapping_add(9)
                .cast::<i8>();
        }
        #[allow(unreachable_code)]
        {
            return core::ptr::null_mut();
        }
    }
}
pub(crate) unsafe extern "C" fn HandleChooseMonSelection(taskId: u8, slotPtr: *mut i8) {
    unsafe {
        let mut taskId = taskId;
        let mut slotPtr = slotPtr;
        if (((slotPtr).read()) as i32) == 6i32 {
            ((((&raw mut gPartyMenu).cast::<u8>())
                .wrapping_add(4)
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .read())
            .unwrap_unchecked()(taskId);
        } else {
            'l1: {
                let __sw1 =
                    (((((&raw mut gPartyMenu).cast::<u8>()).wrapping_add(11)).read()) as i32);
                let __matched = __sw1 == 10i32
                    || __sw1 == 3i32
                    || __sw1 == 12i32
                    || __sw1 == 7i32
                    || __sw1 == 5i32
                    || __sw1 == 6i32
                    || __sw1 == 8i32
                    || __sw1 == 11i32
                    || __sw1 == 13i32
                    || __sw1 == 4i32
                    || __sw1 == 9i32;
                if __sw1 == 10i32 {
                    if (IsSelectedMonNotEgg((slotPtr).cast::<u8>())) != 0 {
                        PartyMenuRemoveWindow(
                            (((((&raw mut sPartyMenuInternal).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(12))
                            .cast::<u8>())
                            .wrapping_offset(1),
                        );
                        Task_TryUseSoftboiledOnPartyMon(taskId);
                    }
                    break 'l1;
                }
                if __sw1 == 3i32 {
                    if (IsSelectedMonNotEgg((slotPtr).cast::<u8>())) != 0 {
                        if ((crate::c::bf_read(
                            ((&raw mut gPartyMenu).cast::<u8>()).wrapping_add(8),
                            0,
                            4,
                            false,
                        ) as u8) as i32)
                            == 1i32
                        {
                            ((((&raw mut sPartyMenuInternal).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(4)
                            .cast::<Option<unsafe extern "C" fn()>>())
                            .write(Some(CB2_SetUpExitToBattleScreen));
                        }
                        PartyMenuRemoveWindow(
                            (((((&raw mut sPartyMenuInternal).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(12))
                            .cast::<u8>())
                            .wrapping_offset(1),
                        );
                        (((&raw mut gItemUseCB).cast::<u8>().cast::<Option<unsafe extern "C" fn(u8, Option<unsafe extern "C" fn(u8)>)>>()).read()).unwrap_unchecked()(taskId, Some(Task_ClosePartyMenuAfterText));
                    }
                    break 'l1;
                }
                if __sw1 == 12i32 {
                    if (IsSelectedMonNotEgg((slotPtr).cast::<u8>())) != 0 {
                        PlaySE(5u16);
                        PartyMenuRemoveWindow(
                            (((((&raw mut sPartyMenuInternal).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(12))
                            .cast::<u8>())
                            .wrapping_offset(1),
                        );
                        TryTutorSelectedMon(taskId);
                    }
                    break 'l1;
                }
                if __sw1 == 7i32 {
                    if (IsSelectedMonNotEgg((slotPtr).cast::<u8>())) != 0 {
                        PlaySE(5u16);
                        PartyMenuRemoveWindow(
                            (((((&raw mut sPartyMenuInternal).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(12))
                            .cast::<u8>())
                            .wrapping_offset(1),
                        );
                        TryGiveMailToSelectedMon(taskId);
                    }
                    break 'l1;
                }
                if __sw1 == 5i32 || __sw1 == 6i32 {
                    if (IsSelectedMonNotEgg((slotPtr).cast::<u8>())) != 0 {
                        PlaySE(5u16);
                        PartyMenuRemoveWindow(
                            (((((&raw mut sPartyMenuInternal).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(12))
                            .cast::<u8>())
                            .wrapping_offset(1),
                        );
                        TryGiveItemOrMailToSelectedMon(taskId);
                    }
                    break 'l1;
                }
                if __sw1 == 8i32 {
                    PlaySE(5u16);
                    SwitchSelectedMons(taskId);
                    break 'l1;
                }
                if __sw1 == 11i32 {
                    PlaySE(5u16);
                    Task_ClosePartyMenu(taskId);
                    break 'l1;
                }
                if __sw1 == 13i32 {
                    if (IsSelectedMonNotEgg((slotPtr).cast::<u8>())) != 0 {
                        TryEnterMonForMinigame(taskId, (((slotPtr).read()) as u8));
                    }
                    break 'l1;
                }
                if __sw1 == 4i32 || __sw1 == 9i32 || !__matched {
                    PlaySE(5u16);
                    Task_TryCreateSelectionWindow(taskId);
                    break 'l1;
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn IsSelectedMonNotEgg(slotPtr: *mut u8) -> u8 {
    unsafe {
        let mut slotPtr = slotPtr;
        if GetMonData2(
            ((&raw mut gPlayerParty).cast::<u8>())
                .wrapping_offset((((slotPtr).read()) as i32) as isize * 100),
            45i32,
        ) == 1u32
        {
            PlaySE(32u16);
            return 0u8;
        }
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn HandleChooseMonCancel(taskId: u8, slotPtr: *mut i8) {
    unsafe {
        let mut taskId = taskId;
        let mut slotPtr = slotPtr;
        'l1: {
            let __sw1 = (((((&raw mut gPartyMenu).cast::<u8>()).wrapping_add(11)).read()) as i32);
            let __matched = __sw1 == 1i32 || __sw1 == 8i32 || __sw1 == 10i32 || __sw1 == 13i32;
            if __sw1 == 1i32 {
                PlaySE(32u16);
                break 'l1;
            }
            if __sw1 == 8i32 || __sw1 == 10i32 {
                PlaySE(5u16);
                FinishTwoMonAction(taskId);
                break 'l1;
            }
            if __sw1 == 13i32 {
                PlaySE(5u16);
                CancelParticipationPrompt(taskId);
                break 'l1;
            }
            if !__matched {
                PlaySE(5u16);
                if ((DisplayCancelChooseMonYesNo(taskId)) as i32) != 1i32 {
                    if !((MenuHelpers_IsLinkActive()) != 0) {
                        ((&raw mut gSpecialVar_0x8004).cast::<u16>()).write(7u16);
                    }
                    ((&raw mut gPartyMenuUseExitCallback)
                        .cast::<u8>()
                        .cast::<u8>())
                    .write(0u8);
                    (slotPtr).write(7i8);
                    Task_ClosePartyMenu(taskId);
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn DisplayCancelChooseMonYesNo(taskId: u8) -> u8 {
    unsafe {
        let mut taskId = taskId;
        let mut stringPtr: *mut u8 = core::ptr::null_mut();
        if ((crate::c::bf_read(
            ((&raw mut gPartyMenu).cast::<u8>()).wrapping_add(8),
            0,
            4,
            false,
        ) as u8) as i32)
            == 2i32
        {
            stringPtr = (&raw mut gText_CancelParticipation).cast::<u8>();
        } else {
            if ((crate::c::bf_read(
                ((&raw mut gPartyMenu).cast::<u8>()).wrapping_add(8),
                0,
                4,
                false,
            ) as u8) as i32)
                == 4i32
            {
                stringPtr = GetFacilityCancelString();
            }
        }
        if ((stringPtr) as usize) == 0usize {
            return 0u8;
        }
        PartyMenuRemoveWindow(
            (((((&raw mut sPartyMenuInternal).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(12))
            .cast::<u8>())
            .wrapping_offset(1),
        );
        StringExpandPlaceholders((&raw mut gStringVar4).cast::<u8>(), stringPtr);
        DisplayPartyMenuMessage((&raw mut gStringVar4).cast::<u8>(), 1u8);
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Task_CancelChooseMonYesNo));
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn Task_CancelChooseMonYesNo(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if ((IsPartyMenuTextPrinterActive()) as i32) != 1i32 {
            PartyMenuDisplayYesNoMenu();
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_HandleCancelChooseMonYesNoInput));
        }
    }
}
pub(crate) unsafe extern "C" fn Task_HandleCancelChooseMonYesNoInput(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        'l1: {
            let __sw1 = ((Menu_ProcessInputNoWrapClearOnChoose()) as i32);
            let mut __fall = false;
            if __sw1 == 0i32 {
                __fall = true;
                ((&raw mut gPartyMenuUseExitCallback)
                    .cast::<u8>()
                    .cast::<u8>())
                .write(0u8);
                (((&raw mut gPartyMenu).cast::<u8>())
                    .wrapping_add(9)
                    .cast::<i8>())
                .write(7i8);
                ClearSelectedPartyOrder();
                Task_ClosePartyMenu(taskId);
                break 'l1;
            }
            if __sw1 == (-1i32) {
                __fall = true;
                PlaySE(5u16);
            }
            if __fall || __sw1 == 1i32 {
                __fall = true;
                Task_ReturnToChooseMonAfterText(taskId);
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn PartyMenuButtonHandler(slotPtr: *mut i8) -> u16 {
    unsafe {
        let mut slotPtr = slotPtr;
        let mut movementDir: i8 = 0i8;
        'l1: {
            let __sw1 = (((((&raw mut gMain).cast::<u8>())
                .wrapping_add(48)
                .cast::<u16>())
            .read()) as i32);
            let __matched = __sw1 == 64i32 || __sw1 == 128i32 || __sw1 == 32i32 || __sw1 == 16i32;
            if __sw1 == 64i32 {
                movementDir = (-1i8);
                break 'l1;
            }
            if __sw1 == 128i32 {
                movementDir = 1i8;
                break 'l1;
            }
            if __sw1 == 32i32 {
                movementDir = (-2i8);
                break 'l1;
            }
            if __sw1 == 16i32 {
                movementDir = 2i8;
                break 'l1;
            }
            if !__matched {
                'l2: {
                    let __sw2 = ((GetLRKeysPressedAndHeld()) as i32);
                    let __matched = __sw2 == 1i32 || __sw2 == 2i32;
                    if __sw2 == 1i32 {
                        movementDir = (-1i8);
                        break 'l2;
                    }
                    if __sw2 == 2i32 {
                        movementDir = 1i8;
                        break 'l2;
                    }
                    if !__matched {
                        movementDir = 0i8;
                        break 'l2;
                    }
                }
                break 'l1;
            }
        }
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(46)
            .cast::<u16>())
        .read()) as i32)
            & 8i32)
            != 0
        {
            return 8u16;
        }
        if (movementDir) != 0 {
            UpdateCurrentPartySelection(slotPtr, movementDir);
            return 0u16;
        }
        if (((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(46)
            .cast::<u16>())
        .read()) as i32)
            & 1i32)
            != 0)
            && ((((slotPtr).read()) as i32) == 7i32)
        {
            return 2u16;
        }
        return (((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(46)
            .cast::<u16>())
        .read()) as i32)
            & 3i32) as u16);
    }
}
pub(crate) unsafe extern "C" fn UpdateCurrentPartySelection(slotPtr: *mut i8, movementDir: i8) {
    unsafe {
        let mut slotPtr = slotPtr;
        let mut movementDir = movementDir;
        let mut newSlotId: i8 = (slotPtr).read();
        let mut layout: u8 = (crate::c::bf_read(
            ((&raw mut gPartyMenu).cast::<u8>()).wrapping_add(8),
            4,
            2,
            false,
        ) as u8);
        if ((layout) as i32) == 0i32 {
            UpdatePartySelectionSingleLayout(slotPtr, movementDir);
        } else {
            UpdatePartySelectionDoubleLayout(slotPtr, movementDir);
        }
        if (((slotPtr).read()) as i32) != ((newSlotId) as i32) {
            PlaySE(5u16);
            AnimatePartySlot(((newSlotId) as u8), 0u8);
            AnimatePartySlot((((slotPtr).read()) as u8), 1u8);
        }
    }
}
pub(crate) unsafe extern "C" fn UpdatePartySelectionSingleLayout(
    slotPtr: *mut i8,
    movementDir: i8,
) {
    unsafe {
        let mut slotPtr = slotPtr;
        let mut movementDir = movementDir;
        'l1: {
            let __sw1 = ((movementDir) as i32);
            if __sw1 == (-1i32) {
                if (((slotPtr).read()) as i32) == 0i32 {
                    (slotPtr).write(7i8);
                } else {
                    if (((slotPtr).read()) as i32) == 6i32 {
                        (slotPtr).write(
                            ((((((&raw mut gPlayerPartyCount).cast::<u8>()).read()) as i32)
                                .wrapping_sub(1i32)) as i8),
                        );
                    } else {
                        if (((slotPtr).read()) as i32) == 7i32 {
                            if (crate::c::bf_read(
                                (((&raw mut sPartyMenuInternal).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(8),
                                0,
                                1,
                                false,
                            ) as u32)
                                != 0
                            {
                                (slotPtr).write(6i8);
                            } else {
                                (slotPtr).write(
                                    ((((((&raw mut gPlayerPartyCount).cast::<u8>()).read()) as i32)
                                        .wrapping_sub(1i32))
                                        as i8),
                                );
                            }
                        } else {
                            (slotPtr).write(((slotPtr).read()).wrapping_sub(1));
                        }
                    }
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                if (((slotPtr).read()) as i32) == 7i32 {
                    (slotPtr).write(0i8);
                } else {
                    if (((slotPtr).read()) as i32)
                        == ((((&raw mut gPlayerPartyCount).cast::<u8>()).read()) as i32)
                            .wrapping_sub(1i32)
                    {
                        if (crate::c::bf_read(
                            (((&raw mut sPartyMenuInternal).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(8),
                            0,
                            1,
                            false,
                        ) as u32)
                            != 0
                        {
                            (slotPtr).write(6i8);
                        } else {
                            (slotPtr).write(7i8);
                        }
                    } else {
                        (slotPtr).write(((slotPtr).read()).wrapping_add(1));
                    }
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                if (((((&raw mut gPlayerPartyCount).cast::<u8>()).read()) as i32) != 1i32)
                    && ((((slotPtr).read()) as i32) == 0i32)
                {
                    if (crate::c::bf_read(
                        (((&raw mut sPartyMenuInternal).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(8),
                        1,
                        3,
                        false,
                    ) as u32)
                        == 0u32
                    {
                        (slotPtr).write(1i8);
                    } else {
                        (slotPtr).write(
                            ((crate::c::bf_read(
                                (((&raw mut sPartyMenuInternal).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(8),
                                1,
                                3,
                                false,
                            ) as u32) as i8),
                        );
                    }
                }
                break 'l1;
            }
            if __sw1 == (-2i32) {
                if (((((slotPtr).read()) as i32) != 0i32) && ((((slotPtr).read()) as i32) != 6i32))
                    && ((((slotPtr).read()) as i32) != 7i32)
                {
                    crate::c::bf_write(
                        (((&raw mut sPartyMenuInternal).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(8),
                        1,
                        3,
                        (((slotPtr).read()) as u32) as i32,
                    );
                    (slotPtr).write(0i8);
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn UpdatePartySelectionDoubleLayout(
    slotPtr: *mut i8,
    movementDir: i8,
) {
    unsafe {
        let mut slotPtr = slotPtr;
        let mut movementDir = movementDir;
        let mut newSlot: i8 = movementDir;
        'l1: {
            let __sw1 = ((movementDir) as i32);
            if __sw1 == (-1i32) {
                if (((slotPtr).read()) as i32) == 0i32 {
                    (slotPtr).write(7i8);
                    break 'l1;
                } else {
                    if (((slotPtr).read()) as i32) == 6i32 {
                        (slotPtr).write(
                            ((((((&raw mut gPlayerPartyCount).cast::<u8>()).read()) as i32)
                                .wrapping_sub(1i32)) as i8),
                        );
                        break 'l1;
                    } else {
                        if (((slotPtr).read()) as i32) == 7i32 {
                            if (crate::c::bf_read(
                                (((&raw mut sPartyMenuInternal).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(8),
                                0,
                                1,
                                false,
                            ) as u32)
                                != 0
                            {
                                (slotPtr).write(6i8);
                                break 'l1;
                            }
                            (slotPtr).write(((slotPtr).read()).wrapping_sub(1));
                        }
                    }
                }
                newSlot = GetNewSlotDoubleLayout((slotPtr).read(), newSlot);
                if ((newSlot) as i32) != (-1i32) {
                    (slotPtr).write(newSlot);
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                if (((slotPtr).read()) as i32) == 6i32 {
                    (slotPtr).write(7i8);
                } else {
                    if (((slotPtr).read()) as i32) == 7i32 {
                        (slotPtr).write(0i8);
                    } else {
                        newSlot = GetNewSlotDoubleLayout((slotPtr).read(), 1i8);
                        if ((newSlot) as i32) == (-1i32) {
                            if (crate::c::bf_read(
                                (((&raw mut sPartyMenuInternal).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(8),
                                0,
                                1,
                                false,
                            ) as u32)
                                != 0
                            {
                                (slotPtr).write(6i8);
                            } else {
                                (slotPtr).write(7i8);
                            }
                        } else {
                            (slotPtr).write(newSlot);
                        }
                    }
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                if (((slotPtr).read()) as i32) == 0i32 {
                    if (crate::c::bf_read(
                        (((&raw mut sPartyMenuInternal).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(8),
                        1,
                        3,
                        false,
                    ) as u32)
                        == 3u32
                    {
                        if GetMonData2(
                            ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(300),
                            11i32,
                        ) != 0u32
                        {
                            (slotPtr).write(3i8);
                        }
                    } else {
                        if GetMonData2(
                            ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(200),
                            11i32,
                        ) != 0u32
                        {
                            (slotPtr).write(2i8);
                        }
                    }
                } else {
                    if (((slotPtr).read()) as i32) == 1i32 {
                        if (crate::c::bf_read(
                            (((&raw mut sPartyMenuInternal).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(8),
                            1,
                            3,
                            false,
                        ) as u32)
                            == 5u32
                        {
                            if GetMonData2(
                                ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(500),
                                11i32,
                            ) != 0u32
                            {
                                (slotPtr).write(5i8);
                            }
                        } else {
                            if GetMonData2(
                                ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(400),
                                11i32,
                            ) != 0u32
                            {
                                (slotPtr).write(4i8);
                            }
                        }
                    }
                }
                break 'l1;
            }
            if __sw1 == (-2i32) {
                if ((((slotPtr).read()) as i32) == 2i32) || ((((slotPtr).read()) as i32) == 3i32) {
                    crate::c::bf_write(
                        (((&raw mut sPartyMenuInternal).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(8),
                        1,
                        3,
                        (((slotPtr).read()) as u32) as i32,
                    );
                    (slotPtr).write(0i8);
                } else {
                    if ((((slotPtr).read()) as i32) == 4i32)
                        || ((((slotPtr).read()) as i32) == 5i32)
                    {
                        crate::c::bf_write(
                            (((&raw mut sPartyMenuInternal).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(8),
                            1,
                            3,
                            (((slotPtr).read()) as u32) as i32,
                        );
                        (slotPtr).write(1i8);
                    }
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn GetNewSlotDoubleLayout(slotId: i8, movementDir: i8) -> i8 {
    unsafe {
        let mut slotId = slotId;
        let mut movementDir = movementDir;
        'l1: loop {
            if !((1i32) != 0) {
                break 'l1;
            }
            slotId = ((((slotId) as i32).wrapping_add(((movementDir) as i32))) as i8);
            if (((slotId) as u8) as i32) >= 6i32 {
                return (-1i8);
            }
            if GetMonData2(
                ((&raw mut gPlayerParty).cast::<u8>())
                    .wrapping_offset(((slotId) as i32) as isize * 100),
                11i32,
            ) != 0u32
            {
                return slotId;
            }
        }
        #[allow(unreachable_code)]
        {
            return 0i8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetMonNickname(mon: *mut u8, dest: *mut u8) -> *mut u8 {
    unsafe {
        let mut mon = mon;
        let mut dest = dest;
        GetMonData3(mon, 2i32, dest);
        return StringGet_Nickname(dest);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DisplayPartyMenuMessage(str: *mut u8, keepOpen: u8) -> u8 {
    unsafe {
        let mut str = str;
        let mut keepOpen = keepOpen;
        let mut taskId: u8 = 0u8;
        PrintMessage(str);
        taskId = CreateTask(Some(Task_PrintAndWaitForText), 1u8);
        (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .write(((keepOpen) as i16));
        return taskId;
    }
}
pub(crate) unsafe extern "C" fn Task_PrintAndWaitForText(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if ((RunTextPrintersRetIsActive(6u8)) as i32) != 1i32 {
            if (((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .read()) as i32)
                == 0i32
            {
                ClearStdWindowAndFrameToTransparent(6u8, 0u8);
                ClearWindowTilemap(6u8);
            }
            DestroyTask(taskId);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsPartyMenuTextPrinterActive() -> u8 {
    unsafe {
        return FuncIsActiveTask(Some(Task_PrintAndWaitForText));
    }
}
pub(crate) unsafe extern "C" fn Task_WaitForLinkAndReturnToChooseMon(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if ((MenuHelpers_ShouldWaitForLinkRecv()) as i32) != 1i32 {
            DisplayPartyMenuStdMessage(0u32);
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_HandleChooseMonInput));
        }
    }
}
pub(crate) unsafe extern "C" fn Task_ReturnToChooseMonAfterText(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if ((IsPartyMenuTextPrinterActive()) as i32) != 1i32 {
            ClearStdWindowAndFrameToTransparent(6u8, 0u8);
            ClearWindowTilemap(6u8);
            if ((MenuHelpers_IsLinkActive()) as i32) == 1i32 {
                ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
                .write(Some(Task_WaitForLinkAndReturnToChooseMon));
            } else {
                DisplayPartyMenuStdMessage(0u32);
                ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
                .write(Some(Task_HandleChooseMonInput));
            }
        }
    }
}
pub(crate) unsafe extern "C" fn DisplayGaveHeldItemMessage(
    mon: *mut u8,
    item: u16,
    keepOpen: u8,
    unused: u8,
) {
    unsafe {
        let mut mon = mon;
        let mut item = item;
        let mut keepOpen = keepOpen;
        let mut unused = unused;
        GetMonNickname(mon, (&raw mut gStringVar1).cast::<u8>());
        CopyItemName(item, (&raw mut gStringVar2).cast::<u8>());
        StringExpandPlaceholders(
            (&raw mut gStringVar4).cast::<u8>(),
            (&raw mut gText_PkmnWasGivenItem).cast::<u8>(),
        );
        DisplayPartyMenuMessage((&raw mut gStringVar4).cast::<u8>(), keepOpen);
        ScheduleBgCopyTilemapToVram(2u8);
    }
}
pub(crate) unsafe extern "C" fn DisplayTookHeldItemMessage(mon: *mut u8, item: u16, keepOpen: u8) {
    unsafe {
        let mut mon = mon;
        let mut item = item;
        let mut keepOpen = keepOpen;
        GetMonNickname(mon, (&raw mut gStringVar1).cast::<u8>());
        CopyItemName(item, (&raw mut gStringVar2).cast::<u8>());
        StringExpandPlaceholders(
            (&raw mut gStringVar4).cast::<u8>(),
            (&raw mut gText_ReceivedItemFromPkmn).cast::<u8>(),
        );
        DisplayPartyMenuMessage((&raw mut gStringVar4).cast::<u8>(), keepOpen);
        ScheduleBgCopyTilemapToVram(2u8);
    }
}
pub(crate) unsafe extern "C" fn DisplayAlreadyHoldingItemSwitchMessage(
    mon: *mut u8,
    item: u16,
    keepOpen: u8,
) {
    unsafe {
        let mut mon = mon;
        let mut item = item;
        let mut keepOpen = keepOpen;
        GetMonNickname(mon, (&raw mut gStringVar1).cast::<u8>());
        CopyItemName(item, (&raw mut gStringVar2).cast::<u8>());
        StringExpandPlaceholders(
            (&raw mut gStringVar4).cast::<u8>(),
            (&raw mut gText_PkmnAlreadyHoldingItemSwitch).cast::<u8>(),
        );
        DisplayPartyMenuMessage((&raw mut gStringVar4).cast::<u8>(), keepOpen);
        ScheduleBgCopyTilemapToVram(2u8);
    }
}
pub(crate) unsafe extern "C" fn DisplaySwitchedHeldItemMessage(
    item: u16,
    item2: u16,
    keepOpen: u8,
) {
    unsafe {
        let mut item = item;
        let mut item2 = item2;
        let mut keepOpen = keepOpen;
        CopyItemName(item, (&raw mut gStringVar1).cast::<u8>());
        CopyItemName(item2, (&raw mut gStringVar2).cast::<u8>());
        StringExpandPlaceholders(
            (&raw mut gStringVar4).cast::<u8>(),
            (&raw mut gText_SwitchedPkmnItem).cast::<u8>(),
        );
        DisplayPartyMenuMessage((&raw mut gStringVar4).cast::<u8>(), keepOpen);
        ScheduleBgCopyTilemapToVram(2u8);
    }
}
pub(crate) unsafe extern "C" fn GiveItemToMon(mon: *mut u8, item: u16) {
    unsafe {
        let mut mon = mon;
        let mut item = item;
        let mut itemBytes = crate::ffi::Align4([0u8; 2]);
        if ((ItemIsMail(item)) as i32) == 1i32 {
            if ((GiveMailToMonByItemId(mon, item)) as i32) == 255i32 {
                return;
            }
        }
        ((&raw mut itemBytes).cast::<u8>()).write(((item) as u8));
        (((&raw mut itemBytes).cast::<u8>()).wrapping_offset(1))
            .write(((((item) as i32) >> 8) as u8));
        SetMonData(mon, 12i32, (&raw mut itemBytes).cast::<u8>());
    }
}
pub(crate) unsafe extern "C" fn TryTakeMonItem(mon: *mut u8) -> u8 {
    unsafe {
        let mut mon = mon;
        let mut item: u16 = ((GetMonData2(mon, 12i32)) as u16);
        if ((item) as i32) == 0i32 {
            return 0u8;
        }
        if ((AddBagItem(item, 1u16)) as i32) == 0i32 {
            return 1u8;
        }
        item = 0u16;
        SetMonData(mon, 12i32, (&raw mut item).cast::<u8>());
        return 2u8;
    }
}
pub(crate) unsafe extern "C" fn BufferBagFullCantTakeItemMessage(itemUnused: u16) {
    unsafe {
        let mut itemUnused = itemUnused;
        StringExpandPlaceholders(
            (&raw mut gStringVar4).cast::<u8>(),
            (&raw mut gText_BagFullCouldNotRemoveItem).cast::<u8>(),
        );
    }
}
pub(crate) unsafe extern "C" fn Task_PartyMenuModifyHP(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        (data).write(
            (((((data).read()) as i32).wrapping_add(((((data).wrapping_offset(2)).read()) as i32)))
                as i16),
        );
        let __p1 = (data).wrapping_offset(3);
        (__p1).write(((__p1).read()).wrapping_sub(1));
        SetMonData(
            ((&raw mut gPlayerParty).cast::<u8>())
                .wrapping_offset(((((data).wrapping_offset(4)).read()) as i32) as isize * 100),
            57i32,
            (data).cast::<u8>(),
        );
        DisplayPartyPokemonHPCheck(
            ((&raw mut gPlayerParty).cast::<u8>())
                .wrapping_offset(((((data).wrapping_offset(4)).read()) as i32) as isize * 100),
            (((&raw mut sPartyMenuBoxes).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_offset(((((data).wrapping_offset(4)).read()) as i32) as isize * 16),
            1u8,
        );
        DisplayPartyPokemonHPBarCheck(
            ((&raw mut gPlayerParty).cast::<u8>())
                .wrapping_offset(((((data).wrapping_offset(4)).read()) as i32) as isize * 100),
            (((&raw mut sPartyMenuBoxes).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_offset(((((data).wrapping_offset(4)).read()) as i32) as isize * 16),
        );
        if ((((((data).wrapping_offset(3)).read()) as i32) == 0i32)
            || ((((data).read()) as i32) == 0i32))
            || ((((data).read()) as i32) == ((((data).wrapping_offset(1)).read()) as i32))
        {
            if (((data).read()) as i32) > ((((data).wrapping_offset(5)).read()) as i32) {
                ConvertIntToDecimalStringN(
                    (&raw mut gStringVar2).cast::<u8>(),
                    (((data).read()) as i32)
                        .wrapping_sub(((((data).wrapping_offset(5)).read()) as i32)),
                    0i32,
                    3u8,
                );
            }
            SwitchTaskToFollowupFunc(taskId);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PartyMenuModifyHP(
    taskId: u8,
    slot: u8,
    hpIncrement: i8,
    hpDifference: i16,
    task: Option<unsafe extern "C" fn(u8)>,
) {
    unsafe {
        let mut taskId = taskId;
        let mut slot = slot;
        let mut hpIncrement = hpIncrement;
        let mut hpDifference = hpDifference;
        let mut task = task;
        let mut mon: *mut u8 =
            ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(((slot) as i32) as isize * 100);
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        (data).write(((GetMonData2(mon, 57i32)) as i16));
        ((data).wrapping_offset(1)).write(((GetMonData2(mon, 58i32)) as i16));
        ((data).wrapping_offset(2)).write(((hpIncrement) as i16));
        ((data).wrapping_offset(3)).write(hpDifference);
        ((data).wrapping_offset(4)).write(((slot) as i16));
        ((data).wrapping_offset(5)).write((data).read());
        SetTaskFuncWithFollowupFunc(taskId, Some(Task_PartyMenuModifyHP), task);
    }
}
pub(crate) unsafe extern "C" fn ResetHPTaskData(taskId: u8, caseId: u8, hp: u32) {
    unsafe {
        let mut taskId = taskId;
        let mut caseId = caseId;
        let mut hp = hp;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        'l1: {
            let __sw1 = ((caseId) as i32);
            if __sw1 == 0i32 {
                (data).write(((hp) as i16));
                ((data).wrapping_offset(5)).write(((hp) as i16));
                break 'l1;
            }
            if __sw1 == 1i32 {
                ((data).wrapping_offset(1)).write(((hp) as i16));
                break 'l1;
            }
            if __sw1 == 2i32 {
                ((data).wrapping_offset(2)).write(((hp) as i16));
                break 'l1;
            }
            if __sw1 == 3i32 {
                ((data).wrapping_offset(3)).write(((hp) as i16));
                break 'l1;
            }
            if __sw1 == 4i32 {
                ((data).wrapping_offset(4)).write(((hp) as i16));
                break 'l1;
            }
            if __sw1 == 5i32 {
                SetTaskFuncWithFollowupFunc(
                    taskId,
                    Some(Task_PartyMenuModifyHP),
                    (core::mem::transmute::<usize, Option<unsafe extern "C" fn(u8)>>(
                        (hp) as usize,
                    )),
                );
                break 'l1;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetAilmentFromStatus(status: u32) -> u8 {
    unsafe {
        let mut status = status;
        if (status & 136u32) != 0 {
            return 1u8;
        }
        if (status & 64u32) != 0 {
            return 2u8;
        }
        if (status & 7u32) != 0 {
            return 3u8;
        }
        if (status & 32u32) != 0 {
            return 4u8;
        }
        if (status & 16u32) != 0 {
            return 5u8;
        }
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetMonAilment(mon: *mut u8) -> u8 {
    unsafe {
        let mut mon = mon;
        let mut ailment: u8 = 0u8;
        if GetMonData2(mon, 57i32) == 0u32 {
            return 7u8;
        }
        ailment = GetAilmentFromStatus(GetMonData2(mon, 55i32));
        if ((ailment) as i32) != 0i32 {
            return ailment;
        }
        if (CheckPartyPokerus(mon, 0u8)) != 0 {
            return 6u8;
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn SetPartyMonsAllowedInMinigame() {
    unsafe {
        let mut ptr: *mut i16 = core::ptr::null_mut();
        if ((crate::c::bf_read(
            ((&raw mut gPartyMenu).cast::<u8>()).wrapping_add(8),
            0,
            4,
            false,
        ) as u8) as i32)
            == 11i32
        {
            let mut i: u8 = 0u8;
            ptr = (((&raw mut gPartyMenu).cast::<u8>()).wrapping_add(14)).cast::<i16>();
            ((((&raw mut gPartyMenu).cast::<u8>()).wrapping_add(14)).cast::<i16>()).write(0i16);
            if ((((&raw mut gSpecialVar_0x8005).cast::<u16>()).read()) as i32) == 0i32 {
                {
                    i = 0u8;
                    'l1: loop {
                        if !(((i) as i32)
                            < ((((&raw mut gPlayerPartyCount).cast::<u8>()).read()) as i32))
                        {
                            break 'l1;
                        }
                        'l2: {
                            (ptr).write(
                                (((((ptr).read()) as i32).wrapping_add(crate::c::shl_i32(
                                    ((IsMonAllowedInPokemonJump(
                                        ((&raw mut gPlayerParty).cast::<u8>())
                                            .wrapping_offset(((i) as i32) as isize * 100),
                                    )) as i32),
                                    ((i) as u32),
                                ))) as i16),
                            );
                        }
                        i = (i).wrapping_add(1);
                    }
                }
            } else {
                {
                    i = 0u8;
                    'l3: loop {
                        if !(((i) as i32)
                            < ((((&raw mut gPlayerPartyCount).cast::<u8>()).read()) as i32))
                        {
                            break 'l3;
                        }
                        'l4: {
                            (ptr).write(
                                (((((ptr).read()) as i32).wrapping_add(crate::c::shl_i32(
                                    ((IsMonAllowedInDodrioBerryPicking(
                                        ((&raw mut gPlayerParty).cast::<u8>())
                                            .wrapping_offset(((i) as i32) as isize * 100),
                                    )) as i32),
                                    ((i) as u32),
                                ))) as i16),
                            );
                        }
                        i = (i).wrapping_add(1);
                    }
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn IsMonAllowedInPokemonJump(mon: *mut u8) -> u16 {
    unsafe {
        let mut mon = mon;
        if (GetMonData2(mon, 45i32) != 1u32)
            && ((IsSpeciesAllowedInPokemonJump(((GetMonData2(mon, 11i32)) as u16))) != 0)
        {
            return 1u16;
        }
        return 0u16;
    }
}
pub(crate) unsafe extern "C" fn IsMonAllowedInDodrioBerryPicking(mon: *mut u8) -> u16 {
    unsafe {
        let mut mon = mon;
        if (GetMonData2(mon, 45i32) != 1u32) && (GetMonData2(mon, 11i32) == 85u32) {
            return 1u16;
        }
        return 0u16;
    }
}
pub(crate) unsafe extern "C" fn IsMonAllowedInMinigame(slot: u8) -> u8 {
    unsafe {
        let mut slot = slot;
        if !((crate::c::shr_i32(
            ((((((&raw mut gPartyMenu).cast::<u8>()).wrapping_add(14)).cast::<i16>()).read())
                as i32),
            ((slot) as u32),
        ) & 1i32)
            != 0)
        {
            return 0u8;
        }
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn TryEnterMonForMinigame(taskId: u8, slot: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut slot = slot;
        if ((IsMonAllowedInMinigame(slot)) as i32) == 1i32 {
            PlaySE(5u16);
            ((&raw mut gSpecialVar_0x8004).cast::<u16>()).write(((slot) as u16));
            Task_ClosePartyMenu(taskId);
        } else {
            PlaySE(32u16);
            DisplayPartyMenuMessage((&raw mut gText_PkmnCantParticipate).cast::<u8>(), 0u8);
            ScheduleBgCopyTilemapToVram(2u8);
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_ReturnToChooseMonAfterText));
        }
    }
}
pub(crate) unsafe extern "C" fn CancelParticipationPrompt(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        DisplayPartyMenuMessage((&raw mut gText_CancelParticipation).cast::<u8>(), 1u8);
        ScheduleBgCopyTilemapToVram(2u8);
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Task_CancelParticipationYesNo));
    }
}
pub(crate) unsafe extern "C" fn Task_CancelParticipationYesNo(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if ((IsPartyMenuTextPrinterActive()) as i32) != 1i32 {
            PartyMenuDisplayYesNoMenu();
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_HandleCancelParticipationYesNoInput));
        }
    }
}
pub(crate) unsafe extern "C" fn Task_HandleCancelParticipationYesNoInput(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        'l1: {
            let __sw1 = ((Menu_ProcessInputNoWrapClearOnChoose()) as i32);
            let mut __fall = false;
            if __sw1 == 0i32 {
                __fall = true;
                ((&raw mut gSpecialVar_0x8004).cast::<u16>()).write(7u16);
                Task_ClosePartyMenu(taskId);
                break 'l1;
            }
            if __sw1 == (-1i32) {
                __fall = true;
                PlaySE(5u16);
            }
            if __fall || __sw1 == 1i32 {
                __fall = true;
                ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
                .write(Some(Task_ReturnToChooseMonAfterText));
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn CanMonLearnTMTutor(mon: *mut u8, item: u16, tutor: u8) -> u8 {
    unsafe {
        let mut mon = mon;
        let mut item = item;
        let mut tutor = tutor;
        let mut r#move: u16 = 0u16;
        if (GetMonData2(mon, 45i32)) != 0 {
            return 3u8;
        }
        if ((item) as i32) >= 289i32 {
            if !((CanMonLearnTMHM(mon, ((((item) as i32).wrapping_sub(289i32)) as u8))) != 0) {
                return 1u8;
            } else {
                r#move = ItemIdToBattleMoveId(item);
            }
        } else {
            if !((CanLearnTutorMove(((GetMonData2(mon, 11i32)) as u16), tutor)) != 0) {
                return 1u8;
            } else {
                r#move = GetTutorMove(tutor);
            }
        }
        if ((MonKnowsMove(mon, r#move)) as i32) == 1i32 {
            return 2u8;
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
pub(crate) unsafe extern "C" fn GetTutorMove(tutor: u8) -> u16 {
    unsafe {
        let mut tutor = tutor;
        return ((((&raw const gTutorMoves)
            .cast::<u8>()
            .cast_mut()
            .cast::<u16>())
        .cast::<u16>())
        .wrapping_offset(((tutor) as i32) as isize))
        .read();
    }
}
pub(crate) unsafe extern "C" fn CanLearnTutorMove(species: u16, tutor: u8) -> u8 {
    unsafe {
        let mut species = species;
        let mut tutor = tutor;
        if (((((&raw const sTutorLearnsets)
            .cast::<u8>()
            .cast_mut()
            .cast::<u32>())
        .cast::<u32>())
        .wrapping_offset(((species) as i32) as isize))
        .read()
            & ((crate::c::shl_i32(1i32, ((tutor) as u32))) as u32))
            != 0
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
pub(crate) unsafe extern "C" fn InitPartyMenuWindows(layout: u8) {
    unsafe {
        let mut layout = layout;
        let mut i: u8 = 0u8;
        'l1: {
            let __sw1 = ((layout) as i32);
            let __matched = __sw1 == 0i32 || __sw1 == 1i32 || __sw1 == 2i32;
            if __sw1 == 0i32 {
                InitWindows(
                    ((&raw const sSinglePartyMenuWindowTemplate)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>(),
                );
                break 'l1;
            }
            if __sw1 == 1i32 {
                InitWindows(
                    ((&raw const sDoublePartyMenuWindowTemplate)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>(),
                );
                break 'l1;
            }
            if __sw1 == 2i32 {
                InitWindows(
                    ((&raw const sMultiPartyMenuWindowTemplate)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>(),
                );
                break 'l1;
            }
            if !__matched {
                InitWindows(
                    ((&raw const sShowcaseMultiPartyMenuWindowTemplate)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>(),
                );
                break 'l1;
            }
        }
        DeactivateAllTextPrinters();
        {
            i = 0u8;
            'l2: loop {
                if !(((i) as i32) < 6i32) {
                    break 'l2;
                }
                'l3: {
                    FillWindowPixelBuffer(i, 0u8);
                }
                i = (i).wrapping_add(1);
            }
        }
        LoadUserWindowBorderGfx(0u8, 79u16, 208u8);
        LoadPalette(
            (GetOverworldTextboxPalettePtr()).cast::<u8>(),
            224u16,
            32u16,
        );
        LoadPalette(
            (((&raw mut gStandardMenuPalette).cast::<u16>()).cast::<u16>()).cast::<u8>(),
            240u16,
            32u16,
        );
    }
}
pub(crate) unsafe extern "C" fn CreateCancelConfirmWindows(chooseHalf: u8) {
    unsafe {
        let mut chooseHalf = chooseHalf;
        let mut confirmWindowId: u8 = 0u8;
        let mut cancelWindowId: u8 = 0u8;
        let mut offset: u8 = 0u8;
        let mut mainOffset: u8 = 0u8;
        if ((crate::c::bf_read(
            ((&raw mut gPartyMenu).cast::<u8>()).wrapping_add(8),
            0,
            4,
            false,
        ) as u8) as i32)
            != 5i32
        {
            if ((chooseHalf) as i32) == 1i32 {
                confirmWindowId = ((AddWindow(
                    (&raw const sConfirmButtonWindowTemplate)
                        .cast::<u8>()
                        .cast_mut(),
                )) as u8);
                FillWindowPixelBuffer(confirmWindowId, 0u8);
                mainOffset = ((GetStringCenterAlignXOffset(
                    0i32,
                    (&raw mut gMenuText_Confirm).cast::<u8>(),
                    48i32,
                )) as u8);
                AddTextPrinterParameterized4(
                    confirmWindowId,
                    0u8,
                    mainOffset,
                    1u8,
                    0u8,
                    0u8,
                    (((&raw const sFontColorTable).cast::<u8>().cast_mut()).cast::<u8>())
                        .cast::<u8>(),
                    (-1i8),
                    (&raw mut gMenuText_Confirm).cast::<u8>(),
                );
                PutWindowTilemap(confirmWindowId);
                CopyWindowToVram(confirmWindowId, 2u8);
                cancelWindowId = ((AddWindow(
                    (&raw const sMultiCancelButtonWindowTemplate)
                        .cast::<u8>()
                        .cast_mut(),
                )) as u8);
                offset = 0u8;
            } else {
                cancelWindowId = ((AddWindow(
                    (&raw const sCancelButtonWindowTemplate)
                        .cast::<u8>()
                        .cast_mut(),
                )) as u8);
                offset = 3u8;
            }
            FillWindowPixelBuffer(cancelWindowId, 0u8);
            if ((crate::c::bf_read(
                ((&raw mut gPartyMenu).cast::<u8>()).wrapping_add(8),
                0,
                4,
                false,
            ) as u8) as i32)
                != 10i32
            {
                mainOffset = ((GetStringCenterAlignXOffset(
                    0i32,
                    (&raw mut gText_Cancel).cast::<u8>(),
                    48i32,
                )) as u8);
                AddTextPrinterParameterized3(
                    cancelWindowId,
                    0u8,
                    ((((mainOffset) as i32).wrapping_add(((offset) as i32))) as u8),
                    1u8,
                    (((&raw const sFontColorTable).cast::<u8>().cast_mut()).cast::<u8>())
                        .cast::<u8>(),
                    (-1i8),
                    (&raw mut gText_Cancel).cast::<u8>(),
                );
            } else {
                mainOffset = ((GetStringCenterAlignXOffset(
                    0i32,
                    (&raw mut gText_Cancel2).cast::<u8>(),
                    48i32,
                )) as u8);
                AddTextPrinterParameterized3(
                    cancelWindowId,
                    0u8,
                    ((((mainOffset) as i32).wrapping_add(((offset) as i32))) as u8),
                    1u8,
                    (((&raw const sFontColorTable).cast::<u8>().cast_mut()).cast::<u8>())
                        .cast::<u8>(),
                    (-1i8),
                    (&raw mut gText_Cancel2).cast::<u8>(),
                );
            }
            PutWindowTilemap(cancelWindowId);
            CopyWindowToVram(cancelWindowId, 2u8);
            ScheduleBgCopyTilemapToVram(0u8);
        }
    }
}
pub(crate) unsafe extern "C" fn GetPartyMenuPalBufferPtr(paletteId: u8) -> *mut u16 {
    unsafe {
        let mut paletteId = paletteId;
        return (((((&raw mut sPartyMenuInternal).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(24))
        .cast::<u16>())
        .wrapping_offset(((paletteId) as i32) as isize);
    }
}
pub(crate) unsafe extern "C" fn BlitBitmapToPartyWindow(
    windowId: u8,
    b: *mut u8,
    c: u8,
    x: u8,
    y: u8,
    width: u8,
    height: u8,
) {
    unsafe {
        let mut windowId = windowId;
        let mut b = b;
        let mut c = c;
        let mut x = x;
        let mut y = y;
        let mut width = width;
        let mut height = height;
        let mut pixels: *mut u8 = AllocZeroed(
            (((((height) as i32).wrapping_mul(((width) as i32))).wrapping_mul(32i32)) as u32),
        );
        let mut i: u8 = 0u8;
        let mut j: u8 = 0u8;
        if ((pixels) as usize) != 0usize {
            {
                i = 0u8;
                'l1: loop {
                    if !(((i) as i32) < ((height) as i32)) {
                        break 'l1;
                    }
                    'l2: {
                        {
                            j = 0u8;
                            'l3: loop {
                                if !(((j) as i32) < ((width) as i32)) {
                                    break 'l3;
                                }
                                'l4: {
                                    'l5: loop {
                                        'l6: {
                                            'l7: loop {
                                                'l8: {
                                                    CpuSet(
                                                        GetPartyMenuBgTile(
                                                            ((((b).wrapping_offset(
                                                                ((((x) as i32)
                                                                    .wrapping_add(((j) as i32)))
                                                                .wrapping_add(
                                                                    (((y) as i32).wrapping_add(
                                                                        ((i) as i32),
                                                                    ))
                                                                    .wrapping_mul(((c) as i32)),
                                                                ))
                                                                    as isize,
                                                            ))
                                                            .read())
                                                                as u16),
                                                        ),
                                                        (pixels).wrapping_offset(
                                                            (((((i) as i32)
                                                                .wrapping_mul(((width) as i32)))
                                                            .wrapping_add(((j) as i32)))
                                                            .wrapping_mul(32i32))
                                                                as isize,
                                                        ),
                                                        ((0i32
                                                            | (crate::c::div_i32(
                                                                32i32,
                                                                crate::c::div_i32(16i32, 8i32),
                                                            ) & 2097151i32))
                                                            as u32),
                                                    );
                                                }
                                                if !((0i32) != 0) {
                                                    break 'l7;
                                                }
                                            }
                                        }
                                        if !((0i32) != 0) {
                                            break 'l5;
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
            BlitBitmapToWindow(
                windowId,
                pixels,
                ((((x) as i32).wrapping_mul(8i32)) as u16),
                ((((y) as i32).wrapping_mul(8i32)) as u16),
                ((((width) as i32).wrapping_mul(8i32)) as u16),
                ((((height) as i32).wrapping_mul(8i32)) as u16),
            );
            Free(pixels);
        }
    }
}
pub(crate) unsafe extern "C" fn BlitBitmapToPartyWindow_LeftColumn(
    windowId: u8,
    x: u8,
    y: u8,
    width: u8,
    height: u8,
    hideHP: u8,
) {
    unsafe {
        let mut windowId = windowId;
        let mut x = x;
        let mut y = y;
        let mut width = width;
        let mut height = height;
        let mut hideHP = hideHP;
        if (((width) as i32) == 0i32) && (((height) as i32) == 0i32) {
            width = 10u8;
            height = 7u8;
        }
        if ((hideHP) as i32) == 0i32 {
            BlitBitmapToPartyWindow(
                windowId,
                ((&raw const sSlotTilemap_Main).cast::<u8>().cast_mut()).cast::<u8>(),
                10u8,
                x,
                y,
                width,
                height,
            );
        } else {
            BlitBitmapToPartyWindow(
                windowId,
                ((&raw const sSlotTilemap_MainNoHP).cast::<u8>().cast_mut()).cast::<u8>(),
                10u8,
                x,
                y,
                width,
                height,
            );
        }
    }
}
pub(crate) unsafe extern "C" fn BlitBitmapToPartyWindow_RightColumn(
    windowId: u8,
    x: u8,
    y: u8,
    width: u8,
    height: u8,
    hideHP: u8,
) {
    unsafe {
        let mut windowId = windowId;
        let mut x = x;
        let mut y = y;
        let mut width = width;
        let mut height = height;
        let mut hideHP = hideHP;
        if (((width) as i32) == 0i32) && (((height) as i32) == 0i32) {
            width = 18u8;
            height = 3u8;
        }
        if ((hideHP) as i32) == 0i32 {
            BlitBitmapToPartyWindow(
                windowId,
                ((&raw const sSlotTilemap_Wide).cast::<u8>().cast_mut()).cast::<u8>(),
                18u8,
                x,
                y,
                width,
                height,
            );
        } else {
            BlitBitmapToPartyWindow(
                windowId,
                ((&raw const sSlotTilemap_WideNoHP).cast::<u8>().cast_mut()).cast::<u8>(),
                18u8,
                x,
                y,
                width,
                height,
            );
        }
    }
}
pub(crate) unsafe extern "C" fn DrawEmptySlot(windowId: u8) {
    unsafe {
        let mut windowId = windowId;
        BlitBitmapToPartyWindow(
            windowId,
            ((&raw const sSlotTilemap_WideEmpty).cast::<u8>().cast_mut()).cast::<u8>(),
            18u8,
            0u8,
            0u8,
            18u8,
            3u8,
        );
    }
}
pub(crate) unsafe extern "C" fn LoadPartyBoxPalette(menuBox: *mut u8, palFlags: u8) {
    unsafe {
        let mut menuBox = menuBox;
        let mut palFlags = palFlags;
        let mut palOffset: u8 = (((0u32).wrapping_add(
            (GetWindowAttribute(((menuBox).wrapping_add(8)).read(), 5u8)).wrapping_mul(16u32),
        )) as u8);
        if (((palFlags) as i32) & 64i32) != 0 {
            {
                LoadPalette(
                    (GetPartyMenuPalBufferPtr(
                        (((&raw const sPartyBoxNoMonPalIds).cast::<u8>().cast_mut()).cast::<u8>())
                            .read(),
                    ))
                    .cast::<u8>(),
                    (((((((&raw const sPartyBoxNoMonPalOffsets)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>())
                    .read()) as i32)
                        .wrapping_add(((palOffset) as i32))) as u16),
                    2u16,
                );
                LoadPalette(
                    (GetPartyMenuPalBufferPtr(
                        ((((&raw const sPartyBoxNoMonPalIds).cast::<u8>().cast_mut())
                            .cast::<u8>())
                        .wrapping_offset(1))
                        .read(),
                    ))
                    .cast::<u8>(),
                    ((((((((&raw const sPartyBoxNoMonPalOffsets)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>())
                    .wrapping_offset(1))
                    .read()) as i32)
                        .wrapping_add(((palOffset) as i32))) as u16),
                    2u16,
                );
                LoadPalette(
                    (GetPartyMenuPalBufferPtr(
                        ((((&raw const sPartyBoxNoMonPalIds).cast::<u8>().cast_mut())
                            .cast::<u8>())
                        .wrapping_offset(2))
                        .read(),
                    ))
                    .cast::<u8>(),
                    ((((((((&raw const sPartyBoxNoMonPalOffsets)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>())
                    .wrapping_offset(2))
                    .read()) as i32)
                        .wrapping_add(((palOffset) as i32))) as u16),
                    2u16,
                );
            }
        } else {
            if (((palFlags) as i32) & 32i32) != 0 {
                if (((palFlags) as i32) & 1i32) != 0 {
                    {
                        LoadPalette(
                            (GetPartyMenuPalBufferPtr(
                                (((&raw const sPartyBoxSelectedForActionPalIds1)
                                    .cast::<u8>()
                                    .cast_mut())
                                .cast::<u8>())
                                .read(),
                            ))
                            .cast::<u8>(),
                            (((((((&raw const sPartyBoxPalOffsets1).cast::<u8>().cast_mut())
                                .cast::<u8>())
                            .read()) as i32)
                                .wrapping_add(((palOffset) as i32)))
                                as u16),
                            2u16,
                        );
                        LoadPalette(
                            (GetPartyMenuPalBufferPtr(
                                ((((&raw const sPartyBoxSelectedForActionPalIds1)
                                    .cast::<u8>()
                                    .cast_mut())
                                .cast::<u8>())
                                .wrapping_offset(1))
                                .read(),
                            ))
                            .cast::<u8>(),
                            ((((((((&raw const sPartyBoxPalOffsets1).cast::<u8>().cast_mut())
                                .cast::<u8>())
                            .wrapping_offset(1))
                            .read()) as i32)
                                .wrapping_add(((palOffset) as i32)))
                                as u16),
                            2u16,
                        );
                        LoadPalette(
                            (GetPartyMenuPalBufferPtr(
                                ((((&raw const sPartyBoxSelectedForActionPalIds1)
                                    .cast::<u8>()
                                    .cast_mut())
                                .cast::<u8>())
                                .wrapping_offset(2))
                                .read(),
                            ))
                            .cast::<u8>(),
                            ((((((((&raw const sPartyBoxPalOffsets1).cast::<u8>().cast_mut())
                                .cast::<u8>())
                            .wrapping_offset(2))
                            .read()) as i32)
                                .wrapping_add(((palOffset) as i32)))
                                as u16),
                            2u16,
                        );
                    }
                    {
                        LoadPalette(
                            (GetPartyMenuPalBufferPtr(
                                (((&raw const sPartyBoxCurrSelectionPalIds2)
                                    .cast::<u8>()
                                    .cast_mut())
                                .cast::<u8>())
                                .read(),
                            ))
                            .cast::<u8>(),
                            (((((((&raw const sPartyBoxPalOffsets2).cast::<u8>().cast_mut())
                                .cast::<u8>())
                            .read()) as i32)
                                .wrapping_add(((palOffset) as i32)))
                                as u16),
                            2u16,
                        );
                        LoadPalette(
                            (GetPartyMenuPalBufferPtr(
                                ((((&raw const sPartyBoxCurrSelectionPalIds2)
                                    .cast::<u8>()
                                    .cast_mut())
                                .cast::<u8>())
                                .wrapping_offset(1))
                                .read(),
                            ))
                            .cast::<u8>(),
                            ((((((((&raw const sPartyBoxPalOffsets2).cast::<u8>().cast_mut())
                                .cast::<u8>())
                            .wrapping_offset(1))
                            .read()) as i32)
                                .wrapping_add(((palOffset) as i32)))
                                as u16),
                            2u16,
                        );
                        LoadPalette(
                            (GetPartyMenuPalBufferPtr(
                                ((((&raw const sPartyBoxCurrSelectionPalIds2)
                                    .cast::<u8>()
                                    .cast_mut())
                                .cast::<u8>())
                                .wrapping_offset(2))
                                .read(),
                            ))
                            .cast::<u8>(),
                            ((((((((&raw const sPartyBoxPalOffsets2).cast::<u8>().cast_mut())
                                .cast::<u8>())
                            .wrapping_offset(2))
                            .read()) as i32)
                                .wrapping_add(((palOffset) as i32)))
                                as u16),
                            2u16,
                        );
                    }
                } else {
                    {
                        LoadPalette(
                            (GetPartyMenuPalBufferPtr(
                                (((&raw const sPartyBoxSelectedForActionPalIds1)
                                    .cast::<u8>()
                                    .cast_mut())
                                .cast::<u8>())
                                .read(),
                            ))
                            .cast::<u8>(),
                            (((((((&raw const sPartyBoxPalOffsets1).cast::<u8>().cast_mut())
                                .cast::<u8>())
                            .read()) as i32)
                                .wrapping_add(((palOffset) as i32)))
                                as u16),
                            2u16,
                        );
                        LoadPalette(
                            (GetPartyMenuPalBufferPtr(
                                ((((&raw const sPartyBoxSelectedForActionPalIds1)
                                    .cast::<u8>()
                                    .cast_mut())
                                .cast::<u8>())
                                .wrapping_offset(1))
                                .read(),
                            ))
                            .cast::<u8>(),
                            ((((((((&raw const sPartyBoxPalOffsets1).cast::<u8>().cast_mut())
                                .cast::<u8>())
                            .wrapping_offset(1))
                            .read()) as i32)
                                .wrapping_add(((palOffset) as i32)))
                                as u16),
                            2u16,
                        );
                        LoadPalette(
                            (GetPartyMenuPalBufferPtr(
                                ((((&raw const sPartyBoxSelectedForActionPalIds1)
                                    .cast::<u8>()
                                    .cast_mut())
                                .cast::<u8>())
                                .wrapping_offset(2))
                                .read(),
                            ))
                            .cast::<u8>(),
                            ((((((((&raw const sPartyBoxPalOffsets1).cast::<u8>().cast_mut())
                                .cast::<u8>())
                            .wrapping_offset(2))
                            .read()) as i32)
                                .wrapping_add(((palOffset) as i32)))
                                as u16),
                            2u16,
                        );
                    }
                    {
                        LoadPalette(
                            (GetPartyMenuPalBufferPtr(
                                (((&raw const sPartyBoxSelectedForActionPalIds2)
                                    .cast::<u8>()
                                    .cast_mut())
                                .cast::<u8>())
                                .read(),
                            ))
                            .cast::<u8>(),
                            (((((((&raw const sPartyBoxPalOffsets2).cast::<u8>().cast_mut())
                                .cast::<u8>())
                            .read()) as i32)
                                .wrapping_add(((palOffset) as i32)))
                                as u16),
                            2u16,
                        );
                        LoadPalette(
                            (GetPartyMenuPalBufferPtr(
                                ((((&raw const sPartyBoxSelectedForActionPalIds2)
                                    .cast::<u8>()
                                    .cast_mut())
                                .cast::<u8>())
                                .wrapping_offset(1))
                                .read(),
                            ))
                            .cast::<u8>(),
                            ((((((((&raw const sPartyBoxPalOffsets2).cast::<u8>().cast_mut())
                                .cast::<u8>())
                            .wrapping_offset(1))
                            .read()) as i32)
                                .wrapping_add(((palOffset) as i32)))
                                as u16),
                            2u16,
                        );
                        LoadPalette(
                            (GetPartyMenuPalBufferPtr(
                                ((((&raw const sPartyBoxSelectedForActionPalIds2)
                                    .cast::<u8>()
                                    .cast_mut())
                                .cast::<u8>())
                                .wrapping_offset(2))
                                .read(),
                            ))
                            .cast::<u8>(),
                            ((((((((&raw const sPartyBoxPalOffsets2).cast::<u8>().cast_mut())
                                .cast::<u8>())
                            .wrapping_offset(2))
                            .read()) as i32)
                                .wrapping_add(((palOffset) as i32)))
                                as u16),
                            2u16,
                        );
                    }
                }
            } else {
                if (((palFlags) as i32) & 16i32) != 0 {
                    {
                        LoadPalette(
                            (GetPartyMenuPalBufferPtr(
                                (((&raw const sPartyBoxSelectedForActionPalIds1)
                                    .cast::<u8>()
                                    .cast_mut())
                                .cast::<u8>())
                                .read(),
                            ))
                            .cast::<u8>(),
                            (((((((&raw const sPartyBoxPalOffsets1).cast::<u8>().cast_mut())
                                .cast::<u8>())
                            .read()) as i32)
                                .wrapping_add(((palOffset) as i32)))
                                as u16),
                            2u16,
                        );
                        LoadPalette(
                            (GetPartyMenuPalBufferPtr(
                                ((((&raw const sPartyBoxSelectedForActionPalIds1)
                                    .cast::<u8>()
                                    .cast_mut())
                                .cast::<u8>())
                                .wrapping_offset(1))
                                .read(),
                            ))
                            .cast::<u8>(),
                            ((((((((&raw const sPartyBoxPalOffsets1).cast::<u8>().cast_mut())
                                .cast::<u8>())
                            .wrapping_offset(1))
                            .read()) as i32)
                                .wrapping_add(((palOffset) as i32)))
                                as u16),
                            2u16,
                        );
                        LoadPalette(
                            (GetPartyMenuPalBufferPtr(
                                ((((&raw const sPartyBoxSelectedForActionPalIds1)
                                    .cast::<u8>()
                                    .cast_mut())
                                .cast::<u8>())
                                .wrapping_offset(2))
                                .read(),
                            ))
                            .cast::<u8>(),
                            ((((((((&raw const sPartyBoxPalOffsets1).cast::<u8>().cast_mut())
                                .cast::<u8>())
                            .wrapping_offset(2))
                            .read()) as i32)
                                .wrapping_add(((palOffset) as i32)))
                                as u16),
                            2u16,
                        );
                    }
                    {
                        LoadPalette(
                            (GetPartyMenuPalBufferPtr(
                                (((&raw const sPartyBoxSelectedForActionPalIds2)
                                    .cast::<u8>()
                                    .cast_mut())
                                .cast::<u8>())
                                .read(),
                            ))
                            .cast::<u8>(),
                            (((((((&raw const sPartyBoxPalOffsets2).cast::<u8>().cast_mut())
                                .cast::<u8>())
                            .read()) as i32)
                                .wrapping_add(((palOffset) as i32)))
                                as u16),
                            2u16,
                        );
                        LoadPalette(
                            (GetPartyMenuPalBufferPtr(
                                ((((&raw const sPartyBoxSelectedForActionPalIds2)
                                    .cast::<u8>()
                                    .cast_mut())
                                .cast::<u8>())
                                .wrapping_offset(1))
                                .read(),
                            ))
                            .cast::<u8>(),
                            ((((((((&raw const sPartyBoxPalOffsets2).cast::<u8>().cast_mut())
                                .cast::<u8>())
                            .wrapping_offset(1))
                            .read()) as i32)
                                .wrapping_add(((palOffset) as i32)))
                                as u16),
                            2u16,
                        );
                        LoadPalette(
                            (GetPartyMenuPalBufferPtr(
                                ((((&raw const sPartyBoxSelectedForActionPalIds2)
                                    .cast::<u8>()
                                    .cast_mut())
                                .cast::<u8>())
                                .wrapping_offset(2))
                                .read(),
                            ))
                            .cast::<u8>(),
                            ((((((((&raw const sPartyBoxPalOffsets2).cast::<u8>().cast_mut())
                                .cast::<u8>())
                            .wrapping_offset(2))
                            .read()) as i32)
                                .wrapping_add(((palOffset) as i32)))
                                as u16),
                            2u16,
                        );
                    }
                } else {
                    if (((palFlags) as i32) & 4i32) != 0 {
                        if (((palFlags) as i32) & 1i32) != 0 {
                            {
                                LoadPalette(
                                    (GetPartyMenuPalBufferPtr(
                                        (((&raw const sPartyBoxSelectedForActionPalIds1)
                                            .cast::<u8>()
                                            .cast_mut())
                                        .cast::<u8>())
                                        .read(),
                                    ))
                                    .cast::<u8>(),
                                    (((((((&raw const sPartyBoxPalOffsets1)
                                        .cast::<u8>()
                                        .cast_mut())
                                    .cast::<u8>())
                                    .read()) as i32)
                                        .wrapping_add(((palOffset) as i32)))
                                        as u16),
                                    2u16,
                                );
                                LoadPalette(
                                    (GetPartyMenuPalBufferPtr(
                                        ((((&raw const sPartyBoxSelectedForActionPalIds1)
                                            .cast::<u8>()
                                            .cast_mut())
                                        .cast::<u8>())
                                        .wrapping_offset(1))
                                        .read(),
                                    ))
                                    .cast::<u8>(),
                                    ((((((((&raw const sPartyBoxPalOffsets1)
                                        .cast::<u8>()
                                        .cast_mut())
                                    .cast::<u8>())
                                    .wrapping_offset(1))
                                    .read()) as i32)
                                        .wrapping_add(((palOffset) as i32)))
                                        as u16),
                                    2u16,
                                );
                                LoadPalette(
                                    (GetPartyMenuPalBufferPtr(
                                        ((((&raw const sPartyBoxSelectedForActionPalIds1)
                                            .cast::<u8>()
                                            .cast_mut())
                                        .cast::<u8>())
                                        .wrapping_offset(2))
                                        .read(),
                                    ))
                                    .cast::<u8>(),
                                    ((((((((&raw const sPartyBoxPalOffsets1)
                                        .cast::<u8>()
                                        .cast_mut())
                                    .cast::<u8>())
                                    .wrapping_offset(2))
                                    .read()) as i32)
                                        .wrapping_add(((palOffset) as i32)))
                                        as u16),
                                    2u16,
                                );
                            }
                            {
                                LoadPalette(
                                    (GetPartyMenuPalBufferPtr(
                                        (((&raw const sPartyBoxCurrSelectionPalIds2)
                                            .cast::<u8>()
                                            .cast_mut())
                                        .cast::<u8>())
                                        .read(),
                                    ))
                                    .cast::<u8>(),
                                    (((((((&raw const sPartyBoxPalOffsets2)
                                        .cast::<u8>()
                                        .cast_mut())
                                    .cast::<u8>())
                                    .read()) as i32)
                                        .wrapping_add(((palOffset) as i32)))
                                        as u16),
                                    2u16,
                                );
                                LoadPalette(
                                    (GetPartyMenuPalBufferPtr(
                                        ((((&raw const sPartyBoxCurrSelectionPalIds2)
                                            .cast::<u8>()
                                            .cast_mut())
                                        .cast::<u8>())
                                        .wrapping_offset(1))
                                        .read(),
                                    ))
                                    .cast::<u8>(),
                                    ((((((((&raw const sPartyBoxPalOffsets2)
                                        .cast::<u8>()
                                        .cast_mut())
                                    .cast::<u8>())
                                    .wrapping_offset(1))
                                    .read()) as i32)
                                        .wrapping_add(((palOffset) as i32)))
                                        as u16),
                                    2u16,
                                );
                                LoadPalette(
                                    (GetPartyMenuPalBufferPtr(
                                        ((((&raw const sPartyBoxCurrSelectionPalIds2)
                                            .cast::<u8>()
                                            .cast_mut())
                                        .cast::<u8>())
                                        .wrapping_offset(2))
                                        .read(),
                                    ))
                                    .cast::<u8>(),
                                    ((((((((&raw const sPartyBoxPalOffsets2)
                                        .cast::<u8>()
                                        .cast_mut())
                                    .cast::<u8>())
                                    .wrapping_offset(2))
                                    .read()) as i32)
                                        .wrapping_add(((palOffset) as i32)))
                                        as u16),
                                    2u16,
                                );
                            }
                        } else {
                            {
                                LoadPalette(
                                    (GetPartyMenuPalBufferPtr(
                                        (((&raw const sPartyBoxSelectedForActionPalIds1)
                                            .cast::<u8>()
                                            .cast_mut())
                                        .cast::<u8>())
                                        .read(),
                                    ))
                                    .cast::<u8>(),
                                    (((((((&raw const sPartyBoxPalOffsets1)
                                        .cast::<u8>()
                                        .cast_mut())
                                    .cast::<u8>())
                                    .read()) as i32)
                                        .wrapping_add(((palOffset) as i32)))
                                        as u16),
                                    2u16,
                                );
                                LoadPalette(
                                    (GetPartyMenuPalBufferPtr(
                                        ((((&raw const sPartyBoxSelectedForActionPalIds1)
                                            .cast::<u8>()
                                            .cast_mut())
                                        .cast::<u8>())
                                        .wrapping_offset(1))
                                        .read(),
                                    ))
                                    .cast::<u8>(),
                                    ((((((((&raw const sPartyBoxPalOffsets1)
                                        .cast::<u8>()
                                        .cast_mut())
                                    .cast::<u8>())
                                    .wrapping_offset(1))
                                    .read()) as i32)
                                        .wrapping_add(((palOffset) as i32)))
                                        as u16),
                                    2u16,
                                );
                                LoadPalette(
                                    (GetPartyMenuPalBufferPtr(
                                        ((((&raw const sPartyBoxSelectedForActionPalIds1)
                                            .cast::<u8>()
                                            .cast_mut())
                                        .cast::<u8>())
                                        .wrapping_offset(2))
                                        .read(),
                                    ))
                                    .cast::<u8>(),
                                    ((((((((&raw const sPartyBoxPalOffsets1)
                                        .cast::<u8>()
                                        .cast_mut())
                                    .cast::<u8>())
                                    .wrapping_offset(2))
                                    .read()) as i32)
                                        .wrapping_add(((palOffset) as i32)))
                                        as u16),
                                    2u16,
                                );
                            }
                            {
                                LoadPalette(
                                    (GetPartyMenuPalBufferPtr(
                                        (((&raw const sPartyBoxSelectedForActionPalIds2)
                                            .cast::<u8>()
                                            .cast_mut())
                                        .cast::<u8>())
                                        .read(),
                                    ))
                                    .cast::<u8>(),
                                    (((((((&raw const sPartyBoxPalOffsets2)
                                        .cast::<u8>()
                                        .cast_mut())
                                    .cast::<u8>())
                                    .read()) as i32)
                                        .wrapping_add(((palOffset) as i32)))
                                        as u16),
                                    2u16,
                                );
                                LoadPalette(
                                    (GetPartyMenuPalBufferPtr(
                                        ((((&raw const sPartyBoxSelectedForActionPalIds2)
                                            .cast::<u8>()
                                            .cast_mut())
                                        .cast::<u8>())
                                        .wrapping_offset(1))
                                        .read(),
                                    ))
                                    .cast::<u8>(),
                                    ((((((((&raw const sPartyBoxPalOffsets2)
                                        .cast::<u8>()
                                        .cast_mut())
                                    .cast::<u8>())
                                    .wrapping_offset(1))
                                    .read()) as i32)
                                        .wrapping_add(((palOffset) as i32)))
                                        as u16),
                                    2u16,
                                );
                                LoadPalette(
                                    (GetPartyMenuPalBufferPtr(
                                        ((((&raw const sPartyBoxSelectedForActionPalIds2)
                                            .cast::<u8>()
                                            .cast_mut())
                                        .cast::<u8>())
                                        .wrapping_offset(2))
                                        .read(),
                                    ))
                                    .cast::<u8>(),
                                    ((((((((&raw const sPartyBoxPalOffsets2)
                                        .cast::<u8>()
                                        .cast_mut())
                                    .cast::<u8>())
                                    .wrapping_offset(2))
                                    .read()) as i32)
                                        .wrapping_add(((palOffset) as i32)))
                                        as u16),
                                    2u16,
                                );
                            }
                        }
                    } else {
                        if (((palFlags) as i32) & 2i32) != 0 {
                            if (((palFlags) as i32) & 1i32) != 0 {
                                {
                                    LoadPalette(
                                        (GetPartyMenuPalBufferPtr(
                                            (((&raw const sPartyBoxCurrSelectionFaintedPalIds)
                                                .cast::<u8>()
                                                .cast_mut())
                                            .cast::<u8>())
                                            .read(),
                                        ))
                                        .cast::<u8>(),
                                        (((((((&raw const sPartyBoxPalOffsets1)
                                            .cast::<u8>()
                                            .cast_mut())
                                        .cast::<u8>())
                                        .read()) as i32)
                                            .wrapping_add(((palOffset) as i32)))
                                            as u16),
                                        2u16,
                                    );
                                    LoadPalette(
                                        (GetPartyMenuPalBufferPtr(
                                            ((((&raw const sPartyBoxCurrSelectionFaintedPalIds)
                                                .cast::<u8>()
                                                .cast_mut())
                                            .cast::<u8>())
                                            .wrapping_offset(1))
                                            .read(),
                                        ))
                                        .cast::<u8>(),
                                        ((((((((&raw const sPartyBoxPalOffsets1)
                                            .cast::<u8>()
                                            .cast_mut())
                                        .cast::<u8>())
                                        .wrapping_offset(1))
                                        .read()) as i32)
                                            .wrapping_add(((palOffset) as i32)))
                                            as u16),
                                        2u16,
                                    );
                                    LoadPalette(
                                        (GetPartyMenuPalBufferPtr(
                                            ((((&raw const sPartyBoxCurrSelectionFaintedPalIds)
                                                .cast::<u8>()
                                                .cast_mut())
                                            .cast::<u8>())
                                            .wrapping_offset(2))
                                            .read(),
                                        ))
                                        .cast::<u8>(),
                                        ((((((((&raw const sPartyBoxPalOffsets1)
                                            .cast::<u8>()
                                            .cast_mut())
                                        .cast::<u8>())
                                        .wrapping_offset(2))
                                        .read()) as i32)
                                            .wrapping_add(((palOffset) as i32)))
                                            as u16),
                                        2u16,
                                    );
                                }
                                {
                                    LoadPalette(
                                        (GetPartyMenuPalBufferPtr(
                                            (((&raw const sPartyBoxCurrSelectionPalIds2)
                                                .cast::<u8>()
                                                .cast_mut())
                                            .cast::<u8>())
                                            .read(),
                                        ))
                                        .cast::<u8>(),
                                        (((((((&raw const sPartyBoxPalOffsets2)
                                            .cast::<u8>()
                                            .cast_mut())
                                        .cast::<u8>())
                                        .read()) as i32)
                                            .wrapping_add(((palOffset) as i32)))
                                            as u16),
                                        2u16,
                                    );
                                    LoadPalette(
                                        (GetPartyMenuPalBufferPtr(
                                            ((((&raw const sPartyBoxCurrSelectionPalIds2)
                                                .cast::<u8>()
                                                .cast_mut())
                                            .cast::<u8>())
                                            .wrapping_offset(1))
                                            .read(),
                                        ))
                                        .cast::<u8>(),
                                        ((((((((&raw const sPartyBoxPalOffsets2)
                                            .cast::<u8>()
                                            .cast_mut())
                                        .cast::<u8>())
                                        .wrapping_offset(1))
                                        .read()) as i32)
                                            .wrapping_add(((palOffset) as i32)))
                                            as u16),
                                        2u16,
                                    );
                                    LoadPalette(
                                        (GetPartyMenuPalBufferPtr(
                                            ((((&raw const sPartyBoxCurrSelectionPalIds2)
                                                .cast::<u8>()
                                                .cast_mut())
                                            .cast::<u8>())
                                            .wrapping_offset(2))
                                            .read(),
                                        ))
                                        .cast::<u8>(),
                                        ((((((((&raw const sPartyBoxPalOffsets2)
                                            .cast::<u8>()
                                            .cast_mut())
                                        .cast::<u8>())
                                        .wrapping_offset(2))
                                        .read()) as i32)
                                            .wrapping_add(((palOffset) as i32)))
                                            as u16),
                                        2u16,
                                    );
                                }
                            } else {
                                {
                                    LoadPalette(
                                        (GetPartyMenuPalBufferPtr(
                                            (((&raw const sPartyBoxFaintedPalIds1)
                                                .cast::<u8>()
                                                .cast_mut())
                                            .cast::<u8>())
                                            .read(),
                                        ))
                                        .cast::<u8>(),
                                        (((((((&raw const sPartyBoxPalOffsets1)
                                            .cast::<u8>()
                                            .cast_mut())
                                        .cast::<u8>())
                                        .read()) as i32)
                                            .wrapping_add(((palOffset) as i32)))
                                            as u16),
                                        2u16,
                                    );
                                    LoadPalette(
                                        (GetPartyMenuPalBufferPtr(
                                            ((((&raw const sPartyBoxFaintedPalIds1)
                                                .cast::<u8>()
                                                .cast_mut())
                                            .cast::<u8>())
                                            .wrapping_offset(1))
                                            .read(),
                                        ))
                                        .cast::<u8>(),
                                        ((((((((&raw const sPartyBoxPalOffsets1)
                                            .cast::<u8>()
                                            .cast_mut())
                                        .cast::<u8>())
                                        .wrapping_offset(1))
                                        .read()) as i32)
                                            .wrapping_add(((palOffset) as i32)))
                                            as u16),
                                        2u16,
                                    );
                                    LoadPalette(
                                        (GetPartyMenuPalBufferPtr(
                                            ((((&raw const sPartyBoxFaintedPalIds1)
                                                .cast::<u8>()
                                                .cast_mut())
                                            .cast::<u8>())
                                            .wrapping_offset(2))
                                            .read(),
                                        ))
                                        .cast::<u8>(),
                                        ((((((((&raw const sPartyBoxPalOffsets1)
                                            .cast::<u8>()
                                            .cast_mut())
                                        .cast::<u8>())
                                        .wrapping_offset(2))
                                        .read()) as i32)
                                            .wrapping_add(((palOffset) as i32)))
                                            as u16),
                                        2u16,
                                    );
                                }
                                {
                                    LoadPalette(
                                        (GetPartyMenuPalBufferPtr(
                                            (((&raw const sPartyBoxFaintedPalIds2)
                                                .cast::<u8>()
                                                .cast_mut())
                                            .cast::<u8>())
                                            .read(),
                                        ))
                                        .cast::<u8>(),
                                        (((((((&raw const sPartyBoxPalOffsets2)
                                            .cast::<u8>()
                                            .cast_mut())
                                        .cast::<u8>())
                                        .read()) as i32)
                                            .wrapping_add(((palOffset) as i32)))
                                            as u16),
                                        2u16,
                                    );
                                    LoadPalette(
                                        (GetPartyMenuPalBufferPtr(
                                            ((((&raw const sPartyBoxFaintedPalIds2)
                                                .cast::<u8>()
                                                .cast_mut())
                                            .cast::<u8>())
                                            .wrapping_offset(1))
                                            .read(),
                                        ))
                                        .cast::<u8>(),
                                        ((((((((&raw const sPartyBoxPalOffsets2)
                                            .cast::<u8>()
                                            .cast_mut())
                                        .cast::<u8>())
                                        .wrapping_offset(1))
                                        .read()) as i32)
                                            .wrapping_add(((palOffset) as i32)))
                                            as u16),
                                        2u16,
                                    );
                                    LoadPalette(
                                        (GetPartyMenuPalBufferPtr(
                                            ((((&raw const sPartyBoxFaintedPalIds2)
                                                .cast::<u8>()
                                                .cast_mut())
                                            .cast::<u8>())
                                            .wrapping_offset(2))
                                            .read(),
                                        ))
                                        .cast::<u8>(),
                                        ((((((((&raw const sPartyBoxPalOffsets2)
                                            .cast::<u8>()
                                            .cast_mut())
                                        .cast::<u8>())
                                        .wrapping_offset(2))
                                        .read()) as i32)
                                            .wrapping_add(((palOffset) as i32)))
                                            as u16),
                                        2u16,
                                    );
                                }
                            }
                        } else {
                            if (((palFlags) as i32) & 8i32) != 0 {
                                if (((palFlags) as i32) & 1i32) != 0 {
                                    {
                                        LoadPalette(
                                            (GetPartyMenuPalBufferPtr(
                                                (((&raw const sPartyBoxCurrSelectionMultiPalIds)
                                                    .cast::<u8>()
                                                    .cast_mut())
                                                .cast::<u8>())
                                                .read(),
                                            ))
                                            .cast::<u8>(),
                                            (((((((&raw const sPartyBoxPalOffsets1)
                                                .cast::<u8>()
                                                .cast_mut())
                                            .cast::<u8>())
                                            .read())
                                                as i32)
                                                .wrapping_add(((palOffset) as i32)))
                                                as u16),
                                            2u16,
                                        );
                                        LoadPalette((GetPartyMenuPalBufferPtr(((((&raw const sPartyBoxCurrSelectionMultiPalIds).cast::<u8>().cast_mut()).cast::<u8>()).wrapping_offset(1)).read())).cast::<u8>(), ((((((((((&raw const sPartyBoxPalOffsets1).cast::<u8>().cast_mut()).cast::<u8>()).wrapping_offset(1)).read()) as i32))).wrapping_add((((palOffset) as i32)))) as u16), 2u16);
                                        LoadPalette((GetPartyMenuPalBufferPtr(((((&raw const sPartyBoxCurrSelectionMultiPalIds).cast::<u8>().cast_mut()).cast::<u8>()).wrapping_offset(2)).read())).cast::<u8>(), ((((((((((&raw const sPartyBoxPalOffsets1).cast::<u8>().cast_mut()).cast::<u8>()).wrapping_offset(2)).read()) as i32))).wrapping_add((((palOffset) as i32)))) as u16), 2u16);
                                    }
                                    {
                                        LoadPalette(
                                            (GetPartyMenuPalBufferPtr(
                                                (((&raw const sPartyBoxCurrSelectionPalIds2)
                                                    .cast::<u8>()
                                                    .cast_mut())
                                                .cast::<u8>())
                                                .read(),
                                            ))
                                            .cast::<u8>(),
                                            (((((((&raw const sPartyBoxPalOffsets2)
                                                .cast::<u8>()
                                                .cast_mut())
                                            .cast::<u8>())
                                            .read())
                                                as i32)
                                                .wrapping_add(((palOffset) as i32)))
                                                as u16),
                                            2u16,
                                        );
                                        LoadPalette(
                                            (GetPartyMenuPalBufferPtr(
                                                ((((&raw const sPartyBoxCurrSelectionPalIds2)
                                                    .cast::<u8>()
                                                    .cast_mut())
                                                .cast::<u8>())
                                                .wrapping_offset(1))
                                                .read(),
                                            ))
                                            .cast::<u8>(),
                                            ((((((((&raw const sPartyBoxPalOffsets2)
                                                .cast::<u8>()
                                                .cast_mut())
                                            .cast::<u8>())
                                            .wrapping_offset(1))
                                            .read())
                                                as i32)
                                                .wrapping_add(((palOffset) as i32)))
                                                as u16),
                                            2u16,
                                        );
                                        LoadPalette(
                                            (GetPartyMenuPalBufferPtr(
                                                ((((&raw const sPartyBoxCurrSelectionPalIds2)
                                                    .cast::<u8>()
                                                    .cast_mut())
                                                .cast::<u8>())
                                                .wrapping_offset(2))
                                                .read(),
                                            ))
                                            .cast::<u8>(),
                                            ((((((((&raw const sPartyBoxPalOffsets2)
                                                .cast::<u8>()
                                                .cast_mut())
                                            .cast::<u8>())
                                            .wrapping_offset(2))
                                            .read())
                                                as i32)
                                                .wrapping_add(((palOffset) as i32)))
                                                as u16),
                                            2u16,
                                        );
                                    }
                                } else {
                                    {
                                        LoadPalette(
                                            (GetPartyMenuPalBufferPtr(
                                                (((&raw const sPartyBoxMultiPalIds1)
                                                    .cast::<u8>()
                                                    .cast_mut())
                                                .cast::<u8>())
                                                .read(),
                                            ))
                                            .cast::<u8>(),
                                            (((((((&raw const sPartyBoxPalOffsets1)
                                                .cast::<u8>()
                                                .cast_mut())
                                            .cast::<u8>())
                                            .read())
                                                as i32)
                                                .wrapping_add(((palOffset) as i32)))
                                                as u16),
                                            2u16,
                                        );
                                        LoadPalette(
                                            (GetPartyMenuPalBufferPtr(
                                                ((((&raw const sPartyBoxMultiPalIds1)
                                                    .cast::<u8>()
                                                    .cast_mut())
                                                .cast::<u8>())
                                                .wrapping_offset(1))
                                                .read(),
                                            ))
                                            .cast::<u8>(),
                                            ((((((((&raw const sPartyBoxPalOffsets1)
                                                .cast::<u8>()
                                                .cast_mut())
                                            .cast::<u8>())
                                            .wrapping_offset(1))
                                            .read())
                                                as i32)
                                                .wrapping_add(((palOffset) as i32)))
                                                as u16),
                                            2u16,
                                        );
                                        LoadPalette(
                                            (GetPartyMenuPalBufferPtr(
                                                ((((&raw const sPartyBoxMultiPalIds1)
                                                    .cast::<u8>()
                                                    .cast_mut())
                                                .cast::<u8>())
                                                .wrapping_offset(2))
                                                .read(),
                                            ))
                                            .cast::<u8>(),
                                            ((((((((&raw const sPartyBoxPalOffsets1)
                                                .cast::<u8>()
                                                .cast_mut())
                                            .cast::<u8>())
                                            .wrapping_offset(2))
                                            .read())
                                                as i32)
                                                .wrapping_add(((palOffset) as i32)))
                                                as u16),
                                            2u16,
                                        );
                                    }
                                    {
                                        LoadPalette(
                                            (GetPartyMenuPalBufferPtr(
                                                (((&raw const sPartyBoxMultiPalIds2)
                                                    .cast::<u8>()
                                                    .cast_mut())
                                                .cast::<u8>())
                                                .read(),
                                            ))
                                            .cast::<u8>(),
                                            (((((((&raw const sPartyBoxPalOffsets2)
                                                .cast::<u8>()
                                                .cast_mut())
                                            .cast::<u8>())
                                            .read())
                                                as i32)
                                                .wrapping_add(((palOffset) as i32)))
                                                as u16),
                                            2u16,
                                        );
                                        LoadPalette(
                                            (GetPartyMenuPalBufferPtr(
                                                ((((&raw const sPartyBoxMultiPalIds2)
                                                    .cast::<u8>()
                                                    .cast_mut())
                                                .cast::<u8>())
                                                .wrapping_offset(1))
                                                .read(),
                                            ))
                                            .cast::<u8>(),
                                            ((((((((&raw const sPartyBoxPalOffsets2)
                                                .cast::<u8>()
                                                .cast_mut())
                                            .cast::<u8>())
                                            .wrapping_offset(1))
                                            .read())
                                                as i32)
                                                .wrapping_add(((palOffset) as i32)))
                                                as u16),
                                            2u16,
                                        );
                                        LoadPalette(
                                            (GetPartyMenuPalBufferPtr(
                                                ((((&raw const sPartyBoxMultiPalIds2)
                                                    .cast::<u8>()
                                                    .cast_mut())
                                                .cast::<u8>())
                                                .wrapping_offset(2))
                                                .read(),
                                            ))
                                            .cast::<u8>(),
                                            ((((((((&raw const sPartyBoxPalOffsets2)
                                                .cast::<u8>()
                                                .cast_mut())
                                            .cast::<u8>())
                                            .wrapping_offset(2))
                                            .read())
                                                as i32)
                                                .wrapping_add(((palOffset) as i32)))
                                                as u16),
                                            2u16,
                                        );
                                    }
                                }
                            } else {
                                if (((palFlags) as i32) & 1i32) != 0 {
                                    {
                                        LoadPalette(
                                            (GetPartyMenuPalBufferPtr(
                                                (((&raw const sPartyBoxCurrSelectionPalIds1)
                                                    .cast::<u8>()
                                                    .cast_mut())
                                                .cast::<u8>())
                                                .read(),
                                            ))
                                            .cast::<u8>(),
                                            (((((((&raw const sPartyBoxPalOffsets1)
                                                .cast::<u8>()
                                                .cast_mut())
                                            .cast::<u8>())
                                            .read())
                                                as i32)
                                                .wrapping_add(((palOffset) as i32)))
                                                as u16),
                                            2u16,
                                        );
                                        LoadPalette(
                                            (GetPartyMenuPalBufferPtr(
                                                ((((&raw const sPartyBoxCurrSelectionPalIds1)
                                                    .cast::<u8>()
                                                    .cast_mut())
                                                .cast::<u8>())
                                                .wrapping_offset(1))
                                                .read(),
                                            ))
                                            .cast::<u8>(),
                                            ((((((((&raw const sPartyBoxPalOffsets1)
                                                .cast::<u8>()
                                                .cast_mut())
                                            .cast::<u8>())
                                            .wrapping_offset(1))
                                            .read())
                                                as i32)
                                                .wrapping_add(((palOffset) as i32)))
                                                as u16),
                                            2u16,
                                        );
                                        LoadPalette(
                                            (GetPartyMenuPalBufferPtr(
                                                ((((&raw const sPartyBoxCurrSelectionPalIds1)
                                                    .cast::<u8>()
                                                    .cast_mut())
                                                .cast::<u8>())
                                                .wrapping_offset(2))
                                                .read(),
                                            ))
                                            .cast::<u8>(),
                                            ((((((((&raw const sPartyBoxPalOffsets1)
                                                .cast::<u8>()
                                                .cast_mut())
                                            .cast::<u8>())
                                            .wrapping_offset(2))
                                            .read())
                                                as i32)
                                                .wrapping_add(((palOffset) as i32)))
                                                as u16),
                                            2u16,
                                        );
                                    }
                                    {
                                        LoadPalette(
                                            (GetPartyMenuPalBufferPtr(
                                                (((&raw const sPartyBoxCurrSelectionPalIds2)
                                                    .cast::<u8>()
                                                    .cast_mut())
                                                .cast::<u8>())
                                                .read(),
                                            ))
                                            .cast::<u8>(),
                                            (((((((&raw const sPartyBoxPalOffsets2)
                                                .cast::<u8>()
                                                .cast_mut())
                                            .cast::<u8>())
                                            .read())
                                                as i32)
                                                .wrapping_add(((palOffset) as i32)))
                                                as u16),
                                            2u16,
                                        );
                                        LoadPalette(
                                            (GetPartyMenuPalBufferPtr(
                                                ((((&raw const sPartyBoxCurrSelectionPalIds2)
                                                    .cast::<u8>()
                                                    .cast_mut())
                                                .cast::<u8>())
                                                .wrapping_offset(1))
                                                .read(),
                                            ))
                                            .cast::<u8>(),
                                            ((((((((&raw const sPartyBoxPalOffsets2)
                                                .cast::<u8>()
                                                .cast_mut())
                                            .cast::<u8>())
                                            .wrapping_offset(1))
                                            .read())
                                                as i32)
                                                .wrapping_add(((palOffset) as i32)))
                                                as u16),
                                            2u16,
                                        );
                                        LoadPalette(
                                            (GetPartyMenuPalBufferPtr(
                                                ((((&raw const sPartyBoxCurrSelectionPalIds2)
                                                    .cast::<u8>()
                                                    .cast_mut())
                                                .cast::<u8>())
                                                .wrapping_offset(2))
                                                .read(),
                                            ))
                                            .cast::<u8>(),
                                            ((((((((&raw const sPartyBoxPalOffsets2)
                                                .cast::<u8>()
                                                .cast_mut())
                                            .cast::<u8>())
                                            .wrapping_offset(2))
                                            .read())
                                                as i32)
                                                .wrapping_add(((palOffset) as i32)))
                                                as u16),
                                            2u16,
                                        );
                                    }
                                } else {
                                    {
                                        LoadPalette(
                                            (GetPartyMenuPalBufferPtr(
                                                (((&raw const sPartyBoxEmptySlotPalIds1)
                                                    .cast::<u8>()
                                                    .cast_mut())
                                                .cast::<u8>())
                                                .read(),
                                            ))
                                            .cast::<u8>(),
                                            (((((((&raw const sPartyBoxPalOffsets1)
                                                .cast::<u8>()
                                                .cast_mut())
                                            .cast::<u8>())
                                            .read())
                                                as i32)
                                                .wrapping_add(((palOffset) as i32)))
                                                as u16),
                                            2u16,
                                        );
                                        LoadPalette(
                                            (GetPartyMenuPalBufferPtr(
                                                ((((&raw const sPartyBoxEmptySlotPalIds1)
                                                    .cast::<u8>()
                                                    .cast_mut())
                                                .cast::<u8>())
                                                .wrapping_offset(1))
                                                .read(),
                                            ))
                                            .cast::<u8>(),
                                            ((((((((&raw const sPartyBoxPalOffsets1)
                                                .cast::<u8>()
                                                .cast_mut())
                                            .cast::<u8>())
                                            .wrapping_offset(1))
                                            .read())
                                                as i32)
                                                .wrapping_add(((palOffset) as i32)))
                                                as u16),
                                            2u16,
                                        );
                                        LoadPalette(
                                            (GetPartyMenuPalBufferPtr(
                                                ((((&raw const sPartyBoxEmptySlotPalIds1)
                                                    .cast::<u8>()
                                                    .cast_mut())
                                                .cast::<u8>())
                                                .wrapping_offset(2))
                                                .read(),
                                            ))
                                            .cast::<u8>(),
                                            ((((((((&raw const sPartyBoxPalOffsets1)
                                                .cast::<u8>()
                                                .cast_mut())
                                            .cast::<u8>())
                                            .wrapping_offset(2))
                                            .read())
                                                as i32)
                                                .wrapping_add(((palOffset) as i32)))
                                                as u16),
                                            2u16,
                                        );
                                    }
                                    {
                                        LoadPalette(
                                            (GetPartyMenuPalBufferPtr(
                                                (((&raw const sPartyBoxEmptySlotPalIds2)
                                                    .cast::<u8>()
                                                    .cast_mut())
                                                .cast::<u8>())
                                                .read(),
                                            ))
                                            .cast::<u8>(),
                                            (((((((&raw const sPartyBoxPalOffsets2)
                                                .cast::<u8>()
                                                .cast_mut())
                                            .cast::<u8>())
                                            .read())
                                                as i32)
                                                .wrapping_add(((palOffset) as i32)))
                                                as u16),
                                            2u16,
                                        );
                                        LoadPalette(
                                            (GetPartyMenuPalBufferPtr(
                                                ((((&raw const sPartyBoxEmptySlotPalIds2)
                                                    .cast::<u8>()
                                                    .cast_mut())
                                                .cast::<u8>())
                                                .wrapping_offset(1))
                                                .read(),
                                            ))
                                            .cast::<u8>(),
                                            ((((((((&raw const sPartyBoxPalOffsets2)
                                                .cast::<u8>()
                                                .cast_mut())
                                            .cast::<u8>())
                                            .wrapping_offset(1))
                                            .read())
                                                as i32)
                                                .wrapping_add(((palOffset) as i32)))
                                                as u16),
                                            2u16,
                                        );
                                        LoadPalette(
                                            (GetPartyMenuPalBufferPtr(
                                                ((((&raw const sPartyBoxEmptySlotPalIds2)
                                                    .cast::<u8>()
                                                    .cast_mut())
                                                .cast::<u8>())
                                                .wrapping_offset(2))
                                                .read(),
                                            ))
                                            .cast::<u8>(),
                                            ((((((((&raw const sPartyBoxPalOffsets2)
                                                .cast::<u8>()
                                                .cast_mut())
                                            .cast::<u8>())
                                            .wrapping_offset(2))
                                            .read())
                                                as i32)
                                                .wrapping_add(((palOffset) as i32)))
                                                as u16),
                                            2u16,
                                        );
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
pub(crate) unsafe extern "C" fn DisplayPartyPokemonBarDetail(
    windowId: u8,
    str: *mut u8,
    color: u8,
    align: *mut u8,
) {
    unsafe {
        let mut windowId = windowId;
        let mut str = str;
        let mut color = color;
        let mut align = align;
        AddTextPrinterParameterized3(
            windowId,
            0u8,
            (align).read(),
            ((align).wrapping_offset(1)).read(),
            ((((&raw const sFontColorTable).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((color) as i32) as isize * 3))
            .cast::<u8>(),
            0i8,
            str,
        );
    }
}
pub(crate) unsafe extern "C" fn DisplayPartyPokemonNickname(mon: *mut u8, menuBox: *mut u8, c: u8) {
    unsafe {
        let mut mon = mon;
        let mut menuBox = menuBox;
        let mut c = c;
        let mut nickname = crate::ffi::Align4([0u8; 11]);
        if GetMonData2(mon, 11i32) != 0u32 {
            if ((c) as i32) == 1i32 {
                (((((menuBox).cast::<*mut u8>()).read())
                    .cast::<Option<unsafe extern "C" fn(u8, u8, u8, u8, u8, u8)>>())
                .read())
                .unwrap_unchecked()(
                    ((menuBox).wrapping_add(8)).read(),
                    (((((((((menuBox).cast::<*mut u8>()).read()).wrapping_add(4)).cast::<u8>())
                        .read()) as i32)
                        >> 3) as u8),
                    ((((((((((menuBox).cast::<*mut u8>()).read()).wrapping_add(4)).cast::<u8>())
                        .wrapping_offset(1))
                    .read()) as i32)
                        >> 3) as u8),
                    ((((((((((menuBox).cast::<*mut u8>()).read()).wrapping_add(4)).cast::<u8>())
                        .wrapping_offset(2))
                    .read()) as i32)
                        >> 3) as u8),
                    ((((((((((menuBox).cast::<*mut u8>()).read()).wrapping_add(4)).cast::<u8>())
                        .wrapping_offset(3))
                    .read()) as i32)
                        >> 3) as u8),
                    0u8,
                );
            }
            GetMonNickname(mon, (&raw mut nickname).cast::<u8>());
            DisplayPartyPokemonBarDetail(
                ((menuBox).wrapping_add(8)).read(),
                (&raw mut nickname).cast::<u8>(),
                0u8,
                ((((menuBox).cast::<*mut u8>()).read()).wrapping_add(4)).cast::<u8>(),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn DisplayPartyPokemonLevelCheck(
    mon: *mut u8,
    menuBox: *mut u8,
    c: u8,
) {
    unsafe {
        let mut mon = mon;
        let mut menuBox = menuBox;
        let mut c = c;
        if GetMonData2(mon, 11i32) != 0u32 {
            let mut ailment: u8 = GetMonAilment(mon);
            if (((ailment) as i32) == 0i32) || (((ailment) as i32) == 6i32) {
                if ((c) as i32) != 0i32 {
                    (((((menuBox).cast::<*mut u8>()).read())
                        .cast::<Option<unsafe extern "C" fn(u8, u8, u8, u8, u8, u8)>>())
                    .read())
                    .unwrap_unchecked()(
                        ((menuBox).wrapping_add(8)).read(),
                        ((((((((((menuBox).cast::<*mut u8>()).read()).wrapping_add(4))
                            .cast::<u8>())
                        .wrapping_offset(4))
                        .read()) as i32)
                            >> 3) as u8),
                        (((((((((((menuBox).cast::<*mut u8>()).read()).wrapping_add(4))
                            .cast::<u8>())
                        .wrapping_offset(5))
                        .read()) as i32)
                            >> 3)
                            .wrapping_add(1i32)) as u8),
                        ((((((((((menuBox).cast::<*mut u8>()).read()).wrapping_add(4))
                            .cast::<u8>())
                        .wrapping_offset(6))
                        .read()) as i32)
                            >> 3) as u8),
                        ((((((((((menuBox).cast::<*mut u8>()).read()).wrapping_add(4))
                            .cast::<u8>())
                        .wrapping_offset(7))
                        .read()) as i32)
                            >> 3) as u8),
                        0u8,
                    );
                }
                if ((c) as i32) != 2i32 {
                    DisplayPartyPokemonLevel(((GetMonData2(mon, 56i32)) as u8), menuBox);
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn DisplayPartyPokemonLevel(level: u8, menuBox: *mut u8) {
    unsafe {
        let mut level = level;
        let mut menuBox = menuBox;
        ConvertIntToDecimalStringN(
            (&raw mut gStringVar2).cast::<u8>(),
            ((level) as i32),
            0i32,
            3u8,
        );
        StringCopy(
            (&raw mut gStringVar1).cast::<u8>(),
            (&raw mut gText_LevelSymbol).cast::<u8>(),
        );
        StringAppend(
            (&raw mut gStringVar1).cast::<u8>(),
            (&raw mut gStringVar2).cast::<u8>(),
        );
        DisplayPartyPokemonBarDetail(
            ((menuBox).wrapping_add(8)).read(),
            (&raw mut gStringVar1).cast::<u8>(),
            0u8,
            (((((menuBox).cast::<*mut u8>()).read()).wrapping_add(4)).cast::<u8>())
                .wrapping_offset(4),
        );
    }
}
pub(crate) unsafe extern "C" fn DisplayPartyPokemonGenderNidoranCheck(
    mon: *mut u8,
    menuBox: *mut u8,
    c: u8,
) {
    unsafe {
        let mut mon = mon;
        let mut menuBox = menuBox;
        let mut c = c;
        let mut nickname = crate::ffi::Align4([0u8; 11]);
        if ((c) as i32) == 1i32 {
            (((((menuBox).cast::<*mut u8>()).read())
                .cast::<Option<unsafe extern "C" fn(u8, u8, u8, u8, u8, u8)>>())
            .read())
            .unwrap_unchecked()(
                ((menuBox).wrapping_add(8)).read(),
                ((((((((((menuBox).cast::<*mut u8>()).read()).wrapping_add(4)).cast::<u8>())
                    .wrapping_offset(8))
                .read()) as i32)
                    >> 3) as u8),
                (((((((((((menuBox).cast::<*mut u8>()).read()).wrapping_add(4)).cast::<u8>())
                    .wrapping_offset(9))
                .read()) as i32)
                    >> 3)
                    .wrapping_add(1i32)) as u8),
                ((((((((((menuBox).cast::<*mut u8>()).read()).wrapping_add(4)).cast::<u8>())
                    .wrapping_offset(10))
                .read()) as i32)
                    >> 3) as u8),
                ((((((((((menuBox).cast::<*mut u8>()).read()).wrapping_add(4)).cast::<u8>())
                    .wrapping_offset(11))
                .read()) as i32)
                    >> 3) as u8),
                0u8,
            );
        }
        GetMonNickname(mon, (&raw mut nickname).cast::<u8>());
        DisplayPartyPokemonGender(
            GetMonGender(mon),
            ((GetMonData2(mon, 11i32)) as u16),
            (&raw mut nickname).cast::<u8>(),
            menuBox,
        );
    }
}
pub(crate) unsafe extern "C" fn DisplayPartyPokemonGender(
    gender: u8,
    species: u16,
    nickname: *mut u8,
    menuBox: *mut u8,
) {
    unsafe {
        let mut gender = gender;
        let mut species = species;
        let mut nickname = nickname;
        let mut menuBox = menuBox;
        let mut palOffset: u8 = (((0u32).wrapping_add(
            (GetWindowAttribute(((menuBox).wrapping_add(8)).read(), 5u8)).wrapping_mul(16u32),
        )) as u8);
        if ((species) as i32) == 0i32 {
            return;
        }
        if ((((species) as i32) == 32i32) || (((species) as i32) == 29i32))
            && (StringCompare(
                nickname,
                (((&raw mut gSpeciesNames).cast::<u8>())
                    .wrapping_offset(((species) as i32) as isize * 11))
                .cast::<u8>(),
            ) == 0i32)
        {
            return;
        }
        'l1: {
            let __sw1 = ((gender) as i32);
            if __sw1 == 0i32 {
                LoadPalette(
                    (GetPartyMenuPalBufferPtr(
                        (((&raw const sGenderMalePalIds).cast::<u8>().cast_mut()).cast::<u8>())
                            .read(),
                    ))
                    .cast::<u8>(),
                    (((((((&raw const sGenderPalOffsets).cast::<u8>().cast_mut()).cast::<u8>())
                        .read()) as i32)
                        .wrapping_add(((palOffset) as i32))) as u16),
                    2u16,
                );
                LoadPalette(
                    (GetPartyMenuPalBufferPtr(
                        ((((&raw const sGenderMalePalIds).cast::<u8>().cast_mut()).cast::<u8>())
                            .wrapping_offset(1))
                        .read(),
                    ))
                    .cast::<u8>(),
                    ((((((((&raw const sGenderPalOffsets).cast::<u8>().cast_mut()).cast::<u8>())
                        .wrapping_offset(1))
                    .read()) as i32)
                        .wrapping_add(((palOffset) as i32))) as u16),
                    2u16,
                );
                DisplayPartyPokemonBarDetail(
                    ((menuBox).wrapping_add(8)).read(),
                    (&raw mut gText_MaleSymbol).cast::<u8>(),
                    2u8,
                    (((((menuBox).cast::<*mut u8>()).read()).wrapping_add(4)).cast::<u8>())
                        .wrapping_offset(8),
                );
                break 'l1;
            }
            if __sw1 == 254i32 {
                LoadPalette(
                    (GetPartyMenuPalBufferPtr(
                        (((&raw const sGenderFemalePalIds).cast::<u8>().cast_mut()).cast::<u8>())
                            .read(),
                    ))
                    .cast::<u8>(),
                    (((((((&raw const sGenderPalOffsets).cast::<u8>().cast_mut()).cast::<u8>())
                        .read()) as i32)
                        .wrapping_add(((palOffset) as i32))) as u16),
                    2u16,
                );
                LoadPalette(
                    (GetPartyMenuPalBufferPtr(
                        ((((&raw const sGenderFemalePalIds).cast::<u8>().cast_mut()).cast::<u8>())
                            .wrapping_offset(1))
                        .read(),
                    ))
                    .cast::<u8>(),
                    ((((((((&raw const sGenderPalOffsets).cast::<u8>().cast_mut()).cast::<u8>())
                        .wrapping_offset(1))
                    .read()) as i32)
                        .wrapping_add(((palOffset) as i32))) as u16),
                    2u16,
                );
                DisplayPartyPokemonBarDetail(
                    ((menuBox).wrapping_add(8)).read(),
                    (&raw mut gText_FemaleSymbol).cast::<u8>(),
                    2u8,
                    (((((menuBox).cast::<*mut u8>()).read()).wrapping_add(4)).cast::<u8>())
                        .wrapping_offset(8),
                );
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn DisplayPartyPokemonHPCheck(mon: *mut u8, menuBox: *mut u8, c: u8) {
    unsafe {
        let mut mon = mon;
        let mut menuBox = menuBox;
        let mut c = c;
        if GetMonData2(mon, 11i32) != 0u32 {
            if ((c) as i32) != 0i32 {
                (((((menuBox).cast::<*mut u8>()).read())
                    .cast::<Option<unsafe extern "C" fn(u8, u8, u8, u8, u8, u8)>>())
                .read())
                .unwrap_unchecked()(
                    ((menuBox).wrapping_add(8)).read(),
                    ((((((((((menuBox).cast::<*mut u8>()).read()).wrapping_add(4)).cast::<u8>())
                        .wrapping_offset(12))
                    .read()) as i32)
                        >> 3) as u8),
                    (((((((((((menuBox).cast::<*mut u8>()).read()).wrapping_add(4)).cast::<u8>())
                        .wrapping_offset(13))
                    .read()) as i32)
                        >> 3)
                        .wrapping_add(1i32)) as u8),
                    ((((((((((menuBox).cast::<*mut u8>()).read()).wrapping_add(4)).cast::<u8>())
                        .wrapping_offset(14))
                    .read()) as i32)
                        >> 3) as u8),
                    ((((((((((menuBox).cast::<*mut u8>()).read()).wrapping_add(4)).cast::<u8>())
                        .wrapping_offset(15))
                    .read()) as i32)
                        >> 3) as u8),
                    0u8,
                );
            }
            if ((c) as i32) != 2i32 {
                DisplayPartyPokemonHP(((GetMonData2(mon, 57i32)) as u16), menuBox);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn DisplayPartyPokemonHP(hp: u16, menuBox: *mut u8) {
    unsafe {
        let mut hp = hp;
        let mut menuBox = menuBox;
        let mut strOut: *mut u8 = ConvertIntToDecimalStringN(
            (&raw mut gStringVar1).cast::<u8>(),
            ((hp) as i32),
            1i32,
            3u8,
        );
        (strOut).write(186u8);
        ((strOut).wrapping_offset(1)).write(255u8);
        DisplayPartyPokemonBarDetail(
            ((menuBox).wrapping_add(8)).read(),
            (&raw mut gStringVar1).cast::<u8>(),
            0u8,
            (((((menuBox).cast::<*mut u8>()).read()).wrapping_add(4)).cast::<u8>())
                .wrapping_offset(12),
        );
    }
}
pub(crate) unsafe extern "C" fn DisplayPartyPokemonMaxHPCheck(
    mon: *mut u8,
    menuBox: *mut u8,
    c: u8,
) {
    unsafe {
        let mut mon = mon;
        let mut menuBox = menuBox;
        let mut c = c;
        if GetMonData2(mon, 11i32) != 0u32 {
            if ((c) as i32) != 0i32 {
                (((((menuBox).cast::<*mut u8>()).read())
                    .cast::<Option<unsafe extern "C" fn(u8, u8, u8, u8, u8, u8)>>())
                .read())
                .unwrap_unchecked()(
                    ((menuBox).wrapping_add(8)).read(),
                    (((((((((((menuBox).cast::<*mut u8>()).read()).wrapping_add(4)).cast::<u8>())
                        .wrapping_offset(16))
                    .read()) as i32)
                        >> 3)
                        .wrapping_add(1i32)) as u8),
                    (((((((((((menuBox).cast::<*mut u8>()).read()).wrapping_add(4)).cast::<u8>())
                        .wrapping_offset(17))
                    .read()) as i32)
                        >> 3)
                        .wrapping_add(1i32)) as u8),
                    ((((((((((menuBox).cast::<*mut u8>()).read()).wrapping_add(4)).cast::<u8>())
                        .wrapping_offset(18))
                    .read()) as i32)
                        >> 3) as u8),
                    ((((((((((menuBox).cast::<*mut u8>()).read()).wrapping_add(4)).cast::<u8>())
                        .wrapping_offset(19))
                    .read()) as i32)
                        >> 3) as u8),
                    0u8,
                );
            }
            if ((c) as i32) != 2i32 {
                DisplayPartyPokemonMaxHP(((GetMonData2(mon, 58i32)) as u16), menuBox);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn DisplayPartyPokemonMaxHP(maxhp: u16, menuBox: *mut u8) {
    unsafe {
        let mut maxhp = maxhp;
        let mut menuBox = menuBox;
        ConvertIntToDecimalStringN(
            (&raw mut gStringVar2).cast::<u8>(),
            ((maxhp) as i32),
            1i32,
            3u8,
        );
        StringCopy(
            (&raw mut gStringVar1).cast::<u8>(),
            (&raw mut gText_Slash).cast::<u8>(),
        );
        StringAppend(
            (&raw mut gStringVar1).cast::<u8>(),
            (&raw mut gStringVar2).cast::<u8>(),
        );
        DisplayPartyPokemonBarDetail(
            ((menuBox).wrapping_add(8)).read(),
            (&raw mut gStringVar1).cast::<u8>(),
            0u8,
            (((((menuBox).cast::<*mut u8>()).read()).wrapping_add(4)).cast::<u8>())
                .wrapping_offset(16),
        );
    }
}
pub(crate) unsafe extern "C" fn DisplayPartyPokemonHPBarCheck(mon: *mut u8, menuBox: *mut u8) {
    unsafe {
        let mut mon = mon;
        let mut menuBox = menuBox;
        if GetMonData2(mon, 11i32) != 0u32 {
            DisplayPartyPokemonHPBar(
                ((GetMonData2(mon, 57i32)) as u16),
                ((GetMonData2(mon, 58i32)) as u16),
                menuBox,
            );
        }
    }
}
pub(crate) unsafe extern "C" fn DisplayPartyPokemonHPBar(hp: u16, maxhp: u16, menuBox: *mut u8) {
    unsafe {
        let mut hp = hp;
        let mut maxhp = maxhp;
        let mut menuBox = menuBox;
        let mut palOffset: u8 = (((0u32).wrapping_add(
            (GetWindowAttribute(((menuBox).wrapping_add(8)).read(), 5u8)).wrapping_mul(16u32),
        )) as u8);
        let mut hpFraction: u8 = 0u8;
        'l1: {
            let __sw1 = ((GetHPBarLevel(((hp) as i16), ((maxhp) as i16))) as i32);
            let __matched = __sw1 == 3i32 || __sw1 == 4i32 || __sw1 == 2i32;
            if __sw1 == 3i32 || __sw1 == 4i32 {
                LoadPalette(
                    (GetPartyMenuPalBufferPtr(
                        (((&raw const sHPBarGreenPalIds).cast::<u8>().cast_mut()).cast::<u8>())
                            .read(),
                    ))
                    .cast::<u8>(),
                    (((((((&raw const sHPBarPalOffsets).cast::<u8>().cast_mut()).cast::<u8>())
                        .read()) as i32)
                        .wrapping_add(((palOffset) as i32))) as u16),
                    2u16,
                );
                LoadPalette(
                    (GetPartyMenuPalBufferPtr(
                        ((((&raw const sHPBarGreenPalIds).cast::<u8>().cast_mut()).cast::<u8>())
                            .wrapping_offset(1))
                        .read(),
                    ))
                    .cast::<u8>(),
                    ((((((((&raw const sHPBarPalOffsets).cast::<u8>().cast_mut()).cast::<u8>())
                        .wrapping_offset(1))
                    .read()) as i32)
                        .wrapping_add(((palOffset) as i32))) as u16),
                    2u16,
                );
                break 'l1;
            }
            if __sw1 == 2i32 {
                LoadPalette(
                    (GetPartyMenuPalBufferPtr(
                        (((&raw const sHPBarYellowPalIds).cast::<u8>().cast_mut()).cast::<u8>())
                            .read(),
                    ))
                    .cast::<u8>(),
                    (((((((&raw const sHPBarPalOffsets).cast::<u8>().cast_mut()).cast::<u8>())
                        .read()) as i32)
                        .wrapping_add(((palOffset) as i32))) as u16),
                    2u16,
                );
                LoadPalette(
                    (GetPartyMenuPalBufferPtr(
                        ((((&raw const sHPBarYellowPalIds).cast::<u8>().cast_mut()).cast::<u8>())
                            .wrapping_offset(1))
                        .read(),
                    ))
                    .cast::<u8>(),
                    ((((((((&raw const sHPBarPalOffsets).cast::<u8>().cast_mut()).cast::<u8>())
                        .wrapping_offset(1))
                    .read()) as i32)
                        .wrapping_add(((palOffset) as i32))) as u16),
                    2u16,
                );
                break 'l1;
            }
            if !__matched {
                LoadPalette(
                    (GetPartyMenuPalBufferPtr(
                        (((&raw const sHPBarRedPalIds).cast::<u8>().cast_mut()).cast::<u8>())
                            .read(),
                    ))
                    .cast::<u8>(),
                    (((((((&raw const sHPBarPalOffsets).cast::<u8>().cast_mut()).cast::<u8>())
                        .read()) as i32)
                        .wrapping_add(((palOffset) as i32))) as u16),
                    2u16,
                );
                LoadPalette(
                    (GetPartyMenuPalBufferPtr(
                        ((((&raw const sHPBarRedPalIds).cast::<u8>().cast_mut()).cast::<u8>())
                            .wrapping_offset(1))
                        .read(),
                    ))
                    .cast::<u8>(),
                    ((((((((&raw const sHPBarPalOffsets).cast::<u8>().cast_mut()).cast::<u8>())
                        .wrapping_offset(1))
                    .read()) as i32)
                        .wrapping_add(((palOffset) as i32))) as u16),
                    2u16,
                );
                break 'l1;
            }
        }
        hpFraction = GetScaledHPFraction(
            ((hp) as i16),
            ((maxhp) as i16),
            ((((((menuBox).cast::<*mut u8>()).read()).wrapping_add(4)).cast::<u8>())
                .wrapping_offset(22))
            .read(),
        );
        FillWindowPixelRect(
            ((menuBox).wrapping_add(8)).read(),
            ((((&raw const sHPBarPalOffsets).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(1))
            .read(),
            ((((((((menuBox).cast::<*mut u8>()).read()).wrapping_add(4)).cast::<u8>())
                .wrapping_offset(20))
            .read()) as u16),
            ((((((((menuBox).cast::<*mut u8>()).read()).wrapping_add(4)).cast::<u8>())
                .wrapping_offset(21))
            .read()) as u16),
            ((hpFraction) as u16),
            1u16,
        );
        FillWindowPixelRect(
            ((menuBox).wrapping_add(8)).read(),
            (((&raw const sHPBarPalOffsets).cast::<u8>().cast_mut()).cast::<u8>()).read(),
            ((((((((menuBox).cast::<*mut u8>()).read()).wrapping_add(4)).cast::<u8>())
                .wrapping_offset(20))
            .read()) as u16),
            ((((((((((menuBox).cast::<*mut u8>()).read()).wrapping_add(4)).cast::<u8>())
                .wrapping_offset(21))
            .read()) as i32)
                .wrapping_add(1i32)) as u16),
            ((hpFraction) as u16),
            2u16,
        );
        if ((hpFraction) as i32)
            != ((((((((menuBox).cast::<*mut u8>()).read()).wrapping_add(4)).cast::<u8>())
                .wrapping_offset(22))
            .read()) as i32)
        {
            FillWindowPixelRect(
                ((menuBox).wrapping_add(8)).read(),
                13u8,
                ((((((((((menuBox).cast::<*mut u8>()).read()).wrapping_add(4)).cast::<u8>())
                    .wrapping_offset(20))
                .read()) as i32)
                    .wrapping_add(((hpFraction) as i32))) as u16),
                ((((((((menuBox).cast::<*mut u8>()).read()).wrapping_add(4)).cast::<u8>())
                    .wrapping_offset(21))
                .read()) as u16),
                ((((((((((menuBox).cast::<*mut u8>()).read()).wrapping_add(4)).cast::<u8>())
                    .wrapping_offset(22))
                .read()) as i32)
                    .wrapping_sub(((hpFraction) as i32))) as u16),
                1u16,
            );
            FillWindowPixelRect(
                ((menuBox).wrapping_add(8)).read(),
                2u8,
                ((((((((((menuBox).cast::<*mut u8>()).read()).wrapping_add(4)).cast::<u8>())
                    .wrapping_offset(20))
                .read()) as i32)
                    .wrapping_add(((hpFraction) as i32))) as u16),
                ((((((((((menuBox).cast::<*mut u8>()).read()).wrapping_add(4)).cast::<u8>())
                    .wrapping_offset(21))
                .read()) as i32)
                    .wrapping_add(1i32)) as u16),
                ((((((((((menuBox).cast::<*mut u8>()).read()).wrapping_add(4)).cast::<u8>())
                    .wrapping_offset(22))
                .read()) as i32)
                    .wrapping_sub(((hpFraction) as i32))) as u16),
                2u16,
            );
        }
        CopyWindowToVram(((menuBox).wrapping_add(8)).read(), 2u8);
    }
}
pub(crate) unsafe extern "C" fn DisplayPartyPokemonDescriptionText(
    stringID: u8,
    menuBox: *mut u8,
    c: u8,
) {
    unsafe {
        let mut stringID = stringID;
        let mut menuBox = menuBox;
        let mut c = c;
        if (c) != 0 {
            let mut width: i32 = crate::c::div_i32(
                ((crate::c::rem_i32(
                    ((((((menuBox).cast::<*mut u8>()).read()).wrapping_add(28)).read()) as i32),
                    8i32,
                ))
                .wrapping_add(
                    ((((((menuBox).cast::<*mut u8>()).read()).wrapping_add(30)).read()) as i32),
                ))
                .wrapping_add(7i32),
                8i32,
            );
            let mut height: i32 = crate::c::div_i32(
                ((crate::c::rem_i32(
                    ((((((menuBox).cast::<*mut u8>()).read()).wrapping_add(29)).read()) as i32),
                    8i32,
                ))
                .wrapping_add(
                    ((((((menuBox).cast::<*mut u8>()).read()).wrapping_add(31)).read()) as i32),
                ))
                .wrapping_add(7i32),
                8i32,
            );
            (((((menuBox).cast::<*mut u8>()).read())
                .cast::<Option<unsafe extern "C" fn(u8, u8, u8, u8, u8, u8)>>())
            .read())
            .unwrap_unchecked()(
                ((menuBox).wrapping_add(8)).read(),
                ((((((((menuBox).cast::<*mut u8>()).read()).wrapping_add(28)).read()) as i32) >> 3)
                    as u8),
                ((((((((menuBox).cast::<*mut u8>()).read()).wrapping_add(29)).read()) as i32) >> 3)
                    as u8),
                ((width) as u8),
                ((height) as u8),
                1u8,
            );
        }
        if ((c) as i32) != 2i32 {
            AddTextPrinterParameterized3(
                ((menuBox).wrapping_add(8)).read(),
                1u8,
                ((((menuBox).cast::<*mut u8>()).read()).wrapping_add(28)).read(),
                ((((menuBox).cast::<*mut u8>()).read()).wrapping_add(29)).read(),
                (((&raw const sFontColorTable).cast::<u8>().cast_mut()).cast::<u8>()).cast::<u8>(),
                0i8,
                ((((&raw const sDescriptionStringTable)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<*mut u8>())
                .cast::<*mut u8>())
                .wrapping_offset(((stringID) as i32) as isize))
                .read(),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn PartyMenuRemoveWindow(ptr: *mut u8) {
    unsafe {
        let mut ptr = ptr;
        if (((ptr).read()) as i32) != 255i32 {
            ClearStdWindowAndFrameToTransparent((ptr).read(), 0u8);
            RemoveWindow((ptr).read());
            (ptr).write(255u8);
            ScheduleBgCopyTilemapToVram(2u8);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DisplayPartyMenuStdMessage(stringId: u32) {
    unsafe {
        let mut stringId = stringId;
        let mut windowPtr: *mut u8 =
            (((((&raw mut sPartyMenuInternal).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(12))
            .cast::<u8>())
            .wrapping_offset(1);
        if (((windowPtr).read()) as i32) != 255i32 {
            PartyMenuRemoveWindow(windowPtr);
        }
        if stringId != 127u32 {
            'l1: {
                let __sw1 = stringId;
                let __matched = __sw1 == 21u32
                    || __sw1 == 24u32
                    || __sw1 == 25u32
                    || __sw1 == 22u32
                    || __sw1 == 23u32
                    || __sw1 == 26u32;
                if __sw1 == 21u32 {
                    (windowPtr).write(
                        ((AddWindow(
                            (&raw const sDoWhatWithMonMsgWindowTemplate)
                                .cast::<u8>()
                                .cast_mut(),
                        )) as u8),
                    );
                    break 'l1;
                }
                if __sw1 == 24u32 {
                    (windowPtr).write(
                        ((AddWindow(
                            (&raw const sDoWhatWithItemMsgWindowTemplate)
                                .cast::<u8>()
                                .cast_mut(),
                        )) as u8),
                    );
                    break 'l1;
                }
                if __sw1 == 25u32 {
                    (windowPtr).write(
                        ((AddWindow(
                            (&raw const sDoWhatWithMailMsgWindowTemplate)
                                .cast::<u8>()
                                .cast_mut(),
                        )) as u8),
                    );
                    break 'l1;
                }
                if __sw1 == 22u32 || __sw1 == 23u32 {
                    (windowPtr).write(
                        ((AddWindow(
                            (&raw const sWhichMoveMsgWindowTemplate)
                                .cast::<u8>()
                                .cast_mut(),
                        )) as u8),
                    );
                    break 'l1;
                }
                if __sw1 == 26u32 {
                    (windowPtr).write(
                        ((AddWindow(
                            (&raw const sAlreadyHoldingOneMsgWindowTemplate)
                                .cast::<u8>()
                                .cast_mut(),
                        )) as u8),
                    );
                    break 'l1;
                }
                if !__matched {
                    (windowPtr).write(
                        ((AddWindow(
                            (&raw const sDefaultPartyMsgWindowTemplate)
                                .cast::<u8>()
                                .cast_mut(),
                        )) as u8),
                    );
                    break 'l1;
                }
            }
            if stringId == 0u32 {
                if (crate::c::bf_read(
                    (((&raw mut sPartyMenuInternal).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(8),
                    0,
                    1,
                    false,
                ) as u32)
                    != 0
                {
                    stringId = 2u32;
                } else {
                    if !((ShouldUseChooseMonText()) != 0) {
                        stringId = 1u32;
                    }
                }
            }
            DrawStdFrameWithCustomTileAndPalette((windowPtr).read(), 0u8, 79u16, 13u8);
            StringExpandPlaceholders(
                (&raw mut gStringVar4).cast::<u8>(),
                ((((&raw const sActionStringTable)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<*mut u8>())
                .cast::<*mut u8>())
                .wrapping_offset(((stringId) as i32) as isize))
                .read(),
            );
            AddTextPrinterParameterized(
                (windowPtr).read(),
                1u8,
                (&raw mut gStringVar4).cast::<u8>(),
                0u8,
                1u8,
                0u8,
                None,
            );
            ScheduleBgCopyTilemapToVram(2u8);
        }
    }
}
pub(crate) unsafe extern "C" fn ShouldUseChooseMonText() -> u8 {
    unsafe {
        let mut party: *mut u8 = (&raw mut gPlayerParty).cast::<u8>();
        let mut i: u8 = 0u8;
        let mut numAliveMons: u8 = 0u8;
        if (((((&raw mut gPartyMenu).cast::<u8>()).wrapping_add(11)).read()) as i32) == 1i32 {
            return 1u8;
        }
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 6i32) {
                    break 'l1;
                }
                'l2: {
                    if (GetMonData2((party).wrapping_offset(((i) as i32) as isize * 100), 11i32)
                        != 0u32)
                        && ((GetMonData2(
                            (party).wrapping_offset(((i) as i32) as isize * 100),
                            57i32,
                        ) != 0u32)
                            || ((GetMonData2(
                                (party).wrapping_offset(((i) as i32) as isize * 100),
                                45i32,
                            )) != 0))
                    {
                        numAliveMons = (numAliveMons).wrapping_add(1);
                    }
                    if ((numAliveMons) as i32) > 1i32 {
                        return 1u8;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn DisplaySelectionWindow(windowType: u8) -> u8 {
    unsafe {
        let mut windowType = windowType;
        let mut window = crate::ffi::Align4([0u8; 8]);
        let mut cursorDimension: u8 = 0u8;
        let mut letterSpacing: u8 = 0u8;
        let mut i: u8 = 0u8;
        'l1: {
            let __sw1 = ((windowType) as i32);
            let __matched = __sw1 == 0i32 || __sw1 == 1i32 || __sw1 == 2i32;
            if __sw1 == 0i32 {
                SetWindowTemplateFields(
                    (&raw mut window).cast::<u8>(),
                    2u8,
                    19u8,
                    (((19i32).wrapping_sub(
                        ((((((&raw mut sPartyMenuInternal).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(23))
                        .read()) as i32)
                            .wrapping_mul(2i32),
                    )) as u8),
                    10u8,
                    ((((((((&raw mut sPartyMenuInternal).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(23))
                    .read()) as i32)
                        .wrapping_mul(2i32)) as u8),
                    14u8,
                    745u16,
                );
                break 'l1;
            }
            if __sw1 == 1i32 {
                (&raw mut window)
                    .cast::<u8>()
                    .cast::<crate::c::Rec4<8>>()
                    .write_unaligned(
                        (&raw const sItemGiveTakeWindowTemplate)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<crate::c::Rec4<8>>()
                            .read_unaligned(),
                    );
                break 'l1;
            }
            if __sw1 == 2i32 {
                (&raw mut window)
                    .cast::<u8>()
                    .cast::<crate::c::Rec4<8>>()
                    .write_unaligned(
                        (&raw const sMailReadTakeWindowTemplate)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<crate::c::Rec4<8>>()
                            .read_unaligned(),
                    );
                break 'l1;
            }
            if !__matched {
                (&raw mut window)
                    .cast::<u8>()
                    .cast::<crate::c::Rec4<8>>()
                    .write_unaligned(
                        (&raw const sMoveSelectWindowTemplate)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<crate::c::Rec4<8>>()
                            .read_unaligned(),
                    );
                break 'l1;
            }
        }
        (((((&raw mut sPartyMenuInternal).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(12))
        .cast::<u8>())
        .write(((AddWindow((&raw mut window).cast::<u8>())) as u8));
        DrawStdFrameWithCustomTileAndPalette(
            (((((&raw mut sPartyMenuInternal).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(12))
            .cast::<u8>())
            .read(),
            0u8,
            79u16,
            13u8,
        );
        if ((windowType) as i32) == 3i32 {
            return (((((&raw mut sPartyMenuInternal).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(12))
            .cast::<u8>())
            .read();
        }
        cursorDimension = GetMenuCursorDimensionByFont(1u8, 0u8);
        letterSpacing = GetFontAttribute(1u8, 2u8);
        {
            i = 0u8;
            'l2: loop {
                if !(((i) as i32)
                    < ((((((&raw mut sPartyMenuInternal).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(23))
                    .read()) as i32))
                {
                    break 'l2;
                }
                'l3: {
                    let mut fontColorsId: u8 = ((if ((((((((&raw mut sPartyMenuInternal)
                        .cast::<u8>()
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(15))
                    .cast::<u8>())
                    .wrapping_offset(((i) as i32) as isize))
                    .read()) as i32)
                        >= 19i32
                    {
                        4i32
                    } else {
                        3i32
                    }) as u8);
                    AddTextPrinterParameterized4(
                        (((((&raw mut sPartyMenuInternal).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(12))
                        .cast::<u8>())
                        .read(),
                        1u8,
                        cursorDimension,
                        (((((i) as i32).wrapping_mul(16i32)).wrapping_add(1i32)) as u8),
                        letterSpacing,
                        0u8,
                        ((((&raw const sFontColorTable).cast::<u8>().cast_mut()).cast::<u8>())
                            .wrapping_offset(((fontColorsId) as i32) as isize * 3))
                        .cast::<u8>(),
                        0i8,
                        (((((&raw const sCursorOptions).cast::<u8>().cast_mut()).cast::<u8>())
                            .wrapping_offset(
                                ((((((((&raw mut sPartyMenuInternal)
                                    .cast::<u8>()
                                    .cast::<*mut u8>())
                                .read())
                                .wrapping_add(15))
                                .cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize))
                                .read()) as i32) as isize
                                    * 8,
                            ))
                        .cast::<*mut u8>())
                        .read(),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        InitMenuInUpperLeftCorner(
            (((((&raw mut sPartyMenuInternal).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(12))
            .cast::<u8>())
            .read(),
            ((((&raw mut sPartyMenuInternal).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(23))
            .read(),
            0u8,
            1u8,
        );
        ScheduleBgCopyTilemapToVram(2u8);
        return (((((&raw mut sPartyMenuInternal).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(12))
        .cast::<u8>())
        .read();
    }
}
pub(crate) unsafe extern "C" fn PrintMessage(text: *mut u8) {
    unsafe {
        let mut text = text;
        DrawStdFrameWithCustomTileAndPalette(6u8, 0u8, 79u16, 13u8);
        crate::c::bf_write(
            ((&raw mut gTextFlags).cast::<u8>()).wrapping_add(0),
            0,
            1,
            (1u8) as i32,
        );
        AddTextPrinterParameterized2(
            6u8,
            1u8,
            text,
            GetPlayerTextSpeedDelay(),
            None,
            2u8,
            1u8,
            3u8,
        );
    }
}
pub(crate) unsafe extern "C" fn PartyMenuDisplayYesNoMenu() {
    unsafe {
        CreateYesNoMenu(
            (&raw const sPartyMenuYesNoWindowTemplate)
                .cast::<u8>()
                .cast_mut(),
            79u16,
            13u8,
            0u8,
        );
    }
}
pub(crate) unsafe extern "C" fn CreateLevelUpStatsWindow() -> u8 {
    unsafe {
        (((((&raw mut sPartyMenuInternal).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(12))
        .cast::<u8>())
        .write(
            ((AddWindow(
                (&raw const sLevelUpStatsWindowTemplate)
                    .cast::<u8>()
                    .cast_mut(),
            )) as u8),
        );
        DrawStdFrameWithCustomTileAndPalette(
            (((((&raw mut sPartyMenuInternal).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(12))
            .cast::<u8>())
            .read(),
            0u8,
            79u16,
            13u8,
        );
        return (((((&raw mut sPartyMenuInternal).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(12))
        .cast::<u8>())
        .read();
    }
}
pub(crate) unsafe extern "C" fn RemoveLevelUpStatsWindow() {
    unsafe {
        ClearWindowTilemap(
            (((((&raw mut sPartyMenuInternal).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(12))
            .cast::<u8>())
            .read(),
        );
        PartyMenuRemoveWindow(
            ((((&raw mut sPartyMenuInternal).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(12))
            .cast::<u8>(),
        );
    }
}
pub(crate) unsafe extern "C" fn SetPartyMonSelectionActions(mons: *mut u8, slotId: u8, action: u8) {
    unsafe {
        let mut mons = mons;
        let mut slotId = slotId;
        let mut action = action;
        let mut i: u8 = 0u8;
        if ((action) as i32) == 0i32 {
            SetPartyMonFieldSelectionActions(mons, slotId);
        } else {
            ((((&raw mut sPartyMenuInternal).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(23))
            .write(
                ((((&raw const sPartyMenuActionCounts).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset(((action) as i32) as isize))
                .read(),
            );
            {
                i = 0u8;
                'l1: loop {
                    if !(((i) as i32)
                        < ((((((&raw mut sPartyMenuInternal).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(23))
                        .read()) as i32))
                    {
                        break 'l1;
                    }
                    'l2: {
                        ((((((&raw mut sPartyMenuInternal).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(15))
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize))
                        .write(
                            ((((((&raw const sPartyMenuActions)
                                .cast::<u8>()
                                .cast_mut()
                                .cast::<*mut u8>())
                            .cast::<*mut u8>())
                            .wrapping_offset(((action) as i32) as isize))
                            .read())
                            .wrapping_offset(((i) as i32) as isize))
                            .read(),
                        );
                    }
                    i = (i).wrapping_add(1);
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SetPartyMonFieldSelectionActions(mons: *mut u8, slotId: u8) {
    unsafe {
        let mut mons = mons;
        let mut slotId = slotId;
        let mut i: u8 = 0u8;
        let mut j: u8 = 0u8;
        ((((&raw mut sPartyMenuInternal).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(23))
            .write(0u8);
        AppendToList(
            ((((&raw mut sPartyMenuInternal).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(15))
            .cast::<u8>(),
            (((&raw mut sPartyMenuInternal).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(23),
            0u8,
        );
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
                            if !(((((((&raw const sFieldMoves)
                                .cast::<u8>()
                                .cast_mut()
                                .cast::<u16>())
                            .cast::<u16>())
                            .wrapping_offset(((j) as i32) as isize))
                            .read()) as i32)
                                != 14i32)
                            {
                                break 'l3;
                            }
                            'l4: {
                                if GetMonData2(
                                    (mons).wrapping_offset(((slotId) as i32) as isize * 100),
                                    ((i) as i32).wrapping_add(13i32),
                                ) == ((((((&raw const sFieldMoves)
                                    .cast::<u8>()
                                    .cast_mut()
                                    .cast::<u16>())
                                .cast::<u16>())
                                .wrapping_offset(((j) as i32) as isize))
                                .read()) as u32)
                                {
                                    AppendToList(
                                        ((((&raw mut sPartyMenuInternal)
                                            .cast::<u8>()
                                            .cast::<*mut u8>())
                                        .read())
                                        .wrapping_add(15))
                                        .cast::<u8>(),
                                        (((&raw mut sPartyMenuInternal)
                                            .cast::<u8>()
                                            .cast::<*mut u8>())
                                        .read())
                                        .wrapping_add(23),
                                        ((((j) as i32).wrapping_add(19i32)) as u8),
                                    );
                                    break 'l3;
                                }
                            }
                            j = (j).wrapping_add(1);
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        if !((InBattlePike()) != 0) {
            if GetMonData2((mons).wrapping_offset(100), 11i32) != 0u32 {
                AppendToList(
                    ((((&raw mut sPartyMenuInternal).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(15))
                    .cast::<u8>(),
                    (((&raw mut sPartyMenuInternal).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(23),
                    1u8,
                );
            }
            if (ItemIsMail(
                ((GetMonData2(
                    (mons).wrapping_offset(((slotId) as i32) as isize * 100),
                    12i32,
                )) as u16),
            )) != 0
            {
                AppendToList(
                    ((((&raw mut sPartyMenuInternal).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(15))
                    .cast::<u8>(),
                    (((&raw mut sPartyMenuInternal).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(23),
                    6u8,
                );
            } else {
                AppendToList(
                    ((((&raw mut sPartyMenuInternal).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(15))
                    .cast::<u8>(),
                    (((&raw mut sPartyMenuInternal).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(23),
                    3u8,
                );
            }
        }
        AppendToList(
            ((((&raw mut sPartyMenuInternal).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(15))
            .cast::<u8>(),
            (((&raw mut sPartyMenuInternal).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(23),
            2u8,
        );
    }
}
pub(crate) unsafe extern "C" fn GetPartyMenuActionsType(mon: *mut u8) -> u8 {
    unsafe {
        let mut mon = mon;
        let mut actionType: u32 = 0u32;
        'l1: {
            let __sw1 = ((crate::c::bf_read(
                ((&raw mut gPartyMenu).cast::<u8>()).wrapping_add(8),
                0,
                4,
                false,
            ) as u8) as i32);
            let __matched = __sw1 == 0i32
                || __sw1 == 1i32
                || __sw1 == 4i32
                || __sw1 == 6i32
                || __sw1 == 8i32
                || __sw1 == 9i32
                || __sw1 == 10i32
                || __sw1 == 12i32;
            if __sw1 == 0i32 {
                if (((InMultiPartnerRoom()) as i32) == 1i32) || ((GetMonData2(mon, 45i32)) != 0) {
                    actionType = 1u32;
                } else {
                    actionType = 0u32;
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                actionType = ((GetPartyMenuActionsTypeInBattle(mon)) as u32);
                break 'l1;
            }
            if __sw1 == 4i32 {
                'l2: {
                    let __sw2 = ((GetPartySlotEntryStatus(
                        (((&raw mut gPartyMenu).cast::<u8>())
                            .wrapping_add(9)
                            .cast::<i8>())
                        .read(),
                    )) as i32);
                    let __matched = __sw2 == 0i32 || __sw2 == 1i32;
                    if !__matched {
                        actionType = 7u32;
                        break 'l2;
                    }
                    if __sw2 == 0i32 {
                        actionType = 4u32;
                        break 'l2;
                    }
                    if __sw2 == 1i32 {
                        actionType = 5u32;
                        break 'l2;
                    }
                }
                break 'l1;
            }
            if __sw1 == 6i32 {
                actionType = ((if (GetMonData2(mon, 45i32)) != 0 {
                    7i32
                } else {
                    6i32
                }) as u32);
                break 'l1;
            }
            if __sw1 == 8i32 {
                actionType = 10u32;
                break 'l1;
            }
            if __sw1 == 9i32 {
                actionType = 11u32;
                break 'l1;
            }
            if __sw1 == 10i32 {
                actionType = 12u32;
                break 'l1;
            }
            if __sw1 == 12i32 {
                actionType = 13u32;
                break 'l1;
            }
            if !__matched {
                actionType = 0u32;
                break 'l1;
            }
        }
        return ((actionType) as u8);
    }
}
pub(crate) unsafe extern "C" fn CreateSelectionWindow(taskId: u8) -> u8 {
    unsafe {
        let mut taskId = taskId;
        let mut mon: *mut u8 = ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
            (((((&raw mut gPartyMenu).cast::<u8>())
                .wrapping_add(9)
                .cast::<i8>())
            .read()) as i32) as isize
                * 100,
        );
        let mut item: u16 = 0u16;
        GetMonNickname(mon, (&raw mut gStringVar1).cast::<u8>());
        PartyMenuRemoveWindow(
            (((((&raw mut sPartyMenuInternal).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(12))
            .cast::<u8>())
            .wrapping_offset(1),
        );
        if ((crate::c::bf_read(
            ((&raw mut gPartyMenu).cast::<u8>()).wrapping_add(8),
            0,
            4,
            false,
        ) as u8) as i32)
            != 12i32
        {
            SetPartyMonSelectionActions(
                (&raw mut gPlayerParty).cast::<u8>(),
                (((((&raw mut gPartyMenu).cast::<u8>())
                    .wrapping_add(9)
                    .cast::<i8>())
                .read()) as u8),
                GetPartyMenuActionsType(mon),
            );
            DisplaySelectionWindow(0u8);
            DisplayPartyMenuStdMessage(21u32);
        } else {
            item = ((GetMonData2(mon, 12i32)) as u16);
            if ((item) as i32) != 0i32 {
                SetPartyMonSelectionActions(
                    (&raw mut gPlayerParty).cast::<u8>(),
                    (((((&raw mut gPartyMenu).cast::<u8>())
                        .wrapping_add(9)
                        .cast::<i8>())
                    .read()) as u8),
                    GetPartyMenuActionsType(mon),
                );
                DisplaySelectionWindow(1u8);
                CopyItemName(item, (&raw mut gStringVar2).cast::<u8>());
                DisplayPartyMenuStdMessage(26u32);
            } else {
                StringExpandPlaceholders(
                    (&raw mut gStringVar4).cast::<u8>(),
                    (&raw mut gText_PkmnNotHolding).cast::<u8>(),
                );
                DisplayPartyMenuMessage((&raw mut gStringVar4).cast::<u8>(), 1u8);
                ScheduleBgCopyTilemapToVram(2u8);
                ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
                .write(Some(Task_UpdateHeldItemSprite));
                return 0u8;
            }
        }
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn Task_TryCreateSelectionWindow(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if (CreateSelectionWindow(taskId)) != 0 {
            (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
            .cast::<i16>())
            .write(255i16);
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_HandleSelectionMenuInput));
        }
    }
}
pub(crate) unsafe extern "C" fn Task_HandleSelectionMenuInput(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if (!((crate::c::bf_read(
            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
            7,
            1,
            false,
        ) as u16)
            != 0))
            && (((MenuHelpers_ShouldWaitForLinkRecv()) as i32) != 1i32)
        {
            let mut input: i8 = 0i8;
            let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>();
            if ((((((&raw mut sPartyMenuInternal).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(23))
            .read()) as i32)
                <= 3i32
            {
                input = Menu_ProcessInputNoWrapAround_other();
            } else {
                input = ProcessMenuInput_other();
            }
            (data).write(((Menu_GetCursorPos()) as i16));
            'l1: {
                let __sw1 = ((input) as i32);
                let __matched = __sw1 == (-2i32) || __sw1 == (-1i32);
                if __sw1 == (-2i32) {
                    break 'l1;
                }
                if __sw1 == (-1i32) {
                    PlaySE(5u16);
                    PartyMenuRemoveWindow(
                        (((((&raw mut sPartyMenuInternal).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(12))
                        .cast::<u8>())
                        .wrapping_offset(2),
                    );
                    ((((((&raw const sCursorOptions).cast::<u8>().cast_mut()).cast::<u8>())
                        .wrapping_offset(
                            ((((((((&raw mut sPartyMenuInternal)
                                .cast::<u8>()
                                .cast::<*mut u8>())
                            .read())
                            .wrapping_add(15))
                            .cast::<u8>())
                            .wrapping_offset(
                                (((((((&raw mut sPartyMenuInternal)
                                    .cast::<u8>()
                                    .cast::<*mut u8>())
                                .read())
                                .wrapping_add(23))
                                .read()) as i32)
                                    .wrapping_sub(1i32)) as isize,
                            ))
                            .read()) as i32) as isize
                                * 8,
                        ))
                    .wrapping_add(4)
                    .cast::<Option<unsafe extern "C" fn(u8)>>())
                    .read())
                    .unwrap_unchecked()(taskId);
                    break 'l1;
                }
                if !__matched {
                    PartyMenuRemoveWindow(
                        (((((&raw mut sPartyMenuInternal).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(12))
                        .cast::<u8>())
                        .wrapping_offset(2),
                    );
                    ((((((&raw const sCursorOptions).cast::<u8>().cast_mut()).cast::<u8>())
                        .wrapping_offset(
                            ((((((((&raw mut sPartyMenuInternal)
                                .cast::<u8>()
                                .cast::<*mut u8>())
                            .read())
                            .wrapping_add(15))
                            .cast::<u8>())
                            .wrapping_offset(((input) as i32) as isize))
                            .read()) as i32) as isize
                                * 8,
                        ))
                    .wrapping_add(4)
                    .cast::<Option<unsafe extern "C" fn(u8)>>())
                    .read())
                    .unwrap_unchecked()(taskId);
                    break 'l1;
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn CursorCb_Summary(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        PlaySE(5u16);
        ((((&raw mut sPartyMenuInternal).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(4)
            .cast::<Option<unsafe extern "C" fn()>>())
        .write(Some(CB2_ShowPokemonSummaryScreen));
        Task_ClosePartyMenu(taskId);
    }
}
pub(crate) unsafe extern "C" fn CB2_ShowPokemonSummaryScreen() {
    unsafe {
        if ((crate::c::bf_read(
            ((&raw mut gPartyMenu).cast::<u8>()).wrapping_add(8),
            0,
            4,
            false,
        ) as u8) as i32)
            == 1i32
        {
            UpdatePartyToBattleOrder();
            ShowPokemonSummaryScreen(
                1u8,
                (&raw mut gPlayerParty).cast::<u8>(),
                (((((&raw mut gPartyMenu).cast::<u8>())
                    .wrapping_add(9)
                    .cast::<i8>())
                .read()) as u8),
                ((((((&raw mut gPlayerPartyCount).cast::<u8>()).read()) as i32).wrapping_sub(1i32))
                    as u8),
                Some(CB2_ReturnToPartyMenuFromSummaryScreen),
            );
        } else {
            ShowPokemonSummaryScreen(
                0u8,
                (&raw mut gPlayerParty).cast::<u8>(),
                (((((&raw mut gPartyMenu).cast::<u8>())
                    .wrapping_add(9)
                    .cast::<i8>())
                .read()) as u8),
                ((((((&raw mut gPlayerPartyCount).cast::<u8>()).read()) as i32).wrapping_sub(1i32))
                    as u8),
                Some(CB2_ReturnToPartyMenuFromSummaryScreen),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn CB2_ReturnToPartyMenuFromSummaryScreen() {
    unsafe {
        crate::c::bf_write(
            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(8),
            7,
            1,
            (1u16) as i32,
        );
        (((&raw mut gPartyMenu).cast::<u8>())
            .wrapping_add(9)
            .cast::<i8>())
        .write(((((&raw mut gLastViewedMonIndex).cast::<u8>()).read()) as i8));
        InitPartyMenu(
            (crate::c::bf_read(
                ((&raw mut gPartyMenu).cast::<u8>()).wrapping_add(8),
                0,
                4,
                false,
            ) as u8),
            255u8,
            (((&raw mut gPartyMenu).cast::<u8>()).wrapping_add(11)).read(),
            1u8,
            21u8,
            Some(Task_TryCreateSelectionWindow),
            (((&raw mut gPartyMenu).cast::<u8>()).cast::<Option<unsafe extern "C" fn()>>()).read(),
        );
    }
}
pub(crate) unsafe extern "C" fn CursorCb_Switch(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        PlaySE(5u16);
        (((&raw mut gPartyMenu).cast::<u8>()).wrapping_add(11)).write(8u8);
        PartyMenuRemoveWindow(
            (((((&raw mut sPartyMenuInternal).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(12))
            .cast::<u8>())
            .wrapping_offset(1),
        );
        PartyMenuRemoveWindow(
            ((((&raw mut sPartyMenuInternal).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(12))
            .cast::<u8>(),
        );
        DisplayPartyMenuStdMessage(3u32);
        AnimatePartySlot(
            (((((&raw mut gPartyMenu).cast::<u8>())
                .wrapping_add(9)
                .cast::<i8>())
            .read()) as u8),
            1u8,
        );
        (((&raw mut gPartyMenu).cast::<u8>())
            .wrapping_add(10)
            .cast::<i8>())
        .write(
            (((&raw mut gPartyMenu).cast::<u8>())
                .wrapping_add(9)
                .cast::<i8>())
            .read(),
        );
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Task_HandleChooseMonInput));
    }
}
pub(crate) unsafe extern "C" fn SwitchSelectedMons(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        let mut windowIds = crate::ffi::Align4([0u8; 2]);
        if (((((&raw mut gPartyMenu).cast::<u8>())
            .wrapping_add(10)
            .cast::<i8>())
        .read()) as i32)
            == (((((&raw mut gPartyMenu).cast::<u8>())
                .wrapping_add(9)
                .cast::<i8>())
            .read()) as i32)
        {
            FinishTwoMonAction(taskId);
        } else {
            ((&raw mut windowIds).cast::<u8>()).write(
                (((((&raw mut sPartyMenuBoxes).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_offset(
                        (((((&raw mut gPartyMenu).cast::<u8>())
                            .wrapping_add(9)
                            .cast::<i8>())
                        .read()) as i32) as isize
                            * 16,
                    ))
                .wrapping_add(8))
                .read(),
            );
            (data).write(
                ((GetWindowAttribute(((&raw mut windowIds).cast::<u8>()).read(), 1u8)) as i16),
            );
            ((data).wrapping_offset(1)).write(
                ((GetWindowAttribute(((&raw mut windowIds).cast::<u8>()).read(), 2u8)) as i16),
            );
            ((data).wrapping_offset(2)).write(
                ((GetWindowAttribute(((&raw mut windowIds).cast::<u8>()).read(), 3u8)) as i16),
            );
            ((data).wrapping_offset(3)).write(
                ((GetWindowAttribute(((&raw mut windowIds).cast::<u8>()).read(), 4u8)) as i16),
            );
            ((data).wrapping_offset(8)).write(0i16);
            if ((((data).wrapping_offset(2)).read()) as i32) == 10i32 {
                ((data).wrapping_offset(10)).write((-1i16));
            } else {
                ((data).wrapping_offset(10)).write(1i16);
            }
            (((&raw mut windowIds).cast::<u8>()).wrapping_offset(1)).write(
                (((((&raw mut sPartyMenuBoxes).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_offset(
                        (((((&raw mut gPartyMenu).cast::<u8>())
                            .wrapping_add(10)
                            .cast::<i8>())
                        .read()) as i32) as isize
                            * 16,
                    ))
                .wrapping_add(8))
                .read(),
            );
            ((data).wrapping_offset(4)).write(
                ((GetWindowAttribute(
                    (((&raw mut windowIds).cast::<u8>()).wrapping_offset(1)).read(),
                    1u8,
                )) as i16),
            );
            ((data).wrapping_offset(5)).write(
                ((GetWindowAttribute(
                    (((&raw mut windowIds).cast::<u8>()).wrapping_offset(1)).read(),
                    2u8,
                )) as i16),
            );
            ((data).wrapping_offset(6)).write(
                ((GetWindowAttribute(
                    (((&raw mut windowIds).cast::<u8>()).wrapping_offset(1)).read(),
                    3u8,
                )) as i16),
            );
            ((data).wrapping_offset(7)).write(
                ((GetWindowAttribute(
                    (((&raw mut windowIds).cast::<u8>()).wrapping_offset(1)).read(),
                    4u8,
                )) as i16),
            );
            ((data).wrapping_offset(9)).write(0i16);
            if ((((data).wrapping_offset(6)).read()) as i32) == 10i32 {
                ((data).wrapping_offset(11)).write((-1i16));
            } else {
                ((data).wrapping_offset(11)).write(1i16);
            }
            ((&raw mut sSlot1TilemapBuffer)
                .cast::<u8>()
                .cast::<*mut u16>())
            .write(
                (Alloc(
                    ((((((data).wrapping_offset(2)).read()) as i32)
                        .wrapping_mul((((((data).wrapping_offset(3)).read()) as i32) << 1)))
                        as u32),
                ))
                .cast::<u16>(),
            );
            ((&raw mut sSlot2TilemapBuffer)
                .cast::<u8>()
                .cast::<*mut u16>())
            .write(
                (Alloc(
                    ((((((data).wrapping_offset(6)).read()) as i32)
                        .wrapping_mul((((((data).wrapping_offset(7)).read()) as i32) << 1)))
                        as u32),
                ))
                .cast::<u16>(),
            );
            CopyToBufferFromBgTilemap(
                0u8,
                ((&raw mut sSlot1TilemapBuffer)
                    .cast::<u8>()
                    .cast::<*mut u16>())
                .read(),
                (((data).read()) as u8),
                ((((data).wrapping_offset(1)).read()) as u8),
                ((((data).wrapping_offset(2)).read()) as u8),
                ((((data).wrapping_offset(3)).read()) as u8),
            );
            CopyToBufferFromBgTilemap(
                0u8,
                ((&raw mut sSlot2TilemapBuffer)
                    .cast::<u8>()
                    .cast::<*mut u16>())
                .read(),
                ((((data).wrapping_offset(4)).read()) as u8),
                ((((data).wrapping_offset(5)).read()) as u8),
                ((((data).wrapping_offset(6)).read()) as u8),
                ((((data).wrapping_offset(7)).read()) as u8),
            );
            ClearWindowTilemap(((&raw mut windowIds).cast::<u8>()).read());
            ClearWindowTilemap((((&raw mut windowIds).cast::<u8>()).wrapping_offset(1)).read());
            (((&raw mut gPartyMenu).cast::<u8>()).wrapping_add(11)).write(9u8);
            AnimatePartySlot(
                (((((&raw mut gPartyMenu).cast::<u8>())
                    .wrapping_add(9)
                    .cast::<i8>())
                .read()) as u8),
                1u8,
            );
            AnimatePartySlot(
                (((((&raw mut gPartyMenu).cast::<u8>())
                    .wrapping_add(10)
                    .cast::<i8>())
                .read()) as u8),
                1u8,
            );
            SlidePartyMenuBoxOneStep(taskId);
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_SlideSelectedSlotsOffscreen));
        }
    }
}
pub(crate) unsafe extern "C" fn TryMovePartySlot(
    x: i16,
    width: i16,
    leftMove: *mut u8,
    newX: *mut u8,
    newWidth: *mut u8,
) -> u8 {
    unsafe {
        let mut x = x;
        let mut width = width;
        let mut leftMove = leftMove;
        let mut newX = newX;
        let mut newWidth = newWidth;
        if ((x) as i32).wrapping_add(((width) as i32)) < 0i32 {
            return 0u8;
        }
        if ((x) as i32) >= 32i32 {
            return 0u8;
        }
        if ((x) as i32) < 0i32 {
            (leftMove).write(((((x) as i32).wrapping_mul((-1i32))) as u8));
            (newX).write(0u8);
            (newWidth).write(((((width) as i32).wrapping_add(((x) as i32))) as u8));
        } else {
            (leftMove).write(0u8);
            (newX).write(((x) as u8));
            if ((x) as i32).wrapping_add(((width) as i32)) >= 32i32 {
                (newWidth).write((((32i32).wrapping_sub(((x) as i32))) as u8));
            } else {
                (newWidth).write(((width) as u8));
            }
        }
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn MoveAndBufferPartySlot(
    rectSrc: *mut u8,
    x: i16,
    y: i16,
    width: i16,
    height: i16,
    dir: i16,
) {
    unsafe {
        let mut rectSrc = rectSrc;
        let mut x = x;
        let mut y = y;
        let mut width = width;
        let mut height = height;
        let mut dir = dir;
        let mut srcX: u8 = 0u8;
        let mut newX: u8 = 0u8;
        let mut newWidth: u8 = 0u8;
        if (TryMovePartySlot(x, width, &raw mut srcX, &raw mut newX, &raw mut newWidth)) != 0 {
            FillBgTilemapBufferRect_Palette0(
                0u8,
                0u16,
                newX,
                ((y) as u8),
                newWidth,
                ((height) as u8),
            );
            if (TryMovePartySlot(
                ((((x) as i32).wrapping_add(((dir) as i32))) as i16),
                width,
                &raw mut srcX,
                &raw mut newX,
                &raw mut newWidth,
            )) != 0
            {
                CopyRectToBgTilemapBufferRect(
                    0u8,
                    rectSrc,
                    srcX,
                    0u8,
                    ((width) as u8),
                    ((height) as u8),
                    newX,
                    ((y) as u8),
                    newWidth,
                    ((height) as u8),
                    17u8,
                    0i16,
                    0i16,
                );
            }
        }
    }
}
pub(crate) unsafe extern "C" fn MovePartyMenuBoxSprites(menuBox: *mut u8, offset: i16) {
    unsafe {
        let mut menuBox = menuBox;
        let mut offset = offset;
        let __p1 = (((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((((menuBox).wrapping_add(11)).read()) as i32) as isize * 68))
        .wrapping_add(36)
        .cast::<i16>();
        (__p1).write(
            (((((__p1).read()) as i32).wrapping_add(((offset) as i32).wrapping_mul(8i32))) as i16),
        );
        let __p2 = (((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((((menuBox).wrapping_add(10)).read()) as i32) as isize * 68))
        .wrapping_add(36)
        .cast::<i16>();
        (__p2).write(
            (((((__p2).read()) as i32).wrapping_add(((offset) as i32).wrapping_mul(8i32))) as i16),
        );
        let __p3 = (((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((((menuBox).wrapping_add(9)).read()) as i32) as isize * 68))
        .wrapping_add(36)
        .cast::<i16>();
        (__p3).write(
            (((((__p3).read()) as i32).wrapping_add(((offset) as i32).wrapping_mul(8i32))) as i16),
        );
        let __p4 = (((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((((menuBox).wrapping_add(12)).read()) as i32) as isize * 68))
        .wrapping_add(36)
        .cast::<i16>();
        (__p4).write(
            (((((__p4).read()) as i32).wrapping_add(((offset) as i32).wrapping_mul(8i32))) as i16),
        );
    }
}
pub(crate) unsafe extern "C" fn SlidePartyMenuBoxSpritesOneStep(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        if ((((data).wrapping_offset(10)).read()) as i32) != 0i32 {
            MovePartyMenuBoxSprites(
                (((&raw mut sPartyMenuBoxes).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_offset(
                        (((((&raw mut gPartyMenu).cast::<u8>())
                            .wrapping_add(9)
                            .cast::<i8>())
                        .read()) as i32) as isize
                            * 16,
                    ),
                ((data).wrapping_offset(10)).read(),
            );
        }
        if ((((data).wrapping_offset(11)).read()) as i32) != 0i32 {
            MovePartyMenuBoxSprites(
                (((&raw mut sPartyMenuBoxes).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_offset(
                        (((((&raw mut gPartyMenu).cast::<u8>())
                            .wrapping_add(10)
                            .cast::<i8>())
                        .read()) as i32) as isize
                            * 16,
                    ),
                ((data).wrapping_offset(11)).read(),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn SlidePartyMenuBoxOneStep(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        if ((((data).wrapping_offset(10)).read()) as i32) != 0i32 {
            MoveAndBufferPartySlot(
                (((&raw mut sSlot1TilemapBuffer)
                    .cast::<u8>()
                    .cast::<*mut u16>())
                .read())
                .cast::<u8>(),
                (((((data).read()) as i32)
                    .wrapping_add(((((data).wrapping_offset(8)).read()) as i32)))
                    as i16),
                ((data).wrapping_offset(1)).read(),
                ((data).wrapping_offset(2)).read(),
                ((data).wrapping_offset(3)).read(),
                ((data).wrapping_offset(10)).read(),
            );
        }
        if ((((data).wrapping_offset(11)).read()) as i32) != 0i32 {
            MoveAndBufferPartySlot(
                (((&raw mut sSlot2TilemapBuffer)
                    .cast::<u8>()
                    .cast::<*mut u16>())
                .read())
                .cast::<u8>(),
                ((((((data).wrapping_offset(4)).read()) as i32)
                    .wrapping_add(((((data).wrapping_offset(9)).read()) as i32)))
                    as i16),
                ((data).wrapping_offset(5)).read(),
                ((data).wrapping_offset(6)).read(),
                ((data).wrapping_offset(7)).read(),
                ((data).wrapping_offset(11)).read(),
            );
        }
        ScheduleBgCopyTilemapToVram(0u8);
    }
}
pub(crate) unsafe extern "C" fn Task_SlideSelectedSlotsOffscreen(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        let mut slidingSlotPositions = crate::ffi::Align4([0u8; 4]);
        SlidePartyMenuBoxOneStep(taskId);
        SlidePartyMenuBoxSpritesOneStep(taskId);
        let __p1 = (data).wrapping_offset(8);
        (__p1).write(
            (((((__p1).read()) as i32).wrapping_add(((((data).wrapping_offset(10)).read()) as i32)))
                as i16),
        );
        let __p2 = (data).wrapping_offset(9);
        (__p2).write(
            (((((__p2).read()) as i32).wrapping_add(((((data).wrapping_offset(11)).read()) as i32)))
                as i16),
        );
        ((&raw mut slidingSlotPositions).cast::<u16>()).write(
            (((((data).read()) as i32).wrapping_add(((((data).wrapping_offset(8)).read()) as i32)))
                as u16),
        );
        (((&raw mut slidingSlotPositions).cast::<u16>()).wrapping_offset(1)).write(
            ((((((data).wrapping_offset(4)).read()) as i32)
                .wrapping_add(((((data).wrapping_offset(9)).read()) as i32))) as u16),
        );
        if (((((&raw mut slidingSlotPositions).cast::<u16>()).read()) as i32) > 33i32)
            && ((((((&raw mut slidingSlotPositions).cast::<u16>()).wrapping_offset(1)).read())
                as i32)
                > 33i32)
        {
            let __p3 = (data).wrapping_offset(10);
            (__p3).write((((((__p3).read()) as i32).wrapping_mul((-1i32))) as i16));
            let __p4 = (data).wrapping_offset(11);
            (__p4).write((((((__p4).read()) as i32).wrapping_mul((-1i32))) as i16));
            SwitchPartyMon();
            DisplayPartyPokemonData(
                (((((&raw mut gPartyMenu).cast::<u8>())
                    .wrapping_add(9)
                    .cast::<i8>())
                .read()) as u8),
            );
            DisplayPartyPokemonData(
                (((((&raw mut gPartyMenu).cast::<u8>())
                    .wrapping_add(10)
                    .cast::<i8>())
                .read()) as u8),
            );
            PutWindowTilemap(
                (((((&raw mut sPartyMenuBoxes).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_offset(
                        (((((&raw mut gPartyMenu).cast::<u8>())
                            .wrapping_add(9)
                            .cast::<i8>())
                        .read()) as i32) as isize
                            * 16,
                    ))
                .wrapping_add(8))
                .read(),
            );
            PutWindowTilemap(
                (((((&raw mut sPartyMenuBoxes).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_offset(
                        (((((&raw mut gPartyMenu).cast::<u8>())
                            .wrapping_add(10)
                            .cast::<i8>())
                        .read()) as i32) as isize
                            * 16,
                    ))
                .wrapping_add(8))
                .read(),
            );
            CopyToBufferFromBgTilemap(
                0u8,
                ((&raw mut sSlot1TilemapBuffer)
                    .cast::<u8>()
                    .cast::<*mut u16>())
                .read(),
                (((data).read()) as u8),
                ((((data).wrapping_offset(1)).read()) as u8),
                ((((data).wrapping_offset(2)).read()) as u8),
                ((((data).wrapping_offset(3)).read()) as u8),
            );
            CopyToBufferFromBgTilemap(
                0u8,
                ((&raw mut sSlot2TilemapBuffer)
                    .cast::<u8>()
                    .cast::<*mut u16>())
                .read(),
                ((((data).wrapping_offset(4)).read()) as u8),
                ((((data).wrapping_offset(5)).read()) as u8),
                ((((data).wrapping_offset(6)).read()) as u8),
                ((((data).wrapping_offset(7)).read()) as u8),
            );
            ClearWindowTilemap(
                (((((&raw mut sPartyMenuBoxes).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_offset(
                        (((((&raw mut gPartyMenu).cast::<u8>())
                            .wrapping_add(9)
                            .cast::<i8>())
                        .read()) as i32) as isize
                            * 16,
                    ))
                .wrapping_add(8))
                .read(),
            );
            ClearWindowTilemap(
                (((((&raw mut sPartyMenuBoxes).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_offset(
                        (((((&raw mut gPartyMenu).cast::<u8>())
                            .wrapping_add(10)
                            .cast::<i8>())
                        .read()) as i32) as isize
                            * 16,
                    ))
                .wrapping_add(8))
                .read(),
            );
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_SlideSelectedSlotsOnscreen));
        }
    }
}
pub(crate) unsafe extern "C" fn Task_SlideSelectedSlotsOnscreen(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        SlidePartyMenuBoxOneStep(taskId);
        SlidePartyMenuBoxSpritesOneStep(taskId);
        if (((((data).wrapping_offset(10)).read()) as i32) == 0i32)
            && (((((data).wrapping_offset(11)).read()) as i32) == 0i32)
        {
            PutWindowTilemap(
                (((((&raw mut sPartyMenuBoxes).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_offset(
                        (((((&raw mut gPartyMenu).cast::<u8>())
                            .wrapping_add(9)
                            .cast::<i8>())
                        .read()) as i32) as isize
                            * 16,
                    ))
                .wrapping_add(8))
                .read(),
            );
            PutWindowTilemap(
                (((((&raw mut sPartyMenuBoxes).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_offset(
                        (((((&raw mut gPartyMenu).cast::<u8>())
                            .wrapping_add(10)
                            .cast::<i8>())
                        .read()) as i32) as isize
                            * 16,
                    ))
                .wrapping_add(8))
                .read(),
            );
            ScheduleBgCopyTilemapToVram(0u8);
            Free(
                (((&raw mut sSlot1TilemapBuffer)
                    .cast::<u8>()
                    .cast::<*mut u16>())
                .read())
                .cast::<u8>(),
            );
            Free(
                (((&raw mut sSlot2TilemapBuffer)
                    .cast::<u8>()
                    .cast::<*mut u16>())
                .read())
                .cast::<u8>(),
            );
            FinishTwoMonAction(taskId);
        } else {
            let __p1 = (data).wrapping_offset(8);
            (__p1).write(
                (((((__p1).read()) as i32)
                    .wrapping_add(((((data).wrapping_offset(10)).read()) as i32)))
                    as i16),
            );
            let __p2 = (data).wrapping_offset(9);
            (__p2).write(
                (((((__p2).read()) as i32)
                    .wrapping_add(((((data).wrapping_offset(11)).read()) as i32)))
                    as i16),
            );
            if ((((data).wrapping_offset(8)).read()) as i32) == 0i32 {
                ((data).wrapping_offset(10)).write(0i16);
            }
            if ((((data).wrapping_offset(9)).read()) as i32) == 0i32 {
                ((data).wrapping_offset(11)).write(0i16);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SwitchMenuBoxSprites(spriteIdPtr1: *mut u8, spriteIdPtr2: *mut u8) {
    unsafe {
        let mut spriteIdPtr1 = spriteIdPtr1;
        let mut spriteIdPtr2 = spriteIdPtr2;
        let mut spriteIdBuffer: u8 = (spriteIdPtr1).read();
        let mut xBuffer1: u16 = 0u16;
        let mut yBuffer1: u16 = 0u16;
        let mut xBuffer2: u16 = 0u16;
        let mut yBuffer2: u16 = 0u16;
        (spriteIdPtr1).write((spriteIdPtr2).read());
        (spriteIdPtr2).write(spriteIdBuffer);
        xBuffer1 = ((((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset((((spriteIdPtr1).read()) as i32) as isize * 68))
        .wrapping_add(32)
        .cast::<i16>())
        .read()) as u16);
        yBuffer1 = ((((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset((((spriteIdPtr1).read()) as i32) as isize * 68))
        .wrapping_add(34)
        .cast::<i16>())
        .read()) as u16);
        xBuffer2 = ((((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset((((spriteIdPtr1).read()) as i32) as isize * 68))
        .wrapping_add(36)
        .cast::<i16>())
        .read()) as u16);
        yBuffer2 = ((((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset((((spriteIdPtr1).read()) as i32) as isize * 68))
        .wrapping_add(38)
        .cast::<i16>())
        .read()) as u16);
        ((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset((((spriteIdPtr1).read()) as i32) as isize * 68))
        .wrapping_add(32)
        .cast::<i16>())
        .write(
            ((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset((((spriteIdPtr2).read()) as i32) as isize * 68))
            .wrapping_add(32)
            .cast::<i16>())
            .read(),
        );
        ((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset((((spriteIdPtr1).read()) as i32) as isize * 68))
        .wrapping_add(34)
        .cast::<i16>())
        .write(
            ((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset((((spriteIdPtr2).read()) as i32) as isize * 68))
            .wrapping_add(34)
            .cast::<i16>())
            .read(),
        );
        ((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset((((spriteIdPtr1).read()) as i32) as isize * 68))
        .wrapping_add(36)
        .cast::<i16>())
        .write(
            ((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset((((spriteIdPtr2).read()) as i32) as isize * 68))
            .wrapping_add(36)
            .cast::<i16>())
            .read(),
        );
        ((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset((((spriteIdPtr1).read()) as i32) as isize * 68))
        .wrapping_add(38)
        .cast::<i16>())
        .write(
            ((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset((((spriteIdPtr2).read()) as i32) as isize * 68))
            .wrapping_add(38)
            .cast::<i16>())
            .read(),
        );
        ((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset((((spriteIdPtr2).read()) as i32) as isize * 68))
        .wrapping_add(32)
        .cast::<i16>())
        .write(((xBuffer1) as i16));
        ((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset((((spriteIdPtr2).read()) as i32) as isize * 68))
        .wrapping_add(34)
        .cast::<i16>())
        .write(((yBuffer1) as i16));
        ((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset((((spriteIdPtr2).read()) as i32) as isize * 68))
        .wrapping_add(36)
        .cast::<i16>())
        .write(((xBuffer2) as i16));
        ((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset((((spriteIdPtr2).read()) as i32) as isize * 68))
        .wrapping_add(38)
        .cast::<i16>())
        .write(((yBuffer2) as i16));
    }
}
pub(crate) unsafe extern "C" fn SwitchPartyMon() {
    unsafe {
        let mut menuBoxes = crate::ffi::Align4([0u8; 8]);
        let mut mon1: *mut u8 = core::ptr::null_mut();
        let mut mon2: *mut u8 = core::ptr::null_mut();
        let mut monBuffer: *mut u8 = core::ptr::null_mut();
        ((&raw mut menuBoxes).cast::<*mut u8>()).write(
            (((&raw mut sPartyMenuBoxes).cast::<u8>().cast::<*mut u8>()).read()).wrapping_offset(
                (((((&raw mut gPartyMenu).cast::<u8>())
                    .wrapping_add(9)
                    .cast::<i8>())
                .read()) as i32) as isize
                    * 16,
            ),
        );
        (((&raw mut menuBoxes).cast::<*mut u8>()).wrapping_offset(1)).write(
            (((&raw mut sPartyMenuBoxes).cast::<u8>().cast::<*mut u8>()).read()).wrapping_offset(
                (((((&raw mut gPartyMenu).cast::<u8>())
                    .wrapping_add(10)
                    .cast::<i8>())
                .read()) as i32) as isize
                    * 16,
            ),
        );
        mon1 = ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
            (((((&raw mut gPartyMenu).cast::<u8>())
                .wrapping_add(9)
                .cast::<i8>())
            .read()) as i32) as isize
                * 100,
        );
        mon2 = ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
            (((((&raw mut gPartyMenu).cast::<u8>())
                .wrapping_add(10)
                .cast::<i8>())
            .read()) as i32) as isize
                * 100,
        );
        monBuffer = Alloc(100u32);
        monBuffer
            .cast::<crate::c::Rec4<100>>()
            .write_unaligned(mon1.cast::<crate::c::Rec4<100>>().read_unaligned());
        mon1.cast::<crate::c::Rec4<100>>()
            .write_unaligned(mon2.cast::<crate::c::Rec4<100>>().read_unaligned());
        mon2.cast::<crate::c::Rec4<100>>()
            .write_unaligned(monBuffer.cast::<crate::c::Rec4<100>>().read_unaligned());
        Free(monBuffer);
        SwitchMenuBoxSprites(
            (((&raw mut menuBoxes).cast::<*mut u8>()).read()).wrapping_add(11),
            ((((&raw mut menuBoxes).cast::<*mut u8>()).wrapping_offset(1)).read()).wrapping_add(11),
        );
        SwitchMenuBoxSprites(
            (((&raw mut menuBoxes).cast::<*mut u8>()).read()).wrapping_add(10),
            ((((&raw mut menuBoxes).cast::<*mut u8>()).wrapping_offset(1)).read()).wrapping_add(10),
        );
        SwitchMenuBoxSprites(
            (((&raw mut menuBoxes).cast::<*mut u8>()).read()).wrapping_add(9),
            ((((&raw mut menuBoxes).cast::<*mut u8>()).wrapping_offset(1)).read()).wrapping_add(9),
        );
        SwitchMenuBoxSprites(
            (((&raw mut menuBoxes).cast::<*mut u8>()).read()).wrapping_add(12),
            ((((&raw mut menuBoxes).cast::<*mut u8>()).wrapping_offset(1)).read()).wrapping_add(12),
        );
    }
}
pub(crate) unsafe extern "C" fn FinishTwoMonAction(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        PartyMenuRemoveWindow(
            (((((&raw mut sPartyMenuInternal).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(12))
            .cast::<u8>())
            .wrapping_offset(1),
        );
        (((&raw mut gPartyMenu).cast::<u8>()).wrapping_add(11)).write(0u8);
        AnimatePartySlot(
            (((((&raw mut gPartyMenu).cast::<u8>())
                .wrapping_add(9)
                .cast::<i8>())
            .read()) as u8),
            0u8,
        );
        (((&raw mut gPartyMenu).cast::<u8>())
            .wrapping_add(9)
            .cast::<i8>())
        .write(
            (((&raw mut gPartyMenu).cast::<u8>())
                .wrapping_add(10)
                .cast::<i8>())
            .read(),
        );
        AnimatePartySlot(
            (((((&raw mut gPartyMenu).cast::<u8>())
                .wrapping_add(10)
                .cast::<i8>())
            .read()) as u8),
            1u8,
        );
        DisplayPartyMenuStdMessage(0u32);
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Task_HandleChooseMonInput));
    }
}
pub(crate) unsafe extern "C" fn CursorCb_Cancel1(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        PlaySE(5u16);
        PartyMenuRemoveWindow(
            ((((&raw mut sPartyMenuInternal).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(12))
            .cast::<u8>(),
        );
        PartyMenuRemoveWindow(
            (((((&raw mut sPartyMenuInternal).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(12))
            .cast::<u8>())
            .wrapping_offset(1),
        );
        if ((crate::c::bf_read(
            ((&raw mut gPartyMenu).cast::<u8>()).wrapping_add(8),
            0,
            4,
            false,
        ) as u8) as i32)
            == 6i32
        {
            DisplayPartyMenuStdMessage(15u32);
        } else {
            DisplayPartyMenuStdMessage(0u32);
        }
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Task_HandleChooseMonInput));
    }
}
pub(crate) unsafe extern "C" fn CursorCb_Item(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        PlaySE(5u16);
        PartyMenuRemoveWindow(
            ((((&raw mut sPartyMenuInternal).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(12))
            .cast::<u8>(),
        );
        PartyMenuRemoveWindow(
            (((((&raw mut sPartyMenuInternal).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(12))
            .cast::<u8>())
            .wrapping_offset(1),
        );
        SetPartyMonSelectionActions(
            (&raw mut gPlayerParty).cast::<u8>(),
            (((((&raw mut gPartyMenu).cast::<u8>())
                .wrapping_add(9)
                .cast::<i8>())
            .read()) as u8),
            8u8,
        );
        DisplaySelectionWindow(1u8);
        DisplayPartyMenuStdMessage(24u32);
        (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .write(255i16);
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Task_HandleSelectionMenuInput));
    }
}
pub(crate) unsafe extern "C" fn CursorCb_Give(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        PlaySE(5u16);
        ((((&raw mut sPartyMenuInternal).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(4)
            .cast::<Option<unsafe extern "C" fn()>>())
        .write(Some(CB2_SelectBagItemToGive));
        Task_ClosePartyMenu(taskId);
    }
}
pub(crate) unsafe extern "C" fn CB2_SelectBagItemToGive() {
    unsafe {
        if ((((CurrentBattlePyramidLocation()) as i32) != 0i32) as i32) == 0i32 {
            GoToBagMenu(2u8, 5u8, Some(CB2_GiveHoldItem));
        } else {
            GoToBattlePyramidBagMenu(2u8, Some(CB2_GiveHoldItem));
        }
    }
}
pub(crate) unsafe extern "C" fn CB2_GiveHoldItem() {
    unsafe {
        if ((((&raw mut gSpecialVar_ItemId).cast::<u16>()).read()) as i32) == 0i32 {
            InitPartyMenu(
                (crate::c::bf_read(
                    ((&raw mut gPartyMenu).cast::<u8>()).wrapping_add(8),
                    0,
                    4,
                    false,
                ) as u8),
                255u8,
                (((&raw mut gPartyMenu).cast::<u8>()).wrapping_add(11)).read(),
                1u8,
                127u8,
                Some(Task_TryCreateSelectionWindow),
                (((&raw mut gPartyMenu).cast::<u8>()).cast::<Option<unsafe extern "C" fn()>>())
                    .read(),
            );
        } else {
            ((&raw mut sPartyMenuItemId).cast::<u8>().cast::<u16>()).write(
                ((GetMonData2(
                    ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                        (((((&raw mut gPartyMenu).cast::<u8>())
                            .wrapping_add(9)
                            .cast::<i8>())
                        .read()) as i32) as isize
                            * 100,
                    ),
                    12i32,
                )) as u16),
            );
            if ((((&raw mut sPartyMenuItemId).cast::<u8>().cast::<u16>()).read()) as i32) != 0i32 {
                InitPartyMenu(
                    (crate::c::bf_read(
                        ((&raw mut gPartyMenu).cast::<u8>()).wrapping_add(8),
                        0,
                        4,
                        false,
                    ) as u8),
                    255u8,
                    (((&raw mut gPartyMenu).cast::<u8>()).wrapping_add(11)).read(),
                    1u8,
                    127u8,
                    Some(Task_SwitchHoldItemsPrompt),
                    (((&raw mut gPartyMenu).cast::<u8>()).cast::<Option<unsafe extern "C" fn()>>())
                        .read(),
                );
            } else {
                if (ItemIsMail(((&raw mut gSpecialVar_ItemId).cast::<u16>()).read())) != 0 {
                    RemoveBagItem(((&raw mut gSpecialVar_ItemId).cast::<u16>()).read(), 1u16);
                    GiveItemToMon(
                        ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                            (((((&raw mut gPartyMenu).cast::<u8>())
                                .wrapping_add(9)
                                .cast::<i8>())
                            .read()) as i32) as isize
                                * 100,
                        ),
                        ((&raw mut gSpecialVar_ItemId).cast::<u16>()).read(),
                    );
                    CB2_WriteMailToGiveMon();
                } else {
                    InitPartyMenu(
                        (crate::c::bf_read(
                            ((&raw mut gPartyMenu).cast::<u8>()).wrapping_add(8),
                            0,
                            4,
                            false,
                        ) as u8),
                        255u8,
                        (((&raw mut gPartyMenu).cast::<u8>()).wrapping_add(11)).read(),
                        1u8,
                        127u8,
                        Some(Task_GiveHoldItem),
                        (((&raw mut gPartyMenu).cast::<u8>())
                            .cast::<Option<unsafe extern "C" fn()>>())
                        .read(),
                    );
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_GiveHoldItem(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut item: u16 = 0u16;
        if !((crate::c::bf_read(
            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
            7,
            1,
            false,
        ) as u16)
            != 0)
        {
            item = ((&raw mut gSpecialVar_ItemId).cast::<u16>()).read();
            DisplayGaveHeldItemMessage(
                ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                    (((((&raw mut gPartyMenu).cast::<u8>())
                        .wrapping_add(9)
                        .cast::<i8>())
                    .read()) as i32) as isize
                        * 100,
                ),
                item,
                0u8,
                0u8,
            );
            GiveItemToMon(
                ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                    (((((&raw mut gPartyMenu).cast::<u8>())
                        .wrapping_add(9)
                        .cast::<i8>())
                    .read()) as i32) as isize
                        * 100,
                ),
                item,
            );
            RemoveBagItem(item, 1u16);
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_UpdateHeldItemSprite));
        }
    }
}
pub(crate) unsafe extern "C" fn Task_SwitchHoldItemsPrompt(taskId: u8) {
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
            DisplayAlreadyHoldingItemSwitchMessage(
                ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                    (((((&raw mut gPartyMenu).cast::<u8>())
                        .wrapping_add(9)
                        .cast::<i8>())
                    .read()) as i32) as isize
                        * 100,
                ),
                ((&raw mut sPartyMenuItemId).cast::<u8>().cast::<u16>()).read(),
                1u8,
            );
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_SwitchItemsYesNo));
        }
    }
}
pub(crate) unsafe extern "C" fn Task_SwitchItemsYesNo(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if ((IsPartyMenuTextPrinterActive()) as i32) != 1i32 {
            PartyMenuDisplayYesNoMenu();
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_HandleSwitchItemsYesNoInput));
        }
    }
}
pub(crate) unsafe extern "C" fn Task_HandleSwitchItemsYesNoInput(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        'l1: {
            let __sw1 = ((Menu_ProcessInputNoWrapClearOnChoose()) as i32);
            let mut __fall = false;
            if __sw1 == 0i32 {
                __fall = true;
                RemoveBagItem(((&raw mut gSpecialVar_ItemId).cast::<u16>()).read(), 1u16);
                if ((AddBagItem(
                    ((&raw mut sPartyMenuItemId).cast::<u8>().cast::<u16>()).read(),
                    1u16,
                )) as i32)
                    == 0i32
                {
                    AddBagItem(((&raw mut gSpecialVar_ItemId).cast::<u16>()).read(), 1u16);
                    BufferBagFullCantTakeItemMessage(
                        ((&raw mut sPartyMenuItemId).cast::<u8>().cast::<u16>()).read(),
                    );
                    DisplayPartyMenuMessage((&raw mut gStringVar4).cast::<u8>(), 0u8);
                    ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .cast::<Option<unsafe extern "C" fn(u8)>>())
                    .write(Some(Task_ReturnToChooseMonAfterText));
                } else {
                    if (ItemIsMail(((&raw mut gSpecialVar_ItemId).cast::<u16>()).read())) != 0 {
                        GiveItemToMon(
                            ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                                (((((&raw mut gPartyMenu).cast::<u8>())
                                    .wrapping_add(9)
                                    .cast::<i8>())
                                .read()) as i32) as isize
                                    * 100,
                            ),
                            ((&raw mut gSpecialVar_ItemId).cast::<u16>()).read(),
                        );
                        ((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .cast::<Option<unsafe extern "C" fn(u8)>>())
                        .write(Some(Task_WriteMailToGiveMonAfterText));
                    } else {
                        GiveItemToMon(
                            ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                                (((((&raw mut gPartyMenu).cast::<u8>())
                                    .wrapping_add(9)
                                    .cast::<i8>())
                                .read()) as i32) as isize
                                    * 100,
                            ),
                            ((&raw mut gSpecialVar_ItemId).cast::<u16>()).read(),
                        );
                        DisplaySwitchedHeldItemMessage(
                            ((&raw mut gSpecialVar_ItemId).cast::<u16>()).read(),
                            ((&raw mut sPartyMenuItemId).cast::<u8>().cast::<u16>()).read(),
                            1u8,
                        );
                        ((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .cast::<Option<unsafe extern "C" fn(u8)>>())
                        .write(Some(Task_UpdateHeldItemSprite));
                    }
                }
                break 'l1;
            }
            if __sw1 == (-1i32) {
                __fall = true;
                PlaySE(5u16);
            }
            if __fall || __sw1 == 1i32 {
                __fall = true;
                ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
                .write(Some(Task_ReturnToChooseMonAfterText));
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_WriteMailToGiveMonAfterText(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if ((IsPartyMenuTextPrinterActive()) as i32) != 1i32 {
            ((((&raw mut sPartyMenuInternal).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(4)
                .cast::<Option<unsafe extern "C" fn()>>())
            .write(Some(CB2_WriteMailToGiveMon));
            Task_ClosePartyMenu(taskId);
        }
    }
}
pub(crate) unsafe extern "C" fn CB2_WriteMailToGiveMon() {
    unsafe {
        let mut mail: u8 = ((GetMonData2(
            ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                (((((&raw mut gPartyMenu).cast::<u8>())
                    .wrapping_add(9)
                    .cast::<i8>())
                .read()) as i32) as isize
                    * 100,
            ),
            64i32,
        )) as u8);
        DoEasyChatScreen(
            4u8,
            ((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(11232))
                .cast::<u8>())
            .wrapping_offset(((mail) as i32) as isize * 36))
            .cast::<u16>(),
            Some(CB2_ReturnToPartyMenuFromWritingMail),
            3u8,
        );
    }
}
pub(crate) unsafe extern "C" fn CB2_ReturnToPartyMenuFromWritingMail() {
    unsafe {
        let mut mon: *mut u8 = ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
            (((((&raw mut gPartyMenu).cast::<u8>())
                .wrapping_add(9)
                .cast::<i8>())
            .read()) as i32) as isize
                * 100,
        );
        let mut item: u16 = ((GetMonData2(mon, 12i32)) as u16);
        if ((((&raw mut gSpecialVar_Result).cast::<u16>()).read()) as i32) == 0i32 {
            TakeMailFromMon(mon);
            SetMonData(
                mon,
                12i32,
                ((&raw mut sPartyMenuItemId).cast::<u8>().cast::<u16>()).cast::<u8>(),
            );
            RemoveBagItem(
                ((&raw mut sPartyMenuItemId).cast::<u8>().cast::<u16>()).read(),
                1u16,
            );
            AddBagItem(item, 1u16);
            InitPartyMenu(
                (crate::c::bf_read(
                    ((&raw mut gPartyMenu).cast::<u8>()).wrapping_add(8),
                    0,
                    4,
                    false,
                ) as u8),
                255u8,
                (((&raw mut gPartyMenu).cast::<u8>()).wrapping_add(11)).read(),
                1u8,
                0u8,
                Some(Task_TryCreateSelectionWindow),
                (((&raw mut gPartyMenu).cast::<u8>()).cast::<Option<unsafe extern "C" fn()>>())
                    .read(),
            );
        } else {
            InitPartyMenu(
                (crate::c::bf_read(
                    ((&raw mut gPartyMenu).cast::<u8>()).wrapping_add(8),
                    0,
                    4,
                    false,
                ) as u8),
                255u8,
                (((&raw mut gPartyMenu).cast::<u8>()).wrapping_add(11)).read(),
                1u8,
                127u8,
                Some(Task_DisplayGaveMailFromPartyMessage),
                (((&raw mut gPartyMenu).cast::<u8>()).cast::<Option<unsafe extern "C" fn()>>())
                    .read(),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn Task_DisplayGaveMailFromPartyMessage(taskId: u8) {
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
            if ((((&raw mut sPartyMenuItemId).cast::<u8>().cast::<u16>()).read()) as i32) == 0i32 {
                DisplayGaveHeldItemMessage(
                    ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                        (((((&raw mut gPartyMenu).cast::<u8>())
                            .wrapping_add(9)
                            .cast::<i8>())
                        .read()) as i32) as isize
                            * 100,
                    ),
                    ((&raw mut gSpecialVar_ItemId).cast::<u16>()).read(),
                    0u8,
                    0u8,
                );
            } else {
                DisplaySwitchedHeldItemMessage(
                    ((&raw mut gSpecialVar_ItemId).cast::<u16>()).read(),
                    ((&raw mut sPartyMenuItemId).cast::<u8>().cast::<u16>()).read(),
                    0u8,
                );
            }
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_UpdateHeldItemSprite));
        }
    }
}
pub(crate) unsafe extern "C" fn Task_UpdateHeldItemSprite(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut mon: *mut u8 = ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
            (((((&raw mut gPartyMenu).cast::<u8>())
                .wrapping_add(9)
                .cast::<i8>())
            .read()) as i32) as isize
                * 100,
        );
        if ((IsPartyMenuTextPrinterActive()) as i32) != 1i32 {
            UpdatePartyMonHeldItemSprite(
                mon,
                (((&raw mut sPartyMenuBoxes).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_offset(
                        (((((&raw mut gPartyMenu).cast::<u8>())
                            .wrapping_add(9)
                            .cast::<i8>())
                        .read()) as i32) as isize
                            * 16,
                    ),
            );
            if ((crate::c::bf_read(
                ((&raw mut gPartyMenu).cast::<u8>()).wrapping_add(8),
                0,
                4,
                false,
            ) as u8) as i32)
                == 12i32
            {
                if GetMonData2(mon, 12i32) != 0u32 {
                    DisplayPartyPokemonDescriptionText(
                        11u8,
                        (((&raw mut sPartyMenuBoxes).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_offset(
                                (((((&raw mut gPartyMenu).cast::<u8>())
                                    .wrapping_add(9)
                                    .cast::<i8>())
                                .read()) as i32) as isize
                                    * 16,
                            ),
                        1u8,
                    );
                } else {
                    DisplayPartyPokemonDescriptionText(
                        12u8,
                        (((&raw mut sPartyMenuBoxes).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_offset(
                                (((((&raw mut gPartyMenu).cast::<u8>())
                                    .wrapping_add(9)
                                    .cast::<i8>())
                                .read()) as i32) as isize
                                    * 16,
                            ),
                        1u8,
                    );
                }
            }
            Task_ReturnToChooseMonAfterText(taskId);
        }
    }
}
pub(crate) unsafe extern "C" fn CursorCb_TakeItem(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut mon: *mut u8 = ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
            (((((&raw mut gPartyMenu).cast::<u8>())
                .wrapping_add(9)
                .cast::<i8>())
            .read()) as i32) as isize
                * 100,
        );
        let mut item: u16 = ((GetMonData2(mon, 12i32)) as u16);
        PlaySE(5u16);
        PartyMenuRemoveWindow(
            ((((&raw mut sPartyMenuInternal).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(12))
            .cast::<u8>(),
        );
        PartyMenuRemoveWindow(
            (((((&raw mut sPartyMenuInternal).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(12))
            .cast::<u8>())
            .wrapping_offset(1),
        );
        'l1: {
            let __sw1 = ((TryTakeMonItem(mon)) as i32);
            let __matched = __sw1 == 0i32 || __sw1 == 1i32;
            if __sw1 == 0i32 {
                GetMonNickname(mon, (&raw mut gStringVar1).cast::<u8>());
                StringExpandPlaceholders(
                    (&raw mut gStringVar4).cast::<u8>(),
                    (&raw mut gText_PkmnNotHolding).cast::<u8>(),
                );
                DisplayPartyMenuMessage((&raw mut gStringVar4).cast::<u8>(), 1u8);
                break 'l1;
            }
            if __sw1 == 1i32 {
                BufferBagFullCantTakeItemMessage(item);
                DisplayPartyMenuMessage((&raw mut gStringVar4).cast::<u8>(), 1u8);
                break 'l1;
            }
            if !__matched {
                DisplayTookHeldItemMessage(mon, item, 1u8);
                break 'l1;
            }
        }
        ScheduleBgCopyTilemapToVram(2u8);
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Task_UpdateHeldItemSprite));
    }
}
pub(crate) unsafe extern "C" fn CursorCb_Toss(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut mon: *mut u8 = ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
            (((((&raw mut gPartyMenu).cast::<u8>())
                .wrapping_add(9)
                .cast::<i8>())
            .read()) as i32) as isize
                * 100,
        );
        let mut item: u16 = ((GetMonData2(mon, 12i32)) as u16);
        PlaySE(5u16);
        PartyMenuRemoveWindow(
            ((((&raw mut sPartyMenuInternal).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(12))
            .cast::<u8>(),
        );
        PartyMenuRemoveWindow(
            (((((&raw mut sPartyMenuInternal).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(12))
            .cast::<u8>())
            .wrapping_offset(1),
        );
        if ((item) as i32) == 0i32 {
            GetMonNickname(mon, (&raw mut gStringVar1).cast::<u8>());
            StringExpandPlaceholders(
                (&raw mut gStringVar4).cast::<u8>(),
                (&raw mut gText_PkmnNotHolding).cast::<u8>(),
            );
            DisplayPartyMenuMessage((&raw mut gStringVar4).cast::<u8>(), 1u8);
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_UpdateHeldItemSprite));
        } else {
            CopyItemName(item, (&raw mut gStringVar1).cast::<u8>());
            StringExpandPlaceholders(
                (&raw mut gStringVar4).cast::<u8>(),
                (&raw mut gText_ThrowAwayItem).cast::<u8>(),
            );
            DisplayPartyMenuMessage((&raw mut gStringVar4).cast::<u8>(), 1u8);
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_TossHeldItemYesNo));
        }
    }
}
pub(crate) unsafe extern "C" fn Task_TossHeldItemYesNo(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if ((IsPartyMenuTextPrinterActive()) as i32) != 1i32 {
            PartyMenuDisplayYesNoMenu();
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_HandleTossHeldItemYesNoInput));
        }
    }
}
pub(crate) unsafe extern "C" fn Task_HandleTossHeldItemYesNoInput(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut mon: *mut u8 = ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
            (((((&raw mut gPartyMenu).cast::<u8>())
                .wrapping_add(9)
                .cast::<i8>())
            .read()) as i32) as isize
                * 100,
        );
        'l1: {
            let __sw1 = ((Menu_ProcessInputNoWrapClearOnChoose()) as i32);
            let mut __fall = false;
            if __sw1 == 0i32 {
                __fall = true;
                CopyItemName(
                    ((GetMonData2(mon, 12i32)) as u16),
                    (&raw mut gStringVar1).cast::<u8>(),
                );
                StringExpandPlaceholders(
                    (&raw mut gStringVar4).cast::<u8>(),
                    (&raw mut gText_ItemThrownAway).cast::<u8>(),
                );
                DisplayPartyMenuMessage((&raw mut gStringVar4).cast::<u8>(), 0u8);
                ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
                .write(Some(Task_TossHeldItem));
                break 'l1;
            }
            if __sw1 == (-1i32) {
                __fall = true;
                PlaySE(5u16);
            }
            if __fall || __sw1 == 1i32 {
                __fall = true;
                ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
                .write(Some(Task_ReturnToChooseMonAfterText));
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_TossHeldItem(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut mon: *mut u8 = ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
            (((((&raw mut gPartyMenu).cast::<u8>())
                .wrapping_add(9)
                .cast::<i8>())
            .read()) as i32) as isize
                * 100,
        );
        if ((IsPartyMenuTextPrinterActive()) as i32) != 1i32 {
            let mut item: u16 = 0u16;
            SetMonData(mon, 12i32, (&raw mut item).cast::<u8>());
            UpdatePartyMonHeldItemSprite(
                mon,
                (((&raw mut sPartyMenuBoxes).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_offset(
                        (((((&raw mut gPartyMenu).cast::<u8>())
                            .wrapping_add(9)
                            .cast::<i8>())
                        .read()) as i32) as isize
                            * 16,
                    ),
            );
            DisplayPartyPokemonDescriptionText(
                12u8,
                (((&raw mut sPartyMenuBoxes).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_offset(
                        (((((&raw mut gPartyMenu).cast::<u8>())
                            .wrapping_add(9)
                            .cast::<i8>())
                        .read()) as i32) as isize
                            * 16,
                    ),
                1u8,
            );
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_ReturnToChooseMonAfterText));
        }
    }
}
pub(crate) unsafe extern "C" fn CursorCb_Mail(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        PlaySE(5u16);
        PartyMenuRemoveWindow(
            ((((&raw mut sPartyMenuInternal).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(12))
            .cast::<u8>(),
        );
        PartyMenuRemoveWindow(
            (((((&raw mut sPartyMenuInternal).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(12))
            .cast::<u8>())
            .wrapping_offset(1),
        );
        SetPartyMonSelectionActions(
            (&raw mut gPlayerParty).cast::<u8>(),
            (((((&raw mut gPartyMenu).cast::<u8>())
                .wrapping_add(9)
                .cast::<i8>())
            .read()) as u8),
            9u8,
        );
        DisplaySelectionWindow(2u8);
        DisplayPartyMenuStdMessage(25u32);
        (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .write(255i16);
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Task_HandleSelectionMenuInput));
    }
}
pub(crate) unsafe extern "C" fn CursorCb_Read(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        PlaySE(5u16);
        ((((&raw mut sPartyMenuInternal).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(4)
            .cast::<Option<unsafe extern "C" fn()>>())
        .write(Some(CB2_ReadHeldMail));
        Task_ClosePartyMenu(taskId);
    }
}
pub(crate) unsafe extern "C" fn CB2_ReadHeldMail() {
    unsafe {
        ReadMail(
            (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(11232))
                .cast::<u8>())
            .wrapping_offset(
                ((GetMonData2(
                    ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                        (((((&raw mut gPartyMenu).cast::<u8>())
                            .wrapping_add(9)
                            .cast::<i8>())
                        .read()) as i32) as isize
                            * 100,
                    ),
                    64i32,
                )) as i32) as isize
                    * 36,
            ),
            Some(CB2_ReturnToPartyMenuFromReadingMail),
            1u8,
        );
    }
}
pub(crate) unsafe extern "C" fn CB2_ReturnToPartyMenuFromReadingMail() {
    unsafe {
        crate::c::bf_write(
            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(8),
            7,
            1,
            (1u16) as i32,
        );
        InitPartyMenu(
            (crate::c::bf_read(
                ((&raw mut gPartyMenu).cast::<u8>()).wrapping_add(8),
                0,
                4,
                false,
            ) as u8),
            255u8,
            (((&raw mut gPartyMenu).cast::<u8>()).wrapping_add(11)).read(),
            1u8,
            21u8,
            Some(Task_TryCreateSelectionWindow),
            (((&raw mut gPartyMenu).cast::<u8>()).cast::<Option<unsafe extern "C" fn()>>()).read(),
        );
    }
}
pub(crate) unsafe extern "C" fn CursorCb_TakeMail(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        PlaySE(5u16);
        PartyMenuRemoveWindow(
            (((((&raw mut sPartyMenuInternal).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(12))
            .cast::<u8>())
            .wrapping_offset(1),
        );
        PartyMenuRemoveWindow(
            ((((&raw mut sPartyMenuInternal).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(12))
            .cast::<u8>(),
        );
        DisplayPartyMenuMessage((&raw mut gText_SendMailToPC).cast::<u8>(), 1u8);
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Task_SendMailToPCYesNo));
    }
}
pub(crate) unsafe extern "C" fn Task_SendMailToPCYesNo(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if ((IsPartyMenuTextPrinterActive()) as i32) != 1i32 {
            PartyMenuDisplayYesNoMenu();
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_HandleSendMailToPCYesNoInput));
        }
    }
}
pub(crate) unsafe extern "C" fn Task_HandleSendMailToPCYesNoInput(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        'l1: {
            let __sw1 = ((Menu_ProcessInputNoWrapClearOnChoose()) as i32);
            let mut __fall = false;
            if __sw1 == 0i32 {
                __fall = true;
                if ((TakeMailFromMonAndSave(
                    ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                        (((((&raw mut gPartyMenu).cast::<u8>())
                            .wrapping_add(9)
                            .cast::<i8>())
                        .read()) as i32) as isize
                            * 100,
                    ),
                )) as i32)
                    != 255i32
                {
                    DisplayPartyMenuMessage((&raw mut gText_MailSentToPC).cast::<u8>(), 0u8);
                    ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .cast::<Option<unsafe extern "C" fn(u8)>>())
                    .write(Some(Task_UpdateHeldItemSprite));
                } else {
                    DisplayPartyMenuMessage((&raw mut gText_PCMailboxFull).cast::<u8>(), 0u8);
                    ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .cast::<Option<unsafe extern "C" fn(u8)>>())
                    .write(Some(Task_ReturnToChooseMonAfterText));
                }
                break 'l1;
            }
            if __sw1 == (-1i32) {
                __fall = true;
                PlaySE(5u16);
            }
            if __fall || __sw1 == 1i32 {
                __fall = true;
                DisplayPartyMenuMessage((&raw mut gText_MailMessageWillBeLost).cast::<u8>(), 1u8);
                ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
                .write(Some(Task_LoseMailMessageYesNo));
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_LoseMailMessageYesNo(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if ((IsPartyMenuTextPrinterActive()) as i32) != 1i32 {
            PartyMenuDisplayYesNoMenu();
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_HandleLoseMailMessageYesNoInput));
        }
    }
}
pub(crate) unsafe extern "C" fn Task_HandleLoseMailMessageYesNoInput(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut item: u16 = 0u16;
        'l1: {
            let __sw1 = ((Menu_ProcessInputNoWrapClearOnChoose()) as i32);
            let mut __fall = false;
            if __sw1 == 0i32 {
                __fall = true;
                item = ((GetMonData2(
                    ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                        (((((&raw mut gPartyMenu).cast::<u8>())
                            .wrapping_add(9)
                            .cast::<i8>())
                        .read()) as i32) as isize
                            * 100,
                    ),
                    12i32,
                )) as u16);
                if ((AddBagItem(item, 1u16)) as i32) == 1i32 {
                    TakeMailFromMon(
                        ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                            (((((&raw mut gPartyMenu).cast::<u8>())
                                .wrapping_add(9)
                                .cast::<i8>())
                            .read()) as i32) as isize
                                * 100,
                        ),
                    );
                    DisplayPartyMenuMessage((&raw mut gText_MailTakenFromPkmn).cast::<u8>(), 0u8);
                    ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .cast::<Option<unsafe extern "C" fn(u8)>>())
                    .write(Some(Task_UpdateHeldItemSprite));
                } else {
                    BufferBagFullCantTakeItemMessage(item);
                    DisplayPartyMenuMessage((&raw mut gStringVar4).cast::<u8>(), 0u8);
                    ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .cast::<Option<unsafe extern "C" fn(u8)>>())
                    .write(Some(Task_ReturnToChooseMonAfterText));
                }
                break 'l1;
            }
            if __sw1 == (-1i32) {
                __fall = true;
                PlaySE(5u16);
            }
            if __fall || __sw1 == 1i32 {
                __fall = true;
                ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
                .write(Some(Task_ReturnToChooseMonAfterText));
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn CursorCb_Cancel2(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut mon: *mut u8 = ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
            (((((&raw mut gPartyMenu).cast::<u8>())
                .wrapping_add(9)
                .cast::<i8>())
            .read()) as i32) as isize
                * 100,
        );
        PlaySE(5u16);
        PartyMenuRemoveWindow(
            ((((&raw mut sPartyMenuInternal).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(12))
            .cast::<u8>(),
        );
        PartyMenuRemoveWindow(
            (((((&raw mut sPartyMenuInternal).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(12))
            .cast::<u8>())
            .wrapping_offset(1),
        );
        SetPartyMonSelectionActions(
            (&raw mut gPlayerParty).cast::<u8>(),
            (((((&raw mut gPartyMenu).cast::<u8>())
                .wrapping_add(9)
                .cast::<i8>())
            .read()) as u8),
            GetPartyMenuActionsType(mon),
        );
        if ((crate::c::bf_read(
            ((&raw mut gPartyMenu).cast::<u8>()).wrapping_add(8),
            0,
            4,
            false,
        ) as u8) as i32)
            != 12i32
        {
            DisplaySelectionWindow(0u8);
            DisplayPartyMenuStdMessage(21u32);
        } else {
            DisplaySelectionWindow(1u8);
            CopyItemName(
                ((GetMonData2(mon, 12i32)) as u16),
                (&raw mut gStringVar2).cast::<u8>(),
            );
            DisplayPartyMenuStdMessage(26u32);
        }
        (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .write(255i16);
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Task_HandleSelectionMenuInput));
    }
}
pub(crate) unsafe extern "C" fn CursorCb_SendMon(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        PlaySE(5u16);
        PartyMenuRemoveWindow(
            ((((&raw mut sPartyMenuInternal).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(12))
            .cast::<u8>(),
        );
        if ((TrySwitchInPokemon()) as i32) == 1i32 {
            Task_ClosePartyMenu(taskId);
        } else {
            PartyMenuRemoveWindow(
                (((((&raw mut sPartyMenuInternal).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(12))
                .cast::<u8>())
                .wrapping_offset(1),
            );
            DisplayPartyMenuMessage((&raw mut gStringVar4).cast::<u8>(), 1u8);
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_ReturnToChooseMonAfterText));
        }
    }
}
pub(crate) unsafe extern "C" fn CursorCb_Enter(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut maxBattlers: u8 = 0u8;
        let mut i: u8 = 0u8;
        PartyMenuRemoveWindow(
            ((((&raw mut sPartyMenuInternal).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(12))
            .cast::<u8>(),
        );
        PartyMenuRemoveWindow(
            (((((&raw mut sPartyMenuInternal).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(12))
            .cast::<u8>())
            .wrapping_offset(1),
        );
        maxBattlers = GetMaxBattleEntries();
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < ((maxBattlers) as i32)) {
                    break 'l1;
                }
                'l2: {
                    if ((((((&raw mut gSelectedOrderFromParty).cast::<u8>()).cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize))
                    .read()) as i32)
                        == 0i32
                    {
                        PlaySE(5u16);
                        ((((&raw mut gSelectedOrderFromParty).cast::<u8>()).cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize))
                        .write(
                            (((((((&raw mut gPartyMenu).cast::<u8>())
                                .wrapping_add(9)
                                .cast::<i8>())
                            .read()) as i32)
                                .wrapping_add(1i32)) as u8),
                        );
                        DisplayPartyPokemonDescriptionText(
                            ((((i) as i32).wrapping_add(2i32)) as u8),
                            (((&raw mut sPartyMenuBoxes).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_offset(
                                    (((((&raw mut gPartyMenu).cast::<u8>())
                                        .wrapping_add(9)
                                        .cast::<i8>())
                                    .read()) as i32) as isize
                                        * 16,
                                ),
                            1u8,
                        );
                        if ((i) as i32) == ((maxBattlers) as i32).wrapping_sub(1i32) {
                            MoveCursorToConfirm();
                        }
                        DisplayPartyMenuStdMessage(0u32);
                        ((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .cast::<Option<unsafe extern "C" fn(u8)>>())
                        .write(Some(Task_HandleChooseMonInput));
                        return;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        ConvertIntToDecimalStringN(
            (&raw mut gStringVar1).cast::<u8>(),
            ((maxBattlers) as i32),
            0i32,
            1u8,
        );
        StringExpandPlaceholders(
            (&raw mut gStringVar4).cast::<u8>(),
            (&raw mut gText_NoMoreThanVar1Pkmn).cast::<u8>(),
        );
        PlaySE(32u16);
        DisplayPartyMenuMessage((&raw mut gStringVar4).cast::<u8>(), 1u8);
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Task_ReturnToChooseMonAfterText));
    }
}
pub(crate) unsafe extern "C" fn MoveCursorToConfirm() {
    unsafe {
        AnimatePartySlot(
            (((((&raw mut gPartyMenu).cast::<u8>())
                .wrapping_add(9)
                .cast::<i8>())
            .read()) as u8),
            0u8,
        );
        (((&raw mut gPartyMenu).cast::<u8>())
            .wrapping_add(9)
            .cast::<i8>())
        .write(6i8);
        AnimatePartySlot(
            (((((&raw mut gPartyMenu).cast::<u8>())
                .wrapping_add(9)
                .cast::<i8>())
            .read()) as u8),
            1u8,
        );
    }
}
pub(crate) unsafe extern "C" fn CursorCb_NoEntry(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut maxBattlers: u8 = 0u8;
        let mut i: u8 = 0u8;
        let mut j: u8 = 0u8;
        PlaySE(5u16);
        PartyMenuRemoveWindow(
            ((((&raw mut sPartyMenuInternal).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(12))
            .cast::<u8>(),
        );
        PartyMenuRemoveWindow(
            (((((&raw mut sPartyMenuInternal).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(12))
            .cast::<u8>())
            .wrapping_offset(1),
        );
        maxBattlers = GetMaxBattleEntries();
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < ((maxBattlers) as i32)) {
                    break 'l1;
                }
                'l2: {
                    if ((((((&raw mut gSelectedOrderFromParty).cast::<u8>()).cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize))
                    .read()) as i32)
                        == (((((&raw mut gPartyMenu).cast::<u8>())
                            .wrapping_add(9)
                            .cast::<i8>())
                        .read()) as i32)
                            .wrapping_add(1i32)
                    {
                        {
                            j = i;
                            'l3: loop {
                                if !(((j) as i32) < ((maxBattlers) as i32).wrapping_sub(1i32)) {
                                    break 'l3;
                                }
                                'l4: {
                                    ((((&raw mut gSelectedOrderFromParty).cast::<u8>())
                                        .cast::<u8>())
                                    .wrapping_offset(((j) as i32) as isize))
                                    .write(
                                        ((((&raw mut gSelectedOrderFromParty).cast::<u8>())
                                            .cast::<u8>())
                                        .wrapping_offset(
                                            (((j) as i32).wrapping_add(1i32)) as isize,
                                        ))
                                        .read(),
                                    );
                                }
                                j = (j).wrapping_add(1);
                            }
                        }
                        ((((&raw mut gSelectedOrderFromParty).cast::<u8>()).cast::<u8>())
                            .wrapping_offset(((j) as i32) as isize))
                        .write(0u8);
                        break 'l1;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        DisplayPartyPokemonDescriptionText(
            1u8,
            (((&raw mut sPartyMenuBoxes).cast::<u8>().cast::<*mut u8>()).read()).wrapping_offset(
                (((((&raw mut gPartyMenu).cast::<u8>())
                    .wrapping_add(9)
                    .cast::<i8>())
                .read()) as i32) as isize
                    * 16,
            ),
            1u8,
        );
        {
            i = 0u8;
            'l5: loop {
                if !(((i) as i32) < ((maxBattlers) as i32).wrapping_sub(1i32)) {
                    break 'l5;
                }
                'l6: {
                    if ((((((&raw mut gSelectedOrderFromParty).cast::<u8>()).cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize))
                    .read()) as i32)
                        != 0i32
                    {
                        DisplayPartyPokemonDescriptionText(
                            ((((i) as i32).wrapping_add(2i32)) as u8),
                            (((&raw mut sPartyMenuBoxes).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_offset(
                                    (((((((&raw mut gSelectedOrderFromParty).cast::<u8>())
                                        .cast::<u8>())
                                    .wrapping_offset(((i) as i32) as isize))
                                    .read()) as i32)
                                        .wrapping_sub(1i32))
                                        as isize
                                        * 16,
                                ),
                            1u8,
                        );
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        DisplayPartyMenuStdMessage(0u32);
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Task_HandleChooseMonInput));
    }
}
pub(crate) unsafe extern "C" fn CursorCb_Store(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        PlaySE(5u16);
        Task_ClosePartyMenu(taskId);
    }
}
pub(crate) unsafe extern "C" fn CursorCb_Register(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut species2: u16 = ((GetMonData2(
            ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                (((((&raw mut gPartyMenu).cast::<u8>())
                    .wrapping_add(9)
                    .cast::<i8>())
                .read()) as i32) as isize
                    * 100,
            ),
            65i32,
        )) as u16);
        let mut species: u16 = ((GetMonData2(
            ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                (((((&raw mut gPartyMenu).cast::<u8>())
                    .wrapping_add(9)
                    .cast::<i8>())
                .read()) as i32) as isize
                    * 100,
            ),
            11i32,
        )) as u16);
        let mut isModernFatefulEncounter: u8 = ((GetMonData2(
            ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                (((((&raw mut gPartyMenu).cast::<u8>())
                    .wrapping_add(9)
                    .cast::<i8>())
                .read()) as i32) as isize
                    * 100,
            ),
            80i32,
        )) as u8);
        'l1: {
            let __sw1 = CanRegisterMonForTradingBoard(
                GetHostRfuGameData()
                    .cast::<crate::c::Rec4<4>>()
                    .read_unaligned(),
                species2,
                species,
                isModernFatefulEncounter,
            );
            let __matched = __sw1 == 1i32 || __sw1 == 2i32;
            if __sw1 == 1i32 {
                StringExpandPlaceholders(
                    (&raw mut gStringVar4).cast::<u8>(),
                    (&raw mut gText_PkmnCantBeTradedNow).cast::<u8>(),
                );
                break 'l1;
            }
            if __sw1 == 2i32 {
                StringExpandPlaceholders(
                    (&raw mut gStringVar4).cast::<u8>(),
                    (&raw mut gText_EggCantBeTradedNow).cast::<u8>(),
                );
                break 'l1;
            }
            if !__matched {
                PlaySE(5u16);
                Task_ClosePartyMenu(taskId);
                return;
            }
        }
        PlaySE(32u16);
        PartyMenuRemoveWindow(
            ((((&raw mut sPartyMenuInternal).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(12))
            .cast::<u8>(),
        );
        PartyMenuRemoveWindow(
            (((((&raw mut sPartyMenuInternal).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(12))
            .cast::<u8>())
            .wrapping_offset(1),
        );
        StringAppend(
            (&raw mut gStringVar4).cast::<u8>(),
            (&raw mut gText_PauseUntilPress).cast::<u8>(),
        );
        DisplayPartyMenuMessage((&raw mut gStringVar4).cast::<u8>(), 1u8);
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Task_ReturnToChooseMonAfterText));
    }
}
pub(crate) unsafe extern "C" fn CursorCb_Trade1(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut species2: u16 = ((GetMonData2(
            ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                (((((&raw mut gPartyMenu).cast::<u8>())
                    .wrapping_add(9)
                    .cast::<i8>())
                .read()) as i32) as isize
                    * 100,
            ),
            65i32,
        )) as u16);
        let mut species: u16 = ((GetMonData2(
            ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                (((((&raw mut gPartyMenu).cast::<u8>())
                    .wrapping_add(9)
                    .cast::<i8>())
                .read()) as i32) as isize
                    * 100,
            ),
            11i32,
        )) as u16);
        let mut isModernFatefulEncounter: u8 = ((GetMonData2(
            ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                (((((&raw mut gPartyMenu).cast::<u8>())
                    .wrapping_add(9)
                    .cast::<i8>())
                .read()) as i32) as isize
                    * 100,
            ),
            80i32,
        )) as u8);
        let mut stringId: u32 = ((GetUnionRoomTradeMessageId(
            GetHostRfuGameData()
                .cast::<crate::c::Rec4<4>>()
                .read_unaligned(),
            (&raw mut gRfuPartnerCompatibilityData)
                .cast::<u8>()
                .cast::<crate::c::Rec4<4>>()
                .read_unaligned(),
            species2,
            ((&raw mut gUnionRoomOfferedSpecies).cast::<u16>()).read(),
            ((&raw mut gUnionRoomRequestedMonType).cast::<u8>()).read(),
            species,
            isModernFatefulEncounter,
        )) as u32);
        if stringId != 0u32 {
            StringExpandPlaceholders(
                (&raw mut gStringVar4).cast::<u8>(),
                ((((&raw const sUnionRoomTradeMessages)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<*mut u8>())
                .cast::<*mut u8>())
                .wrapping_offset((((stringId).wrapping_sub(1u32)) as i32) as isize))
                .read(),
            );
            PlaySE(32u16);
            PartyMenuRemoveWindow(
                ((((&raw mut sPartyMenuInternal).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(12))
                .cast::<u8>(),
            );
            PartyMenuRemoveWindow(
                (((((&raw mut sPartyMenuInternal).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(12))
                .cast::<u8>())
                .wrapping_offset(1),
            );
            StringAppend(
                (&raw mut gStringVar4).cast::<u8>(),
                (&raw mut gText_PauseUntilPress).cast::<u8>(),
            );
            DisplayPartyMenuMessage((&raw mut gStringVar4).cast::<u8>(), 1u8);
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_ReturnToChooseMonAfterText));
        } else {
            PlaySE(5u16);
            Task_ClosePartyMenu(taskId);
        }
    }
}
pub(crate) unsafe extern "C" fn CursorCb_Trade2(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        PartyMenuRemoveWindow(
            ((((&raw mut sPartyMenuInternal).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(12))
            .cast::<u8>(),
        );
        PartyMenuRemoveWindow(
            (((((&raw mut sPartyMenuInternal).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(12))
            .cast::<u8>())
            .wrapping_offset(1),
        );
        'l1: {
            let __sw1 = CanSpinTradeMon(
                (&raw mut gPlayerParty).cast::<u8>(),
                (((((&raw mut gPartyMenu).cast::<u8>())
                    .wrapping_add(9)
                    .cast::<i8>())
                .read()) as u16),
            );
            let __matched = __sw1 == 1i32 || __sw1 == 2i32 || __sw1 == 3i32;
            if __sw1 == 1i32 {
                StringExpandPlaceholders(
                    (&raw mut gStringVar4).cast::<u8>(),
                    (&raw mut gText_OnlyPkmnForBattle).cast::<u8>(),
                );
                break 'l1;
            }
            if __sw1 == 2i32 {
                StringExpandPlaceholders(
                    (&raw mut gStringVar4).cast::<u8>(),
                    (&raw mut gText_PkmnCantBeTradedNow).cast::<u8>(),
                );
                break 'l1;
            }
            if __sw1 == 3i32 {
                StringExpandPlaceholders(
                    (&raw mut gStringVar4).cast::<u8>(),
                    (&raw mut gText_EggCantBeTradedNow).cast::<u8>(),
                );
                break 'l1;
            }
            if !__matched {
                PlaySE(5u16);
                GetMonNickname(
                    ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                        (((((&raw mut gPartyMenu).cast::<u8>())
                            .wrapping_add(9)
                            .cast::<i8>())
                        .read()) as i32) as isize
                            * 100,
                    ),
                    (&raw mut gStringVar1).cast::<u8>(),
                );
                StringExpandPlaceholders(
                    (&raw mut gStringVar4).cast::<u8>(),
                    (&raw mut gJPText_AreYouSureYouWantToSpinTradeMon).cast::<u8>(),
                );
                DisplayPartyMenuMessage((&raw mut gStringVar4).cast::<u8>(), 1u8);
                ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
                .write(Some(Task_SpinTradeYesNo));
                return;
            }
        }
        PlaySE(32u16);
        StringAppend(
            (&raw mut gStringVar4).cast::<u8>(),
            (&raw mut gText_PauseUntilPress).cast::<u8>(),
        );
        DisplayPartyMenuMessage((&raw mut gStringVar4).cast::<u8>(), 1u8);
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Task_ReturnToChooseMonAfterText));
    }
}
pub(crate) unsafe extern "C" fn Task_SpinTradeYesNo(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if ((IsPartyMenuTextPrinterActive()) as i32) != 1i32 {
            PartyMenuDisplayYesNoMenu();
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_HandleSpinTradeYesNoInput));
        }
    }
}
pub(crate) unsafe extern "C" fn Task_HandleSpinTradeYesNoInput(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        'l1: {
            let __sw1 = ((Menu_ProcessInputNoWrapClearOnChoose()) as i32);
            let mut __fall = false;
            if __sw1 == 0i32 {
                __fall = true;
                Task_ClosePartyMenu(taskId);
                break 'l1;
            }
            if __sw1 == (-1i32) {
                __fall = true;
                PlaySE(5u16);
            }
            if __fall || __sw1 == 1i32 {
                __fall = true;
                Task_ReturnToChooseMonAfterText(taskId);
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn CursorCb_FieldMove(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut fieldMove: u8 =
            ((((((((((&raw mut sPartyMenuInternal).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(15))
            .cast::<u8>())
            .wrapping_offset(((Menu_GetCursorPos()) as i32) as isize))
            .read()) as i32)
                .wrapping_sub(19i32)) as u8);
        let mut mapHeader: *mut u8 = core::ptr::null_mut();
        PlaySE(5u16);
        if core::mem::transmute::<_, usize>(
            (((((&raw const sFieldMoveCursorCallbacks)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>())
            .wrapping_offset(((fieldMove) as i32) as isize * 8))
            .cast::<Option<unsafe extern "C" fn() -> u8>>())
            .read(),
        ) == 0usize
        {
            return;
        }
        PartyMenuRemoveWindow(
            ((((&raw mut sPartyMenuInternal).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(12))
            .cast::<u8>(),
        );
        PartyMenuRemoveWindow(
            (((((&raw mut sPartyMenuInternal).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(12))
            .cast::<u8>())
            .wrapping_offset(1),
        );
        if (((MenuHelpers_IsLinkActive()) as i32) == 1i32) || (InUnionRoom() == 1u32) {
            if (((fieldMove) as i32) == 11i32) || (((fieldMove) as i32) == 12i32) {
                DisplayPartyMenuStdMessage(13u32);
            } else {
                DisplayPartyMenuStdMessage(
                    (((((((&raw const sFieldMoveCursorCallbacks)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>())
                    .wrapping_offset(((fieldMove) as i32) as isize * 8))
                    .wrapping_add(4))
                    .read()) as u32),
                );
            }
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_CancelAfterAorBPress));
        } else {
            if (((fieldMove) as i32) <= 7i32)
                && (((FlagGet((((2151i32).wrapping_add(((fieldMove) as i32))) as u16))) as i32)
                    != 1i32)
            {
                DisplayPartyMenuMessage((&raw mut gText_CantUseUntilNewBadge).cast::<u8>(), 1u8);
                ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
                .write(Some(Task_ReturnToChooseMonAfterText));
            } else {
                if ((((((((&raw const sFieldMoveCursorCallbacks)
                    .cast::<u8>()
                    .cast_mut())
                .cast::<u8>())
                .wrapping_offset(((fieldMove) as i32) as isize * 8))
                .cast::<Option<unsafe extern "C" fn() -> u8>>())
                .read())
                .unwrap_unchecked()()) as i32)
                    == 1i32
                {
                    'l1: {
                        let __sw1 = ((fieldMove) as i32);
                        let __matched = __sw1 == 11i32
                            || __sw1 == 12i32
                            || __sw1 == 8i32
                            || __sw1 == 9i32
                            || __sw1 == 5i32;
                        if __sw1 == 11i32 || __sw1 == 12i32 {
                            ChooseMonForSoftboiled(taskId);
                            break 'l1;
                        }
                        if __sw1 == 8i32 {
                            mapHeader = Overworld_GetMapHeaderByGroupAndId(
                                (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                                    .wrapping_add(28))
                                .cast::<i8>())
                                .read()) as u16),
                                (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                                    .wrapping_add(28))
                                .wrapping_add(1)
                                .cast::<i8>())
                                .read()) as u16),
                            );
                            GetMapNameGeneric(
                                (&raw mut gStringVar1).cast::<u8>(),
                                ((((mapHeader).wrapping_add(20)).read()) as u16),
                            );
                            StringExpandPlaceholders(
                                (&raw mut gStringVar4).cast::<u8>(),
                                (&raw mut gText_ReturnToHealingSpot).cast::<u8>(),
                            );
                            DisplayFieldMoveExitAreaMessage(taskId);
                            (((((&raw mut sPartyMenuInternal).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(536))
                            .cast::<i16>())
                            .write(((fieldMove) as i16));
                            break 'l1;
                        }
                        if __sw1 == 9i32 {
                            mapHeader = Overworld_GetMapHeaderByGroupAndId(
                                (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                                    .wrapping_add(36))
                                .cast::<i8>())
                                .read()) as u16),
                                (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                                    .wrapping_add(36))
                                .wrapping_add(1)
                                .cast::<i8>())
                                .read()) as u16),
                            );
                            GetMapNameGeneric(
                                (&raw mut gStringVar1).cast::<u8>(),
                                ((((mapHeader).wrapping_add(20)).read()) as u16),
                            );
                            StringExpandPlaceholders(
                                (&raw mut gStringVar4).cast::<u8>(),
                                (&raw mut gText_EscapeFromHere).cast::<u8>(),
                            );
                            DisplayFieldMoveExitAreaMessage(taskId);
                            (((((&raw mut sPartyMenuInternal).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(536))
                            .cast::<i16>())
                            .write(((fieldMove) as i16));
                            break 'l1;
                        }
                        if __sw1 == 5i32 {
                            (((&raw mut gPartyMenu).cast::<u8>())
                                .cast::<Option<unsafe extern "C" fn()>>())
                            .write(Some(CB2_OpenFlyMap));
                            Task_ClosePartyMenu(taskId);
                            break 'l1;
                        }
                        if !__matched {
                            (((&raw mut gPartyMenu).cast::<u8>())
                                .cast::<Option<unsafe extern "C" fn()>>())
                            .write(Some(CB2_ReturnToField));
                            Task_ClosePartyMenu(taskId);
                            break 'l1;
                        }
                    }
                } else {
                    'l2: {
                        let __sw2 = ((fieldMove) as i32);
                        let __matched = __sw2 == 4i32 || __sw2 == 1i32;
                        if __sw2 == 4i32 {
                            DisplayCantUseSurfMessage();
                            break 'l2;
                        }
                        if __sw2 == 1i32 {
                            DisplayCantUseFlashMessage();
                            break 'l2;
                        }
                        if !__matched {
                            DisplayPartyMenuStdMessage(
                                (((((((&raw const sFieldMoveCursorCallbacks)
                                    .cast::<u8>()
                                    .cast_mut())
                                .cast::<u8>())
                                .wrapping_offset(((fieldMove) as i32) as isize * 8))
                                .wrapping_add(4))
                                .read()) as u32),
                            );
                            break 'l2;
                        }
                    }
                    ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .cast::<Option<unsafe extern "C" fn(u8)>>())
                    .write(Some(Task_CancelAfterAorBPress));
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn DisplayFieldMoveExitAreaMessage(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        DisplayPartyMenuMessage((&raw mut gStringVar4).cast::<u8>(), 1u8);
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Task_FieldMoveExitAreaYesNo));
    }
}
pub(crate) unsafe extern "C" fn Task_FieldMoveExitAreaYesNo(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if ((IsPartyMenuTextPrinterActive()) as i32) != 1i32 {
            PartyMenuDisplayYesNoMenu();
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_HandleFieldMoveExitAreaYesNoInput));
        }
    }
}
pub(crate) unsafe extern "C" fn Task_HandleFieldMoveExitAreaYesNoInput(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        'l1: {
            let __sw1 = ((Menu_ProcessInputNoWrapClearOnChoose()) as i32);
            let mut __fall = false;
            if __sw1 == 0i32 {
                __fall = true;
                (((&raw mut gPartyMenu).cast::<u8>()).cast::<Option<unsafe extern "C" fn()>>())
                    .write(Some(CB2_ReturnToField));
                Task_ClosePartyMenu(taskId);
                break 'l1;
            }
            if __sw1 == (-1i32) {
                __fall = true;
                PlaySE(5u16);
            }
            if __fall || __sw1 == 1i32 {
                __fall = true;
                ((&raw mut gFieldCallback2).cast::<Option<unsafe extern "C" fn() -> u8>>())
                    .write(None);
                ((&raw mut gPostMenuFieldCallback)
                    .cast::<u8>()
                    .cast::<Option<unsafe extern "C" fn()>>())
                .write(None);
                Task_ReturnToChooseMonAfterText(taskId);
                break 'l1;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FieldCallback_PrepareFadeInFromMenu() -> u8 {
    unsafe {
        FadeInFromBlack();
        CreateTask(Some(Task_FieldMoveWaitForFade), 8u8);
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn Task_FieldMoveWaitForFade(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if ((IsWeatherNotFadingIn()) as i32) == 1i32 {
            (((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>())
                .write(((GetFieldMoveMonSpecies()) as i32));
            (((&raw mut gPostMenuFieldCallback)
                .cast::<u8>()
                .cast::<Option<unsafe extern "C" fn()>>())
            .read())
            .unwrap_unchecked()();
            DestroyTask(taskId);
        }
    }
}
pub(crate) unsafe extern "C" fn GetFieldMoveMonSpecies() -> u16 {
    unsafe {
        return ((GetMonData2(
            ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                (((((&raw mut gPartyMenu).cast::<u8>())
                    .wrapping_add(9)
                    .cast::<i8>())
                .read()) as i32) as isize
                    * 100,
            ),
            11i32,
        )) as u16);
    }
}
pub(crate) unsafe extern "C" fn Task_CancelAfterAorBPress(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
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
            CursorCb_Cancel1(taskId);
        }
    }
}
pub(crate) unsafe extern "C" fn DisplayCantUseFlashMessage() {
    unsafe {
        if ((FlagGet(2184u16)) as i32) == 1i32 {
            DisplayPartyMenuStdMessage(12u32);
        } else {
            DisplayPartyMenuStdMessage(13u32);
        }
    }
}
pub(crate) unsafe extern "C" fn FieldCallback_Surf() {
    unsafe {
        (((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>())
            .write(((GetCursorSelectionMonId()) as i32));
        FieldEffectStart(9u8);
    }
}
pub(crate) unsafe extern "C" fn SetUpFieldMove_Surf() -> u8 {
    unsafe {
        if (((PartyHasMonWithSurf()) as i32) == 1i32)
            && (((IsPlayerFacingSurfableFishableWater()) as i32) == 1i32)
        {
            ((&raw mut gFieldCallback2).cast::<Option<unsafe extern "C" fn() -> u8>>())
                .write(Some(FieldCallback_PrepareFadeInFromMenu));
            ((&raw mut gPostMenuFieldCallback)
                .cast::<u8>()
                .cast::<Option<unsafe extern "C" fn()>>())
            .write(Some(FieldCallback_Surf));
            return 1u8;
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn DisplayCantUseSurfMessage() {
    unsafe {
        if (TestPlayerAvatarFlags(8u8)) != 0 {
            DisplayPartyMenuStdMessage(9u32);
        } else {
            DisplayPartyMenuStdMessage(8u32);
        }
    }
}
pub(crate) unsafe extern "C" fn SetUpFieldMove_Fly() -> u8 {
    unsafe {
        if ((Overworld_MapTypeAllowsTeleportAndFly(
            (((&raw mut gMapHeader).cast::<u8>()).wrapping_add(23)).read(),
        )) as i32)
            == 1i32
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CB2_ReturnToPartyMenuFromFlyMap() {
    unsafe {
        InitPartyMenu(
            0u8,
            0u8,
            0u8,
            1u8,
            0u8,
            Some(Task_HandleChooseMonInput),
            Some(CB2_ReturnToFieldWithOpenMenu),
        );
    }
}
pub(crate) unsafe extern "C" fn FieldCallback_Waterfall() {
    unsafe {
        (((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>())
            .write(((GetCursorSelectionMonId()) as i32));
        FieldEffectStart(43u8);
    }
}
pub(crate) unsafe extern "C" fn SetUpFieldMove_Waterfall() -> u8 {
    unsafe {
        let mut x: i16 = 0i16;
        let mut y: i16 = 0i16;
        GetXYCoordsOneStepInFrontOfPlayer(&raw mut x, &raw mut y);
        if (((MetatileBehavior_IsWaterfall(
            ((MapGridGetMetatileBehaviorAt(((x) as i32), ((y) as i32))) as u8),
        )) as i32)
            == 1i32)
            && (((IsPlayerSurfingNorth()) as i32) == 1i32)
        {
            ((&raw mut gFieldCallback2).cast::<Option<unsafe extern "C" fn() -> u8>>())
                .write(Some(FieldCallback_PrepareFadeInFromMenu));
            ((&raw mut gPostMenuFieldCallback)
                .cast::<u8>()
                .cast::<Option<unsafe extern "C" fn()>>())
            .write(Some(FieldCallback_Waterfall));
            return 1u8;
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn FieldCallback_Dive() {
    unsafe {
        (((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>())
            .write(((GetCursorSelectionMonId()) as i32));
        FieldEffectStart(44u8);
    }
}
pub(crate) unsafe extern "C" fn SetUpFieldMove_Dive() -> u8 {
    unsafe {
        ((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>()).wrapping_offset(1))
            .write(((TrySetDiveWarp()) as i32));
        if ((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>()).wrapping_offset(1))
            .read()
            != 0i32
        {
            ((&raw mut gFieldCallback2).cast::<Option<unsafe extern "C" fn() -> u8>>())
                .write(Some(FieldCallback_PrepareFadeInFromMenu));
            ((&raw mut gPostMenuFieldCallback)
                .cast::<u8>()
                .cast::<Option<unsafe extern "C" fn()>>())
            .write(Some(FieldCallback_Dive));
            return 1u8;
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn CreatePartyMonIconSprite(
    mon: *mut u8,
    menuBox: *mut u8,
    slot: u32,
) {
    unsafe {
        let mut mon = mon;
        let mut menuBox = menuBox;
        let mut slot = slot;
        let mut handleDeoxys: u32 = 1u32;
        let mut species2: u16 = 0u16;
        if (((IsMultiBattle()) as i32) == 1i32)
            && ((crate::c::bf_read(
                ((&raw mut gMain).cast::<u8>()).wrapping_add(1081),
                1,
                1,
                false,
            ) as u8)
                != 0)
        {
            handleDeoxys = ((if (((((((&raw const sMultiBattlePartnersPartyMask)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>())
            .wrapping_offset(((slot) as i32) as isize))
            .read()) as u32)
                ^ handleDeoxys)
                != 0
            {
                1i32
            } else {
                0i32
            }) as u32);
        }
        species2 = ((GetMonData2(mon, 65i32)) as u16);
        CreatePartyMonIconSpriteParameterized(
            species2,
            GetMonData2(mon, 0i32),
            menuBox,
            1u8,
            handleDeoxys,
        );
        UpdatePartyMonHPBar(((menuBox).wrapping_add(9)).read(), mon);
    }
}
pub(crate) unsafe extern "C" fn CreatePartyMonIconSpriteParameterized(
    species: u16,
    pid: u32,
    menuBox: *mut u8,
    priority: u8,
    handleDeoxys: u32,
) {
    unsafe {
        let mut species = species;
        let mut pid = pid;
        let mut menuBox = menuBox;
        let mut priority = priority;
        let mut handleDeoxys = handleDeoxys;
        if ((species) as i32) != 0i32 {
            ((menuBox).wrapping_add(9)).write(CreateMonIcon(
                species,
                Some(SpriteCB_MonIcon),
                (((((menuBox).wrapping_add(4).cast::<*mut u8>()).read()).read()) as i16),
                ((((((menuBox).wrapping_add(4).cast::<*mut u8>()).read()).wrapping_offset(1))
                    .read()) as i16),
                4u8,
                pid,
                handleDeoxys,
            ));
            crate::c::bf_write(
                (((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((((menuBox).wrapping_add(9)).read()) as i32) as isize * 68))
                .wrapping_add(5),
                2,
                2,
                ((priority) as u16) as i32,
            );
        }
    }
}
pub(crate) unsafe extern "C" fn UpdateHPBar(spriteId: u8, hp: u16, maxhp: u16) {
    unsafe {
        let mut spriteId = spriteId;
        let mut hp = hp;
        let mut maxhp = maxhp;
        'l1: {
            let __sw1 = ((GetHPBarLevel(((hp) as i16), ((maxhp) as i16))) as i32);
            let __matched = __sw1 == 4i32 || __sw1 == 3i32 || __sw1 == 2i32 || __sw1 == 1i32;
            if __sw1 == 4i32 {
                SetPartyHPBarSprite(
                    ((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68),
                    0u8,
                );
                break 'l1;
            }
            if __sw1 == 3i32 {
                SetPartyHPBarSprite(
                    ((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68),
                    1u8,
                );
                break 'l1;
            }
            if __sw1 == 2i32 {
                SetPartyHPBarSprite(
                    ((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68),
                    2u8,
                );
                break 'l1;
            }
            if __sw1 == 1i32 {
                SetPartyHPBarSprite(
                    ((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68),
                    3u8,
                );
                break 'l1;
            }
            if !__matched {
                SetPartyHPBarSprite(
                    ((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68),
                    4u8,
                );
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn UpdatePartyMonHPBar(spriteId: u8, mon: *mut u8) {
    unsafe {
        let mut spriteId = spriteId;
        let mut mon = mon;
        UpdateHPBar(
            spriteId,
            ((GetMonData2(mon, 57i32)) as u16),
            ((GetMonData2(mon, 58i32)) as u16),
        );
    }
}
pub(crate) unsafe extern "C" fn AnimateSelectedPartyIcon(spriteId: u8, animNum: u8) {
    unsafe {
        let mut spriteId = spriteId;
        let mut animNum = animNum;
        (((((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(46))
        .cast::<i16>())
        .write(0i16);
        if ((animNum) as i32) == 0i32 {
            if ((((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(32)
            .cast::<i16>())
            .read()) as i32)
                == 16i32
            {
                ((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(36)
                .cast::<i16>())
                .write(0i16);
                ((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(38)
                .cast::<i16>())
                .write((-4i16));
            } else {
                ((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(36)
                .cast::<i16>())
                .write((-4i16));
                ((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(38)
                .cast::<i16>())
                .write(0i16);
            }
            ((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(SpriteCB_UpdatePartyMonIcon));
        } else {
            ((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(36)
            .cast::<i16>())
            .write(0i16);
            ((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(38)
            .cast::<i16>())
            .write(0i16);
            ((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(SpriteCB_BouncePartyMonIcon));
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_BouncePartyMonIcon(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut animCmd: u8 = UpdateMonIconFrame(sprite);
        if ((animCmd) as i32) != 0i32 {
            if (((animCmd) as i32) & 1i32) != 0 {
                ((sprite).wrapping_add(38).cast::<i16>()).write((-3i16));
            } else {
                ((sprite).wrapping_add(38).cast::<i16>()).write(1i16);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_UpdatePartyMonIcon(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        UpdateMonIconFrame(sprite);
    }
}
pub(crate) unsafe extern "C" fn CreatePartyMonHeldItemSprite(mon: *mut u8, menuBox: *mut u8) {
    unsafe {
        let mut mon = mon;
        let mut menuBox = menuBox;
        if GetMonData2(mon, 11i32) != 0u32 {
            ((menuBox).wrapping_add(10)).write(CreateSprite(
                (&raw const sSpriteTemplate_HeldItem)
                    .cast::<u8>()
                    .cast_mut(),
                ((((((menuBox).wrapping_add(4).cast::<*mut u8>()).read()).wrapping_offset(2))
                    .read()) as i16),
                ((((((menuBox).wrapping_add(4).cast::<*mut u8>()).read()).wrapping_offset(3))
                    .read()) as i16),
                0u8,
            ));
            UpdatePartyMonHeldItemSprite(mon, menuBox);
        }
    }
}
pub(crate) unsafe extern "C" fn CreatePartyMonHeldItemSpriteParameterized(
    species: u16,
    item: u16,
    menuBox: *mut u8,
) {
    unsafe {
        let mut species = species;
        let mut item = item;
        let mut menuBox = menuBox;
        if ((species) as i32) != 0i32 {
            ((menuBox).wrapping_add(10)).write(CreateSprite(
                (&raw const sSpriteTemplate_HeldItem)
                    .cast::<u8>()
                    .cast_mut(),
                ((((((menuBox).wrapping_add(4).cast::<*mut u8>()).read()).wrapping_offset(2))
                    .read()) as i16),
                ((((((menuBox).wrapping_add(4).cast::<*mut u8>()).read()).wrapping_offset(3))
                    .read()) as i16),
                0u8,
            ));
            crate::c::bf_write(
                (((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((((menuBox).wrapping_add(10)).read()) as i32) as isize * 68))
                .wrapping_add(5),
                2,
                2,
                (0u16) as i32,
            );
            ShowOrHideHeldItemSprite(item, menuBox);
        }
    }
}
pub(crate) unsafe extern "C" fn UpdatePartyMonHeldItemSprite(mon: *mut u8, menuBox: *mut u8) {
    unsafe {
        let mut mon = mon;
        let mut menuBox = menuBox;
        ShowOrHideHeldItemSprite(((GetMonData2(mon, 12i32)) as u16), menuBox);
    }
}
pub(crate) unsafe extern "C" fn ShowOrHideHeldItemSprite(item: u16, menuBox: *mut u8) {
    unsafe {
        let mut item = item;
        let mut menuBox = menuBox;
        if ((item) as i32) == 0i32 {
            crate::c::bf_write(
                (((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((((menuBox).wrapping_add(10)).read()) as i32) as isize * 68))
                .wrapping_add(62),
                2,
                1,
                (1u16) as i32,
            );
        } else {
            if (ItemIsMail(item)) != 0 {
                StartSpriteAnim(
                    ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((menuBox).wrapping_add(10)).read()) as i32) as isize * 68,
                    ),
                    1u8,
                );
            } else {
                StartSpriteAnim(
                    ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((menuBox).wrapping_add(10)).read()) as i32) as isize * 68,
                    ),
                    0u8,
                );
            }
            crate::c::bf_write(
                (((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((((menuBox).wrapping_add(10)).read()) as i32) as isize * 68))
                .wrapping_add(62),
                2,
                1,
                (0u16) as i32,
            );
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn LoadHeldItemIcons() {
    unsafe {
        LoadSpriteSheet((&raw const sSpriteSheet_HeldItem).cast::<u8>().cast_mut());
        LoadSpritePalette((&raw const sSpritePalette_HeldItem).cast::<u8>().cast_mut());
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DrawHeldItemIconsForTrade(
    partyCounts: *mut u8,
    partySpriteIds: *mut u8,
    whichParty: u8,
) {
    unsafe {
        let mut partyCounts = partyCounts;
        let mut partySpriteIds = partySpriteIds;
        let mut whichParty = whichParty;
        let mut i: u16 = 0u16;
        let mut item: u16 = 0u16;
        'l1: {
            let __sw1 = ((whichParty) as i32);
            if __sw1 == 0i32 {
                {
                    i = 0u16;
                    'l2: loop {
                        if !(((i) as i32) < (((partyCounts).read()) as i32)) {
                            break 'l2;
                        }
                        'l3: {
                            item = ((GetMonData2(
                                ((&raw mut gPlayerParty).cast::<u8>())
                                    .wrapping_offset(((i) as i32) as isize * 100),
                                12i32,
                            )) as u16);
                            if ((item) as i32) != 0i32 {
                                CreateHeldItemSpriteForTrade(
                                    ((partySpriteIds).wrapping_offset(((i) as i32) as isize))
                                        .read(),
                                    ItemIsMail(item),
                                );
                            }
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
                        if !(((i) as i32) < ((((partyCounts).wrapping_offset(1)).read()) as i32)) {
                            break 'l4;
                        }
                        'l5: {
                            item = ((GetMonData2(
                                ((&raw mut gEnemyParty).cast::<u8>())
                                    .wrapping_offset(((i) as i32) as isize * 100),
                                12i32,
                            )) as u16);
                            if ((item) as i32) != 0i32 {
                                CreateHeldItemSpriteForTrade(
                                    ((partySpriteIds).wrapping_offset(
                                        (((i) as i32).wrapping_add(6i32)) as isize,
                                    ))
                                    .read(),
                                    ItemIsMail(item),
                                );
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
pub(crate) unsafe extern "C" fn CreateHeldItemSpriteForTrade(spriteId: u8, isMail: u8) {
    unsafe {
        let mut spriteId = spriteId;
        let mut isMail = isMail;
        let mut subpriority: u8 = ((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((spriteId) as i32) as isize * 68))
        .wrapping_add(67))
        .read();
        let mut newSpriteId: u8 = CreateSprite(
            (&raw const sSpriteTemplate_HeldItem)
                .cast::<u8>()
                .cast_mut(),
            250i16,
            170i16,
            ((((subpriority) as i32).wrapping_sub(1i32)) as u8),
        );
        ((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((newSpriteId) as i32) as isize * 68))
        .wrapping_add(36)
        .cast::<i16>())
        .write(4i16);
        ((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((newSpriteId) as i32) as isize * 68))
        .wrapping_add(38)
        .cast::<i16>())
        .write(10i16);
        ((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((newSpriteId) as i32) as isize * 68))
        .wrapping_add(28)
        .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(SpriteCB_HeldItem));
        ((((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((newSpriteId) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(7))
        .write(((spriteId) as i16));
        StartSpriteAnim(
            ((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((newSpriteId) as i32) as isize * 68),
            isMail,
        );
        (((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((newSpriteId) as i32) as isize * 68))
        .wrapping_add(28)
        .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .read())
        .unwrap_unchecked()(
            ((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((newSpriteId) as i32) as isize * 68),
        );
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_HeldItem(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut otherSpriteId: u8 =
            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read()) as u8);
        if (crate::c::bf_read(
            (((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((otherSpriteId) as i32) as isize * 68))
            .wrapping_add(62),
            2,
            1,
            false,
        ) as u16)
            != 0
        {
            crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (1u16) as i32);
        } else {
            crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (0u16) as i32);
            ((sprite).wrapping_add(32).cast::<i16>()).write(
                ((((((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((otherSpriteId) as i32) as isize * 68))
                .wrapping_add(32)
                .cast::<i16>())
                .read()) as i32)
                    .wrapping_add(
                        ((((((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((otherSpriteId) as i32) as isize * 68))
                        .wrapping_add(36)
                        .cast::<i16>())
                        .read()) as i32),
                    )) as i16),
            );
            ((sprite).wrapping_add(34).cast::<i16>()).write(
                ((((((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((otherSpriteId) as i32) as isize * 68))
                .wrapping_add(34)
                .cast::<i16>())
                .read()) as i32)
                    .wrapping_add(
                        ((((((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((otherSpriteId) as i32) as isize * 68))
                        .wrapping_add(38)
                        .cast::<i16>())
                        .read()) as i32),
                    )) as i16),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn CreatePartyMonPokeballSprite(mon: *mut u8, menuBox: *mut u8) {
    unsafe {
        let mut mon = mon;
        let mut menuBox = menuBox;
        if GetMonData2(mon, 11i32) != 0u32 {
            ((menuBox).wrapping_add(11)).write(CreateSprite(
                (&raw const sSpriteTemplate_MenuPokeball)
                    .cast::<u8>()
                    .cast_mut(),
                ((((((menuBox).wrapping_add(4).cast::<*mut u8>()).read()).wrapping_offset(6))
                    .read()) as i16),
                ((((((menuBox).wrapping_add(4).cast::<*mut u8>()).read()).wrapping_offset(7))
                    .read()) as i16),
                8u8,
            ));
        }
    }
}
pub(crate) unsafe extern "C" fn CreatePartyMonPokeballSpriteParameterized(
    species: u16,
    menuBox: *mut u8,
) {
    unsafe {
        let mut species = species;
        let mut menuBox = menuBox;
        if ((species) as i32) != 0i32 {
            ((menuBox).wrapping_add(11)).write(CreateSprite(
                (&raw const sSpriteTemplate_MenuPokeball)
                    .cast::<u8>()
                    .cast_mut(),
                ((((((menuBox).wrapping_add(4).cast::<*mut u8>()).read()).wrapping_offset(6))
                    .read()) as i16),
                ((((((menuBox).wrapping_add(4).cast::<*mut u8>()).read()).wrapping_offset(7))
                    .read()) as i16),
                8u8,
            ));
            crate::c::bf_write(
                (((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((((menuBox).wrapping_add(11)).read()) as i32) as isize * 68))
                .wrapping_add(5),
                2,
                2,
                (0u16) as i32,
            );
        }
    }
}
pub(crate) unsafe extern "C" fn CreatePokeballButtonSprite(x: u8, y: u8) -> u8 {
    unsafe {
        let mut x = x;
        let mut y = y;
        let mut spriteId: u8 = CreateSprite(
            (&raw const sSpriteTemplate_MenuPokeball)
                .cast::<u8>()
                .cast_mut(),
            ((x) as i16),
            ((y) as i16),
            8u8,
        );
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(5),
            2,
            2,
            (2u16) as i32,
        );
        return spriteId;
    }
}
pub(crate) unsafe extern "C" fn CreateSmallPokeballButtonSprite(x: u8, y: u8) -> u8 {
    unsafe {
        let mut x = x;
        let mut y = y;
        return CreateSprite(
            (&raw const sSpriteTemplate_MenuPokeballSmall)
                .cast::<u8>()
                .cast_mut(),
            ((x) as i16),
            ((y) as i16),
            8u8,
        );
    }
}
pub(crate) unsafe extern "C" fn PartyMenuStartSpriteAnim(spriteId: u8, animNum: u8) {
    unsafe {
        let mut spriteId = spriteId;
        let mut animNum = animNum;
        StartSpriteAnim(
            ((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68),
            animNum,
        );
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_BounceConfirmCancelButton(
    spriteId: u8,
    spriteId2: u8,
    animNum: u8,
) {
    unsafe {
        let mut spriteId = spriteId;
        let mut spriteId2 = spriteId2;
        let mut animNum = animNum;
        if ((animNum) as i32) == 0i32 {
            StartSpriteAnim(
                ((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68),
                2u8,
            );
            StartSpriteAnim(
                ((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId2) as i32) as isize * 68),
                4u8,
            );
            ((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(38)
            .cast::<i16>())
            .write(0i16);
            ((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId2) as i32) as isize * 68))
            .wrapping_add(38)
            .cast::<i16>())
            .write(0i16);
        } else {
            StartSpriteAnim(
                ((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68),
                3u8,
            );
            StartSpriteAnim(
                ((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId2) as i32) as isize * 68),
                5u8,
            );
            ((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(38)
            .cast::<i16>())
            .write((-4i16));
            ((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId2) as i32) as isize * 68))
            .wrapping_add(38)
            .cast::<i16>())
            .write(4i16);
        }
    }
}
pub(crate) unsafe extern "C" fn LoadPartyMenuPokeballGfx() {
    unsafe {
        LoadCompressedSpriteSheet(
            (&raw const sSpriteSheet_MenuPokeball)
                .cast::<u8>()
                .cast_mut(),
        );
        LoadCompressedSpriteSheet(
            (&raw const sSpriteSheet_MenuPokeballSmall)
                .cast::<u8>()
                .cast_mut(),
        );
        LoadCompressedSpritePalette(
            (&raw const sSpritePalette_MenuPokeball)
                .cast::<u8>()
                .cast_mut(),
        );
    }
}
pub(crate) unsafe extern "C" fn CreatePartyMonStatusSprite(mon: *mut u8, menuBox: *mut u8) {
    unsafe {
        let mut mon = mon;
        let mut menuBox = menuBox;
        if GetMonData2(mon, 11i32) != 0u32 {
            ((menuBox).wrapping_add(12)).write(CreateSprite(
                (&raw const sSpriteTemplate_StatusIcons)
                    .cast::<u8>()
                    .cast_mut(),
                ((((((menuBox).wrapping_add(4).cast::<*mut u8>()).read()).wrapping_offset(4))
                    .read()) as i16),
                ((((((menuBox).wrapping_add(4).cast::<*mut u8>()).read()).wrapping_offset(5))
                    .read()) as i16),
                0u8,
            ));
            SetPartyMonAilmentGfx(mon, menuBox);
        }
    }
}
pub(crate) unsafe extern "C" fn CreatePartyMonStatusSpriteParameterized(
    species: u16,
    status: u8,
    menuBox: *mut u8,
) {
    unsafe {
        let mut species = species;
        let mut status = status;
        let mut menuBox = menuBox;
        if ((species) as i32) != 0i32 {
            ((menuBox).wrapping_add(12)).write(CreateSprite(
                (&raw const sSpriteTemplate_StatusIcons)
                    .cast::<u8>()
                    .cast_mut(),
                ((((((menuBox).wrapping_add(4).cast::<*mut u8>()).read()).wrapping_offset(4))
                    .read()) as i16),
                ((((((menuBox).wrapping_add(4).cast::<*mut u8>()).read()).wrapping_offset(5))
                    .read()) as i16),
                0u8,
            ));
            UpdatePartyMonAilmentGfx(status, menuBox);
            crate::c::bf_write(
                (((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((((menuBox).wrapping_add(12)).read()) as i32) as isize * 68))
                .wrapping_add(5),
                2,
                2,
                (0u16) as i32,
            );
        }
    }
}
pub(crate) unsafe extern "C" fn SetPartyMonAilmentGfx(mon: *mut u8, menuBox: *mut u8) {
    unsafe {
        let mut mon = mon;
        let mut menuBox = menuBox;
        UpdatePartyMonAilmentGfx(GetMonAilment(mon), menuBox);
    }
}
pub(crate) unsafe extern "C" fn UpdatePartyMonAilmentGfx(status: u8, menuBox: *mut u8) {
    unsafe {
        let mut status = status;
        let mut menuBox = menuBox;
        'l1: {
            let __sw1 = ((status) as i32);
            let __matched = __sw1 == 0i32 || __sw1 == 6i32;
            if __sw1 == 0i32 || __sw1 == 6i32 {
                crate::c::bf_write(
                    (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((menuBox).wrapping_add(12)).read()) as i32) as isize * 68,
                    ))
                    .wrapping_add(62),
                    2,
                    1,
                    (1u16) as i32,
                );
                break 'l1;
            }
            if !__matched {
                StartSpriteAnim(
                    ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((menuBox).wrapping_add(12)).read()) as i32) as isize * 68,
                    ),
                    ((((status) as i32).wrapping_sub(1i32)) as u8),
                );
                crate::c::bf_write(
                    (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((menuBox).wrapping_add(12)).read()) as i32) as isize * 68,
                    ))
                    .wrapping_add(62),
                    2,
                    1,
                    (0u16) as i32,
                );
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn LoadPartyMenuAilmentGfx() {
    unsafe {
        LoadCompressedSpriteSheet(
            (&raw const sSpriteSheet_StatusIcons)
                .cast::<u8>()
                .cast_mut(),
        );
        LoadCompressedSpritePalette(
            (&raw const sSpritePalette_StatusIcons)
                .cast::<u8>()
                .cast_mut(),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CB2_ShowPartyMenuForItemUse() {
    unsafe {
        let mut callback: Option<unsafe extern "C" fn()> = Some(CB2_ReturnToBagMenu);
        let mut partyLayout: u8 = 0u8;
        let mut menuType: u8 = 0u8;
        let mut i: u8 = 0u8;
        let mut msgId: u8 = 0u8;
        let mut task: Option<unsafe extern "C" fn(u8)> = None;
        if (crate::c::bf_read(
            ((&raw mut gMain).cast::<u8>()).wrapping_add(1081),
            1,
            1,
            false,
        ) as u8)
            != 0
        {
            menuType = 1u8;
            partyLayout = GetPartyLayoutFromBattleType();
        } else {
            menuType = 0u8;
            partyLayout = 0u8;
        }
        if ((GetItemEffectType(((&raw mut gSpecialVar_ItemId).cast::<u16>()).read())) as i32)
            == 10i32
        {
            (((&raw mut gPartyMenu).cast::<u8>())
                .wrapping_add(9)
                .cast::<i8>())
            .write(0i8);
            {
                i = 0u8;
                'l1: loop {
                    if !(((i) as i32) < 6i32) {
                        break 'l1;
                    }
                    'l2: {
                        if (GetMonData2(
                            ((&raw mut gPlayerParty).cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize * 100),
                            11i32,
                        ) != 0u32)
                            && (GetMonData2(
                                ((&raw mut gPlayerParty).cast::<u8>())
                                    .wrapping_offset(((i) as i32) as isize * 100),
                                57i32,
                            ) == 0u32)
                        {
                            (((&raw mut gPartyMenu).cast::<u8>())
                                .wrapping_add(9)
                                .cast::<i8>())
                            .write(((i) as i8));
                            break 'l1;
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
            task = Some(Task_SetSacredAshCB);
            msgId = 127u8;
        } else {
            if ((GetPocketByItemId(((&raw mut gSpecialVar_ItemId).cast::<u16>()).read())) as i32)
                == 3i32
            {
                msgId = 4u8;
            } else {
                msgId = 5u8;
            }
            task = Some(Task_HandleChooseMonInput);
        }
        InitPartyMenu(menuType, partyLayout, 3u8, 1u8, msgId, task, callback);
    }
}
pub(crate) unsafe extern "C" fn CB2_ReturnToBagMenu() {
    unsafe {
        if ((((CurrentBattlePyramidLocation()) as i32) != 0i32) as i32) == 0i32 {
            GoToBagMenu(12u8, 5u8, None);
        } else {
            GoToBattlePyramidBagMenu(
                4u8,
                (((&raw mut gPyramidBagMenuState).cast::<u8>())
                    .cast::<Option<unsafe extern "C" fn()>>())
                .read(),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn Task_SetSacredAshCB(taskId: u8) {
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
            if ((crate::c::bf_read(
                ((&raw mut gPartyMenu).cast::<u8>()).wrapping_add(8),
                0,
                4,
                false,
            ) as u8) as i32)
                == 1i32
            {
                ((((&raw mut sPartyMenuInternal).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(4)
                    .cast::<Option<unsafe extern "C" fn()>>())
                .write(Some(CB2_SetUpExitToBattleScreen));
            }
            (((&raw mut gItemUseCB)
                .cast::<u8>()
                .cast::<Option<unsafe extern "C" fn(u8, Option<unsafe extern "C" fn(u8)>)>>())
            .read())
            .unwrap_unchecked()(taskId, Some(Task_ClosePartyMenuAfterText));
        }
    }
}
pub(crate) unsafe extern "C" fn IsHPRecoveryItem(item: u16) -> u8 {
    unsafe {
        let mut item = item;
        let mut effect: *mut u8 = core::ptr::null_mut();
        if ((item) as i32) == 175i32 {
            effect = (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(12792))
                .wrapping_add(28))
            .cast::<u8>();
        } else {
            effect = ((((&raw mut gItemEffectTable).cast::<*mut u8>()).cast::<*mut u8>())
                .wrapping_offset((((item) as i32).wrapping_sub(13i32)) as isize))
            .read();
        }
        if (((((effect).wrapping_offset(4)).read()) as i32) & 4i32) != 0 {
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
pub(crate) unsafe extern "C" fn GetMedicineItemEffectMessage(item: u16) {
    unsafe {
        let mut item = item;
        'l1: {
            let __sw1 = ((GetItemEffectType(item)) as i32);
            let __matched = __sw1 == 3i32
                || __sw1 == 4i32
                || __sw1 == 5i32
                || __sw1 == 6i32
                || __sw1 == 7i32
                || __sw1 == 8i32
                || __sw1 == 9i32
                || __sw1 == 11i32
                || __sw1 == 13i32
                || __sw1 == 12i32
                || __sw1 == 17i32
                || __sw1 == 16i32
                || __sw1 == 14i32
                || __sw1 == 15i32
                || __sw1 == 19i32
                || __sw1 == 20i32
                || __sw1 == 21i32;
            if __sw1 == 3i32 {
                StringExpandPlaceholders(
                    (&raw mut gStringVar4).cast::<u8>(),
                    (&raw mut gText_PkmnCuredOfPoison).cast::<u8>(),
                );
                break 'l1;
            }
            if __sw1 == 4i32 {
                StringExpandPlaceholders(
                    (&raw mut gStringVar4).cast::<u8>(),
                    (&raw mut gText_PkmnWokeUp2).cast::<u8>(),
                );
                break 'l1;
            }
            if __sw1 == 5i32 {
                StringExpandPlaceholders(
                    (&raw mut gStringVar4).cast::<u8>(),
                    (&raw mut gText_PkmnBurnHealed).cast::<u8>(),
                );
                break 'l1;
            }
            if __sw1 == 6i32 {
                StringExpandPlaceholders(
                    (&raw mut gStringVar4).cast::<u8>(),
                    (&raw mut gText_PkmnThawedOut).cast::<u8>(),
                );
                break 'l1;
            }
            if __sw1 == 7i32 {
                StringExpandPlaceholders(
                    (&raw mut gStringVar4).cast::<u8>(),
                    (&raw mut gText_PkmnCuredOfParalysis).cast::<u8>(),
                );
                break 'l1;
            }
            if __sw1 == 8i32 {
                StringExpandPlaceholders(
                    (&raw mut gStringVar4).cast::<u8>(),
                    (&raw mut gText_PkmnSnappedOutOfConfusion).cast::<u8>(),
                );
                break 'l1;
            }
            if __sw1 == 9i32 {
                StringExpandPlaceholders(
                    (&raw mut gStringVar4).cast::<u8>(),
                    (&raw mut gText_PkmnGotOverInfatuation).cast::<u8>(),
                );
                break 'l1;
            }
            if __sw1 == 11i32 {
                StringExpandPlaceholders(
                    (&raw mut gStringVar4).cast::<u8>(),
                    (&raw mut gText_PkmnBecameHealthy).cast::<u8>(),
                );
                break 'l1;
            }
            if __sw1 == 13i32 {
                StringCopy(
                    (&raw mut gStringVar2).cast::<u8>(),
                    (&raw mut gText_HP3).cast::<u8>(),
                );
                StringExpandPlaceholders(
                    (&raw mut gStringVar4).cast::<u8>(),
                    (&raw mut gText_PkmnBaseVar2StatIncreased).cast::<u8>(),
                );
                break 'l1;
            }
            if __sw1 == 12i32 {
                StringCopy(
                    (&raw mut gStringVar2).cast::<u8>(),
                    (&raw mut gText_Attack3).cast::<u8>(),
                );
                StringExpandPlaceholders(
                    (&raw mut gStringVar4).cast::<u8>(),
                    (&raw mut gText_PkmnBaseVar2StatIncreased).cast::<u8>(),
                );
                break 'l1;
            }
            if __sw1 == 17i32 {
                StringCopy(
                    (&raw mut gStringVar2).cast::<u8>(),
                    (&raw mut gText_Defense3).cast::<u8>(),
                );
                StringExpandPlaceholders(
                    (&raw mut gStringVar4).cast::<u8>(),
                    (&raw mut gText_PkmnBaseVar2StatIncreased).cast::<u8>(),
                );
                break 'l1;
            }
            if __sw1 == 16i32 {
                StringCopy(
                    (&raw mut gStringVar2).cast::<u8>(),
                    (&raw mut gText_Speed2).cast::<u8>(),
                );
                StringExpandPlaceholders(
                    (&raw mut gStringVar4).cast::<u8>(),
                    (&raw mut gText_PkmnBaseVar2StatIncreased).cast::<u8>(),
                );
                break 'l1;
            }
            if __sw1 == 14i32 {
                StringCopy(
                    (&raw mut gStringVar2).cast::<u8>(),
                    (&raw mut gText_SpAtk3).cast::<u8>(),
                );
                StringExpandPlaceholders(
                    (&raw mut gStringVar4).cast::<u8>(),
                    (&raw mut gText_PkmnBaseVar2StatIncreased).cast::<u8>(),
                );
                break 'l1;
            }
            if __sw1 == 15i32 {
                StringCopy(
                    (&raw mut gStringVar2).cast::<u8>(),
                    (&raw mut gText_SpDef3).cast::<u8>(),
                );
                StringExpandPlaceholders(
                    (&raw mut gStringVar4).cast::<u8>(),
                    (&raw mut gText_PkmnBaseVar2StatIncreased).cast::<u8>(),
                );
                break 'l1;
            }
            if __sw1 == 19i32 || __sw1 == 20i32 {
                StringExpandPlaceholders(
                    (&raw mut gStringVar4).cast::<u8>(),
                    (&raw mut gText_MovesPPIncreased).cast::<u8>(),
                );
                break 'l1;
            }
            if __sw1 == 21i32 {
                StringExpandPlaceholders(
                    (&raw mut gStringVar4).cast::<u8>(),
                    (&raw mut gText_PPWasRestored).cast::<u8>(),
                );
                break 'l1;
            }
            if !__matched {
                StringExpandPlaceholders(
                    (&raw mut gStringVar4).cast::<u8>(),
                    (&raw mut gText_WontHaveEffect).cast::<u8>(),
                );
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn NotUsingHPEVItemOnShedinja(mon: *mut u8, item: u16) -> u8 {
    unsafe {
        let mut mon = mon;
        let mut item = item;
        if (((GetItemEffectType(item)) as i32) == 13i32) && (GetMonData2(mon, 11i32) == 303u32) {
            return 0u8;
        }
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn IsItemFlute(item: u16) -> u8 {
    unsafe {
        let mut item = item;
        if ((((item) as i32) == 39i32) || (((item) as i32) == 41i32)) || (((item) as i32) == 40i32)
        {
            return 1u8;
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn ExecuteTableBasedItemEffect_(
    partyMonIndex: u8,
    item: u16,
    monMoveIndex: u8,
) -> u8 {
    unsafe {
        let mut partyMonIndex = partyMonIndex;
        let mut item = item;
        let mut monMoveIndex = monMoveIndex;
        if (crate::c::bf_read(
            ((&raw mut gMain).cast::<u8>()).wrapping_add(1081),
            1,
            1,
            false,
        ) as u8)
            != 0
        {
            return ExecuteTableBasedItemEffect(
                ((&raw mut gPlayerParty).cast::<u8>())
                    .wrapping_offset(((partyMonIndex) as i32) as isize * 100),
                item,
                GetPartyIdFromBattleSlot(partyMonIndex),
                monMoveIndex,
            );
        } else {
            return ExecuteTableBasedItemEffect(
                ((&raw mut gPlayerParty).cast::<u8>())
                    .wrapping_offset(((partyMonIndex) as i32) as isize * 100),
                item,
                partyMonIndex,
                monMoveIndex,
            );
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ItemUseCB_Medicine(taskId: u8, task: Option<unsafe extern "C" fn(u8)>) {
    unsafe {
        let mut taskId = taskId;
        let mut task = task;
        let mut hp: u16 = 0u16;
        let mut mon: *mut u8 = ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
            (((((&raw mut gPartyMenu).cast::<u8>())
                .wrapping_add(9)
                .cast::<i8>())
            .read()) as i32) as isize
                * 100,
        );
        let mut item: u16 = ((&raw mut gSpecialVar_ItemId).cast::<u16>()).read();
        let mut canHeal: u8 = 0u8;
        let mut cannotUse: u8 = 0u8;
        if ((NotUsingHPEVItemOnShedinja(mon, item)) as i32) == 0i32 {
            cannotUse = 1u8;
        } else {
            canHeal = IsHPRecoveryItem(item);
            if ((canHeal) as i32) == 1i32 {
                hp = ((GetMonData2(mon, 57i32)) as u16);
                if ((hp) as u32) == GetMonData2(mon, 58i32) {
                    canHeal = 0u8;
                }
            }
            cannotUse = ExecuteTableBasedItemEffect_(
                (((((&raw mut gPartyMenu).cast::<u8>())
                    .wrapping_add(9)
                    .cast::<i8>())
                .read()) as u8),
                item,
                0u8,
            );
        }
        if ((cannotUse) as i32) != 0i32 {
            ((&raw mut gPartyMenuUseExitCallback)
                .cast::<u8>()
                .cast::<u8>())
            .write(0u8);
            PlaySE(5u16);
            DisplayPartyMenuMessage((&raw mut gText_WontHaveEffect).cast::<u8>(), 1u8);
            ScheduleBgCopyTilemapToVram(2u8);
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(task);
        } else {
            ((&raw mut gPartyMenuUseExitCallback)
                .cast::<u8>()
                .cast::<u8>())
            .write(1u8);
            if !((IsItemFlute(item)) != 0) {
                PlaySE(1u16);
                if (((((&raw mut gPartyMenu).cast::<u8>()).wrapping_add(11)).read()) as i32)
                    != 14i32
                {
                    RemoveBagItem(item, 1u16);
                }
            } else {
                PlaySE(117u16);
            }
            SetPartyMonAilmentGfx(
                mon,
                (((&raw mut sPartyMenuBoxes).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_offset(
                        (((((&raw mut gPartyMenu).cast::<u8>())
                            .wrapping_add(9)
                            .cast::<i8>())
                        .read()) as i32) as isize
                            * 16,
                    ),
            );
            if (crate::c::bf_read(
                (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    (((((((&raw mut sPartyMenuBoxes).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(
                            (((((&raw mut gPartyMenu).cast::<u8>())
                                .wrapping_add(9)
                                .cast::<i8>())
                            .read()) as i32) as isize
                                * 16,
                        ))
                    .wrapping_add(12))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(62),
                2,
                1,
                false,
            ) as u16)
                != 0
            {
                DisplayPartyPokemonLevelCheck(
                    mon,
                    (((&raw mut sPartyMenuBoxes).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_offset(
                            (((((&raw mut gPartyMenu).cast::<u8>())
                                .wrapping_add(9)
                                .cast::<i8>())
                            .read()) as i32) as isize
                                * 16,
                        ),
                    1u8,
                );
            }
            if ((canHeal) as i32) == 1i32 {
                if ((hp) as i32) == 0i32 {
                    AnimatePartySlot(
                        (((((&raw mut gPartyMenu).cast::<u8>())
                            .wrapping_add(9)
                            .cast::<i8>())
                        .read()) as u8),
                        1u8,
                    );
                }
                PartyMenuModifyHP(
                    taskId,
                    (((((&raw mut gPartyMenu).cast::<u8>())
                        .wrapping_add(9)
                        .cast::<i8>())
                    .read()) as u8),
                    1i8,
                    (((GetMonData2(mon, 57i32)).wrapping_sub(((hp) as u32))) as i16),
                    Some(Task_DisplayHPRestoredMessage),
                );
                ResetHPTaskData(taskId, 0u8, ((hp) as u32));
                return;
            } else {
                GetMonNickname(mon, (&raw mut gStringVar1).cast::<u8>());
                GetMedicineItemEffectMessage(item);
                DisplayPartyMenuMessage((&raw mut gStringVar4).cast::<u8>(), 1u8);
                ScheduleBgCopyTilemapToVram(2u8);
                ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
                .write(task);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_DisplayHPRestoredMessage(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        GetMonNickname(
            ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                (((((&raw mut gPartyMenu).cast::<u8>())
                    .wrapping_add(9)
                    .cast::<i8>())
                .read()) as i32) as isize
                    * 100,
            ),
            (&raw mut gStringVar1).cast::<u8>(),
        );
        StringExpandPlaceholders(
            (&raw mut gStringVar4).cast::<u8>(),
            (&raw mut gText_PkmnHPRestoredByVar2).cast::<u8>(),
        );
        DisplayPartyMenuMessage((&raw mut gStringVar4).cast::<u8>(), 0u8);
        ScheduleBgCopyTilemapToVram(2u8);
        HandleBattleLowHpMusicChange();
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Task_ClosePartyMenuAfterText));
    }
}
pub(crate) unsafe extern "C" fn Task_ClosePartyMenuAfterText(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if ((IsPartyMenuTextPrinterActive()) as i32) != 1i32 {
            if ((((&raw mut gPartyMenuUseExitCallback)
                .cast::<u8>()
                .cast::<u8>())
            .read()) as i32)
                == 0i32
            {
                ((((&raw mut sPartyMenuInternal).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(4)
                    .cast::<Option<unsafe extern "C" fn()>>())
                .write(None);
            }
            Task_ClosePartyMenu(taskId);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ItemUseCB_ReduceEV(taskId: u8, task: Option<unsafe extern "C" fn(u8)>) {
    unsafe {
        let mut taskId = taskId;
        let mut task = task;
        let mut mon: *mut u8 = ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
            (((((&raw mut gPartyMenu).cast::<u8>())
                .wrapping_add(9)
                .cast::<i8>())
            .read()) as i32) as isize
                * 100,
        );
        let mut item: u16 = ((&raw mut gSpecialVar_ItemId).cast::<u16>()).read();
        let mut effectType: u8 = GetItemEffectType(item);
        let mut friendship: u16 = ((GetMonData2(mon, 32i32)) as u16);
        let mut ev: u16 = ItemEffectToMonEv(mon, effectType);
        let mut cannotUseEffect: u8 = ExecuteTableBasedItemEffect_(
            (((((&raw mut gPartyMenu).cast::<u8>())
                .wrapping_add(9)
                .cast::<i8>())
            .read()) as u8),
            item,
            0u8,
        );
        let mut newFriendship: u16 = ((GetMonData2(mon, 32i32)) as u16);
        let mut newEv: u16 = ItemEffectToMonEv(mon, effectType);
        if ((cannotUseEffect) != 0)
            || ((((friendship) as i32) == ((newFriendship) as i32))
                && (((ev) as i32) == ((newEv) as i32)))
        {
            ((&raw mut gPartyMenuUseExitCallback)
                .cast::<u8>()
                .cast::<u8>())
            .write(0u8);
            PlaySE(5u16);
            DisplayPartyMenuMessage((&raw mut gText_WontHaveEffect).cast::<u8>(), 1u8);
            ScheduleBgCopyTilemapToVram(2u8);
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(task);
        } else {
            ((&raw mut gPartyMenuUseExitCallback)
                .cast::<u8>()
                .cast::<u8>())
            .write(1u8);
            PlaySE(1u16);
            RemoveBagItem(item, 1u16);
            GetMonNickname(mon, (&raw mut gStringVar1).cast::<u8>());
            ItemEffectToStatString(effectType, (&raw mut gStringVar2).cast::<u8>());
            if ((friendship) as i32) != ((newFriendship) as i32) {
                if ((ev) as i32) != ((newEv) as i32) {
                    StringExpandPlaceholders(
                        (&raw mut gStringVar4).cast::<u8>(),
                        (&raw mut gText_PkmnFriendlyBaseVar2Fell).cast::<u8>(),
                    );
                } else {
                    StringExpandPlaceholders(
                        (&raw mut gStringVar4).cast::<u8>(),
                        (&raw mut gText_PkmnFriendlyBaseVar2CantFall).cast::<u8>(),
                    );
                }
            } else {
                StringExpandPlaceholders(
                    (&raw mut gStringVar4).cast::<u8>(),
                    (&raw mut gText_PkmnAdoresBaseVar2Fell).cast::<u8>(),
                );
            }
            DisplayPartyMenuMessage((&raw mut gStringVar4).cast::<u8>(), 1u8);
            ScheduleBgCopyTilemapToVram(2u8);
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(task);
        }
    }
}
pub(crate) unsafe extern "C" fn ItemEffectToMonEv(mon: *mut u8, effectType: u8) -> u16 {
    unsafe {
        let mut mon = mon;
        let mut effectType = effectType;
        'l1: {
            let __sw1 = ((effectType) as i32);
            if __sw1 == 13i32 {
                if GetMonData2(mon, 11i32) != 303u32 {
                    return ((GetMonData2(mon, 26i32)) as u16);
                }
                break 'l1;
            }
            if __sw1 == 12i32 {
                return ((GetMonData2(mon, 27i32)) as u16);
            }
            if __sw1 == 17i32 {
                return ((GetMonData2(mon, 28i32)) as u16);
            }
            if __sw1 == 16i32 {
                return ((GetMonData2(mon, 29i32)) as u16);
            }
            if __sw1 == 14i32 {
                return ((GetMonData2(mon, 30i32)) as u16);
            }
            if __sw1 == 15i32 {
                return ((GetMonData2(mon, 31i32)) as u16);
            }
        }
        return 0u16;
    }
}
pub(crate) unsafe extern "C" fn ItemEffectToStatString(effectType: u8, dest: *mut u8) {
    unsafe {
        let mut effectType = effectType;
        let mut dest = dest;
        'l1: {
            let __sw1 = ((effectType) as i32);
            if __sw1 == 13i32 {
                StringCopy(dest, (&raw mut gText_HP3).cast::<u8>());
                break 'l1;
            }
            if __sw1 == 12i32 {
                StringCopy(dest, (&raw mut gText_Attack3).cast::<u8>());
                break 'l1;
            }
            if __sw1 == 17i32 {
                StringCopy(dest, (&raw mut gText_Defense3).cast::<u8>());
                break 'l1;
            }
            if __sw1 == 16i32 {
                StringCopy(dest, (&raw mut gText_Speed2).cast::<u8>());
                break 'l1;
            }
            if __sw1 == 14i32 {
                StringCopy(dest, (&raw mut gText_SpAtk3).cast::<u8>());
                break 'l1;
            }
            if __sw1 == 15i32 {
                StringCopy(dest, (&raw mut gText_SpDef3).cast::<u8>());
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn ShowMoveSelectWindow(slot: u8) {
    unsafe {
        let mut slot = slot;
        let mut i: u8 = 0u8;
        let mut moveCount: u8 = 0u8;
        let mut fontId: u8 = 1u8;
        let mut windowId: u8 = DisplaySelectionWindow(3u8);
        let mut r#move: u16 = 0u16;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 4i32) {
                    break 'l1;
                }
                'l2: {
                    r#move = ((GetMonData2(
                        ((&raw mut gPlayerParty).cast::<u8>())
                            .wrapping_offset(((slot) as i32) as isize * 100),
                        (13i32).wrapping_add(((i) as i32)),
                    )) as u16);
                    AddTextPrinterParameterized(
                        windowId,
                        fontId,
                        (((&raw mut gMoveNames).cast::<u8>())
                            .wrapping_offset(((r#move) as i32) as isize * 13))
                        .cast::<u8>(),
                        8u8,
                        (((((i) as i32).wrapping_mul(16i32)).wrapping_add(1i32)) as u8),
                        255u8,
                        None,
                    );
                    if ((r#move) as i32) != 0i32 {
                        moveCount = (moveCount).wrapping_add(1);
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        InitMenuInUpperLeftCornerNormal(windowId, moveCount, 0u8);
        ScheduleBgCopyTilemapToVram(2u8);
    }
}
pub(crate) unsafe extern "C" fn Task_HandleWhichMoveInput(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut input: i8 = Menu_ProcessInput();
        if ((input) as i32) != (-2i32) {
            if ((input) as i32) == (-1i32) {
                PlaySE(5u16);
                ReturnToUseOnWhichMon(taskId);
            } else {
                PartyMenuRemoveWindow(
                    (((((&raw mut sPartyMenuInternal).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(12))
                    .cast::<u8>())
                    .wrapping_offset(1),
                );
                SetSelectedMoveForPPItem(taskId);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ItemUseCB_PPRecovery(taskId: u8, task: Option<unsafe extern "C" fn(u8)>) {
    unsafe {
        let mut taskId = taskId;
        let mut task = task;
        let mut effect: *mut u8 = core::ptr::null_mut();
        let mut item: u16 = ((&raw mut gSpecialVar_ItemId).cast::<u16>()).read();
        if ((item) as i32) == 175i32 {
            effect = (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(12792))
                .wrapping_add(28))
            .cast::<u8>();
        } else {
            effect = ((((&raw mut gItemEffectTable).cast::<*mut u8>()).cast::<*mut u8>())
                .wrapping_offset((((item) as i32).wrapping_sub(13i32)) as isize))
            .read();
        }
        if !((((((effect).wrapping_offset(4)).read()) as i32) & 16i32) != 0) {
            ((((&raw mut gPartyMenu).cast::<u8>()).wrapping_add(14)).cast::<i16>()).write(0i16);
            TryUsePPItem(taskId);
        } else {
            PlaySE(5u16);
            DisplayPartyMenuStdMessage(22u32);
            ShowMoveSelectWindow(
                (((((&raw mut gPartyMenu).cast::<u8>())
                    .wrapping_add(9)
                    .cast::<i8>())
                .read()) as u8),
            );
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_HandleWhichMoveInput));
        }
    }
}
pub(crate) unsafe extern "C" fn SetSelectedMoveForPPItem(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        PartyMenuRemoveWindow(
            ((((&raw mut sPartyMenuInternal).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(12))
            .cast::<u8>(),
        );
        ((((&raw mut gPartyMenu).cast::<u8>()).wrapping_add(14)).cast::<i16>())
            .write(((Menu_GetCursorPos()) as i16));
        TryUsePPItem(taskId);
    }
}
pub(crate) unsafe extern "C" fn ReturnToUseOnWhichMon(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Task_HandleChooseMonInput));
        ((((&raw mut sPartyMenuInternal).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(4)
            .cast::<Option<unsafe extern "C" fn()>>())
        .write(None);
        PartyMenuRemoveWindow(
            ((((&raw mut sPartyMenuInternal).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(12))
            .cast::<u8>(),
        );
        DisplayPartyMenuStdMessage(5u32);
    }
}
pub(crate) unsafe extern "C" fn TryUsePPItem(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut r#move: u16 = 0u16;
        let mut moveSlot: *mut i16 =
            (((&raw mut gPartyMenu).cast::<u8>()).wrapping_add(14)).cast::<i16>();
        let mut item: u16 = ((&raw mut gSpecialVar_ItemId).cast::<u16>()).read();
        let mut ptr: *mut u8 = (&raw mut gPartyMenu).cast::<u8>();
        let mut mon: *mut u8 = core::ptr::null_mut();
        if (ExecuteTableBasedItemEffect_(
            ((((ptr).wrapping_add(9).cast::<i8>()).read()) as u8),
            item,
            (((moveSlot).read()) as u8),
        )) != 0
        {
            ((&raw mut gPartyMenuUseExitCallback)
                .cast::<u8>()
                .cast::<u8>())
            .write(0u8);
            PlaySE(5u16);
            DisplayPartyMenuMessage((&raw mut gText_WontHaveEffect).cast::<u8>(), 1u8);
            ScheduleBgCopyTilemapToVram(2u8);
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_ClosePartyMenuAfterText));
        } else {
            ((&raw mut gPartyMenuUseExitCallback)
                .cast::<u8>()
                .cast::<u8>())
            .write(1u8);
            mon = ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                ((((ptr).wrapping_add(9).cast::<i8>()).read()) as i32) as isize * 100,
            );
            PlaySE(1u16);
            RemoveBagItem(item, 1u16);
            r#move =
                ((GetMonData2(mon, (13i32).wrapping_add((((moveSlot).read()) as i32)))) as u16);
            StringCopy(
                (&raw mut gStringVar1).cast::<u8>(),
                (((&raw mut gMoveNames).cast::<u8>())
                    .wrapping_offset(((r#move) as i32) as isize * 13))
                .cast::<u8>(),
            );
            GetMedicineItemEffectMessage(item);
            DisplayPartyMenuMessage((&raw mut gStringVar4).cast::<u8>(), 1u8);
            ScheduleBgCopyTilemapToVram(2u8);
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_ClosePartyMenuAfterText));
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ItemUseCB_PPUp(taskId: u8, task: Option<unsafe extern "C" fn(u8)>) {
    unsafe {
        let mut taskId = taskId;
        let mut task = task;
        PlaySE(5u16);
        DisplayPartyMenuStdMessage(23u32);
        ShowMoveSelectWindow(
            (((((&raw mut gPartyMenu).cast::<u8>())
                .wrapping_add(9)
                .cast::<i8>())
            .read()) as u8),
        );
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Task_HandleWhichMoveInput));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ItemIdToBattleMoveId(item: u16) -> u16 {
    unsafe {
        let mut item = item;
        let mut tmNumber: u16 = ((((item) as i32).wrapping_sub(289i32)) as u16);
        return ((((&raw const sTMHMMoves)
            .cast::<u8>()
            .cast_mut()
            .cast::<u16>())
        .cast::<u16>())
        .wrapping_offset(((tmNumber) as i32) as isize))
        .read();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsMoveHm(r#move: u16) -> u8 {
    unsafe {
        let mut r#move = r#move;
        let mut i: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 8i32) {
                    break 'l1;
                }
                'l2: {
                    if ((((((&raw const sTMHMMoves)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<u16>())
                    .cast::<u16>())
                    .wrapping_offset((((i) as i32).wrapping_add(50i32)) as isize))
                    .read()) as i32)
                        == ((r#move) as i32)
                    {
                        return 1u8;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MonKnowsMove(mon: *mut u8, r#move: u16) -> u8 {
    unsafe {
        let mut mon = mon;
        let mut r#move = r#move;
        let mut i: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 4i32) {
                    break 'l1;
                }
                'l2: {
                    if GetMonData2(mon, (13i32).wrapping_add(((i) as i32))) == ((r#move) as u32) {
                        return 1u8;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn DisplayLearnMoveMessage(str: *mut u8) {
    unsafe {
        let mut str = str;
        StringExpandPlaceholders((&raw mut gStringVar4).cast::<u8>(), str);
        DisplayPartyMenuMessage((&raw mut gStringVar4).cast::<u8>(), 1u8);
        ScheduleBgCopyTilemapToVram(2u8);
    }
}
pub(crate) unsafe extern "C" fn DisplayLearnMoveMessageAndClose(taskId: u8, str: *mut u8) {
    unsafe {
        let mut taskId = taskId;
        let mut str = str;
        DisplayLearnMoveMessage(str);
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Task_ClosePartyMenuAfterText));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ItemUseCB_TMHM(taskId: u8, task: Option<unsafe extern "C" fn(u8)>) {
    unsafe {
        let mut taskId = taskId;
        let mut task = task;
        let mut mon: *mut u8 = core::ptr::null_mut();
        let mut r#move: *mut i16 = core::ptr::null_mut();
        let mut item: u16 = 0u16;
        PlaySE(5u16);
        mon = ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
            (((((&raw mut gPartyMenu).cast::<u8>())
                .wrapping_add(9)
                .cast::<i8>())
            .read()) as i32) as isize
                * 100,
        );
        r#move = (((&raw mut gPartyMenu).cast::<u8>()).wrapping_add(14)).cast::<i16>();
        item = ((&raw mut gSpecialVar_ItemId).cast::<u16>()).read();
        GetMonNickname(mon, (&raw mut gStringVar1).cast::<u8>());
        (r#move).write(((ItemIdToBattleMoveId(item)) as i16));
        StringCopy(
            (&raw mut gStringVar2).cast::<u8>(),
            (((&raw mut gMoveNames).cast::<u8>())
                .wrapping_offset((((r#move).read()) as i32) as isize * 13))
            .cast::<u8>(),
        );
        ((r#move).wrapping_offset(1)).write(0i16);
        'l1: {
            let __sw1 = ((CanMonLearnTMTutor(mon, item, 0u8)) as i32);
            if __sw1 == 1i32 {
                DisplayLearnMoveMessageAndClose(
                    taskId,
                    (&raw mut gText_PkmnCantLearnMove).cast::<u8>(),
                );
                return;
            }
            if __sw1 == 2i32 {
                DisplayLearnMoveMessageAndClose(
                    taskId,
                    (&raw mut gText_PkmnAlreadyKnows).cast::<u8>(),
                );
                return;
            }
        }
        if ((GiveMoveToMon(mon, (((r#move).read()) as u16))) as i32) != 65535i32 {
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_LearnedMove));
        } else {
            DisplayLearnMoveMessage((&raw mut gText_PkmnNeedsToReplaceMove).cast::<u8>());
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_ReplaceMoveYesNo));
        }
    }
}
pub(crate) unsafe extern "C" fn Task_LearnedMove(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut mon: *mut u8 = ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
            (((((&raw mut gPartyMenu).cast::<u8>())
                .wrapping_add(9)
                .cast::<i8>())
            .read()) as i32) as isize
                * 100,
        );
        let mut r#move: *mut i16 =
            (((&raw mut gPartyMenu).cast::<u8>()).wrapping_add(14)).cast::<i16>();
        let mut item: u16 = ((&raw mut gSpecialVar_ItemId).cast::<u16>()).read();
        if ((((r#move).wrapping_offset(1)).read()) as i32) == 0i32 {
            AdjustFriendship(mon, 4u8);
            if ((item) as i32) < 339i32 {
                RemoveBagItem(item, 1u16);
            }
        }
        GetMonNickname(mon, (&raw mut gStringVar1).cast::<u8>());
        StringCopy(
            (&raw mut gStringVar2).cast::<u8>(),
            (((&raw mut gMoveNames).cast::<u8>())
                .wrapping_offset((((r#move).read()) as i32) as isize * 13))
            .cast::<u8>(),
        );
        StringExpandPlaceholders(
            (&raw mut gStringVar4).cast::<u8>(),
            (&raw mut gText_PkmnLearnedMove3).cast::<u8>(),
        );
        DisplayPartyMenuMessage((&raw mut gStringVar4).cast::<u8>(), 1u8);
        ScheduleBgCopyTilemapToVram(2u8);
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Task_DoLearnedMoveFanfareAfterText));
    }
}
pub(crate) unsafe extern "C" fn Task_DoLearnedMoveFanfareAfterText(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if ((IsPartyMenuTextPrinterActive()) as i32) != 1i32 {
            PlayFanfare(367u16);
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_LearnNextMoveOrClosePartyMenu));
        }
    }
}
pub(crate) unsafe extern "C" fn Task_LearnNextMoveOrClosePartyMenu(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if ((IsFanfareTaskInactive()) != 0)
            && ((((((((&raw mut gMain).cast::<u8>())
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
                    != 0))
        {
            if (((((((&raw mut gPartyMenu).cast::<u8>()).wrapping_add(14)).cast::<i16>())
                .wrapping_offset(1))
            .read()) as i32)
                == 1i32
            {
                Task_TryLearningNextMove(taskId);
            } else {
                if (((((((&raw mut gPartyMenu).cast::<u8>()).wrapping_add(14)).cast::<i16>())
                    .wrapping_offset(1))
                .read()) as i32)
                    == 2i32
                {
                    ((&raw mut gSpecialVar_Result).cast::<u16>()).write(1u16);
                }
                Task_ClosePartyMenu(taskId);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_ReplaceMoveYesNo(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if ((IsPartyMenuTextPrinterActive()) as i32) != 1i32 {
            PartyMenuDisplayYesNoMenu();
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_HandleReplaceMoveYesNoInput));
        }
    }
}
pub(crate) unsafe extern "C" fn Task_HandleReplaceMoveYesNoInput(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        'l1: {
            let __sw1 = ((Menu_ProcessInputNoWrapClearOnChoose()) as i32);
            let mut __fall = false;
            if __sw1 == 0i32 {
                __fall = true;
                DisplayPartyMenuMessage((&raw mut gText_WhichMoveToForget).cast::<u8>(), 1u8);
                ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
                .write(Some(Task_ShowSummaryScreenToForgetMove));
                break 'l1;
            }
            if __sw1 == (-1i32) {
                __fall = true;
                PlaySE(5u16);
            }
            if __fall || __sw1 == 1i32 {
                __fall = true;
                StopLearningMovePrompt(taskId);
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_ShowSummaryScreenToForgetMove(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if ((IsPartyMenuTextPrinterActive()) as i32) != 1i32 {
            ((((&raw mut sPartyMenuInternal).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(4)
                .cast::<Option<unsafe extern "C" fn()>>())
            .write(Some(CB2_ShowSummaryScreenToForgetMove));
            Task_ClosePartyMenu(taskId);
        }
    }
}
pub(crate) unsafe extern "C" fn CB2_ShowSummaryScreenToForgetMove() {
    unsafe {
        ShowSelectMovePokemonSummaryScreen(
            (&raw mut gPlayerParty).cast::<u8>(),
            (((((&raw mut gPartyMenu).cast::<u8>())
                .wrapping_add(9)
                .cast::<i8>())
            .read()) as u8),
            ((((((&raw mut gPlayerPartyCount).cast::<u8>()).read()) as i32).wrapping_sub(1i32))
                as u8),
            Some(CB2_ReturnToPartyMenuWhileLearningMove),
            ((((((&raw mut gPartyMenu).cast::<u8>()).wrapping_add(14)).cast::<i16>()).read())
                as u16),
        );
    }
}
pub(crate) unsafe extern "C" fn CB2_ReturnToPartyMenuWhileLearningMove() {
    unsafe {
        InitPartyMenu(
            0u8,
            0u8,
            0u8,
            1u8,
            127u8,
            Some(Task_ReturnToPartyMenuWhileLearningMove),
            (((&raw mut gPartyMenu).cast::<u8>()).cast::<Option<unsafe extern "C" fn()>>()).read(),
        );
    }
}
pub(crate) unsafe extern "C" fn Task_ReturnToPartyMenuWhileLearningMove(taskId: u8) {
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
            if ((GetMoveSlotToReplace()) as i32) != 4i32 {
                DisplayPartyMenuForgotMoveMessage(taskId);
            } else {
                StopLearningMovePrompt(taskId);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn DisplayPartyMenuForgotMoveMessage(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut mon: *mut u8 = ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
            (((((&raw mut gPartyMenu).cast::<u8>())
                .wrapping_add(9)
                .cast::<i8>())
            .read()) as i32) as isize
                * 100,
        );
        let mut r#move: u16 =
            ((GetMonData2(mon, (13i32).wrapping_add(((GetMoveSlotToReplace()) as i32)))) as u16);
        GetMonNickname(mon, (&raw mut gStringVar1).cast::<u8>());
        StringCopy(
            (&raw mut gStringVar2).cast::<u8>(),
            (((&raw mut gMoveNames).cast::<u8>()).wrapping_offset(((r#move) as i32) as isize * 13))
                .cast::<u8>(),
        );
        DisplayLearnMoveMessage((&raw mut gText_12PoofForgotMove).cast::<u8>());
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Task_PartyMenuReplaceMove));
    }
}
pub(crate) unsafe extern "C" fn Task_PartyMenuReplaceMove(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut mon: *mut u8 = core::ptr::null_mut();
        let mut r#move: u16 = 0u16;
        if ((IsPartyMenuTextPrinterActive()) as i32) != 1i32 {
            mon = ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                (((((&raw mut gPartyMenu).cast::<u8>())
                    .wrapping_add(9)
                    .cast::<i8>())
                .read()) as i32) as isize
                    * 100,
            );
            RemoveMonPPBonus(mon, GetMoveSlotToReplace());
            r#move = ((((((&raw mut gPartyMenu).cast::<u8>()).wrapping_add(14)).cast::<i16>())
                .read()) as u16);
            SetMonMoveSlot(mon, r#move, GetMoveSlotToReplace());
            Task_LearnedMove(taskId);
        }
    }
}
pub(crate) unsafe extern "C" fn StopLearningMovePrompt(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        StringCopy(
            (&raw mut gStringVar2).cast::<u8>(),
            (((&raw mut gMoveNames).cast::<u8>()).wrapping_offset(
                ((((((&raw mut gPartyMenu).cast::<u8>()).wrapping_add(14)).cast::<i16>()).read())
                    as i32) as isize
                    * 13,
            ))
            .cast::<u8>(),
        );
        StringExpandPlaceholders(
            (&raw mut gStringVar4).cast::<u8>(),
            (&raw mut gText_StopLearningMove2).cast::<u8>(),
        );
        DisplayPartyMenuMessage((&raw mut gStringVar4).cast::<u8>(), 1u8);
        ScheduleBgCopyTilemapToVram(2u8);
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Task_StopLearningMoveYesNo));
    }
}
pub(crate) unsafe extern "C" fn Task_StopLearningMoveYesNo(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if ((IsPartyMenuTextPrinterActive()) as i32) != 1i32 {
            PartyMenuDisplayYesNoMenu();
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_HandleStopLearningMoveYesNoInput));
        }
    }
}
pub(crate) unsafe extern "C" fn Task_HandleStopLearningMoveYesNoInput(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut mon: *mut u8 = ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
            (((((&raw mut gPartyMenu).cast::<u8>())
                .wrapping_add(9)
                .cast::<i8>())
            .read()) as i32) as isize
                * 100,
        );
        'l1: {
            let __sw1 = ((Menu_ProcessInputNoWrapClearOnChoose()) as i32);
            let mut __fall = false;
            if __sw1 == 0i32 {
                __fall = true;
                GetMonNickname(mon, (&raw mut gStringVar1).cast::<u8>());
                StringCopy(
                    (&raw mut gStringVar2).cast::<u8>(),
                    (((&raw mut gMoveNames).cast::<u8>()).wrapping_offset(
                        ((((((&raw mut gPartyMenu).cast::<u8>()).wrapping_add(14)).cast::<i16>())
                            .read()) as i32) as isize
                            * 13,
                    ))
                    .cast::<u8>(),
                );
                StringExpandPlaceholders(
                    (&raw mut gStringVar4).cast::<u8>(),
                    (&raw mut gText_MoveNotLearned).cast::<u8>(),
                );
                DisplayPartyMenuMessage((&raw mut gStringVar4).cast::<u8>(), 1u8);
                if (((((((&raw mut gPartyMenu).cast::<u8>()).wrapping_add(14)).cast::<i16>())
                    .wrapping_offset(1))
                .read()) as i32)
                    == 1i32
                {
                    ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .cast::<Option<unsafe extern "C" fn(u8)>>())
                    .write(Some(Task_TryLearningNextMoveAfterText));
                } else {
                    if (((((((&raw mut gPartyMenu).cast::<u8>()).wrapping_add(14)).cast::<i16>())
                        .wrapping_offset(1))
                    .read()) as i32)
                        == 2i32
                    {
                        ((&raw mut gSpecialVar_Result).cast::<u16>()).write(0u16);
                    }
                    ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .cast::<Option<unsafe extern "C" fn(u8)>>())
                    .write(Some(Task_ClosePartyMenuAfterText));
                }
                break 'l1;
            }
            if __sw1 == (-1i32) {
                __fall = true;
                PlaySE(5u16);
            }
            if __fall || __sw1 == 1i32 {
                __fall = true;
                GetMonNickname(mon, (&raw mut gStringVar1).cast::<u8>());
                StringCopy(
                    (&raw mut gStringVar2).cast::<u8>(),
                    (((&raw mut gMoveNames).cast::<u8>()).wrapping_offset(
                        ((((((&raw mut gPartyMenu).cast::<u8>()).wrapping_add(14)).cast::<i16>())
                            .read()) as i32) as isize
                            * 13,
                    ))
                    .cast::<u8>(),
                );
                DisplayLearnMoveMessage((&raw mut gText_PkmnNeedsToReplaceMove).cast::<u8>());
                ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
                .write(Some(Task_ReplaceMoveYesNo));
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_TryLearningNextMoveAfterText(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if ((IsPartyMenuTextPrinterActive()) as i32) != 1i32 {
            Task_TryLearningNextMove(taskId);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ItemUseCB_RareCandy(taskId: u8, task: Option<unsafe extern "C" fn(u8)>) {
    unsafe {
        let mut taskId = taskId;
        let mut task = task;
        let mut mon: *mut u8 = ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
            (((((&raw mut gPartyMenu).cast::<u8>())
                .wrapping_add(9)
                .cast::<i8>())
            .read()) as i32) as isize
                * 100,
        );
        let mut ptr: *mut u8 =
            ((&raw mut sPartyMenuInternal).cast::<u8>().cast::<*mut u8>()).read();
        let mut arrayPtr: *mut i16 = ((ptr).wrapping_add(536)).cast::<i16>();
        let mut itemPtr: *mut u16 = (&raw mut gSpecialVar_ItemId).cast::<u16>();
        let mut cannotUseEffect: u8 = 0u8;
        if GetMonData2(mon, 56i32) != 100u32 {
            BufferMonStatsToTaskData(mon, arrayPtr);
            cannotUseEffect = ExecuteTableBasedItemEffect_(
                (((((&raw mut gPartyMenu).cast::<u8>())
                    .wrapping_add(9)
                    .cast::<i8>())
                .read()) as u8),
                (itemPtr).read(),
                0u8,
            );
            BufferMonStatsToTaskData(
                mon,
                (((ptr).wrapping_add(536)).cast::<i16>()).wrapping_offset(6),
            );
        } else {
            cannotUseEffect = 1u8;
        }
        PlaySE(5u16);
        if (cannotUseEffect) != 0 {
            ((&raw mut gPartyMenuUseExitCallback)
                .cast::<u8>()
                .cast::<u8>())
            .write(0u8);
            DisplayPartyMenuMessage((&raw mut gText_WontHaveEffect).cast::<u8>(), 1u8);
            ScheduleBgCopyTilemapToVram(2u8);
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(task);
        } else {
            ((&raw mut gPartyMenuUseExitCallback)
                .cast::<u8>()
                .cast::<u8>())
            .write(1u8);
            PlayFanfareByFanfareNum(0u8);
            UpdateMonDisplayInfoAfterRareCandy(
                (((((&raw mut gPartyMenu).cast::<u8>())
                    .wrapping_add(9)
                    .cast::<i8>())
                .read()) as u8),
                mon,
            );
            RemoveBagItem(((&raw mut gSpecialVar_ItemId).cast::<u16>()).read(), 1u16);
            GetMonNickname(mon, (&raw mut gStringVar1).cast::<u8>());
            ConvertIntToDecimalStringN(
                (&raw mut gStringVar2).cast::<u8>(),
                ((GetMonData2(mon, 56i32)) as i32),
                0i32,
                3u8,
            );
            StringExpandPlaceholders(
                (&raw mut gStringVar4).cast::<u8>(),
                (&raw mut gText_PkmnElevatedToLvVar2).cast::<u8>(),
            );
            DisplayPartyMenuMessage((&raw mut gStringVar4).cast::<u8>(), 1u8);
            ScheduleBgCopyTilemapToVram(2u8);
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_DisplayLevelUpStatsPg1));
        }
    }
}
pub(crate) unsafe extern "C" fn UpdateMonDisplayInfoAfterRareCandy(slot: u8, mon: *mut u8) {
    unsafe {
        let mut slot = slot;
        let mut mon = mon;
        SetPartyMonAilmentGfx(
            mon,
            (((&raw mut sPartyMenuBoxes).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_offset(((slot) as i32) as isize * 16),
        );
        if (crate::c::bf_read(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                (((((((&raw mut sPartyMenuBoxes).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_offset(((slot) as i32) as isize * 16))
                .wrapping_add(12))
                .read()) as i32) as isize
                    * 68,
            ))
            .wrapping_add(62),
            2,
            1,
            false,
        ) as u16)
            != 0
        {
            DisplayPartyPokemonLevelCheck(
                mon,
                (((&raw mut sPartyMenuBoxes).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_offset(((slot) as i32) as isize * 16),
                1u8,
            );
        }
        DisplayPartyPokemonHPCheck(
            mon,
            (((&raw mut sPartyMenuBoxes).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_offset(((slot) as i32) as isize * 16),
            1u8,
        );
        DisplayPartyPokemonMaxHPCheck(
            mon,
            (((&raw mut sPartyMenuBoxes).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_offset(((slot) as i32) as isize * 16),
            1u8,
        );
        DisplayPartyPokemonHPBarCheck(
            mon,
            (((&raw mut sPartyMenuBoxes).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_offset(((slot) as i32) as isize * 16),
        );
        UpdatePartyMonHPBar(
            (((((&raw mut sPartyMenuBoxes).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_offset(((slot) as i32) as isize * 16))
            .wrapping_add(9))
            .read(),
            mon,
        );
        AnimatePartySlot(slot, 1u8);
        ScheduleBgCopyTilemapToVram(0u8);
    }
}
pub(crate) unsafe extern "C" fn Task_DisplayLevelUpStatsPg1(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if (((WaitFanfare(0u8)) != 0) && (((IsPartyMenuTextPrinterActive()) as i32) != 1i32))
            && ((((((((&raw mut gMain).cast::<u8>())
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
                    != 0))
        {
            PlaySE(5u16);
            DisplayLevelUpStatsPg1(taskId);
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_DisplayLevelUpStatsPg2));
        }
    }
}
pub(crate) unsafe extern "C" fn Task_DisplayLevelUpStatsPg2(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
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
            DisplayLevelUpStatsPg2(taskId);
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_TryLearnNewMoves));
        }
    }
}
pub(crate) unsafe extern "C" fn DisplayLevelUpStatsPg1(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut arrayPtr: *mut i16 =
            ((((&raw mut sPartyMenuInternal).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(536))
            .cast::<i16>();
        ((arrayPtr).wrapping_offset(12)).write(((CreateLevelUpStatsWindow()) as i16));
        DrawLevelUpWindowPg1(
            ((((arrayPtr).wrapping_offset(12)).read()) as u16),
            (arrayPtr).cast::<u16>(),
            ((arrayPtr).wrapping_offset(6)).cast::<u16>(),
            1u8,
            2u8,
            3u8,
        );
        CopyWindowToVram(((((arrayPtr).wrapping_offset(12)).read()) as u8), 2u8);
        ScheduleBgCopyTilemapToVram(2u8);
    }
}
pub(crate) unsafe extern "C" fn DisplayLevelUpStatsPg2(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut arrayPtr: *mut i16 =
            ((((&raw mut sPartyMenuInternal).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(536))
            .cast::<i16>();
        DrawLevelUpWindowPg2(
            ((((arrayPtr).wrapping_offset(12)).read()) as u16),
            ((arrayPtr).wrapping_offset(6)).cast::<u16>(),
            1u8,
            2u8,
            3u8,
        );
        CopyWindowToVram(((((arrayPtr).wrapping_offset(12)).read()) as u8), 2u8);
        ScheduleBgCopyTilemapToVram(2u8);
    }
}
pub(crate) unsafe extern "C" fn Task_TryLearnNewMoves(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut learnMove: u16 = 0u16;
        if ((WaitFanfare(0u8)) != 0)
            && ((((((((&raw mut gMain).cast::<u8>())
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
                    != 0))
        {
            RemoveLevelUpStatsWindow();
            learnMove = MonTryLearningNewMove(
                ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                    (((((&raw mut gPartyMenu).cast::<u8>())
                        .wrapping_add(9)
                        .cast::<i8>())
                    .read()) as i32) as isize
                        * 100,
                ),
                1u8,
            );
            (((((&raw mut gPartyMenu).cast::<u8>()).wrapping_add(14)).cast::<i16>())
                .wrapping_offset(1))
            .write(1i16);
            'l1: {
                let __sw1 = ((learnMove) as i32);
                let __matched = __sw1 == 0i32 || __sw1 == 65535i32 || __sw1 == 65534i32;
                if __sw1 == 0i32 {
                    PartyMenuTryEvolution(taskId);
                    break 'l1;
                }
                if __sw1 == 65535i32 {
                    DisplayMonNeedsToReplaceMove(taskId);
                    break 'l1;
                }
                if __sw1 == 65534i32 {
                    ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .cast::<Option<unsafe extern "C" fn(u8)>>())
                    .write(Some(Task_TryLearningNextMove));
                    break 'l1;
                }
                if !__matched {
                    DisplayMonLearnedMove(taskId, learnMove);
                    break 'l1;
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_TryLearningNextMove(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut result: u16 = MonTryLearningNewMove(
            ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                (((((&raw mut gPartyMenu).cast::<u8>())
                    .wrapping_add(9)
                    .cast::<i8>())
                .read()) as i32) as isize
                    * 100,
            ),
            0u8,
        );
        'l1: {
            let __sw1 = ((result) as i32);
            let __matched = __sw1 == 0i32 || __sw1 == 65535i32 || __sw1 == 65534i32;
            if __sw1 == 0i32 {
                PartyMenuTryEvolution(taskId);
                break 'l1;
            }
            if __sw1 == 65535i32 {
                DisplayMonNeedsToReplaceMove(taskId);
                break 'l1;
            }
            if __sw1 == 65534i32 {
                return;
            }
            if !__matched {
                DisplayMonLearnedMove(taskId, result);
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn PartyMenuTryEvolution(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut mon: *mut u8 = ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
            (((((&raw mut gPartyMenu).cast::<u8>())
                .wrapping_add(9)
                .cast::<i8>())
            .read()) as i32) as isize
                * 100,
        );
        let mut targetSpecies: u16 = GetEvolutionTargetSpecies(mon, 0u8, 0u16);
        if ((targetSpecies) as i32) != 0i32 {
            FreePartyPointers();
            ((&raw mut gCB2_AfterEvolution).cast::<Option<unsafe extern "C" fn()>>()).write(
                (((&raw mut gPartyMenu).cast::<u8>()).cast::<Option<unsafe extern "C" fn()>>())
                    .read(),
            );
            BeginEvolutionScene(
                mon,
                targetSpecies,
                1u8,
                (((((&raw mut gPartyMenu).cast::<u8>())
                    .wrapping_add(9)
                    .cast::<i8>())
                .read()) as u8),
            );
            DestroyTask(taskId);
        } else {
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_ClosePartyMenuAfterText));
        }
    }
}
pub(crate) unsafe extern "C" fn DisplayMonNeedsToReplaceMove(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        GetMonNickname(
            ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                (((((&raw mut gPartyMenu).cast::<u8>())
                    .wrapping_add(9)
                    .cast::<i8>())
                .read()) as i32) as isize
                    * 100,
            ),
            (&raw mut gStringVar1).cast::<u8>(),
        );
        StringCopy(
            (&raw mut gStringVar2).cast::<u8>(),
            (((&raw mut gMoveNames).cast::<u8>()).wrapping_offset(
                ((((&raw mut gMoveToLearn).cast::<u16>()).read()) as i32) as isize * 13,
            ))
            .cast::<u8>(),
        );
        StringExpandPlaceholders(
            (&raw mut gStringVar4).cast::<u8>(),
            (&raw mut gText_PkmnNeedsToReplaceMove).cast::<u8>(),
        );
        DisplayPartyMenuMessage((&raw mut gStringVar4).cast::<u8>(), 1u8);
        ScheduleBgCopyTilemapToVram(2u8);
        ((((&raw mut gPartyMenu).cast::<u8>()).wrapping_add(14)).cast::<i16>())
            .write(((((&raw mut gMoveToLearn).cast::<u16>()).read()) as i16));
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Task_ReplaceMoveYesNo));
    }
}
pub(crate) unsafe extern "C" fn DisplayMonLearnedMove(taskId: u8, r#move: u16) {
    unsafe {
        let mut taskId = taskId;
        let mut r#move = r#move;
        GetMonNickname(
            ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                (((((&raw mut gPartyMenu).cast::<u8>())
                    .wrapping_add(9)
                    .cast::<i8>())
                .read()) as i32) as isize
                    * 100,
            ),
            (&raw mut gStringVar1).cast::<u8>(),
        );
        StringCopy(
            (&raw mut gStringVar2).cast::<u8>(),
            (((&raw mut gMoveNames).cast::<u8>()).wrapping_offset(((r#move) as i32) as isize * 13))
                .cast::<u8>(),
        );
        StringExpandPlaceholders(
            (&raw mut gStringVar4).cast::<u8>(),
            (&raw mut gText_PkmnLearnedMove3).cast::<u8>(),
        );
        DisplayPartyMenuMessage((&raw mut gStringVar4).cast::<u8>(), 1u8);
        ScheduleBgCopyTilemapToVram(2u8);
        ((((&raw mut gPartyMenu).cast::<u8>()).wrapping_add(14)).cast::<i16>())
            .write(((r#move) as i16));
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Task_DoLearnedMoveFanfareAfterText));
    }
}
pub(crate) unsafe extern "C" fn BufferMonStatsToTaskData(mon: *mut u8, data: *mut i16) {
    unsafe {
        let mut mon = mon;
        let mut data = data;
        (data).write(((GetMonData2(mon, 58i32)) as i16));
        ((data).wrapping_offset(1)).write(((GetMonData2(mon, 59i32)) as i16));
        ((data).wrapping_offset(2)).write(((GetMonData2(mon, 60i32)) as i16));
        ((data).wrapping_offset(4)).write(((GetMonData2(mon, 62i32)) as i16));
        ((data).wrapping_offset(5)).write(((GetMonData2(mon, 63i32)) as i16));
        ((data).wrapping_offset(3)).write(((GetMonData2(mon, 61i32)) as i16));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ItemUseCB_SacredAsh(taskId: u8, task: Option<unsafe extern "C" fn(u8)>) {
    unsafe {
        let mut taskId = taskId;
        let mut task = task;
        (((((&raw mut sPartyMenuInternal).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(536))
        .cast::<i16>())
        .write(0i16);
        ((((((&raw mut sPartyMenuInternal).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(536))
        .cast::<i16>())
        .wrapping_offset(1))
        .write(0i16);
        ((((((&raw mut sPartyMenuInternal).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(536))
        .cast::<i16>())
        .wrapping_offset(2))
        .write(
            (((((&raw mut gPartyMenu).cast::<u8>())
                .wrapping_add(9)
                .cast::<i8>())
            .read()) as i16),
        );
        UseSacredAsh(taskId);
    }
}
pub(crate) unsafe extern "C" fn UseSacredAsh(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut mon: *mut u8 = ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
            (((((&raw mut gPartyMenu).cast::<u8>())
                .wrapping_add(9)
                .cast::<i8>())
            .read()) as i32) as isize
                * 100,
        );
        let mut hp: u16 = 0u16;
        if GetMonData2(mon, 11i32) == 0u32 {
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_SacredAshLoop));
            return;
        }
        hp = ((GetMonData2(mon, 57i32)) as u16);
        if (ExecuteTableBasedItemEffect_(
            (((((&raw mut gPartyMenu).cast::<u8>())
                .wrapping_add(9)
                .cast::<i8>())
            .read()) as u8),
            ((&raw mut gSpecialVar_ItemId).cast::<u16>()).read(),
            0u8,
        )) != 0
        {
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_SacredAshLoop));
            return;
        }
        PlaySE(1u16);
        SetPartyMonAilmentGfx(
            mon,
            (((&raw mut sPartyMenuBoxes).cast::<u8>().cast::<*mut u8>()).read()).wrapping_offset(
                (((((&raw mut gPartyMenu).cast::<u8>())
                    .wrapping_add(9)
                    .cast::<i8>())
                .read()) as i32) as isize
                    * 16,
            ),
        );
        if (crate::c::bf_read(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                (((((((&raw mut sPartyMenuBoxes).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_offset(
                        (((((&raw mut gPartyMenu).cast::<u8>())
                            .wrapping_add(9)
                            .cast::<i8>())
                        .read()) as i32) as isize
                            * 16,
                    ))
                .wrapping_add(12))
                .read()) as i32) as isize
                    * 68,
            ))
            .wrapping_add(62),
            2,
            1,
            false,
        ) as u16)
            != 0
        {
            DisplayPartyPokemonLevelCheck(
                mon,
                (((&raw mut sPartyMenuBoxes).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_offset(
                        (((((&raw mut gPartyMenu).cast::<u8>())
                            .wrapping_add(9)
                            .cast::<i8>())
                        .read()) as i32) as isize
                            * 16,
                    ),
                1u8,
            );
        }
        AnimatePartySlot(
            ((((((((&raw mut sPartyMenuInternal).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(536))
            .cast::<i16>())
            .wrapping_offset(2))
            .read()) as u8),
            0u8,
        );
        AnimatePartySlot(
            (((((&raw mut gPartyMenu).cast::<u8>())
                .wrapping_add(9)
                .cast::<i8>())
            .read()) as u8),
            1u8,
        );
        PartyMenuModifyHP(
            taskId,
            (((((&raw mut gPartyMenu).cast::<u8>())
                .wrapping_add(9)
                .cast::<i8>())
            .read()) as u8),
            1i8,
            (((GetMonData2(mon, 57i32)).wrapping_sub(((hp) as u32))) as i16),
            Some(Task_SacredAshDisplayHPRestored),
        );
        ResetHPTaskData(taskId, 0u8, ((hp) as u32));
        (((((&raw mut sPartyMenuInternal).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(536))
        .cast::<i16>())
        .write(1i16);
        ((((((&raw mut sPartyMenuInternal).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(536))
        .cast::<i16>())
        .wrapping_offset(1))
        .write(1i16);
    }
}
pub(crate) unsafe extern "C" fn Task_SacredAshLoop(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if ((IsPartyMenuTextPrinterActive()) as i32) != 1i32 {
            if (((((((&raw mut sPartyMenuInternal).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(536))
            .cast::<i16>())
            .read()) as i32)
                == 1i32
            {
                (((((&raw mut sPartyMenuInternal).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(536))
                .cast::<i16>())
                .write(0i16);
                ((((((&raw mut sPartyMenuInternal).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(536))
                .cast::<i16>())
                .wrapping_offset(2))
                .write(
                    (((((&raw mut gPartyMenu).cast::<u8>())
                        .wrapping_add(9)
                        .cast::<i8>())
                    .read()) as i16),
                );
            }
            if (({
                let __p1 = ((&raw mut gPartyMenu).cast::<u8>())
                    .wrapping_add(9)
                    .cast::<i8>();
                let __t2 = ((__p1).read()).wrapping_add(1);
                (__p1).write(__t2);
                __t2
            }) as i32)
                == 6i32
            {
                if ((((((((&raw mut sPartyMenuInternal).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(536))
                .cast::<i16>())
                .wrapping_offset(1))
                .read()) as i32)
                    == 0i32
                {
                    ((&raw mut gPartyMenuUseExitCallback)
                        .cast::<u8>()
                        .cast::<u8>())
                    .write(0u8);
                    DisplayPartyMenuMessage((&raw mut gText_WontHaveEffect).cast::<u8>(), 1u8);
                    ScheduleBgCopyTilemapToVram(2u8);
                } else {
                    ((&raw mut gPartyMenuUseExitCallback)
                        .cast::<u8>()
                        .cast::<u8>())
                    .write(1u8);
                    RemoveBagItem(((&raw mut gSpecialVar_ItemId).cast::<u16>()).read(), 1u16);
                }
                ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
                .write(Some(Task_ClosePartyMenuAfterText));
                (((&raw mut gPartyMenu).cast::<u8>())
                    .wrapping_add(9)
                    .cast::<i8>())
                .write(0i8);
            } else {
                UseSacredAsh(taskId);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_SacredAshDisplayHPRestored(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        GetMonNickname(
            ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                (((((&raw mut gPartyMenu).cast::<u8>())
                    .wrapping_add(9)
                    .cast::<i8>())
                .read()) as i32) as isize
                    * 100,
            ),
            (&raw mut gStringVar1).cast::<u8>(),
        );
        StringExpandPlaceholders(
            (&raw mut gStringVar4).cast::<u8>(),
            (&raw mut gText_PkmnHPRestoredByVar2).cast::<u8>(),
        );
        DisplayPartyMenuMessage((&raw mut gStringVar4).cast::<u8>(), 0u8);
        ScheduleBgCopyTilemapToVram(2u8);
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Task_SacredAshLoop));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ItemUseCB_EvolutionStone(
    taskId: u8,
    task: Option<unsafe extern "C" fn(u8)>,
) {
    unsafe {
        let mut taskId = taskId;
        let mut task = task;
        PlaySE(5u16);
        ((&raw mut gCB2_AfterEvolution).cast::<Option<unsafe extern "C" fn()>>()).write(
            (((&raw mut gPartyMenu).cast::<u8>()).cast::<Option<unsafe extern "C" fn()>>()).read(),
        );
        if (ExecuteTableBasedItemEffect_(
            (((((&raw mut gPartyMenu).cast::<u8>())
                .wrapping_add(9)
                .cast::<i8>())
            .read()) as u8),
            ((&raw mut gSpecialVar_ItemId).cast::<u16>()).read(),
            0u8,
        )) != 0
        {
            ((&raw mut gPartyMenuUseExitCallback)
                .cast::<u8>()
                .cast::<u8>())
            .write(0u8);
            DisplayPartyMenuMessage((&raw mut gText_WontHaveEffect).cast::<u8>(), 1u8);
            ScheduleBgCopyTilemapToVram(2u8);
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(task);
        } else {
            RemoveBagItem(((&raw mut gSpecialVar_ItemId).cast::<u16>()).read(), 1u16);
            FreePartyPointers();
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetItemEffectType(item: u16) -> u8 {
    unsafe {
        let mut item = item;
        let mut itemEffect: *mut u8 = core::ptr::null_mut();
        let mut statusCure: u32 = 0u32;
        if !((((item) as i32) >= 13i32) && (((item) as i32) <= 178i32)) {
            return 22u8;
        }
        if ((item) as i32) == 175i32 {
            itemEffect = (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                .wrapping_add(12792))
            .wrapping_add(28))
            .cast::<u8>();
        } else {
            itemEffect = ((((&raw mut gItemEffectTable).cast::<*mut u8>()).cast::<*mut u8>())
                .wrapping_offset((((item) as i32).wrapping_sub(13i32)) as isize))
            .read();
        }
        if (((((((itemEffect).read()) as i32) & 63i32) != 0)
            || ((((itemEffect).wrapping_offset(1)).read()) != 0))
            || ((((itemEffect).wrapping_offset(2)).read()) != 0))
            || ((((((itemEffect).wrapping_offset(3)).read()) as i32) & 128i32) != 0)
        {
            return 0u8;
        } else {
            if ((((itemEffect).read()) as i32) & 64i32) != 0 {
                return 10u8;
            } else {
                if (((((itemEffect).wrapping_offset(3)).read()) as i32) & 64i32) != 0 {
                    return 1u8;
                }
            }
        }
        statusCure = ((((((itemEffect).wrapping_offset(3)).read()) as i32) & 63i32) as u32);
        if ((statusCure) != 0) || (((((itemEffect).read()) as i32) >> 7) != 0) {
            if statusCure == 32u32 {
                return 4u8;
            } else {
                if statusCure == 16u32 {
                    return 3u8;
                } else {
                    if statusCure == 8u32 {
                        return 5u8;
                    } else {
                        if statusCure == 4u32 {
                            return 6u8;
                        } else {
                            if statusCure == 2u32 {
                                return 7u8;
                            } else {
                                if statusCure == 1u32 {
                                    return 8u8;
                                } else {
                                    if (((((itemEffect).read()) as i32) >> 7) != 0)
                                        && (!((statusCure) != 0))
                                    {
                                        return 9u8;
                                    } else {
                                        return 11u8;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        if (((((itemEffect).wrapping_offset(4)).read()) as i32) & 68i32) != 0 {
            return 2u8;
        } else {
            if (((((itemEffect).wrapping_offset(4)).read()) as i32) & 2i32) != 0 {
                return 12u8;
            } else {
                if (((((itemEffect).wrapping_offset(4)).read()) as i32) & 1i32) != 0 {
                    return 13u8;
                } else {
                    if (((((itemEffect).wrapping_offset(5)).read()) as i32) & 8i32) != 0 {
                        return 14u8;
                    } else {
                        if (((((itemEffect).wrapping_offset(5)).read()) as i32) & 4i32) != 0 {
                            return 15u8;
                        } else {
                            if (((((itemEffect).wrapping_offset(5)).read()) as i32) & 2i32) != 0 {
                                return 16u8;
                            } else {
                                if (((((itemEffect).wrapping_offset(5)).read()) as i32) & 1i32) != 0
                                {
                                    return 17u8;
                                } else {
                                    if (((((itemEffect).wrapping_offset(4)).read()) as i32)
                                        & 128i32)
                                        != 0
                                    {
                                        return 18u8;
                                    } else {
                                        if (((((itemEffect).wrapping_offset(4)).read()) as i32)
                                            & 32i32)
                                            != 0
                                        {
                                            return 19u8;
                                        } else {
                                            if (((((itemEffect).wrapping_offset(5)).read()) as i32)
                                                & 16i32)
                                                != 0
                                            {
                                                return 20u8;
                                            } else {
                                                if (((((itemEffect).wrapping_offset(4)).read())
                                                    as i32)
                                                    & 24i32)
                                                    != 0
                                                {
                                                    return 21u8;
                                                } else {
                                                    return 22u8;
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
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
pub(crate) unsafe extern "C" fn TryTutorSelectedMon(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut mon: *mut u8 = core::ptr::null_mut();
        let mut r#move: *mut i16 = core::ptr::null_mut();
        if !((crate::c::bf_read(
            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
            7,
            1,
            false,
        ) as u16)
            != 0)
        {
            mon = ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                (((((&raw mut gPartyMenu).cast::<u8>())
                    .wrapping_add(9)
                    .cast::<i8>())
                .read()) as i32) as isize
                    * 100,
            );
            r#move = (((&raw mut gPartyMenu).cast::<u8>()).wrapping_add(14)).cast::<i16>();
            GetMonNickname(mon, (&raw mut gStringVar1).cast::<u8>());
            ((((&raw mut gPartyMenu).cast::<u8>()).wrapping_add(14)).cast::<i16>()).write(
                ((GetTutorMove(((((&raw mut gSpecialVar_0x8005).cast::<u16>()).read()) as u8)))
                    as i16),
            );
            StringCopy(
                (&raw mut gStringVar2).cast::<u8>(),
                (((&raw mut gMoveNames).cast::<u8>()).wrapping_offset(
                    ((((((&raw mut gPartyMenu).cast::<u8>()).wrapping_add(14)).cast::<i16>())
                        .read()) as i32) as isize
                        * 13,
                ))
                .cast::<u8>(),
            );
            ((r#move).wrapping_offset(1)).write(2i16);
            'l1: {
                let __sw1 = ((CanMonLearnTMTutor(
                    mon,
                    0u16,
                    ((((&raw mut gSpecialVar_0x8005).cast::<u16>()).read()) as u8),
                )) as i32);
                let __matched = __sw1 == 1i32 || __sw1 == 2i32;
                if __sw1 == 1i32 {
                    DisplayLearnMoveMessageAndClose(
                        taskId,
                        (&raw mut gText_PkmnCantLearnMove).cast::<u8>(),
                    );
                    return;
                }
                if __sw1 == 2i32 {
                    DisplayLearnMoveMessageAndClose(
                        taskId,
                        (&raw mut gText_PkmnAlreadyKnows).cast::<u8>(),
                    );
                    return;
                }
                if !__matched {
                    if ((GiveMoveToMon(
                        mon,
                        ((((((&raw mut gPartyMenu).cast::<u8>()).wrapping_add(14)).cast::<i16>())
                            .read()) as u16),
                    )) as i32)
                        != 65535i32
                    {
                        Task_LearnedMove(taskId);
                        return;
                    }
                    break 'l1;
                }
            }
            DisplayLearnMoveMessage((&raw mut gText_PkmnNeedsToReplaceMove).cast::<u8>());
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_ReplaceMoveYesNo));
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CB2_PartyMenuFromStartMenu() {
    unsafe {
        InitPartyMenu(
            0u8,
            0u8,
            0u8,
            0u8,
            0u8,
            Some(Task_HandleChooseMonInput),
            Some(CB2_ReturnToFieldWithOpenMenu),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CB2_ChooseMonToGiveItem() {
    unsafe {
        let mut callback: Option<unsafe extern "C" fn()> =
            (if ((CurrentBattlePyramidLocation()) as i32) == 0i32 {
                Some(CB2_ReturnToBagMenu)
            } else {
                Some(CB2_ReturnToPyramidBagMenu)
            });
        InitPartyMenu(
            0u8,
            0u8,
            5u8,
            0u8,
            6u8,
            Some(Task_HandleChooseMonInput),
            callback,
        );
        (((&raw mut gPartyMenu).cast::<u8>())
            .wrapping_add(12)
            .cast::<u16>())
        .write(((&raw mut gSpecialVar_ItemId).cast::<u16>()).read());
    }
}
pub(crate) unsafe extern "C" fn TryGiveItemOrMailToSelectedMon(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        ((&raw mut sPartyMenuItemId).cast::<u8>().cast::<u16>()).write(
            ((GetMonData2(
                ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                    (((((&raw mut gPartyMenu).cast::<u8>())
                        .wrapping_add(9)
                        .cast::<i8>())
                    .read()) as i32) as isize
                        * 100,
                ),
                12i32,
            )) as u16),
        );
        if ((((&raw mut sPartyMenuItemId).cast::<u8>().cast::<u16>()).read()) as i32) == 0i32 {
            GiveItemOrMailToSelectedMon(taskId);
        } else {
            if (ItemIsMail(((&raw mut sPartyMenuItemId).cast::<u8>().cast::<u16>()).read())) != 0 {
                DisplayItemMustBeRemovedFirstMessage(taskId);
            } else {
                DisplayAlreadyHoldingItemSwitchMessage(
                    ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                        (((((&raw mut gPartyMenu).cast::<u8>())
                            .wrapping_add(9)
                            .cast::<i8>())
                        .read()) as i32) as isize
                            * 100,
                    ),
                    ((&raw mut sPartyMenuItemId).cast::<u8>().cast::<u16>()).read(),
                    1u8,
                );
                ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
                .write(Some(Task_SwitchItemsFromBagYesNo));
            }
        }
    }
}
pub(crate) unsafe extern "C" fn GiveItemOrMailToSelectedMon(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if (ItemIsMail(
            (((&raw mut gPartyMenu).cast::<u8>())
                .wrapping_add(12)
                .cast::<u16>())
            .read(),
        )) != 0
        {
            RemoveItemToGiveFromBag(
                (((&raw mut gPartyMenu).cast::<u8>())
                    .wrapping_add(12)
                    .cast::<u16>())
                .read(),
            );
            ((((&raw mut sPartyMenuInternal).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(4)
                .cast::<Option<unsafe extern "C" fn()>>())
            .write(Some(CB2_WriteMailToGiveMonFromBag));
            Task_ClosePartyMenu(taskId);
        } else {
            GiveItemToSelectedMon(taskId);
        }
    }
}
pub(crate) unsafe extern "C" fn GiveItemToSelectedMon(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut item: u16 = 0u16;
        if !((crate::c::bf_read(
            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
            7,
            1,
            false,
        ) as u16)
            != 0)
        {
            item = (((&raw mut gPartyMenu).cast::<u8>())
                .wrapping_add(12)
                .cast::<u16>())
            .read();
            DisplayGaveHeldItemMessage(
                ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                    (((((&raw mut gPartyMenu).cast::<u8>())
                        .wrapping_add(9)
                        .cast::<i8>())
                    .read()) as i32) as isize
                        * 100,
                ),
                item,
                0u8,
                1u8,
            );
            GiveItemToMon(
                ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                    (((((&raw mut gPartyMenu).cast::<u8>())
                        .wrapping_add(9)
                        .cast::<i8>())
                    .read()) as i32) as isize
                        * 100,
                ),
                item,
            );
            RemoveItemToGiveFromBag(item);
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_UpdateHeldItemSpriteAndClosePartyMenu));
        }
    }
}
pub(crate) unsafe extern "C" fn Task_UpdateHeldItemSpriteAndClosePartyMenu(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut slot: i8 = (((&raw mut gPartyMenu).cast::<u8>())
            .wrapping_add(9)
            .cast::<i8>())
        .read();
        if ((IsPartyMenuTextPrinterActive()) as i32) != 1i32 {
            UpdatePartyMonHeldItemSprite(
                ((&raw mut gPlayerParty).cast::<u8>())
                    .wrapping_offset(((slot) as i32) as isize * 100),
                (((&raw mut sPartyMenuBoxes).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_offset(((slot) as i32) as isize * 16),
            );
            Task_ClosePartyMenu(taskId);
        }
    }
}
pub(crate) unsafe extern "C" fn CB2_WriteMailToGiveMonFromBag() {
    unsafe {
        let mut mail: u8 = 0u8;
        GiveItemToMon(
            ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                (((((&raw mut gPartyMenu).cast::<u8>())
                    .wrapping_add(9)
                    .cast::<i8>())
                .read()) as i32) as isize
                    * 100,
            ),
            (((&raw mut gPartyMenu).cast::<u8>())
                .wrapping_add(12)
                .cast::<u16>())
            .read(),
        );
        mail = ((GetMonData2(
            ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                (((((&raw mut gPartyMenu).cast::<u8>())
                    .wrapping_add(9)
                    .cast::<i8>())
                .read()) as i32) as isize
                    * 100,
            ),
            64i32,
        )) as u8);
        DoEasyChatScreen(
            4u8,
            ((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(11232))
                .cast::<u8>())
            .wrapping_offset(((mail) as i32) as isize * 36))
            .cast::<u16>(),
            Some(CB2_ReturnToPartyOrBagMenuFromWritingMail),
            3u8,
        );
    }
}
pub(crate) unsafe extern "C" fn CB2_ReturnToPartyOrBagMenuFromWritingMail() {
    unsafe {
        let mut mon: *mut u8 = ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
            (((((&raw mut gPartyMenu).cast::<u8>())
                .wrapping_add(9)
                .cast::<i8>())
            .read()) as i32) as isize
                * 100,
        );
        let mut item: u16 = ((GetMonData2(mon, 12i32)) as u16);
        if ((((&raw mut gSpecialVar_Result).cast::<u16>()).read()) as i32) == 0i32 {
            TakeMailFromMon(mon);
            SetMonData(
                mon,
                12i32,
                ((&raw mut sPartyMenuItemId).cast::<u8>().cast::<u16>()).cast::<u8>(),
            );
            RemoveBagItem(
                ((&raw mut sPartyMenuItemId).cast::<u8>().cast::<u16>()).read(),
                1u16,
            );
            ReturnGiveItemToBagOrPC(item);
            SetMainCallback2(
                (((&raw mut gPartyMenu).cast::<u8>()).cast::<Option<unsafe extern "C" fn()>>())
                    .read(),
            );
        } else {
            InitPartyMenu(
                (crate::c::bf_read(
                    ((&raw mut gPartyMenu).cast::<u8>()).wrapping_add(8),
                    0,
                    4,
                    false,
                ) as u8),
                255u8,
                (((&raw mut gPartyMenu).cast::<u8>()).wrapping_add(11)).read(),
                1u8,
                127u8,
                Some(Task_DisplayGaveMailFromBagMessage),
                (((&raw mut gPartyMenu).cast::<u8>()).cast::<Option<unsafe extern "C" fn()>>())
                    .read(),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn Task_DisplayGaveMailFromBagMessage(taskId: u8) {
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
            if ((((&raw mut sPartyMenuItemId).cast::<u8>().cast::<u16>()).read()) as i32) != 0i32 {
                DisplaySwitchedHeldItemMessage(
                    (((&raw mut gPartyMenu).cast::<u8>())
                        .wrapping_add(12)
                        .cast::<u16>())
                    .read(),
                    ((&raw mut sPartyMenuItemId).cast::<u8>().cast::<u16>()).read(),
                    0u8,
                );
            } else {
                DisplayGaveHeldItemMessage(
                    ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                        (((((&raw mut gPartyMenu).cast::<u8>())
                            .wrapping_add(9)
                            .cast::<i8>())
                        .read()) as i32) as isize
                            * 100,
                    ),
                    (((&raw mut gPartyMenu).cast::<u8>())
                        .wrapping_add(12)
                        .cast::<u16>())
                    .read(),
                    0u8,
                    1u8,
                );
            }
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_UpdateHeldItemSpriteAndClosePartyMenu));
        }
    }
}
pub(crate) unsafe extern "C" fn Task_SwitchItemsFromBagYesNo(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if ((IsPartyMenuTextPrinterActive()) as i32) != 1i32 {
            PartyMenuDisplayYesNoMenu();
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_HandleSwitchItemsFromBagYesNoInput));
        }
    }
}
pub(crate) unsafe extern "C" fn Task_HandleSwitchItemsFromBagYesNoInput(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut item: u16 = 0u16;
        'l1: {
            let __sw1 = ((Menu_ProcessInputNoWrapClearOnChoose()) as i32);
            let mut __fall = false;
            if __sw1 == 0i32 {
                __fall = true;
                item = (((&raw mut gPartyMenu).cast::<u8>())
                    .wrapping_add(12)
                    .cast::<u16>())
                .read();
                RemoveItemToGiveFromBag(item);
                if ((AddBagItem(
                    ((&raw mut sPartyMenuItemId).cast::<u8>().cast::<u16>()).read(),
                    1u16,
                )) as i32)
                    == 0i32
                {
                    ReturnGiveItemToBagOrPC(item);
                    BufferBagFullCantTakeItemMessage(
                        ((&raw mut sPartyMenuItemId).cast::<u8>().cast::<u16>()).read(),
                    );
                    DisplayPartyMenuMessage((&raw mut gStringVar4).cast::<u8>(), 0u8);
                    ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .cast::<Option<unsafe extern "C" fn(u8)>>())
                    .write(Some(Task_UpdateHeldItemSpriteAndClosePartyMenu));
                } else {
                    if (ItemIsMail(item)) != 0 {
                        ((((&raw mut sPartyMenuInternal).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(4)
                            .cast::<Option<unsafe extern "C" fn()>>())
                        .write(Some(CB2_WriteMailToGiveMonFromBag));
                        Task_ClosePartyMenu(taskId);
                    } else {
                        GiveItemToMon(
                            ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                                (((((&raw mut gPartyMenu).cast::<u8>())
                                    .wrapping_add(9)
                                    .cast::<i8>())
                                .read()) as i32) as isize
                                    * 100,
                            ),
                            item,
                        );
                        DisplaySwitchedHeldItemMessage(
                            item,
                            ((&raw mut sPartyMenuItemId).cast::<u8>().cast::<u16>()).read(),
                            1u8,
                        );
                        ((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .cast::<Option<unsafe extern "C" fn(u8)>>())
                        .write(Some(Task_UpdateHeldItemSpriteAndClosePartyMenu));
                    }
                }
                break 'l1;
            }
            if __sw1 == (-1i32) {
                __fall = true;
                PlaySE(5u16);
            }
            if __fall || __sw1 == 1i32 {
                __fall = true;
                ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
                .write(Some(Task_UpdateHeldItemSpriteAndClosePartyMenu));
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn DisplayItemMustBeRemovedFirstMessage(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        DisplayPartyMenuMessage((&raw mut gText_RemoveMailBeforeItem).cast::<u8>(), 1u8);
        ScheduleBgCopyTilemapToVram(2u8);
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Task_UpdateHeldItemSpriteAndClosePartyMenu));
    }
}
pub(crate) unsafe extern "C" fn RemoveItemToGiveFromBag(item: u16) {
    unsafe {
        let mut item = item;
        if (((((&raw mut gPartyMenu).cast::<u8>()).wrapping_add(11)).read()) as i32) == 6i32 {
            RemovePCItem(((item) as u8), 1u16);
        } else {
            RemoveBagItem(item, 1u16);
        }
    }
}
pub(crate) unsafe extern "C" fn ReturnGiveItemToBagOrPC(item: u16) -> u8 {
    unsafe {
        let mut item = item;
        if (((((&raw mut gPartyMenu).cast::<u8>()).wrapping_add(11)).read()) as i32) == 5i32 {
            return AddBagItem(item, 1u16);
        } else {
            return AddPCItem(item, 1u16);
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ChooseMonToGiveMailFromMailbox() {
    unsafe {
        InitPartyMenu(
            0u8,
            0u8,
            7u8,
            0u8,
            6u8,
            Some(Task_HandleChooseMonInput),
            Some(Mailbox_ReturnToMailListAfterDeposit),
        );
    }
}
pub(crate) unsafe extern "C" fn TryGiveMailToSelectedMon(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut mon: *mut u8 = ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
            (((((&raw mut gPartyMenu).cast::<u8>())
                .wrapping_add(9)
                .cast::<i8>())
            .read()) as i32) as isize
                * 100,
        );
        let mut mail: *mut u8 = core::ptr::null_mut();
        ((&raw mut gPartyMenuUseExitCallback)
            .cast::<u8>()
            .cast::<u8>())
        .write(0u8);
        mail = (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(11232))
            .cast::<u8>())
        .wrapping_offset(
            (((((((&raw mut gPlayerPCItemPageInfo).cast::<u8>())
                .wrapping_add(2)
                .cast::<u16>())
            .read()) as i32)
                .wrapping_add(6i32))
            .wrapping_add(
                (((((&raw mut gPlayerPCItemPageInfo).cast::<u8>()).cast::<u16>()).read()) as i32),
            )) as isize
                * 36,
        );
        if GetMonData2(mon, 12i32) != 0u32 {
            DisplayPartyMenuMessage(
                (&raw mut gText_PkmnHoldingItemCantHoldMail).cast::<u8>(),
                1u8,
            );
        } else {
            GiveMailToMon(mon, mail);
            ClearMail(mail);
            DisplayPartyMenuMessage(
                (&raw mut gText_MailTransferredFromMailbox).cast::<u8>(),
                1u8,
            );
        }
        ScheduleBgCopyTilemapToVram(2u8);
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Task_UpdateHeldItemSpriteAndClosePartyMenu));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn InitChooseHalfPartyForBattle(unused: u8) {
    unsafe {
        let mut unused = unused;
        ClearSelectedPartyOrder();
        InitPartyMenu(
            4u8,
            0u8,
            0u8,
            0u8,
            0u8,
            Some(Task_HandleChooseMonInput),
            (((&raw mut gMain).cast::<u8>())
                .wrapping_add(8)
                .cast::<Option<unsafe extern "C" fn()>>())
            .read(),
        );
        (((&raw mut gPartyMenu).cast::<u8>())
            .wrapping_add(4)
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Task_ValidateChosenHalfParty));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ClearSelectedPartyOrder() {
    unsafe {
        crate::c::memset(
            ((&raw mut gSelectedOrderFromParty).cast::<u8>()).cast::<u8>(),
            0i32,
            4u32,
        );
    }
}
pub(crate) unsafe extern "C" fn GetPartySlotEntryStatus(slot: i8) -> u8 {
    unsafe {
        let mut slot = slot;
        if ((GetBattleEntryEligibility(
            ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(((slot) as i32) as isize * 100),
        )) as i32)
            == 0i32
        {
            return 2u8;
        }
        if ((HasPartySlotAlreadyBeenSelected(((((slot) as i32).wrapping_add(1i32)) as u8))) as i32)
            == 1i32
        {
            return 1u8;
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn GetBattleEntryEligibility(mon: *mut u8) -> u8 {
    unsafe {
        let mut mon = mon;
        let mut i: u16 = 0u16;
        let mut species: u16 = 0u16;
        if (((GetMonData2(mon, 45i32)) != 0)
            || (GetMonData2(mon, 56i32) > ((GetBattleEntryLevelCap()) as u32)))
            || ((((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4))
                .cast::<i8>())
            .read()) as i32)
                == 26i32)
                && ((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4))
                    .wrapping_add(1)
                    .cast::<i8>())
                .read()) as i32)
                    == 25i32))
                && (GetMonData2(mon, 12i32) != 0u32))
        {
            return 0u8;
        }
        'l1: {
            let __sw1 = ((VarGet(16591u16)) as i32);
            let __matched = __sw1 == 9i32 || __sw1 == 8i32;
            if __sw1 == 9i32 {
                if GetMonData2(mon, 57i32) != 0u32 {
                    return 1u8;
                }
                return 0u8;
            }
            if __sw1 == 8i32 {
                return 1u8;
            }
            if !__matched {
                species = ((GetMonData2(mon, 11i32)) as u16);
                {
                    'l2: loop {
                        if !(((((((&raw mut gFrontierBannedSpecies).cast::<u16>()).cast::<u16>())
                            .wrapping_offset(((i) as i32) as isize))
                        .read()) as i32)
                            != 65535i32)
                        {
                            break 'l2;
                        }
                        'l3: {
                            if ((((((&raw mut gFrontierBannedSpecies).cast::<u16>())
                                .cast::<u16>())
                            .wrapping_offset(((i) as i32) as isize))
                            .read()) as i32)
                                == ((species) as i32)
                            {
                                return 0u8;
                            }
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                return 1u8;
            }
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
pub(crate) unsafe extern "C" fn CheckBattleEntriesAndGetMessage() -> u8 {
    unsafe {
        let mut maxBattlers: u8 = 0u8;
        let mut i: u8 = 0u8;
        let mut j: u8 = 0u8;
        let mut facility: u8 = 0u8;
        let mut party: *mut u8 = (&raw mut gPlayerParty).cast::<u8>();
        let mut minBattlers: u8 = GetMinBattleEntries();
        let mut order: *mut u8 = ((&raw mut gSelectedOrderFromParty).cast::<u8>()).cast::<u8>();
        if ((((order).wrapping_offset((((minBattlers) as i32).wrapping_sub(1i32)) as isize)).read())
            as i32)
            == 0i32
        {
            if ((minBattlers) as i32) == 1i32 {
                return 14u8;
            }
            ConvertIntToDecimalStringN(
                (&raw mut gStringVar1).cast::<u8>(),
                ((minBattlers) as i32),
                0i32,
                1u8,
            );
            return 17u8;
        }
        facility = ((VarGet(16591u16)) as u8);
        if (((facility) as i32) == 8i32) || (((facility) as i32) == 9i32) {
            return 255u8;
        }
        maxBattlers = GetMaxBattleEntries();
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < ((maxBattlers) as i32).wrapping_sub(1i32)) {
                    break 'l1;
                }
                'l2: {
                    let mut species: u16 = ((GetMonData2(
                        (party).wrapping_offset(
                            (((((order).wrapping_offset(((i) as i32) as isize)).read()) as i32)
                                .wrapping_sub(1i32)) as isize
                                * 100,
                        ),
                        11i32,
                    )) as u16);
                    let mut item: u16 = ((GetMonData2(
                        (party).wrapping_offset(
                            (((((order).wrapping_offset(((i) as i32) as isize)).read()) as i32)
                                .wrapping_sub(1i32)) as isize
                                * 100,
                        ),
                        12i32,
                    )) as u16);
                    {
                        j = ((((i) as i32).wrapping_add(1i32)) as u8);
                        'l3: loop {
                            if !(((j) as i32) < ((maxBattlers) as i32)) {
                                break 'l3;
                            }
                            'l4: {
                                if ((species) as u32)
                                    == GetMonData2(
                                        (party).wrapping_offset(
                                            (((((order).wrapping_offset(((j) as i32) as isize))
                                                .read())
                                                as i32)
                                                .wrapping_sub(1i32))
                                                as isize
                                                * 100,
                                        ),
                                        11i32,
                                    )
                                {
                                    return 18u8;
                                }
                                if (((item) as i32) != 0i32)
                                    && (((item) as u32)
                                        == GetMonData2(
                                            (party).wrapping_offset(
                                                (((((order).wrapping_offset(((j) as i32) as isize))
                                                    .read())
                                                    as i32)
                                                    .wrapping_sub(1i32))
                                                    as isize
                                                    * 100,
                                            ),
                                            12i32,
                                        ))
                                {
                                    return 19u8;
                                }
                            }
                            j = (j).wrapping_add(1);
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return 255u8;
    }
}
pub(crate) unsafe extern "C" fn HasPartySlotAlreadyBeenSelected(slot: u8) -> u8 {
    unsafe {
        let mut slot = slot;
        let mut i: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as u32) < crate::c::div_u32(4u32, 1u32)) {
                    break 'l1;
                }
                'l2: {
                    if ((((((&raw mut gSelectedOrderFromParty).cast::<u8>()).cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize))
                    .read()) as i32)
                        == ((slot) as i32)
                    {
                        return 1u8;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn Task_ValidateChosenHalfParty(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut msgId: u8 = CheckBattleEntriesAndGetMessage();
        if ((msgId) as i32) != 255i32 {
            PlaySE(32u16);
            DisplayPartyMenuStdMessage(((msgId) as u32));
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_ContinueChoosingHalfParty));
        } else {
            PlaySE(5u16);
            Task_ClosePartyMenu(taskId);
        }
    }
}
pub(crate) unsafe extern "C" fn Task_ContinueChoosingHalfParty(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
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
            DisplayPartyMenuStdMessage(0u32);
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_HandleChooseMonInput));
        }
    }
}
pub(crate) unsafe extern "C" fn GetMaxBattleEntries() -> u8 {
    unsafe {
        'l1: {
            let __sw1 = ((VarGet(16591u16)) as i32);
            let __matched = __sw1 == 9i32 || __sw1 == 8i32;
            if __sw1 == 9i32 {
                return ((crate::c::div_i32(6i32, 2i32)) as u8);
            }
            if __sw1 == 8i32 {
                return 2u8;
            }
            if !__matched {
                return ((((&raw mut gSpecialVar_0x8005).cast::<u16>()).read()) as u8);
            }
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
pub(crate) unsafe extern "C" fn GetMinBattleEntries() -> u8 {
    unsafe {
        'l1: {
            let __sw1 = ((VarGet(16591u16)) as i32);
            let __matched = __sw1 == 9i32 || __sw1 == 8i32;
            if __sw1 == 9i32 {
                return 1u8;
            }
            if __sw1 == 8i32 {
                return 2u8;
            }
            if !__matched {
                return ((((&raw mut gSpecialVar_0x8005).cast::<u16>()).read()) as u8);
            }
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
pub(crate) unsafe extern "C" fn GetBattleEntryLevelCap() -> u8 {
    unsafe {
        'l1: {
            let __sw1 = ((VarGet(16591u16)) as i32);
            let __matched = __sw1 == 9i32 || __sw1 == 8i32;
            if __sw1 == 9i32 {
                return 100u8;
            }
            if __sw1 == 8i32 {
                return 30u8;
            }
            if !__matched {
                if ((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32) == 0i32 {
                    return 50u8;
                }
                return 100u8;
            }
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
pub(crate) unsafe extern "C" fn GetFacilityCancelString() -> *mut u8 {
    unsafe {
        let mut facilityNum: u8 = ((VarGet(16591u16)) as u8);
        if !((((facilityNum) as i32) != 8i32) && (((facilityNum) as i32) != 9i32)) {
            return (&raw mut gText_CancelBattle).cast::<u8>();
        } else {
            if (((facilityNum) as i32) == 1i32)
                && (((((&raw mut gSpecialVar_0x8005).cast::<u16>()).read()) as i32) == 2i32)
            {
                return (&raw mut gText_ReturnToWaitingRoom).cast::<u8>();
            } else {
                return (&raw mut gText_CancelChallenge).cast::<u8>();
            }
        }
        #[allow(unreachable_code)]
        {
            return core::ptr::null_mut();
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ChooseMonForTradingBoard(
    menuType: u8,
    callback: Option<unsafe extern "C" fn()>,
) {
    unsafe {
        let mut menuType = menuType;
        let mut callback = callback;
        InitPartyMenu(
            menuType,
            0u8,
            0u8,
            0u8,
            0u8,
            Some(Task_HandleChooseMonInput),
            callback,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ChooseMonForMoveTutor() {
    unsafe {
        InitPartyMenu(
            0u8,
            0u8,
            12u8,
            0u8,
            4u8,
            Some(Task_HandleChooseMonInput),
            Some(CB2_ReturnToFieldContinueScriptPlayMapMusic),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ChooseMonForWirelessMinigame() {
    unsafe {
        InitPartyMenu(
            11u8,
            0u8,
            13u8,
            0u8,
            1u8,
            Some(Task_HandleChooseMonInput),
            Some(CB2_ReturnToFieldContinueScriptPlayMapMusic),
        );
    }
}
pub(crate) unsafe extern "C" fn GetPartyLayoutFromBattleType() -> u8 {
    unsafe {
        if ((IsDoubleBattle()) as i32) == 0i32 {
            return 0u8;
        }
        if ((IsMultiBattle()) as i32) == 1i32 {
            return 2u8;
        }
        return 1u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn OpenPartyMenuInBattle(partyAction: u8) {
    unsafe {
        let mut partyAction = partyAction;
        InitPartyMenu(
            1u8,
            GetPartyLayoutFromBattleType(),
            partyAction,
            0u8,
            0u8,
            Some(Task_HandleChooseMonInput),
            Some(CB2_SetUpReshowBattleScreenAfterMenu),
        );
        ReshowBattleScreenDummy();
        UpdatePartyToBattleOrder();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ChooseMonForInBattleItem() {
    unsafe {
        InitPartyMenu(
            1u8,
            GetPartyLayoutFromBattleType(),
            3u8,
            0u8,
            5u8,
            Some(Task_HandleChooseMonInput),
            Some(CB2_ReturnToBagMenu),
        );
        ReshowBattleScreenDummy();
        UpdatePartyToBattleOrder();
    }
}
pub(crate) unsafe extern "C" fn GetPartyMenuActionsTypeInBattle(mon: *mut u8) -> u8 {
    unsafe {
        let mut mon = mon;
        if (GetMonData2(
            ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(100),
            11i32,
        ) != 0u32)
            && (GetMonData2(mon, 45i32) == 0u32)
        {
            if (((((&raw mut gPartyMenu).cast::<u8>()).wrapping_add(11)).read()) as i32) == 1i32 {
                return 3u8;
            }
            if !((((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 262144u32) != 0) {
                return 2u8;
            }
        }
        return 7u8;
    }
}
pub(crate) unsafe extern "C" fn TrySwitchInPokemon() -> u8 {
    unsafe {
        let mut slot: u8 = GetCursorSelectionMonId();
        let mut newSlot: u8 = 0u8;
        let mut i: u8 = 0u8;
        if (((IsMultiBattle()) as i32) == 1i32)
            && (((((slot) as i32) == 1i32) || (((slot) as i32) == 4i32))
                || (((slot) as i32) == 5i32))
        {
            StringCopy((&raw mut gStringVar1).cast::<u8>(), GetTrainerPartnerName());
            StringExpandPlaceholders(
                (&raw mut gStringVar4).cast::<u8>(),
                (&raw mut gText_CantSwitchWithAlly).cast::<u8>(),
            );
            return 0u8;
        }
        if GetMonData2(
            ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(((slot) as i32) as isize * 100),
            57i32,
        ) == 0u32
        {
            GetMonNickname(
                ((&raw mut gPlayerParty).cast::<u8>())
                    .wrapping_offset(((slot) as i32) as isize * 100),
                (&raw mut gStringVar1).cast::<u8>(),
            );
            StringExpandPlaceholders(
                (&raw mut gStringVar4).cast::<u8>(),
                (&raw mut gText_PkmnHasNoEnergy).cast::<u8>(),
            );
            return 0u8;
        }
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < ((((&raw mut gBattlersCount).cast::<u8>()).read()) as i32)) {
                    break 'l1;
                }
                'l2: {
                    if (((GetBattlerSide(i)) as i32) == 0i32)
                        && (((GetPartyIdFromBattleSlot(slot)) as i32)
                            == ((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                                .wrapping_offset(((i) as i32) as isize))
                            .read()) as i32))
                    {
                        GetMonNickname(
                            ((&raw mut gPlayerParty).cast::<u8>())
                                .wrapping_offset(((slot) as i32) as isize * 100),
                            (&raw mut gStringVar1).cast::<u8>(),
                        );
                        StringExpandPlaceholders(
                            (&raw mut gStringVar4).cast::<u8>(),
                            (&raw mut gText_PkmnAlreadyInBattle).cast::<u8>(),
                        );
                        return 0u8;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        if (GetMonData2(
            ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(((slot) as i32) as isize * 100),
            45i32,
        )) != 0
        {
            StringExpandPlaceholders(
                (&raw mut gStringVar4).cast::<u8>(),
                (&raw mut gText_EggCantBattle).cast::<u8>(),
            );
            return 0u8;
        }
        if ((GetPartyIdFromBattleSlot(slot)) as i32)
            == ((((((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(139)).read())
                as i32)
        {
            GetMonNickname(
                ((&raw mut gPlayerParty).cast::<u8>())
                    .wrapping_offset(((slot) as i32) as isize * 100),
                (&raw mut gStringVar1).cast::<u8>(),
            );
            StringExpandPlaceholders(
                (&raw mut gStringVar4).cast::<u8>(),
                (&raw mut gText_PkmnAlreadySelected).cast::<u8>(),
            );
            return 0u8;
        }
        if (((((&raw mut gPartyMenu).cast::<u8>()).wrapping_add(11)).read()) as i32) == 4i32 {
            SetMonPreventsSwitchingString();
            return 0u8;
        }
        if (((((&raw mut gPartyMenu).cast::<u8>()).wrapping_add(11)).read()) as i32) == 2i32 {
            let mut currBattler: u8 = ((&raw mut gBattlerInMenuId).cast::<u8>()).read();
            GetMonNickname(
                ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                    ((GetPartyIdFromBattlePartyId(
                        ((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                            .wrapping_offset(((currBattler) as i32) as isize))
                        .read()) as u8),
                    )) as i32) as isize
                        * 100,
                ),
                (&raw mut gStringVar1).cast::<u8>(),
            );
            StringExpandPlaceholders(
                (&raw mut gStringVar4).cast::<u8>(),
                (&raw mut gText_PkmnCantSwitchOut).cast::<u8>(),
            );
            return 0u8;
        }
        ((&raw mut gSelectedMonPartyId).cast::<u8>().cast::<u8>())
            .write(GetPartyIdFromBattleSlot(slot));
        ((&raw mut gPartyMenuUseExitCallback)
            .cast::<u8>()
            .cast::<u8>())
        .write(1u8);
        newSlot = GetPartyIdFromBattlePartyId(
            ((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>()).wrapping_offset(
                ((((&raw mut gBattlerInMenuId).cast::<u8>()).read()) as i32) as isize,
            ))
            .read()) as u8),
        );
        SwitchPartyMonSlots(newSlot, slot);
        SwapPartyPokemon(
            ((&raw mut gPlayerParty).cast::<u8>())
                .wrapping_offset(((newSlot) as i32) as isize * 100),
            ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(((slot) as i32) as isize * 100),
        );
        return 1u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BufferBattlePartyCurrentOrder() {
    unsafe {
        BufferBattlePartyOrder(
            ((&raw mut gBattlePartyCurrentOrder).cast::<u8>()).cast::<u8>(),
            GetPlayerFlankId(),
        );
    }
}
pub(crate) unsafe extern "C" fn BufferBattlePartyOrder(partyBattleOrder: *mut u8, flankId: u8) {
    unsafe {
        let mut partyBattleOrder = partyBattleOrder;
        let mut flankId = flankId;
        let mut partyIds = crate::ffi::Align4([0u8; 6]);
        let mut i: i32 = 0i32;
        let mut j: i32 = 0i32;
        if ((IsMultiBattle()) as i32) == 1i32 {
            if ((flankId) as i32) != 0i32 {
                (partyBattleOrder).write(48u8);
                ((partyBattleOrder).wrapping_offset(1)).write(69u8);
                ((partyBattleOrder).wrapping_offset(2)).write(18u8);
            } else {
                (partyBattleOrder).write(3u8);
                ((partyBattleOrder).wrapping_offset(1)).write(18u8);
                ((partyBattleOrder).wrapping_offset(2)).write(69u8);
            }
            return;
        } else {
            if ((IsDoubleBattle()) as i32) == 0i32 {
                j = 1i32;
                ((&raw mut partyIds).cast::<u8>()).write(
                    ((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                        .wrapping_offset(((GetBattlerAtPosition(0u8)) as i32) as isize))
                    .read()) as u8),
                );
                {
                    i = 0i32;
                    'l1: loop {
                        if !(i < 6i32) {
                            break 'l1;
                        }
                        'l2: {
                            if i != ((((&raw mut partyIds).cast::<u8>()).read()) as i32) {
                                (((&raw mut partyIds).cast::<u8>()).wrapping_offset((j) as isize))
                                    .write(((i) as u8));
                                j = (j).wrapping_add(1);
                            }
                        }
                        i = (i).wrapping_add(1);
                    }
                }
            } else {
                j = 2i32;
                ((&raw mut partyIds).cast::<u8>()).write(
                    ((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                        .wrapping_offset(((GetBattlerAtPosition(0u8)) as i32) as isize))
                    .read()) as u8),
                );
                (((&raw mut partyIds).cast::<u8>()).wrapping_offset(1)).write(
                    ((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                        .wrapping_offset(((GetBattlerAtPosition(2u8)) as i32) as isize))
                    .read()) as u8),
                );
                {
                    i = 0i32;
                    'l3: loop {
                        if !(i < 6i32) {
                            break 'l3;
                        }
                        'l4: {
                            if (i != ((((&raw mut partyIds).cast::<u8>()).read()) as i32))
                                && (i
                                    != (((((&raw mut partyIds).cast::<u8>()).wrapping_offset(1))
                                        .read()) as i32))
                            {
                                (((&raw mut partyIds).cast::<u8>()).wrapping_offset((j) as isize))
                                    .write(((i) as u8));
                                j = (j).wrapping_add(1);
                            }
                        }
                        i = (i).wrapping_add(1);
                    }
                }
            }
        }
        {
            i = 0i32;
            'l5: loop {
                if !(i < ((crate::c::div_u32(3u32, 1u32)) as i32)) {
                    break 'l5;
                }
                'l6: {
                    ((partyBattleOrder).wrapping_offset((i) as isize)).write(
                        ((((((((&raw mut partyIds).cast::<u8>()).wrapping_offset(
                            ((0i32).wrapping_add((i).wrapping_mul(2i32))) as isize,
                        ))
                        .read()) as i32)
                            << 4)
                            | (((((&raw mut partyIds).cast::<u8>()).wrapping_offset(
                                ((1i32).wrapping_add((i).wrapping_mul(2i32))) as isize,
                            ))
                            .read()) as i32)) as u8),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BufferBattlePartyCurrentOrderBySide(battler: u8, flankId: u8) {
    unsafe {
        let mut battler = battler;
        let mut flankId = flankId;
        BufferBattlePartyOrderBySide(
            ((((((&raw mut gBattleStruct).cast::<*mut u8>()).read()).wrapping_add(96))
                .cast::<u8>())
            .wrapping_offset(((battler) as i32) as isize * 3))
            .cast::<u8>(),
            flankId,
            battler,
        );
    }
}
pub(crate) unsafe extern "C" fn BufferBattlePartyOrderBySide(
    partyBattleOrder: *mut u8,
    flankId: u8,
    battler: u8,
) {
    unsafe {
        let mut partyBattleOrder = partyBattleOrder;
        let mut flankId = flankId;
        let mut battler = battler;
        let mut partyIndexes = crate::ffi::Align4([0u8; 6]);
        let mut i: i32 = 0i32;
        let mut j: i32 = 0i32;
        let mut leftBattler: u8 = 0u8;
        let mut rightBattler: u8 = 0u8;
        if ((GetBattlerSide(battler)) as i32) == 0i32 {
            leftBattler = GetBattlerAtPosition(0u8);
            rightBattler = GetBattlerAtPosition(2u8);
        } else {
            leftBattler = GetBattlerAtPosition(1u8);
            rightBattler = GetBattlerAtPosition(3u8);
        }
        if ((IsMultiBattle()) as i32) == 1i32 {
            if ((flankId) as i32) != 0i32 {
                (partyBattleOrder).write(48u8);
                ((partyBattleOrder).wrapping_offset(1)).write(69u8);
                ((partyBattleOrder).wrapping_offset(2)).write(18u8);
            } else {
                (partyBattleOrder).write(3u8);
                ((partyBattleOrder).wrapping_offset(1)).write(18u8);
                ((partyBattleOrder).wrapping_offset(2)).write(69u8);
            }
            return;
        } else {
            if ((IsDoubleBattle()) as i32) == 0i32 {
                j = 1i32;
                ((&raw mut partyIndexes).cast::<u8>()).write(
                    ((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                        .wrapping_offset(((leftBattler) as i32) as isize))
                    .read()) as u8),
                );
                {
                    i = 0i32;
                    'l1: loop {
                        if !(i < 6i32) {
                            break 'l1;
                        }
                        'l2: {
                            if i != ((((&raw mut partyIndexes).cast::<u8>()).read()) as i32) {
                                (((&raw mut partyIndexes).cast::<u8>())
                                    .wrapping_offset((j) as isize))
                                .write(((i) as u8));
                                j = (j).wrapping_add(1);
                            }
                        }
                        i = (i).wrapping_add(1);
                    }
                }
            } else {
                j = 2i32;
                ((&raw mut partyIndexes).cast::<u8>()).write(
                    ((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                        .wrapping_offset(((leftBattler) as i32) as isize))
                    .read()) as u8),
                );
                (((&raw mut partyIndexes).cast::<u8>()).wrapping_offset(1)).write(
                    ((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                        .wrapping_offset(((rightBattler) as i32) as isize))
                    .read()) as u8),
                );
                {
                    i = 0i32;
                    'l3: loop {
                        if !(i < 6i32) {
                            break 'l3;
                        }
                        'l4: {
                            if (i != ((((&raw mut partyIndexes).cast::<u8>()).read()) as i32))
                                && (i
                                    != (((((&raw mut partyIndexes).cast::<u8>())
                                        .wrapping_offset(1))
                                    .read()) as i32))
                            {
                                (((&raw mut partyIndexes).cast::<u8>())
                                    .wrapping_offset((j) as isize))
                                .write(((i) as u8));
                                j = (j).wrapping_add(1);
                            }
                        }
                        i = (i).wrapping_add(1);
                    }
                }
            }
        }
        {
            i = 0i32;
            'l5: loop {
                if !(i < 3i32) {
                    break 'l5;
                }
                'l6: {
                    ((partyBattleOrder).wrapping_offset((i) as isize)).write(
                        ((((((((&raw mut partyIndexes).cast::<u8>()).wrapping_offset(
                            ((0i32).wrapping_add((i).wrapping_mul(2i32))) as isize,
                        ))
                        .read()) as i32)
                            << 4)
                            | (((((&raw mut partyIndexes).cast::<u8>()).wrapping_offset(
                                ((1i32).wrapping_add((i).wrapping_mul(2i32))) as isize,
                            ))
                            .read()) as i32)) as u8),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SwitchPartyOrderLinkMulti(battler: u8, slot: u8, slot2: u8) {
    unsafe {
        let mut battler = battler;
        let mut slot = slot;
        let mut slot2 = slot2;
        let mut partyIds = crate::ffi::Align4([0u8; 6]);
        let mut tempSlot: u8 = 0u8;
        let mut i: i32 = 0i32;
        let mut j: i32 = 0i32;
        let mut partyBattleOrder: *mut u8 = core::ptr::null_mut();
        let mut partyIdBuffer: u8 = 0u8;
        if (IsMultiBattle()) != 0 {
            partyBattleOrder = ((((((&raw mut gBattleStruct).cast::<*mut u8>()).read())
                .wrapping_add(96))
            .cast::<u8>())
            .wrapping_offset(((battler) as i32) as isize * 3))
            .cast::<u8>();
            {
                i = {
                    let __v1 = 0i32;
                    j = __v1;
                    __v1
                };
                'l1: loop {
                    if !(i < crate::c::div_i32(6i32, 2i32)) {
                        break 'l1;
                    }
                    'l2: {
                        (((&raw mut partyIds).cast::<u8>()).wrapping_offset((j) as isize)).write(
                            ((((((partyBattleOrder).wrapping_offset((i) as isize)).read()) as i32)
                                >> 4) as u8),
                        );
                        j = (j).wrapping_add(1);
                        (((&raw mut partyIds).cast::<u8>()).wrapping_offset((j) as isize)).write(
                            ((((((partyBattleOrder).wrapping_offset((i) as isize)).read()) as i32)
                                & 15i32) as u8),
                        );
                    }
                    j = (j).wrapping_add(1);
                    i = (i).wrapping_add(1);
                }
            }
            partyIdBuffer = (((&raw mut partyIds).cast::<u8>())
                .wrapping_offset(((slot2) as i32) as isize))
            .read();
            {
                i = 0i32;
                'l3: loop {
                    if !(i < 6i32) {
                        break 'l3;
                    }
                    'l4: {
                        if (((((&raw mut partyIds).cast::<u8>()).wrapping_offset((i) as isize))
                            .read()) as i32)
                            == ((slot) as i32)
                        {
                            tempSlot = (((&raw mut partyIds).cast::<u8>())
                                .wrapping_offset((i) as isize))
                            .read();
                            (((&raw mut partyIds).cast::<u8>()).wrapping_offset((i) as isize))
                                .write(partyIdBuffer);
                            break 'l3;
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
            if i != 6i32 {
                (((&raw mut partyIds).cast::<u8>()).wrapping_offset(((slot2) as i32) as isize))
                    .write(tempSlot);
                (partyBattleOrder).write(
                    (((((((&raw mut partyIds).cast::<u8>()).read()) as i32) << 4)
                        | (((((&raw mut partyIds).cast::<u8>()).wrapping_offset(1)).read()) as i32))
                        as u8),
                );
                ((partyBattleOrder).wrapping_offset(1)).write(
                    ((((((((&raw mut partyIds).cast::<u8>()).wrapping_offset(2)).read()) as i32)
                        << 4)
                        | (((((&raw mut partyIds).cast::<u8>()).wrapping_offset(3)).read()) as i32))
                        as u8),
                );
                ((partyBattleOrder).wrapping_offset(2)).write(
                    ((((((((&raw mut partyIds).cast::<u8>()).wrapping_offset(4)).read()) as i32)
                        << 4)
                        | (((((&raw mut partyIds).cast::<u8>()).wrapping_offset(5)).read()) as i32))
                        as u8),
                );
            }
        }
    }
}
pub(crate) unsafe extern "C" fn GetPartyIdFromBattleSlot(slot: u8) -> u8 {
    unsafe {
        let mut slot = slot;
        let mut modResult: u8 = ((((slot) as i32) & 1i32) as u8);
        let mut retVal: u8 = 0u8;
        slot = ((crate::c::div_i32(((slot) as i32), 2i32)) as u8);
        if ((modResult) as i32) != 0i32 {
            retVal = ((((((((&raw mut gBattlePartyCurrentOrder).cast::<u8>()).cast::<u8>())
                .wrapping_offset(((slot) as i32) as isize))
            .read()) as i32)
                & 15i32) as u8);
        } else {
            retVal = ((((((((&raw mut gBattlePartyCurrentOrder).cast::<u8>()).cast::<u8>())
                .wrapping_offset(((slot) as i32) as isize))
            .read()) as i32)
                >> 4) as u8);
        }
        return retVal;
    }
}
pub(crate) unsafe extern "C" fn SetPartyIdAtBattleSlot(slot: u8, setVal: u8) {
    unsafe {
        let mut slot = slot;
        let mut setVal = setVal;
        let mut modResult: u32 = ((((slot) as i32) & 1i32) as u32);
        slot = ((crate::c::div_i32(((slot) as i32), 2i32)) as u8);
        if modResult != 0u32 {
            ((((&raw mut gBattlePartyCurrentOrder).cast::<u8>()).cast::<u8>())
                .wrapping_offset(((slot) as i32) as isize))
            .write(
                (((((((((&raw mut gBattlePartyCurrentOrder).cast::<u8>()).cast::<u8>())
                    .wrapping_offset(((slot) as i32) as isize))
                .read()) as i32)
                    & 240i32)
                    | ((setVal) as i32)) as u8),
            );
        } else {
            ((((&raw mut gBattlePartyCurrentOrder).cast::<u8>()).cast::<u8>())
                .wrapping_offset(((slot) as i32) as isize))
            .write(
                (((((((((&raw mut gBattlePartyCurrentOrder).cast::<u8>()).cast::<u8>())
                    .wrapping_offset(((slot) as i32) as isize))
                .read()) as i32)
                    & 15i32)
                    | (((setVal) as i32) << 4)) as u8),
            );
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SwitchPartyMonSlots(slot: u8, slot2: u8) {
    unsafe {
        let mut slot = slot;
        let mut slot2 = slot2;
        let mut partyId: u8 = GetPartyIdFromBattleSlot(slot);
        SetPartyIdAtBattleSlot(slot, GetPartyIdFromBattleSlot(slot2));
        SetPartyIdAtBattleSlot(slot2, partyId);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetPartyIdFromBattlePartyId(battlePartyId: u8) -> u8 {
    unsafe {
        let mut battlePartyId = battlePartyId;
        let mut i: u8 = 0u8;
        let mut j: u8 = 0u8;
        {
            j = {
                let __v1 = 0u8;
                i = __v1;
                __v1
            };
            'l1: loop {
                if !(((i) as i32) < ((crate::c::div_u32(3u32, 1u32)) as i32)) {
                    break 'l1;
                }
                'l2: {
                    if (((((((&raw mut gBattlePartyCurrentOrder).cast::<u8>()).cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize))
                    .read()) as i32)
                        >> 4)
                        != ((battlePartyId) as i32)
                    {
                        j = (j).wrapping_add(1);
                        if (((((((&raw mut gBattlePartyCurrentOrder).cast::<u8>()).cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize))
                        .read()) as i32)
                            & 15i32)
                            == ((battlePartyId) as i32)
                        {
                            return j;
                        }
                    } else {
                        return j;
                    }
                }
                j = (j).wrapping_add(1);
                i = (i).wrapping_add(1);
            }
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn UpdatePartyToBattleOrder() {
    unsafe {
        let mut partyBuffer: *mut u8 = Alloc(600u32);
        let mut i: u8 = 0u8;
        crate::c::memcpy(partyBuffer, (&raw mut gPlayerParty).cast::<u8>(), 600u32);
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 6i32) {
                    break 'l1;
                }
                'l2: {
                    crate::c::memcpy(
                        ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                            ((GetPartyIdFromBattlePartyId(i)) as i32) as isize * 100,
                        ),
                        (partyBuffer).wrapping_offset(((i) as i32) as isize * 100),
                        100u32,
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        Free(partyBuffer);
    }
}
pub(crate) unsafe extern "C" fn UpdatePartyToFieldOrder() {
    unsafe {
        let mut partyBuffer: *mut u8 = Alloc(600u32);
        let mut i: u8 = 0u8;
        crate::c::memcpy(partyBuffer, (&raw mut gPlayerParty).cast::<u8>(), 600u32);
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 6i32) {
                    break 'l1;
                }
                'l2: {
                    crate::c::memcpy(
                        ((&raw mut gPlayerParty).cast::<u8>())
                            .wrapping_offset(((GetPartyIdFromBattleSlot(i)) as i32) as isize * 100),
                        (partyBuffer).wrapping_offset(((i) as i32) as isize * 100),
                        100u32,
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        Free(partyBuffer);
    }
}
pub(crate) unsafe extern "C" fn SwitchAliveMonIntoLeadSlot() {
    unsafe {
        let mut i: u8 = 0u8;
        let mut mon: *mut u8 = core::ptr::null_mut();
        let mut partyId: u8 = 0u8;
        {
            i = 1u8;
            'l1: loop {
                if !(((i) as i32) < 6i32) {
                    break 'l1;
                }
                'l2: {
                    mon = ((&raw mut gPlayerParty).cast::<u8>())
                        .wrapping_offset(((GetPartyIdFromBattleSlot(i)) as i32) as isize * 100);
                    if (GetMonData2(mon, 11i32) != 0u32) && (GetMonData2(mon, 57i32) != 0u32) {
                        partyId = GetPartyIdFromBattleSlot(0u8);
                        SwitchPartyMonSlots(0u8, i);
                        SwapPartyPokemon(
                            ((&raw mut gPlayerParty).cast::<u8>())
                                .wrapping_offset(((partyId) as i32) as isize * 100),
                            mon,
                        );
                        break 'l1;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn CB2_SetUpExitToBattleScreen() {
    unsafe {
        SetMainCallback2(Some(CB2_SetUpReshowBattleScreenAfterMenu));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ShowPartyMenuToShowcaseMultiBattleParty() {
    unsafe {
        InitPartyMenu(
            5u8,
            3u8,
            0u8,
            0u8,
            127u8,
            Some(Task_InitMultiPartnerPartySlideIn),
            (((&raw mut gMain).cast::<u8>())
                .wrapping_add(8)
                .cast::<Option<unsafe extern "C" fn()>>())
            .read(),
        );
    }
}
pub(crate) unsafe extern "C" fn Task_InitMultiPartnerPartySlideIn(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .write(256i16);
        SlideMultiPartyMenuBoxSpritesOneStep(taskId);
        ChangeBgX(2u8, 65536i32, 0u8);
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Task_MultiPartnerPartySlideIn));
    }
}
pub(crate) unsafe extern "C" fn Task_MultiPartnerPartySlideIn(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        let mut i: u8 = 0u8;
        if !((crate::c::bf_read(
            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
            7,
            1,
            false,
        ) as u16)
            != 0)
        {
            (data).write((((((data).read()) as i32).wrapping_sub(8i32)) as i16));
            SlideMultiPartyMenuBoxSpritesOneStep(taskId);
            if (((data).read()) as i32) == 0i32 {
                {
                    i = ((crate::c::div_i32(6i32, 2i32)) as u8);
                    'l1: loop {
                        if !(((i) as i32) < 6i32) {
                            break 'l1;
                        }
                        'l2: {
                            if ((((((&raw mut gMultiPartnerParty).cast::<u8>()).wrapping_offset(
                                (((i) as i32).wrapping_sub(crate::c::div_i32(6i32, 2i32))) as isize
                                    * 32,
                            ))
                            .cast::<u16>())
                            .read()) as i32)
                                != 0i32
                            {
                                AnimateSelectedPartyIcon(
                                    (((((&raw mut sPartyMenuBoxes)
                                        .cast::<u8>()
                                        .cast::<*mut u8>())
                                    .read())
                                    .wrapping_offset(((i) as i32) as isize * 16))
                                    .wrapping_add(9))
                                    .read(),
                                    0u8,
                                );
                            }
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                PlaySE(120u16);
                ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
                .write(Some(Task_WaitAfterMultiPartnerPartySlideIn));
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_WaitAfterMultiPartnerPartySlideIn(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        if (({
            let __t1 = ((data).read()).wrapping_add(1);
            (data).write(__t1);
            __t1
        }) as i32)
            == 256i32
        {
            Task_ClosePartyMenu(taskId);
        }
    }
}
pub(crate) unsafe extern "C" fn MoveMultiPartyMenuBoxSprite(spriteId: u8, x: i16) {
    unsafe {
        let mut spriteId = spriteId;
        let mut x = x;
        if ((x) as i32) >= 0i32 {
            ((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(36)
            .cast::<i16>())
            .write(x);
        }
    }
}
pub(crate) unsafe extern "C" fn SlideMultiPartyMenuBoxSpritesOneStep(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        let mut i: u8 = 0u8;
        {
            i = ((crate::c::div_i32(6i32, 2i32)) as u8);
            'l1: loop {
                if !(((i) as i32) < 6i32) {
                    break 'l1;
                }
                'l2: {
                    if ((((((&raw mut gMultiPartnerParty).cast::<u8>()).wrapping_offset(
                        (((i) as i32).wrapping_sub(crate::c::div_i32(6i32, 2i32))) as isize * 32,
                    ))
                    .cast::<u16>())
                    .read()) as i32)
                        != 0i32
                    {
                        MoveMultiPartyMenuBoxSprite(
                            (((((&raw mut sPartyMenuBoxes).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_offset(((i) as i32) as isize * 16))
                            .wrapping_add(9))
                            .read(),
                            (((((data).read()) as i32).wrapping_sub(8i32)) as i16),
                        );
                        MoveMultiPartyMenuBoxSprite(
                            (((((&raw mut sPartyMenuBoxes).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_offset(((i) as i32) as isize * 16))
                            .wrapping_add(10))
                            .read(),
                            (((((data).read()) as i32).wrapping_sub(8i32)) as i16),
                        );
                        MoveMultiPartyMenuBoxSprite(
                            (((((&raw mut sPartyMenuBoxes).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_offset(((i) as i32) as isize * 16))
                            .wrapping_add(11))
                            .read(),
                            (((((data).read()) as i32).wrapping_sub(8i32)) as i16),
                        );
                        MoveMultiPartyMenuBoxSprite(
                            (((((&raw mut sPartyMenuBoxes).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_offset(((i) as i32) as isize * 16))
                            .wrapping_add(12))
                            .read(),
                            (((((data).read()) as i32).wrapping_sub(8i32)) as i16),
                        );
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        ChangeBgX(2u8, 2048i32, 1u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ChooseMonForDaycare() {
    unsafe {
        InitPartyMenu(
            6u8,
            0u8,
            0u8,
            0u8,
            15u8,
            Some(Task_HandleChooseMonInput),
            Some(BufferMonSelection),
        );
    }
}
pub(crate) unsafe extern "C" fn ChoosePartyMonByMenuType(menuType: u8) {
    unsafe {
        let mut menuType = menuType;
        ((&raw mut gFieldCallback2).cast::<Option<unsafe extern "C" fn() -> u8>>())
            .write(Some(CB2_FadeFromPartyMenu));
        InitPartyMenu(
            menuType,
            0u8,
            11u8,
            0u8,
            0u8,
            Some(Task_HandleChooseMonInput),
            Some(CB2_ReturnToField),
        );
    }
}
pub(crate) unsafe extern "C" fn BufferMonSelection() {
    unsafe {
        ((&raw mut gSpecialVar_0x8004).cast::<u16>()).write(((GetCursorSelectionMonId()) as u16));
        if ((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32) >= 6i32 {
            ((&raw mut gSpecialVar_0x8004).cast::<u16>()).write(255u16);
        }
        ((&raw mut gFieldCallback2).cast::<Option<unsafe extern "C" fn() -> u8>>())
            .write(Some(CB2_FadeFromPartyMenu));
        SetMainCallback2(Some(CB2_ReturnToField));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CB2_FadeFromPartyMenu() -> u8 {
    unsafe {
        FadeInFromBlack();
        CreateTask(Some(Task_PartyMenuWaitForFade), 10u8);
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn Task_PartyMenuWaitForFade(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if (IsWeatherNotFadingIn()) != 0 {
            DestroyTask(taskId);
            UnlockPlayerFieldControls();
            ScriptContext_Enable();
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ChooseContestMon() {
    unsafe {
        LockPlayerFieldControls();
        FadeScreen(1u8, 0i8);
        CreateTask(Some(Task_ChooseContestMon), 10u8);
    }
}
pub(crate) unsafe extern "C" fn Task_ChooseContestMon(taskId: u8) {
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
            CleanupOverworldWindowsAndTilemaps();
            InitPartyMenu(
                2u8,
                0u8,
                11u8,
                0u8,
                0u8,
                Some(Task_HandleChooseMonInput),
                Some(CB2_ChooseContestMon),
            );
            DestroyTask(taskId);
        }
    }
}
pub(crate) unsafe extern "C" fn CB2_ChooseContestMon() {
    unsafe {
        ((&raw mut gContestMonPartyIndex).cast::<u8>()).write(GetCursorSelectionMonId());
        if ((((&raw mut gContestMonPartyIndex).cast::<u8>()).read()) as i32) >= 6i32 {
            ((&raw mut gContestMonPartyIndex).cast::<u8>()).write(255u8);
        }
        ((&raw mut gSpecialVar_0x8004).cast::<u16>())
            .write(((((&raw mut gContestMonPartyIndex).cast::<u8>()).read()) as u16));
        ((&raw mut gFieldCallback2).cast::<Option<unsafe extern "C" fn() -> u8>>())
            .write(Some(CB2_FadeFromPartyMenu));
        SetMainCallback2(Some(CB2_ReturnToField));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ChoosePartyMon() {
    unsafe {
        LockPlayerFieldControls();
        FadeScreen(1u8, 0i8);
        CreateTask(Some(Task_ChoosePartyMon), 10u8);
    }
}
pub(crate) unsafe extern "C" fn Task_ChoosePartyMon(taskId: u8) {
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
            CleanupOverworldWindowsAndTilemaps();
            InitPartyMenu(
                3u8,
                0u8,
                11u8,
                0u8,
                0u8,
                Some(Task_HandleChooseMonInput),
                Some(BufferMonSelection),
            );
            DestroyTask(taskId);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ChooseMonForMoveRelearner() {
    unsafe {
        LockPlayerFieldControls();
        FadeScreen(1u8, 0i8);
        CreateTask(Some(Task_ChooseMonForMoveRelearner), 10u8);
    }
}
pub(crate) unsafe extern "C" fn Task_ChooseMonForMoveRelearner(taskId: u8) {
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
            CleanupOverworldWindowsAndTilemaps();
            InitPartyMenu(
                7u8,
                0u8,
                11u8,
                0u8,
                0u8,
                Some(Task_HandleChooseMonInput),
                Some(CB2_ChooseMonForMoveRelearner),
            );
            DestroyTask(taskId);
        }
    }
}
pub(crate) unsafe extern "C" fn CB2_ChooseMonForMoveRelearner() {
    unsafe {
        ((&raw mut gSpecialVar_0x8004).cast::<u16>()).write(((GetCursorSelectionMonId()) as u16));
        if ((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32) >= 6i32 {
            ((&raw mut gSpecialVar_0x8004).cast::<u16>()).write(255u16);
        } else {
            ((&raw mut gSpecialVar_0x8005).cast::<u16>()).write(
                ((GetNumberOfRelearnableMoves(
                    ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                        ((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32) as isize
                            * 100,
                    ),
                )) as u16),
            );
        }
        ((&raw mut gFieldCallback2).cast::<Option<unsafe extern "C" fn() -> u8>>())
            .write(Some(CB2_FadeFromPartyMenu));
        SetMainCallback2(Some(CB2_ReturnToField));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DoBattlePyramidMonsHaveHeldItem() {
    unsafe {
        let mut i: u8 = 0u8;
        ((&raw mut gSpecialVar_Result).cast::<u16>()).write(0u16);
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 3i32) {
                    break 'l1;
                }
                'l2: {
                    if GetMonData2(
                        ((&raw mut gPlayerParty).cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 100),
                        12i32,
                    ) != 0u32
                    {
                        ((&raw mut gSpecialVar_Result).cast::<u16>()).write(1u16);
                        break 'l1;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BattlePyramidChooseMonHeldItems() {
    unsafe {
        LockPlayerFieldControls();
        FadeScreen(1u8, 0i8);
        CreateTask(Some(Task_BattlePyramidChooseMonHeldItems), 10u8);
    }
}
pub(crate) unsafe extern "C" fn Task_BattlePyramidChooseMonHeldItems(taskId: u8) {
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
            CleanupOverworldWindowsAndTilemaps();
            InitPartyMenu(
                12u8,
                0u8,
                0u8,
                0u8,
                0u8,
                Some(Task_HandleChooseMonInput),
                Some(BufferMonSelection),
            );
            DestroyTask(taskId);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MoveDeleterChooseMoveToForget() {
    unsafe {
        ShowPokemonSummaryScreen(
            3u8,
            (&raw mut gPlayerParty).cast::<u8>(),
            ((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as u8),
            ((((((&raw mut gPlayerPartyCount).cast::<u8>()).read()) as i32).wrapping_sub(1i32))
                as u8),
            Some(CB2_ReturnToField),
        );
        ((&raw mut gFieldCallback).cast::<Option<unsafe extern "C" fn()>>())
            .write(Some(FieldCB_ContinueScriptHandleMusic));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetNumMovesSelectedMonHas() {
    unsafe {
        let mut i: u8 = 0u8;
        ((&raw mut gSpecialVar_Result).cast::<u16>()).write(0u16);
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 4i32) {
                    break 'l1;
                }
                'l2: {
                    if GetMonData2(
                        ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                            ((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32)
                                as isize
                                * 100,
                        ),
                        (13i32).wrapping_add(((i) as i32)),
                    ) != 0u32
                    {
                        let __p1 = (&raw mut gSpecialVar_Result).cast::<u16>();
                        (__p1).write(((__p1).read()).wrapping_add(1));
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BufferMoveDeleterNicknameAndMove() {
    unsafe {
        let mut mon: *mut u8 = ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
            ((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32) as isize * 100,
        );
        let mut r#move: u16 = ((GetMonData2(
            mon,
            (13i32).wrapping_add(((((&raw mut gSpecialVar_0x8005).cast::<u16>()).read()) as i32)),
        )) as u16);
        GetMonNickname(mon, (&raw mut gStringVar1).cast::<u8>());
        StringCopy(
            (&raw mut gStringVar2).cast::<u8>(),
            (((&raw mut gMoveNames).cast::<u8>()).wrapping_offset(((r#move) as i32) as isize * 13))
                .cast::<u8>(),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MoveDeleterForgetMove() {
    unsafe {
        let mut i: u16 = 0u16;
        SetMonMoveSlot(
            ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                ((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32) as isize * 100,
            ),
            0u16,
            ((((&raw mut gSpecialVar_0x8005).cast::<u16>()).read()) as u8),
        );
        RemoveMonPPBonus(
            ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                ((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32) as isize * 100,
            ),
            ((((&raw mut gSpecialVar_0x8005).cast::<u16>()).read()) as u8),
        );
        {
            i = ((&raw mut gSpecialVar_0x8005).cast::<u16>()).read();
            'l1: loop {
                if !(((i) as i32) < 3i32) {
                    break 'l1;
                }
                'l2: {
                    ShiftMoveSlot(
                        ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                            ((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32)
                                as isize
                                * 100,
                        ),
                        ((i) as u8),
                        ((((i) as i32).wrapping_add(1i32)) as u8),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn ShiftMoveSlot(mon: *mut u8, slotTo: u8, slotFrom: u8) {
    unsafe {
        let mut mon = mon;
        let mut slotTo = slotTo;
        let mut slotFrom = slotFrom;
        let mut move1: u16 = ((GetMonData2(mon, (13i32).wrapping_add(((slotTo) as i32)))) as u16);
        let mut move0: u16 = ((GetMonData2(mon, (13i32).wrapping_add(((slotFrom) as i32)))) as u16);
        let mut pp1: u8 = ((GetMonData2(mon, (17i32).wrapping_add(((slotTo) as i32)))) as u8);
        let mut pp0: u8 = ((GetMonData2(mon, (17i32).wrapping_add(((slotFrom) as i32)))) as u8);
        let mut ppBonuses: u8 = ((GetMonData2(mon, 21i32)) as u8);
        let mut ppBonusMask1: u8 = (((&raw mut gPPUpGetMask).cast::<u8>())
            .wrapping_offset(((slotTo) as i32) as isize))
        .read();
        let mut ppBonusMove1: u8 = ((crate::c::shr_i32(
            (((ppBonuses) as i32) & ((ppBonusMask1) as i32)),
            ((((slotTo) as i32).wrapping_mul(2i32)) as u32),
        )) as u8);
        let mut ppBonusMask2: u8 = (((&raw mut gPPUpGetMask).cast::<u8>())
            .wrapping_offset(((slotFrom) as i32) as isize))
        .read();
        let mut ppBonusMove2: u8 = ((crate::c::shr_i32(
            (((ppBonuses) as i32) & ((ppBonusMask2) as i32)),
            ((((slotFrom) as i32).wrapping_mul(2i32)) as u32),
        )) as u8);
        ppBonuses = ((((ppBonuses) as i32) & !((ppBonusMask1) as i32)) as u8);
        ppBonuses = ((((ppBonuses) as i32) & !((ppBonusMask2) as i32)) as u8);
        ppBonuses = ((((ppBonuses) as i32)
            | (crate::c::shl_i32(
                ((ppBonusMove1) as i32),
                ((((slotFrom) as i32).wrapping_mul(2i32)) as u32),
            ))
            .wrapping_add(crate::c::shl_i32(
                ((ppBonusMove2) as i32),
                ((((slotTo) as i32).wrapping_mul(2i32)) as u32),
            ))) as u8);
        SetMonData(
            mon,
            (13i32).wrapping_add(((slotTo) as i32)),
            (&raw mut move0).cast::<u8>(),
        );
        SetMonData(
            mon,
            (13i32).wrapping_add(((slotFrom) as i32)),
            (&raw mut move1).cast::<u8>(),
        );
        SetMonData(mon, (17i32).wrapping_add(((slotTo) as i32)), &raw mut pp0);
        SetMonData(mon, (17i32).wrapping_add(((slotFrom) as i32)), &raw mut pp1);
        SetMonData(mon, 21i32, &raw mut ppBonuses);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsSelectedMonEgg() {
    unsafe {
        if (GetMonData2(
            ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                ((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32) as isize * 100,
            ),
            45i32,
        )) != 0
        {
            ((&raw mut gSpecialVar_Result).cast::<u16>()).write(1u16);
        } else {
            ((&raw mut gSpecialVar_Result).cast::<u16>()).write(0u16);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsLastMonThatKnowsSurf() {
    unsafe {
        let mut r#move: u16 = 0u16;
        let mut i: u32 = 0u32;
        let mut j: u32 = 0u32;
        ((&raw mut gSpecialVar_Result).cast::<u16>()).write(0u16);
        r#move = ((GetMonData2(
            ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                ((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32) as isize * 100,
            ),
            (13i32).wrapping_add(((((&raw mut gSpecialVar_0x8005).cast::<u16>()).read()) as i32)),
        )) as u16);
        if ((r#move) as i32) == 57i32 {
            {
                i = 0u32;
                'l1: loop {
                    if !(i < ((CalculatePlayerPartyCount()) as u32)) {
                        break 'l1;
                    }
                    'l2: {
                        if i != ((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as u32) {
                            {
                                j = 0u32;
                                'l3: loop {
                                    if !(j < 4u32) {
                                        break 'l3;
                                    }
                                    'l4: {
                                        if GetMonData2(
                                            ((&raw mut gPlayerParty).cast::<u8>())
                                                .wrapping_offset(((i) as i32) as isize * 100),
                                            (((13u32).wrapping_add(j)) as i32),
                                        ) == 57u32
                                        {
                                            return;
                                        }
                                    }
                                    j = (j).wrapping_add(1);
                                }
                            }
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
            if AnyStorageMonWithMove(r#move) != 1u32 {
                ((&raw mut gSpecialVar_Result).cast::<u16>()).write(1u16);
            }
        }
    }
}
