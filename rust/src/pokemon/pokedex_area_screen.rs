//! Translated from `src/pokedex_area_screen.c` by tools/rustport/c2rs.py.
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
// Data tables (translate with cdata.py): sAreaGlow_Pal sAreaGlow_Gfx sSpeciesHiddenFromAreaScreen sMovingRegionMapSections sFeebasData sLandmarkData sAreaGlowTilemapMapping sPokedexAreaMapTemplate sAreaMarkerTiles sAreaMarkerSpriteSheet sAreaMarkerPalette sAreaMarkerSpritePalette sAreaMarkerOamData sAreaMarkerSpriteTemplate sAreaMarkerPalette sAreaMarkerTiles sAreaUnknownSpritePalette sAreaUnknownOamData sAreaUnknownSpriteTemplate

/// `__typeof__(*((__typeof__(sPokedexAreaScreen))0))`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct typeof___sPokedexAreaScreen_0_t {
    pub callback: Option<unsafe extern "C" fn()>,
    pub prev: Option<unsafe extern "C" fn()>,
    pub next: Option<unsafe extern "C" fn()>,
    pub state: u16,
    pub species: u16,
    pub overworldAreasWithMons: CArray<OverworldArea, 64>,
    pub numOverworldAreas: u16,
    pub numSpecialAreas: u16,
    pub drawAreaGlowState: u16,
    pub areaGlowTilemap: CArray<u16, 640>,
    pub markerTimer: u16,
    pub glowTimer: u16,
    pub areaShadeBldArgLo: u16,
    pub areaShadeBldArgHi: u16,
    pub showingMarkers: u8,
    pub markerFlashCounter: u8,
    pub specialAreaRegionMapSectionIds: CArray<u16, 32>,
    pub areaMarkerSprites: CArray<*mut Sprite, 32>,
    pub numAreaMarkerSprites: u16,
    pub alteringCaveCounter: u16,
    pub alteringCaveId: u16,
    pub screenSwitchState: *mut u8,
    pub regionMap: RegionMap,
    pub charBuffer: CArray<u8, 64>,
    pub areaUnknownSprites: CArray<*mut Sprite, 3>,
    pub areaUnknownGraphicsBuffer: CArray<u8, 1536>,
}

unsafe impl Sync for typeof___sPokedexAreaScreen_0_t {}

/// `struct OverworldArea`
#[repr(C, align(4))]
#[derive(Clone, Copy)]
pub struct OverworldArea {
    pub mapGroup: u8,
    pub mapNum: u8,
    pub regionMapSectionId: u16,
}

unsafe impl Sync for OverworldArea {}

#[cfg(target_arch = "arm")]
const _: () = {
    #[allow(unused_imports)]
    use core::mem::{offset_of, size_of};
    assert!(size_of::<typeof___sPokedexAreaScreen_0_t>() == 5564);
    assert!(offset_of!(typeof___sPokedexAreaScreen_0_t, callback) == 0);
    assert!(offset_of!(typeof___sPokedexAreaScreen_0_t, prev) == 4);
    assert!(offset_of!(typeof___sPokedexAreaScreen_0_t, next) == 8);
    assert!(offset_of!(typeof___sPokedexAreaScreen_0_t, state) == 12);
    assert!(offset_of!(typeof___sPokedexAreaScreen_0_t, species) == 14);
    assert!(offset_of!(typeof___sPokedexAreaScreen_0_t, overworldAreasWithMons) == 16);
    assert!(offset_of!(typeof___sPokedexAreaScreen_0_t, numOverworldAreas) == 272);
    assert!(offset_of!(typeof___sPokedexAreaScreen_0_t, numSpecialAreas) == 274);
    assert!(offset_of!(typeof___sPokedexAreaScreen_0_t, drawAreaGlowState) == 276);
    assert!(offset_of!(typeof___sPokedexAreaScreen_0_t, areaGlowTilemap) == 278);
    assert!(offset_of!(typeof___sPokedexAreaScreen_0_t, markerTimer) == 1558);
    assert!(offset_of!(typeof___sPokedexAreaScreen_0_t, glowTimer) == 1560);
    assert!(offset_of!(typeof___sPokedexAreaScreen_0_t, areaShadeBldArgLo) == 1562);
    assert!(offset_of!(typeof___sPokedexAreaScreen_0_t, areaShadeBldArgHi) == 1564);
    assert!(offset_of!(typeof___sPokedexAreaScreen_0_t, showingMarkers) == 1566);
    assert!(offset_of!(typeof___sPokedexAreaScreen_0_t, markerFlashCounter) == 1567);
    assert!(
        offset_of!(
            typeof___sPokedexAreaScreen_0_t,
            specialAreaRegionMapSectionIds
        ) == 1568
    );
    assert!(offset_of!(typeof___sPokedexAreaScreen_0_t, areaMarkerSprites) == 1632);
    assert!(offset_of!(typeof___sPokedexAreaScreen_0_t, numAreaMarkerSprites) == 1760);
    assert!(offset_of!(typeof___sPokedexAreaScreen_0_t, alteringCaveCounter) == 1762);
    assert!(offset_of!(typeof___sPokedexAreaScreen_0_t, alteringCaveId) == 1764);
    assert!(offset_of!(typeof___sPokedexAreaScreen_0_t, screenSwitchState) == 1768);
    assert!(offset_of!(typeof___sPokedexAreaScreen_0_t, regionMap) == 1772);
    assert!(offset_of!(typeof___sPokedexAreaScreen_0_t, charBuffer) == 3952);
    assert!(offset_of!(typeof___sPokedexAreaScreen_0_t, areaUnknownSprites) == 4016);
    assert!(offset_of!(typeof___sPokedexAreaScreen_0_t, areaUnknownGraphicsBuffer) == 4028);
    assert!(size_of::<OverworldArea>() == 4);
    assert!(offset_of!(OverworldArea, mapGroup) == 0);
    assert!(offset_of!(OverworldArea, mapNum) == 1);
    assert!(offset_of!(OverworldArea, regionMapSectionId) == 2);
};

