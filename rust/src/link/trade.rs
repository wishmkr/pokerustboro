//! Translated from `src/trade.c` by tools/rustport/c2rs.py.
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
    clippy::missing_transmute_annotations,
    clippy::too_many_arguments,
    clippy::unnecessary_cast,
    clippy::useless_transmute,
    dead_code,
    unused_assignments,
    unused_variables
)]

use crate::AgbRfu_LinkManager::lman;
use crate::agb_main::gMain;
use crate::agb_main::{SetVBlankCallback, gSoftResetDisabled};
use crate::battle_anim_mons::{StoreSpriteCallbackInData6, Trade_MoveSelectedMonToTarget};
use crate::battle_gfx_sfx_util::{AllocateMonSpritesGfx, FreeMonSpritesGfx};
use crate::battle_interface::GetHPBarLevel;
use crate::battle_main::gMonSpritesGfxPtr;
use crate::bg::{
    ChangeBgX, ChangeBgY, CopyBgTilemapBufferToVram, FillBgTilemapBufferRect,
    FillBgTilemapBufferRect_Palette0, ResetBgsAndClearDma3BusyFlags, ShowBg,
};
#[allow(unused_imports)]
use crate::c::*;
use crate::cable_club::Task_WaitForLinkPlayerConnection;
#[allow(unused_imports)]
use crate::consts::*;
use crate::daycare::NameHasGenderSymbol;
use crate::event_data::{FlagSet, IsNationalPokedexEnabled};
use crate::evolution_scene::{TradeEvolutionScene, gCB2_AfterEvolution};
use crate::ffi::{gSpecialVar_0x8004, gSpecialVar_0x8005};
use crate::field_screen_effect::FieldCB_ContinueScriptHandleMusic;
use crate::gpu_regs::{ClearGpuRegBits, GetGpuReg, SetGpuReg, SetGpuRegBits};
use crate::international_string_util::PadNameString;
use crate::librfu_rfu::{gRfuSlotStatusNI, rfu_NI_setSendData, rfu_clearSlot};
use crate::link::{
    BitmaskAllOtherLinkPlayers, CB2_LinkError, CheckShouldAdvanceLinkState, CloseLink,
    GetBlockReceivedStatus, GetLinkPlayerCount, GetLinkPlayerCount_2, GetMultiplayerId,
    GetSavedPlayerCount, HasLinkErrorOccurred, IsLinkMaster, IsLinkPlayerDataExchangeComplete,
    IsLinkTaskFinished, OpenLink, ResetBlockReceivedFlag, ResetBlockReceivedFlags, SendBlock,
    SendBlockRequest, SetCloseLinkCallback, SetCloseLinkCallbackAndType, SetLinkStandbyCallback,
    SetWirelessCommType1, gLinkPlayers, gLinkType, gReceivedRemoteLinkPlayers, gWirelessCommType,
};
use crate::link::{gBlockRecvBuffer, gBlockSendBuffer};
use crate::link_rfu_2::{
    CreateTask_RfuIdle, DestroyTask_RfuIdle, IsLinkRfuTaskFinished, Rfu_GetIndexOfNewestChild,
    Rfu_SetLinkRecovery,
};
use crate::link_rfu_3::{
    CreateWirelessStatusIndicatorSprite, DestroyWirelessStatusIndicatorSprite,
    LoadWirelessStatusIndicatorSpriteGfx,
};
use crate::load_save::{ClearContinueGameWarpStatus2, SetContinueGameWarpStatusToDynamicWarp};
use crate::load_save::{gSaveBlock1Ptr, gSaveBlock2Ptr};
use crate::mail_data::ItemIsMail;
use crate::menu::{
    AddTextPrinterParameterized3, AddTextPrinterParameterized4, CreateYesNoMenu,
    DecompressAndLoadBgGfxUsingHeap, InitMenuInUpperLeftCornerNormal, Menu_ProcessInputNoWrap,
    Menu_ProcessInputNoWrapClearOnChoose, PrintMenuTable, RunTextPrintersAndIsPrinter0Active,
};
use crate::mystery_gift::MysteryGift_TryIncrementStat;
use crate::overworld::{
    CB2_ReturnToField, CB2_ReturnToFieldFromMultiplayer, IncrementGameStat, gFieldCallback,
};
use crate::palette::{
    BeginNormalPaletteFade, BlendPalettes, LoadCompressedPalette, LoadPalette, ResetPaletteFade,
    TransferPlttBuffer, UpdatePaletteFade, gPaletteFade,
};
use crate::party_menu::{DrawHeldItemIconsForTrade, LoadHeldItemIcons};
use crate::pokeball::{CreatePokeballSpriteToReleaseMon, CreateTradePokeballSprite};
use crate::pokedex::GetSetPokedexFlag;
use crate::pokemon::{
    CalculateEnemyPartyCount, CalculateMonStats, CalculatePlayerPartyCount, CreateMon,
    GetEvolutionTargetSpecies, GetMonData2, GetMonData3, GetMonGender, GetMonSpritePalStruct,
    HandleSetPokedexFlag, IsMonSpriteNotFlipped, IsSpeciesInHoennDex, SetMonData,
    SetMultiuseSpriteTemplateToPokemon, SpeciesToNationalPokedexNum, gEnemyParty, gEnemyPartyCount,
    gMultiuseSpriteTemplate, gPlayerParty, gPlayerPartyCount,
};
use crate::pokemon_icon::{
    CreateMonIcon, LoadMonIconPalettes, SetPartyHPBarSprite, SpriteCB_MonIcon,
};
use crate::pokemon_storage_system::DrawTextWindowAndBufferTiles;
use crate::pokemon_summary_screen::{ShowPokemonSummaryScreen, gLastViewedMonIndex};
use crate::random::Random;
use crate::save::{
    LinkFullSave_Init, LinkFullSave_ReplaceLastSector, LinkFullSave_SetLastSectorSignature,
    LinkFullSave_WriteSector,
};
use crate::script::LockPlayerFieldControls;
use crate::sound::{
    FadeOutBGM, GetCurrentMapMusic, IsBGMStopped, IsCryFinished, PlayBGM, PlayCry_Normal,
    PlayFanfare, PlayNewMapMusic, PlaySE,
};
use crate::sprite::gSprites;
use crate::sprite::{
    AnimateSprites, BuildOamBuffer, FreeAllSpritePalettes, LoadOam, ProcessSpriteCopyRequests,
    ResetSpriteData,
};
use crate::string_util::{gStringVar1, gStringVar2, gStringVar3, gStringVar4};
use crate::task::{DestroyTask, ResetTasks, RunTasks};
use crate::task::{gTasks, task_set};
use crate::text::{DeactivateAllTextPrinters, GetFontAttribute, RunTextPrinters};
use crate::text_window::{
    DrawTextBorderOuter, LoadUserWindowBorderGfx, LoadUserWindowBorderGfx_, rbox_fill_rectangle,
};
#[allow(unused_imports)]
use crate::types::*;
use crate::union_room::{InUnionRoom, gPlayerCurrActivity};
use crate::window::{
    ClearWindowTilemap, CopyWindowToVram, FillWindowPixelBuffer, FreeAllWindowBuffers,
    PutWindowTilemap,
};
#[allow(unused_imports)]
use core::ffi::c_void;
#[allow(unused_imports)]
use core::mem::zeroed;
#[allow(unused_imports)]
use core::ptr::null_mut;
/// `CalcCenterToCornerVec` with this module's view of its types.
#[inline]
unsafe fn CalcCenterToCornerVec(a0: *mut Sprite, a1: u8, a2: u8, a3: u8) {
    unsafe {
        crate::sprite::CalcCenterToCornerVec(a0 as _, a1, a2, a3);
    }
}
/// `ClearMail` with this module's view of its types.
#[inline]
unsafe fn ClearMail(a0: *mut Mail) {
    unsafe {
        crate::mail_data::ClearMail(a0 as _);
    }
}
/// `CopyToBgTilemapBuffer` with this module's view of its types.
#[inline]
unsafe fn CopyToBgTilemapBuffer(a0: u8, a1: *mut c_void, a2: u16, a3: u16) {
    unsafe {
        crate::bg::CopyToBgTilemapBuffer(a0, a1 as _, a2, a3);
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
/// `DestroySprite` with this module's view of its types.
#[inline]
unsafe fn DestroySprite(a0: *mut Sprite) {
    unsafe {
        crate::sprite::DestroySprite(a0 as _);
    }
}
/// `DoBgAffineSet` with this module's view of its types.
#[inline]
fn DoBgAffineSet(
    a0: *mut BgAffineDstData,
    a1: u32,
    a2: u32,
    a3: i16,
    a4: i16,
    a5: i16,
    a6: i16,
    a7: u16,
) {
    unsafe {
        crate::util::DoBgAffineSet(a0 as _, a1, a2, a3, a4, a5, a6, a7);
    }
}
/// `Free` with this module's view of its types.
#[inline]
unsafe fn Free(a0: *mut c_void) {
    unsafe {
        crate::malloc::Free(a0 as _);
    }
}
/// `FreeSpriteOamMatrix` with this module's view of its types.
#[inline]
unsafe fn FreeSpriteOamMatrix(a0: *mut Sprite) {
    unsafe {
        crate::sprite::FreeSpriteOamMatrix(a0 as _);
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
/// `GetStringWidth` with this module's view of its types.
#[inline]
unsafe fn GetStringWidth(a0: u8, a1: *mut u8, a2: i16) -> i32 {
    unsafe { crate::text::GetStringWidth(a0, a1 as _, a2) }
}
/// `GiveMailToMon` with this module's view of its types.
#[inline]
fn GiveMailToMon(a0: *mut Pokemon, a1: *mut Mail) -> u8 {
    unsafe { crate::mail_data::GiveMailToMon(a0 as _, a1 as _) }
}
/// `HandleLoadSpecialPokePic_2` with this module's view of its types.
#[inline]
fn HandleLoadSpecialPokePic_2(a0: *mut CompressedSpriteSheet, a1: *mut c_void, a2: i32, a3: u32) {
    unsafe {
        crate::decompress::HandleLoadSpecialPokePic_2(a0 as _, a1 as _, a2, a3);
    }
}
/// `HandleLoadSpecialPokePic_DontHandleDeoxys` with this module's view of its types.
#[inline]
fn HandleLoadSpecialPokePic_DontHandleDeoxys(
    a0: *mut CompressedSpriteSheet,
    a1: *mut c_void,
    a2: i32,
    a3: u32,
) {
    unsafe {
        crate::decompress::HandleLoadSpecialPokePic_DontHandleDeoxys(a0 as _, a1 as _, a2, a3);
    }
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
/// `LoadBgTilemap` with this module's view of its types.
#[inline]
unsafe fn LoadBgTilemap(a0: u8, a1: *mut c_void, a2: u16, a3: u16) -> u16 {
    unsafe { crate::bg::LoadBgTilemap(a0, a1 as _, a2, a3) }
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
/// `SpriteCallbackDummy` with this module's view of its types.
#[inline]
unsafe fn SpriteCallbackDummy(a0: *mut Sprite) {
    unsafe {
        crate::sprite::SpriteCallbackDummy(a0 as _);
    }
}
/// `StartSpriteAffineAnim` with this module's view of its types.
#[inline]
unsafe fn StartSpriteAffineAnim(a0: *mut Sprite, a1: u8) {
    unsafe {
        crate::sprite::StartSpriteAffineAnim(a0 as _, a1);
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
/// `StringCompareWithoutExtCtrlCodes` with this module's view of its types.
#[inline]
unsafe fn StringCompareWithoutExtCtrlCodes(a0: *mut u8, a1: *mut u8) -> i32 {
    unsafe { crate::string_util::StringCompareWithoutExtCtrlCodes(a0 as _, a1 as _) }
}
/// `StringCopy` with this module's view of its types.
#[inline]
unsafe fn StringCopy(a0: *mut u8, a1: *mut u8) -> *mut u8 {
    unsafe { crate::string_util::StringCopy(a0 as _, a1 as _) as *mut u8 }
}
/// `StringCopy_Nickname` with this module's view of its types.
#[inline]
unsafe fn StringCopy_Nickname(a0: *mut u8, a1: *mut u8) -> *mut u8 {
    unsafe { crate::string_util::StringCopy_Nickname(a0 as _, a1 as _) as *mut u8 }
}
/// `StringExpandPlaceholders` with this module's view of its types.
#[inline]
unsafe fn StringExpandPlaceholders(a0: *mut u8, a1: *mut u8) -> *mut u8 {
    unsafe { crate::string_util::StringExpandPlaceholders(a0 as _, a1 as _) as *mut u8 }
}
// The C's names for task and sprite data slots.
const tSignalComingBack: usize = 2;
// Data tables (translate with cdata.py): sUnusedStructSizes sTradeMovesBoxTilemap sTradePartyBoxTilemap sTradeStripesBG2Tilemap sTradeStripesBG3Tilemap sText_EmptyString sText_UnusedTextFormat gText_MaleSymbol4 gText_FemaleSymbol4 gText_GenderlessSymbol sText_SpaceMove sText_NewLine sText_Slash sText_Lv sText_ThreeDashes sText_FourQuestionMarks sText_UnusedEmpty sText_IsThisTradeOkay sText_Cancel sText_ChooseAPkmn sText_Summary sText_Trade sText_CancelTrade sJPText_PressBButtonToQuit sText_Summary2 sText_Trade2 sText_CommunicationStandby sText_TheTradeHasBeenCanceled sText_OnlyPkmnForBattle sText_WaitingForYourFriend sText_YourFriendWantsToTrade sOamData_MenuText sOamData_Cursor sAnim_Cursor_Normal sAnim_Cursor_OnCancel sAnims_Cursor sCursor_SpriteSheet sCursor_SpritePalette sAnim_MenuText_0 sAnim_MenuText_1 sAnim_MenuText_2 sAnim_MenuText_3 sAnim_MenuText_4 sAnim_MenuText_5 sAnims_MenuText sSpriteTemplate_Cursor sSpriteTemplate_MenuText sMenuText_Pal sSpritePalette_MenuText sCursorMoveDestinations sTradeMonSpriteCoords sTradeMonLevelCoords sTradeMonBoxCoords sUnusedCoords sActionTexts sSelectTradeMonActions sMessages sTradeTextColors sBgTemplates sWindowTemplates sTradeYesNoWindowTemplate sText_ShedinjaJP sSelectedMonLevelGenderCoords sPokeball_Pal sPokeball_Gfx sPokeballSymbol_Gfx sCableCloseup_Map sPokeballSymbol_Map sUnusedPal1 sGba_Pal sUnusedPal2 sWirelessSignalNone_Pal_Unused sLinkMon_Pal sLinkMonGlow_Gfx sLinkMonShadow_Gfx sCableEnd_Gfx sGbaScreen_Gfx gTradePlatform_Tilemap sGbaAffine_Gfx sEmptyGfx sGbaAffineMapCable sGbaAffineMapWireless sGbaMapWireless sGbaMapCable sWirelessCloseup_Map sWirelessSignalSend_Pal sWirelessSignalRecv_Pal sWirelessSignalNone_Pal sWirelessSignal_Gfx sWirelessSignal_Tilemap sOamData_Pokeball sAnim_Pokeball_SpinOnce sAnim_Pokeball_SpinTwice sAnims_Pokeball sAffineAnim_Pokeball_Normal sAffineAnim_Pokeball_Squish sAffineAnim_Pokeball_Unsquish sAffineAnims_Pokeball sPokeBallSpriteSheet sPokeBallSpritePalette sSpriteTemplate_Pokeball sOamData_LinkMonGlow sAnim_LinkMonGlow sAnims_LinkMonGlow sAffineAnim_LinkMonGlow sAffineAnims_LinkMonGlow sSpriteSheet_LinkMonGlow sSpritePalette_LinkMon sSpritePalette_Gba sSpriteTemplate_LinkMonGlow sOamData_LinkMonShadow sAnim_LinkMonShadow_Big sAnim_LinkMonShadow_Small sAnims_LinkMonShadow sSpriteSheet_LinkMonShadow sSpriteTemplate_LinkMonShadow sOamData_CableEnd sAnim_CableEnd sAnims_CableEnd sSpriteSheet_CableEnd sSpriteTemplate_CableEnd sOamData_GbaScreen sAnim_GbaScreen_Long sAnim_GbaScreen_Short sAnims_GbaScreen_Long sAnims_GbaScreen_Short sSpriteSheet_GbaScreen sSpriteTemplate_GbaScreenFlash_Long sSpriteTemplate_GbaScreenFlash_Short sLinkMonShadow_Pal sAffineAnim_CrossingMonPic sAffineAnims_CrossingMonPics sIngameTrades sIngameTradeMail sTradeSequenceWindowTemplates gTradeEvolutionSceneYesNoWindowTemplate sTradeSequenceBgTemplates sTradeBallVerticalVelocityTable sWirelessSignalAnimParams

/// `__typeof__(*((__typeof__(sTradeMenu))0))`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct typeof___sTradeMenu_0_t {
    pub bg2hofs: u8,
    pub bg3hofs: u8,
    pub filler_2: CArray<u8, 38>,
    pub partySpriteIds: CArray<CArray<u8, 6>, 2>,
    pub cursorSpriteId: u8,
    pub cursorPosition: u8,
    pub partyCounts: CArray<u8, 2>,
    pub optionsActive: CArray<u8, 13>,
    pub isLiveMon: CArray<CArray<u8, 6>, 2>,
    pub isEgg: CArray<CArray<u8, 6>, 2>,
    pub hpBarLevels: CArray<CArray<u8, 6>, 2>,
    pub bufferPartyState: u8,
    pub filler_6A: CArray<u8, 5>,
    pub callbackId: u8,
    pub neverRead_70: u8,
    pub bottomTextTileStart: u16,
    pub drawSelectedMonState: CArray<u8, 2>,
    pub selectedMonIdx: CArray<u8, 2>,
    pub playerSelectStatus: u8,
    pub partnerSelectStatus: u8,
    pub playerConfirmStatus: u8,
    pub partnerConfirmStatus: u8,
    pub filler_7C: CArray<u8, 2>,
    pub partnerCursorPosition: u8,
    pub linkData: CArray<u16, 20>,
    pub timer: u8,
    pub giftRibbons: CArray<u8, 11>,
    pub filler_B4: CArray<u8, 2076>,
    pub queuedActions: CArray<typeof___sTradeMenu_0_t_queuedActions, 4>,
    pub tilemapBuffer: CArray<u16, 1024>,
}

unsafe impl Sync for typeof___sTradeMenu_0_t {}

/// `__typeof__(*((__typeof__(sTradeAnim))0))`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct typeof___sTradeAnim_0_t {
    pub tempMon: Pokemon,
    pub timer: u32,
    pub monPersonalities: CArray<u32, 2>,
    pub filler_70: CArray<u8, 2>,
    pub playerFinishStatus: u8,
    pub partnerFinishStatus: u8,
    pub linkData: CArray<u16, 10>,
    pub linkTimeoutZero1: u8,
    pub linkTimeoutZero2: u8,
    pub linkTimeoutTimer: u16,
    pub neverRead_8C: u16,
    pub monSpriteIds: CArray<u8, 2>,
    pub connectionSpriteId1: u8,
    pub connectionSpriteId2: u8,
    pub cableEndSpriteId: u8,
    pub scheduleLinkTransfer: u8,
    pub state: u16,
    pub filler_96: CArray<u8, 60>,
    pub releasePokeballSpriteId: u8,
    pub bouncingPokeballSpriteId: u8,
    pub texX: u16,
    pub texY: u16,
    pub neverRead_D8: u16,
    pub neverRead_DA: u16,
    pub scrX: u16,
    pub scrY: u16,
    pub bg1vofs: i16,
    pub bg1hofs: i16,
    pub bg2vofs: i16,
    pub bg2hofs: i16,
    pub sXY: u16,
    pub gbaScale: u16,
    pub alpha: u16,
    pub isLinkTrade: u8,
    pub monSpecies: CArray<u16, 2>,
    pub cachedMapMusic: u16,
    pub textColors: CArray<u8, 3>,
    pub filler_F9: u8,
    pub isCableTrade: u8,
    pub wirelessWinLeft: u8,
    pub wirelessWinTop: u8,
    pub wirelessWinRight: u8,
    pub wirelessWinBottom: u8,
}

unsafe impl Sync for typeof___sTradeAnim_0_t {}

/// `struct InGameTrade`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct InGameTrade {
    pub nickname: CArray<u8, 11>,
    pub species: u16,
    pub ivs: CArray<u8, 6>,
    pub abilityNum: u8,
    pub otId: u32,
    pub conditions: CArray<u8, 5>,
    pub personality: u32,
    pub heldItem: u16,
    pub mailNum: u8,
    pub otName: CArray<u8, 11>,
    pub otGender: u8,
    pub sheen: u8,
    pub requestedSpecies: u16,
}

unsafe impl Sync for InGameTrade {}

/// The anonymous type of `typeof___sTradeMenu_0_t::queuedActions`.
#[repr(C, align(4))]
#[derive(Clone, Copy)]
pub struct typeof___sTradeMenu_0_t_queuedActions {
    pub active: u8,
    pub delay: u16,
    pub actionId: u8,
}

unsafe impl Sync for typeof___sTradeMenu_0_t_queuedActions {}

#[cfg(target_arch = "arm")]
const _: () = {
    #[allow(unused_imports)]
    use core::mem::{offset_of, size_of};
    assert!(size_of::<typeof___sTradeMenu_0_t>() == 4336);
    assert!(offset_of!(typeof___sTradeMenu_0_t, bg2hofs) == 0);
    assert!(offset_of!(typeof___sTradeMenu_0_t, bg3hofs) == 1);
    assert!(offset_of!(typeof___sTradeMenu_0_t, filler_2) == 2);
    assert!(offset_of!(typeof___sTradeMenu_0_t, partySpriteIds) == 40);
    assert!(offset_of!(typeof___sTradeMenu_0_t, cursorSpriteId) == 52);
    assert!(offset_of!(typeof___sTradeMenu_0_t, cursorPosition) == 53);
    assert!(offset_of!(typeof___sTradeMenu_0_t, partyCounts) == 54);
    assert!(offset_of!(typeof___sTradeMenu_0_t, optionsActive) == 56);
    assert!(offset_of!(typeof___sTradeMenu_0_t, isLiveMon) == 69);
    assert!(offset_of!(typeof___sTradeMenu_0_t, isEgg) == 81);
    assert!(offset_of!(typeof___sTradeMenu_0_t, hpBarLevels) == 93);
    assert!(offset_of!(typeof___sTradeMenu_0_t, bufferPartyState) == 105);
    assert!(offset_of!(typeof___sTradeMenu_0_t, filler_6A) == 106);
    assert!(offset_of!(typeof___sTradeMenu_0_t, callbackId) == 111);
    assert!(offset_of!(typeof___sTradeMenu_0_t, neverRead_70) == 112);
    assert!(offset_of!(typeof___sTradeMenu_0_t, bottomTextTileStart) == 114);
    assert!(offset_of!(typeof___sTradeMenu_0_t, drawSelectedMonState) == 116);
    assert!(offset_of!(typeof___sTradeMenu_0_t, selectedMonIdx) == 118);
    assert!(offset_of!(typeof___sTradeMenu_0_t, playerSelectStatus) == 120);
    assert!(offset_of!(typeof___sTradeMenu_0_t, partnerSelectStatus) == 121);
    assert!(offset_of!(typeof___sTradeMenu_0_t, playerConfirmStatus) == 122);
    assert!(offset_of!(typeof___sTradeMenu_0_t, partnerConfirmStatus) == 123);
    assert!(offset_of!(typeof___sTradeMenu_0_t, filler_7C) == 124);
    assert!(offset_of!(typeof___sTradeMenu_0_t, partnerCursorPosition) == 126);
    assert!(offset_of!(typeof___sTradeMenu_0_t, linkData) == 128);
    assert!(offset_of!(typeof___sTradeMenu_0_t, timer) == 168);
    assert!(offset_of!(typeof___sTradeMenu_0_t, giftRibbons) == 169);
    assert!(offset_of!(typeof___sTradeMenu_0_t, filler_B4) == 180);
    assert!(offset_of!(typeof___sTradeMenu_0_t, queuedActions) == 2256);
    assert!(offset_of!(typeof___sTradeMenu_0_t, tilemapBuffer) == 2288);
    assert!(size_of::<typeof___sTradeAnim_0_t>() == 256);
    assert!(offset_of!(typeof___sTradeAnim_0_t, tempMon) == 0);
    assert!(offset_of!(typeof___sTradeAnim_0_t, timer) == 100);
    assert!(offset_of!(typeof___sTradeAnim_0_t, monPersonalities) == 104);
    assert!(offset_of!(typeof___sTradeAnim_0_t, filler_70) == 112);
    assert!(offset_of!(typeof___sTradeAnim_0_t, playerFinishStatus) == 114);
    assert!(offset_of!(typeof___sTradeAnim_0_t, partnerFinishStatus) == 115);
    assert!(offset_of!(typeof___sTradeAnim_0_t, linkData) == 116);
    assert!(offset_of!(typeof___sTradeAnim_0_t, linkTimeoutZero1) == 136);
    assert!(offset_of!(typeof___sTradeAnim_0_t, linkTimeoutZero2) == 137);
    assert!(offset_of!(typeof___sTradeAnim_0_t, linkTimeoutTimer) == 138);
    assert!(offset_of!(typeof___sTradeAnim_0_t, neverRead_8C) == 140);
    assert!(offset_of!(typeof___sTradeAnim_0_t, monSpriteIds) == 142);
    assert!(offset_of!(typeof___sTradeAnim_0_t, connectionSpriteId1) == 144);
    assert!(offset_of!(typeof___sTradeAnim_0_t, connectionSpriteId2) == 145);
    assert!(offset_of!(typeof___sTradeAnim_0_t, cableEndSpriteId) == 146);
    assert!(offset_of!(typeof___sTradeAnim_0_t, scheduleLinkTransfer) == 147);
    assert!(offset_of!(typeof___sTradeAnim_0_t, state) == 148);
    assert!(offset_of!(typeof___sTradeAnim_0_t, filler_96) == 150);
    assert!(offset_of!(typeof___sTradeAnim_0_t, releasePokeballSpriteId) == 210);
    assert!(offset_of!(typeof___sTradeAnim_0_t, bouncingPokeballSpriteId) == 211);
    assert!(offset_of!(typeof___sTradeAnim_0_t, texX) == 212);
    assert!(offset_of!(typeof___sTradeAnim_0_t, texY) == 214);
    assert!(offset_of!(typeof___sTradeAnim_0_t, neverRead_D8) == 216);
    assert!(offset_of!(typeof___sTradeAnim_0_t, neverRead_DA) == 218);
    assert!(offset_of!(typeof___sTradeAnim_0_t, scrX) == 220);
    assert!(offset_of!(typeof___sTradeAnim_0_t, scrY) == 222);
    assert!(offset_of!(typeof___sTradeAnim_0_t, bg1vofs) == 224);
    assert!(offset_of!(typeof___sTradeAnim_0_t, bg1hofs) == 226);
    assert!(offset_of!(typeof___sTradeAnim_0_t, bg2vofs) == 228);
    assert!(offset_of!(typeof___sTradeAnim_0_t, bg2hofs) == 230);
    assert!(offset_of!(typeof___sTradeAnim_0_t, sXY) == 232);
    assert!(offset_of!(typeof___sTradeAnim_0_t, gbaScale) == 234);
    assert!(offset_of!(typeof___sTradeAnim_0_t, alpha) == 236);
    assert!(offset_of!(typeof___sTradeAnim_0_t, isLinkTrade) == 238);
    assert!(offset_of!(typeof___sTradeAnim_0_t, monSpecies) == 240);
    assert!(offset_of!(typeof___sTradeAnim_0_t, cachedMapMusic) == 244);
    assert!(offset_of!(typeof___sTradeAnim_0_t, textColors) == 246);
    assert!(offset_of!(typeof___sTradeAnim_0_t, filler_F9) == 249);
    assert!(offset_of!(typeof___sTradeAnim_0_t, isCableTrade) == 250);
    assert!(offset_of!(typeof___sTradeAnim_0_t, wirelessWinLeft) == 251);
    assert!(offset_of!(typeof___sTradeAnim_0_t, wirelessWinTop) == 252);
    assert!(offset_of!(typeof___sTradeAnim_0_t, wirelessWinRight) == 253);
    assert!(offset_of!(typeof___sTradeAnim_0_t, wirelessWinBottom) == 254);
    assert!(size_of::<InGameTrade>() == 60);
    assert!(offset_of!(InGameTrade, nickname) == 0);
    assert!(offset_of!(InGameTrade, species) == 12);
    assert!(offset_of!(InGameTrade, ivs) == 14);
    assert!(offset_of!(InGameTrade, abilityNum) == 20);
    assert!(offset_of!(InGameTrade, otId) == 24);
    assert!(offset_of!(InGameTrade, conditions) == 28);
    assert!(offset_of!(InGameTrade, personality) == 36);
    assert!(offset_of!(InGameTrade, heldItem) == 40);
    assert!(offset_of!(InGameTrade, mailNum) == 42);
    assert!(offset_of!(InGameTrade, otName) == 43);
    assert!(offset_of!(InGameTrade, otGender) == 54);
    assert!(offset_of!(InGameTrade, sheen) == 55);
    assert!(offset_of!(InGameTrade, requestedSpecies) == 56);
    assert!(size_of::<typeof___sTradeMenu_0_t_queuedActions>() == 8);
    assert!(offset_of!(typeof___sTradeMenu_0_t_queuedActions, active) == 0);
    assert!(offset_of!(typeof___sTradeMenu_0_t_queuedActions, delay) == 2);
    assert!(offset_of!(typeof___sTradeMenu_0_t_queuedActions, actionId) == 4);
};

const ANIM_LINKMON_SMALL: u8 = 1;
const CB_CANCEL_TRADE_PROMPT: u8 = 4;
const CB_CONFIRM_TRADE_PROMPT: u8 = 3;
const CB_EXIT_CANCELED_TRADE: u8 = 12;
const CB_FADE_TO_START_TRADE: u8 = 9;
const CB_HANDLE_TRADE_CANCELED: u8 = 8;
const CB_IDLE: u8 = 100;
const CB_INIT_CONFIRM_TRADE_PROMPT: u8 = 14;
const CB_INIT_EXIT_CANCELED_TRADE: u8 = 11;
const CB_MAIN_MENU: u8 = 0;
const CB_PARTNER_MON_INVALID: u8 = 17;
const CB_PRINT_IS_THIS_OKAY: u8 = 7;
const CB_SELECTED_MON: u8 = 1;
const CB_SET_SELECTED_MONS: u8 = 6;
const CB_SHOW_MON_SUMMARY: u8 = 2;
const CB_START_LINK_TRADE: u8 = 13;
const CB_UNUSED_CLOSE_MSG: u8 = 15;
const CB_WAIT_TO_START_RFU_TRADE: u8 = 16;
const CB_WAIT_TO_START_TRADE: u8 = 10;
const CURSOR_ANIM_NORMAL: u8 = 0;
const CURSOR_ANIM_ON_CANCEL: u8 = 1;
const DRAW_SELECTED_FINISH: u8 = 5;
const GFXTAG_CANCEL_L: u16 = 6;
const GFXTAG_CANCEL_R: u16 = 7;
const GFXTAG_CHOOSE_PKMN_EMPTY_1: u8 = 11;
const GFXTAG_CHOOSE_PKMN_EMPTY_2: u8 = 12;
const GFXTAG_CHOOSE_PKMN_EMPTY_3: u8 = 13;
const GFXTAG_CHOOSE_PKMN_L: u16 = 8;
const GFXTAG_CHOOSE_PKMN_M: u8 = 9;
const GFXTAG_CHOOSE_PKMN_R: u8 = 10;
const GFXTAG_MENU_TEXT: u16 = 200;
const GFXTAG_PARTNER_NAME_L: u16 = 3;
const GFXTAG_PARTNER_NAME_M: u8 = 4;
const GFXTAG_PARTNER_NAME_R: u8 = 5;
const GFXTAG_PLAYER_NAME_L: u16 = 0;
const GFXTAG_PLAYER_NAME_M: u8 = 1;
const GFXTAG_PLAYER_NAME_R: u8 = 2;
const MSG_CANCELED: u8 = 1;
const MSG_EGG_CANT_BE_TRADED: u8 = 7;
const MSG_FRIENDS_MON_CANT_BE_TRADED: u8 = 8;
const MSG_FRIEND_WANTS_TO_TRADE: u8 = 5;
const MSG_MON_CANT_BE_TRADED: u8 = 6;
const MSG_ONLY_MON1: u8 = 2;
const MSG_ONLY_MON2: u8 = 3;
const MSG_STANDBY: u8 = 0;
const MSG_WAITING_FOR_FRIEND: u8 = 4;
const NUM_MENU_TEXT_SPRITES: u8 = 14;
const QUEUE_DELAY_DATA: u16 = 5;
const QUEUE_DELAY_MSG: u16 = 3;
const QUEUE_EGG_CANT_BE_TRADED: u8 = 7;
const QUEUE_FRIENDS_MON_CANT_BE_TRADED: u8 = 8;
const QUEUE_MON_CANT_BE_TRADED: u8 = 6;
const QUEUE_ONLY_MON1: u8 = 2;
const QUEUE_ONLY_MON2: u8 = 3;
const QUEUE_SEND_DATA: u8 = 0;
const QUEUE_STANDBY: u8 = 1;
const QUEUE_UNUSED1: u8 = 4;
const QUEUE_UNUSED2: u8 = 5;
const STATE_AFTER_NEW_MON_DELAY: u16 = 69;
const STATE_BYE_BYE: u16 = 11;
const STATE_CHECK_RIBBONS: u16 = 70;
const STATE_CREATE_LINK_MON_ARRIVING: u16 = 40;
const STATE_CREATE_LINK_MON_LEAVING: u16 = 27;
const STATE_CROSSING_BLEND_WHITE_1: u16 = 34;
const STATE_CROSSING_BLEND_WHITE_2: u16 = 35;
const STATE_CROSSING_BLEND_WHITE_3: u16 = 36;
const STATE_CROSSING_CREATE_MON_PICS: u16 = 37;
const STATE_CROSSING_LINK_MONS_ENTER: u16 = 33;
const STATE_CROSSING_LINK_MONS_EXIT: u16 = 39;
const STATE_CROSSING_MON_PICS_MOVE: u16 = 38;
const STATE_DELAY_FOR_MON_ANIM: u16 = 167;
const STATE_DESTROY_LINK_MON: u16 = 45;
const STATE_DESTROY_LINK_MON_WIRELESS: u16 = 145;
const STATE_END_LINK_TRADE: u16 = 71;
const STATE_FADE_IN_TO_CROSSING: u16 = 31;
const STATE_FADE_IN_TO_GBA_SEND: u16 = 21;
const STATE_FADE_IN_TO_NEW_MON: u16 = 61;
const STATE_FADE_OUT_END: u16 = 73;
const STATE_FADE_OUT_TO_CROSSING: u16 = 29;
const STATE_FADE_OUT_TO_GBA_RECV: u16 = 41;
const STATE_FADE_OUT_TO_GBA_SEND: u16 = 14;
const STATE_FADE_OUT_TO_NEW_MON: u16 = 52;
const STATE_FADE_POKEBALL_TO_NORMAL: u16 = 64;
const STATE_GBA_FLASH_RECV: u16 = 48;
const STATE_GBA_FLASH_SEND: u16 = 24;
const STATE_GBA_FLASH_SEND_WIRELESS: u16 = 124;
const STATE_GBA_STOP_FLASH_RECV: u16 = 50;
const STATE_GBA_STOP_FLASH_SEND: u16 = 25;
const STATE_GBA_STOP_FLASH_SEND_WIRELESS: u16 = 125;
const STATE_GBA_ZOOM_IN: u16 = 51;
const STATE_GBA_ZOOM_OUT: u16 = 23;
const STATE_LINK_MON_ARRIVED_DELAY: u16 = 46;
const STATE_LINK_MON_TRAVEL_IN: u16 = 43;
const STATE_LINK_MON_TRAVEL_OFFSCREEN: u16 = 200;
const STATE_LINK_MON_TRAVEL_OUT: u16 = 28;
const STATE_MON_SLIDE_IN: u16 = 1;
const STATE_MOVE_GBA_TO_CENTER: u16 = 47;
const STATE_NEW_MON_MSG: u16 = 67;
const STATE_PAN_AWAY_GBA: u16 = 26;
const STATE_PAN_TO_GBA: u16 = 44;
const STATE_PAN_TO_GBA_WIRELESS: u16 = 144;
const STATE_POKEBALL_ARRIVE: u16 = 63;
const STATE_POKEBALL_ARRIVE_WAIT: u16 = 65;
const STATE_POKEBALL_DEPART: u16 = 12;
const STATE_POKEBALL_DEPART_WAIT: u16 = 13;
const STATE_SEND_MSG: u16 = 10;
const STATE_SHOW_NEW_MON: u16 = 66;
const STATE_START: u16 = 0;
const STATE_TAKE_CARE_OF_MON: u16 = 68;
const STATE_TRY_EVOLUTION: u16 = 72;
const STATE_WAIT_FADE_IN_TO_CROSSING: u16 = 32;
const STATE_WAIT_FADE_IN_TO_GBA_SEND: u16 = 22;
const STATE_WAIT_FADE_IN_TO_NEW_MON: u16 = 62;
const STATE_WAIT_FADE_OUT_END: u16 = 74;
const STATE_WAIT_FADE_OUT_TO_CROSSING: u16 = 30;
const STATE_WAIT_FADE_OUT_TO_GBA_RECV: u16 = 42;
const STATE_WAIT_FADE_OUT_TO_GBA_SEND: u16 = 20;
const STATE_WAIT_FADE_OUT_TO_NEW_MON: u16 = 60;
const STATE_WAIT_FOR_MON_CRY: u16 = 267;
const STATE_WAIT_WIRELESS_SIGNAL_RECV: u16 = 146;
const STATE_WAIT_WIRELESS_SIGNAL_SEND: u16 = 126;
const STATUS_CANCEL: u8 = 2;
const STATUS_NONE: u8 = 0;
const STATUS_READY: u8 = 1;
const TEXT_CANCEL: i32 = 0;
const TEXT_CANCEL_TRADE: i32 = 4;
const TEXT_CHOOSE_MON: i32 = 1;

static gTradePlatform_Tilemap: Table<CArray<u16, 2048>> =
    Table((&raw const crate::data::trade::gTradePlatform_Tilemap).cast());
static sActionTexts: Table<CArray<*mut u8, 6>> =
    Table((&raw const crate::data::trade::sActionTexts).cast());
static sAffineAnims_CrossingMonPics: Table<CArray<*mut AffineAnimCmd, 1>> =
    Table((&raw const crate::data::trade::sAffineAnims_CrossingMonPics).cast());
static sBgTemplates: Table<CArray<BgTemplate, 4>> =
    Table((&raw const crate::data::trade::sBgTemplates).cast());
static sCableCloseup_Map: Table<CArray<u16, 1024>> =
    Table((&raw const crate::data::trade::sCableCloseup_Map).cast());
static sCursorMoveDestinations: Table<CArray<CArray<CArray<u8, 6>, 4>, 13>> =
    Table((&raw const crate::data::trade::sCursorMoveDestinations).cast());
static sCursor_SpritePalette: Table<SpritePalette> =
    Table((&raw const crate::data::trade::sCursor_SpritePalette).cast());
static sCursor_SpriteSheet: Table<SpriteSheet> =
    Table((&raw const crate::data::trade::sCursor_SpriteSheet).cast());
static sGbaAffineMapCable: Table<CArray<u8, 256>> =
    Table((&raw const crate::data::trade::sGbaAffineMapCable).cast());
static sGbaAffineMapWireless: Table<CArray<u8, 256>> =
    Table((&raw const crate::data::trade::sGbaAffineMapWireless).cast());
static sGbaAffine_Gfx: Table<CArray<u8, 10240>> =
    Table((&raw const crate::data::trade::sGbaAffine_Gfx).cast());
static sGbaMapCable: Table<CArray<u16, 2048>> =
    Table((&raw const crate::data::trade::sGbaMapCable).cast());
static sGbaMapWireless: Table<CArray<u16, 2048>> =
    Table((&raw const crate::data::trade::sGbaMapWireless).cast());
static sIngameTradeMail: Table<CArray<CArray<u16, 10>, 3>> =
    Table((&raw const crate::data::trade::sIngameTradeMail).cast());
static sIngameTrades: Table<CArray<InGameTrade, 4>> =
    Table((&raw const crate::data::trade::sIngameTrades).cast());
static sLinkMonShadow_Pal: Table<CArray<u16, 12>> =
    Table((&raw const crate::data::trade::sLinkMonShadow_Pal).cast());
static sMessages: Table<CArray<*mut u8, 9>> =
    Table((&raw const crate::data::trade::sMessages).cast());
static sPokeBallSpritePalette: Table<SpritePalette> =
    Table((&raw const crate::data::trade::sPokeBallSpritePalette).cast());
static sPokeBallSpriteSheet: Table<SpriteSheet> =
    Table((&raw const crate::data::trade::sPokeBallSpriteSheet).cast());
static sSelectTradeMonActions: Table<CArray<MenuAction, 2>> =
    Table((&raw const crate::data::trade::sSelectTradeMonActions).cast());
static sSelectedMonLevelGenderCoords: Table<CArray<CArray<u8, 2>, 3>> =
    Table((&raw const crate::data::trade::sSelectedMonLevelGenderCoords).cast());
static sSpritePalette_Gba: Table<SpritePalette> =
    Table((&raw const crate::data::trade::sSpritePalette_Gba).cast());
static sSpritePalette_LinkMon: Table<SpritePalette> =
    Table((&raw const crate::data::trade::sSpritePalette_LinkMon).cast());
static sSpritePalette_MenuText: Table<SpritePalette> =
    Table((&raw const crate::data::trade::sSpritePalette_MenuText).cast());
static sSpriteSheet_CableEnd: Table<SpriteSheet> =
    Table((&raw const crate::data::trade::sSpriteSheet_CableEnd).cast());
static sSpriteSheet_GbaScreen: Table<SpriteSheet> =
    Table((&raw const crate::data::trade::sSpriteSheet_GbaScreen).cast());
static sSpriteSheet_LinkMonGlow: Table<SpriteSheet> =
    Table((&raw const crate::data::trade::sSpriteSheet_LinkMonGlow).cast());
static sSpriteSheet_LinkMonShadow: Table<SpriteSheet> =
    Table((&raw const crate::data::trade::sSpriteSheet_LinkMonShadow).cast());
static sSpriteTemplate_CableEnd: Table<SpriteTemplate> =
    Table((&raw const crate::data::trade::sSpriteTemplate_CableEnd).cast());
static sSpriteTemplate_Cursor: Table<SpriteTemplate> =
    Table((&raw const crate::data::trade::sSpriteTemplate_Cursor).cast());
static sSpriteTemplate_GbaScreenFlash_Long: Table<SpriteTemplate> =
    Table((&raw const crate::data::trade::sSpriteTemplate_GbaScreenFlash_Long).cast());
static sSpriteTemplate_GbaScreenFlash_Short: Table<SpriteTemplate> =
    Table((&raw const crate::data::trade::sSpriteTemplate_GbaScreenFlash_Short).cast());
static sSpriteTemplate_LinkMonGlow: Table<SpriteTemplate> =
    Table((&raw const crate::data::trade::sSpriteTemplate_LinkMonGlow).cast());
static sSpriteTemplate_LinkMonShadow: Table<SpriteTemplate> =
    Table((&raw const crate::data::trade::sSpriteTemplate_LinkMonShadow).cast());
static sSpriteTemplate_MenuText: Table<SpriteTemplate> =
    Table((&raw const crate::data::trade::sSpriteTemplate_MenuText).cast());
static sSpriteTemplate_Pokeball: Table<SpriteTemplate> =
    Table((&raw const crate::data::trade::sSpriteTemplate_Pokeball).cast());
static sText_EmptyString: Table<CArray<u8, 1>> =
    Table((&raw const crate::data::trade::sText_EmptyString).cast());
static sText_FourQuestionMarks: Table<CArray<u8, 5>> =
    Table((&raw const crate::data::trade::sText_FourQuestionMarks).cast());
static sText_IsThisTradeOkay: Table<CArray<u8, 20>> =
    Table((&raw const crate::data::trade::sText_IsThisTradeOkay).cast());
static sText_NewLine: Table<CArray<u8, 2>> =
    Table((&raw const crate::data::trade::sText_NewLine).cast());
static sText_ShedinjaJP: Table<CArray<u8, 5>> =
    Table((&raw const crate::data::trade::sText_ShedinjaJP).cast());
static sTradeBallVerticalVelocityTable: Table<CArray<i8, 108>> =
    Table((&raw const crate::data::trade::sTradeBallVerticalVelocityTable).cast());
static sTradeMonBoxCoords: Table<CArray<CArray<u8, 2>, 12>> =
    Table((&raw const crate::data::trade::sTradeMonBoxCoords).cast());
static sTradeMonLevelCoords: Table<CArray<CArray<u8, 2>, 12>> =
    Table((&raw const crate::data::trade::sTradeMonLevelCoords).cast());
static sTradeMonSpriteCoords: Table<CArray<CArray<u8, 2>, 13>> =
    Table((&raw const crate::data::trade::sTradeMonSpriteCoords).cast());
static sTradeMovesBoxTilemap: Table<CArray<u16, 255>> =
    Table((&raw const crate::data::trade::sTradeMovesBoxTilemap).cast());
static sTradePartyBoxTilemap: Table<CArray<u16, 255>> =
    Table((&raw const crate::data::trade::sTradePartyBoxTilemap).cast());
static sTradeSequenceBgTemplates: Table<CArray<BgTemplate, 4>> =
    Table((&raw const crate::data::trade::sTradeSequenceBgTemplates).cast());
static sTradeSequenceWindowTemplates: Table<CArray<WindowTemplate, 2>> =
    Table((&raw const crate::data::trade::sTradeSequenceWindowTemplates).cast());
static sTradeStripesBG2Tilemap: Table<CArray<u8, 2048>> =
    Table((&raw const crate::data::trade::sTradeStripesBG2Tilemap).cast());
static sTradeStripesBG3Tilemap: Table<CArray<u8, 2048>> =
    Table((&raw const crate::data::trade::sTradeStripesBG3Tilemap).cast());
static sTradeTextColors: Table<CArray<u8, 3>> =
    Table((&raw const crate::data::trade::sTradeTextColors).cast());
static sTradeYesNoWindowTemplate: Table<WindowTemplate> =
    Table((&raw const crate::data::trade::sTradeYesNoWindowTemplate).cast());
static sWindowTemplates: Table<CArray<WindowTemplate, 19>> =
    Table((&raw const crate::data::trade::sWindowTemplates).cast());
static sWirelessCloseup_Map: Table<CArray<u32, 64>> =
    Table((&raw const crate::data::trade::sWirelessCloseup_Map).cast());
static sWirelessSignalAnimParams: Table<CArray<CArray<u8, 2>, 34>> =
    Table((&raw const crate::data::trade::sWirelessSignalAnimParams).cast());
static sWirelessSignalNone_Pal: Table<CArray<u16, 16>> =
    Table((&raw const crate::data::trade::sWirelessSignalNone_Pal).cast());
static sWirelessSignalRecv_Pal: Table<CArray<u16, 256>> =
    Table((&raw const crate::data::trade::sWirelessSignalRecv_Pal).cast());
static sWirelessSignalSend_Pal: Table<CArray<u16, 256>> =
    Table((&raw const crate::data::trade::sWirelessSignalSend_Pal).cast());
static sWirelessSignal_Gfx: Table<CArray<u32, 420>> =
    Table((&raw const crate::data::trade::sWirelessSignal_Gfx).cast());
static sWirelessSignal_Tilemap: Table<CArray<u32, 445>> =
    Table((&raw const crate::data::trade::sWirelessSignal_Tilemap).cast());

#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub(crate) static sMenuTextTileBuffer: crate::global::Global<*mut u8> =
    crate::global::Global::new(null_mut());
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub(crate) static sMenuTextTileBuffers: crate::global::Global<CArray<*mut u8, 14>> =
    crate::global::Global::new(unsafe { zeroed() });
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub static gTradeMail: crate::global::Global<CArray<Mail, 6>> =
    crate::global::Global::new(unsafe { zeroed() });
#[unsafe(no_mangle)]
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub static gSelectedTradeMonPositions: crate::global::Global<Aligned<CArray<u8, 2>>> =
    crate::global::Global::new(Aligned(unsafe { zeroed() }));
pub(crate) static sTradeMenu: crate::global::Global<*mut typeof___sTradeMenu_0_t> =
    crate::global::Global::new(null_mut());
pub(crate) static sTradeAnim: crate::global::Global<*mut typeof___sTradeAnim_0_t> =
    crate::global::Global::new(null_mut());

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
/// `GetBgTilemapBuffer` with this module's view of its types.
#[inline]
unsafe fn GetBgTilemapBuffer(a0: u8) -> *mut c_void {
    unsafe { crate::bg::GetBgTilemapBuffer(a0) as *mut c_void }
}
/// `LZ77UnCompVram` with this module's view of its types.
#[inline]
fn LZ77UnCompVram(a0: *mut u32, a1: *mut c_void) {
    unsafe {
        crate::syscall::LZ77UnCompVram(a0 as _, a1 as _);
    }
}
/// `SetMainCallback2` with this module's view of its types.
#[inline]
unsafe fn SetMainCallback2(a0: Option<unsafe fn()>) {
    unsafe {
        crate::agb_main::SetMainCallback2(core::mem::transmute(a0));
    }
}

unsafe fn SendLinkData(linkData: *mut c_void, size: u32) -> u8 {
    if gPlayerCurrActivity.get() == ACTIVITY_29 {
        rfu_NI_setSendData(crate::gl!(lman.acceptSlot_flag), 84, linkData, size);
        return TRUE;
    } else {
        return SendBlock(0, linkData, size as u16);
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
unsafe fn RequestLinkData(r#type: u8) {
    SendBlockRequest(r#type);
}
unsafe fn IsLinkTradeTaskFinished() -> u32 {
    if gPlayerCurrActivity.get() == ACTIVITY_29 {
        if (*(*gRfuSlotStatusNI.as_ptr())
            [Rfu_GetIndexOfNewestChild(crate::gl!(lman.acceptSlot_flag))])
        .send
        .state
            == 0
        {
            return TRUE as u32;
        } else {
            return FALSE as u32;
        }
    } else {
        return IsLinkTaskFinished() as u32;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
unsafe fn _GetBlockReceivedStatus() -> u32 {
    GetBlockReceivedStatus() as u32
}
unsafe fn TradeResetReceivedFlags() {
    if IsWirelessTrade() != 0 {
        rfu_clearSlot(12, crate::gl!(lman.acceptSlot_flag));
    } else {
        ResetBlockReceivedFlags();
    }
}
unsafe fn TradeResetReceivedFlag(who: u32) {
    if IsWirelessTrade() != 0 {
        rfu_clearSlot(12, crate::gl!(lman.acceptSlot_flag));
    } else {
        ResetBlockReceivedFlag(who as u8);
    }
}
fn IsWirelessTrade() -> u32 {
    if gWirelessCommType.get() != 0 && gPlayerCurrActivity.get() == ACTIVITY_29 {
        return TRUE as u32;
    } else {
        return FALSE as u32;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
unsafe fn SetTradeLinkStandbyCallback(unused: u8) {
    SetLinkStandbyCallback();
}
unsafe fn _IsLinkTaskFinished() -> u32 {
    IsLinkTaskFinished() as u32
}
unsafe fn InitTradeMenu() {
    ResetSpriteData();
    FreeAllSpritePalettes();
    ResetTasks();
    ResetPaletteFade();
    (*gPaletteFade.as_ptr()).set_bufferTransferDisabled(TRUE as u16);
    SetVBlankCallback(Some(VBlankCB_TradeMenu));
    LoadPalette(
        (*(&raw const crate::data::menu::gStandardMenuPalette).cast::<CArray<u16, 0>>())
            .as_ptr()
            .cast_mut() as *mut c_void,
        240,
        20,
    );
    LoadPalette(
        (*(&raw const crate::data::menu::gStandardMenuPalette).cast::<CArray<u16, 0>>())
            .as_ptr()
            .cast_mut() as *mut c_void,
        208,
        20,
    );
    ResetBgsAndClearDma3BusyFlags(0);
    InitBgsFromTemplates(0, sBgTemplates.as_ptr().cast_mut(), 4);
    SetBgTilemapBuffer(
        1,
        (*crate::gl!(sTradeMenu)).tilemapBuffer.as_mut_ptr() as *mut c_void,
    );
    if InitWindows(sWindowTemplates.as_ptr().cast_mut()) != 0 {
        DeactivateAllTextPrinters();
        for i in 0..18u32 {
            ClearWindowTilemap(i as u8);
            FillWindowPixelBuffer(i as u8, 0);
        }
        FillBgTilemapBufferRect(0, 0, 0, 0, DISPLAY_TILE_WIDTH, DISPLAY_TILE_HEIGHT, 15);
        LoadUserWindowBorderGfx_(0, 20, 192);
        LoadUserWindowBorderGfx(2, 1, 224);
        LoadMonIconPalettes();
        (*crate::gl!(sTradeMenu)).bufferPartyState = 0;
        (*crate::gl!(sTradeMenu)).callbackId = CB_MAIN_MENU;
        (*crate::gl!(sTradeMenu)).neverRead_70 = 0;
        (*crate::gl!(sTradeMenu)).drawSelectedMonState[0] = 0;
        (*crate::gl!(sTradeMenu)).drawSelectedMonState[1] = 0;
        (*crate::gl!(sTradeMenu)).playerConfirmStatus = STATUS_NONE;
        (*crate::gl!(sTradeMenu)).partnerConfirmStatus = STATUS_NONE;
        (*crate::gl!(sTradeMenu)).timer = 0;
    }
}
pub unsafe fn CB2_StartCreateTradeMenu() {
    SetMainCallback2(Some(CB2_CreateTradeMenu));
    crate::gl!(gMain.callback1 = None);
    gEnemyPartyCount.set(0);
}
pub(crate) unsafe fn CB2_CreateTradeMenu() {
    let mut i: i32 = 0;
    let mut temp: SpriteTemplate = zeroed();
    let mut id: u8 = 0;
    let mut xPos: u32 = 0;
    'l1: {
        let sw1: u8 = crate::gl!(gMain.state);
        let mut fall = false;
        if sw1 == 0 {
            crate::gl!(sTradeMenu = AllocZeroed(4336) as *mut typeof___sTradeMenu_0_t);
            InitTradeMenu();
            crate::gl!(sMenuTextTileBuffer = AllocZeroed(3584) as *mut u8);
            for i in 0..(NUM_MENU_TEXT_SPRITES as i32) {
                crate::gl!(sMenuTextTileBuffers[i] = (*sMenuTextTileBuffer.as_ptr()).at(i * 256));
            }
            crate::gl!(gMain.state += 1);
            break 'l1;
        }
        if sw1 == 1 {
            (*gPaletteFade.as_ptr()).set_bufferTransferDisabled(FALSE as u16);
            for i in 0..PARTY_SIZE {
                CreateMon(
                    crate::gl!(&raw gEnemyParty[i]),
                    0,
                    0,
                    USE_RANDOM_IVS,
                    0,
                    0,
                    0,
                    0,
                );
            }
            PrintTradeMessage(MSG_STANDBY);
            ShowBg(0);
            if gReceivedRemoteLinkPlayers.get() == 0 {
                gLinkType.set(LINKTYPE_TRADE_CONNECTING);
                (*crate::gl!(sTradeMenu)).timer = 0;
                if gWirelessCommType.get() != 0 {
                    SetWirelessCommType1();
                    OpenLink();
                    CreateTask_RfuIdle();
                } else {
                    OpenLink();
                    crate::gl!(gMain.state += 1);
                    CreateTask(Some(Task_WaitForLinkPlayerConnection), 1);
                }
            } else {
                crate::gl!(gMain.state = 4);
            }
            break 'l1;
        }
        if sw1 == 2 {
            (*crate::gl!(sTradeMenu)).timer += 1;
            if (*crate::gl!(sTradeMenu)).timer > 11 {
                (*crate::gl!(sTradeMenu)).timer = 0;
                crate::gl!(gMain.state += 1);
            }
            break 'l1;
        }
        if sw1 == 3 {
            if GetLinkPlayerCount_2() >= GetSavedPlayerCount() {
                if IsLinkMaster() != 0 {
                    if ({
                        (*crate::gl!(sTradeMenu)).timer += 1;
                        (*crate::gl!(sTradeMenu)).timer
                    }) > 30
                    {
                        CheckShouldAdvanceLinkState();
                        crate::gl!(gMain.state += 1);
                    }
                } else {
                    crate::gl!(gMain.state += 1);
                }
            }
            break 'l1;
        }
        if sw1 == 4 {
            if gReceivedRemoteLinkPlayers.get() == TRUE
                && IsLinkPlayerDataExchangeComplete() == TRUE
            {
                DestroyTask_RfuIdle();
                CalculatePlayerPartyCount();
                crate::gl!(gMain.state += 1);
                (*crate::gl!(sTradeMenu)).timer = 0;
                if gWirelessCommType.get() != 0 {
                    Rfu_SetLinkRecovery(TRUE as u32);
                    SetLinkStandbyCallback();
                }
            }
            break 'l1;
        }
        if sw1 == 5 {
            if gWirelessCommType.get() != 0 {
                if IsLinkRfuTaskFinished() != 0 {
                    crate::gl!(gMain.state += 1);
                    LoadWirelessStatusIndicatorSpriteGfx();
                    CreateWirelessStatusIndicatorSprite(0, 0);
                }
            } else {
                crate::gl!(gMain.state += 1);
            }
            break 'l1;
        }
        if sw1 == 6 {
            if BufferTradeParties() != 0 {
                SaveTradeGiftRibbons();
                crate::gl!(gMain.state += 1);
            }
            break 'l1;
        }
        if sw1 == 7 {
            CalculateEnemyPartyCount();
            SetGpuReg(0x0, 0);
            SetGpuReg(REG_OFFSET_BLDCNT, 0);
            (*crate::gl!(sTradeMenu)).partyCounts[0] = gPlayerPartyCount.get();
            (*crate::gl!(sTradeMenu)).partyCounts[1] = gEnemyPartyCount.get();
            i = 0;
            while i < (*crate::gl!(sTradeMenu)).partyCounts[0] as i32 {
                let mon: *mut Pokemon = crate::gl!(&raw gPlayerParty[i]);
                (*crate::gl!(sTradeMenu)).partySpriteIds[0][i] = CreateMonIcon(
                    GetMonData2(mon, MON_DATA_SPECIES_OR_EGG) as u16,
                    Some(SpriteCB_MonIcon),
                    sTradeMonSpriteCoords[i][0] as i16 * 8 + 14,
                    sTradeMonSpriteCoords[i][1] as i16 * 8 - 12,
                    1,
                    GetMonData2(mon, MON_DATA_PERSONALITY),
                    TRUE as u32,
                );
                i += 1;
            }
            i = 0;
            while i < (*crate::gl!(sTradeMenu)).partyCounts[1] as i32 {
                let mon: *mut Pokemon = crate::gl!(&raw gEnemyParty[i]);
                (*crate::gl!(sTradeMenu)).partySpriteIds[1][i] = CreateMonIcon(
                    GetMonData3(mon, MON_DATA_SPECIES_OR_EGG, null_mut()) as u16,
                    Some(SpriteCB_MonIcon),
                    sTradeMonSpriteCoords[i + PARTY_SIZE][0] as i16 * 8 + 14,
                    sTradeMonSpriteCoords[i + PARTY_SIZE][1] as i16 * 8 - 12,
                    1,
                    GetMonData2(mon, MON_DATA_PERSONALITY),
                    FALSE as u32,
                );
                i += 1;
            }
            crate::gl!(gMain.state += 1);
            break 'l1;
        }
        if sw1 == 8 {
            LoadHeldItemIcons();
            DrawHeldItemIconsForTrade(
                &raw mut (*crate::gl!(sTradeMenu)).partyCounts[0],
                (*crate::gl!(sTradeMenu)).partySpriteIds[0].as_mut_ptr(),
                0,
            );
            crate::gl!(gMain.state += 1);
            break 'l1;
        }
        if sw1 == 9 {
            DrawHeldItemIconsForTrade(
                &raw mut (*crate::gl!(sTradeMenu)).partyCounts[0],
                (*crate::gl!(sTradeMenu)).partySpriteIds[0].as_mut_ptr(),
                TRADE_PARTNER,
            );
            crate::gl!(gMain.state += 1);
            break 'l1;
        }
        if sw1 == 10 {
            DrawTextWindowAndBufferTiles(
                (*crate::gl!(gSaveBlock2Ptr)).playerName.as_mut_ptr(),
                crate::gl!(sMenuTextTileBuffers[0]) as *mut c_void,
                0,
                0,
                3,
            );
            id = GetMultiplayerId();
            DrawTextWindowAndBufferTiles(
                (*gLinkPlayers.as_ptr())[id as i32 ^ 1].name.as_mut_ptr(),
                crate::gl!(sMenuTextTileBuffers[3]) as *mut c_void,
                0,
                0,
                3,
            );
            DrawTextWindowAndBufferTiles(
                sActionTexts[0],
                crate::gl!(sMenuTextTileBuffers[6]) as *mut c_void,
                0,
                0,
                2,
            );
            DrawBottomRowText(sActionTexts[1], crate::gl!(sMenuTextTileBuffers[8]), 24);
            crate::gl!(gMain.state += 1);
            (*crate::gl!(sTradeMenu)).timer = 0;
            break 'l1;
        }
        if sw1 == 11 {
            if LoadUISpriteGfx() != 0 {
                crate::gl!(gMain.state += 1);
            }
            break 'l1;
        }
        if sw1 == 12 {
            xPos = GetStringCenterAlignXOffset(
                FONT_NORMAL as i32,
                (*crate::gl!(gSaveBlock2Ptr)).playerName.as_mut_ptr(),
                120,
            ) as u32;
            for i in 0..3i32 {
                temp = *sSpriteTemplate_MenuText;
                temp.tileTag += i as u16 + GFXTAG_PLAYER_NAME_L;
                CreateSprite(&raw mut temp, xPos as i16 + i as i16 * 32 + 16, 10, 1);
            }
            xPos = GetStringCenterAlignXOffset(
                FONT_NORMAL as i32,
                (*gLinkPlayers.as_ptr())[GetMultiplayerId() as i32 ^ 1]
                    .name
                    .as_mut_ptr(),
                120,
            ) as u32;
            for i in 0..3i32 {
                temp = *sSpriteTemplate_MenuText;
                temp.tileTag += i as u16 + GFXTAG_PARTNER_NAME_L;
                CreateSprite(&raw mut temp, xPos as i16 + i as i16 * 32 + 136, 10, 1);
            }
            crate::gl!(gMain.state += 1);
            break 'l1;
        }
        if sw1 == 13 {
            temp = *sSpriteTemplate_MenuText;
            temp.tileTag += GFXTAG_CANCEL_L;
            CreateSprite(&raw mut temp, 215, 152, 1);
            temp = *sSpriteTemplate_MenuText;
            temp.tileTag += GFXTAG_CANCEL_R;
            CreateSprite(&raw mut temp, 247, 152, 1);
            for i in 0..6i32 {
                temp = *sSpriteTemplate_MenuText;
                temp.tileTag += i as u16 + GFXTAG_CHOOSE_PKMN_L;
                CreateSprite(&raw mut temp, i as i16 * 32 + 24, 150, 1);
            }
            (*crate::gl!(sTradeMenu)).cursorSpriteId = CreateSprite(
                (&raw const *sSpriteTemplate_Cursor).cast_mut(),
                sTradeMonSpriteCoords[0][0] as i16 * 8 + 32,
                sTradeMonSpriteCoords[0][1] as i16 * 8,
                2,
            );
            (*crate::gl!(sTradeMenu)).cursorPosition = 0;
            crate::gl!(gMain.state += 1);
            rbox_fill_rectangle(0);
            break 'l1;
        }
        if sw1 == 14 {
            ComputePartyTradeableFlags(TRADE_PLAYER);
            PrintPartyNicknames(TRADE_PLAYER);
            (*crate::gl!(sTradeMenu)).bg2hofs = 0;
            (*crate::gl!(sTradeMenu)).bg3hofs = 0;
            SetActiveMenuOptions();
            crate::gl!(gMain.state += 1);
            PlayBGM(MUS_SCHOOL);
            break 'l1;
        }
        if sw1 == 15 {
            fall = true;
            ComputePartyTradeableFlags(TRADE_PARTNER);
            PrintPartyNicknames(TRADE_PARTNER);
            crate::gl!(gMain.state += 1);
        }
        if fall || sw1 == 16 {
            LoadTradeBgGfx(0);
            crate::gl!(gMain.state += 1);
            break 'l1;
        }
        if sw1 == 17 {
            LoadTradeBgGfx(1);
            crate::gl!(gMain.state += 1);
            break 'l1;
        }
        if sw1 == 18 {
            BeginNormalPaletteFade(PALETTES_ALL, 0, 16, 0, 0);
            crate::gl!(gMain.state += 1);
            break 'l1;
        }
        if sw1 == 19 {
            SetGpuReg(REG_OFFSET_DISPCNT, 4160);
            LoadTradeBgGfx(2);
            crate::gl!(gMain.state += 1);
            break 'l1;
        }
        if sw1 == 20 {
            ComputePartyHPBarLevels(TRADE_PLAYER);
            crate::gl!(gMain.state += 1);
            break 'l1;
        }
        if sw1 == 21 {
            ComputePartyHPBarLevels(TRADE_PARTNER);
            SetTradePartyHPBarSprites();
            crate::gl!(gMain.state += 1);
            break 'l1;
        }
        if sw1 == 22 {
            if (*gPaletteFade.as_ptr()).active() == 0 {
                crate::gl!(gMain.callback1 = Some(CB1_UpdateLink));
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
pub(crate) unsafe fn CB2_ReturnToTradeMenu() {
    let mut i: i32 = 0;
    let mut temp: SpriteTemplate = zeroed();
    let mut id: u8 = 0;
    let mut xPos: u32 = 0;
    match crate::gl!(gMain.state) {
        0 => {
            InitTradeMenu();
            crate::gl!(gMain.state += 1);
        }
        1 => {
            crate::gl!(gMain.state += 1);
            (*crate::gl!(sTradeMenu)).timer = 0;
        }
        2 => {
            crate::gl!(gMain.state += 1);
        }
        3 => {
            crate::gl!(gMain.state += 1);
        }
        4 => {
            CalculatePlayerPartyCount();
            crate::gl!(gMain.state += 1);
        }
        5 => {
            if gWirelessCommType.get() != 0 {
                LoadWirelessStatusIndicatorSpriteGfx();
                CreateWirelessStatusIndicatorSprite(0, 0);
            }
            crate::gl!(gMain.state += 1);
        }
        6 => {
            crate::gl!(gMain.state += 1);
        }
        7 => {
            CalculateEnemyPartyCount();
            (*crate::gl!(sTradeMenu)).partyCounts[0] = gPlayerPartyCount.get();
            (*crate::gl!(sTradeMenu)).partyCounts[1] = gEnemyPartyCount.get();
            ClearWindowTilemap(0);
            PrintPartyNicknames(TRADE_PLAYER);
            PrintPartyNicknames(TRADE_PARTNER);
            i = 0;
            while i < (*crate::gl!(sTradeMenu)).partyCounts[0] as i32 {
                let mon: *mut Pokemon = crate::gl!(&raw gPlayerParty[i]);
                (*crate::gl!(sTradeMenu)).partySpriteIds[0][i] = CreateMonIcon(
                    GetMonData3(mon, MON_DATA_SPECIES_OR_EGG, null_mut()) as u16,
                    Some(SpriteCB_MonIcon),
                    sTradeMonSpriteCoords[i][0] as i16 * 8 + 14,
                    sTradeMonSpriteCoords[i][1] as i16 * 8 - 12,
                    1,
                    GetMonData2(mon, MON_DATA_PERSONALITY),
                    TRUE as u32,
                );
                i += 1;
            }
            i = 0;
            while i < (*crate::gl!(sTradeMenu)).partyCounts[1] as i32 {
                let mon: *mut Pokemon = crate::gl!(&raw gEnemyParty[i]);
                (*crate::gl!(sTradeMenu)).partySpriteIds[1][i] = CreateMonIcon(
                    GetMonData3(mon, MON_DATA_SPECIES_OR_EGG, null_mut()) as u16,
                    Some(SpriteCB_MonIcon),
                    sTradeMonSpriteCoords[i + PARTY_SIZE][0] as i16 * 8 + 14,
                    sTradeMonSpriteCoords[i + PARTY_SIZE][1] as i16 * 8 - 12,
                    1,
                    GetMonData2(mon, MON_DATA_PERSONALITY),
                    FALSE as u32,
                );
                i += 1;
            }
            crate::gl!(gMain.state += 1);
        }
        8 => {
            LoadHeldItemIcons();
            DrawHeldItemIconsForTrade(
                &raw mut (*crate::gl!(sTradeMenu)).partyCounts[0],
                (*crate::gl!(sTradeMenu)).partySpriteIds[0].as_mut_ptr(),
                0,
            );
            crate::gl!(gMain.state += 1);
        }
        9 => {
            DrawHeldItemIconsForTrade(
                &raw mut (*crate::gl!(sTradeMenu)).partyCounts[0],
                (*crate::gl!(sTradeMenu)).partySpriteIds[0].as_mut_ptr(),
                TRADE_PARTNER,
            );
            crate::gl!(gMain.state += 1);
        }
        10 => {
            DrawTextWindowAndBufferTiles(
                (*crate::gl!(gSaveBlock2Ptr)).playerName.as_mut_ptr(),
                crate::gl!(sMenuTextTileBuffers[0]) as *mut c_void,
                0,
                0,
                3,
            );
            id = GetMultiplayerId();
            DrawTextWindowAndBufferTiles(
                (*gLinkPlayers.as_ptr())[id as i32 ^ 1].name.as_mut_ptr(),
                crate::gl!(sMenuTextTileBuffers[3]) as *mut c_void,
                0,
                0,
                3,
            );
            DrawTextWindowAndBufferTiles(
                sActionTexts[0],
                crate::gl!(sMenuTextTileBuffers[6]) as *mut c_void,
                0,
                0,
                2,
            );
            DrawBottomRowText(sActionTexts[1], crate::gl!(sMenuTextTileBuffers[8]), 24);
            crate::gl!(gMain.state += 1);
            (*crate::gl!(sTradeMenu)).timer = 0;
        }
        11 => {
            if LoadUISpriteGfx() != 0 {
                crate::gl!(gMain.state += 1);
            }
        }
        12 => {
            xPos = GetStringCenterAlignXOffset(
                FONT_NORMAL as i32,
                (*crate::gl!(gSaveBlock2Ptr)).playerName.as_mut_ptr(),
                120,
            ) as u32;
            for i in 0..3i32 {
                temp = *sSpriteTemplate_MenuText;
                temp.tileTag += i as u16 + GFXTAG_PLAYER_NAME_L;
                CreateSprite(&raw mut temp, xPos as i16 + i as i16 * 32 + 16, 10, 1);
            }
            xPos = GetStringCenterAlignXOffset(
                FONT_NORMAL as i32,
                (*gLinkPlayers.as_ptr())[GetMultiplayerId() as i32 ^ 1]
                    .name
                    .as_mut_ptr(),
                120,
            ) as u32;
            for i in 0..3i32 {
                temp = *sSpriteTemplate_MenuText;
                temp.tileTag += i as u16 + GFXTAG_PARTNER_NAME_L;
                CreateSprite(&raw mut temp, xPos as i16 + i as i16 * 32 + 136, 10, 1);
            }
            crate::gl!(gMain.state += 1);
        }
        13 => {
            temp = *sSpriteTemplate_MenuText;
            temp.tileTag += GFXTAG_CANCEL_L;
            CreateSprite(&raw mut temp, 215, 152, 1);
            temp = *sSpriteTemplate_MenuText;
            temp.tileTag += GFXTAG_CANCEL_R;
            CreateSprite(&raw mut temp, 247, 152, 1);
            for i in 0..6i32 {
                temp = *sSpriteTemplate_MenuText;
                temp.tileTag += i as u16 + GFXTAG_CHOOSE_PKMN_L;
                CreateSprite(&raw mut temp, i as i16 * 32 + 24, 150, 1);
            }
            if (*crate::gl!(sTradeMenu)).cursorPosition < PARTY_SIZE as u8 {
                (*crate::gl!(sTradeMenu)).cursorPosition = gLastViewedMonIndex.get();
            } else {
                (*crate::gl!(sTradeMenu)).cursorPosition =
                    gLastViewedMonIndex.get() + PARTY_SIZE as u8;
            }
            (*crate::gl!(sTradeMenu)).cursorSpriteId = CreateSprite(
                (&raw const *sSpriteTemplate_Cursor).cast_mut(),
                sTradeMonSpriteCoords[(*crate::gl!(sTradeMenu)).cursorPosition][0] as i16 * 8 + 32,
                sTradeMonSpriteCoords[(*crate::gl!(sTradeMenu)).cursorPosition][1] as i16 * 8,
                2,
            );
            crate::gl!(gMain.state = 16);
        }
        16 => {
            LoadTradeBgGfx(0);
            crate::gl!(gMain.state += 1);
        }
        17 => {
            LoadTradeBgGfx(1);
            (*crate::gl!(sTradeMenu)).bg2hofs = 0;
            (*crate::gl!(sTradeMenu)).bg3hofs = 0;
            SetActiveMenuOptions();
            crate::gl!(gMain.state += 1);
        }
        18 => {
            (*gPaletteFade.as_ptr()).set_bufferTransferDisabled(FALSE as u16);
            BlendPalettes(PALETTES_ALL, 16, 0);
            BeginNormalPaletteFade(PALETTES_ALL, 0, 16, 0, 0);
            crate::gl!(gMain.state += 1);
        }
        19 => {
            SetGpuReg(REG_OFFSET_DISPCNT, 4160);
            LoadTradeBgGfx(2);
            crate::gl!(gMain.state += 1);
        }
        20 => {
            crate::gl!(gMain.state += 1);
        }
        21 => {
            SetTradePartyHPBarSprites();
            crate::gl!(gMain.state += 1);
        }
        22 if (*gPaletteFade.as_ptr()).active() == 0 => {
            SetMainCallback2(Some(CB2_TradeMenu));
        }
        _ => {}
    }
    RunTasks();
    AnimateSprites();
    BuildOamBuffer();
    UpdatePaletteFade();
}
pub(crate) unsafe fn VBlankCB_TradeMenu() {
    LoadOam();
    ProcessSpriteCopyRequests();
    TransferPlttBuffer();
}
unsafe fn CB_FadeToStartTrade() {
    if ({
        (*crate::gl!(sTradeMenu)).timer += 1;
        (*crate::gl!(sTradeMenu)).timer
    }) > 15
    {
        BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 16, 0);
        (*crate::gl!(sTradeMenu)).callbackId = CB_WAIT_TO_START_TRADE;
    }
}
unsafe fn CB_WaitToStartTrade() {
    if (*gPaletteFade.as_ptr()).active() == 0 {
        crate::gl!(gSelectedTradeMonPositions[0] = (*crate::gl!(sTradeMenu)).cursorPosition);
        crate::gl!(gSelectedTradeMonPositions[1] = (*crate::gl!(sTradeMenu)).partnerCursorPosition);
        if gWirelessCommType.get() != 0 {
            (*crate::gl!(sTradeMenu)).callbackId = CB_WAIT_TO_START_RFU_TRADE;
        } else {
            SetCloseLinkCallbackAndType(32);
            (*crate::gl!(sTradeMenu)).callbackId = CB_START_LINK_TRADE;
        }
    }
}
unsafe fn CB_StartLinkTrade() {
    crate::gl!(gMain.savedCallback = Some(CB2_StartCreateTradeMenu));
    if gWirelessCommType.get() != 0 {
        if IsLinkRfuTaskFinished() != 0 {
            Free(crate::gl!(sMenuTextTileBuffer) as *mut c_void);
            FreeAllWindowBuffers();
            Free(crate::gl!(sTradeMenu) as *mut c_void);
            crate::gl!(gMain.callback1 = None);
            DestroyWirelessStatusIndicatorSprite();
            SetMainCallback2(Some(CB2_LinkTrade));
        }
    } else {
        if gReceivedRemoteLinkPlayers.get() == 0 {
            Free(crate::gl!(sMenuTextTileBuffer) as *mut c_void);
            FreeAllWindowBuffers();
            Free(crate::gl!(sTradeMenu) as *mut c_void);
            crate::gl!(gMain.callback1 = None);
            SetMainCallback2(Some(CB2_LinkTrade));
        }
    }
}
pub(crate) unsafe fn CB2_TradeMenu() {
    RunTradeMenuCallback();
    DoQueuedActions();
    DrawSelectedMonScreen(TRADE_PLAYER);
    DrawSelectedMonScreen(TRADE_PARTNER);
    SetGpuReg(
        REG_OFFSET_BG2HOFS,
        ({
            let t1 = (*crate::gl!(sTradeMenu)).bg2hofs;
            (*crate::gl!(sTradeMenu)).bg2hofs += 1;
            t1
        }) as u16,
    );
    SetGpuReg(
        REG_OFFSET_BG3HOFS,
        ({
            let t2 = (*crate::gl!(sTradeMenu)).bg3hofs;
            (*crate::gl!(sTradeMenu)).bg3hofs -= 1;
            t2
        }) as u16,
    );
    RunTextPrintersAndIsPrinter0Active();
    RunTasks();
    AnimateSprites();
    BuildOamBuffer();
    UpdatePaletteFade();
}
unsafe fn LoadTradeBgGfx(state: u8) {
    match state {
        0 => {
            LoadPalette(
                (*(&raw const crate::data::graphics::gTradeMenu_Pal).cast::<CArray<u16, 0>>())
                    .as_ptr()
                    .cast_mut() as *mut c_void,
                0,
                96,
            );
            LoadBgTiles(
                1,
                (*(&raw const crate::data::graphics::gTradeMenu_Gfx).cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut() as *mut c_void,
                0x1280,
                0,
            );
            CopyToBgTilemapBufferRect_ChangePalette(
                1,
                (*(&raw const crate::data::graphics::gTradeMenu_Tilemap).cast::<CArray<u16, 0>>())
                    .as_ptr()
                    .cast_mut() as *mut c_void,
                0,
                0,
                32,
                20,
                0,
            );
            LoadBgTilemap(
                2,
                sTradeStripesBG2Tilemap.as_ptr().cast_mut() as *mut c_void,
                0x800,
                0,
            );
        }
        1 => {
            LoadBgTilemap(
                3,
                sTradeStripesBG3Tilemap.as_ptr().cast_mut() as *mut c_void,
                0x800,
                0,
            );
            PrintPartyLevelsAndGenders(TRADE_PLAYER);
            PrintPartyLevelsAndGenders(TRADE_PARTNER);
            CopyBgTilemapBufferToVram(1);
        }
        2 => {
            for i in 0..4i32 {
                SetGpuReg(REG_OFFSET_BG0HOFS + i as u8 * 2, 0);
            }
            ShowBg(0);
            ShowBg(1);
            ShowBg(2);
            ShowBg(3);
        }
        _ => {}
    }
}
unsafe fn SetActiveMenuOptions() {
    for i in 0..PARTY_SIZE {
        if i < (*crate::gl!(sTradeMenu)).partyCounts[0] as i32 {
            (*gSprites.as_ptr())[(*crate::gl!(sTradeMenu)).partySpriteIds[0][i]].set_invisible(0);
            (*crate::gl!(sTradeMenu)).optionsActive[i] = TRUE;
        } else {
            (*crate::gl!(sTradeMenu)).optionsActive[i] = FALSE;
        }
        if i < (*crate::gl!(sTradeMenu)).partyCounts[1] as i32 {
            (*gSprites.as_ptr())[(*crate::gl!(sTradeMenu)).partySpriteIds[1][i]]
                .set_invisible(FALSE as u16);
            (*crate::gl!(sTradeMenu)).optionsActive[i + PARTY_SIZE] = TRUE;
        } else {
            (*crate::gl!(sTradeMenu)).optionsActive[i + PARTY_SIZE] = FALSE;
        }
    }
    (*crate::gl!(sTradeMenu)).optionsActive[12] = TRUE;
}
unsafe fn Trade_Memcpy(dest: *mut c_void, src: *mut c_void, size: u32) {
    let mut _dest: *mut u8 = dest as *mut u8;
    let mut _src: *mut u8 = src as *mut u8;
    for i in 0..size {
        *_dest.at(i) = *_src.at(i);
    }
}
unsafe fn BufferTradeParties() -> u8 {
    let id: u8 = GetMultiplayerId();
    let mut mon: *mut Pokemon = null_mut();
    match (*crate::gl!(sTradeMenu)).bufferPartyState {
        0 => {
            Trade_Memcpy(
                (*gBlockSendBuffer.as_ptr()).as_mut_ptr() as *mut c_void,
                crate::gl!(&raw gPlayerParty[0]) as *mut c_void,
                200,
            );
            (*crate::gl!(sTradeMenu)).bufferPartyState += 1;
            (*crate::gl!(sTradeMenu)).timer = 0;
        }
        1 => {
            if IsLinkTradeTaskFinished() != 0 {
                if _GetBlockReceivedStatus() == 0 {
                    (*crate::gl!(sTradeMenu)).bufferPartyState += 1;
                } else {
                    TradeResetReceivedFlags();
                    (*crate::gl!(sTradeMenu)).bufferPartyState += 1;
                }
            }
        }
        3 => {
            if id == 0 {
                RequestLinkData(BLOCK_REQ_SIZE_200);
            }
            (*crate::gl!(sTradeMenu)).bufferPartyState += 1;
        }
        4 => {
            if _GetBlockReceivedStatus() == 3 {
                Trade_Memcpy(
                    crate::gl!(&raw gEnemyParty[0]) as *mut c_void,
                    (*gBlockRecvBuffer.as_ptr())[id as i32 ^ 1].as_mut_ptr() as *mut c_void,
                    200,
                );
                TradeResetReceivedFlags();
                (*crate::gl!(sTradeMenu)).bufferPartyState += 1;
            }
        }
        5 => {
            Trade_Memcpy(
                (*gBlockSendBuffer.as_ptr()).as_mut_ptr() as *mut c_void,
                crate::gl!(&raw gPlayerParty[2]) as *mut c_void,
                200,
            );
            (*crate::gl!(sTradeMenu)).bufferPartyState += 1;
        }
        7 => {
            if id == 0 {
                RequestLinkData(BLOCK_REQ_SIZE_200);
            }
            (*crate::gl!(sTradeMenu)).bufferPartyState += 1;
        }
        8 => {
            if _GetBlockReceivedStatus() == 3 {
                Trade_Memcpy(
                    crate::gl!(&raw gEnemyParty[2]) as *mut c_void,
                    (*gBlockRecvBuffer.as_ptr())[id as i32 ^ 1].as_mut_ptr() as *mut c_void,
                    200,
                );
                TradeResetReceivedFlags();
                (*crate::gl!(sTradeMenu)).bufferPartyState += 1;
            }
        }
        9 => {
            Trade_Memcpy(
                (*gBlockSendBuffer.as_ptr()).as_mut_ptr() as *mut c_void,
                crate::gl!(&raw gPlayerParty[4]) as *mut c_void,
                200,
            );
            (*crate::gl!(sTradeMenu)).bufferPartyState += 1;
        }
        11 => {
            if id == 0 {
                RequestLinkData(BLOCK_REQ_SIZE_200);
            }
            (*crate::gl!(sTradeMenu)).bufferPartyState += 1;
        }
        12 => {
            if _GetBlockReceivedStatus() == 3 {
                Trade_Memcpy(
                    crate::gl!(&raw gEnemyParty[4]) as *mut c_void,
                    (*gBlockRecvBuffer.as_ptr())[id as i32 ^ 1].as_mut_ptr() as *mut c_void,
                    200,
                );
                TradeResetReceivedFlags();
                (*crate::gl!(sTradeMenu)).bufferPartyState += 1;
            }
        }
        13 => {
            Trade_Memcpy(
                (*gBlockSendBuffer.as_ptr()).as_mut_ptr() as *mut c_void,
                (*crate::gl!(gSaveBlock1Ptr)).mail.as_mut_ptr() as *mut c_void,
                220,
            );
            (*crate::gl!(sTradeMenu)).bufferPartyState += 1;
        }
        15 => {
            if id == 0 {
                RequestLinkData(BLOCK_REQ_SIZE_220);
            }
            (*crate::gl!(sTradeMenu)).bufferPartyState += 1;
        }
        16 => {
            if _GetBlockReceivedStatus() == 3 {
                Trade_Memcpy(
                    (*gTradeMail.as_ptr()).as_mut_ptr() as *mut c_void,
                    (*gBlockRecvBuffer.as_ptr())[id as i32 ^ 1].as_mut_ptr() as *mut c_void,
                    216,
                );
                TradeResetReceivedFlags();
                (*crate::gl!(sTradeMenu)).bufferPartyState += 1;
            }
        }
        17 => {
            Trade_Memcpy(
                (*gBlockSendBuffer.as_ptr()).as_mut_ptr() as *mut c_void,
                (*crate::gl!(gSaveBlock1Ptr)).giftRibbons.as_mut_ptr() as *mut c_void,
                11,
            );
            (*crate::gl!(sTradeMenu)).bufferPartyState += 1;
        }
        19 => {
            if id == 0 {
                RequestLinkData(BLOCK_REQ_SIZE_40);
            }
            (*crate::gl!(sTradeMenu)).bufferPartyState += 1;
        }
        20 => {
            if _GetBlockReceivedStatus() == 3 {
                Trade_Memcpy(
                    (*crate::gl!(sTradeMenu)).giftRibbons.as_mut_ptr() as *mut c_void,
                    (*gBlockRecvBuffer.as_ptr())[id as i32 ^ 1].as_mut_ptr() as *mut c_void,
                    11,
                );
                TradeResetReceivedFlags();
                (*crate::gl!(sTradeMenu)).bufferPartyState += 1;
            }
        }
        21 => {
            mon = (*gEnemyParty.as_ptr()).as_mut_ptr();
            for i in 0..PARTY_SIZE {
                let mut name: CArray<u8, 11> = zeroed();
                let species: u16 = GetMonData2(mon, MON_DATA_SPECIES) as u16;
                if species != SPECIES_NONE
                    && species == SPECIES_SHEDINJA
                    && GetMonData2(mon, MON_DATA_LANGUAGE) != LANGUAGE_JAPANESE as u32
                {
                    GetMonData3(mon, MON_DATA_NICKNAME, name.as_mut_ptr());
                    if StringCompareWithoutExtCtrlCodes(
                        name.as_mut_ptr(),
                        sText_ShedinjaJP.as_ptr().cast_mut(),
                    ) == 0
                    {
                        SetMonData(
                            mon,
                            MON_DATA_NICKNAME,
                            (*(&raw const crate::data::data_tables::gSpeciesNames).cast::<CArray<
                                CArray<u8, 11>,
                                0,
                            >>(
                            ))[303]
                                .as_ptr()
                                .cast_mut() as *mut c_void,
                        );
                    }
                }
                mon = mon.at(1);
            }
            return TRUE;
        }
        2 | 6 | 10 | 14 | 18 => {
            (*crate::gl!(sTradeMenu)).timer += 1;
            if (*crate::gl!(sTradeMenu)).timer > 10 {
                (*crate::gl!(sTradeMenu)).timer = 0;
                (*crate::gl!(sTradeMenu)).bufferPartyState += 1;
            }
        }
        _ => {}
    }
    FALSE
}
unsafe fn PrintIsThisTradeOkay() {
    DrawBottomRowText(
        sText_IsThisTradeOkay.as_ptr().cast_mut(),
        (OBJ_VRAM0 + (*crate::gl!(sTradeMenu)).bottomTextTileStart as i32 * 32) as usize
            as *mut c_void as *mut u8,
        24,
    );
}
unsafe fn Leader_ReadLinkBuffer(mpId: u8, status: u8) {
    if status as i32 & 1 != 0 {
        match crate::gl!(gBlockRecvBuffer[0][0]) {
            LINKCMD_REQUEST_CANCEL => {
                (*crate::gl!(sTradeMenu)).playerSelectStatus = STATUS_CANCEL;
            }
            LINKCMD_READY_TO_TRADE => {
                (*crate::gl!(sTradeMenu)).playerSelectStatus = STATUS_READY;
            }
            LINKCMD_INIT_BLOCK => {
                (*crate::gl!(sTradeMenu)).playerConfirmStatus = STATUS_READY;
            }
            LINKCMD_READY_CANCEL_TRADE => {
                (*crate::gl!(sTradeMenu)).playerConfirmStatus = STATUS_CANCEL;
            }
            _ => {}
        }
        TradeResetReceivedFlag(0);
    }
    if status as i32 & 2 != 0 {
        match crate::gl!(gBlockRecvBuffer[1][0]) {
            LINKCMD_REQUEST_CANCEL => {
                (*crate::gl!(sTradeMenu)).partnerSelectStatus = STATUS_CANCEL;
            }
            LINKCMD_READY_TO_TRADE => {
                (*crate::gl!(sTradeMenu)).partnerCursorPosition =
                    crate::gl!(gBlockRecvBuffer[1][1]) as u8 + PARTY_SIZE as u8;
                (*crate::gl!(sTradeMenu)).partnerSelectStatus = STATUS_READY;
            }
            LINKCMD_INIT_BLOCK => {
                (*crate::gl!(sTradeMenu)).partnerConfirmStatus = STATUS_READY;
            }
            LINKCMD_READY_CANCEL_TRADE => {
                (*crate::gl!(sTradeMenu)).partnerConfirmStatus = STATUS_CANCEL;
            }
            _ => {}
        }
        TradeResetReceivedFlag(1);
    }
}
unsafe fn Follower_ReadLinkBuffer(mpId: u8, status: u8) {
    if status as i32 & 1 != 0 {
        match crate::gl!(gBlockRecvBuffer[0][0]) {
            LINKCMD_BOTH_CANCEL_TRADE => {
                BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 16, 0);
                PrintTradeMessage(MSG_WAITING_FOR_FRIEND);
                (*crate::gl!(sTradeMenu)).callbackId = CB_INIT_EXIT_CANCELED_TRADE;
            }
            LINKCMD_PARTNER_CANCEL_TRADE => {
                PrintTradeMessage(MSG_FRIEND_WANTS_TO_TRADE);
                (*crate::gl!(sTradeMenu)).callbackId = CB_HANDLE_TRADE_CANCELED;
            }
            LINKCMD_SET_MONS_TO_TRADE => {
                (*crate::gl!(sTradeMenu)).partnerCursorPosition =
                    crate::gl!(gBlockRecvBuffer[0][1]) as u8 + PARTY_SIZE as u8;
                rbox_fill_rectangle(0);
                SetSelectedMon((*crate::gl!(sTradeMenu)).cursorPosition);
                SetSelectedMon((*crate::gl!(sTradeMenu)).partnerCursorPosition);
                (*crate::gl!(sTradeMenu)).callbackId = CB_PRINT_IS_THIS_OKAY;
            }
            LINKCMD_START_TRADE => {
                BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 16, 0);
                (*crate::gl!(sTradeMenu)).callbackId = CB_WAIT_TO_START_TRADE;
            }
            LINKCMD_PLAYER_CANCEL_TRADE => {
                PrintTradeMessage(MSG_CANCELED);
                (*crate::gl!(sTradeMenu)).callbackId = CB_HANDLE_TRADE_CANCELED;
            }
            _ => {}
        }
        TradeResetReceivedFlag(0);
    }
    if status as i32 & 2 != 0 {
        TradeResetReceivedFlag(1);
    }
}
unsafe fn Leader_HandleCommunication() {
    if (*crate::gl!(sTradeMenu)).playerSelectStatus != STATUS_NONE
        && (*crate::gl!(sTradeMenu)).partnerSelectStatus != STATUS_NONE
    {
        if (*crate::gl!(sTradeMenu)).playerSelectStatus == STATUS_READY
            && (*crate::gl!(sTradeMenu)).partnerSelectStatus == STATUS_READY
        {
            (*crate::gl!(sTradeMenu)).callbackId = CB_SET_SELECTED_MONS;
            (*crate::gl!(sTradeMenu)).linkData[0] = LINKCMD_SET_MONS_TO_TRADE;
            (*crate::gl!(sTradeMenu)).linkData[1] = (*crate::gl!(sTradeMenu)).cursorPosition as u16;
            QueueAction(QUEUE_DELAY_DATA, QUEUE_SEND_DATA);
            (*crate::gl!(sTradeMenu)).playerSelectStatus = {
                (*crate::gl!(sTradeMenu)).partnerSelectStatus = STATUS_NONE;
                (*crate::gl!(sTradeMenu)).partnerSelectStatus
            };
        } else if (*crate::gl!(sTradeMenu)).playerSelectStatus == STATUS_READY
            && (*crate::gl!(sTradeMenu)).partnerSelectStatus == STATUS_CANCEL
        {
            PrintTradeMessage(MSG_CANCELED);
            (*crate::gl!(sTradeMenu)).linkData[0] = LINKCMD_PARTNER_CANCEL_TRADE;
            (*crate::gl!(sTradeMenu)).linkData[1] = 0;
            QueueAction(QUEUE_DELAY_DATA, QUEUE_SEND_DATA);
            (*crate::gl!(sTradeMenu)).playerConfirmStatus = {
                (*crate::gl!(sTradeMenu)).partnerConfirmStatus = STATUS_NONE;
                (*crate::gl!(sTradeMenu)).partnerConfirmStatus
            };
            (*crate::gl!(sTradeMenu)).playerSelectStatus = {
                (*crate::gl!(sTradeMenu)).partnerSelectStatus = STATUS_NONE;
                (*crate::gl!(sTradeMenu)).partnerSelectStatus
            };
            (*crate::gl!(sTradeMenu)).callbackId = CB_HANDLE_TRADE_CANCELED;
        } else if (*crate::gl!(sTradeMenu)).playerSelectStatus == STATUS_CANCEL
            && (*crate::gl!(sTradeMenu)).partnerSelectStatus == STATUS_READY
        {
            PrintTradeMessage(MSG_FRIEND_WANTS_TO_TRADE);
            (*crate::gl!(sTradeMenu)).linkData[0] = LINKCMD_PLAYER_CANCEL_TRADE;
            (*crate::gl!(sTradeMenu)).linkData[1] = 0;
            QueueAction(QUEUE_DELAY_DATA, QUEUE_SEND_DATA);
            (*crate::gl!(sTradeMenu)).playerConfirmStatus = {
                (*crate::gl!(sTradeMenu)).partnerConfirmStatus = STATUS_NONE;
                (*crate::gl!(sTradeMenu)).partnerConfirmStatus
            };
            (*crate::gl!(sTradeMenu)).playerSelectStatus = {
                (*crate::gl!(sTradeMenu)).partnerSelectStatus = STATUS_NONE;
                (*crate::gl!(sTradeMenu)).partnerSelectStatus
            };
            (*crate::gl!(sTradeMenu)).callbackId = CB_HANDLE_TRADE_CANCELED;
        } else if (*crate::gl!(sTradeMenu)).playerSelectStatus == STATUS_CANCEL
            && (*crate::gl!(sTradeMenu)).partnerSelectStatus == STATUS_CANCEL
        {
            (*crate::gl!(sTradeMenu)).linkData[0] = LINKCMD_BOTH_CANCEL_TRADE;
            (*crate::gl!(sTradeMenu)).linkData[1] = 0;
            QueueAction(QUEUE_DELAY_DATA, QUEUE_SEND_DATA);
            BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 16, 0);
            (*crate::gl!(sTradeMenu)).playerSelectStatus = {
                (*crate::gl!(sTradeMenu)).partnerSelectStatus = STATUS_NONE;
                (*crate::gl!(sTradeMenu)).partnerSelectStatus
            };
            (*crate::gl!(sTradeMenu)).callbackId = CB_INIT_EXIT_CANCELED_TRADE;
        }
    }
    if (*crate::gl!(sTradeMenu)).playerConfirmStatus != STATUS_NONE
        && (*crate::gl!(sTradeMenu)).partnerConfirmStatus != STATUS_NONE
    {
        if (*crate::gl!(sTradeMenu)).playerConfirmStatus == STATUS_READY
            && (*crate::gl!(sTradeMenu)).partnerConfirmStatus == STATUS_READY
        {
            (*crate::gl!(sTradeMenu)).linkData[0] = LINKCMD_START_TRADE;
            (*crate::gl!(sTradeMenu)).linkData[1] = 0;
            QueueAction(QUEUE_DELAY_DATA, QUEUE_SEND_DATA);
            (*crate::gl!(sTradeMenu)).playerConfirmStatus = STATUS_NONE;
            (*crate::gl!(sTradeMenu)).partnerConfirmStatus = STATUS_NONE;
            (*crate::gl!(sTradeMenu)).callbackId = CB_FADE_TO_START_TRADE;
        }
        if (*crate::gl!(sTradeMenu)).playerConfirmStatus == STATUS_CANCEL
            || (*crate::gl!(sTradeMenu)).partnerConfirmStatus == STATUS_CANCEL
        {
            PrintTradeMessage(MSG_CANCELED);
            (*crate::gl!(sTradeMenu)).linkData[0] = LINKCMD_PLAYER_CANCEL_TRADE;
            (*crate::gl!(sTradeMenu)).linkData[1] = 0;
            QueueAction(QUEUE_DELAY_DATA, QUEUE_SEND_DATA);
            (*crate::gl!(sTradeMenu)).playerConfirmStatus = STATUS_NONE;
            (*crate::gl!(sTradeMenu)).partnerConfirmStatus = STATUS_NONE;
            (*crate::gl!(sTradeMenu)).callbackId = CB_HANDLE_TRADE_CANCELED;
        }
    }
}
unsafe fn _SetLinkData(linkData: *mut u16, linkCmd: u16, cursorPosition: u16) {
    *linkData = linkCmd;
    *linkData.at(1) = cursorPosition;
    QueueAction(QUEUE_DELAY_DATA, QUEUE_SEND_DATA);
}
unsafe fn SetLinkData(linkCmd: u16, cursorPosition: u16) {
    _SetLinkData(
        (*crate::gl!(sTradeMenu)).linkData.as_mut_ptr(),
        linkCmd,
        cursorPosition,
    );
}
pub(crate) unsafe fn CB1_UpdateLink() {
    let mpId: u8 = GetMultiplayerId();
    let mut status: u8 = 0;
    if ({
        status = _GetBlockReceivedStatus() as u8;
        status
    }) != 0
    {
        if mpId == 0 {
            Leader_ReadLinkBuffer(mpId, status);
        } else {
            Follower_ReadLinkBuffer(mpId, status);
        }
    }
    if mpId == 0 {
        Leader_HandleCommunication();
    }
}
unsafe fn GetNewCursorPosition(oldPosition: u8, direction: u8) -> u8 {
    let mut newPosition: u8 = 0;
    for i in 0..PARTY_SIZE {
        if (*crate::gl!(sTradeMenu)).optionsActive
            [sCursorMoveDestinations[oldPosition][direction][i]]
            == TRUE
        {
            newPosition = sCursorMoveDestinations[oldPosition][direction][i];
            break;
        }
    }
    newPosition
}
unsafe fn TradeMenuMoveCursor(cursorPosition: *mut u8, direction: u8) {
    let newPosition: u8 = GetNewCursorPosition(*cursorPosition, direction);
    if newPosition == 12 {
        StartSpriteAnim(
            {
                let gl_i0 = (*crate::gl!(sTradeMenu)).cursorSpriteId;
                crate::gl!(&raw gSprites[gl_i0])
            },
            CURSOR_ANIM_ON_CANCEL,
        );
        {
            let gl_i1 = (*crate::gl!(sTradeMenu)).cursorSpriteId;
            crate::gl!(gSprites[gl_i1].x = 224)
        };
        {
            let gl_i2 = (*crate::gl!(sTradeMenu)).cursorSpriteId;
            crate::gl!(gSprites[gl_i2].y = DISPLAY_HEIGHT as i16)
        };
    } else {
        StartSpriteAnim(
            {
                let gl_i3 = (*crate::gl!(sTradeMenu)).cursorSpriteId;
                crate::gl!(&raw gSprites[gl_i3])
            },
            CURSOR_ANIM_NORMAL,
        );
        {
            let gl_i4 = (*crate::gl!(sTradeMenu)).cursorSpriteId;
            crate::gl!(gSprites[gl_i4].x = sTradeMonSpriteCoords[newPosition][0] as i16 * 8 + 32)
        };
        {
            let gl_i5 = (*crate::gl!(sTradeMenu)).cursorSpriteId;
            crate::gl!(gSprites[gl_i5].y = sTradeMonSpriteCoords[newPosition][1] as i16 * 8)
        };
    }
    if *cursorPosition != newPosition {
        PlaySE(SE_SELECT);
    }
    *cursorPosition = newPosition;
}
unsafe fn SetReadyToTrade() {
    PrintTradeMessage(MSG_STANDBY);
    (*crate::gl!(sTradeMenu)).callbackId = CB_IDLE;
    if GetMultiplayerId() == 1 {
        SetLinkData(
            LINKCMD_READY_TO_TRADE,
            (*crate::gl!(sTradeMenu)).cursorPosition as u16,
        );
    } else {
        (*crate::gl!(sTradeMenu)).playerSelectStatus = STATUS_READY;
    }
}
unsafe fn CB_ProcessMenuInput() {
    if crate::gl!(gMain.newAndRepeatedKeys) as i32 & DPAD_UP != 0 {
        TradeMenuMoveCursor(&raw mut (*crate::gl!(sTradeMenu)).cursorPosition, 0);
    } else if crate::gl!(gMain.newAndRepeatedKeys) as i32 & DPAD_DOWN != 0 {
        TradeMenuMoveCursor(&raw mut (*crate::gl!(sTradeMenu)).cursorPosition, 1);
    } else if crate::gl!(gMain.newAndRepeatedKeys) as i32 & DPAD_LEFT != 0 {
        TradeMenuMoveCursor(&raw mut (*crate::gl!(sTradeMenu)).cursorPosition, 2);
    } else if crate::gl!(gMain.newAndRepeatedKeys) as i32 & DPAD_RIGHT != 0 {
        TradeMenuMoveCursor(&raw mut (*crate::gl!(sTradeMenu)).cursorPosition, 3);
    }
    if crate::gl!(gMain.newKeys) as i32 & A_BUTTON != 0 {
        PlaySE(SE_SELECT);
        if (*crate::gl!(sTradeMenu)).cursorPosition < PARTY_SIZE as u8 {
            DrawTextBorderOuter(1, 1, 14);
            FillWindowPixelBuffer(1, 17);
            PrintMenuTable(1, 2, sSelectTradeMonActions.as_ptr().cast_mut());
            InitMenuInUpperLeftCornerNormal(1, 2, 0);
            PutWindowTilemap(1);
            CopyWindowToVram(1, COPYWIN_FULL);
            (*crate::gl!(sTradeMenu)).callbackId = CB_SELECTED_MON;
        } else if (*crate::gl!(sTradeMenu)).cursorPosition < 12 {
            BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 16, 0);
            (*crate::gl!(sTradeMenu)).callbackId = CB_SHOW_MON_SUMMARY;
        } else if (*crate::gl!(sTradeMenu)).cursorPosition == 12 {
            CreateYesNoMenu((&raw const *sTradeYesNoWindowTemplate).cast_mut(), 1, 14, 0);
            (*crate::gl!(sTradeMenu)).callbackId = CB_CANCEL_TRADE_PROMPT;
            DrawBottomRowText(
                sActionTexts[4],
                (OBJ_VRAM0 + (*crate::gl!(sTradeMenu)).bottomTextTileStart as i32 * 32) as usize
                    as *mut c_void as *mut u8,
                24,
            );
        }
    }
}
unsafe fn RedrawChooseAPokemonWindow() {
    PrintTradePartnerPartyNicknames();
    (*crate::gl!(sTradeMenu)).callbackId = CB_MAIN_MENU;
    (*gSprites.as_ptr())[(*crate::gl!(sTradeMenu)).cursorSpriteId].set_invisible(FALSE as u16);
    DrawBottomRowText(
        sActionTexts[1],
        (OBJ_VRAM0 + (*crate::gl!(sTradeMenu)).bottomTextTileStart as i32 * 32) as usize
            as *mut c_void as *mut u8,
        24,
    );
}
unsafe fn CB_ProcessSelectedMonInput() {
    match Menu_ProcessInputNoWrap() {
        MENU_B_PRESSED => {
            PlaySE(SE_SELECT);
            RedrawChooseAPokemonWindow();
        }
        MENU_NOTHING_CHOSEN => {}
        0 => {
            BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 16, 0);
            (*crate::gl!(sTradeMenu)).callbackId = CB_SHOW_MON_SUMMARY;
        }
        1 => {
            match CanTradeSelectedMon(
                (*gPlayerParty.as_ptr()).as_mut_ptr(),
                gPlayerPartyCount.get() as i32,
                (*crate::gl!(sTradeMenu)).cursorPosition as i32,
            ) {
                CAN_TRADE_MON => {
                    SetReadyToTrade();
                    (*gSprites.as_ptr())[(*crate::gl!(sTradeMenu)).cursorSpriteId]
                        .set_invisible(TRUE as u16);
                }
                1 => {
                    QueueAction(QUEUE_DELAY_MSG, QUEUE_ONLY_MON2);
                    (*crate::gl!(sTradeMenu)).callbackId = CB_HANDLE_TRADE_CANCELED;
                }
                2 | CANT_TRADE_INVALID_MON => {
                    QueueAction(QUEUE_DELAY_MSG, QUEUE_MON_CANT_BE_TRADED);
                    (*crate::gl!(sTradeMenu)).callbackId = CB_HANDLE_TRADE_CANCELED;
                }
                3 | CANT_TRADE_PARTNER_EGG_YET => {
                    QueueAction(QUEUE_DELAY_MSG, QUEUE_EGG_CANT_BE_TRADED);
                    (*crate::gl!(sTradeMenu)).callbackId = CB_HANDLE_TRADE_CANCELED;
                }
                _ => {}
            }
        }
        _ => {}
    }
}
unsafe fn CB_ChooseMonAfterButtonPress() {
    if crate::gl!(gMain.newKeys) as i32 & A_BUTTON != 0
        || crate::gl!(gMain.newKeys) as i32 & B_BUTTON != 0
    {
        PlaySE(SE_SELECT);
        RedrawChooseAPokemonWindow();
    }
}
unsafe fn CB_ShowTradeMonSummaryScreen() {
    if (*gPaletteFade.as_ptr()).active() == 0 {
        if (*crate::gl!(sTradeMenu)).cursorPosition < PARTY_SIZE as u8 {
            ShowPokemonSummaryScreen(
                SUMMARY_MODE_LOCK_MOVES,
                (*gPlayerParty.as_ptr()).as_mut_ptr() as *mut c_void,
                (*crate::gl!(sTradeMenu)).cursorPosition,
                (*crate::gl!(sTradeMenu)).partyCounts[0] - 1,
                Some(CB2_ReturnToTradeMenu),
            );
        } else {
            ShowPokemonSummaryScreen(
                SUMMARY_MODE_LOCK_MOVES,
                (*gEnemyParty.as_ptr()).as_mut_ptr() as *mut c_void,
                (*crate::gl!(sTradeMenu)).cursorPosition - PARTY_SIZE as u8,
                (*crate::gl!(sTradeMenu)).partyCounts[1] - 1,
                Some(CB2_ReturnToTradeMenu),
            );
        }
        FreeAllWindowBuffers();
    }
}
unsafe fn CheckValidityOfTradeMons(
    aliveMons: *mut u8,
    playerPartyCount: u8,
    playerMonIdx: u8,
    mut partnerMonIdx: u8,
) -> u8 {
    let mut hasLiveMon: u8 = 0;
    for i in 0..(playerPartyCount as i32) {
        if playerMonIdx as i32 != i {
            hasLiveMon += *aliveMons.at(i);
        }
    }
    partnerMonIdx = (partnerMonIdx as i32 % 6) as u8;
    let partnerSpecies: u16 = GetMonData2(
        crate::gl!(&raw gEnemyParty[partnerMonIdx]),
        MON_DATA_SPECIES,
    ) as u16;
    if (partnerSpecies == SPECIES_DEOXYS as u16 || partnerSpecies == SPECIES_MEW as u16)
        && GetMonData2(
            crate::gl!(&raw gEnemyParty[partnerMonIdx]),
            MON_DATA_MODERN_FATEFUL_ENCOUNTER,
        ) == 0
    {
        return PARTNER_MON_INVALID;
    }
    if IsNationalPokedexEnabled() == 0
        && ((*crate::gl!(sTradeMenu)).isEgg[1][partnerMonIdx] != 0
            || IsSpeciesInHoennDex(partnerSpecies) == 0)
    {
        return PARTNER_MON_INVALID;
    }
    if hasLiveMon != 0 {
        hasLiveMon = BOTH_MONS_VALID;
    }
    hasLiveMon
}
unsafe fn CheckMonsBeforeTrade() -> u32 {
    let mut aliveMons: CArray<u8, 12> = zeroed();
    for i in 0..((*crate::gl!(sTradeMenu)).partyCounts[0] as i32) {
        aliveMons[i] = (*crate::gl!(sTradeMenu)).isLiveMon[0][i];
    }
    match CheckValidityOfTradeMons(
        aliveMons.as_mut_ptr(),
        (*crate::gl!(sTradeMenu)).partyCounts[0],
        (*crate::gl!(sTradeMenu)).cursorPosition,
        (*crate::gl!(sTradeMenu)).partnerCursorPosition,
    ) {
        PLAYER_MON_INVALID => {
            QueueAction(QUEUE_DELAY_MSG, QUEUE_ONLY_MON2);
            SetLinkData(LINKCMD_READY_CANCEL_TRADE, 0);
        }
        BOTH_MONS_VALID => {
            QueueAction(QUEUE_DELAY_MSG, QUEUE_STANDBY);
            SetLinkData(LINKCMD_INIT_BLOCK, 0);
        }
        PARTNER_MON_INVALID => {
            QueueAction(QUEUE_DELAY_MSG, QUEUE_FRIENDS_MON_CANT_BE_TRADED);
            return TRUE as u32;
        }
        _ => {}
    }
    FALSE as u32
}
unsafe fn CB_ProcessConfirmTradeInput() {
    match Menu_ProcessInputNoWrapClearOnChoose() {
        0 => {
            if CheckMonsBeforeTrade() == 0 {
                (*crate::gl!(sTradeMenu)).callbackId = CB_IDLE;
            } else {
                (*crate::gl!(sTradeMenu)).callbackId = CB_PARTNER_MON_INVALID;
            }
            PutWindowTilemap(17);
        }
        1 | MENU_B_PRESSED => {
            QueueAction(QUEUE_DELAY_MSG, QUEUE_STANDBY);
            if IsLinkTradeTaskFinished() != 0 {
                SetLinkData(LINKCMD_READY_CANCEL_TRADE, 0);
            }
            (*crate::gl!(sTradeMenu)).callbackId = CB_IDLE;
            PutWindowTilemap(17);
        }
        _ => {}
    }
}
unsafe fn RestoreNicknamesCoveredByYesNo() {
    let mut i: i32 = 0;
    while i < (*crate::gl!(sTradeMenu)).partyCounts[1] as i32 - 4 {
        PutWindowTilemap(i as u8 + 12);
        CopyWindowToVram(i as u8 + 12, COPYWIN_MAP);
        i += 1;
    }
}
unsafe fn CB_ProcessCancelTradeInput() {
    match Menu_ProcessInputNoWrapClearOnChoose() {
        0 => {
            PrintTradeMessage(MSG_WAITING_FOR_FRIEND);
            SetLinkData(LINKCMD_REQUEST_CANCEL, 0);
            (*gSprites.as_ptr())[(*crate::gl!(sTradeMenu)).cursorSpriteId]
                .set_invisible(TRUE as u16);
            (*crate::gl!(sTradeMenu)).callbackId = CB_IDLE;
            RestoreNicknamesCoveredByYesNo();
        }
        1 | MENU_B_PRESSED => {
            PlaySE(SE_SELECT);
            RedrawChooseAPokemonWindow();
        }
        _ => {}
    }
}
unsafe fn CB_SetSelectedMons() {
    if GetMultiplayerId() == 0 {
        rbox_fill_rectangle(0);
        SetSelectedMon((*crate::gl!(sTradeMenu)).cursorPosition);
        SetSelectedMon((*crate::gl!(sTradeMenu)).partnerCursorPosition);
    }
    (*crate::gl!(sTradeMenu)).callbackId = CB_PRINT_IS_THIS_OKAY;
}
unsafe fn CB_PrintIsThisTradeOkay() {
    if (*crate::gl!(sTradeMenu)).drawSelectedMonState[0] == DRAW_SELECTED_FINISH
        && (*crate::gl!(sTradeMenu)).drawSelectedMonState[1] == DRAW_SELECTED_FINISH
    {
        PrintIsThisTradeOkay();
        (*crate::gl!(sTradeMenu)).callbackId = CB_INIT_CONFIRM_TRADE_PROMPT;
    }
}
unsafe fn CB_InitConfirmTradePrompt() {
    (*crate::gl!(sTradeMenu)).timer += 1;
    if (*crate::gl!(sTradeMenu)).timer > 120 {
        CreateYesNoMenu((&raw const *sTradeYesNoWindowTemplate).cast_mut(), 1, 14, 0);
        (*crate::gl!(sTradeMenu)).timer = 0;
        (*crate::gl!(sTradeMenu)).callbackId = CB_CONFIRM_TRADE_PROMPT;
    }
}
unsafe fn CB_HandleTradeCanceled() {
    if crate::gl!(gMain.newKeys) as i32 & A_BUTTON != 0 {
        PlaySE(SE_SELECT);
        rbox_fill_rectangle(0);
        rbox_fill_rectangle(1);
        for i in 0..4i32 {
            FillWindowPixelBuffer(i as u8 + 14, 0);
            rbox_fill_rectangle(i as u8 + 14);
        }
        RedrawPartyWindow(TRADE_PLAYER);
        RedrawPartyWindow(TRADE_PARTNER);
        (*crate::gl!(sTradeMenu)).callbackId = CB_MAIN_MENU;
        (*gSprites.as_ptr())[(*crate::gl!(sTradeMenu)).cursorSpriteId].set_invisible(FALSE as u16);
    }
}
unsafe fn CB_InitExitCanceledTrade() {
    if (*gPaletteFade.as_ptr()).active() == 0 {
        if gWirelessCommType.get() != 0 {
            SetLinkStandbyCallback();
        } else {
            SetCloseLinkCallbackAndType(12);
        }
        (*crate::gl!(sTradeMenu)).callbackId = CB_EXIT_CANCELED_TRADE;
    }
}
unsafe fn CB_ExitCanceledTrade() {
    if gWirelessCommType.get() != 0 {
        if IsLinkTradeTaskFinished() != 0 && GetNumQueuedActions() == 0 {
            Free(crate::gl!(sMenuTextTileBuffer) as *mut c_void);
            Free(crate::gl!(sTradeMenu) as *mut c_void);
            FreeAllWindowBuffers();
            DestroyWirelessStatusIndicatorSprite();
            SetMainCallback2(Some(CB2_ReturnToFieldFromMultiplayer));
        }
    } else {
        if gReceivedRemoteLinkPlayers.get() == 0 {
            Free(crate::gl!(sMenuTextTileBuffer) as *mut c_void);
            Free(crate::gl!(sTradeMenu) as *mut c_void);
            FreeAllWindowBuffers();
            SetMainCallback2(Some(CB2_ReturnToFieldFromMultiplayer));
        }
    }
}
unsafe fn CB_WaitToStartRfuTrade() {
    if Rfu_SetLinkRecovery(0) == 0 && GetNumQueuedActions() == 0 {
        SetLinkStandbyCallback();
        (*crate::gl!(sTradeMenu)).callbackId = CB_START_LINK_TRADE;
    }
}
unsafe fn CB_PartnersMonWasInvalid() {
    if crate::gl!(gMain.newKeys) as i32 & A_BUTTON != 0 {
        SetLinkData(LINKCMD_READY_CANCEL_TRADE, 0);
        (*crate::gl!(sTradeMenu)).callbackId = CB_IDLE;
    }
}
unsafe fn RunTradeMenuCallback() {
    match (*crate::gl!(sTradeMenu)).callbackId {
        CB_MAIN_MENU => {
            CB_ProcessMenuInput();
        }
        CB_SELECTED_MON => {
            CB_ProcessSelectedMonInput();
        }
        CB_SHOW_MON_SUMMARY => {
            CB_ShowTradeMonSummaryScreen();
        }
        CB_CONFIRM_TRADE_PROMPT => {
            CB_ProcessConfirmTradeInput();
        }
        CB_CANCEL_TRADE_PROMPT => {
            CB_ProcessCancelTradeInput();
        }
        CB_SET_SELECTED_MONS => {
            CB_SetSelectedMons();
        }
        CB_PRINT_IS_THIS_OKAY => {
            CB_PrintIsThisTradeOkay();
        }
        CB_HANDLE_TRADE_CANCELED => {
            CB_HandleTradeCanceled();
        }
        CB_FADE_TO_START_TRADE => {
            CB_FadeToStartTrade();
        }
        CB_WAIT_TO_START_TRADE => {
            CB_WaitToStartTrade();
        }
        CB_INIT_EXIT_CANCELED_TRADE => {
            CB_InitExitCanceledTrade();
        }
        CB_EXIT_CANCELED_TRADE => {
            CB_ExitCanceledTrade();
        }
        CB_START_LINK_TRADE => {
            CB_StartLinkTrade();
        }
        CB_INIT_CONFIRM_TRADE_PROMPT => {
            CB_InitConfirmTradePrompt();
        }
        CB_UNUSED_CLOSE_MSG => {
            CB_ChooseMonAfterButtonPress();
        }
        CB_WAIT_TO_START_RFU_TRADE => {
            CB_WaitToStartRfuTrade();
        }
        CB_PARTNER_MON_INVALID => {
            CB_PartnersMonWasInvalid();
        }
        _ => {}
    }
}
unsafe fn SetSelectedMon(cursorPosition: u8) {
    let whichParty: u8 = (cursorPosition as i32 / 6) as u8;
    if (*crate::gl!(sTradeMenu)).drawSelectedMonState[whichParty] == 0 {
        (*crate::gl!(sTradeMenu)).drawSelectedMonState[whichParty] = 1;
        (*crate::gl!(sTradeMenu)).selectedMonIdx[whichParty] = cursorPosition;
    }
}
unsafe fn DrawSelectedMonScreen(whichParty: u8) {
    let mut nickname: CArray<u8, 20> = zeroed();
    let mut movesString: CArray<u8, 56> = zeroed();
    let mut i: u8 = 0;
    let selectedMonIdx: u8 = (*crate::gl!(sTradeMenu)).selectedMonIdx[whichParty];
    let mut selectedMonParty: u8 = TRADE_PARTNER;
    if (*crate::gl!(sTradeMenu)).selectedMonIdx[whichParty] < PARTY_SIZE as u8 {
        selectedMonParty = TRADE_PLAYER;
    }
    let partyIdx: u8 = (selectedMonIdx as i32 % 6) as u8;
    let mut nameStringWidth: i8 = 0;
    match (*crate::gl!(sTradeMenu)).drawSelectedMonState[whichParty] {
        1 => {
            i = 0;
            while i < (*crate::gl!(sTradeMenu)).partyCounts[whichParty] {
                (*gSprites.as_ptr())[(*crate::gl!(sTradeMenu)).partySpriteIds[selectedMonParty][i]]
                    .set_invisible(TRUE as u16);
                i += 1;
            }
            for i in 0..(PARTY_SIZE as u8) {
                ClearWindowTilemap(i + (whichParty * PARTY_SIZE as u8 + 2));
            }
            (*gSprites.as_ptr())
                [(*crate::gl!(sTradeMenu)).partySpriteIds[selectedMonParty][partyIdx]]
                .set_invisible(FALSE as u16);
            {
                let gl_i6 = (*crate::gl!(sTradeMenu)).partySpriteIds[selectedMonParty][partyIdx];
                crate::gl!(gSprites[gl_i6].data[0] = 20)
            };
            {
                let gl_i7 = (*crate::gl!(sTradeMenu)).partySpriteIds[selectedMonParty][partyIdx];
                crate::gl!(
                    gSprites[gl_i7].data[2] =
                        ((sTradeMonSpriteCoords[selectedMonParty as i32 * PARTY_SIZE][0] as i32
                            + sTradeMonSpriteCoords[selectedMonParty as i32 * PARTY_SIZE + 1][0]
                                as i32)
                            / 2) as i16
                            * 8
                            + 14
                )
            };
            {
                let gl_i8 = (*crate::gl!(sTradeMenu)).partySpriteIds[selectedMonParty][partyIdx];
                crate::gl!(
                    gSprites[gl_i8].data[4] =
                        sTradeMonSpriteCoords[selectedMonParty as i32 * PARTY_SIZE][1] as i16 * 8
                            - 12
                )
            };
            StoreSpriteCallbackInData6(
                {
                    let gl_i9 =
                        (*crate::gl!(sTradeMenu)).partySpriteIds[selectedMonParty][partyIdx];
                    crate::gl!(&raw gSprites[gl_i9])
                },
                Some(SpriteCB_MonIcon),
            );
            (*crate::gl!(sTradeMenu)).drawSelectedMonState[whichParty] += 1;
            Trade_MoveSelectedMonToTarget({
                let gl_i10 = (*crate::gl!(sTradeMenu)).partySpriteIds[selectedMonParty][partyIdx];
                crate::gl!(&raw gSprites[gl_i10])
            });
            CopyToBgTilemapBufferRect_ChangePalette(
                1,
                sTradePartyBoxTilemap.as_ptr().cast_mut() as *mut c_void,
                whichParty * 15,
                0,
                15,
                17,
                0,
            );
            CopyBgTilemapBufferToVram(1);
            CopyBgTilemapBufferToVram(0);
            if selectedMonParty == TRADE_PLAYER {
                PrintTradePartnerPartyNicknames();
            }
        }
        2 => {
            if ({
                let gl_i11 = (*crate::gl!(sTradeMenu)).partySpriteIds[selectedMonParty][partyIdx];
                crate::gl!(gSprites[gl_i11].callback)
            }) == Some(SpriteCB_MonIcon as unsafe fn(*mut Sprite))
            {
                (*crate::gl!(sTradeMenu)).drawSelectedMonState[whichParty] = 3;
            }
        }
        3 => {
            CopyToBgTilemapBufferRect_ChangePalette(
                1,
                sTradeMovesBoxTilemap.as_ptr().cast_mut() as *mut c_void,
                selectedMonParty * 15,
                0,
                15,
                17,
                0,
            );
            CopyBgTilemapBufferToVram(1);
            {
                let gl_i12 = (*crate::gl!(sTradeMenu)).partySpriteIds[selectedMonParty][partyIdx];
                crate::gl!(
                    gSprites[gl_i12].x =
                        ((sTradeMonSpriteCoords[selectedMonParty as i32 * PARTY_SIZE][0] as i32
                            + sTradeMonSpriteCoords[selectedMonParty as i32 * PARTY_SIZE + 1][0]
                                as i32)
                            / 2) as i16
                            * 8
                            + 14
                )
            };
            {
                let gl_i13 = (*crate::gl!(sTradeMenu)).partySpriteIds[selectedMonParty][partyIdx];
                crate::gl!(
                    gSprites[gl_i13].y =
                        sTradeMonSpriteCoords[selectedMonParty as i32 * PARTY_SIZE][1] as i16 * 8
                            - 12
                )
            };
            {
                let gl_i14 = (*crate::gl!(sTradeMenu)).partySpriteIds[selectedMonParty][partyIdx];
                crate::gl!(gSprites[gl_i14].x2 = 0)
            };
            {
                let gl_i15 = (*crate::gl!(sTradeMenu)).partySpriteIds[selectedMonParty][partyIdx];
                crate::gl!(gSprites[gl_i15].y2 = 0)
            };
            nameStringWidth =
                GetMonNicknameWidth(nickname.as_mut_ptr(), selectedMonParty, partyIdx) as i8;
            AddTextPrinterParameterized3(
                whichParty * 2 + 14,
                FONT_SMALL,
                ((80 - nameStringWidth as i32) / 2) as u8,
                4,
                sTradeTextColors.as_ptr().cast_mut(),
                0,
                nickname.as_mut_ptr(),
            );
            BufferMovesString(movesString.as_mut_ptr(), selectedMonParty, partyIdx);
            AddTextPrinterParameterized4(
                whichParty * 2 + 15,
                FONT_NORMAL,
                0,
                0,
                0,
                0,
                sTradeTextColors.as_ptr().cast_mut(),
                0,
                movesString.as_mut_ptr(),
            );
            PutWindowTilemap(whichParty * 2 + 14);
            CopyWindowToVram(whichParty * 2 + 14, COPYWIN_FULL);
            PutWindowTilemap(whichParty * 2 + 15);
            CopyWindowToVram(whichParty * 2 + 15, COPYWIN_FULL);
            (*crate::gl!(sTradeMenu)).drawSelectedMonState[whichParty] += 1;
        }
        4 => {
            PrintLevelAndGender(
                whichParty,
                partyIdx,
                sSelectedMonLevelGenderCoords[whichParty][0] + 4,
                sSelectedMonLevelGenderCoords[whichParty][1] + 1,
                sSelectedMonLevelGenderCoords[whichParty][0],
                sSelectedMonLevelGenderCoords[whichParty][1],
            );
            (*crate::gl!(sTradeMenu)).drawSelectedMonState[whichParty] += 1;
        }
        _ => {}
    }
}
unsafe fn GetMonNicknameWidth(str: *mut u8, whichParty: u8, partyIdx: u8) -> u8 {
    let mut nickname: CArray<u8, 11> = zeroed();
    if whichParty == TRADE_PLAYER {
        GetMonData3(
            crate::gl!(&raw gPlayerParty[partyIdx]),
            MON_DATA_NICKNAME,
            nickname.as_mut_ptr(),
        );
    } else {
        GetMonData3(
            crate::gl!(&raw gEnemyParty[partyIdx]),
            MON_DATA_NICKNAME,
            nickname.as_mut_ptr(),
        );
    }
    StringCopy_Nickname(str, nickname.as_mut_ptr());
    GetStringWidth(
        FONT_SMALL,
        str,
        GetFontAttribute(FONT_SMALL, FONTATTR_LETTER_SPACING) as i16,
    ) as u8
}
unsafe fn BufferMovesString(str: *mut u8, whichParty: u8, partyIdx: u8) {
    let mut moves: CArray<u16, 4> = zeroed();
    let mut i: u16 = 0;
    if (*crate::gl!(sTradeMenu)).isEgg[whichParty][partyIdx] == 0 {
        i = 0;
        while i < MAX_MON_MOVES as u16 {
            if whichParty == TRADE_PLAYER {
                moves[i] = GetMonData3(
                    crate::gl!(&raw gPlayerParty[partyIdx]),
                    i as i32 + MON_DATA_MOVE1,
                    null_mut(),
                ) as u16;
            } else {
                moves[i] = GetMonData3(
                    crate::gl!(&raw gEnemyParty[partyIdx]),
                    i as i32 + MON_DATA_MOVE1,
                    null_mut(),
                ) as u16;
            }
            i += 1;
        }
        StringCopy(str, sText_EmptyString.as_ptr().cast_mut());
        for i in 0..(MAX_MON_MOVES as u16) {
            if moves[i] != MOVE_NONE {
                StringAppend(
                    str,
                    (*(&raw const crate::data::data_tables::gMoveNames)
                        .cast::<CArray<CArray<u8, 13>, 355>>())[moves[i]]
                        .as_ptr()
                        .cast_mut(),
                );
            }
            StringAppend(str, sText_NewLine.as_ptr().cast_mut());
        }
    } else {
        StringCopy(str, sText_EmptyString.as_ptr().cast_mut());
        StringAppend(str, sText_FourQuestionMarks.as_ptr().cast_mut());
    }
}
unsafe fn PrintPartyMonNickname(whichParty: u8, mut windowId: u8, nickname: *mut u8) {
    windowId += whichParty * PARTY_SIZE as u8 + 2;
    let xPos: u8 = GetStringCenterAlignXOffset(FONT_SMALL as i32, nickname, 64) as u8;
    AddTextPrinterParameterized3(
        windowId,
        FONT_SMALL,
        xPos,
        4,
        sTradeTextColors.as_ptr().cast_mut(),
        0,
        nickname,
    );
    PutWindowTilemap(windowId);
    CopyWindowToVram(windowId, COPYWIN_FULL);
}
unsafe fn PrintPartyNicknames(whichParty: u8) {
    let mut nickname: CArray<u8, 20> = zeroed();
    let mut str: CArray<u8, 32> = zeroed();
    let party: *mut Pokemon = if whichParty == TRADE_PLAYER {
        (*gPlayerParty.as_ptr()).as_mut_ptr()
    } else {
        (*gEnemyParty.as_ptr()).as_mut_ptr()
    };
    let mut i: u8 = 0;
    while i < (*crate::gl!(sTradeMenu)).partyCounts[whichParty] {
        GetMonData3(party.at(i), MON_DATA_NICKNAME, nickname.as_mut_ptr());
        StringCopy_Nickname(str.as_mut_ptr(), nickname.as_mut_ptr());
        PrintPartyMonNickname(whichParty, i, str.as_mut_ptr());
        i += 1;
    }
}
unsafe fn PrintLevelAndGender(whichParty: u8, monIdx: u8, x: u8, y: u8, width: u8, height: u8) {
    let mut level: u8 = 0;
    let mut symbolTile: u32 = 0;
    let mut gender: u8 = 0;
    let mut nickname: CArray<u8, 11> = zeroed();
    CopyToBgTilemapBufferRect_ChangePalette(
        1,
        (*(&raw const crate::data::graphics::gTradeMenuMonBox_Tilemap).cast::<CArray<u16, 0>>())
            .as_ptr()
            .cast_mut() as *mut c_void,
        width,
        height,
        6,
        3,
        0,
    );
    CopyBgTilemapBufferToVram(1);
    if whichParty == TRADE_PLAYER {
        level = GetMonData3(
            crate::gl!(&raw gPlayerParty[monIdx]),
            MON_DATA_LEVEL,
            null_mut(),
        ) as u8;
    } else {
        level = GetMonData3(
            crate::gl!(&raw gEnemyParty[monIdx]),
            MON_DATA_LEVEL,
            null_mut(),
        ) as u8;
    }
    if (*crate::gl!(sTradeMenu)).isEgg[whichParty][monIdx] == 0 {
        if level as i32 / 10 != 0 {
            (*crate::gl!(sTradeMenu)).tilemapBuffer[x as i32 + y as i32 * 32] =
                (level as i32 / 10) as u16 + 0x60;
        }
        (*crate::gl!(sTradeMenu)).tilemapBuffer[x as i32 + y as i32 * 32 + 1] =
            (level as i32 % 10) as u16 + 0x70;
    } else {
        (*crate::gl!(sTradeMenu)).tilemapBuffer[x as i32 + y as i32 * 32 - 32] =
            (*crate::gl!(sTradeMenu)).tilemapBuffer[x as i32 + y as i32 * 32 - 33];
        (*crate::gl!(sTradeMenu)).tilemapBuffer[x as i32 + y as i32 * 32 - 31] =
            (*crate::gl!(sTradeMenu)).tilemapBuffer[x as i32 + y as i32 * 32 - 36] | 0x400;
    }
    if (*crate::gl!(sTradeMenu)).isEgg[whichParty][monIdx] != 0 {
        symbolTile = 0x480;
    } else {
        if whichParty == TRADE_PLAYER {
            gender = GetMonGender(crate::gl!(&raw gPlayerParty[monIdx]));
            GetMonData3(
                crate::gl!(&raw gPlayerParty[monIdx]),
                MON_DATA_NICKNAME,
                nickname.as_mut_ptr(),
            );
        } else {
            gender = GetMonGender(crate::gl!(&raw gEnemyParty[monIdx]));
            GetMonData3(
                crate::gl!(&raw gEnemyParty[monIdx]),
                MON_DATA_NICKNAME,
                nickname.as_mut_ptr(),
            );
        }
        match gender {
            MON_MALE => {
                symbolTile = (if NameHasGenderSymbol(nickname.as_mut_ptr(), MON_MALE) == 0 {
                    0x84
                } else {
                    0x83
                }) as u32;
            }
            MON_FEMALE => {
                symbolTile = (if NameHasGenderSymbol(nickname.as_mut_ptr(), MON_FEMALE) == 0 {
                    0x85
                } else {
                    0x83
                }) as u32;
            }
            _ => {
                symbolTile = 0x83;
            }
        }
    }
    (*crate::gl!(sTradeMenu)).tilemapBuffer[(y as i32 - 1) * 32 + x as i32 + 1] = symbolTile as u16;
}
unsafe fn PrintPartyLevelsAndGenders(whichParty: u8) {
    let mut i: i32 = 0;
    while i < (*crate::gl!(sTradeMenu)).partyCounts[whichParty] as i32 {
        let j: i32 = i + PARTY_SIZE * whichParty as i32;
        PrintLevelAndGender(
            whichParty,
            i as u8,
            sTradeMonLevelCoords[j][0],
            sTradeMonLevelCoords[j][1],
            sTradeMonBoxCoords[j][0],
            sTradeMonBoxCoords[j][1],
        );
        i += 1;
    }
}
unsafe fn ShowTradePartyMonIcons(whichParty: u8) {
    let mut i: i32 = 0;
    while i < (*crate::gl!(sTradeMenu)).partyCounts[whichParty] as i32 {
        (*gSprites.as_ptr())[(*crate::gl!(sTradeMenu)).partySpriteIds[whichParty][i]]
            .set_invisible(FALSE as u16);
        {
            let gl_i16 = (*crate::gl!(sTradeMenu)).partySpriteIds[whichParty][i];
            crate::gl!(
                gSprites[gl_i16].x =
                    sTradeMonSpriteCoords[whichParty as i32 * PARTY_SIZE + i][0] as i16 * 8 + 14
            )
        };
        {
            let gl_i17 = (*crate::gl!(sTradeMenu)).partySpriteIds[whichParty][i];
            crate::gl!(
                gSprites[gl_i17].y =
                    sTradeMonSpriteCoords[whichParty as i32 * PARTY_SIZE + i][1] as i16 * 8 - 12
            )
        };
        {
            let gl_i18 = (*crate::gl!(sTradeMenu)).partySpriteIds[whichParty][i];
            crate::gl!(gSprites[gl_i18].x2 = 0)
        };
        {
            let gl_i19 = (*crate::gl!(sTradeMenu)).partySpriteIds[whichParty][i];
            crate::gl!(gSprites[gl_i19].y2 = 0)
        };
        i += 1;
    }
}
unsafe fn PrintTradePartnerPartyNicknames() {
    rbox_fill_rectangle(1);
    PrintPartyNicknames(TRADE_PARTNER);
}
unsafe fn RedrawPartyWindow(whichParty: u8) {
    CopyToBgTilemapBufferRect_ChangePalette(
        1,
        sTradePartyBoxTilemap.as_ptr().cast_mut() as *mut c_void,
        whichParty * 15,
        0,
        15,
        17,
        0,
    );
    CopyBgTilemapBufferToVram(1);
    PrintPartyLevelsAndGenders(whichParty);
    PrintPartyNicknames(whichParty);
    ShowTradePartyMonIcons(whichParty);
    DrawBottomRowText(
        sActionTexts[1],
        (OBJ_VRAM0 + (*crate::gl!(sTradeMenu)).bottomTextTileStart as i32 * 32) as usize
            as *mut c_void as *mut u8,
        24,
    );
    (*crate::gl!(sTradeMenu)).drawSelectedMonState[whichParty] = 0;
}
pub(crate) unsafe fn Task_DrawSelectionSummary(taskId: u8) {
    FillBgTilemapBufferRect_Palette0(0, 0, 0, 0, DISPLAY_TILE_WIDTH, DISPLAY_TILE_HEIGHT);
    CopyBgTilemapBufferToVram(0);
}
pub(crate) unsafe fn Task_DrawSelectionTrade(taskId: u8) {
    FillBgTilemapBufferRect_Palette0(0, 0, 0, 0, DISPLAY_TILE_WIDTH, DISPLAY_TILE_HEIGHT);
    CopyBgTilemapBufferToVram(0);
}
unsafe fn QueueAction(delay: u16, actionId: u8) {
    for i in 0..4i32 {
        if (*crate::gl!(sTradeMenu)).queuedActions[i].active == 0 {
            (*crate::gl!(sTradeMenu)).queuedActions[i].delay = delay;
            (*crate::gl!(sTradeMenu)).queuedActions[i].actionId = actionId;
            (*crate::gl!(sTradeMenu)).queuedActions[i].active = TRUE;
            break;
        }
    }
}
unsafe fn GetNumQueuedActions() -> u32 {
    let mut numActions: u32 = 0;
    for i in 0..4i32 {
        numActions += (*crate::gl!(sTradeMenu)).queuedActions[i].active as u32;
    }
    numActions
}
unsafe fn DoQueuedActions() {
    for i in 0..4i32 {
        if (*crate::gl!(sTradeMenu)).queuedActions[i].active != 0 {
            if (*crate::gl!(sTradeMenu)).queuedActions[i].delay != 0 {
                (*crate::gl!(sTradeMenu)).queuedActions[i].delay -= 1;
            } else {
                match (*crate::gl!(sTradeMenu)).queuedActions[i].actionId {
                    QUEUE_SEND_DATA => {
                        SendLinkData(
                            (*crate::gl!(sTradeMenu)).linkData.as_mut_ptr() as *mut c_void,
                            20,
                        );
                    }
                    QUEUE_STANDBY => {
                        PrintTradeMessage(MSG_STANDBY);
                    }
                    QUEUE_ONLY_MON1 => {
                        PrintTradeMessage(MSG_ONLY_MON1);
                    }
                    QUEUE_ONLY_MON2 | QUEUE_UNUSED1 | QUEUE_UNUSED2 => {
                        PrintTradeMessage(MSG_ONLY_MON2);
                    }
                    QUEUE_MON_CANT_BE_TRADED => {
                        PrintTradeMessage(MSG_MON_CANT_BE_TRADED);
                    }
                    QUEUE_EGG_CANT_BE_TRADED => {
                        PrintTradeMessage(MSG_EGG_CANT_BE_TRADED);
                    }
                    QUEUE_FRIENDS_MON_CANT_BE_TRADED => {
                        PrintTradeMessage(MSG_FRIENDS_MON_CANT_BE_TRADED);
                    }
                    _ => {}
                }
                (*crate::gl!(sTradeMenu)).queuedActions[i].active = FALSE;
            }
        }
    }
}
unsafe fn PrintTradeMessage(messageId: u8) {
    FillWindowPixelBuffer(0, 17);
    AddTextPrinterParameterized(
        0,
        FONT_NORMAL,
        sMessages[messageId],
        0,
        1,
        TEXT_SKIP_DRAW,
        None,
    );
    DrawTextBorderOuter(0, 20, 12);
    PutWindowTilemap(0);
    CopyWindowToVram(0, COPYWIN_FULL);
}
unsafe fn LoadUISpriteGfx() -> u8 {
    let mut sheet: SpriteSheet = zeroed();
    if (*crate::gl!(sTradeMenu)).timer < NUM_MENU_TEXT_SPRITES {
        sheet.data =
            (*sMenuTextTileBuffers.as_ptr())[(*crate::gl!(sTradeMenu)).timer] as *mut c_void;
        sheet.size = 0x100;
        sheet.tag = GFXTAG_MENU_TEXT + (*crate::gl!(sTradeMenu)).timer as u16;
    }
    match (*crate::gl!(sTradeMenu)).timer {
        0
        | GFXTAG_PLAYER_NAME_M
        | GFXTAG_PLAYER_NAME_R
        | 3
        | GFXTAG_PARTNER_NAME_M
        | GFXTAG_PARTNER_NAME_R
        | 6
        | 7 => {
            LoadSpriteSheet(&raw mut sheet);
            (*crate::gl!(sTradeMenu)).timer += 1;
        }
        8 => {
            (*crate::gl!(sTradeMenu)).bottomTextTileStart = LoadSpriteSheet(&raw mut sheet);
            (*crate::gl!(sTradeMenu)).timer += 1;
        }
        GFXTAG_CHOOSE_PKMN_M
        | GFXTAG_CHOOSE_PKMN_R
        | GFXTAG_CHOOSE_PKMN_EMPTY_1
        | GFXTAG_CHOOSE_PKMN_EMPTY_2
        | GFXTAG_CHOOSE_PKMN_EMPTY_3 => {
            LoadSpriteSheet(&raw mut sheet);
            (*crate::gl!(sTradeMenu)).timer += 1;
        }
        NUM_MENU_TEXT_SPRITES => {
            LoadSpritePalette((&raw const *sSpritePalette_MenuText).cast_mut());
            (*crate::gl!(sTradeMenu)).timer += 1;
        }
        15 => {
            LoadSpritePalette((&raw const *sCursor_SpritePalette).cast_mut());
            (*crate::gl!(sTradeMenu)).timer += 1;
        }
        16 => {
            LoadSpriteSheet((&raw const *sCursor_SpriteSheet).cast_mut());
            (*crate::gl!(sTradeMenu)).timer += 1;
        }
        17 => {
            (*crate::gl!(sTradeMenu)).timer = 0;
            return TRUE;
        }
        _ => {}
    }
    FALSE
}
unsafe fn DrawBottomRowText(str: *mut u8, dest: *mut u8, unused: u8) {
    DrawTextWindowAndBufferTiles(str, dest as *mut c_void, 0, 0, 6);
}
unsafe fn ComputePartyTradeableFlags(whichParty: u8) {
    let mut i: i32 = 0;
    match whichParty {
        TRADE_PLAYER => {
            i = 0;
            while i < (*crate::gl!(sTradeMenu)).partyCounts[whichParty] as i32 {
                if GetMonData2(crate::gl!(&raw gPlayerParty[i]), MON_DATA_IS_EGG) == TRUE as u32 {
                    (*crate::gl!(sTradeMenu)).isLiveMon[whichParty][i] = FALSE;
                    (*crate::gl!(sTradeMenu)).isEgg[whichParty][i] = TRUE;
                } else if GetMonData2(crate::gl!(&raw gPlayerParty[i]), MON_DATA_HP) == 0 {
                    (*crate::gl!(sTradeMenu)).isLiveMon[whichParty][i] = FALSE;
                    (*crate::gl!(sTradeMenu)).isEgg[whichParty][i] = FALSE;
                } else {
                    (*crate::gl!(sTradeMenu)).isLiveMon[whichParty][i] = TRUE;
                    (*crate::gl!(sTradeMenu)).isEgg[whichParty][i] = FALSE;
                }
                i += 1;
            }
        }
        TRADE_PARTNER => {
            i = 0;
            while i < (*crate::gl!(sTradeMenu)).partyCounts[whichParty] as i32 {
                if GetMonData2(crate::gl!(&raw gEnemyParty[i]), MON_DATA_IS_EGG) == TRUE as u32 {
                    (*crate::gl!(sTradeMenu)).isLiveMon[whichParty][i] = FALSE;
                    (*crate::gl!(sTradeMenu)).isEgg[whichParty][i] = TRUE;
                } else if GetMonData2(crate::gl!(&raw gEnemyParty[i]), MON_DATA_HP) == 0 {
                    (*crate::gl!(sTradeMenu)).isLiveMon[whichParty][i] = FALSE;
                    (*crate::gl!(sTradeMenu)).isEgg[whichParty][i] = FALSE;
                } else {
                    (*crate::gl!(sTradeMenu)).isLiveMon[whichParty][i] = TRUE;
                    (*crate::gl!(sTradeMenu)).isEgg[whichParty][i] = FALSE;
                }
                i += 1;
            }
        }
        _ => {}
    }
}
unsafe fn ComputePartyHPBarLevels(whichParty: u8) {
    let mut i: u16 = 0;
    let mut curHp: u16 = 0;
    let mut maxHp: u16 = 0;
    match whichParty {
        TRADE_PLAYER => {
            i = 0;
            while i < (*crate::gl!(sTradeMenu)).partyCounts[0] as u16 {
                curHp = GetMonData2(crate::gl!(&raw gPlayerParty[i]), MON_DATA_HP) as u16;
                maxHp = GetMonData2(crate::gl!(&raw gPlayerParty[i]), MON_DATA_MAX_HP) as u16;
                (*crate::gl!(sTradeMenu)).hpBarLevels[0][i] =
                    GetHPBarLevel(curHp as i16, maxHp as i16);
                i += 1;
            }
        }
        TRADE_PARTNER => {
            i = 0;
            while i < (*crate::gl!(sTradeMenu)).partyCounts[1] as u16 {
                curHp = GetMonData2(crate::gl!(&raw gEnemyParty[i]), MON_DATA_HP) as u16;
                maxHp = GetMonData2(crate::gl!(&raw gEnemyParty[i]), MON_DATA_MAX_HP) as u16;
                (*crate::gl!(sTradeMenu)).hpBarLevels[1][i] =
                    GetHPBarLevel(curHp as i16, maxHp as i16);
                i += 1;
            }
        }
        _ => {}
    }
}
unsafe fn SetTradePartyHPBarSprites() {
    let mut j: i32 = 0;
    for i in 0..2i32 {
        j = 0;
        while j < (*crate::gl!(sTradeMenu)).partyCounts[i] as i32 {
            SetPartyHPBarSprite(
                {
                    let gl_i20 = (*crate::gl!(sTradeMenu)).partySpriteIds[i][j];
                    crate::gl!(&raw gSprites[gl_i20])
                },
                4 - (*crate::gl!(sTradeMenu)).hpBarLevels[i][j],
            );
            j += 1;
        }
    }
}
unsafe fn SaveTradeGiftRibbons() {
    for i in 0..11i32 {
        if (*crate::gl!(gSaveBlock1Ptr)).giftRibbons[i] == 0
            && (*crate::gl!(sTradeMenu)).giftRibbons[i] != 0
            && (*crate::gl!(sTradeMenu)).giftRibbons[i] < MAX_GIFT_RIBBON
        {
            (*crate::gl!(gSaveBlock1Ptr)).giftRibbons[i] = (*crate::gl!(sTradeMenu)).giftRibbons[i];
        }
    }
}
unsafe fn CanTradeSelectedMon(playerParty: *mut Pokemon, partyCount: i32, monIdx: i32) -> u32 {
    let mut species: CArray<u32, 6> = zeroed();
    let mut species2: CArray<u32, 6> = zeroed();
    for i in 0..partyCount {
        species2[i] = GetMonData2(playerParty.at(i), MON_DATA_SPECIES_OR_EGG);
        species[i] = GetMonData2(playerParty.at(i), MON_DATA_SPECIES);
    }
    if IsNationalPokedexEnabled() == 0 {
        if species2[monIdx] == SPECIES_EGG {
            return CANT_TRADE_EGG_YET as u32;
        }
        if IsSpeciesInHoennDex(species2[monIdx] as u16) == 0 {
            return CANT_TRADE_NATIONAL as u32;
        }
    }
    let partner: *mut LinkPlayer = {
        let gl_i21 = GetMultiplayerId() as i32 ^ 1;
        crate::gl!(&raw gLinkPlayers[gl_i21])
    };
    if (*partner).version as i32 & 0xFF != VERSION_RUBY
        && (*partner).version as i32 & 0xFF != VERSION_SAPPHIRE
        && (*partner).progressFlagsCopy as i32 & 0xF == 0
    {
        if species2[monIdx] == SPECIES_EGG {
            return CANT_TRADE_PARTNER_EGG_YET;
        }
        if IsSpeciesInHoennDex(species2[monIdx] as u16) == 0 {
            return CANT_TRADE_INVALID_MON;
        }
    }
    if (species[monIdx] == SPECIES_DEOXYS || species[monIdx] == SPECIES_MEW)
        && GetMonData2(playerParty.at(monIdx), MON_DATA_MODERN_FATEFUL_ENCOUNTER) == 0
    {
        return CANT_TRADE_INVALID_MON;
    }
    let mut i: i32 = 0;
    while i < partyCount {
        if species2[i] == SPECIES_EGG {
            species2[i] = SPECIES_NONE as u32;
        }
        i += 1;
    }
    let mut numMonsLeft: i32 = 0;
    for i in 0..partyCount {
        if i != monIdx {
            numMonsLeft += species2[i] as i32;
        }
    }
    if numMonsLeft != 0 {
        return CAN_TRADE_MON;
    } else {
        return CANT_TRADE_LAST_MON as u32;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
pub unsafe fn GetGameProgressForLinkTrade() -> i32 {
    let mut versionId: i32 = 0;
    let mut version: u16 = 0;
    if gReceivedRemoteLinkPlayers.get() != 0 {
        versionId = 0;
        version = ({
            let gl_i22 = GetMultiplayerId() as i32 ^ 1;
            crate::gl!(gLinkPlayers[gl_i22].version)
        }) & 0xFF;
        if version == VERSION_RUBY as u16
            || version == VERSION_SAPPHIRE as u16
            || version == VERSION_EMERALD as u16
        {
            versionId = 0;
        } else if version == VERSION_FIRE_RED as u16 || version == VERSION_LEAF_GREEN as u16 {
            versionId = 2;
        }
        if versionId > 0 {
            if ({
                let gl_i23 = GetMultiplayerId();
                crate::gl!(gLinkPlayers[gl_i23].progressFlagsCopy)
            }) as i32
                & 0xF0
                != 0
            {
                if versionId == 2 {
                    if ({
                        let gl_i24 = GetMultiplayerId() as i32 ^ 1;
                        crate::gl!(gLinkPlayers[gl_i24].progressFlagsCopy)
                    }) as i32
                        & 0xF0
                        != 0
                    {
                        return TRADE_BOTH_PLAYERS_READY;
                    } else {
                        return TRADE_PARTNER_NOT_READY;
                    }
                }
            } else {
                return TRADE_PLAYER_NOT_READY;
            }
        }
    }
    TRADE_BOTH_PLAYERS_READY
}
fn IsDeoxysOrMewUntradable(species: u16, isModernFatefulEncounter: u8) -> u32 {
    if (species == SPECIES_DEOXYS as u16 || species == SPECIES_MEW as u16)
        && isModernFatefulEncounter == 0
    {
        return TRUE as u32;
    }
    FALSE as u32
}
pub unsafe fn GetUnionRoomTradeMessageId(
    player: RfuGameCompatibilityData,
    partner: RfuGameCompatibilityData,
    playerSpecies2: u16,
    partnerSpecies: u16,
    requestedType: u8,
    playerSpecies: u16,
    isModernFatefulEncounter: u8,
) -> i32 {
    let playerHasNationalDex: u8 = player.hasNationalDex() as u8;
    let playerCanLinkNationally: u8 = player.canLinkNationally() as u8;
    let partnerHasNationalDex: u8 = partner.hasNationalDex() as u8;
    let partnerCanLinkNationally: u8 = partner.canLinkNationally() as u8;
    let partnerVersion: u8 = partner.version() as u8;
    if partnerVersion != VERSION_EMERALD {
        if playerCanLinkNationally == 0 {
            return UR_TRADE_MSG_CANT_TRADE_WITH_PARTNER_1;
        } else if partnerCanLinkNationally == 0 {
            return UR_TRADE_MSG_CANT_TRADE_WITH_PARTNER_2;
        }
    }
    if IsDeoxysOrMewUntradable(playerSpecies, isModernFatefulEncounter) != 0 {
        return UR_TRADE_MSG_MON_CANT_BE_TRADED_2;
    }
    if partnerSpecies == SPECIES_EGG as u16 {
        if playerSpecies2 != partnerSpecies {
            return UR_TRADE_MSG_NOT_EGG;
        }
    } else {
        if (*(&raw const crate::data::pokemon::gSpeciesInfo).cast::<CArray<SpeciesInfo, 0>>())
            [playerSpecies2]
            .types[0]
            != requestedType
            && (*(&raw const crate::data::pokemon::gSpeciesInfo).cast::<CArray<SpeciesInfo, 0>>())
                [playerSpecies2]
                .types[1]
                != requestedType
        {
            return UR_TRADE_MSG_NOT_MON_PARTNER_WANTS;
        }
    }
    if playerSpecies2 == SPECIES_EGG as u16 && playerSpecies2 != partnerSpecies {
        return UR_TRADE_MSG_MON_CANT_BE_TRADED_1;
    }
    if playerHasNationalDex == 0 {
        if playerSpecies2 == SPECIES_EGG as u16 {
            return UR_TRADE_MSG_EGG_CANT_BE_TRADED;
        }
        if IsSpeciesInHoennDex(playerSpecies2) == 0 {
            return UR_TRADE_MSG_MON_CANT_BE_TRADED_2;
        }
        if IsSpeciesInHoennDex(partnerSpecies) == 0 {
            return UR_TRADE_MSG_PARTNERS_MON_CANT_BE_TRADED;
        }
    }
    if partnerHasNationalDex == 0 && IsSpeciesInHoennDex(playerSpecies2) == 0 {
        return UR_TRADE_MSG_PARTNER_CANT_ACCEPT_MON;
    }
    UR_TRADE_MSG_NONE as i32
}
pub fn CanRegisterMonForTradingBoard(
    player: RfuGameCompatibilityData,
    species2: u16,
    species: u16,
    isModernFatefulEncounter: u8,
) -> i32 {
    let hasNationalDex: u8 = player.hasNationalDex() as u8;
    if IsDeoxysOrMewUntradable(species, isModernFatefulEncounter) != 0 {
        return CANT_REGISTER_MON;
    }
    if hasNationalDex != 0 {
        return CAN_REGISTER_MON;
    }
    if species2 == SPECIES_EGG as u16 {
        return CANT_REGISTER_EGG;
    }
    if IsSpeciesInHoennDex(species2) != 0 {
        return CAN_REGISTER_MON;
    }
    CANT_REGISTER_MON
}
pub unsafe fn CanSpinTradeMon(mon: *mut Pokemon, monIdx: u16) -> i32 {
    let mut version: i32 = 0;
    let mut speciesArray: CArray<i32, 6> = zeroed();
    let mut i: i32 = 0;
    while i < gPlayerPartyCount.get() as i32 {
        speciesArray[i] = GetMonData2(mon.at(i), MON_DATA_SPECIES_OR_EGG) as i32;
        if speciesArray[i] == SPECIES_EGG as i32 {
            speciesArray[i] = SPECIES_NONE as i32;
        }
        i += 1;
    }
    let mut versions: i32 = 0;
    let mut canTradeAnyMon: i32 = TRUE as i32;
    i = 0;
    while i < GetLinkPlayerCount() as i32 {
        version = crate::gl!(gLinkPlayers[i].version) as i32 & 0xFF;
        if version == VERSION_FIRE_RED || version == VERSION_LEAF_GREEN {
            versions = 0;
        } else {
            versions |= 1;
        }
        i += 1;
    }
    i = 0;
    while i < GetLinkPlayerCount() as i32 {
        let player: *mut LinkPlayer = crate::gl!(&raw gLinkPlayers[i]);
        if (*player).progressFlags as i32 & 0xF == 0 {
            canTradeAnyMon = FALSE as i32;
        }
        if versions != 0 && (*player).progressFlags as i32 / 16 != 0 {
            canTradeAnyMon = FALSE as i32;
        }
        i += 1;
    }
    if canTradeAnyMon == FALSE as i32 {
        if IsSpeciesInHoennDex(speciesArray[monIdx] as u16) == 0 {
            return CANT_TRADE_NATIONAL;
        }
        if speciesArray[monIdx] == SPECIES_NONE as i32 {
            return CANT_TRADE_EGG_YET;
        }
    }
    let mut numMonsLeft: i32 = 0;
    for i in 0..(gPlayerPartyCount.get() as i32) {
        if monIdx as i32 != i {
            numMonsLeft += speciesArray[i];
        }
    }
    if numMonsLeft == 0 {
        return CANT_TRADE_LAST_MON;
    } else {
        return CAN_TRADE_MON as i32;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
pub(crate) unsafe fn SpriteCB_LinkMonGlow(sprite: *mut Sprite) {
    if ({
        (*sprite).data[0] += 1;
        (*sprite).data[0]
    }) == 10
    {
        PlaySE(SE_BALL);
        (*sprite).data[0] = 0;
    }
}
pub(crate) unsafe fn SpriteCB_LinkMonGlowWireless(sprite: *mut Sprite) {
    if (*sprite).invisible() == 0
        && ({
            (*sprite).data[0] += 1;
            (*sprite).data[0]
        }) == 10
    {
        PlaySE(SE_M_SWAGGER2);
        (*sprite).data[0] = 0;
    }
}
pub(crate) unsafe fn SpriteCB_LinkMonShadow(sprite: *mut Sprite) {
    if (*sprite).data[1] == 0 {
        if ({
            (*sprite).data[0] += 1;
            (*sprite).data[0]
        }) == 12
        {
            (*sprite).data[0] = 0;
        }
        LoadPalette(
            (&raw const sLinkMonShadow_Pal[(*sprite).data[0]]).cast_mut() as *mut c_void,
            ((*sprite).oam.paletteNum() + 16) * 16 + 4,
            2,
        );
    }
}
pub(crate) unsafe fn SpriteCB_CableEndSending(sprite: *mut Sprite) {
    (*sprite).data[0] += 1;
    (*sprite).y2 += 1;
    if (*sprite).data[0] == 10 {
        DestroySprite(sprite);
    }
}
pub(crate) unsafe fn SpriteCB_CableEndReceiving(sprite: *mut Sprite) {
    (*sprite).data[0] += 1;
    (*sprite).y2 -= 1;
    if (*sprite).data[0] == 10 {
        DestroySprite(sprite);
    }
}
pub(crate) unsafe fn SpriteCB_GbaScreen(sprite: *mut Sprite) {
    if ({
        (*sprite).data[0] += 1;
        (*sprite).data[0]
    }) == 15
    {
        PlaySE(SE_M_MINIMIZE);
        (*sprite).data[0] = 0;
    }
}
unsafe fn SetTradeBGAffine() {
    let mut affine: BgAffineDstData = zeroed();
    DoBgAffineSet(
        &raw mut affine,
        (*crate::gl!(sTradeAnim)).texX as u32 * 0x100,
        (*crate::gl!(sTradeAnim)).texY as u32 * 0x100,
        (*crate::gl!(sTradeAnim)).scrX as i16,
        (*crate::gl!(sTradeAnim)).scrY as i16,
        (*crate::gl!(sTradeAnim)).sXY as i16,
        (*crate::gl!(sTradeAnim)).sXY as i16,
        (*crate::gl!(sTradeAnim)).alpha,
    );
    SetGpuReg(REG_OFFSET_BG2PA, affine.pa as u16);
    SetGpuReg(REG_OFFSET_BG2PB, affine.pb as u16);
    SetGpuReg(REG_OFFSET_BG2PC, affine.pc as u16);
    SetGpuReg(REG_OFFSET_BG2PD, affine.pd as u16);
    SetGpuReg(REG_OFFSET_BG2X_L, affine.dx as u16);
    SetGpuReg(REG_OFFSET_BG2X_H, (affine.dx >> 16) as u16);
    SetGpuReg(REG_OFFSET_BG2Y_L, affine.dy as u16);
    SetGpuReg(REG_OFFSET_BG2Y_H, (affine.dy >> 16) as u16);
}
unsafe fn SetTradeGpuRegs() {
    SetGpuReg(REG_OFFSET_BG1VOFS, (*crate::gl!(sTradeAnim)).bg1vofs as u16);
    SetGpuReg(REG_OFFSET_BG1HOFS, (*crate::gl!(sTradeAnim)).bg1hofs as u16);
    let dispcnt: u16 = GetGpuReg(REG_OFFSET_DISPCNT);
    if dispcnt as i32 & 7 == DISPCNT_MODE_0 {
        SetGpuReg(REG_OFFSET_BG2VOFS, (*crate::gl!(sTradeAnim)).bg2vofs as u16);
        SetGpuReg(REG_OFFSET_BG2HOFS, (*crate::gl!(sTradeAnim)).bg2hofs as u16);
    } else {
        SetTradeBGAffine();
    }
}
pub(crate) unsafe fn VBlankCB_TradeAnim() {
    SetTradeGpuRegs();
    LoadOam();
    ProcessSpriteCopyRequests();
    TransferPlttBuffer();
}
unsafe fn ClearLinkTimeoutTimer() {
    (*crate::gl!(sTradeAnim)).linkTimeoutTimer = 0;
    (*crate::gl!(sTradeAnim)).linkTimeoutZero1 = 0;
    (*crate::gl!(sTradeAnim)).linkTimeoutZero2 = 0;
}
unsafe fn CheckForLinkTimeout() {
    if (*crate::gl!(sTradeAnim)).linkTimeoutZero1 == (*crate::gl!(sTradeAnim)).linkTimeoutZero2 {
        (*crate::gl!(sTradeAnim)).linkTimeoutTimer += 1;
    } else {
        (*crate::gl!(sTradeAnim)).linkTimeoutTimer = 0;
    }
    if (*crate::gl!(sTradeAnim)).linkTimeoutTimer > 300 {
        CloseLink();
        SetMainCallback2(Some(CB2_LinkError));
        (*crate::gl!(sTradeAnim)).linkTimeoutTimer = 0;
        (*crate::gl!(sTradeAnim)).linkTimeoutZero2 = 0;
        (*crate::gl!(sTradeAnim)).linkTimeoutZero1 = 0;
    }
    (*crate::gl!(sTradeAnim)).linkTimeoutZero2 = (*crate::gl!(sTradeAnim)).linkTimeoutZero1;
}
unsafe fn TradeGetMultiplayerId() -> u32 {
    if gReceivedRemoteLinkPlayers.get() != 0 {
        return GetMultiplayerId() as u32;
    }
    0
}
unsafe fn LoadTradeMonPic(whichParty: u8, state: u8) {
    let mut pos: i32 = 0;
    let mut mon: *mut Pokemon = null_mut();
    let mut species: u16 = 0;
    let mut personality: u32 = 0;
    if whichParty == TRADE_PLAYER {
        mon = {
            let gl_i25 = crate::gl!(gSelectedTradeMonPositions[0]);
            crate::gl!(&raw gPlayerParty[gl_i25])
        };
        pos = B_POSITION_OPPONENT_LEFT as i32;
    }
    if whichParty == TRADE_PARTNER {
        mon = {
            let gl_i26 = crate::gl!(gSelectedTradeMonPositions[1]) as i32 % 6;
            crate::gl!(&raw gEnemyParty[gl_i26])
        };
        pos = B_POSITION_OPPONENT_RIGHT as i32;
    }
    match state {
        0 => {
            species = GetMonData2(mon, MON_DATA_SPECIES_OR_EGG) as u16;
            personality = GetMonData2(mon, MON_DATA_PERSONALITY);
            if whichParty == TRADE_PLAYER {
                HandleLoadSpecialPokePic_2(
                    (&raw const (*(&raw const crate::data::data_tables::gMonFrontPicTable)
                        .cast::<CArray<CompressedSpriteSheet, 0>>())[species])
                        .cast_mut(),
                    (*crate::gl!(gMonSpritesGfxPtr)).sprites.ptr[1],
                    species as i32,
                    personality,
                );
            } else {
                HandleLoadSpecialPokePic_DontHandleDeoxys(
                    (&raw const (*(&raw const crate::data::data_tables::gMonFrontPicTable)
                        .cast::<CArray<CompressedSpriteSheet, 0>>())[species])
                        .cast_mut(),
                    (*crate::gl!(gMonSpritesGfxPtr)).sprites.ptr
                        [whichParty as i32 * 2 + B_POSITION_OPPONENT_LEFT as i32],
                    species as i32,
                    personality,
                );
            }
            LoadCompressedSpritePalette(GetMonSpritePalStruct(mon));
            (*crate::gl!(sTradeAnim)).monSpecies[whichParty] = species;
            (*crate::gl!(sTradeAnim)).monPersonalities[whichParty] = personality;
        }
        1 => {
            SetMultiuseSpriteTemplateToPokemon((*GetMonSpritePalStruct(mon)).tag, pos as u8);
            (*crate::gl!(sTradeAnim)).monSpriteIds[whichParty] =
                CreateSprite(crate::gl!(&raw gMultiuseSpriteTemplate), 120, 60, 6);
            (*gSprites.as_ptr())[(*crate::gl!(sTradeAnim)).monSpriteIds[whichParty]]
                .set_invisible(TRUE as u16);
            {
                let gl_i27 = (*crate::gl!(sTradeAnim)).monSpriteIds[whichParty];
                crate::gl!(gSprites[gl_i27].callback = Some(SpriteCallbackDummy))
            };
        }
        _ => {}
    }
}
pub unsafe fn CB2_LinkTrade() {
    match crate::gl!(gMain.state) {
        0 => {
            if gReceivedRemoteLinkPlayers.get() == 0 {
                gLinkType.set(LINKTYPE_TRADE_DISCONNECTED);
                CloseLink();
            }
            crate::gl!(sTradeAnim = AllocZeroed(256) as *mut typeof___sTradeAnim_0_t);
            AllocateMonSpritesGfx();
            ResetTasks();
            ResetSpriteData();
            FreeAllSpritePalettes();
            SetVBlankCallback(Some(VBlankCB_TradeAnim));
            TradeAnimInit_LoadGfx();
            ClearLinkTimeoutTimer();
            crate::gl!(gMain.state += 1);
            (*crate::gl!(sTradeAnim)).neverRead_8C = 0;
            (*crate::gl!(sTradeAnim)).state = 0;
            (*crate::gl!(sTradeAnim)).isLinkTrade = TRUE;
            (*crate::gl!(sTradeAnim)).texX = 64;
            (*crate::gl!(sTradeAnim)).texY = 64;
            (*crate::gl!(sTradeAnim)).neverRead_D8 = 0;
            (*crate::gl!(sTradeAnim)).neverRead_DA = 0;
            (*crate::gl!(sTradeAnim)).scrX = 120;
            (*crate::gl!(sTradeAnim)).scrY = 80;
            (*crate::gl!(sTradeAnim)).sXY = 256;
            (*crate::gl!(sTradeAnim)).alpha = 0;
        }
        1 => {
            if gReceivedRemoteLinkPlayers.get() == 0 {
                (*crate::gl!(sTradeAnim)).isCableTrade = TRUE;
                OpenLink();
                crate::gl!(gMain.state += 1);
                (*crate::gl!(sTradeAnim)).timer = 0;
            } else {
                crate::gl!(gMain.state = 4);
            }
        }
        2 => {
            if ({
                (*crate::gl!(sTradeAnim)).timer += 1;
                (*crate::gl!(sTradeAnim)).timer
            }) > 60
            {
                (*crate::gl!(sTradeAnim)).timer = 0;
                crate::gl!(gMain.state += 1);
            }
        }
        3 => {
            if IsLinkMaster() != 0 {
                if GetLinkPlayerCount_2() >= GetSavedPlayerCount() {
                    if ({
                        (*crate::gl!(sTradeAnim)).timer += 1;
                        (*crate::gl!(sTradeAnim)).timer
                    }) > 30
                    {
                        CheckShouldAdvanceLinkState();
                        crate::gl!(gMain.state += 1);
                    }
                } else {
                    CheckForLinkTimeout();
                }
            } else {
                crate::gl!(gMain.state += 1);
            }
        }
        4 => {
            CheckForLinkTimeout();
            if gReceivedRemoteLinkPlayers.get() == TRUE
                && IsLinkPlayerDataExchangeComplete() == TRUE
            {
                crate::gl!(gMain.state += 1);
            }
        }
        5 => {
            (*crate::gl!(sTradeAnim)).playerFinishStatus = 0;
            (*crate::gl!(sTradeAnim)).partnerFinishStatus = 0;
            (*crate::gl!(sTradeAnim)).scheduleLinkTransfer = 0;
            LoadTradeMonPic(0, 0);
            crate::gl!(gMain.state += 1);
        }
        6 => {
            LoadTradeMonPic(TRADE_PLAYER, 1);
            crate::gl!(gMain.state += 1);
        }
        7 => {
            LoadTradeMonPic(TRADE_PARTNER, 0);
            crate::gl!(gMain.state += 1);
        }
        8 => {
            LoadTradeMonPic(1, 1);
            LinkTradeDrawWindow();
            crate::gl!(gMain.state += 1);
        }
        9 => {
            LoadTradeSequenceSpriteSheetsAndPalettes();
            LoadSpriteSheet((&raw const *sPokeBallSpriteSheet).cast_mut());
            LoadSpritePalette((&raw const *sPokeBallSpritePalette).cast_mut());
            crate::gl!(gMain.state += 1);
        }
        10 => {
            BeginNormalPaletteFade(PALETTES_ALL, 0, 16, 0, 0);
            ShowBg(0);
            crate::gl!(gMain.state += 1);
        }
        11 => {
            InitTradeSequenceBgGpuRegs();
            BufferTradeSceneStrings();
            crate::gl!(gMain.state += 1);
        }
        12 if (*gPaletteFade.as_ptr()).active() == 0 => {
            if gWirelessCommType.get() != 0 {
                LoadWirelessStatusIndicatorSpriteGfx();
                CreateWirelessStatusIndicatorSprite(0, 0);
            }
            SetMainCallback2(Some(CB2_UpdateLinkTrade));
        }
        _ => {}
    }
    RunTasks();
    RunTextPrinters();
    AnimateSprites();
    BuildOamBuffer();
    UpdatePaletteFade();
}
pub unsafe fn InitTradeSequenceBgGpuRegs() {
    SetTradeSequenceBgGpuRegs(5);
    SetTradeSequenceBgGpuRegs(0);
}
pub unsafe fn LinkTradeDrawWindow() {
    FillWindowPixelBuffer(0, 255);
    PutWindowTilemap(0);
    CopyWindowToVram(0, COPYWIN_FULL);
}
unsafe fn TradeAnimInit_LoadGfx() {
    SetGpuReg(0x0, 0);
    ResetBgsAndClearDma3BusyFlags(0);
    InitBgsFromTemplates(0, sTradeSequenceBgTemplates.as_ptr().cast_mut(), 4);
    ChangeBgX(0, 0, BG_COORD_SET);
    ChangeBgY(0, 0, BG_COORD_SET);
    SetBgTilemapBuffer(0, Alloc(BG_SCREEN_SIZE));
    SetBgTilemapBuffer(1, Alloc(BG_SCREEN_SIZE));
    SetBgTilemapBuffer(3, Alloc(BG_SCREEN_SIZE));
    DeactivateAllTextPrinters();
    DecompressAndLoadBgGfxUsingHeap(
        0,
        (*(&raw const crate::data::graphics::gBattleTextboxTiles).cast::<CArray<u32, 0>>())
            .as_ptr()
            .cast_mut() as *mut c_void,
        0,
        0,
        0,
    );
    LZDecompressWram(
        (*(&raw const crate::data::graphics::gBattleTextboxTilemap).cast::<CArray<u32, 0>>())
            .as_ptr()
            .cast_mut(),
        (*(crate::gl!(&raw @at crate::decompress::gDecompressionBuffer;).cast_const())
            .cast::<CArray<u8, 16384>>()
            .cast_mut())
        .as_mut_ptr() as *mut c_void,
    );
    CopyToBgTilemapBuffer(
        0,
        (*(crate::gl!(&raw @at crate::decompress::gDecompressionBuffer;).cast_const())
            .cast::<CArray<u8, 16384>>()
            .cast_mut())
        .as_mut_ptr() as *mut c_void,
        BG_SCREEN_SIZE as u16,
        0,
    );
    LoadCompressedPalette(
        (*(&raw const crate::data::graphics::gBattleTextboxPalette).cast::<CArray<u32, 0>>())
            .as_ptr()
            .cast_mut(),
        0,
        32,
    );
    InitWindows(sTradeSequenceWindowTemplates.as_ptr().cast_mut());
    DecompressAndLoadBgGfxUsingHeap(
        0,
        (*(&raw const crate::data::graphics::gBattleTextboxTiles).cast::<CArray<u32, 0>>())
            .as_ptr()
            .cast_mut() as *mut c_void,
        0,
        0,
        0,
    );
    LZDecompressWram(
        (*(&raw const crate::data::graphics::gBattleTextboxTilemap).cast::<CArray<u32, 0>>())
            .as_ptr()
            .cast_mut(),
        (*(crate::gl!(&raw @at crate::decompress::gDecompressionBuffer;).cast_const())
            .cast::<CArray<u8, 16384>>()
            .cast_mut())
        .as_mut_ptr() as *mut c_void,
    );
    CopyToBgTilemapBuffer(
        0,
        (*(crate::gl!(&raw @at crate::decompress::gDecompressionBuffer;).cast_const())
            .cast::<CArray<u8, 16384>>()
            .cast_mut())
        .as_mut_ptr() as *mut c_void,
        BG_SCREEN_SIZE as u16,
        0,
    );
    LoadCompressedPalette(
        (*(&raw const crate::data::graphics::gBattleTextboxPalette).cast::<CArray<u32, 0>>())
            .as_ptr()
            .cast_mut(),
        0,
        32,
    );
}
pub(crate) unsafe fn CB2_InitInGameTrade() {
    let mut otName: CArray<u8, 11> = zeroed();
    match crate::gl!(gMain.state) {
        0 => {
            crate::gl!(gSelectedTradeMonPositions[0] = gSpecialVar_0x8005 as u8);
            crate::gl!(gSelectedTradeMonPositions[1] = PARTY_SIZE as u8);
            StringCopy(
                (*gLinkPlayers.as_ptr())[0].name.as_mut_ptr(),
                (*crate::gl!(gSaveBlock2Ptr)).playerName.as_mut_ptr(),
            );
            GetMonData3(
                crate::gl!(&raw gEnemyParty[0]),
                MON_DATA_OT_NAME,
                otName.as_mut_ptr(),
            );
            StringCopy(
                (*gLinkPlayers.as_ptr())[1].name.as_mut_ptr(),
                otName.as_mut_ptr(),
            );
            crate::gl!(gLinkPlayers[0].language = GAME_LANGUAGE as u16);
            crate::gl!(
                gLinkPlayers[1].language =
                    GetMonData2(crate::gl!(&raw gEnemyParty[0]), MON_DATA_LANGUAGE) as u16
            );
            crate::gl!(sTradeAnim = AllocZeroed(256) as *mut typeof___sTradeAnim_0_t);
            AllocateMonSpritesGfx();
            ResetTasks();
            ResetSpriteData();
            FreeAllSpritePalettes();
            SetVBlankCallback(Some(VBlankCB_TradeAnim));
            TradeAnimInit_LoadGfx();
            (*crate::gl!(sTradeAnim)).isLinkTrade = FALSE;
            (*crate::gl!(sTradeAnim)).neverRead_8C = 0;
            (*crate::gl!(sTradeAnim)).state = 0;
            (*crate::gl!(sTradeAnim)).texX = 64;
            (*crate::gl!(sTradeAnim)).texY = 64;
            (*crate::gl!(sTradeAnim)).neverRead_D8 = 0;
            (*crate::gl!(sTradeAnim)).neverRead_DA = 0;
            (*crate::gl!(sTradeAnim)).scrX = 120;
            (*crate::gl!(sTradeAnim)).scrY = 80;
            (*crate::gl!(sTradeAnim)).sXY = 256;
            (*crate::gl!(sTradeAnim)).alpha = 0;
            (*crate::gl!(sTradeAnim)).timer = 0;
            crate::gl!(gMain.state = 5);
        }
        5 => {
            LoadTradeMonPic(0, 0);
            crate::gl!(gMain.state += 1);
        }
        6 => {
            LoadTradeMonPic(TRADE_PLAYER, 1);
            crate::gl!(gMain.state += 1);
        }
        7 => {
            LoadTradeMonPic(TRADE_PARTNER, 0);
            ShowBg(0);
            crate::gl!(gMain.state += 1);
        }
        8 => {
            LoadTradeMonPic(1, 1);
            FillWindowPixelBuffer(0, 255);
            PutWindowTilemap(0);
            CopyWindowToVram(0, COPYWIN_FULL);
            crate::gl!(gMain.state += 1);
        }
        9 => {
            LoadTradeSequenceSpriteSheetsAndPalettes();
            LoadSpriteSheet((&raw const *sPokeBallSpriteSheet).cast_mut());
            LoadSpritePalette((&raw const *sPokeBallSpritePalette).cast_mut());
            crate::gl!(gMain.state += 1);
        }
        10 => {
            ShowBg(0);
            crate::gl!(gMain.state += 1);
        }
        11 => {
            SetTradeSequenceBgGpuRegs(5);
            SetTradeSequenceBgGpuRegs(0);
            BufferTradeSceneStrings();
            crate::gl!(gMain.state += 1);
        }
        12 => {
            SetMainCallback2(Some(CB2_InGameTrade));
        }
        _ => {}
    }
    RunTasks();
    RunTextPrinters();
    AnimateSprites();
    BuildOamBuffer();
    UpdatePaletteFade();
}
unsafe fn UpdatePokedexForReceivedMon(partyIdx: u8) {
    let mon: *mut Pokemon = crate::gl!(&raw gPlayerParty[partyIdx]);
    if GetMonData2(mon, MON_DATA_IS_EGG) == 0 {
        let mut species: u16 = GetMonData3(mon, MON_DATA_SPECIES, null_mut()) as u16;
        let personality: u32 = GetMonData3(mon, MON_DATA_PERSONALITY, null_mut());
        species = SpeciesToNationalPokedexNum(species);
        GetSetPokedexFlag(species, FLAG_SET_SEEN);
        HandleSetPokedexFlag(species, FLAG_SET_CAUGHT, personality);
    }
}
unsafe fn TryEnableNationalDexFromLinkPartner() {
    let mpId: u8 = GetMultiplayerId();
}
unsafe fn TradeMons(playerPartyIdx: u8, partnerPartyIdx: u8) {
    let playerMon: *mut Pokemon = crate::gl!(&raw gPlayerParty[playerPartyIdx]);
    let playerMail: u16 = GetMonData2(playerMon, MON_DATA_MAIL) as u16;
    let partnerMon: *mut Pokemon = crate::gl!(&raw gEnemyParty[partnerPartyIdx]);
    let partnerMail: u16 = GetMonData2(partnerMon, MON_DATA_MAIL) as u16;
    if playerMail != MAIL_NONE as u16 {
        ClearMail(&raw mut (*crate::gl!(gSaveBlock1Ptr)).mail[playerMail]);
    }
    (*crate::gl!(sTradeAnim)).tempMon = *playerMon;
    *playerMon = *partnerMon;
    *partnerMon = (*crate::gl!(sTradeAnim)).tempMon;
    let mut friendship: u8 = 70;
    if GetMonData2(playerMon, MON_DATA_IS_EGG) == 0 {
        SetMonData(
            playerMon,
            MON_DATA_FRIENDSHIP,
            &raw mut friendship as *mut c_void,
        );
    }
    if partnerMail != MAIL_NONE as u16 {
        GiveMailToMon(playerMon, crate::gl!(&raw gTradeMail[partnerMail]));
    }
    UpdatePokedexForReceivedMon(playerPartyIdx);
    if gReceivedRemoteLinkPlayers.get() != 0 {
        TryEnableNationalDexFromLinkPartner();
    }
}
unsafe fn HandleLinkDataSend() {
    'l1: {
        let sw1: u8 = (*crate::gl!(sTradeAnim)).scheduleLinkTransfer;
        let mut fall = false;
        if sw1 == 1 {
            fall = true;
            if IsLinkTaskFinished() != 0 {
                SendBlock(
                    BitmaskAllOtherLinkPlayers(),
                    (*crate::gl!(sTradeAnim)).linkData.as_mut_ptr() as *mut c_void,
                    20,
                );
                (*crate::gl!(sTradeAnim)).scheduleLinkTransfer += 1;
            }
        }
        if fall || sw1 == 2 {
            (*crate::gl!(sTradeAnim)).scheduleLinkTransfer = 0;
            break 'l1;
        }
    }
}
pub(crate) unsafe fn CB2_InGameTrade() {
    DoTradeAnim();
    RunTasks();
    RunTextPrinters();
    AnimateSprites();
    BuildOamBuffer();
    UpdatePaletteFade();
}
unsafe fn SetTradeSequenceBgGpuRegs(state: u8) {
    match state {
        0 => {
            (*crate::gl!(sTradeAnim)).bg2vofs = 0;
            (*crate::gl!(sTradeAnim)).bg2hofs = 180;
            SetGpuReg(0x0, 5440);
            SetGpuReg(REG_OFFSET_BG2CNT, 20998);
            LoadPalette(
                (*(&raw const crate::data::graphics::gTradeGba2_Pal).cast::<CArray<u16, 0>>())
                    .as_ptr()
                    .cast_mut() as *mut c_void,
                16,
                96,
            );
            {
                let mut _src: *mut c_void = (*(&raw const crate::data::graphics::gTradeGba_Gfx)
                    .cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut() as *mut c_void;
                let mut _dest: *mut c_void = 0x6004000_usize as *mut c_void;
                let mut _size: u32 = 0x1420;
                loop {
                    if _size <= 0x1000 {
                        {
                            {
                                {
                                    let dmaRegs: *mut u32 = 67109076_usize as *mut u32;
                                    volatile_write(dmaRegs, _src as usize as u32);
                                    volatile_write(dmaRegs.at(1), _dest as usize as u32);
                                    volatile_write(dmaRegs.at(2), 0x80000000 | (_size / 2));
                                    let _ = (dmaRegs.at(2)).read_volatile();
                                }
                            }
                        }
                        break;
                    }
                    {
                        {
                            {
                                let dmaRegs: *mut u32 = 67109076_usize as *mut u32;
                                volatile_write(dmaRegs, _src as usize as u32);
                                volatile_write(dmaRegs.at(1), _dest as usize as u32);
                                volatile_write(dmaRegs.at(2), 0x80000800);
                                let _ = (dmaRegs.at(2)).read_volatile();
                            }
                        }
                    }
                    _src = (_src as *mut u8).at(4096) as *mut c_void;
                    _dest = (_dest as *mut u8).at(4096) as *mut c_void;
                    _size -= 0x1000;
                }
            }
            {
                let mut _src: *mut c_void =
                    gTradePlatform_Tilemap.as_ptr().cast_mut() as *mut c_void;
                let mut _dest: *mut c_void = 0x6009000_usize as *mut c_void;
                let mut _size: u32 = 0x1000;
                {
                    {
                        {
                            let dmaRegs: *mut u32 = 67109076_usize as *mut u32;
                            volatile_write(dmaRegs, _src as usize as u32);
                            volatile_write(dmaRegs.at(1), _dest as usize as u32);
                            volatile_write(dmaRegs.at(2), 0x80000000 | (_size / 2));
                            let _ = (dmaRegs.at(2)).read_volatile();
                        }
                    }
                }
            }
        }
        1 => {
            (*crate::gl!(sTradeAnim)).bg1hofs = 0;
            (*crate::gl!(sTradeAnim)).bg1vofs = 348;
            SetGpuReg(REG_OFFSET_BG1VOFS, 348);
            SetGpuReg(REG_OFFSET_BG1CNT, 34050);
            SetGpuReg(REG_OFFSET_BG2CNT, 37382);
            if (*crate::gl!(sTradeAnim)).isCableTrade != 0 {
                {
                    let mut _src: *mut c_void = sGbaMapCable.as_ptr().cast_mut() as *mut c_void;
                    let mut _dest: *mut c_void = 0x6002800_usize as *mut c_void;
                    let mut _size: u32 = 0x1000;
                    {
                        {
                            {
                                let dmaRegs: *mut u32 = 67109076_usize as *mut u32;
                                volatile_write(dmaRegs, _src as usize as u32);
                                volatile_write(dmaRegs.at(1), _dest as usize as u32);
                                volatile_write(dmaRegs.at(2), 0x80000000 | (_size / 2));
                                let _ = (dmaRegs.at(2)).read_volatile();
                            }
                        }
                    }
                }
            } else {
                {
                    let mut _src: *mut c_void = sGbaMapWireless.as_ptr().cast_mut() as *mut c_void;
                    let mut _dest: *mut c_void = 0x6002800_usize as *mut c_void;
                    let mut _size: u32 = 0x1000;
                    {
                        {
                            {
                                let dmaRegs: *mut u32 = 67109076_usize as *mut u32;
                                volatile_write(dmaRegs, _src as usize as u32);
                                volatile_write(dmaRegs.at(1), _dest as usize as u32);
                                volatile_write(dmaRegs.at(2), 0x80000000 | (_size / 2));
                                let _ = (dmaRegs.at(2)).read_volatile();
                            }
                        }
                    }
                }
            }
            {
                let mut _src: *mut c_void = (*(&raw const crate::data::graphics::gTradeGba_Gfx)
                    .cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut() as *mut c_void;
                let mut _dest: *mut c_void = 0x6000000_usize as *mut c_void;
                let mut _size: u32 = 0x1420;
                loop {
                    if _size <= 0x1000 {
                        {
                            {
                                {
                                    let dmaRegs: *mut u32 = 67109076_usize as *mut u32;
                                    volatile_write(dmaRegs, _src as usize as u32);
                                    volatile_write(dmaRegs.at(1), _dest as usize as u32);
                                    volatile_write(dmaRegs.at(2), 0x80000000 | (_size / 2));
                                    let _ = (dmaRegs.at(2)).read_volatile();
                                }
                            }
                        }
                        break;
                    }
                    {
                        {
                            {
                                let dmaRegs: *mut u32 = 67109076_usize as *mut u32;
                                volatile_write(dmaRegs, _src as usize as u32);
                                volatile_write(dmaRegs.at(1), _dest as usize as u32);
                                volatile_write(dmaRegs.at(2), 0x80000800);
                                let _ = (dmaRegs.at(2)).read_volatile();
                            }
                        }
                    }
                    _src = (_src as *mut u8).at(4096) as *mut c_void;
                    _dest = (_dest as *mut u8).at(4096) as *mut c_void;
                    _size -= 0x1000;
                }
            }
            SetGpuReg(0x0, 4672);
        }
        2 => {
            (*crate::gl!(sTradeAnim)).bg1vofs = 0;
            (*crate::gl!(sTradeAnim)).bg1hofs = 0;
            if (*crate::gl!(sTradeAnim)).isCableTrade == 0 {
                SetGpuReg(REG_OFFSET_DISPCNT, 4673);
                LZ77UnCompVram(
                    sWirelessCloseup_Map.as_ptr().cast_mut(),
                    0x6002800_usize as *mut c_void,
                );
                BlendPalettes(0x8, 16, 0);
            } else {
                SetGpuReg(REG_OFFSET_DISPCNT, 4673);
                {
                    let mut _src: *mut c_void =
                        sCableCloseup_Map.as_ptr().cast_mut() as *mut c_void;
                    let mut _dest: *mut c_void = 0x6002800_usize as *mut c_void;
                    let mut _size: u32 = 0x800;
                    {
                        {
                            {
                                let dmaRegs: *mut u32 = 67109076_usize as *mut u32;
                                volatile_write(dmaRegs, _src as usize as u32);
                                volatile_write(dmaRegs.at(1), _dest as usize as u32);
                                volatile_write(dmaRegs.at(2), 0x80000000 | (_size / 2));
                                let _ = (dmaRegs.at(2)).read_volatile();
                            }
                        }
                    }
                }
                BlendPalettes(0x1, 16, 0);
            }
        }
        3 => {
            LoadPalette(
                sWirelessSignalNone_Pal.as_ptr().cast_mut() as *mut c_void,
                48,
                32,
            );
            LZ77UnCompVram(
                sWirelessSignal_Gfx.as_ptr().cast_mut(),
                0x6004000_usize as *mut c_void,
            );
            LZ77UnCompVram(
                sWirelessSignal_Tilemap.as_ptr().cast_mut(),
                0x6009000_usize as *mut c_void,
            );
            (*crate::gl!(sTradeAnim)).bg2vofs = 80;
            SetGpuReg(0x0, 5696);
        }
        4 => {
            SetGpuReg(REG_OFFSET_DISPCNT, 5185);
            SetGpuReg(REG_OFFSET_BG2CNT, 4743);
            (*crate::gl!(sTradeAnim)).texX = 64;
            (*crate::gl!(sTradeAnim)).texY = 92;
            (*crate::gl!(sTradeAnim)).sXY = 32;
            (*crate::gl!(sTradeAnim)).gbaScale = 1024;
            (*crate::gl!(sTradeAnim)).alpha = 0;
            {
                let mut _src: *mut c_void = sGbaAffine_Gfx.as_ptr().cast_mut() as *mut c_void;
                let mut _dest: *mut c_void = 0x6004000_usize as *mut c_void;
                let mut _size: u32 = 0x2840;
                loop {
                    if _size <= 0x1000 {
                        {
                            {
                                {
                                    let dmaRegs: *mut u32 = 67109076_usize as *mut u32;
                                    volatile_write(dmaRegs, _src as usize as u32);
                                    volatile_write(dmaRegs.at(1), _dest as usize as u32);
                                    volatile_write(dmaRegs.at(2), 0x80000000 | (_size / 2));
                                    let _ = (dmaRegs.at(2)).read_volatile();
                                }
                            }
                        }
                        break;
                    }
                    {
                        {
                            {
                                let dmaRegs: *mut u32 = 67109076_usize as *mut u32;
                                volatile_write(dmaRegs, _src as usize as u32);
                                volatile_write(dmaRegs.at(1), _dest as usize as u32);
                                volatile_write(dmaRegs.at(2), 0x80000800);
                                let _ = (dmaRegs.at(2)).read_volatile();
                            }
                        }
                    }
                    _src = (_src as *mut u8).at(4096) as *mut c_void;
                    _dest = (_dest as *mut u8).at(4096) as *mut c_void;
                    _size -= 0x1000;
                }
            }
            if (*crate::gl!(sTradeAnim)).isCableTrade != 0 {
                {
                    let mut _src: *mut c_void =
                        sGbaAffineMapCable.as_ptr().cast_mut() as *mut c_void;
                    let mut _dest: *mut c_void = 0x6009000_usize as *mut c_void;
                    let mut _size: u32 = 0x100;
                    {
                        {
                            {
                                let dmaRegs: *mut u32 = 67109076_usize as *mut u32;
                                volatile_write(dmaRegs, _src as usize as u32);
                                volatile_write(dmaRegs.at(1), _dest as usize as u32);
                                volatile_write(dmaRegs.at(2), 0x80000000 | (_size / 2));
                                let _ = (dmaRegs.at(2)).read_volatile();
                            }
                        }
                    }
                }
            } else {
                {
                    let mut _src: *mut c_void =
                        sGbaAffineMapWireless.as_ptr().cast_mut() as *mut c_void;
                    let mut _dest: *mut c_void = 0x6009000_usize as *mut c_void;
                    let mut _size: u32 = 0x100;
                    {
                        {
                            {
                                let dmaRegs: *mut u32 = 67109076_usize as *mut u32;
                                volatile_write(dmaRegs, _src as usize as u32);
                                volatile_write(dmaRegs.at(1), _dest as usize as u32);
                                volatile_write(dmaRegs.at(2), 0x80000000 | (_size / 2));
                                let _ = (dmaRegs.at(2)).read_volatile();
                            }
                        }
                    }
                }
            }
        }
        5 => {
            (*crate::gl!(sTradeAnim)).bg1vofs = 0;
            (*crate::gl!(sTradeAnim)).bg1hofs = 0;
        }
        6 => {
            SetGpuReg(REG_OFFSET_DISPCNT, 5185);
            SetGpuReg(REG_OFFSET_BG2CNT, 4743);
            (*crate::gl!(sTradeAnim)).texX = 64;
            (*crate::gl!(sTradeAnim)).texY = 92;
            (*crate::gl!(sTradeAnim)).sXY = 256;
            (*crate::gl!(sTradeAnim)).gbaScale = 128;
            (*crate::gl!(sTradeAnim)).scrX = 120;
            (*crate::gl!(sTradeAnim)).scrY = 80;
            (*crate::gl!(sTradeAnim)).alpha = 0;
            {
                let mut _src: *mut c_void = sGbaAffine_Gfx.as_ptr().cast_mut() as *mut c_void;
                let mut _dest: *mut c_void = 0x6004000_usize as *mut c_void;
                let mut _size: u32 = 0x2840;
                loop {
                    if _size <= 0x1000 {
                        {
                            {
                                {
                                    let dmaRegs: *mut u32 = 67109076_usize as *mut u32;
                                    volatile_write(dmaRegs, _src as usize as u32);
                                    volatile_write(dmaRegs.at(1), _dest as usize as u32);
                                    volatile_write(dmaRegs.at(2), 0x80000000 | (_size / 2));
                                    let _ = (dmaRegs.at(2)).read_volatile();
                                }
                            }
                        }
                        break;
                    }
                    {
                        {
                            {
                                let dmaRegs: *mut u32 = 67109076_usize as *mut u32;
                                volatile_write(dmaRegs, _src as usize as u32);
                                volatile_write(dmaRegs.at(1), _dest as usize as u32);
                                volatile_write(dmaRegs.at(2), 0x80000800);
                                let _ = (dmaRegs.at(2)).read_volatile();
                            }
                        }
                    }
                    _src = (_src as *mut u8).at(4096) as *mut c_void;
                    _dest = (_dest as *mut u8).at(4096) as *mut c_void;
                    _size -= 0x1000;
                }
            }
            if (*crate::gl!(sTradeAnim)).isCableTrade != 0 {
                {
                    let mut _src: *mut c_void =
                        sGbaAffineMapCable.as_ptr().cast_mut() as *mut c_void;
                    let mut _dest: *mut c_void = 0x6009000_usize as *mut c_void;
                    let mut _size: u32 = 0x100;
                    {
                        {
                            {
                                let dmaRegs: *mut u32 = 67109076_usize as *mut u32;
                                volatile_write(dmaRegs, _src as usize as u32);
                                volatile_write(dmaRegs.at(1), _dest as usize as u32);
                                volatile_write(dmaRegs.at(2), 0x80000000 | (_size / 2));
                                let _ = (dmaRegs.at(2)).read_volatile();
                            }
                        }
                    }
                }
            } else {
                {
                    let mut _src: *mut c_void =
                        sGbaAffineMapWireless.as_ptr().cast_mut() as *mut c_void;
                    let mut _dest: *mut c_void = 0x6009000_usize as *mut c_void;
                    let mut _size: u32 = 0x100;
                    {
                        {
                            {
                                let dmaRegs: *mut u32 = 67109076_usize as *mut u32;
                                volatile_write(dmaRegs, _src as usize as u32);
                                volatile_write(dmaRegs.at(1), _dest as usize as u32);
                                volatile_write(dmaRegs.at(2), 0x80000000 | (_size / 2));
                                let _ = (dmaRegs.at(2)).read_volatile();
                            }
                        }
                    }
                }
            }
        }
        7 => {
            (*crate::gl!(sTradeAnim)).bg2vofs = 0;
            (*crate::gl!(sTradeAnim)).bg2hofs = 0;
            SetGpuReg(REG_OFFSET_BLDCNT, 0);
            SetGpuReg(REG_OFFSET_BG2CNT, 20998);
            LoadPalette(
                (*(&raw const crate::data::graphics::gTradeGba2_Pal).cast::<CArray<u16, 0>>())
                    .as_ptr()
                    .cast_mut() as *mut c_void,
                16,
                96,
            );
            {
                let mut _src: *mut c_void = (*(&raw const crate::data::graphics::gTradeGba_Gfx)
                    .cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut() as *mut c_void;
                let mut _dest: *mut c_void = 0x6004000_usize as *mut c_void;
                let mut _size: u32 = 0x1420;
                loop {
                    if _size <= 0x1000 {
                        {
                            {
                                {
                                    let dmaRegs: *mut u32 = 67109076_usize as *mut u32;
                                    volatile_write(dmaRegs, _src as usize as u32);
                                    volatile_write(dmaRegs.at(1), _dest as usize as u32);
                                    volatile_write(dmaRegs.at(2), 0x80000000 | (_size / 2));
                                    let _ = (dmaRegs.at(2)).read_volatile();
                                }
                            }
                        }
                        break;
                    }
                    {
                        {
                            {
                                let dmaRegs: *mut u32 = 67109076_usize as *mut u32;
                                volatile_write(dmaRegs, _src as usize as u32);
                                volatile_write(dmaRegs.at(1), _dest as usize as u32);
                                volatile_write(dmaRegs.at(2), 0x80000800);
                                let _ = (dmaRegs.at(2)).read_volatile();
                            }
                        }
                    }
                    _src = (_src as *mut u8).at(4096) as *mut c_void;
                    _dest = (_dest as *mut u8).at(4096) as *mut c_void;
                    _size -= 0x1000;
                }
            }
            {
                let mut _src: *mut c_void =
                    gTradePlatform_Tilemap.as_ptr().cast_mut() as *mut c_void;
                let mut _dest: *mut c_void = 0x6009000_usize as *mut c_void;
                let mut _size: u32 = 0x1000;
                {
                    {
                        {
                            let dmaRegs: *mut u32 = 67109076_usize as *mut u32;
                            volatile_write(dmaRegs, _src as usize as u32);
                            volatile_write(dmaRegs.at(1), _dest as usize as u32);
                            volatile_write(dmaRegs.at(2), 0x80000000 | (_size / 2));
                            let _ = (dmaRegs.at(2)).read_volatile();
                        }
                    }
                }
            }
        }
        _ => {}
    }
}
unsafe fn LoadTradeSequenceSpriteSheetsAndPalettes() {
    LoadSpriteSheet((&raw const *sSpriteSheet_LinkMonGlow).cast_mut());
    LoadSpriteSheet((&raw const *sSpriteSheet_LinkMonShadow).cast_mut());
    LoadSpriteSheet((&raw const *sSpriteSheet_CableEnd).cast_mut());
    LoadSpriteSheet((&raw const *sSpriteSheet_GbaScreen).cast_mut());
    LoadSpritePalette((&raw const *sSpritePalette_LinkMon).cast_mut());
    LoadSpritePalette((&raw const *sSpritePalette_Gba).cast_mut());
}
unsafe fn BufferTradeSceneStrings() {
    let mut mpId: u8 = 0;
    let mut name: CArray<u8, 20> = zeroed();
    let mut ingameTrade: *mut InGameTrade = null_mut();
    if (*crate::gl!(sTradeAnim)).isLinkTrade != 0 {
        mpId = GetMultiplayerId();
        StringCopy(
            gStringVar1.as_mut_ptr(),
            (*gLinkPlayers.as_ptr())[mpId as i32 ^ 1].name.as_mut_ptr(),
        );
        GetMonData3(
            {
                let gl_i28 = crate::gl!(gSelectedTradeMonPositions[1]) as i32 % 6;
                crate::gl!(&raw gEnemyParty[gl_i28])
            },
            MON_DATA_NICKNAME,
            name.as_mut_ptr(),
        );
        StringCopy_Nickname((*gStringVar3.as_ptr()).as_mut_ptr(), name.as_mut_ptr());
        GetMonData3(
            {
                let gl_i29 = crate::gl!(gSelectedTradeMonPositions[0]);
                crate::gl!(&raw gPlayerParty[gl_i29])
            },
            MON_DATA_NICKNAME,
            name.as_mut_ptr(),
        );
        StringCopy_Nickname((*gStringVar2.as_ptr()).as_mut_ptr(), name.as_mut_ptr());
    } else {
        ingameTrade = (&raw const sIngameTrades[*(&raw const crate::ffi::gSpecialVar_0x8004)
            .cast::<u16>()
            .cast_mut()])
            .cast_mut();
        StringCopy(gStringVar1.as_mut_ptr(), (*ingameTrade).otName.as_mut_ptr());
        StringCopy_Nickname(
            (*gStringVar3.as_ptr()).as_mut_ptr(),
            (*ingameTrade).nickname.as_mut_ptr(),
        );
        GetMonData3(
            {
                let gl_i30 = *(&raw const crate::ffi::gSpecialVar_0x8005)
                    .cast::<u16>()
                    .cast_mut();
                crate::gl!(&raw gPlayerParty[gl_i30])
            },
            MON_DATA_NICKNAME,
            name.as_mut_ptr(),
        );
        StringCopy_Nickname((*gStringVar2.as_ptr()).as_mut_ptr(), name.as_mut_ptr());
    }
}
unsafe fn DoTradeAnim() -> u8 {
    if (*crate::gl!(sTradeAnim)).isCableTrade != 0 {
        return DoTradeAnim_Cable();
    } else {
        return DoTradeAnim_Wireless();
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
unsafe fn DoTradeAnim_Cable() -> u8 {
    let mut evoTarget: u16 = 0;
    match (*crate::gl!(sTradeAnim)).state {
        STATE_START => {
            (*gSprites.as_ptr())[(*crate::gl!(sTradeAnim)).monSpriteIds[0]].set_invisible(0);
            {
                let gl_i31 = (*crate::gl!(sTradeAnim)).monSpriteIds[0];
                crate::gl!(gSprites[gl_i31].x2 = -180)
            };
            {
                let gl_v = (*(&raw const crate::data::data_tables::gMonFrontPicCoords)
                    .cast::<CArray<MonCoords, 0>>())[(*crate::gl!(sTradeAnim)).monSpecies[0]]
                    .y_offset as i16;
                let gl_i32 = (*crate::gl!(sTradeAnim)).monSpriteIds[0];
                crate::gl!(gSprites[gl_i32].y2 = gl_v)
            };
            (*crate::gl!(sTradeAnim)).state += 1;
            (*crate::gl!(sTradeAnim)).cachedMapMusic = GetCurrentMapMusic();
            PlayNewMapMusic(MUS_EVOLUTION);
        }
        STATE_MON_SLIDE_IN => {
            if (*crate::gl!(sTradeAnim)).bg2hofs > 0 {
                {
                    let gl_i33 = (*crate::gl!(sTradeAnim)).monSpriteIds[0];
                    crate::gl!(gSprites[gl_i33].x2 += 3)
                };
                (*crate::gl!(sTradeAnim)).bg2hofs -= 3;
            } else {
                {
                    let gl_i34 = (*crate::gl!(sTradeAnim)).monSpriteIds[0];
                    crate::gl!(gSprites[gl_i34].x2 = 0)
                };
                (*crate::gl!(sTradeAnim)).bg2hofs = 0;
                (*crate::gl!(sTradeAnim)).state = STATE_SEND_MSG;
            }
        }
        STATE_SEND_MSG => {
            StringExpandPlaceholders(
                gStringVar4.as_mut_ptr(),
                (*(&raw const crate::data::strings::gText_XWillBeSentToY).cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut(),
            );
            DrawTextOnTradeWindow(0, gStringVar4.as_mut_ptr(), 0);
            if (*crate::gl!(sTradeAnim)).monSpecies[0] != SPECIES_EGG as u16 {
                PlayCry_Normal((*crate::gl!(sTradeAnim)).monSpecies[0], 0);
            }
            (*crate::gl!(sTradeAnim)).state = STATE_BYE_BYE;
            (*crate::gl!(sTradeAnim)).timer = 0;
        }
        STATE_BYE_BYE => {
            if ({
                (*crate::gl!(sTradeAnim)).timer += 1;
                (*crate::gl!(sTradeAnim)).timer
            }) == 80
            {
                (*crate::gl!(sTradeAnim)).releasePokeballSpriteId = CreateTradePokeballSprite(
                    (*crate::gl!(sTradeAnim)).monSpriteIds[0],
                    (*gSprites.as_ptr())[(*crate::gl!(sTradeAnim)).monSpriteIds[0]]
                        .oam
                        .paletteNum() as u8,
                    120,
                    32,
                    2,
                    1,
                    0x14,
                    0xfffff,
                );
                (*crate::gl!(sTradeAnim)).state += 1;
                StringExpandPlaceholders(
                    gStringVar4.as_mut_ptr(),
                    (*(&raw const crate::data::strings::gText_ByeByeVar1).cast::<CArray<u8, 0>>())
                        .as_ptr()
                        .cast_mut(),
                );
                DrawTextOnTradeWindow(0, gStringVar4.as_mut_ptr(), 0);
            }
        }
        STATE_POKEBALL_DEPART => {
            if ({
                let gl_i35 = (*crate::gl!(sTradeAnim)).releasePokeballSpriteId;
                crate::gl!(gSprites[gl_i35].callback)
            }) == Some(SpriteCallbackDummy as unsafe fn(*mut Sprite))
            {
                (*crate::gl!(sTradeAnim)).bouncingPokeballSpriteId = CreateSprite(
                    (&raw const *sSpriteTemplate_Pokeball).cast_mut(),
                    120,
                    32,
                    0,
                );
                {
                    let gl_i36 = (*crate::gl!(sTradeAnim)).bouncingPokeballSpriteId;
                    crate::gl!(gSprites[gl_i36].callback = Some(SpriteCB_BouncingPokeballDepart))
                };
                DestroySprite({
                    let gl_i37 = (*crate::gl!(sTradeAnim)).releasePokeballSpriteId;
                    crate::gl!(&raw gSprites[gl_i37])
                });
                (*crate::gl!(sTradeAnim)).state += 1;
            }
        }
        STATE_POKEBALL_DEPART_WAIT => {}
        STATE_FADE_OUT_TO_GBA_SEND => {
            BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 16, 0);
            (*crate::gl!(sTradeAnim)).state = STATE_WAIT_FADE_OUT_TO_GBA_SEND;
        }
        STATE_WAIT_FADE_OUT_TO_GBA_SEND => {
            if (*gPaletteFade.as_ptr()).active() == 0 {
                SetTradeSequenceBgGpuRegs(4);
                FillWindowPixelBuffer(0, 255);
                CopyWindowToVram(0, COPYWIN_FULL);
                (*crate::gl!(sTradeAnim)).state += 1;
            }
        }
        STATE_FADE_IN_TO_GBA_SEND => {
            BeginNormalPaletteFade(PALETTES_ALL, -1, 16, 0, 0);
            (*crate::gl!(sTradeAnim)).state += 1;
        }
        STATE_WAIT_FADE_IN_TO_GBA_SEND => {
            if (*gPaletteFade.as_ptr()).active() == 0 {
                (*crate::gl!(sTradeAnim)).state = STATE_GBA_ZOOM_OUT;
            }
        }
        STATE_GBA_ZOOM_OUT => {
            if (*crate::gl!(sTradeAnim)).gbaScale > 0x100 {
                (*crate::gl!(sTradeAnim)).gbaScale -= 0x34;
            } else {
                SetTradeSequenceBgGpuRegs(1);
                (*crate::gl!(sTradeAnim)).gbaScale = 0x80;
                (*crate::gl!(sTradeAnim)).state += 1;
                (*crate::gl!(sTradeAnim)).timer = 0;
            }
            (*crate::gl!(sTradeAnim)).sXY =
                div_i32(0x8000, (*crate::gl!(sTradeAnim)).gbaScale as i32) as u16;
        }
        STATE_GBA_FLASH_SEND => {
            if ({
                (*crate::gl!(sTradeAnim)).timer += 1;
                (*crate::gl!(sTradeAnim)).timer
            }) > 20
            {
                SetTradeBGAffine();
                (*crate::gl!(sTradeAnim)).connectionSpriteId2 = CreateSprite(
                    (&raw const *sSpriteTemplate_GbaScreenFlash_Long).cast_mut(),
                    120,
                    80,
                    0,
                );
                (*crate::gl!(sTradeAnim)).state += 1;
            }
        }
        STATE_GBA_STOP_FLASH_SEND => {
            if (*gSprites.as_ptr())[(*crate::gl!(sTradeAnim)).connectionSpriteId2].animEnded() != 0
            {
                DestroySprite({
                    let gl_i38 = (*crate::gl!(sTradeAnim)).connectionSpriteId2;
                    crate::gl!(&raw gSprites[gl_i38])
                });
                SetGpuReg(REG_OFFSET_BLDCNT, 1600);
                SetGpuReg(REG_OFFSET_BLDALPHA, 1036);
                (*crate::gl!(sTradeAnim)).state += 1;
            }
        }
        STATE_PAN_AWAY_GBA => {
            if ({
                (*crate::gl!(sTradeAnim)).bg1vofs -= 1;
                (*crate::gl!(sTradeAnim)).bg1vofs
            }) == 316
            {
                (*crate::gl!(sTradeAnim)).state += 1;
            }
            if (*crate::gl!(sTradeAnim)).bg1vofs == 328 {
                (*crate::gl!(sTradeAnim)).cableEndSpriteId = CreateSprite(
                    (&raw const *sSpriteTemplate_CableEnd).cast_mut(),
                    128,
                    65,
                    0,
                );
            }
        }
        STATE_CREATE_LINK_MON_LEAVING => {
            (*crate::gl!(sTradeAnim)).connectionSpriteId1 = CreateSprite(
                (&raw const *sSpriteTemplate_LinkMonGlow).cast_mut(),
                128,
                80,
                3,
            );
            (*crate::gl!(sTradeAnim)).connectionSpriteId2 = CreateSprite(
                (&raw const *sSpriteTemplate_LinkMonShadow).cast_mut(),
                128,
                80,
                0,
            );
            StartSpriteAnim(
                {
                    let gl_i39 = (*crate::gl!(sTradeAnim)).connectionSpriteId2;
                    crate::gl!(&raw gSprites[gl_i39])
                },
                ANIM_LINKMON_SMALL,
            );
            (*crate::gl!(sTradeAnim)).state += 1;
        }
        STATE_LINK_MON_TRAVEL_OUT => {
            if ({
                (*crate::gl!(sTradeAnim)).bg1vofs -= 2;
                (*crate::gl!(sTradeAnim)).bg1vofs
            }) == 166
            {
                (*crate::gl!(sTradeAnim)).state = STATE_LINK_MON_TRAVEL_OFFSCREEN;
            }
            SetGpuReg(REG_OFFSET_DISPCNT, 4673);
        }
        STATE_LINK_MON_TRAVEL_OFFSCREEN => {
            {
                let gl_i40 = (*crate::gl!(sTradeAnim)).connectionSpriteId1;
                crate::gl!(gSprites[gl_i40].y -= 2)
            };
            {
                let gl_i41 = (*crate::gl!(sTradeAnim)).connectionSpriteId2;
                crate::gl!(gSprites[gl_i41].y -= 2)
            };
            if ({
                let gl_i42 = (*crate::gl!(sTradeAnim)).connectionSpriteId1;
                crate::gl!(gSprites[gl_i42].y)
            }) < -8
            {
                (*crate::gl!(sTradeAnim)).state = STATE_FADE_OUT_TO_CROSSING;
            }
        }
        STATE_FADE_OUT_TO_CROSSING => {
            BeginNormalPaletteFade(PALETTES_ALL, -1, 0, 16, 0);
            (*crate::gl!(sTradeAnim)).state = STATE_WAIT_FADE_OUT_TO_CROSSING;
        }
        STATE_WAIT_FADE_OUT_TO_CROSSING => {
            if (*gPaletteFade.as_ptr()).active() == 0 {
                DestroySprite({
                    let gl_i43 = (*crate::gl!(sTradeAnim)).connectionSpriteId1;
                    crate::gl!(&raw gSprites[gl_i43])
                });
                DestroySprite({
                    let gl_i44 = (*crate::gl!(sTradeAnim)).connectionSpriteId2;
                    crate::gl!(&raw gSprites[gl_i44])
                });
                SetTradeSequenceBgGpuRegs(2);
                (*crate::gl!(sTradeAnim)).state += 1;
            }
        }
        STATE_FADE_IN_TO_CROSSING => {
            BeginNormalPaletteFade(PALETTES_ALL, -1, 16, 0, 0);
            (*crate::gl!(sTradeAnim)).connectionSpriteId1 = CreateSprite(
                (&raw const *sSpriteTemplate_LinkMonShadow).cast_mut(),
                111,
                170,
                0,
            );
            (*crate::gl!(sTradeAnim)).connectionSpriteId2 = CreateSprite(
                (&raw const *sSpriteTemplate_LinkMonShadow).cast_mut(),
                129,
                -10,
                0,
            );
            (*crate::gl!(sTradeAnim)).state += 1;
        }
        STATE_WAIT_FADE_IN_TO_CROSSING => {
            if (*gPaletteFade.as_ptr()).active() == 0 {
                PlaySE(SE_WARP_OUT);
                (*crate::gl!(sTradeAnim)).state += 1;
            }
            {
                let gl_i45 = (*crate::gl!(sTradeAnim)).connectionSpriteId1;
                crate::gl!(gSprites[gl_i45].y2 -= 3)
            };
            {
                let gl_i46 = (*crate::gl!(sTradeAnim)).connectionSpriteId2;
                crate::gl!(gSprites[gl_i46].y2 += 3)
            };
        }
        STATE_CROSSING_LINK_MONS_ENTER => {
            {
                let gl_i47 = (*crate::gl!(sTradeAnim)).connectionSpriteId1;
                crate::gl!(gSprites[gl_i47].y2 -= 3)
            };
            {
                let gl_i48 = (*crate::gl!(sTradeAnim)).connectionSpriteId2;
                crate::gl!(gSprites[gl_i48].y2 += 3)
            };
            if ({
                let gl_i49 = (*crate::gl!(sTradeAnim)).connectionSpriteId1;
                crate::gl!(gSprites[gl_i49].y2)
            }) <= -90
            {
                {
                    let gl_i50 = (*crate::gl!(sTradeAnim)).connectionSpriteId1;
                    crate::gl!(gSprites[gl_i50].data[1] = 1)
                };
                {
                    let gl_i51 = (*crate::gl!(sTradeAnim)).connectionSpriteId2;
                    crate::gl!(gSprites[gl_i51].data[1] = 1)
                };
                (*crate::gl!(sTradeAnim)).state += 1;
            }
        }
        STATE_CROSSING_BLEND_WHITE_1 => {
            BlendPalettes(0x1, 16, 65535);
            (*crate::gl!(sTradeAnim)).state += 1;
        }
        STATE_CROSSING_BLEND_WHITE_2 => {
            BlendPalettes(0x1, 0, 65535);
            (*crate::gl!(sTradeAnim)).state += 1;
        }
        STATE_CROSSING_BLEND_WHITE_3 => {
            BlendPalettes(0x1, 16, 65535);
            (*crate::gl!(sTradeAnim)).state += 1;
        }
        STATE_CROSSING_CREATE_MON_PICS => {
            if IsMonSpriteNotFlipped((*crate::gl!(sTradeAnim)).monSpecies[0]) == 0 {
                {
                    let gl_v = sAffineAnims_CrossingMonPics.as_ptr().cast_mut();
                    let gl_i52 = (*crate::gl!(sTradeAnim)).monSpriteIds[0];
                    crate::gl!(gSprites[gl_i52].affineAnims = gl_v)
                };
                (*gSprites.as_ptr())[(*crate::gl!(sTradeAnim)).monSpriteIds[0]]
                    .oam
                    .set_affineMode(ST_OAM_AFFINE_DOUBLE);
                CalcCenterToCornerVec(
                    {
                        let gl_i53 = (*crate::gl!(sTradeAnim)).monSpriteIds[0];
                        crate::gl!(&raw gSprites[gl_i53])
                    },
                    TRADE_PLAYER,
                    ST_OAM_AFFINE_DOUBLE as u8,
                    ST_OAM_AFFINE_DOUBLE as u8,
                );
                StartSpriteAffineAnim(
                    {
                        let gl_i54 = (*crate::gl!(sTradeAnim)).monSpriteIds[0];
                        crate::gl!(&raw gSprites[gl_i54])
                    },
                    0,
                );
            } else {
                StartSpriteAffineAnim(
                    {
                        let gl_i55 = (*crate::gl!(sTradeAnim)).monSpriteIds[0];
                        crate::gl!(&raw gSprites[gl_i55])
                    },
                    0,
                );
            }
            StartSpriteAffineAnim(
                {
                    let gl_i56 = (*crate::gl!(sTradeAnim)).monSpriteIds[1];
                    crate::gl!(&raw gSprites[gl_i56])
                },
                0,
            );
            {
                let gl_i57 = (*crate::gl!(sTradeAnim)).monSpriteIds[0];
                crate::gl!(gSprites[gl_i57].x = 60)
            };
            {
                let gl_i58 = (*crate::gl!(sTradeAnim)).monSpriteIds[1];
                crate::gl!(gSprites[gl_i58].x = 180)
            };
            {
                let gl_i59 = (*crate::gl!(sTradeAnim)).monSpriteIds[0];
                crate::gl!(gSprites[gl_i59].y = 192)
            };
            {
                let gl_i60 = (*crate::gl!(sTradeAnim)).monSpriteIds[1];
                crate::gl!(gSprites[gl_i60].y = -32)
            };
            (*gSprites.as_ptr())[(*crate::gl!(sTradeAnim)).monSpriteIds[0]].set_invisible(0);
            (*gSprites.as_ptr())[(*crate::gl!(sTradeAnim)).monSpriteIds[1]]
                .set_invisible(FALSE as u16);
            (*crate::gl!(sTradeAnim)).state += 1;
        }
        STATE_CROSSING_MON_PICS_MOVE => {
            {
                let gl_i61 = (*crate::gl!(sTradeAnim)).monSpriteIds[0];
                crate::gl!(gSprites[gl_i61].y2 -= 3)
            };
            {
                let gl_i62 = (*crate::gl!(sTradeAnim)).monSpriteIds[1];
                crate::gl!(gSprites[gl_i62].y2 += 3)
            };
            if ({
                let gl_i63 = (*crate::gl!(sTradeAnim)).monSpriteIds[0];
                crate::gl!(gSprites[gl_i63].y2)
            }) < -160
                && ({
                    let gl_i64 = (*crate::gl!(sTradeAnim)).monSpriteIds[0];
                    crate::gl!(gSprites[gl_i64].y2)
                }) >= -163
            {
                PlaySE(SE_WARP_IN);
            }
            if ({
                let gl_i65 = (*crate::gl!(sTradeAnim)).monSpriteIds[0];
                crate::gl!(gSprites[gl_i65].y2)
            }) < -222
            {
                {
                    let gl_i66 = (*crate::gl!(sTradeAnim)).connectionSpriteId1;
                    crate::gl!(gSprites[gl_i66].data[1] = 0)
                };
                {
                    let gl_i67 = (*crate::gl!(sTradeAnim)).connectionSpriteId2;
                    crate::gl!(gSprites[gl_i67].data[1] = 0)
                };
                (*crate::gl!(sTradeAnim)).state += 1;
                (*gSprites.as_ptr())[(*crate::gl!(sTradeAnim)).monSpriteIds[0]]
                    .set_invisible(TRUE as u16);
                (*gSprites.as_ptr())[(*crate::gl!(sTradeAnim)).monSpriteIds[1]].set_invisible(1);
                BlendPalettes(0x1, 0, 65535);
            }
        }
        STATE_CROSSING_LINK_MONS_EXIT => {
            {
                let gl_i68 = (*crate::gl!(sTradeAnim)).connectionSpriteId1;
                crate::gl!(gSprites[gl_i68].y2 -= 3)
            };
            {
                let gl_i69 = (*crate::gl!(sTradeAnim)).connectionSpriteId2;
                crate::gl!(gSprites[gl_i69].y2 += 3)
            };
            if ({
                let gl_i70 = (*crate::gl!(sTradeAnim)).connectionSpriteId1;
                crate::gl!(gSprites[gl_i70].y2)
            }) <= -222
            {
                BeginNormalPaletteFade(PALETTES_ALL, -1, 0, 16, 0);
                (*crate::gl!(sTradeAnim)).state += 1;
                DestroySprite({
                    let gl_i71 = (*crate::gl!(sTradeAnim)).connectionSpriteId1;
                    crate::gl!(&raw gSprites[gl_i71])
                });
                DestroySprite({
                    let gl_i72 = (*crate::gl!(sTradeAnim)).connectionSpriteId2;
                    crate::gl!(&raw gSprites[gl_i72])
                });
            }
        }
        STATE_CREATE_LINK_MON_ARRIVING => {
            if (*gPaletteFade.as_ptr()).active() == 0 {
                (*crate::gl!(sTradeAnim)).state += 1;
                SetTradeSequenceBgGpuRegs(1);
                (*crate::gl!(sTradeAnim)).bg1vofs = 166;
                (*crate::gl!(sTradeAnim)).connectionSpriteId1 = CreateSprite(
                    (&raw const *sSpriteTemplate_LinkMonGlow).cast_mut(),
                    128,
                    -20,
                    3,
                );
                (*crate::gl!(sTradeAnim)).connectionSpriteId2 = CreateSprite(
                    (&raw const *sSpriteTemplate_LinkMonShadow).cast_mut(),
                    128,
                    -20,
                    0,
                );
                StartSpriteAnim(
                    {
                        let gl_i73 = (*crate::gl!(sTradeAnim)).connectionSpriteId2;
                        crate::gl!(&raw gSprites[gl_i73])
                    },
                    ANIM_LINKMON_SMALL,
                );
            }
        }
        STATE_FADE_OUT_TO_GBA_RECV => {
            BeginNormalPaletteFade(PALETTES_ALL, -1, 16, 0, 0);
            (*crate::gl!(sTradeAnim)).state += 1;
        }
        STATE_WAIT_FADE_OUT_TO_GBA_RECV => {
            SetGpuReg(0x0, 4672);
            if (*gPaletteFade.as_ptr()).active() == 0 {
                (*crate::gl!(sTradeAnim)).state += 1;
            }
        }
        STATE_LINK_MON_TRAVEL_IN => {
            {
                let gl_i74 = (*crate::gl!(sTradeAnim)).connectionSpriteId1;
                crate::gl!(gSprites[gl_i74].y2 += 3)
            };
            {
                let gl_i75 = (*crate::gl!(sTradeAnim)).connectionSpriteId2;
                crate::gl!(gSprites[gl_i75].y2 += 3)
            };
            if ({
                let gl_i76 = (*crate::gl!(sTradeAnim)).connectionSpriteId1;
                crate::gl!(gSprites[gl_i76].y2)
            }) as i32
                + ({
                    let gl_i77 = (*crate::gl!(sTradeAnim)).connectionSpriteId1;
                    crate::gl!(gSprites[gl_i77].y)
                }) as i32
                == 64
            {
                (*crate::gl!(sTradeAnim)).state += 1;
            }
        }
        STATE_PAN_TO_GBA => {
            if ({
                (*crate::gl!(sTradeAnim)).bg1vofs += 2;
                (*crate::gl!(sTradeAnim)).bg1vofs
            }) > 316
            {
                (*crate::gl!(sTradeAnim)).bg1vofs = 316;
                (*crate::gl!(sTradeAnim)).state += 1;
            }
        }
        STATE_DESTROY_LINK_MON => {
            DestroySprite({
                let gl_i78 = (*crate::gl!(sTradeAnim)).connectionSpriteId1;
                crate::gl!(&raw gSprites[gl_i78])
            });
            DestroySprite({
                let gl_i79 = (*crate::gl!(sTradeAnim)).connectionSpriteId2;
                crate::gl!(&raw gSprites[gl_i79])
            });
            (*crate::gl!(sTradeAnim)).state += 1;
            (*crate::gl!(sTradeAnim)).timer = 0;
        }
        STATE_LINK_MON_ARRIVED_DELAY => {
            if ({
                (*crate::gl!(sTradeAnim)).timer += 1;
                (*crate::gl!(sTradeAnim)).timer
            }) == 10
            {
                (*crate::gl!(sTradeAnim)).state += 1;
            }
        }
        STATE_MOVE_GBA_TO_CENTER => {
            if ({
                (*crate::gl!(sTradeAnim)).bg1vofs += 1;
                (*crate::gl!(sTradeAnim)).bg1vofs
            }) > 348
            {
                (*crate::gl!(sTradeAnim)).bg1vofs = 348;
                (*crate::gl!(sTradeAnim)).state += 1;
            }
            if (*crate::gl!(sTradeAnim)).bg1vofs == 328
                && (*crate::gl!(sTradeAnim)).isCableTrade != 0
            {
                (*crate::gl!(sTradeAnim)).cableEndSpriteId = CreateSprite(
                    (&raw const *sSpriteTemplate_CableEnd).cast_mut(),
                    128,
                    65,
                    0,
                );
                {
                    let gl_i80 = (*crate::gl!(sTradeAnim)).cableEndSpriteId;
                    crate::gl!(gSprites[gl_i80].callback = Some(SpriteCB_CableEndReceiving))
                };
            }
        }
        STATE_GBA_FLASH_RECV => {
            (*crate::gl!(sTradeAnim)).connectionSpriteId2 = CreateSprite(
                (&raw const *sSpriteTemplate_GbaScreenFlash_Long).cast_mut(),
                120,
                80,
                0,
            );
            (*crate::gl!(sTradeAnim)).state = STATE_GBA_STOP_FLASH_RECV;
        }
        STATE_GBA_STOP_FLASH_RECV => {
            if (*gSprites.as_ptr())[(*crate::gl!(sTradeAnim)).connectionSpriteId2].animEnded() != 0
            {
                DestroySprite({
                    let gl_i81 = (*crate::gl!(sTradeAnim)).connectionSpriteId2;
                    crate::gl!(&raw gSprites[gl_i81])
                });
                SetTradeSequenceBgGpuRegs(6);
                (*crate::gl!(sTradeAnim)).state += 1;
                PlaySE(SE_M_SAND_ATTACK);
            }
        }
        STATE_GBA_ZOOM_IN => {
            if (*crate::gl!(sTradeAnim)).gbaScale < 0x400 {
                (*crate::gl!(sTradeAnim)).gbaScale += 0x34;
            } else {
                (*crate::gl!(sTradeAnim)).gbaScale = 0x400;
                (*crate::gl!(sTradeAnim)).state += 1;
            }
            (*crate::gl!(sTradeAnim)).sXY =
                div_i32(0x8000, (*crate::gl!(sTradeAnim)).gbaScale as i32) as u16;
        }
        STATE_FADE_OUT_TO_NEW_MON => {
            BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 16, 0);
            (*crate::gl!(sTradeAnim)).state = STATE_WAIT_FADE_OUT_TO_NEW_MON;
        }
        STATE_WAIT_FADE_OUT_TO_NEW_MON => {
            if (*gPaletteFade.as_ptr()).active() == 0 {
                SetTradeSequenceBgGpuRegs(5);
                SetTradeSequenceBgGpuRegs(7);
                (*gPaletteFade.as_ptr()).set_bufferTransferDisabled(TRUE as u16);
                (*crate::gl!(sTradeAnim)).state += 1;
            }
        }
        STATE_FADE_IN_TO_NEW_MON => {
            (*gPaletteFade.as_ptr()).set_bufferTransferDisabled(FALSE as u16);
            BeginNormalPaletteFade(PALETTES_ALL, 0, 16, 0, 0);
            (*crate::gl!(sTradeAnim)).state += 1;
        }
        STATE_WAIT_FADE_IN_TO_NEW_MON => {
            SetGpuReg(0x0, 5184);
            if (*gPaletteFade.as_ptr()).active() == 0 {
                (*crate::gl!(sTradeAnim)).state += 1;
            }
        }
        STATE_POKEBALL_ARRIVE => {
            (*crate::gl!(sTradeAnim)).bouncingPokeballSpriteId = CreateSprite(
                (&raw const *sSpriteTemplate_Pokeball).cast_mut(),
                120,
                -8,
                0,
            );
            {
                let gl_i82 = (*crate::gl!(sTradeAnim)).bouncingPokeballSpriteId;
                crate::gl!(gSprites[gl_i82].data[3] = 74)
            };
            {
                let gl_i83 = (*crate::gl!(sTradeAnim)).bouncingPokeballSpriteId;
                crate::gl!(gSprites[gl_i83].callback = Some(SpriteCB_BouncingPokeballArrive))
            };
            StartSpriteAnim(
                {
                    let gl_i84 = (*crate::gl!(sTradeAnim)).bouncingPokeballSpriteId;
                    crate::gl!(&raw gSprites[gl_i84])
                },
                1,
            );
            StartSpriteAffineAnim(
                {
                    let gl_i85 = (*crate::gl!(sTradeAnim)).bouncingPokeballSpriteId;
                    crate::gl!(&raw gSprites[gl_i85])
                },
                2,
            );
            BlendPalettes(
                shl_i32(
                    1,
                    16 + (*gSprites.as_ptr())[(*crate::gl!(sTradeAnim)).bouncingPokeballSpriteId]
                        .oam
                        .paletteNum() as u32,
                ) as u32,
                16,
                65535,
            );
            (*crate::gl!(sTradeAnim)).state += 1;
            (*crate::gl!(sTradeAnim)).timer = 0;
        }
        STATE_FADE_POKEBALL_TO_NORMAL => {
            BeginNormalPaletteFade(
                shl_i32(
                    1,
                    16 + (*gSprites.as_ptr())[(*crate::gl!(sTradeAnim)).bouncingPokeballSpriteId]
                        .oam
                        .paletteNum() as u32,
                ) as u32,
                1,
                16,
                0,
                65535,
            );
            (*crate::gl!(sTradeAnim)).state += 1;
        }
        STATE_POKEBALL_ARRIVE_WAIT => {
            if ({
                let gl_i86 = (*crate::gl!(sTradeAnim)).bouncingPokeballSpriteId;
                crate::gl!(gSprites[gl_i86].callback)
            }) == Some(SpriteCallbackDummy as unsafe fn(*mut Sprite))
            {
                HandleLoadSpecialPokePic_2(
                    (&raw const (*(&raw const crate::data::data_tables::gMonFrontPicTable)
                        .cast::<CArray<CompressedSpriteSheet, 0>>())
                        [(*crate::gl!(sTradeAnim)).monSpecies[1]])
                        .cast_mut(),
                    (*crate::gl!(gMonSpritesGfxPtr)).sprites.ptr[3],
                    (*crate::gl!(sTradeAnim)).monSpecies[1] as i32,
                    (*crate::gl!(sTradeAnim)).monPersonalities[1],
                );
                (*crate::gl!(sTradeAnim)).state += 1;
            }
        }
        STATE_SHOW_NEW_MON => {
            {
                let gl_i87 = (*crate::gl!(sTradeAnim)).monSpriteIds[1];
                crate::gl!(gSprites[gl_i87].x = 120)
            };
            {
                let gl_v = (*(&raw const crate::data::data_tables::gMonFrontPicCoords)
                    .cast::<CArray<MonCoords, 0>>())[(*crate::gl!(sTradeAnim)).monSpecies[1]]
                    .y_offset as i16
                    + 60;
                let gl_i88 = (*crate::gl!(sTradeAnim)).monSpriteIds[1];
                crate::gl!(gSprites[gl_i88].y = gl_v)
            };
            {
                let gl_i89 = (*crate::gl!(sTradeAnim)).monSpriteIds[1];
                crate::gl!(gSprites[gl_i89].x2 = 0)
            };
            {
                let gl_i90 = (*crate::gl!(sTradeAnim)).monSpriteIds[1];
                crate::gl!(gSprites[gl_i90].y2 = 0)
            };
            StartSpriteAnim(
                {
                    let gl_i91 = (*crate::gl!(sTradeAnim)).monSpriteIds[1];
                    crate::gl!(&raw gSprites[gl_i91])
                },
                0,
            );
            CreatePokeballSpriteToReleaseMon(
                (*crate::gl!(sTradeAnim)).monSpriteIds[1],
                (*gSprites.as_ptr())[(*crate::gl!(sTradeAnim)).monSpriteIds[1]]
                    .oam
                    .paletteNum() as u8,
                120,
                84,
                2,
                1,
                20,
                0xfffff,
                (*crate::gl!(sTradeAnim)).monSpecies[1],
            );
            FreeSpriteOamMatrix({
                let gl_i92 = (*crate::gl!(sTradeAnim)).bouncingPokeballSpriteId;
                crate::gl!(&raw gSprites[gl_i92])
            });
            DestroySprite({
                let gl_i93 = (*crate::gl!(sTradeAnim)).bouncingPokeballSpriteId;
                crate::gl!(&raw gSprites[gl_i93])
            });
            (*crate::gl!(sTradeAnim)).state += 1;
        }
        STATE_NEW_MON_MSG => {
            SetGpuReg(0x0, 5440);
            StringExpandPlaceholders(
                gStringVar4.as_mut_ptr(),
                (*(&raw const crate::data::strings::gText_XSentOverY).cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut(),
            );
            DrawTextOnTradeWindow(0, gStringVar4.as_mut_ptr(), 0);
            (*crate::gl!(sTradeAnim)).state = STATE_DELAY_FOR_MON_ANIM;
            (*crate::gl!(sTradeAnim)).timer = 0;
        }
        STATE_DELAY_FOR_MON_ANIM => {
            if ({
                (*crate::gl!(sTradeAnim)).timer += 1;
                (*crate::gl!(sTradeAnim)).timer
            }) > 60
            {
                (*crate::gl!(sTradeAnim)).state = STATE_WAIT_FOR_MON_CRY;
                (*crate::gl!(sTradeAnim)).timer = 0;
            }
        }
        STATE_WAIT_FOR_MON_CRY => {
            if IsCryFinished() != 0 {
                (*crate::gl!(sTradeAnim)).state = STATE_TAKE_CARE_OF_MON;
            }
        }
        STATE_TAKE_CARE_OF_MON => {
            if ({
                (*crate::gl!(sTradeAnim)).timer += 1;
                (*crate::gl!(sTradeAnim)).timer
            }) == 10
            {
                PlayFanfare(MUS_EVOLVED);
            }
            if (*crate::gl!(sTradeAnim)).timer == 250 {
                (*crate::gl!(sTradeAnim)).state += 1;
                StringExpandPlaceholders(
                    gStringVar4.as_mut_ptr(),
                    (*(&raw const crate::data::strings::gText_TakeGoodCareOfX)
                        .cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut(),
                );
                DrawTextOnTradeWindow(0, gStringVar4.as_mut_ptr(), 0);
                (*crate::gl!(sTradeAnim)).timer = 0;
            }
        }
        STATE_AFTER_NEW_MON_DELAY => {
            if ({
                (*crate::gl!(sTradeAnim)).timer += 1;
                (*crate::gl!(sTradeAnim)).timer
            }) == 60
            {
                (*crate::gl!(sTradeAnim)).state += 1;
            }
        }
        STATE_CHECK_RIBBONS => {
            CheckPartnersMonForRibbons();
            (*crate::gl!(sTradeAnim)).state += 1;
        }
        STATE_END_LINK_TRADE => {
            if (*crate::gl!(sTradeAnim)).isLinkTrade != 0 {
                return TRUE;
            } else if crate::gl!(gMain.newKeys) as i32 & A_BUTTON != 0 {
                (*crate::gl!(sTradeAnim)).state += 1;
            }
        }
        STATE_TRY_EVOLUTION => {
            TradeMons(gSpecialVar_0x8005 as u8, 0);
            crate::gl!(gCB2_AfterEvolution = Some(CB2_InGameTrade));
            evoTarget = GetEvolutionTargetSpecies(
                {
                    let gl_i94 = crate::gl!(gSelectedTradeMonPositions[0]);
                    crate::gl!(&raw gPlayerParty[gl_i94])
                },
                EVO_MODE_TRADE,
                ITEM_NONE,
            );
            if evoTarget != SPECIES_NONE {
                TradeEvolutionScene(
                    {
                        let gl_i95 = crate::gl!(gSelectedTradeMonPositions[0]);
                        crate::gl!(&raw gPlayerParty[gl_i95])
                    },
                    evoTarget,
                    (*crate::gl!(sTradeAnim)).monSpriteIds[1],
                    crate::gl!(gSelectedTradeMonPositions[0]),
                );
            }
            (*crate::gl!(sTradeAnim)).state += 1;
        }
        STATE_FADE_OUT_END => {
            BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 16, 0);
            (*crate::gl!(sTradeAnim)).state += 1;
        }
        STATE_WAIT_FADE_OUT_END if (*gPaletteFade.as_ptr()).active() == 0 => {
            PlayNewMapMusic((*crate::gl!(sTradeAnim)).cachedMapMusic);
            if !(*sTradeAnim.as_ptr()).is_null() {
                FreeAllWindowBuffers();
                Free(GetBgTilemapBuffer(3));
                Free(GetBgTilemapBuffer(1));
                Free(GetBgTilemapBuffer(0));
                FreeMonSpritesGfx();
                Free(crate::gl!(sTradeAnim) as *mut c_void);
                crate::gl!(sTradeAnim = null_mut());
            }
            SetMainCallback2(Some(CB2_ReturnToField));
            BufferInGameTradeMonName();
        }
        _ => {}
    }
    FALSE
}
unsafe fn DoTradeAnim_Wireless() -> u8 {
    let mut evoTarget: u16 = 0;
    match (*crate::gl!(sTradeAnim)).state {
        STATE_START => {
            (*gSprites.as_ptr())[(*crate::gl!(sTradeAnim)).monSpriteIds[0]].set_invisible(0);
            {
                let gl_i96 = (*crate::gl!(sTradeAnim)).monSpriteIds[0];
                crate::gl!(gSprites[gl_i96].x2 = -180)
            };
            {
                let gl_v = (*(&raw const crate::data::data_tables::gMonFrontPicCoords)
                    .cast::<CArray<MonCoords, 0>>())[(*crate::gl!(sTradeAnim)).monSpecies[0]]
                    .y_offset as i16;
                let gl_i97 = (*crate::gl!(sTradeAnim)).monSpriteIds[0];
                crate::gl!(gSprites[gl_i97].y2 = gl_v)
            };
            (*crate::gl!(sTradeAnim)).state += 1;
            (*crate::gl!(sTradeAnim)).cachedMapMusic = GetCurrentMapMusic();
            PlayNewMapMusic(MUS_EVOLUTION);
        }
        STATE_MON_SLIDE_IN => {
            if (*crate::gl!(sTradeAnim)).bg2hofs > 0 {
                {
                    let gl_i98 = (*crate::gl!(sTradeAnim)).monSpriteIds[0];
                    crate::gl!(gSprites[gl_i98].x2 += 3)
                };
                (*crate::gl!(sTradeAnim)).bg2hofs -= 3;
            } else {
                {
                    let gl_i99 = (*crate::gl!(sTradeAnim)).monSpriteIds[0];
                    crate::gl!(gSprites[gl_i99].x2 = 0)
                };
                (*crate::gl!(sTradeAnim)).bg2hofs = 0;
                (*crate::gl!(sTradeAnim)).state = STATE_SEND_MSG;
            }
        }
        STATE_SEND_MSG => {
            StringExpandPlaceholders(
                gStringVar4.as_mut_ptr(),
                (*(&raw const crate::data::strings::gText_XWillBeSentToY).cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut(),
            );
            DrawTextOnTradeWindow(0, gStringVar4.as_mut_ptr(), 0);
            if (*crate::gl!(sTradeAnim)).monSpecies[0] != SPECIES_EGG as u16 {
                PlayCry_Normal((*crate::gl!(sTradeAnim)).monSpecies[0], 0);
            }
            (*crate::gl!(sTradeAnim)).state = STATE_BYE_BYE;
            (*crate::gl!(sTradeAnim)).timer = 0;
        }
        STATE_BYE_BYE => {
            if ({
                (*crate::gl!(sTradeAnim)).timer += 1;
                (*crate::gl!(sTradeAnim)).timer
            }) == 80
            {
                (*crate::gl!(sTradeAnim)).releasePokeballSpriteId = CreateTradePokeballSprite(
                    (*crate::gl!(sTradeAnim)).monSpriteIds[0],
                    (*gSprites.as_ptr())[(*crate::gl!(sTradeAnim)).monSpriteIds[0]]
                        .oam
                        .paletteNum() as u8,
                    120,
                    32,
                    2,
                    1,
                    0x14,
                    0xfffff,
                );
                (*crate::gl!(sTradeAnim)).state += 1;
                StringExpandPlaceholders(
                    gStringVar4.as_mut_ptr(),
                    (*(&raw const crate::data::strings::gText_ByeByeVar1).cast::<CArray<u8, 0>>())
                        .as_ptr()
                        .cast_mut(),
                );
                DrawTextOnTradeWindow(0, gStringVar4.as_mut_ptr(), 0);
            }
        }
        STATE_POKEBALL_DEPART => {
            if ({
                let gl_i100 = (*crate::gl!(sTradeAnim)).releasePokeballSpriteId;
                crate::gl!(gSprites[gl_i100].callback)
            }) == Some(SpriteCallbackDummy as unsafe fn(*mut Sprite))
            {
                (*crate::gl!(sTradeAnim)).bouncingPokeballSpriteId = CreateSprite(
                    (&raw const *sSpriteTemplate_Pokeball).cast_mut(),
                    120,
                    32,
                    0,
                );
                {
                    let gl_i101 = (*crate::gl!(sTradeAnim)).bouncingPokeballSpriteId;
                    crate::gl!(gSprites[gl_i101].callback = Some(SpriteCB_BouncingPokeballDepart))
                };
                DestroySprite({
                    let gl_i102 = (*crate::gl!(sTradeAnim)).releasePokeballSpriteId;
                    crate::gl!(&raw gSprites[gl_i102])
                });
                (*crate::gl!(sTradeAnim)).state += 1;
            }
        }
        STATE_POKEBALL_DEPART_WAIT => {}
        STATE_FADE_OUT_TO_GBA_SEND => {
            BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 16, 0);
            (*crate::gl!(sTradeAnim)).state = STATE_WAIT_FADE_OUT_TO_GBA_SEND;
        }
        STATE_WAIT_FADE_OUT_TO_GBA_SEND => {
            if (*gPaletteFade.as_ptr()).active() == 0 {
                SetTradeSequenceBgGpuRegs(4);
                FillWindowPixelBuffer(0, 255);
                CopyWindowToVram(0, COPYWIN_FULL);
                (*crate::gl!(sTradeAnim)).state += 1;
            }
        }
        STATE_FADE_IN_TO_GBA_SEND => {
            BeginNormalPaletteFade(PALETTES_ALL, -1, 16, 0, 0);
            (*crate::gl!(sTradeAnim)).state += 1;
        }
        STATE_WAIT_FADE_IN_TO_GBA_SEND => {
            if (*gPaletteFade.as_ptr()).active() == 0 {
                (*crate::gl!(sTradeAnim)).state = STATE_GBA_ZOOM_OUT;
            }
        }
        STATE_GBA_ZOOM_OUT => {
            if (*crate::gl!(sTradeAnim)).gbaScale > 0x100 {
                (*crate::gl!(sTradeAnim)).gbaScale -= 0x34;
            } else {
                SetTradeSequenceBgGpuRegs(1);
                (*crate::gl!(sTradeAnim)).gbaScale = 0x80;
                (*crate::gl!(sTradeAnim)).state = STATE_GBA_FLASH_SEND_WIRELESS;
                (*crate::gl!(sTradeAnim)).timer = 0;
            }
            (*crate::gl!(sTradeAnim)).sXY =
                div_i32(0x8000, (*crate::gl!(sTradeAnim)).gbaScale as i32) as u16;
        }
        STATE_GBA_FLASH_SEND_WIRELESS => {
            if ({
                (*crate::gl!(sTradeAnim)).timer += 1;
                (*crate::gl!(sTradeAnim)).timer
            }) > 20
            {
                SetTradeSequenceBgGpuRegs(3);
                (*crate::gl!(sTradeAnim)).connectionSpriteId2 = CreateSprite(
                    (&raw const *sSpriteTemplate_GbaScreenFlash_Short).cast_mut(),
                    120,
                    80,
                    0,
                );
                (*crate::gl!(sTradeAnim)).state += 1;
            }
        }
        STATE_GBA_STOP_FLASH_SEND_WIRELESS => {
            if (*gSprites.as_ptr())[(*crate::gl!(sTradeAnim)).connectionSpriteId2].animEnded() != 0
            {
                DestroySprite({
                    let gl_i103 = (*crate::gl!(sTradeAnim)).connectionSpriteId2;
                    crate::gl!(&raw gSprites[gl_i103])
                });
                SetGpuReg(REG_OFFSET_BLDCNT, 1106);
                SetGpuReg(REG_OFFSET_BLDALPHA, 1040);
                CreateTask(Some(Task_AnimateWirelessSignal), 5);
                (*crate::gl!(sTradeAnim)).state += 1;
            }
        }
        STATE_WAIT_WIRELESS_SIGNAL_SEND => {
            if FuncIsActiveTask(Some(Task_AnimateWirelessSignal)) == 0 {
                (*crate::gl!(sTradeAnim)).state = STATE_PAN_AWAY_GBA;
            }
        }
        STATE_PAN_AWAY_GBA => {
            if ({
                (*crate::gl!(sTradeAnim)).bg1vofs -= 1;
                (*crate::gl!(sTradeAnim)).bg1vofs
            }) == 316
            {
                (*crate::gl!(sTradeAnim)).state += 1;
            }
        }
        STATE_CREATE_LINK_MON_LEAVING => {
            (*crate::gl!(sTradeAnim)).connectionSpriteId1 = CreateSprite(
                (&raw const *sSpriteTemplate_LinkMonGlow).cast_mut(),
                120,
                80,
                3,
            );
            {
                let gl_i104 = (*crate::gl!(sTradeAnim)).connectionSpriteId1;
                crate::gl!(gSprites[gl_i104].callback = Some(SpriteCB_LinkMonGlowWireless))
            };
            (*crate::gl!(sTradeAnim)).connectionSpriteId2 = CreateSprite(
                (&raw const *sSpriteTemplate_LinkMonShadow).cast_mut(),
                120,
                80,
                0,
            );
            StartSpriteAnim(
                {
                    let gl_i105 = (*crate::gl!(sTradeAnim)).connectionSpriteId2;
                    crate::gl!(&raw gSprites[gl_i105])
                },
                ANIM_LINKMON_SMALL,
            );
            (*crate::gl!(sTradeAnim)).state += 1;
        }
        STATE_LINK_MON_TRAVEL_OUT => {
            if ({
                (*crate::gl!(sTradeAnim)).bg1vofs -= 3;
                (*crate::gl!(sTradeAnim)).bg1vofs
            }) == 166
            {
                (*crate::gl!(sTradeAnim)).state = STATE_LINK_MON_TRAVEL_OFFSCREEN;
            }
            SetGpuReg(REG_OFFSET_DISPCNT, 4673);
        }
        STATE_LINK_MON_TRAVEL_OFFSCREEN => {
            {
                let gl_i106 = (*crate::gl!(sTradeAnim)).connectionSpriteId1;
                crate::gl!(gSprites[gl_i106].y -= 2)
            };
            {
                let gl_i107 = (*crate::gl!(sTradeAnim)).connectionSpriteId2;
                crate::gl!(gSprites[gl_i107].y -= 2)
            };
            if ({
                let gl_i108 = (*crate::gl!(sTradeAnim)).connectionSpriteId1;
                crate::gl!(gSprites[gl_i108].y)
            }) < -8
            {
                (*crate::gl!(sTradeAnim)).state = STATE_FADE_OUT_TO_CROSSING;
            }
        }
        STATE_FADE_OUT_TO_CROSSING => {
            BeginNormalPaletteFade(PALETTES_ALL, -1, 0, 16, 0);
            (*crate::gl!(sTradeAnim)).state = STATE_WAIT_FADE_OUT_TO_CROSSING;
        }
        STATE_WAIT_FADE_OUT_TO_CROSSING => {
            if (*gPaletteFade.as_ptr()).active() == 0 {
                DestroySprite({
                    let gl_i109 = (*crate::gl!(sTradeAnim)).connectionSpriteId1;
                    crate::gl!(&raw gSprites[gl_i109])
                });
                DestroySprite({
                    let gl_i110 = (*crate::gl!(sTradeAnim)).connectionSpriteId2;
                    crate::gl!(&raw gSprites[gl_i110])
                });
                SetTradeSequenceBgGpuRegs(2);
                (*crate::gl!(sTradeAnim)).state += 1;
            }
        }
        STATE_FADE_IN_TO_CROSSING => {
            BeginNormalPaletteFade(PALETTES_ALL, -1, 16, 0, 0);
            (*crate::gl!(sTradeAnim)).connectionSpriteId1 = CreateSprite(
                (&raw const *sSpriteTemplate_LinkMonShadow).cast_mut(),
                111,
                170,
                0,
            );
            (*crate::gl!(sTradeAnim)).connectionSpriteId2 = CreateSprite(
                (&raw const *sSpriteTemplate_LinkMonShadow).cast_mut(),
                129,
                -10,
                0,
            );
            (*crate::gl!(sTradeAnim)).state += 1;
        }
        STATE_WAIT_FADE_IN_TO_CROSSING => {
            if (*gPaletteFade.as_ptr()).active() == 0 {
                PlaySE(SE_WARP_OUT);
                (*crate::gl!(sTradeAnim)).state += 1;
            }
            {
                let gl_i111 = (*crate::gl!(sTradeAnim)).connectionSpriteId1;
                crate::gl!(gSprites[gl_i111].y2 -= 3)
            };
            {
                let gl_i112 = (*crate::gl!(sTradeAnim)).connectionSpriteId2;
                crate::gl!(gSprites[gl_i112].y2 += 3)
            };
        }
        STATE_CROSSING_LINK_MONS_ENTER => {
            {
                let gl_i113 = (*crate::gl!(sTradeAnim)).connectionSpriteId1;
                crate::gl!(gSprites[gl_i113].y2 -= 3)
            };
            {
                let gl_i114 = (*crate::gl!(sTradeAnim)).connectionSpriteId2;
                crate::gl!(gSprites[gl_i114].y2 += 3)
            };
            if ({
                let gl_i115 = (*crate::gl!(sTradeAnim)).connectionSpriteId1;
                crate::gl!(gSprites[gl_i115].y2)
            }) <= -90
            {
                {
                    let gl_i116 = (*crate::gl!(sTradeAnim)).connectionSpriteId1;
                    crate::gl!(gSprites[gl_i116].data[1] = 1)
                };
                {
                    let gl_i117 = (*crate::gl!(sTradeAnim)).connectionSpriteId2;
                    crate::gl!(gSprites[gl_i117].data[1] = 1)
                };
                (*crate::gl!(sTradeAnim)).state += 1;
                CreateTask(Some(Task_OpenCenterWhiteColumn), 5);
            }
        }
        STATE_CROSSING_BLEND_WHITE_1 => {
            BlendPalettes(0x8, 16, 65535);
            (*crate::gl!(sTradeAnim)).state += 1;
        }
        STATE_CROSSING_BLEND_WHITE_2 => {
            BlendPalettes(0x8, 16, 65535);
            (*crate::gl!(sTradeAnim)).state += 1;
        }
        STATE_CROSSING_BLEND_WHITE_3 => {
            BlendPalettes(0x8, 16, 65535);
            (*crate::gl!(sTradeAnim)).state += 1;
        }
        STATE_CROSSING_CREATE_MON_PICS => {
            if IsMonSpriteNotFlipped((*crate::gl!(sTradeAnim)).monSpecies[0]) == 0 {
                {
                    let gl_v = sAffineAnims_CrossingMonPics.as_ptr().cast_mut();
                    let gl_i118 = (*crate::gl!(sTradeAnim)).monSpriteIds[0];
                    crate::gl!(gSprites[gl_i118].affineAnims = gl_v)
                };
                (*gSprites.as_ptr())[(*crate::gl!(sTradeAnim)).monSpriteIds[0]]
                    .oam
                    .set_affineMode(ST_OAM_AFFINE_DOUBLE);
                CalcCenterToCornerVec(
                    {
                        let gl_i119 = (*crate::gl!(sTradeAnim)).monSpriteIds[0];
                        crate::gl!(&raw gSprites[gl_i119])
                    },
                    TRADE_PLAYER,
                    ST_OAM_AFFINE_DOUBLE as u8,
                    ST_OAM_AFFINE_DOUBLE as u8,
                );
                StartSpriteAffineAnim(
                    {
                        let gl_i120 = (*crate::gl!(sTradeAnim)).monSpriteIds[0];
                        crate::gl!(&raw gSprites[gl_i120])
                    },
                    0,
                );
            } else {
                StartSpriteAffineAnim(
                    {
                        let gl_i121 = (*crate::gl!(sTradeAnim)).monSpriteIds[0];
                        crate::gl!(&raw gSprites[gl_i121])
                    },
                    0,
                );
            }
            StartSpriteAffineAnim(
                {
                    let gl_i122 = (*crate::gl!(sTradeAnim)).monSpriteIds[1];
                    crate::gl!(&raw gSprites[gl_i122])
                },
                0,
            );
            {
                let gl_i123 = (*crate::gl!(sTradeAnim)).monSpriteIds[0];
                crate::gl!(gSprites[gl_i123].x = 40)
            };
            {
                let gl_i124 = (*crate::gl!(sTradeAnim)).monSpriteIds[1];
                crate::gl!(gSprites[gl_i124].x = 200)
            };
            {
                let gl_i125 = (*crate::gl!(sTradeAnim)).monSpriteIds[0];
                crate::gl!(gSprites[gl_i125].y = 192)
            };
            {
                let gl_i126 = (*crate::gl!(sTradeAnim)).monSpriteIds[1];
                crate::gl!(gSprites[gl_i126].y = -32)
            };
            (*gSprites.as_ptr())[(*crate::gl!(sTradeAnim)).monSpriteIds[0]].set_invisible(0);
            (*gSprites.as_ptr())[(*crate::gl!(sTradeAnim)).monSpriteIds[1]]
                .set_invisible(FALSE as u16);
            (*crate::gl!(sTradeAnim)).state += 1;
        }
        STATE_CROSSING_MON_PICS_MOVE => {
            {
                let gl_i127 = (*crate::gl!(sTradeAnim)).monSpriteIds[0];
                crate::gl!(gSprites[gl_i127].y2 -= 3)
            };
            {
                let gl_i128 = (*crate::gl!(sTradeAnim)).monSpriteIds[1];
                crate::gl!(gSprites[gl_i128].y2 += 3)
            };
            if ({
                let gl_i129 = (*crate::gl!(sTradeAnim)).monSpriteIds[0];
                crate::gl!(gSprites[gl_i129].y2)
            }) < -160
                && ({
                    let gl_i130 = (*crate::gl!(sTradeAnim)).monSpriteIds[0];
                    crate::gl!(gSprites[gl_i130].y2)
                }) >= -163
            {
                PlaySE(SE_WARP_IN);
            }
            if ({
                let gl_i131 = (*crate::gl!(sTradeAnim)).monSpriteIds[0];
                crate::gl!(gSprites[gl_i131].y2)
            }) < -222
            {
                {
                    let gl_i132 = (*crate::gl!(sTradeAnim)).connectionSpriteId1;
                    crate::gl!(gSprites[gl_i132].data[1] = 0)
                };
                {
                    let gl_i133 = (*crate::gl!(sTradeAnim)).connectionSpriteId2;
                    crate::gl!(gSprites[gl_i133].data[1] = 0)
                };
                (*crate::gl!(sTradeAnim)).state += 1;
                (*gSprites.as_ptr())[(*crate::gl!(sTradeAnim)).monSpriteIds[0]]
                    .set_invisible(TRUE as u16);
                (*gSprites.as_ptr())[(*crate::gl!(sTradeAnim)).monSpriteIds[1]].set_invisible(1);
                CreateTask(Some(Task_CloseCenterWhiteColumn), 5);
            }
        }
        STATE_CROSSING_LINK_MONS_EXIT => {
            {
                let gl_i134 = (*crate::gl!(sTradeAnim)).connectionSpriteId1;
                crate::gl!(gSprites[gl_i134].y2 -= 3)
            };
            {
                let gl_i135 = (*crate::gl!(sTradeAnim)).connectionSpriteId2;
                crate::gl!(gSprites[gl_i135].y2 += 3)
            };
            if ({
                let gl_i136 = (*crate::gl!(sTradeAnim)).connectionSpriteId1;
                crate::gl!(gSprites[gl_i136].y2)
            }) <= -222
            {
                BeginNormalPaletteFade(PALETTES_ALL, -1, 0, 16, 0);
                (*crate::gl!(sTradeAnim)).state += 1;
                DestroySprite({
                    let gl_i137 = (*crate::gl!(sTradeAnim)).connectionSpriteId1;
                    crate::gl!(&raw gSprites[gl_i137])
                });
                DestroySprite({
                    let gl_i138 = (*crate::gl!(sTradeAnim)).connectionSpriteId2;
                    crate::gl!(&raw gSprites[gl_i138])
                });
            }
        }
        STATE_CREATE_LINK_MON_ARRIVING => {
            if (*gPaletteFade.as_ptr()).active() == 0 {
                (*crate::gl!(sTradeAnim)).state += 1;
                SetTradeSequenceBgGpuRegs(1);
                (*crate::gl!(sTradeAnim)).bg1vofs = 166;
                SetTradeSequenceBgGpuRegs(3);
                (*crate::gl!(sTradeAnim)).bg2vofs = 412;
                (*crate::gl!(sTradeAnim)).connectionSpriteId1 = CreateSprite(
                    (&raw const *sSpriteTemplate_LinkMonGlow).cast_mut(),
                    120,
                    -20,
                    3,
                );
                {
                    let gl_i139 = (*crate::gl!(sTradeAnim)).connectionSpriteId1;
                    crate::gl!(gSprites[gl_i139].callback = Some(SpriteCB_LinkMonGlowWireless))
                };
                (*crate::gl!(sTradeAnim)).connectionSpriteId2 = CreateSprite(
                    (&raw const *sSpriteTemplate_LinkMonShadow).cast_mut(),
                    120,
                    -20,
                    0,
                );
                StartSpriteAnim(
                    {
                        let gl_i140 = (*crate::gl!(sTradeAnim)).connectionSpriteId2;
                        crate::gl!(&raw gSprites[gl_i140])
                    },
                    ANIM_LINKMON_SMALL,
                );
            }
        }
        STATE_FADE_OUT_TO_GBA_RECV => {
            BeginNormalPaletteFade(PALETTES_ALL, -1, 16, 0, 0);
            (*crate::gl!(sTradeAnim)).state += 1;
        }
        STATE_WAIT_FADE_OUT_TO_GBA_RECV => {
            SetGpuReg(0x0, 4672);
            if (*gPaletteFade.as_ptr()).active() == 0 {
                (*crate::gl!(sTradeAnim)).state += 1;
            }
        }
        STATE_LINK_MON_TRAVEL_IN => {
            {
                let gl_i141 = (*crate::gl!(sTradeAnim)).connectionSpriteId1;
                crate::gl!(gSprites[gl_i141].y2 += 4)
            };
            {
                let gl_i142 = (*crate::gl!(sTradeAnim)).connectionSpriteId2;
                crate::gl!(gSprites[gl_i142].y2 += 4)
            };
            if ({
                let gl_i143 = (*crate::gl!(sTradeAnim)).connectionSpriteId1;
                crate::gl!(gSprites[gl_i143].y2)
            }) as i32
                + ({
                    let gl_i144 = (*crate::gl!(sTradeAnim)).connectionSpriteId1;
                    crate::gl!(gSprites[gl_i144].y)
                }) as i32
                == 64
            {
                (*crate::gl!(sTradeAnim)).state = STATE_PAN_TO_GBA_WIRELESS;
                (*crate::gl!(sTradeAnim)).timer = 0;
            }
        }
        STATE_PAN_TO_GBA_WIRELESS => {
            SetGpuReg(0x0, 5696);
            (*crate::gl!(sTradeAnim)).bg1vofs += 3;
            (*crate::gl!(sTradeAnim)).bg2vofs += 3;
            if ({
                (*crate::gl!(sTradeAnim)).timer += 1;
                (*crate::gl!(sTradeAnim)).timer
            }) == 10
            {
                let taskId: u8 = CreateTask(Some(Task_AnimateWirelessSignal), 5);
                task_set(taskId, tSignalComingBack, TRUE as i16);
            }
            if (*crate::gl!(sTradeAnim)).bg1vofs > 316 {
                (*crate::gl!(sTradeAnim)).bg1vofs = 316;
                (*crate::gl!(sTradeAnim)).state += 1;
            }
        }
        STATE_DESTROY_LINK_MON_WIRELESS => {
            DestroySprite({
                let gl_i145 = (*crate::gl!(sTradeAnim)).connectionSpriteId1;
                crate::gl!(&raw gSprites[gl_i145])
            });
            DestroySprite({
                let gl_i146 = (*crate::gl!(sTradeAnim)).connectionSpriteId2;
                crate::gl!(&raw gSprites[gl_i146])
            });
            (*crate::gl!(sTradeAnim)).state += 1;
            (*crate::gl!(sTradeAnim)).timer = 0;
        }
        STATE_WAIT_WIRELESS_SIGNAL_RECV => {
            if FuncIsActiveTask(Some(Task_AnimateWirelessSignal)) == 0 {
                (*crate::gl!(sTradeAnim)).state = STATE_LINK_MON_ARRIVED_DELAY;
                (*crate::gl!(sTradeAnim)).timer = 0;
            }
        }
        STATE_LINK_MON_ARRIVED_DELAY => {
            if ({
                (*crate::gl!(sTradeAnim)).timer += 1;
                (*crate::gl!(sTradeAnim)).timer
            }) == 10
            {
                (*crate::gl!(sTradeAnim)).state += 1;
            }
        }
        STATE_MOVE_GBA_TO_CENTER => {
            if ({
                (*crate::gl!(sTradeAnim)).bg1vofs += 1;
                (*crate::gl!(sTradeAnim)).bg1vofs
            }) > 348
            {
                (*crate::gl!(sTradeAnim)).bg1vofs = 348;
                (*crate::gl!(sTradeAnim)).state += 1;
            }
        }
        STATE_GBA_FLASH_RECV => {
            (*crate::gl!(sTradeAnim)).connectionSpriteId2 = CreateSprite(
                (&raw const *sSpriteTemplate_GbaScreenFlash_Long).cast_mut(),
                120,
                80,
                0,
            );
            (*crate::gl!(sTradeAnim)).state = STATE_GBA_STOP_FLASH_RECV;
        }
        STATE_GBA_STOP_FLASH_RECV => {
            if (*gSprites.as_ptr())[(*crate::gl!(sTradeAnim)).connectionSpriteId2].animEnded() != 0
            {
                DestroySprite({
                    let gl_i147 = (*crate::gl!(sTradeAnim)).connectionSpriteId2;
                    crate::gl!(&raw gSprites[gl_i147])
                });
                SetTradeSequenceBgGpuRegs(6);
                (*crate::gl!(sTradeAnim)).state += 1;
                PlaySE(SE_M_SAND_ATTACK);
            }
        }
        STATE_GBA_ZOOM_IN => {
            if (*crate::gl!(sTradeAnim)).gbaScale < 0x400 {
                (*crate::gl!(sTradeAnim)).gbaScale += 0x34;
            } else {
                (*crate::gl!(sTradeAnim)).gbaScale = 0x400;
                (*crate::gl!(sTradeAnim)).state += 1;
            }
            (*crate::gl!(sTradeAnim)).sXY =
                div_i32(0x8000, (*crate::gl!(sTradeAnim)).gbaScale as i32) as u16;
        }
        STATE_FADE_OUT_TO_NEW_MON => {
            BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 16, 0);
            (*crate::gl!(sTradeAnim)).state = STATE_WAIT_FADE_OUT_TO_NEW_MON;
        }
        STATE_WAIT_FADE_OUT_TO_NEW_MON => {
            if (*gPaletteFade.as_ptr()).active() == 0 {
                SetTradeSequenceBgGpuRegs(5);
                SetTradeSequenceBgGpuRegs(7);
                (*gPaletteFade.as_ptr()).set_bufferTransferDisabled(TRUE as u16);
                (*crate::gl!(sTradeAnim)).state += 1;
            }
        }
        STATE_FADE_IN_TO_NEW_MON => {
            (*gPaletteFade.as_ptr()).set_bufferTransferDisabled(FALSE as u16);
            BeginNormalPaletteFade(PALETTES_ALL, 0, 16, 0, 0);
            (*crate::gl!(sTradeAnim)).state += 1;
        }
        STATE_WAIT_FADE_IN_TO_NEW_MON => {
            SetGpuReg(0x0, 5184);
            if (*gPaletteFade.as_ptr()).active() == 0 {
                (*crate::gl!(sTradeAnim)).state += 1;
            }
        }
        STATE_POKEBALL_ARRIVE => {
            (*crate::gl!(sTradeAnim)).bouncingPokeballSpriteId = CreateSprite(
                (&raw const *sSpriteTemplate_Pokeball).cast_mut(),
                120,
                -8,
                0,
            );
            {
                let gl_i148 = (*crate::gl!(sTradeAnim)).bouncingPokeballSpriteId;
                crate::gl!(gSprites[gl_i148].data[3] = 74)
            };
            {
                let gl_i149 = (*crate::gl!(sTradeAnim)).bouncingPokeballSpriteId;
                crate::gl!(gSprites[gl_i149].callback = Some(SpriteCB_BouncingPokeballArrive))
            };
            StartSpriteAnim(
                {
                    let gl_i150 = (*crate::gl!(sTradeAnim)).bouncingPokeballSpriteId;
                    crate::gl!(&raw gSprites[gl_i150])
                },
                1,
            );
            StartSpriteAffineAnim(
                {
                    let gl_i151 = (*crate::gl!(sTradeAnim)).bouncingPokeballSpriteId;
                    crate::gl!(&raw gSprites[gl_i151])
                },
                2,
            );
            BlendPalettes(
                shl_i32(
                    1,
                    16 + (*gSprites.as_ptr())[(*crate::gl!(sTradeAnim)).bouncingPokeballSpriteId]
                        .oam
                        .paletteNum() as u32,
                ) as u32,
                16,
                65535,
            );
            (*crate::gl!(sTradeAnim)).state += 1;
            (*crate::gl!(sTradeAnim)).timer = 0;
        }
        STATE_FADE_POKEBALL_TO_NORMAL => {
            BeginNormalPaletteFade(
                shl_i32(
                    1,
                    16 + (*gSprites.as_ptr())[(*crate::gl!(sTradeAnim)).bouncingPokeballSpriteId]
                        .oam
                        .paletteNum() as u32,
                ) as u32,
                1,
                16,
                0,
                65535,
            );
            (*crate::gl!(sTradeAnim)).state += 1;
        }
        STATE_POKEBALL_ARRIVE_WAIT => {
            if ({
                let gl_i152 = (*crate::gl!(sTradeAnim)).bouncingPokeballSpriteId;
                crate::gl!(gSprites[gl_i152].callback)
            }) == Some(SpriteCallbackDummy as unsafe fn(*mut Sprite))
            {
                HandleLoadSpecialPokePic_2(
                    (&raw const (*(&raw const crate::data::data_tables::gMonFrontPicTable)
                        .cast::<CArray<CompressedSpriteSheet, 0>>())
                        [(*crate::gl!(sTradeAnim)).monSpecies[1]])
                        .cast_mut(),
                    (*crate::gl!(gMonSpritesGfxPtr)).sprites.ptr[3],
                    (*crate::gl!(sTradeAnim)).monSpecies[1] as i32,
                    (*crate::gl!(sTradeAnim)).monPersonalities[1],
                );
                (*crate::gl!(sTradeAnim)).state += 1;
            }
        }
        STATE_SHOW_NEW_MON => {
            {
                let gl_i153 = (*crate::gl!(sTradeAnim)).monSpriteIds[1];
                crate::gl!(gSprites[gl_i153].x = 120)
            };
            {
                let gl_v = (*(&raw const crate::data::data_tables::gMonFrontPicCoords)
                    .cast::<CArray<MonCoords, 0>>())[(*crate::gl!(sTradeAnim)).monSpecies[1]]
                    .y_offset as i16
                    + 60;
                let gl_i154 = (*crate::gl!(sTradeAnim)).monSpriteIds[1];
                crate::gl!(gSprites[gl_i154].y = gl_v)
            };
            {
                let gl_i155 = (*crate::gl!(sTradeAnim)).monSpriteIds[1];
                crate::gl!(gSprites[gl_i155].x2 = 0)
            };
            {
                let gl_i156 = (*crate::gl!(sTradeAnim)).monSpriteIds[1];
                crate::gl!(gSprites[gl_i156].y2 = 0)
            };
            StartSpriteAnim(
                {
                    let gl_i157 = (*crate::gl!(sTradeAnim)).monSpriteIds[1];
                    crate::gl!(&raw gSprites[gl_i157])
                },
                0,
            );
            CreatePokeballSpriteToReleaseMon(
                (*crate::gl!(sTradeAnim)).monSpriteIds[1],
                (*gSprites.as_ptr())[(*crate::gl!(sTradeAnim)).monSpriteIds[1]]
                    .oam
                    .paletteNum() as u8,
                120,
                84,
                2,
                1,
                20,
                0xfffff,
                (*crate::gl!(sTradeAnim)).monSpecies[1],
            );
            FreeSpriteOamMatrix({
                let gl_i158 = (*crate::gl!(sTradeAnim)).bouncingPokeballSpriteId;
                crate::gl!(&raw gSprites[gl_i158])
            });
            DestroySprite({
                let gl_i159 = (*crate::gl!(sTradeAnim)).bouncingPokeballSpriteId;
                crate::gl!(&raw gSprites[gl_i159])
            });
            (*crate::gl!(sTradeAnim)).state += 1;
        }
        STATE_NEW_MON_MSG => {
            SetGpuReg(0x0, 5440);
            StringExpandPlaceholders(
                gStringVar4.as_mut_ptr(),
                (*(&raw const crate::data::strings::gText_XSentOverY).cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut(),
            );
            DrawTextOnTradeWindow(0, gStringVar4.as_mut_ptr(), 0);
            (*crate::gl!(sTradeAnim)).state = STATE_DELAY_FOR_MON_ANIM;
            (*crate::gl!(sTradeAnim)).timer = 0;
        }
        STATE_DELAY_FOR_MON_ANIM => {
            if ({
                (*crate::gl!(sTradeAnim)).timer += 1;
                (*crate::gl!(sTradeAnim)).timer
            }) > 60
            {
                (*crate::gl!(sTradeAnim)).state = STATE_WAIT_FOR_MON_CRY;
                (*crate::gl!(sTradeAnim)).timer = 0;
            }
        }
        STATE_WAIT_FOR_MON_CRY => {
            if IsCryFinished() != 0 {
                (*crate::gl!(sTradeAnim)).state = STATE_TAKE_CARE_OF_MON;
            }
        }
        STATE_TAKE_CARE_OF_MON => {
            if ({
                (*crate::gl!(sTradeAnim)).timer += 1;
                (*crate::gl!(sTradeAnim)).timer
            }) == 10
            {
                PlayFanfare(MUS_EVOLVED);
            }
            if (*crate::gl!(sTradeAnim)).timer == 250 {
                (*crate::gl!(sTradeAnim)).state += 1;
                StringExpandPlaceholders(
                    gStringVar4.as_mut_ptr(),
                    (*(&raw const crate::data::strings::gText_TakeGoodCareOfX)
                        .cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut(),
                );
                DrawTextOnTradeWindow(0, gStringVar4.as_mut_ptr(), 0);
                (*crate::gl!(sTradeAnim)).timer = 0;
            }
        }
        STATE_AFTER_NEW_MON_DELAY => {
            if ({
                (*crate::gl!(sTradeAnim)).timer += 1;
                (*crate::gl!(sTradeAnim)).timer
            }) == 60
            {
                (*crate::gl!(sTradeAnim)).state += 1;
            }
        }
        STATE_CHECK_RIBBONS => {
            CheckPartnersMonForRibbons();
            (*crate::gl!(sTradeAnim)).state += 1;
        }
        STATE_END_LINK_TRADE => {
            if (*crate::gl!(sTradeAnim)).isLinkTrade != 0 {
                return TRUE;
            } else if crate::gl!(gMain.newKeys) as i32 & A_BUTTON != 0 {
                (*crate::gl!(sTradeAnim)).state += 1;
            }
        }
        STATE_TRY_EVOLUTION => {
            TradeMons(gSpecialVar_0x8005 as u8, 0);
            crate::gl!(gCB2_AfterEvolution = Some(CB2_InGameTrade));
            evoTarget = GetEvolutionTargetSpecies(
                {
                    let gl_i160 = crate::gl!(gSelectedTradeMonPositions[0]);
                    crate::gl!(&raw gPlayerParty[gl_i160])
                },
                EVO_MODE_TRADE,
                ITEM_NONE,
            );
            if evoTarget != SPECIES_NONE {
                TradeEvolutionScene(
                    {
                        let gl_i161 = crate::gl!(gSelectedTradeMonPositions[0]);
                        crate::gl!(&raw gPlayerParty[gl_i161])
                    },
                    evoTarget,
                    (*crate::gl!(sTradeAnim)).monSpriteIds[1],
                    crate::gl!(gSelectedTradeMonPositions[0]),
                );
            }
            (*crate::gl!(sTradeAnim)).state += 1;
        }
        STATE_FADE_OUT_END => {
            BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 16, 0);
            (*crate::gl!(sTradeAnim)).state += 1;
        }
        STATE_WAIT_FADE_OUT_END if (*gPaletteFade.as_ptr()).active() == 0 => {
            PlayNewMapMusic((*crate::gl!(sTradeAnim)).cachedMapMusic);
            if !(*sTradeAnim.as_ptr()).is_null() {
                FreeAllWindowBuffers();
                Free(GetBgTilemapBuffer(3));
                Free(GetBgTilemapBuffer(1));
                Free(GetBgTilemapBuffer(0));
                FreeMonSpritesGfx();
                Free(crate::gl!(sTradeAnim) as *mut c_void);
                crate::gl!(sTradeAnim = null_mut());
            }
            SetMainCallback2(Some(CB2_ReturnToField));
            BufferInGameTradeMonName();
        }
        _ => {}
    }
    FALSE
}
pub(crate) unsafe fn CB2_TryLinkTradeEvolution() {
    let mut evoTarget: u16 = 0;
    match crate::gl!(gMain.state) {
        0 => {
            crate::gl!(gMain.state = 4);
            gSoftResetDisabled.set(TRUE);
        }
        4 => {
            crate::gl!(gCB2_AfterEvolution = Some(CB2_SaveAndEndTrade));
            evoTarget = GetEvolutionTargetSpecies(
                {
                    let gl_i162 = crate::gl!(gSelectedTradeMonPositions[0]);
                    crate::gl!(&raw gPlayerParty[gl_i162])
                },
                EVO_MODE_TRADE,
                ITEM_NONE,
            );
            if evoTarget != SPECIES_NONE {
                TradeEvolutionScene(
                    {
                        let gl_i163 = crate::gl!(gSelectedTradeMonPositions[0]);
                        crate::gl!(&raw gPlayerParty[gl_i163])
                    },
                    evoTarget,
                    (*crate::gl!(sTradeAnim)).monSpriteIds[1],
                    crate::gl!(gSelectedTradeMonPositions[0]),
                );
            } else if IsWirelessTrade() != 0 {
                SetMainCallback2(Some(CB2_SaveAndEndWirelessTrade));
            } else {
                SetMainCallback2(Some(CB2_SaveAndEndTrade));
            }
            crate::gl!(gSelectedTradeMonPositions[0] = 255);
        }
        _ => {}
    }
    if HasLinkErrorOccurred() == 0 {
        RunTasks();
    }
    AnimateSprites();
    BuildOamBuffer();
    UpdatePaletteFade();
}
unsafe fn HandleLinkDataReceive() {
    TradeGetMultiplayerId();
    let recvStatus: u8 = GetBlockReceivedStatus();
    if recvStatus as i32 & 1 != 0 {
        if crate::gl!(gBlockRecvBuffer[0][0]) == LINKCMD_CONFIRM_FINISH_TRADE {
            SetMainCallback2(Some(CB2_TryLinkTradeEvolution));
        }
        if crate::gl!(gBlockRecvBuffer[0][0]) == LINKCMD_READY_FINISH_TRADE {
            (*crate::gl!(sTradeAnim)).playerFinishStatus = STATUS_READY;
        }
        ResetBlockReceivedFlag(0);
    }
    if recvStatus as i32 & 2 != 0 {
        if crate::gl!(gBlockRecvBuffer[1][0]) == LINKCMD_READY_FINISH_TRADE {
            (*crate::gl!(sTradeAnim)).partnerFinishStatus = STATUS_READY;
        }
        ResetBlockReceivedFlag(1);
    }
}
pub(crate) unsafe fn SpriteCB_BouncingPokeball(sprite: *mut Sprite) {
    (*sprite).y += (*sprite).data[0] / 10;
    (*sprite).data[5] += (*sprite).data[1];
    (*sprite).x = (*sprite).data[5] / 10;
    if (*sprite).y > 0x4c {
        (*sprite).y = 0x4c;
        (*sprite).data[0] = (-((*sprite).data[0] as i32 * (*sprite).data[2] as i32) / 100) as i16;
        (*sprite).data[3] += 1;
    }
    if (*sprite).x == 0x78 {
        (*sprite).data[1] = 0;
    }
    (*sprite).data[0] += (*sprite).data[4];
    if (*sprite).data[3] == 4 {
        (*sprite).data[7] = 1;
        (*sprite).callback = Some(SpriteCallbackDummy);
    }
}
pub(crate) unsafe fn SpriteCB_BouncingPokeballDepart(sprite: *mut Sprite) {
    (*sprite).y2 += sTradeBallVerticalVelocityTable[(*sprite).data[0]] as i16;
    if (*sprite).data[0] == 22 {
        PlaySE(SE_BALL_BOUNCE_1);
    }
    if ({
        (*sprite).data[0] += 1;
        (*sprite).data[0]
    }) == 44
    {
        PlaySE(SE_M_MEGA_KICK);
        (*sprite).callback = Some(SpriteCB_BouncingPokeballDepartEnd);
        (*sprite).data[0] = 0;
        BeginNormalPaletteFade(
            shl_i32(1, 16 + (*sprite).oam.paletteNum() as u32) as u32,
            -1,
            0,
            16,
            65535,
        );
    }
}
pub(crate) unsafe fn SpriteCB_BouncingPokeballDepartEnd(sprite: *mut Sprite) {
    if (*sprite).data[1] == 20 {
        StartSpriteAffineAnim(sprite, 1);
    }
    if ({
        (*sprite).data[1] += 1;
        (*sprite).data[1]
    }) > 20
    {
        (*sprite).y2 -= sTradeBallVerticalVelocityTable[(*sprite).data[0]] as i16;
        if ({
            (*sprite).data[0] += 1;
            (*sprite).data[0]
        }) == 23
        {
            DestroySprite(sprite);
            (*crate::gl!(sTradeAnim)).state = STATE_FADE_OUT_TO_GBA_SEND;
        }
    }
}
pub(crate) unsafe fn SpriteCB_BouncingPokeballArrive(sprite: *mut Sprite) {
    if (*sprite).data[2] == 0 {
        if ({
            (*sprite).y += 4;
            (*sprite).y
        }) > (*sprite).data[3]
        {
            (*sprite).data[2] += 1;
            (*sprite).data[0] = 0x16;
            PlaySE(SE_BALL_BOUNCE_1);
        }
    } else {
        if (*sprite).data[0] == 0x42 {
            PlaySE(SE_BALL_BOUNCE_2);
        }
        if (*sprite).data[0] == 0x5c {
            PlaySE(SE_BALL_BOUNCE_3);
        }
        if (*sprite).data[0] == 0x6b {
            PlaySE(SE_BALL_BOUNCE_4);
        }
        (*sprite).y2 += sTradeBallVerticalVelocityTable[(*sprite).data[0]] as i16;
        if ({
            (*sprite).data[0] += 1;
            (*sprite).data[0]
        }) == 0x6c
        {
            (*sprite).callback = Some(SpriteCallbackDummy);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe fn GetInGameTradeSpeciesInfo() -> u16 {
    let inGameTrade: *mut InGameTrade = (&raw const sIngameTrades
        [*(&raw const crate::ffi::gSpecialVar_0x8004)
            .cast::<u16>()
            .cast_mut()])
        .cast_mut();
    StringCopy(
        gStringVar1.as_mut_ptr(),
        (*(&raw const crate::data::data_tables::gSpeciesNames).cast::<CArray<CArray<u8, 11>, 0>>())
            [(*inGameTrade).requestedSpecies]
            .as_ptr()
            .cast_mut(),
    );
    StringCopy(
        (*gStringVar2.as_ptr()).as_mut_ptr(),
        (*(&raw const crate::data::data_tables::gSpeciesNames).cast::<CArray<CArray<u8, 11>, 0>>())
            [(*inGameTrade).species]
            .as_ptr()
            .cast_mut(),
    );
    (*inGameTrade).requestedSpecies
}
unsafe fn BufferInGameTradeMonName() {
    let mut nickname: CArray<u8, 32> = zeroed();
    let inGameTrade: *mut InGameTrade = (&raw const sIngameTrades
        [*(&raw const crate::ffi::gSpecialVar_0x8004)
            .cast::<u16>()
            .cast_mut()])
        .cast_mut();
    GetMonData3(
        {
            let gl_i164 = *(&raw const crate::ffi::gSpecialVar_0x8005)
                .cast::<u16>()
                .cast_mut();
            crate::gl!(&raw gPlayerParty[gl_i164])
        },
        MON_DATA_NICKNAME,
        nickname.as_mut_ptr(),
    );
    StringCopy_Nickname(gStringVar1.as_mut_ptr(), nickname.as_mut_ptr());
    StringCopy(
        (*gStringVar2.as_ptr()).as_mut_ptr(),
        (*(&raw const crate::data::data_tables::gSpeciesNames).cast::<CArray<CArray<u8, 11>, 0>>())
            [(*inGameTrade).species]
            .as_ptr()
            .cast_mut(),
    );
}
unsafe fn CreateInGameTradePokemonInternal(whichPlayerMon: u8, whichInGameTrade: u8) {
    let inGameTrade: *mut InGameTrade = (&raw const sIngameTrades[whichInGameTrade]).cast_mut();
    let level: u8 = GetMonData2(
        crate::gl!(&raw gPlayerParty[whichPlayerMon]),
        MON_DATA_LEVEL,
    ) as u8;
    let mut mail: Mail = zeroed();
    let mut metLocation: u8 = METLOC_IN_GAME_TRADE;
    let pokemon: *mut Pokemon = crate::gl!(&raw gEnemyParty[0]);
    CreateMon(
        pokemon,
        (*inGameTrade).species,
        level,
        USE_RANDOM_IVS,
        1,
        (*inGameTrade).personality,
        1,
        (*inGameTrade).otId,
    );
    SetMonData(
        pokemon,
        MON_DATA_HP_IV,
        &raw mut (*inGameTrade).ivs[0] as *mut c_void,
    );
    SetMonData(
        pokemon,
        MON_DATA_ATK_IV,
        &raw mut (*inGameTrade).ivs[1] as *mut c_void,
    );
    SetMonData(
        pokemon,
        MON_DATA_DEF_IV,
        &raw mut (*inGameTrade).ivs[2] as *mut c_void,
    );
    SetMonData(
        pokemon,
        MON_DATA_SPEED_IV,
        &raw mut (*inGameTrade).ivs[3] as *mut c_void,
    );
    SetMonData(
        pokemon,
        MON_DATA_SPATK_IV,
        &raw mut (*inGameTrade).ivs[4] as *mut c_void,
    );
    SetMonData(
        pokemon,
        MON_DATA_SPDEF_IV,
        &raw mut (*inGameTrade).ivs[5] as *mut c_void,
    );
    SetMonData(
        pokemon,
        MON_DATA_NICKNAME,
        (*inGameTrade).nickname.as_mut_ptr() as *mut c_void,
    );
    SetMonData(
        pokemon,
        MON_DATA_OT_NAME,
        (*inGameTrade).otName.as_mut_ptr() as *mut c_void,
    );
    SetMonData(
        pokemon,
        MON_DATA_OT_GENDER,
        &raw mut (*inGameTrade).otGender as *mut c_void,
    );
    SetMonData(
        pokemon,
        MON_DATA_ABILITY_NUM,
        &raw mut (*inGameTrade).abilityNum as *mut c_void,
    );
    SetMonData(
        pokemon,
        MON_DATA_BEAUTY,
        &raw mut (*inGameTrade).conditions[1] as *mut c_void,
    );
    SetMonData(
        pokemon,
        MON_DATA_CUTE,
        &raw mut (*inGameTrade).conditions[2] as *mut c_void,
    );
    SetMonData(
        pokemon,
        MON_DATA_COOL,
        &raw mut (*inGameTrade).conditions[0] as *mut c_void,
    );
    SetMonData(
        pokemon,
        MON_DATA_SMART,
        &raw mut (*inGameTrade).conditions[3] as *mut c_void,
    );
    SetMonData(
        pokemon,
        MON_DATA_TOUGH,
        &raw mut (*inGameTrade).conditions[4] as *mut c_void,
    );
    SetMonData(
        pokemon,
        MON_DATA_SHEEN,
        &raw mut (*inGameTrade).sheen as *mut c_void,
    );
    SetMonData(
        pokemon,
        MON_DATA_MET_LOCATION,
        &raw mut metLocation as *mut c_void,
    );
    let mut mailNum: u8 = 0;
    if (*inGameTrade).heldItem != ITEM_NONE {
        if ItemIsMail((*inGameTrade).heldItem) != 0 {
            GetInGameTradeMail(&raw mut mail, inGameTrade);
            crate::gl!(gTradeMail[0] = mail);
            SetMonData(pokemon, MON_DATA_MAIL, &raw mut mailNum as *mut c_void);
            SetMonData(
                pokemon,
                MON_DATA_HELD_ITEM,
                &raw mut (*inGameTrade).heldItem as *mut c_void,
            );
        } else {
            SetMonData(
                pokemon,
                MON_DATA_HELD_ITEM,
                &raw mut (*inGameTrade).heldItem as *mut c_void,
            );
        }
    }
    CalculateMonStats(crate::gl!(&raw gEnemyParty[0]));
}
unsafe fn GetInGameTradeMail(mail: *mut Mail, trade: *mut InGameTrade) {
    for i in 0..(MAIL_WORDS_COUNT as i32) {
        (*mail).words[i] = sIngameTradeMail[(*trade).mailNum][i];
    }
    StringCopy(
        (*mail).playerName.as_mut_ptr(),
        (*trade).otName.as_mut_ptr(),
    );
    PadNameString((*mail).playerName.as_mut_ptr(), CHAR_SPACE);
    (*mail).trainerId[0] = ((*trade).otId >> 24) as u8;
    (*mail).trainerId[1] = ((*trade).otId >> 16) as u8;
    (*mail).trainerId[2] = ((*trade).otId >> 8) as u8;
    (*mail).trainerId[3] = (*trade).otId as u8;
    (*mail).species = (*trade).species;
    (*mail).itemId = (*trade).heldItem;
}
#[unsafe(no_mangle)]
pub unsafe fn GetTradeSpecies() -> u16 {
    if GetMonData2(
        {
            let gl_i165 = *(&raw const crate::ffi::gSpecialVar_0x8005)
                .cast::<u16>()
                .cast_mut();
            crate::gl!(&raw gPlayerParty[gl_i165])
        },
        MON_DATA_IS_EGG,
    ) != 0
    {
        return SPECIES_NONE;
    }
    GetMonData2(
        {
            let gl_i166 = *(&raw const crate::ffi::gSpecialVar_0x8005)
                .cast::<u16>()
                .cast_mut();
            crate::gl!(&raw gPlayerParty[gl_i166])
        },
        MON_DATA_SPECIES,
    ) as u16
}
#[unsafe(no_mangle)]
pub unsafe fn CreateInGameTradePokemon() {
    CreateInGameTradePokemonInternal(gSpecialVar_0x8005 as u8, gSpecialVar_0x8004 as u8);
}
pub(crate) unsafe fn CB2_UpdateLinkTrade() {
    if DoTradeAnim() == TRUE {
        DestroySprite({
            let gl_i167 = (*crate::gl!(sTradeAnim)).monSpriteIds[0];
            crate::gl!(&raw gSprites[gl_i167])
        });
        FreeSpriteOamMatrix({
            let gl_i168 = (*crate::gl!(sTradeAnim)).monSpriteIds[1];
            crate::gl!(&raw gSprites[gl_i168])
        });
        TradeMons(
            crate::gl!(gSelectedTradeMonPositions[0]),
            (crate::gl!(gSelectedTradeMonPositions[1]) as i32 % 6) as u8,
        );
        if IsWirelessTrade() == 0 {
            (*crate::gl!(sTradeAnim)).linkData[0] = LINKCMD_READY_FINISH_TRADE;
            (*crate::gl!(sTradeAnim)).scheduleLinkTransfer = 1;
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
pub(crate) unsafe fn CB2_WaitTradeComplete() {
    let mpId: u8 = TradeGetMultiplayerId() as u8;
    if IsWirelessTrade() != 0 {
        SetMainCallback2(Some(CB2_TryLinkTradeEvolution));
    } else {
        HandleLinkDataReceive();
        if mpId == 0
            && (*crate::gl!(sTradeAnim)).playerFinishStatus == STATUS_READY
            && (*crate::gl!(sTradeAnim)).partnerFinishStatus == STATUS_READY
        {
            (*crate::gl!(sTradeAnim)).linkData[0] = LINKCMD_CONFIRM_FINISH_TRADE;
            SendBlock(
                BitmaskAllOtherLinkPlayers(),
                (*crate::gl!(sTradeAnim)).linkData.as_mut_ptr() as *mut c_void,
                20,
            );
            (*crate::gl!(sTradeAnim)).playerFinishStatus = STATUS_CANCEL;
            (*crate::gl!(sTradeAnim)).partnerFinishStatus = STATUS_CANCEL;
        }
    }
    RunTasks();
    AnimateSprites();
    BuildOamBuffer();
    UpdatePaletteFade();
}
pub(crate) unsafe fn CB2_SaveAndEndTrade() {
    match crate::gl!(gMain.state) {
        0 => {
            crate::gl!(gMain.state += 1);
            StringExpandPlaceholders(
                gStringVar4.as_mut_ptr(),
                (*(&raw const crate::data::strings::gText_CommunicationStandby5)
                    .cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
            );
            DrawTextOnTradeWindow(0, gStringVar4.as_mut_ptr(), 0);
        }
        1 => {
            SetTradeLinkStandbyCallback(0);
            crate::gl!(gMain.state = 100);
            (*crate::gl!(sTradeAnim)).timer = 0;
        }
        100 => {
            if ({
                (*crate::gl!(sTradeAnim)).timer += 1;
                (*crate::gl!(sTradeAnim)).timer
            }) > 180
            {
                crate::gl!(gMain.state = 101);
                (*crate::gl!(sTradeAnim)).timer = 0;
            }
            if _IsLinkTaskFinished() != 0 {
                crate::gl!(gMain.state = 2);
            }
        }
        101 => {
            if _IsLinkTaskFinished() != 0 {
                crate::gl!(gMain.state = 2);
            }
        }
        2 => {
            crate::gl!(gMain.state = 50);
            StringExpandPlaceholders(
                gStringVar4.as_mut_ptr(),
                (*crate::asmdata::gText_SavingDontTurnOffPower.cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut(),
            );
            DrawTextOnTradeWindow(0, gStringVar4.as_mut_ptr(), 0);
        }
        50 => {
            if InUnionRoom() == 0 {
                IncrementGameStat(GAME_STAT_POKEMON_TRADES);
            }
            if gWirelessCommType.get() != 0 {
                MysteryGift_TryIncrementStat(CARD_STAT_NUM_TRADES, {
                    let gl_i169 = GetMultiplayerId() as i32 ^ 1;
                    crate::gl!(gLinkPlayers[gl_i169].trainerId)
                });
            }
            SetContinueGameWarpStatusToDynamicWarp();
            LinkFullSave_Init();
            crate::gl!(gMain.state += 1);
            (*crate::gl!(sTradeAnim)).timer = 0;
        }
        51 => {
            if ({
                (*crate::gl!(sTradeAnim)).timer += 1;
                (*crate::gl!(sTradeAnim)).timer
            }) == 5
            {
                crate::gl!(gMain.state += 1);
            }
        }
        52 => {
            if LinkFullSave_WriteSector() != 0 {
                ClearContinueGameWarpStatus2();
                crate::gl!(gMain.state = 4);
            } else {
                (*crate::gl!(sTradeAnim)).timer = 0;
                crate::gl!(gMain.state = 51);
            }
        }
        4 => {
            LinkFullSave_ReplaceLastSector();
            crate::gl!(gMain.state = 40);
            (*crate::gl!(sTradeAnim)).timer = 0;
        }
        40 => {
            if ({
                (*crate::gl!(sTradeAnim)).timer += 1;
                (*crate::gl!(sTradeAnim)).timer
            }) > 50
            {
                if GetMultiplayerId() == 0 {
                    (*crate::gl!(sTradeAnim)).timer = (Random() as i32 % 30) as u32;
                } else {
                    (*crate::gl!(sTradeAnim)).timer = 0;
                }
                crate::gl!(gMain.state = 41);
            }
        }
        41 => {
            if (*crate::gl!(sTradeAnim)).timer == 0 {
                SetTradeLinkStandbyCallback(1);
                crate::gl!(gMain.state = 42);
            } else {
                (*crate::gl!(sTradeAnim)).timer -= 1;
            }
        }
        42 => {
            if _IsLinkTaskFinished() != 0 {
                LinkFullSave_SetLastSectorSignature();
                crate::gl!(gMain.state = 5);
            }
        }
        5 => {
            if ({
                (*crate::gl!(sTradeAnim)).timer += 1;
                (*crate::gl!(sTradeAnim)).timer
            }) > 60
            {
                crate::gl!(gMain.state += 1);
                SetTradeLinkStandbyCallback(2);
            }
        }
        6 => {
            if _IsLinkTaskFinished() != 0 {
                BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 16, 0);
                crate::gl!(gMain.state += 1);
            }
        }
        7 => {
            if (*gPaletteFade.as_ptr()).active() == 0 {
                FadeOutBGM(3);
                crate::gl!(gMain.state += 1);
            }
        }
        8 => {
            if IsBGMStopped() == TRUE {
                if gWirelessCommType.get() != 0
                    && crate::gl!(gMain.savedCallback)
                        == Some(CB2_StartCreateTradeMenu as unsafe fn())
                {
                    SetTradeLinkStandbyCallback(3);
                } else {
                    SetCloseLinkCallback();
                }
                crate::gl!(gMain.state += 1);
            }
        }
        9 => {
            if gWirelessCommType.get() != 0
                && crate::gl!(gMain.savedCallback) == Some(CB2_StartCreateTradeMenu as unsafe fn())
            {
                if _IsLinkTaskFinished() != 0 {
                    gSoftResetDisabled.set(FALSE);
                    SetMainCallback2(Some(CB2_FreeTradeAnim));
                }
            } else if gReceivedRemoteLinkPlayers.get() == 0 {
                gSoftResetDisabled.set(FALSE);
                SetMainCallback2(Some(CB2_FreeTradeAnim));
            }
        }
        _ => {}
    }
    if HasLinkErrorOccurred() == 0 {
        RunTasks();
    }
    AnimateSprites();
    BuildOamBuffer();
    UpdatePaletteFade();
}
pub(crate) unsafe fn CB2_FreeTradeAnim() {
    if (*gPaletteFade.as_ptr()).active() == 0 {
        FreeAllWindowBuffers();
        Free(GetBgTilemapBuffer(3));
        Free(GetBgTilemapBuffer(1));
        Free(GetBgTilemapBuffer(0));
        FreeMonSpritesGfx();
        Free(crate::gl!(sTradeAnim) as *mut c_void);
        crate::gl!(sTradeAnim = null_mut());
        if gWirelessCommType.get() != 0 {
            DestroyWirelessStatusIndicatorSprite();
        }
        SetMainCallback2(crate::gl!(gMain.savedCallback));
    }
    RunTasks();
    AnimateSprites();
    BuildOamBuffer();
    UpdatePaletteFade();
}
#[unsafe(no_mangle)]
pub unsafe fn DoInGameTradeScene() {
    LockPlayerFieldControls();
    CreateTask(Some(Task_InGameTrade), 10);
    BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 16, 0);
}
pub(crate) unsafe fn Task_InGameTrade(taskId: u8) {
    if (*gPaletteFade.as_ptr()).active() == 0 {
        SetMainCallback2(Some(CB2_InitInGameTrade));
        crate::gl!(gFieldCallback = Some(FieldCB_ContinueScriptHandleMusic));
        DestroyTask(taskId);
    }
}
unsafe fn CheckPartnersMonForRibbons() {
    let mut numRibbons: u8 = 0;
    for i in 0..12u8 {
        numRibbons += GetMonData2(
            {
                let gl_i170 = crate::gl!(gSelectedTradeMonPositions[1]) as i32 % 6;
                crate::gl!(&raw gEnemyParty[gl_i170])
            },
            MON_DATA_CHAMPION_RIBBON + i as i32,
        ) as u8;
    }
    if numRibbons != 0 {
        FlagSet(FLAG_SYS_RIBBON_GET);
    }
}
pub unsafe fn LoadTradeAnimGfx() {
    TradeAnimInit_LoadGfx();
}
pub unsafe fn DrawTextOnTradeWindow(windowId: u8, str: *mut u8, speed: u8) {
    FillWindowPixelBuffer(windowId, 255);
    (*crate::gl!(sTradeAnim)).textColors[0] = TEXT_DYNAMIC_COLOR_6;
    (*crate::gl!(sTradeAnim)).textColors[1] = 0x1;
    (*crate::gl!(sTradeAnim)).textColors[2] = TEXT_COLOR_GREEN;
    AddTextPrinterParameterized4(
        windowId,
        FONT_NORMAL,
        0,
        2,
        0,
        0,
        (*crate::gl!(sTradeAnim)).textColors.as_mut_ptr(),
        speed as i8,
        str,
    );
    CopyWindowToVram(windowId, COPYWIN_FULL);
}
pub(crate) unsafe fn Task_AnimateWirelessSignal(taskId: u8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    let paletteIdx: u16 = sWirelessSignalAnimParams[*data][0] as u16 * 16;
    if *data.at(2) == 0 {
        if paletteIdx == 256 {
            LoadPalette(
                sWirelessSignalNone_Pal.as_ptr().cast_mut() as *mut c_void,
                48,
                32,
            );
        } else {
            LoadPalette(
                (&raw const sWirelessSignalSend_Pal[paletteIdx]).cast_mut() as *mut c_void,
                48,
                32,
            );
        }
    } else {
        if paletteIdx == 256 {
            LoadPalette(
                sWirelessSignalNone_Pal.as_ptr().cast_mut() as *mut c_void,
                48,
                32,
            );
        } else {
            LoadPalette(
                (&raw const sWirelessSignalRecv_Pal[paletteIdx]).cast_mut() as *mut c_void,
                48,
                32,
            );
        }
    }
    if sWirelessSignalAnimParams[*data][0] == 0 && *data.at(1) == 0 {
        PlaySE(SE_M_HEAL_BELL);
    }
    if *data.at(1) == sWirelessSignalAnimParams[*data][1] as i16 {
        *data += 1;
        *data.at(1) = 0;
        if sWirelessSignalAnimParams[*data][1] == 0xFF {
            DestroyTask(taskId);
        }
    } else {
        *data.at(1) += 1;
    }
}
pub(crate) unsafe fn Task_OpenCenterWhiteColumn(taskId: u8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    if *data == 0 {
        (*crate::gl!(sTradeAnim)).wirelessWinLeft = {
            (*crate::gl!(sTradeAnim)).wirelessWinRight = 120;
            (*crate::gl!(sTradeAnim)).wirelessWinRight
        };
        (*crate::gl!(sTradeAnim)).wirelessWinTop = 0;
        (*crate::gl!(sTradeAnim)).wirelessWinBottom = DISPLAY_HEIGHT as u8;
        SetGpuRegBits(REG_OFFSET_DISPCNT, DISPCNT_WIN0_ON);
        SetGpuReg(REG_OFFSET_WINOUT, WINOUT_WIN01_OBJ);
        SetGpuReg(REG_OFFSET_WININ, 19);
    }
    SetGpuReg(
        REG_OFFSET_WIN0H,
        (*crate::gl!(sTradeAnim)).wirelessWinRight as u16
            | ((*crate::gl!(sTradeAnim)).wirelessWinLeft as u16) << 8,
    );
    SetGpuReg(
        REG_OFFSET_WIN0V,
        (*crate::gl!(sTradeAnim)).wirelessWinBottom as u16
            | ((*crate::gl!(sTradeAnim)).wirelessWinTop as u16) << 8,
    );
    *data += 1;
    (*crate::gl!(sTradeAnim)).wirelessWinLeft -= 5;
    (*crate::gl!(sTradeAnim)).wirelessWinRight += 5;
    if (*crate::gl!(sTradeAnim)).wirelessWinLeft < 80 {
        DestroyTask(taskId);
    }
}
pub(crate) unsafe fn Task_CloseCenterWhiteColumn(taskId: u8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    if *data == 0 {
        (*crate::gl!(sTradeAnim)).wirelessWinLeft = 80;
        (*crate::gl!(sTradeAnim)).wirelessWinRight = 160;
        SetGpuReg(REG_OFFSET_WINOUT, WINOUT_WIN01_OBJ);
        SetGpuReg(REG_OFFSET_WININ, 19);
    }
    SetGpuReg(
        REG_OFFSET_WIN0H,
        (*crate::gl!(sTradeAnim)).wirelessWinRight as u16
            | ((*crate::gl!(sTradeAnim)).wirelessWinLeft as u16) << 8,
    );
    SetGpuReg(
        REG_OFFSET_WIN0V,
        (*crate::gl!(sTradeAnim)).wirelessWinBottom as u16
            | ((*crate::gl!(sTradeAnim)).wirelessWinTop as u16) << 8,
    );
    if (*crate::gl!(sTradeAnim)).wirelessWinLeft != 120 {
        *data += 1;
        (*crate::gl!(sTradeAnim)).wirelessWinLeft += 5;
        (*crate::gl!(sTradeAnim)).wirelessWinRight -= 5;
        if (*crate::gl!(sTradeAnim)).wirelessWinLeft > 115 {
            BlendPalettes(0x8, 0, 65535);
        }
    } else {
        ClearGpuRegBits(REG_OFFSET_DISPCNT, DISPCNT_WIN0_ON);
        DestroyTask(taskId);
    }
}
pub(crate) unsafe fn CB2_SaveAndEndWirelessTrade() {
    match crate::gl!(gMain.state) {
        0 => {
            crate::gl!(gMain.state = 1);
            StringExpandPlaceholders(
                gStringVar4.as_mut_ptr(),
                (*(&raw const crate::data::strings::gText_CommunicationStandby5)
                    .cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
            );
            DrawTextOnTradeWindow(0, gStringVar4.as_mut_ptr(), 0);
        }
        1 => {
            SetTradeLinkStandbyCallback(0);
            crate::gl!(gMain.state = 2);
            (*crate::gl!(sTradeAnim)).timer = 0;
        }
        2 => {
            if _IsLinkTaskFinished() != 0 {
                crate::gl!(gMain.state = 3);
                StringExpandPlaceholders(
                    gStringVar4.as_mut_ptr(),
                    (*crate::asmdata::gText_SavingDontTurnOffPower.cast::<CArray<u8, 0>>())
                        .as_ptr()
                        .cast_mut(),
                );
                DrawTextOnTradeWindow(0, gStringVar4.as_mut_ptr(), 0);
                IncrementGameStat(GAME_STAT_POKEMON_TRADES);
                LinkFullSave_Init();
                (*crate::gl!(sTradeAnim)).timer = 0;
            }
        }
        3 => {
            if ({
                (*crate::gl!(sTradeAnim)).timer += 1;
                (*crate::gl!(sTradeAnim)).timer
            }) == 5
            {
                crate::gl!(gMain.state = 4);
            }
        }
        4 => {
            if LinkFullSave_WriteSector() != 0 {
                crate::gl!(gMain.state = 5);
            } else {
                (*crate::gl!(sTradeAnim)).timer = 0;
                crate::gl!(gMain.state = 3);
            }
        }
        5 => {
            LinkFullSave_ReplaceLastSector();
            crate::gl!(gMain.state = 6);
            (*crate::gl!(sTradeAnim)).timer = 0;
        }
        6 => {
            if ({
                (*crate::gl!(sTradeAnim)).timer += 1;
                (*crate::gl!(sTradeAnim)).timer
            }) > 10
            {
                if GetMultiplayerId() == 0 {
                    (*crate::gl!(sTradeAnim)).timer = (Random() as i32 % 30) as u32;
                } else {
                    (*crate::gl!(sTradeAnim)).timer = 0;
                }
                crate::gl!(gMain.state = 7);
            }
        }
        7 => {
            if (*crate::gl!(sTradeAnim)).timer == 0 {
                SetTradeLinkStandbyCallback(1);
                crate::gl!(gMain.state = 8);
            } else {
                (*crate::gl!(sTradeAnim)).timer -= 1;
            }
        }
        8 => {
            if _IsLinkTaskFinished() != 0 {
                LinkFullSave_SetLastSectorSignature();
                crate::gl!(gMain.state = 9);
            }
        }
        9 => {
            if ({
                (*crate::gl!(sTradeAnim)).timer += 1;
                (*crate::gl!(sTradeAnim)).timer
            }) > 60
            {
                crate::gl!(gMain.state += 1);
                SetTradeLinkStandbyCallback(2);
            }
        }
        10 => {
            if _IsLinkTaskFinished() != 0 {
                FadeOutBGM(3);
                BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 16, 0);
                crate::gl!(gMain.state = 11);
            }
        }
        11 => {
            if (*gPaletteFade.as_ptr()).active() == 0 && IsBGMStopped() == TRUE {
                SetTradeLinkStandbyCallback(3);
                crate::gl!(gMain.state = 12);
            }
        }
        12 if _IsLinkTaskFinished() != 0 => {
            gSoftResetDisabled.set(FALSE);
            SetMainCallback2(Some(CB2_FreeTradeAnim));
        }
        _ => {}
    }
    if HasLinkErrorOccurred() == 0 {
        RunTasks();
    }
    AnimateSprites();
    BuildOamBuffer();
    UpdatePaletteFade();
}
