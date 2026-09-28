//! Translated from `src/region_map.c` by tools/rustport/c2rs.py.
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
// Data tables (translate with cdata.py): sRegionMapCursorPal sRegionMapCursorSmallGfxLZ sRegionMapCursorLargeGfxLZ sRegionMapBg_Pal sRegionMapBg_GfxLZ sRegionMapBg_TilemapLZ sRegionMapPlayerIcon_BrendanPal sRegionMapPlayerIcon_BrendanGfx sRegionMapPlayerIcon_MayPal sRegionMapPlayerIcon_MayGfx sRegionMap_MapSectionLayout sMapName_LITTLEROOT_TOWN sMapName_OLDALE_TOWN sMapName_DEWFORD_TOWN sMapName_LAVARIDGE_TOWN sMapName_FALLARBOR_TOWN sMapName_VERDANTURF_TOWN sMapName_PACIFIDLOG_TOWN sMapName_PETALBURG_CITY sMapName_SLATEPORT_CITY sMapName_MAUVILLE_CITY sMapName_RUSTBORO_CITY sMapName_FORTREE_CITY sMapName_LILYCOVE_CITY sMapName_MOSSDEEP_CITY sMapName_SOOTOPOLIS_CITY sMapName_EVER_GRANDE_CITY sMapName_ROUTE_101 sMapName_ROUTE_102 sMapName_ROUTE_103 sMapName_ROUTE_104 sMapName_ROUTE_105 sMapName_ROUTE_106 sMapName_ROUTE_107 sMapName_ROUTE_108 sMapName_ROUTE_109 sMapName_ROUTE_110 sMapName_ROUTE_111 sMapName_ROUTE_112 sMapName_ROUTE_113 sMapName_ROUTE_114 sMapName_ROUTE_115 sMapName_ROUTE_116 sMapName_ROUTE_117 sMapName_ROUTE_118 sMapName_ROUTE_119 sMapName_ROUTE_120 sMapName_ROUTE_121 sMapName_ROUTE_122 sMapName_ROUTE_123 sMapName_ROUTE_124 sMapName_ROUTE_125 sMapName_ROUTE_126 sMapName_ROUTE_127 sMapName_ROUTE_128 sMapName_ROUTE_129 sMapName_ROUTE_130 sMapName_ROUTE_131 sMapName_ROUTE_132 sMapName_ROUTE_133 sMapName_ROUTE_134 sMapName_UNDERWATER sMapName_GRANITE_CAVE sMapName_MT__CHIMNEY sMapName_SAFARI_ZONE sMapName_BATTLE_FRONTIER sMapName_PETALBURG_WOODS sMapName_RUSTURF_TUNNEL sMapName_ABANDONED_SHIP sMapName_NEW_MAUVILLE sMapName_METEOR_FALLS sMapName_MT__PYRE sMapName__AQUA__HIDEOUT_Clone sMapName_SHOAL_CAVE sMapName_SEAFLOOR_CAVERN sMapName_VICTORY_ROAD sMapName_MIRAGE_ISLAND sMapName_CAVE_OF_ORIGIN sMapName_SOUTHERN_ISLAND sMapName_FIERY_PATH sMapName_JAGGED_PASS sMapName_SEALED_CHAMBER sMapName_SCORCHED_SLAB sMapName_ISLAND_CAVE sMapName_DESERT_RUINS sMapName_ANCIENT_TOMB sMapName_INSIDE_OF_TRUCK sMapName_SKY_PILLAR sMapName_SECRET_BASE sMapName_ sMapName_PALLET_TOWN sMapName_VIRIDIAN_CITY sMapName_PEWTER_CITY sMapName_CERULEAN_CITY sMapName_LAVENDER_TOWN sMapName_VERMILION_CITY sMapName_CELADON_CITY sMapName_FUCHSIA_CITY sMapName_CINNABAR_ISLAND sMapName_INDIGO_PLATEAU sMapName_SAFFRON_CITY sMapName_ROUTE_4_Clone sMapName_ROUTE_10_Clone sMapName_ROUTE_1 sMapName_ROUTE_2 sMapName_ROUTE_3 sMapName_ROUTE_4 sMapName_ROUTE_5 sMapName_ROUTE_6 sMapName_ROUTE_7 sMapName_ROUTE_8 sMapName_ROUTE_9 sMapName_ROUTE_10 sMapName_ROUTE_11 sMapName_ROUTE_12 sMapName_ROUTE_13 sMapName_ROUTE_14 sMapName_ROUTE_15 sMapName_ROUTE_16 sMapName_ROUTE_17 sMapName_ROUTE_18 sMapName_ROUTE_19 sMapName_ROUTE_20 sMapName_ROUTE_21 sMapName_ROUTE_22 sMapName_ROUTE_23 sMapName_ROUTE_24 sMapName_ROUTE_25 sMapName_VIRIDIAN_FOREST sMapName_MT__MOON sMapName_S_S__ANNE sMapName_UNDERGROUND_PATH sMapName_UNDERGROUND_PATH_Clone sMapName_DIGLETT_S_CAVE sMapName_VICTORY_ROAD_Clone sMapName_ROCKET_HIDEOUT sMapName_SILPH_CO_ sMapName_POK__MON_MANSION sMapName_SAFARI_ZONE_Clone sMapName_POK__MON_LEAGUE sMapName_ROCK_TUNNEL sMapName_SEAFOAM_ISLANDS sMapName_POK__MON_TOWER sMapName_CERULEAN_CAVE sMapName_POWER_PLANT sMapName_ONE_ISLAND sMapName_TWO_ISLAND sMapName_THREE_ISLAND sMapName_FOUR_ISLAND sMapName_FIVE_ISLAND sMapName_SEVEN_ISLAND sMapName_SIX_ISLAND sMapName_KINDLE_ROAD sMapName_TREASURE_BEACH sMapName_CAPE_BRINK sMapName_BOND_BRIDGE sMapName_THREE_ISLE_PORT sMapName_SEVII_ISLE_6 sMapName_SEVII_ISLE_7 sMapName_SEVII_ISLE_8 sMapName_SEVII_ISLE_9 sMapName_RESORT_GORGEOUS sMapName_WATER_LABYRINTH sMapName_FIVE_ISLE_MEADOW sMapName_MEMORIAL_PILLAR sMapName_OUTCAST_ISLAND sMapName_GREEN_PATH sMapName_WATER_PATH sMapName_RUIN_VALLEY sMapName_TRAINER_TOWER sMapName_CANYON_ENTRANCE sMapName_SEVAULT_CANYON sMapName_TANOBY_RUINS sMapName_SEVII_ISLE_22 sMapName_SEVII_ISLE_23 sMapName_SEVII_ISLE_24 sMapName_NAVEL_ROCK sMapName_MT__EMBER sMapName_BERRY_FOREST sMapName_ICEFALL_CAVE sMapName_ROCKET_WAREHOUSE sMapName_TRAINER_TOWER_Clone sMapName_DOTTED_HOLE sMapName_LOST_CAVE sMapName_PATTERN_BUSH sMapName_ALTERING_CAVE sMapName_TANOBY_CHAMBERS sMapName_THREE_ISLE_PATH sMapName_TANOBY_KEY sMapName_BIRTH_ISLAND sMapName_MONEAN_CHAMBER sMapName_LIPTOO_CHAMBER sMapName_WEEPTH_CHAMBER sMapName_DILFORD_CHAMBER sMapName_SCUFIB_CHAMBER sMapName_RIXY_CHAMBER sMapName_VIAPOIS_CHAMBER sMapName_EMBER_SPA sMapName_SPECIAL_AREA sMapName_AQUA_HIDEOUT sMapName_MAGMA_HIDEOUT sMapName_MIRAGE_TOWER sMapName_FARAWAY_ISLAND sMapName_ARTISAN_CAVE sMapName_MARINE_CAVE sMapName_TERRA_CAVE sMapName_DESERT_UNDERPASS sMapName_TRAINER_HILL gRegionMapEntries sRegionMap_SpecialPlaceLocations sMarineCaveMapSecIds sTerraOrMarineCaveMapSecIds sMarineCaveLocationCoords sMapSecAquaHideoutOld sRegionMapCursorOam sRegionMapCursorAnim1 sRegionMapCursorAnim2 sRegionMapCursorAnimTable sRegionMapCursorSpritePalette sRegionMapCursorSpriteTemplate sRegionMapPlayerIconOam sRegionMapPlayerIconAnim1 sRegionMapPlayerIconAnimTable sMapSecIdsOffMap sRegionMapFramePal sRegionMapFrameGfxLZ sRegionMapFrameTilemapLZ sFlyTargetIcons_Pal sFlyTargetIcons_Gfx sMapHealLocations sEverGrandeCityNames sMultiNameFlyDestinations sFlyMapBgTemplates sFlyMapWindowTemplates sFlyTargetIconsSpritePalette sRedOutlineFlyDestinations sFlyDestIcon_OamData sFlyDestIcon_Anim_8x8CanFly sFlyDestIcon_Anim_16x8CanFly sFlyDestIcon_Anim_8x16CanFly sFlyDestIcon_Anim_8x8CantFly sFlyDestIcon_Anim_16x8CantFly sFlyDestIcon_Anim_8x16CantFly sFlyDestIcon_Anim_RedOutline sFlyDestIcon_Anims sFlyDestIconSpriteTemplate

/// `__typeof__(*((__typeof__(sFlyMap))0))`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct typeof___sFlyMap_0_t {
    pub callback: Option<unsafe extern "C" fn()>,
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

