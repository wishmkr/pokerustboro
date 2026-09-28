//! Translated from `src/decoration.c` by tools/rustport/c2rs.py.
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
// Data tables (translate with cdata.py): DecorGfx_SMALL_DESK DecorGfx_POKEMON_DESK DecorGfx_HEAVY_DESK DecorGfx_RAGGED_DESK DecorGfx_COMFORT_DESK DecorGfx_PRETTY_DESK DecorGfx_BRICK_DESK DecorGfx_CAMP_DESK DecorGfx_HARD_DESK DecorGfx_SMALL_CHAIR DecorGfx_POKEMON_CHAIR DecorGfx_HEAVY_CHAIR DecorGfx_PRETTY_CHAIR DecorGfx_COMFORT_CHAIR DecorGfx_RAGGED_CHAIR DecorGfx_BRICK_CHAIR DecorGfx_CAMP_CHAIR DecorGfx_HARD_CHAIR DecorGfx_RED_PLANT DecorGfx_TROPICAL_PLANT DecorGfx_PRETTY_FLOWERS DecorGfx_COLORFUL_PLANT DecorGfx_BIG_PLANT DecorGfx_GORGEOUS_PLANT DecorGfx_RED_BRICK DecorGfx_YELLOW_BRICK DecorGfx_BLUE_BRICK DecorGfx_RED_BALLOON DecorGfx_BLUE_BALLOON DecorGfx_YELLOW_BALLOON DecorGfx_RED_TENT DecorGfx_BLUE_TENT DecorGfx_SOLID_BOARD DecorGfx_SLIDE DecorGfx_FENCE_LENGTH DecorGfx_FENCE_WIDTH DecorGfx_TIRE DecorGfx_STAND DecorGfx_MUD_BALL DecorGfx_BREAKABLE_DOOR DecorGfx_SAND_ORNAMENT DecorGfx_SILVER_SHIELD DecorGfx_GOLD_SHIELD DecorGfx_GLASS_ORNAMENT DecorGfx_TV DecorGfx_ROUND_TV DecorGfx_CUTE_TV DecorGfx_GLITTER_MAT DecorGfx_JUMP_MAT DecorGfx_SPIN_MAT DecorGfx_C_LOW_NOTE_MAT DecorGfx_D_NOTE_MAT DecorGfx_E_NOTE_MAT DecorGfx_F_NOTE_MAT DecorGfx_G_NOTE_MAT DecorGfx_A_NOTE_MAT DecorGfx_B_NOTE_MAT DecorGfx_C_HIGH_NOTE_MAT DecorGfx_SURF_MAT DecorGfx_THUNDER_MAT DecorGfx_FIRE_BLAST_MAT DecorGfx_POWDER_SNOW_MAT DecorGfx_ATTRACT_MAT DecorGfx_FISSURE_MAT DecorGfx_SPIKES_MAT DecorGfx_BALL_POSTER DecorGfx_GREEN_POSTER DecorGfx_RED_POSTER DecorGfx_BLUE_POSTER DecorGfx_CUTE_POSTER DecorGfx_PIKA_POSTER DecorGfx_LONG_POSTER DecorGfx_SEA_POSTER DecorGfx_SKY_POSTER DecorGfx_KISS_POSTER DecorGfx_PICHU_DOLL DecorGfx_PIKACHU_DOLL DecorGfx_MARILL_DOLL DecorGfx_TOGEPI_DOLL DecorGfx_CYNDAQUIL_DOLL DecorGfx_CHIKORITA_DOLL DecorGfx_TOTODILE_DOLL DecorGfx_JIGGLYPUFF_DOLL DecorGfx_MEOWTH_DOLL DecorGfx_CLEFAIRY_DOLL DecorGfx_DITTO_DOLL DecorGfx_SMOOCHUM_DOLL DecorGfx_TREECKO_DOLL DecorGfx_TORCHIC_DOLL DecorGfx_MUDKIP_DOLL DecorGfx_DUSKULL_DOLL DecorGfx_WYNAUT_DOLL DecorGfx_BALTOY_DOLL DecorGfx_KECLEON_DOLL DecorGfx_AZURILL_DOLL DecorGfx_SKITTY_DOLL DecorGfx_SWABLU_DOLL DecorGfx_GULPIN_DOLL DecorGfx_LOTAD_DOLL DecorGfx_SEEDOT_DOLL DecorGfx_PIKA_CUSHION DecorGfx_ROUND_CUSHION DecorGfx_KISS_CUSHION DecorGfx_ZIGZAG_CUSHION DecorGfx_SPIN_CUSHION DecorGfx_DIAMOND_CUSHION DecorGfx_BALL_CUSHION DecorGfx_GRASS_CUSHION DecorGfx_FIRE_CUSHION DecorGfx_WATER_CUSHION DecorGfx_SNORLAX_DOLL DecorGfx_RHYDON_DOLL DecorGfx_LAPRAS_DOLL DecorGfx_VENUSAUR_DOLL DecorGfx_CHARIZARD_DOLL DecorGfx_BLASTOISE_DOLL DecorGfx_WAILMER_DOLL DecorGfx_REGIROCK_DOLL DecorGfx_REGICE_DOLL DecorGfx_REGISTEEL_DOLL DecorDesc_SMALL_DESK DecorDesc_POKEMON_DESK DecorDesc_HEAVY_DESK DecorDesc_RAGGED_DESK DecorDesc_COMFORT_DESK DecorDesc_PRETTY_DESK DecorDesc_BRICK_DESK DecorDesc_CAMP_DESK DecorDesc_HARD_DESK DecorDesc_SMALL_CHAIR DecorDesc_POKEMON_CHAIR DecorDesc_HEAVY_CHAIR DecorDesc_PRETTY_CHAIR DecorDesc_COMFORT_CHAIR DecorDesc_RAGGED_CHAIR DecorDesc_BRICK_CHAIR DecorDesc_CAMP_CHAIR DecorDesc_HARD_CHAIR DecorDesc_RED_PLANT DecorDesc_TROPICAL_PLANT DecorDesc_PRETTY_FLOWERS DecorDesc_COLORFUL_PLANT DecorDesc_BIG_PLANT DecorDesc_GORGEOUS_PLANT DecorDesc_RED_BRICK DecorDesc_YELLOW_BRICK DecorDesc_BLUE_BRICK DecorDesc_RED_BALLOON DecorDesc_BLUE_BALLOON DecorDesc_YELLOW_BALLOON DecorDesc_RED_TENT DecorDesc_BLUE_TENT DecorDesc_SOLID_BOARD DecorDesc_SLIDE DecorDesc_FENCE_LENGTH DecorDesc_FENCE_WIDTH DecorDesc_TIRE DecorDesc_STAND DecorDesc_MUD_BALL DecorDesc_BREAKABLE_DOOR DecorDesc_SAND_ORNAMENT DecorDesc_SILVER_SHIELD DecorDesc_GOLD_SHIELD DecorDesc_GLASS_ORNAMENT DecorDesc_TV DecorDesc_ROUND_TV DecorDesc_CUTE_TV DecorDesc_GLITTER_MAT DecorDesc_JUMP_MAT DecorDesc_SPIN_MAT DecorDesc_C_LOW_NOTE_MAT DecorDesc_D_NOTE_MAT DecorDesc_E_NOTE_MAT DecorDesc_F_NOTE_MAT DecorDesc_G_NOTE_MAT DecorDesc_A_NOTE_MAT DecorDesc_B_NOTE_MAT DecorDesc_C_HIGH_NOTE_MAT DecorDesc_SURF_MAT DecorDesc_THUNDER_MAT DecorDesc_FIRE_BLAST_MAT DecorDesc_POWDER_SNOW_MAT DecorDesc_ATTRACT_MAT DecorDesc_FISSURE_MAT DecorDesc_SPIKES_MAT DecorDesc_BALL_POSTER DecorDesc_GREEN_POSTER DecorDesc_RED_POSTER DecorDesc_BLUE_POSTER DecorDesc_CUTE_POSTER DecorDesc_PIKA_POSTER DecorDesc_LONG_POSTER DecorDesc_SEA_POSTER DecorDesc_SKY_POSTER DecorDesc_KISS_POSTER DecorDesc_PICHU_DOLL DecorDesc_PIKACHU_DOLL DecorDesc_MARILL_DOLL DecorDesc_TOGEPI_DOLL DecorDesc_CYNDAQUIL_DOLL DecorDesc_CHIKORITA_DOLL DecorDesc_TOTODILE_DOLL DecorDesc_JIGGLYPUFF_DOLL DecorDesc_MEOWTH_DOLL DecorDesc_CLEFAIRY_DOLL DecorDesc_DITTO_DOLL DecorDesc_SMOOCHUM_DOLL DecorDesc_TREECKO_DOLL DecorDesc_TORCHIC_DOLL DecorDesc_MUDKIP_DOLL DecorDesc_DUSKULL_DOLL DecorDesc_WYNAUT_DOLL DecorDesc_BALTOY_DOLL DecorDesc_KECLEON_DOLL DecorDesc_AZURILL_DOLL DecorDesc_SKITTY_DOLL DecorDesc_SWABLU_DOLL DecorDesc_GULPIN_DOLL DecorDesc_LOTAD_DOLL DecorDesc_SEEDOT_DOLL DecorDesc_PIKA_CUSHION DecorDesc_ROUND_CUSHION DecorDesc_KISS_CUSHION DecorDesc_ZIGZAG_CUSHION DecorDesc_SPIN_CUSHION DecorDesc_DIAMOND_CUSHION DecorDesc_BALL_CUSHION DecorDesc_GRASS_CUSHION DecorDesc_FIRE_CUSHION DecorDesc_WATER_CUSHION DecorDesc_SNORLAX_DOLL DecorDesc_RHYDON_DOLL DecorDesc_LAPRAS_DOLL DecorDesc_VENUSAUR_DOLL DecorDesc_CHARIZARD_DOLL DecorDesc_BLASTOISE_DOLL DecorDesc_WAILMER_DOLL DecorDesc_REGIROCK_DOLL DecorDesc_REGICE_DOLL DecorDesc_REGISTEEL_DOLL gDecorations sDecorationCategoryNames sDecorationMainMenuActions sSecretBasePCMenuItemDescriptions sSecretBasePC_SelectedDecorationActions sDecorationWindowTemplates sDecorationMenuPalette sDecorationItemsListMenuTemplate gDecorIconTable sDecorTilemap_1x1_Tiles sDecorTilemap_3x1_Tiles sDecorTilemap_2x2_Tiles sDecorTilemap_1x3_Tiles sDecorTilemap_2x1_Tiles sDecorTilemap_4x2_Tiles sDecorTilemap_3x3_Tiles sDecorTilemap_3x2_Tiles sDecorTilemap_1x1_Y sDecorTilemap_2x1_Y sDecorTilemap_3x1_Y sDecorTilemap_4x2_Y sDecorTilemap_2x2_Y sDecorTilemap_1x2_Y sDecorTilemap_1x3_Y sDecorTilemap_2x4_Y sDecorTilemap_3x3_Y sDecorTilemap_3x2_Y sDecorTilemap_1x1_X sDecorTilemap_2x1_X sDecorTilemap_3x1_X sDecorTilemap_4x2_X sDecorTilemap_2x2_X sDecorTilemap_1x2_X sDecorTilemap_1x3_X sDecorTilemap_2x4_X sDecorTilemap_3x3_X sDecorTilemap_3x2_X sDecorTilemaps sDecorationMovementInfo sDecorSelectorAnimCmd0 sDecorSelectorAnimCmds sDecorSelectorSpriteFrameImages sDecorationSelectorSpriteTemplate sDecorWhilePlacingSpriteTemplate sSpritePal_PlaceDecoration sPlaceDecorationYesNoFunctions sCancelDecoratingYesNoFunctions sPlacePutAwayYesNoFunctions sDecorationStandElevations sDecorationSlideElevation sDecorShapeSizes sBrendanPalette sMayPalette sReturnDecorationYesNoFunctions sStopPuttingAwayDecorationsYesNoFunctions sDecorationPuttingAwayCursor sSpritePal_PuttingAwayCursorBrendan sSpritePal_PuttingAwayCursorMay sPuttingAwayCursorOamData sPuttingAwayCursorAnimCmd0 sPuttingAwayCursorAnimCmds sPuttingAwayCursorPicTable sPuttingAwayCursorSpriteTemplate sTossDecorationYesNoFunctions

/// `struct DecorationPCContext`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DecorationPCContext {
    pub items: *mut u8,
    pub pos: *mut u8,
    pub size: u8,
    pub isPlayerRoom: u8,
}

unsafe impl Sync for DecorationPCContext {}

/// `struct DecorationItemsMenu`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DecorationItemsMenu {
    pub items: CArray<ListMenuItem, 41>,
    pub names: CArray<CArray<u8, 24>, 41>,
    pub numMenuItems: u8,
    pub maxShownItems: u8,
    pub scrollIndicatorsTaskId: u8,
}

unsafe impl Sync for DecorationItemsMenu {}

/// `struct PlaceDecorationGraphicsDataBuffer`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PlaceDecorationGraphicsDataBuffer {
    pub decoration: *mut Decoration,
    pub tiles: CArray<u16, 64>,
    pub image: CArray<u8, 2048>,
    pub palette: CArray<u16, 16>,
}

unsafe impl Sync for PlaceDecorationGraphicsDataBuffer {}

/// `struct DecorRearrangementDataBuffer`
#[repr(C, align(4))]
#[derive(Clone, Copy)]
pub struct DecorRearrangementDataBuffer {
    pub idx: u8,
    pub width: u8,
    pub height: u8,
    pub flagId: u16,
}

unsafe impl Sync for DecorRearrangementDataBuffer {}

/// `__typeof__(sDecorTilemaps[0])`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct sDecorTilemaps_0_t {
    pub tiles: *mut u8,
    pub y: *mut u8,
    pub x: *mut u8,
    pub size: u8,
}

unsafe impl Sync for sDecorTilemaps_0_t {}

/// `__typeof__(sDecorationMovementInfo[0])`
#[repr(C, align(4))]
#[derive(Clone, Copy)]
pub struct sDecorationMovementInfo_0_t {
    pub shape: u8,
    pub size: u8,
    pub cameraX: u8,
    pub cameraY: u8,
}

unsafe impl Sync for sDecorationMovementInfo_0_t {}

#[cfg(target_arch = "arm")]
const _: () = {
    #[allow(unused_imports)]
    use core::mem::{offset_of, size_of};
    assert!(size_of::<DecorationPCContext>() == 12);
    assert!(offset_of!(DecorationPCContext, items) == 0);
    assert!(offset_of!(DecorationPCContext, pos) == 4);
    assert!(offset_of!(DecorationPCContext, size) == 8);
    assert!(offset_of!(DecorationPCContext, isPlayerRoom) == 9);
    assert!(size_of::<DecorationItemsMenu>() == 1316);
    assert!(offset_of!(DecorationItemsMenu, items) == 0);
    assert!(offset_of!(DecorationItemsMenu, names) == 328);
    assert!(offset_of!(DecorationItemsMenu, numMenuItems) == 1312);
    assert!(offset_of!(DecorationItemsMenu, maxShownItems) == 1313);
    assert!(offset_of!(DecorationItemsMenu, scrollIndicatorsTaskId) == 1314);
    assert!(size_of::<PlaceDecorationGraphicsDataBuffer>() == 2212);
    assert!(offset_of!(PlaceDecorationGraphicsDataBuffer, decoration) == 0);
    assert!(offset_of!(PlaceDecorationGraphicsDataBuffer, tiles) == 4);
    assert!(offset_of!(PlaceDecorationGraphicsDataBuffer, image) == 132);
    assert!(offset_of!(PlaceDecorationGraphicsDataBuffer, palette) == 2180);
    assert!(size_of::<DecorRearrangementDataBuffer>() == 8);
    assert!(offset_of!(DecorRearrangementDataBuffer, idx) == 0);
    assert!(offset_of!(DecorRearrangementDataBuffer, width) == 1);
    assert!(offset_of!(DecorRearrangementDataBuffer, height) == 2);
    assert!(offset_of!(DecorRearrangementDataBuffer, flagId) == 4);
    assert!(size_of::<sDecorTilemaps_0_t>() == 16);
    assert!(offset_of!(sDecorTilemaps_0_t, tiles) == 0);
    assert!(offset_of!(sDecorTilemaps_0_t, y) == 4);
    assert!(offset_of!(sDecorTilemaps_0_t, x) == 8);
    assert!(offset_of!(sDecorTilemaps_0_t, size) == 12);
    assert!(size_of::<sDecorationMovementInfo_0_t>() == 4);
    assert!(offset_of!(sDecorationMovementInfo_0_t, shape) == 0);
    assert!(offset_of!(sDecorationMovementInfo_0_t, size) == 1);
    assert!(offset_of!(sDecorationMovementInfo_0_t, cameraX) == 2);
    assert!(offset_of!(sDecorationMovementInfo_0_t, cameraY) == 3);
};

