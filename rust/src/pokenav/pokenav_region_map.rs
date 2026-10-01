//! Translated from `src/pokenav_region_map.c` by tools/rustport/c2rs.py.
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
    clippy::type_complexity,
    clippy::unnecessary_cast,
    unused_assignments,
    unused_variables
)]

use crate::agb_main::gMain;
use crate::bg::{
    ChangeBgX, ChangeBgY, CopyBgTilemapBufferToVram, FillBgTilemapBufferRect,
    FillBgTilemapBufferRect_Palette0, GetBgY, HideBg, IsDma3ManagerBusyWithBgCopy, SetBgMode,
    ShowBg,
};
#[allow(unused_imports)]
use crate::c::*;
#[allow(unused_imports)]
use crate::consts::*;
use crate::fieldmap::gMapHeader;
use crate::load_save::gSaveBlock2Ptr;
use crate::menu::{BgDmaFill, DecompressAndCopyTileDataToVram, FreeTempTileDataBuffersIfPossible};
use crate::palette::TransferPlttBuffer;
use crate::pokenav::{
    AllocSubstruct, FreePokenavSubstruct, GetSubstructPtr, IsLoopedTaskActive,
    SetPokenavVBlankCallback, SetVBlankCallback_,
};
use crate::pokenav_main_menu::{
    AreLeftHeaderSpritesMoving, CopyPaletteIntoBufferUnfaded, FadeToBlackExceptPrimary,
    InitBgTemplates, IsPaletteFadeActive, LoadLeftHeaderGfxForIndex, MainMenuLoopedTaskIsBusy,
    Pokenav_AllocAndLoadPalettes, PokenavFadeScreen, PrintHelpBarText,
    SetLeftHeaderSpritesInvisibility, ShowLeftHeaderGfx, SlideMenuHeaderDown,
    UpdateRegionMapRightHeaderTiles, WaitForHelpBar,
};
use crate::region_map::{
    BlendRegionMap, CreateRegionMapCursor, CreateRegionMapPlayerIcon, DoRegionMapInputCallback,
    FreeRegionMapIconResources, InitRegionMapData, IsEventIslandMapSecId, IsRegionMapZoomed,
    LoadRegionMapGfx, SetRegionMapDataForZoom, TrySetPlayerIconBlink, UpdateRegionMapVideoRegs,
    UpdateRegionMapZoom,
};
use crate::sound::PlaySE;
use crate::sprite::gSprites;
use crate::sprite::{
    FreeSpritePaletteByTag, FreeSpriteTilesByTag, LoadOam, ProcessSpriteCopyRequests,
};
use crate::string_util::gStringVar1;
use crate::task::DestroyTask;
use crate::task::{task_get, task_set};
use crate::text_window::{DrawTextBorderOuter, LoadUserWindowBorderGfx_};
#[allow(unused_imports)]
use crate::types::*;
use crate::window::{
    CopyWindowToVram, FillWindowPixelBuffer, PutWindowRectTilemap, PutWindowTilemap, RemoveWindow,
};
#[allow(unused_imports)]
use core::ffi::c_void;
#[allow(unused_imports)]
use core::mem::zeroed;
#[allow(unused_imports)]
use core::ptr::null_mut;
/// `AddWindow` with this module's view of its types.
#[inline]
unsafe fn AddWindow(a0: *mut WindowTemplate) -> u16 {
    unsafe { crate::window::AddWindow(a0 as _) }
}
/// `CopyToBgTilemapBufferRect` with this module's view of its types.
#[inline]
unsafe fn CopyToBgTilemapBufferRect(a0: u8, a1: *mut c_void, a2: u8, a3: u8, a4: u8, a5: u8) {
    unsafe {
        crate::bg::CopyToBgTilemapBufferRect(a0, a1 as _, a2, a3, a4, a5);
    }
}
/// `CreateLoopedTask` with this module's view of its types.
#[inline]
unsafe fn CreateLoopedTask(a0: Option<unsafe fn(i32) -> u32>, a1: u32) -> u32 {
    unsafe { crate::pokenav::CreateLoopedTask(a0, a1) }
}
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
/// `FuncIsActiveLoopedTask` with this module's view of its types.
#[inline]
unsafe fn FuncIsActiveLoopedTask(a0: Option<unsafe fn(i32) -> u32>) -> u32 {
    unsafe { crate::pokenav::FuncIsActiveLoopedTask(a0) }
}
/// `FuncIsActiveTask` with this module's view of its types.
#[inline]
unsafe fn FuncIsActiveTask(a0: Option<unsafe fn(u8)>) -> u8 {
    unsafe { crate::task::FuncIsActiveTask(core::mem::transmute(a0)) }
}
/// `LoadCompressedSpriteSheet` with this module's view of its types.
#[inline]
unsafe fn LoadCompressedSpriteSheet(a0: *mut CompressedSpriteSheet) -> u16 {
    unsafe { crate::decompress::LoadCompressedSpriteSheet(a0 as _) }
}
/// `SetBgTilemapBuffer` with this module's view of its types.
#[inline]
unsafe fn SetBgTilemapBuffer(a0: u8, a1: *mut c_void) {
    unsafe {
        crate::bg::SetBgTilemapBuffer(a0, a1 as _);
    }
}
/// `StringCopyPadded` with this module's view of its types.
#[inline]
unsafe fn StringCopyPadded(a0: *mut u8, a1: *mut u8, a2: u8, a3: u16) -> *mut u8 {
    unsafe { crate::string_util::StringCopyPadded(a0 as _, a1 as _, a2, a3) as *mut u8 }
}
// The C's names for task and sprite data slots.
const tZoomIn: usize = 0;
// Data tables (translate with cdata.py): sMapSecInfoWindow_Pal sRegionMapCityZoomTiles_Gfx gPokenavCityMap_Lavaridge_0 gPokenavCityMap_Fallarbor_0 gPokenavCityMap_Fortree_0 gPokenavCityMap_Slateport_0 gPokenavCityMap_Slateport_1 gPokenavCityMap_Rustboro_0 gPokenavCityMap_Rustboro_1 gPokenavCityMap_Pacifidlog_0 gPokenavCityMap_Mauville_1 gPokenavCityMap_Mauville_0 gPokenavCityMap_Oldale_0 gPokenavCityMap_Lilycove_1 gPokenavCityMap_Lilycove_0 gPokenavCityMap_Littleroot_0 gPokenavCityMap_Dewford_0 gPokenavCityMap_Sootopolis_0 gPokenavCityMap_EverGrande_0 gPokenavCityMap_EverGrande_1 gPokenavCityMap_Verdanturf_0 gPokenavCityMap_Mossdeep_1 gPokenavCityMap_Mossdeep_0 gPokenavCityMap_Petalburg_0 sRegionMapBgTemplates sRegionMapLoopTaskFuncs sCityZoomTextSpriteSheet sCityZoomTilesSpritePalette sMapSecInfoWindowTemplate sPokenavCityMaps sCityZoomTextSprite_OamData sCityZoomTextSpriteTemplate

