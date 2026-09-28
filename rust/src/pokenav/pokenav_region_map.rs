//! Translated from `src/pokenav_region_map.c` by tools/rustport/c2rs.py.
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
// Data tables (translate with cdata.py): sMapSecInfoWindow_Pal sRegionMapCityZoomTiles_Gfx gPokenavCityMap_Lavaridge_0 gPokenavCityMap_Fallarbor_0 gPokenavCityMap_Fortree_0 gPokenavCityMap_Slateport_0 gPokenavCityMap_Slateport_1 gPokenavCityMap_Rustboro_0 gPokenavCityMap_Rustboro_1 gPokenavCityMap_Pacifidlog_0 gPokenavCityMap_Mauville_1 gPokenavCityMap_Mauville_0 gPokenavCityMap_Oldale_0 gPokenavCityMap_Lilycove_1 gPokenavCityMap_Lilycove_0 gPokenavCityMap_Littleroot_0 gPokenavCityMap_Dewford_0 gPokenavCityMap_Sootopolis_0 gPokenavCityMap_EverGrande_0 gPokenavCityMap_EverGrande_1 gPokenavCityMap_Verdanturf_0 gPokenavCityMap_Mossdeep_1 gPokenavCityMap_Mossdeep_0 gPokenavCityMap_Petalburg_0 sRegionMapBgTemplates sRegionMapLoopTaskFuncs sCityZoomTextSpriteSheet sCityZoomTilesSpritePalette sMapSecInfoWindowTemplate sPokenavCityMaps sCityZoomTextSprite_OamData sCityZoomTextSpriteTemplate

/// `struct Pokenav_RegionMapMenu`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Pokenav_RegionMapMenu {
    pub unused: CArray<u8, 12>,
    pub zoomDisabled: u32,
    pub callback: Option<unsafe extern "C" fn(*mut Pokenav_RegionMapMenu) -> u32>,
}

unsafe impl Sync for Pokenav_RegionMapMenu {}

/// `struct Pokenav_RegionMapGfx`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Pokenav_RegionMapGfx {
    pub isTaskActiveCB: Option<unsafe extern "C" fn() -> u32>,
    pub loopTaskId: u32,
    pub infoWindowId: u16,
    pub cityZoomTextSprites: CArray<*mut Sprite, 3>,
    pub tilemapBuffer: CArray<u8, 2048>,
    pub cityZoomPics: CArray<CArray<u8, 200>, 22>,
}

unsafe impl Sync for Pokenav_RegionMapGfx {}

/// `struct CityMapEntry`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CityMapEntry {
    pub mapSecId: u16,
    pub index: u16,
    pub tilemap: *mut u32,
}

unsafe impl Sync for CityMapEntry {}

#[cfg(target_arch = "arm")]
const _: () = {
    #[allow(unused_imports)]
    use core::mem::{offset_of, size_of};
    assert!(size_of::<Pokenav_RegionMapMenu>() == 20);
    assert!(offset_of!(Pokenav_RegionMapMenu, unused) == 0);
    assert!(offset_of!(Pokenav_RegionMapMenu, zoomDisabled) == 12);
    assert!(offset_of!(Pokenav_RegionMapMenu, callback) == 16);
    assert!(size_of::<Pokenav_RegionMapGfx>() == 6472);
    assert!(offset_of!(Pokenav_RegionMapGfx, isTaskActiveCB) == 0);
    assert!(offset_of!(Pokenav_RegionMapGfx, loopTaskId) == 4);
    assert!(offset_of!(Pokenav_RegionMapGfx, infoWindowId) == 8);
    assert!(offset_of!(Pokenav_RegionMapGfx, cityZoomTextSprites) == 12);
    assert!(offset_of!(Pokenav_RegionMapGfx, tilemapBuffer) == 24);
    assert!(offset_of!(Pokenav_RegionMapGfx, cityZoomPics) == 2072);
    assert!(size_of::<CityMapEntry>() == 8);
    assert!(offset_of!(CityMapEntry, mapSecId) == 0);
    assert!(offset_of!(CityMapEntry, index) == 2);
    assert!(offset_of!(CityMapEntry, tilemap) == 4);
};

const GFXTAG_CITY_ZOOM: u16 = 6;
const NUM_CITY_MAPS: i32 = 22;
const PALTAG_CITY_ZOOM: u16 = 11;

static sCityZoomTextSpriteSheet: Table<CArray<CompressedSpriteSheet, 1>> =
    Table((&raw const crate::data::pokenav_region_map::sCityZoomTextSpriteSheet).cast());
static sCityZoomTextSpriteTemplate: Table<SpriteTemplate> =
    Table((&raw const crate::data::pokenav_region_map::sCityZoomTextSpriteTemplate).cast());
