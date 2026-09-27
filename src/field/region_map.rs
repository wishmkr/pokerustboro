//! Translated from `src/region_map.c` by tools/rustport/c2rs.py, then reviewed.
#![allow(
    non_snake_case,
    non_upper_case_globals,
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
    clippy::all,
    clashing_extern_declarations,
    unpredictable_function_pointer_comparisons
)]

// Data tables (translate with cdata.py): sRegionMapCursorPal sRegionMapCursorSmallGfxLZ sRegionMapCursorLargeGfxLZ sRegionMapBg_Pal sRegionMapBg_GfxLZ sRegionMapBg_TilemapLZ sRegionMapPlayerIcon_BrendanPal sRegionMapPlayerIcon_BrendanGfx sRegionMapPlayerIcon_MayPal sRegionMapPlayerIcon_MayGfx sRegionMap_MapSectionLayout sMapName_LITTLEROOT_TOWN sMapName_OLDALE_TOWN sMapName_DEWFORD_TOWN sMapName_LAVARIDGE_TOWN sMapName_FALLARBOR_TOWN sMapName_VERDANTURF_TOWN sMapName_PACIFIDLOG_TOWN sMapName_PETALBURG_CITY sMapName_SLATEPORT_CITY sMapName_MAUVILLE_CITY sMapName_RUSTBORO_CITY sMapName_FORTREE_CITY sMapName_LILYCOVE_CITY sMapName_MOSSDEEP_CITY sMapName_SOOTOPOLIS_CITY sMapName_EVER_GRANDE_CITY sMapName_ROUTE_101 sMapName_ROUTE_102 sMapName_ROUTE_103 sMapName_ROUTE_104 sMapName_ROUTE_105 sMapName_ROUTE_106 sMapName_ROUTE_107 sMapName_ROUTE_108 sMapName_ROUTE_109 sMapName_ROUTE_110 sMapName_ROUTE_111 sMapName_ROUTE_112 sMapName_ROUTE_113 sMapName_ROUTE_114 sMapName_ROUTE_115 sMapName_ROUTE_116 sMapName_ROUTE_117 sMapName_ROUTE_118 sMapName_ROUTE_119 sMapName_ROUTE_120 sMapName_ROUTE_121 sMapName_ROUTE_122 sMapName_ROUTE_123 sMapName_ROUTE_124 sMapName_ROUTE_125 sMapName_ROUTE_126 sMapName_ROUTE_127 sMapName_ROUTE_128 sMapName_ROUTE_129 sMapName_ROUTE_130 sMapName_ROUTE_131 sMapName_ROUTE_132 sMapName_ROUTE_133 sMapName_ROUTE_134 sMapName_UNDERWATER sMapName_GRANITE_CAVE sMapName_MT__CHIMNEY sMapName_SAFARI_ZONE sMapName_BATTLE_FRONTIER sMapName_PETALBURG_WOODS sMapName_RUSTURF_TUNNEL sMapName_ABANDONED_SHIP sMapName_NEW_MAUVILLE sMapName_METEOR_FALLS sMapName_MT__PYRE sMapName__AQUA__HIDEOUT_Clone sMapName_SHOAL_CAVE sMapName_SEAFLOOR_CAVERN sMapName_VICTORY_ROAD sMapName_MIRAGE_ISLAND sMapName_CAVE_OF_ORIGIN sMapName_SOUTHERN_ISLAND sMapName_FIERY_PATH sMapName_JAGGED_PASS sMapName_SEALED_CHAMBER sMapName_SCORCHED_SLAB sMapName_ISLAND_CAVE sMapName_DESERT_RUINS sMapName_ANCIENT_TOMB sMapName_INSIDE_OF_TRUCK sMapName_SKY_PILLAR sMapName_SECRET_BASE sMapName_ sMapName_PALLET_TOWN sMapName_VIRIDIAN_CITY sMapName_PEWTER_CITY sMapName_CERULEAN_CITY sMapName_LAVENDER_TOWN sMapName_VERMILION_CITY sMapName_CELADON_CITY sMapName_FUCHSIA_CITY sMapName_CINNABAR_ISLAND sMapName_INDIGO_PLATEAU sMapName_SAFFRON_CITY sMapName_ROUTE_4_Clone sMapName_ROUTE_10_Clone sMapName_ROUTE_1 sMapName_ROUTE_2 sMapName_ROUTE_3 sMapName_ROUTE_4 sMapName_ROUTE_5 sMapName_ROUTE_6 sMapName_ROUTE_7 sMapName_ROUTE_8 sMapName_ROUTE_9 sMapName_ROUTE_10 sMapName_ROUTE_11 sMapName_ROUTE_12 sMapName_ROUTE_13 sMapName_ROUTE_14 sMapName_ROUTE_15 sMapName_ROUTE_16 sMapName_ROUTE_17 sMapName_ROUTE_18 sMapName_ROUTE_19 sMapName_ROUTE_20 sMapName_ROUTE_21 sMapName_ROUTE_22 sMapName_ROUTE_23 sMapName_ROUTE_24 sMapName_ROUTE_25 sMapName_VIRIDIAN_FOREST sMapName_MT__MOON sMapName_S_S__ANNE sMapName_UNDERGROUND_PATH sMapName_UNDERGROUND_PATH_Clone sMapName_DIGLETT_S_CAVE sMapName_VICTORY_ROAD_Clone sMapName_ROCKET_HIDEOUT sMapName_SILPH_CO_ sMapName_POK__MON_MANSION sMapName_SAFARI_ZONE_Clone sMapName_POK__MON_LEAGUE sMapName_ROCK_TUNNEL sMapName_SEAFOAM_ISLANDS sMapName_POK__MON_TOWER sMapName_CERULEAN_CAVE sMapName_POWER_PLANT sMapName_ONE_ISLAND sMapName_TWO_ISLAND sMapName_THREE_ISLAND sMapName_FOUR_ISLAND sMapName_FIVE_ISLAND sMapName_SEVEN_ISLAND sMapName_SIX_ISLAND sMapName_KINDLE_ROAD sMapName_TREASURE_BEACH sMapName_CAPE_BRINK sMapName_BOND_BRIDGE sMapName_THREE_ISLE_PORT sMapName_SEVII_ISLE_6 sMapName_SEVII_ISLE_7 sMapName_SEVII_ISLE_8 sMapName_SEVII_ISLE_9 sMapName_RESORT_GORGEOUS sMapName_WATER_LABYRINTH sMapName_FIVE_ISLE_MEADOW sMapName_MEMORIAL_PILLAR sMapName_OUTCAST_ISLAND sMapName_GREEN_PATH sMapName_WATER_PATH sMapName_RUIN_VALLEY sMapName_TRAINER_TOWER sMapName_CANYON_ENTRANCE sMapName_SEVAULT_CANYON sMapName_TANOBY_RUINS sMapName_SEVII_ISLE_22 sMapName_SEVII_ISLE_23 sMapName_SEVII_ISLE_24 sMapName_NAVEL_ROCK sMapName_MT__EMBER sMapName_BERRY_FOREST sMapName_ICEFALL_CAVE sMapName_ROCKET_WAREHOUSE sMapName_TRAINER_TOWER_Clone sMapName_DOTTED_HOLE sMapName_LOST_CAVE sMapName_PATTERN_BUSH sMapName_ALTERING_CAVE sMapName_TANOBY_CHAMBERS sMapName_THREE_ISLE_PATH sMapName_TANOBY_KEY sMapName_BIRTH_ISLAND sMapName_MONEAN_CHAMBER sMapName_LIPTOO_CHAMBER sMapName_WEEPTH_CHAMBER sMapName_DILFORD_CHAMBER sMapName_SCUFIB_CHAMBER sMapName_RIXY_CHAMBER sMapName_VIAPOIS_CHAMBER sMapName_EMBER_SPA sMapName_SPECIAL_AREA sMapName_AQUA_HIDEOUT sMapName_MAGMA_HIDEOUT sMapName_MIRAGE_TOWER sMapName_FARAWAY_ISLAND sMapName_ARTISAN_CAVE sMapName_MARINE_CAVE sMapName_TERRA_CAVE sMapName_DESERT_UNDERPASS sMapName_TRAINER_HILL gRegionMapEntries sRegionMap_SpecialPlaceLocations sMarineCaveMapSecIds sTerraOrMarineCaveMapSecIds sMarineCaveLocationCoords sMapSecAquaHideoutOld sRegionMapCursorOam sRegionMapCursorAnim1 sRegionMapCursorAnim2 sRegionMapCursorAnimTable sRegionMapCursorSpritePalette sRegionMapCursorSpriteTemplate sRegionMapPlayerIconOam sRegionMapPlayerIconAnim1 sRegionMapPlayerIconAnimTable sMapSecIdsOffMap sRegionMapFramePal sRegionMapFrameGfxLZ sRegionMapFrameTilemapLZ sFlyTargetIcons_Pal sFlyTargetIcons_Gfx sMapHealLocations sEverGrandeCityNames sMultiNameFlyDestinations sFlyMapBgTemplates sFlyMapWindowTemplates sFlyTargetIconsSpritePalette sRedOutlineFlyDestinations sFlyDestIcon_OamData sFlyDestIcon_Anim_8x8CanFly sFlyDestIcon_Anim_16x8CanFly sFlyDestIcon_Anim_8x16CanFly sFlyDestIcon_Anim_8x8CantFly sFlyDestIcon_Anim_16x8CantFly sFlyDestIcon_Anim_8x16CantFly sFlyDestIcon_Anim_RedOutline sFlyDestIcon_Anims sFlyDestIconSpriteTemplate
#[allow(unused_imports)]
use crate::data::region_map::*;

#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sRegionMap: *mut u8 = core::ptr::null_mut();
pub(crate) static mut sFlyMap: *mut u8 = core::ptr::null_mut();
pub(crate) static mut sDrawFlyDestTextWindow: u32 = 0u32;