const AREA_SCREEN_HEIGHT: u16 = 20;
const AREA_SCREEN_WIDTH: i32 = 32;
const GLOW_CORNER_BL: i32 = 32;
const GLOW_CORNER_BR: u16 = 128;
const GLOW_CORNER_TL: u16 = 16;
const GLOW_CORNER_TR: u16 = 64;
const GLOW_EDGE_B: u16 = 4;
const GLOW_EDGE_L: u16 = 2;
const GLOW_EDGE_R: i32 = 1;
const GLOW_EDGE_T: u16 = 8;
const GLOW_FULL: u16 = 65535;
const GLOW_PALETTE: i32 = 10;
const GLOW_TILE_FULL: u16 = 16;
const MAX_AREA_HIGHLIGHTS: u16 = 64;
const MAX_AREA_MARKERS: u16 = 32;
const TAG_AREA_MARKER: u16 = 2;
const TAG_AREA_UNKNOWN: u16 = 3;

static sAreaGlowTilemapMapping: Table<CArray<u8, 256>> =
    Table((&raw const crate::data::pokedex_area_screen::sAreaGlowTilemapMapping).cast());
static sAreaGlow_Gfx: Table<CArray<u32, 77>> =
    Table((&raw const crate::data::pokedex_area_screen::sAreaGlow_Gfx).cast());
static sAreaGlow_Pal: Table<CArray<u32, 8>> =
    Table((&raw const crate::data::pokedex_area_screen::sAreaGlow_Pal).cast());
static sAreaMarkerSpritePalette: Table<SpritePalette> =
    Table((&raw const crate::data::pokedex_area_screen::sAreaMarkerSpritePalette).cast());
static sAreaMarkerSpriteSheet: Table<SpriteSheet> =
    Table((&raw const crate::data::pokedex_area_screen::sAreaMarkerSpriteSheet).cast());
static sAreaMarkerSpriteTemplate: Table<SpriteTemplate> =
    Table((&raw const crate::data::pokedex_area_screen::sAreaMarkerSpriteTemplate).cast());
static sAreaUnknownSpritePalette: Table<SpritePalette> =
    Table((&raw const crate::data::pokedex_area_screen::sAreaUnknownSpritePalette).cast());
static sAreaUnknownSpriteTemplate: Table<SpriteTemplate> =
    Table((&raw const crate::data::pokedex_area_screen::sAreaUnknownSpriteTemplate).cast());
static sFeebasData: Table<CArray<CArray<u16, 3>, 2>> =
    Table((&raw const crate::data::pokedex_area_screen::sFeebasData).cast());
static sLandmarkData: Table<CArray<CArray<u16, 2>, 7>> =
    Table((&raw const crate::data::pokedex_area_screen::sLandmarkData).cast());
static sMovingRegionMapSections: Table<CArray<u16, 3>> =
    Table((&raw const crate::data::pokedex_area_screen::sMovingRegionMapSections).cast());
static sPokedexAreaMapTemplate: Table<PokedexAreaMapTemplate> =
    Table((&raw const crate::data::pokedex_area_screen::sPokedexAreaMapTemplate).cast());
static sSpeciesHiddenFromAreaScreen: Table<CArray<u16, 1>> =
    Table((&raw const crate::data::pokedex_area_screen::sSpeciesHiddenFromAreaScreen).cast());

#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sPokedexAreaScreen: *mut typeof___sPokedexAreaScreen_0_t = null_mut();
static mut CreateAreaMarkerSprites_x: i16 = 0;
static mut CreateAreaMarkerSprites_y: i16 = 0;
static mut CreateAreaMarkerSprites_i: i16 = 0;
static mut CreateAreaMarkerSprites_mapSecId: i16 = 0;
static mut CreateAreaMarkerSprites_numSprites: i16 = 0;

