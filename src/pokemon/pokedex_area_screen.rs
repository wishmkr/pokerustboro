//! Translated from `src/pokedex_area_screen.c` by tools/rustport/c2rs.py, then reviewed.
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

// Data tables (translate with cdata.py): sAreaGlow_Pal sAreaGlow_Gfx sSpeciesHiddenFromAreaScreen sMovingRegionMapSections sFeebasData sLandmarkData sAreaGlowTilemapMapping sPokedexAreaMapTemplate sAreaMarkerTiles sAreaMarkerSpriteSheet sAreaMarkerPalette sAreaMarkerSpritePalette sAreaMarkerOamData sAreaMarkerSpriteTemplate sAreaMarkerPalette sAreaMarkerTiles sAreaUnknownSpritePalette sAreaUnknownOamData sAreaUnknownSpriteTemplate
#[allow(unused_imports)]
use crate::data::pokedex_area_screen::*;

#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sPokedexAreaScreen: *mut u8 = core::ptr::null_mut();
static mut CREATEAREAMARKERSPRITES_X: i16 = 0i16;
static mut CREATEAREAMARKERSPRITES_Y: i16 = 0i16;
static mut CREATEAREAMARKERSPRITES_I: i16 = 0i16;
static mut CREATEAREAMARKERSPRITES_MAPSECID: i16 = 0i16;
static mut CREATEAREAMARKERSPRITES_NUMSPRITES: i16 = 0i16;

unsafe extern "C" {
    static mut gMain: u8;
    static mut gPaletteFade: u8;
    static mut gPlttBufferUnfaded: u8;
    static mut gPokedexAreaScreenAreaUnknown_Gfx: u8;
    static mut gRegionMapEntries: u8;
    static mut gSaveBlock1Ptr: u8;
    static mut gSaveBlock2Ptr: u8;
    static mut gSineTable: u8;
    static mut gSprites: u8;
    static mut gTasks: u8;
    static mut gWildMonHeaders: u8;
    fn AllocZeroed(a0: u32) -> *mut u8;
    fn BeginNormalPaletteFade(a0: u32, a1: i8, a2: u8, a3: u8, a4: u16) -> u8;
    fn ChangeBgY(a0: u8, a1: i32, a2: u8) -> i32;
    fn CorrectSpecialMapSecId(a0: u16) -> u16;
    fn CpuSet(a0: *mut u8, a1: *mut u8, a2: u32);
    fn CreateRegionMapPlayerIcon(a0: u16, a1: u16);
    fn CreateSprite(a0: *mut u8, a1: i16, a2: i16, a3: u8) -> u8;
    fn CreateTask(a0: Option<unsafe extern "C" fn(u8)>, a1: u8) -> u8;
    fn DecompressAndCopyTileDataToVram(a0: u8, a1: *mut u8, a2: u32, a3: u16, a4: u8) -> *mut u8;
    fn DestroySprite(a0: *mut u8);
    fn DestroyTask(a0: u8);
    fn FlagGet(a0: u16) -> u8;
    fn Free(a0: *mut u8);
    fn FreeAllSpritePalettes();
    fn FreePokedexAreaMapBgNum();
    fn FreeSpritePaletteByTag(a0: u16);
    fn FreeSpriteTilesByTag(a0: u16);
    fn FreeTempTileDataBuffersIfPossible() -> u8;
    fn GetRegionMapSecIdAt(a0: u16, a1: u16) -> u16;
    fn GetRoamerLocation(a0: *mut u8, a1: *mut u8);
    fn HideBg(a0: u8);
    fn LZ77UnCompWram(a0: *mut u32, a1: *mut u8);
    fn LoadBgTilemap(a0: u8, a1: *mut u8, a2: u16, a3: u16) -> u16;
    fn LoadPokedexAreaMapGfx(a0: *mut u8);
    fn LoadSpritePalette(a0: *mut u8) -> u8;
    fn LoadSpriteSheet(a0: *mut u8) -> u16;
    fn Overworld_GetMapHeaderByGroupAndId(a0: u16, a1: u16) -> *mut u8;
    fn PlaySE(a0: u16);
    fn PokedexAreaMapChangeBgY(a0: u32);
    fn PokedexAreaScreen_UpdateRegionMapVariablesAndVideoRegs(a0: i16, a1: i16);
    fn ResetSpriteData();
    fn SetBgAttribute(a0: u8, a1: u8, a2: u8);
    fn SetGpuReg(a0: u8, a1: u16);
    fn SetGpuRegBits(a0: u8, a1: u16);
    fn ShowBg(a0: u8);
    fn ShowRegionMapForPokedexAreaScreen(a0: *mut u8);
    fn StringFill(a0: *mut u8, a1: u8, a2: u16) -> *mut u8;
    fn TryShowPokedexAreaMap() -> u32;
    fn VarGet(a0: u16) -> u16;
}

