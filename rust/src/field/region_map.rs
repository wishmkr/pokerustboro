//! Translated from `src/region_map.c` by tools/rustport/c2rs.py.
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
    clippy::useless_transmute,
    dead_code,
    unused_assignments,
    unused_variables
)]

use crate::agb_main::SetVBlankCallback;
use crate::agb_main::gMain;
use crate::bg::{ResetBgsAndClearDma3BusyFlags, SetBgAttribute, ShowBg};
#[allow(unused_imports)]
use crate::c::*;
#[allow(unused_imports)]
use crate::consts::*;
use crate::event_data::{FlagGet, VarGet};
use crate::field_effect::ReturnToFieldFromFlyMapSelect;
use crate::field_specials::GetSSTidalLocation;
use crate::fieldmap::gMapHeader;
use crate::gpu_regs::{SetGpuReg, SetGpuRegBits};
use crate::international_string_util::GetStringRightAlignXOffset;
use crate::load_save::{gSaveBlock1Ptr, gSaveBlock2Ptr};
use crate::m4a::m4aSongNumStart;
use crate::menu::{
    ClearScheduledBgCopiesToVram, ClearStdWindowAndFrameToTransparent,
    DecompressAndCopyTileDataToVram, DoScheduledBgTilemapCopiesToVram,
    DrawStdFrameWithCustomTileAndPalette, FreeTempTileDataBuffersIfPossible,
    ScheduleBgCopyTilemapToVram,
};
use crate::overworld::{
    CB2_ReturnToFieldWithOpenMenu, GetMapTypeByGroupAndId, Overworld_GetMapHeaderByGroupAndId,
    SetWarpDestinationToHealLocation, SetWarpDestinationToMapWarp,
};
use crate::palette::{
    BeginNormalPaletteFade, BlendPalettes, LoadPalette, ResetPaletteFade, TransferPlttBuffer,
    UpdatePaletteFade,
};
use crate::palette::{gPlttBufferFaded, gPlttBufferUnfaded};
use crate::party_menu::CB2_ReturnToPartyMenuFromFlyMap;
use crate::secret_base::GetSecretBaseMapName;
use crate::sprite::gSprites;
use crate::sprite::{
    AnimateSprites, BuildOamBuffer, FreeAllSpritePalettes, FreeSpritePaletteByTag,
    FreeSpriteTileRanges, FreeSpriteTilesByTag, IndexOfSpritePaletteTag, LoadOam,
    ProcessSpriteCopyRequests, ResetSpriteData,
};
use crate::string_util::StringFill;
use crate::string_util::{StringCopy, StringLength};
use crate::text::DeactivateAllTextPrinters;
use crate::text_window::LoadUserWindowBorderGfx;
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
/// `CreateSprite` with this module's view of its types.
#[inline]
unsafe fn CreateSprite(a0: *mut SpriteTemplate, a1: i16, a2: i16, a3: u8) -> u8 {
    unsafe { crate::sprite::CreateSprite(a0 as _, a1, a2, a3) }
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
// The C's names for task and sprite data slots.
const sIconMapSec: usize = 0;
const sY: usize = 0;
const sFlickerTimer: usize = 1;
const sX: usize = 1;
const sVisible: usize = 2;
const sTimer: usize = 7;
// Data tables (translate with cdata.py): sRegionMapCursorPal sRegionMapCursorSmallGfxLZ sRegionMapCursorLargeGfxLZ sRegionMapBg_Pal sRegionMapBg_GfxLZ sRegionMapBg_TilemapLZ sRegionMapPlayerIcon_BrendanPal sRegionMapPlayerIcon_BrendanGfx sRegionMapPlayerIcon_MayPal sRegionMapPlayerIcon_MayGfx sRegionMap_MapSectionLayout sMapName_LITTLEROOT_TOWN sMapName_OLDALE_TOWN sMapName_DEWFORD_TOWN sMapName_LAVARIDGE_TOWN sMapName_FALLARBOR_TOWN sMapName_VERDANTURF_TOWN sMapName_PACIFIDLOG_TOWN sMapName_PETALBURG_CITY sMapName_SLATEPORT_CITY sMapName_MAUVILLE_CITY sMapName_RUSTBORO_CITY sMapName_FORTREE_CITY sMapName_LILYCOVE_CITY sMapName_MOSSDEEP_CITY sMapName_SOOTOPOLIS_CITY sMapName_EVER_GRANDE_CITY sMapName_ROUTE_101 sMapName_ROUTE_102 sMapName_ROUTE_103 sMapName_ROUTE_104 sMapName_ROUTE_105 sMapName_ROUTE_106 sMapName_ROUTE_107 sMapName_ROUTE_108 sMapName_ROUTE_109 sMapName_ROUTE_110 sMapName_ROUTE_111 sMapName_ROUTE_112 sMapName_ROUTE_113 sMapName_ROUTE_114 sMapName_ROUTE_115 sMapName_ROUTE_116 sMapName_ROUTE_117 sMapName_ROUTE_118 sMapName_ROUTE_119 sMapName_ROUTE_120 sMapName_ROUTE_121 sMapName_ROUTE_122 sMapName_ROUTE_123 sMapName_ROUTE_124 sMapName_ROUTE_125 sMapName_ROUTE_126 sMapName_ROUTE_127 sMapName_ROUTE_128 sMapName_ROUTE_129 sMapName_ROUTE_130 sMapName_ROUTE_131 sMapName_ROUTE_132 sMapName_ROUTE_133 sMapName_ROUTE_134 sMapName_UNDERWATER sMapName_GRANITE_CAVE sMapName_MT__CHIMNEY sMapName_SAFARI_ZONE sMapName_BATTLE_FRONTIER sMapName_PETALBURG_WOODS sMapName_RUSTURF_TUNNEL sMapName_ABANDONED_SHIP sMapName_NEW_MAUVILLE sMapName_METEOR_FALLS sMapName_MT__PYRE sMapName__AQUA__HIDEOUT_Clone sMapName_SHOAL_CAVE sMapName_SEAFLOOR_CAVERN sMapName_VICTORY_ROAD sMapName_MIRAGE_ISLAND sMapName_CAVE_OF_ORIGIN sMapName_SOUTHERN_ISLAND sMapName_FIERY_PATH sMapName_JAGGED_PASS sMapName_SEALED_CHAMBER sMapName_SCORCHED_SLAB sMapName_ISLAND_CAVE sMapName_DESERT_RUINS sMapName_ANCIENT_TOMB sMapName_INSIDE_OF_TRUCK sMapName_SKY_PILLAR sMapName_SECRET_BASE sMapName_ sMapName_PALLET_TOWN sMapName_VIRIDIAN_CITY sMapName_PEWTER_CITY sMapName_CERULEAN_CITY sMapName_LAVENDER_TOWN sMapName_VERMILION_CITY sMapName_CELADON_CITY sMapName_FUCHSIA_CITY sMapName_CINNABAR_ISLAND sMapName_INDIGO_PLATEAU sMapName_SAFFRON_CITY sMapName_ROUTE_4_Clone sMapName_ROUTE_10_Clone sMapName_ROUTE_1 sMapName_ROUTE_2 sMapName_ROUTE_3 sMapName_ROUTE_4 sMapName_ROUTE_5 sMapName_ROUTE_6 sMapName_ROUTE_7 sMapName_ROUTE_8 sMapName_ROUTE_9 sMapName_ROUTE_10 sMapName_ROUTE_11 sMapName_ROUTE_12 sMapName_ROUTE_13 sMapName_ROUTE_14 sMapName_ROUTE_15 sMapName_ROUTE_16 sMapName_ROUTE_17 sMapName_ROUTE_18 sMapName_ROUTE_19 sMapName_ROUTE_20 sMapName_ROUTE_21 sMapName_ROUTE_22 sMapName_ROUTE_23 sMapName_ROUTE_24 sMapName_ROUTE_25 sMapName_VIRIDIAN_FOREST sMapName_MT__MOON sMapName_S_S__ANNE sMapName_UNDERGROUND_PATH sMapName_UNDERGROUND_PATH_Clone sMapName_DIGLETT_S_CAVE sMapName_VICTORY_ROAD_Clone sMapName_ROCKET_HIDEOUT sMapName_SILPH_CO_ sMapName_POK__MON_MANSION sMapName_SAFARI_ZONE_Clone sMapName_POK__MON_LEAGUE sMapName_ROCK_TUNNEL sMapName_SEAFOAM_ISLANDS sMapName_POK__MON_TOWER sMapName_CERULEAN_CAVE sMapName_POWER_PLANT sMapName_ONE_ISLAND sMapName_TWO_ISLAND sMapName_THREE_ISLAND sMapName_FOUR_ISLAND sMapName_FIVE_ISLAND sMapName_SEVEN_ISLAND sMapName_SIX_ISLAND sMapName_KINDLE_ROAD sMapName_TREASURE_BEACH sMapName_CAPE_BRINK sMapName_BOND_BRIDGE sMapName_THREE_ISLE_PORT sMapName_SEVII_ISLE_6 sMapName_SEVII_ISLE_7 sMapName_SEVII_ISLE_8 sMapName_SEVII_ISLE_9 sMapName_RESORT_GORGEOUS sMapName_WATER_LABYRINTH sMapName_FIVE_ISLE_MEADOW sMapName_MEMORIAL_PILLAR sMapName_OUTCAST_ISLAND sMapName_GREEN_PATH sMapName_WATER_PATH sMapName_RUIN_VALLEY sMapName_TRAINER_TOWER sMapName_CANYON_ENTRANCE sMapName_SEVAULT_CANYON sMapName_TANOBY_RUINS sMapName_SEVII_ISLE_22 sMapName_SEVII_ISLE_23 sMapName_SEVII_ISLE_24 sMapName_NAVEL_ROCK sMapName_MT__EMBER sMapName_BERRY_FOREST sMapName_ICEFALL_CAVE sMapName_ROCKET_WAREHOUSE sMapName_TRAINER_TOWER_Clone sMapName_DOTTED_HOLE sMapName_LOST_CAVE sMapName_PATTERN_BUSH sMapName_ALTERING_CAVE sMapName_TANOBY_CHAMBERS sMapName_THREE_ISLE_PATH sMapName_TANOBY_KEY sMapName_BIRTH_ISLAND sMapName_MONEAN_CHAMBER sMapName_LIPTOO_CHAMBER sMapName_WEEPTH_CHAMBER sMapName_DILFORD_CHAMBER sMapName_SCUFIB_CHAMBER sMapName_RIXY_CHAMBER sMapName_VIAPOIS_CHAMBER sMapName_EMBER_SPA sMapName_SPECIAL_AREA sMapName_AQUA_HIDEOUT sMapName_MAGMA_HIDEOUT sMapName_MIRAGE_TOWER sMapName_FARAWAY_ISLAND sMapName_ARTISAN_CAVE sMapName_MARINE_CAVE sMapName_TERRA_CAVE sMapName_DESERT_UNDERPASS sMapName_TRAINER_HILL gRegionMapEntries sRegionMap_SpecialPlaceLocations sMarineCaveMapSecIds sTerraOrMarineCaveMapSecIds sMarineCaveLocationCoords sMapSecAquaHideoutOld sRegionMapCursorOam sRegionMapCursorAnim1 sRegionMapCursorAnim2 sRegionMapCursorAnimTable sRegionMapCursorSpritePalette sRegionMapCursorSpriteTemplate sRegionMapPlayerIconOam sRegionMapPlayerIconAnim1 sRegionMapPlayerIconAnimTable sMapSecIdsOffMap sRegionMapFramePal sRegionMapFrameGfxLZ sRegionMapFrameTilemapLZ sFlyTargetIcons_Pal sFlyTargetIcons_Gfx sMapHealLocations sEverGrandeCityNames sMultiNameFlyDestinations sFlyMapBgTemplates sFlyMapWindowTemplates sFlyTargetIconsSpritePalette sRedOutlineFlyDestinations sFlyDestIcon_OamData sFlyDestIcon_Anim_8x8CanFly sFlyDestIcon_Anim_16x8CanFly sFlyDestIcon_Anim_8x16CanFly sFlyDestIcon_Anim_8x8CantFly sFlyDestIcon_Anim_16x8CantFly sFlyDestIcon_Anim_8x16CantFly sFlyDestIcon_Anim_RedOutline sFlyDestIcon_Anims sFlyDestIconSpriteTemplate

/// `__typeof__(*((__typeof__(sFlyMap))0))`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct typeof___sFlyMap_0_t {
    pub callback: Option<unsafe fn()>,
    pub state: u16,
    pub mapSecId: u16,
    pub regionMap: RegionMap,
    pub tileBuffer: CArray<u8, 448>,
    pub nameBuffer: CArray<u8, 38>,
    pub choseFlyLocation: u8,
}

unsafe impl Sync for typeof___sFlyMap_0_t {}

/// `struct MultiNameFlyDest`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MultiNameFlyDest {
    pub name: *mut *mut u8,
    pub mapSecId: u16,
    pub flag: u16,
}

