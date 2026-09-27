//! Translated from `src/pokenav_match_call_gfx.c` by tools/rustport/c2rs.py, then reviewed.
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

// Data tables (translate with cdata.py): sMatchCallUI_Pal sMatchCallUI_Gfx sMatchCallUI_Tilemap sOptionsCursor_Pal sOptionsCursor_Gfx sCallWindow_Pal sListWindow_Pal sPokeball_Pal sPokeball_Gfx sMatchCallBgTemplates sMatchCallLoopTaskFuncs sMatchCallLocationWindowTemplate sMatchCallInfoBoxWindowTemplate sMatchCallOptionTexts sText_CallingDots sCallMsgBoxWindowTemplate sOptionsCursorSpriteSheets sOptionsCursorSpritePalettes sOptionsCursorOamData sOptionsCursorSpriteTemplate sTrainerPicOamData sTrainerPicSpriteTemplate
#[allow(unused_imports)]
use crate::data::pokenav_match_call_gfx::*;

unsafe extern "C" {
    static mut gMain: u8;
    static mut gPaletteFade: u8;
    static mut gPlttBufferFaded: u8;
    static mut gPlttBufferUnfaded: u8;
    static mut gSineTable: u8;
    static mut gSprites: u8;
    static mut gTasks: u8;
    static mut gTextFlags: u8;
    static mut gText_NumberOfBattles: u8;
    static mut gText_NumberRegistered: u8;
    static mut gText_TrainerCloseBy: u8;
    static mut gText_Unknown: u8;
    static mut gTrainerFrontPicPaletteTable: u8;
    static mut gTrainerFrontPicTable: u8;
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
    fn AllocSpritePalette(a0: u16) -> u8;
    fn AllocSubstruct(a0: u32, a1: u32) -> *mut u8;
    fn AreLeftHeaderSpritesMoving() -> u32;
    fn BgDmaFill(a0: u32, a1: u8, a2: i32, a3: i32);
    fn BufferMatchCallNameAndDesc(a0: *mut u8, a1: *mut u8);
    fn ChangeBgX(a0: u8, a1: i32, a2: u8) -> i32;
    fn ChangeBgY(a0: u8, a1: i32, a2: u8) -> i32;
    fn CheckForSpaceForDma3Request(a0: i16) -> i16;
    fn ConvertIntToDecimalStringN(a0: *mut u8, a1: i32, a2: i32, a3: u8) -> *mut u8;
    fn CopyBgTilemapBufferToVram(a0: u8);
    fn CopyPaletteIntoBufferUnfaded(a0: *mut u16, a1: u32, a2: u32);
    fn CopyToBgTilemapBuffer(a0: u8, a1: *mut u8, a2: u16, a3: u16);
    fn CopyWindowToVram(a0: u8, a1: u8);
    fn CpuSet(a0: *mut u8, a1: *mut u8, a2: u32);
    fn CreateLoopedTask(a0: Option<unsafe extern "C" fn(i32) -> u32>, a1: u32) -> u32;
    fn CreatePokenavList(a0: *mut u8, a1: *mut u8, a2: u32) -> u32;
    fn CreateSprite(a0: *mut u8, a1: i16, a2: i16, a3: u8) -> u8;
    fn CreateTask(a0: Option<unsafe extern "C" fn(u8)>, a1: u8) -> u8;
    fn DecompressAndCopyTileDataToVram(a0: u8, a1: *mut u8, a2: u32, a3: u16, a4: u8) -> *mut u8;
    fn DecompressPicFromTable(a0: *mut u8, a1: *mut u8, a2: i32);
    fn DestroyPokenavList();
    fn DestroySprite(a0: *mut u8);
    fn DestroyTask(a0: u8);
    fn DrawMatchCallTextBoxBorder(a0: u32, a1: u32, a2: u32);
    fn DrawTextBorderOuter(a0: u8, a1: u16, a2: u8);
    fn FadeToBlackExceptPrimary();
    fn FillBgTilemapBufferRect_Palette0(a0: u8, a1: u16, a2: u8, a3: u8, a4: u8, a5: u8);
    fn FillWindowPixelBuffer(a0: u8, a1: u8);
    fn FindTaskIdByFunc(a0: Option<unsafe extern "C" fn(u8)>) -> u8;
    fn FreePokenavSubstruct(a0: u32);
    fn FreeSpritePaletteByTag(a0: u16);
    fn FreeSpriteTilesByTag(a0: u16);
    fn FreeTempTileDataBuffersIfPossible() -> u8;
    fn GetBgTilemapBuffer(a0: u8) -> *mut u8;
    fn GetGameStat(a0: u8) -> u32;
    fn GetIndexDeltaOfNextCheckPageDown(a0: i32) -> i32;
    fn GetIndexDeltaOfNextCheckPageUp(a0: i32) -> i32;
    fn GetMapName(a0: *mut u8, a1: u16, a2: u16) -> *mut u8;
    fn GetMatchCallList() -> *mut u8;
    fn GetMatchCallMapSec(a0: i32) -> u16;
    fn GetMatchCallMessageText(a0: i32, a1: *mut u8) -> *mut u8;
    fn GetMatchCallOptionCursorPos() -> u16;
    fn GetMatchCallOptionId(a0: i32) -> u16;
    fn GetMatchCallTrainerPic(a0: i32) -> i32;
    fn GetNumberRegistered() -> i32;
    fn GetPlayerTextSpeedDelay() -> u8;
    fn GetSpinningPokenavSprite() -> *mut u8;
    fn GetStringCenterAlignXOffset(a0: i32, a1: *mut u8, a2: i32) -> i32;
    fn GetStringRightAlignXOffset(a0: i32, a1: *mut u8, a2: i32) -> i32;
    fn GetSubstructPtr(a0: u32) -> *mut u8;
    fn GetWindowAttribute(a0: u8, a1: u8) -> u32;
    fn HideSpinningPokenavSprite();
    fn InitBgTemplates(a0: *mut u8, a1: i32);
    fn IsCreatePokenavListTaskActive() -> u32;
    fn IsDma3ManagerBusyWithBgCopy() -> u8;
    fn IsLoopedTaskActive(a0: u32) -> u32;
    fn IsMatchCallListInitFinished() -> i32;
    fn IsPaletteFadeActive() -> u32;
    fn IsTextPrinterActive(a0: u8) -> u16;
    fn LZ77UnCompWram(a0: *mut u32, a1: *mut u8);
    fn LoadCompressedSpriteSheet(a0: *mut u8) -> u16;
    fn LoadLeftHeaderGfxForIndex(a0: u32);
    fn LoadMatchCallWindowGfx(a0: u32, a1: u32, a2: u32);
    fn LoadPalette(a0: *mut u8, a1: u16, a2: u16);
    fn LoadSpriteSheet(a0: *mut u8) -> u16;
    fn LoadUserWindowBorderGfx(a0: u8, a1: u16, a2: u8);
    fn MainMenuLoopedTaskIsBusy() -> u32;
    fn PlaySE(a0: u16);
    fn PokenavCopyPalette(a0: *mut u16, a1: *mut u16, a2: i32, a3: i32, a4: i32, a5: *mut u16);
    fn PokenavFadeScreen(a0: i32);
    fn PokenavList_DrawCurrentItemIcon();
    fn PokenavList_EraseListForCheckPage();
    fn PokenavList_GetSelectedIndex() -> u32;
    fn PokenavList_GetTopIndex() -> u32;
    fn PokenavList_IsMoveWindowTaskActive() -> u32;
    fn PokenavList_IsTaskActive() -> u32;
    fn PokenavList_MoveCursorDown() -> i32;
    fn PokenavList_MoveCursorUp() -> i32;
    fn PokenavList_PageDown() -> i32;
    fn PokenavList_PageUp() -> i32;
    fn PokenavList_ReshowListFromCheckPage();
    fn PokenavList_ToggleVerticalArrows(a0: u32);
    fn Pokenav_AllocAndLoadPalettes(a0: *mut u8);
    fn PrintCheckPageInfo(a0: i16);
    fn PrintHelpBarText(a0: u32);
    fn PutWindowTilemap(a0: u8);
    fn RemoveWindow(a0: u8);
    fn RequestDma3Copy(a0: *mut u8, a1: *mut u8, a2: u16, a3: u8) -> i16;
    fn RunTextPrinters();
    fn SetBgTilemapBuffer(a0: u8, a1: *mut u8);
    fn SetLeftHeaderSpritesInvisibility();
    fn ShouldDrawRematchPokeballIcon(a0: i32) -> u32;
    fn ShowBg(a0: u8);
    fn ShowLeftHeaderGfx(a0: u32, a1: u32, a2: u32);
    fn SlideMenuHeaderDown();
    fn SpriteCallbackDummy(a0: *mut u8);
    fn StringCopy(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn WaitForHelpBar() -> u32;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn OpenMatchCall() -> u32 {
    unsafe {
        let mut gfx: *mut u8 = AllocSubstruct(6u32, 8264u32);
        if !(!(gfx).is_null()) {
            return 0u32;
        }
        ((gfx).wrapping_add(25)).write(0u8);
        ((gfx).wrapping_add(4).cast::<u32>())
            .write(CreateLoopedTask(Some(LoopedTask_OpenMatchCall), 1u32));
        ((gfx).cast::<Option<unsafe extern "C" fn() -> u32>>())
            .write(Some(GetCurrentLoopedTaskActive));
        return 1u32;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CreateMatchCallLoopedTask(index: i32) {
    unsafe {
        let mut index = index;
        let mut gfx: *mut u8 = GetSubstructPtr(6u32);
        ((gfx).wrapping_add(4).cast::<u32>()).write(CreateLoopedTask(
            ((((&raw const sMatchCallLoopTaskFuncs)
                .cast::<u8>()
                .cast_mut()
                .cast::<Option<unsafe extern "C" fn(i32) -> u32>>())
            .cast::<Option<unsafe extern "C" fn(i32) -> u32>>())
            .wrapping_offset((index) as isize))
            .read(),
            1u32,
        ));
        ((gfx).cast::<Option<unsafe extern "C" fn() -> u32>>())
            .write(Some(GetCurrentLoopedTaskActive));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsMatchCallLoopedTaskActive() -> u32 {
    unsafe {
        let mut gfx: *mut u8 = GetSubstructPtr(6u32);
        return (((gfx).cast::<Option<unsafe extern "C" fn() -> u32>>()).read()).unwrap_unchecked()(
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FreeMatchCallSubstruct2() {
    unsafe {
        let mut gfx: *mut u8 = GetSubstructPtr(6u32);
        FreeMatchCallSprites();
        DestroyMatchCallList();
        RemoveWindow(((((gfx).wrapping_add(18).cast::<u16>()).read()) as u8));
        RemoveWindow(((((gfx).wrapping_add(16).cast::<u16>()).read()) as u8));
        RemoveWindow(((((gfx).wrapping_add(20).cast::<u16>()).read()) as u8));
        FreePokenavSubstruct(6u32);
    }
}
pub(crate) unsafe extern "C" fn GetCurrentLoopedTaskActive() -> u32 {
    unsafe {
        let mut gfx: *mut u8 = GetSubstructPtr(6u32);
        return IsLoopedTaskActive(((gfx).wrapping_add(4).cast::<u32>()).read());
    }
}
pub(crate) unsafe extern "C" fn LoopedTask_OpenMatchCall(state: i32) -> u32 {
    unsafe {
        let mut state = state;
        let mut gfx: *mut u8 = GetSubstructPtr(6u32);
        'l1: {
            let __sw1 = state;
            let __matched = __sw1 == 0i32
                || __sw1 == 1i32
                || __sw1 == 2i32
                || __sw1 == 3i32
                || __sw1 == 4i32
                || __sw1 == 5i32
                || __sw1 == 6i32
                || __sw1 == 7i32;
            if __sw1 == 0i32 {
                InitBgTemplates(
                    ((&raw const sMatchCallBgTemplates).cast::<u8>().cast_mut()).cast::<u8>(),
                    ((crate::c::div_u32(12u32, 4u32)) as i32),
                );
                ChangeBgX(2u8, 0i32, 0u8);
                ChangeBgY(2u8, 0i32, 0u8);
                DecompressAndCopyTileDataToVram(
                    2u8,
                    (((&raw const sMatchCallUI_Gfx)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<u32>())
                    .cast::<u32>())
                    .cast::<u8>(),
                    0u32,
                    0u16,
                    0u8,
                );
                SetBgTilemapBuffer(2u8, ((gfx).wrapping_add(4132)).cast::<u8>());
                CopyToBgTilemapBuffer(
                    2u8,
                    (((&raw const sMatchCallUI_Tilemap)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<u32>())
                    .cast::<u32>())
                    .cast::<u8>(),
                    0u16,
                    0u16,
                );
                CopyBgTilemapBufferToVram(2u8);
                CopyPaletteIntoBufferUnfaded(
                    ((&raw const sMatchCallUI_Pal)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<u16>())
                    .cast::<u16>(),
                    32u32,
                    32u32,
                );
                CopyBgTilemapBufferToVram(2u8);
                return 0u32;
            }
            if __sw1 == 1i32 {
                if (FreeTempTileDataBuffersIfPossible()) != 0 {
                    return 2u32;
                }
                BgDmaFill(1u32, 0u8, 0i32, 1i32);
                SetBgTilemapBuffer(1u8, ((gfx).wrapping_add(36)).cast::<u8>());
                FillBgTilemapBufferRect_Palette0(1u8, 4096u16, 0u8, 0u8, 32u8, 20u8);
                CopyPaletteIntoBufferUnfaded(
                    ((&raw const sCallWindow_Pal)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<u16>())
                    .cast::<u16>(),
                    16u32,
                    32u32,
                );
                CopyBgTilemapBufferToVram(1u8);
                return 0u32;
            }
            if __sw1 == 2i32 {
                if (FreeTempTileDataBuffersIfPossible()) != 0 {
                    return 2u32;
                }
                LoadCallWindowAndFade(gfx);
                DecompressAndCopyTileDataToVram(
                    3u8,
                    (((&raw const sPokeball_Gfx)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<u32>())
                    .cast::<u32>())
                    .cast::<u8>(),
                    0u32,
                    0u16,
                    0u8,
                );
                CopyPaletteIntoBufferUnfaded(
                    ((&raw const sListWindow_Pal)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<u16>())
                    .cast::<u16>(),
                    48u32,
                    32u32,
                );
                CopyPaletteIntoBufferUnfaded(
                    ((&raw const sPokeball_Pal)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<u16>())
                    .cast::<u16>(),
                    80u32,
                    32u32,
                );
                return 0u32;
            }
            if __sw1 == 3i32 {
                if ((FreeTempTileDataBuffersIfPossible()) != 0)
                    || (!((IsMatchCallListInitFinished()) != 0))
                {
                    return 2u32;
                }
                CreateMatchCallList();
                return 0u32;
            }
            if __sw1 == 4i32 {
                if (IsCreatePokenavListTaskActive()) != 0 {
                    return 2u32;
                }
                DrawMatchCallLeftColumnWindows(gfx);
                return 0u32;
            }
            if __sw1 == 5i32 {
                UpdateMatchCallInfoBox(gfx);
                PrintMatchCallLocation(gfx, 0i32);
                return 0u32;
            }
            if __sw1 == 6i32 {
                ChangeBgX(1u8, 0i32, 0u8);
                ChangeBgY(1u8, 0i32, 0u8);
                ShowBg(2u8);
                ShowBg(3u8);
                ShowBg(1u8);
                AllocMatchCallSprites();
                LoadLeftHeaderGfxForIndex(3u32);
                ShowLeftHeaderGfx(3u32, 1u32, 0u32);
                PokenavFadeScreen(1i32);
                return 0u32;
            }
            if __sw1 == 7i32 {
                if ((IsPaletteFadeActive()) != 0) || ((AreLeftHeaderSpritesMoving()) != 0) {
                    return 2u32;
                }
                SetPokeballIconsFlashing(1u32);
                return 4u32;
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
pub(crate) unsafe extern "C" fn MatchCallListCursorDown(state: i32) -> u32 {
    unsafe {
        let mut state = state;
        let mut gfx: *mut u8 = GetSubstructPtr(6u32);
        'l1: {
            let __sw1 = state;
            if __sw1 == 0i32 {
                'l2: {
                    let __sw2 = PokenavList_MoveCursorDown();
                    let __matched = __sw2 == 0i32 || __sw2 == 1i32 || __sw2 == 2i32;
                    let mut __fall = false;
                    if __sw2 == 0i32 {
                        __fall = true;
                        break 'l2;
                    }
                    if __sw2 == 1i32 {
                        __fall = true;
                        PlaySE(5u16);
                        return 7u32;
                    }
                    if __sw2 == 2i32 {
                        __fall = true;
                        PlaySE(5u16);
                    }
                    if __fall || !__matched {
                        __fall = true;
                        return 0u32;
                    }
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                if (PokenavList_IsMoveWindowTaskActive()) != 0 {
                    return 2u32;
                }
                PrintMatchCallLocation(gfx, 0i32);
                return 0u32;
            }
            if __sw1 == 2i32 {
                PrintMatchCallLocation(gfx, 0i32);
                return 0u32;
            }
            if __sw1 == 3i32 {
                if (IsDma3ManagerBusyWithBgCopy()) != 0 {
                    return 2u32;
                }
                break 'l1;
            }
        }
        return 4u32;
    }
}
pub(crate) unsafe extern "C" fn MatchCallListCursorUp(state: i32) -> u32 {
    unsafe {
        let mut state = state;
        let mut gfx: *mut u8 = GetSubstructPtr(6u32);
        'l1: {
            let __sw1 = state;
            if __sw1 == 0i32 {
                'l2: {
                    let __sw2 = PokenavList_MoveCursorUp();
                    let __matched = __sw2 == 0i32 || __sw2 == 1i32 || __sw2 == 2i32;
                    let mut __fall = false;
                    if __sw2 == 0i32 {
                        __fall = true;
                        break 'l2;
                    }
                    if __sw2 == 1i32 {
                        __fall = true;
                        PlaySE(5u16);
                        return 7u32;
                    }
                    if __sw2 == 2i32 {
                        __fall = true;
                        PlaySE(5u16);
                    }
                    if __fall || !__matched {
                        __fall = true;
                        return 0u32;
                    }
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                if (PokenavList_IsMoveWindowTaskActive()) != 0 {
                    return 2u32;
                }
                PrintMatchCallLocation(gfx, 0i32);
                return 0u32;
            }
            if __sw1 == 2i32 {
                PrintMatchCallLocation(gfx, 0i32);
                return 0u32;
            }
            if __sw1 == 3i32 {
                if (IsDma3ManagerBusyWithBgCopy()) != 0 {
                    return 2u32;
                }
                break 'l1;
            }
        }
        return 4u32;
    }
}
pub(crate) unsafe extern "C" fn MatchCallListPageDown(state: i32) -> u32 {
    unsafe {
        let mut state = state;
        let mut gfx: *mut u8 = GetSubstructPtr(6u32);
        'l1: {
            let __sw1 = state;
            if __sw1 == 0i32 {
                'l2: {
                    let __sw2 = PokenavList_PageDown();
                    let __matched = __sw2 == 0i32 || __sw2 == 1i32 || __sw2 == 2i32;
                    let mut __fall = false;
                    if __sw2 == 0i32 {
                        __fall = true;
                        break 'l2;
                    }
                    if __sw2 == 1i32 {
                        __fall = true;
                        PlaySE(5u16);
                        return 7u32;
                    }
                    if __sw2 == 2i32 {
                        __fall = true;
                        PlaySE(5u16);
                    }
                    if __fall || !__matched {
                        __fall = true;
                        return 0u32;
                    }
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                if (PokenavList_IsMoveWindowTaskActive()) != 0 {
                    return 2u32;
                }
                PrintMatchCallLocation(gfx, 0i32);
                return 0u32;
            }
            if __sw1 == 2i32 {
                PrintMatchCallLocation(gfx, 0i32);
                return 0u32;
            }
            if __sw1 == 3i32 {
                if (IsDma3ManagerBusyWithBgCopy()) != 0 {
                    return 2u32;
                }
                break 'l1;
            }
        }
        return 4u32;
    }
}
pub(crate) unsafe extern "C" fn MatchCallListPageUp(state: i32) -> u32 {
    unsafe {
        let mut state = state;
        let mut gfx: *mut u8 = GetSubstructPtr(6u32);
        'l1: {
            let __sw1 = state;
            if __sw1 == 0i32 {
                'l2: {
                    let __sw2 = PokenavList_PageUp();
                    let __matched = __sw2 == 0i32 || __sw2 == 1i32 || __sw2 == 2i32;
                    let mut __fall = false;
                    if __sw2 == 0i32 {
                        __fall = true;
                        break 'l2;
                    }
                    if __sw2 == 1i32 {
                        __fall = true;
                        PlaySE(5u16);
                        return 7u32;
                    }
                    if __sw2 == 2i32 {
                        __fall = true;
                        PlaySE(5u16);
                    }
                    if __fall || !__matched {
                        __fall = true;
                        return 0u32;
                    }
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                if (PokenavList_IsMoveWindowTaskActive()) != 0 {
                    return 2u32;
                }
                PrintMatchCallLocation(gfx, 0i32);
                return 0u32;
            }
            if __sw1 == 2i32 {
                PrintMatchCallLocation(gfx, 0i32);
                return 0u32;
            }
            if __sw1 == 3i32 {
                if (IsDma3ManagerBusyWithBgCopy()) != 0 {
                    return 2u32;
                }
                break 'l1;
            }
        }
        return 4u32;
    }
}
pub(crate) unsafe extern "C" fn SelectMatchCallEntry(state: i32) -> u32 {
    unsafe {
        let mut state = state;
        let mut gfx: *mut u8 = GetSubstructPtr(6u32);
        'l1: {
            let __sw1 = state;
            if __sw1 == 0i32 {
                PlaySE(5u16);
                PrintMatchCallSelectionOptions(gfx);
                PrintHelpBarText(7u32);
                return 0u32;
            }
            if __sw1 == 1i32 {
                if (ShowOptionsCursor(gfx)) != 0 {
                    return 2u32;
                }
                break 'l1;
            }
        }
        return 4u32;
    }
}
pub(crate) unsafe extern "C" fn MoveMatchCallOptionsCursor(state: i32) -> u32 {
    unsafe {
        let mut state = state;
        let mut gfx: *mut u8 = core::ptr::null_mut();
        let mut cursorPos: u16 = 0u16;
        PlaySE(5u16);
        gfx = GetSubstructPtr(6u32);
        cursorPos = GetMatchCallOptionCursorPos();
        UpdateCursorGfxPos(gfx, ((cursorPos) as i32));
        return 4u32;
    }
}
pub(crate) unsafe extern "C" fn CancelMatchCallSelection(state: i32) -> u32 {
    unsafe {
        let mut state = state;
        let mut gfx: *mut u8 = GetSubstructPtr(6u32);
        'l1: {
            let __sw1 = state;
            if __sw1 == 0i32 {
                PlaySE(5u16);
                UpdateWindowsReturnToTrainerList(gfx);
                PrintHelpBarText(6u32);
                return 0u32;
            }
            if __sw1 == 1i32 {
                if (IsDma3ManagerBusyWithBgCopy1(gfx)) != 0 {
                    return 2u32;
                }
                break 'l1;
            }
        }
        return 4u32;
    }
}
pub(crate) unsafe extern "C" fn DoMatchCallMessage(state: i32) -> u32 {
    unsafe {
        let mut state = state;
        let mut gfx: *mut u8 = GetSubstructPtr(6u32);
        'l1: {
            let __sw1 = state;
            if __sw1 == 0i32 {
                PokenavList_ToggleVerticalArrows(1u32);
                DrawMsgBoxForMatchCallMsg(gfx);
                return 0u32;
            }
            if __sw1 == 1i32 {
                if (IsDma3ManagerBusyWithBgCopy2(gfx)) != 0 {
                    return 2u32;
                }
                PrintCallingDots(gfx);
                PlaySE(263u16);
                ((gfx).wrapping_add(14)).write(0u8);
                return 0u32;
            }
            if __sw1 == 2i32 {
                if (WaitForCallingDotsText(gfx)) != 0 {
                    return 2u32;
                }
                PrintMatchCallMessage(gfx);
                return 0u32;
            }
            if __sw1 == 3i32 {
                if (WaitForMatchCallMessageText(gfx)) != 0 {
                    return 2u32;
                }
                break 'l1;
            }
        }
        return 4u32;
    }
}
pub(crate) unsafe extern "C" fn DoTrainerCloseByMessage(state: i32) -> u32 {
    unsafe {
        let mut state = state;
        let mut gfx: *mut u8 = GetSubstructPtr(6u32);
        'l1: {
            let __sw1 = state;
            if __sw1 == 0i32 {
                PlaySE(5u16);
                DrawMsgBoxForCloseByMsg(gfx);
                PokenavList_ToggleVerticalArrows(1u32);
                ((gfx).wrapping_add(14)).write(1u8);
                return 0u32;
            }
            if __sw1 == 1i32 {
                if (IsDma3ManagerBusyWithBgCopy2(gfx)) != 0 {
                    return 2u32;
                }
                PrintTrainerIsCloseBy(gfx);
                return 0u32;
            }
            if __sw1 == 2i32 {
                if (WaitForTrainerIsCloseByText(gfx)) != 0 {
                    return 2u32;
                }
                break 'l1;
            }
        }
        return 4u32;
    }
}
pub(crate) unsafe extern "C" fn CloseMatchCallMessage(state: i32) -> u32 {
    unsafe {
        let mut state = state;
        let mut gfx: *mut u8 = GetSubstructPtr(6u32);
        let mut result: u32 = 0u32;
        'l1: {
            let __sw1 = state;
            if __sw1 == 0i32 {
                if !((((gfx).wrapping_add(14)).read()) != 0) {
                    PlaySE(264u16);
                }
                PlaySE(5u16);
                break 'l1;
            }
            if __sw1 == 1i32 {
                EraseCallMessageBox(gfx);
                break 'l1;
            }
            if __sw1 == 2i32 {
                if (WaitForCallMessageBoxErase(gfx)) != 0 {
                    result = 2u32;
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                UpdateWindowsReturnToTrainerList(gfx);
                break 'l1;
            }
            if __sw1 == 4i32 {
                if (IsDma3ManagerBusyWithBgCopy1(gfx)) != 0 {
                    result = 2u32;
                }
                PrintHelpBarText(6u32);
                break 'l1;
            }
            if __sw1 == 5i32 {
                if (WaitForHelpBar()) != 0 {
                    result = 2u32;
                } else {
                    if (((gfx).wrapping_add(15)).read()) != 0 {
                        PokenavList_DrawCurrentItemIcon();
                        result = 1u32;
                    } else {
                        PokenavList_ToggleVerticalArrows(0u32);
                        result = 4u32;
                    }
                }
                break 'l1;
            }
            if __sw1 == 6i32 {
                if (IsDma3ManagerBusyWithBgCopy()) != 0 {
                    result = 2u32;
                } else {
                    PokenavList_ToggleVerticalArrows(0u32);
                    result = 4u32;
                }
                break 'l1;
            }
        }
        return result;
    }
}
pub(crate) unsafe extern "C" fn ShowCheckPage(state: i32) -> u32 {
    unsafe {
        let mut state = state;
        let mut gfx: *mut u8 = GetSubstructPtr(6u32);
        'l1: {
            let __sw1 = state;
            if __sw1 == 0i32 {
                PlaySE(5u16);
                PokenavList_EraseListForCheckPage();
                UpdateWindowsToShowCheckPage(gfx);
                return 0u32;
            }
            if __sw1 == 1i32 {
                if ((PokenavList_IsTaskActive()) != 0) || ((IsDma3ManagerBusyWithBgCopy1(gfx)) != 0)
                {
                    return 2u32;
                }
                PrintHelpBarText(8u32);
                return 0u32;
            }
            if __sw1 == 2i32 {
                PrintCheckPageInfo(0i16);
                LoadCheckPageTrainerPic(gfx);
                return 0u32;
            }
            if __sw1 == 3i32 {
                if (((PokenavList_IsTaskActive()) != 0) || ((WaitForTrainerPic(gfx)) != 0))
                    || ((WaitForHelpBar()) != 0)
                {
                    return 2u32;
                }
                break 'l1;
            }
        }
        return 4u32;
    }
}
pub(crate) unsafe extern "C" fn ShowCheckPageDown(state: i32) -> u32 {
    unsafe {
        let mut state = state;
        let mut topId: i32 = 0i32;
        let mut delta: i32 = 0i32;
        let mut gfx: *mut u8 = GetSubstructPtr(6u32);
        'l1: {
            let __sw1 = state;
            if __sw1 == 0i32 {
                topId = ((PokenavList_GetTopIndex()) as i32);
                delta = GetIndexDeltaOfNextCheckPageDown(topId);
                if (delta) != 0 {
                    PlaySE(5u16);
                    ((gfx).wrapping_add(22).cast::<i16>()).write(((delta) as i16));
                    TrainerPicSlideOffscreen(gfx);
                    return 0u32;
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                if (WaitForTrainerPic(gfx)) != 0 {
                    return 2u32;
                }
                PrintMatchCallLocation(
                    gfx,
                    ((((gfx).wrapping_add(22).cast::<i16>()).read()) as i32),
                );
                return 0u32;
            }
            if __sw1 == 2i32 {
                PrintCheckPageInfo(((gfx).wrapping_add(22).cast::<i16>()).read());
                return 0u32;
            }
            if __sw1 == 3i32 {
                LoadCheckPageTrainerPic(gfx);
                return 0u32;
            }
            if __sw1 == 4i32 {
                if ((PokenavList_IsTaskActive()) != 0) || ((WaitForTrainerPic(gfx)) != 0) {
                    return 2u32;
                }
                break 'l1;
            }
        }
        return 4u32;
    }
}
pub(crate) unsafe extern "C" fn ExitCheckPage(state: i32) -> u32 {
    unsafe {
        let mut state = state;
        let mut gfx: *mut u8 = GetSubstructPtr(6u32);
        'l1: {
            let __sw1 = state;
            if __sw1 == 0i32 {
                PlaySE(5u16);
                TrainerPicSlideOffscreen(gfx);
                PokenavList_ReshowListFromCheckPage();
                return 0u32;
            }
            if __sw1 == 1i32 {
                if ((PokenavList_IsTaskActive()) != 0) || ((WaitForTrainerPic(gfx)) != 0) {
                    return 2u32;
                }
                PrintHelpBarText(6u32);
                UpdateMatchCallInfoBox(gfx);
                return 0u32;
            }
            if __sw1 == 2i32 {
                if (IsDma3ManagerBusyWithBgCopy()) != 0 {
                    return 2u32;
                }
                break 'l1;
            }
        }
        return 4u32;
    }
}
pub(crate) unsafe extern "C" fn ShowCheckPageUp(state: i32) -> u32 {
    unsafe {
        let mut state = state;
        let mut topId: i32 = 0i32;
        let mut delta: i32 = 0i32;
        let mut gfx: *mut u8 = GetSubstructPtr(6u32);
        'l1: {
            let __sw1 = state;
            if __sw1 == 0i32 {
                topId = ((PokenavList_GetTopIndex()) as i32);
                delta = GetIndexDeltaOfNextCheckPageUp(topId);
                if (delta) != 0 {
                    PlaySE(5u16);
                    ((gfx).wrapping_add(22).cast::<i16>()).write(((delta) as i16));
                    TrainerPicSlideOffscreen(gfx);
                    return 0u32;
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                if (WaitForTrainerPic(gfx)) != 0 {
                    return 2u32;
                }
                PrintMatchCallLocation(
                    gfx,
                    ((((gfx).wrapping_add(22).cast::<i16>()).read()) as i32),
                );
                return 0u32;
            }
            if __sw1 == 2i32 {
                PrintCheckPageInfo(((gfx).wrapping_add(22).cast::<i16>()).read());
                return 0u32;
            }
            if __sw1 == 3i32 {
                LoadCheckPageTrainerPic(gfx);
                return 0u32;
            }
            if __sw1 == 4i32 {
                if ((PokenavList_IsTaskActive()) != 0) || ((WaitForTrainerPic(gfx)) != 0) {
                    return 2u32;
                }
                break 'l1;
            }
        }
        return 4u32;
    }
}
pub(crate) unsafe extern "C" fn ExitMatchCall(state: i32) -> u32 {
    unsafe {
        let mut state = state;
        'l1: {
            let __sw1 = state;
            if __sw1 == 0i32 {
                PlaySE(5u16);
                SetPokeballIconsFlashing(0u32);
                PokenavFadeScreen(0i32);
                SlideMenuHeaderDown();
                return 0u32;
            }
            if __sw1 == 1i32 {
                if ((IsPaletteFadeActive()) != 0) || ((MainMenuLoopedTaskIsBusy()) != 0) {
                    return 2u32;
                }
                SetLeftHeaderSpritesInvisibility();
                break 'l1;
            }
        }
        return 4u32;
    }
}
pub(crate) unsafe extern "C" fn CreateMatchCallList() {
    unsafe {
        let mut template = crate::ffi::Align4([0u8; 24]);
        (((&raw mut template).cast::<u8>()).cast::<*mut u8>()).write(GetMatchCallList());
        (((&raw mut template).cast::<u8>())
            .wrapping_add(4)
            .cast::<u16>())
        .write(((GetNumberRegistered()) as u16));
        (((&raw mut template).cast::<u8>()).wrapping_add(8)).write(4u8);
        (((&raw mut template).cast::<u8>())
            .wrapping_add(6)
            .cast::<u16>())
        .write(0u16);
        (((&raw mut template).cast::<u8>()).wrapping_add(9)).write(13u8);
        (((&raw mut template).cast::<u8>()).wrapping_add(10)).write(16u8);
        (((&raw mut template).cast::<u8>()).wrapping_add(11)).write(1u8);
        (((&raw mut template).cast::<u8>()).wrapping_add(12)).write(8u8);
        (((&raw mut template).cast::<u8>()).wrapping_add(13)).write(3u8);
        (((&raw mut template).cast::<u8>()).wrapping_add(14)).write(7u8);
        (((&raw mut template).cast::<u8>())
            .wrapping_add(16)
            .cast::<Option<unsafe extern "C" fn(*mut u8, *mut u8)>>())
        .write(Some(BufferMatchCallNameAndDesc));
        (((&raw mut template).cast::<u8>())
            .wrapping_add(20)
            .cast::<Option<unsafe extern "C" fn(u16, u32, u32)>>())
        .write(Some(TryDrawRematchPokeballIcon));
        CreatePokenavList(
            (((&raw const sMatchCallBgTemplates).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(8),
            (&raw mut template).cast::<u8>(),
            2u32,
        );
        CreateTask(Some(Task_FlashPokeballIcons), 7u8);
    }
}
pub(crate) unsafe extern "C" fn DestroyMatchCallList() {
    unsafe {
        DestroyPokenavList();
        DestroyTask(FindTaskIdByFunc(Some(Task_FlashPokeballIcons)));
    }
}
pub(crate) unsafe extern "C" fn SetPokeballIconsFlashing(active: u32) {
    unsafe {
        let mut active = active;
        let mut taskId: u8 = FindTaskIdByFunc(Some(Task_FlashPokeballIcons));
        if ((taskId) as i32) != 255i32 {
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(15))
            .write(((active) as i16));
        }
    }
}
pub(crate) unsafe extern "C" fn Task_FlashPokeballIcons(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        if (((data).wrapping_offset(15)).read()) != 0 {
            (data).write((((((data).read()) as i32).wrapping_add(4i32)) as i16));
            (data).write((((((data).read()) as i32) & 127i32) as i16));
            ((data).wrapping_offset(1)).write(
                ((((((((&raw mut gSineTable).cast::<i16>()).cast::<i16>())
                    .wrapping_offset((((data).read()) as i32) as isize))
                .read()) as i32)
                    >> 4) as i16),
            );
            PokenavCopyPalette(
                ((&raw const sPokeball_Pal)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<u16>())
                .cast::<u16>(),
                (((&raw const sPokeball_Pal)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<u16>())
                .cast::<u16>())
                .wrapping_offset(16),
                16i32,
                16i32,
                ((((data).wrapping_offset(1)).read()) as i32),
                (((&raw mut gPlttBufferUnfaded).cast::<u16>()).cast::<u16>()).wrapping_offset(80),
            );
            if !((crate::c::bf_read(
                ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
                7,
                1,
                false,
            ) as u16)
                != 0)
            {
                'l1: loop {
                    'l2: {
                        'l3: loop {
                            'l4: {
                                CpuSet(
                                    ((((&raw mut gPlttBufferUnfaded).cast::<u16>()).cast::<u16>())
                                        .wrapping_offset(80))
                                    .cast::<u8>(),
                                    ((((&raw mut gPlttBufferFaded).cast::<u16>()).cast::<u16>())
                                        .wrapping_offset(80))
                                    .cast::<u8>(),
                                    (67108864u32
                                        | (crate::c::div_u32(
                                            32u32,
                                            ((crate::c::div_i32(32i32, 8i32)) as u32),
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
    }
}
pub(crate) unsafe extern "C" fn TryDrawRematchPokeballIcon(
    windowId: u16,
    rematchId: u32,
    tileOffset: u32,
) {
    unsafe {
        let mut windowId = windowId;
        let mut rematchId = rematchId;
        let mut tileOffset = tileOffset;
        let mut bg: u8 = ((GetWindowAttribute(((windowId) as u8), 0u8)) as u8);
        let mut tilemap: *mut u16 = (GetBgTilemapBuffer(bg)).cast::<u16>();
        tilemap = (tilemap).wrapping_offset(
            ((((tileOffset).wrapping_mul(64u32)).wrapping_add(29u32)) as i32) as isize,
        );
        if (ShouldDrawRematchPokeballIcon(((rematchId) as i32))) != 0 {
            (tilemap).write(20480u16);
            ((tilemap).wrapping_offset(32)).write(20481u16);
        } else {
            (tilemap).write(20482u16);
            ((tilemap).wrapping_offset(32)).write(20482u16);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ClearRematchPokeballIcon(windowId: u16, tileOffset: u32) {
    unsafe {
        let mut windowId = windowId;
        let mut tileOffset = tileOffset;
        let mut bg: u8 = ((GetWindowAttribute(((windowId) as u8), 0u8)) as u8);
        let mut tilemap: *mut u16 = (GetBgTilemapBuffer(bg)).cast::<u16>();
        tilemap = (tilemap).wrapping_offset(
            ((((tileOffset).wrapping_mul(64u32)).wrapping_add(29u32)) as i32) as isize,
        );
        (tilemap).write(20482u16);
        ((tilemap).wrapping_offset(32)).write(20482u16);
    }
}
pub(crate) unsafe extern "C" fn DrawMatchCallLeftColumnWindows(gfx: *mut u8) {
    unsafe {
        let mut gfx = gfx;
        ((gfx).wrapping_add(16).cast::<u16>()).write(AddWindow(
            (&raw const sMatchCallLocationWindowTemplate)
                .cast::<u8>()
                .cast_mut(),
        ));
        ((gfx).wrapping_add(18).cast::<u16>()).write(AddWindow(
            (&raw const sMatchCallInfoBoxWindowTemplate)
                .cast::<u8>()
                .cast_mut(),
        ));
        FillWindowPixelBuffer(
            ((((gfx).wrapping_add(16).cast::<u16>()).read()) as u8),
            17u8,
        );
        PutWindowTilemap(((((gfx).wrapping_add(16).cast::<u16>()).read()) as u8));
        FillWindowPixelBuffer(
            ((((gfx).wrapping_add(18).cast::<u16>()).read()) as u8),
            17u8,
        );
        PutWindowTilemap(((((gfx).wrapping_add(18).cast::<u16>()).read()) as u8));
        CopyWindowToVram(((((gfx).wrapping_add(16).cast::<u16>()).read()) as u8), 1u8);
    }
}
pub(crate) unsafe extern "C" fn UpdateMatchCallInfoBox(gfx: *mut u8) {
    unsafe {
        let mut gfx = gfx;
        FillWindowPixelBuffer(
            ((((gfx).wrapping_add(18).cast::<u16>()).read()) as u8),
            17u8,
        );
        PrintNumberRegisteredLabel(((gfx).wrapping_add(18).cast::<u16>()).read());
        PrintNumberRegistered(((gfx).wrapping_add(18).cast::<u16>()).read());
        PrintNumberOfBattlesLabel(((gfx).wrapping_add(18).cast::<u16>()).read());
        PrintNumberOfBattles(((gfx).wrapping_add(18).cast::<u16>()).read());
        CopyWindowToVram(((((gfx).wrapping_add(18).cast::<u16>()).read()) as u8), 2u8);
    }
}
pub(crate) unsafe extern "C" fn PrintNumberRegisteredLabel(windowId: u16) {
    unsafe {
        let mut windowId = windowId;
        PrintMatchCallInfoLabel(
            windowId,
            (&raw mut gText_NumberRegistered).cast::<u8>(),
            0i32,
        );
    }
}
pub(crate) unsafe extern "C" fn PrintNumberRegistered(windowId: u16) {
    unsafe {
        let mut windowId = windowId;
        let mut str = crate::ffi::Align4([0u8; 3]);
        ConvertIntToDecimalStringN(
            (&raw mut str).cast::<u8>(),
            GetNumberRegistered(),
            0i32,
            3u8,
        );
        PrintMatchCallInfoNumber(windowId, (&raw mut str).cast::<u8>(), 1i32);
    }
}
pub(crate) unsafe extern "C" fn PrintNumberOfBattlesLabel(windowId: u16) {
    unsafe {
        let mut windowId = windowId;
        PrintMatchCallInfoLabel(
            windowId,
            (&raw mut gText_NumberOfBattles).cast::<u8>(),
            2i32,
        );
    }
}
pub(crate) unsafe extern "C" fn PrintNumberOfBattles(windowId: u16) {
    unsafe {
        let mut windowId = windowId;
        let mut str = crate::ffi::Align4([0u8; 5]);
        let mut numTrainerBattles: i32 = ((GetGameStat(9u8)) as i32);
        if numTrainerBattles > 99999i32 {
            numTrainerBattles = 99999i32;
        }
        ConvertIntToDecimalStringN((&raw mut str).cast::<u8>(), numTrainerBattles, 0i32, 5u8);
        PrintMatchCallInfoNumber(windowId, (&raw mut str).cast::<u8>(), 3i32);
    }
}
pub(crate) unsafe extern "C" fn PrintMatchCallInfoLabel(windowId: u16, str: *mut u8, top: i32) {
    unsafe {
        let mut windowId = windowId;
        let mut str = str;
        let mut top = top;
        let mut y: i32 = ((top).wrapping_mul(16i32)).wrapping_add(1i32);
        AddTextPrinterParameterized(((windowId) as u8), 7u8, str, 2u8, ((y) as u8), 255u8, None);
    }
}
pub(crate) unsafe extern "C" fn PrintMatchCallInfoNumber(windowId: u16, str: *mut u8, top: i32) {
    unsafe {
        let mut windowId = windowId;
        let mut str = str;
        let mut top = top;
        let mut x: i32 = GetStringRightAlignXOffset(7i32, str, 86i32);
        let mut y: i32 = ((top).wrapping_mul(16i32)).wrapping_add(1i32);
        AddTextPrinterParameterized(
            ((windowId) as u8),
            7u8,
            str,
            ((x) as u8),
            ((y) as u8),
            255u8,
            None,
        );
    }
}
pub(crate) unsafe extern "C" fn PrintMatchCallLocation(gfx: *mut u8, delta: i32) {
    unsafe {
        let mut gfx = gfx;
        let mut delta = delta;
        let mut mapName = crate::ffi::Align4([0u8; 32]);
        let mut x: i32 = 0i32;
        let mut index: i32 =
            (((PokenavList_GetSelectedIndex()).wrapping_add(((delta) as u32))) as i32);
        let mut mapSec: i32 = ((GetMatchCallMapSec(index)) as i32);
        if mapSec != 213i32 {
            GetMapName((&raw mut mapName).cast::<u8>(), ((mapSec) as u16), 0u16);
        } else {
            StringCopy(
                (&raw mut mapName).cast::<u8>(),
                (&raw mut gText_Unknown).cast::<u8>(),
            );
        }
        x = GetStringCenterAlignXOffset(7i32, (&raw mut mapName).cast::<u8>(), 88i32);
        FillWindowPixelBuffer(
            ((((gfx).wrapping_add(16).cast::<u16>()).read()) as u8),
            17u8,
        );
        AddTextPrinterParameterized(
            ((((gfx).wrapping_add(16).cast::<u16>()).read()) as u8),
            7u8,
            (&raw mut mapName).cast::<u8>(),
            ((x) as u8),
            1u8,
            0u8,
            None,
        );
    }
}
pub(crate) unsafe extern "C" fn PrintMatchCallSelectionOptions(gfx: *mut u8) {
    unsafe {
        let mut gfx = gfx;
        let mut i: u32 = 0u32;
        FillWindowPixelBuffer(
            ((((gfx).wrapping_add(18).cast::<u16>()).read()) as u8),
            17u8,
        );
        {
            i = 0u32;
            'l1: loop {
                if !(i < 3u32) {
                    break 'l1;
                }
                'l2: {
                    let mut optionText: i32 = ((GetMatchCallOptionId(((i) as i32))) as i32);
                    if optionText == 3i32 {
                        break 'l1;
                    }
                    AddTextPrinterParameterized(
                        ((((gfx).wrapping_add(18).cast::<u16>()).read()) as u8),
                        7u8,
                        ((((&raw const sMatchCallOptionTexts)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<*mut u8>())
                        .cast::<*mut u8>())
                        .wrapping_offset((optionText) as isize))
                        .read(),
                        16u8,
                        ((((i).wrapping_mul(16u32)).wrapping_add(1u32)) as u8),
                        255u8,
                        None,
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        CopyWindowToVram(((((gfx).wrapping_add(18).cast::<u16>()).read()) as u8), 2u8);
    }
}
pub(crate) unsafe extern "C" fn ShowOptionsCursor(gfx: *mut u8) -> u32 {
    unsafe {
        let mut gfx = gfx;
        if !((IsDma3ManagerBusyWithBgCopy()) != 0) {
            CreateOptionsCursorSprite(gfx, ((GetMatchCallOptionCursorPos()) as i32));
            return 0u32;
        }
        return 1u32;
    }
}
pub(crate) unsafe extern "C" fn UpdateWindowsReturnToTrainerList(gfx: *mut u8) {
    unsafe {
        let mut gfx = gfx;
        CloseMatchCallSelectOptionsWindow(gfx);
        UpdateMatchCallInfoBox(gfx);
    }
}
pub(crate) unsafe extern "C" fn IsDma3ManagerBusyWithBgCopy1(gfx: *mut u8) -> u32 {
    unsafe {
        let mut gfx = gfx;
        return ((IsDma3ManagerBusyWithBgCopy()) as u32);
    }
}
pub(crate) unsafe extern "C" fn UpdateWindowsToShowCheckPage(gfx: *mut u8) {
    unsafe {
        let mut gfx = gfx;
        CloseMatchCallSelectOptionsWindow(gfx);
        FillWindowPixelBuffer(
            ((((gfx).wrapping_add(18).cast::<u16>()).read()) as u8),
            17u8,
        );
        CopyWindowToVram(((((gfx).wrapping_add(18).cast::<u16>()).read()) as u8), 2u8);
    }
}
pub(crate) unsafe extern "C" fn LoadCallWindowAndFade(gfx: *mut u8) {
    unsafe {
        let mut gfx = gfx;
        ((gfx).wrapping_add(20).cast::<u16>()).write(AddWindow(
            (&raw const sCallMsgBoxWindowTemplate)
                .cast::<u8>()
                .cast_mut(),
        ));
        LoadMatchCallWindowGfx(
            ((((gfx).wrapping_add(20).cast::<u16>()).read()) as u32),
            1u32,
            4u32,
        );
        FadeToBlackExceptPrimary();
    }
}
pub(crate) unsafe extern "C" fn DrawMsgBoxForMatchCallMsg(gfx: *mut u8) {
    unsafe {
        let mut gfx = gfx;
        let mut sprite: *mut u8 = core::ptr::null_mut();
        LoadMatchCallWindowGfx(
            ((((gfx).wrapping_add(20).cast::<u16>()).read()) as u32),
            1u32,
            4u32,
        );
        DrawMatchCallTextBoxBorder(
            ((((gfx).wrapping_add(20).cast::<u16>()).read()) as u32),
            1u32,
            4u32,
        );
        FillWindowPixelBuffer(
            ((((gfx).wrapping_add(20).cast::<u16>()).read()) as u8),
            17u8,
        );
        PutWindowTilemap(((((gfx).wrapping_add(20).cast::<u16>()).read()) as u8));
        CopyWindowToVram(((((gfx).wrapping_add(20).cast::<u16>()).read()) as u8), 3u8);
        sprite = GetSpinningPokenavSprite();
        ((sprite).wrapping_add(32).cast::<i16>()).write(24i16);
        ((sprite).wrapping_add(34).cast::<i16>()).write(112i16);
        ((sprite).wrapping_add(38).cast::<i16>()).write(0i16);
    }
}
pub(crate) unsafe extern "C" fn DrawMsgBoxForCloseByMsg(gfx: *mut u8) {
    unsafe {
        let mut gfx = gfx;
        LoadUserWindowBorderGfx(
            ((((gfx).wrapping_add(20).cast::<u16>()).read()) as u8),
            1u16,
            64u8,
        );
        DrawTextBorderOuter(
            ((((gfx).wrapping_add(20).cast::<u16>()).read()) as u8),
            1u16,
            4u8,
        );
        FillWindowPixelBuffer(
            ((((gfx).wrapping_add(20).cast::<u16>()).read()) as u8),
            17u8,
        );
        PutWindowTilemap(((((gfx).wrapping_add(20).cast::<u16>()).read()) as u8));
        CopyWindowToVram(((((gfx).wrapping_add(20).cast::<u16>()).read()) as u8), 3u8);
    }
}
pub(crate) unsafe extern "C" fn IsDma3ManagerBusyWithBgCopy2(gfx: *mut u8) -> u32 {
    unsafe {
        let mut gfx = gfx;
        return ((IsDma3ManagerBusyWithBgCopy()) as u32);
    }
}
pub(crate) unsafe extern "C" fn PrintCallingDots(gfx: *mut u8) {
    unsafe {
        let mut gfx = gfx;
        AddTextPrinterParameterized(
            ((((gfx).wrapping_add(20).cast::<u16>()).read()) as u8),
            1u8,
            ((&raw const sText_CallingDots).cast::<u8>().cast_mut()).cast::<u8>(),
            32u8,
            1u8,
            1u8,
            None,
        );
    }
}
pub(crate) unsafe extern "C" fn WaitForCallingDotsText(gfx: *mut u8) -> u32 {
    unsafe {
        let mut gfx = gfx;
        RunTextPrinters();
        return ((IsTextPrinterActive(((((gfx).wrapping_add(20).cast::<u16>()).read()) as u8)))
            as u32);
    }
}
pub(crate) unsafe extern "C" fn PrintTrainerIsCloseBy(gfx: *mut u8) {
    unsafe {
        let mut gfx = gfx;
        AddTextPrinterParameterized(
            ((((gfx).wrapping_add(20).cast::<u16>()).read()) as u8),
            1u8,
            (&raw mut gText_TrainerCloseBy).cast::<u8>(),
            0u8,
            1u8,
            1u8,
            None,
        );
    }
}
pub(crate) unsafe extern "C" fn WaitForTrainerIsCloseByText(gfx: *mut u8) -> u32 {
    unsafe {
        let mut gfx = gfx;
        RunTextPrinters();
        return ((IsTextPrinterActive(((((gfx).wrapping_add(20).cast::<u16>()).read()) as u8)))
            as u32);
    }
}
pub(crate) unsafe extern "C" fn PrintMatchCallMessage(gfx: *mut u8) {
    unsafe {
        let mut gfx = gfx;
        let mut index: i32 = ((PokenavList_GetSelectedIndex()) as i32);
        let mut str: *mut u8 = GetMatchCallMessageText(index, (gfx).wrapping_add(15));
        let mut speed: u8 = GetPlayerTextSpeedDelay();
        AddTextPrinterParameterized(
            ((((gfx).wrapping_add(20).cast::<u16>()).read()) as u8),
            1u8,
            str,
            32u8,
            1u8,
            speed,
            None,
        );
    }
}
pub(crate) unsafe extern "C" fn WaitForMatchCallMessageText(gfx: *mut u8) -> u32 {
    unsafe {
        let mut gfx = gfx;
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(44)
            .cast::<u16>())
        .read()) as i32)
            & 1i32)
            != 0
        {
            crate::c::bf_write(
                ((&raw mut gTextFlags).cast::<u8>()).wrapping_add(0),
                0,
                1,
                (1u8) as i32,
            );
        } else {
            crate::c::bf_write(
                ((&raw mut gTextFlags).cast::<u8>()).wrapping_add(0),
                0,
                1,
                (0u8) as i32,
            );
        }
        RunTextPrinters();
        return ((IsTextPrinterActive(((((gfx).wrapping_add(20).cast::<u16>()).read()) as u8)))
            as u32);
    }
}
pub(crate) unsafe extern "C" fn EraseCallMessageBox(gfx: *mut u8) {
    unsafe {
        let mut gfx = gfx;
        HideSpinningPokenavSprite();
        FillBgTilemapBufferRect_Palette0(1u8, 0u16, 0u8, 0u8, 32u8, 20u8);
        CopyBgTilemapBufferToVram(1u8);
    }
}
pub(crate) unsafe extern "C" fn WaitForCallMessageBoxErase(gfx: *mut u8) -> u32 {
    unsafe {
        let mut gfx = gfx;
        return ((IsDma3ManagerBusyWithBgCopy()) as u32);
    }
}
pub(crate) unsafe extern "C" fn AllocMatchCallSprites() {
    unsafe {
        let mut i: i32 = 0i32;
        let mut paletteNum: u8 = 0u8;
        let mut spriteSheet = crate::ffi::Align4([0u8; 8]);
        let mut gfx: *mut u8 = GetSubstructPtr(6u32);
        {
            i = 0i32;
            'l1: loop {
                if !(((i) as u32) < crate::c::div_u32(8u32, 8u32)) {
                    break 'l1;
                }
                'l2: {
                    LoadCompressedSpriteSheet(
                        (((&raw const sOptionsCursorSpriteSheets)
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
            ((&raw const sOptionsCursorSpritePalettes)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>(),
        );
        ((gfx).wrapping_add(28).cast::<*mut u8>()).write(core::ptr::null_mut());
        (((&raw mut spriteSheet).cast::<u8>()).cast::<*mut u8>())
            .write(((gfx).wrapping_add(6184)).cast::<u8>());
        (((&raw mut spriteSheet).cast::<u8>())
            .wrapping_add(4)
            .cast::<u16>())
        .write(2048u16);
        (((&raw mut spriteSheet).cast::<u8>())
            .wrapping_add(6)
            .cast::<u16>())
        .write(8u16);
        ((gfx).wrapping_add(6180).cast::<*mut u8>()).write(
            ((100728832i32) as usize as *mut u8).wrapping_offset(
                (((LoadSpriteSheet((&raw mut spriteSheet).cast::<u8>())) as i32)
                    .wrapping_mul(32i32)) as isize,
            ),
        );
        paletteNum = AllocSpritePalette(13u16);
        ((gfx).wrapping_add(26).cast::<u16>())
            .write((((256i32).wrapping_add(((paletteNum) as i32).wrapping_mul(16i32))) as u16));
        ((gfx).wrapping_add(32).cast::<*mut u8>()).write(CreateTrainerPicSprite());
        crate::c::bf_write(
            (((gfx).wrapping_add(32).cast::<*mut u8>()).read()).wrapping_add(62),
            2,
            1,
            (1u16) as i32,
        );
    }
}
pub(crate) unsafe extern "C" fn FreeMatchCallSprites() {
    unsafe {
        let mut gfx: *mut u8 = GetSubstructPtr(6u32);
        if !(((gfx).wrapping_add(28).cast::<*mut u8>()).read()).is_null() {
            DestroySprite(((gfx).wrapping_add(28).cast::<*mut u8>()).read());
        }
        if !(((gfx).wrapping_add(32).cast::<*mut u8>()).read()).is_null() {
            DestroySprite(((gfx).wrapping_add(32).cast::<*mut u8>()).read());
        }
        FreeSpriteTilesByTag(8u16);
        FreeSpriteTilesByTag(7u16);
        FreeSpritePaletteByTag(12u16);
        FreeSpritePaletteByTag(13u16);
    }
}
pub(crate) unsafe extern "C" fn CreateOptionsCursorSprite(gfx: *mut u8, top: i32) {
    unsafe {
        let mut gfx = gfx;
        let mut top = top;
        if !(!(((gfx).wrapping_add(28).cast::<*mut u8>()).read()).is_null()) {
            let mut spriteId: u8 = CreateSprite(
                (&raw const sOptionsCursorSpriteTemplate)
                    .cast::<u8>()
                    .cast_mut(),
                4i16,
                80i16,
                5u8,
            );
            ((gfx).wrapping_add(28).cast::<*mut u8>()).write(
                ((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68),
            );
            UpdateCursorGfxPos(gfx, top);
        }
    }
}
pub(crate) unsafe extern "C" fn CloseMatchCallSelectOptionsWindow(gfx: *mut u8) {
    unsafe {
        let mut gfx = gfx;
        DestroySprite(((gfx).wrapping_add(28).cast::<*mut u8>()).read());
        ((gfx).wrapping_add(28).cast::<*mut u8>()).write(core::ptr::null_mut());
    }
}
pub(crate) unsafe extern "C" fn UpdateCursorGfxPos(gfx: *mut u8, top: i32) {
    unsafe {
        let mut gfx = gfx;
        let mut top = top;
        ((((gfx).wrapping_add(28).cast::<*mut u8>()).read())
            .wrapping_add(38)
            .cast::<i16>())
        .write((((top).wrapping_mul(16i32)) as i16));
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_OptionsCursor(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (({
            let __p1 = ((sprite).wrapping_add(46)).cast::<i16>();
            let __t2 = ((__p1).read()).wrapping_add(1);
            (__p1).write(__t2);
            __t2
        }) as i32)
            > 3i32
        {
            (((sprite).wrapping_add(46)).cast::<i16>()).write(0i16);
            ((sprite).wrapping_add(36).cast::<i16>()).write(
                ((((((sprite).wrapping_add(36).cast::<i16>()).read()) as i32).wrapping_add(1i32)
                    & 7i32) as i16),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn CreateTrainerPicSprite() -> *mut u8 {
    unsafe {
        let mut spriteId: u8 = CreateSprite(
            (&raw const sTrainerPicSpriteTemplate)
                .cast::<u8>()
                .cast_mut(),
            44i16,
            104i16,
            6u8,
        );
        return ((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((spriteId) as i32) as isize * 68);
    }
}
pub(crate) unsafe extern "C" fn LoadCheckPageTrainerPic(gfx: *mut u8) {
    unsafe {
        let mut gfx = gfx;
        let mut cursor: u16 = 0u16;
        let mut trainerPic: i32 = GetMatchCallTrainerPic(((PokenavList_GetSelectedIndex()) as i32));
        if trainerPic >= 0i32 {
            DecompressPicFromTable(
                ((&raw mut gTrainerFrontPicTable).cast::<u8>())
                    .wrapping_offset((trainerPic) as isize * 8),
                ((gfx).wrapping_add(6184)).cast::<u8>(),
                0i32,
            );
            LZ77UnCompWram(
                ((((&raw mut gTrainerFrontPicPaletteTable).cast::<u8>())
                    .wrapping_offset((trainerPic) as isize * 8))
                .cast::<*mut u32>())
                .read(),
                ((gfx).wrapping_add(8232)).cast::<u8>(),
            );
            cursor = ((RequestDma3Copy(
                ((gfx).wrapping_add(6184)).cast::<u8>(),
                ((gfx).wrapping_add(6180).cast::<*mut u8>()).read(),
                2048u16,
                1u8,
            )) as u16);
            LoadPalette(
                ((gfx).wrapping_add(8232)).cast::<u8>(),
                ((gfx).wrapping_add(26).cast::<u16>()).read(),
                32u16,
            );
            (((((gfx).wrapping_add(32).cast::<*mut u8>()).read()).wrapping_add(46)).cast::<i16>())
                .write(0i16);
            ((((((gfx).wrapping_add(32).cast::<*mut u8>()).read()).wrapping_add(46))
                .cast::<i16>())
            .wrapping_offset(7))
            .write(((cursor) as i16));
            ((((gfx).wrapping_add(32).cast::<*mut u8>()).read())
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(SpriteCB_TrainerPicSlideOnscreen));
        }
    }
}
pub(crate) unsafe extern "C" fn TrainerPicSlideOffscreen(gfx: *mut u8) {
    unsafe {
        let mut gfx = gfx;
        ((((gfx).wrapping_add(32).cast::<*mut u8>()).read())
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(SpriteCB_TrainerPicSlideOffscreen));
    }
}
pub(crate) unsafe extern "C" fn WaitForTrainerPic(gfx: *mut u8) -> u32 {
    unsafe {
        let mut gfx = gfx;
        return ((core::mem::transmute::<_, usize>(
            ((((gfx).wrapping_add(32).cast::<*mut u8>()).read())
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .read(),
        ) != (SpriteCallbackDummy as *const () as usize)) as u32);
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_TrainerPicSlideOnscreen(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        'l1: {
            let __sw1 = (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32);
            if __sw1 == 0i32 {
                if ((CheckForSpaceForDma3Request(
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read(),
                )) as i32)
                    != (-1i32)
                {
                    ((sprite).wrapping_add(36).cast::<i16>()).write((-80i16));
                    crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (0u16) as i32);
                    let __p2 = ((sprite).wrapping_add(46)).cast::<i16>();
                    (__p2).write(((__p2).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                let __p3 = (sprite).wrapping_add(36).cast::<i16>();
                (__p3).write((((((__p3).read()) as i32).wrapping_add(8i32)) as i16));
                if ((((sprite).wrapping_add(36).cast::<i16>()).read()) as i32) >= 0i32 {
                    ((sprite).wrapping_add(36).cast::<i16>()).write(0i16);
                    ((sprite)
                        .wrapping_add(28)
                        .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                    .write(Some(SpriteCallbackDummy));
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_TrainerPicSlideOffscreen(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let __p1 = (sprite).wrapping_add(36).cast::<i16>();
        (__p1).write((((((__p1).read()) as i32).wrapping_sub(8i32)) as i16));
        if ((((sprite).wrapping_add(36).cast::<i16>()).read()) as i32) <= (-80i32) {
            crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (1u16) as i32);
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(SpriteCallbackDummy));
        }
    }
}
