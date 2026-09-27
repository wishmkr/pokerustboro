//! Translated from `src/pokenav_menu_handler_gfx.c` by tools/rustport/c2rs.py, then reviewed.
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

// Data tables (translate with cdata.py): sPokenavBgDotsPal sPokenavBgDotsTiles sPokenavBgDotsTilemap sPokenavDeviceBgPal sPokenavDeviceBgTiles sPokenavDeviceBgTilemap sMatchCallBlueLightPal sMatchCallBlueLightTiles sPokenavMainMenuBgTemplates sMenuHandlerLoopTaskFuncs sPokenavOptionsSpriteSheets sPokenavOptionsSpritePalettes sOptionsLabelGfx_RegionMap sOptionsLabelGfx_Condition sOptionsLabelGfx_MatchCall sOptionsLabelGfx_Ribbons sOptionsLabelGfx_SwitchOff sOptionsLabelGfx_Party sOptionsLabelGfx_Search sOptionsLabelGfx_Cool sOptionsLabelGfx_Beauty sOptionsLabelGfx_Cute sOptionsLabelGfx_Smart sOptionsLabelGfx_Tough sOptionsLabelGfx_Cancel sPokenavMenuOptionLabelGfx sOptionDescWindowTemplate sPageDescriptions sOptionDescTextColors sOptionDescTextColors2 sOamData_MenuOption sAffineAnim_MenuOption_Normal sAffineAnim_MenuOption_Zoom sAffineAnims_MenuOption sMenuOptionSpriteTemplate sBlueLightOamData sMatchCallBlueLightSpriteTemplate sPokenavMainMenuScanlineEffectParams
#[allow(unused_imports)]
use crate::data::pokenav_menu_handler_gfx::*;

unsafe extern "C" {
    static mut gMapHeader: u8;
    static mut gPokenavMessageBox_Gfx: u8;
    static mut gPokenavMessageBox_Pal: u8;
    static mut gPokenavMessageBox_Tilemap: u8;
    static mut gSaveBlock1Ptr: u8;
    static mut gScanlineEffectRegBuffers: u8;
    static mut gSineTable: u8;
    static mut gSprites: u8;
    static mut gTasks: u8;
    static mut gText_NoRibbonWinners: u8;
    fn AddTextPrinterParameterized3(
        a0: u8,
        a1: u8,
        a2: u8,
        a3: u8,
        a4: *mut u8,
        a5: i8,
        a6: *mut u8,
    );
    fn AddWindow(a0: *mut u8) -> u16;
    fn AllocSubstruct(a0: u32, a1: u32) -> *mut u8;
    fn AreLeftHeaderSpritesMoving() -> u32;
    fn CalcCenterToCornerVec(a0: *mut u8, a1: u8, a2: u8, a3: u8);
    fn ChangeBgX(a0: u8, a1: i32, a2: u8) -> i32;
    fn ChangeBgY(a0: u8, a1: i32, a2: u8) -> i32;
    fn ClearGpuRegBits(a0: u8, a1: u16);
    fn CopyBgTilemapBufferToVram(a0: u8);
    fn CopyPaletteIntoBufferUnfaded(a0: *mut u16, a1: u32, a2: u32);
    fn CopyToBgTilemapBuffer(a0: u8, a1: *mut u8, a2: u16, a3: u16);
    fn CopyWindowToVram(a0: u8, a1: u8);
    fn CpuSet(a0: *mut u8, a1: *mut u8, a2: u32);
    fn CreateLoopedTask(a0: Option<unsafe extern "C" fn(i32) -> u32>, a1: u32) -> u32;
    fn CreateSprite(a0: *mut u8, a1: i16, a2: i16, a3: u8) -> u8;
    fn CreateTask(a0: Option<unsafe extern "C" fn(u8)>, a1: u8) -> u8;
    fn DecompressAndCopyTileDataToVram(a0: u8, a1: *mut u8, a2: u32, a3: u16, a4: u8) -> *mut u8;
    fn DestroySprite(a0: *mut u8);
    fn DestroyTask(a0: u8);
    fn FillWindowPixelBuffer(a0: u8, a1: u8);
    fn FindTaskIdByFunc(a0: Option<unsafe extern "C" fn(u8)>) -> u8;
    fn FreeOamMatrix(a0: u8);
    fn FreePokenavSubstruct(a0: u32);
    fn FreeSpriteOamMatrix(a0: *mut u8);
    fn FreeSpritePaletteByTag(a0: u16);
    fn FreeSpriteTilesByTag(a0: u16);
    fn FreeTempTileDataBuffersIfPossible() -> u8;
    fn FuncIsActiveTask(a0: Option<unsafe extern "C" fn(u8)>) -> u8;
    fn GetCurrentMenuItemId() -> i32;
    fn GetHelpBarTextId() -> u16;
    fn GetMatchTableMapSectionId(a0: i32) -> u8;
    fn GetPokenavCursorPos() -> i32;
    fn GetPokenavMenuType() -> i32;
    fn GetSpriteTileStartByTag(a0: u16) -> u16;
    fn GetStringWidth(a0: u8, a1: *mut u8, a2: i16) -> i32;
    fn GetSubstructPtr(a0: u32) -> *mut u8;
    fn GetWordTaskArg(a0: u8, a1: u8) -> u32;
    fn HideMainOrSubMenuLeftHeader(a0: u32, a1: u32);
    fn IndexOfSpritePaletteTag(a0: u16) -> u8;
    fn InitBgTemplates(a0: *mut u8, a1: i32);
    fn InitSpriteAffineAnim(a0: *mut u8);
    fn IsDma3ManagerBusyWithBgCopy() -> u8;
    fn IsLoopedTaskActive(a0: u32) -> u32;
    fn IsPaletteFadeActive() -> u32;
    fn IsRematchEntryRegistered(a0: i32) -> u32;
    fn LoadCompressedSpriteSheet(a0: *mut u8) -> u16;
    fn LoadLeftHeaderGfxForIndex(a0: u32);
    fn LoadOam();
    fn LoadPalette(a0: *mut u8, a1: u16, a2: u16);
    fn PlaySE(a0: u16);
    fn PokenavCopyPalette(a0: *mut u16, a1: *mut u16, a2: i32, a3: i32, a4: i32, a5: *mut u16);
    fn PokenavFadeScreen(a0: i32);
    fn Pokenav_AllocAndLoadPalettes(a0: *mut u8);
    fn PrintHelpBarText(a0: u32);
    fn ProcessSpriteCopyRequests();
    fn PutWindowTilemap(a0: u8);
    fn RemoveWindow(a0: u8);
    fn ScanlineEffect_InitHBlankDmaTransfer();
    fn ScanlineEffect_SetParams(a0: crate::c::Rec4<12>);
    fn ScanlineEffect_Stop();
    fn SetBgTilemapBuffer(a0: u8, a1: *mut u8);
    fn SetGpuReg(a0: u8, a1: u16);
    fn SetGpuRegBits(a0: u8, a1: u16);
    fn SetPokenavVBlankCallback();
    fn SetVBlankCallback_(a0: Option<unsafe extern "C" fn()>);
    fn SetWordTaskArg(a0: u8, a1: u8, a2: u32);
    fn ShowBg(a0: u8);
    fn ShowLeftHeaderGfx(a0: u32, a1: u32, a2: u32);
    fn SlideMenuHeaderUp();
    fn SpriteCallbackDummy(a0: *mut u8);
    fn StartSpriteAffineAnim(a0: *mut u8, a1: u8);
    fn TransferPlttBuffer();
    fn WaitForHelpBar() -> u32;
}

