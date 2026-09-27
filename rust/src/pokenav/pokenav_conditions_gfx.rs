//! Translated from `src/pokenav_conditions_gfx.c` by tools/rustport/c2rs.py, then reviewed.
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

// Data tables (translate with cdata.py): gConditionGraphData_Pal gConditionText_Pal sConditionGraphData_Gfx sConditionGraphData_Tilemap sMonMarkings_Pal sMenuBgTemplates sMonNameGenderWindowTemplate sListIndexWindowTemplate sUnusedWindowTemplate1 sUnusedWindowTemplate2 sLoopedTaskFuncs
#[allow(unused_imports)]
use crate::data::pokenav_conditions_gfx::*;

pub(crate) static mut sInitialLoadId: u8 = 0u8;

unsafe extern "C" {
    static mut gPokenavCondition_Gfx: u8;
    static mut gPokenavCondition_Pal: u8;
    static mut gPokenavCondition_Tilemap: u8;
    static mut gPokenavOptions_Tilemap: u8;
    static mut gSprites: u8;
    static mut gText_Number2: u8;
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
    fn BufferMonMarkingsMenuTiles();
    fn ChangeBgX(a0: u8, a1: i32, a2: u8) -> i32;
    fn ChangeBgY(a0: u8, a1: i32, a2: u8) -> i32;
    fn ConditionGraph_Draw(a0: *mut u8);
    fn ConditionGraph_InitResetScanline(a0: *mut u8);
    fn ConditionGraph_InitWindow(a0: u8);
    fn ConditionGraph_ResetScanline(a0: *mut u8) -> u8;
    fn ConditionGraph_SetNewPositions(a0: *mut u8, a1: *mut u8, a2: *mut u8);
    fn ConditionGraph_TryUpdate(a0: *mut u8) -> u8;
    fn ConditionMenu_UpdateMonEnter(a0: *mut u8, a1: *mut i16) -> u8;
    fn ConditionMenu_UpdateMonExit(a0: *mut u8, a1: *mut i16) -> u8;
    fn ConvertIntToDecimalStringN(a0: *mut u8, a1: i32, a2: i32, a3: u8) -> *mut u8;
    fn CopyBgTilemapBufferToVram(a0: u8);
    fn CopyPaletteIntoBufferUnfaded(a0: *mut u16, a1: u32, a2: u32);
    fn CopyToBgTilemapBufferRect(a0: u8, a1: *mut u8, a2: u8, a3: u8, a4: u8, a5: u8);
    fn CopyWindowToVram(a0: u8, a1: u8);
    fn CpuSet(a0: *mut u8, a1: *mut u8, a2: u32);
    fn CreateConditionSparkleSprites(a0: *mut *mut u8, a1: u8, a2: u8);
    fn CreateLoopedTask(a0: Option<unsafe extern "C" fn(i32) -> u32>, a1: u32) -> u32;
    fn CreateMonMarkingAllCombosSprite(a0: u16, a1: u16, a2: *mut u16) -> *mut u8;
    fn CreateSprite(a0: *mut u8, a1: i16, a2: i16, a3: u8) -> u8;
    fn DeactivateAllTextPrinters();
    fn DecompressAndCopyTileDataToVram(a0: u8, a1: *mut u8, a2: u32, a3: u16, a4: u8) -> *mut u8;
    fn DestroyConditionSparkleSprites(a0: *mut *mut u8);
    fn DestroySprite(a0: *mut u8);
    fn FillWindowPixelBuffer(a0: u8, a1: u8);
    fn FreeConditionSparkles(a0: *mut *mut u8);
    fn FreeMonMarkingsMenu();
    fn FreePokenavSubstruct(a0: u32);
    fn FreeSpritePaletteByTag(a0: u16);
    fn FreeSpriteTilesByTag(a0: u16);
    fn FreeTempTileDataBuffersIfPossible() -> u8;
    fn GetConditionGraphCurrentListIndex() -> u16;
    fn GetConditionGraphMenuCurrentLoadIndex() -> i8;
    fn GetConditionGraphPtr() -> *mut u8;
    fn GetConditionMonDataBuffer() -> u16;
    fn GetConditionMonLocationText(a0: u8) -> *mut u8;
    fn GetConditionMonNameText(a0: u8) -> *mut u8;
    fn GetConditionMonPal(a0: u8) -> *mut u8;
    fn GetConditionMonPicGfx(a0: u8) -> *mut u8;
    fn GetMonListCount() -> u16;
    fn GetNumConditionMonSparkles() -> u8;
    fn GetSubstructPtr(a0: u32) -> *mut u8;
    fn HideBg(a0: u8);
    fn IndexOfSpritePaletteTag(a0: u16) -> u8;
    fn InitBgTemplates(a0: *mut u8, a1: i32);
    fn InitMonMarkingsMenu(a0: *mut u8);
    fn IsConditionMenuSearchMode() -> u32;
    fn IsLoopedTaskActive(a0: u32) -> u32;
    fn IsPaletteFadeActive() -> u32;
    fn LZ77UnCompVram(a0: *mut u32, a1: *mut u8);
    fn LoadConditionGraphMenuGfx() -> u32;
    fn LoadConditionMonPicTemplate(a0: *mut u8, a1: *mut u8, a2: *mut u8);
    fn LoadConditionSelectionIcons(a0: *mut u8, a1: *mut u8, a2: *mut u8);
    fn LoadConditionSparkle(a0: *mut u8, a1: *mut u8);
    fn LoadLeftHeaderGfxForIndex(a0: u32);
    fn LoadNextConditionMenuMonData(a0: u8) -> u32;
    fn LoadOam();
    fn LoadPalette(a0: *mut u8, a1: u16, a2: u16);
    fn LoadSpritePalette(a0: *mut u8) -> u8;
    fn LoadSpriteSheet(a0: *mut u8) -> u16;
    fn LoadSpriteSheets(a0: *mut u8);
    fn MainMenuLoopedTaskIsBusy() -> u32;
    fn MoveConditionMonOffscreen(a0: *mut i16) -> u8;
    fn OpenMonMarkingsMenu(a0: u8, a1: i16, a2: i16);
    fn PokenavFadeScreen(a0: i32);
    fn PokenavFillPalette(a0: u32, a1: u16);
    fn Pokenav_AllocAndLoadPalettes(a0: *mut u8);
    fn PrintHelpBarText(a0: u32);
    fn ProcessSpriteCopyRequests();
    fn PutWindowTilemap(a0: u8);
    fn RemoveWindow(a0: u8);
    fn ResetConditionSparkleSprites(a0: *mut *mut u8);
    fn ScanlineEffect_InitHBlankDmaTransfer();
    fn SetBgTilemapBuffer(a0: u8, a1: *mut u8);
    fn SetGpuReg(a0: u8, a1: u16);
    fn SetLeftHeaderSpritesInvisibility();
    fn SetPokenavVBlankCallback();
    fn SetVBlankCallback_(a0: Option<unsafe extern "C" fn()>);
    fn ShowBg(a0: u8);
    fn ShowLeftHeaderGfx(a0: u32, a1: u32, a2: u32);
    fn SlideMenuHeaderDown();
    fn SpriteCallbackDummy(a0: *mut u8);
    fn StartSpriteAnim(a0: *mut u8, a1: u8);
    fn StringCopy(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn TransferPlttBuffer();
    fn TryGetMonMarkId() -> u8;
    fn WaitForHelpBar() -> u32;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn OpenConditionGraphMenu() -> u32 {
    unsafe {
        let mut menu: *mut u8 = AllocSubstruct(12u32, 14508u32);
        if ((menu) as usize) == 0usize {
            return 0u32;
        }
        ((menu).wrapping_add(6166)).write(255u8);
        ((menu).cast::<u32>()).write(CreateLoopedTask(
            Some(LoopedTask_OpenConditionGraphMenu),
            1u32,
        ));
        ((menu)
            .wrapping_add(6160)
            .cast::<Option<unsafe extern "C" fn() -> u32>>())
        .write(Some(GetConditionGraphMenuLoopedTaskActive));
        ((menu).wrapping_add(10504)).write(0u8);
        return 1u32;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CreateConditionGraphMenuLoopedTask(id: i32) {
    unsafe {
        let mut id = id;
        let mut menu: *mut u8 = GetSubstructPtr(12u32);
        ((menu).cast::<u32>()).write(CreateLoopedTask(
            ((((&raw const sLoopedTaskFuncs)
                .cast::<u8>()
                .cast_mut()
                .cast::<Option<unsafe extern "C" fn(i32) -> u32>>())
            .cast::<Option<unsafe extern "C" fn(i32) -> u32>>())
            .wrapping_offset((id) as isize))
            .read(),
            1u32,
        ));
        ((menu)
            .wrapping_add(6160)
            .cast::<Option<unsafe extern "C" fn() -> u32>>())
        .write(Some(GetConditionGraphMenuLoopedTaskActive));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsConditionGraphMenuLoopedTaskActive() -> u32 {
    unsafe {
        let mut menu: *mut u8 = GetSubstructPtr(12u32);
        return (((menu)
            .wrapping_add(6160)
            .cast::<Option<unsafe extern "C" fn() -> u32>>())
        .read())
        .unwrap_unchecked()();
    }
}
pub(crate) unsafe extern "C" fn GetConditionGraphMenuLoopedTaskActive() -> u32 {
    unsafe {
        let mut menu: *mut u8 = GetSubstructPtr(12u32);
        return IsLoopedTaskActive(((menu).cast::<u32>()).read());
    }
}
pub(crate) unsafe extern "C" fn LoopedTask_OpenConditionGraphMenu(state: i32) -> u32 {
    unsafe {
        let mut state = state;
        let mut menu: *mut u8 = GetSubstructPtr(12u32);
        'l1: {
            let __sw1 = state;
            if __sw1 == 0i32 {
                if LoadConditionGraphMenuGfx() != 1u32 {
                    return 2u32;
                }
                return 0u32;
            }
            if __sw1 == 1i32 {
                InitBgTemplates(
                    ((&raw const sMenuBgTemplates).cast::<u8>().cast_mut()).cast::<u8>(),
                    ((crate::c::div_u32(12u32, 4u32)) as i32),
                );
                ChangeBgX(1u8, 0i32, 0u8);
                ChangeBgY(1u8, 0i32, 0u8);
                ChangeBgX(2u8, 0i32, 0u8);
                ChangeBgY(2u8, 0i32, 0u8);
                ChangeBgX(3u8, 0i32, 0u8);
                ChangeBgY(3u8, 0i32, 0u8);
                SetGpuReg(0u8, 31040u16);
                SetGpuReg(80u8, 2116u16);
                SetGpuReg(82u8, 1035u16);
                DecompressAndCopyTileDataToVram(
                    3u8,
                    (((&raw mut gPokenavCondition_Gfx).cast::<u32>()).cast::<u32>()).cast::<u8>(),
                    0u32,
                    0u16,
                    0u8,
                );
                return 0u32;
            }
            if __sw1 == 2i32 {
                if (FreeTempTileDataBuffersIfPossible()) != 0 {
                    return 2u32;
                }
                DecompressAndCopyTileDataToVram(
                    2u8,
                    (((&raw const sConditionGraphData_Gfx)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<u32>())
                    .cast::<u32>())
                    .cast::<u8>(),
                    0u32,
                    0u16,
                    0u8,
                );
                return 0u32;
            }
            if __sw1 == 3i32 {
                if (FreeTempTileDataBuffersIfPossible()) != 0 {
                    return 2u32;
                }
                LZ77UnCompVram(
                    ((&raw mut gPokenavCondition_Tilemap).cast::<u32>()).cast::<u32>(),
                    (((menu).wrapping_add(4)).cast::<u8>()).cast::<u8>(),
                );
                SetBgTilemapBuffer(3u8, (((menu).wrapping_add(4)).cast::<u8>()).cast::<u8>());
                if IsConditionMenuSearchMode() == 1u32 {
                    CopyToBgTilemapBufferRect(
                        3u8,
                        (((&raw mut gPokenavOptions_Tilemap).cast::<u16>()).cast::<u16>())
                            .cast::<u8>(),
                        0u8,
                        5u8,
                        9u8,
                        4u8,
                    );
                }
                CopyBgTilemapBufferToVram(3u8);
                CopyPaletteIntoBufferUnfaded(
                    ((&raw mut gPokenavCondition_Pal).cast::<u16>()).cast::<u16>(),
                    16u32,
                    32u32,
                );
                CopyPaletteIntoBufferUnfaded(
                    ((&raw const gConditionText_Pal)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<u16>())
                    .cast::<u16>(),
                    240u32,
                    32u32,
                );
                ((menu).wrapping_add(6164).cast::<i16>()).write((-80i16));
                return 0u32;
            }
            if __sw1 == 4i32 {
                if (FreeTempTileDataBuffersIfPossible()) != 0 {
                    return 2u32;
                }
                LZ77UnCompVram(
                    ((&raw const sConditionGraphData_Tilemap)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<u32>())
                    .cast::<u32>(),
                    ((((menu).wrapping_add(4)).cast::<u8>()).wrapping_offset(4096)).cast::<u8>(),
                );
                SetBgTilemapBuffer(
                    2u8,
                    ((((menu).wrapping_add(4)).cast::<u8>()).wrapping_offset(4096)).cast::<u8>(),
                );
                CopyBgTilemapBufferToVram(2u8);
                CopyPaletteIntoBufferUnfaded(
                    ((&raw const gConditionGraphData_Pal)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<u16>())
                    .cast::<u16>(),
                    48u32,
                    32u32,
                );
                ConditionGraph_InitWindow(2u8);
                return 0u32;
            }
            if __sw1 == 5i32 {
                BgDmaFill(1u32, 0u8, 0i32, 1i32);
                BgDmaFill(1u32, 17u8, 1i32, 1i32);
                'l2: loop {
                    'l3: {
                        {
                            let mut tmp: u32 = 0u32;
                            (&raw mut tmp).write_volatile(0u32);
                            'l4: loop {
                                'l5: {
                                    CpuSet(
                                        (&raw mut tmp).cast::<u8>(),
                                        ((((menu).wrapping_add(4)).cast::<u8>())
                                            .wrapping_offset(2048))
                                        .cast::<u8>(),
                                        ((83886080i32
                                            | (crate::c::div_i32(
                                                2048i32,
                                                crate::c::div_i32(32i32, 8i32),
                                            ) & 2097151i32))
                                            as u32),
                                    );
                                }
                                if !((0i32) != 0) {
                                    break 'l4;
                                }
                            }
                        }
                    }
                    if !((0i32) != 0) {
                        break 'l2;
                    }
                }
                SetBgTilemapBuffer(
                    1u8,
                    ((((menu).wrapping_add(4)).cast::<u8>()).wrapping_offset(2048)).cast::<u8>(),
                );
                return 0u32;
            }
            if __sw1 == 6i32 {
                if (FreeTempTileDataBuffersIfPossible()) != 0 {
                    return 2u32;
                }
                ((menu).wrapping_add(6176)).write(
                    ((AddWindow(
                        (&raw const sMonNameGenderWindowTemplate)
                            .cast::<u8>()
                            .cast_mut(),
                    )) as u8),
                );
                if IsConditionMenuSearchMode() == 1u32 {
                    ((menu).wrapping_add(6177)).write(
                        ((AddWindow(
                            (&raw const sListIndexWindowTemplate)
                                .cast::<u8>()
                                .cast_mut(),
                        )) as u8),
                    );
                    ((menu).wrapping_add(6178)).write(
                        ((AddWindow((&raw const sUnusedWindowTemplate1).cast::<u8>().cast_mut()))
                            as u8),
                    );
                    ((menu).wrapping_add(6179)).write(
                        ((AddWindow((&raw const sUnusedWindowTemplate2).cast::<u8>().cast_mut()))
                            as u8),
                    );
                }
                DeactivateAllTextPrinters();
                return 0u32;
            }
            if __sw1 == 7i32 {
                CreateConditionMonPic(0u8);
                return 0u32;
            }
            if __sw1 == 8i32 {
                CreateMonMarkingsOrPokeballIndicators();
                return 0u32;
            }
            if __sw1 == 9i32 {
                if IsConditionMenuSearchMode() == 1u32 {
                    CopyUnusedConditionWindowsToVram();
                }
                return 0u32;
            }
            if __sw1 == 10i32 {
                UpdateConditionGraphMenuWindows(
                    0u8,
                    ((GetConditionGraphMenuCurrentLoadIndex()) as u16),
                    1u8,
                );
                return 0u32;
            }
            if __sw1 == 11i32 {
                UpdateConditionGraphMenuWindows(
                    1u8,
                    ((GetConditionGraphMenuCurrentLoadIndex()) as u16),
                    1u8,
                );
                return 0u32;
            }
            if __sw1 == 12i32 {
                UpdateConditionGraphMenuWindows(
                    2u8,
                    ((GetConditionGraphMenuCurrentLoadIndex()) as u16),
                    1u8,
                );
                return 0u32;
            }
            if __sw1 == 13i32 {
                if UpdateConditionGraphMenuWindows(
                    3u8,
                    ((GetConditionGraphMenuCurrentLoadIndex()) as u16),
                    1u8,
                ) != 1u32
                {
                    return 2u32;
                }
                PutWindowTilemap(((menu).wrapping_add(6176)).read());
                if IsConditionMenuSearchMode() == 1u32 {
                    PutWindowTilemap(((menu).wrapping_add(6177)).read());
                    PutWindowTilemap(((menu).wrapping_add(6178)).read());
                    PutWindowTilemap(((menu).wrapping_add(6179)).read());
                }
                return 0u32;
            }
            if __sw1 == 14i32 {
                ShowBg(1u8);
                HideBg(2u8);
                ShowBg(3u8);
                if IsConditionMenuSearchMode() == 1u32 {
                    PrintHelpBarText(4u32);
                }
                return 0u32;
            }
            if __sw1 == 15i32 {
                PokenavFadeScreen(1i32);
                if !((IsConditionMenuSearchMode()) != 0) {
                    LoadLeftHeaderGfxForIndex(6u32);
                    ShowLeftHeaderGfx(1u32, 1u32, 0u32);
                    ShowLeftHeaderGfx(6u32, 1u32, 0u32);
                }
                return 0u32;
            }
            if __sw1 == 16i32 {
                if (IsPaletteFadeActive()) != 0 {
                    return 2u32;
                }
                if (!((IsConditionMenuSearchMode()) != 0)) && ((AreLeftHeaderSpritesMoving()) != 0)
                {
                    return 2u32;
                }
                SetVBlankCallback_(Some(VBlankCB_PokenavConditionGraph));
                return 0u32;
            }
            if __sw1 == 17i32 {
                DoConditionGraphEnterTransition();
                ConditionGraph_InitResetScanline(GetConditionGraphPtr());
                return 0u32;
            }
            if __sw1 == 18i32 {
                if (ConditionGraph_ResetScanline(GetConditionGraphPtr())) != 0 {
                    return 2u32;
                }
                return 0u32;
            }
            if __sw1 == 19i32 {
                ToggleGraphData(1u8);
                return 0u32;
            }
            if __sw1 == 20i32 {
                if !((ConditionMenu_UpdateMonEnter(
                    GetConditionGraphPtr(),
                    (menu).wrapping_add(6164).cast::<i16>(),
                )) != 0)
                {
                    ResetConditionSparkleSprites(((menu).wrapping_add(10464)).cast::<*mut u8>());
                    if (IsConditionMenuSearchMode() == 1u32)
                        || (((GetConditionGraphCurrentListIndex()) as i32)
                            != ((GetMonListCount()) as i32))
                    {
                        CreateConditionSparkleSprites(
                            ((menu).wrapping_add(10464)).cast::<*mut u8>(),
                            ((menu).wrapping_add(6166)).read(),
                            GetNumConditionMonSparkles(),
                        );
                    }
                    return 4u32;
                }
                return 2u32;
            }
        }
        return 4u32;
    }
}
pub(crate) unsafe extern "C" fn LoopedTask_ExitConditionGraphMenu(state: i32) -> u32 {
    unsafe {
        let mut state = state;
        let mut menu: *mut u8 = GetSubstructPtr(12u32);
        'l1: {
            let __sw1 = state;
            if __sw1 == 0i32 {
                DoConditionGraphExitTransition();
                DestroyConditionSparkleSprites(((menu).wrapping_add(10464)).cast::<*mut u8>());
                return 1u32;
            }
            if __sw1 == 1i32 {
                if (ConditionMenu_UpdateMonExit(
                    GetConditionGraphPtr(),
                    (menu).wrapping_add(6164).cast::<i16>(),
                )) != 0
                {
                    return 2u32;
                }
                ToggleGraphData(0u8);
                return 1u32;
            }
            if __sw1 == 2i32 {
                PokenavFadeScreen(0i32);
                if !((IsConditionMenuSearchMode()) != 0) {
                    SlideMenuHeaderDown();
                }
                return 0u32;
            }
            if __sw1 == 3i32 {
                if ((IsPaletteFadeActive()) != 0) || ((MainMenuLoopedTaskIsBusy()) != 0) {
                    return 2u32;
                }
                FreeConditionSparkles(((menu).wrapping_add(10464)).cast::<*mut u8>());
                HideBg(1u8);
                HideBg(2u8);
                HideBg(3u8);
                return 1u32;
            }
        }
        return 4u32;
    }
}
pub(crate) unsafe extern "C" fn LoopedTask_TransitionMons(state: i32) -> u32 {
    unsafe {
        let mut state = state;
        let mut menu: *mut u8 = GetSubstructPtr(12u32);
        let mut graph: *mut u8 = GetConditionGraphPtr();
        'l1: {
            let __sw1 = state;
            if __sw1 == 0i32 {
                LoadNextConditionMenuMonData(0u8);
                return 1u32;
            }
            if __sw1 == 1i32 {
                LoadNextConditionMenuMonData(1u8);
                return 1u32;
            }
            if __sw1 == 2i32 {
                LoadNextConditionMenuMonData(2u8);
                DestroyConditionSparkleSprites(((menu).wrapping_add(10464)).cast::<*mut u8>());
                return 1u32;
            }
            if __sw1 == 3i32 {
                ConditionGraph_TryUpdate(graph);
                return 1u32;
            }
            if __sw1 == 4i32 {
                if !((MoveConditionMonOffscreen((menu).wrapping_add(6164).cast::<i16>())) != 0) {
                    CreateConditionMonPic(((GetConditionGraphMenuCurrentLoadIndex()) as u8));
                    return 1u32;
                }
                return 2u32;
            }
            if __sw1 == 5i32 {
                UpdateConditionGraphMenuWindows(
                    0u8,
                    ((GetConditionGraphMenuCurrentLoadIndex()) as u16),
                    0u8,
                );
                return 1u32;
            }
            if __sw1 == 6i32 {
                UpdateConditionGraphMenuWindows(
                    1u8,
                    ((GetConditionGraphMenuCurrentLoadIndex()) as u16),
                    0u8,
                );
                return 1u32;
            }
            if __sw1 == 7i32 {
                UpdateConditionGraphMenuWindows(
                    2u8,
                    ((GetConditionGraphMenuCurrentLoadIndex()) as u16),
                    0u8,
                );
                return 1u32;
            }
            if __sw1 == 8i32 {
                if UpdateConditionGraphMenuWindows(
                    3u8,
                    ((GetConditionGraphMenuCurrentLoadIndex()) as u16),
                    0u8,
                ) == 1u32
                {
                    return 1u32;
                }
                return 2u32;
            }
            if __sw1 == 9i32 {
                graph = GetConditionGraphPtr();
                if !((ConditionMenu_UpdateMonEnter(graph, (menu).wrapping_add(6164).cast::<i16>()))
                    != 0)
                {
                    ResetConditionSparkleSprites(((menu).wrapping_add(10464)).cast::<*mut u8>());
                    if (IsConditionMenuSearchMode() != 1u32)
                        && (((GetConditionGraphCurrentListIndex()) as i32)
                            == ((GetMonListCount()) as i32))
                    {
                        return 1u32;
                    }
                    CreateConditionSparkleSprites(
                        ((menu).wrapping_add(10464)).cast::<*mut u8>(),
                        ((menu).wrapping_add(6166)).read(),
                        GetNumConditionMonSparkles(),
                    );
                    return 1u32;
                }
                return 2u32;
            }
        }
        return 4u32;
    }
}
pub(crate) unsafe extern "C" fn LoopedTask_MoveCursorNoTransition(state: i32) -> u32 {
    unsafe {
        let mut state = state;
        let mut menu: *mut u8 = GetSubstructPtr(12u32);
        'l1: {
            let __sw1 = state;
            if __sw1 == 0i32 {
                LoadNextConditionMenuMonData(0u8);
                return 1u32;
            }
            if __sw1 == 1i32 {
                LoadNextConditionMenuMonData(1u8);
                return 1u32;
            }
            if __sw1 == 2i32 {
                LoadNextConditionMenuMonData(2u8);
                return 1u32;
            }
            if __sw1 == 3i32 {
                CreateConditionMonPic(((GetConditionGraphMenuCurrentLoadIndex()) as u8));
                return 1u32;
            }
            if __sw1 == 4i32 {
                UpdateConditionGraphMenuWindows(
                    0u8,
                    ((GetConditionGraphMenuCurrentLoadIndex()) as u16),
                    0u8,
                );
                return 1u32;
            }
            if __sw1 == 5i32 {
                UpdateConditionGraphMenuWindows(
                    1u8,
                    ((GetConditionGraphMenuCurrentLoadIndex()) as u16),
                    0u8,
                );
                return 1u32;
            }
            if __sw1 == 6i32 {
                UpdateConditionGraphMenuWindows(
                    2u8,
                    ((GetConditionGraphMenuCurrentLoadIndex()) as u16),
                    0u8,
                );
                return 1u32;
            }
            if __sw1 == 7i32 {
                if UpdateConditionGraphMenuWindows(
                    3u8,
                    ((GetConditionGraphMenuCurrentLoadIndex()) as u16),
                    0u8,
                ) == 1u32
                {
                    return 1u32;
                }
                return 2u32;
            }
            if __sw1 == 8i32 {
                if !((ConditionMenu_UpdateMonEnter(
                    GetConditionGraphPtr(),
                    (menu).wrapping_add(6164).cast::<i16>(),
                )) != 0)
                {
                    ResetConditionSparkleSprites(((menu).wrapping_add(10464)).cast::<*mut u8>());
                    CreateConditionSparkleSprites(
                        ((menu).wrapping_add(10464)).cast::<*mut u8>(),
                        ((menu).wrapping_add(6166)).read(),
                        GetNumConditionMonSparkles(),
                    );
                    return 1u32;
                }
                return 2u32;
            }
        }
        return 4u32;
    }
}
pub(crate) unsafe extern "C" fn LoopedTask_SlideMonOut(state: i32) -> u32 {
    unsafe {
        let mut state = state;
        let mut menu: *mut u8 = GetSubstructPtr(12u32);
        'l1: {
            let __sw1 = state;
            if __sw1 == 0i32 {
                LoadNextConditionMenuMonData(0u8);
                return 1u32;
            }
            if __sw1 == 1i32 {
                LoadNextConditionMenuMonData(1u8);
                return 1u32;
            }
            if __sw1 == 2i32 {
                LoadNextConditionMenuMonData(2u8);
                DestroyConditionSparkleSprites(((menu).wrapping_add(10464)).cast::<*mut u8>());
                return 1u32;
            }
            if __sw1 == 3i32 {
                if !((ConditionMenu_UpdateMonExit(
                    GetConditionGraphPtr(),
                    (menu).wrapping_add(6164).cast::<i16>(),
                )) != 0)
                {
                    return 1u32;
                }
                return 2u32;
            }
            if __sw1 == 4i32 {
                UpdateConditionGraphMenuWindows(
                    0u8,
                    ((GetConditionGraphMenuCurrentLoadIndex()) as u16),
                    0u8,
                );
                return 1u32;
            }
            if __sw1 == 5i32 {
                UpdateConditionGraphMenuWindows(
                    1u8,
                    ((GetConditionGraphMenuCurrentLoadIndex()) as u16),
                    0u8,
                );
                return 1u32;
            }
            if __sw1 == 6i32 {
                UpdateConditionGraphMenuWindows(
                    2u8,
                    ((GetConditionGraphMenuCurrentLoadIndex()) as u16),
                    0u8,
                );
                return 1u32;
            }
            if __sw1 == 7i32 {
                if UpdateConditionGraphMenuWindows(
                    3u8,
                    ((GetConditionGraphMenuCurrentLoadIndex()) as u16),
                    0u8,
                ) == 1u32
                {
                    return 1u32;
                }
                return 2u32;
            }
        }
        return 4u32;
    }
}
pub(crate) unsafe extern "C" fn LoopedTask_OpenMonMarkingsWindow(state: i32) -> u32 {
    unsafe {
        let mut state = state;
        'l1: {
            let __sw1 = state;
            if __sw1 == 0i32 {
                OpenMonMarkingsMenu(TryGetMonMarkId(), 176i16, 32i16);
                return 1u32;
            }
            if __sw1 == 1i32 {
                PrintHelpBarText(5u32);
                return 1u32;
            }
            if __sw1 == 2i32 {
                if WaitForHelpBar() == 1u32 {
                    return 2u32;
                }
                return 1u32;
            }
        }
        return 4u32;
    }
}
pub(crate) unsafe extern "C" fn LoopedTask_CloseMonMarkingsWindow(state: i32) -> u32 {
    unsafe {
        let mut state = state;
        'l1: {
            let __sw1 = state;
            if __sw1 == 0i32 {
                FreeMonMarkingsMenu();
                return 1u32;
            }
            if __sw1 == 1i32 {
                PrintHelpBarText(4u32);
                return 1u32;
            }
            if __sw1 == 2i32 {
                if WaitForHelpBar() == 1u32 {
                    return 2u32;
                }
                return 1u32;
            }
        }
        return 4u32;
    }
}
pub(crate) unsafe extern "C" fn UnusedPrintNumberString(dst: *mut u8, num: u16) -> *mut u8 {
    unsafe {
        let mut dst = dst;
        let mut num = num;
        let mut txtPtr: *mut u8 = ConvertIntToDecimalStringN(dst, ((num) as i32), 1i32, 4u8);
        txtPtr = StringCopy(txtPtr, (&raw mut gText_Number2).cast::<u8>());
        return txtPtr;
    }
}
pub(crate) unsafe extern "C" fn UpdateConditionGraphMenuWindows(
    mode: u8,
    bufferIndex: u16,
    winMode: u8,
) -> u32 {
    unsafe {
        let mut mode = mode;
        let mut bufferIndex = bufferIndex;
        let mut winMode = winMode;
        let mut text = crate::ffi::Align4([0u8; 32]);
        let mut str: *mut u8 = core::ptr::null_mut();
        let mut menu: *mut u8 = GetSubstructPtr(12u32);
        'l1: {
            let __sw1 = ((mode) as i32);
            if __sw1 == 0i32 {
                FillWindowPixelBuffer(((menu).wrapping_add(6176)).read(), 0u8);
                if IsConditionMenuSearchMode() == 1u32 {
                    FillWindowPixelBuffer(((menu).wrapping_add(6177)).read(), 0u8);
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                if (((GetConditionGraphCurrentListIndex()) as i32)
                    != ((GetMonListCount()) as i32).wrapping_sub(1i32))
                    || (IsConditionMenuSearchMode() == 1u32)
                {
                    str = GetConditionMonNameText(((bufferIndex) as u8));
                    AddTextPrinterParameterized(
                        ((menu).wrapping_add(6176)).read(),
                        1u8,
                        str,
                        0u8,
                        1u8,
                        0u8,
                        None,
                    );
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                if IsConditionMenuSearchMode() == 1u32 {
                    str = GetConditionMonLocationText(((bufferIndex) as u8));
                    AddTextPrinterParameterized(
                        ((menu).wrapping_add(6176)).read(),
                        1u8,
                        str,
                        0u8,
                        17u8,
                        0u8,
                        None,
                    );
                    ((&raw mut text).cast::<u8>()).write(252u8);
                    (((&raw mut text).cast::<u8>()).wrapping_offset(1)).write(4u8);
                    (((&raw mut text).cast::<u8>()).wrapping_offset(2)).write(8u8);
                    (((&raw mut text).cast::<u8>()).wrapping_offset(3)).write(0u8);
                    (((&raw mut text).cast::<u8>()).wrapping_offset(4)).write(9u8);
                    StringCopy(
                        ((&raw mut text).cast::<u8>()).wrapping_offset(5),
                        (&raw mut gText_Number2).cast::<u8>(),
                    );
                    AddTextPrinterParameterized(
                        ((menu).wrapping_add(6177)).read(),
                        1u8,
                        (&raw mut text).cast::<u8>(),
                        4u8,
                        1u8,
                        0u8,
                        None,
                    );
                    ConvertIntToDecimalStringN(
                        ((&raw mut text).cast::<u8>()).wrapping_offset(5),
                        ((GetConditionMonDataBuffer()) as i32),
                        1i32,
                        4u8,
                    );
                    AddTextPrinterParameterized(
                        ((menu).wrapping_add(6177)).read(),
                        1u8,
                        (&raw mut text).cast::<u8>(),
                        28u8,
                        1u8,
                        0u8,
                        None,
                    );
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                'l2: {
                    let __sw2 = ((((menu).wrapping_add(10504)).read()) as i32);
                    let mut __fall = false;
                    if __sw2 == 0i32 {
                        __fall = true;
                        if (winMode) != 0 {
                            CopyWindowToVram(((menu).wrapping_add(6176)).read(), 3u8);
                        } else {
                            CopyWindowToVram(((menu).wrapping_add(6176)).read(), 2u8);
                        }
                        if IsConditionMenuSearchMode() == 1u32 {
                            let __p3 = (menu).wrapping_add(10504);
                            (__p3).write(((__p3).read()).wrapping_add(1));
                            return 0u32;
                        } else {
                            ((menu).wrapping_add(10504)).write(0u8);
                            return 1u32;
                        }
                    }
                    if __fall || __sw2 == 1i32 {
                        __fall = true;
                        if (winMode) != 0 {
                            CopyWindowToVram(((menu).wrapping_add(6177)).read(), 3u8);
                        } else {
                            CopyWindowToVram(((menu).wrapping_add(6177)).read(), 2u8);
                        }
                        ((menu).wrapping_add(10504)).write(0u8);
                        return 1u32;
                    }
                }
            }
        }
        return 0u32;
    }
}
pub(crate) unsafe extern "C" fn CopyUnusedConditionWindowsToVram() {
    unsafe {
        let mut menu: *mut u8 = GetSubstructPtr(12u32);
        CopyWindowToVram(((menu).wrapping_add(6178)).read(), 3u8);
        CopyWindowToVram(((menu).wrapping_add(6179)).read(), 3u8);
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_PartyPokeball(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32)
            == ((GetConditionGraphCurrentListIndex()) as i32)
        {
            StartSpriteAnim(sprite, 0u8);
        } else {
            StartSpriteAnim(sprite, 1u8);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn HighlightCurrentPartyIndexPokeball(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if ((GetConditionGraphCurrentListIndex()) as i32)
            == ((GetMonListCount()) as i32).wrapping_sub(1i32)
        {
            crate::c::bf_write(
                (sprite).wrapping_add(5),
                4,
                4,
                ((IndexOfSpritePaletteTag(101u16)) as u16) as i32,
            );
        } else {
            crate::c::bf_write(
                (sprite).wrapping_add(5),
                4,
                4,
                ((IndexOfSpritePaletteTag(102u16)) as u16) as i32,
            );
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MonMarkingsCallback(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        StartSpriteAnim(sprite, TryGetMonMarkId());
    }
}
pub(crate) unsafe extern "C" fn CreateMonMarkingsOrPokeballIndicators() {
    unsafe {
        let mut sprSheets = crate::ffi::Align4([0u8; 32]);
        let mut sprTemplate = crate::ffi::Align4([0u8; 24]);
        let mut sprPals = crate::ffi::Align4([0u8; 24]);
        let mut sprSheet = crate::ffi::Align4([0u8; 8]);
        let mut sprite: *mut u8 = core::ptr::null_mut();
        let mut i: u16 = 0u16;
        let mut spriteId: u16 = 0u16;
        let mut menu: *mut u8 = GetSubstructPtr(12u32);
        LoadConditionSelectionIcons(
            (&raw mut sprSheets).cast::<u8>(),
            (&raw mut sprTemplate).cast::<u8>(),
            (&raw mut sprPals).cast::<u8>(),
        );
        if IsConditionMenuSearchMode() == 1u32 {
            (((menu).wrapping_add(6180)).cast::<u16>()).write(106u16);
            (((menu).wrapping_add(6180)).wrapping_add(2).cast::<u16>()).write(106u16);
            InitMonMarkingsMenu((menu).wrapping_add(6180));
            BufferMonMarkingsMenuTiles();
            sprite = CreateMonMarkingAllCombosSprite(
                105u16,
                105u16,
                ((&raw const sMonMarkings_Pal)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<u16>())
                .cast::<u16>(),
            );
            crate::c::bf_write((sprite).wrapping_add(5), 2, 2, (3u16) as i32);
            ((sprite).wrapping_add(32).cast::<i16>()).write(192i16);
            ((sprite).wrapping_add(34).cast::<i16>()).write(32i16);
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(MonMarkingsCallback));
            ((menu).wrapping_add(10460).cast::<*mut u8>()).write(sprite);
            PokenavFillPalette(((IndexOfSpritePaletteTag(105u16)) as u32), 0u16);
        } else {
            LoadSpriteSheets((&raw mut sprSheets).cast::<u8>());
            Pokenav_AllocAndLoadPalettes((&raw mut sprPals).cast::<u8>());
            {
                i = 0u16;
                'l1: loop {
                    if !(((i) as i32) < ((GetMonListCount()) as i32).wrapping_sub(1i32)) {
                        break 'l1;
                    }
                    'l2: {
                        spriteId = ((CreateSprite(
                            (&raw mut sprTemplate).cast::<u8>(),
                            226i16,
                            (((((i) as i32).wrapping_mul(20i32)).wrapping_add(8i32)) as i16),
                            0u8,
                        )) as u16);
                        if ((spriteId) as i32) != 64i32 {
                            ((((menu).wrapping_add(6150)).cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize))
                            .write(((spriteId) as u8));
                            (((((&raw mut gSprites).cast::<u8>())
                                .wrapping_offset(((spriteId) as i32) as isize * 68))
                            .wrapping_add(46))
                            .cast::<i16>())
                            .write(((i) as i16));
                            ((((&raw mut gSprites).cast::<u8>())
                                .wrapping_offset(((spriteId) as i32) as isize * 68))
                            .wrapping_add(28)
                            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                            .write(Some(SpriteCB_PartyPokeball));
                        } else {
                            ((((menu).wrapping_add(6150)).cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize))
                            .write(255u8);
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
            (((&raw mut sprTemplate).cast::<u8>()).cast::<u16>()).write(103u16);
            (((&raw mut sprTemplate).cast::<u8>())
                .wrapping_add(20)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(SpriteCallbackDummy));
            {
                'l3: loop {
                    if !(((i) as i32) < 6i32) {
                        break 'l3;
                    }
                    'l4: {
                        spriteId = ((CreateSprite(
                            (&raw mut sprTemplate).cast::<u8>(),
                            230i16,
                            (((((i) as i32).wrapping_mul(20i32)).wrapping_add(8i32)) as i16),
                            0u8,
                        )) as u16);
                        if ((spriteId) as i32) != 64i32 {
                            ((((menu).wrapping_add(6150)).cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize))
                            .write(((spriteId) as u8));
                            crate::c::bf_write(
                                (((&raw mut gSprites).cast::<u8>())
                                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                                .wrapping_add(3),
                                6,
                                2,
                                (0u32) as i32,
                            );
                        } else {
                            ((((menu).wrapping_add(6150)).cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize))
                            .write(255u8);
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
            (((&raw mut sprTemplate).cast::<u8>()).cast::<u16>()).write(102u16);
            (((&raw mut sprTemplate).cast::<u8>())
                .wrapping_add(20)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(HighlightCurrentPartyIndexPokeball));
            spriteId = ((CreateSprite(
                (&raw mut sprTemplate).cast::<u8>(),
                222i16,
                (((((i) as i32).wrapping_mul(20i32)).wrapping_add(8i32)) as i16),
                0u8,
            )) as u16);
            if ((spriteId) as i32) != 64i32 {
                ((((menu).wrapping_add(6150)).cast::<u8>()).wrapping_offset(((i) as i32) as isize))
                    .write(((spriteId) as u8));
                crate::c::bf_write(
                    (((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68))
                    .wrapping_add(1),
                    6,
                    2,
                    (1u32) as i32,
                );
                crate::c::bf_write(
                    (((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68))
                    .wrapping_add(3),
                    6,
                    2,
                    (2u32) as i32,
                );
            } else {
                ((((menu).wrapping_add(6150)).cast::<u8>()).wrapping_offset(((i) as i32) as isize))
                    .write(255u8);
            }
        }
        LoadConditionSparkle(
            (&raw mut sprSheet).cast::<u8>(),
            (&raw mut sprPals).cast::<u8>(),
        );
        LoadSpriteSheet((&raw mut sprSheet).cast::<u8>());
        ((((&raw mut sprPals).cast::<u8>()).wrapping_offset(8)).cast::<*mut u16>())
            .write(core::ptr::null_mut());
        Pokenav_AllocAndLoadPalettes((&raw mut sprPals).cast::<u8>());
    }
}
pub(crate) unsafe extern "C" fn FreeConditionMenuGfx(menu: *mut u8) {
    unsafe {
        let mut menu = menu;
        let mut i: u8 = 0u8;
        if IsConditionMenuSearchMode() == 1u32 {
            DestroySprite(((menu).wrapping_add(10460).cast::<*mut u8>()).read());
            FreeSpriteTilesByTag(106u16);
            FreeSpriteTilesByTag(105u16);
            FreeSpritePaletteByTag(106u16);
            FreeSpritePaletteByTag(105u16);
        } else {
            {
                i = 0u8;
                'l1: loop {
                    if !(((i) as i32) < 7i32) {
                        break 'l1;
                    }
                    'l2: {
                        DestroySprite(
                            ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                ((((((menu).wrapping_add(6150)).cast::<u8>())
                                    .wrapping_offset(((i) as i32) as isize))
                                .read()) as i32) as isize
                                    * 68,
                            ),
                        );
                    }
                    i = (i).wrapping_add(1);
                }
            }
            FreeSpriteTilesByTag(101u16);
            FreeSpriteTilesByTag(102u16);
            FreeSpriteTilesByTag(103u16);
            FreeSpritePaletteByTag(101u16);
            FreeSpritePaletteByTag(102u16);
        }
        if ((((menu).wrapping_add(6166)).read()) as i32) != 255i32 {
            DestroySprite(
                ((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((((menu).wrapping_add(6166)).read()) as i32) as isize * 68),
            );
            FreeSpriteTilesByTag(100u16);
            FreeSpritePaletteByTag(100u16);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FreeConditionGraphMenuSubstruct2() {
    unsafe {
        let mut menu: *mut u8 = GetSubstructPtr(12u32);
        RemoveWindow(((menu).wrapping_add(6176)).read());
        if IsConditionMenuSearchMode() == 1u32 {
            RemoveWindow(((menu).wrapping_add(6177)).read());
            RemoveWindow(((menu).wrapping_add(6178)).read());
            RemoveWindow(((menu).wrapping_add(6179)).read());
        } else {
            SetLeftHeaderSpritesInvisibility();
        }
        SetGpuReg(0u8, 4416u16);
        FreeConditionMenuGfx(menu);
        SetExitVBlank();
        FreePokenavSubstruct(12u32);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MonPicGfxSpriteCallback(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut menu: *mut u8 = GetSubstructPtr(12u32);
        ((sprite).wrapping_add(32).cast::<i16>()).write(
            ((((((menu).wrapping_add(6164).cast::<i16>()).read()) as i32).wrapping_add(38i32))
                as i16),
        );
    }
}
pub(crate) unsafe extern "C" fn CreateConditionMonPic(id: u8) {
    unsafe {
        let mut id = id;
        let mut sprTemplate = crate::ffi::Align4([0u8; 24]);
        let mut sprSheet = crate::ffi::Align4([0u8; 8]);
        let mut sprPal = crate::ffi::Align4([0u8; 8]);
        let mut spriteId: u8 = 0u8;
        let mut menu: *mut u8 = GetSubstructPtr(12u32);
        if ((((menu).wrapping_add(6166)).read()) as i32) == 255i32 {
            LoadConditionMonPicTemplate(
                (&raw mut sprSheet).cast::<u8>(),
                (&raw mut sprTemplate).cast::<u8>(),
                (&raw mut sprPal).cast::<u8>(),
            );
            (((&raw mut sprSheet).cast::<u8>()).cast::<*mut u8>()).write(GetConditionMonPicGfx(id));
            (((&raw mut sprPal).cast::<u8>()).cast::<*mut u16>())
                .write((GetConditionMonPal(id)).cast::<u16>());
            ((menu).wrapping_add(6168).cast::<u16>())
                .write(((LoadSpritePalette((&raw mut sprPal).cast::<u8>())) as u16));
            ((menu).wrapping_add(6170).cast::<u16>())
                .write(LoadSpriteSheet((&raw mut sprSheet).cast::<u8>()));
            spriteId = CreateSprite((&raw mut sprTemplate).cast::<u8>(), 38i16, 104i16, 0u8);
            ((menu).wrapping_add(6166)).write(spriteId);
            if ((spriteId) as i32) == 64i32 {
                FreeSpriteTilesByTag(100u16);
                FreeSpritePaletteByTag(100u16);
                ((menu).wrapping_add(6166)).write(255u8);
            } else {
                ((menu).wrapping_add(6166)).write(spriteId);
                ((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((((menu).wrapping_add(6166)).read()) as i32) as isize * 68))
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                .write(Some(MonPicGfxSpriteCallback));
                ((menu).wrapping_add(6172).cast::<*mut u8>()).write(
                    (((100663296i32) as usize as *mut u8).wrapping_offset(65536)).wrapping_offset(
                        (((((menu).wrapping_add(6170).cast::<u16>()).read()) as i32)
                            .wrapping_mul(32i32)) as isize
                            * 1,
                    ),
                );
                ((menu).wrapping_add(6168).cast::<u16>()).write(
                    (((256i32).wrapping_add(
                        ((((menu).wrapping_add(6168).cast::<u16>()).read()) as i32)
                            .wrapping_mul(16i32),
                    )) as u16),
                );
            }
        } else {
            {
                let mut _src: *mut u8 = GetConditionMonPicGfx(id);
                let mut _dest: *mut u8 = ((menu).wrapping_add(6172).cast::<*mut u8>()).read();
                let mut _size: u32 = ((crate::c::div_i32(4096i32, 2i32)) as u32);
                'l1: loop {
                    'l2: {
                        'l3: loop {
                            'l4: {
                                {
                                    let mut dmaRegs: *mut u32 =
                                        ((67109076i32) as usize as *mut u32);
                                    crate::c::volatile_write(dmaRegs, ((_src) as usize as u32));
                                    crate::c::volatile_write(
                                        (dmaRegs).wrapping_offset(1),
                                        ((_dest) as usize as u32),
                                    );
                                    crate::c::volatile_write(
                                        (dmaRegs).wrapping_offset(2),
                                        (2147483648u32
                                            | crate::c::div_u32(
                                                _size,
                                                ((crate::c::div_i32(16i32, 8i32)) as u32),
                                            )),
                                    );
                                    let _ = ((dmaRegs).wrapping_offset(2)).read_volatile();
                                }
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
            LoadPalette(
                GetConditionMonPal(id),
                ((menu).wrapping_add(6168).cast::<u16>()).read(),
                32u16,
            );
        }
    }
}
pub(crate) unsafe extern "C" fn VBlankCB_PokenavConditionGraph() {
    unsafe {
        let mut graph: *mut u8 = GetConditionGraphPtr();
        LoadOam();
        ProcessSpriteCopyRequests();
        TransferPlttBuffer();
        ConditionGraph_Draw(graph);
        ScanlineEffect_InitHBlankDmaTransfer();
    }
}
pub(crate) unsafe extern "C" fn SetExitVBlank() {
    unsafe {
        SetPokenavVBlankCallback();
    }
}
pub(crate) unsafe extern "C" fn ToggleGraphData(showBg: u8) {
    unsafe {
        let mut showBg = showBg;
        if (showBg) != 0 {
            ShowBg(2u8);
        } else {
            HideBg(2u8);
        }
    }
}
pub(crate) unsafe extern "C" fn DoConditionGraphEnterTransition() {
    unsafe {
        let mut graph: *mut u8 = GetConditionGraphPtr();
        let mut id: u8 = ((GetConditionGraphMenuCurrentLoadIndex()) as u8);
        ((&raw mut sInitialLoadId).cast::<u8>().cast::<u8>()).write(id);
        ConditionGraph_SetNewPositions(
            graph,
            ((((graph).wrapping_add(20)).cast::<u8>()).wrapping_offset(60)).cast::<u8>(),
            ((((graph).wrapping_add(20)).cast::<u8>())
                .wrapping_offset(((id) as i32) as isize * 20))
            .cast::<u8>(),
        );
        ConditionGraph_TryUpdate(graph);
    }
}
pub(crate) unsafe extern "C" fn DoConditionGraphExitTransition() {
    unsafe {
        let mut graph: *mut u8 = GetConditionGraphPtr();
        if ((IsConditionMenuSearchMode()) != 0)
            || (((GetConditionGraphCurrentListIndex()) as i32)
                != ((GetMonListCount()) as i32).wrapping_sub(1i32))
        {
            ConditionGraph_SetNewPositions(
                graph,
                ((((graph).wrapping_add(20)).cast::<u8>()).wrapping_offset(
                    ((GetConditionGraphMenuCurrentLoadIndex()) as i32) as isize * 20,
                ))
                .cast::<u8>(),
                ((((graph).wrapping_add(20)).cast::<u8>()).wrapping_offset(60)).cast::<u8>(),
            );
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetMonMarkingsData() -> u8 {
    unsafe {
        let mut menu: *mut u8 = GetSubstructPtr(12u32);
        if IsConditionMenuSearchMode() == 1u32 {
            return (((menu).wrapping_add(6180)).wrapping_add(4)).read();
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