const DECOR_ITEMS_MENU_PLACE: i16 = 0;
const DECOR_ITEMS_MENU_PUT_AWAY: i16 = 1;
const DECOR_MENU_PLACE: i16 = 0;
const DECOR_MENU_TOSS: i16 = 1;
const DECOR_MENU_TRADE: i16 = 2;
const NUM_DECORATION_FLAGS: u8 = 14;
const PLACE_DECORATION_PLAYER_TAG: u16 = 8;
const PLACE_DECORATION_SELECTOR_TAG: u16 = 3045;
const WINDOW_DECORATION_CATEGORIES: u8 = 1;
const WINDOW_DECORATION_CATEGORY_ITEMS: u8 = 3;
const WINDOW_DECORATION_CATEGORY_SUMMARY: u8 = 2;
const WINDOW_MAIN_MENU: u8 = 0;

static gDecorIconTable: Table<CArray<CArray<*mut u32, 2>, 121>> =
    Table((&raw const crate::data::decoration::gDecorIconTable).cast());
static gDecorations: Table<CArray<Decoration, 121>> =
    Table((&raw const crate::data::decoration::gDecorations).cast());
static sCancelDecoratingYesNoFunctions: Table<YesNoFuncTable> =
    Table((&raw const crate::data::decoration::sCancelDecoratingYesNoFunctions).cast());
static sDecorShapeSizes: Table<CArray<u16, 10>> =
    Table((&raw const crate::data::decoration::sDecorShapeSizes).cast());
static sDecorTilemaps: Table<CArray<sDecorTilemaps_0_t, 10>> =
    Table((&raw const crate::data::decoration::sDecorTilemaps).cast());
static sDecorWhilePlacingSpriteTemplate: Table<SpriteTemplate> =
    Table((&raw const crate::data::decoration::sDecorWhilePlacingSpriteTemplate).cast());
static sDecorationCategoryNames: Table<CArray<*mut u8, 8>> =
    Table((&raw const crate::data::decoration::sDecorationCategoryNames).cast());
static sDecorationItemsListMenuTemplate: Table<ListMenuTemplate> =
    Table((&raw const crate::data::decoration::sDecorationItemsListMenuTemplate).cast());
static sDecorationMainMenuActions: Table<CArray<MenuAction, 4>> =
    Table((&raw const crate::data::decoration::sDecorationMainMenuActions).cast());
static sDecorationMenuPalette: Table<CArray<u16, 16>> =
    Table((&raw const crate::data::decoration::sDecorationMenuPalette).cast());
static sDecorationMovementInfo: Table<CArray<sDecorationMovementInfo_0_t, 10>> =
    Table((&raw const crate::data::decoration::sDecorationMovementInfo).cast());
static sDecorationSelectorSpriteTemplate: Table<SpriteTemplate> =
    Table((&raw const crate::data::decoration::sDecorationSelectorSpriteTemplate).cast());
static sDecorationSlideElevation: Table<CArray<u8, 8>> =
    Table((&raw const crate::data::decoration::sDecorationSlideElevation).cast());
static sDecorationStandElevations: Table<CArray<u8, 8>> =
    Table((&raw const crate::data::decoration::sDecorationStandElevations).cast());
static sDecorationWindowTemplates: Table<CArray<WindowTemplate, 4>> =
    Table((&raw const crate::data::decoration::sDecorationWindowTemplates).cast());
static sPlaceDecorationYesNoFunctions: Table<YesNoFuncTable> =
    Table((&raw const crate::data::decoration::sPlaceDecorationYesNoFunctions).cast());
static sPlacePutAwayYesNoFunctions: Table<CArray<YesNoFuncTable, 2>> =
    Table((&raw const crate::data::decoration::sPlacePutAwayYesNoFunctions).cast());
static sPuttingAwayCursorSpriteTemplate: Table<SpriteTemplate> =
    Table((&raw const crate::data::decoration::sPuttingAwayCursorSpriteTemplate).cast());
static sReturnDecorationYesNoFunctions: Table<YesNoFuncTable> =
    Table((&raw const crate::data::decoration::sReturnDecorationYesNoFunctions).cast());
static sSecretBasePCMenuItemDescriptions: Table<CArray<*mut u8, 4>> =
    Table((&raw const crate::data::decoration::sSecretBasePCMenuItemDescriptions).cast());
static sSecretBasePC_SelectedDecorationActions: Table<
    CArray<CArray<Option<unsafe extern "C" fn(u8)>, 2>, 3>,
> = Table((&raw const crate::data::decoration::sSecretBasePC_SelectedDecorationActions).cast());
static sSpritePal_PlaceDecoration: Table<SpritePalette> =
    Table((&raw const crate::data::decoration::sSpritePal_PlaceDecoration).cast());
static sSpritePal_PuttingAwayCursorBrendan: Table<SpritePalette> =
    Table((&raw const crate::data::decoration::sSpritePal_PuttingAwayCursorBrendan).cast());
static sSpritePal_PuttingAwayCursorMay: Table<SpritePalette> =
    Table((&raw const crate::data::decoration::sSpritePal_PuttingAwayCursorMay).cast());
static sStopPuttingAwayDecorationsYesNoFunctions: Table<YesNoFuncTable> =
    Table((&raw const crate::data::decoration::sStopPuttingAwayDecorationsYesNoFunctions).cast());
static sTossDecorationYesNoFunctions: Table<YesNoFuncTable> =
    Table((&raw const crate::data::decoration::sTossDecorationYesNoFunctions).cast());

#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gCurDecorationItems: *mut u8 = null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sDecorationActionsCursorPos: u8 = 0;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sNumOwnedDecorationsInCurCategory: u8 = 0;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sSecretBaseItemsIndicesBuffer: Aligned<CArray<u8, 16>> =
    Aligned(unsafe { zeroed() });
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sPlayerRoomItemsIndicesBuffer: Aligned<CArray<u8, 12>> =
    Aligned(unsafe { zeroed() });
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sDecorationsCursorPos: u16 = 0;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sDecorationsScrollOffset: u16 = 0;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gCurDecorationIndex: u8 = 0;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sCurDecorationCategory: u8 = 0;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sFiller: CArray<u32, 2> = unsafe { zeroed() };
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sDecorationContext: DecorationPCContext = unsafe { zeroed() };
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sDecorMenuWindowIds: Aligned<CArray<u8, 4>> = Aligned(unsafe { zeroed() });
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sDecorationItemsMenu: *mut DecorationItemsMenu = null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sPlaceDecorationGraphicsDataBuffer: PlaceDecorationGraphicsDataBuffer =
    unsafe { zeroed() };
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sCurDecorMapX: u16 = 0;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sCurDecorMapY: u16 = 0;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sDecor_CameraSpriteObjectIdx1: u8 = 0;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sDecor_CameraSpriteObjectIdx2: u8 = 0;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sDecorationLastDirectionMoved: u8 = 0;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sDecorSelectorOam: OamData = unsafe { zeroed() };
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sDecorRearrangementDataBuffer: CArray<DecorRearrangementDataBuffer, 16> =
    unsafe { zeroed() };
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sCurDecorSelectedInRearrangement: u8 = 0;