/// `struct Pokenav_RegionMapMenu`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Pokenav_RegionMapMenu {
    pub unused: CArray<u8, 12>,
    pub zoomDisabled: u32,
    pub callback: Option<unsafe fn(*mut Pokenav_RegionMapMenu) -> u32>,
}

unsafe impl Sync for Pokenav_RegionMapMenu {}

/// `struct Pokenav_RegionMapGfx`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Pokenav_RegionMapGfx {
    pub isTaskActiveCB: Option<unsafe fn() -> u32>,
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
static sRegionMapLoopTaskFuncs: Table<CArray<Option<unsafe fn(i32) -> u32>, 5>> =
    Table((&raw const crate::data::pokenav_region_map::sRegionMapLoopTaskFuncs).cast());

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
/// `CpuSet` with this module's view of its types.
#[inline]
unsafe fn CpuSet(a0: *mut c_void, a1: *mut c_void, a2: u32) {
    unsafe {
        crate::syscall::CpuSet(a0 as _, a1 as _, a2);
    }
}
/// `GetLandmarkName` with this module's view of its types.
#[inline]
unsafe fn GetLandmarkName(a0: u8, a1: u8, a2: u8) -> *mut u8 {
    unsafe { crate::landmark::GetLandmarkName(a0, a1, a2) as *mut u8 }
}
/// `LZ77UnCompWram` with this module's view of its types.
#[inline]
unsafe fn LZ77UnCompWram(a0: *mut u32, a1: *mut c_void) {
    unsafe {
        crate::syscall::LZ77UnCompWram(a0 as _, a1 as _);
    }
}

