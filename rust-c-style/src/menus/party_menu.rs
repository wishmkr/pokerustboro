//! Translated from `src/party_menu.c` by tools/rustport/c2rs.py.
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
// Data tables (translate with cdata.py): gTutorMoves sTutorLearnsets sPartyMenuBgTemplates sPartyBoxInfoRects sPartyMenuSpriteCoords sConfirmButton_Tilemap sCancelButton_Tilemap sFontColorTable sSinglePartyMenuWindowTemplate sDoublePartyMenuWindowTemplate sMultiPartyMenuWindowTemplate sShowcaseMultiPartyMenuWindowTemplate sCancelButtonWindowTemplate sMultiCancelButtonWindowTemplate sConfirmButtonWindowTemplate sDefaultPartyMsgWindowTemplate sDoWhatWithMonMsgWindowTemplate sDoWhatWithItemMsgWindowTemplate sDoWhatWithMailMsgWindowTemplate sWhichMoveMsgWindowTemplate sAlreadyHoldingOneMsgWindowTemplate sItemGiveTakeWindowTemplate sMailReadTakeWindowTemplate sMoveSelectWindowTemplate sPartyMenuYesNoWindowTemplate sLevelUpStatsWindowTemplate sUnusedWindowTemplate1 sUnusedWindowTemplate2 sSlotTilemap_Main sSlotTilemap_MainNoHP sSlotTilemap_Wide sSlotTilemap_WideNoHP sSlotTilemap_WideEmpty sGenderPalOffsets sHPBarPalOffsets sPartyBoxPalOffsets1 sPartyBoxPalOffsets2 sPartyBoxNoMonPalOffsets sGenderMalePalIds sGenderFemalePalIds sHPBarGreenPalIds sHPBarYellowPalIds sHPBarRedPalIds sPartyBoxEmptySlotPalIds1 sPartyBoxMultiPalIds1 sPartyBoxFaintedPalIds1 sPartyBoxCurrSelectionPalIds1 sPartyBoxCurrSelectionMultiPalIds sPartyBoxCurrSelectionFaintedPalIds sPartyBoxSelectedForActionPalIds1 sPartyBoxEmptySlotPalIds2 sPartyBoxMultiPalIds2 sPartyBoxFaintedPalIds2 sPartyBoxCurrSelectionPalIds2 sPartyBoxSelectedForActionPalIds2 sPartyBoxNoMonPalIds sActionStringTable sDescriptionStringTable sUnusedData sCursorOptions sPartyMenuAction_SummarySwitchCancel sPartyMenuAction_ShiftSummaryCancel sPartyMenuAction_SendOutSummaryCancel sPartyMenuAction_SummaryCancel sPartyMenuAction_EnterSummaryCancel sPartyMenuAction_NoEntrySummaryCancel sPartyMenuAction_StoreSummaryCancel sPartyMenuAction_GiveTakeItemCancel sPartyMenuAction_ReadTakeMailCancel sPartyMenuAction_RegisterSummaryCancel sPartyMenuAction_TradeSummaryCancel1 sPartyMenuAction_TradeSummaryCancel2 sPartyMenuAction_TakeItemTossCancel sPartyMenuActions sPartyMenuActionCounts sFieldMoves sFieldMoveCursorCallbacks sUnionRoomTradeMessages sHeldItemGfx sHeldItemPalette sOamData_HeldItem sSpriteAnim_HeldItem sSpriteAnim_HeldMail sSpriteAnimTable_HeldItem sSpriteSheet_HeldItem sSpritePalette_HeldItem sSpriteTemplate_HeldItem sOamData_MenuPokeball sPokeballAnim_Closed sPokeballAnim_Open sSpriteAnimTable_MenuPokeball sSpriteSheet_MenuPokeball sSpritePalette_MenuPokeball sSpriteTemplate_MenuPokeball sOamData_MenuPokeballSmall sSmallPokeballAnim_Closed sSmallPokeballAnim_Open sSmallPokeballAnim_Blank1 sSmallPokeballAnim_Blank2 sSmallPokeballAnim_Blank3 sSmallPokeballAnim_Blank4 sSpriteAnimTable_MenuPokeballSmall sSpriteSheet_MenuPokeballSmall sSpriteTemplate_MenuPokeballSmall sOamData_StatusCondition sSpriteAnim_StatusPoison sSpriteAnim_StatusParalyzed sSpriteAnim_StatusSleep sSpriteAnim_StatusFrozen sSpriteAnim_StatusBurn sSpriteAnim_StatusPokerus sSpriteAnim_StatusFaint sSpriteAnim_Blank sSpriteTemplate_StatusCondition sSpriteSheet_StatusIcons sSpritePalette_StatusIcons sSpriteTemplate_StatusIcons sMultiBattlePartnersPartyMask sUnused_StatStrings sTMHMMoves

/// `struct PartyMenuInternal`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PartyMenuInternal {
    pub task: Option<unsafe extern "C" fn(u8)>,
    pub exitCallback: Option<unsafe extern "C" fn()>,
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
        ((self.bits_8 as u32 >> 0) & 0x1) as u32
    }
    #[inline(always)]
    pub fn set_chooseHalf(&mut self, v: u32) {
        self.bits_8 = (self.bits_8 & !(0x1 << 0)) | ((v as u32 & 0x1) << 0);
    }
    #[inline(always)]
    pub fn lastSelectedSlot(&self) -> u32 {
        ((self.bits_8 as u32 >> 1) & 0x7) as u32
    }
    #[inline(always)]
    pub fn set_lastSelectedSlot(&mut self, v: u32) {
        self.bits_8 = (self.bits_8 & !(0x7 << 1)) | ((v as u32 & 0x7) << 1);
    }
    #[inline(always)]
    pub fn spriteIdConfirmPokeball(&self) -> u32 {
        ((self.bits_8 as u32 >> 4) & 0x7f) as u32
    }
    #[inline(always)]
    pub fn set_spriteIdConfirmPokeball(&mut self, v: u32) {
        self.bits_8 = (self.bits_8 & !(0x7f << 4)) | ((v as u32 & 0x7f) << 4);
    }
    #[inline(always)]
    pub fn spriteIdCancelPokeball(&self) -> u32 {
        ((self.bits_8 as u32 >> 11) & 0x7f) as u32
    }
    #[inline(always)]
    pub fn set_spriteIdCancelPokeball(&mut self, v: u32) {
        self.bits_8 = (self.bits_8 & !(0x7f << 11)) | ((v as u32 & 0x7f) << 11);
    }
    #[inline(always)]
    pub fn messageId(&self) -> u32 {
        ((self.bits_8 as u32 >> 18) & 0x3fff) as u32
    }
    #[inline(always)]
    pub fn set_messageId(&mut self, v: u32) {
        self.bits_8 = (self.bits_8 & !(0x3fff << 18)) | ((v as u32 & 0x3fff) << 18);
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
    pub blitFunc: Option<unsafe extern "C" fn(u8, u8, u8, u8, u8, u8)>,
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
    pub func: Option<unsafe extern "C" fn(u8)>,
}

unsafe impl Sync for sCursorOptions_0_t {}

/// `__typeof__(sFieldMoveCursorCallbacks[0])`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct sFieldMoveCursorCallbacks_0_t {
    pub fieldMoveFunc: Option<unsafe extern "C" fn() -> u8>,
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

#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sPartyMenuInternal: *mut PartyMenuInternal = null_mut();
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gPartyMenu: PartyMenu = unsafe { zeroed() };
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sPartyMenuBoxes: *mut PartyMenuBox = null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sPartyBgGfxTilemap: *mut u8 = null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sPartyBgTilemapBuffer: *mut u8 = null_mut();
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gPartyMenuUseExitCallback: u8 = 0;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gSelectedMonPartyId: u8 = 0;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gPostMenuFieldCallback: Option<unsafe extern "C" fn()> = None;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sSlot1TilemapBuffer: *mut u16 = null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sSlot2TilemapBuffer: *mut u16 = null_mut();
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gSelectedOrderFromParty: Aligned<CArray<u8, 4>> = Aligned(unsafe { zeroed() });
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sPartyMenuItemId: u16 = 0;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sUnused: u16 = 0;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gBattlePartyCurrentOrder: Aligned<CArray<u8, 3>> = Aligned(unsafe { zeroed() });
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gItemUseCB: Option<unsafe extern "C" fn(u8, Option<unsafe extern "C" fn(u8)>)> =
    None;