pub(crate) unsafe extern "C" fn ResetDrawAreaGlowState() {
    unsafe {
        ((((&raw mut sPokedexAreaScreen).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(276)
            .cast::<u16>())
        .write(0u16);
    }
}
pub(crate) unsafe extern "C" fn DrawAreaGlow() -> u8 {
    unsafe {
        'l1: {
            let __sw1 = ((((((&raw mut sPokedexAreaScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(276)
                .cast::<u16>())
            .read()) as i32);
            let __matched =
                __sw1 == 0i32 || __sw1 == 1i32 || __sw1 == 2i32 || __sw1 == 3i32 || __sw1 == 4i32;
            if __sw1 == 0i32 {
                FindMapsWithMon(
                    ((((&raw mut sPokedexAreaScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(14)
                        .cast::<u16>())
                    .read(),
                );
                break 'l1;
            }
            if __sw1 == 1i32 {
                BuildAreaGlowTilemap();
                break 'l1;
            }
            if __sw1 == 2i32 {
                DecompressAndCopyTileDataToVram(
                    2u8,
                    (((&raw const sAreaGlow_Gfx)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<u32>())
                    .cast::<u32>())
                    .cast::<u8>(),
                    0u32,
                    0u16,
                    0u8,
                );
                LoadBgTilemap(
                    2u8,
                    (((((&raw mut sPokedexAreaScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(278))
                    .cast::<u16>())
                    .cast::<u8>(),
                    1280u16,
                    0u16,
                );
                break 'l1;
            }
            if __sw1 == 3i32 {
                if !((FreeTempTileDataBuffersIfPossible()) != 0) {
                    'l2: loop {
                        'l3: {
                            'l4: loop {
                                'l5: {
                                    CpuSet(
                                        (((&raw const sAreaGlow_Pal)
                                            .cast::<u8>()
                                            .cast_mut()
                                            .cast::<u32>())
                                        .cast::<u32>())
                                        .cast::<u8>(),
                                        ((((&raw mut gPlttBufferUnfaded).cast::<u16>())
                                            .cast::<u16>())
                                        .wrapping_offset(160))
                                        .cast::<u8>(),
                                        (67108864u32
                                            | (crate::c::div_u32(
                                                32u32,
                                                ((crate::c::div_i32(32i32, 8i32)) as u32),
                                            ) & 2097151u32)),
                                    );
                                }
                                if !((0i32) != 0) {
                                    break 'l4;
                                }
                            }
                        }
                        if !((0i32) != 0) {
                            break 'l2;
                        }
                    }
                    let __p2 = (((&raw mut sPokedexAreaScreen).cast::<u8>().cast::<*mut u8>())
                        .read())
                    .wrapping_add(276)
                    .cast::<u16>();
                    (__p2).write(((__p2).read()).wrapping_add(1));
                }
                return 1u8;
            }
            if __sw1 == 4i32 {
                ChangeBgY(2u8, (-2048i32), 0u8);
                break 'l1;
            }
            if !__matched {
                return 0u8;
            }
        }
        let __p3 = (((&raw mut sPokedexAreaScreen).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(276)
            .cast::<u16>();
        (__p3).write(((__p3).read()).wrapping_add(1));
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn FindMapsWithMon(species: u16) {
    unsafe {
        let mut species = species;
        let mut i: u16 = 0u16;
        let mut roamer: *mut u8 = core::ptr::null_mut();
        ((((&raw mut sPokedexAreaScreen).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(1762)
            .cast::<u16>())
        .write(0u16);
        ((((&raw mut sPokedexAreaScreen).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(1764)
            .cast::<u16>())
        .write(VarGet(16446u16));
        if ((((((&raw mut sPokedexAreaScreen).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(1764)
            .cast::<u16>())
        .read()) as i32)
            >= 9i32
        {
            ((((&raw mut sPokedexAreaScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(1764)
                .cast::<u16>())
            .write(0u16);
        }
        roamer = (((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(12764);
        if ((species) as i32) != ((((roamer).wrapping_add(8).cast::<u16>()).read()) as i32) {
            ((((&raw mut sPokedexAreaScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(272)
                .cast::<u16>())
            .write(0u16);
            ((((&raw mut sPokedexAreaScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(274)
                .cast::<u16>())
            .write(0u16);
            {
                i = 0u16;
                'l1: loop {
                    if !(((i) as u32) < crate::c::div_u32(2u32, 2u32)) {
                        break 'l1;
                    }
                    'l2: {
                        if ((((((&raw const sSpeciesHiddenFromAreaScreen)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<u16>())
                        .cast::<u16>())
                        .wrapping_offset(((i) as i32) as isize))
                        .read()) as i32)
                            == ((species) as i32)
                        {
                            return;
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
            {
                i = 0u16;
                'l3: loop {
                    if !((((((((&raw const sFeebasData).cast::<u8>().cast_mut()).cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 6))
                    .cast::<u16>())
                    .read()) as i32)
                        != 412i32)
                    {
                        break 'l3;
                    }
                    'l4: {
                        if ((species) as i32)
                            == (((((((&raw const sFeebasData).cast::<u8>().cast_mut())
                                .cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 6))
                            .cast::<u16>())
                            .read()) as i32)
                        {
                            'l5: {
                                let __sw1 =
                                    ((((((((&raw const sFeebasData).cast::<u8>().cast_mut())
                                        .cast::<u8>())
                                    .wrapping_offset(((i) as i32) as isize * 6))
                                    .cast::<u16>())
                                    .wrapping_offset(1))
                                    .read()) as i32);
                                if __sw1 == 0i32 {
                                    SetAreaHasMon(
                                        ((((((&raw const sFeebasData).cast::<u8>().cast_mut())
                                            .cast::<u8>())
                                        .wrapping_offset(((i) as i32) as isize * 6))
                                        .cast::<u16>())
                                        .wrapping_offset(1))
                                        .read(),
                                        ((((((&raw const sFeebasData).cast::<u8>().cast_mut())
                                            .cast::<u8>())
                                        .wrapping_offset(((i) as i32) as isize * 6))
                                        .cast::<u16>())
                                        .wrapping_offset(2))
                                        .read(),
                                    );
                                    break 'l5;
                                }
                                if __sw1 == 24i32 || __sw1 == 26i32 {
                                    SetSpecialMapHasMon(
                                        ((((((&raw const sFeebasData).cast::<u8>().cast_mut())
                                            .cast::<u8>())
                                        .wrapping_offset(((i) as i32) as isize * 6))
                                        .cast::<u16>())
                                        .wrapping_offset(1))
                                        .read(),
                                        ((((((&raw const sFeebasData).cast::<u8>().cast_mut())
                                            .cast::<u8>())
                                        .wrapping_offset(((i) as i32) as isize * 6))
                                        .cast::<u16>())
                                        .wrapping_offset(2))
                                        .read(),
                                    );
                                    break 'l5;
                                }
                            }
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
            {
                i = 0u16;
                'l6: loop {
                    if !((((((&raw mut gWildMonHeaders).cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 20))
                    .read()) as i32)
                        != 255i32)
                    {
                        break 'l6;
                    }
                    'l7: {
                        if (MapHasSpecies(
                            ((&raw mut gWildMonHeaders).cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize * 20),
                            species,
                        )) != 0
                        {
                            'l8: {
                                let __sw2 = (((((&raw mut gWildMonHeaders).cast::<u8>())
                                    .wrapping_offset(((i) as i32) as isize * 20))
                                .read()) as i32);
                                if __sw2 == 0i32 {
                                    SetAreaHasMon(
                                        (((((&raw mut gWildMonHeaders).cast::<u8>())
                                            .wrapping_offset(((i) as i32) as isize * 20))
                                        .read()) as u16),
                                        ((((((&raw mut gWildMonHeaders).cast::<u8>())
                                            .wrapping_offset(((i) as i32) as isize * 20))
                                        .wrapping_add(1))
                                        .read()) as u16),
                                    );
                                    break 'l8;
                                }
                                if __sw2 == 24i32 || __sw2 == 26i32 {
                                    SetSpecialMapHasMon(
                                        (((((&raw mut gWildMonHeaders).cast::<u8>())
                                            .wrapping_offset(((i) as i32) as isize * 20))
                                        .read()) as u16),
                                        ((((((&raw mut gWildMonHeaders).cast::<u8>())
                                            .wrapping_offset(((i) as i32) as isize * 20))
                                        .wrapping_add(1))
                                        .read()) as u16),
                                    );
                                    break 'l8;
                                }
                            }
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
        } else {
            ((((&raw mut sPokedexAreaScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(274)
                .cast::<u16>())
            .write(0u16);
            if (((roamer).wrapping_add(19)).read()) != 0 {
                GetRoamerLocation(
                    (((((&raw mut sPokedexAreaScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(16))
                    .cast::<u8>()),
                    (((((&raw mut sPokedexAreaScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(16))
                    .cast::<u8>())
                    .wrapping_add(1),
                );
                ((((((&raw mut sPokedexAreaScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(16))
                .cast::<u8>())
                .wrapping_add(2)
                .cast::<u16>())
                .write(
                    ((((Overworld_GetMapHeaderByGroupAndId(
                        (((((((&raw mut sPokedexAreaScreen).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(16))
                        .cast::<u8>())
                        .read()) as u16),
                        ((((((((&raw mut sPokedexAreaScreen).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(16))
                        .cast::<u8>())
                        .wrapping_add(1))
                        .read()) as u16),
                    ))
                    .wrapping_add(20))
                    .read()) as u16),
                );
                ((((&raw mut sPokedexAreaScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(272)
                    .cast::<u16>())
                .write(1u16);
            } else {
                ((((&raw mut sPokedexAreaScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(272)
                    .cast::<u16>())
                .write(0u16);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SetAreaHasMon(mapGroup: u16, mapNum: u16) {
    unsafe {
        let mut mapGroup = mapGroup;
        let mut mapNum = mapNum;
        if ((((((&raw mut sPokedexAreaScreen).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(272)
            .cast::<u16>())
        .read()) as i32)
            < 64i32
        {
            ((((((&raw mut sPokedexAreaScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(16))
            .cast::<u8>())
            .wrapping_offset(
                ((((((&raw mut sPokedexAreaScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(272)
                    .cast::<u16>())
                .read()) as i32) as isize
                    * 4,
            ))
            .write(((mapGroup) as u8));
            (((((((&raw mut sPokedexAreaScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(16))
            .cast::<u8>())
            .wrapping_offset(
                ((((((&raw mut sPokedexAreaScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(272)
                    .cast::<u16>())
                .read()) as i32) as isize
                    * 4,
            ))
            .wrapping_add(1))
            .write(((mapNum) as u8));
            (((((((&raw mut sPokedexAreaScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(16))
            .cast::<u8>())
            .wrapping_offset(
                ((((((&raw mut sPokedexAreaScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(272)
                    .cast::<u16>())
                .read()) as i32) as isize
                    * 4,
            ))
            .wrapping_add(2)
            .cast::<u16>())
            .write(CorrectSpecialMapSecId(
                ((((Overworld_GetMapHeaderByGroupAndId(mapGroup, mapNum)).wrapping_add(20)).read())
                    as u16),
            ));
            let __p1 = (((&raw mut sPokedexAreaScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(272)
                .cast::<u16>();
            (__p1).write(((__p1).read()).wrapping_add(1));
        }
    }
}
pub(crate) unsafe extern "C" fn SetSpecialMapHasMon(mapGroup: u16, mapNum: u16) {
    unsafe {
        let mut mapGroup = mapGroup;
        let mut mapNum = mapNum;
        let mut i: i32 = 0i32;
        if ((((((&raw mut sPokedexAreaScreen).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(274)
            .cast::<u16>())
        .read()) as i32)
            < 32i32
        {
            let mut regionMapSectionId: u16 =
                GetRegionMapSectionId(((mapGroup) as u8), ((mapNum) as u8));
            if ((regionMapSectionId) as i32) < 213i32 {
                {
                    i = 0i32;
                    'l1: loop {
                        if !(((i) as u32) < crate::c::div_u32(6u32, 2u32)) {
                            break 'l1;
                        }
                        'l2: {
                            if ((regionMapSectionId) as i32)
                                == ((((((&raw const sMovingRegionMapSections)
                                    .cast::<u8>()
                                    .cast_mut()
                                    .cast::<u16>())
                                .cast::<u16>())
                                .wrapping_offset((i) as isize))
                                .read()) as i32)
                            {
                                return;
                            }
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                {
                    i = 0i32;
                    'l3: loop {
                        if !((((((((&raw const sLandmarkData).cast::<u8>().cast_mut())
                            .cast::<u8>())
                        .wrapping_offset((i) as isize * 4))
                        .cast::<u16>())
                        .read()) as i32)
                            != 213i32)
                        {
                            break 'l3;
                        }
                        'l4: {
                            if (((regionMapSectionId) as i32)
                                == (((((((&raw const sLandmarkData).cast::<u8>().cast_mut())
                                    .cast::<u8>())
                                .wrapping_offset((i) as isize * 4))
                                .cast::<u16>())
                                .read()) as i32))
                                && (!((FlagGet(
                                    ((((((&raw const sLandmarkData).cast::<u8>().cast_mut())
                                        .cast::<u8>())
                                    .wrapping_offset((i) as isize * 4))
                                    .cast::<u16>())
                                    .wrapping_offset(1))
                                    .read(),
                                )) != 0))
                            {
                                return;
                            }
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                {
                    i = 0i32;
                    'l5: loop {
                        if !(i
                            < ((((((&raw mut sPokedexAreaScreen).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(274)
                            .cast::<u16>())
                            .read()) as i32))
                        {
                            break 'l5;
                        }
                        'l6: {
                            if ((((((((&raw mut sPokedexAreaScreen)
                                .cast::<u8>()
                                .cast::<*mut u8>())
                            .read())
                            .wrapping_add(1568))
                            .cast::<u16>())
                            .wrapping_offset((i) as isize))
                            .read()) as i32)
                                == ((regionMapSectionId) as i32)
                            {
                                break 'l5;
                            }
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                if i == ((((((&raw mut sPokedexAreaScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(274)
                    .cast::<u16>())
                .read()) as i32)
                {
                    ((((((&raw mut sPokedexAreaScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(1568))
                    .cast::<u16>())
                    .wrapping_offset((i) as isize))
                    .write(regionMapSectionId);
                    let __p1 = (((&raw mut sPokedexAreaScreen).cast::<u8>().cast::<*mut u8>())
                        .read())
                    .wrapping_add(274)
                    .cast::<u16>();
                    (__p1).write(((__p1).read()).wrapping_add(1));
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn GetRegionMapSectionId(mapGroup: u8, mapNum: u8) -> u16 {
    unsafe {
        let mut mapGroup = mapGroup;
        let mut mapNum = mapNum;
        return ((((Overworld_GetMapHeaderByGroupAndId(((mapGroup) as u16), ((mapNum) as u16)))
            .wrapping_add(20))
        .read()) as u16);
    }
}
pub(crate) unsafe extern "C" fn MapHasSpecies(info: *mut u8, species: u16) -> u8 {
    unsafe {
        let mut info = info;
        let mut species = species;
        if ((GetRegionMapSectionId((info).read(), ((info).wrapping_add(1)).read())) as i32)
            == 210i32
        {
            let __p1 = (((&raw mut sPokedexAreaScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(1762)
                .cast::<u16>();
            (__p1).write(((__p1).read()).wrapping_add(1));
            if ((((((&raw mut sPokedexAreaScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(1762)
                .cast::<u16>())
            .read()) as i32)
                != ((((((&raw mut sPokedexAreaScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(1764)
                    .cast::<u16>())
                .read()) as i32)
                    .wrapping_add(1i32)
            {
                return 0u8;
            }
        }
        if (MonListHasSpecies(
            ((info).wrapping_add(4).cast::<*mut u8>()).read(),
            species,
            12u16,
        )) != 0
        {
            return 1u8;
        }
        if (MonListHasSpecies(
            ((info).wrapping_add(8).cast::<*mut u8>()).read(),
            species,
            5u16,
        )) != 0
        {
            return 1u8;
        }
        if (MonListHasSpecies(
            ((info).wrapping_add(16).cast::<*mut u8>()).read(),
            species,
            12u16,
        )) != 0
        {
            return 1u8;
        }
        if (MonListHasSpecies(
            ((info).wrapping_add(12).cast::<*mut u8>()).read(),
            species,
            5u16,
        )) != 0
        {
            return 1u8;
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn MonListHasSpecies(info: *mut u8, species: u16, size: u16) -> u8 {
    unsafe {
        let mut info = info;
        let mut species = species;
        let mut size = size;
        let mut i: u16 = 0u16;
        if ((info) as usize) != 0usize {
            {
                i = 0u16;
                'l1: loop {
                    if !(((i) as i32) < ((size) as i32)) {
                        break 'l1;
                    }
                    'l2: {
                        if (((((((info).wrapping_add(4).cast::<*mut u8>()).read())
                            .wrapping_offset(((i) as i32) as isize * 4))
                        .wrapping_add(2)
                        .cast::<u16>())
                        .read()) as i32)
                            == ((species) as i32)
                        {
                            return 1u8;
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn BuildAreaGlowTilemap() {
    unsafe {
        let mut i: u16 = 0u16;
        let mut y: u16 = 0u16;
        let mut x: u16 = 0u16;
        let mut j: u16 = 0u16;
        {
            i = 0u16;
            'l1: loop {
                if !(((i) as u32) < crate::c::div_u32(1280u32, 2u32)) {
                    break 'l1;
                }
                'l2: {
                    ((((((&raw mut sPokedexAreaScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(278))
                    .cast::<u16>())
                    .wrapping_offset(((i) as i32) as isize))
                    .write(0u16);
                }
                i = (i).wrapping_add(1);
            }
        }
        {
            i = 0u16;
            'l3: loop {
                if !(((i) as i32)
                    < ((((((&raw mut sPokedexAreaScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(272)
                        .cast::<u16>())
                    .read()) as i32))
                {
                    break 'l3;
                }
                'l4: {
                    j = 0u16;
                    {
                        y = 0u16;
                        'l5: loop {
                            if !(((y) as i32) < 20i32) {
                                break 'l5;
                            }
                            'l6: {
                                {
                                    x = 0u16;
                                    'l7: loop {
                                        if !(((x) as i32) < 32i32) {
                                            break 'l7;
                                        }
                                        'l8: {
                                            if ((GetRegionMapSecIdAt(x, y)) as i32)
                                                == (((((((((&raw mut sPokedexAreaScreen)
                                                    .cast::<u8>()
                                                    .cast::<*mut u8>())
                                                .read())
                                                .wrapping_add(16))
                                                .cast::<u8>())
                                                .wrapping_offset(((i) as i32) as isize * 4))
                                                .wrapping_add(2)
                                                .cast::<u16>())
                                                .read())
                                                    as i32)
                                            {
                                                ((((((&raw mut sPokedexAreaScreen)
                                                    .cast::<u8>()
                                                    .cast::<*mut u8>())
                                                .read())
                                                .wrapping_add(278))
                                                .cast::<u16>())
                                                .wrapping_offset(((j) as i32) as isize))
                                                .write(65535u16);
                                            }
                                            j = (j).wrapping_add(1);
                                        }
                                        x = (x).wrapping_add(1);
                                    }
                                }
                            }
                            y = (y).wrapping_add(1);
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        j = 0u16;
        {
            y = 0u16;
            'l9: loop {
                if !(((y) as i32) < 20i32) {
                    break 'l9;
                }
                'l10: {
                    {
                        x = 0u16;
                        'l11: loop {
                            if !(((x) as i32) < 32i32) {
                                break 'l11;
                            }
                            'l12: {
                                if ((((((((&raw mut sPokedexAreaScreen)
                                    .cast::<u8>()
                                    .cast::<*mut u8>())
                                .read())
                                .wrapping_add(278))
                                .cast::<u16>())
                                .wrapping_offset(((j) as i32) as isize))
                                .read()) as i32)
                                    == 65535i32
                                {
                                    if (((x) as i32) != 0i32)
                                        && (((((((((&raw mut sPokedexAreaScreen)
                                            .cast::<u8>()
                                            .cast::<*mut u8>())
                                        .read())
                                        .wrapping_add(278))
                                        .cast::<u16>())
                                        .wrapping_offset(
                                            (((j) as i32).wrapping_sub(1i32)) as isize,
                                        ))
                                        .read())
                                            as i32)
                                            != 65535i32)
                                    {
                                        let __p1 = (((((&raw mut sPokedexAreaScreen)
                                            .cast::<u8>()
                                            .cast::<*mut u8>())
                                        .read())
                                        .wrapping_add(278))
                                        .cast::<u16>())
                                        .wrapping_offset(
                                            (((j) as i32).wrapping_sub(1i32)) as isize,
                                        );
                                        (__p1).write((((((__p1).read()) as i32) | 2i32) as u16));
                                    }
                                    if (((x) as i32) != 31i32)
                                        && (((((((((&raw mut sPokedexAreaScreen)
                                            .cast::<u8>()
                                            .cast::<*mut u8>())
                                        .read())
                                        .wrapping_add(278))
                                        .cast::<u16>())
                                        .wrapping_offset(
                                            (((j) as i32).wrapping_add(1i32)) as isize,
                                        ))
                                        .read())
                                            as i32)
                                            != 65535i32)
                                    {
                                        let __p2 = (((((&raw mut sPokedexAreaScreen)
                                            .cast::<u8>()
                                            .cast::<*mut u8>())
                                        .read())
                                        .wrapping_add(278))
                                        .cast::<u16>())
                                        .wrapping_offset(
                                            (((j) as i32).wrapping_add(1i32)) as isize,
                                        );
                                        (__p2).write((((((__p2).read()) as i32) | 1i32) as u16));
                                    }
                                    if (((y) as i32) != 0i32)
                                        && (((((((((&raw mut sPokedexAreaScreen)
                                            .cast::<u8>()
                                            .cast::<*mut u8>())
                                        .read())
                                        .wrapping_add(278))
                                        .cast::<u16>())
                                        .wrapping_offset(
                                            (((j) as i32).wrapping_sub(32i32)) as isize,
                                        ))
                                        .read())
                                            as i32)
                                            != 65535i32)
                                    {
                                        let __p3 = (((((&raw mut sPokedexAreaScreen)
                                            .cast::<u8>()
                                            .cast::<*mut u8>())
                                        .read())
                                        .wrapping_add(278))
                                        .cast::<u16>())
                                        .wrapping_offset(
                                            (((j) as i32).wrapping_sub(32i32)) as isize,
                                        );
                                        (__p3).write((((((__p3).read()) as i32) | 8i32) as u16));
                                    }
                                    if (((y) as i32) != 19i32)
                                        && (((((((((&raw mut sPokedexAreaScreen)
                                            .cast::<u8>()
                                            .cast::<*mut u8>())
                                        .read())
                                        .wrapping_add(278))
                                        .cast::<u16>())
                                        .wrapping_offset(
                                            (((j) as i32).wrapping_add(32i32)) as isize,
                                        ))
                                        .read())
                                            as i32)
                                            != 65535i32)
                                    {
                                        let __p4 = (((((&raw mut sPokedexAreaScreen)
                                            .cast::<u8>()
                                            .cast::<*mut u8>())
                                        .read())
                                        .wrapping_add(278))
                                        .cast::<u16>())
                                        .wrapping_offset(
                                            (((j) as i32).wrapping_add(32i32)) as isize,
                                        );
                                        (__p4).write((((((__p4).read()) as i32) | 4i32) as u16));
                                    }
                                    if ((((x) as i32) != 0i32) && (((y) as i32) != 0i32))
                                        && (((((((((&raw mut sPokedexAreaScreen)
                                            .cast::<u8>()
                                            .cast::<*mut u8>())
                                        .read())
                                        .wrapping_add(278))
                                        .cast::<u16>())
                                        .wrapping_offset(
                                            ((((j) as i32).wrapping_sub(32i32)).wrapping_sub(1i32))
                                                as isize,
                                        ))
                                        .read())
                                            as i32)
                                            != 65535i32)
                                    {
                                        let __p5 = (((((&raw mut sPokedexAreaScreen)
                                            .cast::<u8>()
                                            .cast::<*mut u8>())
                                        .read())
                                        .wrapping_add(278))
                                        .cast::<u16>())
                                        .wrapping_offset(
                                            ((((j) as i32).wrapping_sub(32i32)).wrapping_sub(1i32))
                                                as isize,
                                        );
                                        (__p5).write((((((__p5).read()) as i32) | 16i32) as u16));
                                    }
                                    if ((((x) as i32) != 31i32) && (((y) as i32) != 0i32))
                                        && (((((((((&raw mut sPokedexAreaScreen)
                                            .cast::<u8>()
                                            .cast::<*mut u8>())
                                        .read())
                                        .wrapping_add(278))
                                        .cast::<u16>())
                                        .wrapping_offset(
                                            ((((j) as i32).wrapping_sub(32i32)).wrapping_add(1i32))
                                                as isize,
                                        ))
                                        .read())
                                            as i32)
                                            != 65535i32)
                                    {
                                        let __p6 = (((((&raw mut sPokedexAreaScreen)
                                            .cast::<u8>()
                                            .cast::<*mut u8>())
                                        .read())
                                        .wrapping_add(278))
                                        .cast::<u16>())
                                        .wrapping_offset(
                                            ((((j) as i32).wrapping_sub(32i32)).wrapping_add(1i32))
                                                as isize,
                                        );
                                        (__p6).write((((((__p6).read()) as i32) | 64i32) as u16));
                                    }
                                    if ((((x) as i32) != 0i32) && (((y) as i32) != 19i32))
                                        && (((((((((&raw mut sPokedexAreaScreen)
                                            .cast::<u8>()
                                            .cast::<*mut u8>())
                                        .read())
                                        .wrapping_add(278))
                                        .cast::<u16>())
                                        .wrapping_offset(
                                            ((((j) as i32).wrapping_add(32i32)).wrapping_sub(1i32))
                                                as isize,
                                        ))
                                        .read())
                                            as i32)
                                            != 65535i32)
                                    {
                                        let __p7 = (((((&raw mut sPokedexAreaScreen)
                                            .cast::<u8>()
                                            .cast::<*mut u8>())
                                        .read())
                                        .wrapping_add(278))
                                        .cast::<u16>())
                                        .wrapping_offset(
                                            ((((j) as i32).wrapping_add(32i32)).wrapping_sub(1i32))
                                                as isize,
                                        );
                                        (__p7).write((((((__p7).read()) as i32) | 32i32) as u16));
                                    }
                                    if ((((x) as i32) != 31i32) && (((y) as i32) != 19i32))
                                        && (((((((((&raw mut sPokedexAreaScreen)
                                            .cast::<u8>()
                                            .cast::<*mut u8>())
                                        .read())
                                        .wrapping_add(278))
                                        .cast::<u16>())
                                        .wrapping_offset(
                                            ((((j) as i32).wrapping_add(32i32)).wrapping_add(1i32))
                                                as isize,
                                        ))
                                        .read())
                                            as i32)
                                            != 65535i32)
                                    {
                                        let __p8 = (((((&raw mut sPokedexAreaScreen)
                                            .cast::<u8>()
                                            .cast::<*mut u8>())
                                        .read())
                                        .wrapping_add(278))
                                        .cast::<u16>())
                                        .wrapping_offset(
                                            ((((j) as i32).wrapping_add(32i32)).wrapping_add(1i32))
                                                as isize,
                                        );
                                        (__p8).write((((((__p8).read()) as i32) | 128i32) as u16));
                                    }
                                }
                                j = (j).wrapping_add(1);
                            }
                            x = (x).wrapping_add(1);
                        }
                    }
                }
                y = (y).wrapping_add(1);
            }
        }
        {
            i = 0u16;
            'l13: loop {
                if !(((i) as u32) < crate::c::div_u32(1280u32, 2u32)) {
                    break 'l13;
                }
                'l14: {
                    if ((((((((&raw mut sPokedexAreaScreen).cast::<u8>().cast::<*mut u8>())
                        .read())
                    .wrapping_add(278))
                    .cast::<u16>())
                    .wrapping_offset(((i) as i32) as isize))
                    .read()) as i32)
                        == 65535i32
                    {
                        ((((((&raw mut sPokedexAreaScreen).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(278))
                        .cast::<u16>())
                        .wrapping_offset(((i) as i32) as isize))
                        .write(16u16);
                        let __p9 =
                            (((((&raw mut sPokedexAreaScreen).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(278))
                            .cast::<u16>())
                            .wrapping_offset(((i) as i32) as isize);
                        (__p9).write((((((__p9).read()) as i32) | 40960i32) as u16));
                    } else {
                        if (((((((&raw mut sPokedexAreaScreen).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(278))
                        .cast::<u16>())
                        .wrapping_offset(((i) as i32) as isize))
                        .read())
                            != 0
                        {
                            if (((((((((&raw mut sPokedexAreaScreen)
                                .cast::<u8>()
                                .cast::<*mut u8>())
                            .read())
                            .wrapping_add(278))
                            .cast::<u16>())
                            .wrapping_offset(((i) as i32) as isize))
                            .read()) as i32)
                                & 2i32)
                                != 0
                            {
                                let __p10 = (((((&raw mut sPokedexAreaScreen)
                                    .cast::<u8>()
                                    .cast::<*mut u8>())
                                .read())
                                .wrapping_add(278))
                                .cast::<u16>())
                                .wrapping_offset(((i) as i32) as isize);
                                (__p10).write((((((__p10).read()) as i32) & (-49i32)) as u16));
                            }
                            if (((((((((&raw mut sPokedexAreaScreen)
                                .cast::<u8>()
                                .cast::<*mut u8>())
                            .read())
                            .wrapping_add(278))
                            .cast::<u16>())
                            .wrapping_offset(((i) as i32) as isize))
                            .read()) as i32)
                                & 1i32)
                                != 0
                            {
                                let __p11 = (((((&raw mut sPokedexAreaScreen)
                                    .cast::<u8>()
                                    .cast::<*mut u8>())
                                .read())
                                .wrapping_add(278))
                                .cast::<u16>())
                                .wrapping_offset(((i) as i32) as isize);
                                (__p11).write((((((__p11).read()) as i32) & (-193i32)) as u16));
                            }
                            if (((((((((&raw mut sPokedexAreaScreen)
                                .cast::<u8>()
                                .cast::<*mut u8>())
                            .read())
                            .wrapping_add(278))
                            .cast::<u16>())
                            .wrapping_offset(((i) as i32) as isize))
                            .read()) as i32)
                                & 8i32)
                                != 0
                            {
                                let __p12 = (((((&raw mut sPokedexAreaScreen)
                                    .cast::<u8>()
                                    .cast::<*mut u8>())
                                .read())
                                .wrapping_add(278))
                                .cast::<u16>())
                                .wrapping_offset(((i) as i32) as isize);
                                (__p12).write((((((__p12).read()) as i32) & (-81i32)) as u16));
                            }
                            if (((((((((&raw mut sPokedexAreaScreen)
                                .cast::<u8>()
                                .cast::<*mut u8>())
                            .read())
                            .wrapping_add(278))
                            .cast::<u16>())
                            .wrapping_offset(((i) as i32) as isize))
                            .read()) as i32)
                                & 4i32)
                                != 0
                            {
                                let __p13 = (((((&raw mut sPokedexAreaScreen)
                                    .cast::<u8>()
                                    .cast::<*mut u8>())
                                .read())
                                .wrapping_add(278))
                                .cast::<u16>())
                                .wrapping_offset(((i) as i32) as isize);
                                (__p13).write((((((__p13).read()) as i32) & (-161i32)) as u16));
                            }
                            ((((((&raw mut sPokedexAreaScreen).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(278))
                            .cast::<u16>())
                            .wrapping_offset(((i) as i32) as isize))
                            .write(
                                ((((((&raw const sAreaGlowTilemapMapping)
                                    .cast::<u8>()
                                    .cast_mut())
                                .cast::<u8>())
                                .wrapping_offset(
                                    ((((((((&raw mut sPokedexAreaScreen)
                                        .cast::<u8>()
                                        .cast::<*mut u8>())
                                    .read())
                                    .wrapping_add(278))
                                    .cast::<u16>())
                                    .wrapping_offset(((i) as i32) as isize))
                                    .read()) as i32) as isize,
                                ))
                                .read()) as u16),
                            );
                            let __p14 = (((((&raw mut sPokedexAreaScreen)
                                .cast::<u8>()
                                .cast::<*mut u8>())
                            .read())
                            .wrapping_add(278))
                            .cast::<u16>())
                            .wrapping_offset(((i) as i32) as isize);
                            (__p14).write((((((__p14).read()) as i32) | 40960i32) as u16));
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn StartAreaGlow() {
    unsafe {
        if ((((((&raw mut sPokedexAreaScreen).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(274)
            .cast::<u16>())
        .read())
            != 0)
            && (((((((&raw mut sPokedexAreaScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(272)
                .cast::<u16>())
            .read()) as i32)
                == 0i32)
        {
            ((((&raw mut sPokedexAreaScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(1566))
            .write(1u8);
        } else {
            ((((&raw mut sPokedexAreaScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(1566))
            .write(0u8);
        }
        ((((&raw mut sPokedexAreaScreen).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(1558)
            .cast::<u16>())
        .write(0u16);
        ((((&raw mut sPokedexAreaScreen).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(1560)
            .cast::<u16>())
        .write(0u16);
        ((((&raw mut sPokedexAreaScreen).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(1562)
            .cast::<u16>())
        .write(0u16);
        ((((&raw mut sPokedexAreaScreen).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(1564)
            .cast::<u16>())
        .write(64u16);
        ((((&raw mut sPokedexAreaScreen).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(1567))
        .write(1u8);
        SetGpuReg(80u8, 16196u16);
        SetGpuReg(82u8, 4096u16);
        DoAreaGlow();
    }
}
pub(crate) unsafe extern "C" fn DoAreaGlow() {
    unsafe {
        let mut x: u16 = 0u16;
        let mut y: u16 = 0u16;
        let mut i: u16 = 0u16;
        if !((((((&raw mut sPokedexAreaScreen).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(1566))
        .read())
            != 0)
        {
            if ((((((&raw mut sPokedexAreaScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(1558)
                .cast::<u16>())
            .read()) as i32)
                == 0i32
            {
                let __p1 = (((&raw mut sPokedexAreaScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(1560)
                    .cast::<u16>();
                (__p1).write(((__p1).read()).wrapping_add(1));
                if (((((((&raw mut sPokedexAreaScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(1560)
                    .cast::<u16>())
                .read()) as i32)
                    & 1i32)
                    != 0
                {
                    ((((&raw mut sPokedexAreaScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(1562)
                        .cast::<u16>())
                    .write(
                        ((((((((&raw mut sPokedexAreaScreen).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(1562)
                        .cast::<u16>())
                        .read()) as i32)
                            .wrapping_add(4i32)
                            & 127i32) as u16),
                    );
                } else {
                    ((((&raw mut sPokedexAreaScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(1564)
                        .cast::<u16>())
                    .write(
                        ((((((((&raw mut sPokedexAreaScreen).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(1564)
                        .cast::<u16>())
                        .read()) as i32)
                            .wrapping_add(4i32)
                            & 127i32) as u16),
                    );
                }
                x = ((((((((&raw mut gSineTable).cast::<i16>()).cast::<i16>()).wrapping_offset(
                    ((((((&raw mut sPokedexAreaScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(1562)
                        .cast::<u16>())
                    .read()) as i32) as isize,
                ))
                .read()) as i32)
                    >> 4) as u16);
                y = ((((((((&raw mut gSineTable).cast::<i16>()).cast::<i16>()).wrapping_offset(
                    ((((((&raw mut sPokedexAreaScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(1564)
                        .cast::<u16>())
                    .read()) as i32) as isize,
                ))
                .read()) as i32)
                    >> 4) as u16);
                SetGpuReg(82u8, (((((y) as i32) << 8) | ((x) as i32)) as u16));
                ((((&raw mut sPokedexAreaScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(1558)
                    .cast::<u16>())
                .write(0u16);
                if ((((((&raw mut sPokedexAreaScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(1560)
                    .cast::<u16>())
                .read()) as i32)
                    == 64i32
                {
                    ((((&raw mut sPokedexAreaScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(1560)
                        .cast::<u16>())
                    .write(0u16);
                    if ((((((&raw mut sPokedexAreaScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(274)
                        .cast::<u16>())
                    .read()) as i32)
                        != 0i32
                    {
                        ((((&raw mut sPokedexAreaScreen).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(1566))
                        .write(1u8);
                    }
                }
            } else {
                let __p2 = (((&raw mut sPokedexAreaScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(1558)
                    .cast::<u16>();
                (__p2).write(((__p2).read()).wrapping_sub(1));
            }
        } else {
            let __p3 = (((&raw mut sPokedexAreaScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(1558)
                .cast::<u16>();
            (__p3).write(((__p3).read()).wrapping_add(1));
            if ((((((&raw mut sPokedexAreaScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(1558)
                .cast::<u16>())
            .read()) as i32)
                > 12i32
            {
                ((((&raw mut sPokedexAreaScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(1558)
                    .cast::<u16>())
                .write(0u16);
                let __p4 = (((&raw mut sPokedexAreaScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(1567);
                (__p4).write(((__p4).read()).wrapping_add(1));
                {
                    i = 0u16;
                    'l1: loop {
                        if !(((i) as i32)
                            < ((((((&raw mut sPokedexAreaScreen).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(274)
                            .cast::<u16>())
                            .read()) as i32))
                        {
                            break 'l1;
                        }
                        'l2: {
                            crate::c::bf_write(
                                (((((((&raw mut sPokedexAreaScreen)
                                    .cast::<u8>()
                                    .cast::<*mut u8>())
                                .read())
                                .wrapping_add(1632))
                                .cast::<*mut u8>())
                                .wrapping_offset(((i) as i32) as isize))
                                .read())
                                .wrapping_add(62),
                                2,
                                1,
                                ((((((((&raw mut sPokedexAreaScreen)
                                    .cast::<u8>()
                                    .cast::<*mut u8>())
                                .read())
                                .wrapping_add(1567))
                                .read()) as i32)
                                    & 1i32) as u16) as i32,
                            );
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                if ((((((&raw mut sPokedexAreaScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(1567))
                .read()) as i32)
                    > 4i32
                {
                    ((((&raw mut sPokedexAreaScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(1567))
                    .write(1u8);
                    if ((((((&raw mut sPokedexAreaScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(272)
                        .cast::<u16>())
                    .read()) as i32)
                        != 0i32
                    {
                        ((((&raw mut sPokedexAreaScreen).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(1566))
                        .write(0u8);
                    }
                }
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ShowPokedexAreaScreen(species: u16, screenSwitchState: *mut u8) {
    unsafe {
        let mut species = species;
        let mut screenSwitchState = screenSwitchState;
        let mut taskId: u8 = 0u8;
        ((&raw mut sPokedexAreaScreen).cast::<u8>().cast::<*mut u8>()).write(AllocZeroed(5564u32));
        ((((&raw mut sPokedexAreaScreen).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(14)
            .cast::<u16>())
        .write(species);
        ((((&raw mut sPokedexAreaScreen).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(1768)
            .cast::<*mut u8>())
        .write(screenSwitchState);
        (screenSwitchState).write(0u8);
        taskId = CreateTask(Some(Task_ShowPokedexAreaScreen), 0u8);
        (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .write(0i16);
    }
}
pub(crate) unsafe extern "C" fn Task_ShowPokedexAreaScreen(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        'l1: {
            let __sw1 = (((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .read()) as i32);
            if __sw1 == 0i32 {
                ResetSpriteData();
                FreeAllSpritePalettes();
                HideBg(3u8);
                HideBg(2u8);
                HideBg(0u8);
                break 'l1;
            }
            if __sw1 == 1i32 {
                SetBgAttribute(3u8, 1u8, 3u8);
                LoadPokedexAreaMapGfx((&raw const sPokedexAreaMapTemplate).cast::<u8>().cast_mut());
                StringFill(
                    ((((&raw mut sPokedexAreaScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(3952))
                    .cast::<u8>(),
                    0u8,
                    16u16,
                );
                break 'l1;
            }
            if __sw1 == 2i32 {
                if TryShowPokedexAreaMap() == 1u32 {
                    return;
                }
                PokedexAreaMapChangeBgY(4294967288u32);
                break 'l1;
            }
            if __sw1 == 3i32 {
                ResetDrawAreaGlowState();
                break 'l1;
            }
            if __sw1 == 4i32 {
                if (DrawAreaGlow()) != 0 {
                    return;
                }
                break 'l1;
            }
            if __sw1 == 5i32 {
                ShowRegionMapForPokedexAreaScreen(
                    (((&raw mut sPokedexAreaScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(1772),
                );
                CreateRegionMapPlayerIcon(1u16, 1u16);
                PokedexAreaScreen_UpdateRegionMapVariablesAndVideoRegs(0i16, (-8i16));
                break 'l1;
            }
            if __sw1 == 6i32 {
                CreateAreaMarkerSprites();
                break 'l1;
            }
            if __sw1 == 7i32 {
                LoadAreaUnknownGraphics();
                break 'l1;
            }
            if __sw1 == 8i32 {
                CreateAreaUnknownSprites();
                break 'l1;
            }
            if __sw1 == 9i32 {
                BeginNormalPaletteFade(4294967275u32, 0i8, 16u8, 0u8, 0u16);
                break 'l1;
            }
            if __sw1 == 10i32 {
                SetGpuReg(80u8, 16193u16);
                StartAreaGlow();
                ShowBg(2u8);
                ShowBg(3u8);
                SetGpuRegBits(0u8, 4096u16);
                break 'l1;
            }
            if __sw1 == 11i32 {
                ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
                .write(Some(Task_HandlePokedexAreaScreenInput));
                (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .write(0i16);
                return;
            }
        }
        let __p2 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        (__p2).write(((__p2).read()).wrapping_add(1));
    }
}
pub(crate) unsafe extern "C" fn Task_HandlePokedexAreaScreenInput(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        DoAreaGlow();
        'l1: {
            let __sw1 = (((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .read()) as i32);
            let __matched = __sw1 == 0i32 || __sw1 == 1i32 || __sw1 == 2i32 || __sw1 == 3i32;
            let mut __fall = false;
            if !__matched {
                __fall = true;
                (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .write(0i16);
            }
            if __fall || __sw1 == 0i32 {
                __fall = true;
                if (crate::c::bf_read(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
                    7,
                    1,
                    false,
                ) as u16)
                    != 0
                {
                    return;
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                __fall = true;
                if ((((((&raw mut gMain).cast::<u8>())
                    .wrapping_add(46)
                    .cast::<u16>())
                .read()) as i32)
                    & 2i32)
                    != 0
                {
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(1))
                    .write(1i16);
                    PlaySE(3u16);
                } else {
                    if (((((((&raw mut gMain).cast::<u8>())
                        .wrapping_add(46)
                        .cast::<u16>())
                    .read()) as i32)
                        & 16i32)
                        != 0)
                        || ((((((((&raw mut gMain).cast::<u8>())
                            .wrapping_add(46)
                            .cast::<u16>())
                        .read()) as i32)
                            & 256i32)
                            != 0)
                            && (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(19))
                            .read()) as i32)
                                == 1i32))
                    {
                        ((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(1))
                        .write(2i16);
                        PlaySE(109u16);
                    } else {
                        return;
                    }
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                __fall = true;
                BeginNormalPaletteFade(4294967275u32, 0i8, 0u8, 16u8, 0u16);
                break 'l1;
            }
            if __sw1 == 3i32 {
                __fall = true;
                if (crate::c::bf_read(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
                    7,
                    1,
                    false,
                ) as u16)
                    != 0
                {
                    return;
                }
                DestroyAreaScreenSprites();
                (((((&raw mut sPokedexAreaScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(1768)
                    .cast::<*mut u8>())
                .read())
                .write(
                    ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(1))
                    .read()) as u8),
                );
                ResetPokedexAreaMapBg();
                DestroyTask(taskId);
                FreePokedexAreaMapBgNum();
                {
                    Free(((&raw mut sPokedexAreaScreen).cast::<u8>().cast::<*mut u8>()).read());
                    ((&raw mut sPokedexAreaScreen).cast::<u8>().cast::<*mut u8>())
                        .write(core::ptr::null_mut());
                }
                return;
            }
        }
        let __p2 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        (__p2).write(((__p2).read()).wrapping_add(1));
    }
}
pub(crate) unsafe extern "C" fn ResetPokedexAreaMapBg() {
    unsafe {
        SetBgAttribute(3u8, 1u8, 0u8);
        SetBgAttribute(3u8, 4u8, 0u8);
    }
}
pub(crate) unsafe extern "C" fn CreateAreaMarkerSprites() {
    unsafe {
        let mut spriteId: u8 = 0u8;
        LoadSpriteSheet((&raw const sAreaMarkerSpriteSheet).cast::<u8>().cast_mut());
        LoadSpritePalette(
            (&raw const sAreaMarkerSpritePalette)
                .cast::<u8>()
                .cast_mut(),
        );
        (&raw mut CREATEAREAMARKERSPRITES_NUMSPRITES).write(0i16);
        {
            (&raw mut CREATEAREAMARKERSPRITES_I).write(0i16);
            'l1: loop {
                if !((((&raw mut CREATEAREAMARKERSPRITES_I).read()) as i32)
                    < ((((((&raw mut sPokedexAreaScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(274)
                        .cast::<u16>())
                    .read()) as i32))
                {
                    break 'l1;
                }
                'l2: {
                    (&raw mut CREATEAREAMARKERSPRITES_MAPSECID).write(
                        ((((((((&raw mut sPokedexAreaScreen).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(1568))
                        .cast::<u16>())
                        .wrapping_offset(
                            (((&raw mut CREATEAREAMARKERSPRITES_I).read()) as i32) as isize,
                        ))
                        .read()) as i16),
                    );
                    (&raw mut CREATEAREAMARKERSPRITES_X).write(
                        ((((8i32).wrapping_mul(
                            (((((&raw mut gRegionMapEntries).cast::<u8>()).wrapping_offset(
                                (((&raw mut CREATEAREAMARKERSPRITES_MAPSECID).read()) as i32)
                                    as isize
                                    * 8,
                            ))
                            .read()) as i32)
                                .wrapping_add(1i32),
                        ))
                        .wrapping_add(4i32)) as i16),
                    );
                    (&raw mut CREATEAREAMARKERSPRITES_Y).write(
                        ((((8i32).wrapping_mul(
                            ((((((&raw mut gRegionMapEntries).cast::<u8>()).wrapping_offset(
                                (((&raw mut CREATEAREAMARKERSPRITES_MAPSECID).read()) as i32)
                                    as isize
                                    * 8,
                            ))
                            .wrapping_add(1))
                            .read()) as i32),
                        ))
                        .wrapping_add(28i32)) as i16),
                    );
                    (&raw mut CREATEAREAMARKERSPRITES_X).write(
                        (((((&raw mut CREATEAREAMARKERSPRITES_X).read()) as i32).wrapping_add(
                            (4i32).wrapping_mul(
                                ((((((&raw mut gRegionMapEntries).cast::<u8>()).wrapping_offset(
                                    (((&raw mut CREATEAREAMARKERSPRITES_MAPSECID).read()) as i32)
                                        as isize
                                        * 8,
                                ))
                                .wrapping_add(2))
                                .read()) as i32)
                                    .wrapping_sub(1i32),
                            ),
                        )) as i16),
                    );
                    (&raw mut CREATEAREAMARKERSPRITES_Y).write(
                        (((((&raw mut CREATEAREAMARKERSPRITES_Y).read()) as i32).wrapping_add(
                            (4i32).wrapping_mul(
                                ((((((&raw mut gRegionMapEntries).cast::<u8>()).wrapping_offset(
                                    (((&raw mut CREATEAREAMARKERSPRITES_MAPSECID).read()) as i32)
                                        as isize
                                        * 8,
                                ))
                                .wrapping_add(3))
                                .read()) as i32)
                                    .wrapping_sub(1i32),
                            ),
                        )) as i16),
                    );
                    spriteId = CreateSprite(
                        (&raw const sAreaMarkerSpriteTemplate)
                            .cast::<u8>()
                            .cast_mut(),
                        (&raw mut CREATEAREAMARKERSPRITES_X).read(),
                        (&raw mut CREATEAREAMARKERSPRITES_Y).read(),
                        0u8,
                    );
                    if ((spriteId) as i32) != 64i32 {
                        crate::c::bf_write(
                            (((&raw mut gSprites).cast::<u8>())
                                .wrapping_offset(((spriteId) as i32) as isize * 68))
                            .wrapping_add(62),
                            2,
                            1,
                            (1u16) as i32,
                        );
                        ((((((&raw mut sPokedexAreaScreen).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(1632))
                        .cast::<*mut u8>())
                        .wrapping_offset(
                            (({
                                let __t1 = (&raw mut CREATEAREAMARKERSPRITES_NUMSPRITES).read();
                                (&raw mut CREATEAREAMARKERSPRITES_NUMSPRITES).write(
                                    ((&raw mut CREATEAREAMARKERSPRITES_NUMSPRITES).read())
                                        .wrapping_add(1),
                                );
                                __t1
                            }) as i32) as isize,
                        ))
                        .write(
                            ((&raw mut gSprites).cast::<u8>())
                                .wrapping_offset(((spriteId) as i32) as isize * 68),
                        );
                    }
                }
                (&raw mut CREATEAREAMARKERSPRITES_I)
                    .write(((&raw mut CREATEAREAMARKERSPRITES_I).read()).wrapping_add(1));
            }
        }
        ((((&raw mut sPokedexAreaScreen).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(1760)
            .cast::<u16>())
        .write((((&raw mut CREATEAREAMARKERSPRITES_NUMSPRITES).read()) as u16));
    }
}
pub(crate) unsafe extern "C" fn DestroyAreaScreenSprites() {
    unsafe {
        let mut i: u16 = 0u16;
        FreeSpriteTilesByTag(2u16);
        FreeSpritePaletteByTag(2u16);
        {
            i = 0u16;
            'l1: loop {
                if !(((i) as i32)
                    < ((((((&raw mut sPokedexAreaScreen).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(1760)
                        .cast::<u16>())
                    .read()) as i32))
                {
                    break 'l1;
                }
                'l2: {
                    DestroySprite(
                        ((((((&raw mut sPokedexAreaScreen).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(1632))
                        .cast::<*mut u8>())
                        .wrapping_offset(((i) as i32) as isize))
                        .read(),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        FreeSpriteTilesByTag(3u16);
        FreeSpritePaletteByTag(3u16);
        {
            i = 0u16;
            'l3: loop {
                if !(((i) as u32) < crate::c::div_u32(12u32, 4u32)) {
                    break 'l3;
                }
                'l4: {
                    if !(((((((&raw mut sPokedexAreaScreen).cast::<u8>().cast::<*mut u8>())
                        .read())
                    .wrapping_add(4016))
                    .cast::<*mut u8>())
                    .wrapping_offset(((i) as i32) as isize))
                    .read())
                    .is_null()
                    {
                        DestroySprite(
                            ((((((&raw mut sPokedexAreaScreen).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(4016))
                            .cast::<*mut u8>())
                            .wrapping_offset(((i) as i32) as isize))
                            .read(),
                        );
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn LoadAreaUnknownGraphics() {
    unsafe {
        let mut spriteSheet = crate::ffi::Align4([0u8; 8]);
        (&raw mut spriteSheet)
            .cast::<u8>()
            .wrapping_add(0)
            .cast::<*mut u8>()
            .write(
                ((((&raw mut sPokedexAreaScreen).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(4028))
                .cast::<u8>(),
            );
        (&raw mut spriteSheet)
            .cast::<u8>()
            .wrapping_add(4)
            .cast::<u16>()
            .write(1536u16);
        (&raw mut spriteSheet)
            .cast::<u8>()
            .wrapping_add(6)
            .cast::<u16>()
            .write(3u16);
        LZ77UnCompWram(
            ((&raw mut gPokedexAreaScreenAreaUnknown_Gfx).cast::<u32>()).cast::<u32>(),
            ((((&raw mut sPokedexAreaScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(4028))
            .cast::<u8>(),
        );
        LoadSpriteSheet((&raw mut spriteSheet).cast::<u8>());
        LoadSpritePalette(
            (&raw const sAreaUnknownSpritePalette)
                .cast::<u8>()
                .cast_mut(),
        );
    }
}
pub(crate) unsafe extern "C" fn CreateAreaUnknownSprites() {
    unsafe {
        let mut i: u16 = 0u16;
        if ((((((&raw mut sPokedexAreaScreen).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(272)
            .cast::<u16>())
        .read())
            != 0)
            || ((((((&raw mut sPokedexAreaScreen).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(274)
                .cast::<u16>())
            .read())
                != 0)
        {
            {
                i = 0u16;
                'l1: loop {
                    if !(((i) as u32) < crate::c::div_u32(12u32, 4u32)) {
                        break 'l1;
                    }
                    'l2: {
                        ((((((&raw mut sPokedexAreaScreen).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(4016))
                        .cast::<*mut u8>())
                        .wrapping_offset(((i) as i32) as isize))
                        .write(core::ptr::null_mut());
                    }
                    i = (i).wrapping_add(1);
                }
            }
        } else {
            {
                i = 0u16;
                'l3: loop {
                    if !(((i) as u32) < crate::c::div_u32(12u32, 4u32)) {
                        break 'l3;
                    }
                    'l4: {
                        let mut spriteId: u8 = CreateSprite(
                            (&raw const sAreaUnknownSpriteTemplate)
                                .cast::<u8>()
                                .cast_mut(),
                            (((((i) as i32).wrapping_mul(32i32)).wrapping_add(160i32)) as i16),
                            140i16,
                            0u8,
                        );
                        if ((spriteId) as i32) != 64i32 {
                            crate::c::bf_write(
                                (((&raw mut gSprites).cast::<u8>())
                                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                                .wrapping_add(4),
                                0,
                                10,
                                ((((crate::c::bf_read(
                                    (((&raw mut gSprites).cast::<u8>())
                                        .wrapping_offset(((spriteId) as i32) as isize * 68))
                                    .wrapping_add(4),
                                    0,
                                    10,
                                    false,
                                ) as u16) as i32)
                                    .wrapping_add(((i) as i32).wrapping_mul(16i32)))
                                    as u16) as i32,
                            );
                            ((((((&raw mut sPokedexAreaScreen).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(4016))
                            .cast::<*mut u8>())
                            .wrapping_offset(((i) as i32) as isize))
                            .write(
                                ((&raw mut gSprites).cast::<u8>())
                                    .wrapping_offset(((spriteId) as i32) as isize * 68),
                            );
                        } else {
                            ((((((&raw mut sPokedexAreaScreen).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(4016))
                            .cast::<*mut u8>())
                            .wrapping_offset(((i) as i32) as isize))
                            .write(core::ptr::null_mut());
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
        }
    }
}