pub unsafe fn PokenavCallback_Init_RegionMap() -> u32 {
    let state: *mut Pokenav_RegionMapMenu =
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
    TRUE as u32
}
pub unsafe fn FreeRegionMapSubstruct1() {
    (*gSaveBlock2Ptr).set_regionMapZoom(IsRegionMapZoomed() as u16);
    FreePokenavSubstruct(POKENAV_SUBSTRUCT_REGION_MAP);
    FreePokenavSubstruct(POKENAV_SUBSTRUCT_REGION_MAP_STATE);
}
pub unsafe fn GetRegionMapCallback() -> u32 {
    let state: *mut Pokenav_RegionMapMenu =
        GetSubstructPtr(POKENAV_SUBSTRUCT_REGION_MAP_STATE) as *mut Pokenav_RegionMapMenu;
    (*state).callback.unwrap_unchecked()(state)
}
pub(crate) unsafe fn HandleRegionMapInput(state: *mut Pokenav_RegionMapMenu) -> u32 {
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
    POKENAV_MAP_FUNC_NONE
}
pub(crate) unsafe fn HandleRegionMapInputZoomDisabled(state: *mut Pokenav_RegionMapMenu) -> u32 {
    if gMain.newKeys as i32 & B_BUTTON != 0 {
        (*state).callback = Some(GetExitRegionMapMenuId);
        return POKENAV_MAP_FUNC_EXIT;
    }
    POKENAV_MAP_FUNC_NONE
}
pub(crate) unsafe fn GetExitRegionMapMenuId(state: *mut Pokenav_RegionMapMenu) -> u32 {
    POKENAV_MAIN_MENU_CURSOR_ON_MAP
}
pub unsafe fn GetZoomDisabled() -> u32 {
    let state: *mut Pokenav_RegionMapMenu =
        GetSubstructPtr(POKENAV_SUBSTRUCT_REGION_MAP_STATE) as *mut Pokenav_RegionMapMenu;
    (*state).zoomDisabled
}
pub unsafe fn OpenPokenavRegionMap() -> u32 {
    let state: *mut Pokenav_RegionMapGfx =
        AllocSubstruct(POKENAV_SUBSTRUCT_REGION_MAP_ZOOM, 6472) as *mut Pokenav_RegionMapGfx;
    if state.is_null() {
        return FALSE as u32;
    }
    (*state).loopTaskId = CreateLoopedTask(Some(LoopedTask_OpenRegionMap), 1);
    (*state).isTaskActiveCB = Some(GetCurrentLoopedTaskActive);
    TRUE as u32
}
pub unsafe fn CreateRegionMapLoopedTask(index: i32) {
    let state: *mut Pokenav_RegionMapGfx =
        GetSubstructPtr(POKENAV_SUBSTRUCT_REGION_MAP_ZOOM) as *mut Pokenav_RegionMapGfx;
    (*state).loopTaskId = CreateLoopedTask(sRegionMapLoopTaskFuncs[index], 1);
    (*state).isTaskActiveCB = Some(GetCurrentLoopedTaskActive);
}
pub unsafe fn IsRegionMapLoopedTaskActive() -> u32 {
    let state: *mut Pokenav_RegionMapGfx =
        GetSubstructPtr(POKENAV_SUBSTRUCT_REGION_MAP_ZOOM) as *mut Pokenav_RegionMapGfx;
    (*state).isTaskActiveCB.unwrap_unchecked()()
}
pub unsafe fn FreeRegionMapSubstruct2() {
    let state: *mut Pokenav_RegionMapGfx =
        GetSubstructPtr(POKENAV_SUBSTRUCT_REGION_MAP_ZOOM) as *mut Pokenav_RegionMapGfx;
    FreeRegionMapIconResources();
    FreeCityZoomViewGfx();
    RemoveWindow((*state).infoWindowId as u8);
    FreePokenavSubstruct(POKENAV_SUBSTRUCT_REGION_MAP);
    FreePokenavSubstruct(POKENAV_SUBSTRUCT_REGION_MAP_ZOOM);
    SetPokenavVBlankCallback();
    SetBgMode(0);
}
pub(crate) unsafe fn VBlankCB_RegionMap() {
    TransferPlttBuffer();
    LoadOam();
    ProcessSpriteCopyRequests();
    UpdateRegionMapVideoRegs();
}
pub(crate) unsafe fn GetCurrentLoopedTaskActive() -> u32 {
    let state: *mut Pokenav_RegionMapGfx =
        GetSubstructPtr(POKENAV_SUBSTRUCT_REGION_MAP_ZOOM) as *mut Pokenav_RegionMapGfx;
    IsLoopedTaskActive((*state).loopTaskId)
}
unsafe fn ShouldOpenRegionMapZoomed() -> u8 {
    if GetZoomDisabled() != 0 {
        return FALSE;
    }
    ((*gSaveBlock2Ptr).regionMapZoom() == TRUE as u16) as u8
}
pub(crate) unsafe fn LoopedTask_OpenRegionMap(taskState: i32) -> u32 {
    let mut menuGfxId: i32 = 0;
    let mut regionMap: *mut RegionMap = null_mut();
    let state: *mut Pokenav_RegionMapGfx =
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
        0
    }
}
pub(crate) unsafe fn LoopedTask_UpdateInfoAfterCursorMove(taskState: i32) -> u32 {
    let state: *mut Pokenav_RegionMapGfx =
        GetSubstructPtr(POKENAV_SUBSTRUCT_REGION_MAP_ZOOM) as *mut Pokenav_RegionMapGfx;
    match taskState {
        0 => {
            UpdateMapSecInfoWindow(state);
            return LT_INC_AND_PAUSE;
        }
        1 if IsDma3ManagerBusyWithBgCopy_(state) != 0 => {
            return LT_PAUSE;
        }
        _ => {}
    }
    LT_FINISH
}
pub(crate) unsafe fn LoopedTask_RegionMapZoomOut(taskState: i32) -> u32 {
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
    LT_FINISH
}
pub(crate) unsafe fn LoopedTask_RegionMapZoomIn(taskState: i32) -> u32 {
    let state: *mut Pokenav_RegionMapGfx =
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
    LT_FINISH
}
pub(crate) unsafe fn LoopedTask_ExitRegionMap(taskState: i32) -> u32 {
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
    LT_FINISH
}
unsafe fn LoadCityZoomViewGfx() {
    for i in 0..1i32 {
        LoadCompressedSpriteSheet((&raw const sCityZoomTextSpriteSheet[i]).cast_mut());
    }
    Pokenav_AllocAndLoadPalettes(sCityZoomTilesSpritePalette.as_ptr().cast_mut());
    CreateCityZoomTextSprites();
}
unsafe fn FreeCityZoomViewGfx() {
    let state: *mut Pokenav_RegionMapGfx =
        GetSubstructPtr(POKENAV_SUBSTRUCT_REGION_MAP_ZOOM) as *mut Pokenav_RegionMapGfx;
    FreeSpriteTilesByTag(GFXTAG_CITY_ZOOM);
    FreeSpritePaletteByTag(PALTAG_CITY_ZOOM);
    for i in 0..3i32 {
        DestroySprite((*state).cityZoomTextSprites[i]);
    }
}
unsafe fn LoadPokenavRegionMapGfx(state: *mut Pokenav_RegionMapGfx) {
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
    CopyPaletteIntoBufferUnfaded(
        (*(&raw const crate::data::graphics::gRegionMapCityZoomTiles_Pal).cast::<CArray<u16, 0>>())
            .as_ptr()
            .cast_mut(),
        48,
        32,
    );
    if IsRegionMapZoomed() == 0 {
        ChangeBgY(1, -24576, BG_COORD_SET);
    } else {
        ChangeBgY(1, 0, BG_COORD_SET);
    }
    ChangeBgX(1, 0, BG_COORD_SET);
}
unsafe fn TryFreeTempTileDataBuffers() -> u32 {
    FreeTempTileDataBuffersIfPossible() as u32
}
unsafe fn UpdateMapSecInfoWindow(state: *mut Pokenav_RegionMapGfx) {
    let regionMap: *mut RegionMap = GetSubstructPtr(POKENAV_SUBSTRUCT_REGION_MAP) as *mut RegionMap;
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
pub(crate) unsafe fn IsDma3ManagerBusyWithBgCopy_(state: *mut Pokenav_RegionMapGfx) -> u32 {
    IsDma3ManagerBusyWithBgCopy() as u32
}
unsafe fn ChangeBgYForZoom(zoomIn: u32) {
    let taskId: u8 = CreateTask(Some(Task_ChangeBgYForZoom), 3);
    task_set(taskId, tZoomIn, zoomIn as i16);
}
unsafe fn IsChangeBgYForZoomActive() -> u32 {
    FuncIsActiveTask(Some(Task_ChangeBgYForZoom)) as u32
}
pub(crate) unsafe fn Task_ChangeBgYForZoom(taskId: u8) {
    if task_get(taskId, tZoomIn) != 0 {
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
unsafe fn DecompressCityMaps() {
    CreateLoopedTask(Some(LoopedTask_DecompressCityMaps), 1);
}
unsafe fn IsDecompressCityMapsActive() -> u32 {
    FuncIsActiveLoopedTask(Some(LoopedTask_DecompressCityMaps))
}
pub(crate) unsafe fn LoopedTask_DecompressCityMaps(taskState: i32) -> u32 {
    let state: *mut Pokenav_RegionMapGfx =
        GetSubstructPtr(POKENAV_SUBSTRUCT_REGION_MAP_ZOOM) as *mut Pokenav_RegionMapGfx;
    if taskState < NUM_CITY_MAPS {
        LZ77UnCompWram(
            sPokenavCityMaps[taskState].tilemap,
            (*state).cityZoomPics[taskState].as_mut_ptr() as *mut c_void,
        );
        return LT_INC_AND_CONTINUE;
    }
    LT_FINISH
}
unsafe fn DrawCityMap(state: *mut Pokenav_RegionMapGfx, mapSecId: i32, pos: i32) {
    let mut i: i32 = 0;
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
unsafe fn PrintLandmarkNames(state: *mut Pokenav_RegionMapGfx, mapSecId: i32, pos: i32) {
    let mut i: i32 = 0;
    loop {
        let landmarkName: *mut u8 = GetLandmarkName(mapSecId as u8, pos as u8, i as u8);
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
unsafe fn CreateCityZoomTextSprites() {
    let mut y: i32 = 0;
    let mut sprite: *mut Sprite = null_mut();
    let state: *mut Pokenav_RegionMapGfx =
        GetSubstructPtr(POKENAV_SUBSTRUCT_REGION_MAP_ZOOM) as *mut Pokenav_RegionMapGfx;
    if IsRegionMapZoomed() == 0 {
        y = 228;
    } else {
        y = 132;
    }
    for i in 0..3i32 {
        let spriteId: u8 = CreateSprite(
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
    }
}
pub(crate) unsafe fn SpriteCB_CityZoomText(sprite: *mut Sprite) {
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
unsafe fn UpdateCityZoomTextPosition() {
    let state: *mut Pokenav_RegionMapGfx =
        GetSubstructPtr(POKENAV_SUBSTRUCT_REGION_MAP_ZOOM) as *mut Pokenav_RegionMapGfx;
    let y: i32 = 132 - (GetBgY(1) >> 8);
    for i in 0..3i32 {
        (*(*state).cityZoomTextSprites[i]).y = y as i16;
    }
}
unsafe fn SetCityZoomTextInvisibility(invisible: u32) {
    let state: *mut Pokenav_RegionMapGfx =
        GetSubstructPtr(POKENAV_SUBSTRUCT_REGION_MAP_ZOOM) as *mut Pokenav_RegionMapGfx;
    for i in 0..3i32 {
        (*(*state).cityZoomTextSprites[i]).set_invisible(invisible as u16);
    }
}
