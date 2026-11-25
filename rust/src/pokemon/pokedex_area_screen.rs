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
fn LoadPokedexAreaMapGfx(a0: *mut PokedexAreaMapTemplate) {
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

#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub(crate) static sPokedexAreaScreen: crate::global::Global<*mut typeof___sPokedexAreaScreen_0_t> =
    crate::global::Global::new(null_mut());
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
fn CpuSet(a0: *mut c_void, a1: *mut c_void, a2: u32) {
    unsafe {
        crate::syscall::CpuSet(a0 as _, a1 as _, a2);
    }
}
/// `LZ77UnCompWram` with this module's view of its types.
#[inline]
fn LZ77UnCompWram(a0: *mut u32, a1: *mut c_void) {
    unsafe {
        crate::syscall::LZ77UnCompWram(a0 as _, a1 as _);
    }
}

unsafe fn ResetDrawAreaGlowState() {
    (*crate::gl!(sPokedexAreaScreen)).drawAreaGlowState = 0;
}
unsafe fn DrawAreaGlow() -> u8 {
    match (*crate::gl!(sPokedexAreaScreen)).drawAreaGlowState {
        0 => {
            FindMapsWithMon((*crate::gl!(sPokedexAreaScreen)).species);
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
                (*crate::gl!(sPokedexAreaScreen))
                    .areaGlowTilemap
                    .as_mut_ptr() as *mut c_void,
                1280,
                0,
            );
        }
        3 => {
            if FreeTempTileDataBuffersIfPossible() == 0 {
                CpuSet(
                    sAreaGlow_Pal.as_ptr().cast_mut() as *mut c_void,
                    &raw mut (*(crate::gl!(&raw @at crate::palette::gPlttBufferUnfaded;)
                        .cast_const())
                    .cast::<CArray<u16, 512>>()
                    .cast_mut())[160] as *mut c_void,
                    0x4000008,
                );
                (*crate::gl!(sPokedexAreaScreen)).drawAreaGlowState += 1;
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
    (*crate::gl!(sPokedexAreaScreen)).drawAreaGlowState += 1;
    TRUE
}
unsafe fn FindMapsWithMon(species: u16) {
    let mut i: u16 = 0;
    let mut roamer: *mut Roamer = null_mut();
    (*crate::gl!(sPokedexAreaScreen)).alteringCaveCounter = 0;
    (*crate::gl!(sPokedexAreaScreen)).alteringCaveId = VarGet(VAR_ALTERING_CAVE_WILD_SET);
    if (*crate::gl!(sPokedexAreaScreen)).alteringCaveId >= NUM_ALTERING_CAVE_TABLES {
        (*crate::gl!(sPokedexAreaScreen)).alteringCaveId = 0;
    }
    roamer = &raw mut (*crate::gl!(gSaveBlock1Ptr)).roamer;
    if species != (*roamer).species {
        (*crate::gl!(sPokedexAreaScreen)).numOverworldAreas = 0;
        (*crate::gl!(sPokedexAreaScreen)).numSpecialAreas = 0;
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
        (*crate::gl!(sPokedexAreaScreen)).numSpecialAreas = 0;
        if (*roamer).active != 0 {
            GetRoamerLocation(
                &raw mut (*crate::gl!(sPokedexAreaScreen)).overworldAreasWithMons[0].mapGroup,
                &raw mut (*crate::gl!(sPokedexAreaScreen)).overworldAreasWithMons[0].mapNum,
            );
            (*crate::gl!(sPokedexAreaScreen)).overworldAreasWithMons[0].regionMapSectionId =
                (*Overworld_GetMapHeaderByGroupAndId(
                    (*crate::gl!(sPokedexAreaScreen)).overworldAreasWithMons[0].mapGroup as u16,
                    (*crate::gl!(sPokedexAreaScreen)).overworldAreasWithMons[0].mapNum as u16,
                ))
                .regionMapSectionId as u16;
            (*crate::gl!(sPokedexAreaScreen)).numOverworldAreas = 1;
        } else {
            (*crate::gl!(sPokedexAreaScreen)).numOverworldAreas = 0;
        }
    }
}
unsafe fn SetAreaHasMon(mapGroup: u16, mapNum: u16) {
    if (*crate::gl!(sPokedexAreaScreen)).numOverworldAreas < MAX_AREA_HIGHLIGHTS {
        (*crate::gl!(sPokedexAreaScreen)).overworldAreasWithMons
            [(*crate::gl!(sPokedexAreaScreen)).numOverworldAreas]
            .mapGroup = mapGroup as u8;
        (*crate::gl!(sPokedexAreaScreen)).overworldAreasWithMons
            [(*crate::gl!(sPokedexAreaScreen)).numOverworldAreas]
            .mapNum = mapNum as u8;
        (*crate::gl!(sPokedexAreaScreen)).overworldAreasWithMons
            [(*crate::gl!(sPokedexAreaScreen)).numOverworldAreas]
            .regionMapSectionId = CorrectSpecialMapSecId(
            (*Overworld_GetMapHeaderByGroupAndId(mapGroup, mapNum)).regionMapSectionId as u16,
        );
        (*crate::gl!(sPokedexAreaScreen)).numOverworldAreas += 1;
    }
}
unsafe fn SetSpecialMapHasMon(mapGroup: u16, mapNum: u16) {
    let mut i: i32 = 0;
    if (*crate::gl!(sPokedexAreaScreen)).numSpecialAreas < MAX_AREA_MARKERS {
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
            while i < (*crate::gl!(sPokedexAreaScreen)).numSpecialAreas as i32 {
                if (*crate::gl!(sPokedexAreaScreen)).specialAreaRegionMapSectionIds[i]
                    == regionMapSectionId
                {
                    break;
                }
                i += 1;
            }
            if i == (*crate::gl!(sPokedexAreaScreen)).numSpecialAreas as i32 {
                (*crate::gl!(sPokedexAreaScreen)).specialAreaRegionMapSectionIds[i] =
                    regionMapSectionId;
                (*crate::gl!(sPokedexAreaScreen)).numSpecialAreas += 1;
            }
        }
    }
}
unsafe fn GetRegionMapSectionId(mapGroup: u8, mapNum: u8) -> u16 {
    (*Overworld_GetMapHeaderByGroupAndId(mapGroup as u16, mapNum as u16)).regionMapSectionId as u16
}
unsafe fn MapHasSpecies(info: *mut WildPokemonHeader, species: u16) -> u8 {
    if GetRegionMapSectionId((*info).mapGroup, (*info).mapNum) == MAPSEC_ALTERING_CAVE {
        (*crate::gl!(sPokedexAreaScreen)).alteringCaveCounter += 1;
        if (*crate::gl!(sPokedexAreaScreen)).alteringCaveCounter as i32
            != (*crate::gl!(sPokedexAreaScreen)).alteringCaveId as i32 + 1
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
        (*crate::gl!(sPokedexAreaScreen)).areaGlowTilemap[i] = 0;
    }
    let mut i: u16 = 0;
    while i < (*crate::gl!(sPokedexAreaScreen)).numOverworldAreas {
        j = 0;
        for y in 0..AREA_SCREEN_HEIGHT {
            for x in 0..(AREA_SCREEN_WIDTH as u16) {
                if GetRegionMapSecIdAt(x, y)
                    == (*crate::gl!(sPokedexAreaScreen)).overworldAreasWithMons[i]
                        .regionMapSectionId
                {
                    (*crate::gl!(sPokedexAreaScreen)).areaGlowTilemap[j] = GLOW_FULL;
                }
                j += 1;
            }
        }
        i += 1;
    }
    j = 0;
    for y in 0..AREA_SCREEN_HEIGHT {
        for x in 0..(AREA_SCREEN_WIDTH as u16) {
            if (*crate::gl!(sPokedexAreaScreen)).areaGlowTilemap[j] == GLOW_FULL {
                if x != 0
                    && (*crate::gl!(sPokedexAreaScreen)).areaGlowTilemap[j as i32 - 1] != GLOW_FULL
                {
                    (*crate::gl!(sPokedexAreaScreen)).areaGlowTilemap[j as i32 - 1] |= GLOW_EDGE_L;
                }
                if x != 31
                    && (*crate::gl!(sPokedexAreaScreen)).areaGlowTilemap[j as i32 + 1] != GLOW_FULL
                {
                    (*crate::gl!(sPokedexAreaScreen)).areaGlowTilemap[j as i32 + 1] |= 1;
                }
                if y != 0
                    && (*crate::gl!(sPokedexAreaScreen)).areaGlowTilemap
                        [j as i32 - AREA_SCREEN_WIDTH]
                        != GLOW_FULL
                {
                    (*crate::gl!(sPokedexAreaScreen)).areaGlowTilemap
                        [j as i32 - AREA_SCREEN_WIDTH] |= GLOW_EDGE_T;
                }
                if y != 19
                    && (*crate::gl!(sPokedexAreaScreen)).areaGlowTilemap
                        [j as i32 + AREA_SCREEN_WIDTH]
                        != GLOW_FULL
                {
                    (*crate::gl!(sPokedexAreaScreen)).areaGlowTilemap
                        [j as i32 + AREA_SCREEN_WIDTH] |= GLOW_EDGE_B;
                }
                if x != 0
                    && y != 0
                    && (*crate::gl!(sPokedexAreaScreen)).areaGlowTilemap
                        [j as i32 - AREA_SCREEN_WIDTH - 1]
                        != GLOW_FULL
                {
                    (*crate::gl!(sPokedexAreaScreen)).areaGlowTilemap
                        [j as i32 - AREA_SCREEN_WIDTH - 1] |= GLOW_CORNER_TL;
                }
                if x != 31
                    && y != 0
                    && (*crate::gl!(sPokedexAreaScreen)).areaGlowTilemap
                        [j as i32 - AREA_SCREEN_WIDTH + 1]
                        != GLOW_FULL
                {
                    (*crate::gl!(sPokedexAreaScreen)).areaGlowTilemap
                        [j as i32 - AREA_SCREEN_WIDTH + 1] |= GLOW_CORNER_TR;
                }
                if x != 0
                    && y != 19
                    && (*crate::gl!(sPokedexAreaScreen)).areaGlowTilemap
                        [j as i32 + AREA_SCREEN_WIDTH - 1]
                        != GLOW_FULL
                {
                    (*crate::gl!(sPokedexAreaScreen)).areaGlowTilemap[j as i32 + 32 - 1] |= 32;
                }
                if x != 31
                    && y != 19
                    && (*crate::gl!(sPokedexAreaScreen)).areaGlowTilemap
                        [j as i32 + AREA_SCREEN_WIDTH + 1]
                        != GLOW_FULL
                {
                    (*crate::gl!(sPokedexAreaScreen)).areaGlowTilemap
                        [j as i32 + AREA_SCREEN_WIDTH + 1] |= GLOW_CORNER_BR;
                }
            }
            j += 1;
        }
    }
    for i in 0..640u16 {
        if (*crate::gl!(sPokedexAreaScreen)).areaGlowTilemap[i] == GLOW_FULL {
            (*crate::gl!(sPokedexAreaScreen)).areaGlowTilemap[i] = GLOW_TILE_FULL;
            (*crate::gl!(sPokedexAreaScreen)).areaGlowTilemap[i] |= 40960;
        } else if (*crate::gl!(sPokedexAreaScreen)).areaGlowTilemap[i] != 0 {
            if (*crate::gl!(sPokedexAreaScreen)).areaGlowTilemap[i] as i32 & GLOW_EDGE_L as i32 != 0
            {
                (*crate::gl!(sPokedexAreaScreen)).areaGlowTilemap[i] &= 65487;
            }
            if (*crate::gl!(sPokedexAreaScreen)).areaGlowTilemap[i] as i32 & GLOW_EDGE_R != 0 {
                (*crate::gl!(sPokedexAreaScreen)).areaGlowTilemap[i] &= 65343;
            }
            if (*crate::gl!(sPokedexAreaScreen)).areaGlowTilemap[i] as i32 & GLOW_EDGE_T as i32 != 0
            {
                (*crate::gl!(sPokedexAreaScreen)).areaGlowTilemap[i] &= 65455;
            }
            if (*crate::gl!(sPokedexAreaScreen)).areaGlowTilemap[i] as i32 & GLOW_EDGE_B as i32 != 0
            {
                (*crate::gl!(sPokedexAreaScreen)).areaGlowTilemap[i] &= 65375;
            }
            (*crate::gl!(sPokedexAreaScreen)).areaGlowTilemap[i] = sAreaGlowTilemapMapping
                [(*crate::gl!(sPokedexAreaScreen)).areaGlowTilemap[i]]
                as u16;
            (*crate::gl!(sPokedexAreaScreen)).areaGlowTilemap[i] |= 40960;
        }
    }
}
unsafe fn StartAreaGlow() {
    if (*crate::gl!(sPokedexAreaScreen)).numSpecialAreas != 0
        && (*crate::gl!(sPokedexAreaScreen)).numOverworldAreas == 0
    {
        (*crate::gl!(sPokedexAreaScreen)).showingMarkers = TRUE;
    } else {
        (*crate::gl!(sPokedexAreaScreen)).showingMarkers = FALSE;
    }
    (*crate::gl!(sPokedexAreaScreen)).markerTimer = 0;
    (*crate::gl!(sPokedexAreaScreen)).glowTimer = 0;
    (*crate::gl!(sPokedexAreaScreen)).areaShadeBldArgLo = 0;
    (*crate::gl!(sPokedexAreaScreen)).areaShadeBldArgHi = 64;
    (*crate::gl!(sPokedexAreaScreen)).markerFlashCounter = 1;
    SetGpuReg(REG_OFFSET_BLDCNT, 16196);
    SetGpuReg(REG_OFFSET_BLDALPHA, 4096);
    DoAreaGlow();
}
unsafe fn DoAreaGlow() {
    let mut x: u16 = 0;
    let mut y: u16 = 0;
    let mut i: u16 = 0;
    if (*crate::gl!(sPokedexAreaScreen)).showingMarkers == 0 {
        if (*crate::gl!(sPokedexAreaScreen)).markerTimer == 0 {
            (*crate::gl!(sPokedexAreaScreen)).glowTimer += 1;
            if (*crate::gl!(sPokedexAreaScreen)).glowTimer as i32 & 1 != 0 {
                (*crate::gl!(sPokedexAreaScreen)).areaShadeBldArgLo =
                    ((*crate::gl!(sPokedexAreaScreen)).areaShadeBldArgLo + 4) & 0x7f;
            } else {
                (*crate::gl!(sPokedexAreaScreen)).areaShadeBldArgHi =
                    ((*crate::gl!(sPokedexAreaScreen)).areaShadeBldArgHi + 4) & 0x7f;
            }
            x = ((*(&raw const crate::trig::gSineTable).cast::<CArray<i16, 0>>())
                [(*crate::gl!(sPokedexAreaScreen)).areaShadeBldArgLo]
                >> 4) as u16;
            y = ((*(&raw const crate::trig::gSineTable).cast::<CArray<i16, 0>>())
                [(*crate::gl!(sPokedexAreaScreen)).areaShadeBldArgHi]
                >> 4) as u16;
            SetGpuReg(REG_OFFSET_BLDALPHA, y << 8 | x);
            (*crate::gl!(sPokedexAreaScreen)).markerTimer = 0;
            if (*crate::gl!(sPokedexAreaScreen)).glowTimer == 64 {
                (*crate::gl!(sPokedexAreaScreen)).glowTimer = 0;
                if (*crate::gl!(sPokedexAreaScreen)).numSpecialAreas != 0 {
                    (*crate::gl!(sPokedexAreaScreen)).showingMarkers = TRUE;
                }
            }
        } else {
            (*crate::gl!(sPokedexAreaScreen)).markerTimer -= 1;
        }
    } else {
        (*crate::gl!(sPokedexAreaScreen)).markerTimer += 1;
        if (*crate::gl!(sPokedexAreaScreen)).markerTimer > 12 {
            (*crate::gl!(sPokedexAreaScreen)).markerTimer = 0;
            (*crate::gl!(sPokedexAreaScreen)).markerFlashCounter += 1;
            i = 0;
            while i < (*crate::gl!(sPokedexAreaScreen)).numSpecialAreas {
                (*(*crate::gl!(sPokedexAreaScreen)).areaMarkerSprites[i])
                    .set_invisible((*crate::gl!(sPokedexAreaScreen)).markerFlashCounter as u16 & 1);
                i += 1;
            }
            if (*crate::gl!(sPokedexAreaScreen)).markerFlashCounter > 4 {
                (*crate::gl!(sPokedexAreaScreen)).markerFlashCounter = 1;
                if (*crate::gl!(sPokedexAreaScreen)).numOverworldAreas != 0 {
                    (*crate::gl!(sPokedexAreaScreen)).showingMarkers = FALSE;
                }
            }
        }
    }
}
pub unsafe fn ShowPokedexAreaScreen(species: u16, screenSwitchState: *mut u8) {
    crate::gl!(sPokedexAreaScreen = AllocZeroed(5564) as *mut typeof___sPokedexAreaScreen_0_t);
    (*crate::gl!(sPokedexAreaScreen)).species = species;
    (*crate::gl!(sPokedexAreaScreen)).screenSwitchState = screenSwitchState;
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
                (*crate::gl!(sPokedexAreaScreen)).charBuffer.as_mut_ptr(),
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
            ShowRegionMapForPokedexAreaScreen(&raw mut (*crate::gl!(sPokedexAreaScreen)).regionMap);
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
            if (*gPaletteFade.as_ptr()).active() != 0 {
                return;
            }
            break 'l1;
        }
        if sw1 == 1 {
            if crate::gl!(gMain.newKeys) as i32 & B_BUTTON != 0 {
                task_set(taskId, 1, 1);
                PlaySE(SE_PC_OFF);
            } else if crate::gl!(gMain.newKeys) as i32 & DPAD_RIGHT != 0
                || crate::gl!(gMain.newKeys) as i32 & R_BUTTON != 0
                    && (*crate::gl!(gSaveBlock2Ptr)).optionsButtonMode == OPTIONS_BUTTON_MODE_LR
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
            if (*gPaletteFade.as_ptr()).active() != 0 {
                return;
            }
            DestroyAreaScreenSprites();
            *(*crate::gl!(sPokedexAreaScreen)).screenSwitchState = task_get(taskId, 1) as u8;
            ResetPokedexAreaMapBg();
            DestroyTask(taskId);
            FreePokedexAreaMapBgNum();
            Free(crate::gl!(sPokedexAreaScreen) as *mut c_void);
            crate::gl!(sPokedexAreaScreen = null_mut());
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
    while (CreateAreaMarkerSprites_i.get() as i32)
        < (*crate::gl!(sPokedexAreaScreen)).numSpecialAreas as i32
    {
        CreateAreaMarkerSprites_mapSecId.set(
            (*crate::gl!(sPokedexAreaScreen)).specialAreaRegionMapSectionIds
                [CreateAreaMarkerSprites_i.get()] as i16,
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
            (*gSprites.as_ptr())[spriteId].set_invisible(TRUE as u16);
            (*crate::gl!(sPokedexAreaScreen)).areaMarkerSprites[{
                let t1 = CreateAreaMarkerSprites_numSprites.get();
                CreateAreaMarkerSprites_numSprites
                    .set(CreateAreaMarkerSprites_numSprites.get() + 1);
                t1
            }] = crate::gl!(&raw gSprites[spriteId]);
        }
        CreateAreaMarkerSprites_i.set(CreateAreaMarkerSprites_i.get() + 1);
    }
    (*crate::gl!(sPokedexAreaScreen)).numAreaMarkerSprites =
        CreateAreaMarkerSprites_numSprites.get() as u16;
}
unsafe fn DestroyAreaScreenSprites() {
    FreeSpriteTilesByTag(TAG_AREA_MARKER);
    FreeSpritePaletteByTag(TAG_AREA_MARKER);
    let mut i: u16 = 0;
    while i < (*crate::gl!(sPokedexAreaScreen)).numAreaMarkerSprites {
        DestroySprite((*crate::gl!(sPokedexAreaScreen)).areaMarkerSprites[i]);
        i += 1;
    }
    FreeSpriteTilesByTag(TAG_AREA_UNKNOWN);
    FreeSpritePaletteByTag(TAG_AREA_UNKNOWN);
    for i in 0..3u16 {
        if !(*crate::gl!(sPokedexAreaScreen)).areaUnknownSprites[i].is_null() {
            DestroySprite((*crate::gl!(sPokedexAreaScreen)).areaUnknownSprites[i]);
        }
    }
}
unsafe fn LoadAreaUnknownGraphics() {
    let mut spriteSheet: SpriteSheet = zeroed();
    spriteSheet.data = (*crate::gl!(sPokedexAreaScreen))
        .areaUnknownGraphicsBuffer
        .as_mut_ptr() as *mut c_void;
    spriteSheet.size = 1536;
    spriteSheet.tag = TAG_AREA_UNKNOWN;
    LZ77UnCompWram(
        (*(&raw const crate::data::graphics::gPokedexAreaScreenAreaUnknown_Gfx)
            .cast::<CArray<u32, 0>>())
        .as_ptr()
        .cast_mut(),
        (*crate::gl!(sPokedexAreaScreen))
            .areaUnknownGraphicsBuffer
            .as_mut_ptr() as *mut c_void,
    );
    LoadSpriteSheet(&raw mut spriteSheet);
    LoadSpritePalette((&raw const *sAreaUnknownSpritePalette).cast_mut());
}
unsafe fn CreateAreaUnknownSprites() {
    if (*crate::gl!(sPokedexAreaScreen)).numOverworldAreas != 0
        || (*crate::gl!(sPokedexAreaScreen)).numSpecialAreas != 0
    {
        for i in 0..3u16 {
            (*crate::gl!(sPokedexAreaScreen)).areaUnknownSprites[i] = null_mut();
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
                (*gSprites.as_ptr())[spriteId]
                    .oam
                    .set_tileNum((*gSprites.as_ptr())[spriteId].oam.tileNum() + i * 16);
                (*crate::gl!(sPokedexAreaScreen)).areaUnknownSprites[i] =
                    crate::gl!(&raw gSprites[spriteId]);
            } else {
                (*crate::gl!(sPokedexAreaScreen)).areaUnknownSprites[i] = null_mut();
            }
        }
    }
}
