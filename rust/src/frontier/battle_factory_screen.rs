//! Translated from `src/battle_factory_screen.c` by tools/rustport/c2rs.py.
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
    pub func: Option<unsafe extern "C" fn(u8)>,
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
static sSelect_MenuOptionFuncs: Table<CArray<Option<unsafe extern "C" fn() -> u8>, 3>> =
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
static sSwap_MenuOptionFuncs: Table<CArray<Option<unsafe extern "C" fn(u8)>, 3>> =
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
pub(crate) static mut sSwap_CurrentOptionFunc: Option<unsafe extern "C" fn(u8)> = None;
pub(crate) static mut sFactorySwapScreen: *mut FactorySwapScreen = null_mut();
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gFactorySelect_CurrentOptionFunc: Option<unsafe extern "C" fn() -> u8> = None;

unsafe extern "C" {
    static gBattleFrontierHeldItems: CArray<u16, 0>;
    static gBattleFrontierMons: CArray<FacilityMon, 0>;
    static mut gEnemyParty: CArray<Pokemon, 6>;
    static mut gFacilityTrainerMons: *mut FacilityMon;
    static gFrontierFactoryMenu_Gfx: CArray<u16, 544>;
    static gFrontierFactoryMenu_Pal: CArray<u16, 0>;
    static gFrontierFactoryMenu_Tilemap: CArray<u16, 0>;
    static mut gLastViewedMonIndex: u8;
    static mut gMain: Main;
    static mut gPaletteFade: PaletteFadeControl;
    static mut gPlayerParty: CArray<Pokemon, 6>;
    static mut gPlttBufferFaded: CArray<u16, 512>;
    static mut gPlttBufferUnfaded: CArray<u16, 512>;
    static mut gSaveBlock2Ptr: *mut SaveBlock2;
    static gSlateportBattleTentMons: CArray<FacilityMon, 0>;
    static mut gSpecialVar_Result: u16;
    static gSpeciesNames: CArray<CArray<u8, 11>, 0>;
    static mut gSprites: CArray<Sprite, 65>;
    static mut gStringVar4: CArray<u8, 1000>;
    static mut gTasks: CArray<Task, 0>;
    static gText_AcceptThisPkmn: CArray<u8, 0>;
    static gText_Cancel3: CArray<u8, 0>;
    static gText_CantSelectSamePkmn: CArray<u8, 0>;
    static gText_Deselect: CArray<u8, 0>;
    static gText_No2: CArray<u8, 0>;
    static gText_No3: CArray<u8, 0>;
    static gText_Others2: CArray<u8, 0>;
    static gText_PkmnForSwap: CArray<u8, 0>;
    static gText_PkmnSwap: CArray<u8, 0>;
    static gText_QuitSwapping: CArray<u8, 0>;
    static gText_Rechoose: CArray<u8, 0>;
    static gText_Rent: CArray<u8, 0>;
    static gText_RentalPkmn2: CArray<u8, 0>;
    static gText_SamePkmnInPartyAlready: CArray<u8, 0>;
    static gText_SelectFirstPkmn: CArray<u8, 0>;
    static gText_SelectPkmnToAccept: CArray<u8, 0>;
    static gText_SelectPkmnToSwap: CArray<u8, 0>;
    static gText_SelectSecondPkmn: CArray<u8, 0>;
    static gText_SelectThirdPkmn: CArray<u8, 0>;
    static gText_Summary: CArray<u8, 0>;
    static gText_Summary2: CArray<u8, 0>;
    static gText_Swap: CArray<u8, 0>;
    static gText_TheseThreePkmnOkay: CArray<u8, 0>;
    static gText_Yes2: CArray<u8, 0>;
    static gText_Yes3: CArray<u8, 0>;
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
    fn Alloc(a0: u32) -> *mut c_void;
    fn AllocZeroed(a0: u32) -> *mut c_void;
    fn AnimateSprites();
    fn BeginNormalPaletteFade(a0: u32, a1: i8, a2: u8, a3: u8, a4: u16) -> u8;
    fn BlendPalettes(a0: u32, a1: u8, a2: u16);
    fn BuildOamBuffer();
    fn CB2_ReturnToFieldContinueScript();
    fn CalculatePlayerPartyCount() -> u8;
    fn ChangeBgX(a0: u8, a1: i32, a2: u8) -> i32;
    fn ChangeBgY(a0: u8, a1: i32, a2: u8) -> i32;
    fn ClearGpuRegBits(a0: u8, a1: u16);
    fn ClearWindowTilemap(a0: u8);
    fn CopyBgTilemapBufferToVram(a0: u8);
    fn CopyMonCategoryText(a0: i32, a1: *mut u8);
    fn CopyToBgTilemapBufferRect(a0: u8, a1: *mut c_void, a2: u8, a3: u8, a4: u8, a5: u8);
    fn CopyWindowToVram(a0: u8, a1: u8);
    fn CpuSet(a0: *mut c_void, a1: *mut c_void, a2: u32);
    fn CreateMonPicSprite_HandleDeoxys(
        a0: u16,
        a1: u32,
        a2: u32,
        a3: u8,
        a4: i16,
        a5: i16,
        a6: u8,
        a7: u16,
    ) -> u16;
    fn CreateMonWithEVSpreadNatureOTID(
        a0: *mut Pokemon,
        a1: u16,
        a2: u8,
        a3: u8,
        a4: u8,
        a5: u8,
        a6: u32,
    );
    fn CreateSprite(a0: *mut SpriteTemplate, a1: i16, a2: i16, a3: u8) -> u8;
    fn CreateTask(a0: Option<unsafe extern "C" fn(u8)>, a1: u8) -> u8;
    fn DeactivateAllTextPrinters();
    fn DestroySprite(a0: *mut Sprite);
    fn DestroyTask(a0: u8);
    fn FillWindowPixelBuffer(a0: u8, a1: u8);
    fn Free(a0: *mut c_void);
    fn FreeAllSpritePalettes();
    fn FreeAllWindowBuffers();
    fn FreeAndDestroyMonPicSprite(a0: u16) -> u16;
    fn FreeOamMatrix(a0: u8);
    fn FuncIsActiveTask(a0: Option<unsafe extern "C" fn(u8)>) -> u8;
    fn GetBoxMonData3(a0: *mut BoxPokemon, a1: i32, a2: *mut u8) -> u32;
    fn GetFactoryMonFixedIV(a0: u8, a1: u8) -> u8;
    fn GetMonData3(a0: *mut Pokemon, a1: i32, a2: *mut u8) -> u32;
    fn GetNumPastRentalsRank(a0: u8, a1: u8) -> u8;
    fn GetStringRightAlignXOffset(a0: i32, a1: *mut u8, a2: i32) -> i32;
    fn HideBg(a0: u8);
    fn IndexOfSpritePaletteTag(a0: u16) -> u8;
    fn InitBgsFromTemplates(a0: u8, a1: *mut BgTemplate, a2: u8);
    fn InitWindows(a0: *mut WindowTemplate) -> u16;
    fn LoadBgTilemap(a0: u8, a1: *mut c_void, a2: u16, a3: u16) -> u16;
    fn LoadBgTiles(a0: u8, a1: *mut c_void, a2: u16, a3: u16) -> u16;
    fn LoadCompressedSpriteSheet(a0: *mut CompressedSpriteSheet) -> u16;
    fn LoadOam();
    fn LoadPalette(a0: *mut c_void, a1: u16, a2: u16);
    fn LoadSpritePalettes(a0: *mut SpritePalette);
    fn LoadSpriteSheets(a0: *mut SpriteSheet);
    fn PlaySE(a0: u16);
    fn ProcessSpriteCopyRequests();
    fn PutWindowTilemap(a0: u8);
    fn Random() -> u16;
    fn ResetAllPicSprites() -> u16;
    fn ResetBgsAndClearDma3BusyFlags(a0: u32);
    fn ResetPaletteFade();
    fn ResetSpriteData();
    fn ResetTasks();
    fn RunTasks();
    fn RunTextPrinters();
    fn SetBgTilemapBuffer(a0: u8, a1: *mut c_void);
    fn SetGpuReg(a0: u8, a1: u16);
    fn SetGpuRegBits(a0: u8, a1: u16);
    fn SetHBlankCallback(a0: Option<unsafe extern "C" fn()>);
    fn SetMainCallback2(a0: Option<unsafe extern "C" fn()>);
    fn SetMonData(a0: *mut Pokemon, a1: i32, a2: *mut c_void);
    fn SetMonMoveAvoidReturn(a0: *mut Pokemon, a1: u16, a2: u8);
    fn SetVBlankCallback(a0: Option<unsafe extern "C" fn()>);
    fn ShowBg(a0: u8);
    fn ShowPokemonSummaryScreen(
        a0: u8,
        a1: *mut c_void,
        a2: u8,
        a3: u8,
        a4: Option<unsafe extern "C" fn()>,
    );
    fn SpeciesToNationalPokedexNum(a0: u16) -> u16;
    fn SpriteCallbackDummy(a0: *mut Sprite);
    fn StartSpriteAffineAnim(a0: *mut Sprite, a1: u8);
    fn StartSpriteAnim(a0: *mut Sprite, a1: u8);
    fn StartSpriteAnimIfDifferent(a0: *mut Sprite, a1: u8);
    fn StringCopy(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn TransferPlttBuffer();
    fn UpdatePaletteFade() -> u8;
    fn VarGet(a0: u16) -> u16;
}

pub(crate) unsafe extern "C" fn SpriteCB_Pokeball(sprite: *mut Sprite) {
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
pub(crate) unsafe extern "C" fn CB2_SelectScreen() {
    AnimateSprites();
    BuildOamBuffer();
    RunTextPrinters();
    UpdatePaletteFade();
    RunTasks();
}
pub(crate) unsafe extern "C" fn VBlankCB_SelectScreen() {
    LoadOam();
    ProcessSpriteCopyRequests();
    TransferPlttBuffer();
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DoBattleFactorySelectScreen() {
    sFactorySelectScreen = null_mut();
    SetMainCallback2(Some(CB2_InitSelectScreen));
}
pub(crate) unsafe extern "C" fn CB2_InitSelectScreen() {
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
                gFrontierFactoryMenu_Gfx.as_ptr().cast_mut() as *mut c_void,
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
                gFrontierFactoryMenu_Tilemap.as_ptr().cast_mut() as *mut c_void,
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
                gFrontierFactoryMenu_Pal.as_ptr().cast_mut() as *mut c_void,
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
                gTasks[(*sFactorySelectScreen).fadeSpeciesNameTaskId].data[0] = FADESTATE_INIT;
                taskId = CreateTask(Some(Select_Task_HandleChooseMons), 0);
                gTasks[taskId].data[0] = STATE_CHOOSE_MONS_INIT;
            } else {
                gTasks[(*sFactorySelectScreen).fadeSpeciesNameTaskId].data[0] = FADESTATE_RUN;
                (*sFactorySelectScreen).fadeSpeciesNameActive = FALSE;
                taskId = CreateTask(Some(Select_Task_HandleMenu), 0);
                gTasks[taskId].data[0] = STATE_MENU_RESHOW;
            }
            SetMainCallback2(Some(CB2_SelectScreen));
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn Select_InitMonsData() {
    let mut i: u8 = 0;
    if !sFactorySelectScreen.is_null() {
        return;
    }
    sFactorySelectScreen = AllocZeroed(684) as *mut FactorySelectScreen;
    (*sFactorySelectScreen).cursorPos = 0;
    (*sFactorySelectScreen).selectingMonsState = 1;
    (*sFactorySelectScreen).fromSummaryScreen = FALSE;
    i = 0;
    while i < SELECTABLE_MONS_COUNT {
        (*sFactorySelectScreen).mons[i].selectedId = 0;
        i += 1;
    }
    if (*gSaveBlock2Ptr).frontier.lvlMode() != FRONTIER_LVL_TENT {
        CreateFrontierFactorySelectableMons(0);
    } else {
        CreateSlateportTentSelectableMons(0);
    }
}
pub(crate) unsafe extern "C" fn Select_InitAllSprites() {
    let mut i: u8 = 0;
    let mut cursorPos: u8 = 0;
    let mut x: i16 = 0;
    i = 0;
    while i < SELECTABLE_MONS_COUNT {
        (*sFactorySelectScreen).mons[i].ballSpriteId = CreateSprite(
            (&raw const *sSpriteTemplate_Select_Pokeball).cast_mut(),
            35 * i as i16 + 32,
            64,
            1,
        ) as u16;
        gSprites[(*sFactorySelectScreen).mons[i].ballSpriteId].data[0] = 0;
        Select_SetBallSpritePaletteNum(i);
        i += 1;
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
pub(crate) unsafe extern "C" fn Select_DestroyAllSprites() {
    let mut i: u8 = 0;
    i = 0;
    while i < SELECTABLE_MONS_COUNT {
        DestroySprite(&raw mut gSprites[(*sFactorySelectScreen).mons[i].ballSpriteId]);
        i += 1;
    }
    DestroySprite(&raw mut gSprites[(*sFactorySelectScreen).cursorSpriteId]);
    DestroySprite(&raw mut gSprites[(*sFactorySelectScreen).menuCursor1SpriteId]);
    DestroySprite(&raw mut gSprites[(*sFactorySelectScreen).menuCursor2SpriteId]);
}
pub(crate) unsafe extern "C" fn Select_UpdateBallCursorPosition(direction: i8) {
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
pub(crate) unsafe extern "C" fn Select_UpdateMenuCursorPosition(direction: i8) {
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
pub(crate) unsafe extern "C" fn Select_UpdateYesNoCursorPosition(direction: i8) {
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
pub(crate) unsafe extern "C" fn Select_HandleMonSelectionChange() {
    let mut i: u8 = 0;
    let mut paletteNum: u8 = 0;
    let mut cursorPos: u8 = (*sFactorySelectScreen).cursorPos;
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
pub(crate) unsafe extern "C" fn Select_SetBallSpritePaletteNum(id: u8) {
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
pub(crate) unsafe extern "C" fn Select_Task_OpenSummaryScreen(taskId: u8) {
    let mut i: u8 = 0;
    let mut currMonId: u8 = 0;
    match gTasks[taskId].data[0] {
        STATE_SUMMARY_FADE => {
            gPlttBufferUnfaded[228] = gPlttBufferFaded[228];
            BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 16, 0);
            gTasks[taskId].data[0] = STATE_SUMMARY_CLEAN;
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
                gTasks[taskId].data[0] = STATE_SUMMARY_SHOW;
            }
        }
        STATE_SUMMARY_SHOW => {
            (*sFactorySelectScreen).speciesNameColorBackup = gPlttBufferUnfaded[228];
            DestroyTask(taskId);
            (*sFactorySelectScreen).fromSummaryScreen = TRUE;
            currMonId = (*sFactorySelectScreen).cursorPos;
            sFactorySelectMons = AllocZeroed(600) as *mut Pokemon;
            i = 0;
            while i < SELECTABLE_MONS_COUNT {
                *sFactorySelectMons.at(i) = (*sFactorySelectScreen).mons[i].monData;
                i += 1;
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
pub(crate) unsafe extern "C" fn Select_Task_Exit(taskId: u8) {
    if (*sFactorySelectScreen).monPicAnimating == TRUE {
        return;
    }
    match gTasks[taskId].data[0] {
        0 => {
            BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 16, 0);
            gTasks[taskId].data[0] += 1;
        }
        1 => {
            if UpdatePaletteFade() == 0 {
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
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn Select_Task_HandleYesNo(taskId: u8) {
    if (*sFactorySelectScreen).monPicAnimating == TRUE {
        return;
    }
    match gTasks[taskId].data[0] {
        STATE_YESNO_SHOW_MONS => {
            Select_ShowChosenMons();
            gTasks[taskId].data[0] = STATE_YESNO_SHOW_OPTIONS;
        }
        STATE_YESNO_SHOW_OPTIONS => {
            Select_ShowYesNoOptions();
            gTasks[taskId].data[0] = STATE_YESNO_HANDLE_INPUT;
        }
        STATE_YESNO_HANDLE_INPUT => {
            if gMain.newKeys as i32 & A_BUTTON != 0 {
                PlaySE(SE_SELECT);
                if (*sFactorySelectScreen).yesNoCursorPos == 0 {
                    Select_HideChosenMons();
                    gTasks[taskId].data[0] = 0;
                    gTasks[taskId].func = Some(Select_Task_Exit);
                } else {
                    Select_ErasePopupMenu(SELECT_WIN_YES_NO);
                    Select_DeclineChosenMons();
                    (*sFactorySelectScreen).fadeSpeciesNameActive = TRUE;
                    gTasks[taskId].data[0] = STATE_CHOOSE_MONS_HANDLE_INPUT;
                    gTasks[taskId].func = Some(Select_Task_HandleChooseMons);
                }
            } else if gMain.newKeys as i32 & B_BUTTON != 0 {
                PlaySE(SE_SELECT);
                Select_ErasePopupMenu(SELECT_WIN_YES_NO);
                Select_DeclineChosenMons();
                (*sFactorySelectScreen).fadeSpeciesNameActive = TRUE;
                gTasks[taskId].data[0] = STATE_CHOOSE_MONS_HANDLE_INPUT;
                gTasks[taskId].func = Some(Select_Task_HandleChooseMons);
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
pub(crate) unsafe extern "C" fn Select_Task_HandleMenu(taskId: u8) {
    match gTasks[taskId].data[0] {
        STATE_MENU_INIT => {
            if (*sFactorySelectScreen).fromSummaryScreen == 0 {
                OpenMonPic(
                    &raw mut (*sFactorySelectScreen).monPics[1].bgSpriteId,
                    &raw mut (*sFactorySelectScreen).monPicAnimating,
                    FALSE,
                );
            }
            gTasks[taskId].data[0] = STATE_MENU_SHOW_OPTIONS;
        }
        STATE_MENU_SHOW_OPTIONS => {
            if (*sFactorySelectScreen).monPicAnimating != TRUE {
                Select_ShowMenuOptions();
                (*sFactorySelectScreen).fromSummaryScreen = FALSE;
                gTasks[taskId].data[0] = STATE_MENU_HANDLE_INPUT;
            }
        }
        STATE_MENU_HANDLE_INPUT => {
            if gMain.newKeys as i32 & A_BUTTON != 0 {
                let mut retVal: u8 = 0;
                PlaySE(SE_SELECT);
                retVal = Select_RunMenuOptionFunc();
                if retVal == SELECT_CONTINUE_CHOOSING {
                    (*sFactorySelectScreen).fadeSpeciesNameActive = TRUE;
                    gTasks[taskId].data[0] = STATE_CHOOSE_MONS_HANDLE_INPUT;
                    gTasks[taskId].func = Some(Select_Task_HandleChooseMons);
                } else if retVal == SELECT_CONFIRM_MONS {
                    gTasks[taskId].data[0] = STATE_YESNO_SHOW_MONS;
                    gTasks[taskId].func = Some(Select_Task_HandleYesNo);
                } else if retVal == SELECT_INVALID_MON {
                    gTasks[taskId].data[0] = STATE_CHOOSE_MONS_INVALID;
                    gTasks[taskId].func = Some(Select_Task_HandleChooseMons);
                } else {
                    gTasks[taskId].data[0] = STATE_SUMMARY_FADE;
                    gTasks[taskId].func = Some(Select_Task_OpenSummaryScreen);
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
                gTasks[taskId].data[0] = STATE_CHOOSE_MONS_HANDLE_INPUT;
                gTasks[taskId].func = Some(Select_Task_HandleChooseMons);
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
                gTasks[taskId].data[0] = STATE_MENU_HANDLE_INPUT;
            }
        }
        STATE_MENU_RESHOW => {
            Select_ShowMenuOptions();
            gTasks[taskId].data[0] = STATE_MENU_REINIT;
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn Select_Task_HandleChooseMons(taskId: u8) {
    if (*sFactorySelectScreen).monPicAnimating == TRUE {
        return;
    }
    match gTasks[taskId].data[0] {
        STATE_CHOOSE_MONS_INIT => {
            if gPaletteFade.active() == 0 {
                gTasks[taskId].data[0] = STATE_CHOOSE_MONS_HANDLE_INPUT;
                (*sFactorySelectScreen).fadeSpeciesNameActive = TRUE;
            }
        }
        STATE_CHOOSE_MONS_HANDLE_INPUT => {
            if gMain.newKeys as i32 & A_BUTTON != 0 {
                PlaySE(SE_SELECT);
                (*sFactorySelectScreen).fadeSpeciesNameActive = FALSE;
                gTasks[taskId].data[0] = STATE_MENU_INIT;
                gTasks[taskId].func = Some(Select_Task_HandleMenu);
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
        STATE_CHOOSE_MONS_INVALID => {
            if gMain.newKeys as i32 & A_BUTTON != 0 {
                PlaySE(SE_SELECT);
                CloseMonPic(
                    (*sFactorySelectScreen).monPics[1],
                    &raw mut (*sFactorySelectScreen).monPicAnimating,
                    FALSE,
                );
                Select_PrintSelectMonString();
                (*sFactorySelectScreen).fadeSpeciesNameActive = TRUE;
                gTasks[taskId].data[0] = STATE_CHOOSE_MONS_HANDLE_INPUT;
            }
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn CreateFrontierFactorySelectableMons(firstMonId: u8) {
    let mut i: u8 = 0;
    let mut j: u8 = 0;
    let mut ivs: u8 = 0;
    let mut level: u8 = 0;
    let mut friendship: u8 = 0;
    let mut otId: u32 = 0;
    let mut battleMode: u8 = VarGet(VAR_FRONTIER_BATTLE_MODE) as u8;
    let mut lvlMode: u8 = (*gSaveBlock2Ptr).frontier.lvlMode();
    let mut challengeNum: u8 =
        ((*gSaveBlock2Ptr).frontier.factoryWinStreaks[battleMode][lvlMode] as i32 / 7) as u8;
    let mut rentalRank: u8 = 0;
    gFacilityTrainerMons = gBattleFrontierMons.as_ptr().cast_mut();
    if (*gSaveBlock2Ptr).frontier.lvlMode() != FRONTIER_LVL_50 {
        level = FRONTIER_MAX_LEVEL_OPEN;
    } else {
        level = FRONTIER_MAX_LEVEL_50;
    }
    rentalRank = GetNumPastRentalsRank(battleMode, lvlMode);
    otId = (*gSaveBlock2Ptr).playerTrainerId[0] as u32
        | ((*gSaveBlock2Ptr).playerTrainerId[1] as u32) << 8
        | ((*gSaveBlock2Ptr).playerTrainerId[2] as u32) << 16
        | ((*gSaveBlock2Ptr).playerTrainerId[3] as u32) << 24;
    i = 0;
    while i < SELECTABLE_MONS_COUNT {
        let mut monId: u16 = (*gSaveBlock2Ptr).frontier.rentalMons[i].monId;
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
        j = 0;
        while j < MAX_MON_MOVES as u8 {
            SetMonMoveAvoidReturn(
                &raw mut (*sFactorySelectScreen).mons[i as i32 + firstMonId as i32].monData,
                (*gFacilityTrainerMons.at(monId)).moves[j],
                j,
            );
            j += 1;
        }
        SetMonData(
            &raw mut (*sFactorySelectScreen).mons[i as i32 + firstMonId as i32].monData,
            MON_DATA_FRIENDSHIP,
            &raw mut friendship as *mut c_void,
        );
        SetMonData(
            &raw mut (*sFactorySelectScreen).mons[i as i32 + firstMonId as i32].monData,
            MON_DATA_HELD_ITEM,
            (&raw const gBattleFrontierHeldItems[(*gFacilityTrainerMons.at(monId)).itemTableId])
                .cast_mut() as *mut c_void,
        );
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn CreateSlateportTentSelectableMons(firstMonId: u8) {
    let mut i: u8 = 0;
    let mut j: u8 = 0;
    let mut ivs: u8 = 0;
    let mut level: u8 = TENT_MIN_LEVEL;
    let mut friendship: u8 = 0;
    let mut otId: u32 = 0;
    gFacilityTrainerMons = gSlateportBattleTentMons.as_ptr().cast_mut();
    otId = (*gSaveBlock2Ptr).playerTrainerId[0] as u32
        | ((*gSaveBlock2Ptr).playerTrainerId[1] as u32) << 8
        | ((*gSaveBlock2Ptr).playerTrainerId[2] as u32) << 16
        | ((*gSaveBlock2Ptr).playerTrainerId[3] as u32) << 24;
    i = 0;
    while i < SELECTABLE_MONS_COUNT {
        let mut monId: u16 = (*gSaveBlock2Ptr).frontier.rentalMons[i].monId;
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
        j = 0;
        while j < MAX_MON_MOVES as u8 {
            SetMonMoveAvoidReturn(
                &raw mut (*sFactorySelectScreen).mons[i as i32 + firstMonId as i32].monData,
                (*gFacilityTrainerMons.at(monId)).moves[j],
                j,
            );
            j += 1;
        }
        SetMonData(
            &raw mut (*sFactorySelectScreen).mons[i as i32 + firstMonId as i32].monData,
            MON_DATA_FRIENDSHIP,
            &raw mut friendship as *mut c_void,
        );
        SetMonData(
            &raw mut (*sFactorySelectScreen).mons[i as i32 + firstMonId as i32].monData,
            MON_DATA_HELD_ITEM,
            (&raw const gBattleFrontierHeldItems[(*gFacilityTrainerMons.at(monId)).itemTableId])
                .cast_mut() as *mut c_void,
        );
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn Select_CopyMonsToPlayerParty() {
    let mut i: u8 = 0;
    let mut j: u8 = 0;
    i = 0;
    while i < FRONTIER_PARTY_SIZE as u8 {
        j = 0;
        while j < SELECTABLE_MONS_COUNT {
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
            j += 1;
        }
        i += 1;
    }
    CalculatePlayerPartyCount();
}
pub(crate) unsafe extern "C" fn Select_ShowMenuOptions() {
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
pub(crate) unsafe extern "C" fn Select_ShowYesNoOptions() {
    (*sFactorySelectScreen).yesNoCursorPos = 0;
    gSprites[(*sFactorySelectScreen).menuCursor1SpriteId].x = 176;
    gSprites[(*sFactorySelectScreen).menuCursor1SpriteId].y = 112;
    gSprites[(*sFactorySelectScreen).menuCursor2SpriteId].x = 208;
    gSprites[(*sFactorySelectScreen).menuCursor2SpriteId].y = 112;
    gSprites[(*sFactorySelectScreen).menuCursor1SpriteId].set_invisible(FALSE as u16);
    gSprites[(*sFactorySelectScreen).menuCursor2SpriteId].set_invisible(FALSE as u16);
    Select_PrintYesNoOptions();
}
pub(crate) unsafe extern "C" fn Select_ErasePopupMenu(windowId: u8) {
    gSprites[(*sFactorySelectScreen).menuCursor1SpriteId].set_invisible(TRUE as u16);
    gSprites[(*sFactorySelectScreen).menuCursor2SpriteId].set_invisible(TRUE as u16);
    FillWindowPixelBuffer(windowId, 0);
    CopyWindowToVram(windowId, COPYWIN_GFX);
    ClearWindowTilemap(windowId);
}
pub(crate) unsafe extern "C" fn Select_PrintRentalPkmnString() {
    FillWindowPixelBuffer(SELECT_WIN_TITLE, 0);
    AddTextPrinterParameterized(
        SELECT_WIN_TITLE,
        FONT_NORMAL,
        gText_RentalPkmn2.as_ptr().cast_mut(),
        2,
        1,
        0,
        None,
    );
    CopyWindowToVram(SELECT_WIN_TITLE, COPYWIN_FULL);
}
pub(crate) unsafe extern "C" fn Select_PrintMonSpecies() {
    let mut species: u16 = 0;
    let mut x: u8 = 0;
    let mut monId: u8 = (*sFactorySelectScreen).cursorPos;
    FillWindowPixelBuffer(SELECT_WIN_SPECIES, 0);
    species = GetMonData3(
        &raw mut (*sFactorySelectScreen).mons[monId].monData,
        MON_DATA_SPECIES,
        null_mut(),
    ) as u16;
    StringCopy(
        gStringVar4.as_mut_ptr(),
        gSpeciesNames[species].as_ptr().cast_mut(),
    );
    x = GetStringRightAlignXOffset(FONT_NORMAL as i32, gStringVar4.as_mut_ptr(), 86) as u8;
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
pub(crate) unsafe extern "C" fn Select_PrintSelectMonString() {
    let mut str: *mut u8 = null_mut();
    FillWindowPixelBuffer(SELECT_WIN_INFO, 0);
    if (*sFactorySelectScreen).selectingMonsState == 1 {
        str = gText_SelectFirstPkmn.as_ptr().cast_mut();
    } else if (*sFactorySelectScreen).selectingMonsState == 2 {
        str = gText_SelectSecondPkmn.as_ptr().cast_mut();
    } else if (*sFactorySelectScreen).selectingMonsState == 3 {
        str = gText_SelectThirdPkmn.as_ptr().cast_mut();
    } else {
        str = gText_TheseThreePkmnOkay.as_ptr().cast_mut();
    }
    AddTextPrinterParameterized(SELECT_WIN_INFO, FONT_NORMAL, str, 2, 5, 0, None);
    CopyWindowToVram(SELECT_WIN_INFO, COPYWIN_GFX);
}
pub(crate) unsafe extern "C" fn Select_PrintCantSelectSameMon() {
    FillWindowPixelBuffer(SELECT_WIN_INFO, 0);
    AddTextPrinterParameterized(
        SELECT_WIN_INFO,
        FONT_NORMAL,
        gText_CantSelectSamePkmn.as_ptr().cast_mut(),
        2,
        5,
        0,
        None,
    );
    CopyWindowToVram(SELECT_WIN_INFO, COPYWIN_GFX);
}
pub(crate) unsafe extern "C" fn Select_PrintMenuOptions() {
    let mut selectedId: u8 =
        (*sFactorySelectScreen).mons[(*sFactorySelectScreen).cursorPos].selectedId;
    PutWindowTilemap(SELECT_WIN_OPTIONS);
    FillWindowPixelBuffer(SELECT_WIN_OPTIONS, 0);
    AddTextPrinterParameterized3(
        SELECT_WIN_OPTIONS,
        FONT_NORMAL,
        7,
        1,
        sMenuOptionTextColors.as_ptr().cast_mut(),
        0,
        gText_Summary.as_ptr().cast_mut(),
    );
    if selectedId != 0 {
        AddTextPrinterParameterized3(
            SELECT_WIN_OPTIONS,
            FONT_NORMAL,
            7,
            17,
            sMenuOptionTextColors.as_ptr().cast_mut(),
            0,
            gText_Deselect.as_ptr().cast_mut(),
        );
    } else {
        AddTextPrinterParameterized3(
            SELECT_WIN_OPTIONS,
            FONT_NORMAL,
            7,
            17,
            sMenuOptionTextColors.as_ptr().cast_mut(),
            0,
            gText_Rent.as_ptr().cast_mut(),
        );
    }
    AddTextPrinterParameterized3(
        SELECT_WIN_OPTIONS,
        FONT_NORMAL,
        7,
        33,
        sMenuOptionTextColors.as_ptr().cast_mut(),
        0,
        gText_Others2.as_ptr().cast_mut(),
    );
    CopyWindowToVram(SELECT_WIN_OPTIONS, COPYWIN_FULL);
}
pub(crate) unsafe extern "C" fn Select_PrintYesNoOptions() {
    PutWindowTilemap(SELECT_WIN_YES_NO);
    FillWindowPixelBuffer(SELECT_WIN_YES_NO, 0);
    AddTextPrinterParameterized3(
        SELECT_WIN_YES_NO,
        FONT_NORMAL,
        7,
        1,
        sMenuOptionTextColors.as_ptr().cast_mut(),
        0,
        gText_Yes2.as_ptr().cast_mut(),
    );
    AddTextPrinterParameterized3(
        SELECT_WIN_YES_NO,
        FONT_NORMAL,
        7,
        17,
        sMenuOptionTextColors.as_ptr().cast_mut(),
        0,
        gText_No2.as_ptr().cast_mut(),
    );
    CopyWindowToVram(SELECT_WIN_YES_NO, COPYWIN_FULL);
}
pub(crate) unsafe extern "C" fn Select_RunMenuOptionFunc() -> u8 {
    gFactorySelect_CurrentOptionFunc =
        sSelect_MenuOptionFuncs[(*sFactorySelectScreen).menuCursorPos];
    return gFactorySelect_CurrentOptionFunc.unwrap_unchecked()();
}
pub(crate) unsafe extern "C" fn Select_OptionRentDeselect() -> u8 {
    let mut selectedId: u8 =
        (*sFactorySelectScreen).mons[(*sFactorySelectScreen).cursorPos].selectedId;
    let mut monId: u16 = (*sFactorySelectScreen).mons[(*sFactorySelectScreen).cursorPos].monId;
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
        return 0;
    }
}
pub(crate) unsafe extern "C" fn Select_DeclineChosenMons() -> u8 {
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
        return 0;
    }
}
pub(crate) unsafe extern "C" fn Select_OptionSummary() -> u8 {
    return SELECT_SUMMARY;
}
pub(crate) unsafe extern "C" fn Select_OptionOthers() -> u8 {
    CloseMonPic(
        (*sFactorySelectScreen).monPics[1],
        &raw mut (*sFactorySelectScreen).monPicAnimating,
        FALSE,
    );
    Select_ErasePopupMenu(SELECT_WIN_OPTIONS);
    return SELECT_CONTINUE_CHOOSING;
}
pub(crate) unsafe extern "C" fn Select_PrintMonCategory() {
    let mut species: u16 = 0;
    let mut text: CArray<u8, 30> = zeroed();
    let mut x: u8 = 0;
    let mut monId: u8 = (*sFactorySelectScreen).cursorPos;
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
pub(crate) unsafe extern "C" fn Select_CreateMonSprite() {
    let mut monId: u8 = (*sFactorySelectScreen).cursorPos;
    let mut mon: *mut Pokemon = &raw mut (*sFactorySelectScreen).mons[monId].monData;
    let mut species: u16 = GetMonData3(mon, MON_DATA_SPECIES, null_mut()) as u16;
    let mut personality: u32 = GetMonData3(mon, MON_DATA_PERSONALITY, null_mut());
    let mut otId: u32 = GetMonData3(mon, MON_DATA_OT_ID, null_mut());
    (*sFactorySelectScreen).monPics[1].monSpriteId =
        CreateMonPicSprite_HandleDeoxys(species, otId, personality, 1, 88, 32, 15, TAG_NONE) as u8;
    gSprites[(*sFactorySelectScreen).monPics[1].monSpriteId].centerToCornerVecX = 0;
    gSprites[(*sFactorySelectScreen).monPics[1].monSpriteId].centerToCornerVecY = 0;
    (*sFactorySelectScreen).monPicAnimating = FALSE;
}
pub(crate) unsafe extern "C" fn Select_SetMonPicAnimating(animating: u8) {
    (*sFactorySelectScreen).monPicAnimating = animating;
}
pub(crate) unsafe extern "C" fn Select_ReshowMonSprite() {
    let mut mon: *mut Pokemon = null_mut();
    let mut species: u16 = 0;
    let mut personality: u32 = 0;
    let mut otId: u32 = 0;
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
    mon = &raw mut (*sFactorySelectScreen).mons[(*sFactorySelectScreen).cursorPos].monData;
    species = GetMonData3(mon, MON_DATA_SPECIES, null_mut()) as u16;
    personality = GetMonData3(mon, MON_DATA_PERSONALITY, null_mut());
    otId = GetMonData3(mon, MON_DATA_OT_ID, null_mut());
    (*sFactorySelectScreen).monPics[1].monSpriteId =
        CreateMonPicSprite_HandleDeoxys(species, otId, personality, 1, 88, 32, 15, TAG_NONE) as u8;
    gSprites[(*sFactorySelectScreen).monPics[1].monSpriteId].centerToCornerVecX = 0;
    gSprites[(*sFactorySelectScreen).monPics[1].monSpriteId].centerToCornerVecY = 0;
    gSprites[(*sFactorySelectScreen).monPics[1].bgSpriteId].set_invisible(1);
}
pub(crate) unsafe extern "C" fn Select_CreateChosenMonsSprites() {
    let mut i: u8 = 0;
    let mut j: u8 = 0;
    i = 0;
    while i < FRONTIER_PARTY_SIZE as u8 {
        j = 0;
        while j < SELECTABLE_MONS_COUNT {
            if (*sFactorySelectScreen).mons[j].selectedId as i32 == i as i32 + 1 {
                let mut mon: *mut Pokemon = &raw mut (*sFactorySelectScreen).mons[j].monData;
                let mut species: u16 = GetMonData3(mon, MON_DATA_SPECIES, null_mut()) as u16;
                let mut personality: u32 = GetMonData3(mon, MON_DATA_PERSONALITY, null_mut());
                let mut otId: u32 = GetMonData3(mon, MON_DATA_OT_ID, null_mut());
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
            j += 1;
        }
        i += 1;
    }
    (*sFactorySelectScreen).monPicAnimating = FALSE;
}
pub(crate) unsafe extern "C" fn SpriteCB_OpenChosenMonPics(sprite: *mut Sprite) {
    let mut taskId: u8 = 0;
    if (*sprite).affineAnimEnded() != 0
        && gSprites[(*sFactorySelectScreen).monPics[0].bgSpriteId].affineAnimEnded() != 0
        && gSprites[(*sFactorySelectScreen).monPics[2].bgSpriteId].affineAnimEnded() != 0
    {
        (*sprite).set_invisible(TRUE as u16);
        gSprites[(*sFactorySelectScreen).monPics[0].bgSpriteId].set_invisible(TRUE as u16);
        gSprites[(*sFactorySelectScreen).monPics[2].bgSpriteId].set_invisible(TRUE as u16);
        taskId = CreateTask(Some(Select_Task_OpenChosenMonPics), 1);
        gTasks[taskId].func.unwrap_unchecked()(taskId);
        (*sprite).callback = Some(SpriteCallbackDummy);
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_CloseChosenMonPics(sprite: *mut Sprite) {
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
pub(crate) unsafe extern "C" fn Select_Task_OpenChosenMonPics(taskId: u8) {
    let mut task: *mut Task = &raw mut gTasks[taskId];
    match (*task).data[0] {
        0 => {
            (*task).data[3] = 16;
            (*task).data[4] = 224;
            (*task).data[5] = 64;
            (*task).data[8] = 65;
            SetGpuRegBits(REG_OFFSET_DISPCNT, DISPCNT_WIN0_ON);
            SetGpuReg(
                REG_OFFSET_WIN0H,
                ((*task).data[3] as u16) << 8 | (*task).data[4] as u16,
            );
            SetGpuReg(
                REG_OFFSET_WIN0V,
                ((*task).data[5] as u16) << 8 | (*task).data[8] as u16,
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
            (*task).data[5] -= 4;
            (*task).data[8] += 4;
            if (*task).data[5] <= 32 || (*task).data[8] >= 96 {
                (*task).data[5] = 32;
                (*task).data[8] = 96;
                ClearGpuRegBits(REG_OFFSET_DISPCNT, DISPCNT_WIN0_ON);
            }
            SetGpuReg(
                REG_OFFSET_WIN0V,
                ((*task).data[5] as u16) << 8 | (*task).data[8] as u16,
            );
            if (*task).data[5] != 32 {
                return;
            }
        }
        _ => {
            DestroyTask(taskId);
            Select_CreateChosenMonsSprites();
            return;
        }
    }
    (*task).data[0] += 1;
}
pub(crate) unsafe extern "C" fn Select_Task_CloseChosenMonPics(taskId: u8) {
    let mut task: *mut Task = &raw mut gTasks[taskId];
    match (*task).data[0] {
        0 => {
            (*task).data[3] = 16;
            (*task).data[4] = 224;
            (*task).data[5] = 32;
            (*task).data[8] = 96;
            SetGpuRegBits(REG_OFFSET_DISPCNT, DISPCNT_WIN0_ON);
            SetGpuReg(
                REG_OFFSET_WIN0H,
                ((*task).data[3] as u16) << 8 | (*task).data[4] as u16,
            );
            SetGpuReg(
                REG_OFFSET_WIN0V,
                ((*task).data[5] as u16) << 8 | (*task).data[8] as u16,
            );
            SetGpuReg(REG_OFFSET_WININ, 63);
            SetGpuReg(REG_OFFSET_WINOUT, 55);
            (*task).data[0] += 1;
        }
        1 => {
            (*task).data[5] += 4;
            (*task).data[8] -= 4;
            if (*task).data[5] >= 64 || (*task).data[8] <= 65 {
                (*task).data[5] = 64;
                (*task).data[8] = 65;
            }
            SetGpuReg(
                REG_OFFSET_WIN0V,
                ((*task).data[5] as u16) << 8 | (*task).data[8] as u16,
            );
            if (*task).data[5] == 64 {
                (*task).data[0] += 1;
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
pub(crate) unsafe extern "C" fn Select_ShowChosenMons() {
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
pub(crate) unsafe extern "C" fn Select_HideChosenMons() {
    let mut taskId: u8 = 0;
    FreeAndDestroyMonPicSprite((*sFactorySelectScreen).monPics[0].monSpriteId as u16);
    FreeAndDestroyMonPicSprite((*sFactorySelectScreen).monPics[1].monSpriteId as u16);
    FreeAndDestroyMonPicSprite((*sFactorySelectScreen).monPics[2].monSpriteId as u16);
    taskId = CreateTask(Some(Select_Task_CloseChosenMonPics), 1);
    gTasks[taskId].func.unwrap_unchecked()(taskId);
    (*sFactorySelectScreen).monPicAnimating = TRUE;
}
pub(crate) unsafe extern "C" fn Select_SetWinRegs(
    mWin0H: i16,
    nWin0H: i16,
    mWin0V: i16,
    nWin0V: i16,
) {
    SetGpuRegBits(REG_OFFSET_DISPCNT, DISPCNT_WIN0_ON);
    SetGpuReg(REG_OFFSET_WIN0H, (mWin0H as u16) << 8 | nWin0H as u16);
    SetGpuReg(REG_OFFSET_WIN0V, (mWin0V as u16) << 8 | nWin0V as u16);
    SetGpuReg(REG_OFFSET_WININ, 63);
    SetGpuReg(REG_OFFSET_WINOUT, 55);
}
pub(crate) unsafe extern "C" fn Select_AreSpeciesValid(monId: u16) -> u32 {
    let mut i: u8 = 0;
    let mut j: u8 = 0;
    let mut species: u32 = (*gFacilityTrainerMons.at(monId)).species as u32;
    let mut selectState: u8 = (*sFactorySelectScreen).selectingMonsState;
    i = 1;
    while i < selectState {
        j = 0;
        while j < SELECTABLE_MONS_COUNT {
            if (*sFactorySelectScreen).mons[j].selectedId == i {
                if (*gFacilityTrainerMons.at((*sFactorySelectScreen).mons[j].monId)).species as u32
                    == species
                {
                    return FALSE as u32;
                }
                break;
            }
            j += 1;
        }
        i += 1;
    }
    return TRUE as u32;
}
pub(crate) unsafe extern "C" fn Select_Task_FadeSpeciesName(taskId: u8) {
    match gTasks[taskId].data[0] {
        FADESTATE_INIT => {
            (*sFactorySelectScreen).fadeSpeciesNameCoeffDelay = 0;
            (*sFactorySelectScreen).fadeSpeciesNameCoeff = 0;
            (*sFactorySelectScreen).fadeSpeciesNameFadeOut = TRUE;
            gTasks[taskId].data[0] = FADESTATE_RUN;
        }
        FADESTATE_RUN => {
            if (*sFactorySelectScreen).fadeSpeciesNameActive != 0 {
                if (*sFactorySelectScreen).faceSpeciesNameDelay != 0 {
                    gTasks[taskId].data[0] = FADESTATE_DELAY;
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
                        gTasks[taskId].data[0] = FADESTATE_DELAY;
                        (*sFactorySelectScreen).fadeSpeciesNameFadeOut = TRUE;
                    }
                }
            }
        }
        FADESTATE_DELAY => {
            if (*sFactorySelectScreen).faceSpeciesNameDelay > 14 {
                (*sFactorySelectScreen).faceSpeciesNameDelay = 0;
                gTasks[taskId].data[0] = FADESTATE_RUN;
            } else {
                (*sFactorySelectScreen).faceSpeciesNameDelay += 1;
            }
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn Swap_CB2() {
    AnimateSprites();
    BuildOamBuffer();
    RunTextPrinters();
    UpdatePaletteFade();
    RunTasks();
}
pub(crate) unsafe extern "C" fn Swap_VblankCb() {
    LoadOam();
    ProcessSpriteCopyRequests();
    TransferPlttBuffer();
}
pub(crate) unsafe extern "C" fn CopySwappedMonData() {
    let mut friendship: u8 = 0;
    gPlayerParty[(*sFactorySwapScreen).playerMonId] = gEnemyParty[(*sFactorySwapScreen).enemyMonId];
    friendship = 0;
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
pub(crate) unsafe extern "C" fn Swap_Task_OpenSummaryScreen(taskId: u8) {
    match gTasks[taskId].data[0] {
        STATE_SUMMARY_FADE => {
            BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 16, 0);
            gTasks[taskId].data[0] = STATE_SUMMARY_CLEAN;
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
                gTasks[taskId].data[0] = STATE_SUMMARY_SHOW;
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
pub(crate) unsafe extern "C" fn Swap_Task_Exit(taskId: u8) {
    if (*sFactorySwapScreen).monPicAnimating == TRUE {
        return;
    }
    match gTasks[taskId].data[0] {
        0 => {
            if (*sFactorySwapScreen).monSwapped == TRUE {
                gTasks[taskId].data[0] += 1;
                gSpecialVar_Result = FALSE as u16;
            } else {
                gTasks[taskId].data[0] = 2;
                gSpecialVar_Result = TRUE as u16;
            }
        }
        1 => {
            if (*sFactorySwapScreen).monSwapped == TRUE {
                (*sFactorySwapScreen).enemyMonId = (*sFactorySwapScreen).cursorPos;
                CopySwappedMonData();
            }
            gTasks[taskId].data[0] += 1;
        }
        2 => {
            BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 16, 0);
            gTasks[taskId].data[0] += 1;
        }
        3 => {
            if UpdatePaletteFade() == 0 {
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
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn Swap_Task_HandleYesNo(taskId: u8) {
    let mut loPtr: u16 = 0;
    let mut hiPtr: u16 = 0;
    if (*sFactorySwapScreen).monPicAnimating == TRUE {
        return;
    }
    match gTasks[taskId].data[0] {
        STATE_YESNO_SHOW => {
            Swap_ShowYesNoOptions();
            gTasks[taskId].data[0] = STATE_YESNO_HANDLE_INPUT;
        }
        STATE_YESNO_HANDLE_INPUT => {
            if gMain.newKeys as i32 & A_BUTTON != 0 {
                PlaySE(SE_SELECT);
                if (*sFactorySwapScreen).yesNoCursorPos == 0 {
                    gTasks[taskId].data[1] = TRUE as i16;
                    hiPtr = gTasks[taskId].data[6] as u16;
                    loPtr = gTasks[taskId].data[7] as u16;
                    gTasks[taskId].func = core::mem::transmute::<_, Option<unsafe extern "C" fn(u8)>>(
                        ((hiPtr as i32) << 16 | loPtr as i32) as usize as *mut c_void,
                    );
                } else {
                    gTasks[taskId].data[1] = FALSE as i16;
                    Swap_ErasePopupMenu(SWAP_WIN_YES_NO);
                    hiPtr = gTasks[taskId].data[6] as u16;
                    loPtr = gTasks[taskId].data[7] as u16;
                    gTasks[taskId].func = core::mem::transmute::<_, Option<unsafe extern "C" fn(u8)>>(
                        ((hiPtr as i32) << 16 | loPtr as i32) as usize as *mut c_void,
                    );
                }
            } else if gMain.newKeys as i32 & B_BUTTON != 0 {
                PlaySE(SE_SELECT);
                gTasks[taskId].data[1] = FALSE as i16;
                Swap_ErasePopupMenu(SWAP_WIN_YES_NO);
                hiPtr = gTasks[taskId].data[6] as u16;
                loPtr = gTasks[taskId].data[7] as u16;
                gTasks[taskId].func = core::mem::transmute::<_, Option<unsafe extern "C" fn(u8)>>(
                    ((hiPtr as i32) << 16 | loPtr as i32) as usize as *mut c_void,
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
pub(crate) unsafe extern "C" fn Swap_HandleQuitSwappingResponse(taskId: u8) {
    if gTasks[taskId].data[1] == TRUE as i16 {
        gTasks[taskId].data[0] = 0;
        gTasks[taskId].func = Some(Swap_Task_Exit);
    } else {
        gTasks[taskId].data[0] = 0;
        gTasks[taskId].data[6] =
            (Swap_Task_HandleChooseMons as *const () as usize as u32 >> 16) as i16;
        gTasks[taskId].data[7] = Swap_Task_HandleChooseMons as *const () as usize as u32 as i16;
        gTasks[taskId].data[5] = STATE_CHOOSE_MONS_HANDLE_INPUT;
        gTasks[taskId].func = Some(Swap_Task_ScreenInfoTransitionIn);
    }
}
pub(crate) unsafe extern "C" fn Swap_AskQuitSwapping(taskId: u8) {
    if gTasks[taskId].data[0] == 0 {
        Swap_PrintOnInfoWindow(gText_QuitSwapping.as_ptr().cast_mut());
        (*sFactorySwapScreen).monSwapped = FALSE;
        gTasks[taskId].data[0] = STATE_YESNO_SHOW;
        gTasks[taskId].data[6] =
            (Swap_HandleQuitSwappingResponse as *const () as usize as u32 >> 16) as i16;
        gTasks[taskId].data[7] =
            Swap_HandleQuitSwappingResponse as *const () as usize as u32 as i16;
        gTasks[taskId].func = Some(Swap_Task_HandleYesNo);
    }
}
pub(crate) unsafe extern "C" fn Swap_HandleAcceptMonResponse(taskId: u8) {
    CloseMonPic(
        (*sFactorySwapScreen).monPic,
        &raw mut (*sFactorySwapScreen).monPicAnimating,
        TRUE,
    );
    if gTasks[taskId].data[1] == TRUE as i16 {
        gTasks[taskId].data[0] = 0;
        gTasks[taskId].func = Some(Swap_Task_Exit);
    } else {
        gTasks[taskId].data[0] = 0;
        gTasks[taskId].data[6] =
            (Swap_Task_HandleChooseMons as *const () as usize as u32 >> 16) as i16;
        gTasks[taskId].data[7] = Swap_Task_HandleChooseMons as *const () as usize as u32 as i16;
        gTasks[taskId].data[5] = STATE_CHOOSE_MONS_HANDLE_INPUT;
        gTasks[taskId].func = Some(Swap_Task_ScreenInfoTransitionIn);
    }
}
pub(crate) unsafe extern "C" fn Swap_AskAcceptMon(taskId: u8) {
    if gTasks[taskId].data[0] == 0 {
        OpenMonPic(
            &raw mut (*sFactorySwapScreen).monPic.bgSpriteId,
            &raw mut (*sFactorySwapScreen).monPicAnimating,
            TRUE,
        );
        Swap_PrintOnInfoWindow(gText_AcceptThisPkmn.as_ptr().cast_mut());
        (*sFactorySwapScreen).monSwapped = TRUE;
        gTasks[taskId].data[0] = STATE_YESNO_SHOW;
        gTasks[taskId].data[6] =
            (Swap_HandleAcceptMonResponse as *const () as usize as u32 >> 16) as i16;
        gTasks[taskId].data[7] = Swap_HandleAcceptMonResponse as *const () as usize as u32 as i16;
        gTasks[taskId].func = Some(Swap_Task_HandleYesNo);
    }
}
pub(crate) unsafe extern "C" fn Swap_Task_HandleMenu(taskId: u8) {
    match gTasks[taskId].data[0] {
        STATE_MENU_INIT => {
            if (*sFactorySwapScreen).fromSummaryScreen == 0 {
                OpenMonPic(
                    &raw mut (*sFactorySwapScreen).monPic.bgSpriteId,
                    &raw mut (*sFactorySwapScreen).monPicAnimating,
                    TRUE,
                );
            }
            gTasks[taskId].data[0] = STATE_MENU_SHOW_OPTIONS;
        }
        STATE_MENU_SHOW_OPTIONS => {
            if (*sFactorySwapScreen).monPicAnimating != TRUE {
                Swap_ShowMenuOptions();
                gTasks[taskId].data[0] = STATE_MENU_HANDLE_INPUT;
            }
        }
        STATE_MENU_HANDLE_INPUT => {
            if (*sFactorySwapScreen).monPicAnimating != TRUE {
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
                    gTasks[taskId].data[0] = 0;
                    gTasks[taskId].data[6] =
                        (Swap_Task_HandleChooseMons as *const () as usize as u32 >> 16) as i16;
                    gTasks[taskId].data[7] =
                        Swap_Task_HandleChooseMons as *const () as usize as u32 as i16;
                    gTasks[taskId].data[5] = STATE_CHOOSE_MONS_HANDLE_INPUT;
                    gTasks[taskId].func = Some(Swap_Task_ScreenInfoTransitionIn);
                } else if gMain.newAndRepeatedKeys as i32 & DPAD_UP != 0 {
                    Swap_UpdateMenuCursorPosition(-1);
                } else if gMain.newAndRepeatedKeys as i32 & DPAD_DOWN != 0 {
                    Swap_UpdateMenuCursorPosition(1);
                }
            }
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn Swap_Task_HandleChooseMons(taskId: u8) {
    match gTasks[taskId].data[0] {
        STATE_CHOOSE_MONS_INIT => {
            if gPaletteFade.active() == 0 {
                (*sFactorySwapScreen).fadeSpeciesNameActive = TRUE;
                gTasks[taskId].data[0] = STATE_CHOOSE_MONS_HANDLE_INPUT;
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
                gTasks[taskId].data[6] =
                    (Swap_AskQuitSwapping as *const () as usize as u32 >> 16) as i16;
                gTasks[taskId].data[7] = Swap_AskQuitSwapping as *const () as usize as u32 as i16;
                gTasks[taskId].data[0] = 0;
                gTasks[taskId].data[5] = 0;
                gTasks[taskId].func = Some(Swap_Task_ScreenInfoTransitionOut);
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
pub(crate) unsafe extern "C" fn Swap_Task_FadeSpeciesName(taskId: u8) {
    match gTasks[taskId].data[0] {
        FADESTATE_INIT => {
            (*sFactorySwapScreen).fadeSpeciesNameCoeffDelay = 0;
            (*sFactorySwapScreen).fadeSpeciesNameCoeff = 0;
            (*sFactorySwapScreen).fadeSpeciesNameFadeOut = TRUE;
            gTasks[taskId].data[0] = FADESTATE_RUN;
        }
        FADESTATE_RUN => {
            if (*sFactorySwapScreen).fadeSpeciesNameActive != 0 {
                if (*sFactorySwapScreen).faceSpeciesNameDelay != 0 {
                    gTasks[taskId].data[0] = FADESTATE_DELAY;
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
                        gTasks[taskId].data[0] = FADESTATE_DELAY;
                        (*sFactorySwapScreen).fadeSpeciesNameFadeOut = TRUE;
                    }
                }
            }
        }
        FADESTATE_DELAY => {
            if (*sFactorySwapScreen).faceSpeciesNameDelay > 14 {
                (*sFactorySwapScreen).faceSpeciesNameDelay = 0;
                gTasks[taskId].data[0] = FADESTATE_RUN;
            } else {
                (*sFactorySwapScreen).faceSpeciesNameDelay += 1;
            }
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn Swap_Task_FadeOutSpeciesName(taskId: u8) {
    match gTasks[taskId].data[0] {
        0 => {
            (*sFactorySwapScreen).fadeSpeciesNameCoeffDelay = 0;
            gTasks[taskId].data[4] = FALSE as i16;
            gTasks[taskId].data[0] += 1;
        }
        1 => {
            LoadPalette(&raw mut gPlttBufferUnfaded[240] as *mut c_void, 224, 10);
            gTasks[taskId].data[0] += 1;
        }
        2 => {
            if (*sFactorySwapScreen).fadeSpeciesNameCoeff > 15 {
                gTasks[taskId].data[4] = TRUE as i16;
                gTasks[taskId].data[0] += 1;
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
pub(crate) unsafe extern "C" fn Swap_Task_SlideCycleBalls(taskId: u8) {
    let mut i: i8 = 0;
    let mut lastX: u8 = 0;
    let mut finished: u8 = 0;
    match gTasks[taskId].data[0] {
        0 => {
            gTasks[taskId].data[1] = 0;
            gTasks[taskId].data[2] = FALSE as i16;
            gTasks[taskId].data[3] = FALSE as i16;
            gTasks[taskId].data[0] = 1;
        }
        1 => {
            lastX = 0;
            i = 2;
            while i >= 0 {
                if i != 2 {
                    let mut posX: u8 =
                        lastX - gSprites[(*sFactorySwapScreen).ballSpriteIds[i]].x as u8;
                    if posX == 16 || gTasks[taskId].data[i as i32 + 1 + 1] == 1 {
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
                if gTasks[taskId].data[i as i32 + TRUE as i32] == TRUE as i16 {
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
                    gTasks[taskId].data[i as i32 + TRUE as i32] = TRUE as i16;
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
pub(crate) unsafe extern "C" fn Swap_Task_SlideButtonOnOffScreen(taskId: u8) {
    let mut i: u8 = 0;
    let mut j: u8 = 0;
    let mut posX: i32 = 0;
    let mut deltaX: i8 = gTasks[taskId].data[3] as i8;
    let mut sliding: u8 = 0;
    let mut currPosX: i16 = 0;
    let mut prevTaskId: u8 = 0;
    if gTasks[taskId].data[2] == TRUE as i16 {
        deltaX *= -1;
    }
    match gTasks[taskId].data[0] {
        SLIDE_BUTTON_PKMN => {
            currPosX = gSprites[(*sFactorySwapScreen).pkmnForSwapButtonSpriteIds[0][0]].x;
            if gTasks[taskId].data[2] == 0 {
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
                i = 0;
                while i < 3 {
                    j = 0;
                    while j < 2 {
                        gSprites[(*sFactorySwapScreen).pkmnForSwapButtonSpriteIds[j][i]].x +=
                            deltaX as i16;
                        j += 1;
                    }
                    i += 1;
                }
            } else {
                j = 0;
                while j < 2 {
                    gSprites[(*sFactorySwapScreen).pkmnForSwapButtonSpriteIds[j][0]].x =
                        posX as i16;
                    gSprites[(*sFactorySwapScreen).pkmnForSwapButtonSpriteIds[j][1]].x =
                        posX as i16 + 16;
                    gSprites[(*sFactorySwapScreen).pkmnForSwapButtonSpriteIds[j][2]].x =
                        posX as i16 + 48;
                    j += 1;
                }
                prevTaskId = gTasks[taskId].data[1] as u8;
                gTasks[prevTaskId].data[3] = TRUE as i16;
                DestroyTask(taskId);
            }
        }
        SLIDE_BUTTON_CANCEL => {
            currPosX = gSprites[(*sFactorySwapScreen).cancelButtonSpriteIds[0][0]].x;
            if gTasks[taskId].data[2] == 0 {
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
                i = 0;
                while i < 2 {
                    j = 0;
                    while j < 2 {
                        gSprites[(*sFactorySwapScreen).cancelButtonSpriteIds[j][i]].x +=
                            deltaX as i16;
                        j += 1;
                    }
                    i += 1;
                }
            } else {
                j = 0;
                while j < 2 {
                    gSprites[(*sFactorySwapScreen).cancelButtonSpriteIds[j][0]].x = posX as i16;
                    gSprites[(*sFactorySwapScreen).cancelButtonSpriteIds[j][1]].x =
                        posX as i16 + 16;
                    j += 1;
                }
                prevTaskId = gTasks[taskId].data[1] as u8;
                gTasks[prevTaskId].data[4] = TRUE as i16;
                DestroyTask(taskId);
            }
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn Swap_Task_ScreenInfoTransitionOut(taskId: u8) {
    let mut slideTaskId: u8 = 0;
    let mut hiPtr: u16 = 0;
    let mut loPtr: u16 = 0;
    match gTasks[taskId].data[0] {
        0 => {
            LoadPalette(sSwapText_Pal.as_ptr().cast_mut() as *mut c_void, 224, 10);
            Swap_PrintActionStrings();
            PutWindowTilemap(SWAP_WIN_ACTION_FADE);
            gTasks[taskId].data[0] += 1;
        }
        1 => {
            Swap_ErasePopupMenu(SWAP_WIN_OPTIONS);
            gTasks[taskId].data[0] += 1;
        }
        2 => {
            BeginNormalPaletteFade(16384, 0, 0, 16, sPokeballGray_Pal[37]);
            gTasks[taskId].data[0] += 1;
        }
        3 => {
            if gPaletteFade.active() == 0 {
                FillWindowPixelBuffer(SWAP_WIN_ACTION_FADE, 0);
                CopyWindowToVram(SWAP_WIN_ACTION_FADE, COPYWIN_GFX);
                if (*sFactorySwapScreen).inEnemyScreen == TRUE {
                    slideTaskId = CreateTask(Some(Swap_Task_SlideButtonOnOffScreen), 0);
                    gTasks[taskId].data[3] = FALSE as i16;
                    gTasks[slideTaskId].data[1] = taskId as i16;
                    gTasks[slideTaskId].data[0] = SLIDE_BUTTON_PKMN;
                    gTasks[slideTaskId].data[2] = FALSE as i16;
                    gTasks[slideTaskId].data[3] = 6;
                    gTasks[taskId].data[2] = 5;
                    gTasks[taskId].data[0] += 1;
                } else {
                    slideTaskId = CreateTask(Some(Swap_Task_SlideButtonOnOffScreen), 0);
                    gTasks[taskId].data[3] = TRUE as i16;
                    gTasks[taskId].data[4] = FALSE as i16;
                    gTasks[slideTaskId].data[1] = taskId as i16;
                    gTasks[slideTaskId].data[0] = SLIDE_BUTTON_CANCEL;
                    gTasks[slideTaskId].data[2] = FALSE as i16;
                    gTasks[slideTaskId].data[3] = 6;
                    gTasks[taskId].data[0] += 2;
                }
            }
        }
        4 => {
            if gTasks[taskId].data[2] == 0 {
                slideTaskId = CreateTask(Some(Swap_Task_SlideButtonOnOffScreen), 0);
                gTasks[taskId].data[4] = FALSE as i16;
                gTasks[slideTaskId].data[1] = taskId as i16;
                gTasks[slideTaskId].data[0] = SLIDE_BUTTON_CANCEL;
                gTasks[slideTaskId].data[2] = FALSE as i16;
                gTasks[slideTaskId].data[3] = 6;
                gTasks[taskId].data[0] += 1;
            } else {
                gTasks[taskId].data[2] -= 1;
            }
        }
        5 => {
            if gTasks[taskId].data[3] == TRUE as i16 && gTasks[taskId].data[4] == TRUE as i16 {
                gTasks[taskId].data[0] = gTasks[taskId].data[5];
                hiPtr = gTasks[taskId].data[6] as u16;
                loPtr = gTasks[taskId].data[7] as u16;
                gTasks[taskId].func = core::mem::transmute::<_, Option<unsafe extern "C" fn(u8)>>(
                    ((hiPtr as i32) << 16 | loPtr as i32) as usize as *mut c_void,
                );
            }
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn Swap_Task_ScreenInfoTransitionIn(taskId: u8) {
    let mut slideTaskId: u8 = 0;
    let mut hiPtr: u16 = 0;
    let mut loPtr: u16 = 0;
    if (*sFactorySwapScreen).monPicAnimating == TRUE {
        return;
    }
    match gTasks[taskId].data[0] {
        0 => {
            if (*sFactorySwapScreen).inEnemyScreen == TRUE {
                slideTaskId = CreateTask(Some(Swap_Task_SlideButtonOnOffScreen), 0);
                gTasks[taskId].data[3] = FALSE as i16;
                gTasks[slideTaskId].data[1] = taskId as i16;
                gTasks[slideTaskId].data[0] = SLIDE_BUTTON_PKMN;
                gTasks[slideTaskId].data[2] = TRUE as i16;
                gTasks[slideTaskId].data[3] = 6;
                gTasks[taskId].data[2] = 10;
                gTasks[taskId].data[0] += 1;
            } else {
                slideTaskId = CreateTask(Some(Swap_Task_SlideButtonOnOffScreen), 0);
                gTasks[taskId].data[3] = TRUE as i16;
                gTasks[taskId].data[4] = FALSE as i16;
                gTasks[slideTaskId].data[1] = taskId as i16;
                gTasks[slideTaskId].data[0] = SLIDE_BUTTON_CANCEL;
                gTasks[slideTaskId].data[2] = TRUE as i16;
                gTasks[slideTaskId].data[3] = 6;
                gTasks[taskId].data[0] += 2;
            }
        }
        1 => {
            if gTasks[taskId].data[2] == 0 {
                slideTaskId = CreateTask(Some(Swap_Task_SlideButtonOnOffScreen), 0);
                gTasks[taskId].data[4] = FALSE as i16;
                gTasks[slideTaskId].data[1] = taskId as i16;
                gTasks[slideTaskId].data[0] = SLIDE_BUTTON_CANCEL;
                gTasks[slideTaskId].data[2] = TRUE as i16;
                gTasks[slideTaskId].data[3] = 6;
                gTasks[taskId].data[0] += 1;
            } else {
                gTasks[taskId].data[2] -= 1;
            }
        }
        2 => {
            if gTasks[taskId].data[3] == TRUE as i16 && gTasks[taskId].data[4] == TRUE as i16 {
                gPlttBufferFaded[226] = sPokeballGray_Pal[37];
                Swap_PrintActionStrings();
                PutWindowTilemap(SWAP_WIN_ACTION_FADE);
                gTasks[taskId].data[0] += 1;
            }
        }
        3 => {
            BeginNormalPaletteFade(16384, 0, 16, 0, sPokeballGray_Pal[37]);
            gTasks[taskId].data[0] += 1;
        }
        4 => {
            if gPaletteFade.active() == 0 {
                Swap_PrintOneActionString(0);
                gTasks[taskId].data[0] += 1;
            }
        }
        5 => {
            Swap_PrintOneActionString(1);
            PutWindowTilemap(SWAP_WIN_OPTIONS);
            gTasks[taskId].data[0] += 1;
        }
        6 => {
            FillWindowPixelBuffer(SWAP_WIN_ACTION_FADE, 0);
            CopyWindowToVram(SWAP_WIN_ACTION_FADE, COPYWIN_GFX);
            gTasks[taskId].data[0] += 1;
        }
        7 => {
            if (*sFactorySwapScreen).inEnemyScreen == 0 {
                Swap_PrintOnInfoWindow(gText_SelectPkmnToSwap.as_ptr().cast_mut());
            } else {
                Swap_PrintOnInfoWindow(gText_SelectPkmnToAccept.as_ptr().cast_mut());
            }
            if (*sFactorySwapScreen).cursorPos < FRONTIER_PARTY_SIZE as u8 {
                gSprites[(*sFactorySwapScreen).cursorSpriteId].set_invisible(FALSE as u16);
            }
            Swap_PrintMonCategory();
            gTasks[taskId].data[0] += 1;
        }
        8 => {
            Swap_PrintMonSpeciesForTransition();
            Swap_EraseSpeciesAtFadeWindow();
            (*sFactorySwapScreen).fadeSpeciesNameActive = TRUE;
            gTasks[taskId].data[0] = gTasks[taskId].data[5];
            hiPtr = gTasks[taskId].data[6] as u16;
            loPtr = gTasks[taskId].data[7] as u16;
            gTasks[taskId].func = core::mem::transmute::<_, Option<unsafe extern "C" fn(u8)>>(
                ((hiPtr as i32) << 16 | loPtr as i32) as usize as *mut c_void,
            );
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn Swap_Task_SwitchPartyScreen(taskId: u8) {
    let mut i: u8 = 0;
    if (*sFactorySwapScreen).monPicAnimating == TRUE {
        return;
    }
    match gTasks[taskId].data[0] {
        0 => {
            Swap_PrintMonSpeciesForTransition();
            gTasks[taskId].data[0] += 1;
        }
        1 => {
            Swap_EraseSpeciesAtFadeWindow();
            gSprites[(*sFactorySwapScreen).cursorSpriteId].set_invisible(TRUE as u16);
            gTasks[taskId].data[0] += 1;
        }
        2 => {
            CreateTask(Some(Swap_Task_SlideCycleBalls), 0);
            gTasks[(*sFactorySwapScreen).fadeSpeciesNameTaskId].func =
                Some(Swap_Task_FadeOutSpeciesName);
            gTasks[taskId].data[0] += 1;
        }
        3 => {
            if FuncIsActiveTask(Some(Swap_Task_SlideCycleBalls)) == 0
                && gTasks[(*sFactorySwapScreen).fadeSpeciesNameTaskId].data[4] == TRUE as i16
            {
                Swap_EraseSpeciesWindow();
                if (*sFactorySwapScreen).inEnemyScreen == 0 {
                    Swap_InitActions(SWAP_ENEMY_SCREEN);
                } else {
                    Swap_InitActions(SWAP_PLAYER_SCREEN);
                    i = 0;
                    while i < 3 {
                        gSprites[(*sFactorySwapScreen).pkmnForSwapButtonSpriteIds[1][i]]
                            .set_invisible(1);
                        i += 1;
                    }
                }
                gSprites[(*sFactorySwapScreen).cursorSpriteId].x = gSprites
                    [(*sFactorySwapScreen).ballSpriteIds[(*sFactorySwapScreen).cursorPos]]
                    .x;
                gTasks[(*sFactorySwapScreen).fadeSpeciesNameTaskId].func =
                    Some(Swap_Task_FadeSpeciesName);
                (*sFactorySwapScreen).fadeSpeciesNameCoeffDelay = 0;
                (*sFactorySwapScreen).fadeSpeciesNameCoeff = 6;
                (*sFactorySwapScreen).fadeSpeciesNameFadeOut = FALSE;
                gTasks[(*sFactorySwapScreen).fadeSpeciesNameTaskId].data[0] = FADESTATE_RUN;
                gTasks[taskId].data[0] += 1;
            }
        }
        4 => {
            gTasks[taskId].data[0] = 0;
            gTasks[taskId].data[6] =
                (Swap_Task_HandleChooseMons as *const () as usize as u32 >> 16) as i16;
            gTasks[taskId].data[7] = Swap_Task_HandleChooseMons as *const () as usize as u32 as i16;
            gTasks[taskId].data[5] = STATE_CHOOSE_MONS_HANDLE_INPUT;
            gTasks[taskId].func = Some(Swap_Task_ScreenInfoTransitionIn);
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn Swap_InitStruct() {
    if sFactorySwapScreen.is_null() {
        sFactorySwapScreen = AllocZeroed(52) as *mut FactorySwapScreen;
        (*sFactorySwapScreen).cursorPos = 0;
        (*sFactorySwapScreen).monPicAnimating = FALSE;
        (*sFactorySwapScreen).fromSummaryScreen = FALSE;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DoBattleFactorySwapScreen() {
    sFactorySwapScreen = null_mut();
    SetMainCallback2(Some(CB2_InitSwapScreen));
}
pub(crate) unsafe extern "C" fn CB2_InitSwapScreen() {
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
                gFrontierFactoryMenu_Gfx.as_ptr().cast_mut() as *mut c_void,
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
                gFrontierFactoryMenu_Tilemap.as_ptr().cast_mut() as *mut c_void,
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
                gFrontierFactoryMenu_Pal.as_ptr().cast_mut() as *mut c_void,
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
            Swap_PrintOnInfoWindow(gText_SelectPkmnToSwap.as_ptr().cast_mut());
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
                gTasks[(*sFactorySwapScreen).fadeSpeciesNameTaskId].data[0] = FADESTATE_INIT;
                taskId = CreateTask(Some(Swap_Task_HandleChooseMons), 0);
                gTasks[taskId].data[0] = STATE_CHOOSE_MONS_INIT;
            } else {
                Swap_EraseActionFadeWindow();
                gTasks[(*sFactorySwapScreen).fadeSpeciesNameTaskId].data[0] = FADESTATE_RUN;
                (*sFactorySwapScreen).fadeSpeciesNameActive = FALSE;
                taskId = CreateTask(Some(Swap_Task_HandleMenu), 0);
                gTasks[taskId].data[0] = STATE_MENU_INIT;
            }
            SetMainCallback2(Some(Swap_CB2));
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn Swap_InitAllSprites() {
    let mut i: u8 = 0;
    let mut x: u8 = 0;
    let mut spriteTemplate: SpriteTemplate = zeroed();
    spriteTemplate = *sSpriteTemplate_Swap_Pokeball;
    spriteTemplate.paletteTag = PALTAG_BALL_SELECTED;
    i = 0;
    while i < FRONTIER_PARTY_SIZE as u8 {
        (*sFactorySwapScreen).ballSpriteIds[i] =
            CreateSprite(&raw mut spriteTemplate, 48 * i as i16 + 72, 64, 1);
        gSprites[(*sFactorySwapScreen).ballSpriteIds[i]].data[0] = 0;
        i += 1;
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
    i = 0;
    while i < 2 {
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
        i += 1;
    }
    gSprites[(*sFactorySwapScreen).cancelButtonSpriteIds[0][0]].set_invisible(0);
    gSprites[(*sFactorySwapScreen).cancelButtonSpriteIds[0][1]].set_invisible(0);
    gSprites[(*sFactorySwapScreen).pkmnForSwapButtonSpriteIds[0][0]].set_invisible(0);
    gSprites[(*sFactorySwapScreen).pkmnForSwapButtonSpriteIds[0][1]].set_invisible(0);
    gSprites[(*sFactorySwapScreen).pkmnForSwapButtonSpriteIds[0][2]].set_invisible(0);
}
pub(crate) unsafe extern "C" fn Swap_DestroyAllSprites() {
    let mut i: u8 = 0;
    let mut j: u8 = 0;
    i = 0;
    while i < FRONTIER_PARTY_SIZE as u8 {
        DestroySprite(&raw mut gSprites[(*sFactorySwapScreen).ballSpriteIds[i]]);
        i += 1;
    }
    DestroySprite(&raw mut gSprites[(*sFactorySwapScreen).cursorSpriteId]);
    DestroySprite(&raw mut gSprites[(*sFactorySwapScreen).menuCursor1SpriteId]);
    DestroySprite(&raw mut gSprites[(*sFactorySwapScreen).menuCursor2SpriteId]);
    i = 0;
    while i < 2 {
        j = 0;
        while j < 3 {
            DestroySprite(
                &raw mut gSprites[(*sFactorySwapScreen).pkmnForSwapButtonSpriteIds[i][j]],
            );
            j += 1;
        }
        i += 1;
    }
    i = 0;
    while i < 2 {
        j = 0;
        while j < 2 {
            DestroySprite(&raw mut gSprites[(*sFactorySwapScreen).cancelButtonSpriteIds[i][j]]);
            j += 1;
        }
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn Swap_HandleActionCursorChange(cursorId: u8) {
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
pub(crate) unsafe extern "C" fn Swap_UpdateBallCursorPosition(direction: i8) {
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
pub(crate) unsafe extern "C" fn Swap_UpdateActionCursorPosition(direction: i8) {
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
pub(crate) unsafe extern "C" fn Swap_UpdateYesNoCursorPosition(direction: i8) {
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
pub(crate) unsafe extern "C" fn Swap_UpdateMenuCursorPosition(direction: i8) {
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
pub(crate) unsafe extern "C" fn Swap_HighlightActionButton(actionId: u8) {
    let mut i: u8 = 0;
    i = 0;
    while i < 3 {
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
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn Swap_HideActionButtonHighlights() {
    let mut i: u8 = 0;
    i = 0;
    while i < 3 {
        gSprites[(*sFactorySwapScreen).pkmnForSwapButtonSpriteIds[1][i]].set_invisible(1);
        if i < 2 {
            gSprites[(*sFactorySwapScreen).cancelButtonSpriteIds[1][i]].set_invisible(1);
        }
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn Swap_ShowMenuOptions() {
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
pub(crate) unsafe extern "C" fn Swap_ShowYesNoOptions() {
    (*sFactorySwapScreen).yesNoCursorPos = 0;
    gSprites[(*sFactorySwapScreen).menuCursor1SpriteId].x = 176;
    gSprites[(*sFactorySwapScreen).menuCursor1SpriteId].y = 112;
    gSprites[(*sFactorySwapScreen).menuCursor2SpriteId].x = 208;
    gSprites[(*sFactorySwapScreen).menuCursor2SpriteId].y = 112;
    gSprites[(*sFactorySwapScreen).menuCursor1SpriteId].set_invisible(FALSE as u16);
    gSprites[(*sFactorySwapScreen).menuCursor2SpriteId].set_invisible(FALSE as u16);
    Swap_PrintYesNoOptions();
}
pub(crate) unsafe extern "C" fn Swap_ErasePopupMenu(windowId: u8) {
    gSprites[(*sFactorySwapScreen).menuCursor1SpriteId].set_invisible(TRUE as u16);
    gSprites[(*sFactorySwapScreen).menuCursor2SpriteId].set_invisible(TRUE as u16);
    FillWindowPixelBuffer(windowId, 0);
    CopyWindowToVram(windowId, COPYWIN_GFX);
    ClearWindowTilemap(windowId);
}
pub(crate) unsafe extern "C" fn Swap_EraseSpeciesWindow() {
    PutWindowTilemap(SWAP_WIN_SPECIES);
    FillWindowPixelBuffer(SWAP_WIN_SPECIES, 0);
    CopyWindowToVram(SWAP_WIN_SPECIES, COPYWIN_GFX);
}
pub(crate) unsafe extern "C" fn Swap_EraseSpeciesAtFadeWindow() {
    PutWindowTilemap(SWAP_WIN_SPECIES_AT_FADE);
    FillWindowPixelBuffer(SWAP_WIN_SPECIES_AT_FADE, 0);
    CopyWindowToVram(SWAP_WIN_SPECIES_AT_FADE, COPYWIN_GFX);
}
pub(crate) unsafe extern "C" fn Swap_EraseActionFadeWindow() {
    Swap_EraseSpeciesWindow();
    PutWindowTilemap(SWAP_WIN_ACTION_FADE);
    FillWindowPixelBuffer(SWAP_WIN_ACTION_FADE, 0);
    CopyWindowToVram(SWAP_WIN_ACTION_FADE, COPYWIN_GFX);
}
pub(crate) unsafe extern "C" fn Swap_PrintPkmnSwap() {
    FillWindowPixelBuffer(SWAP_WIN_TITLE, 17);
    AddTextPrinterParameterized(
        SWAP_WIN_TITLE,
        FONT_NORMAL,
        gText_PkmnSwap.as_ptr().cast_mut(),
        2,
        1,
        0,
        None,
    );
    CopyWindowToVram(SWAP_WIN_TITLE, COPYWIN_FULL);
}
pub(crate) unsafe extern "C" fn Swap_PrintMonSpecies() {
    let mut species: u16 = 0;
    let mut x: u8 = 0;
    FillWindowPixelBuffer(SWAP_WIN_SPECIES, 0);
    if (*sFactorySwapScreen).cursorPos >= FRONTIER_PARTY_SIZE as u8 {
        CopyWindowToVram(SWAP_WIN_SPECIES, COPYWIN_GFX);
    } else {
        let mut monId: u8 = (*sFactorySwapScreen).cursorPos;
        if (*sFactorySwapScreen).inEnemyScreen == 0 {
            species =
                GetMonData3(&raw mut gPlayerParty[monId], MON_DATA_SPECIES, null_mut()) as u16;
        } else {
            species = GetMonData3(&raw mut gEnemyParty[monId], MON_DATA_SPECIES, null_mut()) as u16;
        }
        StringCopy(
            gStringVar4.as_mut_ptr(),
            gSpeciesNames[species].as_ptr().cast_mut(),
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
pub(crate) unsafe extern "C" fn Swap_PrintOnInfoWindow(str: *mut u8) {
    FillWindowPixelBuffer(SWAP_WIN_INFO, 0);
    AddTextPrinterParameterized(SWAP_WIN_INFO, FONT_NORMAL, str, 2, 5, 0, None);
    CopyWindowToVram(SWAP_WIN_INFO, COPYWIN_GFX);
}
pub(crate) unsafe extern "C" fn Swap_PrintMenuOptions() {
    PutWindowTilemap(SWAP_WIN_OPTIONS);
    FillWindowPixelBuffer(SWAP_WIN_OPTIONS, 0);
    AddTextPrinterParameterized3(
        SWAP_WIN_OPTIONS,
        FONT_NORMAL,
        15,
        1,
        sSwapMenuOptionsTextColors.as_ptr().cast_mut(),
        0,
        gText_Summary2.as_ptr().cast_mut(),
    );
    AddTextPrinterParameterized3(
        SWAP_WIN_OPTIONS,
        FONT_NORMAL,
        15,
        17,
        sSwapMenuOptionsTextColors.as_ptr().cast_mut(),
        0,
        gText_Swap.as_ptr().cast_mut(),
    );
    AddTextPrinterParameterized3(
        SWAP_WIN_OPTIONS,
        FONT_NORMAL,
        15,
        33,
        sSwapMenuOptionsTextColors.as_ptr().cast_mut(),
        0,
        gText_Rechoose.as_ptr().cast_mut(),
    );
    CopyWindowToVram(SWAP_WIN_OPTIONS, COPYWIN_FULL);
}
pub(crate) unsafe extern "C" fn Swap_PrintYesNoOptions() {
    PutWindowTilemap(SWAP_WIN_YES_NO);
    FillWindowPixelBuffer(SWAP_WIN_YES_NO, 0);
    AddTextPrinterParameterized3(
        SWAP_WIN_YES_NO,
        FONT_NORMAL,
        7,
        1,
        sSwapMenuOptionsTextColors.as_ptr().cast_mut(),
        0,
        gText_Yes3.as_ptr().cast_mut(),
    );
    AddTextPrinterParameterized3(
        SWAP_WIN_YES_NO,
        FONT_NORMAL,
        7,
        17,
        sSwapMenuOptionsTextColors.as_ptr().cast_mut(),
        0,
        gText_No3.as_ptr().cast_mut(),
    );
    CopyWindowToVram(SWAP_WIN_YES_NO, COPYWIN_FULL);
}
pub(crate) unsafe extern "C" fn Swap_PrintActionString(str: *mut u8, y: u32, windowId: u32) {
    let mut x: i32 = GetStringRightAlignXOffset(FONT_SMALL as i32, str, 70);
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
pub(crate) unsafe extern "C" fn Swap_PrintActionStrings() {
    FillWindowPixelBuffer(SWAP_WIN_ACTION_FADE, 0);
    'l1: {
        let sw1: u8 = (*sFactorySwapScreen).inEnemyScreen;
        let mut fall = false;
        if sw1 == TRUE {
            fall = true;
            Swap_PrintActionString(
                gText_PkmnForSwap.as_ptr().cast_mut(),
                0,
                SWAP_WIN_ACTION_FADE as u32,
            );
        }
        if fall || sw1 == FALSE {
            fall = true;
            Swap_PrintActionString(
                gText_Cancel3.as_ptr().cast_mut(),
                24,
                SWAP_WIN_ACTION_FADE as u32,
            );
            break 'l1;
        }
    }
    CopyWindowToVram(SWAP_WIN_ACTION_FADE, COPYWIN_FULL);
}
pub(crate) unsafe extern "C" fn Swap_PrintActionStrings2() {
    FillWindowPixelBuffer(SWAP_WIN_OPTIONS, 0);
    'l1: {
        let sw1: u8 = (*sFactorySwapScreen).inEnemyScreen;
        let mut fall = false;
        if sw1 == TRUE {
            fall = true;
            Swap_PrintActionString(
                gText_PkmnForSwap.as_ptr().cast_mut(),
                8,
                SWAP_WIN_OPTIONS as u32,
            );
        }
        if fall || sw1 == FALSE {
            fall = true;
            Swap_PrintActionString(
                gText_Cancel3.as_ptr().cast_mut(),
                32,
                SWAP_WIN_OPTIONS as u32,
            );
            break 'l1;
        }
    }
    CopyWindowToVram(SWAP_WIN_OPTIONS, COPYWIN_FULL);
}
pub(crate) unsafe extern "C" fn Swap_PrintOneActionString(which: u8) {
    match which {
        0 => {
            if (*sFactorySwapScreen).inEnemyScreen == TRUE {
                Swap_PrintActionString(
                    gText_PkmnForSwap.as_ptr().cast_mut(),
                    8,
                    SWAP_WIN_OPTIONS as u32,
                );
            }
        }
        1 => {
            Swap_PrintActionString(
                gText_Cancel3.as_ptr().cast_mut(),
                32,
                SWAP_WIN_OPTIONS as u32,
            );
        }
        _ => {}
    }
    CopyWindowToVram(SWAP_WIN_OPTIONS, COPYWIN_FULL);
}
pub(crate) unsafe extern "C" fn Swap_PrintMonSpeciesAtFade() {
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
        let mut monId: u8 = (*sFactorySwapScreen).cursorPos;
        if (*sFactorySwapScreen).inEnemyScreen == 0 {
            species =
                GetMonData3(&raw mut gPlayerParty[monId], MON_DATA_SPECIES, null_mut()) as u16;
        } else {
            species = GetMonData3(&raw mut gEnemyParty[monId], MON_DATA_SPECIES, null_mut()) as u16;
        }
        StringCopy(
            gStringVar4.as_mut_ptr(),
            gSpeciesNames[species].as_ptr().cast_mut(),
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
pub(crate) unsafe extern "C" fn Swap_PrintMonSpeciesForTransition() {
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
        let mut monId: u8 = (*sFactorySwapScreen).cursorPos;
        if (*sFactorySwapScreen).inEnemyScreen == 0 {
            species =
                GetMonData3(&raw mut gPlayerParty[monId], MON_DATA_SPECIES, null_mut()) as u16;
        } else {
            species = GetMonData3(&raw mut gEnemyParty[monId], MON_DATA_SPECIES, null_mut()) as u16;
        }
        StringCopy(
            gStringVar4.as_mut_ptr(),
            gSpeciesNames[species].as_ptr().cast_mut(),
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
pub(crate) unsafe extern "C" fn Swap_PrintMonCategory() {
    let mut species: u16 = 0;
    let mut text: CArray<u8, 30> = zeroed();
    let mut x: u8 = 0;
    let mut monId: u8 = (*sFactorySwapScreen).cursorPos;
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
pub(crate) unsafe extern "C" fn Swap_InitActions(id: u8) {
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
pub(crate) unsafe extern "C" fn Swap_RunMenuOptionFunc(taskId: u8) {
    sSwap_CurrentOptionFunc = sSwap_MenuOptionFuncs[(*sFactorySwapScreen).menuCursorPos];
    sSwap_CurrentOptionFunc.unwrap_unchecked()(taskId);
}
pub(crate) unsafe extern "C" fn Swap_OptionSwap(taskId: u8) {
    CloseMonPic(
        (*sFactorySwapScreen).monPic,
        &raw mut (*sFactorySwapScreen).monPicAnimating,
        TRUE,
    );
    (*sFactorySwapScreen).playerMonId = (*sFactorySwapScreen).cursorPos;
    Swap_ErasePopupMenu(SWAP_WIN_OPTIONS);
    gTasks[taskId].data[0] = 0;
    gTasks[taskId].func = Some(Swap_Task_SwitchPartyScreen);
}
pub(crate) unsafe extern "C" fn Swap_OptionSummary(taskId: u8) {
    gTasks[taskId].data[0] = STATE_SUMMARY_FADE;
    gTasks[taskId].func = Some(Swap_Task_OpenSummaryScreen);
}
pub(crate) unsafe extern "C" fn Swap_OptionRechoose(taskId: u8) {
    CloseMonPic(
        (*sFactorySwapScreen).monPic,
        &raw mut (*sFactorySwapScreen).monPicAnimating,
        TRUE,
    );
    Swap_ErasePopupMenu(SWAP_WIN_OPTIONS);
    gTasks[taskId].data[0] = 0;
    gTasks[taskId].data[6] = (Swap_Task_HandleChooseMons as *const () as usize as u32 >> 16) as i16;
    gTasks[taskId].data[7] = Swap_Task_HandleChooseMons as *const () as usize as u32 as i16;
    gTasks[taskId].data[5] = STATE_CHOOSE_MONS_HANDLE_INPUT;
    gTasks[taskId].func = Some(Swap_Task_ScreenInfoTransitionIn);
}
pub(crate) unsafe extern "C" fn Swap_RunActionFunc(taskId: u8) {
    sSwap_CurrentOptionFunc = (*(*sFactorySwapScreen)
        .actionsData
        .at((*sFactorySwapScreen).cursorPos))
    .func;
    sSwap_CurrentOptionFunc.unwrap_unchecked()(taskId);
}
pub(crate) unsafe extern "C" fn Swap_ActionCancel(taskId: u8) {
    gTasks[taskId].data[6] = (Swap_AskQuitSwapping as *const () as usize as u32 >> 16) as i16;
    gTasks[taskId].data[7] = Swap_AskQuitSwapping as *const () as usize as u32 as i16;
    gTasks[taskId].data[0] = 0;
    gTasks[taskId].data[5] = 0;
    gTasks[taskId].func = Some(Swap_Task_ScreenInfoTransitionOut);
}
pub(crate) unsafe extern "C" fn Swap_ActionPkmnForSwap(taskId: u8) {
    gTasks[taskId].data[6] =
        (Swap_Task_SwitchPartyScreen as *const () as usize as u32 >> 16) as i16;
    gTasks[taskId].data[7] = Swap_Task_SwitchPartyScreen as *const () as usize as u32 as i16;
    gTasks[taskId].data[5] = 0;
    gTasks[taskId].data[0] = 0;
    gTasks[taskId].func = Some(Swap_Task_ScreenInfoTransitionOut);
}
pub(crate) unsafe extern "C" fn Swap_ActionMon(taskId: u8) {
    if (*sFactorySwapScreen).inEnemyScreen == 0 {
        gTasks[taskId].data[6] = (Swap_Task_HandleMenu as *const () as usize as u32 >> 16) as i16;
        gTasks[taskId].data[7] = Swap_Task_HandleMenu as *const () as usize as u32 as i16;
        gTasks[taskId].data[5] = STATE_MENU_INIT;
    } else if Swap_AlreadyHasSameSpecies((*sFactorySwapScreen).cursorPos) == TRUE {
        OpenMonPic(
            &raw mut (*sFactorySwapScreen).monPic.bgSpriteId,
            &raw mut (*sFactorySwapScreen).monPicAnimating,
            TRUE,
        );
        gTasks[taskId].data[0] = 0;
        gTasks[taskId].data[5] = STATE_CHOOSE_MONS_HANDLE_INPUT;
        gTasks[taskId].func = Some(Swap_TaskCantHaveSameMons);
        return;
    } else {
        gTasks[taskId].data[6] = (Swap_AskAcceptMon as *const () as usize as u32 >> 16) as i16;
        gTasks[taskId].data[7] = Swap_AskAcceptMon as *const () as usize as u32 as i16;
        gTasks[taskId].data[5] = 0;
    }
    gTasks[taskId].data[0] = 0;
    gTasks[taskId].func = Some(Swap_Task_ScreenInfoTransitionOut);
}
pub(crate) unsafe extern "C" fn OpenMonPic(spriteId: *mut u8, animating: *mut u8, swapScreen: u8) {
    *spriteId = CreateSprite(
        (&raw const *sSpriteTemplate_Swap_MonPicBgAnim).cast_mut(),
        120,
        64,
        1,
    );
    gSprites[*spriteId].callback = Some(SpriteCB_OpenMonPic);
    gSprites[*spriteId].data[7] = swapScreen as i16;
    *animating = TRUE;
}
pub(crate) unsafe extern "C" fn Swap_ShowSummaryMonSprite() {
    let mut mon: *mut Pokemon = null_mut();
    let mut species: u16 = 0;
    let mut personality: u32 = 0;
    let mut otId: u32 = 0;
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
    mon = &raw mut gPlayerParty[(*sFactorySwapScreen).cursorPos];
    species = GetMonData3(mon, MON_DATA_SPECIES, null_mut()) as u16;
    personality = GetMonData3(mon, MON_DATA_PERSONALITY, null_mut());
    otId = GetMonData3(mon, MON_DATA_OT_ID, null_mut());
    (*sFactorySwapScreen).monPic.monSpriteId =
        CreateMonPicSprite_HandleDeoxys(species, personality, otId, TRUE, 88, 32, 15, TAG_NONE)
            as u8;
    gSprites[(*sFactorySwapScreen).monPic.monSpriteId].centerToCornerVecX = 0;
    gSprites[(*sFactorySwapScreen).monPic.monSpriteId].centerToCornerVecY = 0;
    gSprites[(*sFactorySwapScreen).monPic.bgSpriteId].set_invisible(TRUE as u16);
}
pub(crate) unsafe extern "C" fn CloseMonPic(
    mut pic: FactoryMonPic,
    animating: *mut u8,
    swapScreen: u8,
) {
    let mut taskId: u8 = 0;
    FreeAndDestroyMonPicSprite(pic.monSpriteId as u16);
    taskId = CreateTask(Some(Task_CloseMonPic), 1);
    gTasks[taskId].data[7] = swapScreen as i16;
    gTasks[taskId].data[6] = pic.bgSpriteId as i16;
    gTasks[taskId].func.unwrap_unchecked()(taskId);
    *animating = TRUE;
}
pub(crate) unsafe extern "C" fn HideMonPic(mut pic: FactoryMonPic, animating: *mut u8) {
    FreeAndDestroyMonPicSprite(pic.monSpriteId as u16);
    FreeOamMatrix(gSprites[pic.bgSpriteId].oam.matrixNum() as u8);
    DestroySprite(&raw mut gSprites[pic.bgSpriteId]);
    *animating = FALSE;
}
pub(crate) unsafe extern "C" fn Swap_TaskCantHaveSameMons(taskId: u8) {
    if (*sFactorySwapScreen).monPicAnimating == TRUE {
        return;
    }
    match gTasks[taskId].data[0] {
        0 => {
            Swap_PrintOnInfoWindow(gText_SamePkmnInPartyAlready.as_ptr().cast_mut());
            (*sFactorySwapScreen).monSwapped = FALSE;
            gTasks[taskId].data[0] += 1;
        }
        1 => {
            if gMain.newKeys as i32 & A_BUTTON != 0 || gMain.newKeys as i32 & B_BUTTON != 0 {
                PlaySE(SE_SELECT);
                CloseMonPic(
                    (*sFactorySwapScreen).monPic,
                    &raw mut (*sFactorySwapScreen).monPicAnimating,
                    TRUE,
                );
                gTasks[taskId].data[0] += 1;
            }
        }
        2 => {
            if (*sFactorySwapScreen).monPicAnimating != TRUE {
                FillWindowPixelBuffer(SWAP_WIN_ACTION_FADE, 0);
                CopyWindowToVram(SWAP_WIN_ACTION_FADE, COPYWIN_GFX);
                gTasks[taskId].data[0] += 1;
            }
        }
        3 => {
            Swap_PrintOnInfoWindow(gText_SelectPkmnToAccept.as_ptr().cast_mut());
            gTasks[taskId].data[0] += 1;
        }
        4 => {
            Swap_PrintMonSpeciesForTransition();
            Swap_EraseSpeciesAtFadeWindow();
            (*sFactorySwapScreen).fadeSpeciesNameActive = TRUE;
            gTasks[taskId].data[0] = gTasks[taskId].data[5];
            gTasks[taskId].func = Some(Swap_Task_HandleChooseMons);
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn Swap_AlreadyHasSameSpecies(monId: u8) -> u8 {
    let mut i: u8 = 0;
    let mut species: u16 =
        GetMonData3(&raw mut gEnemyParty[monId], MON_DATA_SPECIES, null_mut()) as u16;
    i = 0;
    while i < FRONTIER_PARTY_SIZE as u8 {
        if i != (*sFactorySwapScreen).playerMonId
            && GetMonData3(&raw mut gPlayerParty[i], MON_DATA_SPECIES, null_mut()) as u16 == species
        {
            return TRUE;
        }
        i += 1;
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn SpriteCB_OpenMonPic(sprite: *mut Sprite) {
    let mut taskId: u8 = 0;
    if (*sprite).affineAnimEnded() != 0 {
        (*sprite).set_invisible(TRUE as u16);
        taskId = CreateTask(Some(Task_OpenMonPic), 1);
        gTasks[taskId].data[7] = (*sprite).data[7];
        gTasks[taskId].func.unwrap_unchecked()(taskId);
        (*sprite).callback = Some(SpriteCallbackDummy);
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_CloseMonPic(sprite: *mut Sprite) {
    if (*sprite).affineAnimEnded() != 0 {
        FreeOamMatrix((*sprite).oam.matrixNum() as u8);
        if (*sprite).data[7] == TRUE as i16 {
            (*sFactorySwapScreen).monPicAnimating = FALSE;
        } else {
            Select_SetMonPicAnimating(FALSE);
        }
        DestroySprite(sprite);
    }
}
pub(crate) unsafe extern "C" fn Task_OpenMonPic(taskId: u8) {
    let mut task: *mut Task = &raw mut gTasks[taskId];
    match (*task).data[0] {
        0 => {
            (*task).data[3] = 88;
            (*task).data[4] = 152;
            (*task).data[5] = 64;
            (*task).data[8] = 65;
            SetGpuRegBits(REG_OFFSET_DISPCNT, DISPCNT_WIN0_ON);
            SetGpuReg(
                REG_OFFSET_WIN0H,
                ((*task).data[3] as u16) << 8 | (*task).data[4] as u16,
            );
            SetGpuReg(
                REG_OFFSET_WIN0V,
                ((*task).data[5] as u16) << 8 | (*task).data[8] as u16,
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
            (*task).data[5] -= 4;
            (*task).data[8] += 4;
            if (*task).data[5] <= 32 || (*task).data[8] >= 96 {
                (*task).data[5] = 32;
                (*task).data[8] = 96;
            }
            SetGpuReg(
                REG_OFFSET_WIN0V,
                ((*task).data[5] as u16) << 8 | (*task).data[8] as u16,
            );
            if (*task).data[5] != 32 {
                return;
            }
        }
        _ => {
            DestroyTask(taskId);
            if gTasks[taskId].data[7] == TRUE as i16 {
                Swap_CreateMonSprite();
            } else {
                Select_CreateMonSprite();
            }
            return;
        }
    }
    (*task).data[0] += 1;
}
pub(crate) unsafe extern "C" fn Task_CloseMonPic(taskId: u8) {
    let mut task: *mut Task = &raw mut gTasks[taskId];
    match (*task).data[0] {
        0 => {
            (*task).data[3] = 88;
            (*task).data[4] = 152;
            (*task).data[5] = 32;
            (*task).data[8] = 96;
            SetGpuRegBits(REG_OFFSET_DISPCNT, DISPCNT_WIN0_ON);
            SetGpuReg(
                REG_OFFSET_WIN0H,
                ((*task).data[3] as u16) << 8 | (*task).data[4] as u16,
            );
            SetGpuReg(
                REG_OFFSET_WIN0V,
                ((*task).data[5] as u16) << 8 | (*task).data[8] as u16,
            );
            SetGpuReg(REG_OFFSET_WININ, 63);
            SetGpuReg(REG_OFFSET_WINOUT, 55);
            (*task).data[0] += 1;
        }
        1 => {
            (*task).data[5] += 4;
            (*task).data[8] -= 4;
            if (*task).data[5] >= 64 || (*task).data[8] <= 65 {
                (*task).data[5] = 64;
                (*task).data[8] = 65;
            }
            SetGpuReg(
                REG_OFFSET_WIN0V,
                ((*task).data[5] as u16) << 8 | (*task).data[8] as u16,
            );
            if (*task).data[5] == 64 {
                (*task).data[0] += 1;
            }
        }
        _ => {
            HideBg(3);
            gSprites[(*task).data[6]].data[7] = (*task).data[7];
            gSprites[(*task).data[6]].set_invisible(FALSE as u16);
            gSprites[(*task).data[6]].callback = Some(SpriteCB_CloseMonPic);
            StartSpriteAffineAnim(&raw mut gSprites[(*task).data[6]], 1);
            ClearGpuRegBits(REG_OFFSET_DISPCNT, DISPCNT_WIN0_ON);
            DestroyTask(taskId);
        }
    }
}
pub(crate) unsafe extern "C" fn Swap_CreateMonSprite() {
    let mut mon: *mut Pokemon = null_mut();
    let mut species: u16 = 0;
    let mut personality: u32 = 0;
    let mut otId: u32 = 0;
    if (*sFactorySwapScreen).inEnemyScreen == 0 {
        mon = &raw mut gPlayerParty[(*sFactorySwapScreen).cursorPos];
    } else {
        mon = &raw mut gEnemyParty[(*sFactorySwapScreen).cursorPos];
    }
    species = GetMonData3(mon, MON_DATA_SPECIES, null_mut()) as u16;
    personality = GetMonData3(mon, MON_DATA_PERSONALITY, null_mut());
    otId = GetMonData3(mon, MON_DATA_OT_ID, null_mut());
    (*sFactorySwapScreen).monPic.monSpriteId =
        CreateMonPicSprite_HandleDeoxys(species, otId, personality, TRUE, 88, 32, 15, TAG_NONE)
            as u8;
    gSprites[(*sFactorySwapScreen).monPic.monSpriteId].centerToCornerVecX = 0;
    gSprites[(*sFactorySwapScreen).monPic.monSpriteId].centerToCornerVecY = 0;
    (*sFactorySwapScreen).monPicAnimating = FALSE;
}