static sCityZoomTilesSpritePalette: Table<CArray<SpritePalette, 2>> =
    Table((&raw const crate::data::pokenav_region_map::sCityZoomTilesSpritePalette).cast());
static sMapSecInfoWindowTemplate: Table<WindowTemplate> =
    Table((&raw const crate::data::pokenav_region_map::sMapSecInfoWindowTemplate).cast());
static sMapSecInfoWindow_Pal: Table<CArray<u16, 16>> =
    Table((&raw const crate::data::pokenav_region_map::sMapSecInfoWindow_Pal).cast());
static sPokenavCityMaps: Table<CArray<CityMapEntry, 22>> =
    Table((&raw const crate::data::pokenav_region_map::sPokenavCityMaps).cast());
static sRegionMapBgTemplates: Table<CArray<BgTemplate, 3>> =
    Table((&raw const crate::data::pokenav_region_map::sRegionMapBgTemplates).cast());
static sRegionMapCityZoomTiles_Gfx: Table<CArray<u32, 125>> =
    Table((&raw const crate::data::pokenav_region_map::sRegionMapCityZoomTiles_Gfx).cast());
static sRegionMapLoopTaskFuncs: Table<CArray<Option<unsafe extern "C" fn(i32) -> u32>, 5>> =
    Table((&raw const crate::data::pokenav_region_map::sRegionMapLoopTaskFuncs).cast());