unsafe extern "C" {
    static mut gBattleStruct: *mut BattleStruct;
    static mut gBattleTypeFlags: u32;
    static mut gBattlerInMenuId: u8;
    static mut gBattlerPartyIndexes: CArray<u16, 4>;
    static mut gBattlersCount: u8;
    static mut gCB2_AfterEvolution: Option<unsafe extern "C" fn()>;
    static mut gContestMonPartyIndex: u8;
    static mut gEnemyParty: CArray<Pokemon, 6>;
    static mut gFieldCallback: Option<unsafe extern "C" fn()>;
    static mut gFieldCallback2: Option<unsafe extern "C" fn() -> u8>;
    static mut gFieldEffectArguments: CArray<i32, 8>;
    static gFrontierBannedSpecies: CArray<u16, 0>;
    static gItemEffectTable: CArray<*mut u8, 0>;
    static gJPText_AreYouSureYouWantToSpinTradeMon: CArray<u8, 0>;
    static mut gLastViewedMonIndex: u8;
    static mut gMain: Main;
    static mut gMapHeader: MapHeader;
    static gMenuText_Confirm: CArray<u8, 0>;
    static gMoveNames: CArray<CArray<u8, 13>, 355>;
    static mut gMoveToLearn: u16;
    static mut gMultiPartnerParty: CArray<MultiPartnerMenuPokemon, 3>;
    static gPPUpGetMask: CArray<u8, 0>;
    static mut gPaletteFade: PaletteFadeControl;
    static gPartyMenuBg_Gfx: CArray<u32, 0>;
    static gPartyMenuBg_Pal: CArray<u32, 0>;
    static gPartyMenuBg_Tilemap: CArray<u32, 0>;
    static mut gPlayerPCItemPageInfo: PlayerPCItemPageStruct;
    static mut gPlayerParty: CArray<Pokemon, 6>;
    static mut gPlayerPartyCount: u8;
    static mut gPlttBufferFaded: CArray<u16, 512>;
    static mut gPlttBufferUnfaded: CArray<u16, 512>;
    static mut gPyramidBagMenuState: PyramidBagMenuState;
    static mut gRfuPartnerCompatibilityData: RfuGameCompatibilityData;
    static mut gSaveBlock1Ptr: *mut SaveBlock1;
    static mut gSpecialVar_0x8004: u16;
    static mut gSpecialVar_0x8005: u16;
    static mut gSpecialVar_ItemId: u16;
    static mut gSpecialVar_Result: u16;
    static gSpeciesNames: CArray<CArray<u8, 11>, 0>;
    static mut gSprites: CArray<Sprite, 65>;
    static gStandardMenuPalette: CArray<u16, 0>;
    static mut gStringVar1: CArray<u8, 256>;
    static mut gStringVar2: CArray<u8, 256>;
    static mut gStringVar4: CArray<u8, 1000>;
    static mut gTasks: CArray<Task, 0>;
    static mut gTextFlags: TextFlags;
    static gText_12PoofForgotMove: CArray<u8, 0>;
    static gText_Attack3: CArray<u8, 0>;
    static gText_BagFullCouldNotRemoveItem: CArray<u8, 0>;
    static gText_Cancel: CArray<u8, 0>;
    static gText_Cancel2: CArray<u8, 0>;
    static gText_CancelBattle: CArray<u8, 0>;
    static gText_CancelChallenge: CArray<u8, 0>;
    static gText_CancelParticipation: CArray<u8, 0>;
    static gText_CantSwitchWithAlly: CArray<u8, 0>;
    static gText_CantUseUntilNewBadge: CArray<u8, 0>;
    static gText_Defense3: CArray<u8, 0>;
    static gText_EggCantBattle: CArray<u8, 0>;
    static gText_EggCantBeTradedNow: CArray<u8, 0>;
    static gText_EscapeFromHere: CArray<u8, 0>;
    static gText_FemaleSymbol: CArray<u8, 0>;
    static gText_HP3: CArray<u8, 0>;
    static gText_ItemThrownAway: CArray<u8, 0>;
    static gText_LevelSymbol: CArray<u8, 0>;
    static gText_MailMessageWillBeLost: CArray<u8, 0>;
    static gText_MailSentToPC: CArray<u8, 0>;
    static gText_MailTakenFromPkmn: CArray<u8, 0>;
    static gText_MailTransferredFromMailbox: CArray<u8, 0>;
    static gText_MaleSymbol: CArray<u8, 0>;
    static gText_MoveNotLearned: CArray<u8, 0>;
    static gText_MovesPPIncreased: CArray<u8, 0>;
    static gText_NoMoreThanVar1Pkmn: CArray<u8, 0>;
    static gText_OnlyPkmnForBattle: CArray<u8, 0>;
    static gText_PCMailboxFull: CArray<u8, 0>;
    static gText_PPWasRestored: CArray<u8, 0>;
    static gText_PauseUntilPress: CArray<u8, 0>;
    static gText_PkmnAdoresBaseVar2Fell: CArray<u8, 0>;
    static gText_PkmnAlreadyHoldingItemSwitch: CArray<u8, 0>;
    static gText_PkmnAlreadyInBattle: CArray<u8, 0>;
    static gText_PkmnAlreadyKnows: CArray<u8, 0>;
    static gText_PkmnAlreadySelected: CArray<u8, 0>;
    static gText_PkmnBaseVar2StatIncreased: CArray<u8, 0>;
    static gText_PkmnBecameHealthy: CArray<u8, 0>;
    static gText_PkmnBurnHealed: CArray<u8, 0>;
    static gText_PkmnCantBeTradedNow: CArray<u8, 0>;
    static gText_PkmnCantLearnMove: CArray<u8, 0>;
    static gText_PkmnCantParticipate: CArray<u8, 0>;
    static gText_PkmnCantSwitchOut: CArray<u8, 0>;
    static gText_PkmnCuredOfParalysis: CArray<u8, 0>;
    static gText_PkmnCuredOfPoison: CArray<u8, 0>;
    static gText_PkmnElevatedToLvVar2: CArray<u8, 0>;
    static gText_PkmnFriendlyBaseVar2CantFall: CArray<u8, 0>;
    static gText_PkmnFriendlyBaseVar2Fell: CArray<u8, 0>;
    static gText_PkmnGotOverInfatuation: CArray<u8, 0>;
    static gText_PkmnHPRestoredByVar2: CArray<u8, 0>;
    static gText_PkmnHasNoEnergy: CArray<u8, 0>;
    static gText_PkmnHoldingItemCantHoldMail: CArray<u8, 0>;
    static gText_PkmnLearnedMove3: CArray<u8, 0>;
    static gText_PkmnNeedsToReplaceMove: CArray<u8, 0>;
    static gText_PkmnNotHolding: CArray<u8, 0>;
    static gText_PkmnSnappedOutOfConfusion: CArray<u8, 0>;
    static gText_PkmnThawedOut: CArray<u8, 0>;
    static gText_PkmnWasGivenItem: CArray<u8, 0>;
    static gText_PkmnWokeUp2: CArray<u8, 0>;
    static gText_ReceivedItemFromPkmn: CArray<u8, 0>;
    static gText_RemoveMailBeforeItem: CArray<u8, 0>;
    static gText_ReturnToHealingSpot: CArray<u8, 0>;
    static gText_ReturnToWaitingRoom: CArray<u8, 0>;
    static gText_SendMailToPC: CArray<u8, 0>;
    static gText_Slash: CArray<u8, 0>;
    static gText_SpAtk3: CArray<u8, 0>;
    static gText_SpDef3: CArray<u8, 0>;
    static gText_Speed2: CArray<u8, 0>;
    static gText_StopLearningMove2: CArray<u8, 0>;
    static gText_SwitchedPkmnItem: CArray<u8, 0>;
    static gText_ThrowAwayItem: CArray<u8, 0>;
    static gText_WhichMoveToForget: CArray<u8, 0>;
    static gText_WontHaveEffect: CArray<u8, 0>;
    static mut gUnionRoomOfferedSpecies: u16;
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
        a6: Option<unsafe extern "C" fn(*mut TextPrinterTemplate, u16)>,
    ) -> u16;
    fn AddTextPrinterParameterized2(
        a0: u8,
        a1: u8,
        a2: *mut u8,
        a3: u8,
        a4: Option<unsafe extern "C" fn(*mut TextPrinterTemplate, u16)>,
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
    fn AddWindow(a0: *mut WindowTemplate) -> u16;
    fn AdjustFriendship(a0: *mut Pokemon, a1: u8);
    fn Alloc(a0: u32) -> *mut c_void;
    fn AllocZeroed(a0: u32) -> *mut c_void;
    fn AnimateSprites();
    fn AnyStorageMonWithMove(a0: u16) -> u32;
    fn AppendToList(a0: *mut u8, a1: *mut u8, a2: u8);
    fn BeginEvolutionScene(a0: *mut Pokemon, a1: u16, a2: u8, a3: u8);
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
    fn CanMonLearnTMHM(a0: *mut Pokemon, a1: u8) -> u32;
    fn CanRegisterMonForTradingBoard(a0: RfuGameCompatibilityData, a1: u16, a2: u16, a3: u8)
    -> i32;
    fn CanSpinTradeMon(a0: *mut Pokemon, a1: u16) -> i32;
    fn ChangeBgX(a0: u8, a1: i32, a2: u8) -> i32;
    fn CheckIfItemIsTMHMOrEvolutionStone(a0: u16) -> u8;
    fn CheckPartyPokerus(a0: *mut Pokemon, a1: u8) -> u8;
    fn ChooseMonForSoftboiled(a0: u8);
    fn CleanupOverworldWindowsAndTilemaps();
    fn ClearMail(a0: *mut Mail);
    fn ClearScheduledBgCopiesToVram();
    fn ClearStdWindowAndFrameToTransparent(a0: u8, a1: u8);
    fn ClearWindowTilemap(a0: u8);
    fn ConvertIntToDecimalStringN(a0: *mut u8, a1: i32, a2: i32, a3: u8) -> *mut u8;
    fn ConvertInternationalPlayerName(a0: *mut u8);
    fn CopyItemName(a0: u16, a1: *mut u8);
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
    );
    fn CopyToBgTilemapBufferRect_ChangePalette(
        a0: u8,
        a1: *mut c_void,
        a2: u8,
        a3: u8,
        a4: u8,
        a5: u8,
        a6: u8,
    );
    fn CopyToBufferFromBgTilemap(a0: u8, a1: *mut u16, a2: u8, a3: u8, a4: u8, a5: u8);
    fn CopyWindowToVram(a0: u8, a1: u8);
    fn CpuSet(a0: *mut c_void, a1: *mut c_void, a2: u32);
    fn CreateMonIcon(
        a0: u16,
        a1: Option<unsafe extern "C" fn(*mut Sprite)>,
        a2: i16,
        a3: i16,
        a4: u8,
        a5: u32,
        a6: u32,
    ) -> u8;
    fn CreateSprite(a0: *mut SpriteTemplate, a1: i16, a2: i16, a3: u8) -> u8;
    fn CreateTask(a0: Option<unsafe extern "C" fn(u8)>, a1: u8) -> u8;
    fn CreateYesNoMenu(a0: *mut WindowTemplate, a1: u16, a2: u8, a3: u8);
    fn CurrentBattlePyramidLocation() -> u8;
    fn DeactivateAllTextPrinters();
    fn DestroyTask(a0: u8);
    fn DoEasyChatScreen(a0: u8, a1: *mut u16, a2: Option<unsafe extern "C" fn()>, a3: u8);
    fn DoScheduledBgTilemapCopiesToVram();
    fn DrawLevelUpWindowPg1(a0: u16, a1: *mut u16, a2: *mut u16, a3: u8, a4: u8, a5: u8);
    fn DrawLevelUpWindowPg2(a0: u16, a1: *mut u16, a2: u8, a3: u8, a4: u8);
    fn DrawStdFrameWithCustomTileAndPalette(a0: u8, a1: u8, a2: u16, a3: u8);
    fn ExecuteTableBasedItemEffect(a0: *mut Pokemon, a1: u16, a2: u8, a3: u8) -> u8;
    fn FadeInFromBlack();
    fn FadeScreen(a0: u8, a1: i8);
    fn FieldCB_ContinueScriptHandleMusic();
    fn FieldEffectStart(a0: u8) -> u32;
    fn FillBgTilemapBufferRect(a0: u8, a1: u16, a2: u8, a3: u8, a4: u8, a5: u8, a6: u8);
    fn FillBgTilemapBufferRect_Palette0(a0: u8, a1: u16, a2: u8, a3: u8, a4: u8, a5: u8);
    fn FillWindowPixelBuffer(a0: u8, a1: u8);
    fn FillWindowPixelRect(a0: u8, a1: u8, a2: u16, a3: u16, a4: u16, a5: u16);
    fn FlagGet(a0: u16) -> u8;
    fn Free(a0: *mut c_void);
    fn FreeAllSpritePalettes();
    fn FreeAllWindowBuffers();
    fn FuncIsActiveTask(a0: Option<unsafe extern "C" fn(u8)>) -> u8;
    fn GetBattlerAtPosition(a0: u8) -> u8;
    fn GetBattlerSide(a0: u8) -> u8;
    fn GetContestEntryEligibility(a0: *mut Pokemon) -> u8;
    fn GetEvolutionTargetSpecies(a0: *mut Pokemon, a1: u8, a2: u16) -> u16;
    fn GetFontAttribute(a0: u8, a1: u8) -> u8;
    fn GetHPBarLevel(a0: i16, a1: i16) -> u8;
    fn GetHostRfuGameData() -> *mut RfuGameData;
    fn GetLRKeysPressedAndHeld() -> u8;
    fn GetMapNameGeneric(a0: *mut u8, a1: u16) -> *mut u8;
    fn GetMenuCursorDimensionByFont(a0: u8, a1: u8) -> u8;
    fn GetMonData2(a0: *mut Pokemon, a1: i32) -> u32;
    fn GetMonData3(a0: *mut Pokemon, a1: i32, a2: *mut u8) -> u32;
    fn GetMonGender(a0: *mut Pokemon) -> u8;
    fn GetMoveSlotToReplace() -> u8;
    fn GetNumberOfRelearnableMoves(a0: *mut Pokemon) -> u8;
    fn GetOverworldTextboxPalettePtr() -> *mut u16;
    fn GetPlayerFlankId() -> u8;
    fn GetPlayerTextSpeedDelay() -> u8;
    fn GetPocketByItemId(a0: u16) -> u8;
    fn GetScaledHPFraction(a0: i16, a1: i16, a2: u8) -> u8;
    fn GetStringCenterAlignXOffset(a0: i32, a1: *mut u8, a2: i32) -> i32;
    fn GetTrainerPartnerName() -> *mut u8;
    fn GetUnionRoomTradeMessageId(
        a0: RfuGameCompatibilityData,
        a1: RfuGameCompatibilityData,
        a2: u16,
        a3: u16,
        a4: u8,
        a5: u16,
        a6: u8,
    ) -> i32;
    fn GetWindowAttribute(a0: u8, a1: u8) -> u32;
    fn GetXYCoordsOneStepInFrontOfPlayer(a0: *mut i16, a1: *mut i16);
    fn GiveMailToMon(a0: *mut Pokemon, a1: *mut Mail) -> u8;
    fn GiveMailToMonByItemId(a0: *mut Pokemon, a1: u16) -> u8;
    fn GiveMoveToMon(a0: *mut Pokemon, a1: u16) -> u16;
    fn GoToBagMenu(a0: u8, a1: u8, a2: Option<unsafe extern "C" fn()>);
    fn GoToBattlePyramidBagMenu(a0: u8, a1: Option<unsafe extern "C" fn()>);
    fn HandleBattleLowHpMusicChange();
    fn InBattlePike() -> u8;
    fn InMultiPartnerRoom() -> u8;
    fn InUnionRoom() -> u32;
    fn InitBgsFromTemplates(a0: u8, a1: *mut BgTemplate, a2: u8);
    fn InitMenuInUpperLeftCorner(a0: u8, a1: u8, a2: u8, a3: u8) -> u8;
    fn InitMenuInUpperLeftCornerNormal(a0: u8, a1: u8, a2: u8) -> u8;
    fn InitWindows(a0: *mut WindowTemplate) -> u16;
    fn IsDma3ManagerBusyWithBgCopy() -> u8;
    fn IsDoubleBattle() -> u8;
    fn IsFanfareTaskInactive() -> u8;
    fn IsPlayerFacingSurfableFishableWater() -> u8;
    fn IsPlayerSurfingNorth() -> u8;
    fn IsSpeciesAllowedInPokemonJump(a0: u16) -> u32;
    fn IsWeatherNotFadingIn() -> u8;
    fn ItemIsMail(a0: u16) -> u8;
    fn LZDecompressWram(a0: *mut u32, a1: *mut c_void);
    fn LoadBgTiles(a0: u8, a1: *mut c_void, a2: u16, a3: u16) -> u16;
    fn LoadCompressedPalette(a0: *mut u32, a1: u16, a2: u16);
    fn LoadCompressedSpritePalette(a0: *mut CompressedSpritePalette);
    fn LoadCompressedSpriteSheet(a0: *mut CompressedSpriteSheet) -> u16;
    fn LoadMonIconPalettes();
    fn LoadOam();
    fn LoadPalette(a0: *mut c_void, a1: u16, a2: u16);
    fn LoadSpritePalette(a0: *mut SpritePalette) -> u8;
    fn LoadSpriteSheet(a0: *mut SpriteSheet) -> u16;
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
    fn MonTryLearningNewMove(a0: *mut Pokemon, a1: u8) -> u16;
    fn Overworld_GetMapHeaderByGroupAndId(a0: u16, a1: u16) -> *mut MapHeader;
    fn Overworld_MapTypeAllowsTeleportAndFly(a0: u8) -> u8;
    fn PartyHasMonWithSurf() -> u8;
    fn PlayFanfare(a0: u16);
    fn PlayFanfareByFanfareNum(a0: u8);
    fn PlaySE(a0: u16);
    fn ProcessMenuInput_other() -> i8;
    fn ProcessSpriteCopyRequests();
    fn PutWindowTilemap(a0: u8);
    fn ReadMail(a0: *mut Mail, a1: Option<unsafe extern "C" fn()>, a2: u8);
    fn RemoveBagItem(a0: u16, a1: u16) -> u8;
    fn RemoveMonPPBonus(a0: *mut Pokemon, a1: u8);
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
    fn SetBgTilemapBuffer(a0: u8, a1: *mut c_void);
    fn SetBgTilemapPalette(a0: u8, a1: u8, a2: u8, a3: u8, a4: u8, a5: u8);
    fn SetGpuReg(a0: u8, a1: u16);
    fn SetMainCallback2(a0: Option<unsafe extern "C" fn()>);
    fn SetMonData(a0: *mut Pokemon, a1: i32, a2: *mut c_void);
    fn SetMonMoveSlot(a0: *mut Pokemon, a1: u16, a2: u8);
    fn SetMonPreventsSwitchingString();
    fn SetPartyHPBarSprite(a0: *mut Sprite, a1: u8);
    fn SetTaskFuncWithFollowupFunc(
        a0: u8,
        a1: Option<unsafe extern "C" fn(u8)>,
        a2: Option<unsafe extern "C" fn(u8)>,
    );
    fn SetVBlankCallback(a0: Option<unsafe extern "C" fn()>);
    fn SetVBlankHBlankCallbacksToNull();
    fn SetWindowTemplateFields(
        a0: *mut WindowTemplate,
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
        a1: *mut c_void,
        a2: u8,
        a3: u8,
        a4: Option<unsafe extern "C" fn()>,
    );
    fn ShowSelectMovePokemonSummaryScreen(
        a0: *mut Pokemon,
        a1: u8,
        a2: u8,
        a3: Option<unsafe extern "C" fn()>,
        a4: u16,
    );
    fn SpriteCB_MonIcon(a0: *mut Sprite);
    fn StartSpriteAnim(a0: *mut Sprite, a1: u8);
    fn StringAppend(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn StringCompare(a0: *mut u8, a1: *mut u8) -> i32;
    fn StringCopy(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn StringExpandPlaceholders(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn StringGet_Nickname(a0: *mut u8) -> *mut u8;
    fn SwitchTaskToFollowupFunc(a0: u8);
    fn TakeMailFromMon(a0: *mut Pokemon);
    fn TakeMailFromMonAndSave(a0: *mut Pokemon) -> u8;
    fn Task_TryUseSoftboiledOnPartyMon(a0: u8);
    fn TestPlayerAvatarFlags(a0: u8) -> u8;
    fn TransferPlttBuffer();
    fn TrySetDiveWarp() -> u8;
    fn UnlockPlayerFieldControls();
    fn UpdateMonIconFrame(a0: *mut Sprite) -> u8;
    fn UpdatePaletteFade() -> u8;
    fn VarGet(a0: u16) -> u16;
    fn WaitFanfare(a0: u8) -> u8;
    fn malloc_and_decompress(a0: *mut c_void, a1: *mut u32) -> *mut c_void;
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
    let mut i: u16 = 0;
    ResetPartyMenu();
    sPartyMenuInternal = Alloc(568) as *mut PartyMenuInternal;
    if sPartyMenuInternal.is_null() {
        SetMainCallback2(callback);
    } else {
        gPartyMenu.set_menuType(menuType);
        gPartyMenu.exitCallback = callback;
        gPartyMenu.action = partyAction;
        (*sPartyMenuInternal).set_messageId(messageId as u32);
        (*sPartyMenuInternal).task = task;
        (*sPartyMenuInternal).exitCallback = None;
        (*sPartyMenuInternal).set_lastSelectedSlot(0);
        (*sPartyMenuInternal).set_spriteIdConfirmPokeball(0x7F);
        (*sPartyMenuInternal).set_spriteIdCancelPokeball(0x7F);
        if menuType == PARTY_MENU_TYPE_CHOOSE_HALF {
            (*sPartyMenuInternal).set_chooseHalf(TRUE as u32);
        } else {
            (*sPartyMenuInternal).set_chooseHalf(FALSE as u32);
        }
        if layout != KEEP_PARTY_LAYOUT {
            gPartyMenu.set_layout(layout);
        }
        i = 0;
        while i < 16 {
            (*sPartyMenuInternal).data[i] = 0;
            i += 1;
        }
        i = 0;
        while i < 3 {
            (*sPartyMenuInternal).windowId[i] = WINDOW_NONE;
            i += 1;
        }
        if keepCursorPos == 0 {
            gPartyMenu.slotId = 0;
        } else if gPartyMenu.slotId > 5
            || GetMonData2(&raw mut gPlayerParty[gPartyMenu.slotId], MON_DATA_SPECIES)
                == SPECIES_NONE as u32
        {
            gPartyMenu.slotId = 0;
        }
        gTextFlags.set_autoScroll(0);
        CalculatePlayerPartyCount();
        SetMainCallback2(Some(CB2_InitPartyMenu));
    }
}
pub(crate) unsafe extern "C" fn CB2_UpdatePartyMenu() {
    RunTasks();
    AnimateSprites();
    BuildOamBuffer();
    DoScheduledBgTilemapCopiesToVram();
    UpdatePaletteFade();
}
pub(crate) unsafe extern "C" fn VBlankCB_PartyMenu() {
    LoadOam();
    ProcessSpriteCopyRequests();
    TransferPlttBuffer();
}
pub(crate) unsafe extern "C" fn CB2_InitPartyMenu() {
    loop {
        if MenuHelpers_ShouldWaitForLinkRecv() == TRUE
            || ShowPartyMenu() == TRUE
            || MenuHelpers_IsLinkActive() == TRUE
        {
            break;
        }
    }
}
pub(crate) unsafe extern "C" fn ShowPartyMenu() -> u8 {
    match gMain.state {
        0 => {
            SetVBlankHBlankCallbacksToNull();
            ResetVramOamAndBgCntRegs();
            ClearScheduledBgCopiesToVram();
            gMain.state += 1;
        }
        1 => {
            ScanlineEffect_Stop();
            gMain.state += 1;
        }
        2 => {
            ResetPaletteFade();
            gPaletteFade.set_bufferTransferDisabled(TRUE as u16);
            gMain.state += 1;
        }
        3 => {
            ResetSpriteData();
            gMain.state += 1;
        }
        4 => {
            FreeAllSpritePalettes();
            gMain.state += 1;
        }
        5 => {
            if MenuHelpers_IsLinkActive() == 0 {
                ResetTasks();
            }
            gMain.state += 1;
        }
        6 => {
            SetPartyMonsAllowedInMinigame();
            gMain.state += 1;
        }
        7 => {
            if AllocPartyMenuBg() == 0 {
                ExitPartyMenu();
                return TRUE;
            } else {
                (*sPartyMenuInternal).data[0] = 0;
                gMain.state += 1;
            }
        }
        8 => {
            if AllocPartyMenuBgGfx() != 0 {
                gMain.state += 1;
            }
        }
        9 => {
            InitPartyMenuWindows(gPartyMenu.layout());
            gMain.state += 1;
        }
        10 => {
            InitPartyMenuBoxes(gPartyMenu.layout());
            (*sPartyMenuInternal).data[0] = 0;
            gMain.state += 1;
        }
        11 => {
            LoadHeldItemIcons();
            gMain.state += 1;
        }
        12 => {
            LoadPartyMenuPokeballGfx();
            gMain.state += 1;
        }
        13 => {
            LoadPartyMenuAilmentGfx();
            gMain.state += 1;
        }
        14 => {
            LoadMonIconPalettes();
            gMain.state += 1;
        }
        15 => {
            if CreatePartyMonSpritesLoop() != 0 {
                (*sPartyMenuInternal).data[0] = 0;
                gMain.state += 1;
            }
        }
        16 => {
            if RenderPartyMenuBoxes() != 0 {
                (*sPartyMenuInternal).data[0] = 0;
                gMain.state += 1;
            }
        }
        17 => {
            CreateCancelConfirmPokeballSprites();
            gMain.state += 1;
        }
        18 => {
            CreateCancelConfirmWindows((*sPartyMenuInternal).chooseHalf() as u8);
            gMain.state += 1;
        }
        19 => {
            gMain.state += 1;
        }
        20 => {
            CreateTask((*sPartyMenuInternal).task, 0);
            DisplayPartyMenuStdMessage((*sPartyMenuInternal).messageId());
            gMain.state += 1;
        }
        21 => {
            BlendPalettes(PALETTES_ALL, 16, 0);
            gPaletteFade.set_bufferTransferDisabled(FALSE as u16);
            gMain.state += 1;
        }
        22 => {
            BeginNormalPaletteFade(PALETTES_ALL, 0, 16, 0, 0);
            gMain.state += 1;
        }
        _ => {
            SetVBlankCallback(Some(VBlankCB_PartyMenu));
            SetMainCallback2(Some(CB2_UpdatePartyMenu));
            return TRUE;
        }
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn ExitPartyMenu() {
    BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 16, 0);
    CreateTask(Some(Task_ExitPartyMenu), 0);
    SetVBlankCallback(Some(VBlankCB_PartyMenu));
    SetMainCallback2(Some(CB2_UpdatePartyMenu));
}
pub(crate) unsafe extern "C" fn Task_ExitPartyMenu(taskId: u8) {
    if gPaletteFade.active() == 0 {
        SetMainCallback2(gPartyMenu.exitCallback);
        FreePartyPointers();
        DestroyTask(taskId);
    }
}
pub(crate) unsafe extern "C" fn ResetPartyMenu() {
    sPartyMenuInternal = null_mut();
    sPartyBgTilemapBuffer = null_mut();
    sPartyMenuBoxes = null_mut();
    sPartyBgGfxTilemap = null_mut();
}
pub(crate) unsafe extern "C" fn AllocPartyMenuBg() -> u8 {
    sPartyBgTilemapBuffer = Alloc(0x800) as *mut u8;
    if sPartyBgTilemapBuffer.is_null() {
        return FALSE;
    }
    memset(sPartyBgTilemapBuffer, 0, 0x800);
    ResetBgsAndClearDma3BusyFlags(0);
    InitBgsFromTemplates(0, sPartyMenuBgTemplates.as_ptr().cast_mut(), 3);
    SetBgTilemapBuffer(1, sPartyBgTilemapBuffer as *mut c_void);
    ResetAllBgsCoordinates();
    ScheduleBgCopyTilemapToVram(1);
    SetGpuReg(REG_OFFSET_DISPCNT, 4160);
    SetGpuReg(REG_OFFSET_BLDCNT, 0);
    ShowBg(0);
    ShowBg(1);
    ShowBg(2);
    return TRUE;
}
pub(crate) unsafe extern "C" fn AllocPartyMenuBgGfx() -> u8 {
    let mut sizeout: u32 = 0;
    match (*sPartyMenuInternal).data[0] {
        0 => {
            sPartyBgGfxTilemap = malloc_and_decompress(
                gPartyMenuBg_Gfx.as_ptr().cast_mut() as *mut c_void,
                &raw mut sizeout,
            ) as *mut u8;
            LoadBgTiles(1, sPartyBgGfxTilemap as *mut c_void, sizeout as u16, 0);
            (*sPartyMenuInternal).data[0] += 1;
        }
        1 => {
            if IsDma3ManagerBusyWithBgCopy() == 0 {
                LZDecompressWram(
                    gPartyMenuBg_Tilemap.as_ptr().cast_mut(),
                    sPartyBgTilemapBuffer as *mut c_void,
                );
                (*sPartyMenuInternal).data[0] += 1;
            }
        }
        2 => {
            LoadCompressedPalette(gPartyMenuBg_Pal.as_ptr().cast_mut(), 0, 352);
            CpuSet(
                gPlttBufferUnfaded.as_mut_ptr() as *mut c_void,
                (*sPartyMenuInternal).palBuffer.as_mut_ptr() as *mut c_void,
                176,
            );
            (*sPartyMenuInternal).data[0] += 1;
        }
        3 => {
            PartyPaletteBufferCopy(4);
            (*sPartyMenuInternal).data[0] += 1;
        }
        4 => {
            PartyPaletteBufferCopy(5);
            (*sPartyMenuInternal).data[0] += 1;
        }
        5 => {
            PartyPaletteBufferCopy(6);
            (*sPartyMenuInternal).data[0] += 1;
        }
        6 => {
            PartyPaletteBufferCopy(7);
            (*sPartyMenuInternal).data[0] += 1;
        }
        7 => {
            PartyPaletteBufferCopy(8);
            (*sPartyMenuInternal).data[0] += 1;
        }
        _ => {
            return TRUE;
        }
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn PartyPaletteBufferCopy(palNum: u8) {
    let mut offset: u8 = palNum * 16;
    CpuSet(
        &raw mut gPlttBufferUnfaded[48] as *mut c_void,
        &raw mut gPlttBufferUnfaded[offset] as *mut c_void,
        16,
    );
    CpuSet(
        &raw mut gPlttBufferUnfaded[48] as *mut c_void,
        &raw mut gPlttBufferFaded[offset] as *mut c_void,
        16,
    );
}
pub(crate) unsafe extern "C" fn FreePartyPointers() {
    if !sPartyMenuInternal.is_null() {
        Free(sPartyMenuInternal as *mut c_void);
    }
    if !sPartyBgTilemapBuffer.is_null() {
        Free(sPartyBgTilemapBuffer as *mut c_void);
    }
    if !sPartyBgGfxTilemap.is_null() {
        Free(sPartyBgGfxTilemap as *mut c_void);
    }
    if !sPartyMenuBoxes.is_null() {
        Free(sPartyMenuBoxes as *mut c_void);
    }
    FreeAllWindowBuffers();
}
pub(crate) unsafe extern "C" fn InitPartyMenuBoxes(layout: u8) {
    let mut i: u8 = 0;
    sPartyMenuBoxes = Alloc(96) as *mut PartyMenuBox;
    i = 0;
    while i < PARTY_SIZE as u8 {
        (*sPartyMenuBoxes.at(i)).infoRects = (&raw const sPartyBoxInfoRects[1]).cast_mut();
        (*sPartyMenuBoxes.at(i)).spriteCoords =
            sPartyMenuSpriteCoords[layout][i].as_ptr().cast_mut();
        (*sPartyMenuBoxes.at(i)).windowId = i;
        (*sPartyMenuBoxes.at(i)).monSpriteId = SPRITE_NONE;
        (*sPartyMenuBoxes.at(i)).itemSpriteId = SPRITE_NONE;
        (*sPartyMenuBoxes.at(i)).pokeballSpriteId = SPRITE_NONE;
        (*sPartyMenuBoxes.at(i)).statusSpriteId = SPRITE_NONE;
        i += 1;
    }
    (*sPartyMenuBoxes).infoRects = (&raw const sPartyBoxInfoRects[0]).cast_mut();
    if layout == PARTY_LAYOUT_MULTI_SHOWCASE {
        (*sPartyMenuBoxes.at(3)).infoRects = (&raw const sPartyBoxInfoRects[0]).cast_mut();
    } else if layout != PARTY_LAYOUT_SINGLE {
        (*sPartyMenuBoxes.at(1)).infoRects = (&raw const sPartyBoxInfoRects[0]).cast_mut();
    }
}
pub(crate) unsafe extern "C" fn RenderPartyMenuBox(slot: u8) {
    if gPartyMenu.menuType() == PARTY_MENU_TYPE_MULTI_SHOWCASE && slot >= MULTI_PARTY_SIZE as u8 {
        DisplayPartyPokemonDataForMultiBattle(slot);
        if gMultiPartnerParty[slot as i32 - MULTI_PARTY_SIZE].species == SPECIES_NONE {
            LoadPartyBoxPalette(sPartyMenuBoxes.at(slot), PARTY_PAL_NO_MON);
        } else {
            LoadPartyBoxPalette(sPartyMenuBoxes.at(slot), PARTY_PAL_MULTI_ALT);
        }
        CopyWindowToVram((*sPartyMenuBoxes.at(slot)).windowId, COPYWIN_GFX);
        PutWindowTilemap((*sPartyMenuBoxes.at(slot)).windowId);
        ScheduleBgCopyTilemapToVram(2);
    } else {
        if GetMonData2(&raw mut gPlayerParty[slot], MON_DATA_SPECIES) == SPECIES_NONE as u32 {
            DrawEmptySlot((*sPartyMenuBoxes.at(slot)).windowId);
            LoadPartyBoxPalette(sPartyMenuBoxes.at(slot), PARTY_PAL_NO_MON);
            CopyWindowToVram((*sPartyMenuBoxes.at(slot)).windowId, COPYWIN_GFX);
        } else {
            if gPartyMenu.menuType() == PARTY_MENU_TYPE_MOVE_RELEARNER {
                DisplayPartyPokemonDataForRelearner(slot);
            } else if gPartyMenu.menuType() == PARTY_MENU_TYPE_CONTEST {
                DisplayPartyPokemonDataForContest(slot);
            } else if gPartyMenu.menuType() == PARTY_MENU_TYPE_CHOOSE_HALF {
                DisplayPartyPokemonDataForChooseHalf(slot);
            } else if gPartyMenu.menuType() == PARTY_MENU_TYPE_MINIGAME {
                DisplayPartyPokemonDataForWirelessMinigame(slot);
            } else if gPartyMenu.menuType() == PARTY_MENU_TYPE_STORE_PYRAMID_HELD_ITEMS {
                DisplayPartyPokemonDataForBattlePyramidHeldItem(slot);
            } else if DisplayPartyPokemonDataForMoveTutorOrEvolutionItem(slot) == 0 {
                DisplayPartyPokemonData(slot);
            }
            if gPartyMenu.menuType() == PARTY_MENU_TYPE_MULTI_SHOWCASE {
                AnimatePartySlot(slot, 0);
            } else if gPartyMenu.slotId as i32 == slot as i32 {
                AnimatePartySlot(slot, 1);
            } else {
                AnimatePartySlot(slot, 0);
            }
        }
        PutWindowTilemap((*sPartyMenuBoxes.at(slot)).windowId);
        ScheduleBgCopyTilemapToVram(0);
    }
}
pub(crate) unsafe extern "C" fn DisplayPartyPokemonData(slot: u8) {
    if GetMonData2(&raw mut gPlayerParty[slot], MON_DATA_IS_EGG) != 0 {
        (*(*sPartyMenuBoxes.at(slot)).infoRects)
            .blitFunc
            .unwrap_unchecked()((*sPartyMenuBoxes.at(slot)).windowId, 0, 0, 0, 0, TRUE);
        DisplayPartyPokemonNickname(&raw mut gPlayerParty[slot], sPartyMenuBoxes.at(slot), 0);
    } else {
        (*(*sPartyMenuBoxes.at(slot)).infoRects)
            .blitFunc
            .unwrap_unchecked()((*sPartyMenuBoxes.at(slot)).windowId, 0, 0, 0, 0, 0);
        DisplayPartyPokemonNickname(&raw mut gPlayerParty[slot], sPartyMenuBoxes.at(slot), 0);
        DisplayPartyPokemonLevelCheck(&raw mut gPlayerParty[slot], sPartyMenuBoxes.at(slot), 0);
        DisplayPartyPokemonGenderNidoranCheck(
            &raw mut gPlayerParty[slot],
            sPartyMenuBoxes.at(slot),
            0,
        );
        DisplayPartyPokemonHPCheck(&raw mut gPlayerParty[slot], sPartyMenuBoxes.at(slot), 0);
        DisplayPartyPokemonMaxHPCheck(&raw mut gPlayerParty[slot], sPartyMenuBoxes.at(slot), 0);
        DisplayPartyPokemonHPBarCheck(&raw mut gPlayerParty[slot], sPartyMenuBoxes.at(slot));
    }
}
pub(crate) unsafe extern "C" fn DisplayPartyPokemonDescriptionData(slot: u8, stringID: u8) {
    let mut mon: *mut Pokemon = &raw mut gPlayerParty[slot];
    (*(*sPartyMenuBoxes.at(slot)).infoRects)
        .blitFunc
        .unwrap_unchecked()((*sPartyMenuBoxes.at(slot)).windowId, 0, 0, 0, 0, TRUE);
    DisplayPartyPokemonNickname(mon, sPartyMenuBoxes.at(slot), 0);
    if GetMonData2(mon, MON_DATA_IS_EGG) == 0 {
        DisplayPartyPokemonLevelCheck(mon, sPartyMenuBoxes.at(slot), 0);
        DisplayPartyPokemonGenderNidoranCheck(mon, sPartyMenuBoxes.at(slot), 0);
    }
    DisplayPartyPokemonDescriptionText(stringID, sPartyMenuBoxes.at(slot), 0);
}
pub(crate) unsafe extern "C" fn DisplayPartyPokemonDataForChooseHalf(slot: u8) {
    let mut i: u8 = 0;
    let mut mon: *mut Pokemon = &raw mut gPlayerParty[slot];
    let mut order: *mut u8 = gSelectedOrderFromParty.as_mut_ptr();
    if GetBattleEntryEligibility(mon) == 0 {
        DisplayPartyPokemonDescriptionData(slot, PARTYBOX_DESC_NOT_ABLE);
        return;
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
pub(crate) unsafe extern "C" fn DisplayPartyPokemonDataForContest(slot: u8) {
    match GetContestEntryEligibility(&raw mut gPlayerParty[slot]) {
        CANT_ENTER_CONTEST | CANT_ENTER_CONTEST_EGG | CANT_ENTER_CONTEST_FAINTED => {
            DisplayPartyPokemonDescriptionData(slot, PARTYBOX_DESC_NOT_ABLE);
        }
        CAN_ENTER_CONTEST_EQUAL_RANK | CAN_ENTER_CONTEST_HIGH_RANK => {
            DisplayPartyPokemonDescriptionData(slot, PARTYBOX_DESC_ABLE);
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn DisplayPartyPokemonDataForRelearner(slot: u8) {
    if GetNumberOfRelearnableMoves(&raw mut gPlayerParty[slot]) == 0 {
        DisplayPartyPokemonDescriptionData(slot, PARTYBOX_DESC_NOT_ABLE_2);
    } else {
        DisplayPartyPokemonDescriptionData(slot, PARTYBOX_DESC_ABLE_2);
    }
}
pub(crate) unsafe extern "C" fn DisplayPartyPokemonDataForWirelessMinigame(slot: u8) {
    if IsMonAllowedInMinigame(slot) == TRUE {
        DisplayPartyPokemonDescriptionData(slot, PARTYBOX_DESC_ABLE);
    } else {
        DisplayPartyPokemonDescriptionData(slot, PARTYBOX_DESC_NOT_ABLE);
    }
}
pub(crate) unsafe extern "C" fn DisplayPartyPokemonDataForBattlePyramidHeldItem(slot: u8) {
    if GetMonData2(&raw mut gPlayerParty[slot], MON_DATA_HELD_ITEM) != 0 {
        DisplayPartyPokemonDescriptionData(slot, PARTYBOX_DESC_HAVE);
    } else {
        DisplayPartyPokemonDescriptionData(slot, PARTYBOX_DESC_DONT_HAVE);
    }
}
pub(crate) unsafe extern "C" fn DisplayPartyPokemonDataForMoveTutorOrEvolutionItem(slot: u8) -> u8 {
    let mut currentPokemon: *mut Pokemon = &raw mut gPlayerParty[slot];
    let mut item: u16 = gSpecialVar_ItemId;
    if gPartyMenu.action == PARTY_ACTION_MOVE_TUTOR {
        gSpecialVar_Result = FALSE as u16;
        DisplayPartyPokemonDataToTeachMove(slot, 0, gSpecialVar_0x8005 as u8);
    } else {
        if gPartyMenu.action != PARTY_ACTION_USE_ITEM {
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
    return TRUE;
}
pub(crate) unsafe extern "C" fn DisplayPartyPokemonDataToTeachMove(slot: u8, item: u16, tutor: u8) {
    match CanMonLearnTMTutor(&raw mut gPlayerParty[slot], item, tutor) {
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
pub(crate) unsafe extern "C" fn DisplayPartyPokemonDataForMultiBattle(slot: u8) {
    let mut menuBox: *mut PartyMenuBox = sPartyMenuBoxes.at(slot);
    let mut actualSlot: u8 = slot - MULTI_PARTY_SIZE as u8;
    if gMultiPartnerParty[actualSlot].species == SPECIES_NONE {
        DrawEmptySlot((*menuBox).windowId);
    } else {
        (*(*menuBox).infoRects).blitFunc.unwrap_unchecked()((*menuBox).windowId, 0, 0, 0, 0, 0);
        StringCopy(
            gStringVar1.as_mut_ptr(),
            gMultiPartnerParty[actualSlot].nickname.as_mut_ptr(),
        );
        StringGet_Nickname(gStringVar1.as_mut_ptr());
        ConvertInternationalPlayerName(gStringVar1.as_mut_ptr());
        DisplayPartyPokemonBarDetail(
            (*menuBox).windowId,
            gStringVar1.as_mut_ptr(),
            0,
            (*(*menuBox).infoRects).dimensions.as_mut_ptr(),
        );
        DisplayPartyPokemonLevel(gMultiPartnerParty[actualSlot].level, menuBox);
        DisplayPartyPokemonGender(
            gMultiPartnerParty[actualSlot].gender,
            gMultiPartnerParty[actualSlot].species,
            gMultiPartnerParty[actualSlot].nickname.as_mut_ptr(),
            menuBox,
        );
        DisplayPartyPokemonHP(gMultiPartnerParty[actualSlot].hp, menuBox);
        DisplayPartyPokemonMaxHP(gMultiPartnerParty[actualSlot].maxhp, menuBox);
        DisplayPartyPokemonHPBar(
            gMultiPartnerParty[actualSlot].hp,
            gMultiPartnerParty[actualSlot].maxhp,
            menuBox,
        );
    }
}
pub(crate) unsafe extern "C" fn RenderPartyMenuBoxes() -> u8 {
    RenderPartyMenuBox((*sPartyMenuInternal).data[0] as u8);
    if ({
        (*sPartyMenuInternal).data[0] += 1;
        (*sPartyMenuInternal).data[0]
    }) == PARTY_SIZE as i16
    {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn GetPartyMenuBgTile(tileId: u16) -> *mut u8 {
    return sPartyBgGfxTilemap.at((tileId as i32) << 5);
}
pub(crate) unsafe extern "C" fn CreatePartyMonSprites(slot: u8) {
    let mut actualSlot: u8 = 0;
    if gPartyMenu.menuType() == PARTY_MENU_TYPE_MULTI_SHOWCASE && slot >= MULTI_PARTY_SIZE as u8 {
        let mut status: u8 = 0;
        actualSlot = slot - MULTI_PARTY_SIZE as u8;
        if gMultiPartnerParty[actualSlot].species != SPECIES_NONE {
            CreatePartyMonIconSpriteParameterized(
                gMultiPartnerParty[actualSlot].species,
                gMultiPartnerParty[actualSlot].personality,
                sPartyMenuBoxes.at(slot),
                0,
                0,
            );
            CreatePartyMonHeldItemSpriteParameterized(
                gMultiPartnerParty[actualSlot].species,
                gMultiPartnerParty[actualSlot].heldItem,
                sPartyMenuBoxes.at(slot),
            );
            CreatePartyMonPokeballSpriteParameterized(
                gMultiPartnerParty[actualSlot].species,
                sPartyMenuBoxes.at(slot),
            );
            if gMultiPartnerParty[actualSlot].hp == 0 {
                status = AILMENT_FNT;
            } else {
                status = GetAilmentFromStatus(gMultiPartnerParty[actualSlot].status);
            }
            CreatePartyMonStatusSpriteParameterized(
                gMultiPartnerParty[actualSlot].species,
                status,
                sPartyMenuBoxes.at(slot),
            );
        }
    } else if GetMonData2(&raw mut gPlayerParty[slot], MON_DATA_SPECIES) != SPECIES_NONE as u32 {
        CreatePartyMonIconSprite(
            &raw mut gPlayerParty[slot],
            sPartyMenuBoxes.at(slot),
            slot as u32,
        );
        CreatePartyMonHeldItemSprite(&raw mut gPlayerParty[slot], sPartyMenuBoxes.at(slot));
        CreatePartyMonPokeballSprite(&raw mut gPlayerParty[slot], sPartyMenuBoxes.at(slot));
        CreatePartyMonStatusSprite(&raw mut gPlayerParty[slot], sPartyMenuBoxes.at(slot));
    }
}
pub(crate) unsafe extern "C" fn CreatePartyMonSpritesLoop() -> u8 {
    CreatePartyMonSprites((*sPartyMenuInternal).data[0] as u8);
    if ({
        (*sPartyMenuInternal).data[0] += 1;
        (*sPartyMenuInternal).data[0]
    }) == PARTY_SIZE as i16
    {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn CreateCancelConfirmPokeballSprites() {
    if gPartyMenu.menuType() == PARTY_MENU_TYPE_MULTI_SHOWCASE {
        FillBgTilemapBufferRect(1, 14, 23, 17, 7, 2, 1);
    } else {
        if (*sPartyMenuInternal).chooseHalf() != 0 {
            (*sPartyMenuInternal)
                .set_spriteIdConfirmPokeball(CreateSmallPokeballButtonSprite(0xBF, 0x88) as u32);
            DrawCancelConfirmButtons();
            (*sPartyMenuInternal)
                .set_spriteIdCancelPokeball(CreateSmallPokeballButtonSprite(0xBF, 0x98) as u32);
        } else {
            (*sPartyMenuInternal)
                .set_spriteIdCancelPokeball(CreatePokeballButtonSprite(198, 148) as u32);
        }
        AnimatePartySlot(gPartyMenu.slotId as u8, 1);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimatePartySlot(slot: u8, animNum: u8) {
    let mut spriteId: u8 = 0;
    match slot {
        6 => {
            if animNum == 0 {
                SetBgTilemapPalette(1, 23, 16, 7, 2, 1);
            } else {
                SetBgTilemapPalette(1, 23, 16, 7, 2, 2);
            }
            spriteId = (*sPartyMenuInternal).spriteIdConfirmPokeball() as u8;
        }
        7 => {
            if (*sPartyMenuInternal).chooseHalf() == 0 {
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
            spriteId = (*sPartyMenuInternal).spriteIdCancelPokeball() as u8;
        }
        _ => {
            if GetMonData2(&raw mut gPlayerParty[slot], MON_DATA_SPECIES) != SPECIES_NONE as u32 {
                LoadPartyBoxPalette(
                    sPartyMenuBoxes.at(slot),
                    GetPartyBoxPaletteFlags(slot, animNum),
                );
                AnimateSelectedPartyIcon((*sPartyMenuBoxes.at(slot)).monSpriteId, animNum);
                PartyMenuStartSpriteAnim((*sPartyMenuBoxes.at(slot)).pokeballSpriteId, animNum);
            }
            return;
        }
    }
    PartyMenuStartSpriteAnim(spriteId, animNum);
    ScheduleBgCopyTilemapToVram(1);
}
pub(crate) unsafe extern "C" fn GetPartyBoxPaletteFlags(slot: u8, animNum: u8) -> u8 {
    let mut palFlags: u8 = 0;
    if animNum == 1 {
        palFlags |= PARTY_PAL_SELECTED as u8;
    }
    if GetMonData2(&raw mut gPlayerParty[slot], MON_DATA_HP) == 0 {
        palFlags |= PARTY_PAL_FAINTED;
    }
    if PartyBoxPal_ParnterOrDisqualifiedInArena(slot) == TRUE {
        palFlags |= PARTY_PAL_MULTI_ALT;
    }
    if gPartyMenu.action == PARTY_ACTION_SWITCHING {
        palFlags |= PARTY_PAL_SWITCHING;
    }
    if gPartyMenu.action == PARTY_ACTION_SWITCH {
        if slot as i32 == gPartyMenu.slotId as i32 || slot as i32 == gPartyMenu.slotId2 as i32 {
            palFlags |= PARTY_PAL_TO_SWITCH;
        }
    }
    if gPartyMenu.action == PARTY_ACTION_SOFTBOILED && slot as i32 == gPartyMenu.slotId as i32 {
        palFlags |= PARTY_PAL_TO_SOFTBOIL;
    }
    return palFlags;
}
pub(crate) unsafe extern "C" fn PartyBoxPal_ParnterOrDisqualifiedInArena(slot: u8) -> u8 {
    if gPartyMenu.layout() == PARTY_LAYOUT_MULTI && (slot == 1 || slot == 4 || slot == 5) {
        return TRUE;
    }
    if slot < MULTI_PARTY_SIZE as u8
        && gBattleTypeFlags & BATTLE_TYPE_ARENA != 0
        && gMain.inBattle() != 0
        && shr_i32(
            (*gBattleStruct).arenaLostPlayerMons as i32,
            GetPartyIdFromBattleSlot(slot) as u32,
        ) & 1
            != 0
    {
        return TRUE;
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn DrawCancelConfirmButtons() {
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsMultiBattle() -> u8 {
    if gBattleTypeFlags & BATTLE_TYPE_MULTI != 0
        && gBattleTypeFlags & BATTLE_TYPE_DOUBLE != 0
        && gBattleTypeFlags & BATTLE_TYPE_TRAINER != 0
        && gMain.inBattle() != 0
    {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn SwapPartyPokemon(mon1: *mut Pokemon, mon2: *mut Pokemon) {
    let mut temp: *mut Pokemon = Alloc(100) as *mut Pokemon;
    *temp = *mon1;
    *mon1 = *mon2;
    *mon2 = *temp;
    Free(temp as *mut c_void);
}
pub(crate) unsafe extern "C" fn Task_ClosePartyMenu(taskId: u8) {
    BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 16, 0);
    gTasks[taskId].func = Some(Task_ClosePartyMenuAndSetCB2);
}
pub(crate) unsafe extern "C" fn Task_ClosePartyMenuAndSetCB2(taskId: u8) {
    if gPaletteFade.active() == 0 {
        if gPartyMenu.menuType() == PARTY_MENU_TYPE_IN_BATTLE {
            UpdatePartyToFieldOrder();
        }
        if (*sPartyMenuInternal).exitCallback.is_some() {
            SetMainCallback2((*sPartyMenuInternal).exitCallback);
        } else {
            SetMainCallback2(gPartyMenu.exitCallback);
        }
        ResetSpriteData();
        FreePartyPointers();
        DestroyTask(taskId);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetCursorSelectionMonId() -> u8 {
    return gPartyMenu.slotId as u8;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetPartyMenuType() -> u8 {
    return gPartyMenu.menuType();
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Task_HandleChooseMonInput(taskId: u8) {
    if gPaletteFade.active() == 0 && MenuHelpers_ShouldWaitForLinkRecv() != TRUE {
        let mut slotPtr: *mut i8 = GetCurrentPartySlotPtr();
        match PartyMenuButtonHandler(slotPtr) {
            1 => {
                HandleChooseMonSelection(taskId, slotPtr);
            }
            2 => {
                HandleChooseMonCancel(taskId, slotPtr);
            }
            8 => {
                if (*sPartyMenuInternal).chooseHalf() != 0 {
                    PlaySE(SE_SELECT);
                    MoveCursorToConfirm();
                }
            }
            _ => {}
        }
    }
}
pub(crate) unsafe extern "C" fn GetCurrentPartySlotPtr() -> *mut i8 {
    if gPartyMenu.action == PARTY_ACTION_SWITCH || gPartyMenu.action == PARTY_ACTION_SOFTBOILED {
        return &raw mut gPartyMenu.slotId2;
    } else {
        return &raw mut gPartyMenu.slotId;
    }
    #[allow(unreachable_code)]
    {
        return null_mut();
    }
}
pub(crate) unsafe extern "C" fn HandleChooseMonSelection(taskId: u8, slotPtr: *mut i8) {
    if *slotPtr == PARTY_SIZE as i8 {
        gPartyMenu.task.unwrap_unchecked()(taskId);
    } else {
        match gPartyMenu.action {
            PARTY_ACTION_SOFTBOILED => {
                if IsSelectedMonNotEgg(slotPtr as *mut u8) != 0 {
                    PartyMenuRemoveWindow(&raw mut (*sPartyMenuInternal).windowId[1]);
                    Task_TryUseSoftboiledOnPartyMon(taskId);
                }
            }
            PARTY_ACTION_USE_ITEM => {
                if IsSelectedMonNotEgg(slotPtr as *mut u8) != 0 {
                    if gPartyMenu.menuType() == PARTY_MENU_TYPE_IN_BATTLE {
                        (*sPartyMenuInternal).exitCallback = Some(CB2_SetUpExitToBattleScreen);
                    }
                    PartyMenuRemoveWindow(&raw mut (*sPartyMenuInternal).windowId[1]);
                    gItemUseCB.unwrap_unchecked()(taskId, Some(Task_ClosePartyMenuAfterText));
                }
            }
            PARTY_ACTION_MOVE_TUTOR => {
                if IsSelectedMonNotEgg(slotPtr as *mut u8) != 0 {
                    PlaySE(SE_SELECT);
                    PartyMenuRemoveWindow(&raw mut (*sPartyMenuInternal).windowId[1]);
                    TryTutorSelectedMon(taskId);
                }
            }
            PARTY_ACTION_GIVE_MAILBOX_MAIL => {
                if IsSelectedMonNotEgg(slotPtr as *mut u8) != 0 {
                    PlaySE(SE_SELECT);
                    PartyMenuRemoveWindow(&raw mut (*sPartyMenuInternal).windowId[1]);
                    TryGiveMailToSelectedMon(taskId);
                }
            }
            PARTY_ACTION_GIVE_ITEM | PARTY_ACTION_GIVE_PC_ITEM => {
                if IsSelectedMonNotEgg(slotPtr as *mut u8) != 0 {
                    PlaySE(SE_SELECT);
                    PartyMenuRemoveWindow(&raw mut (*sPartyMenuInternal).windowId[1]);
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
pub(crate) unsafe extern "C" fn IsSelectedMonNotEgg(slotPtr: *mut u8) -> u8 {
    if GetMonData2(&raw mut gPlayerParty[*slotPtr], MON_DATA_IS_EGG) == TRUE as u32 {
        PlaySE(SE_FAILURE);
        return FALSE;
    }
    return TRUE;
}
pub(crate) unsafe extern "C" fn HandleChooseMonCancel(taskId: u8, slotPtr: *mut i8) {
    match gPartyMenu.action {
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
                gPartyMenuUseExitCallback = FALSE;
                *slotPtr = 7;
                Task_ClosePartyMenu(taskId);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn DisplayCancelChooseMonYesNo(taskId: u8) -> u8 {
    let mut stringPtr: *mut u8 = null_mut();
    if gPartyMenu.menuType() == PARTY_MENU_TYPE_CONTEST {
        stringPtr = gText_CancelParticipation.as_ptr().cast_mut();
    } else if gPartyMenu.menuType() == PARTY_MENU_TYPE_CHOOSE_HALF {
        stringPtr = GetFacilityCancelString();
    }
    if stringPtr.is_null() {
        return FALSE;
    }
    PartyMenuRemoveWindow(&raw mut (*sPartyMenuInternal).windowId[1]);
    StringExpandPlaceholders(gStringVar4.as_mut_ptr(), stringPtr);
    DisplayPartyMenuMessage(gStringVar4.as_mut_ptr(), TRUE);
    gTasks[taskId].func = Some(Task_CancelChooseMonYesNo);
    return TRUE;
}
pub(crate) unsafe extern "C" fn Task_CancelChooseMonYesNo(taskId: u8) {
    if IsPartyMenuTextPrinterActive() != TRUE {
        PartyMenuDisplayYesNoMenu();
        gTasks[taskId].func = Some(Task_HandleCancelChooseMonYesNoInput);
    }
}
pub(crate) unsafe extern "C" fn Task_HandleCancelChooseMonYesNoInput(taskId: u8) {
    'l1: {
        let sw1: i8 = Menu_ProcessInputNoWrapClearOnChoose();
        let mut fall = false;
        if sw1 == 0 {
            fall = true;
            gPartyMenuUseExitCallback = FALSE;
            gPartyMenu.slotId = 7;
            ClearSelectedPartyOrder();
            Task_ClosePartyMenu(taskId);
            break 'l1;
        }
        if sw1 == MENU_B_PRESSED {
            fall = true;
            PlaySE(SE_SELECT);
        }
        if fall || sw1 == 1 {
            fall = true;
            Task_ReturnToChooseMonAfterText(taskId);
            break 'l1;
        }
    }
}
pub(crate) unsafe extern "C" fn PartyMenuButtonHandler(slotPtr: *mut i8) -> u16 {
    let mut movementDir: i8 = 0;
    match gMain.newAndRepeatedKeys {
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
    if gMain.newKeys as i32 & START_BUTTON != 0 {
        return START_BUTTON as u16;
    }
    if movementDir != 0 {
        UpdateCurrentPartySelection(slotPtr, movementDir);
        return 0;
    }
    if gMain.newKeys as i32 & 0x0001 != 0 && *slotPtr == 7 {
        return B_BUTTON as u16;
    }
    return gMain.newKeys & 3;
}
pub(crate) unsafe extern "C" fn UpdateCurrentPartySelection(slotPtr: *mut i8, movementDir: i8) {
    let mut newSlotId: i8 = *slotPtr;
    let mut layout: u8 = gPartyMenu.layout();
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
pub(crate) unsafe extern "C" fn UpdatePartySelectionSingleLayout(
    slotPtr: *mut i8,
    movementDir: i8,
) {
    match movementDir {
        MENU_DIR_UP => {
            if *slotPtr == 0 {
                *slotPtr = 7;
            } else if *slotPtr == PARTY_SIZE as i8 {
                *slotPtr = gPlayerPartyCount as i8 - 1;
            } else if *slotPtr == 7 {
                if (*sPartyMenuInternal).chooseHalf() != 0 {
                    *slotPtr = PARTY_SIZE as i8;
                } else {
                    *slotPtr = gPlayerPartyCount as i8 - 1;
                }
            } else {
                *slotPtr -= 1;
            }
        }
        MENU_DIR_DOWN => {
            if *slotPtr == 7 {
                *slotPtr = 0;
            } else {
                if *slotPtr as i32 == gPlayerPartyCount as i32 - 1 {
                    if (*sPartyMenuInternal).chooseHalf() != 0 {
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
            if gPlayerPartyCount != 1 && *slotPtr == 0 {
                if (*sPartyMenuInternal).lastSelectedSlot() == 0 {
                    *slotPtr = 1;
                } else {
                    *slotPtr = (*sPartyMenuInternal).lastSelectedSlot() as i8;
                }
            }
        }
        MENU_DIR_LEFT => {
            if *slotPtr != 0 && *slotPtr != PARTY_SIZE as i8 && *slotPtr != 7 {
                (*sPartyMenuInternal).set_lastSelectedSlot(*slotPtr as u32);
                *slotPtr = 0;
            }
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn UpdatePartySelectionDoubleLayout(
    slotPtr: *mut i8,
    movementDir: i8,
) {
    let mut newSlot: i8 = movementDir;
    'l1: {
        match movementDir {
            MENU_DIR_UP => {
                if *slotPtr == 0 {
                    *slotPtr = 7;
                    break 'l1;
                } else if *slotPtr == PARTY_SIZE as i8 {
                    *slotPtr = gPlayerPartyCount as i8 - 1;
                    break 'l1;
                } else if *slotPtr == 7 {
                    if (*sPartyMenuInternal).chooseHalf() != 0 {
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
                        if (*sPartyMenuInternal).chooseHalf() != 0 {
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
                    if (*sPartyMenuInternal).lastSelectedSlot() == 3 {
                        if GetMonData2(&raw mut gPlayerParty[3], MON_DATA_SPECIES)
                            != SPECIES_NONE as u32
                        {
                            *slotPtr = 3;
                        }
                    } else if GetMonData2(&raw mut gPlayerParty[2], MON_DATA_SPECIES)
                        != SPECIES_NONE as u32
                    {
                        *slotPtr = 2;
                    }
                } else if *slotPtr == 1 {
                    if (*sPartyMenuInternal).lastSelectedSlot() == 5 {
                        if GetMonData2(&raw mut gPlayerParty[5], MON_DATA_SPECIES)
                            != SPECIES_NONE as u32
                        {
                            *slotPtr = 5;
                        }
                    } else if GetMonData2(&raw mut gPlayerParty[4], MON_DATA_SPECIES)
                        != SPECIES_NONE as u32
                    {
                        *slotPtr = 4;
                    }
                }
            }
            MENU_DIR_LEFT => {
                if *slotPtr == 2 || *slotPtr == 3 {
                    (*sPartyMenuInternal).set_lastSelectedSlot(*slotPtr as u32);
                    *slotPtr = 0;
                } else if *slotPtr == 4 || *slotPtr == 5 {
                    (*sPartyMenuInternal).set_lastSelectedSlot(*slotPtr as u32);
                    *slotPtr = 1;
                }
            }
            _ => {}
        }
    }
}
pub(crate) unsafe extern "C" fn GetNewSlotDoubleLayout(mut slotId: i8, movementDir: i8) -> i8 {
    loop {
        slotId += movementDir;
        if slotId as u8 >= PARTY_SIZE as u8 {
            return -1;
        }
        if GetMonData2(&raw mut gPlayerParty[slotId], MON_DATA_SPECIES) != SPECIES_NONE as u32 {
            return slotId;
        }
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetMonNickname(mon: *mut Pokemon, dest: *mut u8) -> *mut u8 {
    GetMonData3(mon, MON_DATA_NICKNAME, dest);
    return StringGet_Nickname(dest);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DisplayPartyMenuMessage(str: *mut u8, keepOpen: u8) -> u8 {
    let mut taskId: u8 = 0;
    PrintMessage(str);
    taskId = CreateTask(Some(Task_PrintAndWaitForText), 1);
    gTasks[taskId].data[0] = keepOpen as i16;
    return taskId;
}
pub(crate) unsafe extern "C" fn Task_PrintAndWaitForText(taskId: u8) {
    if RunTextPrintersRetIsActive(WIN_MSG) != TRUE as u16 {
        if gTasks[taskId].data[0] == FALSE as i16 {
            ClearStdWindowAndFrameToTransparent(WIN_MSG, FALSE);
            ClearWindowTilemap(WIN_MSG);
        }
        DestroyTask(taskId);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsPartyMenuTextPrinterActive() -> u8 {
    return FuncIsActiveTask(Some(Task_PrintAndWaitForText));
}
pub(crate) unsafe extern "C" fn Task_WaitForLinkAndReturnToChooseMon(taskId: u8) {
    if MenuHelpers_ShouldWaitForLinkRecv() != TRUE {
        DisplayPartyMenuStdMessage(PARTY_MSG_CHOOSE_MON);
        gTasks[taskId].func = Some(Task_HandleChooseMonInput);
    }
}
pub(crate) unsafe extern "C" fn Task_ReturnToChooseMonAfterText(taskId: u8) {
    if IsPartyMenuTextPrinterActive() != TRUE {
        ClearStdWindowAndFrameToTransparent(WIN_MSG, FALSE);
        ClearWindowTilemap(WIN_MSG);
        if MenuHelpers_IsLinkActive() == TRUE {
            gTasks[taskId].func = Some(Task_WaitForLinkAndReturnToChooseMon);
        } else {
            DisplayPartyMenuStdMessage(PARTY_MSG_CHOOSE_MON);
            gTasks[taskId].func = Some(Task_HandleChooseMonInput);
        }
    }
}
pub(crate) unsafe extern "C" fn DisplayGaveHeldItemMessage(
    mon: *mut Pokemon,
    item: u16,
    keepOpen: u8,
    unused: u8,
) {
    GetMonNickname(mon, gStringVar1.as_mut_ptr());
    CopyItemName(item, gStringVar2.as_mut_ptr());
    StringExpandPlaceholders(
        gStringVar4.as_mut_ptr(),
        gText_PkmnWasGivenItem.as_ptr().cast_mut(),
    );
    DisplayPartyMenuMessage(gStringVar4.as_mut_ptr(), keepOpen);
    ScheduleBgCopyTilemapToVram(2);
}
pub(crate) unsafe extern "C" fn DisplayTookHeldItemMessage(
    mon: *mut Pokemon,
    item: u16,
    keepOpen: u8,
) {
    GetMonNickname(mon, gStringVar1.as_mut_ptr());
    CopyItemName(item, gStringVar2.as_mut_ptr());
    StringExpandPlaceholders(
        gStringVar4.as_mut_ptr(),
        gText_ReceivedItemFromPkmn.as_ptr().cast_mut(),
    );
    DisplayPartyMenuMessage(gStringVar4.as_mut_ptr(), keepOpen);
    ScheduleBgCopyTilemapToVram(2);
}
pub(crate) unsafe extern "C" fn DisplayAlreadyHoldingItemSwitchMessage(
    mon: *mut Pokemon,
    item: u16,
    keepOpen: u8,
) {
    GetMonNickname(mon, gStringVar1.as_mut_ptr());
    CopyItemName(item, gStringVar2.as_mut_ptr());
    StringExpandPlaceholders(
        gStringVar4.as_mut_ptr(),
        gText_PkmnAlreadyHoldingItemSwitch.as_ptr().cast_mut(),
    );
    DisplayPartyMenuMessage(gStringVar4.as_mut_ptr(), keepOpen);
    ScheduleBgCopyTilemapToVram(2);
}
pub(crate) unsafe extern "C" fn DisplaySwitchedHeldItemMessage(
    item: u16,
    item2: u16,
    keepOpen: u8,
) {
    CopyItemName(item, gStringVar1.as_mut_ptr());
    CopyItemName(item2, gStringVar2.as_mut_ptr());
    StringExpandPlaceholders(
        gStringVar4.as_mut_ptr(),
        gText_SwitchedPkmnItem.as_ptr().cast_mut(),
    );
    DisplayPartyMenuMessage(gStringVar4.as_mut_ptr(), keepOpen);
    ScheduleBgCopyTilemapToVram(2);
}
pub(crate) unsafe extern "C" fn GiveItemToMon(mon: *mut Pokemon, item: u16) {
    let mut itemBytes: CArray<u8, 2> = zeroed();
    if ItemIsMail(item) == TRUE {
        if GiveMailToMonByItemId(mon, item) == MAIL_NONE as u8 {
            return;
        }
    }
    itemBytes[0] = item as u8;
    itemBytes[1] = (item >> 8) as u8;
    SetMonData(
        mon,
        MON_DATA_HELD_ITEM,
        itemBytes.as_mut_ptr() as *mut c_void,
    );
}
pub(crate) unsafe extern "C" fn TryTakeMonItem(mon: *mut Pokemon) -> u8 {
    let mut item: u16 = GetMonData2(mon, MON_DATA_HELD_ITEM) as u16;
    if item == ITEM_NONE {
        return 0;
    }
    if AddBagItem(item, 1) == FALSE {
        return 1;
    }
    item = ITEM_NONE;
    SetMonData(mon, MON_DATA_HELD_ITEM, &raw mut item as *mut c_void);
    return 2;
}
pub(crate) unsafe extern "C" fn BufferBagFullCantTakeItemMessage(itemUnused: u16) {
    StringExpandPlaceholders(
        gStringVar4.as_mut_ptr(),
        gText_BagFullCouldNotRemoveItem.as_ptr().cast_mut(),
    );
}
pub(crate) unsafe extern "C" fn Task_PartyMenuModifyHP(taskId: u8) {
    let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
    *data += *data.at(2);
    *data.at(3) -= 1;
    SetMonData(
        &raw mut gPlayerParty[*data.at(4)],
        MON_DATA_HP,
        data as *mut c_void,
    );
    DisplayPartyPokemonHPCheck(
        &raw mut gPlayerParty[*data.at(4)],
        sPartyMenuBoxes.at(*data.at(4)),
        1,
    );
    DisplayPartyPokemonHPBarCheck(
        &raw mut gPlayerParty[*data.at(4)],
        sPartyMenuBoxes.at(*data.at(4)),
    );
    if *data.at(3) == 0 || *data == 0 || *data == *data.at(1) {
        if *data > *data.at(5) {
            ConvertIntToDecimalStringN(
                gStringVar2.as_mut_ptr(),
                *data as i32 - *data.at(5) as i32,
                STR_CONV_MODE_LEFT_ALIGN,
                3,
            );
        }
        SwitchTaskToFollowupFunc(taskId);
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
    let mut mon: *mut Pokemon = &raw mut gPlayerParty[slot];
    let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
    *data = GetMonData2(mon, MON_DATA_HP) as i16;
    *data.at(1) = GetMonData2(mon, MON_DATA_MAX_HP) as i16;
    *data.at(2) = hpIncrement as i16;
    *data.at(3) = hpDifference;
    *data.at(4) = slot as i16;
    *data.at(5) = *data;
    SetTaskFuncWithFollowupFunc(taskId, Some(Task_PartyMenuModifyHP), task);
}
pub(crate) unsafe extern "C" fn ResetHPTaskData(taskId: u8, caseId: u8, hp: u32) {
    let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
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
                core::mem::transmute::<usize, Option<unsafe extern "C" fn(u8)>>(hp as usize),
            );
        }
        _ => {}
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetAilmentFromStatus(status: u32) -> u8 {
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
    return AILMENT_NONE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetMonAilment(mon: *mut Pokemon) -> u8 {
    let mut ailment: u8 = 0;
    if GetMonData2(mon, MON_DATA_HP) == 0 {
        return AILMENT_FNT;
    }
    ailment = GetAilmentFromStatus(GetMonData2(mon, MON_DATA_STATUS));
    if ailment != AILMENT_NONE {
        return ailment;
    }
    if CheckPartyPokerus(mon, 0) != 0 {
        return AILMENT_PKRS;
    }
    return AILMENT_NONE;
}
pub(crate) unsafe extern "C" fn SetPartyMonsAllowedInMinigame() {
    let mut ptr: *mut i16 = null_mut();
    if gPartyMenu.menuType() == PARTY_MENU_TYPE_MINIGAME {
        let mut i: u8 = 0;
        ptr = &raw mut gPartyMenu.data[0];
        gPartyMenu.data[0] = 0;
        if gSpecialVar_0x8005 == 0 {
            i = 0;
            while i < gPlayerPartyCount {
                *ptr += shl_i32(
                    IsMonAllowedInPokemonJump(&raw mut gPlayerParty[i]) as i32,
                    i as u32,
                ) as i16;
                i += 1;
            }
        } else {
            i = 0;
            while i < gPlayerPartyCount {
                *ptr += shl_i32(
                    IsMonAllowedInDodrioBerryPicking(&raw mut gPlayerParty[i]) as i32,
                    i as u32,
                ) as i16;
                i += 1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn IsMonAllowedInPokemonJump(mon: *mut Pokemon) -> u16 {
    if GetMonData2(mon, MON_DATA_IS_EGG) != TRUE as u32
        && IsSpeciesAllowedInPokemonJump(GetMonData2(mon, MON_DATA_SPECIES) as u16) != 0
    {
        return TRUE as u16;
    }
    return FALSE as u16;
}
pub(crate) unsafe extern "C" fn IsMonAllowedInDodrioBerryPicking(mon: *mut Pokemon) -> u16 {
    if GetMonData2(mon, MON_DATA_IS_EGG) != TRUE as u32
        && GetMonData2(mon, MON_DATA_SPECIES) == SPECIES_DODRIO
    {
        return TRUE as u16;
    }
    return FALSE as u16;
}
pub(crate) unsafe extern "C" fn IsMonAllowedInMinigame(slot: u8) -> u8 {
    if shr_i32(gPartyMenu.data[0] as i32, slot as u32) & 1 == 0 {
        return FALSE;
    }
    return TRUE;
}
pub(crate) unsafe extern "C" fn TryEnterMonForMinigame(taskId: u8, slot: u8) {
    if IsMonAllowedInMinigame(slot) == TRUE {
        PlaySE(SE_SELECT);
        gSpecialVar_0x8004 = slot as u16;
        Task_ClosePartyMenu(taskId);
    } else {
        PlaySE(SE_FAILURE);
        DisplayPartyMenuMessage(gText_PkmnCantParticipate.as_ptr().cast_mut(), FALSE);
        ScheduleBgCopyTilemapToVram(2);
        gTasks[taskId].func = Some(Task_ReturnToChooseMonAfterText);
    }
}
pub(crate) unsafe extern "C" fn CancelParticipationPrompt(taskId: u8) {
    DisplayPartyMenuMessage(gText_CancelParticipation.as_ptr().cast_mut(), TRUE);
    ScheduleBgCopyTilemapToVram(2);
    gTasks[taskId].func = Some(Task_CancelParticipationYesNo);
}
pub(crate) unsafe extern "C" fn Task_CancelParticipationYesNo(taskId: u8) {
    if IsPartyMenuTextPrinterActive() != TRUE {
        PartyMenuDisplayYesNoMenu();
        gTasks[taskId].func = Some(Task_HandleCancelParticipationYesNoInput);
    }
}
pub(crate) unsafe extern "C" fn Task_HandleCancelParticipationYesNoInput(taskId: u8) {
    'l1: {
        let sw1: i8 = Menu_ProcessInputNoWrapClearOnChoose();
        let mut fall = false;
        if sw1 == 0 {
            fall = true;
            gSpecialVar_0x8004 = 7;
            Task_ClosePartyMenu(taskId);
            break 'l1;
        }
        if sw1 == MENU_B_PRESSED {
            fall = true;
            PlaySE(SE_SELECT);
        }
        if fall || sw1 == 1 {
            fall = true;
            gTasks[taskId].func = Some(Task_ReturnToChooseMonAfterText);
            break 'l1;
        }
    }
}
pub(crate) unsafe extern "C" fn CanMonLearnTMTutor(mon: *mut Pokemon, item: u16, tutor: u8) -> u8 {
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
        return 0;
    }
}
pub(crate) unsafe extern "C" fn GetTutorMove(tutor: u8) -> u16 {
    return gTutorMoves[tutor];
}
pub(crate) unsafe extern "C" fn CanLearnTutorMove(species: u16, tutor: u8) -> u8 {
    if sTutorLearnsets[species] & shl_i32(1, tutor as u32) as u32 != 0 {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn InitPartyMenuWindows(layout: u8) {
    let mut i: u8 = 0;
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
    i = 0;
    while i < PARTY_SIZE as u8 {
        FillWindowPixelBuffer(i, 0);
        i += 1;
    }
    LoadUserWindowBorderGfx(0, 0x4F, 208);
    LoadPalette(GetOverworldTextboxPalettePtr() as *mut c_void, 224, 32);
    LoadPalette(
        gStandardMenuPalette.as_ptr().cast_mut() as *mut c_void,
        240,
        32,
    );
}
pub(crate) unsafe extern "C" fn CreateCancelConfirmWindows(chooseHalf: u8) {
    let mut confirmWindowId: u8 = 0;
    let mut cancelWindowId: u8 = 0;
    let mut offset: u8 = 0;
    let mut mainOffset: u8 = 0;
    if gPartyMenu.menuType() != PARTY_MENU_TYPE_MULTI_SHOWCASE {
        if chooseHalf == TRUE {
            confirmWindowId =
                AddWindow((&raw const *sConfirmButtonWindowTemplate).cast_mut()) as u8;
            FillWindowPixelBuffer(confirmWindowId, 0);
            mainOffset = GetStringCenterAlignXOffset(
                FONT_SMALL as i32,
                gMenuText_Confirm.as_ptr().cast_mut(),
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
                gMenuText_Confirm.as_ptr().cast_mut(),
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
        if gPartyMenu.menuType() != PARTY_MENU_TYPE_SPIN_TRADE {
            mainOffset = GetStringCenterAlignXOffset(
                FONT_SMALL as i32,
                gText_Cancel.as_ptr().cast_mut(),
                48,
            ) as u8;
            AddTextPrinterParameterized3(
                cancelWindowId,
                FONT_SMALL,
                mainOffset + offset,
                1,
                sFontColorTable[0].as_ptr().cast_mut(),
                TEXT_SKIP_DRAW as i8,
                gText_Cancel.as_ptr().cast_mut(),
            );
        } else {
            mainOffset = GetStringCenterAlignXOffset(
                FONT_SMALL as i32,
                gText_Cancel2.as_ptr().cast_mut(),
                48,
            ) as u8;
            AddTextPrinterParameterized3(
                cancelWindowId,
                FONT_SMALL,
                mainOffset + offset,
                1,
                sFontColorTable[0].as_ptr().cast_mut(),
                TEXT_SKIP_DRAW as i8,
                gText_Cancel2.as_ptr().cast_mut(),
            );
        }
        PutWindowTilemap(cancelWindowId);
        CopyWindowToVram(cancelWindowId, COPYWIN_GFX);
        ScheduleBgCopyTilemapToVram(0);
    }
}
pub(crate) unsafe extern "C" fn GetPartyMenuPalBufferPtr(paletteId: u8) -> *mut u16 {
    return &raw mut (*sPartyMenuInternal).palBuffer[paletteId];
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
    let mut pixels: *mut u8 = AllocZeroed(height as u32 * width as u32 * 32) as *mut u8;
    let mut i: u8 = 0;
    let mut j: u8 = 0;
    if !pixels.is_null() {
        i = 0;
        while i < height {
            j = 0;
            while j < width {
                CpuSet(
                    GetPartyMenuBgTile(
                        *b.at(x as i32 + j as i32 + (y as i32 + i as i32) * c as i32) as u16,
                    ) as *mut c_void,
                    pixels.at((i as i32 * width as i32 + j as i32) * 32) as *mut c_void,
                    16,
                );
                j += 1;
            }
            i += 1;
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
pub(crate) unsafe extern "C" fn BlitBitmapToPartyWindow_LeftColumn(
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
pub(crate) unsafe extern "C" fn BlitBitmapToPartyWindow_RightColumn(
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
pub(crate) unsafe extern "C" fn DrawEmptySlot(windowId: u8) {
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
pub(crate) unsafe extern "C" fn LoadPartyBoxPalette(menuBox: *mut PartyMenuBox, palFlags: u8) {
    let mut palOffset: u8 =
        0x000 + GetWindowAttribute((*menuBox).windowId, WINDOW_PALETTE_NUM) as u8 * 16;
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
pub(crate) unsafe extern "C" fn DisplayPartyPokemonBarDetail(
    windowId: u8,
    str: *mut u8,
    color: u8,
    align: *mut u8,
) {
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
pub(crate) unsafe extern "C" fn DisplayPartyPokemonNickname(
    mon: *mut Pokemon,
    menuBox: *mut PartyMenuBox,
    c: u8,
) {
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
pub(crate) unsafe extern "C" fn DisplayPartyPokemonLevelCheck(
    mon: *mut Pokemon,
    menuBox: *mut PartyMenuBox,
    c: u8,
) {
    if GetMonData2(mon, MON_DATA_SPECIES) != SPECIES_NONE as u32 {
        let mut ailment: u8 = GetMonAilment(mon);
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
pub(crate) unsafe extern "C" fn DisplayPartyPokemonLevel(level: u8, menuBox: *mut PartyMenuBox) {
    ConvertIntToDecimalStringN(
        gStringVar2.as_mut_ptr(),
        level as i32,
        STR_CONV_MODE_LEFT_ALIGN,
        3,
    );
    StringCopy(
        gStringVar1.as_mut_ptr(),
        gText_LevelSymbol.as_ptr().cast_mut(),
    );
    StringAppend(gStringVar1.as_mut_ptr(), gStringVar2.as_mut_ptr());
    DisplayPartyPokemonBarDetail(
        (*menuBox).windowId,
        gStringVar1.as_mut_ptr(),
        0,
        &raw mut (*(*menuBox).infoRects).dimensions[4],
    );
}
pub(crate) unsafe extern "C" fn DisplayPartyPokemonGenderNidoranCheck(
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
pub(crate) unsafe extern "C" fn DisplayPartyPokemonGender(
    gender: u8,
    species: u16,
    nickname: *mut u8,
    menuBox: *mut PartyMenuBox,
) {
    let mut palOffset: u8 =
        0x000 + GetWindowAttribute((*menuBox).windowId, WINDOW_PALETTE_NUM) as u8 * 16;
    if species == SPECIES_NONE {
        return;
    }
    if (species == SPECIES_NIDORAN_M || species == SPECIES_NIDORAN_F)
        && StringCompare(nickname, gSpeciesNames[species].as_ptr().cast_mut()) == 0
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
                gText_MaleSymbol.as_ptr().cast_mut(),
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
                gText_FemaleSymbol.as_ptr().cast_mut(),
                2,
                &raw mut (*(*menuBox).infoRects).dimensions[8],
            );
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn DisplayPartyPokemonHPCheck(
    mon: *mut Pokemon,
    menuBox: *mut PartyMenuBox,
    c: u8,
) {
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
pub(crate) unsafe extern "C" fn DisplayPartyPokemonHP(hp: u16, menuBox: *mut PartyMenuBox) {
    let mut strOut: *mut u8 = ConvertIntToDecimalStringN(
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
pub(crate) unsafe extern "C" fn DisplayPartyPokemonMaxHPCheck(
    mon: *mut Pokemon,
    menuBox: *mut PartyMenuBox,
    c: u8,
) {
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
pub(crate) unsafe extern "C" fn DisplayPartyPokemonMaxHP(maxhp: u16, menuBox: *mut PartyMenuBox) {
    ConvertIntToDecimalStringN(
        gStringVar2.as_mut_ptr(),
        maxhp as i32,
        STR_CONV_MODE_RIGHT_ALIGN,
        3,
    );
    StringCopy(gStringVar1.as_mut_ptr(), gText_Slash.as_ptr().cast_mut());
    StringAppend(gStringVar1.as_mut_ptr(), gStringVar2.as_mut_ptr());
    DisplayPartyPokemonBarDetail(
        (*menuBox).windowId,
        gStringVar1.as_mut_ptr(),
        0,
        &raw mut (*(*menuBox).infoRects).dimensions[16],
    );
}
pub(crate) unsafe extern "C" fn DisplayPartyPokemonHPBarCheck(
    mon: *mut Pokemon,
    menuBox: *mut PartyMenuBox,
) {
    if GetMonData2(mon, MON_DATA_SPECIES) != SPECIES_NONE as u32 {
        DisplayPartyPokemonHPBar(
            GetMonData2(mon, MON_DATA_HP) as u16,
            GetMonData2(mon, MON_DATA_MAX_HP) as u16,
            menuBox,
        );
    }
}
pub(crate) unsafe extern "C" fn DisplayPartyPokemonHPBar(
    hp: u16,
    maxhp: u16,
    menuBox: *mut PartyMenuBox,
) {
    let mut palOffset: u8 =
        0x000 + GetWindowAttribute((*menuBox).windowId, WINDOW_PALETTE_NUM) as u8 * 16;
    let mut hpFraction: u8 = 0;
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
    hpFraction = GetScaledHPFraction(
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
pub(crate) unsafe extern "C" fn DisplayPartyPokemonDescriptionText(
    stringID: u8,
    menuBox: *mut PartyMenuBox,
    c: u8,
) {
    if c != 0 {
        let mut width: i32 = ((*(*menuBox).infoRects).descTextLeft as i32 % 8
            + (*(*menuBox).infoRects).descTextWidth as i32
            + 7)
            / 8;
        let mut height: i32 = ((*(*menuBox).infoRects).descTextTop as i32 % 8
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
pub(crate) unsafe extern "C" fn PartyMenuRemoveWindow(ptr: *mut u8) {
    if *ptr != WINDOW_NONE {
        ClearStdWindowAndFrameToTransparent(*ptr, FALSE);
        RemoveWindow(*ptr);
        *ptr = WINDOW_NONE;
        ScheduleBgCopyTilemapToVram(2);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DisplayPartyMenuStdMessage(mut stringId: u32) {
    let mut windowPtr: *mut u8 = &raw mut (*sPartyMenuInternal).windowId[1];
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
            if (*sPartyMenuInternal).chooseHalf() != 0 {
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
pub(crate) unsafe extern "C" fn ShouldUseChooseMonText() -> u8 {
    let mut party: *mut Pokemon = gPlayerParty.as_mut_ptr();
    let mut i: u8 = 0;
    let mut numAliveMons: u8 = 0;
    if gPartyMenu.action == PARTY_ACTION_SEND_OUT {
        return TRUE;
    }
    i = 0;
    while i < PARTY_SIZE as u8 {
        if GetMonData2(party.at(i), MON_DATA_SPECIES) != 0
            && (GetMonData2(party.at(i), MON_DATA_HP) != 0
                || GetMonData2(party.at(i), MON_DATA_IS_EGG) != 0)
        {
            numAliveMons += 1;
        }
        if numAliveMons > 1 {
            return TRUE;
        }
        i += 1;
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn DisplaySelectionWindow(windowType: u8) -> u8 {
    let mut window: WindowTemplate = zeroed();
    let mut cursorDimension: u8 = 0;
    let mut letterSpacing: u8 = 0;
    let mut i: u8 = 0;
    match windowType {
        SELECTWINDOW_ACTIONS => {
            SetWindowTemplateFields(
                &raw mut window,
                2,
                19,
                19 - (*sPartyMenuInternal).numActions * 2,
                10,
                (*sPartyMenuInternal).numActions * 2,
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
    (*sPartyMenuInternal).windowId[0] = AddWindow(&raw mut window) as u8;
    DrawStdFrameWithCustomTileAndPalette((*sPartyMenuInternal).windowId[0], 0, 0x4F, 13);
    if windowType == SELECTWINDOW_MOVES {
        return (*sPartyMenuInternal).windowId[0];
    }
    cursorDimension = GetMenuCursorDimensionByFont(FONT_NORMAL, 0);
    letterSpacing = GetFontAttribute(FONT_NORMAL, FONTATTR_LETTER_SPACING);
    i = 0;
    while i < (*sPartyMenuInternal).numActions {
        let mut fontColorsId: u8 = (if (*sPartyMenuInternal).actions[i] >= MENU_FIELD_MOVES {
            4
        } else {
            3
        }) as u8;
        AddTextPrinterParameterized4(
            (*sPartyMenuInternal).windowId[0],
            FONT_NORMAL,
            cursorDimension,
            i * 16 + 1,
            letterSpacing,
            0,
            sFontColorTable[fontColorsId].as_ptr().cast_mut(),
            0,
            sCursorOptions[(*sPartyMenuInternal).actions[i]].text,
        );
        i += 1;
    }
    InitMenuInUpperLeftCorner(
        (*sPartyMenuInternal).windowId[0],
        (*sPartyMenuInternal).numActions,
        0,
        TRUE,
    );
    ScheduleBgCopyTilemapToVram(2);
    return (*sPartyMenuInternal).windowId[0];
}
pub(crate) unsafe extern "C" fn PrintMessage(text: *mut u8) {
    DrawStdFrameWithCustomTileAndPalette(WIN_MSG, FALSE, 0x4F, 13);
    gTextFlags.set_canABSpeedUpPrint(TRUE);
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
pub(crate) unsafe extern "C" fn PartyMenuDisplayYesNoMenu() {
    CreateYesNoMenu(
        (&raw const *sPartyMenuYesNoWindowTemplate).cast_mut(),
        0x4F,
        13,
        0,
    );
}
pub(crate) unsafe extern "C" fn CreateLevelUpStatsWindow() -> u8 {
    (*sPartyMenuInternal).windowId[0] =
        AddWindow((&raw const *sLevelUpStatsWindowTemplate).cast_mut()) as u8;
    DrawStdFrameWithCustomTileAndPalette((*sPartyMenuInternal).windowId[0], 0, 0x4F, 13);
    return (*sPartyMenuInternal).windowId[0];
}
pub(crate) unsafe extern "C" fn RemoveLevelUpStatsWindow() {
    ClearWindowTilemap((*sPartyMenuInternal).windowId[0]);
    PartyMenuRemoveWindow(&raw mut (*sPartyMenuInternal).windowId[0]);
}
pub(crate) unsafe extern "C" fn SetPartyMonSelectionActions(
    mons: *mut Pokemon,
    slotId: u8,
    action: u8,
) {
    let mut i: u8 = 0;
    if action == ACTIONS_NONE as u8 {
        SetPartyMonFieldSelectionActions(mons, slotId);
    } else {
        (*sPartyMenuInternal).numActions = sPartyMenuActionCounts[action];
        i = 0;
        while i < (*sPartyMenuInternal).numActions {
            (*sPartyMenuInternal).actions[i] = *sPartyMenuActions[action].at(i);
            i += 1;
        }
    }
}
pub(crate) unsafe extern "C" fn SetPartyMonFieldSelectionActions(
    mut mons: *mut Pokemon,
    slotId: u8,
) {
    let mut i: u8 = 0;
    let mut j: u8 = 0;
    (*sPartyMenuInternal).numActions = 0;
    AppendToList(
        (*sPartyMenuInternal).actions.as_mut_ptr(),
        &raw mut (*sPartyMenuInternal).numActions,
        MENU_SUMMARY,
    );
    i = 0;
    while i < MAX_MON_MOVES as u8 {
        j = 0;
        while sFieldMoves[j] != FIELD_MOVES_COUNT {
            if GetMonData2(mons.at(slotId), i as i32 + MON_DATA_MOVE1) == sFieldMoves[j] as u32 {
                AppendToList(
                    (*sPartyMenuInternal).actions.as_mut_ptr(),
                    &raw mut (*sPartyMenuInternal).numActions,
                    j + MENU_FIELD_MOVES,
                );
                break;
            }
            j += 1;
        }
        i += 1;
    }
    if InBattlePike() == 0 {
        if GetMonData2(mons.at(1), MON_DATA_SPECIES) != SPECIES_NONE as u32 {
            AppendToList(
                (*sPartyMenuInternal).actions.as_mut_ptr(),
                &raw mut (*sPartyMenuInternal).numActions,
                MENU_SWITCH,
            );
        }
        if ItemIsMail(GetMonData2(mons.at(slotId), MON_DATA_HELD_ITEM) as u16) != 0 {
            AppendToList(
                (*sPartyMenuInternal).actions.as_mut_ptr(),
                &raw mut (*sPartyMenuInternal).numActions,
                MENU_MAIL,
            );
        } else {
            AppendToList(
                (*sPartyMenuInternal).actions.as_mut_ptr(),
                &raw mut (*sPartyMenuInternal).numActions,
                MENU_ITEM,
            );
        }
    }
    AppendToList(
        (*sPartyMenuInternal).actions.as_mut_ptr(),
        &raw mut (*sPartyMenuInternal).numActions,
        MENU_CANCEL1,
    );
}
pub(crate) unsafe extern "C" fn GetPartyMenuActionsType(mon: *mut Pokemon) -> u8 {
    let mut actionType: u32 = 0;
    match gPartyMenu.menuType() {
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
        PARTY_MENU_TYPE_CHOOSE_HALF => match GetPartySlotEntryStatus(gPartyMenu.slotId) {
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
    return actionType as u8;
}
pub(crate) unsafe extern "C" fn CreateSelectionWindow(taskId: u8) -> u8 {
    let mut mon: *mut Pokemon = &raw mut gPlayerParty[gPartyMenu.slotId];
    let mut item: u16 = 0;
    GetMonNickname(mon, gStringVar1.as_mut_ptr());
    PartyMenuRemoveWindow(&raw mut (*sPartyMenuInternal).windowId[1]);
    if gPartyMenu.menuType() != PARTY_MENU_TYPE_STORE_PYRAMID_HELD_ITEMS {
        SetPartyMonSelectionActions(
            gPlayerParty.as_mut_ptr(),
            gPartyMenu.slotId as u8,
            GetPartyMenuActionsType(mon),
        );
        DisplaySelectionWindow(SELECTWINDOW_ACTIONS);
        DisplayPartyMenuStdMessage(PARTY_MSG_DO_WHAT_WITH_MON);
    } else {
        item = GetMonData2(mon, MON_DATA_HELD_ITEM) as u16;
        if item != ITEM_NONE {
            SetPartyMonSelectionActions(
                gPlayerParty.as_mut_ptr(),
                gPartyMenu.slotId as u8,
                GetPartyMenuActionsType(mon),
            );
            DisplaySelectionWindow(SELECTWINDOW_ITEM);
            CopyItemName(item, gStringVar2.as_mut_ptr());
            DisplayPartyMenuStdMessage(PARTY_MSG_ALREADY_HOLDING_ONE);
        } else {
            StringExpandPlaceholders(
                gStringVar4.as_mut_ptr(),
                gText_PkmnNotHolding.as_ptr().cast_mut(),
            );
            DisplayPartyMenuMessage(gStringVar4.as_mut_ptr(), TRUE);
            ScheduleBgCopyTilemapToVram(2);
            gTasks[taskId].func = Some(Task_UpdateHeldItemSprite);
            return FALSE;
        }
    }
    return TRUE;
}
pub(crate) unsafe extern "C" fn Task_TryCreateSelectionWindow(taskId: u8) {
    if CreateSelectionWindow(taskId) != 0 {
        gTasks[taskId].data[0] = 0xFF;
        gTasks[taskId].func = Some(Task_HandleSelectionMenuInput);
    }
}
pub(crate) unsafe extern "C" fn Task_HandleSelectionMenuInput(taskId: u8) {
    if gPaletteFade.active() == 0 && MenuHelpers_ShouldWaitForLinkRecv() != TRUE {
        let mut input: i8 = 0;
        let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
        if (*sPartyMenuInternal).numActions <= 3 {
            input = Menu_ProcessInputNoWrapAround_other();
        } else {
            input = ProcessMenuInput_other();
        }
        *data = Menu_GetCursorPos() as i16;
        match input {
            MENU_NOTHING_CHOSEN => {}
            MENU_B_PRESSED => {
                PlaySE(SE_SELECT);
                PartyMenuRemoveWindow(&raw mut (*sPartyMenuInternal).windowId[2]);
                sCursorOptions
                    [(*sPartyMenuInternal).actions[(*sPartyMenuInternal).numActions as i32 - 1]]
                    .func
                    .unwrap_unchecked()(taskId);
            }
            _ => {
                PartyMenuRemoveWindow(&raw mut (*sPartyMenuInternal).windowId[2]);
                sCursorOptions[(*sPartyMenuInternal).actions[input]]
                    .func
                    .unwrap_unchecked()(taskId);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn CursorCb_Summary(taskId: u8) {
    PlaySE(SE_SELECT);
    (*sPartyMenuInternal).exitCallback = Some(CB2_ShowPokemonSummaryScreen);
    Task_ClosePartyMenu(taskId);
}
pub(crate) unsafe extern "C" fn CB2_ShowPokemonSummaryScreen() {
    if gPartyMenu.menuType() == PARTY_MENU_TYPE_IN_BATTLE {
        UpdatePartyToBattleOrder();
        ShowPokemonSummaryScreen(
            SUMMARY_MODE_LOCK_MOVES,
            gPlayerParty.as_mut_ptr() as *mut c_void,
            gPartyMenu.slotId as u8,
            gPlayerPartyCount - 1,
            Some(CB2_ReturnToPartyMenuFromSummaryScreen),
        );
    } else {
        ShowPokemonSummaryScreen(
            SUMMARY_MODE_NORMAL,
            gPlayerParty.as_mut_ptr() as *mut c_void,
            gPartyMenu.slotId as u8,
            gPlayerPartyCount - 1,
            Some(CB2_ReturnToPartyMenuFromSummaryScreen),
        );
    }
}
pub(crate) unsafe extern "C" fn CB2_ReturnToPartyMenuFromSummaryScreen() {
    gPaletteFade.set_bufferTransferDisabled(TRUE as u16);
    gPartyMenu.slotId = gLastViewedMonIndex as i8;
    InitPartyMenu(
        gPartyMenu.menuType(),
        KEEP_PARTY_LAYOUT,
        gPartyMenu.action,
        TRUE,
        PARTY_MSG_DO_WHAT_WITH_MON as u8,
        Some(Task_TryCreateSelectionWindow),
        gPartyMenu.exitCallback,
    );
}
pub(crate) unsafe extern "C" fn CursorCb_Switch(taskId: u8) {
    PlaySE(SE_SELECT);
    gPartyMenu.action = PARTY_ACTION_SWITCH;
    PartyMenuRemoveWindow(&raw mut (*sPartyMenuInternal).windowId[1]);
    PartyMenuRemoveWindow(&raw mut (*sPartyMenuInternal).windowId[0]);
    DisplayPartyMenuStdMessage(PARTY_MSG_MOVE_TO_WHERE);
    AnimatePartySlot(gPartyMenu.slotId as u8, 1);
    gPartyMenu.slotId2 = gPartyMenu.slotId;
    gTasks[taskId].func = Some(Task_HandleChooseMonInput);
}
pub(crate) unsafe extern "C" fn SwitchSelectedMons(taskId: u8) {
    let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
    let mut windowIds: CArray<u8, 2> = zeroed();
    if gPartyMenu.slotId2 == gPartyMenu.slotId {
        FinishTwoMonAction(taskId);
    } else {
        windowIds[0] = (*sPartyMenuBoxes.at(gPartyMenu.slotId)).windowId;
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
        windowIds[1] = (*sPartyMenuBoxes.at(gPartyMenu.slotId2)).windowId;
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
        sSlot1TilemapBuffer = Alloc(*data.at(2) as u32 * ((*data.at(3) as u32) << 1)) as *mut u16;
        sSlot2TilemapBuffer = Alloc(*data.at(6) as u32 * ((*data.at(7) as u32) << 1)) as *mut u16;
        CopyToBufferFromBgTilemap(
            0,
            sSlot1TilemapBuffer,
            *data as u8,
            *data.at(1) as u8,
            *data.at(2) as u8,
            *data.at(3) as u8,
        );
        CopyToBufferFromBgTilemap(
            0,
            sSlot2TilemapBuffer,
            *data.at(4) as u8,
            *data.at(5) as u8,
            *data.at(6) as u8,
            *data.at(7) as u8,
        );
        ClearWindowTilemap(windowIds[0]);
        ClearWindowTilemap(windowIds[1]);
        gPartyMenu.action = PARTY_ACTION_SWITCHING;
        AnimatePartySlot(gPartyMenu.slotId as u8, 1);
        AnimatePartySlot(gPartyMenu.slotId2 as u8, 1);
        SlidePartyMenuBoxOneStep(taskId);
        gTasks[taskId].func = Some(Task_SlideSelectedSlotsOffscreen);
    }
}
pub(crate) unsafe extern "C" fn TryMovePartySlot(
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
    return TRUE;
}
pub(crate) unsafe extern "C" fn MoveAndBufferPartySlot(
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
pub(crate) unsafe extern "C" fn MovePartyMenuBoxSprites(menuBox: *mut PartyMenuBox, offset: i16) {
    gSprites[(*menuBox).pokeballSpriteId].x2 += offset * 8;
    gSprites[(*menuBox).itemSpriteId].x2 += offset * 8;
    gSprites[(*menuBox).monSpriteId].x2 += offset * 8;
    gSprites[(*menuBox).statusSpriteId].x2 += offset * 8;
}
pub(crate) unsafe extern "C" fn SlidePartyMenuBoxSpritesOneStep(taskId: u8) {
    let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
    if *data.at(10) != 0 {
        MovePartyMenuBoxSprites(sPartyMenuBoxes.at(gPartyMenu.slotId), *data.at(10));
    }
    if *data.at(11) != 0 {
        MovePartyMenuBoxSprites(sPartyMenuBoxes.at(gPartyMenu.slotId2), *data.at(11));
    }
}
pub(crate) unsafe extern "C" fn SlidePartyMenuBoxOneStep(taskId: u8) {
    let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
    if *data.at(10) != 0 {
        MoveAndBufferPartySlot(
            sSlot1TilemapBuffer as *mut c_void,
            *data + *data.at(8),
            *data.at(1),
            *data.at(2),
            *data.at(3),
            *data.at(10),
        );
    }
    if *data.at(11) != 0 {
        MoveAndBufferPartySlot(
            sSlot2TilemapBuffer as *mut c_void,
            *data.at(4) + *data.at(9),
            *data.at(5),
            *data.at(6),
            *data.at(7),
            *data.at(11),
        );
    }
    ScheduleBgCopyTilemapToVram(0);
}
pub(crate) unsafe extern "C" fn Task_SlideSelectedSlotsOffscreen(taskId: u8) {
    let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
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
        DisplayPartyPokemonData(gPartyMenu.slotId as u8);
        DisplayPartyPokemonData(gPartyMenu.slotId2 as u8);
        PutWindowTilemap((*sPartyMenuBoxes.at(gPartyMenu.slotId)).windowId);
        PutWindowTilemap((*sPartyMenuBoxes.at(gPartyMenu.slotId2)).windowId);
        CopyToBufferFromBgTilemap(
            0,
            sSlot1TilemapBuffer,
            *data as u8,
            *data.at(1) as u8,
            *data.at(2) as u8,
            *data.at(3) as u8,
        );
        CopyToBufferFromBgTilemap(
            0,
            sSlot2TilemapBuffer,
            *data.at(4) as u8,
            *data.at(5) as u8,
            *data.at(6) as u8,
            *data.at(7) as u8,
        );
        ClearWindowTilemap((*sPartyMenuBoxes.at(gPartyMenu.slotId)).windowId);
        ClearWindowTilemap((*sPartyMenuBoxes.at(gPartyMenu.slotId2)).windowId);
        gTasks[taskId].func = Some(Task_SlideSelectedSlotsOnscreen);
    }
}
pub(crate) unsafe extern "C" fn Task_SlideSelectedSlotsOnscreen(taskId: u8) {
    let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
    SlidePartyMenuBoxOneStep(taskId);
    SlidePartyMenuBoxSpritesOneStep(taskId);
    if *data.at(10) == 0 && *data.at(11) == 0 {
        PutWindowTilemap((*sPartyMenuBoxes.at(gPartyMenu.slotId)).windowId);
        PutWindowTilemap((*sPartyMenuBoxes.at(gPartyMenu.slotId2)).windowId);
        ScheduleBgCopyTilemapToVram(0);
        Free(sSlot1TilemapBuffer as *mut c_void);
        Free(sSlot2TilemapBuffer as *mut c_void);
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
pub(crate) unsafe extern "C" fn SwitchMenuBoxSprites(spriteIdPtr1: *mut u8, spriteIdPtr2: *mut u8) {
    let mut spriteIdBuffer: u8 = *spriteIdPtr1;
    let mut xBuffer1: u16 = 0;
    let mut yBuffer1: u16 = 0;
    let mut xBuffer2: u16 = 0;
    let mut yBuffer2: u16 = 0;
    *spriteIdPtr1 = *spriteIdPtr2;
    *spriteIdPtr2 = spriteIdBuffer;
    xBuffer1 = gSprites[*spriteIdPtr1].x as u16;
    yBuffer1 = gSprites[*spriteIdPtr1].y as u16;
    xBuffer2 = gSprites[*spriteIdPtr1].x2 as u16;
    yBuffer2 = gSprites[*spriteIdPtr1].y2 as u16;
    gSprites[*spriteIdPtr1].x = gSprites[*spriteIdPtr2].x;
    gSprites[*spriteIdPtr1].y = gSprites[*spriteIdPtr2].y;
    gSprites[*spriteIdPtr1].x2 = gSprites[*spriteIdPtr2].x2;
    gSprites[*spriteIdPtr1].y2 = gSprites[*spriteIdPtr2].y2;
    gSprites[*spriteIdPtr2].x = xBuffer1 as i16;
    gSprites[*spriteIdPtr2].y = yBuffer1 as i16;
    gSprites[*spriteIdPtr2].x2 = xBuffer2 as i16;
    gSprites[*spriteIdPtr2].y2 = yBuffer2 as i16;
}
pub(crate) unsafe extern "C" fn SwitchPartyMon() {
    let mut menuBoxes: CArray<*mut PartyMenuBox, 2> = zeroed();
    let mut mon1: *mut Pokemon = null_mut();
    let mut mon2: *mut Pokemon = null_mut();
    let mut monBuffer: *mut Pokemon = null_mut();
    menuBoxes[0] = sPartyMenuBoxes.at(gPartyMenu.slotId);
    menuBoxes[1] = sPartyMenuBoxes.at(gPartyMenu.slotId2);
    mon1 = &raw mut gPlayerParty[gPartyMenu.slotId];
    mon2 = &raw mut gPlayerParty[gPartyMenu.slotId2];
    monBuffer = Alloc(100) as *mut Pokemon;
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
pub(crate) unsafe extern "C" fn FinishTwoMonAction(taskId: u8) {
    PartyMenuRemoveWindow(&raw mut (*sPartyMenuInternal).windowId[1]);
    gPartyMenu.action = PARTY_ACTION_CHOOSE_MON;
    AnimatePartySlot(gPartyMenu.slotId as u8, 0);
    gPartyMenu.slotId = gPartyMenu.slotId2;
    AnimatePartySlot(gPartyMenu.slotId2 as u8, 1);
    DisplayPartyMenuStdMessage(PARTY_MSG_CHOOSE_MON);
    gTasks[taskId].func = Some(Task_HandleChooseMonInput);
}
pub(crate) unsafe extern "C" fn CursorCb_Cancel1(taskId: u8) {
    PlaySE(SE_SELECT);
    PartyMenuRemoveWindow(&raw mut (*sPartyMenuInternal).windowId[0]);
    PartyMenuRemoveWindow(&raw mut (*sPartyMenuInternal).windowId[1]);
    if gPartyMenu.menuType() == PARTY_MENU_TYPE_DAYCARE {
        DisplayPartyMenuStdMessage(PARTY_MSG_CHOOSE_MON_2);
    } else {
        DisplayPartyMenuStdMessage(PARTY_MSG_CHOOSE_MON);
    }
    gTasks[taskId].func = Some(Task_HandleChooseMonInput);
}
pub(crate) unsafe extern "C" fn CursorCb_Item(taskId: u8) {
    PlaySE(SE_SELECT);
    PartyMenuRemoveWindow(&raw mut (*sPartyMenuInternal).windowId[0]);
    PartyMenuRemoveWindow(&raw mut (*sPartyMenuInternal).windowId[1]);
    SetPartyMonSelectionActions(
        gPlayerParty.as_mut_ptr(),
        gPartyMenu.slotId as u8,
        ACTIONS_ITEM,
    );
    DisplaySelectionWindow(SELECTWINDOW_ITEM);
    DisplayPartyMenuStdMessage(PARTY_MSG_DO_WHAT_WITH_ITEM);
    gTasks[taskId].data[0] = 0xFF;
    gTasks[taskId].func = Some(Task_HandleSelectionMenuInput);
}
pub(crate) unsafe extern "C" fn CursorCb_Give(taskId: u8) {
    PlaySE(SE_SELECT);
    (*sPartyMenuInternal).exitCallback = Some(CB2_SelectBagItemToGive);
    Task_ClosePartyMenu(taskId);
}
pub(crate) unsafe extern "C" fn CB2_SelectBagItemToGive() {
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
pub(crate) unsafe extern "C" fn CB2_GiveHoldItem() {
    if gSpecialVar_ItemId == ITEM_NONE {
        InitPartyMenu(
            gPartyMenu.menuType(),
            KEEP_PARTY_LAYOUT,
            gPartyMenu.action,
            TRUE,
            PARTY_MSG_NONE,
            Some(Task_TryCreateSelectionWindow),
            gPartyMenu.exitCallback,
        );
    } else {
        sPartyMenuItemId =
            GetMonData2(&raw mut gPlayerParty[gPartyMenu.slotId], MON_DATA_HELD_ITEM) as u16;
        if sPartyMenuItemId != ITEM_NONE {
            InitPartyMenu(
                gPartyMenu.menuType(),
                KEEP_PARTY_LAYOUT,
                gPartyMenu.action,
                TRUE,
                PARTY_MSG_NONE,
                Some(Task_SwitchHoldItemsPrompt),
                gPartyMenu.exitCallback,
            );
        } else if ItemIsMail(gSpecialVar_ItemId) != 0 {
            RemoveBagItem(gSpecialVar_ItemId, 1);
            GiveItemToMon(&raw mut gPlayerParty[gPartyMenu.slotId], gSpecialVar_ItemId);
            CB2_WriteMailToGiveMon();
        } else {
            InitPartyMenu(
                gPartyMenu.menuType(),
                KEEP_PARTY_LAYOUT,
                gPartyMenu.action,
                TRUE,
                PARTY_MSG_NONE,
                Some(Task_GiveHoldItem),
                gPartyMenu.exitCallback,
            );
        }
    }
}
pub(crate) unsafe extern "C" fn Task_GiveHoldItem(taskId: u8) {
    let mut item: u16 = 0;
    if gPaletteFade.active() == 0 {
        item = gSpecialVar_ItemId;
        DisplayGaveHeldItemMessage(&raw mut gPlayerParty[gPartyMenu.slotId], item, 0, 0);
        GiveItemToMon(&raw mut gPlayerParty[gPartyMenu.slotId], item);
        RemoveBagItem(item, 1);
        gTasks[taskId].func = Some(Task_UpdateHeldItemSprite);
    }
}
pub(crate) unsafe extern "C" fn Task_SwitchHoldItemsPrompt(taskId: u8) {
    if gPaletteFade.active() == 0 {
        DisplayAlreadyHoldingItemSwitchMessage(
            &raw mut gPlayerParty[gPartyMenu.slotId],
            sPartyMenuItemId,
            TRUE,
        );
        gTasks[taskId].func = Some(Task_SwitchItemsYesNo);
    }
}
pub(crate) unsafe extern "C" fn Task_SwitchItemsYesNo(taskId: u8) {
    if IsPartyMenuTextPrinterActive() != TRUE {
        PartyMenuDisplayYesNoMenu();
        gTasks[taskId].func = Some(Task_HandleSwitchItemsYesNoInput);
    }
}
pub(crate) unsafe extern "C" fn Task_HandleSwitchItemsYesNoInput(taskId: u8) {
    'l1: {
        let sw1: i8 = Menu_ProcessInputNoWrapClearOnChoose();
        let mut fall = false;
        if sw1 == 0 {
            fall = true;
            RemoveBagItem(gSpecialVar_ItemId, 1);
            if AddBagItem(sPartyMenuItemId, 1) == FALSE {
                AddBagItem(gSpecialVar_ItemId, 1);
                BufferBagFullCantTakeItemMessage(sPartyMenuItemId);
                DisplayPartyMenuMessage(gStringVar4.as_mut_ptr(), FALSE);
                gTasks[taskId].func = Some(Task_ReturnToChooseMonAfterText);
            } else if ItemIsMail(gSpecialVar_ItemId) != 0 {
                GiveItemToMon(&raw mut gPlayerParty[gPartyMenu.slotId], gSpecialVar_ItemId);
                gTasks[taskId].func = Some(Task_WriteMailToGiveMonAfterText);
            } else {
                GiveItemToMon(&raw mut gPlayerParty[gPartyMenu.slotId], gSpecialVar_ItemId);
                DisplaySwitchedHeldItemMessage(gSpecialVar_ItemId, sPartyMenuItemId, TRUE);
                gTasks[taskId].func = Some(Task_UpdateHeldItemSprite);
            }
            break 'l1;
        }
        if sw1 == MENU_B_PRESSED {
            fall = true;
            PlaySE(SE_SELECT);
        }
        if fall || sw1 == 1 {
            fall = true;
            gTasks[taskId].func = Some(Task_ReturnToChooseMonAfterText);
            break 'l1;
        }
    }
}
pub(crate) unsafe extern "C" fn Task_WriteMailToGiveMonAfterText(taskId: u8) {
    if IsPartyMenuTextPrinterActive() != TRUE {
        (*sPartyMenuInternal).exitCallback = Some(CB2_WriteMailToGiveMon);
        Task_ClosePartyMenu(taskId);
    }
}
pub(crate) unsafe extern "C" fn CB2_WriteMailToGiveMon() {
    let mut mail: u8 = GetMonData2(&raw mut gPlayerParty[gPartyMenu.slotId], MON_DATA_MAIL) as u8;
    DoEasyChatScreen(
        EASY_CHAT_TYPE_MAIL,
        (*gSaveBlock1Ptr).mail[mail].words.as_mut_ptr(),
        Some(CB2_ReturnToPartyMenuFromWritingMail),
        EASY_CHAT_PERSON_DISPLAY_NONE,
    );
}
pub(crate) unsafe extern "C" fn CB2_ReturnToPartyMenuFromWritingMail() {
    let mut mon: *mut Pokemon = &raw mut gPlayerParty[gPartyMenu.slotId];
    let mut item: u16 = GetMonData2(mon, MON_DATA_HELD_ITEM) as u16;
    if gSpecialVar_Result == FALSE as u16 {
        TakeMailFromMon(mon);
        SetMonData(
            mon,
            MON_DATA_HELD_ITEM,
            &raw mut sPartyMenuItemId as *mut c_void,
        );
        RemoveBagItem(sPartyMenuItemId, 1);
        AddBagItem(item, 1);
        InitPartyMenu(
            gPartyMenu.menuType(),
            KEEP_PARTY_LAYOUT,
            gPartyMenu.action,
            TRUE,
            PARTY_MSG_CHOOSE_MON as u8,
            Some(Task_TryCreateSelectionWindow),
            gPartyMenu.exitCallback,
        );
    } else {
        InitPartyMenu(
            gPartyMenu.menuType(),
            KEEP_PARTY_LAYOUT,
            gPartyMenu.action,
            TRUE,
            PARTY_MSG_NONE,
            Some(Task_DisplayGaveMailFromPartyMessage),
            gPartyMenu.exitCallback,
        );
    }
}
pub(crate) unsafe extern "C" fn Task_DisplayGaveMailFromPartyMessage(taskId: u8) {
    if gPaletteFade.active() == 0 {
        if sPartyMenuItemId == ITEM_NONE {
            DisplayGaveHeldItemMessage(
                &raw mut gPlayerParty[gPartyMenu.slotId],
                gSpecialVar_ItemId,
                0,
                0,
            );
        } else {
            DisplaySwitchedHeldItemMessage(gSpecialVar_ItemId, sPartyMenuItemId, FALSE);
        }
        gTasks[taskId].func = Some(Task_UpdateHeldItemSprite);
    }
}
pub(crate) unsafe extern "C" fn Task_UpdateHeldItemSprite(taskId: u8) {
    let mut mon: *mut Pokemon = &raw mut gPlayerParty[gPartyMenu.slotId];
    if IsPartyMenuTextPrinterActive() != TRUE {
        UpdatePartyMonHeldItemSprite(mon, sPartyMenuBoxes.at(gPartyMenu.slotId));
        if gPartyMenu.menuType() == PARTY_MENU_TYPE_STORE_PYRAMID_HELD_ITEMS {
            if GetMonData2(mon, MON_DATA_HELD_ITEM) != ITEM_NONE as u32 {
                DisplayPartyPokemonDescriptionText(
                    PARTYBOX_DESC_HAVE,
                    sPartyMenuBoxes.at(gPartyMenu.slotId),
                    1,
                );
            } else {
                DisplayPartyPokemonDescriptionText(
                    PARTYBOX_DESC_DONT_HAVE,
                    sPartyMenuBoxes.at(gPartyMenu.slotId),
                    1,
                );
            }
        }
        Task_ReturnToChooseMonAfterText(taskId);
    }
}
pub(crate) unsafe extern "C" fn CursorCb_TakeItem(taskId: u8) {
    let mut mon: *mut Pokemon = &raw mut gPlayerParty[gPartyMenu.slotId];
    let mut item: u16 = GetMonData2(mon, MON_DATA_HELD_ITEM) as u16;
    PlaySE(SE_SELECT);
    PartyMenuRemoveWindow(&raw mut (*sPartyMenuInternal).windowId[0]);
    PartyMenuRemoveWindow(&raw mut (*sPartyMenuInternal).windowId[1]);
    match TryTakeMonItem(mon) {
        0 => {
            GetMonNickname(mon, gStringVar1.as_mut_ptr());
            StringExpandPlaceholders(
                gStringVar4.as_mut_ptr(),
                gText_PkmnNotHolding.as_ptr().cast_mut(),
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
    gTasks[taskId].func = Some(Task_UpdateHeldItemSprite);
}
pub(crate) unsafe extern "C" fn CursorCb_Toss(taskId: u8) {
    let mut mon: *mut Pokemon = &raw mut gPlayerParty[gPartyMenu.slotId];
    let mut item: u16 = GetMonData2(mon, MON_DATA_HELD_ITEM) as u16;
    PlaySE(SE_SELECT);
    PartyMenuRemoveWindow(&raw mut (*sPartyMenuInternal).windowId[0]);
    PartyMenuRemoveWindow(&raw mut (*sPartyMenuInternal).windowId[1]);
    if item == ITEM_NONE {
        GetMonNickname(mon, gStringVar1.as_mut_ptr());
        StringExpandPlaceholders(
            gStringVar4.as_mut_ptr(),
            gText_PkmnNotHolding.as_ptr().cast_mut(),
        );
        DisplayPartyMenuMessage(gStringVar4.as_mut_ptr(), TRUE);
        gTasks[taskId].func = Some(Task_UpdateHeldItemSprite);
    } else {
        CopyItemName(item, gStringVar1.as_mut_ptr());
        StringExpandPlaceholders(
            gStringVar4.as_mut_ptr(),
            gText_ThrowAwayItem.as_ptr().cast_mut(),
        );
        DisplayPartyMenuMessage(gStringVar4.as_mut_ptr(), TRUE);
        gTasks[taskId].func = Some(Task_TossHeldItemYesNo);
    }
}
pub(crate) unsafe extern "C" fn Task_TossHeldItemYesNo(taskId: u8) {
    if IsPartyMenuTextPrinterActive() != TRUE {
        PartyMenuDisplayYesNoMenu();
        gTasks[taskId].func = Some(Task_HandleTossHeldItemYesNoInput);
    }
}
pub(crate) unsafe extern "C" fn Task_HandleTossHeldItemYesNoInput(taskId: u8) {
    let mut mon: *mut Pokemon = &raw mut gPlayerParty[gPartyMenu.slotId];
    'l1: {
        let sw1: i8 = Menu_ProcessInputNoWrapClearOnChoose();
        let mut fall = false;
        if sw1 == 0 {
            fall = true;
            CopyItemName(
                GetMonData2(mon, MON_DATA_HELD_ITEM) as u16,
                gStringVar1.as_mut_ptr(),
            );
            StringExpandPlaceholders(
                gStringVar4.as_mut_ptr(),
                gText_ItemThrownAway.as_ptr().cast_mut(),
            );
            DisplayPartyMenuMessage(gStringVar4.as_mut_ptr(), FALSE);
            gTasks[taskId].func = Some(Task_TossHeldItem);
            break 'l1;
        }
        if sw1 == MENU_B_PRESSED {
            fall = true;
            PlaySE(SE_SELECT);
        }
        if fall || sw1 == 1 {
            fall = true;
            gTasks[taskId].func = Some(Task_ReturnToChooseMonAfterText);
            break 'l1;
        }
    }
}
pub(crate) unsafe extern "C" fn Task_TossHeldItem(taskId: u8) {
    let mut mon: *mut Pokemon = &raw mut gPlayerParty[gPartyMenu.slotId];
    if IsPartyMenuTextPrinterActive() != TRUE {
        let mut item: u16 = ITEM_NONE;
        SetMonData(mon, MON_DATA_HELD_ITEM, &raw mut item as *mut c_void);
        UpdatePartyMonHeldItemSprite(mon, sPartyMenuBoxes.at(gPartyMenu.slotId));
        DisplayPartyPokemonDescriptionText(
            PARTYBOX_DESC_DONT_HAVE,
            sPartyMenuBoxes.at(gPartyMenu.slotId),
            1,
        );
        gTasks[taskId].func = Some(Task_ReturnToChooseMonAfterText);
    }
}
pub(crate) unsafe extern "C" fn CursorCb_Mail(taskId: u8) {
    PlaySE(SE_SELECT);
    PartyMenuRemoveWindow(&raw mut (*sPartyMenuInternal).windowId[0]);
    PartyMenuRemoveWindow(&raw mut (*sPartyMenuInternal).windowId[1]);
    SetPartyMonSelectionActions(
        gPlayerParty.as_mut_ptr(),
        gPartyMenu.slotId as u8,
        ACTIONS_MAIL,
    );
    DisplaySelectionWindow(SELECTWINDOW_MAIL);
    DisplayPartyMenuStdMessage(PARTY_MSG_DO_WHAT_WITH_MAIL);
    gTasks[taskId].data[0] = 0xFF;
    gTasks[taskId].func = Some(Task_HandleSelectionMenuInput);
}
pub(crate) unsafe extern "C" fn CursorCb_Read(taskId: u8) {
    PlaySE(SE_SELECT);
    (*sPartyMenuInternal).exitCallback = Some(CB2_ReadHeldMail);
    Task_ClosePartyMenu(taskId);
}
pub(crate) unsafe extern "C" fn CB2_ReadHeldMail() {
    ReadMail(
        &raw mut (*gSaveBlock1Ptr).mail
            [GetMonData2(&raw mut gPlayerParty[gPartyMenu.slotId], MON_DATA_MAIL)],
        Some(CB2_ReturnToPartyMenuFromReadingMail),
        TRUE,
    );
}
pub(crate) unsafe extern "C" fn CB2_ReturnToPartyMenuFromReadingMail() {
    gPaletteFade.set_bufferTransferDisabled(TRUE as u16);
    InitPartyMenu(
        gPartyMenu.menuType(),
        KEEP_PARTY_LAYOUT,
        gPartyMenu.action,
        TRUE,
        PARTY_MSG_DO_WHAT_WITH_MON as u8,
        Some(Task_TryCreateSelectionWindow),
        gPartyMenu.exitCallback,
    );
}
pub(crate) unsafe extern "C" fn CursorCb_TakeMail(taskId: u8) {
    PlaySE(SE_SELECT);
    PartyMenuRemoveWindow(&raw mut (*sPartyMenuInternal).windowId[1]);
    PartyMenuRemoveWindow(&raw mut (*sPartyMenuInternal).windowId[0]);
    DisplayPartyMenuMessage(gText_SendMailToPC.as_ptr().cast_mut(), TRUE);
    gTasks[taskId].func = Some(Task_SendMailToPCYesNo);
}
pub(crate) unsafe extern "C" fn Task_SendMailToPCYesNo(taskId: u8) {
    if IsPartyMenuTextPrinterActive() != TRUE {
        PartyMenuDisplayYesNoMenu();
        gTasks[taskId].func = Some(Task_HandleSendMailToPCYesNoInput);
    }
}
pub(crate) unsafe extern "C" fn Task_HandleSendMailToPCYesNoInput(taskId: u8) {
    'l1: {
        let sw1: i8 = Menu_ProcessInputNoWrapClearOnChoose();
        let mut fall = false;
        if sw1 == 0 {
            fall = true;
            if TakeMailFromMonAndSave(&raw mut gPlayerParty[gPartyMenu.slotId]) != MAIL_NONE as u8 {
                DisplayPartyMenuMessage(gText_MailSentToPC.as_ptr().cast_mut(), FALSE);
                gTasks[taskId].func = Some(Task_UpdateHeldItemSprite);
            } else {
                DisplayPartyMenuMessage(gText_PCMailboxFull.as_ptr().cast_mut(), FALSE);
                gTasks[taskId].func = Some(Task_ReturnToChooseMonAfterText);
            }
            break 'l1;
        }
        if sw1 == MENU_B_PRESSED {
            fall = true;
            PlaySE(SE_SELECT);
        }
        if fall || sw1 == 1 {
            fall = true;
            DisplayPartyMenuMessage(gText_MailMessageWillBeLost.as_ptr().cast_mut(), TRUE);
            gTasks[taskId].func = Some(Task_LoseMailMessageYesNo);
            break 'l1;
        }
    }
}
pub(crate) unsafe extern "C" fn Task_LoseMailMessageYesNo(taskId: u8) {
    if IsPartyMenuTextPrinterActive() != TRUE {
        PartyMenuDisplayYesNoMenu();
        gTasks[taskId].func = Some(Task_HandleLoseMailMessageYesNoInput);
    }
}
pub(crate) unsafe extern "C" fn Task_HandleLoseMailMessageYesNoInput(taskId: u8) {
    let mut item: u16 = 0;
    'l1: {
        let sw1: i8 = Menu_ProcessInputNoWrapClearOnChoose();
        let mut fall = false;
        if sw1 == 0 {
            fall = true;
            item = GetMonData2(&raw mut gPlayerParty[gPartyMenu.slotId], MON_DATA_HELD_ITEM) as u16;
            if AddBagItem(item, 1) == 1 {
                TakeMailFromMon(&raw mut gPlayerParty[gPartyMenu.slotId]);
                DisplayPartyMenuMessage(gText_MailTakenFromPkmn.as_ptr().cast_mut(), FALSE);
                gTasks[taskId].func = Some(Task_UpdateHeldItemSprite);
            } else {
                BufferBagFullCantTakeItemMessage(item);
                DisplayPartyMenuMessage(gStringVar4.as_mut_ptr(), FALSE);
                gTasks[taskId].func = Some(Task_ReturnToChooseMonAfterText);
            }
            break 'l1;
        }
        if sw1 == MENU_B_PRESSED {
            fall = true;
            PlaySE(SE_SELECT);
        }
        if fall || sw1 == 1 {
            fall = true;
            gTasks[taskId].func = Some(Task_ReturnToChooseMonAfterText);
            break 'l1;
        }
    }
}
pub(crate) unsafe extern "C" fn CursorCb_Cancel2(taskId: u8) {
    let mut mon: *mut Pokemon = &raw mut gPlayerParty[gPartyMenu.slotId];
    PlaySE(SE_SELECT);
    PartyMenuRemoveWindow(&raw mut (*sPartyMenuInternal).windowId[0]);
    PartyMenuRemoveWindow(&raw mut (*sPartyMenuInternal).windowId[1]);
    SetPartyMonSelectionActions(
        gPlayerParty.as_mut_ptr(),
        gPartyMenu.slotId as u8,
        GetPartyMenuActionsType(mon),
    );
    if gPartyMenu.menuType() != PARTY_MENU_TYPE_STORE_PYRAMID_HELD_ITEMS {
        DisplaySelectionWindow(SELECTWINDOW_ACTIONS);
        DisplayPartyMenuStdMessage(PARTY_MSG_DO_WHAT_WITH_MON);
    } else {
        DisplaySelectionWindow(SELECTWINDOW_ITEM);
        CopyItemName(
            GetMonData2(mon, MON_DATA_HELD_ITEM) as u16,
            gStringVar2.as_mut_ptr(),
        );
        DisplayPartyMenuStdMessage(PARTY_MSG_ALREADY_HOLDING_ONE);
    }
    gTasks[taskId].data[0] = 0xFF;
    gTasks[taskId].func = Some(Task_HandleSelectionMenuInput);
}
pub(crate) unsafe extern "C" fn CursorCb_SendMon(taskId: u8) {
    PlaySE(SE_SELECT);
    PartyMenuRemoveWindow(&raw mut (*sPartyMenuInternal).windowId[0]);
    if TrySwitchInPokemon() == TRUE {
        Task_ClosePartyMenu(taskId);
    } else {
        PartyMenuRemoveWindow(&raw mut (*sPartyMenuInternal).windowId[1]);
        DisplayPartyMenuMessage(gStringVar4.as_mut_ptr(), TRUE);
        gTasks[taskId].func = Some(Task_ReturnToChooseMonAfterText);
    }
}
pub(crate) unsafe extern "C" fn CursorCb_Enter(taskId: u8) {
    let mut maxBattlers: u8 = 0;
    let mut i: u8 = 0;
    PartyMenuRemoveWindow(&raw mut (*sPartyMenuInternal).windowId[0]);
    PartyMenuRemoveWindow(&raw mut (*sPartyMenuInternal).windowId[1]);
    maxBattlers = GetMaxBattleEntries();
    i = 0;
    while i < maxBattlers {
        if gSelectedOrderFromParty[i] == 0 {
            PlaySE(SE_SELECT);
            gSelectedOrderFromParty[i] = gPartyMenu.slotId as u8 + 1;
            DisplayPartyPokemonDescriptionText(
                i + PARTYBOX_DESC_FIRST,
                sPartyMenuBoxes.at(gPartyMenu.slotId),
                1,
            );
            if i as i32 == maxBattlers as i32 - 1 {
                MoveCursorToConfirm();
            }
            DisplayPartyMenuStdMessage(PARTY_MSG_CHOOSE_MON);
            gTasks[taskId].func = Some(Task_HandleChooseMonInput);
            return;
        }
        i += 1;
    }
    ConvertIntToDecimalStringN(
        gStringVar1.as_mut_ptr(),
        maxBattlers as i32,
        STR_CONV_MODE_LEFT_ALIGN,
        1,
    );
    StringExpandPlaceholders(
        gStringVar4.as_mut_ptr(),
        gText_NoMoreThanVar1Pkmn.as_ptr().cast_mut(),
    );
    PlaySE(SE_FAILURE);
    DisplayPartyMenuMessage(gStringVar4.as_mut_ptr(), TRUE);
    gTasks[taskId].func = Some(Task_ReturnToChooseMonAfterText);
}
pub(crate) unsafe extern "C" fn MoveCursorToConfirm() {
    AnimatePartySlot(gPartyMenu.slotId as u8, 0);
    gPartyMenu.slotId = PARTY_SIZE as i8;
    AnimatePartySlot(gPartyMenu.slotId as u8, 1);
}
pub(crate) unsafe extern "C" fn CursorCb_NoEntry(taskId: u8) {
    let mut maxBattlers: u8 = 0;
    let mut i: u8 = 0;
    let mut j: u8 = 0;
    PlaySE(SE_SELECT);
    PartyMenuRemoveWindow(&raw mut (*sPartyMenuInternal).windowId[0]);
    PartyMenuRemoveWindow(&raw mut (*sPartyMenuInternal).windowId[1]);
    maxBattlers = GetMaxBattleEntries();
    i = 0;
    while i < maxBattlers {
        if gSelectedOrderFromParty[i] as i32 == gPartyMenu.slotId as i32 + 1 {
            j = i;
            while (j as i32) < maxBattlers as i32 - 1 {
                gSelectedOrderFromParty[j] = gSelectedOrderFromParty[j as i32 + 1];
                j += 1;
            }
            gSelectedOrderFromParty[j] = 0;
            break;
        }
        i += 1;
    }
    DisplayPartyPokemonDescriptionText(1, sPartyMenuBoxes.at(gPartyMenu.slotId), 1);
    i = 0;
    while (i as i32) < maxBattlers as i32 - 1 {
        if gSelectedOrderFromParty[i] != 0 {
            DisplayPartyPokemonDescriptionText(
                i + PARTYBOX_DESC_FIRST,
                sPartyMenuBoxes.at(gSelectedOrderFromParty[i] as i32 - 1),
                1,
            );
        }
        i += 1;
    }
    DisplayPartyMenuStdMessage(PARTY_MSG_CHOOSE_MON);
    gTasks[taskId].func = Some(Task_HandleChooseMonInput);
}
pub(crate) unsafe extern "C" fn CursorCb_Store(taskId: u8) {
    PlaySE(SE_SELECT);
    Task_ClosePartyMenu(taskId);
}
pub(crate) unsafe extern "C" fn CursorCb_Register(taskId: u8) {
    let mut species2: u16 = GetMonData2(
        &raw mut gPlayerParty[gPartyMenu.slotId],
        MON_DATA_SPECIES_OR_EGG,
    ) as u16;
    let mut species: u16 =
        GetMonData2(&raw mut gPlayerParty[gPartyMenu.slotId], MON_DATA_SPECIES) as u16;
    let mut isModernFatefulEncounter: u8 = GetMonData2(
        &raw mut gPlayerParty[gPartyMenu.slotId],
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
                gText_PkmnCantBeTradedNow.as_ptr().cast_mut(),
            );
        }
        CANT_REGISTER_EGG => {
            StringExpandPlaceholders(
                gStringVar4.as_mut_ptr(),
                gText_EggCantBeTradedNow.as_ptr().cast_mut(),
            );
        }
        _ => {
            PlaySE(SE_SELECT);
            Task_ClosePartyMenu(taskId);
            return;
        }
    }
    PlaySE(SE_FAILURE);
    PartyMenuRemoveWindow(&raw mut (*sPartyMenuInternal).windowId[0]);
    PartyMenuRemoveWindow(&raw mut (*sPartyMenuInternal).windowId[1]);
    StringAppend(
        gStringVar4.as_mut_ptr(),
        gText_PauseUntilPress.as_ptr().cast_mut(),
    );
    DisplayPartyMenuMessage(gStringVar4.as_mut_ptr(), TRUE);
    gTasks[taskId].func = Some(Task_ReturnToChooseMonAfterText);
}
pub(crate) unsafe extern "C" fn CursorCb_Trade1(taskId: u8) {
    let mut species2: u16 = GetMonData2(
        &raw mut gPlayerParty[gPartyMenu.slotId],
        MON_DATA_SPECIES_OR_EGG,
    ) as u16;
    let mut species: u16 =
        GetMonData2(&raw mut gPlayerParty[gPartyMenu.slotId], MON_DATA_SPECIES) as u16;
    let mut isModernFatefulEncounter: u8 = GetMonData2(
        &raw mut gPlayerParty[gPartyMenu.slotId],
        MON_DATA_MODERN_FATEFUL_ENCOUNTER,
    ) as u8;
    let mut stringId: u32 = GetUnionRoomTradeMessageId(
        *(GetHostRfuGameData() as *mut RfuGameCompatibilityData),
        gRfuPartnerCompatibilityData,
        species2,
        gUnionRoomOfferedSpecies,
        gUnionRoomRequestedMonType,
        species,
        isModernFatefulEncounter,
    ) as u32;
    if stringId != UR_TRADE_MSG_NONE {
        StringExpandPlaceholders(
            gStringVar4.as_mut_ptr(),
            sUnionRoomTradeMessages[stringId - 1],
        );
        PlaySE(SE_FAILURE);
        PartyMenuRemoveWindow(&raw mut (*sPartyMenuInternal).windowId[0]);
        PartyMenuRemoveWindow(&raw mut (*sPartyMenuInternal).windowId[1]);
        StringAppend(
            gStringVar4.as_mut_ptr(),
            gText_PauseUntilPress.as_ptr().cast_mut(),
        );
        DisplayPartyMenuMessage(gStringVar4.as_mut_ptr(), TRUE);
        gTasks[taskId].func = Some(Task_ReturnToChooseMonAfterText);
    } else {
        PlaySE(SE_SELECT);
        Task_ClosePartyMenu(taskId);
    }
}
pub(crate) unsafe extern "C" fn CursorCb_Trade2(taskId: u8) {
    PartyMenuRemoveWindow(&raw mut (*sPartyMenuInternal).windowId[0]);
    PartyMenuRemoveWindow(&raw mut (*sPartyMenuInternal).windowId[1]);
    match CanSpinTradeMon(gPlayerParty.as_mut_ptr(), gPartyMenu.slotId as u16) {
        CANT_TRADE_LAST_MON => {
            StringExpandPlaceholders(
                gStringVar4.as_mut_ptr(),
                gText_OnlyPkmnForBattle.as_ptr().cast_mut(),
            );
        }
        CANT_TRADE_NATIONAL => {
            StringExpandPlaceholders(
                gStringVar4.as_mut_ptr(),
                gText_PkmnCantBeTradedNow.as_ptr().cast_mut(),
            );
        }
        CANT_TRADE_EGG_YET => {
            StringExpandPlaceholders(
                gStringVar4.as_mut_ptr(),
                gText_EggCantBeTradedNow.as_ptr().cast_mut(),
            );
        }
        _ => {
            PlaySE(SE_SELECT);
            GetMonNickname(
                &raw mut gPlayerParty[gPartyMenu.slotId],
                gStringVar1.as_mut_ptr(),
            );
            StringExpandPlaceholders(
                gStringVar4.as_mut_ptr(),
                gJPText_AreYouSureYouWantToSpinTradeMon.as_ptr().cast_mut(),
            );
            DisplayPartyMenuMessage(gStringVar4.as_mut_ptr(), TRUE);
            gTasks[taskId].func = Some(Task_SpinTradeYesNo);
            return;
        }
    }
    PlaySE(SE_FAILURE);
    StringAppend(
        gStringVar4.as_mut_ptr(),
        gText_PauseUntilPress.as_ptr().cast_mut(),
    );
    DisplayPartyMenuMessage(gStringVar4.as_mut_ptr(), TRUE);
    gTasks[taskId].func = Some(Task_ReturnToChooseMonAfterText);
}
pub(crate) unsafe extern "C" fn Task_SpinTradeYesNo(taskId: u8) {
    if IsPartyMenuTextPrinterActive() != TRUE {
        PartyMenuDisplayYesNoMenu();
        gTasks[taskId].func = Some(Task_HandleSpinTradeYesNoInput);
    }
}
pub(crate) unsafe extern "C" fn Task_HandleSpinTradeYesNoInput(taskId: u8) {
    'l1: {
        let sw1: i8 = Menu_ProcessInputNoWrapClearOnChoose();
        let mut fall = false;
        if sw1 == 0 {
            fall = true;
            Task_ClosePartyMenu(taskId);
            break 'l1;
        }
        if sw1 == MENU_B_PRESSED {
            fall = true;
            PlaySE(SE_SELECT);
        }
        if fall || sw1 == 1 {
            fall = true;
            Task_ReturnToChooseMonAfterText(taskId);
            break 'l1;
        }
    }
}
pub(crate) unsafe extern "C" fn CursorCb_FieldMove(taskId: u8) {
    let mut fieldMove: u8 = (*sPartyMenuInternal).actions[Menu_GetCursorPos()] - MENU_FIELD_MOVES;
    let mut mapHeader: *mut MapHeader = null_mut();
    PlaySE(SE_SELECT);
    if sFieldMoveCursorCallbacks[fieldMove].fieldMoveFunc.is_none() {
        return;
    }
    PartyMenuRemoveWindow(&raw mut (*sPartyMenuInternal).windowId[0]);
    PartyMenuRemoveWindow(&raw mut (*sPartyMenuInternal).windowId[1]);
    if MenuHelpers_IsLinkActive() == TRUE || InUnionRoom() == TRUE as u32 {
        if fieldMove == FIELD_MOVE_MILK_DRINK || fieldMove == FIELD_MOVE_SOFT_BOILED {
            DisplayPartyMenuStdMessage(PARTY_MSG_CANT_USE_HERE);
        } else {
            DisplayPartyMenuStdMessage(sFieldMoveCursorCallbacks[fieldMove].msgId as u32);
        }
        gTasks[taskId].func = Some(Task_CancelAfterAorBPress);
    } else {
        if fieldMove <= FIELD_MOVE_WATERFALL
            && FlagGet(FLAG_BADGE01_GET as u16 + fieldMove as u16) != TRUE
        {
            DisplayPartyMenuMessage(gText_CantUseUntilNewBadge.as_ptr().cast_mut(), TRUE);
            gTasks[taskId].func = Some(Task_ReturnToChooseMonAfterText);
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
                        (*gSaveBlock1Ptr).lastHealLocation.mapGroup as u16,
                        (*gSaveBlock1Ptr).lastHealLocation.mapNum as u16,
                    );
                    GetMapNameGeneric(
                        gStringVar1.as_mut_ptr(),
                        (*mapHeader).regionMapSectionId as u16,
                    );
                    StringExpandPlaceholders(
                        gStringVar4.as_mut_ptr(),
                        gText_ReturnToHealingSpot.as_ptr().cast_mut(),
                    );
                    DisplayFieldMoveExitAreaMessage(taskId);
                    (*sPartyMenuInternal).data[0] = fieldMove as i16;
                }
                FIELD_MOVE_DIG => {
                    mapHeader = Overworld_GetMapHeaderByGroupAndId(
                        (*gSaveBlock1Ptr).escapeWarp.mapGroup as u16,
                        (*gSaveBlock1Ptr).escapeWarp.mapNum as u16,
                    );
                    GetMapNameGeneric(
                        gStringVar1.as_mut_ptr(),
                        (*mapHeader).regionMapSectionId as u16,
                    );
                    StringExpandPlaceholders(
                        gStringVar4.as_mut_ptr(),
                        gText_EscapeFromHere.as_ptr().cast_mut(),
                    );
                    DisplayFieldMoveExitAreaMessage(taskId);
                    (*sPartyMenuInternal).data[0] = fieldMove as i16;
                }
                FIELD_MOVE_FLY => {
                    gPartyMenu.exitCallback = Some(CB2_OpenFlyMap);
                    Task_ClosePartyMenu(taskId);
                }
                _ => {
                    gPartyMenu.exitCallback = Some(CB2_ReturnToField);
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
            gTasks[taskId].func = Some(Task_CancelAfterAorBPress);
        }
    }
}
pub(crate) unsafe extern "C" fn DisplayFieldMoveExitAreaMessage(taskId: u8) {
    DisplayPartyMenuMessage(gStringVar4.as_mut_ptr(), TRUE);
    gTasks[taskId].func = Some(Task_FieldMoveExitAreaYesNo);
}
pub(crate) unsafe extern "C" fn Task_FieldMoveExitAreaYesNo(taskId: u8) {
    if IsPartyMenuTextPrinterActive() != TRUE {
        PartyMenuDisplayYesNoMenu();
        gTasks[taskId].func = Some(Task_HandleFieldMoveExitAreaYesNoInput);
    }
}
pub(crate) unsafe extern "C" fn Task_HandleFieldMoveExitAreaYesNoInput(taskId: u8) {
    'l1: {
        let sw1: i8 = Menu_ProcessInputNoWrapClearOnChoose();
        let mut fall = false;
        if sw1 == 0 {
            fall = true;
            gPartyMenu.exitCallback = Some(CB2_ReturnToField);
            Task_ClosePartyMenu(taskId);
            break 'l1;
        }
        if sw1 == MENU_B_PRESSED {
            fall = true;
            PlaySE(SE_SELECT);
        }
        if fall || sw1 == 1 {
            fall = true;
            gFieldCallback2 = None;
            gPostMenuFieldCallback = None;
            Task_ReturnToChooseMonAfterText(taskId);
            break 'l1;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FieldCallback_PrepareFadeInFromMenu() -> u8 {
    FadeInFromBlack();
    CreateTask(Some(Task_FieldMoveWaitForFade), 8);
    return TRUE;
}
pub(crate) unsafe extern "C" fn Task_FieldMoveWaitForFade(taskId: u8) {
    if IsWeatherNotFadingIn() == TRUE {
        gFieldEffectArguments[0] = GetFieldMoveMonSpecies() as i32;
        gPostMenuFieldCallback.unwrap_unchecked()();
        DestroyTask(taskId);
    }
}
pub(crate) unsafe extern "C" fn GetFieldMoveMonSpecies() -> u16 {
    return GetMonData2(&raw mut gPlayerParty[gPartyMenu.slotId], MON_DATA_SPECIES) as u16;
}
pub(crate) unsafe extern "C" fn Task_CancelAfterAorBPress(taskId: u8) {
    if gMain.newKeys as i32 & A_BUTTON != 0 || gMain.newKeys as i32 & B_BUTTON != 0 {
        CursorCb_Cancel1(taskId);
    }
}
pub(crate) unsafe extern "C" fn DisplayCantUseFlashMessage() {
    if FlagGet(FLAG_SYS_USE_FLASH) == TRUE {
        DisplayPartyMenuStdMessage(PARTY_MSG_ALREADY_IN_USE);
    } else {
        DisplayPartyMenuStdMessage(PARTY_MSG_CANT_USE_HERE);
    }
}
pub(crate) unsafe extern "C" fn FieldCallback_Surf() {
    gFieldEffectArguments[0] = GetCursorSelectionMonId() as i32;
    FieldEffectStart(FLDEFF_USE_SURF);
}
pub(crate) unsafe extern "C" fn SetUpFieldMove_Surf() -> u8 {
    if PartyHasMonWithSurf() == TRUE && IsPlayerFacingSurfableFishableWater() == TRUE {
        gFieldCallback2 = Some(FieldCallback_PrepareFadeInFromMenu);
        gPostMenuFieldCallback = Some(FieldCallback_Surf);
        return TRUE;
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn DisplayCantUseSurfMessage() {
    if TestPlayerAvatarFlags(PLAYER_AVATAR_FLAG_SURFING) != 0 {
        DisplayPartyMenuStdMessage(PARTY_MSG_ALREADY_SURFING);
    } else {
        DisplayPartyMenuStdMessage(PARTY_MSG_CANT_SURF_HERE);
    }
}
pub(crate) unsafe extern "C" fn SetUpFieldMove_Fly() -> u8 {
    if Overworld_MapTypeAllowsTeleportAndFly(gMapHeader.mapType) == TRUE {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CB2_ReturnToPartyMenuFromFlyMap() {
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
pub(crate) unsafe extern "C" fn FieldCallback_Waterfall() {
    gFieldEffectArguments[0] = GetCursorSelectionMonId() as i32;
    FieldEffectStart(FLDEFF_USE_WATERFALL);
}
pub(crate) unsafe extern "C" fn SetUpFieldMove_Waterfall() -> u8 {
    let mut x: i16 = 0;
    let mut y: i16 = 0;
    GetXYCoordsOneStepInFrontOfPlayer(&raw mut x, &raw mut y);
    if MetatileBehavior_IsWaterfall(MapGridGetMetatileBehaviorAt(x as i32, y as i32) as u8) == TRUE
        && IsPlayerSurfingNorth() == TRUE
    {
        gFieldCallback2 = Some(FieldCallback_PrepareFadeInFromMenu);
        gPostMenuFieldCallback = Some(FieldCallback_Waterfall);
        return TRUE;
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn FieldCallback_Dive() {
    gFieldEffectArguments[0] = GetCursorSelectionMonId() as i32;
    FieldEffectStart(FLDEFF_USE_DIVE);
}
pub(crate) unsafe extern "C" fn SetUpFieldMove_Dive() -> u8 {
    gFieldEffectArguments[1] = TrySetDiveWarp() as i32;
    if gFieldEffectArguments[1] != 0 {
        gFieldCallback2 = Some(FieldCallback_PrepareFadeInFromMenu);
        gPostMenuFieldCallback = Some(FieldCallback_Dive);
        return TRUE;
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn CreatePartyMonIconSprite(
    mon: *mut Pokemon,
    menuBox: *mut PartyMenuBox,
    slot: u32,
) {
    let mut handleDeoxys: u32 = TRUE as u32;
    let mut species2: u16 = 0;
    if IsMultiBattle() == TRUE && gMain.inBattle() != 0 {
        handleDeoxys = (if sMultiBattlePartnersPartyMask[slot] as u32 ^ handleDeoxys != 0 {
            TRUE as i32
        } else {
            FALSE as i32
        }) as u32;
    }
    species2 = GetMonData2(mon, MON_DATA_SPECIES_OR_EGG) as u16;
    CreatePartyMonIconSpriteParameterized(
        species2,
        GetMonData2(mon, MON_DATA_PERSONALITY),
        menuBox,
        1,
        handleDeoxys,
    );
    UpdatePartyMonHPBar((*menuBox).monSpriteId, mon);
}
pub(crate) unsafe extern "C" fn CreatePartyMonIconSpriteParameterized(
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
        gSprites[(*menuBox).monSpriteId]
            .oam
            .set_priority(priority as u16);
    }
}
pub(crate) unsafe extern "C" fn UpdateHPBar(spriteId: u8, hp: u16, maxhp: u16) {
    match GetHPBarLevel(hp as i16, maxhp as i16) {
        HP_BAR_FULL => {
            SetPartyHPBarSprite(&raw mut gSprites[spriteId], 0);
        }
        HP_BAR_GREEN => {
            SetPartyHPBarSprite(&raw mut gSprites[spriteId], 1);
        }
        HP_BAR_YELLOW => {
            SetPartyHPBarSprite(&raw mut gSprites[spriteId], 2);
        }
        HP_BAR_RED => {
            SetPartyHPBarSprite(&raw mut gSprites[spriteId], 3);
        }
        _ => {
            SetPartyHPBarSprite(&raw mut gSprites[spriteId], 4);
        }
    }
}
pub(crate) unsafe extern "C" fn UpdatePartyMonHPBar(spriteId: u8, mon: *mut Pokemon) {
    UpdateHPBar(
        spriteId,
        GetMonData2(mon, MON_DATA_HP) as u16,
        GetMonData2(mon, MON_DATA_MAX_HP) as u16,
    );
}
pub(crate) unsafe extern "C" fn AnimateSelectedPartyIcon(spriteId: u8, animNum: u8) {
    gSprites[spriteId].data[0] = 0;
    if animNum == 0 {
        if gSprites[spriteId].x == 16 {
            gSprites[spriteId].x2 = 0;
            gSprites[spriteId].y2 = -4;
        } else {
            gSprites[spriteId].x2 = -4;
            gSprites[spriteId].y2 = 0;
        }
        gSprites[spriteId].callback = Some(SpriteCB_UpdatePartyMonIcon);
    } else {
        gSprites[spriteId].x2 = 0;
        gSprites[spriteId].y2 = 0;
        gSprites[spriteId].callback = Some(SpriteCB_BouncePartyMonIcon);
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_BouncePartyMonIcon(sprite: *mut Sprite) {
    let mut animCmd: u8 = UpdateMonIconFrame(sprite);
    if animCmd != 0 {
        if animCmd as i32 & 1 != 0 {
            (*sprite).y2 = -3;
        } else {
            (*sprite).y2 = 1;
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_UpdatePartyMonIcon(sprite: *mut Sprite) {
    UpdateMonIconFrame(sprite);
}
pub(crate) unsafe extern "C" fn CreatePartyMonHeldItemSprite(
    mon: *mut Pokemon,
    menuBox: *mut PartyMenuBox,
) {
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
pub(crate) unsafe extern "C" fn CreatePartyMonHeldItemSpriteParameterized(
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
        gSprites[(*menuBox).itemSpriteId].oam.set_priority(0);
        ShowOrHideHeldItemSprite(item, menuBox);
    }
}
pub(crate) unsafe extern "C" fn UpdatePartyMonHeldItemSprite(
    mon: *mut Pokemon,
    menuBox: *mut PartyMenuBox,
) {
    ShowOrHideHeldItemSprite(GetMonData2(mon, MON_DATA_HELD_ITEM) as u16, menuBox);
}
pub(crate) unsafe extern "C" fn ShowOrHideHeldItemSprite(item: u16, menuBox: *mut PartyMenuBox) {
    if item == ITEM_NONE {
        gSprites[(*menuBox).itemSpriteId].set_invisible(TRUE as u16);
    } else {
        if ItemIsMail(item) != 0 {
            StartSpriteAnim(&raw mut gSprites[(*menuBox).itemSpriteId], 1);
        } else {
            StartSpriteAnim(&raw mut gSprites[(*menuBox).itemSpriteId], 0);
        }
        gSprites[(*menuBox).itemSpriteId].set_invisible(FALSE as u16);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn LoadHeldItemIcons() {
    LoadSpriteSheet((&raw const *sSpriteSheet_HeldItem).cast_mut());
    LoadSpritePalette((&raw const *sSpritePalette_HeldItem).cast_mut());
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DrawHeldItemIconsForTrade(
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
                item = GetMonData2(&raw mut gPlayerParty[i], MON_DATA_HELD_ITEM) as u16;
                if item != ITEM_NONE {
                    CreateHeldItemSpriteForTrade(*partySpriteIds.at(i), ItemIsMail(item));
                }
                i += 1;
            }
        }
        TRADE_PARTNER => {
            i = 0;
            while i < *partyCounts.at(1) as u16 {
                item = GetMonData2(&raw mut gEnemyParty[i], MON_DATA_HELD_ITEM) as u16;
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
pub(crate) unsafe extern "C" fn CreateHeldItemSpriteForTrade(spriteId: u8, isMail: u8) {
    let mut subpriority: u8 = gSprites[spriteId].subpriority;
    let mut newSpriteId: u8 = CreateSprite(
        (&raw const *sSpriteTemplate_HeldItem).cast_mut(),
        250,
        170,
        subpriority - 1,
    );
    gSprites[newSpriteId].x2 = 4;
    gSprites[newSpriteId].y2 = 10;
    gSprites[newSpriteId].callback = Some(SpriteCB_HeldItem);
    gSprites[newSpriteId].data[7] = spriteId as i16;
    StartSpriteAnim(&raw mut gSprites[newSpriteId], isMail);
    gSprites[newSpriteId].callback.unwrap_unchecked()(&raw mut gSprites[newSpriteId]);
}
pub(crate) unsafe extern "C" fn SpriteCB_HeldItem(sprite: *mut Sprite) {
    let mut otherSpriteId: u8 = (*sprite).data[7] as u8;
    if gSprites[otherSpriteId].invisible() != 0 {
        (*sprite).set_invisible(TRUE as u16);
    } else {
        (*sprite).set_invisible(FALSE as u16);
        (*sprite).x = gSprites[otherSpriteId].x + gSprites[otherSpriteId].x2;
        (*sprite).y = gSprites[otherSpriteId].y + gSprites[otherSpriteId].y2;
    }
}
pub(crate) unsafe extern "C" fn CreatePartyMonPokeballSprite(
    mon: *mut Pokemon,
    menuBox: *mut PartyMenuBox,
) {
    if GetMonData2(mon, MON_DATA_SPECIES) != SPECIES_NONE as u32 {
        (*menuBox).pokeballSpriteId = CreateSprite(
            (&raw const *sSpriteTemplate_MenuPokeball).cast_mut(),
            *(*menuBox).spriteCoords.at(6) as i16,
            *(*menuBox).spriteCoords.at(7) as i16,
            8,
        );
    }
}
pub(crate) unsafe extern "C" fn CreatePartyMonPokeballSpriteParameterized(
    species: u16,
    menuBox: *mut PartyMenuBox,
) {
    if species != SPECIES_NONE {
        (*menuBox).pokeballSpriteId = CreateSprite(
            (&raw const *sSpriteTemplate_MenuPokeball).cast_mut(),
            *(*menuBox).spriteCoords.at(6) as i16,
            *(*menuBox).spriteCoords.at(7) as i16,
            8,
        );
        gSprites[(*menuBox).pokeballSpriteId].oam.set_priority(0);
    }
}
pub(crate) unsafe extern "C" fn CreatePokeballButtonSprite(x: u8, y: u8) -> u8 {
    let mut spriteId: u8 = CreateSprite(
        (&raw const *sSpriteTemplate_MenuPokeball).cast_mut(),
        x as i16,
        y as i16,
        8,
    );
    gSprites[spriteId].oam.set_priority(2);
    return spriteId;
}
pub(crate) unsafe extern "C" fn CreateSmallPokeballButtonSprite(x: u8, y: u8) -> u8 {
    return CreateSprite(
        (&raw const *sSpriteTemplate_MenuPokeballSmall).cast_mut(),
        x as i16,
        y as i16,
        8,
    );
}
pub(crate) unsafe extern "C" fn PartyMenuStartSpriteAnim(spriteId: u8, animNum: u8) {
    StartSpriteAnim(&raw mut gSprites[spriteId], animNum);
}
pub(crate) unsafe extern "C" fn SpriteCB_BounceConfirmCancelButton(
    spriteId: u8,
    spriteId2: u8,
    animNum: u8,
) {
    if animNum == 0 {
        StartSpriteAnim(&raw mut gSprites[spriteId], 2);
        StartSpriteAnim(&raw mut gSprites[spriteId2], 4);
        gSprites[spriteId].y2 = 0;
        gSprites[spriteId2].y2 = 0;
    } else {
        StartSpriteAnim(&raw mut gSprites[spriteId], 3);
        StartSpriteAnim(&raw mut gSprites[spriteId2], 5);
        gSprites[spriteId].y2 = -4;
        gSprites[spriteId2].y2 = 4;
    }
}
pub(crate) unsafe extern "C" fn LoadPartyMenuPokeballGfx() {
    LoadCompressedSpriteSheet((&raw const *sSpriteSheet_MenuPokeball).cast_mut());
    LoadCompressedSpriteSheet((&raw const *sSpriteSheet_MenuPokeballSmall).cast_mut());
    LoadCompressedSpritePalette((&raw const *sSpritePalette_MenuPokeball).cast_mut());
}
pub(crate) unsafe extern "C" fn CreatePartyMonStatusSprite(
    mon: *mut Pokemon,
    menuBox: *mut PartyMenuBox,
) {
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
pub(crate) unsafe extern "C" fn CreatePartyMonStatusSpriteParameterized(
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
        gSprites[(*menuBox).statusSpriteId].oam.set_priority(0);
    }
}
pub(crate) unsafe extern "C" fn SetPartyMonAilmentGfx(
    mon: *mut Pokemon,
    menuBox: *mut PartyMenuBox,
) {
    UpdatePartyMonAilmentGfx(GetMonAilment(mon), menuBox);
}
pub(crate) unsafe extern "C" fn UpdatePartyMonAilmentGfx(status: u8, menuBox: *mut PartyMenuBox) {
    match status {
        AILMENT_NONE | AILMENT_PKRS => {
            gSprites[(*menuBox).statusSpriteId].set_invisible(TRUE as u16);
        }
        _ => {
            StartSpriteAnim(&raw mut gSprites[(*menuBox).statusSpriteId], status - 1);
            gSprites[(*menuBox).statusSpriteId].set_invisible(FALSE as u16);
        }
    }
}
pub(crate) unsafe extern "C" fn LoadPartyMenuAilmentGfx() {
    LoadCompressedSpriteSheet((&raw const *sSpriteSheet_StatusIcons).cast_mut());
    LoadCompressedSpritePalette((&raw const *sSpritePalette_StatusIcons).cast_mut());
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CB2_ShowPartyMenuForItemUse() {
    let mut callback: Option<unsafe extern "C" fn()> = Some(CB2_ReturnToBagMenu);
    let mut partyLayout: u8 = 0;
    let mut menuType: u8 = 0;
    let mut i: u8 = 0;
    let mut msgId: u8 = 0;
    let mut task: Option<unsafe extern "C" fn(u8)> = None;
    if gMain.inBattle() != 0 {
        menuType = PARTY_MENU_TYPE_IN_BATTLE;
        partyLayout = GetPartyLayoutFromBattleType();
    } else {
        menuType = PARTY_MENU_TYPE_FIELD;
        partyLayout = PARTY_LAYOUT_SINGLE;
    }
    if GetItemEffectType(gSpecialVar_ItemId) == ITEM_EFFECT_SACRED_ASH {
        gPartyMenu.slotId = 0;
        i = 0;
        while i < PARTY_SIZE as u8 {
            if GetMonData2(&raw mut gPlayerParty[i], MON_DATA_SPECIES) != 0
                && GetMonData2(&raw mut gPlayerParty[i], MON_DATA_HP) == 0
            {
                gPartyMenu.slotId = i as i8;
                break;
            }
            i += 1;
        }
        task = Some(Task_SetSacredAshCB);
        msgId = PARTY_MSG_NONE;
    } else {
        if GetPocketByItemId(gSpecialVar_ItemId) == POCKET_TM_HM {
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
pub(crate) unsafe extern "C" fn CB2_ReturnToBagMenu() {
    if (CurrentBattlePyramidLocation() != 0) as i32 == 0 {
        GoToBagMenu(ITEMMENULOCATION_LAST, POCKETS_COUNT, None);
    } else {
        GoToBattlePyramidBagMenu(PYRAMIDBAG_LOC_PREV, gPyramidBagMenuState.exitCallback);
    }
}
pub(crate) unsafe extern "C" fn Task_SetSacredAshCB(taskId: u8) {
    if gPaletteFade.active() == 0 {
        if gPartyMenu.menuType() == PARTY_MENU_TYPE_IN_BATTLE {
            (*sPartyMenuInternal).exitCallback = Some(CB2_SetUpExitToBattleScreen);
        }
        gItemUseCB.unwrap_unchecked()(taskId, Some(Task_ClosePartyMenuAfterText));
    }
}
pub(crate) unsafe extern "C" fn IsHPRecoveryItem(item: u16) -> u8 {
    let mut effect: *mut u8 = null_mut();
    if item == ITEM_ENIGMA_BERRY {
        effect = (*gSaveBlock1Ptr).enigmaBerry.itemEffect.as_mut_ptr();
    } else {
        effect = gItemEffectTable[item as i32 - ITEM_POTION];
    }
    if *effect.at(4) as i32 & 0x4 != 0 {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn GetMedicineItemEffectMessage(item: u16) {
    match GetItemEffectType(item) {
        ITEM_EFFECT_CURE_POISON => {
            StringExpandPlaceholders(
                gStringVar4.as_mut_ptr(),
                gText_PkmnCuredOfPoison.as_ptr().cast_mut(),
            );
        }
        ITEM_EFFECT_CURE_SLEEP => {
            StringExpandPlaceholders(
                gStringVar4.as_mut_ptr(),
                gText_PkmnWokeUp2.as_ptr().cast_mut(),
            );
        }
        ITEM_EFFECT_CURE_BURN => {
            StringExpandPlaceholders(
                gStringVar4.as_mut_ptr(),
                gText_PkmnBurnHealed.as_ptr().cast_mut(),
            );
        }
        ITEM_EFFECT_CURE_FREEZE => {
            StringExpandPlaceholders(
                gStringVar4.as_mut_ptr(),
                gText_PkmnThawedOut.as_ptr().cast_mut(),
            );
        }
        ITEM_EFFECT_CURE_PARALYSIS => {
            StringExpandPlaceholders(
                gStringVar4.as_mut_ptr(),
                gText_PkmnCuredOfParalysis.as_ptr().cast_mut(),
            );
        }
        ITEM_EFFECT_CURE_CONFUSION => {
            StringExpandPlaceholders(
                gStringVar4.as_mut_ptr(),
                gText_PkmnSnappedOutOfConfusion.as_ptr().cast_mut(),
            );
        }
        ITEM_EFFECT_CURE_INFATUATION => {
            StringExpandPlaceholders(
                gStringVar4.as_mut_ptr(),
                gText_PkmnGotOverInfatuation.as_ptr().cast_mut(),
            );
        }
        ITEM_EFFECT_CURE_ALL_STATUS => {
            StringExpandPlaceholders(
                gStringVar4.as_mut_ptr(),
                gText_PkmnBecameHealthy.as_ptr().cast_mut(),
            );
        }
        ITEM_EFFECT_HP_EV => {
            StringCopy(gStringVar2.as_mut_ptr(), gText_HP3.as_ptr().cast_mut());
            StringExpandPlaceholders(
                gStringVar4.as_mut_ptr(),
                gText_PkmnBaseVar2StatIncreased.as_ptr().cast_mut(),
            );
        }
        ITEM_EFFECT_ATK_EV => {
            StringCopy(gStringVar2.as_mut_ptr(), gText_Attack3.as_ptr().cast_mut());
            StringExpandPlaceholders(
                gStringVar4.as_mut_ptr(),
                gText_PkmnBaseVar2StatIncreased.as_ptr().cast_mut(),
            );
        }
        ITEM_EFFECT_DEF_EV => {
            StringCopy(gStringVar2.as_mut_ptr(), gText_Defense3.as_ptr().cast_mut());
            StringExpandPlaceholders(
                gStringVar4.as_mut_ptr(),
                gText_PkmnBaseVar2StatIncreased.as_ptr().cast_mut(),
            );
        }
        ITEM_EFFECT_SPEED_EV => {
            StringCopy(gStringVar2.as_mut_ptr(), gText_Speed2.as_ptr().cast_mut());
            StringExpandPlaceholders(
                gStringVar4.as_mut_ptr(),
                gText_PkmnBaseVar2StatIncreased.as_ptr().cast_mut(),
            );
        }
        ITEM_EFFECT_SPATK_EV => {
            StringCopy(gStringVar2.as_mut_ptr(), gText_SpAtk3.as_ptr().cast_mut());
            StringExpandPlaceholders(
                gStringVar4.as_mut_ptr(),
                gText_PkmnBaseVar2StatIncreased.as_ptr().cast_mut(),
            );
        }
        ITEM_EFFECT_SPDEF_EV => {
            StringCopy(gStringVar2.as_mut_ptr(), gText_SpDef3.as_ptr().cast_mut());
            StringExpandPlaceholders(
                gStringVar4.as_mut_ptr(),
                gText_PkmnBaseVar2StatIncreased.as_ptr().cast_mut(),
            );
        }
        ITEM_EFFECT_PP_UP | ITEM_EFFECT_PP_MAX => {
            StringExpandPlaceholders(
                gStringVar4.as_mut_ptr(),
                gText_MovesPPIncreased.as_ptr().cast_mut(),
            );
        }
        ITEM_EFFECT_HEAL_PP => {
            StringExpandPlaceholders(
                gStringVar4.as_mut_ptr(),
                gText_PPWasRestored.as_ptr().cast_mut(),
            );
        }
        _ => {
            StringExpandPlaceholders(
                gStringVar4.as_mut_ptr(),
                gText_WontHaveEffect.as_ptr().cast_mut(),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn NotUsingHPEVItemOnShedinja(mon: *mut Pokemon, item: u16) -> u8 {
    if GetItemEffectType(item) == ITEM_EFFECT_HP_EV
        && GetMonData2(mon, MON_DATA_SPECIES) == SPECIES_SHEDINJA as u32
    {
        return FALSE;
    }
    return TRUE;
}
pub(crate) unsafe extern "C" fn IsItemFlute(item: u16) -> u8 {
    if item == ITEM_BLUE_FLUTE || item == ITEM_RED_FLUTE || item == ITEM_YELLOW_FLUTE {
        return TRUE;
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn ExecuteTableBasedItemEffect_(
    partyMonIndex: u8,
    item: u16,
    monMoveIndex: u8,
) -> u8 {
    if gMain.inBattle() != 0 {
        return ExecuteTableBasedItemEffect(
            &raw mut gPlayerParty[partyMonIndex],
            item,
            GetPartyIdFromBattleSlot(partyMonIndex),
            monMoveIndex,
        );
    } else {
        return ExecuteTableBasedItemEffect(
            &raw mut gPlayerParty[partyMonIndex],
            item,
            partyMonIndex,
            monMoveIndex,
        );
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ItemUseCB_Medicine(taskId: u8, task: Option<unsafe extern "C" fn(u8)>) {
    let mut hp: u16 = 0;
    let mut mon: *mut Pokemon = &raw mut gPlayerParty[gPartyMenu.slotId];
    let mut item: u16 = gSpecialVar_ItemId;
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
        cannotUse = ExecuteTableBasedItemEffect_(gPartyMenu.slotId as u8, item, 0);
    }
    if cannotUse != FALSE {
        gPartyMenuUseExitCallback = FALSE;
        PlaySE(SE_SELECT);
        DisplayPartyMenuMessage(gText_WontHaveEffect.as_ptr().cast_mut(), TRUE);
        ScheduleBgCopyTilemapToVram(2);
        gTasks[taskId].func = task;
    } else {
        gPartyMenuUseExitCallback = TRUE;
        if IsItemFlute(item) == 0 {
            PlaySE(SE_USE_ITEM);
            if gPartyMenu.action != PARTY_ACTION_REUSABLE_ITEM {
                RemoveBagItem(item, 1);
            }
        } else {
            PlaySE(SE_GLASS_FLUTE);
        }
        SetPartyMonAilmentGfx(mon, sPartyMenuBoxes.at(gPartyMenu.slotId));
        if gSprites[(*sPartyMenuBoxes.at(gPartyMenu.slotId)).statusSpriteId].invisible() != 0 {
            DisplayPartyPokemonLevelCheck(mon, sPartyMenuBoxes.at(gPartyMenu.slotId), 1);
        }
        if canHeal == TRUE {
            if hp == 0 {
                AnimatePartySlot(gPartyMenu.slotId as u8, 1);
            }
            PartyMenuModifyHP(
                taskId,
                gPartyMenu.slotId as u8,
                1,
                GetMonData2(mon, MON_DATA_HP) as i16 - hp as i16,
                Some(Task_DisplayHPRestoredMessage),
            );
            ResetHPTaskData(taskId, 0, hp as u32);
            return;
        } else {
            GetMonNickname(mon, gStringVar1.as_mut_ptr());
            GetMedicineItemEffectMessage(item);
            DisplayPartyMenuMessage(gStringVar4.as_mut_ptr(), TRUE);
            ScheduleBgCopyTilemapToVram(2);
            gTasks[taskId].func = task;
        }
    }
}
pub(crate) unsafe extern "C" fn Task_DisplayHPRestoredMessage(taskId: u8) {
    GetMonNickname(
        &raw mut gPlayerParty[gPartyMenu.slotId],
        gStringVar1.as_mut_ptr(),
    );
    StringExpandPlaceholders(
        gStringVar4.as_mut_ptr(),
        gText_PkmnHPRestoredByVar2.as_ptr().cast_mut(),
    );
    DisplayPartyMenuMessage(gStringVar4.as_mut_ptr(), FALSE);
    ScheduleBgCopyTilemapToVram(2);
    HandleBattleLowHpMusicChange();
    gTasks[taskId].func = Some(Task_ClosePartyMenuAfterText);
}
pub(crate) unsafe extern "C" fn Task_ClosePartyMenuAfterText(taskId: u8) {
    if IsPartyMenuTextPrinterActive() != TRUE {
        if gPartyMenuUseExitCallback == FALSE {
            (*sPartyMenuInternal).exitCallback = None;
        }
        Task_ClosePartyMenu(taskId);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ItemUseCB_ReduceEV(taskId: u8, task: Option<unsafe extern "C" fn(u8)>) {
    let mut mon: *mut Pokemon = &raw mut gPlayerParty[gPartyMenu.slotId];
    let mut item: u16 = gSpecialVar_ItemId;
    let mut effectType: u8 = GetItemEffectType(item);
    let mut friendship: u16 = GetMonData2(mon, MON_DATA_FRIENDSHIP) as u16;
    let mut ev: u16 = ItemEffectToMonEv(mon, effectType);
    let mut cannotUseEffect: u8 = ExecuteTableBasedItemEffect_(gPartyMenu.slotId as u8, item, 0);
    let mut newFriendship: u16 = GetMonData2(mon, MON_DATA_FRIENDSHIP) as u16;
    let mut newEv: u16 = ItemEffectToMonEv(mon, effectType);
    if cannotUseEffect != 0 || friendship == newFriendship && ev == newEv {
        gPartyMenuUseExitCallback = FALSE;
        PlaySE(SE_SELECT);
        DisplayPartyMenuMessage(gText_WontHaveEffect.as_ptr().cast_mut(), TRUE);
        ScheduleBgCopyTilemapToVram(2);
        gTasks[taskId].func = task;
    } else {
        gPartyMenuUseExitCallback = TRUE;
        PlaySE(SE_USE_ITEM);
        RemoveBagItem(item, 1);
        GetMonNickname(mon, gStringVar1.as_mut_ptr());
        ItemEffectToStatString(effectType, gStringVar2.as_mut_ptr());
        if friendship != newFriendship {
            if ev != newEv {
                StringExpandPlaceholders(
                    gStringVar4.as_mut_ptr(),
                    gText_PkmnFriendlyBaseVar2Fell.as_ptr().cast_mut(),
                );
            } else {
                StringExpandPlaceholders(
                    gStringVar4.as_mut_ptr(),
                    gText_PkmnFriendlyBaseVar2CantFall.as_ptr().cast_mut(),
                );
            }
        } else {
            StringExpandPlaceholders(
                gStringVar4.as_mut_ptr(),
                gText_PkmnAdoresBaseVar2Fell.as_ptr().cast_mut(),
            );
        }
        DisplayPartyMenuMessage(gStringVar4.as_mut_ptr(), TRUE);
        ScheduleBgCopyTilemapToVram(2);
        gTasks[taskId].func = task;
    }
}
pub(crate) unsafe extern "C" fn ItemEffectToMonEv(mon: *mut Pokemon, effectType: u8) -> u16 {
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
    return 0;
}
pub(crate) unsafe extern "C" fn ItemEffectToStatString(effectType: u8, dest: *mut u8) {
    match effectType {
        ITEM_EFFECT_HP_EV => {
            StringCopy(dest, gText_HP3.as_ptr().cast_mut());
        }
        ITEM_EFFECT_ATK_EV => {
            StringCopy(dest, gText_Attack3.as_ptr().cast_mut());
        }
        ITEM_EFFECT_DEF_EV => {
            StringCopy(dest, gText_Defense3.as_ptr().cast_mut());
        }
        ITEM_EFFECT_SPEED_EV => {
            StringCopy(dest, gText_Speed2.as_ptr().cast_mut());
        }
        ITEM_EFFECT_SPATK_EV => {
            StringCopy(dest, gText_SpAtk3.as_ptr().cast_mut());
        }
        ITEM_EFFECT_SPDEF_EV => {
            StringCopy(dest, gText_SpDef3.as_ptr().cast_mut());
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn ShowMoveSelectWindow(slot: u8) {
    let mut i: u8 = 0;
    let mut moveCount: u8 = 0;
    let mut fontId: u8 = FONT_NORMAL;
    let mut windowId: u8 = DisplaySelectionWindow(SELECTWINDOW_MOVES);
    let mut r#move: u16 = 0;
    i = 0;
    while i < MAX_MON_MOVES as u8 {
        r#move = GetMonData2(&raw mut gPlayerParty[slot], MON_DATA_MOVE1 + i as i32) as u16;
        AddTextPrinterParameterized(
            windowId,
            fontId,
            gMoveNames[r#move].as_ptr().cast_mut(),
            8,
            i * 16 + 1,
            TEXT_SKIP_DRAW,
            None,
        );
        if r#move != MOVE_NONE {
            moveCount += 1;
        }
        i += 1;
    }
    InitMenuInUpperLeftCornerNormal(windowId, moveCount, 0);
    ScheduleBgCopyTilemapToVram(2);
}
pub(crate) unsafe extern "C" fn Task_HandleWhichMoveInput(taskId: u8) {
    let mut input: i8 = Menu_ProcessInput();
    if input != MENU_NOTHING_CHOSEN {
        if input == MENU_B_PRESSED {
            PlaySE(SE_SELECT);
            ReturnToUseOnWhichMon(taskId);
        } else {
            PartyMenuRemoveWindow(&raw mut (*sPartyMenuInternal).windowId[1]);
            SetSelectedMoveForPPItem(taskId);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ItemUseCB_PPRecovery(taskId: u8, task: Option<unsafe extern "C" fn(u8)>) {
    let mut effect: *mut u8 = null_mut();
    let mut item: u16 = gSpecialVar_ItemId;
    if item == ITEM_ENIGMA_BERRY {
        effect = (*gSaveBlock1Ptr).enigmaBerry.itemEffect.as_mut_ptr();
    } else {
        effect = gItemEffectTable[item as i32 - ITEM_POTION];
    }
    if *effect.at(4) as i32 & ITEM4_HEAL_PP_ONE == 0 {
        gPartyMenu.data[0] = 0;
        TryUsePPItem(taskId);
    } else {
        PlaySE(SE_SELECT);
        DisplayPartyMenuStdMessage(PARTY_MSG_RESTORE_WHICH_MOVE);
        ShowMoveSelectWindow(gPartyMenu.slotId as u8);
        gTasks[taskId].func = Some(Task_HandleWhichMoveInput);
    }
}
pub(crate) unsafe extern "C" fn SetSelectedMoveForPPItem(taskId: u8) {
    PartyMenuRemoveWindow(&raw mut (*sPartyMenuInternal).windowId[0]);
    gPartyMenu.data[0] = Menu_GetCursorPos() as i16;
    TryUsePPItem(taskId);
}
pub(crate) unsafe extern "C" fn ReturnToUseOnWhichMon(taskId: u8) {
    gTasks[taskId].func = Some(Task_HandleChooseMonInput);
    (*sPartyMenuInternal).exitCallback = None;
    PartyMenuRemoveWindow(&raw mut (*sPartyMenuInternal).windowId[0]);
    DisplayPartyMenuStdMessage(PARTY_MSG_USE_ON_WHICH_MON as u32);
}
pub(crate) unsafe extern "C" fn TryUsePPItem(taskId: u8) {
    let mut r#move: u16 = MOVE_NONE;
    let mut moveSlot: *mut i16 = &raw mut gPartyMenu.data[0];
    let mut item: u16 = gSpecialVar_ItemId;
    let mut ptr: *mut PartyMenu = &raw mut gPartyMenu;
    let mut mon: *mut Pokemon = null_mut();
    if ExecuteTableBasedItemEffect_((*ptr).slotId as u8, item, *moveSlot as u8) != 0 {
        gPartyMenuUseExitCallback = FALSE;
        PlaySE(SE_SELECT);
        DisplayPartyMenuMessage(gText_WontHaveEffect.as_ptr().cast_mut(), TRUE);
        ScheduleBgCopyTilemapToVram(2);
        gTasks[taskId].func = Some(Task_ClosePartyMenuAfterText);
    } else {
        gPartyMenuUseExitCallback = TRUE;
        mon = &raw mut gPlayerParty[(*ptr).slotId];
        PlaySE(SE_USE_ITEM);
        RemoveBagItem(item, 1);
        r#move = GetMonData2(mon, MON_DATA_MOVE1 + *moveSlot as i32) as u16;
        StringCopy(
            gStringVar1.as_mut_ptr(),
            gMoveNames[r#move].as_ptr().cast_mut(),
        );
        GetMedicineItemEffectMessage(item);
        DisplayPartyMenuMessage(gStringVar4.as_mut_ptr(), TRUE);
        ScheduleBgCopyTilemapToVram(2);
        gTasks[taskId].func = Some(Task_ClosePartyMenuAfterText);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ItemUseCB_PPUp(taskId: u8, task: Option<unsafe extern "C" fn(u8)>) {
    PlaySE(SE_SELECT);
    DisplayPartyMenuStdMessage(PARTY_MSG_BOOST_PP_WHICH_MOVE);
    ShowMoveSelectWindow(gPartyMenu.slotId as u8);
    gTasks[taskId].func = Some(Task_HandleWhichMoveInput);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ItemIdToBattleMoveId(item: u16) -> u16 {
    let mut tmNumber: u16 = item - ITEM_TM01;
    return sTMHMMoves[tmNumber];
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsMoveHm(r#move: u16) -> u8 {
    let mut i: u8 = 0;
    i = 0;
    while i < NUM_HIDDEN_MACHINES {
        if sTMHMMoves[i as i32 + NUM_TECHNICAL_MACHINES] == r#move {
            return TRUE;
        }
        i += 1;
    }
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MonKnowsMove(mon: *mut Pokemon, r#move: u16) -> u8 {
    let mut i: u8 = 0;
    i = 0;
    while i < MAX_MON_MOVES as u8 {
        if GetMonData2(mon, MON_DATA_MOVE1 + i as i32) == r#move as u32 {
            return TRUE;
        }
        i += 1;
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn DisplayLearnMoveMessage(str: *mut u8) {
    StringExpandPlaceholders(gStringVar4.as_mut_ptr(), str);
    DisplayPartyMenuMessage(gStringVar4.as_mut_ptr(), TRUE);
    ScheduleBgCopyTilemapToVram(2);
}
pub(crate) unsafe extern "C" fn DisplayLearnMoveMessageAndClose(taskId: u8, str: *mut u8) {
    DisplayLearnMoveMessage(str);
    gTasks[taskId].func = Some(Task_ClosePartyMenuAfterText);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ItemUseCB_TMHM(taskId: u8, task: Option<unsafe extern "C" fn(u8)>) {
    let mut mon: *mut Pokemon = null_mut();
    let mut r#move: *mut i16 = null_mut();
    let mut item: u16 = 0;
    PlaySE(SE_SELECT);
    mon = &raw mut gPlayerParty[gPartyMenu.slotId];
    r#move = gPartyMenu.data.as_mut_ptr();
    item = gSpecialVar_ItemId;
    GetMonNickname(mon, gStringVar1.as_mut_ptr());
    *r#move = ItemIdToBattleMoveId(item) as i16;
    StringCopy(
        gStringVar2.as_mut_ptr(),
        gMoveNames[*r#move].as_ptr().cast_mut(),
    );
    *r#move.at(1) = 0;
    match CanMonLearnTMTutor(mon, item, 0) {
        CANNOT_LEARN_MOVE => {
            DisplayLearnMoveMessageAndClose(taskId, gText_PkmnCantLearnMove.as_ptr().cast_mut());
            return;
        }
        ALREADY_KNOWS_MOVE => {
            DisplayLearnMoveMessageAndClose(taskId, gText_PkmnAlreadyKnows.as_ptr().cast_mut());
            return;
        }
        _ => {}
    }
    if GiveMoveToMon(mon, *r#move as u16) != MON_HAS_MAX_MOVES {
        gTasks[taskId].func = Some(Task_LearnedMove);
    } else {
        DisplayLearnMoveMessage(gText_PkmnNeedsToReplaceMove.as_ptr().cast_mut());
        gTasks[taskId].func = Some(Task_ReplaceMoveYesNo);
    }
}
pub(crate) unsafe extern "C" fn Task_LearnedMove(taskId: u8) {
    let mut mon: *mut Pokemon = &raw mut gPlayerParty[gPartyMenu.slotId];
    let mut r#move: *mut i16 = &raw mut gPartyMenu.data[0];
    let mut item: u16 = gSpecialVar_ItemId;
    if *r#move.at(1) == 0 {
        AdjustFriendship(mon, FRIENDSHIP_EVENT_LEARN_TMHM);
        if item < ITEM_HM01 {
            RemoveBagItem(item, 1);
        }
    }
    GetMonNickname(mon, gStringVar1.as_mut_ptr());
    StringCopy(
        gStringVar2.as_mut_ptr(),
        gMoveNames[*r#move].as_ptr().cast_mut(),
    );
    StringExpandPlaceholders(
        gStringVar4.as_mut_ptr(),
        gText_PkmnLearnedMove3.as_ptr().cast_mut(),
    );
    DisplayPartyMenuMessage(gStringVar4.as_mut_ptr(), TRUE);
    ScheduleBgCopyTilemapToVram(2);
    gTasks[taskId].func = Some(Task_DoLearnedMoveFanfareAfterText);
}
pub(crate) unsafe extern "C" fn Task_DoLearnedMoveFanfareAfterText(taskId: u8) {
    if IsPartyMenuTextPrinterActive() != TRUE {
        PlayFanfare(MUS_LEVEL_UP);
        gTasks[taskId].func = Some(Task_LearnNextMoveOrClosePartyMenu);
    }
}
pub(crate) unsafe extern "C" fn Task_LearnNextMoveOrClosePartyMenu(taskId: u8) {
    if IsFanfareTaskInactive() != 0
        && (gMain.newKeys as i32 & A_BUTTON != 0 || gMain.newKeys as i32 & B_BUTTON != 0)
    {
        if gPartyMenu.data[1] == 1 {
            Task_TryLearningNextMove(taskId);
        } else {
            if gPartyMenu.data[1] == 2 {
                gSpecialVar_Result = TRUE as u16;
            }
            Task_ClosePartyMenu(taskId);
        }
    }
}
pub(crate) unsafe extern "C" fn Task_ReplaceMoveYesNo(taskId: u8) {
    if IsPartyMenuTextPrinterActive() != TRUE {
        PartyMenuDisplayYesNoMenu();
        gTasks[taskId].func = Some(Task_HandleReplaceMoveYesNoInput);
    }
}
pub(crate) unsafe extern "C" fn Task_HandleReplaceMoveYesNoInput(taskId: u8) {
    'l1: {
        let sw1: i8 = Menu_ProcessInputNoWrapClearOnChoose();
        let mut fall = false;
        if sw1 == 0 {
            fall = true;
            DisplayPartyMenuMessage(gText_WhichMoveToForget.as_ptr().cast_mut(), TRUE);
            gTasks[taskId].func = Some(Task_ShowSummaryScreenToForgetMove);
            break 'l1;
        }
        if sw1 == MENU_B_PRESSED {
            fall = true;
            PlaySE(SE_SELECT);
        }
        if fall || sw1 == 1 {
            fall = true;
            StopLearningMovePrompt(taskId);
            break 'l1;
        }
    }
}
pub(crate) unsafe extern "C" fn Task_ShowSummaryScreenToForgetMove(taskId: u8) {
    if IsPartyMenuTextPrinterActive() != TRUE {
        (*sPartyMenuInternal).exitCallback = Some(CB2_ShowSummaryScreenToForgetMove);
        Task_ClosePartyMenu(taskId);
    }
}
pub(crate) unsafe extern "C" fn CB2_ShowSummaryScreenToForgetMove() {
    ShowSelectMovePokemonSummaryScreen(
        gPlayerParty.as_mut_ptr(),
        gPartyMenu.slotId as u8,
        gPlayerPartyCount - 1,
        Some(CB2_ReturnToPartyMenuWhileLearningMove),
        gPartyMenu.data[0] as u16,
    );
}
pub(crate) unsafe extern "C" fn CB2_ReturnToPartyMenuWhileLearningMove() {
    InitPartyMenu(
        0,
        0,
        0,
        TRUE,
        PARTY_MSG_NONE,
        Some(Task_ReturnToPartyMenuWhileLearningMove),
        gPartyMenu.exitCallback,
    );
}
pub(crate) unsafe extern "C" fn Task_ReturnToPartyMenuWhileLearningMove(taskId: u8) {
    if gPaletteFade.active() == 0 {
        if GetMoveSlotToReplace() != MAX_MON_MOVES as u8 {
            DisplayPartyMenuForgotMoveMessage(taskId);
        } else {
            StopLearningMovePrompt(taskId);
        }
    }
}
pub(crate) unsafe extern "C" fn DisplayPartyMenuForgotMoveMessage(taskId: u8) {
    let mut mon: *mut Pokemon = &raw mut gPlayerParty[gPartyMenu.slotId];
    let mut r#move: u16 = GetMonData2(mon, MON_DATA_MOVE1 + GetMoveSlotToReplace() as i32) as u16;
    GetMonNickname(mon, gStringVar1.as_mut_ptr());
    StringCopy(
        gStringVar2.as_mut_ptr(),
        gMoveNames[r#move].as_ptr().cast_mut(),
    );
    DisplayLearnMoveMessage(gText_12PoofForgotMove.as_ptr().cast_mut());
    gTasks[taskId].func = Some(Task_PartyMenuReplaceMove);
}
pub(crate) unsafe extern "C" fn Task_PartyMenuReplaceMove(taskId: u8) {
    let mut mon: *mut Pokemon = null_mut();
    let mut r#move: u16 = 0;
    if IsPartyMenuTextPrinterActive() != TRUE {
        mon = &raw mut gPlayerParty[gPartyMenu.slotId];
        RemoveMonPPBonus(mon, GetMoveSlotToReplace());
        r#move = gPartyMenu.data[0] as u16;
        SetMonMoveSlot(mon, r#move, GetMoveSlotToReplace());
        Task_LearnedMove(taskId);
    }
}
pub(crate) unsafe extern "C" fn StopLearningMovePrompt(taskId: u8) {
    StringCopy(
        gStringVar2.as_mut_ptr(),
        gMoveNames[gPartyMenu.data[0]].as_ptr().cast_mut(),
    );
    StringExpandPlaceholders(
        gStringVar4.as_mut_ptr(),
        gText_StopLearningMove2.as_ptr().cast_mut(),
    );
    DisplayPartyMenuMessage(gStringVar4.as_mut_ptr(), TRUE);
    ScheduleBgCopyTilemapToVram(2);
    gTasks[taskId].func = Some(Task_StopLearningMoveYesNo);
}
pub(crate) unsafe extern "C" fn Task_StopLearningMoveYesNo(taskId: u8) {
    if IsPartyMenuTextPrinterActive() != TRUE {
        PartyMenuDisplayYesNoMenu();
        gTasks[taskId].func = Some(Task_HandleStopLearningMoveYesNoInput);
    }
}
pub(crate) unsafe extern "C" fn Task_HandleStopLearningMoveYesNoInput(taskId: u8) {
    let mut mon: *mut Pokemon = &raw mut gPlayerParty[gPartyMenu.slotId];
    'l1: {
        let sw1: i8 = Menu_ProcessInputNoWrapClearOnChoose();
        let mut fall = false;
        if sw1 == 0 {
            fall = true;
            GetMonNickname(mon, gStringVar1.as_mut_ptr());
            StringCopy(
                gStringVar2.as_mut_ptr(),
                gMoveNames[gPartyMenu.data[0]].as_ptr().cast_mut(),
            );
            StringExpandPlaceholders(
                gStringVar4.as_mut_ptr(),
                gText_MoveNotLearned.as_ptr().cast_mut(),
            );
            DisplayPartyMenuMessage(gStringVar4.as_mut_ptr(), TRUE);
            if gPartyMenu.data[1] == 1 {
                gTasks[taskId].func = Some(Task_TryLearningNextMoveAfterText);
            } else {
                if gPartyMenu.data[1] == 2 {
                    gSpecialVar_Result = FALSE as u16;
                }
                gTasks[taskId].func = Some(Task_ClosePartyMenuAfterText);
            }
            break 'l1;
        }
        if sw1 == MENU_B_PRESSED {
            fall = true;
            PlaySE(SE_SELECT);
        }
        if fall || sw1 == 1 {
            fall = true;
            GetMonNickname(mon, gStringVar1.as_mut_ptr());
            StringCopy(
                gStringVar2.as_mut_ptr(),
                gMoveNames[gPartyMenu.data[0]].as_ptr().cast_mut(),
            );
            DisplayLearnMoveMessage(gText_PkmnNeedsToReplaceMove.as_ptr().cast_mut());
            gTasks[taskId].func = Some(Task_ReplaceMoveYesNo);
            break 'l1;
        }
    }
}
pub(crate) unsafe extern "C" fn Task_TryLearningNextMoveAfterText(taskId: u8) {
    if IsPartyMenuTextPrinterActive() != TRUE {
        Task_TryLearningNextMove(taskId);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ItemUseCB_RareCandy(taskId: u8, task: Option<unsafe extern "C" fn(u8)>) {
    let mut mon: *mut Pokemon = &raw mut gPlayerParty[gPartyMenu.slotId];
    let mut ptr: *mut PartyMenuInternal = sPartyMenuInternal;
    let mut arrayPtr: *mut i16 = (*ptr).data.as_mut_ptr();
    let mut itemPtr: *mut u16 = &raw mut gSpecialVar_ItemId;
    let mut cannotUseEffect: u8 = 0;
    if GetMonData2(mon, MON_DATA_LEVEL) != MAX_LEVEL {
        BufferMonStatsToTaskData(mon, arrayPtr);
        cannotUseEffect = ExecuteTableBasedItemEffect_(gPartyMenu.slotId as u8, *itemPtr, 0);
        BufferMonStatsToTaskData(mon, &raw mut (*ptr).data[6]);
    } else {
        cannotUseEffect = TRUE;
    }
    PlaySE(SE_SELECT);
    if cannotUseEffect != 0 {
        gPartyMenuUseExitCallback = FALSE;
        DisplayPartyMenuMessage(gText_WontHaveEffect.as_ptr().cast_mut(), TRUE);
        ScheduleBgCopyTilemapToVram(2);
        gTasks[taskId].func = task;
    } else {
        gPartyMenuUseExitCallback = TRUE;
        PlayFanfareByFanfareNum(FANFARE_LEVEL_UP);
        UpdateMonDisplayInfoAfterRareCandy(gPartyMenu.slotId as u8, mon);
        RemoveBagItem(gSpecialVar_ItemId, 1);
        GetMonNickname(mon, gStringVar1.as_mut_ptr());
        ConvertIntToDecimalStringN(
            gStringVar2.as_mut_ptr(),
            GetMonData2(mon, MON_DATA_LEVEL) as i32,
            STR_CONV_MODE_LEFT_ALIGN,
            3,
        );
        StringExpandPlaceholders(
            gStringVar4.as_mut_ptr(),
            gText_PkmnElevatedToLvVar2.as_ptr().cast_mut(),
        );
        DisplayPartyMenuMessage(gStringVar4.as_mut_ptr(), TRUE);
        ScheduleBgCopyTilemapToVram(2);
        gTasks[taskId].func = Some(Task_DisplayLevelUpStatsPg1);
    }
}
pub(crate) unsafe extern "C" fn UpdateMonDisplayInfoAfterRareCandy(slot: u8, mon: *mut Pokemon) {
    SetPartyMonAilmentGfx(mon, sPartyMenuBoxes.at(slot));
    if gSprites[(*sPartyMenuBoxes.at(slot)).statusSpriteId].invisible() != 0 {
        DisplayPartyPokemonLevelCheck(mon, sPartyMenuBoxes.at(slot), 1);
    }
    DisplayPartyPokemonHPCheck(mon, sPartyMenuBoxes.at(slot), 1);
    DisplayPartyPokemonMaxHPCheck(mon, sPartyMenuBoxes.at(slot), 1);
    DisplayPartyPokemonHPBarCheck(mon, sPartyMenuBoxes.at(slot));
    UpdatePartyMonHPBar((*sPartyMenuBoxes.at(slot)).monSpriteId, mon);
    AnimatePartySlot(slot, 1);
    ScheduleBgCopyTilemapToVram(0);
}
pub(crate) unsafe extern "C" fn Task_DisplayLevelUpStatsPg1(taskId: u8) {
    if WaitFanfare(FALSE) != 0
        && IsPartyMenuTextPrinterActive() != 1
        && (gMain.newKeys as i32 & 0x0001 != 0 || gMain.newKeys as i32 & B_BUTTON != 0)
    {
        PlaySE(SE_SELECT);
        DisplayLevelUpStatsPg1(taskId);
        gTasks[taskId].func = Some(Task_DisplayLevelUpStatsPg2);
    }
}
pub(crate) unsafe extern "C" fn Task_DisplayLevelUpStatsPg2(taskId: u8) {
    if gMain.newKeys as i32 & A_BUTTON != 0 || gMain.newKeys as i32 & B_BUTTON != 0 {
        PlaySE(SE_SELECT);
        DisplayLevelUpStatsPg2(taskId);
        gTasks[taskId].func = Some(Task_TryLearnNewMoves);
    }
}
pub(crate) unsafe extern "C" fn DisplayLevelUpStatsPg1(taskId: u8) {
    let mut arrayPtr: *mut i16 = (*sPartyMenuInternal).data.as_mut_ptr();
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
pub(crate) unsafe extern "C" fn DisplayLevelUpStatsPg2(taskId: u8) {
    let mut arrayPtr: *mut i16 = (*sPartyMenuInternal).data.as_mut_ptr();
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
pub(crate) unsafe extern "C" fn Task_TryLearnNewMoves(taskId: u8) {
    let mut learnMove: u16 = 0;
    if WaitFanfare(FALSE) != 0
        && (gMain.newKeys as i32 & A_BUTTON != 0 || gMain.newKeys as i32 & B_BUTTON != 0)
    {
        RemoveLevelUpStatsWindow();
        learnMove = MonTryLearningNewMove(&raw mut gPlayerParty[gPartyMenu.slotId], TRUE);
        gPartyMenu.data[1] = 1;
        match learnMove {
            0 => {
                PartyMenuTryEvolution(taskId);
            }
            MON_HAS_MAX_MOVES => {
                DisplayMonNeedsToReplaceMove(taskId);
            }
            MON_ALREADY_KNOWS_MOVE => {
                gTasks[taskId].func = Some(Task_TryLearningNextMove);
            }
            _ => {
                DisplayMonLearnedMove(taskId, learnMove);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_TryLearningNextMove(taskId: u8) {
    let mut result: u16 = MonTryLearningNewMove(&raw mut gPlayerParty[gPartyMenu.slotId], FALSE);
    match result {
        0 => {
            PartyMenuTryEvolution(taskId);
        }
        MON_HAS_MAX_MOVES => {
            DisplayMonNeedsToReplaceMove(taskId);
        }
        MON_ALREADY_KNOWS_MOVE => {
            return;
        }
        _ => {
            DisplayMonLearnedMove(taskId, result);
        }
    }
}
pub(crate) unsafe extern "C" fn PartyMenuTryEvolution(taskId: u8) {
    let mut mon: *mut Pokemon = &raw mut gPlayerParty[gPartyMenu.slotId];
    let mut targetSpecies: u16 = GetEvolutionTargetSpecies(mon, EVO_MODE_NORMAL, ITEM_NONE);
    if targetSpecies != SPECIES_NONE {
        FreePartyPointers();
        gCB2_AfterEvolution = gPartyMenu.exitCallback;
        BeginEvolutionScene(mon, targetSpecies, TRUE, gPartyMenu.slotId as u8);
        DestroyTask(taskId);
    } else {
        gTasks[taskId].func = Some(Task_ClosePartyMenuAfterText);
    }
}
pub(crate) unsafe extern "C" fn DisplayMonNeedsToReplaceMove(taskId: u8) {
    GetMonNickname(
        &raw mut gPlayerParty[gPartyMenu.slotId],
        gStringVar1.as_mut_ptr(),
    );
    StringCopy(
        gStringVar2.as_mut_ptr(),
        gMoveNames[gMoveToLearn].as_ptr().cast_mut(),
    );
    StringExpandPlaceholders(
        gStringVar4.as_mut_ptr(),
        gText_PkmnNeedsToReplaceMove.as_ptr().cast_mut(),
    );
    DisplayPartyMenuMessage(gStringVar4.as_mut_ptr(), TRUE);
    ScheduleBgCopyTilemapToVram(2);
    gPartyMenu.data[0] = gMoveToLearn as i16;
    gTasks[taskId].func = Some(Task_ReplaceMoveYesNo);
}
pub(crate) unsafe extern "C" fn DisplayMonLearnedMove(taskId: u8, r#move: u16) {
    GetMonNickname(
        &raw mut gPlayerParty[gPartyMenu.slotId],
        gStringVar1.as_mut_ptr(),
    );
    StringCopy(
        gStringVar2.as_mut_ptr(),
        gMoveNames[r#move].as_ptr().cast_mut(),
    );
    StringExpandPlaceholders(
        gStringVar4.as_mut_ptr(),
        gText_PkmnLearnedMove3.as_ptr().cast_mut(),
    );
    DisplayPartyMenuMessage(gStringVar4.as_mut_ptr(), TRUE);
    ScheduleBgCopyTilemapToVram(2);
    gPartyMenu.data[0] = r#move as i16;
    gTasks[taskId].func = Some(Task_DoLearnedMoveFanfareAfterText);
}
pub(crate) unsafe extern "C" fn BufferMonStatsToTaskData(mon: *mut Pokemon, mut data: *mut i16) {
    *data = GetMonData2(mon, MON_DATA_MAX_HP) as i16;
    *data.at(1) = GetMonData2(mon, MON_DATA_ATK) as i16;
    *data.at(2) = GetMonData2(mon, MON_DATA_DEF) as i16;
    *data.at(4) = GetMonData2(mon, MON_DATA_SPATK) as i16;
    *data.at(5) = GetMonData2(mon, MON_DATA_SPDEF) as i16;
    *data.at(3) = GetMonData2(mon, MON_DATA_SPEED) as i16;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ItemUseCB_SacredAsh(taskId: u8, task: Option<unsafe extern "C" fn(u8)>) {
    (*sPartyMenuInternal).data[0] = FALSE as i16;
    (*sPartyMenuInternal).data[1] = FALSE as i16;
    (*sPartyMenuInternal).data[2] = gPartyMenu.slotId as i16;
    UseSacredAsh(taskId);
}
pub(crate) unsafe extern "C" fn UseSacredAsh(taskId: u8) {
    let mut mon: *mut Pokemon = &raw mut gPlayerParty[gPartyMenu.slotId];
    let mut hp: u16 = 0;
    if GetMonData2(mon, MON_DATA_SPECIES) == SPECIES_NONE as u32 {
        gTasks[taskId].func = Some(Task_SacredAshLoop);
        return;
    }
    hp = GetMonData2(mon, MON_DATA_HP) as u16;
    if ExecuteTableBasedItemEffect_(gPartyMenu.slotId as u8, gSpecialVar_ItemId, 0) != 0 {
        gTasks[taskId].func = Some(Task_SacredAshLoop);
        return;
    }
    PlaySE(SE_USE_ITEM);
    SetPartyMonAilmentGfx(mon, sPartyMenuBoxes.at(gPartyMenu.slotId));
    if gSprites[(*sPartyMenuBoxes.at(gPartyMenu.slotId)).statusSpriteId].invisible() != 0 {
        DisplayPartyPokemonLevelCheck(mon, sPartyMenuBoxes.at(gPartyMenu.slotId), 1);
    }
    AnimatePartySlot((*sPartyMenuInternal).data[2] as u8, 0);
    AnimatePartySlot(gPartyMenu.slotId as u8, 1);
    PartyMenuModifyHP(
        taskId,
        gPartyMenu.slotId as u8,
        1,
        GetMonData2(mon, MON_DATA_HP) as i16 - hp as i16,
        Some(Task_SacredAshDisplayHPRestored),
    );
    ResetHPTaskData(taskId, 0, hp as u32);
    (*sPartyMenuInternal).data[0] = TRUE as i16;
    (*sPartyMenuInternal).data[1] = TRUE as i16;
}
pub(crate) unsafe extern "C" fn Task_SacredAshLoop(taskId: u8) {
    if IsPartyMenuTextPrinterActive() != TRUE {
        if (*sPartyMenuInternal).data[0] == TRUE as i16 {
            (*sPartyMenuInternal).data[0] = FALSE as i16;
            (*sPartyMenuInternal).data[2] = gPartyMenu.slotId as i16;
        }
        if ({
            gPartyMenu.slotId += 1;
            gPartyMenu.slotId
        }) == PARTY_SIZE as i8
        {
            if (*sPartyMenuInternal).data[1] == FALSE as i16 {
                gPartyMenuUseExitCallback = FALSE;
                DisplayPartyMenuMessage(gText_WontHaveEffect.as_ptr().cast_mut(), TRUE);
                ScheduleBgCopyTilemapToVram(2);
            } else {
                gPartyMenuUseExitCallback = TRUE;
                RemoveBagItem(gSpecialVar_ItemId, 1);
            }
            gTasks[taskId].func = Some(Task_ClosePartyMenuAfterText);
            gPartyMenu.slotId = 0;
        } else {
            UseSacredAsh(taskId);
        }
    }
}
pub(crate) unsafe extern "C" fn Task_SacredAshDisplayHPRestored(taskId: u8) {
    GetMonNickname(
        &raw mut gPlayerParty[gPartyMenu.slotId],
        gStringVar1.as_mut_ptr(),
    );
    StringExpandPlaceholders(
        gStringVar4.as_mut_ptr(),
        gText_PkmnHPRestoredByVar2.as_ptr().cast_mut(),
    );
    DisplayPartyMenuMessage(gStringVar4.as_mut_ptr(), FALSE);
    ScheduleBgCopyTilemapToVram(2);
    gTasks[taskId].func = Some(Task_SacredAshLoop);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ItemUseCB_EvolutionStone(
    taskId: u8,
    task: Option<unsafe extern "C" fn(u8)>,
) {
    PlaySE(SE_SELECT);
    gCB2_AfterEvolution = gPartyMenu.exitCallback;
    if ExecuteTableBasedItemEffect_(gPartyMenu.slotId as u8, gSpecialVar_ItemId, 0) != 0 {
        gPartyMenuUseExitCallback = FALSE;
        DisplayPartyMenuMessage(gText_WontHaveEffect.as_ptr().cast_mut(), TRUE);
        ScheduleBgCopyTilemapToVram(2);
        gTasks[taskId].func = task;
    } else {
        RemoveBagItem(gSpecialVar_ItemId, 1);
        FreePartyPointers();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetItemEffectType(item: u16) -> u8 {
    let mut itemEffect: *mut u8 = null_mut();
    let mut statusCure: u32 = 0;
    if !(item >= ITEM_POTION as u16 && item <= ITEM_UNUSED_BERRY_3) {
        return ITEM_EFFECT_NONE;
    }
    if item == ITEM_ENIGMA_BERRY {
        itemEffect = (*gSaveBlock1Ptr).enigmaBerry.itemEffect.as_mut_ptr();
    } else {
        itemEffect = gItemEffectTable[item as i32 - ITEM_POTION];
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
    statusCure = *itemEffect.at(3) as u32 & ITEM3_STATUS_ALL as u32;
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
        return 0;
    }
}
pub(crate) unsafe extern "C" fn TryTutorSelectedMon(taskId: u8) {
    let mut mon: *mut Pokemon = null_mut();
    let mut r#move: *mut i16 = null_mut();
    if gPaletteFade.active() == 0 {
        mon = &raw mut gPlayerParty[gPartyMenu.slotId];
        r#move = &raw mut gPartyMenu.data[0];
        GetMonNickname(mon, gStringVar1.as_mut_ptr());
        gPartyMenu.data[0] = GetTutorMove(gSpecialVar_0x8005 as u8) as i16;
        StringCopy(
            gStringVar2.as_mut_ptr(),
            gMoveNames[gPartyMenu.data[0]].as_ptr().cast_mut(),
        );
        *r#move.at(1) = 2;
        match CanMonLearnTMTutor(mon, 0, gSpecialVar_0x8005 as u8) {
            CANNOT_LEARN_MOVE => {
                DisplayLearnMoveMessageAndClose(
                    taskId,
                    gText_PkmnCantLearnMove.as_ptr().cast_mut(),
                );
                return;
            }
            ALREADY_KNOWS_MOVE => {
                DisplayLearnMoveMessageAndClose(taskId, gText_PkmnAlreadyKnows.as_ptr().cast_mut());
                return;
            }
            _ => {
                if GiveMoveToMon(mon, gPartyMenu.data[0] as u16) != MON_HAS_MAX_MOVES {
                    Task_LearnedMove(taskId);
                    return;
                }
            }
        }
        DisplayLearnMoveMessage(gText_PkmnNeedsToReplaceMove.as_ptr().cast_mut());
        gTasks[taskId].func = Some(Task_ReplaceMoveYesNo);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CB2_PartyMenuFromStartMenu() {
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CB2_ChooseMonToGiveItem() {
    let mut callback: Option<unsafe extern "C" fn()> =
        if CurrentBattlePyramidLocation() == PYRAMID_LOCATION_NONE {
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
    gPartyMenu.bagItem = gSpecialVar_ItemId;
}
pub(crate) unsafe extern "C" fn TryGiveItemOrMailToSelectedMon(taskId: u8) {
    sPartyMenuItemId =
        GetMonData2(&raw mut gPlayerParty[gPartyMenu.slotId], MON_DATA_HELD_ITEM) as u16;
    if sPartyMenuItemId == ITEM_NONE {
        GiveItemOrMailToSelectedMon(taskId);
    } else if ItemIsMail(sPartyMenuItemId) != 0 {
        DisplayItemMustBeRemovedFirstMessage(taskId);
    } else {
        DisplayAlreadyHoldingItemSwitchMessage(
            &raw mut gPlayerParty[gPartyMenu.slotId],
            sPartyMenuItemId,
            TRUE,
        );
        gTasks[taskId].func = Some(Task_SwitchItemsFromBagYesNo);
    }
}
pub(crate) unsafe extern "C" fn GiveItemOrMailToSelectedMon(taskId: u8) {
    if ItemIsMail(gPartyMenu.bagItem) != 0 {
        RemoveItemToGiveFromBag(gPartyMenu.bagItem);
        (*sPartyMenuInternal).exitCallback = Some(CB2_WriteMailToGiveMonFromBag);
        Task_ClosePartyMenu(taskId);
    } else {
        GiveItemToSelectedMon(taskId);
    }
}
pub(crate) unsafe extern "C" fn GiveItemToSelectedMon(taskId: u8) {
    let mut item: u16 = 0;
    if gPaletteFade.active() == 0 {
        item = gPartyMenu.bagItem;
        DisplayGaveHeldItemMessage(&raw mut gPlayerParty[gPartyMenu.slotId], item, FALSE, 1);
        GiveItemToMon(&raw mut gPlayerParty[gPartyMenu.slotId], item);
        RemoveItemToGiveFromBag(item);
        gTasks[taskId].func = Some(Task_UpdateHeldItemSpriteAndClosePartyMenu);
    }
}
pub(crate) unsafe extern "C" fn Task_UpdateHeldItemSpriteAndClosePartyMenu(taskId: u8) {
    let mut slot: i8 = gPartyMenu.slotId;
    if IsPartyMenuTextPrinterActive() != TRUE {
        UpdatePartyMonHeldItemSprite(&raw mut gPlayerParty[slot], sPartyMenuBoxes.at(slot));
        Task_ClosePartyMenu(taskId);
    }
}
pub(crate) unsafe extern "C" fn CB2_WriteMailToGiveMonFromBag() {
    let mut mail: u8 = 0;
    GiveItemToMon(&raw mut gPlayerParty[gPartyMenu.slotId], gPartyMenu.bagItem);
    mail = GetMonData2(&raw mut gPlayerParty[gPartyMenu.slotId], MON_DATA_MAIL) as u8;
    DoEasyChatScreen(
        EASY_CHAT_TYPE_MAIL,
        (*gSaveBlock1Ptr).mail[mail].words.as_mut_ptr(),
        Some(CB2_ReturnToPartyOrBagMenuFromWritingMail),
        EASY_CHAT_PERSON_DISPLAY_NONE,
    );
}
pub(crate) unsafe extern "C" fn CB2_ReturnToPartyOrBagMenuFromWritingMail() {
    let mut mon: *mut Pokemon = &raw mut gPlayerParty[gPartyMenu.slotId];
    let mut item: u16 = GetMonData2(mon, MON_DATA_HELD_ITEM) as u16;
    if gSpecialVar_Result == FALSE as u16 {
        TakeMailFromMon(mon);
        SetMonData(
            mon,
            MON_DATA_HELD_ITEM,
            &raw mut sPartyMenuItemId as *mut c_void,
        );
        RemoveBagItem(sPartyMenuItemId, 1);
        ReturnGiveItemToBagOrPC(item);
        SetMainCallback2(gPartyMenu.exitCallback);
    } else {
        InitPartyMenu(
            gPartyMenu.menuType(),
            KEEP_PARTY_LAYOUT,
            gPartyMenu.action,
            TRUE,
            PARTY_MSG_NONE,
            Some(Task_DisplayGaveMailFromBagMessage),
            gPartyMenu.exitCallback,
        );
    }
}
pub(crate) unsafe extern "C" fn Task_DisplayGaveMailFromBagMessage(taskId: u8) {
    if gPaletteFade.active() == 0 {
        if sPartyMenuItemId != ITEM_NONE {
            DisplaySwitchedHeldItemMessage(gPartyMenu.bagItem, sPartyMenuItemId, FALSE);
        } else {
            DisplayGaveHeldItemMessage(
                &raw mut gPlayerParty[gPartyMenu.slotId],
                gPartyMenu.bagItem,
                FALSE,
                1,
            );
        }
        gTasks[taskId].func = Some(Task_UpdateHeldItemSpriteAndClosePartyMenu);
    }
}
pub(crate) unsafe extern "C" fn Task_SwitchItemsFromBagYesNo(taskId: u8) {
    if IsPartyMenuTextPrinterActive() != TRUE {
        PartyMenuDisplayYesNoMenu();
        gTasks[taskId].func = Some(Task_HandleSwitchItemsFromBagYesNoInput);
    }
}
pub(crate) unsafe extern "C" fn Task_HandleSwitchItemsFromBagYesNoInput(taskId: u8) {
    let mut item: u16 = 0;
    'l1: {
        let sw1: i8 = Menu_ProcessInputNoWrapClearOnChoose();
        let mut fall = false;
        if sw1 == 0 {
            fall = true;
            item = gPartyMenu.bagItem;
            RemoveItemToGiveFromBag(item);
            if AddBagItem(sPartyMenuItemId, 1) == FALSE {
                ReturnGiveItemToBagOrPC(item);
                BufferBagFullCantTakeItemMessage(sPartyMenuItemId);
                DisplayPartyMenuMessage(gStringVar4.as_mut_ptr(), FALSE);
                gTasks[taskId].func = Some(Task_UpdateHeldItemSpriteAndClosePartyMenu);
            } else if ItemIsMail(item) != 0 {
                (*sPartyMenuInternal).exitCallback = Some(CB2_WriteMailToGiveMonFromBag);
                Task_ClosePartyMenu(taskId);
            } else {
                GiveItemToMon(&raw mut gPlayerParty[gPartyMenu.slotId], item);
                DisplaySwitchedHeldItemMessage(item, sPartyMenuItemId, TRUE);
                gTasks[taskId].func = Some(Task_UpdateHeldItemSpriteAndClosePartyMenu);
            }
            break 'l1;
        }
        if sw1 == MENU_B_PRESSED {
            fall = true;
            PlaySE(SE_SELECT);
        }
        if fall || sw1 == 1 {
            fall = true;
            gTasks[taskId].func = Some(Task_UpdateHeldItemSpriteAndClosePartyMenu);
            break 'l1;
        }
    }
}
pub(crate) unsafe extern "C" fn DisplayItemMustBeRemovedFirstMessage(taskId: u8) {
    DisplayPartyMenuMessage(gText_RemoveMailBeforeItem.as_ptr().cast_mut(), TRUE);
    ScheduleBgCopyTilemapToVram(2);
    gTasks[taskId].func = Some(Task_UpdateHeldItemSpriteAndClosePartyMenu);
}
pub(crate) unsafe extern "C" fn RemoveItemToGiveFromBag(item: u16) {
    if gPartyMenu.action == PARTY_ACTION_GIVE_PC_ITEM {
        RemovePCItem(item as u8, 1);
    } else {
        RemoveBagItem(item, 1);
    }
}
pub(crate) unsafe extern "C" fn ReturnGiveItemToBagOrPC(item: u16) -> u8 {
    if gPartyMenu.action == PARTY_ACTION_GIVE_ITEM {
        return AddBagItem(item, 1);
    } else {
        return AddPCItem(item, 1);
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ChooseMonToGiveMailFromMailbox() {
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
pub(crate) unsafe extern "C" fn TryGiveMailToSelectedMon(taskId: u8) {
    let mut mon: *mut Pokemon = &raw mut gPlayerParty[gPartyMenu.slotId];
    let mut mail: *mut Mail = null_mut();
    gPartyMenuUseExitCallback = FALSE;
    mail = &raw mut (*gSaveBlock1Ptr).mail[gPlayerPCItemPageInfo.itemsAbove as i32
        + PARTY_SIZE
        + gPlayerPCItemPageInfo.cursorPos as i32];
    if GetMonData2(mon, MON_DATA_HELD_ITEM) != ITEM_NONE as u32 {
        DisplayPartyMenuMessage(gText_PkmnHoldingItemCantHoldMail.as_ptr().cast_mut(), TRUE);
    } else {
        GiveMailToMon(mon, mail);
        ClearMail(mail);
        DisplayPartyMenuMessage(gText_MailTransferredFromMailbox.as_ptr().cast_mut(), TRUE);
    }
    ScheduleBgCopyTilemapToVram(2);
    gTasks[taskId].func = Some(Task_UpdateHeldItemSpriteAndClosePartyMenu);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn InitChooseHalfPartyForBattle(unused: u8) {
    ClearSelectedPartyOrder();
    InitPartyMenu(
        PARTY_MENU_TYPE_CHOOSE_HALF,
        0,
        0,
        0,
        0,
        Some(Task_HandleChooseMonInput),
        gMain.savedCallback,
    );
    gPartyMenu.task = Some(Task_ValidateChosenHalfParty);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ClearSelectedPartyOrder() {
    memset(gSelectedOrderFromParty.as_mut_ptr(), 0, 4);
}
pub(crate) unsafe extern "C" fn GetPartySlotEntryStatus(slot: i8) -> u8 {
    if GetBattleEntryEligibility(&raw mut gPlayerParty[slot]) == FALSE {
        return 2;
    }
    if HasPartySlotAlreadyBeenSelected(slot as u8 + 1) == 1 {
        return 1;
    }
    return 0;
}
pub(crate) unsafe extern "C" fn GetBattleEntryEligibility(mon: *mut Pokemon) -> u8 {
    let mut i: u16 = 0;
    let mut species: u16 = 0;
    if GetMonData2(mon, MON_DATA_IS_EGG) != 0
        || GetMonData2(mon, MON_DATA_LEVEL) > GetBattleEntryLevelCap() as u32
        || (*gSaveBlock1Ptr).location.mapGroup == 26
            && (*gSaveBlock1Ptr).location.mapNum == 25
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
            while gFrontierBannedSpecies[i] != 0xFFFF {
                if gFrontierBannedSpecies[i] == species {
                    return FALSE;
                }
                i += 1;
            }
            return TRUE;
        }
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn CheckBattleEntriesAndGetMessage() -> u8 {
    let mut maxBattlers: u8 = 0;
    let mut i: u8 = 0;
    let mut j: u8 = 0;
    let mut facility: u8 = 0;
    let mut party: *mut Pokemon = gPlayerParty.as_mut_ptr();
    let mut minBattlers: u8 = GetMinBattleEntries();
    let mut order: *mut u8 = gSelectedOrderFromParty.as_mut_ptr();
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
    facility = VarGet(VAR_FRONTIER_FACILITY) as u8;
    if facility == FACILITY_UNION_ROOM as u8 || facility == FACILITY_MULTI_OR_EREADER as u8 {
        return 0xFF;
    }
    maxBattlers = GetMaxBattleEntries();
    i = 0;
    while (i as i32) < maxBattlers as i32 - 1 {
        let mut species: u16 =
            GetMonData2(party.at(*order.at(i) as i32 - 1), MON_DATA_SPECIES) as u16;
        let mut item: u16 =
            GetMonData2(party.at(*order.at(i) as i32 - 1), MON_DATA_HELD_ITEM) as u16;
        j = i + 1;
        while j < maxBattlers {
            if species as u32 == GetMonData2(party.at(*order.at(j) as i32 - 1), MON_DATA_SPECIES) {
                return PARTY_MSG_MONS_CANT_BE_SAME;
            }
            if item != ITEM_NONE
                && item as u32 == GetMonData2(party.at(*order.at(j) as i32 - 1), MON_DATA_HELD_ITEM)
            {
                return PARTY_MSG_NO_SAME_HOLD_ITEMS;
            }
            j += 1;
        }
        i += 1;
    }
    return 0xFF;
}
pub(crate) unsafe extern "C" fn HasPartySlotAlreadyBeenSelected(slot: u8) -> u8 {
    let mut i: u8 = 0;
    i = 0;
    while i < 4 {
        if gSelectedOrderFromParty[i] == slot {
            return TRUE;
        }
        i += 1;
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn Task_ValidateChosenHalfParty(taskId: u8) {
    let mut msgId: u8 = CheckBattleEntriesAndGetMessage();
    if msgId != 0xFF {
        PlaySE(SE_FAILURE);
        DisplayPartyMenuStdMessage(msgId as u32);
        gTasks[taskId].func = Some(Task_ContinueChoosingHalfParty);
    } else {
        PlaySE(SE_SELECT);
        Task_ClosePartyMenu(taskId);
    }
}
pub(crate) unsafe extern "C" fn Task_ContinueChoosingHalfParty(taskId: u8) {
    if gMain.newKeys as i32 & A_BUTTON != 0 || gMain.newKeys as i32 & B_BUTTON != 0 {
        PlaySE(SE_SELECT);
        DisplayPartyMenuStdMessage(PARTY_MSG_CHOOSE_MON);
        gTasks[taskId].func = Some(Task_HandleChooseMonInput);
    }
}
pub(crate) unsafe extern "C" fn GetMaxBattleEntries() -> u8 {
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
        return 0;
    }
}
pub(crate) unsafe extern "C" fn GetMinBattleEntries() -> u8 {
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
        return 0;
    }
}
pub(crate) unsafe extern "C" fn GetBattleEntryLevelCap() -> u8 {
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
        return 0;
    }
}
pub(crate) unsafe extern "C" fn GetFacilityCancelString() -> *mut u8 {
    let mut facilityNum: u8 = VarGet(VAR_FRONTIER_FACILITY) as u8;
    if !(facilityNum != FACILITY_UNION_ROOM as u8 && facilityNum != FACILITY_MULTI_OR_EREADER as u8)
    {
        return gText_CancelBattle.as_ptr().cast_mut();
    } else if facilityNum == FRONTIER_FACILITY_DOME as u8 && gSpecialVar_0x8005 == 2 {
        return gText_ReturnToWaitingRoom.as_ptr().cast_mut();
    } else {
        return gText_CancelChallenge.as_ptr().cast_mut();
    }
    #[allow(unreachable_code)]
    {
        return null_mut();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ChooseMonForTradingBoard(
    menuType: u8,
    callback: Option<unsafe extern "C" fn()>,
) {
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
pub unsafe extern "C" fn ChooseMonForMoveTutor() {
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
pub unsafe extern "C" fn ChooseMonForWirelessMinigame() {
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
pub(crate) unsafe extern "C" fn GetPartyLayoutFromBattleType() -> u8 {
    if IsDoubleBattle() == FALSE {
        return PARTY_LAYOUT_SINGLE;
    }
    if IsMultiBattle() == TRUE {
        return PARTY_LAYOUT_MULTI;
    }
    return PARTY_LAYOUT_DOUBLE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn OpenPartyMenuInBattle(partyAction: u8) {
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ChooseMonForInBattleItem() {
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
pub(crate) unsafe extern "C" fn GetPartyMenuActionsTypeInBattle(mon: *mut Pokemon) -> u8 {
    if GetMonData2(&raw mut gPlayerParty[1], MON_DATA_SPECIES) != 0
        && GetMonData2(mon, MON_DATA_IS_EGG) == 0
    {
        if gPartyMenu.action == PARTY_ACTION_SEND_OUT {
            return ACTIONS_SEND_OUT;
        }
        if gBattleTypeFlags & BATTLE_TYPE_ARENA == 0 {
            return ACTIONS_SHIFT;
        }
    }
    return ACTIONS_SUMMARY_ONLY as u8;
}
pub(crate) unsafe extern "C" fn TrySwitchInPokemon() -> u8 {
    let mut slot: u8 = GetCursorSelectionMonId();
    let mut newSlot: u8 = 0;
    let mut i: u8 = 0;
    if IsMultiBattle() == 1 && (slot == 1 || slot == 4 || slot == 5) {
        StringCopy(gStringVar1.as_mut_ptr(), GetTrainerPartnerName());
        StringExpandPlaceholders(
            gStringVar4.as_mut_ptr(),
            gText_CantSwitchWithAlly.as_ptr().cast_mut(),
        );
        return FALSE;
    }
    if GetMonData2(&raw mut gPlayerParty[slot], MON_DATA_HP) == 0 {
        GetMonNickname(&raw mut gPlayerParty[slot], gStringVar1.as_mut_ptr());
        StringExpandPlaceholders(
            gStringVar4.as_mut_ptr(),
            gText_PkmnHasNoEnergy.as_ptr().cast_mut(),
        );
        return FALSE;
    }
    i = 0;
    while i < gBattlersCount {
        if GetBattlerSide(i) == B_SIDE_PLAYER
            && GetPartyIdFromBattleSlot(slot) as u16 == gBattlerPartyIndexes[i]
        {
            GetMonNickname(&raw mut gPlayerParty[slot], gStringVar1.as_mut_ptr());
            StringExpandPlaceholders(
                gStringVar4.as_mut_ptr(),
                gText_PkmnAlreadyInBattle.as_ptr().cast_mut(),
            );
            return FALSE;
        }
        i += 1;
    }
    if GetMonData2(&raw mut gPlayerParty[slot], MON_DATA_IS_EGG) != 0 {
        StringExpandPlaceholders(
            gStringVar4.as_mut_ptr(),
            gText_EggCantBattle.as_ptr().cast_mut(),
        );
        return FALSE;
    }
    if GetPartyIdFromBattleSlot(slot) == (*gBattleStruct).prevSelectedPartySlot {
        GetMonNickname(&raw mut gPlayerParty[slot], gStringVar1.as_mut_ptr());
        StringExpandPlaceholders(
            gStringVar4.as_mut_ptr(),
            gText_PkmnAlreadySelected.as_ptr().cast_mut(),
        );
        return FALSE;
    }
    if gPartyMenu.action == PARTY_ACTION_ABILITY_PREVENTS {
        SetMonPreventsSwitchingString();
        return FALSE;
    }
    if gPartyMenu.action == PARTY_ACTION_CANT_SWITCH {
        let mut currBattler: u8 = gBattlerInMenuId;
        GetMonNickname(
            &raw mut gPlayerParty
                [GetPartyIdFromBattlePartyId(gBattlerPartyIndexes[currBattler] as u8)],
            gStringVar1.as_mut_ptr(),
        );
        StringExpandPlaceholders(
            gStringVar4.as_mut_ptr(),
            gText_PkmnCantSwitchOut.as_ptr().cast_mut(),
        );
        return FALSE;
    }
    gSelectedMonPartyId = GetPartyIdFromBattleSlot(slot);
    gPartyMenuUseExitCallback = TRUE;
    newSlot = GetPartyIdFromBattlePartyId(gBattlerPartyIndexes[gBattlerInMenuId] as u8);
    SwitchPartyMonSlots(newSlot, slot);
    SwapPartyPokemon(&raw mut gPlayerParty[newSlot], &raw mut gPlayerParty[slot]);
    return TRUE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BufferBattlePartyCurrentOrder() {
    BufferBattlePartyOrder(gBattlePartyCurrentOrder.as_mut_ptr(), GetPlayerFlankId());
}
pub(crate) unsafe extern "C" fn BufferBattlePartyOrder(mut partyBattleOrder: *mut u8, flankId: u8) {
    let mut partyIds: CArray<u8, 6> = zeroed();
    let mut i: i32 = 0;
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
        partyIds[0] = gBattlerPartyIndexes[GetBattlerAtPosition(B_POSITION_PLAYER_LEFT)] as u8;
        i = 0;
        while i < PARTY_SIZE {
            if i != partyIds[0] as i32 {
                partyIds[j] = i as u8;
                j += 1;
            }
            i += 1;
        }
    } else {
        j = 2;
        partyIds[0] = gBattlerPartyIndexes[GetBattlerAtPosition(B_POSITION_PLAYER_LEFT)] as u8;
        partyIds[1] = gBattlerPartyIndexes[GetBattlerAtPosition(B_POSITION_PLAYER_RIGHT)] as u8;
        i = 0;
        while i < PARTY_SIZE {
            if i != partyIds[0] as i32 && i != partyIds[1] as i32 {
                partyIds[j] = i as u8;
                j += 1;
            }
            i += 1;
        }
    }
    i = 0;
    while i < 3 {
        *partyBattleOrder.at(i) = partyIds[0 + i * 2] << 4 | partyIds[1 + i * 2];
        i += 1;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BufferBattlePartyCurrentOrderBySide(battler: u8, flankId: u8) {
    BufferBattlePartyOrderBySide(
        (*gBattleStruct).battlerPartyOrders[battler].as_mut_ptr(),
        flankId,
        battler,
    );
}
pub(crate) unsafe extern "C" fn BufferBattlePartyOrderBySide(
    mut partyBattleOrder: *mut u8,
    flankId: u8,
    battler: u8,
) {
    let mut partyIndexes: CArray<u8, 6> = zeroed();
    let mut i: i32 = 0;
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
        partyIndexes[0] = gBattlerPartyIndexes[leftBattler] as u8;
        i = 0;
        while i < PARTY_SIZE {
            if i != partyIndexes[0] as i32 {
                partyIndexes[j] = i as u8;
                j += 1;
            }
            i += 1;
        }
    } else {
        j = 2;
        partyIndexes[0] = gBattlerPartyIndexes[leftBattler] as u8;
        partyIndexes[1] = gBattlerPartyIndexes[rightBattler] as u8;
        i = 0;
        while i < PARTY_SIZE {
            if i != partyIndexes[0] as i32 && i != partyIndexes[1] as i32 {
                partyIndexes[j] = i as u8;
                j += 1;
            }
            i += 1;
        }
    }
    i = 0;
    while i < 3 {
        *partyBattleOrder.at(i) = partyIndexes[0 + i * 2] << 4 | partyIndexes[1 + i * 2];
        i += 1;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SwitchPartyOrderLinkMulti(battler: u8, slot: u8, slot2: u8) {
    let mut partyIds: CArray<u8, 6> = zeroed();
    let mut tempSlot: u8 = 0;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut partyBattleOrder: *mut u8 = null_mut();
    let mut partyIdBuffer: u8 = 0;
    if IsMultiBattle() != 0 {
        partyBattleOrder = (*gBattleStruct).battlerPartyOrders[battler].as_mut_ptr();
        i = {
            j = 0;
            j
        };
        while i < 3 {
            partyIds[j] = *partyBattleOrder.at(i) >> 4;
            j += 1;
            partyIds[j] = *partyBattleOrder.at(i) & 0xF;
            j += 1;
            i += 1;
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
pub(crate) unsafe extern "C" fn GetPartyIdFromBattleSlot(mut slot: u8) -> u8 {
    let mut modResult: u8 = slot & 1;
    let mut retVal: u8 = 0;
    slot = (slot as i32 / 2) as u8;
    if modResult != 0 {
        retVal = gBattlePartyCurrentOrder[slot] & 0xF;
    } else {
        retVal = gBattlePartyCurrentOrder[slot] >> 4;
    }
    return retVal;
}
pub(crate) unsafe extern "C" fn SetPartyIdAtBattleSlot(mut slot: u8, setVal: u8) {
    let mut modResult: u32 = slot as u32 & 1;
    slot = (slot as i32 / 2) as u8;
    if modResult != 0 {
        gBattlePartyCurrentOrder[slot] = gBattlePartyCurrentOrder[slot] & 0xF0 | setVal;
    } else {
        gBattlePartyCurrentOrder[slot] = gBattlePartyCurrentOrder[slot] & 0xF | setVal << 4;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SwitchPartyMonSlots(slot: u8, slot2: u8) {
    let mut partyId: u8 = GetPartyIdFromBattleSlot(slot);
    SetPartyIdAtBattleSlot(slot, GetPartyIdFromBattleSlot(slot2));
    SetPartyIdAtBattleSlot(slot2, partyId);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetPartyIdFromBattlePartyId(battlePartyId: u8) -> u8 {
    let mut i: u8 = 0;
    let mut j: u8 = 0;
    j = {
        i = 0;
        i
    };
    while i < 3 {
        if gBattlePartyCurrentOrder[i] >> 4 != battlePartyId {
            j += 1;
            if gBattlePartyCurrentOrder[i] as i32 & 0xF == battlePartyId as i32 {
                return j;
            }
        } else {
            return j;
        }
        j += 1;
        i += 1;
    }
    return 0;
}
pub(crate) unsafe extern "C" fn UpdatePartyToBattleOrder() {
    let mut partyBuffer: *mut Pokemon = Alloc(600) as *mut Pokemon;
    let mut i: u8 = 0;
    memcpy(
        partyBuffer as *mut u8,
        gPlayerParty.as_mut_ptr() as *mut u8,
        600,
    );
    i = 0;
    while i < PARTY_SIZE as u8 {
        memcpy(
            &raw mut gPlayerParty[GetPartyIdFromBattlePartyId(i)] as *mut u8,
            partyBuffer.at(i) as *mut u8,
            100,
        );
        i += 1;
    }
    Free(partyBuffer as *mut c_void);
}
pub(crate) unsafe extern "C" fn UpdatePartyToFieldOrder() {
    let mut partyBuffer: *mut Pokemon = Alloc(600) as *mut Pokemon;
    let mut i: u8 = 0;
    memcpy(
        partyBuffer as *mut u8,
        gPlayerParty.as_mut_ptr() as *mut u8,
        600,
    );
    i = 0;
    while i < PARTY_SIZE as u8 {
        memcpy(
            &raw mut gPlayerParty[GetPartyIdFromBattleSlot(i)] as *mut u8,
            partyBuffer.at(i) as *mut u8,
            100,
        );
        i += 1;
    }
    Free(partyBuffer as *mut c_void);
}
pub(crate) unsafe extern "C" fn SwitchAliveMonIntoLeadSlot() {
    let mut i: u8 = 0;
    let mut mon: *mut Pokemon = null_mut();
    let mut partyId: u8 = 0;
    i = 1;
    while i < PARTY_SIZE as u8 {
        mon = &raw mut gPlayerParty[GetPartyIdFromBattleSlot(i)];
        if GetMonData2(mon, MON_DATA_SPECIES) != 0 && GetMonData2(mon, MON_DATA_HP) != 0 {
            partyId = GetPartyIdFromBattleSlot(0);
            SwitchPartyMonSlots(0, i);
            SwapPartyPokemon(&raw mut gPlayerParty[partyId], mon);
            break;
        }
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn CB2_SetUpExitToBattleScreen() {
    SetMainCallback2(Some(CB2_SetUpReshowBattleScreenAfterMenu));
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ShowPartyMenuToShowcaseMultiBattleParty() {
    InitPartyMenu(
        PARTY_MENU_TYPE_MULTI_SHOWCASE,
        PARTY_LAYOUT_MULTI_SHOWCASE,
        0,
        0,
        PARTY_MSG_NONE,
        Some(Task_InitMultiPartnerPartySlideIn),
        gMain.savedCallback,
    );
}
pub(crate) unsafe extern "C" fn Task_InitMultiPartnerPartySlideIn(taskId: u8) {
    gTasks[taskId].data[0] = 256;
    SlideMultiPartyMenuBoxSpritesOneStep(taskId);
    ChangeBgX(2, 0x10000, BG_COORD_SET);
    gTasks[taskId].func = Some(Task_MultiPartnerPartySlideIn);
}
pub(crate) unsafe extern "C" fn Task_MultiPartnerPartySlideIn(taskId: u8) {
    let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
    let mut i: u8 = 0;
    if gPaletteFade.active() == 0 {
        *data -= 8;
        SlideMultiPartyMenuBoxSpritesOneStep(taskId);
        if *data == 0 {
            i = MULTI_PARTY_SIZE as u8;
            while i < PARTY_SIZE as u8 {
                if gMultiPartnerParty[i as i32 - MULTI_PARTY_SIZE].species != SPECIES_NONE {
                    AnimateSelectedPartyIcon((*sPartyMenuBoxes.at(i)).monSpriteId, 0);
                }
                i += 1;
            }
            PlaySE(SE_M_HARDEN);
            gTasks[taskId].func = Some(Task_WaitAfterMultiPartnerPartySlideIn);
        }
    }
}
pub(crate) unsafe extern "C" fn Task_WaitAfterMultiPartnerPartySlideIn(taskId: u8) {
    let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
    if ({
        *data += 1;
        *data
    }) == 256
    {
        Task_ClosePartyMenu(taskId);
    }
}
pub(crate) unsafe extern "C" fn MoveMultiPartyMenuBoxSprite(spriteId: u8, x: i16) {
    if x >= 0 {
        gSprites[spriteId].x2 = x;
    }
}
pub(crate) unsafe extern "C" fn SlideMultiPartyMenuBoxSpritesOneStep(taskId: u8) {
    let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
    let mut i: u8 = 0;
    i = MULTI_PARTY_SIZE as u8;
    while i < PARTY_SIZE as u8 {
        if gMultiPartnerParty[i as i32 - MULTI_PARTY_SIZE].species != SPECIES_NONE {
            MoveMultiPartyMenuBoxSprite((*sPartyMenuBoxes.at(i)).monSpriteId, *data - 8);
            MoveMultiPartyMenuBoxSprite((*sPartyMenuBoxes.at(i)).itemSpriteId, *data - 8);
            MoveMultiPartyMenuBoxSprite((*sPartyMenuBoxes.at(i)).pokeballSpriteId, *data - 8);
            MoveMultiPartyMenuBoxSprite((*sPartyMenuBoxes.at(i)).statusSpriteId, *data - 8);
        }
        i += 1;
    }
    ChangeBgX(2, 0x800, BG_COORD_ADD);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ChooseMonForDaycare() {
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
pub(crate) unsafe extern "C" fn ChoosePartyMonByMenuType(menuType: u8) {
    gFieldCallback2 = Some(CB2_FadeFromPartyMenu);
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
pub(crate) unsafe extern "C" fn BufferMonSelection() {
    gSpecialVar_0x8004 = GetCursorSelectionMonId() as u16;
    if gSpecialVar_0x8004 >= PARTY_SIZE as u16 {
        gSpecialVar_0x8004 = PARTY_NOTHING_CHOSEN;
    }
    gFieldCallback2 = Some(CB2_FadeFromPartyMenu);
    SetMainCallback2(Some(CB2_ReturnToField));
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CB2_FadeFromPartyMenu() -> u8 {
    FadeInFromBlack();
    CreateTask(Some(Task_PartyMenuWaitForFade), 10);
    return TRUE;
}
pub(crate) unsafe extern "C" fn Task_PartyMenuWaitForFade(taskId: u8) {
    if IsWeatherNotFadingIn() != 0 {
        DestroyTask(taskId);
        UnlockPlayerFieldControls();
        ScriptContext_Enable();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ChooseContestMon() {
    LockPlayerFieldControls();
    FadeScreen(FADE_TO_BLACK, 0);
    CreateTask(Some(Task_ChooseContestMon), 10);
}
pub(crate) unsafe extern "C" fn Task_ChooseContestMon(taskId: u8) {
    if gPaletteFade.active() == 0 {
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
pub(crate) unsafe extern "C" fn CB2_ChooseContestMon() {
    gContestMonPartyIndex = GetCursorSelectionMonId();
    if gContestMonPartyIndex >= PARTY_SIZE as u8 {
        gContestMonPartyIndex = PARTY_NOTHING_CHOSEN as u8;
    }
    gSpecialVar_0x8004 = gContestMonPartyIndex as u16;
    gFieldCallback2 = Some(CB2_FadeFromPartyMenu);
    SetMainCallback2(Some(CB2_ReturnToField));
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ChoosePartyMon() {
    LockPlayerFieldControls();
    FadeScreen(FADE_TO_BLACK, 0);
    CreateTask(Some(Task_ChoosePartyMon), 10);
}
pub(crate) unsafe extern "C" fn Task_ChoosePartyMon(taskId: u8) {
    if gPaletteFade.active() == 0 {
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
pub unsafe extern "C" fn ChooseMonForMoveRelearner() {
    LockPlayerFieldControls();
    FadeScreen(FADE_TO_BLACK, 0);
    CreateTask(Some(Task_ChooseMonForMoveRelearner), 10);
}
pub(crate) unsafe extern "C" fn Task_ChooseMonForMoveRelearner(taskId: u8) {
    if gPaletteFade.active() == 0 {
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
pub(crate) unsafe extern "C" fn CB2_ChooseMonForMoveRelearner() {
    gSpecialVar_0x8004 = GetCursorSelectionMonId() as u16;
    if gSpecialVar_0x8004 >= PARTY_SIZE as u16 {
        gSpecialVar_0x8004 = PARTY_NOTHING_CHOSEN;
    } else {
        gSpecialVar_0x8005 =
            GetNumberOfRelearnableMoves(&raw mut gPlayerParty[gSpecialVar_0x8004]) as u16;
    }
    gFieldCallback2 = Some(CB2_FadeFromPartyMenu);
    SetMainCallback2(Some(CB2_ReturnToField));
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DoBattlePyramidMonsHaveHeldItem() {
    let mut i: u8 = 0;
    gSpecialVar_Result = FALSE as u16;
    i = 0;
    while i < FRONTIER_PARTY_SIZE as u8 {
        if GetMonData2(&raw mut gPlayerParty[i], MON_DATA_HELD_ITEM) != ITEM_NONE as u32 {
            gSpecialVar_Result = TRUE as u16;
            break;
        }
        i += 1;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BattlePyramidChooseMonHeldItems() {
    LockPlayerFieldControls();
    FadeScreen(FADE_TO_BLACK, 0);
    CreateTask(Some(Task_BattlePyramidChooseMonHeldItems), 10);
}
pub(crate) unsafe extern "C" fn Task_BattlePyramidChooseMonHeldItems(taskId: u8) {
    if gPaletteFade.active() == 0 {
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
pub unsafe extern "C" fn MoveDeleterChooseMoveToForget() {
    ShowPokemonSummaryScreen(
        SUMMARY_MODE_SELECT_MOVE,
        gPlayerParty.as_mut_ptr() as *mut c_void,
        gSpecialVar_0x8004 as u8,
        gPlayerPartyCount - 1,
        Some(CB2_ReturnToField),
    );
    gFieldCallback = Some(FieldCB_ContinueScriptHandleMusic);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetNumMovesSelectedMonHas() {
    let mut i: u8 = 0;
    gSpecialVar_Result = 0;
    i = 0;
    while i < MAX_MON_MOVES as u8 {
        if GetMonData2(
            &raw mut gPlayerParty[gSpecialVar_0x8004],
            MON_DATA_MOVE1 + i as i32,
        ) != MOVE_NONE as u32
        {
            gSpecialVar_Result += 1;
        }
        i += 1;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BufferMoveDeleterNicknameAndMove() {
    let mut mon: *mut Pokemon = &raw mut gPlayerParty[gSpecialVar_0x8004];
    let mut r#move: u16 = GetMonData2(mon, MON_DATA_MOVE1 + gSpecialVar_0x8005 as i32) as u16;
    GetMonNickname(mon, gStringVar1.as_mut_ptr());
    StringCopy(
        gStringVar2.as_mut_ptr(),
        gMoveNames[r#move].as_ptr().cast_mut(),
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MoveDeleterForgetMove() {
    let mut i: u16 = 0;
    SetMonMoveSlot(
        &raw mut gPlayerParty[gSpecialVar_0x8004],
        MOVE_NONE,
        gSpecialVar_0x8005 as u8,
    );
    RemoveMonPPBonus(
        &raw mut gPlayerParty[gSpecialVar_0x8004],
        gSpecialVar_0x8005 as u8,
    );
    i = gSpecialVar_0x8005;
    while i < 3 {
        ShiftMoveSlot(
            &raw mut gPlayerParty[gSpecialVar_0x8004],
            i as u8,
            i as u8 + 1,
        );
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn ShiftMoveSlot(mon: *mut Pokemon, slotTo: u8, slotFrom: u8) {
    let mut move1: u16 = GetMonData2(mon, MON_DATA_MOVE1 + slotTo as i32) as u16;
    let mut move0: u16 = GetMonData2(mon, MON_DATA_MOVE1 + slotFrom as i32) as u16;
    let mut pp1: u8 = GetMonData2(mon, MON_DATA_PP1 + slotTo as i32) as u8;
    let mut pp0: u8 = GetMonData2(mon, MON_DATA_PP1 + slotFrom as i32) as u8;
    let mut ppBonuses: u8 = GetMonData2(mon, MON_DATA_PP_BONUSES) as u8;
    let mut ppBonusMask1: u8 = gPPUpGetMask[slotTo];
    let mut ppBonusMove1: u8 =
        shr_i32(ppBonuses as i32 & ppBonusMask1 as i32, slotTo as u32 * 2) as u8;
    let mut ppBonusMask2: u8 = gPPUpGetMask[slotFrom];
    let mut ppBonusMove2: u8 =
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
pub unsafe extern "C" fn IsSelectedMonEgg() {
    if GetMonData2(&raw mut gPlayerParty[gSpecialVar_0x8004], MON_DATA_IS_EGG) != 0 {
        gSpecialVar_Result = TRUE as u16;
    } else {
        gSpecialVar_Result = FALSE as u16;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsLastMonThatKnowsSurf() {
    let mut r#move: u16 = 0;
    let mut i: u32 = 0;
    let mut j: u32 = 0;
    gSpecialVar_Result = FALSE as u16;
    r#move = GetMonData2(
        &raw mut gPlayerParty[gSpecialVar_0x8004],
        MON_DATA_MOVE1 + gSpecialVar_0x8005 as i32,
    ) as u16;
    if r#move == MOVE_SURF {
        i = 0;
        while i < CalculatePlayerPartyCount() as u32 {
            if i != gSpecialVar_0x8004 as u32 {
                j = 0;
                while j < MAX_MON_MOVES as u32 {
                    if GetMonData2(&raw mut gPlayerParty[i], MON_DATA_MOVE1 + j as i32)
                        == MOVE_SURF as u32
                    {
                        return;
                    }
                    j += 1;
                }
            }
            i += 1;
        }
        if AnyStorageMonWithMove(r#move) != TRUE as u32 {
            gSpecialVar_Result = TRUE as u16;
        }
    }
}
