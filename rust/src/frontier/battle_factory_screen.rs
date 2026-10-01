//! Translated from `src/battle_factory_screen.c` by tools/rustport/c2rs.py.
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
    clippy::type_complexity,
    clippy::useless_transmute,
    dead_code,
    unused_assignments
)]

use crate::agb_main::gMain;
use crate::agb_main::{SetHBlankCallback, SetVBlankCallback};
use crate::battle_factory::{GetFactoryMonFixedIV, GetNumPastRentalsRank, SetMonMoveAvoidReturn};
use crate::battle_tower::gFacilityTrainerMons;
use crate::bg::{
    ChangeBgX, ChangeBgY, CopyBgTilemapBufferToVram, HideBg, ResetBgsAndClearDma3BusyFlags, ShowBg,
};
use crate::bg::{CopyToBgTilemapBufferRect, LoadBgTilemap, LoadBgTiles};
use crate::box_mon::GetBoxMonData3;
#[allow(unused_imports)]
use crate::c::*;
#[allow(unused_imports)]
use crate::consts::*;
use crate::event_data::VarGet;
use crate::ffi::gSpecialVar_Result;
use crate::gpu_regs::{ClearGpuRegBits, SetGpuReg, SetGpuRegBits};
use crate::international_string_util::{CopyMonCategoryText, GetStringRightAlignXOffset};
use crate::load_save::gSaveBlock2Ptr;
use crate::menu::AddTextPrinterParameterized3;
use crate::overworld::CB2_ReturnToFieldContinueScript;
use crate::palette::{
    BeginNormalPaletteFade, BlendPalettes, LoadPalette, ResetPaletteFade, TransferPlttBuffer,
    UpdatePaletteFade, gPaletteFade,
};
use crate::palette::{gPlttBufferFaded, gPlttBufferUnfaded};
use crate::pokemon::{
    CalculatePlayerPartyCount, CreateMonWithEVSpreadNatureOTID, GetMonData3, SetMonData,
    SpeciesToNationalPokedexNum, gEnemyParty, gPlayerParty,
};
use crate::pokemon_summary_screen::{ShowPokemonSummaryScreen, gLastViewedMonIndex};
use crate::random::Random;
use crate::sound::PlaySE;
use crate::sprite::gSprites;
use crate::sprite::{
    AnimateSprites, BuildOamBuffer, FreeAllSpritePalettes, FreeOamMatrix, IndexOfSpritePaletteTag,
    LoadOam, ProcessSpriteCopyRequests, ResetSpriteData,
};
use crate::string_util::StringCopy;
use crate::string_util::gStringVar4;
use crate::task::gTasks;
use crate::task::{DestroyTask, ResetTasks, RunTasks};
use crate::task::{task_func, task_get, task_set, task_set_func};
use crate::text::{DeactivateAllTextPrinters, RunTextPrinters};
use crate::trainer_pokemon_sprites::{
    CreateMonPicSprite_HandleDeoxys, FreeAndDestroyMonPicSprite, ResetAllPicSprites,
};
#[allow(unused_imports)]
use crate::types::*;
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
/// `InitBgsFromTemplates` with this module's view of its types.
#[inline]
unsafe fn InitBgsFromTemplates(a0: u8, a1: *mut BgTemplate, a2: u8) {
    unsafe {
        crate::bg::InitBgsFromTemplates(a0, a1 as _, a2);
    }
}
/// `InitWindows` with this module's view of its types.
#[inline]
unsafe fn InitWindows(a0: *mut WindowTemplate) -> u16 {
    unsafe { crate::window::InitWindows(a0 as _) }
}
/// `LoadCompressedSpriteSheet` with this module's view of its types.
#[inline]
unsafe fn LoadCompressedSpriteSheet(a0: *mut CompressedSpriteSheet) -> u16 {
    unsafe { crate::decompress::LoadCompressedSpriteSheet(a0 as _) }
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
/// `StartSpriteAnimIfDifferent` with this module's view of its types.
#[inline]
unsafe fn StartSpriteAnimIfDifferent(a0: *mut Sprite, a1: u8) {
    unsafe {
        crate::sprite::StartSpriteAnimIfDifferent(a0 as _, a1);
    }
}
// The C's names for task and sprite data slots.
const tState: usize = 0;
const tSaidYes: usize = 1;
const tTaskId: usize = 1;
const tSlidingOn: usize = 2;
const tWinLeft: usize = 3;
const tFadeOutFinished: usize = 4;
const tSlideFinishedCancel: usize = 4;
const tWinRight: usize = 4;
const tFollowUpTaskState: usize = 5;
const tWinTop: usize = 5;
const tFollowUpTaskPtrHi: usize = 6;
const tSpriteId: usize = 6;
const sIsSwapScreen: usize = 7;
const tFollowUpTaskPtrLo: usize = 7;
const tIsSwapScreen: usize = 7;
const tWinBottom: usize = 8;
// Data tables (translate with cdata.py): sPokeballGray_Pal sPokeballSelected_Pal sInterface_Pal sPokeball_Gfx sArrow_Gfx sMenuHighlightLeft_Gfx sMenuHighlightRight_Gfx sActionBoxLeft_Gfx sActionBoxRight_Gfx sActionHighlightLeft_Gfx sActionHighlightMiddle_Gfx sActionHighlightRight_Gfx sMonPicBgAnim_Gfx sMonPicBg_Tilemap sMonPicBg_Gfx sMonPicBg_Pal sSelect_SpriteSheets sSelect_BallGfx sSelect_SpritePalettes sSelect_MenuOptionFuncs sSelect_BgTemplates sSelect_WindowTemplates sSelectText_Pal sMenuOptionTextColors sSpeciesNameTextColors sOam_Select_Pokeball sOam_Select_Arrow sOam_Select_MenuHighlight sOam_Select_MonPicBgAnim sAnim_Select_Interface sAnim_Select_MonPicBgAnim sAnim_Select_Pokeball_Still sAnim_Select_Pokeball_Moving sAnims_Select_Interface sAnims_Select_MonPicBgAnim sAnims_Select_Pokeball sAffineAnim_Select_MonPicBg_Opening sAffineAnim_Select_MonPicBg_Closing sAffineAnim_Select_MonPicBg_Open sAffineAnims_Select_MonPicBgAnim sSpriteTemplate_Select_Pokeball sSpriteTemplate_Select_Arrow sSpriteTemplate_Select_MenuHighlightLeft sSpriteTemplate_Select_MenuHighlightRight sSpriteTemplate_Select_MonPicBgAnim sSwap_SpriteSheets sSwap_BallGfx sSwap_SpritePalettes sOam_Swap_Pokeball sOam_Swap_Arrow sOam_Swap_MenuHighlight sOam_Swap_MonPicBgAnim sAnim_Swap_Interface sAnim_Swap_MonPicBgAnim sAnim_Swap_Pokeball_Still sAnim_Swap_Pokeball_Moving sAnims_Swap_Interface sAnims_Swap_MonPicBgAnim sAnims_Swap_Pokeball sAffineAnim_Swap_MonPicBg_Opening sAffineAnim_Swap_MonPicBg_Closing sAffineAnim_Swap_MonPicBg_Open sAffineAnims_Swap_MonPicBgAnim sSpriteTemplate_Swap_Pokeball sSpriteTemplate_Swap_Arrow sSpriteTemplate_Swap_MenuHighlightLeft sSpriteTemplate_Swap_MenuHighlightRight sSpriteTemplate_Swap_MonPicBgAnim sSwap_MenuOptionFuncs sSwap_BgTemplates sSwap_WindowTemplates sSwapText_Pal sSwapMenuOptionsTextColors sSwapSpeciesNameTextColors sSwap_PlayerScreenActions sSwap_EnemyScreenActions

/// `struct FactorySelectScreen`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct FactorySelectScreen {
    pub menuCursorPos: u8,
    pub menuCursor1SpriteId: u8,
    pub menuCursor2SpriteId: u8,
    pub cursorPos: u8,
    pub cursorSpriteId: u8,
    pub selectingMonsState: u8,
    pub fromSummaryScreen: u8,
    pub yesNoCursorPos: u8,
    pub unused: u8,
    pub mons: CArray<FactorySelectableMon, 6>,
    pub monPics: CArray<FactoryMonPic, 3>,
    pub monPicAnimating: u8,
    pub fadeSpeciesNameTaskId: u8,
    pub fadeSpeciesNameActive: u8,
    pub speciesNameColorBackup: u16,
    pub fadeSpeciesNameFadeOut: u8,
    pub fadeSpeciesNameCoeffDelay: u8,
    pub fadeSpeciesNameCoeff: u8,
    pub faceSpeciesNameDelay: u8,
}

unsafe impl Sync for FactorySelectScreen {}

/// `struct FactorySwapScreen`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct FactorySwapScreen {
    pub menuCursorPos: u8,
    pub menuCursor1SpriteId: u8,
    pub menuCursor2SpriteId: u8,
    pub cursorPos: u8,
    pub cursorSpriteId: u8,
    pub ballSpriteIds: CArray<u8, 3>,
    pub pkmnForSwapButtonSpriteIds: CArray<CArray<u8, 3>, 2>,
    pub cancelButtonSpriteIds: CArray<CArray<u8, 2>, 2>,
    pub playerMonId: u8,
    pub enemyMonId: u8,
    pub inEnemyScreen: u8,
    pub fromSummaryScreen: u8,
    pub yesNoCursorPos: u8,
    pub actionsCount: u8,
    pub actionsData: *mut SwapScreenAction,
    pub unused: CArray<u8, 4>,
    pub monSwapped: u8,
    pub fadeSpeciesNameTaskId: u8,
    pub fadeSpeciesNameActive: u8,
    pub speciesNameColorBackup: u16,
    pub fadeSpeciesNameFadeOut: u8,
    pub fadeSpeciesNameCoeffDelay: u8,
    pub fadeSpeciesNameCoeff: u8,
    pub faceSpeciesNameDelay: u8,
    pub monPic: FactoryMonPic,
    pub monPicAnimating: u8,
}

unsafe impl Sync for FactorySwapScreen {}

/// `struct FactoryMonPic`
#[repr(C, align(4))]
#[derive(Clone, Copy)]
pub struct FactoryMonPic {
    pub monSpriteId: u8,
    pub bgSpriteId: u8,
}

unsafe impl Sync for FactoryMonPic {}

/// `struct SwapScreenAction`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SwapScreenAction {
    pub id: u8,
    pub func: Option<unsafe fn(u8)>,
}

unsafe impl Sync for SwapScreenAction {}

/// `struct FactorySelectableMon`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct FactorySelectableMon {
    pub monId: u16,
    pub ballSpriteId: u16,
    pub selectedId: u8,
    pub monData: Pokemon,
}

unsafe impl Sync for FactorySelectableMon {}

#[cfg(target_arch = "arm")]
const _: () = {
    #[allow(unused_imports)]
    use core::mem::{offset_of, size_of};
    assert!(size_of::<FactorySelectScreen>() == 684);
    assert!(offset_of!(FactorySelectScreen, menuCursorPos) == 0);
    assert!(offset_of!(FactorySelectScreen, menuCursor1SpriteId) == 1);
    assert!(offset_of!(FactorySelectScreen, menuCursor2SpriteId) == 2);
    assert!(offset_of!(FactorySelectScreen, cursorPos) == 3);
    assert!(offset_of!(FactorySelectScreen, cursorSpriteId) == 4);
    assert!(offset_of!(FactorySelectScreen, selectingMonsState) == 5);
    assert!(offset_of!(FactorySelectScreen, fromSummaryScreen) == 6);
    assert!(offset_of!(FactorySelectScreen, yesNoCursorPos) == 7);
    assert!(offset_of!(FactorySelectScreen, unused) == 8);
    assert!(offset_of!(FactorySelectScreen, mons) == 12);
    assert!(offset_of!(FactorySelectScreen, monPics) == 660);
    assert!(offset_of!(FactorySelectScreen, monPicAnimating) == 672);
    assert!(offset_of!(FactorySelectScreen, fadeSpeciesNameTaskId) == 673);
    assert!(offset_of!(FactorySelectScreen, fadeSpeciesNameActive) == 674);
    assert!(offset_of!(FactorySelectScreen, speciesNameColorBackup) == 676);
    assert!(offset_of!(FactorySelectScreen, fadeSpeciesNameFadeOut) == 678);
    assert!(offset_of!(FactorySelectScreen, fadeSpeciesNameCoeffDelay) == 679);
    assert!(offset_of!(FactorySelectScreen, fadeSpeciesNameCoeff) == 680);
    assert!(offset_of!(FactorySelectScreen, faceSpeciesNameDelay) == 681);
    assert!(size_of::<FactorySwapScreen>() == 52);
    assert!(offset_of!(FactorySwapScreen, menuCursorPos) == 0);
    assert!(offset_of!(FactorySwapScreen, menuCursor1SpriteId) == 1);
    assert!(offset_of!(FactorySwapScreen, menuCursor2SpriteId) == 2);
    assert!(offset_of!(FactorySwapScreen, cursorPos) == 3);
    assert!(offset_of!(FactorySwapScreen, cursorSpriteId) == 4);
    assert!(offset_of!(FactorySwapScreen, ballSpriteIds) == 5);
    assert!(offset_of!(FactorySwapScreen, pkmnForSwapButtonSpriteIds) == 8);
    assert!(offset_of!(FactorySwapScreen, cancelButtonSpriteIds) == 14);
    assert!(offset_of!(FactorySwapScreen, playerMonId) == 18);
    assert!(offset_of!(FactorySwapScreen, enemyMonId) == 19);
    assert!(offset_of!(FactorySwapScreen, inEnemyScreen) == 20);
    assert!(offset_of!(FactorySwapScreen, fromSummaryScreen) == 21);
    assert!(offset_of!(FactorySwapScreen, yesNoCursorPos) == 22);
    assert!(offset_of!(FactorySwapScreen, actionsCount) == 23);
    assert!(offset_of!(FactorySwapScreen, actionsData) == 24);
    assert!(offset_of!(FactorySwapScreen, unused) == 28);
    assert!(offset_of!(FactorySwapScreen, monSwapped) == 32);
    assert!(offset_of!(FactorySwapScreen, fadeSpeciesNameTaskId) == 33);
    assert!(offset_of!(FactorySwapScreen, fadeSpeciesNameActive) == 34);
    assert!(offset_of!(FactorySwapScreen, speciesNameColorBackup) == 36);
    assert!(offset_of!(FactorySwapScreen, fadeSpeciesNameFadeOut) == 38);
    assert!(offset_of!(FactorySwapScreen, fadeSpeciesNameCoeffDelay) == 39);
    assert!(offset_of!(FactorySwapScreen, fadeSpeciesNameCoeff) == 40);
    assert!(offset_of!(FactorySwapScreen, faceSpeciesNameDelay) == 41);
    assert!(offset_of!(FactorySwapScreen, monPic) == 44);
    assert!(offset_of!(FactorySwapScreen, monPicAnimating) == 48);
    assert!(size_of::<FactoryMonPic>() == 4);
    assert!(offset_of!(FactoryMonPic, monSpriteId) == 0);
    assert!(offset_of!(FactoryMonPic, bgSpriteId) == 1);
    assert!(size_of::<SwapScreenAction>() == 8);
    assert!(offset_of!(SwapScreenAction, id) == 0);
    assert!(offset_of!(SwapScreenAction, func) == 4);
    assert!(size_of::<FactorySelectableMon>() == 108);
    assert!(offset_of!(FactorySelectableMon, monId) == 0);
    assert!(offset_of!(FactorySelectableMon, ballSpriteId) == 2);
    assert!(offset_of!(FactorySelectableMon, selectedId) == 4);
    assert!(offset_of!(FactorySelectableMon, monData) == 8);
};

const FADESTATE_DELAY: i16 = 2;
const FADESTATE_INIT: i16 = 0;
const FADESTATE_RUN: i16 = 1;
const GFXTAG_ACTION_BOX_LEFT: u16 = 104;
const GFXTAG_ACTION_BOX_RIGHT: u16 = 105;
const GFXTAG_ACTION_HIGHLIGHT_LEFT: u16 = 106;
const GFXTAG_ACTION_HIGHLIGHT_MIDDLE: u16 = 107;
const GFXTAG_ACTION_HIGHLIGHT_RIGHT: u16 = 108;
const PALNUM_FADE_TEXT: i32 = 14;
const PALNUM_TEXT: i32 = 15;
const PALTAG_BALL_GRAY: u16 = 100;
const PALTAG_BALL_SELECTED: u16 = 101;
const SELECTABLE_MONS_COUNT: u8 = 6;
const SELECT_CONFIRM_MONS: u8 = 2;
const SELECT_CONTINUE_CHOOSING: u8 = 1;
const SELECT_INVALID_MON: u8 = 3;
const SELECT_SUMMARY: u8 = 0;
const SELECT_WIN_INFO: u8 = 2;
const SELECT_WIN_MON_CATEGORY: u8 = 5;
const SELECT_WIN_OPTIONS: u8 = 3;
const SELECT_WIN_SPECIES: u8 = 1;
const SELECT_WIN_TITLE: u8 = 0;
const SELECT_WIN_YES_NO: u8 = 4;
const SLIDE_BUTTON_CANCEL: i16 = 1;
const SLIDE_BUTTON_PKMN: i16 = 0;
const STATE_CHOOSE_MONS_HANDLE_INPUT: i16 = 1;
const STATE_CHOOSE_MONS_INIT: i16 = 0;
const STATE_CHOOSE_MONS_INVALID: i16 = 11;
const STATE_MENU_HANDLE_INPUT: i16 = 3;
const STATE_MENU_INIT: i16 = 2;
const STATE_MENU_REINIT: i16 = 12;
const STATE_MENU_RESHOW: i16 = 13;
const STATE_MENU_SHOW_OPTIONS: i16 = 9;
const STATE_SUMMARY_CLEAN: i16 = 7;
const STATE_SUMMARY_FADE: i16 = 6;
const STATE_SUMMARY_SHOW: i16 = 8;
const STATE_YESNO_HANDLE_INPUT: i16 = 5;
const STATE_YESNO_SHOW: i16 = 4;
const STATE_YESNO_SHOW_MONS: i16 = 10;
const STATE_YESNO_SHOW_OPTIONS: i16 = 4;
const SWAPACTION_CANCEL: u8 = 3;
const SWAPACTION_PKMN_FOR_SWAP: u8 = 2;
const SWAP_ENEMY_SCREEN: u8 = 1;
const SWAP_PLAYER_SCREEN: u8 = 0;
const SWAP_WIN_ACTION_FADE: u8 = 5;
const SWAP_WIN_INFO: u8 = 2;
const SWAP_WIN_MON_CATEGORY: u8 = 8;
const SWAP_WIN_OPTIONS: u8 = 3;
const SWAP_WIN_SPECIES: u8 = 1;
const SWAP_WIN_SPECIES_AT_FADE: u8 = 7;
const SWAP_WIN_TITLE: u8 = 0;
const SWAP_WIN_YES_NO: u8 = 4;

static sMenuOptionTextColors: Table<CArray<u8, 3>> =
    Table((&raw const crate::data::battle_factory_screen::sMenuOptionTextColors).cast());
static sMonPicBg_Gfx: Table<CArray<u16, 48>> =
    Table((&raw const crate::data::battle_factory_screen::sMonPicBg_Gfx).cast());
static sMonPicBg_Pal: Table<CArray<u16, 16>> =
    Table((&raw const crate::data::battle_factory_screen::sMonPicBg_Pal).cast());
static sMonPicBg_Tilemap: Table<CArray<u8, 256>> =
    Table((&raw const crate::data::battle_factory_screen::sMonPicBg_Tilemap).cast());
static sPokeballGray_Pal: Table<CArray<u16, 16>> =
    Table((&raw const crate::data::battle_factory_screen::sPokeballGray_Pal).cast());
static sSelectText_Pal: Table<CArray<u16, 5>> =
    Table((&raw const crate::data::battle_factory_screen::sSelectText_Pal).cast());
static sSelect_BallGfx: Table<CArray<CompressedSpriteSheet, 2>> =
    Table((&raw const crate::data::battle_factory_screen::sSelect_BallGfx).cast());
static sSelect_BgTemplates: Table<CArray<BgTemplate, 3>> =
    Table((&raw const crate::data::battle_factory_screen::sSelect_BgTemplates).cast());
static sSelect_MenuOptionFuncs: Table<CArray<Option<unsafe fn() -> u8>, 3>> =
    Table((&raw const crate::data::battle_factory_screen::sSelect_MenuOptionFuncs).cast());
static sSelect_SpritePalettes: Table<CArray<SpritePalette, 5>> =
    Table((&raw const crate::data::battle_factory_screen::sSelect_SpritePalettes).cast());
static sSelect_SpriteSheets: Table<CArray<SpriteSheet, 5>> =
    Table((&raw const crate::data::battle_factory_screen::sSelect_SpriteSheets).cast());
static sSelect_WindowTemplates: Table<CArray<WindowTemplate, 7>> =
    Table((&raw const crate::data::battle_factory_screen::sSelect_WindowTemplates).cast());
static sSpeciesNameTextColors: Table<CArray<u8, 3>> =
    Table((&raw const crate::data::battle_factory_screen::sSpeciesNameTextColors).cast());
static sSpriteTemplate_Select_Arrow: Table<SpriteTemplate> =
    Table((&raw const crate::data::battle_factory_screen::sSpriteTemplate_Select_Arrow).cast());
static sSpriteTemplate_Select_MenuHighlightLeft: Table<SpriteTemplate> = Table(
    (&raw const crate::data::battle_factory_screen::sSpriteTemplate_Select_MenuHighlightLeft)
        .cast(),
);
static sSpriteTemplate_Select_MenuHighlightRight: Table<SpriteTemplate> = Table(
    (&raw const crate::data::battle_factory_screen::sSpriteTemplate_Select_MenuHighlightRight)
        .cast(),
);
static sSpriteTemplate_Select_MonPicBgAnim: Table<SpriteTemplate> = Table(
    (&raw const crate::data::battle_factory_screen::sSpriteTemplate_Select_MonPicBgAnim).cast(),
);
static sSpriteTemplate_Select_Pokeball: Table<SpriteTemplate> =
    Table((&raw const crate::data::battle_factory_screen::sSpriteTemplate_Select_Pokeball).cast());
static sSpriteTemplate_Swap_Arrow: Table<SpriteTemplate> =
    Table((&raw const crate::data::battle_factory_screen::sSpriteTemplate_Swap_Arrow).cast());
static sSpriteTemplate_Swap_MenuHighlightLeft: Table<SpriteTemplate> = Table(
    (&raw const crate::data::battle_factory_screen::sSpriteTemplate_Swap_MenuHighlightLeft).cast(),
);
static sSpriteTemplate_Swap_MenuHighlightRight: Table<SpriteTemplate> = Table(
    (&raw const crate::data::battle_factory_screen::sSpriteTemplate_Swap_MenuHighlightRight).cast(),
);
static sSpriteTemplate_Swap_MonPicBgAnim: Table<SpriteTemplate> = Table(
    (&raw const crate::data::battle_factory_screen::sSpriteTemplate_Swap_MonPicBgAnim).cast(),
);
static sSpriteTemplate_Swap_Pokeball: Table<SpriteTemplate> =
    Table((&raw const crate::data::battle_factory_screen::sSpriteTemplate_Swap_Pokeball).cast());
