//! Translated from `src/naming_screen.c` by tools/rustport/c2rs.py.
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
    clippy::eq_op,
    clippy::manual_swap,
    clippy::missing_transmute_annotations,
    clippy::type_complexity,
    clippy::unnecessary_cast,
    clippy::useless_transmute,
    dead_code,
    unused_assignments,
    unused_variables
)]

use crate::agb_main::gMain;
use crate::agb_main::{
    SeedRngAndSetTrainerId, SetHBlankCallback, SetVBlankCallback, StartTimer1, gKeyRepeatStartDelay,
};
use crate::bg::{
    ChangeBgX, ChangeBgY, CopyBgTilemapBufferToVram, FillBgTilemapBufferRect_Palette0,
    ResetBgsAndClearDma3BusyFlags, ShowBg,
};
#[allow(unused_imports)]
use crate::c::*;
#[allow(unused_imports)]
use crate::consts::*;
use crate::event_data::{FlagGet, VarGet};
use crate::event_object_movement::CreateObjectGraphicsSprite;
use crate::field_effect::MultiplyInvertedPaletteRGBComponents;
use crate::field_player_avatar::GetRivalAvatarGraphicsIdByStateIdAndGender;
use crate::field_specials::{GetPCBoxToSendMon, IsDestinationBoxFull};
use crate::gpu_regs::{GetGpuReg, SetGpuReg, SetGpuRegBits};
use crate::load_save::gSaveBlock2Ptr;
use crate::menu::{
    AddTextPrinterParameterized2, AddTextPrinterParameterized3, DrawDialogueFrame,
    GetPlayerTextSpeedDelay, InitStandardTextBoxWindows, InitTextBoxGfxAndPrinters,
};
use crate::overworld::CB2_ReturnToFieldWithOpenMenu;
use crate::palette::{
    BeginNormalPaletteFade, BlendPalettes, LoadPalette, ResetPaletteFade, TransferPlttBuffer,
    UpdatePaletteFade, gPaletteFade,
};
use crate::palette::{gPlttBufferFaded, gPlttBufferUnfaded};
use crate::pokemon::CalculatePlayerPartyCount;
use crate::pokemon_icon::{CreateMonIcon, LoadMonIconPalettes};
use crate::pokemon_storage_system::GetBoxNamePtr;
use crate::sound::PlaySE;
use crate::sprite::gSprites;
use crate::sprite::{
    AnimateSprites, BuildOamBuffer, FreeAllSpritePalettes, GetSpriteTileStartByTag,
    IndexOfSpritePaletteTag, LoadOam, ProcessSpriteCopyRequests, ResetSpriteData,
};
use crate::string_util::{gStringVar1, gStringVar2, gStringVar3, gStringVar4};
use crate::task::{DestroyTask, ResetTasks, RunTasks};
use crate::task::{gTasks, task_get, task_set};
use crate::text::{IsTextPrinterActive, RunTextPrinters};
use crate::trig::Sin;
#[allow(unused_imports)]
use crate::types::*;
use crate::window::{
    CopyWindowToVram, FillWindowPixelBuffer, FreeAllWindowBuffers, PutWindowTilemap,
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
/// `CopyToBgTilemapBuffer` with this module's view of its types.
#[inline]
unsafe fn CopyToBgTilemapBuffer(a0: u8, a1: *mut c_void, a2: u16, a3: u16) {
    unsafe {
        crate::bg::CopyToBgTilemapBuffer(a0, a1 as _, a2, a3);
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
/// `FindTaskIdByFunc` with this module's view of its types.
#[inline]
unsafe fn FindTaskIdByFunc(a0: Option<unsafe fn(u8)>) -> u8 {
    unsafe { crate::task::FindTaskIdByFunc(core::mem::transmute(a0)) }
}
/// `Free` with this module's view of its types.
#[inline]
unsafe fn Free(a0: *mut c_void) {
    unsafe {
        crate::malloc::Free(a0 as _);
    }
}
/// `InitBgsFromTemplates` with this module's view of its types.
#[inline]
unsafe fn InitBgsFromTemplates(a0: u8, a1: *mut BgTemplate, a2: u8) {
    unsafe {
        crate::bg::InitBgsFromTemplates(a0, a1 as _, a2);
    }
}
/// `LoadBgTiles` with this module's view of its types.
#[inline]
unsafe fn LoadBgTiles(a0: u8, a1: *mut c_void, a2: u16, a3: u16) -> u16 {
    unsafe { crate::bg::LoadBgTiles(a0, a1 as _, a2, a3) }
}
/// `LoadSpritePalettes` with this module's view of its types.
#[inline]
unsafe fn LoadSpritePalettes(a0: *mut SpritePalette) {
    unsafe {
        crate::sprite::LoadSpritePalettes(a0 as _);
    }
}
/// `LoadSpriteSheets` with this module's view of its types.
#[inline]
unsafe fn LoadSpriteSheets(a0: *mut SpriteSheet) {
    unsafe {
        crate::sprite::LoadSpriteSheets(a0 as _);
    }
}
/// `SetBgTilemapBuffer` with this module's view of its types.
#[inline]
unsafe fn SetBgTilemapBuffer(a0: u8, a1: *mut c_void) {
    unsafe {
        crate::bg::SetBgTilemapBuffer(a0, a1 as _);
    }
}
/// `SetSubspriteTables` with this module's view of its types.
#[inline]
unsafe fn SetSubspriteTables(a0: *mut Sprite, a1: *mut SubspriteTable) {
    unsafe {
        crate::sprite::SetSubspriteTables(a0 as _, a1 as _);
    }
}
/// `SpriteCallbackDummy` with this module's view of its types.
#[inline]
unsafe fn SpriteCallbackDummy(a0: *mut Sprite) {
    unsafe {
        crate::sprite::SpriteCallbackDummy(a0 as _);
    }
}
/// `StartSpriteAnim` with this module's view of its types.
#[inline]
unsafe fn StartSpriteAnim(a0: *mut Sprite, a1: u8) {
    unsafe {
        crate::sprite::StartSpriteAnim(a0 as _, a1);
    }
}
/// `StringAppendN` with this module's view of its types.
#[inline]
unsafe fn StringAppendN(a0: *mut u8, a1: *mut u8, a2: u8) -> *mut u8 {
    unsafe { crate::string_util::StringAppendN(a0 as _, a1 as _, a2) as *mut u8 }
}
/// `StringCopy` with this module's view of its types.
#[inline]
unsafe fn StringCopy(a0: *mut u8, a1: *mut u8) -> *mut u8 {
    unsafe { crate::string_util::StringCopy(a0 as _, a1 as _) as *mut u8 }
}
/// `StringCopyN` with this module's view of its types.
#[inline]
unsafe fn StringCopyN(a0: *mut u8, a1: *mut u8, a2: u8) -> *mut u8 {
    unsafe { crate::string_util::StringCopyN(a0 as _, a1 as _, a2) as *mut u8 }
}
/// `StringExpandPlaceholders` with this module's view of its types.
#[inline]
unsafe fn StringExpandPlaceholders(a0: *mut u8, a1: *mut u8) -> *mut u8 {
    unsafe { crate::string_util::StringExpandPlaceholders(a0 as _, a1 as _) as *mut u8 }
}
// The C's names for task and sprite data slots.
const sId: usize = 0;
const sState: usize = 0;
const sX: usize = 0;
const tState: usize = 0;
const sPage: usize = 1;
const sXPosId: usize = 1;
const sY: usize = 1;
const sYPosId: usize = 1;
const tFrameCount: usize = 1;
const tKeepFlashing: usize = 1;
const tKeyboardEvent: usize = 1;
const sPrevX: usize = 2;
const tAllowFlash: usize = 2;
const sPrevY: usize = 3;
const tColor: usize = 3;
const tColorIncr: usize = 4;
const sColor: usize = 5;
const tColorDelay: usize = 5;
const sColorIncr: usize = 6;
const sTextSpriteId: usize = 6;
const tColorDelta: usize = 6;
const sButtonSpriteId: usize = 7;
const sColorDelay: usize = 7;
// Data tables (translate with cdata.py): sPCIconOff_Gfx sPCIconOn_Gfx sKeyboard_Pal sRival_Pal sTransferredToPCMessages sText_AlphabetUpperLower sBgTemplates sWindowTemplates sKeyboardChars sPageColumnCounts sPageColumnXPos sNamingScreenTemplates sSubspriteTable_PageSwapFrame sSubspriteTable_PageSwapText sSubspriteTable_Button sSubspriteTable_PCIcon sSpriteTemplate_PageSwapFrame sSpriteTemplate_PageSwapButton sSpriteTemplate_PageSwapText sSpriteTemplate_BackButton sSpriteTemplate_OkButton sSpriteTemplate_Cursor sSpriteTemplate_InputArrow sSpriteTemplate_Underscore sSpriteTemplate_PCIcon sNamingScreenKeyboardText sSpriteSheets sSpritePalettes sPageToNextGfxId sPageToNextKeyboardId sPageToKeyboardId sPageSwapAnimStateFuncs sButtonKeyRoles sPageSwapSpriteFuncs sPageSwapPalTags sPageSwapGfxTags sIconFunctions sKeyboardKeyHandlers sInputFuncs sDrawTextEntryBoxFuncs sDrawGenderIconFuncs sGenderColors sTextColorStruct sFillValues sKeyboardTextColors sNextKeyboardPageTilemaps sPlayerNamingScreenTemplate sPCBoxNamingTemplate sMonNamingScreenTemplate sWaldaWordsScreenTemplate sNamingScreenTemplates sOam_8x8 sOam_16x16 sOam_32x16 sSubsprites_PageSwapFrame sSubsprites_PageSwapText sSubsprites_Button sSubsprites_PCIcon sSubspriteTable_PageSwapFrame sSubspriteTable_PageSwapText sSubspriteTable_Button sSubspriteTable_PCIcon sImageTable_PCIcon sAnim_Loop sAnim_CursorSquish sAnim_PCIcon sAnims_Loop sAnims_Cursor sAnims_PCIcon sSpriteTemplate_PageSwapFrame sSpriteTemplate_PageSwapButton sSpriteTemplate_PageSwapText sSpriteTemplate_BackButton sSpriteTemplate_OkButton sSpriteTemplate_Cursor sSpriteTemplate_InputArrow sSpriteTemplate_Underscore sSpriteTemplate_PCIcon sNamingScreenKeyboardText sSpriteSheets sSpritePalettes

/// `struct NamingScreenData`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NamingScreenData {
    pub tilemapBuffer1: CArray<u8, 2048>,
    pub tilemapBuffer2: CArray<u8, 2048>,
    pub tilemapBuffer3: CArray<u8, 2048>,
    pub textBuffer: CArray<u8, 16>,
    pub tileBuffer: CArray<u8, 1536>,
    pub state: u8,
    pub windows: CArray<u8, 5>,
    pub inputCharBaseXPos: u16,
    pub bg1vOffset: u16,
    pub bg2vOffset: u16,
    pub bg1Priority: u16,
    pub bg2Priority: u16,
    pub bgToReveal: u8,
    pub bgToHide: u8,
    pub currentPage: u8,
    pub cursorSpriteId: u8,
    pub swapBtnFrameSpriteId: u8,
    pub keyRepeatStartDelayCopy: u8,
    pub template: *mut NamingScreenTemplate,
    pub templateNum: u8,
    pub destBuffer: *mut u8,
    pub monSpecies: u16,
    pub monGender: u16,
    pub monPersonality: u32,
    pub returnCallback: Option<unsafe fn()>,
}

unsafe impl Sync for NamingScreenData {}

/// `struct NamingScreenTemplate`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NamingScreenTemplate {
    pub copyExistingString: u8,
    pub maxChars: u8,
    pub iconFunction: u8,
    pub addGenderIcon: u8,
    pub initialPage: u8,
    pub unused: u8,
    pub title: *mut u8,
}

unsafe impl Sync for NamingScreenTemplate {}

#[cfg(target_arch = "arm")]
const _: () = {
    #[allow(unused_imports)]
    use core::mem::{offset_of, size_of};
    assert!(size_of::<NamingScreenData>() == 7744);
    assert!(offset_of!(NamingScreenData, tilemapBuffer1) == 0);
    assert!(offset_of!(NamingScreenData, tilemapBuffer2) == 2048);
    assert!(offset_of!(NamingScreenData, tilemapBuffer3) == 4096);
    assert!(offset_of!(NamingScreenData, textBuffer) == 6144);
    assert!(offset_of!(NamingScreenData, tileBuffer) == 6160);
    assert!(offset_of!(NamingScreenData, state) == 7696);
    assert!(offset_of!(NamingScreenData, windows) == 7697);
    assert!(offset_of!(NamingScreenData, inputCharBaseXPos) == 7702);
    assert!(offset_of!(NamingScreenData, bg1vOffset) == 7704);
    assert!(offset_of!(NamingScreenData, bg2vOffset) == 7706);
    assert!(offset_of!(NamingScreenData, bg1Priority) == 7708);
    assert!(offset_of!(NamingScreenData, bg2Priority) == 7710);
    assert!(offset_of!(NamingScreenData, bgToReveal) == 7712);
    assert!(offset_of!(NamingScreenData, bgToHide) == 7713);
    assert!(offset_of!(NamingScreenData, currentPage) == 7714);
    assert!(offset_of!(NamingScreenData, cursorSpriteId) == 7715);
    assert!(offset_of!(NamingScreenData, swapBtnFrameSpriteId) == 7716);
    assert!(offset_of!(NamingScreenData, keyRepeatStartDelayCopy) == 7717);
    assert!(offset_of!(NamingScreenData, template) == 7720);
    assert!(offset_of!(NamingScreenData, templateNum) == 7724);
    assert!(offset_of!(NamingScreenData, destBuffer) == 7728);
    assert!(offset_of!(NamingScreenData, monSpecies) == 7732);
    assert!(offset_of!(NamingScreenData, monGender) == 7734);
    assert!(offset_of!(NamingScreenData, monPersonality) == 7736);
    assert!(offset_of!(NamingScreenData, returnCallback) == 7740);
    assert!(size_of::<NamingScreenTemplate>() == 12);
    assert!(offset_of!(NamingScreenTemplate, copyExistingString) == 0);
    assert!(offset_of!(NamingScreenTemplate, maxChars) == 1);
    assert!(offset_of!(NamingScreenTemplate, iconFunction) == 2);
    assert!(offset_of!(NamingScreenTemplate, addGenderIcon) == 3);
    assert!(offset_of!(NamingScreenTemplate, initialPage) == 4);
    assert!(offset_of!(NamingScreenTemplate, unused) == 5);
    assert!(offset_of!(NamingScreenTemplate, title) == 8);
};

const BUTTON_BACK: u8 = 1;
const BUTTON_COUNT: i16 = 3;
const BUTTON_OK: u8 = 2;
const BUTTON_PAGE: u8 = 0;
const INPUT_A_BUTTON: u8 = 5;
const INPUT_B_BUTTON: u8 = 6;
const INPUT_DPAD_DOWN: u16 = 2;
const INPUT_DPAD_LEFT: u16 = 3;
const INPUT_DPAD_RIGHT: u16 = 4;
const INPUT_DPAD_UP: u16 = 1;
const INPUT_NONE: i16 = 0;
const INPUT_SELECT: u8 = 8;
const INPUT_START: u8 = 9;
const INPUT_STATE_DISABLED: u8 = 0;
const INPUT_STATE_ENABLED: u8 = 1;
const INPUT_STATE_OVERRIDE: u8 = 2;
const KBPAGE_COUNT: i32 = 3;
const KBPAGE_LETTERS_UPPER: u8 = 1;
const KBROW_COUNT: u8 = 4;
const KEYBOARD_LETTERS_LOWER: u8 = 0;
const KEYBOARD_LETTERS_UPPER: u8 = 1;
const KEY_ROLE_BACKSPACE: u8 = 2;
const KEY_ROLE_CHAR: u8 = 0;
const PALTAG_BACK_BUTTON: u16 = 6;
const PALTAG_CURSOR: u16 = 5;
const PALTAG_OK_BUTTON: u16 = 7;
const PALTAG_PAGE_SWAP: u16 = 4;
const STATE_EXIT: u8 = 9;
const STATE_FADE_IN: u8 = 0;
const STATE_FADE_OUT: u8 = 8;
const STATE_HANDLE_INPUT: u8 = 2;
const STATE_MOVE_TO_OK_BUTTON: u8 = 3;
const STATE_PRESSED_OK: u8 = 6;
const STATE_START_PAGE_SWAP: u8 = 4;
const STATE_WAIT_FADE_IN: u8 = 1;
const STATE_WAIT_PAGE_SWAP: u8 = 5;
const STATE_WAIT_SENT_TO_PC_MESSAGE: u8 = 7;
const WIN_BANNER: i32 = 4;
const WIN_COUNT: u8 = 5;
const WIN_KB_PAGE_1: i32 = 0;
const WIN_KB_PAGE_2: i32 = 1;
const WIN_TEXT_ENTRY: i32 = 2;
const WIN_TEXT_ENTRY_BOX: i32 = 3;

static sBgTemplates: Table<CArray<BgTemplate, 4>> =
    Table((&raw const crate::data::naming_screen::sBgTemplates).cast());
static sButtonKeyRoles: Table<CArray<u8, 3>> =
    Table((&raw const crate::data::naming_screen::sButtonKeyRoles).cast());
static sDrawGenderIconFuncs: Table<CArray<Option<unsafe fn()>, 2>> =
    Table((&raw const crate::data::naming_screen::sDrawGenderIconFuncs).cast());
static sDrawTextEntryBoxFuncs: Table<CArray<Option<unsafe fn()>, 5>> =
    Table((&raw const crate::data::naming_screen::sDrawTextEntryBoxFuncs).cast());
static sFillValues: Table<CArray<u8, 3>> =
    Table((&raw const crate::data::naming_screen::sFillValues).cast());
static sGenderColors: Table<CArray<CArray<u8, 3>, 2>> =
    Table((&raw const crate::data::naming_screen::sGenderColors).cast());
static sIconFunctions: Table<CArray<Option<unsafe fn()>, 5>> =
    Table((&raw const crate::data::naming_screen::sIconFunctions).cast());
static sInputFuncs: Table<CArray<Option<unsafe fn(*mut Task)>, 3>> =
    Table((&raw const crate::data::naming_screen::sInputFuncs).cast());
static sKeyboardChars: Table<CArray<CArray<CArray<u8, 8>, 4>, 3>> =
    Table((&raw const crate::data::naming_screen::sKeyboardChars).cast());
static sKeyboardKeyHandlers: Table<CArray<Option<unsafe fn(u8) -> u8>, 4>> =
    Table((&raw const crate::data::naming_screen::sKeyboardKeyHandlers).cast());
static sKeyboardTextColors: Table<CArray<*mut u8, 3>> =
    Table((&raw const crate::data::naming_screen::sKeyboardTextColors).cast());
static sKeyboard_Pal: Table<CArray<u16, 16>> =
    Table((&raw const crate::data::naming_screen::sKeyboard_Pal).cast());
static sNamingScreenKeyboardText: Table<CArray<CArray<*mut u8, 4>, 3>> =
    Table((&raw const crate::data::naming_screen::sNamingScreenKeyboardText).cast());
static sNamingScreenTemplates: Table<CArray<*mut NamingScreenTemplate, 5>> =
    Table((&raw const crate::data::naming_screen::sNamingScreenTemplates).cast());
static sNextKeyboardPageTilemaps: Table<CArray<*mut u32, 3>> =
    Table((&raw const crate::data::naming_screen::sNextKeyboardPageTilemaps).cast());
static sPageColumnCounts: Table<CArray<u8, 3>> =
    Table((&raw const crate::data::naming_screen::sPageColumnCounts).cast());
static sPageColumnXPos: Table<CArray<CArray<u8, 8>, 3>> =
    Table((&raw const crate::data::naming_screen::sPageColumnXPos).cast());
static sPageSwapAnimStateFuncs: Table<CArray<Option<unsafe fn(*mut Task) -> u8>, 4>> =
    Table((&raw const crate::data::naming_screen::sPageSwapAnimStateFuncs).cast());
static sPageSwapGfxTags: Table<CArray<u16, 3>> =
    Table((&raw const crate::data::naming_screen::sPageSwapGfxTags).cast());
static sPageSwapPalTags: Table<CArray<u16, 3>> =
    Table((&raw const crate::data::naming_screen::sPageSwapPalTags).cast());
static sPageSwapSpriteFuncs: Table<CArray<Option<unsafe fn(*mut Sprite) -> u8>, 4>> =
    Table((&raw const crate::data::naming_screen::sPageSwapSpriteFuncs).cast());
static sPageToKeyboardId: Table<CArray<u8, 3>> =
    Table((&raw const crate::data::naming_screen::sPageToKeyboardId).cast());
static sPageToNextGfxId: Table<CArray<u8, 3>> =
    Table((&raw const crate::data::naming_screen::sPageToNextGfxId).cast());
static sPageToNextKeyboardId: Table<CArray<u8, 3>> =
    Table((&raw const crate::data::naming_screen::sPageToNextKeyboardId).cast());
static sSpritePalettes: Table<CArray<SpritePalette, 9>> =
    Table((&raw const crate::data::naming_screen::sSpritePalettes).cast());
static sSpriteSheets: Table<CArray<SpriteSheet, 13>> =
    Table((&raw const crate::data::naming_screen::sSpriteSheets).cast());
static sSpriteTemplate_BackButton: Table<SpriteTemplate> =
    Table((&raw const crate::data::naming_screen::sSpriteTemplate_BackButton).cast());
static sSpriteTemplate_Cursor: Table<SpriteTemplate> =
    Table((&raw const crate::data::naming_screen::sSpriteTemplate_Cursor).cast());
static sSpriteTemplate_InputArrow: Table<SpriteTemplate> =
    Table((&raw const crate::data::naming_screen::sSpriteTemplate_InputArrow).cast());
static sSpriteTemplate_OkButton: Table<SpriteTemplate> =
    Table((&raw const crate::data::naming_screen::sSpriteTemplate_OkButton).cast());
static sSpriteTemplate_PCIcon: Table<SpriteTemplate> =
    Table((&raw const crate::data::naming_screen::sSpriteTemplate_PCIcon).cast());
static sSpriteTemplate_PageSwapButton: Table<SpriteTemplate> =
    Table((&raw const crate::data::naming_screen::sSpriteTemplate_PageSwapButton).cast());
static sSpriteTemplate_PageSwapFrame: Table<SpriteTemplate> =
    Table((&raw const crate::data::naming_screen::sSpriteTemplate_PageSwapFrame).cast());
static sSpriteTemplate_PageSwapText: Table<SpriteTemplate> =
    Table((&raw const crate::data::naming_screen::sSpriteTemplate_PageSwapText).cast());
static sSpriteTemplate_Underscore: Table<SpriteTemplate> =
    Table((&raw const crate::data::naming_screen::sSpriteTemplate_Underscore).cast());
static sSubspriteTable_Button: Table<CArray<SubspriteTable, 1>> =
    Table((&raw const crate::data::naming_screen::sSubspriteTable_Button).cast());
static sSubspriteTable_PCIcon: Table<CArray<SubspriteTable, 1>> =
    Table((&raw const crate::data::naming_screen::sSubspriteTable_PCIcon).cast());
static sSubspriteTable_PageSwapFrame: Table<CArray<SubspriteTable, 1>> =
    Table((&raw const crate::data::naming_screen::sSubspriteTable_PageSwapFrame).cast());
static sSubspriteTable_PageSwapText: Table<CArray<SubspriteTable, 3>> =
    Table((&raw const crate::data::naming_screen::sSubspriteTable_PageSwapText).cast());
static sText_AlphabetUpperLower: Table<CArray<u8, 54>> =
    Table((&raw const crate::data::naming_screen::sText_AlphabetUpperLower).cast());
static sTransferredToPCMessages: Table<CArray<*mut u8, 4>> =
    Table((&raw const crate::data::naming_screen::sTransferredToPCMessages).cast());
static sWindowTemplates: Table<CArray<WindowTemplate, 6>> =
    Table((&raw const crate::data::naming_screen::sWindowTemplates).cast());

#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sNamingScreen: *mut NamingScreenData = null_mut();

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
/// `GetTextWindowPalette` with this module's view of its types.
#[inline]
unsafe fn GetTextWindowPalette(a0: u8) -> *mut u16 {
    unsafe { crate::text_window::GetTextWindowPalette(a0) as *mut u16 }
}
/// `LZ77UnCompWram` with this module's view of its types.
#[inline]
unsafe fn LZ77UnCompWram(a0: *mut u32, a1: *mut c_void) {
    unsafe {
        crate::syscall::LZ77UnCompWram(a0 as _, a1 as _);
    }
}
/// `SetMainCallback2` with this module's view of its types.
#[inline]
unsafe fn SetMainCallback2(a0: Option<unsafe fn()>) {
    unsafe {
        crate::agb_main::SetMainCallback2(core::mem::transmute(a0));
    }
}

#[unsafe(no_mangle)]
pub unsafe fn DoNamingScreen(
    templateNum: u8,
    destBuffer: *mut u8,
    monSpecies: u16,
    monGender: u16,
    monPersonality: u32,
    returnCallback: Option<unsafe fn()>,
) {
    sNamingScreen = Alloc(7744) as *mut NamingScreenData;
    if sNamingScreen.is_null() {
        SetMainCallback2(returnCallback);
    } else {
        (*sNamingScreen).templateNum = templateNum;
        (*sNamingScreen).monSpecies = monSpecies;
        (*sNamingScreen).monGender = monGender;
        (*sNamingScreen).monPersonality = monPersonality;
        (*sNamingScreen).destBuffer = destBuffer;
        (*sNamingScreen).returnCallback = returnCallback;
        if templateNum == NAMING_SCREEN_PLAYER {
            StartTimer1();
        }
        SetMainCallback2(Some(CB2_LoadNamingScreen));
    }
}
pub(crate) unsafe fn CB2_LoadNamingScreen() {
    match gMain.state {
        0 => {
            ResetVHBlank();
            NamingScreen_Init();
            gMain.state += 1;
        }
        1 => {
            NamingScreen_InitBGs();
            gMain.state += 1;
        }
        2 => {
            ResetPaletteFade();
            gMain.state += 1;
        }
        3 => {
            ResetSpriteData();
            FreeAllSpritePalettes();
            gMain.state += 1;
        }
        4 => {
            ResetTasks();
            gMain.state += 1;
        }
        5 => {
            LoadPalettes();
            gMain.state += 1;
        }
        6 => {
            LoadGfx();
            gMain.state += 1;
        }
        7 => {
            CreateSprites();
            UpdatePaletteFade();
            NamingScreen_ShowBgs();
            gMain.state += 1;
        }
        _ => {
            CreateHelperTasks();
            CreateNamingScreenTask();
        }
    }
}
unsafe fn NamingScreen_Init() {
    (*sNamingScreen).state = STATE_FADE_IN;
    (*sNamingScreen).bg1vOffset = 0;
    (*sNamingScreen).bg2vOffset = 0;
    (*sNamingScreen).bg1Priority = 1;
    (*sNamingScreen).bg2Priority = 2;
    (*sNamingScreen).bgToReveal = 0;
    (*sNamingScreen).bgToHide = 1;
    (*sNamingScreen).template = sNamingScreenTemplates[(*sNamingScreen).templateNum];
    (*sNamingScreen).currentPage = (*(*sNamingScreen).template).initialPage;
    (*sNamingScreen).inputCharBaseXPos =
        ((DISPLAY_WIDTH as i32 - (*(*sNamingScreen).template).maxChars as i32 * 8) / 2) as u16 + 6;
    if (*sNamingScreen).templateNum == NAMING_SCREEN_WALDA {
        (*sNamingScreen).inputCharBaseXPos += 11;
    }
    (*sNamingScreen).keyRepeatStartDelayCopy = gKeyRepeatStartDelay as u8;
    memset((*sNamingScreen).textBuffer.as_mut_ptr(), EOS as i32, 16);
    if (*(*sNamingScreen).template).copyExistingString != 0 {
        StringCopy(
            (*sNamingScreen).textBuffer.as_mut_ptr(),
            (*sNamingScreen).destBuffer,
        );
    }
    gKeyRepeatStartDelay = 16;
}
unsafe fn SetSpritesVisible() {
    for i in 0..MAX_SPRITES {
        if gSprites[i].inUse() != 0 {
            gSprites[i].set_invisible(FALSE as u16);
        }
    }
    SetCursorInvisibility(FALSE);
}
unsafe fn NamingScreen_InitBGs() {
    {
        let mut _dest: *mut c_void = VRAM as usize as *mut c_void;
        let mut _size: u32 = VRAM_SIZE;
        loop {
            {
                {
                    let mut tmp: u16 = 0;
                    volatile_write(&raw mut tmp, 0);
                    {
                        {
                            let dmaRegs: *mut u32 = 67109076_usize as *mut u32;
                            volatile_write(dmaRegs, &raw mut tmp as usize as u32);
                            volatile_write(dmaRegs.at(1), _dest as usize as u32);
                            volatile_write(dmaRegs.at(2), 0x81000800);
                            let _ = (dmaRegs.at(2)).read_volatile();
                        }
                    }
                }
            }
            _dest = (_dest as *mut u8).at(4096) as *mut c_void;
            _size -= 0x1000;
            if _size <= 0x1000 {
                {
                    {
                        let mut tmp: u16 = 0;
                        volatile_write(&raw mut tmp, 0);
                        {
                            {
                                let dmaRegs: *mut u32 = 67109076_usize as *mut u32;
                                volatile_write(dmaRegs, &raw mut tmp as usize as u32);
                                volatile_write(dmaRegs.at(1), _dest as usize as u32);
                                volatile_write(dmaRegs.at(2), 0x81000000 | (_size / 2));
                                let _ = (dmaRegs.at(2)).read_volatile();
                            }
                        }
                    }
                }
                break;
            }
        }
    }
    {
        {
            let mut _dest: *mut u32 = OAM as i32 as usize as *mut c_void as *mut u32;
            let mut _size: u32 = OAM_SIZE;
            {
                {
                    let mut tmp: u32 = 0;
                    volatile_write(&raw mut tmp, 0);
                    {
                        {
                            let dmaRegs: *mut u32 = 67109076_usize as *mut u32;
                            volatile_write(dmaRegs, &raw mut tmp as usize as u32);
                            volatile_write(dmaRegs.at(1), _dest as usize as u32);
                            volatile_write(dmaRegs.at(2), 0x85000000 | (_size / 4));
                            let _ = (dmaRegs.at(2)).read_volatile();
                        }
                    }
                }
            }
        }
    }
    {
        {
            let mut _dest: *mut u16 = PLTT as i32 as usize as *mut c_void as *mut u16;
            let mut _size: u32 = PLTT_SIZE;
            {
                {
                    let mut tmp: u16 = 0;
                    volatile_write(&raw mut tmp, 0);
                    {
                        {
                            let dmaRegs: *mut u32 = 67109076_usize as *mut u32;
                            volatile_write(dmaRegs, &raw mut tmp as usize as u32);
                            volatile_write(dmaRegs.at(1), _dest as usize as u32);
                            volatile_write(dmaRegs.at(2), 0x81000000 | (_size / 2));
                            let _ = (dmaRegs.at(2)).read_volatile();
                        }
                    }
                }
            }
        }
    }
    SetGpuReg(0x0, 0x0000);
    ResetBgsAndClearDma3BusyFlags(0);
    InitBgsFromTemplates(0, sBgTemplates.as_ptr().cast_mut(), 4);
    ChangeBgX(0, 0, BG_COORD_SET);
    ChangeBgY(0, 0, BG_COORD_SET);
    ChangeBgX(1, 0, BG_COORD_SET);
    ChangeBgY(1, 0, BG_COORD_SET);
    ChangeBgX(2, 0, BG_COORD_SET);
    ChangeBgY(2, 0, BG_COORD_SET);
    ChangeBgX(3, 0, BG_COORD_SET);
    ChangeBgY(3, 0, BG_COORD_SET);
    InitStandardTextBoxWindows();
    InitTextBoxGfxAndPrinters();
    for i in 0..WIN_COUNT {
        (*sNamingScreen).windows[i] = AddWindow((&raw const sWindowTemplates[i]).cast_mut()) as u8;
    }
    SetGpuReg(REG_OFFSET_DISPCNT, 4160);
    SetGpuReg(REG_OFFSET_BLDCNT, 1600);
    SetGpuReg(REG_OFFSET_BLDALPHA, 2060);
    SetBgTilemapBuffer(
        1,
        (*sNamingScreen).tilemapBuffer1.as_mut_ptr() as *mut c_void,
    );
    SetBgTilemapBuffer(
        2,
        (*sNamingScreen).tilemapBuffer2.as_mut_ptr() as *mut c_void,
    );
    SetBgTilemapBuffer(
        3,
        (*sNamingScreen).tilemapBuffer3.as_mut_ptr() as *mut c_void,
    );
    FillBgTilemapBufferRect_Palette0(1, 0, 0, 0, 0x20, 0x20);
    FillBgTilemapBufferRect_Palette0(2, 0, 0, 0, 0x20, 0x20);
    FillBgTilemapBufferRect_Palette0(3, 0, 0, 0, 0x20, 0x20);
}
unsafe fn CreateNamingScreenTask() {
    CreateTask(Some(Task_NamingScreen), 2);
    SetMainCallback2(Some(CB2_NamingScreen));
}
pub(crate) unsafe fn Task_NamingScreen(taskId: u8) {
    match (*sNamingScreen).state {
        STATE_FADE_IN => {
            MainState_FadeIn();
            SetSpritesVisible();
            SetVBlank();
        }
        STATE_WAIT_FADE_IN => {
            MainState_WaitFadeIn();
        }
        STATE_HANDLE_INPUT => {
            MainState_HandleInput();
        }
        STATE_MOVE_TO_OK_BUTTON => {
            MainState_MoveToOKButton();
            MainState_HandleInput();
        }
        STATE_START_PAGE_SWAP => {
            MainState_StartPageSwap();
        }
        STATE_WAIT_PAGE_SWAP => {
            MainState_WaitPageSwap();
        }
        STATE_PRESSED_OK => {
            MainState_PressedOKButton();
        }
        STATE_WAIT_SENT_TO_PC_MESSAGE => {
            MainState_WaitSentToPCMessage();
        }
        STATE_FADE_OUT => {
            MainState_FadeOut();
        }
        STATE_EXIT => {
            MainState_Exit();
        }
        _ => {}
    }
}
unsafe fn PageToNextGfxId(page: u8) -> u8 {
    sPageToNextGfxId[page]
}
unsafe fn CurrentPageToNextKeyboardId() -> u8 {
    sPageToNextKeyboardId[(*sNamingScreen).currentPage]
}
unsafe fn CurrentPageToKeyboardId() -> u8 {
    sPageToKeyboardId[(*sNamingScreen).currentPage]
}
unsafe fn MainState_FadeIn() -> u8 {
    DrawBgTilemap(
        3,
        (*(&raw const crate::data::graphics::gNamingScreenBackground_Tilemap)
            .cast::<CArray<u32, 0>>())
        .as_ptr()
        .cast_mut() as *mut c_void,
    );
    (*sNamingScreen).currentPage = KBPAGE_LETTERS_UPPER;
    DrawBgTilemap(
        2,
        (*(&raw const crate::data::graphics::gNamingScreenKeyboardLower_Tilemap)
            .cast::<CArray<u32, 0>>())
        .as_ptr()
        .cast_mut() as *mut c_void,
    );
    DrawBgTilemap(
        1,
        (*(&raw const crate::data::graphics::gNamingScreenKeyboardUpper_Tilemap)
            .cast::<CArray<u32, 0>>())
        .as_ptr()
        .cast_mut() as *mut c_void,
    );
    PrintKeyboardKeys((*sNamingScreen).windows[1], KEYBOARD_LETTERS_LOWER);
    PrintKeyboardKeys((*sNamingScreen).windows[0], KEYBOARD_LETTERS_UPPER);
    NamingScreen_Dummy(2, KEYBOARD_LETTERS_LOWER);
    NamingScreen_Dummy(1, KEYBOARD_LETTERS_UPPER);
    DrawTextEntry();
    DrawTextEntryBox();
    PrintControls();
    CopyBgTilemapBufferToVram(1);
    CopyBgTilemapBufferToVram(2);
    CopyBgTilemapBufferToVram(3);
    BlendPalettes(PALETTES_ALL, 16, 0);
    BeginNormalPaletteFade(PALETTES_ALL, 0, 16, 0, 0);
    (*sNamingScreen).state += 1;
    FALSE
}
unsafe fn MainState_WaitFadeIn() -> u8 {
    if gPaletteFade.active() == 0 {
        SetInputState(INPUT_STATE_ENABLED);
        SetCursorFlashing(TRUE);
        (*sNamingScreen).state += 1;
    }
    FALSE
}
unsafe fn MainState_HandleInput() -> u8 {
    HandleKeyboardEvent()
}
unsafe fn MainState_MoveToOKButton() -> u8 {
    if IsCursorAnimFinished() != 0 {
        SetInputState(INPUT_STATE_ENABLED);
        MoveCursorToOKButton();
        (*sNamingScreen).state = STATE_HANDLE_INPUT;
    }
    FALSE
}
unsafe fn MainState_PressedOKButton() -> u8 {
    SaveInputText();
    SetInputState(INPUT_STATE_DISABLED);
    SetCursorFlashing(FALSE);
    TryStartButtonFlash(BUTTON_COUNT as u8, FALSE, TRUE);
    if (*sNamingScreen).templateNum == NAMING_SCREEN_CAUGHT_MON
        && CalculatePlayerPartyCount() >= PARTY_SIZE as u8
    {
        DisplaySentToPCMessage();
        (*sNamingScreen).state = STATE_WAIT_SENT_TO_PC_MESSAGE;
        return FALSE;
    } else {
        (*sNamingScreen).state = STATE_FADE_OUT;
        return TRUE;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
unsafe fn MainState_FadeOut() -> u8 {
    BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 16, 0);
    (*sNamingScreen).state += 1;
    FALSE
}
unsafe fn MainState_Exit() -> u8 {
    if gPaletteFade.active() == 0 {
        if (*sNamingScreen).templateNum == NAMING_SCREEN_PLAYER {
            SeedRngAndSetTrainerId();
        }
        SetMainCallback2((*sNamingScreen).returnCallback);
        DestroyTask(FindTaskIdByFunc(Some(Task_NamingScreen)));
        FreeAllWindowBuffers();
        Free(sNamingScreen as *mut c_void);
        sNamingScreen = null_mut();
    }
    FALSE
}
unsafe fn DisplaySentToPCMessage() {
    let mut stringToDisplay: u8 = 0;
    if IsDestinationBoxFull() == 0 {
        StringCopy(
            gStringVar1.as_mut_ptr(),
            GetBoxNamePtr(VarGet(VAR_PC_BOX_TO_SEND_MON) as u8),
        );
        StringCopy(gStringVar2.as_mut_ptr(), (*sNamingScreen).destBuffer);
    } else {
        StringCopy(
            gStringVar1.as_mut_ptr(),
            GetBoxNamePtr(VarGet(VAR_PC_BOX_TO_SEND_MON) as u8),
        );
        StringCopy(gStringVar2.as_mut_ptr(), (*sNamingScreen).destBuffer);
        StringCopy(
            gStringVar3.as_mut_ptr(),
            GetBoxNamePtr(GetPCBoxToSendMon() as u8),
        );
        stringToDisplay = 2;
    }
    if FlagGet(FLAG_SYS_PC_LANETTE) != 0 {
        stringToDisplay += 1;
    }
    StringExpandPlaceholders(
        gStringVar4.as_mut_ptr(),
        sTransferredToPCMessages[stringToDisplay],
    );
    DrawDialogueFrame(0, 0);
    (*(&raw const crate::text::gTextFlags)
        .cast::<TextFlags>()
        .cast_mut())
    .set_canABSpeedUpPrint(TRUE);
    AddTextPrinterParameterized2(
        0,
        FONT_NORMAL,
        gStringVar4.as_mut_ptr(),
        GetPlayerTextSpeedDelay(),
        None,
        TEXT_COLOR_DARK_GRAY,
        TEXT_COLOR_WHITE,
        TEXT_COLOR_LIGHT_GRAY,
    );
    CopyWindowToVram(0, COPYWIN_FULL);
}
unsafe fn MainState_WaitSentToPCMessage() -> u8 {
    RunTextPrinters();
    if IsTextPrinterActive(0) == 0 && gMain.newKeys as i32 & A_BUTTON != 0 {
        (*sNamingScreen).state = STATE_FADE_OUT;
    }
    FALSE
}
unsafe fn MainState_StartPageSwap() -> u8 {
    SetInputState(INPUT_STATE_DISABLED);
    StartPageSwapButtonAnim();
    StartPageSwapAnim();
    SetCursorInvisibility(TRUE);
    TryStartButtonFlash(BUTTON_PAGE, FALSE, TRUE);
    PlaySE(SE_WIN_OPEN);
    (*sNamingScreen).state = STATE_WAIT_PAGE_SWAP;
    FALSE
}
unsafe fn MainState_WaitPageSwap() -> u8 {
    let mut cursorX: i16 = 0;
    let mut cursorY: i16 = 0;
    let mut onLastColumn: u32 = 0;
    if IsPageSwapAnimNotInProgress() != 0 {
        GetCursorPos(&raw mut cursorX, &raw mut cursorY);
        onLastColumn = (cursorX == GetCurrentPageColumnCount() as i16) as u32;
        (*sNamingScreen).state = STATE_HANDLE_INPUT;
        (*sNamingScreen).currentPage += 1;
        (*sNamingScreen).currentPage = ((*sNamingScreen).currentPage as i32 % 3) as u8;
        if onLastColumn != 0 {
            cursorX = GetCurrentPageColumnCount() as i16;
        } else {
            if cursorX >= GetCurrentPageColumnCount() as i16 {
                cursorX = GetCurrentPageColumnCount() as i16 - 1;
            }
        }
        SetCursorPos(cursorX, cursorY);
        DrawKeyboardPageOnDeck();
        SetInputState(INPUT_STATE_ENABLED);
        SetCursorInvisibility(FALSE);
    }
    FALSE
}
unsafe fn StartPageSwapAnim() {
    let taskId: u8 = CreateTask(Some(Task_HandlePageSwapAnim), 0);
    Task_HandlePageSwapAnim(taskId);
}
pub(crate) unsafe fn Task_HandlePageSwapAnim(taskId: u8) {
    while sPageSwapAnimStateFuncs[task_get(taskId, tState)].unwrap_unchecked()(
        &raw mut (*gTasks.as_ptr())[taskId],
    ) != 0
    {}
}
unsafe fn IsPageSwapAnimNotInProgress() -> u8 {
    if FindTaskIdByFunc(Some(Task_HandlePageSwapAnim)) == TASK_NONE {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
pub(crate) unsafe fn PageSwapAnimState_Init(task: *mut Task) -> u8 {
    (*sNamingScreen).bg1vOffset = 0;
    (*sNamingScreen).bg2vOffset = 0;
    (*task).data[tState] += 1;
    0
}
pub(crate) unsafe fn PageSwapAnimState_1(task: *mut Task) -> u8 {
    let mut vOffsets: CArray<*mut u16, 2> = zeroed();
    vOffsets[0] = &raw mut (*sNamingScreen).bg2vOffset;
    vOffsets[1] = &raw mut (*sNamingScreen).bg1vOffset;
    (*task).data[tFrameCount] += 4;
    *vOffsets[(*sNamingScreen).bgToReveal] = Sin((*task).data[tFrameCount], 40) as u16;
    *vOffsets[(*sNamingScreen).bgToHide] = Sin(((*task).data[tFrameCount] + 128) & 0xFF, 40) as u16;
    if (*task).data[tFrameCount] >= 64 {
        let temp: u8 = (*sNamingScreen).bg1Priority as u8;
        (*sNamingScreen).bg1Priority = (*sNamingScreen).bg2Priority;
        (*sNamingScreen).bg2Priority = temp as u16;
        (*task).data[tState] += 1;
    }
    0
}
pub(crate) unsafe fn PageSwapAnimState_2(task: *mut Task) -> u8 {
    let mut vOffsets: CArray<*mut u16, 2> = zeroed();
    vOffsets[0] = &raw mut (*sNamingScreen).bg2vOffset;
    vOffsets[1] = &raw mut (*sNamingScreen).bg1vOffset;
    (*task).data[tFrameCount] += 4;
    *vOffsets[(*sNamingScreen).bgToReveal] = Sin((*task).data[tFrameCount], 40) as u16;
    *vOffsets[(*sNamingScreen).bgToHide] = Sin(((*task).data[tFrameCount] + 128) & 0xFF, 40) as u16;
    if (*task).data[tFrameCount] >= 128 {
        let temp: u8 = (*sNamingScreen).bgToReveal;
        (*sNamingScreen).bgToReveal = (*sNamingScreen).bgToHide;
        (*sNamingScreen).bgToHide = temp;
        (*task).data[tState] += 1;
    }
    0
}
pub(crate) unsafe fn PageSwapAnimState_Done(task: *mut Task) -> u8 {
    DestroyTask(FindTaskIdByFunc(Some(Task_HandlePageSwapAnim)));
    0
}
unsafe fn CreateButtonFlashTask() {
    let taskId: u8 = CreateTask(Some(Task_UpdateButtonFlash), 3);
    task_set(taskId, 0, BUTTON_COUNT);
}
unsafe fn TryStartButtonFlash(button: u8, keepFlashing: u8, interruptCurFlash: u8) {
    let task: *mut Task =
        &raw mut (*gTasks.as_ptr())[FindTaskIdByFunc(Some(Task_UpdateButtonFlash))];
    if button as i16 == (*task).data[0] && interruptCurFlash == 0 {
        (*task).data[tKeepFlashing] = keepFlashing as i16;
        (*task).data[tAllowFlash] = TRUE as i16;
        return;
    }
    if button == BUTTON_COUNT as u8 && (*task).data[tKeepFlashing] == 0 && interruptCurFlash == 0 {
        return;
    }
    if (*task).data[0] != BUTTON_COUNT {
        RestoreButtonColor((*task).data[0] as u8);
    }
    StartButtonFlash(task, button, keepFlashing);
}
pub(crate) unsafe fn Task_UpdateButtonFlash(taskId: u8) {
    let task: *mut Task = &raw mut (*gTasks.as_ptr())[taskId];
    if (*task).data[0] == BUTTON_COUNT || (*task).data[tAllowFlash] == 0 {
        return;
    }
    MultiplyInvertedPaletteRGBComponents(
        GetButtonPalOffset((*task).data[0] as u8),
        (*task).data[tColor] as u8,
        (*task).data[tColor] as u8,
        (*task).data[tColor] as u8,
    );
    if (*task).data[tColorDelay] != 0
        && ({
            (*task).data[tColorDelay] -= 1;
            (*task).data[tColorDelay]
        }) != 0
    {
        return;
    }
    (*task).data[tColorDelay] = 2;
    if (*task).data[tColorIncr] >= 0 {
        if (*task).data[tColor] < 14 {
            (*task).data[tColor] += (*task).data[tColorIncr];
            (*task).data[tColorDelta] += (*task).data[tColorIncr];
        } else {
            (*task).data[tColor] = 16;
            (*task).data[tColorDelta] += 1;
        }
    } else {
        (*task).data[tColor] += (*task).data[tColorIncr];
        (*task).data[tColorDelta] += (*task).data[tColorIncr];
    }
    if (*task).data[tColor] == 16 && (*task).data[tColorDelta] == 22 {
        (*task).data[tColorIncr] = -4;
    } else if (*task).data[tColor] == 0 {
        (*task).data[tAllowFlash] = (*task).data[tKeepFlashing];
        (*task).data[tColorIncr] = 2;
        (*task).data[tColorDelta] = 0;
    }
}
unsafe fn GetButtonPalOffset(button: u8) -> u16 {
    let mut palOffsets: CArray<u16, 4> = zeroed();
    palOffsets[0] = 0x100 + IndexOfSpritePaletteTag(PALTAG_PAGE_SWAP) as u16 * 16 + 14;
    palOffsets[1] = 0x100 + IndexOfSpritePaletteTag(PALTAG_BACK_BUTTON) as u16 * 16 + 14;
    palOffsets[2] = 0x100 + IndexOfSpritePaletteTag(PALTAG_OK_BUTTON) as u16 * 16 + 14;
    palOffsets[3] = 0x100 + IndexOfSpritePaletteTag(PALTAG_OK_BUTTON) as u16 * 16 + 1;
    palOffsets[button]
}
unsafe fn RestoreButtonColor(button: u8) {
    let index: u16 = GetButtonPalOffset(button);
    gPlttBufferFaded[index] = gPlttBufferUnfaded[index];
}
unsafe fn StartButtonFlash(task: *mut Task, button: u8, keepFlashing: u8) {
    (*task).data[0] = button as i16;
    (*task).data[tKeepFlashing] = keepFlashing as i16;
    (*task).data[tAllowFlash] = TRUE as i16;
    (*task).data[tColor] = 4;
    (*task).data[tColorIncr] = 2;
    (*task).data[tColorDelay] = 0;
    (*task).data[tColorDelta] = 4;
}
pub(crate) unsafe fn SpriteCB_Cursor(sprite: *mut Sprite) {
    if (*sprite).animEnded() != 0 {
        StartSpriteAnim(sprite, 0);
    }
    (*sprite).set_invisible((*sprite).data[4] as u16 & 0x00FF);
    if (*sprite).data[sX] == GetCurrentPageColumnCount() as i16 {
        (*sprite).set_invisible(TRUE as u16);
    }
    if (*sprite).invisible() != 0
        || (*sprite).data[4] as i32 & 0xFF00 == 0
        || (*sprite).data[sX] != (*sprite).data[sPrevX]
        || (*sprite).data[sY] != (*sprite).data[sPrevY]
    {
        (*sprite).data[sColor] = 0;
        (*sprite).data[sColorIncr] = 2;
        (*sprite).data[sColorDelay] = 2;
    }
    (*sprite).data[sColorDelay] -= 1;
    if (*sprite).data[sColorDelay] == 0 {
        (*sprite).data[sColor] += (*sprite).data[sColorIncr];
        if (*sprite).data[sColor] == 16 || (*sprite).data[sColor] == 0 {
            (*sprite).data[sColorIncr] = -(*sprite).data[sColorIncr];
        }
        (*sprite).data[sColorDelay] = 2;
    }
    if (*sprite).data[4] as i32 & 0xFF00 != 0 {
        let gb: i8 = (*sprite).data[sColor] as i8;
        let r: i8 = ((*sprite).data[sColor] >> 1) as i8;
        let index: u16 = 0x100 + IndexOfSpritePaletteTag(PALTAG_CURSOR) as u16 * 16 + 1;
        MultiplyInvertedPaletteRGBComponents(index, r as u8, gb as u8, gb as u8);
    }
}
pub(crate) unsafe fn SpriteCB_InputArrow(sprite: *mut Sprite) {
    let x: CArray<i16, 4> = CArray([0, -4, -2, -1]);
    if (*sprite).data[0] == 0
        || ({
            (*sprite).data[0] -= 1;
            (*sprite).data[0]
        }) == 0
    {
        (*sprite).data[0] = 8;
        (*sprite).data[sXPosId] = (if 0 != 0 {
            ((*sprite).data[sXPosId] as u32 + 1) % 4
        } else {
            ((*sprite).data[sXPosId] as u32 + 1) & 3
        }) as i16;
    }
    (*sprite).x2 = x[(*sprite).data[sXPosId]];
}
pub(crate) unsafe fn SpriteCB_Underscore(sprite: *mut Sprite) {
    let y: CArray<i16, 4> = CArray([2, 3, 2, 1]);
    let pos: u8 = GetTextEntryPosition();
    if pos != (*sprite).data[sId] as u8 {
        (*sprite).y2 = 0;
        (*sprite).data[sYPosId] = 0;
        (*sprite).data[2] = 0;
    } else {
        (*sprite).y2 = y[(*sprite).data[sYPosId]];
        (*sprite).data[2] += 1;
        if (*sprite).data[2] > 8 {
            (*sprite).data[sYPosId] = (if 0 != 0 {
                ((*sprite).data[sYPosId] as u32 + 1) % 4
            } else {
                ((*sprite).data[sYPosId] as u32 + 1) & 3
            }) as i16;
            (*sprite).data[2] = 0;
        }
    }
}
unsafe fn CreateSprites() {
    CreateCursorSprite();
    CreatePageSwapButtonSprites();
    CreateBackOkSprites();
    CreateTextEntrySprites();
    CreateInputTargetIcon();
}
unsafe fn CreateCursorSprite() {
    (*sNamingScreen).cursorSpriteId =
        CreateSprite((&raw const *sSpriteTemplate_Cursor).cast_mut(), 38, 88, 1);
    SetCursorInvisibility(TRUE);
    gSprites[(*sNamingScreen).cursorSpriteId]
        .oam
        .set_priority(1);
    gSprites[(*sNamingScreen).cursorSpriteId]
        .oam
        .set_objMode(ST_OAM_OBJ_BLEND);
    gSprites[(*sNamingScreen).cursorSpriteId].data[sColorIncr] = 1;
    gSprites[(*sNamingScreen).cursorSpriteId].data[sColorIncr] = 2;
    SetCursorPos(0, 0);
}
unsafe fn SetCursorPos(x: i16, y: i16) {
    let cursorSprite: *mut Sprite = &raw mut gSprites[(*sNamingScreen).cursorSpriteId];
    if x < sPageColumnCounts[CurrentPageToKeyboardId()] as i16 {
        (*cursorSprite).x = sPageColumnXPos[CurrentPageToKeyboardId()][x] as i16 + 38;
    } else {
        (*cursorSprite).x = 0;
    }
    (*cursorSprite).y = y * 16 + 88;
    (*cursorSprite).data[sPrevX] = (*cursorSprite).data[sX];
    (*cursorSprite).data[sPrevY] = (*cursorSprite).data[sY];
    (*cursorSprite).data[sX] = x;
    (*cursorSprite).data[sY] = y;
}
unsafe fn GetCursorPos(x: *mut i16, y: *mut i16) {
    let cursorSprite: *mut Sprite = &raw mut gSprites[(*sNamingScreen).cursorSpriteId];
    *x = (*cursorSprite).data[sX];
    *y = (*cursorSprite).data[sY];
}
unsafe fn MoveCursorToOKButton() {
    SetCursorPos(GetCurrentPageColumnCount() as i16, 2);
}
unsafe fn SetCursorInvisibility(invisible: u8) {
    gSprites[(*sNamingScreen).cursorSpriteId].data[4] &= -256;
    gSprites[(*sNamingScreen).cursorSpriteId].data[4] |= invisible as i16;
    StartSpriteAnim(&raw mut gSprites[(*sNamingScreen).cursorSpriteId], 0);
}
unsafe fn SetCursorFlashing(flashing: u8) {
    gSprites[(*sNamingScreen).cursorSpriteId].data[4] &= 0xFF;
    gSprites[(*sNamingScreen).cursorSpriteId].data[4] |= (flashing as i16) << 8;
}
unsafe fn SquishCursor() {
    StartSpriteAnim(&raw mut gSprites[(*sNamingScreen).cursorSpriteId], 1);
}
unsafe fn IsCursorAnimFinished() -> u8 {
    gSprites[(*sNamingScreen).cursorSpriteId].animEnded() as u8
}
unsafe fn GetKeyRoleAtCursorPos() -> u8 {
    let mut cursorX: i16 = 0;
    let mut cursorY: i16 = 0;
    GetCursorPos(&raw mut cursorX, &raw mut cursorY);
    if cursorX < GetCurrentPageColumnCount() as i16 {
        return KEY_ROLE_CHAR;
    } else {
        return sButtonKeyRoles[cursorY];
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
unsafe fn GetCurrentPageColumnCount() -> u8 {
    sPageColumnCounts[CurrentPageToKeyboardId()]
}
unsafe fn CreatePageSwapButtonSprites() {
    let frameSpriteId: u8 = CreateSprite(
        (&raw const *sSpriteTemplate_PageSwapFrame).cast_mut(),
        204,
        88,
        0,
    );
    (*sNamingScreen).swapBtnFrameSpriteId = frameSpriteId;
    SetSubspriteTables(
        &raw mut gSprites[frameSpriteId],
        sSubspriteTable_PageSwapFrame.as_ptr().cast_mut(),
    );
    gSprites[frameSpriteId].set_invisible(TRUE as u16);
    let textSpriteId: u8 = CreateSprite(
        (&raw const *sSpriteTemplate_PageSwapText).cast_mut(),
        204,
        84,
        1,
    );
    gSprites[frameSpriteId].data[sTextSpriteId] = textSpriteId as i16;
    SetSubspriteTables(
        &raw mut gSprites[textSpriteId],
        sSubspriteTable_PageSwapText.as_ptr().cast_mut(),
    );
    gSprites[textSpriteId].set_invisible(TRUE as u16);
    let buttonSpriteId: u8 = CreateSprite(
        (&raw const *sSpriteTemplate_PageSwapButton).cast_mut(),
        204,
        83,
        2,
    );
    gSprites[buttonSpriteId].oam.set_priority(1);
    gSprites[frameSpriteId].data[sButtonSpriteId] = buttonSpriteId as i16;
    gSprites[buttonSpriteId].set_invisible(TRUE as u16);
}
unsafe fn StartPageSwapButtonAnim() {
    let sprite: *mut Sprite = &raw mut gSprites[(*sNamingScreen).swapBtnFrameSpriteId];
    (*sprite).data[sState] = 2;
    (*sprite).data[sPage] = (*sNamingScreen).currentPage as i16;
}
pub(crate) unsafe fn SpriteCB_PageSwap(sprite: *mut Sprite) {
    while sPageSwapSpriteFuncs[(*sprite).data[sState]].unwrap_unchecked()(sprite) != 0 {}
}
pub(crate) unsafe fn PageSwapSprite_Init(sprite: *mut Sprite) -> u8 {
    let text: *mut Sprite = &raw mut gSprites[(*sprite).data[sTextSpriteId]];
    let button: *mut Sprite = &raw mut gSprites[(*sprite).data[sButtonSpriteId]];
    SetPageSwapButtonGfx(PageToNextGfxId((*sNamingScreen).currentPage), text, button);
    (*sprite).data[sState] += 1;
    FALSE
}
pub(crate) fn PageSwapSprite_Idle(sprite: *mut Sprite) -> u8 {
    FALSE
}
pub(crate) unsafe fn PageSwapSprite_SlideOff(sprite: *mut Sprite) -> u8 {
    let text: *mut Sprite = &raw mut gSprites[(*sprite).data[sTextSpriteId]];
    let button: *mut Sprite = &raw mut gSprites[(*sprite).data[sButtonSpriteId]];
    (*text).y2 += 1;
    if (*text).y2 > 7 {
        (*sprite).data[sState] += 1;
        (*text).y2 = -4;
        (*text).set_invisible(TRUE as u16);
        SetPageSwapButtonGfx(
            PageToNextGfxId((((*sprite).data[sPage] as u8 as i32 + 1) % 3) as u8),
            text,
            button,
        );
    }
    FALSE
}
pub(crate) unsafe fn PageSwapSprite_SlideOn(sprite: *mut Sprite) -> u8 {
    let text: *mut Sprite = &raw mut gSprites[(*sprite).data[sTextSpriteId]];
    (*text).set_invisible(FALSE as u16);
    (*text).y2 += 1;
    if (*text).y2 >= 0 {
        (*text).y2 = 0;
        (*sprite).data[sState] = 1;
    }
    FALSE
}
unsafe fn SetPageSwapButtonGfx(page: u8, text: *mut Sprite, button: *mut Sprite) {
    (*button)
        .oam
        .set_paletteNum(IndexOfSpritePaletteTag(sPageSwapPalTags[page]) as u16);
    (*text).sheetTileStart = GetSpriteTileStartByTag(sPageSwapGfxTags[page]);
    (*text).set_subspriteTableNum(page);
}
unsafe fn CreateBackOkSprites() {
    let mut spriteId: u8 = CreateSprite(
        (&raw const *sSpriteTemplate_BackButton).cast_mut(),
        204,
        116,
        0,
    );
    SetSubspriteTables(
        &raw mut gSprites[spriteId],
        sSubspriteTable_Button.as_ptr().cast_mut(),
    );
    gSprites[spriteId].set_invisible(TRUE as u16);
    spriteId = CreateSprite(
        (&raw const *sSpriteTemplate_OkButton).cast_mut(),
        204,
        140,
        0,
    );
    SetSubspriteTables(
        &raw mut gSprites[spriteId],
        sSubspriteTable_Button.as_ptr().cast_mut(),
    );
    gSprites[spriteId].set_invisible(TRUE as u16);
}
pub(crate) unsafe fn CreateTextEntrySprites() {
    let mut xPos: i16 = (*sNamingScreen).inputCharBaseXPos as i16 - 5;
    let mut spriteId: u8 = CreateSprite(
        (&raw const *sSpriteTemplate_InputArrow).cast_mut(),
        xPos,
        56,
        0,
    );
    gSprites[spriteId].oam.set_priority(3);
    gSprites[spriteId].set_invisible(TRUE as u16);
    xPos = (*sNamingScreen).inputCharBaseXPos as i16;
    let mut i: u8 = 0;
    while i < (*(*sNamingScreen).template).maxChars {
        spriteId = CreateSprite(
            (&raw const *sSpriteTemplate_Underscore).cast_mut(),
            xPos + 3,
            60,
            0,
        );
        gSprites[spriteId].oam.set_priority(3);
        gSprites[spriteId].data[0] = i as i16;
        gSprites[spriteId].set_invisible(TRUE as u16);
        i += 1;
        xPos += 8;
    }
}
unsafe fn CreateInputTargetIcon() {
    sIconFunctions[(*(*sNamingScreen).template).iconFunction].unwrap_unchecked()();
}
pub(crate) fn NamingScreen_NoIcon() {}
pub(crate) unsafe fn NamingScreen_CreatePlayerIcon() {
    let rivalGfxId: u8 = GetRivalAvatarGraphicsIdByStateIdAndGender(
        PLAYER_AVATAR_STATE_NORMAL,
        (*sNamingScreen).monSpecies as u8,
    );
    let spriteId: u8 =
        CreateObjectGraphicsSprite(rivalGfxId as u16, Some(SpriteCallbackDummy), 56, 37, 0);
    gSprites[spriteId].oam.set_priority(3);
    StartSpriteAnim(&raw mut gSprites[spriteId], ANIM_STD_GO_SOUTH);
}
pub(crate) unsafe fn NamingScreen_CreatePCIcon() {
    let spriteId: u8 = CreateSprite((&raw const *sSpriteTemplate_PCIcon).cast_mut(), 56, 41, 0);
    SetSubspriteTables(
        &raw mut gSprites[spriteId],
        sSubspriteTable_PCIcon.as_ptr().cast_mut(),
    );
    gSprites[spriteId].oam.set_priority(3);
}
pub(crate) unsafe fn NamingScreen_CreateMonIcon() {
    LoadMonIconPalettes();
    let spriteId: u8 = CreateMonIcon(
        (*sNamingScreen).monSpecies,
        Some(SpriteCallbackDummy),
        56,
        40,
        0,
        (*sNamingScreen).monPersonality,
        1,
    );
    gSprites[spriteId].oam.set_priority(3);
}
pub(crate) unsafe fn NamingScreen_CreateWaldaDadIcon() {
    let spriteId: u8 =
        CreateObjectGraphicsSprite(OBJ_EVENT_GFX_MAN_1, Some(SpriteCallbackDummy), 56, 37, 0);
    gSprites[spriteId].oam.set_priority(3);
    StartSpriteAnim(&raw mut gSprites[spriteId], ANIM_STD_GO_SOUTH);
}
unsafe fn HandleKeyboardEvent() -> u8 {
    let input: u8 = GetInputEvent();
    let keyRole: u8 = GetKeyRoleAtCursorPos();
    if input == INPUT_SELECT {
        return SwapKeyboardPage();
    } else if input == INPUT_B_BUTTON {
        DeleteTextCharacter();
        return FALSE;
    } else if input == INPUT_START {
        MoveCursorToOKButton();
        return FALSE;
    } else {
        return sKeyboardKeyHandlers[keyRole].unwrap_unchecked()(input);
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
pub(crate) unsafe fn KeyboardKeyHandler_Character(input: u8) -> u8 {
    TryStartButtonFlash(BUTTON_COUNT as u8, FALSE, FALSE);
    if input == INPUT_A_BUTTON {
        let textFull: u8 = AddTextCharacter();
        SquishCursor();
        if textFull != 0 {
            SetInputState(INPUT_STATE_OVERRIDE);
            (*sNamingScreen).state = STATE_MOVE_TO_OK_BUTTON;
        }
    }
    FALSE
}
pub(crate) unsafe fn KeyboardKeyHandler_Page(input: u8) -> u8 {
    TryStartButtonFlash(BUTTON_PAGE, TRUE, FALSE);
    if input == INPUT_A_BUTTON {
        return SwapKeyboardPage();
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
pub(crate) unsafe fn KeyboardKeyHandler_Backspace(input: u8) -> u8 {
    TryStartButtonFlash(BUTTON_BACK, TRUE, FALSE);
    if input == INPUT_A_BUTTON {
        DeleteTextCharacter();
    }
    FALSE
}
pub(crate) unsafe fn KeyboardKeyHandler_OK(input: u8) -> u8 {
    TryStartButtonFlash(BUTTON_OK, TRUE, FALSE);
    if input == INPUT_A_BUTTON {
        PlaySE(SE_SELECT);
        (*sNamingScreen).state = STATE_PRESSED_OK;
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
unsafe fn SwapKeyboardPage() -> u8 {
    (*sNamingScreen).state = STATE_START_PAGE_SWAP;
    TRUE
}
unsafe fn CreateInputHandlerTask() {
    CreateTask(Some(Task_HandleInput), 1);
}
unsafe fn GetInputEvent() -> u8 {
    let taskId: u8 = FindTaskIdByFunc(Some(Task_HandleInput));
    task_get(taskId, tKeyboardEvent) as u8
}
unsafe fn SetInputState(state: u8) {
    let taskId: u8 = FindTaskIdByFunc(Some(Task_HandleInput));
    task_set(taskId, tState, state as i16);
}
pub(crate) unsafe fn Task_HandleInput(taskId: u8) {
    sInputFuncs[task_get(taskId, tState)].unwrap_unchecked()(&raw mut (*gTasks.as_ptr())[taskId]);
}
pub(crate) unsafe fn Input_Disabled(task: *mut Task) {
    (*task).data[tKeyboardEvent] = INPUT_NONE;
}
pub(crate) unsafe fn Input_Enabled(task: *mut Task) {
    (*task).data[tKeyboardEvent] = INPUT_NONE;
    if gMain.newKeys as i32 & A_BUTTON != 0 {
        (*task).data[tKeyboardEvent] = INPUT_A_BUTTON as i16;
    } else if gMain.newKeys as i32 & B_BUTTON != 0 {
        (*task).data[tKeyboardEvent] = INPUT_B_BUTTON as i16;
    } else if gMain.newKeys as i32 & SELECT_BUTTON != 0 {
        (*task).data[tKeyboardEvent] = INPUT_SELECT as i16;
    } else if gMain.newKeys as i32 & START_BUTTON != 0 {
        (*task).data[tKeyboardEvent] = INPUT_START as i16;
    } else {
        HandleDpadMovement(task);
    }
}
pub(crate) unsafe fn Input_Override(task: *mut Task) {
    (*task).data[tKeyboardEvent] = INPUT_NONE;
}
unsafe fn HandleDpadMovement(task: *mut Task) {
    let mut sDpadDeltaX: CArray<i16, 5> = zeroed();
    sDpadDeltaX[0] = 0;
    sDpadDeltaX[1] = 0;
    sDpadDeltaX[2] = 0;
    sDpadDeltaX[3] = -1;
    sDpadDeltaX[4] = 1;
    let mut sDpadDeltaY: CArray<i16, 5> = zeroed();
    sDpadDeltaY[0] = 0;
    sDpadDeltaY[1] = -1;
    sDpadDeltaY[2] = 1;
    sDpadDeltaY[3] = 0;
    sDpadDeltaY[4] = 0;
    let sKeyRowToButtonRow: CArray<i16, 4> = CArray([0, 1, 1, 2]);
    let sButtonRowToKeyRow: CArray<i16, 3> = CArray([0, 0, 3]);
    let mut cursorX: i16 = 0;
    let mut cursorY: i16 = 0;
    GetCursorPos(&raw mut cursorX, &raw mut cursorY);
    let mut input: u16 = INPUT_NONE as u16;
    if gMain.newAndRepeatedKeys as i32 & DPAD_UP != 0 {
        input = INPUT_DPAD_UP;
    }
    if gMain.newAndRepeatedKeys as i32 & DPAD_DOWN != 0 {
        input = INPUT_DPAD_DOWN;
    }
    if gMain.newAndRepeatedKeys as i32 & DPAD_LEFT != 0 {
        input = INPUT_DPAD_LEFT;
    }
    if gMain.newAndRepeatedKeys as i32 & DPAD_RIGHT != 0 {
        input = INPUT_DPAD_RIGHT;
    }
    let prevCursorX: i16 = cursorX;
    cursorX += sDpadDeltaX[input];
    cursorY += sDpadDeltaY[input];
    if cursorX < 0 {
        cursorX = GetCurrentPageColumnCount() as i16;
    }
    if cursorX > GetCurrentPageColumnCount() as i16 {
        cursorX = 0;
    }
    if sDpadDeltaX[input] != 0 {
        if cursorX == GetCurrentPageColumnCount() as i16 {
            (*task).data[2] = cursorY;
            cursorY = sKeyRowToButtonRow[cursorY];
        } else if prevCursorX == GetCurrentPageColumnCount() as i16 {
            if cursorY == 1 {
                cursorY = (*task).data[2];
            } else {
                cursorY = sButtonRowToKeyRow[cursorY];
            }
        }
    }
    if cursorX == GetCurrentPageColumnCount() as i16 {
        if cursorY < 0 {
            cursorY = 2;
        }
        if cursorY >= BUTTON_COUNT {
            cursorY = 0;
        }
        if cursorY == 0 {
            (*task).data[2] = BUTTON_BACK as i16;
        } else if cursorY == 2 {
            (*task).data[2] = BUTTON_OK as i16;
        }
    } else {
        if cursorY < 0 {
            cursorY = 3;
        }
        if cursorY > 3 {
            cursorY = 0;
        }
    }
    SetCursorPos(cursorX, cursorY);
}
pub(crate) unsafe fn DrawNormalTextEntryBox() {
    FillWindowPixelBuffer((*sNamingScreen).windows[3], 17);
    AddTextPrinterParameterized(
        (*sNamingScreen).windows[3],
        FONT_NORMAL,
        (*(*sNamingScreen).template).title,
        8,
        1,
        0,
        None,
    );
    PutWindowTilemap((*sNamingScreen).windows[3]);
}
pub(crate) unsafe fn DrawMonTextEntryBox() {
    let mut buffer: CArray<u8, 32> = zeroed();
    StringCopy(
        buffer.as_mut_ptr(),
        (*(&raw const crate::data::data_tables::gSpeciesNames).cast::<CArray<CArray<u8, 11>, 0>>())
            [(*sNamingScreen).monSpecies]
            .as_ptr()
            .cast_mut(),
    );
    StringAppendN(buffer.as_mut_ptr(), (*(*sNamingScreen).template).title, 15);
    FillWindowPixelBuffer((*sNamingScreen).windows[3], 17);
    AddTextPrinterParameterized(
        (*sNamingScreen).windows[3],
        FONT_NORMAL,
        buffer.as_mut_ptr(),
        8,
        1,
        0,
        None,
    );
    PutWindowTilemap((*sNamingScreen).windows[3]);
}
unsafe fn DrawTextEntryBox() {
    sDrawTextEntryBoxFuncs[(*sNamingScreen).templateNum].unwrap_unchecked()();
}
unsafe fn TryDrawGenderIcon() {
    sDrawGenderIconFuncs[(*(*sNamingScreen).template).addGenderIcon].unwrap_unchecked()();
}
pub(crate) fn DummyGenderIcon() {}
pub(crate) unsafe fn DrawGenderIcon() {
    let mut text: CArray<u8, 2> = zeroed();
    let mut isFemale: u8 = FALSE;
    StringCopy(
        text.as_mut_ptr(),
        (*(&raw const crate::data::strings::gText_MaleSymbol).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
    );
    if (*sNamingScreen).monGender != MON_GENDERLESS as u16 {
        if (*sNamingScreen).monGender == MON_FEMALE as u16 {
            StringCopy(
                text.as_mut_ptr(),
                (*(&raw const crate::data::strings::gText_FemaleSymbol).cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut(),
            );
            isFemale = TRUE;
        }
        AddTextPrinterParameterized3(
            (*sNamingScreen).windows[2],
            FONT_NORMAL,
            104,
            1,
            sGenderColors[isFemale].as_ptr().cast_mut(),
            TEXT_SKIP_DRAW as i8,
            text.as_mut_ptr(),
        );
    }
}
unsafe fn GetCharAtKeyboardPos(x: i16, y: i16) -> u8 {
    sKeyboardChars[CurrentPageToKeyboardId()][y][x]
}
unsafe fn GetTextEntryPosition() -> u8 {
    for i in 0..(*(*sNamingScreen).template).maxChars {
        if (*sNamingScreen).textBuffer[i] == EOS {
            return i;
        }
    }
    (*(*sNamingScreen).template).maxChars - 1
}
unsafe fn GetPreviousTextCaretPosition() -> u8 {
    let mut i: i8 = (*(*sNamingScreen).template).maxChars as i8 - 1;
    while i > 0 {
        if (*sNamingScreen).textBuffer[i] != EOS {
            return i as u8;
        }
        i -= 1;
    }
    0
}
unsafe fn DeleteTextCharacter() {
    let index: u8 = GetPreviousTextCaretPosition();
    (*sNamingScreen).textBuffer[index] = 0;
    DrawTextEntry();
    CopyBgTilemapBufferToVram(3);
    (*sNamingScreen).textBuffer[index] = EOS;
    let keyRole: u8 = GetKeyRoleAtCursorPos();
    if keyRole == KEY_ROLE_CHAR || keyRole == KEY_ROLE_BACKSPACE {
        TryStartButtonFlash(BUTTON_BACK, FALSE, TRUE);
    }
    PlaySE(SE_BALL);
}
unsafe fn AddTextCharacter() -> u8 {
    let mut x: i16 = 0;
    let mut y: i16 = 0;
    GetCursorPos(&raw mut x, &raw mut y);
    BufferCharacter(GetCharAtKeyboardPos(x, y));
    DrawTextEntry();
    CopyBgTilemapBufferToVram(3);
    PlaySE(SE_SELECT);
    if GetPreviousTextCaretPosition() as i32 != (*(*sNamingScreen).template).maxChars as i32 - 1 {
        return FALSE;
    } else {
        return TRUE;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
unsafe fn BufferCharacter(ch: u8) {
    let index: u8 = GetTextEntryPosition();
    (*sNamingScreen).textBuffer[index] = ch;
}
unsafe fn SaveInputText() {
    let mut i: u8 = 0;
    while i < (*(*sNamingScreen).template).maxChars {
        if (*sNamingScreen).textBuffer[i] != CHAR_SPACE && (*sNamingScreen).textBuffer[i] != EOS {
            StringCopyN(
                (*sNamingScreen).destBuffer,
                (*sNamingScreen).textBuffer.as_mut_ptr(),
                (*(*sNamingScreen).template).maxChars + 1,
            );
            break;
        }
        i += 1;
    }
}
pub(crate) unsafe fn LoadGfx() {
    LZ77UnCompWram(
        (*(&raw const crate::data::graphics::gNamingScreenMenu_Gfx).cast::<CArray<u32, 0>>())
            .as_ptr()
            .cast_mut(),
        (*sNamingScreen).tileBuffer.as_mut_ptr() as *mut c_void,
    );
    LoadBgTiles(
        1,
        (*sNamingScreen).tileBuffer.as_mut_ptr() as *mut c_void,
        1536,
        0,
    );
    LoadBgTiles(
        2,
        (*sNamingScreen).tileBuffer.as_mut_ptr() as *mut c_void,
        1536,
        0,
    );
    LoadBgTiles(
        3,
        (*sNamingScreen).tileBuffer.as_mut_ptr() as *mut c_void,
        1536,
        0,
    );
    LoadSpriteSheets(sSpriteSheets.as_ptr().cast_mut());
    LoadSpritePalettes(sSpritePalettes.as_ptr().cast_mut());
}
unsafe fn CreateHelperTasks() {
    CreateInputHandlerTask();
    CreateButtonFlashTask();
}
unsafe fn LoadPalettes() {
    LoadPalette(
        (*(&raw const crate::data::graphics::gNamingScreenMenu_Pal)
            .cast::<CArray<CArray<u16, 16>, 6>>())
        .as_ptr()
        .cast_mut() as *mut c_void,
        0,
        192,
    );
    LoadPalette(sKeyboard_Pal.as_ptr().cast_mut() as *mut c_void, 160, 32);
    LoadPalette(GetTextWindowPalette(2) as *mut c_void, 176, 32);
}
unsafe fn DrawBgTilemap(bg: u8, src: *mut c_void) {
    CopyToBgTilemapBuffer(bg, src, 0, 0);
}
fn NamingScreen_Dummy(bg: u8, page: u8) {}
unsafe fn DrawTextEntry() {
    let mut temp: CArray<u8, 2> = zeroed();
    let mut extraWidth: u16 = 0;
    let maxChars: u8 = (*(*sNamingScreen).template).maxChars;
    let x: u16 = (*sNamingScreen).inputCharBaseXPos - 0x40;
    FillWindowPixelBuffer((*sNamingScreen).windows[2], 17);
    for i in 0..maxChars {
        temp[0] = (*sNamingScreen).textBuffer[i];
        temp[1] = (*(&raw const crate::data::strings::gText_ExpandedPlaceholder_Empty)
            .cast::<CArray<u8, 0>>())[0];
        extraWidth = (if IsWideLetter(temp[0]) == TRUE { 2 } else { 0 }) as u16;
        AddTextPrinterParameterized(
            (*sNamingScreen).windows[2],
            FONT_NORMAL,
            temp.as_mut_ptr(),
            i * 8 + x as u8 + extraWidth as u8,
            1,
            TEXT_SKIP_DRAW,
            None,
        );
    }
    TryDrawGenderIcon();
    CopyWindowToVram((*sNamingScreen).windows[2], COPYWIN_GFX);
    PutWindowTilemap((*sNamingScreen).windows[2]);
}
unsafe fn PrintKeyboardKeys(window: u8, page: u8) {
    FillWindowPixelBuffer(window, sFillValues[page]);
    for i in 0..KBROW_COUNT {
        AddTextPrinterParameterized3(
            window,
            FONT_NORMAL,
            0,
            i * 16 + 1,
            sKeyboardTextColors[page],
            0,
            sNamingScreenKeyboardText[page][i],
        );
    }
    PutWindowTilemap(window);
}
unsafe fn DrawKeyboardPageOnDeck() {
    let mut bg: u8 = 0;
    let mut bg_: u8 = 0;
    let mut windowId: u8 = 0;
    let bg1Priority: u8 = GetGpuReg(REG_OFFSET_BG1CNT) as u8 & 3;
    let bg2Priority: u8 = GetGpuReg(REG_OFFSET_BG2CNT) as u8 & 3;
    if bg1Priority > bg2Priority {
        bg = 1;
        bg_ = 1;
        windowId = (*sNamingScreen).windows[0];
    } else {
        bg = 2;
        bg_ = 2;
        windowId = (*sNamingScreen).windows[1];
    }
    DrawBgTilemap(
        bg,
        sNextKeyboardPageTilemaps[(*sNamingScreen).currentPage] as *mut c_void,
    );
    PrintKeyboardKeys(windowId, CurrentPageToNextKeyboardId());
    NamingScreen_Dummy(bg, CurrentPageToNextKeyboardId());
    CopyBgTilemapBufferToVram(bg_);
}
unsafe fn PrintControls() {
    let mut color: CArray<u8, 3> = CArray([15, 1, 2]);
    FillWindowPixelBuffer((*sNamingScreen).windows[4], 255);
    AddTextPrinterParameterized3(
        (*sNamingScreen).windows[4],
        FONT_SMALL,
        2,
        1,
        color.as_mut_ptr(),
        0,
        (*(&raw const crate::data::strings::gText_MoveOkBack).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
    );
    PutWindowTilemap((*sNamingScreen).windows[4]);
    CopyWindowToVram((*sNamingScreen).windows[4], COPYWIN_FULL);
}
pub(crate) unsafe fn CB2_NamingScreen() {
    RunTasks();
    AnimateSprites();
    BuildOamBuffer();
    UpdatePaletteFade();
}
unsafe fn ResetVHBlank() {
    SetVBlankCallback(None);
    SetHBlankCallback(None);
}
unsafe fn SetVBlank() {
    SetVBlankCallback(Some(VBlankCB_NamingScreen));
}
pub(crate) unsafe fn VBlankCB_NamingScreen() {
    LoadOam();
    ProcessSpriteCopyRequests();
    TransferPlttBuffer();
    SetGpuReg(REG_OFFSET_BG1VOFS, (*sNamingScreen).bg1vOffset);
    SetGpuReg(REG_OFFSET_BG2VOFS, (*sNamingScreen).bg2vOffset);
    SetGpuReg(REG_OFFSET_BG1CNT, GetGpuReg(REG_OFFSET_BG1CNT) & 0xFFFC);
    SetGpuRegBits(REG_OFFSET_BG1CNT, (*sNamingScreen).bg1Priority);
    SetGpuReg(REG_OFFSET_BG2CNT, GetGpuReg(REG_OFFSET_BG2CNT) & 0xFFFC);
    SetGpuRegBits(REG_OFFSET_BG2CNT, (*sNamingScreen).bg2Priority);
}
unsafe fn NamingScreen_ShowBgs() {
    ShowBg(0);
    ShowBg(1);
    ShowBg(2);
    ShowBg(3);
}
fn IsWideLetter(character: u8) -> u8 {
    let mut i: u8 = 0;
    while sText_AlphabetUpperLower[i] != EOS {
        if character == sText_AlphabetUpperLower[i] {
            return FALSE;
        }
        i += 1;
    }
    FALSE
}
unsafe fn Debug_NamingScreenPlayer() {
    DoNamingScreen(
        NAMING_SCREEN_PLAYER,
        (*gSaveBlock2Ptr).playerName.as_mut_ptr(),
        (*gSaveBlock2Ptr).playerGender as u16,
        0,
        0,
        Some(CB2_ReturnToFieldWithOpenMenu),
    );
}
unsafe fn Debug_NamingScreenBox() {
    DoNamingScreen(
        NAMING_SCREEN_BOX,
        (*gSaveBlock2Ptr).playerName.as_mut_ptr(),
        (*gSaveBlock2Ptr).playerGender as u16,
        0,
        0,
        Some(CB2_ReturnToFieldWithOpenMenu),
    );
}
unsafe fn Debug_NamingScreenCaughtMon() {
    DoNamingScreen(
        NAMING_SCREEN_CAUGHT_MON,
        (*gSaveBlock2Ptr).playerName.as_mut_ptr(),
        (*gSaveBlock2Ptr).playerGender as u16,
        0,
        0,
        Some(CB2_ReturnToFieldWithOpenMenu),
    );
}
unsafe fn Debug_NamingScreenNickname() {
    DoNamingScreen(
        NAMING_SCREEN_NICKNAME,
        (*gSaveBlock2Ptr).playerName.as_mut_ptr(),
        (*gSaveBlock2Ptr).playerGender as u16,
        0,
        0,
        Some(CB2_ReturnToFieldWithOpenMenu),
    );
}