unsafe impl Sync for MultiNameFlyDest {}

#[cfg(target_arch = "arm")]
const _: () = {
    #[allow(unused_imports)]
    use core::mem::{offset_of, size_of};
    assert!(size_of::<typeof___sFlyMap_0_t>() == 2676);
    assert!(offset_of!(typeof___sFlyMap_0_t, callback) == 0);
    assert!(offset_of!(typeof___sFlyMap_0_t, state) == 4);
    assert!(offset_of!(typeof___sFlyMap_0_t, mapSecId) == 6);
    assert!(offset_of!(typeof___sFlyMap_0_t, regionMap) == 8);
    assert!(offset_of!(typeof___sFlyMap_0_t, tileBuffer) == 2188);
    assert!(offset_of!(typeof___sFlyMap_0_t, nameBuffer) == 2636);
    assert!(offset_of!(typeof___sFlyMap_0_t, choseFlyLocation) == 2674);
    assert!(size_of::<MultiNameFlyDest>() == 8);
    assert!(offset_of!(MultiNameFlyDest, name) == 0);
    assert!(offset_of!(MultiNameFlyDest, mapSecId) == 4);
    assert!(offset_of!(MultiNameFlyDest, flag) == 6);
};

const FLYDESTICON_RED_OUTLINE: u8 = 6;
const MAPCURSOR_X_MAX: u16 = 28;
const MAPCURSOR_X_MIN: u16 = 1;
const MAPCURSOR_Y_MAX: u16 = 16;
const MAPCURSOR_Y_MIN: u16 = 2;
const TAG_CURSOR: u16 = 0;
const TAG_FLY_ICON: u16 = 2;
const TAG_PLAYER_ICON: u16 = 1;
const WIN_FLY_TO_WHERE: u8 = 2;
const WIN_MAPSEC_NAME: u8 = 0;
const WIN_MAPSEC_NAME_TALL: u8 = 1;

static gRegionMapEntries: Table<CArray<RegionMapLocation, 213>> =
    Table((&raw const crate::data::region_map::gRegionMapEntries).cast());
static sFlyDestIconSpriteTemplate: Table<SpriteTemplate> =
    Table((&raw const crate::data::region_map::sFlyDestIconSpriteTemplate).cast());
static sFlyMapBgTemplates: Table<CArray<BgTemplate, 3>> =
    Table((&raw const crate::data::region_map::sFlyMapBgTemplates).cast());
static sFlyMapWindowTemplates: Table<CArray<WindowTemplate, 4>> =
    Table((&raw const crate::data::region_map::sFlyMapWindowTemplates).cast());
static sFlyTargetIconsSpritePalette: Table<SpritePalette> =
    Table((&raw const crate::data::region_map::sFlyTargetIconsSpritePalette).cast());
static sFlyTargetIcons_Gfx: Table<CArray<u32, 53>> =
    Table((&raw const crate::data::region_map::sFlyTargetIcons_Gfx).cast());
static sMapHealLocations: Table<CArray<CArray<u8, 3>, 50>> =
    Table((&raw const crate::data::region_map::sMapHealLocations).cast());
static sMapSecAquaHideoutOld: Table<CArray<u8, 1>> =
    Table((&raw const crate::data::region_map::sMapSecAquaHideoutOld).cast());
static sMapSecIdsOffMap: Table<CArray<u8, 3>> =
    Table((&raw const crate::data::region_map::sMapSecIdsOffMap).cast());
static sMarineCaveLocationCoords: Table<CArray<UCoords16, 8>> =
    Table((&raw const crate::data::region_map::sMarineCaveLocationCoords).cast());
static sMarineCaveMapSecIds: Table<CArray<u16, 3>> =
    Table((&raw const crate::data::region_map::sMarineCaveMapSecIds).cast());
static sMultiNameFlyDestinations: Table<CArray<MultiNameFlyDest, 1>> =
    Table((&raw const crate::data::region_map::sMultiNameFlyDestinations).cast());
static sRedOutlineFlyDestinations: Table<CArray<CArray<u16, 2>, 2>> =
    Table((&raw const crate::data::region_map::sRedOutlineFlyDestinations).cast());
static sRegionMapBg_GfxLZ: Table<CArray<u32, 857>> =
    Table((&raw const crate::data::region_map::sRegionMapBg_GfxLZ).cast());
static sRegionMapBg_Pal: Table<CArray<u16, 32>> =
    Table((&raw const crate::data::region_map::sRegionMapBg_Pal).cast());
static sRegionMapBg_TilemapLZ: Table<CArray<u32, 211>> =
    Table((&raw const crate::data::region_map::sRegionMapBg_TilemapLZ).cast());
static sRegionMapCursorLargeGfxLZ: Table<CArray<u32, 59>> =
    Table((&raw const crate::data::region_map::sRegionMapCursorLargeGfxLZ).cast());
static sRegionMapCursorSmallGfxLZ: Table<CArray<u32, 17>> =
    Table((&raw const crate::data::region_map::sRegionMapCursorSmallGfxLZ).cast());
static sRegionMapCursorSpritePalette: Table<SpritePalette> =
    Table((&raw const crate::data::region_map::sRegionMapCursorSpritePalette).cast());
static sRegionMapCursorSpriteTemplate: Table<SpriteTemplate> =
    Table((&raw const crate::data::region_map::sRegionMapCursorSpriteTemplate).cast());
static sRegionMapFrameGfxLZ: Table<CArray<u32, 14>> =
    Table((&raw const crate::data::region_map::sRegionMapFrameGfxLZ).cast());
static sRegionMapFramePal: Table<CArray<u16, 16>> =
    Table((&raw const crate::data::region_map::sRegionMapFramePal).cast());
static sRegionMapFrameTilemapLZ: Table<CArray<u32, 46>> =
    Table((&raw const crate::data::region_map::sRegionMapFrameTilemapLZ).cast());
static sRegionMapPlayerIconAnimTable: Table<CArray<*mut AnimCmd, 1>> =
    Table((&raw const crate::data::region_map::sRegionMapPlayerIconAnimTable).cast());
static sRegionMapPlayerIconOam: Table<OamData> =
    Table((&raw const crate::data::region_map::sRegionMapPlayerIconOam).cast());
static sRegionMapPlayerIcon_BrendanGfx: Table<CArray<u8, 128>> =
    Table((&raw const crate::data::region_map::sRegionMapPlayerIcon_BrendanGfx).cast());
static sRegionMapPlayerIcon_BrendanPal: Table<CArray<u16, 16>> =
    Table((&raw const crate::data::region_map::sRegionMapPlayerIcon_BrendanPal).cast());
static sRegionMapPlayerIcon_MayGfx: Table<CArray<u8, 128>> =
    Table((&raw const crate::data::region_map::sRegionMapPlayerIcon_MayGfx).cast());
static sRegionMapPlayerIcon_MayPal: Table<CArray<u16, 16>> =
    Table((&raw const crate::data::region_map::sRegionMapPlayerIcon_MayPal).cast());
static sRegionMap_MapSectionLayout: Table<CArray<CArray<u8, 28>, 15>> =
    Table((&raw const crate::data::region_map::sRegionMap_MapSectionLayout).cast());
static sRegionMap_SpecialPlaceLocations: Table<CArray<CArray<u16, 2>, 24>> =
    Table((&raw const crate::data::region_map::sRegionMap_SpecialPlaceLocations).cast());
static sTerraOrMarineCaveMapSecIds: Table<CArray<u16, 16>> =
    Table((&raw const crate::data::region_map::sTerraOrMarineCaveMapSecIds).cast());

#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub(crate) static sRegionMap: crate::global::Global<*mut RegionMap> =
    crate::global::Global::new(null_mut());
pub(crate) static sFlyMap: crate::global::Global<*mut typeof___sFlyMap_0_t> =
    crate::global::Global::new(null_mut());
