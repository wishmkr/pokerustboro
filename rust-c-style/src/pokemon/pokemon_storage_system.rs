//! Translated from `src/pokemon_storage_system.c` by tools/rustport/c2rs.py.
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
// Data tables (translate with cdata.py): sMainMenuTexts sWindowTemplate_MainMenu sAnim_ChooseBoxMenu_TopLeft sAnim_ChooseBoxMenu_BottomLeft sAnim_ChooseBoxMenu_TopRight sAnim_ChooseBoxMenu_BottomRight sAnims_ChooseBoxMenu sAffineAnim_ChooseBoxMenu sAffineAnims_ChooseBoxMenu sChooseBoxMenu_TextColors sText_OutOf30 sChooseBoxMenu_Pal sChooseBoxMenuCenter_Gfx sChooseBoxMenuSides_Gfx sScrollingBg_Gfx sScrollingBg_Tilemap sDisplayMenu_Pal sDisplayMenu_Tilemap sPkmnData_Tilemap sInterface_Pal sPkmnDataGray_Pal sScrollingBg_Pal sScrollingBgMoveItems_Pal sCloseBoxButton_Tilemap sPartySlotFilled_Tilemap sPartySlotEmpty_Tilemap sWaveform_Pal sWaveform_Gfx sUnused_Pal sTextWindows_Pal sWindowTemplates sBgTemplates sWaveformSpritePalette sSpriteSheet_Waveform sOamData_DisplayMon sSpriteTemplate_DisplayMon sMessages sYesNoWindowTemplate sOamData_DisplayMon sOamData_Waveform sAnim_Waveform_LeftOff sAnim_Waveform_LeftOn sAnim_Waveform_RightOff sAnim_Waveform_RightOn sAnims_Waveform sSpriteTemplate_Waveform sOamData_MonIcon sSpriteTemplate_MonIcon sOamData_MonIcon sAffineAnim_ReleaseMon_Release sAffineAnim_ReleaseMon_CameBack sAffineAnims_ReleaseMon sWallpaperPalettes_Forest sWallpaperTiles_Forest sWallpaperTilemap_Forest sWallpaperPalettes_City sWallpaperTiles_City sWallpaperTilemap_City sWallpaperPalettes_Desert sWallpaperTiles_Desert sWallpaperTilemap_Desert sWallpaperPalettes_Savanna sWallpaperTiles_Savanna sWallpaperTilemap_Savanna sWallpaperPalettes_Crag sWallpaperTiles_Crag sWallpaperTilemap_Crag sWallpaperPalettes_Volcano sWallpaperTiles_Volcano sWallpaperTilemap_Volcano sWallpaperPalettes_Snow sWallpaperTiles_Snow sWallpaperTilemap_Snow sWallpaperPalettes_Cave sWallpaperTiles_Cave sWallpaperTilemap_Cave sWallpaperPalettes_Beach sWallpaperTiles_Beach sWallpaperTilemap_Beach sWallpaperPalettes_Seafloor sWallpaperTiles_Seafloor sWallpaperTilemap_Seafloor sWallpaperPalettes_River sWallpaperTiles_River sWallpaperTilemap_River sWallpaperPalettes_Sky sWallpaperTiles_Sky sWallpaperTilemap_Sky sWallpaperPalettes_PolkaDot sWallpaperTiles_PolkaDot sWallpaperTilemap_PolkaDot sWallpaperPalettes_Pokecenter sWallpaperTiles_Pokecenter sWallpaperTilemap_Pokecenter sWallpaperPalettes_Machine sWallpaperTiles_Machine sWallpaperTilemap_Machine sWallpaperPalettes_Plain sWallpaperTiles_Plain sWallpaperTilemap_Plain sWallpaperTilemap_Unused sBoxTitleColors sWallpapers sArrow_Gfx sWallpaperPalettes_Zigzagoon sWallpaperTiles_Zigzagoon sWallpaperTilemap_Zigzagoon sWallpaperPalettes_Screen sWallpaperTiles_Screen sWallpaperTilemap_Screen sWallpaperPalettes_Diagonal sWallpaperTiles_Diagonal sWallpaperTilemap_Diagonal sWallpaperPalettes_Block sWallpaperTiles_Block sWallpaperTilemap_Block sWallpaperPalettes_Pokecenter2 sWallpaperTiles_Pokecenter2 sWallpaperTilemap_Pokecenter2 sWallpaperPalettes_Frame sWallpaperTiles_Frame sWallpaperTilemap_Frame sWallpaperPalettes_Blank sWallpaperTiles_Blank sWallpaperTilemap_Blank sWallpaperPalettes_Circles sWallpaperTiles_Circles sWallpaperTilemap_Circles sWallpaperPalettes_Azumarill sWallpaperTiles_Azumarill sWallpaperTilemap_Azumarill sWallpaperPalettes_Pikachu sWallpaperTiles_Pikachu sWallpaperTilemap_Pikachu sWallpaperPalettes_Legendary sWallpaperTiles_Legendary sWallpaperTilemap_Legendary sWallpaperPalettes_Dusclops sWallpaperTiles_Dusclops sWallpaperTilemap_Dusclops sWallpaperPalettes_Ludicolo sWallpaperTiles_Ludicolo sWallpaperTilemap_Ludicolo sWallpaperPalettes_Whiscash sWallpaperTiles_Whiscash sWallpaperTilemap_Whiscash sWallpaperIcon_Aqua sWallpaperIcon_Heart sWallpaperIcon_FiveStar sWallpaperIcon_Brick sWallpaperIcon_FourStar sWallpaperIcon_Asterisk sWallpaperIcon_Dot sWallpaperIcon_LineCircle sWallpaperIcon_PokeBall sWallpaperIcon_Maze sWallpaperIcon_Footprint sWallpaperIcon_BigAsterisk sWallpaperIcon_Circle sWallpaperIcon_Koffing sWallpaperIcon_Ribbon sWallpaperIcon_FourCircles sWallpaperIcon_Lotad sWallpaperIcon_Crystal sWallpaperIcon_Pichu sWallpaperIcon_Diglett sWallpaperIcon_Luvdisc sWallpaperIcon_StarInCircle sWallpaperIcon_Spinda sWallpaperIcon_Latis sWallpaperIcon_Minun sWallpaperIcon_Togepi sWallpaperIcon_Magma sWaldaWallpapers sWaldaWallpaperIcons sUnusedColor sSpriteSheet_Arrow sOamData_BoxTitle sAnim_BoxTitle_Left sAnim_BoxTitle_Right sAnims_BoxTitle sSpriteTemplate_BoxTitle sOamData_Arrow sAnim_Arrow_Left sAnim_Arrow_Right sAnims_Arrow sSpriteTemplate_Arrow sHandCursor_Pal sHandCursor_Gfx sHandCursorShadow_Gfx sRestrictedReleaseMoves sMenuTexts sWindowTemplate_MultiMove sItemInfoFrame_Gfx sOamData_ItemIcon sAffineAnim_ItemIcon_Small sAffineAnim_ItemIcon_Appear sAffineAnim_ItemIcon_Disappear sAffineAnim_ItemIcon_PickUp sAffineAnim_ItemIcon_PutDown sAffineAnim_ItemIcon_PutAway sAffineAnim_ItemIcon_Large sAffineAnims_ItemIcon sSpriteTemplate_ItemIcon sTilemapDimensions inputFuncs.9 placeChangeFuncs.10 sAnim_Cursor_Bouncing.2 sAnim_Cursor_Fist.5 sAnim_Cursor_Open.4 sAnim_Cursor_Still.3 sAnims_Cursor.6 sOamData_Cursor.0 sOamData_CursorShadow.1 sSpriteTemplate_Cursor.8 sSpriteTemplate_CursorShadow.7

/// `struct ChooseBoxMenu`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct ChooseBoxMenu {
    pub menuSprite: *mut Sprite,
    pub menuSideSprites: CArray<*mut Sprite, 4>,
    pub unused1: CArray<u32, 3>,
    pub arrowSprites: CArray<*mut Sprite, 2>,
    pub unused2: CArray<u8, 532>,
    pub loadedPalette: u32,
    pub tileTag: u16,
    pub paletteTag: u16,
    pub curBox: u8,
    pub unused3: u8,
    pub subpriority: u8,
}

unsafe impl Sync for ChooseBoxMenu {}

/// `struct PokemonStorageSystemData`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PokemonStorageSystemData {
    pub state: u8,
    pub boxOption: u8,
    pub screenChangeType: u8,
    pub isReopening: u8,
    pub taskId: u8,
    pub unkUtil: UnkUtil,
    pub unkUtilData: CArray<UnkUtilData, 8>,
    pub partyMenuTilemapBuffer: CArray<u16, 264>,
    pub partyMenuUnused1: u16,
    pub partyMenuY: u16,
    pub partyMenuUnused2: u8,
    pub partyMenuMoveTimer: u8,
    pub showPartyMenuState: u8,
    pub closeBoxFlashing: u8,
    pub closeBoxFlashTimer: u8,
    pub closeBoxFlashState: u8,
    pub newCurrBoxId: i16,
    pub bg2_X: u16,
    pub scrollSpeed: i16,
    pub scrollTimer: u16,
    pub wallpaperOffset: u8,
    pub scrollUnused1: u8,
    pub scrollToBoxIdUnused: u8,
    pub scrollUnused2: u16,
    pub scrollDirectionUnused: i16,
    pub scrollUnused3: u16,
    pub scrollUnused4: u16,
    pub scrollUnused5: u16,
    pub scrollUnused6: u16,
    pub filler1: CArray<u8, 22>,
    pub boxTitleTiles: CArray<u8, 1024>,
    pub boxTitleCycleId: u8,
    pub wallpaperLoadState: u8,
    pub wallpaperLoadBoxId: u8,
    pub wallpaperLoadDir: i8,
    pub boxTitlePal: CArray<u16, 16>,
    pub boxTitlePalOffset: u16,
    pub boxTitleAltPalOffset: u16,
    pub curBoxTitleSprites: CArray<*mut Sprite, 2>,
    pub nextBoxTitleSprites: CArray<*mut Sprite, 2>,
    pub arrowSprites: CArray<*mut Sprite, 2>,
    pub wallpaperPalBits: u32,
    pub filler2: CArray<u8, 80>,
    pub unkUnused1: u16,
    pub wallpaperSetId: i16,
    pub wallpaperId: i16,
    pub wallpaperTilemap: CArray<u16, 360>,
    pub wallpaperChangeState: u8,
    pub scrollState: u8,
    pub scrollToBoxId: u8,
    pub scrollDirection: i8,
    pub wallpaperTiles: *mut u8,
    pub movingMonSprite: *mut Sprite,
    pub partySprites: CArray<*mut Sprite, 6>,
    pub boxMonsSprites: CArray<*mut Sprite, 30>,
    pub shiftMonSpritePtr: *mut *mut Sprite,
    pub releaseMonSpritePtr: *mut *mut Sprite,
    pub numIconsPerSpecies: CArray<u16, 40>,
    pub iconSpeciesList: CArray<u16, 40>,
    pub boxSpecies: CArray<u16, 30>,
    pub boxPersonalities: CArray<u32, 30>,
    pub incomingBoxId: u8,
    pub shiftTimer: u8,
    pub numPartyToCompact: u8,
    pub iconScrollDistance: u16,
    pub iconScrollPos: i16,
    pub iconScrollSpeed: i16,
    pub iconScrollNumIncoming: u16,
    pub iconScrollCurColumn: u8,
    pub iconScrollDirection: i8,
    pub iconScrollState: u8,
    pub iconScrollToBoxId: u8,
    pub menuWindow: WindowTemplate,
    pub menuItems: CArray<StorageMenu, 7>,
    pub menuItemsCount: u8,
    pub menuWidth: u8,
    pub menuUnusedField: u8,
    pub menuWindowId: u16,
    pub cursorSprite: *mut Sprite,
    pub cursorShadowSprite: *mut Sprite,
    pub cursorNewX: i32,
    pub cursorNewY: i32,
    pub cursorSpeedX: u32,
    pub cursorSpeedY: u32,
    pub cursorTargetX: i16,
    pub cursorTargetY: i16,
    pub cursorMoveSteps: u16,
    pub cursorVerticalWrap: i8,
    pub cursorHorizontalWrap: i8,
    pub newCursorArea: u8,
    pub newCursorPosition: u8,
    pub cursorPrevHorizPos: u8,
    pub cursorFlipTimer: u8,
    pub cursorPalNums: CArray<u8, 2>,
    pub displayMonPalette: *mut u32,
    pub displayMonPersonality: u32,
    pub displayMonSpecies: u16,
    pub displayMonItemId: u16,
    pub displayUnusedVar: u16,
    pub setMosaic: u8,
    pub displayMonMarkings: u8,
    pub displayMonLevel: u8,
    pub displayMonIsEgg: u8,
    pub displayMonName: CArray<u8, 11>,
    pub displayMonNameText: CArray<u8, 36>,
    pub displayMonSpeciesName: CArray<u8, 36>,
    pub displayMonGenderLvlText: CArray<u8, 36>,
    pub displayMonItemName: CArray<u8, 36>,
    pub monPlaceChangeFunc: Option<unsafe extern "C" fn() -> u8>,
    pub monPlaceChangeState: u8,
    pub shiftBoxId: u8,
    pub markingComboSprite: *mut Sprite,
    pub waveformSprites: CArray<*mut Sprite, 2>,
    pub markingComboTilesPtr: *mut u16,
    pub markMenu: MonMarkingsMenu,
    pub chooseBoxMenu: ChooseBoxMenu,
    pub movingMon: Pokemon,
    pub tempMon: Pokemon,
    pub canReleaseMon: i8,
    pub releaseStatusResolved: u8,
    pub releaseCheckBoxId: i8,
    pub releaseCheckBoxPos: i8,
    pub releaseBoxId: i8,
    pub releaseBoxPos: i8,
    pub releaseCheckState: u16,
    pub restrictedReleaseMonMoves: u16,
    pub restrictedMoveList: CArray<u16, 8>,
    pub summaryMaxPos: u8,
    pub summaryStartPos: u8,
    pub summaryScreenMode: u8,
    pub summaryMon: PokemonStorageSystemData_summaryMon,
    pub messageText: CArray<u8, 40>,
    pub boxTitleText: CArray<u8, 40>,
    pub releaseMonName: CArray<u8, 11>,
    pub itemName: CArray<u8, 20>,
    pub inBoxMovingMode: u8,
    pub multiMoveWindowId: u16,
    pub itemIcons: CArray<ItemIcon, 3>,
    pub movingItemId: u16,
    pub itemInfoWindowOffset: u16,
    pub unkUnused2: u8,
    pub displayMonPalOffset: u16,
    pub displayMonTilePtr: *mut u16,
    pub displayMonSprite: *mut Sprite,
    pub displayMonPalBuffer: CArray<u16, 64>,
    pub tileBuffer: CArray<u8, 8192>,
    pub itemIconBuffer: CArray<u8, 2048>,
    pub wallpaperBgTilemapBuffer: CArray<u8, 4096>,
    pub displayMenuTilemapBuffer: CArray<u8, 2048>,
}

unsafe impl Sync for PokemonStorageSystemData {}

/// `__typeof__(*((__typeof__(sMultiMove))0))`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct typeof___sMultiMove_0_t {
    pub funcId: u8,
    pub state: u8,
    pub fromColumn: u8,
    pub fromRow: u8,
    pub toColumn: u8,
    pub toRow: u8,
    pub cursorColumn: u8,
    pub cursorRow: u8,
    pub minColumn: u8,
    pub minRow: u8,
    pub columnsTotal: u8,
    pub rowsTotal: u8,
    pub bgX: u16,
    pub bgY: u16,
    pub bgMoveSteps: u16,
    pub boxMons: CArray<BoxPokemon, 30>,
}

unsafe impl Sync for typeof___sMultiMove_0_t {}

/// `struct TilemapUtil`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct TilemapUtil {
    pub prev: TilemapUtil_RectData,
    pub cur: TilemapUtil_RectData,
    pub savedTilemap: *mut core::ffi::c_void,
    pub tilemap: *mut core::ffi::c_void,
    pub altWidth: u16,
    pub altHeight: u16,
    pub width: u16,
    pub height: u16,
    pub rowSize: u16,
    pub tileSize: u8,
    pub bg: u8,
    pub active: u8,
}

unsafe impl Sync for TilemapUtil {}

/// `struct UnkUtil`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct UnkUtil {
    pub data: *mut UnkUtilData,
    pub numActive: u8,
    pub max: u8,
}

unsafe impl Sync for UnkUtil {}

/// `__typeof__(sMainMenuTexts[0])`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct sMainMenuTexts_0_t {
    pub text: *mut u8,
    pub desc: *mut u8,
}

unsafe impl Sync for sMainMenuTexts_0_t {}

/// `struct UnkUtilData`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct UnkUtilData {
    pub src: *mut u8,
    pub dest: *mut u8,
    pub size: u16,
    pub unk: u16,
    pub height: u16,
    pub func: Option<unsafe extern "C" fn(*mut UnkUtilData)>,
}

unsafe impl Sync for UnkUtilData {}

/// `struct Wallpaper`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Wallpaper {
    pub tiles: *mut u32,
    pub tilemap: *mut u32,
    pub palettes: *mut u16,
}

unsafe impl Sync for Wallpaper {}

/// `struct StorageMenu`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct StorageMenu {
    pub text: *mut u8,
    pub textId: i32,
}

unsafe impl Sync for StorageMenu {}

/// `struct TilemapUtil_RectData`
#[repr(C, align(4))]
#[derive(Clone, Copy)]
pub struct TilemapUtil_RectData {
    pub x: i16,
    pub y: i16,
    pub width: u16,
    pub height: u16,
    pub destX: i16,
    pub destY: i16,
}

unsafe impl Sync for TilemapUtil_RectData {}

/// `__anon1`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Anon1 {
    pub func: Option<unsafe extern "C" fn() -> u8>,
    pub area: i8,
}

unsafe impl Sync for Anon1 {}

/// `struct StorageMessage`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct StorageMessage {
    pub text: *mut u8,
    pub format: u8,
}

unsafe impl Sync for StorageMessage {}

/// `__typeof__(sRestrictedReleaseMoves[0])`
#[repr(C, align(4))]
#[derive(Clone, Copy)]
pub struct sRestrictedReleaseMoves_0_t {
    pub mapGroup: i8,
    pub mapNum: i8,
    pub r#move: u16,
}

unsafe impl Sync for sRestrictedReleaseMoves_0_t {}

/// `__typeof__(sTilemapDimensions[0][0])`
#[repr(C, align(4))]
#[derive(Clone, Copy)]
pub struct sTilemapDimensions_0_0_t {
    pub width: u16,
    pub height: u16,
}

unsafe impl Sync for sTilemapDimensions_0_0_t {}

/// The anonymous type of `PokemonStorageSystemData::summaryMon`.
#[repr(C)]
#[derive(Clone, Copy)]
pub union PokemonStorageSystemData_summaryMon {
    pub mon: *mut Pokemon,
    pub r#box: *mut BoxPokemon,
}

unsafe impl Sync for PokemonStorageSystemData_summaryMon {}

/// `struct ItemIcon`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct ItemIcon {
    pub sprite: *mut Sprite,
    pub tiles: *mut u8,
    pub palIndex: u16,
    pub area: u8,
    pub pos: u8,
    pub active: u8,
}

unsafe impl Sync for ItemIcon {}

#[cfg(target_arch = "arm")]
const _: () = {
    #[allow(unused_imports)]
    use core::mem::{offset_of, size_of};
    assert!(size_of::<ChooseBoxMenu>() == 584);
    assert!(offset_of!(ChooseBoxMenu, menuSprite) == 0);
    assert!(offset_of!(ChooseBoxMenu, menuSideSprites) == 4);
    assert!(offset_of!(ChooseBoxMenu, unused1) == 20);
    assert!(offset_of!(ChooseBoxMenu, arrowSprites) == 32);
    assert!(offset_of!(ChooseBoxMenu, unused2) == 40);
    assert!(offset_of!(ChooseBoxMenu, loadedPalette) == 572);
    assert!(offset_of!(ChooseBoxMenu, tileTag) == 576);
    assert!(offset_of!(ChooseBoxMenu, paletteTag) == 578);
    assert!(offset_of!(ChooseBoxMenu, curBox) == 580);
    assert!(offset_of!(ChooseBoxMenu, unused3) == 581);
    assert!(offset_of!(ChooseBoxMenu, subpriority) == 582);
    assert!(size_of::<PokemonStorageSystemData>() == 25284);
    assert!(offset_of!(PokemonStorageSystemData, state) == 0);
    assert!(offset_of!(PokemonStorageSystemData, boxOption) == 1);
    assert!(offset_of!(PokemonStorageSystemData, screenChangeType) == 2);
    assert!(offset_of!(PokemonStorageSystemData, isReopening) == 3);
    assert!(offset_of!(PokemonStorageSystemData, taskId) == 4);
    assert!(offset_of!(PokemonStorageSystemData, unkUtil) == 8);
    assert!(offset_of!(PokemonStorageSystemData, unkUtilData) == 16);
    assert!(offset_of!(PokemonStorageSystemData, partyMenuTilemapBuffer) == 176);
    assert!(offset_of!(PokemonStorageSystemData, partyMenuUnused1) == 704);
    assert!(offset_of!(PokemonStorageSystemData, partyMenuY) == 706);
    assert!(offset_of!(PokemonStorageSystemData, partyMenuUnused2) == 708);
    assert!(offset_of!(PokemonStorageSystemData, partyMenuMoveTimer) == 709);
    assert!(offset_of!(PokemonStorageSystemData, showPartyMenuState) == 710);
    assert!(offset_of!(PokemonStorageSystemData, closeBoxFlashing) == 711);
    assert!(offset_of!(PokemonStorageSystemData, closeBoxFlashTimer) == 712);
    assert!(offset_of!(PokemonStorageSystemData, closeBoxFlashState) == 713);
    assert!(offset_of!(PokemonStorageSystemData, newCurrBoxId) == 714);
    assert!(offset_of!(PokemonStorageSystemData, bg2_X) == 716);
    assert!(offset_of!(PokemonStorageSystemData, scrollSpeed) == 718);
    assert!(offset_of!(PokemonStorageSystemData, scrollTimer) == 720);
    assert!(offset_of!(PokemonStorageSystemData, wallpaperOffset) == 722);
    assert!(offset_of!(PokemonStorageSystemData, scrollUnused1) == 723);
    assert!(offset_of!(PokemonStorageSystemData, scrollToBoxIdUnused) == 724);
    assert!(offset_of!(PokemonStorageSystemData, scrollUnused2) == 726);
    assert!(offset_of!(PokemonStorageSystemData, scrollDirectionUnused) == 728);
    assert!(offset_of!(PokemonStorageSystemData, scrollUnused3) == 730);
    assert!(offset_of!(PokemonStorageSystemData, scrollUnused4) == 732);
    assert!(offset_of!(PokemonStorageSystemData, scrollUnused5) == 734);
    assert!(offset_of!(PokemonStorageSystemData, scrollUnused6) == 736);
    assert!(offset_of!(PokemonStorageSystemData, filler1) == 738);
    assert!(offset_of!(PokemonStorageSystemData, boxTitleTiles) == 760);
    assert!(offset_of!(PokemonStorageSystemData, boxTitleCycleId) == 1784);
    assert!(offset_of!(PokemonStorageSystemData, wallpaperLoadState) == 1785);
    assert!(offset_of!(PokemonStorageSystemData, wallpaperLoadBoxId) == 1786);
    assert!(offset_of!(PokemonStorageSystemData, wallpaperLoadDir) == 1787);
    assert!(offset_of!(PokemonStorageSystemData, boxTitlePal) == 1788);
    assert!(offset_of!(PokemonStorageSystemData, boxTitlePalOffset) == 1820);
    assert!(offset_of!(PokemonStorageSystemData, boxTitleAltPalOffset) == 1822);
    assert!(offset_of!(PokemonStorageSystemData, curBoxTitleSprites) == 1824);
    assert!(offset_of!(PokemonStorageSystemData, nextBoxTitleSprites) == 1832);
    assert!(offset_of!(PokemonStorageSystemData, arrowSprites) == 1840);
    assert!(offset_of!(PokemonStorageSystemData, wallpaperPalBits) == 1848);
    assert!(offset_of!(PokemonStorageSystemData, filler2) == 1852);
    assert!(offset_of!(PokemonStorageSystemData, unkUnused1) == 1932);
    assert!(offset_of!(PokemonStorageSystemData, wallpaperSetId) == 1934);
    assert!(offset_of!(PokemonStorageSystemData, wallpaperId) == 1936);
    assert!(offset_of!(PokemonStorageSystemData, wallpaperTilemap) == 1938);
    assert!(offset_of!(PokemonStorageSystemData, wallpaperChangeState) == 2658);
    assert!(offset_of!(PokemonStorageSystemData, scrollState) == 2659);
    assert!(offset_of!(PokemonStorageSystemData, scrollToBoxId) == 2660);
    assert!(offset_of!(PokemonStorageSystemData, scrollDirection) == 2661);
    assert!(offset_of!(PokemonStorageSystemData, wallpaperTiles) == 2664);
    assert!(offset_of!(PokemonStorageSystemData, movingMonSprite) == 2668);
    assert!(offset_of!(PokemonStorageSystemData, partySprites) == 2672);
    assert!(offset_of!(PokemonStorageSystemData, boxMonsSprites) == 2696);
    assert!(offset_of!(PokemonStorageSystemData, shiftMonSpritePtr) == 2816);
    assert!(offset_of!(PokemonStorageSystemData, releaseMonSpritePtr) == 2820);
    assert!(offset_of!(PokemonStorageSystemData, numIconsPerSpecies) == 2824);
    assert!(offset_of!(PokemonStorageSystemData, iconSpeciesList) == 2904);
    assert!(offset_of!(PokemonStorageSystemData, boxSpecies) == 2984);
    assert!(offset_of!(PokemonStorageSystemData, boxPersonalities) == 3044);
    assert!(offset_of!(PokemonStorageSystemData, incomingBoxId) == 3164);
    assert!(offset_of!(PokemonStorageSystemData, shiftTimer) == 3165);
    assert!(offset_of!(PokemonStorageSystemData, numPartyToCompact) == 3166);
    assert!(offset_of!(PokemonStorageSystemData, iconScrollDistance) == 3168);
    assert!(offset_of!(PokemonStorageSystemData, iconScrollPos) == 3170);
    assert!(offset_of!(PokemonStorageSystemData, iconScrollSpeed) == 3172);
    assert!(offset_of!(PokemonStorageSystemData, iconScrollNumIncoming) == 3174);
    assert!(offset_of!(PokemonStorageSystemData, iconScrollCurColumn) == 3176);
    assert!(offset_of!(PokemonStorageSystemData, iconScrollDirection) == 3177);
    assert!(offset_of!(PokemonStorageSystemData, iconScrollState) == 3178);
    assert!(offset_of!(PokemonStorageSystemData, iconScrollToBoxId) == 3179);
    assert!(offset_of!(PokemonStorageSystemData, menuWindow) == 3180);
    assert!(offset_of!(PokemonStorageSystemData, menuItems) == 3188);
    assert!(offset_of!(PokemonStorageSystemData, menuItemsCount) == 3244);
    assert!(offset_of!(PokemonStorageSystemData, menuWidth) == 3245);
    assert!(offset_of!(PokemonStorageSystemData, menuUnusedField) == 3246);
    assert!(offset_of!(PokemonStorageSystemData, menuWindowId) == 3248);
    assert!(offset_of!(PokemonStorageSystemData, cursorSprite) == 3252);
    assert!(offset_of!(PokemonStorageSystemData, cursorShadowSprite) == 3256);
    assert!(offset_of!(PokemonStorageSystemData, cursorNewX) == 3260);
    assert!(offset_of!(PokemonStorageSystemData, cursorNewY) == 3264);
    assert!(offset_of!(PokemonStorageSystemData, cursorSpeedX) == 3268);
    assert!(offset_of!(PokemonStorageSystemData, cursorSpeedY) == 3272);
    assert!(offset_of!(PokemonStorageSystemData, cursorTargetX) == 3276);
    assert!(offset_of!(PokemonStorageSystemData, cursorTargetY) == 3278);
    assert!(offset_of!(PokemonStorageSystemData, cursorMoveSteps) == 3280);
    assert!(offset_of!(PokemonStorageSystemData, cursorVerticalWrap) == 3282);
    assert!(offset_of!(PokemonStorageSystemData, cursorHorizontalWrap) == 3283);
    assert!(offset_of!(PokemonStorageSystemData, newCursorArea) == 3284);
    assert!(offset_of!(PokemonStorageSystemData, newCursorPosition) == 3285);
    assert!(offset_of!(PokemonStorageSystemData, cursorPrevHorizPos) == 3286);
    assert!(offset_of!(PokemonStorageSystemData, cursorFlipTimer) == 3287);
    assert!(offset_of!(PokemonStorageSystemData, cursorPalNums) == 3288);
    assert!(offset_of!(PokemonStorageSystemData, displayMonPalette) == 3292);
    assert!(offset_of!(PokemonStorageSystemData, displayMonPersonality) == 3296);
    assert!(offset_of!(PokemonStorageSystemData, displayMonSpecies) == 3300);
    assert!(offset_of!(PokemonStorageSystemData, displayMonItemId) == 3302);
    assert!(offset_of!(PokemonStorageSystemData, displayUnusedVar) == 3304);
    assert!(offset_of!(PokemonStorageSystemData, setMosaic) == 3306);
    assert!(offset_of!(PokemonStorageSystemData, displayMonMarkings) == 3307);
    assert!(offset_of!(PokemonStorageSystemData, displayMonLevel) == 3308);
    assert!(offset_of!(PokemonStorageSystemData, displayMonIsEgg) == 3309);
    assert!(offset_of!(PokemonStorageSystemData, displayMonName) == 3310);
    assert!(offset_of!(PokemonStorageSystemData, displayMonNameText) == 3321);
    assert!(offset_of!(PokemonStorageSystemData, displayMonSpeciesName) == 3357);
    assert!(offset_of!(PokemonStorageSystemData, displayMonGenderLvlText) == 3393);
    assert!(offset_of!(PokemonStorageSystemData, displayMonItemName) == 3429);
    assert!(offset_of!(PokemonStorageSystemData, monPlaceChangeFunc) == 3468);
    assert!(offset_of!(PokemonStorageSystemData, monPlaceChangeState) == 3472);
    assert!(offset_of!(PokemonStorageSystemData, shiftBoxId) == 3473);
    assert!(offset_of!(PokemonStorageSystemData, markingComboSprite) == 3476);
    assert!(offset_of!(PokemonStorageSystemData, waveformSprites) == 3480);
    assert!(offset_of!(PokemonStorageSystemData, markingComboTilesPtr) == 3488);
    assert!(offset_of!(PokemonStorageSystemData, markMenu) == 3492);
    assert!(offset_of!(PokemonStorageSystemData, chooseBoxMenu) == 7772);
    assert!(offset_of!(PokemonStorageSystemData, movingMon) == 8356);
    assert!(offset_of!(PokemonStorageSystemData, tempMon) == 8456);
    assert!(offset_of!(PokemonStorageSystemData, canReleaseMon) == 8556);
    assert!(offset_of!(PokemonStorageSystemData, releaseStatusResolved) == 8557);
    assert!(offset_of!(PokemonStorageSystemData, releaseCheckBoxId) == 8558);
    assert!(offset_of!(PokemonStorageSystemData, releaseCheckBoxPos) == 8559);
    assert!(offset_of!(PokemonStorageSystemData, releaseBoxId) == 8560);
    assert!(offset_of!(PokemonStorageSystemData, releaseBoxPos) == 8561);
    assert!(offset_of!(PokemonStorageSystemData, releaseCheckState) == 8562);
    assert!(offset_of!(PokemonStorageSystemData, restrictedReleaseMonMoves) == 8564);
    assert!(offset_of!(PokemonStorageSystemData, restrictedMoveList) == 8566);
    assert!(offset_of!(PokemonStorageSystemData, summaryMaxPos) == 8582);
    assert!(offset_of!(PokemonStorageSystemData, summaryStartPos) == 8583);
    assert!(offset_of!(PokemonStorageSystemData, summaryScreenMode) == 8584);
    assert!(offset_of!(PokemonStorageSystemData, summaryMon) == 8588);
    assert!(offset_of!(PokemonStorageSystemData, messageText) == 8592);
    assert!(offset_of!(PokemonStorageSystemData, boxTitleText) == 8632);
    assert!(offset_of!(PokemonStorageSystemData, releaseMonName) == 8672);
    assert!(offset_of!(PokemonStorageSystemData, itemName) == 8683);
    assert!(offset_of!(PokemonStorageSystemData, inBoxMovingMode) == 8703);
    assert!(offset_of!(PokemonStorageSystemData, multiMoveWindowId) == 8704);
    assert!(offset_of!(PokemonStorageSystemData, itemIcons) == 8708);
    assert!(offset_of!(PokemonStorageSystemData, movingItemId) == 8756);
    assert!(offset_of!(PokemonStorageSystemData, itemInfoWindowOffset) == 8758);
    assert!(offset_of!(PokemonStorageSystemData, unkUnused2) == 8760);
    assert!(offset_of!(PokemonStorageSystemData, displayMonPalOffset) == 8762);
    assert!(offset_of!(PokemonStorageSystemData, displayMonTilePtr) == 8764);
    assert!(offset_of!(PokemonStorageSystemData, displayMonSprite) == 8768);
    assert!(offset_of!(PokemonStorageSystemData, displayMonPalBuffer) == 8772);
    assert!(offset_of!(PokemonStorageSystemData, tileBuffer) == 8900);
    assert!(offset_of!(PokemonStorageSystemData, itemIconBuffer) == 17092);
    assert!(offset_of!(PokemonStorageSystemData, wallpaperBgTilemapBuffer) == 19140);
    assert!(offset_of!(PokemonStorageSystemData, displayMenuTilemapBuffer) == 23236);
    assert!(size_of::<typeof___sMultiMove_0_t>() == 2420);
    assert!(offset_of!(typeof___sMultiMove_0_t, funcId) == 0);
    assert!(offset_of!(typeof___sMultiMove_0_t, state) == 1);
    assert!(offset_of!(typeof___sMultiMove_0_t, fromColumn) == 2);
    assert!(offset_of!(typeof___sMultiMove_0_t, fromRow) == 3);
    assert!(offset_of!(typeof___sMultiMove_0_t, toColumn) == 4);
    assert!(offset_of!(typeof___sMultiMove_0_t, toRow) == 5);
    assert!(offset_of!(typeof___sMultiMove_0_t, cursorColumn) == 6);
    assert!(offset_of!(typeof___sMultiMove_0_t, cursorRow) == 7);
    assert!(offset_of!(typeof___sMultiMove_0_t, minColumn) == 8);
    assert!(offset_of!(typeof___sMultiMove_0_t, minRow) == 9);
    assert!(offset_of!(typeof___sMultiMove_0_t, columnsTotal) == 10);
    assert!(offset_of!(typeof___sMultiMove_0_t, rowsTotal) == 11);
    assert!(offset_of!(typeof___sMultiMove_0_t, bgX) == 12);
    assert!(offset_of!(typeof___sMultiMove_0_t, bgY) == 14);
    assert!(offset_of!(typeof___sMultiMove_0_t, bgMoveSteps) == 16);
    assert!(offset_of!(typeof___sMultiMove_0_t, boxMons) == 20);
    assert!(size_of::<TilemapUtil>() == 48);
    assert!(offset_of!(TilemapUtil, prev) == 0);
    assert!(offset_of!(TilemapUtil, cur) == 12);
    assert!(offset_of!(TilemapUtil, savedTilemap) == 24);
    assert!(offset_of!(TilemapUtil, tilemap) == 28);
    assert!(offset_of!(TilemapUtil, altWidth) == 32);
    assert!(offset_of!(TilemapUtil, altHeight) == 34);
    assert!(offset_of!(TilemapUtil, width) == 36);
    assert!(offset_of!(TilemapUtil, height) == 38);
    assert!(offset_of!(TilemapUtil, rowSize) == 40);
    assert!(offset_of!(TilemapUtil, tileSize) == 42);
    assert!(offset_of!(TilemapUtil, bg) == 43);
    assert!(offset_of!(TilemapUtil, active) == 44);
    assert!(size_of::<UnkUtil>() == 8);
    assert!(offset_of!(UnkUtil, data) == 0);
    assert!(offset_of!(UnkUtil, numActive) == 4);
    assert!(offset_of!(UnkUtil, max) == 5);
    assert!(size_of::<sMainMenuTexts_0_t>() == 8);
    assert!(offset_of!(sMainMenuTexts_0_t, text) == 0);
    assert!(offset_of!(sMainMenuTexts_0_t, desc) == 4);
    assert!(size_of::<UnkUtilData>() == 20);
    assert!(offset_of!(UnkUtilData, src) == 0);
    assert!(offset_of!(UnkUtilData, dest) == 4);
    assert!(offset_of!(UnkUtilData, size) == 8);
    assert!(offset_of!(UnkUtilData, unk) == 10);
    assert!(offset_of!(UnkUtilData, height) == 12);
    assert!(offset_of!(UnkUtilData, func) == 16);
    assert!(size_of::<Wallpaper>() == 12);
    assert!(offset_of!(Wallpaper, tiles) == 0);
    assert!(offset_of!(Wallpaper, tilemap) == 4);
    assert!(offset_of!(Wallpaper, palettes) == 8);
    assert!(size_of::<StorageMenu>() == 8);
    assert!(offset_of!(StorageMenu, text) == 0);
    assert!(offset_of!(StorageMenu, textId) == 4);
    assert!(size_of::<TilemapUtil_RectData>() == 12);
    assert!(offset_of!(TilemapUtil_RectData, x) == 0);
    assert!(offset_of!(TilemapUtil_RectData, y) == 2);
    assert!(offset_of!(TilemapUtil_RectData, width) == 4);
    assert!(offset_of!(TilemapUtil_RectData, height) == 6);
    assert!(offset_of!(TilemapUtil_RectData, destX) == 8);
    assert!(offset_of!(TilemapUtil_RectData, destY) == 10);
    assert!(size_of::<Anon1>() == 8);
    assert!(offset_of!(Anon1, func) == 0);
    assert!(offset_of!(Anon1, area) == 4);
    assert!(size_of::<StorageMessage>() == 8);
    assert!(offset_of!(StorageMessage, text) == 0);
    assert!(offset_of!(StorageMessage, format) == 4);
    assert!(size_of::<sRestrictedReleaseMoves_0_t>() == 4);
    assert!(offset_of!(sRestrictedReleaseMoves_0_t, mapGroup) == 0);
    assert!(offset_of!(sRestrictedReleaseMoves_0_t, mapNum) == 1);
    assert!(offset_of!(sRestrictedReleaseMoves_0_t, r#move) == 2);
    assert!(size_of::<sTilemapDimensions_0_0_t>() == 4);
    assert!(offset_of!(sTilemapDimensions_0_0_t, width) == 0);
    assert!(offset_of!(sTilemapDimensions_0_0_t, height) == 2);
    assert!(size_of::<PokemonStorageSystemData_summaryMon>() == 4);
    assert!(size_of::<ItemIcon>() == 16);
    assert!(offset_of!(ItemIcon, sprite) == 0);
    assert!(offset_of!(ItemIcon, tiles) == 4);
    assert!(offset_of!(ItemIcon, palIndex) == 8);
    assert!(offset_of!(ItemIcon, area) == 10);
    assert!(offset_of!(ItemIcon, pos) == 11);
    assert!(offset_of!(ItemIcon, active) == 12);
};

const BOXID_CANCELED: u8 = 201;
const BOXID_NONE_CHOSEN: u8 = 200;
const CHANGE_GRAB: u8 = 0;
const CHANGE_PLACE: u8 = 1;
const CHANGE_SHIFT: u8 = 2;
const CURSOR_ANIM_BOUNCE: u8 = 0;
const CURSOR_ANIM_FIST: u8 = 3;
const CURSOR_ANIM_OPEN: u8 = 2;
const CURSOR_ANIM_STILL: u8 = 1;
const CURSOR_AREA_BOX_TITLE: u8 = 2;
const CURSOR_AREA_BUTTONS: i8 = 3;
const CURSOR_AREA_IN_BOX: u8 = 0;
const CURSOR_AREA_IN_PARTY: i8 = 1;
const GFXTAG_BOX_TITLE: u16 = 3;
const GFXTAG_BOX_TITLE_ALT: u16 = 4;
const GFXTAG_CHOOSE_BOX_MENU: u16 = 10;
const GFXTAG_CURSOR: u16 = 0;
const GFXTAG_CURSOR_SHADOW: u16 = 1;
const GFXTAG_DISPLAY_MON: u16 = 2;
const GFXTAG_ITEM_ICON_0: u16 = 7;
const GFXTAG_MARKING_COMBO: u16 = 16;
const GFXTAG_MARKING_MENU: u16 = 13;
const INPUT_BOX_OPTIONS: u8 = 7;
const INPUT_CLOSE_BOX: u8 = 4;
const INPUT_DEPOSIT: u8 = 11;
const INPUT_GIVE_ITEM: u8 = 17;
const INPUT_HIDE_PARTY: u8 = 6;
const INPUT_IN_MENU: u8 = 8;
const INPUT_MOVE_CURSOR: u8 = 1;
const INPUT_MOVE_MON: u8 = 13;
const INPUT_MULTIMOVE_CHANGE_SELECTION: u8 = 21;
const INPUT_MULTIMOVE_GRAB_SELECTION: u8 = 23;
const INPUT_MULTIMOVE_MOVE_MONS: u8 = 25;
const INPUT_MULTIMOVE_PLACE_MONS: u8 = 26;
const INPUT_MULTIMOVE_SINGLE: u8 = 22;
const INPUT_MULTIMOVE_START: u8 = 20;
const INPUT_MULTIMOVE_UNABLE: u8 = 24;
const INPUT_NONE: u8 = 0;
const INPUT_PLACE_MON: u8 = 15;
const INPUT_PRESSED_B: u8 = 19;
const INPUT_SCROLL_LEFT: u8 = 10;
const INPUT_SCROLL_RIGHT: u8 = 9;
const INPUT_SHIFT_MON: u8 = 14;
const INPUT_SHOW_PARTY: u8 = 5;
const INPUT_SWITCH_ITEMS: u8 = 18;
const INPUT_TAKE_ITEM: u8 = 16;
const INPUT_WITHDRAW: u8 = 12;
const ITEM_ANIM_APPEAR: u8 = 1;
const ITEM_ANIM_DISAPPEAR: u8 = 2;
const ITEM_ANIM_LARGE: u8 = 6;
const ITEM_ANIM_PICK_UP: u8 = 3;
const ITEM_ANIM_PUT_AWAY: u8 = 5;
const ITEM_ANIM_PUT_DOWN: u8 = 4;
const ITEM_CB_HIDE_PARTY: u8 = 7;
const ITEM_CB_SWAP_TO_HAND: u8 = 3;
const ITEM_CB_SWAP_TO_MON: u8 = 4;
const ITEM_CB_TO_HAND: u8 = 1;
const ITEM_CB_TO_MON: u8 = 2;
const ITEM_CB_WAIT_ANIM: u8 = 0;
const MAX_ITEM_ICONS: u8 = 3;
const MENU_BAG: i16 = 16;
const MENU_BEACH: u8 = 31;
const MENU_CANCEL: u8 = 0;
const MENU_CAVE: u8 = 30;
const MENU_CITY: u8 = 24;
const MENU_CRAG: u8 = 27;
const MENU_DESERT: u8 = 25;
const MENU_ETCETERA: i16 = 21;
const MENU_FOREST: i16 = 23;
const MENU_FRIENDS: i16 = 22;
const MENU_GIVE: i8 = 13;
const MENU_GIVE_2: i16 = 14;
const MENU_INFO: i16 = 17;
const MENU_JUMP: i16 = 9;
const MENU_MACHINE: u8 = 37;
const MENU_MARK: i16 = 8;
const MENU_MOVE: i8 = 3;
const MENU_NAME: i16 = 11;
const MENU_PLACE: i8 = 5;
const MENU_POKECENTER: u8 = 36;
const MENU_POLKADOT: u8 = 35;
const MENU_RELEASE: i16 = 7;
const MENU_RIVER: u8 = 33;
const MENU_SAVANNA: u8 = 26;
const MENU_SCENERY_1: i16 = 18;
const MENU_SCENERY_2: i16 = 19;
const MENU_SCENERY_3: i16 = 20;
const MENU_SEAFLOOR: u8 = 32;
const MENU_SHIFT: i8 = 4;
const MENU_SIMPLE: u8 = 38;
const MENU_SKY: u8 = 34;
const MENU_SNOW: u8 = 29;
const MENU_STORE: i8 = 1;
const MENU_SUMMARY: i16 = 6;
const MENU_SWITCH: i8 = 15;
const MENU_TAKE: i8 = 12;
const MENU_VOLCANO: u8 = 28;
const MENU_WALLPAPER: i16 = 10;
const MENU_WITHDRAW: i8 = 2;
const MODE_BOX: u8 = 1;
const MODE_MOVE: u8 = 2;
const MODE_PARTY: u8 = 0;
const MOVE_MODE_MULTIPLE_MOVING: u8 = 2;
const MOVE_MODE_MULTIPLE_SELECTING: u8 = 1;
const MOVE_MODE_NORMAL: u8 = 0;
const MSG_BAG_FULL: u8 = 26;
const MSG_BOX_IS_FULL: u8 = 8;
const MSG_BYE_BYE: u8 = 11;
const MSG_CAME_BACK: u8 = 19;
const MSG_CANT_RELEASE_EGG: u8 = 17;
const MSG_CANT_STORE_MAIL: u8 = 30;
const MSG_CHANGED_TO_ITEM: u8 = 29;
const MSG_CONTINUE_BOX: u8 = 18;
const MSG_DEPOSIT_IN_WHICH_BOX: u8 = 6;
const MSG_EXIT_BOX: u8 = 0;
const MSG_GIVE_TO_MON: u8 = 24;
const MSG_HOLDING_POKE: u8 = 15;
const MSG_IS_SELECTED: u8 = 4;
const MSG_IS_SELECTED2: u8 = 23;
const MSG_ITEM_IS_HELD: u8 = 28;
const MSG_JUMP_TO_WHICH_BOX: u8 = 5;
const MSG_LAST_POKE: u8 = 13;
const MSG_MARK_POKE: u8 = 12;
const MSG_PARTY_FULL: u8 = 14;
const MSG_PICK_A_THEME: u8 = 2;
const MSG_PICK_A_WALLPAPER: u8 = 3;
const MSG_PLACED_IN_BAG: u8 = 25;
const MSG_PLEASE_REMOVE_MAIL: u8 = 22;
const MSG_PUT_IN_BAG: u8 = 27;
const MSG_RELEASE_POKE: u8 = 9;
const MSG_SURPRISE: u8 = 21;
const MSG_VAR_ITEM_NAME: u8 = 7;
const MSG_VAR_MON_NAME_1: u8 = 1;
const MSG_VAR_MON_NAME_2: u8 = 2;
const MSG_VAR_MON_NAME_3: u8 = 3;
const MSG_VAR_NONE: u8 = 0;
const MSG_VAR_RELEASE_MON_1: u8 = 4;
const MSG_VAR_RELEASE_MON_2: u8 = 5;
const MSG_VAR_RELEASE_MON_3: u8 = 6;
const MSG_WAS_RELEASED: u8 = 10;
const MSG_WHAT_YOU_DO: u8 = 1;
const MSG_WHICH_ONE_WILL_TAKE: u8 = 16;
const MSG_WORRIED: u8 = 20;
const MSTATE_ERROR_HAS_MAIL: u8 = 5;
const MSTATE_ERROR_LAST_PARTY_MON: u8 = 4;
const MSTATE_HANDLE_INPUT: u8 = 0;
const MSTATE_MOVE_CURSOR: u8 = 1;
const MSTATE_MULTIMOVE_RUN: u8 = 7;
const MSTATE_MULTIMOVE_RUN_CANCEL: u8 = 8;
const MSTATE_MULTIMOVE_RUN_MOVED: u8 = 9;
const MSTATE_SCROLL_BOX: u8 = 2;
const MSTATE_SCROLL_BOX_ITEM: u8 = 10;
const MSTATE_WAIT_ERROR_MSG: u8 = 6;
const MSTATE_WAIT_ITEM_ANIM: u8 = 11;
const MSTATE_WAIT_MSG: u8 = 3;
const MULTIMOVE_CANCEL: u8 = 1;
const MULTIMOVE_CHANGE_SELECTION: u8 = 2;
const MULTIMOVE_GRAB_SELECTION: u8 = 3;
const MULTIMOVE_MOVE_MONS: u8 = 4;
const MULTIMOVE_PLACE_MONS: u8 = 5;
const MULTIMOVE_START: u8 = 0;
const OPTIONS_COUNT: u8 = 5;
const OPTION_DEPOSIT: u8 = 1;
const OPTION_EXIT: i16 = 4;
const OPTION_MOVE_ITEMS: u8 = 3;
const OPTION_MOVE_MONS: u8 = 2;
const OPTION_WITHDRAW: i16 = 0;
const PALTAG_BOX_TITLE: u16 = 56009;
const PALTAG_DISPLAY_MON: u16 = 56006;
const PALTAG_ITEM_ICON_0: u16 = 56011;
const PALTAG_MARKING_COMBO: u16 = 56008;
const PALTAG_MARKING_MENU: u16 = 56014;
const PALTAG_MISC_1: u16 = 56007;
const PALTAG_MISC_2: u16 = 56010;
const PALTAG_MON_ICON_0: u16 = 56000;
const RELEASE_ANIM_CAME_BACK: u8 = 1;
const RELEASE_ANIM_RELEASE: u8 = 0;
const SCREEN_CHANGE_EXIT_BOX: u8 = 0;
const SCREEN_CHANGE_ITEM_FROM_BAG: u8 = 3;
const SCREEN_CHANGE_NAME_BOX: u8 = 2;
const SCREEN_CHANGE_SUMMARY_SCREEN: u8 = 1;
const STATE_ENTER_PC: i16 = 4;
const STATE_ERROR_MSG: i16 = 3;
const STATE_FADE_IN: i16 = 1;
const STATE_HANDLE_INPUT: i16 = 2;
const STATE_LOAD: i16 = 0;
const TILEMAPID_CLOSE_BUTTON: u8 = 2;
const TILEMAPID_COUNT: u8 = 3;
const TILEMAPID_PARTY_MENU: u8 = 1;
const TILEMAPID_PKMN_DATA: u8 = 0;
const WALLPAPER_COUNT: u8 = 17;
const WALLPAPER_FRIENDS: i16 = 16;
const WALLPAPER_SAVANNA: i32 = 3;
const WIN_DISPLAY_INFO: u8 = 0;
const WIN_ITEM_DESC: u8 = 2;
const WIN_MESSAGE: u8 = 1;

static inputFuncs_9: Table<CArray<Anon1, 5>> =
    Table((&raw const crate::data::pokemon_storage_system::inputFuncs_9).cast());
static placeChangeFuncs_10: Table<CArray<Option<unsafe extern "C" fn() -> u8>, 3>> =
    Table((&raw const crate::data::pokemon_storage_system::placeChangeFuncs_10).cast());
static sAffineAnims_ReleaseMon: Table<CArray<*mut AffineAnimCmd, 2>> =
    Table((&raw const crate::data::pokemon_storage_system::sAffineAnims_ReleaseMon).cast());
static sAnim_Cursor_Bouncing_2: Table<CArray<AnimCmd, 3>> =
    Table((&raw const crate::data::pokemon_storage_system::sAnim_Cursor_Bouncing_2).cast());
static sAnim_Cursor_Fist_5: Table<CArray<AnimCmd, 2>> =
    Table((&raw const crate::data::pokemon_storage_system::sAnim_Cursor_Fist_5).cast());
static sAnim_Cursor_Open_4: Table<CArray<AnimCmd, 2>> =
    Table((&raw const crate::data::pokemon_storage_system::sAnim_Cursor_Open_4).cast());
static sAnim_Cursor_Still_3: Table<CArray<AnimCmd, 2>> =
    Table((&raw const crate::data::pokemon_storage_system::sAnim_Cursor_Still_3).cast());
static sAnims_ChooseBoxMenu: Table<CArray<*mut AnimCmd, 4>> =
    Table((&raw const crate::data::pokemon_storage_system::sAnims_ChooseBoxMenu).cast());
static sAnims_Cursor_6: Table<CArray<*mut AnimCmd, 4>> =
    Table((&raw const crate::data::pokemon_storage_system::sAnims_Cursor_6).cast());
static sBgTemplates: Table<CArray<BgTemplate, 4>> =
    Table((&raw const crate::data::pokemon_storage_system::sBgTemplates).cast());
static sBoxTitleColors: Table<CArray<CArray<u16, 2>, 17>> =
    Table((&raw const crate::data::pokemon_storage_system::sBoxTitleColors).cast());
static sChooseBoxMenuCenter_Gfx: Table<CArray<u8, 2048>> =
    Table((&raw const crate::data::pokemon_storage_system::sChooseBoxMenuCenter_Gfx).cast());
static sChooseBoxMenuSides_Gfx: Table<CArray<u8, 384>> =
    Table((&raw const crate::data::pokemon_storage_system::sChooseBoxMenuSides_Gfx).cast());
static sChooseBoxMenu_Pal: Table<CArray<u16, 16>> =
    Table((&raw const crate::data::pokemon_storage_system::sChooseBoxMenu_Pal).cast());
static sChooseBoxMenu_TextColors: Table<CArray<u8, 3>> =
    Table((&raw const crate::data::pokemon_storage_system::sChooseBoxMenu_TextColors).cast());
static sCloseBoxButton_Tilemap: Table<CArray<u16, 36>> =
    Table((&raw const crate::data::pokemon_storage_system::sCloseBoxButton_Tilemap).cast());
static sDisplayMenu_Tilemap: Table<CArray<u32, 63>> =
    Table((&raw const crate::data::pokemon_storage_system::sDisplayMenu_Tilemap).cast());
static sHandCursorShadow_Gfx: Table<CArray<u8, 128>> =
    Table((&raw const crate::data::pokemon_storage_system::sHandCursorShadow_Gfx).cast());
static sHandCursor_Gfx: Table<CArray<u8, 2048>> =
    Table((&raw const crate::data::pokemon_storage_system::sHandCursor_Gfx).cast());
static sHandCursor_Pal: Table<CArray<u16, 16>> =
    Table((&raw const crate::data::pokemon_storage_system::sHandCursor_Pal).cast());
static sInterface_Pal: Table<CArray<u16, 16>> =
    Table((&raw const crate::data::pokemon_storage_system::sInterface_Pal).cast());
static sItemInfoFrame_Gfx: Table<CArray<u32, 32>> =
    Table((&raw const crate::data::pokemon_storage_system::sItemInfoFrame_Gfx).cast());
static sMainMenuTexts: Table<CArray<sMainMenuTexts_0_t, 5>> =
    Table((&raw const crate::data::pokemon_storage_system::sMainMenuTexts).cast());
static sMenuTexts: Table<CArray<*mut u8, 39>> =
    Table((&raw const crate::data::pokemon_storage_system::sMenuTexts).cast());
static sMessages: Table<CArray<StorageMessage, 31>> =
    Table((&raw const crate::data::pokemon_storage_system::sMessages).cast());
static sOamData_CursorShadow_1: Table<OamData> =
    Table((&raw const crate::data::pokemon_storage_system::sOamData_CursorShadow_1).cast());
static sOamData_Cursor_0: Table<OamData> =
    Table((&raw const crate::data::pokemon_storage_system::sOamData_Cursor_0).cast());
static sPartySlotEmpty_Tilemap: Table<CArray<u16, 12>> =
    Table((&raw const crate::data::pokemon_storage_system::sPartySlotEmpty_Tilemap).cast());
static sPartySlotFilled_Tilemap: Table<CArray<u16, 12>> =
    Table((&raw const crate::data::pokemon_storage_system::sPartySlotFilled_Tilemap).cast());
static sPkmnDataGray_Pal: Table<CArray<u16, 16>> =
    Table((&raw const crate::data::pokemon_storage_system::sPkmnDataGray_Pal).cast());
static sPkmnData_Tilemap: Table<CArray<u16, 32>> =
    Table((&raw const crate::data::pokemon_storage_system::sPkmnData_Tilemap).cast());
static sRestrictedReleaseMoves: Table<CArray<sRestrictedReleaseMoves_0_t, 6>> =
    Table((&raw const crate::data::pokemon_storage_system::sRestrictedReleaseMoves).cast());
static sScrollingBgMoveItems_Pal: Table<CArray<u16, 16>> =
    Table((&raw const crate::data::pokemon_storage_system::sScrollingBgMoveItems_Pal).cast());
static sScrollingBg_Gfx: Table<CArray<u32, 38>> =
    Table((&raw const crate::data::pokemon_storage_system::sScrollingBg_Gfx).cast());
static sScrollingBg_Pal: Table<CArray<u16, 16>> =
    Table((&raw const crate::data::pokemon_storage_system::sScrollingBg_Pal).cast());
static sScrollingBg_Tilemap: Table<CArray<u32, 67>> =
    Table((&raw const crate::data::pokemon_storage_system::sScrollingBg_Tilemap).cast());
static sSpriteSheet_Arrow: Table<SpriteSheet> =
    Table((&raw const crate::data::pokemon_storage_system::sSpriteSheet_Arrow).cast());
static sSpriteSheet_Waveform: Table<SpriteSheet> =
    Table((&raw const crate::data::pokemon_storage_system::sSpriteSheet_Waveform).cast());
static sSpriteTemplate_Arrow: Table<SpriteTemplate> =
    Table((&raw const crate::data::pokemon_storage_system::sSpriteTemplate_Arrow).cast());
static sSpriteTemplate_BoxTitle: Table<SpriteTemplate> =
    Table((&raw const crate::data::pokemon_storage_system::sSpriteTemplate_BoxTitle).cast());
static sSpriteTemplate_CursorShadow_7: Table<SpriteTemplate> =
    Table((&raw const crate::data::pokemon_storage_system::sSpriteTemplate_CursorShadow_7).cast());
static sSpriteTemplate_Cursor_8: Table<SpriteTemplate> =
    Table((&raw const crate::data::pokemon_storage_system::sSpriteTemplate_Cursor_8).cast());
static sSpriteTemplate_DisplayMon: Table<SpriteTemplate> =
    Table((&raw const crate::data::pokemon_storage_system::sSpriteTemplate_DisplayMon).cast());
static sSpriteTemplate_ItemIcon: Table<SpriteTemplate> =
    Table((&raw const crate::data::pokemon_storage_system::sSpriteTemplate_ItemIcon).cast());
static sSpriteTemplate_MonIcon: Table<SpriteTemplate> =
    Table((&raw const crate::data::pokemon_storage_system::sSpriteTemplate_MonIcon).cast());
static sSpriteTemplate_Waveform: Table<SpriteTemplate> =
    Table((&raw const crate::data::pokemon_storage_system::sSpriteTemplate_Waveform).cast());
static sTextWindows_Pal: Table<CArray<u16, 16>> =
    Table((&raw const crate::data::pokemon_storage_system::sTextWindows_Pal).cast());
static sText_OutOf30: Table<CArray<u8, 4>> =
    Table((&raw const crate::data::pokemon_storage_system::sText_OutOf30).cast());
static sTilemapDimensions: Table<CArray<CArray<sTilemapDimensions_0_0_t, 4>, 2>> =
    Table((&raw const crate::data::pokemon_storage_system::sTilemapDimensions).cast());
static sWaldaWallpaperIcons: Table<CArray<*mut u32, 30>> =
    Table((&raw const crate::data::pokemon_storage_system::sWaldaWallpaperIcons).cast());
static sWaldaWallpapers: Table<CArray<Wallpaper, 16>> =
    Table((&raw const crate::data::pokemon_storage_system::sWaldaWallpapers).cast());
static sWallpapers: Table<CArray<Wallpaper, 16>> =
    Table((&raw const crate::data::pokemon_storage_system::sWallpapers).cast());
static sWaveformSpritePalette: Table<SpritePalette> =
    Table((&raw const crate::data::pokemon_storage_system::sWaveformSpritePalette).cast());
static sWindowTemplate_MainMenu: Table<WindowTemplate> =
    Table((&raw const crate::data::pokemon_storage_system::sWindowTemplate_MainMenu).cast());
static sWindowTemplate_MultiMove: Table<WindowTemplate> =
    Table((&raw const crate::data::pokemon_storage_system::sWindowTemplate_MultiMove).cast());
static sWindowTemplates: Table<CArray<WindowTemplate, 4>> =
    Table((&raw const crate::data::pokemon_storage_system::sWindowTemplates).cast());
static sYesNoWindowTemplate: Table<WindowTemplate> =
    Table((&raw const crate::data::pokemon_storage_system::sYesNoWindowTemplate).cast());

pub(crate) static mut sItemIconGfxBuffer: CArray<u32, 98> = unsafe { zeroed() };
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sPreviousBoxOption: u8 = 0;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sChooseBoxMenu: *mut ChooseBoxMenu = null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sStorage: *mut PokemonStorageSystemData = null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sInPartyMenu: u8 = 0;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sCurrentBoxOption: u8 = 0;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sDepositBoxId: u8 = 0;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sWhichToReshow: u8 = 0;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sLastUsedBox: u8 = 0;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sMovingItemId: u16 = 0;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sSavedMovingMon: Pokemon = unsafe { zeroed() };
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sCursorArea: i8 = 0;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sCursorPosition: i8 = 0;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sIsMonBeingMoved: u8 = 0;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sMovingMonOrigBoxId: u8 = 0;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sMovingMonOrigBoxPos: u8 = 0;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sAutoActionOn: u8 = 0;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sSavedCursorPosition: u8 = 0;
pub(crate) static mut sMultiMove: *mut typeof___sMultiMove_0_t = null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sTilemapUtil: *mut TilemapUtil = null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sNumTilemapUtilIds: u16 = 0;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sUnkUtil: *mut UnkUtil = null_mut();

unsafe extern "C" {
    static gDummySpriteAffineAnimTable: CArray<*mut AffineAnimCmd, 0>;
    static gDummySpriteAnimTable: CArray<*mut AnimCmd, 0>;
    static mut gFieldCallback: Option<unsafe extern "C" fn()>;
    static mut gKeyRepeatStartDelay: u16;
    static mut gLastViewedMonIndex: u8;
    static mut gMain: Main;
    static gMonFrontPicTable: CArray<CompressedSpriteSheet, 0>;
    static gMonIconPaletteIndices: CArray<u8, 0>;
    static mut gPaletteFade: PaletteFadeControl;
    static mut gPlayerParty: CArray<Pokemon, 6>;
    static mut gPlayerPartyCount: u8;
    static mut gPlttBufferUnfaded: CArray<u16, 512>;
    static mut gPokemonStoragePtr: *mut PokemonStorage;
    static mut gReservedSpriteTileCount: u16;
    static mut gSaveBlock1Ptr: *mut SaveBlock1;
    static mut gSaveBlock2Ptr: *mut SaveBlock2;
    static gSineTable: CArray<i16, 0>;
    static mut gSpecialVar_0x8004: u16;
    static mut gSpecialVar_ItemId: u16;
    static gSpeciesNames: CArray<CArray<u8, 11>, 0>;
    static mut gSprites: CArray<Sprite, 65>;
    static gStorageSystemMenu_Gfx: CArray<u32, 0>;
    static gStorageSystemPartyMenu_Pal: CArray<u16, 0>;
    static gStorageSystemPartyMenu_Tilemap: CArray<u32, 0>;
    static mut gTasks: CArray<Task, 0>;
    static gText_Box: CArray<u8, 0>;
    static gText_EggNickname: CArray<u8, 0>;
    static gText_JustOnePkmn: CArray<u8, 0>;
    static gText_PartyFull: CArray<u8, 0>;
    fn AddBagItem(a0: u16, a1: u16) -> u8;
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
    fn AddTextPrinterParameterized5(
        a0: u8,
        a1: u8,
        a2: *mut u8,
        a3: u8,
        a4: u8,
        a5: u8,
        a6: Option<unsafe extern "C" fn(*mut TextPrinterTemplate, u16)>,
        a7: u8,
        a8: u8,
    );
    fn AddWindow(a0: *mut WindowTemplate) -> u16;
    fn AddWindow8Bit(a0: *mut WindowTemplate) -> u16;
    fn Alloc(a0: u32) -> *mut c_void;
    fn AllocSpritePalette(a0: u16) -> u8;
    fn AnimateSprites();
    fn BeginNormalPaletteFade(a0: u32, a1: i8, a2: u8, a3: u8, a4: u16) -> u8;
    fn BlendPalettes(a0: u32, a1: u8, a2: u16);
    fn BlitBitmapRectToWindow4BitTo8Bit(
        a0: u8,
        a1: *mut u8,
        a2: u16,
        a3: u16,
        a4: u16,
        a5: i32,
        a6: u16,
        a7: u16,
        a8: u16,
        a9: u16,
        a10: u8,
    );
    fn BoxMonRestorePP(a0: *mut BoxPokemon);
    fn BoxMonToMon(a0: *mut BoxPokemon, a1: *mut Pokemon);
    fn BufferMonMarkingsMenuTiles();
    fn BuildOamBuffer();
    fn CB2_ReturnToField();
    fn CalculatePlayerPartyCount() -> u8;
    fn ChangeBgX(a0: u8, a1: i32, a2: u8) -> i32;
    fn ChangeBgY(a0: u8, a1: i32, a2: u8) -> i32;
    fn CheckForSpaceForDma3Request(a0: i16) -> i16;
    fn CleanupOverworldWindowsAndTilemaps();
    fn ClearDma3Requests();
    fn ClearGpuRegBits(a0: u8, a1: u16);
    fn ClearScheduledBgCopiesToVram();
    fn ClearStdWindowAndFrame(a0: u8, a1: u8);
    fn ClearStdWindowAndFrameToTransparent(a0: u8, a1: u8);
    fn ClearWindowTilemap(a0: u8);
    fn ComputerScreenCloseEffect(a0: u16, a1: u16, a2: u8);
    fn ComputerScreenOpenEffect(a0: u16, a1: u16, a2: u8);
    fn ConvertIntToDecimalStringN(a0: *mut u8, a1: i32, a2: i32, a3: u8) -> *mut u8;
    fn CopyBgTilemapBufferToVram(a0: u8);
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
    fn CopyToBgTilemapBufferRect(a0: u8, a1: *mut c_void, a2: u8, a3: u8, a4: u8, a5: u8);
    fn CopyWindowToVram(a0: u8, a1: u8);
    fn CopyWindowToVram8Bit(a0: u8, a1: u8);
    fn CpuFastSet(a0: *mut c_void, a1: *mut c_void, a2: u32);
    fn CpuSet(a0: *mut c_void, a1: *mut c_void, a2: u32);
    fn CreateBoxMon(a0: *mut BoxPokemon, a1: u16, a2: u8, a3: u8, a4: u8, a5: u32, a6: u8, a7: u32);
    fn CreateMonMarkingComboSprite(a0: u16, a1: u16, a2: *mut u16) -> *mut Sprite;
    fn CreateSprite(a0: *mut SpriteTemplate, a1: i16, a2: i16, a3: u8) -> u8;
    fn CreateTask(a0: Option<unsafe extern "C" fn(u8)>, a1: u8) -> u8;
    fn CreateYesNoMenu(a0: *mut WindowTemplate, a1: u16, a2: u8, a3: u8);
    fn DeactivateAllTextPrinters();
    fn DecompressAndLoadBgGfxUsingHeap(a0: u8, a1: *mut c_void, a2: u32, a3: u16, a4: u8);
    fn DestroySprite(a0: *mut Sprite);
    fn DestroyTask(a0: u8);
    fn DoNamingScreen(
        a0: u8,
        a1: *mut u8,
        a2: u16,
        a3: u16,
        a4: u32,
        a5: Option<unsafe extern "C" fn()>,
    );
    fn DoScheduledBgTilemapCopiesToVram();
    fn DrawDialogueFrame(a0: u8, a1: u8);
    fn DrawStdFrameWithCustomTileAndPalette(a0: u8, a1: u8, a2: u16, a3: u8);
    fn DrawStdWindowFrame(a0: u8, a1: u8);
    fn DrawTextBorderOuter(a0: u8, a1: u16, a2: u8);
    fn DynamicPlaceholderTextUtil_ExpandPlaceholders(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn DynamicPlaceholderTextUtil_Reset();
    fn DynamicPlaceholderTextUtil_SetPlaceholderPtr(a0: u8, a1: *mut u8);
    fn FadeInFromBlack();
    fn FadeScreen(a0: u8, a1: i8);
    fn FillBgTilemapBufferRect(a0: u8, a1: u16, a2: u8, a3: u8, a4: u8, a5: u8, a6: u8);
    fn FillBgTilemapBufferRect_Palette0(a0: u8, a1: u16, a2: u8, a3: u8, a4: u8, a5: u8);
    fn FillWindowPixelBuffer(a0: u8, a1: u8);
    fn FillWindowPixelBuffer8Bit(a0: u8, a1: u8);
    fn FillWindowPixelRect8Bit(a0: u8, a1: u8, a2: u16, a3: u16, a4: u16, a5: u16);
    fn FlagClear(a0: u16) -> u8;
    fn Free(a0: *mut c_void);
    fn FreeAllSpritePalettes();
    fn FreeAllWindowBuffers();
    fn FreeMonMarkingsMenu();
    fn FreeOamMatrix(a0: u8);
    fn FreeSpritePaletteByTag(a0: u16);
    fn FreeSpriteTileRanges();
    fn FreeSpriteTilesByTag(a0: u16);
    fn FuncIsActiveTask(a0: Option<unsafe extern "C" fn(u8)>) -> u8;
    fn GetBgAttribute(a0: u8, a1: u8) -> u16;
    fn GetBoxMonData2(a0: *mut BoxPokemon, a1: i32) -> u32;
    fn GetBoxMonData3(a0: *mut BoxPokemon, a1: i32, a2: *mut u8) -> u32;
    fn GetGenderFromSpeciesAndPersonality(a0: u16, a1: u32) -> u8;
    fn GetIconSpecies(a0: u16, a1: u32) -> u16;
    fn GetItemDescription(a0: u16) -> *mut u8;
    fn GetItemIconPicOrPalette(a0: u16, a1: u8) -> *mut c_void;
    fn GetItemName(a0: u16) -> *mut u8;
    fn GetLevelFromBoxMonExp(a0: *mut BoxPokemon) -> u8;
    fn GetMaxWidthInMenuTable(a0: *mut MenuAction, a1: i32) -> i32;
    fn GetMonData2(a0: *mut Pokemon, a1: i32) -> u32;
    fn GetMonData3(a0: *mut Pokemon, a1: i32, a2: *mut u8) -> u32;
    fn GetMonFrontSpritePal(a0: *mut Pokemon) -> *mut u32;
    fn GetMonGender(a0: *mut Pokemon) -> u8;
    fn GetMonIconPtr(a0: u16, a1: u32, a2: u32) -> *mut u8;
    fn GetMonIconTiles(a0: u16, a1: u32) -> *mut u8;
    fn GetMonSpritePalFromSpeciesAndPersonality(a0: u16, a1: u32, a2: u32) -> *mut u32;
    fn GetSpriteTileStartByTag(a0: u16) -> u16;
    fn GetStringCenterAlignXOffset(a0: i32, a1: *mut u8, a2: i32) -> i32;
    fn GetStringWidth(a0: u8, a1: *mut u8, a2: i16) -> i32;
    fn GetTextWindowPalette(a0: u8) -> *mut u16;
    fn GetValidMonIconPalIndex(a0: u16) -> u8;
    fn GetWindowAttribute(a0: u8, a1: u8) -> u32;
    fn GoToBagMenu(a0: u8, a1: u8, a2: Option<unsafe extern "C" fn()>);
    fn HandleMonMarkingsMenuInput() -> u8;
    fn HideBg(a0: u8);
    fn IndexOfSpritePaletteTag(a0: u16) -> u8;
    fn InitBgsFromTemplates(a0: u8, a1: *mut BgTemplate, a2: u8);
    fn InitMenuInUpperLeftCornerNormal(a0: u8, a1: u8, a2: u8) -> u8;
    fn InitMonMarkingsMenu(a0: *mut MonMarkingsMenu);
    fn InitSpriteAffineAnim(a0: *mut Sprite);
    fn InitWindows(a0: *mut WindowTemplate) -> u16;
    fn IsComputerScreenCloseEffectActive() -> u8;
    fn IsComputerScreenOpenEffectActive() -> u8;
    fn IsDma3ManagerBusyWithBgCopy() -> u8;
    fn IsWeatherNotFadingIn() -> u8;
    fn ItemIsMail(a0: u16) -> u8;
    fn LZ77UnCompVram(a0: *mut u32, a1: *mut c_void);
    fn LZ77UnCompWram(a0: *mut u32, a1: *mut c_void);
    fn LoadBgTiles(a0: u8, a1: *mut c_void, a2: u16, a3: u16) -> u16;
    fn LoadCompressedSpriteSheet(a0: *mut CompressedSpriteSheet) -> u16;
    fn LoadMessageBoxAndBorderGfx();
    fn LoadMonIconPalettes();
    fn LoadOam();
    fn LoadPalette(a0: *mut c_void, a1: u16, a2: u16);
    fn LoadSpecialPokePic(
        a0: *mut CompressedSpriteSheet,
        a1: *mut c_void,
        a2: i32,
        a3: u32,
        a4: u8,
    );
    fn LoadSpritePalette(a0: *mut SpritePalette) -> u8;
    fn LoadSpritePalettes(a0: *mut SpritePalette);
    fn LoadSpriteSheet(a0: *mut SpriteSheet) -> u16;
    fn LoadSpriteSheets(a0: *mut SpriteSheet);
    fn LoadUserWindowBorderGfx(a0: u8, a1: u16, a2: u8);
    fn LockPlayerFieldControls();
    fn Menu_GetCursorPos() -> u8;
    fn Menu_MoveCursor(a0: i8) -> u8;
    fn Menu_MoveCursorNoWrapAround(a0: i8) -> u8;
    fn Menu_ProcessInput() -> i8;
    fn Menu_ProcessInputNoWrapClearOnChoose() -> i8;
    fn OpenMonMarkingsMenu(a0: u8, a1: i16, a2: i16);
    fn PlaySE(a0: u16);
    fn PrintMenuTable(a0: u8, a1: u8, a2: *mut MenuAction);
    fn ProcessSpriteCopyRequests();
    fn PutWindowTilemap(a0: u8);
    fn RemoveBagItem(a0: u16, a1: u16) -> u8;
    fn RemoveWindow(a0: u8);
    fn RequestDma3Fill(a0: i32, a1: *mut c_void, a2: u16, a3: u8) -> i16;
    fn ResetPaletteFade();
    fn ResetSpriteData();
    fn ResetTasks();
    fn RunTasks();
    fn ScheduleBgCopyTilemapToVram(a0: u8);
    fn ScriptContext_Enable();
    fn SetBgAttribute(a0: u8, a1: u8, a2: u8);
    fn SetBgTilemapBuffer(a0: u8, a1: *mut c_void);
    fn SetBoxMonData(a0: *mut BoxPokemon, a1: i32, a2: *mut c_void);
    fn SetGpuReg(a0: u8, a1: u16);
    fn SetGpuRegBits(a0: u8, a1: u16);
    fn SetMainCallback2(a0: Option<unsafe extern "C" fn()>);
    fn SetMonData(a0: *mut Pokemon, a1: i32, a2: *mut c_void);
    fn SetVBlankCallback(a0: Option<unsafe extern "C" fn()>);
    fn ShowBg(a0: u8);
    fn ShowPokemonSummaryScreen(
        a0: u8,
        a1: *mut c_void,
        a2: u8,
        a3: u8,
        a4: Option<unsafe extern "C" fn()>,
    );
    fn ShowPokemonSummaryScreenHandleDeoxys(
        a0: u8,
        a1: *mut BoxPokemon,
        a2: u8,
        a3: u8,
        a4: Option<unsafe extern "C" fn()>,
    );
    fn SpriteCallbackDummy(a0: *mut Sprite);
    fn StartSpriteAffineAnim(a0: *mut Sprite, a1: u8);
    fn StartSpriteAnim(a0: *mut Sprite, a1: u8);
    fn StartSpriteAnimIfDifferent(a0: *mut Sprite, a1: u8);
    fn StringAppend(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn StringCopy(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn StringCopyPadded(a0: *mut u8, a1: *mut u8, a2: u8, a3: u16) -> *mut u8;
    fn StringFill(a0: *mut u8, a1: u8, a2: u16) -> *mut u8;
    fn StringGet_Nickname(a0: *mut u8) -> *mut u8;
    fn StringLength(a0: *mut u8) -> u16;
    fn StringLength_Multibyte(a0: *mut u8) -> u32;
    fn TransferPlttBuffer();
    fn TryLoadAllMonIconPalettesAtOffset(a0: u16);
    fn UnlockPlayerFieldControls();
    fn UpdateMonMarkingTiles(a0: u8, a1: *mut c_void);
    fn UpdatePaletteFade() -> u8;
    fn VarSet(a0: u16, a1: u16) -> u8;
    fn WriteSequenceToBgTilemapBuffer(
        a0: u8,
        a1: u16,
        a2: u8,
        a3: u8,
        a4: u8,
        a5: u8,
        a6: u8,
        a7: i16,
    );
    fn ZeroBoxMonData(a0: *mut BoxPokemon);
    fn ZeroMonData(a0: *mut Pokemon);
    fn malloc_and_decompress(a0: *mut c_void, a1: *mut u32) -> *mut c_void;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn DrawTextWindowAndBufferTiles(
    string: *mut u8,
    mut dst: *mut c_void,
    zero1: u8,
    zero2: u8,
    bytesToBuffer: i32,
) {
    let mut i: i32 = 0;
    let mut tileBytesToBuffer: i32 = 0;
    let mut remainingBytes: i32 = 0;
    let mut windowId: u16 = 0;
    let mut txtColor: CArray<u8, 3> = zeroed();
    let mut tileData1: *mut u8 = null_mut();
    let mut tileData2: *mut u8 = null_mut();
    let mut winTemplate: WindowTemplate = zeroed();
    winTemplate.bg = 0;
    winTemplate.width = 24;
    winTemplate.height = 2;
    windowId = AddWindow(&raw mut winTemplate);
    FillWindowPixelBuffer(windowId as u8, zero2 | zero2 << 4);
    tileData1 = GetWindowAttribute(windowId as u8, WINDOW_TILE_DATA) as usize as *mut u8;
    tileData2 = tileData1.at(winTemplate.width as i32 * 32);
    if zero1 == 0 {
        txtColor[0] = 0x0;
    } else {
        txtColor[0] = zero2;
    }
    txtColor[1] = TEXT_DYNAMIC_COLOR_6;
    txtColor[2] = TEXT_DYNAMIC_COLOR_5;
    AddTextPrinterParameterized4(
        windowId as u8,
        FONT_NORMAL,
        0,
        1,
        0,
        0,
        txtColor.as_mut_ptr(),
        TEXT_SKIP_DRAW as i8,
        string,
    );
    tileBytesToBuffer = bytesToBuffer;
    if tileBytesToBuffer > 6 {
        tileBytesToBuffer = 6;
    }
    remainingBytes = bytesToBuffer - 6;
    if tileBytesToBuffer > 0 {
        i = tileBytesToBuffer;
        while i != 0 {
            CpuSet(tileData1 as *mut c_void, dst, 64);
            CpuSet(
                tileData2 as *mut c_void,
                (dst as *mut u8).at(128) as *mut c_void,
                64,
            );
            tileData1 = tileData1.at(128);
            tileData2 = tileData2.at(128);
            dst = (dst as *mut u8).at(256) as *mut c_void;
            i -= 1;
        }
    }
    if remainingBytes > 0 {
        {
            {
                let mut tmp: u16 = 0;
                volatile_write(&raw mut tmp, (zero2 as u16) << 4 | zero2 as u16);
                CpuSet(
                    &raw mut tmp as *mut c_void,
                    dst,
                    0x1000000 | remainingBytes as u32 * 0x100 / 2 & 0x1FFFFF,
                );
            }
        }
    }
    RemoveWindow(windowId as u8);
}
pub(crate) unsafe extern "C" fn UnusedDrawTextWindow(
    string: *mut u8,
    dst: *mut c_void,
    offset: u16,
    bgColor: u8,
    fgColor: u8,
    shadowColor: u8,
) {
    let mut tilesSize: u32 = 0;
    let mut windowId: u8 = 0;
    let mut txtColor: CArray<u8, 3> = zeroed();
    let mut tileData1: *mut u8 = null_mut();
    let mut tileData2: *mut u8 = null_mut();
    let mut winTemplate: WindowTemplate = zeroed();
    winTemplate.bg = 0;
    winTemplate.width = StringLength_Multibyte(string) as u8;
    winTemplate.height = 2;
    tilesSize = winTemplate.width as u32 * 32;
    windowId = AddWindow(&raw mut winTemplate) as u8;
    FillWindowPixelBuffer(windowId, bgColor | bgColor << 4);
    tileData1 = GetWindowAttribute(windowId, WINDOW_TILE_DATA) as usize as *mut u8;
    tileData2 = tileData1.at(winTemplate.width as i32 * 32);
    txtColor[0] = bgColor;
    txtColor[1] = fgColor;
    txtColor[2] = shadowColor;
    AddTextPrinterParameterized4(
        windowId,
        FONT_NORMAL,
        0,
        2,
        0,
        0,
        txtColor.as_mut_ptr(),
        TEXT_SKIP_DRAW as i8,
        string,
    );
    CpuSet(
        tileData1 as *mut c_void,
        dst,
        0x00000000 | tilesSize / 2 & 0x1FFFFF,
    );
    CpuSet(
        tileData2 as *mut c_void,
        (dst as *mut u8).at(offset) as *mut c_void,
        0x00000000 | tilesSize / 2 & 0x1FFFFF,
    );
    RemoveWindow(windowId);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CountMonsInBox(boxId: u8) -> u8 {
    let mut i: u16 = 0;
    let mut count: u16 = 0;
    i = 0;
    count = 0;
    while i < IN_BOX_COUNT as u16 {
        if GetBoxMonDataAt(boxId, i as u8, MON_DATA_SPECIES) != SPECIES_NONE as u32 {
            count += 1;
        }
        i += 1;
    }
    return count as u8;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetFirstFreeBoxSpot(boxId: u8) -> i16 {
    let mut i: u16 = 0;
    i = 0;
    while i < IN_BOX_COUNT as u16 {
        if GetBoxMonDataAt(boxId, i as u8, MON_DATA_SPECIES) == SPECIES_NONE as u32 {
            return i as i16;
        }
        i += 1;
    }
    return -1;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CountPartyNonEggMons() -> u8 {
    let mut i: u16 = 0;
    let mut count: u16 = 0;
    i = 0;
    count = 0;
    while i < PARTY_SIZE as u16 {
        if GetMonData2(&raw mut gPlayerParty[i], MON_DATA_SPECIES) != SPECIES_NONE as u32
            && GetMonData2(&raw mut gPlayerParty[i], MON_DATA_IS_EGG) == 0
        {
            count += 1;
        }
        i += 1;
    }
    return count as u8;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CountPartyAliveNonEggMonsExcept(slotToIgnore: u8) -> u8 {
    let mut i: u16 = 0;
    let mut count: u16 = 0;
    i = 0;
    count = 0;
    while i < PARTY_SIZE as u16 {
        if i != slotToIgnore as u16
            && GetMonData2(&raw mut gPlayerParty[i], MON_DATA_SPECIES) != SPECIES_NONE as u32
            && GetMonData2(&raw mut gPlayerParty[i], MON_DATA_IS_EGG) == 0
            && GetMonData2(&raw mut gPlayerParty[i], MON_DATA_HP) != 0
        {
            count += 1;
        }
        i += 1;
    }
    return count as u8;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CountPartyAliveNonEggMons_IgnoreVar0x8004Slot() -> u16 {
    return CountPartyAliveNonEggMonsExcept(gSpecialVar_0x8004 as u8) as u16;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CountPartyMons() -> u8 {
    let mut i: u16 = 0;
    let mut count: u16 = 0;
    i = 0;
    count = 0;
    while i < PARTY_SIZE as u16 {
        if GetMonData2(&raw mut gPlayerParty[i], MON_DATA_SPECIES) != SPECIES_NONE as u32 {
            count += 1;
        }
        i += 1;
    }
    return count as u8;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn StringCopyAndFillWithSpaces(
    dst: *mut u8,
    src: *mut u8,
    n: u16,
) -> *mut u8 {
    let mut str: *mut u8 = null_mut();
    str = StringCopy(dst, src);
    while str < dst.at(n) {
        *str = CHAR_SPACE;
        str = str.at(1);
    }
    *str = EOS;
    return str;
}
pub(crate) unsafe extern "C" fn UnusedWriteRectCpu(
    mut dest: *mut u16,
    dest_left: u16,
    dest_top: u16,
    mut src: *mut u16,
    src_left: u16,
    src_top: u16,
    mut dest_width: u16,
    dest_height: u16,
    src_width: u16,
) {
    let mut i: u16 = 0;
    dest_width *= 2;
    dest = dest.at(dest_top as i32 * 0x20 + dest_left as i32);
    src = src.at(src_top as i32 * src_width as i32 + src_left as i32);
    i = 0;
    while i < dest_height {
        CpuSet(
            src as *mut c_void,
            dest as *mut c_void,
            0x00000000 | (dest_width as i32 / 2) as u32 & 0x1FFFFF,
        );
        dest = dest.at(32);
        src = src.at(src_width);
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn UnusedWriteRectDma(
    mut dest: *mut u16,
    dest_left: u16,
    dest_top: u16,
    mut width: u16,
    height: u16,
) {
    let mut i: u16 = 0;
    dest = dest.at(dest_top as i32 * 0x20 + dest_left as i32);
    width *= 2;
    i = 0;
    while i < height {
        let mut _dest: *mut c_void = dest as *mut c_void;
        let mut _size: u32 = width as u32;
        loop {
            if _size <= 0x1000 {
                {
                    {
                        let mut tmp: u16 = 0;
                        volatile_write(&raw mut tmp, 0);
                        {
                            {
                                let mut dmaRegs: *mut u32 = 67109076 as usize as *mut u32;
                                volatile_write(dmaRegs, &raw mut tmp as usize as u32);
                                volatile_write(dmaRegs.at(1), _dest as usize as u32);
                                volatile_write(dmaRegs.at(2), 0x81000000 | _size / 2);
                                let _ = (dmaRegs.at(2)).read_volatile();
                            }
                        }
                    }
                }
                break;
            }
            {
                {
                    let mut tmp: u16 = 0;
                    volatile_write(&raw mut tmp, 0);
                    {
                        {
                            let mut dmaRegs: *mut u32 = 67109076 as usize as *mut u32;
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
        }
        dest = dest.at(32);
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn Task_PCMainMenu(taskId: u8) {
    let mut task: *mut Task = &raw mut gTasks[taskId];
    match (*task).data[0] {
        STATE_LOAD => {
            CreateMainMenu((*task).data[1] as u8, &raw mut (*task).data[15]);
            LoadMessageBoxAndBorderGfx();
            DrawDialogueFrame(0, 0);
            FillWindowPixelBuffer(0, 17);
            AddTextPrinterParameterized2(
                0,
                FONT_NORMAL,
                sMainMenuTexts[(*task).data[1]].desc,
                TEXT_SKIP_DRAW,
                None,
                TEXT_COLOR_DARK_GRAY,
                TEXT_COLOR_WHITE,
                TEXT_COLOR_LIGHT_GRAY,
            );
            CopyWindowToVram(0, COPYWIN_FULL);
            CopyWindowToVram((*task).data[15] as u8, COPYWIN_FULL);
            (*task).data[0] += 1;
        }
        STATE_FADE_IN => {
            if IsWeatherNotFadingIn() != 0 {
                (*task).data[0] += 1;
            }
        }
        STATE_HANDLE_INPUT => {
            (*task).data[2] = Menu_ProcessInput() as i16;
            match (*task).data[2] {
                -2 => {
                    (*task).data[3] = (*task).data[1];
                    if gMain.newKeys as i32 & DPAD_UP != 0
                        && ({
                            (*task).data[3] -= 1;
                            (*task).data[3]
                        }) < 0
                    {
                        (*task).data[3] = 4;
                    }
                    if gMain.newKeys as i32 & DPAD_DOWN != 0
                        && ({
                            (*task).data[3] += 1;
                            (*task).data[3]
                        }) > 4
                    {
                        (*task).data[3] = 0;
                    }
                    if (*task).data[1] != (*task).data[3] {
                        (*task).data[1] = (*task).data[3];
                        FillWindowPixelBuffer(0, 17);
                        AddTextPrinterParameterized2(
                            0,
                            FONT_NORMAL,
                            sMainMenuTexts[(*task).data[1]].desc,
                            0,
                            None,
                            TEXT_COLOR_DARK_GRAY,
                            TEXT_COLOR_WHITE,
                            TEXT_COLOR_LIGHT_GRAY,
                        );
                    }
                }
                -1 | OPTION_EXIT => {
                    ClearStdWindowAndFrame((*task).data[15] as u8, TRUE);
                    UnlockPlayerFieldControls();
                    ScriptContext_Enable();
                    RemoveWindow((*task).data[15] as u8);
                    DestroyTask(taskId);
                }
                _ => {
                    if (*task).data[2] == OPTION_WITHDRAW && CountPartyMons() == PARTY_SIZE as u8 {
                        FillWindowPixelBuffer(0, 17);
                        AddTextPrinterParameterized2(
                            0,
                            FONT_NORMAL,
                            gText_PartyFull.as_ptr().cast_mut(),
                            0,
                            None,
                            TEXT_COLOR_DARK_GRAY,
                            TEXT_COLOR_WHITE,
                            TEXT_COLOR_LIGHT_GRAY,
                        );
                        (*task).data[0] = STATE_ERROR_MSG;
                    } else if (*task).data[2] == OPTION_DEPOSIT as i16 && CountPartyMons() == 1 {
                        FillWindowPixelBuffer(0, 17);
                        AddTextPrinterParameterized2(
                            0,
                            FONT_NORMAL,
                            gText_JustOnePkmn.as_ptr().cast_mut(),
                            0,
                            None,
                            TEXT_COLOR_DARK_GRAY,
                            TEXT_COLOR_WHITE,
                            TEXT_COLOR_LIGHT_GRAY,
                        );
                        (*task).data[0] = STATE_ERROR_MSG;
                    } else {
                        FadeScreen(FADE_TO_BLACK, 0);
                        (*task).data[0] = STATE_ENTER_PC;
                    }
                }
            }
        }
        STATE_ERROR_MSG => {
            if gMain.newKeys as i32 & 3 != 0 {
                FillWindowPixelBuffer(0, 17);
                AddTextPrinterParameterized2(
                    0,
                    FONT_NORMAL,
                    sMainMenuTexts[(*task).data[1]].desc,
                    0,
                    None,
                    TEXT_COLOR_DARK_GRAY,
                    TEXT_COLOR_WHITE,
                    TEXT_COLOR_LIGHT_GRAY,
                );
                (*task).data[0] = STATE_HANDLE_INPUT;
            } else if gMain.newKeys as i32 & DPAD_UP != 0 {
                if ({
                    (*task).data[1] -= 1;
                    (*task).data[1]
                }) < 0
                {
                    (*task).data[1] = 4;
                }
                Menu_MoveCursor(-1);
                (*task).data[1] = Menu_GetCursorPos() as i16;
                FillWindowPixelBuffer(0, 17);
                AddTextPrinterParameterized2(
                    0,
                    FONT_NORMAL,
                    sMainMenuTexts[(*task).data[1]].desc,
                    0,
                    None,
                    TEXT_COLOR_DARK_GRAY,
                    TEXT_COLOR_WHITE,
                    TEXT_COLOR_LIGHT_GRAY,
                );
                (*task).data[0] = STATE_HANDLE_INPUT;
            } else if gMain.newKeys as i32 & DPAD_DOWN != 0 {
                if ({
                    (*task).data[1] += 1;
                    (*task).data[1]
                }) >= 4
                {
                    (*task).data[1] = 0;
                }
                Menu_MoveCursor(1);
                (*task).data[1] = Menu_GetCursorPos() as i16;
                FillWindowPixelBuffer(0, 17);
                AddTextPrinterParameterized2(
                    0,
                    FONT_NORMAL,
                    sMainMenuTexts[(*task).data[1]].desc,
                    0,
                    None,
                    TEXT_COLOR_DARK_GRAY,
                    TEXT_COLOR_WHITE,
                    TEXT_COLOR_LIGHT_GRAY,
                );
                (*task).data[0] = STATE_HANDLE_INPUT;
            }
        }
        STATE_ENTER_PC => {
            if gPaletteFade.active() == 0 {
                CleanupOverworldWindowsAndTilemaps();
                EnterPokeStorage((*task).data[2] as u8);
                RemoveWindow((*task).data[15] as u8);
                DestroyTask(taskId);
            }
        }
        _ => {}
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ShowPokemonStorageSystemPC() {
    let mut taskId: u8 = CreateTask(Some(Task_PCMainMenu), 80);
    gTasks[taskId].data[0] = 0;
    gTasks[taskId].data[1] = 0;
    LockPlayerFieldControls();
}
pub(crate) unsafe extern "C" fn FieldTask_ReturnToPcMenu() {
    let mut taskId: u8 = 0;
    let mut vblankCb: Option<unsafe extern "C" fn()> = gMain.vblankCallback;
    SetVBlankCallback(None);
    taskId = CreateTask(Some(Task_PCMainMenu), 80);
    gTasks[taskId].data[0] = 0;
    gTasks[taskId].data[1] = sPreviousBoxOption as i16;
    Task_PCMainMenu(taskId);
    SetVBlankCallback(vblankCb);
    FadeInFromBlack();
}
pub(crate) unsafe extern "C" fn CreateMainMenu(whichMenu: u8, windowIdPtr: *mut i16) {
    let mut windowId: i16 = 0;
    let mut template: WindowTemplate = zeroed();
    template = *sWindowTemplate_MainMenu;
    template.width = GetMaxWidthInMenuTable(
        sMainMenuTexts.as_ptr().cast_mut() as *mut c_void as *mut MenuAction,
        OPTIONS_COUNT as i32,
    ) as u8;
    windowId = AddWindow(&raw mut template) as i16;
    DrawStdWindowFrame(windowId as u8, FALSE);
    PrintMenuTable(
        windowId as u8,
        OPTIONS_COUNT,
        sMainMenuTexts.as_ptr().cast_mut() as *mut c_void as *mut MenuAction,
    );
    InitMenuInUpperLeftCornerNormal(windowId as u8, OPTIONS_COUNT, whichMenu);
    *windowIdPtr = windowId;
}
pub(crate) unsafe extern "C" fn CB2_ExitPokeStorage() {
    sPreviousBoxOption = GetCurrentBoxOption();
    gFieldCallback = Some(FieldTask_ReturnToPcMenu);
    SetMainCallback2(Some(CB2_ReturnToField));
}
pub(crate) unsafe extern "C" fn StorageSystemGetNextMonIndex(
    r#box: *mut BoxPokemon,
    startIdx: i8,
    stopIdx: u8,
    mode: u8,
) -> i16 {
    let mut i: i16 = 0;
    let mut direction: i16 = 0;
    if mode == 0 || mode == 1 {
        direction = 1;
    } else {
        direction = -1;
    }
    if mode == 1 || mode == 3 {
        i = startIdx as i16 + direction;
        while i >= 0 && i <= stopIdx as i16 {
            if GetBoxMonData2(r#box.at(i), MON_DATA_SPECIES) != 0 {
                return i;
            }
            i += direction;
        }
    } else {
        i = startIdx as i16 + direction;
        while i >= 0 && i <= stopIdx as i16 {
            if GetBoxMonData2(r#box.at(i), MON_DATA_SPECIES) != 0
                && GetBoxMonData2(r#box.at(i), MON_DATA_IS_EGG) == 0
            {
                return i;
            }
            i += direction;
        }
    }
    return -1;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ResetPokemonStorageSystem() {
    let mut boxId: u16 = 0;
    let mut boxPosition: u16 = 0;
    SetCurrentBox(0);
    boxId = 0;
    while boxId < TOTAL_BOXES_COUNT as u16 {
        boxPosition = 0;
        while boxPosition < IN_BOX_COUNT as u16 {
            ZeroBoxMonAt(boxId as u8, boxPosition as u8);
            boxPosition += 1;
        }
        boxId += 1;
    }
    boxId = 0;
    while boxId < TOTAL_BOXES_COUNT as u16 {
        let mut dest: *mut u8 =
            StringCopy(GetBoxNamePtr(boxId as u8), gText_Box.as_ptr().cast_mut());
        ConvertIntToDecimalStringN(dest, boxId as i32 + 1, STR_CONV_MODE_LEFT_ALIGN, 2);
        boxId += 1;
    }
    boxId = 0;
    while boxId < TOTAL_BOXES_COUNT as u16 {
        SetBoxWallpaper(boxId as u8, (boxId as i32 % 4) as u8);
        boxId += 1;
    }
    ResetWaldaWallpaper();
}
pub(crate) unsafe extern "C" fn LoadChooseBoxMenuGfx(
    menu: *mut ChooseBoxMenu,
    tileTag: u16,
    palTag: u16,
    subpriority: u8,
    loadPal: u32,
) {
    let mut palette: SpritePalette = zeroed();
    palette.data = sChooseBoxMenu_Pal.as_ptr().cast_mut();
    palette.tag = palTag;
    let mut sheets: CArray<SpriteSheet, 3> = zeroed();
    sheets[0].data = sChooseBoxMenuCenter_Gfx.as_ptr().cast_mut() as *mut c_void;
    sheets[0].size = 0x800;
    sheets[0].tag = tileTag;
    sheets[1].data = sChooseBoxMenuSides_Gfx.as_ptr().cast_mut() as *mut c_void;
    sheets[1].size = 0x180;
    sheets[1].tag = tileTag + 1;
    if loadPal != 0 {
        LoadSpritePalette(&raw mut palette);
    }
    LoadSpriteSheets(sheets.as_mut_ptr());
    sChooseBoxMenu = menu;
    (*menu).tileTag = tileTag;
    (*menu).paletteTag = palTag;
    (*menu).subpriority = subpriority;
    (*menu).loadedPalette = loadPal;
}
pub(crate) unsafe extern "C" fn FreeChooseBoxMenu() {
    if (*sChooseBoxMenu).loadedPalette != 0 {
        FreeSpritePaletteByTag((*sChooseBoxMenu).paletteTag);
    }
    FreeSpriteTilesByTag((*sChooseBoxMenu).tileTag);
    FreeSpriteTilesByTag((*sChooseBoxMenu).tileTag + 1);
}
pub(crate) unsafe extern "C" fn CreateChooseBoxMenuSprites(curBox: u8) {
    ChooseBoxMenu_CreateSprites(curBox);
}
pub(crate) unsafe extern "C" fn DestroyChooseBoxMenuSprites() {
    ChooseBoxMenu_DestroySprites();
}
pub(crate) unsafe extern "C" fn HandleChooseBoxMenuInput() -> u8 {
    if gMain.newKeys as i32 & B_BUTTON != 0 {
        PlaySE(SE_SELECT);
        return BOXID_CANCELED;
    }
    if gMain.newKeys as i32 & A_BUTTON != 0 {
        PlaySE(SE_SELECT);
        return (*sChooseBoxMenu).curBox;
    }
    if gMain.newKeys as i32 & DPAD_LEFT != 0 {
        PlaySE(SE_SELECT);
        ChooseBoxMenu_MoveLeft();
    } else if gMain.newKeys as i32 & DPAD_RIGHT != 0 {
        PlaySE(SE_SELECT);
        ChooseBoxMenu_MoveRight();
    }
    return BOXID_NONE_CHOSEN;
}
pub(crate) unsafe extern "C" fn ChooseBoxMenu_CreateSprites(curBox: u8) {
    let mut i: u16 = 0;
    let mut spriteId: u8 = 0;
    let mut template: SpriteTemplate = zeroed();
    let mut oamData: OamData = zeroed();
    oamData.set_size(3);
    oamData.set_paletteNum(1);
    template = {
        let mut lit1: SpriteTemplate = zeroed();
        lit1.tileTag = 0;
        lit1.paletteTag = 0;
        lit1.oam = &raw mut oamData;
        lit1.anims = gDummySpriteAnimTable.as_ptr().cast_mut();
        lit1.images = null_mut();
        lit1.affineAnims = gDummySpriteAffineAnimTable.as_ptr().cast_mut();
        lit1.callback = Some(SpriteCallbackDummy);
        lit1
    };
    (*sChooseBoxMenu).curBox = curBox;
    template.tileTag = (*sChooseBoxMenu).tileTag;
    template.paletteTag = (*sChooseBoxMenu).paletteTag;
    spriteId = CreateSprite(&raw mut template, 160, 96, 0);
    (*sChooseBoxMenu).menuSprite = &raw mut gSprites[spriteId];
    oamData.set_shape(2);
    oamData.set_size(1);
    template.tileTag = (*sChooseBoxMenu).tileTag + 1;
    template.anims = sAnims_ChooseBoxMenu.as_ptr().cast_mut();
    i = 0;
    while i < 4 {
        let mut anim: u16 = 0;
        spriteId = CreateSprite(&raw mut template, 124, 80, (*sChooseBoxMenu).subpriority);
        (*sChooseBoxMenu).menuSideSprites[i] = &raw mut gSprites[spriteId];
        anim = 0;
        if i as i32 & 2 != 0 {
            (*(*sChooseBoxMenu).menuSideSprites[i]).x = 196;
            anim = 2;
        }
        if i as i32 & 1 != 0 {
            (*(*sChooseBoxMenu).menuSideSprites[i]).y = 112;
            (*(*sChooseBoxMenu).menuSideSprites[i]).oam.set_size(0);
            anim += 1;
        }
        StartSpriteAnim((*sChooseBoxMenu).menuSideSprites[i], anim as u8);
        i += 1;
    }
    i = 0;
    while i < 2 {
        (*sChooseBoxMenu).arrowSprites[i] =
            CreateChooseBoxArrows(72 * i + 124, 88, i as u8, 0, (*sChooseBoxMenu).subpriority);
        if !(*sChooseBoxMenu).arrowSprites[i].is_null() {
            (*(*sChooseBoxMenu).arrowSprites[i]).data[0] = (if i == 0 { -1 } else { 1 }) as i16;
            (*(*sChooseBoxMenu).arrowSprites[i]).callback = Some(SpriteCB_ChooseBoxArrow);
        }
        i += 1;
    }
    ChooseBoxMenu_PrintInfo();
}
pub(crate) unsafe extern "C" fn ChooseBoxMenu_DestroySprites() {
    let mut i: u16 = 0;
    if !(*sChooseBoxMenu).menuSprite.is_null() {
        DestroySprite((*sChooseBoxMenu).menuSprite);
        (*sChooseBoxMenu).menuSprite = null_mut();
    }
    i = 0;
    while i < 4 {
        if !(*sChooseBoxMenu).menuSideSprites[i].is_null() {
            DestroySprite((*sChooseBoxMenu).menuSideSprites[i]);
            (*sChooseBoxMenu).menuSideSprites[i] = null_mut();
        }
        i += 1;
    }
    i = 0;
    while i < 2 {
        if !(*sChooseBoxMenu).arrowSprites[i].is_null() {
            DestroySprite((*sChooseBoxMenu).arrowSprites[i]);
        }
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn ChooseBoxMenu_MoveRight() {
    if ({
        (*sChooseBoxMenu).curBox += 1;
        (*sChooseBoxMenu).curBox
    }) >= TOTAL_BOXES_COUNT
    {
        (*sChooseBoxMenu).curBox = 0;
    }
    ChooseBoxMenu_PrintInfo();
}
pub(crate) unsafe extern "C" fn ChooseBoxMenu_MoveLeft() {
    (*sChooseBoxMenu).curBox = (if (*sChooseBoxMenu).curBox == 0 {
        13
    } else {
        (*sChooseBoxMenu).curBox as i32 - 1
    }) as u8;
    ChooseBoxMenu_PrintInfo();
}
pub(crate) unsafe extern "C" fn ChooseBoxMenu_PrintInfo() {
    let mut numBoxMonsText: CArray<u8, 16> = zeroed();
    let mut template: WindowTemplate = zeroed();
    let mut windowId: u8 = 0;
    let mut boxName: *mut u8 = GetBoxNamePtr((*sChooseBoxMenu).curBox);
    let mut numInBox: u8 = CountMonsInBox((*sChooseBoxMenu).curBox);
    let mut winTileData: u32 = 0;
    let mut center: i32 = 0;
    memset(&raw mut template as *mut u8, 0, 8);
    template.width = 8;
    template.height = 4;
    windowId = AddWindow(&raw mut template) as u8;
    FillWindowPixelBuffer(windowId, 68);
    center = GetStringCenterAlignXOffset(FONT_NORMAL as i32, boxName, 64);
    AddTextPrinterParameterized3(
        windowId,
        FONT_NORMAL,
        center as u8,
        1,
        sChooseBoxMenu_TextColors.as_ptr().cast_mut(),
        TEXT_SKIP_DRAW as i8,
        boxName,
    );
    ConvertIntToDecimalStringN(
        numBoxMonsText.as_mut_ptr(),
        numInBox as i32,
        STR_CONV_MODE_RIGHT_ALIGN,
        2,
    );
    StringAppend(
        numBoxMonsText.as_mut_ptr(),
        sText_OutOf30.as_ptr().cast_mut(),
    );
    center = GetStringCenterAlignXOffset(FONT_NORMAL as i32, numBoxMonsText.as_mut_ptr(), 64);
    AddTextPrinterParameterized3(
        windowId,
        FONT_NORMAL,
        center as u8,
        17,
        sChooseBoxMenu_TextColors.as_ptr().cast_mut(),
        TEXT_SKIP_DRAW as i8,
        numBoxMonsText.as_mut_ptr(),
    );
    winTileData = GetWindowAttribute(windowId, WINDOW_TILE_DATA);
    CpuSet(
        winTileData as usize as *mut c_void,
        ((OBJ_VRAM0 as usize as *mut c_void as *mut u8).at(256) as *mut c_void as *mut u8)
            .at(GetSpriteTileStartByTag((*sChooseBoxMenu).tileTag) as i32 * 32)
            as *mut c_void,
        0x4000100,
    );
    RemoveWindow(windowId);
}
pub(crate) unsafe extern "C" fn SpriteCB_ChooseBoxArrow(sprite: *mut Sprite) {
    if ({
        (*sprite).data[1] += 1;
        (*sprite).data[1]
    }) > 3
    {
        (*sprite).data[1] = 0;
        (*sprite).x2 += (*sprite).data[0];
        if ({
            (*sprite).data[2] += 1;
            (*sprite).data[2]
        }) > 5
        {
            (*sprite).data[2] = 0;
            (*sprite).x2 = 0;
        }
    }
}
pub(crate) unsafe extern "C" fn VBlankCB_PokeStorage() {
    LoadOam();
    ProcessSpriteCopyRequests();
    UnkUtil_Run();
    TransferPlttBuffer();
    SetGpuReg(REG_OFFSET_BG2HOFS, (*sStorage).bg2_X);
}
pub(crate) unsafe extern "C" fn CB2_PokeStorage() {
    RunTasks();
    DoScheduledBgTilemapCopiesToVram();
    ScrollBackground();
    UpdateCloseBoxButtonFlash();
    AnimateSprites();
    BuildOamBuffer();
}
pub(crate) unsafe extern "C" fn EnterPokeStorage(boxOption: u8) {
    ResetTasks();
    sCurrentBoxOption = boxOption;
    sStorage = Alloc(25284) as *mut PokemonStorageSystemData;
    if sStorage.is_null() {
        SetMainCallback2(Some(CB2_ExitPokeStorage));
    } else {
        (*sStorage).boxOption = boxOption;
        (*sStorage).isReopening = FALSE;
        sMovingItemId = ITEM_NONE;
        (*sStorage).state = 0;
        (*sStorage).taskId = CreateTask(Some(Task_InitPokeStorage), 3);
        sLastUsedBox = StorageGetCurrentBox();
        SetMainCallback2(Some(CB2_PokeStorage));
    }
}
pub(crate) unsafe extern "C" fn CB2_ReturnToPokeStorage() {
    ResetTasks();
    sStorage = Alloc(25284) as *mut PokemonStorageSystemData;
    if sStorage.is_null() {
        SetMainCallback2(Some(CB2_ExitPokeStorage));
    } else {
        (*sStorage).boxOption = sCurrentBoxOption;
        (*sStorage).isReopening = TRUE;
        (*sStorage).state = 0;
        (*sStorage).taskId = CreateTask(Some(Task_InitPokeStorage), 3);
        SetMainCallback2(Some(CB2_PokeStorage));
    }
}
pub(crate) unsafe extern "C" fn ResetAllBgCoords() {
    SetGpuReg(REG_OFFSET_BG0HOFS, 0);
    SetGpuReg(REG_OFFSET_BG0VOFS, 0);
    SetGpuReg(REG_OFFSET_BG1HOFS, 0);
    SetGpuReg(REG_OFFSET_BG1VOFS, 0);
    SetGpuReg(REG_OFFSET_BG2HOFS, 0);
    SetGpuReg(REG_OFFSET_BG2VOFS, 0);
    SetGpuReg(REG_OFFSET_BG3HOFS, 0);
    SetGpuReg(REG_OFFSET_BG3VOFS, 0);
}
pub(crate) unsafe extern "C" fn ResetForPokeStorage() {
    ResetPaletteFade();
    ResetSpriteData();
    FreeSpriteTileRanges();
    FreeAllSpritePalettes();
    ClearDma3Requests();
    gReservedSpriteTileCount = 0x280;
    UnkUtil_Init(
        &raw mut (*sStorage).unkUtil,
        (*sStorage).unkUtilData.as_mut_ptr(),
        8,
    );
    gKeyRepeatStartDelay = 20;
    ClearScheduledBgCopiesToVram();
    TilemapUtil_Init(TILEMAPID_COUNT);
    TilemapUtil_SetMap(
        TILEMAPID_PKMN_DATA,
        1,
        sPkmnData_Tilemap.as_ptr().cast_mut() as *mut c_void,
        8,
        4,
    );
    TilemapUtil_SetPos(TILEMAPID_PKMN_DATA, 1, 0);
    (*sStorage).closeBoxFlashing = FALSE;
}
pub(crate) unsafe extern "C" fn InitStartingPosData() {
    ClearSavedCursorPos();
    sInPartyMenu = ((*sStorage).boxOption == OPTION_DEPOSIT) as u8;
    sDepositBoxId = 0;
}
pub(crate) unsafe extern "C" fn SetMonIconTransparency() {
    if (*sStorage).boxOption == OPTION_MOVE_ITEMS {
        SetGpuReg(REG_OFFSET_BLDCNT, BLDCNT_TGT2_ALL);
        SetGpuReg(REG_OFFSET_BLDALPHA, 2823);
    }
    SetGpuReg(REG_OFFSET_DISPCNT, 8000);
}
pub(crate) unsafe extern "C" fn SetPokeStorageTask(newFunc: Option<unsafe extern "C" fn(u8)>) {
    gTasks[(*sStorage).taskId].func = newFunc;
    (*sStorage).state = 0;
}
pub(crate) unsafe extern "C" fn Task_InitPokeStorage(taskId: u8) {
    match (*sStorage).state {
        0 => {
            SetVBlankCallback(None);
            SetGpuReg(0x0, 0);
            ResetForPokeStorage();
            if (*sStorage).isReopening != 0 {
                match sWhichToReshow {
                    1 => {
                        LoadSavedMovingMon();
                    }
                    0 => {
                        SetSelectionAfterSummaryScreen();
                    }
                    2 => {
                        GiveChosenBagItem();
                    }
                    _ => {}
                }
            }
            LoadPokeStorageMenuGfx();
            LoadWaveformSpritePalette();
        }
        1 => {
            if InitPokeStorageWindows() == 0 {
                SetPokeStorageTask(Some(Task_ChangeScreen));
                return;
            }
        }
        2 => {
            PutWindowTilemap(WIN_DISPLAY_INFO);
            ClearWindowTilemap(WIN_MESSAGE);
            {
                {
                    let mut tmp: u32 = 0;
                    volatile_write(&raw mut tmp, 0);
                    CpuSet(
                        &raw mut tmp as *mut c_void,
                        VRAM as usize as *mut c_void,
                        0x5000080,
                    );
                }
            }
            LoadUserWindowBorderGfx(WIN_MESSAGE, 0xB, 224);
        }
        3 => {
            ResetAllBgCoords();
            if (*sStorage).isReopening == 0 {
                InitStartingPosData();
            }
        }
        4 => {
            InitMonIconFields();
            if (*sStorage).isReopening == 0 {
                InitCursor();
            } else {
                InitCursorOnReopen();
            }
        }
        5 => {
            if MultiMove_Init() == 0 {
                SetPokeStorageTask(Some(Task_ChangeScreen));
                return;
            } else {
                SetScrollingBackground();
                InitPokeStorageBg0();
            }
        }
        6 => {
            InitPalettesAndSprites();
        }
        7 => {
            InitSupplementalTilemaps();
        }
        8 => {
            CreateInitBoxTask(StorageGetCurrentBox());
        }
        9 => {
            if IsInitBoxActive() != 0 {
                return;
            }
            if (*sStorage).boxOption != OPTION_MOVE_ITEMS {
                (*sStorage).markMenu.baseTileTag = GFXTAG_MARKING_MENU;
                (*sStorage).markMenu.basePaletteTag = PALTAG_MARKING_MENU;
                InitMonMarkingsMenu(&raw mut (*sStorage).markMenu);
                BufferMonMarkingsMenuTiles();
            } else {
                CreateItemIconSprites();
                InitCursorItemIcon();
            }
        }
        10 => {
            SetMonIconTransparency();
            if (*sStorage).isReopening == 0 {
                BlendPalettes(PALETTES_ALL, 16, 0);
                SetPokeStorageTask(Some(Task_ShowPokeStorage));
            } else {
                BlendPalettes(PALETTES_ALL, 16, 0);
                SetPokeStorageTask(Some(Task_ReshowPokeStorage));
            }
            SetVBlankCallback(Some(VBlankCB_PokeStorage));
            return;
        }
        _ => {
            return;
        }
    }
    (*sStorage).state += 1;
}
pub(crate) unsafe extern "C" fn Task_ShowPokeStorage(taskId: u8) {
    match (*sStorage).state {
        0 => {
            PlaySE(SE_PC_LOGIN);
            ComputerScreenOpenEffect(20, 0, 1);
            (*sStorage).state += 1;
        }
        1 => {
            if IsComputerScreenOpenEffectActive() == 0 {
                SetPokeStorageTask(Some(Task_PokeStorageMain));
            }
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn Task_ReshowPokeStorage(taskId: u8) {
    match (*sStorage).state {
        0 => {
            BeginNormalPaletteFade(PALETTES_ALL, -1, 0x10, 0, 0);
            (*sStorage).state += 1;
        }
        1 => {
            if UpdatePaletteFade() == 0 {
                if sWhichToReshow == 2 && gSpecialVar_ItemId != ITEM_NONE {
                    PrintMessage(MSG_ITEM_IS_HELD);
                    (*sStorage).state += 1;
                } else {
                    SetPokeStorageTask(Some(Task_PokeStorageMain));
                }
            }
        }
        2 => {
            if IsDma3ManagerBusyWithBgCopy() == 0 && gMain.newKeys as i32 & 3 != 0 {
                ClearBottomWindow();
                (*sStorage).state += 1;
            }
        }
        3 => {
            if IsDma3ManagerBusyWithBgCopy() == 0 {
                SetPokeStorageTask(Some(Task_PokeStorageMain));
            }
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn Task_PokeStorageMain(taskId: u8) {
    match (*sStorage).state {
        MSTATE_HANDLE_INPUT => match HandleInput() {
            INPUT_MOVE_CURSOR => {
                PlaySE(SE_SELECT);
                (*sStorage).state = MSTATE_MOVE_CURSOR;
            }
            INPUT_SHOW_PARTY => {
                if (*sStorage).boxOption != OPTION_MOVE_MONS
                    && (*sStorage).boxOption != OPTION_MOVE_ITEMS
                {
                    PrintMessage(MSG_WHICH_ONE_WILL_TAKE);
                    (*sStorage).state = MSTATE_WAIT_MSG;
                } else {
                    ClearSavedCursorPos();
                    SetPokeStorageTask(Some(Task_ShowPartyPokemon));
                }
            }
            INPUT_HIDE_PARTY => {
                if (*sStorage).boxOption == OPTION_MOVE_MONS {
                    if IsMonBeingMoved() != 0 && ItemIsMail((*sStorage).displayMonItemId) != 0 {
                        (*sStorage).state = MSTATE_ERROR_HAS_MAIL;
                    } else {
                        SetPokeStorageTask(Some(Task_HidePartyPokemon));
                    }
                } else if (*sStorage).boxOption == OPTION_MOVE_ITEMS {
                    SetPokeStorageTask(Some(Task_HidePartyPokemon));
                }
            }
            INPUT_CLOSE_BOX => {
                SetPokeStorageTask(Some(Task_OnCloseBoxPressed));
            }
            INPUT_PRESSED_B => {
                SetPokeStorageTask(Some(Task_OnBPressed));
            }
            INPUT_BOX_OPTIONS => {
                PlaySE(SE_SELECT);
                SetPokeStorageTask(Some(Task_HandleBoxOptions));
            }
            INPUT_IN_MENU => {
                SetPokeStorageTask(Some(Task_OnSelectedMon));
            }
            INPUT_SCROLL_RIGHT => {
                PlaySE(SE_SELECT);
                (*sStorage).newCurrBoxId = StorageGetCurrentBox() as i16 + 1;
                if (*sStorage).newCurrBoxId >= TOTAL_BOXES_COUNT as i16 {
                    (*sStorage).newCurrBoxId = 0;
                }
                if (*sStorage).boxOption != OPTION_MOVE_ITEMS {
                    SetUpScrollToBox((*sStorage).newCurrBoxId as u8);
                    (*sStorage).state = MSTATE_SCROLL_BOX;
                } else {
                    TryHideItemAtCursor();
                    (*sStorage).state = MSTATE_SCROLL_BOX_ITEM;
                }
            }
            INPUT_SCROLL_LEFT => {
                PlaySE(SE_SELECT);
                (*sStorage).newCurrBoxId = StorageGetCurrentBox() as i16 - 1;
                if (*sStorage).newCurrBoxId < 0 {
                    (*sStorage).newCurrBoxId = 13;
                }
                if (*sStorage).boxOption != OPTION_MOVE_ITEMS {
                    SetUpScrollToBox((*sStorage).newCurrBoxId as u8);
                    (*sStorage).state = MSTATE_SCROLL_BOX;
                } else {
                    TryHideItemAtCursor();
                    (*sStorage).state = MSTATE_SCROLL_BOX_ITEM;
                }
            }
            INPUT_DEPOSIT => {
                if IsRemovingLastPartyMon() == 0 {
                    if ItemIsMail((*sStorage).displayMonItemId) != 0 {
                        (*sStorage).state = MSTATE_ERROR_HAS_MAIL;
                    } else {
                        PlaySE(SE_SELECT);
                        SetPokeStorageTask(Some(Task_DepositMenu));
                    }
                } else {
                    (*sStorage).state = MSTATE_ERROR_LAST_PARTY_MON;
                }
            }
            INPUT_MOVE_MON => {
                if IsRemovingLastPartyMon() != 0 {
                    (*sStorage).state = MSTATE_ERROR_LAST_PARTY_MON;
                } else {
                    PlaySE(SE_SELECT);
                    SetPokeStorageTask(Some(Task_MoveMon));
                }
            }
            INPUT_SHIFT_MON => {
                if CanShiftMon() == 0 {
                    (*sStorage).state = MSTATE_ERROR_LAST_PARTY_MON;
                } else {
                    PlaySE(SE_SELECT);
                    SetPokeStorageTask(Some(Task_ShiftMon));
                }
            }
            INPUT_WITHDRAW => {
                PlaySE(SE_SELECT);
                SetPokeStorageTask(Some(Task_WithdrawMon));
            }
            INPUT_PLACE_MON => {
                PlaySE(SE_SELECT);
                SetPokeStorageTask(Some(Task_PlaceMon));
            }
            INPUT_TAKE_ITEM => {
                PlaySE(SE_SELECT);
                SetPokeStorageTask(Some(Task_TakeItemForMoving));
            }
            INPUT_GIVE_ITEM => {
                PlaySE(SE_SELECT);
                SetPokeStorageTask(Some(Task_GiveMovingItemToMon));
            }
            INPUT_SWITCH_ITEMS => {
                PlaySE(SE_SELECT);
                SetPokeStorageTask(Some(Task_SwitchSelectedItem));
            }
            INPUT_MULTIMOVE_START => {
                PlaySE(SE_SELECT);
                MultiMove_SetFunction(MULTIMOVE_START);
                (*sStorage).state = MSTATE_MULTIMOVE_RUN;
            }
            INPUT_MULTIMOVE_SINGLE => {
                MultiMove_SetFunction(MULTIMOVE_CANCEL);
                (*sStorage).state = MSTATE_MULTIMOVE_RUN_CANCEL;
            }
            INPUT_MULTIMOVE_CHANGE_SELECTION => {
                PlaySE(SE_SELECT);
                MultiMove_SetFunction(MULTIMOVE_CHANGE_SELECTION);
                (*sStorage).state = MSTATE_MULTIMOVE_RUN_MOVED;
            }
            INPUT_MULTIMOVE_GRAB_SELECTION => {
                MultiMove_SetFunction(MULTIMOVE_GRAB_SELECTION);
                (*sStorage).state = MSTATE_MULTIMOVE_RUN;
            }
            INPUT_MULTIMOVE_MOVE_MONS => {
                PlaySE(SE_SELECT);
                MultiMove_SetFunction(MULTIMOVE_MOVE_MONS);
                (*sStorage).state = MSTATE_MULTIMOVE_RUN_MOVED;
            }
            INPUT_MULTIMOVE_PLACE_MONS => {
                PlaySE(SE_SELECT);
                MultiMove_SetFunction(MULTIMOVE_PLACE_MONS);
                (*sStorage).state = MSTATE_MULTIMOVE_RUN;
            }
            INPUT_MULTIMOVE_UNABLE => {
                PlaySE(SE_FAILURE);
            }
            _ => {}
        },
        MSTATE_MOVE_CURSOR => {
            if UpdateCursorPos() == 0 {
                if IsCursorOnCloseBox() != 0 {
                    StartFlashingCloseBoxButton();
                } else {
                    StopFlashingCloseBoxButton();
                }
                if (*sStorage).setMosaic != 0 {
                    StartDisplayMonMosaicEffect();
                }
                (*sStorage).state = MSTATE_HANDLE_INPUT;
            }
        }
        MSTATE_SCROLL_BOX => {
            if ScrollToBox() == 0 {
                SetCurrentBox((*sStorage).newCurrBoxId as u8);
                if sInPartyMenu == 0 && IsMonBeingMoved() == 0 {
                    RefreshDisplayMon();
                    StartDisplayMonMosaicEffect();
                }
                if (*sStorage).boxOption == OPTION_MOVE_ITEMS {
                    TryShowItemAtCursor();
                    (*sStorage).state = MSTATE_WAIT_ITEM_ANIM;
                } else {
                    (*sStorage).state = MSTATE_HANDLE_INPUT;
                }
            }
        }
        MSTATE_WAIT_MSG => {
            if gMain.newKeys as i32 & 243 != 0 {
                ClearBottomWindow();
                (*sStorage).state = MSTATE_HANDLE_INPUT;
            }
        }
        MSTATE_ERROR_LAST_PARTY_MON => {
            PlaySE(SE_FAILURE);
            PrintMessage(MSG_LAST_POKE);
            (*sStorage).state = MSTATE_WAIT_ERROR_MSG;
        }
        MSTATE_ERROR_HAS_MAIL => {
            PlaySE(SE_FAILURE);
            PrintMessage(MSG_PLEASE_REMOVE_MAIL);
            (*sStorage).state = MSTATE_WAIT_ERROR_MSG;
        }
        MSTATE_WAIT_ERROR_MSG => {
            if gMain.newKeys as i32 & 243 != 0 {
                ClearBottomWindow();
                SetPokeStorageTask(Some(Task_PokeStorageMain));
            }
        }
        MSTATE_MULTIMOVE_RUN => {
            if MultiMove_RunFunction() == 0 {
                (*sStorage).state = MSTATE_HANDLE_INPUT;
            }
        }
        MSTATE_MULTIMOVE_RUN_CANCEL => {
            if MultiMove_RunFunction() == 0 {
                SetPokeStorageTask(Some(Task_MoveMon));
            }
        }
        MSTATE_MULTIMOVE_RUN_MOVED => {
            if MultiMove_RunFunction() == 0 {
                if (*sStorage).setMosaic != 0 {
                    StartDisplayMonMosaicEffect();
                }
                (*sStorage).state = MSTATE_HANDLE_INPUT;
            }
        }
        MSTATE_SCROLL_BOX_ITEM => {
            if IsItemIconAnimActive() == 0 {
                SetUpScrollToBox((*sStorage).newCurrBoxId as u8);
                (*sStorage).state = MSTATE_SCROLL_BOX;
            }
        }
        MSTATE_WAIT_ITEM_ANIM => {
            if IsItemIconAnimActive() == 0 {
                (*sStorage).state = MSTATE_HANDLE_INPUT;
            }
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn Task_ShowPartyPokemon(taskId: u8) {
    match (*sStorage).state {
        0 => {
            SetUpDoShowPartyMenu();
            (*sStorage).state += 1;
        }
        1 => {
            if DoShowPartyMenu() == 0 {
                SetPokeStorageTask(Some(Task_PokeStorageMain));
            }
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn Task_HidePartyPokemon(taskId: u8) {
    match (*sStorage).state {
        0 => {
            PlaySE(SE_SELECT);
            SetUpHidePartyMenu();
            (*sStorage).state += 1;
        }
        1 => {
            if HidePartyMenu() == 0 {
                SetCursorBoxPosition(GetSavedCursorPos());
                (*sStorage).state += 1;
            }
        }
        2 => {
            if UpdateCursorPos() == 0 {
                if (*sStorage).setMosaic != 0 {
                    StartDisplayMonMosaicEffect();
                }
                SetPokeStorageTask(Some(Task_PokeStorageMain));
            }
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn Task_OnSelectedMon(taskId: u8) {
    match (*sStorage).state {
        0 => {
            if IsDisplayMosaicActive() == 0 {
                PlaySE(SE_SELECT);
                if (*sStorage).boxOption != OPTION_MOVE_ITEMS {
                    PrintMessage(MSG_IS_SELECTED);
                } else if IsMovingItem() != 0 || (*sStorage).displayMonItemId != ITEM_NONE {
                    PrintMessage(MSG_IS_SELECTED2);
                } else {
                    PrintMessage(MSG_GIVE_TO_MON);
                }
                AddMenu();
                (*sStorage).state = 1;
            }
        }
        1 => {
            if IsMenuLoading() == 0 {
                (*sStorage).state = 2;
            }
        }
        2 => match HandleMenuInput() {
            -1 | 0 => {
                ClearBottomWindow();
                SetPokeStorageTask(Some(Task_PokeStorageMain));
            }
            3 => {
                if IsRemovingLastPartyMon() != 0 {
                    (*sStorage).state = 3;
                } else {
                    PlaySE(SE_SELECT);
                    ClearBottomWindow();
                    SetPokeStorageTask(Some(Task_MoveMon));
                }
            }
            5 => {
                PlaySE(SE_SELECT);
                ClearBottomWindow();
                SetPokeStorageTask(Some(Task_PlaceMon));
            }
            4 => {
                if CanShiftMon() == 0 {
                    (*sStorage).state = 3;
                } else {
                    PlaySE(SE_SELECT);
                    ClearBottomWindow();
                    SetPokeStorageTask(Some(Task_ShiftMon));
                }
            }
            2 => {
                PlaySE(SE_SELECT);
                ClearBottomWindow();
                SetPokeStorageTask(Some(Task_WithdrawMon));
            }
            1 => {
                if IsRemovingLastPartyMon() != 0 {
                    (*sStorage).state = 3;
                } else if ItemIsMail((*sStorage).displayMonItemId) != 0 {
                    (*sStorage).state = 4;
                } else {
                    PlaySE(SE_SELECT);
                    ClearBottomWindow();
                    SetPokeStorageTask(Some(Task_DepositMenu));
                }
            }
            MENU_RELEASE => {
                if IsRemovingLastPartyMon() != 0 {
                    (*sStorage).state = 3;
                } else if (*sStorage).displayMonIsEgg != 0 {
                    (*sStorage).state = 5;
                } else if ItemIsMail((*sStorage).displayMonItemId) != 0 {
                    (*sStorage).state = 4;
                } else {
                    PlaySE(SE_SELECT);
                    SetPokeStorageTask(Some(Task_ReleaseMon));
                }
            }
            MENU_SUMMARY => {
                PlaySE(SE_SELECT);
                SetPokeStorageTask(Some(Task_ShowMonSummary));
            }
            MENU_MARK => {
                PlaySE(SE_SELECT);
                SetPokeStorageTask(Some(Task_ShowMarkMenu));
            }
            12 => {
                PlaySE(SE_SELECT);
                SetPokeStorageTask(Some(Task_TakeItemForMoving));
            }
            13 => {
                PlaySE(SE_SELECT);
                SetPokeStorageTask(Some(Task_GiveMovingItemToMon));
            }
            MENU_BAG => {
                SetPokeStorageTask(Some(Task_ItemToBag));
            }
            15 => {
                PlaySE(SE_SELECT);
                SetPokeStorageTask(Some(Task_SwitchSelectedItem));
            }
            MENU_GIVE_2 => {
                PlaySE(SE_SELECT);
                SetPokeStorageTask(Some(Task_GiveItemFromBag));
            }
            MENU_INFO => {
                SetPokeStorageTask(Some(Task_ShowItemInfo));
            }
            _ => {}
        },
        3 => {
            PlaySE(SE_FAILURE);
            PrintMessage(MSG_LAST_POKE);
            (*sStorage).state = 6;
        }
        5 => {
            PlaySE(SE_FAILURE);
            PrintMessage(MSG_CANT_RELEASE_EGG);
            (*sStorage).state = 6;
        }
        4 => {
            PlaySE(SE_FAILURE);
            PrintMessage(MSG_PLEASE_REMOVE_MAIL);
            (*sStorage).state = 6;
        }
        6 => {
            if gMain.newKeys as i32 & 243 != 0 {
                ClearBottomWindow();
                SetPokeStorageTask(Some(Task_PokeStorageMain));
            }
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn Task_MoveMon(taskId: u8) {
    match (*sStorage).state {
        0 => {
            InitMonPlaceChange(CHANGE_GRAB);
            (*sStorage).state += 1;
        }
        1 => {
            if DoMonPlaceChange() == 0 {
                if sInPartyMenu != 0 {
                    SetPokeStorageTask(Some(Task_HandleMovingMonFromParty));
                } else {
                    SetPokeStorageTask(Some(Task_PokeStorageMain));
                }
            }
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn Task_PlaceMon(taskId: u8) {
    match (*sStorage).state {
        0 => {
            InitMonPlaceChange(CHANGE_PLACE);
            (*sStorage).state += 1;
        }
        1 => {
            if DoMonPlaceChange() == 0 {
                if sInPartyMenu != 0 {
                    SetPokeStorageTask(Some(Task_HandleMovingMonFromParty));
                } else {
                    SetPokeStorageTask(Some(Task_PokeStorageMain));
                }
            }
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn Task_ShiftMon(taskId: u8) {
    match (*sStorage).state {
        0 => {
            InitMonPlaceChange(CHANGE_SHIFT);
            (*sStorage).state += 1;
        }
        1 => {
            if DoMonPlaceChange() == 0 {
                StartDisplayMonMosaicEffect();
                SetPokeStorageTask(Some(Task_PokeStorageMain));
            }
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn Task_WithdrawMon(taskId: u8) {
    match (*sStorage).state {
        0 => {
            if CalculatePlayerPartyCount() == PARTY_SIZE as u8 {
                PrintMessage(MSG_PARTY_FULL);
                (*sStorage).state = 1;
            } else {
                SaveCursorPos();
                InitMonPlaceChange(CHANGE_GRAB);
                (*sStorage).state = 2;
            }
        }
        1 => {
            if gMain.newKeys as i32 & 243 != 0 {
                ClearBottomWindow();
                SetPokeStorageTask(Some(Task_PokeStorageMain));
            }
        }
        2 => {
            if DoMonPlaceChange() == 0 {
                SetMovingMonPriority(1);
                SetUpDoShowPartyMenu();
                (*sStorage).state += 1;
            }
        }
        3 => {
            if DoShowPartyMenu() == 0 {
                InitMonPlaceChange(CHANGE_PLACE);
                (*sStorage).state += 1;
            }
        }
        4 => {
            if DoMonPlaceChange() == 0 {
                UpdatePartySlotColors();
                (*sStorage).state += 1;
            }
        }
        5 => {
            SetPokeStorageTask(Some(Task_HidePartyPokemon));
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn Task_DepositMenu(taskId: u8) {
    let mut boxId: u8 = 0;
    match (*sStorage).state {
        0 => {
            PrintMessage(MSG_DEPOSIT_IN_WHICH_BOX);
            LoadChooseBoxMenuGfx(
                &raw mut (*sStorage).chooseBoxMenu,
                GFXTAG_CHOOSE_BOX_MENU,
                PALTAG_MISC_1,
                3,
                FALSE as u32,
            );
            CreateChooseBoxMenuSprites(sDepositBoxId);
            (*sStorage).state += 1;
        }
        1 => {
            boxId = HandleChooseBoxMenuInput();
            match boxId {
                BOXID_NONE_CHOSEN => {}
                BOXID_CANCELED => {
                    ClearBottomWindow();
                    DestroyChooseBoxMenuSprites();
                    FreeChooseBoxMenu();
                    SetPokeStorageTask(Some(Task_PokeStorageMain));
                }
                _ => {
                    if TryStorePartyMonInBox(boxId) != 0 {
                        sDepositBoxId = boxId;
                        ClearBottomWindow();
                        DestroyChooseBoxMenuSprites();
                        FreeChooseBoxMenu();
                        (*sStorage).state = 2;
                    } else {
                        PrintMessage(MSG_BOX_IS_FULL);
                        (*sStorage).state = 4;
                    }
                }
            }
        }
        2 => {
            CompactPartySlots();
            CompactPartySprites();
            (*sStorage).state += 1;
        }
        3 => {
            if GetNumPartySpritesCompacting() == 0 {
                ResetSelectionAfterDeposit();
                StartDisplayMonMosaicEffect();
                UpdatePartySlotColors();
                SetPokeStorageTask(Some(Task_PokeStorageMain));
            }
        }
        4 => {
            if gMain.newKeys as i32 & 243 != 0 {
                PrintMessage(MSG_DEPOSIT_IN_WHICH_BOX);
                (*sStorage).state = 1;
            }
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn Task_ReleaseMon(taskId: u8) {
    'l1: {
        let sw1: u8 = (*sStorage).state;
        let mut fall = false;
        if sw1 == 0 {
            fall = true;
            PrintMessage(MSG_RELEASE_POKE);
            ShowYesNoWindow(1);
            (*sStorage).state += 1;
        }
        if fall || sw1 == 1 {
            fall = true;
            match Menu_ProcessInputNoWrapClearOnChoose() {
                MENU_B_PRESSED | 1 => {
                    ClearBottomWindow();
                    SetPokeStorageTask(Some(Task_PokeStorageMain));
                }
                0 => {
                    ClearBottomWindow();
                    InitCanReleaseMonVars();
                    InitReleaseMon();
                    (*sStorage).state += 1;
                }
                _ => {}
            }
            break 'l1;
        }
        if sw1 == 2 {
            fall = true;
            RunCanReleaseMon();
            if TryHideReleaseMon() == 0 {
                loop {
                    let mut canRelease: i8 = RunCanReleaseMon();
                    if canRelease == TRUE as i8 {
                        (*sStorage).state += 1;
                        break;
                    } else if canRelease == 0 {
                        (*sStorage).state = 8;
                        break;
                    }
                }
            }
            break 'l1;
        }
        if sw1 == 3 {
            fall = true;
            ReleaseMon();
            RefreshDisplayMonData();
            PrintMessage(MSG_WAS_RELEASED);
            (*sStorage).state += 1;
            break 'l1;
        }
        if sw1 == 4 {
            fall = true;
            if gMain.newKeys as i32 & 243 != 0 {
                PrintMessage(MSG_BYE_BYE);
                (*sStorage).state += 1;
            }
            break 'l1;
        }
        if sw1 == 5 {
            fall = true;
            if gMain.newKeys as i32 & 243 != 0 {
                ClearBottomWindow();
                if sInPartyMenu != 0 {
                    CompactPartySlots();
                    CompactPartySprites();
                    (*sStorage).state += 1;
                } else {
                    (*sStorage).state = 7;
                }
            }
            break 'l1;
        }
        if sw1 == 6 {
            fall = true;
            if GetNumPartySpritesCompacting() == 0 {
                RefreshDisplayMon();
                StartDisplayMonMosaicEffect();
                UpdatePartySlotColors();
                (*sStorage).state += 1;
            }
            break 'l1;
        }
        if sw1 == 7 {
            fall = true;
            SetPokeStorageTask(Some(Task_PokeStorageMain));
            break 'l1;
        }
        if sw1 == 8 {
            fall = true;
            PrintMessage(MSG_WAS_RELEASED);
            (*sStorage).state += 1;
            break 'l1;
        }
        if sw1 == 9 {
            fall = true;
            if gMain.newKeys as i32 & 243 != 0 {
                PrintMessage(MSG_SURPRISE);
                (*sStorage).state += 1;
            }
            break 'l1;
        }
        if sw1 == 10 {
            fall = true;
            if gMain.newKeys as i32 & 243 != 0 {
                ClearBottomWindow();
                ReshowReleaseMon();
                (*sStorage).state += 1;
            }
            break 'l1;
        }
        if sw1 == 11 {
            fall = true;
            if ResetReleaseMonSpritePtr() == 0 {
                TrySetCursorFistAnim();
                PrintMessage(MSG_CAME_BACK);
                (*sStorage).state += 1;
            }
            break 'l1;
        }
        if sw1 == 12 {
            fall = true;
            if gMain.newKeys as i32 & 243 != 0 {
                PrintMessage(MSG_WORRIED);
                (*sStorage).state += 1;
            }
            break 'l1;
        }
        if sw1 == 13 {
            fall = true;
            if gMain.newKeys as i32 & 243 != 0 {
                ClearBottomWindow();
                SetPokeStorageTask(Some(Task_PokeStorageMain));
            }
            break 'l1;
        }
    }
}
pub(crate) unsafe extern "C" fn Task_ShowMarkMenu(taskId: u8) {
    match (*sStorage).state {
        0 => {
            PrintMessage(MSG_MARK_POKE);
            (*sStorage).markMenu.markings = (*sStorage).displayMonMarkings;
            OpenMonMarkingsMenu((*sStorage).displayMonMarkings, 0xb0, 0x10);
            (*sStorage).state += 1;
        }
        1 => {
            if HandleMonMarkingsMenuInput() == 0 {
                FreeMonMarkingsMenu();
                ClearBottomWindow();
                SetMonMarkings((*sStorage).markMenu.markings);
                RefreshDisplayMonData();
                SetPokeStorageTask(Some(Task_PokeStorageMain));
            }
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn Task_TakeItemForMoving(taskId: u8) {
    match (*sStorage).state {
        0 => {
            if ItemIsMail((*sStorage).displayMonItemId) == 0 {
                ClearBottomWindow();
                (*sStorage).state += 1;
            } else {
                SetPokeStorageTask(Some(Task_PrintCantStoreMail));
            }
        }
        1 => {
            StartCursorAnim(CURSOR_ANIM_OPEN);
            TakeItemFromMon(
                (if sInPartyMenu != 0 {
                    CURSOR_AREA_IN_PARTY as i32
                } else {
                    CURSOR_AREA_IN_BOX as i32
                }) as u8,
                GetCursorPosition(),
            );
            (*sStorage).state += 1;
        }
        2 => {
            if IsItemIconAnimActive() == 0 {
                StartCursorAnim(CURSOR_ANIM_FIST);
                ClearBottomWindow();
                RefreshDisplayMon();
                PrintDisplayMonInfo();
                (*sStorage).state += 1;
            }
        }
        3 => {
            if IsDma3ManagerBusyWithBgCopy() == 0 {
                SetPokeStorageTask(Some(Task_PokeStorageMain));
            }
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn Task_GiveMovingItemToMon(taskId: u8) {
    match (*sStorage).state {
        0 => {
            ClearBottomWindow();
            (*sStorage).state += 1;
        }
        1 => {
            StartCursorAnim(CURSOR_ANIM_OPEN);
            GiveItemToMon(
                (if sInPartyMenu != 0 {
                    CURSOR_AREA_IN_PARTY as i32
                } else {
                    CURSOR_AREA_IN_BOX as i32
                }) as u8,
                GetCursorPosition(),
            );
            (*sStorage).state += 1;
        }
        2 => {
            if IsItemIconAnimActive() == 0 {
                StartCursorAnim(CURSOR_ANIM_BOUNCE);
                RefreshDisplayMon();
                PrintDisplayMonInfo();
                PrintMessage(MSG_ITEM_IS_HELD);
                (*sStorage).state += 1;
            }
        }
        3 => {
            if gMain.newKeys as i32 & 243 != 0 {
                ClearBottomWindow();
                (*sStorage).state += 1;
            }
        }
        4 => {
            if IsDma3ManagerBusyWithBgCopy() == 0 {
                SetPokeStorageTask(Some(Task_PokeStorageMain));
            }
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn Task_ItemToBag(taskId: u8) {
    match (*sStorage).state {
        0 => {
            if AddBagItem((*sStorage).displayMonItemId, 1) == 0 {
                PlaySE(SE_FAILURE);
                PrintMessage(MSG_BAG_FULL);
                (*sStorage).state = 3;
            } else {
                PlaySE(SE_SELECT);
                MoveItemFromMonToBag(
                    (if sInPartyMenu != 0 {
                        CURSOR_AREA_IN_PARTY as i32
                    } else {
                        CURSOR_AREA_IN_BOX as i32
                    }) as u8,
                    GetCursorPosition(),
                );
                (*sStorage).state = 1;
            }
        }
        1 => {
            if IsItemIconAnimActive() == 0 {
                PrintMessage(MSG_PLACED_IN_BAG);
                (*sStorage).state = 2;
            }
        }
        2 => {
            if gMain.newKeys as i32 & 243 != 0 {
                ClearBottomWindow();
                RefreshDisplayMon();
                PrintDisplayMonInfo();
                (*sStorage).state = 4;
            }
        }
        4 => {
            if IsDma3ManagerBusyWithBgCopy() == 0 {
                SetPokeStorageTask(Some(Task_PokeStorageMain));
            }
        }
        3 => {
            if gMain.newKeys as i32 & 243 != 0 {
                ClearBottomWindow();
                SetPokeStorageTask(Some(Task_PokeStorageMain));
            }
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn Task_SwitchSelectedItem(taskId: u8) {
    match (*sStorage).state {
        0 => {
            if ItemIsMail((*sStorage).displayMonItemId) == 0 {
                ClearBottomWindow();
                (*sStorage).state += 1;
            } else {
                SetPokeStorageTask(Some(Task_PrintCantStoreMail));
            }
        }
        1 => {
            StartCursorAnim(CURSOR_ANIM_OPEN);
            SwapItemsWithMon(
                (if sInPartyMenu != 0 {
                    CURSOR_AREA_IN_PARTY as i32
                } else {
                    CURSOR_AREA_IN_BOX as i32
                }) as u8,
                GetCursorPosition(),
            );
            (*sStorage).state += 1;
        }
        2 => {
            if IsItemIconAnimActive() == 0 {
                StartCursorAnim(CURSOR_ANIM_FIST);
                RefreshDisplayMon();
                PrintDisplayMonInfo();
                PrintMessage(MSG_CHANGED_TO_ITEM);
                (*sStorage).state += 1;
            }
        }
        3 => {
            if gMain.newKeys as i32 & 243 != 0 {
                ClearBottomWindow();
                (*sStorage).state += 1;
            }
        }
        4 => {
            if IsDma3ManagerBusyWithBgCopy() == 0 {
                SetPokeStorageTask(Some(Task_PokeStorageMain));
            }
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn Task_ShowItemInfo(taskId: u8) {
    match (*sStorage).state {
        0 => {
            ClearBottomWindow();
            (*sStorage).state += 1;
        }
        1 => {
            if IsDma3ManagerBusyWithBgCopy() == 0 {
                PlaySE(SE_WIN_OPEN);
                PrintItemDescription();
                InitItemInfoWindow();
                (*sStorage).state += 1;
            }
        }
        2 => {
            if UpdateItemInfoWindowSlideIn() == 0 {
                (*sStorage).state += 1;
            }
        }
        3 => {
            if IsDma3ManagerBusyWithBgCopy() == 0 {
                (*sStorage).state += 1;
            }
        }
        4 => {
            if gMain.newKeys as i32 & 243 != 0 {
                PlaySE(SE_WIN_OPEN);
                (*sStorage).state += 1;
            }
        }
        5 => {
            if UpdateItemInfoWindowSlideOut() == 0 {
                (*sStorage).state += 1;
            }
        }
        6 => {
            if IsDma3ManagerBusyWithBgCopy() == 0 {
                SetPokeStorageTask(Some(Task_PokeStorageMain));
            }
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn Task_CloseBoxWhileHoldingItem(taskId: u8) {
    match (*sStorage).state {
        0 => {
            PlaySE(SE_SELECT);
            PrintMessage(MSG_PUT_IN_BAG);
            ShowYesNoWindow(0);
            (*sStorage).state = 1;
        }
        1 => match Menu_ProcessInputNoWrapClearOnChoose() {
            MENU_B_PRESSED | 1 => {
                ClearBottomWindow();
                SetPokeStorageTask(Some(Task_PokeStorageMain));
            }
            0 => {
                if AddBagItem((*sStorage).movingItemId, 1) == 1 {
                    ClearBottomWindow();
                    (*sStorage).state = 3;
                } else {
                    PrintMessage(MSG_BAG_FULL);
                    (*sStorage).state = 2;
                }
            }
            _ => {}
        },
        2 => {
            if gMain.newKeys as i32 & 243 != 0 {
                ClearBottomWindow();
                (*sStorage).state = 5;
            }
        }
        3 => {
            MoveItemFromCursorToBag();
            (*sStorage).state = 4;
        }
        4 => {
            if IsItemIconAnimActive() == 0 {
                StartCursorAnim(CURSOR_ANIM_BOUNCE);
                SetPokeStorageTask(Some(Task_PokeStorageMain));
            }
        }
        5 => {
            if IsDma3ManagerBusyWithBgCopy() == 0 {
                SetPokeStorageTask(Some(Task_PokeStorageMain));
            }
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn Task_HandleMovingMonFromParty(taskId: u8) {
    match (*sStorage).state {
        0 => {
            CompactPartySlots();
            CompactPartySprites();
            (*sStorage).state += 1;
        }
        1 => {
            if GetNumPartySpritesCompacting() == 0 {
                UpdatePartySlotColors();
                SetPokeStorageTask(Some(Task_PokeStorageMain));
            }
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn Task_PrintCantStoreMail(taskId: u8) {
    match (*sStorage).state {
        0 => {
            PrintMessage(MSG_CANT_STORE_MAIL);
            (*sStorage).state += 1;
        }
        1 => {
            if IsDma3ManagerBusyWithBgCopy() == 0 {
                (*sStorage).state += 1;
            }
        }
        2 => {
            if gMain.newKeys as i32 & 243 != 0 {
                ClearBottomWindow();
                (*sStorage).state += 1;
            }
        }
        3 => {
            if IsDma3ManagerBusyWithBgCopy() == 0 {
                SetPokeStorageTask(Some(Task_PokeStorageMain));
            }
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn Task_HandleBoxOptions(taskId: u8) {
    'l1: {
        let sw1: u8 = (*sStorage).state;
        let mut fall = false;
        if sw1 == 0 {
            fall = true;
            PrintMessage(MSG_WHAT_YOU_DO);
            AddMenu();
            (*sStorage).state += 1;
            break 'l1;
        }
        if sw1 == 1 {
            fall = true;
            if IsMenuLoading() != 0 {
                return;
            }
            (*sStorage).state += 1;
        }
        if fall || sw1 == 2 {
            fall = true;
            match HandleMenuInput() {
                -1 | 0 => {
                    AnimateBoxScrollArrows(TRUE);
                    ClearBottomWindow();
                    SetPokeStorageTask(Some(Task_PokeStorageMain));
                }
                MENU_NAME => {
                    PlaySE(SE_SELECT);
                    SetPokeStorageTask(Some(Task_NameBox));
                }
                MENU_WALLPAPER => {
                    PlaySE(SE_SELECT);
                    ClearBottomWindow();
                    SetPokeStorageTask(Some(Task_HandleWallpapers));
                }
                MENU_JUMP => {
                    PlaySE(SE_SELECT);
                    ClearBottomWindow();
                    SetPokeStorageTask(Some(Task_JumpBox));
                }
                _ => {}
            }
            break 'l1;
        }
    }
}
pub(crate) unsafe extern "C" fn Task_HandleWallpapers(taskId: u8) {
    match (*sStorage).state {
        0 => {
            AddWallpaperSetsMenu();
            PrintMessage(MSG_PICK_A_THEME);
            (*sStorage).state += 1;
        }
        1 => {
            if IsMenuLoading() == 0 {
                (*sStorage).state += 1;
            }
        }
        2 => {
            (*sStorage).wallpaperSetId = HandleMenuInput();
            match (*sStorage).wallpaperSetId {
                -1 => {
                    AnimateBoxScrollArrows(TRUE);
                    ClearBottomWindow();
                    SetPokeStorageTask(Some(Task_PokeStorageMain));
                }
                MENU_SCENERY_1 | MENU_SCENERY_2 | MENU_SCENERY_3 | MENU_ETCETERA => {
                    PlaySE(SE_SELECT);
                    RemoveMenu();
                    (*sStorage).wallpaperSetId -= MENU_SCENERY_1;
                    (*sStorage).state += 1;
                }
                MENU_FRIENDS => {
                    PlaySE(SE_SELECT);
                    (*sStorage).wallpaperId = WALLPAPER_FRIENDS;
                    RemoveMenu();
                    ClearBottomWindow();
                    (*sStorage).state = 6;
                }
                _ => {}
            }
        }
        3 => {
            if IsDma3ManagerBusyWithBgCopy() == 0 {
                AddWallpapersMenu((*sStorage).wallpaperSetId as u8);
                PrintMessage(MSG_PICK_A_WALLPAPER);
                (*sStorage).state += 1;
            }
        }
        4 => {
            (*sStorage).wallpaperId = HandleMenuInput();
            match (*sStorage).wallpaperId {
                -2 => {}
                -1 => {
                    ClearBottomWindow();
                    (*sStorage).state = 0;
                }
                _ => {
                    PlaySE(SE_SELECT);
                    ClearBottomWindow();
                    (*sStorage).wallpaperId -= MENU_FOREST;
                    SetWallpaperForCurrentBox((*sStorage).wallpaperId as u8);
                    (*sStorage).state += 1;
                }
            }
        }
        5 => {
            if DoWallpaperGfxChange() == 0 {
                AnimateBoxScrollArrows(TRUE);
                SetPokeStorageTask(Some(Task_PokeStorageMain));
            }
        }
        6 => {
            if IsDma3ManagerBusyWithBgCopy() == 0 {
                SetWallpaperForCurrentBox((*sStorage).wallpaperId as u8);
                (*sStorage).state = 5;
            }
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn Task_JumpBox(taskId: u8) {
    match (*sStorage).state {
        0 => {
            PrintMessage(MSG_JUMP_TO_WHICH_BOX);
            LoadChooseBoxMenuGfx(
                &raw mut (*sStorage).chooseBoxMenu,
                GFXTAG_CHOOSE_BOX_MENU,
                PALTAG_MISC_1,
                3,
                FALSE as u32,
            );
            CreateChooseBoxMenuSprites(StorageGetCurrentBox());
            (*sStorage).state += 1;
        }
        1 => {
            (*sStorage).newCurrBoxId = HandleChooseBoxMenuInput() as i16;
            match (*sStorage).newCurrBoxId {
                200 => {}
                _ => {
                    ClearBottomWindow();
                    DestroyChooseBoxMenuSprites();
                    FreeChooseBoxMenu();
                    if (*sStorage).newCurrBoxId == BOXID_CANCELED as i16
                        || (*sStorage).newCurrBoxId == StorageGetCurrentBox() as i16
                    {
                        AnimateBoxScrollArrows(TRUE);
                        SetPokeStorageTask(Some(Task_PokeStorageMain));
                    } else {
                        (*sStorage).state += 1;
                    }
                }
            }
        }
        2 => {
            SetUpScrollToBox((*sStorage).newCurrBoxId as u8);
            (*sStorage).state += 1;
        }
        3 => {
            if ScrollToBox() == 0 {
                SetCurrentBox((*sStorage).newCurrBoxId as u8);
                SetPokeStorageTask(Some(Task_PokeStorageMain));
            }
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn Task_NameBox(taskId: u8) {
    match (*sStorage).state {
        0 => {
            SaveMovingMon();
            BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 16, 0);
            (*sStorage).state += 1;
        }
        1 => {
            if UpdatePaletteFade() == 0 {
                sWhichToReshow = 1;
                (*sStorage).screenChangeType = SCREEN_CHANGE_NAME_BOX;
                SetPokeStorageTask(Some(Task_ChangeScreen));
            }
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn Task_ShowMonSummary(taskId: u8) {
    match (*sStorage).state {
        0 => {
            InitSummaryScreenData();
            BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 16, 0);
            (*sStorage).state += 1;
        }
        1 => {
            if UpdatePaletteFade() == 0 {
                sWhichToReshow = 0;
                (*sStorage).screenChangeType = SCREEN_CHANGE_SUMMARY_SCREEN;
                SetPokeStorageTask(Some(Task_ChangeScreen));
            }
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn Task_GiveItemFromBag(taskId: u8) {
    match (*sStorage).state {
        0 => {
            BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 16, 0);
            (*sStorage).state += 1;
        }
        1 => {
            if UpdatePaletteFade() == 0 {
                sWhichToReshow = 2;
                (*sStorage).screenChangeType = SCREEN_CHANGE_ITEM_FROM_BAG;
                SetPokeStorageTask(Some(Task_ChangeScreen));
            }
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn Task_OnCloseBoxPressed(taskId: u8) {
    match (*sStorage).state {
        0 => {
            if IsMonBeingMoved() != 0 {
                PlaySE(SE_FAILURE);
                PrintMessage(MSG_HOLDING_POKE);
                (*sStorage).state = 1;
            } else if IsMovingItem() != 0 {
                SetPokeStorageTask(Some(Task_CloseBoxWhileHoldingItem));
            } else {
                PlaySE(SE_SELECT);
                PrintMessage(MSG_EXIT_BOX);
                ShowYesNoWindow(0);
                (*sStorage).state = 2;
            }
        }
        1 => {
            if gMain.newKeys as i32 & 243 != 0 {
                ClearBottomWindow();
                SetPokeStorageTask(Some(Task_PokeStorageMain));
            }
        }
        2 => match Menu_ProcessInputNoWrapClearOnChoose() {
            MENU_B_PRESSED | 1 => {
                ClearBottomWindow();
                SetPokeStorageTask(Some(Task_PokeStorageMain));
            }
            0 => {
                PlaySE(SE_PC_OFF);
                ClearBottomWindow();
                (*sStorage).state += 1;
            }
            _ => {}
        },
        3 => {
            ComputerScreenCloseEffect(20, 0, 1);
            (*sStorage).state += 1;
        }
        4 => {
            if IsComputerScreenCloseEffectActive() == 0 {
                UpdateBoxToSendMons();
                gPlayerPartyCount = CalculatePlayerPartyCount();
                (*sStorage).screenChangeType = SCREEN_CHANGE_EXIT_BOX;
                SetPokeStorageTask(Some(Task_ChangeScreen));
            }
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn Task_OnBPressed(taskId: u8) {
    match (*sStorage).state {
        0 => {
            if IsMonBeingMoved() != 0 {
                PlaySE(SE_FAILURE);
                PrintMessage(MSG_HOLDING_POKE);
                (*sStorage).state = 1;
            } else if IsMovingItem() != 0 {
                SetPokeStorageTask(Some(Task_CloseBoxWhileHoldingItem));
            } else {
                PlaySE(SE_SELECT);
                PrintMessage(MSG_CONTINUE_BOX);
                ShowYesNoWindow(0);
                (*sStorage).state = 2;
            }
        }
        1 => {
            if gMain.newKeys as i32 & 243 != 0 {
                ClearBottomWindow();
                SetPokeStorageTask(Some(Task_PokeStorageMain));
            }
        }
        2 => match Menu_ProcessInputNoWrapClearOnChoose() {
            0 => {
                ClearBottomWindow();
                SetPokeStorageTask(Some(Task_PokeStorageMain));
            }
            1 | MENU_B_PRESSED => {
                PlaySE(SE_PC_OFF);
                ClearBottomWindow();
                (*sStorage).state += 1;
            }
            _ => {}
        },
        3 => {
            ComputerScreenCloseEffect(20, 0, 0);
            (*sStorage).state += 1;
        }
        4 => {
            if IsComputerScreenCloseEffectActive() == 0 {
                UpdateBoxToSendMons();
                gPlayerPartyCount = CalculatePlayerPartyCount();
                (*sStorage).screenChangeType = SCREEN_CHANGE_EXIT_BOX;
                SetPokeStorageTask(Some(Task_ChangeScreen));
            }
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn Task_ChangeScreen(taskId: u8) {
    let mut boxMons: *mut BoxPokemon = null_mut();
    let mut mode: u8 = 0;
    let mut monIndex: u8 = 0;
    let mut maxMonIndex: u8 = 0;
    let mut screenChangeType: u8 = (*sStorage).screenChangeType;
    if (*sStorage).boxOption == OPTION_MOVE_ITEMS && IsMovingItem() == TRUE {
        sMovingItemId = GetMovingItemId();
    } else {
        sMovingItemId = ITEM_NONE;
    }
    match screenChangeType {
        SCREEN_CHANGE_SUMMARY_SCREEN => {
            boxMons = (*sStorage).summaryMon.r#box;
            monIndex = (*sStorage).summaryStartPos;
            maxMonIndex = (*sStorage).summaryMaxPos;
            mode = (*sStorage).summaryScreenMode;
            FreePokeStorageData();
            if mode == SUMMARY_MODE_NORMAL && boxMons == &raw mut sSavedMovingMon.r#box {
                ShowPokemonSummaryScreenHandleDeoxys(
                    mode,
                    boxMons,
                    monIndex,
                    maxMonIndex,
                    Some(CB2_ReturnToPokeStorage),
                );
            } else {
                ShowPokemonSummaryScreen(
                    mode,
                    boxMons as *mut c_void,
                    monIndex,
                    maxMonIndex,
                    Some(CB2_ReturnToPokeStorage),
                );
            }
        }
        SCREEN_CHANGE_NAME_BOX => {
            FreePokeStorageData();
            DoNamingScreen(
                NAMING_SCREEN_BOX,
                GetBoxNamePtr(StorageGetCurrentBox()),
                0,
                0,
                0,
                Some(CB2_ReturnToPokeStorage),
            );
        }
        SCREEN_CHANGE_ITEM_FROM_BAG => {
            FreePokeStorageData();
            GoToBagMenu(ITEMMENULOCATION_PCBOX, 0, Some(CB2_ReturnToPokeStorage));
        }
        _ => {
            FreePokeStorageData();
            SetMainCallback2(Some(CB2_ExitPokeStorage));
        }
    }
    DestroyTask(taskId);
}
pub(crate) unsafe extern "C" fn GiveChosenBagItem() {
    let mut itemId: u16 = gSpecialVar_ItemId;
    if itemId != ITEM_NONE {
        let mut pos: u8 = GetCursorPosition();
        if sInPartyMenu != 0 {
            SetMonData(
                &raw mut gPlayerParty[pos],
                MON_DATA_HELD_ITEM,
                &raw mut itemId as *mut c_void,
            );
        } else {
            SetCurrentBoxMonData(pos, MON_DATA_HELD_ITEM, &raw mut itemId as *mut c_void);
        }
        RemoveBagItem(itemId, 1);
    }
}
pub(crate) unsafe extern "C" fn FreePokeStorageData() {
    TilemapUtil_Free();
    MultiMove_Free();
    Free(sStorage as *mut c_void);
    sStorage = null_mut();
    FreeAllWindowBuffers();
}
pub(crate) unsafe extern "C" fn SetScrollingBackground() {
    SetGpuReg(REG_OFFSET_BG3CNT, 7951);
    DecompressAndLoadBgGfxUsingHeap(
        3,
        sScrollingBg_Gfx.as_ptr().cast_mut() as *mut c_void,
        0,
        0,
        0,
    );
    LZ77UnCompVram(
        sScrollingBg_Tilemap.as_ptr().cast_mut(),
        0x600f800 as usize as *mut c_void,
    );
}
pub(crate) unsafe extern "C" fn ScrollBackground() {
    ChangeBgX(3, 128, BG_COORD_ADD);
    ChangeBgY(3, 128, BG_COORD_SUB);
}
pub(crate) unsafe extern "C" fn LoadPokeStorageMenuGfx() {
    InitBgsFromTemplates(0, sBgTemplates.as_ptr().cast_mut(), 4);
    DecompressAndLoadBgGfxUsingHeap(
        1,
        gStorageSystemMenu_Gfx.as_ptr().cast_mut() as *mut c_void,
        0,
        0,
        0,
    );
    LZ77UnCompWram(
        sDisplayMenu_Tilemap.as_ptr().cast_mut(),
        (*sStorage).displayMenuTilemapBuffer.as_mut_ptr() as *mut c_void,
    );
    SetBgTilemapBuffer(
        1,
        (*sStorage).displayMenuTilemapBuffer.as_mut_ptr() as *mut c_void,
    );
    ShowBg(1);
    ScheduleBgCopyTilemapToVram(1);
}
pub(crate) unsafe extern "C" fn InitPokeStorageWindows() -> u8 {
    if InitWindows(sWindowTemplates.as_ptr().cast_mut()) == 0 {
        return FALSE;
    } else {
        DeactivateAllTextPrinters();
        return TRUE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn LoadWaveformSpritePalette() {
    LoadSpritePalette((&raw const *sWaveformSpritePalette).cast_mut());
}
pub(crate) unsafe extern "C" fn InitPalettesAndSprites() {
    LoadPalette(sInterface_Pal.as_ptr().cast_mut() as *mut c_void, 0, 32);
    LoadPalette(sPkmnDataGray_Pal.as_ptr().cast_mut() as *mut c_void, 32, 32);
    LoadPalette(sTextWindows_Pal.as_ptr().cast_mut() as *mut c_void, 240, 32);
    if (*sStorage).boxOption != OPTION_MOVE_ITEMS {
        LoadPalette(sScrollingBg_Pal.as_ptr().cast_mut() as *mut c_void, 48, 32);
    } else {
        LoadPalette(
            sScrollingBgMoveItems_Pal.as_ptr().cast_mut() as *mut c_void,
            48,
            32,
        );
    }
    SetGpuReg(REG_OFFSET_BG1CNT, 7685);
    CreateDisplayMonSprite();
    CreateMarkingComboSprite();
    CreateWaveformSprites();
    RefreshDisplayMonData();
}
pub(crate) unsafe extern "C" fn CreateMarkingComboSprite() {
    (*sStorage).markingComboSprite =
        CreateMonMarkingComboSprite(GFXTAG_MARKING_COMBO, PALTAG_MARKING_COMBO, null_mut());
    (*(*sStorage).markingComboSprite).oam.set_priority(1);
    (*(*sStorage).markingComboSprite).subpriority = 1;
    (*(*sStorage).markingComboSprite).x = 40;
    (*(*sStorage).markingComboSprite).y = 150;
    (*sStorage).markingComboTilesPtr = (OBJ_VRAM0 as usize as *mut c_void as *mut u8)
        .at(32 * GetSpriteTileStartByTag(GFXTAG_MARKING_COMBO) as i32)
        as *mut c_void as *mut u16;
}
pub(crate) unsafe extern "C" fn CreateWaveformSprites() {
    let mut i: u16 = 0;
    let mut sheet: SpriteSheet = zeroed();
    sheet = *sSpriteSheet_Waveform;
    LoadSpriteSheet(&raw mut sheet);
    i = 0;
    while i < 2 {
        let mut spriteId: u8 = CreateSprite(
            (&raw const *sSpriteTemplate_Waveform).cast_mut(),
            i as i16 * 63 + 8,
            9,
            2,
        );
        (*sStorage).waveformSprites[i] = &raw mut gSprites[spriteId];
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn RefreshDisplayMonData() {
    LoadDisplayMonGfx(
        (*sStorage).displayMonSpecies,
        (*sStorage).displayMonPersonality,
    );
    PrintDisplayMonInfo();
    UpdateWaveformAnimation();
    ScheduleBgCopyTilemapToVram(0);
}
pub(crate) unsafe extern "C" fn StartDisplayMonMosaicEffect() {
    RefreshDisplayMonData();
    if !(*sStorage).displayMonSprite.is_null() {
        (*(*sStorage).displayMonSprite).oam.set_mosaic(TRUE as u32);
        (*(*sStorage).displayMonSprite).data[0] = 10;
        (*(*sStorage).displayMonSprite).data[1] = 1;
        (*(*sStorage).displayMonSprite).callback = Some(SpriteCB_DisplayMonMosaic);
        SetGpuReg(
            REG_OFFSET_MOSAIC,
            ((*(*sStorage).displayMonSprite).data[0] as u16) << 12
                | ((*(*sStorage).displayMonSprite).data[0] as u16) << 8,
        );
    }
}
pub(crate) unsafe extern "C" fn IsDisplayMosaicActive() -> u8 {
    return (*(*sStorage).displayMonSprite).oam.mosaic() as u8;
}
pub(crate) unsafe extern "C" fn SpriteCB_DisplayMonMosaic(sprite: *mut Sprite) {
    (*sprite).data[0] -= (*sprite).data[1];
    if (*sprite).data[0] < 0 {
        (*sprite).data[0] = 0;
    }
    SetGpuReg(
        REG_OFFSET_MOSAIC,
        ((*sprite).data[0] as u16) << 12 | ((*sprite).data[0] as u16) << 8,
    );
    if (*sprite).data[0] == 0 {
        (*sprite).oam.set_mosaic(FALSE as u32);
        (*sprite).callback = Some(SpriteCallbackDummy);
    }
}
pub(crate) unsafe extern "C" fn CreateDisplayMonSprite() {
    let mut i: u16 = 0;
    let mut tileStart: u16 = 0;
    let mut palSlot: u8 = 0;
    let mut spriteId: u8 = 0;
    let mut sheet: SpriteSheet = zeroed();
    sheet.data = (*sStorage).tileBuffer.as_mut_ptr() as *mut c_void;
    sheet.size = MON_PIC_SIZE;
    sheet.tag = GFXTAG_DISPLAY_MON;
    let mut palette: SpritePalette = zeroed();
    palette.data = (*sStorage).displayMonPalBuffer.as_mut_ptr();
    palette.tag = PALTAG_DISPLAY_MON;
    let mut template: SpriteTemplate = zeroed();
    template = *sSpriteTemplate_DisplayMon;
    i = 0;
    while i < MON_PIC_SIZE {
        (*sStorage).tileBuffer[i] = 0;
        i += 1;
    }
    i = 0;
    while i < 16 {
        (*sStorage).displayMonPalBuffer[i] = 0;
        i += 1;
    }
    (*sStorage).displayMonSprite = null_mut();
    'l4: {
        tileStart = LoadSpriteSheet(&raw mut sheet);
        if tileStart == 0 {
            break 'l4;
        }
        palSlot = LoadSpritePalette(&raw mut palette);
        if palSlot == 0xFF {
            break 'l4;
        }
        spriteId = CreateSprite(&raw mut template, 40, 48, 0);
        if spriteId == MAX_SPRITES {
            break 'l4;
        }
        (*sStorage).displayMonSprite = &raw mut gSprites[spriteId];
        (*sStorage).displayMonPalOffset = 0x100 + palSlot as u16 * 16;
        (*sStorage).displayMonTilePtr = (OBJ_VRAM0 as usize as *mut c_void as *mut u8)
            .at(tileStart as i32 * 32) as *mut c_void
            as *mut u16;
    }
    if (*sStorage).displayMonSprite.is_null() {
        FreeSpriteTilesByTag(GFXTAG_DISPLAY_MON);
        FreeSpritePaletteByTag(PALTAG_DISPLAY_MON);
    }
}
pub(crate) unsafe extern "C" fn LoadDisplayMonGfx(species: u16, pid: u32) {
    if (*sStorage).displayMonSprite.is_null() {
        return;
    }
    if species != SPECIES_NONE {
        LoadSpecialPokePic(
            (&raw const gMonFrontPicTable[species]).cast_mut(),
            (*sStorage).tileBuffer.as_mut_ptr() as *mut c_void,
            species as i32,
            pid,
            TRUE,
        );
        LZ77UnCompWram(
            (*sStorage).displayMonPalette,
            (*sStorage).displayMonPalBuffer.as_mut_ptr() as *mut c_void,
        );
        CpuSet(
            (*sStorage).tileBuffer.as_mut_ptr() as *mut c_void,
            (*sStorage).displayMonTilePtr as *mut c_void,
            0x4000200,
        );
        LoadPalette(
            (*sStorage).displayMonPalBuffer.as_mut_ptr() as *mut c_void,
            (*sStorage).displayMonPalOffset,
            32,
        );
        (*(*sStorage).displayMonSprite).set_invisible(FALSE as u16);
    } else {
        (*(*sStorage).displayMonSprite).set_invisible(TRUE as u16);
    }
}
pub(crate) unsafe extern "C" fn PrintDisplayMonInfo() {
    FillWindowPixelBuffer(WIN_DISPLAY_INFO, 17);
    if (*sStorage).boxOption != OPTION_MOVE_ITEMS {
        AddTextPrinterParameterized(
            WIN_DISPLAY_INFO,
            FONT_NORMAL,
            (*sStorage).displayMonNameText.as_mut_ptr(),
            6,
            0,
            TEXT_SKIP_DRAW,
            None,
        );
        AddTextPrinterParameterized(
            WIN_DISPLAY_INFO,
            FONT_SHORT,
            (*sStorage).displayMonSpeciesName.as_mut_ptr(),
            6,
            15,
            TEXT_SKIP_DRAW,
            None,
        );
        AddTextPrinterParameterized(
            WIN_DISPLAY_INFO,
            FONT_SHORT,
            (*sStorage).displayMonGenderLvlText.as_mut_ptr(),
            10,
            29,
            TEXT_SKIP_DRAW,
            None,
        );
        AddTextPrinterParameterized(
            WIN_DISPLAY_INFO,
            FONT_SMALL,
            (*sStorage).displayMonItemName.as_mut_ptr(),
            6,
            43,
            TEXT_SKIP_DRAW,
            None,
        );
    } else {
        AddTextPrinterParameterized(
            WIN_DISPLAY_INFO,
            FONT_SMALL,
            (*sStorage).displayMonItemName.as_mut_ptr(),
            6,
            0,
            TEXT_SKIP_DRAW,
            None,
        );
        AddTextPrinterParameterized(
            WIN_DISPLAY_INFO,
            FONT_NORMAL,
            (*sStorage).displayMonNameText.as_mut_ptr(),
            6,
            13,
            TEXT_SKIP_DRAW,
            None,
        );
        AddTextPrinterParameterized(
            WIN_DISPLAY_INFO,
            FONT_SHORT,
            (*sStorage).displayMonSpeciesName.as_mut_ptr(),
            6,
            28,
            TEXT_SKIP_DRAW,
            None,
        );
        AddTextPrinterParameterized(
            WIN_DISPLAY_INFO,
            FONT_SHORT,
            (*sStorage).displayMonGenderLvlText.as_mut_ptr(),
            10,
            42,
            TEXT_SKIP_DRAW,
            None,
        );
    }
    CopyWindowToVram(WIN_DISPLAY_INFO, COPYWIN_GFX);
    if (*sStorage).displayMonSpecies != SPECIES_NONE {
        UpdateMonMarkingTiles(
            (*sStorage).displayMonMarkings,
            (*sStorage).markingComboTilesPtr as *mut c_void,
        );
        (*(*sStorage).markingComboSprite).set_invisible(FALSE as u16);
    } else {
        (*(*sStorage).markingComboSprite).set_invisible(TRUE as u16);
    }
}
pub(crate) unsafe extern "C" fn UpdateWaveformAnimation() {
    let mut i: u16 = 0;
    if (*sStorage).displayMonSpecies != SPECIES_NONE {
        TilemapUtil_SetRect(TILEMAPID_PKMN_DATA, 0, 0, 8, 2);
        i = 0;
        while i < 2 {
            StartSpriteAnimIfDifferent((*sStorage).waveformSprites[i], i as u8 * 2 + 1);
            i += 1;
        }
    } else {
        TilemapUtil_SetRect(TILEMAPID_PKMN_DATA, 0, 2, 8, 2);
        i = 0;
        while i < 2 {
            StartSpriteAnim((*sStorage).waveformSprites[i], i as u8 * 2);
            i += 1;
        }
    }
    TilemapUtil_Update(TILEMAPID_PKMN_DATA);
    ScheduleBgCopyTilemapToVram(1);
}
pub(crate) unsafe extern "C" fn InitSupplementalTilemaps() {
    LZ77UnCompWram(
        gStorageSystemPartyMenu_Tilemap.as_ptr().cast_mut(),
        (*sStorage).partyMenuTilemapBuffer.as_mut_ptr() as *mut c_void,
    );
    LoadPalette(
        gStorageSystemPartyMenu_Pal.as_ptr().cast_mut() as *mut c_void,
        16,
        32,
    );
    TilemapUtil_SetMap(
        TILEMAPID_PARTY_MENU,
        1,
        (*sStorage).partyMenuTilemapBuffer.as_mut_ptr() as *mut c_void,
        12,
        22,
    );
    TilemapUtil_SetMap(
        TILEMAPID_CLOSE_BUTTON,
        1,
        sCloseBoxButton_Tilemap.as_ptr().cast_mut() as *mut c_void,
        9,
        4,
    );
    TilemapUtil_SetPos(TILEMAPID_PARTY_MENU, 10, 0);
    TilemapUtil_SetPos(TILEMAPID_CLOSE_BUTTON, 21, 0);
    SetPartySlotTilemaps();
    if sInPartyMenu != 0 {
        UpdateCloseBoxButtonTilemap(TRUE);
        CreatePartyMonsSprites(TRUE);
        TilemapUtil_Update(TILEMAPID_CLOSE_BUTTON);
        TilemapUtil_Update(TILEMAPID_PARTY_MENU);
    } else {
        TilemapUtil_SetRect(TILEMAPID_PARTY_MENU, 0, 20, 12, 2);
        UpdateCloseBoxButtonTilemap(TRUE);
        TilemapUtil_Update(TILEMAPID_PARTY_MENU);
        TilemapUtil_Update(TILEMAPID_CLOSE_BUTTON);
    }
    ScheduleBgCopyTilemapToVram(1);
    (*sStorage).closeBoxFlashing = FALSE;
}
pub(crate) unsafe extern "C" fn SetUpShowPartyMenu() {
    (*sStorage).partyMenuUnused1 = 20;
    (*sStorage).partyMenuY = 2;
    (*sStorage).partyMenuMoveTimer = 0;
    CreatePartyMonsSprites(FALSE);
}
pub(crate) unsafe extern "C" fn ShowPartyMenu() -> u8 {
    if (*sStorage).partyMenuMoveTimer == 20 {
        return FALSE;
    }
    (*sStorage).partyMenuUnused1 -= 1;
    (*sStorage).partyMenuY += 1;
    TilemapUtil_Move(TILEMAPID_PARTY_MENU, 3, 1);
    TilemapUtil_Update(TILEMAPID_PARTY_MENU);
    ScheduleBgCopyTilemapToVram(1);
    MovePartySprites(8);
    if ({
        (*sStorage).partyMenuMoveTimer += 1;
        (*sStorage).partyMenuMoveTimer
    }) == 20
    {
        sInPartyMenu = TRUE;
        return FALSE;
    } else {
        return TRUE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn SetUpHidePartyMenu() {
    (*sStorage).partyMenuUnused1 = 0;
    (*sStorage).partyMenuY = 22;
    (*sStorage).partyMenuMoveTimer = 0;
    if (*sStorage).boxOption == OPTION_MOVE_ITEMS {
        MoveHeldItemWithPartyMenu();
    }
}
pub(crate) unsafe extern "C" fn HidePartyMenu() -> u8 {
    if (*sStorage).partyMenuMoveTimer != 20 {
        (*sStorage).partyMenuUnused1 += 1;
        (*sStorage).partyMenuY -= 1;
        TilemapUtil_Move(TILEMAPID_PARTY_MENU, 3, -1);
        TilemapUtil_Update(TILEMAPID_PARTY_MENU);
        FillBgTilemapBufferRect_Palette0(1, 0x100, 10, (*sStorage).partyMenuY as u8, 12, 1);
        MovePartySprites(-8);
        if ({
            (*sStorage).partyMenuMoveTimer += 1;
            (*sStorage).partyMenuMoveTimer
        }) != 20
        {
            ScheduleBgCopyTilemapToVram(1);
            return TRUE;
        } else {
            sInPartyMenu = FALSE;
            DestroyAllPartyMonIcons();
            CompactPartySlots();
            TilemapUtil_SetRect(TILEMAPID_CLOSE_BUTTON, 0, 0, 9, 2);
            TilemapUtil_Update(TILEMAPID_CLOSE_BUTTON);
            ScheduleBgCopyTilemapToVram(1);
            return FALSE;
        }
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn UpdateCloseBoxButtonTilemap(normal: u8) {
    if normal != 0 {
        TilemapUtil_SetRect(TILEMAPID_CLOSE_BUTTON, 0, 0, 9, 2);
    } else {
        TilemapUtil_SetRect(TILEMAPID_CLOSE_BUTTON, 0, 2, 9, 2);
    }
    TilemapUtil_Update(TILEMAPID_CLOSE_BUTTON);
    ScheduleBgCopyTilemapToVram(1);
}
pub(crate) unsafe extern "C" fn StartFlashingCloseBoxButton() {
    (*sStorage).closeBoxFlashing = TRUE;
    (*sStorage).closeBoxFlashTimer = 30;
    (*sStorage).closeBoxFlashState = TRUE;
}
pub(crate) unsafe extern "C" fn StopFlashingCloseBoxButton() {
    if (*sStorage).closeBoxFlashing != 0 {
        (*sStorage).closeBoxFlashing = FALSE;
        UpdateCloseBoxButtonTilemap(TRUE);
    }
}
pub(crate) unsafe extern "C" fn UpdateCloseBoxButtonFlash() {
    if (*sStorage).closeBoxFlashing != 0
        && ({
            (*sStorage).closeBoxFlashTimer += 1;
            (*sStorage).closeBoxFlashTimer
        }) > 30
    {
        (*sStorage).closeBoxFlashTimer = 0;
        (*sStorage).closeBoxFlashState = ((*sStorage).closeBoxFlashState == FALSE) as u8;
        UpdateCloseBoxButtonTilemap((*sStorage).closeBoxFlashState);
    }
}
pub(crate) unsafe extern "C" fn SetPartySlotTilemaps() {
    let mut i: u8 = 0;
    i = 1;
    while i < PARTY_SIZE as u8 {
        let mut species: i32 = GetMonData2(&raw mut gPlayerParty[i], MON_DATA_SPECIES) as i32;
        SetPartySlotTilemap(i, (species != SPECIES_NONE as i32) as u8);
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn SetPartySlotTilemap(partyId: u8, hasMon: u8) {
    let mut i: u16 = 0;
    let mut j: u16 = 0;
    let mut index: u16 = 0;
    let mut data: *mut u16 = null_mut();
    if hasMon != 0 {
        data = sPartySlotFilled_Tilemap.as_ptr().cast_mut();
    } else {
        data = sPartySlotEmpty_Tilemap.as_ptr().cast_mut();
    }
    index = 3 * (3 * (partyId as u16 - 1) + 1);
    index *= 4;
    index += 7;
    i = 0;
    while i < 3 {
        j = 0;
        while j < 4 {
            (*sStorage).partyMenuTilemapBuffer[index as i32 + j as i32] = *data.at(j);
            j += 1;
        }
        data = data.at(4);
        index += 12;
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn UpdatePartySlotColors() {
    SetPartySlotTilemaps();
    TilemapUtil_SetRect(TILEMAPID_PARTY_MENU, 0, 0, 12, 22);
    TilemapUtil_Update(TILEMAPID_PARTY_MENU);
    ScheduleBgCopyTilemapToVram(1);
}
pub(crate) unsafe extern "C" fn SetUpDoShowPartyMenu() {
    (*sStorage).showPartyMenuState = 0;
    PlaySE(SE_WIN_OPEN);
    SetUpShowPartyMenu();
}
pub(crate) unsafe extern "C" fn DoShowPartyMenu() -> u8 {
    match (*sStorage).showPartyMenuState {
        0 => {
            if ShowPartyMenu() == 0 {
                SetCursorInParty();
                (*sStorage).showPartyMenuState += 1;
            }
        }
        1 => {
            if UpdateCursorPos() == 0 {
                if (*sStorage).setMosaic != 0 {
                    StartDisplayMonMosaicEffect();
                }
                (*sStorage).showPartyMenuState += 1;
            }
        }
        2 => {
            return FALSE;
        }
        _ => {}
    }
    return TRUE;
}
pub(crate) unsafe extern "C" fn UpdateBoxToSendMons() {
    if sLastUsedBox != StorageGetCurrentBox() {
        FlagClear(FLAG_SHOWN_BOX_WAS_FULL_MESSAGE);
        VarSet(VAR_PC_BOX_TO_SEND_MON, StorageGetCurrentBox() as u16);
    }
}
pub(crate) unsafe extern "C" fn InitPokeStorageBg0() {
    SetGpuReg(REG_OFFSET_BG0CNT, 7424);
    LoadUserWindowBorderGfx(WIN_MESSAGE, 2, 208);
    FillBgTilemapBufferRect(0, 0, 0, 0, 32, 20, 17);
    CopyBgTilemapBufferToVram(0);
}
pub(crate) unsafe extern "C" fn PrintMessage(id: u8) {
    let mut txtPtr: *mut u8 = null_mut();
    DynamicPlaceholderTextUtil_Reset();
    match sMessages[id].format {
        MSG_VAR_NONE => {}
        MSG_VAR_MON_NAME_1 | MSG_VAR_MON_NAME_2 | MSG_VAR_MON_NAME_3 => {
            DynamicPlaceholderTextUtil_SetPlaceholderPtr(
                0,
                (*sStorage).displayMonName.as_mut_ptr(),
            );
        }
        MSG_VAR_RELEASE_MON_1 | MSG_VAR_RELEASE_MON_2 | MSG_VAR_RELEASE_MON_3 => {
            DynamicPlaceholderTextUtil_SetPlaceholderPtr(
                0,
                (*sStorage).releaseMonName.as_mut_ptr(),
            );
        }
        MSG_VAR_ITEM_NAME => {
            if IsMovingItem() != 0 {
                txtPtr = StringCopy((*sStorage).itemName.as_mut_ptr(), GetMovingItemName());
            } else {
                txtPtr = StringCopy(
                    (*sStorage).itemName.as_mut_ptr(),
                    (*sStorage).displayMonItemName.as_mut_ptr(),
                );
            }
            while *txtPtr.at(-1) == CHAR_SPACE {
                txtPtr = txtPtr.at(-1);
            }
            *txtPtr = EOS;
            DynamicPlaceholderTextUtil_SetPlaceholderPtr(0, (*sStorage).itemName.as_mut_ptr());
        }
        _ => {}
    }
    DynamicPlaceholderTextUtil_ExpandPlaceholders(
        (*sStorage).messageText.as_mut_ptr(),
        sMessages[id].text,
    );
    FillWindowPixelBuffer(WIN_MESSAGE, 17);
    AddTextPrinterParameterized(
        WIN_MESSAGE,
        FONT_NORMAL,
        (*sStorage).messageText.as_mut_ptr(),
        0,
        1,
        TEXT_SKIP_DRAW,
        None,
    );
    DrawTextBorderOuter(WIN_MESSAGE, 2, 14);
    PutWindowTilemap(WIN_MESSAGE);
    CopyWindowToVram(WIN_MESSAGE, COPYWIN_GFX);
    ScheduleBgCopyTilemapToVram(0);
}
pub(crate) unsafe extern "C" fn ShowYesNoWindow(cursorPos: i8) {
    CreateYesNoMenu((&raw const *sYesNoWindowTemplate).cast_mut(), 11, 14, 0);
    Menu_MoveCursorNoWrapAround(cursorPos);
}
pub(crate) unsafe extern "C" fn ClearBottomWindow() {
    ClearStdWindowAndFrameToTransparent(WIN_MESSAGE, FALSE);
    ScheduleBgCopyTilemapToVram(0);
}
pub(crate) unsafe extern "C" fn AddWallpaperSetsMenu() {
    InitMenu();
    SetMenuText(MENU_SCENERY_1 as u8);
    SetMenuText(MENU_SCENERY_2 as u8);
    SetMenuText(MENU_SCENERY_3 as u8);
    SetMenuText(MENU_ETCETERA as u8);
    if IsWaldaWallpaperUnlocked() != 0 {
        SetMenuText(MENU_FRIENDS as u8);
    }
    AddMenu();
}
pub(crate) unsafe extern "C" fn AddWallpapersMenu(wallpaperSet: u8) {
    InitMenu();
    match wallpaperSet {
        0 => {
            SetMenuText(MENU_FOREST as u8);
            SetMenuText(MENU_CITY);
            SetMenuText(MENU_DESERT);
            SetMenuText(MENU_SAVANNA);
        }
        1 => {
            SetMenuText(MENU_CRAG);
            SetMenuText(MENU_VOLCANO);
            SetMenuText(MENU_SNOW);
            SetMenuText(MENU_CAVE);
        }
        2 => {
            SetMenuText(MENU_BEACH);
            SetMenuText(MENU_SEAFLOOR);
            SetMenuText(MENU_RIVER);
            SetMenuText(MENU_SKY);
        }
        3 => {
            SetMenuText(MENU_POLKADOT);
            SetMenuText(MENU_POKECENTER);
            SetMenuText(MENU_MACHINE);
            SetMenuText(MENU_SIMPLE);
        }
        _ => {}
    }
    AddMenu();
}
pub(crate) unsafe extern "C" fn GetCurrentBoxOption() -> u8 {
    return sCurrentBoxOption;
}
pub(crate) unsafe extern "C" fn InitCursorItemIcon() {
    if IsCursorOnBoxTitle() == 0 {
        if sInPartyMenu != 0 {
            TryLoadItemIconAtPos(CURSOR_AREA_IN_PARTY as u8, GetCursorPosition());
        } else {
            TryLoadItemIconAtPos(CURSOR_AREA_IN_BOX, GetCursorPosition());
        }
    }
    if sMovingItemId != ITEM_NONE {
        InitItemIconInCursor(sMovingItemId);
        StartCursorAnim(CURSOR_ANIM_FIST);
    }
}
pub(crate) unsafe extern "C" fn InitMonIconFields() {
    let mut i: u16 = 0;
    LoadMonIconPalettes();
    i = 0;
    while (i as i32) < (if 37 >= 40 { 37 } else { 40 }) {
        (*sStorage).numIconsPerSpecies[i] = 0;
        i += 1;
    }
    i = 0;
    while (i as i32) < (if 37 >= 40 { 37 } else { 40 }) {
        (*sStorage).iconSpeciesList[i] = SPECIES_NONE;
        i += 1;
    }
    i = 0;
    while i < PARTY_SIZE as u16 {
        (*sStorage).partySprites[i] = null_mut();
        i += 1;
    }
    i = 0;
    while i < IN_BOX_COUNT as u16 {
        (*sStorage).boxMonsSprites[i] = null_mut();
        i += 1;
    }
    (*sStorage).movingMonSprite = null_mut();
    (*sStorage).unkUnused1 = 0;
}
pub(crate) unsafe extern "C" fn GetMonIconPriorityByCursorPos() -> u8 {
    return (if IsCursorInBox() != 0 { 2 } else { 1 }) as u8;
}
pub(crate) unsafe extern "C" fn CreateMovingMonIcon() {
    let mut personality: u32 = GetMonData2(&raw mut (*sStorage).movingMon, MON_DATA_PERSONALITY);
    let mut species: u16 =
        GetMonData2(&raw mut (*sStorage).movingMon, MON_DATA_SPECIES_OR_EGG) as u16;
    let mut priority: u8 = GetMonIconPriorityByCursorPos();
    (*sStorage).movingMonSprite = CreateMonIconSprite(species, personality, 0, 0, priority, 7);
    (*(*sStorage).movingMonSprite).callback = Some(SpriteCB_HeldMon);
}
pub(crate) unsafe extern "C" fn InitBoxMonSprites(boxId: u8) {
    let mut boxPosition: u8 = 0;
    let mut i: u16 = 0;
    let mut j: u16 = 0;
    let mut count: u16 = 0;
    let mut species: u16 = 0;
    let mut personality: u32 = 0;
    count = 0;
    boxPosition = 0;
    i = 0;
    while i < IN_BOX_ROWS as u16 {
        j = 0;
        while j < IN_BOX_COLUMNS as u16 {
            species = GetBoxMonDataAt(boxId, boxPosition, MON_DATA_SPECIES_OR_EGG) as u16;
            if species != SPECIES_NONE {
                personality = GetBoxMonDataAt(boxId, boxPosition, MON_DATA_PERSONALITY);
                (*sStorage).boxMonsSprites[count] = CreateMonIconSprite(
                    species,
                    personality,
                    8 * (3 * j as i16) + 100,
                    8 * (3 * i as i16) + 44,
                    2,
                    19 - j as u8,
                );
            } else {
                (*sStorage).boxMonsSprites[count] = null_mut();
            }
            boxPosition += 1;
            count += 1;
            j += 1;
        }
        i += 1;
    }
    if (*sStorage).boxOption == OPTION_MOVE_ITEMS {
        boxPosition = 0;
        while boxPosition < IN_BOX_COUNT as u8 {
            if GetBoxMonDataAt(boxId, boxPosition, MON_DATA_HELD_ITEM) == ITEM_NONE as u32 {
                (*(*sStorage).boxMonsSprites[boxPosition])
                    .oam
                    .set_objMode(ST_OAM_OBJ_BLEND);
            }
            boxPosition += 1;
        }
    }
}
pub(crate) unsafe extern "C" fn CreateBoxMonIconAtPos(boxPosition: u8) {
    let mut species: u16 = GetCurrentBoxMonData(boxPosition, MON_DATA_SPECIES_OR_EGG) as u16;
    if species != SPECIES_NONE {
        let mut x: i16 = 8 * (3 * (boxPosition as i32 % 6) as i16) + 100;
        let mut y: i16 = 8 * (3 * (boxPosition as i32 / 6) as i16) + 44;
        let mut personality: u32 = GetCurrentBoxMonData(boxPosition, MON_DATA_PERSONALITY);
        (*sStorage).boxMonsSprites[boxPosition] = CreateMonIconSprite(
            species,
            personality,
            x,
            y,
            2,
            19 - (boxPosition as i32 % 6) as u8,
        );
        if (*sStorage).boxOption == OPTION_MOVE_ITEMS {
            (*(*sStorage).boxMonsSprites[boxPosition])
                .oam
                .set_objMode(ST_OAM_OBJ_BLEND);
        }
    }
}
pub(crate) unsafe extern "C" fn StartBoxMonIconsScrollOut(speed: i16) {
    let mut i: u16 = 0;
    i = 0;
    while i < IN_BOX_COUNT as u16 {
        if !(*sStorage).boxMonsSprites[i].is_null() {
            (*(*sStorage).boxMonsSprites[i]).data[2] = speed;
            (*(*sStorage).boxMonsSprites[i]).data[4] = 1;
            (*(*sStorage).boxMonsSprites[i]).callback = Some(SpriteCB_BoxMonIconScrollOut);
        }
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_BoxMonIconScrollIn(sprite: *mut Sprite) {
    if (*sprite).data[1] != 0 {
        (*sprite).data[1] -= 1;
        (*sprite).x += (*sprite).data[2];
    } else {
        (*sStorage).iconScrollNumIncoming -= 1;
        (*sprite).x = (*sprite).data[3];
        (*sprite).callback = Some(SpriteCallbackDummy);
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_BoxMonIconScrollOut(sprite: *mut Sprite) {
    if (*sprite).data[4] != 0 {
        (*sprite).data[4] -= 1;
    } else {
        (*sprite).x += (*sprite).data[2];
        (*sprite).data[5] = (*sprite).x + (*sprite).x2;
        if (*sprite).data[5] <= 68 || (*sprite).data[5] >= 252 {
            (*sprite).callback = Some(SpriteCallbackDummy);
        }
    }
}
pub(crate) unsafe extern "C" fn DestroyBoxMonIconsInColumn(column: u8) {
    let mut row: u16 = 0;
    let mut boxPosition: u8 = column;
    row = 0;
    while row < IN_BOX_ROWS as u16 {
        if !(*sStorage).boxMonsSprites[boxPosition].is_null() {
            DestroyBoxMonIcon((*sStorage).boxMonsSprites[boxPosition]);
            (*sStorage).boxMonsSprites[boxPosition] = null_mut();
        }
        boxPosition += IN_BOX_COLUMNS;
        row += 1;
    }
}
pub(crate) unsafe extern "C" fn CreateBoxMonIconsInColumn(
    column: u8,
    distance: u16,
    speed: i16,
) -> u8 {
    let mut i: i32 = 0;
    let mut y: u16 = 44;
    let mut xDest: i16 = 8 * (3 * column as i16) + 100;
    let mut x: u16 = xDest as u16 - (distance + 1) * speed as u16;
    let mut subpriority: u8 = 19 - column;
    let mut iconsCreated: u8 = 0;
    let mut boxPosition: u8 = column;
    if (*sStorage).boxOption != OPTION_MOVE_ITEMS {
        i = 0;
        while i < IN_BOX_ROWS {
            if (*sStorage).boxSpecies[boxPosition] != SPECIES_NONE {
                (*sStorage).boxMonsSprites[boxPosition] = CreateMonIconSprite(
                    (*sStorage).boxSpecies[boxPosition],
                    (*sStorage).boxPersonalities[boxPosition],
                    x as i16,
                    y as i16,
                    2,
                    subpriority,
                );
                if !(*sStorage).boxMonsSprites[boxPosition].is_null() {
                    (*(*sStorage).boxMonsSprites[boxPosition]).data[1] = distance as i16;
                    (*(*sStorage).boxMonsSprites[boxPosition]).data[2] = speed;
                    (*(*sStorage).boxMonsSprites[boxPosition]).data[3] = xDest;
                    (*(*sStorage).boxMonsSprites[boxPosition]).callback =
                        Some(SpriteCB_BoxMonIconScrollIn);
                    iconsCreated += 1;
                }
            }
            boxPosition += IN_BOX_COLUMNS;
            y += 24;
            i += 1;
        }
    } else {
        i = 0;
        while i < IN_BOX_ROWS {
            if (*sStorage).boxSpecies[boxPosition] != SPECIES_NONE {
                (*sStorage).boxMonsSprites[boxPosition] = CreateMonIconSprite(
                    (*sStorage).boxSpecies[boxPosition],
                    (*sStorage).boxPersonalities[boxPosition],
                    x as i16,
                    y as i16,
                    2,
                    subpriority,
                );
                if !(*sStorage).boxMonsSprites[boxPosition].is_null() {
                    (*(*sStorage).boxMonsSprites[boxPosition]).data[1] = distance as i16;
                    (*(*sStorage).boxMonsSprites[boxPosition]).data[2] = speed;
                    (*(*sStorage).boxMonsSprites[boxPosition]).data[3] = xDest;
                    (*(*sStorage).boxMonsSprites[boxPosition]).callback =
                        Some(SpriteCB_BoxMonIconScrollIn);
                    if GetBoxMonDataAt((*sStorage).incomingBoxId, boxPosition, MON_DATA_HELD_ITEM)
                        == ITEM_NONE as u32
                    {
                        (*(*sStorage).boxMonsSprites[boxPosition])
                            .oam
                            .set_objMode(ST_OAM_OBJ_BLEND);
                    }
                    iconsCreated += 1;
                }
            }
            boxPosition += IN_BOX_COLUMNS;
            y += 24;
            i += 1;
        }
    }
    return iconsCreated;
}
pub(crate) unsafe extern "C" fn InitBoxMonIconScroll(boxId: u8, direction: i8) {
    (*sStorage).iconScrollState = 0;
    (*sStorage).iconScrollToBoxId = boxId;
    (*sStorage).iconScrollDirection = direction;
    (*sStorage).iconScrollDistance = 32;
    (*sStorage).iconScrollSpeed = -(6 * direction as i16);
    (*sStorage).iconScrollNumIncoming = 0;
    GetIncomingBoxMonData(boxId);
    if direction > 0 {
        (*sStorage).iconScrollCurColumn = 0;
    } else {
        (*sStorage).iconScrollCurColumn = 5;
    }
    (*sStorage).iconScrollPos = 24 * (*sStorage).iconScrollCurColumn as i16 + 100;
    StartBoxMonIconsScrollOut((*sStorage).iconScrollSpeed);
}
pub(crate) unsafe extern "C" fn UpdateBoxMonIconScroll() -> u8 {
    if (*sStorage).iconScrollDistance != 0 {
        (*sStorage).iconScrollDistance -= 1;
    }
    match (*sStorage).iconScrollState {
        0 => {
            (*sStorage).iconScrollPos += (*sStorage).iconScrollSpeed;
            if (*sStorage).iconScrollPos <= 64 || (*sStorage).iconScrollPos >= 252 {
                DestroyBoxMonIconsInColumn((*sStorage).iconScrollCurColumn);
                (*sStorage).iconScrollPos += (*sStorage).iconScrollDirection as i16 * 24;
                (*sStorage).iconScrollState += 1;
            }
        }
        1 => {
            (*sStorage).iconScrollPos += (*sStorage).iconScrollSpeed;
            (*sStorage).iconScrollNumIncoming += CreateBoxMonIconsInColumn(
                (*sStorage).iconScrollCurColumn,
                (*sStorage).iconScrollDistance,
                (*sStorage).iconScrollSpeed,
            ) as u16;
            if (*sStorage).iconScrollDirection > 0 && (*sStorage).iconScrollCurColumn == 5
                || (*sStorage).iconScrollDirection < 0 && (*sStorage).iconScrollCurColumn == 0
            {
                (*sStorage).iconScrollState += 1;
            } else {
                (*sStorage).iconScrollCurColumn += (*sStorage).iconScrollDirection as u8;
                (*sStorage).iconScrollState = 0;
            }
        }
        2 => {
            if (*sStorage).iconScrollNumIncoming == 0 {
                (*sStorage).iconScrollDistance += 1;
                return FALSE;
            }
        }
        _ => {
            return FALSE;
        }
    }
    return TRUE;
}
pub(crate) unsafe extern "C" fn GetIncomingBoxMonData(boxId: u8) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut boxPosition: i32 = 0;
    boxPosition = 0;
    i = 0;
    while i < IN_BOX_ROWS {
        j = 0;
        while j < IN_BOX_COLUMNS as i32 {
            (*sStorage).boxSpecies[boxPosition] =
                GetBoxMonDataAt(boxId, boxPosition as u8, MON_DATA_SPECIES_OR_EGG) as u16;
            if (*sStorage).boxSpecies[boxPosition] != SPECIES_NONE {
                (*sStorage).boxPersonalities[boxPosition] =
                    GetBoxMonDataAt(boxId, boxPosition as u8, MON_DATA_PERSONALITY);
            }
            boxPosition += 1;
            j += 1;
        }
        i += 1;
    }
    (*sStorage).incomingBoxId = boxId;
}
pub(crate) unsafe extern "C" fn DestroyBoxMonIconAtPosition(boxPosition: u8) {
    if !(*sStorage).boxMonsSprites[boxPosition].is_null() {
        DestroyBoxMonIcon((*sStorage).boxMonsSprites[boxPosition]);
        (*sStorage).boxMonsSprites[boxPosition] = null_mut();
    }
}
pub(crate) unsafe extern "C" fn SetBoxMonIconObjMode(boxPosition: u8, objMode: u8) {
    if !(*sStorage).boxMonsSprites[boxPosition].is_null() {
        (*(*sStorage).boxMonsSprites[boxPosition])
            .oam
            .set_objMode(objMode as u32);
    }
}
pub(crate) unsafe extern "C" fn CreatePartyMonsSprites(visible: u8) {
    let mut i: u16 = 0;
    let mut count: u16 = 0;
    let mut species: u16 = GetMonData2(&raw mut gPlayerParty[0], MON_DATA_SPECIES_OR_EGG) as u16;
    let mut personality: u32 = GetMonData2(&raw mut gPlayerParty[0], MON_DATA_PERSONALITY);
    (*sStorage).partySprites[0] = CreateMonIconSprite(species, personality, 104, 64, 1, 12);
    count = 1;
    i = 1;
    while i < PARTY_SIZE as u16 {
        species = GetMonData2(&raw mut gPlayerParty[i], MON_DATA_SPECIES_OR_EGG) as u16;
        if species != SPECIES_NONE {
            personality = GetMonData2(&raw mut gPlayerParty[i], MON_DATA_PERSONALITY);
            (*sStorage).partySprites[i] = CreateMonIconSprite(
                species,
                personality,
                152,
                8 * (3 * (i as i16 - 1)) + 16,
                1,
                12,
            );
            count += 1;
        } else {
            (*sStorage).partySprites[i] = null_mut();
        }
        i += 1;
    }
    if visible == 0 {
        i = 0;
        while i < count {
            (*(*sStorage).partySprites[i]).y -= DISPLAY_HEIGHT as i16;
            (*(*sStorage).partySprites[i]).set_invisible(TRUE as u16);
            i += 1;
        }
    }
    if (*sStorage).boxOption == OPTION_MOVE_ITEMS {
        i = 0;
        while i < PARTY_SIZE as u16 {
            if !(*sStorage).partySprites[i].is_null()
                && GetMonData2(&raw mut gPlayerParty[i], MON_DATA_HELD_ITEM) == ITEM_NONE as u32
            {
                (*(*sStorage).partySprites[i])
                    .oam
                    .set_objMode(ST_OAM_OBJ_BLEND);
            }
            i += 1;
        }
    }
}
pub(crate) unsafe extern "C" fn CompactPartySprites() {
    let mut i: u16 = 0;
    let mut targetSlot: u16 = 0;
    (*sStorage).numPartyToCompact = 0;
    i = 0;
    targetSlot = 0;
    while i < PARTY_SIZE as u16 {
        if !(*sStorage).partySprites[i].is_null() {
            if i != targetSlot {
                MovePartySpriteToNextSlot((*sStorage).partySprites[i], targetSlot);
                (*sStorage).partySprites[i] = null_mut();
                (*sStorage).numPartyToCompact += 1;
            }
            targetSlot += 1;
        }
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn GetNumPartySpritesCompacting() -> u8 {
    return (*sStorage).numPartyToCompact;
}
pub(crate) unsafe extern "C" fn MovePartySpriteToNextSlot(sprite: *mut Sprite, partyId: u16) {
    let mut x: i16 = 0;
    let mut y: i16 = 0;
    (*sprite).data[1] = partyId as i16;
    if partyId == 0 {
        x = 104;
        y = 64;
    } else {
        x = 152;
        y = 8 * (3 * (partyId as i16 - 1)) + 16;
    }
    (*sprite).data[2] = (*sprite).x as u16 as i16 * 8;
    (*sprite).data[3] = (*sprite).y as u16 as i16 * 8;
    (*sprite).data[4] = ((x as i32 * 8 - (*sprite).data[2] as i32) / 8) as i16;
    (*sprite).data[5] = ((y as i32 * 8 - (*sprite).data[3] as i32) / 8) as i16;
    (*sprite).data[6] = 8;
    (*sprite).callback = Some(SpriteCB_MovePartyMonToNextSlot);
}
pub(crate) unsafe extern "C" fn SpriteCB_MovePartyMonToNextSlot(sprite: *mut Sprite) {
    if (*sprite).data[6] != 0 {
        let mut x: i16 = {
            (*sprite).data[2] += (*sprite).data[4];
            (*sprite).data[2]
        };
        let mut y: i16 = {
            (*sprite).data[3] += (*sprite).data[5];
            (*sprite).data[3]
        };
        (*sprite).x = (x as u32 / 8) as i16;
        (*sprite).y = (y as u32 / 8) as i16;
        (*sprite).data[6] -= 1;
    } else {
        if (*sprite).data[1] == 0 {
            (*sprite).x = 104;
            (*sprite).y = 64;
        } else {
            (*sprite).x = 152;
            (*sprite).y = 8 * (3 * ((*sprite).data[1] - 1)) + 16;
        }
        (*sprite).callback = Some(SpriteCallbackDummy);
        (*sStorage).partySprites[(*sprite).data[1]] = sprite;
        (*sStorage).numPartyToCompact -= 1;
    }
}
pub(crate) unsafe extern "C" fn DestroyMovingMonIcon() {
    if !(*sStorage).movingMonSprite.is_null() {
        DestroyBoxMonIcon((*sStorage).movingMonSprite);
        (*sStorage).movingMonSprite = null_mut();
    }
}
pub(crate) unsafe extern "C" fn MovePartySprites(yDelta: i16) {
    let mut i: u16 = 0;
    let mut posY: u16 = 0;
    i = 0;
    while i < PARTY_SIZE as u16 {
        if !(*sStorage).partySprites[i].is_null() {
            (*(*sStorage).partySprites[i]).y += yDelta;
            posY = (*(*sStorage).partySprites[i]).y as u16
                + (*(*sStorage).partySprites[i]).y2 as u16
                + (*(*sStorage).partySprites[i]).centerToCornerVecY as u16;
            posY += 16;
            if posY > 192 {
                (*(*sStorage).partySprites[i]).set_invisible(TRUE as u16);
            } else {
                (*(*sStorage).partySprites[i]).set_invisible(FALSE as u16);
            }
        }
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn DestroyPartyMonIcon(partyId: u8) {
    if !(*sStorage).partySprites[partyId].is_null() {
        DestroyBoxMonIcon((*sStorage).partySprites[partyId]);
        (*sStorage).partySprites[partyId] = null_mut();
    }
}
pub(crate) unsafe extern "C" fn DestroyAllPartyMonIcons() {
    let mut i: u16 = 0;
    i = 0;
    while i < PARTY_SIZE as u16 {
        if !(*sStorage).partySprites[i].is_null() {
            DestroyBoxMonIcon((*sStorage).partySprites[i]);
            (*sStorage).partySprites[i] = null_mut();
        }
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn SetPartyMonIconObjMode(partyId: u8, objMode: u8) {
    if !(*sStorage).partySprites[partyId].is_null() {
        (*(*sStorage).partySprites[partyId])
            .oam
            .set_objMode(objMode as u32);
    }
}
pub(crate) unsafe extern "C" fn SetMovingMonSprite(mode: u8, id: u8) {
    if mode == MODE_PARTY {
        (*sStorage).movingMonSprite = (*sStorage).partySprites[id];
        (*sStorage).partySprites[id] = null_mut();
    } else if mode == MODE_BOX {
        (*sStorage).movingMonSprite = (*sStorage).boxMonsSprites[id];
        (*sStorage).boxMonsSprites[id] = null_mut();
    } else {
        return;
    }
    (*(*sStorage).movingMonSprite).callback = Some(SpriteCB_HeldMon);
    (*(*sStorage).movingMonSprite)
        .oam
        .set_priority(GetMonIconPriorityByCursorPos() as u16);
    (*(*sStorage).movingMonSprite).subpriority = 7;
}
pub(crate) unsafe extern "C" fn SetPlacedMonSprite(boxId: u8, position: u8) {
    if boxId == TOTAL_BOXES_COUNT {
        (*sStorage).partySprites[position] = (*sStorage).movingMonSprite;
        (*(*sStorage).partySprites[position]).oam.set_priority(1);
        (*(*sStorage).partySprites[position]).subpriority = 12;
    } else {
        (*sStorage).boxMonsSprites[position] = (*sStorage).movingMonSprite;
        (*(*sStorage).boxMonsSprites[position]).oam.set_priority(2);
        (*(*sStorage).boxMonsSprites[position]).subpriority = 19 - (position as i32 % 6) as u8;
    }
    (*(*sStorage).movingMonSprite).callback = Some(SpriteCallbackDummy);
    (*sStorage).movingMonSprite = null_mut();
}
pub(crate) unsafe extern "C" fn SaveMonSpriteAtPos(boxId: u8, position: u8) {
    if boxId == TOTAL_BOXES_COUNT {
        (*sStorage).shiftMonSpritePtr = &raw mut (*sStorage).partySprites[position];
    } else {
        (*sStorage).shiftMonSpritePtr = &raw mut (*sStorage).boxMonsSprites[position];
    }
    (*(*sStorage).movingMonSprite).callback = Some(SpriteCallbackDummy);
    (*sStorage).shiftTimer = 0;
}
pub(crate) unsafe extern "C" fn MoveShiftingMons() -> u8 {
    if (*sStorage).shiftTimer == 16 {
        return FALSE;
    }
    (*sStorage).shiftTimer += 1;
    if (*sStorage).shiftTimer as i32 & 1 != 0 {
        (*(*(*sStorage).shiftMonSpritePtr)).y -= 1;
        (*(*sStorage).movingMonSprite).y += 1;
    }
    (*(*(*sStorage).shiftMonSpritePtr)).x2 = gSineTable[(*sStorage).shiftTimer as i32 * 8] / 16;
    (*(*sStorage).movingMonSprite).x2 = -(gSineTable[(*sStorage).shiftTimer as i32 * 8] / 16);
    if (*sStorage).shiftTimer == 8 {
        (*(*sStorage).movingMonSprite)
            .oam
            .set_priority((*(*(*sStorage).shiftMonSpritePtr)).oam.priority());
        (*(*sStorage).movingMonSprite).subpriority =
            (*(*(*sStorage).shiftMonSpritePtr)).subpriority;
        (*(*(*sStorage).shiftMonSpritePtr))
            .oam
            .set_priority(GetMonIconPriorityByCursorPos() as u16);
        (*(*(*sStorage).shiftMonSpritePtr)).subpriority = 7;
    }
    if (*sStorage).shiftTimer == 16 {
        let mut sprite: *mut Sprite = (*sStorage).movingMonSprite;
        (*sStorage).movingMonSprite = *(*sStorage).shiftMonSpritePtr;
        *(*sStorage).shiftMonSpritePtr = sprite;
        (*(*sStorage).movingMonSprite).callback = Some(SpriteCB_HeldMon);
        (*(*(*sStorage).shiftMonSpritePtr)).callback = Some(SpriteCallbackDummy);
    }
    return TRUE;
}
pub(crate) unsafe extern "C" fn SetReleaseMon(mode: u8, position: u8) {
    match mode {
        MODE_PARTY => {
            (*sStorage).releaseMonSpritePtr = &raw mut (*sStorage).partySprites[position];
        }
        MODE_BOX => {
            (*sStorage).releaseMonSpritePtr = &raw mut (*sStorage).boxMonsSprites[position];
        }
        MODE_MOVE => {
            (*sStorage).releaseMonSpritePtr = &raw mut (*sStorage).movingMonSprite;
        }
        _ => {
            return;
        }
    }
    if !(*(*sStorage).releaseMonSpritePtr).is_null() {
        InitSpriteAffineAnim(*(*sStorage).releaseMonSpritePtr);
        (*(*(*sStorage).releaseMonSpritePtr))
            .oam
            .set_affineMode(ST_OAM_AFFINE_NORMAL);
        (*(*(*sStorage).releaseMonSpritePtr)).affineAnims =
            sAffineAnims_ReleaseMon.as_ptr().cast_mut();
        StartSpriteAffineAnim(*(*sStorage).releaseMonSpritePtr, RELEASE_ANIM_RELEASE);
    }
}
pub(crate) unsafe extern "C" fn TryHideReleaseMonSprite() -> u8 {
    if (*(*sStorage).releaseMonSpritePtr).is_null()
        || (*(*(*sStorage).releaseMonSpritePtr)).invisible() != 0
    {
        return FALSE;
    }
    if (*(*(*sStorage).releaseMonSpritePtr)).affineAnimEnded() != 0 {
        (*(*(*sStorage).releaseMonSpritePtr)).set_invisible(TRUE as u16);
    }
    return TRUE;
}
pub(crate) unsafe extern "C" fn DestroyReleaseMonIcon() {
    if !(*(*sStorage).releaseMonSpritePtr).is_null() {
        FreeOamMatrix((*(*(*sStorage).releaseMonSpritePtr)).oam.matrixNum() as u8);
        DestroyBoxMonIcon(*(*sStorage).releaseMonSpritePtr);
        *(*sStorage).releaseMonSpritePtr = null_mut();
    }
}
pub(crate) unsafe extern "C" fn ReshowReleaseMon() {
    if !(*(*sStorage).releaseMonSpritePtr).is_null() {
        (*(*(*sStorage).releaseMonSpritePtr)).set_invisible(FALSE as u16);
        StartSpriteAffineAnim(*(*sStorage).releaseMonSpritePtr, RELEASE_ANIM_CAME_BACK);
    }
}
pub(crate) unsafe extern "C" fn ResetReleaseMonSpritePtr() -> u8 {
    if (*sStorage).releaseMonSpritePtr.is_null() {
        return FALSE;
    }
    if (*(*(*sStorage).releaseMonSpritePtr)).affineAnimEnded() != 0 {
        (*sStorage).releaseMonSpritePtr = null_mut();
    }
    return TRUE;
}
pub(crate) unsafe extern "C" fn SetMovingMonPriority(priority: u8) {
    (*(*sStorage).movingMonSprite)
        .oam
        .set_priority(priority as u16);
}
pub(crate) unsafe extern "C" fn SpriteCB_HeldMon(sprite: *mut Sprite) {
    (*sprite).x = (*(*sStorage).cursorSprite).x;
    (*sprite).y = (*(*sStorage).cursorSprite).y + (*(*sStorage).cursorSprite).y2 + 4;
}
pub(crate) unsafe extern "C" fn TryLoadMonIconTiles(species: u16) -> u16 {
    let mut i: u16 = 0;
    let mut offset: u16 = 0;
    i = 0;
    while (i as i32) < (if 37 >= 40 { 37 } else { 40 }) {
        if (*sStorage).iconSpeciesList[i] == species {
            break;
        }
        i += 1;
    }
    if i as i32 == (if 37 >= 40 { 37 } else { 40 }) {
        i = 0;
        while (i as i32) < (if 37 >= 40 { 37 } else { 40 }) {
            if (*sStorage).iconSpeciesList[i] == 0 {
                break;
            }
            i += 1;
        }
        if i as i32 == (if 37 >= 40 { 37 } else { 40 }) {
            return 0xFFFF;
        }
    }
    (*sStorage).iconSpeciesList[i] = species;
    (*sStorage).numIconsPerSpecies[i] += 1;
    offset = 16 * i;
    CpuSet(
        GetMonIconTiles(species, TRUE as u32) as *mut c_void,
        (OBJ_VRAM0 as usize as *mut c_void as *mut u8).at(offset as i32 * 32) as *mut c_void,
        0x4000080,
    );
    return offset;
}
pub(crate) unsafe extern "C" fn RemoveSpeciesFromIconList(species: u16) {
    let mut i: u16 = 0;
    i = 0;
    while (i as i32) < (if 37 >= 40 { 37 } else { 40 }) {
        if (*sStorage).iconSpeciesList[i] == species {
            if ({
                (*sStorage).numIconsPerSpecies[i] -= 1;
                (*sStorage).numIconsPerSpecies[i]
            }) == 0
            {
                (*sStorage).iconSpeciesList[i] = SPECIES_NONE;
            }
            break;
        }
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn CreateMonIconSprite(
    mut species: u16,
    personality: u32,
    x: i16,
    y: i16,
    oamPriority: u8,
    subpriority: u8,
) -> *mut Sprite {
    let mut tileNum: u16 = 0;
    let mut spriteId: u8 = 0;
    let mut template: SpriteTemplate = zeroed();
    template = *sSpriteTemplate_MonIcon;
    species = GetIconSpecies(species, personality);
    template.paletteTag = PALTAG_MON_ICON_0 + gMonIconPaletteIndices[species] as u16;
    tileNum = TryLoadMonIconTiles(species);
    if tileNum == 0xFFFF {
        return null_mut();
    }
    spriteId = CreateSprite(&raw mut template, x, y, subpriority);
    if spriteId == MAX_SPRITES {
        RemoveSpeciesFromIconList(species);
        return null_mut();
    }
    gSprites[spriteId].oam.set_tileNum(tileNum);
    gSprites[spriteId].oam.set_priority(oamPriority as u16);
    gSprites[spriteId].data[0] = species as i16;
    return &raw mut gSprites[spriteId];
}
pub(crate) unsafe extern "C" fn DestroyBoxMonIcon(sprite: *mut Sprite) {
    RemoveSpeciesFromIconList((*sprite).data[0] as u16);
    DestroySprite(sprite);
}
pub(crate) unsafe extern "C" fn CreateInitBoxTask(boxId: u8) {
    let mut taskId: u8 = CreateTask(Some(Task_InitBox), 2);
    gTasks[taskId].data[2] = boxId as i16;
}
pub(crate) unsafe extern "C" fn IsInitBoxActive() -> u8 {
    return FuncIsActiveTask(Some(Task_InitBox));
}
pub(crate) unsafe extern "C" fn Task_InitBox(taskId: u8) {
    let mut task: *mut Task = &raw mut gTasks[taskId];
    match (*task).data[0] {
        0 => {
            (*sStorage).wallpaperOffset = 0;
            (*sStorage).bg2_X = 0;
            (*task).data[1] = RequestDma3Fill(
                0,
                (*sStorage).wallpaperBgTilemapBuffer.as_mut_ptr() as *mut c_void,
                4096,
                1,
            );
        }
        1 => {
            if CheckForSpaceForDma3Request((*task).data[1]) == -1 {
                return;
            }
            SetBgTilemapBuffer(
                2,
                (*sStorage).wallpaperBgTilemapBuffer.as_mut_ptr() as *mut c_void,
            );
            ShowBg(2);
        }
        2 => {
            LoadWallpaperGfx((*task).data[2] as u8, 0);
        }
        3 => {
            if WaitForWallpaperGfxLoad() == 0 {
                return;
            }
            InitBoxTitle((*task).data[2] as u8);
            CreateBoxScrollArrows();
            InitBoxMonSprites((*task).data[2] as u8);
            SetGpuReg(REG_OFFSET_BG2CNT, 23306);
        }
        4 => {
            DestroyTask(taskId);
        }
        _ => {
            (*task).data[0] = 0;
            return;
        }
    }
    (*task).data[0] += 1;
}
pub(crate) unsafe extern "C" fn SetUpScrollToBox(boxId: u8) {
    let mut direction: i8 = DetermineBoxScrollDirection(boxId);
    (*sStorage).scrollSpeed = (if direction > 0 { 6 } else { -6 }) as i16;
    (*sStorage).scrollUnused1 = (if direction > 0 { 1 } else { 2 }) as u8;
    (*sStorage).scrollTimer = 32;
    (*sStorage).scrollToBoxIdUnused = boxId;
    (*sStorage).scrollUnused2 = (if direction <= 0 { 5 } else { 0 }) as u16;
    (*sStorage).scrollDirectionUnused = direction as i16;
    (*sStorage).scrollUnused3 = (if direction > 0 { 264 } else { 56 }) as u16;
    (*sStorage).scrollUnused4 = (if direction <= 0 { 5 } else { 0 }) as u16;
    (*sStorage).scrollUnused5 = 0;
    (*sStorage).scrollUnused6 = 2;
    (*sStorage).scrollToBoxId = boxId;
    (*sStorage).scrollDirection = direction;
    (*sStorage).scrollState = 0;
}
pub(crate) unsafe extern "C" fn ScrollToBox() -> u8 {
    let mut iconsScrolling: u8 = 0;
    'l1: {
        let sw1: u8 = (*sStorage).scrollState;
        let mut fall = false;
        if sw1 == 0 {
            fall = true;
            LoadWallpaperGfx((*sStorage).scrollToBoxId, (*sStorage).scrollDirection);
            (*sStorage).scrollState += 1;
        }
        if fall || sw1 == 1 {
            fall = true;
            if WaitForWallpaperGfxLoad() == 0 {
                return TRUE;
            }
            InitBoxMonIconScroll((*sStorage).scrollToBoxId, (*sStorage).scrollDirection);
            CreateIncomingBoxTitle((*sStorage).scrollToBoxId, (*sStorage).scrollDirection);
            StartBoxScrollArrowsSlide((*sStorage).scrollDirection);
            break 'l1;
        }
        if sw1 == 2 {
            fall = true;
            iconsScrolling = UpdateBoxMonIconScroll();
            if (*sStorage).scrollTimer != 0 {
                (*sStorage).bg2_X += (*sStorage).scrollSpeed as u16;
                if ({
                    (*sStorage).scrollTimer -= 1;
                    (*sStorage).scrollTimer
                }) != 0
                {
                    return TRUE;
                }
                CycleBoxTitleSprites();
                StopBoxScrollArrowsSlide();
            }
            return iconsScrolling;
        }
    }
    (*sStorage).scrollState += 1;
    return TRUE;
}
pub(crate) unsafe extern "C" fn DetermineBoxScrollDirection(boxId: u8) -> i8 {
    let mut i: u8 = 0;
    let mut currentBox: u8 = StorageGetCurrentBox();
    i = 0;
    while currentBox != boxId {
        currentBox += 1;
        if currentBox >= TOTAL_BOXES_COUNT {
            currentBox = 0;
        }
        i += 1;
    }
    return (if i < 7 { 1 } else { -1 }) as i8;
}
pub(crate) unsafe extern "C" fn SetWallpaperForCurrentBox(wallpaperId: u8) {
    let mut boxId: u8 = StorageGetCurrentBox();
    SetBoxWallpaper(boxId, wallpaperId);
    (*sStorage).wallpaperChangeState = 0;
}
pub(crate) unsafe extern "C" fn DoWallpaperGfxChange() -> u8 {
    match (*sStorage).wallpaperChangeState {
        0 => {
            BeginNormalPaletteFade((*sStorage).wallpaperPalBits, 1, 0, 16, 65535);
            (*sStorage).wallpaperChangeState += 1;
        }
        1 => {
            if UpdatePaletteFade() == 0 {
                let mut curBox: u8 = StorageGetCurrentBox();
                LoadWallpaperGfx(curBox, 0);
                (*sStorage).wallpaperChangeState += 1;
            }
        }
        2 => {
            if WaitForWallpaperGfxLoad() == TRUE as u32 {
                CycleBoxTitleColor();
                BeginNormalPaletteFade((*sStorage).wallpaperPalBits, 1, 16, 0, 65535);
                (*sStorage).wallpaperChangeState += 1;
            }
        }
        3 => {
            if UpdatePaletteFade() == 0 {
                (*sStorage).wallpaperChangeState += 1;
            }
        }
        4 => {
            return FALSE;
        }
        _ => {}
    }
    return TRUE;
}
pub(crate) unsafe extern "C" fn LoadWallpaperGfx(boxId: u8, direction: i8) {
    let mut wallpaperId: u8 = 0;
    let mut wallpaper: *mut Wallpaper = null_mut();
    let mut iconGfx: *mut c_void = null_mut();
    let mut tilesSize: u32 = 0;
    let mut iconSize: u32 = 0;
    (*sStorage).wallpaperLoadState = 0;
    (*sStorage).wallpaperLoadBoxId = boxId;
    (*sStorage).wallpaperLoadDir = direction;
    if (*sStorage).wallpaperLoadDir != 0 {
        (*sStorage).wallpaperOffset = ((*sStorage).wallpaperOffset == 0) as u8;
        TrimOldWallpaper((*sStorage).wallpaperBgTilemapBuffer.as_mut_ptr() as *mut c_void);
    }
    wallpaperId = GetBoxWallpaper((*sStorage).wallpaperLoadBoxId);
    if wallpaperId != WALLPAPER_FRIENDS as u8 {
        wallpaper = (&raw const sWallpapers[wallpaperId]).cast_mut();
        LZ77UnCompWram(
            (*wallpaper).tilemap,
            (*sStorage).wallpaperTilemap.as_mut_ptr() as *mut c_void,
        );
        DrawWallpaper(
            (*sStorage).wallpaperTilemap.as_mut_ptr() as *mut c_void,
            (*sStorage).wallpaperLoadDir,
            (*sStorage).wallpaperOffset,
        );
        if (*sStorage).wallpaperLoadDir != 0 {
            LoadPalette(
                (*wallpaper).palettes as *mut c_void,
                64 + (0x000 + (*sStorage).wallpaperOffset as u16 * 2 * 16),
                64,
            );
        } else {
            CpuSet(
                (*wallpaper).palettes as *mut c_void,
                &raw mut gPlttBufferUnfaded
                    [64 + (0x000 + (*sStorage).wallpaperOffset as i32 * 2 * 16)]
                    as *mut c_void,
                32,
            );
        }
        (*sStorage).wallpaperTiles =
            malloc_and_decompress((*wallpaper).tiles as *mut c_void, &raw mut tilesSize) as *mut u8;
        LoadBgTiles(
            2,
            (*sStorage).wallpaperTiles as *mut c_void,
            tilesSize as u16,
            ((*sStorage).wallpaperOffset as u16) << 8,
        );
    } else {
        wallpaper = (&raw const sWaldaWallpapers[GetWaldaWallpaperPatternId()]).cast_mut();
        LZ77UnCompWram(
            (*wallpaper).tilemap,
            (*sStorage).wallpaperTilemap.as_mut_ptr() as *mut c_void,
        );
        DrawWallpaper(
            (*sStorage).wallpaperTilemap.as_mut_ptr() as *mut c_void,
            (*sStorage).wallpaperLoadDir,
            (*sStorage).wallpaperOffset,
        );
        CpuSet(
            (*wallpaper).palettes as *mut c_void,
            (*sStorage).wallpaperTilemap.as_mut_ptr() as *mut c_void,
            32,
        );
        CpuSet(
            GetWaldaWallpaperColorsPtr() as *mut c_void,
            &raw mut (*sStorage).wallpaperTilemap[1] as *mut c_void,
            2,
        );
        CpuSet(
            GetWaldaWallpaperColorsPtr() as *mut c_void,
            &raw mut (*sStorage).wallpaperTilemap[17] as *mut c_void,
            2,
        );
        if (*sStorage).wallpaperLoadDir != 0 {
            LoadPalette(
                (*sStorage).wallpaperTilemap.as_mut_ptr() as *mut c_void,
                64 + (0x000 + (*sStorage).wallpaperOffset as u16 * 2 * 16),
                64,
            );
        } else {
            CpuSet(
                (*sStorage).wallpaperTilemap.as_mut_ptr() as *mut c_void,
                &raw mut gPlttBufferUnfaded
                    [64 + (0x000 + (*sStorage).wallpaperOffset as i32 * 2 * 16)]
                    as *mut c_void,
                32,
            );
        }
        (*sStorage).wallpaperTiles =
            malloc_and_decompress((*wallpaper).tiles as *mut c_void, &raw mut tilesSize) as *mut u8;
        iconGfx = malloc_and_decompress(
            sWaldaWallpaperIcons[GetWaldaWallpaperIconId()] as *mut c_void,
            &raw mut iconSize,
        );
        CpuSet(
            iconGfx,
            (*sStorage).wallpaperTiles.at(2048) as *mut c_void,
            0x04000000 | iconSize / 4 & 0x1FFFFF,
        );
        Free(iconGfx);
        LoadBgTiles(
            2,
            (*sStorage).wallpaperTiles as *mut c_void,
            tilesSize as u16,
            ((*sStorage).wallpaperOffset as u16) << 8,
        );
    }
    CopyBgTilemapBufferToVram(2);
}
pub(crate) unsafe extern "C" fn WaitForWallpaperGfxLoad() -> u32 {
    if IsDma3ManagerBusyWithBgCopy() != 0 {
        return FALSE as u32;
    }
    if !(*sStorage).wallpaperTiles.is_null() {
        Free((*sStorage).wallpaperTiles as *mut c_void);
        (*sStorage).wallpaperTiles = null_mut();
    }
    return TRUE as u32;
}
pub(crate) unsafe extern "C" fn DrawWallpaper(tilemap: *mut c_void, direction: i8, offset: u8) {
    let mut tileOffset: i16 = offset as i16 * 256;
    let mut paletteNum: i16 = offset as i16 * 2 + 3;
    let mut x: i16 = ((*sStorage).bg2_X as i32 / 8) as i16 + 10 + direction as i16 * 24 & 0x3F;
    CopyRectToBgTilemapBufferRect(
        2, tilemap, 0, 0, 20, 18, x as u8, 2, 20, 18, 17, tileOffset, paletteNum,
    );
    if direction == 0 {
        return;
    }
    if direction > 0 {
        x += 20;
    } else {
        x -= 4;
    }
    FillBgTilemapBufferRect(2, 0, x as u8, 2, 4, 0x12, 17);
}
pub(crate) unsafe extern "C" fn TrimOldWallpaper(tilemap: *mut c_void) {
    let mut i: u16 = 0;
    let mut dest: *mut u16 = tilemap as *mut u16;
    let mut r3: i16 = ((*sStorage).bg2_X as i32 / 8) as i16 + 30 & 0x3F;
    if r3 <= 31 {
        dest = dest.at(r3 as i32 + 0x260);
    } else {
        dest = dest.at(r3 as i32 + 0x640);
    }
    i = 0;
    while i < 0x2C {
        *({
            let t1 = dest;
            dest = dest.at(1);
            t1
        }) = 0;
        r3 = r3 + 1 & 0x3F;
        if r3 == 0 {
            dest = dest.at(-1056);
        }
        if r3 == 0x20 {
            dest = dest.at(992);
        }
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn InitBoxTitle(boxId: u8) {
    let mut tagIndex: u8 = 0;
    let mut x: i16 = 0;
    let mut i: u16 = 0;
    let mut spriteSheet: SpriteSheet = zeroed();
    spriteSheet.data = (*sStorage).boxTitleTiles.as_mut_ptr() as *mut c_void;
    spriteSheet.size = 0x200;
    spriteSheet.tag = GFXTAG_BOX_TITLE;
    let mut palettes: CArray<SpritePalette, 2> = zeroed();
    palettes[0].data = (*sStorage).boxTitlePal.as_mut_ptr();
    palettes[0].tag = PALTAG_BOX_TITLE;
    let mut wallpaperId: u16 = GetBoxWallpaper(boxId) as u16;
    (*sStorage).boxTitlePal[14] = sBoxTitleColors[wallpaperId][0];
    (*sStorage).boxTitlePal[15] = sBoxTitleColors[wallpaperId][1];
    LoadSpritePalettes(palettes.as_mut_ptr());
    (*sStorage).wallpaperPalBits = 0x3f0;
    tagIndex = IndexOfSpritePaletteTag(PALTAG_BOX_TITLE);
    (*sStorage).boxTitlePalOffset = 0x100 + tagIndex as u16 * 16 + 14;
    (*sStorage).wallpaperPalBits |= shl_i32(0x10000, tagIndex as u32) as u32;
    tagIndex = IndexOfSpritePaletteTag(PALTAG_BOX_TITLE);
    (*sStorage).boxTitleAltPalOffset = 0x100 + tagIndex as u16 * 16 + 14;
    (*sStorage).wallpaperPalBits |= shl_i32(0x10000, tagIndex as u32) as u32;
    StringCopyPadded(
        (*sStorage).boxTitleText.as_mut_ptr(),
        GetBoxNamePtr(boxId),
        0,
        BOX_NAME_LENGTH as u16,
    );
    DrawTextWindowAndBufferTiles(
        (*sStorage).boxTitleText.as_mut_ptr(),
        (*sStorage).boxTitleTiles.as_mut_ptr() as *mut c_void,
        0,
        0,
        2,
    );
    LoadSpriteSheet(&raw mut spriteSheet);
    x = GetBoxTitleBaseX(GetBoxNamePtr(boxId));
    i = 0;
    while i < 2 {
        let mut spriteId: u8 = CreateSprite(
            (&raw const *sSpriteTemplate_BoxTitle).cast_mut(),
            x + i as i16 * 32,
            28,
            24,
        );
        (*sStorage).curBoxTitleSprites[i] = &raw mut gSprites[spriteId];
        StartSpriteAnim((*sStorage).curBoxTitleSprites[i], i as u8);
        i += 1;
    }
    (*sStorage).boxTitleCycleId = 0;
}
pub(crate) unsafe extern "C" fn CreateIncomingBoxTitle(boxId: u8, direction: i8) {
    let mut palOffset: u16 = 0;
    let mut x: i16 = 0;
    let mut adjustedX: i16 = 0;
    let mut i: u16 = 0;
    let mut spriteSheet: SpriteSheet = zeroed();
    spriteSheet.data = (*sStorage).boxTitleTiles.as_mut_ptr() as *mut c_void;
    spriteSheet.size = 0x200;
    spriteSheet.tag = GFXTAG_BOX_TITLE;
    let mut template: SpriteTemplate = zeroed();
    template = *sSpriteTemplate_BoxTitle;
    (*sStorage).boxTitleCycleId = ((*sStorage).boxTitleCycleId == 0) as u8;
    if (*sStorage).boxTitleCycleId == 0 {
        spriteSheet.tag = GFXTAG_BOX_TITLE;
        palOffset = (*sStorage).boxTitlePalOffset;
    } else {
        spriteSheet.tag = GFXTAG_BOX_TITLE_ALT;
        palOffset = (*sStorage).boxTitlePalOffset;
        template.tileTag = GFXTAG_BOX_TITLE_ALT;
        template.paletteTag = PALTAG_BOX_TITLE;
    }
    StringCopyPadded(
        (*sStorage).boxTitleText.as_mut_ptr(),
        GetBoxNamePtr(boxId),
        0,
        BOX_NAME_LENGTH as u16,
    );
    DrawTextWindowAndBufferTiles(
        (*sStorage).boxTitleText.as_mut_ptr(),
        (*sStorage).boxTitleTiles.as_mut_ptr() as *mut c_void,
        0,
        0,
        2,
    );
    LoadSpriteSheet(&raw mut spriteSheet);
    LoadPalette(
        sBoxTitleColors[GetBoxWallpaper(boxId)].as_ptr().cast_mut() as *mut c_void,
        palOffset,
        4,
    );
    x = GetBoxTitleBaseX(GetBoxNamePtr(boxId));
    adjustedX = x;
    adjustedX += direction as i16 * 192;
    i = 0;
    while i < 2 {
        let mut spriteId: u8 = CreateSprite(&raw mut template, i as i16 * 32 + adjustedX, 28, 24);
        (*sStorage).nextBoxTitleSprites[i] = &raw mut gSprites[spriteId];
        (*(*sStorage).nextBoxTitleSprites[i]).data[0] = -(direction as i16) * 6;
        (*(*sStorage).nextBoxTitleSprites[i]).data[1] = i as i16 * 32 + x;
        (*(*sStorage).nextBoxTitleSprites[i]).data[2] = 0;
        (*(*sStorage).nextBoxTitleSprites[i]).callback = Some(SpriteCB_IncomingBoxTitle);
        StartSpriteAnim((*sStorage).nextBoxTitleSprites[i], i as u8);
        (*(*sStorage).curBoxTitleSprites[i]).data[0] = -(direction as i16) * 6;
        (*(*sStorage).curBoxTitleSprites[i]).data[1] = 1;
        (*(*sStorage).curBoxTitleSprites[i]).callback = Some(SpriteCB_OutgoingBoxTitle);
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn CycleBoxTitleSprites() {
    if (*sStorage).boxTitleCycleId == 0 {
        FreeSpriteTilesByTag(GFXTAG_BOX_TITLE_ALT);
    } else {
        FreeSpriteTilesByTag(GFXTAG_BOX_TITLE);
    }
    (*sStorage).curBoxTitleSprites[0] = (*sStorage).nextBoxTitleSprites[0];
    (*sStorage).curBoxTitleSprites[1] = (*sStorage).nextBoxTitleSprites[1];
}
pub(crate) unsafe extern "C" fn SpriteCB_IncomingBoxTitle(sprite: *mut Sprite) {
    if (*sprite).data[2] != 0 {
        (*sprite).data[2] -= 1;
    } else if ({
        (*sprite).x += (*sprite).data[0];
        (*sprite).x
    }) == (*sprite).data[1]
    {
        (*sprite).callback = Some(SpriteCallbackDummy);
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_OutgoingBoxTitle(sprite: *mut Sprite) {
    if (*sprite).data[1] != 0 {
        (*sprite).data[1] -= 1;
    } else {
        (*sprite).x += (*sprite).data[0];
        (*sprite).data[2] = (*sprite).x + (*sprite).x2;
        if (*sprite).data[2] < 64 || (*sprite).data[2] > 256 {
            DestroySprite(sprite);
        }
    }
}
pub(crate) unsafe extern "C" fn CycleBoxTitleColor() {
    let mut boxId: u8 = StorageGetCurrentBox();
    let mut wallpaperId: u8 = GetBoxWallpaper(boxId);
    if (*sStorage).boxTitleCycleId == 0 {
        CpuSet(
            sBoxTitleColors[wallpaperId].as_ptr().cast_mut() as *mut c_void,
            &raw mut gPlttBufferUnfaded[(*sStorage).boxTitlePalOffset] as *mut c_void,
            2,
        );
    } else {
        CpuSet(
            sBoxTitleColors[wallpaperId].as_ptr().cast_mut() as *mut c_void,
            &raw mut gPlttBufferUnfaded[(*sStorage).boxTitleAltPalOffset] as *mut c_void,
            2,
        );
    }
}
pub(crate) unsafe extern "C" fn GetBoxTitleBaseX(string: *mut u8) -> i16 {
    return 176 - (GetStringWidth(FONT_NORMAL, string, 0) / 2) as i16;
}
pub(crate) unsafe extern "C" fn CreateBoxScrollArrows() {
    let mut i: u16 = 0;
    LoadSpriteSheet((&raw const *sSpriteSheet_Arrow).cast_mut());
    i = 0;
    while i < 2 {
        let mut spriteId: u8 = CreateSprite(
            (&raw const *sSpriteTemplate_Arrow).cast_mut(),
            92 + i as i16 * 136,
            28,
            22,
        );
        if spriteId != MAX_SPRITES {
            let mut sprite: *mut Sprite = &raw mut gSprites[spriteId];
            StartSpriteAnim(sprite, i as u8);
            (*sprite).data[3] = (if i == 0 { -1 } else { 1 }) as i16;
            (*sStorage).arrowSprites[i] = sprite;
        }
        i += 1;
    }
    if IsCursorOnBoxTitle() != 0 {
        AnimateBoxScrollArrows(TRUE);
    }
}
pub(crate) unsafe extern "C" fn StartBoxScrollArrowsSlide(direction: i8) {
    let mut i: u16 = 0;
    i = 0;
    while i < 2 {
        (*(*sStorage).arrowSprites[i]).x2 = 0;
        (*(*sStorage).arrowSprites[i]).data[0] = 2;
        i += 1;
    }
    if direction < 0 {
        (*(*sStorage).arrowSprites[0]).data[1] = 29;
        (*(*sStorage).arrowSprites[1]).data[1] = 5;
        (*(*sStorage).arrowSprites[0]).data[2] = 72;
        (*(*sStorage).arrowSprites[1]).data[2] = 72;
    } else {
        (*(*sStorage).arrowSprites[0]).data[1] = 5;
        (*(*sStorage).arrowSprites[1]).data[1] = 29;
        (*(*sStorage).arrowSprites[0]).data[2] = 248;
        (*(*sStorage).arrowSprites[1]).data[2] = 248;
    }
    (*(*sStorage).arrowSprites[0]).data[7] = 0;
    (*(*sStorage).arrowSprites[1]).data[7] = 1;
}
pub(crate) unsafe extern "C" fn StopBoxScrollArrowsSlide() {
    let mut i: u16 = 0;
    i = 0;
    while i < 2 {
        (*(*sStorage).arrowSprites[i]).x = 136 * i as i16 + 92;
        (*(*sStorage).arrowSprites[i]).x2 = 0;
        (*(*sStorage).arrowSprites[i]).set_invisible(FALSE as u16);
        i += 1;
    }
    AnimateBoxScrollArrows(TRUE);
}
pub(crate) unsafe extern "C" fn AnimateBoxScrollArrows(animate: u8) {
    let mut i: u16 = 0;
    if animate != 0 {
        i = 0;
        while i < 2 {
            (*(*sStorage).arrowSprites[i]).data[0] = 1;
            (*(*sStorage).arrowSprites[i]).data[1] = 0;
            (*(*sStorage).arrowSprites[i]).data[2] = 0;
            (*(*sStorage).arrowSprites[i]).data[4] = 0;
            i += 1;
        }
    } else {
        i = 0;
        while i < 2 {
            (*(*sStorage).arrowSprites[i]).data[0] = 0;
            i += 1;
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_Arrow(sprite: *mut Sprite) {
    match (*sprite).data[0] {
        0 => {
            (*sprite).x2 = 0;
        }
        1 => {
            if ({
                (*sprite).data[1] += 1;
                (*sprite).data[1]
            }) > 3
            {
                (*sprite).data[1] = 0;
                (*sprite).x2 += (*sprite).data[3];
                if ({
                    (*sprite).data[2] += 1;
                    (*sprite).data[2]
                }) > 5
                {
                    (*sprite).data[2] = 0;
                    (*sprite).x2 = 0;
                }
            }
        }
        2 => {
            (*sprite).data[0] = 3;
        }
        3 => {
            (*sprite).x -= (*sStorage).scrollSpeed;
            if (*sprite).x <= 72 || (*sprite).x >= 248 {
                (*sprite).set_invisible(TRUE as u16);
            }
            if ({
                (*sprite).data[1] -= 1;
                (*sprite).data[1]
            }) == 0
            {
                (*sprite).x = (*sprite).data[2];
                (*sprite).set_invisible(FALSE as u16);
                (*sprite).data[0] = 4;
            }
        }
        4 => {
            (*sprite).x -= (*sStorage).scrollSpeed;
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn CreateChooseBoxArrows(
    x: u16,
    y: u16,
    mut animId: u8,
    priority: u8,
    subpriority: u8,
) -> *mut Sprite {
    let mut spriteId: u8 = CreateSprite(
        (&raw const *sSpriteTemplate_Arrow).cast_mut(),
        x as i16,
        y as i16,
        subpriority,
    );
    if spriteId == MAX_SPRITES {
        return null_mut();
    }
    animId = (animId as i32 % 2) as u8;
    StartSpriteAnim(&raw mut gSprites[spriteId], animId);
    gSprites[spriteId].oam.set_priority(priority as u16);
    gSprites[spriteId].callback = Some(SpriteCallbackDummy);
    return &raw mut gSprites[spriteId];
}
pub(crate) unsafe extern "C" fn InitCursor() {
    if (*sStorage).boxOption != OPTION_DEPOSIT {
        sCursorArea = CURSOR_AREA_IN_BOX as i8;
    } else {
        sCursorArea = CURSOR_AREA_IN_PARTY;
    }
    sCursorPosition = 0;
    sIsMonBeingMoved = FALSE;
    sMovingMonOrigBoxId = 0;
    sMovingMonOrigBoxPos = 0;
    sAutoActionOn = FALSE;
    ClearSavedCursorPos();
    CreateCursorSprites();
    (*sStorage).cursorPrevHorizPos = 1;
    (*sStorage).inBoxMovingMode = MOVE_MODE_NORMAL;
    TryRefreshDisplayMon();
}
pub(crate) unsafe extern "C" fn InitCursorOnReopen() {
    CreateCursorSprites();
    ReshowDisplayMon();
    (*sStorage).cursorPrevHorizPos = 1;
    (*sStorage).inBoxMovingMode = MOVE_MODE_NORMAL;
    if sIsMonBeingMoved != 0 {
        (*sStorage).movingMon = sSavedMovingMon;
        CreateMovingMonIcon();
    }
}
pub(crate) unsafe extern "C" fn GetCursorCoordsByPos(
    cursorArea: u8,
    cursorPosition: u8,
    x: *mut u16,
    y: *mut u16,
) {
    match cursorArea {
        CURSOR_AREA_IN_BOX => {
            *x = (cursorPosition as i32 % 6) as u16 * 24 + 100;
            *y = (cursorPosition as i32 / 6) as u16 * 24 + 32;
        }
        1 => {
            if cursorPosition == 0 {
                *x = 104;
                *y = 52;
            } else if cursorPosition == PARTY_SIZE as u8 {
                *x = 152;
                *y = 132;
            } else {
                *x = 152;
                *y = (cursorPosition as u16 - 1) * 24 + 4;
            }
        }
        CURSOR_AREA_BOX_TITLE => {
            *x = 162;
            *y = 12;
        }
        3 => {
            *y = (if sIsMonBeingMoved != 0 { 8 } else { 14 }) as u16;
            *x = cursorPosition as u16 * 88 + 120;
        }
        4 => {
            *x = 160;
            *y = 96;
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn GetSpeciesAtCursorPosition() -> u16 {
    match sCursorArea {
        CURSOR_AREA_IN_PARTY => {
            return GetMonData2(&raw mut gPlayerParty[sCursorPosition], MON_DATA_SPECIES) as u16;
        }
        0 => {
            return GetCurrentBoxMonData(sCursorPosition as u8, MON_DATA_SPECIES) as u16;
        }
        _ => {
            return SPECIES_NONE;
        }
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn UpdateCursorPos() -> u8 {
    let mut tmp: i16 = 0;
    if (*sStorage).cursorMoveSteps == 0 {
        if (*sStorage).boxOption != OPTION_MOVE_ITEMS {
            return FALSE;
        } else {
            return IsItemIconAnimActive();
        }
    } else if ({
        (*sStorage).cursorMoveSteps -= 1;
        (*sStorage).cursorMoveSteps
    }) != 0
    {
        (*sStorage).cursorNewX += (*sStorage).cursorSpeedX as i32;
        (*sStorage).cursorNewY += (*sStorage).cursorSpeedY as i32;
        (*(*sStorage).cursorSprite).x = ((*sStorage).cursorNewX >> 8) as i16;
        (*(*sStorage).cursorSprite).y = ((*sStorage).cursorNewY >> 8) as i16;
        if (*(*sStorage).cursorSprite).x > 256 {
            tmp = (*(*sStorage).cursorSprite).x - 256;
            (*(*sStorage).cursorSprite).x = tmp + 64;
        }
        if (*(*sStorage).cursorSprite).x < 64 {
            tmp = 64 - (*(*sStorage).cursorSprite).x;
            (*(*sStorage).cursorSprite).x = 256 - tmp;
        }
        if (*(*sStorage).cursorSprite).y > 176 {
            tmp = (*(*sStorage).cursorSprite).y - 176;
            (*(*sStorage).cursorSprite).y = tmp - 16;
        }
        if (*(*sStorage).cursorSprite).y < -16 {
            tmp = -16 - (*(*sStorage).cursorSprite).y;
            (*(*sStorage).cursorSprite).y = 176 - tmp;
        }
        if (*sStorage).cursorFlipTimer != 0
            && ({
                (*sStorage).cursorFlipTimer -= 1;
                (*sStorage).cursorFlipTimer
            }) == 0
        {
            (*(*sStorage).cursorSprite)
                .set_vFlip(((*(*sStorage).cursorSprite).vFlip() == FALSE as u16) as u16);
        }
    } else {
        (*(*sStorage).cursorSprite).x = (*sStorage).cursorTargetX;
        (*(*sStorage).cursorSprite).y = (*sStorage).cursorTargetY;
        DoCursorNewPosUpdate();
    }
    return TRUE;
}
pub(crate) unsafe extern "C" fn InitNewCursorPos(newCursorArea: u8, newCursorPosition: u8) {
    let mut x: u16 = 0;
    let mut y: u16 = 0;
    GetCursorCoordsByPos(newCursorArea, newCursorPosition, &raw mut x, &raw mut y);
    (*sStorage).newCursorArea = newCursorArea;
    (*sStorage).newCursorPosition = newCursorPosition;
    (*sStorage).cursorTargetX = x as i16;
    (*sStorage).cursorTargetY = y as i16;
}
pub(crate) unsafe extern "C" fn InitCursorMove() {
    let mut yDistance: i32 = 0;
    let mut xDistance: i32 = 0;
    if (*sStorage).cursorVerticalWrap != 0 || (*sStorage).cursorHorizontalWrap != 0 {
        (*sStorage).cursorMoveSteps = 12;
    } else {
        (*sStorage).cursorMoveSteps = 6;
    }
    if (*sStorage).cursorFlipTimer != 0 {
        (*sStorage).cursorFlipTimer = ((*sStorage).cursorMoveSteps >> 1) as u8;
    }
    match (*sStorage).cursorVerticalWrap {
        -1 => {
            yDistance =
                (*sStorage).cursorTargetY as i32 - 192 - (*(*sStorage).cursorSprite).y as i32;
        }
        1 => {
            yDistance =
                (*sStorage).cursorTargetY as i32 + 192 - (*(*sStorage).cursorSprite).y as i32;
        }
        _ => {
            yDistance = (*sStorage).cursorTargetY as i32 - (*(*sStorage).cursorSprite).y as i32;
        }
    }
    match (*sStorage).cursorHorizontalWrap {
        -1 => {
            xDistance =
                (*sStorage).cursorTargetX as i32 - 192 - (*(*sStorage).cursorSprite).x as i32;
        }
        1 => {
            xDistance =
                (*sStorage).cursorTargetX as i32 + 192 - (*(*sStorage).cursorSprite).x as i32;
        }
        _ => {
            xDistance = (*sStorage).cursorTargetX as i32 - (*(*sStorage).cursorSprite).x as i32;
        }
    }
    yDistance <<= 8;
    xDistance <<= 8;
    (*sStorage).cursorSpeedX = div_i32(xDistance, (*sStorage).cursorMoveSteps as i32) as u32;
    (*sStorage).cursorSpeedY = div_i32(yDistance, (*sStorage).cursorMoveSteps as i32) as u32;
    (*sStorage).cursorNewX = ((*(*sStorage).cursorSprite).x as i32) << 8;
    (*sStorage).cursorNewY = ((*(*sStorage).cursorSprite).y as i32) << 8;
}
pub(crate) unsafe extern "C" fn SetCursorPosition(newCursorArea: u8, newCursorPosition: u8) {
    InitNewCursorPos(newCursorArea, newCursorPosition);
    InitCursorMove();
    if (*sStorage).boxOption != OPTION_MOVE_ITEMS {
        if (*sStorage).inBoxMovingMode == MOVE_MODE_NORMAL && sIsMonBeingMoved == 0 {
            StartSpriteAnim((*sStorage).cursorSprite, CURSOR_ANIM_STILL);
        }
    } else {
        if IsMovingItem() == 0 {
            StartSpriteAnim((*sStorage).cursorSprite, CURSOR_ANIM_STILL);
        }
    }
    if (*sStorage).boxOption == OPTION_MOVE_ITEMS {
        if sCursorArea == CURSOR_AREA_IN_BOX as i8 {
            TryHideItemIconAtPos(CURSOR_AREA_IN_BOX, sCursorPosition as u8);
        } else if sCursorArea == CURSOR_AREA_IN_PARTY {
            TryHideItemIconAtPos(CURSOR_AREA_IN_PARTY as u8, sCursorPosition as u8);
        }
        if newCursorArea == CURSOR_AREA_IN_BOX {
            TryLoadItemIconAtPos(newCursorArea, newCursorPosition);
        } else if newCursorArea == CURSOR_AREA_IN_PARTY as u8 {
            TryLoadItemIconAtPos(newCursorArea, newCursorPosition);
        }
    }
    if newCursorArea == CURSOR_AREA_IN_PARTY as u8 && sCursorArea != CURSOR_AREA_IN_PARTY {
        (*sStorage).cursorPrevHorizPos = 1;
        (*(*sStorage).cursorShadowSprite).set_invisible(TRUE as u16);
    }
    match newCursorArea {
        1 | CURSOR_AREA_BOX_TITLE | 3 => {
            (*(*sStorage).cursorSprite).oam.set_priority(1);
            (*(*sStorage).cursorShadowSprite).set_invisible(TRUE as u16);
            (*(*sStorage).cursorShadowSprite).oam.set_priority(1);
        }
        CURSOR_AREA_IN_BOX => {
            if (*sStorage).inBoxMovingMode != MOVE_MODE_NORMAL {
                (*(*sStorage).cursorSprite).oam.set_priority(0);
                (*(*sStorage).cursorShadowSprite).set_invisible(TRUE as u16);
            } else {
                (*(*sStorage).cursorSprite).oam.set_priority(2);
                if sCursorArea == CURSOR_AREA_IN_BOX as i8 && sIsMonBeingMoved != 0 {
                    SetMovingMonPriority(2);
                }
            }
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn DoCursorNewPosUpdate() {
    sCursorArea = (*sStorage).newCursorArea as i8;
    sCursorPosition = (*sStorage).newCursorPosition as i8;
    if (*sStorage).boxOption != OPTION_MOVE_ITEMS {
        if (*sStorage).inBoxMovingMode == MOVE_MODE_NORMAL && sIsMonBeingMoved == 0 {
            StartSpriteAnim((*sStorage).cursorSprite, CURSOR_ANIM_BOUNCE);
        }
    } else {
        if IsMovingItem() == 0 {
            StartSpriteAnim((*sStorage).cursorSprite, CURSOR_ANIM_BOUNCE);
        }
    }
    TryRefreshDisplayMon();
    match sCursorArea {
        CURSOR_AREA_BUTTONS => {
            SetMovingMonPriority(1);
        }
        2 => {
            AnimateBoxScrollArrows(TRUE);
        }
        CURSOR_AREA_IN_PARTY => {
            (*(*sStorage).cursorShadowSprite).subpriority = 13;
            SetMovingMonPriority(1);
        }
        0 => {
            if (*sStorage).inBoxMovingMode == MOVE_MODE_NORMAL {
                (*(*sStorage).cursorSprite).oam.set_priority(1);
                (*(*sStorage).cursorShadowSprite).oam.set_priority(2);
                (*(*sStorage).cursorShadowSprite).subpriority = 21;
                (*(*sStorage).cursorShadowSprite).set_invisible(FALSE as u16);
                SetMovingMonPriority(2);
            }
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn SetCursorInParty() {
    let mut partyCount: u8 = 0;
    if sIsMonBeingMoved == 0 {
        partyCount = 0;
    } else {
        partyCount = CalculatePlayerPartyCount();
        if partyCount >= PARTY_SIZE as u8 {
            partyCount = 5;
        }
    }
    if (*(*sStorage).cursorSprite).vFlip() != 0 {
        (*sStorage).cursorFlipTimer = 1;
    }
    SetCursorPosition(CURSOR_AREA_IN_PARTY as u8, partyCount);
}
pub(crate) unsafe extern "C" fn SetCursorBoxPosition(cursorBoxPosition: u8) {
    SetCursorPosition(CURSOR_AREA_IN_BOX, cursorBoxPosition);
}
pub(crate) unsafe extern "C" fn ClearSavedCursorPos() {
    sSavedCursorPosition = 0;
}
pub(crate) unsafe extern "C" fn SaveCursorPos() {
    sSavedCursorPosition = sCursorPosition as u8;
}
pub(crate) unsafe extern "C" fn GetSavedCursorPos() -> u8 {
    return sSavedCursorPosition;
}
pub(crate) unsafe extern "C" fn InitMonPlaceChange(r#type: u8) {
    (*sStorage).monPlaceChangeFunc = placeChangeFuncs_10[r#type];
    (*sStorage).monPlaceChangeState = 0;
}
pub(crate) unsafe extern "C" fn InitMultiMonPlaceChange(up: u8) {
    if up == 0 {
        (*sStorage).monPlaceChangeFunc = Some(MultiMonPlaceChange_Down);
    } else {
        (*sStorage).monPlaceChangeFunc = Some(MultiMonPlaceChange_Up);
    }
    (*sStorage).monPlaceChangeState = 0;
}
pub(crate) unsafe extern "C" fn DoMonPlaceChange() -> u8 {
    return (*sStorage).monPlaceChangeFunc.unwrap_unchecked()();
}
pub(crate) unsafe extern "C" fn MonPlaceChange_Grab() -> u8 {
    match (*sStorage).monPlaceChangeState {
        0 => {
            if sIsMonBeingMoved != 0 {
                return FALSE;
            }
            StartSpriteAnim((*sStorage).cursorSprite, CURSOR_ANIM_OPEN);
            (*sStorage).monPlaceChangeState += 1;
        }
        1 => {
            if MonPlaceChange_CursorDown() == 0 {
                StartSpriteAnim((*sStorage).cursorSprite, CURSOR_ANIM_FIST);
                MoveMon();
                (*sStorage).monPlaceChangeState += 1;
            }
        }
        2 => {
            if MonPlaceChange_CursorUp() == 0 {
                (*sStorage).monPlaceChangeState += 1;
            }
        }
        3 => {
            return FALSE;
        }
        _ => {}
    }
    return TRUE;
}
pub(crate) unsafe extern "C" fn MonPlaceChange_Place() -> u8 {
    match (*sStorage).monPlaceChangeState {
        0 => {
            if MonPlaceChange_CursorDown() == 0 {
                StartSpriteAnim((*sStorage).cursorSprite, CURSOR_ANIM_OPEN);
                PlaceMon();
                (*sStorage).monPlaceChangeState += 1;
            }
        }
        1 => {
            if MonPlaceChange_CursorUp() == 0 {
                StartSpriteAnim((*sStorage).cursorSprite, CURSOR_ANIM_BOUNCE);
                (*sStorage).monPlaceChangeState += 1;
            }
        }
        2 => {
            return FALSE;
        }
        _ => {}
    }
    return TRUE;
}
pub(crate) unsafe extern "C" fn MonPlaceChange_Shift() -> u8 {
    match (*sStorage).monPlaceChangeState {
        0 => {
            match sCursorArea {
                CURSOR_AREA_IN_PARTY => {
                    (*sStorage).shiftBoxId = TOTAL_BOXES_COUNT;
                }
                0 => {
                    (*sStorage).shiftBoxId = StorageGetCurrentBox();
                }
                _ => {
                    return FALSE;
                }
            }
            StartSpriteAnim((*sStorage).cursorSprite, CURSOR_ANIM_OPEN);
            SaveMonSpriteAtPos((*sStorage).shiftBoxId, sCursorPosition as u8);
            (*sStorage).monPlaceChangeState += 1;
        }
        1 => {
            if MoveShiftingMons() == 0 {
                StartSpriteAnim((*sStorage).cursorSprite, CURSOR_ANIM_FIST);
                SetShiftedMonData((*sStorage).shiftBoxId, sCursorPosition as u8);
                (*sStorage).monPlaceChangeState += 1;
            }
        }
        2 => {
            return FALSE;
        }
        _ => {}
    }
    return TRUE;
}
pub(crate) unsafe extern "C" fn MultiMonPlaceChange_Down() -> u8 {
    return MonPlaceChange_CursorDown();
}
pub(crate) unsafe extern "C" fn MultiMonPlaceChange_Up() -> u8 {
    return MonPlaceChange_CursorUp();
}
pub(crate) unsafe extern "C" fn MonPlaceChange_CursorDown() -> u8 {
    match (*(*sStorage).cursorSprite).y2 {
        0 => {
            (*(*sStorage).cursorSprite).y2 += 1;
        }
        8 => {
            return FALSE;
        }
        _ => {
            (*(*sStorage).cursorSprite).y2 += 1;
        }
    }
    return TRUE;
}
pub(crate) unsafe extern "C" fn MonPlaceChange_CursorUp() -> u8 {
    match (*(*sStorage).cursorSprite).y2 {
        0 => {
            return FALSE;
        }
        _ => {
            (*(*sStorage).cursorSprite).y2 -= 1;
        }
    }
    return TRUE;
}
pub(crate) unsafe extern "C" fn MoveMon() {
    match sCursorArea {
        CURSOR_AREA_IN_PARTY => {
            SetMovingMonData(TOTAL_BOXES_COUNT, sCursorPosition as u8);
            SetMovingMonSprite(MODE_PARTY, sCursorPosition as u8);
        }
        0 => {
            if (*sStorage).inBoxMovingMode == MOVE_MODE_NORMAL {
                SetMovingMonData(StorageGetCurrentBox(), sCursorPosition as u8);
                SetMovingMonSprite(MODE_BOX, sCursorPosition as u8);
            }
        }
        _ => {
            return;
        }
    }
    sIsMonBeingMoved = TRUE;
}
pub(crate) unsafe extern "C" fn PlaceMon() {
    let mut boxId: u8 = 0;
    match sCursorArea {
        CURSOR_AREA_IN_PARTY => {
            SetPlacedMonData(TOTAL_BOXES_COUNT, sCursorPosition as u8);
            SetPlacedMonSprite(TOTAL_BOXES_COUNT, sCursorPosition as u8);
        }
        0 => {
            boxId = StorageGetCurrentBox();
            SetPlacedMonData(boxId, sCursorPosition as u8);
            SetPlacedMonSprite(boxId, sCursorPosition as u8);
        }
        _ => {
            return;
        }
    }
    sIsMonBeingMoved = FALSE;
}
pub(crate) unsafe extern "C" fn RefreshDisplayMon() {
    TryRefreshDisplayMon();
}
pub(crate) unsafe extern "C" fn SetMovingMonData(boxId: u8, position: u8) {
    if boxId == TOTAL_BOXES_COUNT {
        (*sStorage).movingMon = gPlayerParty[sCursorPosition];
    } else {
        BoxMonAtToMon(boxId, position, &raw mut (*sStorage).movingMon);
    }
    PurgeMonOrBoxMon(boxId, position);
    sMovingMonOrigBoxId = boxId;
    sMovingMonOrigBoxPos = position;
}
pub(crate) unsafe extern "C" fn SetPlacedMonData(boxId: u8, position: u8) {
    if boxId == TOTAL_BOXES_COUNT {
        gPlayerParty[position] = (*sStorage).movingMon;
    } else {
        BoxMonRestorePP(&raw mut (*sStorage).movingMon.r#box);
        SetBoxMonAt(boxId, position, &raw mut (*sStorage).movingMon.r#box);
    }
}
pub(crate) unsafe extern "C" fn PurgeMonOrBoxMon(boxId: u8, position: u8) {
    if boxId == TOTAL_BOXES_COUNT {
        ZeroMonData(&raw mut gPlayerParty[position]);
    } else {
        ZeroBoxMonAt(boxId, position);
    }
}
pub(crate) unsafe extern "C" fn SetShiftedMonData(boxId: u8, position: u8) {
    if boxId == TOTAL_BOXES_COUNT {
        (*sStorage).tempMon = gPlayerParty[position];
    } else {
        BoxMonAtToMon(boxId, position, &raw mut (*sStorage).tempMon);
    }
    SetPlacedMonData(boxId, position);
    (*sStorage).movingMon = (*sStorage).tempMon;
    SetDisplayMonData(&raw mut (*sStorage).movingMon as *mut c_void, MODE_PARTY);
    sMovingMonOrigBoxId = boxId;
    sMovingMonOrigBoxPos = position;
}
pub(crate) unsafe extern "C" fn TryStorePartyMonInBox(boxId: u8) -> u8 {
    let mut boxPosition: i16 = GetFirstFreeBoxSpot(boxId);
    if boxPosition == -1 {
        return FALSE;
    }
    if sIsMonBeingMoved != 0 {
        SetPlacedMonData(boxId, boxPosition as u8);
        DestroyMovingMonIcon();
        sIsMonBeingMoved = FALSE;
    } else {
        SetMovingMonData(TOTAL_BOXES_COUNT, sCursorPosition as u8);
        SetPlacedMonData(boxId, boxPosition as u8);
        DestroyPartyMonIcon(sCursorPosition as u8);
    }
    if boxId == StorageGetCurrentBox() {
        CreateBoxMonIconAtPos(boxPosition as u8);
    }
    StartSpriteAnim((*sStorage).cursorSprite, CURSOR_ANIM_STILL);
    return TRUE;
}
pub(crate) unsafe extern "C" fn ResetSelectionAfterDeposit() {
    StartSpriteAnim((*sStorage).cursorSprite, CURSOR_ANIM_BOUNCE);
    TryRefreshDisplayMon();
}
pub(crate) unsafe extern "C" fn InitReleaseMon() {
    let mut mode: u8 = 0;
    if sIsMonBeingMoved != 0 {
        mode = MODE_MOVE;
    } else if sCursorArea == CURSOR_AREA_IN_PARTY {
        mode = MODE_PARTY;
    } else {
        mode = MODE_BOX;
    }
    SetReleaseMon(mode, sCursorPosition as u8);
    StringCopy(
        (*sStorage).releaseMonName.as_mut_ptr(),
        (*sStorage).displayMonName.as_mut_ptr(),
    );
}
pub(crate) unsafe extern "C" fn TryHideReleaseMon() -> u8 {
    if TryHideReleaseMonSprite() == 0 {
        StartSpriteAnim((*sStorage).cursorSprite, CURSOR_ANIM_BOUNCE);
        return FALSE;
    } else {
        return TRUE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn ReleaseMon() {
    let mut boxId: u8 = 0;
    DestroyReleaseMonIcon();
    if sIsMonBeingMoved != 0 {
        sIsMonBeingMoved = FALSE;
    } else {
        if sCursorArea == CURSOR_AREA_IN_PARTY {
            boxId = TOTAL_BOXES_COUNT;
        } else {
            boxId = StorageGetCurrentBox();
        }
        PurgeMonOrBoxMon(boxId, sCursorPosition as u8);
    }
    TryRefreshDisplayMon();
}
pub(crate) unsafe extern "C" fn TrySetCursorFistAnim() {
    if sIsMonBeingMoved != 0 {
        StartSpriteAnim((*sStorage).cursorSprite, CURSOR_ANIM_FIST);
    }
}
pub(crate) unsafe extern "C" fn GetRestrictedReleaseMoves(mut moves: *mut u16) {
    let mut i: i32 = 0;
    i = 0;
    while i < 6 {
        if sRestrictedReleaseMoves[i].mapGroup == MAP_GROUPS_COUNT
            || sRestrictedReleaseMoves[i].mapGroup == (*gSaveBlock1Ptr).location.mapGroup
                && sRestrictedReleaseMoves[i].mapNum == (*gSaveBlock1Ptr).location.mapNum
        {
            *moves = sRestrictedReleaseMoves[i].r#move;
            moves = moves.at(1);
        }
        i += 1;
    }
    *moves = MOVES_COUNT;
}
pub(crate) unsafe extern "C" fn InitCanReleaseMonVars() {
    if AtLeastThreeUsableMons() == 0 {
        (*sStorage).releaseStatusResolved = TRUE;
        (*sStorage).canReleaseMon = FALSE as i8;
        return;
    }
    if sIsMonBeingMoved != 0 {
        (*sStorage).tempMon = (*sStorage).movingMon;
        (*sStorage).releaseBoxId = -1;
        (*sStorage).releaseBoxPos = -1;
    } else {
        if sCursorArea == CURSOR_AREA_IN_PARTY {
            (*sStorage).tempMon = gPlayerParty[sCursorPosition];
            (*sStorage).releaseBoxId = TOTAL_BOXES_COUNT as i8;
        } else {
            BoxMonAtToMon(
                StorageGetCurrentBox(),
                sCursorPosition as u8,
                &raw mut (*sStorage).tempMon,
            );
            (*sStorage).releaseBoxId = StorageGetCurrentBox() as i8;
        }
        (*sStorage).releaseBoxPos = sCursorPosition;
    }
    GetRestrictedReleaseMoves((*sStorage).restrictedMoveList.as_mut_ptr());
    (*sStorage).restrictedReleaseMonMoves = GetMonData3(
        &raw mut (*sStorage).tempMon,
        MON_DATA_KNOWN_MOVES,
        (*sStorage).restrictedMoveList.as_mut_ptr() as *mut u8,
    ) as u16;
    if (*sStorage).restrictedReleaseMonMoves != 0 {
        (*sStorage).releaseStatusResolved = FALSE;
    } else {
        (*sStorage).releaseStatusResolved = TRUE;
        (*sStorage).canReleaseMon = TRUE as i8;
    }
    (*sStorage).releaseCheckState = 0;
}
pub(crate) unsafe extern "C" fn AtLeastThreeUsableMons() -> u32 {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut count: i32 = (sIsMonBeingMoved != FALSE) as i32;
    j = 0;
    while j < PARTY_SIZE {
        if GetMonData2(&raw mut gPlayerParty[j], MON_DATA_SANITY_HAS_SPECIES) != 0 {
            count += 1;
        }
        j += 1;
    }
    if count >= 3 {
        return TRUE as u32;
    }
    i = 0;
    while i < TOTAL_BOXES_COUNT as i32 {
        j = 0;
        while j < IN_BOX_COUNT {
            if CheckBoxMonSanityAt(i as u32, j as u32) != 0 {
                if ({
                    count += 1;
                    count
                }) >= 3
                {
                    return TRUE as u32;
                }
            }
            j += 1;
        }
        i += 1;
    }
    return FALSE as u32;
}
pub(crate) unsafe extern "C" fn RunCanReleaseMon() -> i8 {
    let mut i: u16 = 0;
    let mut knownMoves: u16 = 0;
    if (*sStorage).releaseStatusResolved != 0 {
        return (*sStorage).canReleaseMon;
    }
    match (*sStorage).releaseCheckState {
        0 => {
            i = 0;
            while i < PARTY_SIZE as u16 {
                if (*sStorage).releaseBoxId != TOTAL_BOXES_COUNT as i8
                    || (*sStorage).releaseBoxPos as i32 != i as i32
                {
                    knownMoves = GetMonData3(
                        &raw mut gPlayerParty[i],
                        MON_DATA_KNOWN_MOVES,
                        (*sStorage).restrictedMoveList.as_mut_ptr() as *mut u8,
                    ) as u16;
                    (*sStorage).restrictedReleaseMonMoves &= !knownMoves;
                }
                i += 1;
            }
            if (*sStorage).restrictedReleaseMonMoves == 0 {
                (*sStorage).releaseStatusResolved = TRUE;
                (*sStorage).canReleaseMon = TRUE as i8;
            } else {
                (*sStorage).releaseCheckBoxId = 0;
                (*sStorage).releaseCheckBoxPos = 0;
                (*sStorage).releaseCheckState += 1;
            }
        }
        1 => {
            i = 0;
            while i < IN_BOX_COUNT as u16 {
                knownMoves = GetAndCopyBoxMonDataAt(
                    (*sStorage).releaseCheckBoxId as u8,
                    (*sStorage).releaseCheckBoxPos as u8,
                    MON_DATA_KNOWN_MOVES,
                    (*sStorage).restrictedMoveList.as_mut_ptr() as *mut u8 as *mut c_void,
                ) as u16;
                if knownMoves != 0
                    && !((*sStorage).releaseBoxId == (*sStorage).releaseCheckBoxId
                        && (*sStorage).releaseBoxPos == (*sStorage).releaseCheckBoxPos)
                {
                    (*sStorage).restrictedReleaseMonMoves &= !knownMoves;
                    if (*sStorage).restrictedReleaseMonMoves == 0 {
                        (*sStorage).releaseStatusResolved = TRUE;
                        (*sStorage).canReleaseMon = TRUE as i8;
                        break;
                    }
                }
                if ({
                    (*sStorage).releaseCheckBoxPos += 1;
                    (*sStorage).releaseCheckBoxPos
                }) >= IN_BOX_COUNT as i8
                {
                    (*sStorage).releaseCheckBoxPos = 0;
                    if ({
                        (*sStorage).releaseCheckBoxId += 1;
                        (*sStorage).releaseCheckBoxId
                    }) >= TOTAL_BOXES_COUNT as i8
                    {
                        (*sStorage).releaseStatusResolved = TRUE;
                        (*sStorage).canReleaseMon = FALSE as i8;
                    }
                }
                i += 1;
            }
        }
        _ => {}
    }
    return -1;
}
pub(crate) unsafe extern "C" fn SaveMovingMon() {
    if sIsMonBeingMoved != 0 {
        sSavedMovingMon = (*sStorage).movingMon;
    }
}
pub(crate) unsafe extern "C" fn LoadSavedMovingMon() {
    if sIsMonBeingMoved != 0 {
        if sMovingMonOrigBoxId == TOTAL_BOXES_COUNT {
            (*sStorage).movingMon = sSavedMovingMon;
        } else {
            (*sStorage).movingMon.r#box = sSavedMovingMon.r#box;
        }
    }
}
pub(crate) unsafe extern "C" fn InitSummaryScreenData() {
    if sIsMonBeingMoved != 0 {
        SaveMovingMon();
        (*sStorage).summaryMon.mon = &raw mut sSavedMovingMon;
        (*sStorage).summaryStartPos = 0;
        (*sStorage).summaryMaxPos = 0;
        (*sStorage).summaryScreenMode = SUMMARY_MODE_NORMAL;
    } else if sCursorArea == CURSOR_AREA_IN_PARTY {
        (*sStorage).summaryMon.mon = gPlayerParty.as_mut_ptr();
        (*sStorage).summaryStartPos = sCursorPosition as u8;
        (*sStorage).summaryMaxPos = CountPartyMons() - 1;
        (*sStorage).summaryScreenMode = SUMMARY_MODE_NORMAL;
    } else {
        (*sStorage).summaryMon.r#box = GetBoxedMonPtr(StorageGetCurrentBox(), 0);
        (*sStorage).summaryStartPos = sCursorPosition as u8;
        (*sStorage).summaryMaxPos = 29;
        (*sStorage).summaryScreenMode = SUMMARY_MODE_BOX;
    }
}
pub(crate) unsafe extern "C" fn SetSelectionAfterSummaryScreen() {
    if sIsMonBeingMoved != 0 {
        LoadSavedMovingMon();
    } else {
        sCursorPosition = gLastViewedMonIndex as i8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CompactPartySlots() -> i16 {
    let mut retVal: i16 = -1;
    let mut i: u16 = 0;
    let mut last: u16 = 0;
    i = 0;
    last = 0;
    while i < PARTY_SIZE as u16 {
        let mut species: u16 = GetMonData2(&raw mut gPlayerParty[i], MON_DATA_SPECIES) as u16;
        if species != SPECIES_NONE {
            if i != last {
                gPlayerParty[last] = gPlayerParty[i];
            }
            last += 1;
        } else if retVal == -1 {
            retVal = i as i16;
        }
        i += 1;
    }
    while last < PARTY_SIZE as u16 {
        ZeroMonData(&raw mut gPlayerParty[last]);
        last += 1;
    }
    return retVal;
}
pub(crate) unsafe extern "C" fn SetMonMarkings(mut markings: u8) {
    (*sStorage).displayMonMarkings = markings;
    if sIsMonBeingMoved != 0 {
        SetMonData(
            &raw mut (*sStorage).movingMon,
            MON_DATA_MARKINGS,
            &raw mut markings as *mut c_void,
        );
    } else {
        if sCursorArea == CURSOR_AREA_IN_PARTY {
            SetMonData(
                &raw mut gPlayerParty[sCursorPosition],
                MON_DATA_MARKINGS,
                &raw mut markings as *mut c_void,
            );
        }
        if sCursorArea == CURSOR_AREA_IN_BOX as i8 {
            SetCurrentBoxMonData(
                sCursorPosition as u8,
                MON_DATA_MARKINGS,
                &raw mut markings as *mut c_void,
            );
        }
    }
}
pub(crate) unsafe extern "C" fn IsRemovingLastPartyMon() -> u8 {
    if sCursorArea == CURSOR_AREA_IN_PARTY
        && sIsMonBeingMoved == 0
        && CountPartyAliveNonEggMonsExcept(sCursorPosition as u8) == 0
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
pub(crate) unsafe extern "C" fn CanShiftMon() -> u8 {
    if sIsMonBeingMoved != 0 {
        if sCursorArea == CURSOR_AREA_IN_PARTY
            && CountPartyAliveNonEggMonsExcept(sCursorPosition as u8) == 0
        {
            if (*sStorage).displayMonIsEgg != 0
                || GetMonData2(&raw mut (*sStorage).movingMon, MON_DATA_HP) == 0
            {
                return FALSE;
            }
        }
        return TRUE;
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn IsMonBeingMoved() -> u8 {
    return sIsMonBeingMoved;
}
pub(crate) unsafe extern "C" fn IsCursorOnBoxTitle() -> u8 {
    return (sCursorArea == CURSOR_AREA_BOX_TITLE as i8) as u8;
}
pub(crate) unsafe extern "C" fn IsCursorOnCloseBox() -> u8 {
    return (sCursorArea == CURSOR_AREA_BUTTONS && sCursorPosition == 1) as u8;
}
pub(crate) unsafe extern "C" fn IsCursorInBox() -> u8 {
    return (sCursorArea == CURSOR_AREA_IN_BOX as i8) as u8;
}
pub(crate) unsafe extern "C" fn TryRefreshDisplayMon() {
    (*sStorage).setMosaic = (sIsMonBeingMoved == FALSE) as u8;
    if sIsMonBeingMoved == 0 {
        'l1: {
            let sw1: i8 = sCursorArea;
            let mut fall = false;
            if sw1 == CURSOR_AREA_IN_PARTY {
                fall = true;
                if sCursorPosition < PARTY_SIZE as i8 {
                    SetDisplayMonData(
                        &raw mut gPlayerParty[sCursorPosition] as *mut c_void,
                        MODE_PARTY,
                    );
                    break 'l1;
                }
            }
            if fall || sw1 == CURSOR_AREA_BUTTONS || sw1 == 2 {
                fall = true;
                SetDisplayMonData(null_mut(), MODE_MOVE);
                break 'l1;
            }
            if sw1 == 0 {
                fall = true;
                SetDisplayMonData(
                    GetBoxedMonPtr(StorageGetCurrentBox(), sCursorPosition as u8) as *mut c_void,
                    MODE_BOX,
                );
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn ReshowDisplayMon() {
    if sIsMonBeingMoved != 0 {
        SetDisplayMonData(&raw mut sSavedMovingMon as *mut c_void, MODE_PARTY);
    } else {
        TryRefreshDisplayMon();
    }
}
pub(crate) unsafe extern "C" fn SetDisplayMonData(pokemon: *mut c_void, mode: u8) {
    let mut txtPtr: *mut u8 = null_mut();
    let mut gender: u16 = 0;
    let mut sanityIsBadEgg: u8 = 0;
    (*sStorage).displayMonItemId = ITEM_NONE;
    gender = MON_MALE as u16;
    sanityIsBadEgg = FALSE;
    if mode == MODE_PARTY {
        let mut mon: *mut Pokemon = pokemon as *mut Pokemon;
        (*sStorage).displayMonSpecies = GetMonData2(mon, MON_DATA_SPECIES_OR_EGG) as u16;
        if (*sStorage).displayMonSpecies != SPECIES_NONE {
            sanityIsBadEgg = GetMonData2(mon, MON_DATA_SANITY_IS_BAD_EGG) as u8;
            if sanityIsBadEgg != 0 {
                (*sStorage).displayMonIsEgg = TRUE;
            } else {
                (*sStorage).displayMonIsEgg = GetMonData2(mon, MON_DATA_IS_EGG) as u8;
            }
            GetMonData3(
                mon,
                MON_DATA_NICKNAME,
                (*sStorage).displayMonName.as_mut_ptr(),
            );
            StringGet_Nickname((*sStorage).displayMonName.as_mut_ptr());
            (*sStorage).displayMonLevel = GetMonData2(mon, MON_DATA_LEVEL) as u8;
            (*sStorage).displayMonMarkings = GetMonData2(mon, MON_DATA_MARKINGS) as u8;
            (*sStorage).displayMonPersonality = GetMonData2(mon, MON_DATA_PERSONALITY);
            (*sStorage).displayMonPalette = GetMonFrontSpritePal(mon);
            gender = GetMonGender(mon) as u16;
            (*sStorage).displayMonItemId = GetMonData2(mon, MON_DATA_HELD_ITEM) as u16;
        }
    } else if mode == MODE_BOX {
        let mut boxMon: *mut BoxPokemon = pokemon as *mut BoxPokemon;
        (*sStorage).displayMonSpecies =
            GetBoxMonData2(pokemon as *mut BoxPokemon, MON_DATA_SPECIES_OR_EGG) as u16;
        if (*sStorage).displayMonSpecies != SPECIES_NONE {
            let mut otId: u32 = GetBoxMonData2(boxMon, MON_DATA_OT_ID);
            sanityIsBadEgg = GetBoxMonData2(boxMon, MON_DATA_SANITY_IS_BAD_EGG) as u8;
            if sanityIsBadEgg != 0 {
                (*sStorage).displayMonIsEgg = TRUE;
            } else {
                (*sStorage).displayMonIsEgg = GetBoxMonData2(boxMon, MON_DATA_IS_EGG) as u8;
            }
            GetBoxMonData3(
                boxMon,
                MON_DATA_NICKNAME,
                (*sStorage).displayMonName.as_mut_ptr(),
            );
            StringGet_Nickname((*sStorage).displayMonName.as_mut_ptr());
            (*sStorage).displayMonLevel = GetLevelFromBoxMonExp(boxMon);
            (*sStorage).displayMonMarkings = GetBoxMonData2(boxMon, MON_DATA_MARKINGS) as u8;
            (*sStorage).displayMonPersonality = GetBoxMonData2(boxMon, MON_DATA_PERSONALITY);
            (*sStorage).displayMonPalette = GetMonSpritePalFromSpeciesAndPersonality(
                (*sStorage).displayMonSpecies,
                otId,
                (*sStorage).displayMonPersonality,
            );
            gender = GetGenderFromSpeciesAndPersonality(
                (*sStorage).displayMonSpecies,
                (*sStorage).displayMonPersonality,
            ) as u16;
            (*sStorage).displayMonItemId = GetBoxMonData2(boxMon, MON_DATA_HELD_ITEM) as u16;
        }
    } else {
        (*sStorage).displayMonSpecies = SPECIES_NONE;
        (*sStorage).displayMonItemId = ITEM_NONE;
    }
    if (*sStorage).displayMonSpecies == SPECIES_NONE {
        StringFill((*sStorage).displayMonName.as_mut_ptr(), CHAR_SPACE, 5);
        StringFill((*sStorage).displayMonNameText.as_mut_ptr(), CHAR_SPACE, 8);
        StringFill(
            (*sStorage).displayMonSpeciesName.as_mut_ptr(),
            CHAR_SPACE,
            8,
        );
        StringFill(
            (*sStorage).displayMonGenderLvlText.as_mut_ptr(),
            CHAR_SPACE,
            8,
        );
        StringFill((*sStorage).displayMonItemName.as_mut_ptr(), CHAR_SPACE, 8);
    } else if (*sStorage).displayMonIsEgg != 0 {
        if sanityIsBadEgg != 0 {
            StringCopyPadded(
                (*sStorage).displayMonNameText.as_mut_ptr(),
                (*sStorage).displayMonName.as_mut_ptr(),
                CHAR_SPACE,
                5,
            );
        } else {
            StringCopyPadded(
                (*sStorage).displayMonNameText.as_mut_ptr(),
                gText_EggNickname.as_ptr().cast_mut(),
                CHAR_SPACE,
                8,
            );
        }
        StringFill(
            (*sStorage).displayMonSpeciesName.as_mut_ptr(),
            CHAR_SPACE,
            8,
        );
        StringFill(
            (*sStorage).displayMonGenderLvlText.as_mut_ptr(),
            CHAR_SPACE,
            8,
        );
        StringFill((*sStorage).displayMonItemName.as_mut_ptr(), CHAR_SPACE, 8);
    } else {
        if (*sStorage).displayMonSpecies == SPECIES_NIDORAN_F
            || (*sStorage).displayMonSpecies == SPECIES_NIDORAN_M
        {
            gender = MON_GENDERLESS as u16;
        }
        StringCopyPadded(
            (*sStorage).displayMonNameText.as_mut_ptr(),
            (*sStorage).displayMonName.as_mut_ptr(),
            CHAR_SPACE,
            5,
        );
        txtPtr = (*sStorage).displayMonSpeciesName.as_mut_ptr();
        *({
            let t1 = txtPtr;
            txtPtr = txtPtr.at(1);
            t1
        }) = CHAR_SLASH;
        StringCopyPadded(
            txtPtr,
            gSpeciesNames[(*sStorage).displayMonSpecies]
                .as_ptr()
                .cast_mut(),
            CHAR_SPACE,
            5,
        );
        txtPtr = (*sStorage).displayMonGenderLvlText.as_mut_ptr();
        *({
            let t2 = txtPtr;
            txtPtr = txtPtr.at(1);
            t2
        }) = EXT_CTRL_CODE_BEGIN;
        *({
            let t3 = txtPtr;
            txtPtr = txtPtr.at(1);
            t3
        }) = EXT_CTRL_CODE_COLOR_HIGHLIGHT_SHADOW;
        match gender {
            0 => {
                *({
                    let t4 = txtPtr;
                    txtPtr = txtPtr.at(1);
                    t4
                }) = TEXT_COLOR_RED;
                *({
                    let t5 = txtPtr;
                    txtPtr = txtPtr.at(1);
                    t5
                }) = TEXT_COLOR_WHITE;
                *({
                    let t6 = txtPtr;
                    txtPtr = txtPtr.at(1);
                    t6
                }) = TEXT_COLOR_LIGHT_RED;
                *({
                    let t7 = txtPtr;
                    txtPtr = txtPtr.at(1);
                    t7
                }) = CHAR_MALE;
            }
            254 => {
                *({
                    let t8 = txtPtr;
                    txtPtr = txtPtr.at(1);
                    t8
                }) = TEXT_COLOR_GREEN;
                *({
                    let t9 = txtPtr;
                    txtPtr = txtPtr.at(1);
                    t9
                }) = TEXT_COLOR_WHITE;
                *({
                    let t10 = txtPtr;
                    txtPtr = txtPtr.at(1);
                    t10
                }) = TEXT_COLOR_LIGHT_GREEN;
                *({
                    let t11 = txtPtr;
                    txtPtr = txtPtr.at(1);
                    t11
                }) = CHAR_FEMALE;
            }
            _ => {
                *({
                    let t12 = txtPtr;
                    txtPtr = txtPtr.at(1);
                    t12
                }) = TEXT_COLOR_DARK_GRAY;
                *({
                    let t13 = txtPtr;
                    txtPtr = txtPtr.at(1);
                    t13
                }) = TEXT_COLOR_WHITE;
                *({
                    let t14 = txtPtr;
                    txtPtr = txtPtr.at(1);
                    t14
                }) = TEXT_COLOR_LIGHT_GRAY;
                *({
                    let t15 = txtPtr;
                    txtPtr = txtPtr.at(1);
                    t15
                }) = CHAR_SPACER;
            }
        }
        *({
            let t16 = txtPtr;
            txtPtr = txtPtr.at(1);
            t16
        }) = EXT_CTRL_CODE_BEGIN;
        *({
            let t17 = txtPtr;
            txtPtr = txtPtr.at(1);
            t17
        }) = EXT_CTRL_CODE_COLOR_HIGHLIGHT_SHADOW;
        *({
            let t18 = txtPtr;
            txtPtr = txtPtr.at(1);
            t18
        }) = TEXT_COLOR_DARK_GRAY;
        *({
            let t19 = txtPtr;
            txtPtr = txtPtr.at(1);
            t19
        }) = TEXT_COLOR_WHITE;
        *({
            let t20 = txtPtr;
            txtPtr = txtPtr.at(1);
            t20
        }) = TEXT_COLOR_LIGHT_GRAY;
        *({
            let t21 = txtPtr;
            txtPtr = txtPtr.at(1);
            t21
        }) = CHAR_SPACE;
        *({
            let t22 = txtPtr;
            txtPtr = txtPtr.at(1);
            t22
        }) = CHAR_EXTRA_SYMBOL;
        *({
            let t23 = txtPtr;
            txtPtr = txtPtr.at(1);
            t23
        }) = CHAR_LV_2;
        txtPtr = ConvertIntToDecimalStringN(
            txtPtr,
            (*sStorage).displayMonLevel as i32,
            STR_CONV_MODE_LEFT_ALIGN,
            3,
        );
        *txtPtr = 0x00;
        *txtPtr.at(1) = EOS;
        if (*sStorage).displayMonItemId != ITEM_NONE {
            StringCopyPadded(
                (*sStorage).displayMonItemName.as_mut_ptr(),
                GetItemName((*sStorage).displayMonItemId),
                CHAR_SPACE,
                8,
            );
        } else {
            StringFill((*sStorage).displayMonItemName.as_mut_ptr(), CHAR_SPACE, 8);
        }
    }
}
pub(crate) unsafe extern "C" fn HandleInput_InBox() -> u8 {
    match (*sStorage).inBoxMovingMode {
        MOVE_MODE_MULTIPLE_SELECTING => {
            return InBoxInput_SelectingMultiple();
        }
        MOVE_MODE_MULTIPLE_MOVING => {
            return InBoxInput_MovingMultiple();
        }
        _ => {
            return InBoxInput_Normal();
        }
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn InBoxInput_Normal() -> u8 {
    let mut retVal: u8 = 0;
    let mut cursorArea: i8 = 0;
    let mut cursorPosition: i8 = 0;
    'l2: {
        cursorArea = sCursorArea;
        cursorPosition = sCursorPosition;
        (*sStorage).cursorVerticalWrap = 0;
        (*sStorage).cursorHorizontalWrap = 0;
        (*sStorage).cursorFlipTimer = 0;
        if gMain.newAndRepeatedKeys as i32 & DPAD_UP != 0 {
            retVal = INPUT_MOVE_CURSOR;
            if sCursorPosition >= IN_BOX_COLUMNS as i8 {
                cursorPosition -= IN_BOX_COLUMNS as i8;
            } else {
                cursorArea = CURSOR_AREA_BOX_TITLE as i8;
                cursorPosition = 0;
            }
            break 'l2;
        } else if gMain.newAndRepeatedKeys as i32 & DPAD_DOWN != 0 {
            retVal = INPUT_MOVE_CURSOR;
            cursorPosition += IN_BOX_COLUMNS as i8;
            if cursorPosition >= IN_BOX_COUNT as i8 {
                cursorArea = CURSOR_AREA_BUTTONS;
                cursorPosition -= IN_BOX_COUNT as i8;
                cursorPosition = cursorPosition / 3;
                (*sStorage).cursorVerticalWrap = 1;
                (*sStorage).cursorFlipTimer = 1;
            }
            break 'l2;
        } else if gMain.newAndRepeatedKeys as i32 & DPAD_LEFT != 0 {
            retVal = INPUT_MOVE_CURSOR;
            if sCursorPosition % 6 != 0 {
                cursorPosition -= 1;
            } else {
                (*sStorage).cursorHorizontalWrap = -1;
                cursorPosition += 5;
            }
            break 'l2;
        } else if gMain.newAndRepeatedKeys as i32 & DPAD_RIGHT != 0 {
            retVal = INPUT_MOVE_CURSOR;
            if (sCursorPosition as i32 + 1) % 6 != 0 {
                cursorPosition += 1;
            } else {
                (*sStorage).cursorHorizontalWrap = 1;
                cursorPosition -= 5;
            }
            break 'l2;
        } else if gMain.newKeys as i32 & START_BUTTON != 0 {
            retVal = INPUT_MOVE_CURSOR;
            cursorArea = CURSOR_AREA_BOX_TITLE as i8;
            cursorPosition = 0;
            break 'l2;
        }
        if gMain.newKeys as i32 & A_BUTTON != 0 && SetSelectionMenuTexts() != 0 {
            if sAutoActionOn == 0 {
                return INPUT_IN_MENU;
            }
            if (*sStorage).boxOption != OPTION_MOVE_MONS || sIsMonBeingMoved == TRUE {
                match GetMenuItemTextId(0) {
                    MENU_STORE => {
                        return INPUT_DEPOSIT;
                    }
                    MENU_WITHDRAW => {
                        return INPUT_WITHDRAW;
                    }
                    MENU_MOVE => {
                        return INPUT_MOVE_MON;
                    }
                    MENU_SHIFT => {
                        return INPUT_SHIFT_MON;
                    }
                    MENU_PLACE => {
                        return INPUT_PLACE_MON;
                    }
                    MENU_TAKE => {
                        return INPUT_TAKE_ITEM;
                    }
                    MENU_GIVE => {
                        return INPUT_GIVE_ITEM;
                    }
                    MENU_SWITCH => {
                        return INPUT_SWITCH_ITEMS;
                    }
                    _ => {}
                }
            } else {
                (*sStorage).inBoxMovingMode = MOVE_MODE_MULTIPLE_SELECTING;
                return INPUT_MULTIMOVE_START;
            }
        }
        if gMain.newKeys as i32 & B_BUTTON != 0 {
            return INPUT_PRESSED_B;
        }
        if (*gSaveBlock2Ptr).optionsButtonMode == OPTIONS_BUTTON_MODE_LR {
            if gMain.heldKeys as i32 & L_BUTTON != 0 {
                return INPUT_SCROLL_LEFT;
            }
            if gMain.heldKeys as i32 & R_BUTTON != 0 {
                return INPUT_SCROLL_RIGHT;
            }
        }
        if gMain.newKeys as i32 & SELECT_BUTTON != 0 {
            ToggleCursorAutoAction();
            return INPUT_NONE;
        }
        retVal = INPUT_NONE;
    }
    if retVal != 0 {
        SetCursorPosition(cursorArea as u8, cursorPosition as u8);
    }
    return retVal;
}
pub(crate) unsafe extern "C" fn InBoxInput_SelectingMultiple() -> u8 {
    if gMain.heldKeys as i32 & A_BUTTON != 0 {
        if gMain.newAndRepeatedKeys as i32 & DPAD_UP != 0 {
            if sCursorPosition / 6 != 0 {
                SetCursorPosition(CURSOR_AREA_IN_BOX, sCursorPosition as u8 - IN_BOX_COLUMNS);
                return INPUT_MULTIMOVE_CHANGE_SELECTION;
            } else {
                return INPUT_MULTIMOVE_UNABLE;
            }
        } else if gMain.newAndRepeatedKeys as i32 & DPAD_DOWN != 0 {
            if (sCursorPosition as i32 + IN_BOX_COLUMNS as i32) < IN_BOX_COUNT {
                SetCursorPosition(CURSOR_AREA_IN_BOX, sCursorPosition as u8 + IN_BOX_COLUMNS);
                return INPUT_MULTIMOVE_CHANGE_SELECTION;
            } else {
                return INPUT_MULTIMOVE_UNABLE;
            }
        } else if gMain.newAndRepeatedKeys as i32 & DPAD_LEFT != 0 {
            if sCursorPosition % 6 != 0 {
                SetCursorPosition(CURSOR_AREA_IN_BOX, sCursorPosition as u8 - 1);
                return INPUT_MULTIMOVE_CHANGE_SELECTION;
            } else {
                return INPUT_MULTIMOVE_UNABLE;
            }
        } else if gMain.newAndRepeatedKeys as i32 & DPAD_RIGHT != 0 {
            if (sCursorPosition as i32 + 1) % 6 != 0 {
                SetCursorPosition(CURSOR_AREA_IN_BOX, sCursorPosition as u8 + 1);
                return INPUT_MULTIMOVE_CHANGE_SELECTION;
            } else {
                return INPUT_MULTIMOVE_UNABLE;
            }
        } else {
            return INPUT_NONE;
        }
    } else {
        if MultiMove_GetOrigin() as i32 == sCursorPosition as i32 {
            (*sStorage).inBoxMovingMode = MOVE_MODE_NORMAL;
            (*(*sStorage).cursorShadowSprite).set_invisible(FALSE as u16);
            return INPUT_MULTIMOVE_SINGLE;
        } else {
            sIsMonBeingMoved = ((*sStorage).displayMonSpecies != SPECIES_NONE) as u8;
            (*sStorage).inBoxMovingMode = MOVE_MODE_MULTIPLE_MOVING;
            sMovingMonOrigBoxId = StorageGetCurrentBox();
            return INPUT_MULTIMOVE_GRAB_SELECTION;
        }
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn InBoxInput_MovingMultiple() -> u8 {
    if gMain.newAndRepeatedKeys as i32 & DPAD_UP != 0 {
        if MultiMove_TryMoveGroup(0) != 0 {
            SetCursorPosition(CURSOR_AREA_IN_BOX, sCursorPosition as u8 - IN_BOX_COLUMNS);
            return INPUT_MULTIMOVE_MOVE_MONS;
        } else {
            return INPUT_MULTIMOVE_UNABLE;
        }
    } else if gMain.newAndRepeatedKeys as i32 & DPAD_DOWN != 0 {
        if MultiMove_TryMoveGroup(1) != 0 {
            SetCursorPosition(CURSOR_AREA_IN_BOX, sCursorPosition as u8 + IN_BOX_COLUMNS);
            return INPUT_MULTIMOVE_MOVE_MONS;
        } else {
            return INPUT_MULTIMOVE_UNABLE;
        }
    } else if gMain.newAndRepeatedKeys as i32 & DPAD_LEFT != 0 {
        if MultiMove_TryMoveGroup(2) != 0 {
            SetCursorPosition(CURSOR_AREA_IN_BOX, sCursorPosition as u8 - 1);
            return INPUT_MULTIMOVE_MOVE_MONS;
        } else {
            return INPUT_SCROLL_LEFT;
        }
    } else if gMain.newAndRepeatedKeys as i32 & DPAD_RIGHT != 0 {
        if MultiMove_TryMoveGroup(3) != 0 {
            SetCursorPosition(CURSOR_AREA_IN_BOX, sCursorPosition as u8 + 1);
            return INPUT_MULTIMOVE_MOVE_MONS;
        } else {
            return INPUT_SCROLL_RIGHT;
        }
    } else if gMain.newKeys as i32 & A_BUTTON != 0 {
        if MultiMove_CanPlaceSelection() != 0 {
            sIsMonBeingMoved = FALSE;
            (*sStorage).inBoxMovingMode = MOVE_MODE_NORMAL;
            return INPUT_MULTIMOVE_PLACE_MONS;
        } else {
            return INPUT_MULTIMOVE_UNABLE;
        }
    } else if gMain.newKeys as i32 & B_BUTTON != 0 {
        return INPUT_MULTIMOVE_UNABLE;
    } else {
        if (*gSaveBlock2Ptr).optionsButtonMode == OPTIONS_BUTTON_MODE_LR {
            if gMain.heldKeys as i32 & L_BUTTON != 0 {
                return INPUT_SCROLL_LEFT;
            }
            if gMain.heldKeys as i32 & R_BUTTON != 0 {
                return INPUT_SCROLL_RIGHT;
            }
        }
        return INPUT_NONE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn HandleInput_InParty() -> u8 {
    let mut retVal: u8 = 0;
    let mut gotoBox: u8 = 0;
    let mut cursorArea: i8 = 0;
    let mut cursorPosition: i8 = 0;
    'l2: {
        cursorArea = sCursorArea;
        cursorPosition = sCursorPosition;
        (*sStorage).cursorHorizontalWrap = 0;
        (*sStorage).cursorVerticalWrap = 0;
        (*sStorage).cursorFlipTimer = 0;
        gotoBox = FALSE;
        retVal = INPUT_NONE;
        if gMain.newAndRepeatedKeys as i32 & DPAD_UP != 0 {
            if ({
                cursorPosition -= 1;
                cursorPosition
            }) < 0
            {
                cursorPosition = PARTY_SIZE as i8;
            }
            if cursorPosition != sCursorPosition {
                retVal = INPUT_MOVE_CURSOR;
            }
            break 'l2;
        } else if gMain.newAndRepeatedKeys as i32 & DPAD_DOWN != 0 {
            if ({
                cursorPosition += 1;
                cursorPosition
            }) > PARTY_SIZE as i8
            {
                cursorPosition = 0;
            }
            if cursorPosition != sCursorPosition {
                retVal = INPUT_MOVE_CURSOR;
            }
            break 'l2;
        } else if gMain.newAndRepeatedKeys as i32 & DPAD_LEFT != 0 && sCursorPosition != 0 {
            retVal = INPUT_MOVE_CURSOR;
            (*sStorage).cursorPrevHorizPos = sCursorPosition as u8;
            cursorPosition = 0;
            break 'l2;
        } else if gMain.newAndRepeatedKeys as i32 & DPAD_RIGHT != 0 {
            if sCursorPosition == 0 {
                retVal = INPUT_MOVE_CURSOR;
                cursorPosition = (*sStorage).cursorPrevHorizPos as i8;
            } else {
                retVal = INPUT_HIDE_PARTY;
                cursorArea = CURSOR_AREA_IN_BOX as i8;
                cursorPosition = 0;
            }
            break 'l2;
        }
        if gMain.newKeys as i32 & A_BUTTON != 0 {
            if sCursorPosition == PARTY_SIZE as i8 {
                if (*sStorage).boxOption == OPTION_DEPOSIT {
                    return INPUT_CLOSE_BOX;
                }
                gotoBox = TRUE;
            } else if SetSelectionMenuTexts() != 0 {
                if sAutoActionOn == 0 {
                    return INPUT_IN_MENU;
                }
                match GetMenuItemTextId(0) {
                    MENU_STORE => {
                        return INPUT_DEPOSIT;
                    }
                    MENU_WITHDRAW => {
                        return INPUT_WITHDRAW;
                    }
                    MENU_MOVE => {
                        return INPUT_MOVE_MON;
                    }
                    MENU_SHIFT => {
                        return INPUT_SHIFT_MON;
                    }
                    MENU_PLACE => {
                        return INPUT_PLACE_MON;
                    }
                    MENU_TAKE => {
                        return INPUT_TAKE_ITEM;
                    }
                    MENU_GIVE => {
                        return INPUT_GIVE_ITEM;
                    }
                    MENU_SWITCH => {
                        return INPUT_SWITCH_ITEMS;
                    }
                    _ => {}
                }
            }
        }
        if gMain.newKeys as i32 & B_BUTTON != 0 {
            if (*sStorage).boxOption == OPTION_DEPOSIT {
                return INPUT_PRESSED_B;
            }
            gotoBox = TRUE;
        }
        if gotoBox != 0 {
            retVal = INPUT_HIDE_PARTY;
            cursorArea = CURSOR_AREA_IN_BOX as i8;
            cursorPosition = 0;
        } else if gMain.newKeys as i32 & SELECT_BUTTON != 0 {
            ToggleCursorAutoAction();
            return INPUT_NONE;
        }
    }
    if retVal != INPUT_NONE {
        if retVal != INPUT_HIDE_PARTY {
            SetCursorPosition(cursorArea as u8, cursorPosition as u8);
        }
    }
    return retVal;
}
pub(crate) unsafe extern "C" fn HandleInput_OnBox() -> u8 {
    let mut retVal: u8 = 0;
    let mut cursorArea: i8 = 0;
    let mut cursorPosition: i8 = 0;
    'l2: {
        (*sStorage).cursorHorizontalWrap = 0;
        (*sStorage).cursorVerticalWrap = 0;
        (*sStorage).cursorFlipTimer = 0;
        if gMain.newAndRepeatedKeys as i32 & DPAD_UP != 0 {
            retVal = INPUT_MOVE_CURSOR;
            cursorArea = CURSOR_AREA_BUTTONS;
            cursorPosition = 0;
            (*sStorage).cursorFlipTimer = 1;
            break 'l2;
        } else if gMain.newAndRepeatedKeys as i32 & DPAD_DOWN != 0 {
            retVal = INPUT_MOVE_CURSOR;
            cursorArea = CURSOR_AREA_IN_BOX as i8;
            cursorPosition = 2;
            break 'l2;
        }
        if gMain.heldKeys as i32 & DPAD_LEFT != 0 {
            return INPUT_SCROLL_LEFT;
        }
        if gMain.heldKeys as i32 & DPAD_RIGHT != 0 {
            return INPUT_SCROLL_RIGHT;
        }
        if (*gSaveBlock2Ptr).optionsButtonMode == OPTIONS_BUTTON_MODE_LR {
            if gMain.heldKeys as i32 & L_BUTTON != 0 {
                return INPUT_SCROLL_LEFT;
            }
            if gMain.heldKeys as i32 & R_BUTTON != 0 {
                return INPUT_SCROLL_RIGHT;
            }
        }
        if gMain.newKeys as i32 & A_BUTTON != 0 {
            AnimateBoxScrollArrows(FALSE);
            AddBoxOptionsMenu();
            return INPUT_BOX_OPTIONS;
        }
        if gMain.newKeys as i32 & B_BUTTON != 0 {
            return INPUT_PRESSED_B;
        }
        if gMain.newKeys as i32 & SELECT_BUTTON != 0 {
            ToggleCursorAutoAction();
            return INPUT_NONE;
        }
        retVal = INPUT_NONE;
    }
    if retVal != INPUT_NONE {
        if cursorArea != CURSOR_AREA_BOX_TITLE as i8 {
            AnimateBoxScrollArrows(FALSE);
        }
        SetCursorPosition(cursorArea as u8, cursorPosition as u8);
    }
    return retVal;
}
pub(crate) unsafe extern "C" fn HandleInput_OnButtons() -> u8 {
    let mut retVal: u8 = 0;
    let mut cursorArea: i8 = 0;
    let mut cursorPosition: i8 = 0;
    'l2: {
        cursorArea = sCursorArea;
        cursorPosition = sCursorPosition;
        (*sStorage).cursorHorizontalWrap = 0;
        (*sStorage).cursorVerticalWrap = 0;
        (*sStorage).cursorFlipTimer = 0;
        if gMain.newAndRepeatedKeys as i32 & DPAD_UP != 0 {
            retVal = INPUT_MOVE_CURSOR;
            cursorArea = CURSOR_AREA_IN_BOX as i8;
            (*sStorage).cursorVerticalWrap = -1;
            if sCursorPosition == 0 {
                cursorPosition = 24;
            } else {
                cursorPosition = 29;
            }
            (*sStorage).cursorFlipTimer = 1;
            break 'l2;
        }
        if gMain.newAndRepeatedKeys as i32 & 136 != 0 {
            retVal = INPUT_MOVE_CURSOR;
            cursorArea = CURSOR_AREA_BOX_TITLE as i8;
            cursorPosition = 0;
            (*sStorage).cursorFlipTimer = 1;
            break 'l2;
        }
        if gMain.newAndRepeatedKeys as i32 & DPAD_LEFT != 0 {
            retVal = INPUT_MOVE_CURSOR;
            if ({
                cursorPosition -= 1;
                cursorPosition
            }) < 0
            {
                cursorPosition = 1;
            }
            break 'l2;
        } else if gMain.newAndRepeatedKeys as i32 & DPAD_RIGHT != 0 {
            retVal = INPUT_MOVE_CURSOR;
            if ({
                cursorPosition += 1;
                cursorPosition
            }) > 1
            {
                cursorPosition = 0;
            }
            break 'l2;
        }
        if gMain.newKeys as i32 & A_BUTTON != 0 {
            return (if cursorPosition == 0 {
                INPUT_SHOW_PARTY as i32
            } else {
                INPUT_CLOSE_BOX as i32
            }) as u8;
        }
        if gMain.newKeys as i32 & B_BUTTON != 0 {
            return INPUT_PRESSED_B;
        }
        if gMain.newKeys as i32 & SELECT_BUTTON != 0 {
            ToggleCursorAutoAction();
            return INPUT_NONE;
        }
        retVal = INPUT_NONE;
    }
    if retVal != INPUT_NONE {
        SetCursorPosition(cursorArea as u8, cursorPosition as u8);
    }
    return retVal;
}
pub(crate) unsafe extern "C" fn HandleInput() -> u8 {
    let mut i: u16 = 0;
    while inputFuncs_9[i].func.is_some() {
        if inputFuncs_9[i].area == sCursorArea {
            return inputFuncs_9[i].func.unwrap_unchecked()();
        }
        i += 1;
    }
    return INPUT_NONE;
}
pub(crate) unsafe extern "C" fn AddBoxOptionsMenu() {
    InitMenu();
    SetMenuText(MENU_JUMP as u8);
    SetMenuText(MENU_WALLPAPER as u8);
    SetMenuText(MENU_NAME as u8);
    SetMenuText(MENU_CANCEL);
}
pub(crate) unsafe extern "C" fn SetSelectionMenuTexts() -> u8 {
    InitMenu();
    if (*sStorage).boxOption != OPTION_MOVE_ITEMS {
        return SetMenuTexts_Mon();
    } else {
        return SetMenuTexts_Item();
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn SetMenuTexts_Mon() -> u8 {
    let mut species: u16 = GetSpeciesAtCursorPosition();
    match (*sStorage).boxOption {
        OPTION_DEPOSIT => {
            if species != SPECIES_NONE {
                SetMenuText(MENU_STORE as u8);
            } else {
                return FALSE;
            }
        }
        0 => {
            if species != SPECIES_NONE {
                SetMenuText(MENU_WITHDRAW as u8);
            } else {
                return FALSE;
            }
        }
        OPTION_MOVE_MONS => {
            if sIsMonBeingMoved != 0 {
                if species != SPECIES_NONE {
                    SetMenuText(MENU_SHIFT as u8);
                } else {
                    SetMenuText(MENU_PLACE as u8);
                }
            } else {
                if species != SPECIES_NONE {
                    SetMenuText(MENU_MOVE as u8);
                } else {
                    return FALSE;
                }
            }
        }
        _ => {
            return FALSE;
        }
    }
    SetMenuText(MENU_SUMMARY as u8);
    if (*sStorage).boxOption == OPTION_MOVE_MONS {
        if sCursorArea == CURSOR_AREA_IN_BOX as i8 {
            SetMenuText(MENU_WITHDRAW as u8);
        } else {
            SetMenuText(MENU_STORE as u8);
        }
    }
    SetMenuText(MENU_MARK as u8);
    SetMenuText(MENU_RELEASE as u8);
    SetMenuText(MENU_CANCEL);
    return TRUE;
}
pub(crate) unsafe extern "C" fn SetMenuTexts_Item() -> u8 {
    if (*sStorage).displayMonSpecies == SPECIES_EGG as u16 {
        return FALSE;
    }
    if IsMovingItem() == 0 {
        if (*sStorage).displayMonItemId == ITEM_NONE {
            if (*sStorage).displayMonSpecies == SPECIES_NONE {
                return FALSE;
            }
            SetMenuText(MENU_GIVE_2 as u8);
        } else {
            if ItemIsMail((*sStorage).displayMonItemId) == 0 {
                SetMenuText(MENU_TAKE as u8);
                SetMenuText(MENU_BAG as u8);
            }
            SetMenuText(MENU_INFO as u8);
        }
    } else {
        if (*sStorage).displayMonItemId == ITEM_NONE {
            if (*sStorage).displayMonSpecies == SPECIES_NONE {
                return FALSE;
            }
            SetMenuText(MENU_GIVE as u8);
        } else {
            if ItemIsMail((*sStorage).displayMonItemId) == TRUE {
                return FALSE;
            }
            SetMenuText(MENU_SWITCH as u8);
        }
    }
    SetMenuText(MENU_CANCEL);
    return TRUE;
}
pub(crate) unsafe extern "C" fn SpriteCB_CursorShadow(sprite: *mut Sprite) {
    (*sprite).x = (*(*sStorage).cursorSprite).x;
    (*sprite).y = (*(*sStorage).cursorSprite).y + 20;
}
pub(crate) unsafe extern "C" fn CreateCursorSprites() {
    let mut x: u16 = 0;
    let mut y: u16 = 0;
    let mut spriteId: u8 = 0;
    let mut priority: u8 = 0;
    let mut subpriority: u8 = 0;
    let mut spriteSheets: CArray<SpriteSheet, 3> = zeroed();
    spriteSheets[0].data = sHandCursor_Gfx.as_ptr().cast_mut() as *mut c_void;
    spriteSheets[0].size = 0x800;
    spriteSheets[0].tag = GFXTAG_CURSOR;
    spriteSheets[1].data = sHandCursorShadow_Gfx.as_ptr().cast_mut() as *mut c_void;
    spriteSheets[1].size = 0x80;
    spriteSheets[1].tag = GFXTAG_CURSOR_SHADOW;
    let mut spritePalettes: CArray<SpritePalette, 2> = zeroed();
    spritePalettes[0].data = sHandCursor_Pal.as_ptr().cast_mut();
    spritePalettes[0].tag = PALTAG_MISC_1;
    LoadSpriteSheets(spriteSheets.as_mut_ptr());
    LoadSpritePalettes(spritePalettes.as_mut_ptr());
    (*sStorage).cursorPalNums[0] = IndexOfSpritePaletteTag(PALTAG_MISC_2);
    (*sStorage).cursorPalNums[1] = IndexOfSpritePaletteTag(PALTAG_MISC_1);
    GetCursorCoordsByPos(
        sCursorArea as u8,
        sCursorPosition as u8,
        &raw mut x,
        &raw mut y,
    );
    spriteId = CreateSprite(
        (&raw const *sSpriteTemplate_Cursor_8).cast_mut(),
        x as i16,
        y as i16,
        6,
    );
    if spriteId != MAX_SPRITES {
        (*sStorage).cursorSprite = &raw mut gSprites[spriteId];
        (*(*sStorage).cursorSprite)
            .oam
            .set_paletteNum((*sStorage).cursorPalNums[sAutoActionOn] as u16);
        (*(*sStorage).cursorSprite).oam.set_priority(1);
        if sIsMonBeingMoved != 0 {
            StartSpriteAnim((*sStorage).cursorSprite, CURSOR_ANIM_FIST);
        }
    } else {
        (*sStorage).cursorSprite = null_mut();
    }
    if sCursorArea == CURSOR_AREA_IN_PARTY {
        subpriority = 13;
        priority = 1;
    } else {
        subpriority = 21;
        priority = 2;
    }
    spriteId = CreateSprite(
        (&raw const *sSpriteTemplate_CursorShadow_7).cast_mut(),
        0,
        0,
        subpriority,
    );
    if spriteId != MAX_SPRITES {
        (*sStorage).cursorShadowSprite = &raw mut gSprites[spriteId];
        (*(*sStorage).cursorShadowSprite)
            .oam
            .set_priority(priority as u16);
        if sCursorArea != 0 {
            (*(*sStorage).cursorShadowSprite).set_invisible(TRUE as u16);
        }
    } else {
        (*sStorage).cursorShadowSprite = null_mut();
    }
}
pub(crate) unsafe extern "C" fn ToggleCursorAutoAction() {
    sAutoActionOn = (sAutoActionOn == 0) as u8;
    (*(*sStorage).cursorSprite)
        .oam
        .set_paletteNum((*sStorage).cursorPalNums[sAutoActionOn] as u16);
}
pub(crate) unsafe extern "C" fn GetCursorPosition() -> u8 {
    return sCursorPosition as u8;
}
pub(crate) unsafe extern "C" fn GetCursorBoxColumnAndRow(column: *mut u8, row: *mut u8) {
    if sCursorArea == CURSOR_AREA_IN_BOX as i8 {
        *column = (sCursorPosition % 6) as u8;
        *row = (sCursorPosition / 6) as u8;
    } else {
        *column = 0;
        *row = 0;
    }
}
pub(crate) unsafe extern "C" fn StartCursorAnim(animNum: u8) {
    StartSpriteAnim((*sStorage).cursorSprite, animNum);
}
pub(crate) unsafe extern "C" fn GetMovingMonOriginalBoxId() -> u8 {
    return sMovingMonOrigBoxId;
}
pub(crate) unsafe extern "C" fn SetCursorPriorityTo1() {
    (*(*sStorage).cursorSprite).oam.set_priority(1);
}
pub(crate) unsafe extern "C" fn TryHideItemAtCursor() {
    if sCursorArea == CURSOR_AREA_IN_BOX as i8 {
        TryHideItemIconAtPos(CURSOR_AREA_IN_BOX, sCursorPosition as u8);
    }
}
pub(crate) unsafe extern "C" fn TryShowItemAtCursor() {
    if sCursorArea == CURSOR_AREA_IN_BOX as i8 {
        TryLoadItemIconAtPos(CURSOR_AREA_IN_BOX, sCursorPosition as u8);
    }
}
pub(crate) unsafe extern "C" fn InitMenu() {
    (*sStorage).menuItemsCount = 0;
    (*sStorage).menuWidth = 0;
    (*sStorage).menuWindow.bg = 0;
    (*sStorage).menuWindow.paletteNum = 15;
    (*sStorage).menuWindow.baseBlock = 92;
}
pub(crate) unsafe extern "C" fn SetMenuText(textId: u8) {
    if (*sStorage).menuItemsCount < 7 {
        let mut len: u8 = 0;
        let mut menu: *mut StorageMenu = &raw mut (*sStorage).menuItems[(*sStorage).menuItemsCount];
        (*menu).text = sMenuTexts[textId];
        (*menu).textId = textId as i32;
        len = StringLength((*menu).text) as u8;
        if len > (*sStorage).menuWidth {
            (*sStorage).menuWidth = len;
        }
        (*sStorage).menuItemsCount += 1;
    }
}
pub(crate) unsafe extern "C" fn GetMenuItemTextId(menuIdx: u8) -> i8 {
    if menuIdx >= (*sStorage).menuItemsCount {
        return -1;
    } else {
        return (*sStorage).menuItems[menuIdx].textId as i8;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn AddMenu() {
    (*sStorage).menuWindow.width = (*sStorage).menuWidth + 2;
    (*sStorage).menuWindow.height = 2 * (*sStorage).menuItemsCount;
    (*sStorage).menuWindow.tilemapLeft = 29 - (*sStorage).menuWindow.width;
    (*sStorage).menuWindow.tilemapTop = 15 - (*sStorage).menuWindow.height;
    (*sStorage).menuWindowId = AddWindow(&raw mut (*sStorage).menuWindow);
    ClearWindowTilemap((*sStorage).menuWindowId as u8);
    DrawStdFrameWithCustomTileAndPalette((*sStorage).menuWindowId as u8, FALSE, 11, 14);
    PrintMenuTable(
        (*sStorage).menuWindowId as u8,
        (*sStorage).menuItemsCount,
        (*sStorage).menuItems.as_mut_ptr() as *mut c_void as *mut MenuAction,
    );
    InitMenuInUpperLeftCornerNormal(
        (*sStorage).menuWindowId as u8,
        (*sStorage).menuItemsCount,
        0,
    );
    ScheduleBgCopyTilemapToVram(0);
    (*sStorage).menuUnusedField = 0;
}
pub(crate) unsafe extern "C" fn IsMenuLoading() -> u8 {
    return FALSE;
}
pub(crate) unsafe extern "C" fn HandleMenuInput() -> i16 {
    let mut input: i32 = MENU_NOTHING_CHOSEN as i32;
    'l2: {
        if gMain.newKeys as i32 & A_BUTTON != 0 {
            input = Menu_GetCursorPos() as i32;
            break 'l2;
        } else if gMain.newKeys as i32 & B_BUTTON != 0 {
            PlaySE(SE_SELECT);
            input = MENU_B_PRESSED as i32;
        }
        if gMain.newKeys as i32 & DPAD_UP != 0 {
            PlaySE(SE_SELECT);
            Menu_MoveCursor(-1);
        } else if gMain.newKeys as i32 & DPAD_DOWN != 0 {
            PlaySE(SE_SELECT);
            Menu_MoveCursor(1);
        }
    }
    if input != MENU_NOTHING_CHOSEN as i32 {
        RemoveMenu();
    }
    if input >= 0 {
        input = (*sStorage).menuItems[input].textId;
    }
    return input as i16;
}
pub(crate) unsafe extern "C" fn RemoveMenu() {
    ClearStdWindowAndFrameToTransparent((*sStorage).menuWindowId as u8, TRUE);
    RemoveWindow((*sStorage).menuWindowId as u8);
}
pub(crate) unsafe extern "C" fn MultiMove_Init() -> u8 {
    sMultiMove = Alloc(2420) as *mut typeof___sMultiMove_0_t;
    if !sMultiMove.is_null() {
        (*sStorage).multiMoveWindowId =
            AddWindow8Bit((&raw const *sWindowTemplate_MultiMove).cast_mut());
        if (*sStorage).multiMoveWindowId != WINDOW_NONE as u16 {
            FillWindowPixelBuffer((*sStorage).multiMoveWindowId as u8, 0);
            return TRUE;
        }
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn MultiMove_Free() {
    if !sMultiMove.is_null() {
        Free(sMultiMove as *mut c_void);
    }
}
pub(crate) unsafe extern "C" fn MultiMove_SetFunction(id: u8) {
    (*sMultiMove).funcId = id;
    (*sMultiMove).state = 0;
}
pub(crate) unsafe extern "C" fn MultiMove_RunFunction() -> u8 {
    match (*sMultiMove).funcId {
        MULTIMOVE_START => {
            return MultiMove_Start();
        }
        MULTIMOVE_CANCEL => {
            return MultiMove_Cancel();
        }
        MULTIMOVE_CHANGE_SELECTION => {
            return MultiMove_ChangeSelection();
        }
        MULTIMOVE_GRAB_SELECTION => {
            return MultiMove_GrabSelection();
        }
        MULTIMOVE_MOVE_MONS => {
            return MultiMove_MoveMons();
        }
        MULTIMOVE_PLACE_MONS => {
            return MultiMove_PlaceMons();
        }
        _ => {}
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn MultiMove_Start() -> u8 {
    match (*sMultiMove).state {
        0 => {
            HideBg(0);
            TryLoadAllMonIconPalettesAtOffset(128);
            (*sMultiMove).state += 1;
        }
        1 => {
            GetCursorBoxColumnAndRow(
                &raw mut (*sMultiMove).fromColumn,
                &raw mut (*sMultiMove).fromRow,
            );
            (*sMultiMove).toColumn = (*sMultiMove).fromColumn;
            (*sMultiMove).toRow = (*sMultiMove).fromRow;
            ChangeBgX(0, -1024, BG_COORD_SET);
            ChangeBgY(0, -1024, BG_COORD_SET);
            FillBgTilemapBufferRect_Palette0(0, 0, 0, 0, 0x20, 0x20);
            FillWindowPixelBuffer8Bit((*sStorage).multiMoveWindowId as u8, 0);
            MultiMove_SetIconToBg((*sMultiMove).fromColumn, (*sMultiMove).fromRow);
            SetBgAttribute(0, BG_ATTR_PALETTEMODE, 1);
            PutWindowTilemap((*sStorage).multiMoveWindowId as u8);
            CopyWindowToVram8Bit((*sStorage).multiMoveWindowId as u8, COPYWIN_FULL);
            BlendPalettes(0x3F00, 8, 32767);
            StartCursorAnim(CURSOR_ANIM_OPEN);
            SetGpuRegBits(REG_OFFSET_BG0CNT, BGCNT_256COLOR);
            (*sMultiMove).state += 1;
        }
        2 => {
            if IsDma3ManagerBusyWithBgCopy() == 0 {
                ShowBg(0);
                return FALSE;
            }
        }
        _ => {}
    }
    return TRUE;
}
pub(crate) unsafe extern "C" fn MultiMove_Cancel() -> u8 {
    match (*sMultiMove).state {
        0 => {
            HideBg(0);
            (*sMultiMove).state += 1;
        }
        1 => {
            MultiMove_ResetBg();
            StartCursorAnim(CURSOR_ANIM_BOUNCE);
            (*sMultiMove).state += 1;
        }
        2 => {
            if IsDma3ManagerBusyWithBgCopy() == 0 {
                SetCursorPriorityTo1();
                LoadPalette(GetTextWindowPalette(3) as *mut c_void, 208, 32);
                ShowBg(0);
                return FALSE;
            }
        }
        _ => {}
    }
    return TRUE;
}
pub(crate) unsafe extern "C" fn MultiMove_ChangeSelection() -> u8 {
    match (*sMultiMove).state {
        0 => {
            if UpdateCursorPos() == 0 {
                GetCursorBoxColumnAndRow(
                    &raw mut (*sMultiMove).cursorColumn,
                    &raw mut (*sMultiMove).cursorRow,
                );
                MultiMove_UpdateSelectedIcons();
                (*sMultiMove).toColumn = (*sMultiMove).cursorColumn;
                (*sMultiMove).toRow = (*sMultiMove).cursorRow;
                CopyWindowToVram8Bit((*sStorage).multiMoveWindowId as u8, COPYWIN_GFX);
                (*sMultiMove).state += 1;
            }
        }
        1 => {
            return IsDma3ManagerBusyWithBgCopy();
        }
        _ => {}
    }
    return TRUE;
}
pub(crate) unsafe extern "C" fn MultiMove_GrabSelection() -> u8 {
    let mut movingBg: u8 = 0;
    let mut movingMon: u8 = 0;
    match (*sMultiMove).state {
        0 => {
            MultiMove_GetMonsFromSelection();
            MultiMove_RemoveMonsFromBox();
            InitMultiMonPlaceChange(FALSE);
            (*sMultiMove).state += 1;
        }
        1 => {
            if DoMonPlaceChange() == 0 {
                StartCursorAnim(CURSOR_ANIM_FIST);
                MultiMove_InitMove(0, 256, 8);
                InitMultiMonPlaceChange(TRUE);
                (*sMultiMove).state += 1;
            }
        }
        2 => {
            movingBg = MultiMove_UpdateMove();
            movingMon = DoMonPlaceChange();
            if movingBg == 0 && movingMon == 0 {
                return FALSE;
            }
        }
        _ => {}
    }
    return TRUE;
}
pub(crate) unsafe extern "C" fn MultiMove_MoveMons() -> u8 {
    let mut movingCursor: u8 = UpdateCursorPos();
    let mut movingBg: u8 = MultiMove_UpdateMove();
    if movingCursor == 0 && movingBg == 0 {
        return FALSE;
    } else {
        return TRUE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn MultiMove_PlaceMons() -> u8 {
    match (*sMultiMove).state {
        0 => {
            MultiMove_SetPlacedMonData();
            MultiMove_InitMove(0, 65280, 8);
            InitMultiMonPlaceChange(FALSE);
            (*sMultiMove).state += 1;
        }
        1 => {
            if DoMonPlaceChange() == 0 && MultiMove_UpdateMove() == 0 {
                MultiMove_CreatePlacedMonIcons();
                StartCursorAnim(CURSOR_ANIM_OPEN);
                InitMultiMonPlaceChange(TRUE);
                HideBg(0);
                (*sMultiMove).state += 1;
            }
        }
        2 => {
            if DoMonPlaceChange() == 0 {
                StartCursorAnim(CURSOR_ANIM_BOUNCE);
                MultiMove_ResetBg();
                (*sMultiMove).state += 1;
            }
        }
        3 => {
            if IsDma3ManagerBusyWithBgCopy() == 0 {
                LoadPalette(GetTextWindowPalette(3) as *mut c_void, 208, 32);
                SetCursorPriorityTo1();
                ShowBg(0);
                return FALSE;
            }
        }
        _ => {}
    }
    return TRUE;
}
pub(crate) unsafe extern "C" fn MultiMove_TryMoveGroup(dir: u8) -> u8 {
    match dir {
        0 => {
            if (*sMultiMove).minRow == 0 {
                return FALSE;
            }
            (*sMultiMove).minRow -= 1;
            MultiMove_InitMove(0, 1024, 6);
        }
        1 => {
            if (*sMultiMove).minRow as i32 + (*sMultiMove).rowsTotal as i32 >= IN_BOX_ROWS {
                return FALSE;
            }
            (*sMultiMove).minRow += 1;
            MultiMove_InitMove(0, 64512, 6);
        }
        2 => {
            if (*sMultiMove).minColumn == 0 {
                return FALSE;
            }
            (*sMultiMove).minColumn -= 1;
            MultiMove_InitMove(1024, 0, 6);
        }
        3 => {
            if (*sMultiMove).minColumn as i32 + (*sMultiMove).columnsTotal as i32
                >= IN_BOX_COLUMNS as i32
            {
                return FALSE;
            }
            (*sMultiMove).minColumn += 1;
            MultiMove_InitMove(64512, 0, 6);
        }
        _ => {}
    }
    return TRUE;
}
pub(crate) unsafe extern "C" fn MultiMove_UpdateSelectedIcons() {
    let mut columnChange: i16 =
        (if ((*sMultiMove).fromColumn as i32 - (*sMultiMove).cursorColumn as i32) < 0 {
            -((*sMultiMove).fromColumn as i32 - (*sMultiMove).cursorColumn as i32)
        } else {
            (*sMultiMove).fromColumn as i32 - (*sMultiMove).cursorColumn as i32
        }) as i16
            - (if ((*sMultiMove).fromColumn as i32 - (*sMultiMove).toColumn as i32) < 0 {
                -((*sMultiMove).fromColumn as i32 - (*sMultiMove).toColumn as i32)
            } else {
                (*sMultiMove).fromColumn as i32 - (*sMultiMove).toColumn as i32
            }) as i16;
    let mut rowChange: i16 = (if ((*sMultiMove).fromRow as i32 - (*sMultiMove).cursorRow as i32) < 0
    {
        -((*sMultiMove).fromRow as i32 - (*sMultiMove).cursorRow as i32)
    } else {
        (*sMultiMove).fromRow as i32 - (*sMultiMove).cursorRow as i32
    }) as i16
        - (if ((*sMultiMove).fromRow as i32 - (*sMultiMove).toRow as i32) < 0 {
            -((*sMultiMove).fromRow as i32 - (*sMultiMove).toRow as i32)
        } else {
            (*sMultiMove).fromRow as i32 - (*sMultiMove).toRow as i32
        }) as i16;
    if columnChange > 0 {
        MultiMove_SelectColumn(
            (*sMultiMove).cursorColumn,
            (*sMultiMove).fromRow,
            (*sMultiMove).toRow,
        );
    }
    if columnChange < 0 {
        MultiMove_DeselectColumn(
            (*sMultiMove).toColumn,
            (*sMultiMove).fromRow,
            (*sMultiMove).toRow,
        );
        MultiMove_SelectColumn(
            (*sMultiMove).cursorColumn,
            (*sMultiMove).fromRow,
            (*sMultiMove).toRow,
        );
    }
    if rowChange > 0 {
        MultiMove_SelectRow(
            (*sMultiMove).cursorRow,
            (*sMultiMove).fromColumn,
            (*sMultiMove).toColumn,
        );
    }
    if rowChange < 0 {
        MultiMove_DeselectRow(
            (*sMultiMove).toRow,
            (*sMultiMove).fromColumn,
            (*sMultiMove).toColumn,
        );
        MultiMove_SelectRow(
            (*sMultiMove).cursorRow,
            (*sMultiMove).fromColumn,
            (*sMultiMove).toColumn,
        );
    }
}
pub(crate) unsafe extern "C" fn MultiMove_SelectColumn(column: u8, mut minRow: u8, mut maxRow: u8) {
    if minRow > maxRow {
        let mut temp: u8 = 0;
        temp = minRow;
        minRow = maxRow;
        maxRow = temp;
    }
    while minRow <= maxRow {
        MultiMove_SetIconToBg(column, {
            let t1 = minRow;
            minRow += 1;
            t1
        });
    }
}
pub(crate) unsafe extern "C" fn MultiMove_SelectRow(row: u8, mut minColumn: u8, mut maxColumn: u8) {
    if minColumn > maxColumn {
        let mut temp: u8 = 0;
        temp = minColumn;
        minColumn = maxColumn;
        maxColumn = temp;
    }
    while minColumn <= maxColumn {
        MultiMove_SetIconToBg(
            {
                let t1 = minColumn;
                minColumn += 1;
                t1
            },
            row,
        );
    }
}
pub(crate) unsafe extern "C" fn MultiMove_DeselectColumn(
    column: u8,
    mut minRow: u8,
    mut maxRow: u8,
) {
    if minRow > maxRow {
        let mut temp: u8 = 0;
        temp = minRow;
        minRow = maxRow;
        maxRow = temp;
    }
    while minRow <= maxRow {
        MultiMove_ClearIconFromBg(column, {
            let t1 = minRow;
            minRow += 1;
            t1
        });
    }
}
pub(crate) unsafe extern "C" fn MultiMove_DeselectRow(
    row: u8,
    mut minColumn: u8,
    mut maxColumn: u8,
) {
    if minColumn > maxColumn {
        let mut temp: u8 = 0;
        temp = minColumn;
        minColumn = maxColumn;
        maxColumn = temp;
    }
    while minColumn <= maxColumn {
        MultiMove_ClearIconFromBg(
            {
                let t1 = minColumn;
                minColumn += 1;
                t1
            },
            row,
        );
    }
}
pub(crate) unsafe extern "C" fn MultiMove_SetIconToBg(x: u8, y: u8) {
    let mut position: u8 = x + IN_BOX_COLUMNS * y;
    let mut species: u16 = GetCurrentBoxMonData(position, MON_DATA_SPECIES_OR_EGG) as u16;
    let mut personality: u32 = GetCurrentBoxMonData(position, MON_DATA_PERSONALITY);
    if species != SPECIES_NONE {
        let mut iconGfx: *mut u8 = GetMonIconPtr(species, personality, 1);
        let mut index: u8 = GetValidMonIconPalIndex(species) + 8;
        BlitBitmapRectToWindow4BitTo8Bit(
            (*sStorage).multiMoveWindowId as u8,
            iconGfx,
            0,
            0,
            32,
            32,
            24 * x as u16,
            24 * y as u16,
            32,
            32,
            index,
        );
    }
}
pub(crate) unsafe extern "C" fn MultiMove_ClearIconFromBg(x: u8, y: u8) {
    let mut position: u8 = x + IN_BOX_COLUMNS * y;
    let mut species: u16 = GetCurrentBoxMonData(position, MON_DATA_SPECIES_OR_EGG) as u16;
    if species != SPECIES_NONE {
        FillWindowPixelRect8Bit(
            (*sStorage).multiMoveWindowId as u8,
            0,
            24 * x as u16,
            24 * y as u16,
            32,
            32,
        );
    }
}
pub(crate) unsafe extern "C" fn MultiMove_InitMove(x: u16, y: u16, moveSteps: u16) {
    (*sMultiMove).bgX = x;
    (*sMultiMove).bgY = y;
    (*sMultiMove).bgMoveSteps = moveSteps;
}
pub(crate) unsafe extern "C" fn MultiMove_UpdateMove() -> u8 {
    if (*sMultiMove).bgMoveSteps != 0 {
        ChangeBgX(0, (*sMultiMove).bgX as i32, BG_COORD_ADD);
        ChangeBgY(0, (*sMultiMove).bgY as i32, BG_COORD_ADD);
        (*sMultiMove).bgMoveSteps -= 1;
    }
    return (*sMultiMove).bgMoveSteps as u8;
}
pub(crate) unsafe extern "C" fn MultiMove_GetMonsFromSelection() {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut columnCount: i32 = 0;
    let mut rowCount: i32 = 0;
    let mut boxId: u8 = 0;
    let mut monArrayId: u8 = 0;
    (*sMultiMove).minColumn = if (*sMultiMove).fromColumn < (*sMultiMove).toColumn {
        (*sMultiMove).fromColumn
    } else {
        (*sMultiMove).toColumn
    };
    (*sMultiMove).minRow = if (*sMultiMove).fromRow < (*sMultiMove).toRow {
        (*sMultiMove).fromRow
    } else {
        (*sMultiMove).toRow
    };
    (*sMultiMove).columnsTotal =
        (if ((*sMultiMove).fromColumn as i32 - (*sMultiMove).toColumn as i32) < 0 {
            -((*sMultiMove).fromColumn as i32 - (*sMultiMove).toColumn as i32)
        } else {
            (*sMultiMove).fromColumn as i32 - (*sMultiMove).toColumn as i32
        }) as u8
            + 1;
    (*sMultiMove).rowsTotal = (if ((*sMultiMove).fromRow as i32 - (*sMultiMove).toRow as i32) < 0 {
        -((*sMultiMove).fromRow as i32 - (*sMultiMove).toRow as i32)
    } else {
        (*sMultiMove).fromRow as i32 - (*sMultiMove).toRow as i32
    }) as u8
        + 1;
    boxId = StorageGetCurrentBox();
    monArrayId = 0;
    columnCount = (*sMultiMove).minColumn as i32 + (*sMultiMove).columnsTotal as i32;
    rowCount = (*sMultiMove).minRow as i32 + (*sMultiMove).rowsTotal as i32;
    i = (*sMultiMove).minRow as i32;
    while i < rowCount {
        let mut boxPosition: u8 = IN_BOX_COLUMNS * i as u8 + (*sMultiMove).minColumn;
        j = (*sMultiMove).minColumn as i32;
        while j < columnCount {
            let mut boxMon: *mut BoxPokemon = GetBoxedMonPtr(boxId, boxPosition);
            if !boxMon.is_null() {
                (*sMultiMove).boxMons[monArrayId] = *boxMon;
            }
            monArrayId += 1;
            boxPosition += 1;
            j += 1;
        }
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn MultiMove_RemoveMonsFromBox() {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut columnCount: i32 = (*sMultiMove).minColumn as i32 + (*sMultiMove).columnsTotal as i32;
    let mut rowCount: i32 = (*sMultiMove).minRow as i32 + (*sMultiMove).rowsTotal as i32;
    let mut boxId: u8 = StorageGetCurrentBox();
    i = (*sMultiMove).minRow as i32;
    while i < rowCount {
        let mut boxPosition: u8 = IN_BOX_COLUMNS * i as u8 + (*sMultiMove).minColumn;
        j = (*sMultiMove).minColumn as i32;
        while j < columnCount {
            DestroyBoxMonIconAtPosition(boxPosition);
            ZeroBoxMonAt(boxId, boxPosition);
            boxPosition += 1;
            j += 1;
        }
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn MultiMove_CreatePlacedMonIcons() {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut columnCount: i32 = (*sMultiMove).minColumn as i32 + (*sMultiMove).columnsTotal as i32;
    let mut rowCount: i32 = (*sMultiMove).minRow as i32 + (*sMultiMove).rowsTotal as i32;
    let mut monArrayId: u8 = 0;
    i = (*sMultiMove).minRow as i32;
    while i < rowCount {
        let mut boxPosition: u8 = IN_BOX_COLUMNS * i as u8 + (*sMultiMove).minColumn;
        j = (*sMultiMove).minColumn as i32;
        while j < columnCount {
            if GetBoxMonData2(
                &raw mut (*sMultiMove).boxMons[monArrayId],
                MON_DATA_SANITY_HAS_SPECIES,
            ) != 0
            {
                CreateBoxMonIconAtPos(boxPosition);
            }
            monArrayId += 1;
            boxPosition += 1;
            j += 1;
        }
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn MultiMove_SetPlacedMonData() {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut columnCount: i32 = (*sMultiMove).minColumn as i32 + (*sMultiMove).columnsTotal as i32;
    let mut rowCount: i32 = (*sMultiMove).minRow as i32 + (*sMultiMove).rowsTotal as i32;
    let mut boxId: u8 = StorageGetCurrentBox();
    let mut monArrayId: u8 = 0;
    i = (*sMultiMove).minRow as i32;
    while i < rowCount {
        let mut boxPosition: u8 = IN_BOX_COLUMNS * i as u8 + (*sMultiMove).minColumn;
        j = (*sMultiMove).minColumn as i32;
        while j < columnCount {
            if GetBoxMonData2(
                &raw mut (*sMultiMove).boxMons[monArrayId],
                MON_DATA_SANITY_HAS_SPECIES,
            ) != 0
            {
                SetBoxMonAt(
                    boxId,
                    boxPosition,
                    &raw mut (*sMultiMove).boxMons[monArrayId],
                );
            }
            boxPosition += 1;
            monArrayId += 1;
            j += 1;
        }
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn MultiMove_ResetBg() {
    ChangeBgX(0, 0, BG_COORD_SET);
    ChangeBgY(0, 0, BG_COORD_SET);
    SetBgAttribute(0, BG_ATTR_PALETTEMODE, 0);
    ClearGpuRegBits(REG_OFFSET_BG0CNT, BGCNT_256COLOR);
    FillBgTilemapBufferRect_Palette0(0, 0, 0, 0, 32, 32);
    CopyBgTilemapBufferToVram(0);
}
pub(crate) unsafe extern "C" fn MultiMove_GetOrigin() -> u8 {
    return IN_BOX_COLUMNS * (*sMultiMove).fromRow + (*sMultiMove).fromColumn;
}
pub(crate) unsafe extern "C" fn MultiMove_CanPlaceSelection() -> u8 {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut columnCount: i32 = (*sMultiMove).minColumn as i32 + (*sMultiMove).columnsTotal as i32;
    let mut rowCount: i32 = (*sMultiMove).minRow as i32 + (*sMultiMove).rowsTotal as i32;
    let mut monArrayId: u8 = 0;
    i = (*sMultiMove).minRow as i32;
    while i < rowCount {
        let mut boxPosition: u8 = IN_BOX_COLUMNS * i as u8 + (*sMultiMove).minColumn;
        j = (*sMultiMove).minColumn as i32;
        while j < columnCount {
            if GetBoxMonData2(
                &raw mut (*sMultiMove).boxMons[monArrayId],
                MON_DATA_SANITY_HAS_SPECIES,
            ) != 0
                && GetCurrentBoxMonData(boxPosition, MON_DATA_SANITY_HAS_SPECIES) != 0
            {
                return FALSE;
            }
            monArrayId += 1;
            boxPosition += 1;
            j += 1;
        }
        i += 1;
    }
    return TRUE;
}
pub(crate) unsafe extern "C" fn CreateItemIconSprites() {
    let mut i: i32 = 0;
    let mut spriteId: u8 = 0;
    let mut spriteSheet: CompressedSpriteSheet = zeroed();
    let mut spriteTemplate: SpriteTemplate = zeroed();
    if (*sStorage).boxOption == OPTION_MOVE_ITEMS {
        spriteSheet.data = sItemIconGfxBuffer.as_mut_ptr();
        spriteSheet.size = 0x200;
        spriteTemplate = *sSpriteTemplate_ItemIcon;
        i = 0;
        while i < MAX_ITEM_ICONS as i32 {
            spriteSheet.tag = GFXTAG_ITEM_ICON_0 + i as u16;
            LoadCompressedSpriteSheet(&raw mut spriteSheet);
            (*sStorage).itemIcons[i].tiles = (OBJ_VRAM0 as usize as *mut c_void as *mut u8)
                .at(GetSpriteTileStartByTag(spriteSheet.tag) as i32 * 32)
                as *mut c_void as *mut u8;
            (*sStorage).itemIcons[i].palIndex =
                AllocSpritePalette(PALTAG_ITEM_ICON_0 + i as u16) as u16;
            (*sStorage).itemIcons[i].palIndex = 0x100 + (*sStorage).itemIcons[i].palIndex * 16;
            spriteTemplate.tileTag = GFXTAG_ITEM_ICON_0 + i as u16;
            spriteTemplate.paletteTag = PALTAG_ITEM_ICON_0 + i as u16;
            spriteId = CreateSprite(&raw mut spriteTemplate, 0, 0, 11);
            (*sStorage).itemIcons[i].sprite = &raw mut gSprites[spriteId];
            (*(*sStorage).itemIcons[i].sprite).set_invisible(TRUE as u16);
            (*sStorage).itemIcons[i].active = FALSE;
            i += 1;
        }
    }
    (*sStorage).movingItemId = ITEM_NONE;
}
pub(crate) unsafe extern "C" fn TryLoadItemIconAtPos(cursorArea: u8, cursorPos: u8) {
    let mut heldItem: u16 = 0;
    if (*sStorage).boxOption != OPTION_MOVE_ITEMS {
        return;
    }
    if IsItemIconAtPosition(cursorArea, cursorPos) != 0 {
        return;
    }
    match cursorArea {
        CURSOR_AREA_IN_BOX => {
            if GetCurrentBoxMonData(cursorPos, MON_DATA_SANITY_HAS_SPECIES) == 0 {
                return;
            }
            heldItem = GetCurrentBoxMonData(cursorPos, MON_DATA_HELD_ITEM) as u16;
        }
        1 => {
            if cursorPos >= PARTY_SIZE as u8
                || GetMonData2(
                    &raw mut gPlayerParty[cursorPos],
                    MON_DATA_SANITY_HAS_SPECIES,
                ) == 0
            {
                return;
            }
            heldItem = GetMonData2(&raw mut gPlayerParty[cursorPos], MON_DATA_HELD_ITEM) as u16;
        }
        _ => {
            return;
        }
    }
    if heldItem != ITEM_NONE {
        let mut tiles: *mut u32 = GetItemIconPic(heldItem);
        let mut pal: *mut u32 = GetItemIconPalette(heldItem);
        let mut id: u8 = GetNewItemIconIdx();
        SetItemIconPosition(id, cursorArea, cursorPos);
        LoadItemIconGfx(id, tiles, pal);
        SetItemIconAffineAnim(id, ITEM_ANIM_APPEAR);
        SetItemIconActive(id, TRUE);
    }
}
pub(crate) unsafe extern "C" fn TryHideItemIconAtPos(cursorArea: u8, cursorPos: u8) {
    let mut id: u8 = 0;
    if (*sStorage).boxOption != OPTION_MOVE_ITEMS {
        return;
    }
    id = GetItemIconIdxByPosition(cursorArea, cursorPos);
    SetItemIconAffineAnim(id, ITEM_ANIM_DISAPPEAR);
    SetItemIconCallback(id, ITEM_CB_WAIT_ANIM, cursorArea, cursorPos);
}
pub(crate) unsafe extern "C" fn TakeItemFromMon(cursorArea: u8, cursorPos: u8) {
    let mut id: u8 = 0;
    let mut itemId: u16 = 0;
    if (*sStorage).boxOption != OPTION_MOVE_ITEMS {
        return;
    }
    id = GetItemIconIdxByPosition(cursorArea, cursorPos);
    itemId = ITEM_NONE;
    SetItemIconAffineAnim(id, ITEM_ANIM_PICK_UP);
    SetItemIconCallback(id, ITEM_CB_TO_HAND, cursorArea, cursorPos);
    SetItemIconPosition(id, CURSOR_AREA_BOX_TITLE, 0);
    if cursorArea == CURSOR_AREA_IN_BOX {
        SetCurrentBoxMonData(
            cursorPos,
            MON_DATA_HELD_ITEM,
            &raw mut itemId as *mut c_void,
        );
        SetBoxMonIconObjMode(cursorPos, ST_OAM_OBJ_BLEND as u8);
    } else {
        SetMonData(
            &raw mut gPlayerParty[cursorPos],
            MON_DATA_HELD_ITEM,
            &raw mut itemId as *mut c_void,
        );
        SetPartyMonIconObjMode(cursorPos, ST_OAM_OBJ_BLEND as u8);
    }
    (*sStorage).movingItemId = (*sStorage).displayMonItemId;
}
pub(crate) unsafe extern "C" fn InitItemIconInCursor(itemId: u16) {
    let mut tiles: *mut u32 = GetItemIconPic(itemId);
    let mut pal: *mut u32 = GetItemIconPalette(itemId);
    let mut id: u8 = GetNewItemIconIdx();
    LoadItemIconGfx(id, tiles, pal);
    SetItemIconAffineAnim(id, ITEM_ANIM_LARGE);
    SetItemIconCallback(id, ITEM_CB_TO_HAND, CURSOR_AREA_IN_BOX, 0);
    SetItemIconPosition(id, CURSOR_AREA_BOX_TITLE, 0);
    SetItemIconActive(id, TRUE);
    (*sStorage).movingItemId = itemId;
}
pub(crate) unsafe extern "C" fn SwapItemsWithMon(cursorArea: u8, cursorPos: u8) {
    let mut id: u8 = 0;
    let mut itemId: u16 = 0;
    if (*sStorage).boxOption != OPTION_MOVE_ITEMS {
        return;
    }
    id = GetItemIconIdxByPosition(cursorArea, cursorPos);
    SetItemIconAffineAnim(id, ITEM_ANIM_PICK_UP);
    SetItemIconCallback(id, ITEM_CB_SWAP_TO_HAND, CURSOR_AREA_BOX_TITLE, 0);
    if cursorArea == CURSOR_AREA_IN_BOX {
        itemId = GetCurrentBoxMonData(cursorPos, MON_DATA_HELD_ITEM) as u16;
        SetCurrentBoxMonData(
            cursorPos,
            MON_DATA_HELD_ITEM,
            &raw mut (*sStorage).movingItemId as *mut c_void,
        );
        (*sStorage).movingItemId = itemId;
    } else {
        itemId = GetMonData2(&raw mut gPlayerParty[cursorPos], MON_DATA_HELD_ITEM) as u16;
        SetMonData(
            &raw mut gPlayerParty[cursorPos],
            MON_DATA_HELD_ITEM,
            &raw mut (*sStorage).movingItemId as *mut c_void,
        );
        (*sStorage).movingItemId = itemId;
    }
    id = GetItemIconIdxByPosition(CURSOR_AREA_BOX_TITLE, 0);
    SetItemIconAffineAnim(id, ITEM_ANIM_PUT_DOWN);
    SetItemIconCallback(id, ITEM_CB_SWAP_TO_MON, cursorArea, cursorPos);
}
pub(crate) unsafe extern "C" fn GiveItemToMon(cursorArea: u8, cursorPos: u8) {
    let mut id: u8 = 0;
    if (*sStorage).boxOption != OPTION_MOVE_ITEMS {
        return;
    }
    id = GetItemIconIdxByPosition(CURSOR_AREA_BOX_TITLE, 0);
    SetItemIconAffineAnim(id, ITEM_ANIM_PUT_DOWN);
    SetItemIconCallback(id, ITEM_CB_TO_MON, cursorArea, cursorPos);
    if cursorArea == CURSOR_AREA_IN_BOX {
        SetCurrentBoxMonData(
            cursorPos,
            MON_DATA_HELD_ITEM,
            &raw mut (*sStorage).movingItemId as *mut c_void,
        );
        SetBoxMonIconObjMode(cursorPos, ST_OAM_OBJ_NORMAL);
    } else {
        SetMonData(
            &raw mut gPlayerParty[cursorPos],
            MON_DATA_HELD_ITEM,
            &raw mut (*sStorage).movingItemId as *mut c_void,
        );
        SetPartyMonIconObjMode(cursorPos, ST_OAM_OBJ_NORMAL);
    }
}
pub(crate) unsafe extern "C" fn MoveItemFromMonToBag(cursorArea: u8, cursorPos: u8) {
    let mut id: u8 = 0;
    let mut itemId: u16 = 0;
    if (*sStorage).boxOption != OPTION_MOVE_ITEMS {
        return;
    }
    itemId = ITEM_NONE;
    id = GetItemIconIdxByPosition(cursorArea, cursorPos);
    SetItemIconAffineAnim(id, ITEM_ANIM_DISAPPEAR);
    SetItemIconCallback(id, ITEM_CB_WAIT_ANIM, cursorArea, cursorPos);
    if cursorArea == CURSOR_AREA_IN_BOX {
        SetCurrentBoxMonData(
            cursorPos,
            MON_DATA_HELD_ITEM,
            &raw mut itemId as *mut c_void,
        );
        SetBoxMonIconObjMode(cursorPos, ST_OAM_OBJ_BLEND as u8);
    } else {
        SetMonData(
            &raw mut gPlayerParty[cursorPos],
            MON_DATA_HELD_ITEM,
            &raw mut itemId as *mut c_void,
        );
        SetPartyMonIconObjMode(cursorPos, ST_OAM_OBJ_BLEND as u8);
    }
}
pub(crate) unsafe extern "C" fn MoveItemFromCursorToBag() {
    if (*sStorage).boxOption == OPTION_MOVE_ITEMS {
        let mut id: u8 = GetItemIconIdxByPosition(CURSOR_AREA_BOX_TITLE, 0);
        SetItemIconAffineAnim(id, ITEM_ANIM_PUT_AWAY);
        SetItemIconCallback(id, ITEM_CB_WAIT_ANIM, CURSOR_AREA_BOX_TITLE, 0);
    }
}
pub(crate) unsafe extern "C" fn MoveHeldItemWithPartyMenu() {
    let mut i: i32 = 0;
    if (*sStorage).boxOption != OPTION_MOVE_ITEMS {
        return;
    }
    i = 0;
    while i < MAX_ITEM_ICONS as i32 {
        if (*sStorage).itemIcons[i].active != 0
            && (*sStorage).itemIcons[i].area == CURSOR_AREA_IN_PARTY as u8
        {
            SetItemIconCallback(i as u8, ITEM_CB_HIDE_PARTY, CURSOR_AREA_BOX_TITLE, 0);
        }
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn IsItemIconAnimActive() -> u8 {
    let mut i: i32 = 0;
    i = 0;
    while i < MAX_ITEM_ICONS as i32 {
        if (*sStorage).itemIcons[i].active != 0 {
            if (*(*sStorage).itemIcons[i].sprite).affineAnimEnded() == 0
                && (*(*sStorage).itemIcons[i].sprite).affineAnimBeginning() != 0
            {
                return TRUE;
            }
            if (*(*sStorage).itemIcons[i].sprite).callback
                != Some(SpriteCallbackDummy as unsafe extern "C" fn(*mut Sprite))
                && (*(*sStorage).itemIcons[i].sprite).callback
                    != Some(SpriteCB_ItemIcon_SetPosToCursor as unsafe extern "C" fn(*mut Sprite))
            {
                return TRUE;
            }
        }
        i += 1;
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn IsMovingItem() -> u8 {
    let mut i: i32 = 0;
    if (*sStorage).boxOption == OPTION_MOVE_ITEMS {
        i = 0;
        while i < MAX_ITEM_ICONS as i32 {
            if (*sStorage).itemIcons[i].active != 0
                && (*sStorage).itemIcons[i].area == CURSOR_AREA_BOX_TITLE
            {
                return TRUE;
            }
            i += 1;
        }
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn GetMovingItemName() -> *mut u8 {
    return GetItemName((*sStorage).movingItemId);
}
pub(crate) unsafe extern "C" fn GetMovingItemId() -> u16 {
    return (*sStorage).movingItemId;
}
pub(crate) unsafe extern "C" fn GetNewItemIconIdx() -> u8 {
    let mut i: u8 = 0;
    i = 0;
    while i < MAX_ITEM_ICONS {
        if (*sStorage).itemIcons[i].active == 0 {
            (*sStorage).itemIcons[i].active = TRUE;
            return i;
        }
        i += 1;
    }
    return MAX_ITEM_ICONS;
}
pub(crate) unsafe extern "C" fn IsItemIconAtPosition(cursorArea: u8, cursorPos: u8) -> u32 {
    let mut i: i32 = 0;
    i = 0;
    while i < MAX_ITEM_ICONS as i32 {
        if (*sStorage).itemIcons[i].active != 0
            && (*sStorage).itemIcons[i].area == cursorArea
            && (*sStorage).itemIcons[i].pos == cursorPos
        {
            return TRUE as u32;
        }
        i += 1;
    }
    return FALSE as u32;
}
pub(crate) unsafe extern "C" fn GetItemIconIdxByPosition(cursorArea: u8, cursorPos: u8) -> u8 {
    let mut i: u8 = 0;
    i = 0;
    while i < MAX_ITEM_ICONS {
        if (*sStorage).itemIcons[i].active != 0
            && (*sStorage).itemIcons[i].area == cursorArea
            && (*sStorage).itemIcons[i].pos == cursorPos
        {
            return i;
        }
        i += 1;
    }
    return MAX_ITEM_ICONS;
}
pub(crate) unsafe extern "C" fn GetItemIconIdxBySprite(sprite: *mut Sprite) -> u8 {
    let mut i: u8 = 0;
    i = 0;
    while i < MAX_ITEM_ICONS {
        if (*sStorage).itemIcons[i].active != 0 && (*sStorage).itemIcons[i].sprite == sprite {
            return i;
        }
        i += 1;
    }
    return MAX_ITEM_ICONS;
}
pub(crate) unsafe extern "C" fn SetItemIconPosition(id: u8, cursorArea: u8, cursorPos: u8) {
    let mut x: u8 = 0;
    let mut y: u8 = 0;
    if id >= MAX_ITEM_ICONS {
        return;
    }
    match cursorArea {
        CURSOR_AREA_IN_BOX => {
            x = (cursorPos as i32 % 6) as u8;
            y = (cursorPos as i32 / 6) as u8;
            (*(*sStorage).itemIcons[id].sprite).x = 24 * x as i16 + 112;
            (*(*sStorage).itemIcons[id].sprite).y = 24 * y as i16 + 56;
            (*(*sStorage).itemIcons[id].sprite).oam.set_priority(2);
        }
        1 => {
            if cursorPos == 0 {
                (*(*sStorage).itemIcons[id].sprite).x = 116;
                (*(*sStorage).itemIcons[id].sprite).y = 76;
            } else {
                (*(*sStorage).itemIcons[id].sprite).x = 164;
                (*(*sStorage).itemIcons[id].sprite).y = 24 * (cursorPos as i16 - 1) + 28;
            }
            (*(*sStorage).itemIcons[id].sprite).oam.set_priority(1);
        }
        _ => {}
    }
    (*sStorage).itemIcons[id].area = cursorArea;
    (*sStorage).itemIcons[id].pos = cursorPos;
}
pub(crate) unsafe extern "C" fn LoadItemIconGfx(id: u8, itemTiles: *mut u32, itemPal: *mut u32) {
    let mut i: i32 = 0;
    if id >= MAX_ITEM_ICONS {
        return;
    }
    {
        let mut tmp: u32 = 0;
        volatile_write(&raw mut tmp, 0);
        CpuFastSet(
            &raw mut tmp as *mut c_void,
            (*sStorage).itemIconBuffer.as_mut_ptr() as *mut c_void,
            0x1000080,
        );
    }
    LZ77UnCompWram(
        itemTiles,
        (*sStorage).tileBuffer.as_mut_ptr() as *mut c_void,
    );
    i = 0;
    while i < 3 {
        CpuFastSet(
            &raw mut (*sStorage).tileBuffer[i * 0x60] as *mut c_void,
            &raw mut (*sStorage).itemIconBuffer[i * 0x80] as *mut c_void,
            24,
        );
        i += 1;
    }
    CpuFastSet(
        (*sStorage).itemIconBuffer.as_mut_ptr() as *mut c_void,
        (*sStorage).itemIcons[id].tiles as *mut c_void,
        128,
    );
    LZ77UnCompWram(
        itemPal,
        (*sStorage).itemIconBuffer.as_mut_ptr() as *mut c_void,
    );
    LoadPalette(
        (*sStorage).itemIconBuffer.as_mut_ptr() as *mut c_void,
        (*sStorage).itemIcons[id].palIndex,
        32,
    );
}
pub(crate) unsafe extern "C" fn SetItemIconAffineAnim(id: u8, animNum: u8) {
    if id >= MAX_ITEM_ICONS {
        return;
    }
    StartSpriteAffineAnim((*sStorage).itemIcons[id].sprite, animNum);
}
pub(crate) unsafe extern "C" fn SetItemIconCallback(
    id: u8,
    callbackId: u8,
    cursorArea: u8,
    cursorPos: u8,
) {
    if id >= MAX_ITEM_ICONS {
        return;
    }
    match callbackId {
        ITEM_CB_WAIT_ANIM => {
            (*(*sStorage).itemIcons[id].sprite).data[0] = id as i16;
            (*(*sStorage).itemIcons[id].sprite).callback = Some(SpriteCB_ItemIcon_WaitAnim);
        }
        ITEM_CB_TO_HAND => {
            (*(*sStorage).itemIcons[id].sprite).data[0] = 0;
            (*(*sStorage).itemIcons[id].sprite).callback = Some(SpriteCB_ItemIcon_ToHand);
        }
        ITEM_CB_TO_MON => {
            (*(*sStorage).itemIcons[id].sprite).data[0] = 0;
            (*(*sStorage).itemIcons[id].sprite).data[6] = cursorArea as i16;
            (*(*sStorage).itemIcons[id].sprite).data[7] = cursorPos as i16;
            (*(*sStorage).itemIcons[id].sprite).callback = Some(SpriteCB_ItemIcon_ToMon);
        }
        ITEM_CB_SWAP_TO_HAND => {
            (*(*sStorage).itemIcons[id].sprite).data[0] = 0;
            (*(*sStorage).itemIcons[id].sprite).callback = Some(SpriteCB_ItemIcon_SwapToHand);
            (*(*sStorage).itemIcons[id].sprite).data[6] = cursorArea as i16;
            (*(*sStorage).itemIcons[id].sprite).data[7] = cursorPos as i16;
        }
        ITEM_CB_SWAP_TO_MON => {
            (*(*sStorage).itemIcons[id].sprite).data[0] = 0;
            (*(*sStorage).itemIcons[id].sprite).data[6] = cursorArea as i16;
            (*(*sStorage).itemIcons[id].sprite).data[7] = cursorPos as i16;
            (*(*sStorage).itemIcons[id].sprite).callback = Some(SpriteCB_ItemIcon_SwapToMon);
        }
        ITEM_CB_HIDE_PARTY => {
            (*(*sStorage).itemIcons[id].sprite).callback = Some(SpriteCB_ItemIcon_HideParty);
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn SetItemIconActive(id: u8, active: u8) {
    if id >= MAX_ITEM_ICONS {
        return;
    }
    (*sStorage).itemIcons[id].active = active;
    (*(*sStorage).itemIcons[id].sprite).set_invisible((active == FALSE) as u16);
}
pub(crate) unsafe extern "C" fn GetItemIconPic(itemId: u16) -> *mut u32 {
    return GetItemIconPicOrPalette(itemId, 0) as *mut u32;
}
pub(crate) unsafe extern "C" fn GetItemIconPalette(itemId: u16) -> *mut u32 {
    return GetItemIconPicOrPalette(itemId, 1) as *mut u32;
}
pub(crate) unsafe extern "C" fn PrintItemDescription() {
    let mut description: *mut u8 = null_mut();
    if IsMovingItem() != 0 {
        description = GetItemDescription((*sStorage).movingItemId);
    } else {
        description = GetItemDescription((*sStorage).displayMonItemId);
    }
    FillWindowPixelBuffer(WIN_ITEM_DESC, 17);
    AddTextPrinterParameterized5(WIN_ITEM_DESC, FONT_NORMAL, description, 4, 0, 0, None, 0, 1);
}
pub(crate) unsafe extern "C" fn InitItemInfoWindow() {
    (*sStorage).itemInfoWindowOffset = 21;
    LoadBgTiles(
        0,
        sItemInfoFrame_Gfx.as_ptr().cast_mut() as *mut c_void,
        0x80,
        0x13A,
    );
    DrawItemInfoWindow(0);
}
pub(crate) unsafe extern "C" fn UpdateItemInfoWindowSlideIn() -> u8 {
    let mut i: i32 = 0;
    let mut pos: i32 = 0;
    if (*sStorage).itemInfoWindowOffset == 0 {
        return FALSE;
    }
    (*sStorage).itemInfoWindowOffset -= 1;
    pos = 21 - (*sStorage).itemInfoWindowOffset as i32;
    i = 0;
    while i < pos {
        WriteSequenceToBgTilemapBuffer(
            0,
            GetBgAttribute(0, BG_ATTR_BASETILE)
                + 0x14
                + (*sStorage).itemInfoWindowOffset
                + i as u16,
            i as u8,
            13,
            1,
            7,
            15,
            21,
        );
        i += 1;
    }
    DrawItemInfoWindow(pos as u32);
    return ((*sStorage).itemInfoWindowOffset != 0) as u8;
}
pub(crate) unsafe extern "C" fn UpdateItemInfoWindowSlideOut() -> u8 {
    let mut i: i32 = 0;
    let mut pos: i32 = 0;
    if (*sStorage).itemInfoWindowOffset == 22 {
        return FALSE;
    }
    if (*sStorage).itemInfoWindowOffset == 0 {
        FillBgTilemapBufferRect(0, 0, 21, 12, 1, 9, 17);
    }
    (*sStorage).itemInfoWindowOffset += 1;
    pos = 21 - (*sStorage).itemInfoWindowOffset as i32;
    i = 0;
    while i < pos {
        WriteSequenceToBgTilemapBuffer(
            0,
            GetBgAttribute(0, BG_ATTR_BASETILE)
                + 0x14
                + (*sStorage).itemInfoWindowOffset
                + i as u16,
            i as u8,
            13,
            1,
            7,
            15,
            21,
        );
        i += 1;
    }
    if pos >= 0 {
        DrawItemInfoWindow(pos as u32);
    }
    FillBgTilemapBufferRect(0, 0, pos as u8 + 1, 12, 1, 9, 17);
    ScheduleBgCopyTilemapToVram(0);
    return TRUE;
}
pub(crate) unsafe extern "C" fn DrawItemInfoWindow(x: u32) {
    if x != 0 {
        FillBgTilemapBufferRect(0, 0x13A, 0, 0xC, x as u8, 1, 15);
        FillBgTilemapBufferRect(0, 0x93A, 0, 0x14, x as u8, 1, 15);
    }
    FillBgTilemapBufferRect(0, 0x13B, x as u8, 0xD, 1, 7, 15);
    FillBgTilemapBufferRect(0, 0x13C, x as u8, 0xC, 1, 1, 15);
    FillBgTilemapBufferRect(0, 0x13D, x as u8, 0x14, 1, 1, 15);
    ScheduleBgCopyTilemapToVram(0);
}
pub(crate) unsafe extern "C" fn SpriteCB_ItemIcon_WaitAnim(sprite: *mut Sprite) {
    if (*sprite).affineAnimEnded() != 0 {
        SetItemIconActive((*sprite).data[0] as u8, FALSE);
        (*sprite).callback = Some(SpriteCallbackDummy);
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_ItemIcon_ToHand(sprite: *mut Sprite) {
    'l1: {
        let sw1: i16 = (*sprite).data[0];
        let mut fall = false;
        if sw1 == 0 {
            fall = true;
            (*sprite).data[1] = (*sprite).x << 4;
            (*sprite).data[2] = (*sprite).y << 4;
            (*sprite).data[3] = 10;
            (*sprite).data[4] = 21;
            (*sprite).data[5] = 0;
            (*sprite).data[0] += 1;
        }
        if fall || sw1 == 1 {
            fall = true;
            (*sprite).data[1] -= (*sprite).data[3];
            (*sprite).data[2] -= (*sprite).data[4];
            (*sprite).x = (*sprite).data[1] >> 4;
            (*sprite).y = (*sprite).data[2] >> 4;
            if ({
                (*sprite).data[5] += 1;
                (*sprite).data[5]
            }) > 11
            {
                (*sprite).callback = Some(SpriteCB_ItemIcon_SetPosToCursor);
            }
            break 'l1;
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_ItemIcon_SetPosToCursor(sprite: *mut Sprite) {
    (*sprite).x = (*(*sStorage).cursorSprite).x + 4;
    (*sprite).y = (*(*sStorage).cursorSprite).y + (*(*sStorage).cursorSprite).y2 + 8;
    (*sprite)
        .oam
        .set_priority((*(*sStorage).cursorSprite).oam.priority());
}
pub(crate) unsafe extern "C" fn SpriteCB_ItemIcon_ToMon(sprite: *mut Sprite) {
    'l1: {
        let sw1: i16 = (*sprite).data[0];
        let mut fall = false;
        if sw1 == 0 {
            fall = true;
            (*sprite).data[1] = (*sprite).x << 4;
            (*sprite).data[2] = (*sprite).y << 4;
            (*sprite).data[3] = 10;
            (*sprite).data[4] = 21;
            (*sprite).data[5] = 0;
            (*sprite).data[0] += 1;
        }
        if fall || sw1 == 1 {
            fall = true;
            (*sprite).data[1] += (*sprite).data[3];
            (*sprite).data[2] += (*sprite).data[4];
            (*sprite).x = (*sprite).data[1] >> 4;
            (*sprite).y = (*sprite).data[2] >> 4;
            if ({
                (*sprite).data[5] += 1;
                (*sprite).data[5]
            }) > 11
            {
                SetItemIconPosition(
                    GetItemIconIdxBySprite(sprite),
                    (*sprite).data[6] as u8,
                    (*sprite).data[7] as u8,
                );
                (*sprite).callback = Some(SpriteCallbackDummy);
            }
            break 'l1;
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_ItemIcon_SwapToHand(sprite: *mut Sprite) {
    'l1: {
        let sw1: i16 = (*sprite).data[0];
        let mut fall = false;
        if sw1 == 0 {
            fall = true;
            (*sprite).data[1] = (*sprite).x << 4;
            (*sprite).data[2] = (*sprite).y << 4;
            (*sprite).data[3] = 10;
            (*sprite).data[4] = 21;
            (*sprite).data[5] = 0;
            (*sprite).data[0] += 1;
        }
        if fall || sw1 == 1 {
            fall = true;
            (*sprite).data[1] -= (*sprite).data[3];
            (*sprite).data[2] -= (*sprite).data[4];
            (*sprite).x = (*sprite).data[1] >> 4;
            (*sprite).y = (*sprite).data[2] >> 4;
            (*sprite).x2 = gSineTable[(*sprite).data[5] as i32 * 8] >> 4;
            if ({
                (*sprite).data[5] += 1;
                (*sprite).data[5]
            }) > 11
            {
                SetItemIconPosition(
                    GetItemIconIdxBySprite(sprite),
                    (*sprite).data[6] as u8,
                    (*sprite).data[7] as u8,
                );
                (*sprite).x2 = 0;
                (*sprite).callback = Some(SpriteCB_ItemIcon_SetPosToCursor);
            }
            break 'l1;
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_ItemIcon_SwapToMon(sprite: *mut Sprite) {
    'l1: {
        let sw1: i16 = (*sprite).data[0];
        let mut fall = false;
        if sw1 == 0 {
            fall = true;
            (*sprite).data[1] = (*sprite).x << 4;
            (*sprite).data[2] = (*sprite).y << 4;
            (*sprite).data[3] = 10;
            (*sprite).data[4] = 21;
            (*sprite).data[5] = 0;
            (*sprite).data[0] += 1;
        }
        if fall || sw1 == 1 {
            fall = true;
            (*sprite).data[1] += (*sprite).data[3];
            (*sprite).data[2] += (*sprite).data[4];
            (*sprite).x = (*sprite).data[1] >> 4;
            (*sprite).y = (*sprite).data[2] >> 4;
            (*sprite).x2 = -(gSineTable[(*sprite).data[5] as i32 * 8] >> 4);
            if ({
                (*sprite).data[5] += 1;
                (*sprite).data[5]
            }) > 11
            {
                SetItemIconPosition(
                    GetItemIconIdxBySprite(sprite),
                    (*sprite).data[6] as u8,
                    (*sprite).data[7] as u8,
                );
                (*sprite).callback = Some(SpriteCallbackDummy);
                (*sprite).x2 = 0;
            }
            break 'l1;
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_ItemIcon_HideParty(sprite: *mut Sprite) {
    (*sprite).y -= 8;
    if ((*sprite).y as i32 + (*sprite).y2 as i32) < -16 {
        (*sprite).callback = Some(SpriteCallbackDummy);
        SetItemIconActive(GetItemIconIdxBySprite(sprite), FALSE);
    }
}
pub(crate) unsafe extern "C" fn BackupPokemonStorage() {}
pub(crate) unsafe extern "C" fn RestorePokemonStorage() {}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn StorageGetCurrentBox() -> u8 {
    return (*gPokemonStoragePtr).currentBox;
}
pub(crate) unsafe extern "C" fn SetCurrentBox(boxId: u8) {
    if boxId < TOTAL_BOXES_COUNT {
        (*gPokemonStoragePtr).currentBox = boxId;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetBoxMonDataAt(boxId: u8, boxPosition: u8, request: i32) -> u32 {
    if boxId < TOTAL_BOXES_COUNT && boxPosition < IN_BOX_COUNT as u8 {
        return GetBoxMonData2(
            &raw mut (*gPokemonStoragePtr).boxes[boxId][boxPosition],
            request,
        );
    } else {
        return 0;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetBoxMonDataAt(
    boxId: u8,
    boxPosition: u8,
    request: i32,
    value: *mut c_void,
) {
    if boxId < TOTAL_BOXES_COUNT && boxPosition < IN_BOX_COUNT as u8 {
        SetBoxMonData(
            &raw mut (*gPokemonStoragePtr).boxes[boxId][boxPosition],
            request,
            value,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetCurrentBoxMonData(boxPosition: u8, request: i32) -> u32 {
    return GetBoxMonDataAt((*gPokemonStoragePtr).currentBox, boxPosition, request);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetCurrentBoxMonData(boxPosition: u8, request: i32, value: *mut c_void) {
    SetBoxMonDataAt(
        (*gPokemonStoragePtr).currentBox,
        boxPosition,
        request,
        value,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetBoxMonNickAt(boxId: u8, boxPosition: u8, dst: *mut u8) {
    if boxId < TOTAL_BOXES_COUNT && boxPosition < IN_BOX_COUNT as u8 {
        GetBoxMonData3(
            &raw mut (*gPokemonStoragePtr).boxes[boxId][boxPosition],
            MON_DATA_NICKNAME,
            dst,
        );
    } else {
        *dst = EOS;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetBoxMonLevelAt(boxId: u8, boxPosition: u8) -> u32 {
    let mut lvl: u32 = 0;
    if boxId < TOTAL_BOXES_COUNT
        && boxPosition < IN_BOX_COUNT as u8
        && GetBoxMonData2(
            &raw mut (*gPokemonStoragePtr).boxes[boxId][boxPosition],
            MON_DATA_SANITY_HAS_SPECIES,
        ) != 0
    {
        lvl =
            GetLevelFromBoxMonExp(&raw mut (*gPokemonStoragePtr).boxes[boxId][boxPosition]) as u32;
    }
    lvl = 0;
    return lvl;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetBoxMonNickAt(boxId: u8, boxPosition: u8, nick: *mut u8) {
    if boxId < TOTAL_BOXES_COUNT && boxPosition < IN_BOX_COUNT as u8 {
        SetBoxMonData(
            &raw mut (*gPokemonStoragePtr).boxes[boxId][boxPosition],
            MON_DATA_NICKNAME,
            nick as *mut c_void,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetAndCopyBoxMonDataAt(
    boxId: u8,
    boxPosition: u8,
    request: i32,
    dst: *mut c_void,
) -> u32 {
    if boxId < TOTAL_BOXES_COUNT && boxPosition < IN_BOX_COUNT as u8 {
        return GetBoxMonData3(
            &raw mut (*gPokemonStoragePtr).boxes[boxId][boxPosition],
            request,
            dst as *mut u8,
        );
    } else {
        return 0;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetBoxMonAt(boxId: u8, boxPosition: u8, src: *mut BoxPokemon) {
    if boxId < TOTAL_BOXES_COUNT && boxPosition < IN_BOX_COUNT as u8 {
        (*gPokemonStoragePtr).boxes[boxId][boxPosition] = *src;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CopyBoxMonAt(boxId: u8, boxPosition: u8, dst: *mut BoxPokemon) {
    if boxId < TOTAL_BOXES_COUNT && boxPosition < IN_BOX_COUNT as u8 {
        *dst = (*gPokemonStoragePtr).boxes[boxId][boxPosition];
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CreateBoxMonAt(
    boxId: u8,
    boxPosition: u8,
    species: u16,
    level: u8,
    fixedIV: u8,
    hasFixedPersonality: u8,
    personality: u32,
    otIDType: u8,
    otID: u32,
) {
    if boxId < TOTAL_BOXES_COUNT && boxPosition < IN_BOX_COUNT as u8 {
        CreateBoxMon(
            &raw mut (*gPokemonStoragePtr).boxes[boxId][boxPosition],
            species,
            level,
            fixedIV,
            hasFixedPersonality,
            personality,
            otIDType,
            otID,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ZeroBoxMonAt(boxId: u8, boxPosition: u8) {
    if boxId < TOTAL_BOXES_COUNT && boxPosition < IN_BOX_COUNT as u8 {
        ZeroBoxMonData(&raw mut (*gPokemonStoragePtr).boxes[boxId][boxPosition]);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BoxMonAtToMon(boxId: u8, boxPosition: u8, dst: *mut Pokemon) {
    if boxId < TOTAL_BOXES_COUNT && boxPosition < IN_BOX_COUNT as u8 {
        BoxMonToMon(
            &raw mut (*gPokemonStoragePtr).boxes[boxId][boxPosition],
            dst,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetBoxedMonPtr(boxId: u8, boxPosition: u8) -> *mut BoxPokemon {
    if boxId < TOTAL_BOXES_COUNT && boxPosition < IN_BOX_COUNT as u8 {
        return &raw mut (*gPokemonStoragePtr).boxes[boxId][boxPosition];
    } else {
        return null_mut();
    }
    #[allow(unreachable_code)]
    {
        return null_mut();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetBoxNamePtr(boxId: u8) -> *mut u8 {
    if boxId < TOTAL_BOXES_COUNT {
        return (*gPokemonStoragePtr).boxNames[boxId].as_mut_ptr();
    } else {
        return null_mut();
    }
    #[allow(unreachable_code)]
    {
        return null_mut();
    }
}
pub(crate) unsafe extern "C" fn GetBoxWallpaper(boxId: u8) -> u8 {
    if boxId < TOTAL_BOXES_COUNT {
        return (*gPokemonStoragePtr).boxWallpapers[boxId];
    } else {
        return 0;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn SetBoxWallpaper(boxId: u8, wallpaperId: u8) {
    if boxId < TOTAL_BOXES_COUNT && wallpaperId < WALLPAPER_COUNT {
        (*gPokemonStoragePtr).boxWallpapers[boxId] = wallpaperId;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AdvanceStorageMonIndex(
    mut boxMons: *mut BoxPokemon,
    currIndex: u8,
    maxIndex: u8,
    mode: u8,
) -> i16 {
    let mut i: i16 = 0;
    let mut direction: i16 = -1;
    if mode == 0 || mode == 1 {
        direction = 1;
    }
    if mode == 1 || mode == 3 {
        i = currIndex as i8 as i16 + direction;
        while i >= 0 && i <= maxIndex as i16 {
            if GetBoxMonData2(boxMons.at(i), MON_DATA_SPECIES) != SPECIES_NONE as u32 {
                return i;
            }
            i += direction;
        }
    } else {
        i = currIndex as i8 as i16 + direction;
        while i >= 0 && i <= maxIndex as i16 {
            if GetBoxMonData2(boxMons.at(i), MON_DATA_SPECIES) != SPECIES_NONE as u32
                && GetBoxMonData2(boxMons.at(i), MON_DATA_IS_EGG) == 0
            {
                return i;
            }
            i += direction;
        }
    }
    return -1;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CheckFreePokemonStorageSpace() -> u8 {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    i = 0;
    while i < TOTAL_BOXES_COUNT as i32 {
        j = 0;
        while j < IN_BOX_COUNT {
            if GetBoxMonData2(
                &raw mut (*gPokemonStoragePtr).boxes[i][j],
                MON_DATA_SANITY_HAS_SPECIES,
            ) == 0
            {
                return TRUE;
            }
            j += 1;
        }
        i += 1;
    }
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CheckBoxMonSanityAt(boxId: u32, boxPosition: u32) -> u32 {
    if boxId < TOTAL_BOXES_COUNT as u32
        && boxPosition < IN_BOX_COUNT as u32
        && GetBoxMonData2(
            &raw mut (*gPokemonStoragePtr).boxes[boxId][boxPosition],
            MON_DATA_SANITY_HAS_SPECIES,
        ) != 0
        && GetBoxMonData2(
            &raw mut (*gPokemonStoragePtr).boxes[boxId][boxPosition],
            MON_DATA_SANITY_IS_EGG,
        ) == 0
        && GetBoxMonData2(
            &raw mut (*gPokemonStoragePtr).boxes[boxId][boxPosition],
            MON_DATA_SANITY_IS_BAD_EGG,
        ) == 0
    {
        return TRUE as u32;
    } else {
        return FALSE as u32;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CountStorageNonEggMons() -> u32 {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut count: u32 = 0;
    i = 0;
    while i < TOTAL_BOXES_COUNT as i32 {
        j = 0;
        while j < IN_BOX_COUNT {
            if GetBoxMonData2(
                &raw mut (*gPokemonStoragePtr).boxes[i][j],
                MON_DATA_SANITY_HAS_SPECIES,
            ) != 0
                && GetBoxMonData2(
                    &raw mut (*gPokemonStoragePtr).boxes[i][j],
                    MON_DATA_SANITY_IS_EGG,
                ) == 0
            {
                count += 1;
            }
            j += 1;
        }
        i += 1;
    }
    return count;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CountAllStorageMons() -> u32 {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut count: u32 = 0;
    i = 0;
    while i < TOTAL_BOXES_COUNT as i32 {
        j = 0;
        while j < IN_BOX_COUNT {
            if GetBoxMonData2(
                &raw mut (*gPokemonStoragePtr).boxes[i][j],
                MON_DATA_SANITY_HAS_SPECIES,
            ) != 0
                || GetBoxMonData2(
                    &raw mut (*gPokemonStoragePtr).boxes[i][j],
                    MON_DATA_SANITY_IS_EGG,
                ) != 0
            {
                count += 1;
            }
            j += 1;
        }
        i += 1;
    }
    return count;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnyStorageMonWithMove(r#move: u16) -> u32 {
    let mut moves: CArray<u16, 2> = zeroed();
    moves[0] = r#move;
    moves[1] = MOVES_COUNT;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    i = 0;
    while i < TOTAL_BOXES_COUNT as i32 {
        j = 0;
        while j < IN_BOX_COUNT {
            if GetBoxMonData2(
                &raw mut (*gPokemonStoragePtr).boxes[i][j],
                MON_DATA_SANITY_HAS_SPECIES,
            ) != 0
                && GetBoxMonData2(
                    &raw mut (*gPokemonStoragePtr).boxes[i][j],
                    MON_DATA_SANITY_IS_EGG,
                ) == 0
                && GetBoxMonData3(
                    &raw mut (*gPokemonStoragePtr).boxes[i][j],
                    MON_DATA_KNOWN_MOVES,
                    moves.as_mut_ptr() as *mut u8,
                ) != 0
            {
                return TRUE as u32;
            }
            j += 1;
        }
        i += 1;
    }
    return FALSE as u32;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ResetWaldaWallpaper() {
    (*gSaveBlock1Ptr).waldaPhrase.iconId = 0;
    (*gSaveBlock1Ptr).waldaPhrase.patternId = 0;
    (*gSaveBlock1Ptr).waldaPhrase.patternUnlocked = FALSE;
    (*gSaveBlock1Ptr).waldaPhrase.colors[0] = 31541;
    (*gSaveBlock1Ptr).waldaPhrase.colors[1] = 24966;
    (*gSaveBlock1Ptr).waldaPhrase.text[0] = EOS;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetWaldaWallpaperLockedOrUnlocked(unlocked: u32) {
    (*gSaveBlock1Ptr).waldaPhrase.patternUnlocked = unlocked as u8;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsWaldaWallpaperUnlocked() -> u32 {
    return (*gSaveBlock1Ptr).waldaPhrase.patternUnlocked as u32;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetWaldaWallpaperPatternId() -> u32 {
    return (*gSaveBlock1Ptr).waldaPhrase.patternId as u32;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetWaldaWallpaperPatternId(id: u8) {
    if id < 16 {
        (*gSaveBlock1Ptr).waldaPhrase.patternId = id;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetWaldaWallpaperIconId() -> u32 {
    return (*gSaveBlock1Ptr).waldaPhrase.iconId as u32;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetWaldaWallpaperIconId(id: u8) {
    if id < 30 {
        (*gSaveBlock1Ptr).waldaPhrase.iconId = id;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetWaldaWallpaperColorsPtr() -> *mut u16 {
    return (*gSaveBlock1Ptr).waldaPhrase.colors.as_mut_ptr();
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetWaldaWallpaperColors(color1: u16, color2: u16) {
    (*gSaveBlock1Ptr).waldaPhrase.colors[0] = color1;
    (*gSaveBlock1Ptr).waldaPhrase.colors[1] = color2;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetWaldaPhrasePtr() -> *mut u8 {
    return (*gSaveBlock1Ptr).waldaPhrase.text.as_mut_ptr();
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetWaldaPhrase(src: *mut u8) {
    StringCopy((*gSaveBlock1Ptr).waldaPhrase.text.as_mut_ptr(), src);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsWaldaPhraseEmpty() -> u32 {
    return ((*gSaveBlock1Ptr).waldaPhrase.text[0] == EOS) as u32;
}
pub(crate) unsafe extern "C" fn TilemapUtil_Init(count: u8) {
    let mut i: u16 = 0;
    sTilemapUtil = Alloc(48 * count as u32) as *mut TilemapUtil;
    sNumTilemapUtilIds = (if sTilemapUtil.is_null() {
        0
    } else {
        count as i32
    }) as u16;
    i = 0;
    while i < sNumTilemapUtilIds {
        (*sTilemapUtil.at(i)).savedTilemap = null_mut();
        (*sTilemapUtil.at(i)).active = FALSE;
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn TilemapUtil_Free() {
    Free(sTilemapUtil as *mut c_void);
}
pub(crate) unsafe extern "C" fn TilemapUtil_UpdateAll() {
    let mut i: i32 = 0;
    i = 0;
    while i < sNumTilemapUtilIds as i32 {
        if (*sTilemapUtil.at(i)).active == TRUE {
            TilemapUtil_Update(i as u8);
        }
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn TilemapUtil_SetMap(
    id: u8,
    bg: u8,
    tilemap: *mut c_void,
    width: u16,
    height: u16,
) {
    let mut bgScreenSize: u16 = 0;
    let mut bgType: u16 = 0;
    if id as u16 >= sNumTilemapUtilIds {
        return;
    }
    (*sTilemapUtil.at(id)).savedTilemap = null_mut();
    (*sTilemapUtil.at(id)).tilemap = tilemap;
    (*sTilemapUtil.at(id)).bg = bg;
    (*sTilemapUtil.at(id)).width = width;
    (*sTilemapUtil.at(id)).height = height;
    bgScreenSize = GetBgAttribute(bg, BG_ATTR_SCREENSIZE);
    bgType = GetBgAttribute(bg, BG_ATTR_TYPE);
    (*sTilemapUtil.at(id)).altWidth = sTilemapDimensions[bgType][bgScreenSize].width;
    (*sTilemapUtil.at(id)).altHeight = sTilemapDimensions[bgType][bgScreenSize].height;
    if bgType != BG_TYPE_NORMAL {
        (*sTilemapUtil.at(id)).tileSize = 1;
    } else {
        (*sTilemapUtil.at(id)).tileSize = 2;
    }
    (*sTilemapUtil.at(id)).rowSize = (*sTilemapUtil.at(id)).tileSize as u16 * width;
    (*sTilemapUtil.at(id)).cur.width = width;
    (*sTilemapUtil.at(id)).cur.height = height;
    (*sTilemapUtil.at(id)).cur.x = 0;
    (*sTilemapUtil.at(id)).cur.y = 0;
    (*sTilemapUtil.at(id)).cur.destX = 0;
    (*sTilemapUtil.at(id)).cur.destY = 0;
    (*sTilemapUtil.at(id)).prev = (*sTilemapUtil.at(id)).cur;
    (*sTilemapUtil.at(id)).active = TRUE;
}
pub(crate) unsafe extern "C" fn TilemapUtil_SetSavedMap(id: u8, tilemap: *mut c_void) {
    if id as u16 >= sNumTilemapUtilIds {
        return;
    }
    (*sTilemapUtil.at(id)).savedTilemap = tilemap;
    (*sTilemapUtil.at(id)).active = TRUE;
}
pub(crate) unsafe extern "C" fn TilemapUtil_SetPos(id: u8, x: u16, y: u16) {
    if id as u16 >= sNumTilemapUtilIds {
        return;
    }
    (*sTilemapUtil.at(id)).cur.destX = x as i16;
    (*sTilemapUtil.at(id)).cur.destY = y as i16;
    (*sTilemapUtil.at(id)).active = TRUE;
}
pub(crate) unsafe extern "C" fn TilemapUtil_SetRect(
    id: u8,
    x: u16,
    y: u16,
    width: u16,
    height: u16,
) {
    if id as u16 >= sNumTilemapUtilIds {
        return;
    }
    (*sTilemapUtil.at(id)).cur.x = x as i16;
    (*sTilemapUtil.at(id)).cur.y = y as i16;
    (*sTilemapUtil.at(id)).cur.width = width;
    (*sTilemapUtil.at(id)).cur.height = height;
    (*sTilemapUtil.at(id)).active = TRUE;
}
pub(crate) unsafe extern "C" fn TilemapUtil_Move(id: u8, mode: u8, val: i8) {
    if id as u16 >= sNumTilemapUtilIds {
        return;
    }
    match mode {
        0 => {
            (*sTilemapUtil.at(id)).cur.destX += val as i16;
            (*sTilemapUtil.at(id)).cur.width -= val as u16;
        }
        1 => {
            (*sTilemapUtil.at(id)).cur.x += val as i16;
            (*sTilemapUtil.at(id)).cur.width += val as u16;
        }
        2 => {
            (*sTilemapUtil.at(id)).cur.destY += val as i16;
            (*sTilemapUtil.at(id)).cur.height -= val as u16;
        }
        3 => {
            (*sTilemapUtil.at(id)).cur.y -= val as i16;
            (*sTilemapUtil.at(id)).cur.height += val as u16;
        }
        4 => {
            (*sTilemapUtil.at(id)).cur.destX += val as i16;
        }
        5 => {
            (*sTilemapUtil.at(id)).cur.destY += val as i16;
        }
        _ => {}
    }
    (*sTilemapUtil.at(id)).active = TRUE;
}
pub(crate) unsafe extern "C" fn TilemapUtil_Update(id: u8) {
    if id as u16 >= sNumTilemapUtilIds {
        return;
    }
    if !(*sTilemapUtil.at(id)).savedTilemap.is_null() {
        TilemapUtil_DrawPrev(id);
    }
    TilemapUtil_Draw(id);
    (*sTilemapUtil.at(id)).prev = (*sTilemapUtil.at(id)).cur;
}
pub(crate) unsafe extern "C" fn TilemapUtil_DrawPrev(id: u8) {
    let mut i: i32 = 0;
    let mut adder: u32 =
        (*sTilemapUtil.at(id)).tileSize as u32 * (*sTilemapUtil.at(id)).altWidth as u32;
    let mut tiles: *mut c_void = (((*sTilemapUtil.at(id)).savedTilemap as *mut u8)
        .at(adder * (*sTilemapUtil.at(id)).prev.destY as u32)
        as *mut c_void as *mut u8)
        .at((*sTilemapUtil.at(id)).tileSize as i32 * (*sTilemapUtil.at(id)).prev.destX as i32)
        as *mut c_void;
    i = 0;
    while i < (*sTilemapUtil.at(id)).prev.height as i32 {
        CopyToBgTilemapBufferRect(
            (*sTilemapUtil.at(id)).bg,
            tiles,
            (*sTilemapUtil.at(id)).prev.destX as u8,
            (*sTilemapUtil.at(id)).prev.destY as u8 + i as u8,
            (*sTilemapUtil.at(id)).prev.width as u8,
            1,
        );
        tiles = (tiles as *mut u8).at(adder) as *mut c_void;
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn TilemapUtil_Draw(id: u8) {
    let mut i: i32 = 0;
    let mut adder: u32 =
        (*sTilemapUtil.at(id)).tileSize as u32 * (*sTilemapUtil.at(id)).width as u32;
    let mut tiles: *mut c_void = (((*sTilemapUtil.at(id)).tilemap as *mut u8)
        .at(adder * (*sTilemapUtil.at(id)).cur.y as u32)
        as *mut c_void as *mut u8)
        .at((*sTilemapUtil.at(id)).tileSize as i32 * (*sTilemapUtil.at(id)).cur.x as i32)
        as *mut c_void;
    i = 0;
    while i < (*sTilemapUtil.at(id)).cur.height as i32 {
        CopyToBgTilemapBufferRect(
            (*sTilemapUtil.at(id)).bg,
            tiles,
            (*sTilemapUtil.at(id)).cur.destX as u8,
            (*sTilemapUtil.at(id)).cur.destY as u8 + i as u8,
            (*sTilemapUtil.at(id)).cur.width as u8,
            1,
        );
        tiles = (tiles as *mut u8).at(adder) as *mut c_void;
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn UnkUtil_Init(util: *mut UnkUtil, data: *mut UnkUtilData, max: u32) {
    sUnkUtil = util;
    (*util).data = data;
    (*util).max = max as u8;
    (*util).numActive = 0;
}
pub(crate) unsafe extern "C" fn UnkUtil_Run() {
    let mut i: u16 = 0;
    if (*sUnkUtil).numActive != 0 {
        i = 0;
        while i < (*sUnkUtil).numActive as u16 {
            let mut data: *mut UnkUtilData = (*sUnkUtil).data.at(i);
            (*data).func.unwrap_unchecked()(data);
            i += 1;
        }
        (*sUnkUtil).numActive = 0;
    }
}
pub(crate) unsafe extern "C" fn UnkUtil_CpuAdd(
    dest: *mut u8,
    dLeft: u16,
    dTop: u16,
    src: *mut u8,
    sLeft: u16,
    sTop: u16,
    width: u16,
    height: u16,
    unkArg: u16,
) -> u8 {
    let mut data: *mut UnkUtilData = null_mut();
    if (*sUnkUtil).numActive >= (*sUnkUtil).max {
        return FALSE;
    }
    data = (*sUnkUtil).data.at({
        let t1 = (*sUnkUtil).numActive;
        (*sUnkUtil).numActive += 1;
        t1
    });
    (*data).size = width * 2;
    (*data).dest = dest.at(2 * (dTop as i32 * 32 + dLeft as i32));
    (*data).src = src.at(2 * (sTop as i32 * unkArg as i32 + sLeft as i32));
    (*data).height = height;
    (*data).unk = unkArg;
    (*data).func = Some(UnkUtil_CpuRun);
    return TRUE;
}
pub(crate) unsafe extern "C" fn UnkUtil_CpuRun(data: *mut UnkUtilData) {
    let mut i: u16 = 0;
    i = 0;
    while i < (*data).height {
        CpuSet(
            (*data).src as *mut c_void,
            (*data).dest as *mut c_void,
            0x00000000 | ((*data).size as i32 / 2) as u32 & 0x1FFFFF,
        );
        (*data).dest = (*data).dest.at(64);
        (*data).src = (*data).src.at((*data).unk as i32 * 2);
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn UnkUtil_DmaAdd(
    dest: *mut c_void,
    dLeft: u16,
    dTop: u16,
    width: u16,
    height: u16,
) -> u8 {
    let mut data: *mut UnkUtilData = null_mut();
    if (*sUnkUtil).numActive >= (*sUnkUtil).max {
        return FALSE;
    }
    data = (*sUnkUtil).data.at({
        let t1 = (*sUnkUtil).numActive;
        (*sUnkUtil).numActive += 1;
        t1
    });
    (*data).size = width * 2;
    (*data).dest =
        (dest as *mut u8).at((dTop as i32 * 32 + dLeft as i32) * 2) as *mut c_void as *mut u8;
    (*data).height = height;
    (*data).func = Some(UnkUtil_DmaRun);
    return TRUE;
}
pub(crate) unsafe extern "C" fn UnkUtil_DmaRun(data: *mut UnkUtilData) {
    let mut i: u16 = 0;
    i = 0;
    while i < (*data).height {
        {
            let mut _dest: *mut c_void = (*data).dest as *mut c_void;
            let mut _size: u32 = (*data).size as u32;
            loop {
                if _size <= 0x1000 {
                    {
                        {
                            let mut tmp: u16 = 0;
                            volatile_write(&raw mut tmp, 0);
                            {
                                {
                                    let mut dmaRegs: *mut u32 = 67109076 as usize as *mut u32;
                                    volatile_write(dmaRegs, &raw mut tmp as usize as u32);
                                    volatile_write(dmaRegs.at(1), _dest as usize as u32);
                                    volatile_write(dmaRegs.at(2), 0x81000000 | _size / 2);
                                    let _ = (dmaRegs.at(2)).read_volatile();
                                }
                            }
                        }
                    }
                    break;
                }
                {
                    {
                        let mut tmp: u16 = 0;
                        volatile_write(&raw mut tmp, 0);
                        {
                            {
                                let mut dmaRegs: *mut u32 = 67109076 as usize as *mut u32;
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
            }
        }
        (*data).dest = (*data).dest.at(64);
        i += 1;
    }
}