static sSwapMenuOptionsTextColors: Table<CArray<u8, 3>> =
    Table((&raw const crate::data::battle_factory_screen::sSwapMenuOptionsTextColors).cast());
static sSwapSpeciesNameTextColors: Table<CArray<u8, 3>> =
    Table((&raw const crate::data::battle_factory_screen::sSwapSpeciesNameTextColors).cast());
static sSwapText_Pal: Table<CArray<u16, 5>> =
    Table((&raw const crate::data::battle_factory_screen::sSwapText_Pal).cast());
static sSwap_BallGfx: Table<CArray<CompressedSpriteSheet, 2>> =
    Table((&raw const crate::data::battle_factory_screen::sSwap_BallGfx).cast());
static sSwap_BgTemplates: Table<CArray<BgTemplate, 4>> =
    Table((&raw const crate::data::battle_factory_screen::sSwap_BgTemplates).cast());
static sSwap_EnemyScreenActions: Table<CArray<SwapScreenAction, 5>> =
    Table((&raw const crate::data::battle_factory_screen::sSwap_EnemyScreenActions).cast());
static sSwap_MenuOptionFuncs: Table<CArray<Option<unsafe fn(u8)>, 3>> =
    Table((&raw const crate::data::battle_factory_screen::sSwap_MenuOptionFuncs).cast());
static sSwap_PlayerScreenActions: Table<CArray<SwapScreenAction, 4>> =
    Table((&raw const crate::data::battle_factory_screen::sSwap_PlayerScreenActions).cast());
static sSwap_SpritePalettes: Table<CArray<SpritePalette, 5>> =
    Table((&raw const crate::data::battle_factory_screen::sSwap_SpritePalettes).cast());
static sSwap_SpriteSheets: Table<CArray<SpriteSheet, 10>> =
    Table((&raw const crate::data::battle_factory_screen::sSwap_SpriteSheets).cast());
static sSwap_WindowTemplates: Table<CArray<WindowTemplate, 10>> =
    Table((&raw const crate::data::battle_factory_screen::sSwap_WindowTemplates).cast());

#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sSelectMenuTilesetBuffer: *mut u8 = null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sSelectMonPicBgTilesetBuffer: *mut u8 = null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sSelectMenuTilemapBuffer: *mut u8 = null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sSelectMonPicBgTilemapBuffer: *mut u8 = null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sFactorySelectMons: *mut Pokemon = null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sSwapMenuTilesetBuffer: *mut u8 = null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sSwapMonPicBgTilesetBuffer: *mut u8 = null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sSwapMenuTilemapBuffer: *mut u8 = null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sSwapMonPicBgTilemapBuffer: *mut u8 = null_mut();
pub(crate) static mut sFactorySelectScreen: *mut FactorySelectScreen = null_mut();
pub(crate) static mut sSwap_CurrentOptionFunc: Option<unsafe fn(u8)> = None;
pub(crate) static mut sFactorySwapScreen: *mut FactorySwapScreen = null_mut();
#[unsafe(link_section = "common_data")]
pub static mut gFactorySelect_CurrentOptionFunc: Option<unsafe fn() -> u8> = None;

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
unsafe fn CpuSet(a0: *mut c_void, a1: *mut c_void, a2: u32) {
    unsafe {
        crate::syscall::CpuSet(a0 as _, a1 as _, a2);
    }
}
/// `SetMainCallback2` with this module's view of its types.
#[inline]
unsafe fn SetMainCallback2(a0: Option<unsafe fn()>) {
    unsafe {
        crate::agb_main::SetMainCallback2(core::mem::transmute(a0));
    }
}