pub(crate) unsafe extern "C" fn AreAnyTrainerRematchesNearby() -> u32 {
    unsafe {
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 78i32) {
                    break 'l1;
                }
                'l2: {
                    if ((((GetMatchTableMapSectionId(i)) as i32)
                        == (((((&raw mut gMapHeader).cast::<u8>()).wrapping_add(20)).read())
                            as i32))
                        && ((IsRematchEntryRegistered(i)) != 0))
                        && ((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(2506))
                        .cast::<u8>())
                        .wrapping_offset((i) as isize))
                        .read())
                            != 0)
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
pub unsafe extern "C" fn OpenPokenavMenuInitial() -> u32 {
    unsafe {
        let mut gfx: *mut u8 = OpenPokenavMenu();
        if ((gfx) as usize) == 0usize {
            return 0u32;
        }
        ((gfx).wrapping_add(13)).write(0u8);
        return 1u32;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn OpenPokenavMenuNotInitial() -> u32 {
    unsafe {
        let mut gfx: *mut u8 = OpenPokenavMenu();
        if ((gfx) as usize) == 0usize {
            return 0u32;
        }
        ((gfx).wrapping_add(13)).write(1u8);
        return 1u32;
    }
}
pub(crate) unsafe extern "C" fn OpenPokenavMenu() -> *mut u8 {
    unsafe {
        let mut gfx: *mut u8 = AllocSubstruct(2u32, 2188u32);
        if ((gfx) as usize) != 0usize {
            ((gfx).wrapping_add(12)).write(0u8);
            ((gfx).wrapping_add(4).cast::<u32>())
                .write(CreateLoopedTask(Some(LoopedTask_OpenMenu), 1u32));
            ((gfx).cast::<Option<unsafe extern "C" fn() -> u32>>())
                .write(Some(GetCurrentLoopedTaskActive));
        }
        return gfx;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CreateMenuHandlerLoopedTask(ltIdx: i32) {
    unsafe {
        let mut ltIdx = ltIdx;
        let mut gfx: *mut u8 = GetSubstructPtr(2u32);
        ((gfx).wrapping_add(4).cast::<u32>()).write(CreateLoopedTask(
            ((((&raw const sMenuHandlerLoopTaskFuncs)
                .cast::<u8>()
                .cast_mut()
                .cast::<Option<unsafe extern "C" fn(i32) -> u32>>())
            .cast::<Option<unsafe extern "C" fn(i32) -> u32>>())
            .wrapping_offset((ltIdx) as isize))
            .read(),
            1u32,
        ));
        ((gfx).cast::<Option<unsafe extern "C" fn() -> u32>>())
            .write(Some(GetCurrentLoopedTaskActive));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsMenuHandlerLoopedTaskActive() -> u32 {
    unsafe {
        let mut gfx: *mut u8 = GetSubstructPtr(2u32);
        return (((gfx).cast::<Option<unsafe extern "C" fn() -> u32>>()).read()).unwrap_unchecked()(
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FreeMenuHandlerSubstruct2() {
    unsafe {
        let mut gfx: *mut u8 = GetSubstructPtr(2u32);
        DestroyMovingDotsBgTask();
        RemoveWindow(((((gfx).wrapping_add(8).cast::<u16>()).read()) as u8));
        FreeAndDestroyMainMenuSprites();
        DestroyMenuOptionGlowTask();
        FreePokenavSubstruct(2u32);
    }
}
pub(crate) unsafe extern "C" fn GetCurrentLoopedTaskActive() -> u32 {
    unsafe {
        let mut gfx: *mut u8 = GetSubstructPtr(2u32);
        return IsLoopedTaskActive(((gfx).wrapping_add(4).cast::<u32>()).read());
    }
}
pub(crate) unsafe extern "C" fn LoopedTask_OpenMenu(state: i32) -> u32 {
    unsafe {
        let mut state = state;
        let mut gfx: *mut u8 = GetSubstructPtr(2u32);
        'l1: {
            let __sw1 = state;
            if __sw1 == 0i32 {
                InitBgTemplates(
                    ((&raw const sPokenavMainMenuBgTemplates)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>(),
                    ((crate::c::div_u32(12u32, 4u32)) as i32),
                );
                DecompressAndCopyTileDataToVram(
                    1u8,
                    (((&raw mut gPokenavMessageBox_Gfx).cast::<u32>()).cast::<u32>()).cast::<u8>(),
                    0u32,
                    0u16,
                    0u8,
                );
                SetBgTilemapBuffer(1u8, ((gfx).wrapping_add(140)).cast::<u8>());
                CopyToBgTilemapBuffer(
                    1u8,
                    (((&raw mut gPokenavMessageBox_Tilemap).cast::<u32>()).cast::<u32>())
                        .cast::<u8>(),
                    0u16,
                    0u16,
                );
                CopyBgTilemapBufferToVram(1u8);
                CopyPaletteIntoBufferUnfaded(
                    ((&raw mut gPokenavMessageBox_Pal).cast::<u16>()).cast::<u16>(),
                    16u32,
                    32u32,
                );
                ChangeBgX(1u8, 0i32, 0u8);
                ChangeBgY(1u8, 0i32, 0u8);
                ChangeBgX(2u8, 0i32, 0u8);
                ChangeBgY(2u8, 0i32, 0u8);
                ChangeBgX(3u8, 0i32, 0u8);
                ChangeBgY(3u8, 0i32, 0u8);
                return 0u32;
            }
            if __sw1 == 1i32 {
                if (FreeTempTileDataBuffersIfPossible()) != 0 {
                    return 2u32;
                }
                DecompressAndCopyTileDataToVram(
                    2u8,
                    (((&raw const sPokenavDeviceBgTiles)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<u32>())
                    .cast::<u32>())
                    .cast::<u8>(),
                    0u32,
                    0u16,
                    0u8,
                );
                DecompressAndCopyTileDataToVram(
                    2u8,
                    (((&raw const sPokenavDeviceBgTilemap)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<u32>())
                    .cast::<u32>())
                    .cast::<u8>(),
                    0u32,
                    0u16,
                    1u8,
                );
                CopyPaletteIntoBufferUnfaded(
                    ((&raw const sPokenavDeviceBgPal)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<u16>())
                    .cast::<u16>(),
                    32u32,
                    32u32,
                );
                return 0u32;
            }
            if __sw1 == 2i32 {
                if (FreeTempTileDataBuffersIfPossible()) != 0 {
                    return 2u32;
                }
                DecompressAndCopyTileDataToVram(
                    3u8,
                    (((&raw const sPokenavBgDotsTiles)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<u32>())
                    .cast::<u32>())
                    .cast::<u8>(),
                    0u32,
                    0u16,
                    0u8,
                );
                DecompressAndCopyTileDataToVram(
                    3u8,
                    (((&raw const sPokenavBgDotsTilemap)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<u32>())
                    .cast::<u32>())
                    .cast::<u8>(),
                    0u32,
                    0u16,
                    1u8,
                );
                CopyPaletteIntoBufferUnfaded(
                    ((&raw const sPokenavBgDotsPal)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<u16>())
                    .cast::<u16>(),
                    48u32,
                    32u32,
                );
                if (GetPokenavMenuType() == 3i32) || (GetPokenavMenuType() == 4i32) {
                    ChangeBgDotsColorToPurple();
                }
                return 0u32;
            }
            if __sw1 == 3i32 {
                if (FreeTempTileDataBuffersIfPossible()) != 0 {
                    return 2u32;
                }
                AddOptionDescriptionWindow();
                CreateMovingBgDotsTask();
                return 1u32;
            }
            if __sw1 == 4i32 {
                LoadPokenavOptionPalettes();
                return 1u32;
            }
            if __sw1 == 5i32 {
                PrintCurrentOptionDescription();
                CreateMenuOptionSprites();
                CreateMatchCallBlueLightSprite();
                DrawCurrentMenuOptionLabels();
                return 0u32;
            }
            if __sw1 == 6i32 {
                if (IsDma3ManagerBusyWithBgCopy_()) != 0 {
                    return 2u32;
                }
                return 1u32;
            }
            if __sw1 == 7i32 {
                ShowBg(1u8);
                ShowBg(2u8);
                ShowBg(3u8);
                if (((gfx).wrapping_add(13)).read()) != 0 {
                    PokenavFadeScreen(1i32);
                } else {
                    PlaySE(110u16);
                    PokenavFadeScreen(3i32);
                }
                'l2: {
                    let __sw2 = GetPokenavMenuType();
                    let __matched = __sw2 == 4i32 || __sw2 == 3i32;
                    let mut __fall = false;
                    if __sw2 == 4i32 {
                        __fall = true;
                        LoadLeftHeaderGfxForIndex(7u32);
                    }
                    if __fall || __sw2 == 3i32 {
                        __fall = true;
                        LoadLeftHeaderGfxForIndex(1u32);
                        break 'l2;
                    }
                    if !__matched {
                        __fall = true;
                        LoadLeftHeaderGfxForIndex(0u32);
                        break 'l2;
                    }
                }
                return 0u32;
            }
            if __sw1 == 8i32 {
                if (IsPaletteFadeActive()) != 0 {
                    return 2u32;
                }
                'l3: {
                    let __sw3 = GetPokenavMenuType();
                    let __matched = __sw3 == 4i32 || __sw3 == 3i32;
                    let mut __fall = false;
                    if __sw3 == 4i32 {
                        __fall = true;
                        ShowLeftHeaderGfx(7u32, 0u32, 0u32);
                    }
                    if __fall || __sw3 == 3i32 {
                        __fall = true;
                        ShowLeftHeaderGfx(1u32, 0u32, 0u32);
                        break 'l3;
                    }
                    if !__matched {
                        __fall = true;
                        ShowLeftHeaderGfx(0u32, 0u32, 0u32);
                        break 'l3;
                    }
                }
                StartOptionAnimations_Enter();
                SetupPokenavMenuScanlineEffects();
                return 1u32;
            }
            if __sw1 == 9i32 {
                if (AreMenuOptionSpritesMoving()) != 0 {
                    return 2u32;
                }
                if (AreLeftHeaderSpritesMoving()) != 0 {
                    return 2u32;
                }
                break 'l1;
            }
        }
        return 4u32;
    }
}
pub(crate) unsafe extern "C" fn LoopedTask_MoveMenuCursor(state: i32) -> u32 {
    unsafe {
        let mut state = state;
        'l1: {
            let __sw1 = state;
            if __sw1 == 0i32 {
                SetMenuOptionGlow();
                StartOptionAnimations_CursorMoved();
                PrintCurrentOptionDescription();
                PlaySE(5u16);
                return 0u32;
            }
            if __sw1 == 1i32 {
                if (AreMenuOptionSpritesMoving()) != 0 {
                    return 2u32;
                }
                if (IsDma3ManagerBusyWithBgCopy_()) != 0 {
                    return 2u32;
                }
                break 'l1;
            }
        }
        return 4u32;
    }
}
pub(crate) unsafe extern "C" fn LoopedTask_OpenConditionMenu(state: i32) -> u32 {
    unsafe {
        let mut state = state;
        'l1: {
            let __sw1 = state;
            if __sw1 == 0i32 {
                ResetBldCnt();
                StartOptionAnimations_Exit();
                HideMainOrSubMenuLeftHeader(0u32, 0u32);
                PlaySE(5u16);
                return 0u32;
            }
            if __sw1 == 1i32 {
                if (AreMenuOptionSpritesMoving()) != 0 {
                    return 2u32;
                }
                if (AreLeftHeaderSpritesMoving()) != 0 {
                    return 2u32;
                }
                DrawCurrentMenuOptionLabels();
                LoadLeftHeaderGfxForIndex(1u32);
                return 0u32;
            }
            if __sw1 == 2i32 {
                StartOptionAnimations_Enter();
                ShowLeftHeaderGfx(1u32, 0u32, 0u32);
                CreateBgDotPurplePalTask();
                PrintCurrentOptionDescription();
                return 0u32;
            }
            if __sw1 == 3i32 {
                if (AreMenuOptionSpritesMoving()) != 0 {
                    return 2u32;
                }
                if (AreLeftHeaderSpritesMoving()) != 0 {
                    return 2u32;
                }
                if (IsTaskActive_UpdateBgDotsPalette()) != 0 {
                    return 2u32;
                }
                if (IsDma3ManagerBusyWithBgCopy_()) != 0 {
                    return 2u32;
                }
                InitMenuOptionGlow();
                break 'l1;
            }
        }
        return 4u32;
    }
}
pub(crate) unsafe extern "C" fn LoopedTask_ReturnToMainMenu(state: i32) -> u32 {
    unsafe {
        let mut state = state;
        'l1: {
            let __sw1 = state;
            if __sw1 == 0i32 {
                ResetBldCnt();
                StartOptionAnimations_Exit();
                HideMainOrSubMenuLeftHeader(1u32, 0u32);
                return 0u32;
            }
            if __sw1 == 1i32 {
                if (AreMenuOptionSpritesMoving()) != 0 {
                    return 2u32;
                }
                if (AreLeftHeaderSpritesMoving()) != 0 {
                    return 2u32;
                }
                DrawCurrentMenuOptionLabels();
                LoadLeftHeaderGfxForIndex(0u32);
                return 0u32;
            }
            if __sw1 == 2i32 {
                StartOptionAnimations_Enter();
                ShowLeftHeaderGfx(0u32, 0u32, 0u32);
                CreateBgDotLightBluePalTask();
                PrintCurrentOptionDescription();
                return 0u32;
            }
            if __sw1 == 3i32 {
                if (AreMenuOptionSpritesMoving()) != 0 {
                    return 2u32;
                }
                if (AreLeftHeaderSpritesMoving()) != 0 {
                    return 2u32;
                }
                if (IsTaskActive_UpdateBgDotsPalette()) != 0 {
                    return 2u32;
                }
                if (IsDma3ManagerBusyWithBgCopy_()) != 0 {
                    return 2u32;
                }
                InitMenuOptionGlow();
                break 'l1;
            }
        }
        return 4u32;
    }
}
pub(crate) unsafe extern "C" fn LoopedTask_OpenConditionSearchMenu(state: i32) -> u32 {
    unsafe {
        let mut state = state;
        'l1: {
            let __sw1 = state;
            if __sw1 == 0i32 {
                ResetBldCnt();
                StartOptionAnimations_Exit();
                PlaySE(5u16);
                return 0u32;
            }
            if __sw1 == 1i32 {
                if (AreMenuOptionSpritesMoving()) != 0 {
                    return 2u32;
                }
                LoadLeftHeaderGfxForIndex(7u32);
                DrawCurrentMenuOptionLabels();
                return 0u32;
            }
            if __sw1 == 2i32 {
                StartOptionAnimations_Enter();
                ShowLeftHeaderGfx(7u32, 0u32, 0u32);
                PrintCurrentOptionDescription();
                return 0u32;
            }
            if __sw1 == 3i32 {
                if (AreMenuOptionSpritesMoving()) != 0 {
                    return 2u32;
                }
                if (AreLeftHeaderSpritesMoving()) != 0 {
                    return 2u32;
                }
                if (IsTaskActive_UpdateBgDotsPalette()) != 0 {
                    return 2u32;
                }
                InitMenuOptionGlow();
                break 'l1;
            }
        }
        return 4u32;
    }
}
pub(crate) unsafe extern "C" fn LoopedTask_ReturnToConditionMenu(state: i32) -> u32 {
    unsafe {
        let mut state = state;
        'l1: {
            let __sw1 = state;
            if __sw1 == 0i32 {
                ResetBldCnt();
                StartOptionAnimations_Exit();
                HideMainOrSubMenuLeftHeader(7u32, 0u32);
                return 0u32;
            }
            if __sw1 == 1i32 {
                if (AreMenuOptionSpritesMoving()) != 0 {
                    return 2u32;
                }
                if (AreLeftHeaderSpritesMoving()) != 0 {
                    return 2u32;
                }
                DrawCurrentMenuOptionLabels();
                return 0u32;
            }
            if __sw1 == 2i32 {
                StartOptionAnimations_Enter();
                PrintCurrentOptionDescription();
                return 0u32;
            }
            if __sw1 == 3i32 {
                if (AreMenuOptionSpritesMoving()) != 0 {
                    return 2u32;
                }
                if (IsTaskActive_UpdateBgDotsPalette()) != 0 {
                    return 2u32;
                }
                InitMenuOptionGlow();
                break 'l1;
            }
        }
        return 4u32;
    }
}
pub(crate) unsafe extern "C" fn LoopedTask_SelectRibbonsNoWinners(state: i32) -> u32 {
    unsafe {
        let mut state = state;
        'l1: {
            let __sw1 = state;
            if __sw1 == 0i32 {
                PlaySE(32u16);
                PrintNoRibbonWinners();
                return 0u32;
            }
            if __sw1 == 1i32 {
                if (IsDma3ManagerBusyWithBgCopy()) != 0 {
                    return 2u32;
                }
                break 'l1;
            }
        }
        return 4u32;
    }
}
pub(crate) unsafe extern "C" fn LoopedTask_ReShowDescription(state: i32) -> u32 {
    unsafe {
        let mut state = state;
        'l1: {
            let __sw1 = state;
            if __sw1 == 0i32 {
                PlaySE(5u16);
                PrintCurrentOptionDescription();
                return 0u32;
            }
            if __sw1 == 1i32 {
                if (IsDma3ManagerBusyWithBgCopy()) != 0 {
                    return 2u32;
                }
                break 'l1;
            }
        }
        return 4u32;
    }
}
pub(crate) unsafe extern "C" fn LoopedTask_OpenPokenavFeature(state: i32) -> u32 {
    unsafe {
        let mut state = state;
        'l1: {
            let __sw1 = state;
            if __sw1 == 0i32 {
                PrintHelpBarText(((GetHelpBarTextId()) as u32));
                return 0u32;
            }
            if __sw1 == 1i32 {
                if (WaitForHelpBar()) != 0 {
                    return 2u32;
                }
                SlideMenuHeaderUp();
                ResetBldCnt();
                StartOptionAnimations_Exit();
                'l2: {
                    let __sw2 = GetPokenavMenuType();
                    let __matched = __sw2 == 4i32 || __sw2 == 3i32;
                    let mut __fall = false;
                    if __sw2 == 4i32 {
                        __fall = true;
                        HideMainOrSubMenuLeftHeader(7u32, 0u32);
                    }
                    if __fall || __sw2 == 3i32 {
                        __fall = true;
                        HideMainOrSubMenuLeftHeader(1u32, 0u32);
                        break 'l2;
                    }
                    if !__matched {
                        __fall = true;
                        HideMainOrSubMenuLeftHeader(0u32, 0u32);
                        break 'l2;
                    }
                }
                PlaySE(5u16);
                return 0u32;
            }
            if __sw1 == 2i32 {
                if (AreMenuOptionSpritesMoving()) != 0 {
                    return 2u32;
                }
                if (AreLeftHeaderSpritesMoving()) != 0 {
                    return 2u32;
                }
                PokenavFadeScreen(0i32);
                return 0u32;
            }
            if __sw1 == 3i32 {
                if (IsPaletteFadeActive()) != 0 {
                    return 2u32;
                }
                break 'l1;
            }
        }
        return 4u32;
    }
}
pub(crate) unsafe extern "C" fn LoadPokenavOptionPalettes() {
    unsafe {
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(((i) as u32) < crate::c::div_u32(16u32, 8u32)) {
                    break 'l1;
                }
                'l2: {
                    LoadCompressedSpriteSheet(
                        (((&raw const sPokenavOptionsSpriteSheets)
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
            ((&raw const sPokenavOptionsSpritePalettes)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>(),
        );
    }
}
pub(crate) unsafe extern "C" fn FreeAndDestroyMainMenuSprites() {
    unsafe {
        FreeSpriteTilesByTag(3u16);
        FreeSpriteTilesByTag(1u16);
        FreeSpritePaletteByTag(4u16);
        FreeSpritePaletteByTag(5u16);
        FreeSpritePaletteByTag(6u16);
        FreeSpritePaletteByTag(7u16);
        FreeSpritePaletteByTag(8u16);
        FreeSpritePaletteByTag(3u16);
        DestroyMenuOptionSprites();
        DestroyRematchBlueLightSprite();
    }
}
pub(crate) unsafe extern "C" fn CreateMenuOptionSprites() {
    unsafe {
        let mut i: i32 = 0i32;
        let mut j: i32 = 0i32;
        let mut gfx: *mut u8 = GetSubstructPtr(2u32);
        {
            i = 0i32;
            'l1: loop {
                if !(i < 6i32) {
                    break 'l1;
                }
                'l2: {
                    {
                        j = 0i32;
                        'l3: loop {
                            if !(j < 4i32) {
                                break 'l3;
                            }
                            'l4: {
                                let mut spriteId: u8 = CreateSprite(
                                    (&raw const sMenuOptionSpriteTemplate)
                                        .cast::<u8>()
                                        .cast_mut(),
                                    140i16,
                                    ((((20i32).wrapping_mul(i)).wrapping_add(40i32)) as i16),
                                    3u8,
                                );
                                ((((((gfx).wrapping_add(44)).cast::<u8>())
                                    .wrapping_offset((i) as isize * 16))
                                .cast::<*mut u8>())
                                .wrapping_offset((j) as isize))
                                .write(
                                    ((&raw mut gSprites).cast::<u8>())
                                        .wrapping_offset(((spriteId) as i32) as isize * 68),
                                );
                                ((((&raw mut gSprites).cast::<u8>())
                                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                                .wrapping_add(36)
                                .cast::<i16>())
                                .write((((32i32).wrapping_mul(j)) as i16));
                            }
                            j = (j).wrapping_add(1);
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn DestroyMenuOptionSprites() {
    unsafe {
        let mut i: i32 = 0i32;
        let mut j: i32 = 0i32;
        let mut gfx: *mut u8 = GetSubstructPtr(2u32);
        {
            i = 0i32;
            'l1: loop {
                if !(i < 6i32) {
                    break 'l1;
                }
                'l2: {
                    {
                        j = 0i32;
                        'l3: loop {
                            if !(j < 4i32) {
                                break 'l3;
                            }
                            'l4: {
                                FreeSpriteOamMatrix(
                                    ((((((gfx).wrapping_add(44)).cast::<u8>())
                                        .wrapping_offset((i) as isize * 16))
                                    .cast::<*mut u8>())
                                    .wrapping_offset((j) as isize))
                                    .read(),
                                );
                                DestroySprite(
                                    ((((((gfx).wrapping_add(44)).cast::<u8>())
                                        .wrapping_offset((i) as isize * 16))
                                    .cast::<*mut u8>())
                                    .wrapping_offset((j) as isize))
                                    .read(),
                                );
                            }
                            j = (j).wrapping_add(1);
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn DrawCurrentMenuOptionLabels() {
    unsafe {
        let mut menuType: i32 = GetPokenavMenuType();
        DrawOptionLabelGfx(
            (((((&raw const sPokenavMenuOptionLabelGfx)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>())
            .wrapping_offset((menuType) as isize * 28))
            .wrapping_add(4))
            .cast::<*mut u16>(),
            (((((((&raw const sPokenavMenuOptionLabelGfx)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>())
            .wrapping_offset((menuType) as isize * 28))
            .cast::<u16>())
            .read()) as i32),
            (((((((&raw const sPokenavMenuOptionLabelGfx)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>())
            .wrapping_offset((menuType) as isize * 28))
            .wrapping_add(2)
            .cast::<u16>())
            .read()) as i32),
        );
    }
}
pub(crate) unsafe extern "C" fn DrawOptionLabelGfx(
    optionGfx: *mut *mut u16,
    yPos: i32,
    deltaY: i32,
) {
    unsafe {
        let mut optionGfx = optionGfx;
        let mut yPos = yPos;
        let mut deltaY = deltaY;
        let mut i: i32 = 0i32;
        let mut j: i32 = 0i32;
        let mut gfx: *mut u8 = GetSubstructPtr(2u32);
        let mut baseTile: i32 = ((GetSpriteTileStartByTag(3u16)) as i32);
        {
            i = 0i32;
            'l1: loop {
                if !(i < 6i32) {
                    break 'l1;
                }
                'l2: {
                    if (((optionGfx).read()) as usize) != 0usize {
                        {
                            j = 0i32;
                            'l3: loop {
                                if !(j < 4i32) {
                                    break 'l3;
                                }
                                'l4: {
                                    crate::c::bf_write(
                                        (((((((gfx).wrapping_add(44)).cast::<u8>())
                                            .wrapping_offset((i) as isize * 16))
                                        .cast::<*mut u8>())
                                        .wrapping_offset((j) as isize))
                                        .read())
                                        .wrapping_add(4),
                                        0,
                                        10,
                                        (((((((optionGfx).read()).read()) as i32)
                                            .wrapping_add(baseTile))
                                        .wrapping_add((8i32).wrapping_mul(j)))
                                            as u16) as i32,
                                    );
                                    crate::c::bf_write(
                                        (((((((gfx).wrapping_add(44)).cast::<u8>())
                                            .wrapping_offset((i) as isize * 16))
                                        .cast::<*mut u8>())
                                        .wrapping_offset((j) as isize))
                                        .read())
                                        .wrapping_add(5),
                                        4,
                                        4,
                                        ((IndexOfSpritePaletteTag(
                                            (((((((optionGfx).read()).wrapping_offset(1)).read())
                                                as i32)
                                                .wrapping_add(4i32))
                                                as u16),
                                        )) as u16) as i32,
                                    );
                                    crate::c::bf_write(
                                        (((((((gfx).wrapping_add(44)).cast::<u8>())
                                            .wrapping_offset((i) as isize * 16))
                                        .cast::<*mut u8>())
                                        .wrapping_offset((j) as isize))
                                        .read())
                                        .wrapping_add(62),
                                        2,
                                        1,
                                        (1u16) as i32,
                                    );
                                    ((((((((gfx).wrapping_add(44)).cast::<u8>())
                                        .wrapping_offset((i) as isize * 16))
                                    .cast::<*mut u8>())
                                    .wrapping_offset((j) as isize))
                                    .read())
                                    .wrapping_add(34)
                                    .cast::<i16>())
                                    .write(((yPos) as i16));
                                    ((((((((gfx).wrapping_add(44)).cast::<u8>())
                                        .wrapping_offset((i) as isize * 16))
                                    .cast::<*mut u8>())
                                    .wrapping_offset((j) as isize))
                                    .read())
                                    .wrapping_add(32)
                                    .cast::<i16>())
                                    .write(140i16);
                                    ((((((((gfx).wrapping_add(44)).cast::<u8>())
                                        .wrapping_offset((i) as isize * 16))
                                    .cast::<*mut u8>())
                                    .wrapping_offset((j) as isize))
                                    .read())
                                    .wrapping_add(36)
                                    .cast::<i16>())
                                    .write((((32i32).wrapping_mul(j)) as i16));
                                }
                                j = (j).wrapping_add(1);
                            }
                        }
                        ((((gfx).wrapping_add(16)).cast::<u32>()).wrapping_offset((i) as isize))
                            .write(1u32);
                    } else {
                        {
                            j = 0i32;
                            'l5: loop {
                                if !(j < 4i32) {
                                    break 'l5;
                                }
                                'l6: {
                                    crate::c::bf_write(
                                        (((((((gfx).wrapping_add(44)).cast::<u8>())
                                            .wrapping_offset((i) as isize * 16))
                                        .cast::<*mut u8>())
                                        .wrapping_offset((j) as isize))
                                        .read())
                                        .wrapping_add(62),
                                        2,
                                        1,
                                        (1u16) as i32,
                                    );
                                }
                                j = (j).wrapping_add(1);
                            }
                        }
                        ((((gfx).wrapping_add(16)).cast::<u32>()).wrapping_offset((i) as isize))
                            .write(0u32);
                    }
                    optionGfx = (optionGfx).wrapping_offset(1);
                    yPos = (yPos).wrapping_add(deltaY);
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn StartOptionAnimations_Enter() {
    unsafe {
        let mut i: i32 = 0i32;
        let mut gfx: *mut u8 = GetSubstructPtr(2u32);
        let mut cursorPos: i32 = GetPokenavCursorPos();
        let mut iconCount: i32 = 0i32;
        let mut x: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 6i32) {
                    break 'l1;
                }
                'l2: {
                    if (((((gfx).wrapping_add(16)).cast::<u32>()).wrapping_offset((i) as isize))
                        .read())
                        != 0
                    {
                        if {
                            let __t1 = iconCount;
                            iconCount = (iconCount).wrapping_add(1);
                            __t1
                        } == cursorPos
                        {
                            x = 130i32;
                            ((gfx).wrapping_add(11)).write(((i) as u8));
                        } else {
                            x = 140i32;
                        }
                        StartOptionSlide(
                            ((((gfx).wrapping_add(44)).cast::<u8>())
                                .wrapping_offset((i) as isize * 16))
                            .cast::<*mut u8>(),
                            256i32,
                            x,
                            12i32,
                        );
                        SetOptionInvisibility(
                            ((((gfx).wrapping_add(44)).cast::<u8>())
                                .wrapping_offset((i) as isize * 16))
                            .cast::<*mut u8>(),
                            0u32,
                        );
                    } else {
                        SetOptionInvisibility(
                            ((((gfx).wrapping_add(44)).cast::<u8>())
                                .wrapping_offset((i) as isize * 16))
                            .cast::<*mut u8>(),
                            1u32,
                        );
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn StartOptionAnimations_CursorMoved() {
    unsafe {
        let mut i: i32 = 0i32;
        let mut gfx: *mut u8 = GetSubstructPtr(2u32);
        let mut prevPos: i32 = GetPokenavCursorPos();
        let mut newPos: i32 = 0i32;
        {
            i = 0i32;
            newPos = 0i32;
            'l1: loop {
                if !(i < 6i32) {
                    break 'l1;
                }
                'l2: {
                    if (((((gfx).wrapping_add(16)).cast::<u32>()).wrapping_offset((i) as isize))
                        .read())
                        != 0
                    {
                        if newPos == prevPos {
                            newPos = i;
                            break 'l1;
                        }
                        newPos = (newPos).wrapping_add(1);
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        StartOptionSlide(
            ((((gfx).wrapping_add(44)).cast::<u8>())
                .wrapping_offset(((((gfx).wrapping_add(11)).read()) as i32) as isize * 16))
            .cast::<*mut u8>(),
            130i32,
            140i32,
            4i32,
        );
        StartOptionSlide(
            ((((gfx).wrapping_add(44)).cast::<u8>()).wrapping_offset((newPos) as isize * 16))
                .cast::<*mut u8>(),
            140i32,
            130i32,
            4i32,
        );
        ((gfx).wrapping_add(11)).write(((newPos) as u8));
    }
}
pub(crate) unsafe extern "C" fn StartOptionAnimations_Exit() {
    unsafe {
        let mut i: i32 = 0i32;
        let mut gfx: *mut u8 = GetSubstructPtr(2u32);
        {
            i = 0i32;
            'l1: loop {
                if !(i < 6i32) {
                    break 'l1;
                }
                'l2: {
                    if (((((gfx).wrapping_add(16)).cast::<u32>()).wrapping_offset((i) as isize))
                        .read())
                        != 0
                    {
                        if ((((gfx).wrapping_add(11)).read()) as i32) != i {
                            StartOptionSlide(
                                ((((gfx).wrapping_add(44)).cast::<u8>())
                                    .wrapping_offset((i) as isize * 16))
                                .cast::<*mut u8>(),
                                140i32,
                                256i32,
                                8i32,
                            );
                        } else {
                            StartOptionZoom(
                                ((((gfx).wrapping_add(44)).cast::<u8>())
                                    .wrapping_offset((i) as isize * 16))
                                .cast::<*mut u8>(),
                            );
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn AreMenuOptionSpritesMoving() -> u32 {
    unsafe {
        let mut i: i32 = 0i32;
        let mut gfx: *mut u8 = GetSubstructPtr(2u32);
        {
            i = 0i32;
            'l1: loop {
                if !(i < 6i32) {
                    break 'l1;
                }
                'l2: {
                    if core::mem::transmute::<_, usize>(
                        (((((((gfx).wrapping_add(44)).cast::<u8>())
                            .wrapping_offset((i) as isize * 16))
                        .cast::<*mut u8>())
                        .read())
                        .wrapping_add(28)
                        .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                        .read(),
                    ) != (SpriteCallbackDummy as *const () as usize)
                    {
                        return 1u32;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        if ((((gfx).wrapping_add(12)).read()) as i32) != 0i32 {
            return 1u32;
        }
        return 0u32;
    }
}
pub(crate) unsafe extern "C" fn StartOptionSlide(
    sprites: *mut *mut u8,
    startX: i32,
    endX: i32,
    time: i32,
) {
    unsafe {
        let mut sprites = sprites;
        let mut startX = startX;
        let mut endX = endX;
        let mut time = time;
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 4i32) {
                    break 'l1;
                }
                'l2: {
                    (((sprites).read()).wrapping_add(32).cast::<i16>()).write(((startX) as i16));
                    ((((sprites).read()).wrapping_add(46)).cast::<i16>()).write(((time) as i16));
                    (((((sprites).read()).wrapping_add(46)).cast::<i16>()).wrapping_offset(1))
                        .write(
                            ((crate::c::div_i32(
                                (16i32).wrapping_mul((endX).wrapping_sub(startX)),
                                time,
                            )) as i16),
                        );
                    (((((sprites).read()).wrapping_add(46)).cast::<i16>()).wrapping_offset(2))
                        .write((((16i32).wrapping_mul(startX)) as i16));
                    (((((sprites).read()).wrapping_add(46)).cast::<i16>()).wrapping_offset(7))
                        .write(((endX) as i16));
                    (((sprites).read())
                        .wrapping_add(28)
                        .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                    .write(Some(SpriteCB_OptionSlide));
                    sprites = (sprites).wrapping_offset(1);
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn StartOptionZoom(sprites: *mut *mut u8) {
    unsafe {
        let mut sprites = sprites;
        let mut i: i32 = 0i32;
        let mut gfx: *mut u8 = GetSubstructPtr(2u32);
        let mut taskId: u8 = 0u8;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 4i32) {
                    break 'l1;
                }
                'l2: {
                    crate::c::bf_write(((sprites).read()).wrapping_add(1), 2, 2, (1u32) as i32);
                    crate::c::bf_write(((sprites).read()).wrapping_add(1), 0, 2, (3u32) as i32);
                    (((sprites).read())
                        .wrapping_add(28)
                        .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                    .write(Some(SpriteCB_OptionZoom));
                    ((((sprites).read()).wrapping_add(46)).cast::<i16>()).write(8i16);
                    (((((sprites).read()).wrapping_add(46)).cast::<i16>()).wrapping_offset(1))
                        .write(0i16);
                    (((((sprites).read()).wrapping_add(46)).cast::<i16>()).wrapping_offset(7))
                        .write(((i) as i16));
                    InitSpriteAffineAnim((sprites).read());
                    StartSpriteAffineAnim((sprites).read(), 0u8);
                    sprites = (sprites).wrapping_offset(1);
                }
                i = (i).wrapping_add(1);
            }
        }
        SetGpuReg(82u8, 16u16);
        taskId = CreateTask(Some(Task_OptionBlend), 3u8);
        (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .write(8i16);
        let __p1 = (gfx).wrapping_add(12);
        (__p1).write(((__p1).read()).wrapping_add(1));
    }
}
pub(crate) unsafe extern "C" fn SetOptionInvisibility(sprites: *mut *mut u8, invisible: u32) {
    unsafe {
        let mut sprites = sprites;
        let mut invisible = invisible;
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 4i32) {
                    break 'l1;
                }
                'l2: {
                    crate::c::bf_write(
                        ((sprites).read()).wrapping_add(62),
                        2,
                        1,
                        ((invisible) as u16) as i32,
                    );
                    sprites = (sprites).wrapping_offset(1);
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_OptionSlide(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let __p1 = ((sprite).wrapping_add(46)).cast::<i16>();
        (__p1).write(((__p1).read()).wrapping_sub(1));
        if (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) != (-1i32) {
            let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
            (__p2).write(
                (((((__p2).read()) as i32).wrapping_add(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                        as i32),
                )) as i16),
            );
            ((sprite).wrapping_add(32).cast::<i16>()).write(
                ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                    as i32)
                    >> 4) as i16),
            );
        } else {
            ((sprite).wrapping_add(32).cast::<i16>())
                .write(((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read());
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(SpriteCallbackDummy));
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_OptionZoom(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut temp: i32 = 0i32;
        let mut x: i32 = 0i32;
        if (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) == 0i32 {
            if !((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) != 0) {
                StartSpriteAffineAnim(sprite, 1u8);
                let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
                (__p1).write(((__p1).read()).wrapping_add(1));
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(256i16);
                let __p2 = (sprite).wrapping_add(32).cast::<i16>();
                (__p2).write(
                    (((((__p2).read()) as i32)
                        .wrapping_add(((((sprite).wrapping_add(36).cast::<i16>()).read()) as i32)))
                        as i16),
                );
                ((sprite).wrapping_add(36).cast::<i16>()).write(0i16);
            } else {
                let __p3 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
                (__p3).write((((((__p3).read()) as i32).wrapping_add(16i32)) as i16));
                temp = ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                    as i32);
                x = (temp >> 3);
                x = crate::c::div_i32((x).wrapping_sub(32i32), 2i32);
                'l1: {
                    let __sw4 = ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7))
                        .read()) as i32);
                    if __sw4 == 0i32 {
                        ((sprite).wrapping_add(36).cast::<i16>())
                            .write(((((x).wrapping_neg()).wrapping_mul(3i32)) as i16));
                        break 'l1;
                    }
                    if __sw4 == 1i32 {
                        ((sprite).wrapping_add(36).cast::<i16>())
                            .write((((x).wrapping_neg()) as i16));
                        break 'l1;
                    }
                    if __sw4 == 2i32 {
                        ((sprite).wrapping_add(36).cast::<i16>()).write(((x) as i16));
                        break 'l1;
                    }
                    if __sw4 == 3i32 {
                        ((sprite).wrapping_add(36).cast::<i16>())
                            .write((((x).wrapping_mul(3i32)) as i16));
                        break 'l1;
                    }
                }
                if (crate::c::bf_read((sprite).wrapping_add(63), 5, 1, false) as u16) != 0 {
                    crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (1u16) as i32);
                    FreeOamMatrix(
                        ((crate::c::bf_read((sprite).wrapping_add(3), 1, 5, false) as u32) as u8),
                    );
                    CalcCenterToCornerVec(
                        sprite,
                        ((crate::c::bf_read((sprite).wrapping_add(1), 6, 2, false) as u32) as u8),
                        ((crate::c::bf_read((sprite).wrapping_add(3), 6, 2, false) as u32) as u8),
                        0u8,
                    );
                    crate::c::bf_write((sprite).wrapping_add(1), 0, 2, (0u32) as i32);
                    crate::c::bf_write((sprite).wrapping_add(1), 2, 2, (0u32) as i32);
                    ((sprite)
                        .wrapping_add(28)
                        .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                    .write(Some(SpriteCallbackDummy));
                }
            }
        } else {
            let __p5 = ((sprite).wrapping_add(46)).cast::<i16>();
            (__p5).write(((__p5).read()).wrapping_sub(1));
        }
    }
}
pub(crate) unsafe extern "C" fn Task_OptionBlend(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        if (((data).read()) as i32) == 0i32 {
            'l1: {
                let __sw1 = ((((data).wrapping_offset(1)).read()) as i32);
                if __sw1 == 0i32 {
                    ((data).wrapping_offset(2)).write(16i16);
                    ((data).wrapping_offset(3)).write(0i16);
                    SetGpuReg(80u8, 16128u16);
                    SetGpuReg(82u8, 16u16);
                    let __p2 = (data).wrapping_offset(1);
                    (__p2).write(((__p2).read()).wrapping_add(1));
                    break 'l1;
                }
                if __sw1 == 1i32 {
                    if (((((data).wrapping_offset(4)).read()) as i32) & 1i32) != 0 {
                        let __p3 = (data).wrapping_offset(2);
                        (__p3).write((((((__p3).read()) as i32).wrapping_sub(3i32)) as i16));
                        if ((((data).wrapping_offset(2)).read()) as i32) < 0i32 {
                            ((data).wrapping_offset(2)).write(0i16);
                        }
                    } else {
                        let __p4 = (data).wrapping_offset(3);
                        (__p4).write((((((__p4).read()) as i32).wrapping_add(3i32)) as i16));
                        if ((((data).wrapping_offset(3)).read()) as i32) > 16i32 {
                            ((data).wrapping_offset(3)).write(16i16);
                        }
                    }
                    SetGpuReg(
                        82u8,
                        (((((((data).wrapping_offset(3)).read()) as i32) << 8)
                            | ((((data).wrapping_offset(2)).read()) as i32))
                            as u16),
                    );
                    let __p5 = (data).wrapping_offset(4);
                    (__p5).write(((__p5).read()).wrapping_add(1));
                    if ((((data).wrapping_offset(4)).read()) as i32) == 12i32 {
                        let __p6 = (GetSubstructPtr(2u32)).wrapping_add(12);
                        (__p6).write(((__p6).read()).wrapping_sub(1));
                        SetGpuReg(82u8, 4096u16);
                        DestroyTask(taskId);
                    }
                    break 'l1;
                }
            }
        } else {
            (data).write(((data).read()).wrapping_sub(1));
        }
    }
}
pub(crate) unsafe extern "C" fn CreateMatchCallBlueLightSprite() {
    unsafe {
        let mut gfx: *mut u8 = GetSubstructPtr(2u32);
        let mut spriteId: u8 = CreateSprite(
            (&raw const sMatchCallBlueLightSpriteTemplate)
                .cast::<u8>()
                .cast_mut(),
            16i16,
            96i16,
            4u8,
        );
        ((gfx).wrapping_add(40).cast::<*mut u8>()).write(
            ((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68),
        );
        if (AreAnyTrainerRematchesNearby()) != 0 {
            ((((gfx).wrapping_add(40).cast::<*mut u8>()).read())
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(SpriteCB_BlinkingBlueLight));
        } else {
            crate::c::bf_write(
                (((gfx).wrapping_add(40).cast::<*mut u8>()).read()).wrapping_add(62),
                2,
                1,
                (1u16) as i32,
            );
        }
    }
}
pub(crate) unsafe extern "C" fn DestroyRematchBlueLightSprite() {
    unsafe {
        let mut gfx: *mut u8 = GetSubstructPtr(2u32);
        DestroySprite(((gfx).wrapping_add(40).cast::<*mut u8>()).read());
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_BlinkingBlueLight(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let __p1 = ((sprite).wrapping_add(46)).cast::<i16>();
        (__p1).write(((__p1).read()).wrapping_add(1));
        if (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) > 8i32 {
            (((sprite).wrapping_add(46)).cast::<i16>()).write(0i16);
            crate::c::bf_write(
                (sprite).wrapping_add(62),
                2,
                1,
                ((((crate::c::bf_read((sprite).wrapping_add(62), 2, 1, false) as u16) as i32)
                    ^ 1i32) as u16) as i32,
            );
        }
    }
}
pub(crate) unsafe extern "C" fn AddOptionDescriptionWindow() {
    unsafe {
        let mut gfx: *mut u8 = GetSubstructPtr(2u32);
        ((gfx).wrapping_add(8).cast::<u16>()).write(AddWindow(
            (&raw const sOptionDescWindowTemplate)
                .cast::<u8>()
                .cast_mut(),
        ));
        PutWindowTilemap(((((gfx).wrapping_add(8).cast::<u16>()).read()) as u8));
        FillWindowPixelBuffer(
            ((((gfx).wrapping_add(8).cast::<u16>()).read()) as u8),
            102u8,
        );
        CopyWindowToVram(((((gfx).wrapping_add(8).cast::<u16>()).read()) as u8), 3u8);
    }
}
pub(crate) unsafe extern "C" fn PrintCurrentOptionDescription() {
    unsafe {
        let mut gfx: *mut u8 = GetSubstructPtr(2u32);
        let mut menuItem: i32 = GetCurrentMenuItemId();
        let mut desc: *mut u8 = ((((&raw const sPageDescriptions)
            .cast::<u8>()
            .cast_mut()
            .cast::<*mut u8>())
        .cast::<*mut u8>())
        .wrapping_offset((menuItem) as isize))
        .read();
        let mut width: u32 = ((GetStringWidth(1u8, desc, (-1i16))) as u32);
        FillWindowPixelBuffer(
            ((((gfx).wrapping_add(8).cast::<u16>()).read()) as u8),
            102u8,
        );
        AddTextPrinterParameterized3(
            ((((gfx).wrapping_add(8).cast::<u16>()).read()) as u8),
            1u8,
            ((crate::c::div_u32((192u32).wrapping_sub(width), 2u32)) as u8),
            1u8,
            ((&raw const sOptionDescTextColors).cast::<u8>().cast_mut()).cast::<u8>(),
            0i8,
            desc,
        );
    }
}
pub(crate) unsafe extern "C" fn PrintNoRibbonWinners() {
    unsafe {
        let mut gfx: *mut u8 = GetSubstructPtr(2u32);
        let mut s: *mut u8 = (&raw mut gText_NoRibbonWinners).cast::<u8>();
        let mut width: u32 = ((GetStringWidth(1u8, s, (-1i16))) as u32);
        FillWindowPixelBuffer(
            ((((gfx).wrapping_add(8).cast::<u16>()).read()) as u8),
            102u8,
        );
        AddTextPrinterParameterized3(
            ((((gfx).wrapping_add(8).cast::<u16>()).read()) as u8),
            1u8,
            ((crate::c::div_u32((192u32).wrapping_sub(width), 2u32)) as u8),
            1u8,
            ((&raw const sOptionDescTextColors2).cast::<u8>().cast_mut()).cast::<u8>(),
            0i8,
            s,
        );
    }
}
pub(crate) unsafe extern "C" fn IsDma3ManagerBusyWithBgCopy_() -> u32 {
    unsafe {
        return ((IsDma3ManagerBusyWithBgCopy()) as u32);
    }
}
pub(crate) unsafe extern "C" fn CreateMovingBgDotsTask() {
    unsafe {
        let mut gfx: *mut u8 = GetSubstructPtr(2u32);
        ((gfx).wrapping_add(10)).write(CreateTask(Some(Task_MoveBgDots), 2u8));
    }
}
pub(crate) unsafe extern "C" fn DestroyMovingDotsBgTask() {
    unsafe {
        let mut gfx: *mut u8 = GetSubstructPtr(2u32);
        DestroyTask(((gfx).wrapping_add(10)).read());
    }
}
pub(crate) unsafe extern "C" fn Task_MoveBgDots(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        ChangeBgX(3u8, 128i32, 1u8);
    }
}
pub(crate) unsafe extern "C" fn CreateBgDotPurplePalTask() {
    unsafe {
        let mut taskId: u8 = CreateTask(Some(Task_UpdateBgDotsPalette), 3u8);
        SetWordTaskArg(
            taskId,
            1u8,
            (((((&raw const sPokenavBgDotsPal)
                .cast::<u8>()
                .cast_mut()
                .cast::<u16>())
            .cast::<u16>())
            .wrapping_offset(1)) as usize as u32),
        );
        SetWordTaskArg(
            taskId,
            3u8,
            (((((&raw const sPokenavBgDotsPal)
                .cast::<u8>()
                .cast_mut()
                .cast::<u16>())
            .cast::<u16>())
            .wrapping_offset(7)) as usize as u32),
        );
    }
}
pub(crate) unsafe extern "C" fn ChangeBgDotsColorToPurple() {
    unsafe {
        CopyPaletteIntoBufferUnfaded(
            (((&raw const sPokenavBgDotsPal)
                .cast::<u8>()
                .cast_mut()
                .cast::<u16>())
            .cast::<u16>())
            .wrapping_offset(7),
            49u32,
            4u32,
        );
    }
}
pub(crate) unsafe extern "C" fn CreateBgDotLightBluePalTask() {
    unsafe {
        let mut taskId: u8 = CreateTask(Some(Task_UpdateBgDotsPalette), 3u8);
        SetWordTaskArg(
            taskId,
            1u8,
            (((((&raw const sPokenavBgDotsPal)
                .cast::<u8>()
                .cast_mut()
                .cast::<u16>())
            .cast::<u16>())
            .wrapping_offset(7)) as usize as u32),
        );
        SetWordTaskArg(
            taskId,
            3u8,
            (((((&raw const sPokenavBgDotsPal)
                .cast::<u8>()
                .cast_mut()
                .cast::<u16>())
            .cast::<u16>())
            .wrapping_offset(1)) as usize as u32),
        );
    }
}
pub(crate) unsafe extern "C" fn IsTaskActive_UpdateBgDotsPalette() -> u32 {
    unsafe {
        return ((FuncIsActiveTask(Some(Task_UpdateBgDotsPalette))) as u32);
    }
}
pub(crate) unsafe extern "C" fn Task_UpdateBgDotsPalette(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut sp8 = crate::ffi::Align4([0u8; 4]);
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        let mut pal1: *mut u16 = ((GetWordTaskArg(taskId, 1u8)) as usize as *mut u16);
        let mut pal2: *mut u16 = ((GetWordTaskArg(taskId, 3u8)) as usize as *mut u16);
        PokenavCopyPalette(
            pal1,
            pal2,
            2i32,
            12i32,
            (({
                let __t1 = ((data).read()).wrapping_add(1);
                (data).write(__t1);
                __t1
            }) as i32),
            (&raw mut sp8).cast::<u16>(),
        );
        LoadPalette(((&raw mut sp8).cast::<u16>()).cast::<u8>(), 49u16, 4u16);
        if (((data).read()) as i32) == 12i32 {
            DestroyTask(taskId);
        }
    }
}
pub(crate) unsafe extern "C" fn VBlankCB_PokenavMainMenu() {
    unsafe {
        TransferPlttBuffer();
        LoadOam();
        ProcessSpriteCopyRequests();
        ScanlineEffect_InitHBlankDmaTransfer();
    }
}
pub(crate) unsafe extern "C" fn SetupPokenavMenuScanlineEffects() {
    unsafe {
        SetGpuReg(80u8, 144u16);
        SetGpuReg(84u8, 0u16);
        SetGpuRegBits(0u8, 8192u16);
        SetGpuRegBits(72u8, 63u16);
        SetGpuRegBits(74u8, 31u16);
        SetGpuRegBits(68u8, 160u16);
        ScanlineEffect_Stop();
        SetMenuOptionGlow();
        ScanlineEffect_SetParams(
            (&raw const sPokenavMainMenuScanlineEffectParams)
                .cast::<u8>()
                .cast_mut()
                .cast::<crate::c::Rec4<12>>()
                .read_unaligned(),
        );
        SetVBlankCallback_(Some(VBlankCB_PokenavMainMenu));
        CreateTask(Some(Task_CurrentMenuOptionGlow), 3u8);
    }
}
pub(crate) unsafe extern "C" fn DestroyMenuOptionGlowTask() {
    unsafe {
        SetGpuReg(80u8, 0u16);
        ClearGpuRegBits(0u8, 8192u16);
        ScanlineEffect_Stop();
        DestroyTask(FindTaskIdByFunc(Some(Task_CurrentMenuOptionGlow)));
        SetPokenavVBlankCallback();
    }
}
pub(crate) unsafe extern "C" fn ResetBldCnt() {
    unsafe {
        SetGpuReg(80u8, 0u16);
    }
}
pub(crate) unsafe extern "C" fn InitMenuOptionGlow() {
    unsafe {
        SetMenuOptionGlow();
        SetGpuReg(80u8, 144u16);
    }
}
pub(crate) unsafe extern "C" fn Task_CurrentMenuOptionGlow(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        (data).write(((data).read()).wrapping_add(1));
        if (((data).read()) as i32) > 0i32 {
            (data).write(0i16);
            let __p1 = (data).wrapping_offset(1);
            (__p1).write((((((__p1).read()) as i32).wrapping_add(3i32)) as i16));
            let __p2 = (data).wrapping_offset(1);
            (__p2).write((((((__p2).read()) as i32) & 127i32) as i16));
            SetGpuReg(
                84u8,
                ((((((((&raw mut gSineTable).cast::<i16>()).cast::<i16>())
                    .wrapping_offset(((((data).wrapping_offset(1)).read()) as i32) as isize))
                .read()) as i32)
                    >> 5) as u16),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn SetMenuOptionGlow() {
    unsafe {
        let mut menuType: i32 = GetPokenavMenuType();
        let mut cursorPos: i32 = GetPokenavCursorPos();
        let mut r4: i32 = (((((((((&raw const sPokenavMenuOptionLabelGfx)
            .cast::<u8>()
            .cast_mut())
        .cast::<u8>())
        .wrapping_offset((menuType) as isize * 28))
        .wrapping_add(2)
        .cast::<u16>())
        .read()) as i32)
            .wrapping_mul(cursorPos))
        .wrapping_add(
            (((((((&raw const sPokenavMenuOptionLabelGfx)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>())
            .wrapping_offset((menuType) as isize * 28))
            .cast::<u16>())
            .read()) as i32),
        ))
        .wrapping_sub(8i32);
        'l1: loop {
            'l2: {
                {
                    let mut tmp: u16 = 0u16;
                    (&raw mut tmp).write_volatile(0u16);
                    'l3: loop {
                        'l4: {
                            CpuSet(
                                (&raw mut tmp).cast::<u8>(),
                                (((&raw mut gScanlineEffectRegBuffers).cast::<u8>()).cast::<u16>())
                                    .cast::<u8>(),
                                ((16777216i32
                                    | (crate::c::div_i32(320i32, crate::c::div_i32(16i32, 8i32))
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
        'l5: loop {
            'l6: {
                {
                    let mut tmp: u16 = 0u16;
                    (&raw mut tmp).write_volatile(0u16);
                    'l7: loop {
                        'l8: {
                            CpuSet(
                                (&raw mut tmp).cast::<u8>(),
                                ((((&raw mut gScanlineEffectRegBuffers).cast::<u8>())
                                    .wrapping_offset(1920))
                                .cast::<u16>())
                                .cast::<u8>(),
                                ((16777216i32
                                    | (crate::c::div_i32(320i32, crate::c::div_i32(16i32, 8i32))
                                        & 2097151i32)) as u32),
                            );
                        }
                        if !((0i32) != 0) {
                            break 'l7;
                        }
                    }
                }
            }
            if !((0i32) != 0) {
                break 'l5;
            }
        }
        'l9: loop {
            'l10: {
                {
                    let mut tmp: u16 = 0u16;
                    (&raw mut tmp).write_volatile(29424u16);
                    'l11: loop {
                        'l12: {
                            CpuSet(
                                (&raw mut tmp).cast::<u8>(),
                                ((((&raw mut gScanlineEffectRegBuffers).cast::<u8>())
                                    .cast::<u16>())
                                .wrapping_offset((r4) as isize))
                                .cast::<u8>(),
                                ((16777216i32
                                    | (crate::c::div_i32(32i32, crate::c::div_i32(16i32, 8i32))
                                        & 2097151i32)) as u32),
                            );
                        }
                        if !((0i32) != 0) {
                            break 'l11;
                        }
                    }
                }
            }
            if !((0i32) != 0) {
                break 'l9;
            }
        }
        'l13: loop {
            'l14: {
                {
                    let mut tmp: u16 = 0u16;
                    (&raw mut tmp).write_volatile(29424u16);
                    'l15: loop {
                        'l16: {
                            CpuSet(
                                (&raw mut tmp).cast::<u8>(),
                                (((((&raw mut gScanlineEffectRegBuffers).cast::<u8>())
                                    .wrapping_offset(1920))
                                .cast::<u16>())
                                .wrapping_offset((r4) as isize))
                                .cast::<u8>(),
                                ((16777216i32
                                    | (crate::c::div_i32(32i32, crate::c::div_i32(16i32, 8i32))
                                        & 2097151i32)) as u32),
                            );
                        }
                        if !((0i32) != 0) {
                            break 'l15;
                        }
                    }
                }
            }
            if !((0i32) != 0) {
                break 'l13;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ResetBldCnt_() {
    unsafe {
        ResetBldCnt();
    }
}