pub(crate) static sDrawFlyDestTextWindow: crate::global::Global<u32> =
    crate::global::Global::new(0);

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
/// `CpuSet` with this module's view of its types.
#[inline]
fn CpuSet(a0: *mut c_void, a1: *mut c_void, a2: u32) {
    unsafe {
        crate::syscall::CpuSet(a0 as _, a1 as _, a2);
    }
}
/// `LZ77UnCompVram` with this module's view of its types.
#[inline]
fn LZ77UnCompVram(a0: *mut u32, a1: *mut c_void) {
    unsafe {
        crate::syscall::LZ77UnCompVram(a0 as _, a1 as _);
    }
}
/// `LZ77UnCompWram` with this module's view of its types.
#[inline]
fn LZ77UnCompWram(a0: *mut u32, a1: *mut c_void) {
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
pub unsafe fn InitRegionMap(regionMap: *mut RegionMap, zoomed: u8) {
    InitRegionMapData(regionMap, null_mut(), zoomed);
    while LoadRegionMapGfx() != 0 {}
}
pub unsafe fn InitRegionMapData(regionMap: *mut RegionMap, template: *mut BgTemplate, zoomed: u8) {
    crate::gl!(sRegionMap = regionMap);
    (*crate::gl!(sRegionMap)).initStep = 0;
    (*crate::gl!(sRegionMap)).zoomed = zoomed;
    (*crate::gl!(sRegionMap)).inputCallback = if zoomed == TRUE {
        Some(ProcessRegionMapInput_Zoomed)
    } else {
        Some(ProcessRegionMapInput_Full)
    };
    if !template.is_null() {
        (*crate::gl!(sRegionMap)).bgNum = (*template).bg() as u8;
        (*crate::gl!(sRegionMap)).charBaseIdx = (*template).charBaseIndex() as u8;
        (*crate::gl!(sRegionMap)).mapBaseIdx = (*template).mapBaseIndex() as u8;
        (*crate::gl!(sRegionMap)).bgManaged = TRUE;
    } else {
        (*crate::gl!(sRegionMap)).bgNum = 2;
        (*crate::gl!(sRegionMap)).charBaseIdx = 2;
        (*crate::gl!(sRegionMap)).mapBaseIdx = 28;
        (*crate::gl!(sRegionMap)).bgManaged = FALSE;
    }
}
pub unsafe fn ShowRegionMapForPokedexAreaScreen(regionMap: *mut RegionMap) {
    crate::gl!(sRegionMap = regionMap);
    InitMapBasedOnPlayerLocation();
    (*crate::gl!(sRegionMap)).playerIconSpritePosX = (*crate::gl!(sRegionMap)).cursorPosX;
    (*crate::gl!(sRegionMap)).playerIconSpritePosY = (*crate::gl!(sRegionMap)).cursorPosY;
}
pub unsafe fn LoadRegionMapGfx() -> u8 {
    match (*crate::gl!(sRegionMap)).initStep {
        0 => {
            if (*crate::gl!(sRegionMap)).bgManaged != 0 {
                DecompressAndCopyTileDataToVram(
                    (*crate::gl!(sRegionMap)).bgNum,
                    sRegionMapBg_GfxLZ.as_ptr().cast_mut() as *mut c_void,
                    0,
                    0,
                    0,
                );
            } else {
                LZ77UnCompVram(
                    sRegionMapBg_GfxLZ.as_ptr().cast_mut(),
                    0x6008000_usize as *mut u16 as *mut c_void,
                );
            }
        }
        1 => {
            if (*crate::gl!(sRegionMap)).bgManaged != 0 {
                if FreeTempTileDataBuffersIfPossible() == 0 {
                    DecompressAndCopyTileDataToVram(
                        (*crate::gl!(sRegionMap)).bgNum,
                        sRegionMapBg_TilemapLZ.as_ptr().cast_mut() as *mut c_void,
                        0,
                        0,
                        1,
                    );
                }
            } else {
                LZ77UnCompVram(
                    sRegionMapBg_TilemapLZ.as_ptr().cast_mut(),
                    0x600e000_usize as *mut u16 as *mut c_void,
                );
            }
        }
        2 => {
            if FreeTempTileDataBuffersIfPossible() == 0 {
                LoadPalette(sRegionMapBg_Pal.as_ptr().cast_mut() as *mut c_void, 112, 96);
            }
        }
        3 => {
            LZ77UnCompWram(
                sRegionMapCursorSmallGfxLZ.as_ptr().cast_mut(),
                (*crate::gl!(sRegionMap)).cursorSmallImage.as_mut_ptr() as *mut c_void,
            );
        }
        4 => {
            LZ77UnCompWram(
                sRegionMapCursorLargeGfxLZ.as_ptr().cast_mut(),
                (*crate::gl!(sRegionMap)).cursorLargeImage.as_mut_ptr() as *mut c_void,
            );
        }
        5 => {
            InitMapBasedOnPlayerLocation();
            (*crate::gl!(sRegionMap)).playerIconSpritePosX = (*crate::gl!(sRegionMap)).cursorPosX;
            (*crate::gl!(sRegionMap)).playerIconSpritePosY = (*crate::gl!(sRegionMap)).cursorPosY;
            (*crate::gl!(sRegionMap)).mapSecId =
                CorrectSpecialMapSecId_Internal((*crate::gl!(sRegionMap)).mapSecId);
            (*crate::gl!(sRegionMap)).mapSecType =
                GetMapsecType((*crate::gl!(sRegionMap)).mapSecId);
            GetMapName(
                (*crate::gl!(sRegionMap)).mapSecName.as_mut_ptr(),
                (*crate::gl!(sRegionMap)).mapSecId,
                MAP_NAME_LENGTH,
            );
        }
        6 => {
            if (*crate::gl!(sRegionMap)).zoomed == FALSE {
                CalcZoomScrollParams(0, 0, 0, 0, 0x100, 0x100, 0);
            } else {
                (*crate::gl!(sRegionMap)).scrollX =
                    (*crate::gl!(sRegionMap)).cursorPosX as i16 * 8 - 0x34;
                (*crate::gl!(sRegionMap)).scrollY =
                    (*crate::gl!(sRegionMap)).cursorPosY as i16 * 8 - 0x44;
                (*crate::gl!(sRegionMap)).zoomedCursorPosX = (*crate::gl!(sRegionMap)).cursorPosX;
                (*crate::gl!(sRegionMap)).zoomedCursorPosY = (*crate::gl!(sRegionMap)).cursorPosY;
                CalcZoomScrollParams(
                    (*crate::gl!(sRegionMap)).scrollX,
                    (*crate::gl!(sRegionMap)).scrollY,
                    0x38,
                    0x48,
                    0x80,
                    0x80,
                    0,
                );
            }
        }
        7 => {
            GetPositionOfCursorWithinMapSec();
            UpdateRegionMapVideoRegs();
            (*crate::gl!(sRegionMap)).cursorSprite = null_mut();
            (*crate::gl!(sRegionMap)).playerIconSprite = null_mut();
            (*crate::gl!(sRegionMap)).cursorMovementFrameCounter = 0;
            (*crate::gl!(sRegionMap)).blinkPlayerIcon = FALSE;
            if (*crate::gl!(sRegionMap)).bgManaged != 0 {
                SetBgAttribute((*crate::gl!(sRegionMap)).bgNum, BG_ATTR_SCREENSIZE, 2);
                SetBgAttribute(
                    (*crate::gl!(sRegionMap)).bgNum,
                    BG_ATTR_CHARBASEINDEX,
                    (*crate::gl!(sRegionMap)).charBaseIdx,
                );
                SetBgAttribute(
                    (*crate::gl!(sRegionMap)).bgNum,
                    BG_ATTR_MAPBASEINDEX,
                    (*crate::gl!(sRegionMap)).mapBaseIdx,
                );
                SetBgAttribute((*crate::gl!(sRegionMap)).bgNum, BG_ATTR_WRAPAROUND, 1);
                SetBgAttribute((*crate::gl!(sRegionMap)).bgNum, BG_ATTR_PALETTEMODE, 1);
            }
            (*crate::gl!(sRegionMap)).initStep += 1;
            return FALSE;
        }
        _ => {
            return FALSE;
        }
    }
    (*crate::gl!(sRegionMap)).initStep += 1;
    TRUE
}
pub unsafe fn BlendRegionMap(color: u16, coeff: u32) {
    BlendPalettes(0x380, coeff as u8, color);
    CpuSet(
        crate::gl!(&raw gPlttBufferFaded[112]) as *mut c_void,
        crate::gl!(&raw gPlttBufferUnfaded[112]) as *mut c_void,
        48,
    );
}
#[unsafe(no_mangle)]
pub unsafe fn FreeRegionMapIconResources() {
    if !(*crate::gl!(sRegionMap)).cursorSprite.is_null() {
        DestroySprite((*crate::gl!(sRegionMap)).cursorSprite);
        FreeSpriteTilesByTag((*crate::gl!(sRegionMap)).cursorTileTag);
        FreeSpritePaletteByTag((*crate::gl!(sRegionMap)).cursorPaletteTag);
    }
    if !(*crate::gl!(sRegionMap)).playerIconSprite.is_null() {
        DestroySprite((*crate::gl!(sRegionMap)).playerIconSprite);
        FreeSpriteTilesByTag((*crate::gl!(sRegionMap)).playerIconTileTag);
        FreeSpritePaletteByTag((*crate::gl!(sRegionMap)).playerIconPaletteTag);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn DoRegionMapInputCallback() -> u8 {
    (*crate::gl!(sRegionMap)).inputCallback.unwrap_unchecked()()
}
pub(crate) unsafe fn ProcessRegionMapInput_Full() -> u8 {
    let mut input: u8 = MAP_INPUT_NONE;
    (*crate::gl!(sRegionMap)).cursorDeltaX = 0;
    (*crate::gl!(sRegionMap)).cursorDeltaY = 0;
    if crate::gl!(gMain.heldKeys) as i32 & DPAD_UP != 0
        && (*crate::gl!(sRegionMap)).cursorPosY > MAPCURSOR_Y_MIN
    {
        (*crate::gl!(sRegionMap)).cursorDeltaY = -1;
        input = MAP_INPUT_MOVE_START;
    }
    if crate::gl!(gMain.heldKeys) as i32 & DPAD_DOWN != 0
        && (*crate::gl!(sRegionMap)).cursorPosY < MAPCURSOR_Y_MAX
    {
        (*crate::gl!(sRegionMap)).cursorDeltaY = 1;
        input = MAP_INPUT_MOVE_START;
    }
    if crate::gl!(gMain.heldKeys) as i32 & DPAD_LEFT != 0
        && (*crate::gl!(sRegionMap)).cursorPosX > MAPCURSOR_X_MIN
    {
        (*crate::gl!(sRegionMap)).cursorDeltaX = -1;
        input = MAP_INPUT_MOVE_START;
    }
    if crate::gl!(gMain.heldKeys) as i32 & DPAD_RIGHT != 0
        && (*crate::gl!(sRegionMap)).cursorPosX < MAPCURSOR_X_MAX
    {
        (*crate::gl!(sRegionMap)).cursorDeltaX = 1;
        input = MAP_INPUT_MOVE_START;
    }
    if crate::gl!(gMain.newKeys) as i32 & A_BUTTON != 0 {
        input = MAP_INPUT_A_BUTTON;
    } else if crate::gl!(gMain.newKeys) as i32 & B_BUTTON != 0 {
        input = MAP_INPUT_B_BUTTON;
    }
    if input == MAP_INPUT_MOVE_START {
        (*crate::gl!(sRegionMap)).cursorMovementFrameCounter = 4;
        (*crate::gl!(sRegionMap)).inputCallback = Some(MoveRegionMapCursor_Full);
    }
    input
}
pub(crate) unsafe fn MoveRegionMapCursor_Full() -> u8 {
    if (*crate::gl!(sRegionMap)).cursorMovementFrameCounter != 0 {
        return MAP_INPUT_MOVE_CONT;
    }
    if (*crate::gl!(sRegionMap)).cursorDeltaX > 0 {
        (*crate::gl!(sRegionMap)).cursorPosX += 1;
    }
    if (*crate::gl!(sRegionMap)).cursorDeltaX < 0 {
        (*crate::gl!(sRegionMap)).cursorPosX -= 1;
    }
    if (*crate::gl!(sRegionMap)).cursorDeltaY > 0 {
        (*crate::gl!(sRegionMap)).cursorPosY += 1;
    }
    if (*crate::gl!(sRegionMap)).cursorDeltaY < 0 {
        (*crate::gl!(sRegionMap)).cursorPosY -= 1;
    }
    let mapSecId: u16 = GetMapSecIdAt(
        (*crate::gl!(sRegionMap)).cursorPosX,
        (*crate::gl!(sRegionMap)).cursorPosY,
    );
    (*crate::gl!(sRegionMap)).mapSecType = GetMapsecType(mapSecId);
    if mapSecId != (*crate::gl!(sRegionMap)).mapSecId {
        (*crate::gl!(sRegionMap)).mapSecId = mapSecId;
        GetMapName(
            (*crate::gl!(sRegionMap)).mapSecName.as_mut_ptr(),
            (*crate::gl!(sRegionMap)).mapSecId,
            MAP_NAME_LENGTH,
        );
    }
    GetPositionOfCursorWithinMapSec();
    (*crate::gl!(sRegionMap)).inputCallback = Some(ProcessRegionMapInput_Full);
    MAP_INPUT_MOVE_END
}
pub(crate) unsafe fn ProcessRegionMapInput_Zoomed() -> u8 {
    let mut input: u8 = MAP_INPUT_NONE;
    (*crate::gl!(sRegionMap)).zoomedCursorDeltaX = 0;
    (*crate::gl!(sRegionMap)).zoomedCursorDeltaY = 0;
    if crate::gl!(gMain.heldKeys) as i32 & DPAD_UP != 0 && (*crate::gl!(sRegionMap)).scrollY > -52 {
        (*crate::gl!(sRegionMap)).zoomedCursorDeltaY = -1;
        input = MAP_INPUT_MOVE_START;
    }
    if crate::gl!(gMain.heldKeys) as i32 & DPAD_DOWN != 0
        && (*crate::gl!(sRegionMap)).scrollY < 0x3c
    {
        (*crate::gl!(sRegionMap)).zoomedCursorDeltaY = 1;
        input = MAP_INPUT_MOVE_START;
    }
    if crate::gl!(gMain.heldKeys) as i32 & DPAD_LEFT != 0 && (*crate::gl!(sRegionMap)).scrollX > -44
    {
        (*crate::gl!(sRegionMap)).zoomedCursorDeltaX = -1;
        input = MAP_INPUT_MOVE_START;
    }
    if crate::gl!(gMain.heldKeys) as i32 & DPAD_RIGHT != 0
        && (*crate::gl!(sRegionMap)).scrollX < 0xac
    {
        (*crate::gl!(sRegionMap)).zoomedCursorDeltaX = 1;
        input = MAP_INPUT_MOVE_START;
    }
    if crate::gl!(gMain.newKeys) as i32 & A_BUTTON != 0 {
        input = MAP_INPUT_A_BUTTON;
    }
    if crate::gl!(gMain.newKeys) as i32 & B_BUTTON != 0 {
        input = MAP_INPUT_B_BUTTON;
    }
    if input == MAP_INPUT_MOVE_START {
        (*crate::gl!(sRegionMap)).inputCallback = Some(MoveRegionMapCursor_Zoomed);
        (*crate::gl!(sRegionMap)).zoomedCursorMovementFrameCounter = 0;
    }
    input
}
pub(crate) unsafe fn MoveRegionMapCursor_Zoomed() -> u8 {
    let mut x: u16 = 0;
    let mut y: u16 = 0;
    let mut mapSecId: u16 = 0;
    (*crate::gl!(sRegionMap)).scrollY += (*crate::gl!(sRegionMap)).zoomedCursorDeltaY;
    (*crate::gl!(sRegionMap)).scrollX += (*crate::gl!(sRegionMap)).zoomedCursorDeltaX;
    RegionMap_SetBG2XAndBG2Y(
        (*crate::gl!(sRegionMap)).scrollX,
        (*crate::gl!(sRegionMap)).scrollY,
    );
    (*crate::gl!(sRegionMap)).zoomedCursorMovementFrameCounter += 1;
    if (*crate::gl!(sRegionMap)).zoomedCursorMovementFrameCounter == 8 {
        x = (((*crate::gl!(sRegionMap)).scrollX as i32 + 0x2c) / 8) as u16 + 1;
        y = (((*crate::gl!(sRegionMap)).scrollY as i32 + 0x34) / 8) as u16 + 2;
        if x != (*crate::gl!(sRegionMap)).zoomedCursorPosX
            || y != (*crate::gl!(sRegionMap)).zoomedCursorPosY
        {
            (*crate::gl!(sRegionMap)).zoomedCursorPosX = x;
            (*crate::gl!(sRegionMap)).zoomedCursorPosY = y;
            mapSecId = GetMapSecIdAt(x, y);
            (*crate::gl!(sRegionMap)).mapSecType = GetMapsecType(mapSecId);
            if mapSecId != (*crate::gl!(sRegionMap)).mapSecId {
                (*crate::gl!(sRegionMap)).mapSecId = mapSecId;
                GetMapName(
                    (*crate::gl!(sRegionMap)).mapSecName.as_mut_ptr(),
                    (*crate::gl!(sRegionMap)).mapSecId,
                    MAP_NAME_LENGTH,
                );
            }
            GetPositionOfCursorWithinMapSec();
        }
        (*crate::gl!(sRegionMap)).zoomedCursorMovementFrameCounter = 0;
        (*crate::gl!(sRegionMap)).inputCallback = Some(ProcessRegionMapInput_Zoomed);
        return MAP_INPUT_MOVE_END;
    }
    MAP_INPUT_MOVE_CONT
}
pub unsafe fn SetRegionMapDataForZoom() {
    if (*crate::gl!(sRegionMap)).zoomed == FALSE {
        (*crate::gl!(sRegionMap)).scrollY = 0;
        (*crate::gl!(sRegionMap)).scrollX = 0;
        (*crate::gl!(sRegionMap)).unk_040 = 0;
        (*crate::gl!(sRegionMap)).unk_03c = 0;
        (*crate::gl!(sRegionMap)).unk_060 = (*crate::gl!(sRegionMap)).cursorPosX as i16 * 8 - 0x34;
        (*crate::gl!(sRegionMap)).unk_062 = (*crate::gl!(sRegionMap)).cursorPosY as i16 * 8 - 0x44;
        (*crate::gl!(sRegionMap)).unk_044 = (((*crate::gl!(sRegionMap)).unk_060 as i32) << 8) / 16;
        (*crate::gl!(sRegionMap)).unk_048 = (((*crate::gl!(sRegionMap)).unk_062 as i32) << 8) / 16;
        (*crate::gl!(sRegionMap)).zoomedCursorPosX = (*crate::gl!(sRegionMap)).cursorPosX;
        (*crate::gl!(sRegionMap)).zoomedCursorPosY = (*crate::gl!(sRegionMap)).cursorPosY;
        (*crate::gl!(sRegionMap)).unk_04c = 0x10000;
        (*crate::gl!(sRegionMap)).unk_050 = -2048;
    } else {
        (*crate::gl!(sRegionMap)).unk_03c = (*crate::gl!(sRegionMap)).scrollX as i32 * 0x100;
        (*crate::gl!(sRegionMap)).unk_040 = (*crate::gl!(sRegionMap)).scrollY as i32 * 0x100;
        (*crate::gl!(sRegionMap)).unk_060 = 0;
        (*crate::gl!(sRegionMap)).unk_062 = 0;
        (*crate::gl!(sRegionMap)).unk_044 = -((*crate::gl!(sRegionMap)).unk_03c / 16);
        (*crate::gl!(sRegionMap)).unk_048 = -((*crate::gl!(sRegionMap)).unk_040 / 16);
        (*crate::gl!(sRegionMap)).cursorPosX = (*crate::gl!(sRegionMap)).zoomedCursorPosX;
        (*crate::gl!(sRegionMap)).cursorPosY = (*crate::gl!(sRegionMap)).zoomedCursorPosY;
        (*crate::gl!(sRegionMap)).unk_04c = 0x8000;
        (*crate::gl!(sRegionMap)).unk_050 = 0x800;
    }
    (*crate::gl!(sRegionMap)).unk_06e = 0;
    FreeRegionMapCursorSprite();
    HideRegionMapPlayerIcon();
}
pub unsafe fn UpdateRegionMapZoom() -> u8 {
    let mut retVal: u8 = 0;
    if (*crate::gl!(sRegionMap)).unk_06e >= 16 {
        return FALSE;
    }
    (*crate::gl!(sRegionMap)).unk_06e += 1;
    if (*crate::gl!(sRegionMap)).unk_06e == 16 {
        (*crate::gl!(sRegionMap)).unk_044 = 0;
        (*crate::gl!(sRegionMap)).unk_048 = 0;
        (*crate::gl!(sRegionMap)).scrollX = (*crate::gl!(sRegionMap)).unk_060;
        (*crate::gl!(sRegionMap)).scrollY = (*crate::gl!(sRegionMap)).unk_062;
        (*crate::gl!(sRegionMap)).unk_04c = if (*crate::gl!(sRegionMap)).zoomed == FALSE {
            32768
        } else {
            0x10000
        };
        (*crate::gl!(sRegionMap)).zoomed = ((*crate::gl!(sRegionMap)).zoomed == 0) as u8;
        (*crate::gl!(sRegionMap)).inputCallback = if (*crate::gl!(sRegionMap)).zoomed == FALSE {
            Some(ProcessRegionMapInput_Full)
        } else {
            Some(ProcessRegionMapInput_Zoomed)
        };
        CreateRegionMapCursor(
            (*crate::gl!(sRegionMap)).cursorTileTag,
            (*crate::gl!(sRegionMap)).cursorPaletteTag,
        );
        UnhideRegionMapPlayerIcon();
        retVal = FALSE;
    } else {
        (*crate::gl!(sRegionMap)).unk_03c += (*crate::gl!(sRegionMap)).unk_044;
        (*crate::gl!(sRegionMap)).unk_040 += (*crate::gl!(sRegionMap)).unk_048;
        (*crate::gl!(sRegionMap)).scrollX = ((*crate::gl!(sRegionMap)).unk_03c >> 8) as i16;
        (*crate::gl!(sRegionMap)).scrollY = ((*crate::gl!(sRegionMap)).unk_040 >> 8) as i16;
        (*crate::gl!(sRegionMap)).unk_04c += (*crate::gl!(sRegionMap)).unk_050;
        if (*crate::gl!(sRegionMap)).unk_044 < 0
            && (*crate::gl!(sRegionMap)).scrollX < (*crate::gl!(sRegionMap)).unk_060
            || (*crate::gl!(sRegionMap)).unk_044 > 0
                && (*crate::gl!(sRegionMap)).scrollX > (*crate::gl!(sRegionMap)).unk_060
        {
            (*crate::gl!(sRegionMap)).scrollX = (*crate::gl!(sRegionMap)).unk_060;
            (*crate::gl!(sRegionMap)).unk_044 = 0;
        }
        if (*crate::gl!(sRegionMap)).unk_048 < 0
            && (*crate::gl!(sRegionMap)).scrollY < (*crate::gl!(sRegionMap)).unk_062
            || (*crate::gl!(sRegionMap)).unk_048 > 0
                && (*crate::gl!(sRegionMap)).scrollY > (*crate::gl!(sRegionMap)).unk_062
        {
            (*crate::gl!(sRegionMap)).scrollY = (*crate::gl!(sRegionMap)).unk_062;
            (*crate::gl!(sRegionMap)).unk_048 = 0;
        }
        if (*crate::gl!(sRegionMap)).zoomed == FALSE {
            if (*crate::gl!(sRegionMap)).unk_04c < 32768 {
                (*crate::gl!(sRegionMap)).unk_04c = 32768;
                (*crate::gl!(sRegionMap)).unk_050 = 0;
            }
        } else {
            if (*crate::gl!(sRegionMap)).unk_04c > 0x10000 {
                (*crate::gl!(sRegionMap)).unk_04c = 0x10000;
                (*crate::gl!(sRegionMap)).unk_050 = 0;
            }
        }
        retVal = TRUE;
    }
    CalcZoomScrollParams(
        (*crate::gl!(sRegionMap)).scrollX,
        (*crate::gl!(sRegionMap)).scrollY,
        0x38,
        0x48,
        ((*crate::gl!(sRegionMap)).unk_04c >> 8) as u16,
        ((*crate::gl!(sRegionMap)).unk_04c >> 8) as u16,
        0,
    );
    retVal
}
unsafe fn CalcZoomScrollParams(
    scrollX: i16,
    scrollY: i16,
    c: i16,
    d: i16,
    e: u16,
    f: u16,
    rotation: u8,
) {
    (*crate::gl!(sRegionMap)).bg2pa = ((e as i32
        * (*(&raw const crate::trig::gSineTable).cast::<CArray<i16, 0>>())[rotation as i32 + 64]
            as i32)
        >> 8) as u32;
    (*crate::gl!(sRegionMap)).bg2pc = ((e as i32
        * -((*(&raw const crate::trig::gSineTable).cast::<CArray<i16, 0>>())[rotation] as i32))
        >> 8) as u32;
    (*crate::gl!(sRegionMap)).bg2pb = ((f as i32
        * (*(&raw const crate::trig::gSineTable).cast::<CArray<i16, 0>>())[rotation] as i32)
        >> 8) as u32;
    (*crate::gl!(sRegionMap)).bg2pd = ((f as i32
        * (*(&raw const crate::trig::gSineTable).cast::<CArray<i16, 0>>())[rotation as i32 + 64]
            as i32)
        >> 8) as u32;
    let var1: i32 = ((scrollX as i32) << 8) + ((c as i32) << 8);
    let var2: i32 = d as i32 * (*crate::gl!(sRegionMap)).bg2pb as i32
        + (*crate::gl!(sRegionMap)).bg2pa as i32 * c as i32;
    (*crate::gl!(sRegionMap)).bg2x = var1 - var2;
    let var3: i32 = ((scrollY as i32) << 8) + ((d as i32) << 8);
    let var4: i32 = (*crate::gl!(sRegionMap)).bg2pd as i32 * d as i32
        + (*crate::gl!(sRegionMap)).bg2pc as i32 * c as i32;
    (*crate::gl!(sRegionMap)).bg2y = var3 - var4;
    (*crate::gl!(sRegionMap)).needUpdateVideoRegs = TRUE;
}
unsafe fn RegionMap_SetBG2XAndBG2Y(x: i16, y: i16) {
    (*crate::gl!(sRegionMap)).bg2x = ((x as i32) << 8) + 0x1c00;
    (*crate::gl!(sRegionMap)).bg2y = ((y as i32) << 8) + 0x2400;
    (*crate::gl!(sRegionMap)).needUpdateVideoRegs = TRUE;
}
pub unsafe fn UpdateRegionMapVideoRegs() {
    if (*crate::gl!(sRegionMap)).needUpdateVideoRegs != 0 {
        SetGpuReg(REG_OFFSET_BG2PA, (*crate::gl!(sRegionMap)).bg2pa as u16);
        SetGpuReg(REG_OFFSET_BG2PB, (*crate::gl!(sRegionMap)).bg2pb as u16);
        SetGpuReg(REG_OFFSET_BG2PC, (*crate::gl!(sRegionMap)).bg2pc as u16);
        SetGpuReg(REG_OFFSET_BG2PD, (*crate::gl!(sRegionMap)).bg2pd as u16);
        SetGpuReg(REG_OFFSET_BG2X_L, (*crate::gl!(sRegionMap)).bg2x as u16);
        SetGpuReg(
            REG_OFFSET_BG2X_H,
            ((*crate::gl!(sRegionMap)).bg2x >> 16) as u16,
        );
        SetGpuReg(REG_OFFSET_BG2Y_L, (*crate::gl!(sRegionMap)).bg2y as u16);
        SetGpuReg(
            REG_OFFSET_BG2Y_H,
            ((*crate::gl!(sRegionMap)).bg2y >> 16) as u16,
        );
        (*crate::gl!(sRegionMap)).needUpdateVideoRegs = FALSE;
    }
}
pub unsafe fn PokedexAreaScreen_UpdateRegionMapVariablesAndVideoRegs(x: i16, y: i16) {
    CalcZoomScrollParams(x, y, 0x38, 0x48, 0x100, 0x100, 0);
    UpdateRegionMapVideoRegs();
    if !(*crate::gl!(sRegionMap)).playerIconSprite.is_null() {
        (*(*crate::gl!(sRegionMap)).playerIconSprite).x2 = -x;
        (*(*crate::gl!(sRegionMap)).playerIconSprite).y2 = -y;
    }
}
unsafe fn GetMapSecIdAt(mut x: u16, mut y: u16) -> u16 {
    if !(MAPCURSOR_Y_MIN..=MAPCURSOR_Y_MAX).contains(&y)
        || !(MAPCURSOR_X_MIN..=MAPCURSOR_X_MAX).contains(&x)
    {
        return MAPSEC_NONE;
    }
    y -= MAPCURSOR_Y_MIN;
    x -= MAPCURSOR_X_MIN;
    sRegionMap_MapSectionLayout[y][x] as u16
}
unsafe fn InitMapBasedOnPlayerLocation() {
    let mut mapHeader: *mut MapHeader = null_mut();
    let mut mapWidth: u16 = 0;
    let mut mapHeight: u16 = 0;
    let mut x: u16 = 0;
    let mut y: u16 = 0;
    let mut warp: *mut WarpData = null_mut();
    if (*crate::gl!(gSaveBlock1Ptr)).location.mapGroup == 25
        && ((*crate::gl!(gSaveBlock1Ptr)).location.mapNum == 41
            || (*crate::gl!(gSaveBlock1Ptr)).location.mapNum == 42
            || (*crate::gl!(gSaveBlock1Ptr)).location.mapNum == 43)
    {
        RegionMap_InitializeStateBasedOnSSTidalLocation();
        return;
    }
    match GetMapTypeByGroupAndId(
        (*crate::gl!(gSaveBlock1Ptr)).location.mapGroup,
        (*crate::gl!(gSaveBlock1Ptr)).location.mapNum,
    ) {
        MAP_TYPE_UNDERGROUND | MAP_TYPE_UNKNOWN => {
            if (*gMapHeader.as_ptr()).allowEscaping() != 0 {
                mapHeader = Overworld_GetMapHeaderByGroupAndId(
                    (*crate::gl!(gSaveBlock1Ptr)).escapeWarp.mapGroup as u16,
                    (*crate::gl!(gSaveBlock1Ptr)).escapeWarp.mapNum as u16,
                );
                (*crate::gl!(sRegionMap)).mapSecId = (*mapHeader).regionMapSectionId as u16;
                (*crate::gl!(sRegionMap)).playerIsInCave = TRUE;
                mapWidth = (*(*mapHeader).mapLayout).width as u16;
                mapHeight = (*(*mapHeader).mapLayout).height as u16;
                x = (*crate::gl!(gSaveBlock1Ptr)).escapeWarp.x as u16;
                y = (*crate::gl!(gSaveBlock1Ptr)).escapeWarp.y as u16;
            } else {
                (*crate::gl!(sRegionMap)).mapSecId =
                    crate::gl!(gMapHeader.regionMapSectionId) as u16;
                (*crate::gl!(sRegionMap)).playerIsInCave = TRUE;
                mapWidth = 1;
                mapHeight = 1;
                x = 1;
                y = 1;
            }
        }
        MAP_TYPE_SECRET_BASE => {
            mapHeader = Overworld_GetMapHeaderByGroupAndId(
                (*crate::gl!(gSaveBlock1Ptr)).dynamicWarp.mapGroup as u16,
                (*crate::gl!(gSaveBlock1Ptr)).dynamicWarp.mapNum as u16,
            );
            (*crate::gl!(sRegionMap)).mapSecId = (*mapHeader).regionMapSectionId as u16;
            (*crate::gl!(sRegionMap)).playerIsInCave = TRUE;
            mapWidth = (*(*mapHeader).mapLayout).width as u16;
            mapHeight = (*(*mapHeader).mapLayout).height as u16;
            x = (*crate::gl!(gSaveBlock1Ptr)).dynamicWarp.x as u16;
            y = (*crate::gl!(gSaveBlock1Ptr)).dynamicWarp.y as u16;
        }
        MAP_TYPE_INDOOR => {
            (*crate::gl!(sRegionMap)).mapSecId = crate::gl!(gMapHeader.regionMapSectionId) as u16;
            if (*crate::gl!(sRegionMap)).mapSecId != MAPSEC_DYNAMIC {
                warp = &raw mut (*crate::gl!(gSaveBlock1Ptr)).escapeWarp;
                mapHeader = Overworld_GetMapHeaderByGroupAndId(
                    (*warp).mapGroup as u16,
                    (*warp).mapNum as u16,
                );
            } else {
                warp = &raw mut (*crate::gl!(gSaveBlock1Ptr)).dynamicWarp;
                mapHeader = Overworld_GetMapHeaderByGroupAndId(
                    (*warp).mapGroup as u16,
                    (*warp).mapNum as u16,
                );
                (*crate::gl!(sRegionMap)).mapSecId = (*mapHeader).regionMapSectionId as u16;
            }
            if IsPlayerInAquaHideout((*crate::gl!(sRegionMap)).mapSecId as u8) != 0 {
                (*crate::gl!(sRegionMap)).playerIsInCave = TRUE;
            } else {
                (*crate::gl!(sRegionMap)).playerIsInCave = FALSE;
            }
            mapWidth = (*(*mapHeader).mapLayout).width as u16;
            mapHeight = (*(*mapHeader).mapLayout).height as u16;
            x = (*warp).x as u16;
            y = (*warp).y as u16;
        }
        _ => {
            (*crate::gl!(sRegionMap)).mapSecId = crate::gl!(gMapHeader.regionMapSectionId) as u16;
            (*crate::gl!(sRegionMap)).playerIsInCave = FALSE;
            mapWidth = (*crate::gl!(gMapHeader.mapLayout)).width as u16;
            mapHeight = (*crate::gl!(gMapHeader.mapLayout)).height as u16;
            x = (*crate::gl!(gSaveBlock1Ptr)).pos.x as u16;
            y = (*crate::gl!(gSaveBlock1Ptr)).pos.y as u16;
            if (*crate::gl!(sRegionMap)).mapSecId == MAPSEC_UNDERWATER_SEAFLOOR_CAVERN
                || (*crate::gl!(sRegionMap)).mapSecId == MAPSEC_UNDERWATER_MARINE_CAVE
            {
                (*crate::gl!(sRegionMap)).playerIsInCave = TRUE;
            }
        }
    }
    let xOnMap: u16 = x;
    let mut dimensionScale: u16 = div_i32(
        mapWidth as i32,
        gRegionMapEntries[(*crate::gl!(sRegionMap)).mapSecId].width as i32,
    ) as u16;
    if dimensionScale == 0 {
        dimensionScale = 1;
    }
    x = div_i32(x as i32, dimensionScale as i32) as u16;
    if x >= gRegionMapEntries[(*crate::gl!(sRegionMap)).mapSecId].width as u16 {
        x = gRegionMapEntries[(*crate::gl!(sRegionMap)).mapSecId].width as u16 - 1;
    }
    dimensionScale = div_i32(
        mapHeight as i32,
        gRegionMapEntries[(*crate::gl!(sRegionMap)).mapSecId].height as i32,
    ) as u16;
    if dimensionScale == 0 {
        dimensionScale = 1;
    }
    y = div_i32(y as i32, dimensionScale as i32) as u16;
    if y >= gRegionMapEntries[(*crate::gl!(sRegionMap)).mapSecId].height as u16 {
        y = gRegionMapEntries[(*crate::gl!(sRegionMap)).mapSecId].height as u16 - 1;
    }
    match (*crate::gl!(sRegionMap)).mapSecId {
        MAPSEC_ROUTE_114 => {
            if y != 0 {
                x = 0;
            }
        }
        MAPSEC_ROUTE_126 | MAPSEC_UNDERWATER_126 => {
            x = 0;
            if (*crate::gl!(gSaveBlock1Ptr)).pos.x > 32 {
                x += 1;
            }
            if (*crate::gl!(gSaveBlock1Ptr)).pos.x > 51 {
                x += 1;
            }
            y = 0;
            if (*crate::gl!(gSaveBlock1Ptr)).pos.y > 37 {
                y += 1;
            }
            if (*crate::gl!(gSaveBlock1Ptr)).pos.y > 56 {
                y += 1;
            }
        }
        MAPSEC_ROUTE_121 => {
            x = 0;
            if xOnMap > 14 {
                x += 1;
            }
            if xOnMap > 28 {
                x += 1;
            }
            if xOnMap > 54 {
                x += 1;
            }
        }
        MAPSEC_UNDERWATER_MARINE_CAVE => {
            GetMarineCaveCoords(
                &raw mut (*crate::gl!(sRegionMap)).cursorPosX,
                &raw mut (*crate::gl!(sRegionMap)).cursorPosY,
            );
            return;
        }
        _ => {}
    }
    (*crate::gl!(sRegionMap)).cursorPosX =
        gRegionMapEntries[(*crate::gl!(sRegionMap)).mapSecId].x as u16 + x + MAPCURSOR_X_MIN;
    (*crate::gl!(sRegionMap)).cursorPosY =
        gRegionMapEntries[(*crate::gl!(sRegionMap)).mapSecId].y as u16 + y + MAPCURSOR_Y_MIN;
}
unsafe fn RegionMap_InitializeStateBasedOnSSTidalLocation() {
    let mut mapGroup: u8 = 0;
    let mut mapNum: u8 = 0;
    let mut dimensionScale: u16 = 0;
    let mut xOnMap: i16 = 0;
    let mut yOnMap: i16 = 0;
    let mut mapHeader: *mut MapHeader = null_mut();
    let mut y: u16 = 0;
    let mut x: u16 = 0;
    match GetSSTidalLocation(
        &raw mut mapGroup as *mut i8,
        &raw mut mapNum as *mut i8,
        &raw mut xOnMap,
        &raw mut yOnMap,
    ) {
        SS_TIDAL_LOCATION_SLATEPORT => {
            (*crate::gl!(sRegionMap)).mapSecId = MAPSEC_SLATEPORT_CITY;
        }
        SS_TIDAL_LOCATION_LILYCOVE => {
            (*crate::gl!(sRegionMap)).mapSecId = MAPSEC_LILYCOVE_CITY;
        }
        SS_TIDAL_LOCATION_ROUTE124 => {
            (*crate::gl!(sRegionMap)).mapSecId = MAPSEC_ROUTE_124;
        }
        SS_TIDAL_LOCATION_ROUTE131 => {
            (*crate::gl!(sRegionMap)).mapSecId = MAPSEC_ROUTE_131;
        }
        _ => {
            mapHeader = Overworld_GetMapHeaderByGroupAndId(mapGroup as u16, mapNum as u16);
            (*crate::gl!(sRegionMap)).mapSecId = (*mapHeader).regionMapSectionId as u16;
            dimensionScale = div_i32(
                (*(*mapHeader).mapLayout).width,
                gRegionMapEntries[(*crate::gl!(sRegionMap)).mapSecId].width as i32,
            ) as u16;
            if dimensionScale == 0 {
                dimensionScale = 1;
            }
            x = div_i32(xOnMap as i32, dimensionScale as i32) as u16;
            if x >= gRegionMapEntries[(*crate::gl!(sRegionMap)).mapSecId].width as u16 {
                x = gRegionMapEntries[(*crate::gl!(sRegionMap)).mapSecId].width as u16 - 1;
            }
            dimensionScale = div_i32(
                (*(*mapHeader).mapLayout).height,
                gRegionMapEntries[(*crate::gl!(sRegionMap)).mapSecId].height as i32,
            ) as u16;
            if dimensionScale == 0 {
                dimensionScale = 1;
            }
            y = div_i32(yOnMap as i32, dimensionScale as i32) as u16;
            if y >= gRegionMapEntries[(*crate::gl!(sRegionMap)).mapSecId].height as u16 {
                y = gRegionMapEntries[(*crate::gl!(sRegionMap)).mapSecId].height as u16 - 1;
            }
        }
    }
    (*crate::gl!(sRegionMap)).playerIsInCave = FALSE;
    (*crate::gl!(sRegionMap)).cursorPosX =
        gRegionMapEntries[(*crate::gl!(sRegionMap)).mapSecId].x as u16 + x + MAPCURSOR_X_MIN;
    (*crate::gl!(sRegionMap)).cursorPosY =
        gRegionMapEntries[(*crate::gl!(sRegionMap)).mapSecId].y as u16 + y + MAPCURSOR_Y_MIN;
}
unsafe fn GetMapsecType(mapSecId: u16) -> u8 {
    match mapSecId {
        MAPSEC_NONE => {
            return MAPSECTYPE_NONE;
        }
        MAPSEC_LITTLEROOT_TOWN => {
            return (if FlagGet(FLAG_VISITED_LITTLEROOT_TOWN) != 0 {
                MAPSECTYPE_CITY_CANFLY
            } else {
                MAPSECTYPE_CITY_CANTFLY
            }) as u8;
        }
        MAPSEC_OLDALE_TOWN => {
            return (if FlagGet(FLAG_VISITED_OLDALE_TOWN) != 0 {
                MAPSECTYPE_CITY_CANFLY
            } else {
                MAPSECTYPE_CITY_CANTFLY
            }) as u8;
        }
        MAPSEC_DEWFORD_TOWN => {
            return (if FlagGet(FLAG_VISITED_DEWFORD_TOWN) != 0 {
                MAPSECTYPE_CITY_CANFLY
            } else {
                MAPSECTYPE_CITY_CANTFLY
            }) as u8;
        }
        MAPSEC_LAVARIDGE_TOWN => {
            return (if FlagGet(FLAG_VISITED_LAVARIDGE_TOWN) != 0 {
                MAPSECTYPE_CITY_CANFLY
            } else {
                MAPSECTYPE_CITY_CANTFLY
            }) as u8;
        }
        MAPSEC_FALLARBOR_TOWN => {
            return (if FlagGet(FLAG_VISITED_FALLARBOR_TOWN) != 0 {
                MAPSECTYPE_CITY_CANFLY
            } else {
                MAPSECTYPE_CITY_CANTFLY
            }) as u8;
        }
        MAPSEC_VERDANTURF_TOWN => {
            return (if FlagGet(FLAG_VISITED_VERDANTURF_TOWN) != 0 {
                MAPSECTYPE_CITY_CANFLY
            } else {
                MAPSECTYPE_CITY_CANTFLY
            }) as u8;
        }
        MAPSEC_PACIFIDLOG_TOWN => {
            return (if FlagGet(FLAG_VISITED_PACIFIDLOG_TOWN) != 0 {
                MAPSECTYPE_CITY_CANFLY
            } else {
                MAPSECTYPE_CITY_CANTFLY
            }) as u8;
        }
        MAPSEC_PETALBURG_CITY => {
            return (if FlagGet(FLAG_VISITED_PETALBURG_CITY) != 0 {
                MAPSECTYPE_CITY_CANFLY
            } else {
                MAPSECTYPE_CITY_CANTFLY
            }) as u8;
        }
        MAPSEC_SLATEPORT_CITY => {
            return (if FlagGet(FLAG_VISITED_SLATEPORT_CITY) != 0 {
                MAPSECTYPE_CITY_CANFLY
            } else {
                MAPSECTYPE_CITY_CANTFLY
            }) as u8;
        }
        MAPSEC_MAUVILLE_CITY => {
            return (if FlagGet(FLAG_VISITED_MAUVILLE_CITY) != 0 {
                MAPSECTYPE_CITY_CANFLY
            } else {
                MAPSECTYPE_CITY_CANTFLY
            }) as u8;
        }
        MAPSEC_RUSTBORO_CITY => {
            return (if FlagGet(FLAG_VISITED_RUSTBORO_CITY) != 0 {
                MAPSECTYPE_CITY_CANFLY
            } else {
                MAPSECTYPE_CITY_CANTFLY
            }) as u8;
        }
        MAPSEC_FORTREE_CITY => {
            return (if FlagGet(FLAG_VISITED_FORTREE_CITY) != 0 {
                MAPSECTYPE_CITY_CANFLY
            } else {
                MAPSECTYPE_CITY_CANTFLY
            }) as u8;
        }
        MAPSEC_LILYCOVE_CITY => {
            return (if FlagGet(FLAG_VISITED_LILYCOVE_CITY) != 0 {
                MAPSECTYPE_CITY_CANFLY
            } else {
                MAPSECTYPE_CITY_CANTFLY
            }) as u8;
        }
        MAPSEC_MOSSDEEP_CITY => {
            return (if FlagGet(FLAG_VISITED_MOSSDEEP_CITY) != 0 {
                MAPSECTYPE_CITY_CANFLY
            } else {
                MAPSECTYPE_CITY_CANTFLY
            }) as u8;
        }
        14 => {
            return (if FlagGet(FLAG_VISITED_SOOTOPOLIS_CITY) != 0 {
                MAPSECTYPE_CITY_CANFLY
            } else {
                MAPSECTYPE_CITY_CANTFLY
            }) as u8;
        }
        MAPSEC_EVER_GRANDE_CITY => {
            return (if FlagGet(FLAG_VISITED_EVER_GRANDE_CITY) != 0 {
                MAPSECTYPE_CITY_CANFLY
            } else {
                MAPSECTYPE_CITY_CANTFLY
            }) as u8;
        }
        58 => {
            return (if FlagGet(FLAG_LANDMARK_BATTLE_FRONTIER) != 0 {
                MAPSECTYPE_BATTLE_FRONTIER as i32
            } else {
                MAPSECTYPE_NONE as i32
            }) as u8;
        }
        MAPSEC_SOUTHERN_ISLAND => {
            return (if FlagGet(FLAG_LANDMARK_SOUTHERN_ISLAND) != 0 {
                MAPSECTYPE_ROUTE as i32
            } else {
                MAPSECTYPE_NONE as i32
            }) as u8;
        }
        _ => {
            return MAPSECTYPE_ROUTE;
        }
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
pub unsafe fn GetRegionMapSecIdAt(x: u16, y: u16) -> u16 {
    GetMapSecIdAt(x, y)
}
unsafe fn CorrectSpecialMapSecId_Internal(mapSecId: u16) -> u16 {
    for i in 0..3u32 {
        if sMarineCaveMapSecIds[i] == mapSecId {
            return GetTerraOrMarineCaveMapSecId();
        }
    }
    let mut i: u32 = 0;
    while sRegionMap_SpecialPlaceLocations[i][0] != MAPSEC_NONE {
        if sRegionMap_SpecialPlaceLocations[i][0] == mapSecId {
            return sRegionMap_SpecialPlaceLocations[i][1];
        }
        i += 1;
    }
    mapSecId
}
unsafe fn GetTerraOrMarineCaveMapSecId() -> u16 {
    let mut idx: i16 = VarGet(VAR_ABNORMAL_WEATHER_LOCATION) as i16 - 1;
    if !(0..=15).contains(&idx) {
        idx = 0;
    }
    sTerraOrMarineCaveMapSecIds[idx]
}
unsafe fn GetMarineCaveCoords(x: *mut u16, y: *mut u16) {
    let mut idx: u16 = VarGet(VAR_ABNORMAL_WEATHER_LOCATION);
    if !(MARINE_CAVE_LOCATIONS_START..=ABNORMAL_WEATHER_LOCATIONS).contains(&idx) {
        idx = MARINE_CAVE_LOCATIONS_START;
    }
    idx -= MARINE_CAVE_LOCATIONS_START;
    *x = sMarineCaveLocationCoords[idx].x + MAPCURSOR_X_MIN;
    *y = sMarineCaveLocationCoords[idx].y + MAPCURSOR_Y_MIN;
}
unsafe fn IsPlayerInAquaHideout(mapSecId: u8) -> u32 {
    for i in 0..1u32 {
        if sMapSecAquaHideoutOld[i] == mapSecId {
            return TRUE as u32;
        }
    }
    FALSE as u32
}
pub unsafe fn CorrectSpecialMapSecId(mapSecId: u16) -> u16 {
    CorrectSpecialMapSecId_Internal(mapSecId)
}
unsafe fn GetPositionOfCursorWithinMapSec() {
    let mut x: u16 = 0;
    let mut y: u16 = 0;
    let mut posWithinMapSec: u16 = 0;
    if (*crate::gl!(sRegionMap)).mapSecId == MAPSEC_NONE {
        (*crate::gl!(sRegionMap)).posWithinMapSec = 0;
        return;
    }
    if (*crate::gl!(sRegionMap)).zoomed == 0 {
        x = (*crate::gl!(sRegionMap)).cursorPosX;
        y = (*crate::gl!(sRegionMap)).cursorPosY;
    } else {
        x = (*crate::gl!(sRegionMap)).zoomedCursorPosX;
        y = (*crate::gl!(sRegionMap)).zoomedCursorPosY;
    }
    posWithinMapSec = 0;
    loop {
        if x <= MAPCURSOR_X_MIN {
            if RegionMap_IsMapSecIdInNextRow(y) != 0 {
                y -= 1;
                x = 29;
            } else {
                break;
            }
        } else {
            x -= 1;
            if GetMapSecIdAt(x, y) == (*crate::gl!(sRegionMap)).mapSecId {
                posWithinMapSec += 1;
            }
        }
    }
    (*crate::gl!(sRegionMap)).posWithinMapSec = posWithinMapSec as u8;
}
unsafe fn RegionMap_IsMapSecIdInNextRow(mut y: u16) -> u8 {
    if ({
        let t1 = y;
        y -= 1;
        t1
    }) == 0
    {
        return FALSE;
    }
    let mut x: u16 = MAPCURSOR_X_MIN;
    while x <= MAPCURSOR_X_MAX {
        if GetMapSecIdAt(x, y) == (*crate::gl!(sRegionMap)).mapSecId {
            return TRUE;
        }
        x += 1;
    }
    FALSE
}
pub(crate) unsafe fn SpriteCB_CursorMapFull(sprite: *mut Sprite) {
    if (*crate::gl!(sRegionMap)).cursorMovementFrameCounter != 0 {
        (*sprite).x += 2 * (*crate::gl!(sRegionMap)).cursorDeltaX as i16;
        (*sprite).y += 2 * (*crate::gl!(sRegionMap)).cursorDeltaY as i16;
        (*crate::gl!(sRegionMap)).cursorMovementFrameCounter -= 1;
    }
}
pub(crate) fn SpriteCB_CursorMapZoomed(sprite: *mut Sprite) {}
#[unsafe(no_mangle)]
pub unsafe fn CreateRegionMapCursor(tileTag: u16, paletteTag: u16) {
    let mut sheet: SpriteSheet = zeroed();
    let mut palette: SpritePalette = *sRegionMapCursorSpritePalette;
    let mut template: SpriteTemplate = *sRegionMapCursorSpriteTemplate;
    sheet.tag = tileTag;
    template.tileTag = tileTag;
    (*crate::gl!(sRegionMap)).cursorTileTag = tileTag;
    palette.tag = paletteTag;
    template.paletteTag = paletteTag;
    (*crate::gl!(sRegionMap)).cursorPaletteTag = paletteTag;
    if (*crate::gl!(sRegionMap)).zoomed == 0 {
        sheet.data = (*crate::gl!(sRegionMap)).cursorSmallImage.as_mut_ptr() as *mut c_void;
        sheet.size = 256;
        template.callback = Some(SpriteCB_CursorMapFull);
    } else {
        sheet.data = (*crate::gl!(sRegionMap)).cursorLargeImage.as_mut_ptr() as *mut c_void;
        sheet.size = 1536;
        template.callback = Some(SpriteCB_CursorMapZoomed);
    }
    LoadSpriteSheet(&raw mut sheet);
    LoadSpritePalette(&raw mut palette);
    let spriteId: u8 = CreateSprite(&raw mut template, 56, 72, 0);
    if spriteId != MAX_SPRITES {
        (*crate::gl!(sRegionMap)).cursorSprite = crate::gl!(&raw gSprites[spriteId]);
        if (*crate::gl!(sRegionMap)).zoomed == TRUE {
            (*(*crate::gl!(sRegionMap)).cursorSprite).oam.set_size(2);
            (*(*crate::gl!(sRegionMap)).cursorSprite).x -= 8;
            (*(*crate::gl!(sRegionMap)).cursorSprite).y -= 8;
            StartSpriteAnim((*crate::gl!(sRegionMap)).cursorSprite, 1);
        } else {
            (*(*crate::gl!(sRegionMap)).cursorSprite).oam.set_size(1);
            (*(*crate::gl!(sRegionMap)).cursorSprite).x =
                8 * (*crate::gl!(sRegionMap)).cursorPosX as i16 + 4;
            (*(*crate::gl!(sRegionMap)).cursorSprite).y =
                8 * (*crate::gl!(sRegionMap)).cursorPosY as i16 + 4;
        }
        (*(*crate::gl!(sRegionMap)).cursorSprite).data[1] = 2;
        (*(*crate::gl!(sRegionMap)).cursorSprite).data[2] =
            0x100 + IndexOfSpritePaletteTag(paletteTag) as i16 * 16 + 1;
        (*(*crate::gl!(sRegionMap)).cursorSprite).data[3] = TRUE as i16;
    }
}
unsafe fn FreeRegionMapCursorSprite() {
    if !(*crate::gl!(sRegionMap)).cursorSprite.is_null() {
        DestroySprite((*crate::gl!(sRegionMap)).cursorSprite);
        FreeSpriteTilesByTag((*crate::gl!(sRegionMap)).cursorTileTag);
        FreeSpritePaletteByTag((*crate::gl!(sRegionMap)).cursorPaletteTag);
    }
}
unsafe fn SetUnkCursorSpriteData() {
    (*(*crate::gl!(sRegionMap)).cursorSprite).data[3] = TRUE as i16;
}
unsafe fn ClearUnkCursorSpriteData() {
    (*(*crate::gl!(sRegionMap)).cursorSprite).data[3] = FALSE as i16;
}
#[unsafe(no_mangle)]
pub unsafe fn CreateRegionMapPlayerIcon(tileTag: u16, paletteTag: u16) {
    let mut sheet: SpriteSheet = zeroed();
    sheet.data = sRegionMapPlayerIcon_BrendanGfx.as_ptr().cast_mut() as *mut c_void;
    sheet.size = 0x80;
    sheet.tag = tileTag;
    let mut palette: SpritePalette = zeroed();
    palette.data = sRegionMapPlayerIcon_BrendanPal.as_ptr().cast_mut();
    palette.tag = paletteTag;
    let mut template: SpriteTemplate = zeroed();
    template.tileTag = tileTag;
    template.paletteTag = paletteTag;
    template.oam = (&raw const *sRegionMapPlayerIconOam).cast_mut();
    template.anims = sRegionMapPlayerIconAnimTable.as_ptr().cast_mut();
    template.images = null_mut();
    template.affineAnims = (*(&raw const crate::sprite::gDummySpriteAffineAnimTable)
        .cast::<CArray<*mut AffineAnimCmd, 0>>())
    .as_ptr()
    .cast_mut();
    template.callback = Some(SpriteCallbackDummy);
    if IsEventIslandMapSecId(crate::gl!(gMapHeader.regionMapSectionId)) != 0 {
        (*crate::gl!(sRegionMap)).playerIconSprite = null_mut();
        return;
    }
    if (*crate::gl!(gSaveBlock2Ptr)).playerGender == FEMALE {
        sheet.data = sRegionMapPlayerIcon_MayGfx.as_ptr().cast_mut() as *mut c_void;
        palette.data = sRegionMapPlayerIcon_MayPal.as_ptr().cast_mut();
    }
    LoadSpriteSheet(&raw mut sheet);
    LoadSpritePalette(&raw mut palette);
    let spriteId: u8 = CreateSprite(&raw mut template, 0, 0, 1);
    (*crate::gl!(sRegionMap)).playerIconSprite = crate::gl!(&raw gSprites[spriteId]);
    if (*crate::gl!(sRegionMap)).zoomed == 0 {
        (*(*crate::gl!(sRegionMap)).playerIconSprite).x =
            (*crate::gl!(sRegionMap)).playerIconSpritePosX as i16 * 8 + 4;
        (*(*crate::gl!(sRegionMap)).playerIconSprite).y =
            (*crate::gl!(sRegionMap)).playerIconSpritePosY as i16 * 8 + 4;
        (*(*crate::gl!(sRegionMap)).playerIconSprite).callback = Some(SpriteCB_PlayerIconMapFull);
    } else {
        (*(*crate::gl!(sRegionMap)).playerIconSprite).x =
            (*crate::gl!(sRegionMap)).playerIconSpritePosX as i16 * 16 - 0x30;
        (*(*crate::gl!(sRegionMap)).playerIconSprite).y =
            (*crate::gl!(sRegionMap)).playerIconSpritePosY as i16 * 16 - 0x42;
        (*(*crate::gl!(sRegionMap)).playerIconSprite).callback = Some(SpriteCB_PlayerIconMapZoomed);
    }
}
unsafe fn HideRegionMapPlayerIcon() {
    if !(*crate::gl!(sRegionMap)).playerIconSprite.is_null() {
        (*(*crate::gl!(sRegionMap)).playerIconSprite).set_invisible(TRUE as u16);
        (*(*crate::gl!(sRegionMap)).playerIconSprite).callback = Some(SpriteCallbackDummy);
    }
}
unsafe fn UnhideRegionMapPlayerIcon() {
    if !(*crate::gl!(sRegionMap)).playerIconSprite.is_null() {
        if (*crate::gl!(sRegionMap)).zoomed == TRUE {
            (*(*crate::gl!(sRegionMap)).playerIconSprite).x =
                (*crate::gl!(sRegionMap)).playerIconSpritePosX as i16 * 16 - 0x30;
            (*(*crate::gl!(sRegionMap)).playerIconSprite).y =
                (*crate::gl!(sRegionMap)).playerIconSpritePosY as i16 * 16 - 0x42;
            (*(*crate::gl!(sRegionMap)).playerIconSprite).callback =
                Some(SpriteCB_PlayerIconMapZoomed);
            (*(*crate::gl!(sRegionMap)).playerIconSprite).set_invisible(FALSE as u16);
        } else {
            (*(*crate::gl!(sRegionMap)).playerIconSprite).x =
                (*crate::gl!(sRegionMap)).playerIconSpritePosX as i16 * 8 + 4;
            (*(*crate::gl!(sRegionMap)).playerIconSprite).y =
                (*crate::gl!(sRegionMap)).playerIconSpritePosY as i16 * 8 + 4;
            (*(*crate::gl!(sRegionMap)).playerIconSprite).x2 = 0;
            (*(*crate::gl!(sRegionMap)).playerIconSprite).y2 = 0;
            (*(*crate::gl!(sRegionMap)).playerIconSprite).callback =
                Some(SpriteCB_PlayerIconMapFull);
            (*(*crate::gl!(sRegionMap)).playerIconSprite).set_invisible(FALSE as u16);
        }
    }
}
pub(crate) unsafe fn SpriteCB_PlayerIconMapZoomed(sprite: *mut Sprite) {
    (*sprite).x2 = -2 * (*crate::gl!(sRegionMap)).scrollX;
    (*sprite).y2 = -2 * (*crate::gl!(sRegionMap)).scrollY;
    (*sprite).data[sY] = (*sprite).y + (*sprite).y2 + (*sprite).centerToCornerVecY as i16;
    (*sprite).data[sX] = (*sprite).x + (*sprite).x2 + (*sprite).centerToCornerVecX as i16;
    if (*sprite).data[sY] < -8
        || (*sprite).data[sY] > 168
        || (*sprite).data[sX] < -8
        || (*sprite).data[sX] > 248
    {
        (*sprite).data[sVisible] = FALSE as i16;
    } else {
        (*sprite).data[sVisible] = TRUE as i16;
    }
    if (*sprite).data[sVisible] == TRUE as i16 {
        SpriteCB_PlayerIcon(sprite);
    } else {
        (*sprite).set_invisible(TRUE as u16);
    }
}
pub(crate) unsafe fn SpriteCB_PlayerIconMapFull(sprite: *mut Sprite) {
    SpriteCB_PlayerIcon(sprite);
}
unsafe fn SpriteCB_PlayerIcon(sprite: *mut Sprite) {
    if (*crate::gl!(sRegionMap)).blinkPlayerIcon != 0 {
        if ({
            (*sprite).data[sTimer] += 1;
            (*sprite).data[sTimer]
        }) > 16
        {
            (*sprite).data[sTimer] = 0;
            (*sprite).set_invisible(
                (if (*sprite).invisible() != 0 {
                    FALSE as i32
                } else {
                    TRUE as i32
                }) as u16,
            );
        }
    } else {
        (*sprite).set_invisible(FALSE as u16);
    }
}
pub unsafe fn TrySetPlayerIconBlink() {
    if (*crate::gl!(sRegionMap)).playerIsInCave != 0 {
        (*crate::gl!(sRegionMap)).blinkPlayerIcon = TRUE;
    }
}
#[unsafe(no_mangle)]
pub unsafe fn GetMapName(dest: *mut u8, regionMapId: u16, mut padLength: u16) -> *mut u8 {
    let mut str: *mut u8 = null_mut();
    if regionMapId == MAPSEC_SECRET_BASE as u16 {
        str = GetSecretBaseMapName(dest);
    } else if regionMapId < MAPSEC_NONE {
        str = StringCopy(dest, gRegionMapEntries[regionMapId].name);
    } else {
        if padLength == 0 {
            padLength = 18;
        }
        return StringFill(dest, CHAR_SPACE, padLength);
    }
    if padLength != 0 {
        for i in ((str as usize).wrapping_sub(dest as usize) as i32 as u16)..padLength {
            *({
                let t1 = str;
                str = str.at(1);
                t1
            }) = CHAR_SPACE;
        }
        *str = EOS;
    }
    str
}
pub unsafe fn GetMapNameGeneric(dest: *mut u8, mapSecId: u16) -> *mut u8 {
    match mapSecId {
        MAPSEC_DYNAMIC => {
            return StringCopy(
                dest,
                (*(&raw const crate::data::strings::gText_Ferry).cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut(),
            );
        }
        86 => {
            return StringCopy(
                dest,
                (*(&raw const crate::data::strings::gText_SecretBase).cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut(),
            );
        }
        _ => {
            return GetMapName(dest, mapSecId, 0);
        }
    }
    #[allow(unreachable_code)]
    {
        null_mut()
    }
}
pub unsafe fn GetMapNameHandleAquaHideout(dest: *mut u8, mapSecId: u16) -> *mut u8 {
    if mapSecId == MAPSEC_AQUA_HIDEOUT_OLD {
        return StringCopy(
            dest,
            (*(&raw const crate::data::strings::gText_Hideout).cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
        );
    } else {
        return GetMapNameGeneric(dest, mapSecId);
    }
    #[allow(unreachable_code)]
    {
        null_mut()
    }
}
unsafe fn GetMapSecDimensions(
    mapSecId: u16,
    x: *mut u16,
    y: *mut u16,
    width: *mut u16,
    height: *mut u16,
) {
    *x = gRegionMapEntries[mapSecId].x as u16;
    *y = gRegionMapEntries[mapSecId].y as u16;
    *width = gRegionMapEntries[mapSecId].width as u16;
    *height = gRegionMapEntries[mapSecId].height as u16;
}
pub unsafe fn IsRegionMapZoomed() -> u8 {
    (*crate::gl!(sRegionMap)).zoomed
}
pub unsafe fn IsEventIslandMapSecId(mapSecId: u8) -> u32 {
    for i in 0..3u32 {
        if mapSecId == sMapSecIdsOffMap[i] {
            return TRUE as u32;
        }
    }
    FALSE as u32
}
pub unsafe fn CB2_OpenFlyMap() {
    match crate::gl!(gMain.state) {
        0 => {
            SetVBlankCallback(None);
            SetGpuReg(0x0, 0);
            SetGpuReg(REG_OFFSET_BG0HOFS, 0);
            SetGpuReg(REG_OFFSET_BG0VOFS, 0);
            SetGpuReg(REG_OFFSET_BG1HOFS, 0);
            SetGpuReg(REG_OFFSET_BG1VOFS, 0);
            SetGpuReg(REG_OFFSET_BG2VOFS, 0);
            SetGpuReg(REG_OFFSET_BG2HOFS, 0);
            SetGpuReg(REG_OFFSET_BG3HOFS, 0);
            SetGpuReg(REG_OFFSET_BG3VOFS, 0);
            crate::gl!(sFlyMap = Alloc(2676) as *mut typeof___sFlyMap_0_t);
            if (*sFlyMap.as_ptr()).is_null() {
                SetMainCallback2(Some(CB2_ReturnToFieldWithOpenMenu));
            } else {
                ResetPaletteFade();
                ResetSpriteData();
                FreeSpriteTileRanges();
                FreeAllSpritePalettes();
                crate::gl!(gMain.state += 1);
            }
        }
        1 => {
            ResetBgsAndClearDma3BusyFlags(0);
            InitBgsFromTemplates(1, sFlyMapBgTemplates.as_ptr().cast_mut(), 3);
            crate::gl!(gMain.state += 1);
        }
        2 => {
            InitWindows(sFlyMapWindowTemplates.as_ptr().cast_mut());
            DeactivateAllTextPrinters();
            crate::gl!(gMain.state += 1);
        }
        3 => {
            LoadUserWindowBorderGfx(0, 0x65, 208);
            ClearScheduledBgCopiesToVram();
            crate::gl!(gMain.state += 1);
        }
        4 => {
            InitRegionMap(&raw mut (*crate::gl!(sFlyMap)).regionMap, FALSE);
            CreateRegionMapCursor(TAG_CURSOR, TAG_CURSOR);
            CreateRegionMapPlayerIcon(TAG_PLAYER_ICON, TAG_PLAYER_ICON);
            (*crate::gl!(sFlyMap)).mapSecId = (*crate::gl!(sFlyMap)).regionMap.mapSecId;
            StringFill(
                (*crate::gl!(sFlyMap)).nameBuffer.as_mut_ptr(),
                CHAR_SPACE,
                MAP_NAME_LENGTH,
            );
            sDrawFlyDestTextWindow.set(TRUE as u32);
            DrawFlyDestTextWindow();
            crate::gl!(gMain.state += 1);
        }
        5 => {
            LZ77UnCompVram(
                sRegionMapFrameGfxLZ.as_ptr().cast_mut(),
                0x600c000_usize as *mut u16 as *mut c_void,
            );
            crate::gl!(gMain.state += 1);
        }
        6 => {
            LZ77UnCompVram(
                sRegionMapFrameTilemapLZ.as_ptr().cast_mut(),
                0x600f000_usize as *mut u16 as *mut c_void,
            );
            crate::gl!(gMain.state += 1);
        }
        7 => {
            LoadPalette(
                sRegionMapFramePal.as_ptr().cast_mut() as *mut c_void,
                16,
                32,
            );
            PutWindowTilemap(WIN_FLY_TO_WHERE);
            FillWindowPixelBuffer(WIN_FLY_TO_WHERE, 0);
            AddTextPrinterParameterized(
                WIN_FLY_TO_WHERE,
                FONT_NORMAL,
                (*(&raw const crate::data::strings::gText_FlyToWhere).cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut(),
                0,
                1,
                0,
                None,
            );
            ScheduleBgCopyTilemapToVram(0);
            crate::gl!(gMain.state += 1);
        }
        8 => {
            LoadFlyDestIcons();
            crate::gl!(gMain.state += 1);
        }
        9 => {
            BlendPalettes(PALETTES_ALL, 16, 0);
            SetVBlankCallback(Some(VBlankCB_FlyMap));
            crate::gl!(gMain.state += 1);
        }
        10 => {
            SetGpuReg(REG_OFFSET_BLDCNT, 0);
            SetGpuRegBits(REG_OFFSET_DISPCNT, 4160);
            ShowBg(0);
            ShowBg(1);
            ShowBg(2);
            SetFlyMapCallback(Some(CB_FadeInFlyMap));
            SetMainCallback2(Some(CB2_FlyMap));
            crate::gl!(gMain.state += 1);
        }
        _ => {}
    }
}
pub(crate) unsafe fn VBlankCB_FlyMap() {
    LoadOam();
    ProcessSpriteCopyRequests();
    TransferPlttBuffer();
}
pub(crate) unsafe fn CB2_FlyMap() {
    (*crate::gl!(sFlyMap)).callback.unwrap_unchecked()();
    AnimateSprites();
    BuildOamBuffer();
    DoScheduledBgTilemapCopiesToVram();
}
unsafe fn SetFlyMapCallback(callback: Option<unsafe fn()>) {
    (*crate::gl!(sFlyMap)).callback = callback;
    (*crate::gl!(sFlyMap)).state = 0;
}
unsafe fn DrawFlyDestTextWindow() {
    let mut namePrinted: u32 = 0;
    let mut name: *mut u8 = null_mut();
    if (*crate::gl!(sFlyMap)).regionMap.mapSecType > MAPSECTYPE_NONE
        && (*crate::gl!(sFlyMap)).regionMap.mapSecType < NUM_MAPSEC_TYPES
    {
        namePrinted = FALSE as u32;
        for i in 0..1u16 {
            if (*crate::gl!(sFlyMap)).regionMap.mapSecId == sMultiNameFlyDestinations[i].mapSecId {
                if FlagGet(sMultiNameFlyDestinations[i].flag) != 0 {
                    StringLength(
                        *sMultiNameFlyDestinations[i]
                            .name
                            .at((*crate::gl!(sFlyMap)).regionMap.posWithinMapSec),
                    );
                    namePrinted = TRUE as u32;
                    ClearStdWindowAndFrameToTransparent(WIN_MAPSEC_NAME, FALSE);
                    DrawStdFrameWithCustomTileAndPalette(WIN_MAPSEC_NAME_TALL, FALSE, 101, 13);
                    AddTextPrinterParameterized(
                        WIN_MAPSEC_NAME_TALL,
                        FONT_NORMAL,
                        (*crate::gl!(sFlyMap)).regionMap.mapSecName.as_mut_ptr(),
                        0,
                        1,
                        0,
                        None,
                    );
                    name = *sMultiNameFlyDestinations[i]
                        .name
                        .at((*crate::gl!(sFlyMap)).regionMap.posWithinMapSec);
                    AddTextPrinterParameterized(
                        WIN_MAPSEC_NAME_TALL,
                        FONT_NORMAL,
                        name,
                        GetStringRightAlignXOffset(FONT_NORMAL as i32, name, 96) as u8,
                        17,
                        0,
                        None,
                    );
                    ScheduleBgCopyTilemapToVram(0);
                    sDrawFlyDestTextWindow.set(TRUE as u32);
                }
                break;
            }
        }
        if namePrinted == 0 {
            if sDrawFlyDestTextWindow.get() == TRUE as u32 {
                ClearStdWindowAndFrameToTransparent(WIN_MAPSEC_NAME_TALL, FALSE);
                DrawStdFrameWithCustomTileAndPalette(WIN_MAPSEC_NAME, FALSE, 101, 13);
            } else {
                FillWindowPixelBuffer(WIN_MAPSEC_NAME, 17);
            }
            AddTextPrinterParameterized(
                WIN_MAPSEC_NAME,
                FONT_NORMAL,
                (*crate::gl!(sFlyMap)).regionMap.mapSecName.as_mut_ptr(),
                0,
                1,
                0,
                None,
            );
            ScheduleBgCopyTilemapToVram(0);
            sDrawFlyDestTextWindow.set(FALSE as u32);
        }
    } else {
        if sDrawFlyDestTextWindow.get() == TRUE as u32 {
            ClearStdWindowAndFrameToTransparent(WIN_MAPSEC_NAME_TALL, FALSE);
            DrawStdFrameWithCustomTileAndPalette(WIN_MAPSEC_NAME, FALSE, 101, 13);
        }
        FillWindowPixelBuffer(WIN_MAPSEC_NAME, 17);
        CopyWindowToVram(WIN_MAPSEC_NAME, COPYWIN_GFX);
        ScheduleBgCopyTilemapToVram(0);
        sDrawFlyDestTextWindow.set(FALSE as u32);
    }
}
unsafe fn LoadFlyDestIcons() {
    let mut sheet: SpriteSheet = zeroed();
    LZ77UnCompWram(
        sFlyTargetIcons_Gfx.as_ptr().cast_mut(),
        (*crate::gl!(sFlyMap)).tileBuffer.as_mut_ptr() as *mut c_void,
    );
    sheet.data = (*crate::gl!(sFlyMap)).tileBuffer.as_mut_ptr() as *mut c_void;
    sheet.size = 448;
    sheet.tag = TAG_FLY_ICON;
    LoadSpriteSheet(&raw mut sheet);
    LoadSpritePalette((&raw const *sFlyTargetIconsSpritePalette).cast_mut());
    CreateFlyDestIcons();
    TryCreateRedOutlineFlyDestIcons();
}
unsafe fn CreateFlyDestIcons() {
    let mut x: u16 = 0;
    let mut y: u16 = 0;
    let mut width: u16 = 0;
    let mut height: u16 = 0;
    let mut shape: u16 = 0;
    let mut spriteId: u8 = 0;
    let mut canFlyFlag: u16 = FLAG_VISITED_LITTLEROOT_TOWN;
    let mut mapSecId: u16 = MAPSEC_LITTLEROOT_TOWN;
    while mapSecId <= MAPSEC_EVER_GRANDE_CITY {
        GetMapSecDimensions(
            mapSecId,
            &raw mut x,
            &raw mut y,
            &raw mut width,
            &raw mut height,
        );
        x = (x + MAPCURSOR_X_MIN) * 8 + 4;
        y = (y + MAPCURSOR_Y_MIN) * 8 + 4;
        if width == 2 {
            shape = 1;
        } else if height == 2 {
            shape = 2;
        } else {
            shape = 0;
        }
        spriteId = CreateSprite(
            (&raw const *sFlyDestIconSpriteTemplate).cast_mut(),
            x as i16,
            y as i16,
            10,
        );
        if spriteId != MAX_SPRITES {
            (*gSprites.as_ptr())[spriteId].oam.set_shape(shape as u32);
            if FlagGet(canFlyFlag) != 0 {
                crate::gl!(gSprites[spriteId].callback = Some(SpriteCB_FlyDestIcon));
            } else {
                shape += 3;
            }
            StartSpriteAnim(crate::gl!(&raw gSprites[spriteId]), shape as u8);
            {
                let gl_i0 = sIconMapSec;
                crate::gl!(gSprites[spriteId].data[gl_i0] = mapSecId as i16)
            };
        }
        canFlyFlag += 1;
        mapSecId += 1;
    }
}
unsafe fn TryCreateRedOutlineFlyDestIcons() {
    let mut x: u16 = 0;
    let mut y: u16 = 0;
    let mut width: u16 = 0;
    let mut height: u16 = 0;
    let mut mapSecId: u16 = 0;
    let mut spriteId: u8 = 0;
    let mut i: u16 = 0;
    while sRedOutlineFlyDestinations[i][1] != MAPSEC_NONE {
        if FlagGet(sRedOutlineFlyDestinations[i][0]) != 0 {
            mapSecId = sRedOutlineFlyDestinations[i][1];
            GetMapSecDimensions(
                mapSecId,
                &raw mut x,
                &raw mut y,
                &raw mut width,
                &raw mut height,
            );
            x = (x + MAPCURSOR_X_MIN) * 8;
            y = (y + MAPCURSOR_Y_MIN) * 8;
            spriteId = CreateSprite(
                (&raw const *sFlyDestIconSpriteTemplate).cast_mut(),
                x as i16,
                y as i16,
                10,
            );
            if spriteId != MAX_SPRITES {
                (*gSprites.as_ptr())[spriteId].oam.set_size(1);
                crate::gl!(gSprites[spriteId].callback = Some(SpriteCB_FlyDestIcon));
                StartSpriteAnim(crate::gl!(&raw gSprites[spriteId]), FLYDESTICON_RED_OUTLINE);
                {
                    let gl_i1 = sIconMapSec;
                    crate::gl!(gSprites[spriteId].data[gl_i1] = mapSecId as i16)
                };
            }
        }
        i += 1;
    }
}
pub(crate) unsafe fn SpriteCB_FlyDestIcon(sprite: *mut Sprite) {
    if (*crate::gl!(sFlyMap)).regionMap.mapSecId as i32 == (*sprite).data[sIconMapSec] as i32 {
        if ({
            (*sprite).data[sFlickerTimer] += 1;
            (*sprite).data[sFlickerTimer]
        }) > 16
        {
            (*sprite).data[sFlickerTimer] = 0;
            (*sprite).set_invisible(
                (if (*sprite).invisible() != 0 {
                    FALSE as i32
                } else {
                    TRUE as i32
                }) as u16,
            );
        }
    } else {
        (*sprite).data[sFlickerTimer] = 16;
        (*sprite).set_invisible(FALSE as u16);
    }
}
pub(crate) unsafe fn CB_FadeInFlyMap() {
    match (*crate::gl!(sFlyMap)).state {
        0 => {
            BeginNormalPaletteFade(PALETTES_ALL, 0, 16, 0, 0);
            (*crate::gl!(sFlyMap)).state += 1;
        }
        1 if UpdatePaletteFade() == 0 => {
            SetFlyMapCallback(Some(CB_HandleFlyMapInput));
        }
        _ => {}
    }
}
pub(crate) unsafe fn CB_HandleFlyMapInput() {
    if (*crate::gl!(sFlyMap)).state == 0 {
        match DoRegionMapInputCallback() {
            MAP_INPUT_NONE | MAP_INPUT_MOVE_START | MAP_INPUT_MOVE_CONT => {}
            MAP_INPUT_MOVE_END => {
                DrawFlyDestTextWindow();
            }
            MAP_INPUT_A_BUTTON => {
                if (*crate::gl!(sFlyMap)).regionMap.mapSecType == MAPSECTYPE_CITY_CANFLY as u8
                    || (*crate::gl!(sFlyMap)).regionMap.mapSecType == MAPSECTYPE_BATTLE_FRONTIER
                {
                    m4aSongNumStart(SE_SELECT);
                    (*crate::gl!(sFlyMap)).choseFlyLocation = TRUE;
                    SetFlyMapCallback(Some(CB_ExitFlyMap));
                }
            }
            MAP_INPUT_B_BUTTON => {
                m4aSongNumStart(SE_SELECT);
                (*crate::gl!(sFlyMap)).choseFlyLocation = FALSE;
                SetFlyMapCallback(Some(CB_ExitFlyMap));
            }
            _ => {}
        }
    }
}
pub(crate) unsafe fn CB_ExitFlyMap() {
    match (*crate::gl!(sFlyMap)).state {
        0 => {
            BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 16, 0);
            (*crate::gl!(sFlyMap)).state += 1;
        }
        1 if UpdatePaletteFade() == 0 => {
            FreeRegionMapIconResources();
            if (*crate::gl!(sFlyMap)).choseFlyLocation != 0 {
                match (*crate::gl!(sFlyMap)).regionMap.mapSecId {
                    MAPSEC_SOUTHERN_ISLAND => {
                        SetWarpDestinationToHealLocation(HEAL_LOCATION_SOUTHERN_ISLAND_EXTERIOR);
                    }
                    58 => {
                        SetWarpDestinationToHealLocation(
                            HEAL_LOCATION_BATTLE_FRONTIER_OUTSIDE_EAST,
                        );
                    }
                    MAPSEC_LITTLEROOT_TOWN => {
                        SetWarpDestinationToHealLocation(
                            (if (*crate::gl!(gSaveBlock2Ptr)).playerGender == MALE {
                                HEAL_LOCATION_LITTLEROOT_TOWN_BRENDANS_HOUSE
                            } else {
                                HEAL_LOCATION_LITTLEROOT_TOWN_MAYS_HOUSE
                            }) as u8,
                        );
                    }
                    MAPSEC_EVER_GRANDE_CITY => {
                        SetWarpDestinationToHealLocation(
                            (if FlagGet(FLAG_LANDMARK_POKEMON_LEAGUE) != 0
                                && (*crate::gl!(sFlyMap)).regionMap.posWithinMapSec == 0
                            {
                                HEAL_LOCATION_EVER_GRANDE_CITY_POKEMON_LEAGUE
                            } else {
                                HEAL_LOCATION_EVER_GRANDE_CITY
                            }) as u8,
                        );
                    }
                    _ => {
                        if sMapHealLocations[(*crate::gl!(sFlyMap)).regionMap.mapSecId][2]
                            != HEAL_LOCATION_NONE
                        {
                            SetWarpDestinationToHealLocation(
                                sMapHealLocations[(*crate::gl!(sFlyMap)).regionMap.mapSecId][2],
                            );
                        } else {
                            SetWarpDestinationToMapWarp(
                                sMapHealLocations[(*crate::gl!(sFlyMap)).regionMap.mapSecId][0]
                                    as i8,
                                sMapHealLocations[(*crate::gl!(sFlyMap)).regionMap.mapSecId][1]
                                    as i8,
                                WARP_ID_NONE,
                            );
                        }
                    }
                }
                ReturnToFieldFromFlyMapSelect();
            } else {
                SetMainCallback2(Some(CB2_ReturnToPartyMenuFromFlyMap));
            }
            if !(*sFlyMap.as_ptr()).is_null() {
                Free(crate::gl!(sFlyMap) as *mut c_void);
                crate::gl!(sFlyMap = null_mut());
            }
            FreeAllWindowBuffers();
        }
        _ => {}
    }
}