#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sRegionMap: *mut RegionMap = null_mut();
pub(crate) static mut sFlyMap: *mut typeof___sFlyMap_0_t = null_mut();
pub(crate) static mut sDrawFlyDestTextWindow: u32 = 0;

unsafe extern "C" {
    static gDummySpriteAffineAnimTable: CArray<*mut AffineAnimCmd, 0>;
    static mut gMain: Main;
    static mut gMapHeader: MapHeader;
    static mut gPlttBufferFaded: CArray<u16, 512>;
    static mut gPlttBufferUnfaded: CArray<u16, 512>;
    static mut gSaveBlock1Ptr: *mut SaveBlock1;
    static mut gSaveBlock2Ptr: *mut SaveBlock2;
    static gSineTable: CArray<i16, 0>;
    static mut gSprites: CArray<Sprite, 65>;
    static gText_Ferry: CArray<u8, 0>;
    static gText_FlyToWhere: CArray<u8, 0>;
    static gText_Hideout: CArray<u8, 0>;
    static gText_SecretBase: CArray<u8, 0>;
    fn AddTextPrinterParameterized(
        a0: u8,
        a1: u8,
        a2: *mut u8,
        a3: u8,
        a4: u8,
        a5: u8,
        a6: Option<unsafe extern "C" fn(*mut TextPrinterTemplate, u16)>,
    ) -> u16;
    fn Alloc(a0: u32) -> *mut c_void;
    fn AnimateSprites();
    fn BeginNormalPaletteFade(a0: u32, a1: i8, a2: u8, a3: u8, a4: u16) -> u8;
    fn BlendPalettes(a0: u32, a1: u8, a2: u16);
    fn BuildOamBuffer();
    fn CB2_ReturnToFieldWithOpenMenu();
    fn CB2_ReturnToPartyMenuFromFlyMap();
    fn ClearScheduledBgCopiesToVram();
    fn ClearStdWindowAndFrameToTransparent(a0: u8, a1: u8);
    fn CopyWindowToVram(a0: u8, a1: u8);
    fn CpuSet(a0: *mut c_void, a1: *mut c_void, a2: u32);
    fn CreateSprite(a0: *mut SpriteTemplate, a1: i16, a2: i16, a3: u8) -> u8;
    fn DeactivateAllTextPrinters();
    fn DecompressAndCopyTileDataToVram(
        a0: u8,
        a1: *mut c_void,
        a2: u32,
        a3: u16,
        a4: u8,
    ) -> *mut c_void;
    fn DestroySprite(a0: *mut Sprite);
    fn DoScheduledBgTilemapCopiesToVram();
    fn DrawStdFrameWithCustomTileAndPalette(a0: u8, a1: u8, a2: u16, a3: u8);
    fn FillWindowPixelBuffer(a0: u8, a1: u8);
    fn FlagGet(a0: u16) -> u8;
    fn Free(a0: *mut c_void);
    fn FreeAllSpritePalettes();
    fn FreeAllWindowBuffers();
    fn FreeSpritePaletteByTag(a0: u16);
    fn FreeSpriteTileRanges();
    fn FreeSpriteTilesByTag(a0: u16);
    fn FreeTempTileDataBuffersIfPossible() -> u8;
    fn GetMapTypeByGroupAndId(a0: i8, a1: i8) -> u8;
    fn GetSSTidalLocation(a0: *mut i8, a1: *mut i8, a2: *mut i16, a3: *mut i16) -> u8;
    fn GetSecretBaseMapName(a0: *mut u8) -> *mut u8;
    fn GetStringRightAlignXOffset(a0: i32, a1: *mut u8, a2: i32) -> i32;
    fn IndexOfSpritePaletteTag(a0: u16) -> u8;
    fn InitBgsFromTemplates(a0: u8, a1: *mut BgTemplate, a2: u8);
    fn InitWindows(a0: *mut WindowTemplate) -> u16;
    fn LZ77UnCompVram(a0: *mut u32, a1: *mut c_void);
    fn LZ77UnCompWram(a0: *mut u32, a1: *mut c_void);
    fn LoadOam();
    fn LoadPalette(a0: *mut c_void, a1: u16, a2: u16);
    fn LoadSpritePalette(a0: *mut SpritePalette) -> u8;
    fn LoadSpriteSheet(a0: *mut SpriteSheet) -> u16;
    fn LoadUserWindowBorderGfx(a0: u8, a1: u16, a2: u8);
    fn Overworld_GetMapHeaderByGroupAndId(a0: u16, a1: u16) -> *mut MapHeader;
    fn ProcessSpriteCopyRequests();
    fn PutWindowTilemap(a0: u8);
    fn ResetBgsAndClearDma3BusyFlags(a0: u32);
    fn ResetPaletteFade();
    fn ResetSpriteData();
    fn ReturnToFieldFromFlyMapSelect();
    fn ScheduleBgCopyTilemapToVram(a0: u8);
    fn SetBgAttribute(a0: u8, a1: u8, a2: u8);
    fn SetGpuReg(a0: u8, a1: u16);
    fn SetGpuRegBits(a0: u8, a1: u16);
    fn SetMainCallback2(a0: Option<unsafe extern "C" fn()>);
    fn SetVBlankCallback(a0: Option<unsafe extern "C" fn()>);
    fn SetWarpDestinationToHealLocation(a0: u8);
    fn SetWarpDestinationToMapWarp(a0: i8, a1: i8, a2: i8);
    fn ShowBg(a0: u8);
    fn SpriteCallbackDummy(a0: *mut Sprite);
    fn StartSpriteAnim(a0: *mut Sprite, a1: u8);
    fn StringCopy(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn StringFill(a0: *mut u8, a1: u8, a2: u16) -> *mut u8;
    fn StringLength(a0: *mut u8) -> u16;
    fn TransferPlttBuffer();
    fn UpdatePaletteFade() -> u8;
    fn VarGet(a0: u16) -> u16;
    fn m4aSongNumStart(a0: u16);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn InitRegionMap(regionMap: *mut RegionMap, zoomed: u8) {
    InitRegionMapData(regionMap, null_mut(), zoomed);
    while LoadRegionMapGfx() != 0 {}
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn InitRegionMapData(
    regionMap: *mut RegionMap,
    template: *mut BgTemplate,
    zoomed: u8,
) {
    sRegionMap = regionMap;
    (*sRegionMap).initStep = 0;
    (*sRegionMap).zoomed = zoomed;
    (*sRegionMap).inputCallback = if zoomed == TRUE {
        Some(ProcessRegionMapInput_Zoomed)
    } else {
        Some(ProcessRegionMapInput_Full)
    };
    if !template.is_null() {
        (*sRegionMap).bgNum = (*template).bg() as u8;
        (*sRegionMap).charBaseIdx = (*template).charBaseIndex() as u8;
        (*sRegionMap).mapBaseIdx = (*template).mapBaseIndex() as u8;
        (*sRegionMap).bgManaged = TRUE;
    } else {
        (*sRegionMap).bgNum = 2;
        (*sRegionMap).charBaseIdx = 2;
        (*sRegionMap).mapBaseIdx = 28;
        (*sRegionMap).bgManaged = FALSE;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ShowRegionMapForPokedexAreaScreen(regionMap: *mut RegionMap) {
    sRegionMap = regionMap;
    InitMapBasedOnPlayerLocation();
    (*sRegionMap).playerIconSpritePosX = (*sRegionMap).cursorPosX;
    (*sRegionMap).playerIconSpritePosY = (*sRegionMap).cursorPosY;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn LoadRegionMapGfx() -> u8 {
    match (*sRegionMap).initStep {
        0 => {
            if (*sRegionMap).bgManaged != 0 {
                DecompressAndCopyTileDataToVram(
                    (*sRegionMap).bgNum,
                    sRegionMapBg_GfxLZ.as_ptr().cast_mut() as *mut c_void,
                    0,
                    0,
                    0,
                );
            } else {
                LZ77UnCompVram(
                    sRegionMapBg_GfxLZ.as_ptr().cast_mut(),
                    0x6008000 as usize as *mut u16 as *mut c_void,
                );
            }
        }
        1 => {
            if (*sRegionMap).bgManaged != 0 {
                if FreeTempTileDataBuffersIfPossible() == 0 {
                    DecompressAndCopyTileDataToVram(
                        (*sRegionMap).bgNum,
                        sRegionMapBg_TilemapLZ.as_ptr().cast_mut() as *mut c_void,
                        0,
                        0,
                        1,
                    );
                }
            } else {
                LZ77UnCompVram(
                    sRegionMapBg_TilemapLZ.as_ptr().cast_mut(),
                    0x600e000 as usize as *mut u16 as *mut c_void,
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
                (*sRegionMap).cursorSmallImage.as_mut_ptr() as *mut c_void,
            );
        }
        4 => {
            LZ77UnCompWram(
                sRegionMapCursorLargeGfxLZ.as_ptr().cast_mut(),
                (*sRegionMap).cursorLargeImage.as_mut_ptr() as *mut c_void,
            );
        }
        5 => {
            InitMapBasedOnPlayerLocation();
            (*sRegionMap).playerIconSpritePosX = (*sRegionMap).cursorPosX;
            (*sRegionMap).playerIconSpritePosY = (*sRegionMap).cursorPosY;
            (*sRegionMap).mapSecId = CorrectSpecialMapSecId_Internal((*sRegionMap).mapSecId);
            (*sRegionMap).mapSecType = GetMapsecType((*sRegionMap).mapSecId);
            GetMapName(
                (*sRegionMap).mapSecName.as_mut_ptr(),
                (*sRegionMap).mapSecId,
                MAP_NAME_LENGTH,
            );
        }
        6 => {
            if (*sRegionMap).zoomed == FALSE {
                CalcZoomScrollParams(0, 0, 0, 0, 0x100, 0x100, 0);
            } else {
                (*sRegionMap).scrollX = (*sRegionMap).cursorPosX as i16 * 8 - 0x34;
                (*sRegionMap).scrollY = (*sRegionMap).cursorPosY as i16 * 8 - 0x44;
                (*sRegionMap).zoomedCursorPosX = (*sRegionMap).cursorPosX;
                (*sRegionMap).zoomedCursorPosY = (*sRegionMap).cursorPosY;
                CalcZoomScrollParams(
                    (*sRegionMap).scrollX,
                    (*sRegionMap).scrollY,
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
            (*sRegionMap).cursorSprite = null_mut();
            (*sRegionMap).playerIconSprite = null_mut();
            (*sRegionMap).cursorMovementFrameCounter = 0;
            (*sRegionMap).blinkPlayerIcon = FALSE;
            if (*sRegionMap).bgManaged != 0 {
                SetBgAttribute((*sRegionMap).bgNum, BG_ATTR_SCREENSIZE, 2);
                SetBgAttribute(
                    (*sRegionMap).bgNum,
                    BG_ATTR_CHARBASEINDEX,
                    (*sRegionMap).charBaseIdx,
                );
                SetBgAttribute(
                    (*sRegionMap).bgNum,
                    BG_ATTR_MAPBASEINDEX,
                    (*sRegionMap).mapBaseIdx,
                );
                SetBgAttribute((*sRegionMap).bgNum, BG_ATTR_WRAPAROUND, 1);
                SetBgAttribute((*sRegionMap).bgNum, BG_ATTR_PALETTEMODE, 1);
            }
            (*sRegionMap).initStep += 1;
            return FALSE;
        }
        _ => {
            return FALSE;
        }
    }
    (*sRegionMap).initStep += 1;
    return TRUE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BlendRegionMap(color: u16, coeff: u32) {
    BlendPalettes(0x380, coeff as u8, color);
    CpuSet(
        &raw mut gPlttBufferFaded[112] as *mut c_void,
        &raw mut gPlttBufferUnfaded[112] as *mut c_void,
        48,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FreeRegionMapIconResources() {
    if !(*sRegionMap).cursorSprite.is_null() {
        DestroySprite((*sRegionMap).cursorSprite);
        FreeSpriteTilesByTag((*sRegionMap).cursorTileTag);
        FreeSpritePaletteByTag((*sRegionMap).cursorPaletteTag);
    }
    if !(*sRegionMap).playerIconSprite.is_null() {
        DestroySprite((*sRegionMap).playerIconSprite);
        FreeSpriteTilesByTag((*sRegionMap).playerIconTileTag);
        FreeSpritePaletteByTag((*sRegionMap).playerIconPaletteTag);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DoRegionMapInputCallback() -> u8 {
    return (*sRegionMap).inputCallback.unwrap_unchecked()();
}
pub(crate) unsafe extern "C" fn ProcessRegionMapInput_Full() -> u8 {
    let mut input: u8 = 0;
    input = MAP_INPUT_NONE;
    (*sRegionMap).cursorDeltaX = 0;
    (*sRegionMap).cursorDeltaY = 0;
    if gMain.heldKeys as i32 & DPAD_UP != 0 && (*sRegionMap).cursorPosY > MAPCURSOR_Y_MIN {
        (*sRegionMap).cursorDeltaY = -1;
        input = MAP_INPUT_MOVE_START;
    }
    if gMain.heldKeys as i32 & DPAD_DOWN != 0 && (*sRegionMap).cursorPosY < MAPCURSOR_Y_MAX {
        (*sRegionMap).cursorDeltaY = 1;
        input = MAP_INPUT_MOVE_START;
    }
    if gMain.heldKeys as i32 & DPAD_LEFT != 0 && (*sRegionMap).cursorPosX > MAPCURSOR_X_MIN {
        (*sRegionMap).cursorDeltaX = -1;
        input = MAP_INPUT_MOVE_START;
    }
    if gMain.heldKeys as i32 & DPAD_RIGHT != 0 && (*sRegionMap).cursorPosX < MAPCURSOR_X_MAX {
        (*sRegionMap).cursorDeltaX = 1;
        input = MAP_INPUT_MOVE_START;
    }
    if gMain.newKeys as i32 & A_BUTTON != 0 {
        input = MAP_INPUT_A_BUTTON;
    } else if gMain.newKeys as i32 & B_BUTTON != 0 {
        input = MAP_INPUT_B_BUTTON;
    }
    if input == MAP_INPUT_MOVE_START {
        (*sRegionMap).cursorMovementFrameCounter = 4;
        (*sRegionMap).inputCallback = Some(MoveRegionMapCursor_Full);
    }
    return input;
}
pub(crate) unsafe extern "C" fn MoveRegionMapCursor_Full() -> u8 {
    let mut mapSecId: u16 = 0;
    if (*sRegionMap).cursorMovementFrameCounter != 0 {
        return MAP_INPUT_MOVE_CONT;
    }
    if (*sRegionMap).cursorDeltaX > 0 {
        (*sRegionMap).cursorPosX += 1;
    }
    if (*sRegionMap).cursorDeltaX < 0 {
        (*sRegionMap).cursorPosX -= 1;
    }
    if (*sRegionMap).cursorDeltaY > 0 {
        (*sRegionMap).cursorPosY += 1;
    }
    if (*sRegionMap).cursorDeltaY < 0 {
        (*sRegionMap).cursorPosY -= 1;
    }
    mapSecId = GetMapSecIdAt((*sRegionMap).cursorPosX, (*sRegionMap).cursorPosY);
    (*sRegionMap).mapSecType = GetMapsecType(mapSecId);
    if mapSecId != (*sRegionMap).mapSecId {
        (*sRegionMap).mapSecId = mapSecId;
        GetMapName(
            (*sRegionMap).mapSecName.as_mut_ptr(),
            (*sRegionMap).mapSecId,
            MAP_NAME_LENGTH,
        );
    }
    GetPositionOfCursorWithinMapSec();
    (*sRegionMap).inputCallback = Some(ProcessRegionMapInput_Full);
    return MAP_INPUT_MOVE_END;
}
pub(crate) unsafe extern "C" fn ProcessRegionMapInput_Zoomed() -> u8 {
    let mut input: u8 = 0;
    input = MAP_INPUT_NONE;
    (*sRegionMap).zoomedCursorDeltaX = 0;
    (*sRegionMap).zoomedCursorDeltaY = 0;
    if gMain.heldKeys as i32 & DPAD_UP != 0 && (*sRegionMap).scrollY > -52 {
        (*sRegionMap).zoomedCursorDeltaY = -1;
        input = MAP_INPUT_MOVE_START;
    }
    if gMain.heldKeys as i32 & DPAD_DOWN != 0 && (*sRegionMap).scrollY < 0x3c {
        (*sRegionMap).zoomedCursorDeltaY = 1;
        input = MAP_INPUT_MOVE_START;
    }
    if gMain.heldKeys as i32 & DPAD_LEFT != 0 && (*sRegionMap).scrollX > -44 {
        (*sRegionMap).zoomedCursorDeltaX = -1;
        input = MAP_INPUT_MOVE_START;
    }
    if gMain.heldKeys as i32 & DPAD_RIGHT != 0 && (*sRegionMap).scrollX < 0xac {
        (*sRegionMap).zoomedCursorDeltaX = 1;
        input = MAP_INPUT_MOVE_START;
    }
    if gMain.newKeys as i32 & A_BUTTON != 0 {
        input = MAP_INPUT_A_BUTTON;
    }
    if gMain.newKeys as i32 & B_BUTTON != 0 {
        input = MAP_INPUT_B_BUTTON;
    }
    if input == MAP_INPUT_MOVE_START {
        (*sRegionMap).inputCallback = Some(MoveRegionMapCursor_Zoomed);
        (*sRegionMap).zoomedCursorMovementFrameCounter = 0;
    }
    return input;
}
pub(crate) unsafe extern "C" fn MoveRegionMapCursor_Zoomed() -> u8 {
    let mut x: u16 = 0;
    let mut y: u16 = 0;
    let mut mapSecId: u16 = 0;
    (*sRegionMap).scrollY += (*sRegionMap).zoomedCursorDeltaY;
    (*sRegionMap).scrollX += (*sRegionMap).zoomedCursorDeltaX;
    RegionMap_SetBG2XAndBG2Y((*sRegionMap).scrollX, (*sRegionMap).scrollY);
    (*sRegionMap).zoomedCursorMovementFrameCounter += 1;
    if (*sRegionMap).zoomedCursorMovementFrameCounter == 8 {
        x = (((*sRegionMap).scrollX as i32 + 0x2c) / 8) as u16 + 1;
        y = (((*sRegionMap).scrollY as i32 + 0x34) / 8) as u16 + 2;
        if x != (*sRegionMap).zoomedCursorPosX || y != (*sRegionMap).zoomedCursorPosY {
            (*sRegionMap).zoomedCursorPosX = x;
            (*sRegionMap).zoomedCursorPosY = y;
            mapSecId = GetMapSecIdAt(x, y);
            (*sRegionMap).mapSecType = GetMapsecType(mapSecId);
            if mapSecId != (*sRegionMap).mapSecId {
                (*sRegionMap).mapSecId = mapSecId;
                GetMapName(
                    (*sRegionMap).mapSecName.as_mut_ptr(),
                    (*sRegionMap).mapSecId,
                    MAP_NAME_LENGTH,
                );
            }
            GetPositionOfCursorWithinMapSec();
        }
        (*sRegionMap).zoomedCursorMovementFrameCounter = 0;
        (*sRegionMap).inputCallback = Some(ProcessRegionMapInput_Zoomed);
        return MAP_INPUT_MOVE_END;
    }
    return MAP_INPUT_MOVE_CONT;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetRegionMapDataForZoom() {
    if (*sRegionMap).zoomed == FALSE {
        (*sRegionMap).scrollY = 0;
        (*sRegionMap).scrollX = 0;
        (*sRegionMap).unk_040 = 0;
        (*sRegionMap).unk_03c = 0;
        (*sRegionMap).unk_060 = (*sRegionMap).cursorPosX as i16 * 8 - 0x34;
        (*sRegionMap).unk_062 = (*sRegionMap).cursorPosY as i16 * 8 - 0x44;
        (*sRegionMap).unk_044 = (((*sRegionMap).unk_060 as i32) << 8) / 16;
        (*sRegionMap).unk_048 = (((*sRegionMap).unk_062 as i32) << 8) / 16;
        (*sRegionMap).zoomedCursorPosX = (*sRegionMap).cursorPosX;
        (*sRegionMap).zoomedCursorPosY = (*sRegionMap).cursorPosY;
        (*sRegionMap).unk_04c = 0x10000;
        (*sRegionMap).unk_050 = -2048;
    } else {
        (*sRegionMap).unk_03c = (*sRegionMap).scrollX as i32 * 0x100;
        (*sRegionMap).unk_040 = (*sRegionMap).scrollY as i32 * 0x100;
        (*sRegionMap).unk_060 = 0;
        (*sRegionMap).unk_062 = 0;
        (*sRegionMap).unk_044 = -((*sRegionMap).unk_03c / 16);
        (*sRegionMap).unk_048 = -((*sRegionMap).unk_040 / 16);
        (*sRegionMap).cursorPosX = (*sRegionMap).zoomedCursorPosX;
        (*sRegionMap).cursorPosY = (*sRegionMap).zoomedCursorPosY;
        (*sRegionMap).unk_04c = 0x8000;
        (*sRegionMap).unk_050 = 0x800;
    }
    (*sRegionMap).unk_06e = 0;
    FreeRegionMapCursorSprite();
    HideRegionMapPlayerIcon();
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn UpdateRegionMapZoom() -> u8 {
    let mut retVal: u8 = 0;
    if (*sRegionMap).unk_06e >= 16 {
        return FALSE;
    }
    (*sRegionMap).unk_06e += 1;
    if (*sRegionMap).unk_06e == 16 {
        (*sRegionMap).unk_044 = 0;
        (*sRegionMap).unk_048 = 0;
        (*sRegionMap).scrollX = (*sRegionMap).unk_060;
        (*sRegionMap).scrollY = (*sRegionMap).unk_062;
        (*sRegionMap).unk_04c = if (*sRegionMap).zoomed == FALSE {
            32768
        } else {
            0x10000
        };
        (*sRegionMap).zoomed = ((*sRegionMap).zoomed == 0) as u8;
        (*sRegionMap).inputCallback = if (*sRegionMap).zoomed == FALSE {
            Some(ProcessRegionMapInput_Full)
        } else {
            Some(ProcessRegionMapInput_Zoomed)
        };
        CreateRegionMapCursor((*sRegionMap).cursorTileTag, (*sRegionMap).cursorPaletteTag);
        UnhideRegionMapPlayerIcon();
        retVal = FALSE;
    } else {
        (*sRegionMap).unk_03c += (*sRegionMap).unk_044;
        (*sRegionMap).unk_040 += (*sRegionMap).unk_048;
        (*sRegionMap).scrollX = ((*sRegionMap).unk_03c >> 8) as i16;
        (*sRegionMap).scrollY = ((*sRegionMap).unk_040 >> 8) as i16;
        (*sRegionMap).unk_04c += (*sRegionMap).unk_050;
        if (*sRegionMap).unk_044 < 0 && (*sRegionMap).scrollX < (*sRegionMap).unk_060
            || (*sRegionMap).unk_044 > 0 && (*sRegionMap).scrollX > (*sRegionMap).unk_060
        {
            (*sRegionMap).scrollX = (*sRegionMap).unk_060;
            (*sRegionMap).unk_044 = 0;
        }
        if (*sRegionMap).unk_048 < 0 && (*sRegionMap).scrollY < (*sRegionMap).unk_062
            || (*sRegionMap).unk_048 > 0 && (*sRegionMap).scrollY > (*sRegionMap).unk_062
        {
            (*sRegionMap).scrollY = (*sRegionMap).unk_062;
            (*sRegionMap).unk_048 = 0;
        }
        if (*sRegionMap).zoomed == FALSE {
            if (*sRegionMap).unk_04c < 32768 {
                (*sRegionMap).unk_04c = 32768;
                (*sRegionMap).unk_050 = 0;
            }
        } else {
            if (*sRegionMap).unk_04c > 0x10000 {
                (*sRegionMap).unk_04c = 0x10000;
                (*sRegionMap).unk_050 = 0;
            }
        }
        retVal = TRUE;
    }
    CalcZoomScrollParams(
        (*sRegionMap).scrollX,
        (*sRegionMap).scrollY,
        0x38,
        0x48,
        ((*sRegionMap).unk_04c >> 8) as u16,
        ((*sRegionMap).unk_04c >> 8) as u16,
        0,
    );
    return retVal;
}
pub(crate) unsafe extern "C" fn CalcZoomScrollParams(
    scrollX: i16,
    scrollY: i16,
    c: i16,
    d: i16,
    e: u16,
    f: u16,
    rotation: u8,
) {
    let mut var1: i32 = 0;
    let mut var2: i32 = 0;
    let mut var3: i32 = 0;
    let mut var4: i32 = 0;
    (*sRegionMap).bg2pa = (e as i32 * gSineTable[rotation as i32 + 64] as i32 >> 8) as u32;
    (*sRegionMap).bg2pc = (e as i32 * -(gSineTable[rotation] as i32) >> 8) as u32;
    (*sRegionMap).bg2pb = (f as i32 * gSineTable[rotation] as i32 >> 8) as u32;
    (*sRegionMap).bg2pd = (f as i32 * gSineTable[rotation as i32 + 64] as i32 >> 8) as u32;
    var1 = ((scrollX as i32) << 8) + ((c as i32) << 8);
    var2 = d as i32 * (*sRegionMap).bg2pb as i32 + (*sRegionMap).bg2pa as i32 * c as i32;
    (*sRegionMap).bg2x = var1 - var2;
    var3 = ((scrollY as i32) << 8) + ((d as i32) << 8);
    var4 = (*sRegionMap).bg2pd as i32 * d as i32 + (*sRegionMap).bg2pc as i32 * c as i32;
    (*sRegionMap).bg2y = var3 - var4;
    (*sRegionMap).needUpdateVideoRegs = TRUE;
}
pub(crate) unsafe extern "C" fn RegionMap_SetBG2XAndBG2Y(x: i16, y: i16) {
    (*sRegionMap).bg2x = ((x as i32) << 8) + 0x1c00;
    (*sRegionMap).bg2y = ((y as i32) << 8) + 0x2400;
    (*sRegionMap).needUpdateVideoRegs = TRUE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn UpdateRegionMapVideoRegs() {
    if (*sRegionMap).needUpdateVideoRegs != 0 {
        SetGpuReg(REG_OFFSET_BG2PA, (*sRegionMap).bg2pa as u16);
        SetGpuReg(REG_OFFSET_BG2PB, (*sRegionMap).bg2pb as u16);
        SetGpuReg(REG_OFFSET_BG2PC, (*sRegionMap).bg2pc as u16);
        SetGpuReg(REG_OFFSET_BG2PD, (*sRegionMap).bg2pd as u16);
        SetGpuReg(REG_OFFSET_BG2X_L, (*sRegionMap).bg2x as u16);
        SetGpuReg(REG_OFFSET_BG2X_H, ((*sRegionMap).bg2x >> 16) as u16);
        SetGpuReg(REG_OFFSET_BG2Y_L, (*sRegionMap).bg2y as u16);
        SetGpuReg(REG_OFFSET_BG2Y_H, ((*sRegionMap).bg2y >> 16) as u16);
        (*sRegionMap).needUpdateVideoRegs = FALSE;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PokedexAreaScreen_UpdateRegionMapVariablesAndVideoRegs(x: i16, y: i16) {
    CalcZoomScrollParams(x, y, 0x38, 0x48, 0x100, 0x100, 0);
    UpdateRegionMapVideoRegs();
    if !(*sRegionMap).playerIconSprite.is_null() {
        (*(*sRegionMap).playerIconSprite).x2 = -x;
        (*(*sRegionMap).playerIconSprite).y2 = -y;
    }
}
pub(crate) unsafe extern "C" fn GetMapSecIdAt(mut x: u16, mut y: u16) -> u16 {
    if y < MAPCURSOR_Y_MIN || y > MAPCURSOR_Y_MAX || x < MAPCURSOR_X_MIN || x > MAPCURSOR_X_MAX {
        return MAPSEC_NONE;
    }
    y -= MAPCURSOR_Y_MIN;
    x -= MAPCURSOR_X_MIN;
    return sRegionMap_MapSectionLayout[y][x] as u16;
}
pub(crate) unsafe extern "C" fn InitMapBasedOnPlayerLocation() {
    let mut mapHeader: *mut MapHeader = null_mut();
    let mut mapWidth: u16 = 0;
    let mut mapHeight: u16 = 0;
    let mut x: u16 = 0;
    let mut y: u16 = 0;
    let mut dimensionScale: u16 = 0;
    let mut xOnMap: u16 = 0;
    let mut warp: *mut WarpData = null_mut();
    if (*gSaveBlock1Ptr).location.mapGroup == 25
        && ((*gSaveBlock1Ptr).location.mapNum == 41
            || (*gSaveBlock1Ptr).location.mapNum == 42
            || (*gSaveBlock1Ptr).location.mapNum == 43)
    {
        RegionMap_InitializeStateBasedOnSSTidalLocation();
        return;
    }
    match GetMapTypeByGroupAndId(
        (*gSaveBlock1Ptr).location.mapGroup,
        (*gSaveBlock1Ptr).location.mapNum,
    ) {
        MAP_TYPE_UNDERGROUND | MAP_TYPE_UNKNOWN => {
            if gMapHeader.allowEscaping() != 0 {
                mapHeader = Overworld_GetMapHeaderByGroupAndId(
                    (*gSaveBlock1Ptr).escapeWarp.mapGroup as u16,
                    (*gSaveBlock1Ptr).escapeWarp.mapNum as u16,
                );
                (*sRegionMap).mapSecId = (*mapHeader).regionMapSectionId as u16;
                (*sRegionMap).playerIsInCave = TRUE;
                mapWidth = (*(*mapHeader).mapLayout).width as u16;
                mapHeight = (*(*mapHeader).mapLayout).height as u16;
                x = (*gSaveBlock1Ptr).escapeWarp.x as u16;
                y = (*gSaveBlock1Ptr).escapeWarp.y as u16;
            } else {
                (*sRegionMap).mapSecId = gMapHeader.regionMapSectionId as u16;
                (*sRegionMap).playerIsInCave = TRUE;
                mapWidth = 1;
                mapHeight = 1;
                x = 1;
                y = 1;
            }
        }
        MAP_TYPE_SECRET_BASE => {
            mapHeader = Overworld_GetMapHeaderByGroupAndId(
                (*gSaveBlock1Ptr).dynamicWarp.mapGroup as u16,
                (*gSaveBlock1Ptr).dynamicWarp.mapNum as u16,
            );
            (*sRegionMap).mapSecId = (*mapHeader).regionMapSectionId as u16;
            (*sRegionMap).playerIsInCave = TRUE;
            mapWidth = (*(*mapHeader).mapLayout).width as u16;
            mapHeight = (*(*mapHeader).mapLayout).height as u16;
            x = (*gSaveBlock1Ptr).dynamicWarp.x as u16;
            y = (*gSaveBlock1Ptr).dynamicWarp.y as u16;
        }
        MAP_TYPE_INDOOR => {
            (*sRegionMap).mapSecId = gMapHeader.regionMapSectionId as u16;
            if (*sRegionMap).mapSecId != MAPSEC_DYNAMIC {
                warp = &raw mut (*gSaveBlock1Ptr).escapeWarp;
                mapHeader = Overworld_GetMapHeaderByGroupAndId(
                    (*warp).mapGroup as u16,
                    (*warp).mapNum as u16,
                );
            } else {
                warp = &raw mut (*gSaveBlock1Ptr).dynamicWarp;
                mapHeader = Overworld_GetMapHeaderByGroupAndId(
                    (*warp).mapGroup as u16,
                    (*warp).mapNum as u16,
                );
                (*sRegionMap).mapSecId = (*mapHeader).regionMapSectionId as u16;
            }
            if IsPlayerInAquaHideout((*sRegionMap).mapSecId as u8) != 0 {
                (*sRegionMap).playerIsInCave = TRUE;
            } else {
                (*sRegionMap).playerIsInCave = FALSE;
            }
            mapWidth = (*(*mapHeader).mapLayout).width as u16;
            mapHeight = (*(*mapHeader).mapLayout).height as u16;
            x = (*warp).x as u16;
            y = (*warp).y as u16;
        }
        _ => {
            (*sRegionMap).mapSecId = gMapHeader.regionMapSectionId as u16;
            (*sRegionMap).playerIsInCave = FALSE;
            mapWidth = (*gMapHeader.mapLayout).width as u16;
            mapHeight = (*gMapHeader.mapLayout).height as u16;
            x = (*gSaveBlock1Ptr).pos.x as u16;
            y = (*gSaveBlock1Ptr).pos.y as u16;
            if (*sRegionMap).mapSecId == MAPSEC_UNDERWATER_SEAFLOOR_CAVERN
                || (*sRegionMap).mapSecId == MAPSEC_UNDERWATER_MARINE_CAVE
            {
                (*sRegionMap).playerIsInCave = TRUE;
            }
        }
    }
    xOnMap = x;
    dimensionScale = div_i32(
        mapWidth as i32,
        gRegionMapEntries[(*sRegionMap).mapSecId].width as i32,
    ) as u16;
    if dimensionScale == 0 {
        dimensionScale = 1;
    }
    x = div_i32(x as i32, dimensionScale as i32) as u16;
    if x >= gRegionMapEntries[(*sRegionMap).mapSecId].width as u16 {
        x = gRegionMapEntries[(*sRegionMap).mapSecId].width as u16 - 1;
    }
    dimensionScale = div_i32(
        mapHeight as i32,
        gRegionMapEntries[(*sRegionMap).mapSecId].height as i32,
    ) as u16;
    if dimensionScale == 0 {
        dimensionScale = 1;
    }
    y = div_i32(y as i32, dimensionScale as i32) as u16;
    if y >= gRegionMapEntries[(*sRegionMap).mapSecId].height as u16 {
        y = gRegionMapEntries[(*sRegionMap).mapSecId].height as u16 - 1;
    }
    match (*sRegionMap).mapSecId {
        MAPSEC_ROUTE_114 => {
            if y != 0 {
                x = 0;
            }
        }
        MAPSEC_ROUTE_126 | MAPSEC_UNDERWATER_126 => {
            x = 0;
            if (*gSaveBlock1Ptr).pos.x > 32 {
                x += 1;
            }
            if (*gSaveBlock1Ptr).pos.x > 51 {
                x += 1;
            }
            y = 0;
            if (*gSaveBlock1Ptr).pos.y > 37 {
                y += 1;
            }
            if (*gSaveBlock1Ptr).pos.y > 56 {
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
                &raw mut (*sRegionMap).cursorPosX,
                &raw mut (*sRegionMap).cursorPosY,
            );
            return;
        }
        _ => {}
    }
    (*sRegionMap).cursorPosX =
        gRegionMapEntries[(*sRegionMap).mapSecId].x as u16 + x + MAPCURSOR_X_MIN;
    (*sRegionMap).cursorPosY =
        gRegionMapEntries[(*sRegionMap).mapSecId].y as u16 + y + MAPCURSOR_Y_MIN;
}
pub(crate) unsafe extern "C" fn RegionMap_InitializeStateBasedOnSSTidalLocation() {
    let mut y: u16 = 0;
    let mut x: u16 = 0;
    let mut mapGroup: u8 = 0;
    let mut mapNum: u8 = 0;
    let mut dimensionScale: u16 = 0;
    let mut xOnMap: i16 = 0;
    let mut yOnMap: i16 = 0;
    let mut mapHeader: *mut MapHeader = null_mut();
    y = 0;
    x = 0;
    match GetSSTidalLocation(
        &raw mut mapGroup as *mut i8,
        &raw mut mapNum as *mut i8,
        &raw mut xOnMap,
        &raw mut yOnMap,
    ) {
        SS_TIDAL_LOCATION_SLATEPORT => {
            (*sRegionMap).mapSecId = MAPSEC_SLATEPORT_CITY;
        }
        SS_TIDAL_LOCATION_LILYCOVE => {
            (*sRegionMap).mapSecId = MAPSEC_LILYCOVE_CITY;
        }
        SS_TIDAL_LOCATION_ROUTE124 => {
            (*sRegionMap).mapSecId = MAPSEC_ROUTE_124;
        }
        SS_TIDAL_LOCATION_ROUTE131 => {
            (*sRegionMap).mapSecId = MAPSEC_ROUTE_131;
        }
        _ => {
            mapHeader = Overworld_GetMapHeaderByGroupAndId(mapGroup as u16, mapNum as u16);
            (*sRegionMap).mapSecId = (*mapHeader).regionMapSectionId as u16;
            dimensionScale = div_i32(
                (*(*mapHeader).mapLayout).width,
                gRegionMapEntries[(*sRegionMap).mapSecId].width as i32,
            ) as u16;
            if dimensionScale == 0 {
                dimensionScale = 1;
            }
            x = div_i32(xOnMap as i32, dimensionScale as i32) as u16;
            if x >= gRegionMapEntries[(*sRegionMap).mapSecId].width as u16 {
                x = gRegionMapEntries[(*sRegionMap).mapSecId].width as u16 - 1;
            }
            dimensionScale = div_i32(
                (*(*mapHeader).mapLayout).height,
                gRegionMapEntries[(*sRegionMap).mapSecId].height as i32,
            ) as u16;
            if dimensionScale == 0 {
                dimensionScale = 1;
            }
            y = div_i32(yOnMap as i32, dimensionScale as i32) as u16;
            if y >= gRegionMapEntries[(*sRegionMap).mapSecId].height as u16 {
                y = gRegionMapEntries[(*sRegionMap).mapSecId].height as u16 - 1;
            }
        }
    }
    (*sRegionMap).playerIsInCave = FALSE;
    (*sRegionMap).cursorPosX =
        gRegionMapEntries[(*sRegionMap).mapSecId].x as u16 + x + MAPCURSOR_X_MIN;
    (*sRegionMap).cursorPosY =
        gRegionMapEntries[(*sRegionMap).mapSecId].y as u16 + y + MAPCURSOR_Y_MIN;
}
pub(crate) unsafe extern "C" fn GetMapsecType(mapSecId: u16) -> u8 {
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
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetRegionMapSecIdAt(x: u16, y: u16) -> u16 {
    return GetMapSecIdAt(x, y);
}
pub(crate) unsafe extern "C" fn CorrectSpecialMapSecId_Internal(mapSecId: u16) -> u16 {
    let mut i: u32 = 0;
    i = 0;
    while i < 3 {
        if sMarineCaveMapSecIds[i] == mapSecId {
            return GetTerraOrMarineCaveMapSecId();
        }
        i += 1;
    }
    i = 0;
    while sRegionMap_SpecialPlaceLocations[i][0] != MAPSEC_NONE {
        if sRegionMap_SpecialPlaceLocations[i][0] == mapSecId {
            return sRegionMap_SpecialPlaceLocations[i][1];
        }
        i += 1;
    }
    return mapSecId;
}
pub(crate) unsafe extern "C" fn GetTerraOrMarineCaveMapSecId() -> u16 {
    let mut idx: i16 = 0;
    idx = VarGet(VAR_ABNORMAL_WEATHER_LOCATION) as i16 - 1;
    if idx < 0 || idx > 15 {
        idx = 0;
    }
    return sTerraOrMarineCaveMapSecIds[idx];
}
pub(crate) unsafe extern "C" fn GetMarineCaveCoords(x: *mut u16, y: *mut u16) {
    let mut idx: u16 = 0;
    idx = VarGet(VAR_ABNORMAL_WEATHER_LOCATION);
    if idx < MARINE_CAVE_LOCATIONS_START || idx > ABNORMAL_WEATHER_LOCATIONS {
        idx = MARINE_CAVE_LOCATIONS_START;
    }
    idx -= MARINE_CAVE_LOCATIONS_START;
    *x = sMarineCaveLocationCoords[idx].x + MAPCURSOR_X_MIN;
    *y = sMarineCaveLocationCoords[idx].y + MAPCURSOR_Y_MIN;
}
pub(crate) unsafe extern "C" fn IsPlayerInAquaHideout(mapSecId: u8) -> u32 {
    let mut i: u32 = 0;
    i = 0;
    while i < 1 {
        if sMapSecAquaHideoutOld[i] == mapSecId {
            return TRUE as u32;
        }
        i += 1;
    }
    return FALSE as u32;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CorrectSpecialMapSecId(mapSecId: u16) -> u16 {
    return CorrectSpecialMapSecId_Internal(mapSecId);
}
pub(crate) unsafe extern "C" fn GetPositionOfCursorWithinMapSec() {
    let mut x: u16 = 0;
    let mut y: u16 = 0;
    let mut posWithinMapSec: u16 = 0;
    if (*sRegionMap).mapSecId == MAPSEC_NONE {
        (*sRegionMap).posWithinMapSec = 0;
        return;
    }
    if (*sRegionMap).zoomed == 0 {
        x = (*sRegionMap).cursorPosX;
        y = (*sRegionMap).cursorPosY;
    } else {
        x = (*sRegionMap).zoomedCursorPosX;
        y = (*sRegionMap).zoomedCursorPosY;
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
            if GetMapSecIdAt(x, y) == (*sRegionMap).mapSecId {
                posWithinMapSec += 1;
            }
        }
    }
    (*sRegionMap).posWithinMapSec = posWithinMapSec as u8;
}
pub(crate) unsafe extern "C" fn RegionMap_IsMapSecIdInNextRow(mut y: u16) -> u8 {
    let mut x: u16 = 0;
    if ({
        let t1 = y;
        y -= 1;
        t1
    }) == 0
    {
        return FALSE;
    }
    x = MAPCURSOR_X_MIN;
    while x <= MAPCURSOR_X_MAX {
        if GetMapSecIdAt(x, y) == (*sRegionMap).mapSecId {
            return TRUE;
        }
        x += 1;
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn SpriteCB_CursorMapFull(sprite: *mut Sprite) {
    if (*sRegionMap).cursorMovementFrameCounter != 0 {
        (*sprite).x += 2 * (*sRegionMap).cursorDeltaX as i16;
        (*sprite).y += 2 * (*sRegionMap).cursorDeltaY as i16;
        (*sRegionMap).cursorMovementFrameCounter -= 1;
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_CursorMapZoomed(sprite: *mut Sprite) {}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CreateRegionMapCursor(tileTag: u16, paletteTag: u16) {
    let mut spriteId: u8 = 0;
    let mut template: SpriteTemplate = zeroed();
    let mut palette: SpritePalette = zeroed();
    let mut sheet: SpriteSheet = zeroed();
    palette = *sRegionMapCursorSpritePalette;
    template = *sRegionMapCursorSpriteTemplate;
    sheet.tag = tileTag;
    template.tileTag = tileTag;
    (*sRegionMap).cursorTileTag = tileTag;
    palette.tag = paletteTag;
    template.paletteTag = paletteTag;
    (*sRegionMap).cursorPaletteTag = paletteTag;
    if (*sRegionMap).zoomed == 0 {
        sheet.data = (*sRegionMap).cursorSmallImage.as_mut_ptr() as *mut c_void;
        sheet.size = 256;
        template.callback = Some(SpriteCB_CursorMapFull);
    } else {
        sheet.data = (*sRegionMap).cursorLargeImage.as_mut_ptr() as *mut c_void;
        sheet.size = 1536;
        template.callback = Some(SpriteCB_CursorMapZoomed);
    }
    LoadSpriteSheet(&raw mut sheet);
    LoadSpritePalette(&raw mut palette);
    spriteId = CreateSprite(&raw mut template, 56, 72, 0);
    if spriteId != MAX_SPRITES {
        (*sRegionMap).cursorSprite = &raw mut gSprites[spriteId];
        if (*sRegionMap).zoomed == TRUE {
            (*(*sRegionMap).cursorSprite).oam.set_size(2);
            (*(*sRegionMap).cursorSprite).x -= 8;
            (*(*sRegionMap).cursorSprite).y -= 8;
            StartSpriteAnim((*sRegionMap).cursorSprite, 1);
        } else {
            (*(*sRegionMap).cursorSprite).oam.set_size(1);
            (*(*sRegionMap).cursorSprite).x = 8 * (*sRegionMap).cursorPosX as i16 + 4;
            (*(*sRegionMap).cursorSprite).y = 8 * (*sRegionMap).cursorPosY as i16 + 4;
        }
        (*(*sRegionMap).cursorSprite).data[1] = 2;
        (*(*sRegionMap).cursorSprite).data[2] =
            0x100 + IndexOfSpritePaletteTag(paletteTag) as i16 * 16 + 1;
        (*(*sRegionMap).cursorSprite).data[3] = TRUE as i16;
    }
}
pub(crate) unsafe extern "C" fn FreeRegionMapCursorSprite() {
    if !(*sRegionMap).cursorSprite.is_null() {
        DestroySprite((*sRegionMap).cursorSprite);
        FreeSpriteTilesByTag((*sRegionMap).cursorTileTag);
        FreeSpritePaletteByTag((*sRegionMap).cursorPaletteTag);
    }
}
pub(crate) unsafe extern "C" fn SetUnkCursorSpriteData() {
    (*(*sRegionMap).cursorSprite).data[3] = TRUE as i16;
}
pub(crate) unsafe extern "C" fn ClearUnkCursorSpriteData() {
    (*(*sRegionMap).cursorSprite).data[3] = FALSE as i16;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CreateRegionMapPlayerIcon(tileTag: u16, paletteTag: u16) {
    let mut spriteId: u8 = 0;
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
    template.affineAnims = gDummySpriteAffineAnimTable.as_ptr().cast_mut();
    template.callback = Some(SpriteCallbackDummy);
    if IsEventIslandMapSecId(gMapHeader.regionMapSectionId) != 0 {
        (*sRegionMap).playerIconSprite = null_mut();
        return;
    }
    if (*gSaveBlock2Ptr).playerGender == FEMALE {
        sheet.data = sRegionMapPlayerIcon_MayGfx.as_ptr().cast_mut() as *mut c_void;
        palette.data = sRegionMapPlayerIcon_MayPal.as_ptr().cast_mut();
    }
    LoadSpriteSheet(&raw mut sheet);
    LoadSpritePalette(&raw mut palette);
    spriteId = CreateSprite(&raw mut template, 0, 0, 1);
    (*sRegionMap).playerIconSprite = &raw mut gSprites[spriteId];
    if (*sRegionMap).zoomed == 0 {
        (*(*sRegionMap).playerIconSprite).x = (*sRegionMap).playerIconSpritePosX as i16 * 8 + 4;
        (*(*sRegionMap).playerIconSprite).y = (*sRegionMap).playerIconSpritePosY as i16 * 8 + 4;
        (*(*sRegionMap).playerIconSprite).callback = Some(SpriteCB_PlayerIconMapFull);
    } else {
        (*(*sRegionMap).playerIconSprite).x = (*sRegionMap).playerIconSpritePosX as i16 * 16 - 0x30;
        (*(*sRegionMap).playerIconSprite).y = (*sRegionMap).playerIconSpritePosY as i16 * 16 - 0x42;
        (*(*sRegionMap).playerIconSprite).callback = Some(SpriteCB_PlayerIconMapZoomed);
    }
}
pub(crate) unsafe extern "C" fn HideRegionMapPlayerIcon() {
    if !(*sRegionMap).playerIconSprite.is_null() {
        (*(*sRegionMap).playerIconSprite).set_invisible(TRUE as u16);
        (*(*sRegionMap).playerIconSprite).callback = Some(SpriteCallbackDummy);
    }
}
pub(crate) unsafe extern "C" fn UnhideRegionMapPlayerIcon() {
    if !(*sRegionMap).playerIconSprite.is_null() {
        if (*sRegionMap).zoomed == TRUE {
            (*(*sRegionMap).playerIconSprite).x =
                (*sRegionMap).playerIconSpritePosX as i16 * 16 - 0x30;
            (*(*sRegionMap).playerIconSprite).y =
                (*sRegionMap).playerIconSpritePosY as i16 * 16 - 0x42;
            (*(*sRegionMap).playerIconSprite).callback = Some(SpriteCB_PlayerIconMapZoomed);
            (*(*sRegionMap).playerIconSprite).set_invisible(FALSE as u16);
        } else {
            (*(*sRegionMap).playerIconSprite).x = (*sRegionMap).playerIconSpritePosX as i16 * 8 + 4;
            (*(*sRegionMap).playerIconSprite).y = (*sRegionMap).playerIconSpritePosY as i16 * 8 + 4;
            (*(*sRegionMap).playerIconSprite).x2 = 0;
            (*(*sRegionMap).playerIconSprite).y2 = 0;
            (*(*sRegionMap).playerIconSprite).callback = Some(SpriteCB_PlayerIconMapFull);
            (*(*sRegionMap).playerIconSprite).set_invisible(FALSE as u16);
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_PlayerIconMapZoomed(sprite: *mut Sprite) {
    (*sprite).x2 = -2 * (*sRegionMap).scrollX;
    (*sprite).y2 = -2 * (*sRegionMap).scrollY;
    (*sprite).data[0] = (*sprite).y + (*sprite).y2 + (*sprite).centerToCornerVecY as i16;
    (*sprite).data[1] = (*sprite).x + (*sprite).x2 + (*sprite).centerToCornerVecX as i16;
    if (*sprite).data[0] < -8
        || (*sprite).data[0] > 168
        || (*sprite).data[1] < -8
        || (*sprite).data[1] > 248
    {
        (*sprite).data[2] = FALSE as i16;
    } else {
        (*sprite).data[2] = TRUE as i16;
    }
    if (*sprite).data[2] == TRUE as i16 {
        SpriteCB_PlayerIcon(sprite);
    } else {
        (*sprite).set_invisible(TRUE as u16);
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_PlayerIconMapFull(sprite: *mut Sprite) {
    SpriteCB_PlayerIcon(sprite);
}
pub(crate) unsafe extern "C" fn SpriteCB_PlayerIcon(sprite: *mut Sprite) {
    if (*sRegionMap).blinkPlayerIcon != 0 {
        if ({
            (*sprite).data[7] += 1;
            (*sprite).data[7]
        }) > 16
        {
            (*sprite).data[7] = 0;
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn TrySetPlayerIconBlink() {
    if (*sRegionMap).playerIsInCave != 0 {
        (*sRegionMap).blinkPlayerIcon = TRUE;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetMapName(
    dest: *mut u8,
    regionMapId: u16,
    mut padLength: u16,
) -> *mut u8 {
    let mut str: *mut u8 = null_mut();
    let mut i: u16 = 0;
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
        i = (str as usize).wrapping_sub(dest as usize) as i32 as u16;
        while i < padLength {
            *({
                let t1 = str;
                str = str.at(1);
                t1
            }) = CHAR_SPACE;
            i += 1;
        }
        *str = EOS;
    }
    return str;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetMapNameGeneric(dest: *mut u8, mapSecId: u16) -> *mut u8 {
    match mapSecId {
        MAPSEC_DYNAMIC => {
            return StringCopy(dest, gText_Ferry.as_ptr().cast_mut());
        }
        86 => {
            return StringCopy(dest, gText_SecretBase.as_ptr().cast_mut());
        }
        _ => {
            return GetMapName(dest, mapSecId, 0);
        }
    }
    #[allow(unreachable_code)]
    {
        return null_mut();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetMapNameHandleAquaHideout(dest: *mut u8, mapSecId: u16) -> *mut u8 {
    if mapSecId == MAPSEC_AQUA_HIDEOUT_OLD {
        return StringCopy(dest, gText_Hideout.as_ptr().cast_mut());
    } else {
        return GetMapNameGeneric(dest, mapSecId);
    }
    #[allow(unreachable_code)]
    {
        return null_mut();
    }
}
pub(crate) unsafe extern "C" fn GetMapSecDimensions(
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsRegionMapZoomed() -> u8 {
    return (*sRegionMap).zoomed;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsEventIslandMapSecId(mapSecId: u8) -> u32 {
    let mut i: u32 = 0;
    i = 0;
    while i < 3 {
        if mapSecId == sMapSecIdsOffMap[i] {
            return TRUE as u32;
        }
        i += 1;
    }
    return FALSE as u32;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CB2_OpenFlyMap() {
    match gMain.state {
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
            sFlyMap = Alloc(2676) as *mut typeof___sFlyMap_0_t;
            if sFlyMap.is_null() {
                SetMainCallback2(Some(CB2_ReturnToFieldWithOpenMenu));
            } else {
                ResetPaletteFade();
                ResetSpriteData();
                FreeSpriteTileRanges();
                FreeAllSpritePalettes();
                gMain.state += 1;
            }
        }
        1 => {
            ResetBgsAndClearDma3BusyFlags(0);
            InitBgsFromTemplates(1, sFlyMapBgTemplates.as_ptr().cast_mut(), 3);
            gMain.state += 1;
        }
        2 => {
            InitWindows(sFlyMapWindowTemplates.as_ptr().cast_mut());
            DeactivateAllTextPrinters();
            gMain.state += 1;
        }
        3 => {
            LoadUserWindowBorderGfx(0, 0x65, 208);
            ClearScheduledBgCopiesToVram();
            gMain.state += 1;
        }
        4 => {
            InitRegionMap(&raw mut (*sFlyMap).regionMap, FALSE);
            CreateRegionMapCursor(TAG_CURSOR, TAG_CURSOR);
            CreateRegionMapPlayerIcon(TAG_PLAYER_ICON, TAG_PLAYER_ICON);
            (*sFlyMap).mapSecId = (*sFlyMap).regionMap.mapSecId;
            StringFill(
                (*sFlyMap).nameBuffer.as_mut_ptr(),
                CHAR_SPACE,
                MAP_NAME_LENGTH,
            );
            sDrawFlyDestTextWindow = TRUE as u32;
            DrawFlyDestTextWindow();
            gMain.state += 1;
        }
        5 => {
            LZ77UnCompVram(
                sRegionMapFrameGfxLZ.as_ptr().cast_mut(),
                0x600c000 as usize as *mut u16 as *mut c_void,
            );
            gMain.state += 1;
        }
        6 => {
            LZ77UnCompVram(
                sRegionMapFrameTilemapLZ.as_ptr().cast_mut(),
                0x600f000 as usize as *mut u16 as *mut c_void,
            );
            gMain.state += 1;
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
                gText_FlyToWhere.as_ptr().cast_mut(),
                0,
                1,
                0,
                None,
            );
            ScheduleBgCopyTilemapToVram(0);
            gMain.state += 1;
        }
        8 => {
            LoadFlyDestIcons();
            gMain.state += 1;
        }
        9 => {
            BlendPalettes(PALETTES_ALL, 16, 0);
            SetVBlankCallback(Some(VBlankCB_FlyMap));
            gMain.state += 1;
        }
        10 => {
            SetGpuReg(REG_OFFSET_BLDCNT, 0);
            SetGpuRegBits(REG_OFFSET_DISPCNT, 4160);
            ShowBg(0);
            ShowBg(1);
            ShowBg(2);
            SetFlyMapCallback(Some(CB_FadeInFlyMap));
            SetMainCallback2(Some(CB2_FlyMap));
            gMain.state += 1;
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn VBlankCB_FlyMap() {
    LoadOam();
    ProcessSpriteCopyRequests();
    TransferPlttBuffer();
}
pub(crate) unsafe extern "C" fn CB2_FlyMap() {
    (*sFlyMap).callback.unwrap_unchecked()();
    AnimateSprites();
    BuildOamBuffer();
    DoScheduledBgTilemapCopiesToVram();
}
pub(crate) unsafe extern "C" fn SetFlyMapCallback(callback: Option<unsafe extern "C" fn()>) {
    (*sFlyMap).callback = callback;
    (*sFlyMap).state = 0;
}
pub(crate) unsafe extern "C" fn DrawFlyDestTextWindow() {
    let mut i: u16 = 0;
    let mut namePrinted: u32 = 0;
    let mut name: *mut u8 = null_mut();
    if (*sFlyMap).regionMap.mapSecType > MAPSECTYPE_NONE
        && (*sFlyMap).regionMap.mapSecType < NUM_MAPSEC_TYPES
    {
        namePrinted = FALSE as u32;
        i = 0;
        while i < 1 {
            if (*sFlyMap).regionMap.mapSecId == sMultiNameFlyDestinations[i].mapSecId {
                if FlagGet(sMultiNameFlyDestinations[i].flag) != 0 {
                    StringLength(
                        *sMultiNameFlyDestinations[i]
                            .name
                            .at((*sFlyMap).regionMap.posWithinMapSec),
                    );
                    namePrinted = TRUE as u32;
                    ClearStdWindowAndFrameToTransparent(WIN_MAPSEC_NAME, FALSE);
                    DrawStdFrameWithCustomTileAndPalette(WIN_MAPSEC_NAME_TALL, FALSE, 101, 13);
                    AddTextPrinterParameterized(
                        WIN_MAPSEC_NAME_TALL,
                        FONT_NORMAL,
                        (*sFlyMap).regionMap.mapSecName.as_mut_ptr(),
                        0,
                        1,
                        0,
                        None,
                    );
                    name = *sMultiNameFlyDestinations[i]
                        .name
                        .at((*sFlyMap).regionMap.posWithinMapSec);
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
                    sDrawFlyDestTextWindow = TRUE as u32;
                }
                break;
            }
            i += 1;
        }
        if namePrinted == 0 {
            if sDrawFlyDestTextWindow == TRUE as u32 {
                ClearStdWindowAndFrameToTransparent(WIN_MAPSEC_NAME_TALL, FALSE);
                DrawStdFrameWithCustomTileAndPalette(WIN_MAPSEC_NAME, FALSE, 101, 13);
            } else {
                FillWindowPixelBuffer(WIN_MAPSEC_NAME, 17);
            }
            AddTextPrinterParameterized(
                WIN_MAPSEC_NAME,
                FONT_NORMAL,
                (*sFlyMap).regionMap.mapSecName.as_mut_ptr(),
                0,
                1,
                0,
                None,
            );
            ScheduleBgCopyTilemapToVram(0);
            sDrawFlyDestTextWindow = FALSE as u32;
        }
    } else {
        if sDrawFlyDestTextWindow == TRUE as u32 {
            ClearStdWindowAndFrameToTransparent(WIN_MAPSEC_NAME_TALL, FALSE);
            DrawStdFrameWithCustomTileAndPalette(WIN_MAPSEC_NAME, FALSE, 101, 13);
        }
        FillWindowPixelBuffer(WIN_MAPSEC_NAME, 17);
        CopyWindowToVram(WIN_MAPSEC_NAME, COPYWIN_GFX);
        ScheduleBgCopyTilemapToVram(0);
        sDrawFlyDestTextWindow = FALSE as u32;
    }
}
pub(crate) unsafe extern "C" fn LoadFlyDestIcons() {
    let mut sheet: SpriteSheet = zeroed();
    LZ77UnCompWram(
        sFlyTargetIcons_Gfx.as_ptr().cast_mut(),
        (*sFlyMap).tileBuffer.as_mut_ptr() as *mut c_void,
    );
    sheet.data = (*sFlyMap).tileBuffer.as_mut_ptr() as *mut c_void;
    sheet.size = 448;
    sheet.tag = TAG_FLY_ICON;
    LoadSpriteSheet(&raw mut sheet);
    LoadSpritePalette((&raw const *sFlyTargetIconsSpritePalette).cast_mut());
    CreateFlyDestIcons();
    TryCreateRedOutlineFlyDestIcons();
}
pub(crate) unsafe extern "C" fn CreateFlyDestIcons() {
    let mut canFlyFlag: u16 = 0;
    let mut mapSecId: u16 = 0;
    let mut x: u16 = 0;
    let mut y: u16 = 0;
    let mut width: u16 = 0;
    let mut height: u16 = 0;
    let mut shape: u16 = 0;
    let mut spriteId: u8 = 0;
    canFlyFlag = FLAG_VISITED_LITTLEROOT_TOWN;
    mapSecId = MAPSEC_LITTLEROOT_TOWN;
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
            gSprites[spriteId].oam.set_shape(shape as u32);
            if FlagGet(canFlyFlag) != 0 {
                gSprites[spriteId].callback = Some(SpriteCB_FlyDestIcon);
            } else {
                shape += 3;
            }
            StartSpriteAnim(&raw mut gSprites[spriteId], shape as u8);
            gSprites[spriteId].data[0] = mapSecId as i16;
        }
        canFlyFlag += 1;
        mapSecId += 1;
    }
}
pub(crate) unsafe extern "C" fn TryCreateRedOutlineFlyDestIcons() {
    let mut i: u16 = 0;
    let mut x: u16 = 0;
    let mut y: u16 = 0;
    let mut width: u16 = 0;
    let mut height: u16 = 0;
    let mut mapSecId: u16 = 0;
    let mut spriteId: u8 = 0;
    i = 0;
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
                gSprites[spriteId].oam.set_size(1);
                gSprites[spriteId].callback = Some(SpriteCB_FlyDestIcon);
                StartSpriteAnim(&raw mut gSprites[spriteId], FLYDESTICON_RED_OUTLINE);
                gSprites[spriteId].data[0] = mapSecId as i16;
            }
        }
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_FlyDestIcon(sprite: *mut Sprite) {
    if (*sFlyMap).regionMap.mapSecId as i32 == (*sprite).data[0] as i32 {
        if ({
            (*sprite).data[1] += 1;
            (*sprite).data[1]
        }) > 16
        {
            (*sprite).data[1] = 0;
            (*sprite).set_invisible(
                (if (*sprite).invisible() != 0 {
                    FALSE as i32
                } else {
                    TRUE as i32
                }) as u16,
            );
        }
    } else {
        (*sprite).data[1] = 16;
        (*sprite).set_invisible(FALSE as u16);
    }
}
pub(crate) unsafe extern "C" fn CB_FadeInFlyMap() {
    match (*sFlyMap).state {
        0 => {
            BeginNormalPaletteFade(PALETTES_ALL, 0, 16, 0, 0);
            (*sFlyMap).state += 1;
        }
        1 => {
            if UpdatePaletteFade() == 0 {
                SetFlyMapCallback(Some(CB_HandleFlyMapInput));
            }
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn CB_HandleFlyMapInput() {
    if (*sFlyMap).state == 0 {
        match DoRegionMapInputCallback() {
            MAP_INPUT_NONE | MAP_INPUT_MOVE_START | MAP_INPUT_MOVE_CONT => {}
            MAP_INPUT_MOVE_END => {
                DrawFlyDestTextWindow();
            }
            MAP_INPUT_A_BUTTON => {
                if (*sFlyMap).regionMap.mapSecType == MAPSECTYPE_CITY_CANFLY as u8
                    || (*sFlyMap).regionMap.mapSecType == MAPSECTYPE_BATTLE_FRONTIER
                {
                    m4aSongNumStart(SE_SELECT);
                    (*sFlyMap).choseFlyLocation = TRUE;
                    SetFlyMapCallback(Some(CB_ExitFlyMap));
                }
            }
            MAP_INPUT_B_BUTTON => {
                m4aSongNumStart(SE_SELECT);
                (*sFlyMap).choseFlyLocation = FALSE;
                SetFlyMapCallback(Some(CB_ExitFlyMap));
            }
            _ => {}
        }
    }
}
pub(crate) unsafe extern "C" fn CB_ExitFlyMap() {
    match (*sFlyMap).state {
        0 => {
            BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 16, 0);
            (*sFlyMap).state += 1;
        }
        1 => {
            if UpdatePaletteFade() == 0 {
                FreeRegionMapIconResources();
                if (*sFlyMap).choseFlyLocation != 0 {
                    match (*sFlyMap).regionMap.mapSecId {
                        MAPSEC_SOUTHERN_ISLAND => {
                            SetWarpDestinationToHealLocation(
                                HEAL_LOCATION_SOUTHERN_ISLAND_EXTERIOR,
                            );
                        }
                        58 => {
                            SetWarpDestinationToHealLocation(
                                HEAL_LOCATION_BATTLE_FRONTIER_OUTSIDE_EAST,
                            );
                        }
                        MAPSEC_LITTLEROOT_TOWN => {
                            SetWarpDestinationToHealLocation(
                                (if (*gSaveBlock2Ptr).playerGender == MALE {
                                    HEAL_LOCATION_LITTLEROOT_TOWN_BRENDANS_HOUSE
                                } else {
                                    HEAL_LOCATION_LITTLEROOT_TOWN_MAYS_HOUSE
                                }) as u8,
                            );
                        }
                        MAPSEC_EVER_GRANDE_CITY => {
                            SetWarpDestinationToHealLocation(
                                (if FlagGet(FLAG_LANDMARK_POKEMON_LEAGUE) != 0
                                    && (*sFlyMap).regionMap.posWithinMapSec == 0
                                {
                                    HEAL_LOCATION_EVER_GRANDE_CITY_POKEMON_LEAGUE
                                } else {
                                    HEAL_LOCATION_EVER_GRANDE_CITY
                                }) as u8,
                            );
                        }
                        _ => {
                            if sMapHealLocations[(*sFlyMap).regionMap.mapSecId][2]
                                != HEAL_LOCATION_NONE
                            {
                                SetWarpDestinationToHealLocation(
                                    sMapHealLocations[(*sFlyMap).regionMap.mapSecId][2],
                                );
                            } else {
                                SetWarpDestinationToMapWarp(
                                    sMapHealLocations[(*sFlyMap).regionMap.mapSecId][0] as i8,
                                    sMapHealLocations[(*sFlyMap).regionMap.mapSecId][1] as i8,
                                    WARP_ID_NONE,
                                );
                            }
                        }
                    }
                    ReturnToFieldFromFlyMapSelect();
                } else {
                    SetMainCallback2(Some(CB2_ReturnToPartyMenuFromFlyMap));
                }
                if !sFlyMap.is_null() {
                    Free(sFlyMap as *mut c_void);
                    sFlyMap = null_mut();
                }
                FreeAllWindowBuffers();
            }
        }
        _ => {}
    }
}
