//! Translated from `src/trade.c` by tools/rustport/c2rs.py.
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

#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sMenuTextTileBuffer: *mut u8 = null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sMenuTextTileBuffers: CArray<*mut u8, 14> = unsafe { zeroed() };
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gTradeMail: CArray<Mail, 6> = unsafe { zeroed() };
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gSelectedTradeMonPositions: Aligned<CArray<u8, 2>> = Aligned(unsafe { zeroed() });
pub(crate) static mut sTradeMenu: *mut typeof___sTradeMenu_0_t = null_mut();
pub(crate) static mut sTradeAnim: *mut typeof___sTradeAnim_0_t = null_mut();

unsafe extern "C" {
    static gBattleTextboxPalette: CArray<u32, 0>;
    static gBattleTextboxTilemap: CArray<u32, 0>;
    static gBattleTextboxTiles: CArray<u32, 0>;
    static mut gBlockRecvBuffer: CArray<CArray<u16, 128>, 5>;
    static mut gBlockSendBuffer: CArray<u8, 256>;
    static mut gCB2_AfterEvolution: Option<unsafe extern "C" fn()>;
    static mut gDecompressionBuffer: CArray<u8, 16384>;
    static mut gEnemyParty: CArray<Pokemon, 6>;
    static mut gEnemyPartyCount: u8;
    static mut gFieldCallback: Option<unsafe extern "C" fn()>;
    static mut gLastViewedMonIndex: u8;
    static mut gLinkPlayers: CArray<LinkPlayer, 5>;
    static mut gLinkType: u16;
    static mut gMain: Main;
    static gMonFrontPicCoords: CArray<MonCoords, 0>;
    static gMonFrontPicTable: CArray<CompressedSpriteSheet, 0>;
    static mut gMonSpritesGfxPtr: *mut MonSpritesGfx;
    static gMoveNames: CArray<CArray<u8, 13>, 355>;
    static mut gMultiuseSpriteTemplate: SpriteTemplate;
    static mut gPaletteFade: PaletteFadeControl;
    static mut gPlayerCurrActivity: u8;
    static mut gPlayerParty: CArray<Pokemon, 6>;
    static mut gPlayerPartyCount: u8;
    static mut gReceivedRemoteLinkPlayers: u8;
    static mut gRfuSlotStatusNI: CArray<*mut RfuSlotStatusNI, 4>;
    static mut gSaveBlock1Ptr: *mut SaveBlock1;
    static mut gSaveBlock2Ptr: *mut SaveBlock2;
    static mut gSoftResetDisabled: u8;
    static mut gSpecialVar_0x8004: u16;
    static mut gSpecialVar_0x8005: u16;
    static gSpeciesInfo: CArray<SpeciesInfo, 0>;
    static gSpeciesNames: CArray<CArray<u8, 11>, 0>;
    static mut gSprites: CArray<Sprite, 65>;
    static gStandardMenuPalette: CArray<u16, 0>;
    static mut gStringVar1: CArray<u8, 256>;
    static mut gStringVar2: CArray<u8, 256>;
    static mut gStringVar3: CArray<u8, 256>;
    static mut gStringVar4: CArray<u8, 1000>;
    static mut gTasks: CArray<Task, 0>;
    static gText_ByeByeVar1: CArray<u8, 0>;
    static gText_CommunicationStandby5: CArray<u8, 0>;
    static gText_SavingDontTurnOffPower: CArray<u8, 0>;
    static gText_TakeGoodCareOfX: CArray<u8, 0>;
    static gText_XSentOverY: CArray<u8, 0>;
    static gText_XWillBeSentToY: CArray<u8, 0>;
    static gTradeGba2_Pal: CArray<u16, 0>;
    static gTradeGba_Gfx: CArray<u8, 0>;
    static gTradeMenuMonBox_Tilemap: CArray<u16, 0>;
    static gTradeMenu_Gfx: CArray<u8, 0>;
    static gTradeMenu_Pal: CArray<u16, 0>;
    static gTradeMenu_Tilemap: CArray<u16, 0>;
    static mut gWirelessCommType: u8;
    static mut lman: linkManagerTag;
    fn AddTextPrinterParameterized(
        a0: u8,
        a1: u8,
        a2: *mut u8,
        a3: u8,
        a4: u8,
        a5: u8,
        a6: Option<unsafe extern "C" fn(*mut TextPrinterTemplate, u16)>,
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
    fn Alloc(a0: u32) -> *mut c_void;
    fn AllocZeroed(a0: u32) -> *mut c_void;
    fn AllocateMonSpritesGfx();
    fn AnimateSprites();
    fn BeginNormalPaletteFade(a0: u32, a1: i8, a2: u8, a3: u8, a4: u16) -> u8;
    fn BitmaskAllOtherLinkPlayers() -> u8;
    fn BlendPalettes(a0: u32, a1: u8, a2: u16);
    fn BuildOamBuffer();
    fn CB2_LinkError();
    fn CB2_ReturnToField();
    fn CB2_ReturnToFieldFromMultiplayer();
    fn CalcCenterToCornerVec(a0: *mut Sprite, a1: u8, a2: u8, a3: u8);
    fn CalculateEnemyPartyCount() -> u8;
    fn CalculateMonStats(a0: *mut Pokemon);
    fn CalculatePlayerPartyCount() -> u8;
    fn ChangeBgX(a0: u8, a1: i32, a2: u8) -> i32;
    fn ChangeBgY(a0: u8, a1: i32, a2: u8) -> i32;
    fn CheckShouldAdvanceLinkState();
    fn ClearContinueGameWarpStatus2();
    fn ClearGpuRegBits(a0: u8, a1: u16);
    fn ClearMail(a0: *mut Mail);
    fn ClearWindowTilemap(a0: u8);
    fn CloseLink();
    fn CopyBgTilemapBufferToVram(a0: u8);
    fn CopyToBgTilemapBuffer(a0: u8, a1: *mut c_void, a2: u16, a3: u16);
    fn CopyToBgTilemapBufferRect_ChangePalette(
        a0: u8,
        a1: *mut c_void,
        a2: u8,
        a3: u8,
        a4: u8,
        a5: u8,
        a6: u8,
    );
    fn CopyWindowToVram(a0: u8, a1: u8);
    fn CreateMon(a0: *mut Pokemon, a1: u16, a2: u8, a3: u8, a4: u8, a5: u32, a6: u8, a7: u32);
    fn CreateMonIcon(
        a0: u16,
        a1: Option<unsafe extern "C" fn(*mut Sprite)>,
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
    fn CreateSprite(a0: *mut SpriteTemplate, a1: i16, a2: i16, a3: u8) -> u8;
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
    fn CreateYesNoMenu(a0: *mut WindowTemplate, a1: u16, a2: u8, a3: u8);
    fn DeactivateAllTextPrinters();
    fn DecompressAndLoadBgGfxUsingHeap(a0: u8, a1: *mut c_void, a2: u32, a3: u16, a4: u8);
    fn DestroySprite(a0: *mut Sprite);
    fn DestroyTask(a0: u8);
    fn DestroyTask_RfuIdle();
    fn DestroyWirelessStatusIndicatorSprite();
    fn DoBgAffineSet(
        a0: *mut BgAffineDstData,
        a1: u32,
        a2: u32,
        a3: i16,
        a4: i16,
        a5: i16,
        a6: i16,
        a7: u16,
    );
    fn DrawHeldItemIconsForTrade(a0: *mut u8, a1: *mut u8, a2: u8);
    fn DrawTextBorderOuter(a0: u8, a1: u16, a2: u8);
    fn DrawTextWindowAndBufferTiles(a0: *mut u8, a1: *mut c_void, a2: u8, a3: u8, a4: i32);
    fn FadeOutBGM(a0: u8);
    fn FieldCB_ContinueScriptHandleMusic();
    fn FillBgTilemapBufferRect(a0: u8, a1: u16, a2: u8, a3: u8, a4: u8, a5: u8, a6: u8);
    fn FillBgTilemapBufferRect_Palette0(a0: u8, a1: u16, a2: u8, a3: u8, a4: u8, a5: u8);
    fn FillWindowPixelBuffer(a0: u8, a1: u8);
    fn FlagSet(a0: u16) -> u8;
    fn Free(a0: *mut c_void);
    fn FreeAllSpritePalettes();
    fn FreeAllWindowBuffers();
    fn FreeMonSpritesGfx();
    fn FreeSpriteOamMatrix(a0: *mut Sprite);
    fn FuncIsActiveTask(a0: Option<unsafe extern "C" fn(u8)>) -> u8;
    fn GetBgTilemapBuffer(a0: u8) -> *mut c_void;
    fn GetBlockReceivedStatus() -> u8;
    fn GetCurrentMapMusic() -> u16;
    fn GetEvolutionTargetSpecies(a0: *mut Pokemon, a1: u8, a2: u16) -> u16;
    fn GetFontAttribute(a0: u8, a1: u8) -> u8;
    fn GetGpuReg(a0: u8) -> u16;
    fn GetHPBarLevel(a0: i16, a1: i16) -> u8;
    fn GetLinkPlayerCount() -> u8;
    fn GetLinkPlayerCount_2() -> u8;
    fn GetMonData2(a0: *mut Pokemon, a1: i32) -> u32;
    fn GetMonData3(a0: *mut Pokemon, a1: i32, a2: *mut u8) -> u32;
    fn GetMonGender(a0: *mut Pokemon) -> u8;
    fn GetMonSpritePalStruct(a0: *mut Pokemon) -> *mut CompressedSpritePalette;
    fn GetMultiplayerId() -> u8;
    fn GetSavedPlayerCount() -> u8;
    fn GetSetPokedexFlag(a0: u16, a1: u8) -> i8;
    fn GetStringCenterAlignXOffset(a0: i32, a1: *mut u8, a2: i32) -> i32;
    fn GetStringWidth(a0: u8, a1: *mut u8, a2: i16) -> i32;
    fn GiveMailToMon(a0: *mut Pokemon, a1: *mut Mail) -> u8;
    fn HandleLoadSpecialPokePic_2(
        a0: *mut CompressedSpriteSheet,
        a1: *mut c_void,
        a2: i32,
        a3: u32,
    );
    fn HandleLoadSpecialPokePic_DontHandleDeoxys(
        a0: *mut CompressedSpriteSheet,
        a1: *mut c_void,
        a2: i32,
        a3: u32,
    );
    fn HandleSetPokedexFlag(a0: u16, a1: u8, a2: u32);
    fn HasLinkErrorOccurred() -> u8;
    fn InUnionRoom() -> u32;
    fn IncrementGameStat(a0: u8);
    fn InitBgsFromTemplates(a0: u8, a1: *mut BgTemplate, a2: u8);
    fn InitMenuInUpperLeftCornerNormal(a0: u8, a1: u8, a2: u8) -> u8;
    fn InitWindows(a0: *mut WindowTemplate) -> u16;
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
    fn LZ77UnCompVram(a0: *mut u32, a1: *mut c_void);
    fn LZDecompressWram(a0: *mut u32, a1: *mut c_void);
    fn LinkFullSave_Init() -> u8;
    fn LinkFullSave_ReplaceLastSector() -> u8;
    fn LinkFullSave_SetLastSectorSignature() -> u8;
    fn LinkFullSave_WriteSector() -> u8;
    fn LoadBgTilemap(a0: u8, a1: *mut c_void, a2: u16, a3: u16) -> u16;
    fn LoadBgTiles(a0: u8, a1: *mut c_void, a2: u16, a3: u16) -> u16;
    fn LoadCompressedPalette(a0: *mut u32, a1: u16, a2: u16);
    fn LoadCompressedSpritePalette(a0: *mut CompressedSpritePalette);
    fn LoadHeldItemIcons();
    fn LoadMonIconPalettes();
    fn LoadOam();
    fn LoadPalette(a0: *mut c_void, a1: u16, a2: u16);
    fn LoadSpritePalette(a0: *mut SpritePalette) -> u8;
    fn LoadSpriteSheet(a0: *mut SpriteSheet) -> u16;
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
    fn PrintMenuTable(a0: u8, a1: u8, a2: *mut MenuAction);
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
    fn SendBlock(a0: u8, a1: *mut c_void, a2: u16) -> u8;
    fn SendBlockRequest(a0: u8) -> u8;
    fn SetBgTilemapBuffer(a0: u8, a1: *mut c_void);
    fn SetCloseLinkCallback();
    fn SetCloseLinkCallbackAndType(a0: u16);
    fn SetContinueGameWarpStatusToDynamicWarp();
    fn SetGpuReg(a0: u8, a1: u16);
    fn SetGpuRegBits(a0: u8, a1: u16);
    fn SetLinkStandbyCallback();
    fn SetMainCallback2(a0: Option<unsafe extern "C" fn()>);
    fn SetMonData(a0: *mut Pokemon, a1: i32, a2: *mut c_void);
    fn SetMultiuseSpriteTemplateToPokemon(a0: u16, a1: u8);
    fn SetPartyHPBarSprite(a0: *mut Sprite, a1: u8);
    fn SetVBlankCallback(a0: Option<unsafe extern "C" fn()>);
    fn SetWirelessCommType1();
    fn ShowBg(a0: u8);
    fn ShowPokemonSummaryScreen(
        a0: u8,
        a1: *mut c_void,
        a2: u8,
        a3: u8,
        a4: Option<unsafe extern "C" fn()>,
    );
    fn SpeciesToNationalPokedexNum(a0: u16) -> u16;
    fn SpriteCB_MonIcon(a0: *mut Sprite);
    fn SpriteCallbackDummy(a0: *mut Sprite);
    fn StartSpriteAffineAnim(a0: *mut Sprite, a1: u8);
    fn StartSpriteAnim(a0: *mut Sprite, a1: u8);
    fn StoreSpriteCallbackInData6(a0: *mut Sprite, a1: Option<unsafe extern "C" fn(*mut Sprite)>);
    fn StringAppend(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn StringCompareWithoutExtCtrlCodes(a0: *mut u8, a1: *mut u8) -> i32;
    fn StringCopy(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn StringCopy_Nickname(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn StringExpandPlaceholders(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn Task_WaitForLinkPlayerConnection(a0: u8);
    fn TradeEvolutionScene(a0: *mut Pokemon, a1: u16, a2: u8, a3: u8);
    fn Trade_MoveSelectedMonToTarget(a0: *mut Sprite);
    fn TransferPlttBuffer();
    fn UpdatePaletteFade() -> u8;
    fn rbox_fill_rectangle(a0: u8);
    fn rfu_NI_setSendData(a0: u8, a1: u8, a2: *mut c_void, a3: u32) -> u16;
    fn rfu_clearSlot(a0: u8, a1: u8) -> u16;
}

pub(crate) unsafe extern "C" fn SendLinkData(linkData: *mut c_void, size: u32) -> u8 {
    if gPlayerCurrActivity == ACTIVITY_29 {
        rfu_NI_setSendData(lman.acceptSlot_flag, 84, linkData, size);
        return TRUE;
    } else {
        return SendBlock(0, linkData, size as u16);
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn RequestLinkData(r#type: u8) {
    SendBlockRequest(r#type);
}
pub(crate) unsafe extern "C" fn IsLinkTradeTaskFinished() -> u32 {
    if gPlayerCurrActivity == ACTIVITY_29 {
        if (*gRfuSlotStatusNI[Rfu_GetIndexOfNewestChild(lman.acceptSlot_flag)])
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
        return 0;
    }
}
pub(crate) unsafe extern "C" fn _GetBlockReceivedStatus() -> u32 {
    return GetBlockReceivedStatus() as u32;
}
pub(crate) unsafe extern "C" fn TradeResetReceivedFlags() {
    if IsWirelessTrade() != 0 {
        rfu_clearSlot(12, lman.acceptSlot_flag);
    } else {
        ResetBlockReceivedFlags();
    }
}
pub(crate) unsafe extern "C" fn TradeResetReceivedFlag(who: u32) {
    if IsWirelessTrade() != 0 {
        rfu_clearSlot(12, lman.acceptSlot_flag);
    } else {
        ResetBlockReceivedFlag(who as u8);
    }
}
pub(crate) unsafe extern "C" fn IsWirelessTrade() -> u32 {
    if gWirelessCommType != 0 && gPlayerCurrActivity == ACTIVITY_29 {
        return TRUE as u32;
    } else {
        return FALSE as u32;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn SetTradeLinkStandbyCallback(unused: u8) {
    SetLinkStandbyCallback();
}
pub(crate) unsafe extern "C" fn _IsLinkTaskFinished() -> u32 {
    return IsLinkTaskFinished() as u32;
}
pub(crate) unsafe extern "C" fn InitTradeMenu() {
    ResetSpriteData();
    FreeAllSpritePalettes();
    ResetTasks();
    ResetPaletteFade();
    gPaletteFade.set_bufferTransferDisabled(TRUE as u16);
    SetVBlankCallback(Some(VBlankCB_TradeMenu));
    LoadPalette(
        gStandardMenuPalette.as_ptr().cast_mut() as *mut c_void,
        240,
        20,
    );
    LoadPalette(
        gStandardMenuPalette.as_ptr().cast_mut() as *mut c_void,
        208,
        20,
    );
    ResetBgsAndClearDma3BusyFlags(0);
    InitBgsFromTemplates(0, sBgTemplates.as_ptr().cast_mut(), 4);
    SetBgTilemapBuffer(1, (*sTradeMenu).tilemapBuffer.as_mut_ptr() as *mut c_void);
    if InitWindows(sWindowTemplates.as_ptr().cast_mut()) != 0 {
        let mut i: u32 = 0;
        DeactivateAllTextPrinters();
        i = 0;
        while i < 18 {
            ClearWindowTilemap(i as u8);
            FillWindowPixelBuffer(i as u8, 0);
            i += 1;
        }
        FillBgTilemapBufferRect(0, 0, 0, 0, DISPLAY_TILE_WIDTH, DISPLAY_TILE_HEIGHT, 15);
        LoadUserWindowBorderGfx_(0, 20, 192);
        LoadUserWindowBorderGfx(2, 1, 224);
        LoadMonIconPalettes();
        (*sTradeMenu).bufferPartyState = 0;
        (*sTradeMenu).callbackId = CB_MAIN_MENU;
        (*sTradeMenu).neverRead_70 = 0;
        (*sTradeMenu).drawSelectedMonState[0] = 0;
        (*sTradeMenu).drawSelectedMonState[1] = 0;
        (*sTradeMenu).playerConfirmStatus = STATUS_NONE;
        (*sTradeMenu).partnerConfirmStatus = STATUS_NONE;
        (*sTradeMenu).timer = 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CB2_StartCreateTradeMenu() {
    SetMainCallback2(Some(CB2_CreateTradeMenu));
    gMain.callback1 = None;
    gEnemyPartyCount = 0;
}
pub(crate) unsafe extern "C" fn CB2_CreateTradeMenu() {
    let mut i: i32 = 0;
    let mut temp: SpriteTemplate = zeroed();
    let mut id: u8 = 0;
    let mut xPos: u32 = 0;
    'l1: {
        let sw1: u8 = gMain.state;
        let mut fall = false;
        if sw1 == 0 {
            fall = true;
            sTradeMenu = AllocZeroed(4336) as *mut typeof___sTradeMenu_0_t;
            InitTradeMenu();
            sMenuTextTileBuffer = AllocZeroed(3584) as *mut u8;
            i = 0;
            while i < NUM_MENU_TEXT_SPRITES as i32 {
                sMenuTextTileBuffers[i] = sMenuTextTileBuffer.at(i * 256);
                i += 1;
            }
            gMain.state += 1;
            break 'l1;
        }
        if sw1 == 1 {
            fall = true;
            gPaletteFade.set_bufferTransferDisabled(FALSE as u16);
            i = 0;
            while i < PARTY_SIZE {
                CreateMon(&raw mut gEnemyParty[i], 0, 0, USE_RANDOM_IVS, 0, 0, 0, 0);
                i += 1;
            }
            PrintTradeMessage(MSG_STANDBY);
            ShowBg(0);
            if gReceivedRemoteLinkPlayers == 0 {
                gLinkType = LINKTYPE_TRADE_CONNECTING;
                (*sTradeMenu).timer = 0;
                if gWirelessCommType != 0 {
                    SetWirelessCommType1();
                    OpenLink();
                    CreateTask_RfuIdle();
                } else {
                    OpenLink();
                    gMain.state += 1;
                    CreateTask(Some(Task_WaitForLinkPlayerConnection), 1);
                }
            } else {
                gMain.state = 4;
            }
            break 'l1;
        }
        if sw1 == 2 {
            fall = true;
            (*sTradeMenu).timer += 1;
            if (*sTradeMenu).timer > 11 {
                (*sTradeMenu).timer = 0;
                gMain.state += 1;
            }
            break 'l1;
        }
        if sw1 == 3 {
            fall = true;
            if GetLinkPlayerCount_2() >= GetSavedPlayerCount() {
                if IsLinkMaster() != 0 {
                    if ({
                        (*sTradeMenu).timer += 1;
                        (*sTradeMenu).timer
                    }) > 30
                    {
                        CheckShouldAdvanceLinkState();
                        gMain.state += 1;
                    }
                } else {
                    gMain.state += 1;
                }
            }
            break 'l1;
        }
        if sw1 == 4 {
            fall = true;
            if gReceivedRemoteLinkPlayers == TRUE && IsLinkPlayerDataExchangeComplete() == TRUE {
                DestroyTask_RfuIdle();
                CalculatePlayerPartyCount();
                gMain.state += 1;
                (*sTradeMenu).timer = 0;
                if gWirelessCommType != 0 {
                    Rfu_SetLinkRecovery(TRUE as u32);
                    SetLinkStandbyCallback();
                }
            }
            break 'l1;
        }
        if sw1 == 5 {
            fall = true;
            if gWirelessCommType != 0 {
                if IsLinkRfuTaskFinished() != 0 {
                    gMain.state += 1;
                    LoadWirelessStatusIndicatorSpriteGfx();
                    CreateWirelessStatusIndicatorSprite(0, 0);
                }
            } else {
                gMain.state += 1;
            }
            break 'l1;
        }
        if sw1 == 6 {
            fall = true;
            if BufferTradeParties() != 0 {
                SaveTradeGiftRibbons();
                gMain.state += 1;
            }
            break 'l1;
        }
        if sw1 == 7 {
            fall = true;
            CalculateEnemyPartyCount();
            SetGpuReg(0x0, 0);
            SetGpuReg(REG_OFFSET_BLDCNT, 0);
            (*sTradeMenu).partyCounts[0] = gPlayerPartyCount;
            (*sTradeMenu).partyCounts[1] = gEnemyPartyCount;
            i = 0;
            while i < (*sTradeMenu).partyCounts[0] as i32 {
                let mut mon: *mut Pokemon = &raw mut gPlayerParty[i];
                (*sTradeMenu).partySpriteIds[0][i] = CreateMonIcon(
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
            while i < (*sTradeMenu).partyCounts[1] as i32 {
                let mut mon: *mut Pokemon = &raw mut gEnemyParty[i];
                (*sTradeMenu).partySpriteIds[1][i] = CreateMonIcon(
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
            gMain.state += 1;
            break 'l1;
        }
        if sw1 == 8 {
            fall = true;
            LoadHeldItemIcons();
            DrawHeldItemIconsForTrade(
                &raw mut (*sTradeMenu).partyCounts[0],
                (*sTradeMenu).partySpriteIds[0].as_mut_ptr(),
                0,
            );
            gMain.state += 1;
            break 'l1;
        }
        if sw1 == 9 {
            fall = true;
            DrawHeldItemIconsForTrade(
                &raw mut (*sTradeMenu).partyCounts[0],
                (*sTradeMenu).partySpriteIds[0].as_mut_ptr(),
                TRADE_PARTNER,
            );
            gMain.state += 1;
            break 'l1;
        }
        if sw1 == 10 {
            fall = true;
            DrawTextWindowAndBufferTiles(
                (*gSaveBlock2Ptr).playerName.as_mut_ptr(),
                sMenuTextTileBuffers[0] as *mut c_void,
                0,
                0,
                3,
            );
            id = GetMultiplayerId();
            DrawTextWindowAndBufferTiles(
                gLinkPlayers[id as i32 ^ 1].name.as_mut_ptr(),
                sMenuTextTileBuffers[3] as *mut c_void,
                0,
                0,
                3,
            );
            DrawTextWindowAndBufferTiles(
                sActionTexts[0],
                sMenuTextTileBuffers[6] as *mut c_void,
                0,
                0,
                2,
            );
            DrawBottomRowText(sActionTexts[1], sMenuTextTileBuffers[8], 24);
            gMain.state += 1;
            (*sTradeMenu).timer = 0;
            break 'l1;
        }
        if sw1 == 11 {
            fall = true;
            if LoadUISpriteGfx() != 0 {
                gMain.state += 1;
            }
            break 'l1;
        }
        if sw1 == 12 {
            fall = true;
            xPos = GetStringCenterAlignXOffset(
                FONT_NORMAL as i32,
                (*gSaveBlock2Ptr).playerName.as_mut_ptr(),
                120,
            ) as u32;
            i = 0;
            while i < 3 {
                temp = *sSpriteTemplate_MenuText;
                temp.tileTag += i as u16 + GFXTAG_PLAYER_NAME_L;
                CreateSprite(&raw mut temp, xPos as i16 + i as i16 * 32 + 16, 10, 1);
                i += 1;
            }
            xPos = GetStringCenterAlignXOffset(
                FONT_NORMAL as i32,
                gLinkPlayers[GetMultiplayerId() as i32 ^ 1]
                    .name
                    .as_mut_ptr(),
                120,
            ) as u32;
            i = 0;
            while i < 3 {
                temp = *sSpriteTemplate_MenuText;
                temp.tileTag += i as u16 + GFXTAG_PARTNER_NAME_L;
                CreateSprite(&raw mut temp, xPos as i16 + i as i16 * 32 + 136, 10, 1);
                i += 1;
            }
            gMain.state += 1;
            break 'l1;
        }
        if sw1 == 13 {
            fall = true;
            temp = *sSpriteTemplate_MenuText;
            temp.tileTag += GFXTAG_CANCEL_L;
            CreateSprite(&raw mut temp, 215, 152, 1);
            temp = *sSpriteTemplate_MenuText;
            temp.tileTag += GFXTAG_CANCEL_R;
            CreateSprite(&raw mut temp, 247, 152, 1);
            i = 0;
            while i < 6 {
                temp = *sSpriteTemplate_MenuText;
                temp.tileTag += i as u16 + GFXTAG_CHOOSE_PKMN_L;
                CreateSprite(&raw mut temp, i as i16 * 32 + 24, 150, 1);
                i += 1;
            }
            (*sTradeMenu).cursorSpriteId = CreateSprite(
                (&raw const *sSpriteTemplate_Cursor).cast_mut(),
                sTradeMonSpriteCoords[0][0] as i16 * 8 + 32,
                sTradeMonSpriteCoords[0][1] as i16 * 8,
                2,
            );
            (*sTradeMenu).cursorPosition = 0;
            gMain.state += 1;
            rbox_fill_rectangle(0);
            break 'l1;
        }
        if sw1 == 14 {
            fall = true;
            ComputePartyTradeableFlags(TRADE_PLAYER);
            PrintPartyNicknames(TRADE_PLAYER);
            (*sTradeMenu).bg2hofs = 0;
            (*sTradeMenu).bg3hofs = 0;
            SetActiveMenuOptions();
            gMain.state += 1;
            PlayBGM(MUS_SCHOOL);
            break 'l1;
        }
        if sw1 == 15 {
            fall = true;
            ComputePartyTradeableFlags(TRADE_PARTNER);
            PrintPartyNicknames(TRADE_PARTNER);
            gMain.state += 1;
        }
        if fall || sw1 == 16 {
            fall = true;
            LoadTradeBgGfx(0);
            gMain.state += 1;
            break 'l1;
        }
        if sw1 == 17 {
            fall = true;
            LoadTradeBgGfx(1);
            gMain.state += 1;
            break 'l1;
        }
        if sw1 == 18 {
            fall = true;
            BeginNormalPaletteFade(PALETTES_ALL, 0, 16, 0, 0);
            gMain.state += 1;
            break 'l1;
        }
        if sw1 == 19 {
            fall = true;
            SetGpuReg(REG_OFFSET_DISPCNT, 4160);
            LoadTradeBgGfx(2);
            gMain.state += 1;
            break 'l1;
        }
        if sw1 == 20 {
            fall = true;
            ComputePartyHPBarLevels(TRADE_PLAYER);
            gMain.state += 1;
            break 'l1;
        }
        if sw1 == 21 {
            fall = true;
            ComputePartyHPBarLevels(TRADE_PARTNER);
            SetTradePartyHPBarSprites();
            gMain.state += 1;
            break 'l1;
        }
        if sw1 == 22 {
            fall = true;
            if gPaletteFade.active() == 0 {
                gMain.callback1 = Some(CB1_UpdateLink);
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
pub(crate) unsafe extern "C" fn CB2_ReturnToTradeMenu() {
    let mut i: i32 = 0;
    let mut temp: SpriteTemplate = zeroed();
    let mut id: u8 = 0;
    let mut xPos: u32 = 0;
    match gMain.state {
        0 => {
            InitTradeMenu();
            gMain.state += 1;
        }
        1 => {
            gMain.state += 1;
            (*sTradeMenu).timer = 0;
        }
        2 => {
            gMain.state += 1;
        }
        3 => {
            gMain.state += 1;
        }
        4 => {
            CalculatePlayerPartyCount();
            gMain.state += 1;
        }
        5 => {
            if gWirelessCommType != 0 {
                LoadWirelessStatusIndicatorSpriteGfx();
                CreateWirelessStatusIndicatorSprite(0, 0);
            }
            gMain.state += 1;
        }
        6 => {
            gMain.state += 1;
        }
        7 => {
            CalculateEnemyPartyCount();
            (*sTradeMenu).partyCounts[0] = gPlayerPartyCount;
            (*sTradeMenu).partyCounts[1] = gEnemyPartyCount;
            ClearWindowTilemap(0);
            PrintPartyNicknames(TRADE_PLAYER);
            PrintPartyNicknames(TRADE_PARTNER);
            i = 0;
            while i < (*sTradeMenu).partyCounts[0] as i32 {
                let mut mon: *mut Pokemon = &raw mut gPlayerParty[i];
                (*sTradeMenu).partySpriteIds[0][i] = CreateMonIcon(
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
            while i < (*sTradeMenu).partyCounts[1] as i32 {
                let mut mon: *mut Pokemon = &raw mut gEnemyParty[i];
                (*sTradeMenu).partySpriteIds[1][i] = CreateMonIcon(
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
            gMain.state += 1;
        }
        8 => {
            LoadHeldItemIcons();
            DrawHeldItemIconsForTrade(
                &raw mut (*sTradeMenu).partyCounts[0],
                (*sTradeMenu).partySpriteIds[0].as_mut_ptr(),
                0,
            );
            gMain.state += 1;
        }
        9 => {
            DrawHeldItemIconsForTrade(
                &raw mut (*sTradeMenu).partyCounts[0],
                (*sTradeMenu).partySpriteIds[0].as_mut_ptr(),
                TRADE_PARTNER,
            );
            gMain.state += 1;
        }
        10 => {
            DrawTextWindowAndBufferTiles(
                (*gSaveBlock2Ptr).playerName.as_mut_ptr(),
                sMenuTextTileBuffers[0] as *mut c_void,
                0,
                0,
                3,
            );
            id = GetMultiplayerId();
            DrawTextWindowAndBufferTiles(
                gLinkPlayers[id as i32 ^ 1].name.as_mut_ptr(),
                sMenuTextTileBuffers[3] as *mut c_void,
                0,
                0,
                3,
            );
            DrawTextWindowAndBufferTiles(
                sActionTexts[0],
                sMenuTextTileBuffers[6] as *mut c_void,
                0,
                0,
                2,
            );
            DrawBottomRowText(sActionTexts[1], sMenuTextTileBuffers[8], 24);
            gMain.state += 1;
            (*sTradeMenu).timer = 0;
        }
        11 => {
            if LoadUISpriteGfx() != 0 {
                gMain.state += 1;
            }
        }
        12 => {
            xPos = GetStringCenterAlignXOffset(
                FONT_NORMAL as i32,
                (*gSaveBlock2Ptr).playerName.as_mut_ptr(),
                120,
            ) as u32;
            i = 0;
            while i < 3 {
                temp = *sSpriteTemplate_MenuText;
                temp.tileTag += i as u16 + GFXTAG_PLAYER_NAME_L;
                CreateSprite(&raw mut temp, xPos as i16 + i as i16 * 32 + 16, 10, 1);
                i += 1;
            }
            xPos = GetStringCenterAlignXOffset(
                FONT_NORMAL as i32,
                gLinkPlayers[GetMultiplayerId() as i32 ^ 1]
                    .name
                    .as_mut_ptr(),
                120,
            ) as u32;
            i = 0;
            while i < 3 {
                temp = *sSpriteTemplate_MenuText;
                temp.tileTag += i as u16 + GFXTAG_PARTNER_NAME_L;
                CreateSprite(&raw mut temp, xPos as i16 + i as i16 * 32 + 136, 10, 1);
                i += 1;
            }
            gMain.state += 1;
        }
        13 => {
            temp = *sSpriteTemplate_MenuText;
            temp.tileTag += GFXTAG_CANCEL_L;
            CreateSprite(&raw mut temp, 215, 152, 1);
            temp = *sSpriteTemplate_MenuText;
            temp.tileTag += GFXTAG_CANCEL_R;
            CreateSprite(&raw mut temp, 247, 152, 1);
            i = 0;
            while i < 6 {
                temp = *sSpriteTemplate_MenuText;
                temp.tileTag += i as u16 + GFXTAG_CHOOSE_PKMN_L;
                CreateSprite(&raw mut temp, i as i16 * 32 + 24, 150, 1);
                i += 1;
            }
            if (*sTradeMenu).cursorPosition < PARTY_SIZE as u8 {
                (*sTradeMenu).cursorPosition = gLastViewedMonIndex;
            } else {
                (*sTradeMenu).cursorPosition = gLastViewedMonIndex + PARTY_SIZE as u8;
            }
            (*sTradeMenu).cursorSpriteId = CreateSprite(
                (&raw const *sSpriteTemplate_Cursor).cast_mut(),
                sTradeMonSpriteCoords[(*sTradeMenu).cursorPosition][0] as i16 * 8 + 32,
                sTradeMonSpriteCoords[(*sTradeMenu).cursorPosition][1] as i16 * 8,
                2,
            );
            gMain.state = 16;
        }
        16 => {
            LoadTradeBgGfx(0);
            gMain.state += 1;
        }
        17 => {
            LoadTradeBgGfx(1);
            (*sTradeMenu).bg2hofs = 0;
            (*sTradeMenu).bg3hofs = 0;
            SetActiveMenuOptions();
            gMain.state += 1;
        }
        18 => {
            gPaletteFade.set_bufferTransferDisabled(FALSE as u16);
            BlendPalettes(PALETTES_ALL, 16, 0);
            BeginNormalPaletteFade(PALETTES_ALL, 0, 16, 0, 0);
            gMain.state += 1;
        }
        19 => {
            SetGpuReg(REG_OFFSET_DISPCNT, 4160);
            LoadTradeBgGfx(2);
            gMain.state += 1;
        }
        20 => {
            gMain.state += 1;
        }
        21 => {
            SetTradePartyHPBarSprites();
            gMain.state += 1;
        }
        22 => {
            if gPaletteFade.active() == 0 {
                SetMainCallback2(Some(CB2_TradeMenu));
            }
        }
        _ => {}
    }
    RunTasks();
    AnimateSprites();
    BuildOamBuffer();
    UpdatePaletteFade();
}
pub(crate) unsafe extern "C" fn VBlankCB_TradeMenu() {
    LoadOam();
    ProcessSpriteCopyRequests();
    TransferPlttBuffer();
}
pub(crate) unsafe extern "C" fn CB_FadeToStartTrade() {
    if ({
        (*sTradeMenu).timer += 1;
        (*sTradeMenu).timer
    }) > 15
    {
        BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 16, 0);
        (*sTradeMenu).callbackId = CB_WAIT_TO_START_TRADE;
    }
}
pub(crate) unsafe extern "C" fn CB_WaitToStartTrade() {
    if gPaletteFade.active() == 0 {
        gSelectedTradeMonPositions[0] = (*sTradeMenu).cursorPosition;
        gSelectedTradeMonPositions[1] = (*sTradeMenu).partnerCursorPosition;
        if gWirelessCommType != 0 {
            (*sTradeMenu).callbackId = CB_WAIT_TO_START_RFU_TRADE;
        } else {
            SetCloseLinkCallbackAndType(32);
            (*sTradeMenu).callbackId = CB_START_LINK_TRADE;
        }
    }
}
pub(crate) unsafe extern "C" fn CB_StartLinkTrade() {
    gMain.savedCallback = Some(CB2_StartCreateTradeMenu);
    if gWirelessCommType != 0 {
        if IsLinkRfuTaskFinished() != 0 {
            Free(sMenuTextTileBuffer as *mut c_void);
            FreeAllWindowBuffers();
            Free(sTradeMenu as *mut c_void);
            gMain.callback1 = None;
            DestroyWirelessStatusIndicatorSprite();
            SetMainCallback2(Some(CB2_LinkTrade));
        }
    } else {
        if gReceivedRemoteLinkPlayers == 0 {
            Free(sMenuTextTileBuffer as *mut c_void);
            FreeAllWindowBuffers();
            Free(sTradeMenu as *mut c_void);
            gMain.callback1 = None;
            SetMainCallback2(Some(CB2_LinkTrade));
        }
    }
}
pub(crate) unsafe extern "C" fn CB2_TradeMenu() {
    RunTradeMenuCallback();
    DoQueuedActions();
    DrawSelectedMonScreen(TRADE_PLAYER);
    DrawSelectedMonScreen(TRADE_PARTNER);
    SetGpuReg(
        REG_OFFSET_BG2HOFS,
        ({
            let t1 = (*sTradeMenu).bg2hofs;
            (*sTradeMenu).bg2hofs += 1;
            t1
        }) as u16,
    );
    SetGpuReg(
        REG_OFFSET_BG3HOFS,
        ({
            let t2 = (*sTradeMenu).bg3hofs;
            (*sTradeMenu).bg3hofs -= 1;
            t2
        }) as u16,
    );
    RunTextPrintersAndIsPrinter0Active();
    RunTasks();
    AnimateSprites();
    BuildOamBuffer();
    UpdatePaletteFade();
}
pub(crate) unsafe extern "C" fn LoadTradeBgGfx(state: u8) {
    let mut i: i32 = 0;
    match state {
        0 => {
            LoadPalette(gTradeMenu_Pal.as_ptr().cast_mut() as *mut c_void, 0, 96);
            LoadBgTiles(
                1,
                gTradeMenu_Gfx.as_ptr().cast_mut() as *mut c_void,
                0x1280,
                0,
            );
            CopyToBgTilemapBufferRect_ChangePalette(
                1,
                gTradeMenu_Tilemap.as_ptr().cast_mut() as *mut c_void,
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
            i = 0;
            while i < 4 {
                SetGpuReg(REG_OFFSET_BG0HOFS + i as u8 * 2, 0);
                i += 1;
            }
            ShowBg(0);
            ShowBg(1);
            ShowBg(2);
            ShowBg(3);
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn SetActiveMenuOptions() {
    let mut i: i32 = 0;
    i = 0;
    while i < PARTY_SIZE {
        if i < (*sTradeMenu).partyCounts[0] as i32 {
            gSprites[(*sTradeMenu).partySpriteIds[0][i]].set_invisible(0);
            (*sTradeMenu).optionsActive[i] = TRUE;
        } else {
            (*sTradeMenu).optionsActive[i] = FALSE;
        }
        if i < (*sTradeMenu).partyCounts[1] as i32 {
            gSprites[(*sTradeMenu).partySpriteIds[1][i]].set_invisible(FALSE as u16);
            (*sTradeMenu).optionsActive[i + PARTY_SIZE] = TRUE;
        } else {
            (*sTradeMenu).optionsActive[i + PARTY_SIZE] = FALSE;
        }
        i += 1;
    }
    (*sTradeMenu).optionsActive[12] = TRUE;
}
pub(crate) unsafe extern "C" fn Trade_Memcpy(dest: *mut c_void, src: *mut c_void, size: u32) {
    let mut _dest: *mut u8 = dest as *mut u8;
    let mut _src: *mut u8 = src as *mut u8;
    let mut i: u32 = 0;
    i = 0;
    while i < size {
        *_dest.at(i) = *_src.at(i);
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn BufferTradeParties() -> u8 {
    let mut id: u8 = GetMultiplayerId();
    let mut i: i32 = 0;
    let mut mon: *mut Pokemon = null_mut();
    match (*sTradeMenu).bufferPartyState {
        0 => {
            Trade_Memcpy(
                gBlockSendBuffer.as_mut_ptr() as *mut c_void,
                &raw mut gPlayerParty[0] as *mut c_void,
                200,
            );
            (*sTradeMenu).bufferPartyState += 1;
            (*sTradeMenu).timer = 0;
        }
        1 => {
            if IsLinkTradeTaskFinished() != 0 {
                if _GetBlockReceivedStatus() == 0 {
                    (*sTradeMenu).bufferPartyState += 1;
                } else {
                    TradeResetReceivedFlags();
                    (*sTradeMenu).bufferPartyState += 1;
                }
            }
        }
        3 => {
            if id == 0 {
                RequestLinkData(BLOCK_REQ_SIZE_200);
            }
            (*sTradeMenu).bufferPartyState += 1;
        }
        4 => {
            if _GetBlockReceivedStatus() == 3 {
                Trade_Memcpy(
                    &raw mut gEnemyParty[0] as *mut c_void,
                    gBlockRecvBuffer[id as i32 ^ 1].as_mut_ptr() as *mut c_void,
                    200,
                );
                TradeResetReceivedFlags();
                (*sTradeMenu).bufferPartyState += 1;
            }
        }
        5 => {
            Trade_Memcpy(
                gBlockSendBuffer.as_mut_ptr() as *mut c_void,
                &raw mut gPlayerParty[2] as *mut c_void,
                200,
            );
            (*sTradeMenu).bufferPartyState += 1;
        }
        7 => {
            if id == 0 {
                RequestLinkData(BLOCK_REQ_SIZE_200);
            }
            (*sTradeMenu).bufferPartyState += 1;
        }
        8 => {
            if _GetBlockReceivedStatus() == 3 {
                Trade_Memcpy(
                    &raw mut gEnemyParty[2] as *mut c_void,
                    gBlockRecvBuffer[id as i32 ^ 1].as_mut_ptr() as *mut c_void,
                    200,
                );
                TradeResetReceivedFlags();
                (*sTradeMenu).bufferPartyState += 1;
            }
        }
        9 => {
            Trade_Memcpy(
                gBlockSendBuffer.as_mut_ptr() as *mut c_void,
                &raw mut gPlayerParty[4] as *mut c_void,
                200,
            );
            (*sTradeMenu).bufferPartyState += 1;
        }
        11 => {
            if id == 0 {
                RequestLinkData(BLOCK_REQ_SIZE_200);
            }
            (*sTradeMenu).bufferPartyState += 1;
        }
        12 => {
            if _GetBlockReceivedStatus() == 3 {
                Trade_Memcpy(
                    &raw mut gEnemyParty[4] as *mut c_void,
                    gBlockRecvBuffer[id as i32 ^ 1].as_mut_ptr() as *mut c_void,
                    200,
                );
                TradeResetReceivedFlags();
                (*sTradeMenu).bufferPartyState += 1;
            }
        }
        13 => {
            Trade_Memcpy(
                gBlockSendBuffer.as_mut_ptr() as *mut c_void,
                (*gSaveBlock1Ptr).mail.as_mut_ptr() as *mut c_void,
                220,
            );
            (*sTradeMenu).bufferPartyState += 1;
        }
        15 => {
            if id == 0 {
                RequestLinkData(BLOCK_REQ_SIZE_220);
            }
            (*sTradeMenu).bufferPartyState += 1;
        }
        16 => {
            if _GetBlockReceivedStatus() == 3 {
                Trade_Memcpy(
                    gTradeMail.as_mut_ptr() as *mut c_void,
                    gBlockRecvBuffer[id as i32 ^ 1].as_mut_ptr() as *mut c_void,
                    216,
                );
                TradeResetReceivedFlags();
                (*sTradeMenu).bufferPartyState += 1;
            }
        }
        17 => {
            Trade_Memcpy(
                gBlockSendBuffer.as_mut_ptr() as *mut c_void,
                (*gSaveBlock1Ptr).giftRibbons.as_mut_ptr() as *mut c_void,
                11,
            );
            (*sTradeMenu).bufferPartyState += 1;
        }
        19 => {
            if id == 0 {
                RequestLinkData(BLOCK_REQ_SIZE_40);
            }
            (*sTradeMenu).bufferPartyState += 1;
        }
        20 => {
            if _GetBlockReceivedStatus() == 3 {
                Trade_Memcpy(
                    (*sTradeMenu).giftRibbons.as_mut_ptr() as *mut c_void,
                    gBlockRecvBuffer[id as i32 ^ 1].as_mut_ptr() as *mut c_void,
                    11,
                );
                TradeResetReceivedFlags();
                (*sTradeMenu).bufferPartyState += 1;
            }
        }
        21 => {
            i = 0;
            mon = gEnemyParty.as_mut_ptr();
            while i < PARTY_SIZE {
                let mut name: CArray<u8, 11> = zeroed();
                let mut species: u16 = GetMonData2(mon, MON_DATA_SPECIES) as u16;
                if species != SPECIES_NONE {
                    if species == SPECIES_SHEDINJA
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
                                gSpeciesNames[303].as_ptr().cast_mut() as *mut c_void,
                            );
                        }
                    }
                }
                mon = mon.at(1);
                i += 1;
            }
            return TRUE;
        }
        2 | 6 | 10 | 14 | 18 => {
            (*sTradeMenu).timer += 1;
            if (*sTradeMenu).timer > 10 {
                (*sTradeMenu).timer = 0;
                (*sTradeMenu).bufferPartyState += 1;
            }
        }
        _ => {}
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn PrintIsThisTradeOkay() {
    DrawBottomRowText(
        sText_IsThisTradeOkay.as_ptr().cast_mut(),
        (OBJ_VRAM0 + (*sTradeMenu).bottomTextTileStart as i32 * 32) as usize as *mut c_void
            as *mut u8,
        24,
    );
}
pub(crate) unsafe extern "C" fn Leader_ReadLinkBuffer(mpId: u8, status: u8) {
    if status as i32 & 1 != 0 {
        match gBlockRecvBuffer[0][0] {
            LINKCMD_REQUEST_CANCEL => {
                (*sTradeMenu).playerSelectStatus = STATUS_CANCEL;
            }
            LINKCMD_READY_TO_TRADE => {
                (*sTradeMenu).playerSelectStatus = STATUS_READY;
            }
            LINKCMD_INIT_BLOCK => {
                (*sTradeMenu).playerConfirmStatus = STATUS_READY;
            }
            LINKCMD_READY_CANCEL_TRADE => {
                (*sTradeMenu).playerConfirmStatus = STATUS_CANCEL;
            }
            _ => {}
        }
        TradeResetReceivedFlag(0);
    }
    if status as i32 & 2 != 0 {
        match gBlockRecvBuffer[1][0] {
            LINKCMD_REQUEST_CANCEL => {
                (*sTradeMenu).partnerSelectStatus = STATUS_CANCEL;
            }
            LINKCMD_READY_TO_TRADE => {
                (*sTradeMenu).partnerCursorPosition =
                    gBlockRecvBuffer[1][1] as u8 + PARTY_SIZE as u8;
                (*sTradeMenu).partnerSelectStatus = STATUS_READY;
            }
            LINKCMD_INIT_BLOCK => {
                (*sTradeMenu).partnerConfirmStatus = STATUS_READY;
            }
            LINKCMD_READY_CANCEL_TRADE => {
                (*sTradeMenu).partnerConfirmStatus = STATUS_CANCEL;
            }
            _ => {}
        }
        TradeResetReceivedFlag(1);
    }
}
pub(crate) unsafe extern "C" fn Follower_ReadLinkBuffer(mpId: u8, status: u8) {
    if status as i32 & 1 != 0 {
        match gBlockRecvBuffer[0][0] {
            LINKCMD_BOTH_CANCEL_TRADE => {
                BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 16, 0);
                PrintTradeMessage(MSG_WAITING_FOR_FRIEND);
                (*sTradeMenu).callbackId = CB_INIT_EXIT_CANCELED_TRADE;
            }
            LINKCMD_PARTNER_CANCEL_TRADE => {
                PrintTradeMessage(MSG_FRIEND_WANTS_TO_TRADE);
                (*sTradeMenu).callbackId = CB_HANDLE_TRADE_CANCELED;
            }
            LINKCMD_SET_MONS_TO_TRADE => {
                (*sTradeMenu).partnerCursorPosition =
                    gBlockRecvBuffer[0][1] as u8 + PARTY_SIZE as u8;
                rbox_fill_rectangle(0);
                SetSelectedMon((*sTradeMenu).cursorPosition);
                SetSelectedMon((*sTradeMenu).partnerCursorPosition);
                (*sTradeMenu).callbackId = CB_PRINT_IS_THIS_OKAY;
            }
            LINKCMD_START_TRADE => {
                BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 16, 0);
                (*sTradeMenu).callbackId = CB_WAIT_TO_START_TRADE;
            }
            LINKCMD_PLAYER_CANCEL_TRADE => {
                PrintTradeMessage(MSG_CANCELED);
                (*sTradeMenu).callbackId = CB_HANDLE_TRADE_CANCELED;
            }
            _ => {}
        }
        TradeResetReceivedFlag(0);
    }
    if status as i32 & 2 != 0 {
        TradeResetReceivedFlag(1);
    }
}
pub(crate) unsafe extern "C" fn Leader_HandleCommunication() {
    if (*sTradeMenu).playerSelectStatus != STATUS_NONE
        && (*sTradeMenu).partnerSelectStatus != STATUS_NONE
    {
        if (*sTradeMenu).playerSelectStatus == STATUS_READY
            && (*sTradeMenu).partnerSelectStatus == STATUS_READY
        {
            (*sTradeMenu).callbackId = CB_SET_SELECTED_MONS;
            (*sTradeMenu).linkData[0] = LINKCMD_SET_MONS_TO_TRADE;
            (*sTradeMenu).linkData[1] = (*sTradeMenu).cursorPosition as u16;
            QueueAction(QUEUE_DELAY_DATA, QUEUE_SEND_DATA);
            (*sTradeMenu).playerSelectStatus = {
                (*sTradeMenu).partnerSelectStatus = STATUS_NONE;
                (*sTradeMenu).partnerSelectStatus
            };
        } else if (*sTradeMenu).playerSelectStatus == STATUS_READY
            && (*sTradeMenu).partnerSelectStatus == STATUS_CANCEL
        {
            PrintTradeMessage(MSG_CANCELED);
            (*sTradeMenu).linkData[0] = LINKCMD_PARTNER_CANCEL_TRADE;
            (*sTradeMenu).linkData[1] = 0;
            QueueAction(QUEUE_DELAY_DATA, QUEUE_SEND_DATA);
            (*sTradeMenu).playerConfirmStatus = {
                (*sTradeMenu).partnerConfirmStatus = STATUS_NONE;
                (*sTradeMenu).partnerConfirmStatus
            };
            (*sTradeMenu).playerSelectStatus = {
                (*sTradeMenu).partnerSelectStatus = STATUS_NONE;
                (*sTradeMenu).partnerSelectStatus
            };
            (*sTradeMenu).callbackId = CB_HANDLE_TRADE_CANCELED;
        } else if (*sTradeMenu).playerSelectStatus == STATUS_CANCEL
            && (*sTradeMenu).partnerSelectStatus == STATUS_READY
        {
            PrintTradeMessage(MSG_FRIEND_WANTS_TO_TRADE);
            (*sTradeMenu).linkData[0] = LINKCMD_PLAYER_CANCEL_TRADE;
            (*sTradeMenu).linkData[1] = 0;
            QueueAction(QUEUE_DELAY_DATA, QUEUE_SEND_DATA);
            (*sTradeMenu).playerConfirmStatus = {
                (*sTradeMenu).partnerConfirmStatus = STATUS_NONE;
                (*sTradeMenu).partnerConfirmStatus
            };
            (*sTradeMenu).playerSelectStatus = {
                (*sTradeMenu).partnerSelectStatus = STATUS_NONE;
                (*sTradeMenu).partnerSelectStatus
            };
            (*sTradeMenu).callbackId = CB_HANDLE_TRADE_CANCELED;
        } else if (*sTradeMenu).playerSelectStatus == STATUS_CANCEL
            && (*sTradeMenu).partnerSelectStatus == STATUS_CANCEL
        {
            (*sTradeMenu).linkData[0] = LINKCMD_BOTH_CANCEL_TRADE;
            (*sTradeMenu).linkData[1] = 0;
            QueueAction(QUEUE_DELAY_DATA, QUEUE_SEND_DATA);
            BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 16, 0);
            (*sTradeMenu).playerSelectStatus = {
                (*sTradeMenu).partnerSelectStatus = STATUS_NONE;
                (*sTradeMenu).partnerSelectStatus
            };
            (*sTradeMenu).callbackId = CB_INIT_EXIT_CANCELED_TRADE;
        }
    }
    if (*sTradeMenu).playerConfirmStatus != STATUS_NONE
        && (*sTradeMenu).partnerConfirmStatus != STATUS_NONE
    {
        if (*sTradeMenu).playerConfirmStatus == STATUS_READY
            && (*sTradeMenu).partnerConfirmStatus == STATUS_READY
        {
            (*sTradeMenu).linkData[0] = LINKCMD_START_TRADE;
            (*sTradeMenu).linkData[1] = 0;
            QueueAction(QUEUE_DELAY_DATA, QUEUE_SEND_DATA);
            (*sTradeMenu).playerConfirmStatus = STATUS_NONE;
            (*sTradeMenu).partnerConfirmStatus = STATUS_NONE;
            (*sTradeMenu).callbackId = CB_FADE_TO_START_TRADE;
        }
        if (*sTradeMenu).playerConfirmStatus == STATUS_CANCEL
            || (*sTradeMenu).partnerConfirmStatus == STATUS_CANCEL
        {
            PrintTradeMessage(MSG_CANCELED);
            (*sTradeMenu).linkData[0] = LINKCMD_PLAYER_CANCEL_TRADE;
            (*sTradeMenu).linkData[1] = 0;
            QueueAction(QUEUE_DELAY_DATA, QUEUE_SEND_DATA);
            (*sTradeMenu).playerConfirmStatus = STATUS_NONE;
            (*sTradeMenu).partnerConfirmStatus = STATUS_NONE;
            (*sTradeMenu).callbackId = CB_HANDLE_TRADE_CANCELED;
        }
    }
}
pub(crate) unsafe extern "C" fn _SetLinkData(
    mut linkData: *mut u16,
    linkCmd: u16,
    cursorPosition: u16,
) {
    *linkData = linkCmd;
    *linkData.at(1) = cursorPosition;
    QueueAction(QUEUE_DELAY_DATA, QUEUE_SEND_DATA);
}
pub(crate) unsafe extern "C" fn SetLinkData(linkCmd: u16, cursorPosition: u16) {
    _SetLinkData((*sTradeMenu).linkData.as_mut_ptr(), linkCmd, cursorPosition);
}
pub(crate) unsafe extern "C" fn CB1_UpdateLink() {
    let mut mpId: u8 = GetMultiplayerId();
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
pub(crate) unsafe extern "C" fn GetNewCursorPosition(oldPosition: u8, direction: u8) -> u8 {
    let mut i: i32 = 0;
    let mut newPosition: u8 = 0;
    i = 0;
    while i < PARTY_SIZE {
        if (*sTradeMenu).optionsActive[sCursorMoveDestinations[oldPosition][direction][i]] == TRUE {
            newPosition = sCursorMoveDestinations[oldPosition][direction][i];
            break;
        }
        i += 1;
    }
    return newPosition;
}
pub(crate) unsafe extern "C" fn TradeMenuMoveCursor(cursorPosition: *mut u8, direction: u8) {
    let mut newPosition: u8 = GetNewCursorPosition(*cursorPosition, direction);
    if newPosition == 12 {
        StartSpriteAnim(
            &raw mut gSprites[(*sTradeMenu).cursorSpriteId],
            CURSOR_ANIM_ON_CANCEL,
        );
        gSprites[(*sTradeMenu).cursorSpriteId].x = 224;
        gSprites[(*sTradeMenu).cursorSpriteId].y = DISPLAY_HEIGHT as i16;
    } else {
        StartSpriteAnim(
            &raw mut gSprites[(*sTradeMenu).cursorSpriteId],
            CURSOR_ANIM_NORMAL,
        );
        gSprites[(*sTradeMenu).cursorSpriteId].x =
            sTradeMonSpriteCoords[newPosition][0] as i16 * 8 + 32;
        gSprites[(*sTradeMenu).cursorSpriteId].y = sTradeMonSpriteCoords[newPosition][1] as i16 * 8;
    }
    if *cursorPosition != newPosition {
        PlaySE(SE_SELECT);
    }
    *cursorPosition = newPosition;
}
pub(crate) unsafe extern "C" fn SetReadyToTrade() {
    PrintTradeMessage(MSG_STANDBY);
    (*sTradeMenu).callbackId = CB_IDLE;
    if GetMultiplayerId() == 1 {
        SetLinkData(LINKCMD_READY_TO_TRADE, (*sTradeMenu).cursorPosition as u16);
    } else {
        (*sTradeMenu).playerSelectStatus = STATUS_READY;
    }
}
pub(crate) unsafe extern "C" fn CB_ProcessMenuInput() {
    if gMain.newAndRepeatedKeys as i32 & DPAD_UP != 0 {
        TradeMenuMoveCursor(&raw mut (*sTradeMenu).cursorPosition, 0);
    } else if gMain.newAndRepeatedKeys as i32 & DPAD_DOWN != 0 {
        TradeMenuMoveCursor(&raw mut (*sTradeMenu).cursorPosition, 1);
    } else if gMain.newAndRepeatedKeys as i32 & DPAD_LEFT != 0 {
        TradeMenuMoveCursor(&raw mut (*sTradeMenu).cursorPosition, 2);
    } else if gMain.newAndRepeatedKeys as i32 & DPAD_RIGHT != 0 {
        TradeMenuMoveCursor(&raw mut (*sTradeMenu).cursorPosition, 3);
    }
    if gMain.newKeys as i32 & A_BUTTON != 0 {
        PlaySE(SE_SELECT);
        if (*sTradeMenu).cursorPosition < PARTY_SIZE as u8 {
            DrawTextBorderOuter(1, 1, 14);
            FillWindowPixelBuffer(1, 17);
            PrintMenuTable(1, 2, sSelectTradeMonActions.as_ptr().cast_mut());
            InitMenuInUpperLeftCornerNormal(1, 2, 0);
            PutWindowTilemap(1);
            CopyWindowToVram(1, COPYWIN_FULL);
            (*sTradeMenu).callbackId = CB_SELECTED_MON;
        } else if (*sTradeMenu).cursorPosition < 12 {
            BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 16, 0);
            (*sTradeMenu).callbackId = CB_SHOW_MON_SUMMARY;
        } else if (*sTradeMenu).cursorPosition == 12 {
            CreateYesNoMenu((&raw const *sTradeYesNoWindowTemplate).cast_mut(), 1, 14, 0);
            (*sTradeMenu).callbackId = CB_CANCEL_TRADE_PROMPT;
            DrawBottomRowText(
                sActionTexts[4],
                (OBJ_VRAM0 + (*sTradeMenu).bottomTextTileStart as i32 * 32) as usize as *mut c_void
                    as *mut u8,
                24,
            );
        }
    }
}
pub(crate) unsafe extern "C" fn RedrawChooseAPokemonWindow() {
    PrintTradePartnerPartyNicknames();
    (*sTradeMenu).callbackId = CB_MAIN_MENU;
    gSprites[(*sTradeMenu).cursorSpriteId].set_invisible(FALSE as u16);
    DrawBottomRowText(
        sActionTexts[1],
        (OBJ_VRAM0 + (*sTradeMenu).bottomTextTileStart as i32 * 32) as usize as *mut c_void
            as *mut u8,
        24,
    );
}
pub(crate) unsafe extern "C" fn CB_ProcessSelectedMonInput() {
    match Menu_ProcessInputNoWrap() {
        MENU_B_PRESSED => {
            PlaySE(SE_SELECT);
            RedrawChooseAPokemonWindow();
        }
        MENU_NOTHING_CHOSEN => {}
        0 => {
            BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 16, 0);
            (*sTradeMenu).callbackId = CB_SHOW_MON_SUMMARY;
        }
        1 => {
            match CanTradeSelectedMon(
                gPlayerParty.as_mut_ptr(),
                gPlayerPartyCount as i32,
                (*sTradeMenu).cursorPosition as i32,
            ) {
                CAN_TRADE_MON => {
                    SetReadyToTrade();
                    gSprites[(*sTradeMenu).cursorSpriteId].set_invisible(TRUE as u16);
                }
                1 => {
                    QueueAction(QUEUE_DELAY_MSG, QUEUE_ONLY_MON2);
                    (*sTradeMenu).callbackId = CB_HANDLE_TRADE_CANCELED;
                }
                2 | CANT_TRADE_INVALID_MON => {
                    QueueAction(QUEUE_DELAY_MSG, QUEUE_MON_CANT_BE_TRADED);
                    (*sTradeMenu).callbackId = CB_HANDLE_TRADE_CANCELED;
                }
                3 | CANT_TRADE_PARTNER_EGG_YET => {
                    QueueAction(QUEUE_DELAY_MSG, QUEUE_EGG_CANT_BE_TRADED);
                    (*sTradeMenu).callbackId = CB_HANDLE_TRADE_CANCELED;
                }
                _ => {}
            }
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn CB_ChooseMonAfterButtonPress() {
    if gMain.newKeys as i32 & A_BUTTON != 0 || gMain.newKeys as i32 & B_BUTTON != 0 {
        PlaySE(SE_SELECT);
        RedrawChooseAPokemonWindow();
    }
}
pub(crate) unsafe extern "C" fn CB_ShowTradeMonSummaryScreen() {
    if gPaletteFade.active() == 0 {
        if (*sTradeMenu).cursorPosition < PARTY_SIZE as u8 {
            ShowPokemonSummaryScreen(
                SUMMARY_MODE_LOCK_MOVES,
                gPlayerParty.as_mut_ptr() as *mut c_void,
                (*sTradeMenu).cursorPosition,
                (*sTradeMenu).partyCounts[0] - 1,
                Some(CB2_ReturnToTradeMenu),
            );
        } else {
            ShowPokemonSummaryScreen(
                SUMMARY_MODE_LOCK_MOVES,
                gEnemyParty.as_mut_ptr() as *mut c_void,
                (*sTradeMenu).cursorPosition - PARTY_SIZE as u8,
                (*sTradeMenu).partyCounts[1] - 1,
                Some(CB2_ReturnToTradeMenu),
            );
        }
        FreeAllWindowBuffers();
    }
}
pub(crate) unsafe extern "C" fn CheckValidityOfTradeMons(
    aliveMons: *mut u8,
    playerPartyCount: u8,
    playerMonIdx: u8,
    mut partnerMonIdx: u8,
) -> u8 {
    let mut i: i32 = 0;
    let mut partnerSpecies: u16 = 0;
    let mut hasLiveMon: u8 = 0;
    i = 0;
    while i < playerPartyCount as i32 {
        if playerMonIdx as i32 != i {
            hasLiveMon += *aliveMons.at(i);
        }
        i += 1;
    }
    partnerMonIdx = (partnerMonIdx as i32 % 6) as u8;
    partnerSpecies = GetMonData2(&raw mut gEnemyParty[partnerMonIdx], MON_DATA_SPECIES) as u16;
    if partnerSpecies == SPECIES_DEOXYS as u16 || partnerSpecies == SPECIES_MEW as u16 {
        if GetMonData2(
            &raw mut gEnemyParty[partnerMonIdx],
            MON_DATA_MODERN_FATEFUL_ENCOUNTER,
        ) == 0
        {
            return PARTNER_MON_INVALID;
        }
    }
    if IsNationalPokedexEnabled() == 0 {
        if (*sTradeMenu).isEgg[1][partnerMonIdx] != 0 || IsSpeciesInHoennDex(partnerSpecies) == 0 {
            return PARTNER_MON_INVALID;
        }
    }
    if hasLiveMon != 0 {
        hasLiveMon = BOTH_MONS_VALID;
    }
    return hasLiveMon;
}
pub(crate) unsafe extern "C" fn CheckMonsBeforeTrade() -> u32 {
    let mut i: i32 = 0;
    let mut aliveMons: CArray<u8, 12> = zeroed();
    i = 0;
    while i < (*sTradeMenu).partyCounts[0] as i32 {
        aliveMons[i] = (*sTradeMenu).isLiveMon[0][i];
        i += 1;
    }
    match CheckValidityOfTradeMons(
        aliveMons.as_mut_ptr(),
        (*sTradeMenu).partyCounts[0],
        (*sTradeMenu).cursorPosition,
        (*sTradeMenu).partnerCursorPosition,
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
    return FALSE as u32;
}
pub(crate) unsafe extern "C" fn CB_ProcessConfirmTradeInput() {
    match Menu_ProcessInputNoWrapClearOnChoose() {
        0 => {
            if CheckMonsBeforeTrade() == 0 {
                (*sTradeMenu).callbackId = CB_IDLE;
            } else {
                (*sTradeMenu).callbackId = CB_PARTNER_MON_INVALID;
            }
            PutWindowTilemap(17);
        }
        1 | MENU_B_PRESSED => {
            QueueAction(QUEUE_DELAY_MSG, QUEUE_STANDBY);
            if IsLinkTradeTaskFinished() != 0 {
                SetLinkData(LINKCMD_READY_CANCEL_TRADE, 0);
            }
            (*sTradeMenu).callbackId = CB_IDLE;
            PutWindowTilemap(17);
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn RestoreNicknamesCoveredByYesNo() {
    let mut i: i32 = 0;
    i = 0;
    while i < (*sTradeMenu).partyCounts[1] as i32 - 4 {
        PutWindowTilemap(i as u8 + 12);
        CopyWindowToVram(i as u8 + 12, COPYWIN_MAP);
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn CB_ProcessCancelTradeInput() {
    match Menu_ProcessInputNoWrapClearOnChoose() {
        0 => {
            PrintTradeMessage(MSG_WAITING_FOR_FRIEND);
            SetLinkData(LINKCMD_REQUEST_CANCEL, 0);
            gSprites[(*sTradeMenu).cursorSpriteId].set_invisible(TRUE as u16);
            (*sTradeMenu).callbackId = CB_IDLE;
            RestoreNicknamesCoveredByYesNo();
        }
        1 | MENU_B_PRESSED => {
            PlaySE(SE_SELECT);
            RedrawChooseAPokemonWindow();
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn CB_SetSelectedMons() {
    if GetMultiplayerId() == 0 {
        rbox_fill_rectangle(0);
        SetSelectedMon((*sTradeMenu).cursorPosition);
        SetSelectedMon((*sTradeMenu).partnerCursorPosition);
    }
    (*sTradeMenu).callbackId = CB_PRINT_IS_THIS_OKAY;
}
pub(crate) unsafe extern "C" fn CB_PrintIsThisTradeOkay() {
    if (*sTradeMenu).drawSelectedMonState[0] == DRAW_SELECTED_FINISH
        && (*sTradeMenu).drawSelectedMonState[1] == DRAW_SELECTED_FINISH
    {
        PrintIsThisTradeOkay();
        (*sTradeMenu).callbackId = CB_INIT_CONFIRM_TRADE_PROMPT;
    }
}
pub(crate) unsafe extern "C" fn CB_InitConfirmTradePrompt() {
    (*sTradeMenu).timer += 1;
    if (*sTradeMenu).timer > 120 {
        CreateYesNoMenu((&raw const *sTradeYesNoWindowTemplate).cast_mut(), 1, 14, 0);
        (*sTradeMenu).timer = 0;
        (*sTradeMenu).callbackId = CB_CONFIRM_TRADE_PROMPT;
    }
}
pub(crate) unsafe extern "C" fn CB_HandleTradeCanceled() {
    let mut i: i32 = 0;
    if gMain.newKeys as i32 & A_BUTTON != 0 {
        PlaySE(SE_SELECT);
        rbox_fill_rectangle(0);
        rbox_fill_rectangle(1);
        i = 0;
        while i < 4 {
            FillWindowPixelBuffer(i as u8 + 14, 0);
            rbox_fill_rectangle(i as u8 + 14);
            i += 1;
        }
        RedrawPartyWindow(TRADE_PLAYER);
        RedrawPartyWindow(TRADE_PARTNER);
        (*sTradeMenu).callbackId = CB_MAIN_MENU;
        gSprites[(*sTradeMenu).cursorSpriteId].set_invisible(FALSE as u16);
    }
}
pub(crate) unsafe extern "C" fn CB_InitExitCanceledTrade() {
    if gPaletteFade.active() == 0 {
        if gWirelessCommType != 0 {
            SetLinkStandbyCallback();
        } else {
            SetCloseLinkCallbackAndType(12);
        }
        (*sTradeMenu).callbackId = CB_EXIT_CANCELED_TRADE;
    }
}
pub(crate) unsafe extern "C" fn CB_ExitCanceledTrade() {
    if gWirelessCommType != 0 {
        if IsLinkTradeTaskFinished() != 0 && GetNumQueuedActions() == 0 {
            Free(sMenuTextTileBuffer as *mut c_void);
            Free(sTradeMenu as *mut c_void);
            FreeAllWindowBuffers();
            DestroyWirelessStatusIndicatorSprite();
            SetMainCallback2(Some(CB2_ReturnToFieldFromMultiplayer));
        }
    } else {
        if gReceivedRemoteLinkPlayers == 0 {
            Free(sMenuTextTileBuffer as *mut c_void);
            Free(sTradeMenu as *mut c_void);
            FreeAllWindowBuffers();
            SetMainCallback2(Some(CB2_ReturnToFieldFromMultiplayer));
        }
    }
}
pub(crate) unsafe extern "C" fn CB_WaitToStartRfuTrade() {
    if Rfu_SetLinkRecovery(0) == 0 && GetNumQueuedActions() == 0 {
        SetLinkStandbyCallback();
        (*sTradeMenu).callbackId = CB_START_LINK_TRADE;
    }
}
pub(crate) unsafe extern "C" fn CB_PartnersMonWasInvalid() {
    if gMain.newKeys as i32 & A_BUTTON != 0 {
        SetLinkData(LINKCMD_READY_CANCEL_TRADE, 0);
        (*sTradeMenu).callbackId = CB_IDLE;
    }
}
pub(crate) unsafe extern "C" fn RunTradeMenuCallback() {
    match (*sTradeMenu).callbackId {
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
pub(crate) unsafe extern "C" fn SetSelectedMon(cursorPosition: u8) {
    let mut whichParty: u8 = (cursorPosition as i32 / 6) as u8;
    if (*sTradeMenu).drawSelectedMonState[whichParty] == 0 {
        (*sTradeMenu).drawSelectedMonState[whichParty] = 1;
        (*sTradeMenu).selectedMonIdx[whichParty] = cursorPosition;
    }
}
pub(crate) unsafe extern "C" fn DrawSelectedMonScreen(whichParty: u8) {
    let mut nameStringWidth: i8 = 0;
    let mut nickname: CArray<u8, 20> = zeroed();
    let mut movesString: CArray<u8, 56> = zeroed();
    let mut i: u8 = 0;
    let mut partyIdx: u8 = 0;
    let mut selectedMonParty: u8 = 0;
    let mut selectedMonIdx: u8 = (*sTradeMenu).selectedMonIdx[whichParty];
    selectedMonParty = TRADE_PARTNER;
    if (*sTradeMenu).selectedMonIdx[whichParty] < PARTY_SIZE as u8 {
        selectedMonParty = TRADE_PLAYER;
    }
    partyIdx = (selectedMonIdx as i32 % 6) as u8;
    nameStringWidth = 0;
    match (*sTradeMenu).drawSelectedMonState[whichParty] {
        1 => {
            i = 0;
            while i < (*sTradeMenu).partyCounts[whichParty] {
                gSprites[(*sTradeMenu).partySpriteIds[selectedMonParty][i]]
                    .set_invisible(TRUE as u16);
                i += 1;
            }
            i = 0;
            while i < PARTY_SIZE as u8 {
                ClearWindowTilemap(i + (whichParty * PARTY_SIZE as u8 + 2));
                i += 1;
            }
            gSprites[(*sTradeMenu).partySpriteIds[selectedMonParty][partyIdx]]
                .set_invisible(FALSE as u16);
            gSprites[(*sTradeMenu).partySpriteIds[selectedMonParty][partyIdx]].data[0] = 20;
            gSprites[(*sTradeMenu).partySpriteIds[selectedMonParty][partyIdx]].data[2] =
                ((sTradeMonSpriteCoords[selectedMonParty as i32 * PARTY_SIZE][0] as i32
                    + sTradeMonSpriteCoords[selectedMonParty as i32 * PARTY_SIZE + 1][0] as i32)
                    / 2) as i16
                    * 8
                    + 14;
            gSprites[(*sTradeMenu).partySpriteIds[selectedMonParty][partyIdx]].data[4] =
                sTradeMonSpriteCoords[selectedMonParty as i32 * PARTY_SIZE][1] as i16 * 8 - 12;
            StoreSpriteCallbackInData6(
                &raw mut gSprites[(*sTradeMenu).partySpriteIds[selectedMonParty][partyIdx]],
                Some(SpriteCB_MonIcon),
            );
            (*sTradeMenu).drawSelectedMonState[whichParty] += 1;
            Trade_MoveSelectedMonToTarget(
                &raw mut gSprites[(*sTradeMenu).partySpriteIds[selectedMonParty][partyIdx]],
            );
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
            if gSprites[(*sTradeMenu).partySpriteIds[selectedMonParty][partyIdx]].callback
                == Some(SpriteCB_MonIcon as unsafe extern "C" fn(*mut Sprite))
            {
                (*sTradeMenu).drawSelectedMonState[whichParty] = 3;
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
            gSprites[(*sTradeMenu).partySpriteIds[selectedMonParty][partyIdx]].x =
                ((sTradeMonSpriteCoords[selectedMonParty as i32 * PARTY_SIZE][0] as i32
                    + sTradeMonSpriteCoords[selectedMonParty as i32 * PARTY_SIZE + 1][0] as i32)
                    / 2) as i16
                    * 8
                    + 14;
            gSprites[(*sTradeMenu).partySpriteIds[selectedMonParty][partyIdx]].y =
                sTradeMonSpriteCoords[selectedMonParty as i32 * PARTY_SIZE][1] as i16 * 8 - 12;
            gSprites[(*sTradeMenu).partySpriteIds[selectedMonParty][partyIdx]].x2 = 0;
            gSprites[(*sTradeMenu).partySpriteIds[selectedMonParty][partyIdx]].y2 = 0;
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
            (*sTradeMenu).drawSelectedMonState[whichParty] += 1;
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
            (*sTradeMenu).drawSelectedMonState[whichParty] += 1;
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn GetMonNicknameWidth(
    str: *mut u8,
    whichParty: u8,
    partyIdx: u8,
) -> u8 {
    let mut nickname: CArray<u8, 11> = zeroed();
    if whichParty == TRADE_PLAYER {
        GetMonData3(
            &raw mut gPlayerParty[partyIdx],
            MON_DATA_NICKNAME,
            nickname.as_mut_ptr(),
        );
    } else {
        GetMonData3(
            &raw mut gEnemyParty[partyIdx],
            MON_DATA_NICKNAME,
            nickname.as_mut_ptr(),
        );
    }
    StringCopy_Nickname(str, nickname.as_mut_ptr());
    return GetStringWidth(
        FONT_SMALL,
        str,
        GetFontAttribute(FONT_SMALL, FONTATTR_LETTER_SPACING) as i16,
    ) as u8;
}
pub(crate) unsafe extern "C" fn BufferMovesString(str: *mut u8, whichParty: u8, partyIdx: u8) {
    let mut moves: CArray<u16, 4> = zeroed();
    let mut i: u16 = 0;
    if (*sTradeMenu).isEgg[whichParty][partyIdx] == 0 {
        i = 0;
        while i < MAX_MON_MOVES as u16 {
            if whichParty == TRADE_PLAYER {
                moves[i] = GetMonData3(
                    &raw mut gPlayerParty[partyIdx],
                    i as i32 + MON_DATA_MOVE1,
                    null_mut(),
                ) as u16;
            } else {
                moves[i] = GetMonData3(
                    &raw mut gEnemyParty[partyIdx],
                    i as i32 + MON_DATA_MOVE1,
                    null_mut(),
                ) as u16;
            }
            i += 1;
        }
        StringCopy(str, sText_EmptyString.as_ptr().cast_mut());
        i = 0;
        while i < MAX_MON_MOVES as u16 {
            if moves[i] != MOVE_NONE {
                StringAppend(str, gMoveNames[moves[i]].as_ptr().cast_mut());
            }
            StringAppend(str, sText_NewLine.as_ptr().cast_mut());
            i += 1;
        }
    } else {
        StringCopy(str, sText_EmptyString.as_ptr().cast_mut());
        StringAppend(str, sText_FourQuestionMarks.as_ptr().cast_mut());
    }
}
pub(crate) unsafe extern "C" fn PrintPartyMonNickname(
    whichParty: u8,
    mut windowId: u8,
    nickname: *mut u8,
) {
    let mut xPos: u8 = 0;
    windowId += whichParty * PARTY_SIZE as u8 + 2;
    xPos = GetStringCenterAlignXOffset(FONT_SMALL as i32, nickname, 64) as u8;
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
pub(crate) unsafe extern "C" fn PrintPartyNicknames(whichParty: u8) {
    let mut i: u8 = 0;
    let mut nickname: CArray<u8, 20> = zeroed();
    let mut str: CArray<u8, 32> = zeroed();
    let mut party: *mut Pokemon = if whichParty == TRADE_PLAYER {
        gPlayerParty.as_mut_ptr()
    } else {
        gEnemyParty.as_mut_ptr()
    };
    i = 0;
    while i < (*sTradeMenu).partyCounts[whichParty] {
        GetMonData3(party.at(i), MON_DATA_NICKNAME, nickname.as_mut_ptr());
        StringCopy_Nickname(str.as_mut_ptr(), nickname.as_mut_ptr());
        PrintPartyMonNickname(whichParty, i, str.as_mut_ptr());
        i += 1;
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
    let mut level: u8 = 0;
    let mut symbolTile: u32 = 0;
    let mut gender: u8 = 0;
    let mut nickname: CArray<u8, 11> = zeroed();
    CopyToBgTilemapBufferRect_ChangePalette(
        1,
        gTradeMenuMonBox_Tilemap.as_ptr().cast_mut() as *mut c_void,
        width,
        height,
        6,
        3,
        0,
    );
    CopyBgTilemapBufferToVram(1);
    if whichParty == TRADE_PLAYER {
        level = GetMonData3(&raw mut gPlayerParty[monIdx], MON_DATA_LEVEL, null_mut()) as u8;
    } else {
        level = GetMonData3(&raw mut gEnemyParty[monIdx], MON_DATA_LEVEL, null_mut()) as u8;
    }
    if (*sTradeMenu).isEgg[whichParty][monIdx] == 0 {
        if level as i32 / 10 != 0 {
            (*sTradeMenu).tilemapBuffer[x as i32 + y as i32 * 32] =
                (level as i32 / 10) as u16 + 0x60;
        }
        (*sTradeMenu).tilemapBuffer[x as i32 + y as i32 * 32 + 1] =
            (level as i32 % 10) as u16 + 0x70;
    } else {
        (*sTradeMenu).tilemapBuffer[x as i32 + y as i32 * 32 - 32] =
            (*sTradeMenu).tilemapBuffer[x as i32 + y as i32 * 32 - 33];
        (*sTradeMenu).tilemapBuffer[x as i32 + y as i32 * 32 - 31] =
            (*sTradeMenu).tilemapBuffer[x as i32 + y as i32 * 32 - 36] | 0x400;
    }
    if (*sTradeMenu).isEgg[whichParty][monIdx] != 0 {
        symbolTile = 0x480;
    } else {
        if whichParty == TRADE_PLAYER {
            gender = GetMonGender(&raw mut gPlayerParty[monIdx]);
            GetMonData3(
                &raw mut gPlayerParty[monIdx],
                MON_DATA_NICKNAME,
                nickname.as_mut_ptr(),
            );
        } else {
            gender = GetMonGender(&raw mut gEnemyParty[monIdx]);
            GetMonData3(
                &raw mut gEnemyParty[monIdx],
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
    (*sTradeMenu).tilemapBuffer[(y as i32 - 1) * 32 + x as i32 + 1] = symbolTile as u16;
}
pub(crate) unsafe extern "C" fn PrintPartyLevelsAndGenders(whichParty: u8) {
    let mut i: i32 = 0;
    i = 0;
    while i < (*sTradeMenu).partyCounts[whichParty] as i32 {
        let mut j: i32 = i + PARTY_SIZE * whichParty as i32;
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
pub(crate) unsafe extern "C" fn ShowTradePartyMonIcons(whichParty: u8) {
    let mut i: i32 = 0;
    i = 0;
    while i < (*sTradeMenu).partyCounts[whichParty] as i32 {
        gSprites[(*sTradeMenu).partySpriteIds[whichParty][i]].set_invisible(FALSE as u16);
        gSprites[(*sTradeMenu).partySpriteIds[whichParty][i]].x =
            sTradeMonSpriteCoords[whichParty as i32 * PARTY_SIZE + i][0] as i16 * 8 + 14;
        gSprites[(*sTradeMenu).partySpriteIds[whichParty][i]].y =
            sTradeMonSpriteCoords[whichParty as i32 * PARTY_SIZE + i][1] as i16 * 8 - 12;
        gSprites[(*sTradeMenu).partySpriteIds[whichParty][i]].x2 = 0;
        gSprites[(*sTradeMenu).partySpriteIds[whichParty][i]].y2 = 0;
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn PrintTradePartnerPartyNicknames() {
    rbox_fill_rectangle(1);
    PrintPartyNicknames(TRADE_PARTNER);
}
pub(crate) unsafe extern "C" fn RedrawPartyWindow(whichParty: u8) {
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
        (OBJ_VRAM0 + (*sTradeMenu).bottomTextTileStart as i32 * 32) as usize as *mut c_void
            as *mut u8,
        24,
    );
    (*sTradeMenu).drawSelectedMonState[whichParty] = 0;
}
pub(crate) unsafe extern "C" fn Task_DrawSelectionSummary(taskId: u8) {
    FillBgTilemapBufferRect_Palette0(0, 0, 0, 0, DISPLAY_TILE_WIDTH, DISPLAY_TILE_HEIGHT);
    CopyBgTilemapBufferToVram(0);
}
pub(crate) unsafe extern "C" fn Task_DrawSelectionTrade(taskId: u8) {
    FillBgTilemapBufferRect_Palette0(0, 0, 0, 0, DISPLAY_TILE_WIDTH, DISPLAY_TILE_HEIGHT);
    CopyBgTilemapBufferToVram(0);
}
pub(crate) unsafe extern "C" fn QueueAction(delay: u16, actionId: u8) {
    let mut i: i32 = 0;
    i = 0;
    while i < 4 {
        if (*sTradeMenu).queuedActions[i].active == 0 {
            (*sTradeMenu).queuedActions[i].delay = delay;
            (*sTradeMenu).queuedActions[i].actionId = actionId;
            (*sTradeMenu).queuedActions[i].active = TRUE;
            break;
        }
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn GetNumQueuedActions() -> u32 {
    let mut numActions: u32 = 0;
    let mut i: i32 = 0;
    i = 0;
    while i < 4 {
        numActions += (*sTradeMenu).queuedActions[i].active as u32;
        i += 1;
    }
    return numActions;
}
pub(crate) unsafe extern "C" fn DoQueuedActions() {
    let mut i: i32 = 0;
    i = 0;
    while i < 4 {
        if (*sTradeMenu).queuedActions[i].active != 0 {
            if (*sTradeMenu).queuedActions[i].delay != 0 {
                (*sTradeMenu).queuedActions[i].delay -= 1;
            } else {
                match (*sTradeMenu).queuedActions[i].actionId {
                    QUEUE_SEND_DATA => {
                        SendLinkData((*sTradeMenu).linkData.as_mut_ptr() as *mut c_void, 20);
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
                (*sTradeMenu).queuedActions[i].active = FALSE;
            }
        }
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn PrintTradeMessage(messageId: u8) {
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
pub(crate) unsafe extern "C" fn LoadUISpriteGfx() -> u8 {
    let mut sheet: SpriteSheet = zeroed();
    if (*sTradeMenu).timer < NUM_MENU_TEXT_SPRITES {
        sheet.data = sMenuTextTileBuffers[(*sTradeMenu).timer] as *mut c_void;
        sheet.size = 0x100;
        sheet.tag = GFXTAG_MENU_TEXT + (*sTradeMenu).timer as u16;
    }
    match (*sTradeMenu).timer {
        0
        | GFXTAG_PLAYER_NAME_M
        | GFXTAG_PLAYER_NAME_R
        | 3
        | GFXTAG_PARTNER_NAME_M
        | GFXTAG_PARTNER_NAME_R
        | 6
        | 7 => {
            LoadSpriteSheet(&raw mut sheet);
            (*sTradeMenu).timer += 1;
        }
        8 => {
            (*sTradeMenu).bottomTextTileStart = LoadSpriteSheet(&raw mut sheet);
            (*sTradeMenu).timer += 1;
        }
        GFXTAG_CHOOSE_PKMN_M
        | GFXTAG_CHOOSE_PKMN_R
        | GFXTAG_CHOOSE_PKMN_EMPTY_1
        | GFXTAG_CHOOSE_PKMN_EMPTY_2
        | GFXTAG_CHOOSE_PKMN_EMPTY_3 => {
            LoadSpriteSheet(&raw mut sheet);
            (*sTradeMenu).timer += 1;
        }
        NUM_MENU_TEXT_SPRITES => {
            LoadSpritePalette((&raw const *sSpritePalette_MenuText).cast_mut());
            (*sTradeMenu).timer += 1;
        }
        15 => {
            LoadSpritePalette((&raw const *sCursor_SpritePalette).cast_mut());
            (*sTradeMenu).timer += 1;
        }
        16 => {
            LoadSpriteSheet((&raw const *sCursor_SpriteSheet).cast_mut());
            (*sTradeMenu).timer += 1;
        }
        17 => {
            (*sTradeMenu).timer = 0;
            return TRUE;
        }
        _ => {}
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn DrawBottomRowText(str: *mut u8, dest: *mut u8, unused: u8) {
    DrawTextWindowAndBufferTiles(str, dest as *mut c_void, 0, 0, 6);
}
pub(crate) unsafe extern "C" fn ComputePartyTradeableFlags(whichParty: u8) {
    let mut i: i32 = 0;
    match whichParty {
        TRADE_PLAYER => {
            i = 0;
            while i < (*sTradeMenu).partyCounts[whichParty] as i32 {
                if GetMonData2(&raw mut gPlayerParty[i], MON_DATA_IS_EGG) == TRUE as u32 {
                    (*sTradeMenu).isLiveMon[whichParty][i] = FALSE;
                    (*sTradeMenu).isEgg[whichParty][i] = TRUE;
                } else if GetMonData2(&raw mut gPlayerParty[i], MON_DATA_HP) == 0 {
                    (*sTradeMenu).isLiveMon[whichParty][i] = FALSE;
                    (*sTradeMenu).isEgg[whichParty][i] = FALSE;
                } else {
                    (*sTradeMenu).isLiveMon[whichParty][i] = TRUE;
                    (*sTradeMenu).isEgg[whichParty][i] = FALSE;
                }
                i += 1;
            }
        }
        TRADE_PARTNER => {
            i = 0;
            while i < (*sTradeMenu).partyCounts[whichParty] as i32 {
                if GetMonData2(&raw mut gEnemyParty[i], MON_DATA_IS_EGG) == TRUE as u32 {
                    (*sTradeMenu).isLiveMon[whichParty][i] = FALSE;
                    (*sTradeMenu).isEgg[whichParty][i] = TRUE;
                } else if GetMonData2(&raw mut gEnemyParty[i], MON_DATA_HP) == 0 {
                    (*sTradeMenu).isLiveMon[whichParty][i] = FALSE;
                    (*sTradeMenu).isEgg[whichParty][i] = FALSE;
                } else {
                    (*sTradeMenu).isLiveMon[whichParty][i] = TRUE;
                    (*sTradeMenu).isEgg[whichParty][i] = FALSE;
                }
                i += 1;
            }
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn ComputePartyHPBarLevels(whichParty: u8) {
    let mut i: u16 = 0;
    let mut curHp: u16 = 0;
    let mut maxHp: u16 = 0;
    match whichParty {
        TRADE_PLAYER => {
            i = 0;
            while i < (*sTradeMenu).partyCounts[0] as u16 {
                curHp = GetMonData2(&raw mut gPlayerParty[i], MON_DATA_HP) as u16;
                maxHp = GetMonData2(&raw mut gPlayerParty[i], MON_DATA_MAX_HP) as u16;
                (*sTradeMenu).hpBarLevels[0][i] = GetHPBarLevel(curHp as i16, maxHp as i16);
                i += 1;
            }
        }
        TRADE_PARTNER => {
            i = 0;
            while i < (*sTradeMenu).partyCounts[1] as u16 {
                curHp = GetMonData2(&raw mut gEnemyParty[i], MON_DATA_HP) as u16;
                maxHp = GetMonData2(&raw mut gEnemyParty[i], MON_DATA_MAX_HP) as u16;
                (*sTradeMenu).hpBarLevels[1][i] = GetHPBarLevel(curHp as i16, maxHp as i16);
                i += 1;
            }
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn SetTradePartyHPBarSprites() {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    i = 0;
    while i < 2 {
        j = 0;
        while j < (*sTradeMenu).partyCounts[i] as i32 {
            SetPartyHPBarSprite(
                &raw mut gSprites[(*sTradeMenu).partySpriteIds[i][j]],
                4 - (*sTradeMenu).hpBarLevels[i][j],
            );
            j += 1;
        }
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn SaveTradeGiftRibbons() {
    let mut i: i32 = 0;
    i = 0;
    while i < 11 {
        if (*gSaveBlock1Ptr).giftRibbons[i] == 0 && (*sTradeMenu).giftRibbons[i] != 0 {
            if (*sTradeMenu).giftRibbons[i] < MAX_GIFT_RIBBON {
                (*gSaveBlock1Ptr).giftRibbons[i] = (*sTradeMenu).giftRibbons[i];
            }
        }
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn CanTradeSelectedMon(
    mut playerParty: *mut Pokemon,
    partyCount: i32,
    monIdx: i32,
) -> u32 {
    let mut i: i32 = 0;
    let mut numMonsLeft: i32 = 0;
    let mut partner: *mut LinkPlayer = null_mut();
    let mut species: CArray<u32, 6> = zeroed();
    let mut species2: CArray<u32, 6> = zeroed();
    i = 0;
    while i < partyCount {
        species2[i] = GetMonData2(playerParty.at(i), MON_DATA_SPECIES_OR_EGG);
        species[i] = GetMonData2(playerParty.at(i), MON_DATA_SPECIES);
        i += 1;
    }
    if IsNationalPokedexEnabled() == 0 {
        if species2[monIdx] == SPECIES_EGG {
            return CANT_TRADE_EGG_YET as u32;
        }
        if IsSpeciesInHoennDex(species2[monIdx] as u16) == 0 {
            return CANT_TRADE_NATIONAL as u32;
        }
    }
    partner = &raw mut gLinkPlayers[GetMultiplayerId() as i32 ^ 1];
    if (*partner).version as i32 & 0xFF != VERSION_RUBY
        && (*partner).version as i32 & 0xFF != VERSION_SAPPHIRE
    {
        if (*partner).progressFlagsCopy as i32 & 0xF == 0 {
            if species2[monIdx] == SPECIES_EGG {
                return CANT_TRADE_PARTNER_EGG_YET;
            }
            if IsSpeciesInHoennDex(species2[monIdx] as u16) == 0 {
                return CANT_TRADE_INVALID_MON;
            }
        }
    }
    if species[monIdx] == SPECIES_DEOXYS || species[monIdx] == SPECIES_MEW {
        if GetMonData2(playerParty.at(monIdx), MON_DATA_MODERN_FATEFUL_ENCOUNTER) == 0 {
            return CANT_TRADE_INVALID_MON;
        }
    }
    i = 0;
    while i < partyCount {
        if species2[i] == SPECIES_EGG {
            species2[i] = SPECIES_NONE as u32;
        }
        i += 1;
    }
    numMonsLeft = 0;
    i = 0;
    while i < partyCount {
        if i != monIdx {
            numMonsLeft += species2[i] as i32;
        }
        i += 1;
    }
    if numMonsLeft != 0 {
        return CAN_TRADE_MON;
    } else {
        return CANT_TRADE_LAST_MON as u32;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetGameProgressForLinkTrade() -> i32 {
    let mut versionId: i32 = 0;
    let mut version: u16 = 0;
    if gReceivedRemoteLinkPlayers != 0 {
        versionId = 0;
        version = gLinkPlayers[GetMultiplayerId() as i32 ^ 1].version & 0xFF;
        if version == VERSION_RUBY as u16
            || version == VERSION_SAPPHIRE as u16
            || version == VERSION_EMERALD as u16
        {
            versionId = 0;
        } else if version == VERSION_FIRE_RED as u16 || version == VERSION_LEAF_GREEN as u16 {
            versionId = 2;
        }
        if versionId > 0 {
            if gLinkPlayers[GetMultiplayerId()].progressFlagsCopy as i32 & 0xF0 != 0 {
                if versionId == 2 {
                    if gLinkPlayers[GetMultiplayerId() as i32 ^ 1].progressFlagsCopy as i32 & 0xF0
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
    return TRADE_BOTH_PLAYERS_READY;
}
pub(crate) unsafe extern "C" fn IsDeoxysOrMewUntradable(
    species: u16,
    isModernFatefulEncounter: u8,
) -> u32 {
    if species == SPECIES_DEOXYS as u16 || species == SPECIES_MEW as u16 {
        if isModernFatefulEncounter == 0 {
            return TRUE as u32;
        }
    }
    return FALSE as u32;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetUnionRoomTradeMessageId(
    mut player: RfuGameCompatibilityData,
    mut partner: RfuGameCompatibilityData,
    playerSpecies2: u16,
    partnerSpecies: u16,
    requestedType: u8,
    playerSpecies: u16,
    isModernFatefulEncounter: u8,
) -> i32 {
    let mut playerHasNationalDex: u8 = player.hasNationalDex() as u8;
    let mut playerCanLinkNationally: u8 = player.canLinkNationally() as u8;
    let mut partnerHasNationalDex: u8 = partner.hasNationalDex() as u8;
    let mut partnerCanLinkNationally: u8 = partner.canLinkNationally() as u8;
    let mut partnerVersion: u8 = partner.version() as u8;
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
        if gSpeciesInfo[playerSpecies2].types[0] != requestedType
            && gSpeciesInfo[playerSpecies2].types[1] != requestedType
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
    return UR_TRADE_MSG_NONE as i32;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CanRegisterMonForTradingBoard(
    mut player: RfuGameCompatibilityData,
    species2: u16,
    species: u16,
    isModernFatefulEncounter: u8,
) -> i32 {
    let mut hasNationalDex: u8 = player.hasNationalDex() as u8;
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
    return CANT_REGISTER_MON;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CanSpinTradeMon(mut mon: *mut Pokemon, monIdx: u16) -> i32 {
    let mut i: i32 = 0;
    let mut version: i32 = 0;
    let mut versions: i32 = 0;
    let mut canTradeAnyMon: i32 = 0;
    let mut numMonsLeft: i32 = 0;
    let mut speciesArray: CArray<i32, 6> = zeroed();
    i = 0;
    while i < gPlayerPartyCount as i32 {
        speciesArray[i] = GetMonData2(mon.at(i), MON_DATA_SPECIES_OR_EGG) as i32;
        if speciesArray[i] == SPECIES_EGG as i32 {
            speciesArray[i] = SPECIES_NONE as i32;
        }
        i += 1;
    }
    versions = 0;
    canTradeAnyMon = TRUE as i32;
    i = 0;
    while i < GetLinkPlayerCount() as i32 {
        version = gLinkPlayers[i].version as i32 & 0xFF;
        if version == VERSION_FIRE_RED || version == VERSION_LEAF_GREEN {
            versions = 0;
        } else {
            versions |= 1;
        }
        i += 1;
    }
    i = 0;
    while i < GetLinkPlayerCount() as i32 {
        let mut player: *mut LinkPlayer = &raw mut gLinkPlayers[i];
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
    numMonsLeft = 0;
    i = 0;
    while i < gPlayerPartyCount as i32 {
        if monIdx as i32 != i {
            numMonsLeft += speciesArray[i];
        }
        i += 1;
    }
    if numMonsLeft == 0 {
        return CANT_TRADE_LAST_MON;
    } else {
        return CAN_TRADE_MON as i32;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_LinkMonGlow(sprite: *mut Sprite) {
    if ({
        (*sprite).data[0] += 1;
        (*sprite).data[0]
    }) == 10
    {
        PlaySE(SE_BALL);
        (*sprite).data[0] = 0;
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_LinkMonGlowWireless(sprite: *mut Sprite) {
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
pub(crate) unsafe extern "C" fn SpriteCB_LinkMonShadow(sprite: *mut Sprite) {
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
pub(crate) unsafe extern "C" fn SpriteCB_CableEndSending(sprite: *mut Sprite) {
    (*sprite).data[0] += 1;
    (*sprite).y2 += 1;
    if (*sprite).data[0] == 10 {
        DestroySprite(sprite);
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_CableEndReceiving(sprite: *mut Sprite) {
    (*sprite).data[0] += 1;
    (*sprite).y2 -= 1;
    if (*sprite).data[0] == 10 {
        DestroySprite(sprite);
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_GbaScreen(sprite: *mut Sprite) {
    if ({
        (*sprite).data[0] += 1;
        (*sprite).data[0]
    }) == 15
    {
        PlaySE(SE_M_MINIMIZE);
        (*sprite).data[0] = 0;
    }
}
pub(crate) unsafe extern "C" fn SetTradeBGAffine() {
    let mut affine: BgAffineDstData = zeroed();
    DoBgAffineSet(
        &raw mut affine,
        (*sTradeAnim).texX as u32 * 0x100,
        (*sTradeAnim).texY as u32 * 0x100,
        (*sTradeAnim).scrX as i16,
        (*sTradeAnim).scrY as i16,
        (*sTradeAnim).sXY as i16,
        (*sTradeAnim).sXY as i16,
        (*sTradeAnim).alpha,
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
pub(crate) unsafe extern "C" fn SetTradeGpuRegs() {
    let mut dispcnt: u16 = 0;
    SetGpuReg(REG_OFFSET_BG1VOFS, (*sTradeAnim).bg1vofs as u16);
    SetGpuReg(REG_OFFSET_BG1HOFS, (*sTradeAnim).bg1hofs as u16);
    dispcnt = GetGpuReg(REG_OFFSET_DISPCNT);
    if dispcnt as i32 & 7 == DISPCNT_MODE_0 {
        SetGpuReg(REG_OFFSET_BG2VOFS, (*sTradeAnim).bg2vofs as u16);
        SetGpuReg(REG_OFFSET_BG2HOFS, (*sTradeAnim).bg2hofs as u16);
    } else {
        SetTradeBGAffine();
    }
}
pub(crate) unsafe extern "C" fn VBlankCB_TradeAnim() {
    SetTradeGpuRegs();
    LoadOam();
    ProcessSpriteCopyRequests();
    TransferPlttBuffer();
}
pub(crate) unsafe extern "C" fn ClearLinkTimeoutTimer() {
    (*sTradeAnim).linkTimeoutTimer = 0;
    (*sTradeAnim).linkTimeoutZero1 = 0;
    (*sTradeAnim).linkTimeoutZero2 = 0;
}
pub(crate) unsafe extern "C" fn CheckForLinkTimeout() {
    if (*sTradeAnim).linkTimeoutZero1 == (*sTradeAnim).linkTimeoutZero2 {
        (*sTradeAnim).linkTimeoutTimer += 1;
    } else {
        (*sTradeAnim).linkTimeoutTimer = 0;
    }
    if (*sTradeAnim).linkTimeoutTimer > 300 {
        CloseLink();
        SetMainCallback2(Some(CB2_LinkError));
        (*sTradeAnim).linkTimeoutTimer = 0;
        (*sTradeAnim).linkTimeoutZero2 = 0;
        (*sTradeAnim).linkTimeoutZero1 = 0;
    }
    (*sTradeAnim).linkTimeoutZero2 = (*sTradeAnim).linkTimeoutZero1;
}
pub(crate) unsafe extern "C" fn TradeGetMultiplayerId() -> u32 {
    if gReceivedRemoteLinkPlayers != 0 {
        return GetMultiplayerId() as u32;
    }
    return 0;
}
pub(crate) unsafe extern "C" fn LoadTradeMonPic(whichParty: u8, state: u8) {
    let mut pos: i32 = 0;
    let mut mon: *mut Pokemon = null_mut();
    let mut species: u16 = 0;
    let mut personality: u32 = 0;
    if whichParty == TRADE_PLAYER {
        mon = &raw mut gPlayerParty[gSelectedTradeMonPositions[0]];
        pos = B_POSITION_OPPONENT_LEFT as i32;
    }
    if whichParty == TRADE_PARTNER {
        mon = &raw mut gEnemyParty[gSelectedTradeMonPositions[1] as i32 % 6];
        pos = B_POSITION_OPPONENT_RIGHT as i32;
    }
    match state {
        0 => {
            species = GetMonData2(mon, MON_DATA_SPECIES_OR_EGG) as u16;
            personality = GetMonData2(mon, MON_DATA_PERSONALITY);
            if whichParty == TRADE_PLAYER {
                HandleLoadSpecialPokePic_2(
                    (&raw const gMonFrontPicTable[species]).cast_mut(),
                    (*gMonSpritesGfxPtr).sprites.ptr[1],
                    species as i32,
                    personality,
                );
            } else {
                HandleLoadSpecialPokePic_DontHandleDeoxys(
                    (&raw const gMonFrontPicTable[species]).cast_mut(),
                    (*gMonSpritesGfxPtr).sprites.ptr
                        [whichParty as i32 * 2 + B_POSITION_OPPONENT_LEFT as i32],
                    species as i32,
                    personality,
                );
            }
            LoadCompressedSpritePalette(GetMonSpritePalStruct(mon));
            (*sTradeAnim).monSpecies[whichParty] = species;
            (*sTradeAnim).monPersonalities[whichParty] = personality;
        }
        1 => {
            SetMultiuseSpriteTemplateToPokemon((*GetMonSpritePalStruct(mon)).tag, pos as u8);
            (*sTradeAnim).monSpriteIds[whichParty] =
                CreateSprite(&raw mut gMultiuseSpriteTemplate, 120, 60, 6);
            gSprites[(*sTradeAnim).monSpriteIds[whichParty]].set_invisible(TRUE as u16);
            gSprites[(*sTradeAnim).monSpriteIds[whichParty]].callback = Some(SpriteCallbackDummy);
        }
        _ => {}
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CB2_LinkTrade() {
    match gMain.state {
        0 => {
            if gReceivedRemoteLinkPlayers == 0 {
                gLinkType = LINKTYPE_TRADE_DISCONNECTED;
                CloseLink();
            }
            sTradeAnim = AllocZeroed(256) as *mut typeof___sTradeAnim_0_t;
            AllocateMonSpritesGfx();
            ResetTasks();
            ResetSpriteData();
            FreeAllSpritePalettes();
            SetVBlankCallback(Some(VBlankCB_TradeAnim));
            TradeAnimInit_LoadGfx();
            ClearLinkTimeoutTimer();
            gMain.state += 1;
            (*sTradeAnim).neverRead_8C = 0;
            (*sTradeAnim).state = 0;
            (*sTradeAnim).isLinkTrade = TRUE;
            (*sTradeAnim).texX = 64;
            (*sTradeAnim).texY = 64;
            (*sTradeAnim).neverRead_D8 = 0;
            (*sTradeAnim).neverRead_DA = 0;
            (*sTradeAnim).scrX = 120;
            (*sTradeAnim).scrY = 80;
            (*sTradeAnim).sXY = 256;
            (*sTradeAnim).alpha = 0;
        }
        1 => {
            if gReceivedRemoteLinkPlayers == 0 {
                (*sTradeAnim).isCableTrade = TRUE;
                OpenLink();
                gMain.state += 1;
                (*sTradeAnim).timer = 0;
            } else {
                gMain.state = 4;
            }
        }
        2 => {
            if ({
                (*sTradeAnim).timer += 1;
                (*sTradeAnim).timer
            }) > 60
            {
                (*sTradeAnim).timer = 0;
                gMain.state += 1;
            }
        }
        3 => {
            if IsLinkMaster() != 0 {
                if GetLinkPlayerCount_2() >= GetSavedPlayerCount() {
                    if ({
                        (*sTradeAnim).timer += 1;
                        (*sTradeAnim).timer
                    }) > 30
                    {
                        CheckShouldAdvanceLinkState();
                        gMain.state += 1;
                    }
                } else {
                    CheckForLinkTimeout();
                }
            } else {
                gMain.state += 1;
            }
        }
        4 => {
            CheckForLinkTimeout();
            if gReceivedRemoteLinkPlayers == TRUE && IsLinkPlayerDataExchangeComplete() == TRUE {
                gMain.state += 1;
            }
        }
        5 => {
            (*sTradeAnim).playerFinishStatus = 0;
            (*sTradeAnim).partnerFinishStatus = 0;
            (*sTradeAnim).scheduleLinkTransfer = 0;
            LoadTradeMonPic(0, 0);
            gMain.state += 1;
        }
        6 => {
            LoadTradeMonPic(TRADE_PLAYER, 1);
            gMain.state += 1;
        }
        7 => {
            LoadTradeMonPic(TRADE_PARTNER, 0);
            gMain.state += 1;
        }
        8 => {
            LoadTradeMonPic(1, 1);
            LinkTradeDrawWindow();
            gMain.state += 1;
        }
        9 => {
            LoadTradeSequenceSpriteSheetsAndPalettes();
            LoadSpriteSheet((&raw const *sPokeBallSpriteSheet).cast_mut());
            LoadSpritePalette((&raw const *sPokeBallSpritePalette).cast_mut());
            gMain.state += 1;
        }
        10 => {
            BeginNormalPaletteFade(PALETTES_ALL, 0, 16, 0, 0);
            ShowBg(0);
            gMain.state += 1;
        }
        11 => {
            InitTradeSequenceBgGpuRegs();
            BufferTradeSceneStrings();
            gMain.state += 1;
        }
        12 => {
            if gPaletteFade.active() == 0 {
                if gWirelessCommType != 0 {
                    LoadWirelessStatusIndicatorSpriteGfx();
                    CreateWirelessStatusIndicatorSprite(0, 0);
                }
                SetMainCallback2(Some(CB2_UpdateLinkTrade));
            }
        }
        _ => {}
    }
    RunTasks();
    RunTextPrinters();
    AnimateSprites();
    BuildOamBuffer();
    UpdatePaletteFade();
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn InitTradeSequenceBgGpuRegs() {
    SetTradeSequenceBgGpuRegs(5);
    SetTradeSequenceBgGpuRegs(0);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn LinkTradeDrawWindow() {
    FillWindowPixelBuffer(0, 255);
    PutWindowTilemap(0);
    CopyWindowToVram(0, COPYWIN_FULL);
}
pub(crate) unsafe extern "C" fn TradeAnimInit_LoadGfx() {
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
        gBattleTextboxTiles.as_ptr().cast_mut() as *mut c_void,
        0,
        0,
        0,
    );
    LZDecompressWram(
        gBattleTextboxTilemap.as_ptr().cast_mut(),
        gDecompressionBuffer.as_mut_ptr() as *mut c_void,
    );
    CopyToBgTilemapBuffer(
        0,
        gDecompressionBuffer.as_mut_ptr() as *mut c_void,
        BG_SCREEN_SIZE as u16,
        0,
    );
    LoadCompressedPalette(gBattleTextboxPalette.as_ptr().cast_mut(), 0, 32);
    InitWindows(sTradeSequenceWindowTemplates.as_ptr().cast_mut());
    DecompressAndLoadBgGfxUsingHeap(
        0,
        gBattleTextboxTiles.as_ptr().cast_mut() as *mut c_void,
        0,
        0,
        0,
    );
    LZDecompressWram(
        gBattleTextboxTilemap.as_ptr().cast_mut(),
        gDecompressionBuffer.as_mut_ptr() as *mut c_void,
    );
    CopyToBgTilemapBuffer(
        0,
        gDecompressionBuffer.as_mut_ptr() as *mut c_void,
        BG_SCREEN_SIZE as u16,
        0,
    );
    LoadCompressedPalette(gBattleTextboxPalette.as_ptr().cast_mut(), 0, 32);
}
pub(crate) unsafe extern "C" fn CB2_InitInGameTrade() {
    let mut otName: CArray<u8, 11> = zeroed();
    match gMain.state {
        0 => {
            gSelectedTradeMonPositions[0] = gSpecialVar_0x8005 as u8;
            gSelectedTradeMonPositions[1] = PARTY_SIZE as u8;
            StringCopy(
                gLinkPlayers[0].name.as_mut_ptr(),
                (*gSaveBlock2Ptr).playerName.as_mut_ptr(),
            );
            GetMonData3(
                &raw mut gEnemyParty[0],
                MON_DATA_OT_NAME,
                otName.as_mut_ptr(),
            );
            StringCopy(gLinkPlayers[1].name.as_mut_ptr(), otName.as_mut_ptr());
            gLinkPlayers[0].language = GAME_LANGUAGE as u16;
            gLinkPlayers[1].language =
                GetMonData2(&raw mut gEnemyParty[0], MON_DATA_LANGUAGE) as u16;
            sTradeAnim = AllocZeroed(256) as *mut typeof___sTradeAnim_0_t;
            AllocateMonSpritesGfx();
            ResetTasks();
            ResetSpriteData();
            FreeAllSpritePalettes();
            SetVBlankCallback(Some(VBlankCB_TradeAnim));
            TradeAnimInit_LoadGfx();
            (*sTradeAnim).isLinkTrade = FALSE;
            (*sTradeAnim).neverRead_8C = 0;
            (*sTradeAnim).state = 0;
            (*sTradeAnim).texX = 64;
            (*sTradeAnim).texY = 64;
            (*sTradeAnim).neverRead_D8 = 0;
            (*sTradeAnim).neverRead_DA = 0;
            (*sTradeAnim).scrX = 120;
            (*sTradeAnim).scrY = 80;
            (*sTradeAnim).sXY = 256;
            (*sTradeAnim).alpha = 0;
            (*sTradeAnim).timer = 0;
            gMain.state = 5;
        }
        5 => {
            LoadTradeMonPic(0, 0);
            gMain.state += 1;
        }
        6 => {
            LoadTradeMonPic(TRADE_PLAYER, 1);
            gMain.state += 1;
        }
        7 => {
            LoadTradeMonPic(TRADE_PARTNER, 0);
            ShowBg(0);
            gMain.state += 1;
        }
        8 => {
            LoadTradeMonPic(1, 1);
            FillWindowPixelBuffer(0, 255);
            PutWindowTilemap(0);
            CopyWindowToVram(0, COPYWIN_FULL);
            gMain.state += 1;
        }
        9 => {
            LoadTradeSequenceSpriteSheetsAndPalettes();
            LoadSpriteSheet((&raw const *sPokeBallSpriteSheet).cast_mut());
            LoadSpritePalette((&raw const *sPokeBallSpritePalette).cast_mut());
            gMain.state += 1;
        }
        10 => {
            ShowBg(0);
            gMain.state += 1;
        }
        11 => {
            SetTradeSequenceBgGpuRegs(5);
            SetTradeSequenceBgGpuRegs(0);
            BufferTradeSceneStrings();
            gMain.state += 1;
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
pub(crate) unsafe extern "C" fn UpdatePokedexForReceivedMon(partyIdx: u8) {
    let mut mon: *mut Pokemon = &raw mut gPlayerParty[partyIdx];
    if GetMonData2(mon, MON_DATA_IS_EGG) == 0 {
        let mut species: u16 = GetMonData3(mon, MON_DATA_SPECIES, null_mut()) as u16;
        let mut personality: u32 = GetMonData3(mon, MON_DATA_PERSONALITY, null_mut());
        species = SpeciesToNationalPokedexNum(species);
        GetSetPokedexFlag(species, FLAG_SET_SEEN);
        HandleSetPokedexFlag(species, FLAG_SET_CAUGHT, personality);
    }
}
pub(crate) unsafe extern "C" fn TryEnableNationalDexFromLinkPartner() {
    let mut mpId: u8 = GetMultiplayerId();
}
pub(crate) unsafe extern "C" fn TradeMons(playerPartyIdx: u8, partnerPartyIdx: u8) {
    let mut friendship: u8 = 0;
    let mut playerMon: *mut Pokemon = &raw mut gPlayerParty[playerPartyIdx];
    let mut playerMail: u16 = GetMonData2(playerMon, MON_DATA_MAIL) as u16;
    let mut partnerMon: *mut Pokemon = &raw mut gEnemyParty[partnerPartyIdx];
    let mut partnerMail: u16 = GetMonData2(partnerMon, MON_DATA_MAIL) as u16;
    if playerMail != MAIL_NONE as u16 {
        ClearMail(&raw mut (*gSaveBlock1Ptr).mail[playerMail]);
    }
    (*sTradeAnim).tempMon = *playerMon;
    *playerMon = *partnerMon;
    *partnerMon = (*sTradeAnim).tempMon;
    friendship = 70;
    if GetMonData2(playerMon, MON_DATA_IS_EGG) == 0 {
        SetMonData(
            playerMon,
            MON_DATA_FRIENDSHIP,
            &raw mut friendship as *mut c_void,
        );
    }
    if partnerMail != MAIL_NONE as u16 {
        GiveMailToMon(playerMon, &raw mut gTradeMail[partnerMail]);
    }
    UpdatePokedexForReceivedMon(playerPartyIdx);
    if gReceivedRemoteLinkPlayers != 0 {
        TryEnableNationalDexFromLinkPartner();
    }
}
pub(crate) unsafe extern "C" fn HandleLinkDataSend() {
    'l1: {
        let sw1: u8 = (*sTradeAnim).scheduleLinkTransfer;
        let mut fall = false;
        if sw1 == 1 {
            fall = true;
            if IsLinkTaskFinished() != 0 {
                SendBlock(
                    BitmaskAllOtherLinkPlayers(),
                    (*sTradeAnim).linkData.as_mut_ptr() as *mut c_void,
                    20,
                );
                (*sTradeAnim).scheduleLinkTransfer += 1;
            }
        }
        if fall || sw1 == 2 {
            fall = true;
            (*sTradeAnim).scheduleLinkTransfer = 0;
            break 'l1;
        }
    }
}
pub(crate) unsafe extern "C" fn CB2_InGameTrade() {
    DoTradeAnim();
    RunTasks();
    RunTextPrinters();
    AnimateSprites();
    BuildOamBuffer();
    UpdatePaletteFade();
}
pub(crate) unsafe extern "C" fn SetTradeSequenceBgGpuRegs(state: u8) {
    match state {
        0 => {
            (*sTradeAnim).bg2vofs = 0;
            (*sTradeAnim).bg2hofs = 180;
            SetGpuReg(0x0, 5440);
            SetGpuReg(REG_OFFSET_BG2CNT, 20998);
            LoadPalette(gTradeGba2_Pal.as_ptr().cast_mut() as *mut c_void, 16, 96);
            {
                let mut _src: *mut c_void = gTradeGba_Gfx.as_ptr().cast_mut() as *mut c_void;
                let mut _dest: *mut c_void = 0x6004000 as usize as *mut c_void;
                let mut _size: u32 = 0x1420;
                loop {
                    if _size <= 0x1000 {
                        {
                            {
                                {
                                    let mut dmaRegs: *mut u32 = 67109076 as usize as *mut u32;
                                    volatile_write(dmaRegs, _src as usize as u32);
                                    volatile_write(dmaRegs.at(1), _dest as usize as u32);
                                    volatile_write(dmaRegs.at(2), 0x80000000 | _size / 2);
                                    let _ = (dmaRegs.at(2)).read_volatile();
                                }
                            }
                        }
                        break;
                    }
                    {
                        {
                            {
                                let mut dmaRegs: *mut u32 = 67109076 as usize as *mut u32;
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
                let mut _dest: *mut c_void = 0x6009000 as usize as *mut c_void;
                let mut _size: u32 = 0x1000;
                {
                    {
                        {
                            let mut dmaRegs: *mut u32 = 67109076 as usize as *mut u32;
                            volatile_write(dmaRegs, _src as usize as u32);
                            volatile_write(dmaRegs.at(1), _dest as usize as u32);
                            volatile_write(dmaRegs.at(2), 0x80000000 | _size / 2);
                            let _ = (dmaRegs.at(2)).read_volatile();
                        }
                    }
                }
            }
        }
        1 => {
            (*sTradeAnim).bg1hofs = 0;
            (*sTradeAnim).bg1vofs = 348;
            SetGpuReg(REG_OFFSET_BG1VOFS, 348);
            SetGpuReg(REG_OFFSET_BG1CNT, 34050);
            SetGpuReg(REG_OFFSET_BG2CNT, 37382);
            if (*sTradeAnim).isCableTrade != 0 {
                {
                    let mut _src: *mut c_void = sGbaMapCable.as_ptr().cast_mut() as *mut c_void;
                    let mut _dest: *mut c_void = 0x6002800 as usize as *mut c_void;
                    let mut _size: u32 = 0x1000;
                    {
                        {
                            {
                                let mut dmaRegs: *mut u32 = 67109076 as usize as *mut u32;
                                volatile_write(dmaRegs, _src as usize as u32);
                                volatile_write(dmaRegs.at(1), _dest as usize as u32);
                                volatile_write(dmaRegs.at(2), 0x80000000 | _size / 2);
                                let _ = (dmaRegs.at(2)).read_volatile();
                            }
                        }
                    }
                }
            } else {
                {
                    let mut _src: *mut c_void = sGbaMapWireless.as_ptr().cast_mut() as *mut c_void;
                    let mut _dest: *mut c_void = 0x6002800 as usize as *mut c_void;
                    let mut _size: u32 = 0x1000;
                    {
                        {
                            {
                                let mut dmaRegs: *mut u32 = 67109076 as usize as *mut u32;
                                volatile_write(dmaRegs, _src as usize as u32);
                                volatile_write(dmaRegs.at(1), _dest as usize as u32);
                                volatile_write(dmaRegs.at(2), 0x80000000 | _size / 2);
                                let _ = (dmaRegs.at(2)).read_volatile();
                            }
                        }
                    }
                }
            }
            {
                let mut _src: *mut c_void = gTradeGba_Gfx.as_ptr().cast_mut() as *mut c_void;
                let mut _dest: *mut c_void = 0x6000000 as usize as *mut c_void;
                let mut _size: u32 = 0x1420;
                loop {
                    if _size <= 0x1000 {
                        {
                            {
                                {
                                    let mut dmaRegs: *mut u32 = 67109076 as usize as *mut u32;
                                    volatile_write(dmaRegs, _src as usize as u32);
                                    volatile_write(dmaRegs.at(1), _dest as usize as u32);
                                    volatile_write(dmaRegs.at(2), 0x80000000 | _size / 2);
                                    let _ = (dmaRegs.at(2)).read_volatile();
                                }
                            }
                        }
                        break;
                    }
                    {
                        {
                            {
                                let mut dmaRegs: *mut u32 = 67109076 as usize as *mut u32;
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
            (*sTradeAnim).bg1vofs = 0;
            (*sTradeAnim).bg1hofs = 0;
            if (*sTradeAnim).isCableTrade == 0 {
                SetGpuReg(REG_OFFSET_DISPCNT, 4673);
                LZ77UnCompVram(
                    sWirelessCloseup_Map.as_ptr().cast_mut(),
                    0x6002800 as usize as *mut c_void,
                );
                BlendPalettes(0x8, 16, 0);
            } else {
                SetGpuReg(REG_OFFSET_DISPCNT, 4673);
                {
                    let mut _src: *mut c_void =
                        sCableCloseup_Map.as_ptr().cast_mut() as *mut c_void;
                    let mut _dest: *mut c_void = 0x6002800 as usize as *mut c_void;
                    let mut _size: u32 = 0x800;
                    {
                        {
                            {
                                let mut dmaRegs: *mut u32 = 67109076 as usize as *mut u32;
                                volatile_write(dmaRegs, _src as usize as u32);
                                volatile_write(dmaRegs.at(1), _dest as usize as u32);
                                volatile_write(dmaRegs.at(2), 0x80000000 | _size / 2);
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
                0x6004000 as usize as *mut c_void,
            );
            LZ77UnCompVram(
                sWirelessSignal_Tilemap.as_ptr().cast_mut(),
                0x6009000 as usize as *mut c_void,
            );
            (*sTradeAnim).bg2vofs = 80;
            SetGpuReg(0x0, 5696);
        }
        4 => {
            SetGpuReg(REG_OFFSET_DISPCNT, 5185);
            SetGpuReg(REG_OFFSET_BG2CNT, 4743);
            (*sTradeAnim).texX = 64;
            (*sTradeAnim).texY = 92;
            (*sTradeAnim).sXY = 32;
            (*sTradeAnim).gbaScale = 1024;
            (*sTradeAnim).alpha = 0;
            {
                let mut _src: *mut c_void = sGbaAffine_Gfx.as_ptr().cast_mut() as *mut c_void;
                let mut _dest: *mut c_void = 0x6004000 as usize as *mut c_void;
                let mut _size: u32 = 0x2840;
                loop {
                    if _size <= 0x1000 {
                        {
                            {
                                {
                                    let mut dmaRegs: *mut u32 = 67109076 as usize as *mut u32;
                                    volatile_write(dmaRegs, _src as usize as u32);
                                    volatile_write(dmaRegs.at(1), _dest as usize as u32);
                                    volatile_write(dmaRegs.at(2), 0x80000000 | _size / 2);
                                    let _ = (dmaRegs.at(2)).read_volatile();
                                }
                            }
                        }
                        break;
                    }
                    {
                        {
                            {
                                let mut dmaRegs: *mut u32 = 67109076 as usize as *mut u32;
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
            if (*sTradeAnim).isCableTrade != 0 {
                {
                    let mut _src: *mut c_void =
                        sGbaAffineMapCable.as_ptr().cast_mut() as *mut c_void;
                    let mut _dest: *mut c_void = 0x6009000 as usize as *mut c_void;
                    let mut _size: u32 = 0x100;
                    {
                        {
                            {
                                let mut dmaRegs: *mut u32 = 67109076 as usize as *mut u32;
                                volatile_write(dmaRegs, _src as usize as u32);
                                volatile_write(dmaRegs.at(1), _dest as usize as u32);
                                volatile_write(dmaRegs.at(2), 0x80000000 | _size / 2);
                                let _ = (dmaRegs.at(2)).read_volatile();
                            }
                        }
                    }
                }
            } else {
                {
                    let mut _src: *mut c_void =
                        sGbaAffineMapWireless.as_ptr().cast_mut() as *mut c_void;
                    let mut _dest: *mut c_void = 0x6009000 as usize as *mut c_void;
                    let mut _size: u32 = 0x100;
                    {
                        {
                            {
                                let mut dmaRegs: *mut u32 = 67109076 as usize as *mut u32;
                                volatile_write(dmaRegs, _src as usize as u32);
                                volatile_write(dmaRegs.at(1), _dest as usize as u32);
                                volatile_write(dmaRegs.at(2), 0x80000000 | _size / 2);
                                let _ = (dmaRegs.at(2)).read_volatile();
                            }
                        }
                    }
                }
            }
        }
        5 => {
            (*sTradeAnim).bg1vofs = 0;
            (*sTradeAnim).bg1hofs = 0;
        }
        6 => {
            SetGpuReg(REG_OFFSET_DISPCNT, 5185);
            SetGpuReg(REG_OFFSET_BG2CNT, 4743);
            (*sTradeAnim).texX = 64;
            (*sTradeAnim).texY = 92;
            (*sTradeAnim).sXY = 256;
            (*sTradeAnim).gbaScale = 128;
            (*sTradeAnim).scrX = 120;
            (*sTradeAnim).scrY = 80;
            (*sTradeAnim).alpha = 0;
            {
                let mut _src: *mut c_void = sGbaAffine_Gfx.as_ptr().cast_mut() as *mut c_void;
                let mut _dest: *mut c_void = 0x6004000 as usize as *mut c_void;
                let mut _size: u32 = 0x2840;
                loop {
                    if _size <= 0x1000 {
                        {
                            {
                                {
                                    let mut dmaRegs: *mut u32 = 67109076 as usize as *mut u32;
                                    volatile_write(dmaRegs, _src as usize as u32);
                                    volatile_write(dmaRegs.at(1), _dest as usize as u32);
                                    volatile_write(dmaRegs.at(2), 0x80000000 | _size / 2);
                                    let _ = (dmaRegs.at(2)).read_volatile();
                                }
                            }
                        }
                        break;
                    }
                    {
                        {
                            {
                                let mut dmaRegs: *mut u32 = 67109076 as usize as *mut u32;
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
            if (*sTradeAnim).isCableTrade != 0 {
                {
                    let mut _src: *mut c_void =
                        sGbaAffineMapCable.as_ptr().cast_mut() as *mut c_void;
                    let mut _dest: *mut c_void = 0x6009000 as usize as *mut c_void;
                    let mut _size: u32 = 0x100;
                    {
                        {
                            {
                                let mut dmaRegs: *mut u32 = 67109076 as usize as *mut u32;
                                volatile_write(dmaRegs, _src as usize as u32);
                                volatile_write(dmaRegs.at(1), _dest as usize as u32);
                                volatile_write(dmaRegs.at(2), 0x80000000 | _size / 2);
                                let _ = (dmaRegs.at(2)).read_volatile();
                            }
                        }
                    }
                }
            } else {
                {
                    let mut _src: *mut c_void =
                        sGbaAffineMapWireless.as_ptr().cast_mut() as *mut c_void;
                    let mut _dest: *mut c_void = 0x6009000 as usize as *mut c_void;
                    let mut _size: u32 = 0x100;
                    {
                        {
                            {
                                let mut dmaRegs: *mut u32 = 67109076 as usize as *mut u32;
                                volatile_write(dmaRegs, _src as usize as u32);
                                volatile_write(dmaRegs.at(1), _dest as usize as u32);
                                volatile_write(dmaRegs.at(2), 0x80000000 | _size / 2);
                                let _ = (dmaRegs.at(2)).read_volatile();
                            }
                        }
                    }
                }
            }
        }
        7 => {
            (*sTradeAnim).bg2vofs = 0;
            (*sTradeAnim).bg2hofs = 0;
            SetGpuReg(REG_OFFSET_BLDCNT, 0);
            SetGpuReg(REG_OFFSET_BG2CNT, 20998);
            LoadPalette(gTradeGba2_Pal.as_ptr().cast_mut() as *mut c_void, 16, 96);
            {
                let mut _src: *mut c_void = gTradeGba_Gfx.as_ptr().cast_mut() as *mut c_void;
                let mut _dest: *mut c_void = 0x6004000 as usize as *mut c_void;
                let mut _size: u32 = 0x1420;
                loop {
                    if _size <= 0x1000 {
                        {
                            {
                                {
                                    let mut dmaRegs: *mut u32 = 67109076 as usize as *mut u32;
                                    volatile_write(dmaRegs, _src as usize as u32);
                                    volatile_write(dmaRegs.at(1), _dest as usize as u32);
                                    volatile_write(dmaRegs.at(2), 0x80000000 | _size / 2);
                                    let _ = (dmaRegs.at(2)).read_volatile();
                                }
                            }
                        }
                        break;
                    }
                    {
                        {
                            {
                                let mut dmaRegs: *mut u32 = 67109076 as usize as *mut u32;
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
                let mut _dest: *mut c_void = 0x6009000 as usize as *mut c_void;
                let mut _size: u32 = 0x1000;
                {
                    {
                        {
                            let mut dmaRegs: *mut u32 = 67109076 as usize as *mut u32;
                            volatile_write(dmaRegs, _src as usize as u32);
                            volatile_write(dmaRegs.at(1), _dest as usize as u32);
                            volatile_write(dmaRegs.at(2), 0x80000000 | _size / 2);
                            let _ = (dmaRegs.at(2)).read_volatile();
                        }
                    }
                }
            }
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn LoadTradeSequenceSpriteSheetsAndPalettes() {
    LoadSpriteSheet((&raw const *sSpriteSheet_LinkMonGlow).cast_mut());
    LoadSpriteSheet((&raw const *sSpriteSheet_LinkMonShadow).cast_mut());
    LoadSpriteSheet((&raw const *sSpriteSheet_CableEnd).cast_mut());
    LoadSpriteSheet((&raw const *sSpriteSheet_GbaScreen).cast_mut());
    LoadSpritePalette((&raw const *sSpritePalette_LinkMon).cast_mut());
    LoadSpritePalette((&raw const *sSpritePalette_Gba).cast_mut());
}
pub(crate) unsafe extern "C" fn BufferTradeSceneStrings() {
    let mut mpId: u8 = 0;
    let mut name: CArray<u8, 20> = zeroed();
    let mut ingameTrade: *mut InGameTrade = null_mut();
    if (*sTradeAnim).isLinkTrade != 0 {
        mpId = GetMultiplayerId();
        StringCopy(
            gStringVar1.as_mut_ptr(),
            gLinkPlayers[mpId as i32 ^ 1].name.as_mut_ptr(),
        );
        GetMonData3(
            &raw mut gEnemyParty[gSelectedTradeMonPositions[1] as i32 % 6],
            MON_DATA_NICKNAME,
            name.as_mut_ptr(),
        );
        StringCopy_Nickname(gStringVar3.as_mut_ptr(), name.as_mut_ptr());
        GetMonData3(
            &raw mut gPlayerParty[gSelectedTradeMonPositions[0]],
            MON_DATA_NICKNAME,
            name.as_mut_ptr(),
        );
        StringCopy_Nickname(gStringVar2.as_mut_ptr(), name.as_mut_ptr());
    } else {
        ingameTrade = (&raw const sIngameTrades[gSpecialVar_0x8004]).cast_mut();
        StringCopy(gStringVar1.as_mut_ptr(), (*ingameTrade).otName.as_mut_ptr());
        StringCopy_Nickname(
            gStringVar3.as_mut_ptr(),
            (*ingameTrade).nickname.as_mut_ptr(),
        );
        GetMonData3(
            &raw mut gPlayerParty[gSpecialVar_0x8005],
            MON_DATA_NICKNAME,
            name.as_mut_ptr(),
        );
        StringCopy_Nickname(gStringVar2.as_mut_ptr(), name.as_mut_ptr());
    }
}
pub(crate) unsafe extern "C" fn DoTradeAnim() -> u8 {
    if (*sTradeAnim).isCableTrade != 0 {
        return DoTradeAnim_Cable();
    } else {
        return DoTradeAnim_Wireless();
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn DoTradeAnim_Cable() -> u8 {
    let mut evoTarget: u16 = 0;
    match (*sTradeAnim).state {
        STATE_START => {
            gSprites[(*sTradeAnim).monSpriteIds[0]].set_invisible(0);
            gSprites[(*sTradeAnim).monSpriteIds[0]].x2 = -180;
            gSprites[(*sTradeAnim).monSpriteIds[0]].y2 =
                gMonFrontPicCoords[(*sTradeAnim).monSpecies[0]].y_offset as i16;
            (*sTradeAnim).state += 1;
            (*sTradeAnim).cachedMapMusic = GetCurrentMapMusic();
            PlayNewMapMusic(MUS_EVOLUTION);
        }
        STATE_MON_SLIDE_IN => {
            if (*sTradeAnim).bg2hofs > 0 {
                gSprites[(*sTradeAnim).monSpriteIds[0]].x2 += 3;
                (*sTradeAnim).bg2hofs -= 3;
            } else {
                gSprites[(*sTradeAnim).monSpriteIds[0]].x2 = 0;
                (*sTradeAnim).bg2hofs = 0;
                (*sTradeAnim).state = STATE_SEND_MSG;
            }
        }
        STATE_SEND_MSG => {
            StringExpandPlaceholders(
                gStringVar4.as_mut_ptr(),
                gText_XWillBeSentToY.as_ptr().cast_mut(),
            );
            DrawTextOnTradeWindow(0, gStringVar4.as_mut_ptr(), 0);
            if (*sTradeAnim).monSpecies[0] != SPECIES_EGG as u16 {
                PlayCry_Normal((*sTradeAnim).monSpecies[0], 0);
            }
            (*sTradeAnim).state = STATE_BYE_BYE;
            (*sTradeAnim).timer = 0;
        }
        STATE_BYE_BYE => {
            if ({
                (*sTradeAnim).timer += 1;
                (*sTradeAnim).timer
            }) == 80
            {
                (*sTradeAnim).releasePokeballSpriteId = CreateTradePokeballSprite(
                    (*sTradeAnim).monSpriteIds[0],
                    gSprites[(*sTradeAnim).monSpriteIds[0]].oam.paletteNum() as u8,
                    120,
                    32,
                    2,
                    1,
                    0x14,
                    0xfffff,
                );
                (*sTradeAnim).state += 1;
                StringExpandPlaceholders(
                    gStringVar4.as_mut_ptr(),
                    gText_ByeByeVar1.as_ptr().cast_mut(),
                );
                DrawTextOnTradeWindow(0, gStringVar4.as_mut_ptr(), 0);
            }
        }
        STATE_POKEBALL_DEPART => {
            if gSprites[(*sTradeAnim).releasePokeballSpriteId].callback
                == Some(SpriteCallbackDummy as unsafe extern "C" fn(*mut Sprite))
            {
                (*sTradeAnim).bouncingPokeballSpriteId = CreateSprite(
                    (&raw const *sSpriteTemplate_Pokeball).cast_mut(),
                    120,
                    32,
                    0,
                );
                gSprites[(*sTradeAnim).bouncingPokeballSpriteId].callback =
                    Some(SpriteCB_BouncingPokeballDepart);
                DestroySprite(&raw mut gSprites[(*sTradeAnim).releasePokeballSpriteId]);
                (*sTradeAnim).state += 1;
            }
        }
        STATE_POKEBALL_DEPART_WAIT => {}
        STATE_FADE_OUT_TO_GBA_SEND => {
            BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 16, 0);
            (*sTradeAnim).state = STATE_WAIT_FADE_OUT_TO_GBA_SEND;
        }
        STATE_WAIT_FADE_OUT_TO_GBA_SEND => {
            if gPaletteFade.active() == 0 {
                SetTradeSequenceBgGpuRegs(4);
                FillWindowPixelBuffer(0, 255);
                CopyWindowToVram(0, COPYWIN_FULL);
                (*sTradeAnim).state += 1;
            }
        }
        STATE_FADE_IN_TO_GBA_SEND => {
            BeginNormalPaletteFade(PALETTES_ALL, -1, 16, 0, 0);
            (*sTradeAnim).state += 1;
        }
        STATE_WAIT_FADE_IN_TO_GBA_SEND => {
            if gPaletteFade.active() == 0 {
                (*sTradeAnim).state = STATE_GBA_ZOOM_OUT;
            }
        }
        STATE_GBA_ZOOM_OUT => {
            if (*sTradeAnim).gbaScale > 0x100 {
                (*sTradeAnim).gbaScale -= 0x34;
            } else {
                SetTradeSequenceBgGpuRegs(1);
                (*sTradeAnim).gbaScale = 0x80;
                (*sTradeAnim).state += 1;
                (*sTradeAnim).timer = 0;
            }
            (*sTradeAnim).sXY = div_i32(0x8000, (*sTradeAnim).gbaScale as i32) as u16;
        }
        STATE_GBA_FLASH_SEND => {
            if ({
                (*sTradeAnim).timer += 1;
                (*sTradeAnim).timer
            }) > 20
            {
                SetTradeBGAffine();
                (*sTradeAnim).connectionSpriteId2 = CreateSprite(
                    (&raw const *sSpriteTemplate_GbaScreenFlash_Long).cast_mut(),
                    120,
                    80,
                    0,
                );
                (*sTradeAnim).state += 1;
            }
        }
        STATE_GBA_STOP_FLASH_SEND => {
            if gSprites[(*sTradeAnim).connectionSpriteId2].animEnded() != 0 {
                DestroySprite(&raw mut gSprites[(*sTradeAnim).connectionSpriteId2]);
                SetGpuReg(REG_OFFSET_BLDCNT, 1600);
                SetGpuReg(REG_OFFSET_BLDALPHA, 1036);
                (*sTradeAnim).state += 1;
            }
        }
        STATE_PAN_AWAY_GBA => {
            if ({
                (*sTradeAnim).bg1vofs -= 1;
                (*sTradeAnim).bg1vofs
            }) == 316
            {
                (*sTradeAnim).state += 1;
            }
            if (*sTradeAnim).bg1vofs == 328 {
                (*sTradeAnim).cableEndSpriteId = CreateSprite(
                    (&raw const *sSpriteTemplate_CableEnd).cast_mut(),
                    128,
                    65,
                    0,
                );
            }
        }
        STATE_CREATE_LINK_MON_LEAVING => {
            (*sTradeAnim).connectionSpriteId1 = CreateSprite(
                (&raw const *sSpriteTemplate_LinkMonGlow).cast_mut(),
                128,
                80,
                3,
            );
            (*sTradeAnim).connectionSpriteId2 = CreateSprite(
                (&raw const *sSpriteTemplate_LinkMonShadow).cast_mut(),
                128,
                80,
                0,
            );
            StartSpriteAnim(
                &raw mut gSprites[(*sTradeAnim).connectionSpriteId2],
                ANIM_LINKMON_SMALL,
            );
            (*sTradeAnim).state += 1;
        }
        STATE_LINK_MON_TRAVEL_OUT => {
            if ({
                (*sTradeAnim).bg1vofs -= 2;
                (*sTradeAnim).bg1vofs
            }) == 166
            {
                (*sTradeAnim).state = STATE_LINK_MON_TRAVEL_OFFSCREEN;
            }
            SetGpuReg(REG_OFFSET_DISPCNT, 4673);
        }
        STATE_LINK_MON_TRAVEL_OFFSCREEN => {
            gSprites[(*sTradeAnim).connectionSpriteId1].y -= 2;
            gSprites[(*sTradeAnim).connectionSpriteId2].y -= 2;
            if gSprites[(*sTradeAnim).connectionSpriteId1].y < -8 {
                (*sTradeAnim).state = STATE_FADE_OUT_TO_CROSSING;
            }
        }
        STATE_FADE_OUT_TO_CROSSING => {
            BeginNormalPaletteFade(PALETTES_ALL, -1, 0, 16, 0);
            (*sTradeAnim).state = STATE_WAIT_FADE_OUT_TO_CROSSING;
        }
        STATE_WAIT_FADE_OUT_TO_CROSSING => {
            if gPaletteFade.active() == 0 {
                DestroySprite(&raw mut gSprites[(*sTradeAnim).connectionSpriteId1]);
                DestroySprite(&raw mut gSprites[(*sTradeAnim).connectionSpriteId2]);
                SetTradeSequenceBgGpuRegs(2);
                (*sTradeAnim).state += 1;
            }
        }
        STATE_FADE_IN_TO_CROSSING => {
            BeginNormalPaletteFade(PALETTES_ALL, -1, 16, 0, 0);
            (*sTradeAnim).connectionSpriteId1 = CreateSprite(
                (&raw const *sSpriteTemplate_LinkMonShadow).cast_mut(),
                111,
                170,
                0,
            );
            (*sTradeAnim).connectionSpriteId2 = CreateSprite(
                (&raw const *sSpriteTemplate_LinkMonShadow).cast_mut(),
                129,
                -10,
                0,
            );
            (*sTradeAnim).state += 1;
        }
        STATE_WAIT_FADE_IN_TO_CROSSING => {
            if gPaletteFade.active() == 0 {
                PlaySE(SE_WARP_OUT);
                (*sTradeAnim).state += 1;
            }
            gSprites[(*sTradeAnim).connectionSpriteId1].y2 -= 3;
            gSprites[(*sTradeAnim).connectionSpriteId2].y2 += 3;
        }
        STATE_CROSSING_LINK_MONS_ENTER => {
            gSprites[(*sTradeAnim).connectionSpriteId1].y2 -= 3;
            gSprites[(*sTradeAnim).connectionSpriteId2].y2 += 3;
            if gSprites[(*sTradeAnim).connectionSpriteId1].y2 <= -90 {
                gSprites[(*sTradeAnim).connectionSpriteId1].data[1] = 1;
                gSprites[(*sTradeAnim).connectionSpriteId2].data[1] = 1;
                (*sTradeAnim).state += 1;
            }
        }
        STATE_CROSSING_BLEND_WHITE_1 => {
            BlendPalettes(0x1, 16, 65535);
            (*sTradeAnim).state += 1;
        }
        STATE_CROSSING_BLEND_WHITE_2 => {
            BlendPalettes(0x1, 0, 65535);
            (*sTradeAnim).state += 1;
        }
        STATE_CROSSING_BLEND_WHITE_3 => {
            BlendPalettes(0x1, 16, 65535);
            (*sTradeAnim).state += 1;
        }
        STATE_CROSSING_CREATE_MON_PICS => {
            if IsMonSpriteNotFlipped((*sTradeAnim).monSpecies[0]) == 0 {
                gSprites[(*sTradeAnim).monSpriteIds[0]].affineAnims =
                    sAffineAnims_CrossingMonPics.as_ptr().cast_mut();
                gSprites[(*sTradeAnim).monSpriteIds[0]]
                    .oam
                    .set_affineMode(ST_OAM_AFFINE_DOUBLE);
                CalcCenterToCornerVec(
                    &raw mut gSprites[(*sTradeAnim).monSpriteIds[0]],
                    TRADE_PLAYER,
                    ST_OAM_AFFINE_DOUBLE as u8,
                    ST_OAM_AFFINE_DOUBLE as u8,
                );
                StartSpriteAffineAnim(&raw mut gSprites[(*sTradeAnim).monSpriteIds[0]], 0);
            } else {
                StartSpriteAffineAnim(&raw mut gSprites[(*sTradeAnim).monSpriteIds[0]], 0);
            }
            StartSpriteAffineAnim(&raw mut gSprites[(*sTradeAnim).monSpriteIds[1]], 0);
            gSprites[(*sTradeAnim).monSpriteIds[0]].x = 60;
            gSprites[(*sTradeAnim).monSpriteIds[1]].x = 180;
            gSprites[(*sTradeAnim).monSpriteIds[0]].y = 192;
            gSprites[(*sTradeAnim).monSpriteIds[1]].y = -32;
            gSprites[(*sTradeAnim).monSpriteIds[0]].set_invisible(0);
            gSprites[(*sTradeAnim).monSpriteIds[1]].set_invisible(FALSE as u16);
            (*sTradeAnim).state += 1;
        }
        STATE_CROSSING_MON_PICS_MOVE => {
            gSprites[(*sTradeAnim).monSpriteIds[0]].y2 -= 3;
            gSprites[(*sTradeAnim).monSpriteIds[1]].y2 += 3;
            if gSprites[(*sTradeAnim).monSpriteIds[0]].y2 < -160
                && gSprites[(*sTradeAnim).monSpriteIds[0]].y2 >= -163
            {
                PlaySE(SE_WARP_IN);
            }
            if gSprites[(*sTradeAnim).monSpriteIds[0]].y2 < -222 {
                gSprites[(*sTradeAnim).connectionSpriteId1].data[1] = 0;
                gSprites[(*sTradeAnim).connectionSpriteId2].data[1] = 0;
                (*sTradeAnim).state += 1;
                gSprites[(*sTradeAnim).monSpriteIds[0]].set_invisible(TRUE as u16);
                gSprites[(*sTradeAnim).monSpriteIds[1]].set_invisible(1);
                BlendPalettes(0x1, 0, 65535);
            }
        }
        STATE_CROSSING_LINK_MONS_EXIT => {
            gSprites[(*sTradeAnim).connectionSpriteId1].y2 -= 3;
            gSprites[(*sTradeAnim).connectionSpriteId2].y2 += 3;
            if gSprites[(*sTradeAnim).connectionSpriteId1].y2 <= -222 {
                BeginNormalPaletteFade(PALETTES_ALL, -1, 0, 16, 0);
                (*sTradeAnim).state += 1;
                DestroySprite(&raw mut gSprites[(*sTradeAnim).connectionSpriteId1]);
                DestroySprite(&raw mut gSprites[(*sTradeAnim).connectionSpriteId2]);
            }
        }
        STATE_CREATE_LINK_MON_ARRIVING => {
            if gPaletteFade.active() == 0 {
                (*sTradeAnim).state += 1;
                SetTradeSequenceBgGpuRegs(1);
                (*sTradeAnim).bg1vofs = 166;
                (*sTradeAnim).connectionSpriteId1 = CreateSprite(
                    (&raw const *sSpriteTemplate_LinkMonGlow).cast_mut(),
                    128,
                    -20,
                    3,
                );
                (*sTradeAnim).connectionSpriteId2 = CreateSprite(
                    (&raw const *sSpriteTemplate_LinkMonShadow).cast_mut(),
                    128,
                    -20,
                    0,
                );
                StartSpriteAnim(
                    &raw mut gSprites[(*sTradeAnim).connectionSpriteId2],
                    ANIM_LINKMON_SMALL,
                );
            }
        }
        STATE_FADE_OUT_TO_GBA_RECV => {
            BeginNormalPaletteFade(PALETTES_ALL, -1, 16, 0, 0);
            (*sTradeAnim).state += 1;
        }
        STATE_WAIT_FADE_OUT_TO_GBA_RECV => {
            SetGpuReg(0x0, 4672);
            if gPaletteFade.active() == 0 {
                (*sTradeAnim).state += 1;
            }
        }
        STATE_LINK_MON_TRAVEL_IN => {
            gSprites[(*sTradeAnim).connectionSpriteId1].y2 += 3;
            gSprites[(*sTradeAnim).connectionSpriteId2].y2 += 3;
            if gSprites[(*sTradeAnim).connectionSpriteId1].y2 as i32
                + gSprites[(*sTradeAnim).connectionSpriteId1].y as i32
                == 64
            {
                (*sTradeAnim).state += 1;
            }
        }
        STATE_PAN_TO_GBA => {
            if ({
                (*sTradeAnim).bg1vofs += 2;
                (*sTradeAnim).bg1vofs
            }) > 316
            {
                (*sTradeAnim).bg1vofs = 316;
                (*sTradeAnim).state += 1;
            }
        }
        STATE_DESTROY_LINK_MON => {
            DestroySprite(&raw mut gSprites[(*sTradeAnim).connectionSpriteId1]);
            DestroySprite(&raw mut gSprites[(*sTradeAnim).connectionSpriteId2]);
            (*sTradeAnim).state += 1;
            (*sTradeAnim).timer = 0;
        }
        STATE_LINK_MON_ARRIVED_DELAY => {
            if ({
                (*sTradeAnim).timer += 1;
                (*sTradeAnim).timer
            }) == 10
            {
                (*sTradeAnim).state += 1;
            }
        }
        STATE_MOVE_GBA_TO_CENTER => {
            if ({
                (*sTradeAnim).bg1vofs += 1;
                (*sTradeAnim).bg1vofs
            }) > 348
            {
                (*sTradeAnim).bg1vofs = 348;
                (*sTradeAnim).state += 1;
            }
            if (*sTradeAnim).bg1vofs == 328 && (*sTradeAnim).isCableTrade != 0 {
                (*sTradeAnim).cableEndSpriteId = CreateSprite(
                    (&raw const *sSpriteTemplate_CableEnd).cast_mut(),
                    128,
                    65,
                    0,
                );
                gSprites[(*sTradeAnim).cableEndSpriteId].callback =
                    Some(SpriteCB_CableEndReceiving);
            }
        }
        STATE_GBA_FLASH_RECV => {
            (*sTradeAnim).connectionSpriteId2 = CreateSprite(
                (&raw const *sSpriteTemplate_GbaScreenFlash_Long).cast_mut(),
                120,
                80,
                0,
            );
            (*sTradeAnim).state = STATE_GBA_STOP_FLASH_RECV;
        }
        STATE_GBA_STOP_FLASH_RECV => {
            if gSprites[(*sTradeAnim).connectionSpriteId2].animEnded() != 0 {
                DestroySprite(&raw mut gSprites[(*sTradeAnim).connectionSpriteId2]);
                SetTradeSequenceBgGpuRegs(6);
                (*sTradeAnim).state += 1;
                PlaySE(SE_M_SAND_ATTACK);
            }
        }
        STATE_GBA_ZOOM_IN => {
            if (*sTradeAnim).gbaScale < 0x400 {
                (*sTradeAnim).gbaScale += 0x34;
            } else {
                (*sTradeAnim).gbaScale = 0x400;
                (*sTradeAnim).state += 1;
            }
            (*sTradeAnim).sXY = div_i32(0x8000, (*sTradeAnim).gbaScale as i32) as u16;
        }
        STATE_FADE_OUT_TO_NEW_MON => {
            BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 16, 0);
            (*sTradeAnim).state = STATE_WAIT_FADE_OUT_TO_NEW_MON;
        }
        STATE_WAIT_FADE_OUT_TO_NEW_MON => {
            if gPaletteFade.active() == 0 {
                SetTradeSequenceBgGpuRegs(5);
                SetTradeSequenceBgGpuRegs(7);
                gPaletteFade.set_bufferTransferDisabled(TRUE as u16);
                (*sTradeAnim).state += 1;
            }
        }
        STATE_FADE_IN_TO_NEW_MON => {
            gPaletteFade.set_bufferTransferDisabled(FALSE as u16);
            BeginNormalPaletteFade(PALETTES_ALL, 0, 16, 0, 0);
            (*sTradeAnim).state += 1;
        }
        STATE_WAIT_FADE_IN_TO_NEW_MON => {
            SetGpuReg(0x0, 5184);
            if gPaletteFade.active() == 0 {
                (*sTradeAnim).state += 1;
            }
        }
        STATE_POKEBALL_ARRIVE => {
            (*sTradeAnim).bouncingPokeballSpriteId = CreateSprite(
                (&raw const *sSpriteTemplate_Pokeball).cast_mut(),
                120,
                -8,
                0,
            );
            gSprites[(*sTradeAnim).bouncingPokeballSpriteId].data[3] = 74;
            gSprites[(*sTradeAnim).bouncingPokeballSpriteId].callback =
                Some(SpriteCB_BouncingPokeballArrive);
            StartSpriteAnim(&raw mut gSprites[(*sTradeAnim).bouncingPokeballSpriteId], 1);
            StartSpriteAffineAnim(&raw mut gSprites[(*sTradeAnim).bouncingPokeballSpriteId], 2);
            BlendPalettes(
                shl_i32(
                    1,
                    16 + gSprites[(*sTradeAnim).bouncingPokeballSpriteId]
                        .oam
                        .paletteNum() as u32,
                ) as u32,
                16,
                65535,
            );
            (*sTradeAnim).state += 1;
            (*sTradeAnim).timer = 0;
        }
        STATE_FADE_POKEBALL_TO_NORMAL => {
            BeginNormalPaletteFade(
                shl_i32(
                    1,
                    16 + gSprites[(*sTradeAnim).bouncingPokeballSpriteId]
                        .oam
                        .paletteNum() as u32,
                ) as u32,
                1,
                16,
                0,
                65535,
            );
            (*sTradeAnim).state += 1;
        }
        STATE_POKEBALL_ARRIVE_WAIT => {
            if gSprites[(*sTradeAnim).bouncingPokeballSpriteId].callback
                == Some(SpriteCallbackDummy as unsafe extern "C" fn(*mut Sprite))
            {
                HandleLoadSpecialPokePic_2(
                    (&raw const gMonFrontPicTable[(*sTradeAnim).monSpecies[1]]).cast_mut(),
                    (*gMonSpritesGfxPtr).sprites.ptr[3],
                    (*sTradeAnim).monSpecies[1] as i32,
                    (*sTradeAnim).monPersonalities[1],
                );
                (*sTradeAnim).state += 1;
            }
        }
        STATE_SHOW_NEW_MON => {
            gSprites[(*sTradeAnim).monSpriteIds[1]].x = 120;
            gSprites[(*sTradeAnim).monSpriteIds[1]].y =
                gMonFrontPicCoords[(*sTradeAnim).monSpecies[1]].y_offset as i16 + 60;
            gSprites[(*sTradeAnim).monSpriteIds[1]].x2 = 0;
            gSprites[(*sTradeAnim).monSpriteIds[1]].y2 = 0;
            StartSpriteAnim(&raw mut gSprites[(*sTradeAnim).monSpriteIds[1]], 0);
            CreatePokeballSpriteToReleaseMon(
                (*sTradeAnim).monSpriteIds[1],
                gSprites[(*sTradeAnim).monSpriteIds[1]].oam.paletteNum() as u8,
                120,
                84,
                2,
                1,
                20,
                0xfffff,
                (*sTradeAnim).monSpecies[1],
            );
            FreeSpriteOamMatrix(&raw mut gSprites[(*sTradeAnim).bouncingPokeballSpriteId]);
            DestroySprite(&raw mut gSprites[(*sTradeAnim).bouncingPokeballSpriteId]);
            (*sTradeAnim).state += 1;
        }
        STATE_NEW_MON_MSG => {
            SetGpuReg(0x0, 5440);
            StringExpandPlaceholders(
                gStringVar4.as_mut_ptr(),
                gText_XSentOverY.as_ptr().cast_mut(),
            );
            DrawTextOnTradeWindow(0, gStringVar4.as_mut_ptr(), 0);
            (*sTradeAnim).state = STATE_DELAY_FOR_MON_ANIM;
            (*sTradeAnim).timer = 0;
        }
        STATE_DELAY_FOR_MON_ANIM => {
            if ({
                (*sTradeAnim).timer += 1;
                (*sTradeAnim).timer
            }) > 60
            {
                (*sTradeAnim).state = STATE_WAIT_FOR_MON_CRY;
                (*sTradeAnim).timer = 0;
            }
        }
        STATE_WAIT_FOR_MON_CRY => {
            if IsCryFinished() != 0 {
                (*sTradeAnim).state = STATE_TAKE_CARE_OF_MON;
            }
        }
        STATE_TAKE_CARE_OF_MON => {
            if ({
                (*sTradeAnim).timer += 1;
                (*sTradeAnim).timer
            }) == 10
            {
                PlayFanfare(MUS_EVOLVED);
            }
            if (*sTradeAnim).timer == 250 {
                (*sTradeAnim).state += 1;
                StringExpandPlaceholders(
                    gStringVar4.as_mut_ptr(),
                    gText_TakeGoodCareOfX.as_ptr().cast_mut(),
                );
                DrawTextOnTradeWindow(0, gStringVar4.as_mut_ptr(), 0);
                (*sTradeAnim).timer = 0;
            }
        }
        STATE_AFTER_NEW_MON_DELAY => {
            if ({
                (*sTradeAnim).timer += 1;
                (*sTradeAnim).timer
            }) == 60
            {
                (*sTradeAnim).state += 1;
            }
        }
        STATE_CHECK_RIBBONS => {
            CheckPartnersMonForRibbons();
            (*sTradeAnim).state += 1;
        }
        STATE_END_LINK_TRADE => {
            if (*sTradeAnim).isLinkTrade != 0 {
                return TRUE;
            } else if gMain.newKeys as i32 & A_BUTTON != 0 {
                (*sTradeAnim).state += 1;
            }
        }
        STATE_TRY_EVOLUTION => {
            TradeMons(gSpecialVar_0x8005 as u8, 0);
            gCB2_AfterEvolution = Some(CB2_InGameTrade);
            evoTarget = GetEvolutionTargetSpecies(
                &raw mut gPlayerParty[gSelectedTradeMonPositions[0]],
                EVO_MODE_TRADE,
                ITEM_NONE,
            );
            if evoTarget != SPECIES_NONE {
                TradeEvolutionScene(
                    &raw mut gPlayerParty[gSelectedTradeMonPositions[0]],
                    evoTarget,
                    (*sTradeAnim).monSpriteIds[1],
                    gSelectedTradeMonPositions[0],
                );
            }
            (*sTradeAnim).state += 1;
        }
        STATE_FADE_OUT_END => {
            BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 16, 0);
            (*sTradeAnim).state += 1;
        }
        STATE_WAIT_FADE_OUT_END => {
            if gPaletteFade.active() == 0 {
                PlayNewMapMusic((*sTradeAnim).cachedMapMusic);
                if !sTradeAnim.is_null() {
                    FreeAllWindowBuffers();
                    Free(GetBgTilemapBuffer(3));
                    Free(GetBgTilemapBuffer(1));
                    Free(GetBgTilemapBuffer(0));
                    FreeMonSpritesGfx();
                    Free(sTradeAnim as *mut c_void);
                    sTradeAnim = null_mut();
                }
                SetMainCallback2(Some(CB2_ReturnToField));
                BufferInGameTradeMonName();
            }
        }
        _ => {}
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn DoTradeAnim_Wireless() -> u8 {
    let mut evoTarget: u16 = 0;
    match (*sTradeAnim).state {
        STATE_START => {
            gSprites[(*sTradeAnim).monSpriteIds[0]].set_invisible(0);
            gSprites[(*sTradeAnim).monSpriteIds[0]].x2 = -180;
            gSprites[(*sTradeAnim).monSpriteIds[0]].y2 =
                gMonFrontPicCoords[(*sTradeAnim).monSpecies[0]].y_offset as i16;
            (*sTradeAnim).state += 1;
            (*sTradeAnim).cachedMapMusic = GetCurrentMapMusic();
            PlayNewMapMusic(MUS_EVOLUTION);
        }
        STATE_MON_SLIDE_IN => {
            if (*sTradeAnim).bg2hofs > 0 {
                gSprites[(*sTradeAnim).monSpriteIds[0]].x2 += 3;
                (*sTradeAnim).bg2hofs -= 3;
            } else {
                gSprites[(*sTradeAnim).monSpriteIds[0]].x2 = 0;
                (*sTradeAnim).bg2hofs = 0;
                (*sTradeAnim).state = STATE_SEND_MSG;
            }
        }
        STATE_SEND_MSG => {
            StringExpandPlaceholders(
                gStringVar4.as_mut_ptr(),
                gText_XWillBeSentToY.as_ptr().cast_mut(),
            );
            DrawTextOnTradeWindow(0, gStringVar4.as_mut_ptr(), 0);
            if (*sTradeAnim).monSpecies[0] != SPECIES_EGG as u16 {
                PlayCry_Normal((*sTradeAnim).monSpecies[0], 0);
            }
            (*sTradeAnim).state = STATE_BYE_BYE;
            (*sTradeAnim).timer = 0;
        }
        STATE_BYE_BYE => {
            if ({
                (*sTradeAnim).timer += 1;
                (*sTradeAnim).timer
            }) == 80
            {
                (*sTradeAnim).releasePokeballSpriteId = CreateTradePokeballSprite(
                    (*sTradeAnim).monSpriteIds[0],
                    gSprites[(*sTradeAnim).monSpriteIds[0]].oam.paletteNum() as u8,
                    120,
                    32,
                    2,
                    1,
                    0x14,
                    0xfffff,
                );
                (*sTradeAnim).state += 1;
                StringExpandPlaceholders(
                    gStringVar4.as_mut_ptr(),
                    gText_ByeByeVar1.as_ptr().cast_mut(),
                );
                DrawTextOnTradeWindow(0, gStringVar4.as_mut_ptr(), 0);
            }
        }
        STATE_POKEBALL_DEPART => {
            if gSprites[(*sTradeAnim).releasePokeballSpriteId].callback
                == Some(SpriteCallbackDummy as unsafe extern "C" fn(*mut Sprite))
            {
                (*sTradeAnim).bouncingPokeballSpriteId = CreateSprite(
                    (&raw const *sSpriteTemplate_Pokeball).cast_mut(),
                    120,
                    32,
                    0,
                );
                gSprites[(*sTradeAnim).bouncingPokeballSpriteId].callback =
                    Some(SpriteCB_BouncingPokeballDepart);
                DestroySprite(&raw mut gSprites[(*sTradeAnim).releasePokeballSpriteId]);
                (*sTradeAnim).state += 1;
            }
        }
        STATE_POKEBALL_DEPART_WAIT => {}
        STATE_FADE_OUT_TO_GBA_SEND => {
            BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 16, 0);
            (*sTradeAnim).state = STATE_WAIT_FADE_OUT_TO_GBA_SEND;
        }
        STATE_WAIT_FADE_OUT_TO_GBA_SEND => {
            if gPaletteFade.active() == 0 {
                SetTradeSequenceBgGpuRegs(4);
                FillWindowPixelBuffer(0, 255);
                CopyWindowToVram(0, COPYWIN_FULL);
                (*sTradeAnim).state += 1;
            }
        }
        STATE_FADE_IN_TO_GBA_SEND => {
            BeginNormalPaletteFade(PALETTES_ALL, -1, 16, 0, 0);
            (*sTradeAnim).state += 1;
        }
        STATE_WAIT_FADE_IN_TO_GBA_SEND => {
            if gPaletteFade.active() == 0 {
                (*sTradeAnim).state = STATE_GBA_ZOOM_OUT;
            }
        }
        STATE_GBA_ZOOM_OUT => {
            if (*sTradeAnim).gbaScale > 0x100 {
                (*sTradeAnim).gbaScale -= 0x34;
            } else {
                SetTradeSequenceBgGpuRegs(1);
                (*sTradeAnim).gbaScale = 0x80;
                (*sTradeAnim).state = STATE_GBA_FLASH_SEND_WIRELESS;
                (*sTradeAnim).timer = 0;
            }
            (*sTradeAnim).sXY = div_i32(0x8000, (*sTradeAnim).gbaScale as i32) as u16;
        }
        STATE_GBA_FLASH_SEND_WIRELESS => {
            if ({
                (*sTradeAnim).timer += 1;
                (*sTradeAnim).timer
            }) > 20
            {
                SetTradeSequenceBgGpuRegs(3);
                (*sTradeAnim).connectionSpriteId2 = CreateSprite(
                    (&raw const *sSpriteTemplate_GbaScreenFlash_Short).cast_mut(),
                    120,
                    80,
                    0,
                );
                (*sTradeAnim).state += 1;
            }
        }
        STATE_GBA_STOP_FLASH_SEND_WIRELESS => {
            if gSprites[(*sTradeAnim).connectionSpriteId2].animEnded() != 0 {
                DestroySprite(&raw mut gSprites[(*sTradeAnim).connectionSpriteId2]);
                SetGpuReg(REG_OFFSET_BLDCNT, 1106);
                SetGpuReg(REG_OFFSET_BLDALPHA, 1040);
                CreateTask(Some(Task_AnimateWirelessSignal), 5);
                (*sTradeAnim).state += 1;
            }
        }
        STATE_WAIT_WIRELESS_SIGNAL_SEND => {
            if FuncIsActiveTask(Some(Task_AnimateWirelessSignal)) == 0 {
                (*sTradeAnim).state = STATE_PAN_AWAY_GBA;
            }
        }
        STATE_PAN_AWAY_GBA => {
            if ({
                (*sTradeAnim).bg1vofs -= 1;
                (*sTradeAnim).bg1vofs
            }) == 316
            {
                (*sTradeAnim).state += 1;
            }
        }
        STATE_CREATE_LINK_MON_LEAVING => {
            (*sTradeAnim).connectionSpriteId1 = CreateSprite(
                (&raw const *sSpriteTemplate_LinkMonGlow).cast_mut(),
                120,
                80,
                3,
            );
            gSprites[(*sTradeAnim).connectionSpriteId1].callback =
                Some(SpriteCB_LinkMonGlowWireless);
            (*sTradeAnim).connectionSpriteId2 = CreateSprite(
                (&raw const *sSpriteTemplate_LinkMonShadow).cast_mut(),
                120,
                80,
                0,
            );
            StartSpriteAnim(
                &raw mut gSprites[(*sTradeAnim).connectionSpriteId2],
                ANIM_LINKMON_SMALL,
            );
            (*sTradeAnim).state += 1;
        }
        STATE_LINK_MON_TRAVEL_OUT => {
            if ({
                (*sTradeAnim).bg1vofs -= 3;
                (*sTradeAnim).bg1vofs
            }) == 166
            {
                (*sTradeAnim).state = STATE_LINK_MON_TRAVEL_OFFSCREEN;
            }
            SetGpuReg(REG_OFFSET_DISPCNT, 4673);
        }
        STATE_LINK_MON_TRAVEL_OFFSCREEN => {
            gSprites[(*sTradeAnim).connectionSpriteId1].y -= 2;
            gSprites[(*sTradeAnim).connectionSpriteId2].y -= 2;
            if gSprites[(*sTradeAnim).connectionSpriteId1].y < -8 {
                (*sTradeAnim).state = STATE_FADE_OUT_TO_CROSSING;
            }
        }
        STATE_FADE_OUT_TO_CROSSING => {
            BeginNormalPaletteFade(PALETTES_ALL, -1, 0, 16, 0);
            (*sTradeAnim).state = STATE_WAIT_FADE_OUT_TO_CROSSING;
        }
        STATE_WAIT_FADE_OUT_TO_CROSSING => {
            if gPaletteFade.active() == 0 {
                DestroySprite(&raw mut gSprites[(*sTradeAnim).connectionSpriteId1]);
                DestroySprite(&raw mut gSprites[(*sTradeAnim).connectionSpriteId2]);
                SetTradeSequenceBgGpuRegs(2);
                (*sTradeAnim).state += 1;
            }
        }
        STATE_FADE_IN_TO_CROSSING => {
            BeginNormalPaletteFade(PALETTES_ALL, -1, 16, 0, 0);
            (*sTradeAnim).connectionSpriteId1 = CreateSprite(
                (&raw const *sSpriteTemplate_LinkMonShadow).cast_mut(),
                111,
                170,
                0,
            );
            (*sTradeAnim).connectionSpriteId2 = CreateSprite(
                (&raw const *sSpriteTemplate_LinkMonShadow).cast_mut(),
                129,
                -10,
                0,
            );
            (*sTradeAnim).state += 1;
        }
        STATE_WAIT_FADE_IN_TO_CROSSING => {
            if gPaletteFade.active() == 0 {
                PlaySE(SE_WARP_OUT);
                (*sTradeAnim).state += 1;
            }
            gSprites[(*sTradeAnim).connectionSpriteId1].y2 -= 3;
            gSprites[(*sTradeAnim).connectionSpriteId2].y2 += 3;
        }
        STATE_CROSSING_LINK_MONS_ENTER => {
            gSprites[(*sTradeAnim).connectionSpriteId1].y2 -= 3;
            gSprites[(*sTradeAnim).connectionSpriteId2].y2 += 3;
            if gSprites[(*sTradeAnim).connectionSpriteId1].y2 <= -90 {
                gSprites[(*sTradeAnim).connectionSpriteId1].data[1] = 1;
                gSprites[(*sTradeAnim).connectionSpriteId2].data[1] = 1;
                (*sTradeAnim).state += 1;
                CreateTask(Some(Task_OpenCenterWhiteColumn), 5);
            }
        }
        STATE_CROSSING_BLEND_WHITE_1 => {
            BlendPalettes(0x8, 16, 65535);
            (*sTradeAnim).state += 1;
        }
        STATE_CROSSING_BLEND_WHITE_2 => {
            BlendPalettes(0x8, 16, 65535);
            (*sTradeAnim).state += 1;
        }
        STATE_CROSSING_BLEND_WHITE_3 => {
            BlendPalettes(0x8, 16, 65535);
            (*sTradeAnim).state += 1;
        }
        STATE_CROSSING_CREATE_MON_PICS => {
            if IsMonSpriteNotFlipped((*sTradeAnim).monSpecies[0]) == 0 {
                gSprites[(*sTradeAnim).monSpriteIds[0]].affineAnims =
                    sAffineAnims_CrossingMonPics.as_ptr().cast_mut();
                gSprites[(*sTradeAnim).monSpriteIds[0]]
                    .oam
                    .set_affineMode(ST_OAM_AFFINE_DOUBLE);
                CalcCenterToCornerVec(
                    &raw mut gSprites[(*sTradeAnim).monSpriteIds[0]],
                    TRADE_PLAYER,
                    ST_OAM_AFFINE_DOUBLE as u8,
                    ST_OAM_AFFINE_DOUBLE as u8,
                );
                StartSpriteAffineAnim(&raw mut gSprites[(*sTradeAnim).monSpriteIds[0]], 0);
            } else {
                StartSpriteAffineAnim(&raw mut gSprites[(*sTradeAnim).monSpriteIds[0]], 0);
            }
            StartSpriteAffineAnim(&raw mut gSprites[(*sTradeAnim).monSpriteIds[1]], 0);
            gSprites[(*sTradeAnim).monSpriteIds[0]].x = 40;
            gSprites[(*sTradeAnim).monSpriteIds[1]].x = 200;
            gSprites[(*sTradeAnim).monSpriteIds[0]].y = 192;
            gSprites[(*sTradeAnim).monSpriteIds[1]].y = -32;
            gSprites[(*sTradeAnim).monSpriteIds[0]].set_invisible(0);
            gSprites[(*sTradeAnim).monSpriteIds[1]].set_invisible(FALSE as u16);
            (*sTradeAnim).state += 1;
        }
        STATE_CROSSING_MON_PICS_MOVE => {
            gSprites[(*sTradeAnim).monSpriteIds[0]].y2 -= 3;
            gSprites[(*sTradeAnim).monSpriteIds[1]].y2 += 3;
            if gSprites[(*sTradeAnim).monSpriteIds[0]].y2 < -160
                && gSprites[(*sTradeAnim).monSpriteIds[0]].y2 >= -163
            {
                PlaySE(SE_WARP_IN);
            }
            if gSprites[(*sTradeAnim).monSpriteIds[0]].y2 < -222 {
                gSprites[(*sTradeAnim).connectionSpriteId1].data[1] = 0;
                gSprites[(*sTradeAnim).connectionSpriteId2].data[1] = 0;
                (*sTradeAnim).state += 1;
                gSprites[(*sTradeAnim).monSpriteIds[0]].set_invisible(TRUE as u16);
                gSprites[(*sTradeAnim).monSpriteIds[1]].set_invisible(1);
                CreateTask(Some(Task_CloseCenterWhiteColumn), 5);
            }
        }
        STATE_CROSSING_LINK_MONS_EXIT => {
            gSprites[(*sTradeAnim).connectionSpriteId1].y2 -= 3;
            gSprites[(*sTradeAnim).connectionSpriteId2].y2 += 3;
            if gSprites[(*sTradeAnim).connectionSpriteId1].y2 <= -222 {
                BeginNormalPaletteFade(PALETTES_ALL, -1, 0, 16, 0);
                (*sTradeAnim).state += 1;
                DestroySprite(&raw mut gSprites[(*sTradeAnim).connectionSpriteId1]);
                DestroySprite(&raw mut gSprites[(*sTradeAnim).connectionSpriteId2]);
            }
        }
        STATE_CREATE_LINK_MON_ARRIVING => {
            if gPaletteFade.active() == 0 {
                (*sTradeAnim).state += 1;
                SetTradeSequenceBgGpuRegs(1);
                (*sTradeAnim).bg1vofs = 166;
                SetTradeSequenceBgGpuRegs(3);
                (*sTradeAnim).bg2vofs = 412;
                (*sTradeAnim).connectionSpriteId1 = CreateSprite(
                    (&raw const *sSpriteTemplate_LinkMonGlow).cast_mut(),
                    120,
                    -20,
                    3,
                );
                gSprites[(*sTradeAnim).connectionSpriteId1].callback =
                    Some(SpriteCB_LinkMonGlowWireless);
                (*sTradeAnim).connectionSpriteId2 = CreateSprite(
                    (&raw const *sSpriteTemplate_LinkMonShadow).cast_mut(),
                    120,
                    -20,
                    0,
                );
                StartSpriteAnim(
                    &raw mut gSprites[(*sTradeAnim).connectionSpriteId2],
                    ANIM_LINKMON_SMALL,
                );
            }
        }
        STATE_FADE_OUT_TO_GBA_RECV => {
            BeginNormalPaletteFade(PALETTES_ALL, -1, 16, 0, 0);
            (*sTradeAnim).state += 1;
        }
        STATE_WAIT_FADE_OUT_TO_GBA_RECV => {
            SetGpuReg(0x0, 4672);
            if gPaletteFade.active() == 0 {
                (*sTradeAnim).state += 1;
            }
        }
        STATE_LINK_MON_TRAVEL_IN => {
            gSprites[(*sTradeAnim).connectionSpriteId1].y2 += 4;
            gSprites[(*sTradeAnim).connectionSpriteId2].y2 += 4;
            if gSprites[(*sTradeAnim).connectionSpriteId1].y2 as i32
                + gSprites[(*sTradeAnim).connectionSpriteId1].y as i32
                == 64
            {
                (*sTradeAnim).state = STATE_PAN_TO_GBA_WIRELESS;
                (*sTradeAnim).timer = 0;
            }
        }
        STATE_PAN_TO_GBA_WIRELESS => {
            SetGpuReg(0x0, 5696);
            (*sTradeAnim).bg1vofs += 3;
            (*sTradeAnim).bg2vofs += 3;
            if ({
                (*sTradeAnim).timer += 1;
                (*sTradeAnim).timer
            }) == 10
            {
                let mut taskId: u8 = CreateTask(Some(Task_AnimateWirelessSignal), 5);
                gTasks[taskId].data[2] = TRUE as i16;
            }
            if (*sTradeAnim).bg1vofs > 316 {
                (*sTradeAnim).bg1vofs = 316;
                (*sTradeAnim).state += 1;
            }
        }
        STATE_DESTROY_LINK_MON_WIRELESS => {
            DestroySprite(&raw mut gSprites[(*sTradeAnim).connectionSpriteId1]);
            DestroySprite(&raw mut gSprites[(*sTradeAnim).connectionSpriteId2]);
            (*sTradeAnim).state += 1;
            (*sTradeAnim).timer = 0;
        }
        STATE_WAIT_WIRELESS_SIGNAL_RECV => {
            if FuncIsActiveTask(Some(Task_AnimateWirelessSignal)) == 0 {
                (*sTradeAnim).state = STATE_LINK_MON_ARRIVED_DELAY;
                (*sTradeAnim).timer = 0;
            }
        }
        STATE_LINK_MON_ARRIVED_DELAY => {
            if ({
                (*sTradeAnim).timer += 1;
                (*sTradeAnim).timer
            }) == 10
            {
                (*sTradeAnim).state += 1;
            }
        }
        STATE_MOVE_GBA_TO_CENTER => {
            if ({
                (*sTradeAnim).bg1vofs += 1;
                (*sTradeAnim).bg1vofs
            }) > 348
            {
                (*sTradeAnim).bg1vofs = 348;
                (*sTradeAnim).state += 1;
            }
        }
        STATE_GBA_FLASH_RECV => {
            (*sTradeAnim).connectionSpriteId2 = CreateSprite(
                (&raw const *sSpriteTemplate_GbaScreenFlash_Long).cast_mut(),
                120,
                80,
                0,
            );
            (*sTradeAnim).state = STATE_GBA_STOP_FLASH_RECV;
        }
        STATE_GBA_STOP_FLASH_RECV => {
            if gSprites[(*sTradeAnim).connectionSpriteId2].animEnded() != 0 {
                DestroySprite(&raw mut gSprites[(*sTradeAnim).connectionSpriteId2]);
                SetTradeSequenceBgGpuRegs(6);
                (*sTradeAnim).state += 1;
                PlaySE(SE_M_SAND_ATTACK);
            }
        }
        STATE_GBA_ZOOM_IN => {
            if (*sTradeAnim).gbaScale < 0x400 {
                (*sTradeAnim).gbaScale += 0x34;
            } else {
                (*sTradeAnim).gbaScale = 0x400;
                (*sTradeAnim).state += 1;
            }
            (*sTradeAnim).sXY = div_i32(0x8000, (*sTradeAnim).gbaScale as i32) as u16;
        }
        STATE_FADE_OUT_TO_NEW_MON => {
            BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 16, 0);
            (*sTradeAnim).state = STATE_WAIT_FADE_OUT_TO_NEW_MON;
        }
        STATE_WAIT_FADE_OUT_TO_NEW_MON => {
            if gPaletteFade.active() == 0 {
                SetTradeSequenceBgGpuRegs(5);
                SetTradeSequenceBgGpuRegs(7);
                gPaletteFade.set_bufferTransferDisabled(TRUE as u16);
                (*sTradeAnim).state += 1;
            }
        }
        STATE_FADE_IN_TO_NEW_MON => {
            gPaletteFade.set_bufferTransferDisabled(FALSE as u16);
            BeginNormalPaletteFade(PALETTES_ALL, 0, 16, 0, 0);
            (*sTradeAnim).state += 1;
        }
        STATE_WAIT_FADE_IN_TO_NEW_MON => {
            SetGpuReg(0x0, 5184);
            if gPaletteFade.active() == 0 {
                (*sTradeAnim).state += 1;
            }
        }
        STATE_POKEBALL_ARRIVE => {
            (*sTradeAnim).bouncingPokeballSpriteId = CreateSprite(
                (&raw const *sSpriteTemplate_Pokeball).cast_mut(),
                120,
                -8,
                0,
            );
            gSprites[(*sTradeAnim).bouncingPokeballSpriteId].data[3] = 74;
            gSprites[(*sTradeAnim).bouncingPokeballSpriteId].callback =
                Some(SpriteCB_BouncingPokeballArrive);
            StartSpriteAnim(&raw mut gSprites[(*sTradeAnim).bouncingPokeballSpriteId], 1);
            StartSpriteAffineAnim(&raw mut gSprites[(*sTradeAnim).bouncingPokeballSpriteId], 2);
            BlendPalettes(
                shl_i32(
                    1,
                    16 + gSprites[(*sTradeAnim).bouncingPokeballSpriteId]
                        .oam
                        .paletteNum() as u32,
                ) as u32,
                16,
                65535,
            );
            (*sTradeAnim).state += 1;
            (*sTradeAnim).timer = 0;
        }
        STATE_FADE_POKEBALL_TO_NORMAL => {
            BeginNormalPaletteFade(
                shl_i32(
                    1,
                    16 + gSprites[(*sTradeAnim).bouncingPokeballSpriteId]
                        .oam
                        .paletteNum() as u32,
                ) as u32,
                1,
                16,
                0,
                65535,
            );
            (*sTradeAnim).state += 1;
        }
        STATE_POKEBALL_ARRIVE_WAIT => {
            if gSprites[(*sTradeAnim).bouncingPokeballSpriteId].callback
                == Some(SpriteCallbackDummy as unsafe extern "C" fn(*mut Sprite))
            {
                HandleLoadSpecialPokePic_2(
                    (&raw const gMonFrontPicTable[(*sTradeAnim).monSpecies[1]]).cast_mut(),
                    (*gMonSpritesGfxPtr).sprites.ptr[3],
                    (*sTradeAnim).monSpecies[1] as i32,
                    (*sTradeAnim).monPersonalities[1],
                );
                (*sTradeAnim).state += 1;
            }
        }
        STATE_SHOW_NEW_MON => {
            gSprites[(*sTradeAnim).monSpriteIds[1]].x = 120;
            gSprites[(*sTradeAnim).monSpriteIds[1]].y =
                gMonFrontPicCoords[(*sTradeAnim).monSpecies[1]].y_offset as i16 + 60;
            gSprites[(*sTradeAnim).monSpriteIds[1]].x2 = 0;
            gSprites[(*sTradeAnim).monSpriteIds[1]].y2 = 0;
            StartSpriteAnim(&raw mut gSprites[(*sTradeAnim).monSpriteIds[1]], 0);
            CreatePokeballSpriteToReleaseMon(
                (*sTradeAnim).monSpriteIds[1],
                gSprites[(*sTradeAnim).monSpriteIds[1]].oam.paletteNum() as u8,
                120,
                84,
                2,
                1,
                20,
                0xfffff,
                (*sTradeAnim).monSpecies[1],
            );
            FreeSpriteOamMatrix(&raw mut gSprites[(*sTradeAnim).bouncingPokeballSpriteId]);
            DestroySprite(&raw mut gSprites[(*sTradeAnim).bouncingPokeballSpriteId]);
            (*sTradeAnim).state += 1;
        }
        STATE_NEW_MON_MSG => {
            SetGpuReg(0x0, 5440);
            StringExpandPlaceholders(
                gStringVar4.as_mut_ptr(),
                gText_XSentOverY.as_ptr().cast_mut(),
            );
            DrawTextOnTradeWindow(0, gStringVar4.as_mut_ptr(), 0);
            (*sTradeAnim).state = STATE_DELAY_FOR_MON_ANIM;
            (*sTradeAnim).timer = 0;
        }
        STATE_DELAY_FOR_MON_ANIM => {
            if ({
                (*sTradeAnim).timer += 1;
                (*sTradeAnim).timer
            }) > 60
            {
                (*sTradeAnim).state = STATE_WAIT_FOR_MON_CRY;
                (*sTradeAnim).timer = 0;
            }
        }
        STATE_WAIT_FOR_MON_CRY => {
            if IsCryFinished() != 0 {
                (*sTradeAnim).state = STATE_TAKE_CARE_OF_MON;
            }
        }
        STATE_TAKE_CARE_OF_MON => {
            if ({
                (*sTradeAnim).timer += 1;
                (*sTradeAnim).timer
            }) == 10
            {
                PlayFanfare(MUS_EVOLVED);
            }
            if (*sTradeAnim).timer == 250 {
                (*sTradeAnim).state += 1;
                StringExpandPlaceholders(
                    gStringVar4.as_mut_ptr(),
                    gText_TakeGoodCareOfX.as_ptr().cast_mut(),
                );
                DrawTextOnTradeWindow(0, gStringVar4.as_mut_ptr(), 0);
                (*sTradeAnim).timer = 0;
            }
        }
        STATE_AFTER_NEW_MON_DELAY => {
            if ({
                (*sTradeAnim).timer += 1;
                (*sTradeAnim).timer
            }) == 60
            {
                (*sTradeAnim).state += 1;
            }
        }
        STATE_CHECK_RIBBONS => {
            CheckPartnersMonForRibbons();
            (*sTradeAnim).state += 1;
        }
        STATE_END_LINK_TRADE => {
            if (*sTradeAnim).isLinkTrade != 0 {
                return TRUE;
            } else if gMain.newKeys as i32 & A_BUTTON != 0 {
                (*sTradeAnim).state += 1;
            }
        }
        STATE_TRY_EVOLUTION => {
            TradeMons(gSpecialVar_0x8005 as u8, 0);
            gCB2_AfterEvolution = Some(CB2_InGameTrade);
            evoTarget = GetEvolutionTargetSpecies(
                &raw mut gPlayerParty[gSelectedTradeMonPositions[0]],
                EVO_MODE_TRADE,
                ITEM_NONE,
            );
            if evoTarget != SPECIES_NONE {
                TradeEvolutionScene(
                    &raw mut gPlayerParty[gSelectedTradeMonPositions[0]],
                    evoTarget,
                    (*sTradeAnim).monSpriteIds[1],
                    gSelectedTradeMonPositions[0],
                );
            }
            (*sTradeAnim).state += 1;
        }
        STATE_FADE_OUT_END => {
            BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 16, 0);
            (*sTradeAnim).state += 1;
        }
        STATE_WAIT_FADE_OUT_END => {
            if gPaletteFade.active() == 0 {
                PlayNewMapMusic((*sTradeAnim).cachedMapMusic);
                if !sTradeAnim.is_null() {
                    FreeAllWindowBuffers();
                    Free(GetBgTilemapBuffer(3));
                    Free(GetBgTilemapBuffer(1));
                    Free(GetBgTilemapBuffer(0));
                    FreeMonSpritesGfx();
                    Free(sTradeAnim as *mut c_void);
                    sTradeAnim = null_mut();
                }
                SetMainCallback2(Some(CB2_ReturnToField));
                BufferInGameTradeMonName();
            }
        }
        _ => {}
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn CB2_TryLinkTradeEvolution() {
    let mut evoTarget: u16 = 0;
    match gMain.state {
        0 => {
            gMain.state = 4;
            gSoftResetDisabled = TRUE;
        }
        4 => {
            gCB2_AfterEvolution = Some(CB2_SaveAndEndTrade);
            evoTarget = GetEvolutionTargetSpecies(
                &raw mut gPlayerParty[gSelectedTradeMonPositions[0]],
                EVO_MODE_TRADE,
                ITEM_NONE,
            );
            if evoTarget != SPECIES_NONE {
                TradeEvolutionScene(
                    &raw mut gPlayerParty[gSelectedTradeMonPositions[0]],
                    evoTarget,
                    (*sTradeAnim).monSpriteIds[1],
                    gSelectedTradeMonPositions[0],
                );
            } else if IsWirelessTrade() != 0 {
                SetMainCallback2(Some(CB2_SaveAndEndWirelessTrade));
            } else {
                SetMainCallback2(Some(CB2_SaveAndEndTrade));
            }
            gSelectedTradeMonPositions[0] = 255;
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
pub(crate) unsafe extern "C" fn HandleLinkDataReceive() {
    let mut recvStatus: u8 = 0;
    TradeGetMultiplayerId();
    recvStatus = GetBlockReceivedStatus();
    if recvStatus as i32 & 1 != 0 {
        if gBlockRecvBuffer[0][0] == LINKCMD_CONFIRM_FINISH_TRADE {
            SetMainCallback2(Some(CB2_TryLinkTradeEvolution));
        }
        if gBlockRecvBuffer[0][0] == LINKCMD_READY_FINISH_TRADE {
            (*sTradeAnim).playerFinishStatus = STATUS_READY;
        }
        ResetBlockReceivedFlag(0);
    }
    if recvStatus as i32 & 2 != 0 {
        if gBlockRecvBuffer[1][0] == LINKCMD_READY_FINISH_TRADE {
            (*sTradeAnim).partnerFinishStatus = STATUS_READY;
        }
        ResetBlockReceivedFlag(1);
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_BouncingPokeball(sprite: *mut Sprite) {
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
pub(crate) unsafe extern "C" fn SpriteCB_BouncingPokeballDepart(sprite: *mut Sprite) {
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
pub(crate) unsafe extern "C" fn SpriteCB_BouncingPokeballDepartEnd(sprite: *mut Sprite) {
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
            (*sTradeAnim).state = STATE_FADE_OUT_TO_GBA_SEND;
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_BouncingPokeballArrive(sprite: *mut Sprite) {
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
pub unsafe extern "C" fn GetInGameTradeSpeciesInfo() -> u16 {
    let mut inGameTrade: *mut InGameTrade =
        (&raw const sIngameTrades[gSpecialVar_0x8004]).cast_mut();
    StringCopy(
        gStringVar1.as_mut_ptr(),
        gSpeciesNames[(*inGameTrade).requestedSpecies]
            .as_ptr()
            .cast_mut(),
    );
    StringCopy(
        gStringVar2.as_mut_ptr(),
        gSpeciesNames[(*inGameTrade).species].as_ptr().cast_mut(),
    );
    return (*inGameTrade).requestedSpecies;
}
pub(crate) unsafe extern "C" fn BufferInGameTradeMonName() {
    let mut nickname: CArray<u8, 32> = zeroed();
    let mut inGameTrade: *mut InGameTrade =
        (&raw const sIngameTrades[gSpecialVar_0x8004]).cast_mut();
    GetMonData3(
        &raw mut gPlayerParty[gSpecialVar_0x8005],
        MON_DATA_NICKNAME,
        nickname.as_mut_ptr(),
    );
    StringCopy_Nickname(gStringVar1.as_mut_ptr(), nickname.as_mut_ptr());
    StringCopy(
        gStringVar2.as_mut_ptr(),
        gSpeciesNames[(*inGameTrade).species].as_ptr().cast_mut(),
    );
}
pub(crate) unsafe extern "C" fn CreateInGameTradePokemonInternal(
    whichPlayerMon: u8,
    whichInGameTrade: u8,
) {
    let mut inGameTrade: *mut InGameTrade = (&raw const sIngameTrades[whichInGameTrade]).cast_mut();
    let mut level: u8 = GetMonData2(&raw mut gPlayerParty[whichPlayerMon], MON_DATA_LEVEL) as u8;
    let mut mail: Mail = zeroed();
    let mut metLocation: u8 = METLOC_IN_GAME_TRADE;
    let mut mailNum: u8 = 0;
    let mut pokemon: *mut Pokemon = &raw mut gEnemyParty[0];
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
    mailNum = 0;
    if (*inGameTrade).heldItem != ITEM_NONE {
        if ItemIsMail((*inGameTrade).heldItem) != 0 {
            GetInGameTradeMail(&raw mut mail, inGameTrade);
            gTradeMail[0] = mail;
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
    CalculateMonStats(&raw mut gEnemyParty[0]);
}
pub(crate) unsafe extern "C" fn GetInGameTradeMail(mail: *mut Mail, trade: *mut InGameTrade) {
    let mut i: i32 = 0;
    i = 0;
    while i < MAIL_WORDS_COUNT as i32 {
        (*mail).words[i] = sIngameTradeMail[(*trade).mailNum][i];
        i += 1;
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
pub unsafe extern "C" fn GetTradeSpecies() -> u16 {
    if GetMonData2(&raw mut gPlayerParty[gSpecialVar_0x8005], MON_DATA_IS_EGG) != 0 {
        return SPECIES_NONE;
    }
    return GetMonData2(&raw mut gPlayerParty[gSpecialVar_0x8005], MON_DATA_SPECIES) as u16;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CreateInGameTradePokemon() {
    CreateInGameTradePokemonInternal(gSpecialVar_0x8005 as u8, gSpecialVar_0x8004 as u8);
}
pub(crate) unsafe extern "C" fn CB2_UpdateLinkTrade() {
    if DoTradeAnim() == TRUE {
        DestroySprite(&raw mut gSprites[(*sTradeAnim).monSpriteIds[0]]);
        FreeSpriteOamMatrix(&raw mut gSprites[(*sTradeAnim).monSpriteIds[1]]);
        TradeMons(
            gSelectedTradeMonPositions[0],
            (gSelectedTradeMonPositions[1] as i32 % 6) as u8,
        );
        if IsWirelessTrade() == 0 {
            (*sTradeAnim).linkData[0] = LINKCMD_READY_FINISH_TRADE;
            (*sTradeAnim).scheduleLinkTransfer = 1;
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
pub(crate) unsafe extern "C" fn CB2_WaitTradeComplete() {
    let mut mpId: u8 = TradeGetMultiplayerId() as u8;
    if IsWirelessTrade() != 0 {
        SetMainCallback2(Some(CB2_TryLinkTradeEvolution));
    } else {
        HandleLinkDataReceive();
        if mpId == 0
            && (*sTradeAnim).playerFinishStatus == STATUS_READY
            && (*sTradeAnim).partnerFinishStatus == STATUS_READY
        {
            (*sTradeAnim).linkData[0] = LINKCMD_CONFIRM_FINISH_TRADE;
            SendBlock(
                BitmaskAllOtherLinkPlayers(),
                (*sTradeAnim).linkData.as_mut_ptr() as *mut c_void,
                20,
            );
            (*sTradeAnim).playerFinishStatus = STATUS_CANCEL;
            (*sTradeAnim).partnerFinishStatus = STATUS_CANCEL;
        }
    }
    RunTasks();
    AnimateSprites();
    BuildOamBuffer();
    UpdatePaletteFade();
}
pub(crate) unsafe extern "C" fn CB2_SaveAndEndTrade() {
    match gMain.state {
        0 => {
            gMain.state += 1;
            StringExpandPlaceholders(
                gStringVar4.as_mut_ptr(),
                gText_CommunicationStandby5.as_ptr().cast_mut(),
            );
            DrawTextOnTradeWindow(0, gStringVar4.as_mut_ptr(), 0);
        }
        1 => {
            SetTradeLinkStandbyCallback(0);
            gMain.state = 100;
            (*sTradeAnim).timer = 0;
        }
        100 => {
            if ({
                (*sTradeAnim).timer += 1;
                (*sTradeAnim).timer
            }) > 180
            {
                gMain.state = 101;
                (*sTradeAnim).timer = 0;
            }
            if _IsLinkTaskFinished() != 0 {
                gMain.state = 2;
            }
        }
        101 => {
            if _IsLinkTaskFinished() != 0 {
                gMain.state = 2;
            }
        }
        2 => {
            gMain.state = 50;
            StringExpandPlaceholders(
                gStringVar4.as_mut_ptr(),
                gText_SavingDontTurnOffPower.as_ptr().cast_mut(),
            );
            DrawTextOnTradeWindow(0, gStringVar4.as_mut_ptr(), 0);
        }
        50 => {
            if InUnionRoom() == 0 {
                IncrementGameStat(GAME_STAT_POKEMON_TRADES);
            }
            if gWirelessCommType != 0 {
                MysteryGift_TryIncrementStat(
                    CARD_STAT_NUM_TRADES,
                    gLinkPlayers[GetMultiplayerId() as i32 ^ 1].trainerId,
                );
            }
            SetContinueGameWarpStatusToDynamicWarp();
            LinkFullSave_Init();
            gMain.state += 1;
            (*sTradeAnim).timer = 0;
        }
        51 => {
            if ({
                (*sTradeAnim).timer += 1;
                (*sTradeAnim).timer
            }) == 5
            {
                gMain.state += 1;
            }
        }
        52 => {
            if LinkFullSave_WriteSector() != 0 {
                ClearContinueGameWarpStatus2();
                gMain.state = 4;
            } else {
                (*sTradeAnim).timer = 0;
                gMain.state = 51;
            }
        }
        4 => {
            LinkFullSave_ReplaceLastSector();
            gMain.state = 40;
            (*sTradeAnim).timer = 0;
        }
        40 => {
            if ({
                (*sTradeAnim).timer += 1;
                (*sTradeAnim).timer
            }) > 50
            {
                if GetMultiplayerId() == 0 {
                    (*sTradeAnim).timer = (Random() as i32 % 30) as u32;
                } else {
                    (*sTradeAnim).timer = 0;
                }
                gMain.state = 41;
            }
        }
        41 => {
            if (*sTradeAnim).timer == 0 {
                SetTradeLinkStandbyCallback(1);
                gMain.state = 42;
            } else {
                (*sTradeAnim).timer -= 1;
            }
        }
        42 => {
            if _IsLinkTaskFinished() != 0 {
                LinkFullSave_SetLastSectorSignature();
                gMain.state = 5;
            }
        }
        5 => {
            if ({
                (*sTradeAnim).timer += 1;
                (*sTradeAnim).timer
            }) > 60
            {
                gMain.state += 1;
                SetTradeLinkStandbyCallback(2);
            }
        }
        6 => {
            if _IsLinkTaskFinished() != 0 {
                BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 16, 0);
                gMain.state += 1;
            }
        }
        7 => {
            if gPaletteFade.active() == 0 {
                FadeOutBGM(3);
                gMain.state += 1;
            }
        }
        8 => {
            if IsBGMStopped() == TRUE {
                if gWirelessCommType != 0
                    && gMain.savedCallback
                        == Some(CB2_StartCreateTradeMenu as unsafe extern "C" fn())
                {
                    SetTradeLinkStandbyCallback(3);
                } else {
                    SetCloseLinkCallback();
                }
                gMain.state += 1;
            }
        }
        9 => {
            if gWirelessCommType != 0
                && gMain.savedCallback == Some(CB2_StartCreateTradeMenu as unsafe extern "C" fn())
            {
                if _IsLinkTaskFinished() != 0 {
                    gSoftResetDisabled = FALSE;
                    SetMainCallback2(Some(CB2_FreeTradeAnim));
                }
            } else if gReceivedRemoteLinkPlayers == 0 {
                gSoftResetDisabled = FALSE;
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
pub(crate) unsafe extern "C" fn CB2_FreeTradeAnim() {
    if gPaletteFade.active() == 0 {
        FreeAllWindowBuffers();
        Free(GetBgTilemapBuffer(3));
        Free(GetBgTilemapBuffer(1));
        Free(GetBgTilemapBuffer(0));
        FreeMonSpritesGfx();
        Free(sTradeAnim as *mut c_void);
        sTradeAnim = null_mut();
        if gWirelessCommType != 0 {
            DestroyWirelessStatusIndicatorSprite();
        }
        SetMainCallback2(gMain.savedCallback);
    }
    RunTasks();
    AnimateSprites();
    BuildOamBuffer();
    UpdatePaletteFade();
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DoInGameTradeScene() {
    LockPlayerFieldControls();
    CreateTask(Some(Task_InGameTrade), 10);
    BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 16, 0);
}
pub(crate) unsafe extern "C" fn Task_InGameTrade(taskId: u8) {
    if gPaletteFade.active() == 0 {
        SetMainCallback2(Some(CB2_InitInGameTrade));
        gFieldCallback = Some(FieldCB_ContinueScriptHandleMusic);
        DestroyTask(taskId);
    }
}
pub(crate) unsafe extern "C" fn CheckPartnersMonForRibbons() {
    let mut i: u8 = 0;
    let mut numRibbons: u8 = 0;
    i = 0;
    while i < 12 {
        numRibbons += GetMonData2(
            &raw mut gEnemyParty[gSelectedTradeMonPositions[1] as i32 % 6],
            MON_DATA_CHAMPION_RIBBON + i as i32,
        ) as u8;
        i += 1;
    }
    if numRibbons != 0 {
        FlagSet(FLAG_SYS_RIBBON_GET);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn LoadTradeAnimGfx() {
    TradeAnimInit_LoadGfx();
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DrawTextOnTradeWindow(windowId: u8, str: *mut u8, speed: u8) {
    FillWindowPixelBuffer(windowId, 255);
    (*sTradeAnim).textColors[0] = TEXT_DYNAMIC_COLOR_6;
    (*sTradeAnim).textColors[1] = 0x1;
    (*sTradeAnim).textColors[2] = TEXT_COLOR_GREEN;
    AddTextPrinterParameterized4(
        windowId,
        FONT_NORMAL,
        0,
        2,
        0,
        0,
        (*sTradeAnim).textColors.as_mut_ptr(),
        speed as i8,
        str,
    );
    CopyWindowToVram(windowId, COPYWIN_FULL);
}
pub(crate) unsafe extern "C" fn Task_AnimateWirelessSignal(taskId: u8) {
    let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
    let mut paletteIdx: u16 = sWirelessSignalAnimParams[*data][0] as u16 * 16;
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
pub(crate) unsafe extern "C" fn Task_OpenCenterWhiteColumn(taskId: u8) {
    let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
    if *data == 0 {
        (*sTradeAnim).wirelessWinLeft = {
            (*sTradeAnim).wirelessWinRight = 120;
            (*sTradeAnim).wirelessWinRight
        };
        (*sTradeAnim).wirelessWinTop = 0;
        (*sTradeAnim).wirelessWinBottom = DISPLAY_HEIGHT as u8;
        SetGpuRegBits(REG_OFFSET_DISPCNT, DISPCNT_WIN0_ON);
        SetGpuReg(REG_OFFSET_WINOUT, WINOUT_WIN01_OBJ);
        SetGpuReg(REG_OFFSET_WININ, 19);
    }
    SetGpuReg(
        REG_OFFSET_WIN0H,
        (*sTradeAnim).wirelessWinRight as u16 | ((*sTradeAnim).wirelessWinLeft as u16) << 8,
    );
    SetGpuReg(
        REG_OFFSET_WIN0V,
        (*sTradeAnim).wirelessWinBottom as u16 | ((*sTradeAnim).wirelessWinTop as u16) << 8,
    );
    *data += 1;
    (*sTradeAnim).wirelessWinLeft -= 5;
    (*sTradeAnim).wirelessWinRight += 5;
    if (*sTradeAnim).wirelessWinLeft < 80 {
        DestroyTask(taskId);
    }
}
pub(crate) unsafe extern "C" fn Task_CloseCenterWhiteColumn(taskId: u8) {
    let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
    if *data == 0 {
        (*sTradeAnim).wirelessWinLeft = 80;
        (*sTradeAnim).wirelessWinRight = 160;
        SetGpuReg(REG_OFFSET_WINOUT, WINOUT_WIN01_OBJ);
        SetGpuReg(REG_OFFSET_WININ, 19);
    }
    SetGpuReg(
        REG_OFFSET_WIN0H,
        (*sTradeAnim).wirelessWinRight as u16 | ((*sTradeAnim).wirelessWinLeft as u16) << 8,
    );
    SetGpuReg(
        REG_OFFSET_WIN0V,
        (*sTradeAnim).wirelessWinBottom as u16 | ((*sTradeAnim).wirelessWinTop as u16) << 8,
    );
    if (*sTradeAnim).wirelessWinLeft != 120 {
        *data += 1;
        (*sTradeAnim).wirelessWinLeft += 5;
        (*sTradeAnim).wirelessWinRight -= 5;
        if (*sTradeAnim).wirelessWinLeft > 115 {
            BlendPalettes(0x8, 0, 65535);
        }
    } else {
        ClearGpuRegBits(REG_OFFSET_DISPCNT, DISPCNT_WIN0_ON);
        DestroyTask(taskId);
    }
}
pub(crate) unsafe extern "C" fn CB2_SaveAndEndWirelessTrade() {
    match gMain.state {
        0 => {
            gMain.state = 1;
            StringExpandPlaceholders(
                gStringVar4.as_mut_ptr(),
                gText_CommunicationStandby5.as_ptr().cast_mut(),
            );
            DrawTextOnTradeWindow(0, gStringVar4.as_mut_ptr(), 0);
        }
        1 => {
            SetTradeLinkStandbyCallback(0);
            gMain.state = 2;
            (*sTradeAnim).timer = 0;
        }
        2 => {
            if _IsLinkTaskFinished() != 0 {
                gMain.state = 3;
                StringExpandPlaceholders(
                    gStringVar4.as_mut_ptr(),
                    gText_SavingDontTurnOffPower.as_ptr().cast_mut(),
                );
                DrawTextOnTradeWindow(0, gStringVar4.as_mut_ptr(), 0);
                IncrementGameStat(GAME_STAT_POKEMON_TRADES);
                LinkFullSave_Init();
                (*sTradeAnim).timer = 0;
            }
        }
        3 => {
            if ({
                (*sTradeAnim).timer += 1;
                (*sTradeAnim).timer
            }) == 5
            {
                gMain.state = 4;
            }
        }
        4 => {
            if LinkFullSave_WriteSector() != 0 {
                gMain.state = 5;
            } else {
                (*sTradeAnim).timer = 0;
                gMain.state = 3;
            }
        }
        5 => {
            LinkFullSave_ReplaceLastSector();
            gMain.state = 6;
            (*sTradeAnim).timer = 0;
        }
        6 => {
            if ({
                (*sTradeAnim).timer += 1;
                (*sTradeAnim).timer
            }) > 10
            {
                if GetMultiplayerId() == 0 {
                    (*sTradeAnim).timer = (Random() as i32 % 30) as u32;
                } else {
                    (*sTradeAnim).timer = 0;
                }
                gMain.state = 7;
            }
        }
        7 => {
            if (*sTradeAnim).timer == 0 {
                SetTradeLinkStandbyCallback(1);
                gMain.state = 8;
            } else {
                (*sTradeAnim).timer -= 1;
            }
        }
        8 => {
            if _IsLinkTaskFinished() != 0 {
                LinkFullSave_SetLastSectorSignature();
                gMain.state = 9;
            }
        }
        9 => {
            if ({
                (*sTradeAnim).timer += 1;
                (*sTradeAnim).timer
            }) > 60
            {
                gMain.state += 1;
                SetTradeLinkStandbyCallback(2);
            }
        }
        10 => {
            if _IsLinkTaskFinished() != 0 {
                FadeOutBGM(3);
                BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 16, 0);
                gMain.state = 11;
            }
        }
        11 => {
            if gPaletteFade.active() == 0 && IsBGMStopped() == TRUE {
                SetTradeLinkStandbyCallback(3);
                gMain.state = 12;
            }
        }
        12 => {
            if _IsLinkTaskFinished() != 0 {
                gSoftResetDisabled = FALSE;
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