unsafe extern "C" {
    static mut gMain: Main;
    static mut gMapHeader: MapHeader;
    static gRegionMapCityZoomTiles_Pal: CArray<u16, 0>;
    static mut gSaveBlock2Ptr: *mut SaveBlock2;
    static mut gSprites: CArray<Sprite, 65>;
    static mut gStringVar1: CArray<u8, 256>;
    static mut gTasks: CArray<Task, 0>;
    fn AddTextPrinterParameterized(
        a0: u8,
        a1: u8,
        a2: *mut u8,
        a3: u8,
        a4: u8,
        a5: u8,
        a6: Option<unsafe extern "C" fn(*mut TextPrinterTemplate, u16)>,
    ) -> u16;
    fn AddWindow(a0: *mut WindowTemplate) -> u16;
    fn AllocSubstruct(a0: u32, a1: u32) -> *mut c_void;
    fn AreLeftHeaderSpritesMoving() -> u32;
    fn BgDmaFill(a0: u32, a1: u8, a2: i32, a3: i32);
    fn BlendRegionMap(a0: u16, a1: u32);
    fn ChangeBgX(a0: u8, a1: i32, a2: u8) -> i32;
    fn ChangeBgY(a0: u8, a1: i32, a2: u8) -> i32;
    fn CopyBgTilemapBufferToVram(a0: u8);
    fn CopyPaletteIntoBufferUnfaded(a0: *mut u16, a1: u32, a2: u32);
    fn CopyToBgTilemapBufferRect(a0: u8, a1: *mut c_void, a2: u8, a3: u8, a4: u8, a5: u8);
    fn CopyWindowToVram(a0: u8, a1: u8);
    fn CpuSet(a0: *mut c_void, a1: *mut c_void, a2: u32);
    fn CreateLoopedTask(a0: Option<unsafe extern "C" fn(i32) -> u32>, a1: u32) -> u32;
    fn CreateRegionMapCursor(a0: u16, a1: u16);
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
    fn DoRegionMapInputCallback() -> u8;
    fn DrawTextBorderOuter(a0: u8, a1: u16, a2: u8);
    fn FadeToBlackExceptPrimary();
    fn FillBgTilemapBufferRect(a0: u8, a1: u16, a2: u8, a3: u8, a4: u8, a5: u8, a6: u8);
    fn FillBgTilemapBufferRect_Palette0(a0: u8, a1: u16, a2: u8, a3: u8, a4: u8, a5: u8);
    fn FillWindowPixelBuffer(a0: u8, a1: u8);
    fn FreePokenavSubstruct(a0: u32);
    fn FreeRegionMapIconResources();
    fn FreeSpritePaletteByTag(a0: u16);
    fn FreeSpriteTilesByTag(a0: u16);
    fn FreeTempTileDataBuffersIfPossible() -> u8;
    fn FuncIsActiveLoopedTask(a0: Option<unsafe extern "C" fn(i32) -> u32>) -> u32;
    fn FuncIsActiveTask(a0: Option<unsafe extern "C" fn(u8)>) -> u8;
    fn GetBgY(a0: u8) -> i32;
    fn GetLandmarkName(a0: u8, a1: u8, a2: u8) -> *mut u8;
    fn GetSubstructPtr(a0: u32) -> *mut c_void;
    fn HideBg(a0: u8);
    fn InitBgTemplates(a0: *mut BgTemplate, a1: i32);
    fn InitRegionMapData(a0: *mut RegionMap, a1: *mut BgTemplate, a2: u8);
    fn IsDma3ManagerBusyWithBgCopy() -> u8;
    fn IsEventIslandMapSecId(a0: u8) -> u32;
    fn IsLoopedTaskActive(a0: u32) -> u32;
    fn IsPaletteFadeActive() -> u32;
    fn IsRegionMapZoomed() -> u8;
    fn LZ77UnCompWram(a0: *mut u32, a1: *mut c_void);
    fn LoadCompressedSpriteSheet(a0: *mut CompressedSpriteSheet) -> u16;
    fn LoadLeftHeaderGfxForIndex(a0: u32);
    fn LoadOam();
    fn LoadRegionMapGfx() -> u8;
    fn LoadUserWindowBorderGfx_(a0: u8, a1: u16, a2: u8);
    fn MainMenuLoopedTaskIsBusy() -> u32;
    fn PlaySE(a0: u16);
    fn PokenavFadeScreen(a0: i32);
    fn Pokenav_AllocAndLoadPalettes(a0: *mut SpritePalette);
    fn PrintHelpBarText(a0: u32);
    fn ProcessSpriteCopyRequests();
    fn PutWindowRectTilemap(a0: u8, a1: u8, a2: u8, a3: u8, a4: u8);
    fn PutWindowTilemap(a0: u8);
    fn RemoveWindow(a0: u8);
    fn SetBgMode(a0: u8);
    fn SetBgTilemapBuffer(a0: u8, a1: *mut c_void);
    fn SetLeftHeaderSpritesInvisibility();
    fn SetPokenavVBlankCallback();
    fn SetRegionMapDataForZoom();
    fn SetVBlankCallback_(a0: Option<unsafe extern "C" fn()>);
    fn ShowBg(a0: u8);
    fn ShowLeftHeaderGfx(a0: u32, a1: u32, a2: u32);
    fn SlideMenuHeaderDown();
    fn StringCopyPadded(a0: *mut u8, a1: *mut u8, a2: u8, a3: u16) -> *mut u8;
    fn TransferPlttBuffer();
    fn TrySetPlayerIconBlink();
    fn UpdateRegionMapRightHeaderTiles(a0: u32);
    fn UpdateRegionMapVideoRegs();
    fn UpdateRegionMapZoom() -> u8;
    fn WaitForHelpBar() -> u32;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn PokenavCallback_Init_RegionMap() -> u32 {
    let mut state: *mut Pokenav_RegionMapMenu =
        AllocSubstruct(POKENAV_SUBSTRUCT_REGION_MAP_STATE, 20) as *mut Pokenav_RegionMapMenu;
    if state.is_null() {
        return FALSE as u32;
    }
    if AllocSubstruct(POKENAV_SUBSTRUCT_REGION_MAP, 2180).is_null() {
        return FALSE as u32;
    }
    (*state).zoomDisabled = IsEventIslandMapSecId(gMapHeader.regionMapSectionId);
    if (*state).zoomDisabled == 0 {
        (*state).callback = Some(HandleRegionMapInput);
    } else {
        (*state).callback = Some(HandleRegionMapInputZoomDisabled);
    }
    return TRUE as u32;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FreeRegionMapSubstruct1() {
    (*gSaveBlock2Ptr).set_regionMapZoom(IsRegionMapZoomed() as u16);
    FreePokenavSubstruct(POKENAV_SUBSTRUCT_REGION_MAP);
    FreePokenavSubstruct(POKENAV_SUBSTRUCT_REGION_MAP_STATE);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetRegionMapCallback() -> u32 {
    let mut state: *mut Pokenav_RegionMapMenu =
        GetSubstructPtr(POKENAV_SUBSTRUCT_REGION_MAP_STATE) as *mut Pokenav_RegionMapMenu;
    return (*state).callback.unwrap_unchecked()(state);
}
pub(crate) unsafe extern "C" fn HandleRegionMapInput(state: *mut Pokenav_RegionMapMenu) -> u32 {
    match DoRegionMapInputCallback() {
        MAP_INPUT_MOVE_END => {
            return POKENAV_MAP_FUNC_CURSOR_MOVED;
        }
        MAP_INPUT_A_BUTTON => {
            if IsRegionMapZoomed() == 0 {
                return POKENAV_MAP_FUNC_ZOOM_IN;
            }
            return POKENAV_MAP_FUNC_ZOOM_OUT;
        }
        MAP_INPUT_B_BUTTON => {
            (*state).callback = Some(GetExitRegionMapMenuId);
            return POKENAV_MAP_FUNC_EXIT;
        }
        _ => {}
    }
    return POKENAV_MAP_FUNC_NONE;
}
pub(crate) unsafe extern "C" fn HandleRegionMapInputZoomDisabled(
    state: *mut Pokenav_RegionMapMenu,
) -> u32 {
    if gMain.newKeys as i32 & B_BUTTON != 0 {
        (*state).callback = Some(GetExitRegionMapMenuId);
        return POKENAV_MAP_FUNC_EXIT;
    }
    return POKENAV_MAP_FUNC_NONE;
}
pub(crate) unsafe extern "C" fn GetExitRegionMapMenuId(state: *mut Pokenav_RegionMapMenu) -> u32 {
    return POKENAV_MAIN_MENU_CURSOR_ON_MAP;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetZoomDisabled() -> u32 {
    let mut state: *mut Pokenav_RegionMapMenu =
        GetSubstructPtr(POKENAV_SUBSTRUCT_REGION_MAP_STATE) as *mut Pokenav_RegionMapMenu;
    return (*state).zoomDisabled;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn OpenPokenavRegionMap() -> u32 {
    let mut state: *mut Pokenav_RegionMapGfx =
        AllocSubstruct(POKENAV_SUBSTRUCT_REGION_MAP_ZOOM, 6472) as *mut Pokenav_RegionMapGfx;
    if state.is_null() {
        return FALSE as u32;
    }
    (*state).loopTaskId = CreateLoopedTask(Some(LoopedTask_OpenRegionMap), 1);
    (*state).isTaskActiveCB = Some(GetCurrentLoopedTaskActive);
    return TRUE as u32;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CreateRegionMapLoopedTask(index: i32) {
    let mut state: *mut Pokenav_RegionMapGfx =
        GetSubstructPtr(POKENAV_SUBSTRUCT_REGION_MAP_ZOOM) as *mut Pokenav_RegionMapGfx;
    (*state).loopTaskId = CreateLoopedTask(sRegionMapLoopTaskFuncs[index], 1);
    (*state).isTaskActiveCB = Some(GetCurrentLoopedTaskActive);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsRegionMapLoopedTaskActive() -> u32 {
    let mut state: *mut Pokenav_RegionMapGfx =
        GetSubstructPtr(POKENAV_SUBSTRUCT_REGION_MAP_ZOOM) as *mut Pokenav_RegionMapGfx;
    return (*state).isTaskActiveCB.unwrap_unchecked()();
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FreeRegionMapSubstruct2() {
    let mut state: *mut Pokenav_RegionMapGfx =
        GetSubstructPtr(POKENAV_SUBSTRUCT_REGION_MAP_ZOOM) as *mut Pokenav_RegionMapGfx;
    FreeRegionMapIconResources();
    FreeCityZoomViewGfx();
    RemoveWindow((*state).infoWindowId as u8);
    FreePokenavSubstruct(POKENAV_SUBSTRUCT_REGION_MAP);
    FreePokenavSubstruct(POKENAV_SUBSTRUCT_REGION_MAP_ZOOM);
    SetPokenavVBlankCallback();
    SetBgMode(0);
}
pub(crate) unsafe extern "C" fn VBlankCB_RegionMap() {
    TransferPlttBuffer();
    LoadOam();
    ProcessSpriteCopyRequests();
    UpdateRegionMapVideoRegs();
}
pub(crate) unsafe extern "C" fn GetCurrentLoopedTaskActive() -> u32 {
    let mut state: *mut Pokenav_RegionMapGfx =
        GetSubstructPtr(POKENAV_SUBSTRUCT_REGION_MAP_ZOOM) as *mut Pokenav_RegionMapGfx;
    return IsLoopedTaskActive((*state).loopTaskId);
}
pub(crate) unsafe extern "C" fn ShouldOpenRegionMapZoomed() -> u8 {
    if GetZoomDisabled() != 0 {
        return FALSE;
    }
    return ((*gSaveBlock2Ptr).regionMapZoom() == TRUE as u16) as u8;
}
pub(crate) unsafe extern "C" fn LoopedTask_OpenRegionMap(taskState: i32) -> u32 {
    let mut menuGfxId: i32 = 0;
    let mut regionMap: *mut RegionMap = null_mut();
    let mut state: *mut Pokenav_RegionMapGfx =
        GetSubstructPtr(POKENAV_SUBSTRUCT_REGION_MAP_ZOOM) as *mut Pokenav_RegionMapGfx;
    match taskState {
        0 => {
            SetVBlankCallback_(None);
            HideBg(1);
            HideBg(2);
            HideBg(3);
            SetBgMode(1);
            InitBgTemplates(sRegionMapBgTemplates.as_ptr().cast_mut(), 2);
            regionMap = GetSubstructPtr(POKENAV_SUBSTRUCT_REGION_MAP) as *mut RegionMap;
            InitRegionMapData(
                regionMap,
                (&raw const sRegionMapBgTemplates[1]).cast_mut(),
                ShouldOpenRegionMapZoomed(),
            );
            LoadCityZoomViewGfx();
            return LT_INC_AND_PAUSE;
        }
        1 => {
            if LoadRegionMapGfx() != 0 {
                return LT_PAUSE;
            }
            if GetZoomDisabled() == 0 {
                CreateRegionMapPlayerIcon(4, 9);
                CreateRegionMapCursor(5, 10);
                TrySetPlayerIconBlink();
            } else {
                BlendRegionMap(0, 6);
            }
            return LT_INC_AND_PAUSE;
        }
        2 => {
            DecompressCityMaps();
            return LT_INC_AND_CONTINUE;
        }
        3 => {
            if IsDecompressCityMapsActive() != 0 {
                return LT_PAUSE;
            }
            LoadPokenavRegionMapGfx(state);
            return LT_INC_AND_CONTINUE;
        }
        4 => {
            if TryFreeTempTileDataBuffers() != 0 {
                return LT_PAUSE;
            }
            UpdateMapSecInfoWindow(state);
            FadeToBlackExceptPrimary();
            return LT_INC_AND_PAUSE;
        }
        5 => {
            if IsDma3ManagerBusyWithBgCopy_(state) != 0 {
                return LT_PAUSE;
            }
            ShowBg(1);
            ShowBg(2);
            SetVBlankCallback_(Some(VBlankCB_RegionMap));
            return LT_INC_AND_PAUSE;
        }
        6 => {
            if ShouldOpenRegionMapZoomed() == 0 {
                menuGfxId = POKENAV_GFX_MAP_MENU_ZOOMED_OUT as i32;
            } else {
                menuGfxId = POKENAV_GFX_MAP_MENU_ZOOMED_IN as i32;
            }
            LoadLeftHeaderGfxForIndex(menuGfxId as u32);
            ShowLeftHeaderGfx(menuGfxId as u32, TRUE as u32, TRUE as u32);
            PokenavFadeScreen(POKENAV_FADE_FROM_BLACK);
            return LT_INC_AND_PAUSE;
        }
        7 => {
            if IsPaletteFadeActive() != 0 || AreLeftHeaderSpritesMoving() != 0 {
                return LT_PAUSE;
            }
            return LT_INC_AND_CONTINUE;
        }
        _ => {
            return LT_FINISH;
        }
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn LoopedTask_UpdateInfoAfterCursorMove(taskState: i32) -> u32 {
    let mut state: *mut Pokenav_RegionMapGfx =
        GetSubstructPtr(POKENAV_SUBSTRUCT_REGION_MAP_ZOOM) as *mut Pokenav_RegionMapGfx;
    match taskState {
        0 => {
            UpdateMapSecInfoWindow(state);
            return LT_INC_AND_PAUSE;
        }
        1 => {
            if IsDma3ManagerBusyWithBgCopy_(state) != 0 {
                return LT_PAUSE;
            }
        }
        _ => {}
    }
    return LT_FINISH;
}
pub(crate) unsafe extern "C" fn LoopedTask_RegionMapZoomOut(taskState: i32) -> u32 {
    match taskState {
        0 => {
            PlaySE(SE_SELECT);
            ChangeBgYForZoom(FALSE as u32);
            SetRegionMapDataForZoom();
            return LT_INC_AND_PAUSE;
        }
        1 => {
            if UpdateRegionMapZoom() != 0 || IsChangeBgYForZoomActive() != 0 {
                return LT_PAUSE;
            }
            PrintHelpBarText(HELPBAR_MAP_ZOOMED_OUT as u32);
            return LT_INC_AND_PAUSE;
        }
        2 => {
            if WaitForHelpBar() != 0 {
                return LT_PAUSE;
            }
            UpdateRegionMapRightHeaderTiles(POKENAV_GFX_MAP_MENU_ZOOMED_OUT);
        }
        _ => {}
    }
    return LT_FINISH;
}
pub(crate) unsafe extern "C" fn LoopedTask_RegionMapZoomIn(taskState: i32) -> u32 {
    let mut state: *mut Pokenav_RegionMapGfx =
        GetSubstructPtr(POKENAV_SUBSTRUCT_REGION_MAP_ZOOM) as *mut Pokenav_RegionMapGfx;
    match taskState {
        0 => {
            PlaySE(SE_SELECT);
            UpdateMapSecInfoWindow(state);
            return LT_INC_AND_PAUSE;
        }
        1 => {
            if IsDma3ManagerBusyWithBgCopy_(state) != 0 {
                return LT_PAUSE;
            }
            ChangeBgYForZoom(TRUE as u32);
            SetRegionMapDataForZoom();
            return LT_INC_AND_PAUSE;
        }
        2 => {
            if UpdateRegionMapZoom() != 0 || IsChangeBgYForZoomActive() != 0 {
                return LT_PAUSE;
            }
            PrintHelpBarText(HELPBAR_MAP_ZOOMED_IN as u32);
            return LT_INC_AND_PAUSE;
        }
        3 => {
            if WaitForHelpBar() != 0 {
                return LT_PAUSE;
            }
            UpdateRegionMapRightHeaderTiles(POKENAV_GFX_MAP_MENU_ZOOMED_IN);
        }
        _ => {}
    }
    return LT_FINISH;
}
pub(crate) unsafe extern "C" fn LoopedTask_ExitRegionMap(taskState: i32) -> u32 {
    match taskState {
        0 => {
            PlaySE(SE_SELECT);
            PokenavFadeScreen(POKENAV_FADE_TO_BLACK);
            return LT_INC_AND_PAUSE;
        }
        1 => {
            if IsPaletteFadeActive() != 0 {
                return LT_PAUSE;
            }
            SetLeftHeaderSpritesInvisibility();
            SlideMenuHeaderDown();
            return LT_INC_AND_PAUSE;
        }
        2 => {
            if MainMenuLoopedTaskIsBusy() != 0 {
                return LT_PAUSE;
            }
            HideBg(1);
            HideBg(2);
            HideBg(3);
            return LT_INC_AND_PAUSE;
        }
        _ => {}
    }
    return LT_FINISH;
}
pub(crate) unsafe extern "C" fn LoadCityZoomViewGfx() {
    let mut i: i32 = 0;
    i = 0;
    while i < 1 {
        LoadCompressedSpriteSheet((&raw const sCityZoomTextSpriteSheet[i]).cast_mut());
        i += 1;
    }
    Pokenav_AllocAndLoadPalettes(sCityZoomTilesSpritePalette.as_ptr().cast_mut());
    CreateCityZoomTextSprites();
}
pub(crate) unsafe extern "C" fn FreeCityZoomViewGfx() {
    let mut i: i32 = 0;
    let mut state: *mut Pokenav_RegionMapGfx =
        GetSubstructPtr(POKENAV_SUBSTRUCT_REGION_MAP_ZOOM) as *mut Pokenav_RegionMapGfx;
    FreeSpriteTilesByTag(GFXTAG_CITY_ZOOM);
    FreeSpritePaletteByTag(PALTAG_CITY_ZOOM);
    i = 0;
    while i < 3 {
        DestroySprite((*state).cityZoomTextSprites[i]);
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn LoadPokenavRegionMapGfx(state: *mut Pokenav_RegionMapGfx) {
    BgDmaFill(1, 0, 0x40, 1);
    BgDmaFill(1, 17, 0x41, 1);
    {
        {
            let mut tmp: u16 = 0;
            volatile_write(&raw mut tmp, 4160);
            CpuSet(
                &raw mut tmp as *mut c_void,
                (*state).tilemapBuffer.as_mut_ptr() as *mut c_void,
                0x1000400,
            );
        }
    }
    SetBgTilemapBuffer(1, (*state).tilemapBuffer.as_mut_ptr() as *mut c_void);
    (*state).infoWindowId = AddWindow((&raw const *sMapSecInfoWindowTemplate).cast_mut());
    LoadUserWindowBorderGfx_((*state).infoWindowId as u8, 0x42, 64);
    DrawTextBorderOuter((*state).infoWindowId as u8, 0x42, 4);
    DecompressAndCopyTileDataToVram(
        1,
        sRegionMapCityZoomTiles_Gfx.as_ptr().cast_mut() as *mut c_void,
        0,
        0,
        0,
    );
    FillWindowPixelBuffer((*state).infoWindowId as u8, 17);
    PutWindowTilemap((*state).infoWindowId as u8);
    CopyWindowToVram((*state).infoWindowId as u8, COPYWIN_FULL);
    CopyPaletteIntoBufferUnfaded(sMapSecInfoWindow_Pal.as_ptr().cast_mut(), 16, 32);
    CopyPaletteIntoBufferUnfaded(gRegionMapCityZoomTiles_Pal.as_ptr().cast_mut(), 48, 32);
    if IsRegionMapZoomed() == 0 {
        ChangeBgY(1, -24576, BG_COORD_SET);
    } else {
        ChangeBgY(1, 0, BG_COORD_SET);
    }
    ChangeBgX(1, 0, BG_COORD_SET);
}
pub(crate) unsafe extern "C" fn TryFreeTempTileDataBuffers() -> u32 {
    return FreeTempTileDataBuffersIfPossible() as u32;
}
pub(crate) unsafe extern "C" fn UpdateMapSecInfoWindow(state: *mut Pokenav_RegionMapGfx) {
    let mut regionMap: *mut RegionMap =
        GetSubstructPtr(POKENAV_SUBSTRUCT_REGION_MAP) as *mut RegionMap;
    match (*regionMap).mapSecType {
        2 => {
            FillWindowPixelBuffer((*state).infoWindowId as u8, 17);
            PutWindowRectTilemap((*state).infoWindowId as u8, 0, 0, 12, 2);
            AddTextPrinterParameterized(
                (*state).infoWindowId as u8,
                FONT_NARROW,
                (*regionMap).mapSecName.as_mut_ptr(),
                0,
                1,
                TEXT_SKIP_DRAW,
                None,
            );
            DrawCityMap(
                state,
                (*regionMap).mapSecId as i32,
                (*regionMap).posWithinMapSec as i32,
            );
            CopyWindowToVram((*state).infoWindowId as u8, COPYWIN_FULL);
            SetCityZoomTextInvisibility(FALSE as u32);
        }
        3 => {
            FillWindowPixelBuffer((*state).infoWindowId as u8, 17);
            PutWindowRectTilemap((*state).infoWindowId as u8, 0, 0, 12, 2);
            AddTextPrinterParameterized(
                (*state).infoWindowId as u8,
                FONT_NARROW,
                (*regionMap).mapSecName.as_mut_ptr(),
                0,
                1,
                TEXT_SKIP_DRAW,
                None,
            );
            FillBgTilemapBufferRect(1, 0x1041, 17, 6, 12, 11, 17);
            CopyWindowToVram((*state).infoWindowId as u8, COPYWIN_FULL);
            SetCityZoomTextInvisibility(TRUE as u32);
        }
        MAPSECTYPE_ROUTE | MAPSECTYPE_BATTLE_FRONTIER => {
            FillWindowPixelBuffer((*state).infoWindowId as u8, 17);
            PutWindowTilemap((*state).infoWindowId as u8);
            AddTextPrinterParameterized(
                (*state).infoWindowId as u8,
                FONT_NARROW,
                (*regionMap).mapSecName.as_mut_ptr(),
                0,
                1,
                TEXT_SKIP_DRAW,
                None,
            );
            PrintLandmarkNames(
                state,
                (*regionMap).mapSecId as i32,
                (*regionMap).posWithinMapSec as i32,
            );
            CopyWindowToVram((*state).infoWindowId as u8, COPYWIN_FULL);
            SetCityZoomTextInvisibility(TRUE as u32);
        }
        MAPSECTYPE_NONE => {
            FillBgTilemapBufferRect(1, 0x1041, 17, 4, 12, 13, 17);
            CopyBgTilemapBufferToVram(1);
            SetCityZoomTextInvisibility(TRUE as u32);
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn IsDma3ManagerBusyWithBgCopy_(
    state: *mut Pokenav_RegionMapGfx,
) -> u32 {
    return IsDma3ManagerBusyWithBgCopy() as u32;
}
pub(crate) unsafe extern "C" fn ChangeBgYForZoom(zoomIn: u32) {
    let mut taskId: u8 = CreateTask(Some(Task_ChangeBgYForZoom), 3);
    gTasks[taskId].data[0] = zoomIn as i16;
}
pub(crate) unsafe extern "C" fn IsChangeBgYForZoomActive() -> u32 {
    return FuncIsActiveTask(Some(Task_ChangeBgYForZoom)) as u32;
}
pub(crate) unsafe extern "C" fn Task_ChangeBgYForZoom(taskId: u8) {
    if gTasks[taskId].data[0] != 0 {
        if ChangeBgY(1, 0x480, BG_COORD_ADD) >= 0 {
            ChangeBgY(1, 0, BG_COORD_SET);
            DestroyTask(taskId);
        }
        UpdateCityZoomTextPosition();
    } else {
        if ChangeBgY(1, 0x480, BG_COORD_SUB) <= -24576 {
            ChangeBgY(1, -24576, BG_COORD_SET);
            DestroyTask(taskId);
        }
        UpdateCityZoomTextPosition();
    }
}
pub(crate) unsafe extern "C" fn DecompressCityMaps() {
    CreateLoopedTask(Some(LoopedTask_DecompressCityMaps), 1);
}
pub(crate) unsafe extern "C" fn IsDecompressCityMapsActive() -> u32 {
    return FuncIsActiveLoopedTask(Some(LoopedTask_DecompressCityMaps));
}
pub(crate) unsafe extern "C" fn LoopedTask_DecompressCityMaps(taskState: i32) -> u32 {
    let mut state: *mut Pokenav_RegionMapGfx =
        GetSubstructPtr(POKENAV_SUBSTRUCT_REGION_MAP_ZOOM) as *mut Pokenav_RegionMapGfx;
    if taskState < NUM_CITY_MAPS {
        LZ77UnCompWram(
            sPokenavCityMaps[taskState].tilemap,
            (*state).cityZoomPics[taskState].as_mut_ptr() as *mut c_void,
        );
        return LT_INC_AND_CONTINUE;
    }
    return LT_FINISH;
}
pub(crate) unsafe extern "C" fn DrawCityMap(
    state: *mut Pokenav_RegionMapGfx,
    mapSecId: i32,
    pos: i32,
) {
    let mut i: i32 = 0;
    i = 0;
    while i < NUM_CITY_MAPS
        && (sPokenavCityMaps[i].mapSecId as i32 != mapSecId
            || sPokenavCityMaps[i].index as i32 != pos)
    {
        i += 1;
    }
    if i == NUM_CITY_MAPS {
        return;
    }
    FillBgTilemapBufferRect_Palette0(1, 0x1041, 17, 6, 12, 11);
    CopyToBgTilemapBufferRect(
        1,
        (*state).cityZoomPics[i].as_mut_ptr() as *mut c_void,
        18,
        6,
        10,
        10,
    );
}
pub(crate) unsafe extern "C" fn PrintLandmarkNames(
    state: *mut Pokenav_RegionMapGfx,
    mapSecId: i32,
    pos: i32,
) {
    let mut i: i32 = 0;
    loop {
        let mut landmarkName: *mut u8 = GetLandmarkName(mapSecId as u8, pos as u8, i as u8);
        if landmarkName.is_null() {
            break;
        }
        StringCopyPadded(gStringVar1.as_mut_ptr(), landmarkName, CHAR_SPACE, 12);
        AddTextPrinterParameterized(
            (*state).infoWindowId as u8,
            FONT_NARROW,
            gStringVar1.as_mut_ptr(),
            0,
            i as u8 * 16 + 17,
            TEXT_SKIP_DRAW,
            None,
        );
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn CreateCityZoomTextSprites() {
    let mut i: i32 = 0;
    let mut y: i32 = 0;
    let mut sprite: *mut Sprite = null_mut();
    let mut state: *mut Pokenav_RegionMapGfx =
        GetSubstructPtr(POKENAV_SUBSTRUCT_REGION_MAP_ZOOM) as *mut Pokenav_RegionMapGfx;
    if IsRegionMapZoomed() == 0 {
        y = 228;
    } else {
        y = 132;
    }
    i = 0;
    while i < 3 {
        let mut spriteId: u8 = CreateSprite(
            (&raw const *sCityZoomTextSpriteTemplate).cast_mut(),
            152 + i as i16 * 32,
            y as i16,
            8,
        );
        sprite = &raw mut gSprites[spriteId];
        (*sprite).data[0] = 0;
        (*sprite).data[1] = i as i16 * 4;
        (*sprite).data[2] = (*sprite).oam.tileNum() as i16;
        (*sprite).data[3] = 150;
        (*sprite).data[4] = i as i16 * 4;
        (*sprite)
            .oam
            .set_tileNum((*sprite).oam.tileNum() + i as u16 * 4);
        (*state).cityZoomTextSprites[i] = sprite;
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_CityZoomText(sprite: *mut Sprite) {
    if (*sprite).data[3] != 0 {
        (*sprite).data[3] -= 1;
        return;
    }
    if ({
        (*sprite).data[0] += 1;
        (*sprite).data[0]
    }) > 11
    {
        (*sprite).data[0] = 0;
    }
    if ({
        (*sprite).data[1] += 1;
        (*sprite).data[1]
    }) > 60
    {
        (*sprite).data[1] = 0;
    }
    (*sprite)
        .oam
        .set_tileNum((*sprite).data[2] as u16 + (*sprite).data[1] as u16);
    if (*sprite).data[5] < 4 {
        if (*sprite).data[0] == 0 {
            (*sprite).data[5] += 1;
            (*sprite).data[3] = 120;
        }
    } else {
        if (*sprite).data[1] == (*sprite).data[4] {
            (*sprite).data[5] = 0;
            (*sprite).data[0] = 0;
            (*sprite).data[3] = 120;
        }
    }
}
pub(crate) unsafe extern "C" fn UpdateCityZoomTextPosition() {
    let mut i: i32 = 0;
    let mut state: *mut Pokenav_RegionMapGfx =
        GetSubstructPtr(POKENAV_SUBSTRUCT_REGION_MAP_ZOOM) as *mut Pokenav_RegionMapGfx;
    let mut y: i32 = 132 - (GetBgY(1) >> 8);
    i = 0;
    while i < 3 {
        (*(*state).cityZoomTextSprites[i]).y = y as i16;
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn SetCityZoomTextInvisibility(invisible: u32) {
    let mut i: i32 = 0;
    let mut state: *mut Pokenav_RegionMapGfx =
        GetSubstructPtr(POKENAV_SUBSTRUCT_REGION_MAP_ZOOM) as *mut Pokenav_RegionMapGfx;
    i = 0;
    while i < 3 {
        (*(*state).cityZoomTextSprites[i]).set_invisible(invisible as u16);
        i += 1;
    }
}