unsafe extern "C" {
    static mut gMain: Main;
    static mut gPaletteFade: PaletteFadeControl;
    static mut gPlttBufferUnfaded: CArray<u16, 512>;
    static gPokedexAreaScreenAreaUnknown_Gfx: CArray<u32, 0>;
    static gRegionMapEntries: CArray<RegionMapLocation, 0>;
    static mut gSaveBlock1Ptr: *mut SaveBlock1;
    static mut gSaveBlock2Ptr: *mut SaveBlock2;
    static gSineTable: CArray<i16, 0>;
    static mut gSprites: CArray<Sprite, 65>;
    static mut gTasks: CArray<Task, 0>;
    static gWildMonHeaders: CArray<WildPokemonHeader, 0>;
    fn AllocZeroed(a0: u32) -> *mut c_void;
    fn BeginNormalPaletteFade(a0: u32, a1: i8, a2: u8, a3: u8, a4: u16) -> u8;
    fn ChangeBgY(a0: u8, a1: i32, a2: u8) -> i32;
    fn CorrectSpecialMapSecId(a0: u16) -> u16;
    fn CpuSet(a0: *mut c_void, a1: *mut c_void, a2: u32);
    fn CreateRegionMapPlayerIcon(a0: u16, a1: u16);
    fn CreateSprite(a0: *mut SpriteTemplate, a1: i16, a2: i16, a3: u8) -> u8;
    fn CreateTask(a0: Option<unsafe extern "C" fn(u8)>, a1: u8) -> u8;
    fn DecompressAndCopyTileDataToVram(
        a0: u8,
        a1: *mut c_void,
        a2: u32,
        a3: u16,
        a4: u8,
    ) -> *mut c_void;
    fn DestroySprite(a0: *mut Sprite);
    fn DestroyTask(a0: u8);
    fn FlagGet(a0: u16) -> u8;
    fn Free(a0: *mut c_void);
    fn FreeAllSpritePalettes();
    fn FreePokedexAreaMapBgNum();
    fn FreeSpritePaletteByTag(a0: u16);
    fn FreeSpriteTilesByTag(a0: u16);
    fn FreeTempTileDataBuffersIfPossible() -> u8;
    fn GetRegionMapSecIdAt(a0: u16, a1: u16) -> u16;
    fn GetRoamerLocation(a0: *mut u8, a1: *mut u8);
    fn HideBg(a0: u8);
    fn LZ77UnCompWram(a0: *mut u32, a1: *mut c_void);
    fn LoadBgTilemap(a0: u8, a1: *mut c_void, a2: u16, a3: u16) -> u16;
    fn LoadPokedexAreaMapGfx(a0: *mut PokedexAreaMapTemplate);
    fn LoadSpritePalette(a0: *mut SpritePalette) -> u8;
    fn LoadSpriteSheet(a0: *mut SpriteSheet) -> u16;
    fn Overworld_GetMapHeaderByGroupAndId(a0: u16, a1: u16) -> *mut MapHeader;
    fn PlaySE(a0: u16);
    fn PokedexAreaMapChangeBgY(a0: u32);
    fn PokedexAreaScreen_UpdateRegionMapVariablesAndVideoRegs(a0: i16, a1: i16);
    fn ResetSpriteData();
    fn SetBgAttribute(a0: u8, a1: u8, a2: u8);
    fn SetGpuReg(a0: u8, a1: u16);
    fn SetGpuRegBits(a0: u8, a1: u16);
    fn ShowBg(a0: u8);
    fn ShowRegionMapForPokedexAreaScreen(a0: *mut RegionMap);
    fn StringFill(a0: *mut u8, a1: u8, a2: u16) -> *mut u8;
    fn TryShowPokedexAreaMap() -> u32;
    fn VarGet(a0: u16) -> u16;
}