unsafe extern "C" {
    static SecretBase_EventScript_InitDecorations: CArray<u8, 0>;
    static SecretBase_EventScript_PCCancel: CArray<u8, 0>;
    static SecretBase_EventScript_PutAwayDecoration: CArray<u8, 0>;
    static SecretBase_EventScript_SetDecoration: CArray<u8, 0>;
    static mut gDecorationInventories: CArray<DecorationInventory, 0>;
    static mut gFieldCallback: Option<unsafe extern "C" fn()>;
    static mut gFieldCamera: CameraObject;
    static mut gItemIcon4x4Buffer: *mut u8;
    static mut gItemIconDecompressionBuffer: *mut u8;
    static gItemIconSpriteTemplate: SpriteTemplate;
    static mut gMain: Main;
    static mut gMapHeader: MapHeader;
    static mut gMultiuseListMenuTemplate: ListMenuTemplate;
    static mut gPaletteFade: PaletteFadeControl;
    static mut gSaveBlock1Ptr: *mut SaveBlock1;
    static mut gSaveBlock2Ptr: *mut SaveBlock2;
    static mut gSpecialVar_0x8004: u16;
    static mut gSpecialVar_0x8005: u16;
    static mut gSpecialVar_0x8006: u16;
    static mut gSpecialVar_0x8007: u16;
    static mut gSpecialVar_Result: u16;
    static mut gSprites: CArray<Sprite, 65>;
    static mut gStringVar1: CArray<u8, 256>;
    static mut gStringVar4: CArray<u8, 1000>;
    static mut gTasks: CArray<Task, 0>;
    static gText_Cancel: CArray<u8, 0>;
    static gText_CancelDecorating: CArray<u8, 0>;
    static gText_CantBePlacedHere: CArray<u8, 0>;
    static gText_CantPlaceInRoom: CArray<u8, 0>;
    static gText_CantThrowAwayInUse: CArray<u8, 0>;
    static gText_Color161Shadow161: CArray<u8, 0>;
    static gText_DecorationReturnedToPC: CArray<u8, 0>;
    static gText_DecorationThrownAway: CArray<u8, 0>;
    static gText_DecorationWillBeDiscarded: CArray<u8, 0>;
    static gText_Exit: CArray<u8, 0>;
    static gText_GoBackPrevMenu: CArray<u8, 0>;
    static gText_InUseAlready: CArray<u8, 0>;
    static gText_NoDecorationHere: CArray<u8, 0>;
    static gText_NoDecorations: CArray<u8, 0>;
    static gText_NoDecorationsInUse: CArray<u8, 0>;
    static gText_NoMoreDecorations: CArray<u8, 0>;
    static gText_NoMoreDecorations2: CArray<u8, 0>;
    static gText_PlaceItHere: CArray<u8, 0>;
    static gText_ReturnDecorationToPC: CArray<u8, 0>;
    static gText_StopPuttingAwayDecorations: CArray<u8, 0>;
    static gTilesetPointer_SecretBase: *mut Tileset;
    static gTilesetPointer_SecretBaseRedCave: *mut Tileset;
    fn AddScrollIndicatorArrowPairParameterized(
        a0: u32,
        a1: i32,
        a2: i32,
        a3: i32,
        a4: i32,
        a5: i32,
        a6: i32,
        a7: *mut u16,
    ) -> u8;
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
    fn AddWindow(a0: *mut WindowTemplate) -> u16;
    fn Alloc(a0: u32) -> *mut c_void;
    fn AllocItemIconTemporaryBuffers() -> u8;
    fn AllocZeroed(a0: u32) -> *mut c_void;
    fn BlitMenuInfoIcon(a0: u8, a1: u8, a2: u16, a3: u16);
    fn CB2_ReturnToField();
    fn ClearDialogWindowAndFrame(a0: u8, a1: u8);
    fn ClearStdWindowAndFrameToTransparent(a0: u8, a1: u8);
    fn ClearWindowTilemap(a0: u8);
    fn CondenseDecorationsInCategory(a0: u8);
    fn ConvertIntToDecimalStringN(a0: *mut u8, a1: i32, a2: i32, a3: u8) -> *mut u8;
    fn CopyItemIconPicTo4x4Buffer(a0: *mut c_void, a1: *mut c_void);
    fn CpuFastSet(a0: *mut c_void, a1: *mut c_void, a2: u32);
    fn CpuSet(a0: *mut c_void, a1: *mut c_void, a2: u32);
    fn CreateObjectGraphicsSprite(
        a0: u16,
        a1: Option<unsafe extern "C" fn(*mut Sprite)>,
        a2: i16,
        a3: i16,
        a4: u8,
    ) -> u8;
    fn CreateSprite(a0: *mut SpriteTemplate, a1: i16, a2: i16, a3: u8) -> u8;
    fn CreateTask(a0: Option<unsafe extern "C" fn(u8)>, a1: u8) -> u8;
    fn DestroyListMenuTask(a0: u8, a1: *mut u16, a2: *mut u16);
    fn DestroySprite(a0: *mut Sprite);
    fn DestroyTask(a0: u8);
    fn DisplayItemMessageOnField(a0: u8, a1: *mut u8, a2: Option<unsafe extern "C" fn(u8)>);
    fn DisplayYesNoMenuDefaultYes();
    fn DoYesNoFuncWithChoice(a0: u8, a1: *mut YesNoFuncTable);
    fn DrawDialogueFrame(a0: u8, a1: u8);
    fn DrawStdFrameWithCustomTileAndPalette(a0: u8, a1: u8, a2: u16, a3: u8);
    fn DrawWholeMapView();
    fn ExitTraderMenu(a0: u8);
    fn FadeInFromBlack();
    fn FadeScreen(a0: u8, a1: i8);
    fn FillWindowPixelBuffer(a0: u8, a1: u8);
    fn FlagClear(a0: u16) -> u8;
    fn FlagGet(a0: u16) -> u8;
    fn Free(a0: *mut c_void);
    fn FreeItemIconTemporaryBuffers();
    fn FreeSpritePaletteByTag(a0: u16);
    fn GetMaxWidthInMenuTable(a0: *mut MenuAction, a1: i32) -> i32;
    fn GetMetatileAttributesById(a0: u16) -> u16;
    fn GetNumOwnedDecorations() -> u8;
    fn GetNumOwnedDecorationsInCategory(a0: u8) -> u8;
    fn GetObjectEventIdByPosition(a0: u16, a1: u16, a2: u8) -> u8;
    fn GetPlayerFacingDirection() -> u8;
    fn GetStringRightAlignXOffset(a0: i32, a1: *mut u8, a2: i32) -> i32;
    fn HideSecretBaseDecorationSprites();
    fn InitMenuInUpperLeftCornerNormal(a0: u8, a1: u8, a2: u8) -> u8;
    fn IsWeatherNotFadingIn() -> u8;
    fn LZDecompressWram(a0: *mut u32, a1: *mut c_void);
    fn ListMenuGetScrollAndRow(a0: u8, a1: *mut u16, a2: *mut u16);
    fn ListMenuInit(a0: *mut ListMenuTemplate, a1: u16, a2: u16) -> u8;
    fn ListMenu_ProcessInput(a0: u8) -> i32;
    fn LoadCompressedSpritePalette(a0: *mut CompressedSpritePalette);
    fn LoadPalette(a0: *mut c_void, a1: u16, a2: u16);
    fn LoadSpritePalette(a0: *mut SpritePalette) -> u8;
    fn LoadSpriteSheet(a0: *mut SpriteSheet) -> u16;
    fn LockPlayerFieldControls();
    fn MapGridGetMetatileBehaviorAt(a0: i32, a1: i32) -> i32;
    fn MapGridGetMetatileIdAt(a0: i32, a1: i32) -> i32;
    fn MapGridSetMetatileEntryAt(a0: i32, a1: i32, a2: u16);
    fn MapGridSetMetatileIdAt(a0: i32, a1: i32, a2: u16);
    fn Menu_GetCursorPos() -> u8;
    fn Menu_ProcessInput() -> i8;
    fn MetatileBehavior_HoldsLargeDecoration(a0: u8) -> u8;
    fn MetatileBehavior_HoldsSmallDecoration(a0: u8) -> u8;
    fn MetatileBehavior_IsNormal(a0: u8) -> u8;
    fn MetatileBehavior_IsPlayerRoomPCOn(a0: u8) -> u8;
    fn MetatileBehavior_IsSecretBaseHole(a0: u8) -> u8;
    fn MetatileBehavior_IsSecretBaseImpassable(a0: u8) -> u8;
    fn MetatileBehavior_IsSecretBaseNorthWall(a0: u8) -> u8;
    fn MetatileBehavior_IsSecretBasePC(a0: u8) -> u8;
    fn MetatileBehavior_IsSecretBaseTrainerSpot(a0: u8) -> u8;
    fn PlaySE(a0: u16);
    fn PlayerGetDestCoords(a0: *mut i16, a1: *mut i16);
    fn PrintMenuTable(a0: u8, a1: u8, a2: *mut MenuAction);
    fn RemoveScrollIndicatorArrowPair(a0: u8);
    fn RemoveWindow(a0: u8);
    fn ReshowPlayerPC(a0: u8);
    fn ScheduleBgCopyTilemapToVram(a0: u8);
    fn ScriptContext_SetupScript(a0: *mut u8);
    fn SetCursorScrollWithinListBounds(a0: *mut u16, a1: *mut u16, a2: u8, a3: u8, a4: u8);
    fn SetCursorWithinListBounds(a0: *mut u16, a1: *mut u16, a2: u8, a3: u8);
    fn SetMainCallback2(a0: Option<unsafe extern "C" fn()>);
    fn SetWarpDestination(a0: i8, a1: i8, a2: i8, a3: i8, a4: i8);
    fn SpriteCallbackDummy(a0: *mut Sprite);
    fn StringAppend(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn StringCopy(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn StringExpandPlaceholders(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn StringLength(a0: *mut u8) -> u16;
    fn TryMoveObjectEventToMapCoords(a0: u8, a1: u8, a2: u8, a3: i16, a4: i16);
    fn TryOverrideObjectEventTemplateCoords(a0: u8, a1: u8, a2: u8);
    fn TryPutSecretBaseVisitOnAir();
    fn TrySpawnObjectEvent(a0: u8, a1: u8, a2: u8) -> u8;
    fn VarSet(a0: u16, a1: u16) -> u8;
    fn WarpIntoMap();
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn InitDecorationContextItems() {
    if sCurDecorationCategory < DECORCAT_COUNT {
        gCurDecorationItems = gDecorationInventories[sCurDecorationCategory].items;
    }
    if sDecorationContext.isPlayerRoom == FALSE {
        sDecorationContext.items = (*gSaveBlock1Ptr).secretBases[0].decorations.as_mut_ptr();
        sDecorationContext.pos = (*gSaveBlock1Ptr).secretBases[0]
            .decorationPositions
            .as_mut_ptr();
    }
    if sDecorationContext.isPlayerRoom == TRUE {
        sDecorationContext.items = (*gSaveBlock1Ptr).playerRoomDecorations.as_mut_ptr();
        sDecorationContext.pos = (*gSaveBlock1Ptr).playerRoomDecorationPositions.as_mut_ptr();
    }
}
pub(crate) unsafe extern "C" fn AddDecorationWindow(windowIndex: u8) -> u8 {
    let mut windowId: *mut u8 = null_mut();
    let mut template: WindowTemplate = zeroed();
    windowId = &raw mut sDecorMenuWindowIds[windowIndex];
    if windowIndex == WINDOW_MAIN_MENU {
        template = sDecorationWindowTemplates[0];
        template.width =
            GetMaxWidthInMenuTable(sDecorationMainMenuActions.as_ptr().cast_mut(), 4) as u8;
        if template.width > 18 {
            template.width = 18;
        }
        *windowId = AddWindow(&raw mut template) as u8;
    } else {
        *windowId =
            AddWindow((&raw const sDecorationWindowTemplates[windowIndex]).cast_mut()) as u8;
    }
    DrawStdFrameWithCustomTileAndPalette(*windowId, FALSE, 0x214, 14);
    ScheduleBgCopyTilemapToVram(0);
    return *windowId;
}
pub(crate) unsafe extern "C" fn RemoveDecorationWindow(windowIndex: u8) {
    ClearStdWindowAndFrameToTransparent(sDecorMenuWindowIds[windowIndex], FALSE);
    ClearWindowTilemap(sDecorMenuWindowIds[windowIndex]);
    RemoveWindow(sDecorMenuWindowIds[windowIndex]);
    ScheduleBgCopyTilemapToVram(0);
}
pub(crate) unsafe extern "C" fn AddDecorationActionsWindow() {
    let mut windowId: u8 = AddDecorationWindow(WINDOW_MAIN_MENU);
    PrintMenuTable(windowId, 4, sDecorationMainMenuActions.as_ptr().cast_mut());
    InitMenuInUpperLeftCornerNormal(windowId, 4, sDecorationActionsCursorPos);
}
pub(crate) unsafe extern "C" fn InitDecorationActionsWindow() {
    sDecorationActionsCursorPos = 0;
    LockPlayerFieldControls();
    AddDecorationActionsWindow();
    PrintCurMainMenuDescription();
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DoSecretBaseDecorationMenu(taskId: u8) {
    InitDecorationActionsWindow();
    sDecorationContext.items = (*gSaveBlock1Ptr).secretBases[0].decorations.as_mut_ptr();
    sDecorationContext.pos = (*gSaveBlock1Ptr).secretBases[0]
        .decorationPositions
        .as_mut_ptr();
    sDecorationContext.size = DECOR_MAX_SECRET_BASE;
    sDecorationContext.isPlayerRoom = FALSE;
    gTasks[taskId].func = Some(HandleDecorationActionsMenuInput);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DoPlayerRoomDecorationMenu(taskId: u8) {
    InitDecorationActionsWindow();
    sDecorationContext.items = (*gSaveBlock1Ptr).playerRoomDecorations.as_mut_ptr();
    sDecorationContext.pos = (*gSaveBlock1Ptr).playerRoomDecorationPositions.as_mut_ptr();
    sDecorationContext.size = DECOR_MAX_PLAYERS_HOUSE;
    sDecorationContext.isPlayerRoom = TRUE;
    gTasks[taskId].func = Some(HandleDecorationActionsMenuInput);
}
pub(crate) unsafe extern "C" fn HandleDecorationActionsMenuInput(taskId: u8) {
    if gPaletteFade.active() == 0 {
        let mut menuPos: i8 = Menu_GetCursorPos() as i8;
        match Menu_ProcessInput() {
            MENU_NOTHING_CHOSEN => {
                sDecorationActionsCursorPos = Menu_GetCursorPos();
                if menuPos as i32 != sDecorationActionsCursorPos as i32 {
                    PrintCurMainMenuDescription();
                }
            }
            MENU_B_PRESSED => {
                PlaySE(SE_SELECT);
                DecorationMenuAction_Cancel(taskId);
            }
            _ => {
                PlaySE(SE_SELECT);
                sDecorationMainMenuActions[sDecorationActionsCursorPos]
                    .func
                    .void_u8
                    .unwrap_unchecked()(taskId);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn PrintCurMainMenuDescription() {
    FillWindowPixelBuffer(0, 17);
    AddTextPrinterParameterized2(
        0,
        FONT_NORMAL,
        sSecretBasePCMenuItemDescriptions[sDecorationActionsCursorPos],
        0,
        None,
        TEXT_COLOR_DARK_GRAY,
        TEXT_COLOR_WHITE,
        TEXT_COLOR_LIGHT_GRAY,
    );
}
pub(crate) unsafe extern "C" fn DecorationMenuAction_Decorate(taskId: u8) {
    if GetNumOwnedDecorations() == 0 {
        StringExpandPlaceholders(
            gStringVar4.as_mut_ptr(),
            gText_NoDecorations.as_ptr().cast_mut(),
        );
        DisplayItemMessageOnField(
            taskId,
            gStringVar4.as_mut_ptr(),
            Some(ReturnToDecorationActionsAfterInvalidSelection),
        );
    } else {
        gTasks[taskId].data[11] = DECOR_MENU_PLACE;
        sCurDecorationCategory = DECORCAT_DESK;
        SecretBasePC_PrepMenuForSelectingStoredDecors(taskId);
    }
}
pub(crate) unsafe extern "C" fn DecorationMenuAction_PutAway(taskId: u8) {
    if HasDecorationsInUse(taskId) == 0 {
        StringExpandPlaceholders(
            gStringVar4.as_mut_ptr(),
            gText_NoDecorationsInUse.as_ptr().cast_mut(),
        );
        DisplayItemMessageOnField(
            taskId,
            gStringVar4.as_mut_ptr(),
            Some(ReturnToDecorationActionsAfterInvalidSelection),
        );
    } else {
        RemoveDecorationWindow(WINDOW_MAIN_MENU);
        ClearDialogWindowAndFrame(0, 0);
        FadeScreen(FADE_TO_BLACK, 0);
        gTasks[taskId].data[2] = 0;
        gTasks[taskId].func = Some(Task_ContinuePuttingAwayDecorations);
    }
}
pub(crate) unsafe extern "C" fn DecorationMenuAction_Toss(taskId: u8) {
    if GetNumOwnedDecorations() == 0 {
        StringExpandPlaceholders(
            gStringVar4.as_mut_ptr(),
            gText_NoDecorations.as_ptr().cast_mut(),
        );
        DisplayItemMessageOnField(
            taskId,
            gStringVar4.as_mut_ptr(),
            Some(ReturnToDecorationActionsAfterInvalidSelection),
        );
    } else {
        gTasks[taskId].data[11] = DECOR_MENU_TOSS;
        sCurDecorationCategory = DECORCAT_DESK;
        SecretBasePC_PrepMenuForSelectingStoredDecors(taskId);
    }
}
pub(crate) unsafe extern "C" fn DecorationMenuAction_Cancel(taskId: u8) {
    RemoveDecorationWindow(WINDOW_MAIN_MENU);
    if sDecorationContext.isPlayerRoom == 0 {
        ScriptContext_SetupScript(SecretBase_EventScript_PCCancel.as_ptr().cast_mut());
        DestroyTask(taskId);
    } else {
        ReshowPlayerPC(taskId);
    }
}
pub(crate) unsafe extern "C" fn ReturnToDecorationActionsAfterInvalidSelection(taskId: u8) {
    PrintCurMainMenuDescription();
    gTasks[taskId].func = Some(HandleDecorationActionsMenuInput);
}
pub(crate) unsafe extern "C" fn SecretBasePC_PrepMenuForSelectingStoredDecors(taskId: u8) {
    LoadPalette(
        sDecorationMenuPalette.as_ptr().cast_mut() as *mut c_void,
        208,
        32,
    );
    ClearDialogWindowAndFrame(0, 0);
    RemoveDecorationWindow(WINDOW_MAIN_MENU);
    InitDecorationCategoriesWindow(taskId);
}
pub(crate) unsafe extern "C" fn InitDecorationCategoriesWindow(taskId: u8) {
    let mut windowId: u8 = AddDecorationWindow(WINDOW_DECORATION_CATEGORIES);
    PrintDecorationCategoryMenuItems(taskId);
    InitMenuInUpperLeftCornerNormal(windowId, 9, sCurDecorationCategory);
    gTasks[taskId].func = Some(HandleDecorationCategoriesMenuInput);
}
pub(crate) unsafe extern "C" fn ReinitDecorationCategoriesWindow(taskId: u8) {
    FillWindowPixelBuffer(sDecorMenuWindowIds[1], 17);
    PrintDecorationCategoryMenuItems(taskId);
    InitMenuInUpperLeftCornerNormal(sDecorMenuWindowIds[1], 9, sCurDecorationCategory);
    gTasks[taskId].func = Some(HandleDecorationCategoriesMenuInput);
}
pub(crate) unsafe extern "C" fn PrintDecorationCategoryMenuItems(taskId: u8) {
    let mut i: u8 = 0;
    let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
    let mut windowId: u8 = sDecorMenuWindowIds[1];
    let mut isPlayerRoom: u8 = sDecorationContext.isPlayerRoom;
    let mut shouldDisable: u8 = FALSE;
    if isPlayerRoom == TRUE && *data.at(11) == DECOR_MENU_PLACE {
        shouldDisable = TRUE;
    }
    i = 0;
    while i < DECORCAT_COUNT {
        if shouldDisable == TRUE && i != DECORCAT_DOLL && i != DECORCAT_CUSHION {
            PrintDecorationCategoryMenuItem(windowId, i, 8, i * 16, TRUE, TEXT_SKIP_DRAW);
        } else {
            PrintDecorationCategoryMenuItem(windowId, i, 8, i * 16, FALSE, TEXT_SKIP_DRAW);
        }
        i += 1;
    }
    AddTextPrinterParameterized(
        windowId,
        FONT_NORMAL,
        if gTasks[taskId].data[11] == DECOR_MENU_TRADE {
            gText_Exit.as_ptr().cast_mut()
        } else {
            gText_Cancel.as_ptr().cast_mut()
        },
        8,
        i * 16 + 1,
        0,
        None,
    );
    ScheduleBgCopyTilemapToVram(0);
}
pub(crate) unsafe extern "C" fn PrintDecorationCategoryMenuItem(
    winid: u8,
    category: u8,
    mut x: u8,
    mut y: u8,
    disabled: u8,
    speed: u8,
) {
    let mut width: u8 = 0;
    let mut str: *mut u8 = null_mut();
    width = (if x == 8 { 104 } else { 96 }) as u8;
    y += 1;
    ColorMenuItemString(gStringVar4.as_mut_ptr(), disabled);
    str = gStringVar4
        .as_mut_ptr()
        .at(StringLength(gStringVar4.as_mut_ptr()));
    StringCopy(str, sDecorationCategoryNames[category]);
    AddTextPrinterParameterized(
        winid,
        FONT_NORMAL,
        gStringVar4.as_mut_ptr(),
        x,
        y,
        speed,
        None,
    );
    str = ConvertIntToDecimalStringN(
        str,
        GetNumOwnedDecorationsInCategory(category) as i32,
        STR_CONV_MODE_RIGHT_ALIGN,
        2,
    );
    *({
        let t1 = str;
        str = str.at(1);
        t1
    }) = CHAR_SLASH;
    ConvertIntToDecimalStringN(
        str,
        gDecorationInventories[category].size as i32,
        STR_CONV_MODE_RIGHT_ALIGN,
        2,
    );
    x = GetStringRightAlignXOffset(FONT_NORMAL as i32, gStringVar4.as_mut_ptr(), width as i32)
        as u8;
    AddTextPrinterParameterized(
        winid,
        FONT_NORMAL,
        gStringVar4.as_mut_ptr(),
        x,
        y,
        speed,
        None,
    );
}
pub(crate) unsafe extern "C" fn ColorMenuItemString(mut str: *mut u8, disabled: u8) {
    StringCopy(str, gText_Color161Shadow161.as_ptr().cast_mut());
    if disabled == TRUE {
        *str.at(2) = 4;
        *str.at(5) = 5;
    } else {
        *str.at(2) = 2;
        *str.at(5) = 3;
    }
}
pub(crate) unsafe extern "C" fn HandleDecorationCategoriesMenuInput(taskId: u8) {
    if gPaletteFade.active() == 0 {
        let mut input: i8 = Menu_ProcessInput();
        match input {
            MENU_B_PRESSED | 8 => {
                PlaySE(SE_SELECT);
                ExitDecorationCategoriesMenu(taskId);
            }
            MENU_NOTHING_CHOSEN => {}
            _ => {
                PlaySE(SE_SELECT);
                sCurDecorationCategory = input as u8;
                SelectDecorationCategory(taskId);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SelectDecorationCategory(taskId: u8) {
    sNumOwnedDecorationsInCurCategory = GetNumOwnedDecorationsInCategory(sCurDecorationCategory);
    if sNumOwnedDecorationsInCurCategory != 0 {
        CondenseDecorationsInCategory(sCurDecorationCategory);
        gCurDecorationItems = gDecorationInventories[sCurDecorationCategory].items;
        IdentifyOwnedDecorationsCurrentlyInUse(taskId);
        sDecorationsScrollOffset = 0;
        sDecorationsCursorPos = 0;
        gTasks[taskId].func = Some(ShowDecorationItemsWindow);
    } else {
        RemoveDecorationWindow(WINDOW_DECORATION_CATEGORIES);
        StringExpandPlaceholders(
            gStringVar4.as_mut_ptr(),
            gText_NoDecorations.as_ptr().cast_mut(),
        );
        DisplayItemMessageOnField(
            taskId,
            gStringVar4.as_mut_ptr(),
            Some(ReturnToDecorationCategoriesAfterInvalidSelection),
        );
    }
}
pub(crate) unsafe extern "C" fn ReturnToDecorationCategoriesAfterInvalidSelection(taskId: u8) {
    ClearDialogWindowAndFrame(0, 0);
    InitDecorationCategoriesWindow(taskId);
}
pub(crate) unsafe extern "C" fn ExitDecorationCategoriesMenu(taskId: u8) {
    if gTasks[taskId].data[11] != DECOR_MENU_TRADE {
        ReturnToActionsMenuFromCategories(taskId);
    } else {
        ExitTraderDecorationMenu(taskId);
    }
}
pub(crate) unsafe extern "C" fn ReturnToActionsMenuFromCategories(taskId: u8) {
    RemoveDecorationWindow(WINDOW_DECORATION_CATEGORIES);
    AddDecorationActionsWindow();
    DrawDialogueFrame(0, 0);
    PrintCurMainMenuDescription();
    gTasks[taskId].func = Some(HandleDecorationActionsMenuInput);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ShowDecorationCategoriesWindow(taskId: u8) {
    LoadPalette(
        sDecorationMenuPalette.as_ptr().cast_mut() as *mut c_void,
        208,
        32,
    );
    ClearDialogWindowAndFrame(0, 0);
    gTasks[taskId].data[11] = DECOR_MENU_TRADE;
    sCurDecorationCategory = DECORCAT_DESK;
    InitDecorationCategoriesWindow(taskId);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CopyDecorationCategoryName(dest: *mut u8, category: u8) {
    StringCopy(dest, sDecorationCategoryNames[category]);
}
pub(crate) unsafe extern "C" fn ExitTraderDecorationMenu(taskId: u8) {
    RemoveDecorationWindow(WINDOW_DECORATION_CATEGORIES);
    ExitTraderMenu(taskId);
}
pub(crate) unsafe extern "C" fn InitDecorationItemsMenuLimits() {
    (*sDecorationItemsMenu).numMenuItems = sNumOwnedDecorationsInCurCategory + 1;
    if (*sDecorationItemsMenu).numMenuItems > 8 {
        (*sDecorationItemsMenu).maxShownItems = 8;
    } else {
        (*sDecorationItemsMenu).maxShownItems = (*sDecorationItemsMenu).numMenuItems;
    }
}
pub(crate) unsafe extern "C" fn InitDecorationItemsMenuScrollAndCursor() {
    SetCursorWithinListBounds(
        &raw mut sDecorationsScrollOffset,
        &raw mut sDecorationsCursorPos,
        (*sDecorationItemsMenu).maxShownItems,
        (*sDecorationItemsMenu).numMenuItems,
    );
}
pub(crate) unsafe extern "C" fn InitDecorationItemsMenuScrollAndCursor2() {
    SetCursorScrollWithinListBounds(
        &raw mut sDecorationsScrollOffset,
        &raw mut sDecorationsCursorPos,
        (*sDecorationItemsMenu).maxShownItems,
        (*sDecorationItemsMenu).numMenuItems,
        8,
    );
}
pub(crate) unsafe extern "C" fn PrintDecorationItemMenuItems(taskId: u8) {
    let mut data: *mut i16 = null_mut();
    let mut i: u16 = 0;
    data = gTasks[taskId].data.as_mut_ptr();
    if (sCurDecorationCategory < DECORCAT_DOLL || sCurDecorationCategory > DECORCAT_CUSHION)
        && sDecorationContext.isPlayerRoom == TRUE
        && *data.at(11) == DECOR_MENU_PLACE
    {
        ColorMenuItemString(gStringVar1.as_mut_ptr(), TRUE);
    } else {
        ColorMenuItemString(gStringVar1.as_mut_ptr(), FALSE);
    }
    i = 0;
    while (i as i32) < (*sDecorationItemsMenu).numMenuItems as i32 - 1 {
        CopyDecorationMenuItemName(
            (*sDecorationItemsMenu).names[i].as_mut_ptr(),
            *gCurDecorationItems.at(i) as u16,
        );
        (*sDecorationItemsMenu).items[i].name = (*sDecorationItemsMenu).names[i].as_mut_ptr();
        (*sDecorationItemsMenu).items[i].id = i as i32;
        i += 1;
    }
    StringCopy(
        (*sDecorationItemsMenu).names[i].as_mut_ptr(),
        gText_Cancel.as_ptr().cast_mut(),
    );
    (*sDecorationItemsMenu).items[i].name = (*sDecorationItemsMenu).names[i].as_mut_ptr();
    (*sDecorationItemsMenu).items[i].id = LIST_CANCEL;
    gMultiuseListMenuTemplate = *sDecorationItemsListMenuTemplate;
    gMultiuseListMenuTemplate.windowId = sDecorMenuWindowIds[1];
    gMultiuseListMenuTemplate.totalItems = (*sDecorationItemsMenu).numMenuItems as u16;
    gMultiuseListMenuTemplate.items = (*sDecorationItemsMenu).items.as_mut_ptr();
    gMultiuseListMenuTemplate.maxShowed = (*sDecorationItemsMenu).maxShownItems as u16;
}
pub(crate) unsafe extern "C" fn CopyDecorationMenuItemName(dest: *mut u8, decoration: u16) {
    StringCopy(dest, gStringVar1.as_mut_ptr());
    StringAppend(dest, gDecorations[decoration].name.as_ptr().cast_mut());
}
pub(crate) unsafe extern "C" fn DecorationItemsMenu_OnCursorMove(
    itemIndex: i32,
    flag: u8,
    menu: *mut ListMenu,
) {
    if flag != TRUE {
        PlaySE(SE_SELECT);
    }
    PrintDecorationItemDescription(itemIndex);
}
pub(crate) unsafe extern "C" fn DecorationItemsMenu_PrintDecorationInUse(
    windowId: u8,
    itemIndex: u32,
    y: u8,
) {
    if itemIndex != LIST_CANCEL as u32 {
        if IsDecorationIndexInSecretBase(itemIndex as u8 + 1) == 1 {
            BlitMenuInfoIcon(windowId, MENU_INFO_ICON_BALL_RED, 92, y as u16 + 2);
        } else if IsDecorationIndexInPlayersRoom(itemIndex as u8 + 1) == 1 {
            BlitMenuInfoIcon(windowId, MENU_INFO_ICON_BALL_BLUE, 92, y as u16 + 2);
        }
    }
}
pub(crate) unsafe extern "C" fn AddDecorationItemsScrollIndicators() {
    if (*sDecorationItemsMenu).scrollIndicatorsTaskId == TASK_NONE {
        (*sDecorationItemsMenu).scrollIndicatorsTaskId = AddScrollIndicatorArrowPairParameterized(
            SCROLL_ARROW_UP,
            0x3c,
            0x0c,
            0x94,
            (*sDecorationItemsMenu).numMenuItems as i32
                - (*sDecorationItemsMenu).maxShownItems as i32,
            0x6e,
            0x6e,
            &raw mut sDecorationsScrollOffset,
        );
    }
}
pub(crate) unsafe extern "C" fn RemoveDecorationItemsScrollIndicators() {
    if (*sDecorationItemsMenu).scrollIndicatorsTaskId != TASK_NONE {
        RemoveScrollIndicatorArrowPair((*sDecorationItemsMenu).scrollIndicatorsTaskId);
        (*sDecorationItemsMenu).scrollIndicatorsTaskId = TASK_NONE;
    }
}
pub(crate) unsafe extern "C" fn AddDecorationItemsWindow(taskId: u8) {
    AddDecorationWindow(WINDOW_DECORATION_CATEGORIES);
    InitDecorationItemsWindow(taskId);
}
pub(crate) unsafe extern "C" fn InitDecorationItemsWindow(taskId: u8) {
    let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
    AddDecorationWindow(WINDOW_DECORATION_CATEGORY_ITEMS);
    ShowDecorationCategorySummaryWindow(sCurDecorationCategory);
    sDecorationItemsMenu = AllocZeroed(1316) as *mut DecorationItemsMenu;
    (*sDecorationItemsMenu).scrollIndicatorsTaskId = TASK_NONE;
    InitDecorationItemsMenuLimits();
    InitDecorationItemsMenuScrollAndCursor();
    InitDecorationItemsMenuScrollAndCursor2();
    PrintDecorationItemMenuItems(taskId);
    *data.at(13) = ListMenuInit(
        &raw mut gMultiuseListMenuTemplate,
        sDecorationsScrollOffset,
        sDecorationsCursorPos,
    ) as i16;
    AddDecorationItemsScrollIndicators();
}
pub(crate) unsafe extern "C" fn ShowDecorationItemsWindow(taskId: u8) {
    InitDecorationItemsWindow(taskId);
    gTasks[taskId].func = Some(HandleDecorationItemsMenuInput);
}
pub(crate) unsafe extern "C" fn HandleDecorationItemsMenuInput(taskId: u8) {
    let mut data: *mut i16 = null_mut();
    let mut input: i32 = 0;
    data = gTasks[taskId].data.as_mut_ptr();
    if gPaletteFade.active() == 0 {
        input = ListMenu_ProcessInput(*data.at(13) as u8);
        ListMenuGetScrollAndRow(
            *data.at(13) as u8,
            &raw mut sDecorationsScrollOffset,
            &raw mut sDecorationsCursorPos,
        );
        match input {
            LIST_NOTHING_CHOSEN => {}
            LIST_CANCEL => {
                PlaySE(SE_SELECT);
                sSecretBasePC_SelectedDecorationActions[*data.at(11)][1].unwrap_unchecked()(taskId);
            }
            _ => {
                PlaySE(SE_SELECT);
                gCurDecorationIndex = input as u8;
                RemoveDecorationItemsScrollIndicators();
                DestroyListMenuTask(
                    *data.at(13) as u8,
                    &raw mut sDecorationsScrollOffset,
                    &raw mut sDecorationsCursorPos,
                );
                RemoveDecorationWindow(WINDOW_DECORATION_CATEGORIES);
                RemoveDecorationItemsOtherWindows();
                Free(sDecorationItemsMenu as *mut c_void);
                sSecretBasePC_SelectedDecorationActions[*data.at(11)][0].unwrap_unchecked()(taskId);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn ShowDecorationCategorySummaryWindow(category: u8) {
    PrintDecorationCategoryMenuItem(
        AddDecorationWindow(WINDOW_DECORATION_CATEGORY_SUMMARY),
        category,
        0,
        0,
        0,
        0,
    );
}
pub(crate) unsafe extern "C" fn PrintDecorationItemDescription(itemIndex: i32) {
    let mut windowId: u8 = 0;
    let mut str: *mut u8 = null_mut();
    windowId = sDecorMenuWindowIds[3];
    FillWindowPixelBuffer(windowId, 17);
    if itemIndex as u32 >= sNumOwnedDecorationsInCurCategory as u32 {
        str = gText_GoBackPrevMenu.as_ptr().cast_mut();
    } else {
        str = gDecorations[*gCurDecorationItems.at(itemIndex)].description;
    }
    AddTextPrinterParameterized(windowId, FONT_NORMAL, str, 0, 1, 0, None);
}
pub(crate) unsafe extern "C" fn RemoveDecorationItemsOtherWindows() {
    RemoveDecorationWindow(WINDOW_DECORATION_CATEGORY_ITEMS);
    RemoveDecorationWindow(WINDOW_DECORATION_CATEGORY_SUMMARY);
}
pub(crate) unsafe extern "C" fn IsDecorationIndexInSecretBase(idx: u8) -> u8 {
    let mut i: u8 = 0;
    i = 0;
    while i < 16 {
        if sSecretBaseItemsIndicesBuffer[i] == idx {
            return TRUE;
        }
        i += 1;
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn IsDecorationIndexInPlayersRoom(idx: u8) -> u8 {
    let mut i: u8 = 0;
    i = 0;
    while i < 12 {
        if sPlayerRoomItemsIndicesBuffer[i] == idx {
            return TRUE;
        }
        i += 1;
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn IdentifyOwnedDecorationsCurrentlyInUseInternal(taskId: u8) {
    let mut i: u16 = 0;
    let mut j: u16 = 0;
    let mut k: u16 = 0;
    let mut count: u16 = 0;
    count = 0;
    memset(sSecretBaseItemsIndicesBuffer.as_mut_ptr(), 0, 16);
    memset(sPlayerRoomItemsIndicesBuffer.as_mut_ptr(), 0, 12);
    i = 0;
    while i < 16 {
        if (*gSaveBlock1Ptr).secretBases[0].decorations[i] != 0 {
            j = 0;
            while j < gDecorationInventories[sCurDecorationCategory].size as u16 {
                if *gCurDecorationItems.at(j) == (*gSaveBlock1Ptr).secretBases[0].decorations[i] {
                    k = 0;
                    while k < count && sSecretBaseItemsIndicesBuffer[k] as i32 != j as i32 + 1 {
                        k += 1;
                    }
                    if k == count {
                        sSecretBaseItemsIndicesBuffer[count] = j as u8 + 1;
                        count += 1;
                        break;
                    }
                }
                j += 1;
            }
        }
        i += 1;
    }
    count = 0;
    i = 0;
    while i < 12 {
        if (*gSaveBlock1Ptr).playerRoomDecorations[i] != DECOR_NONE {
            j = 0;
            while j < gDecorationInventories[sCurDecorationCategory].size as u16 {
                if *gCurDecorationItems.at(j) == (*gSaveBlock1Ptr).playerRoomDecorations[i]
                    && IsDecorationIndexInSecretBase(j as u8 + 1) != 1
                {
                    k = 0;
                    while k < count && sPlayerRoomItemsIndicesBuffer[k] as i32 != j as i32 + 1 {
                        k += 1;
                    }
                    if k == count {
                        sPlayerRoomItemsIndicesBuffer[count] = j as u8 + 1;
                        count += 1;
                        break;
                    }
                }
                j += 1;
            }
        }
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn IdentifyOwnedDecorationsCurrentlyInUse(taskId: u8) {
    IdentifyOwnedDecorationsCurrentlyInUseInternal(taskId);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsSelectedDecorInThePC() -> u8 {
    let mut i: u16 = 0;
    i = 0;
    while i < 16 {
        if sSecretBaseItemsIndicesBuffer[i] as i32
            == sDecorationsScrollOffset as i32 + sDecorationsCursorPos as i32 + 1
        {
            return FALSE;
        }
        if i < 12
            && sPlayerRoomItemsIndicesBuffer[i] as i32
                == sDecorationsScrollOffset as i32 + sDecorationsCursorPos as i32 + 1
        {
            return FALSE;
        }
        i += 1;
    }
    return TRUE;
}
pub(crate) unsafe extern "C" fn Task_ShowDecorationItemsWindow(taskId: u8) {
    AddDecorationWindow(WINDOW_DECORATION_CATEGORIES);
    ShowDecorationItemsWindow(taskId);
}
pub(crate) unsafe extern "C" fn DontTossDecoration(taskId: u8) {
    ClearDialogWindowAndFrame(0, 0);
    gTasks[taskId].func = Some(Task_ShowDecorationItemsWindow);
}
pub(crate) unsafe extern "C" fn ReturnToDecorationItemsAfterInvalidSelection(taskId: u8) {
    if gMain.newKeys as i32 & 3 != 0 {
        ClearDialogWindowAndFrame(0, 0);
        AddDecorationWindow(WINDOW_DECORATION_CATEGORIES);
        ShowDecorationItemsWindow(taskId);
    }
}
pub(crate) unsafe extern "C" fn DecorationItemsMenuAction_Cancel(taskId: u8) {
    let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
    RemoveDecorationItemsScrollIndicators();
    RemoveDecorationItemsOtherWindows();
    DestroyListMenuTask(*data.at(13) as u8, null_mut(), null_mut());
    Free(sDecorationItemsMenu as *mut c_void);
    ReinitDecorationCategoriesWindow(taskId);
}
pub(crate) unsafe extern "C" fn SetInitialPositions(taskId: u8) {
    gTasks[taskId].data[3] = (*gSaveBlock1Ptr).pos.x;
    gTasks[taskId].data[4] = (*gSaveBlock1Ptr).pos.y;
    PlayerGetDestCoords(
        &raw mut gTasks[taskId].data[0],
        &raw mut gTasks[taskId].data[1],
    );
}
pub(crate) unsafe extern "C" fn WarpToInitialPosition(taskId: u8) {
    DrawWholeMapView();
    SetWarpDestination(
        (*gSaveBlock1Ptr).location.mapGroup,
        (*gSaveBlock1Ptr).location.mapNum,
        WARP_ID_NONE,
        gTasks[taskId].data[3] as i8,
        gTasks[taskId].data[4] as i8,
    );
    WarpIntoMap();
}
pub(crate) unsafe extern "C" fn GetDecorationElevation(decoration: u8, tileIndex: u8) -> u16 {
    let mut elevation: u16 = ELEVATION_INVALID;
    match decoration {
        DECOR_STAND => {
            elevation = (sDecorationStandElevations[tileIndex] as u16) << 12;
            return elevation;
        }
        DECOR_SLIDE => {
            elevation = (sDecorationSlideElevation[tileIndex] as u16) << 12;
            return elevation;
        }
        _ => {
            return elevation;
        }
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn ShowDecorationOnMap_(
    mapX: u16,
    mapY: u16,
    decWidth: u8,
    decHeight: u8,
    decoration: u16,
) {
    let mut i: u16 = 0;
    let mut j: u16 = 0;
    let mut x: i16 = 0;
    let mut y: i16 = 0;
    let mut attributes: u16 = 0;
    let mut impassableFlag: u16 = 0;
    let mut overlapsWall: u16 = 0;
    let mut elevation: u16 = 0;
    j = 0;
    while j < decHeight as u16 {
        y = mapY as i16 - decHeight as i16 + 1 + j as i16;
        i = 0;
        while i < decWidth as u16 {
            x = mapX as i16 + i as i16;
            attributes = GetMetatileAttributesById(
                NUM_TILES_IN_PRIMARY
                    + *gDecorations[decoration]
                        .tiles
                        .at(j as i32 * decWidth as i32 + i as i32),
            );
            if MetatileBehavior_IsSecretBaseImpassable(((attributes as i32 & 0x00FF) >> 0) as u8)
                == TRUE
                || gDecorations[decoration].permission != DECORPERM_PASS_FLOOR
                    && attributes >> 12 != METATILE_LAYER_TYPE_NORMAL
            {
                impassableFlag = MAPGRID_IMPASSABLE;
            } else {
                impassableFlag = 0;
            }
            if gDecorations[decoration].permission != DECORPERM_NA_WALL
                && MetatileBehavior_IsSecretBaseNorthWall(MapGridGetMetatileBehaviorAt(
                    x as i32, y as i32,
                ) as u8)
                    == TRUE
            {
                overlapsWall = 1;
            } else {
                overlapsWall = 0;
            }
            elevation =
                GetDecorationElevation(gDecorations[decoration].id, j as u8 * decWidth + i as u8);
            if elevation != ELEVATION_INVALID {
                MapGridSetMetatileEntryAt(
                    x as i32,
                    y as i32,
                    *gDecorations[decoration]
                        .tiles
                        .at(j as i32 * decWidth as i32 + i as i32)
                        + (NUM_TILES_IN_PRIMARY | overlapsWall)
                        | impassableFlag
                        | elevation,
                );
            } else {
                MapGridSetMetatileIdAt(
                    x as i32,
                    y as i32,
                    *gDecorations[decoration]
                        .tiles
                        .at(j as i32 * decWidth as i32 + i as i32)
                        + (NUM_TILES_IN_PRIMARY | overlapsWall)
                        | impassableFlag,
                );
            }
            i += 1;
        }
        j += 1;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ShowDecorationOnMap(mapX: u16, mapY: u16, decoration: u16) {
    match gDecorations[decoration].shape {
        DECORSHAPE_1x1 => {
            ShowDecorationOnMap_(mapX, mapY, 1, 1, decoration);
        }
        DECORSHAPE_2x1 => {
            ShowDecorationOnMap_(mapX, mapY, 2, 1, decoration);
        }
        DECORSHAPE_3x1 => {
            ShowDecorationOnMap_(mapX, mapY, 3, 1, decoration);
        }
        DECORSHAPE_4x2 => {
            ShowDecorationOnMap_(mapX, mapY, 4, 2, decoration);
        }
        DECORSHAPE_2x2 => {
            ShowDecorationOnMap_(mapX, mapY, 2, 2, decoration);
        }
        DECORSHAPE_1x2 => {
            ShowDecorationOnMap_(mapX, mapY, 1, 2, decoration);
        }
        DECORSHAPE_1x3 => {
            ShowDecorationOnMap_(mapX, mapY, 1, 3, decoration);
        }
        DECORSHAPE_2x4 => {
            ShowDecorationOnMap_(mapX, mapY, 2, 4, decoration);
        }
        DECORSHAPE_3x3 => {
            ShowDecorationOnMap_(mapX, mapY, 3, 3, decoration);
        }
        DECORSHAPE_3x2 => {
            ShowDecorationOnMap_(mapX, mapY, 3, 2, decoration);
        }
        _ => {}
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetDecoration() {
    let mut i: u8 = 0;
    let mut j: u8 = 0;
    i = 0;
    while i < NUM_DECORATION_FLAGS {
        if FlagGet(FLAG_DECORATION_1 + i as u16) == TRUE {
            FlagClear(FLAG_DECORATION_1 + i as u16);
            j = 0;
            while j < (*gMapHeader.events).objectEventCount {
                if (*(*gMapHeader.events).objectEvents.at(j)).flagId as i32
                    == FLAG_DECORATION_1 as i32 + i as i32
                {
                    break;
                }
                j += 1;
            }
            VarSet(
                VAR_OBJ_GFX_ID_0
                    + ((*(*gMapHeader.events).objectEvents.at(j)).graphicsId as u16
                        - OBJ_EVENT_GFX_VAR_0),
                *(*sPlaceDecorationGraphicsDataBuffer.decoration).tiles,
            );
            gSpecialVar_0x8005 = (*(*gMapHeader.events).objectEvents.at(j)).localId as u16;
            gSpecialVar_0x8006 = sCurDecorMapX;
            gSpecialVar_0x8007 = sCurDecorMapY;
            TrySpawnObjectEvent(
                gSpecialVar_0x8005 as u8,
                (*gSaveBlock1Ptr).location.mapNum as u8,
                (*gSaveBlock1Ptr).location.mapGroup as u8,
            );
            TryMoveObjectEventToMapCoords(
                gSpecialVar_0x8005 as u8,
                (*gSaveBlock1Ptr).location.mapNum as u8,
                (*gSaveBlock1Ptr).location.mapGroup as u8,
                gSpecialVar_0x8006 as i16,
                gSpecialVar_0x8007 as i16,
            );
            TryOverrideObjectEventTemplateCoords(
                gSpecialVar_0x8005 as u8,
                (*gSaveBlock1Ptr).location.mapNum as u8,
                (*gSaveBlock1Ptr).location.mapGroup as u8,
            );
            break;
        }
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn HasDecorationSpace() -> u8 {
    let mut i: u16 = 0;
    i = 0;
    while i < sDecorationContext.size as u16 {
        if *sDecorationContext.items.at(i) == DECOR_NONE {
            return TRUE;
        }
        i += 1;
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn DecorationItemsMenuAction_AttemptPlace(taskId: u8) {
    if sDecorationContext.isPlayerRoom == TRUE
        && sCurDecorationCategory != DECORCAT_DOLL
        && sCurDecorationCategory != DECORCAT_CUSHION
    {
        StringExpandPlaceholders(
            gStringVar4.as_mut_ptr(),
            gText_CantPlaceInRoom.as_ptr().cast_mut(),
        );
        DisplayItemMessageOnField(
            taskId,
            gStringVar4.as_mut_ptr(),
            Some(ReturnToDecorationItemsAfterInvalidSelection),
        );
    } else if IsSelectedDecorInThePC() == TRUE {
        if HasDecorationSpace() == TRUE {
            FadeScreen(FADE_TO_BLACK, 0);
            gTasks[taskId].data[2] = 0;
            gTasks[taskId].func = Some(Task_PlaceDecoration);
        } else {
            ConvertIntToDecimalStringN(
                gStringVar1.as_mut_ptr(),
                sDecorationContext.size as i32,
                STR_CONV_MODE_RIGHT_ALIGN,
                2,
            );
            if sDecorationContext.isPlayerRoom == FALSE {
                StringExpandPlaceholders(
                    gStringVar4.as_mut_ptr(),
                    gText_NoMoreDecorations.as_ptr().cast_mut(),
                );
            } else {
                StringExpandPlaceholders(
                    gStringVar4.as_mut_ptr(),
                    gText_NoMoreDecorations2.as_ptr().cast_mut(),
                );
            }
            DisplayItemMessageOnField(
                taskId,
                gStringVar4.as_mut_ptr(),
                Some(ReturnToDecorationItemsAfterInvalidSelection),
            );
        }
    } else {
        StringExpandPlaceholders(
            gStringVar4.as_mut_ptr(),
            gText_InUseAlready.as_ptr().cast_mut(),
        );
        DisplayItemMessageOnField(
            taskId,
            gStringVar4.as_mut_ptr(),
            Some(ReturnToDecorationItemsAfterInvalidSelection),
        );
    }
}
pub(crate) unsafe extern "C" fn Task_PlaceDecoration(taskId: u8) {
    match gTasks[taskId].data[2] {
        0 => {
            if gPaletteFade.active() == 0 {
                SetInitialPositions(taskId);
                gTasks[taskId].data[2] = 1;
            }
        }
        1 => {
            gPaletteFade.set_bufferTransferDisabled(TRUE as u16);
            ConfigureCameraObjectForPlacingDecoration(
                &raw mut sPlaceDecorationGraphicsDataBuffer,
                *gCurDecorationItems.at(gCurDecorationIndex),
            );
            SetUpDecorationShape(taskId);
            SetUpPlacingDecorationPlayerAvatar(taskId, &raw mut sPlaceDecorationGraphicsDataBuffer);
            FadeInFromBlack();
            gPaletteFade.set_bufferTransferDisabled(FALSE as u16);
            gTasks[taskId].data[2] = 2;
        }
        2 => {
            if IsWeatherNotFadingIn() == TRUE {
                gTasks[taskId].data[12] = DECOR_ITEMS_MENU_PLACE;
                ContinueDecorating(taskId);
            }
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn ConfigureCameraObjectForPlacingDecoration(
    data: *mut PlaceDecorationGraphicsDataBuffer,
    decor: u8,
) {
    sDecor_CameraSpriteObjectIdx1 = gSprites[gFieldCamera.spriteId].data[0] as u8;
    gFieldCamera.spriteId = gpu_pal_decompress_alloc_tag_and_upload(data, decor) as u32;
    gSprites[gFieldCamera.spriteId].oam.set_priority(1);
    gSprites[gFieldCamera.spriteId].callback = Some(InitializePuttingAwayCursorSprite);
    gSprites[gFieldCamera.spriteId].x =
        sDecorationMovementInfo[(*(*data).decoration).shape].cameraX as i16;
    gSprites[gFieldCamera.spriteId].y =
        sDecorationMovementInfo[(*(*data).decoration).shape].cameraY as i16;
}
pub(crate) unsafe extern "C" fn SetUpPlacingDecorationPlayerAvatar(
    taskId: u8,
    data: *mut PlaceDecorationGraphicsDataBuffer,
) {
    let mut x: u8 = 0;
    x = 16 * gTasks[taskId].data[5] as u8
        + sDecorationMovementInfo[(*(*data).decoration).shape].cameraX
        - 8 * (gTasks[taskId].data[5] as u8 - 1);
    if (*(*data).decoration).shape == DECORSHAPE_3x1
        || (*(*data).decoration).shape == DECORSHAPE_3x3
        || (*(*data).decoration).shape == DECORSHAPE_3x2
    {
        x -= 8;
    }
    if (*gSaveBlock2Ptr).playerGender == MALE {
        sDecor_CameraSpriteObjectIdx2 = CreateObjectGraphicsSprite(
            OBJ_EVENT_GFX_BRENDAN_DECORATING,
            Some(SpriteCallbackDummy),
            x as i16,
            72,
            0,
        );
    } else {
        sDecor_CameraSpriteObjectIdx2 = CreateObjectGraphicsSprite(
            OBJ_EVENT_GFX_MAY_DECORATING,
            Some(SpriteCallbackDummy),
            x as i16,
            72,
            0,
        );
    }
    gSprites[sDecor_CameraSpriteObjectIdx2].oam.set_priority(1);
    DestroySprite(&raw mut gSprites[sDecor_CameraSpriteObjectIdx1]);
    sDecor_CameraSpriteObjectIdx1 = gFieldCamera.spriteId as u8;
}
pub(crate) unsafe extern "C" fn SetUpDecorationShape(taskId: u8) {
    match gDecorations[*gCurDecorationItems.at(gCurDecorationIndex)].shape {
        DECORSHAPE_1x1 => {
            gTasks[taskId].data[5] = 1;
            gTasks[taskId].data[6] = 1;
        }
        DECORSHAPE_2x1 => {
            gTasks[taskId].data[5] = 2;
            gTasks[taskId].data[6] = 1;
        }
        DECORSHAPE_3x1 => {
            gTasks[taskId].data[5] = 3;
            gTasks[taskId].data[6] = 1;
        }
        DECORSHAPE_4x2 => {
            gTasks[taskId].data[5] = 4;
            gTasks[taskId].data[6] = 2;
        }
        DECORSHAPE_2x2 => {
            gTasks[taskId].data[5] = 2;
            gTasks[taskId].data[6] = 2;
        }
        DECORSHAPE_1x2 => {
            gTasks[taskId].data[5] = 1;
            gTasks[taskId].data[6] = 2;
        }
        DECORSHAPE_1x3 => {
            gTasks[taskId].data[5] = 1;
            gTasks[taskId].data[6] = 3;
            gTasks[taskId].data[1] += 1;
        }
        DECORSHAPE_2x4 => {
            gTasks[taskId].data[5] = 2;
            gTasks[taskId].data[6] = 4;
        }
        DECORSHAPE_3x3 => {
            gTasks[taskId].data[5] = 3;
            gTasks[taskId].data[6] = 3;
        }
        DECORSHAPE_3x2 => {
            gTasks[taskId].data[5] = 3;
            gTasks[taskId].data[6] = 2;
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn AttemptPlaceDecoration(taskId: u8) {
    gTasks[taskId].data[10] = 0;
    gSprites[sDecor_CameraSpriteObjectIdx1].data[7] = 1;
    gSprites[sDecor_CameraSpriteObjectIdx2].data[7] = 1;
    ResetCursorMovement();
    AttemptPlaceDecoration_(taskId);
}
pub(crate) unsafe extern "C" fn AttemptCancelPlaceDecoration(taskId: u8) {
    gTasks[taskId].data[10] = 0;
    gSprites[sDecor_CameraSpriteObjectIdx1].data[7] = 1;
    gSprites[sDecor_CameraSpriteObjectIdx2].data[7] = 1;
    ResetCursorMovement();
    StringExpandPlaceholders(
        gStringVar4.as_mut_ptr(),
        gText_CancelDecorating.as_ptr().cast_mut(),
    );
    DisplayItemMessageOnField(
        taskId,
        gStringVar4.as_mut_ptr(),
        Some(CancelDecoratingPrompt),
    );
}
pub(crate) unsafe extern "C" fn IsSecretBaseTrainerSpot(behaviorAt: u8, layerType: u16) -> u8 {
    if !(MetatileBehavior_IsSecretBaseTrainerSpot(behaviorAt) == TRUE
        && layerType == METATILE_LAYER_TYPE_NORMAL)
    {
        return FALSE;
    }
    return TRUE;
}
pub(crate) unsafe extern "C" fn IsntInitialPosition(
    taskId: u8,
    x: i16,
    y: i16,
    layerType: u16,
) -> u8 {
    if x as i32 == gTasks[taskId].data[3] as i32 + MAP_OFFSET
        && y as i32 == gTasks[taskId].data[4] as i32 + MAP_OFFSET
        && layerType != METATILE_LAYER_TYPE_NORMAL
    {
        return FALSE;
    }
    return TRUE;
}
pub(crate) unsafe extern "C" fn IsFloorOrBoardAndHole(
    behaviorAt: u16,
    decoration: *mut Decoration,
) -> u8 {
    if MetatileBehavior_IsSecretBaseTrainerSpot(behaviorAt as u8) != TRUE {
        if (*decoration).id == DECOR_SOLID_BOARD
            && MetatileBehavior_IsSecretBaseHole(behaviorAt as u8) == TRUE
        {
            return TRUE;
        }
        if MetatileBehavior_IsNormal(behaviorAt as u8) != 0 {
            return TRUE;
        }
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn CanPlaceDecoration(taskId: u8, decoration: *mut Decoration) -> u8 {
    let mut i: u8 = 0;
    let mut j: u8 = 0;
    let mut behaviorAt: u8 = 0;
    let mut layerType: u16 = 0;
    let mut mapY: u8 = 0;
    let mut mapX: u8 = 0;
    let mut curY: i16 = 0;
    let mut curX: i16 = 0;
    mapY = gTasks[taskId].data[6] as u8;
    mapX = gTasks[taskId].data[5] as u8;
    match (*decoration).permission {
        DECORPERM_SOLID_FLOOR | DECORPERM_PASS_FLOOR => {
            i = 0;
            while i < mapY {
                curY = gTasks[taskId].data[1] - i as i16;
                j = 0;
                while j < mapX {
                    curX = gTasks[taskId].data[0] + j as i16;
                    behaviorAt = MapGridGetMetatileBehaviorAt(curX as i32, curY as i32) as u8;
                    layerType = GetMetatileAttributesById(
                        NUM_TILES_IN_PRIMARY
                            + *(*decoration)
                                .tiles
                                .at((mapY as i32 - 1 - i as i32) * mapX as i32 + j as i32),
                    ) & 0xF000;
                    if IsFloorOrBoardAndHole(behaviorAt as u16, decoration) == 0 {
                        return FALSE;
                    }
                    if IsntInitialPosition(taskId, curX, curY, layerType) == 0 {
                        return FALSE;
                    }
                    behaviorAt = GetObjectEventIdByPosition(curX as u16, curY as u16, 0);
                    if behaviorAt != 0 && behaviorAt != OBJECT_EVENTS_COUNT {
                        return FALSE;
                    }
                    j += 1;
                }
                i += 1;
            }
        }
        DECORPERM_BEHIND_FLOOR => {
            i = 0;
            while (i as i32) < mapY as i32 - 1 {
                curY = gTasks[taskId].data[1] - i as i16;
                j = 0;
                while j < mapX {
                    curX = gTasks[taskId].data[0] + j as i16;
                    behaviorAt = MapGridGetMetatileBehaviorAt(curX as i32, curY as i32) as u8;
                    layerType = GetMetatileAttributesById(
                        NUM_TILES_IN_PRIMARY
                            + *(*decoration)
                                .tiles
                                .at((mapY as i32 - 1 - i as i32) * mapX as i32 + j as i32),
                    ) & 0xF000;
                    if MetatileBehavior_IsNormal(behaviorAt) == 0
                        && IsSecretBaseTrainerSpot(behaviorAt, layerType) == 0
                    {
                        return FALSE;
                    }
                    if IsntInitialPosition(taskId, curX, curY, layerType) == 0 {
                        return FALSE;
                    }
                    if GetObjectEventIdByPosition(curX as u16, curY as u16, 0)
                        != OBJECT_EVENTS_COUNT
                    {
                        return FALSE;
                    }
                    j += 1;
                }
                i += 1;
            }
            curY = gTasks[taskId].data[1] - mapY as i16 + 1;
            j = 0;
            while j < mapX {
                curX = gTasks[taskId].data[0] + j as i16;
                behaviorAt = MapGridGetMetatileBehaviorAt(curX as i32, curY as i32) as u8;
                layerType =
                    GetMetatileAttributesById(NUM_TILES_IN_PRIMARY + *(*decoration).tiles.at(j))
                        & 0xF000;
                if MetatileBehavior_IsNormal(behaviorAt) == 0
                    && MetatileBehavior_IsSecretBaseNorthWall(behaviorAt) == 0
                {
                    return FALSE;
                }
                if IsntInitialPosition(taskId, curX, curY, layerType) == 0 {
                    return FALSE;
                }
                behaviorAt = GetObjectEventIdByPosition(curX as u16, curY as u16, 0);
                if behaviorAt != 0 && behaviorAt != OBJECT_EVENTS_COUNT {
                    return FALSE;
                }
                j += 1;
            }
        }
        DECORPERM_NA_WALL => {
            i = 0;
            while i < mapY {
                curY = gTasks[taskId].data[1] - i as i16;
                j = 0;
                while j < mapX {
                    curX = gTasks[taskId].data[0] + j as i16;
                    if MetatileBehavior_IsSecretBaseNorthWall(MapGridGetMetatileBehaviorAt(
                        curX as i32,
                        curY as i32,
                    ) as u8)
                        == 0
                    {
                        return FALSE;
                    }
                    if MapGridGetMetatileIdAt(curX as i32, curY as i32 + 1)
                        == METATILE_SecretBase_SandOrnament_BrokenBase
                    {
                        return FALSE;
                    }
                    j += 1;
                }
                i += 1;
            }
        }
        DECORPERM_SPRITE => {
            curY = gTasks[taskId].data[1];
            j = 0;
            while j < mapX {
                curX = gTasks[taskId].data[0] + j as i16;
                behaviorAt = MapGridGetMetatileBehaviorAt(curX as i32, curY as i32) as u8;
                if (*decoration).shape == DECORSHAPE_1x2 {
                    if MetatileBehavior_HoldsLargeDecoration(behaviorAt) == 0 {
                        return FALSE;
                    }
                } else if MetatileBehavior_HoldsSmallDecoration(behaviorAt) == 0 {
                    if MetatileBehavior_HoldsLargeDecoration(behaviorAt) == 0 {
                        return FALSE;
                    }
                }
                if GetObjectEventIdByPosition(curX as u16, curY as u16, 0) != OBJECT_EVENTS_COUNT {
                    return FALSE;
                }
                j += 1;
            }
        }
        _ => {}
    }
    return TRUE;
}
pub(crate) unsafe extern "C" fn AttemptPlaceDecoration_(taskId: u8) {
    if CanPlaceDecoration(
        taskId,
        (&raw const gDecorations[*gCurDecorationItems.at(gCurDecorationIndex)]).cast_mut(),
    ) == TRUE
    {
        StringExpandPlaceholders(
            gStringVar4.as_mut_ptr(),
            gText_PlaceItHere.as_ptr().cast_mut(),
        );
        DisplayItemMessageOnField(
            taskId,
            gStringVar4.as_mut_ptr(),
            Some(PlaceDecorationPrompt),
        );
    } else {
        PlaySE(SE_FAILURE);
        StringExpandPlaceholders(
            gStringVar4.as_mut_ptr(),
            gText_CantBePlacedHere.as_ptr().cast_mut(),
        );
        DisplayItemMessageOnField(
            taskId,
            gStringVar4.as_mut_ptr(),
            Some(CantPlaceDecorationPrompt),
        );
    }
}
pub(crate) unsafe extern "C" fn PlaceDecorationPrompt(taskId: u8) {
    DisplayYesNoMenuDefaultYes();
    DoYesNoFuncWithChoice(
        taskId,
        (&raw const *sPlaceDecorationYesNoFunctions).cast_mut(),
    );
}
pub(crate) unsafe extern "C" fn PlaceDecoration(taskId: u8) {
    ClearDialogWindowAndFrame(0, 0);
    PlaceDecoration_(taskId);
    if gDecorations[*gCurDecorationItems.at(gCurDecorationIndex)].permission != DECORPERM_SPRITE {
        ShowDecorationOnMap(
            gTasks[taskId].data[0] as u16,
            gTasks[taskId].data[1] as u16,
            *gCurDecorationItems.at(gCurDecorationIndex) as u16,
        );
    } else {
        sCurDecorMapX = gTasks[taskId].data[0] as u16 - MAP_OFFSET as u16;
        sCurDecorMapY = gTasks[taskId].data[1] as u16 - MAP_OFFSET as u16;
        ScriptContext_SetupScript(SecretBase_EventScript_SetDecoration.as_ptr().cast_mut());
    }
    gSprites[sDecor_CameraSpriteObjectIdx1].y += 2;
    if gMapHeader.regionMapSectionId == MAPSEC_SECRET_BASE {
        TryPutSecretBaseVisitOnAir();
    }
    CancelDecorating_(taskId);
}
pub(crate) unsafe extern "C" fn PlaceDecoration_(taskId: u8) {
    let mut i: u16 = 0;
    i = 0;
    while i < sDecorationContext.size as u16 {
        if *sDecorationContext.items.at(i) == DECOR_NONE {
            *sDecorationContext.items.at(i) = *gCurDecorationItems.at(gCurDecorationIndex);
            *sDecorationContext.pos.at(i) = ((gTasks[taskId].data[0] as u8 - MAP_OFFSET as u8)
                << 4)
                + (gTasks[taskId].data[1] as u8 - MAP_OFFSET as u8);
            break;
        }
        i += 1;
    }
    if sDecorationContext.isPlayerRoom == 0 {
        i = 0;
        while i < DECOR_MAX_SECRET_BASE as u16 {
            if sSecretBaseItemsIndicesBuffer[i] == DECOR_NONE {
                sSecretBaseItemsIndicesBuffer[i] = gCurDecorationIndex + 1;
                break;
            }
            i += 1;
        }
    } else {
        i = 0;
        while i < DECOR_MAX_PLAYERS_HOUSE as u16 {
            if sPlayerRoomItemsIndicesBuffer[i] == DECOR_NONE {
                sPlayerRoomItemsIndicesBuffer[i] = gCurDecorationIndex + 1;
                break;
            }
            i += 1;
        }
    }
}
pub(crate) unsafe extern "C" fn CancelDecoratingPrompt(taskId: u8) {
    DisplayYesNoMenuDefaultYes();
    DoYesNoFuncWithChoice(
        taskId,
        (&raw const *sCancelDecoratingYesNoFunctions).cast_mut(),
    );
}
pub(crate) unsafe extern "C" fn CancelDecorating(taskId: u8) {
    ClearDialogWindowAndFrame(0, 0);
    CancelDecorating_(taskId);
}
pub(crate) unsafe extern "C" fn CancelDecorating_(taskId: u8) {
    FadeScreen(FADE_TO_BLACK, 0);
    gTasks[taskId].data[2] = 0;
    gTasks[taskId].func = Some(c1_overworld_prev_quest);
}
pub(crate) unsafe extern "C" fn c1_overworld_prev_quest(taskId: u8) {
    match gTasks[taskId].data[2] {
        0 => {
            LockPlayerFieldControls();
            if gPaletteFade.active() == 0 {
                WarpToInitialPosition(taskId);
                gTasks[taskId].data[2] = 1;
            }
        }
        1 => {
            FreePlayerSpritePalette();
            FreeSpritePaletteByTag(PLACE_DECORATION_SELECTOR_TAG);
            gFieldCallback = Some(FieldCB_InitDecorationItemsWindow);
            SetMainCallback2(Some(CB2_ReturnToField));
            DestroyTask(taskId);
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn Task_InitDecorationItemsWindow(taskId: u8) {
    let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
    match *data.at(2) {
        0 => {
            HideSecretBaseDecorationSprites();
            *data.at(2) += 1;
        }
        1 => {
            ScriptContext_SetupScript(SecretBase_EventScript_InitDecorations.as_ptr().cast_mut());
            *data.at(2) += 1;
        }
        2 => {
            LockPlayerFieldControls();
            *data.at(2) += 1;
        }
        3 => {
            if IsWeatherNotFadingIn() == TRUE {
                gTasks[taskId].func = Some(HandleDecorationItemsMenuInput);
            }
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn FieldCB_InitDecorationItemsWindow() {
    let mut taskId: u8 = 0;
    LockPlayerFieldControls();
    FadeInFromBlack();
    taskId = CreateTask(Some(Task_InitDecorationItemsWindow), 8);
    AddDecorationItemsWindow(taskId);
    gTasks[taskId].data[2] = 0;
}
pub(crate) unsafe extern "C" fn ApplyCursorMovement_IsInvalid(taskId: u8) -> u8 {
    let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
    if sDecorationLastDirectionMoved == DIR_SOUTH && *data.at(1) as i32 - *data.at(6) as i32 - 6 < 0
    {
        *data.at(1) += 1;
        return FALSE;
    }
    if sDecorationLastDirectionMoved == DIR_NORTH
        && *data.at(1) as i32 - 7 >= (*gMapHeader.mapLayout).height
    {
        *data.at(1) -= 1;
        return FALSE;
    }
    if sDecorationLastDirectionMoved == DIR_WEST && *data as i32 - 7 < 0 {
        *data += 1;
        return FALSE;
    }
    if sDecorationLastDirectionMoved == DIR_EAST
        && *data as i32 + *data.at(5) as i32 - 8 >= (*gMapHeader.mapLayout).width
    {
        *data -= 1;
        return FALSE;
    }
    return TRUE;
}
pub(crate) unsafe extern "C" fn IsHoldingDirection() -> u8 {
    let mut heldKeys: u16 = gMain.heldKeys & DPAD_ANY as u16;
    if heldKeys != DPAD_UP as u16
        && heldKeys != DPAD_DOWN as u16
        && heldKeys != DPAD_LEFT as u16
        && heldKeys != DPAD_RIGHT as u16
    {
        return FALSE;
    }
    return TRUE;
}
pub(crate) unsafe extern "C" fn ResetCursorMovement() {
    sDecorationLastDirectionMoved = 0;
    gSprites[sDecor_CameraSpriteObjectIdx1].data[2] = 0;
    gSprites[sDecor_CameraSpriteObjectIdx1].data[3] = 0;
}
pub(crate) unsafe extern "C" fn Task_SelectLocation(taskId: u8) {
    let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
    if gSprites[sDecor_CameraSpriteObjectIdx1].data[4] == 0 {
        if *data.at(10) == A_BUTTON as i16 {
            sPlacePutAwayYesNoFunctions[*data.at(12)]
                .yesFunc
                .unwrap_unchecked()(taskId);
            return;
        }
        if *data.at(10) == B_BUTTON as i16 {
            sPlacePutAwayYesNoFunctions[*data.at(12)]
                .noFunc
                .unwrap_unchecked()(taskId);
            return;
        }
        if gMain.heldKeys as i32 & DPAD_ANY == DPAD_UP {
            sDecorationLastDirectionMoved = DIR_SOUTH;
            gSprites[sDecor_CameraSpriteObjectIdx1].data[2] = 0;
            gSprites[sDecor_CameraSpriteObjectIdx1].data[3] = -2;
            *data.at(1) -= 1;
        }
        if gMain.heldKeys as i32 & DPAD_ANY == DPAD_DOWN {
            sDecorationLastDirectionMoved = DIR_NORTH;
            gSprites[sDecor_CameraSpriteObjectIdx1].data[2] = 0;
            gSprites[sDecor_CameraSpriteObjectIdx1].data[3] = 2;
            *data.at(1) += 1;
        }
        if gMain.heldKeys as i32 & DPAD_ANY == DPAD_LEFT {
            sDecorationLastDirectionMoved = DIR_WEST;
            gSprites[sDecor_CameraSpriteObjectIdx1].data[2] = -2;
            gSprites[sDecor_CameraSpriteObjectIdx1].data[3] = 0;
            *data -= 1;
        }
        if gMain.heldKeys as i32 & DPAD_ANY == DPAD_RIGHT {
            sDecorationLastDirectionMoved = DIR_EAST;
            gSprites[sDecor_CameraSpriteObjectIdx1].data[2] = 2;
            gSprites[sDecor_CameraSpriteObjectIdx1].data[3] = 0;
            *data += 1;
        }
        if IsHoldingDirection() == 0 || ApplyCursorMovement_IsInvalid(taskId) == 0 {
            ResetCursorMovement();
        }
    }
    if sDecorationLastDirectionMoved != 0 {
        gSprites[sDecor_CameraSpriteObjectIdx1].data[4] += 1;
        gSprites[sDecor_CameraSpriteObjectIdx1].data[4] &= 7;
    }
    if *data.at(10) == 0 {
        if gMain.newKeys as i32 & A_BUTTON != 0 {
            *data.at(10) = A_BUTTON as i16;
        }
        if gMain.newKeys as i32 & B_BUTTON != 0 {
            *data.at(10) = B_BUTTON as i16;
        }
    }
}
pub(crate) unsafe extern "C" fn ContinueDecorating(taskId: u8) {
    ClearDialogWindowAndFrame(0, TRUE);
    gSprites[sDecor_CameraSpriteObjectIdx1].data[7] = 0;
    gTasks[taskId].data[10] = 0;
    gTasks[taskId].func = Some(Task_SelectLocation);
}
pub(crate) unsafe extern "C" fn CantPlaceDecorationPrompt(taskId: u8) {
    if gMain.newKeys as i32 & A_BUTTON != 0 || gMain.newKeys as i32 & B_BUTTON != 0 {
        ContinueDecorating(taskId);
    }
}
pub(crate) unsafe extern "C" fn ClearPlaceDecorationGraphicsDataBuffer(
    data: *mut PlaceDecorationGraphicsDataBuffer,
) {
    {
        {
            let mut tmp: u16 = 0;
            volatile_write(&raw mut tmp, 0);
            CpuSet(&raw mut tmp as *mut c_void, data as *mut c_void, 0x1000452);
        }
    }
}
pub(crate) unsafe extern "C" fn CopyPalette(dest: *mut u16, pal: u16) {
    CpuFastSet(
        (*gTilesetPointer_SecretBase).palettes.at(pal) as *mut c_void,
        dest as *mut c_void,
        8,
    );
}
pub(crate) unsafe extern "C" fn CopyTile(mut dest: *mut u8, mut tile: u16) {
    let mut buffer: CArray<u8, 32> = zeroed();
    let mut mode: u16 = 0;
    let mut i: u16 = 0;
    mode = tile >> 10;
    if tile != 0 {
        tile &= 0x03FF;
    }
    CpuFastSet(
        (*gTilesetPointer_SecretBase).tiles.at(tile as u32 * 32 / 4) as *mut c_void,
        buffer.as_mut_ptr() as *mut c_void,
        8,
    );
    match mode {
        0 => {
            CpuFastSet(buffer.as_mut_ptr() as *mut c_void, dest as *mut c_void, 8);
        }
        1 => {
            i = 0;
            while i < 8 {
                *dest.at(4 * i as i32 + 0) = (buffer[4 * (i as i32 + 1) - 1] >> 4)
                    + ((buffer[4 * (i as i32 + 1) - 1] & 0x0F) << 4);
                *dest.at(4 * i as i32 + 1) = (buffer[4 * (i as i32 + 1) - 2] >> 4)
                    + ((buffer[4 * (i as i32 + 1) - 2] & 0x0F) << 4);
                *dest.at(4 * i as i32 + 2) = (buffer[4 * (i as i32 + 1) - 3] >> 4)
                    + ((buffer[4 * (i as i32 + 1) - 3] & 0x0F) << 4);
                *dest.at(4 * i as i32 + 3) = (buffer[4 * (i as i32 + 1) - 4] >> 4)
                    + ((buffer[4 * (i as i32 + 1) - 4] & 0x0F) << 4);
                i += 1;
            }
        }
        2 => {
            i = 0;
            while i < 8 {
                *dest.at(4 * i as i32 + 0) = buffer[4 * (7 - i as i32) + 0];
                *dest.at(4 * i as i32 + 1) = buffer[4 * (7 - i as i32) + 1];
                *dest.at(4 * i as i32 + 2) = buffer[4 * (7 - i as i32) + 2];
                *dest.at(4 * i as i32 + 3) = buffer[4 * (7 - i as i32) + 3];
                i += 1;
            }
        }
        3 => {
            i = 0;
            while i < 32 {
                *dest.at(i) = (buffer[31 - i as i32] >> 4) + ((buffer[31 - i as i32] & 0x0F) << 4);
                i += 1;
            }
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn SetDecorSelectionBoxTiles(
    data: *mut PlaceDecorationGraphicsDataBuffer,
) {
    let mut i: u16 = 0;
    i = 0;
    while i < 64 {
        CopyTile(&raw mut (*data).image[i as i32 * 32], (*data).tiles[i]);
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn GetMetatile(tile: u16) -> u16 {
    return *(*gTilesetPointer_SecretBaseRedCave).metatiles.at(tile) & 0xFFF;
}
pub(crate) unsafe extern "C" fn SetDecorSelectionMetatiles(
    data: *mut PlaceDecorationGraphicsDataBuffer,
) {
    let mut i: u8 = 0;
    let mut shape: u8 = 0;
    shape = (*(*data).decoration).shape;
    i = 0;
    while i < sDecorTilemaps[shape].size {
        (*data).tiles[*sDecorTilemaps[shape].tiles.at(i)] = GetMetatile(
            *(*(*data).decoration)
                .tiles
                .at(*sDecorTilemaps[shape].y.at(i))
                * NUM_TILES_PER_METATILE as u16
                + *sDecorTilemaps[shape].x.at(i) as u16,
        );
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn SetDecorSelectionBoxOamAttributes(decorShape: u8) {
    sDecorSelectorOam.set_y(0);
    sDecorSelectorOam.set_affineMode(ST_OAM_AFFINE_OFF);
    sDecorSelectorOam.set_objMode(ST_OAM_OBJ_NORMAL as u32);
    sDecorSelectorOam.set_mosaic(FALSE as u32);
    sDecorSelectorOam.set_bpp(ST_OAM_4BPP);
    sDecorSelectorOam.set_shape(sDecorationMovementInfo[decorShape].shape as u32);
    sDecorSelectorOam.set_x(0);
    sDecorSelectorOam.set_matrixNum(0);
    sDecorSelectorOam.set_size(sDecorationMovementInfo[decorShape].size as u32);
    sDecorSelectorOam.set_tileNum(0);
    sDecorSelectorOam.set_priority(0);
    sDecorSelectorOam.set_paletteNum(0);
}
pub(crate) unsafe extern "C" fn InitializePuttingAwayCursorSprite(sprite: *mut Sprite) {
    (*sprite).data[2] = 0;
    (*sprite).data[3] = 0;
    (*sprite).data[4] = 0;
    (*sprite).data[5] = 0;
    (*sprite).data[6] = 0;
    (*sprite).data[7] = 0;
    (*sprite).callback = Some(InitializePuttingAwayCursorSprite2);
}
pub(crate) unsafe extern "C" fn InitializePuttingAwayCursorSprite2(sprite: *mut Sprite) {
    if (*sprite).data[7] == 0 {
        if (*sprite).data[6] < 15 {
            (*sprite).set_invisible(0);
        } else {
            (*sprite).set_invisible(1);
        }
        (*sprite).data[6] += 1;
        (*sprite).data[6] &= 0x1F;
    } else {
        (*sprite).set_invisible(FALSE as u16);
    }
}
pub(crate) unsafe extern "C" fn gpu_pal_decompress_alloc_tag_and_upload(
    data: *mut PlaceDecorationGraphicsDataBuffer,
    decor: u8,
) -> u8 {
    ClearPlaceDecorationGraphicsDataBuffer(data);
    (*data).decoration = (&raw const gDecorations[decor]).cast_mut();
    if (*(*data).decoration).permission == DECORPERM_SPRITE {
        return CreateObjectGraphicsSprite(
            *(*(*data).decoration).tiles,
            Some(SpriteCallbackDummy),
            0,
            0,
            1,
        );
    }
    FreeSpritePaletteByTag(PLACE_DECORATION_SELECTOR_TAG);
    SetDecorSelectionMetatiles(data);
    SetDecorSelectionBoxOamAttributes((*(*data).decoration).shape);
    SetDecorSelectionBoxTiles(data);
    CopyPalette(
        (*data).palette.as_mut_ptr(),
        *(*gTilesetPointer_SecretBaseRedCave)
            .metatiles
            .at(*(*(*data).decoration).tiles as i32 * NUM_TILES_PER_METATILE + 7)
            >> 12,
    );
    LoadSpritePalette((&raw const *sSpritePal_PlaceDecoration).cast_mut());
    return CreateSprite(
        (&raw const *sDecorationSelectorSpriteTemplate).cast_mut(),
        0,
        0,
        0,
    );
}
pub(crate) unsafe extern "C" fn AddDecorationIconObjectFromIconTable(
    tilesTag: u16,
    paletteTag: u16,
    decor: u8,
) -> u8 {
    let mut sheet: SpriteSheet = zeroed();
    let mut palette: CompressedSpritePalette = zeroed();
    let mut template: *mut SpriteTemplate = null_mut();
    let mut spriteId: u8 = 0;
    if AllocItemIconTemporaryBuffers() == 0 {
        return MAX_SPRITES;
    }
    LZDecompressWram(
        GetDecorationIconPicOrPalette(decor as u16, 0),
        gItemIconDecompressionBuffer as *mut c_void,
    );
    CopyItemIconPicTo4x4Buffer(
        gItemIconDecompressionBuffer as *mut c_void,
        gItemIcon4x4Buffer as *mut c_void,
    );
    sheet.data = gItemIcon4x4Buffer as *mut c_void;
    sheet.size = 0x200;
    sheet.tag = tilesTag;
    LoadSpriteSheet(&raw mut sheet);
    palette.data = GetDecorationIconPicOrPalette(decor as u16, 1);
    palette.tag = paletteTag;
    LoadCompressedSpritePalette(&raw mut palette);
    template = Alloc(24) as *mut SpriteTemplate;
    *template = gItemIconSpriteTemplate;
    (*template).tileTag = tilesTag;
    (*template).paletteTag = paletteTag;
    spriteId = CreateSprite(template, 0, 0, 0);
    FreeItemIconTemporaryBuffers();
    Free(template as *mut c_void);
    return spriteId;
}
pub(crate) unsafe extern "C" fn GetDecorationIconPicOrPalette(
    mut decor: u16,
    mode: u8,
) -> *mut u32 {
    if decor > NUM_DECORATIONS as u16 {
        decor = DECOR_NONE as u16;
    }
    return gDecorIconTable[decor][mode];
}
pub(crate) unsafe extern "C" fn AddDecorationIconObjectFromObjectEvent(
    tilesTag: u16,
    paletteTag: u16,
    decor: u8,
) -> u8 {
    let mut spriteId: u8 = 0;
    let mut sheet: SpriteSheet = zeroed();
    let mut palette: SpritePalette = zeroed();
    let mut template: *mut SpriteTemplate = null_mut();
    ClearPlaceDecorationGraphicsDataBuffer(&raw mut sPlaceDecorationGraphicsDataBuffer);
    sPlaceDecorationGraphicsDataBuffer.decoration = (&raw const gDecorations[decor]).cast_mut();
    if (*sPlaceDecorationGraphicsDataBuffer.decoration).permission != DECORPERM_SPRITE {
        SetDecorSelectionMetatiles(&raw mut sPlaceDecorationGraphicsDataBuffer);
        SetDecorSelectionBoxOamAttributes((*sPlaceDecorationGraphicsDataBuffer.decoration).shape);
        SetDecorSelectionBoxTiles(&raw mut sPlaceDecorationGraphicsDataBuffer);
        CopyPalette(
            sPlaceDecorationGraphicsDataBuffer.palette.as_mut_ptr(),
            *(*gTilesetPointer_SecretBaseRedCave).metatiles.at(
                *(*sPlaceDecorationGraphicsDataBuffer.decoration).tiles as i32
                    * NUM_TILES_PER_METATILE
                    + 7,
            ) >> 12,
        );
        sheet.data = sPlaceDecorationGraphicsDataBuffer.image.as_mut_ptr() as *mut c_void;
        sheet.size = sDecorShapeSizes[(*sPlaceDecorationGraphicsDataBuffer.decoration).shape] * 32;
        sheet.tag = tilesTag;
        LoadSpriteSheet(&raw mut sheet);
        palette.data = sPlaceDecorationGraphicsDataBuffer.palette.as_mut_ptr();
        palette.tag = paletteTag;
        LoadSpritePalette(&raw mut palette);
        template = Alloc(24) as *mut SpriteTemplate;
        *template = *sDecorWhilePlacingSpriteTemplate;
        (*template).tileTag = tilesTag;
        (*template).paletteTag = paletteTag;
        spriteId = CreateSprite(template, 0, 0, 0);
        Free(template as *mut c_void);
    } else {
        spriteId = CreateObjectGraphicsSprite(
            *(*sPlaceDecorationGraphicsDataBuffer.decoration).tiles,
            Some(SpriteCallbackDummy),
            0,
            0,
            1,
        );
    }
    return spriteId;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AddDecorationIconObject(
    decor: u8,
    x: i16,
    y: i16,
    priority: u8,
    tilesTag: u16,
    paletteTag: u16,
) -> u8 {
    let mut spriteId: u8 = 0;
    if decor > NUM_DECORATIONS {
        spriteId = AddDecorationIconObjectFromIconTable(tilesTag, paletteTag, DECOR_NONE);
        if spriteId == MAX_SPRITES {
            return MAX_SPRITES;
        }
        gSprites[spriteId].x2 = x + 4;
        gSprites[spriteId].y2 = y + 4;
    } else if gDecorIconTable[decor][0].is_null() {
        spriteId = AddDecorationIconObjectFromObjectEvent(tilesTag, paletteTag, decor);
        if spriteId == MAX_SPRITES {
            return MAX_SPRITES;
        }
        gSprites[spriteId].x2 = x;
        if decor == DECOR_SILVER_SHIELD || decor == DECOR_GOLD_SHIELD {
            gSprites[spriteId].y2 = y - 4;
        } else {
            gSprites[spriteId].y2 = y;
        }
    } else {
        spriteId = AddDecorationIconObjectFromIconTable(tilesTag, paletteTag, decor);
        if spriteId == MAX_SPRITES {
            return MAX_SPRITES;
        }
        gSprites[spriteId].x2 = x + 4;
        gSprites[spriteId].y2 = y + 4;
    }
    gSprites[spriteId].oam.set_priority(priority as u16);
    return spriteId;
}
pub(crate) unsafe extern "C" fn ClearDecorationContextIndex(idx: u8) {
    *sDecorationContext.items.at(idx) = DECOR_NONE;
    *sDecorationContext.pos.at(idx) = 0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PutAwayDecorationIteration() {
    let mut i: u16 = 0;
    gSpecialVar_0x8005 = 0;
    gSpecialVar_Result = FALSE as u16;
    if gSpecialVar_0x8004 == sCurDecorSelectedInRearrangement as u16 {
        gSpecialVar_Result = TRUE as u16;
    } else if gDecorations[*sDecorationContext
        .items
        .at(sDecorRearrangementDataBuffer[gSpecialVar_0x8004].idx)]
    .permission
        == DECORPERM_SPRITE
    {
        gSpecialVar_0x8005 = sDecorRearrangementDataBuffer[gSpecialVar_0x8004].flagId;
        ClearDecorationContextIndex(sDecorRearrangementDataBuffer[gSpecialVar_0x8004].idx);
        i = 0;
        while i < (*gMapHeader.events).objectEventCount as u16 {
            if (*(*gMapHeader.events).objectEvents.at(i)).flagId == gSpecialVar_0x8005 {
                gSpecialVar_0x8006 = (*(*gMapHeader.events).objectEvents.at(i)).localId as u16;
                break;
            }
            i += 1;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetObjectEventLocalIdByFlag() {
    let mut i: u8 = 0;
    i = 0;
    while i < (*gMapHeader.events).objectEventCount {
        if (*(*gMapHeader.events).objectEvents.at(i)).flagId == gSpecialVar_0x8004 {
            gSpecialVar_0x8005 = (*(*gMapHeader.events).objectEvents.at(i)).localId as u16;
            break;
        }
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn ClearRearrangementNonSprites() {
    let mut i: u8 = 0;
    let mut y: u8 = 0;
    let mut x: u8 = 0;
    let mut posX: i32 = 0;
    let mut posY: i32 = 0;
    let mut perm: u8 = 0;
    i = 0;
    while i < sCurDecorSelectedInRearrangement {
        perm = gDecorations[*sDecorationContext
            .items
            .at(sDecorRearrangementDataBuffer[i].idx)]
        .permission;
        posX = (*sDecorationContext
            .pos
            .at(sDecorRearrangementDataBuffer[i].idx)
            >> 4) as i32;
        posY = *sDecorationContext
            .pos
            .at(sDecorRearrangementDataBuffer[i].idx) as i32
            & 0x0F;
        if perm != DECORPERM_SPRITE {
            y = 0;
            while y < sDecorRearrangementDataBuffer[i].height {
                x = 0;
                while x < sDecorRearrangementDataBuffer[i].width {
                    MapGridSetMetatileEntryAt(
                        posX + MAP_OFFSET + x as i32,
                        posY + MAP_OFFSET - y as i32,
                        *(*gMapHeader.mapLayout).map.at(posX
                            + x as i32
                            + (*gMapHeader.mapLayout).width * (posY - y as i32))
                            | 0x3000,
                    );
                    x += 1;
                }
                y += 1;
            }
            ClearDecorationContextIndex(sDecorRearrangementDataBuffer[i].idx);
        }
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn Task_PutAwayDecoration(taskId: u8) {
    match gTasks[taskId].data[2] {
        0 => {
            ClearRearrangementNonSprites();
            gTasks[taskId].data[2] = 1;
        }
        1 => {
            if gPaletteFade.active() == 0 {
                DrawWholeMapView();
                ScriptContext_SetupScript(
                    SecretBase_EventScript_PutAwayDecoration.as_ptr().cast_mut(),
                );
                ClearDialogWindowAndFrame(0, TRUE);
                gTasks[taskId].data[2] = 2;
            }
        }
        2 => {
            LockPlayerFieldControls();
            IdentifyOwnedDecorationsCurrentlyInUseInternal(taskId);
            FadeInFromBlack();
            gTasks[taskId].data[2] = 3;
        }
        3 => {
            if IsWeatherNotFadingIn() == TRUE {
                StringExpandPlaceholders(
                    gStringVar4.as_mut_ptr(),
                    gText_DecorationReturnedToPC.as_ptr().cast_mut(),
                );
                DisplayItemMessageOnField(
                    taskId,
                    gStringVar4.as_mut_ptr(),
                    Some(ContinuePuttingAwayDecorationsPrompt),
                );
                if gMapHeader.regionMapSectionId == MAPSEC_SECRET_BASE {
                    TryPutSecretBaseVisitOnAir();
                }
            }
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn HasDecorationsInUse(taskId: u8) -> u8 {
    let mut i: u16 = 0;
    i = 0;
    while i < sDecorationContext.size as u16 {
        if *sDecorationContext.items.at(i) != DECOR_NONE {
            return TRUE;
        }
        i += 1;
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn SetUpPuttingAwayDecorationPlayerAvatar() {
    GetPlayerFacingDirection();
    sDecor_CameraSpriteObjectIdx1 = gSprites[gFieldCamera.spriteId].data[0] as u8;
    LoadPlayerSpritePalette();
    gFieldCamera.spriteId = CreateSprite(
        (&raw const *sPuttingAwayCursorSpriteTemplate).cast_mut(),
        120,
        80,
        0,
    ) as u32;
    if (*gSaveBlock2Ptr).playerGender == MALE {
        sDecor_CameraSpriteObjectIdx2 = CreateObjectGraphicsSprite(
            OBJ_EVENT_GFX_BRENDAN_DECORATING,
            Some(SpriteCallbackDummy),
            136,
            72,
            0,
        );
    } else {
        sDecor_CameraSpriteObjectIdx2 = CreateObjectGraphicsSprite(
            OBJ_EVENT_GFX_MAY_DECORATING,
            Some(SpriteCallbackDummy),
            136,
            72,
            0,
        );
    }
    gSprites[sDecor_CameraSpriteObjectIdx2].oam.set_priority(1);
    DestroySprite(&raw mut gSprites[sDecor_CameraSpriteObjectIdx1]);
    sDecor_CameraSpriteObjectIdx1 = gFieldCamera.spriteId as u8;
    gSprites[sDecor_CameraSpriteObjectIdx1].oam.set_priority(1);
}
pub(crate) unsafe extern "C" fn Task_ContinuePuttingAwayDecorations(taskId: u8) {
    let mut data: *mut i16 = null_mut();
    data = gTasks[taskId].data.as_mut_ptr();
    match *data.at(2) {
        0 => {
            if gPaletteFade.active() == 0 {
                SetInitialPositions(taskId);
                *data.at(2) = 1;
                *data.at(6) = 1;
                *data.at(5) = 1;
            }
        }
        1 => {
            SetUpPuttingAwayDecorationPlayerAvatar();
            FadeInFromBlack();
            *data.at(2) = 2;
        }
        2 => {
            if IsWeatherNotFadingIn() == TRUE {
                *data.at(12) = DECOR_ITEMS_MENU_PUT_AWAY;
                ContinuePuttingAwayDecorations(taskId);
            }
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn ContinuePuttingAwayDecorations(taskId: u8) {
    ClearDialogWindowAndFrame(0, TRUE);
    gSprites[sDecor_CameraSpriteObjectIdx1].data[7] = 0;
    gSprites[sDecor_CameraSpriteObjectIdx1].set_invisible(FALSE as u16);
    gSprites[sDecor_CameraSpriteObjectIdx1].callback = Some(InitializeCameraSprite1);
    gSprites[sDecor_CameraSpriteObjectIdx2].x = 136;
    gSprites[sDecor_CameraSpriteObjectIdx2].y = 72;
    gTasks[taskId].data[10] = 0;
    gTasks[taskId].func = Some(Task_SelectLocation);
}
pub(crate) unsafe extern "C" fn AttemptPutAwayDecoration(taskId: u8) {
    gTasks[taskId].data[10] = 0;
    ResetCursorMovement();
    AttemptPutAwayDecoration_(taskId);
}
pub(crate) unsafe extern "C" fn AttemptCancelPutAwayDecoration(taskId: u8) {
    gTasks[taskId].data[10] = 0;
    ResetCursorMovement();
    gSprites[sDecor_CameraSpriteObjectIdx1].set_invisible(FALSE as u16);
    gSprites[sDecor_CameraSpriteObjectIdx1].callback = Some(SpriteCallbackDummy);
    StringExpandPlaceholders(
        gStringVar4.as_mut_ptr(),
        gText_StopPuttingAwayDecorations.as_ptr().cast_mut(),
    );
    DisplayItemMessageOnField(
        taskId,
        gStringVar4.as_mut_ptr(),
        Some(StopPuttingAwayDecorationsPrompt),
    );
}
pub(crate) unsafe extern "C" fn AttemptPutAwayDecoration_(taskId: u8) {
    let mut data: *mut i16 = null_mut();
    let mut behavior: u8 = 0;
    AttemptMarkDecorUnderCursorForRemoval(taskId);
    if sCurDecorSelectedInRearrangement != 0 {
        StringExpandPlaceholders(
            gStringVar4.as_mut_ptr(),
            gText_ReturnDecorationToPC.as_ptr().cast_mut(),
        );
        DisplayItemMessageOnField(
            taskId,
            gStringVar4.as_mut_ptr(),
            Some(ReturnDecorationPrompt),
        );
    } else {
        data = gTasks[taskId].data.as_mut_ptr();
        behavior = MapGridGetMetatileBehaviorAt(*data as i32, *data.at(1) as i32) as u8;
        if MetatileBehavior_IsSecretBasePC(behavior) == TRUE
            || MetatileBehavior_IsPlayerRoomPCOn(behavior) == TRUE
        {
            gSprites[sDecor_CameraSpriteObjectIdx1].set_invisible(FALSE as u16);
            gSprites[sDecor_CameraSpriteObjectIdx1].callback = Some(SpriteCallbackDummy);
            StringExpandPlaceholders(
                gStringVar4.as_mut_ptr(),
                gText_StopPuttingAwayDecorations.as_ptr().cast_mut(),
            );
            DisplayItemMessageOnField(
                taskId,
                gStringVar4.as_mut_ptr(),
                Some(StopPuttingAwayDecorationsPrompt),
            );
        } else {
            StringExpandPlaceholders(
                gStringVar4.as_mut_ptr(),
                gText_NoDecorationHere.as_ptr().cast_mut(),
            );
            DisplayItemMessageOnField(
                taskId,
                gStringVar4.as_mut_ptr(),
                Some(ContinuePuttingAwayDecorationsPrompt),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn ContinuePuttingAwayDecorationsPrompt(taskId: u8) {
    if gMain.newKeys as i32 & A_BUTTON != 0 || gMain.newKeys as i32 & B_BUTTON != 0 {
        ContinuePuttingAwayDecorations(taskId);
    }
}
pub(crate) unsafe extern "C" fn SetDecorRearrangementShape(
    decor: u8,
    data: *mut DecorRearrangementDataBuffer,
) {
    if gDecorations[decor].shape == DECORSHAPE_1x1 {
        (*data).width = 1;
        (*data).height = 1;
    } else if gDecorations[decor].shape == DECORSHAPE_2x1 {
        (*data).width = 2;
        (*data).height = 1;
    } else if gDecorations[decor].shape == DECORSHAPE_3x1 {
        (*data).width = 3;
        (*data).height = 1;
    } else if gDecorations[decor].shape == DECORSHAPE_4x2 {
        (*data).width = 4;
        (*data).height = 2;
    } else if gDecorations[decor].shape == DECORSHAPE_2x2 {
        (*data).width = 2;
        (*data).height = 2;
    } else if gDecorations[decor].shape == DECORSHAPE_1x2 {
        (*data).width = 1;
        (*data).height = 2;
    } else if gDecorations[decor].shape == DECORSHAPE_1x3 {
        (*data).width = 1;
        (*data).height = 3;
    } else if gDecorations[decor].shape == DECORSHAPE_2x4 {
        (*data).width = 2;
        (*data).height = 4;
    } else if gDecorations[decor].shape == DECORSHAPE_3x3 {
        (*data).width = 3;
        (*data).height = 3;
    } else if gDecorations[decor].shape == DECORSHAPE_3x2 {
        (*data).width = 3;
        (*data).height = 2;
    }
}
pub(crate) unsafe extern "C" fn SetCameraSpritePosition(x: u8, y: u8) {
    gSprites[sDecor_CameraSpriteObjectIdx1].set_invisible(TRUE as u16);
    gSprites[sDecor_CameraSpriteObjectIdx1].callback = Some(SpriteCallbackDummy);
    gSprites[sDecor_CameraSpriteObjectIdx2].x = x as i16 * 16 + 136;
    gSprites[sDecor_CameraSpriteObjectIdx2].y = y as i16 * 16 + 72;
}
pub(crate) unsafe extern "C" fn DecorationIsUnderCursor(
    taskId: u8,
    idx: u8,
    data: *mut DecorRearrangementDataBuffer,
) -> u8 {
    let mut x: u8 = 0;
    let mut y: u8 = 0;
    let mut xOff: u8 = 0;
    let mut yOff: u8 = 0;
    let mut ht: u8 = 0;
    x = gTasks[taskId].data[0] as u8 - MAP_OFFSET as u8;
    y = gTasks[taskId].data[1] as u8 - MAP_OFFSET as u8;
    xOff = *sDecorationContext.pos.at(idx) >> 4;
    yOff = *sDecorationContext.pos.at(idx) & 0x0F;
    ht = (*data).height;
    if *sDecorationContext.items.at(idx) == DECOR_SAND_ORNAMENT
        && MapGridGetMetatileIdAt(xOff as i32 + MAP_OFFSET, yOff as i32 + MAP_OFFSET)
            == METATILE_SecretBase_SandOrnament_BrokenBase
    {
        ht -= 1;
    }
    if x >= xOff
        && (x as i32) < xOff as i32 + (*data).width as i32
        && y as i32 > yOff as i32 - ht as i32
        && y <= yOff
    {
        SetCameraSpritePosition((*data).width - (x - xOff + 1), yOff - y);
        return TRUE;
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn SetDecorRearrangementFlagIdIfFlagUnset() {
    let mut xOff: u8 = 0;
    let mut yOff: u8 = 0;
    let mut i: u16 = 0;
    xOff = *sDecorationContext
        .pos
        .at(sDecorRearrangementDataBuffer[sCurDecorSelectedInRearrangement].idx)
        >> 4;
    yOff = *sDecorationContext
        .pos
        .at(sDecorRearrangementDataBuffer[sCurDecorSelectedInRearrangement].idx)
        & 0x0F;
    i = 0;
    while i < OBJECT_EVENT_TEMPLATES_COUNT as u16 {
        if (*gSaveBlock1Ptr).objectEventTemplates[i].x == xOff as i16
            && (*gSaveBlock1Ptr).objectEventTemplates[i].y == yOff as i16
            && FlagGet((*gSaveBlock1Ptr).objectEventTemplates[i].flagId) == 0
        {
            sDecorRearrangementDataBuffer[sCurDecorSelectedInRearrangement].flagId =
                (*gSaveBlock1Ptr).objectEventTemplates[i].flagId;
            break;
        }
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn AttemptMarkSpriteDecorUnderCursorForRemoval(taskId: u8) -> u8 {
    let mut i: u16 = 0;
    i = 0;
    while i < sDecorationContext.size as u16 {
        if *sDecorationContext.items.at(i) != DECOR_NONE {
            if gDecorations[*sDecorationContext.items.at(i)].permission == DECORPERM_SPRITE {
                SetDecorRearrangementShape(
                    *sDecorationContext.items.at(i),
                    sDecorRearrangementDataBuffer.as_mut_ptr(),
                );
                if DecorationIsUnderCursor(
                    taskId,
                    i as u8,
                    sDecorRearrangementDataBuffer.as_mut_ptr(),
                ) == TRUE
                {
                    (*sDecorRearrangementDataBuffer.as_mut_ptr()).idx = i as u8;
                    SetDecorRearrangementFlagIdIfFlagUnset();
                    sCurDecorSelectedInRearrangement = 1;
                    return TRUE;
                }
            }
        }
        i += 1;
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn MarkSpriteDecorsInBoundsForRemoval(
    left: u8,
    top: u8,
    right: u8,
    bottom: u8,
) {
    let mut i: u8 = 0;
    let mut xOff: u8 = 0;
    let mut yOff: u8 = 0;
    let mut decor: u8 = 0;
    i = 0;
    while i < sDecorationContext.size {
        decor = *sDecorationContext.items.at(i);
        xOff = *sDecorationContext.pos.at(i) >> 4;
        yOff = *sDecorationContext.pos.at(i) & 0x0F;
        if decor != DECOR_NONE
            && gDecorations[decor].permission == DECORPERM_SPRITE
            && left <= xOff
            && top <= yOff
            && right >= xOff
            && bottom >= yOff
        {
            sDecorRearrangementDataBuffer[sCurDecorSelectedInRearrangement].idx = i;
            SetDecorRearrangementFlagIdIfFlagUnset();
            sCurDecorSelectedInRearrangement += 1;
        }
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn AttemptMarkDecorUnderCursorForRemoval(taskId: u8) {
    let mut i: u8 = 0;
    let mut xOff: u8 = 0;
    let mut yOff: u8 = 0;
    let mut var1: u8 = 0;
    let mut var2: u32 = 0;
    sCurDecorSelectedInRearrangement = 0;
    if AttemptMarkSpriteDecorUnderCursorForRemoval(taskId) != TRUE {
        i = 0;
        while i < sDecorationContext.size {
            var1 = *sDecorationContext.items.at(i);
            if var1 != DECOR_NONE {
                SetDecorRearrangementShape(var1, &raw mut sDecorRearrangementDataBuffer[0]);
                if DecorationIsUnderCursor(taskId, i, &raw mut sDecorRearrangementDataBuffer[0])
                    == TRUE
                {
                    sDecorRearrangementDataBuffer[0].idx = i;
                    sCurDecorSelectedInRearrangement += 1;
                    break;
                }
            }
            i += 1;
        }
        if sCurDecorSelectedInRearrangement != 0 {
            xOff = *sDecorationContext
                .pos
                .at(sDecorRearrangementDataBuffer[0].idx)
                >> 4;
            yOff = *sDecorationContext
                .pos
                .at(sDecorRearrangementDataBuffer[0].idx)
                & 0x0F;
            var1 = yOff - sDecorRearrangementDataBuffer[0].height + 1;
            var2 = sDecorRearrangementDataBuffer[0].width as u32 + xOff as u32 - 1;
            MarkSpriteDecorsInBoundsForRemoval(xOff, var1, var2 as u8, yOff);
        }
    }
}
pub(crate) unsafe extern "C" fn ReturnDecorationPrompt(taskId: u8) {
    DisplayYesNoMenuDefaultYes();
    DoYesNoFuncWithChoice(
        taskId,
        (&raw const *sReturnDecorationYesNoFunctions).cast_mut(),
    );
}
pub(crate) unsafe extern "C" fn PutAwayDecoration(taskId: u8) {
    FadeScreen(FADE_TO_BLACK, 0);
    gTasks[taskId].data[2] = 0;
    gTasks[taskId].func = Some(Task_PutAwayDecoration);
}
pub(crate) unsafe extern "C" fn StopPuttingAwayDecorationsPrompt(taskId: u8) {
    DisplayYesNoMenuDefaultYes();
    DoYesNoFuncWithChoice(
        taskId,
        (&raw const *sStopPuttingAwayDecorationsYesNoFunctions).cast_mut(),
    );
}
pub(crate) unsafe extern "C" fn StopPuttingAwayDecorations(taskId: u8) {
    ClearDialogWindowAndFrame(0, 0);
    StopPuttingAwayDecorations_(taskId);
}
pub(crate) unsafe extern "C" fn StopPuttingAwayDecorations_(taskId: u8) {
    FadeScreen(FADE_TO_BLACK, 0);
    gTasks[taskId].data[2] = 0;
    gTasks[taskId].func = Some(Task_StopPuttingAwayDecorations);
}
pub(crate) unsafe extern "C" fn Task_StopPuttingAwayDecorations(taskId: u8) {
    match gTasks[taskId].data[2] {
        0 => {
            if gPaletteFade.active() == 0 {
                WarpToInitialPosition(taskId);
                gTasks[taskId].data[2] = 1;
            }
        }
        1 => {
            FreePlayerSpritePalette();
            gFieldCallback = Some(FieldCB_StopPuttingAwayDecorations);
            SetMainCallback2(Some(CB2_ReturnToField));
            DestroyTask(taskId);
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn Task_ReinitializeDecorationMenuHandler(taskId: u8) {
    let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
    match *data.at(2) {
        0 => {
            HideSecretBaseDecorationSprites();
            *data.at(2) += 1;
        }
        1 => {
            ScriptContext_SetupScript(SecretBase_EventScript_InitDecorations.as_ptr().cast_mut());
            *data.at(2) += 1;
        }
        2 => {
            LockPlayerFieldControls();
            *data.at(2) += 1;
        }
        3 => {
            if IsWeatherNotFadingIn() == TRUE {
                gTasks[taskId].func = Some(HandleDecorationActionsMenuInput);
            }
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn FieldCB_StopPuttingAwayDecorations() {
    let mut taskId: u8 = 0;
    FadeInFromBlack();
    DrawDialogueFrame(0, TRUE);
    InitDecorationActionsWindow();
    taskId = CreateTask(Some(Task_ReinitializeDecorationMenuHandler), 8);
    gTasks[taskId].data[2] = 0;
}
pub(crate) unsafe extern "C" fn InitializeCameraSprite1(sprite: *mut Sprite) {
    (*sprite).data[0] += 1;
    (*sprite).data[0] &= 0x1F;
    if (*sprite).data[0] > 15 {
        (*sprite).set_invisible(TRUE as u16);
    } else {
        (*sprite).set_invisible(FALSE as u16);
    }
}
pub(crate) unsafe extern "C" fn LoadPlayerSpritePalette() {
    if (*gSaveBlock2Ptr).playerGender == MALE {
        LoadSpritePalette((&raw const *sSpritePal_PuttingAwayCursorBrendan).cast_mut());
    } else {
        LoadSpritePalette((&raw const *sSpritePal_PuttingAwayCursorMay).cast_mut());
    }
}
pub(crate) unsafe extern "C" fn FreePlayerSpritePalette() {
    FreeSpritePaletteByTag(PLACE_DECORATION_PLAYER_TAG);
}
pub(crate) unsafe extern "C" fn DecorationItemsMenuAction_AttemptToss(taskId: u8) {
    if IsSelectedDecorInThePC() == TRUE {
        StringCopy(
            gStringVar1.as_mut_ptr(),
            gDecorations[*gCurDecorationItems.at(gCurDecorationIndex)]
                .name
                .as_ptr()
                .cast_mut(),
        );
        StringExpandPlaceholders(
            gStringVar4.as_mut_ptr(),
            gText_DecorationWillBeDiscarded.as_ptr().cast_mut(),
        );
        DisplayItemMessageOnField(taskId, gStringVar4.as_mut_ptr(), Some(TossDecorationPrompt));
    } else {
        StringExpandPlaceholders(
            gStringVar4.as_mut_ptr(),
            gText_CantThrowAwayInUse.as_ptr().cast_mut(),
        );
        DisplayItemMessageOnField(
            taskId,
            gStringVar4.as_mut_ptr(),
            Some(ReturnToDecorationItemsAfterInvalidSelection),
        );
    }
}
pub(crate) unsafe extern "C" fn TossDecorationPrompt(taskId: u8) {
    DisplayYesNoMenuDefaultYes();
    DoYesNoFuncWithChoice(
        taskId,
        (&raw const *sTossDecorationYesNoFunctions).cast_mut(),
    );
}
pub(crate) unsafe extern "C" fn TossDecoration(taskId: u8) {
    *gCurDecorationItems.at(gCurDecorationIndex) = DECOR_NONE;
    sNumOwnedDecorationsInCurCategory = GetNumOwnedDecorationsInCategory(sCurDecorationCategory);
    CondenseDecorationsInCategory(sCurDecorationCategory);
    IdentifyOwnedDecorationsCurrentlyInUseInternal(taskId);
    StringExpandPlaceholders(
        gStringVar4.as_mut_ptr(),
        gText_DecorationThrownAway.as_ptr().cast_mut(),
    );
    DisplayItemMessageOnField(
        taskId,
        gStringVar4.as_mut_ptr(),
        Some(ReturnToDecorationItemsAfterInvalidSelection),
    );
}
