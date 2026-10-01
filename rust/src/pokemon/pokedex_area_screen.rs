//! Translated from `src/pokedex_area_screen.c` by tools/rustport/c2rs.py.
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
    dead_code,
    unused_assignments
)]

use crate::agb_main::gMain;
use crate::bg::{ChangeBgY, HideBg, SetBgAttribute, ShowBg};
#[allow(unused_imports)]
use crate::c::*;
#[allow(unused_imports)]
use crate::consts::*;
use crate::event_data::{FlagGet, VarGet};
use crate::gpu_regs::{SetGpuReg, SetGpuRegBits};
use crate::load_save::{gSaveBlock1Ptr, gSaveBlock2Ptr};
use crate::menu::{DecompressAndCopyTileDataToVram, FreeTempTileDataBuffersIfPossible};
use crate::overworld::Overworld_GetMapHeaderByGroupAndId;
use crate::palette::{BeginNormalPaletteFade, gPaletteFade};
use crate::pokedex_area_region_map::{
    FreePokedexAreaMapBgNum, PokedexAreaMapChangeBgY, TryShowPokedexAreaMap,
};
use crate::region_map::{
    CorrectSpecialMapSecId, CreateRegionMapPlayerIcon, GetRegionMapSecIdAt,
    PokedexAreaScreen_UpdateRegionMapVariablesAndVideoRegs, ShowRegionMapForPokedexAreaScreen,
};
use crate::roamer::GetRoamerLocation;
use crate::sound::PlaySE;
use crate::sprite::gSprites;
use crate::sprite::{
    FreeAllSpritePalettes, FreeSpritePaletteByTag, FreeSpriteTilesByTag, ResetSpriteData,
};
use crate::string_util::StringFill;
use crate::task::DestroyTask;
use crate::task::{task_get, task_set, task_set_func};
#[allow(unused_imports)]
use crate::types::*;
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
/// `LoadBgTilemap` with this module's view of its types.
#[inline]
unsafe fn LoadBgTilemap(a0: u8, a1: *mut c_void, a2: u16, a3: u16) -> u16 {
    unsafe { crate::bg::LoadBgTilemap(a0, a1 as _, a2, a3) }
}
/// `LoadPokedexAreaMapGfx` with this module's view of its types.
#[inline]
unsafe fn LoadPokedexAreaMapGfx(a0: *mut PokedexAreaMapTemplate) {
    unsafe {
        crate::pokedex_area_region_map::LoadPokedexAreaMapGfx(a0 as _);
    }
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
// The C's names for task and sprite data slots.
const tState: usize = 0;
// Data tables (translate with cdata.py): sAreaGlow_Pal sAreaGlow_Gfx sSpeciesHiddenFromAreaScreen sMovingRegionMapSections sFeebasData sLandmarkData sAreaGlowTilemapMapping sPokedexAreaMapTemplate sAreaMarkerTiles sAreaMarkerSpriteSheet sAreaMarkerPalette sAreaMarkerSpritePalette sAreaMarkerOamData sAreaMarkerSpriteTemplate sAreaMarkerPalette sAreaMarkerTiles sAreaUnknownSpritePalette sAreaUnknownOamData sAreaUnknownSpriteTemplate

/// `__typeof__(*((__typeof__(sPokedexAreaScreen))0))`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct typeof___sPokedexAreaScreen_0_t {
    pub callback: Option<unsafe fn()>,
    pub prev: Option<unsafe fn()>,
    pub next: Option<unsafe fn()>,
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
static CreateAreaMarkerSprites_x: crate::global::Global<i16> = crate::global::Global::new(0);
static CreateAreaMarkerSprites_y: crate::global::Global<i16> = crate::global::Global::new(0);
static CreateAreaMarkerSprites_i: crate::global::Global<i16> = crate::global::Global::new(0);
static CreateAreaMarkerSprites_mapSecId: crate::global::Global<i16> = crate::global::Global::new(0);
static CreateAreaMarkerSprites_numSprites: crate::global::Global<i16> =
    crate::global::Global::new(0);

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
/// `LZ77UnCompWram` with this module's view of its types.
#[inline]
unsafe fn LZ77UnCompWram(a0: *mut u32, a1: *mut c_void) {
    unsafe {
        crate::syscall::LZ77UnCompWram(a0 as _, a1 as _);
    }
}

unsafe fn ResetDrawAreaGlowState() {
    (*sPokedexAreaScreen).drawAreaGlowState = 0;
}
unsafe fn DrawAreaGlow() -> u8 {
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
                    &raw mut (*(&raw const crate::palette::gPlttBufferUnfaded)
                        .cast::<CArray<u16, 512>>()
                        .cast_mut())[160] as *mut c_void,
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
    TRUE
}
unsafe fn FindMapsWithMon(species: u16) {
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
        for i in 0..1u16 {
            if sSpeciesHiddenFromAreaScreen[i] == species {
                return;
            }
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
        while (*(&raw const crate::data::wild_encounter::gWildMonHeaders)
            .cast::<CArray<WildPokemonHeader, 0>>())[i]
            .mapGroup
            != 255
        {
            if MapHasSpecies(
                (&raw const (*(&raw const crate::data::wild_encounter::gWildMonHeaders)
                    .cast::<CArray<WildPokemonHeader, 0>>())[i])
                    .cast_mut(),
                species,
            ) != 0
            {
                match (*(&raw const crate::data::wild_encounter::gWildMonHeaders).cast::<CArray<
                    WildPokemonHeader,
                    0,
                >>(
                ))[i]
                    .mapGroup
                {
                    0 => {
                        SetAreaHasMon(
                            (*(&raw const crate::data::wild_encounter::gWildMonHeaders)
                                .cast::<CArray<WildPokemonHeader, 0>>())[i]
                                .mapGroup as u16,
                            (*(&raw const crate::data::wild_encounter::gWildMonHeaders)
                                .cast::<CArray<WildPokemonHeader, 0>>())[i]
                                .mapNum as u16,
                        );
                    }
                    24 | 26 => {
                        SetSpecialMapHasMon(
                            (*(&raw const crate::data::wild_encounter::gWildMonHeaders)
                                .cast::<CArray<WildPokemonHeader, 0>>())[i]
                                .mapGroup as u16,
                            (*(&raw const crate::data::wild_encounter::gWildMonHeaders)
                                .cast::<CArray<WildPokemonHeader, 0>>())[i]
                                .mapNum as u16,
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
unsafe fn SetAreaHasMon(mapGroup: u16, mapNum: u16) {
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
unsafe fn SetSpecialMapHasMon(mapGroup: u16, mapNum: u16) {
    let mut i: i32 = 0;
    if (*sPokedexAreaScreen).numSpecialAreas < MAX_AREA_MARKERS {
        let regionMapSectionId: u16 = GetRegionMapSectionId(mapGroup as u8, mapNum as u8);
        if regionMapSectionId < MAPSEC_NONE {
            for i in 0..3i32 {
                if regionMapSectionId == sMovingRegionMapSections[i] {
                    return;
                }
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
unsafe fn GetRegionMapSectionId(mapGroup: u8, mapNum: u8) -> u16 {
    (*Overworld_GetMapHeaderByGroupAndId(mapGroup as u16, mapNum as u16)).regionMapSectionId as u16
}
unsafe fn MapHasSpecies(info: *mut WildPokemonHeader, species: u16) -> u8 {
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
    FALSE
}
unsafe fn MonListHasSpecies(info: *mut WildPokemonInfo, species: u16, size: u16) -> u8 {
    if !info.is_null() {
        for i in 0..size {
            if (*(*info).wildPokemon.at(i)).species == species {
                return TRUE;
            }
        }
    }
    FALSE
}
unsafe fn BuildAreaGlowTilemap() {
    let mut j: u16 = 0;
    for i in 0..640u16 {
        (*sPokedexAreaScreen).areaGlowTilemap[i] = 0;
    }
    let mut i: u16 = 0;
    while i < (*sPokedexAreaScreen).numOverworldAreas {
        j = 0;
        for y in 0..AREA_SCREEN_HEIGHT {
            for x in 0..(AREA_SCREEN_WIDTH as u16) {
                if GetRegionMapSecIdAt(x, y)
                    == (*sPokedexAreaScreen).overworldAreasWithMons[i].regionMapSectionId
                {
                    (*sPokedexAreaScreen).areaGlowTilemap[j] = GLOW_FULL;
                }
                j += 1;
            }
        }
        i += 1;
    }
    j = 0;
    for y in 0..AREA_SCREEN_HEIGHT {
        for x in 0..(AREA_SCREEN_WIDTH as u16) {
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
        }
    }
    for i in 0..640u16 {
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
    }
}
unsafe fn StartAreaGlow() {
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
unsafe fn DoAreaGlow() {
    let mut x: u16 = 0;
    let mut y: u16 = 0;
    let mut i: u16 = 0;
    if (*sPokedexAreaScreen).showingMarkers == 0 {
        if (*sPokedexAreaScreen).markerTimer == 0 {
            (*sPokedexAreaScreen).glowTimer += 1;
            if (*sPokedexAreaScreen).glowTimer as i32 & 1 != 0 {
                (*sPokedexAreaScreen).areaShadeBldArgLo =
                    ((*sPokedexAreaScreen).areaShadeBldArgLo + 4) & 0x7f;
            } else {
                (*sPokedexAreaScreen).areaShadeBldArgHi =
                    ((*sPokedexAreaScreen).areaShadeBldArgHi + 4) & 0x7f;
            }
            x = ((*(&raw const crate::trig::gSineTable).cast::<CArray<i16, 0>>())
                [(*sPokedexAreaScreen).areaShadeBldArgLo]
                >> 4) as u16;
            y = ((*(&raw const crate::trig::gSineTable).cast::<CArray<i16, 0>>())
                [(*sPokedexAreaScreen).areaShadeBldArgHi]
                >> 4) as u16;
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
pub unsafe fn ShowPokedexAreaScreen(species: u16, screenSwitchState: *mut u8) {
    sPokedexAreaScreen = AllocZeroed(5564) as *mut typeof___sPokedexAreaScreen_0_t;
    (*sPokedexAreaScreen).species = species;
    (*sPokedexAreaScreen).screenSwitchState = screenSwitchState;
    *screenSwitchState = 0;
    let taskId: u8 = CreateTask(Some(Task_ShowPokedexAreaScreen), 0);
    task_set(taskId, tState, 0);
}
pub(crate) unsafe fn Task_ShowPokedexAreaScreen(taskId: u8) {
    match task_get(taskId, tState) {
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
            task_set_func(taskId, Some(Task_HandlePokedexAreaScreenInput));
            task_set(taskId, tState, 0);
            return;
        }
        _ => {}
    }
    task_set(taskId, tState, task_get(taskId, tState) + 1);
}
pub(crate) unsafe fn Task_HandlePokedexAreaScreenInput(taskId: u8) {
    DoAreaGlow();
    'l1: {
        let sw1: i16 = task_get(taskId, tState);
        let matched = sw1 == 0 || sw1 == 1 || sw1 == 2 || sw1 == 3;
        let mut fall = false;
        if !matched {
            fall = true;
            task_set(taskId, tState, 0);
        }
        if fall || sw1 == 0 {
            if gPaletteFade.active() != 0 {
                return;
            }
            break 'l1;
        }
        if sw1 == 1 {
            if gMain.newKeys as i32 & B_BUTTON != 0 {
                task_set(taskId, 1, 1);
                PlaySE(SE_PC_OFF);
            } else if gMain.newKeys as i32 & DPAD_RIGHT != 0
                || gMain.newKeys as i32 & R_BUTTON != 0
                    && (*gSaveBlock2Ptr).optionsButtonMode == OPTIONS_BUTTON_MODE_LR
            {
                task_set(taskId, 1, 2);
                PlaySE(SE_DEX_PAGE);
            } else {
                return;
            }
            break 'l1;
        }
        if sw1 == 2 {
            BeginNormalPaletteFade(0xffffffeb, 0, 0, 16, 0);
            break 'l1;
        }
        if sw1 == 3 {
            if gPaletteFade.active() != 0 {
                return;
            }
            DestroyAreaScreenSprites();
            *(*sPokedexAreaScreen).screenSwitchState = task_get(taskId, 1) as u8;
            ResetPokedexAreaMapBg();
            DestroyTask(taskId);
            FreePokedexAreaMapBgNum();
            Free(sPokedexAreaScreen as *mut c_void);
            sPokedexAreaScreen = null_mut();
            return;
        }
    }
    task_set(taskId, tState, task_get(taskId, tState) + 1);
}
unsafe fn ResetPokedexAreaMapBg() {
    SetBgAttribute(3, BG_ATTR_CHARBASEINDEX, 0);
    SetBgAttribute(3, BG_ATTR_PALETTEMODE, 0);
}
unsafe fn CreateAreaMarkerSprites() {
    let mut spriteId: u8 = 0;
    LoadSpriteSheet((&raw const *sAreaMarkerSpriteSheet).cast_mut());
    LoadSpritePalette((&raw const *sAreaMarkerSpritePalette).cast_mut());
    CreateAreaMarkerSprites_numSprites.set(0);
    CreateAreaMarkerSprites_i.set(0);
    while (CreateAreaMarkerSprites_i.get() as i32) < (*sPokedexAreaScreen).numSpecialAreas as i32 {
        CreateAreaMarkerSprites_mapSecId.set(
            (*sPokedexAreaScreen).specialAreaRegionMapSectionIds[CreateAreaMarkerSprites_i.get()]
                as i16,
        );
        CreateAreaMarkerSprites_x.set(
            8 * ((*(&raw const crate::data::region_map::gRegionMapEntries)
                .cast::<CArray<RegionMapLocation, 0>>())[CreateAreaMarkerSprites_mapSecId.get()]
            .x as i16
                + 1)
                + 4,
        );
        CreateAreaMarkerSprites_y.set(
            8 * (*(&raw const crate::data::region_map::gRegionMapEntries)
                .cast::<CArray<RegionMapLocation, 0>>())[CreateAreaMarkerSprites_mapSecId.get()]
            .y as i16
                + 28,
        );
        CreateAreaMarkerSprites_x.set(
            CreateAreaMarkerSprites_x.get()
                + (4 * ((*(&raw const crate::data::region_map::gRegionMapEntries).cast::<CArray<
                    RegionMapLocation,
                    0,
                >>(
                ))[CreateAreaMarkerSprites_mapSecId.get()]
                .width as i16
                    - 1)),
        );
        CreateAreaMarkerSprites_y.set(
            CreateAreaMarkerSprites_y.get()
                + (4 * ((*(&raw const crate::data::region_map::gRegionMapEntries).cast::<CArray<
                    RegionMapLocation,
                    0,
                >>(
                ))[CreateAreaMarkerSprites_mapSecId.get()]
                .height as i16
                    - 1)),
        );
        spriteId = CreateSprite(
            (&raw const *sAreaMarkerSpriteTemplate).cast_mut(),
            CreateAreaMarkerSprites_x.get(),
            CreateAreaMarkerSprites_y.get(),
            0,
        );
        if spriteId != MAX_SPRITES {
            gSprites[spriteId].set_invisible(TRUE as u16);
            (*sPokedexAreaScreen).areaMarkerSprites[{
                let t1 = CreateAreaMarkerSprites_numSprites.get();
                CreateAreaMarkerSprites_numSprites
                    .set(CreateAreaMarkerSprites_numSprites.get() + 1);
                t1
            }] = &raw mut gSprites[spriteId];
        }
        CreateAreaMarkerSprites_i.set(CreateAreaMarkerSprites_i.get() + 1);
    }
    (*sPokedexAreaScreen).numAreaMarkerSprites = CreateAreaMarkerSprites_numSprites.get() as u16;
}
unsafe fn DestroyAreaScreenSprites() {
    FreeSpriteTilesByTag(TAG_AREA_MARKER);
    FreeSpritePaletteByTag(TAG_AREA_MARKER);
    let mut i: u16 = 0;
    while i < (*sPokedexAreaScreen).numAreaMarkerSprites {
        DestroySprite((*sPokedexAreaScreen).areaMarkerSprites[i]);
        i += 1;
    }
    FreeSpriteTilesByTag(TAG_AREA_UNKNOWN);
    FreeSpritePaletteByTag(TAG_AREA_UNKNOWN);
    for i in 0..3u16 {
        if !(*sPokedexAreaScreen).areaUnknownSprites[i].is_null() {
            DestroySprite((*sPokedexAreaScreen).areaUnknownSprites[i]);
        }
    }
}
unsafe fn LoadAreaUnknownGraphics() {
    let mut spriteSheet: SpriteSheet = zeroed();
    spriteSheet.data = (*sPokedexAreaScreen).areaUnknownGraphicsBuffer.as_mut_ptr() as *mut c_void;
    spriteSheet.size = 1536;
    spriteSheet.tag = TAG_AREA_UNKNOWN;
    LZ77UnCompWram(
        (*(&raw const crate::data::graphics::gPokedexAreaScreenAreaUnknown_Gfx)
            .cast::<CArray<u32, 0>>())
        .as_ptr()
        .cast_mut(),
        (*sPokedexAreaScreen).areaUnknownGraphicsBuffer.as_mut_ptr() as *mut c_void,
    );
    LoadSpriteSheet(&raw mut spriteSheet);
    LoadSpritePalette((&raw const *sAreaUnknownSpritePalette).cast_mut());
}
unsafe fn CreateAreaUnknownSprites() {
    if (*sPokedexAreaScreen).numOverworldAreas != 0 || (*sPokedexAreaScreen).numSpecialAreas != 0 {
        for i in 0..3u16 {
            (*sPokedexAreaScreen).areaUnknownSprites[i] = null_mut();
        }
    } else {
        for i in 0..3u16 {
            let spriteId: u8 = CreateSprite(
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
        }
    }
}
