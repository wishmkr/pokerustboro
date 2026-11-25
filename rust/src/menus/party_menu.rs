//! Translated from `src/party_menu.c` by tools/rustport/c2rs.py.
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
    clippy::manual_swap,
    clippy::missing_transmute_annotations,
    clippy::too_many_arguments,
    clippy::type_complexity,
    clippy::unnecessary_cast,
    clippy::useless_transmute,
    dead_code,
    unused_assignments,
    unused_variables
)]

use crate::agb_main::SetVBlankCallback;
use crate::agb_main::gMain;
use crate::battle_anim_mons::{GetBattlerAtPosition, GetBattlerSide, IsDoubleBattle};
use crate::battle_controller_player::CB2_SetUpReshowBattleScreenAfterMenu;
use crate::battle_gfx_sfx_util::HandleBattleLowHpMusicChange;
use crate::battle_interface::{GetHPBarLevel, GetScaledHPFraction};
use crate::battle_main::gBattlerPartyIndexes;
use crate::battle_main::{
    gBattleStruct, gBattleTypeFlags, gBattlerInMenuId, gBattlersCount, gMoveToLearn,
    gMultiPartnerParty,
};
use crate::battle_pike::InBattlePike;
use crate::battle_pyramid::CurrentBattlePyramidLocation;
use crate::battle_pyramid_bag::{
    CB2_ReturnToPyramidBagMenu, GoToBattlePyramidBagMenu, gPyramidBagMenuState,
};
use crate::bg::{
    ChangeBgX, FillBgTilemapBufferRect, FillBgTilemapBufferRect_Palette0,
    IsDma3ManagerBusyWithBgCopy, ResetBgsAndClearDma3BusyFlags, ShowBg,
};
#[allow(unused_imports)]
use crate::c::*;
#[allow(unused_imports)]
use crate::consts::*;
use crate::contest::{GetContestEntryEligibility, gContestMonPartyIndex};
use crate::easy_chat::DoEasyChatScreen;
use crate::event_data::{FlagGet, VarGet};
use crate::evolution_scene::{BeginEvolutionScene, gCB2_AfterEvolution};
use crate::ffi::{gSpecialVar_0x8004, gSpecialVar_0x8005, gSpecialVar_Result};
use crate::field_control_avatar::TrySetDiveWarp;
use crate::field_effect::{FieldEffectStart, gFieldEffectArguments};
use crate::field_player_avatar::{
    GetXYCoordsOneStepInFrontOfPlayer, IsPlayerFacingSurfableFishableWater, IsPlayerSurfingNorth,
    PartyHasMonWithSurf, TestPlayerAvatarFlags,
};
use crate::field_screen_effect::{FadeInFromBlack, FieldCB_ContinueScriptHandleMusic};
use crate::field_specials::InMultiPartnerRoom;
use crate::field_weather::{FadeScreen, IsWeatherNotFadingIn};
use crate::fieldmap::{MapGridGetMetatileBehaviorAt, gMapHeader};
use crate::fldeff_softboiled::{ChooseMonForSoftboiled, Task_TryUseSoftboiledOnPartyMon};
use crate::gpu_regs::SetGpuReg;
use crate::international_string_util::ConvertInternationalPlayerName;
use crate::item::{
    AddBagItem, AddPCItem, CopyItemName, GetPocketByItemId, RemoveBagItem, RemovePCItem,
};
use crate::item_menu::{GoToBagMenu, gSpecialVar_ItemId};
use crate::item_use::CheckIfItemIsTMHMOrEvolutionStone;
use crate::link_rfu_2::GetHostRfuGameData;
use crate::load_save::gSaveBlock1Ptr;
use crate::mail::ReadMail;
use crate::mail_data::ItemIsMail;
use crate::menu::{
    AddTextPrinterParameterized2, AddTextPrinterParameterized3, AddTextPrinterParameterized4,
    ClearScheduledBgCopiesToVram, ClearStdWindowAndFrameToTransparent, CopyToBufferFromBgTilemap,
    CreateYesNoMenu, DoScheduledBgTilemapCopiesToVram, DrawStdFrameWithCustomTileAndPalette,
    GetPlayerTextSpeedDelay, InitMenuInUpperLeftCorner, InitMenuInUpperLeftCornerNormal,
    Menu_GetCursorPos, Menu_ProcessInput, Menu_ProcessInputNoWrapAround_other,
    Menu_ProcessInputNoWrapClearOnChoose, ProcessMenuInput_other, ScheduleBgCopyTilemapToVram,
    SetBgTilemapPalette, SetWindowTemplateFields, malloc_and_decompress,
};
use crate::menu_helpers::{
    GetLRKeysPressedAndHeld, MenuHelpers_IsLinkActive, MenuHelpers_ShouldWaitForLinkRecv,
    ResetAllBgsCoordinates, ResetVramOamAndBgCntRegs, RunTextPrintersRetIsActive,
    SetVBlankHBlankCallbacksToNull,
};
use crate::menu_specialized::{DrawLevelUpWindowPg1, DrawLevelUpWindowPg2};
use crate::metatile_behavior::MetatileBehavior_IsWaterfall;
use crate::overworld::{
    CB2_ReturnToField, CB2_ReturnToFieldContinueScriptPlayMapMusic, CB2_ReturnToFieldWithOpenMenu,
    CleanupOverworldWindowsAndTilemaps, Overworld_GetMapHeaderByGroupAndId,
    Overworld_MapTypeAllowsTeleportAndFly, gFieldCallback, gFieldCallback2,
};
use crate::palette::gPlttBufferUnfaded;
use crate::palette::{
    BeginNormalPaletteFade, BlendPalettes, LoadCompressedPalette, LoadPalette, ResetPaletteFade,
    TransferPlttBuffer, UpdatePaletteFade, gPaletteFade,
};
use crate::player_pc::{Mailbox_ReturnToMailListAfterDeposit, gPlayerPCItemPageInfo};
use crate::pokemon::{
    AdjustFriendship, CalculatePlayerPartyCount, CanMonLearnTMHM, CheckPartyPokerus,
    ExecuteTableBasedItemEffect, GetEvolutionTargetSpecies, GetMonData2, GetMonData3, GetMonGender,
    GetNumberOfRelearnableMoves, GetPlayerFlankId, GetTrainerPartnerName, GiveMoveToMon,
    MonTryLearningNewMove, RemoveMonPPBonus, SetMonData, SetMonMoveSlot,
    SetMonPreventsSwitchingString, gEnemyParty, gPlayerParty, gPlayerPartyCount,
};
use crate::pokemon_icon::{
    CreateMonIcon, LoadMonIconPalettes, SetPartyHPBarSprite, SpriteCB_MonIcon, UpdateMonIconFrame,
};
use crate::pokemon_jump::IsSpeciesAllowedInPokemonJump;
use crate::pokemon_storage_system::AnyStorageMonWithMove;
use crate::pokemon_summary_screen::{
    GetMoveSlotToReplace, ShowPokemonSummaryScreen, ShowSelectMovePokemonSummaryScreen,
    gLastViewedMonIndex,
};
use crate::region_map::{CB2_OpenFlyMap, GetMapNameGeneric};
use crate::reshow_battle_screen::ReshowBattleScreenDummy;
use crate::scanline_effect::ScanlineEffect_Stop;
use crate::script::{LockPlayerFieldControls, ScriptContext_Enable, UnlockPlayerFieldControls};
use crate::sound::{
    IsFanfareTaskInactive, PlayFanfare, PlayFanfareByFanfareNum, PlaySE, WaitFanfare,
};
use crate::sprite::gSprites;
use crate::sprite::{
    AnimateSprites, BuildOamBuffer, FreeAllSpritePalettes, LoadOam, ProcessSpriteCopyRequests,
    ResetSpriteData,
};
use crate::start_menu::AppendToList;
use crate::string_util::StringGet_Nickname;
use crate::string_util::{gStringVar1, gStringVar2, gStringVar4};
use crate::task::{DestroyTask, ResetTasks, RunTasks, SwitchTaskToFollowupFunc};
use crate::task::{gTasks, task_get, task_set, task_set_func};
use crate::text::{DeactivateAllTextPrinters, GetFontAttribute, GetMenuCursorDimensionByFont};
use crate::text_window::LoadUserWindowBorderGfx;
use crate::trade::{CanRegisterMonForTradingBoard, CanSpinTradeMon, GetUnionRoomTradeMessageId};
#[allow(unused_imports)]
use crate::types::*;
use crate::union_room::{
    InUnionRoom, gRfuPartnerCompatibilityData, gUnionRoomOfferedSpecies, gUnionRoomRequestedMonType,
};
use crate::window::{
    ClearWindowTilemap, CopyWindowToVram, FillWindowPixelBuffer, FillWindowPixelRect,
    FreeAllWindowBuffers, GetWindowAttribute, PutWindowTilemap, RemoveWindow,
};
#[allow(unused_imports)]
use core::ffi::c_void;
#[allow(unused_imports)]
use core::mem::zeroed;
#[allow(unused_imports)]
use core::ptr::null_mut;
/// `AddWindow` with this module's view of its types.
#[inline]
unsafe fn AddWindow(a0: *mut WindowTemplate) -> u16 {
    unsafe { crate::window::AddWindow(a0 as _) }
}
/// `BlitBitmapToWindow` with this module's view of its types.
#[inline]
fn BlitBitmapToWindow(a0: u8, a1: *mut u8, a2: u16, a3: u16, a4: u16, a5: u16) {
    unsafe {
        crate::window::BlitBitmapToWindow(a0, a1 as _, a2, a3, a4, a5);
    }
}
/// `ClearMail` with this module's view of its types.
#[inline]
unsafe fn ClearMail(a0: *mut Mail) {
    unsafe {
        crate::mail_data::ClearMail(a0 as _);
    }
}
/// `ConvertIntToDecimalStringN` with this module's view of its types.
#[inline]
unsafe fn ConvertIntToDecimalStringN(a0: *mut u8, a1: i32, a2: i32, a3: u8) -> *mut u8 {
    unsafe { crate::string_util::ConvertIntToDecimalStringN(a0 as _, a1, a2, a3) as *mut u8 }
}
/// `CopyRectToBgTilemapBufferRect` with this module's view of its types.
#[inline]
fn CopyRectToBgTilemapBufferRect(
    a0: u8,
    a1: *mut c_void,
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
) {
    unsafe {
        crate::bg::CopyRectToBgTilemapBufferRect(
            a0, a1 as _, a2, a3, a4, a5, a6, a7, a8, a9, a10, a11, a12,
        );
    }
}
/// `CopyToBgTilemapBufferRect_ChangePalette` with this module's view of its types.
#[inline]
unsafe fn CopyToBgTilemapBufferRect_ChangePalette(
    a0: u8,
    a1: *mut c_void,
    a2: u8,
    a3: u8,
    a4: u8,
    a5: u8,
    a6: u8,
) {
    unsafe {
        crate::bg::CopyToBgTilemapBufferRect_ChangePalette(a0, a1 as _, a2, a3, a4, a5, a6);
    }
}
/// `CreateSprite` with this module's view of its types.
#[inline]
unsafe fn CreateSprite(a0: *mut SpriteTemplate, a1: i16, a2: i16, a3: u8) -> u8 {
    unsafe { crate::sprite::CreateSprite(a0 as _, a1, a2, a3) }
}
/// `CreateTask` with this module's view of its types.
#[inline]
unsafe fn CreateTask(a0: Option<unsafe fn(u8)>, a1: u8) -> u8 {
    unsafe { crate::task::CreateTask(core::mem::transmute(a0), a1) }
}
/// `Free` with this module's view of its types.
#[inline]
unsafe fn Free(a0: *mut c_void) {
    unsafe {
        crate::malloc::Free(a0 as _);
    }
}
/// `FuncIsActiveTask` with this module's view of its types.
#[inline]
unsafe fn FuncIsActiveTask(a0: Option<unsafe fn(u8)>) -> u8 {
    unsafe { crate::task::FuncIsActiveTask(core::mem::transmute(a0)) }
}
/// `GetStringCenterAlignXOffset` with this module's view of its types.
#[inline]
unsafe fn GetStringCenterAlignXOffset(a0: i32, a1: *mut u8, a2: i32) -> i32 {
    unsafe { crate::international_string_util::GetStringCenterAlignXOffset(a0, a1 as _, a2) }
}
/// `GiveMailToMon` with this module's view of its types.
#[inline]
fn GiveMailToMon(a0: *mut Pokemon, a1: *mut Mail) -> u8 {
    unsafe { crate::mail_data::GiveMailToMon(a0 as _, a1 as _) }
}
/// `GiveMailToMonByItemId` with this module's view of its types.
#[inline]
unsafe fn GiveMailToMonByItemId(a0: *mut Pokemon, a1: u16) -> u8 {
    unsafe { crate::mail_data::GiveMailToMonByItemId(a0 as _, a1) }
}
/// `InitBgsFromTemplates` with this module's view of its types.
#[inline]
fn InitBgsFromTemplates(a0: u8, a1: *mut BgTemplate, a2: u8) {
    unsafe {
        crate::bg::InitBgsFromTemplates(a0, a1 as _, a2);
    }
}
/// `InitWindows` with this module's view of its types.
#[inline]
fn InitWindows(a0: *mut WindowTemplate) -> u16 {
    unsafe { crate::window::InitWindows(a0 as _) }
}
/// `LZDecompressWram` with this module's view of its types.
#[inline]
fn LZDecompressWram(a0: *mut u32, a1: *mut c_void) {
    unsafe {
        crate::decompress::LZDecompressWram(a0 as _, a1 as _);
    }
}
/// `LoadBgTiles` with this module's view of its types.
#[inline]
unsafe fn LoadBgTiles(a0: u8, a1: *mut c_void, a2: u16, a3: u16) -> u16 {
    unsafe { crate::bg::LoadBgTiles(a0, a1 as _, a2, a3) }
}
/// `LoadCompressedSpritePalette` with this module's view of its types.
#[inline]
fn LoadCompressedSpritePalette(a0: *mut CompressedSpritePalette) {
    unsafe {
        crate::decompress::LoadCompressedSpritePalette(a0 as _);
    }
}
/// `LoadCompressedSpriteSheet` with this module's view of its types.
#[inline]
unsafe fn LoadCompressedSpriteSheet(a0: *mut CompressedSpriteSheet) -> u16 {
    unsafe { crate::decompress::LoadCompressedSpriteSheet(a0 as _) }
}
/// `LoadSpritePalette` with this module's view of its types.
#[inline]
unsafe fn LoadSpritePalette(a0: *mut SpritePalette) -> u8 {
    unsafe { crate::sprite::LoadSpritePalette(a0 as _) }
}
/// `LoadSpriteSheet` with this module's view of its types.
#[inline]
unsafe fn LoadSpriteSheet(a0: *mut SpriteSheet) -> u16 {
    unsafe { crate::sprite::LoadSpriteSheet(a0 as _) }
}
/// `SetBgTilemapBuffer` with this module's view of its types.
#[inline]
unsafe fn SetBgTilemapBuffer(a0: u8, a1: *mut c_void) {
    unsafe {
        crate::bg::SetBgTilemapBuffer(a0, a1 as _);
    }
}
/// `SetTaskFuncWithFollowupFunc` with this module's view of its types.
#[inline]
fn SetTaskFuncWithFollowupFunc(a0: u8, a1: Option<unsafe fn(u8)>, a2: Option<unsafe fn(u8)>) {
    unsafe {
        crate::task::SetTaskFuncWithFollowupFunc(
            a0,
            core::mem::transmute(a1),
            core::mem::transmute(a2),
        );
    }
}
/// `StartSpriteAnim` with this module's view of its types.
#[inline]
unsafe fn StartSpriteAnim(a0: *mut Sprite, a1: u8) {
    unsafe {
        crate::sprite::StartSpriteAnim(a0 as _, a1);
    }
}
/// `StringAppend` with this module's view of its types.
#[inline]
unsafe fn StringAppend(a0: *mut u8, a1: *mut u8) -> *mut u8 {
    unsafe { crate::string_util::StringAppend(a0 as _, a1 as _) as *mut u8 }
}
/// `StringCompare` with this module's view of its types.
#[inline]
unsafe fn StringCompare(a0: *mut u8, a1: *mut u8) -> i32 {
    unsafe { crate::string_util::StringCompare(a0 as _, a1 as _) }
}
/// `StringCopy` with this module's view of its types.
#[inline]
unsafe fn StringCopy(a0: *mut u8, a1: *mut u8) -> *mut u8 {
    unsafe { crate::string_util::StringCopy(a0 as _, a1 as _) as *mut u8 }
}
/// `StringExpandPlaceholders` with this module's view of its types.
#[inline]
unsafe fn StringExpandPlaceholders(a0: *mut u8, a1: *mut u8) -> *mut u8 {
    unsafe { crate::string_util::StringExpandPlaceholders(a0 as _, a1 as _) as *mut u8 }
}
/// `TakeMailFromMon` with this module's view of its types.
#[inline]
fn TakeMailFromMon(a0: *mut Pokemon) {
    unsafe {
        crate::mail_data::TakeMailFromMon(a0 as _);
    }
}
/// `TakeMailFromMonAndSave` with this module's view of its types.
#[inline]
fn TakeMailFromMonAndSave(a0: *mut Pokemon) -> u8 {
    unsafe { crate::mail_data::TakeMailFromMonAndSave(a0 as _) }
}
// The C's names for task and sprite data slots.
const tKeepOpen: usize = 0;
const tUsedOnSlot: usize = 0;
const tXPos: usize = 0;
const tHadEffect: usize = 1;
const tLastSlotUsed: usize = 2;
// Data tables (translate with cdata.py): gTutorMoves sTutorLearnsets sPartyMenuBgTemplates sPartyBoxInfoRects sPartyMenuSpriteCoords sConfirmButton_Tilemap sCancelButton_Tilemap sFontColorTable sSinglePartyMenuWindowTemplate sDoublePartyMenuWindowTemplate sMultiPartyMenuWindowTemplate sShowcaseMultiPartyMenuWindowTemplate sCancelButtonWindowTemplate sMultiCancelButtonWindowTemplate sConfirmButtonWindowTemplate sDefaultPartyMsgWindowTemplate sDoWhatWithMonMsgWindowTemplate sDoWhatWithItemMsgWindowTemplate sDoWhatWithMailMsgWindowTemplate sWhichMoveMsgWindowTemplate sAlreadyHoldingOneMsgWindowTemplate sItemGiveTakeWindowTemplate sMailReadTakeWindowTemplate sMoveSelectWindowTemplate sPartyMenuYesNoWindowTemplate sLevelUpStatsWindowTemplate sUnusedWindowTemplate1 sUnusedWindowTemplate2 sSlotTilemap_Main sSlotTilemap_MainNoHP sSlotTilemap_Wide sSlotTilemap_WideNoHP sSlotTilemap_WideEmpty sGenderPalOffsets sHPBarPalOffsets sPartyBoxPalOffsets1 sPartyBoxPalOffsets2 sPartyBoxNoMonPalOffsets sGenderMalePalIds sGenderFemalePalIds sHPBarGreenPalIds sHPBarYellowPalIds sHPBarRedPalIds sPartyBoxEmptySlotPalIds1 sPartyBoxMultiPalIds1 sPartyBoxFaintedPalIds1 sPartyBoxCurrSelectionPalIds1 sPartyBoxCurrSelectionMultiPalIds sPartyBoxCurrSelectionFaintedPalIds sPartyBoxSelectedForActionPalIds1 sPartyBoxEmptySlotPalIds2 sPartyBoxMultiPalIds2 sPartyBoxFaintedPalIds2 sPartyBoxCurrSelectionPalIds2 sPartyBoxSelectedForActionPalIds2 sPartyBoxNoMonPalIds sActionStringTable sDescriptionStringTable sUnusedData sCursorOptions sPartyMenuAction_SummarySwitchCancel sPartyMenuAction_ShiftSummaryCancel sPartyMenuAction_SendOutSummaryCancel sPartyMenuAction_SummaryCancel sPartyMenuAction_EnterSummaryCancel sPartyMenuAction_NoEntrySummaryCancel sPartyMenuAction_StoreSummaryCancel sPartyMenuAction_GiveTakeItemCancel sPartyMenuAction_ReadTakeMailCancel sPartyMenuAction_RegisterSummaryCancel sPartyMenuAction_TradeSummaryCancel1 sPartyMenuAction_TradeSummaryCancel2 sPartyMenuAction_TakeItemTossCancel sPartyMenuActions sPartyMenuActionCounts sFieldMoves sFieldMoveCursorCallbacks sUnionRoomTradeMessages sHeldItemGfx sHeldItemPalette sOamData_HeldItem sSpriteAnim_HeldItem sSpriteAnim_HeldMail sSpriteAnimTable_HeldItem sSpriteSheet_HeldItem sSpritePalette_HeldItem sSpriteTemplate_HeldItem sOamData_MenuPokeball sPokeballAnim_Closed sPokeballAnim_Open sSpriteAnimTable_MenuPokeball sSpriteSheet_MenuPokeball sSpritePalette_MenuPokeball sSpriteTemplate_MenuPokeball sOamData_MenuPokeballSmall sSmallPokeballAnim_Closed sSmallPokeballAnim_Open sSmallPokeballAnim_Blank1 sSmallPokeballAnim_Blank2 sSmallPokeballAnim_Blank3 sSmallPokeballAnim_Blank4 sSpriteAnimTable_MenuPokeballSmall sSpriteSheet_MenuPokeballSmall sSpriteTemplate_MenuPokeballSmall sOamData_StatusCondition sSpriteAnim_StatusPoison sSpriteAnim_StatusParalyzed sSpriteAnim_StatusSleep sSpriteAnim_StatusFrozen sSpriteAnim_StatusBurn sSpriteAnim_StatusPokerus sSpriteAnim_StatusFaint sSpriteAnim_Blank sSpriteTemplate_StatusCondition sSpriteSheet_StatusIcons sSpritePalette_StatusIcons sSpriteTemplate_StatusIcons sMultiBattlePartnersPartyMask sUnused_StatStrings sTMHMMoves

/// `struct PartyMenuInternal`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PartyMenuInternal {
    pub task: Option<unsafe fn(u8)>,
    pub exitCallback: Option<unsafe fn()>,
    bits_8: u32,
    pub windowId: CArray<u8, 3>,
    pub actions: CArray<u8, 8>,
    pub numActions: u8,
    pub palBuffer: CArray<u16, 256>,
    pub data: CArray<i16, 16>,
}

impl PartyMenuInternal {
    #[inline(always)]
    pub fn chooseHalf(&self) -> u32 {
        self.bits_8 & 0x1
    }
    #[inline(always)]
    pub fn set_chooseHalf(&mut self, v: u32) {
        self.bits_8 = (self.bits_8 & !(0x1 << 0)) | (v & 0x1);
    }
    #[inline(always)]
    pub fn lastSelectedSlot(&self) -> u32 {
        (self.bits_8 >> 1) & 0x7
    }
    #[inline(always)]
    pub fn set_lastSelectedSlot(&mut self, v: u32) {
        self.bits_8 = (self.bits_8 & !(0x7 << 1)) | ((v & 0x7) << 1);
    }
    #[inline(always)]
    pub fn spriteIdConfirmPokeball(&self) -> u32 {
        (self.bits_8 >> 4) & 0x7f
    }
    #[inline(always)]
    pub fn set_spriteIdConfirmPokeball(&mut self, v: u32) {
        self.bits_8 = (self.bits_8 & !(0x7f << 4)) | ((v & 0x7f) << 4);
    }
    #[inline(always)]
    pub fn spriteIdCancelPokeball(&self) -> u32 {
        (self.bits_8 >> 11) & 0x7f
    }
    #[inline(always)]
    pub fn set_spriteIdCancelPokeball(&mut self, v: u32) {
        self.bits_8 = (self.bits_8 & !(0x7f << 11)) | ((v & 0x7f) << 11);
    }
    #[inline(always)]
    pub fn messageId(&self) -> u32 {
        (self.bits_8 >> 18) & 0x3fff
    }
    #[inline(always)]
    pub fn set_messageId(&mut self, v: u32) {
        self.bits_8 = (self.bits_8 & !(0x3fff << 18)) | ((v & 0x3fff) << 18);
    }
}

unsafe impl Sync for PartyMenuInternal {}

/// `struct PartyMenuBox`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PartyMenuBox {
    pub infoRects: *mut PartyMenuBoxInfoRects,
    pub spriteCoords: *mut u8,
    pub windowId: u8,
    pub monSpriteId: u8,
    pub itemSpriteId: u8,
    pub pokeballSpriteId: u8,
    pub statusSpriteId: u8,
}

unsafe impl Sync for PartyMenuBox {}

/// `struct PartyMenuBoxInfoRects`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PartyMenuBoxInfoRects {
    pub blitFunc: Option<unsafe fn(u8, u8, u8, u8, u8, u8)>,
    pub dimensions: CArray<u8, 24>,
    pub descTextLeft: u8,
    pub descTextTop: u8,
    pub descTextWidth: u8,
    pub descTextHeight: u8,
}

unsafe impl Sync for PartyMenuBoxInfoRects {}

/// `__typeof__(sCursorOptions[0])`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct sCursorOptions_0_t {
    pub text: *mut u8,
    pub func: Option<unsafe fn(u8)>,
}

unsafe impl Sync for sCursorOptions_0_t {}

/// `__typeof__(sFieldMoveCursorCallbacks[0])`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct sFieldMoveCursorCallbacks_0_t {
    pub fieldMoveFunc: Option<unsafe fn() -> u8>,
    pub msgId: u8,
}

unsafe impl Sync for sFieldMoveCursorCallbacks_0_t {}

#[cfg(target_arch = "arm")]
const _: () = {
    #[allow(unused_imports)]
    use core::mem::{offset_of, size_of};
    assert!(size_of::<PartyMenuInternal>() == 568);
    assert!(offset_of!(PartyMenuInternal, task) == 0);
    assert!(offset_of!(PartyMenuInternal, exitCallback) == 4);
    assert!(offset_of!(PartyMenuInternal, bits_8) == 8);
    assert!(offset_of!(PartyMenuInternal, windowId) == 12);
    assert!(offset_of!(PartyMenuInternal, actions) == 15);
    assert!(offset_of!(PartyMenuInternal, numActions) == 23);
    assert!(offset_of!(PartyMenuInternal, palBuffer) == 24);
    assert!(offset_of!(PartyMenuInternal, data) == 536);
    assert!(size_of::<PartyMenuBox>() == 16);
    assert!(offset_of!(PartyMenuBox, infoRects) == 0);
    assert!(offset_of!(PartyMenuBox, spriteCoords) == 4);
    assert!(offset_of!(PartyMenuBox, windowId) == 8);
    assert!(offset_of!(PartyMenuBox, monSpriteId) == 9);
    assert!(offset_of!(PartyMenuBox, itemSpriteId) == 10);
    assert!(offset_of!(PartyMenuBox, pokeballSpriteId) == 11);
    assert!(offset_of!(PartyMenuBox, statusSpriteId) == 12);
    assert!(size_of::<PartyMenuBoxInfoRects>() == 32);
    assert!(offset_of!(PartyMenuBoxInfoRects, blitFunc) == 0);
    assert!(offset_of!(PartyMenuBoxInfoRects, dimensions) == 4);
    assert!(offset_of!(PartyMenuBoxInfoRects, descTextLeft) == 28);
    assert!(offset_of!(PartyMenuBoxInfoRects, descTextTop) == 29);
    assert!(offset_of!(PartyMenuBoxInfoRects, descTextWidth) == 30);
    assert!(offset_of!(PartyMenuBoxInfoRects, descTextHeight) == 31);
    assert!(size_of::<sCursorOptions_0_t>() == 8);
    assert!(offset_of!(sCursorOptions_0_t, text) == 0);
    assert!(offset_of!(sCursorOptions_0_t, func) == 4);
    assert!(size_of::<sFieldMoveCursorCallbacks_0_t>() == 8);
    assert!(offset_of!(sFieldMoveCursorCallbacks_0_t, fieldMoveFunc) == 0);
    assert!(offset_of!(sFieldMoveCursorCallbacks_0_t, msgId) == 4);
};

const ACTIONS_ENTER: u32 = 4;
const ACTIONS_ITEM: u8 = 8;
const ACTIONS_MAIL: u8 = 9;
const ACTIONS_NONE: u32 = 0;
const ACTIONS_NO_ENTRY: u32 = 5;
const ACTIONS_REGISTER: u32 = 10;
const ACTIONS_SEND_OUT: u8 = 3;
const ACTIONS_SHIFT: u8 = 2;
const ACTIONS_SPIN_TRADE: u32 = 12;
const ACTIONS_STORE: i32 = 6;
const ACTIONS_SUMMARY_ONLY: u32 = 7;
const ACTIONS_SWITCH: u32 = 1;
const ACTIONS_TAKEITEM_TOSS: u32 = 13;
const ACTIONS_TRADE: u32 = 11;
const ALREADY_KNOWS_MOVE: u8 = 2;
const CANNOT_LEARN_MOVE: u8 = 1;
const CANNOT_LEARN_MOVE_IS_EGG: u8 = 3;
const CAN_LEARN_MOVE: u8 = 0;
const FIELD_MOVES_COUNT: u16 = 14;
const FIELD_MOVE_DIG: u8 = 9;
const FIELD_MOVE_FLASH: u8 = 1;
const FIELD_MOVE_FLY: u8 = 5;
const FIELD_MOVE_MILK_DRINK: u8 = 11;
const FIELD_MOVE_SOFT_BOILED: u8 = 12;
const FIELD_MOVE_SURF: u8 = 4;
const FIELD_MOVE_TELEPORT: u8 = 8;
const FIELD_MOVE_WATERFALL: u8 = 7;
const MENU_CANCEL1: u8 = 2;
const MENU_DIR_DOWN: i8 = 1;
const MENU_DIR_LEFT: i8 = -2;
const MENU_DIR_RIGHT: i8 = 2;
const MENU_DIR_UP: i8 = -1;
const MENU_FIELD_MOVES: u8 = 19;
const MENU_ITEM: u8 = 3;
const MENU_MAIL: u8 = 6;
const MENU_SUMMARY: u8 = 0;
const MENU_SWITCH: u8 = 1;
const PARTY_BOX_LEFT_COLUMN: i32 = 0;
const PARTY_BOX_RIGHT_COLUMN: i32 = 1;
const PARTY_PAL_FAINTED: u8 = 2;
const PARTY_PAL_MULTI_ALT: u8 = 8;
const PARTY_PAL_NO_MON: u8 = 64;
const PARTY_PAL_SELECTED: i32 = 1;
const PARTY_PAL_SWITCHING: u8 = 16;
const PARTY_PAL_TO_SOFTBOIL: u8 = 32;
const PARTY_PAL_TO_SWITCH: u8 = 4;
const WIN_MSG: u8 = 6;

static gTutorMoves: Table<CArray<u16, 30>> =
    Table((&raw const crate::data::party_menu::gTutorMoves).cast());
static sActionStringTable: Table<CArray<*mut u8, 27>> =
    Table((&raw const crate::data::party_menu::sActionStringTable).cast());
static sAlreadyHoldingOneMsgWindowTemplate: Table<WindowTemplate> =
    Table((&raw const crate::data::party_menu::sAlreadyHoldingOneMsgWindowTemplate).cast());
static sCancelButtonWindowTemplate: Table<WindowTemplate> =
    Table((&raw const crate::data::party_menu::sCancelButtonWindowTemplate).cast());
static sCancelButton_Tilemap: Table<CArray<u32, 7>> =
    Table((&raw const crate::data::party_menu::sCancelButton_Tilemap).cast());
static sConfirmButtonWindowTemplate: Table<WindowTemplate> =
    Table((&raw const crate::data::party_menu::sConfirmButtonWindowTemplate).cast());
static sConfirmButton_Tilemap: Table<CArray<u32, 7>> =
    Table((&raw const crate::data::party_menu::sConfirmButton_Tilemap).cast());
static sCursorOptions: Table<CArray<sCursorOptions_0_t, 33>> =
    Table((&raw const crate::data::party_menu::sCursorOptions).cast());
static sDefaultPartyMsgWindowTemplate: Table<WindowTemplate> =
    Table((&raw const crate::data::party_menu::sDefaultPartyMsgWindowTemplate).cast());
static sDescriptionStringTable: Table<CArray<*mut u8, 13>> =
    Table((&raw const crate::data::party_menu::sDescriptionStringTable).cast());
static sDoWhatWithItemMsgWindowTemplate: Table<WindowTemplate> =
    Table((&raw const crate::data::party_menu::sDoWhatWithItemMsgWindowTemplate).cast());
static sDoWhatWithMailMsgWindowTemplate: Table<WindowTemplate> =
    Table((&raw const crate::data::party_menu::sDoWhatWithMailMsgWindowTemplate).cast());
static sDoWhatWithMonMsgWindowTemplate: Table<WindowTemplate> =
    Table((&raw const crate::data::party_menu::sDoWhatWithMonMsgWindowTemplate).cast());
static sDoublePartyMenuWindowTemplate: Table<CArray<WindowTemplate, 8>> =
    Table((&raw const crate::data::party_menu::sDoublePartyMenuWindowTemplate).cast());
static sFieldMoveCursorCallbacks: Table<CArray<sFieldMoveCursorCallbacks_0_t, 14>> =
    Table((&raw const crate::data::party_menu::sFieldMoveCursorCallbacks).cast());
static sFieldMoves: Table<CArray<u16, 15>> =
    Table((&raw const crate::data::party_menu::sFieldMoves).cast());
static sFontColorTable: Table<CArray<CArray<u8, 3>, 6>> =
    Table((&raw const crate::data::party_menu::sFontColorTable).cast());
static sGenderFemalePalIds: Table<CArray<u8, 2>> =
    Table((&raw const crate::data::party_menu::sGenderFemalePalIds).cast());
static sGenderMalePalIds: Table<CArray<u8, 2>> =
    Table((&raw const crate::data::party_menu::sGenderMalePalIds).cast());
static sGenderPalOffsets: Table<CArray<u8, 2>> =
    Table((&raw const crate::data::party_menu::sGenderPalOffsets).cast());
static sHPBarGreenPalIds: Table<CArray<u8, 2>> =
    Table((&raw const crate::data::party_menu::sHPBarGreenPalIds).cast());
static sHPBarPalOffsets: Table<CArray<u8, 2>> =
    Table((&raw const crate::data::party_menu::sHPBarPalOffsets).cast());
static sHPBarRedPalIds: Table<CArray<u8, 2>> =
    Table((&raw const crate::data::party_menu::sHPBarRedPalIds).cast());
static sHPBarYellowPalIds: Table<CArray<u8, 2>> =
    Table((&raw const crate::data::party_menu::sHPBarYellowPalIds).cast());
static sItemGiveTakeWindowTemplate: Table<WindowTemplate> =
    Table((&raw const crate::data::party_menu::sItemGiveTakeWindowTemplate).cast());
static sLevelUpStatsWindowTemplate: Table<WindowTemplate> =
    Table((&raw const crate::data::party_menu::sLevelUpStatsWindowTemplate).cast());
static sMailReadTakeWindowTemplate: Table<WindowTemplate> =
    Table((&raw const crate::data::party_menu::sMailReadTakeWindowTemplate).cast());
static sMoveSelectWindowTemplate: Table<WindowTemplate> =
    Table((&raw const crate::data::party_menu::sMoveSelectWindowTemplate).cast());
static sMultiBattlePartnersPartyMask: Table<CArray<u8, 8>> =
    Table((&raw const crate::data::party_menu::sMultiBattlePartnersPartyMask).cast());
static sMultiCancelButtonWindowTemplate: Table<WindowTemplate> =
    Table((&raw const crate::data::party_menu::sMultiCancelButtonWindowTemplate).cast());
static sMultiPartyMenuWindowTemplate: Table<CArray<WindowTemplate, 8>> =
    Table((&raw const crate::data::party_menu::sMultiPartyMenuWindowTemplate).cast());
static sPartyBoxCurrSelectionFaintedPalIds: Table<CArray<u8, 3>> =
    Table((&raw const crate::data::party_menu::sPartyBoxCurrSelectionFaintedPalIds).cast());
static sPartyBoxCurrSelectionMultiPalIds: Table<CArray<u8, 3>> =
    Table((&raw const crate::data::party_menu::sPartyBoxCurrSelectionMultiPalIds).cast());
static sPartyBoxCurrSelectionPalIds1: Table<CArray<u8, 3>> =
    Table((&raw const crate::data::party_menu::sPartyBoxCurrSelectionPalIds1).cast());
static sPartyBoxCurrSelectionPalIds2: Table<CArray<u8, 3>> =
    Table((&raw const crate::data::party_menu::sPartyBoxCurrSelectionPalIds2).cast());
static sPartyBoxEmptySlotPalIds1: Table<CArray<u8, 3>> =
    Table((&raw const crate::data::party_menu::sPartyBoxEmptySlotPalIds1).cast());
static sPartyBoxEmptySlotPalIds2: Table<CArray<u8, 3>> =
    Table((&raw const crate::data::party_menu::sPartyBoxEmptySlotPalIds2).cast());
static sPartyBoxFaintedPalIds1: Table<CArray<u8, 3>> =
    Table((&raw const crate::data::party_menu::sPartyBoxFaintedPalIds1).cast());
static sPartyBoxFaintedPalIds2: Table<CArray<u8, 3>> =
    Table((&raw const crate::data::party_menu::sPartyBoxFaintedPalIds2).cast());
static sPartyBoxInfoRects: Table<CArray<PartyMenuBoxInfoRects, 2>> =
    Table((&raw const crate::data::party_menu::sPartyBoxInfoRects).cast());
static sPartyBoxMultiPalIds1: Table<CArray<u8, 3>> =
    Table((&raw const crate::data::party_menu::sPartyBoxMultiPalIds1).cast());
static sPartyBoxMultiPalIds2: Table<CArray<u8, 3>> =
    Table((&raw const crate::data::party_menu::sPartyBoxMultiPalIds2).cast());
static sPartyBoxNoMonPalIds: Table<CArray<u8, 3>> =
    Table((&raw const crate::data::party_menu::sPartyBoxNoMonPalIds).cast());
static sPartyBoxNoMonPalOffsets: Table<CArray<u8, 3>> =
    Table((&raw const crate::data::party_menu::sPartyBoxNoMonPalOffsets).cast());
static sPartyBoxPalOffsets1: Table<CArray<u8, 3>> =
    Table((&raw const crate::data::party_menu::sPartyBoxPalOffsets1).cast());
static sPartyBoxPalOffsets2: Table<CArray<u8, 3>> =
    Table((&raw const crate::data::party_menu::sPartyBoxPalOffsets2).cast());
static sPartyBoxSelectedForActionPalIds1: Table<CArray<u8, 3>> =
    Table((&raw const crate::data::party_menu::sPartyBoxSelectedForActionPalIds1).cast());
static sPartyBoxSelectedForActionPalIds2: Table<CArray<u8, 3>> =
    Table((&raw const crate::data::party_menu::sPartyBoxSelectedForActionPalIds2).cast());
static sPartyMenuActionCounts: Table<CArray<u8, 14>> =
    Table((&raw const crate::data::party_menu::sPartyMenuActionCounts).cast());
static sPartyMenuActions: Table<CArray<*mut u8, 14>> =
    Table((&raw const crate::data::party_menu::sPartyMenuActions).cast());
static sPartyMenuBgTemplates: Table<CArray<BgTemplate, 3>> =
    Table((&raw const crate::data::party_menu::sPartyMenuBgTemplates).cast());
static sPartyMenuSpriteCoords: Table<CArray<CArray<CArray<u8, 8>, 6>, 4>> =
    Table((&raw const crate::data::party_menu::sPartyMenuSpriteCoords).cast());
static sPartyMenuYesNoWindowTemplate: Table<WindowTemplate> =
    Table((&raw const crate::data::party_menu::sPartyMenuYesNoWindowTemplate).cast());
static sShowcaseMultiPartyMenuWindowTemplate: Table<CArray<WindowTemplate, 7>> =
    Table((&raw const crate::data::party_menu::sShowcaseMultiPartyMenuWindowTemplate).cast());
static sSinglePartyMenuWindowTemplate: Table<CArray<WindowTemplate, 8>> =
    Table((&raw const crate::data::party_menu::sSinglePartyMenuWindowTemplate).cast());
static sSlotTilemap_Main: Table<CArray<u8, 70>> =
    Table((&raw const crate::data::party_menu::sSlotTilemap_Main).cast());
static sSlotTilemap_MainNoHP: Table<CArray<u8, 70>> =
    Table((&raw const crate::data::party_menu::sSlotTilemap_MainNoHP).cast());
static sSlotTilemap_Wide: Table<CArray<u8, 54>> =
    Table((&raw const crate::data::party_menu::sSlotTilemap_Wide).cast());
static sSlotTilemap_WideEmpty: Table<CArray<u8, 54>> =
    Table((&raw const crate::data::party_menu::sSlotTilemap_WideEmpty).cast());
static sSlotTilemap_WideNoHP: Table<CArray<u8, 54>> =
    Table((&raw const crate::data::party_menu::sSlotTilemap_WideNoHP).cast());
static sSpritePalette_HeldItem: Table<SpritePalette> =
    Table((&raw const crate::data::party_menu::sSpritePalette_HeldItem).cast());
static sSpritePalette_MenuPokeball: Table<CompressedSpritePalette> =
    Table((&raw const crate::data::party_menu::sSpritePalette_MenuPokeball).cast());
static sSpritePalette_StatusIcons: Table<CompressedSpritePalette> =
    Table((&raw const crate::data::party_menu::sSpritePalette_StatusIcons).cast());
static sSpriteSheet_HeldItem: Table<SpriteSheet> =
    Table((&raw const crate::data::party_menu::sSpriteSheet_HeldItem).cast());
static sSpriteSheet_MenuPokeball: Table<CompressedSpriteSheet> =
    Table((&raw const crate::data::party_menu::sSpriteSheet_MenuPokeball).cast());
static sSpriteSheet_MenuPokeballSmall: Table<CompressedSpriteSheet> =
    Table((&raw const crate::data::party_menu::sSpriteSheet_MenuPokeballSmall).cast());
static sSpriteSheet_StatusIcons: Table<CompressedSpriteSheet> =
    Table((&raw const crate::data::party_menu::sSpriteSheet_StatusIcons).cast());
static sSpriteTemplate_HeldItem: Table<SpriteTemplate> =
    Table((&raw const crate::data::party_menu::sSpriteTemplate_HeldItem).cast());
static sSpriteTemplate_MenuPokeball: Table<SpriteTemplate> =
    Table((&raw const crate::data::party_menu::sSpriteTemplate_MenuPokeball).cast());
static sSpriteTemplate_MenuPokeballSmall: Table<SpriteTemplate> =
    Table((&raw const crate::data::party_menu::sSpriteTemplate_MenuPokeballSmall).cast());
static sSpriteTemplate_StatusIcons: Table<SpriteTemplate> =
    Table((&raw const crate::data::party_menu::sSpriteTemplate_StatusIcons).cast());
static sTMHMMoves: Table<CArray<u16, 58>> =
    Table((&raw const crate::data::party_menu::sTMHMMoves).cast());
static sTutorLearnsets: Table<CArray<u32, 412>> =
    Table((&raw const crate::data::party_menu::sTutorLearnsets).cast());
static sUnionRoomTradeMessages: Table<CArray<*mut u8, 9>> =
    Table((&raw const crate::data::party_menu::sUnionRoomTradeMessages).cast());
static sWhichMoveMsgWindowTemplate: Table<WindowTemplate> =
    Table((&raw const crate::data::party_menu::sWhichMoveMsgWindowTemplate).cast());

#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub(crate) static sPartyMenuInternal: crate::global::Global<*mut PartyMenuInternal> =
    crate::global::Global::new(null_mut());
#[unsafe(no_mangle)]
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub static gPartyMenu: crate::global::Global<PartyMenu> =
    crate::global::Global::new(unsafe { zeroed() });
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub(crate) static sPartyMenuBoxes: crate::global::Global<*mut PartyMenuBox> =
    crate::global::Global::new(null_mut());
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub(crate) static sPartyBgGfxTilemap: crate::global::Global<*mut u8> =
    crate::global::Global::new(null_mut());
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub(crate) static sPartyBgTilemapBuffer: crate::global::Global<*mut u8> =
    crate::global::Global::new(null_mut());
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub static gPartyMenuUseExitCallback: crate::global::Global<u8> = crate::global::Global::new(0);
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub static gSelectedMonPartyId: crate::global::Global<u8> = crate::global::Global::new(0);
#[unsafe(no_mangle)]
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub static gPostMenuFieldCallback: crate::global::Global<Option<unsafe fn()>> =
    crate::global::Global::new(None);
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub(crate) static sSlot1TilemapBuffer: crate::global::Global<*mut u16> =
    crate::global::Global::new(null_mut());
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub(crate) static sSlot2TilemapBuffer: crate::global::Global<*mut u16> =
    crate::global::Global::new(null_mut());
#[unsafe(no_mangle)]
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub static gSelectedOrderFromParty: crate::global::Global<Aligned<CArray<u8, 4>>> =
    crate::global::Global::new(Aligned(unsafe { zeroed() }));
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub(crate) static sPartyMenuItemId: crate::global::Global<u16> = crate::global::Global::new(0);
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub(crate) static sUnused: crate::global::Global<u16> = crate::global::Global::new(0);
#[unsafe(no_mangle)]
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub static gBattlePartyCurrentOrder: crate::global::Global<Aligned<CArray<u8, 3>>> =
    crate::global::Global::new(Aligned(unsafe { zeroed() }));
#[cfg_attr(target_os = "none", unsafe(link_section = "common_data"))]
pub static gItemUseCB: crate::global::Global<Option<unsafe fn(u8, Option<unsafe fn(u8)>)>> =
    crate::global::Global::new(None);

/// `AddTextPrinterParameterized` with this module's view of its types.
#[inline]
unsafe fn AddTextPrinterParameterized(
    a0: u8,
    a1: u8,
    a2: *mut u8,
    a3: u8,
    a4: u8,
    a5: u8,
    a6: Option<unsafe fn(*mut TextPrinterTemplate, u16)>,
) -> u16 {
    unsafe {
        crate::text::AddTextPrinterParameterized(
            a0,
            a1,
            a2 as _,
            a3,
            a4,
            a5,
            core::mem::transmute(a6),
        )
    }
}
/// `Alloc` with this module's view of its types.
#[inline]
unsafe fn Alloc(a0: u32) -> *mut c_void {
    unsafe { crate::malloc::Alloc(a0) as *mut c_void }
}
/// `AllocZeroed` with this module's view of its types.
#[inline]
unsafe fn AllocZeroed(a0: u32) -> *mut c_void {
    unsafe { crate::malloc::AllocZeroed(a0) as *mut c_void }
}
/// `CpuSet` with this module's view of its types.
#[inline]
fn CpuSet(a0: *mut c_void, a1: *mut c_void, a2: u32) {
    unsafe {
        crate::syscall::CpuSet(a0 as _, a1 as _, a2);
    }
}
/// `GetOverworldTextboxPalettePtr` with this module's view of its types.
#[inline]
fn GetOverworldTextboxPalettePtr() -> *mut u16 {
    unsafe { crate::text_window::GetOverworldTextboxPalettePtr() as *mut u16 }
}
/// `SetMainCallback2` with this module's view of its types.
#[inline]
unsafe fn SetMainCallback2(a0: Option<unsafe fn()>) {
    unsafe {
        crate::agb_main::SetMainCallback2(core::mem::transmute(a0));
    }
}

unsafe fn InitPartyMenu(
    menuType: u8,
    layout: u8,
    partyAction: u8,
    keepCursorPos: u8,
    messageId: u8,
    task: Option<unsafe fn(u8)>,
    callback: Option<unsafe fn()>,
) {
    ResetPartyMenu();
    crate::gl!(sPartyMenuInternal = Alloc(568) as *mut PartyMenuInternal);
    if (*sPartyMenuInternal.as_ptr()).is_null() {
        SetMainCallback2(callback);
    } else {
        (*gPartyMenu.as_ptr()).set_menuType(menuType);
        crate::gl!(gPartyMenu.exitCallback = callback);
        crate::gl!(gPartyMenu.action = partyAction);
        (*crate::gl!(sPartyMenuInternal)).set_messageId(messageId as u32);
        (*crate::gl!(sPartyMenuInternal)).task = task;
        (*crate::gl!(sPartyMenuInternal)).exitCallback = None;
        (*crate::gl!(sPartyMenuInternal)).set_lastSelectedSlot(0);
        (*crate::gl!(sPartyMenuInternal)).set_spriteIdConfirmPokeball(0x7F);
        (*crate::gl!(sPartyMenuInternal)).set_spriteIdCancelPokeball(0x7F);
        if menuType == PARTY_MENU_TYPE_CHOOSE_HALF {
            (*crate::gl!(sPartyMenuInternal)).set_chooseHalf(TRUE as u32);
        } else {
            (*crate::gl!(sPartyMenuInternal)).set_chooseHalf(FALSE as u32);
        }
        if layout != KEEP_PARTY_LAYOUT {
            (*gPartyMenu.as_ptr()).set_layout(layout);
        }
        for i in 0..16u16 {
            (*crate::gl!(sPartyMenuInternal)).data[i] = 0;
        }
        for i in 0..3u16 {
            (*crate::gl!(sPartyMenuInternal)).windowId[i] = WINDOW_NONE;
        }
        if keepCursorPos == 0 {
            crate::gl!(gPartyMenu.slotId = 0);
        } else if crate::gl!(gPartyMenu.slotId) > 5
            || GetMonData2(
                {
                    let gl_i0 = crate::gl!(gPartyMenu.slotId);
                    crate::gl!(&raw gPlayerParty[gl_i0])
                },
                MON_DATA_SPECIES,
            ) == SPECIES_NONE as u32
        {
            crate::gl!(gPartyMenu.slotId = 0);
        }
        (*(crate::gl!(&raw @at crate::text::gTextFlags;).cast_const())
            .cast::<TextFlags>()
            .cast_mut())
        .set_autoScroll(0);
        CalculatePlayerPartyCount();
        SetMainCallback2(Some(CB2_InitPartyMenu));
    }
}
pub(crate) unsafe fn CB2_UpdatePartyMenu() {
    RunTasks();
    AnimateSprites();
    BuildOamBuffer();
    DoScheduledBgTilemapCopiesToVram();
    UpdatePaletteFade();
}
pub(crate) unsafe fn VBlankCB_PartyMenu() {
    LoadOam();
    ProcessSpriteCopyRequests();
    TransferPlttBuffer();
}
pub(crate) unsafe fn CB2_InitPartyMenu() {
    loop {
        if MenuHelpers_ShouldWaitForLinkRecv() == TRUE
            || ShowPartyMenu() == TRUE
            || MenuHelpers_IsLinkActive() == TRUE
        {
            break;
        }
    }
}
pub(crate) unsafe fn ShowPartyMenu() -> u8 {
    match crate::gl!(gMain.state) {
        0 => {
            SetVBlankHBlankCallbacksToNull();
            ResetVramOamAndBgCntRegs();
            ClearScheduledBgCopiesToVram();
            crate::gl!(gMain.state += 1);
        }
        1 => {
            ScanlineEffect_Stop();
            crate::gl!(gMain.state += 1);
        }
        2 => {
            ResetPaletteFade();
            (*gPaletteFade.as_ptr()).set_bufferTransferDisabled(TRUE as u16);
            crate::gl!(gMain.state += 1);
        }
        3 => {
            ResetSpriteData();
            crate::gl!(gMain.state += 1);
        }
        4 => {
            FreeAllSpritePalettes();
            crate::gl!(gMain.state += 1);
        }
        5 => {
            if MenuHelpers_IsLinkActive() == 0 {
                ResetTasks();
            }
            crate::gl!(gMain.state += 1);
        }
        6 => {
            SetPartyMonsAllowedInMinigame();
            crate::gl!(gMain.state += 1);
        }
        7 => {
            if AllocPartyMenuBg() == 0 {
                ExitPartyMenu();
                return TRUE;
            } else {
                (*crate::gl!(sPartyMenuInternal)).data[0] = 0;
                crate::gl!(gMain.state += 1);
            }
        }
        8 => {
            if AllocPartyMenuBgGfx() != 0 {
                crate::gl!(gMain.state += 1);
            }
        }
        9 => {
            InitPartyMenuWindows((*gPartyMenu.as_ptr()).layout());
            crate::gl!(gMain.state += 1);
        }
        10 => {
            InitPartyMenuBoxes((*gPartyMenu.as_ptr()).layout());
            (*crate::gl!(sPartyMenuInternal)).data[0] = 0;
            crate::gl!(gMain.state += 1);
        }
        11 => {
            LoadHeldItemIcons();
            crate::gl!(gMain.state += 1);
        }
        12 => {
            LoadPartyMenuPokeballGfx();
            crate::gl!(gMain.state += 1);
        }
        13 => {
            LoadPartyMenuAilmentGfx();
            crate::gl!(gMain.state += 1);
        }
        14 => {
            LoadMonIconPalettes();
            crate::gl!(gMain.state += 1);
        }
        15 => {
            if CreatePartyMonSpritesLoop() != 0 {
                (*crate::gl!(sPartyMenuInternal)).data[0] = 0;
                crate::gl!(gMain.state += 1);
            }
        }
        16 => {
            if RenderPartyMenuBoxes() != 0 {
                (*crate::gl!(sPartyMenuInternal)).data[0] = 0;
                crate::gl!(gMain.state += 1);
            }
        }
        17 => {
            CreateCancelConfirmPokeballSprites();
            crate::gl!(gMain.state += 1);
        }
        18 => {
            CreateCancelConfirmWindows((*crate::gl!(sPartyMenuInternal)).chooseHalf() as u8);
            crate::gl!(gMain.state += 1);
        }
        19 => {
            crate::gl!(gMain.state += 1);
        }
        20 => {
            CreateTask((*crate::gl!(sPartyMenuInternal)).task, 0);
            DisplayPartyMenuStdMessage((*crate::gl!(sPartyMenuInternal)).messageId());
            crate::gl!(gMain.state += 1);
        }
        21 => {
            BlendPalettes(PALETTES_ALL, 16, 0);
            (*gPaletteFade.as_ptr()).set_bufferTransferDisabled(FALSE as u16);
            crate::gl!(gMain.state += 1);
        }
        22 => {
            BeginNormalPaletteFade(PALETTES_ALL, 0, 16, 0, 0);
            crate::gl!(gMain.state += 1);
        }
        _ => {
            SetVBlankCallback(Some(VBlankCB_PartyMenu));
            SetMainCallback2(Some(CB2_UpdatePartyMenu));
            return TRUE;
        }
    }
    FALSE
}
unsafe fn ExitPartyMenu() {
    BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 16, 0);
    CreateTask(Some(Task_ExitPartyMenu), 0);
    SetVBlankCallback(Some(VBlankCB_PartyMenu));
    SetMainCallback2(Some(CB2_UpdatePartyMenu));
}
pub(crate) unsafe fn Task_ExitPartyMenu(taskId: u8) {
    if (*gPaletteFade.as_ptr()).active() == 0 {
        SetMainCallback2(crate::gl!(gPartyMenu.exitCallback));
        FreePartyPointers();
        DestroyTask(taskId);
    }
}
fn ResetPartyMenu() {
    crate::gl!(sPartyMenuInternal = null_mut());
    crate::gl!(sPartyBgTilemapBuffer = null_mut());
    crate::gl!(sPartyMenuBoxes = null_mut());
    crate::gl!(sPartyBgGfxTilemap = null_mut());
}
unsafe fn AllocPartyMenuBg() -> u8 {
    crate::gl!(sPartyBgTilemapBuffer = Alloc(0x800) as *mut u8);
    if (*sPartyBgTilemapBuffer.as_ptr()).is_null() {
        return FALSE;
    }
    memset(crate::gl!(sPartyBgTilemapBuffer), 0, 0x800);
    ResetBgsAndClearDma3BusyFlags(0);
    InitBgsFromTemplates(0, sPartyMenuBgTemplates.as_ptr().cast_mut(), 3);
    SetBgTilemapBuffer(1, crate::gl!(sPartyBgTilemapBuffer) as *mut c_void);
    ResetAllBgsCoordinates();
    ScheduleBgCopyTilemapToVram(1);
    SetGpuReg(REG_OFFSET_DISPCNT, 4160);
    SetGpuReg(REG_OFFSET_BLDCNT, 0);
    ShowBg(0);
    ShowBg(1);
    ShowBg(2);
    TRUE
}
unsafe fn AllocPartyMenuBgGfx() -> u8 {
    let mut sizeout: u32 = 0;
    match (*crate::gl!(sPartyMenuInternal)).data[0] {
        0 => {
            crate::gl!(
                sPartyBgGfxTilemap = malloc_and_decompress(
                    (*(&raw const crate::data::graphics::gPartyMenuBg_Gfx).cast::<CArray<u32, 0>>())
                        .as_ptr()
                        .cast_mut() as *mut c_void,
                    &raw mut sizeout,
                ) as *mut u8
            );
            LoadBgTiles(
                1,
                crate::gl!(sPartyBgGfxTilemap) as *mut c_void,
                sizeout as u16,
                0,
            );
            (*crate::gl!(sPartyMenuInternal)).data[0] += 1;
        }
        1 => {
            if IsDma3ManagerBusyWithBgCopy() == 0 {
                LZDecompressWram(
                    (*(&raw const crate::data::graphics::gPartyMenuBg_Tilemap)
                        .cast::<CArray<u32, 0>>())
                    .as_ptr()
                    .cast_mut(),
                    crate::gl!(sPartyBgTilemapBuffer) as *mut c_void,
                );
                (*crate::gl!(sPartyMenuInternal)).data[0] += 1;
            }
        }
        2 => {
            LoadCompressedPalette(
                (*(&raw const crate::data::graphics::gPartyMenuBg_Pal).cast::<CArray<u32, 0>>())
                    .as_ptr()
                    .cast_mut(),
                0,
                352,
            );
            CpuSet(
                (*gPlttBufferUnfaded.as_ptr()).as_mut_ptr() as *mut c_void,
                (*crate::gl!(sPartyMenuInternal)).palBuffer.as_mut_ptr() as *mut c_void,
                176,
            );
            (*crate::gl!(sPartyMenuInternal)).data[0] += 1;
        }
        3 => {
            PartyPaletteBufferCopy(4);
            (*crate::gl!(sPartyMenuInternal)).data[0] += 1;
        }
        4 => {
            PartyPaletteBufferCopy(5);
            (*crate::gl!(sPartyMenuInternal)).data[0] += 1;
        }
        5 => {
            PartyPaletteBufferCopy(6);
            (*crate::gl!(sPartyMenuInternal)).data[0] += 1;
        }
        6 => {
            PartyPaletteBufferCopy(7);
            (*crate::gl!(sPartyMenuInternal)).data[0] += 1;
        }
        7 => {
            PartyPaletteBufferCopy(8);
            (*crate::gl!(sPartyMenuInternal)).data[0] += 1;
        }
        _ => {
            return TRUE;
        }
    }
    FALSE
}
unsafe fn PartyPaletteBufferCopy(palNum: u8) {
    let offset: u8 = palNum * 16;
    CpuSet(
        &raw mut (*(crate::gl!(&raw @at crate::palette::gPlttBufferUnfaded;).cast_const())
            .cast::<CArray<u16, 512>>()
            .cast_mut())[48] as *mut c_void,
        &raw mut (*(crate::gl!(&raw @at crate::palette::gPlttBufferUnfaded;).cast_const())
            .cast::<CArray<u16, 512>>()
            .cast_mut())[offset] as *mut c_void,
        16,
    );
    CpuSet(
        &raw mut (*(crate::gl!(&raw @at crate::palette::gPlttBufferUnfaded;).cast_const())
            .cast::<CArray<u16, 512>>()
            .cast_mut())[48] as *mut c_void,
        &raw mut (*(crate::gl!(&raw @at crate::palette::gPlttBufferFaded;).cast_const())
            .cast::<CArray<u16, 512>>()
            .cast_mut())[offset] as *mut c_void,
        16,
    );
}
unsafe fn FreePartyPointers() {
    if !(*sPartyMenuInternal.as_ptr()).is_null() {
        Free(crate::gl!(sPartyMenuInternal) as *mut c_void);
    }
    if !(*sPartyBgTilemapBuffer.as_ptr()).is_null() {
        Free(crate::gl!(sPartyBgTilemapBuffer) as *mut c_void);
    }
    if !(*sPartyBgGfxTilemap.as_ptr()).is_null() {
        Free(crate::gl!(sPartyBgGfxTilemap) as *mut c_void);
    }
    if !(*sPartyMenuBoxes.as_ptr()).is_null() {
        Free(crate::gl!(sPartyMenuBoxes) as *mut c_void);
    }
    FreeAllWindowBuffers();
}
unsafe fn InitPartyMenuBoxes(layout: u8) {
    crate::gl!(sPartyMenuBoxes = Alloc(96) as *mut PartyMenuBox);
    for i in 0..(PARTY_SIZE as u8) {
        (*(*sPartyMenuBoxes.as_ptr()).at(i)).infoRects =
            (&raw const sPartyBoxInfoRects[1]).cast_mut();
        (*(*sPartyMenuBoxes.as_ptr()).at(i)).spriteCoords =
            sPartyMenuSpriteCoords[layout][i].as_ptr().cast_mut();
        (*(*sPartyMenuBoxes.as_ptr()).at(i)).windowId = i;
        (*(*sPartyMenuBoxes.as_ptr()).at(i)).monSpriteId = SPRITE_NONE;
        (*(*sPartyMenuBoxes.as_ptr()).at(i)).itemSpriteId = SPRITE_NONE;
        (*(*sPartyMenuBoxes.as_ptr()).at(i)).pokeballSpriteId = SPRITE_NONE;
        (*(*sPartyMenuBoxes.as_ptr()).at(i)).statusSpriteId = SPRITE_NONE;
    }
    (*crate::gl!(sPartyMenuBoxes)).infoRects = (&raw const sPartyBoxInfoRects[0]).cast_mut();
    if layout == PARTY_LAYOUT_MULTI_SHOWCASE {
        (*(*sPartyMenuBoxes.as_ptr()).at(3)).infoRects =
            (&raw const sPartyBoxInfoRects[0]).cast_mut();
    } else if layout != PARTY_LAYOUT_SINGLE {
        (*(*sPartyMenuBoxes.as_ptr()).at(1)).infoRects =
            (&raw const sPartyBoxInfoRects[0]).cast_mut();
    }
}
unsafe fn RenderPartyMenuBox(slot: u8) {
    if (*gPartyMenu.as_ptr()).menuType() == PARTY_MENU_TYPE_MULTI_SHOWCASE
        && slot >= MULTI_PARTY_SIZE as u8
    {
        DisplayPartyPokemonDataForMultiBattle(slot);
        if {
            let gl_i0 = slot as i32 - MULTI_PARTY_SIZE;
            crate::gl!(gMultiPartnerParty[gl_i0].species)
        } == SPECIES_NONE
        {
            LoadPartyBoxPalette((*sPartyMenuBoxes.as_ptr()).at(slot), PARTY_PAL_NO_MON);
        } else {
            LoadPartyBoxPalette((*sPartyMenuBoxes.as_ptr()).at(slot), PARTY_PAL_MULTI_ALT);
        }
        CopyWindowToVram(
            (*(*sPartyMenuBoxes.as_ptr()).at(slot)).windowId,
            COPYWIN_GFX,
        );
        PutWindowTilemap((*(*sPartyMenuBoxes.as_ptr()).at(slot)).windowId);
        ScheduleBgCopyTilemapToVram(2);
    } else {
        if GetMonData2(crate::gl!(&raw gPlayerParty[slot]), MON_DATA_SPECIES) == SPECIES_NONE as u32
        {
            DrawEmptySlot((*(*sPartyMenuBoxes.as_ptr()).at(slot)).windowId);
            LoadPartyBoxPalette((*sPartyMenuBoxes.as_ptr()).at(slot), PARTY_PAL_NO_MON);
            CopyWindowToVram(
                (*(*sPartyMenuBoxes.as_ptr()).at(slot)).windowId,
                COPYWIN_GFX,
            );
        } else {
            if (*gPartyMenu.as_ptr()).menuType() == PARTY_MENU_TYPE_MOVE_RELEARNER {
                DisplayPartyPokemonDataForRelearner(slot);
            } else if (*gPartyMenu.as_ptr()).menuType() == PARTY_MENU_TYPE_CONTEST {
                DisplayPartyPokemonDataForContest(slot);
            } else if (*gPartyMenu.as_ptr()).menuType() == PARTY_MENU_TYPE_CHOOSE_HALF {
                DisplayPartyPokemonDataForChooseHalf(slot);
            } else if (*gPartyMenu.as_ptr()).menuType() == PARTY_MENU_TYPE_MINIGAME {
                DisplayPartyPokemonDataForWirelessMinigame(slot);
            } else if (*gPartyMenu.as_ptr()).menuType() == PARTY_MENU_TYPE_STORE_PYRAMID_HELD_ITEMS
            {
                DisplayPartyPokemonDataForBattlePyramidHeldItem(slot);
            } else if DisplayPartyPokemonDataForMoveTutorOrEvolutionItem(slot) == 0 {
                DisplayPartyPokemonData(slot);
            }
            if (*gPartyMenu.as_ptr()).menuType() == PARTY_MENU_TYPE_MULTI_SHOWCASE {
                AnimatePartySlot(slot, 0);
            } else if crate::gl!(gPartyMenu.slotId) as i32 == slot as i32 {
                AnimatePartySlot(slot, 1);
            } else {
                AnimatePartySlot(slot, 0);
            }
        }
        PutWindowTilemap((*(*sPartyMenuBoxes.as_ptr()).at(slot)).windowId);
        ScheduleBgCopyTilemapToVram(0);
    }
}
unsafe fn DisplayPartyPokemonData(slot: u8) {
    if GetMonData2(crate::gl!(&raw gPlayerParty[slot]), MON_DATA_IS_EGG) != 0 {
        (*(*(*sPartyMenuBoxes.as_ptr()).at(slot)).infoRects)
            .blitFunc
            .unwrap_unchecked()(
            (*(*sPartyMenuBoxes.as_ptr()).at(slot)).windowId,
            0,
            0,
            0,
            0,
            TRUE,
        );
        DisplayPartyPokemonNickname(
            crate::gl!(&raw gPlayerParty[slot]),
            (*sPartyMenuBoxes.as_ptr()).at(slot),
            0,
        );
    } else {
        (*(*(*sPartyMenuBoxes.as_ptr()).at(slot)).infoRects)
            .blitFunc
            .unwrap_unchecked()(
            (*(*sPartyMenuBoxes.as_ptr()).at(slot)).windowId,
            0,
            0,
            0,
            0,
            0,
        );
        DisplayPartyPokemonNickname(
            crate::gl!(&raw gPlayerParty[slot]),
            (*sPartyMenuBoxes.as_ptr()).at(slot),
            0,
        );
        DisplayPartyPokemonLevelCheck(
            crate::gl!(&raw gPlayerParty[slot]),
            (*sPartyMenuBoxes.as_ptr()).at(slot),
            0,
        );
        DisplayPartyPokemonGenderNidoranCheck(
            crate::gl!(&raw gPlayerParty[slot]),
            (*sPartyMenuBoxes.as_ptr()).at(slot),
            0,
        );
        DisplayPartyPokemonHPCheck(
            crate::gl!(&raw gPlayerParty[slot]),
            (*sPartyMenuBoxes.as_ptr()).at(slot),
            0,
        );
        DisplayPartyPokemonMaxHPCheck(
            crate::gl!(&raw gPlayerParty[slot]),
            (*sPartyMenuBoxes.as_ptr()).at(slot),
            0,
        );
        DisplayPartyPokemonHPBarCheck(
            crate::gl!(&raw gPlayerParty[slot]),
            (*sPartyMenuBoxes.as_ptr()).at(slot),
        );
    }
}
unsafe fn DisplayPartyPokemonDescriptionData(slot: u8, stringID: u8) {
    let mon: *mut Pokemon = crate::gl!(&raw gPlayerParty[slot]);
    (*(*(*sPartyMenuBoxes.as_ptr()).at(slot)).infoRects)
        .blitFunc
        .unwrap_unchecked()(
        (*(*sPartyMenuBoxes.as_ptr()).at(slot)).windowId,
        0,
        0,
        0,
        0,
        TRUE,
    );
    DisplayPartyPokemonNickname(mon, (*sPartyMenuBoxes.as_ptr()).at(slot), 0);
    if GetMonData2(mon, MON_DATA_IS_EGG) == 0 {
        DisplayPartyPokemonLevelCheck(mon, (*sPartyMenuBoxes.as_ptr()).at(slot), 0);
        DisplayPartyPokemonGenderNidoranCheck(mon, (*sPartyMenuBoxes.as_ptr()).at(slot), 0);
    }
    DisplayPartyPokemonDescriptionText(stringID, (*sPartyMenuBoxes.as_ptr()).at(slot), 0);
}
unsafe fn DisplayPartyPokemonDataForChooseHalf(slot: u8) {
    let mut i: u8 = 0;
    let mon: *mut Pokemon = crate::gl!(&raw gPlayerParty[slot]);
    let order: *mut u8 = (*gSelectedOrderFromParty.as_ptr()).as_mut_ptr();
    if GetBattleEntryEligibility(mon) == 0 {
        DisplayPartyPokemonDescriptionData(slot, PARTYBOX_DESC_NOT_ABLE);
    } else {
        i = 0;
        while i < GetMaxBattleEntries() {
            if *order.at(i) != 0 && *order.at(i) as i32 - 1 == slot as i32 {
                DisplayPartyPokemonDescriptionData(slot, i + PARTYBOX_DESC_FIRST);
                return;
            }
            i += 1;
        }
        DisplayPartyPokemonDescriptionData(slot, PARTYBOX_DESC_ABLE_3);
    }
}
unsafe fn DisplayPartyPokemonDataForContest(slot: u8) {
    match GetContestEntryEligibility(crate::gl!(&raw gPlayerParty[slot])) {
        CANT_ENTER_CONTEST | CANT_ENTER_CONTEST_EGG | CANT_ENTER_CONTEST_FAINTED => {
            DisplayPartyPokemonDescriptionData(slot, PARTYBOX_DESC_NOT_ABLE);
        }
        CAN_ENTER_CONTEST_EQUAL_RANK | CAN_ENTER_CONTEST_HIGH_RANK => {
            DisplayPartyPokemonDescriptionData(slot, PARTYBOX_DESC_ABLE);
        }
        _ => {}
    }
}
unsafe fn DisplayPartyPokemonDataForRelearner(slot: u8) {
    if GetNumberOfRelearnableMoves(crate::gl!(&raw gPlayerParty[slot])) == 0 {
        DisplayPartyPokemonDescriptionData(slot, PARTYBOX_DESC_NOT_ABLE_2);
    } else {
        DisplayPartyPokemonDescriptionData(slot, PARTYBOX_DESC_ABLE_2);
    }
}
unsafe fn DisplayPartyPokemonDataForWirelessMinigame(slot: u8) {
    if IsMonAllowedInMinigame(slot) == TRUE {
        DisplayPartyPokemonDescriptionData(slot, PARTYBOX_DESC_ABLE);
    } else {
        DisplayPartyPokemonDescriptionData(slot, PARTYBOX_DESC_NOT_ABLE);
    }
}
unsafe fn DisplayPartyPokemonDataForBattlePyramidHeldItem(slot: u8) {
    if GetMonData2(crate::gl!(&raw gPlayerParty[slot]), MON_DATA_HELD_ITEM) != 0 {
        DisplayPartyPokemonDescriptionData(slot, PARTYBOX_DESC_HAVE);
    } else {
        DisplayPartyPokemonDescriptionData(slot, PARTYBOX_DESC_DONT_HAVE);
    }
}
unsafe fn DisplayPartyPokemonDataForMoveTutorOrEvolutionItem(slot: u8) -> u8 {
    let currentPokemon: *mut Pokemon = crate::gl!(&raw gPlayerParty[slot]);
    let item: u16 = gSpecialVar_ItemId.get();
    if crate::gl!(gPartyMenu.action) == PARTY_ACTION_MOVE_TUTOR {
        gSpecialVar_Result = FALSE as u16;
        DisplayPartyPokemonDataToTeachMove(slot, 0, gSpecialVar_0x8005 as u8);
    } else {
        if crate::gl!(gPartyMenu.action) != PARTY_ACTION_USE_ITEM {
            return FALSE;
        }
        match CheckIfItemIsTMHMOrEvolutionStone(item) {
            ITEM_IS_TM_HM => {
                DisplayPartyPokemonDataToTeachMove(slot, item, 0);
            }
            ITEM_IS_EVOLUTION_STONE => {
                if GetMonData2(currentPokemon, MON_DATA_IS_EGG) == 0
                    && GetEvolutionTargetSpecies(currentPokemon, EVO_MODE_ITEM_CHECK, item)
                        != SPECIES_NONE
                {
                    return FALSE;
                }
                DisplayPartyPokemonDescriptionData(slot, PARTYBOX_DESC_NO_USE);
            }
            _ => {
                return FALSE;
            }
        }
    }
    TRUE
}
unsafe fn DisplayPartyPokemonDataToTeachMove(slot: u8, item: u16, tutor: u8) {
    match CanMonLearnTMTutor(crate::gl!(&raw gPlayerParty[slot]), item, tutor) {
        CANNOT_LEARN_MOVE | CANNOT_LEARN_MOVE_IS_EGG => {
            DisplayPartyPokemonDescriptionData(slot, PARTYBOX_DESC_NOT_ABLE_2);
        }
        ALREADY_KNOWS_MOVE => {
            DisplayPartyPokemonDescriptionData(slot, PARTYBOX_DESC_LEARNED);
        }
        _ => {
            DisplayPartyPokemonDescriptionData(slot, PARTYBOX_DESC_ABLE_2);
        }
    }
}
unsafe fn DisplayPartyPokemonDataForMultiBattle(slot: u8) {
    let menuBox: *mut PartyMenuBox = (*sPartyMenuBoxes.as_ptr()).at(slot);
    let actualSlot: u8 = slot - MULTI_PARTY_SIZE as u8;
    if crate::gl!(gMultiPartnerParty[actualSlot].species) == SPECIES_NONE {
        DrawEmptySlot((*menuBox).windowId);
    } else {
        (*(*menuBox).infoRects).blitFunc.unwrap_unchecked()((*menuBox).windowId, 0, 0, 0, 0, 0);
        StringCopy(
            gStringVar1.as_mut_ptr(),
            (*gMultiPartnerParty.as_ptr())[actualSlot]
                .nickname
                .as_mut_ptr(),
        );
        StringGet_Nickname(gStringVar1.as_mut_ptr());
        ConvertInternationalPlayerName(gStringVar1.as_mut_ptr());
        DisplayPartyPokemonBarDetail(
            (*menuBox).windowId,
            gStringVar1.as_mut_ptr(),
            0,
            (*(*menuBox).infoRects).dimensions.as_mut_ptr(),
        );
        DisplayPartyPokemonLevel(crate::gl!(gMultiPartnerParty[actualSlot].level), menuBox);
        DisplayPartyPokemonGender(
            crate::gl!(gMultiPartnerParty[actualSlot].gender),
            crate::gl!(gMultiPartnerParty[actualSlot].species),
            (*gMultiPartnerParty.as_ptr())[actualSlot]
                .nickname
                .as_mut_ptr(),
            menuBox,
        );
        DisplayPartyPokemonHP(crate::gl!(gMultiPartnerParty[actualSlot].hp), menuBox);
        DisplayPartyPokemonMaxHP(crate::gl!(gMultiPartnerParty[actualSlot].maxhp), menuBox);
        DisplayPartyPokemonHPBar(
            crate::gl!(gMultiPartnerParty[actualSlot].hp),
            crate::gl!(gMultiPartnerParty[actualSlot].maxhp),
            menuBox,
        );
    }
}
unsafe fn RenderPartyMenuBoxes() -> u8 {
    RenderPartyMenuBox((*crate::gl!(sPartyMenuInternal)).data[0] as u8);
    if ({
        (*crate::gl!(sPartyMenuInternal)).data[0] += 1;
        (*crate::gl!(sPartyMenuInternal)).data[0]
    }) == PARTY_SIZE as i16
    {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
unsafe fn GetPartyMenuBgTile(tileId: u16) -> *mut u8 {
    (*sPartyBgGfxTilemap.as_ptr()).at((tileId as i32) << 5)
}
unsafe fn CreatePartyMonSprites(slot: u8) {
    let mut actualSlot: u8 = 0;
    if (*gPartyMenu.as_ptr()).menuType() == PARTY_MENU_TYPE_MULTI_SHOWCASE
        && slot >= MULTI_PARTY_SIZE as u8
    {
        let mut status: u8 = 0;
        actualSlot = slot - MULTI_PARTY_SIZE as u8;
        if crate::gl!(gMultiPartnerParty[actualSlot].species) != SPECIES_NONE {
            CreatePartyMonIconSpriteParameterized(
                crate::gl!(gMultiPartnerParty[actualSlot].species),
                crate::gl!(gMultiPartnerParty[actualSlot].personality),
                (*sPartyMenuBoxes.as_ptr()).at(slot),
                0,
                0,
            );
            CreatePartyMonHeldItemSpriteParameterized(
                crate::gl!(gMultiPartnerParty[actualSlot].species),
                crate::gl!(gMultiPartnerParty[actualSlot].heldItem),
                (*sPartyMenuBoxes.as_ptr()).at(slot),
            );
            CreatePartyMonPokeballSpriteParameterized(
                crate::gl!(gMultiPartnerParty[actualSlot].species),
                (*sPartyMenuBoxes.as_ptr()).at(slot),
            );
            if crate::gl!(gMultiPartnerParty[actualSlot].hp) == 0 {
                status = AILMENT_FNT;
            } else {
                status = GetAilmentFromStatus(crate::gl!(gMultiPartnerParty[actualSlot].status));
            }
            CreatePartyMonStatusSpriteParameterized(
                crate::gl!(gMultiPartnerParty[actualSlot].species),
                status,
                (*sPartyMenuBoxes.as_ptr()).at(slot),
            );
        }
    } else if GetMonData2(crate::gl!(&raw gPlayerParty[slot]), MON_DATA_SPECIES)
        != SPECIES_NONE as u32
    {
        CreatePartyMonIconSprite(
            crate::gl!(&raw gPlayerParty[slot]),
            (*sPartyMenuBoxes.as_ptr()).at(slot),
            slot as u32,
        );
        CreatePartyMonHeldItemSprite(
            crate::gl!(&raw gPlayerParty[slot]),
            (*sPartyMenuBoxes.as_ptr()).at(slot),
        );
        CreatePartyMonPokeballSprite(
            crate::gl!(&raw gPlayerParty[slot]),
            (*sPartyMenuBoxes.as_ptr()).at(slot),
        );
        CreatePartyMonStatusSprite(
            crate::gl!(&raw gPlayerParty[slot]),
            (*sPartyMenuBoxes.as_ptr()).at(slot),
        );
    }
}
unsafe fn CreatePartyMonSpritesLoop() -> u8 {
    CreatePartyMonSprites((*crate::gl!(sPartyMenuInternal)).data[0] as u8);
    if ({
        (*crate::gl!(sPartyMenuInternal)).data[0] += 1;
        (*crate::gl!(sPartyMenuInternal)).data[0]
    }) == PARTY_SIZE as i16
    {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
unsafe fn CreateCancelConfirmPokeballSprites() {
    if (*gPartyMenu.as_ptr()).menuType() == PARTY_MENU_TYPE_MULTI_SHOWCASE {
        FillBgTilemapBufferRect(1, 14, 23, 17, 7, 2, 1);
    } else {
        if (*crate::gl!(sPartyMenuInternal)).chooseHalf() != 0 {
            (*crate::gl!(sPartyMenuInternal))
                .set_spriteIdConfirmPokeball(CreateSmallPokeballButtonSprite(0xBF, 0x88) as u32);
            DrawCancelConfirmButtons();
            (*crate::gl!(sPartyMenuInternal))
                .set_spriteIdCancelPokeball(CreateSmallPokeballButtonSprite(0xBF, 0x98) as u32);
        } else {
            (*crate::gl!(sPartyMenuInternal))
                .set_spriteIdCancelPokeball(CreatePokeballButtonSprite(198, 148) as u32);
        }
        AnimatePartySlot(crate::gl!(gPartyMenu.slotId) as u8, 1);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn AnimatePartySlot(slot: u8, animNum: u8) {
    let mut spriteId: u8 = 0;
    match slot {
        6 => {
            if animNum == 0 {
                SetBgTilemapPalette(1, 23, 16, 7, 2, 1);
            } else {
                SetBgTilemapPalette(1, 23, 16, 7, 2, 2);
            }
            spriteId = (*crate::gl!(sPartyMenuInternal)).spriteIdConfirmPokeball() as u8;
        }
        7 => {
            if (*crate::gl!(sPartyMenuInternal)).chooseHalf() == 0 {
                if animNum == 0 {
                    SetBgTilemapPalette(1, 23, 17, 7, 2, 1);
                } else {
                    SetBgTilemapPalette(1, 23, 17, 7, 2, 2);
                }
            } else if animNum == 0 {
                SetBgTilemapPalette(1, 23, 18, 7, 2, 1);
            } else {
                SetBgTilemapPalette(1, 23, 18, 7, 2, 2);
            }
            spriteId = (*crate::gl!(sPartyMenuInternal)).spriteIdCancelPokeball() as u8;
        }
        _ => {
            if GetMonData2(crate::gl!(&raw gPlayerParty[slot]), MON_DATA_SPECIES)
                != SPECIES_NONE as u32
            {
                LoadPartyBoxPalette(
                    (*sPartyMenuBoxes.as_ptr()).at(slot),
                    GetPartyBoxPaletteFlags(slot, animNum),
                );
                AnimateSelectedPartyIcon(
                    (*(*sPartyMenuBoxes.as_ptr()).at(slot)).monSpriteId,
                    animNum,
                );
                PartyMenuStartSpriteAnim(
                    (*(*sPartyMenuBoxes.as_ptr()).at(slot)).pokeballSpriteId,
                    animNum,
                );
            }
            return;
        }
    }
    PartyMenuStartSpriteAnim(spriteId, animNum);
    ScheduleBgCopyTilemapToVram(1);
}
unsafe fn GetPartyBoxPaletteFlags(slot: u8, animNum: u8) -> u8 {
    let mut palFlags: u8 = 0;
    if animNum == 1 {
        palFlags |= PARTY_PAL_SELECTED as u8;
    }
    if GetMonData2(crate::gl!(&raw gPlayerParty[slot]), MON_DATA_HP) == 0 {
        palFlags |= PARTY_PAL_FAINTED;
    }
    if PartyBoxPal_ParnterOrDisqualifiedInArena(slot) == TRUE {
        palFlags |= PARTY_PAL_MULTI_ALT;
    }
    if crate::gl!(gPartyMenu.action) == PARTY_ACTION_SWITCHING {
        palFlags |= PARTY_PAL_SWITCHING;
    }
    if crate::gl!(gPartyMenu.action) == PARTY_ACTION_SWITCH
        && (slot as i32 == crate::gl!(gPartyMenu.slotId) as i32
            || slot as i32 == crate::gl!(gPartyMenu.slotId2) as i32)
    {
        palFlags |= PARTY_PAL_TO_SWITCH;
    }
    if crate::gl!(gPartyMenu.action) == PARTY_ACTION_SOFTBOILED
        && slot as i32 == crate::gl!(gPartyMenu.slotId) as i32
    {
        palFlags |= PARTY_PAL_TO_SOFTBOIL;
    }
    palFlags
}
unsafe fn PartyBoxPal_ParnterOrDisqualifiedInArena(slot: u8) -> u8 {
    if (*gPartyMenu.as_ptr()).layout() == PARTY_LAYOUT_MULTI
        && (slot == 1 || slot == 4 || slot == 5)
    {
        return TRUE;
    }
    if slot < MULTI_PARTY_SIZE as u8
        && gBattleTypeFlags.get() & BATTLE_TYPE_ARENA != 0
        && (*gMain.as_ptr()).inBattle() != 0
        && shr_i32(
            (*crate::gl!(gBattleStruct)).arenaLostPlayerMons as i32,
            GetPartyIdFromBattleSlot(slot) as u32,
        ) & 1
            != 0
    {
        return TRUE;
    }
    FALSE
}
unsafe fn DrawCancelConfirmButtons() {
    CopyToBgTilemapBufferRect_ChangePalette(
        1,
        sConfirmButton_Tilemap.as_ptr().cast_mut() as *mut c_void,
        23,
        16,
        7,
        2,
        17,
    );
    CopyToBgTilemapBufferRect_ChangePalette(
        1,
        sCancelButton_Tilemap.as_ptr().cast_mut() as *mut c_void,
        23,
        18,
        7,
        2,
        17,
    );
    ScheduleBgCopyTilemapToVram(1);
}
pub unsafe fn IsMultiBattle() -> u8 {
    if gBattleTypeFlags.get() & BATTLE_TYPE_MULTI != 0
        && gBattleTypeFlags.get() & BATTLE_TYPE_DOUBLE != 0
        && gBattleTypeFlags.get() & BATTLE_TYPE_TRAINER != 0
        && (*gMain.as_ptr()).inBattle() != 0
    {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
unsafe fn SwapPartyPokemon(mon1: *mut Pokemon, mon2: *mut Pokemon) {
    let temp: *mut Pokemon = Alloc(100) as *mut Pokemon;
    *temp = *mon1;
    *mon1 = *mon2;
    *mon2 = *temp;
    Free(temp as *mut c_void);
}
unsafe fn Task_ClosePartyMenu(taskId: u8) {
    BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 16, 0);
    task_set_func(taskId, Some(Task_ClosePartyMenuAndSetCB2));
}
pub(crate) unsafe fn Task_ClosePartyMenuAndSetCB2(taskId: u8) {
    if (*gPaletteFade.as_ptr()).active() == 0 {
        if (*gPartyMenu.as_ptr()).menuType() == PARTY_MENU_TYPE_IN_BATTLE {
            UpdatePartyToFieldOrder();
        }
        if (*crate::gl!(sPartyMenuInternal)).exitCallback.is_some() {
            SetMainCallback2((*crate::gl!(sPartyMenuInternal)).exitCallback);
        } else {
            SetMainCallback2(crate::gl!(gPartyMenu.exitCallback));
        }
        ResetSpriteData();
        FreePartyPointers();
        DestroyTask(taskId);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn GetCursorSelectionMonId() -> u8 {
    crate::gl!(gPartyMenu.slotId) as u8
}
pub unsafe fn GetPartyMenuType() -> u8 {
    (*gPartyMenu.as_ptr()).menuType()
}
#[unsafe(no_mangle)]
pub unsafe fn Task_HandleChooseMonInput(taskId: u8) {
    if (*gPaletteFade.as_ptr()).active() == 0 && MenuHelpers_ShouldWaitForLinkRecv() != TRUE {
        let slotPtr: *mut i8 = GetCurrentPartySlotPtr();
        match PartyMenuButtonHandler(slotPtr) {
            1 => {
                HandleChooseMonSelection(taskId, slotPtr);
            }
            2 => {
                HandleChooseMonCancel(taskId, slotPtr);
            }
            8 if (*crate::gl!(sPartyMenuInternal)).chooseHalf() != 0 => {
                PlaySE(SE_SELECT);
                MoveCursorToConfirm();
            }
            _ => {}
        }
    }
}
fn GetCurrentPartySlotPtr() -> *mut i8 {
    if crate::gl!(gPartyMenu.action) == PARTY_ACTION_SWITCH
        || crate::gl!(gPartyMenu.action) == PARTY_ACTION_SOFTBOILED
    {
        return crate::gl!(&raw gPartyMenu.slotId2);
    } else {
        return crate::gl!(&raw gPartyMenu.slotId);
    }
    #[allow(unreachable_code)]
    {
        null_mut()
    }
}
unsafe fn HandleChooseMonSelection(taskId: u8, slotPtr: *mut i8) {
    if *slotPtr == PARTY_SIZE as i8 {
        (*gPartyMenu.as_ptr()).task.unwrap_unchecked()(taskId);
    } else {
        match crate::gl!(gPartyMenu.action) {
            PARTY_ACTION_SOFTBOILED => {
                if IsSelectedMonNotEgg(slotPtr as *mut u8) != 0 {
                    PartyMenuRemoveWindow(&raw mut (*crate::gl!(sPartyMenuInternal)).windowId[1]);
                    Task_TryUseSoftboiledOnPartyMon(taskId);
                }
            }
            PARTY_ACTION_USE_ITEM => {
                if IsSelectedMonNotEgg(slotPtr as *mut u8) != 0 {
                    if (*gPartyMenu.as_ptr()).menuType() == PARTY_MENU_TYPE_IN_BATTLE {
                        (*crate::gl!(sPartyMenuInternal)).exitCallback =
                            Some(CB2_SetUpExitToBattleScreen);
                    }
                    PartyMenuRemoveWindow(&raw mut (*crate::gl!(sPartyMenuInternal)).windowId[1]);
                    (*gItemUseCB.as_ptr()).unwrap_unchecked()(
                        taskId,
                        Some(Task_ClosePartyMenuAfterText),
                    );
                }
            }
            PARTY_ACTION_MOVE_TUTOR => {
                if IsSelectedMonNotEgg(slotPtr as *mut u8) != 0 {
                    PlaySE(SE_SELECT);
                    PartyMenuRemoveWindow(&raw mut (*crate::gl!(sPartyMenuInternal)).windowId[1]);
                    TryTutorSelectedMon(taskId);
                }
            }
            PARTY_ACTION_GIVE_MAILBOX_MAIL => {
                if IsSelectedMonNotEgg(slotPtr as *mut u8) != 0 {
                    PlaySE(SE_SELECT);
                    PartyMenuRemoveWindow(&raw mut (*crate::gl!(sPartyMenuInternal)).windowId[1]);
                    TryGiveMailToSelectedMon(taskId);
                }
            }
            PARTY_ACTION_GIVE_ITEM | PARTY_ACTION_GIVE_PC_ITEM => {
                if IsSelectedMonNotEgg(slotPtr as *mut u8) != 0 {
                    PlaySE(SE_SELECT);
                    PartyMenuRemoveWindow(&raw mut (*crate::gl!(sPartyMenuInternal)).windowId[1]);
                    TryGiveItemOrMailToSelectedMon(taskId);
                }
            }
            PARTY_ACTION_SWITCH => {
                PlaySE(SE_SELECT);
                SwitchSelectedMons(taskId);
            }
            PARTY_ACTION_CHOOSE_AND_CLOSE => {
                PlaySE(SE_SELECT);
                Task_ClosePartyMenu(taskId);
            }
            PARTY_ACTION_MINIGAME => {
                if IsSelectedMonNotEgg(slotPtr as *mut u8) != 0 {
                    TryEnterMonForMinigame(taskId, *slotPtr as u8);
                }
            }
            _ => {
                PlaySE(SE_SELECT);
                Task_TryCreateSelectionWindow(taskId);
            }
        }
    }
}
unsafe fn IsSelectedMonNotEgg(slotPtr: *mut u8) -> u8 {
    if GetMonData2(
        {
            let gl_i1 = *slotPtr;
            crate::gl!(&raw gPlayerParty[gl_i1])
        },
        MON_DATA_IS_EGG,
    ) == TRUE as u32
    {
        PlaySE(SE_FAILURE);
        return FALSE;
    }
    TRUE
}
unsafe fn HandleChooseMonCancel(taskId: u8, slotPtr: *mut i8) {
    match crate::gl!(gPartyMenu.action) {
        PARTY_ACTION_SEND_OUT => {
            PlaySE(SE_FAILURE);
        }
        PARTY_ACTION_SWITCH | PARTY_ACTION_SOFTBOILED => {
            PlaySE(SE_SELECT);
            FinishTwoMonAction(taskId);
        }
        PARTY_ACTION_MINIGAME => {
            PlaySE(SE_SELECT);
            CancelParticipationPrompt(taskId);
        }
        _ => {
            PlaySE(SE_SELECT);
            if DisplayCancelChooseMonYesNo(taskId) != TRUE {
                if MenuHelpers_IsLinkActive() == 0 {
                    gSpecialVar_0x8004 = 7;
                }
                gPartyMenuUseExitCallback.set(FALSE);
                *slotPtr = 7;
                Task_ClosePartyMenu(taskId);
            }
        }
    }
}
unsafe fn DisplayCancelChooseMonYesNo(taskId: u8) -> u8 {
    let mut stringPtr: *mut u8 = null_mut();
    if (*gPartyMenu.as_ptr()).menuType() == PARTY_MENU_TYPE_CONTEST {
        stringPtr = (*(&raw const crate::data::strings::gText_CancelParticipation)
            .cast::<CArray<u8, 0>>())
        .as_ptr()
        .cast_mut();
    } else if (*gPartyMenu.as_ptr()).menuType() == PARTY_MENU_TYPE_CHOOSE_HALF {
        stringPtr = GetFacilityCancelString();
    }
    if stringPtr.is_null() {
        return FALSE;
    }
    PartyMenuRemoveWindow(&raw mut (*crate::gl!(sPartyMenuInternal)).windowId[1]);
    StringExpandPlaceholders(gStringVar4.as_mut_ptr(), stringPtr);
    DisplayPartyMenuMessage(gStringVar4.as_mut_ptr(), TRUE);
    task_set_func(taskId, Some(Task_CancelChooseMonYesNo));
    TRUE
}
pub(crate) unsafe fn Task_CancelChooseMonYesNo(taskId: u8) {
    if IsPartyMenuTextPrinterActive() != TRUE {
        PartyMenuDisplayYesNoMenu();
        task_set_func(taskId, Some(Task_HandleCancelChooseMonYesNoInput));
    }
}
pub(crate) unsafe fn Task_HandleCancelChooseMonYesNoInput(taskId: u8) {
    'l1: {
        let sw1: i8 = Menu_ProcessInputNoWrapClearOnChoose();
        let mut fall = false;
        if sw1 == 0 {
            gPartyMenuUseExitCallback.set(FALSE);
            crate::gl!(gPartyMenu.slotId = 7);
            ClearSelectedPartyOrder();
            Task_ClosePartyMenu(taskId);
            break 'l1;
        }
        if sw1 == MENU_B_PRESSED {
            fall = true;
            PlaySE(SE_SELECT);
        }
        if fall || sw1 == 1 {
            Task_ReturnToChooseMonAfterText(taskId);
            break 'l1;
        }
    }
}
unsafe fn PartyMenuButtonHandler(slotPtr: *mut i8) -> u16 {
    let mut movementDir: i8 = 0;
    match crate::gl!(gMain.newAndRepeatedKeys) {
        64 => {
            movementDir = MENU_DIR_UP;
        }
        128 => {
            movementDir = MENU_DIR_DOWN;
        }
        32 => {
            movementDir = MENU_DIR_LEFT;
        }
        16 => {
            movementDir = MENU_DIR_RIGHT;
        }
        _ => match GetLRKeysPressedAndHeld() {
            MENU_L_PRESSED => {
                movementDir = MENU_DIR_UP;
            }
            MENU_R_PRESSED => {
                movementDir = MENU_DIR_DOWN;
            }
            _ => {
                movementDir = 0;
            }
        },
    }
    if crate::gl!(gMain.newKeys) as i32 & START_BUTTON != 0 {
        return START_BUTTON as u16;
    }
    if movementDir != 0 {
        UpdateCurrentPartySelection(slotPtr, movementDir);
        return 0;
    }
    if crate::gl!(gMain.newKeys) as i32 & 0x0001 != 0 && *slotPtr == 7 {
        return B_BUTTON as u16;
    }
    crate::gl!(gMain.newKeys) & 3
}
unsafe fn UpdateCurrentPartySelection(slotPtr: *mut i8, movementDir: i8) {
    let newSlotId: i8 = *slotPtr;
    let layout: u8 = (*gPartyMenu.as_ptr()).layout();
    if layout == PARTY_LAYOUT_SINGLE {
        UpdatePartySelectionSingleLayout(slotPtr, movementDir);
    } else {
        UpdatePartySelectionDoubleLayout(slotPtr, movementDir);
    }
    if *slotPtr != newSlotId {
        PlaySE(SE_SELECT);
        AnimatePartySlot(newSlotId as u8, 0);
        AnimatePartySlot(*slotPtr as u8, 1);
    }
}
unsafe fn UpdatePartySelectionSingleLayout(slotPtr: *mut i8, movementDir: i8) {
    match movementDir {
        MENU_DIR_UP => {
            if *slotPtr == 0 {
                *slotPtr = 7;
            } else if *slotPtr == PARTY_SIZE as i8 {
                *slotPtr = gPlayerPartyCount.get() as i8 - 1;
            } else if *slotPtr == 7 {
                if (*crate::gl!(sPartyMenuInternal)).chooseHalf() != 0 {
                    *slotPtr = PARTY_SIZE as i8;
                } else {
                    *slotPtr = gPlayerPartyCount.get() as i8 - 1;
                }
            } else {
                *slotPtr -= 1;
            }
        }
        MENU_DIR_DOWN => {
            if *slotPtr == 7 {
                *slotPtr = 0;
            } else {
                if *slotPtr as i32 == gPlayerPartyCount.get() as i32 - 1 {
                    if (*crate::gl!(sPartyMenuInternal)).chooseHalf() != 0 {
                        *slotPtr = PARTY_SIZE as i8;
                    } else {
                        *slotPtr = 7;
                    }
                } else {
                    *slotPtr += 1;
                }
            }
        }
        MENU_DIR_RIGHT => {
            if gPlayerPartyCount.get() != 1 && *slotPtr == 0 {
                if (*crate::gl!(sPartyMenuInternal)).lastSelectedSlot() == 0 {
                    *slotPtr = 1;
                } else {
                    *slotPtr = (*crate::gl!(sPartyMenuInternal)).lastSelectedSlot() as i8;
                }
            }
        }
        MENU_DIR_LEFT if *slotPtr != 0 && *slotPtr != PARTY_SIZE as i8 && *slotPtr != 7 => {
            (*crate::gl!(sPartyMenuInternal)).set_lastSelectedSlot(*slotPtr as u32);
            *slotPtr = 0;
        }
        _ => {}
    }
}
unsafe fn UpdatePartySelectionDoubleLayout(slotPtr: *mut i8, movementDir: i8) {
    let mut newSlot: i8 = movementDir;
    'l1: {
        match movementDir {
            MENU_DIR_UP => {
                if *slotPtr == 0 {
                    *slotPtr = 7;
                    break 'l1;
                } else if *slotPtr == PARTY_SIZE as i8 {
                    *slotPtr = gPlayerPartyCount.get() as i8 - 1;
                    break 'l1;
                } else if *slotPtr == 7 {
                    if (*crate::gl!(sPartyMenuInternal)).chooseHalf() != 0 {
                        *slotPtr = PARTY_SIZE as i8;
                        break 'l1;
                    }
                    *slotPtr -= 1;
                }
                newSlot = GetNewSlotDoubleLayout(*slotPtr, newSlot);
                if newSlot != -1 {
                    *slotPtr = newSlot;
                }
            }
            MENU_DIR_DOWN => {
                if *slotPtr == PARTY_SIZE as i8 {
                    *slotPtr = 7;
                } else if *slotPtr == 7 {
                    *slotPtr = 0;
                } else {
                    newSlot = GetNewSlotDoubleLayout(*slotPtr, MENU_DIR_DOWN);
                    if newSlot == -1 {
                        if (*crate::gl!(sPartyMenuInternal)).chooseHalf() != 0 {
                            *slotPtr = PARTY_SIZE as i8;
                        } else {
                            *slotPtr = 7;
                        }
                    } else {
                        *slotPtr = newSlot;
                    }
                }
            }
            MENU_DIR_RIGHT => {
                if *slotPtr == 0 {
                    if (*crate::gl!(sPartyMenuInternal)).lastSelectedSlot() == 3 {
                        if GetMonData2(crate::gl!(&raw gPlayerParty[3]), MON_DATA_SPECIES)
                            != SPECIES_NONE as u32
                        {
                            *slotPtr = 3;
                        }
                    } else if GetMonData2(crate::gl!(&raw gPlayerParty[2]), MON_DATA_SPECIES)
                        != SPECIES_NONE as u32
                    {
                        *slotPtr = 2;
                    }
                } else if *slotPtr == 1 {
                    if (*crate::gl!(sPartyMenuInternal)).lastSelectedSlot() == 5 {
                        if GetMonData2(crate::gl!(&raw gPlayerParty[5]), MON_DATA_SPECIES)
                            != SPECIES_NONE as u32
                        {
                            *slotPtr = 5;
                        }
                    } else if GetMonData2(crate::gl!(&raw gPlayerParty[4]), MON_DATA_SPECIES)
                        != SPECIES_NONE as u32
                    {
                        *slotPtr = 4;
                    }
                }
            }
            MENU_DIR_LEFT => {
                if *slotPtr == 2 || *slotPtr == 3 {
                    (*crate::gl!(sPartyMenuInternal)).set_lastSelectedSlot(*slotPtr as u32);
                    *slotPtr = 0;
                } else if *slotPtr == 4 || *slotPtr == 5 {
                    (*crate::gl!(sPartyMenuInternal)).set_lastSelectedSlot(*slotPtr as u32);
                    *slotPtr = 1;
                }
            }
            _ => {}
        }
    }
}
unsafe fn GetNewSlotDoubleLayout(mut slotId: i8, movementDir: i8) -> i8 {
    loop {
        slotId += movementDir;
        if slotId as u8 >= PARTY_SIZE as u8 {
            return -1;
        }
        if GetMonData2(crate::gl!(&raw gPlayerParty[slotId]), MON_DATA_SPECIES)
            != SPECIES_NONE as u32
        {
            return slotId;
        }
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
#[unsafe(no_mangle)]
pub unsafe fn GetMonNickname(mon: *mut Pokemon, dest: *mut u8) -> *mut u8 {
    GetMonData3(mon, MON_DATA_NICKNAME, dest);
    StringGet_Nickname(dest)
}
#[unsafe(no_mangle)]
pub unsafe fn DisplayPartyMenuMessage(str: *mut u8, keepOpen: u8) -> u8 {
    PrintMessage(str);
    let taskId: u8 = CreateTask(Some(Task_PrintAndWaitForText), 1);
    task_set(taskId, tKeepOpen, keepOpen as i16);
    taskId
}
pub(crate) unsafe fn Task_PrintAndWaitForText(taskId: u8) {
    if RunTextPrintersRetIsActive(WIN_MSG) != TRUE as u16 {
        if task_get(taskId, tKeepOpen) == FALSE as i16 {
            ClearStdWindowAndFrameToTransparent(WIN_MSG, FALSE);
            ClearWindowTilemap(WIN_MSG);
        }
        DestroyTask(taskId);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn IsPartyMenuTextPrinterActive() -> u8 {
    FuncIsActiveTask(Some(Task_PrintAndWaitForText))
}
pub(crate) unsafe fn Task_WaitForLinkAndReturnToChooseMon(taskId: u8) {
    if MenuHelpers_ShouldWaitForLinkRecv() != TRUE {
        DisplayPartyMenuStdMessage(PARTY_MSG_CHOOSE_MON);
        task_set_func(taskId, Some(Task_HandleChooseMonInput));
    }
}
pub(crate) unsafe fn Task_ReturnToChooseMonAfterText(taskId: u8) {
    if IsPartyMenuTextPrinterActive() != TRUE {
        ClearStdWindowAndFrameToTransparent(WIN_MSG, FALSE);
        ClearWindowTilemap(WIN_MSG);
        if MenuHelpers_IsLinkActive() == TRUE {
            task_set_func(taskId, Some(Task_WaitForLinkAndReturnToChooseMon));
        } else {
            DisplayPartyMenuStdMessage(PARTY_MSG_CHOOSE_MON);
            task_set_func(taskId, Some(Task_HandleChooseMonInput));
        }
    }
}
unsafe fn DisplayGaveHeldItemMessage(mon: *mut Pokemon, item: u16, keepOpen: u8, unused: u8) {
    GetMonNickname(mon, gStringVar1.as_mut_ptr());
    CopyItemName(item, (*gStringVar2.as_ptr()).as_mut_ptr());
    StringExpandPlaceholders(
        gStringVar4.as_mut_ptr(),
        (*(&raw const crate::data::strings::gText_PkmnWasGivenItem).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
    );
    DisplayPartyMenuMessage(gStringVar4.as_mut_ptr(), keepOpen);
    ScheduleBgCopyTilemapToVram(2);
}
unsafe fn DisplayTookHeldItemMessage(mon: *mut Pokemon, item: u16, keepOpen: u8) {
    GetMonNickname(mon, gStringVar1.as_mut_ptr());
    CopyItemName(item, (*gStringVar2.as_ptr()).as_mut_ptr());
    StringExpandPlaceholders(
        gStringVar4.as_mut_ptr(),
        (*(&raw const crate::data::strings::gText_ReceivedItemFromPkmn).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
    );
    DisplayPartyMenuMessage(gStringVar4.as_mut_ptr(), keepOpen);
    ScheduleBgCopyTilemapToVram(2);
}
unsafe fn DisplayAlreadyHoldingItemSwitchMessage(mon: *mut Pokemon, item: u16, keepOpen: u8) {
    GetMonNickname(mon, gStringVar1.as_mut_ptr());
    CopyItemName(item, (*gStringVar2.as_ptr()).as_mut_ptr());
    StringExpandPlaceholders(
        gStringVar4.as_mut_ptr(),
        (*(&raw const crate::data::strings::gText_PkmnAlreadyHoldingItemSwitch)
            .cast::<CArray<u8, 0>>())
        .as_ptr()
        .cast_mut(),
    );
    DisplayPartyMenuMessage(gStringVar4.as_mut_ptr(), keepOpen);
    ScheduleBgCopyTilemapToVram(2);
}
unsafe fn DisplaySwitchedHeldItemMessage(item: u16, item2: u16, keepOpen: u8) {
    CopyItemName(item, gStringVar1.as_mut_ptr());
    CopyItemName(item2, (*gStringVar2.as_ptr()).as_mut_ptr());
    StringExpandPlaceholders(
        gStringVar4.as_mut_ptr(),
        (*(&raw const crate::data::strings::gText_SwitchedPkmnItem).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
    );
    DisplayPartyMenuMessage(gStringVar4.as_mut_ptr(), keepOpen);
    ScheduleBgCopyTilemapToVram(2);
}
pub(crate) unsafe fn GiveItemToMon(mon: *mut Pokemon, item: u16) {
    let mut itemBytes: CArray<u8, 2> = zeroed();
    if ItemIsMail(item) == TRUE && GiveMailToMonByItemId(mon, item) == MAIL_NONE as u8 {
        return;
    }
    itemBytes[0] = item as u8;
    itemBytes[1] = (item >> 8) as u8;
    SetMonData(
        mon,
        MON_DATA_HELD_ITEM,
        itemBytes.as_mut_ptr() as *mut c_void,
    );
}
unsafe fn TryTakeMonItem(mon: *mut Pokemon) -> u8 {
    let mut item: u16 = GetMonData2(mon, MON_DATA_HELD_ITEM) as u16;
    if item == ITEM_NONE {
        return 0;
    }
    if AddBagItem(item, 1) == FALSE {
        return 1;
    }
    item = ITEM_NONE;
    SetMonData(mon, MON_DATA_HELD_ITEM, &raw mut item as *mut c_void);
    2
}
unsafe fn BufferBagFullCantTakeItemMessage(itemUnused: u16) {
    StringExpandPlaceholders(
        gStringVar4.as_mut_ptr(),
        (*(&raw const crate::data::strings::gText_BagFullCouldNotRemoveItem)
            .cast::<CArray<u8, 0>>())
        .as_ptr()
        .cast_mut(),
    );
}
pub(crate) unsafe fn Task_PartyMenuModifyHP(taskId: u8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    *data += *data.at(2);
    *data.at(3) -= 1;
    SetMonData(
        {
            let gl_i2 = *data.at(4);
            crate::gl!(&raw gPlayerParty[gl_i2])
        },
        MON_DATA_HP,
        data as *mut c_void,
    );
    DisplayPartyPokemonHPCheck(
        {
            let gl_i3 = *data.at(4);
            crate::gl!(&raw gPlayerParty[gl_i3])
        },
        (*sPartyMenuBoxes.as_ptr()).at(*data.at(4)),
        1,
    );
    DisplayPartyPokemonHPBarCheck(
        {
            let gl_i4 = *data.at(4);
            crate::gl!(&raw gPlayerParty[gl_i4])
        },
        (*sPartyMenuBoxes.as_ptr()).at(*data.at(4)),
    );
    if *data.at(3) == 0 || *data == 0 || *data == *data.at(1) {
        if *data > *data.at(5) {
            ConvertIntToDecimalStringN(
                (*gStringVar2.as_ptr()).as_mut_ptr(),
                *data as i32 - *data.at(5) as i32,
                STR_CONV_MODE_LEFT_ALIGN,
                3,
            );
        }
        SwitchTaskToFollowupFunc(taskId);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn PartyMenuModifyHP(
    taskId: u8,
    slot: u8,
    hpIncrement: i8,
    hpDifference: i16,
    task: Option<unsafe fn(u8)>,
) {
    let mon: *mut Pokemon = crate::gl!(&raw gPlayerParty[slot]);
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    *data = GetMonData2(mon, MON_DATA_HP) as i16;
    *data.at(1) = GetMonData2(mon, MON_DATA_MAX_HP) as i16;
    *data.at(2) = hpIncrement as i16;
    *data.at(3) = hpDifference;
    *data.at(4) = slot as i16;
    *data.at(5) = *data;
    SetTaskFuncWithFollowupFunc(taskId, Some(Task_PartyMenuModifyHP), task);
}
unsafe fn ResetHPTaskData(taskId: u8, caseId: u8, hp: u32) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    match caseId {
        0 => {
            *data = hp as i16;
            *data.at(5) = hp as i16;
        }
        1 => {
            *data.at(1) = hp as i16;
        }
        2 => {
            *data.at(2) = hp as i16;
        }
        3 => {
            *data.at(3) = hp as i16;
        }
        4 => {
            *data.at(4) = hp as i16;
        }
        5 => {
            SetTaskFuncWithFollowupFunc(
                taskId,
                Some(Task_PartyMenuModifyHP),
                core::mem::transmute::<usize, Option<unsafe fn(u8)>>(hp as usize),
            );
        }
        _ => {}
    }
}
#[unsafe(no_mangle)]
pub fn GetAilmentFromStatus(status: u32) -> u8 {
    if status & STATUS1_PSN_ANY != 0 {
        return AILMENT_PSN;
    }
    if status & STATUS1_PARALYSIS != 0 {
        return AILMENT_PRZ;
    }
    if status & STATUS1_SLEEP != 0 {
        return AILMENT_SLP;
    }
    if status & STATUS1_FREEZE != 0 {
        return AILMENT_FRZ;
    }
    if status & STATUS1_BURN != 0 {
        return AILMENT_BRN;
    }
    AILMENT_NONE
}
pub unsafe fn GetMonAilment(mon: *mut Pokemon) -> u8 {
    if GetMonData2(mon, MON_DATA_HP) == 0 {
        return AILMENT_FNT;
    }
    let ailment: u8 = GetAilmentFromStatus(GetMonData2(mon, MON_DATA_STATUS));
    if ailment != AILMENT_NONE {
        return ailment;
    }
    if CheckPartyPokerus(mon, 0) != 0 {
        return AILMENT_PKRS;
    }
    AILMENT_NONE
}
unsafe fn SetPartyMonsAllowedInMinigame() {
    let mut ptr: *mut i16 = null_mut();
    if (*gPartyMenu.as_ptr()).menuType() == PARTY_MENU_TYPE_MINIGAME {
        let mut i: u8 = 0;
        ptr = crate::gl!(&raw gPartyMenu.data[0]);
        crate::gl!(gPartyMenu.data[0] = 0);
        if gSpecialVar_0x8005 == 0 {
            i = 0;
            while i < gPlayerPartyCount.get() {
                *ptr += shl_i32(
                    IsMonAllowedInPokemonJump(crate::gl!(&raw gPlayerParty[i])) as i32,
                    i as u32,
                ) as i16;
                i += 1;
            }
        } else {
            i = 0;
            while i < gPlayerPartyCount.get() {
                *ptr += shl_i32(
                    IsMonAllowedInDodrioBerryPicking(crate::gl!(&raw gPlayerParty[i])) as i32,
                    i as u32,
                ) as i16;
                i += 1;
            }
        }
    }
}
unsafe fn IsMonAllowedInPokemonJump(mon: *mut Pokemon) -> u16 {
    if GetMonData2(mon, MON_DATA_IS_EGG) != TRUE as u32
        && IsSpeciesAllowedInPokemonJump(GetMonData2(mon, MON_DATA_SPECIES) as u16) != 0
    {
        return TRUE as u16;
    }
    FALSE as u16
}
unsafe fn IsMonAllowedInDodrioBerryPicking(mon: *mut Pokemon) -> u16 {
    if GetMonData2(mon, MON_DATA_IS_EGG) != TRUE as u32
        && GetMonData2(mon, MON_DATA_SPECIES) == SPECIES_DODRIO
    {
        return TRUE as u16;
    }
    FALSE as u16
}
fn IsMonAllowedInMinigame(slot: u8) -> u8 {
    if shr_i32(crate::gl!(gPartyMenu.data[0]) as i32, slot as u32) & 1 == 0 {
        return FALSE;
    }
    TRUE
}
unsafe fn TryEnterMonForMinigame(taskId: u8, slot: u8) {
    if IsMonAllowedInMinigame(slot) == TRUE {
        PlaySE(SE_SELECT);
        gSpecialVar_0x8004 = slot as u16;
        Task_ClosePartyMenu(taskId);
    } else {
        PlaySE(SE_FAILURE);
        DisplayPartyMenuMessage(
            (*(&raw const crate::data::strings::gText_PkmnCantParticipate).cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
            FALSE,
        );
        ScheduleBgCopyTilemapToVram(2);
        task_set_func(taskId, Some(Task_ReturnToChooseMonAfterText));
    }
}
unsafe fn CancelParticipationPrompt(taskId: u8) {
    DisplayPartyMenuMessage(
        (*(&raw const crate::data::strings::gText_CancelParticipation).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        TRUE,
    );
    ScheduleBgCopyTilemapToVram(2);
    task_set_func(taskId, Some(Task_CancelParticipationYesNo));
}
pub(crate) unsafe fn Task_CancelParticipationYesNo(taskId: u8) {
    if IsPartyMenuTextPrinterActive() != TRUE {
        PartyMenuDisplayYesNoMenu();
        task_set_func(taskId, Some(Task_HandleCancelParticipationYesNoInput));
    }
}
pub(crate) unsafe fn Task_HandleCancelParticipationYesNoInput(taskId: u8) {
    'l1: {
        let sw1: i8 = Menu_ProcessInputNoWrapClearOnChoose();
        let mut fall = false;
        if sw1 == 0 {
            gSpecialVar_0x8004 = 7;
            Task_ClosePartyMenu(taskId);
            break 'l1;
        }
        if sw1 == MENU_B_PRESSED {
            fall = true;
            PlaySE(SE_SELECT);
        }
        if fall || sw1 == 1 {
            task_set_func(taskId, Some(Task_ReturnToChooseMonAfterText));
            break 'l1;
        }
    }
}
unsafe fn CanMonLearnTMTutor(mon: *mut Pokemon, item: u16, tutor: u8) -> u8 {
    let mut r#move: u16 = 0;
    if GetMonData2(mon, MON_DATA_IS_EGG) != 0 {
        return CANNOT_LEARN_MOVE_IS_EGG;
    }
    if item >= ITEM_TM01 {
        if CanMonLearnTMHM(mon, item as u8 - ITEM_TM01 as u8) == 0 {
            return CANNOT_LEARN_MOVE;
        } else {
            r#move = ItemIdToBattleMoveId(item);
        }
    } else {
        if CanLearnTutorMove(GetMonData2(mon, MON_DATA_SPECIES) as u16, tutor) == 0 {
            return CANNOT_LEARN_MOVE;
        } else {
            r#move = GetTutorMove(tutor);
        }
    }
    if MonKnowsMove(mon, r#move) == TRUE {
        return ALREADY_KNOWS_MOVE;
    } else {
        return CAN_LEARN_MOVE;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
unsafe fn GetTutorMove(tutor: u8) -> u16 {
    gTutorMoves[tutor]
}
fn CanLearnTutorMove(species: u16, tutor: u8) -> u8 {
    if sTutorLearnsets[species] & shl_i32(1, tutor as u32) as u32 != 0 {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
unsafe fn InitPartyMenuWindows(layout: u8) {
    match layout {
        PARTY_LAYOUT_SINGLE => {
            InitWindows(sSinglePartyMenuWindowTemplate.as_ptr().cast_mut());
        }
        PARTY_LAYOUT_DOUBLE => {
            InitWindows(sDoublePartyMenuWindowTemplate.as_ptr().cast_mut());
        }
        PARTY_LAYOUT_MULTI => {
            InitWindows(sMultiPartyMenuWindowTemplate.as_ptr().cast_mut());
        }
        _ => {
            InitWindows(sShowcaseMultiPartyMenuWindowTemplate.as_ptr().cast_mut());
        }
    }
    DeactivateAllTextPrinters();
    for i in 0..(PARTY_SIZE as u8) {
        FillWindowPixelBuffer(i, 0);
    }
    LoadUserWindowBorderGfx(0, 0x4F, 208);
    LoadPalette(GetOverworldTextboxPalettePtr() as *mut c_void, 224, 32);
    LoadPalette(
        (*(&raw const crate::data::menu::gStandardMenuPalette).cast::<CArray<u16, 0>>())
            .as_ptr()
            .cast_mut() as *mut c_void,
        240,
        32,
    );
}
unsafe fn CreateCancelConfirmWindows(chooseHalf: u8) {
    let mut confirmWindowId: u8 = 0;
    let mut cancelWindowId: u8 = 0;
    let mut offset: u8 = 0;
    let mut mainOffset: u8 = 0;
    if (*gPartyMenu.as_ptr()).menuType() != PARTY_MENU_TYPE_MULTI_SHOWCASE {
        if chooseHalf == TRUE {
            confirmWindowId =
                AddWindow((&raw const *sConfirmButtonWindowTemplate).cast_mut()) as u8;
            FillWindowPixelBuffer(confirmWindowId, 0);
            mainOffset = GetStringCenterAlignXOffset(
                FONT_SMALL as i32,
                (*(&raw const crate::data::strings::gMenuText_Confirm).cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut(),
                48,
            ) as u8;
            AddTextPrinterParameterized4(
                confirmWindowId,
                FONT_SMALL,
                mainOffset,
                1,
                0,
                0,
                sFontColorTable[0].as_ptr().cast_mut(),
                TEXT_SKIP_DRAW as i8,
                (*(&raw const crate::data::strings::gMenuText_Confirm).cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut(),
            );
            PutWindowTilemap(confirmWindowId);
            CopyWindowToVram(confirmWindowId, COPYWIN_GFX);
            cancelWindowId =
                AddWindow((&raw const *sMultiCancelButtonWindowTemplate).cast_mut()) as u8;
            offset = 0;
        } else {
            cancelWindowId = AddWindow((&raw const *sCancelButtonWindowTemplate).cast_mut()) as u8;
            offset = 3;
        }
        FillWindowPixelBuffer(cancelWindowId, 0);
        if (*gPartyMenu.as_ptr()).menuType() != PARTY_MENU_TYPE_SPIN_TRADE {
            mainOffset = GetStringCenterAlignXOffset(
                FONT_SMALL as i32,
                (*(&raw const crate::data::strings::gText_Cancel).cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut(),
                48,
            ) as u8;
            AddTextPrinterParameterized3(
                cancelWindowId,
                FONT_SMALL,
                mainOffset + offset,
                1,
                sFontColorTable[0].as_ptr().cast_mut(),
                TEXT_SKIP_DRAW as i8,
                (*(&raw const crate::data::strings::gText_Cancel).cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut(),
            );
        } else {
            mainOffset = GetStringCenterAlignXOffset(
                FONT_SMALL as i32,
                (*(&raw const crate::data::strings::gText_Cancel2).cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut(),
                48,
            ) as u8;
            AddTextPrinterParameterized3(
                cancelWindowId,
                FONT_SMALL,
                mainOffset + offset,
                1,
                sFontColorTable[0].as_ptr().cast_mut(),
                TEXT_SKIP_DRAW as i8,
                (*(&raw const crate::data::strings::gText_Cancel2).cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut(),
            );
        }
        PutWindowTilemap(cancelWindowId);
        CopyWindowToVram(cancelWindowId, COPYWIN_GFX);
        ScheduleBgCopyTilemapToVram(0);
    }
}
unsafe fn GetPartyMenuPalBufferPtr(paletteId: u8) -> *mut u16 {
    &raw mut (*crate::gl!(sPartyMenuInternal)).palBuffer[paletteId]
}
unsafe fn BlitBitmapToPartyWindow(
    windowId: u8,
    b: *mut u8,
    c: u8,
    x: u8,
    y: u8,
    width: u8,
    height: u8,
) {
    let pixels: *mut u8 = AllocZeroed(height as u32 * width as u32 * 32) as *mut u8;
    if !pixels.is_null() {
        for i in 0..height {
            for j in 0..width {
                CpuSet(
                    GetPartyMenuBgTile(
                        *b.at(x as i32 + j as i32 + (y as i32 + i as i32) * c as i32) as u16,
                    ) as *mut c_void,
                    pixels.at((i as i32 * width as i32 + j as i32) * 32) as *mut c_void,
                    16,
                );
            }
        }
        BlitBitmapToWindow(
            windowId,
            pixels,
            x as u16 * 8,
            y as u16 * 8,
            width as u16 * 8,
            height as u16 * 8,
        );
        Free(pixels as *mut c_void);
    }
}
pub(crate) unsafe fn BlitBitmapToPartyWindow_LeftColumn(
    windowId: u8,
    x: u8,
    y: u8,
    mut width: u8,
    mut height: u8,
    hideHP: u8,
) {
    if width == 0 && height == 0 {
        width = 10;
        height = 7;
    }
    if hideHP == FALSE {
        BlitBitmapToPartyWindow(
            windowId,
            sSlotTilemap_Main.as_ptr().cast_mut(),
            10,
            x,
            y,
            width,
            height,
        );
    } else {
        BlitBitmapToPartyWindow(
            windowId,
            sSlotTilemap_MainNoHP.as_ptr().cast_mut(),
            10,
            x,
            y,
            width,
            height,
        );
    }
}
pub(crate) unsafe fn BlitBitmapToPartyWindow_RightColumn(
    windowId: u8,
    x: u8,
    y: u8,
    mut width: u8,
    mut height: u8,
    hideHP: u8,
) {
    if width == 0 && height == 0 {
        width = 18;
        height = 3;
    }
    if hideHP == FALSE {
        BlitBitmapToPartyWindow(
            windowId,
            sSlotTilemap_Wide.as_ptr().cast_mut(),
            18,
            x,
            y,
            width,
            height,
        );
    } else {
        BlitBitmapToPartyWindow(
            windowId,
            sSlotTilemap_WideNoHP.as_ptr().cast_mut(),
            18,
            x,
            y,
            width,
            height,
        );
    }
}
unsafe fn DrawEmptySlot(windowId: u8) {
    BlitBitmapToPartyWindow(
        windowId,
        sSlotTilemap_WideEmpty.as_ptr().cast_mut(),
        18,
        0,
        0,
        18,
        3,
    );
}
unsafe fn LoadPartyBoxPalette(menuBox: *mut PartyMenuBox, palFlags: u8) {
    let palOffset: u8 = GetWindowAttribute((*menuBox).windowId, WINDOW_PALETTE_NUM) as u8 * 16;
    if palFlags as i32 & PARTY_PAL_NO_MON as i32 != 0 {
        LoadPalette(
            GetPartyMenuPalBufferPtr(sPartyBoxNoMonPalIds[0]) as *mut c_void,
            sPartyBoxNoMonPalOffsets[0] as u16 + palOffset as u16,
            2,
        );
        LoadPalette(
            GetPartyMenuPalBufferPtr(sPartyBoxNoMonPalIds[1]) as *mut c_void,
            sPartyBoxNoMonPalOffsets[1] as u16 + palOffset as u16,
            2,
        );
        LoadPalette(
            GetPartyMenuPalBufferPtr(sPartyBoxNoMonPalIds[2]) as *mut c_void,
            sPartyBoxNoMonPalOffsets[2] as u16 + palOffset as u16,
            2,
        );
    } else if palFlags as i32 & PARTY_PAL_TO_SOFTBOIL as i32 != 0 {
        if palFlags as i32 & PARTY_PAL_SELECTED != 0 {
            LoadPalette(
                GetPartyMenuPalBufferPtr(sPartyBoxSelectedForActionPalIds1[0]) as *mut c_void,
                sPartyBoxPalOffsets1[0] as u16 + palOffset as u16,
                2,
            );
            LoadPalette(
                GetPartyMenuPalBufferPtr(sPartyBoxSelectedForActionPalIds1[1]) as *mut c_void,
                sPartyBoxPalOffsets1[1] as u16 + palOffset as u16,
                2,
            );
            LoadPalette(
                GetPartyMenuPalBufferPtr(sPartyBoxSelectedForActionPalIds1[2]) as *mut c_void,
                sPartyBoxPalOffsets1[2] as u16 + palOffset as u16,
                2,
            );
            LoadPalette(
                GetPartyMenuPalBufferPtr(sPartyBoxCurrSelectionPalIds2[0]) as *mut c_void,
                sPartyBoxPalOffsets2[0] as u16 + palOffset as u16,
                2,
            );
            LoadPalette(
                GetPartyMenuPalBufferPtr(sPartyBoxCurrSelectionPalIds2[1]) as *mut c_void,
                sPartyBoxPalOffsets2[1] as u16 + palOffset as u16,
                2,
            );
            LoadPalette(
                GetPartyMenuPalBufferPtr(sPartyBoxCurrSelectionPalIds2[2]) as *mut c_void,
                sPartyBoxPalOffsets2[2] as u16 + palOffset as u16,
                2,
            );
        } else {
            LoadPalette(
                GetPartyMenuPalBufferPtr(sPartyBoxSelectedForActionPalIds1[0]) as *mut c_void,
                sPartyBoxPalOffsets1[0] as u16 + palOffset as u16,
                2,
            );
            LoadPalette(
                GetPartyMenuPalBufferPtr(sPartyBoxSelectedForActionPalIds1[1]) as *mut c_void,
                sPartyBoxPalOffsets1[1] as u16 + palOffset as u16,
                2,
            );
            LoadPalette(
                GetPartyMenuPalBufferPtr(sPartyBoxSelectedForActionPalIds1[2]) as *mut c_void,
                sPartyBoxPalOffsets1[2] as u16 + palOffset as u16,
                2,
            );
            LoadPalette(
                GetPartyMenuPalBufferPtr(sPartyBoxSelectedForActionPalIds2[0]) as *mut c_void,
                sPartyBoxPalOffsets2[0] as u16 + palOffset as u16,
                2,
            );
            LoadPalette(
                GetPartyMenuPalBufferPtr(sPartyBoxSelectedForActionPalIds2[1]) as *mut c_void,
                sPartyBoxPalOffsets2[1] as u16 + palOffset as u16,
                2,
            );
            LoadPalette(
                GetPartyMenuPalBufferPtr(sPartyBoxSelectedForActionPalIds2[2]) as *mut c_void,
                sPartyBoxPalOffsets2[2] as u16 + palOffset as u16,
                2,
            );
        }
    } else if palFlags as i32 & PARTY_PAL_SWITCHING as i32 != 0 {
        LoadPalette(
            GetPartyMenuPalBufferPtr(sPartyBoxSelectedForActionPalIds1[0]) as *mut c_void,
            sPartyBoxPalOffsets1[0] as u16 + palOffset as u16,
            2,
        );
        LoadPalette(
            GetPartyMenuPalBufferPtr(sPartyBoxSelectedForActionPalIds1[1]) as *mut c_void,
            sPartyBoxPalOffsets1[1] as u16 + palOffset as u16,
            2,
        );
        LoadPalette(
            GetPartyMenuPalBufferPtr(sPartyBoxSelectedForActionPalIds1[2]) as *mut c_void,
            sPartyBoxPalOffsets1[2] as u16 + palOffset as u16,
            2,
        );
        LoadPalette(
            GetPartyMenuPalBufferPtr(sPartyBoxSelectedForActionPalIds2[0]) as *mut c_void,
            sPartyBoxPalOffsets2[0] as u16 + palOffset as u16,
            2,
        );
        LoadPalette(
            GetPartyMenuPalBufferPtr(sPartyBoxSelectedForActionPalIds2[1]) as *mut c_void,
            sPartyBoxPalOffsets2[1] as u16 + palOffset as u16,
            2,
        );
        LoadPalette(
            GetPartyMenuPalBufferPtr(sPartyBoxSelectedForActionPalIds2[2]) as *mut c_void,
            sPartyBoxPalOffsets2[2] as u16 + palOffset as u16,
            2,
        );
    } else if palFlags as i32 & PARTY_PAL_TO_SWITCH as i32 != 0 {
        if palFlags as i32 & PARTY_PAL_SELECTED != 0 {
            LoadPalette(
                GetPartyMenuPalBufferPtr(sPartyBoxSelectedForActionPalIds1[0]) as *mut c_void,
                sPartyBoxPalOffsets1[0] as u16 + palOffset as u16,
                2,
            );
            LoadPalette(
                GetPartyMenuPalBufferPtr(sPartyBoxSelectedForActionPalIds1[1]) as *mut c_void,
                sPartyBoxPalOffsets1[1] as u16 + palOffset as u16,
                2,
            );
            LoadPalette(
                GetPartyMenuPalBufferPtr(sPartyBoxSelectedForActionPalIds1[2]) as *mut c_void,
                sPartyBoxPalOffsets1[2] as u16 + palOffset as u16,
                2,
            );
            LoadPalette(
                GetPartyMenuPalBufferPtr(sPartyBoxCurrSelectionPalIds2[0]) as *mut c_void,
                sPartyBoxPalOffsets2[0] as u16 + palOffset as u16,
                2,
            );
            LoadPalette(
                GetPartyMenuPalBufferPtr(sPartyBoxCurrSelectionPalIds2[1]) as *mut c_void,
                sPartyBoxPalOffsets2[1] as u16 + palOffset as u16,
                2,
            );
            LoadPalette(
                GetPartyMenuPalBufferPtr(sPartyBoxCurrSelectionPalIds2[2]) as *mut c_void,
                sPartyBoxPalOffsets2[2] as u16 + palOffset as u16,
                2,
            );
        } else {
            LoadPalette(
                GetPartyMenuPalBufferPtr(sPartyBoxSelectedForActionPalIds1[0]) as *mut c_void,
                sPartyBoxPalOffsets1[0] as u16 + palOffset as u16,
                2,
            );
            LoadPalette(
                GetPartyMenuPalBufferPtr(sPartyBoxSelectedForActionPalIds1[1]) as *mut c_void,
                sPartyBoxPalOffsets1[1] as u16 + palOffset as u16,
                2,
            );
            LoadPalette(
                GetPartyMenuPalBufferPtr(sPartyBoxSelectedForActionPalIds1[2]) as *mut c_void,
                sPartyBoxPalOffsets1[2] as u16 + palOffset as u16,
                2,
            );
            LoadPalette(
                GetPartyMenuPalBufferPtr(sPartyBoxSelectedForActionPalIds2[0]) as *mut c_void,
                sPartyBoxPalOffsets2[0] as u16 + palOffset as u16,
                2,
            );
            LoadPalette(
                GetPartyMenuPalBufferPtr(sPartyBoxSelectedForActionPalIds2[1]) as *mut c_void,
                sPartyBoxPalOffsets2[1] as u16 + palOffset as u16,
                2,
            );
            LoadPalette(
                GetPartyMenuPalBufferPtr(sPartyBoxSelectedForActionPalIds2[2]) as *mut c_void,
                sPartyBoxPalOffsets2[2] as u16 + palOffset as u16,
                2,
            );
        }
    } else if palFlags as i32 & PARTY_PAL_FAINTED as i32 != 0 {
        if palFlags as i32 & PARTY_PAL_SELECTED != 0 {
            LoadPalette(
                GetPartyMenuPalBufferPtr(sPartyBoxCurrSelectionFaintedPalIds[0]) as *mut c_void,
                sPartyBoxPalOffsets1[0] as u16 + palOffset as u16,
                2,
            );
            LoadPalette(
                GetPartyMenuPalBufferPtr(sPartyBoxCurrSelectionFaintedPalIds[1]) as *mut c_void,
                sPartyBoxPalOffsets1[1] as u16 + palOffset as u16,
                2,
            );
            LoadPalette(
                GetPartyMenuPalBufferPtr(sPartyBoxCurrSelectionFaintedPalIds[2]) as *mut c_void,
                sPartyBoxPalOffsets1[2] as u16 + palOffset as u16,
                2,
            );
            LoadPalette(
                GetPartyMenuPalBufferPtr(sPartyBoxCurrSelectionPalIds2[0]) as *mut c_void,
                sPartyBoxPalOffsets2[0] as u16 + palOffset as u16,
                2,
            );
            LoadPalette(
                GetPartyMenuPalBufferPtr(sPartyBoxCurrSelectionPalIds2[1]) as *mut c_void,
                sPartyBoxPalOffsets2[1] as u16 + palOffset as u16,
                2,
            );
            LoadPalette(
                GetPartyMenuPalBufferPtr(sPartyBoxCurrSelectionPalIds2[2]) as *mut c_void,
                sPartyBoxPalOffsets2[2] as u16 + palOffset as u16,
                2,
            );
        } else {
            LoadPalette(
                GetPartyMenuPalBufferPtr(sPartyBoxFaintedPalIds1[0]) as *mut c_void,
                sPartyBoxPalOffsets1[0] as u16 + palOffset as u16,
                2,
            );
            LoadPalette(
                GetPartyMenuPalBufferPtr(sPartyBoxFaintedPalIds1[1]) as *mut c_void,
                sPartyBoxPalOffsets1[1] as u16 + palOffset as u16,
                2,
            );
            LoadPalette(
                GetPartyMenuPalBufferPtr(sPartyBoxFaintedPalIds1[2]) as *mut c_void,
                sPartyBoxPalOffsets1[2] as u16 + palOffset as u16,
                2,
            );
            LoadPalette(
                GetPartyMenuPalBufferPtr(sPartyBoxFaintedPalIds2[0]) as *mut c_void,
                sPartyBoxPalOffsets2[0] as u16 + palOffset as u16,
                2,
            );
            LoadPalette(
                GetPartyMenuPalBufferPtr(sPartyBoxFaintedPalIds2[1]) as *mut c_void,
                sPartyBoxPalOffsets2[1] as u16 + palOffset as u16,
                2,
            );
            LoadPalette(
                GetPartyMenuPalBufferPtr(sPartyBoxFaintedPalIds2[2]) as *mut c_void,
                sPartyBoxPalOffsets2[2] as u16 + palOffset as u16,
                2,
            );
        }
    } else if palFlags as i32 & PARTY_PAL_MULTI_ALT as i32 != 0 {
        if palFlags as i32 & PARTY_PAL_SELECTED != 0 {
            LoadPalette(
                GetPartyMenuPalBufferPtr(sPartyBoxCurrSelectionMultiPalIds[0]) as *mut c_void,
                sPartyBoxPalOffsets1[0] as u16 + palOffset as u16,
                2,
            );
            LoadPalette(
                GetPartyMenuPalBufferPtr(sPartyBoxCurrSelectionMultiPalIds[1]) as *mut c_void,
                sPartyBoxPalOffsets1[1] as u16 + palOffset as u16,
                2,
            );
            LoadPalette(
                GetPartyMenuPalBufferPtr(sPartyBoxCurrSelectionMultiPalIds[2]) as *mut c_void,
                sPartyBoxPalOffsets1[2] as u16 + palOffset as u16,
                2,
            );
            LoadPalette(
                GetPartyMenuPalBufferPtr(sPartyBoxCurrSelectionPalIds2[0]) as *mut c_void,
                sPartyBoxPalOffsets2[0] as u16 + palOffset as u16,
                2,
            );
            LoadPalette(
                GetPartyMenuPalBufferPtr(sPartyBoxCurrSelectionPalIds2[1]) as *mut c_void,
                sPartyBoxPalOffsets2[1] as u16 + palOffset as u16,
                2,
            );
            LoadPalette(
                GetPartyMenuPalBufferPtr(sPartyBoxCurrSelectionPalIds2[2]) as *mut c_void,
                sPartyBoxPalOffsets2[2] as u16 + palOffset as u16,
                2,
            );
        } else {
            LoadPalette(
                GetPartyMenuPalBufferPtr(sPartyBoxMultiPalIds1[0]) as *mut c_void,
                sPartyBoxPalOffsets1[0] as u16 + palOffset as u16,
                2,
            );
            LoadPalette(
                GetPartyMenuPalBufferPtr(sPartyBoxMultiPalIds1[1]) as *mut c_void,
                sPartyBoxPalOffsets1[1] as u16 + palOffset as u16,
                2,
            );
            LoadPalette(
                GetPartyMenuPalBufferPtr(sPartyBoxMultiPalIds1[2]) as *mut c_void,
                sPartyBoxPalOffsets1[2] as u16 + palOffset as u16,
                2,
            );
            LoadPalette(
                GetPartyMenuPalBufferPtr(sPartyBoxMultiPalIds2[0]) as *mut c_void,
                sPartyBoxPalOffsets2[0] as u16 + palOffset as u16,
                2,
            );
            LoadPalette(
                GetPartyMenuPalBufferPtr(sPartyBoxMultiPalIds2[1]) as *mut c_void,
                sPartyBoxPalOffsets2[1] as u16 + palOffset as u16,
                2,
            );
            LoadPalette(
                GetPartyMenuPalBufferPtr(sPartyBoxMultiPalIds2[2]) as *mut c_void,
                sPartyBoxPalOffsets2[2] as u16 + palOffset as u16,
                2,
            );
        }
    } else if palFlags as i32 & PARTY_PAL_SELECTED != 0 {
        LoadPalette(
            GetPartyMenuPalBufferPtr(sPartyBoxCurrSelectionPalIds1[0]) as *mut c_void,
            sPartyBoxPalOffsets1[0] as u16 + palOffset as u16,
            2,
        );
        LoadPalette(
            GetPartyMenuPalBufferPtr(sPartyBoxCurrSelectionPalIds1[1]) as *mut c_void,
            sPartyBoxPalOffsets1[1] as u16 + palOffset as u16,
            2,
        );
        LoadPalette(
            GetPartyMenuPalBufferPtr(sPartyBoxCurrSelectionPalIds1[2]) as *mut c_void,
            sPartyBoxPalOffsets1[2] as u16 + palOffset as u16,
            2,
        );
        LoadPalette(
            GetPartyMenuPalBufferPtr(sPartyBoxCurrSelectionPalIds2[0]) as *mut c_void,
            sPartyBoxPalOffsets2[0] as u16 + palOffset as u16,
            2,
        );
        LoadPalette(
            GetPartyMenuPalBufferPtr(sPartyBoxCurrSelectionPalIds2[1]) as *mut c_void,
            sPartyBoxPalOffsets2[1] as u16 + palOffset as u16,
            2,
        );
        LoadPalette(
            GetPartyMenuPalBufferPtr(sPartyBoxCurrSelectionPalIds2[2]) as *mut c_void,
            sPartyBoxPalOffsets2[2] as u16 + palOffset as u16,
            2,
        );
    } else {
        LoadPalette(
            GetPartyMenuPalBufferPtr(sPartyBoxEmptySlotPalIds1[0]) as *mut c_void,
            sPartyBoxPalOffsets1[0] as u16 + palOffset as u16,
            2,
        );
        LoadPalette(
            GetPartyMenuPalBufferPtr(sPartyBoxEmptySlotPalIds1[1]) as *mut c_void,
            sPartyBoxPalOffsets1[1] as u16 + palOffset as u16,
            2,
        );
        LoadPalette(
            GetPartyMenuPalBufferPtr(sPartyBoxEmptySlotPalIds1[2]) as *mut c_void,
            sPartyBoxPalOffsets1[2] as u16 + palOffset as u16,
            2,
        );
        LoadPalette(
            GetPartyMenuPalBufferPtr(sPartyBoxEmptySlotPalIds2[0]) as *mut c_void,
            sPartyBoxPalOffsets2[0] as u16 + palOffset as u16,
            2,
        );
        LoadPalette(
            GetPartyMenuPalBufferPtr(sPartyBoxEmptySlotPalIds2[1]) as *mut c_void,
            sPartyBoxPalOffsets2[1] as u16 + palOffset as u16,
            2,
        );
        LoadPalette(
            GetPartyMenuPalBufferPtr(sPartyBoxEmptySlotPalIds2[2]) as *mut c_void,
            sPartyBoxPalOffsets2[2] as u16 + palOffset as u16,
            2,
        );
    }
}
unsafe fn DisplayPartyPokemonBarDetail(windowId: u8, str: *mut u8, color: u8, align: *mut u8) {
    AddTextPrinterParameterized3(
        windowId,
        FONT_SMALL,
        *align,
        *align.at(1),
        sFontColorTable[color].as_ptr().cast_mut(),
        0,
        str,
    );
}
unsafe fn DisplayPartyPokemonNickname(mon: *mut Pokemon, menuBox: *mut PartyMenuBox, c: u8) {
    let mut nickname: CArray<u8, 11> = zeroed();
    if GetMonData2(mon, MON_DATA_SPECIES) != SPECIES_NONE as u32 {
        if c == 1 {
            (*(*menuBox).infoRects).blitFunc.unwrap_unchecked()(
                (*menuBox).windowId,
                (*(*menuBox).infoRects).dimensions[0] >> 3,
                (*(*menuBox).infoRects).dimensions[1] >> 3,
                (*(*menuBox).infoRects).dimensions[2] >> 3,
                (*(*menuBox).infoRects).dimensions[3] >> 3,
                0,
            );
        }
        GetMonNickname(mon, nickname.as_mut_ptr());
        DisplayPartyPokemonBarDetail(
            (*menuBox).windowId,
            nickname.as_mut_ptr(),
            0,
            (*(*menuBox).infoRects).dimensions.as_mut_ptr(),
        );
    }
}
unsafe fn DisplayPartyPokemonLevelCheck(mon: *mut Pokemon, menuBox: *mut PartyMenuBox, c: u8) {
    if GetMonData2(mon, MON_DATA_SPECIES) != SPECIES_NONE as u32 {
        let ailment: u8 = GetMonAilment(mon);
        if ailment == AILMENT_NONE || ailment == AILMENT_PKRS {
            if c != 0 {
                (*(*menuBox).infoRects).blitFunc.unwrap_unchecked()(
                    (*menuBox).windowId,
                    (*(*menuBox).infoRects).dimensions[4] >> 3,
                    ((*(*menuBox).infoRects).dimensions[5] >> 3) + 1,
                    (*(*menuBox).infoRects).dimensions[6] >> 3,
                    (*(*menuBox).infoRects).dimensions[7] >> 3,
                    FALSE,
                );
            }
            if c != 2 {
                DisplayPartyPokemonLevel(GetMonData2(mon, MON_DATA_LEVEL) as u8, menuBox);
            }
        }
    }
}
unsafe fn DisplayPartyPokemonLevel(level: u8, menuBox: *mut PartyMenuBox) {
    ConvertIntToDecimalStringN(
        (*gStringVar2.as_ptr()).as_mut_ptr(),
        level as i32,
        STR_CONV_MODE_LEFT_ALIGN,
        3,
    );
    StringCopy(
        gStringVar1.as_mut_ptr(),
        (*(&raw const crate::data::strings::gText_LevelSymbol).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
    );
    StringAppend(
        gStringVar1.as_mut_ptr(),
        (*gStringVar2.as_ptr()).as_mut_ptr(),
    );
    DisplayPartyPokemonBarDetail(
        (*menuBox).windowId,
        gStringVar1.as_mut_ptr(),
        0,
        &raw mut (*(*menuBox).infoRects).dimensions[4],
    );
}
unsafe fn DisplayPartyPokemonGenderNidoranCheck(
    mon: *mut Pokemon,
    menuBox: *mut PartyMenuBox,
    c: u8,
) {
    let mut nickname: CArray<u8, 11> = zeroed();
    if c == 1 {
        (*(*menuBox).infoRects).blitFunc.unwrap_unchecked()(
            (*menuBox).windowId,
            (*(*menuBox).infoRects).dimensions[8] >> 3,
            ((*(*menuBox).infoRects).dimensions[9] >> 3) + 1,
            (*(*menuBox).infoRects).dimensions[10] >> 3,
            (*(*menuBox).infoRects).dimensions[11] >> 3,
            FALSE,
        );
    }
    GetMonNickname(mon, nickname.as_mut_ptr());
    DisplayPartyPokemonGender(
        GetMonGender(mon),
        GetMonData2(mon, MON_DATA_SPECIES) as u16,
        nickname.as_mut_ptr(),
        menuBox,
    );
}
unsafe fn DisplayPartyPokemonGender(
    gender: u8,
    species: u16,
    nickname: *mut u8,
    menuBox: *mut PartyMenuBox,
) {
    let palOffset: u8 = GetWindowAttribute((*menuBox).windowId, WINDOW_PALETTE_NUM) as u8 * 16;
    if species == SPECIES_NONE {
        return;
    }
    if (species == SPECIES_NIDORAN_M || species == SPECIES_NIDORAN_F)
        && StringCompare(
            nickname,
            (*(&raw const crate::data::data_tables::gSpeciesNames)
                .cast::<CArray<CArray<u8, 11>, 0>>())[species]
                .as_ptr()
                .cast_mut(),
        ) == 0
    {
        return;
    }
    match gender {
        MON_MALE => {
            LoadPalette(
                GetPartyMenuPalBufferPtr(sGenderMalePalIds[0]) as *mut c_void,
                sGenderPalOffsets[0] as u16 + palOffset as u16,
                2,
            );
            LoadPalette(
                GetPartyMenuPalBufferPtr(sGenderMalePalIds[1]) as *mut c_void,
                sGenderPalOffsets[1] as u16 + palOffset as u16,
                2,
            );
            DisplayPartyPokemonBarDetail(
                (*menuBox).windowId,
                (*(&raw const crate::data::strings::gText_MaleSymbol).cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut(),
                2,
                &raw mut (*(*menuBox).infoRects).dimensions[8],
            );
        }
        MON_FEMALE => {
            LoadPalette(
                GetPartyMenuPalBufferPtr(sGenderFemalePalIds[0]) as *mut c_void,
                sGenderPalOffsets[0] as u16 + palOffset as u16,
                2,
            );
            LoadPalette(
                GetPartyMenuPalBufferPtr(sGenderFemalePalIds[1]) as *mut c_void,
                sGenderPalOffsets[1] as u16 + palOffset as u16,
                2,
            );
            DisplayPartyPokemonBarDetail(
                (*menuBox).windowId,
                (*(&raw const crate::data::strings::gText_FemaleSymbol).cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut(),
                2,
                &raw mut (*(*menuBox).infoRects).dimensions[8],
            );
        }
        _ => {}
    }
}
unsafe fn DisplayPartyPokemonHPCheck(mon: *mut Pokemon, menuBox: *mut PartyMenuBox, c: u8) {
    if GetMonData2(mon, MON_DATA_SPECIES) != SPECIES_NONE as u32 {
        if c != 0 {
            (*(*menuBox).infoRects).blitFunc.unwrap_unchecked()(
                (*menuBox).windowId,
                (*(*menuBox).infoRects).dimensions[12] >> 3,
                ((*(*menuBox).infoRects).dimensions[13] >> 3) + 1,
                (*(*menuBox).infoRects).dimensions[14] >> 3,
                (*(*menuBox).infoRects).dimensions[15] >> 3,
                FALSE,
            );
        }
        if c != 2 {
            DisplayPartyPokemonHP(GetMonData2(mon, MON_DATA_HP) as u16, menuBox);
        }
    }
}
unsafe fn DisplayPartyPokemonHP(hp: u16, menuBox: *mut PartyMenuBox) {
    let strOut: *mut u8 = ConvertIntToDecimalStringN(
        gStringVar1.as_mut_ptr(),
        hp as i32,
        STR_CONV_MODE_RIGHT_ALIGN,
        3,
    );
    *strOut = CHAR_SLASH;
    *strOut.at(1) = EOS;
    DisplayPartyPokemonBarDetail(
        (*menuBox).windowId,
        gStringVar1.as_mut_ptr(),
        0,
        &raw mut (*(*menuBox).infoRects).dimensions[12],
    );
}
unsafe fn DisplayPartyPokemonMaxHPCheck(mon: *mut Pokemon, menuBox: *mut PartyMenuBox, c: u8) {
    if GetMonData2(mon, MON_DATA_SPECIES) != SPECIES_NONE as u32 {
        if c != 0 {
            (*(*menuBox).infoRects).blitFunc.unwrap_unchecked()(
                (*menuBox).windowId,
                ((*(*menuBox).infoRects).dimensions[16] >> 3) + 1,
                ((*(*menuBox).infoRects).dimensions[17] >> 3) + 1,
                (*(*menuBox).infoRects).dimensions[18] >> 3,
                (*(*menuBox).infoRects).dimensions[19] >> 3,
                FALSE,
            );
        }
        if c != 2 {
            DisplayPartyPokemonMaxHP(GetMonData2(mon, MON_DATA_MAX_HP) as u16, menuBox);
        }
    }
}
unsafe fn DisplayPartyPokemonMaxHP(maxhp: u16, menuBox: *mut PartyMenuBox) {
    ConvertIntToDecimalStringN(
        (*gStringVar2.as_ptr()).as_mut_ptr(),
        maxhp as i32,
        STR_CONV_MODE_RIGHT_ALIGN,
        3,
    );
    StringCopy(
        gStringVar1.as_mut_ptr(),
        (*(&raw const crate::data::strings::gText_Slash).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
    );
    StringAppend(
        gStringVar1.as_mut_ptr(),
        (*gStringVar2.as_ptr()).as_mut_ptr(),
    );
    DisplayPartyPokemonBarDetail(
        (*menuBox).windowId,
        gStringVar1.as_mut_ptr(),
        0,
        &raw mut (*(*menuBox).infoRects).dimensions[16],
    );
}
unsafe fn DisplayPartyPokemonHPBarCheck(mon: *mut Pokemon, menuBox: *mut PartyMenuBox) {
    if GetMonData2(mon, MON_DATA_SPECIES) != SPECIES_NONE as u32 {
        DisplayPartyPokemonHPBar(
            GetMonData2(mon, MON_DATA_HP) as u16,
            GetMonData2(mon, MON_DATA_MAX_HP) as u16,
            menuBox,
        );
    }
}
unsafe fn DisplayPartyPokemonHPBar(hp: u16, maxhp: u16, menuBox: *mut PartyMenuBox) {
    let palOffset: u8 = GetWindowAttribute((*menuBox).windowId, WINDOW_PALETTE_NUM) as u8 * 16;
    match GetHPBarLevel(hp as i16, maxhp as i16) {
        HP_BAR_GREEN | HP_BAR_FULL => {
            LoadPalette(
                GetPartyMenuPalBufferPtr(sHPBarGreenPalIds[0]) as *mut c_void,
                sHPBarPalOffsets[0] as u16 + palOffset as u16,
                2,
            );
            LoadPalette(
                GetPartyMenuPalBufferPtr(sHPBarGreenPalIds[1]) as *mut c_void,
                sHPBarPalOffsets[1] as u16 + palOffset as u16,
                2,
            );
        }
        HP_BAR_YELLOW => {
            LoadPalette(
                GetPartyMenuPalBufferPtr(sHPBarYellowPalIds[0]) as *mut c_void,
                sHPBarPalOffsets[0] as u16 + palOffset as u16,
                2,
            );
            LoadPalette(
                GetPartyMenuPalBufferPtr(sHPBarYellowPalIds[1]) as *mut c_void,
                sHPBarPalOffsets[1] as u16 + palOffset as u16,
                2,
            );
        }
        _ => {
            LoadPalette(
                GetPartyMenuPalBufferPtr(sHPBarRedPalIds[0]) as *mut c_void,
                sHPBarPalOffsets[0] as u16 + palOffset as u16,
                2,
            );
            LoadPalette(
                GetPartyMenuPalBufferPtr(sHPBarRedPalIds[1]) as *mut c_void,
                sHPBarPalOffsets[1] as u16 + palOffset as u16,
                2,
            );
        }
    }
    let hpFraction: u8 = GetScaledHPFraction(
        hp as i16,
        maxhp as i16,
        (*(*menuBox).infoRects).dimensions[22],
    );
    FillWindowPixelRect(
        (*menuBox).windowId,
        sHPBarPalOffsets[1],
        (*(*menuBox).infoRects).dimensions[20] as u16,
        (*(*menuBox).infoRects).dimensions[21] as u16,
        hpFraction as u16,
        1,
    );
    FillWindowPixelRect(
        (*menuBox).windowId,
        sHPBarPalOffsets[0],
        (*(*menuBox).infoRects).dimensions[20] as u16,
        (*(*menuBox).infoRects).dimensions[21] as u16 + 1,
        hpFraction as u16,
        2,
    );
    if hpFraction != (*(*menuBox).infoRects).dimensions[22] {
        FillWindowPixelRect(
            (*menuBox).windowId,
            0x0D,
            (*(*menuBox).infoRects).dimensions[20] as u16 + hpFraction as u16,
            (*(*menuBox).infoRects).dimensions[21] as u16,
            (*(*menuBox).infoRects).dimensions[22] as u16 - hpFraction as u16,
            1,
        );
        FillWindowPixelRect(
            (*menuBox).windowId,
            0x02,
            (*(*menuBox).infoRects).dimensions[20] as u16 + hpFraction as u16,
            (*(*menuBox).infoRects).dimensions[21] as u16 + 1,
            (*(*menuBox).infoRects).dimensions[22] as u16 - hpFraction as u16,
            2,
        );
    }
    CopyWindowToVram((*menuBox).windowId, COPYWIN_GFX);
}
unsafe fn DisplayPartyPokemonDescriptionText(stringID: u8, menuBox: *mut PartyMenuBox, c: u8) {
    if c != 0 {
        let width: i32 = ((*(*menuBox).infoRects).descTextLeft as i32 % 8
            + (*(*menuBox).infoRects).descTextWidth as i32
            + 7)
            / 8;
        let height: i32 = ((*(*menuBox).infoRects).descTextTop as i32 % 8
            + (*(*menuBox).infoRects).descTextHeight as i32
            + 7)
            / 8;
        (*(*menuBox).infoRects).blitFunc.unwrap_unchecked()(
            (*menuBox).windowId,
            (*(*menuBox).infoRects).descTextLeft >> 3,
            (*(*menuBox).infoRects).descTextTop >> 3,
            width as u8,
            height as u8,
            TRUE,
        );
    }
    if c != 2 {
        AddTextPrinterParameterized3(
            (*menuBox).windowId,
            FONT_NORMAL,
            (*(*menuBox).infoRects).descTextLeft,
            (*(*menuBox).infoRects).descTextTop,
            sFontColorTable[0].as_ptr().cast_mut(),
            0,
            sDescriptionStringTable[stringID],
        );
    }
}
unsafe fn PartyMenuRemoveWindow(ptr: *mut u8) {
    if *ptr != WINDOW_NONE {
        ClearStdWindowAndFrameToTransparent(*ptr, FALSE);
        RemoveWindow(*ptr);
        *ptr = WINDOW_NONE;
        ScheduleBgCopyTilemapToVram(2);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn DisplayPartyMenuStdMessage(mut stringId: u32) {
    let windowPtr: *mut u8 = &raw mut (*crate::gl!(sPartyMenuInternal)).windowId[1];
    if *windowPtr != WINDOW_NONE {
        PartyMenuRemoveWindow(windowPtr);
    }
    if stringId != PARTY_MSG_NONE as u32 {
        match stringId {
            PARTY_MSG_DO_WHAT_WITH_MON => {
                *windowPtr =
                    AddWindow((&raw const *sDoWhatWithMonMsgWindowTemplate).cast_mut()) as u8;
            }
            PARTY_MSG_DO_WHAT_WITH_ITEM => {
                *windowPtr =
                    AddWindow((&raw const *sDoWhatWithItemMsgWindowTemplate).cast_mut()) as u8;
            }
            PARTY_MSG_DO_WHAT_WITH_MAIL => {
                *windowPtr =
                    AddWindow((&raw const *sDoWhatWithMailMsgWindowTemplate).cast_mut()) as u8;
            }
            PARTY_MSG_RESTORE_WHICH_MOVE | PARTY_MSG_BOOST_PP_WHICH_MOVE => {
                *windowPtr = AddWindow((&raw const *sWhichMoveMsgWindowTemplate).cast_mut()) as u8;
            }
            PARTY_MSG_ALREADY_HOLDING_ONE => {
                *windowPtr =
                    AddWindow((&raw const *sAlreadyHoldingOneMsgWindowTemplate).cast_mut()) as u8;
            }
            _ => {
                *windowPtr =
                    AddWindow((&raw const *sDefaultPartyMsgWindowTemplate).cast_mut()) as u8;
            }
        }
        if stringId == PARTY_MSG_CHOOSE_MON {
            if (*crate::gl!(sPartyMenuInternal)).chooseHalf() != 0 {
                stringId = PARTY_MSG_CHOOSE_MON_AND_CONFIRM;
            } else if ShouldUseChooseMonText() == 0 {
                stringId = PARTY_MSG_CHOOSE_MON_OR_CANCEL;
            }
        }
        DrawStdFrameWithCustomTileAndPalette(*windowPtr, FALSE, 0x4F, 13);
        StringExpandPlaceholders(gStringVar4.as_mut_ptr(), sActionStringTable[stringId]);
        AddTextPrinterParameterized(
            *windowPtr,
            FONT_NORMAL,
            gStringVar4.as_mut_ptr(),
            0,
            1,
            0,
            None,
        );
        ScheduleBgCopyTilemapToVram(2);
    }
}
unsafe fn ShouldUseChooseMonText() -> u8 {
    let party: *mut Pokemon = (*gPlayerParty.as_ptr()).as_mut_ptr();
    let mut numAliveMons: u8 = 0;
    if crate::gl!(gPartyMenu.action) == PARTY_ACTION_SEND_OUT {
        return TRUE;
    }
    for i in 0..(PARTY_SIZE as u8) {
        if GetMonData2(party.at(i), MON_DATA_SPECIES) != 0
            && (GetMonData2(party.at(i), MON_DATA_HP) != 0
                || GetMonData2(party.at(i), MON_DATA_IS_EGG) != 0)
        {
            numAliveMons += 1;
        }
        if numAliveMons > 1 {
            return TRUE;
        }
    }
    FALSE
}
unsafe fn DisplaySelectionWindow(windowType: u8) -> u8 {
    let mut window: WindowTemplate = zeroed();
    match windowType {
        SELECTWINDOW_ACTIONS => {
            SetWindowTemplateFields(
                &raw mut window,
                2,
                19,
                19 - (*crate::gl!(sPartyMenuInternal)).numActions * 2,
                10,
                (*crate::gl!(sPartyMenuInternal)).numActions * 2,
                14,
                0x2E9,
            );
        }
        SELECTWINDOW_ITEM => {
            window = *sItemGiveTakeWindowTemplate;
        }
        SELECTWINDOW_MAIL => {
            window = *sMailReadTakeWindowTemplate;
        }
        _ => {
            window = *sMoveSelectWindowTemplate;
        }
    }
    (*crate::gl!(sPartyMenuInternal)).windowId[0] = AddWindow(&raw mut window) as u8;
    DrawStdFrameWithCustomTileAndPalette(
        (*crate::gl!(sPartyMenuInternal)).windowId[0],
        0,
        0x4F,
        13,
    );
    if windowType == SELECTWINDOW_MOVES {
        return (*crate::gl!(sPartyMenuInternal)).windowId[0];
    }
    let cursorDimension: u8 = GetMenuCursorDimensionByFont(FONT_NORMAL, 0);
    let letterSpacing: u8 = GetFontAttribute(FONT_NORMAL, FONTATTR_LETTER_SPACING);
    let mut i: u8 = 0;
    while i < (*crate::gl!(sPartyMenuInternal)).numActions {
        let fontColorsId: u8 = (if (*crate::gl!(sPartyMenuInternal)).actions[i] >= MENU_FIELD_MOVES
        {
            4
        } else {
            3
        }) as u8;
        AddTextPrinterParameterized4(
            (*crate::gl!(sPartyMenuInternal)).windowId[0],
            FONT_NORMAL,
            cursorDimension,
            i * 16 + 1,
            letterSpacing,
            0,
            sFontColorTable[fontColorsId].as_ptr().cast_mut(),
            0,
            sCursorOptions[(*crate::gl!(sPartyMenuInternal)).actions[i]].text,
        );
        i += 1;
    }
    InitMenuInUpperLeftCorner(
        (*crate::gl!(sPartyMenuInternal)).windowId[0],
        (*crate::gl!(sPartyMenuInternal)).numActions,
        0,
        TRUE,
    );
    ScheduleBgCopyTilemapToVram(2);
    (*crate::gl!(sPartyMenuInternal)).windowId[0]
}
pub(crate) unsafe fn PrintMessage(text: *mut u8) {
    DrawStdFrameWithCustomTileAndPalette(WIN_MSG, FALSE, 0x4F, 13);
    (*(crate::gl!(&raw @at crate::text::gTextFlags;).cast_const())
        .cast::<TextFlags>()
        .cast_mut())
    .set_canABSpeedUpPrint(TRUE);
    AddTextPrinterParameterized2(
        WIN_MSG,
        FONT_NORMAL,
        text,
        GetPlayerTextSpeedDelay(),
        None,
        TEXT_COLOR_DARK_GRAY,
        TEXT_COLOR_WHITE,
        TEXT_COLOR_LIGHT_GRAY,
    );
}
unsafe fn PartyMenuDisplayYesNoMenu() {
    CreateYesNoMenu(
        (&raw const *sPartyMenuYesNoWindowTemplate).cast_mut(),
        0x4F,
        13,
        0,
    );
}
unsafe fn CreateLevelUpStatsWindow() -> u8 {
    (*crate::gl!(sPartyMenuInternal)).windowId[0] =
        AddWindow((&raw const *sLevelUpStatsWindowTemplate).cast_mut()) as u8;
    DrawStdFrameWithCustomTileAndPalette(
        (*crate::gl!(sPartyMenuInternal)).windowId[0],
        0,
        0x4F,
        13,
    );
    (*crate::gl!(sPartyMenuInternal)).windowId[0]
}
unsafe fn RemoveLevelUpStatsWindow() {
    ClearWindowTilemap((*crate::gl!(sPartyMenuInternal)).windowId[0]);
    PartyMenuRemoveWindow(&raw mut (*crate::gl!(sPartyMenuInternal)).windowId[0]);
}
unsafe fn SetPartyMonSelectionActions(mons: *mut Pokemon, slotId: u8, action: u8) {
    let mut i: u8 = 0;
    if action == ACTIONS_NONE as u8 {
        SetPartyMonFieldSelectionActions(mons, slotId);
    } else {
        (*crate::gl!(sPartyMenuInternal)).numActions = sPartyMenuActionCounts[action];
        i = 0;
        while i < (*crate::gl!(sPartyMenuInternal)).numActions {
            (*crate::gl!(sPartyMenuInternal)).actions[i] = *sPartyMenuActions[action].at(i);
            i += 1;
        }
    }
}
unsafe fn SetPartyMonFieldSelectionActions(mons: *mut Pokemon, slotId: u8) {
    let mut j: u8 = 0;
    (*crate::gl!(sPartyMenuInternal)).numActions = 0;
    AppendToList(
        (*crate::gl!(sPartyMenuInternal)).actions.as_mut_ptr(),
        &raw mut (*crate::gl!(sPartyMenuInternal)).numActions,
        MENU_SUMMARY,
    );
    for i in 0..(MAX_MON_MOVES as u8) {
        j = 0;
        while sFieldMoves[j] != FIELD_MOVES_COUNT {
            if GetMonData2(mons.at(slotId), i as i32 + MON_DATA_MOVE1) == sFieldMoves[j] as u32 {
                AppendToList(
                    (*crate::gl!(sPartyMenuInternal)).actions.as_mut_ptr(),
                    &raw mut (*crate::gl!(sPartyMenuInternal)).numActions,
                    j + MENU_FIELD_MOVES,
                );
                break;
            }
            j += 1;
        }
    }
    if InBattlePike() == 0 {
        if GetMonData2(mons.at(1), MON_DATA_SPECIES) != SPECIES_NONE as u32 {
            AppendToList(
                (*crate::gl!(sPartyMenuInternal)).actions.as_mut_ptr(),
                &raw mut (*crate::gl!(sPartyMenuInternal)).numActions,
                MENU_SWITCH,
            );
        }
        if ItemIsMail(GetMonData2(mons.at(slotId), MON_DATA_HELD_ITEM) as u16) != 0 {
            AppendToList(
                (*crate::gl!(sPartyMenuInternal)).actions.as_mut_ptr(),
                &raw mut (*crate::gl!(sPartyMenuInternal)).numActions,
                MENU_MAIL,
            );
        } else {
            AppendToList(
                (*crate::gl!(sPartyMenuInternal)).actions.as_mut_ptr(),
                &raw mut (*crate::gl!(sPartyMenuInternal)).numActions,
                MENU_ITEM,
            );
        }
    }
    AppendToList(
        (*crate::gl!(sPartyMenuInternal)).actions.as_mut_ptr(),
        &raw mut (*crate::gl!(sPartyMenuInternal)).numActions,
        MENU_CANCEL1,
    );
}
unsafe fn GetPartyMenuActionsType(mon: *mut Pokemon) -> u8 {
    let mut actionType: u32 = 0;
    match (*gPartyMenu.as_ptr()).menuType() {
        PARTY_MENU_TYPE_FIELD => {
            if InMultiPartnerRoom() == TRUE || GetMonData2(mon, MON_DATA_IS_EGG) != 0 {
                actionType = ACTIONS_SWITCH;
            } else {
                actionType = ACTIONS_NONE;
            }
        }
        PARTY_MENU_TYPE_IN_BATTLE => {
            actionType = GetPartyMenuActionsTypeInBattle(mon) as u32;
        }
        PARTY_MENU_TYPE_CHOOSE_HALF => match GetPartySlotEntryStatus(crate::gl!(gPartyMenu.slotId))
        {
            0 => {
                actionType = ACTIONS_ENTER;
            }
            1 => {
                actionType = ACTIONS_NO_ENTRY;
            }
            _ => {
                actionType = ACTIONS_SUMMARY_ONLY;
            }
        },
        PARTY_MENU_TYPE_DAYCARE => {
            actionType = (if GetMonData2(mon, MON_DATA_IS_EGG) != 0 {
                ACTIONS_SUMMARY_ONLY as i32
            } else {
                ACTIONS_STORE
            }) as u32;
        }
        PARTY_MENU_TYPE_UNION_ROOM_REGISTER => {
            actionType = ACTIONS_REGISTER;
        }
        PARTY_MENU_TYPE_UNION_ROOM_TRADE => {
            actionType = ACTIONS_TRADE;
        }
        PARTY_MENU_TYPE_SPIN_TRADE => {
            actionType = ACTIONS_SPIN_TRADE;
        }
        PARTY_MENU_TYPE_STORE_PYRAMID_HELD_ITEMS => {
            actionType = ACTIONS_TAKEITEM_TOSS;
        }
        _ => {
            actionType = ACTIONS_NONE;
        }
    }
    actionType as u8
}
unsafe fn CreateSelectionWindow(taskId: u8) -> u8 {
    let mon: *mut Pokemon = {
        let gl_i5 = crate::gl!(gPartyMenu.slotId);
        crate::gl!(&raw gPlayerParty[gl_i5])
    };
    let mut item: u16 = 0;
    GetMonNickname(mon, gStringVar1.as_mut_ptr());
    PartyMenuRemoveWindow(&raw mut (*crate::gl!(sPartyMenuInternal)).windowId[1]);
    if (*gPartyMenu.as_ptr()).menuType() != PARTY_MENU_TYPE_STORE_PYRAMID_HELD_ITEMS {
        SetPartyMonSelectionActions(
            (*gPlayerParty.as_ptr()).as_mut_ptr(),
            crate::gl!(gPartyMenu.slotId) as u8,
            GetPartyMenuActionsType(mon),
        );
        DisplaySelectionWindow(SELECTWINDOW_ACTIONS);
        DisplayPartyMenuStdMessage(PARTY_MSG_DO_WHAT_WITH_MON);
    } else {
        item = GetMonData2(mon, MON_DATA_HELD_ITEM) as u16;
        if item != ITEM_NONE {
            SetPartyMonSelectionActions(
                (*gPlayerParty.as_ptr()).as_mut_ptr(),
                crate::gl!(gPartyMenu.slotId) as u8,
                GetPartyMenuActionsType(mon),
            );
            DisplaySelectionWindow(SELECTWINDOW_ITEM);
            CopyItemName(item, (*gStringVar2.as_ptr()).as_mut_ptr());
            DisplayPartyMenuStdMessage(PARTY_MSG_ALREADY_HOLDING_ONE);
        } else {
            StringExpandPlaceholders(
                gStringVar4.as_mut_ptr(),
                (*(&raw const crate::data::strings::gText_PkmnNotHolding).cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut(),
            );
            DisplayPartyMenuMessage(gStringVar4.as_mut_ptr(), TRUE);
            ScheduleBgCopyTilemapToVram(2);
            task_set_func(taskId, Some(Task_UpdateHeldItemSprite));
            return FALSE;
        }
    }
    TRUE
}
pub(crate) unsafe fn Task_TryCreateSelectionWindow(taskId: u8) {
    if CreateSelectionWindow(taskId) != 0 {
        task_set(taskId, 0, 0xFF);
        task_set_func(taskId, Some(Task_HandleSelectionMenuInput));
    }
}
pub(crate) unsafe fn Task_HandleSelectionMenuInput(taskId: u8) {
    if (*gPaletteFade.as_ptr()).active() == 0 && MenuHelpers_ShouldWaitForLinkRecv() != TRUE {
        let mut input: i8 = 0;
        let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
        if (*crate::gl!(sPartyMenuInternal)).numActions <= 3 {
            input = Menu_ProcessInputNoWrapAround_other();
        } else {
            input = ProcessMenuInput_other();
        }
        *data = Menu_GetCursorPos() as i16;
        match input {
            MENU_NOTHING_CHOSEN => {}
            MENU_B_PRESSED => {
                PlaySE(SE_SELECT);
                PartyMenuRemoveWindow(&raw mut (*crate::gl!(sPartyMenuInternal)).windowId[2]);
                sCursorOptions[(*crate::gl!(sPartyMenuInternal)).actions
                    [(*crate::gl!(sPartyMenuInternal)).numActions as i32 - 1]]
                    .func
                    .unwrap_unchecked()(taskId);
            }
            _ => {
                PartyMenuRemoveWindow(&raw mut (*crate::gl!(sPartyMenuInternal)).windowId[2]);
                sCursorOptions[(*crate::gl!(sPartyMenuInternal)).actions[input]]
                    .func
                    .unwrap_unchecked()(taskId);
            }
        }
    }
}
pub(crate) unsafe fn CursorCb_Summary(taskId: u8) {
    PlaySE(SE_SELECT);
    (*crate::gl!(sPartyMenuInternal)).exitCallback = Some(CB2_ShowPokemonSummaryScreen);
    Task_ClosePartyMenu(taskId);
}
pub(crate) unsafe fn CB2_ShowPokemonSummaryScreen() {
    if (*gPartyMenu.as_ptr()).menuType() == PARTY_MENU_TYPE_IN_BATTLE {
        UpdatePartyToBattleOrder();
        ShowPokemonSummaryScreen(
            SUMMARY_MODE_LOCK_MOVES,
            (*gPlayerParty.as_ptr()).as_mut_ptr() as *mut c_void,
            crate::gl!(gPartyMenu.slotId) as u8,
            gPlayerPartyCount.get() - 1,
            Some(CB2_ReturnToPartyMenuFromSummaryScreen),
        );
    } else {
        ShowPokemonSummaryScreen(
            SUMMARY_MODE_NORMAL,
            (*gPlayerParty.as_ptr()).as_mut_ptr() as *mut c_void,
            crate::gl!(gPartyMenu.slotId) as u8,
            gPlayerPartyCount.get() - 1,
            Some(CB2_ReturnToPartyMenuFromSummaryScreen),
        );
    }
}
pub(crate) unsafe fn CB2_ReturnToPartyMenuFromSummaryScreen() {
    (*gPaletteFade.as_ptr()).set_bufferTransferDisabled(TRUE as u16);
    crate::gl!(gPartyMenu.slotId = gLastViewedMonIndex.get() as i8);
    InitPartyMenu(
        (*gPartyMenu.as_ptr()).menuType(),
        KEEP_PARTY_LAYOUT,
        crate::gl!(gPartyMenu.action),
        TRUE,
        PARTY_MSG_DO_WHAT_WITH_MON as u8,
        Some(Task_TryCreateSelectionWindow),
        crate::gl!(gPartyMenu.exitCallback),
    );
}
pub(crate) unsafe fn CursorCb_Switch(taskId: u8) {
    PlaySE(SE_SELECT);
    crate::gl!(gPartyMenu.action = PARTY_ACTION_SWITCH);
    PartyMenuRemoveWindow(&raw mut (*crate::gl!(sPartyMenuInternal)).windowId[1]);
    PartyMenuRemoveWindow(&raw mut (*crate::gl!(sPartyMenuInternal)).windowId[0]);
    DisplayPartyMenuStdMessage(PARTY_MSG_MOVE_TO_WHERE);
    AnimatePartySlot(crate::gl!(gPartyMenu.slotId) as u8, 1);
    crate::gl!(gPartyMenu.slotId2 = crate::gl!(gPartyMenu.slotId));
    task_set_func(taskId, Some(Task_HandleChooseMonInput));
}
unsafe fn SwitchSelectedMons(taskId: u8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    let mut windowIds: CArray<u8, 2> = zeroed();
    if crate::gl!(gPartyMenu.slotId2) == crate::gl!(gPartyMenu.slotId) {
        FinishTwoMonAction(taskId);
    } else {
        windowIds[0] = (*(*sPartyMenuBoxes.as_ptr()).at(crate::gl!(gPartyMenu.slotId))).windowId;
        *data = GetWindowAttribute(windowIds[0], WINDOW_TILEMAP_LEFT) as i16;
        *data.at(1) = GetWindowAttribute(windowIds[0], WINDOW_TILEMAP_TOP) as i16;
        *data.at(2) = GetWindowAttribute(windowIds[0], WINDOW_WIDTH) as i16;
        *data.at(3) = GetWindowAttribute(windowIds[0], WINDOW_HEIGHT) as i16;
        *data.at(8) = 0;
        if *data.at(2) == 10 {
            *data.at(10) = -1;
        } else {
            *data.at(10) = 1;
        }
        windowIds[1] = (*(*sPartyMenuBoxes.as_ptr()).at(crate::gl!(gPartyMenu.slotId2))).windowId;
        *data.at(4) = GetWindowAttribute(windowIds[1], WINDOW_TILEMAP_LEFT) as i16;
        *data.at(5) = GetWindowAttribute(windowIds[1], WINDOW_TILEMAP_TOP) as i16;
        *data.at(6) = GetWindowAttribute(windowIds[1], WINDOW_WIDTH) as i16;
        *data.at(7) = GetWindowAttribute(windowIds[1], WINDOW_HEIGHT) as i16;
        *data.at(9) = 0;
        if *data.at(6) == 10 {
            *data.at(11) = -1;
        } else {
            *data.at(11) = 1;
        }
        crate::gl!(
            sSlot1TilemapBuffer =
                Alloc(*data.at(2) as u32 * ((*data.at(3) as u32) << 1)) as *mut u16
        );
        crate::gl!(
            sSlot2TilemapBuffer =
                Alloc(*data.at(6) as u32 * ((*data.at(7) as u32) << 1)) as *mut u16
        );
        CopyToBufferFromBgTilemap(
            0,
            crate::gl!(sSlot1TilemapBuffer),
            *data as u8,
            *data.at(1) as u8,
            *data.at(2) as u8,
            *data.at(3) as u8,
        );
        CopyToBufferFromBgTilemap(
            0,
            crate::gl!(sSlot2TilemapBuffer),
            *data.at(4) as u8,
            *data.at(5) as u8,
            *data.at(6) as u8,
            *data.at(7) as u8,
        );
        ClearWindowTilemap(windowIds[0]);
        ClearWindowTilemap(windowIds[1]);
        crate::gl!(gPartyMenu.action = PARTY_ACTION_SWITCHING);
        AnimatePartySlot(crate::gl!(gPartyMenu.slotId) as u8, 1);
        AnimatePartySlot(crate::gl!(gPartyMenu.slotId2) as u8, 1);
        SlidePartyMenuBoxOneStep(taskId);
        task_set_func(taskId, Some(Task_SlideSelectedSlotsOffscreen));
    }
}
unsafe fn TryMovePartySlot(
    x: i16,
    width: i16,
    leftMove: *mut u8,
    newX: *mut u8,
    newWidth: *mut u8,
) -> u8 {
    if (x as i32 + width as i32) < 0 {
        return FALSE;
    }
    if x >= 32 {
        return FALSE;
    }
    if x < 0 {
        *leftMove = x as u8 * 255;
        *newX = 0;
        *newWidth = width as u8 + x as u8;
    } else {
        *leftMove = 0;
        *newX = x as u8;
        if x as i32 + width as i32 >= 32 {
            *newWidth = 32 - x as u8;
        } else {
            *newWidth = width as u8;
        }
    }
    TRUE
}
unsafe fn MoveAndBufferPartySlot(
    rectSrc: *mut c_void,
    x: i16,
    y: i16,
    width: i16,
    height: i16,
    dir: i16,
) {
    let mut srcX: u8 = 0;
    let mut newX: u8 = 0;
    let mut newWidth: u8 = 0;
    if TryMovePartySlot(x, width, &raw mut srcX, &raw mut newX, &raw mut newWidth) != 0 {
        FillBgTilemapBufferRect_Palette0(0, 0, newX, y as u8, newWidth, height as u8);
        if TryMovePartySlot(
            x + dir,
            width,
            &raw mut srcX,
            &raw mut newX,
            &raw mut newWidth,
        ) != 0
        {
            CopyRectToBgTilemapBufferRect(
                0,
                rectSrc,
                srcX,
                0,
                width as u8,
                height as u8,
                newX,
                y as u8,
                newWidth,
                height as u8,
                17,
                0,
                0,
            );
        }
    }
}
unsafe fn MovePartyMenuBoxSprites(menuBox: *mut PartyMenuBox, offset: i16) {
    {
        let gl_i6 = (*menuBox).pokeballSpriteId;
        crate::gl!(gSprites[gl_i6].x2 += offset * 8)
    };
    {
        let gl_i7 = (*menuBox).itemSpriteId;
        crate::gl!(gSprites[gl_i7].x2 += offset * 8)
    };
    {
        let gl_i8 = (*menuBox).monSpriteId;
        crate::gl!(gSprites[gl_i8].x2 += offset * 8)
    };
    {
        let gl_i9 = (*menuBox).statusSpriteId;
        crate::gl!(gSprites[gl_i9].x2 += offset * 8)
    };
}
unsafe fn SlidePartyMenuBoxSpritesOneStep(taskId: u8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    if *data.at(10) != 0 {
        MovePartyMenuBoxSprites(
            (*sPartyMenuBoxes.as_ptr()).at(crate::gl!(gPartyMenu.slotId)),
            *data.at(10),
        );
    }
    if *data.at(11) != 0 {
        MovePartyMenuBoxSprites(
            (*sPartyMenuBoxes.as_ptr()).at(crate::gl!(gPartyMenu.slotId2)),
            *data.at(11),
        );
    }
}
unsafe fn SlidePartyMenuBoxOneStep(taskId: u8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    if *data.at(10) != 0 {
        MoveAndBufferPartySlot(
            crate::gl!(sSlot1TilemapBuffer) as *mut c_void,
            *data + *data.at(8),
            *data.at(1),
            *data.at(2),
            *data.at(3),
            *data.at(10),
        );
    }
    if *data.at(11) != 0 {
        MoveAndBufferPartySlot(
            crate::gl!(sSlot2TilemapBuffer) as *mut c_void,
            *data.at(4) + *data.at(9),
            *data.at(5),
            *data.at(6),
            *data.at(7),
            *data.at(11),
        );
    }
    ScheduleBgCopyTilemapToVram(0);
}
pub(crate) unsafe fn Task_SlideSelectedSlotsOffscreen(taskId: u8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    let mut slidingSlotPositions: CArray<u16, 2> = zeroed();
    SlidePartyMenuBoxOneStep(taskId);
    SlidePartyMenuBoxSpritesOneStep(taskId);
    *data.at(8) += *data.at(10);
    *data.at(9) += *data.at(11);
    slidingSlotPositions[0] = *data as u16 + *data.at(8) as u16;
    slidingSlotPositions[1] = *data.at(4) as u16 + *data.at(9) as u16;
    if slidingSlotPositions[0] > 33 && slidingSlotPositions[1] > 33 {
        *data.at(10) *= -1;
        *data.at(11) *= -1;
        SwitchPartyMon();
        DisplayPartyPokemonData(crate::gl!(gPartyMenu.slotId) as u8);
        DisplayPartyPokemonData(crate::gl!(gPartyMenu.slotId2) as u8);
        PutWindowTilemap((*(*sPartyMenuBoxes.as_ptr()).at(crate::gl!(gPartyMenu.slotId))).windowId);
        PutWindowTilemap(
            (*(*sPartyMenuBoxes.as_ptr()).at(crate::gl!(gPartyMenu.slotId2))).windowId,
        );
        CopyToBufferFromBgTilemap(
            0,
            crate::gl!(sSlot1TilemapBuffer),
            *data as u8,
            *data.at(1) as u8,
            *data.at(2) as u8,
            *data.at(3) as u8,
        );
        CopyToBufferFromBgTilemap(
            0,
            crate::gl!(sSlot2TilemapBuffer),
            *data.at(4) as u8,
            *data.at(5) as u8,
            *data.at(6) as u8,
            *data.at(7) as u8,
        );
        ClearWindowTilemap(
            (*(*sPartyMenuBoxes.as_ptr()).at(crate::gl!(gPartyMenu.slotId))).windowId,
        );
        ClearWindowTilemap(
            (*(*sPartyMenuBoxes.as_ptr()).at(crate::gl!(gPartyMenu.slotId2))).windowId,
        );
        task_set_func(taskId, Some(Task_SlideSelectedSlotsOnscreen));
    }
}
pub(crate) unsafe fn Task_SlideSelectedSlotsOnscreen(taskId: u8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    SlidePartyMenuBoxOneStep(taskId);
    SlidePartyMenuBoxSpritesOneStep(taskId);
    if *data.at(10) == 0 && *data.at(11) == 0 {
        PutWindowTilemap((*(*sPartyMenuBoxes.as_ptr()).at(crate::gl!(gPartyMenu.slotId))).windowId);
        PutWindowTilemap(
            (*(*sPartyMenuBoxes.as_ptr()).at(crate::gl!(gPartyMenu.slotId2))).windowId,
        );
        ScheduleBgCopyTilemapToVram(0);
        Free(crate::gl!(sSlot1TilemapBuffer) as *mut c_void);
        Free(crate::gl!(sSlot2TilemapBuffer) as *mut c_void);
        FinishTwoMonAction(taskId);
    } else {
        *data.at(8) += *data.at(10);
        *data.at(9) += *data.at(11);
        if *data.at(8) == 0 {
            *data.at(10) = 0;
        }
        if *data.at(9) == 0 {
            *data.at(11) = 0;
        }
    }
}
unsafe fn SwitchMenuBoxSprites(spriteIdPtr1: *mut u8, spriteIdPtr2: *mut u8) {
    let spriteIdBuffer: u8 = *spriteIdPtr1;
    *spriteIdPtr1 = *spriteIdPtr2;
    *spriteIdPtr2 = spriteIdBuffer;
    let xBuffer1: u16 = ({
        let gl_i10 = *spriteIdPtr1;
        crate::gl!(gSprites[gl_i10].x)
    }) as u16;
    let yBuffer1: u16 = ({
        let gl_i11 = *spriteIdPtr1;
        crate::gl!(gSprites[gl_i11].y)
    }) as u16;
    let xBuffer2: u16 = ({
        let gl_i12 = *spriteIdPtr1;
        crate::gl!(gSprites[gl_i12].x2)
    }) as u16;
    let yBuffer2: u16 = ({
        let gl_i13 = *spriteIdPtr1;
        crate::gl!(gSprites[gl_i13].y2)
    }) as u16;
    {
        let gl_i14 = *spriteIdPtr1;
        crate::gl!(
            gSprites[gl_i14].x = ({
                let gl_i15 = *spriteIdPtr2;
                crate::gl!(gSprites[gl_i15].x)
            })
        )
    };
    {
        let gl_i16 = *spriteIdPtr1;
        crate::gl!(
            gSprites[gl_i16].y = ({
                let gl_i17 = *spriteIdPtr2;
                crate::gl!(gSprites[gl_i17].y)
            })
        )
    };
    {
        let gl_i18 = *spriteIdPtr1;
        crate::gl!(
            gSprites[gl_i18].x2 = ({
                let gl_i19 = *spriteIdPtr2;
                crate::gl!(gSprites[gl_i19].x2)
            })
        )
    };
    {
        let gl_i20 = *spriteIdPtr1;
        crate::gl!(
            gSprites[gl_i20].y2 = ({
                let gl_i21 = *spriteIdPtr2;
                crate::gl!(gSprites[gl_i21].y2)
            })
        )
    };
    {
        let gl_i22 = *spriteIdPtr2;
        crate::gl!(gSprites[gl_i22].x = xBuffer1 as i16)
    };
    {
        let gl_i23 = *spriteIdPtr2;
        crate::gl!(gSprites[gl_i23].y = yBuffer1 as i16)
    };
    {
        let gl_i24 = *spriteIdPtr2;
        crate::gl!(gSprites[gl_i24].x2 = xBuffer2 as i16)
    };
    {
        let gl_i25 = *spriteIdPtr2;
        crate::gl!(gSprites[gl_i25].y2 = yBuffer2 as i16)
    };
}
unsafe fn SwitchPartyMon() {
    let mut menuBoxes: CArray<*mut PartyMenuBox, 2> = zeroed();
    menuBoxes[0] = (*sPartyMenuBoxes.as_ptr()).at(crate::gl!(gPartyMenu.slotId));
    menuBoxes[1] = (*sPartyMenuBoxes.as_ptr()).at(crate::gl!(gPartyMenu.slotId2));
    let mon1: *mut Pokemon = {
        let gl_i26 = crate::gl!(gPartyMenu.slotId);
        crate::gl!(&raw gPlayerParty[gl_i26])
    };
    let mon2: *mut Pokemon = {
        let gl_i27 = crate::gl!(gPartyMenu.slotId2);
        crate::gl!(&raw gPlayerParty[gl_i27])
    };
    let monBuffer: *mut Pokemon = Alloc(100) as *mut Pokemon;
    *monBuffer = *mon1;
    *mon1 = *mon2;
    *mon2 = *monBuffer;
    Free(monBuffer as *mut c_void);
    SwitchMenuBoxSprites(
        &raw mut (*menuBoxes[0]).pokeballSpriteId,
        &raw mut (*menuBoxes[1]).pokeballSpriteId,
    );
    SwitchMenuBoxSprites(
        &raw mut (*menuBoxes[0]).itemSpriteId,
        &raw mut (*menuBoxes[1]).itemSpriteId,
    );
    SwitchMenuBoxSprites(
        &raw mut (*menuBoxes[0]).monSpriteId,
        &raw mut (*menuBoxes[1]).monSpriteId,
    );
    SwitchMenuBoxSprites(
        &raw mut (*menuBoxes[0]).statusSpriteId,
        &raw mut (*menuBoxes[1]).statusSpriteId,
    );
}
unsafe fn FinishTwoMonAction(taskId: u8) {
    PartyMenuRemoveWindow(&raw mut (*crate::gl!(sPartyMenuInternal)).windowId[1]);
    crate::gl!(gPartyMenu.action = PARTY_ACTION_CHOOSE_MON);
    AnimatePartySlot(crate::gl!(gPartyMenu.slotId) as u8, 0);
    crate::gl!(gPartyMenu.slotId = crate::gl!(gPartyMenu.slotId2));
    AnimatePartySlot(crate::gl!(gPartyMenu.slotId2) as u8, 1);
    DisplayPartyMenuStdMessage(PARTY_MSG_CHOOSE_MON);
    task_set_func(taskId, Some(Task_HandleChooseMonInput));
}
pub(crate) unsafe fn CursorCb_Cancel1(taskId: u8) {
    PlaySE(SE_SELECT);
    PartyMenuRemoveWindow(&raw mut (*crate::gl!(sPartyMenuInternal)).windowId[0]);
    PartyMenuRemoveWindow(&raw mut (*crate::gl!(sPartyMenuInternal)).windowId[1]);
    if (*gPartyMenu.as_ptr()).menuType() == PARTY_MENU_TYPE_DAYCARE {
        DisplayPartyMenuStdMessage(PARTY_MSG_CHOOSE_MON_2);
    } else {
        DisplayPartyMenuStdMessage(PARTY_MSG_CHOOSE_MON);
    }
    task_set_func(taskId, Some(Task_HandleChooseMonInput));
}
pub(crate) unsafe fn CursorCb_Item(taskId: u8) {
    PlaySE(SE_SELECT);
    PartyMenuRemoveWindow(&raw mut (*crate::gl!(sPartyMenuInternal)).windowId[0]);
    PartyMenuRemoveWindow(&raw mut (*crate::gl!(sPartyMenuInternal)).windowId[1]);
    SetPartyMonSelectionActions(
        (*gPlayerParty.as_ptr()).as_mut_ptr(),
        crate::gl!(gPartyMenu.slotId) as u8,
        ACTIONS_ITEM,
    );
    DisplaySelectionWindow(SELECTWINDOW_ITEM);
    DisplayPartyMenuStdMessage(PARTY_MSG_DO_WHAT_WITH_ITEM);
    task_set(taskId, 0, 0xFF);
    task_set_func(taskId, Some(Task_HandleSelectionMenuInput));
}
pub(crate) unsafe fn CursorCb_Give(taskId: u8) {
    PlaySE(SE_SELECT);
    (*crate::gl!(sPartyMenuInternal)).exitCallback = Some(CB2_SelectBagItemToGive);
    Task_ClosePartyMenu(taskId);
}
pub(crate) unsafe fn CB2_SelectBagItemToGive() {
    if (CurrentBattlePyramidLocation() != 0) as i32 == 0 {
        GoToBagMenu(
            ITEMMENULOCATION_PARTY,
            POCKETS_COUNT,
            Some(CB2_GiveHoldItem),
        );
    } else {
        GoToBattlePyramidBagMenu(PYRAMIDBAG_LOC_PARTY, Some(CB2_GiveHoldItem));
    }
}
pub(crate) unsafe fn CB2_GiveHoldItem() {
    if gSpecialVar_ItemId.get() == ITEM_NONE {
        InitPartyMenu(
            (*gPartyMenu.as_ptr()).menuType(),
            KEEP_PARTY_LAYOUT,
            crate::gl!(gPartyMenu.action),
            TRUE,
            PARTY_MSG_NONE,
            Some(Task_TryCreateSelectionWindow),
            crate::gl!(gPartyMenu.exitCallback),
        );
    } else {
        sPartyMenuItemId.set(GetMonData2(
            {
                let gl_i28 = crate::gl!(gPartyMenu.slotId);
                crate::gl!(&raw gPlayerParty[gl_i28])
            },
            MON_DATA_HELD_ITEM,
        ) as u16);
        if sPartyMenuItemId.get() != ITEM_NONE {
            InitPartyMenu(
                (*gPartyMenu.as_ptr()).menuType(),
                KEEP_PARTY_LAYOUT,
                crate::gl!(gPartyMenu.action),
                TRUE,
                PARTY_MSG_NONE,
                Some(Task_SwitchHoldItemsPrompt),
                crate::gl!(gPartyMenu.exitCallback),
            );
        } else if ItemIsMail(gSpecialVar_ItemId.get()) != 0 {
            RemoveBagItem(gSpecialVar_ItemId.get(), 1);
            GiveItemToMon(
                {
                    let gl_i29 = crate::gl!(gPartyMenu.slotId);
                    crate::gl!(&raw gPlayerParty[gl_i29])
                },
                gSpecialVar_ItemId.get(),
            );
            CB2_WriteMailToGiveMon();
        } else {
            InitPartyMenu(
                (*gPartyMenu.as_ptr()).menuType(),
                KEEP_PARTY_LAYOUT,
                crate::gl!(gPartyMenu.action),
                TRUE,
                PARTY_MSG_NONE,
                Some(Task_GiveHoldItem),
                crate::gl!(gPartyMenu.exitCallback),
            );
        }
    }
}
pub(crate) unsafe fn Task_GiveHoldItem(taskId: u8) {
    let mut item: u16 = 0;
    if (*gPaletteFade.as_ptr()).active() == 0 {
        item = gSpecialVar_ItemId.get();
        DisplayGaveHeldItemMessage(
            {
                let gl_i30 = crate::gl!(gPartyMenu.slotId);
                crate::gl!(&raw gPlayerParty[gl_i30])
            },
            item,
            0,
            0,
        );
        GiveItemToMon(
            {
                let gl_i31 = crate::gl!(gPartyMenu.slotId);
                crate::gl!(&raw gPlayerParty[gl_i31])
            },
            item,
        );
        RemoveBagItem(item, 1);
        task_set_func(taskId, Some(Task_UpdateHeldItemSprite));
    }
}
pub(crate) unsafe fn Task_SwitchHoldItemsPrompt(taskId: u8) {
    if (*gPaletteFade.as_ptr()).active() == 0 {
        DisplayAlreadyHoldingItemSwitchMessage(
            {
                let gl_i32 = crate::gl!(gPartyMenu.slotId);
                crate::gl!(&raw gPlayerParty[gl_i32])
            },
            sPartyMenuItemId.get(),
            TRUE,
        );
        task_set_func(taskId, Some(Task_SwitchItemsYesNo));
    }
}
pub(crate) unsafe fn Task_SwitchItemsYesNo(taskId: u8) {
    if IsPartyMenuTextPrinterActive() != TRUE {
        PartyMenuDisplayYesNoMenu();
        task_set_func(taskId, Some(Task_HandleSwitchItemsYesNoInput));
    }
}
pub(crate) unsafe fn Task_HandleSwitchItemsYesNoInput(taskId: u8) {
    'l1: {
        let sw1: i8 = Menu_ProcessInputNoWrapClearOnChoose();
        let mut fall = false;
        if sw1 == 0 {
            RemoveBagItem(gSpecialVar_ItemId.get(), 1);
            if AddBagItem(sPartyMenuItemId.get(), 1) == FALSE {
                AddBagItem(gSpecialVar_ItemId.get(), 1);
                BufferBagFullCantTakeItemMessage(sPartyMenuItemId.get());
                DisplayPartyMenuMessage(gStringVar4.as_mut_ptr(), FALSE);
                task_set_func(taskId, Some(Task_ReturnToChooseMonAfterText));
            } else if ItemIsMail(gSpecialVar_ItemId.get()) != 0 {
                GiveItemToMon(
                    {
                        let gl_i33 = crate::gl!(gPartyMenu.slotId);
                        crate::gl!(&raw gPlayerParty[gl_i33])
                    },
                    gSpecialVar_ItemId.get(),
                );
                task_set_func(taskId, Some(Task_WriteMailToGiveMonAfterText));
            } else {
                GiveItemToMon(
                    {
                        let gl_i34 = crate::gl!(gPartyMenu.slotId);
                        crate::gl!(&raw gPlayerParty[gl_i34])
                    },
                    gSpecialVar_ItemId.get(),
                );
                DisplaySwitchedHeldItemMessage(
                    gSpecialVar_ItemId.get(),
                    sPartyMenuItemId.get(),
                    TRUE,
                );
                task_set_func(taskId, Some(Task_UpdateHeldItemSprite));
            }
            break 'l1;
        }
        if sw1 == MENU_B_PRESSED {
            fall = true;
            PlaySE(SE_SELECT);
        }
        if fall || sw1 == 1 {
            task_set_func(taskId, Some(Task_ReturnToChooseMonAfterText));
            break 'l1;
        }
    }
}
pub(crate) unsafe fn Task_WriteMailToGiveMonAfterText(taskId: u8) {
    if IsPartyMenuTextPrinterActive() != TRUE {
        (*crate::gl!(sPartyMenuInternal)).exitCallback = Some(CB2_WriteMailToGiveMon);
        Task_ClosePartyMenu(taskId);
    }
}
pub(crate) unsafe fn CB2_WriteMailToGiveMon() {
    let mail: u8 = GetMonData2(
        {
            let gl_i35 = crate::gl!(gPartyMenu.slotId);
            crate::gl!(&raw gPlayerParty[gl_i35])
        },
        MON_DATA_MAIL,
    ) as u8;
    DoEasyChatScreen(
        EASY_CHAT_TYPE_MAIL,
        (*crate::gl!(gSaveBlock1Ptr)).mail[mail].words.as_mut_ptr(),
        Some(CB2_ReturnToPartyMenuFromWritingMail),
        EASY_CHAT_PERSON_DISPLAY_NONE,
    );
}
pub(crate) unsafe fn CB2_ReturnToPartyMenuFromWritingMail() {
    let mon: *mut Pokemon = {
        let gl_i36 = crate::gl!(gPartyMenu.slotId);
        crate::gl!(&raw gPlayerParty[gl_i36])
    };
    let item: u16 = GetMonData2(mon, MON_DATA_HELD_ITEM) as u16;
    if gSpecialVar_Result == FALSE as u16 {
        TakeMailFromMon(mon);
        SetMonData(
            mon,
            MON_DATA_HELD_ITEM,
            sPartyMenuItemId.as_ptr() as *mut c_void,
        );
        RemoveBagItem(sPartyMenuItemId.get(), 1);
        AddBagItem(item, 1);
        InitPartyMenu(
            (*gPartyMenu.as_ptr()).menuType(),
            KEEP_PARTY_LAYOUT,
            crate::gl!(gPartyMenu.action),
            TRUE,
            PARTY_MSG_CHOOSE_MON as u8,
            Some(Task_TryCreateSelectionWindow),
            crate::gl!(gPartyMenu.exitCallback),
        );
    } else {
        InitPartyMenu(
            (*gPartyMenu.as_ptr()).menuType(),
            KEEP_PARTY_LAYOUT,
            crate::gl!(gPartyMenu.action),
            TRUE,
            PARTY_MSG_NONE,
            Some(Task_DisplayGaveMailFromPartyMessage),
            crate::gl!(gPartyMenu.exitCallback),
        );
    }
}
pub(crate) unsafe fn Task_DisplayGaveMailFromPartyMessage(taskId: u8) {
    if (*gPaletteFade.as_ptr()).active() == 0 {
        if sPartyMenuItemId.get() == ITEM_NONE {
            DisplayGaveHeldItemMessage(
                {
                    let gl_i37 = crate::gl!(gPartyMenu.slotId);
                    crate::gl!(&raw gPlayerParty[gl_i37])
                },
                gSpecialVar_ItemId.get(),
                0,
                0,
            );
        } else {
            DisplaySwitchedHeldItemMessage(gSpecialVar_ItemId.get(), sPartyMenuItemId.get(), FALSE);
        }
        task_set_func(taskId, Some(Task_UpdateHeldItemSprite));
    }
}
pub(crate) unsafe fn Task_UpdateHeldItemSprite(taskId: u8) {
    let mon: *mut Pokemon = {
        let gl_i38 = crate::gl!(gPartyMenu.slotId);
        crate::gl!(&raw gPlayerParty[gl_i38])
    };
    if IsPartyMenuTextPrinterActive() != TRUE {
        UpdatePartyMonHeldItemSprite(
            mon,
            (*sPartyMenuBoxes.as_ptr()).at(crate::gl!(gPartyMenu.slotId)),
        );
        if (*gPartyMenu.as_ptr()).menuType() == PARTY_MENU_TYPE_STORE_PYRAMID_HELD_ITEMS {
            if GetMonData2(mon, MON_DATA_HELD_ITEM) != ITEM_NONE as u32 {
                DisplayPartyPokemonDescriptionText(
                    PARTYBOX_DESC_HAVE,
                    (*sPartyMenuBoxes.as_ptr()).at(crate::gl!(gPartyMenu.slotId)),
                    1,
                );
            } else {
                DisplayPartyPokemonDescriptionText(
                    PARTYBOX_DESC_DONT_HAVE,
                    (*sPartyMenuBoxes.as_ptr()).at(crate::gl!(gPartyMenu.slotId)),
                    1,
                );
            }
        }
        Task_ReturnToChooseMonAfterText(taskId);
    }
}
pub(crate) unsafe fn CursorCb_TakeItem(taskId: u8) {
    let mon: *mut Pokemon = {
        let gl_i39 = crate::gl!(gPartyMenu.slotId);
        crate::gl!(&raw gPlayerParty[gl_i39])
    };
    let item: u16 = GetMonData2(mon, MON_DATA_HELD_ITEM) as u16;
    PlaySE(SE_SELECT);
    PartyMenuRemoveWindow(&raw mut (*crate::gl!(sPartyMenuInternal)).windowId[0]);
    PartyMenuRemoveWindow(&raw mut (*crate::gl!(sPartyMenuInternal)).windowId[1]);
    match TryTakeMonItem(mon) {
        0 => {
            GetMonNickname(mon, gStringVar1.as_mut_ptr());
            StringExpandPlaceholders(
                gStringVar4.as_mut_ptr(),
                (*(&raw const crate::data::strings::gText_PkmnNotHolding).cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut(),
            );
            DisplayPartyMenuMessage(gStringVar4.as_mut_ptr(), TRUE);
        }
        1 => {
            BufferBagFullCantTakeItemMessage(item);
            DisplayPartyMenuMessage(gStringVar4.as_mut_ptr(), TRUE);
        }
        _ => {
            DisplayTookHeldItemMessage(mon, item, TRUE);
        }
    }
    ScheduleBgCopyTilemapToVram(2);
    task_set_func(taskId, Some(Task_UpdateHeldItemSprite));
}
pub(crate) unsafe fn CursorCb_Toss(taskId: u8) {
    let mon: *mut Pokemon = {
        let gl_i40 = crate::gl!(gPartyMenu.slotId);
        crate::gl!(&raw gPlayerParty[gl_i40])
    };
    let item: u16 = GetMonData2(mon, MON_DATA_HELD_ITEM) as u16;
    PlaySE(SE_SELECT);
    PartyMenuRemoveWindow(&raw mut (*crate::gl!(sPartyMenuInternal)).windowId[0]);
    PartyMenuRemoveWindow(&raw mut (*crate::gl!(sPartyMenuInternal)).windowId[1]);
    if item == ITEM_NONE {
        GetMonNickname(mon, gStringVar1.as_mut_ptr());
        StringExpandPlaceholders(
            gStringVar4.as_mut_ptr(),
            (*(&raw const crate::data::strings::gText_PkmnNotHolding).cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
        );
        DisplayPartyMenuMessage(gStringVar4.as_mut_ptr(), TRUE);
        task_set_func(taskId, Some(Task_UpdateHeldItemSprite));
    } else {
        CopyItemName(item, gStringVar1.as_mut_ptr());
        StringExpandPlaceholders(
            gStringVar4.as_mut_ptr(),
            (*(&raw const crate::data::strings::gText_ThrowAwayItem).cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
        );
        DisplayPartyMenuMessage(gStringVar4.as_mut_ptr(), TRUE);
        task_set_func(taskId, Some(Task_TossHeldItemYesNo));
    }
}
pub(crate) unsafe fn Task_TossHeldItemYesNo(taskId: u8) {
    if IsPartyMenuTextPrinterActive() != TRUE {
        PartyMenuDisplayYesNoMenu();
        task_set_func(taskId, Some(Task_HandleTossHeldItemYesNoInput));
    }
}
pub(crate) unsafe fn Task_HandleTossHeldItemYesNoInput(taskId: u8) {
    let mon: *mut Pokemon = {
        let gl_i41 = crate::gl!(gPartyMenu.slotId);
        crate::gl!(&raw gPlayerParty[gl_i41])
    };
    'l1: {
        let sw1: i8 = Menu_ProcessInputNoWrapClearOnChoose();
        let mut fall = false;
        if sw1 == 0 {
            CopyItemName(
                GetMonData2(mon, MON_DATA_HELD_ITEM) as u16,
                gStringVar1.as_mut_ptr(),
            );
            StringExpandPlaceholders(
                gStringVar4.as_mut_ptr(),
                (*(&raw const crate::data::strings::gText_ItemThrownAway).cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut(),
            );
            DisplayPartyMenuMessage(gStringVar4.as_mut_ptr(), FALSE);
            task_set_func(taskId, Some(Task_TossHeldItem));
            break 'l1;
        }
        if sw1 == MENU_B_PRESSED {
            fall = true;
            PlaySE(SE_SELECT);
        }
        if fall || sw1 == 1 {
            task_set_func(taskId, Some(Task_ReturnToChooseMonAfterText));
            break 'l1;
        }
    }
}
pub(crate) unsafe fn Task_TossHeldItem(taskId: u8) {
    let mon: *mut Pokemon = {
        let gl_i42 = crate::gl!(gPartyMenu.slotId);
        crate::gl!(&raw gPlayerParty[gl_i42])
    };
    if IsPartyMenuTextPrinterActive() != TRUE {
        let mut item: u16 = ITEM_NONE;
        SetMonData(mon, MON_DATA_HELD_ITEM, &raw mut item as *mut c_void);
        UpdatePartyMonHeldItemSprite(
            mon,
            (*sPartyMenuBoxes.as_ptr()).at(crate::gl!(gPartyMenu.slotId)),
        );
        DisplayPartyPokemonDescriptionText(
            PARTYBOX_DESC_DONT_HAVE,
            (*sPartyMenuBoxes.as_ptr()).at(crate::gl!(gPartyMenu.slotId)),
            1,
        );
        task_set_func(taskId, Some(Task_ReturnToChooseMonAfterText));
    }
}
pub(crate) unsafe fn CursorCb_Mail(taskId: u8) {
    PlaySE(SE_SELECT);
    PartyMenuRemoveWindow(&raw mut (*crate::gl!(sPartyMenuInternal)).windowId[0]);
    PartyMenuRemoveWindow(&raw mut (*crate::gl!(sPartyMenuInternal)).windowId[1]);
    SetPartyMonSelectionActions(
        (*gPlayerParty.as_ptr()).as_mut_ptr(),
        crate::gl!(gPartyMenu.slotId) as u8,
        ACTIONS_MAIL,
    );
    DisplaySelectionWindow(SELECTWINDOW_MAIL);
    DisplayPartyMenuStdMessage(PARTY_MSG_DO_WHAT_WITH_MAIL);
    task_set(taskId, 0, 0xFF);
    task_set_func(taskId, Some(Task_HandleSelectionMenuInput));
}
pub(crate) unsafe fn CursorCb_Read(taskId: u8) {
    PlaySE(SE_SELECT);
    (*crate::gl!(sPartyMenuInternal)).exitCallback = Some(CB2_ReadHeldMail);
    Task_ClosePartyMenu(taskId);
}
pub(crate) unsafe fn CB2_ReadHeldMail() {
    ReadMail(
        &raw mut (*crate::gl!(gSaveBlock1Ptr)).mail[GetMonData2(
            {
                let gl_i43 = crate::gl!(gPartyMenu.slotId);
                crate::gl!(&raw gPlayerParty[gl_i43])
            },
            MON_DATA_MAIL,
        )],
        Some(CB2_ReturnToPartyMenuFromReadingMail),
        TRUE,
    );
}
pub(crate) unsafe fn CB2_ReturnToPartyMenuFromReadingMail() {
    (*gPaletteFade.as_ptr()).set_bufferTransferDisabled(TRUE as u16);
    InitPartyMenu(
        (*gPartyMenu.as_ptr()).menuType(),
        KEEP_PARTY_LAYOUT,
        crate::gl!(gPartyMenu.action),
        TRUE,
        PARTY_MSG_DO_WHAT_WITH_MON as u8,
        Some(Task_TryCreateSelectionWindow),
        crate::gl!(gPartyMenu.exitCallback),
    );
}
pub(crate) unsafe fn CursorCb_TakeMail(taskId: u8) {
    PlaySE(SE_SELECT);
    PartyMenuRemoveWindow(&raw mut (*crate::gl!(sPartyMenuInternal)).windowId[1]);
    PartyMenuRemoveWindow(&raw mut (*crate::gl!(sPartyMenuInternal)).windowId[0]);
    DisplayPartyMenuMessage(
        (*(&raw const crate::data::strings::gText_SendMailToPC).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        TRUE,
    );
    task_set_func(taskId, Some(Task_SendMailToPCYesNo));
}
pub(crate) unsafe fn Task_SendMailToPCYesNo(taskId: u8) {
    if IsPartyMenuTextPrinterActive() != TRUE {
        PartyMenuDisplayYesNoMenu();
        task_set_func(taskId, Some(Task_HandleSendMailToPCYesNoInput));
    }
}
pub(crate) unsafe fn Task_HandleSendMailToPCYesNoInput(taskId: u8) {
    'l1: {
        let sw1: i8 = Menu_ProcessInputNoWrapClearOnChoose();
        let mut fall = false;
        if sw1 == 0 {
            if TakeMailFromMonAndSave({
                let gl_i44 = crate::gl!(gPartyMenu.slotId);
                crate::gl!(&raw gPlayerParty[gl_i44])
            }) != MAIL_NONE as u8
            {
                DisplayPartyMenuMessage(
                    (*(&raw const crate::data::strings::gText_MailSentToPC)
                        .cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut(),
                    FALSE,
                );
                task_set_func(taskId, Some(Task_UpdateHeldItemSprite));
            } else {
                DisplayPartyMenuMessage(
                    (*(&raw const crate::data::strings::gText_PCMailboxFull)
                        .cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut(),
                    FALSE,
                );
                task_set_func(taskId, Some(Task_ReturnToChooseMonAfterText));
            }
            break 'l1;
        }
        if sw1 == MENU_B_PRESSED {
            fall = true;
            PlaySE(SE_SELECT);
        }
        if fall || sw1 == 1 {
            DisplayPartyMenuMessage(
                (*(&raw const crate::data::strings::gText_MailMessageWillBeLost)
                    .cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
                TRUE,
            );
            task_set_func(taskId, Some(Task_LoseMailMessageYesNo));
            break 'l1;
        }
    }
}
pub(crate) unsafe fn Task_LoseMailMessageYesNo(taskId: u8) {
    if IsPartyMenuTextPrinterActive() != TRUE {
        PartyMenuDisplayYesNoMenu();
        task_set_func(taskId, Some(Task_HandleLoseMailMessageYesNoInput));
    }
}
pub(crate) unsafe fn Task_HandleLoseMailMessageYesNoInput(taskId: u8) {
    let mut item: u16 = 0;
    'l1: {
        let sw1: i8 = Menu_ProcessInputNoWrapClearOnChoose();
        let mut fall = false;
        if sw1 == 0 {
            item = GetMonData2(
                {
                    let gl_i45 = crate::gl!(gPartyMenu.slotId);
                    crate::gl!(&raw gPlayerParty[gl_i45])
                },
                MON_DATA_HELD_ITEM,
            ) as u16;
            if AddBagItem(item, 1) == 1 {
                TakeMailFromMon({
                    let gl_i46 = crate::gl!(gPartyMenu.slotId);
                    crate::gl!(&raw gPlayerParty[gl_i46])
                });
                DisplayPartyMenuMessage(
                    (*(&raw const crate::data::strings::gText_MailTakenFromPkmn)
                        .cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut(),
                    FALSE,
                );
                task_set_func(taskId, Some(Task_UpdateHeldItemSprite));
            } else {
                BufferBagFullCantTakeItemMessage(item);
                DisplayPartyMenuMessage(gStringVar4.as_mut_ptr(), FALSE);
                task_set_func(taskId, Some(Task_ReturnToChooseMonAfterText));
            }
            break 'l1;
        }
        if sw1 == MENU_B_PRESSED {
            fall = true;
            PlaySE(SE_SELECT);
        }
        if fall || sw1 == 1 {
            task_set_func(taskId, Some(Task_ReturnToChooseMonAfterText));
            break 'l1;
        }
    }
}
pub(crate) unsafe fn CursorCb_Cancel2(taskId: u8) {
    let mon: *mut Pokemon = {
        let gl_i47 = crate::gl!(gPartyMenu.slotId);
        crate::gl!(&raw gPlayerParty[gl_i47])
    };
    PlaySE(SE_SELECT);
    PartyMenuRemoveWindow(&raw mut (*crate::gl!(sPartyMenuInternal)).windowId[0]);
    PartyMenuRemoveWindow(&raw mut (*crate::gl!(sPartyMenuInternal)).windowId[1]);
    SetPartyMonSelectionActions(
        (*gPlayerParty.as_ptr()).as_mut_ptr(),
        crate::gl!(gPartyMenu.slotId) as u8,
        GetPartyMenuActionsType(mon),
    );
    if (*gPartyMenu.as_ptr()).menuType() != PARTY_MENU_TYPE_STORE_PYRAMID_HELD_ITEMS {
        DisplaySelectionWindow(SELECTWINDOW_ACTIONS);
        DisplayPartyMenuStdMessage(PARTY_MSG_DO_WHAT_WITH_MON);
    } else {
        DisplaySelectionWindow(SELECTWINDOW_ITEM);
        CopyItemName(
            GetMonData2(mon, MON_DATA_HELD_ITEM) as u16,
            (*gStringVar2.as_ptr()).as_mut_ptr(),
        );
        DisplayPartyMenuStdMessage(PARTY_MSG_ALREADY_HOLDING_ONE);
    }
    task_set(taskId, 0, 0xFF);
    task_set_func(taskId, Some(Task_HandleSelectionMenuInput));
}
pub(crate) unsafe fn CursorCb_SendMon(taskId: u8) {
    PlaySE(SE_SELECT);
    PartyMenuRemoveWindow(&raw mut (*crate::gl!(sPartyMenuInternal)).windowId[0]);
    if TrySwitchInPokemon() == TRUE {
        Task_ClosePartyMenu(taskId);
    } else {
        PartyMenuRemoveWindow(&raw mut (*crate::gl!(sPartyMenuInternal)).windowId[1]);
        DisplayPartyMenuMessage(gStringVar4.as_mut_ptr(), TRUE);
        task_set_func(taskId, Some(Task_ReturnToChooseMonAfterText));
    }
}
pub(crate) unsafe fn CursorCb_Enter(taskId: u8) {
    PartyMenuRemoveWindow(&raw mut (*crate::gl!(sPartyMenuInternal)).windowId[0]);
    PartyMenuRemoveWindow(&raw mut (*crate::gl!(sPartyMenuInternal)).windowId[1]);
    let maxBattlers: u8 = GetMaxBattleEntries();
    for i in 0..maxBattlers {
        if crate::gl!(gSelectedOrderFromParty[i]) == 0 {
            PlaySE(SE_SELECT);
            crate::gl!(gSelectedOrderFromParty[i] = crate::gl!(gPartyMenu.slotId) as u8 + 1);
            DisplayPartyPokemonDescriptionText(
                i + PARTYBOX_DESC_FIRST,
                (*sPartyMenuBoxes.as_ptr()).at(crate::gl!(gPartyMenu.slotId)),
                1,
            );
            if i as i32 == maxBattlers as i32 - 1 {
                MoveCursorToConfirm();
            }
            DisplayPartyMenuStdMessage(PARTY_MSG_CHOOSE_MON);
            task_set_func(taskId, Some(Task_HandleChooseMonInput));
            return;
        }
    }
    ConvertIntToDecimalStringN(
        gStringVar1.as_mut_ptr(),
        maxBattlers as i32,
        STR_CONV_MODE_LEFT_ALIGN,
        1,
    );
    StringExpandPlaceholders(
        gStringVar4.as_mut_ptr(),
        (*(&raw const crate::data::strings::gText_NoMoreThanVar1Pkmn).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
    );
    PlaySE(SE_FAILURE);
    DisplayPartyMenuMessage(gStringVar4.as_mut_ptr(), TRUE);
    task_set_func(taskId, Some(Task_ReturnToChooseMonAfterText));
}
unsafe fn MoveCursorToConfirm() {
    AnimatePartySlot(crate::gl!(gPartyMenu.slotId) as u8, 0);
    crate::gl!(gPartyMenu.slotId = PARTY_SIZE as i8);
    AnimatePartySlot(crate::gl!(gPartyMenu.slotId) as u8, 1);
}
pub(crate) unsafe fn CursorCb_NoEntry(taskId: u8) {
    let mut j: u8 = 0;
    PlaySE(SE_SELECT);
    PartyMenuRemoveWindow(&raw mut (*crate::gl!(sPartyMenuInternal)).windowId[0]);
    PartyMenuRemoveWindow(&raw mut (*crate::gl!(sPartyMenuInternal)).windowId[1]);
    let maxBattlers: u8 = GetMaxBattleEntries();
    for i in 0..maxBattlers {
        if crate::gl!(gSelectedOrderFromParty[i]) as i32 == crate::gl!(gPartyMenu.slotId) as i32 + 1
        {
            j = i;
            while (j as i32) < maxBattlers as i32 - 1 {
                crate::gl!(
                    gSelectedOrderFromParty[j] = {
                        let gl_i1 = j as i32 + 1;
                        crate::gl!(gSelectedOrderFromParty[gl_i1])
                    }
                );
                j += 1;
            }
            crate::gl!(gSelectedOrderFromParty[j] = 0);
            break;
        }
    }
    DisplayPartyPokemonDescriptionText(
        1,
        (*sPartyMenuBoxes.as_ptr()).at(crate::gl!(gPartyMenu.slotId)),
        1,
    );
    let mut i: u8 = 0;
    while (i as i32) < maxBattlers as i32 - 1 {
        if crate::gl!(gSelectedOrderFromParty[i]) != 0 {
            DisplayPartyPokemonDescriptionText(
                i + PARTYBOX_DESC_FIRST,
                (*sPartyMenuBoxes.as_ptr()).at(crate::gl!(gSelectedOrderFromParty[i]) as i32 - 1),
                1,
            );
        }
        i += 1;
    }
    DisplayPartyMenuStdMessage(PARTY_MSG_CHOOSE_MON);
    task_set_func(taskId, Some(Task_HandleChooseMonInput));
}
pub(crate) unsafe fn CursorCb_Store(taskId: u8) {
    PlaySE(SE_SELECT);
    Task_ClosePartyMenu(taskId);
}
pub(crate) unsafe fn CursorCb_Register(taskId: u8) {
    let species2: u16 = GetMonData2(
        {
            let gl_i48 = crate::gl!(gPartyMenu.slotId);
            crate::gl!(&raw gPlayerParty[gl_i48])
        },
        MON_DATA_SPECIES_OR_EGG,
    ) as u16;
    let species: u16 = GetMonData2(
        {
            let gl_i49 = crate::gl!(gPartyMenu.slotId);
            crate::gl!(&raw gPlayerParty[gl_i49])
        },
        MON_DATA_SPECIES,
    ) as u16;
    let isModernFatefulEncounter: u8 = GetMonData2(
        {
            let gl_i50 = crate::gl!(gPartyMenu.slotId);
            crate::gl!(&raw gPlayerParty[gl_i50])
        },
        MON_DATA_MODERN_FATEFUL_ENCOUNTER,
    ) as u8;
    match CanRegisterMonForTradingBoard(
        *(GetHostRfuGameData() as *mut RfuGameCompatibilityData),
        species2,
        species,
        isModernFatefulEncounter,
    ) {
        CANT_REGISTER_MON => {
            StringExpandPlaceholders(
                gStringVar4.as_mut_ptr(),
                (*(&raw const crate::data::strings::gText_PkmnCantBeTradedNow)
                    .cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
            );
        }
        CANT_REGISTER_EGG => {
            StringExpandPlaceholders(
                gStringVar4.as_mut_ptr(),
                (*(&raw const crate::data::strings::gText_EggCantBeTradedNow)
                    .cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
            );
        }
        _ => {
            PlaySE(SE_SELECT);
            Task_ClosePartyMenu(taskId);
            return;
        }
    }
    PlaySE(SE_FAILURE);
    PartyMenuRemoveWindow(&raw mut (*crate::gl!(sPartyMenuInternal)).windowId[0]);
    PartyMenuRemoveWindow(&raw mut (*crate::gl!(sPartyMenuInternal)).windowId[1]);
    StringAppend(
        gStringVar4.as_mut_ptr(),
        (*(&raw const crate::data::strings::gText_PauseUntilPress).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
    );
    DisplayPartyMenuMessage(gStringVar4.as_mut_ptr(), TRUE);
    task_set_func(taskId, Some(Task_ReturnToChooseMonAfterText));
}
pub(crate) unsafe fn CursorCb_Trade1(taskId: u8) {
    let species2: u16 = GetMonData2(
        {
            let gl_i51 = crate::gl!(gPartyMenu.slotId);
            crate::gl!(&raw gPlayerParty[gl_i51])
        },
        MON_DATA_SPECIES_OR_EGG,
    ) as u16;
    let species: u16 = GetMonData2(
        {
            let gl_i52 = crate::gl!(gPartyMenu.slotId);
            crate::gl!(&raw gPlayerParty[gl_i52])
        },
        MON_DATA_SPECIES,
    ) as u16;
    let isModernFatefulEncounter: u8 = GetMonData2(
        {
            let gl_i53 = crate::gl!(gPartyMenu.slotId);
            crate::gl!(&raw gPlayerParty[gl_i53])
        },
        MON_DATA_MODERN_FATEFUL_ENCOUNTER,
    ) as u8;
    let stringId: u32 = GetUnionRoomTradeMessageId(
        *(GetHostRfuGameData() as *mut RfuGameCompatibilityData),
        crate::gl!(gRfuPartnerCompatibilityData),
        species2,
        gUnionRoomOfferedSpecies.get(),
        gUnionRoomRequestedMonType.get(),
        species,
        isModernFatefulEncounter,
    ) as u32;
    if stringId != UR_TRADE_MSG_NONE {
        StringExpandPlaceholders(
            gStringVar4.as_mut_ptr(),
            sUnionRoomTradeMessages[stringId - 1],
        );
        PlaySE(SE_FAILURE);
        PartyMenuRemoveWindow(&raw mut (*crate::gl!(sPartyMenuInternal)).windowId[0]);
        PartyMenuRemoveWindow(&raw mut (*crate::gl!(sPartyMenuInternal)).windowId[1]);
        StringAppend(
            gStringVar4.as_mut_ptr(),
            (*(&raw const crate::data::strings::gText_PauseUntilPress).cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
        );
        DisplayPartyMenuMessage(gStringVar4.as_mut_ptr(), TRUE);
        task_set_func(taskId, Some(Task_ReturnToChooseMonAfterText));
    } else {
        PlaySE(SE_SELECT);
        Task_ClosePartyMenu(taskId);
    }
}
pub(crate) unsafe fn CursorCb_Trade2(taskId: u8) {
    PartyMenuRemoveWindow(&raw mut (*crate::gl!(sPartyMenuInternal)).windowId[0]);
    PartyMenuRemoveWindow(&raw mut (*crate::gl!(sPartyMenuInternal)).windowId[1]);
    match CanSpinTradeMon(
        (*gPlayerParty.as_ptr()).as_mut_ptr(),
        crate::gl!(gPartyMenu.slotId) as u16,
    ) {
        CANT_TRADE_LAST_MON => {
            StringExpandPlaceholders(
                gStringVar4.as_mut_ptr(),
                (*(&raw const crate::data::strings::gText_OnlyPkmnForBattle)
                    .cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
            );
        }
        CANT_TRADE_NATIONAL => {
            StringExpandPlaceholders(
                gStringVar4.as_mut_ptr(),
                (*(&raw const crate::data::strings::gText_PkmnCantBeTradedNow)
                    .cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
            );
        }
        CANT_TRADE_EGG_YET => {
            StringExpandPlaceholders(
                gStringVar4.as_mut_ptr(),
                (*(&raw const crate::data::strings::gText_EggCantBeTradedNow)
                    .cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
            );
        }
        _ => {
            PlaySE(SE_SELECT);
            GetMonNickname(
                {
                    let gl_i54 = crate::gl!(gPartyMenu.slotId);
                    crate::gl!(&raw gPlayerParty[gl_i54])
                },
                gStringVar1.as_mut_ptr(),
            );
            StringExpandPlaceholders(
                gStringVar4.as_mut_ptr(),
                (*(&raw const crate::data::strings::gJPText_AreYouSureYouWantToSpinTradeMon)
                    .cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
            );
            DisplayPartyMenuMessage(gStringVar4.as_mut_ptr(), TRUE);
            task_set_func(taskId, Some(Task_SpinTradeYesNo));
            return;
        }
    }
    PlaySE(SE_FAILURE);
    StringAppend(
        gStringVar4.as_mut_ptr(),
        (*(&raw const crate::data::strings::gText_PauseUntilPress).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
    );
    DisplayPartyMenuMessage(gStringVar4.as_mut_ptr(), TRUE);
    task_set_func(taskId, Some(Task_ReturnToChooseMonAfterText));
}
pub(crate) unsafe fn Task_SpinTradeYesNo(taskId: u8) {
    if IsPartyMenuTextPrinterActive() != TRUE {
        PartyMenuDisplayYesNoMenu();
        task_set_func(taskId, Some(Task_HandleSpinTradeYesNoInput));
    }
}
pub(crate) unsafe fn Task_HandleSpinTradeYesNoInput(taskId: u8) {
    'l1: {
        let sw1: i8 = Menu_ProcessInputNoWrapClearOnChoose();
        let mut fall = false;
        if sw1 == 0 {
            Task_ClosePartyMenu(taskId);
            break 'l1;
        }
        if sw1 == MENU_B_PRESSED {
            fall = true;
            PlaySE(SE_SELECT);
        }
        if fall || sw1 == 1 {
            Task_ReturnToChooseMonAfterText(taskId);
            break 'l1;
        }
    }
}
pub(crate) unsafe fn CursorCb_FieldMove(taskId: u8) {
    let fieldMove: u8 =
        (*crate::gl!(sPartyMenuInternal)).actions[Menu_GetCursorPos()] - MENU_FIELD_MOVES;
    let mut mapHeader: *mut MapHeader = null_mut();
    PlaySE(SE_SELECT);
    if sFieldMoveCursorCallbacks[fieldMove].fieldMoveFunc.is_none() {
        return;
    }
    PartyMenuRemoveWindow(&raw mut (*crate::gl!(sPartyMenuInternal)).windowId[0]);
    PartyMenuRemoveWindow(&raw mut (*crate::gl!(sPartyMenuInternal)).windowId[1]);
    if MenuHelpers_IsLinkActive() == TRUE || InUnionRoom() == TRUE as u32 {
        if fieldMove == FIELD_MOVE_MILK_DRINK || fieldMove == FIELD_MOVE_SOFT_BOILED {
            DisplayPartyMenuStdMessage(PARTY_MSG_CANT_USE_HERE);
        } else {
            DisplayPartyMenuStdMessage(sFieldMoveCursorCallbacks[fieldMove].msgId as u32);
        }
        task_set_func(taskId, Some(Task_CancelAfterAorBPress));
    } else {
        if fieldMove <= FIELD_MOVE_WATERFALL
            && FlagGet(FLAG_BADGE01_GET as u16 + fieldMove as u16) != TRUE
        {
            DisplayPartyMenuMessage(
                (*(&raw const crate::data::strings::gText_CantUseUntilNewBadge)
                    .cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
                TRUE,
            );
            task_set_func(taskId, Some(Task_ReturnToChooseMonAfterText));
        } else if sFieldMoveCursorCallbacks[fieldMove]
            .fieldMoveFunc
            .unwrap_unchecked()()
            == TRUE
        {
            match fieldMove {
                FIELD_MOVE_MILK_DRINK | FIELD_MOVE_SOFT_BOILED => {
                    ChooseMonForSoftboiled(taskId);
                }
                FIELD_MOVE_TELEPORT => {
                    mapHeader = Overworld_GetMapHeaderByGroupAndId(
                        (*crate::gl!(gSaveBlock1Ptr)).lastHealLocation.mapGroup as u16,
                        (*crate::gl!(gSaveBlock1Ptr)).lastHealLocation.mapNum as u16,
                    );
                    GetMapNameGeneric(
                        gStringVar1.as_mut_ptr(),
                        (*mapHeader).regionMapSectionId as u16,
                    );
                    StringExpandPlaceholders(
                        gStringVar4.as_mut_ptr(),
                        (*(&raw const crate::data::strings::gText_ReturnToHealingSpot)
                            .cast::<CArray<u8, 0>>())
                        .as_ptr()
                        .cast_mut(),
                    );
                    DisplayFieldMoveExitAreaMessage(taskId);
                    (*crate::gl!(sPartyMenuInternal)).data[0] = fieldMove as i16;
                }
                FIELD_MOVE_DIG => {
                    mapHeader = Overworld_GetMapHeaderByGroupAndId(
                        (*crate::gl!(gSaveBlock1Ptr)).escapeWarp.mapGroup as u16,
                        (*crate::gl!(gSaveBlock1Ptr)).escapeWarp.mapNum as u16,
                    );
                    GetMapNameGeneric(
                        gStringVar1.as_mut_ptr(),
                        (*mapHeader).regionMapSectionId as u16,
                    );
                    StringExpandPlaceholders(
                        gStringVar4.as_mut_ptr(),
                        (*(&raw const crate::data::strings::gText_EscapeFromHere)
                            .cast::<CArray<u8, 0>>())
                        .as_ptr()
                        .cast_mut(),
                    );
                    DisplayFieldMoveExitAreaMessage(taskId);
                    (*crate::gl!(sPartyMenuInternal)).data[0] = fieldMove as i16;
                }
                FIELD_MOVE_FLY => {
                    crate::gl!(gPartyMenu.exitCallback = Some(CB2_OpenFlyMap));
                    Task_ClosePartyMenu(taskId);
                }
                _ => {
                    crate::gl!(gPartyMenu.exitCallback = Some(CB2_ReturnToField));
                    Task_ClosePartyMenu(taskId);
                }
            }
        } else {
            match fieldMove {
                FIELD_MOVE_SURF => {
                    DisplayCantUseSurfMessage();
                }
                FIELD_MOVE_FLASH => {
                    DisplayCantUseFlashMessage();
                }
                _ => {
                    DisplayPartyMenuStdMessage(sFieldMoveCursorCallbacks[fieldMove].msgId as u32);
                }
            }
            task_set_func(taskId, Some(Task_CancelAfterAorBPress));
        }
    }
}
unsafe fn DisplayFieldMoveExitAreaMessage(taskId: u8) {
    DisplayPartyMenuMessage(gStringVar4.as_mut_ptr(), TRUE);
    task_set_func(taskId, Some(Task_FieldMoveExitAreaYesNo));
}
pub(crate) unsafe fn Task_FieldMoveExitAreaYesNo(taskId: u8) {
    if IsPartyMenuTextPrinterActive() != TRUE {
        PartyMenuDisplayYesNoMenu();
        task_set_func(taskId, Some(Task_HandleFieldMoveExitAreaYesNoInput));
    }
}
pub(crate) unsafe fn Task_HandleFieldMoveExitAreaYesNoInput(taskId: u8) {
    'l1: {
        let sw1: i8 = Menu_ProcessInputNoWrapClearOnChoose();
        let mut fall = false;
        if sw1 == 0 {
            crate::gl!(gPartyMenu.exitCallback = Some(CB2_ReturnToField));
            Task_ClosePartyMenu(taskId);
            break 'l1;
        }
        if sw1 == MENU_B_PRESSED {
            fall = true;
            PlaySE(SE_SELECT);
        }
        if fall || sw1 == 1 {
            crate::gl!(gFieldCallback2 = None);
            crate::gl!(gPostMenuFieldCallback = None);
            Task_ReturnToChooseMonAfterText(taskId);
            break 'l1;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe fn FieldCallback_PrepareFadeInFromMenu() -> u8 {
    FadeInFromBlack();
    CreateTask(Some(Task_FieldMoveWaitForFade), 8);
    TRUE
}
pub(crate) unsafe fn Task_FieldMoveWaitForFade(taskId: u8) {
    if IsWeatherNotFadingIn() == TRUE {
        crate::gl!(gFieldEffectArguments[0] = GetFieldMoveMonSpecies() as i32);
        (*gPostMenuFieldCallback.as_ptr()).unwrap_unchecked()();
        DestroyTask(taskId);
    }
}
unsafe fn GetFieldMoveMonSpecies() -> u16 {
    GetMonData2(
        {
            let gl_i55 = crate::gl!(gPartyMenu.slotId);
            crate::gl!(&raw gPlayerParty[gl_i55])
        },
        MON_DATA_SPECIES,
    ) as u16
}
pub(crate) unsafe fn Task_CancelAfterAorBPress(taskId: u8) {
    if crate::gl!(gMain.newKeys) as i32 & A_BUTTON != 0
        || crate::gl!(gMain.newKeys) as i32 & B_BUTTON != 0
    {
        CursorCb_Cancel1(taskId);
    }
}
unsafe fn DisplayCantUseFlashMessage() {
    if FlagGet(FLAG_SYS_USE_FLASH) == TRUE {
        DisplayPartyMenuStdMessage(PARTY_MSG_ALREADY_IN_USE);
    } else {
        DisplayPartyMenuStdMessage(PARTY_MSG_CANT_USE_HERE);
    }
}
pub(crate) unsafe fn FieldCallback_Surf() {
    crate::gl!(gFieldEffectArguments[0] = GetCursorSelectionMonId() as i32);
    FieldEffectStart(FLDEFF_USE_SURF);
}
pub(crate) unsafe fn SetUpFieldMove_Surf() -> u8 {
    if PartyHasMonWithSurf() == TRUE && IsPlayerFacingSurfableFishableWater() == TRUE {
        crate::gl!(gFieldCallback2 = Some(FieldCallback_PrepareFadeInFromMenu));
        crate::gl!(gPostMenuFieldCallback = Some(FieldCallback_Surf));
        return TRUE;
    }
    FALSE
}
unsafe fn DisplayCantUseSurfMessage() {
    if TestPlayerAvatarFlags(PLAYER_AVATAR_FLAG_SURFING) != 0 {
        DisplayPartyMenuStdMessage(PARTY_MSG_ALREADY_SURFING);
    } else {
        DisplayPartyMenuStdMessage(PARTY_MSG_CANT_SURF_HERE);
    }
}
pub(crate) fn SetUpFieldMove_Fly() -> u8 {
    if Overworld_MapTypeAllowsTeleportAndFly(crate::gl!(gMapHeader.mapType)) == TRUE {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
pub unsafe fn CB2_ReturnToPartyMenuFromFlyMap() {
    InitPartyMenu(
        0,
        0,
        0,
        TRUE,
        0,
        Some(Task_HandleChooseMonInput),
        Some(CB2_ReturnToFieldWithOpenMenu),
    );
}
pub(crate) unsafe fn FieldCallback_Waterfall() {
    crate::gl!(gFieldEffectArguments[0] = GetCursorSelectionMonId() as i32);
    FieldEffectStart(FLDEFF_USE_WATERFALL);
}
pub(crate) unsafe fn SetUpFieldMove_Waterfall() -> u8 {
    let mut x: i16 = 0;
    let mut y: i16 = 0;
    GetXYCoordsOneStepInFrontOfPlayer(&raw mut x, &raw mut y);
    if MetatileBehavior_IsWaterfall(MapGridGetMetatileBehaviorAt(x as i32, y as i32) as u8) == TRUE
        && IsPlayerSurfingNorth() == TRUE
    {
        crate::gl!(gFieldCallback2 = Some(FieldCallback_PrepareFadeInFromMenu));
        crate::gl!(gPostMenuFieldCallback = Some(FieldCallback_Waterfall));
        return TRUE;
    }
    FALSE
}
pub(crate) unsafe fn FieldCallback_Dive() {
    crate::gl!(gFieldEffectArguments[0] = GetCursorSelectionMonId() as i32);
    FieldEffectStart(FLDEFF_USE_DIVE);
}
pub(crate) unsafe fn SetUpFieldMove_Dive() -> u8 {
    crate::gl!(gFieldEffectArguments[1] = TrySetDiveWarp() as i32);
    if crate::gl!(gFieldEffectArguments[1]) != 0 {
        crate::gl!(gFieldCallback2 = Some(FieldCallback_PrepareFadeInFromMenu));
        crate::gl!(gPostMenuFieldCallback = Some(FieldCallback_Dive));
        return TRUE;
    }
    FALSE
}
unsafe fn CreatePartyMonIconSprite(mon: *mut Pokemon, menuBox: *mut PartyMenuBox, slot: u32) {
    let mut handleDeoxys: u32 = TRUE as u32;
    if IsMultiBattle() == TRUE && (*gMain.as_ptr()).inBattle() != 0 {
        handleDeoxys = (if sMultiBattlePartnersPartyMask[slot] as u32 ^ handleDeoxys != 0 {
            TRUE as i32
        } else {
            FALSE as i32
        }) as u32;
    }
    let species2: u16 = GetMonData2(mon, MON_DATA_SPECIES_OR_EGG) as u16;
    CreatePartyMonIconSpriteParameterized(
        species2,
        GetMonData2(mon, MON_DATA_PERSONALITY),
        menuBox,
        1,
        handleDeoxys,
    );
    UpdatePartyMonHPBar((*menuBox).monSpriteId, mon);
}
unsafe fn CreatePartyMonIconSpriteParameterized(
    species: u16,
    pid: u32,
    menuBox: *mut PartyMenuBox,
    priority: u8,
    handleDeoxys: u32,
) {
    if species != SPECIES_NONE {
        (*menuBox).monSpriteId = CreateMonIcon(
            species,
            Some(SpriteCB_MonIcon),
            *(*menuBox).spriteCoords as i16,
            *(*menuBox).spriteCoords.at(1) as i16,
            4,
            pid,
            handleDeoxys,
        );
        (*gSprites.as_ptr())[(*menuBox).monSpriteId]
            .oam
            .set_priority(priority as u16);
    }
}
unsafe fn UpdateHPBar(spriteId: u8, hp: u16, maxhp: u16) {
    match GetHPBarLevel(hp as i16, maxhp as i16) {
        HP_BAR_FULL => {
            SetPartyHPBarSprite(crate::gl!(&raw gSprites[spriteId]), 0);
        }
        HP_BAR_GREEN => {
            SetPartyHPBarSprite(crate::gl!(&raw gSprites[spriteId]), 1);
        }
        HP_BAR_YELLOW => {
            SetPartyHPBarSprite(crate::gl!(&raw gSprites[spriteId]), 2);
        }
        HP_BAR_RED => {
            SetPartyHPBarSprite(crate::gl!(&raw gSprites[spriteId]), 3);
        }
        _ => {
            SetPartyHPBarSprite(crate::gl!(&raw gSprites[spriteId]), 4);
        }
    }
}
unsafe fn UpdatePartyMonHPBar(spriteId: u8, mon: *mut Pokemon) {
    UpdateHPBar(
        spriteId,
        GetMonData2(mon, MON_DATA_HP) as u16,
        GetMonData2(mon, MON_DATA_MAX_HP) as u16,
    );
}
unsafe fn AnimateSelectedPartyIcon(spriteId: u8, animNum: u8) {
    crate::gl!(gSprites[spriteId].data[0] = 0);
    if animNum == 0 {
        if crate::gl!(gSprites[spriteId].x) == 16 {
            crate::gl!(gSprites[spriteId].x2 = 0);
            crate::gl!(gSprites[spriteId].y2 = -4);
        } else {
            crate::gl!(gSprites[spriteId].x2 = -4);
            crate::gl!(gSprites[spriteId].y2 = 0);
        }
        crate::gl!(gSprites[spriteId].callback = Some(SpriteCB_UpdatePartyMonIcon));
    } else {
        crate::gl!(gSprites[spriteId].x2 = 0);
        crate::gl!(gSprites[spriteId].y2 = 0);
        crate::gl!(gSprites[spriteId].callback = Some(SpriteCB_BouncePartyMonIcon));
    }
}
pub(crate) unsafe fn SpriteCB_BouncePartyMonIcon(sprite: *mut Sprite) {
    let animCmd: u8 = UpdateMonIconFrame(sprite);
    if animCmd != 0 {
        if animCmd as i32 & 1 != 0 {
            (*sprite).y2 = -3;
        } else {
            (*sprite).y2 = 1;
        }
    }
}
pub(crate) unsafe fn SpriteCB_UpdatePartyMonIcon(sprite: *mut Sprite) {
    UpdateMonIconFrame(sprite);
}
unsafe fn CreatePartyMonHeldItemSprite(mon: *mut Pokemon, menuBox: *mut PartyMenuBox) {
    if GetMonData2(mon, MON_DATA_SPECIES) != SPECIES_NONE as u32 {
        (*menuBox).itemSpriteId = CreateSprite(
            (&raw const *sSpriteTemplate_HeldItem).cast_mut(),
            *(*menuBox).spriteCoords.at(2) as i16,
            *(*menuBox).spriteCoords.at(3) as i16,
            0,
        );
        UpdatePartyMonHeldItemSprite(mon, menuBox);
    }
}
unsafe fn CreatePartyMonHeldItemSpriteParameterized(
    species: u16,
    item: u16,
    menuBox: *mut PartyMenuBox,
) {
    if species != SPECIES_NONE {
        (*menuBox).itemSpriteId = CreateSprite(
            (&raw const *sSpriteTemplate_HeldItem).cast_mut(),
            *(*menuBox).spriteCoords.at(2) as i16,
            *(*menuBox).spriteCoords.at(3) as i16,
            0,
        );
        (*gSprites.as_ptr())[(*menuBox).itemSpriteId]
            .oam
            .set_priority(0);
        ShowOrHideHeldItemSprite(item, menuBox);
    }
}
unsafe fn UpdatePartyMonHeldItemSprite(mon: *mut Pokemon, menuBox: *mut PartyMenuBox) {
    ShowOrHideHeldItemSprite(GetMonData2(mon, MON_DATA_HELD_ITEM) as u16, menuBox);
}
unsafe fn ShowOrHideHeldItemSprite(item: u16, menuBox: *mut PartyMenuBox) {
    if item == ITEM_NONE {
        (*gSprites.as_ptr())[(*menuBox).itemSpriteId].set_invisible(TRUE as u16);
    } else {
        if ItemIsMail(item) != 0 {
            StartSpriteAnim(
                {
                    let gl_i56 = (*menuBox).itemSpriteId;
                    crate::gl!(&raw gSprites[gl_i56])
                },
                1,
            );
        } else {
            StartSpriteAnim(
                {
                    let gl_i57 = (*menuBox).itemSpriteId;
                    crate::gl!(&raw gSprites[gl_i57])
                },
                0,
            );
        }
        (*gSprites.as_ptr())[(*menuBox).itemSpriteId].set_invisible(FALSE as u16);
    }
}
pub unsafe fn LoadHeldItemIcons() {
    LoadSpriteSheet((&raw const *sSpriteSheet_HeldItem).cast_mut());
    LoadSpritePalette((&raw const *sSpritePalette_HeldItem).cast_mut());
}
pub unsafe fn DrawHeldItemIconsForTrade(
    partyCounts: *mut u8,
    partySpriteIds: *mut u8,
    whichParty: u8,
) {
    let mut i: u16 = 0;
    let mut item: u16 = 0;
    match whichParty {
        TRADE_PLAYER => {
            i = 0;
            while i < *partyCounts as u16 {
                item = GetMonData2(crate::gl!(&raw gPlayerParty[i]), MON_DATA_HELD_ITEM) as u16;
                if item != ITEM_NONE {
                    CreateHeldItemSpriteForTrade(*partySpriteIds.at(i), ItemIsMail(item));
                }
                i += 1;
            }
        }
        TRADE_PARTNER => {
            i = 0;
            while i < *partyCounts.at(1) as u16 {
                item = GetMonData2(crate::gl!(&raw gEnemyParty[i]), MON_DATA_HELD_ITEM) as u16;
                if item != ITEM_NONE {
                    CreateHeldItemSpriteForTrade(
                        *partySpriteIds.at(i as i32 + PARTY_SIZE),
                        ItemIsMail(item),
                    );
                }
                i += 1;
            }
        }
        _ => {}
    }
}
unsafe fn CreateHeldItemSpriteForTrade(spriteId: u8, isMail: u8) {
    let subpriority: u8 = crate::gl!(gSprites[spriteId].subpriority);
    let newSpriteId: u8 = CreateSprite(
        (&raw const *sSpriteTemplate_HeldItem).cast_mut(),
        250,
        170,
        subpriority - 1,
    );
    crate::gl!(gSprites[newSpriteId].x2 = 4);
    crate::gl!(gSprites[newSpriteId].y2 = 10);
    crate::gl!(gSprites[newSpriteId].callback = Some(SpriteCB_HeldItem));
    crate::gl!(gSprites[newSpriteId].data[7] = spriteId as i16);
    StartSpriteAnim(crate::gl!(&raw gSprites[newSpriteId]), isMail);
    (*gSprites.as_ptr())[newSpriteId]
        .callback
        .unwrap_unchecked()(crate::gl!(&raw gSprites[newSpriteId]));
}
pub(crate) unsafe fn SpriteCB_HeldItem(sprite: *mut Sprite) {
    let otherSpriteId: u8 = (*sprite).data[7] as u8;
    if (*gSprites.as_ptr())[otherSpriteId].invisible() != 0 {
        (*sprite).set_invisible(TRUE as u16);
    } else {
        (*sprite).set_invisible(FALSE as u16);
        (*sprite).x =
            crate::gl!(gSprites[otherSpriteId].x) + crate::gl!(gSprites[otherSpriteId].x2);
        (*sprite).y =
            crate::gl!(gSprites[otherSpriteId].y) + crate::gl!(gSprites[otherSpriteId].y2);
    }
}
unsafe fn CreatePartyMonPokeballSprite(mon: *mut Pokemon, menuBox: *mut PartyMenuBox) {
    if GetMonData2(mon, MON_DATA_SPECIES) != SPECIES_NONE as u32 {
        (*menuBox).pokeballSpriteId = CreateSprite(
            (&raw const *sSpriteTemplate_MenuPokeball).cast_mut(),
            *(*menuBox).spriteCoords.at(6) as i16,
            *(*menuBox).spriteCoords.at(7) as i16,
            8,
        );
    }
}
unsafe fn CreatePartyMonPokeballSpriteParameterized(species: u16, menuBox: *mut PartyMenuBox) {
    if species != SPECIES_NONE {
        (*menuBox).pokeballSpriteId = CreateSprite(
            (&raw const *sSpriteTemplate_MenuPokeball).cast_mut(),
            *(*menuBox).spriteCoords.at(6) as i16,
            *(*menuBox).spriteCoords.at(7) as i16,
            8,
        );
        (*gSprites.as_ptr())[(*menuBox).pokeballSpriteId]
            .oam
            .set_priority(0);
    }
}
unsafe fn CreatePokeballButtonSprite(x: u8, y: u8) -> u8 {
    let spriteId: u8 = CreateSprite(
        (&raw const *sSpriteTemplate_MenuPokeball).cast_mut(),
        x as i16,
        y as i16,
        8,
    );
    (*gSprites.as_ptr())[spriteId].oam.set_priority(2);
    spriteId
}
unsafe fn CreateSmallPokeballButtonSprite(x: u8, y: u8) -> u8 {
    CreateSprite(
        (&raw const *sSpriteTemplate_MenuPokeballSmall).cast_mut(),
        x as i16,
        y as i16,
        8,
    )
}
unsafe fn PartyMenuStartSpriteAnim(spriteId: u8, animNum: u8) {
    StartSpriteAnim(crate::gl!(&raw gSprites[spriteId]), animNum);
}
unsafe fn SpriteCB_BounceConfirmCancelButton(spriteId: u8, spriteId2: u8, animNum: u8) {
    if animNum == 0 {
        StartSpriteAnim(crate::gl!(&raw gSprites[spriteId]), 2);
        StartSpriteAnim(crate::gl!(&raw gSprites[spriteId2]), 4);
        crate::gl!(gSprites[spriteId].y2 = 0);
        crate::gl!(gSprites[spriteId2].y2 = 0);
    } else {
        StartSpriteAnim(crate::gl!(&raw gSprites[spriteId]), 3);
        StartSpriteAnim(crate::gl!(&raw gSprites[spriteId2]), 5);
        crate::gl!(gSprites[spriteId].y2 = -4);
        crate::gl!(gSprites[spriteId2].y2 = 4);
    }
}
unsafe fn LoadPartyMenuPokeballGfx() {
    LoadCompressedSpriteSheet((&raw const *sSpriteSheet_MenuPokeball).cast_mut());
    LoadCompressedSpriteSheet((&raw const *sSpriteSheet_MenuPokeballSmall).cast_mut());
    LoadCompressedSpritePalette((&raw const *sSpritePalette_MenuPokeball).cast_mut());
}
unsafe fn CreatePartyMonStatusSprite(mon: *mut Pokemon, menuBox: *mut PartyMenuBox) {
    if GetMonData2(mon, MON_DATA_SPECIES) != SPECIES_NONE as u32 {
        (*menuBox).statusSpriteId = CreateSprite(
            (&raw const *sSpriteTemplate_StatusIcons).cast_mut(),
            *(*menuBox).spriteCoords.at(4) as i16,
            *(*menuBox).spriteCoords.at(5) as i16,
            0,
        );
        SetPartyMonAilmentGfx(mon, menuBox);
    }
}
unsafe fn CreatePartyMonStatusSpriteParameterized(
    species: u16,
    status: u8,
    menuBox: *mut PartyMenuBox,
) {
    if species != SPECIES_NONE {
        (*menuBox).statusSpriteId = CreateSprite(
            (&raw const *sSpriteTemplate_StatusIcons).cast_mut(),
            *(*menuBox).spriteCoords.at(4) as i16,
            *(*menuBox).spriteCoords.at(5) as i16,
            0,
        );
        UpdatePartyMonAilmentGfx(status, menuBox);
        (*gSprites.as_ptr())[(*menuBox).statusSpriteId]
            .oam
            .set_priority(0);
    }
}
unsafe fn SetPartyMonAilmentGfx(mon: *mut Pokemon, menuBox: *mut PartyMenuBox) {
    UpdatePartyMonAilmentGfx(GetMonAilment(mon), menuBox);
}
unsafe fn UpdatePartyMonAilmentGfx(status: u8, menuBox: *mut PartyMenuBox) {
    match status {
        AILMENT_NONE | AILMENT_PKRS => {
            (*gSprites.as_ptr())[(*menuBox).statusSpriteId].set_invisible(TRUE as u16);
        }
        _ => {
            StartSpriteAnim(
                {
                    let gl_i58 = (*menuBox).statusSpriteId;
                    crate::gl!(&raw gSprites[gl_i58])
                },
                status - 1,
            );
            (*gSprites.as_ptr())[(*menuBox).statusSpriteId].set_invisible(FALSE as u16);
        }
    }
}
unsafe fn LoadPartyMenuAilmentGfx() {
    LoadCompressedSpriteSheet((&raw const *sSpriteSheet_StatusIcons).cast_mut());
    LoadCompressedSpritePalette((&raw const *sSpritePalette_StatusIcons).cast_mut());
}
pub unsafe fn CB2_ShowPartyMenuForItemUse() {
    let callback: Option<unsafe fn()> = Some(CB2_ReturnToBagMenu);
    let mut partyLayout: u8 = 0;
    let mut menuType: u8 = 0;
    let mut msgId: u8 = 0;
    let mut task: Option<unsafe fn(u8)> = None;
    if (*gMain.as_ptr()).inBattle() != 0 {
        menuType = PARTY_MENU_TYPE_IN_BATTLE;
        partyLayout = GetPartyLayoutFromBattleType();
    } else {
        menuType = PARTY_MENU_TYPE_FIELD;
        partyLayout = PARTY_LAYOUT_SINGLE;
    }
    if GetItemEffectType(gSpecialVar_ItemId.get()) == ITEM_EFFECT_SACRED_ASH {
        crate::gl!(gPartyMenu.slotId = 0);
        for i in 0..(PARTY_SIZE as u8) {
            if GetMonData2(crate::gl!(&raw gPlayerParty[i]), MON_DATA_SPECIES) != 0
                && GetMonData2(crate::gl!(&raw gPlayerParty[i]), MON_DATA_HP) == 0
            {
                crate::gl!(gPartyMenu.slotId = i as i8);
                break;
            }
        }
        task = Some(Task_SetSacredAshCB);
        msgId = PARTY_MSG_NONE;
    } else {
        if GetPocketByItemId(gSpecialVar_ItemId.get()) == POCKET_TM_HM {
            msgId = PARTY_MSG_TEACH_WHICH_MON;
        } else {
            msgId = PARTY_MSG_USE_ON_WHICH_MON;
        }
        task = Some(Task_HandleChooseMonInput);
    }
    InitPartyMenu(
        menuType,
        partyLayout,
        PARTY_ACTION_USE_ITEM,
        TRUE,
        msgId,
        task,
        callback,
    );
}
pub(crate) unsafe fn CB2_ReturnToBagMenu() {
    if (CurrentBattlePyramidLocation() != 0) as i32 == 0 {
        GoToBagMenu(ITEMMENULOCATION_LAST, POCKETS_COUNT, None);
    } else {
        GoToBattlePyramidBagMenu(
            PYRAMIDBAG_LOC_PREV,
            crate::gl!(gPyramidBagMenuState.exitCallback),
        );
    }
}
pub(crate) unsafe fn Task_SetSacredAshCB(taskId: u8) {
    if (*gPaletteFade.as_ptr()).active() == 0 {
        if (*gPartyMenu.as_ptr()).menuType() == PARTY_MENU_TYPE_IN_BATTLE {
            (*crate::gl!(sPartyMenuInternal)).exitCallback = Some(CB2_SetUpExitToBattleScreen);
        }
        (*gItemUseCB.as_ptr()).unwrap_unchecked()(taskId, Some(Task_ClosePartyMenuAfterText));
    }
}
unsafe fn IsHPRecoveryItem(item: u16) -> u8 {
    let mut effect: *mut u8 = null_mut();
    if item == ITEM_ENIGMA_BERRY {
        effect = (*crate::gl!(gSaveBlock1Ptr))
            .enigmaBerry
            .itemEffect
            .as_mut_ptr();
    } else {
        effect = (*(&raw const crate::data::pokemon::gItemEffectTable)
            .cast::<CArray<*mut u8, 0>>())[item as i32 - ITEM_POTION];
    }
    if *effect.at(4) as i32 & 0x4 != 0 {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
unsafe fn GetMedicineItemEffectMessage(item: u16) {
    match GetItemEffectType(item) {
        ITEM_EFFECT_CURE_POISON => {
            StringExpandPlaceholders(
                gStringVar4.as_mut_ptr(),
                (*(&raw const crate::data::strings::gText_PkmnCuredOfPoison)
                    .cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
            );
        }
        ITEM_EFFECT_CURE_SLEEP => {
            StringExpandPlaceholders(
                gStringVar4.as_mut_ptr(),
                (*(&raw const crate::data::strings::gText_PkmnWokeUp2).cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut(),
            );
        }
        ITEM_EFFECT_CURE_BURN => {
            StringExpandPlaceholders(
                gStringVar4.as_mut_ptr(),
                (*(&raw const crate::data::strings::gText_PkmnBurnHealed).cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut(),
            );
        }
        ITEM_EFFECT_CURE_FREEZE => {
            StringExpandPlaceholders(
                gStringVar4.as_mut_ptr(),
                (*(&raw const crate::data::strings::gText_PkmnThawedOut).cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut(),
            );
        }
        ITEM_EFFECT_CURE_PARALYSIS => {
            StringExpandPlaceholders(
                gStringVar4.as_mut_ptr(),
                (*(&raw const crate::data::strings::gText_PkmnCuredOfParalysis)
                    .cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
            );
        }
        ITEM_EFFECT_CURE_CONFUSION => {
            StringExpandPlaceholders(
                gStringVar4.as_mut_ptr(),
                (*(&raw const crate::data::strings::gText_PkmnSnappedOutOfConfusion)
                    .cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
            );
        }
        ITEM_EFFECT_CURE_INFATUATION => {
            StringExpandPlaceholders(
                gStringVar4.as_mut_ptr(),
                (*(&raw const crate::data::strings::gText_PkmnGotOverInfatuation)
                    .cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
            );
        }
        ITEM_EFFECT_CURE_ALL_STATUS => {
            StringExpandPlaceholders(
                gStringVar4.as_mut_ptr(),
                (*(&raw const crate::data::strings::gText_PkmnBecameHealthy)
                    .cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
            );
        }
        ITEM_EFFECT_HP_EV => {
            StringCopy(
                (*gStringVar2.as_ptr()).as_mut_ptr(),
                (*(&raw const crate::data::strings::gText_HP3).cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut(),
            );
            StringExpandPlaceholders(
                gStringVar4.as_mut_ptr(),
                (*(&raw const crate::data::strings::gText_PkmnBaseVar2StatIncreased)
                    .cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
            );
        }
        ITEM_EFFECT_ATK_EV => {
            StringCopy(
                (*gStringVar2.as_ptr()).as_mut_ptr(),
                (*(&raw const crate::data::strings::gText_Attack3).cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut(),
            );
            StringExpandPlaceholders(
                gStringVar4.as_mut_ptr(),
                (*(&raw const crate::data::strings::gText_PkmnBaseVar2StatIncreased)
                    .cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
            );
        }
        ITEM_EFFECT_DEF_EV => {
            StringCopy(
                (*gStringVar2.as_ptr()).as_mut_ptr(),
                (*(&raw const crate::data::strings::gText_Defense3).cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut(),
            );
            StringExpandPlaceholders(
                gStringVar4.as_mut_ptr(),
                (*(&raw const crate::data::strings::gText_PkmnBaseVar2StatIncreased)
                    .cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
            );
        }
        ITEM_EFFECT_SPEED_EV => {
            StringCopy(
                (*gStringVar2.as_ptr()).as_mut_ptr(),
                (*(&raw const crate::data::strings::gText_Speed2).cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut(),
            );
            StringExpandPlaceholders(
                gStringVar4.as_mut_ptr(),
                (*(&raw const crate::data::strings::gText_PkmnBaseVar2StatIncreased)
                    .cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
            );
        }
        ITEM_EFFECT_SPATK_EV => {
            StringCopy(
                (*gStringVar2.as_ptr()).as_mut_ptr(),
                (*(&raw const crate::data::strings::gText_SpAtk3).cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut(),
            );
            StringExpandPlaceholders(
                gStringVar4.as_mut_ptr(),
                (*(&raw const crate::data::strings::gText_PkmnBaseVar2StatIncreased)
                    .cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
            );
        }
        ITEM_EFFECT_SPDEF_EV => {
            StringCopy(
                (*gStringVar2.as_ptr()).as_mut_ptr(),
                (*(&raw const crate::data::strings::gText_SpDef3).cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut(),
            );
            StringExpandPlaceholders(
                gStringVar4.as_mut_ptr(),
                (*(&raw const crate::data::strings::gText_PkmnBaseVar2StatIncreased)
                    .cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
            );
        }
        ITEM_EFFECT_PP_UP | ITEM_EFFECT_PP_MAX => {
            StringExpandPlaceholders(
                gStringVar4.as_mut_ptr(),
                (*(&raw const crate::data::strings::gText_MovesPPIncreased)
                    .cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
            );
        }
        ITEM_EFFECT_HEAL_PP => {
            StringExpandPlaceholders(
                gStringVar4.as_mut_ptr(),
                (*(&raw const crate::data::strings::gText_PPWasRestored).cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut(),
            );
        }
        _ => {
            StringExpandPlaceholders(
                gStringVar4.as_mut_ptr(),
                (*(&raw const crate::data::strings::gText_WontHaveEffect).cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut(),
            );
        }
    }
}
unsafe fn NotUsingHPEVItemOnShedinja(mon: *mut Pokemon, item: u16) -> u8 {
    if GetItemEffectType(item) == ITEM_EFFECT_HP_EV
        && GetMonData2(mon, MON_DATA_SPECIES) == SPECIES_SHEDINJA as u32
    {
        return FALSE;
    }
    TRUE
}
fn IsItemFlute(item: u16) -> u8 {
    if item == ITEM_BLUE_FLUTE || item == ITEM_RED_FLUTE || item == ITEM_YELLOW_FLUTE {
        return TRUE;
    }
    FALSE
}
unsafe fn ExecuteTableBasedItemEffect_(partyMonIndex: u8, item: u16, monMoveIndex: u8) -> u8 {
    if (*gMain.as_ptr()).inBattle() != 0 {
        return ExecuteTableBasedItemEffect(
            crate::gl!(&raw gPlayerParty[partyMonIndex]),
            item,
            GetPartyIdFromBattleSlot(partyMonIndex),
            monMoveIndex,
        );
    } else {
        return ExecuteTableBasedItemEffect(
            crate::gl!(&raw gPlayerParty[partyMonIndex]),
            item,
            partyMonIndex,
            monMoveIndex,
        );
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
pub unsafe fn ItemUseCB_Medicine(taskId: u8, task: Option<unsafe fn(u8)>) {
    let mut hp: u16 = 0;
    let mon: *mut Pokemon = {
        let gl_i59 = crate::gl!(gPartyMenu.slotId);
        crate::gl!(&raw gPlayerParty[gl_i59])
    };
    let item: u16 = gSpecialVar_ItemId.get();
    let mut canHeal: u8 = 0;
    let mut cannotUse: u8 = 0;
    if NotUsingHPEVItemOnShedinja(mon, item) == FALSE {
        cannotUse = TRUE;
    } else {
        canHeal = IsHPRecoveryItem(item);
        if canHeal == TRUE {
            hp = GetMonData2(mon, MON_DATA_HP) as u16;
            if hp as u32 == GetMonData2(mon, MON_DATA_MAX_HP) {
                canHeal = FALSE;
            }
        }
        cannotUse = ExecuteTableBasedItemEffect_(crate::gl!(gPartyMenu.slotId) as u8, item, 0);
    }
    if cannotUse != FALSE {
        gPartyMenuUseExitCallback.set(FALSE);
        PlaySE(SE_SELECT);
        DisplayPartyMenuMessage(
            (*(&raw const crate::data::strings::gText_WontHaveEffect).cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
            TRUE,
        );
        ScheduleBgCopyTilemapToVram(2);
        task_set_func(taskId, task);
    } else {
        gPartyMenuUseExitCallback.set(TRUE);
        if IsItemFlute(item) == 0 {
            PlaySE(SE_USE_ITEM);
            if crate::gl!(gPartyMenu.action) != PARTY_ACTION_REUSABLE_ITEM {
                RemoveBagItem(item, 1);
            }
        } else {
            PlaySE(SE_GLASS_FLUTE);
        }
        SetPartyMonAilmentGfx(
            mon,
            (*sPartyMenuBoxes.as_ptr()).at(crate::gl!(gPartyMenu.slotId)),
        );
        if (*gSprites.as_ptr())
            [(*(*sPartyMenuBoxes.as_ptr()).at(crate::gl!(gPartyMenu.slotId))).statusSpriteId]
            .invisible()
            != 0
        {
            DisplayPartyPokemonLevelCheck(
                mon,
                (*sPartyMenuBoxes.as_ptr()).at(crate::gl!(gPartyMenu.slotId)),
                1,
            );
        }
        if canHeal == TRUE {
            if hp == 0 {
                AnimatePartySlot(crate::gl!(gPartyMenu.slotId) as u8, 1);
            }
            PartyMenuModifyHP(
                taskId,
                crate::gl!(gPartyMenu.slotId) as u8,
                1,
                GetMonData2(mon, MON_DATA_HP) as i16 - hp as i16,
                Some(Task_DisplayHPRestoredMessage),
            );
            ResetHPTaskData(taskId, 0, hp as u32);
        } else {
            GetMonNickname(mon, gStringVar1.as_mut_ptr());
            GetMedicineItemEffectMessage(item);
            DisplayPartyMenuMessage(gStringVar4.as_mut_ptr(), TRUE);
            ScheduleBgCopyTilemapToVram(2);
            task_set_func(taskId, task);
        }
    }
}
pub(crate) unsafe fn Task_DisplayHPRestoredMessage(taskId: u8) {
    GetMonNickname(
        {
            let gl_i60 = crate::gl!(gPartyMenu.slotId);
            crate::gl!(&raw gPlayerParty[gl_i60])
        },
        gStringVar1.as_mut_ptr(),
    );
    StringExpandPlaceholders(
        gStringVar4.as_mut_ptr(),
        (*(&raw const crate::data::strings::gText_PkmnHPRestoredByVar2).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
    );
    DisplayPartyMenuMessage(gStringVar4.as_mut_ptr(), FALSE);
    ScheduleBgCopyTilemapToVram(2);
    HandleBattleLowHpMusicChange();
    task_set_func(taskId, Some(Task_ClosePartyMenuAfterText));
}
pub(crate) unsafe fn Task_ClosePartyMenuAfterText(taskId: u8) {
    if IsPartyMenuTextPrinterActive() != TRUE {
        if gPartyMenuUseExitCallback.get() == FALSE {
            (*crate::gl!(sPartyMenuInternal)).exitCallback = None;
        }
        Task_ClosePartyMenu(taskId);
    }
}
pub unsafe fn ItemUseCB_ReduceEV(taskId: u8, task: Option<unsafe fn(u8)>) {
    let mon: *mut Pokemon = {
        let gl_i61 = crate::gl!(gPartyMenu.slotId);
        crate::gl!(&raw gPlayerParty[gl_i61])
    };
    let item: u16 = gSpecialVar_ItemId.get();
    let effectType: u8 = GetItemEffectType(item);
    let friendship: u16 = GetMonData2(mon, MON_DATA_FRIENDSHIP) as u16;
    let ev: u16 = ItemEffectToMonEv(mon, effectType);
    let cannotUseEffect: u8 =
        ExecuteTableBasedItemEffect_(crate::gl!(gPartyMenu.slotId) as u8, item, 0);
    let newFriendship: u16 = GetMonData2(mon, MON_DATA_FRIENDSHIP) as u16;
    let newEv: u16 = ItemEffectToMonEv(mon, effectType);
    if cannotUseEffect != 0 || friendship == newFriendship && ev == newEv {
        gPartyMenuUseExitCallback.set(FALSE);
        PlaySE(SE_SELECT);
        DisplayPartyMenuMessage(
            (*(&raw const crate::data::strings::gText_WontHaveEffect).cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
            TRUE,
        );
        ScheduleBgCopyTilemapToVram(2);
        task_set_func(taskId, task);
    } else {
        gPartyMenuUseExitCallback.set(TRUE);
        PlaySE(SE_USE_ITEM);
        RemoveBagItem(item, 1);
        GetMonNickname(mon, gStringVar1.as_mut_ptr());
        ItemEffectToStatString(effectType, (*gStringVar2.as_ptr()).as_mut_ptr());
        if friendship != newFriendship {
            if ev != newEv {
                StringExpandPlaceholders(
                    gStringVar4.as_mut_ptr(),
                    (*(&raw const crate::data::strings::gText_PkmnFriendlyBaseVar2Fell)
                        .cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut(),
                );
            } else {
                StringExpandPlaceholders(
                    gStringVar4.as_mut_ptr(),
                    (*(&raw const crate::data::strings::gText_PkmnFriendlyBaseVar2CantFall)
                        .cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut(),
                );
            }
        } else {
            StringExpandPlaceholders(
                gStringVar4.as_mut_ptr(),
                (*(&raw const crate::data::strings::gText_PkmnAdoresBaseVar2Fell)
                    .cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
            );
        }
        DisplayPartyMenuMessage(gStringVar4.as_mut_ptr(), TRUE);
        ScheduleBgCopyTilemapToVram(2);
        task_set_func(taskId, task);
    }
}
unsafe fn ItemEffectToMonEv(mon: *mut Pokemon, effectType: u8) -> u16 {
    match effectType {
        ITEM_EFFECT_HP_EV => {
            if GetMonData2(mon, MON_DATA_SPECIES) != SPECIES_SHEDINJA as u32 {
                return GetMonData2(mon, MON_DATA_HP_EV) as u16;
            }
        }
        ITEM_EFFECT_ATK_EV => {
            return GetMonData2(mon, MON_DATA_ATK_EV) as u16;
        }
        ITEM_EFFECT_DEF_EV => {
            return GetMonData2(mon, MON_DATA_DEF_EV) as u16;
        }
        ITEM_EFFECT_SPEED_EV => {
            return GetMonData2(mon, MON_DATA_SPEED_EV) as u16;
        }
        ITEM_EFFECT_SPATK_EV => {
            return GetMonData2(mon, MON_DATA_SPATK_EV) as u16;
        }
        ITEM_EFFECT_SPDEF_EV => {
            return GetMonData2(mon, MON_DATA_SPDEF_EV) as u16;
        }
        _ => {}
    }
    0
}
unsafe fn ItemEffectToStatString(effectType: u8, dest: *mut u8) {
    match effectType {
        ITEM_EFFECT_HP_EV => {
            StringCopy(
                dest,
                (*(&raw const crate::data::strings::gText_HP3).cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut(),
            );
        }
        ITEM_EFFECT_ATK_EV => {
            StringCopy(
                dest,
                (*(&raw const crate::data::strings::gText_Attack3).cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut(),
            );
        }
        ITEM_EFFECT_DEF_EV => {
            StringCopy(
                dest,
                (*(&raw const crate::data::strings::gText_Defense3).cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut(),
            );
        }
        ITEM_EFFECT_SPEED_EV => {
            StringCopy(
                dest,
                (*(&raw const crate::data::strings::gText_Speed2).cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut(),
            );
        }
        ITEM_EFFECT_SPATK_EV => {
            StringCopy(
                dest,
                (*(&raw const crate::data::strings::gText_SpAtk3).cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut(),
            );
        }
        ITEM_EFFECT_SPDEF_EV => {
            StringCopy(
                dest,
                (*(&raw const crate::data::strings::gText_SpDef3).cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut(),
            );
        }
        _ => {}
    }
}
unsafe fn ShowMoveSelectWindow(slot: u8) {
    let mut moveCount: u8 = 0;
    let fontId: u8 = FONT_NORMAL;
    let windowId: u8 = DisplaySelectionWindow(SELECTWINDOW_MOVES);
    let mut r#move: u16 = 0;
    for i in 0..(MAX_MON_MOVES as u8) {
        r#move = GetMonData2(
            crate::gl!(&raw gPlayerParty[slot]),
            MON_DATA_MOVE1 + i as i32,
        ) as u16;
        AddTextPrinterParameterized(
            windowId,
            fontId,
            (*(&raw const crate::data::data_tables::gMoveNames)
                .cast::<CArray<CArray<u8, 13>, 355>>())[r#move]
                .as_ptr()
                .cast_mut(),
            8,
            i * 16 + 1,
            TEXT_SKIP_DRAW,
            None,
        );
        if r#move != MOVE_NONE {
            moveCount += 1;
        }
    }
    InitMenuInUpperLeftCornerNormal(windowId, moveCount, 0);
    ScheduleBgCopyTilemapToVram(2);
}
pub(crate) unsafe fn Task_HandleWhichMoveInput(taskId: u8) {
    let input: i8 = Menu_ProcessInput();
    if input != MENU_NOTHING_CHOSEN {
        if input == MENU_B_PRESSED {
            PlaySE(SE_SELECT);
            ReturnToUseOnWhichMon(taskId);
        } else {
            PartyMenuRemoveWindow(&raw mut (*crate::gl!(sPartyMenuInternal)).windowId[1]);
            SetSelectedMoveForPPItem(taskId);
        }
    }
}
pub unsafe fn ItemUseCB_PPRecovery(taskId: u8, task: Option<unsafe fn(u8)>) {
    let mut effect: *mut u8 = null_mut();
    let item: u16 = gSpecialVar_ItemId.get();
    if item == ITEM_ENIGMA_BERRY {
        effect = (*crate::gl!(gSaveBlock1Ptr))
            .enigmaBerry
            .itemEffect
            .as_mut_ptr();
    } else {
        effect = (*(&raw const crate::data::pokemon::gItemEffectTable)
            .cast::<CArray<*mut u8, 0>>())[item as i32 - ITEM_POTION];
    }
    if *effect.at(4) as i32 & ITEM4_HEAL_PP_ONE == 0 {
        crate::gl!(gPartyMenu.data[0] = 0);
        TryUsePPItem(taskId);
    } else {
        PlaySE(SE_SELECT);
        DisplayPartyMenuStdMessage(PARTY_MSG_RESTORE_WHICH_MOVE);
        ShowMoveSelectWindow(crate::gl!(gPartyMenu.slotId) as u8);
        task_set_func(taskId, Some(Task_HandleWhichMoveInput));
    }
}
unsafe fn SetSelectedMoveForPPItem(taskId: u8) {
    PartyMenuRemoveWindow(&raw mut (*crate::gl!(sPartyMenuInternal)).windowId[0]);
    crate::gl!(gPartyMenu.data[0] = Menu_GetCursorPos() as i16);
    TryUsePPItem(taskId);
}
unsafe fn ReturnToUseOnWhichMon(taskId: u8) {
    task_set_func(taskId, Some(Task_HandleChooseMonInput));
    (*crate::gl!(sPartyMenuInternal)).exitCallback = None;
    PartyMenuRemoveWindow(&raw mut (*crate::gl!(sPartyMenuInternal)).windowId[0]);
    DisplayPartyMenuStdMessage(PARTY_MSG_USE_ON_WHICH_MON as u32);
}
unsafe fn TryUsePPItem(taskId: u8) {
    let mut r#move: u16 = MOVE_NONE;
    let moveSlot: *mut i16 = crate::gl!(&raw gPartyMenu.data[0]);
    let item: u16 = gSpecialVar_ItemId.get();
    let ptr: *mut PartyMenu = crate::gl!(&raw gPartyMenu);
    let mut mon: *mut Pokemon = null_mut();
    if ExecuteTableBasedItemEffect_((*ptr).slotId as u8, item, *moveSlot as u8) != 0 {
        gPartyMenuUseExitCallback.set(FALSE);
        PlaySE(SE_SELECT);
        DisplayPartyMenuMessage(
            (*(&raw const crate::data::strings::gText_WontHaveEffect).cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
            TRUE,
        );
        ScheduleBgCopyTilemapToVram(2);
        task_set_func(taskId, Some(Task_ClosePartyMenuAfterText));
    } else {
        gPartyMenuUseExitCallback.set(TRUE);
        mon = {
            let gl_i62 = (*ptr).slotId;
            crate::gl!(&raw gPlayerParty[gl_i62])
        };
        PlaySE(SE_USE_ITEM);
        RemoveBagItem(item, 1);
        r#move = GetMonData2(mon, MON_DATA_MOVE1 + *moveSlot as i32) as u16;
        StringCopy(
            gStringVar1.as_mut_ptr(),
            (*(&raw const crate::data::data_tables::gMoveNames)
                .cast::<CArray<CArray<u8, 13>, 355>>())[r#move]
                .as_ptr()
                .cast_mut(),
        );
        GetMedicineItemEffectMessage(item);
        DisplayPartyMenuMessage(gStringVar4.as_mut_ptr(), TRUE);
        ScheduleBgCopyTilemapToVram(2);
        task_set_func(taskId, Some(Task_ClosePartyMenuAfterText));
    }
}
pub unsafe fn ItemUseCB_PPUp(taskId: u8, task: Option<unsafe fn(u8)>) {
    PlaySE(SE_SELECT);
    DisplayPartyMenuStdMessage(PARTY_MSG_BOOST_PP_WHICH_MOVE);
    ShowMoveSelectWindow(crate::gl!(gPartyMenu.slotId) as u8);
    task_set_func(taskId, Some(Task_HandleWhichMoveInput));
}
pub unsafe fn ItemIdToBattleMoveId(item: u16) -> u16 {
    let tmNumber: u16 = item - ITEM_TM01;
    sTMHMMoves[tmNumber]
}
pub fn IsMoveHm(r#move: u16) -> u8 {
    for i in 0..NUM_HIDDEN_MACHINES {
        if sTMHMMoves[i as i32 + NUM_TECHNICAL_MACHINES] == r#move {
            return TRUE;
        }
    }
    FALSE
}
pub unsafe fn MonKnowsMove(mon: *mut Pokemon, r#move: u16) -> u8 {
    for i in 0..(MAX_MON_MOVES as u8) {
        if GetMonData2(mon, MON_DATA_MOVE1 + i as i32) == r#move as u32 {
            return TRUE;
        }
    }
    FALSE
}
unsafe fn DisplayLearnMoveMessage(str: *mut u8) {
    StringExpandPlaceholders(gStringVar4.as_mut_ptr(), str);
    DisplayPartyMenuMessage(gStringVar4.as_mut_ptr(), TRUE);
    ScheduleBgCopyTilemapToVram(2);
}
unsafe fn DisplayLearnMoveMessageAndClose(taskId: u8, str: *mut u8) {
    DisplayLearnMoveMessage(str);
    task_set_func(taskId, Some(Task_ClosePartyMenuAfterText));
}
pub unsafe fn ItemUseCB_TMHM(taskId: u8, task: Option<unsafe fn(u8)>) {
    PlaySE(SE_SELECT);
    let mon: *mut Pokemon = {
        let gl_i63 = crate::gl!(gPartyMenu.slotId);
        crate::gl!(&raw gPlayerParty[gl_i63])
    };
    let r#move: *mut i16 = (*gPartyMenu.as_ptr()).data.as_mut_ptr();
    let item: u16 = gSpecialVar_ItemId.get();
    GetMonNickname(mon, gStringVar1.as_mut_ptr());
    *r#move = ItemIdToBattleMoveId(item) as i16;
    StringCopy(
        (*gStringVar2.as_ptr()).as_mut_ptr(),
        (*(&raw const crate::data::data_tables::gMoveNames).cast::<CArray<CArray<u8, 13>, 355>>())
            [*r#move]
            .as_ptr()
            .cast_mut(),
    );
    *r#move.at(1) = 0;
    match CanMonLearnTMTutor(mon, item, 0) {
        CANNOT_LEARN_MOVE => {
            DisplayLearnMoveMessageAndClose(
                taskId,
                (*(&raw const crate::data::strings::gText_PkmnCantLearnMove)
                    .cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
            );
            return;
        }
        ALREADY_KNOWS_MOVE => {
            DisplayLearnMoveMessageAndClose(
                taskId,
                (*(&raw const crate::data::strings::gText_PkmnAlreadyKnows)
                    .cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
            );
            return;
        }
        _ => {}
    }
    if GiveMoveToMon(mon, *r#move as u16) != MON_HAS_MAX_MOVES {
        task_set_func(taskId, Some(Task_LearnedMove));
    } else {
        DisplayLearnMoveMessage(
            (*(&raw const crate::data::strings::gText_PkmnNeedsToReplaceMove)
                .cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        );
        task_set_func(taskId, Some(Task_ReplaceMoveYesNo));
    }
}
pub(crate) unsafe fn Task_LearnedMove(taskId: u8) {
    let mon: *mut Pokemon = {
        let gl_i64 = crate::gl!(gPartyMenu.slotId);
        crate::gl!(&raw gPlayerParty[gl_i64])
    };
    let r#move: *mut i16 = crate::gl!(&raw gPartyMenu.data[0]);
    let item: u16 = gSpecialVar_ItemId.get();
    if *r#move.at(1) == 0 {
        AdjustFriendship(mon, FRIENDSHIP_EVENT_LEARN_TMHM);
        if item < ITEM_HM01 {
            RemoveBagItem(item, 1);
        }
    }
    GetMonNickname(mon, gStringVar1.as_mut_ptr());
    StringCopy(
        (*gStringVar2.as_ptr()).as_mut_ptr(),
        (*(&raw const crate::data::data_tables::gMoveNames).cast::<CArray<CArray<u8, 13>, 355>>())
            [*r#move]
            .as_ptr()
            .cast_mut(),
    );
    StringExpandPlaceholders(
        gStringVar4.as_mut_ptr(),
        (*(&raw const crate::data::strings::gText_PkmnLearnedMove3).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
    );
    DisplayPartyMenuMessage(gStringVar4.as_mut_ptr(), TRUE);
    ScheduleBgCopyTilemapToVram(2);
    task_set_func(taskId, Some(Task_DoLearnedMoveFanfareAfterText));
}
pub(crate) unsafe fn Task_DoLearnedMoveFanfareAfterText(taskId: u8) {
    if IsPartyMenuTextPrinterActive() != TRUE {
        PlayFanfare(MUS_LEVEL_UP);
        task_set_func(taskId, Some(Task_LearnNextMoveOrClosePartyMenu));
    }
}
pub(crate) unsafe fn Task_LearnNextMoveOrClosePartyMenu(taskId: u8) {
    if IsFanfareTaskInactive() != 0
        && (crate::gl!(gMain.newKeys) as i32 & A_BUTTON != 0
            || crate::gl!(gMain.newKeys) as i32 & B_BUTTON != 0)
    {
        if crate::gl!(gPartyMenu.data[1]) == 1 {
            Task_TryLearningNextMove(taskId);
        } else {
            if crate::gl!(gPartyMenu.data[1]) == 2 {
                gSpecialVar_Result = TRUE as u16;
            }
            Task_ClosePartyMenu(taskId);
        }
    }
}
pub(crate) unsafe fn Task_ReplaceMoveYesNo(taskId: u8) {
    if IsPartyMenuTextPrinterActive() != TRUE {
        PartyMenuDisplayYesNoMenu();
        task_set_func(taskId, Some(Task_HandleReplaceMoveYesNoInput));
    }
}
pub(crate) unsafe fn Task_HandleReplaceMoveYesNoInput(taskId: u8) {
    'l1: {
        let sw1: i8 = Menu_ProcessInputNoWrapClearOnChoose();
        let mut fall = false;
        if sw1 == 0 {
            DisplayPartyMenuMessage(
                (*(&raw const crate::data::strings::gText_WhichMoveToForget)
                    .cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
                TRUE,
            );
            task_set_func(taskId, Some(Task_ShowSummaryScreenToForgetMove));
            break 'l1;
        }
        if sw1 == MENU_B_PRESSED {
            fall = true;
            PlaySE(SE_SELECT);
        }
        if fall || sw1 == 1 {
            StopLearningMovePrompt(taskId);
            break 'l1;
        }
    }
}
pub(crate) unsafe fn Task_ShowSummaryScreenToForgetMove(taskId: u8) {
    if IsPartyMenuTextPrinterActive() != TRUE {
        (*crate::gl!(sPartyMenuInternal)).exitCallback = Some(CB2_ShowSummaryScreenToForgetMove);
        Task_ClosePartyMenu(taskId);
    }
}
pub(crate) unsafe fn CB2_ShowSummaryScreenToForgetMove() {
    ShowSelectMovePokemonSummaryScreen(
        (*gPlayerParty.as_ptr()).as_mut_ptr(),
        crate::gl!(gPartyMenu.slotId) as u8,
        gPlayerPartyCount.get() - 1,
        Some(CB2_ReturnToPartyMenuWhileLearningMove),
        crate::gl!(gPartyMenu.data[0]) as u16,
    );
}
pub(crate) unsafe fn CB2_ReturnToPartyMenuWhileLearningMove() {
    InitPartyMenu(
        0,
        0,
        0,
        TRUE,
        PARTY_MSG_NONE,
        Some(Task_ReturnToPartyMenuWhileLearningMove),
        crate::gl!(gPartyMenu.exitCallback),
    );
}
pub(crate) unsafe fn Task_ReturnToPartyMenuWhileLearningMove(taskId: u8) {
    if (*gPaletteFade.as_ptr()).active() == 0 {
        if GetMoveSlotToReplace() != MAX_MON_MOVES as u8 {
            DisplayPartyMenuForgotMoveMessage(taskId);
        } else {
            StopLearningMovePrompt(taskId);
        }
    }
}
unsafe fn DisplayPartyMenuForgotMoveMessage(taskId: u8) {
    let mon: *mut Pokemon = {
        let gl_i65 = crate::gl!(gPartyMenu.slotId);
        crate::gl!(&raw gPlayerParty[gl_i65])
    };
    let r#move: u16 = GetMonData2(mon, MON_DATA_MOVE1 + GetMoveSlotToReplace() as i32) as u16;
    GetMonNickname(mon, gStringVar1.as_mut_ptr());
    StringCopy(
        (*gStringVar2.as_ptr()).as_mut_ptr(),
        (*(&raw const crate::data::data_tables::gMoveNames).cast::<CArray<CArray<u8, 13>, 355>>())
            [r#move]
            .as_ptr()
            .cast_mut(),
    );
    DisplayLearnMoveMessage(
        (*(&raw const crate::data::strings::gText_12PoofForgotMove).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
    );
    task_set_func(taskId, Some(Task_PartyMenuReplaceMove));
}
pub(crate) unsafe fn Task_PartyMenuReplaceMove(taskId: u8) {
    let mut mon: *mut Pokemon = null_mut();
    let mut r#move: u16 = 0;
    if IsPartyMenuTextPrinterActive() != TRUE {
        mon = {
            let gl_i66 = crate::gl!(gPartyMenu.slotId);
            crate::gl!(&raw gPlayerParty[gl_i66])
        };
        RemoveMonPPBonus(mon, GetMoveSlotToReplace());
        r#move = crate::gl!(gPartyMenu.data[0]) as u16;
        SetMonMoveSlot(mon, r#move, GetMoveSlotToReplace());
        Task_LearnedMove(taskId);
    }
}
unsafe fn StopLearningMovePrompt(taskId: u8) {
    StringCopy(
        (*gStringVar2.as_ptr()).as_mut_ptr(),
        (*(&raw const crate::data::data_tables::gMoveNames).cast::<CArray<CArray<u8, 13>, 355>>())
            [crate::gl!(gPartyMenu.data[0])]
        .as_ptr()
        .cast_mut(),
    );
    StringExpandPlaceholders(
        gStringVar4.as_mut_ptr(),
        (*(&raw const crate::data::strings::gText_StopLearningMove2).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
    );
    DisplayPartyMenuMessage(gStringVar4.as_mut_ptr(), TRUE);
    ScheduleBgCopyTilemapToVram(2);
    task_set_func(taskId, Some(Task_StopLearningMoveYesNo));
}
pub(crate) unsafe fn Task_StopLearningMoveYesNo(taskId: u8) {
    if IsPartyMenuTextPrinterActive() != TRUE {
        PartyMenuDisplayYesNoMenu();
        task_set_func(taskId, Some(Task_HandleStopLearningMoveYesNoInput));
    }
}
pub(crate) unsafe fn Task_HandleStopLearningMoveYesNoInput(taskId: u8) {
    let mon: *mut Pokemon = {
        let gl_i67 = crate::gl!(gPartyMenu.slotId);
        crate::gl!(&raw gPlayerParty[gl_i67])
    };
    'l1: {
        let sw1: i8 = Menu_ProcessInputNoWrapClearOnChoose();
        let mut fall = false;
        if sw1 == 0 {
            GetMonNickname(mon, gStringVar1.as_mut_ptr());
            StringCopy(
                (*gStringVar2.as_ptr()).as_mut_ptr(),
                (*(&raw const crate::data::data_tables::gMoveNames)
                    .cast::<CArray<CArray<u8, 13>, 355>>())[crate::gl!(gPartyMenu.data[0])]
                .as_ptr()
                .cast_mut(),
            );
            StringExpandPlaceholders(
                gStringVar4.as_mut_ptr(),
                (*(&raw const crate::data::strings::gText_MoveNotLearned).cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut(),
            );
            DisplayPartyMenuMessage(gStringVar4.as_mut_ptr(), TRUE);
            if crate::gl!(gPartyMenu.data[1]) == 1 {
                task_set_func(taskId, Some(Task_TryLearningNextMoveAfterText));
            } else {
                if crate::gl!(gPartyMenu.data[1]) == 2 {
                    gSpecialVar_Result = FALSE as u16;
                }
                task_set_func(taskId, Some(Task_ClosePartyMenuAfterText));
            }
            break 'l1;
        }
        if sw1 == MENU_B_PRESSED {
            fall = true;
            PlaySE(SE_SELECT);
        }
        if fall || sw1 == 1 {
            GetMonNickname(mon, gStringVar1.as_mut_ptr());
            StringCopy(
                (*gStringVar2.as_ptr()).as_mut_ptr(),
                (*(&raw const crate::data::data_tables::gMoveNames)
                    .cast::<CArray<CArray<u8, 13>, 355>>())[crate::gl!(gPartyMenu.data[0])]
                .as_ptr()
                .cast_mut(),
            );
            DisplayLearnMoveMessage(
                (*(&raw const crate::data::strings::gText_PkmnNeedsToReplaceMove)
                    .cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
            );
            task_set_func(taskId, Some(Task_ReplaceMoveYesNo));
            break 'l1;
        }
    }
}
pub(crate) unsafe fn Task_TryLearningNextMoveAfterText(taskId: u8) {
    if IsPartyMenuTextPrinterActive() != TRUE {
        Task_TryLearningNextMove(taskId);
    }
}
pub unsafe fn ItemUseCB_RareCandy(taskId: u8, task: Option<unsafe fn(u8)>) {
    let mon: *mut Pokemon = {
        let gl_i68 = crate::gl!(gPartyMenu.slotId);
        crate::gl!(&raw gPlayerParty[gl_i68])
    };
    let ptr: *mut PartyMenuInternal = crate::gl!(sPartyMenuInternal);
    let arrayPtr: *mut i16 = (*ptr).data.as_mut_ptr();
    let itemPtr: *mut u16 = gSpecialVar_ItemId.as_ptr();
    let mut cannotUseEffect: u8 = 0;
    if GetMonData2(mon, MON_DATA_LEVEL) != MAX_LEVEL {
        BufferMonStatsToTaskData(mon, arrayPtr);
        cannotUseEffect =
            ExecuteTableBasedItemEffect_(crate::gl!(gPartyMenu.slotId) as u8, *itemPtr, 0);
        BufferMonStatsToTaskData(mon, &raw mut (*ptr).data[6]);
    } else {
        cannotUseEffect = TRUE;
    }
    PlaySE(SE_SELECT);
    if cannotUseEffect != 0 {
        gPartyMenuUseExitCallback.set(FALSE);
        DisplayPartyMenuMessage(
            (*(&raw const crate::data::strings::gText_WontHaveEffect).cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
            TRUE,
        );
        ScheduleBgCopyTilemapToVram(2);
        task_set_func(taskId, task);
    } else {
        gPartyMenuUseExitCallback.set(TRUE);
        PlayFanfareByFanfareNum(FANFARE_LEVEL_UP);
        UpdateMonDisplayInfoAfterRareCandy(crate::gl!(gPartyMenu.slotId) as u8, mon);
        RemoveBagItem(gSpecialVar_ItemId.get(), 1);
        GetMonNickname(mon, gStringVar1.as_mut_ptr());
        ConvertIntToDecimalStringN(
            (*gStringVar2.as_ptr()).as_mut_ptr(),
            GetMonData2(mon, MON_DATA_LEVEL) as i32,
            STR_CONV_MODE_LEFT_ALIGN,
            3,
        );
        StringExpandPlaceholders(
            gStringVar4.as_mut_ptr(),
            (*(&raw const crate::data::strings::gText_PkmnElevatedToLvVar2)
                .cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        );
        DisplayPartyMenuMessage(gStringVar4.as_mut_ptr(), TRUE);
        ScheduleBgCopyTilemapToVram(2);
        task_set_func(taskId, Some(Task_DisplayLevelUpStatsPg1));
    }
}
unsafe fn UpdateMonDisplayInfoAfterRareCandy(slot: u8, mon: *mut Pokemon) {
    SetPartyMonAilmentGfx(mon, (*sPartyMenuBoxes.as_ptr()).at(slot));
    if (*gSprites.as_ptr())[(*(*sPartyMenuBoxes.as_ptr()).at(slot)).statusSpriteId].invisible() != 0
    {
        DisplayPartyPokemonLevelCheck(mon, (*sPartyMenuBoxes.as_ptr()).at(slot), 1);
    }
    DisplayPartyPokemonHPCheck(mon, (*sPartyMenuBoxes.as_ptr()).at(slot), 1);
    DisplayPartyPokemonMaxHPCheck(mon, (*sPartyMenuBoxes.as_ptr()).at(slot), 1);
    DisplayPartyPokemonHPBarCheck(mon, (*sPartyMenuBoxes.as_ptr()).at(slot));
    UpdatePartyMonHPBar((*(*sPartyMenuBoxes.as_ptr()).at(slot)).monSpriteId, mon);
    AnimatePartySlot(slot, 1);
    ScheduleBgCopyTilemapToVram(0);
}
pub(crate) unsafe fn Task_DisplayLevelUpStatsPg1(taskId: u8) {
    if WaitFanfare(FALSE) != 0
        && IsPartyMenuTextPrinterActive() != 1
        && (crate::gl!(gMain.newKeys) as i32 & 0x0001 != 0
            || crate::gl!(gMain.newKeys) as i32 & B_BUTTON != 0)
    {
        PlaySE(SE_SELECT);
        DisplayLevelUpStatsPg1(taskId);
        task_set_func(taskId, Some(Task_DisplayLevelUpStatsPg2));
    }
}
pub(crate) unsafe fn Task_DisplayLevelUpStatsPg2(taskId: u8) {
    if crate::gl!(gMain.newKeys) as i32 & A_BUTTON != 0
        || crate::gl!(gMain.newKeys) as i32 & B_BUTTON != 0
    {
        PlaySE(SE_SELECT);
        DisplayLevelUpStatsPg2(taskId);
        task_set_func(taskId, Some(Task_TryLearnNewMoves));
    }
}
unsafe fn DisplayLevelUpStatsPg1(taskId: u8) {
    let arrayPtr: *mut i16 = (*crate::gl!(sPartyMenuInternal)).data.as_mut_ptr();
    *arrayPtr.at(12) = CreateLevelUpStatsWindow() as i16;
    DrawLevelUpWindowPg1(
        *arrayPtr.at(12) as u16,
        arrayPtr as *mut u16,
        arrayPtr.at(6) as *mut u16,
        TEXT_COLOR_WHITE,
        TEXT_COLOR_DARK_GRAY,
        TEXT_COLOR_LIGHT_GRAY,
    );
    CopyWindowToVram(*arrayPtr.at(12) as u8, COPYWIN_GFX);
    ScheduleBgCopyTilemapToVram(2);
}
unsafe fn DisplayLevelUpStatsPg2(taskId: u8) {
    let arrayPtr: *mut i16 = (*crate::gl!(sPartyMenuInternal)).data.as_mut_ptr();
    DrawLevelUpWindowPg2(
        *arrayPtr.at(12) as u16,
        arrayPtr.at(6) as *mut u16,
        TEXT_COLOR_WHITE,
        TEXT_COLOR_DARK_GRAY,
        TEXT_COLOR_LIGHT_GRAY,
    );
    CopyWindowToVram(*arrayPtr.at(12) as u8, COPYWIN_GFX);
    ScheduleBgCopyTilemapToVram(2);
}
pub(crate) unsafe fn Task_TryLearnNewMoves(taskId: u8) {
    let mut learnMove: u16 = 0;
    if WaitFanfare(FALSE) != 0
        && (crate::gl!(gMain.newKeys) as i32 & A_BUTTON != 0
            || crate::gl!(gMain.newKeys) as i32 & B_BUTTON != 0)
    {
        RemoveLevelUpStatsWindow();
        learnMove = MonTryLearningNewMove(
            {
                let gl_i69 = crate::gl!(gPartyMenu.slotId);
                crate::gl!(&raw gPlayerParty[gl_i69])
            },
            TRUE,
        );
        crate::gl!(gPartyMenu.data[1] = 1);
        match learnMove {
            0 => {
                PartyMenuTryEvolution(taskId);
            }
            MON_HAS_MAX_MOVES => {
                DisplayMonNeedsToReplaceMove(taskId);
            }
            MON_ALREADY_KNOWS_MOVE => {
                task_set_func(taskId, Some(Task_TryLearningNextMove));
            }
            _ => {
                DisplayMonLearnedMove(taskId, learnMove);
            }
        }
    }
}
pub(crate) unsafe fn Task_TryLearningNextMove(taskId: u8) {
    let result: u16 = MonTryLearningNewMove(
        {
            let gl_i70 = crate::gl!(gPartyMenu.slotId);
            crate::gl!(&raw gPlayerParty[gl_i70])
        },
        FALSE,
    );
    match result {
        0 => {
            PartyMenuTryEvolution(taskId);
        }
        MON_HAS_MAX_MOVES => {
            DisplayMonNeedsToReplaceMove(taskId);
        }
        MON_ALREADY_KNOWS_MOVE => {}
        _ => {
            DisplayMonLearnedMove(taskId, result);
        }
    }
}
unsafe fn PartyMenuTryEvolution(taskId: u8) {
    let mon: *mut Pokemon = {
        let gl_i71 = crate::gl!(gPartyMenu.slotId);
        crate::gl!(&raw gPlayerParty[gl_i71])
    };
    let targetSpecies: u16 = GetEvolutionTargetSpecies(mon, EVO_MODE_NORMAL, ITEM_NONE);
    if targetSpecies != SPECIES_NONE {
        FreePartyPointers();
        crate::gl!(gCB2_AfterEvolution = crate::gl!(gPartyMenu.exitCallback));
        BeginEvolutionScene(
            mon,
            targetSpecies,
            TRUE,
            crate::gl!(gPartyMenu.slotId) as u8,
        );
        DestroyTask(taskId);
    } else {
        task_set_func(taskId, Some(Task_ClosePartyMenuAfterText));
    }
}
unsafe fn DisplayMonNeedsToReplaceMove(taskId: u8) {
    GetMonNickname(
        {
            let gl_i72 = crate::gl!(gPartyMenu.slotId);
            crate::gl!(&raw gPlayerParty[gl_i72])
        },
        gStringVar1.as_mut_ptr(),
    );
    StringCopy(
        (*gStringVar2.as_ptr()).as_mut_ptr(),
        (*(&raw const crate::data::data_tables::gMoveNames).cast::<CArray<CArray<u8, 13>, 355>>())
            [gMoveToLearn.get()]
        .as_ptr()
        .cast_mut(),
    );
    StringExpandPlaceholders(
        gStringVar4.as_mut_ptr(),
        (*(&raw const crate::data::strings::gText_PkmnNeedsToReplaceMove).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
    );
    DisplayPartyMenuMessage(gStringVar4.as_mut_ptr(), TRUE);
    ScheduleBgCopyTilemapToVram(2);
    crate::gl!(gPartyMenu.data[0] = gMoveToLearn.get() as i16);
    task_set_func(taskId, Some(Task_ReplaceMoveYesNo));
}
unsafe fn DisplayMonLearnedMove(taskId: u8, r#move: u16) {
    GetMonNickname(
        {
            let gl_i73 = crate::gl!(gPartyMenu.slotId);
            crate::gl!(&raw gPlayerParty[gl_i73])
        },
        gStringVar1.as_mut_ptr(),
    );
    StringCopy(
        (*gStringVar2.as_ptr()).as_mut_ptr(),
        (*(&raw const crate::data::data_tables::gMoveNames).cast::<CArray<CArray<u8, 13>, 355>>())
            [r#move]
            .as_ptr()
            .cast_mut(),
    );
    StringExpandPlaceholders(
        gStringVar4.as_mut_ptr(),
        (*(&raw const crate::data::strings::gText_PkmnLearnedMove3).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
    );
    DisplayPartyMenuMessage(gStringVar4.as_mut_ptr(), TRUE);
    ScheduleBgCopyTilemapToVram(2);
    crate::gl!(gPartyMenu.data[0] = r#move as i16);
    task_set_func(taskId, Some(Task_DoLearnedMoveFanfareAfterText));
}
unsafe fn BufferMonStatsToTaskData(mon: *mut Pokemon, data: *mut i16) {
    *data = GetMonData2(mon, MON_DATA_MAX_HP) as i16;
    *data.at(1) = GetMonData2(mon, MON_DATA_ATK) as i16;
    *data.at(2) = GetMonData2(mon, MON_DATA_DEF) as i16;
    *data.at(4) = GetMonData2(mon, MON_DATA_SPATK) as i16;
    *data.at(5) = GetMonData2(mon, MON_DATA_SPDEF) as i16;
    *data.at(3) = GetMonData2(mon, MON_DATA_SPEED) as i16;
}
pub unsafe fn ItemUseCB_SacredAsh(taskId: u8, task: Option<unsafe fn(u8)>) {
    (*crate::gl!(sPartyMenuInternal)).data[tUsedOnSlot] = FALSE as i16;
    (*crate::gl!(sPartyMenuInternal)).data[tHadEffect] = FALSE as i16;
    (*crate::gl!(sPartyMenuInternal)).data[tLastSlotUsed] = crate::gl!(gPartyMenu.slotId) as i16;
    UseSacredAsh(taskId);
}
unsafe fn UseSacredAsh(taskId: u8) {
    let mon: *mut Pokemon = {
        let gl_i74 = crate::gl!(gPartyMenu.slotId);
        crate::gl!(&raw gPlayerParty[gl_i74])
    };
    if GetMonData2(mon, MON_DATA_SPECIES) == SPECIES_NONE as u32 {
        task_set_func(taskId, Some(Task_SacredAshLoop));
        return;
    }
    let hp: u16 = GetMonData2(mon, MON_DATA_HP) as u16;
    if ExecuteTableBasedItemEffect_(
        crate::gl!(gPartyMenu.slotId) as u8,
        gSpecialVar_ItemId.get(),
        0,
    ) != 0
    {
        task_set_func(taskId, Some(Task_SacredAshLoop));
        return;
    }
    PlaySE(SE_USE_ITEM);
    SetPartyMonAilmentGfx(
        mon,
        (*sPartyMenuBoxes.as_ptr()).at(crate::gl!(gPartyMenu.slotId)),
    );
    if (*gSprites.as_ptr())
        [(*(*sPartyMenuBoxes.as_ptr()).at(crate::gl!(gPartyMenu.slotId))).statusSpriteId]
        .invisible()
        != 0
    {
        DisplayPartyPokemonLevelCheck(
            mon,
            (*sPartyMenuBoxes.as_ptr()).at(crate::gl!(gPartyMenu.slotId)),
            1,
        );
    }
    AnimatePartySlot(
        (*crate::gl!(sPartyMenuInternal)).data[tLastSlotUsed] as u8,
        0,
    );
    AnimatePartySlot(crate::gl!(gPartyMenu.slotId) as u8, 1);
    PartyMenuModifyHP(
        taskId,
        crate::gl!(gPartyMenu.slotId) as u8,
        1,
        GetMonData2(mon, MON_DATA_HP) as i16 - hp as i16,
        Some(Task_SacredAshDisplayHPRestored),
    );
    ResetHPTaskData(taskId, 0, hp as u32);
    (*crate::gl!(sPartyMenuInternal)).data[tUsedOnSlot] = TRUE as i16;
    (*crate::gl!(sPartyMenuInternal)).data[tHadEffect] = TRUE as i16;
}
pub(crate) unsafe fn Task_SacredAshLoop(taskId: u8) {
    if IsPartyMenuTextPrinterActive() != TRUE {
        if (*crate::gl!(sPartyMenuInternal)).data[tUsedOnSlot] == TRUE as i16 {
            (*crate::gl!(sPartyMenuInternal)).data[tUsedOnSlot] = FALSE as i16;
            (*crate::gl!(sPartyMenuInternal)).data[tLastSlotUsed] =
                crate::gl!(gPartyMenu.slotId) as i16;
        }
        if ({
            crate::gl!(gPartyMenu.slotId += 1);
            crate::gl!(gPartyMenu.slotId)
        }) == PARTY_SIZE as i8
        {
            if (*crate::gl!(sPartyMenuInternal)).data[tHadEffect] == FALSE as i16 {
                gPartyMenuUseExitCallback.set(FALSE);
                DisplayPartyMenuMessage(
                    (*(&raw const crate::data::strings::gText_WontHaveEffect)
                        .cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut(),
                    TRUE,
                );
                ScheduleBgCopyTilemapToVram(2);
            } else {
                gPartyMenuUseExitCallback.set(TRUE);
                RemoveBagItem(gSpecialVar_ItemId.get(), 1);
            }
            task_set_func(taskId, Some(Task_ClosePartyMenuAfterText));
            crate::gl!(gPartyMenu.slotId = 0);
        } else {
            UseSacredAsh(taskId);
        }
    }
}
pub(crate) unsafe fn Task_SacredAshDisplayHPRestored(taskId: u8) {
    GetMonNickname(
        {
            let gl_i75 = crate::gl!(gPartyMenu.slotId);
            crate::gl!(&raw gPlayerParty[gl_i75])
        },
        gStringVar1.as_mut_ptr(),
    );
    StringExpandPlaceholders(
        gStringVar4.as_mut_ptr(),
        (*(&raw const crate::data::strings::gText_PkmnHPRestoredByVar2).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
    );
    DisplayPartyMenuMessage(gStringVar4.as_mut_ptr(), FALSE);
    ScheduleBgCopyTilemapToVram(2);
    task_set_func(taskId, Some(Task_SacredAshLoop));
}
pub unsafe fn ItemUseCB_EvolutionStone(taskId: u8, task: Option<unsafe fn(u8)>) {
    PlaySE(SE_SELECT);
    crate::gl!(gCB2_AfterEvolution = crate::gl!(gPartyMenu.exitCallback));
    if ExecuteTableBasedItemEffect_(
        crate::gl!(gPartyMenu.slotId) as u8,
        gSpecialVar_ItemId.get(),
        0,
    ) != 0
    {
        gPartyMenuUseExitCallback.set(FALSE);
        DisplayPartyMenuMessage(
            (*(&raw const crate::data::strings::gText_WontHaveEffect).cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
            TRUE,
        );
        ScheduleBgCopyTilemapToVram(2);
        task_set_func(taskId, task);
    } else {
        RemoveBagItem(gSpecialVar_ItemId.get(), 1);
        FreePartyPointers();
    }
}
pub unsafe fn GetItemEffectType(item: u16) -> u8 {
    let mut itemEffect: *mut u8 = null_mut();
    if !(item >= ITEM_POTION as u16 && item <= ITEM_UNUSED_BERRY_3) {
        return ITEM_EFFECT_NONE;
    }
    if item == ITEM_ENIGMA_BERRY {
        itemEffect = (*crate::gl!(gSaveBlock1Ptr))
            .enigmaBerry
            .itemEffect
            .as_mut_ptr();
    } else {
        itemEffect = (*(&raw const crate::data::pokemon::gItemEffectTable)
            .cast::<CArray<*mut u8, 0>>())[item as i32 - ITEM_POTION];
    }
    if *itemEffect as i32 & 63 != 0
        || *itemEffect.at(1) != 0
        || *itemEffect.at(2) != 0
        || *itemEffect.at(3) as i32 & ITEM3_GUARD_SPEC != 0
    {
        return ITEM_EFFECT_X_ITEM;
    } else if *itemEffect as i32 & ITEM0_SACRED_ASH != 0 {
        return ITEM_EFFECT_SACRED_ASH;
    } else if *itemEffect.at(3) as i32 & ITEM3_LEVEL_UP != 0 {
        return ITEM_EFFECT_RAISE_LEVEL;
    }
    let statusCure: u32 = *itemEffect.at(3) as u32 & ITEM3_STATUS_ALL as u32;
    if statusCure != 0 || *itemEffect >> 7 != 0 {
        if statusCure == ITEM3_SLEEP as u32 {
            return ITEM_EFFECT_CURE_SLEEP;
        } else if statusCure == ITEM3_POISON as u32 {
            return ITEM_EFFECT_CURE_POISON;
        } else if statusCure == ITEM3_BURN as u32 {
            return ITEM_EFFECT_CURE_BURN;
        } else if statusCure == ITEM3_FREEZE as u32 {
            return ITEM_EFFECT_CURE_FREEZE;
        } else if statusCure == ITEM3_PARALYSIS as u32 {
            return ITEM_EFFECT_CURE_PARALYSIS;
        } else if statusCure == ITEM3_CONFUSION as u32 {
            return ITEM_EFFECT_CURE_CONFUSION;
        } else if *itemEffect >> 7 != 0 && statusCure == 0 {
            return ITEM_EFFECT_CURE_INFATUATION;
        } else {
            return ITEM_EFFECT_CURE_ALL_STATUS;
        }
    }
    if *itemEffect.at(4) as i32 & 68 != 0 {
        return ITEM_EFFECT_HEAL_HP;
    } else if *itemEffect.at(4) as i32 & ITEM4_EV_ATK != 0 {
        return ITEM_EFFECT_ATK_EV;
    } else if *itemEffect.at(4) as i32 & ITEM4_EV_HP != 0 {
        return ITEM_EFFECT_HP_EV;
    } else if *itemEffect.at(5) as i32 & ITEM5_EV_SPATK != 0 {
        return ITEM_EFFECT_SPATK_EV;
    } else if *itemEffect.at(5) as i32 & ITEM5_EV_SPDEF != 0 {
        return ITEM_EFFECT_SPDEF_EV;
    } else if *itemEffect.at(5) as i32 & ITEM5_EV_SPEED != 0 {
        return ITEM_EFFECT_SPEED_EV;
    } else if *itemEffect.at(5) as i32 & ITEM5_EV_DEF != 0 {
        return ITEM_EFFECT_DEF_EV;
    } else if *itemEffect.at(4) as i32 & ITEM4_EVO_STONE != 0 {
        return ITEM_EFFECT_EVO_STONE;
    } else if *itemEffect.at(4) as i32 & ITEM4_PP_UP != 0 {
        return ITEM_EFFECT_PP_UP;
    } else if *itemEffect.at(5) as i32 & ITEM5_PP_MAX != 0 {
        return ITEM_EFFECT_PP_MAX;
    } else if *itemEffect.at(4) as i32 & 24 != 0 {
        return ITEM_EFFECT_HEAL_PP;
    } else {
        return ITEM_EFFECT_NONE;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
unsafe fn TryTutorSelectedMon(taskId: u8) {
    let mut mon: *mut Pokemon = null_mut();
    let mut r#move: *mut i16 = null_mut();
    if (*gPaletteFade.as_ptr()).active() == 0 {
        mon = {
            let gl_i76 = crate::gl!(gPartyMenu.slotId);
            crate::gl!(&raw gPlayerParty[gl_i76])
        };
        r#move = crate::gl!(&raw gPartyMenu.data[0]);
        GetMonNickname(mon, gStringVar1.as_mut_ptr());
        crate::gl!(gPartyMenu.data[0] = GetTutorMove(gSpecialVar_0x8005 as u8) as i16);
        StringCopy(
            (*gStringVar2.as_ptr()).as_mut_ptr(),
            (*(&raw const crate::data::data_tables::gMoveNames)
                .cast::<CArray<CArray<u8, 13>, 355>>())[crate::gl!(gPartyMenu.data[0])]
            .as_ptr()
            .cast_mut(),
        );
        *r#move.at(1) = 2;
        match CanMonLearnTMTutor(mon, 0, gSpecialVar_0x8005 as u8) {
            CANNOT_LEARN_MOVE => {
                DisplayLearnMoveMessageAndClose(
                    taskId,
                    (*(&raw const crate::data::strings::gText_PkmnCantLearnMove)
                        .cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut(),
                );
                return;
            }
            ALREADY_KNOWS_MOVE => {
                DisplayLearnMoveMessageAndClose(
                    taskId,
                    (*(&raw const crate::data::strings::gText_PkmnAlreadyKnows)
                        .cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut(),
                );
                return;
            }
            _ => {
                if GiveMoveToMon(mon, crate::gl!(gPartyMenu.data[0]) as u16) != MON_HAS_MAX_MOVES {
                    Task_LearnedMove(taskId);
                    return;
                }
            }
        }
        DisplayLearnMoveMessage(
            (*(&raw const crate::data::strings::gText_PkmnNeedsToReplaceMove)
                .cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        );
        task_set_func(taskId, Some(Task_ReplaceMoveYesNo));
    }
}
pub unsafe fn CB2_PartyMenuFromStartMenu() {
    InitPartyMenu(
        0,
        0,
        0,
        0,
        0,
        Some(Task_HandleChooseMonInput),
        Some(CB2_ReturnToFieldWithOpenMenu),
    );
}
pub unsafe fn CB2_ChooseMonToGiveItem() {
    let callback: Option<unsafe fn()> = if CurrentBattlePyramidLocation() == PYRAMID_LOCATION_NONE {
        Some(CB2_ReturnToBagMenu)
    } else {
        Some(CB2_ReturnToPyramidBagMenu)
    };
    InitPartyMenu(
        0,
        0,
        PARTY_ACTION_GIVE_ITEM,
        0,
        PARTY_MSG_GIVE_TO_WHICH_MON,
        Some(Task_HandleChooseMonInput),
        callback,
    );
    crate::gl!(gPartyMenu.bagItem = gSpecialVar_ItemId.get());
}
unsafe fn TryGiveItemOrMailToSelectedMon(taskId: u8) {
    sPartyMenuItemId.set(GetMonData2(
        {
            let gl_i77 = crate::gl!(gPartyMenu.slotId);
            crate::gl!(&raw gPlayerParty[gl_i77])
        },
        MON_DATA_HELD_ITEM,
    ) as u16);
    if sPartyMenuItemId.get() == ITEM_NONE {
        GiveItemOrMailToSelectedMon(taskId);
    } else if ItemIsMail(sPartyMenuItemId.get()) != 0 {
        DisplayItemMustBeRemovedFirstMessage(taskId);
    } else {
        DisplayAlreadyHoldingItemSwitchMessage(
            {
                let gl_i78 = crate::gl!(gPartyMenu.slotId);
                crate::gl!(&raw gPlayerParty[gl_i78])
            },
            sPartyMenuItemId.get(),
            TRUE,
        );
        task_set_func(taskId, Some(Task_SwitchItemsFromBagYesNo));
    }
}
unsafe fn GiveItemOrMailToSelectedMon(taskId: u8) {
    if ItemIsMail(crate::gl!(gPartyMenu.bagItem)) != 0 {
        RemoveItemToGiveFromBag(crate::gl!(gPartyMenu.bagItem));
        (*crate::gl!(sPartyMenuInternal)).exitCallback = Some(CB2_WriteMailToGiveMonFromBag);
        Task_ClosePartyMenu(taskId);
    } else {
        GiveItemToSelectedMon(taskId);
    }
}
unsafe fn GiveItemToSelectedMon(taskId: u8) {
    let mut item: u16 = 0;
    if (*gPaletteFade.as_ptr()).active() == 0 {
        item = crate::gl!(gPartyMenu.bagItem);
        DisplayGaveHeldItemMessage(
            {
                let gl_i79 = crate::gl!(gPartyMenu.slotId);
                crate::gl!(&raw gPlayerParty[gl_i79])
            },
            item,
            FALSE,
            1,
        );
        GiveItemToMon(
            {
                let gl_i80 = crate::gl!(gPartyMenu.slotId);
                crate::gl!(&raw gPlayerParty[gl_i80])
            },
            item,
        );
        RemoveItemToGiveFromBag(item);
        task_set_func(taskId, Some(Task_UpdateHeldItemSpriteAndClosePartyMenu));
    }
}
pub(crate) unsafe fn Task_UpdateHeldItemSpriteAndClosePartyMenu(taskId: u8) {
    let slot: i8 = crate::gl!(gPartyMenu.slotId);
    if IsPartyMenuTextPrinterActive() != TRUE {
        UpdatePartyMonHeldItemSprite(
            crate::gl!(&raw gPlayerParty[slot]),
            (*sPartyMenuBoxes.as_ptr()).at(slot),
        );
        Task_ClosePartyMenu(taskId);
    }
}
pub(crate) unsafe fn CB2_WriteMailToGiveMonFromBag() {
    GiveItemToMon(
        {
            let gl_i81 = crate::gl!(gPartyMenu.slotId);
            crate::gl!(&raw gPlayerParty[gl_i81])
        },
        crate::gl!(gPartyMenu.bagItem),
    );
    let mail: u8 = GetMonData2(
        {
            let gl_i82 = crate::gl!(gPartyMenu.slotId);
            crate::gl!(&raw gPlayerParty[gl_i82])
        },
        MON_DATA_MAIL,
    ) as u8;
    DoEasyChatScreen(
        EASY_CHAT_TYPE_MAIL,
        (*crate::gl!(gSaveBlock1Ptr)).mail[mail].words.as_mut_ptr(),
        Some(CB2_ReturnToPartyOrBagMenuFromWritingMail),
        EASY_CHAT_PERSON_DISPLAY_NONE,
    );
}
pub(crate) unsafe fn CB2_ReturnToPartyOrBagMenuFromWritingMail() {
    let mon: *mut Pokemon = {
        let gl_i83 = crate::gl!(gPartyMenu.slotId);
        crate::gl!(&raw gPlayerParty[gl_i83])
    };
    let item: u16 = GetMonData2(mon, MON_DATA_HELD_ITEM) as u16;
    if gSpecialVar_Result == FALSE as u16 {
        TakeMailFromMon(mon);
        SetMonData(
            mon,
            MON_DATA_HELD_ITEM,
            sPartyMenuItemId.as_ptr() as *mut c_void,
        );
        RemoveBagItem(sPartyMenuItemId.get(), 1);
        ReturnGiveItemToBagOrPC(item);
        SetMainCallback2(crate::gl!(gPartyMenu.exitCallback));
    } else {
        InitPartyMenu(
            (*gPartyMenu.as_ptr()).menuType(),
            KEEP_PARTY_LAYOUT,
            crate::gl!(gPartyMenu.action),
            TRUE,
            PARTY_MSG_NONE,
            Some(Task_DisplayGaveMailFromBagMessage),
            crate::gl!(gPartyMenu.exitCallback),
        );
    }
}
pub(crate) unsafe fn Task_DisplayGaveMailFromBagMessage(taskId: u8) {
    if (*gPaletteFade.as_ptr()).active() == 0 {
        if sPartyMenuItemId.get() != ITEM_NONE {
            DisplaySwitchedHeldItemMessage(
                crate::gl!(gPartyMenu.bagItem),
                sPartyMenuItemId.get(),
                FALSE,
            );
        } else {
            DisplayGaveHeldItemMessage(
                {
                    let gl_i84 = crate::gl!(gPartyMenu.slotId);
                    crate::gl!(&raw gPlayerParty[gl_i84])
                },
                crate::gl!(gPartyMenu.bagItem),
                FALSE,
                1,
            );
        }
        task_set_func(taskId, Some(Task_UpdateHeldItemSpriteAndClosePartyMenu));
    }
}
pub(crate) unsafe fn Task_SwitchItemsFromBagYesNo(taskId: u8) {
    if IsPartyMenuTextPrinterActive() != TRUE {
        PartyMenuDisplayYesNoMenu();
        task_set_func(taskId, Some(Task_HandleSwitchItemsFromBagYesNoInput));
    }
}
pub(crate) unsafe fn Task_HandleSwitchItemsFromBagYesNoInput(taskId: u8) {
    let mut item: u16 = 0;
    'l1: {
        let sw1: i8 = Menu_ProcessInputNoWrapClearOnChoose();
        let mut fall = false;
        if sw1 == 0 {
            item = crate::gl!(gPartyMenu.bagItem);
            RemoveItemToGiveFromBag(item);
            if AddBagItem(sPartyMenuItemId.get(), 1) == FALSE {
                ReturnGiveItemToBagOrPC(item);
                BufferBagFullCantTakeItemMessage(sPartyMenuItemId.get());
                DisplayPartyMenuMessage(gStringVar4.as_mut_ptr(), FALSE);
                task_set_func(taskId, Some(Task_UpdateHeldItemSpriteAndClosePartyMenu));
            } else if ItemIsMail(item) != 0 {
                (*crate::gl!(sPartyMenuInternal)).exitCallback =
                    Some(CB2_WriteMailToGiveMonFromBag);
                Task_ClosePartyMenu(taskId);
            } else {
                GiveItemToMon(
                    {
                        let gl_i85 = crate::gl!(gPartyMenu.slotId);
                        crate::gl!(&raw gPlayerParty[gl_i85])
                    },
                    item,
                );
                DisplaySwitchedHeldItemMessage(item, sPartyMenuItemId.get(), TRUE);
                task_set_func(taskId, Some(Task_UpdateHeldItemSpriteAndClosePartyMenu));
            }
            break 'l1;
        }
        if sw1 == MENU_B_PRESSED {
            fall = true;
            PlaySE(SE_SELECT);
        }
        if fall || sw1 == 1 {
            task_set_func(taskId, Some(Task_UpdateHeldItemSpriteAndClosePartyMenu));
            break 'l1;
        }
    }
}
unsafe fn DisplayItemMustBeRemovedFirstMessage(taskId: u8) {
    DisplayPartyMenuMessage(
        (*(&raw const crate::data::strings::gText_RemoveMailBeforeItem).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        TRUE,
    );
    ScheduleBgCopyTilemapToVram(2);
    task_set_func(taskId, Some(Task_UpdateHeldItemSpriteAndClosePartyMenu));
}
fn RemoveItemToGiveFromBag(item: u16) {
    if crate::gl!(gPartyMenu.action) == PARTY_ACTION_GIVE_PC_ITEM {
        RemovePCItem(item as u8, 1);
    } else {
        RemoveBagItem(item, 1);
    }
}
fn ReturnGiveItemToBagOrPC(item: u16) -> u8 {
    if crate::gl!(gPartyMenu.action) == PARTY_ACTION_GIVE_ITEM {
        return AddBagItem(item, 1);
    } else {
        return AddPCItem(item, 1);
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
pub unsafe fn ChooseMonToGiveMailFromMailbox() {
    InitPartyMenu(
        0,
        0,
        PARTY_ACTION_GIVE_MAILBOX_MAIL,
        0,
        PARTY_MSG_GIVE_TO_WHICH_MON,
        Some(Task_HandleChooseMonInput),
        Some(Mailbox_ReturnToMailListAfterDeposit),
    );
}
unsafe fn TryGiveMailToSelectedMon(taskId: u8) {
    let mon: *mut Pokemon = {
        let gl_i86 = crate::gl!(gPartyMenu.slotId);
        crate::gl!(&raw gPlayerParty[gl_i86])
    };
    let mut mail: *mut Mail = null_mut();
    gPartyMenuUseExitCallback.set(FALSE);
    mail = &raw mut (*crate::gl!(gSaveBlock1Ptr)).mail[crate::gl!(gPlayerPCItemPageInfo.itemsAbove)
        as i32
        + PARTY_SIZE
        + crate::gl!(gPlayerPCItemPageInfo.cursorPos) as i32];
    if GetMonData2(mon, MON_DATA_HELD_ITEM) != ITEM_NONE as u32 {
        DisplayPartyMenuMessage(
            (*(&raw const crate::data::strings::gText_PkmnHoldingItemCantHoldMail)
                .cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
            TRUE,
        );
    } else {
        GiveMailToMon(mon, mail);
        ClearMail(mail);
        DisplayPartyMenuMessage(
            (*(&raw const crate::data::strings::gText_MailTransferredFromMailbox)
                .cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
            TRUE,
        );
    }
    ScheduleBgCopyTilemapToVram(2);
    task_set_func(taskId, Some(Task_UpdateHeldItemSpriteAndClosePartyMenu));
}
#[unsafe(no_mangle)]
pub unsafe fn InitChooseHalfPartyForBattle(unused: u8) {
    ClearSelectedPartyOrder();
    InitPartyMenu(
        PARTY_MENU_TYPE_CHOOSE_HALF,
        0,
        0,
        0,
        0,
        Some(Task_HandleChooseMonInput),
        crate::gl!(gMain.savedCallback),
    );
    crate::gl!(gPartyMenu.task = Some(Task_ValidateChosenHalfParty));
}
pub unsafe fn ClearSelectedPartyOrder() {
    memset((*gSelectedOrderFromParty.as_ptr()).as_mut_ptr(), 0, 4);
}
unsafe fn GetPartySlotEntryStatus(slot: i8) -> u8 {
    if GetBattleEntryEligibility(crate::gl!(&raw gPlayerParty[slot])) == FALSE {
        return 2;
    }
    if HasPartySlotAlreadyBeenSelected(slot as u8 + 1) == 1 {
        return 1;
    }
    0
}
unsafe fn GetBattleEntryEligibility(mon: *mut Pokemon) -> u8 {
    let mut i: u16 = 0;
    let mut species: u16 = 0;
    if GetMonData2(mon, MON_DATA_IS_EGG) != 0
        || GetMonData2(mon, MON_DATA_LEVEL) > GetBattleEntryLevelCap() as u32
        || (*crate::gl!(gSaveBlock1Ptr)).location.mapGroup == 26
            && (*crate::gl!(gSaveBlock1Ptr)).location.mapNum == 25
            && GetMonData2(mon, MON_DATA_HELD_ITEM) != ITEM_NONE as u32
    {
        return FALSE;
    }
    match VarGet(VAR_FRONTIER_FACILITY) {
        FACILITY_MULTI_OR_EREADER => {
            if GetMonData2(mon, MON_DATA_HP) != 0 {
                return TRUE;
            }
            return FALSE;
        }
        FACILITY_UNION_ROOM => {
            return TRUE;
        }
        _ => {
            species = GetMonData2(mon, MON_DATA_SPECIES) as u16;
            while (*(&raw const crate::data::frontier_util::gFrontierBannedSpecies)
                .cast::<CArray<u16, 0>>())[i]
                != 0xFFFF
            {
                if (*(&raw const crate::data::frontier_util::gFrontierBannedSpecies)
                    .cast::<CArray<u16, 0>>())[i]
                    == species
                {
                    return FALSE;
                }
                i += 1;
            }
            return TRUE;
        }
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
unsafe fn CheckBattleEntriesAndGetMessage() -> u8 {
    let party: *mut Pokemon = (*gPlayerParty.as_ptr()).as_mut_ptr();
    let minBattlers: u8 = GetMinBattleEntries();
    let order: *mut u8 = (*gSelectedOrderFromParty.as_ptr()).as_mut_ptr();
    if *order.at(minBattlers as i32 - 1) == 0 {
        if minBattlers == 1 {
            return PARTY_MSG_NO_MON_FOR_BATTLE;
        }
        ConvertIntToDecimalStringN(
            gStringVar1.as_mut_ptr(),
            minBattlers as i32,
            STR_CONV_MODE_LEFT_ALIGN,
            1,
        );
        return PARTY_MSG_X_MONS_ARE_NEEDED;
    }
    let facility: u8 = VarGet(VAR_FRONTIER_FACILITY) as u8;
    if facility == FACILITY_UNION_ROOM as u8 || facility == FACILITY_MULTI_OR_EREADER as u8 {
        return 0xFF;
    }
    let maxBattlers: u8 = GetMaxBattleEntries();
    let mut i: u8 = 0;
    while (i as i32) < maxBattlers as i32 - 1 {
        let species: u16 = GetMonData2(party.at(*order.at(i) as i32 - 1), MON_DATA_SPECIES) as u16;
        let item: u16 = GetMonData2(party.at(*order.at(i) as i32 - 1), MON_DATA_HELD_ITEM) as u16;
        for j in (i + 1)..maxBattlers {
            if species as u32 == GetMonData2(party.at(*order.at(j) as i32 - 1), MON_DATA_SPECIES) {
                return PARTY_MSG_MONS_CANT_BE_SAME;
            }
            if item != ITEM_NONE
                && item as u32 == GetMonData2(party.at(*order.at(j) as i32 - 1), MON_DATA_HELD_ITEM)
            {
                return PARTY_MSG_NO_SAME_HOLD_ITEMS;
            }
        }
        i += 1;
    }
    0xFF
}
fn HasPartySlotAlreadyBeenSelected(slot: u8) -> u8 {
    for i in 0..4u8 {
        if crate::gl!(gSelectedOrderFromParty[i]) == slot {
            return TRUE;
        }
    }
    FALSE
}
pub(crate) unsafe fn Task_ValidateChosenHalfParty(taskId: u8) {
    let msgId: u8 = CheckBattleEntriesAndGetMessage();
    if msgId != 0xFF {
        PlaySE(SE_FAILURE);
        DisplayPartyMenuStdMessage(msgId as u32);
        task_set_func(taskId, Some(Task_ContinueChoosingHalfParty));
    } else {
        PlaySE(SE_SELECT);
        Task_ClosePartyMenu(taskId);
    }
}
pub(crate) unsafe fn Task_ContinueChoosingHalfParty(taskId: u8) {
    if crate::gl!(gMain.newKeys) as i32 & A_BUTTON != 0
        || crate::gl!(gMain.newKeys) as i32 & B_BUTTON != 0
    {
        PlaySE(SE_SELECT);
        DisplayPartyMenuStdMessage(PARTY_MSG_CHOOSE_MON);
        task_set_func(taskId, Some(Task_HandleChooseMonInput));
    }
}
unsafe fn GetMaxBattleEntries() -> u8 {
    match VarGet(VAR_FRONTIER_FACILITY) {
        FACILITY_MULTI_OR_EREADER => {
            return MULTI_PARTY_SIZE as u8;
        }
        FACILITY_UNION_ROOM => {
            return UNION_ROOM_PARTY_SIZE;
        }
        _ => {
            return gSpecialVar_0x8005 as u8;
        }
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
unsafe fn GetMinBattleEntries() -> u8 {
    match VarGet(VAR_FRONTIER_FACILITY) {
        FACILITY_MULTI_OR_EREADER => {
            return 1;
        }
        FACILITY_UNION_ROOM => {
            return UNION_ROOM_PARTY_SIZE;
        }
        _ => {
            return gSpecialVar_0x8005 as u8;
        }
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
unsafe fn GetBattleEntryLevelCap() -> u8 {
    match VarGet(VAR_FRONTIER_FACILITY) {
        FACILITY_MULTI_OR_EREADER => {
            return MAX_LEVEL as u8;
        }
        FACILITY_UNION_ROOM => {
            return UNION_ROOM_MAX_LEVEL;
        }
        _ => {
            if gSpecialVar_0x8004 == FRONTIER_LVL_50 as u16 {
                return FRONTIER_MAX_LEVEL_50;
            }
            return FRONTIER_MAX_LEVEL_OPEN;
        }
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
unsafe fn GetFacilityCancelString() -> *mut u8 {
    let facilityNum: u8 = VarGet(VAR_FRONTIER_FACILITY) as u8;
    if !(facilityNum != FACILITY_UNION_ROOM as u8 && facilityNum != FACILITY_MULTI_OR_EREADER as u8)
    {
        return (*(&raw const crate::data::strings::gText_CancelBattle).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut();
    } else if facilityNum == FRONTIER_FACILITY_DOME as u8 && gSpecialVar_0x8005 == 2 {
        return (*(&raw const crate::data::strings::gText_ReturnToWaitingRoom)
            .cast::<CArray<u8, 0>>())
        .as_ptr()
        .cast_mut();
    } else {
        return (*(&raw const crate::data::strings::gText_CancelChallenge).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut();
    }
    #[allow(unreachable_code)]
    {
        null_mut()
    }
}
pub unsafe fn ChooseMonForTradingBoard(menuType: u8, callback: Option<unsafe fn()>) {
    InitPartyMenu(
        menuType,
        0,
        0,
        0,
        0,
        Some(Task_HandleChooseMonInput),
        callback,
    );
}
#[unsafe(no_mangle)]
pub unsafe fn ChooseMonForMoveTutor() {
    InitPartyMenu(
        0,
        0,
        PARTY_ACTION_MOVE_TUTOR,
        0,
        PARTY_MSG_TEACH_WHICH_MON,
        Some(Task_HandleChooseMonInput),
        Some(CB2_ReturnToFieldContinueScriptPlayMapMusic),
    );
}
#[unsafe(no_mangle)]
pub unsafe fn ChooseMonForWirelessMinigame() {
    InitPartyMenu(
        PARTY_MENU_TYPE_MINIGAME,
        0,
        PARTY_ACTION_MINIGAME,
        0,
        PARTY_MSG_CHOOSE_MON_OR_CANCEL as u8,
        Some(Task_HandleChooseMonInput),
        Some(CB2_ReturnToFieldContinueScriptPlayMapMusic),
    );
}
unsafe fn GetPartyLayoutFromBattleType() -> u8 {
    if IsDoubleBattle() == FALSE {
        return PARTY_LAYOUT_SINGLE;
    }
    if IsMultiBattle() == TRUE {
        return PARTY_LAYOUT_MULTI;
    }
    PARTY_LAYOUT_DOUBLE
}
pub unsafe fn OpenPartyMenuInBattle(partyAction: u8) {
    InitPartyMenu(
        PARTY_MENU_TYPE_IN_BATTLE,
        GetPartyLayoutFromBattleType(),
        partyAction,
        0,
        0,
        Some(Task_HandleChooseMonInput),
        Some(CB2_SetUpReshowBattleScreenAfterMenu),
    );
    ReshowBattleScreenDummy();
    UpdatePartyToBattleOrder();
}
pub unsafe fn ChooseMonForInBattleItem() {
    InitPartyMenu(
        PARTY_MENU_TYPE_IN_BATTLE,
        GetPartyLayoutFromBattleType(),
        PARTY_ACTION_USE_ITEM,
        FALSE,
        PARTY_MSG_USE_ON_WHICH_MON,
        Some(Task_HandleChooseMonInput),
        Some(CB2_ReturnToBagMenu),
    );
    ReshowBattleScreenDummy();
    UpdatePartyToBattleOrder();
}
unsafe fn GetPartyMenuActionsTypeInBattle(mon: *mut Pokemon) -> u8 {
    if GetMonData2(crate::gl!(&raw gPlayerParty[1]), MON_DATA_SPECIES) != 0
        && GetMonData2(mon, MON_DATA_IS_EGG) == 0
    {
        if crate::gl!(gPartyMenu.action) == PARTY_ACTION_SEND_OUT {
            return ACTIONS_SEND_OUT;
        }
        if gBattleTypeFlags.get() & BATTLE_TYPE_ARENA == 0 {
            return ACTIONS_SHIFT;
        }
    }
    ACTIONS_SUMMARY_ONLY as u8
}
unsafe fn TrySwitchInPokemon() -> u8 {
    let slot: u8 = GetCursorSelectionMonId();
    if IsMultiBattle() == 1 && (slot == 1 || slot == 4 || slot == 5) {
        StringCopy(gStringVar1.as_mut_ptr(), GetTrainerPartnerName());
        StringExpandPlaceholders(
            gStringVar4.as_mut_ptr(),
            (*(&raw const crate::data::strings::gText_CantSwitchWithAlly).cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
        );
        return FALSE;
    }
    if GetMonData2(crate::gl!(&raw gPlayerParty[slot]), MON_DATA_HP) == 0 {
        GetMonNickname(
            crate::gl!(&raw gPlayerParty[slot]),
            gStringVar1.as_mut_ptr(),
        );
        StringExpandPlaceholders(
            gStringVar4.as_mut_ptr(),
            (*(&raw const crate::data::strings::gText_PkmnHasNoEnergy).cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
        );
        return FALSE;
    }
    let mut i: u8 = 0;
    while i < gBattlersCount.get() {
        if GetBattlerSide(i) == B_SIDE_PLAYER
            && GetPartyIdFromBattleSlot(slot) as u16 == crate::gl!(gBattlerPartyIndexes[i])
        {
            GetMonNickname(
                crate::gl!(&raw gPlayerParty[slot]),
                gStringVar1.as_mut_ptr(),
            );
            StringExpandPlaceholders(
                gStringVar4.as_mut_ptr(),
                (*(&raw const crate::data::strings::gText_PkmnAlreadyInBattle)
                    .cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
            );
            return FALSE;
        }
        i += 1;
    }
    if GetMonData2(crate::gl!(&raw gPlayerParty[slot]), MON_DATA_IS_EGG) != 0 {
        StringExpandPlaceholders(
            gStringVar4.as_mut_ptr(),
            (*(&raw const crate::data::strings::gText_EggCantBattle).cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
        );
        return FALSE;
    }
    if GetPartyIdFromBattleSlot(slot) == (*crate::gl!(gBattleStruct)).prevSelectedPartySlot {
        GetMonNickname(
            crate::gl!(&raw gPlayerParty[slot]),
            gStringVar1.as_mut_ptr(),
        );
        StringExpandPlaceholders(
            gStringVar4.as_mut_ptr(),
            (*(&raw const crate::data::strings::gText_PkmnAlreadySelected).cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
        );
        return FALSE;
    }
    if crate::gl!(gPartyMenu.action) == PARTY_ACTION_ABILITY_PREVENTS {
        SetMonPreventsSwitchingString();
        return FALSE;
    }
    if crate::gl!(gPartyMenu.action) == PARTY_ACTION_CANT_SWITCH {
        let currBattler: u8 = gBattlerInMenuId.get();
        GetMonNickname(
            &raw mut (*gPlayerParty.as_ptr())
                [GetPartyIdFromBattlePartyId(crate::gl!(gBattlerPartyIndexes[currBattler]) as u8)],
            gStringVar1.as_mut_ptr(),
        );
        StringExpandPlaceholders(
            gStringVar4.as_mut_ptr(),
            (*(&raw const crate::data::strings::gText_PkmnCantSwitchOut).cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
        );
        return FALSE;
    }
    gSelectedMonPartyId.set(GetPartyIdFromBattleSlot(slot));
    gPartyMenuUseExitCallback.set(TRUE);
    let newSlot: u8 = GetPartyIdFromBattlePartyId(
        ({
            let gl_i87 = gBattlerInMenuId.get();
            crate::gl!(gBattlerPartyIndexes[gl_i87])
        }) as u8,
    );
    SwitchPartyMonSlots(newSlot, slot);
    SwapPartyPokemon(
        crate::gl!(&raw gPlayerParty[newSlot]),
        crate::gl!(&raw gPlayerParty[slot]),
    );
    TRUE
}
pub unsafe fn BufferBattlePartyCurrentOrder() {
    BufferBattlePartyOrder(
        (*gBattlePartyCurrentOrder.as_ptr()).as_mut_ptr(),
        GetPlayerFlankId(),
    );
}
unsafe fn BufferBattlePartyOrder(partyBattleOrder: *mut u8, flankId: u8) {
    let mut partyIds: CArray<u8, 6> = zeroed();
    let mut j: i32 = 0;
    if IsMultiBattle() == TRUE {
        if flankId != 0 {
            *partyBattleOrder = 48;
            *partyBattleOrder.at(1) = 69;
            *partyBattleOrder.at(2) = 18;
        } else {
            *partyBattleOrder = 3;
            *partyBattleOrder.at(1) = 18;
            *partyBattleOrder.at(2) = 69;
        }
        return;
    } else if IsDoubleBattle() == FALSE {
        j = 1;
        partyIds[0] = ({
            let gl_i88 = GetBattlerAtPosition(B_POSITION_PLAYER_LEFT);
            crate::gl!(gBattlerPartyIndexes[gl_i88])
        }) as u8;
        for i in 0..PARTY_SIZE {
            if i != partyIds[0] as i32 {
                partyIds[j] = i as u8;
                j += 1;
            }
        }
    } else {
        j = 2;
        partyIds[0] = ({
            let gl_i89 = GetBattlerAtPosition(B_POSITION_PLAYER_LEFT);
            crate::gl!(gBattlerPartyIndexes[gl_i89])
        }) as u8;
        partyIds[1] = ({
            let gl_i90 = GetBattlerAtPosition(B_POSITION_PLAYER_RIGHT);
            crate::gl!(gBattlerPartyIndexes[gl_i90])
        }) as u8;
        for i in 0..PARTY_SIZE {
            if i != partyIds[0] as i32 && i != partyIds[1] as i32 {
                partyIds[j] = i as u8;
                j += 1;
            }
        }
    }
    for i in 0..3i32 {
        *partyBattleOrder.at(i) = partyIds[i * 2] << 4 | partyIds[1 + i * 2];
    }
}
pub unsafe fn BufferBattlePartyCurrentOrderBySide(battler: u8, flankId: u8) {
    BufferBattlePartyOrderBySide(
        (*crate::gl!(gBattleStruct)).battlerPartyOrders[battler].as_mut_ptr(),
        flankId,
        battler,
    );
}
unsafe fn BufferBattlePartyOrderBySide(partyBattleOrder: *mut u8, flankId: u8, battler: u8) {
    let mut partyIndexes: CArray<u8, 6> = zeroed();
    let mut j: i32 = 0;
    let mut leftBattler: u8 = 0;
    let mut rightBattler: u8 = 0;
    if GetBattlerSide(battler) == B_SIDE_PLAYER {
        leftBattler = GetBattlerAtPosition(B_POSITION_PLAYER_LEFT);
        rightBattler = GetBattlerAtPosition(B_POSITION_PLAYER_RIGHT);
    } else {
        leftBattler = GetBattlerAtPosition(B_POSITION_OPPONENT_LEFT);
        rightBattler = GetBattlerAtPosition(B_POSITION_OPPONENT_RIGHT);
    }
    if IsMultiBattle() == TRUE {
        if flankId != 0 {
            *partyBattleOrder = 48;
            *partyBattleOrder.at(1) = 69;
            *partyBattleOrder.at(2) = 18;
        } else {
            *partyBattleOrder = 3;
            *partyBattleOrder.at(1) = 18;
            *partyBattleOrder.at(2) = 69;
        }
        return;
    } else if IsDoubleBattle() == FALSE {
        j = 1;
        partyIndexes[0] = crate::gl!(gBattlerPartyIndexes[leftBattler]) as u8;
        for i in 0..PARTY_SIZE {
            if i != partyIndexes[0] as i32 {
                partyIndexes[j] = i as u8;
                j += 1;
            }
        }
    } else {
        j = 2;
        partyIndexes[0] = crate::gl!(gBattlerPartyIndexes[leftBattler]) as u8;
        partyIndexes[1] = crate::gl!(gBattlerPartyIndexes[rightBattler]) as u8;
        for i in 0..PARTY_SIZE {
            if i != partyIndexes[0] as i32 && i != partyIndexes[1] as i32 {
                partyIndexes[j] = i as u8;
                j += 1;
            }
        }
    }
    for i in 0..3i32 {
        *partyBattleOrder.at(i) = partyIndexes[i * 2] << 4 | partyIndexes[1 + i * 2];
    }
}
pub unsafe fn SwitchPartyOrderLinkMulti(battler: u8, slot: u8, slot2: u8) {
    let mut partyIds: CArray<u8, 6> = zeroed();
    let mut tempSlot: u8 = 0;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut partyBattleOrder: *mut u8 = null_mut();
    let mut partyIdBuffer: u8 = 0;
    if IsMultiBattle() != 0 {
        partyBattleOrder = (*crate::gl!(gBattleStruct)).battlerPartyOrders[battler].as_mut_ptr();
        for i in {
            j = 0;
            j
        }..3
        {
            partyIds[j] = *partyBattleOrder.at(i) >> 4;
            j += 1;
            partyIds[j] = *partyBattleOrder.at(i) & 0xF;
            j += 1;
        }
        partyIdBuffer = partyIds[slot2];
        i = 0;
        while i < PARTY_SIZE {
            if partyIds[i] == slot {
                tempSlot = partyIds[i];
                partyIds[i] = partyIdBuffer;
                break;
            }
            i += 1;
        }
        if i != PARTY_SIZE {
            partyIds[slot2] = tempSlot;
            *partyBattleOrder = partyIds[0] << 4 | partyIds[1];
            *partyBattleOrder.at(1) = partyIds[2] << 4 | partyIds[3];
            *partyBattleOrder.at(2) = partyIds[4] << 4 | partyIds[5];
        }
    }
}
unsafe fn GetPartyIdFromBattleSlot(mut slot: u8) -> u8 {
    let modResult: u8 = slot & 1;
    let mut retVal: u8 = 0;
    slot = (slot as i32 / 2) as u8;
    if modResult != 0 {
        retVal = crate::gl!(gBattlePartyCurrentOrder[slot]) & 0xF;
    } else {
        retVal = crate::gl!(gBattlePartyCurrentOrder[slot]) >> 4;
    }
    retVal
}
unsafe fn SetPartyIdAtBattleSlot(mut slot: u8, setVal: u8) {
    let modResult: u32 = slot as u32 & 1;
    slot = (slot as i32 / 2) as u8;
    if modResult != 0 {
        crate::gl!(
            gBattlePartyCurrentOrder[slot] =
                crate::gl!(gBattlePartyCurrentOrder[slot]) & 0xF0 | setVal
        );
    } else {
        crate::gl!(
            gBattlePartyCurrentOrder[slot] =
                crate::gl!(gBattlePartyCurrentOrder[slot]) & 0xF | setVal << 4
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe fn SwitchPartyMonSlots(slot: u8, slot2: u8) {
    let partyId: u8 = GetPartyIdFromBattleSlot(slot);
    SetPartyIdAtBattleSlot(slot, GetPartyIdFromBattleSlot(slot2));
    SetPartyIdAtBattleSlot(slot2, partyId);
}
#[unsafe(no_mangle)]
pub unsafe fn GetPartyIdFromBattlePartyId(battlePartyId: u8) -> u8 {
    let mut i: u8 = 0;
    let mut j: u8 = {
        i = 0;
        i
    };
    while i < 3 {
        if crate::gl!(gBattlePartyCurrentOrder[i]) >> 4 != battlePartyId {
            j += 1;
            if crate::gl!(gBattlePartyCurrentOrder[i]) as i32 & 0xF == battlePartyId as i32 {
                return j;
            }
        } else {
            return j;
        }
        j += 1;
        i += 1;
    }
    0
}
unsafe fn UpdatePartyToBattleOrder() {
    let partyBuffer: *mut Pokemon = Alloc(600) as *mut Pokemon;
    memcpy(
        partyBuffer as *mut u8,
        (*gPlayerParty.as_ptr()).as_mut_ptr() as *mut u8,
        600,
    );
    for i in 0..(PARTY_SIZE as u8) {
        memcpy(
            ({
                let gl_i91 = GetPartyIdFromBattlePartyId(i);
                crate::gl!(&raw gPlayerParty[gl_i91])
            }) as *mut u8,
            partyBuffer.at(i) as *mut u8,
            100,
        );
    }
    Free(partyBuffer as *mut c_void);
}
unsafe fn UpdatePartyToFieldOrder() {
    let partyBuffer: *mut Pokemon = Alloc(600) as *mut Pokemon;
    memcpy(
        partyBuffer as *mut u8,
        (*gPlayerParty.as_ptr()).as_mut_ptr() as *mut u8,
        600,
    );
    for i in 0..(PARTY_SIZE as u8) {
        memcpy(
            ({
                let gl_i92 = GetPartyIdFromBattleSlot(i);
                crate::gl!(&raw gPlayerParty[gl_i92])
            }) as *mut u8,
            partyBuffer.at(i) as *mut u8,
            100,
        );
    }
    Free(partyBuffer as *mut c_void);
}
unsafe fn SwitchAliveMonIntoLeadSlot() {
    let mut mon: *mut Pokemon = null_mut();
    let mut partyId: u8 = 0;
    for i in 1..(PARTY_SIZE as u8) {
        mon = {
            let gl_i93 = GetPartyIdFromBattleSlot(i);
            crate::gl!(&raw gPlayerParty[gl_i93])
        };
        if GetMonData2(mon, MON_DATA_SPECIES) != 0 && GetMonData2(mon, MON_DATA_HP) != 0 {
            partyId = GetPartyIdFromBattleSlot(0);
            SwitchPartyMonSlots(0, i);
            SwapPartyPokemon(crate::gl!(&raw gPlayerParty[partyId]), mon);
            break;
        }
    }
}
pub(crate) unsafe fn CB2_SetUpExitToBattleScreen() {
    SetMainCallback2(Some(CB2_SetUpReshowBattleScreenAfterMenu));
}
pub unsafe fn ShowPartyMenuToShowcaseMultiBattleParty() {
    InitPartyMenu(
        PARTY_MENU_TYPE_MULTI_SHOWCASE,
        PARTY_LAYOUT_MULTI_SHOWCASE,
        0,
        0,
        PARTY_MSG_NONE,
        Some(Task_InitMultiPartnerPartySlideIn),
        crate::gl!(gMain.savedCallback),
    );
}
pub(crate) unsafe fn Task_InitMultiPartnerPartySlideIn(taskId: u8) {
    task_set(taskId, tXPos, 256);
    SlideMultiPartyMenuBoxSpritesOneStep(taskId);
    ChangeBgX(2, 0x10000, BG_COORD_SET);
    task_set_func(taskId, Some(Task_MultiPartnerPartySlideIn));
}
pub(crate) unsafe fn Task_MultiPartnerPartySlideIn(taskId: u8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    if (*gPaletteFade.as_ptr()).active() == 0 {
        *data -= 8;
        SlideMultiPartyMenuBoxSpritesOneStep(taskId);
        if *data == 0 {
            for i in (MULTI_PARTY_SIZE as u8)..(PARTY_SIZE as u8) {
                if {
                    let gl_i2 = i as i32 - MULTI_PARTY_SIZE;
                    crate::gl!(gMultiPartnerParty[gl_i2].species)
                } != SPECIES_NONE
                {
                    AnimateSelectedPartyIcon((*(*sPartyMenuBoxes.as_ptr()).at(i)).monSpriteId, 0);
                }
            }
            PlaySE(SE_M_HARDEN);
            task_set_func(taskId, Some(Task_WaitAfterMultiPartnerPartySlideIn));
        }
    }
}
pub(crate) unsafe fn Task_WaitAfterMultiPartnerPartySlideIn(taskId: u8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    if ({
        *data += 1;
        *data
    }) == 256
    {
        Task_ClosePartyMenu(taskId);
    }
}
fn MoveMultiPartyMenuBoxSprite(spriteId: u8, x: i16) {
    if x >= 0 {
        crate::gl!(gSprites[spriteId].x2 = x);
    }
}
unsafe fn SlideMultiPartyMenuBoxSpritesOneStep(taskId: u8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    for i in (MULTI_PARTY_SIZE as u8)..(PARTY_SIZE as u8) {
        if {
            let gl_i3 = i as i32 - MULTI_PARTY_SIZE;
            crate::gl!(gMultiPartnerParty[gl_i3].species)
        } != SPECIES_NONE
        {
            MoveMultiPartyMenuBoxSprite(
                (*(*sPartyMenuBoxes.as_ptr()).at(i)).monSpriteId,
                *data - 8,
            );
            MoveMultiPartyMenuBoxSprite(
                (*(*sPartyMenuBoxes.as_ptr()).at(i)).itemSpriteId,
                *data - 8,
            );
            MoveMultiPartyMenuBoxSprite(
                (*(*sPartyMenuBoxes.as_ptr()).at(i)).pokeballSpriteId,
                *data - 8,
            );
            MoveMultiPartyMenuBoxSprite(
                (*(*sPartyMenuBoxes.as_ptr()).at(i)).statusSpriteId,
                *data - 8,
            );
        }
    }
    ChangeBgX(2, 0x800, BG_COORD_ADD);
}
pub unsafe fn ChooseMonForDaycare() {
    InitPartyMenu(
        PARTY_MENU_TYPE_DAYCARE,
        0,
        0,
        0,
        PARTY_MSG_CHOOSE_MON_2 as u8,
        Some(Task_HandleChooseMonInput),
        Some(BufferMonSelection),
    );
}
unsafe fn ChoosePartyMonByMenuType(menuType: u8) {
    crate::gl!(gFieldCallback2 = Some(CB2_FadeFromPartyMenu));
    InitPartyMenu(
        menuType,
        0,
        PARTY_ACTION_CHOOSE_AND_CLOSE,
        0,
        0,
        Some(Task_HandleChooseMonInput),
        Some(CB2_ReturnToField),
    );
}
pub(crate) unsafe fn BufferMonSelection() {
    gSpecialVar_0x8004 = GetCursorSelectionMonId() as u16;
    if gSpecialVar_0x8004 >= PARTY_SIZE as u16 {
        gSpecialVar_0x8004 = PARTY_NOTHING_CHOSEN;
    }
    crate::gl!(gFieldCallback2 = Some(CB2_FadeFromPartyMenu));
    SetMainCallback2(Some(CB2_ReturnToField));
}
pub unsafe fn CB2_FadeFromPartyMenu() -> u8 {
    FadeInFromBlack();
    CreateTask(Some(Task_PartyMenuWaitForFade), 10);
    TRUE
}
pub(crate) unsafe fn Task_PartyMenuWaitForFade(taskId: u8) {
    if IsWeatherNotFadingIn() != 0 {
        DestroyTask(taskId);
        UnlockPlayerFieldControls();
        ScriptContext_Enable();
    }
}
pub unsafe fn ChooseContestMon() {
    LockPlayerFieldControls();
    FadeScreen(FADE_TO_BLACK, 0);
    CreateTask(Some(Task_ChooseContestMon), 10);
}
pub(crate) unsafe fn Task_ChooseContestMon(taskId: u8) {
    if (*gPaletteFade.as_ptr()).active() == 0 {
        CleanupOverworldWindowsAndTilemaps();
        InitPartyMenu(
            PARTY_MENU_TYPE_CONTEST,
            0,
            PARTY_ACTION_CHOOSE_AND_CLOSE,
            0,
            0,
            Some(Task_HandleChooseMonInput),
            Some(CB2_ChooseContestMon),
        );
        DestroyTask(taskId);
    }
}
pub(crate) unsafe fn CB2_ChooseContestMon() {
    gContestMonPartyIndex.set(GetCursorSelectionMonId());
    if gContestMonPartyIndex.get() >= PARTY_SIZE as u8 {
        gContestMonPartyIndex.set(PARTY_NOTHING_CHOSEN as u8);
    }
    gSpecialVar_0x8004 = gContestMonPartyIndex.get() as u16;
    crate::gl!(gFieldCallback2 = Some(CB2_FadeFromPartyMenu));
    SetMainCallback2(Some(CB2_ReturnToField));
}
#[unsafe(no_mangle)]
pub unsafe fn ChoosePartyMon() {
    LockPlayerFieldControls();
    FadeScreen(FADE_TO_BLACK, 0);
    CreateTask(Some(Task_ChoosePartyMon), 10);
}
pub(crate) unsafe fn Task_ChoosePartyMon(taskId: u8) {
    if (*gPaletteFade.as_ptr()).active() == 0 {
        CleanupOverworldWindowsAndTilemaps();
        InitPartyMenu(
            PARTY_MENU_TYPE_CHOOSE_MON,
            0,
            PARTY_ACTION_CHOOSE_AND_CLOSE,
            0,
            0,
            Some(Task_HandleChooseMonInput),
            Some(BufferMonSelection),
        );
        DestroyTask(taskId);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn ChooseMonForMoveRelearner() {
    LockPlayerFieldControls();
    FadeScreen(FADE_TO_BLACK, 0);
    CreateTask(Some(Task_ChooseMonForMoveRelearner), 10);
}
pub(crate) unsafe fn Task_ChooseMonForMoveRelearner(taskId: u8) {
    if (*gPaletteFade.as_ptr()).active() == 0 {
        CleanupOverworldWindowsAndTilemaps();
        InitPartyMenu(
            PARTY_MENU_TYPE_MOVE_RELEARNER,
            0,
            PARTY_ACTION_CHOOSE_AND_CLOSE,
            0,
            0,
            Some(Task_HandleChooseMonInput),
            Some(CB2_ChooseMonForMoveRelearner),
        );
        DestroyTask(taskId);
    }
}
pub(crate) unsafe fn CB2_ChooseMonForMoveRelearner() {
    gSpecialVar_0x8004 = GetCursorSelectionMonId() as u16;
    if gSpecialVar_0x8004 >= PARTY_SIZE as u16 {
        gSpecialVar_0x8004 = PARTY_NOTHING_CHOSEN;
    } else {
        gSpecialVar_0x8005 = GetNumberOfRelearnableMoves({
            let gl_i94 = *(&raw const crate::ffi::gSpecialVar_0x8004)
                .cast::<u16>()
                .cast_mut();
            crate::gl!(&raw gPlayerParty[gl_i94])
        }) as u16;
    }
    crate::gl!(gFieldCallback2 = Some(CB2_FadeFromPartyMenu));
    SetMainCallback2(Some(CB2_ReturnToField));
}
#[unsafe(no_mangle)]
pub unsafe fn DoBattlePyramidMonsHaveHeldItem() {
    gSpecialVar_Result = FALSE as u16;
    for i in 0..(FRONTIER_PARTY_SIZE as u8) {
        if GetMonData2(crate::gl!(&raw gPlayerParty[i]), MON_DATA_HELD_ITEM) != ITEM_NONE as u32 {
            gSpecialVar_Result = TRUE as u16;
            break;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe fn BattlePyramidChooseMonHeldItems() {
    LockPlayerFieldControls();
    FadeScreen(FADE_TO_BLACK, 0);
    CreateTask(Some(Task_BattlePyramidChooseMonHeldItems), 10);
}
pub(crate) unsafe fn Task_BattlePyramidChooseMonHeldItems(taskId: u8) {
    if (*gPaletteFade.as_ptr()).active() == 0 {
        CleanupOverworldWindowsAndTilemaps();
        InitPartyMenu(
            PARTY_MENU_TYPE_STORE_PYRAMID_HELD_ITEMS,
            0,
            0,
            0,
            0,
            Some(Task_HandleChooseMonInput),
            Some(BufferMonSelection),
        );
        DestroyTask(taskId);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn MoveDeleterChooseMoveToForget() {
    ShowPokemonSummaryScreen(
        SUMMARY_MODE_SELECT_MOVE,
        (*gPlayerParty.as_ptr()).as_mut_ptr() as *mut c_void,
        gSpecialVar_0x8004 as u8,
        gPlayerPartyCount.get() - 1,
        Some(CB2_ReturnToField),
    );
    crate::gl!(gFieldCallback = Some(FieldCB_ContinueScriptHandleMusic));
}
#[unsafe(no_mangle)]
pub unsafe fn GetNumMovesSelectedMonHas() {
    gSpecialVar_Result = 0;
    for i in 0..(MAX_MON_MOVES as u8) {
        if GetMonData2(
            {
                let gl_i95 = *(&raw const crate::ffi::gSpecialVar_0x8004)
                    .cast::<u16>()
                    .cast_mut();
                crate::gl!(&raw gPlayerParty[gl_i95])
            },
            MON_DATA_MOVE1 + i as i32,
        ) != MOVE_NONE as u32
        {
            gSpecialVar_Result += 1;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe fn BufferMoveDeleterNicknameAndMove() {
    let mon: *mut Pokemon = {
        let gl_i96 = *(&raw const crate::ffi::gSpecialVar_0x8004)
            .cast::<u16>()
            .cast_mut();
        crate::gl!(&raw gPlayerParty[gl_i96])
    };
    let r#move: u16 = GetMonData2(mon, MON_DATA_MOVE1 + gSpecialVar_0x8005 as i32) as u16;
    GetMonNickname(mon, gStringVar1.as_mut_ptr());
    StringCopy(
        (*gStringVar2.as_ptr()).as_mut_ptr(),
        (*(&raw const crate::data::data_tables::gMoveNames).cast::<CArray<CArray<u8, 13>, 355>>())
            [r#move]
            .as_ptr()
            .cast_mut(),
    );
}
#[unsafe(no_mangle)]
pub unsafe fn MoveDeleterForgetMove() {
    SetMonMoveSlot(
        {
            let gl_i97 = *(&raw const crate::ffi::gSpecialVar_0x8004)
                .cast::<u16>()
                .cast_mut();
            crate::gl!(&raw gPlayerParty[gl_i97])
        },
        MOVE_NONE,
        gSpecialVar_0x8005 as u8,
    );
    RemoveMonPPBonus(
        {
            let gl_i98 = *(&raw const crate::ffi::gSpecialVar_0x8004)
                .cast::<u16>()
                .cast_mut();
            crate::gl!(&raw gPlayerParty[gl_i98])
        },
        gSpecialVar_0x8005 as u8,
    );
    for i in gSpecialVar_0x8005..3 {
        ShiftMoveSlot(
            {
                let gl_i99 = *(&raw const crate::ffi::gSpecialVar_0x8004)
                    .cast::<u16>()
                    .cast_mut();
                crate::gl!(&raw gPlayerParty[gl_i99])
            },
            i as u8,
            i as u8 + 1,
        );
    }
}
unsafe fn ShiftMoveSlot(mon: *mut Pokemon, slotTo: u8, slotFrom: u8) {
    let mut move1: u16 = GetMonData2(mon, MON_DATA_MOVE1 + slotTo as i32) as u16;
    let mut move0: u16 = GetMonData2(mon, MON_DATA_MOVE1 + slotFrom as i32) as u16;
    let mut pp1: u8 = GetMonData2(mon, MON_DATA_PP1 + slotTo as i32) as u8;
    let mut pp0: u8 = GetMonData2(mon, MON_DATA_PP1 + slotFrom as i32) as u8;
    let mut ppBonuses: u8 = GetMonData2(mon, MON_DATA_PP_BONUSES) as u8;
    let ppBonusMask1: u8 =
        (*(&raw const crate::data::pokemon::gPPUpGetMask).cast::<CArray<u8, 0>>())[slotTo];
    let ppBonusMove1: u8 = shr_i32(ppBonuses as i32 & ppBonusMask1 as i32, slotTo as u32 * 2) as u8;
    let ppBonusMask2: u8 =
        (*(&raw const crate::data::pokemon::gPPUpGetMask).cast::<CArray<u8, 0>>())[slotFrom];
    let ppBonusMove2: u8 =
        shr_i32(ppBonuses as i32 & ppBonusMask2 as i32, slotFrom as u32 * 2) as u8;
    ppBonuses &= !ppBonusMask1;
    ppBonuses &= !ppBonusMask2;
    ppBonuses |= shl_i32(ppBonusMove1 as i32, slotFrom as u32 * 2) as u8
        + shl_i32(ppBonusMove2 as i32, slotTo as u32 * 2) as u8;
    SetMonData(
        mon,
        MON_DATA_MOVE1 + slotTo as i32,
        &raw mut move0 as *mut c_void,
    );
    SetMonData(
        mon,
        MON_DATA_MOVE1 + slotFrom as i32,
        &raw mut move1 as *mut c_void,
    );
    SetMonData(
        mon,
        MON_DATA_PP1 + slotTo as i32,
        &raw mut pp0 as *mut c_void,
    );
    SetMonData(
        mon,
        MON_DATA_PP1 + slotFrom as i32,
        &raw mut pp1 as *mut c_void,
    );
    SetMonData(mon, MON_DATA_PP_BONUSES, &raw mut ppBonuses as *mut c_void);
}
#[unsafe(no_mangle)]
pub unsafe fn IsSelectedMonEgg() {
    if GetMonData2(
        {
            let gl_i100 = *(&raw const crate::ffi::gSpecialVar_0x8004)
                .cast::<u16>()
                .cast_mut();
            crate::gl!(&raw gPlayerParty[gl_i100])
        },
        MON_DATA_IS_EGG,
    ) != 0
    {
        gSpecialVar_Result = TRUE as u16;
    } else {
        gSpecialVar_Result = FALSE as u16;
    }
}
#[unsafe(no_mangle)]
pub unsafe fn IsLastMonThatKnowsSurf() {
    let mut i: u32 = 0;
    gSpecialVar_Result = FALSE as u16;
    let r#move: u16 = GetMonData2(
        {
            let gl_i101 = *(&raw const crate::ffi::gSpecialVar_0x8004)
                .cast::<u16>()
                .cast_mut();
            crate::gl!(&raw gPlayerParty[gl_i101])
        },
        MON_DATA_MOVE1 + gSpecialVar_0x8005 as i32,
    ) as u16;
    if r#move == MOVE_SURF {
        i = 0;
        while i < CalculatePlayerPartyCount() as u32 {
            if i != gSpecialVar_0x8004 as u32 {
                for j in 0..(MAX_MON_MOVES as u32) {
                    if GetMonData2(crate::gl!(&raw gPlayerParty[i]), MON_DATA_MOVE1 + j as i32)
                        == MOVE_SURF as u32
                    {
                        return;
                    }
                }
            }
            i += 1;
        }
        if AnyStorageMonWithMove(r#move) != TRUE as u32 {
            gSpecialVar_Result = TRUE as u16;
        }
    }
}
