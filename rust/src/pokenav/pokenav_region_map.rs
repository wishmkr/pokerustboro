//! Translated from `src/pokenav_region_map.c` by tools/rustport/c2rs.py, then reviewed.
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

// Data tables (translate with cdata.py): sMapSecInfoWindow_Pal sRegionMapCityZoomTiles_Gfx gPokenavCityMap_Lavaridge_0 gPokenavCityMap_Fallarbor_0 gPokenavCityMap_Fortree_0 gPokenavCityMap_Slateport_0 gPokenavCityMap_Slateport_1 gPokenavCityMap_Rustboro_0 gPokenavCityMap_Rustboro_1 gPokenavCityMap_Pacifidlog_0 gPokenavCityMap_Mauville_1 gPokenavCityMap_Mauville_0 gPokenavCityMap_Oldale_0 gPokenavCityMap_Lilycove_1 gPokenavCityMap_Lilycove_0 gPokenavCityMap_Littleroot_0 gPokenavCityMap_Dewford_0 gPokenavCityMap_Sootopolis_0 gPokenavCityMap_EverGrande_0 gPokenavCityMap_EverGrande_1 gPokenavCityMap_Verdanturf_0 gPokenavCityMap_Mossdeep_1 gPokenavCityMap_Mossdeep_0 gPokenavCityMap_Petalburg_0 sRegionMapBgTemplates sRegionMapLoopTaskFuncs sCityZoomTextSpriteSheet sCityZoomTilesSpritePalette sMapSecInfoWindowTemplate sPokenavCityMaps sCityZoomTextSprite_OamData sCityZoomTextSpriteTemplate
#[allow(unused_imports)]
use crate::data::pokenav_region_map::*;