pub(crate) unsafe extern "C" fn ResetDrawAreaGlowState() {
    (*sPokedexAreaScreen).drawAreaGlowState = 0;
}
pub(crate) unsafe extern "C" fn DrawAreaGlow() -> u8 {
    match (*sPokedexAreaScreen).drawAreaGlowState {
        0 => {
            FindMapsWithMon((*sPokedexAreaScreen).species);
        }
        1 => {
            BuildAreaGlowTilemap();
        }
        2 => {
            DecompressAndCopyTileDataToVram(
                2,
                sAreaGlow_Gfx.as_ptr().cast_mut() as *mut c_void,
                0,
                0,
                0,
            );
            LoadBgTilemap(
                2,
                (*sPokedexAreaScreen).areaGlowTilemap.as_mut_ptr() as *mut c_void,
                1280,
                0,
            );
        }
        3 => {
            if FreeTempTileDataBuffersIfPossible() == 0 {
                CpuSet(
                    sAreaGlow_Pal.as_ptr().cast_mut() as *mut c_void,
                    &raw mut gPlttBufferUnfaded[160] as *mut c_void,
                    0x4000008,
                );
                (*sPokedexAreaScreen).drawAreaGlowState += 1;
            }
            return TRUE;
        }
        4 => {
            ChangeBgY(2, -2048, BG_COORD_SET);
        }
        _ => {
            return FALSE;
        }
    }
    (*sPokedexAreaScreen).drawAreaGlowState += 1;
    return TRUE;
}
pub(crate) unsafe extern "C" fn FindMapsWithMon(species: u16) {
    let mut i: u16 = 0;
    let mut roamer: *mut Roamer = null_mut();
    (*sPokedexAreaScreen).alteringCaveCounter = 0;
    (*sPokedexAreaScreen).alteringCaveId = VarGet(VAR_ALTERING_CAVE_WILD_SET);
    if (*sPokedexAreaScreen).alteringCaveId >= NUM_ALTERING_CAVE_TABLES {
        (*sPokedexAreaScreen).alteringCaveId = 0;
    }
    roamer = &raw mut (*gSaveBlock1Ptr).roamer;
    if species != (*roamer).species {
        (*sPokedexAreaScreen).numOverworldAreas = 0;
        (*sPokedexAreaScreen).numSpecialAreas = 0;
        i = 0;
        while i < 1 {
            if sSpeciesHiddenFromAreaScreen[i] == species {
                return;
            }
            i += 1;
        }
        i = 0;
        while sFeebasData[i][0] != NUM_SPECIES {
            if species == sFeebasData[i][0] {
                match sFeebasData[i][1] {
                    0 => {
                        SetAreaHasMon(sFeebasData[i][1], sFeebasData[i][2]);
                    }
                    24 | 26 => {
                        SetSpecialMapHasMon(sFeebasData[i][1], sFeebasData[i][2]);
                    }
                    _ => {}
                }
            }
            i += 1;
        }
        i = 0;
        while gWildMonHeaders[i].mapGroup != 255 {
            if MapHasSpecies((&raw const gWildMonHeaders[i]).cast_mut(), species) != 0 {
                match gWildMonHeaders[i].mapGroup {
                    0 => {
                        SetAreaHasMon(
                            gWildMonHeaders[i].mapGroup as u16,
                            gWildMonHeaders[i].mapNum as u16,
                        );
                    }
                    24 | 26 => {
                        SetSpecialMapHasMon(
                            gWildMonHeaders[i].mapGroup as u16,
                            gWildMonHeaders[i].mapNum as u16,
                        );
                    }
                    _ => {}
                }
            }
            i += 1;
        }
    } else {
        (*sPokedexAreaScreen).numSpecialAreas = 0;
        if (*roamer).active != 0 {
            GetRoamerLocation(
                &raw mut (*sPokedexAreaScreen).overworldAreasWithMons[0].mapGroup,
                &raw mut (*sPokedexAreaScreen).overworldAreasWithMons[0].mapNum,
            );
            (*sPokedexAreaScreen).overworldAreasWithMons[0].regionMapSectionId =
                (*Overworld_GetMapHeaderByGroupAndId(
                    (*sPokedexAreaScreen).overworldAreasWithMons[0].mapGroup as u16,
                    (*sPokedexAreaScreen).overworldAreasWithMons[0].mapNum as u16,
                ))
                .regionMapSectionId as u16;
            (*sPokedexAreaScreen).numOverworldAreas = 1;
        } else {
            (*sPokedexAreaScreen).numOverworldAreas = 0;
        }
    }
}
pub(crate) unsafe extern "C" fn SetAreaHasMon(mapGroup: u16, mapNum: u16) {
    if (*sPokedexAreaScreen).numOverworldAreas < MAX_AREA_HIGHLIGHTS {
        (*sPokedexAreaScreen).overworldAreasWithMons[(*sPokedexAreaScreen).numOverworldAreas]
            .mapGroup = mapGroup as u8;
        (*sPokedexAreaScreen).overworldAreasWithMons[(*sPokedexAreaScreen).numOverworldAreas]
            .mapNum = mapNum as u8;
        (*sPokedexAreaScreen).overworldAreasWithMons[(*sPokedexAreaScreen).numOverworldAreas]
            .regionMapSectionId = CorrectSpecialMapSecId(
            (*Overworld_GetMapHeaderByGroupAndId(mapGroup, mapNum)).regionMapSectionId as u16,
        );
        (*sPokedexAreaScreen).numOverworldAreas += 1;
    }
}
pub(crate) unsafe extern "C" fn SetSpecialMapHasMon(mapGroup: u16, mapNum: u16) {
    let mut i: i32 = 0;
    if (*sPokedexAreaScreen).numSpecialAreas < MAX_AREA_MARKERS {
        let mut regionMapSectionId: u16 = GetRegionMapSectionId(mapGroup as u8, mapNum as u8);
        if regionMapSectionId < MAPSEC_NONE {
            i = 0;
            while i < 3 {
                if regionMapSectionId == sMovingRegionMapSections[i] {
                    return;
                }
                i += 1;
            }
            i = 0;
            while sLandmarkData[i][0] != MAPSEC_NONE {
                if regionMapSectionId == sLandmarkData[i][0] && FlagGet(sLandmarkData[i][1]) == 0 {
                    return;
                }
                i += 1;
            }
            i = 0;
            while i < (*sPokedexAreaScreen).numSpecialAreas as i32 {
                if (*sPokedexAreaScreen).specialAreaRegionMapSectionIds[i] == regionMapSectionId {
                    break;
                }
                i += 1;
            }
            if i == (*sPokedexAreaScreen).numSpecialAreas as i32 {
                (*sPokedexAreaScreen).specialAreaRegionMapSectionIds[i] = regionMapSectionId;
                (*sPokedexAreaScreen).numSpecialAreas += 1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn GetRegionMapSectionId(mapGroup: u8, mapNum: u8) -> u16 {
    return (*Overworld_GetMapHeaderByGroupAndId(mapGroup as u16, mapNum as u16)).regionMapSectionId
        as u16;
}
pub(crate) unsafe extern "C" fn MapHasSpecies(info: *mut WildPokemonHeader, species: u16) -> u8 {
    if GetRegionMapSectionId((*info).mapGroup, (*info).mapNum) == MAPSEC_ALTERING_CAVE {
        (*sPokedexAreaScreen).alteringCaveCounter += 1;
        if (*sPokedexAreaScreen).alteringCaveCounter as i32
            != (*sPokedexAreaScreen).alteringCaveId as i32 + 1
        {
            return FALSE;
        }
    }
    if MonListHasSpecies((*info).landMonsInfo, species, NUM_LAND_MONS_ENCOUNTER_SLOTS) != 0 {
        return TRUE;
    }
    if MonListHasSpecies(
        (*info).waterMonsInfo,
        species,
        NUM_WATER_MONS_ENCOUNTER_SLOTS,
    ) != 0
    {
        return TRUE;
    }
    if MonListHasSpecies(
        (*info).fishingMonsInfo,
        species,
        NUM_LAND_MONS_ENCOUNTER_SLOTS,
    ) != 0
    {
        return TRUE;
    }
    if MonListHasSpecies(
        (*info).rockSmashMonsInfo,
        species,
        NUM_ROCK_SMASH_MONS_ENCOUNTER_SLOTS,
    ) != 0
    {
        return TRUE;
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn MonListHasSpecies(
    info: *mut WildPokemonInfo,
    species: u16,
    size: u16,
) -> u8 {
    let mut i: u16 = 0;
    if !info.is_null() {
        i = 0;
        while i < size {
            if (*(*info).wildPokemon.at(i)).species == species {
                return TRUE;
            }
            i += 1;
        }
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn BuildAreaGlowTilemap() {
    let mut i: u16 = 0;
    let mut y: u16 = 0;
    let mut x: u16 = 0;
    let mut j: u16 = 0;
    i = 0;
    while i < 640 {
        (*sPokedexAreaScreen).areaGlowTilemap[i] = 0;
        i += 1;
    }
    i = 0;
    while i < (*sPokedexAreaScreen).numOverworldAreas {
        j = 0;
        y = 0;
        while y < AREA_SCREEN_HEIGHT {
            x = 0;
            while x < AREA_SCREEN_WIDTH as u16 {
                if GetRegionMapSecIdAt(x, y)
                    == (*sPokedexAreaScreen).overworldAreasWithMons[i].regionMapSectionId
                {
                    (*sPokedexAreaScreen).areaGlowTilemap[j] = GLOW_FULL;
                }
                j += 1;
                x += 1;
            }
            y += 1;
        }
        i += 1;
    }
    j = 0;
    y = 0;
    while y < AREA_SCREEN_HEIGHT {
        x = 0;
        while x < AREA_SCREEN_WIDTH as u16 {
            if (*sPokedexAreaScreen).areaGlowTilemap[j] == GLOW_FULL {
                if x != 0 && (*sPokedexAreaScreen).areaGlowTilemap[j as i32 - 1] != GLOW_FULL {
                    (*sPokedexAreaScreen).areaGlowTilemap[j as i32 - 1] |= GLOW_EDGE_L;
                }
                if x != 31 && (*sPokedexAreaScreen).areaGlowTilemap[j as i32 + 1] != GLOW_FULL {
                    (*sPokedexAreaScreen).areaGlowTilemap[j as i32 + 1] |= 1;
                }
                if y != 0
                    && (*sPokedexAreaScreen).areaGlowTilemap[j as i32 - AREA_SCREEN_WIDTH]
                        != GLOW_FULL
                {
                    (*sPokedexAreaScreen).areaGlowTilemap[j as i32 - AREA_SCREEN_WIDTH] |=
                        GLOW_EDGE_T;
                }
                if y != 19
                    && (*sPokedexAreaScreen).areaGlowTilemap[j as i32 + AREA_SCREEN_WIDTH]
                        != GLOW_FULL
                {
                    (*sPokedexAreaScreen).areaGlowTilemap[j as i32 + AREA_SCREEN_WIDTH] |=
                        GLOW_EDGE_B;
                }
                if x != 0
                    && y != 0
                    && (*sPokedexAreaScreen).areaGlowTilemap[j as i32 - AREA_SCREEN_WIDTH - 1]
                        != GLOW_FULL
                {
                    (*sPokedexAreaScreen).areaGlowTilemap[j as i32 - AREA_SCREEN_WIDTH - 1] |=
                        GLOW_CORNER_TL;
                }
                if x != 31
                    && y != 0
                    && (*sPokedexAreaScreen).areaGlowTilemap[j as i32 - AREA_SCREEN_WIDTH + 1]
                        != GLOW_FULL
                {
                    (*sPokedexAreaScreen).areaGlowTilemap[j as i32 - AREA_SCREEN_WIDTH + 1] |=
                        GLOW_CORNER_TR;
                }
                if x != 0
                    && y != 19
                    && (*sPokedexAreaScreen).areaGlowTilemap[j as i32 + AREA_SCREEN_WIDTH - 1]
                        != GLOW_FULL
                {
                    (*sPokedexAreaScreen).areaGlowTilemap[j as i32 + 32 - 1] |= 32;
                }
                if x != 31
                    && y != 19
                    && (*sPokedexAreaScreen).areaGlowTilemap[j as i32 + AREA_SCREEN_WIDTH + 1]
                        != GLOW_FULL
                {
                    (*sPokedexAreaScreen).areaGlowTilemap[j as i32 + AREA_SCREEN_WIDTH + 1] |=
                        GLOW_CORNER_BR;
                }
            }
            j += 1;
            x += 1;
        }
        y += 1;
    }
    i = 0;
    while i < 640 {
        if (*sPokedexAreaScreen).areaGlowTilemap[i] == GLOW_FULL {
            (*sPokedexAreaScreen).areaGlowTilemap[i] = GLOW_TILE_FULL;
            (*sPokedexAreaScreen).areaGlowTilemap[i] |= 40960;
        } else if (*sPokedexAreaScreen).areaGlowTilemap[i] != 0 {
            if (*sPokedexAreaScreen).areaGlowTilemap[i] as i32 & GLOW_EDGE_L as i32 != 0 {
                (*sPokedexAreaScreen).areaGlowTilemap[i] &= 65487;
            }
            if (*sPokedexAreaScreen).areaGlowTilemap[i] as i32 & GLOW_EDGE_R != 0 {
                (*sPokedexAreaScreen).areaGlowTilemap[i] &= 65343;
            }
            if (*sPokedexAreaScreen).areaGlowTilemap[i] as i32 & GLOW_EDGE_T as i32 != 0 {
                (*sPokedexAreaScreen).areaGlowTilemap[i] &= 65455;
            }
            if (*sPokedexAreaScreen).areaGlowTilemap[i] as i32 & GLOW_EDGE_B as i32 != 0 {
                (*sPokedexAreaScreen).areaGlowTilemap[i] &= 65375;
            }
            (*sPokedexAreaScreen).areaGlowTilemap[i] =
                sAreaGlowTilemapMapping[(*sPokedexAreaScreen).areaGlowTilemap[i]] as u16;
            (*sPokedexAreaScreen).areaGlowTilemap[i] |= 40960;
        }
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn StartAreaGlow() {
    if (*sPokedexAreaScreen).numSpecialAreas != 0 && (*sPokedexAreaScreen).numOverworldAreas == 0 {
        (*sPokedexAreaScreen).showingMarkers = TRUE;
    } else {
        (*sPokedexAreaScreen).showingMarkers = FALSE;
    }
    (*sPokedexAreaScreen).markerTimer = 0;
    (*sPokedexAreaScreen).glowTimer = 0;
    (*sPokedexAreaScreen).areaShadeBldArgLo = 0;
    (*sPokedexAreaScreen).areaShadeBldArgHi = 64;
    (*sPokedexAreaScreen).markerFlashCounter = 1;
    SetGpuReg(REG_OFFSET_BLDCNT, 16196);
    SetGpuReg(REG_OFFSET_BLDALPHA, 4096);
    DoAreaGlow();
}
pub(crate) unsafe extern "C" fn DoAreaGlow() {
    let mut x: u16 = 0;
    let mut y: u16 = 0;
    let mut i: u16 = 0;
    if (*sPokedexAreaScreen).showingMarkers == 0 {
        if (*sPokedexAreaScreen).markerTimer == 0 {
            (*sPokedexAreaScreen).glowTimer += 1;
            if (*sPokedexAreaScreen).glowTimer as i32 & 1 != 0 {
                (*sPokedexAreaScreen).areaShadeBldArgLo =
                    (*sPokedexAreaScreen).areaShadeBldArgLo + 4 & 0x7f;
            } else {
                (*sPokedexAreaScreen).areaShadeBldArgHi =
                    (*sPokedexAreaScreen).areaShadeBldArgHi + 4 & 0x7f;
            }
            x = (gSineTable[(*sPokedexAreaScreen).areaShadeBldArgLo] >> 4) as u16;
            y = (gSineTable[(*sPokedexAreaScreen).areaShadeBldArgHi] >> 4) as u16;
            SetGpuReg(REG_OFFSET_BLDALPHA, y << 8 | x);
            (*sPokedexAreaScreen).markerTimer = 0;
            if (*sPokedexAreaScreen).glowTimer == 64 {
                (*sPokedexAreaScreen).glowTimer = 0;
                if (*sPokedexAreaScreen).numSpecialAreas != 0 {
                    (*sPokedexAreaScreen).showingMarkers = TRUE;
                }
            }
        } else {
            (*sPokedexAreaScreen).markerTimer -= 1;
        }
    } else {
        (*sPokedexAreaScreen).markerTimer += 1;
        if (*sPokedexAreaScreen).markerTimer > 12 {
            (*sPokedexAreaScreen).markerTimer = 0;
            (*sPokedexAreaScreen).markerFlashCounter += 1;
            i = 0;
            while i < (*sPokedexAreaScreen).numSpecialAreas {
                (*(*sPokedexAreaScreen).areaMarkerSprites[i])
                    .set_invisible((*sPokedexAreaScreen).markerFlashCounter as u16 & 1);
                i += 1;
            }
            if (*sPokedexAreaScreen).markerFlashCounter > 4 {
                (*sPokedexAreaScreen).markerFlashCounter = 1;
                if (*sPokedexAreaScreen).numOverworldAreas != 0 {
                    (*sPokedexAreaScreen).showingMarkers = FALSE;
                }
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ShowPokedexAreaScreen(species: u16, mut screenSwitchState: *mut u8) {
    let mut taskId: u8 = 0;
    sPokedexAreaScreen = AllocZeroed(5564) as *mut typeof___sPokedexAreaScreen_0_t;
    (*sPokedexAreaScreen).species = species;
    (*sPokedexAreaScreen).screenSwitchState = screenSwitchState;
    *screenSwitchState = 0;
    taskId = CreateTask(Some(Task_ShowPokedexAreaScreen), 0);
    gTasks[taskId].data[0] = 0;
}
pub(crate) unsafe extern "C" fn Task_ShowPokedexAreaScreen(taskId: u8) {
    match gTasks[taskId].data[0] {
        0 => {
            ResetSpriteData();
            FreeAllSpritePalettes();
            HideBg(3);
            HideBg(2);
            HideBg(0);
        }
        1 => {
            SetBgAttribute(3, BG_ATTR_CHARBASEINDEX, 3);
            LoadPokedexAreaMapGfx((&raw const *sPokedexAreaMapTemplate).cast_mut());
            StringFill(
                (*sPokedexAreaScreen).charBuffer.as_mut_ptr(),
                CHAR_SPACE,
                16,
            );
        }
        2 => {
            if TryShowPokedexAreaMap() == TRUE as u32 {
                return;
            }
            PokedexAreaMapChangeBgY(0xfffffff8);
        }
        3 => {
            ResetDrawAreaGlowState();
        }
        4 => {
            if DrawAreaGlow() != 0 {
                return;
            }
        }
        5 => {
            ShowRegionMapForPokedexAreaScreen(&raw mut (*sPokedexAreaScreen).regionMap);
            CreateRegionMapPlayerIcon(1, 1);
            PokedexAreaScreen_UpdateRegionMapVariablesAndVideoRegs(0, -8);
        }
        6 => {
            CreateAreaMarkerSprites();
        }
        7 => {
            LoadAreaUnknownGraphics();
        }
        8 => {
            CreateAreaUnknownSprites();
        }
        9 => {
            BeginNormalPaletteFade(0xffffffeb, 0, 16, 0, 0);
        }
        10 => {
            SetGpuReg(REG_OFFSET_BLDCNT, 16193);
            StartAreaGlow();
            ShowBg(2);
            ShowBg(3);
            SetGpuRegBits(REG_OFFSET_DISPCNT, DISPCNT_OBJ_ON);
        }
        11 => {
            gTasks[taskId].func = Some(Task_HandlePokedexAreaScreenInput);
            gTasks[taskId].data[0] = 0;
            return;
        }
        _ => {}
    }
    gTasks[taskId].data[0] += 1;
}
pub(crate) unsafe extern "C" fn Task_HandlePokedexAreaScreenInput(taskId: u8) {
    DoAreaGlow();
    'l1: {
        let sw1: i16 = gTasks[taskId].data[0];
        let matched = sw1 == 0 || sw1 == 1 || sw1 == 2 || sw1 == 3;
        let mut fall = false;
        if !matched {
            fall = true;
            gTasks[taskId].data[0] = 0;
        }
        if fall || sw1 == 0 {
            fall = true;
            if gPaletteFade.active() != 0 {
                return;
            }
            break 'l1;
        }
        if sw1 == 1 {
            fall = true;
            if gMain.newKeys as i32 & B_BUTTON != 0 {
                gTasks[taskId].data[1] = 1;
                PlaySE(SE_PC_OFF);
            } else if gMain.newKeys as i32 & DPAD_RIGHT != 0
                || gMain.newKeys as i32 & R_BUTTON != 0
                    && (*gSaveBlock2Ptr).optionsButtonMode == OPTIONS_BUTTON_MODE_LR
            {
                gTasks[taskId].data[1] = 2;
                PlaySE(SE_DEX_PAGE);
            } else {
                return;
            }
            break 'l1;
        }
        if sw1 == 2 {
            fall = true;
            BeginNormalPaletteFade(0xffffffeb, 0, 0, 16, 0);
            break 'l1;
        }
        if sw1 == 3 {
            fall = true;
            if gPaletteFade.active() != 0 {
                return;
            }
            DestroyAreaScreenSprites();
            *(*sPokedexAreaScreen).screenSwitchState = gTasks[taskId].data[1] as u8;
            ResetPokedexAreaMapBg();
            DestroyTask(taskId);
            FreePokedexAreaMapBgNum();
            Free(sPokedexAreaScreen as *mut c_void);
            sPokedexAreaScreen = null_mut();
            return;
        }
    }
    gTasks[taskId].data[0] += 1;
}
pub(crate) unsafe extern "C" fn ResetPokedexAreaMapBg() {
    SetBgAttribute(3, BG_ATTR_CHARBASEINDEX, 0);
    SetBgAttribute(3, BG_ATTR_PALETTEMODE, 0);
}
pub(crate) unsafe extern "C" fn CreateAreaMarkerSprites() {
    let mut spriteId: u8 = 0;
    LoadSpriteSheet((&raw const *sAreaMarkerSpriteSheet).cast_mut());
    LoadSpritePalette((&raw const *sAreaMarkerSpritePalette).cast_mut());
    CreateAreaMarkerSprites_numSprites = 0;
    CreateAreaMarkerSprites_i = 0;
    while (CreateAreaMarkerSprites_i as i32) < (*sPokedexAreaScreen).numSpecialAreas as i32 {
        CreateAreaMarkerSprites_mapSecId =
            (*sPokedexAreaScreen).specialAreaRegionMapSectionIds[CreateAreaMarkerSprites_i] as i16;
        CreateAreaMarkerSprites_x =
            8 * (gRegionMapEntries[CreateAreaMarkerSprites_mapSecId].x as i16 + 1) + 4;
        CreateAreaMarkerSprites_y =
            8 * gRegionMapEntries[CreateAreaMarkerSprites_mapSecId].y as i16 + 28;
        CreateAreaMarkerSprites_x +=
            4 * (gRegionMapEntries[CreateAreaMarkerSprites_mapSecId].width as i16 - 1);
        CreateAreaMarkerSprites_y +=
            4 * (gRegionMapEntries[CreateAreaMarkerSprites_mapSecId].height as i16 - 1);
        spriteId = CreateSprite(
            (&raw const *sAreaMarkerSpriteTemplate).cast_mut(),
            CreateAreaMarkerSprites_x,
            CreateAreaMarkerSprites_y,
            0,
        );
        if spriteId != MAX_SPRITES {
            gSprites[spriteId].set_invisible(TRUE as u16);
            (*sPokedexAreaScreen).areaMarkerSprites[{
                let t1 = CreateAreaMarkerSprites_numSprites;
                CreateAreaMarkerSprites_numSprites += 1;
                t1
            }] = &raw mut gSprites[spriteId];
        }
        CreateAreaMarkerSprites_i += 1;
    }
    (*sPokedexAreaScreen).numAreaMarkerSprites = CreateAreaMarkerSprites_numSprites as u16;
}
pub(crate) unsafe extern "C" fn DestroyAreaScreenSprites() {
    let mut i: u16 = 0;
    FreeSpriteTilesByTag(TAG_AREA_MARKER);
    FreeSpritePaletteByTag(TAG_AREA_MARKER);
    i = 0;
    while i < (*sPokedexAreaScreen).numAreaMarkerSprites {
        DestroySprite((*sPokedexAreaScreen).areaMarkerSprites[i]);
        i += 1;
    }
    FreeSpriteTilesByTag(TAG_AREA_UNKNOWN);
    FreeSpritePaletteByTag(TAG_AREA_UNKNOWN);
    i = 0;
    while i < 3 {
        if !(*sPokedexAreaScreen).areaUnknownSprites[i].is_null() {
            DestroySprite((*sPokedexAreaScreen).areaUnknownSprites[i]);
        }
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn LoadAreaUnknownGraphics() {
    let mut spriteSheet: SpriteSheet = zeroed();
    spriteSheet.data = (*sPokedexAreaScreen).areaUnknownGraphicsBuffer.as_mut_ptr() as *mut c_void;
    spriteSheet.size = 1536;
    spriteSheet.tag = TAG_AREA_UNKNOWN;
    LZ77UnCompWram(
        gPokedexAreaScreenAreaUnknown_Gfx.as_ptr().cast_mut(),
        (*sPokedexAreaScreen).areaUnknownGraphicsBuffer.as_mut_ptr() as *mut c_void,
    );
    LoadSpriteSheet(&raw mut spriteSheet);
    LoadSpritePalette((&raw const *sAreaUnknownSpritePalette).cast_mut());
}
pub(crate) unsafe extern "C" fn CreateAreaUnknownSprites() {
    let mut i: u16 = 0;
    if (*sPokedexAreaScreen).numOverworldAreas != 0 || (*sPokedexAreaScreen).numSpecialAreas != 0 {
        i = 0;
        while i < 3 {
            (*sPokedexAreaScreen).areaUnknownSprites[i] = null_mut();
            i += 1;
        }
    } else {
        i = 0;
        while i < 3 {
            let mut spriteId: u8 = CreateSprite(
                (&raw const *sAreaUnknownSpriteTemplate).cast_mut(),
                i as i16 * 32 + 160,
                140,
                0,
            );
            if spriteId != MAX_SPRITES {
                gSprites[spriteId]
                    .oam
                    .set_tileNum(gSprites[spriteId].oam.tileNum() + i * 16);
                (*sPokedexAreaScreen).areaUnknownSprites[i] = &raw mut gSprites[spriteId];
            } else {
                (*sPokedexAreaScreen).areaUnknownSprites[i] = null_mut();
            }
            i += 1;
        }
    }
}