pub(crate) unsafe fn SpriteCB_Pokeball(sprite: *mut Sprite) {
    if (*sprite).oam.paletteNum() == IndexOfSpritePaletteTag(PALTAG_BALL_SELECTED) as u16 {
        if (*sprite).animEnded() != 0 {
            if (*sprite).data[0] != 0 {
                (*sprite).data[0] -= 1;
            } else if Random() as i32 % 5 == 0 {
                StartSpriteAnim(sprite, 0);
                (*sprite).data[0] = 32;
            } else {
                StartSpriteAnim(sprite, 1);
            }
        } else {
            StartSpriteAnimIfDifferent(sprite, 1);
        }
    } else {
        StartSpriteAnimIfDifferent(sprite, 0);
    }
}
pub(crate) unsafe fn CB2_SelectScreen() {
    AnimateSprites();
    BuildOamBuffer();
    RunTextPrinters();
    UpdatePaletteFade();
    RunTasks();
}
pub(crate) unsafe fn VBlankCB_SelectScreen() {
    LoadOam();
    ProcessSpriteCopyRequests();
    TransferPlttBuffer();
}
pub unsafe fn DoBattleFactorySelectScreen() {
    sFactorySelectScreen = null_mut();
    SetMainCallback2(Some(CB2_InitSelectScreen));
}
pub(crate) unsafe fn CB2_InitSelectScreen() {
    let mut taskId: u8 = 0;
    match gMain.state {
        0 => {
            if !sFactorySelectMons.is_null() {
                Free(sFactorySelectMons as *mut c_void);
                sFactorySelectMons = null_mut();
            }
            SetHBlankCallback(None);
            SetVBlankCallback(None);
            {
                {
                    let mut tmp: u32 = 0;
                    volatile_write(&raw mut tmp, 0);
                    CpuSet(
                        &raw mut tmp as *mut c_void,
                        VRAM as usize as *mut c_void,
                        0x5006000,
                    );
                }
            }
            ResetBgsAndClearDma3BusyFlags(0);
            InitBgsFromTemplates(0, sSelect_BgTemplates.as_ptr().cast_mut(), 3);
            InitWindows(sSelect_WindowTemplates.as_ptr().cast_mut());
            DeactivateAllTextPrinters();
            gMain.state += 1;
        }
        1 => {
            sSelectMenuTilesetBuffer = Alloc(1088) as *mut u8;
            sSelectMonPicBgTilesetBuffer = AllocZeroed(1088) as *mut u8;
            sSelectMenuTilemapBuffer = Alloc(BG_SCREEN_SIZE) as *mut u8;
            sSelectMonPicBgTilemapBuffer = AllocZeroed(BG_SCREEN_SIZE) as *mut u8;
            ChangeBgX(0, 0, BG_COORD_SET);
            ChangeBgY(0, 0, BG_COORD_SET);
            ChangeBgX(1, 0, BG_COORD_SET);
            ChangeBgY(1, 0, BG_COORD_SET);
            ChangeBgX(3, 0, BG_COORD_SET);
            ChangeBgY(3, 0, BG_COORD_SET);
            SetGpuReg(REG_OFFSET_BLDCNT, 0);
            SetGpuReg(REG_OFFSET_BLDALPHA, 0);
            SetGpuReg(REG_OFFSET_BLDY, 0);
            SetGpuReg(REG_OFFSET_MOSAIC, 0);
            SetGpuReg(REG_OFFSET_WIN0H, 0);
            SetGpuReg(REG_OFFSET_WIN0V, 0);
            SetGpuReg(REG_OFFSET_WIN1H, 0);
            SetGpuReg(REG_OFFSET_WIN1V, 0);
            SetGpuReg(REG_OFFSET_WININ, 0);
            SetGpuReg(REG_OFFSET_WINOUT, 0);
            gMain.state += 1;
        }
        2 => {
            ResetPaletteFade();
            ResetSpriteData();
            ResetTasks();
            FreeAllSpritePalettes();
            CpuSet(
                (*(&raw const crate::data::graphics::gFrontierFactoryMenu_Gfx)
                    .cast::<CArray<u16, 544>>())
                .as_ptr()
                .cast_mut() as *mut c_void,
                sSelectMenuTilesetBuffer as *mut c_void,
                544,
            );
            CpuSet(
                sMonPicBg_Gfx.as_ptr().cast_mut() as *mut c_void,
                sSelectMonPicBgTilesetBuffer as *mut c_void,
                48,
            );
            LoadBgTiles(1, sSelectMenuTilesetBuffer as *mut c_void, 1088, 0);
            LoadBgTiles(3, sSelectMonPicBgTilesetBuffer as *mut c_void, 96, 0);
            CpuSet(
                (*(&raw const crate::data::graphics::gFrontierFactoryMenu_Tilemap)
                    .cast::<CArray<u16, 0>>())
                .as_ptr()
                .cast_mut() as *mut c_void,
                sSelectMenuTilemapBuffer as *mut c_void,
                1024,
            );
            LoadBgTilemap(
                1,
                sSelectMenuTilemapBuffer as *mut c_void,
                BG_SCREEN_SIZE as u16,
                0,
            );
            LoadPalette(
                (*(&raw const crate::data::graphics::gFrontierFactoryMenu_Pal)
                    .cast::<CArray<u16, 0>>())
                .as_ptr()
                .cast_mut() as *mut c_void,
                0,
                64,
            );
            LoadPalette(sSelectText_Pal.as_ptr().cast_mut() as *mut c_void, 240, 8);
            LoadPalette(sSelectText_Pal.as_ptr().cast_mut() as *mut c_void, 224, 10);
            if !sFactorySelectScreen.is_null() && (*sFactorySelectScreen).fromSummaryScreen != 0 {
                gPlttBufferUnfaded[228] = (*sFactorySelectScreen).speciesNameColorBackup;
            }
            LoadPalette(sMonPicBg_Pal.as_ptr().cast_mut() as *mut c_void, 32, 4);
            gMain.state += 1;
        }
        3 => {
            SetBgTilemapBuffer(3, sSelectMonPicBgTilemapBuffer as *mut c_void);
            CopyToBgTilemapBufferRect(
                3,
                sMonPicBg_Tilemap.as_ptr().cast_mut() as *mut c_void,
                11,
                4,
                8,
                8,
            );
            CopyToBgTilemapBufferRect(
                3,
                sMonPicBg_Tilemap.as_ptr().cast_mut() as *mut c_void,
                2,
                4,
                8,
                8,
            );
            CopyToBgTilemapBufferRect(
                3,
                sMonPicBg_Tilemap.as_ptr().cast_mut() as *mut c_void,
                20,
                4,
                8,
                8,
            );
            CopyBgTilemapBufferToVram(3);
            gMain.state += 1;
        }
        4 => {
            LoadSpritePalettes(sSelect_SpritePalettes.as_ptr().cast_mut());
            LoadSpriteSheets(sSelect_SpriteSheets.as_ptr().cast_mut());
            LoadCompressedSpriteSheet(sSelect_BallGfx.as_ptr().cast_mut());
            ShowBg(0);
            ShowBg(1);
            SetVBlankCallback(Some(VBlankCB_SelectScreen));
            BeginNormalPaletteFade(PALETTES_ALL, 0, 16, 0, 0);
            SetGpuReg(REG_OFFSET_DISPCNT, 4928);
            if !sFactorySelectScreen.is_null() && (*sFactorySelectScreen).fromSummaryScreen != 0 {
                Select_SetWinRegs(88, 152, 32, 96);
                ShowBg(3);
                SetGpuReg(REG_OFFSET_BLDCNT, 4680);
                SetGpuReg(REG_OFFSET_BLDALPHA, 1035);
            } else {
                HideBg(3);
            }
            gMain.state += 1;
        }
        5 => {
            if !sFactorySelectScreen.is_null() && (*sFactorySelectScreen).fromSummaryScreen != 0 {
                (*sFactorySelectScreen).cursorPos = gLastViewedMonIndex;
            }
            Select_InitMonsData();
            Select_InitAllSprites();
            if (*sFactorySelectScreen).fromSummaryScreen == TRUE {
                Select_ReshowMonSprite();
            }
            gMain.state += 1;
        }
        6 => {
            Select_PrintSelectMonString();
            PutWindowTilemap(SELECT_WIN_INFO);
            gMain.state += 1;
        }
        7 => {
            Select_PrintMonCategory();
            PutWindowTilemap(SELECT_WIN_MON_CATEGORY);
            gMain.state += 1;
        }
        8 => {
            Select_PrintMonSpecies();
            PutWindowTilemap(SELECT_WIN_SPECIES);
            gMain.state += 1;
        }
        9 => {
            Select_PrintRentalPkmnString();
            PutWindowTilemap(SELECT_WIN_TITLE);
            gMain.state += 1;
        }
        10 => {
            (*sFactorySelectScreen).fadeSpeciesNameTaskId =
                CreateTask(Some(Select_Task_FadeSpeciesName), 0);
            if (*sFactorySelectScreen).fromSummaryScreen == 0 {
                task_set(
                    (*sFactorySelectScreen).fadeSpeciesNameTaskId,
                    tState,
                    FADESTATE_INIT,
                );
                taskId = CreateTask(Some(Select_Task_HandleChooseMons), 0);
                task_set(taskId, tState, STATE_CHOOSE_MONS_INIT);
            } else {
                task_set(
                    (*sFactorySelectScreen).fadeSpeciesNameTaskId,
                    tState,
                    FADESTATE_RUN,
                );
                (*sFactorySelectScreen).fadeSpeciesNameActive = FALSE;
                taskId = CreateTask(Some(Select_Task_HandleMenu), 0);
                task_set(taskId, tState, STATE_MENU_RESHOW);
            }
            SetMainCallback2(Some(CB2_SelectScreen));
        }
        _ => {}
    }
}
unsafe fn Select_InitMonsData() {
    if !sFactorySelectScreen.is_null() {
        return;
    }
    sFactorySelectScreen = AllocZeroed(684) as *mut FactorySelectScreen;
    (*sFactorySelectScreen).cursorPos = 0;
    (*sFactorySelectScreen).selectingMonsState = 1;
    (*sFactorySelectScreen).fromSummaryScreen = FALSE;
    for i in 0..SELECTABLE_MONS_COUNT {
        (*sFactorySelectScreen).mons[i].selectedId = 0;
    }
    if (*gSaveBlock2Ptr).frontier.lvlMode() != FRONTIER_LVL_TENT {
        CreateFrontierFactorySelectableMons(0);
    } else {
        CreateSlateportTentSelectableMons(0);
    }
}
unsafe fn Select_InitAllSprites() {
    let mut cursorPos: u8 = 0;
    let mut x: i16 = 0;
    for i in 0..SELECTABLE_MONS_COUNT {
        (*sFactorySelectScreen).mons[i].ballSpriteId = CreateSprite(
            (&raw const *sSpriteTemplate_Select_Pokeball).cast_mut(),
            35 * i as i16 + 32,
            64,
            1,
        ) as u16;
        gSprites[(*sFactorySelectScreen).mons[i].ballSpriteId].data[0] = 0;
        Select_SetBallSpritePaletteNum(i);
    }
    cursorPos = (*sFactorySelectScreen).cursorPos;
    x = gSprites[(*sFactorySelectScreen).mons[cursorPos].ballSpriteId].x;
    (*sFactorySelectScreen).cursorSpriteId = CreateSprite(
        (&raw const *sSpriteTemplate_Select_Arrow).cast_mut(),
        x,
        88,
        0,
    );
    (*sFactorySelectScreen).menuCursor1SpriteId = CreateSprite(
        (&raw const *sSpriteTemplate_Select_MenuHighlightLeft).cast_mut(),
        176,
        112,
        0,
    );
    (*sFactorySelectScreen).menuCursor2SpriteId = CreateSprite(
        (&raw const *sSpriteTemplate_Select_MenuHighlightRight).cast_mut(),
        176,
        144,
        0,
    );
    gSprites[(*sFactorySelectScreen).menuCursor1SpriteId].set_invisible(TRUE as u16);
    gSprites[(*sFactorySelectScreen).menuCursor2SpriteId].set_invisible(TRUE as u16);
    gSprites[(*sFactorySelectScreen).menuCursor1SpriteId].centerToCornerVecX = 0;
    gSprites[(*sFactorySelectScreen).menuCursor1SpriteId].centerToCornerVecY = 0;
    gSprites[(*sFactorySelectScreen).menuCursor2SpriteId].centerToCornerVecX = 0;
    gSprites[(*sFactorySelectScreen).menuCursor2SpriteId].centerToCornerVecY = 0;
}
unsafe fn Select_DestroyAllSprites() {
    for i in 0..SELECTABLE_MONS_COUNT {
        DestroySprite(&raw mut gSprites[(*sFactorySelectScreen).mons[i].ballSpriteId]);
    }
    DestroySprite(&raw mut gSprites[(*sFactorySelectScreen).cursorSpriteId]);
    DestroySprite(&raw mut gSprites[(*sFactorySelectScreen).menuCursor1SpriteId]);
    DestroySprite(&raw mut gSprites[(*sFactorySelectScreen).menuCursor2SpriteId]);
}
unsafe fn Select_UpdateBallCursorPosition(direction: i8) {
    let mut cursorPos: u8 = 0;
    if direction > 0 {
        if (*sFactorySelectScreen).cursorPos != 5 {
            (*sFactorySelectScreen).cursorPos += 1;
        } else {
            (*sFactorySelectScreen).cursorPos = 0;
        }
    } else {
        if (*sFactorySelectScreen).cursorPos != 0 {
            (*sFactorySelectScreen).cursorPos -= 1;
        } else {
            (*sFactorySelectScreen).cursorPos = 5;
        }
    }
    cursorPos = (*sFactorySelectScreen).cursorPos;
    gSprites[(*sFactorySelectScreen).cursorSpriteId].x =
        gSprites[(*sFactorySelectScreen).mons[cursorPos].ballSpriteId].x;
}
unsafe fn Select_UpdateMenuCursorPosition(direction: i8) {
    if direction > 0 {
        if (*sFactorySelectScreen).menuCursorPos != 2 {
            (*sFactorySelectScreen).menuCursorPos += 1;
        } else {
            (*sFactorySelectScreen).menuCursorPos = 0;
        }
    } else {
        if (*sFactorySelectScreen).menuCursorPos != 0 {
            (*sFactorySelectScreen).menuCursorPos -= 1;
        } else {
            (*sFactorySelectScreen).menuCursorPos = 2;
        }
    }
    gSprites[(*sFactorySelectScreen).menuCursor1SpriteId].y =
        (*sFactorySelectScreen).menuCursorPos as i16 * 16 + 112;
    gSprites[(*sFactorySelectScreen).menuCursor2SpriteId].y =
        (*sFactorySelectScreen).menuCursorPos as i16 * 16 + 112;
}
unsafe fn Select_UpdateYesNoCursorPosition(direction: i8) {
    if direction > 0 {
        if (*sFactorySelectScreen).yesNoCursorPos != 1 {
            (*sFactorySelectScreen).yesNoCursorPos += 1;
        } else {
            (*sFactorySelectScreen).yesNoCursorPos = 0;
        }
    } else {
        if (*sFactorySelectScreen).yesNoCursorPos != 0 {
            (*sFactorySelectScreen).yesNoCursorPos -= 1;
        } else {
            (*sFactorySelectScreen).yesNoCursorPos = 1;
        }
    }
    gSprites[(*sFactorySelectScreen).menuCursor1SpriteId].y =
        (*sFactorySelectScreen).yesNoCursorPos as i16 * 16 + 112;
    gSprites[(*sFactorySelectScreen).menuCursor2SpriteId].y =
        (*sFactorySelectScreen).yesNoCursorPos as i16 * 16 + 112;
}
unsafe fn Select_HandleMonSelectionChange() {
    let mut i: u8 = 0;
    let mut paletteNum: u8 = 0;
    let cursorPos: u8 = (*sFactorySelectScreen).cursorPos;
    if (*sFactorySelectScreen).mons[cursorPos].selectedId != 0 {
        paletteNum = IndexOfSpritePaletteTag(PALTAG_BALL_GRAY);
        if (*sFactorySelectScreen).selectingMonsState == FRONTIER_PARTY_SIZE as u8
            && (*sFactorySelectScreen).mons[cursorPos].selectedId == 1
        {
            i = 0;
            while i < SELECTABLE_MONS_COUNT {
                if (*sFactorySelectScreen).mons[i].selectedId == 2 {
                    break;
                }
                i += 1;
            }
            if i == SELECTABLE_MONS_COUNT {
                return;
            } else {
                (*sFactorySelectScreen).mons[i].selectedId = 1;
            }
        }
        (*sFactorySelectScreen).mons[cursorPos].selectedId = 0;
        (*sFactorySelectScreen).selectingMonsState -= 1;
    } else {
        paletteNum = IndexOfSpritePaletteTag(PALTAG_BALL_SELECTED);
        (*sFactorySelectScreen).mons[cursorPos].selectedId =
            (*sFactorySelectScreen).selectingMonsState;
        (*sFactorySelectScreen).selectingMonsState += 1;
    }
    gSprites[(*sFactorySelectScreen).mons[cursorPos].ballSpriteId]
        .oam
        .set_paletteNum(paletteNum as u16);
}
unsafe fn Select_SetBallSpritePaletteNum(id: u8) {
    let mut palNum: u8 = 0;
    if (*sFactorySelectScreen).mons[id].selectedId != 0 {
        palNum = IndexOfSpritePaletteTag(PALTAG_BALL_SELECTED);
    } else {
        palNum = IndexOfSpritePaletteTag(PALTAG_BALL_GRAY);
    }
    gSprites[(*sFactorySelectScreen).mons[id].ballSpriteId]
        .oam
        .set_paletteNum(palNum as u16);
}
pub(crate) unsafe fn Select_Task_OpenSummaryScreen(taskId: u8) {
    let mut currMonId: u8 = 0;
    match task_get(taskId, tState) {
        STATE_SUMMARY_FADE => {
            gPlttBufferUnfaded[228] = gPlttBufferFaded[228];
            BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 16, 0);
            task_set(taskId, tState, STATE_SUMMARY_CLEAN);
        }
        STATE_SUMMARY_CLEAN => {
            if gPaletteFade.active() == 0 {
                DestroyTask((*sFactorySelectScreen).fadeSpeciesNameTaskId);
                HideMonPic(
                    (*sFactorySelectScreen).monPics[1],
                    &raw mut (*sFactorySelectScreen).monPicAnimating,
                );
                Select_DestroyAllSprites();
                Free(sSelectMenuTilesetBuffer as *mut c_void);
                sSelectMenuTilesetBuffer = null_mut();
                Free(sSelectMonPicBgTilesetBuffer as *mut c_void);
                sSelectMonPicBgTilesetBuffer = null_mut();
                Free(sSelectMenuTilemapBuffer as *mut c_void);
                sSelectMenuTilemapBuffer = null_mut();
                Free(sSelectMonPicBgTilemapBuffer as *mut c_void);
                sSelectMonPicBgTilemapBuffer = null_mut();
                FreeAllWindowBuffers();
                task_set(taskId, tState, STATE_SUMMARY_SHOW);
            }
        }
        STATE_SUMMARY_SHOW => {
            (*sFactorySelectScreen).speciesNameColorBackup = gPlttBufferUnfaded[228];
            DestroyTask(taskId);
            (*sFactorySelectScreen).fromSummaryScreen = TRUE;
            currMonId = (*sFactorySelectScreen).cursorPos;
            sFactorySelectMons = AllocZeroed(600) as *mut Pokemon;
            for i in 0..SELECTABLE_MONS_COUNT {
                *sFactorySelectMons.at(i) = (*sFactorySelectScreen).mons[i].monData;
            }
            ShowPokemonSummaryScreen(
                SUMMARY_MODE_LOCK_MOVES,
                sFactorySelectMons as *mut c_void,
                currMonId,
                5,
                Some(CB2_InitSelectScreen),
            );
        }
        _ => {}
    }
}
pub(crate) unsafe fn Select_Task_Exit(taskId: u8) {
    if (*sFactorySelectScreen).monPicAnimating == TRUE {
        return;
    }
    match task_get(taskId, tState) {
        0 => {
            BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 16, 0);
            task_set(taskId, tState, task_get(taskId, tState) + 1);
        }
        1 if UpdatePaletteFade() == 0 => {
            Select_CopyMonsToPlayerParty();
            DestroyTask((*sFactorySelectScreen).fadeSpeciesNameTaskId);
            Select_DestroyAllSprites();
            Free(sSelectMenuTilesetBuffer as *mut c_void);
            sSelectMenuTilesetBuffer = null_mut();
            Free(sSelectMenuTilemapBuffer as *mut c_void);
            sSelectMenuTilemapBuffer = null_mut();
            Free(sSelectMonPicBgTilemapBuffer as *mut c_void);
            sSelectMonPicBgTilemapBuffer = null_mut();
            Free(sFactorySelectScreen as *mut c_void);
            sFactorySelectScreen = null_mut();
            FreeAllWindowBuffers();
            SetMainCallback2(Some(CB2_ReturnToFieldContinueScript));
            DestroyTask(taskId);
        }
        _ => {}
    }
}
pub(crate) unsafe fn Select_Task_HandleYesNo(taskId: u8) {
    if (*sFactorySelectScreen).monPicAnimating == TRUE {
        return;
    }
    match task_get(taskId, tState) {
        STATE_YESNO_SHOW_MONS => {
            Select_ShowChosenMons();
            task_set(taskId, tState, STATE_YESNO_SHOW_OPTIONS);
        }
        STATE_YESNO_SHOW_OPTIONS => {
            Select_ShowYesNoOptions();
            task_set(taskId, tState, STATE_YESNO_HANDLE_INPUT);
        }
        STATE_YESNO_HANDLE_INPUT => {
            if gMain.newKeys as i32 & A_BUTTON != 0 {
                PlaySE(SE_SELECT);
                if (*sFactorySelectScreen).yesNoCursorPos == 0 {
                    Select_HideChosenMons();
                    task_set(taskId, tState, 0);
                    task_set_func(taskId, Some(Select_Task_Exit));
                } else {
                    Select_ErasePopupMenu(SELECT_WIN_YES_NO);
                    Select_DeclineChosenMons();
                    (*sFactorySelectScreen).fadeSpeciesNameActive = TRUE;
                    task_set(taskId, tState, STATE_CHOOSE_MONS_HANDLE_INPUT);
                    task_set_func(taskId, Some(Select_Task_HandleChooseMons));
                }
            } else if gMain.newKeys as i32 & B_BUTTON != 0 {
                PlaySE(SE_SELECT);
                Select_ErasePopupMenu(SELECT_WIN_YES_NO);
                Select_DeclineChosenMons();
                (*sFactorySelectScreen).fadeSpeciesNameActive = TRUE;
                task_set(taskId, tState, STATE_CHOOSE_MONS_HANDLE_INPUT);
                task_set_func(taskId, Some(Select_Task_HandleChooseMons));
            } else if gMain.newAndRepeatedKeys as i32 & DPAD_UP != 0 {
                PlaySE(SE_SELECT);
                Select_UpdateYesNoCursorPosition(-1);
            } else if gMain.newAndRepeatedKeys as i32 & DPAD_DOWN != 0 {
                PlaySE(SE_SELECT);
                Select_UpdateYesNoCursorPosition(1);
            }
        }
        _ => {}
    }
}
pub(crate) unsafe fn Select_Task_HandleMenu(taskId: u8) {
    match task_get(taskId, tState) {
        STATE_MENU_INIT => {
            if (*sFactorySelectScreen).fromSummaryScreen == 0 {
                OpenMonPic(
                    &raw mut (*sFactorySelectScreen).monPics[1].bgSpriteId,
                    &raw mut (*sFactorySelectScreen).monPicAnimating,
                    FALSE,
                );
            }
            task_set(taskId, tState, STATE_MENU_SHOW_OPTIONS);
        }
        STATE_MENU_SHOW_OPTIONS => {
            if (*sFactorySelectScreen).monPicAnimating != TRUE {
                Select_ShowMenuOptions();
                (*sFactorySelectScreen).fromSummaryScreen = FALSE;
                task_set(taskId, tState, STATE_MENU_HANDLE_INPUT);
            }
        }
        STATE_MENU_HANDLE_INPUT => {
            if gMain.newKeys as i32 & A_BUTTON != 0 {
                PlaySE(SE_SELECT);
                let retVal: u8 = Select_RunMenuOptionFunc();
                if retVal == SELECT_CONTINUE_CHOOSING {
                    (*sFactorySelectScreen).fadeSpeciesNameActive = TRUE;
                    task_set(taskId, tState, STATE_CHOOSE_MONS_HANDLE_INPUT);
                    task_set_func(taskId, Some(Select_Task_HandleChooseMons));
                } else if retVal == SELECT_CONFIRM_MONS {
                    task_set(taskId, tState, STATE_YESNO_SHOW_MONS);
                    task_set_func(taskId, Some(Select_Task_HandleYesNo));
                } else if retVal == SELECT_INVALID_MON {
                    task_set(taskId, tState, STATE_CHOOSE_MONS_INVALID);
                    task_set_func(taskId, Some(Select_Task_HandleChooseMons));
                } else {
                    task_set(taskId, tState, STATE_SUMMARY_FADE);
                    task_set_func(taskId, Some(Select_Task_OpenSummaryScreen));
                }
            } else if gMain.newKeys as i32 & B_BUTTON != 0 {
                PlaySE(SE_SELECT);
                CloseMonPic(
                    (*sFactorySelectScreen).monPics[1],
                    &raw mut (*sFactorySelectScreen).monPicAnimating,
                    FALSE,
                );
                Select_ErasePopupMenu(SELECT_WIN_OPTIONS);
                (*sFactorySelectScreen).fadeSpeciesNameActive = TRUE;
                task_set(taskId, tState, STATE_CHOOSE_MONS_HANDLE_INPUT);
                task_set_func(taskId, Some(Select_Task_HandleChooseMons));
            } else if gMain.newAndRepeatedKeys as i32 & DPAD_UP != 0 {
                PlaySE(SE_SELECT);
                Select_UpdateMenuCursorPosition(-1);
            } else if gMain.newAndRepeatedKeys as i32 & DPAD_DOWN != 0 {
                PlaySE(SE_SELECT);
                Select_UpdateMenuCursorPosition(1);
            }
        }
        STATE_MENU_REINIT => {
            if gPaletteFade.active() == 0 {
                if (*sFactorySelectScreen).fromSummaryScreen == TRUE {
                    gPlttBufferFaded[228] = (*sFactorySelectScreen).speciesNameColorBackup;
                    gPlttBufferUnfaded[228] = gPlttBufferUnfaded[244];
                }
                (*sFactorySelectScreen).fromSummaryScreen = FALSE;
                task_set(taskId, tState, STATE_MENU_HANDLE_INPUT);
            }
        }
        STATE_MENU_RESHOW => {
            Select_ShowMenuOptions();
            task_set(taskId, tState, STATE_MENU_REINIT);
        }
        _ => {}
    }
}
pub(crate) unsafe fn Select_Task_HandleChooseMons(taskId: u8) {
    if (*sFactorySelectScreen).monPicAnimating == TRUE {
        return;
    }
    match task_get(taskId, tState) {
        STATE_CHOOSE_MONS_INIT => {
            if gPaletteFade.active() == 0 {
                task_set(taskId, tState, STATE_CHOOSE_MONS_HANDLE_INPUT);
                (*sFactorySelectScreen).fadeSpeciesNameActive = TRUE;
            }
        }
        STATE_CHOOSE_MONS_HANDLE_INPUT => {
            if gMain.newKeys as i32 & A_BUTTON != 0 {
                PlaySE(SE_SELECT);
                (*sFactorySelectScreen).fadeSpeciesNameActive = FALSE;
                task_set(taskId, tState, STATE_MENU_INIT);
                task_set_func(taskId, Some(Select_Task_HandleMenu));
            } else if gMain.newAndRepeatedKeys as i32 & DPAD_LEFT != 0 {
                PlaySE(SE_SELECT);
                Select_UpdateBallCursorPosition(-1);
                Select_PrintMonCategory();
                Select_PrintMonSpecies();
            } else if gMain.newAndRepeatedKeys as i32 & DPAD_RIGHT != 0 {
                PlaySE(SE_SELECT);
                Select_UpdateBallCursorPosition(1);
                Select_PrintMonCategory();
                Select_PrintMonSpecies();
            }
        }
        STATE_CHOOSE_MONS_INVALID if gMain.newKeys as i32 & A_BUTTON != 0 => {
            PlaySE(SE_SELECT);
            CloseMonPic(
                (*sFactorySelectScreen).monPics[1],
                &raw mut (*sFactorySelectScreen).monPicAnimating,
                FALSE,
            );
            Select_PrintSelectMonString();
            (*sFactorySelectScreen).fadeSpeciesNameActive = TRUE;
            task_set(taskId, tState, STATE_CHOOSE_MONS_HANDLE_INPUT);
        }
        _ => {}
    }
}
unsafe fn CreateFrontierFactorySelectableMons(firstMonId: u8) {
    let mut ivs: u8 = 0;
    let mut level: u8 = 0;
    let mut friendship: u8 = 0;
    let battleMode: u8 = VarGet(VAR_FRONTIER_BATTLE_MODE) as u8;
    let lvlMode: u8 = (*gSaveBlock2Ptr).frontier.lvlMode();
    let challengeNum: u8 =
        ((*gSaveBlock2Ptr).frontier.factoryWinStreaks[battleMode][lvlMode] as i32 / 7) as u8;
    gFacilityTrainerMons = (*(&raw const crate::data::battle_tower::gBattleFrontierMons)
        .cast::<CArray<FacilityMon, 0>>())
    .as_ptr()
    .cast_mut();
    if (*gSaveBlock2Ptr).frontier.lvlMode() != FRONTIER_LVL_50 {
        level = FRONTIER_MAX_LEVEL_OPEN;
    } else {
        level = FRONTIER_MAX_LEVEL_50;
    }
    let rentalRank: u8 = GetNumPastRentalsRank(battleMode, lvlMode);
    let otId: u32 = (*gSaveBlock2Ptr).playerTrainerId[0] as u32
        | ((*gSaveBlock2Ptr).playerTrainerId[1] as u32) << 8
        | ((*gSaveBlock2Ptr).playerTrainerId[2] as u32) << 16
        | ((*gSaveBlock2Ptr).playerTrainerId[3] as u32) << 24;
    for i in 0..SELECTABLE_MONS_COUNT {
        let monId: u16 = (*gSaveBlock2Ptr).frontier.rentalMons[i].monId;
        (*sFactorySelectScreen).mons[i as i32 + firstMonId as i32].monId = monId;
        if i < rentalRank {
            ivs = GetFactoryMonFixedIV(challengeNum + 1, FALSE);
        } else {
            ivs = GetFactoryMonFixedIV(challengeNum, FALSE);
        }
        CreateMonWithEVSpreadNatureOTID(
            &raw mut (*sFactorySelectScreen).mons[i as i32 + firstMonId as i32].monData,
            (*gFacilityTrainerMons.at(monId)).species,
            level,
            (*gFacilityTrainerMons.at(monId)).nature,
            ivs,
            (*gFacilityTrainerMons.at(monId)).evSpread,
            otId,
        );
        friendship = 0;
        for j in 0..(MAX_MON_MOVES as u8) {
            SetMonMoveAvoidReturn(
                &raw mut (*sFactorySelectScreen).mons[i as i32 + firstMonId as i32].monData,
                (*gFacilityTrainerMons.at(monId)).moves[j],
                j,
            );
        }
        SetMonData(
            &raw mut (*sFactorySelectScreen).mons[i as i32 + firstMonId as i32].monData,
            MON_DATA_FRIENDSHIP,
            &raw mut friendship as *mut c_void,
        );
        SetMonData(
            &raw mut (*sFactorySelectScreen).mons[i as i32 + firstMonId as i32].monData,
            MON_DATA_HELD_ITEM,
            (&raw const (*(&raw const crate::data::battle_tower::gBattleFrontierHeldItems)
                .cast::<CArray<u16, 0>>())[(*gFacilityTrainerMons.at(monId)).itemTableId])
                .cast_mut() as *mut c_void,
        );
    }
}
unsafe fn CreateSlateportTentSelectableMons(firstMonId: u8) {
    let ivs: u8 = 0;
    let level: u8 = TENT_MIN_LEVEL;
    let mut friendship: u8 = 0;
    gFacilityTrainerMons = (*(&raw const crate::data::battle_tower::gSlateportBattleTentMons)
        .cast::<CArray<FacilityMon, 0>>())
    .as_ptr()
    .cast_mut();
    let otId: u32 = (*gSaveBlock2Ptr).playerTrainerId[0] as u32
        | ((*gSaveBlock2Ptr).playerTrainerId[1] as u32) << 8
        | ((*gSaveBlock2Ptr).playerTrainerId[2] as u32) << 16
        | ((*gSaveBlock2Ptr).playerTrainerId[3] as u32) << 24;
    for i in 0..SELECTABLE_MONS_COUNT {
        let monId: u16 = (*gSaveBlock2Ptr).frontier.rentalMons[i].monId;
        (*sFactorySelectScreen).mons[i as i32 + firstMonId as i32].monId = monId;
        CreateMonWithEVSpreadNatureOTID(
            &raw mut (*sFactorySelectScreen).mons[i as i32 + firstMonId as i32].monData,
            (*gFacilityTrainerMons.at(monId)).species,
            level,
            (*gFacilityTrainerMons.at(monId)).nature,
            ivs,
            (*gFacilityTrainerMons.at(monId)).evSpread,
            otId,
        );
        friendship = 0;
        for j in 0..(MAX_MON_MOVES as u8) {
            SetMonMoveAvoidReturn(
                &raw mut (*sFactorySelectScreen).mons[i as i32 + firstMonId as i32].monData,
                (*gFacilityTrainerMons.at(monId)).moves[j],
                j,
            );
        }
        SetMonData(
            &raw mut (*sFactorySelectScreen).mons[i as i32 + firstMonId as i32].monData,
            MON_DATA_FRIENDSHIP,
            &raw mut friendship as *mut c_void,
        );
        SetMonData(
            &raw mut (*sFactorySelectScreen).mons[i as i32 + firstMonId as i32].monData,
            MON_DATA_HELD_ITEM,
            (&raw const (*(&raw const crate::data::battle_tower::gBattleFrontierHeldItems)
                .cast::<CArray<u16, 0>>())[(*gFacilityTrainerMons.at(monId)).itemTableId])
                .cast_mut() as *mut c_void,
        );
    }
}
unsafe fn Select_CopyMonsToPlayerParty() {
    for i in 0..(FRONTIER_PARTY_SIZE as u8) {
        for j in 0..SELECTABLE_MONS_COUNT {
            if (*sFactorySelectScreen).mons[j].selectedId as i32 == i as i32 + 1 {
                gPlayerParty[i] = (*sFactorySelectScreen).mons[j].monData;
                (*gSaveBlock2Ptr).frontier.rentalMons[i].monId =
                    (*sFactorySelectScreen).mons[j].monId;
                (*gSaveBlock2Ptr).frontier.rentalMons[i].personality =
                    GetMonData3(&raw mut gPlayerParty[i], MON_DATA_PERSONALITY, null_mut());
                (*gSaveBlock2Ptr).frontier.rentalMons[i].abilityNum = GetBoxMonData3(
                    &raw mut gPlayerParty[i].r#box,
                    MON_DATA_ABILITY_NUM,
                    null_mut(),
                ) as u8;
                (*gSaveBlock2Ptr).frontier.rentalMons[i].ivs =
                    GetBoxMonData3(&raw mut gPlayerParty[i].r#box, MON_DATA_ATK_IV, null_mut())
                        as u8;
                break;
            }
        }
    }
    CalculatePlayerPartyCount();
}
unsafe fn Select_ShowMenuOptions() {
    if (*sFactorySelectScreen).fromSummaryScreen == 0 {
        (*sFactorySelectScreen).menuCursorPos = 0;
    }
    gSprites[(*sFactorySelectScreen).menuCursor1SpriteId].x = 176;
    gSprites[(*sFactorySelectScreen).menuCursor1SpriteId].y =
        (*sFactorySelectScreen).menuCursorPos as i16 * 16 + 112;
    gSprites[(*sFactorySelectScreen).menuCursor2SpriteId].x = 208;
    gSprites[(*sFactorySelectScreen).menuCursor2SpriteId].y =
        (*sFactorySelectScreen).menuCursorPos as i16 * 16 + 112;
    gSprites[(*sFactorySelectScreen).menuCursor1SpriteId].set_invisible(FALSE as u16);
    gSprites[(*sFactorySelectScreen).menuCursor2SpriteId].set_invisible(FALSE as u16);
    Select_PrintMenuOptions();
}
unsafe fn Select_ShowYesNoOptions() {
    (*sFactorySelectScreen).yesNoCursorPos = 0;
    gSprites[(*sFactorySelectScreen).menuCursor1SpriteId].x = 176;
    gSprites[(*sFactorySelectScreen).menuCursor1SpriteId].y = 112;
    gSprites[(*sFactorySelectScreen).menuCursor2SpriteId].x = 208;
    gSprites[(*sFactorySelectScreen).menuCursor2SpriteId].y = 112;
    gSprites[(*sFactorySelectScreen).menuCursor1SpriteId].set_invisible(FALSE as u16);
    gSprites[(*sFactorySelectScreen).menuCursor2SpriteId].set_invisible(FALSE as u16);
    Select_PrintYesNoOptions();
}
unsafe fn Select_ErasePopupMenu(windowId: u8) {
    gSprites[(*sFactorySelectScreen).menuCursor1SpriteId].set_invisible(TRUE as u16);
    gSprites[(*sFactorySelectScreen).menuCursor2SpriteId].set_invisible(TRUE as u16);
    FillWindowPixelBuffer(windowId, 0);
    CopyWindowToVram(windowId, COPYWIN_GFX);
    ClearWindowTilemap(windowId);
}
unsafe fn Select_PrintRentalPkmnString() {
    FillWindowPixelBuffer(SELECT_WIN_TITLE, 0);
    AddTextPrinterParameterized(
        SELECT_WIN_TITLE,
        FONT_NORMAL,
        (*(&raw const crate::data::strings::gText_RentalPkmn2).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        2,
        1,
        0,
        None,
    );
    CopyWindowToVram(SELECT_WIN_TITLE, COPYWIN_FULL);
}
unsafe fn Select_PrintMonSpecies() {
    let monId: u8 = (*sFactorySelectScreen).cursorPos;
    FillWindowPixelBuffer(SELECT_WIN_SPECIES, 0);
    let species: u16 = GetMonData3(
        &raw mut (*sFactorySelectScreen).mons[monId].monData,
        MON_DATA_SPECIES,
        null_mut(),
    ) as u16;
    StringCopy(
        gStringVar4.as_mut_ptr(),
        (*(&raw const crate::data::data_tables::gSpeciesNames).cast::<CArray<CArray<u8, 11>, 0>>())
            [species]
            .as_ptr()
            .cast_mut(),
    );
    let x: u8 = GetStringRightAlignXOffset(FONT_NORMAL as i32, gStringVar4.as_mut_ptr(), 86) as u8;
    AddTextPrinterParameterized3(
        SELECT_WIN_SPECIES,
        FONT_NORMAL,
        x,
        1,
        sSpeciesNameTextColors.as_ptr().cast_mut(),
        0,
        gStringVar4.as_mut_ptr(),
    );
    CopyWindowToVram(SELECT_WIN_SPECIES, COPYWIN_GFX);
}
unsafe fn Select_PrintSelectMonString() {
    let mut str: *mut u8 = null_mut();
    FillWindowPixelBuffer(SELECT_WIN_INFO, 0);
    if (*sFactorySelectScreen).selectingMonsState == 1 {
        str = (*(&raw const crate::data::strings::gText_SelectFirstPkmn).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut();
    } else if (*sFactorySelectScreen).selectingMonsState == 2 {
        str = (*(&raw const crate::data::strings::gText_SelectSecondPkmn).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut();
    } else if (*sFactorySelectScreen).selectingMonsState == 3 {
        str = (*(&raw const crate::data::strings::gText_SelectThirdPkmn).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut();
    } else {
        str = (*(&raw const crate::data::strings::gText_TheseThreePkmnOkay)
            .cast::<CArray<u8, 0>>())
        .as_ptr()
        .cast_mut();
    }
    AddTextPrinterParameterized(SELECT_WIN_INFO, FONT_NORMAL, str, 2, 5, 0, None);
    CopyWindowToVram(SELECT_WIN_INFO, COPYWIN_GFX);
}
unsafe fn Select_PrintCantSelectSameMon() {
    FillWindowPixelBuffer(SELECT_WIN_INFO, 0);
    AddTextPrinterParameterized(
        SELECT_WIN_INFO,
        FONT_NORMAL,
        (*(&raw const crate::data::strings::gText_CantSelectSamePkmn).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        2,
        5,
        0,
        None,
    );
    CopyWindowToVram(SELECT_WIN_INFO, COPYWIN_GFX);
}
unsafe fn Select_PrintMenuOptions() {
    let selectedId: u8 = (*sFactorySelectScreen).mons[(*sFactorySelectScreen).cursorPos].selectedId;
    PutWindowTilemap(SELECT_WIN_OPTIONS);
    FillWindowPixelBuffer(SELECT_WIN_OPTIONS, 0);
    AddTextPrinterParameterized3(
        SELECT_WIN_OPTIONS,
        FONT_NORMAL,
        7,
        1,
        sMenuOptionTextColors.as_ptr().cast_mut(),
        0,
        (*(&raw const crate::data::strings::gText_Summary).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
    );
    if selectedId != 0 {
        AddTextPrinterParameterized3(
            SELECT_WIN_OPTIONS,
            FONT_NORMAL,
            7,
            17,
            sMenuOptionTextColors.as_ptr().cast_mut(),
            0,
            (*(&raw const crate::data::strings::gText_Deselect).cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
        );
    } else {
        AddTextPrinterParameterized3(
            SELECT_WIN_OPTIONS,
            FONT_NORMAL,
            7,
            17,
            sMenuOptionTextColors.as_ptr().cast_mut(),
            0,
            (*(&raw const crate::data::strings::gText_Rent).cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
        );
    }
    AddTextPrinterParameterized3(
        SELECT_WIN_OPTIONS,
        FONT_NORMAL,
        7,
        33,
        sMenuOptionTextColors.as_ptr().cast_mut(),
        0,
        (*(&raw const crate::data::strings::gText_Others2).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
    );
    CopyWindowToVram(SELECT_WIN_OPTIONS, COPYWIN_FULL);
}
unsafe fn Select_PrintYesNoOptions() {
    PutWindowTilemap(SELECT_WIN_YES_NO);
    FillWindowPixelBuffer(SELECT_WIN_YES_NO, 0);
    AddTextPrinterParameterized3(
        SELECT_WIN_YES_NO,
        FONT_NORMAL,
        7,
        1,
        sMenuOptionTextColors.as_ptr().cast_mut(),
        0,
        (*(&raw const crate::data::strings::gText_Yes2).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
    );
    AddTextPrinterParameterized3(
        SELECT_WIN_YES_NO,
        FONT_NORMAL,
        7,
        17,
        sMenuOptionTextColors.as_ptr().cast_mut(),
        0,
        (*(&raw const crate::data::strings::gText_No2).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
    );
    CopyWindowToVram(SELECT_WIN_YES_NO, COPYWIN_FULL);
}
unsafe fn Select_RunMenuOptionFunc() -> u8 {
    gFactorySelect_CurrentOptionFunc =
        sSelect_MenuOptionFuncs[(*sFactorySelectScreen).menuCursorPos];
    gFactorySelect_CurrentOptionFunc.unwrap_unchecked()()
}
pub(crate) unsafe fn Select_OptionRentDeselect() -> u8 {
    let selectedId: u8 = (*sFactorySelectScreen).mons[(*sFactorySelectScreen).cursorPos].selectedId;
    let monId: u16 = (*sFactorySelectScreen).mons[(*sFactorySelectScreen).cursorPos].monId;
    if selectedId == 0 && Select_AreSpeciesValid(monId) == 0 {
        Select_PrintCantSelectSameMon();
        Select_ErasePopupMenu(SELECT_WIN_OPTIONS);
        return SELECT_INVALID_MON;
    } else {
        CloseMonPic(
            (*sFactorySelectScreen).monPics[1],
            &raw mut (*sFactorySelectScreen).monPicAnimating,
            FALSE,
        );
        Select_HandleMonSelectionChange();
        Select_PrintSelectMonString();
        Select_ErasePopupMenu(SELECT_WIN_OPTIONS);
        if (*sFactorySelectScreen).selectingMonsState > FRONTIER_PARTY_SIZE as u8 {
            return SELECT_CONFIRM_MONS;
        } else {
            return SELECT_CONTINUE_CHOOSING;
        }
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
unsafe fn Select_DeclineChosenMons() -> u8 {
    Select_HideChosenMons();
    Select_HandleMonSelectionChange();
    Select_PrintSelectMonString();
    Select_ErasePopupMenu(SELECT_WIN_OPTIONS);
    if (*sFactorySelectScreen).selectingMonsState > FRONTIER_PARTY_SIZE as u8 {
        return 2;
    } else {
        return 1;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
pub(crate) fn Select_OptionSummary() -> u8 {
    SELECT_SUMMARY
}
pub(crate) unsafe fn Select_OptionOthers() -> u8 {
    CloseMonPic(
        (*sFactorySelectScreen).monPics[1],
        &raw mut (*sFactorySelectScreen).monPicAnimating,
        FALSE,
    );
    Select_ErasePopupMenu(SELECT_WIN_OPTIONS);
    SELECT_CONTINUE_CHOOSING
}
unsafe fn Select_PrintMonCategory() {
    let mut species: u16 = 0;
    let mut text: CArray<u8, 30> = zeroed();
    let mut x: u8 = 0;
    let monId: u8 = (*sFactorySelectScreen).cursorPos;
    if monId < SELECTABLE_MONS_COUNT {
        PutWindowTilemap(SELECT_WIN_MON_CATEGORY);
        FillWindowPixelBuffer(SELECT_WIN_MON_CATEGORY, 0);
        species = GetMonData3(
            &raw mut (*sFactorySelectScreen).mons[monId].monData,
            MON_DATA_SPECIES,
            null_mut(),
        ) as u16;
        CopyMonCategoryText(
            SpeciesToNationalPokedexNum(species) as i32,
            text.as_mut_ptr(),
        );
        x = GetStringRightAlignXOffset(FONT_NORMAL as i32, text.as_mut_ptr(), 118) as u8;
        AddTextPrinterParameterized(
            SELECT_WIN_MON_CATEGORY,
            FONT_NORMAL,
            text.as_mut_ptr(),
            x,
            1,
            0,
            None,
        );
        CopyWindowToVram(SELECT_WIN_MON_CATEGORY, COPYWIN_GFX);
    }
}
unsafe fn Select_CreateMonSprite() {
    let monId: u8 = (*sFactorySelectScreen).cursorPos;
    let mon: *mut Pokemon = &raw mut (*sFactorySelectScreen).mons[monId].monData;
    let species: u16 = GetMonData3(mon, MON_DATA_SPECIES, null_mut()) as u16;
    let personality: u32 = GetMonData3(mon, MON_DATA_PERSONALITY, null_mut());
    let otId: u32 = GetMonData3(mon, MON_DATA_OT_ID, null_mut());
    (*sFactorySelectScreen).monPics[1].monSpriteId =
        CreateMonPicSprite_HandleDeoxys(species, otId, personality, 1, 88, 32, 15, TAG_NONE) as u8;
    gSprites[(*sFactorySelectScreen).monPics[1].monSpriteId].centerToCornerVecX = 0;
    gSprites[(*sFactorySelectScreen).monPics[1].monSpriteId].centerToCornerVecY = 0;
    (*sFactorySelectScreen).monPicAnimating = FALSE;
}
unsafe fn Select_SetMonPicAnimating(animating: u8) {
    (*sFactorySelectScreen).monPicAnimating = animating;
}
unsafe fn Select_ReshowMonSprite() {
    (*sFactorySelectScreen).monPics[1].bgSpriteId = CreateSprite(
        (&raw const *sSpriteTemplate_Select_MonPicBgAnim).cast_mut(),
        120,
        64,
        1,
    );
    StartSpriteAffineAnim(
        &raw mut gSprites[(*sFactorySelectScreen).monPics[1].bgSpriteId],
        2,
    );
    let mon: *mut Pokemon =
        &raw mut (*sFactorySelectScreen).mons[(*sFactorySelectScreen).cursorPos].monData;
    let species: u16 = GetMonData3(mon, MON_DATA_SPECIES, null_mut()) as u16;
    let personality: u32 = GetMonData3(mon, MON_DATA_PERSONALITY, null_mut());
    let otId: u32 = GetMonData3(mon, MON_DATA_OT_ID, null_mut());
    (*sFactorySelectScreen).monPics[1].monSpriteId =
        CreateMonPicSprite_HandleDeoxys(species, otId, personality, 1, 88, 32, 15, TAG_NONE) as u8;
    gSprites[(*sFactorySelectScreen).monPics[1].monSpriteId].centerToCornerVecX = 0;
    gSprites[(*sFactorySelectScreen).monPics[1].monSpriteId].centerToCornerVecY = 0;
    gSprites[(*sFactorySelectScreen).monPics[1].bgSpriteId].set_invisible(1);
}
unsafe fn Select_CreateChosenMonsSprites() {
    for i in 0..(FRONTIER_PARTY_SIZE as u8) {
        for j in 0..SELECTABLE_MONS_COUNT {
            if (*sFactorySelectScreen).mons[j].selectedId as i32 == i as i32 + 1 {
                let mon: *mut Pokemon = &raw mut (*sFactorySelectScreen).mons[j].monData;
                let species: u16 = GetMonData3(mon, MON_DATA_SPECIES, null_mut()) as u16;
                let personality: u32 = GetMonData3(mon, MON_DATA_PERSONALITY, null_mut());
                let otId: u32 = GetMonData3(mon, MON_DATA_OT_ID, null_mut());
                (*sFactorySelectScreen).monPics[i].monSpriteId = CreateMonPicSprite_HandleDeoxys(
                    species,
                    otId,
                    personality,
                    TRUE,
                    i as i16 * 72 + 16,
                    32,
                    i + 13,
                    TAG_NONE,
                ) as u8;
                gSprites[(*sFactorySelectScreen).monPics[i].monSpriteId].centerToCornerVecX = 0;
                gSprites[(*sFactorySelectScreen).monPics[i].monSpriteId].centerToCornerVecY = 0;
                break;
            }
        }
    }
    (*sFactorySelectScreen).monPicAnimating = FALSE;
}
pub(crate) unsafe fn SpriteCB_OpenChosenMonPics(sprite: *mut Sprite) {
    let mut taskId: u8 = 0;
    if (*sprite).affineAnimEnded() != 0
        && gSprites[(*sFactorySelectScreen).monPics[0].bgSpriteId].affineAnimEnded() != 0
        && gSprites[(*sFactorySelectScreen).monPics[2].bgSpriteId].affineAnimEnded() != 0
    {
        (*sprite).set_invisible(TRUE as u16);
        gSprites[(*sFactorySelectScreen).monPics[0].bgSpriteId].set_invisible(TRUE as u16);
        gSprites[(*sFactorySelectScreen).monPics[2].bgSpriteId].set_invisible(TRUE as u16);
        taskId = CreateTask(Some(Select_Task_OpenChosenMonPics), 1);
        task_func(taskId).unwrap_unchecked()(taskId);
        (*sprite).callback = Some(SpriteCallbackDummy);
    }
}
pub(crate) unsafe fn SpriteCB_CloseChosenMonPics(sprite: *mut Sprite) {
    if (*sprite).affineAnimEnded() != 0
        && gSprites[(*sFactorySelectScreen).monPics[0].bgSpriteId].affineAnimEnded() != 0
        && gSprites[(*sFactorySelectScreen).monPics[2].bgSpriteId].affineAnimEnded() != 0
    {
        FreeOamMatrix((*sprite).oam.matrixNum() as u8);
        FreeOamMatrix(
            gSprites[(*sFactorySelectScreen).monPics[0].bgSpriteId]
                .oam
                .matrixNum() as u8,
        );
        FreeOamMatrix(
            gSprites[(*sFactorySelectScreen).monPics[2].bgSpriteId]
                .oam
                .matrixNum() as u8,
        );
        (*sFactorySelectScreen).monPicAnimating = FALSE;
        DestroySprite(&raw mut gSprites[(*sFactorySelectScreen).monPics[0].bgSpriteId]);
        DestroySprite(&raw mut gSprites[(*sFactorySelectScreen).monPics[2].bgSpriteId]);
        DestroySprite(sprite);
    }
}
pub(crate) unsafe fn Select_Task_OpenChosenMonPics(taskId: u8) {
    let task: *mut Task = &raw mut (*gTasks.as_ptr())[taskId];
    match (*task).data[tState] {
        0 => {
            (*task).data[tWinLeft] = 16;
            (*task).data[tWinRight] = 224;
            (*task).data[tWinTop] = 64;
            (*task).data[tWinBottom] = 65;
            SetGpuRegBits(REG_OFFSET_DISPCNT, DISPCNT_WIN0_ON);
            SetGpuReg(
                REG_OFFSET_WIN0H,
                ((*task).data[tWinLeft] as u16) << 8 | (*task).data[tWinRight] as u16,
            );
            SetGpuReg(
                REG_OFFSET_WIN0V,
                ((*task).data[tWinTop] as u16) << 8 | (*task).data[tWinBottom] as u16,
            );
            SetGpuReg(REG_OFFSET_WININ, 63);
            SetGpuReg(REG_OFFSET_WINOUT, 55);
        }
        1 => {
            ShowBg(3);
            SetGpuReg(REG_OFFSET_BLDCNT, 4680);
            SetGpuReg(REG_OFFSET_BLDALPHA, 1035);
        }
        2 => {
            (*task).data[tWinTop] -= 4;
            (*task).data[tWinBottom] += 4;
            if (*task).data[tWinTop] <= 32 || (*task).data[tWinBottom] >= 96 {
                (*task).data[tWinTop] = 32;
                (*task).data[tWinBottom] = 96;
                ClearGpuRegBits(REG_OFFSET_DISPCNT, DISPCNT_WIN0_ON);
            }
            SetGpuReg(
                REG_OFFSET_WIN0V,
                ((*task).data[tWinTop] as u16) << 8 | (*task).data[tWinBottom] as u16,
            );
            if (*task).data[tWinTop] != 32 {
                return;
            }
        }
        _ => {
            DestroyTask(taskId);
            Select_CreateChosenMonsSprites();
            return;
        }
    }
    (*task).data[tState] += 1;
}
pub(crate) unsafe fn Select_Task_CloseChosenMonPics(taskId: u8) {
    let task: *mut Task = &raw mut (*gTasks.as_ptr())[taskId];
    match (*task).data[tState] {
        0 => {
            (*task).data[tWinLeft] = 16;
            (*task).data[tWinRight] = 224;
            (*task).data[tWinTop] = 32;
            (*task).data[tWinBottom] = 96;
            SetGpuRegBits(REG_OFFSET_DISPCNT, DISPCNT_WIN0_ON);
            SetGpuReg(
                REG_OFFSET_WIN0H,
                ((*task).data[tWinLeft] as u16) << 8 | (*task).data[tWinRight] as u16,
            );
            SetGpuReg(
                REG_OFFSET_WIN0V,
                ((*task).data[tWinTop] as u16) << 8 | (*task).data[tWinBottom] as u16,
            );
            SetGpuReg(REG_OFFSET_WININ, 63);
            SetGpuReg(REG_OFFSET_WINOUT, 55);
            (*task).data[tState] += 1;
        }
        1 => {
            (*task).data[tWinTop] += 4;
            (*task).data[tWinBottom] -= 4;
            if (*task).data[tWinTop] >= 64 || (*task).data[tWinBottom] <= 65 {
                (*task).data[tWinTop] = 64;
                (*task).data[tWinBottom] = 65;
            }
            SetGpuReg(
                REG_OFFSET_WIN0V,
                ((*task).data[tWinTop] as u16) << 8 | (*task).data[tWinBottom] as u16,
            );
            if (*task).data[tWinTop] == 64 {
                (*task).data[tState] += 1;
            }
        }
        _ => {
            HideBg(3);
            gSprites[(*sFactorySelectScreen).monPics[1].bgSpriteId].set_invisible(FALSE as u16);
            gSprites[(*sFactorySelectScreen).monPics[1].bgSpriteId].callback =
                Some(SpriteCB_CloseChosenMonPics);
            gSprites[(*sFactorySelectScreen).monPics[0].bgSpriteId].set_invisible(0);
            gSprites[(*sFactorySelectScreen).monPics[0].bgSpriteId].callback =
                Some(SpriteCallbackDummy);
            gSprites[(*sFactorySelectScreen).monPics[2].bgSpriteId].set_invisible(FALSE as u16);
            gSprites[(*sFactorySelectScreen).monPics[2].bgSpriteId].callback =
                Some(SpriteCallbackDummy);
            StartSpriteAffineAnim(
                &raw mut gSprites[(*sFactorySelectScreen).monPics[1].bgSpriteId],
                1,
            );
            StartSpriteAffineAnim(
                &raw mut gSprites[(*sFactorySelectScreen).monPics[0].bgSpriteId],
                1,
            );
            StartSpriteAffineAnim(
                &raw mut gSprites[(*sFactorySelectScreen).monPics[2].bgSpriteId],
                1,
            );
            ClearGpuRegBits(REG_OFFSET_DISPCNT, DISPCNT_WIN0_ON);
            DestroyTask(taskId);
        }
    }
}
unsafe fn Select_ShowChosenMons() {
    (*sFactorySelectScreen).monPics[1].bgSpriteId = CreateSprite(
        (&raw const *sSpriteTemplate_Select_MonPicBgAnim).cast_mut(),
        120,
        64,
        1,
    );
    (*sFactorySelectScreen).monPics[0].bgSpriteId = CreateSprite(
        (&raw const *sSpriteTemplate_Select_MonPicBgAnim).cast_mut(),
        44,
        64,
        1,
    );
    (*sFactorySelectScreen).monPics[2].bgSpriteId = CreateSprite(
        (&raw const *sSpriteTemplate_Select_MonPicBgAnim).cast_mut(),
        196,
        64,
        1,
    );
    gSprites[(*sFactorySelectScreen).monPics[1].bgSpriteId].callback =
        Some(SpriteCB_OpenChosenMonPics);
    gSprites[(*sFactorySelectScreen).monPics[0].bgSpriteId].callback = Some(SpriteCallbackDummy);
    gSprites[(*sFactorySelectScreen).monPics[2].bgSpriteId].callback = Some(SpriteCallbackDummy);
    (*sFactorySelectScreen).monPicAnimating = TRUE;
}
unsafe fn Select_HideChosenMons() {
    FreeAndDestroyMonPicSprite((*sFactorySelectScreen).monPics[0].monSpriteId as u16);
    FreeAndDestroyMonPicSprite((*sFactorySelectScreen).monPics[1].monSpriteId as u16);
    FreeAndDestroyMonPicSprite((*sFactorySelectScreen).monPics[2].monSpriteId as u16);
    let taskId: u8 = CreateTask(Some(Select_Task_CloseChosenMonPics), 1);
    task_func(taskId).unwrap_unchecked()(taskId);
    (*sFactorySelectScreen).monPicAnimating = TRUE;
}
unsafe fn Select_SetWinRegs(mWin0H: i16, nWin0H: i16, mWin0V: i16, nWin0V: i16) {
    SetGpuRegBits(REG_OFFSET_DISPCNT, DISPCNT_WIN0_ON);
    SetGpuReg(REG_OFFSET_WIN0H, (mWin0H as u16) << 8 | nWin0H as u16);
    SetGpuReg(REG_OFFSET_WIN0V, (mWin0V as u16) << 8 | nWin0V as u16);
    SetGpuReg(REG_OFFSET_WININ, 63);
    SetGpuReg(REG_OFFSET_WINOUT, 55);
}
unsafe fn Select_AreSpeciesValid(monId: u16) -> u32 {
    let species: u32 = (*gFacilityTrainerMons.at(monId)).species as u32;
    let selectState: u8 = (*sFactorySelectScreen).selectingMonsState;
    for i in 1..selectState {
        for j in 0..SELECTABLE_MONS_COUNT {
            if (*sFactorySelectScreen).mons[j].selectedId == i {
                if (*gFacilityTrainerMons.at((*sFactorySelectScreen).mons[j].monId)).species as u32
                    == species
                {
                    return FALSE as u32;
                }
                break;
            }
        }
    }
    TRUE as u32
}
pub(crate) unsafe fn Select_Task_FadeSpeciesName(taskId: u8) {
    match task_get(taskId, tState) {
        FADESTATE_INIT => {
            (*sFactorySelectScreen).fadeSpeciesNameCoeffDelay = 0;
            (*sFactorySelectScreen).fadeSpeciesNameCoeff = 0;
            (*sFactorySelectScreen).fadeSpeciesNameFadeOut = TRUE;
            task_set(taskId, tState, FADESTATE_RUN);
        }
        FADESTATE_RUN => {
            if (*sFactorySelectScreen).fadeSpeciesNameActive != 0 {
                if (*sFactorySelectScreen).faceSpeciesNameDelay != 0 {
                    task_set(taskId, tState, FADESTATE_DELAY);
                } else {
                    (*sFactorySelectScreen).fadeSpeciesNameCoeffDelay += 1;
                    if (*sFactorySelectScreen).fadeSpeciesNameCoeffDelay > 6 {
                        (*sFactorySelectScreen).fadeSpeciesNameCoeffDelay = 0;
                        if (*sFactorySelectScreen).fadeSpeciesNameFadeOut == 0 {
                            (*sFactorySelectScreen).fadeSpeciesNameCoeff -= 1;
                        } else {
                            (*sFactorySelectScreen).fadeSpeciesNameCoeff += 1;
                        }
                    }
                    BlendPalettes(16384, (*sFactorySelectScreen).fadeSpeciesNameCoeff, 0);
                    if (*sFactorySelectScreen).fadeSpeciesNameCoeff > 5 {
                        (*sFactorySelectScreen).fadeSpeciesNameFadeOut = FALSE;
                    } else if (*sFactorySelectScreen).fadeSpeciesNameCoeff == 0 {
                        task_set(taskId, tState, FADESTATE_DELAY);
                        (*sFactorySelectScreen).fadeSpeciesNameFadeOut = TRUE;
                    }
                }
            }
        }
        FADESTATE_DELAY => {
            if (*sFactorySelectScreen).faceSpeciesNameDelay > 14 {
                (*sFactorySelectScreen).faceSpeciesNameDelay = 0;
                task_set(taskId, tState, FADESTATE_RUN);
            } else {
                (*sFactorySelectScreen).faceSpeciesNameDelay += 1;
            }
        }
        _ => {}
    }
}
pub(crate) unsafe fn Swap_CB2() {
    AnimateSprites();
    BuildOamBuffer();
    RunTextPrinters();
    UpdatePaletteFade();
    RunTasks();
}
pub(crate) unsafe fn Swap_VblankCb() {
    LoadOam();
    ProcessSpriteCopyRequests();
    TransferPlttBuffer();
}
unsafe fn CopySwappedMonData() {
    gPlayerParty[(*sFactorySwapScreen).playerMonId] = gEnemyParty[(*sFactorySwapScreen).enemyMonId];
    let mut friendship: u8 = 0;
    SetMonData(
        &raw mut gPlayerParty[(*sFactorySwapScreen).playerMonId],
        MON_DATA_FRIENDSHIP,
        &raw mut friendship as *mut c_void,
    );
    (*gSaveBlock2Ptr).frontier.rentalMons[(*sFactorySwapScreen).playerMonId].monId =
        (*gSaveBlock2Ptr).frontier.rentalMons
            [(*sFactorySwapScreen).enemyMonId as i32 + FRONTIER_PARTY_SIZE]
            .monId;
    (*gSaveBlock2Ptr).frontier.rentalMons[(*sFactorySwapScreen).playerMonId].ivs =
        (*gSaveBlock2Ptr).frontier.rentalMons
            [(*sFactorySwapScreen).enemyMonId as i32 + FRONTIER_PARTY_SIZE]
            .ivs;
    (*gSaveBlock2Ptr).frontier.rentalMons[(*sFactorySwapScreen).playerMonId].personality =
        GetMonData3(
            &raw mut gEnemyParty[(*sFactorySwapScreen).enemyMonId],
            MON_DATA_PERSONALITY,
            null_mut(),
        );
    (*gSaveBlock2Ptr).frontier.rentalMons[(*sFactorySwapScreen).playerMonId].abilityNum =
        GetBoxMonData3(
            &raw mut gEnemyParty[(*sFactorySwapScreen).enemyMonId].r#box,
            MON_DATA_ABILITY_NUM,
            null_mut(),
        ) as u8;
}
pub(crate) unsafe fn Swap_Task_OpenSummaryScreen(taskId: u8) {
    match task_get(taskId, tState) {
        STATE_SUMMARY_FADE => {
            BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 16, 0);
            task_set(taskId, tState, STATE_SUMMARY_CLEAN);
        }
        STATE_SUMMARY_CLEAN => {
            if gPaletteFade.active() == 0 {
                DestroyTask((*sFactorySwapScreen).fadeSpeciesNameTaskId);
                HideMonPic(
                    (*sFactorySwapScreen).monPic,
                    &raw mut (*sFactorySwapScreen).monPicAnimating,
                );
                Swap_DestroyAllSprites();
                Free(sSwapMenuTilesetBuffer as *mut c_void);
                sSwapMenuTilesetBuffer = null_mut();
                Free(sSwapMonPicBgTilesetBuffer as *mut c_void);
                sSwapMonPicBgTilesetBuffer = null_mut();
                Free(sSwapMenuTilemapBuffer as *mut c_void);
                sSwapMenuTilemapBuffer = null_mut();
                Free(sSwapMonPicBgTilemapBuffer as *mut c_void);
                sSwapMonPicBgTilemapBuffer = null_mut();
                FreeAllWindowBuffers();
                task_set(taskId, tState, STATE_SUMMARY_SHOW);
            }
        }
        STATE_SUMMARY_SHOW => {
            DestroyTask(taskId);
            (*sFactorySwapScreen).fromSummaryScreen = TRUE;
            (*sFactorySwapScreen).speciesNameColorBackup = gPlttBufferUnfaded[244];
            ShowPokemonSummaryScreen(
                SUMMARY_MODE_NORMAL,
                gPlayerParty.as_mut_ptr() as *mut c_void,
                (*sFactorySwapScreen).cursorPos,
                2,
                Some(CB2_InitSwapScreen),
            );
        }
        _ => {}
    }
}
pub(crate) unsafe fn Swap_Task_Exit(taskId: u8) {
    if (*sFactorySwapScreen).monPicAnimating == TRUE {
        return;
    }
    match task_get(taskId, tState) {
        0 => {
            if (*sFactorySwapScreen).monSwapped == TRUE {
                task_set(taskId, tState, task_get(taskId, tState) + 1);
                gSpecialVar_Result = FALSE as u16;
            } else {
                task_set(taskId, tState, 2);
                gSpecialVar_Result = TRUE as u16;
            }
        }
        1 => {
            if (*sFactorySwapScreen).monSwapped == TRUE {
                (*sFactorySwapScreen).enemyMonId = (*sFactorySwapScreen).cursorPos;
                CopySwappedMonData();
            }
            task_set(taskId, tState, task_get(taskId, tState) + 1);
        }
        2 => {
            BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 16, 0);
            task_set(taskId, tState, task_get(taskId, tState) + 1);
        }
        3 if UpdatePaletteFade() == 0 => {
            DestroyTask((*sFactorySwapScreen).fadeSpeciesNameTaskId);
            Swap_DestroyAllSprites();
            Free(sSwapMenuTilesetBuffer as *mut c_void);
            sSwapMenuTilesetBuffer = null_mut();
            Free(sSwapMonPicBgTilesetBuffer as *mut c_void);
            sSwapMonPicBgTilesetBuffer = null_mut();
            Free(sSwapMenuTilemapBuffer as *mut c_void);
            sSwapMenuTilemapBuffer = null_mut();
            Free(sSwapMonPicBgTilemapBuffer as *mut c_void);
            sSwapMonPicBgTilemapBuffer = null_mut();
            Free(sFactorySwapScreen as *mut c_void);
            sFactorySwapScreen = null_mut();
            FreeAllWindowBuffers();
            SetMainCallback2(Some(CB2_ReturnToFieldContinueScript));
            DestroyTask(taskId);
        }
        _ => {}
    }
}
pub(crate) unsafe fn Swap_Task_HandleYesNo(taskId: u8) {
    let mut loPtr: u16 = 0;
    let mut hiPtr: u16 = 0;
    if (*sFactorySwapScreen).monPicAnimating == TRUE {
        return;
    }
    match task_get(taskId, tState) {
        STATE_YESNO_SHOW => {
            Swap_ShowYesNoOptions();
            task_set(taskId, tState, STATE_YESNO_HANDLE_INPUT);
        }
        STATE_YESNO_HANDLE_INPUT => {
            if gMain.newKeys as i32 & A_BUTTON != 0 {
                PlaySE(SE_SELECT);
                if (*sFactorySwapScreen).yesNoCursorPos == 0 {
                    task_set(taskId, tSaidYes, TRUE as i16);
                    hiPtr = task_get(taskId, tFollowUpTaskPtrHi) as u16;
                    loPtr = task_get(taskId, tFollowUpTaskPtrLo) as u16;
                    task_set_func(
                        taskId,
                        core::mem::transmute::<_, Option<unsafe fn(u8)>>(
                            ((hiPtr as i32) << 16 | loPtr as i32) as usize as *mut c_void,
                        ),
                    );
                } else {
                    task_set(taskId, tSaidYes, FALSE as i16);
                    Swap_ErasePopupMenu(SWAP_WIN_YES_NO);
                    hiPtr = task_get(taskId, tFollowUpTaskPtrHi) as u16;
                    loPtr = task_get(taskId, tFollowUpTaskPtrLo) as u16;
                    task_set_func(
                        taskId,
                        core::mem::transmute::<_, Option<unsafe fn(u8)>>(
                            ((hiPtr as i32) << 16 | loPtr as i32) as usize as *mut c_void,
                        ),
                    );
                }
            } else if gMain.newKeys as i32 & B_BUTTON != 0 {
                PlaySE(SE_SELECT);
                task_set(taskId, tSaidYes, FALSE as i16);
                Swap_ErasePopupMenu(SWAP_WIN_YES_NO);
                hiPtr = task_get(taskId, tFollowUpTaskPtrHi) as u16;
                loPtr = task_get(taskId, tFollowUpTaskPtrLo) as u16;
                task_set_func(
                    taskId,
                    core::mem::transmute::<_, Option<unsafe fn(u8)>>(
                        ((hiPtr as i32) << 16 | loPtr as i32) as usize as *mut c_void,
                    ),
                );
            } else if gMain.newAndRepeatedKeys as i32 & DPAD_UP != 0 {
                PlaySE(SE_SELECT);
                Swap_UpdateYesNoCursorPosition(-1);
            } else if gMain.newAndRepeatedKeys as i32 & DPAD_DOWN != 0 {
                PlaySE(SE_SELECT);
                Swap_UpdateYesNoCursorPosition(1);
            }
        }
        _ => {}
    }
}
pub(crate) fn Swap_HandleQuitSwappingResponse(taskId: u8) {
    if task_get(taskId, tSaidYes) == TRUE as i16 {
        task_set(taskId, tState, 0);
        task_set_func(taskId, Some(Swap_Task_Exit));
    } else {
        task_set(taskId, tState, 0);
        task_set(
            taskId,
            tFollowUpTaskPtrHi,
            (Swap_Task_HandleChooseMons as *const () as usize as u32 >> 16) as i16,
        );
        task_set(
            taskId,
            tFollowUpTaskPtrLo,
            Swap_Task_HandleChooseMons as *const () as usize as u32 as i16,
        );
        task_set(taskId, tFollowUpTaskState, STATE_CHOOSE_MONS_HANDLE_INPUT);
        task_set_func(taskId, Some(Swap_Task_ScreenInfoTransitionIn));
    }
}
pub(crate) unsafe fn Swap_AskQuitSwapping(taskId: u8) {
    if task_get(taskId, tState) == 0 {
        Swap_PrintOnInfoWindow(
            (*(&raw const crate::data::strings::gText_QuitSwapping).cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
        );
        (*sFactorySwapScreen).monSwapped = FALSE;
        task_set(taskId, tState, STATE_YESNO_SHOW);
        task_set(
            taskId,
            tFollowUpTaskPtrHi,
            (Swap_HandleQuitSwappingResponse as *const () as usize as u32 >> 16) as i16,
        );
        task_set(
            taskId,
            tFollowUpTaskPtrLo,
            Swap_HandleQuitSwappingResponse as *const () as usize as u32 as i16,
        );
        task_set_func(taskId, Some(Swap_Task_HandleYesNo));
    }
}
pub(crate) unsafe fn Swap_HandleAcceptMonResponse(taskId: u8) {
    CloseMonPic(
        (*sFactorySwapScreen).monPic,
        &raw mut (*sFactorySwapScreen).monPicAnimating,
        TRUE,
    );
    if task_get(taskId, tSaidYes) == TRUE as i16 {
        task_set(taskId, tState, 0);
        task_set_func(taskId, Some(Swap_Task_Exit));
    } else {
        task_set(taskId, tState, 0);
        task_set(
            taskId,
            tFollowUpTaskPtrHi,
            (Swap_Task_HandleChooseMons as *const () as usize as u32 >> 16) as i16,
        );
        task_set(
            taskId,
            tFollowUpTaskPtrLo,
            Swap_Task_HandleChooseMons as *const () as usize as u32 as i16,
        );
        task_set(taskId, tFollowUpTaskState, STATE_CHOOSE_MONS_HANDLE_INPUT);
        task_set_func(taskId, Some(Swap_Task_ScreenInfoTransitionIn));
    }
}
pub(crate) unsafe fn Swap_AskAcceptMon(taskId: u8) {
    if task_get(taskId, tState) == 0 {
        OpenMonPic(
            &raw mut (*sFactorySwapScreen).monPic.bgSpriteId,
            &raw mut (*sFactorySwapScreen).monPicAnimating,
            TRUE,
        );
        Swap_PrintOnInfoWindow(
            (*(&raw const crate::data::strings::gText_AcceptThisPkmn).cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
        );
        (*sFactorySwapScreen).monSwapped = TRUE;
        task_set(taskId, tState, STATE_YESNO_SHOW);
        task_set(
            taskId,
            tFollowUpTaskPtrHi,
            (Swap_HandleAcceptMonResponse as *const () as usize as u32 >> 16) as i16,
        );
        task_set(
            taskId,
            tFollowUpTaskPtrLo,
            Swap_HandleAcceptMonResponse as *const () as usize as u32 as i16,
        );
        task_set_func(taskId, Some(Swap_Task_HandleYesNo));
    }
}
pub(crate) unsafe fn Swap_Task_HandleMenu(taskId: u8) {
    match task_get(taskId, tState) {
        STATE_MENU_INIT => {
            if (*sFactorySwapScreen).fromSummaryScreen == 0 {
                OpenMonPic(
                    &raw mut (*sFactorySwapScreen).monPic.bgSpriteId,
                    &raw mut (*sFactorySwapScreen).monPicAnimating,
                    TRUE,
                );
            }
            task_set(taskId, tState, STATE_MENU_SHOW_OPTIONS);
        }
        STATE_MENU_SHOW_OPTIONS => {
            if (*sFactorySwapScreen).monPicAnimating != TRUE {
                Swap_ShowMenuOptions();
                task_set(taskId, tState, STATE_MENU_HANDLE_INPUT);
            }
        }
        STATE_MENU_HANDLE_INPUT if (*sFactorySwapScreen).monPicAnimating != TRUE => {
            if gMain.newKeys as i32 & A_BUTTON != 0 {
                PlaySE(SE_SELECT);
                Swap_RunMenuOptionFunc(taskId);
            } else if gMain.newKeys as i32 & B_BUTTON != 0 {
                PlaySE(SE_SELECT);
                CloseMonPic(
                    (*sFactorySwapScreen).monPic,
                    &raw mut (*sFactorySwapScreen).monPicAnimating,
                    TRUE,
                );
                Swap_ErasePopupMenu(SWAP_WIN_OPTIONS);
                task_set(taskId, tState, 0);
                task_set(
                    taskId,
                    tFollowUpTaskPtrHi,
                    (Swap_Task_HandleChooseMons as *const () as usize as u32 >> 16) as i16,
                );
                task_set(
                    taskId,
                    tFollowUpTaskPtrLo,
                    Swap_Task_HandleChooseMons as *const () as usize as u32 as i16,
                );
                task_set(taskId, tFollowUpTaskState, STATE_CHOOSE_MONS_HANDLE_INPUT);
                task_set_func(taskId, Some(Swap_Task_ScreenInfoTransitionIn));
            } else if gMain.newAndRepeatedKeys as i32 & DPAD_UP != 0 {
                Swap_UpdateMenuCursorPosition(-1);
            } else if gMain.newAndRepeatedKeys as i32 & DPAD_DOWN != 0 {
                Swap_UpdateMenuCursorPosition(1);
            }
        }
        _ => {}
    }
}
pub(crate) unsafe fn Swap_Task_HandleChooseMons(taskId: u8) {
    match task_get(taskId, tState) {
        STATE_CHOOSE_MONS_INIT => {
            if gPaletteFade.active() == 0 {
                (*sFactorySwapScreen).fadeSpeciesNameActive = TRUE;
                task_set(taskId, tState, STATE_CHOOSE_MONS_HANDLE_INPUT);
            }
        }
        STATE_CHOOSE_MONS_HANDLE_INPUT => {
            if gMain.newKeys as i32 & A_BUTTON != 0 {
                PlaySE(SE_SELECT);
                (*sFactorySwapScreen).fadeSpeciesNameActive = FALSE;
                Swap_PrintMonSpeciesAtFade();
                Swap_EraseSpeciesWindow();
                Swap_RunActionFunc(taskId);
            } else if gMain.newKeys as i32 & B_BUTTON != 0 {
                PlaySE(SE_SELECT);
                (*sFactorySwapScreen).fadeSpeciesNameActive = FALSE;
                Swap_PrintMonSpeciesAtFade();
                Swap_EraseSpeciesWindow();
                task_set(
                    taskId,
                    tFollowUpTaskPtrHi,
                    (Swap_AskQuitSwapping as *const () as usize as u32 >> 16) as i16,
                );
                task_set(
                    taskId,
                    tFollowUpTaskPtrLo,
                    Swap_AskQuitSwapping as *const () as usize as u32 as i16,
                );
                task_set(taskId, tState, 0);
                task_set(taskId, tFollowUpTaskState, 0);
                task_set_func(taskId, Some(Swap_Task_ScreenInfoTransitionOut));
            } else if gMain.newAndRepeatedKeys as i32 & DPAD_LEFT != 0 {
                Swap_UpdateBallCursorPosition(-1);
                Swap_PrintMonCategory();
                Swap_PrintMonSpecies();
            } else if gMain.newAndRepeatedKeys as i32 & DPAD_RIGHT != 0 {
                Swap_UpdateBallCursorPosition(1);
                Swap_PrintMonCategory();
                Swap_PrintMonSpecies();
            } else if gMain.newAndRepeatedKeys as i32 & DPAD_DOWN != 0 {
                Swap_UpdateActionCursorPosition(1);
                Swap_PrintMonCategory();
                Swap_PrintMonSpecies();
            } else if gMain.newAndRepeatedKeys as i32 & DPAD_UP != 0 {
                Swap_UpdateActionCursorPosition(-1);
                Swap_PrintMonCategory();
                Swap_PrintMonSpecies();
            }
        }
        _ => {}
    }
}
pub(crate) unsafe fn Swap_Task_FadeSpeciesName(taskId: u8) {
    match task_get(taskId, tState) {
        FADESTATE_INIT => {
            (*sFactorySwapScreen).fadeSpeciesNameCoeffDelay = 0;
            (*sFactorySwapScreen).fadeSpeciesNameCoeff = 0;
            (*sFactorySwapScreen).fadeSpeciesNameFadeOut = TRUE;
            task_set(taskId, tState, FADESTATE_RUN);
        }
        FADESTATE_RUN => {
            if (*sFactorySwapScreen).fadeSpeciesNameActive != 0 {
                if (*sFactorySwapScreen).faceSpeciesNameDelay != 0 {
                    task_set(taskId, tState, FADESTATE_DELAY);
                } else {
                    (*sFactorySwapScreen).fadeSpeciesNameCoeffDelay += 1;
                    if (*sFactorySwapScreen).fadeSpeciesNameCoeffDelay > 6 {
                        (*sFactorySwapScreen).fadeSpeciesNameCoeffDelay = 0;
                        if (*sFactorySwapScreen).fadeSpeciesNameFadeOut == 0 {
                            (*sFactorySwapScreen).fadeSpeciesNameCoeff -= 1;
                        } else {
                            (*sFactorySwapScreen).fadeSpeciesNameCoeff += 1;
                        }
                    }
                    BlendPalettes(16384, (*sFactorySwapScreen).fadeSpeciesNameCoeff, 0);
                    if (*sFactorySwapScreen).fadeSpeciesNameCoeff > 5 {
                        (*sFactorySwapScreen).fadeSpeciesNameFadeOut = FALSE;
                    } else if (*sFactorySwapScreen).fadeSpeciesNameCoeff == 0 {
                        task_set(taskId, tState, FADESTATE_DELAY);
                        (*sFactorySwapScreen).fadeSpeciesNameFadeOut = TRUE;
                    }
                }
            }
        }
        FADESTATE_DELAY => {
            if (*sFactorySwapScreen).faceSpeciesNameDelay > 14 {
                (*sFactorySwapScreen).faceSpeciesNameDelay = 0;
                task_set(taskId, tState, FADESTATE_RUN);
            } else {
                (*sFactorySwapScreen).faceSpeciesNameDelay += 1;
            }
        }
        _ => {}
    }
}
pub(crate) unsafe fn Swap_Task_FadeOutSpeciesName(taskId: u8) {
    match task_get(taskId, tState) {
        0 => {
            (*sFactorySwapScreen).fadeSpeciesNameCoeffDelay = 0;
            task_set(taskId, tFadeOutFinished, FALSE as i16);
            task_set(taskId, tState, task_get(taskId, tState) + 1);
        }
        1 => {
            LoadPalette(&raw mut gPlttBufferUnfaded[240] as *mut c_void, 224, 10);
            task_set(taskId, tState, task_get(taskId, tState) + 1);
        }
        2 => {
            if (*sFactorySwapScreen).fadeSpeciesNameCoeff > 15 {
                task_set(taskId, tFadeOutFinished, TRUE as i16);
                task_set(taskId, tState, task_get(taskId, tState) + 1);
            }
            (*sFactorySwapScreen).fadeSpeciesNameCoeffDelay += 1;
            if (*sFactorySwapScreen).fadeSpeciesNameCoeffDelay > 3 {
                (*sFactorySwapScreen).fadeSpeciesNameCoeffDelay = 0;
                gPlttBufferUnfaded[244] = gPlttBufferFaded[228];
                (*sFactorySwapScreen).fadeSpeciesNameCoeff += 1;
            }
            BlendPalettes(16384, (*sFactorySwapScreen).fadeSpeciesNameCoeff, 0);
        }
        _ => {}
    }
}
pub(crate) unsafe fn Swap_Task_SlideCycleBalls(taskId: u8) {
    let mut i: i8 = 0;
    let mut lastX: u8 = 0;
    let mut finished: u8 = 0;
    match task_get(taskId, tState) {
        0 => {
            task_set(taskId, 1, 0);
            task_set(taskId, 2, FALSE as i16);
            task_set(taskId, 3, FALSE as i16);
            task_set(taskId, tState, 1);
        }
        1 => {
            lastX = 0;
            i = 2;
            while i >= 0 {
                if i != 2 {
                    let posX: u8 = lastX - gSprites[(*sFactorySwapScreen).ballSpriteIds[i]].x as u8;
                    if posX == 16 || task_get(taskId, i as i32 + 1 + 1) == 1 {
                        lastX = gSprites[(*sFactorySwapScreen).ballSpriteIds[i]].x as u8;
                        gSprites[(*sFactorySwapScreen).ballSpriteIds[i]].x += 10;
                    } else if posX > 16 {
                        gSprites[(*sFactorySwapScreen).ballSpriteIds[i]].x =
                            gSprites[(*sFactorySwapScreen).ballSpriteIds[i as i32 + 1]].x - 48;
                    }
                } else {
                    lastX = gSprites[(*sFactorySwapScreen).ballSpriteIds[i]].x as u8;
                    gSprites[(*sFactorySwapScreen).ballSpriteIds[i]].x += 10;
                }
                if task_get(taskId, i as i32 + TRUE as i32) == TRUE as i16 {
                    if gSprites[(*sFactorySwapScreen).ballSpriteIds[i]].x as i32
                        > i as i32 * 48 + 72
                    {
                        gSprites[(*sFactorySwapScreen).ballSpriteIds[i]].x = i as i16 * 48 + 72;
                        finished = TRUE;
                    } else if gSprites[(*sFactorySwapScreen).ballSpriteIds[i]].x as i32
                        == i as i32 * 48 + 72
                    {
                        finished = TRUE;
                    } else {
                        finished = FALSE;
                    }
                } else {
                    finished = FALSE;
                }
                if gSprites[(*sFactorySwapScreen).ballSpriteIds[i]].x as i32 - 16
                    > DISPLAY_WIDTH as i32
                {
                    lastX = gSprites[(*sFactorySwapScreen).ballSpriteIds[i]].x as u8;
                    gSprites[(*sFactorySwapScreen).ballSpriteIds[i]].x = -16;
                    if (*sFactorySwapScreen).inEnemyScreen == TRUE {
                        gSprites[(*sFactorySwapScreen).ballSpriteIds[i]]
                            .oam
                            .set_paletteNum(IndexOfSpritePaletteTag(PALTAG_BALL_SELECTED) as u16);
                    } else {
                        gSprites[(*sFactorySwapScreen).ballSpriteIds[i]]
                            .oam
                            .set_paletteNum(IndexOfSpritePaletteTag(PALTAG_BALL_GRAY) as u16);
                    }
                    task_set(taskId, i as i32 + TRUE as i32, TRUE as i16);
                }
                i -= 1;
            }
            if finished == TRUE {
                DestroyTask(taskId);
            }
        }
        _ => {}
    }
}
pub(crate) unsafe fn Swap_Task_SlideButtonOnOffScreen(taskId: u8) {
    let mut posX: i32 = 0;
    let mut deltaX: i8 = task_get(taskId, 3) as i8;
    let mut sliding: u8 = 0;
    let mut currPosX: i16 = 0;
    let mut prevTaskId: u8 = 0;
    if task_get(taskId, tSlidingOn) == TRUE as i16 {
        deltaX *= -1;
    }
    match task_get(taskId, tState) {
        SLIDE_BUTTON_PKMN => {
            currPosX = gSprites[(*sFactorySwapScreen).pkmnForSwapButtonSpriteIds[0][0]].x;
            if task_get(taskId, tSlidingOn) == 0 {
                if (currPosX as i32 + deltaX as i32) < DISPLAY_WIDTH as i32 {
                    sliding = TRUE;
                } else {
                    sliding = FALSE;
                    posX = DISPLAY_WIDTH as i32;
                }
            } else {
                if currPosX as i32 + deltaX as i32 > 160 {
                    sliding = TRUE;
                } else {
                    sliding = FALSE;
                    posX = 160;
                }
            }
            if sliding == TRUE {
                for i in 0..3u8 {
                    for j in 0..2u8 {
                        gSprites[(*sFactorySwapScreen).pkmnForSwapButtonSpriteIds[j][i]].x +=
                            deltaX as i16;
                    }
                }
            } else {
                for j in 0..2u8 {
                    gSprites[(*sFactorySwapScreen).pkmnForSwapButtonSpriteIds[j][0]].x =
                        posX as i16;
                    gSprites[(*sFactorySwapScreen).pkmnForSwapButtonSpriteIds[j][1]].x =
                        posX as i16 + 16;
                    gSprites[(*sFactorySwapScreen).pkmnForSwapButtonSpriteIds[j][2]].x =
                        posX as i16 + 48;
                }
                prevTaskId = task_get(taskId, tTaskId) as u8;
                task_set(prevTaskId, 3, TRUE as i16);
                DestroyTask(taskId);
            }
        }
        SLIDE_BUTTON_CANCEL => {
            currPosX = gSprites[(*sFactorySwapScreen).cancelButtonSpriteIds[0][0]].x;
            if task_get(taskId, tSlidingOn) == 0 {
                if (currPosX as i32 + deltaX as i32) < DISPLAY_WIDTH as i32 {
                    sliding = TRUE;
                } else {
                    sliding = FALSE;
                    posX = DISPLAY_WIDTH as i32;
                }
            } else {
                if currPosX as i32 + deltaX as i32 > 192 {
                    sliding = TRUE;
                } else {
                    sliding = FALSE;
                    posX = 192;
                }
            }
            if sliding == TRUE {
                for i in 0..2u8 {
                    for j in 0..2u8 {
                        gSprites[(*sFactorySwapScreen).cancelButtonSpriteIds[j][i]].x +=
                            deltaX as i16;
                    }
                }
            } else {
                for j in 0..2u8 {
                    gSprites[(*sFactorySwapScreen).cancelButtonSpriteIds[j][0]].x = posX as i16;
                    gSprites[(*sFactorySwapScreen).cancelButtonSpriteIds[j][1]].x =
                        posX as i16 + 16;
                }
                prevTaskId = task_get(taskId, tTaskId) as u8;
                task_set(prevTaskId, tSlideFinishedCancel, TRUE as i16);
                DestroyTask(taskId);
            }
        }
        _ => {}
    }
}
pub(crate) unsafe fn Swap_Task_ScreenInfoTransitionOut(taskId: u8) {
    let mut slideTaskId: u8 = 0;
    let mut hiPtr: u16 = 0;
    let mut loPtr: u16 = 0;
    match task_get(taskId, tState) {
        0 => {
            LoadPalette(sSwapText_Pal.as_ptr().cast_mut() as *mut c_void, 224, 10);
            Swap_PrintActionStrings();
            PutWindowTilemap(SWAP_WIN_ACTION_FADE);
            task_set(taskId, tState, task_get(taskId, tState) + 1);
        }
        1 => {
            Swap_ErasePopupMenu(SWAP_WIN_OPTIONS);
            task_set(taskId, tState, task_get(taskId, tState) + 1);
        }
        2 => {
            BeginNormalPaletteFade(16384, 0, 0, 16, sPokeballGray_Pal[37]);
            task_set(taskId, tState, task_get(taskId, tState) + 1);
        }
        3 => {
            if gPaletteFade.active() == 0 {
                FillWindowPixelBuffer(SWAP_WIN_ACTION_FADE, 0);
                CopyWindowToVram(SWAP_WIN_ACTION_FADE, COPYWIN_GFX);
                if (*sFactorySwapScreen).inEnemyScreen == TRUE {
                    slideTaskId = CreateTask(Some(Swap_Task_SlideButtonOnOffScreen), 0);
                    task_set(taskId, 3, FALSE as i16);
                    task_set(slideTaskId, tTaskId, taskId as i16);
                    task_set(slideTaskId, tState, SLIDE_BUTTON_PKMN);
                    task_set(slideTaskId, 2, FALSE as i16);
                    task_set(slideTaskId, 3, 6);
                    task_set(taskId, 2, 5);
                    task_set(taskId, tState, task_get(taskId, tState) + 1);
                } else {
                    slideTaskId = CreateTask(Some(Swap_Task_SlideButtonOnOffScreen), 0);
                    task_set(taskId, 3, TRUE as i16);
                    task_set(taskId, tSlideFinishedCancel, FALSE as i16);
                    task_set(slideTaskId, tTaskId, taskId as i16);
                    task_set(slideTaskId, tState, SLIDE_BUTTON_CANCEL);
                    task_set(slideTaskId, 2, FALSE as i16);
                    task_set(slideTaskId, 3, 6);
                    task_set(taskId, tState, task_get(taskId, tState) + 2);
                }
            }
        }
        4 => {
            if task_get(taskId, 2) == 0 {
                slideTaskId = CreateTask(Some(Swap_Task_SlideButtonOnOffScreen), 0);
                task_set(taskId, tSlideFinishedCancel, FALSE as i16);
                task_set(slideTaskId, tTaskId, taskId as i16);
                task_set(slideTaskId, tState, SLIDE_BUTTON_CANCEL);
                task_set(slideTaskId, 2, FALSE as i16);
                task_set(slideTaskId, 3, 6);
                task_set(taskId, tState, task_get(taskId, tState) + 1);
            } else {
                task_set(taskId, 2, task_get(taskId, 2) - 1);
            }
        }
        5 if task_get(taskId, 3) == TRUE as i16
            && task_get(taskId, tSlideFinishedCancel) == TRUE as i16 =>
        {
            task_set(taskId, tState, task_get(taskId, tFollowUpTaskState));
            hiPtr = task_get(taskId, tFollowUpTaskPtrHi) as u16;
            loPtr = task_get(taskId, tFollowUpTaskPtrLo) as u16;
            task_set_func(
                taskId,
                core::mem::transmute::<_, Option<unsafe fn(u8)>>(
                    ((hiPtr as i32) << 16 | loPtr as i32) as usize as *mut c_void,
                ),
            );
        }
        _ => {}
    }
}
pub(crate) unsafe fn Swap_Task_ScreenInfoTransitionIn(taskId: u8) {
    let mut slideTaskId: u8 = 0;
    let mut hiPtr: u16 = 0;
    let mut loPtr: u16 = 0;
    if (*sFactorySwapScreen).monPicAnimating == TRUE {
        return;
    }
    match task_get(taskId, tState) {
        0 => {
            if (*sFactorySwapScreen).inEnemyScreen == TRUE {
                slideTaskId = CreateTask(Some(Swap_Task_SlideButtonOnOffScreen), 0);
                task_set(taskId, 3, FALSE as i16);
                task_set(slideTaskId, tTaskId, taskId as i16);
                task_set(slideTaskId, tState, SLIDE_BUTTON_PKMN);
                task_set(slideTaskId, 2, TRUE as i16);
                task_set(slideTaskId, 3, 6);
                task_set(taskId, 2, 10);
                task_set(taskId, tState, task_get(taskId, tState) + 1);
            } else {
                slideTaskId = CreateTask(Some(Swap_Task_SlideButtonOnOffScreen), 0);
                task_set(taskId, 3, TRUE as i16);
                task_set(taskId, tSlideFinishedCancel, FALSE as i16);
                task_set(slideTaskId, tTaskId, taskId as i16);
                task_set(slideTaskId, tState, SLIDE_BUTTON_CANCEL);
                task_set(slideTaskId, 2, TRUE as i16);
                task_set(slideTaskId, 3, 6);
                task_set(taskId, tState, task_get(taskId, tState) + 2);
            }
        }
        1 => {
            if task_get(taskId, 2) == 0 {
                slideTaskId = CreateTask(Some(Swap_Task_SlideButtonOnOffScreen), 0);
                task_set(taskId, tSlideFinishedCancel, FALSE as i16);
                task_set(slideTaskId, tTaskId, taskId as i16);
                task_set(slideTaskId, tState, SLIDE_BUTTON_CANCEL);
                task_set(slideTaskId, 2, TRUE as i16);
                task_set(slideTaskId, 3, 6);
                task_set(taskId, tState, task_get(taskId, tState) + 1);
            } else {
                task_set(taskId, 2, task_get(taskId, 2) - 1);
            }
        }
        2 => {
            if task_get(taskId, 3) == TRUE as i16
                && task_get(taskId, tSlideFinishedCancel) == TRUE as i16
            {
                gPlttBufferFaded[226] = sPokeballGray_Pal[37];
                Swap_PrintActionStrings();
                PutWindowTilemap(SWAP_WIN_ACTION_FADE);
                task_set(taskId, tState, task_get(taskId, tState) + 1);
            }
        }
        3 => {
            BeginNormalPaletteFade(16384, 0, 16, 0, sPokeballGray_Pal[37]);
            task_set(taskId, tState, task_get(taskId, tState) + 1);
        }
        4 => {
            if gPaletteFade.active() == 0 {
                Swap_PrintOneActionString(0);
                task_set(taskId, tState, task_get(taskId, tState) + 1);
            }
        }
        5 => {
            Swap_PrintOneActionString(1);
            PutWindowTilemap(SWAP_WIN_OPTIONS);
            task_set(taskId, tState, task_get(taskId, tState) + 1);
        }
        6 => {
            FillWindowPixelBuffer(SWAP_WIN_ACTION_FADE, 0);
            CopyWindowToVram(SWAP_WIN_ACTION_FADE, COPYWIN_GFX);
            task_set(taskId, tState, task_get(taskId, tState) + 1);
        }
        7 => {
            if (*sFactorySwapScreen).inEnemyScreen == 0 {
                Swap_PrintOnInfoWindow(
                    (*(&raw const crate::data::strings::gText_SelectPkmnToSwap)
                        .cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut(),
                );
            } else {
                Swap_PrintOnInfoWindow(
                    (*(&raw const crate::data::strings::gText_SelectPkmnToAccept)
                        .cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut(),
                );
            }
            if (*sFactorySwapScreen).cursorPos < FRONTIER_PARTY_SIZE as u8 {
                gSprites[(*sFactorySwapScreen).cursorSpriteId].set_invisible(FALSE as u16);
            }
            Swap_PrintMonCategory();
            task_set(taskId, tState, task_get(taskId, tState) + 1);
        }
        8 => {
            Swap_PrintMonSpeciesForTransition();
            Swap_EraseSpeciesAtFadeWindow();
            (*sFactorySwapScreen).fadeSpeciesNameActive = TRUE;
            task_set(taskId, tState, task_get(taskId, tFollowUpTaskState));
            hiPtr = task_get(taskId, tFollowUpTaskPtrHi) as u16;
            loPtr = task_get(taskId, tFollowUpTaskPtrLo) as u16;
            task_set_func(
                taskId,
                core::mem::transmute::<_, Option<unsafe fn(u8)>>(
                    ((hiPtr as i32) << 16 | loPtr as i32) as usize as *mut c_void,
                ),
            );
        }
        _ => {}
    }
}
pub(crate) unsafe fn Swap_Task_SwitchPartyScreen(taskId: u8) {
    if (*sFactorySwapScreen).monPicAnimating == TRUE {
        return;
    }
    match task_get(taskId, tState) {
        0 => {
            Swap_PrintMonSpeciesForTransition();
            task_set(taskId, tState, task_get(taskId, tState) + 1);
        }
        1 => {
            Swap_EraseSpeciesAtFadeWindow();
            gSprites[(*sFactorySwapScreen).cursorSpriteId].set_invisible(TRUE as u16);
            task_set(taskId, tState, task_get(taskId, tState) + 1);
        }
        2 => {
            CreateTask(Some(Swap_Task_SlideCycleBalls), 0);
            task_set_func(
                (*sFactorySwapScreen).fadeSpeciesNameTaskId,
                Some(Swap_Task_FadeOutSpeciesName),
            );
            task_set(taskId, tState, task_get(taskId, tState) + 1);
        }
        3 => {
            if FuncIsActiveTask(Some(Swap_Task_SlideCycleBalls)) == 0
                && task_get(
                    (*sFactorySwapScreen).fadeSpeciesNameTaskId,
                    tFadeOutFinished,
                ) == TRUE as i16
            {
                Swap_EraseSpeciesWindow();
                if (*sFactorySwapScreen).inEnemyScreen == 0 {
                    Swap_InitActions(SWAP_ENEMY_SCREEN);
                } else {
                    Swap_InitActions(SWAP_PLAYER_SCREEN);
                    for i in 0..3u8 {
                        gSprites[(*sFactorySwapScreen).pkmnForSwapButtonSpriteIds[1][i]]
                            .set_invisible(1);
                    }
                }
                gSprites[(*sFactorySwapScreen).cursorSpriteId].x = gSprites
                    [(*sFactorySwapScreen).ballSpriteIds[(*sFactorySwapScreen).cursorPos]]
                    .x;
                task_set_func(
                    (*sFactorySwapScreen).fadeSpeciesNameTaskId,
                    Some(Swap_Task_FadeSpeciesName),
                );
                (*sFactorySwapScreen).fadeSpeciesNameCoeffDelay = 0;
                (*sFactorySwapScreen).fadeSpeciesNameCoeff = 6;
                (*sFactorySwapScreen).fadeSpeciesNameFadeOut = FALSE;
                task_set(
                    (*sFactorySwapScreen).fadeSpeciesNameTaskId,
                    tState,
                    FADESTATE_RUN,
                );
                task_set(taskId, tState, task_get(taskId, tState) + 1);
            }
        }
        4 => {
            task_set(taskId, tState, 0);
            task_set(
                taskId,
                tFollowUpTaskPtrHi,
                (Swap_Task_HandleChooseMons as *const () as usize as u32 >> 16) as i16,
            );
            task_set(
                taskId,
                tFollowUpTaskPtrLo,
                Swap_Task_HandleChooseMons as *const () as usize as u32 as i16,
            );
            task_set(taskId, tFollowUpTaskState, STATE_CHOOSE_MONS_HANDLE_INPUT);
            task_set_func(taskId, Some(Swap_Task_ScreenInfoTransitionIn));
        }
        _ => {}
    }
}
unsafe fn Swap_InitStruct() {
    if sFactorySwapScreen.is_null() {
        sFactorySwapScreen = AllocZeroed(52) as *mut FactorySwapScreen;
        (*sFactorySwapScreen).cursorPos = 0;
        (*sFactorySwapScreen).monPicAnimating = FALSE;
        (*sFactorySwapScreen).fromSummaryScreen = FALSE;
    }
}
pub unsafe fn DoBattleFactorySwapScreen() {
    sFactorySwapScreen = null_mut();
    SetMainCallback2(Some(CB2_InitSwapScreen));
}
pub(crate) unsafe fn CB2_InitSwapScreen() {
    let mut taskId: u8 = 0;
    match gMain.state {
        0 => {
            SetHBlankCallback(None);
            SetVBlankCallback(None);
            {
                {
                    let mut tmp: u32 = 0;
                    volatile_write(&raw mut tmp, 0);
                    CpuSet(
                        &raw mut tmp as *mut c_void,
                        VRAM as usize as *mut c_void,
                        0x5006000,
                    );
                }
            }
            ResetBgsAndClearDma3BusyFlags(0);
            InitBgsFromTemplates(0, sSwap_BgTemplates.as_ptr().cast_mut(), 4);
            InitWindows(sSwap_WindowTemplates.as_ptr().cast_mut());
            DeactivateAllTextPrinters();
            gMain.state += 1;
        }
        1 => {
            sSwapMenuTilesetBuffer = Alloc(1088) as *mut u8;
            sSwapMonPicBgTilesetBuffer = AllocZeroed(1088) as *mut u8;
            sSwapMenuTilemapBuffer = Alloc(BG_SCREEN_SIZE) as *mut u8;
            sSwapMonPicBgTilemapBuffer = AllocZeroed(BG_SCREEN_SIZE) as *mut u8;
            ChangeBgX(0, 0, BG_COORD_SET);
            ChangeBgY(0, 0, BG_COORD_SET);
            ChangeBgX(1, 0, BG_COORD_SET);
            ChangeBgY(1, 0, BG_COORD_SET);
            ChangeBgX(2, 0, BG_COORD_SET);
            ChangeBgY(2, 0, BG_COORD_SET);
            ChangeBgX(3, 0, BG_COORD_SET);
            ChangeBgY(3, 0, BG_COORD_SET);
            SetGpuReg(REG_OFFSET_BLDY, 0);
            SetGpuReg(REG_OFFSET_MOSAIC, 0);
            SetGpuReg(REG_OFFSET_WIN0H, 0);
            SetGpuReg(REG_OFFSET_WIN0V, 0);
            SetGpuReg(REG_OFFSET_WIN1H, 0);
            SetGpuReg(REG_OFFSET_WIN1V, 0);
            SetGpuReg(REG_OFFSET_WININ, 0);
            SetGpuReg(REG_OFFSET_WINOUT, 0);
            gMain.state += 1;
        }
        2 => {
            ResetPaletteFade();
            ResetSpriteData();
            ResetTasks();
            FreeAllSpritePalettes();
            ResetAllPicSprites();
            CpuSet(
                (*(&raw const crate::data::graphics::gFrontierFactoryMenu_Gfx)
                    .cast::<CArray<u16, 544>>())
                .as_ptr()
                .cast_mut() as *mut c_void,
                sSwapMenuTilesetBuffer as *mut c_void,
                544,
            );
            CpuSet(
                sMonPicBg_Gfx.as_ptr().cast_mut() as *mut c_void,
                sSwapMonPicBgTilesetBuffer as *mut c_void,
                48,
            );
            LoadBgTiles(1, sSwapMenuTilesetBuffer as *mut c_void, 1088, 0);
            LoadBgTiles(3, sSwapMonPicBgTilesetBuffer as *mut c_void, 96, 0);
            CpuSet(
                (*(&raw const crate::data::graphics::gFrontierFactoryMenu_Tilemap)
                    .cast::<CArray<u16, 0>>())
                .as_ptr()
                .cast_mut() as *mut c_void,
                sSwapMenuTilemapBuffer as *mut c_void,
                1024,
            );
            LoadBgTilemap(
                1,
                sSwapMenuTilemapBuffer as *mut c_void,
                BG_SCREEN_SIZE as u16,
                0,
            );
            LoadPalette(
                (*(&raw const crate::data::graphics::gFrontierFactoryMenu_Pal)
                    .cast::<CArray<u16, 0>>())
                .as_ptr()
                .cast_mut() as *mut c_void,
                0,
                64,
            );
            LoadPalette(sSwapText_Pal.as_ptr().cast_mut() as *mut c_void, 240, 10);
            LoadPalette(sSwapText_Pal.as_ptr().cast_mut() as *mut c_void, 224, 10);
            LoadPalette(sMonPicBg_Pal.as_ptr().cast_mut() as *mut c_void, 32, 4);
            gMain.state += 1;
        }
        3 => {
            SetBgTilemapBuffer(3, sSwapMonPicBgTilemapBuffer as *mut c_void);
            CopyToBgTilemapBufferRect(
                3,
                sMonPicBg_Tilemap.as_ptr().cast_mut() as *mut c_void,
                11,
                4,
                8,
                8,
            );
            CopyBgTilemapBufferToVram(3);
            gMain.state += 1;
        }
        4 => {
            LoadSpritePalettes(sSwap_SpritePalettes.as_ptr().cast_mut());
            LoadSpriteSheets(sSwap_SpriteSheets.as_ptr().cast_mut());
            LoadCompressedSpriteSheet(sSwap_BallGfx.as_ptr().cast_mut());
            SetVBlankCallback(Some(Swap_VblankCb));
            gMain.state += 1;
        }
        5 => {
            if !sFactorySwapScreen.is_null() && (*sFactorySwapScreen).fromSummaryScreen != 0 {
                (*sFactorySwapScreen).cursorPos = gLastViewedMonIndex;
            }
            gMain.state += 1;
        }
        6 => {
            Swap_InitStruct();
            Swap_InitAllSprites();
            if (*sFactorySwapScreen).fromSummaryScreen == TRUE {
                Swap_ShowSummaryMonSprite();
            }
            Swap_InitActions(SWAP_PLAYER_SCREEN);
            gMain.state += 1;
        }
        7 => {
            Swap_PrintOnInfoWindow(
                (*(&raw const crate::data::strings::gText_SelectPkmnToSwap)
                    .cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
            );
            PutWindowTilemap(SWAP_WIN_INFO);
            gMain.state += 1;
        }
        8 => {
            Swap_PrintMonCategory();
            PutWindowTilemap(SWAP_WIN_MON_CATEGORY);
            gMain.state += 1;
        }
        9 => {
            if (*sFactorySwapScreen).fromSummaryScreen == 0 {
                Swap_PrintMonSpecies();
            }
            PutWindowTilemap(SWAP_WIN_SPECIES);
            gMain.state += 1;
        }
        10 => {
            Swap_PrintPkmnSwap();
            PutWindowTilemap(SWAP_WIN_TITLE);
            gMain.state += 1;
        }
        11 => {
            gMain.state += 1;
        }
        12 => {
            if (*sFactorySwapScreen).fromSummaryScreen != 0 {
                Swap_PrintMonSpeciesAtFade();
            }
            gMain.state += 1;
        }
        13 => {
            Swap_PrintActionStrings2();
            PutWindowTilemap(SWAP_WIN_OPTIONS);
            gMain.state += 1;
        }
        14 => {
            BeginNormalPaletteFade(PALETTES_ALL, 0, 16, 0, 0);
            SetGpuReg(REG_OFFSET_DISPCNT, 4160);
            ShowBg(0);
            ShowBg(1);
            ShowBg(2);
            if (*sFactorySwapScreen).fromSummaryScreen == TRUE {
                ShowBg(3);
                SetGpuReg(REG_OFFSET_BLDCNT, 4680);
                SetGpuReg(REG_OFFSET_BLDALPHA, 1035);
            } else {
                HideBg(3);
            }
            gMain.state += 1;
        }
        15 => {
            (*sFactorySwapScreen).fadeSpeciesNameTaskId =
                CreateTask(Some(Swap_Task_FadeSpeciesName), 0);
            if (*sFactorySwapScreen).fromSummaryScreen == 0 {
                task_set(
                    (*sFactorySwapScreen).fadeSpeciesNameTaskId,
                    tState,
                    FADESTATE_INIT,
                );
                taskId = CreateTask(Some(Swap_Task_HandleChooseMons), 0);
                task_set(taskId, tState, STATE_CHOOSE_MONS_INIT);
            } else {
                Swap_EraseActionFadeWindow();
                task_set(
                    (*sFactorySwapScreen).fadeSpeciesNameTaskId,
                    tState,
                    FADESTATE_RUN,
                );
                (*sFactorySwapScreen).fadeSpeciesNameActive = FALSE;
                taskId = CreateTask(Some(Swap_Task_HandleMenu), 0);
                task_set(taskId, tState, STATE_MENU_INIT);
            }
            SetMainCallback2(Some(Swap_CB2));
        }
        _ => {}
    }
}
unsafe fn Swap_InitAllSprites() {
    let mut x: u8 = 0;
    let mut spriteTemplate: SpriteTemplate = *sSpriteTemplate_Swap_Pokeball;
    spriteTemplate.paletteTag = PALTAG_BALL_SELECTED;
    for i in 0..(FRONTIER_PARTY_SIZE as u8) {
        (*sFactorySwapScreen).ballSpriteIds[i] =
            CreateSprite(&raw mut spriteTemplate, 48 * i as i16 + 72, 64, 1);
        gSprites[(*sFactorySwapScreen).ballSpriteIds[i]].data[0] = 0;
    }
    (*sFactorySwapScreen).cursorSpriteId = CreateSprite(
        (&raw const *sSpriteTemplate_Swap_Arrow).cast_mut(),
        gSprites[(*sFactorySwapScreen).ballSpriteIds[(*sFactorySwapScreen).cursorPos]].x,
        88,
        0,
    );
    (*sFactorySwapScreen).menuCursor1SpriteId = CreateSprite(
        (&raw const *sSpriteTemplate_Swap_MenuHighlightLeft).cast_mut(),
        176,
        112,
        0,
    );
    (*sFactorySwapScreen).menuCursor2SpriteId = CreateSprite(
        (&raw const *sSpriteTemplate_Swap_MenuHighlightRight).cast_mut(),
        176,
        144,
        0,
    );
    gSprites[(*sFactorySwapScreen).menuCursor1SpriteId].set_invisible(TRUE as u16);
    gSprites[(*sFactorySwapScreen).menuCursor2SpriteId].set_invisible(TRUE as u16);
    gSprites[(*sFactorySwapScreen).menuCursor1SpriteId].centerToCornerVecX = 0;
    gSprites[(*sFactorySwapScreen).menuCursor1SpriteId].centerToCornerVecY = 0;
    gSprites[(*sFactorySwapScreen).menuCursor2SpriteId].centerToCornerVecX = 0;
    gSprites[(*sFactorySwapScreen).menuCursor2SpriteId].centerToCornerVecY = 0;
    if (*sFactorySwapScreen).fromSummaryScreen == TRUE {
        x = DISPLAY_WIDTH as u8;
    } else {
        x = 192;
    }
    spriteTemplate = *sSpriteTemplate_Swap_Arrow;
    spriteTemplate.tileTag = GFXTAG_ACTION_BOX_LEFT;
    (*sFactorySwapScreen).pkmnForSwapButtonSpriteIds[0][0] =
        CreateSprite(&raw mut spriteTemplate, DISPLAY_WIDTH as i16, 120, 10);
    spriteTemplate = *sSpriteTemplate_Swap_MenuHighlightLeft;
    spriteTemplate.tileTag = GFXTAG_ACTION_BOX_RIGHT;
    (*sFactorySwapScreen).pkmnForSwapButtonSpriteIds[0][1] =
        CreateSprite(&raw mut spriteTemplate, 256, 120, 10);
    (*sFactorySwapScreen).pkmnForSwapButtonSpriteIds[0][2] =
        CreateSprite(&raw mut spriteTemplate, 288, 120, 10);
    spriteTemplate = *sSpriteTemplate_Swap_Arrow;
    spriteTemplate.tileTag = GFXTAG_ACTION_HIGHLIGHT_LEFT;
    (*sFactorySwapScreen).pkmnForSwapButtonSpriteIds[1][0] =
        CreateSprite(&raw mut spriteTemplate, DISPLAY_WIDTH as i16, 120, 1);
    spriteTemplate = *sSpriteTemplate_Swap_MenuHighlightLeft;
    spriteTemplate.tileTag = GFXTAG_ACTION_HIGHLIGHT_MIDDLE;
    (*sFactorySwapScreen).pkmnForSwapButtonSpriteIds[1][1] =
        CreateSprite(&raw mut spriteTemplate, 256, 120, 1);
    spriteTemplate.tileTag = GFXTAG_ACTION_HIGHLIGHT_RIGHT;
    (*sFactorySwapScreen).pkmnForSwapButtonSpriteIds[1][2] =
        CreateSprite(&raw mut spriteTemplate, 288, 120, 1);
    spriteTemplate = *sSpriteTemplate_Swap_Arrow;
    spriteTemplate.tileTag = GFXTAG_ACTION_BOX_LEFT;
    (*sFactorySwapScreen).cancelButtonSpriteIds[0][0] =
        CreateSprite(&raw mut spriteTemplate, x as i16, 144, 10);
    spriteTemplate = *sSpriteTemplate_Swap_MenuHighlightLeft;
    spriteTemplate.tileTag = GFXTAG_ACTION_BOX_RIGHT;
    (*sFactorySwapScreen).cancelButtonSpriteIds[0][1] =
        CreateSprite(&raw mut spriteTemplate, x as i16 + 16, 144, 10);
    spriteTemplate = *sSpriteTemplate_Swap_Arrow;
    spriteTemplate.tileTag = GFXTAG_ACTION_HIGHLIGHT_LEFT;
    (*sFactorySwapScreen).cancelButtonSpriteIds[1][0] =
        CreateSprite(&raw mut spriteTemplate, x as i16, 144, 1);
    spriteTemplate = *sSpriteTemplate_Swap_MenuHighlightLeft;
    spriteTemplate.tileTag = GFXTAG_ACTION_HIGHLIGHT_RIGHT;
    (*sFactorySwapScreen).cancelButtonSpriteIds[1][1] =
        CreateSprite(&raw mut spriteTemplate, x as i16 + 16, 144, 1);
    for i in 0..2u8 {
        gSprites[(*sFactorySwapScreen).pkmnForSwapButtonSpriteIds[i][0]].centerToCornerVecX = 0;
        gSprites[(*sFactorySwapScreen).pkmnForSwapButtonSpriteIds[i][0]].centerToCornerVecY = 0;
        gSprites[(*sFactorySwapScreen).pkmnForSwapButtonSpriteIds[i][1]].centerToCornerVecX = 0;
        gSprites[(*sFactorySwapScreen).pkmnForSwapButtonSpriteIds[i][1]].centerToCornerVecY = 0;
        gSprites[(*sFactorySwapScreen).pkmnForSwapButtonSpriteIds[i][2]].centerToCornerVecX = 0;
        gSprites[(*sFactorySwapScreen).pkmnForSwapButtonSpriteIds[i][2]].centerToCornerVecY = 0;
        gSprites[(*sFactorySwapScreen).cancelButtonSpriteIds[i][0]].centerToCornerVecX = 0;
        gSprites[(*sFactorySwapScreen).cancelButtonSpriteIds[i][0]].centerToCornerVecY = 0;
        gSprites[(*sFactorySwapScreen).cancelButtonSpriteIds[i][1]].centerToCornerVecX = 0;
        gSprites[(*sFactorySwapScreen).cancelButtonSpriteIds[i][1]].centerToCornerVecY = 0;
        gSprites[(*sFactorySwapScreen).pkmnForSwapButtonSpriteIds[i][0]].set_invisible(TRUE as u16);
        gSprites[(*sFactorySwapScreen).pkmnForSwapButtonSpriteIds[i][1]].set_invisible(1);
        gSprites[(*sFactorySwapScreen).pkmnForSwapButtonSpriteIds[i][2]].set_invisible(TRUE as u16);
        gSprites[(*sFactorySwapScreen).cancelButtonSpriteIds[i][0]].set_invisible(TRUE as u16);
        gSprites[(*sFactorySwapScreen).cancelButtonSpriteIds[i][1]].set_invisible(1);
    }
    gSprites[(*sFactorySwapScreen).cancelButtonSpriteIds[0][0]].set_invisible(0);
    gSprites[(*sFactorySwapScreen).cancelButtonSpriteIds[0][1]].set_invisible(0);
    gSprites[(*sFactorySwapScreen).pkmnForSwapButtonSpriteIds[0][0]].set_invisible(0);
    gSprites[(*sFactorySwapScreen).pkmnForSwapButtonSpriteIds[0][1]].set_invisible(0);
    gSprites[(*sFactorySwapScreen).pkmnForSwapButtonSpriteIds[0][2]].set_invisible(0);
}
unsafe fn Swap_DestroyAllSprites() {
    for i in 0..(FRONTIER_PARTY_SIZE as u8) {
        DestroySprite(&raw mut gSprites[(*sFactorySwapScreen).ballSpriteIds[i]]);
    }
    DestroySprite(&raw mut gSprites[(*sFactorySwapScreen).cursorSpriteId]);
    DestroySprite(&raw mut gSprites[(*sFactorySwapScreen).menuCursor1SpriteId]);
    DestroySprite(&raw mut gSprites[(*sFactorySwapScreen).menuCursor2SpriteId]);
    let mut i: u8 = 0;
    while i < 2 {
        for j in 0..3u8 {
            DestroySprite(
                &raw mut gSprites[(*sFactorySwapScreen).pkmnForSwapButtonSpriteIds[i][j]],
            );
        }
        i += 1;
    }
    for i in 0..2u8 {
        for j in 0..2u8 {
            DestroySprite(&raw mut gSprites[(*sFactorySwapScreen).cancelButtonSpriteIds[i][j]]);
        }
    }
}
unsafe fn Swap_HandleActionCursorChange(cursorId: u8) {
    if cursorId < FRONTIER_PARTY_SIZE as u8 {
        gSprites[(*sFactorySwapScreen).cursorSpriteId].set_invisible(FALSE as u16);
        Swap_HideActionButtonHighlights();
        gSprites[(*sFactorySwapScreen).cursorSpriteId].x =
            gSprites[(*sFactorySwapScreen).ballSpriteIds[cursorId]].x;
    } else {
        gSprites[(*sFactorySwapScreen).cursorSpriteId].set_invisible(TRUE as u16);
        Swap_HighlightActionButton((*(*sFactorySwapScreen).actionsData.at(cursorId)).id);
    }
}
unsafe fn Swap_UpdateBallCursorPosition(direction: i8) {
    let mut cursorPos: u8 = 0;
    PlaySE(SE_SELECT);
    if direction > 0 {
        if (*sFactorySwapScreen).cursorPos as i32 + 1 != (*sFactorySwapScreen).actionsCount as i32 {
            (*sFactorySwapScreen).cursorPos += 1;
        } else {
            (*sFactorySwapScreen).cursorPos = 0;
        }
    } else {
        if (*sFactorySwapScreen).cursorPos != 0 {
            (*sFactorySwapScreen).cursorPos -= 1;
        } else {
            (*sFactorySwapScreen).cursorPos = (*sFactorySwapScreen).actionsCount - 1;
        }
    }
    cursorPos = (*sFactorySwapScreen).cursorPos;
    Swap_HandleActionCursorChange(cursorPos);
}
unsafe fn Swap_UpdateActionCursorPosition(direction: i8) {
    let mut cursorPos: u8 = 0;
    PlaySE(SE_SELECT);
    if direction > 0 {
        if (*sFactorySwapScreen).cursorPos < FRONTIER_PARTY_SIZE as u8 {
            (*sFactorySwapScreen).cursorPos = FRONTIER_PARTY_SIZE as u8;
        } else if (*sFactorySwapScreen).cursorPos as i32 + 1
            != (*sFactorySwapScreen).actionsCount as i32
        {
            (*sFactorySwapScreen).cursorPos += 1;
        } else {
            (*sFactorySwapScreen).cursorPos = 0;
        }
    } else {
        if (*sFactorySwapScreen).cursorPos < FRONTIER_PARTY_SIZE as u8 {
            (*sFactorySwapScreen).cursorPos = (*sFactorySwapScreen).actionsCount - 1;
        } else if (*sFactorySwapScreen).cursorPos != 0 {
            (*sFactorySwapScreen).cursorPos -= 1;
        } else {
            (*sFactorySwapScreen).cursorPos = (*sFactorySwapScreen).actionsCount - 1;
        }
    }
    cursorPos = (*sFactorySwapScreen).cursorPos;
    Swap_HandleActionCursorChange(cursorPos);
}
unsafe fn Swap_UpdateYesNoCursorPosition(direction: i8) {
    if direction > 0 {
        if (*sFactorySwapScreen).yesNoCursorPos != 1 {
            (*sFactorySwapScreen).yesNoCursorPos += 1;
        } else {
            (*sFactorySwapScreen).yesNoCursorPos = 0;
        }
    } else {
        if (*sFactorySwapScreen).yesNoCursorPos != 0 {
            (*sFactorySwapScreen).yesNoCursorPos -= 1;
        } else {
            (*sFactorySwapScreen).yesNoCursorPos = 1;
        }
    }
    gSprites[(*sFactorySwapScreen).menuCursor1SpriteId].y =
        (*sFactorySwapScreen).yesNoCursorPos as i16 * 16 + 112;
    gSprites[(*sFactorySwapScreen).menuCursor2SpriteId].y =
        (*sFactorySwapScreen).yesNoCursorPos as i16 * 16 + 112;
}
unsafe fn Swap_UpdateMenuCursorPosition(direction: i8) {
    PlaySE(SE_SELECT);
    if direction > 0 {
        if (*sFactorySwapScreen).menuCursorPos != 2 {
            (*sFactorySwapScreen).menuCursorPos += 1;
        } else {
            (*sFactorySwapScreen).menuCursorPos = 0;
        }
    } else {
        if (*sFactorySwapScreen).menuCursorPos != 0 {
            (*sFactorySwapScreen).menuCursorPos -= 1;
        } else {
            (*sFactorySwapScreen).menuCursorPos = 2;
        }
    }
    gSprites[(*sFactorySwapScreen).menuCursor1SpriteId].y =
        (*sFactorySwapScreen).menuCursorPos as i16 * 16 + 112;
    gSprites[(*sFactorySwapScreen).menuCursor2SpriteId].y =
        (*sFactorySwapScreen).menuCursorPos as i16 * 16 + 112;
}
unsafe fn Swap_HighlightActionButton(actionId: u8) {
    for i in 0..3u8 {
        if actionId == SWAPACTION_PKMN_FOR_SWAP {
            gSprites[(*sFactorySwapScreen).pkmnForSwapButtonSpriteIds[1][i]]
                .set_invisible(FALSE as u16);
            if i < 2 {
                gSprites[(*sFactorySwapScreen).cancelButtonSpriteIds[1][i]].set_invisible(1);
            }
        } else if actionId == SWAPACTION_CANCEL {
            if i < 2 {
                gSprites[(*sFactorySwapScreen).cancelButtonSpriteIds[1][i]]
                    .set_invisible(FALSE as u16);
            }
            gSprites[(*sFactorySwapScreen).pkmnForSwapButtonSpriteIds[1][i]].set_invisible(1);
        }
    }
}
unsafe fn Swap_HideActionButtonHighlights() {
    for i in 0..3u8 {
        gSprites[(*sFactorySwapScreen).pkmnForSwapButtonSpriteIds[1][i]].set_invisible(1);
        if i < 2 {
            gSprites[(*sFactorySwapScreen).cancelButtonSpriteIds[1][i]].set_invisible(1);
        }
    }
}
unsafe fn Swap_ShowMenuOptions() {
    if (*sFactorySwapScreen).fromSummaryScreen == TRUE {
        (*sFactorySwapScreen).fromSummaryScreen = FALSE;
    } else {
        (*sFactorySwapScreen).menuCursorPos = 0;
    }
    gSprites[(*sFactorySwapScreen).menuCursor1SpriteId].x = 176;
    gSprites[(*sFactorySwapScreen).menuCursor1SpriteId].y =
        (*sFactorySwapScreen).menuCursorPos as i16 * 16 + 112;
    gSprites[(*sFactorySwapScreen).menuCursor2SpriteId].x = 208;
    gSprites[(*sFactorySwapScreen).menuCursor2SpriteId].y =
        (*sFactorySwapScreen).menuCursorPos as i16 * 16 + 112;
    gSprites[(*sFactorySwapScreen).menuCursor1SpriteId].set_invisible(FALSE as u16);
    gSprites[(*sFactorySwapScreen).menuCursor2SpriteId].set_invisible(FALSE as u16);
    Swap_PrintMenuOptions();
}
unsafe fn Swap_ShowYesNoOptions() {
    (*sFactorySwapScreen).yesNoCursorPos = 0;
    gSprites[(*sFactorySwapScreen).menuCursor1SpriteId].x = 176;
    gSprites[(*sFactorySwapScreen).menuCursor1SpriteId].y = 112;
    gSprites[(*sFactorySwapScreen).menuCursor2SpriteId].x = 208;
    gSprites[(*sFactorySwapScreen).menuCursor2SpriteId].y = 112;
    gSprites[(*sFactorySwapScreen).menuCursor1SpriteId].set_invisible(FALSE as u16);
    gSprites[(*sFactorySwapScreen).menuCursor2SpriteId].set_invisible(FALSE as u16);
    Swap_PrintYesNoOptions();
}
unsafe fn Swap_ErasePopupMenu(windowId: u8) {
    gSprites[(*sFactorySwapScreen).menuCursor1SpriteId].set_invisible(TRUE as u16);
    gSprites[(*sFactorySwapScreen).menuCursor2SpriteId].set_invisible(TRUE as u16);
    FillWindowPixelBuffer(windowId, 0);
    CopyWindowToVram(windowId, COPYWIN_GFX);
    ClearWindowTilemap(windowId);
}
unsafe fn Swap_EraseSpeciesWindow() {
    PutWindowTilemap(SWAP_WIN_SPECIES);
    FillWindowPixelBuffer(SWAP_WIN_SPECIES, 0);
    CopyWindowToVram(SWAP_WIN_SPECIES, COPYWIN_GFX);
}
unsafe fn Swap_EraseSpeciesAtFadeWindow() {
    PutWindowTilemap(SWAP_WIN_SPECIES_AT_FADE);
    FillWindowPixelBuffer(SWAP_WIN_SPECIES_AT_FADE, 0);
    CopyWindowToVram(SWAP_WIN_SPECIES_AT_FADE, COPYWIN_GFX);
}
unsafe fn Swap_EraseActionFadeWindow() {
    Swap_EraseSpeciesWindow();
    PutWindowTilemap(SWAP_WIN_ACTION_FADE);
    FillWindowPixelBuffer(SWAP_WIN_ACTION_FADE, 0);
    CopyWindowToVram(SWAP_WIN_ACTION_FADE, COPYWIN_GFX);
}
unsafe fn Swap_PrintPkmnSwap() {
    FillWindowPixelBuffer(SWAP_WIN_TITLE, 17);
    AddTextPrinterParameterized(
        SWAP_WIN_TITLE,
        FONT_NORMAL,
        (*(&raw const crate::data::strings::gText_PkmnSwap).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        2,
        1,
        0,
        None,
    );
    CopyWindowToVram(SWAP_WIN_TITLE, COPYWIN_FULL);
}
unsafe fn Swap_PrintMonSpecies() {
    let mut species: u16 = 0;
    let mut x: u8 = 0;
    FillWindowPixelBuffer(SWAP_WIN_SPECIES, 0);
    if (*sFactorySwapScreen).cursorPos >= FRONTIER_PARTY_SIZE as u8 {
        CopyWindowToVram(SWAP_WIN_SPECIES, COPYWIN_GFX);
    } else {
        let monId: u8 = (*sFactorySwapScreen).cursorPos;
        if (*sFactorySwapScreen).inEnemyScreen == 0 {
            species =
                GetMonData3(&raw mut gPlayerParty[monId], MON_DATA_SPECIES, null_mut()) as u16;
        } else {
            species = GetMonData3(&raw mut gEnemyParty[monId], MON_DATA_SPECIES, null_mut()) as u16;
        }
        StringCopy(
            gStringVar4.as_mut_ptr(),
            (*(&raw const crate::data::data_tables::gSpeciesNames)
                .cast::<CArray<CArray<u8, 11>, 0>>())[species]
                .as_ptr()
                .cast_mut(),
        );
        x = GetStringRightAlignXOffset(FONT_NORMAL as i32, gStringVar4.as_mut_ptr(), 86) as u8;
        AddTextPrinterParameterized3(
            SWAP_WIN_SPECIES,
            FONT_NORMAL,
            x,
            1,
            sSwapSpeciesNameTextColors.as_ptr().cast_mut(),
            0,
            gStringVar4.as_mut_ptr(),
        );
        CopyWindowToVram(SWAP_WIN_SPECIES, COPYWIN_FULL);
    }
}
unsafe fn Swap_PrintOnInfoWindow(str: *mut u8) {
    FillWindowPixelBuffer(SWAP_WIN_INFO, 0);
    AddTextPrinterParameterized(SWAP_WIN_INFO, FONT_NORMAL, str, 2, 5, 0, None);
    CopyWindowToVram(SWAP_WIN_INFO, COPYWIN_GFX);
}
unsafe fn Swap_PrintMenuOptions() {
    PutWindowTilemap(SWAP_WIN_OPTIONS);
    FillWindowPixelBuffer(SWAP_WIN_OPTIONS, 0);
    AddTextPrinterParameterized3(
        SWAP_WIN_OPTIONS,
        FONT_NORMAL,
        15,
        1,
        sSwapMenuOptionsTextColors.as_ptr().cast_mut(),
        0,
        (*(&raw const crate::data::strings::gText_Summary2).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
    );
    AddTextPrinterParameterized3(
        SWAP_WIN_OPTIONS,
        FONT_NORMAL,
        15,
        17,
        sSwapMenuOptionsTextColors.as_ptr().cast_mut(),
        0,
        (*(&raw const crate::data::strings::gText_Swap).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
    );
    AddTextPrinterParameterized3(
        SWAP_WIN_OPTIONS,
        FONT_NORMAL,
        15,
        33,
        sSwapMenuOptionsTextColors.as_ptr().cast_mut(),
        0,
        (*(&raw const crate::data::strings::gText_Rechoose).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
    );
    CopyWindowToVram(SWAP_WIN_OPTIONS, COPYWIN_FULL);
}
unsafe fn Swap_PrintYesNoOptions() {
    PutWindowTilemap(SWAP_WIN_YES_NO);
    FillWindowPixelBuffer(SWAP_WIN_YES_NO, 0);
    AddTextPrinterParameterized3(
        SWAP_WIN_YES_NO,
        FONT_NORMAL,
        7,
        1,
        sSwapMenuOptionsTextColors.as_ptr().cast_mut(),
        0,
        (*(&raw const crate::data::strings::gText_Yes3).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
    );
    AddTextPrinterParameterized3(
        SWAP_WIN_YES_NO,
        FONT_NORMAL,
        7,
        17,
        sSwapMenuOptionsTextColors.as_ptr().cast_mut(),
        0,
        (*(&raw const crate::data::strings::gText_No3).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
    );
    CopyWindowToVram(SWAP_WIN_YES_NO, COPYWIN_FULL);
}
unsafe fn Swap_PrintActionString(str: *mut u8, y: u32, windowId: u32) {
    let x: i32 = GetStringRightAlignXOffset(FONT_SMALL as i32, str, 70);
    AddTextPrinterParameterized3(
        windowId as u8,
        FONT_SMALL,
        x as u8,
        y as u8,
        sSwapMenuOptionsTextColors.as_ptr().cast_mut(),
        0,
        str,
    );
}
unsafe fn Swap_PrintActionStrings() {
    FillWindowPixelBuffer(SWAP_WIN_ACTION_FADE, 0);
    'l1: {
        let sw1: u8 = (*sFactorySwapScreen).inEnemyScreen;
        let mut fall = false;
        if sw1 == TRUE {
            fall = true;
            Swap_PrintActionString(
                (*(&raw const crate::data::strings::gText_PkmnForSwap).cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut(),
                0,
                SWAP_WIN_ACTION_FADE as u32,
            );
        }
        if fall || sw1 == FALSE {
            Swap_PrintActionString(
                (*(&raw const crate::data::strings::gText_Cancel3).cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut(),
                24,
                SWAP_WIN_ACTION_FADE as u32,
            );
            break 'l1;
        }
    }
    CopyWindowToVram(SWAP_WIN_ACTION_FADE, COPYWIN_FULL);
}
unsafe fn Swap_PrintActionStrings2() {
    FillWindowPixelBuffer(SWAP_WIN_OPTIONS, 0);
    'l1: {
        let sw1: u8 = (*sFactorySwapScreen).inEnemyScreen;
        let mut fall = false;
        if sw1 == TRUE {
            fall = true;
            Swap_PrintActionString(
                (*(&raw const crate::data::strings::gText_PkmnForSwap).cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut(),
                8,
                SWAP_WIN_OPTIONS as u32,
            );
        }
        if fall || sw1 == FALSE {
            Swap_PrintActionString(
                (*(&raw const crate::data::strings::gText_Cancel3).cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut(),
                32,
                SWAP_WIN_OPTIONS as u32,
            );
            break 'l1;
        }
    }
    CopyWindowToVram(SWAP_WIN_OPTIONS, COPYWIN_FULL);
}
unsafe fn Swap_PrintOneActionString(which: u8) {
    match which {
        0 => {
            if (*sFactorySwapScreen).inEnemyScreen == TRUE {
                Swap_PrintActionString(
                    (*(&raw const crate::data::strings::gText_PkmnForSwap).cast::<CArray<u8, 0>>())
                        .as_ptr()
                        .cast_mut(),
                    8,
                    SWAP_WIN_OPTIONS as u32,
                );
            }
        }
        1 => {
            Swap_PrintActionString(
                (*(&raw const crate::data::strings::gText_Cancel3).cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut(),
                32,
                SWAP_WIN_OPTIONS as u32,
            );
        }
        _ => {}
    }
    CopyWindowToVram(SWAP_WIN_OPTIONS, COPYWIN_FULL);
}
unsafe fn Swap_PrintMonSpeciesAtFade() {
    let mut species: u16 = 0;
    let mut x: u8 = 0;
    let mut pal: CArray<u16, 5> = zeroed();
    CpuSet(
        sSwapText_Pal.as_ptr().cast_mut() as *mut c_void,
        pal.as_mut_ptr() as *mut c_void,
        4,
    );
    if (*sFactorySwapScreen).fromSummaryScreen == 0 {
        pal[4] = gPlttBufferFaded[228];
    } else {
        pal[4] = (*sFactorySwapScreen).speciesNameColorBackup;
    }
    LoadPalette(pal.as_mut_ptr() as *mut c_void, 240, 10);
    PutWindowTilemap(SWAP_WIN_SPECIES_AT_FADE);
    FillWindowPixelBuffer(SWAP_WIN_SPECIES_AT_FADE, 0);
    if (*sFactorySwapScreen).cursorPos >= FRONTIER_PARTY_SIZE as u8 {
        CopyWindowToVram(SWAP_WIN_SPECIES_AT_FADE, COPYWIN_FULL);
    } else {
        let monId: u8 = (*sFactorySwapScreen).cursorPos;
        if (*sFactorySwapScreen).inEnemyScreen == 0 {
            species =
                GetMonData3(&raw mut gPlayerParty[monId], MON_DATA_SPECIES, null_mut()) as u16;
        } else {
            species = GetMonData3(&raw mut gEnemyParty[monId], MON_DATA_SPECIES, null_mut()) as u16;
        }
        StringCopy(
            gStringVar4.as_mut_ptr(),
            (*(&raw const crate::data::data_tables::gSpeciesNames)
                .cast::<CArray<CArray<u8, 11>, 0>>())[species]
                .as_ptr()
                .cast_mut(),
        );
        x = GetStringRightAlignXOffset(FONT_NORMAL as i32, gStringVar4.as_mut_ptr(), 86) as u8;
        AddTextPrinterParameterized3(
            SWAP_WIN_SPECIES_AT_FADE,
            FONT_NORMAL,
            x,
            1,
            sSwapSpeciesNameTextColors.as_ptr().cast_mut(),
            0,
            gStringVar4.as_mut_ptr(),
        );
        CopyWindowToVram(SWAP_WIN_SPECIES_AT_FADE, COPYWIN_FULL);
    }
}
unsafe fn Swap_PrintMonSpeciesForTransition() {
    let mut species: u16 = 0;
    let mut x: u8 = 0;
    LoadPalette(sSwapText_Pal.as_ptr().cast_mut() as *mut c_void, 224, 10);
    CpuSet(
        &raw mut gPlttBufferUnfaded[240] as *mut c_void,
        &raw mut gPlttBufferFaded[224] as *mut c_void,
        5,
    );
    if (*sFactorySwapScreen).cursorPos >= FRONTIER_PARTY_SIZE as u8 {
        CopyWindowToVram(SWAP_WIN_SPECIES, COPYWIN_GFX);
    } else {
        let monId: u8 = (*sFactorySwapScreen).cursorPos;
        if (*sFactorySwapScreen).inEnemyScreen == 0 {
            species =
                GetMonData3(&raw mut gPlayerParty[monId], MON_DATA_SPECIES, null_mut()) as u16;
        } else {
            species = GetMonData3(&raw mut gEnemyParty[monId], MON_DATA_SPECIES, null_mut()) as u16;
        }
        StringCopy(
            gStringVar4.as_mut_ptr(),
            (*(&raw const crate::data::data_tables::gSpeciesNames)
                .cast::<CArray<CArray<u8, 11>, 0>>())[species]
                .as_ptr()
                .cast_mut(),
        );
        x = GetStringRightAlignXOffset(FONT_NORMAL as i32, gStringVar4.as_mut_ptr(), 86) as u8;
        AddTextPrinterParameterized3(
            SWAP_WIN_SPECIES,
            FONT_NORMAL,
            x,
            1,
            sSwapSpeciesNameTextColors.as_ptr().cast_mut(),
            0,
            gStringVar4.as_mut_ptr(),
        );
        CopyWindowToVram(SWAP_WIN_SPECIES, COPYWIN_FULL);
    }
}
unsafe fn Swap_PrintMonCategory() {
    let mut species: u16 = 0;
    let mut text: CArray<u8, 30> = zeroed();
    let mut x: u8 = 0;
    let monId: u8 = (*sFactorySwapScreen).cursorPos;
    FillWindowPixelBuffer(SWAP_WIN_MON_CATEGORY, 0);
    if monId >= FRONTIER_PARTY_SIZE as u8 {
        CopyWindowToVram(SWAP_WIN_MON_CATEGORY, COPYWIN_GFX);
    } else {
        PutWindowTilemap(SWAP_WIN_MON_CATEGORY);
        if (*sFactorySwapScreen).inEnemyScreen == 0 {
            species =
                GetMonData3(&raw mut gPlayerParty[monId], MON_DATA_SPECIES, null_mut()) as u16;
        } else {
            species = GetMonData3(&raw mut gEnemyParty[monId], MON_DATA_SPECIES, null_mut()) as u16;
        }
        CopyMonCategoryText(
            SpeciesToNationalPokedexNum(species) as i32,
            text.as_mut_ptr(),
        );
        x = GetStringRightAlignXOffset(FONT_NORMAL as i32, text.as_mut_ptr(), 118) as u8;
        AddTextPrinterParameterized(
            SWAP_WIN_MON_CATEGORY,
            FONT_NORMAL,
            text.as_mut_ptr(),
            x,
            1,
            0,
            None,
        );
        CopyWindowToVram(SWAP_WIN_MON_CATEGORY, COPYWIN_GFX);
    }
}
unsafe fn Swap_InitActions(id: u8) {
    if (*sFactorySwapScreen).fromSummaryScreen != TRUE {
        match id {
            SWAP_PLAYER_SCREEN => {
                (*sFactorySwapScreen).inEnemyScreen = FALSE;
                (*sFactorySwapScreen).cursorPos = 0;
                (*sFactorySwapScreen).actionsCount = 4;
                (*sFactorySwapScreen).actionsData = sSwap_PlayerScreenActions.as_ptr().cast_mut();
            }
            SWAP_ENEMY_SCREEN => {
                (*sFactorySwapScreen).inEnemyScreen = TRUE;
                (*sFactorySwapScreen).cursorPos = 0;
                (*sFactorySwapScreen).actionsCount = 5;
                (*sFactorySwapScreen).actionsData = sSwap_EnemyScreenActions.as_ptr().cast_mut();
            }
            _ => {}
        }
    }
}
unsafe fn Swap_RunMenuOptionFunc(taskId: u8) {
    sSwap_CurrentOptionFunc = sSwap_MenuOptionFuncs[(*sFactorySwapScreen).menuCursorPos];
    sSwap_CurrentOptionFunc.unwrap_unchecked()(taskId);
}
pub(crate) unsafe fn Swap_OptionSwap(taskId: u8) {
    CloseMonPic(
        (*sFactorySwapScreen).monPic,
        &raw mut (*sFactorySwapScreen).monPicAnimating,
        TRUE,
    );
    (*sFactorySwapScreen).playerMonId = (*sFactorySwapScreen).cursorPos;
    Swap_ErasePopupMenu(SWAP_WIN_OPTIONS);
    task_set(taskId, tState, 0);
    task_set_func(taskId, Some(Swap_Task_SwitchPartyScreen));
}
pub(crate) fn Swap_OptionSummary(taskId: u8) {
    task_set(taskId, tState, STATE_SUMMARY_FADE);
    task_set_func(taskId, Some(Swap_Task_OpenSummaryScreen));
}
pub(crate) unsafe fn Swap_OptionRechoose(taskId: u8) {
    CloseMonPic(
        (*sFactorySwapScreen).monPic,
        &raw mut (*sFactorySwapScreen).monPicAnimating,
        TRUE,
    );
    Swap_ErasePopupMenu(SWAP_WIN_OPTIONS);
    task_set(taskId, tState, 0);
    task_set(
        taskId,
        tFollowUpTaskPtrHi,
        (Swap_Task_HandleChooseMons as *const () as usize as u32 >> 16) as i16,
    );
    task_set(
        taskId,
        tFollowUpTaskPtrLo,
        Swap_Task_HandleChooseMons as *const () as usize as u32 as i16,
    );
    task_set(taskId, tFollowUpTaskState, STATE_CHOOSE_MONS_HANDLE_INPUT);
    task_set_func(taskId, Some(Swap_Task_ScreenInfoTransitionIn));
}
unsafe fn Swap_RunActionFunc(taskId: u8) {
    sSwap_CurrentOptionFunc = (*(*sFactorySwapScreen)
        .actionsData
        .at((*sFactorySwapScreen).cursorPos))
    .func;
    sSwap_CurrentOptionFunc.unwrap_unchecked()(taskId);
}
pub(crate) fn Swap_ActionCancel(taskId: u8) {
    task_set(
        taskId,
        tFollowUpTaskPtrHi,
        (Swap_AskQuitSwapping as *const () as usize as u32 >> 16) as i16,
    );
    task_set(
        taskId,
        tFollowUpTaskPtrLo,
        Swap_AskQuitSwapping as *const () as usize as u32 as i16,
    );
    task_set(taskId, tState, 0);
    task_set(taskId, tFollowUpTaskState, 0);
    task_set_func(taskId, Some(Swap_Task_ScreenInfoTransitionOut));
}
pub(crate) fn Swap_ActionPkmnForSwap(taskId: u8) {
    task_set(
        taskId,
        tFollowUpTaskPtrHi,
        (Swap_Task_SwitchPartyScreen as *const () as usize as u32 >> 16) as i16,
    );
    task_set(
        taskId,
        tFollowUpTaskPtrLo,
        Swap_Task_SwitchPartyScreen as *const () as usize as u32 as i16,
    );
    task_set(taskId, tFollowUpTaskState, 0);
    task_set(taskId, tState, 0);
    task_set_func(taskId, Some(Swap_Task_ScreenInfoTransitionOut));
}
pub(crate) unsafe fn Swap_ActionMon(taskId: u8) {
    if (*sFactorySwapScreen).inEnemyScreen == 0 {
        task_set(
            taskId,
            tFollowUpTaskPtrHi,
            (Swap_Task_HandleMenu as *const () as usize as u32 >> 16) as i16,
        );
        task_set(
            taskId,
            tFollowUpTaskPtrLo,
            Swap_Task_HandleMenu as *const () as usize as u32 as i16,
        );
        task_set(taskId, tFollowUpTaskState, STATE_MENU_INIT);
    } else if Swap_AlreadyHasSameSpecies((*sFactorySwapScreen).cursorPos) == TRUE {
        OpenMonPic(
            &raw mut (*sFactorySwapScreen).monPic.bgSpriteId,
            &raw mut (*sFactorySwapScreen).monPicAnimating,
            TRUE,
        );
        task_set(taskId, tState, 0);
        task_set(taskId, tFollowUpTaskState, STATE_CHOOSE_MONS_HANDLE_INPUT);
        task_set_func(taskId, Some(Swap_TaskCantHaveSameMons));
        return;
    } else {
        task_set(
            taskId,
            tFollowUpTaskPtrHi,
            (Swap_AskAcceptMon as *const () as usize as u32 >> 16) as i16,
        );
        task_set(
            taskId,
            tFollowUpTaskPtrLo,
            Swap_AskAcceptMon as *const () as usize as u32 as i16,
        );
        task_set(taskId, tFollowUpTaskState, 0);
    }
    task_set(taskId, tState, 0);
    task_set_func(taskId, Some(Swap_Task_ScreenInfoTransitionOut));
}
unsafe fn OpenMonPic(spriteId: *mut u8, animating: *mut u8, swapScreen: u8) {
    *spriteId = CreateSprite(
        (&raw const *sSpriteTemplate_Swap_MonPicBgAnim).cast_mut(),
        120,
        64,
        1,
    );
    gSprites[*spriteId].callback = Some(SpriteCB_OpenMonPic);
    gSprites[*spriteId].data[sIsSwapScreen] = swapScreen as i16;
    *animating = TRUE;
}
unsafe fn Swap_ShowSummaryMonSprite() {
    (*sFactorySwapScreen).monPic.bgSpriteId = CreateSprite(
        (&raw const *sSpriteTemplate_Swap_MonPicBgAnim).cast_mut(),
        120,
        64,
        1,
    );
    StartSpriteAffineAnim(
        &raw mut gSprites[(*sFactorySwapScreen).monPic.bgSpriteId],
        2,
    );
    let mon: *mut Pokemon = &raw mut gPlayerParty[(*sFactorySwapScreen).cursorPos];
    let species: u16 = GetMonData3(mon, MON_DATA_SPECIES, null_mut()) as u16;
    let personality: u32 = GetMonData3(mon, MON_DATA_PERSONALITY, null_mut());
    let otId: u32 = GetMonData3(mon, MON_DATA_OT_ID, null_mut());
    (*sFactorySwapScreen).monPic.monSpriteId =
        CreateMonPicSprite_HandleDeoxys(species, personality, otId, TRUE, 88, 32, 15, TAG_NONE)
            as u8;
    gSprites[(*sFactorySwapScreen).monPic.monSpriteId].centerToCornerVecX = 0;
    gSprites[(*sFactorySwapScreen).monPic.monSpriteId].centerToCornerVecY = 0;
    gSprites[(*sFactorySwapScreen).monPic.bgSpriteId].set_invisible(TRUE as u16);
}
unsafe fn CloseMonPic(pic: FactoryMonPic, animating: *mut u8, swapScreen: u8) {
    FreeAndDestroyMonPicSprite(pic.monSpriteId as u16);
    let taskId: u8 = CreateTask(Some(Task_CloseMonPic), 1);
    task_set(taskId, tIsSwapScreen, swapScreen as i16);
    task_set(taskId, tSpriteId, pic.bgSpriteId as i16);
    task_func(taskId).unwrap_unchecked()(taskId);
    *animating = TRUE;
}
unsafe fn HideMonPic(pic: FactoryMonPic, animating: *mut u8) {
    FreeAndDestroyMonPicSprite(pic.monSpriteId as u16);
    FreeOamMatrix(gSprites[pic.bgSpriteId].oam.matrixNum() as u8);
    DestroySprite(&raw mut gSprites[pic.bgSpriteId]);
    *animating = FALSE;
}
pub(crate) unsafe fn Swap_TaskCantHaveSameMons(taskId: u8) {
    if (*sFactorySwapScreen).monPicAnimating == TRUE {
        return;
    }
    match task_get(taskId, tState) {
        0 => {
            Swap_PrintOnInfoWindow(
                (*(&raw const crate::data::strings::gText_SamePkmnInPartyAlready)
                    .cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
            );
            (*sFactorySwapScreen).monSwapped = FALSE;
            task_set(taskId, tState, task_get(taskId, tState) + 1);
        }
        1 => {
            if gMain.newKeys as i32 & A_BUTTON != 0 || gMain.newKeys as i32 & B_BUTTON != 0 {
                PlaySE(SE_SELECT);
                CloseMonPic(
                    (*sFactorySwapScreen).monPic,
                    &raw mut (*sFactorySwapScreen).monPicAnimating,
                    TRUE,
                );
                task_set(taskId, tState, task_get(taskId, tState) + 1);
            }
        }
        2 => {
            if (*sFactorySwapScreen).monPicAnimating != TRUE {
                FillWindowPixelBuffer(SWAP_WIN_ACTION_FADE, 0);
                CopyWindowToVram(SWAP_WIN_ACTION_FADE, COPYWIN_GFX);
                task_set(taskId, tState, task_get(taskId, tState) + 1);
            }
        }
        3 => {
            Swap_PrintOnInfoWindow(
                (*(&raw const crate::data::strings::gText_SelectPkmnToAccept)
                    .cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
            );
            task_set(taskId, tState, task_get(taskId, tState) + 1);
        }
        4 => {
            Swap_PrintMonSpeciesForTransition();
            Swap_EraseSpeciesAtFadeWindow();
            (*sFactorySwapScreen).fadeSpeciesNameActive = TRUE;
            task_set(taskId, tState, task_get(taskId, tFollowUpTaskState));
            task_set_func(taskId, Some(Swap_Task_HandleChooseMons));
        }
        _ => {}
    }
}
unsafe fn Swap_AlreadyHasSameSpecies(monId: u8) -> u8 {
    let species: u16 =
        GetMonData3(&raw mut gEnemyParty[monId], MON_DATA_SPECIES, null_mut()) as u16;
    for i in 0..(FRONTIER_PARTY_SIZE as u8) {
        if i != (*sFactorySwapScreen).playerMonId
            && GetMonData3(&raw mut gPlayerParty[i], MON_DATA_SPECIES, null_mut()) as u16 == species
        {
            return TRUE;
        }
    }
    FALSE
}
pub(crate) unsafe fn SpriteCB_OpenMonPic(sprite: *mut Sprite) {
    let mut taskId: u8 = 0;
    if (*sprite).affineAnimEnded() != 0 {
        (*sprite).set_invisible(TRUE as u16);
        taskId = CreateTask(Some(Task_OpenMonPic), 1);
        task_set(taskId, 7, (*sprite).data[7]);
        task_func(taskId).unwrap_unchecked()(taskId);
        (*sprite).callback = Some(SpriteCallbackDummy);
    }
}
pub(crate) unsafe fn SpriteCB_CloseMonPic(sprite: *mut Sprite) {
    if (*sprite).affineAnimEnded() != 0 {
        FreeOamMatrix((*sprite).oam.matrixNum() as u8);
        if (*sprite).data[sIsSwapScreen] == TRUE as i16 {
            (*sFactorySwapScreen).monPicAnimating = FALSE;
        } else {
            Select_SetMonPicAnimating(FALSE);
        }
        DestroySprite(sprite);
    }
}
pub(crate) unsafe fn Task_OpenMonPic(taskId: u8) {
    let task: *mut Task = &raw mut (*gTasks.as_ptr())[taskId];
    match (*task).data[tState] {
        0 => {
            (*task).data[tWinLeft] = 88;
            (*task).data[tWinRight] = 152;
            (*task).data[tWinTop] = 64;
            (*task).data[tWinBottom] = 65;
            SetGpuRegBits(REG_OFFSET_DISPCNT, DISPCNT_WIN0_ON);
            SetGpuReg(
                REG_OFFSET_WIN0H,
                ((*task).data[tWinLeft] as u16) << 8 | (*task).data[tWinRight] as u16,
            );
            SetGpuReg(
                REG_OFFSET_WIN0V,
                ((*task).data[tWinTop] as u16) << 8 | (*task).data[tWinBottom] as u16,
            );
            SetGpuReg(REG_OFFSET_WININ, 63);
            SetGpuReg(REG_OFFSET_WINOUT, 55);
        }
        1 => {
            ShowBg(3);
            SetGpuReg(REG_OFFSET_BLDCNT, 4680);
            SetGpuReg(REG_OFFSET_BLDALPHA, 1035);
        }
        2 => {
            (*task).data[tWinTop] -= 4;
            (*task).data[tWinBottom] += 4;
            if (*task).data[tWinTop] <= 32 || (*task).data[tWinBottom] >= 96 {
                (*task).data[tWinTop] = 32;
                (*task).data[tWinBottom] = 96;
            }
            SetGpuReg(
                REG_OFFSET_WIN0V,
                ((*task).data[tWinTop] as u16) << 8 | (*task).data[tWinBottom] as u16,
            );
            if (*task).data[tWinTop] != 32 {
                return;
            }
        }
        _ => {
            DestroyTask(taskId);
            if task_get(taskId, tIsSwapScreen) == TRUE as i16 {
                Swap_CreateMonSprite();
            } else {
                Select_CreateMonSprite();
            }
            return;
        }
    }
    (*task).data[tState] += 1;
}
pub(crate) unsafe fn Task_CloseMonPic(taskId: u8) {
    let task: *mut Task = &raw mut (*gTasks.as_ptr())[taskId];
    match (*task).data[tState] {
        0 => {
            (*task).data[tWinLeft] = 88;
            (*task).data[tWinRight] = 152;
            (*task).data[tWinTop] = 32;
            (*task).data[tWinBottom] = 96;
            SetGpuRegBits(REG_OFFSET_DISPCNT, DISPCNT_WIN0_ON);
            SetGpuReg(
                REG_OFFSET_WIN0H,
                ((*task).data[tWinLeft] as u16) << 8 | (*task).data[tWinRight] as u16,
            );
            SetGpuReg(
                REG_OFFSET_WIN0V,
                ((*task).data[tWinTop] as u16) << 8 | (*task).data[tWinBottom] as u16,
            );
            SetGpuReg(REG_OFFSET_WININ, 63);
            SetGpuReg(REG_OFFSET_WINOUT, 55);
            (*task).data[tState] += 1;
        }
        1 => {
            (*task).data[tWinTop] += 4;
            (*task).data[tWinBottom] -= 4;
            if (*task).data[tWinTop] >= 64 || (*task).data[tWinBottom] <= 65 {
                (*task).data[tWinTop] = 64;
                (*task).data[tWinBottom] = 65;
            }
            SetGpuReg(
                REG_OFFSET_WIN0V,
                ((*task).data[tWinTop] as u16) << 8 | (*task).data[tWinBottom] as u16,
            );
            if (*task).data[tWinTop] == 64 {
                (*task).data[tState] += 1;
            }
        }
        _ => {
            HideBg(3);
            gSprites[(*task).data[tSpriteId]].data[7] = (*task).data[7];
            gSprites[(*task).data[tSpriteId]].set_invisible(FALSE as u16);
            gSprites[(*task).data[tSpriteId]].callback = Some(SpriteCB_CloseMonPic);
            StartSpriteAffineAnim(&raw mut gSprites[(*task).data[tSpriteId]], 1);
            ClearGpuRegBits(REG_OFFSET_DISPCNT, DISPCNT_WIN0_ON);
            DestroyTask(taskId);
        }
    }
}
unsafe fn Swap_CreateMonSprite() {
    let mut mon: *mut Pokemon = null_mut();
    if (*sFactorySwapScreen).inEnemyScreen == 0 {
        mon = &raw mut gPlayerParty[(*sFactorySwapScreen).cursorPos];
    } else {
        mon = &raw mut gEnemyParty[(*sFactorySwapScreen).cursorPos];
    }
    let species: u16 = GetMonData3(mon, MON_DATA_SPECIES, null_mut()) as u16;
    let personality: u32 = GetMonData3(mon, MON_DATA_PERSONALITY, null_mut());
    let otId: u32 = GetMonData3(mon, MON_DATA_OT_ID, null_mut());
    (*sFactorySwapScreen).monPic.monSpriteId =
        CreateMonPicSprite_HandleDeoxys(species, otId, personality, TRUE, 88, 32, 15, TAG_NONE)
            as u8;
    gSprites[(*sFactorySwapScreen).monPic.monSpriteId].centerToCornerVecX = 0;
    gSprites[(*sFactorySwapScreen).monPic.monSpriteId].centerToCornerVecY = 0;
    (*sFactorySwapScreen).monPicAnimating = FALSE;
}