unsafe extern "C" {
    static mut gMain: u8;
    static mut gMapHeader: u8;
    static mut gRegionMapCityZoomTiles_Pal: u8;
    static mut gSaveBlock2Ptr: u8;
    static mut gSprites: u8;
    static mut gStringVar1: u8;
    static mut gTasks: u8;
    fn AddTextPrinterParameterized(
        a0: u8,
        a1: u8,
        a2: *mut u8,
        a3: u8,
        a4: u8,
        a5: u8,
        a6: Option<unsafe extern "C" fn(*mut u8, u16)>,
    ) -> u16;
    fn AddWindow(a0: *mut u8) -> u16;
    fn AllocSubstruct(a0: u32, a1: u32) -> *mut u8;
    fn AreLeftHeaderSpritesMoving() -> u32;
    fn BgDmaFill(a0: u32, a1: u8, a2: i32, a3: i32);
    fn BlendRegionMap(a0: u16, a1: u32);
    fn ChangeBgX(a0: u8, a1: i32, a2: u8) -> i32;
    fn ChangeBgY(a0: u8, a1: i32, a2: u8) -> i32;
    fn CopyBgTilemapBufferToVram(a0: u8);
    fn CopyPaletteIntoBufferUnfaded(a0: *mut u16, a1: u32, a2: u32);
    fn CopyToBgTilemapBufferRect(a0: u8, a1: *mut u8, a2: u8, a3: u8, a4: u8, a5: u8);
    fn CopyWindowToVram(a0: u8, a1: u8);
    fn CpuSet(a0: *mut u8, a1: *mut u8, a2: u32);
    fn CreateLoopedTask(a0: Option<unsafe extern "C" fn(i32) -> u32>, a1: u32) -> u32;
    fn CreateRegionMapCursor(a0: u16, a1: u16);
    fn CreateRegionMapPlayerIcon(a0: u16, a1: u16);
    fn CreateSprite(a0: *mut u8, a1: i16, a2: i16, a3: u8) -> u8;
    fn CreateTask(a0: Option<unsafe extern "C" fn(u8)>, a1: u8) -> u8;
    fn DecompressAndCopyTileDataToVram(a0: u8, a1: *mut u8, a2: u32, a3: u16, a4: u8) -> *mut u8;
    fn DestroySprite(a0: *mut u8);
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
    fn GetSubstructPtr(a0: u32) -> *mut u8;
    fn HideBg(a0: u8);
    fn InitBgTemplates(a0: *mut u8, a1: i32);
    fn InitRegionMapData(a0: *mut u8, a1: *mut u8, a2: u8);
    fn IsDma3ManagerBusyWithBgCopy() -> u8;
    fn IsEventIslandMapSecId(a0: u8) -> u32;
    fn IsLoopedTaskActive(a0: u32) -> u32;
    fn IsPaletteFadeActive() -> u32;
    fn IsRegionMapZoomed() -> u8;
    fn LZ77UnCompWram(a0: *mut u32, a1: *mut u8);
    fn LoadCompressedSpriteSheet(a0: *mut u8) -> u16;
    fn LoadLeftHeaderGfxForIndex(a0: u32);
    fn LoadOam();
    fn LoadRegionMapGfx() -> u8;
    fn LoadUserWindowBorderGfx_(a0: u8, a1: u16, a2: u8);
    fn MainMenuLoopedTaskIsBusy() -> u32;
    fn PlaySE(a0: u16);
    fn PokenavFadeScreen(a0: i32);
    fn Pokenav_AllocAndLoadPalettes(a0: *mut u8);
    fn PrintHelpBarText(a0: u32);
    fn ProcessSpriteCopyRequests();
    fn PutWindowRectTilemap(a0: u8, a1: u8, a2: u8, a3: u8, a4: u8);
    fn PutWindowTilemap(a0: u8);
    fn RemoveWindow(a0: u8);
    fn SetBgMode(a0: u8);
    fn SetBgTilemapBuffer(a0: u8, a1: *mut u8);
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
    unsafe {
        let mut state: *mut u8 = AllocSubstruct(3u32, 20u32);
        if !(!(state).is_null()) {
            return 0u32;
        }
        if !(!(AllocSubstruct(16u32, 2180u32)).is_null()) {
            return 0u32;
        }
        ((state).wrapping_add(12).cast::<u32>()).write(IsEventIslandMapSecId(
            (((&raw mut gMapHeader).cast::<u8>()).wrapping_add(20)).read(),
        ));
        if !((((state).wrapping_add(12).cast::<u32>()).read()) != 0) {
            ((state)
                .wrapping_add(16)
                .cast::<Option<unsafe extern "C" fn(*mut u8) -> u32>>())
            .write(Some(HandleRegionMapInput));
        } else {
            ((state)
                .wrapping_add(16)
                .cast::<Option<unsafe extern "C" fn(*mut u8) -> u32>>())
            .write(Some(HandleRegionMapInputZoomDisabled));
        }
        return 1u32;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FreeRegionMapSubstruct1() {
    unsafe {
        crate::c::bf_write(
            (((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(21),
            3,
            1,
            ((IsRegionMapZoomed()) as u16) as i32,
        );
        FreePokenavSubstruct(16u32);
        FreePokenavSubstruct(3u32);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetRegionMapCallback() -> u32 {
    unsafe {
        let mut state: *mut u8 = GetSubstructPtr(3u32);
        return (((state)
            .wrapping_add(16)
            .cast::<Option<unsafe extern "C" fn(*mut u8) -> u32>>())
        .read())
        .unwrap_unchecked()(state);
    }
}
pub(crate) unsafe extern "C" fn HandleRegionMapInput(state: *mut u8) -> u32 {
    unsafe {
        let mut state = state;
        'l1: {
            let __sw1 = ((DoRegionMapInputCallback()) as i32);
            if __sw1 == 3i32 {
                return 1u32;
            }
            if __sw1 == 4i32 {
                if !((IsRegionMapZoomed()) != 0) {
                    return 3u32;
                }
                return 2u32;
            }
            if __sw1 == 5i32 {
                ((state)
                    .wrapping_add(16)
                    .cast::<Option<unsafe extern "C" fn(*mut u8) -> u32>>())
                .write(Some(GetExitRegionMapMenuId));
                return 4u32;
            }
        }
        return 0u32;
    }
}
pub(crate) unsafe extern "C" fn HandleRegionMapInputZoomDisabled(state: *mut u8) -> u32 {
    unsafe {
        let mut state = state;
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(46)
            .cast::<u16>())
        .read()) as i32)
            & 2i32)
            != 0
        {
            ((state)
                .wrapping_add(16)
                .cast::<Option<unsafe extern "C" fn(*mut u8) -> u32>>())
            .write(Some(GetExitRegionMapMenuId));
            return 4u32;
        }
        return 0u32;
    }
}
pub(crate) unsafe extern "C" fn GetExitRegionMapMenuId(state: *mut u8) -> u32 {
    unsafe {
        let mut state = state;
        return 100001u32;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetZoomDisabled() -> u32 {
    unsafe {
        let mut state: *mut u8 = GetSubstructPtr(3u32);
        return ((state).wrapping_add(12).cast::<u32>()).read();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn OpenPokenavRegionMap() -> u32 {
    unsafe {
        let mut state: *mut u8 = AllocSubstruct(4u32, 6472u32);
        if !(!(state).is_null()) {
            return 0u32;
        }
        ((state).wrapping_add(4).cast::<u32>())
            .write(CreateLoopedTask(Some(LoopedTask_OpenRegionMap), 1u32));
        ((state).cast::<Option<unsafe extern "C" fn() -> u32>>())
            .write(Some(GetCurrentLoopedTaskActive));
        return 1u32;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CreateRegionMapLoopedTask(index: i32) {
    unsafe {
        let mut index = index;
        let mut state: *mut u8 = GetSubstructPtr(4u32);
        ((state).wrapping_add(4).cast::<u32>()).write(CreateLoopedTask(
            ((((&raw const sRegionMapLoopTaskFuncs)
                .cast::<u8>()
                .cast_mut()
                .cast::<Option<unsafe extern "C" fn(i32) -> u32>>())
            .cast::<Option<unsafe extern "C" fn(i32) -> u32>>())
            .wrapping_offset((index) as isize))
            .read(),
            1u32,
        ));
        ((state).cast::<Option<unsafe extern "C" fn() -> u32>>())
            .write(Some(GetCurrentLoopedTaskActive));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsRegionMapLoopedTaskActive() -> u32 {
    unsafe {
        let mut state: *mut u8 = GetSubstructPtr(4u32);
        return (((state).cast::<Option<unsafe extern "C" fn() -> u32>>()).read())
            .unwrap_unchecked()();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FreeRegionMapSubstruct2() {
    unsafe {
        let mut state: *mut u8 = GetSubstructPtr(4u32);
        FreeRegionMapIconResources();
        FreeCityZoomViewGfx();
        RemoveWindow(((((state).wrapping_add(8).cast::<u16>()).read()) as u8));
        FreePokenavSubstruct(16u32);
        FreePokenavSubstruct(4u32);
        SetPokenavVBlankCallback();
        SetBgMode(0u8);
    }
}
pub(crate) unsafe extern "C" fn VBlankCB_RegionMap() {
    unsafe {
        TransferPlttBuffer();
        LoadOam();
        ProcessSpriteCopyRequests();
        UpdateRegionMapVideoRegs();
    }
}
pub(crate) unsafe extern "C" fn GetCurrentLoopedTaskActive() -> u32 {
    unsafe {
        let mut state: *mut u8 = GetSubstructPtr(4u32);
        return IsLoopedTaskActive(((state).wrapping_add(4).cast::<u32>()).read());
    }
}
pub(crate) unsafe extern "C" fn ShouldOpenRegionMapZoomed() -> u8 {
    unsafe {
        if (GetZoomDisabled()) != 0 {
            return 0u8;
        }
        return ((((crate::c::bf_read(
            (((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(21),
            3,
            1,
            false,
        ) as u16) as i32)
            == 1i32) as u8);
    }
}
pub(crate) unsafe extern "C" fn LoopedTask_OpenRegionMap(taskState: i32) -> u32 {
    unsafe {
        let mut taskState = taskState;
        let mut menuGfxId: i32 = 0i32;
        let mut regionMap: *mut u8 = core::ptr::null_mut();
        let mut state: *mut u8 = GetSubstructPtr(4u32);
        'l1: {
            let __sw1 = taskState;
            let __matched = __sw1 == 0i32
                || __sw1 == 1i32
                || __sw1 == 2i32
                || __sw1 == 3i32
                || __sw1 == 4i32
                || __sw1 == 5i32
                || __sw1 == 6i32
                || __sw1 == 7i32;
            if __sw1 == 0i32 {
                SetVBlankCallback_(None);
                HideBg(1u8);
                HideBg(2u8);
                HideBg(3u8);
                SetBgMode(1u8);
                InitBgTemplates(
                    ((&raw const sRegionMapBgTemplates).cast::<u8>().cast_mut()).cast::<u8>(),
                    (((crate::c::div_u32(12u32, 4u32)).wrapping_sub(1u32)) as i32),
                );
                regionMap = GetSubstructPtr(16u32);
                InitRegionMapData(
                    regionMap,
                    (((&raw const sRegionMapBgTemplates).cast::<u8>().cast_mut()).cast::<u8>())
                        .wrapping_offset(4),
                    ShouldOpenRegionMapZoomed(),
                );
                LoadCityZoomViewGfx();
                return 0u32;
            }
            if __sw1 == 1i32 {
                if (LoadRegionMapGfx()) != 0 {
                    return 2u32;
                }
                if !((GetZoomDisabled()) != 0) {
                    CreateRegionMapPlayerIcon(4u16, 9u16);
                    CreateRegionMapCursor(5u16, 10u16);
                    TrySetPlayerIconBlink();
                } else {
                    BlendRegionMap(0u16, 6u32);
                }
                return 0u32;
            }
            if __sw1 == 2i32 {
                DecompressCityMaps();
                return 1u32;
            }
            if __sw1 == 3i32 {
                if (IsDecompressCityMapsActive()) != 0 {
                    return 2u32;
                }
                LoadPokenavRegionMapGfx(state);
                return 1u32;
            }
            if __sw1 == 4i32 {
                if (TryFreeTempTileDataBuffers()) != 0 {
                    return 2u32;
                }
                UpdateMapSecInfoWindow(state);
                FadeToBlackExceptPrimary();
                return 0u32;
            }
            if __sw1 == 5i32 {
                if (IsDma3ManagerBusyWithBgCopy_(state)) != 0 {
                    return 2u32;
                }
                ShowBg(1u8);
                ShowBg(2u8);
                SetVBlankCallback_(Some(VBlankCB_RegionMap));
                return 0u32;
            }
            if __sw1 == 6i32 {
                if !((ShouldOpenRegionMapZoomed()) != 0) {
                    menuGfxId = 4i32;
                } else {
                    menuGfxId = 5i32;
                }
                LoadLeftHeaderGfxForIndex(((menuGfxId) as u32));
                ShowLeftHeaderGfx(((menuGfxId) as u32), 1u32, 1u32);
                PokenavFadeScreen(1i32);
                return 0u32;
            }
            if __sw1 == 7i32 {
                if ((IsPaletteFadeActive()) != 0) || ((AreLeftHeaderSpritesMoving()) != 0) {
                    return 2u32;
                }
                return 1u32;
            }
            if !__matched {
                return 4u32;
            }
        }
        #[allow(unreachable_code)]
        {
            return 0u32;
        }
    }
}
pub(crate) unsafe extern "C" fn LoopedTask_UpdateInfoAfterCursorMove(taskState: i32) -> u32 {
    unsafe {
        let mut taskState = taskState;
        let mut state: *mut u8 = GetSubstructPtr(4u32);
        'l1: {
            let __sw1 = taskState;
            if __sw1 == 0i32 {
                UpdateMapSecInfoWindow(state);
                return 0u32;
            }
            if __sw1 == 1i32 {
                if (IsDma3ManagerBusyWithBgCopy_(state)) != 0 {
                    return 2u32;
                }
                break 'l1;
            }
        }
        return 4u32;
    }
}
pub(crate) unsafe extern "C" fn LoopedTask_RegionMapZoomOut(taskState: i32) -> u32 {
    unsafe {
        let mut taskState = taskState;
        'l1: {
            let __sw1 = taskState;
            if __sw1 == 0i32 {
                PlaySE(5u16);
                ChangeBgYForZoom(0u32);
                SetRegionMapDataForZoom();
                return 0u32;
            }
            if __sw1 == 1i32 {
                if ((UpdateRegionMapZoom()) != 0) || ((IsChangeBgYForZoomActive()) != 0) {
                    return 2u32;
                }
                PrintHelpBarText(1u32);
                return 0u32;
            }
            if __sw1 == 2i32 {
                if (WaitForHelpBar()) != 0 {
                    return 2u32;
                }
                UpdateRegionMapRightHeaderTiles(4u32);
                break 'l1;
            }
        }
        return 4u32;
    }
}
pub(crate) unsafe extern "C" fn LoopedTask_RegionMapZoomIn(taskState: i32) -> u32 {
    unsafe {
        let mut taskState = taskState;
        let mut state: *mut u8 = GetSubstructPtr(4u32);
        'l1: {
            let __sw1 = taskState;
            if __sw1 == 0i32 {
                PlaySE(5u16);
                UpdateMapSecInfoWindow(state);
                return 0u32;
            }
            if __sw1 == 1i32 {
                if (IsDma3ManagerBusyWithBgCopy_(state)) != 0 {
                    return 2u32;
                }
                ChangeBgYForZoom(1u32);
                SetRegionMapDataForZoom();
                return 0u32;
            }
            if __sw1 == 2i32 {
                if ((UpdateRegionMapZoom()) != 0) || ((IsChangeBgYForZoomActive()) != 0) {
                    return 2u32;
                }
                PrintHelpBarText(2u32);
                return 0u32;
            }
            if __sw1 == 3i32 {
                if (WaitForHelpBar()) != 0 {
                    return 2u32;
                }
                UpdateRegionMapRightHeaderTiles(5u32);
                break 'l1;
            }
        }
        return 4u32;
    }
}
pub(crate) unsafe extern "C" fn LoopedTask_ExitRegionMap(taskState: i32) -> u32 {
    unsafe {
        let mut taskState = taskState;
        'l1: {
            let __sw1 = taskState;
            if __sw1 == 0i32 {
                PlaySE(5u16);
                PokenavFadeScreen(0i32);
                return 0u32;
            }
            if __sw1 == 1i32 {
                if (IsPaletteFadeActive()) != 0 {
                    return 2u32;
                }
                SetLeftHeaderSpritesInvisibility();
                SlideMenuHeaderDown();
                return 0u32;
            }
            if __sw1 == 2i32 {
                if (MainMenuLoopedTaskIsBusy()) != 0 {
                    return 2u32;
                }
                HideBg(1u8);
                HideBg(2u8);
                HideBg(3u8);
                return 0u32;
            }
        }
        return 4u32;
    }
}
pub(crate) unsafe extern "C" fn LoadCityZoomViewGfx() {
    unsafe {
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(((i) as u32) < crate::c::div_u32(8u32, 8u32)) {
                    break 'l1;
                }
                'l2: {
                    LoadCompressedSpriteSheet(
                        (((&raw const sCityZoomTextSpriteSheet)
                            .cast::<u8>()
                            .cast_mut())
                        .cast::<u8>())
                        .wrapping_offset((i) as isize * 8),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        Pokenav_AllocAndLoadPalettes(
            ((&raw const sCityZoomTilesSpritePalette)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>(),
        );
        CreateCityZoomTextSprites();
    }
}
pub(crate) unsafe extern "C" fn FreeCityZoomViewGfx() {
    unsafe {
        let mut i: i32 = 0i32;
        let mut state: *mut u8 = GetSubstructPtr(4u32);
        FreeSpriteTilesByTag(6u16);
        FreeSpritePaletteByTag(11u16);
        {
            i = 0i32;
            'l1: loop {
                if !(i < ((crate::c::div_u32(12u32, 4u32)) as i32)) {
                    break 'l1;
                }
                'l2: {
                    DestroySprite(
                        ((((state).wrapping_add(12)).cast::<*mut u8>())
                            .wrapping_offset((i) as isize))
                        .read(),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn LoadPokenavRegionMapGfx(state: *mut u8) {
    unsafe {
        let mut state = state;
        BgDmaFill(1u32, 0u8, 64i32, 1i32);
        BgDmaFill(1u32, 17u8, 65i32, 1i32);
        'l1: loop {
            'l2: {
                {
                    let mut tmp: u16 = 0u16;
                    (&raw mut tmp).write_volatile(4160u16);
                    'l3: loop {
                        'l4: {
                            CpuSet(
                                (&raw mut tmp).cast::<u8>(),
                                ((state).wrapping_add(24)).cast::<u8>(),
                                ((16777216i32
                                    | (crate::c::div_i32(2048i32, crate::c::div_i32(16i32, 8i32))
                                        & 2097151i32)) as u32),
                            );
                        }
                        if !((0i32) != 0) {
                            break 'l3;
                        }
                    }
                }
            }
            if !((0i32) != 0) {
                break 'l1;
            }
        }
        SetBgTilemapBuffer(1u8, ((state).wrapping_add(24)).cast::<u8>());
        ((state).wrapping_add(8).cast::<u16>()).write(AddWindow(
            (&raw const sMapSecInfoWindowTemplate)
                .cast::<u8>()
                .cast_mut(),
        ));
        LoadUserWindowBorderGfx_(
            ((((state).wrapping_add(8).cast::<u16>()).read()) as u8),
            66u16,
            64u8,
        );
        DrawTextBorderOuter(
            ((((state).wrapping_add(8).cast::<u16>()).read()) as u8),
            66u16,
            4u8,
        );
        DecompressAndCopyTileDataToVram(
            1u8,
            (((&raw const sRegionMapCityZoomTiles_Gfx)
                .cast::<u8>()
                .cast_mut()
                .cast::<u32>())
            .cast::<u32>())
            .cast::<u8>(),
            0u32,
            0u16,
            0u8,
        );
        FillWindowPixelBuffer(
            ((((state).wrapping_add(8).cast::<u16>()).read()) as u8),
            17u8,
        );
        PutWindowTilemap(((((state).wrapping_add(8).cast::<u16>()).read()) as u8));
        CopyWindowToVram(
            ((((state).wrapping_add(8).cast::<u16>()).read()) as u8),
            3u8,
        );
        CopyPaletteIntoBufferUnfaded(
            ((&raw const sMapSecInfoWindow_Pal)
                .cast::<u8>()
                .cast_mut()
                .cast::<u16>())
            .cast::<u16>(),
            16u32,
            32u32,
        );
        CopyPaletteIntoBufferUnfaded(
            ((&raw mut gRegionMapCityZoomTiles_Pal).cast::<u16>()).cast::<u16>(),
            48u32,
            32u32,
        );
        if !((IsRegionMapZoomed()) != 0) {
            ChangeBgY(1u8, (-24576i32), 0u8);
        } else {
            ChangeBgY(1u8, 0i32, 0u8);
        }
        ChangeBgX(1u8, 0i32, 0u8);
    }
}
pub(crate) unsafe extern "C" fn TryFreeTempTileDataBuffers() -> u32 {
    unsafe {
        return ((FreeTempTileDataBuffersIfPossible()) as u32);
    }
}
pub(crate) unsafe extern "C" fn UpdateMapSecInfoWindow(state: *mut u8) {
    unsafe {
        let mut state = state;
        let mut regionMap: *mut u8 = GetSubstructPtr(16u32);
        'l1: {
            let __sw1 = ((((regionMap).wrapping_add(2)).read()) as i32);
            if __sw1 == 2i32 {
                FillWindowPixelBuffer(
                    ((((state).wrapping_add(8).cast::<u16>()).read()) as u8),
                    17u8,
                );
                PutWindowRectTilemap(
                    ((((state).wrapping_add(8).cast::<u16>()).read()) as u8),
                    0u8,
                    0u8,
                    12u8,
                    2u8,
                );
                AddTextPrinterParameterized(
                    ((((state).wrapping_add(8).cast::<u16>()).read()) as u8),
                    7u8,
                    ((regionMap).wrapping_add(4)).cast::<u8>(),
                    0u8,
                    1u8,
                    255u8,
                    None,
                );
                DrawCityMap(
                    state,
                    ((((regionMap).cast::<u16>()).read()) as i32),
                    ((((regionMap).wrapping_add(3)).read()) as i32),
                );
                CopyWindowToVram(
                    ((((state).wrapping_add(8).cast::<u16>()).read()) as u8),
                    3u8,
                );
                SetCityZoomTextInvisibility(0u32);
                break 'l1;
            }
            if __sw1 == 3i32 {
                FillWindowPixelBuffer(
                    ((((state).wrapping_add(8).cast::<u16>()).read()) as u8),
                    17u8,
                );
                PutWindowRectTilemap(
                    ((((state).wrapping_add(8).cast::<u16>()).read()) as u8),
                    0u8,
                    0u8,
                    12u8,
                    2u8,
                );
                AddTextPrinterParameterized(
                    ((((state).wrapping_add(8).cast::<u16>()).read()) as u8),
                    7u8,
                    ((regionMap).wrapping_add(4)).cast::<u8>(),
                    0u8,
                    1u8,
                    255u8,
                    None,
                );
                FillBgTilemapBufferRect(1u8, 4161u16, 17u8, 6u8, 12u8, 11u8, 17u8);
                CopyWindowToVram(
                    ((((state).wrapping_add(8).cast::<u16>()).read()) as u8),
                    3u8,
                );
                SetCityZoomTextInvisibility(1u32);
                break 'l1;
            }
            if __sw1 == 1i32 || __sw1 == 4i32 {
                FillWindowPixelBuffer(
                    ((((state).wrapping_add(8).cast::<u16>()).read()) as u8),
                    17u8,
                );
                PutWindowTilemap(((((state).wrapping_add(8).cast::<u16>()).read()) as u8));
                AddTextPrinterParameterized(
                    ((((state).wrapping_add(8).cast::<u16>()).read()) as u8),
                    7u8,
                    ((regionMap).wrapping_add(4)).cast::<u8>(),
                    0u8,
                    1u8,
                    255u8,
                    None,
                );
                PrintLandmarkNames(
                    state,
                    ((((regionMap).cast::<u16>()).read()) as i32),
                    ((((regionMap).wrapping_add(3)).read()) as i32),
                );
                CopyWindowToVram(
                    ((((state).wrapping_add(8).cast::<u16>()).read()) as u8),
                    3u8,
                );
                SetCityZoomTextInvisibility(1u32);
                break 'l1;
            }
            if __sw1 == 0i32 {
                FillBgTilemapBufferRect(1u8, 4161u16, 17u8, 4u8, 12u8, 13u8, 17u8);
                CopyBgTilemapBufferToVram(1u8);
                SetCityZoomTextInvisibility(1u32);
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn IsDma3ManagerBusyWithBgCopy_(state: *mut u8) -> u32 {
    unsafe {
        let mut state = state;
        return ((IsDma3ManagerBusyWithBgCopy()) as u32);
    }
}
pub(crate) unsafe extern "C" fn ChangeBgYForZoom(zoomIn: u32) {
    unsafe {
        let mut zoomIn = zoomIn;
        let mut taskId: u8 = CreateTask(Some(Task_ChangeBgYForZoom), 3u8);
        (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .write(((zoomIn) as i16));
    }
}
pub(crate) unsafe extern "C" fn IsChangeBgYForZoomActive() -> u32 {
    unsafe {
        return ((FuncIsActiveTask(Some(Task_ChangeBgYForZoom))) as u32);
    }
}
pub(crate) unsafe extern "C" fn Task_ChangeBgYForZoom(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .read())
            != 0
        {
            if ChangeBgY(1u8, 1152i32, 1u8) >= 0i32 {
                ChangeBgY(1u8, 0i32, 0u8);
                DestroyTask(taskId);
            }
            UpdateCityZoomTextPosition();
        } else {
            if ChangeBgY(1u8, 1152i32, 2u8) <= (-24576i32) {
                ChangeBgY(1u8, (-24576i32), 0u8);
                DestroyTask(taskId);
            }
            UpdateCityZoomTextPosition();
        }
    }
}
pub(crate) unsafe extern "C" fn DecompressCityMaps() {
    unsafe {
        CreateLoopedTask(Some(LoopedTask_DecompressCityMaps), 1u32);
    }
}
pub(crate) unsafe extern "C" fn IsDecompressCityMapsActive() -> u32 {
    unsafe {
        return FuncIsActiveLoopedTask(Some(LoopedTask_DecompressCityMaps));
    }
}
pub(crate) unsafe extern "C" fn LoopedTask_DecompressCityMaps(taskState: i32) -> u32 {
    unsafe {
        let mut taskState = taskState;
        let mut state: *mut u8 = GetSubstructPtr(4u32);
        if taskState < 22i32 {
            LZ77UnCompWram(
                (((((&raw const sPokenavCityMaps).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset((taskState) as isize * 8))
                .wrapping_add(4)
                .cast::<*mut u32>())
                .read(),
                ((((state).wrapping_add(2072)).cast::<u8>())
                    .wrapping_offset((taskState) as isize * 200))
                .cast::<u8>(),
            );
            return 1u32;
        }
        return 4u32;
    }
}
pub(crate) unsafe extern "C" fn DrawCityMap(state: *mut u8, mapSecId: i32, pos: i32) {
    unsafe {
        let mut state = state;
        let mut mapSecId = mapSecId;
        let mut pos = pos;
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !((i < 22i32)
                    && (((((((((&raw const sPokenavCityMaps).cast::<u8>().cast_mut())
                        .cast::<u8>())
                    .wrapping_offset((i) as isize * 8))
                    .cast::<u16>())
                    .read()) as i32)
                        != mapSecId)
                        || ((((((((&raw const sPokenavCityMaps).cast::<u8>().cast_mut())
                            .cast::<u8>())
                        .wrapping_offset((i) as isize * 8))
                        .wrapping_add(2)
                        .cast::<u16>())
                        .read()) as i32)
                            != pos)))
                {
                    break 'l1;
                }
                'l2: {}
                i = (i).wrapping_add(1);
            }
        }
        if i == 22i32 {
            return;
        }
        FillBgTilemapBufferRect_Palette0(1u8, 4161u16, 17u8, 6u8, 12u8, 11u8);
        CopyToBgTilemapBufferRect(
            1u8,
            ((((state).wrapping_add(2072)).cast::<u8>()).wrapping_offset((i) as isize * 200))
                .cast::<u8>(),
            18u8,
            6u8,
            10u8,
            10u8,
        );
    }
}
pub(crate) unsafe extern "C" fn PrintLandmarkNames(state: *mut u8, mapSecId: i32, pos: i32) {
    unsafe {
        let mut state = state;
        let mut mapSecId = mapSecId;
        let mut pos = pos;
        let mut i: i32 = 0i32;
        'l1: loop {
            if !((1i32) != 0) {
                break 'l1;
            }
            let mut landmarkName: *mut u8 =
                GetLandmarkName(((mapSecId) as u8), ((pos) as u8), ((i) as u8));
            if !(!(landmarkName).is_null()) {
                break 'l1;
            }
            StringCopyPadded(
                (&raw mut gStringVar1).cast::<u8>(),
                landmarkName,
                0u8,
                12u16,
            );
            AddTextPrinterParameterized(
                ((((state).wrapping_add(8).cast::<u16>()).read()) as u8),
                7u8,
                (&raw mut gStringVar1).cast::<u8>(),
                0u8,
                ((((i).wrapping_mul(16i32)).wrapping_add(17i32)) as u8),
                255u8,
                None,
            );
            i = (i).wrapping_add(1);
        }
    }
}
pub(crate) unsafe extern "C" fn CreateCityZoomTextSprites() {
    unsafe {
        let mut i: i32 = 0i32;
        let mut y: i32 = 0i32;
        let mut sprite: *mut u8 = core::ptr::null_mut();
        let mut state: *mut u8 = GetSubstructPtr(4u32);
        if !((IsRegionMapZoomed()) != 0) {
            y = 228i32;
        } else {
            y = 132i32;
        }
        {
            i = 0i32;
            'l1: loop {
                if !(i < ((crate::c::div_u32(12u32, 4u32)) as i32)) {
                    break 'l1;
                }
                'l2: {
                    let mut spriteId: u8 = CreateSprite(
                        (&raw const sCityZoomTextSpriteTemplate)
                            .cast::<u8>()
                            .cast_mut(),
                        (((152i32).wrapping_add((i).wrapping_mul(32i32))) as i16),
                        ((y) as i16),
                        8u8,
                    );
                    sprite = ((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68);
                    (((sprite).wrapping_add(46)).cast::<i16>()).write(0i16);
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1))
                        .write((((i).wrapping_mul(4i32)) as i16));
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(
                        ((crate::c::bf_read((sprite).wrapping_add(4), 0, 10, false) as u16) as i16),
                    );
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(150i16);
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4))
                        .write((((i).wrapping_mul(4i32)) as i16));
                    crate::c::bf_write(
                        (sprite).wrapping_add(4),
                        0,
                        10,
                        ((((crate::c::bf_read((sprite).wrapping_add(4), 0, 10, false) as u16)
                            as i32)
                            .wrapping_add((i).wrapping_mul(4i32))) as u16)
                            as i32,
                    );
                    ((((state).wrapping_add(12)).cast::<*mut u8>()).wrapping_offset((i) as isize))
                        .write(sprite);
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_CityZoomText(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read()) != 0 {
            let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3);
            (__p1).write(((__p1).read()).wrapping_sub(1));
            return;
        }
        if (({
            let __p2 = ((sprite).wrapping_add(46)).cast::<i16>();
            let __t3 = ((__p2).read()).wrapping_add(1);
            (__p2).write(__t3);
            __t3
        }) as i32)
            > 11i32
        {
            (((sprite).wrapping_add(46)).cast::<i16>()).write(0i16);
        }
        if (({
            let __p4 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
            let __t5 = ((__p4).read()).wrapping_add(1);
            (__p4).write(__t5);
            __t5
        }) as i32)
            > 60i32
        {
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(0i16);
        }
        crate::c::bf_write(
            (sprite).wrapping_add(4),
            0,
            10,
            ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
                .wrapping_add(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                        as i32),
                )) as u16) as i32,
        );
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read()) as i32) < 4i32
        {
            if (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) == 0i32 {
                let __p6 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5);
                (__p6).write(((__p6).read()).wrapping_add(1));
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(120i16);
            }
        } else {
            if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
                == ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read())
                    as i32)
            {
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write(0i16);
                (((sprite).wrapping_add(46)).cast::<i16>()).write(0i16);
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(120i16);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn UpdateCityZoomTextPosition() {
    unsafe {
        let mut i: i32 = 0i32;
        let mut state: *mut u8 = GetSubstructPtr(4u32);
        let mut y: i32 = (132i32).wrapping_sub((GetBgY(1u8) >> 8));
        {
            i = 0i32;
            'l1: loop {
                if !(i < ((crate::c::div_u32(12u32, 4u32)) as i32)) {
                    break 'l1;
                }
                'l2: {
                    ((((((state).wrapping_add(12)).cast::<*mut u8>())
                        .wrapping_offset((i) as isize))
                    .read())
                    .wrapping_add(34)
                    .cast::<i16>())
                    .write(((y) as i16));
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SetCityZoomTextInvisibility(invisible: u32) {
    unsafe {
        let mut invisible = invisible;
        let mut i: i32 = 0i32;
        let mut state: *mut u8 = GetSubstructPtr(4u32);
        {
            i = 0i32;
            'l1: loop {
                if !(i < ((crate::c::div_u32(12u32, 4u32)) as i32)) {
                    break 'l1;
                }
                'l2: {
                    crate::c::bf_write(
                        (((((state).wrapping_add(12)).cast::<*mut u8>())
                            .wrapping_offset((i) as isize))
                        .read())
                        .wrapping_add(62),
                        2,
                        1,
                        ((invisible) as u16) as i32,
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