unsafe extern "C" {
    static mut gDummySpriteAffineAnimTable: u8;
    static mut gMain: u8;
    static mut gMapHeader: u8;
    static mut gPlttBufferFaded: u8;
    static mut gPlttBufferUnfaded: u8;
    static mut gSaveBlock1Ptr: u8;
    static mut gSaveBlock2Ptr: u8;
    static mut gSineTable: u8;
    static mut gSprites: u8;
    static mut gText_Ferry: u8;
    static mut gText_FlyToWhere: u8;
    static mut gText_Hideout: u8;
    static mut gText_SecretBase: u8;
    fn AddTextPrinterParameterized(
        a0: u8,
        a1: u8,
        a2: *mut u8,
        a3: u8,
        a4: u8,
        a5: u8,
        a6: Option<unsafe extern "C" fn(*mut u8, u16)>,
    ) -> u16;
    fn Alloc(a0: u32) -> *mut u8;
    fn AnimateSprites();
    fn BeginNormalPaletteFade(a0: u32, a1: i8, a2: u8, a3: u8, a4: u16) -> u8;
    fn BlendPalettes(a0: u32, a1: u8, a2: u16);
    fn BuildOamBuffer();
    fn CB2_ReturnToFieldWithOpenMenu();
    fn CB2_ReturnToPartyMenuFromFlyMap();
    fn ClearScheduledBgCopiesToVram();
    fn ClearStdWindowAndFrameToTransparent(a0: u8, a1: u8);
    fn CopyWindowToVram(a0: u8, a1: u8);
    fn CpuSet(a0: *mut u8, a1: *mut u8, a2: u32);
    fn CreateSprite(a0: *mut u8, a1: i16, a2: i16, a3: u8) -> u8;
    fn DeactivateAllTextPrinters();
    fn DecompressAndCopyTileDataToVram(a0: u8, a1: *mut u8, a2: u32, a3: u16, a4: u8) -> *mut u8;
    fn DestroySprite(a0: *mut u8);
    fn DoScheduledBgTilemapCopiesToVram();
    fn DrawStdFrameWithCustomTileAndPalette(a0: u8, a1: u8, a2: u16, a3: u8);
    fn FillWindowPixelBuffer(a0: u8, a1: u8);
    fn FlagGet(a0: u16) -> u8;
    fn Free(a0: *mut u8);
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
    fn InitBgsFromTemplates(a0: u8, a1: *mut u8, a2: u8);
    fn InitWindows(a0: *mut u8) -> u16;
    fn LZ77UnCompVram(a0: *mut u32, a1: *mut u8);
    fn LZ77UnCompWram(a0: *mut u32, a1: *mut u8);
    fn LoadOam();
    fn LoadPalette(a0: *mut u8, a1: u16, a2: u16);
    fn LoadSpritePalette(a0: *mut u8) -> u8;
    fn LoadSpriteSheet(a0: *mut u8) -> u16;
    fn LoadUserWindowBorderGfx(a0: u8, a1: u16, a2: u8);
    fn Overworld_GetMapHeaderByGroupAndId(a0: u16, a1: u16) -> *mut u8;
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
    fn SpriteCallbackDummy(a0: *mut u8);
    fn StartSpriteAnim(a0: *mut u8, a1: u8);
    fn StringCopy(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn StringFill(a0: *mut u8, a1: u8, a2: u16) -> *mut u8;
    fn StringLength(a0: *mut u8) -> u16;
    fn TransferPlttBuffer();
    fn UpdatePaletteFade() -> u8;
    fn VarGet(a0: u16) -> u16;
    fn m4aSongNumStart(a0: u16);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn InitRegionMap(regionMap: *mut u8, zoomed: u8) {
    unsafe {
        let mut regionMap = regionMap;
        let mut zoomed = zoomed;
        InitRegionMapData(regionMap, core::ptr::null_mut(), zoomed);
        'l1: loop {
            if !((LoadRegionMapGfx()) != 0) {
                break 'l1;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn InitRegionMapData(regionMap: *mut u8, template: *mut u8, zoomed: u8) {
    unsafe {
        let mut regionMap = regionMap;
        let mut template = template;
        let mut zoomed = zoomed;
        ((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).write(regionMap);
        ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(121))
            .write(0u8);
        ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(120))
            .write(zoomed);
        ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(24)
            .cast::<Option<unsafe extern "C" fn() -> u8>>())
        .write(
            (if ((zoomed) as i32) == 1i32 {
                Some(ProcessRegionMapInput_Zoomed)
            } else {
                Some(ProcessRegionMapInput_Full)
            }),
        );
        if ((template) as usize) != 0usize {
            ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(128))
                .write(((crate::c::bf_read((template).wrapping_add(0), 0, 2, false) as u16) as u8));
            ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(129))
                .write(((crate::c::bf_read((template).wrapping_add(0), 2, 2, false) as u16) as u8));
            ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(130))
                .write(((crate::c::bf_read((template).wrapping_add(0), 4, 5, false) as u16) as u8));
            ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(131))
                .write(1u8);
        } else {
            ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(128))
                .write(2u8);
            ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(129))
                .write(2u8);
            ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(130))
                .write(28u8);
            ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(131))
                .write(0u8);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ShowRegionMapForPokedexAreaScreen(regionMap: *mut u8) {
    unsafe {
        let mut regionMap = regionMap;
        ((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).write(regionMap);
        InitMapBasedOnPlayerLocation();
        ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(116)
            .cast::<u16>())
        .write(
            ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(84)
                .cast::<u16>())
            .read(),
        );
        ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(118)
            .cast::<u16>())
        .write(
            ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(86)
                .cast::<u16>())
            .read(),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn LoadRegionMapGfx() -> u8 {
    unsafe {
        'l1: {
            let __sw1 = ((((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(121))
            .read()) as i32);
            let __matched = __sw1 == 0i32
                || __sw1 == 1i32
                || __sw1 == 2i32
                || __sw1 == 3i32
                || __sw1 == 4i32
                || __sw1 == 5i32
                || __sw1 == 6i32
                || __sw1 == 7i32;
            if __sw1 == 0i32 {
                if (((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(131))
                .read())
                    != 0
                {
                    DecompressAndCopyTileDataToVram(
                        ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(128))
                        .read(),
                        (((&raw const sRegionMapBg_GfxLZ)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<u32>())
                        .cast::<u32>())
                        .cast::<u8>(),
                        0u32,
                        0u16,
                        0u8,
                    );
                } else {
                    LZ77UnCompVram(
                        ((&raw const sRegionMapBg_GfxLZ)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<u32>())
                        .cast::<u32>(),
                        ((100696064i32) as usize as *mut u16).cast::<u8>(),
                    );
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                if (((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(131))
                .read())
                    != 0
                {
                    if !((FreeTempTileDataBuffersIfPossible()) != 0) {
                        DecompressAndCopyTileDataToVram(
                            ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(128))
                            .read(),
                            (((&raw const sRegionMapBg_TilemapLZ)
                                .cast::<u8>()
                                .cast_mut()
                                .cast::<u32>())
                            .cast::<u32>())
                            .cast::<u8>(),
                            0u32,
                            0u16,
                            1u8,
                        );
                    }
                } else {
                    LZ77UnCompVram(
                        ((&raw const sRegionMapBg_TilemapLZ)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<u32>())
                        .cast::<u32>(),
                        ((100720640i32) as usize as *mut u16).cast::<u8>(),
                    );
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                if !((FreeTempTileDataBuffersIfPossible()) != 0) {
                    LoadPalette(
                        (((&raw const sRegionMapBg_Pal)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<u16>())
                        .cast::<u16>())
                        .cast::<u8>(),
                        112u16,
                        96u16,
                    );
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                LZ77UnCompWram(
                    ((&raw const sRegionMapCursorSmallGfxLZ)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<u32>())
                    .cast::<u32>(),
                    ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(388))
                    .cast::<u8>(),
                );
                break 'l1;
            }
            if __sw1 == 4i32 {
                LZ77UnCompWram(
                    ((&raw const sRegionMapCursorLargeGfxLZ)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<u32>())
                    .cast::<u32>(),
                    ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(644))
                    .cast::<u8>(),
                );
                break 'l1;
            }
            if __sw1 == 5i32 {
                InitMapBasedOnPlayerLocation();
                ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(116)
                    .cast::<u16>())
                .write(
                    ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(84)
                        .cast::<u16>())
                    .read(),
                );
                ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(118)
                    .cast::<u16>())
                .write(
                    ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(86)
                        .cast::<u16>())
                    .read(),
                );
                ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read()).cast::<u16>())
                    .write(CorrectSpecialMapSecId_Internal(
                        ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                            .cast::<u16>())
                        .read(),
                    ));
                ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(2))
                    .write(GetMapsecType(
                        ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                            .cast::<u16>())
                        .read(),
                    ));
                GetMapName(
                    ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(4))
                    .cast::<u8>(),
                    ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read()).cast::<u16>())
                        .read(),
                    16u16,
                );
                break 'l1;
            }
            if __sw1 == 6i32 {
                if ((((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(120))
                .read()) as i32)
                    == 0i32
                {
                    CalcZoomScrollParams(0i16, 0i16, 0i16, 0i16, 256u16, 256u16, 0u8);
                } else {
                    ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(92)
                        .cast::<i16>())
                    .write(
                        (((((((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(84)
                            .cast::<u16>())
                        .read()) as i32)
                            .wrapping_mul(8i32))
                        .wrapping_sub(52i32)) as i16),
                    );
                    ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(94)
                        .cast::<i16>())
                    .write(
                        (((((((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(86)
                            .cast::<u16>())
                        .read()) as i32)
                            .wrapping_mul(8i32))
                        .wrapping_sub(68i32)) as i16),
                    );
                    ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(100)
                        .cast::<u16>())
                    .write(
                        ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(84)
                            .cast::<u16>())
                        .read(),
                    );
                    ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(102)
                        .cast::<u16>())
                    .write(
                        ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(86)
                            .cast::<u16>())
                        .read(),
                    );
                    CalcZoomScrollParams(
                        ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(92)
                            .cast::<i16>())
                        .read(),
                        ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(94)
                            .cast::<i16>())
                        .read(),
                        56i16,
                        72i16,
                        128u16,
                        128u16,
                        0u8,
                    );
                }
                break 'l1;
            }
            if __sw1 == 7i32 {
                GetPositionOfCursorWithinMapSec();
                UpdateRegionMapVideoRegs();
                ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(28)
                    .cast::<*mut u8>())
                .write(core::ptr::null_mut());
                ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(32)
                    .cast::<*mut u8>())
                .write(core::ptr::null_mut());
                ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(122)
                    .cast::<i8>())
                .write(0i8);
                ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(126))
                    .write(0u8);
                if (((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(131))
                .read())
                    != 0
                {
                    SetBgAttribute(
                        ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(128))
                        .read(),
                        3u8,
                        2u8,
                    );
                    SetBgAttribute(
                        ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(128))
                        .read(),
                        1u8,
                        ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(129))
                        .read(),
                    );
                    SetBgAttribute(
                        ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(128))
                        .read(),
                        2u8,
                        ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(130))
                        .read(),
                    );
                    SetBgAttribute(
                        ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(128))
                        .read(),
                        6u8,
                        1u8,
                    );
                    SetBgAttribute(
                        ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(128))
                        .read(),
                        4u8,
                        1u8,
                    );
                }
                let __p2 = (((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(121);
                (__p2).write(((__p2).read()).wrapping_add(1));
                return 0u8;
            }
            if !__matched {
                return 0u8;
            }
        }
        let __p3 =
            (((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(121);
        (__p3).write(((__p3).read()).wrapping_add(1));
        return 1u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BlendRegionMap(color: u16, coeff: u32) {
    unsafe {
        let mut color = color;
        let mut coeff = coeff;
        BlendPalettes(896u32, ((coeff) as u8), color);
        'l1: loop {
            'l2: {
                'l3: loop {
                    'l4: {
                        CpuSet(
                            ((((&raw mut gPlttBufferFaded).cast::<u16>()).cast::<u16>())
                                .wrapping_offset(112))
                            .cast::<u8>(),
                            ((((&raw mut gPlttBufferUnfaded).cast::<u16>()).cast::<u16>())
                                .wrapping_offset(112))
                            .cast::<u8>(),
                            (0u32
                                | (crate::c::div_u32(
                                    96u32,
                                    ((crate::c::div_i32(16i32, 8i32)) as u32),
                                ) & 2097151u32)),
                        );
                    }
                    if !((0i32) != 0) {
                        break 'l3;
                    }
                }
            }
            if !((0i32) != 0) {
                break 'l1;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FreeRegionMapIconResources() {
    unsafe {
        if ((((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(28)
            .cast::<*mut u8>())
        .read()) as usize)
            != 0usize
        {
            DestroySprite(
                ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(28)
                    .cast::<*mut u8>())
                .read(),
            );
            FreeSpriteTilesByTag(
                ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(88)
                    .cast::<u16>())
                .read(),
            );
            FreeSpritePaletteByTag(
                ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(90)
                    .cast::<u16>())
                .read(),
            );
        }
        if ((((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(32)
            .cast::<*mut u8>())
        .read()) as usize)
            != 0usize
        {
            DestroySprite(
                ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(32)
                    .cast::<*mut u8>())
                .read(),
            );
            FreeSpriteTilesByTag(
                ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(112)
                    .cast::<u16>())
                .read(),
            );
            FreeSpritePaletteByTag(
                ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(114)
                    .cast::<u16>())
                .read(),
            );
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DoRegionMapInputCallback() -> u8 {
    unsafe {
        return (((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(24)
            .cast::<Option<unsafe extern "C" fn() -> u8>>())
        .read())
        .unwrap_unchecked()();
    }
}
pub(crate) unsafe extern "C" fn ProcessRegionMapInput_Full() -> u8 {
    unsafe {
        let mut input: u8 = 0u8;
        input = 0u8;
        ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(123)
            .cast::<i8>())
        .write(0i8);
        ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(124)
            .cast::<i8>())
        .write(0i8);
        if (((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(44)
            .cast::<u16>())
        .read()) as i32)
            & 64i32)
            != 0)
            && (((((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(86)
                .cast::<u16>())
            .read()) as i32)
                > 2i32)
        {
            ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(124)
                .cast::<i8>())
            .write((-1i8));
            input = 1u8;
        }
        if (((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(44)
            .cast::<u16>())
        .read()) as i32)
            & 128i32)
            != 0)
            && (((((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(86)
                .cast::<u16>())
            .read()) as i32)
                < 16i32)
        {
            ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(124)
                .cast::<i8>())
            .write(1i8);
            input = 1u8;
        }
        if (((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(44)
            .cast::<u16>())
        .read()) as i32)
            & 32i32)
            != 0)
            && (((((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(84)
                .cast::<u16>())
            .read()) as i32)
                > 1i32)
        {
            ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(123)
                .cast::<i8>())
            .write((-1i8));
            input = 1u8;
        }
        if (((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(44)
            .cast::<u16>())
        .read()) as i32)
            & 16i32)
            != 0)
            && (((((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(84)
                .cast::<u16>())
            .read()) as i32)
                < 28i32)
        {
            ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(123)
                .cast::<i8>())
            .write(1i8);
            input = 1u8;
        }
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(46)
            .cast::<u16>())
        .read()) as i32)
            & 1i32)
            != 0
        {
            input = 4u8;
        } else {
            if ((((((&raw mut gMain).cast::<u8>())
                .wrapping_add(46)
                .cast::<u16>())
            .read()) as i32)
                & 2i32)
                != 0
            {
                input = 5u8;
            }
        }
        if ((input) as i32) == 1i32 {
            ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(122)
                .cast::<i8>())
            .write(4i8);
            ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(24)
                .cast::<Option<unsafe extern "C" fn() -> u8>>())
            .write(Some(MoveRegionMapCursor_Full));
        }
        return input;
    }
}
pub(crate) unsafe extern "C" fn MoveRegionMapCursor_Full() -> u8 {
    unsafe {
        let mut mapSecId: u16 = 0u16;
        if ((((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(122)
            .cast::<i8>())
        .read()) as i32)
            != 0i32
        {
            return 2u8;
        }
        if ((((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(123)
            .cast::<i8>())
        .read()) as i32)
            > 0i32
        {
            let __p1 = (((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(84)
                .cast::<u16>();
            (__p1).write(((__p1).read()).wrapping_add(1));
        }
        if ((((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(123)
            .cast::<i8>())
        .read()) as i32)
            < 0i32
        {
            let __p2 = (((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(84)
                .cast::<u16>();
            (__p2).write(((__p2).read()).wrapping_sub(1));
        }
        if ((((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(124)
            .cast::<i8>())
        .read()) as i32)
            > 0i32
        {
            let __p3 = (((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(86)
                .cast::<u16>();
            (__p3).write(((__p3).read()).wrapping_add(1));
        }
        if ((((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(124)
            .cast::<i8>())
        .read()) as i32)
            < 0i32
        {
            let __p4 = (((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(86)
                .cast::<u16>();
            (__p4).write(((__p4).read()).wrapping_sub(1));
        }
        mapSecId = GetMapSecIdAt(
            ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(84)
                .cast::<u16>())
            .read(),
            ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(86)
                .cast::<u16>())
            .read(),
        );
        ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(2))
            .write(GetMapsecType(mapSecId));
        if ((mapSecId) as i32)
            != ((((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read()).cast::<u16>())
                .read()) as i32)
        {
            ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read()).cast::<u16>())
                .write(mapSecId);
            GetMapName(
                ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(4))
                    .cast::<u8>(),
                ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read()).cast::<u16>())
                    .read(),
                16u16,
            );
        }
        GetPositionOfCursorWithinMapSec();
        ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(24)
            .cast::<Option<unsafe extern "C" fn() -> u8>>())
        .write(Some(ProcessRegionMapInput_Full));
        return 3u8;
    }
}
pub(crate) unsafe extern "C" fn ProcessRegionMapInput_Zoomed() -> u8 {
    unsafe {
        let mut input: u8 = 0u8;
        input = 0u8;
        ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(106)
            .cast::<i16>())
        .write(0i16);
        ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(104)
            .cast::<i16>())
        .write(0i16);
        if (((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(44)
            .cast::<u16>())
        .read()) as i32)
            & 64i32)
            != 0)
            && (((((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(94)
                .cast::<i16>())
            .read()) as i32)
                > (-52i32))
        {
            ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(104)
                .cast::<i16>())
            .write((-1i16));
            input = 1u8;
        }
        if (((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(44)
            .cast::<u16>())
        .read()) as i32)
            & 128i32)
            != 0)
            && (((((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(94)
                .cast::<i16>())
            .read()) as i32)
                < 60i32)
        {
            ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(104)
                .cast::<i16>())
            .write(1i16);
            input = 1u8;
        }
        if (((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(44)
            .cast::<u16>())
        .read()) as i32)
            & 32i32)
            != 0)
            && (((((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(92)
                .cast::<i16>())
            .read()) as i32)
                > (-44i32))
        {
            ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(106)
                .cast::<i16>())
            .write((-1i16));
            input = 1u8;
        }
        if (((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(44)
            .cast::<u16>())
        .read()) as i32)
            & 16i32)
            != 0)
            && (((((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(92)
                .cast::<i16>())
            .read()) as i32)
                < 172i32)
        {
            ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(106)
                .cast::<i16>())
            .write(1i16);
            input = 1u8;
        }
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(46)
            .cast::<u16>())
        .read()) as i32)
            & 1i32)
            != 0
        {
            input = 4u8;
        }
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(46)
            .cast::<u16>())
        .read()) as i32)
            & 2i32)
            != 0
        {
            input = 5u8;
        }
        if ((input) as i32) == 1i32 {
            ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(24)
                .cast::<Option<unsafe extern "C" fn() -> u8>>())
            .write(Some(MoveRegionMapCursor_Zoomed));
            ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(108)
                .cast::<u16>())
            .write(0u16);
        }
        return input;
    }
}
pub(crate) unsafe extern "C" fn MoveRegionMapCursor_Zoomed() -> u8 {
    unsafe {
        let mut x: u16 = 0u16;
        let mut y: u16 = 0u16;
        let mut mapSecId: u16 = 0u16;
        let __p1 = (((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(94)
            .cast::<i16>();
        (__p1).write(
            (((((__p1).read()) as i32).wrapping_add(
                ((((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(104)
                    .cast::<i16>())
                .read()) as i32),
            )) as i16),
        );
        let __p2 = (((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(92)
            .cast::<i16>();
        (__p2).write(
            (((((__p2).read()) as i32).wrapping_add(
                ((((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(106)
                    .cast::<i16>())
                .read()) as i32),
            )) as i16),
        );
        RegionMap_SetBG2XAndBG2Y(
            ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(92)
                .cast::<i16>())
            .read(),
            ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(94)
                .cast::<i16>())
            .read(),
        );
        let __p3 = (((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(108)
            .cast::<u16>();
        (__p3).write(((__p3).read()).wrapping_add(1));
        if ((((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(108)
            .cast::<u16>())
        .read()) as i32)
            == 8i32
        {
            x = (((crate::c::div_i32(
                ((((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(92)
                    .cast::<i16>())
                .read()) as i32)
                    .wrapping_add(44i32),
                8i32,
            ))
            .wrapping_add(1i32)) as u16);
            y = (((crate::c::div_i32(
                ((((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(94)
                    .cast::<i16>())
                .read()) as i32)
                    .wrapping_add(52i32),
                8i32,
            ))
            .wrapping_add(2i32)) as u16);
            if (((x) as i32)
                != ((((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(100)
                    .cast::<u16>())
                .read()) as i32))
                || (((y) as i32)
                    != ((((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(102)
                        .cast::<u16>())
                    .read()) as i32))
            {
                ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(100)
                    .cast::<u16>())
                .write(x);
                ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(102)
                    .cast::<u16>())
                .write(y);
                mapSecId = GetMapSecIdAt(x, y);
                ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(2))
                    .write(GetMapsecType(mapSecId));
                if ((mapSecId) as i32)
                    != ((((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                        .cast::<u16>())
                    .read()) as i32)
                {
                    ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read()).cast::<u16>())
                        .write(mapSecId);
                    GetMapName(
                        ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(4))
                        .cast::<u8>(),
                        ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                            .cast::<u16>())
                        .read(),
                        16u16,
                    );
                }
                GetPositionOfCursorWithinMapSec();
            }
            ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(108)
                .cast::<u16>())
            .write(0u16);
            ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(24)
                .cast::<Option<unsafe extern "C" fn() -> u8>>())
            .write(Some(ProcessRegionMapInput_Zoomed));
            return 3u8;
        }
        return 2u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetRegionMapDataForZoom() {
    unsafe {
        if ((((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(120))
            .read()) as i32)
            == 0i32
        {
            ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(94)
                .cast::<i16>())
            .write(0i16);
            ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(92)
                .cast::<i16>())
            .write(0i16);
            ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(64)
                .cast::<i32>())
            .write(0i32);
            ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(60)
                .cast::<i32>())
            .write(0i32);
            ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(96)
                .cast::<i16>())
            .write(
                (((((((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(84)
                    .cast::<u16>())
                .read()) as i32)
                    .wrapping_mul(8i32))
                .wrapping_sub(52i32)) as i16),
            );
            ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(98)
                .cast::<i16>())
            .write(
                (((((((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(86)
                    .cast::<u16>())
                .read()) as i32)
                    .wrapping_mul(8i32))
                .wrapping_sub(68i32)) as i16),
            );
            ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(68)
                .cast::<i32>())
            .write(crate::c::div_i32(
                (((((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(96)
                    .cast::<i16>())
                .read()) as i32)
                    << 8),
                16i32,
            ));
            ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(72)
                .cast::<i32>())
            .write(crate::c::div_i32(
                (((((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(98)
                    .cast::<i16>())
                .read()) as i32)
                    << 8),
                16i32,
            ));
            ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(100)
                .cast::<u16>())
            .write(
                ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(84)
                    .cast::<u16>())
                .read(),
            );
            ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(102)
                .cast::<u16>())
            .write(
                ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(86)
                    .cast::<u16>())
                .read(),
            );
            ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(76)
                .cast::<i32>())
            .write(65536i32);
            ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(80)
                .cast::<i32>())
            .write((-2048i32));
        } else {
            ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(60)
                .cast::<i32>())
            .write(
                ((((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(92)
                    .cast::<i16>())
                .read()) as i32)
                    .wrapping_mul(256i32),
            );
            ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(64)
                .cast::<i32>())
            .write(
                ((((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(94)
                    .cast::<i16>())
                .read()) as i32)
                    .wrapping_mul(256i32),
            );
            ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(96)
                .cast::<i16>())
            .write(0i16);
            ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(98)
                .cast::<i16>())
            .write(0i16);
            ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(68)
                .cast::<i32>())
            .write(
                (crate::c::div_i32(
                    ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(60)
                        .cast::<i32>())
                    .read(),
                    16i32,
                ))
                .wrapping_neg(),
            );
            ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(72)
                .cast::<i32>())
            .write(
                (crate::c::div_i32(
                    ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(64)
                        .cast::<i32>())
                    .read(),
                    16i32,
                ))
                .wrapping_neg(),
            );
            ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(84)
                .cast::<u16>())
            .write(
                ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(100)
                    .cast::<u16>())
                .read(),
            );
            ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(86)
                .cast::<u16>())
            .write(
                ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(102)
                    .cast::<u16>())
                .read(),
            );
            ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(76)
                .cast::<i32>())
            .write(32768i32);
            ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(80)
                .cast::<i32>())
            .write(2048i32);
        }
        ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(110)
            .cast::<u16>())
        .write(0u16);
        FreeRegionMapCursorSprite();
        HideRegionMapPlayerIcon();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn UpdateRegionMapZoom() -> u8 {
    unsafe {
        let mut retVal: u8 = 0u8;
        if ((((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(110)
            .cast::<u16>())
        .read()) as i32)
            >= 16i32
        {
            return 0u8;
        }
        let __p1 = (((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(110)
            .cast::<u16>();
        (__p1).write(((__p1).read()).wrapping_add(1));
        if ((((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(110)
            .cast::<u16>())
        .read()) as i32)
            == 16i32
        {
            ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(68)
                .cast::<i32>())
            .write(0i32);
            ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(72)
                .cast::<i32>())
            .write(0i32);
            ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(92)
                .cast::<i16>())
            .write(
                ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(96)
                    .cast::<i16>())
                .read(),
            );
            ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(94)
                .cast::<i16>())
            .write(
                ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(98)
                    .cast::<i16>())
                .read(),
            );
            ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(76)
                .cast::<i32>())
            .write(
                (if ((((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(120))
                .read()) as i32)
                    == 0i32
                {
                    32768i32
                } else {
                    65536i32
                }),
            );
            ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(120))
                .write(
                    ((!((((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(120))
                    .read())
                        != 0)) as u8),
                );
            ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(24)
                .cast::<Option<unsafe extern "C" fn() -> u8>>())
            .write(
                (if ((((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(120))
                .read()) as i32)
                    == 0i32
                {
                    Some(ProcessRegionMapInput_Full)
                } else {
                    Some(ProcessRegionMapInput_Zoomed)
                }),
            );
            CreateRegionMapCursor(
                ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(88)
                    .cast::<u16>())
                .read(),
                ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(90)
                    .cast::<u16>())
                .read(),
            );
            UnhideRegionMapPlayerIcon();
            retVal = 0u8;
        } else {
            let __p2 = (((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(60)
                .cast::<i32>();
            (__p2).write(
                ((__p2).read()).wrapping_add(
                    ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(68)
                        .cast::<i32>())
                    .read(),
                ),
            );
            let __p3 = (((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(64)
                .cast::<i32>();
            (__p3).write(
                ((__p3).read()).wrapping_add(
                    ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(72)
                        .cast::<i32>())
                    .read(),
                ),
            );
            ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(92)
                .cast::<i16>())
            .write(
                ((((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(60)
                    .cast::<i32>())
                .read()
                    >> 8) as i16),
            );
            ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(94)
                .cast::<i16>())
            .write(
                ((((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(64)
                    .cast::<i32>())
                .read()
                    >> 8) as i16),
            );
            let __p4 = (((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(76)
                .cast::<i32>();
            (__p4).write(
                ((__p4).read()).wrapping_add(
                    ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(80)
                        .cast::<i32>())
                    .read(),
                ),
            );
            if ((((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(68)
                .cast::<i32>())
            .read()
                < 0i32)
                && (((((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(92)
                    .cast::<i16>())
                .read()) as i32)
                    < ((((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(96)
                        .cast::<i16>())
                    .read()) as i32)))
                || ((((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(68)
                    .cast::<i32>())
                .read()
                    > 0i32)
                    && (((((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(92)
                        .cast::<i16>())
                    .read()) as i32)
                        > ((((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(96)
                            .cast::<i16>())
                        .read()) as i32)))
            {
                ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(92)
                    .cast::<i16>())
                .write(
                    ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(96)
                        .cast::<i16>())
                    .read(),
                );
                ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(68)
                    .cast::<i32>())
                .write(0i32);
            }
            if ((((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(72)
                .cast::<i32>())
            .read()
                < 0i32)
                && (((((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(94)
                    .cast::<i16>())
                .read()) as i32)
                    < ((((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(98)
                        .cast::<i16>())
                    .read()) as i32)))
                || ((((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(72)
                    .cast::<i32>())
                .read()
                    > 0i32)
                    && (((((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(94)
                        .cast::<i16>())
                    .read()) as i32)
                        > ((((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(98)
                            .cast::<i16>())
                        .read()) as i32)))
            {
                ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(94)
                    .cast::<i16>())
                .write(
                    ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(98)
                        .cast::<i16>())
                    .read(),
                );
                ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(72)
                    .cast::<i32>())
                .write(0i32);
            }
            if ((((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(120))
            .read()) as i32)
                == 0i32
            {
                if ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(76)
                    .cast::<i32>())
                .read()
                    < 32768i32
                {
                    ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(76)
                        .cast::<i32>())
                    .write(32768i32);
                    ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(80)
                        .cast::<i32>())
                    .write(0i32);
                }
            } else {
                if ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(76)
                    .cast::<i32>())
                .read()
                    > 65536i32
                {
                    ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(76)
                        .cast::<i32>())
                    .write(65536i32);
                    ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(80)
                        .cast::<i32>())
                    .write(0i32);
                }
            }
            retVal = 1u8;
        }
        CalcZoomScrollParams(
            ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(92)
                .cast::<i16>())
            .read(),
            ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(94)
                .cast::<i16>())
            .read(),
            56i16,
            72i16,
            ((((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(76)
                .cast::<i32>())
            .read()
                >> 8) as u16),
            ((((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(76)
                .cast::<i32>())
            .read()
                >> 8) as u16),
            0u8,
        );
        return retVal;
    }
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
    unsafe {
        let mut scrollX = scrollX;
        let mut scrollY = scrollY;
        let mut c = c;
        let mut d = d;
        let mut e = e;
        let mut f = f;
        let mut rotation = rotation;
        let mut var1: i32 = 0i32;
        let mut var2: i32 = 0i32;
        let mut var3: i32 = 0i32;
        let mut var4: i32 = 0i32;
        ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(44)
            .cast::<u32>())
        .write(
            ((((e) as i32).wrapping_mul(
                ((((((&raw mut gSineTable).cast::<i16>()).cast::<i16>())
                    .wrapping_offset((((rotation) as i32).wrapping_add(64i32)) as isize))
                .read()) as i32),
            ) >> 8) as u32),
        );
        ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(48)
            .cast::<u32>())
        .write(
            ((((e) as i32).wrapping_mul(
                ((((((&raw mut gSineTable).cast::<i16>()).cast::<i16>())
                    .wrapping_offset(((rotation) as i32) as isize))
                .read()) as i32)
                    .wrapping_neg(),
            ) >> 8) as u32),
        );
        ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(52)
            .cast::<u32>())
        .write(
            ((((f) as i32).wrapping_mul(
                ((((((&raw mut gSineTable).cast::<i16>()).cast::<i16>())
                    .wrapping_offset(((rotation) as i32) as isize))
                .read()) as i32),
            ) >> 8) as u32),
        );
        ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(56)
            .cast::<u32>())
        .write(
            ((((f) as i32).wrapping_mul(
                ((((((&raw mut gSineTable).cast::<i16>()).cast::<i16>())
                    .wrapping_offset((((rotation) as i32).wrapping_add(64i32)) as isize))
                .read()) as i32),
            ) >> 8) as u32),
        );
        var1 = (((scrollX) as i32) << 8).wrapping_add((((c) as i32) << 8));
        var2 = (((((d) as u32).wrapping_mul(
            ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(52)
                .cast::<u32>())
            .read(),
        ))
        .wrapping_add(
            (((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(44)
                .cast::<u32>())
            .read())
            .wrapping_mul(((c) as u32)),
        )) as i32);
        ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(36)
            .cast::<i32>())
        .write((var1).wrapping_sub(var2));
        var3 = (((scrollY) as i32) << 8).wrapping_add((((d) as i32) << 8));
        var4 = ((((((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(56)
            .cast::<u32>())
        .read())
        .wrapping_mul(((d) as u32)))
        .wrapping_add(
            (((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(48)
                .cast::<u32>())
            .read())
            .wrapping_mul(((c) as u32)),
        )) as i32);
        ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(40)
            .cast::<i32>())
        .write((var3).wrapping_sub(var4));
        ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(125))
            .write(1u8);
    }
}
pub(crate) unsafe extern "C" fn RegionMap_SetBG2XAndBG2Y(x: i16, y: i16) {
    unsafe {
        let mut x = x;
        let mut y = y;
        ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(36)
            .cast::<i32>())
        .write((((x) as i32) << 8).wrapping_add(7168i32));
        ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(40)
            .cast::<i32>())
        .write((((y) as i32) << 8).wrapping_add(9216i32));
        ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(125))
            .write(1u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn UpdateRegionMapVideoRegs() {
    unsafe {
        if (((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(125))
            .read())
            != 0
        {
            SetGpuReg(
                32u8,
                ((((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(44)
                    .cast::<u32>())
                .read()) as u16),
            );
            SetGpuReg(
                34u8,
                ((((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(52)
                    .cast::<u32>())
                .read()) as u16),
            );
            SetGpuReg(
                36u8,
                ((((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(48)
                    .cast::<u32>())
                .read()) as u16),
            );
            SetGpuReg(
                38u8,
                ((((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(56)
                    .cast::<u32>())
                .read()) as u16),
            );
            SetGpuReg(
                40u8,
                ((((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(36)
                    .cast::<i32>())
                .read()) as u16),
            );
            SetGpuReg(
                42u8,
                ((((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(36)
                    .cast::<i32>())
                .read()
                    >> 16) as u16),
            );
            SetGpuReg(
                44u8,
                ((((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(40)
                    .cast::<i32>())
                .read()) as u16),
            );
            SetGpuReg(
                46u8,
                ((((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(40)
                    .cast::<i32>())
                .read()
                    >> 16) as u16),
            );
            ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(125))
                .write(0u8);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PokedexAreaScreen_UpdateRegionMapVariablesAndVideoRegs(x: i16, y: i16) {
    unsafe {
        let mut x = x;
        let mut y = y;
        CalcZoomScrollParams(x, y, 56i16, 72i16, 256u16, 256u16, 0u8);
        UpdateRegionMapVideoRegs();
        if ((((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(32)
            .cast::<*mut u8>())
        .read()) as usize)
            != 0usize
        {
            ((((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(32)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(36)
            .cast::<i16>())
            .write(((((x) as i32).wrapping_neg()) as i16));
            ((((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(32)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(38)
            .cast::<i16>())
            .write(((((y) as i32).wrapping_neg()) as i16));
        }
    }
}
pub(crate) unsafe extern "C" fn GetMapSecIdAt(x: u16, y: u16) -> u16 {
    unsafe {
        let mut x = x;
        let mut y = y;
        if (((((y) as i32) < 2i32) || (((y) as i32) > 16i32)) || (((x) as i32) < 1i32))
            || (((x) as i32) > 28i32)
        {
            return 213u16;
        }
        y = ((((y) as i32).wrapping_sub(2i32)) as u16);
        x = ((((x) as i32).wrapping_sub(1i32)) as u16);
        return ((((((((&raw const sRegionMap_MapSectionLayout)
            .cast::<u8>()
            .cast_mut())
        .cast::<u8>())
        .wrapping_offset(((y) as i32) as isize * 28))
        .cast::<u8>())
        .wrapping_offset(((x) as i32) as isize))
        .read()) as u16);
    }
}
pub(crate) unsafe extern "C" fn InitMapBasedOnPlayerLocation() {
    unsafe {
        let mut mapHeader: *mut u8 = core::ptr::null_mut();
        let mut mapWidth: u16 = 0u16;
        let mut mapHeight: u16 = 0u16;
        let mut x: u16 = 0u16;
        let mut y: u16 = 0u16;
        let mut dimensionScale: u16 = 0u16;
        let mut xOnMap: u16 = 0u16;
        let mut warp: *mut u8 = core::ptr::null_mut();
        if ((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4))
            .cast::<i8>())
        .read()) as i32)
            == 25i32)
            && ((((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4))
                .wrapping_add(1)
                .cast::<i8>())
            .read()) as i32)
                == 41i32)
                || ((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4))
                    .wrapping_add(1)
                    .cast::<i8>())
                .read()) as i32)
                    == 42i32))
                || ((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4))
                    .wrapping_add(1)
                    .cast::<i8>())
                .read()) as i32)
                    == 43i32))
        {
            RegionMap_InitializeStateBasedOnSSTidalLocation();
            return;
        }
        'l1: {
            let __sw1 = ((GetMapTypeByGroupAndId(
                (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4))
                    .cast::<i8>())
                .read(),
                (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4))
                    .wrapping_add(1)
                    .cast::<i8>())
                .read(),
            )) as i32);
            let __matched = __sw1 == 1i32
                || __sw1 == 2i32
                || __sw1 == 3i32
                || __sw1 == 5i32
                || __sw1 == 6i32
                || __sw1 == 4i32
                || __sw1 == 7i32
                || __sw1 == 9i32
                || __sw1 == 8i32;
            if __sw1 == 1i32
                || __sw1 == 2i32
                || __sw1 == 3i32
                || __sw1 == 5i32
                || __sw1 == 6i32
                || !__matched
            {
                ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read()).cast::<u16>())
                    .write(
                        (((((&raw mut gMapHeader).cast::<u8>()).wrapping_add(20)).read()) as u16),
                    );
                ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(127))
                    .write(0u8);
                mapWidth = (((((((&raw mut gMapHeader).cast::<u8>()).cast::<*mut u8>()).read())
                    .cast::<i32>())
                .read()) as u16);
                mapHeight = (((((((&raw mut gMapHeader).cast::<u8>()).cast::<*mut u8>()).read())
                    .wrapping_add(4)
                    .cast::<i32>())
                .read()) as u16);
                x = ((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).cast::<i16>()).read())
                    as u16);
                y = ((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                    .wrapping_add(2)
                    .cast::<i16>())
                .read()) as u16);
                if (((((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                    .cast::<u16>())
                .read()) as i32)
                    == 69i32)
                    || (((((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                        .cast::<u16>())
                    .read()) as i32)
                        == 204i32)
                {
                    ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(127))
                    .write(1u8);
                }
                break 'l1;
            }
            if __sw1 == 4i32 || __sw1 == 7i32 {
                if (crate::c::bf_read(
                    ((&raw mut gMapHeader).cast::<u8>()).wrapping_add(26),
                    1,
                    1,
                    false,
                ) as u8)
                    != 0
                {
                    mapHeader = Overworld_GetMapHeaderByGroupAndId(
                        (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(36))
                        .cast::<i8>())
                        .read()) as u16),
                        (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(36))
                        .wrapping_add(1)
                        .cast::<i8>())
                        .read()) as u16),
                    );
                    ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read()).cast::<u16>())
                        .write(((((mapHeader).wrapping_add(20)).read()) as u16));
                    ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(127))
                    .write(1u8);
                    mapWidth =
                        ((((((mapHeader).cast::<*mut u8>()).read()).cast::<i32>()).read()) as u16);
                    mapHeight = ((((((mapHeader).cast::<*mut u8>()).read())
                        .wrapping_add(4)
                        .cast::<i32>())
                    .read()) as u16);
                    x = (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(36))
                    .wrapping_add(4)
                    .cast::<i16>())
                    .read()) as u16);
                    y = (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(36))
                    .wrapping_add(6)
                    .cast::<i16>())
                    .read()) as u16);
                } else {
                    ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read()).cast::<u16>())
                        .write(
                            (((((&raw mut gMapHeader).cast::<u8>()).wrapping_add(20)).read())
                                as u16),
                        );
                    ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(127))
                    .write(1u8);
                    mapWidth = 1u16;
                    mapHeight = 1u16;
                    x = 1u16;
                    y = 1u16;
                }
                break 'l1;
            }
            if __sw1 == 9i32 {
                mapHeader = Overworld_GetMapHeaderByGroupAndId(
                    (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(20))
                        .cast::<i8>())
                    .read()) as u16),
                    (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(20))
                        .wrapping_add(1)
                        .cast::<i8>())
                    .read()) as u16),
                );
                ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read()).cast::<u16>())
                    .write(((((mapHeader).wrapping_add(20)).read()) as u16));
                ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(127))
                    .write(1u8);
                mapWidth =
                    ((((((mapHeader).cast::<*mut u8>()).read()).cast::<i32>()).read()) as u16);
                mapHeight = ((((((mapHeader).cast::<*mut u8>()).read())
                    .wrapping_add(4)
                    .cast::<i32>())
                .read()) as u16);
                x = (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(20))
                    .wrapping_add(4)
                    .cast::<i16>())
                .read()) as u16);
                y = (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(20))
                    .wrapping_add(6)
                    .cast::<i16>())
                .read()) as u16);
                break 'l1;
            }
            if __sw1 == 8i32 {
                ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read()).cast::<u16>())
                    .write(
                        (((((&raw mut gMapHeader).cast::<u8>()).wrapping_add(20)).read()) as u16),
                    );
                if ((((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                    .cast::<u16>())
                .read()) as i32)
                    != 87i32
                {
                    warp = (((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(36);
                    mapHeader = Overworld_GetMapHeaderByGroupAndId(
                        ((((warp).cast::<i8>()).read()) as u16),
                        ((((warp).wrapping_add(1).cast::<i8>()).read()) as u16),
                    );
                } else {
                    warp = (((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(20);
                    mapHeader = Overworld_GetMapHeaderByGroupAndId(
                        ((((warp).cast::<i8>()).read()) as u16),
                        ((((warp).wrapping_add(1).cast::<i8>()).read()) as u16),
                    );
                    ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read()).cast::<u16>())
                        .write(((((mapHeader).wrapping_add(20)).read()) as u16));
                }
                if (IsPlayerInAquaHideout(
                    ((((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                        .cast::<u16>())
                    .read()) as u8),
                )) != 0
                {
                    ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(127))
                    .write(1u8);
                } else {
                    ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(127))
                    .write(0u8);
                }
                mapWidth =
                    ((((((mapHeader).cast::<*mut u8>()).read()).cast::<i32>()).read()) as u16);
                mapHeight = ((((((mapHeader).cast::<*mut u8>()).read())
                    .wrapping_add(4)
                    .cast::<i32>())
                .read()) as u16);
                x = ((((warp).wrapping_add(4).cast::<i16>()).read()) as u16);
                y = ((((warp).wrapping_add(6).cast::<i16>()).read()) as u16);
                break 'l1;
            }
        }
        xOnMap = x;
        dimensionScale = ((crate::c::div_i32(
            ((mapWidth) as i32),
            (((((((&raw const gRegionMapEntries).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(
                    ((((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                        .cast::<u16>())
                    .read()) as i32) as isize
                        * 8,
                ))
            .wrapping_add(2))
            .read()) as i32),
        )) as u16);
        if ((dimensionScale) as i32) == 0i32 {
            dimensionScale = 1u16;
        }
        x = ((crate::c::div_i32(((x) as i32), ((dimensionScale) as i32))) as u16);
        if ((x) as i32)
            >= (((((((&raw const gRegionMapEntries).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(
                    ((((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                        .cast::<u16>())
                    .read()) as i32) as isize
                        * 8,
                ))
            .wrapping_add(2))
            .read()) as i32)
        {
            x = (((((((((&raw const gRegionMapEntries).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(
                    ((((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                        .cast::<u16>())
                    .read()) as i32) as isize
                        * 8,
                ))
            .wrapping_add(2))
            .read()) as i32)
                .wrapping_sub(1i32)) as u16);
        }
        dimensionScale = ((crate::c::div_i32(
            ((mapHeight) as i32),
            (((((((&raw const gRegionMapEntries).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(
                    ((((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                        .cast::<u16>())
                    .read()) as i32) as isize
                        * 8,
                ))
            .wrapping_add(3))
            .read()) as i32),
        )) as u16);
        if ((dimensionScale) as i32) == 0i32 {
            dimensionScale = 1u16;
        }
        y = ((crate::c::div_i32(((y) as i32), ((dimensionScale) as i32))) as u16);
        if ((y) as i32)
            >= (((((((&raw const gRegionMapEntries).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(
                    ((((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                        .cast::<u16>())
                    .read()) as i32) as isize
                        * 8,
                ))
            .wrapping_add(3))
            .read()) as i32)
        {
            y = (((((((((&raw const gRegionMapEntries).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(
                    ((((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                        .cast::<u16>())
                    .read()) as i32) as isize
                        * 8,
                ))
            .wrapping_add(3))
            .read()) as i32)
                .wrapping_sub(1i32)) as u16);
        }
        'l2: {
            let __sw2 = ((((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                .cast::<u16>())
            .read()) as i32);
            if __sw2 == 29i32 {
                if ((y) as i32) != 0i32 {
                    x = 0u16;
                }
                break 'l2;
            }
            if __sw2 == 41i32 || __sw2 == 51i32 {
                x = 0u16;
                if ((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).cast::<i16>()).read())
                    as i32)
                    > 32i32
                {
                    x = (x).wrapping_add(1);
                }
                if ((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).cast::<i16>()).read())
                    as i32)
                    > 51i32
                {
                    x = (x).wrapping_add(1);
                }
                y = 0u16;
                if ((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                    .wrapping_add(2)
                    .cast::<i16>())
                .read()) as i32)
                    > 37i32
                {
                    y = (y).wrapping_add(1);
                }
                if ((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                    .wrapping_add(2)
                    .cast::<i16>())
                .read()) as i32)
                    > 56i32
                {
                    y = (y).wrapping_add(1);
                }
                break 'l2;
            }
            if __sw2 == 36i32 {
                x = 0u16;
                if ((xOnMap) as i32) > 14i32 {
                    x = (x).wrapping_add(1);
                }
                if ((xOnMap) as i32) > 28i32 {
                    x = (x).wrapping_add(1);
                }
                if ((xOnMap) as i32) > 54i32 {
                    x = (x).wrapping_add(1);
                }
                break 'l2;
            }
            if __sw2 == 204i32 {
                GetMarineCaveCoords(
                    (((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(84)
                        .cast::<u16>(),
                    (((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(86)
                        .cast::<u16>(),
                );
                return;
            }
        }
        ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(84)
            .cast::<u16>())
        .write(
            (((((((((&raw const gRegionMapEntries).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(
                    ((((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                        .cast::<u16>())
                    .read()) as i32) as isize
                        * 8,
                ))
            .read()) as i32)
                .wrapping_add(((x) as i32)))
            .wrapping_add(1i32)) as u16),
        );
        ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(86)
            .cast::<u16>())
        .write(
            ((((((((((&raw const gRegionMapEntries).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(
                    ((((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                        .cast::<u16>())
                    .read()) as i32) as isize
                        * 8,
                ))
            .wrapping_add(1))
            .read()) as i32)
                .wrapping_add(((y) as i32)))
            .wrapping_add(2i32)) as u16),
        );
    }
}
pub(crate) unsafe extern "C" fn RegionMap_InitializeStateBasedOnSSTidalLocation() {
    unsafe {
        let mut y: u16 = 0u16;
        let mut x: u16 = 0u16;
        let mut mapGroup: u8 = 0u8;
        let mut mapNum: u8 = 0u8;
        let mut dimensionScale: u16 = 0u16;
        let mut xOnMap: i16 = 0i16;
        let mut yOnMap: i16 = 0i16;
        let mut mapHeader: *mut u8 = core::ptr::null_mut();
        y = 0u16;
        x = 0u16;
        'l1: {
            let __sw1 = ((GetSSTidalLocation(
                (&raw mut mapGroup).cast::<i8>(),
                (&raw mut mapNum).cast::<i8>(),
                &raw mut xOnMap,
                &raw mut yOnMap,
            )) as i32);
            let __matched =
                __sw1 == 1i32 || __sw1 == 2i32 || __sw1 == 3i32 || __sw1 == 4i32 || __sw1 == 0i32;
            if __sw1 == 1i32 {
                ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read()).cast::<u16>())
                    .write(8u16);
                break 'l1;
            }
            if __sw1 == 2i32 {
                ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read()).cast::<u16>())
                    .write(12u16);
                break 'l1;
            }
            if __sw1 == 3i32 {
                ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read()).cast::<u16>())
                    .write(39u16);
                break 'l1;
            }
            if __sw1 == 4i32 {
                ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read()).cast::<u16>())
                    .write(46u16);
                break 'l1;
            }
            if __sw1 == 0i32 || !__matched {
                mapHeader =
                    Overworld_GetMapHeaderByGroupAndId(((mapGroup) as u16), ((mapNum) as u16));
                ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read()).cast::<u16>())
                    .write(((((mapHeader).wrapping_add(20)).read()) as u16));
                dimensionScale = ((crate::c::div_i32(
                    ((((mapHeader).cast::<*mut u8>()).read()).cast::<i32>()).read(),
                    (((((((&raw const gRegionMapEntries).cast::<u8>().cast_mut()).cast::<u8>())
                        .wrapping_offset(
                            ((((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                                .cast::<u16>())
                            .read()) as i32) as isize
                                * 8,
                        ))
                    .wrapping_add(2))
                    .read()) as i32),
                )) as u16);
                if ((dimensionScale) as i32) == 0i32 {
                    dimensionScale = 1u16;
                }
                x = ((crate::c::div_i32(((xOnMap) as i32), ((dimensionScale) as i32))) as u16);
                if ((x) as i32)
                    >= (((((((&raw const gRegionMapEntries).cast::<u8>().cast_mut())
                        .cast::<u8>())
                    .wrapping_offset(
                        ((((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                            .cast::<u16>())
                        .read()) as i32) as isize
                            * 8,
                    ))
                    .wrapping_add(2))
                    .read()) as i32)
                {
                    x = (((((((((&raw const gRegionMapEntries).cast::<u8>().cast_mut())
                        .cast::<u8>())
                    .wrapping_offset(
                        ((((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                            .cast::<u16>())
                        .read()) as i32) as isize
                            * 8,
                    ))
                    .wrapping_add(2))
                    .read()) as i32)
                        .wrapping_sub(1i32)) as u16);
                }
                dimensionScale = ((crate::c::div_i32(
                    ((((mapHeader).cast::<*mut u8>()).read())
                        .wrapping_add(4)
                        .cast::<i32>())
                    .read(),
                    (((((((&raw const gRegionMapEntries).cast::<u8>().cast_mut()).cast::<u8>())
                        .wrapping_offset(
                            ((((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                                .cast::<u16>())
                            .read()) as i32) as isize
                                * 8,
                        ))
                    .wrapping_add(3))
                    .read()) as i32),
                )) as u16);
                if ((dimensionScale) as i32) == 0i32 {
                    dimensionScale = 1u16;
                }
                y = ((crate::c::div_i32(((yOnMap) as i32), ((dimensionScale) as i32))) as u16);
                if ((y) as i32)
                    >= (((((((&raw const gRegionMapEntries).cast::<u8>().cast_mut())
                        .cast::<u8>())
                    .wrapping_offset(
                        ((((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                            .cast::<u16>())
                        .read()) as i32) as isize
                            * 8,
                    ))
                    .wrapping_add(3))
                    .read()) as i32)
                {
                    y = (((((((((&raw const gRegionMapEntries).cast::<u8>().cast_mut())
                        .cast::<u8>())
                    .wrapping_offset(
                        ((((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                            .cast::<u16>())
                        .read()) as i32) as isize
                            * 8,
                    ))
                    .wrapping_add(3))
                    .read()) as i32)
                        .wrapping_sub(1i32)) as u16);
                }
                break 'l1;
            }
        }
        ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(127))
            .write(0u8);
        ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(84)
            .cast::<u16>())
        .write(
            (((((((((&raw const gRegionMapEntries).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(
                    ((((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                        .cast::<u16>())
                    .read()) as i32) as isize
                        * 8,
                ))
            .read()) as i32)
                .wrapping_add(((x) as i32)))
            .wrapping_add(1i32)) as u16),
        );
        ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(86)
            .cast::<u16>())
        .write(
            ((((((((((&raw const gRegionMapEntries).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(
                    ((((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                        .cast::<u16>())
                    .read()) as i32) as isize
                        * 8,
                ))
            .wrapping_add(1))
            .read()) as i32)
                .wrapping_add(((y) as i32)))
            .wrapping_add(2i32)) as u16),
        );
    }
}
pub(crate) unsafe extern "C" fn GetMapsecType(mapSecId: u16) -> u8 {
    unsafe {
        let mut mapSecId = mapSecId;
        'l1: {
            let __sw1 = ((mapSecId) as i32);
            let __matched = __sw1 == 213i32
                || __sw1 == 0i32
                || __sw1 == 1i32
                || __sw1 == 2i32
                || __sw1 == 3i32
                || __sw1 == 4i32
                || __sw1 == 5i32
                || __sw1 == 6i32
                || __sw1 == 7i32
                || __sw1 == 8i32
                || __sw1 == 9i32
                || __sw1 == 10i32
                || __sw1 == 11i32
                || __sw1 == 12i32
                || __sw1 == 13i32
                || __sw1 == 14i32
                || __sw1 == 15i32
                || __sw1 == 58i32
                || __sw1 == 73i32;
            if __sw1 == 213i32 {
                return 0u8;
            }
            if __sw1 == 0i32 {
                return ((if (FlagGet(2159u16)) != 0 { 2i32 } else { 3i32 }) as u8);
            }
            if __sw1 == 1i32 {
                return ((if (FlagGet(2160u16)) != 0 { 2i32 } else { 3i32 }) as u8);
            }
            if __sw1 == 2i32 {
                return ((if (FlagGet(2161u16)) != 0 { 2i32 } else { 3i32 }) as u8);
            }
            if __sw1 == 3i32 {
                return ((if (FlagGet(2162u16)) != 0 { 2i32 } else { 3i32 }) as u8);
            }
            if __sw1 == 4i32 {
                return ((if (FlagGet(2163u16)) != 0 { 2i32 } else { 3i32 }) as u8);
            }
            if __sw1 == 5i32 {
                return ((if (FlagGet(2164u16)) != 0 { 2i32 } else { 3i32 }) as u8);
            }
            if __sw1 == 6i32 {
                return ((if (FlagGet(2165u16)) != 0 { 2i32 } else { 3i32 }) as u8);
            }
            if __sw1 == 7i32 {
                return ((if (FlagGet(2166u16)) != 0 { 2i32 } else { 3i32 }) as u8);
            }
            if __sw1 == 8i32 {
                return ((if (FlagGet(2167u16)) != 0 { 2i32 } else { 3i32 }) as u8);
            }
            if __sw1 == 9i32 {
                return ((if (FlagGet(2168u16)) != 0 { 2i32 } else { 3i32 }) as u8);
            }
            if __sw1 == 10i32 {
                return ((if (FlagGet(2169u16)) != 0 { 2i32 } else { 3i32 }) as u8);
            }
            if __sw1 == 11i32 {
                return ((if (FlagGet(2170u16)) != 0 { 2i32 } else { 3i32 }) as u8);
            }
            if __sw1 == 12i32 {
                return ((if (FlagGet(2171u16)) != 0 { 2i32 } else { 3i32 }) as u8);
            }
            if __sw1 == 13i32 {
                return ((if (FlagGet(2172u16)) != 0 { 2i32 } else { 3i32 }) as u8);
            }
            if __sw1 == 14i32 {
                return ((if (FlagGet(2173u16)) != 0 { 2i32 } else { 3i32 }) as u8);
            }
            if __sw1 == 15i32 {
                return ((if (FlagGet(2174u16)) != 0 { 2i32 } else { 3i32 }) as u8);
            }
            if __sw1 == 58i32 {
                return ((if (FlagGet(2216u16)) != 0 { 4i32 } else { 0i32 }) as u8);
            }
            if __sw1 == 73i32 {
                return ((if (FlagGet(2217u16)) != 0 { 1i32 } else { 0i32 }) as u8);
            }
            if !__matched {
                return 1u8;
            }
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetRegionMapSecIdAt(x: u16, y: u16) -> u16 {
    unsafe {
        let mut x = x;
        let mut y = y;
        return GetMapSecIdAt(x, y);
    }
}
pub(crate) unsafe extern "C" fn CorrectSpecialMapSecId_Internal(mapSecId: u16) -> u16 {
    unsafe {
        let mut mapSecId = mapSecId;
        let mut i: u32 = 0u32;
        {
            i = 0u32;
            'l1: loop {
                if !(i < crate::c::div_u32(6u32, 2u32)) {
                    break 'l1;
                }
                'l2: {
                    if ((((((&raw const sMarineCaveMapSecIds)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<u16>())
                    .cast::<u16>())
                    .wrapping_offset(((i) as i32) as isize))
                    .read()) as i32)
                        == ((mapSecId) as i32)
                    {
                        return GetTerraOrMarineCaveMapSecId();
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        {
            i = 0u32;
            'l3: loop {
                if !((((((((&raw const sRegionMap_SpecialPlaceLocations)
                    .cast::<u8>()
                    .cast_mut())
                .cast::<u8>())
                .wrapping_offset(((i) as i32) as isize * 4))
                .cast::<u16>())
                .read()) as i32)
                    != 213i32)
                {
                    break 'l3;
                }
                'l4: {
                    if (((((((&raw const sRegionMap_SpecialPlaceLocations)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>())
                    .wrapping_offset(((i) as i32) as isize * 4))
                    .cast::<u16>())
                    .read()) as i32)
                        == ((mapSecId) as i32)
                    {
                        return ((((((&raw const sRegionMap_SpecialPlaceLocations)
                            .cast::<u8>()
                            .cast_mut())
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 4))
                        .cast::<u16>())
                        .wrapping_offset(1))
                        .read();
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return mapSecId;
    }
}
pub(crate) unsafe extern "C" fn GetTerraOrMarineCaveMapSecId() -> u16 {
    unsafe {
        let mut idx: i16 = 0i16;
        idx = ((((VarGet(16439u16)) as i32).wrapping_sub(1i32)) as i16);
        if (((idx) as i32) < 0i32) || (((idx) as i32) > 15i32) {
            idx = 0i16;
        }
        return ((((&raw const sTerraOrMarineCaveMapSecIds)
            .cast::<u8>()
            .cast_mut()
            .cast::<u16>())
        .cast::<u16>())
        .wrapping_offset(((idx) as i32) as isize))
        .read();
    }
}
pub(crate) unsafe extern "C" fn GetMarineCaveCoords(x: *mut u16, y: *mut u16) {
    unsafe {
        let mut x = x;
        let mut y = y;
        let mut idx: u16 = 0u16;
        idx = VarGet(16439u16);
        if (((idx) as i32) < 9i32) || (((idx) as i32) > 16i32) {
            idx = 9u16;
        }
        idx = ((((idx) as i32).wrapping_sub(9i32)) as u16);
        (x).write(
            (((((((((&raw const sMarineCaveLocationCoords)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>())
            .wrapping_offset(((idx) as i32) as isize * 4))
            .cast::<u16>())
            .read()) as i32)
                .wrapping_add(1i32)) as u16),
        );
        (y).write(
            (((((((((&raw const sMarineCaveLocationCoords)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>())
            .wrapping_offset(((idx) as i32) as isize * 4))
            .wrapping_add(2)
            .cast::<u16>())
            .read()) as i32)
                .wrapping_add(2i32)) as u16),
        );
    }
}
pub(crate) unsafe extern "C" fn IsPlayerInAquaHideout(mapSecId: u8) -> u32 {
    unsafe {
        let mut mapSecId = mapSecId;
        let mut i: u32 = 0u32;
        {
            i = 0u32;
            'l1: loop {
                if !(i < crate::c::div_u32(1u32, 1u32)) {
                    break 'l1;
                }
                'l2: {
                    if ((((((&raw const sMapSecAquaHideoutOld).cast::<u8>().cast_mut())
                        .cast::<u8>())
                    .wrapping_offset(((i) as i32) as isize))
                    .read()) as i32)
                        == ((mapSecId) as i32)
                    {
                        return 1u32;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return 0u32;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CorrectSpecialMapSecId(mapSecId: u16) -> u16 {
    unsafe {
        let mut mapSecId = mapSecId;
        return CorrectSpecialMapSecId_Internal(mapSecId);
    }
}
pub(crate) unsafe extern "C" fn GetPositionOfCursorWithinMapSec() {
    unsafe {
        let mut x: u16 = 0u16;
        let mut y: u16 = 0u16;
        let mut posWithinMapSec: u16 = 0u16;
        if ((((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read()).cast::<u16>()).read())
            as i32)
            == 213i32
        {
            ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(3))
                .write(0u8);
            return;
        }
        if !((((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(120))
            .read())
            != 0)
        {
            x = ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(84)
                .cast::<u16>())
            .read();
            y = ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(86)
                .cast::<u16>())
            .read();
        } else {
            x = ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(100)
                .cast::<u16>())
            .read();
            y = ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(102)
                .cast::<u16>())
            .read();
        }
        posWithinMapSec = 0u16;
        'l1: loop {
            if !((1i32) != 0) {
                break 'l1;
            }
            if ((x) as i32) <= 1i32 {
                if (RegionMap_IsMapSecIdInNextRow(y)) != 0 {
                    y = (y).wrapping_sub(1);
                    x = 29u16;
                } else {
                    break 'l1;
                }
            } else {
                x = (x).wrapping_sub(1);
                if ((GetMapSecIdAt(x, y)) as i32)
                    == ((((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                        .cast::<u16>())
                    .read()) as i32)
                {
                    posWithinMapSec = (posWithinMapSec).wrapping_add(1);
                }
            }
        }
        ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(3))
            .write(((posWithinMapSec) as u8));
    }
}
pub(crate) unsafe extern "C" fn RegionMap_IsMapSecIdInNextRow(y: u16) -> u8 {
    unsafe {
        let mut y = y;
        let mut x: u16 = 0u16;
        if (({
            let __t1 = y;
            y = (y).wrapping_sub(1);
            __t1
        }) as i32)
            == 0i32
        {
            return 0u8;
        }
        {
            x = 1u16;
            'l1: loop {
                if !(((x) as i32) <= 28i32) {
                    break 'l1;
                }
                'l2: {
                    if ((GetMapSecIdAt(x, y)) as i32)
                        == ((((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                            .cast::<u16>())
                        .read()) as i32)
                    {
                        return 1u8;
                    }
                }
                x = (x).wrapping_add(1);
            }
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_CursorMapFull(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if ((((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(122)
            .cast::<i8>())
        .read()) as i32)
            != 0i32
        {
            let __p1 = (sprite).wrapping_add(32).cast::<i16>();
            (__p1).write(
                (((((__p1).read()) as i32).wrapping_add(
                    (2i32).wrapping_mul(
                        ((((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(123)
                            .cast::<i8>())
                        .read()) as i32),
                    ),
                )) as i16),
            );
            let __p2 = (sprite).wrapping_add(34).cast::<i16>();
            (__p2).write(
                (((((__p2).read()) as i32).wrapping_add(
                    (2i32).wrapping_mul(
                        ((((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(124)
                            .cast::<i8>())
                        .read()) as i32),
                    ),
                )) as i16),
            );
            let __p3 = (((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(122)
                .cast::<i8>();
            (__p3).write(((__p3).read()).wrapping_sub(1));
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_CursorMapZoomed(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CreateRegionMapCursor(tileTag: u16, paletteTag: u16) {
    unsafe {
        let mut tileTag = tileTag;
        let mut paletteTag = paletteTag;
        let mut spriteId: u8 = 0u8;
        let mut template = crate::ffi::Align4([0u8; 24]);
        let mut palette = crate::ffi::Align4([0u8; 8]);
        let mut sheet = crate::ffi::Align4([0u8; 8]);
        (&raw mut palette)
            .cast::<u8>()
            .cast::<crate::c::Rec4<8>>()
            .write_unaligned(
                (&raw const sRegionMapCursorSpritePalette)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<crate::c::Rec4<8>>()
                    .read_unaligned(),
            );
        (&raw mut template)
            .cast::<u8>()
            .cast::<crate::c::Rec4<24>>()
            .write_unaligned(
                (&raw const sRegionMapCursorSpriteTemplate)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<crate::c::Rec4<24>>()
                    .read_unaligned(),
            );
        (((&raw mut sheet).cast::<u8>())
            .wrapping_add(6)
            .cast::<u16>())
        .write(tileTag);
        (((&raw mut template).cast::<u8>()).cast::<u16>()).write(tileTag);
        ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(88)
            .cast::<u16>())
        .write(tileTag);
        (((&raw mut palette).cast::<u8>())
            .wrapping_add(4)
            .cast::<u16>())
        .write(paletteTag);
        (((&raw mut template).cast::<u8>())
            .wrapping_add(2)
            .cast::<u16>())
        .write(paletteTag);
        ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(90)
            .cast::<u16>())
        .write(paletteTag);
        if !((((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(120))
            .read())
            != 0)
        {
            (((&raw mut sheet).cast::<u8>()).cast::<*mut u8>()).write(
                ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(388))
                    .cast::<u8>(),
            );
            (((&raw mut sheet).cast::<u8>())
                .wrapping_add(4)
                .cast::<u16>())
            .write(256u16);
            (((&raw mut template).cast::<u8>())
                .wrapping_add(20)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(SpriteCB_CursorMapFull));
        } else {
            (((&raw mut sheet).cast::<u8>()).cast::<*mut u8>()).write(
                ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(644))
                    .cast::<u8>(),
            );
            (((&raw mut sheet).cast::<u8>())
                .wrapping_add(4)
                .cast::<u16>())
            .write(1536u16);
            (((&raw mut template).cast::<u8>())
                .wrapping_add(20)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(SpriteCB_CursorMapZoomed));
        }
        LoadSpriteSheet((&raw mut sheet).cast::<u8>());
        LoadSpritePalette((&raw mut palette).cast::<u8>());
        spriteId = CreateSprite((&raw mut template).cast::<u8>(), 56i16, 72i16, 0u8);
        if ((spriteId) as i32) != 64i32 {
            ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(28)
                .cast::<*mut u8>())
            .write(
                ((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68),
            );
            if ((((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(120))
            .read()) as i32)
                == 1i32
            {
                crate::c::bf_write(
                    (((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(28)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(3),
                    6,
                    2,
                    (2u32) as i32,
                );
                let __p1 = (((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(28)
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(32)
                .cast::<i16>();
                (__p1).write((((((__p1).read()) as i32).wrapping_sub(8i32)) as i16));
                let __p2 = (((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(28)
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(34)
                .cast::<i16>();
                (__p2).write((((((__p2).read()) as i32).wrapping_sub(8i32)) as i16));
                StartSpriteAnim(
                    ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(28)
                        .cast::<*mut u8>())
                    .read(),
                    1u8,
                );
            } else {
                crate::c::bf_write(
                    (((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(28)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(3),
                    6,
                    2,
                    (1u32) as i32,
                );
                ((((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(28)
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(32)
                .cast::<i16>())
                .write(
                    ((((8i32).wrapping_mul(
                        ((((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(84)
                            .cast::<u16>())
                        .read()) as i32),
                    ))
                    .wrapping_add(4i32)) as i16),
                );
                ((((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(28)
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(34)
                .cast::<i16>())
                .write(
                    ((((8i32).wrapping_mul(
                        ((((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(86)
                            .cast::<u16>())
                        .read()) as i32),
                    ))
                    .wrapping_add(4i32)) as i16),
                );
            }
            ((((((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(28)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(46))
            .cast::<i16>())
            .wrapping_offset(1))
            .write(2i16);
            ((((((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(28)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(46))
            .cast::<i16>())
            .wrapping_offset(2))
            .write(
                ((((256i32).wrapping_add(
                    ((IndexOfSpritePaletteTag(paletteTag)) as i32).wrapping_mul(16i32),
                ))
                .wrapping_add(1i32)) as i16),
            );
            ((((((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(28)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(46))
            .cast::<i16>())
            .wrapping_offset(3))
            .write(1i16);
        }
    }
}
pub(crate) unsafe extern "C" fn FreeRegionMapCursorSprite() {
    unsafe {
        if ((((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(28)
            .cast::<*mut u8>())
        .read()) as usize)
            != 0usize
        {
            DestroySprite(
                ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(28)
                    .cast::<*mut u8>())
                .read(),
            );
            FreeSpriteTilesByTag(
                ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(88)
                    .cast::<u16>())
                .read(),
            );
            FreeSpritePaletteByTag(
                ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(90)
                    .cast::<u16>())
                .read(),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn SetUnkCursorSpriteData() {
    unsafe {
        ((((((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(28)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(3))
        .write(1i16);
    }
}
pub(crate) unsafe extern "C" fn ClearUnkCursorSpriteData() {
    unsafe {
        ((((((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(28)
            .cast::<*mut u8>())
        .read())
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(3))
        .write(0i16);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CreateRegionMapPlayerIcon(tileTag: u16, paletteTag: u16) {
    unsafe {
        let mut tileTag = tileTag;
        let mut paletteTag = paletteTag;
        let mut spriteId: u8 = 0u8;
        let mut sheet = crate::ffi::Align4([0u8; 8]);
        (&raw mut sheet)
            .cast::<u8>()
            .wrapping_add(0)
            .cast::<*mut u8>()
            .write(
                ((&raw const sRegionMapPlayerIcon_BrendanGfx)
                    .cast::<u8>()
                    .cast_mut())
                .cast::<u8>(),
            );
        (&raw mut sheet)
            .cast::<u8>()
            .wrapping_add(4)
            .cast::<u16>()
            .write(128u16);
        (&raw mut sheet)
            .cast::<u8>()
            .wrapping_add(6)
            .cast::<u16>()
            .write(tileTag);
        let mut palette = crate::ffi::Align4([0u8; 8]);
        (&raw mut palette)
            .cast::<u8>()
            .wrapping_add(0)
            .cast::<*mut u16>()
            .write(
                ((&raw const sRegionMapPlayerIcon_BrendanPal)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<u16>())
                .cast::<u16>(),
            );
        (&raw mut palette)
            .cast::<u8>()
            .wrapping_add(4)
            .cast::<u16>()
            .write(paletteTag);
        let mut template = crate::ffi::Align4([0u8; 24]);
        (&raw mut template)
            .cast::<u8>()
            .wrapping_add(0)
            .cast::<u16>()
            .write(tileTag);
        (&raw mut template)
            .cast::<u8>()
            .wrapping_add(2)
            .cast::<u16>()
            .write(paletteTag);
        (&raw mut template)
            .cast::<u8>()
            .wrapping_add(4)
            .cast::<*mut u8>()
            .write((&raw const sRegionMapPlayerIconOam).cast::<u8>().cast_mut());
        (&raw mut template)
            .cast::<u8>()
            .wrapping_add(8)
            .cast::<*mut *mut u8>()
            .write(
                ((&raw const sRegionMapPlayerIconAnimTable)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<*mut u8>())
                .cast::<*mut u8>(),
            );
        (&raw mut template)
            .cast::<u8>()
            .wrapping_add(12)
            .cast::<*mut u8>()
            .write(core::ptr::null_mut());
        (&raw mut template)
            .cast::<u8>()
            .wrapping_add(16)
            .cast::<*mut *mut u8>()
            .write(((&raw mut gDummySpriteAffineAnimTable).cast::<*mut u8>()).cast::<*mut u8>());
        (&raw mut template)
            .cast::<u8>()
            .wrapping_add(20)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>()
            .write(Some(SpriteCallbackDummy));
        if (IsEventIslandMapSecId((((&raw mut gMapHeader).cast::<u8>()).wrapping_add(20)).read()))
            != 0
        {
            ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(32)
                .cast::<*mut u8>())
            .write(core::ptr::null_mut());
            return;
        }
        if ((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(8)).read())
            as i32)
            == 1i32
        {
            (((&raw mut sheet).cast::<u8>()).cast::<*mut u8>()).write(
                ((&raw const sRegionMapPlayerIcon_MayGfx)
                    .cast::<u8>()
                    .cast_mut())
                .cast::<u8>(),
            );
            (((&raw mut palette).cast::<u8>()).cast::<*mut u16>()).write(
                ((&raw const sRegionMapPlayerIcon_MayPal)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<u16>())
                .cast::<u16>(),
            );
        }
        LoadSpriteSheet((&raw mut sheet).cast::<u8>());
        LoadSpritePalette((&raw mut palette).cast::<u8>());
        spriteId = CreateSprite((&raw mut template).cast::<u8>(), 0i16, 0i16, 1u8);
        ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(32)
            .cast::<*mut u8>())
        .write(
            ((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68),
        );
        if !((((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(120))
            .read())
            != 0)
        {
            ((((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(32)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(32)
            .cast::<i16>())
            .write(
                (((((((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(116)
                    .cast::<u16>())
                .read()) as i32)
                    .wrapping_mul(8i32))
                .wrapping_add(4i32)) as i16),
            );
            ((((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(32)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(34)
            .cast::<i16>())
            .write(
                (((((((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(118)
                    .cast::<u16>())
                .read()) as i32)
                    .wrapping_mul(8i32))
                .wrapping_add(4i32)) as i16),
            );
            ((((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(32)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(SpriteCB_PlayerIconMapFull));
        } else {
            ((((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(32)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(32)
            .cast::<i16>())
            .write(
                (((((((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(116)
                    .cast::<u16>())
                .read()) as i32)
                    .wrapping_mul(16i32))
                .wrapping_sub(48i32)) as i16),
            );
            ((((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(32)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(34)
            .cast::<i16>())
            .write(
                (((((((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(118)
                    .cast::<u16>())
                .read()) as i32)
                    .wrapping_mul(16i32))
                .wrapping_sub(66i32)) as i16),
            );
            ((((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(32)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(SpriteCB_PlayerIconMapZoomed));
        }
    }
}
pub(crate) unsafe extern "C" fn HideRegionMapPlayerIcon() {
    unsafe {
        if ((((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(32)
            .cast::<*mut u8>())
        .read()) as usize)
            != 0usize
        {
            crate::c::bf_write(
                (((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(32)
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(62),
                2,
                1,
                (1u16) as i32,
            );
            ((((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(32)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(SpriteCallbackDummy));
        }
    }
}
pub(crate) unsafe extern "C" fn UnhideRegionMapPlayerIcon() {
    unsafe {
        if ((((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(32)
            .cast::<*mut u8>())
        .read()) as usize)
            != 0usize
        {
            if ((((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(120))
            .read()) as i32)
                == 1i32
            {
                ((((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(32)
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(32)
                .cast::<i16>())
                .write(
                    (((((((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(116)
                        .cast::<u16>())
                    .read()) as i32)
                        .wrapping_mul(16i32))
                    .wrapping_sub(48i32)) as i16),
                );
                ((((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(32)
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(34)
                .cast::<i16>())
                .write(
                    (((((((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(118)
                        .cast::<u16>())
                    .read()) as i32)
                        .wrapping_mul(16i32))
                    .wrapping_sub(66i32)) as i16),
                );
                ((((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(32)
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                .write(Some(SpriteCB_PlayerIconMapZoomed));
                crate::c::bf_write(
                    (((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(32)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(62),
                    2,
                    1,
                    (0u16) as i32,
                );
            } else {
                ((((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(32)
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(32)
                .cast::<i16>())
                .write(
                    (((((((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(116)
                        .cast::<u16>())
                    .read()) as i32)
                        .wrapping_mul(8i32))
                    .wrapping_add(4i32)) as i16),
                );
                ((((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(32)
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(34)
                .cast::<i16>())
                .write(
                    (((((((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(118)
                        .cast::<u16>())
                    .read()) as i32)
                        .wrapping_mul(8i32))
                    .wrapping_add(4i32)) as i16),
                );
                ((((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(32)
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(36)
                .cast::<i16>())
                .write(0i16);
                ((((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(32)
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(38)
                .cast::<i16>())
                .write(0i16);
                ((((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(32)
                    .cast::<*mut u8>())
                .read())
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                .write(Some(SpriteCB_PlayerIconMapFull));
                crate::c::bf_write(
                    (((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(32)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(62),
                    2,
                    1,
                    (0u16) as i32,
                );
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_PlayerIconMapZoomed(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        ((sprite).wrapping_add(36).cast::<i16>()).write(
            (((-2i32).wrapping_mul(
                ((((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(92)
                    .cast::<i16>())
                .read()) as i32),
            )) as i16),
        );
        ((sprite).wrapping_add(38).cast::<i16>()).write(
            (((-2i32).wrapping_mul(
                ((((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(94)
                    .cast::<i16>())
                .read()) as i32),
            )) as i16),
        );
        (((sprite).wrapping_add(46)).cast::<i16>()).write(
            (((((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32)
                .wrapping_add(((((sprite).wrapping_add(38).cast::<i16>()).read()) as i32)))
            .wrapping_add(((((sprite).wrapping_add(41).cast::<i8>()).read()) as i32)))
                as i16),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(
            (((((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32)
                .wrapping_add(((((sprite).wrapping_add(36).cast::<i16>()).read()) as i32)))
            .wrapping_add(((((sprite).wrapping_add(40).cast::<i8>()).read()) as i32)))
                as i16),
        );
        if ((((((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) < (-8i32))
            || ((((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) > 168i32))
            || (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
                < (-8i32)))
            || (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
                > 248i32)
        {
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(0i16);
        } else {
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(1i16);
        }
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
            == 1i32
        {
            SpriteCB_PlayerIcon(sprite);
        } else {
            crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (1u16) as i32);
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_PlayerIconMapFull(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        SpriteCB_PlayerIcon(sprite);
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_PlayerIcon(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(126))
            .read())
            != 0
        {
            if (({
                let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7);
                let __t2 = ((__p1).read()).wrapping_add(1);
                (__p1).write(__t2);
                __t2
            }) as i32)
                > 16i32
            {
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(0i16);
                crate::c::bf_write(
                    (sprite).wrapping_add(62),
                    2,
                    1,
                    ((if (crate::c::bf_read((sprite).wrapping_add(62), 2, 1, false) as u16) != 0 {
                        0i32
                    } else {
                        1i32
                    }) as u16) as i32,
                );
            }
        } else {
            crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (0u16) as i32);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn TrySetPlayerIconBlink() {
    unsafe {
        if (((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(127))
            .read())
            != 0
        {
            ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(126))
                .write(1u8);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetMapName(dest: *mut u8, regionMapId: u16, padLength: u16) -> *mut u8 {
    unsafe {
        let mut dest = dest;
        let mut regionMapId = regionMapId;
        let mut padLength = padLength;
        let mut str: *mut u8 = core::ptr::null_mut();
        let mut i: u16 = 0u16;
        if ((regionMapId) as i32) == 86i32 {
            str = GetSecretBaseMapName(dest);
        } else {
            if ((regionMapId) as i32) < 213i32 {
                str = StringCopy(
                    dest,
                    (((((&raw const gRegionMapEntries).cast::<u8>().cast_mut()).cast::<u8>())
                        .wrapping_offset(((regionMapId) as i32) as isize * 8))
                    .wrapping_add(4)
                    .cast::<*mut u8>())
                    .read(),
                );
            } else {
                if ((padLength) as i32) == 0i32 {
                    padLength = 18u16;
                }
                return StringFill(dest, 0u8, padLength);
            }
        }
        if ((padLength) as i32) != 0i32 {
            {
                i = ((((str) as usize).wrapping_sub((dest) as usize) as i32 / 1) as u16);
                'l1: loop {
                    if !(((i) as i32) < ((padLength) as i32)) {
                        break 'l1;
                    }
                    'l2: {
                        ({
                            let __t1 = str;
                            str = (str).wrapping_offset(1);
                            __t1
                        })
                        .write(0u8);
                    }
                    i = (i).wrapping_add(1);
                }
            }
            (str).write(255u8);
        }
        return str;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetMapNameGeneric(dest: *mut u8, mapSecId: u16) -> *mut u8 {
    unsafe {
        let mut dest = dest;
        let mut mapSecId = mapSecId;
        'l1: {
            let __sw1 = ((mapSecId) as i32);
            let __matched = __sw1 == 87i32 || __sw1 == 86i32;
            if __sw1 == 87i32 {
                return StringCopy(dest, (&raw mut gText_Ferry).cast::<u8>());
            }
            if __sw1 == 86i32 {
                return StringCopy(dest, (&raw mut gText_SecretBase).cast::<u8>());
            }
            if !__matched {
                return GetMapName(dest, mapSecId, 0u16);
            }
        }
        #[allow(unreachable_code)]
        {
            return core::ptr::null_mut();
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetMapNameHandleAquaHideout(dest: *mut u8, mapSecId: u16) -> *mut u8 {
    unsafe {
        let mut dest = dest;
        let mut mapSecId = mapSecId;
        if ((mapSecId) as i32) == 66i32 {
            return StringCopy(dest, (&raw mut gText_Hideout).cast::<u8>());
        } else {
            return GetMapNameGeneric(dest, mapSecId);
        }
        #[allow(unreachable_code)]
        {
            return core::ptr::null_mut();
        }
    }
}
pub(crate) unsafe extern "C" fn GetMapSecDimensions(
    mapSecId: u16,
    x: *mut u16,
    y: *mut u16,
    width: *mut u16,
    height: *mut u16,
) {
    unsafe {
        let mut mapSecId = mapSecId;
        let mut x = x;
        let mut y = y;
        let mut width = width;
        let mut height = height;
        (x).write(
            ((((((&raw const gRegionMapEntries).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((mapSecId) as i32) as isize * 8))
            .read()) as u16),
        );
        (y).write(
            (((((((&raw const gRegionMapEntries).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((mapSecId) as i32) as isize * 8))
            .wrapping_add(1))
            .read()) as u16),
        );
        (width).write(
            (((((((&raw const gRegionMapEntries).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((mapSecId) as i32) as isize * 8))
            .wrapping_add(2))
            .read()) as u16),
        );
        (height).write(
            (((((((&raw const gRegionMapEntries).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((mapSecId) as i32) as isize * 8))
            .wrapping_add(3))
            .read()) as u16),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsRegionMapZoomed() -> u8 {
    unsafe {
        return ((((&raw mut sRegionMap).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(120))
            .read();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsEventIslandMapSecId(mapSecId: u8) -> u32 {
    unsafe {
        let mut mapSecId = mapSecId;
        let mut i: u32 = 0u32;
        {
            i = 0u32;
            'l1: loop {
                if !(i < crate::c::div_u32(3u32, 1u32)) {
                    break 'l1;
                }
                'l2: {
                    if ((mapSecId) as i32)
                        == ((((((&raw const sMapSecIdsOffMap).cast::<u8>().cast_mut())
                            .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize))
                        .read()) as i32)
                    {
                        return 1u32;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return 0u32;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CB2_OpenFlyMap() {
    unsafe {
        'l1: {
            let __sw1 = (((((&raw mut gMain).cast::<u8>()).wrapping_add(1080)).read()) as i32);
            if __sw1 == 0i32 {
                SetVBlankCallback(None);
                SetGpuReg(0u8, 0u16);
                SetGpuReg(16u8, 0u16);
                SetGpuReg(18u8, 0u16);
                SetGpuReg(20u8, 0u16);
                SetGpuReg(22u8, 0u16);
                SetGpuReg(26u8, 0u16);
                SetGpuReg(24u8, 0u16);
                SetGpuReg(28u8, 0u16);
                SetGpuReg(30u8, 0u16);
                ((&raw mut sFlyMap).cast::<u8>().cast::<*mut u8>()).write(Alloc(2676u32));
                if ((((&raw mut sFlyMap).cast::<u8>().cast::<*mut u8>()).read()) as usize) == 0usize
                {
                    SetMainCallback2(Some(CB2_ReturnToFieldWithOpenMenu));
                } else {
                    ResetPaletteFade();
                    ResetSpriteData();
                    FreeSpriteTileRanges();
                    FreeAllSpritePalettes();
                    let __p2 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                    (__p2).write(((__p2).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                ResetBgsAndClearDma3BusyFlags(0u32);
                InitBgsFromTemplates(
                    1u8,
                    ((&raw const sFlyMapBgTemplates).cast::<u8>().cast_mut()).cast::<u8>(),
                    ((crate::c::div_u32(12u32, 4u32)) as u8),
                );
                let __p3 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p3).write(((__p3).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 2i32 {
                InitWindows(
                    ((&raw const sFlyMapWindowTemplates).cast::<u8>().cast_mut()).cast::<u8>(),
                );
                DeactivateAllTextPrinters();
                let __p4 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p4).write(((__p4).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 3i32 {
                LoadUserWindowBorderGfx(0u8, 101u16, 208u8);
                ClearScheduledBgCopiesToVram();
                let __p5 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p5).write(((__p5).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 4i32 {
                InitRegionMap(
                    (((&raw mut sFlyMap).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(8),
                    0u8,
                );
                CreateRegionMapCursor(0u16, 0u16);
                CreateRegionMapPlayerIcon(1u16, 1u16);
                ((((&raw mut sFlyMap).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(6)
                    .cast::<u16>())
                .write(
                    (((((&raw mut sFlyMap).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(8))
                    .cast::<u16>())
                    .read(),
                );
                StringFill(
                    ((((&raw mut sFlyMap).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(2636))
                    .cast::<u8>(),
                    0u8,
                    16u16,
                );
                ((&raw mut sDrawFlyDestTextWindow).cast::<u8>().cast::<u32>()).write(1u32);
                DrawFlyDestTextWindow();
                let __p6 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p6).write(((__p6).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 5i32 {
                LZ77UnCompVram(
                    ((&raw const sRegionMapFrameGfxLZ)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<u32>())
                    .cast::<u32>(),
                    ((100712448i32) as usize as *mut u16).cast::<u8>(),
                );
                let __p7 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p7).write(((__p7).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 6i32 {
                LZ77UnCompVram(
                    ((&raw const sRegionMapFrameTilemapLZ)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<u32>())
                    .cast::<u32>(),
                    ((100724736i32) as usize as *mut u16).cast::<u8>(),
                );
                let __p8 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p8).write(((__p8).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 7i32 {
                LoadPalette(
                    (((&raw const sRegionMapFramePal)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<u16>())
                    .cast::<u16>())
                    .cast::<u8>(),
                    16u16,
                    32u16,
                );
                PutWindowTilemap(2u8);
                FillWindowPixelBuffer(2u8, 0u8);
                AddTextPrinterParameterized(
                    2u8,
                    1u8,
                    (&raw mut gText_FlyToWhere).cast::<u8>(),
                    0u8,
                    1u8,
                    0u8,
                    None,
                );
                ScheduleBgCopyTilemapToVram(0u8);
                let __p9 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p9).write(((__p9).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 8i32 {
                LoadFlyDestIcons();
                let __p10 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p10).write(((__p10).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 9i32 {
                BlendPalettes(4294967295u32, 16u8, 0u16);
                SetVBlankCallback(Some(VBlankCB_FlyMap));
                let __p11 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p11).write(((__p11).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 10i32 {
                SetGpuReg(80u8, 0u16);
                SetGpuRegBits(0u8, 4160u16);
                ShowBg(0u8);
                ShowBg(1u8);
                ShowBg(2u8);
                SetFlyMapCallback(Some(CB_FadeInFlyMap));
                SetMainCallback2(Some(CB2_FlyMap));
                let __p12 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p12).write(((__p12).read()).wrapping_add(1));
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn VBlankCB_FlyMap() {
    unsafe {
        LoadOam();
        ProcessSpriteCopyRequests();
        TransferPlttBuffer();
    }
}
pub(crate) unsafe extern "C" fn CB2_FlyMap() {
    unsafe {
        (((((&raw mut sFlyMap).cast::<u8>().cast::<*mut u8>()).read())
            .cast::<Option<unsafe extern "C" fn()>>())
        .read())
        .unwrap_unchecked()();
        AnimateSprites();
        BuildOamBuffer();
        DoScheduledBgTilemapCopiesToVram();
    }
}
pub(crate) unsafe extern "C" fn SetFlyMapCallback(callback: Option<unsafe extern "C" fn()>) {
    unsafe {
        let mut callback = callback;
        ((((&raw mut sFlyMap).cast::<u8>().cast::<*mut u8>()).read())
            .cast::<Option<unsafe extern "C" fn()>>())
        .write(callback);
        ((((&raw mut sFlyMap).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(4)
            .cast::<u16>())
        .write(0u16);
    }
}
pub(crate) unsafe extern "C" fn DrawFlyDestTextWindow() {
    unsafe {
        let mut i: u16 = 0u16;
        let mut namePrinted: u32 = 0u32;
        let mut name: *mut u8 = core::ptr::null_mut();
        if ((((((((&raw mut sFlyMap).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(8))
            .wrapping_add(2))
        .read()) as i32)
            > 0i32)
            && ((((((((&raw mut sFlyMap).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(8))
                .wrapping_add(2))
            .read()) as i32)
                < 5i32)
        {
            namePrinted = 0u32;
            {
                i = 0u16;
                'l1: loop {
                    if !(((i) as u32) < crate::c::div_u32(8u32, 8u32)) {
                        break 'l1;
                    }
                    'l2: {
                        if (((((((&raw mut sFlyMap).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(8))
                        .cast::<u16>())
                        .read()) as i32)
                            == (((((((&raw const sMultiNameFlyDestinations)
                                .cast::<u8>()
                                .cast_mut())
                            .cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 8))
                            .wrapping_add(4)
                            .cast::<u16>())
                            .read()) as i32)
                        {
                            if (FlagGet(
                                (((((&raw const sMultiNameFlyDestinations)
                                    .cast::<u8>()
                                    .cast_mut())
                                .cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize * 8))
                                .wrapping_add(6)
                                .cast::<u16>())
                                .read(),
                            )) != 0
                            {
                                StringLength(
                                    (((((((&raw const sMultiNameFlyDestinations)
                                        .cast::<u8>()
                                        .cast_mut())
                                    .cast::<u8>())
                                    .wrapping_offset(((i) as i32) as isize * 8))
                                    .cast::<*mut *mut u8>())
                                    .read())
                                    .wrapping_offset(
                                        (((((((&raw mut sFlyMap).cast::<u8>().cast::<*mut u8>())
                                            .read())
                                        .wrapping_add(8))
                                        .wrapping_add(3))
                                        .read()) as i32)
                                            as isize,
                                    ))
                                    .read(),
                                );
                                namePrinted = 1u32;
                                ClearStdWindowAndFrameToTransparent(0u8, 0u8);
                                DrawStdFrameWithCustomTileAndPalette(1u8, 0u8, 101u16, 13u8);
                                AddTextPrinterParameterized(
                                    1u8,
                                    1u8,
                                    (((((&raw mut sFlyMap).cast::<u8>().cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(8))
                                    .wrapping_add(4))
                                    .cast::<u8>(),
                                    0u8,
                                    1u8,
                                    0u8,
                                    None,
                                );
                                name = (((((((&raw const sMultiNameFlyDestinations)
                                    .cast::<u8>()
                                    .cast_mut())
                                .cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize * 8))
                                .cast::<*mut *mut u8>())
                                .read())
                                .wrapping_offset(
                                    (((((((&raw mut sFlyMap).cast::<u8>().cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(8))
                                    .wrapping_add(3))
                                    .read()) as i32) as isize,
                                ))
                                .read();
                                AddTextPrinterParameterized(
                                    1u8,
                                    1u8,
                                    name,
                                    ((GetStringRightAlignXOffset(1i32, name, 96i32)) as u8),
                                    17u8,
                                    0u8,
                                    None,
                                );
                                ScheduleBgCopyTilemapToVram(0u8);
                                ((&raw mut sDrawFlyDestTextWindow).cast::<u8>().cast::<u32>())
                                    .write(1u32);
                            }
                            break 'l1;
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
            if !((namePrinted) != 0) {
                if ((&raw mut sDrawFlyDestTextWindow).cast::<u8>().cast::<u32>()).read() == 1u32 {
                    ClearStdWindowAndFrameToTransparent(1u8, 0u8);
                    DrawStdFrameWithCustomTileAndPalette(0u8, 0u8, 101u16, 13u8);
                } else {
                    FillWindowPixelBuffer(0u8, 17u8);
                }
                AddTextPrinterParameterized(
                    0u8,
                    1u8,
                    (((((&raw mut sFlyMap).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(8))
                    .wrapping_add(4))
                    .cast::<u8>(),
                    0u8,
                    1u8,
                    0u8,
                    None,
                );
                ScheduleBgCopyTilemapToVram(0u8);
                ((&raw mut sDrawFlyDestTextWindow).cast::<u8>().cast::<u32>()).write(0u32);
            }
        } else {
            if ((&raw mut sDrawFlyDestTextWindow).cast::<u8>().cast::<u32>()).read() == 1u32 {
                ClearStdWindowAndFrameToTransparent(1u8, 0u8);
                DrawStdFrameWithCustomTileAndPalette(0u8, 0u8, 101u16, 13u8);
            }
            FillWindowPixelBuffer(0u8, 17u8);
            CopyWindowToVram(0u8, 2u8);
            ScheduleBgCopyTilemapToVram(0u8);
            ((&raw mut sDrawFlyDestTextWindow).cast::<u8>().cast::<u32>()).write(0u32);
        }
    }
}
pub(crate) unsafe extern "C" fn LoadFlyDestIcons() {
    unsafe {
        let mut sheet = crate::ffi::Align4([0u8; 8]);
        LZ77UnCompWram(
            ((&raw const sFlyTargetIcons_Gfx)
                .cast::<u8>()
                .cast_mut()
                .cast::<u32>())
            .cast::<u32>(),
            ((((&raw mut sFlyMap).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(2188))
                .cast::<u8>(),
        );
        (((&raw mut sheet).cast::<u8>()).cast::<*mut u8>()).write(
            ((((&raw mut sFlyMap).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(2188))
                .cast::<u8>(),
        );
        (((&raw mut sheet).cast::<u8>())
            .wrapping_add(4)
            .cast::<u16>())
        .write(448u16);
        (((&raw mut sheet).cast::<u8>())
            .wrapping_add(6)
            .cast::<u16>())
        .write(2u16);
        LoadSpriteSheet((&raw mut sheet).cast::<u8>());
        LoadSpritePalette(
            (&raw const sFlyTargetIconsSpritePalette)
                .cast::<u8>()
                .cast_mut(),
        );
        CreateFlyDestIcons();
        TryCreateRedOutlineFlyDestIcons();
    }
}
pub(crate) unsafe extern "C" fn CreateFlyDestIcons() {
    unsafe {
        let mut canFlyFlag: u16 = 0u16;
        let mut mapSecId: u16 = 0u16;
        let mut x: u16 = 0u16;
        let mut y: u16 = 0u16;
        let mut width: u16 = 0u16;
        let mut height: u16 = 0u16;
        let mut shape: u16 = 0u16;
        let mut spriteId: u8 = 0u8;
        canFlyFlag = 2159u16;
        {
            mapSecId = 0u16;
            'l1: loop {
                if !(((mapSecId) as i32) <= 15i32) {
                    break 'l1;
                }
                'l2: {
                    GetMapSecDimensions(
                        mapSecId,
                        &raw mut x,
                        &raw mut y,
                        &raw mut width,
                        &raw mut height,
                    );
                    x = ((((((x) as i32).wrapping_add(1i32)).wrapping_mul(8i32)).wrapping_add(4i32))
                        as u16);
                    y = ((((((y) as i32).wrapping_add(2i32)).wrapping_mul(8i32)).wrapping_add(4i32))
                        as u16);
                    if ((width) as i32) == 2i32 {
                        shape = 1u16;
                    } else {
                        if ((height) as i32) == 2i32 {
                            shape = 2u16;
                        } else {
                            shape = 0u16;
                        }
                    }
                    spriteId = CreateSprite(
                        (&raw const sFlyDestIconSpriteTemplate)
                            .cast::<u8>()
                            .cast_mut(),
                        ((x) as i16),
                        ((y) as i16),
                        10u8,
                    );
                    if ((spriteId) as i32) != 64i32 {
                        crate::c::bf_write(
                            (((&raw mut gSprites).cast::<u8>())
                                .wrapping_offset(((spriteId) as i32) as isize * 68))
                            .wrapping_add(1),
                            6,
                            2,
                            ((shape) as u32) as i32,
                        );
                        if (FlagGet(canFlyFlag)) != 0 {
                            ((((&raw mut gSprites).cast::<u8>())
                                .wrapping_offset(((spriteId) as i32) as isize * 68))
                            .wrapping_add(28)
                            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                            .write(Some(SpriteCB_FlyDestIcon));
                        } else {
                            shape = ((((shape) as i32).wrapping_add(3i32)) as u16);
                        }
                        StartSpriteAnim(
                            ((&raw mut gSprites).cast::<u8>())
                                .wrapping_offset(((spriteId) as i32) as isize * 68),
                            ((shape) as u8),
                        );
                        (((((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((spriteId) as i32) as isize * 68))
                        .wrapping_add(46))
                        .cast::<i16>())
                        .write(((mapSecId) as i16));
                    }
                    canFlyFlag = (canFlyFlag).wrapping_add(1);
                }
                mapSecId = (mapSecId).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn TryCreateRedOutlineFlyDestIcons() {
    unsafe {
        let mut i: u16 = 0u16;
        let mut x: u16 = 0u16;
        let mut y: u16 = 0u16;
        let mut width: u16 = 0u16;
        let mut height: u16 = 0u16;
        let mut mapSecId: u16 = 0u16;
        let mut spriteId: u8 = 0u8;
        {
            i = 0u16;
            'l1: loop {
                if !(((((((((&raw const sRedOutlineFlyDestinations)
                    .cast::<u8>()
                    .cast_mut())
                .cast::<u8>())
                .wrapping_offset(((i) as i32) as isize * 4))
                .cast::<u16>())
                .wrapping_offset(1))
                .read()) as i32)
                    != 213i32)
                {
                    break 'l1;
                }
                'l2: {
                    if (FlagGet(
                        (((((&raw const sRedOutlineFlyDestinations)
                            .cast::<u8>()
                            .cast_mut())
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 4))
                        .cast::<u16>())
                        .read(),
                    )) != 0
                    {
                        mapSecId = ((((((&raw const sRedOutlineFlyDestinations)
                            .cast::<u8>()
                            .cast_mut())
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 4))
                        .cast::<u16>())
                        .wrapping_offset(1))
                        .read();
                        GetMapSecDimensions(
                            mapSecId,
                            &raw mut x,
                            &raw mut y,
                            &raw mut width,
                            &raw mut height,
                        );
                        x = (((((x) as i32).wrapping_add(1i32)).wrapping_mul(8i32)) as u16);
                        y = (((((y) as i32).wrapping_add(2i32)).wrapping_mul(8i32)) as u16);
                        spriteId = CreateSprite(
                            (&raw const sFlyDestIconSpriteTemplate)
                                .cast::<u8>()
                                .cast_mut(),
                            ((x) as i16),
                            ((y) as i16),
                            10u8,
                        );
                        if ((spriteId) as i32) != 64i32 {
                            crate::c::bf_write(
                                (((&raw mut gSprites).cast::<u8>())
                                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                                .wrapping_add(3),
                                6,
                                2,
                                (1u32) as i32,
                            );
                            ((((&raw mut gSprites).cast::<u8>())
                                .wrapping_offset(((spriteId) as i32) as isize * 68))
                            .wrapping_add(28)
                            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                            .write(Some(SpriteCB_FlyDestIcon));
                            StartSpriteAnim(
                                ((&raw mut gSprites).cast::<u8>())
                                    .wrapping_offset(((spriteId) as i32) as isize * 68),
                                6u8,
                            );
                            (((((&raw mut gSprites).cast::<u8>())
                                .wrapping_offset(((spriteId) as i32) as isize * 68))
                            .wrapping_add(46))
                            .cast::<i16>())
                            .write(((mapSecId) as i16));
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_FlyDestIcon(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (((((((&raw mut sFlyMap).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(8))
            .cast::<u16>())
        .read()) as i32)
            == (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32)
        {
            if (({
                let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
                let __t2 = ((__p1).read()).wrapping_add(1);
                (__p1).write(__t2);
                __t2
            }) as i32)
                > 16i32
            {
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(0i16);
                crate::c::bf_write(
                    (sprite).wrapping_add(62),
                    2,
                    1,
                    ((if (crate::c::bf_read((sprite).wrapping_add(62), 2, 1, false) as u16) != 0 {
                        0i32
                    } else {
                        1i32
                    }) as u16) as i32,
                );
            }
        } else {
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(16i16);
            crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (0u16) as i32);
        }
    }
}
pub(crate) unsafe extern "C" fn CB_FadeInFlyMap() {
    unsafe {
        'l1: {
            let __sw1 = ((((((&raw mut sFlyMap).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(4)
                .cast::<u16>())
            .read()) as i32);
            if __sw1 == 0i32 {
                BeginNormalPaletteFade(4294967295u32, 0i8, 16u8, 0u8, 0u16);
                let __p2 = (((&raw mut sFlyMap).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(4)
                    .cast::<u16>();
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                if !((UpdatePaletteFade()) != 0) {
                    SetFlyMapCallback(Some(CB_HandleFlyMapInput));
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn CB_HandleFlyMapInput() {
    unsafe {
        if ((((((&raw mut sFlyMap).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(4)
            .cast::<u16>())
        .read()) as i32)
            == 0i32
        {
            'l1: {
                let __sw1 = ((DoRegionMapInputCallback()) as i32);
                if __sw1 == 0i32 || __sw1 == 1i32 || __sw1 == 2i32 {
                    break 'l1;
                }
                if __sw1 == 3i32 {
                    DrawFlyDestTextWindow();
                    break 'l1;
                }
                if __sw1 == 4i32 {
                    if ((((((((&raw mut sFlyMap).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(8))
                    .wrapping_add(2))
                    .read()) as i32)
                        == 2i32)
                        || ((((((((&raw mut sFlyMap).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(8))
                        .wrapping_add(2))
                        .read()) as i32)
                            == 4i32)
                    {
                        m4aSongNumStart(5u16);
                        ((((&raw mut sFlyMap).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(2674))
                        .write(1u8);
                        SetFlyMapCallback(Some(CB_ExitFlyMap));
                    }
                    break 'l1;
                }
                if __sw1 == 5i32 {
                    m4aSongNumStart(5u16);
                    ((((&raw mut sFlyMap).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(2674))
                    .write(0u8);
                    SetFlyMapCallback(Some(CB_ExitFlyMap));
                    break 'l1;
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn CB_ExitFlyMap() {
    unsafe {
        'l1: {
            let __sw1 = ((((((&raw mut sFlyMap).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(4)
                .cast::<u16>())
            .read()) as i32);
            if __sw1 == 0i32 {
                BeginNormalPaletteFade(4294967295u32, 0i8, 0u8, 16u8, 0u16);
                let __p2 = (((&raw mut sFlyMap).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(4)
                    .cast::<u16>();
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                if !((UpdatePaletteFade()) != 0) {
                    FreeRegionMapIconResources();
                    if (((((&raw mut sFlyMap).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(2674))
                    .read())
                        != 0
                    {
                        'l2: {
                            let __sw3 = (((((((&raw mut sFlyMap).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(8))
                            .cast::<u16>())
                            .read()) as i32);
                            let __matched =
                                __sw3 == 73i32 || __sw3 == 58i32 || __sw3 == 0i32 || __sw3 == 15i32;
                            if __sw3 == 73i32 {
                                SetWarpDestinationToHealLocation(21u8);
                                break 'l2;
                            }
                            if __sw3 == 58i32 {
                                SetWarpDestinationToHealLocation(22u8);
                                break 'l2;
                            }
                            if __sw3 == 0i32 {
                                SetWarpDestinationToHealLocation(
                                    ((if ((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                        .wrapping_add(8))
                                    .read()) as i32)
                                        == 0i32
                                    {
                                        12i32
                                    } else {
                                        13i32
                                    }) as u8),
                                );
                                break 'l2;
                            }
                            if __sw3 == 15i32 {
                                SetWarpDestinationToHealLocation(
                                    ((if ((FlagGet(2228u16)) != 0)
                                        && ((((((((&raw mut sFlyMap)
                                            .cast::<u8>()
                                            .cast::<*mut u8>())
                                        .read())
                                        .wrapping_add(8))
                                        .wrapping_add(3))
                                        .read())
                                            as i32)
                                            == 0i32)
                                    {
                                        20i32
                                    } else {
                                        11i32
                                    }) as u8),
                                );
                                break 'l2;
                            }
                            if !__matched {
                                if ((((((((&raw const sMapHealLocations)
                                    .cast::<u8>()
                                    .cast_mut())
                                .cast::<u8>())
                                .wrapping_offset(
                                    (((((((&raw mut sFlyMap).cast::<u8>().cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(8))
                                    .cast::<u16>())
                                    .read()) as i32) as isize
                                        * 3,
                                ))
                                .cast::<u8>())
                                .wrapping_offset(2))
                                .read()) as i32)
                                    != 0i32
                                {
                                    SetWarpDestinationToHealLocation(
                                        ((((((&raw const sMapHealLocations)
                                            .cast::<u8>()
                                            .cast_mut())
                                        .cast::<u8>())
                                        .wrapping_offset(
                                            (((((((&raw mut sFlyMap)
                                                .cast::<u8>()
                                                .cast::<*mut u8>())
                                            .read())
                                            .wrapping_add(8))
                                            .cast::<u16>())
                                            .read())
                                                as i32)
                                                as isize
                                                * 3,
                                        ))
                                        .cast::<u8>())
                                        .wrapping_offset(2))
                                        .read(),
                                    );
                                } else {
                                    SetWarpDestinationToMapWarp(
                                        (((((((&raw const sMapHealLocations)
                                            .cast::<u8>()
                                            .cast_mut())
                                        .cast::<u8>())
                                        .wrapping_offset(
                                            (((((((&raw mut sFlyMap)
                                                .cast::<u8>()
                                                .cast::<*mut u8>())
                                            .read())
                                            .wrapping_add(8))
                                            .cast::<u16>())
                                            .read())
                                                as i32)
                                                as isize
                                                * 3,
                                        ))
                                        .cast::<u8>())
                                        .read()) as i8),
                                        ((((((((&raw const sMapHealLocations)
                                            .cast::<u8>()
                                            .cast_mut())
                                        .cast::<u8>())
                                        .wrapping_offset(
                                            (((((((&raw mut sFlyMap)
                                                .cast::<u8>()
                                                .cast::<*mut u8>())
                                            .read())
                                            .wrapping_add(8))
                                            .cast::<u16>())
                                            .read())
                                                as i32)
                                                as isize
                                                * 3,
                                        ))
                                        .cast::<u8>())
                                        .wrapping_offset(1))
                                        .read()) as i8),
                                        (-1i8),
                                    );
                                }
                                break 'l2;
                            }
                        }
                        ReturnToFieldFromFlyMapSelect();
                    } else {
                        SetMainCallback2(Some(CB2_ReturnToPartyMenuFromFlyMap));
                    }
                    if ((((&raw mut sFlyMap).cast::<u8>().cast::<*mut u8>()).read()) as usize)
                        != 0usize
                    {
                        Free(((&raw mut sFlyMap).cast::<u8>().cast::<*mut u8>()).read());
                        ((&raw mut sFlyMap).cast::<u8>().cast::<*mut u8>())
                            .write(core::ptr::null_mut());
                    }
                    FreeAllWindowBuffers();
                }
                break 'l1;
            }
        }
    }
}
